<template>
  <n-modal
    v-model:show="showModal"
    preset="card"
    title="检查更新"
    class="update-modal"
    :mask-closable="!loading"
    :close-on-esc="!loading"
    @after-enter="handleEnter"
  >
    <!-- 状态 1: 加载中 -->
    <div v-if="loading" class="state-container">
      <n-spin size="large" />
      <p class="mt-4 text-gray-500">正在获取最新版本信息...</p>
    </div>

    <!-- 状态 2: 出错 -->
    <div v-else-if="error" class="state-container">
      <n-result status="warning" title="检查失败" :description="error">
        <template #footer>
          <n-button @click="retry">重试</n-button>
        </template>
      </n-result>
    </div>

    <!-- 状态 3: 已经是最新版 -->
    <div v-else-if="!hasUpdate" class="state-container">
      <n-result
        status="success"
        title="当前已是最新版本"
        :description="`版本号: v${currentVersion}`"
      >
        <template #footer>
          <n-button @click="close">关闭</n-button>
        </template>
      </n-result>
    </div>

    <!-- 状态 4: 发现新版本 -->
    <div v-else class="update-content">
      <div class="header-section">
        <n-tag type="success" size="large">发现新版本 v{{ latestVersion }}</n-tag>
        <span class="date">{{ formatDate(releaseDate) }}</span>
      </div>

      <div class="current-ver-tip">
        当前版本: v{{ currentVersion }}
      </div>

      <n-divider title-placement="left" style="margin: 12px 0;">更新内容</n-divider>

      <!-- 更新日志滚动区域 -->
      <n-scrollbar style="max-height: 200px" class="log-scroll">
        <div class="release-note">{{ releaseNote }}</div>
      </n-scrollbar>

      <div class="actions">
        <n-button @click="close" ghost>暂不更新</n-button>
        <n-button type="primary" @click="handleDownload">
          前往下载
        </n-button>
      </div>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useUpdateCheck } from '@/composables/useUpdateCheck';
import { 
  NModal, NSpin, NResult, NButton, NTag, NDivider, NScrollbar 
} from 'naive-ui';

const props = defineProps<{
  show: boolean;
}>();

const emit = defineEmits(['update:show']);

// 双向绑定 show
const showModal = computed({
  get: () => props.show,
  set: (val) => emit('update:show', val),
});

// 引入逻辑 Hook
const {
  loading,
  error,
  hasUpdate,
  currentVersion,
  latestVersion,
  releaseNote,
  releaseDate,
  checkUpdate,
  goToDownload,
} = useUpdateCheck();

// 当模态框打开时，自动开始检查
const handleEnter = () => {
  checkUpdate();
};

const retry = () => {
  checkUpdate();
};

const close = () => {
  showModal.value = false;
};

const handleDownload = () => {
  goToDownload();
  // 可选：点击下载后关闭弹窗
  // showModal.value = false;
};

// 简单的日期格式化
const formatDate = (dateStr: string) => {
  if (!dateStr) return '';
  return new Date(dateStr).toLocaleDateString();
};
</script>

<style scoped>
/* 样式适配移动端和桌面端 */
.update-modal {
  width: 90%;
  max-width: 450px; /* 电脑上不要太宽，手机上占90% */
  margin: 0 auto;
}

.state-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px 0;
  min-height: 200px;
}

.mt-4 { margin-top: 16px; }
.text-gray-500 { color: var(--text-muted); }

.update-content {
  padding-top: 10px;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.date {
  font-size: 12px;
  color: var(--secondary-text-color);
}

.current-ver-tip {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 8px;
}

.release-note {
  white-space: pre-wrap; /* 保留换行符 */
  font-family: v-sans, system-ui, -apple-system, sans-serif;
  line-height: 1.6;
  color: var(--primary-text-color);
  background: var(--input-bg-color);
  padding: 10px;
  border-radius: 6px;
  font-size: 13px;
  border: 1px solid var(--border-color);
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 20px;
}

/* 响应式布局 - 平板及以上 */
@media (max-width: 768px) {
  .update-modal {
    width: 85%;
    max-width: 400px;
    margin: 0 auto;
  }

  .state-container {
    padding: 16px 0;
    min-height: 160px;
  }

  .header-section {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .date {
    font-size: 11px;
    align-self: flex-start;
  }

  .current-ver-tip {
    font-size: 11px;
    margin-top: 6px;
  }

  .release-note {
    font-size: 12px;
    padding: 8px;
  }

  .actions {
    flex-direction: column-reverse;
    gap: 8px;
  }

  .actions :deep(.n-button) {
    width: 100%;
  }
}

/* 响应式布局 - 手机 */
@media (max-width: 480px) {
  .update-modal {
    width: 90%;
    max-width: calc(100% - 32px);
    margin: 0 16px;
    border-radius: 12px 12px 0 0;
  }

  .state-container {
    padding: 12px 0;
    min-height: 140px;
  }

  .mt-4 {
    margin-top: 12px;
  }

  .header-section {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .date {
    font-size: 10px;
  }

  .current-ver-tip {
    font-size: 10px;
    margin-top: 4px;
  }

  .release-note {
    font-size: 11px;
    padding: 6px;
    line-height: 1.4;
  }

  .update-content {
    padding-top: 6px;
  }

  .actions {
    flex-direction: column-reverse;
    gap: 6px;
  }

  .actions :deep(.n-button) {
    width: 100%;
    font-size: 14px;
  }

  :deep(.n-modal__body) {
    padding: 12px;
  }

  :deep(.n-modal__header) {
    padding: 12px;
  }

  :deep(.n-divider) {
    margin: 8px 0 !important;
  }
}
</style>