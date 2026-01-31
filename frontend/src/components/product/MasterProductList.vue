<template>
  <div class="list-container">
    <div class="section-header" @click="isListCollapsed = !isListCollapsed">
      <h2>商品列表</h2>
      <n-button text class="toggle-btn">
        {{ isListCollapsed ? '展开' : '折叠' }}
      </n-button>
    </div>

    <transition name="expand">
      <div v-show="!isListCollapsed" class="section-container">
        <div class="search-section">
          <div class="search-header">
            <h3>搜索和过滤</h3>
            <p class="search-hint">输入关键词快速查找商品，或按分类筛选</p>
          </div>

          <div class="search-box">
            <n-input
              v-model:value="store.searchTerm"
              placeholder="搜索商品名称或编号..."
              clearable
              class="search-input"
            />
            <n-select
              v-model:value="selectedCategory"
              :options="categoryOptions.map(c => ({ label: c, value: c }))"
              clearable
              placeholder="选择分类"
              class="category-select"
            />
            <n-button tertiary @click="handleClearFilters" v-if="store.searchTerm || selectedCategory">
              清空
            </n-button>
          </div>
        </div>

        <div class="filter-options">
          <n-checkbox
            v-model:checked="store.showInactive"
            @update:checked="store.fetchMasterProducts()"
            class="show-inactive-checkbox"
          >
            <span class="checkbox-label">显示已停用的商品</span>
          </n-checkbox>
        </div>

        <n-spin :show="store.isLoading">
          <div v-if="store.error" class="error-message">{{ store.error }}</div>

          <div v-else-if="filteredProducts.length" class="table-wrapper">
            <table class="product-table">
              <thead>
                <tr>
                  <th>图像</th>
                  <th>编号</th>
                  <th>名称</th>
                  <th>默认价格</th>
                  <th>商品分类</th>
                  <th>操作</th>
                </tr>
              </thead>

              <tbody>
                <tr
                  v-for="product in filteredProducts"
                  :key="product.id"
                  :class="{ inactive: !product.is_active }"
                >
                  <td>
                    <n-image
                      v-if="product.image_url"
                      :src="product.image_url"
                      :alt="product.name"
                      class="preview-img"
                      preview-disabled
                      style="width: 80px; height: 80px;"
                      :img-props="{ style: 'width: 100%; height: 100%; object-fit: contain; display: block;' }"
                    />
                    <span v-else class="no-img">无图</span>
                  </td>

                  <td>{{ product.product_code }}</td>
                  <td>{{ product.name }}</td>
                  <td>¥{{ Number(product.default_price ?? 0).toFixed(2) }}</td>
                  <td>{{ product.category || '未分类' }}</td>

                  <td class="action-cell">
                    <n-button size="small" tertiary @click="$emit('edit', product)">编辑</n-button>
                    <n-button
                      size="small"
                      :type="product.is_active ? 'error' : 'success'"
                      tertiary
                      @click="$emit('toggleStatus', product)"
                      style="margin-left: 8px;"
                    >
                      {{ product.is_active ? '停用' : '启用' }}
                    </n-button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <p v-else-if="store.searchTerm && !store.filteredProducts.length">
            没有找到匹配 "<strong>{{ store.searchTerm }}</strong>" 的商品。
          </p>
          <p v-else>商品库为空。</p>
        </n-spin>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useProductStore } from '@/stores/productStore'
import { NInput, NSelect, NImage, NButton, NSpin, NCheckbox } from 'naive-ui'

const store = useProductStore()
defineEmits(['edit', 'toggleStatus'])

const isListCollapsed = ref(false)
const selectedCategory = ref('')

function handleClearFilters() {
  store.searchTerm = ''
  selectedCategory.value = ''
}

const categoryOptions = computed(() => {
  const cats = (store.masterProducts || [])
    .map(p => p.category)
    .filter(cat => !!cat && String(cat).trim() !== '')
  return [...new Set(cats)]
})

const filteredProducts = computed(() => {
  let list = store.filteredProducts || []
  if (selectedCategory.value) {
    list = list.filter(p => p.category === selectedCategory.value)
  }
  return list
})
</script>

<style scoped>
/* ✅ 直接复用你旧版的样式（已去除 sync 相关样式） */
.list-container {
  margin-bottom: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  user-select: none;
  padding: 0.75rem 1rem;
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  transition: all 0.2s ease;
  margin-bottom: 0.5rem;
}

.section-header:hover {
  background: var(--hover-bg-color, var(--card-bg-color));
  border-color: var(--accent-color);
}

.section-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--accent-color);
  font-weight: 600;
}

