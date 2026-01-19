<template>
  <div
    class="shopping-cart"
    :class="{ 'mobile-collapsed': isMobile && !expanded, 'mobile-expanded': isMobile && expanded }"
  >
    <!-- 手机端底部栏 -->
    <div v-if="isMobile" class="cart-mobile-bar" @click="toggleCart">
      <span class="bar-left">🛒 我的订单</span>
      <span class="cart-mobile-total">¥{{ total.toFixed(2) }}</span>
      <span class="cart-mobile-arrow">{{ expanded ? '▼' : '▲' }}</span>
    </div>

    <!-- 只有展开时才显示详细内容 -->
    <div v-show="!isMobile || expanded" class="cart-content">
      <h3 class="cart-title">我的订单</h3>

      <div class="cart-list-wrapper">
        <ul v-if="cart.length" class="cart-list">
          <li v-for="item in cart" :key="item.id" class="cart-item">
            <!-- 左：信息区 -->
            <div class="item-info">
              <span class="item-name" :title="item.name">{{ item.name }}</span>
              <span class="item-meta">
                <span class="item-price">¥{{ item.price.toFixed(2) }}</span>
                <span class="item-subtotal">小计 ¥{{ (item.price * item.quantity).toFixed(2) }}</span>
              </span>
            </div>

            <!-- 右：控件区（固定宽度，不挤文字） -->
            <div class="item-controls">
              <n-button
                size="small"
                circle
                tertiary
                @click.stop="$emit('removeFromCart', item.id)"
              >
                -
              </n-button>

              <span class="item-quantity">{{ item.quantity }}</span>

              <n-button
                size="small"
                circle
                tertiary
                @click.stop="$emit('addToCart', item)"
              >
                +
              </n-button>
            </div>
          </li>
        </ul>

        <p v-else class="empty-cart">
          <span>🛒</span>
          请点击左侧商品添加到这里
        </p>
      </div>

      <div class="cart-summary">
        <div class="total">
          <span>总计</span>
          <strong class="total-amount">¥{{ total.toFixed(2) }}</strong>
        </div>

        <n-button
          type="primary"
          block
          :disabled="!cart.length || isCheckingOut"
          :loading="isCheckingOut"
          @click="$emit('checkout')"
        >
          {{ isCheckingOut ? '下单中...' : '确认下单' }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { NButton } from 'naive-ui'

defineProps({
  cart: { type: Array, required: true },
  total: { type: Number, required: true },
  isCheckingOut: { type: Boolean, default: false }
})
defineEmits(['addToCart', 'removeFromCart', 'checkout'])

const isMobile = ref(false)
const expanded = ref(false)

function checkMobile() {
  isMobile.value = window.innerWidth <= 600
  if (!isMobile.value) expanded.value = false
}
function toggleCart() {
  expanded.value = !expanded.value
}

onMounted(() => {
  checkMobile()
  window.addEventListener('resize', checkMobile)
})
onUnmounted(() => {
  window.removeEventListener('resize', checkMobile)
})
</script>

<style scoped>
.shopping-cart {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--card-bg-color);
  transition: box-shadow 0.2s, transform 0.2s;
}

/* 内容区：让列表可滚动，总结固定在底部 */
.cart-content {
  display: flex;
  flex-direction: column;
  min-height: 0; /* 关键：允许内部滚动容器生效 */
  background: var(--card-bg-color);
}

.cart-title {
  font-size: 1.25rem;
  color: var(--accent-color);
  text-align: center;
  margin: 0.75rem 0 1rem;
  flex-shrink: 0;
}

.cart-list-wrapper {
  flex: 1 1 auto;
  min-height: 0;
  overflow-y: auto;
  padding: 0 0.25rem;
}

.cart-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

/* ✅ 用 Grid 保证“左信息 + 右控件”不互相挤爆 */
.cart-item {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 0.5rem;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--border-color);
}

.item-info {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.item-name {
  font-size: 0.95rem;
  font-weight: 600;
  line-height: 1.2;

  /* ✅ 两行截断（移动端很重要） */
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;

  overflow: hidden;
  text-overflow: ellipsis;
  word-break: break-word;
}

.item-meta {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
  font-size: 0.8rem;
  color: var(--text-muted);
  line-height: 1.2;
}

.item-price {
  white-space: nowrap;
}

.item-subtotal {
  white-space: nowrap;
  opacity: 0.9;
}

/* 控件区固定，不挤信息区 */
.item-controls {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  flex: 0 0 auto;
}

.item-quantity {
  font-size: 0.95rem;
  font-weight: 700;
  min-width: 2.2ch; /* 数字宽度稳定 */
  text-align: center;
}

.empty-cart {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: var(--text-disabled);
  font-size: 0.95rem;
}
.empty-cart span {
  font-size: 3rem;
  margin-bottom: 0.5rem;
}

.cart-summary {
  flex-shrink: 0;
  padding: 0.9rem 0.25rem 0.25rem;
  border-top: 1px solid var(--border-color);
}

.total {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  font-size: 1rem;
  margin-bottom: 0.75rem;
}

.total-amount {
  font-size: 1.35rem;
  font-weight: 800;
  color: var(--accent-color);
}

/* 手机端底部悬浮栏 */
.cart-mobile-bar {
  display: flex;
  align-items: center;
  padding: 0.85rem 1rem;
  background: var(--accent-color);
  color: var(--text-white);
  font-size: 0.95rem;
  font-weight: 700;
  border-radius: 12px 12px 0 0;
  cursor: pointer;
  box-shadow: 0 -2px 12px var(--shadow-color);
  position: relative;
  z-index: 10;
  gap: 0.5rem;
}

.bar-left {
  white-space: nowrap;
}

.cart-mobile-total {
  margin-left: auto;
  white-space: nowrap;
}

.cart-mobile-arrow {
  white-space: nowrap;
  font-size: 0.95rem;
}

/* 响应式：手机端底部抽屉效果 */
@media (max-width: 600px) {
  .shopping-cart {
    position: fixed;
    left: 0;
    right: 0;
    bottom: 40px; /* 避免系统导航栏遮挡 */
    width: 100vw;
    max-width: 100vw;
    z-index: 1201;
    box-shadow: 0 -2px 16px var(--shadow-color);
    border-radius: 12px 12px 0 0;
    transition: transform 0.25s cubic-bezier(.4, 2, .6, 1), box-shadow 0.2s;
    height: auto;
    margin-bottom: env(safe-area-inset-bottom, 0);
  }

  .cart-title {
    display: none;
  }

  .cart-item {
    padding: 0.6rem 0;
  }

  .item-name {
    font-size: 0.7rem;
    -webkit-line-clamp: 2;
  }

  .item-meta {
    font-size: 0.78rem;
  }

  .item-controls {
    gap: 0.35rem;
  }

  .item-quantity {
    font-size: 0.9rem;
  }

  .shopping-cart.mobile-collapsed .cart-content {
    display: none;
  }

  .shopping-cart.mobile-expanded {
    max-height: calc(80vh - 60px);
    overflow: hidden; /* 让内部 cart-list-wrapper 自己滚 */
  }

  .shopping-cart.mobile-collapsed {
    max-height: 64px;
    overflow: hidden;
  }

  .cart-summary {
    padding-bottom: 0.75rem;
  }
}
</style>
