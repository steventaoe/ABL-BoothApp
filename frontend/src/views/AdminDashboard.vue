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
        <!-- 安全设置 -->
         <n-card title="手机/平板扫码访问" size="small">
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
                <qrcode-vue :value="serverInfo.order_url" :size="180" level="M"/>
                <a :href="serverInfo.order_url" target="_blank">{{ serverInfo.order_url }}</a>
              </div>
              <div class="qr-card">
                <div class="qr-title">摊主入口</div>
                <qrcode-vue :value="serverInfo.vendor_url" :size="180" level="M"/>
                <a :href="serverInfo.vendor_url" target="_blank">{{ serverInfo.vendor_url }}</a>
              </div>
              <div class="qr-card">
                <div class="qr-title">管理员入口</div>
                <qrcode-vue :value="serverInfo.admin_url" :size="180" level="M"/>
                <a :href="serverInfo.admin_url" target="_blank">{{ serverInfo.admin_url }}</a>
              </div>
            </div>
          </n-space>
        </n-card>
        
        <n-card title="安全设置" size="small">
          <template #header-extra>
            <n-button text size="tiny" @click="securityCollapsed = !securityCollapsed">
              {{ securityCollapsed ? '展开' : '收起' }}
            </n-button>
          </template>
          <div v-if="!securityCollapsed" class="settings-grid">
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
        </n-card>

        <!-- 创建展会的表单组件 -->
        <CreateEventForm />
        
        <!-- 局域网扫码访问 在网站部署版本里面要隐藏 -->
        
        
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
import { NSpace, NCard, NButton, NAlert, NForm, NFormItem, NInput, NModal } from 'naive-ui';
import QrcodeVue from 'qrcode.vue';
import api from '@/services/api';
import { ref } from 'vue';

const isFetching = ref(false);
const fetchError = ref('');
const serverInfo = ref(null);

const adminForm = ref({ oldPassword: '', newPassword: '' });
const vendorForm = ref({ newPassword: '' });
const adminSaving = ref(false);
const vendorSaving = ref(false);
const adminMessage = ref('');
const vendorMessage = ref('');
const securityCollapsed = ref(false);
const showThemeModal = ref(false);

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
    adminMessage.value = { type: 'success', text: '管理员密码已更新' };
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
</script>

<style scoped>
.dashboard-container {
  max-width: 800px;
  margin: 0 auto;
}
header {
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}
header .header-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}
header h1 {
  color: var(--accent-color);
  margin: 0;
}
header p {
  color: var(--text-muted);
  margin-top: 0.5rem;
}
header .header-actions {
  display: flex;
  gap: 0.5rem;
}

.qr-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}
.hint {
  color: var(--text-muted);
  font-size: 0.9rem;
}
.qr-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
  margin-top: 0.5rem;
}
.qr-card {
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 0.75rem;
  background: var(--card-bg-color);
  text-align: center;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.qr-title { font-weight: 600; }
.qr-card canvas, .qr-card img {
  width: 180px;
  height: 180px;
  align-self: center;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--text-white);
}
.qr-card a {
  word-break: break-all;
  color: var(--accent-color);
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
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
</style>