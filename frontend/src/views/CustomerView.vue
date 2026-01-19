<template>
  <div class="customer-view">
    <div class="sidebar">
      <n-scrollbar style="max-height: 100vh;">
        <n-space vertical class="category-list">
          <n-button
            class="category-btn"
            :type="selectedCategory === '' ? 'primary' : 'default'"
            :tertiary="selectedCategory !== ''"
            block
            @click="selectedCategory = ''"
          >
            全部分类
          </n-button>

          <n-button
            v-for="cat in categoryOptions"
            :key="cat"
            class="category-btn"
            :type="selectedCategory === cat ? 'primary' : 'default'"
            :tertiary="selectedCategory !== cat"
            block
            @click="selectedCategory = cat"
          >
            {{ cat }}
          </n-button>
        </n-space>
      </n-scrollbar>
    </div>

    <div class="product-panel">
      <!-- ✅ 卡片大小滑动条（替代按钮组） -->
      <div class="card-size-toolbar">
        <n-space align="center" justify="space-between" style="width: 100%">
          <n-space align="center" :size="10" class="toolbar-left">
            <n-text depth="2" class="toolbar-label">卡片大小</n-text>

            <div class="slider-wrap">
              <n-slider
                v-model:value="cardSizeIndex"
                :min="0"
                :max="2"
                :step="1"
                :marks="cardSizeMarks"
                @update:value="onCardSizeUserChange"
              />
            </div>

            <n-tag size="small" type="info" :bordered="false" class="size-tag">
              {{ cardSizeLabel }}
            </n-tag>
          </n-space>
        </n-space>
      </div>

      <n-spin :show="store.isLoading">
        <div class="product-scroll" v-if="!store.isLoading">
          <ProductGrid
            :products="filteredProducts"
            :card-size="cardSize"
            @add-to-cart="store.addToCart"
          />
        </div>
      </n-spin>
    </div>

    <div class="cart-panel" v-if="!isMobile">
      <n-card size="small" embedded>
        <ShoppingCart
          :cart="store.cart"
          :total="store.cartTotal"
          :is-checking-out="isCheckingOut"
          @add-to-cart="store.addToCart"
          @remove-from-cart="store.removeFromCart"
          @checkout="handleCheckout"
        />
      </n-card>
    </div>

    <!-- 手机端底部悬浮购物车 -->
    <ShoppingCart
      v-if="isMobile"
      :cart="store.cart"
      :total="store.cartTotal"
      :is-checking-out="isCheckingOut"
      @add-to-cart="store.addToCart"
      @remove-from-cart="store.removeFromCart"
      @checkout="handleCheckout"
    />

    <PaymentModal
      :show="showPaymentModal"
      :total="orderTotal"
      :qr-code-url="store.qrCodeUrl"
      @close="closePaymentModal"
    />
  </div>
</template>

<script setup>
import { useAlert } from '@/services/useAlert'
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import { useCustomerStore } from '@/stores/customerStore'
import ProductGrid from '@/components/customer/ProductGrid.vue'
import ShoppingCart from '@/components/customer/ShoppingCart.vue'
import PaymentModal from '@/components/customer/PaymentModal.vue'
import {
  NButton,
  NSpace,
  NScrollbar,
  NCard,
  NSpin,
  NSlider,
  NText,
  NTag
} from 'naive-ui'

// 通过 props 接收来自路由的展会 ID
const props = defineProps({
  id: { type: String, required: true }
})

const store = useCustomerStore()

const showPaymentModal = ref(false)
const orderTotal = ref(0)
const isCheckingOut = ref(false)

const selectedCategory = ref('')

// ===============================
// ✅ 卡片大小：slider 三档
// ===============================
const cardSizeMarks = { 0: '小', 1: '中', 2: '大' }
const cardSizeIndex = ref(1) // 默认中
const userTouchedCardSize = ref(false) // 用户是否手动调过

const cardSize = computed(() => {
  return cardSizeIndex.value === 0 ? 'small' : cardSizeIndex.value === 1 ? 'medium' : 'large'
})
const cardSizeLabel = computed(() => (cardSizeIndex.value === 0 ? '小' : cardSizeIndex.value === 1 ? '中' : '大'))

function onCardSizeUserChange() {
  userTouchedCardSize.value = true
}

// ===============================
// 分类/过滤
// ===============================
const categoryOptions = computed(() => {
  const cats = (store.products || [])
    .map((p) => p.category)
    .filter((cat) => !!cat && cat.trim() !== '')
  return [...new Set(cats)]
})

