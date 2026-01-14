<template>
  <div class="dashboard-container">
    <header>
      <div class="header-row">
        <div>
          <h1>管理后台</h1>
          <p>在这里管理您的所有展会和商品。</p>
        </div>
        <div class="header-actions">
          <n-button secondary type="primary" @click="showThemeModal = true">主题设置</n-button>
        </div>
      </div>
    </header>

    <n-modal
      v-model:show="showThemeModal"
      preset="card"
      title="主题设置"
      :mask-closable="false"
      style="max-width: 960px"
      class="theme-modal"
    >
      <ThemeSetting />
    </n-modal>
    <main>
      <n-space vertical size="large">
        <!-- 手机/平板扫码访问 -->
        <section class="qr-section">
          <div class="section-header" @click="qrCollapsed = !qrCollapsed">
            <h2>手机/平板扫码访问</h2>
            <n-button text class="toggle-btn">
              {{ qrCollapsed ? '展开' : '折叠' }}
            </n-button>
          </div>
          <transition name="expand">
            <div v-show="!qrCollapsed" class="section-container">
              <n-space vertical size="small">
                <div class="qr-actions">
                  <n-button type="primary" :loading="isFetching" @click="fetchServerInfo">
                    {{ isFetching ? '获取中...' : '获取局域网二维码' }}
                  </n-button>
                  <span class="hint">同一局域网内扫码可直接访问对应页面, 若无法连接请检查主机的防火墙设置</span>
                </div>
                <n-alert v-if="fetchError" type="error" :bordered="false">{{ fetchError }}</n-alert>
                <div v-if="serverInfo" class="qr-grid">
                  <div class="qr-card">
                    <div class="qr-title">顾客入口</div>
                    <div class="qr-content">
                      <qrcode-vue :value="serverInfo.order_url" :size="180" level="M" class="qr-code"/>
                    </div>
                    <div class="qr-url">{{ serverInfo.order_url }}</div>
                    <n-button type="primary" size="small" @click="copyToClipboard(serverInfo.order_url)" class="copy-btn">
                      点击复制链接
                    </n-button>
                  </div>
                  <div class="qr-card">
                    <div class="qr-title">摊主入口</div>
                    <div class="qr-content">
                      <qrcode-vue :value="serverInfo.vendor_url" :size="180" level="M" class="qr-code"/>
                    </div>
                    <div class="qr-url">{{ serverInfo.vendor_url }}</div>
                    <n-button type="primary" size="small" @click="copyToClipboard(serverInfo.vendor_url)" class="copy-btn">
                      点击复制链接
                    </n-button>
                  </div>
                  <div class="qr-card">
                    <div class="qr-title">管理员入口</div>
                    <div class="qr-content">
                      <qrcode-vue :value="serverInfo.admin_url" :size="180" level="M" class="qr-code"/>
                    </div>
                    <div class="qr-url">{{ serverInfo.admin_url }}</div>
                    <n-button type="primary" size="small" @click="copyToClipboard(serverInfo.admin_url)" class="copy-btn">
                      点击复制链接
                    </n-button>
                  </div>
                </div>
              </n-space>
            </div>
          </transition>
        </section>
        
        <!-- 安全设置 -->
        <section class="security-section">
          <div class="section-header" @click="securityCollapsed = !securityCollapsed">
            <h2>安全设置</h2>
            <n-button text class="toggle-btn">
              {{ securityCollapsed ? '展开' : '折叠' }}
            </n-button>
          </div>
          <transition name="expand">
            <div v-show="!securityCollapsed" class="section-container">
              <div class="settings-grid">
                <div class="settings-card">
                  <div class="settings-title">管理员密码</div>
                  <n-form :model="adminForm" label-placement="top">
                    <n-form-item label="旧密码">
                      <n-input v-model:value="adminForm.oldPassword" type="password" placeholder="请输入旧密码" />
                    </n-form-item>
                    <n-form-item label="新密码 (至少 4 位)">
                      <n-input v-model:value="adminForm.newPassword" type="password" placeholder="请输入新密码" />
                    </n-form-item>
                    <n-space justify="end">
                      <n-button type="primary" :loading="adminSaving" @click="updateAdminPassword">保存</n-button>
                    </n-space>
                  </n-form>
                  <n-alert v-if="adminMessage" :type="adminMessage.type" :bordered="false" class="mt-8">{{ adminMessage.text }}</n-alert>
                </div>

                <div class="settings-card">
                  <div class="settings-title">默认摊主密码(未配置摊主密码时采用)</div>
                  <n-form :model="vendorForm" label-placement="top">
                    <n-form-item label="新密码 (至少 4 位)">
                      <n-input v-model:value="vendorForm.newPassword" type="password" placeholder="请输入新密码" />
                    </n-form-item>
                    <n-space justify="end">
                      <n-button type="primary" :loading="vendorSaving" @click="updateVendorPassword">保存</n-button>
                    </n-space>
                  </n-form>
                  <n-alert v-if="vendorMessage" :type="vendorMessage.type" :bordered="false" class="mt-8">{{ vendorMessage.text }}</n-alert>
                </div>
              </div>
            </div>
          </transition>
        </section>

        <!-- 创建展会的表单组件 -->
        <CreateEventForm />
        
        <!-- 显示展会列表的组件 -->
        <EventList />
      </n-space>
    </main>
  </div>
