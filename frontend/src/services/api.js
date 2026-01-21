import axios from 'axios';
import { fetch } from '@tauri-apps/plugin-http'; 
import router from '@/router';

// ============================================================
// 1. 环境判断与 BaseURL 配置
// ============================================================

const isTauri = window.__TAURI_INTERNALS__ !== undefined;
const API_PORT = 5140;

let baseURL = '';

// [Debug] 强制指定 IP，排除 DNS 干扰
if (isTauri) {
  // 建议：在安卓真机调试时，尝试用 127.0.0.1
  baseURL = `http://127.0.0.1:${API_PORT}/api`;
} else if (import.meta.env.DEV) {
  baseURL = '/api';
} else {
  baseURL = '/api';
}

console.log(`%c[Config] 🚀 Environment Init`, 'background: #333; color: #bada55');
console.log(`[Config] isTauri: ${isTauri}`);
console.log(`[Config] BaseURL: ${baseURL}`);

// ============================================================
// 2. 自定义 Tauri Adapter (Debug 增强版)
// ============================================================

const tauriAdapter = async (config) => {
  // 生成一个随机 Request ID，方便在海量日志中追踪同一个请求
  const reqId = Math.floor(Math.random() * 10000);
  const startTime = performance.now();

  // A. URL 处理
  const basePath = config.baseURL || '';
  const requestPath = config.url || '';
  const fullUrl = requestPath.startsWith('http') 
    ? requestPath 
    : `${basePath.replace(/\/$/, '')}/${requestPath.replace(/^\//, '')}`;

  console.log(`🔵 [Req #${reqId}] PREPARE: ${config.method?.toUpperCase()} ${fullUrl}`);

  try {
    // B. Headers 清洗
    const headers = new Headers();
    const axiosHeaders = config.headers;
    
    if (axiosHeaders) {
      const headersObj = typeof axiosHeaders.toJSON === 'function' 
        ? axiosHeaders.toJSON() 
        : axiosHeaders;

      for (const [key, val] of Object.entries(headersObj)) {
        if (val !== undefined && val !== null) {
          if (key.toLowerCase() === 'content-length') continue;
          if (key.toLowerCase() === 'host') continue; 
          headers.set(key, String(val));
        }
      }
    }

    // C. Body 处理
    let body = undefined;
    if (config.data) {
      if (typeof config.data === 'string') {
        body = config.data;
        if (!headers.has('Content-Type')) headers.set('Content-Type', 'text/plain');
      } else if (config.data instanceof FormData) {
        headers.delete('Content-Type'); 
        body = config.data;
      } else {
        body = JSON.stringify(config.data);
        headers.set('Content-Type', 'application/json');
      }
    }

    // [Debug] 打印即将发送的详细信息
    console.log(`🔍 [Req #${reqId}] DETAILS:`, {
      url: fullUrl,
      headers: Object.fromEntries(headers.entries()),
      bodyType: typeof body,
      bodyPreview: body ? String(body).substring(0, 100) : 'null'
    });

    // D. 发起请求 (执行 fetch)
    // -------------------------------------------------------
    console.log(`⏳ [Req #${reqId}] Sending fetch...`);
    const response = await fetch(fullUrl, {
      method: config.method?.toUpperCase(),
      headers: headers,
      body: body,
    });
    // -------------------------------------------------------

    const duration = (performance.now() - startTime).toFixed(2);
    console.log(`✅ [Req #${reqId}] FETCH SUCCESS (${duration}ms) Status: ${response.status}`);

    // ============================================================
    // E. 处理响应 Body (修复版)
    // ============================================================
    
    let responseData;
    const responseType = config.responseType || 'json'; // 默认为 json

    // 1. 如果请求的是二进制数据 (ArrayBuffer 或 Blob)
    if (responseType === 'arraybuffer') {
      console.log(`📦 [Req #${reqId}] Handling as ArrayBuffer`);
      responseData = await response.arrayBuffer();
    } 
    else if (responseType === 'blob') {
      console.log(`📦 [Req #${reqId}] Handling as Blob`);
      responseData = await response.blob();
    } 
    // 2. 默认作为文本/JSON 处理
    else {
      // 先读文本
      const rawText = await response.text(); 
      
      // [Debug] 打印文本内容（截断）
      console.log(`📦 [Req #${reqId}] RAW RESPONSE:`, rawText.substring(0, 300) + (rawText.length > 300 ? '...' : ''));

      try {
        // 尝试解析 JSON，如果配置是 'json' 或未指定
        if (responseType === 'json' || !responseType) {
            responseData = JSON.parse(rawText);
        } else {
            // 如果明确指定是 'text'
            responseData = rawText;
        }
      } catch (e) {
        // 如果虽然说是 JSON 但解析失败了（或者后端发回了错误文本），回退为文本
        console.warn(`⚠️ [Req #${reqId}] JSON Parse failed, returning text.`);
        responseData = rawText;
      }
    }

    return {
      data: responseData,
      status: response.status,
      statusText: response.statusText,
      headers: Object.fromEntries(response.headers.entries()),
      config: config,
      request: null,
    };

  } catch (error) {
    const duration = (performance.now() - startTime).toFixed(2);
    // [Debug] 捕获所有 fetch 阶段的致命错误
    console.error(`❌ [Req #${reqId}] FATAL ERROR (${duration}ms):`);
    console.error(`   Message: ${error.message}`);
    console.error(`   Stack: ${error.stack}`);
    // 将错误原样抛出给 Axios 处理
    throw error;
  }
};

// ============================================================
// 3. Axios 实例
// ============================================================

const apiClient = axios.create({
  baseURL: baseURL,
  timeout: 30000, // [Debug] 延长超时时间到 30s，排除超时干扰
  adapter: isTauri ? tauriAdapter : undefined,
  headers: {
    'Accept': 'application/json',
  }
});

// ============================================================
// 4. 拦截器
// ============================================================

apiClient.interceptors.request.use(
  (config) => {
    const token = sessionStorage.getItem('access_token');
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    const status = error.response?.status || 0;
    
    // [Debug] 打印 Axios 最终捕获的错误
    console.error(`🚨 [Axios Error] Status: ${status} | Code: ${error.code} | Message: ${error.message}`);
    
    if (status === 401 || status === 403) {
       // ... 登录跳转逻辑 ...
    }
    return Promise.reject(error);
  }
);

export default apiClient;