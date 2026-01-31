<template>
  <!-- 创建表单 -->
  <div class="form-section">
    <div class="section-header" @click="isFormExpanded = !isFormExpanded">
      <h2>添加新商品到仓库</h2>
      <n-button text class="toggle-btn">
        {{ isFormExpanded ? '折叠' : '展开' }}
      </n-button>
    </div>

    <transition name="expand">
      <div v-show="isFormExpanded" class="form-wrapper">
        <n-card class="form-container" size="small">
          <form @submit.prevent="handleCreate">
            <div class="form-grid">
              <div class="form-group">
                <label for="create-code">商品编号:</label>
                <n-input
                  id="create-code"
                  v-model:value="createFormData.product_code"
                  placeholder="A01"
                  clearable
                  required
                />
              </div>

              <div class="form-group">
                <label for="create-name">商品名称:</label>
                <n-input
                  id="create-name"
                  v-model:value="createFormData.name"
                  placeholder="灵梦亚克力立牌"
                  clearable
                  required
                />
              </div>

              <div class="form-group">
                <label for="create-price">默认价格 (¥):</label>
                <n-input-number
                  id="create-price"
                  v-model:value="createFormData.default_price"
                  :step="0.01"
                  placeholder="45.00"
                  required
                />
              </div>

              <div class="form-group">
                <label for="create-category">商品分类:</label>
                <n-input
                  id="create-category"
                  v-model:value="createFormData.category"
                  placeholder="漫画、亚克力、毛绒玩具等"
                  clearable
                />
              </div>
            </div>

            <!-- 使用可复用的 ImageUploader 组件 -->
            <ImageUploader
              label="预览图"
              v-model="createFormFile"
            />

            <n-button type="primary" attr-type="submit" :disabled="isCreating">
              {{ isCreating ? '添加中...' : '添加到仓库' }}
            </n-button>

            <p v-if="createError" class="error-message">{{ createError }}</p>
          </form>
        </n-card>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useProductStore } from '@/stores/productStore'
import ImageUploader from '@/components/shared/ImageUploader.vue'
import { NCard, NInput, NInputNumber, NButton } from 'naive-ui'

const emit = defineEmits(['created'])

const store = useProductStore()

// --- 创建逻辑的状态 ---
const isCreating = ref(false)
const createError = ref('')

// ✅ 修复：补上 category 字段（你旧代码漏了，但模板在用）
const createFormData = ref({
  product_code: '',
  name: '',
  default_price: null,
  category: ''
})

// ImageUploader 会通过 v-model 更新这个
const createFormFile = ref(null)

const isFormExpanded = ref(true)

async function handleCreate() {
  isCreating.value = true
  createError.value = ''

  try {
    // 构建 FormData（逻辑保持不变）
    const formData = new FormData()

    const code = String(createFormData.value.product_code || '').trim()
    const name = String(createFormData.value.name || '').trim()
    const price = createFormData.value.default_price
    const category = String(createFormData.value.category ?? '').trim()

    // ✅ 小兜底：避免空格提交
    if (!code || !name || price == null) {
      throw new Error('请填写商品编号、名称和默认价格')
    }

    formData.append('product_code', code)
    formData.append('name', name)
    formData.append('default_price', String(price))
    if (category) formData.append('category', category)

    if (createFormFile.value) {
      // 字段名保持你原版本：image
      formData.append('image', createFormFile.value)
    }

    // 假设 store action 已被修改为接收 FormData
    await store.createMasterProduct(formData)

    // 成功后重置表单
    createFormData.value = { product_code: '', name: '', default_price: null, category: '' }
    createFormFile.value = null

    // ✅ 通知父组件刷新列表（不强制，父组件可选择监听）
    emit('created')
  } catch (error) {
    createError.value = error?.message || '添加失败'
  } finally {
    isCreating.value = false
  }
}
</script>

<style scoped>
/* 通用区块样式（保持你原 AdminMasterProduct.vue 风格） */
.form-section {
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

/* 展开/折叠动画 */
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

/* 表单容器（保持原版） */
.form-wrapper {
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
}

.form-container {
  background-color: transparent;
  border: none;
  padding: 0;
  border-radius: 0;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 1.5rem;
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
  margin-top: 1rem;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