.toggle-btn {
  font-size: 0.9rem;
  padding: 0.25rem 0.75rem;
  min-width: auto;
  color: var(--accent-color);
}

.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
}

.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 2000px;
}

.section-container {
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
}

.search-section {
  margin-bottom: 1.5rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.search-header {
  margin-bottom: 1rem;
}

.search-header h3 {
  margin: 0;
  color: var(--accent-color);
  font-size: 1rem;
  font-weight: 600;
}

.search-hint {
  margin: 0.5rem 0 0 0;
  color: var(--text-muted);
  font-size: 0.85rem;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 220px;
}

.category-select {
  min-width: 150px;
}

.filter-options {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.show-inactive-checkbox {
  display: flex;
  align-items: center;
}

.checkbox-label {
  color: var(--primary-text-color);
  font-size: 0.9rem;
  margin-left: 0.5rem;
}

.table-wrapper {
  width: 100%;
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}

.product-table {
  width: 100%;
  margin-top: 0;
  border-collapse: collapse;
  border-spacing: 0;
  text-align: left;
  font-size: 0.9rem;
  min-width: 700px;
}

.product-table th {
  padding: 12px 16px;
  background-color: var(--card-bg-color);
  color: var(--primary-text-color);
  font-weight: 600;
  border-bottom: 2px solid var(--accent-color);
  white-space: nowrap;
}

.product-table td {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  color: var(--secondary-text-color);
  vertical-align: middle;
}

.product-table tbody tr {
  transition: background-color 0.2s ease-in-out;
}

.product-table tbody tr:hover {
  background-color: var(--accent-color-light);
}

.product-table th:first-child,
.product-table td:first-child {
  padding-left: 0;
}

.product-table th:last-child,
.product-table td:last-child {
  text-align: right;
  padding-right: 0;
}

.action-cell {
  white-space: nowrap;
}

.preview-img {
  width: 64px;
  height: 64px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  vertical-align: middle;
}

:deep(.preview-img img) {
  width: 100%;
  height: 100%;
  object-fit: contain; /* ✅ 缩略图改为 contain */
  display: block;
  background: var(--bg-color);
}

.no-img {
  display: inline-block;
  width: 50px;
  height: 50px;
  line-height: 50px;
  text-align: center;
  font-size: 0.8rem;
  color: var(--text-disabled);
  background-color: var(--bg-color);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  vertical-align: middle;
}

.inactive {
  opacity: 0.5;
  background-color: var(--bg-elevated);
}
.inactive td {
  text-decoration: line-through;
}

/* 响应式（保留你原来的） */
@media (max-width: 768px) {
  .section-container { padding: 1rem; }
  .search-section { margin-bottom: 1rem; padding-bottom: 1rem; }
  .search-header h3 { font-size: 0.95rem; }
  .search-hint { font-size: 0.8rem; }
  .search-box { gap: 8px; }
  .search-input { min-width: 180px; }
  .category-select { min-width: 120px; }
  .product-table { font-size: 0.85rem; min-width: 650px; }
  .product-table th, .product-table td { padding: 10px 12px; }
  .preview-img { width: 60px; height: 60px; }
  .no-img { width: 60px; height: 60px; line-height: 60px; font-size: 0.75rem; }
}

@media (max-width: 480px) {
  .list-container { margin-bottom: 1.5rem; }
  .section-header { padding: 0.6rem 0.75rem; }
  .section-header h2 { font-size: 1.1rem; }
  .section-container { padding: 0.75rem; }
  .search-section { margin-bottom: 0.75rem; padding-bottom: 0.75rem; }
  .search-header h3 { font-size: 0.9rem; }
  .search-hint { font-size: 0.75rem; }
  .search-box { gap: 6px; }
  .search-input { min-width: 150px; }
  .category-select { min-width: 100px; }
  .filter-options { margin-top: 0.75rem; padding-top: 0.75rem; }
  .checkbox-label { font-size: 0.85rem; }
  .product-table { font-size: 0.75rem; min-width: 600px; }
  .product-table th, .product-table td { padding: 8px 10px; }
  .product-table th { font-size: 0.7rem; }
  .preview-img { width: 50px; height: 50px; }
  .no-img { width: 50px; height: 50px; line-height: 50px; font-size: 0.7rem; }
  .action-cell :deep(.n-button) { font-size: 0.75rem; padding: 4px 8px; }
}

/* 你原先对 category-select 的覆盖保留 */
.category-select {
  margin-left: 0;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  background: var(--card-bg-color);
  color: var(--primary-text-color);
  min-width: 120px;
}
</style>
