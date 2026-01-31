<template>
  <div class="page">
    <header class="page-header">
      <div class="header-content">
        <h1>全局商品库</h1>
        <p>管理可复用的商品模板：创建 / 导入导出 / 搜索编辑。</p>
      </div>
    </header>

    <main class="page-body">
      <CreateMasterProductForm @created="refreshProducts('created')" />

      <BoothpackSyncPanel @imported="refreshProducts('imported')" />

      <MasterProductList
        @edit="openEditModal"
        @toggleStatus="handleToggleStatus"
      />

      <EditMasterProductModal
        :show="isEditModalVisible"
        :product="editableProduct"
        @close="closeEditModal"
        @updated="onProductUpdated"
      />
    </main>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { useMessage } from 'naive-ui'
import { useProductStore } from '@/stores/productStore'

import CreateMasterProductForm from '@/components/product/CreateMasterProductForm.vue'
import BoothpackSyncPanel from '@/components/product/BoothpackSyncPanel.vue'
import MasterProductList from '@/components/product/MasterProductList.vue'
import EditMasterProductModal from '@/components/product/EditMasterProductModal.vue'

defineEmits(['edit', 'toggleStatus'])

const store = useProductStore()

const isEditModalVisible = ref(false)
const editableProduct = ref(null)

function openEditModal(product) {
  editableProduct.value = product
  isEditModalVisible.value = true
}

function closeEditModal() {
  isEditModalVisible.value = false
  editableProduct.value = null
}

async function onProductUpdated() {
  // 最稳：刷新一下列表（避免本地状态与后端不一致）
  await store.fetchMasterProducts()
}
const message = useMessage()

async function refreshProducts(reason = '') {
  await store.fetchMasterProducts()

  if (reason === 'created') {
    message.success('已添加商品，列表已刷新', { duration: 2500, closable: true })
  } else if (reason === 'imported') {
    message.success('已导入数据包，列表已刷新', { duration: 2500, closable: true })
  }
}

onMounted(async () => {
  await store.fetchMasterProducts()
})

</script>

<style scoped>
.page {
  width: 100%;
}

.page-header {
  padding: 1rem 1.25rem;
  margin-bottom: 1rem;
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 10px;
}

.header-content h1 {
  margin: 0;
  font-size: 1.35rem;
  color: var(--accent-color);
  font-weight: 700;
}

.header-content p {
  margin: 0.35rem 0 0 0;
  color: var(--text-muted);
  font-size: 0.9rem;
}

.page-body {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
</style>