const filteredProducts = computed(() => {
  let list = store.products || []
  if (selectedCategory.value) {
    list = list.filter((p) => p.category === selectedCategory.value)
  }
  return list
})

// ===============================
// 下单
// ===============================
async function handleCheckout() {
  const { showSuccess, showError } = useAlert()
  if (isCheckingOut.value) return

  isCheckingOut.value = true
  try {
    const newOrder = await store.submitOrder()
    if (newOrder) {
      orderTotal.value = store.cartTotal
      showPaymentModal.value = true
      showSuccess('订单已成功提交！')
      store.clearCart()
    }
  } catch (error) {
    console.log(error)
    showError(error?.message || '下单失败，请重试')
  } finally {
    isCheckingOut.value = false
  }
}

function closePaymentModal() {
  showPaymentModal.value = false
}

// ===============================
// 响应式：移动端判定 + 自动默认小卡
// ===============================
const isMobile = ref(false)

function syncLayoutByWidth() {
  const mobile = window.innerWidth <= 600
  isMobile.value = mobile

  // 仅当用户没手动调过时，移动端默认切小
  if (mobile && !userTouchedCardSize.value) {
    cardSizeIndex.value = 0
  }

  // 如果从移动端回到桌面端，且用户没手动调过，可以回到中（可选）
  if (!mobile && !userTouchedCardSize.value) {
    cardSizeIndex.value = 1
  }
}

onMounted(() => {
  store.setupStoreForEvent(props.id)
  syncLayoutByWidth()
  window.addEventListener('resize', syncLayoutByWidth)
})

onUnmounted(() => {
  window.removeEventListener('resize', syncLayoutByWidth)
})

// 如果你希望：切换分类时自动回到顶部（可选）
// watch(selectedCategory, () => {
//   // 这里可以触发 product-scroll 的滚动归零
// })
</script>

<style scoped>
.sidebar {
  flex: 0 0 100px;
  padding: 1rem;
  background: var(--card-bg-color);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
}

.customer-view {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background-color: var(--bg-color);
}

.product-panel {
  flex: 3;
  padding: 1.5rem 1.5rem 1.25rem;
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.product-scroll {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding-right: 0.25rem;
  box-sizing: border-box;
}

.cart-panel {
  flex: 1;
  padding: 1.5rem;
  background-color: var(--card-bg-color);
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  height: 100vh;
  box-sizing: border-box;
  overflow-x: hidden;
  overflow-y: auto;
}

.category-btn {
  border-radius: 6px;
  font-size: 0.95rem;
}

/* ✅ Toolbar：更紧凑 + sticky，避免占空间 */
.card-size-toolbar {
  position: sticky;
  top: 0;
  z-index: 5;
  padding: 0.4rem 0.25rem 0.9rem;
  background: var(--bg-color);
}

.toolbar-label {
  font-size: 0.9rem;
  white-space: nowrap;
}

.slider-wrap {
  width: 180px;
}

.size-tag {
  min-width: 32px;
  text-align: center;
}

.category-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  overflow-y: auto;
}

/* 平板端适配 */
@media (max-width: 1024px) {
  .sidebar {
    flex: 0 0 140px;
    padding: 1rem;
  }
  .product-panel {
    padding: 1.1rem;
  }
  .cart-panel {
    padding: 1.1rem;
  }
}

/* 移动端布局（堆叠） */
@media (max-width: 768px) {
  .customer-view {
    flex-direction: column;
    height: auto;
    min-height: 100vh;
    overflow: visible;
  }

  .sidebar {
    position: sticky;
    top: 0;
    width: 100%;
    height: auto;
    flex: none;
    padding: 0.75rem;
    border-right: none;
    border-bottom: 1px solid var(--border-color);
    z-index: 10;
  }

  .category-list {
    flex-direction: row;
    gap: 0.5rem;
    overflow-x: auto;
    padding-bottom: 0.25rem;
  }

  .category-btn {
    flex: 0 0 auto;
    min-width: 92px;
    text-align: center;
    font-size: 0.88rem;
  }

  .product-panel {
    padding: 0.9rem;
    width: 100%;
    box-sizing: border-box;
    height: auto;
  }

  .product-scroll {
    max-height: none;
  }

  .cart-panel {
    display: none;
  }

  .slider-wrap {
    width: 140px;
  }
}

/* 小屏再收紧间距 */
@media (max-width: 480px) {
  .sidebar {
    padding: 0.6rem;
  }
  .category-btn {
    min-width: 80px;
    padding: 0.45rem 0.6rem;
    font-size: 0.82rem;
  }
  .product-panel {
    padding: 0.75rem;
  }

  .slider-wrap {
    width: 120px;
  }
}
</style>
