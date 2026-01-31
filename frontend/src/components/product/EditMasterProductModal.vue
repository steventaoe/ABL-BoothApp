<template>
  <AppModal :show="show" @close="handleClose">
    <template #header>
      <h3 style="margin: 0;">编辑商品</h3>
    </template>

    <template #body>
      <form v-if="localProduct" class="edit-form" @submit.prevent="handleUpdate">
        <div class="form-group">
          <label>商品编号:</label>
          <n-input v-model:value="localProduct.product_code" clearable required />
        </div>

        <div class="form-group">
          <label>商品名称:</label>
          <n-input v-model:value="localProduct.name" clearable required />
        </div>

        <div class="form-group">
          <label>默认价格 (¥):</label>
          <n-input-number
            v-model:value="localProduct.default_price"
            :step="0.01"
            :precision="2"
            required
            style="width: 100%;"
          />
        </div>

        <div class="form-group">
          <label>商品分类:</label>
          <n-input
            v-model:value="localProduct.category"
            placeholder="漫画、亚克力、毛绒玩具等"
            clearable
          />
        </div>

        <ImageUploader
          label="更换预览图"
          :initial-image-url="localProduct.image_url"
          v-model="editFormFile"
          @image-removed="handleImageRemoval"
        />

        <p v-if="editError" class="error-message">{{ editError }}</p>
      </form>

      <div v-else class="empty-hint">
        未选择要编辑的商品
      </div>
    </template>

    <template #footer>
      <n-space justify="end">
        <n-button @click="handleClose">取消</n-button>
        <n-button
          type="primary"
          :loading="isUpdating"
          :disabled="isUpdating || !localProduct"
          @click="handleUpdate"
        >
          {{ isUpdating ? '保存中...' : '保存更改' }}
        </n-button>
      </n-space>
    </template>
  </AppModal>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useProductStore } from '@/stores/productStore'
import AppModal from '@/components/shared/AppModal.vue'
import ImageUploader from '@/components/shared/ImageUploader.vue'
import { NInput, NInputNumber, NButton, NSpace } from 'naive-ui'

const props = defineProps({
  show: { type: Boolean, default: false },
  // 列表行点击传入的 product
  product: { type: Object, default: null }
})

const emit = defineEmits(['close', 'updated'])

const store = useProductStore()

const isUpdating = ref(false)
const editError = ref('')

// 本地副本（避免直接改 props）
const localProduct = ref(null)

// ImageUploader v-model 文件
const editFormFile = ref(null)
// 用户是否点击“移除图片”
const isImageRemovedForEdit = ref(false)

// 当外部 product 变化：重置本地表单
watch(
  () => props.product,
  (p) => {
    if (!p) {
      localProduct.value = null
      editFormFile.value = null
      isImageRemovedForEdit.value = false
      editError.value = ''
      return
    }
    localProduct.value = { ...p }
    editFormFile.value = null
    isImageRemovedForEdit.value = false
    editError.value = ''
  },
  { immediate: true }
)

// 当 modal 关闭：清理一下状态（下次打开不残留）
watch(
  () => props.show,
  (v) => {
    if (!v) {
      editError.value = ''
      editFormFile.value = null
      isImageRemovedForEdit.value = false
    }
  }
)

function handleImageRemoval() {
  isImageRemovedForEdit.value = true
}

function handleClose() {
  emit('close')
}

async function handleUpdate() {
  if (!localProduct.value || isUpdating.value) return

  isUpdating.value = true
  editError.value = ''

  try {
    const formData = new FormData()

    const eCode = String(localProduct.value.product_code || '').trim()
    const eName = String(localProduct.value.name || '').trim()
    const ePrice = localProduct.value.default_price
    const eCategory = String(localProduct.value.category ?? '').trim()

    if (!eCode || !eName || ePrice == null) {
      throw new Error('请填写商品编号、名称和默认价格')
    }

    formData.append('product_code', eCode)
    formData.append('name', eName)
    formData.append('default_price', String(ePrice))
    if (eCategory) formData.append('category', eCategory)

    // ✅ 与旧版一致：有新图传 image；否则如果点了移除，传 remove_image=true
    if (editFormFile.value) {
      formData.append('image', editFormFile.value)
    } else if (isImageRemovedForEdit.value) {
      formData.append('remove_image', 'true')
    }

    await store.updateMasterProduct(localProduct.value.id, formData)

    emit('updated')
    emit('close')
  } catch (err) {
    editError.value = err?.message || '保存失败'
  } finally {
    isUpdating.value = false
  }
}
</script>

<style scoped>
.edit-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.form-group {
  display: flex;
  flex-direction: column;
}

label {
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.error-message {
  color: var(--error-color);
  margin-top: 0.25rem;
}

.empty-hint {
  color: var(--text-muted);
  padding: 0.75rem 0;
}
</style>