</template>

<script setup>
// 导入需要的组件
import CreateEventForm from '@/components/event/CreateEventForm.vue';
import EventList from '@/components/event/EventList.vue';
import ThemeSetting from '@/views/ThemeSetting.vue';
import { NSpace, NCard, NButton, NAlert, NForm, NFormItem, NInput, NModal, useMessage } from 'naive-ui';
import QrcodeVue from 'qrcode.vue';
import api from '@/services/api';
import { ref } from 'vue';
import { useAuthStore } from '@/stores/authStore';

const isFetching = ref(false);
const fetchError = ref('');
const serverInfo = ref(null);
const message = useMessage();

const adminForm = ref({ oldPassword: '', newPassword: '' });
const vendorForm = ref({ newPassword: '' });
const adminSaving = ref(false);
const vendorSaving = ref(false);
const adminMessage = ref('');
const vendorMessage = ref('');
const securityCollapsed = ref(false);
const qrCollapsed = ref(false);
const showThemeModal = ref(false);
const authStore = useAuthStore();

// 本地二维码组件由 qrcode.vue 渲染

async function fetchServerInfo() {
  isFetching.value = true;
  fetchError.value = '';
  try {
    const { data } = await api.get('/server-info');
    serverInfo.value = data;
  } catch (err) {
    fetchError.value = err.response?.data?.error || '获取服务器信息失败，请检查后端服务';
  } finally {
    isFetching.value = false;
  }
}

async function updateAdminPassword() {
  adminMessage.value = '';
  adminSaving.value = true;
  try {
    await api.put('/admin/password', {
      oldPassword: adminForm.value.oldPassword,
      newPassword: adminForm.value.newPassword,
    });

    // 密码更新成功后尝试自动重新登录，确保新的 Cookie 生效
    const newPwd = adminForm.value.newPassword;
    adminMessage.value = { type: 'success', text: '管理员密码已更新，正在重新登录…' };

    // 小工具函数：延迟
    const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

    try {
      // 先尝试在服务器侧注销旧会话（忽略失败），避免旧 Cookie 干扰
      try { await api.post('/auth/logout'); } catch (_) {}

      // 给后端一点时间提交事务/刷新会话材料
      await sleep(300);

      // 指数退避重试：最多 3 次（~300ms, ~600ms, ~1200ms）
      let lastErr = null;
      for (let attempt = 1; attempt <= 3; attempt++) {
        try {
          await authStore.login(newPwd, 'admin', null, '/admin');
          adminMessage.value = { type: 'success', text: '管理员密码已更新并已重新登录' };
          lastErr = null;
          break;
        } catch (e) {
          lastErr = e;
          // 间隔递增后再试
          await sleep(300 * Math.pow(2, attempt - 1));
        }
      }
      if (lastErr) throw lastErr;
    } catch (loginErr) {
      console.error('Auto re-login failed after password change', loginErr);
      adminMessage.value = { type: 'error', text: '密码已更新，但自动登录失败，请用新密码重新登录。' };
    }

    adminForm.value.oldPassword = '';
    adminForm.value.newPassword = '';
  } catch (err) {
    adminMessage.value = { type: 'error', text: err.response?.data?.error || '更新失败，请重试' };
  } finally {
    adminSaving.value = false;
  }
}

