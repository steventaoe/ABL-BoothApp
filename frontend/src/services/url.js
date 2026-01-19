// src/services/url.js

const isTauri = typeof window !== 'undefined' && window.__TAURI_INTERNALS__ !== undefined;

// 你的后端端口
const API_PORT = 5140;

// 注意：图片不在 /api 下，而在根路径的 /uploads 下
const SERVER_ORIGIN = `http://127.0.0.1:${API_PORT}`;

/**
 * 将数据库存储的相对路径转换为完整的 URL
 * @param {string} path - 数据库存的路径 (如 "uploads/products/abc.png" 或 "/uploads/products/abc.png")
 */
export function getImageUrl(path) {
  if (!path) return '';

  // 已经是完整链接 or blob
  if (path.startsWith('http') || path.startsWith('blob:') || path.startsWith('data:')) {
    return path;
  }

  // 规范成以 / 开头
  const cleanPath = path.startsWith('/') ? path : `/${path}`;

  // 关键：Tauri 里前端不是 http origin，必须拼绝对地址指向你的 axum server
  if (isTauri) {
    console.log(`[getImageUrl] Tauri detected, converting to absolute URL: ${SERVER_ORIGIN}${cleanPath}`);
    return `${SERVER_ORIGIN}${cleanPath}`;
  }

  // 浏览器环境仍保持相对路径（同源）
  return cleanPath;
}
