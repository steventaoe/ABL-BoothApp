<template>
  <section class="form-container">
    <div class="section-header" @click="isCollapsed = !isCollapsed">
      <h2>创建新展会</h2>
      <n-button text class="toggle-btn">
        {{ isCollapsed ? '展开' : '折叠' }}
      </n-button>
    </div>
    <transition name="expand">
      <div v-show="!isCollapsed" class="section-container">
        <n-form @submit.prevent class="two-column-form">
      <div class="form-group">
        <label for="name">展会名称:</label>
        <n-input id="name" v-model:value="formData.name" placeholder="例如：COMICUP 31" />
      </div>
      <div class="form-group">
        <label for="date">日期:</label>
        <n-date-picker id="date" v-model:formatted-value="formData.date" type="date" value-format="yyyy-MM-dd" />
      </div>
      <div class="form-group">
        <label for="location">地点:</label>
        <n-input id="location" v-model:value="formData.location" placeholder="例如：上海" />
      </div>
      <div class="form-group">
        <label for="vendor_password">摊主密码 (可选):</label>
        <n-input id="vendor_password" v-model:value="formData.vendor_password" placeholder="留空则使用全局密码" />
      </div>

      <div class="form-group full-width">
        <ImageUploader 
          label="展会收款码图片 (可选)"
          v-model="paymentQrCodeFile"
        />
      </div>

      <div class="form-actions full-width">
        <n-button type="primary" :loading="isSubmitting" @click="handleSubmit">
          {{ isSubmitting ? '创建中...' : '创建' }}
        </n-button>
        <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
      </div>
    </n-form>
      </div>
    </transition>
  </section>
</template>

<script setup>
import { ref } from 'vue';
import { useEventStore } from '@/stores/eventStore';
import ImageUploader from '@/components/shared/ImageUploader.vue';
import { NForm, NInput, NDatePicker, NButton } from 'naive-ui';

const store = useEventStore();
const isSubmitting = ref(false);
const errorMessage = ref('');

const isCollapsed = ref(false);

const formData = ref({
  name: '',
  date: null,
  location: '',
  vendor_password: '' 
});

// 【修改】只保留 v-model 需要的 ref
const paymentQrCodeFile = ref(null);


async function handleSubmit() {
  // 简单必填校验，保持与原先 required 一致
  if (!formData.value.name || !formData.value.date) {
    errorMessage.value = '展会名称和日期不能为空。';
    return;
  }
  isSubmitting.value = true;
  errorMessage.value = '';

  const submissionData = new FormData();
  submissionData.append('name', formData.value.name);
  submissionData.append('date', formData.value.date);
  submissionData.append('location', formData.value.location);
  submissionData.append('vendor_password', formData.value.vendor_password);
  
  if (paymentQrCodeFile.value) {
    submissionData.append('payment_qr_code', paymentQrCodeFile.value);
  }

  try {
    await store.createEvent(submissionData);
    
    formData.value = { name: '', date: null, location: '', vendor_password: '' };
    paymentQrCodeFile.value = null; 
    
  } catch (error) {
    errorMessage.value = error.message;
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<style scoped>
.form-container {
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
  max-height: 0;
  opacity: 0;
}

.expand-enter-to,
.expand-leave-from {
  max-height: 2000px;
  opacity: 1;
}

.section-container {
  background-color: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
}

/* 【新增】使用 CSS Grid 实现两列布局 */
.two-column-form {
  display: grid;
  /* 创建两列，每列占据相等的剩余空间 */
  grid-template-columns: 1fr 1fr;
  /* 定义列间距和行间距 */
  gap: 0.6rem 1rem;
  padding: 0;
}

.full-width {
  /* 从第1条网格线跨越到最后1条 (-1) */
  grid-column: 1 / -1;
}

.form-group {
  margin-bottom: 0;
  display: flex;
  flex-direction: column;
}

label {
  display: block; /* 确保 label 独占一行 */
  margin-bottom: 0.3rem;
  font-size: 0.95em;
  font-weight: 500;
}

input[type="text"],
input[type="date"] {
  width: 100%;
  background-color: var(--bg-color);
  border: 1px solid var(--border-color);
  color: var(--primary-text-color);
  padding: 6px 8px;
  border-radius: 3px;
  font-size: 0.96em;
  height: 32px;
  box-sizing: border-box;
}

/* Naive UI 组件样式 */
:deep(.n-input) {
  width: 100%;
}

:deep(.n-date-picker) {
  width: 100%;
}

:deep(.n-input__input-el) {
  width: 100%;
}

.form-actions :deep(.n-button) {
  min-width: 80px;
}

/* 【新增】提交按钮和错误信息的容器 */
.form-actions {
  margin-top: 0.5rem; /* 与上方元素留出一些间距 */
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
}

button,
.btn {
  padding: 6px 14px;
  font-size: 0.96em;
  border-radius: 3px;
}

.error-message {
  margin-top: 0.5rem;
  font-size: 0.95em;
  /* 确保错误信息在按钮下方 */
  width: 100%; 
  color: var(--error-color);
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 平板适配 (768px 以下) */
@media (max-width: 768px) {
  .section-container {
    padding: 1rem;
  }

  .two-column-form {
    grid-template-columns: 1fr;
    gap: 0.8rem;
  }

  label {
    font-size: 0.9rem;
    margin-bottom: 0.4rem;
  }

  input[type="text"],
  input[type="date"] {
    padding: 8px;
    font-size: 16px; /* 防止移动设备自动缩放 */
    height: 36px;
  }
}

/* 手机适配 (480px 以下) */
@media (max-width: 480px) {
  .section-container {
    padding: 1rem;
  }

  .two-column-form {
    gap: 0.6rem;
  }

  .form-group {
    margin-bottom: 0;
  }

  label {
    font-size: 0.85rem;
    margin-bottom: 0.3rem;
    font-weight: 500;
  }

  input[type="text"],
  input[type="date"] {
    padding: 8px;
    font-size: 16px;
    height: 40px;
    border-radius: 3px;
  }

  .form-actions {
    margin-top: 1rem;
    gap: 0.5rem;
  }

  .error-message {
    font-size: 0.8rem;
    margin-top: 0.5rem;
  }

  button,
  .btn {
    padding: 8px 12px;
    font-size: 0.9rem;
    flex-grow: 1;
  }
}

/* ImageUploader 的内部样式由其自身 scoped CSS 控制，这里无需再写 */
</style>