async function updateVendorPassword() {
  vendorMessage.value = '';
  vendorSaving.value = true;
  try {
    await api.put('/admin/vendor-default-password', {
      newPassword: vendorForm.value.newPassword,
    });
    vendorMessage.value = { type: 'success', text: '默认摊主密码已更新' };
    vendorForm.value.newPassword = '';
  } catch (err) {
    vendorMessage.value = { type: 'error', text: err.response?.data?.error || '更新失败，请重试' };
  } finally {
    vendorSaving.value = false;
  }
}

async function copyToClipboard(text) {
  try {
    await navigator.clipboard.writeText(text);
    message.success('链接已复制到剪贴板');
  } catch (err) {
    console.error('Failed to copy:', err);
    message.error('复制失败');
  }
}
</script>

<style scoped>
.dashboard-container {
  margin: 0 auto;
  padding: 0 1rem;
}
header {
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}
header .header-row {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}
header h1 {
  color: var(--accent-color);
  margin: 0;
  flex: 1 1 auto;
  min-width: 0;
}
header p {
  color: var(--text-muted);
  margin-top: 0.5rem;
}
header .header-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  flex: 0 0 auto;
}

/* 通用区块样式 - 与 AdminMasterProducts 统一 */
.qr-section,
.security-section {
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

.section-container {
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
}

.qr-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 1rem;
}

.hint {
  color: var(--secondary-text-color);
  font-size: 0.9rem;
  flex: 1 1 auto;
  min-width: 200px;
}

.qr-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-top: 1rem;
}

.qr-card {
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.25rem;
  background: var(--card-bg-color);
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  transition: all 0.2s ease;
}

.qr-card:hover {
  border-color: var(--accent-color);
  box-shadow: 0 4px 12px var(--shadow-color);
}

.qr-title {
  font-weight: 700;
  font-size: 1rem;
  color: var(--accent-color);
}

.qr-content {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0.75rem;
  background: var(--text-white);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.qr-code {
  width: 180px !important;
  height: 180px !important;
}

.qr-url {
  font-size: 0.85rem;
  color: var(--text-muted);
  word-break: break-all;
  line-height: 1.4;
  max-width: 100%;
  padding: 0.5rem 0;
}

.copy-btn {
  width: 100%;
  margin-top: 0.5rem;
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 1rem;
}

.settings-card {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  background: var(--card-bg-color);
}

.settings-title {
  font-weight: 600;
  margin-bottom: 0.75rem;
}

.mt-8 {
  margin-top: 0.5rem;
}

.theme-modal :deep(.n-card__content) {
  padding-top: 0;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .dashboard-container {
    padding: 0 0.5rem;
  }

  header {
    margin-bottom: 1.5rem;
    padding-bottom: 0.75rem;
  }

  header .header-row {
    flex-direction: column;
    align-items: stretch;
  }

  header h1 {
    font-size: 1.5rem;
  }

  header .header-actions {
    justify-content: flex-start;
  }

  .qr-actions {
    flex-direction: column;
    align-items: flex-start;
  }

  .hint {
    min-width: auto;
    width: 100%;
  }

  .qr-grid {
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 1rem;
  }

  .qr-card {
    padding: 1rem;
  }

  .qr-code {
    width: 140px !important;
    height: 140px !important;
  }

  .qr-title {
    font-size: 0.95rem;
  }

  .qr-url {
    font-size: 0.8rem;
  }

  .settings-grid {
    grid-template-columns: 1fr;
  }

  .settings-card {
    padding: 0.75rem;
  }

  .settings-title {
    margin-bottom: 0.5rem;
    font-size: 0.95rem;
  }
}

@media (max-width: 480px) {
  .dashboard-container {
    padding: 0 0.25rem;
  }

  header {
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
  }

  header h1 {
    font-size: 1.25rem;
  }

  header p {
    font-size: 0.85rem;
  }

  .qr-grid {
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }

  .qr-card {
    padding: 0.75rem;
  }

  .qr-code {
    width: 120px !important;
    height: 120px !important;
  }

  .qr-title {
    font-size: 0.9rem;
  }

  .qr-url {
    font-size: 0.75rem;
  }

  .copy-btn {
    font-size: 0.85rem;
  }

  .hint {
    font-size: 0.8rem;
  }

  :deep(.n-card) {
    margin-bottom: 1rem;
  }

  :deep(.n-form-item) {
    margin-bottom: 0.8rem;
  }

  :deep(.n-input) {
    font-size: 16px;
  }
}
</style>