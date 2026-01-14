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
          >全部分类</n-button>
          <n-button
            v-for="cat in categoryOptions"
            :key="cat"
            class="category-btn"
            :type="selectedCategory === cat ? 'primary' : 'default'"
            :tertiary="selectedCategory !== cat"
            block
            @click="selectedCategory = cat"
          >{{ cat }}</n-button>
        </n-space>
      </n-scrollbar>
    </div>
    <div class="product-panel">
      <div class="card-size-toolbar">
        <n-button-group>
          <n-button :type="cardSize === 'small' ? 'primary' : 'default'" quaternary @click="cardSize = 'small'">小</n-button>
          <n-button :type="cardSize === 'medium' ? 'primary' : 'default'" quaternary @click="cardSize = 'medium'">中</n-button>
          <n-button :type="cardSize === 'large' ? 'primary' : 'default'" quaternary @click="cardSize = 'large'">大</n-button>
        </n-button-group>
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
import { useAlert } from '@/services/useAlert';

// 获取弹窗函数


import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useCustomerStore } from '@/stores/customerStore';
import { socket } from '@/services/socketService';
import ProductGrid from '@/components/customer/ProductGrid.vue';
import ShoppingCart from '@/components/customer/ShoppingCart.vue';
import PaymentModal from '@/components/customer/PaymentModal.vue';
import { NButton, NButtonGroup, NSpace, NScrollbar, NCard, NSpin } from 'naive-ui';

// 【核心改动】通过 props 接收来自路由的展会 ID
const props = defineProps({
  id: {
    type: String,
    required: true,
  },
});

const store = useCustomerStore();
const showPaymentModal = ref(false);
const orderTotal = ref(0);
const isCheckingOut = ref(false); 
const selectedCategory = ref('');
const cardSize = ref('medium'); // 默认中等大小
const categoryOptions = computed(() => {
  const cats = (store.products || [])
    .map(p => p.category)
    .filter(cat => !!cat && cat.trim() !== '');
  return [...new Set(cats)];
});
const filteredProducts = computed(() => {
  let list = store.products || [];
  if (selectedCategory.value) {
    list = list.filter(p => p.category === selectedCategory.value);
  }
  // 你可以在这里加搜索过滤
  return list;
});
async function handleCheckout() {
  const { showAlert, showSuccess, showError } = useAlert();
  if (isCheckingOut.value) return; // 防止重复提交

  isCheckingOut.value = true;
  try {
    const newOrder = await store.submitOrder();
    if (newOrder) {
      orderTotal.value = store.cartTotal;
      showPaymentModal.value = true;
      showSuccess('订单已成功提交！');
      store.clearCart();
    }
  } catch (error) {
    console.log(error)
    
    showError(error.message);;
  } finally {
    isCheckingOut.value = false; // 无论成功失败，都解除禁用
  }
}
function closePaymentModal() {
  showPaymentModal.value = false;
}

onMounted(() => {
  store.setupStoreForEvent(props.id);;
});


function checkMobileCardSize() {
  if (window.innerWidth <= 600) {
    cardSize.value = 'small';
  }
}
const isMobile = ref(window.innerWidth <= 600);
function checkMobile() {
  isMobile.value = window.innerWidth <= 600;
}
onMounted(() => {
  checkMobile();
  checkMobileCardSize();
  window.addEventListener('resize', checkMobileCardSize);
});
onUnmounted(() => {
  window.removeEventListener('resize', checkMobileCardSize);
});
</script>

<style scoped>
.sidebar {
  flex: 0 0 100px; /* 固定宽度 */
  padding: 1rem 1rem 1rem 1rem;
  background: var(--card-bg-color);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
}
.category-select {
  margin-bottom: 1rem;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  background: var(--card-bg-color);
  color: var(--primary-text-color);
  min-width: 120px;
}
.customer-view {
  display: flex;
  height: 100vh;
  overflow: hidden; /* 禁止整体页面滚动，改为内部滚动 */
  background-color: var(--bg-color); /* 确保背景色统一 */
}

.product-panel {
  flex: 3; /* 占据约 3/4 宽度 */
  padding: 2rem;
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden; /* 交给内部滚动容器处理 */
}

.product-scroll {
  flex: 1;
  min-height: 0; /* 允许子元素在 flex 容器中正确收缩以启用滚动 */
  overflow-y: auto;
  padding-right: 0.25rem; /* 防止滚动条遮挡内容 */
  box-sizing: border-box;
}

.cart-panel {
  flex: 1; /* 占据约 1/4 宽度 */
  padding: 2rem;
  background-color: var(--card-bg-color);
  border-left: 1px solid var(--border-color);
  display: flex; /* 让购物车内容能更好地布局 */
  flex-direction: column;
  height: 100vh; /* 确保侧边栏与视口等高 */
  box-sizing: border-box; /* 防止 padding 导致溢出 */
  overflow-x:hidden; /* 防止横向滚动 */
  overflow-y: auto;
}
.category-btn {
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 0.75rem 1rem;
  color: var(--primary-text-color);
  text-align: left;
  cursor: pointer;
  transition: background 0.2s, color 0.2s, border-color 0.2s;
  font-size: 1rem;
}

.category-btn.active,
.category-btn:hover {
  background: var(--accent-color);
  color: var(--text-on-dark);
  border-color: var(--accent-color);
}
.category-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  overflow-y: auto; /* 当分类过多时，允许滚动 */
}
.card-size-toolbar {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
}

.card-size-toolbar button {
  background: none;
  border: 2px solid var(--border-color);
  border-radius: 4px;
  padding: 0.5rem 1.2rem;
  color: var(--primary-text-color);
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: 
    background 0.2s, 
    color 0.2s, 
    border-color 0.2s, 
    box-shadow 0.2s;
  outline: none;
}

.card-size-toolbar button.active,
.card-size-toolbar button:focus {
  background: var(--accent-color);
  color: var(--text-on-dark);
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px var(--accent-color-light);
}

.card-size-toolbar button:hover:not(.active) {
  background: var(--card-bg-color);
  border-color: var(--accent-color);
  color: var(--accent-color);
}
/* 平板端适配 */
@media (max-width: 1024px) {
  .sidebar {
    flex: 0 0 140px;
    padding: 1rem;
  }
  .product-panel {
    padding: 1.25rem;
  }
  .cart-panel {
    padding: 1.25rem;
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
  }

  .product-panel {
    padding: 1rem;
    margin-left: 0;
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
}

/* 小屏再收紧间距 */
@media (max-width: 480px) {
  .sidebar {
    padding: 0.6rem;
  }
  .category-btn {
    min-width: 80px;
    padding: 0.45rem 0.6rem;
    font-size: 0.85rem;
  }
  .product-panel {
    padding: 0.75rem;
  }
  .card-size-toolbar {
    justify-content: center;
    margin-bottom: 1rem;
  }
}
</style>