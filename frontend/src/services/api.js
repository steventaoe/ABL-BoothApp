import axios from 'axios';
import router from '@/router';

// ============================================================
// 1. 动态计算 BaseURL (核心修复)
// ============================================================

// 检测是否在 Tauri 容器内运行
const isTauri = window.__TAURI_INTERNALS__ !== undefined;
// 你的后端端口
const API_PORT = 5000;

let baseURL = '';

if (import.meta.env.DEV) {
  // A. 开发环境：走 Vite 代理 (vite.config.ts)
  baseURL = '/api';
} else if (isTauri) {
  // B. Tauri 生产环境：强制指向本地后端
  // 必须写完整 URL，否则 Tauri 可能会在 tauri://localhost 下寻找 /api
  baseURL = `http://localhost:${API_PORT}/api`;
} else {
  // C. 手机/局域网浏览器环境：使用相对路径
  // 浏览器会自动拼接当前 IP (例如 http://192.168.1.5:5000/api)
  baseURL = '/api';
}

console.log(`[Config] Axios BaseURL set to: ${baseURL}`);

const apiClient = axios.create({
  baseURL: baseURL,
  withCredentials: true, // 确保 Cookie 跨域传输
  timeout: 15000, // 建议设置超时，防止网络卡死
});

// ============================================================
// 2. 拦截器保持不变 (写得很好)
// ============================================================

// 添加请求拦截器，用于调试
apiClient.interceptors.request.use(
  (config) => {
    if (import.meta.env.DEV) {
      console.log(`🚀 API请求: ${config.method?.toUpperCase()} ${config.baseURL}${config.url}`);
      // 调试：如果是 FormData，打印所有键值，便于核对字段名是否匹配后端
      if (config.data instanceof FormData) {
        const entries = [];
        for (const [k, v] of config.data.entries()) {
          entries.push([k, v instanceof Blob ? `(Blob:${v.type||'unknown'})` : String(v)]);
        }
        console.log('📦 FormData payload:', entries);
      }
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// 添加响应拦截器，用于调试
apiClient.interceptors.response.use(
  (response) => {
    if (import.meta.env.DEV) {
      console.log(`✅ API响应: ${response.config.method?.toUpperCase()} ${response.config.url}`, response.status);
    }
    return response;
  },
  (error) => {
    const url = error.config?.url || 'unknown';
    const status = error.response?.status || 'network_error';
    
    if (import.meta.env.DEV) {
      console.error(`❌ API错误: ${url}`, status, error.message);
    }

    if (error.response && [401, 403].includes(status)) {
      // 防止重复跳转（可选优化）
      const currentPath = router.currentRoute.value.path;
      if (!currentPath.includes('/login')) {
        // 清理本地登录状态并跳转到登录页
        sessionStorage.removeItem('user');
        const role = router.currentRoute.value.meta?.role;
        const target = role === 'vendor' ? '/login/vendor' : '/login/admin';
        router.push(target);
      }
    }
    return Promise.reject(error);
  }
);

export default apiClient;