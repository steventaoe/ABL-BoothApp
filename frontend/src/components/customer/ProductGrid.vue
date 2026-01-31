<template>
  <draggable
    v-model="localList"
    class="product-grid"
    :class="[`card-size-${cardSize}`, { 'is-editing': editable }]"
    item-key="id"
    :animation="250"
    ghost-class="ghost-card"
    drag-class="drag-card"
    :disabled="!editable"
    :force-fallback="true"
    :fallback-on-body="false"
    :fallback-tolerance="3"
    :touch-start-threshold="4"
    @end="handleDragEnd"
  >
    <template #item="{ element: product }">
      <n-card
        class="product-card"
        :class="{
          'out-of-stock': product.current_stock === 0,
          'low-stock': !editable && product.current_stock > 0 && product.current_stock <= 10
        }"
        embedded
        :content-style="{ padding: 0 }"
        :bordered="false"
      >
        <div class="card-inner" @click="handleCardClick(product)">
          <div class="media-box">
            <n-image
              v-if="product.image_url"
              class="media-img"
              :src="product.image_url"
              :alt="product.name"
              preview-disabled
              :img-props="{ loading: 'lazy', draggable: false }"
            >
              <!-- ✅ 加载中：Skeleton -->
              <template #placeholder>
                <div class="media-skeleton">
                  <n-skeleton class="sk-img" :sharp="false" height="100%" width="100%" />
                  <div class="sk-shine" />
                </div>
              </template>

              <!-- ✅ 加载失败：Skeleton + 提示 -->
              <template #error>
                <div class="media-error">
                  <n-skeleton class="sk-img" :sharp="false" height="100%" width="100%" />
                  <div class="err-text">图片加载失败</div>
                </div>
              </template>
            </n-image>

            <div v-else class="media-placeholder">
              <span class="placeholder-emoji">{{ product.name?.charAt(0) || '🛍️' }}</span>
            </div>

            <div v-if="editable" class="edit-overlay">
              <span class="drag-icon">✋ 拖动排序</span>
            </div>

            <template v-else>
              <div
                v-if="product.current_stock > 0 && product.current_stock <= 10"
                class="chip stock-warning"
              >
                <span>剩 {{ product.current_stock }}</span>
              </div>

              <!-- ✅ 更美观的 SOLD OUT -->
              <div v-if="product.current_stock === 0" class="sold-overlay">
                <div class="sold-badge">SOLD OUT</div>
                <div class="sold-sub">已售罄</div>
              </div>
            </template>
          </div>

          <div class="info-box">
            <div class="title" :title="product.name">
              {{ product.name }}
            </div>

            <div class="bottom-row">
              <div class="price-wrapper">
                <span class="currency">¥</span>
                <span class="value">{{ formatPrice(product.price) }}</span>
              </div>

              <div class="action-icon" v-if="!editable && product.current_stock > 0">
                <span class="plus-sign">+</span>
              </div>
            </div>
          </div>
        </div>
      </n-card>
    </template>
  </draggable>
</template>

<script setup>
import { ref, watch } from 'vue'
import draggable from 'vuedraggable'
import { NCard, NImage, NSkeleton } from 'naive-ui'

const props = defineProps({
  products: { type: Array, default: () => [] },
  cardSize: {
    type: String,
    default: 'medium',
    validator: (v) => ['small', 'medium', 'large'].includes(v)
  },
  editable: { type: Boolean, default: false }
})

const emit = defineEmits(['addToCart', 'update:products', 'order-changed'])

const localList = ref([])

watch(
  () => props.products,
  (val) => {
    if (!props.editable) localList.value = Array.isArray(val) ? [...val] : []
    if (props.editable && localList.value.length === 0) localList.value = Array.isArray(val) ? [...val] : []
  },
  { immediate: true }
)

function handleCardClick(product) {
  if (props.editable) return
  if (product?.current_stock > 0) emit('addToCart', product)
}

function handleDragEnd() {
  if (!props.editable) return
  const next = [...localList.value]
  emit('update:products', next)
  emit('order-changed')
}

function formatPrice(price) {
  const n = Number(price)
  return Number.isFinite(n) ? n.toFixed(2) : '--'
}
</script>

<style scoped>
.product-grid {
  --pg-bg: var(--card-bg-color);
  --pg-border: var(--border-color);
  --pg-accent: var(--accent-color);
  --pg-radius: 12px;

  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(var(--min-col), 1fr));
  gap: 12px;
  padding: 4px;
}

.product-grid.card-size-small  { --min-col: 110px; }
.product-grid.card-size-medium { --min-col: 150px; }
.product-grid.card-size-large  { --min-col: 220px; }

.product-card {
  border-radius: var(--pg-radius);
  transition: transform 0.2s, box-shadow 0.2s;
  border: 1px solid var(--pg-border);
  background-color: var(--pg-bg);
  overflow: hidden;
  height: 100%;
}

.product-grid:not(.is-editing) .product-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.08);
}

