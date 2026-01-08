// src/services/url.js

/**
 * 将数据库存储的相对路径转换为完整的 URL
 * 因为前端本身就是从 5000 端口加载的，所有请求自动发送到同一服务器
 * @param {string} path - 数据库存的路径 (如 "uploads/products/abc.png")
 */
export function getImageUrl(path) {
  if (!path) return '';

  // 如果已经是完整链接，直接返回
  if (path.startsWith('http') || path.startsWith('blob:')) {
    return path;
  }

  // 返回相对路径，浏览器会自动发送到当前服务器
  const cleanPath = path.startsWith('/') ? path : `/${path}`;
  return cleanPath;
}