// src/utils/url.js

// 检测是否在 Tauri 环境中
const isTauri = window.__TAURI_INTERNALS__ !== undefined;

// 后端端口 (必须与 server.rs 保持一致)
const API_PORT = 5000;

/**
 * 将相对路径转换为完整的图片 URL
 * @param {string} path - 数据库存的路径 (如 "products/abc.png")
 */
export function getImageUrl(path) {
  if (!path) return '';

  // 如果已经是完整链接 (比如网图)，直接返回
  if (path.startsWith('http') || path.startsWith('blob:')) {
    return path;
  }

  // 统一补全前缀
  // 最终变成: /static/uploads/products/abc.png
  // 注意：不要重复添加
  let fullPath = path;
  if (!path.startsWith('/static/uploads/')) {
    // 确保 path 前面没有多余的 /
    const cleanPath = path.startsWith('/') ? path.slice(1) : path;
    fullPath = `/static/uploads/${cleanPath}`;
  }

  // 区分环境返回
  if (isTauri) {
    // Tauri 环境：必须是绝对地址 http://localhost:5000/static/uploads/...
    return `http://localhost:${API_PORT}${fullPath}`;
  } else {
    // 浏览器开发环境：返回相对地址，由 Vite 代理处理
    // 请求 /static/uploads/... -> Vite Proxy -> http://127.0.0.1:5000/static/uploads/...
    return fullPath;
  }
}