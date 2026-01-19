<template>
  <div class="product-grid" :class="`card-size-${cardSize}`">
    <n-card
      v-for="product in products"
      :key="product.id"
      class="product-card"
      :class="{ 'out-of-stock': product.current_stock === 0 }"
      :hoverable="product.current_stock > 0"
      embedded
      :content-style="{ padding: 0 }"
      @click="handleCardClick(product)"
    >
      <div class="image-container">
        <n-image
          v-if="product.image_url"
          :src="product.image_url"
          :alt="product.name"
          preview-disabled
          :img-props="{ style: 'width: 100%; height: 100%; object-fit: cover;' }"
          style="width: 100%; height: 100%;"
        />
        <div v-else class="no-img-placeholder">
          <span>{{ product.name?.charAt(0) || '🛍️' }}</span>
        </div>
      </div>

      <div class="product-info">
        <span class="product-name" :title="product.name">{{ product.name }}</span>
        <span class="product-price">¥{{ Number(product.price).toFixed(2) }}</span>
      </div>

      <div v-if="product.current_stock === 0" class="stock-overlay">
        <span>完售</span>
      </div>
    </n-card>
  </div>
</template>

<script setup>
import { NCard, NImage } from 'naive-ui'

const props = defineProps({
  products: { type: Array, required: true },
  cardSize: {
    type: String,
    default: 'medium',
    validator: (v) => ['small', 'medium', 'large'].includes(v)
  }
})

const emit = defineEmits(['addToCart'])

function handleCardClick(product) {
  if (product?.current_stock > 0) emit('addToCart', product)
}
</script>

<style scoped>
/* --- 网格布局 --- */
.product-grid {
  display: grid;
  gap: 0.8rem;
}

/* 三档：决定列宽（卡片宽度交给 grid） */
.product-grid.card-size-small {
  grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
}
.product-grid.card-size-medium {
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
}
.product-grid.card-size-large {
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
}

/* --- 商品卡片 --- */
.product-card {
  background-color: var(--card-bg-color);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  overflow: hidden;
  cursor: pointer;
  position: relative;
  transition: transform 0.18s, box-shadow 0.18s, border-color 0.18s;
  display: flex;
  flex-direction: column;
}

.product-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 16px var(--shadow-color);
  border-color: var(--accent-color);
}

/* 完售 */
.out-of-stock {
  cursor: not-allowed;
}
.out-of-stock:hover {
  transform: none;
  box-shadow: none;
  border-color: var(--border-color);
}

/* --- 图片区：用固定高度更稳 --- */
.image-container {
  width: 100%;
  background-color: var(--bg-color);
  overflow: hidden;
}

/* 三档图片高度 */
.product-grid.card-size-small .image-container { height: 110px; }
.product-grid.card-size-medium .image-container { height: 150px; }
.product-grid.card-size-large .image-container { height: 220px; }

/* 让 n-image 填满容器 */
:deep(.image-container .n-image) {
  width: 100%;
  height: 100%;
}
:deep(.image-container .n-image img) {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.no-img-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 2.2rem;
  color: var(--accent-color);
  opacity: 0.45;
}

/* --- 信息区 --- */
.product-info {
  padding: 0.45rem 0.55rem 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

/* 三档字号 */
.product-grid.card-size-small .product-name { font-size: 0.72rem; }
.product-grid.card-size-medium .product-name { font-size: 0.84rem; }
.product-grid.card-size-large .product-name { font-size: 0.98rem; }

.product-name {
  font-weight: 650;
  color: var(--primary-text-color);
  line-height: 1.15;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  white-space: normal;
}

.product-grid.card-size-small .product-price { font-size: 0.82rem; }
.product-grid.card-size-medium .product-price { font-size: 1.02rem; }
.product-grid.card-size-large .product-price { font-size: 1.18rem; }

.product-price {
  color: var(--accent-color);
  font-weight: 800;
  white-space: nowrap;
}

/* 完售遮罩 */
.stock-overlay {
  position: absolute;
  inset: 0;
  background-color: var(--overlay-color);
  display: flex;
  justify-content: center;
  align-items: center;
  color: var(--text-white);
  font-size: 1.4rem;
  font-weight: 900;
  backdrop-filter: blur(2px);
}
</style>