.card-inner {
  height: 100%;
  display: flex;
  flex-direction: column;
  cursor: pointer;
  user-select: none;
}

.media-box {
  position: relative;
  width: 100%;
  background-color: var(--bg-secondary);
  overflow: hidden;
}

/* 维持网格比例（你原来的逻辑） */
.product-grid { --pg-media-pad: 100%; }
.product-grid.card-size-large { --pg-media-pad: 75%; }

.media-box::before {
  content: "";
  display: block;
  padding-top: var(--pg-media-pad);
}

:deep(.media-img),
.media-placeholder {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
}

/* ✅ Naive n-image 内部 wrapper 也铺满 */
:deep(.media-img .n-image),
:deep(.media-img .n-image .n-image-wrapper),
:deep(.media-img .n-image img) {
  width: 100%;
  height: 100%;
  display: block;
}

/* ✅ 关键改动：cover -> contain（不裁切竖图/二维码） */
:deep(.media-img .n-image img) {
  object-fit: contain;
  object-position: center;
  background: var(--bg-secondary);
}

/* Skeleton / error 覆盖整个 media 区域 */
.media-skeleton,
.media-error {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
}

.media-skeleton .sk-img,
.media-error .sk-img {
  width: 100%;
  height: 100%;
}

.media-skeleton {
  overflow: hidden;
}
.media-skeleton .sk-shine {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    110deg,
    rgba(255,255,255,0.0) 0%,
    rgba(255,255,255,0.20) 30%,
    rgba(255,255,255,0.0) 60%
  );
  transform: translateX(-60%);
  animation: shine 1.2s infinite;
}

@keyframes shine {
  0% { transform: translateX(-60%); }
  100% { transform: translateX(60%); }
}

.media-error {
  display: flex;
  align-items: center;
  justify-content: center;
}
.media-error .err-text {
  position: absolute;
  bottom: 8px;
  left: 8px;
  right: 8px;
  font-size: 12px;
  font-weight: 800;
  color: rgba(0,0,0,0.55);
  background: rgba(255,255,255,0.7);
  border-radius: 8px;
  padding: 6px 8px;
  text-align: center;
}

.media-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2.5em;
  opacity: 0.5;
}

.chip {
  position: absolute;
  top: 6px;
  right: 6px;
  padding: 2px 6px;
  border-radius: 8px;
  font-size: 10px;
  font-weight: 900;
  color: white;
  background: rgba(0,0,0,0.55);
  backdrop-filter: blur(6px);
}
.chip.stock-warning { background: #d03050; }

/* ✅ 更美观 SOLD OUT：磨砂 + badge */
.sold-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
  align-items: center;
  justify-content: center;
  background: rgba(255,255,255,0.55);
  backdrop-filter: blur(6px);
}

.sold-badge {
  padding: 6px 12px;
  border-radius: 999px;
  font-weight: 950;
  letter-spacing: 0.06em;
  font-size: 12px;
  color: white;
  background: rgba(20,20,20,0.86);
  box-shadow: 0 8px 18px rgba(0,0,0,0.18);
  transform: rotate(-6deg);
}

.sold-sub {
  font-size: 12px;
  font-weight: 800;
  color: rgba(0,0,0,0.55);
}

/* 信息区 */
.info-box {
  padding: 10px 10px;
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 6px;
  min-width: 0;
}

.title {
  font-size: 0.92rem;
  line-height: 1.35;
  color: var(--primary-text-color);
  font-weight: 650;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.bottom-row {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  flex-wrap: nowrap;
  gap: 8px;
}

.price-wrapper {
  color: var(--pg-accent);
  line-height: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.currency { font-size: 0.75rem; margin-right: 1px; }
.value { font-size: 1.1rem; font-weight: 900; font-family: sans-serif; }

.action-icon {
  flex-shrink: 0;
  width: 22px;
  height: 22px;
  border-radius: 50%;
  background: var(--bg-secondary);
  color: var(--primary-text-color);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
}

/* 拖拽视觉 */
.ghost-card {
  opacity: 0.5;
  background: #e0e0e0;
  border: 2px dashed #999;
  border-radius: var(--pg-radius);
}
.drag-card {
  opacity: 1;
  transform: scale(1.05) rotate(2deg);
  box-shadow: 0 12px 24px rgba(0,0,0,0.2);
  z-index: 1000;
  cursor: grabbing;
}

.is-editing .product-card {
  cursor: grab;
  animation: shake 2s infinite ease-in-out;
}
.is-editing .product-card:active { cursor: grabbing; }

.edit-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: center;
  justify-content: center;
  border: 2px dashed var(--pg-accent);
}
.drag-icon {
  background: var(--pg-accent);
  color: white;
  padding: 4px 10px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 800;
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}

@keyframes shake {
  0% { transform: rotate(0deg); }
  25% { transform: rotate(0.5deg); }
  75% { transform: rotate(-0.5deg); }
  100% { transform: rotate(0deg); }
}
</style>
