import { defineStore } from 'pinia';
import api from '@/services/api';
import { ref } from 'vue';

export const useSyncStore = defineStore('sync', () => {
  const isExporting = ref(false);
  const isImporting = ref(false);
  const lastError = ref(null);
  const isTauri = typeof window !== 'undefined' && window.__TAURI_INTERNALS__ !== undefined;
  const isMobileUA = typeof navigator !== 'undefined' && /android|iphone|ipad|ipod/i.test(navigator.userAgent || '');
  const isTauriDesktop = isTauri && !isMobileUA;
  const isTauriMobile = isTauri && isMobileUA;

  async function exportProducts() {
    isExporting.value = true;
    lastError.value = null;
    try {
      const response = await api.get('/sync/export-products', { responseType: isTauriDesktop ? 'arraybuffer' : 'blob' });
      const disposition = response.headers['content-disposition'] || '';
      const match = disposition.match(/filename="?([^";]+)"?/i);
      const filename = match ? match[1] : 'booth_catalog.boothpack';

      if (isTauriDesktop) {
        // 桌面端：调用 Tauri 保存对话框让用户选择路径
        const [dialogModule, fsModule] = await Promise.all([
          import('@tauri-apps/plugin-dialog'),
          import('@tauri-apps/plugin-fs'),
        ]);

        const filePath = await dialogModule.save({
          defaultPath: filename,
          filters: [{ name: 'Booth Pack', extensions: ['boothpack', 'zip'] }],
        });

        if (!filePath) {
          throw new Error('已取消保存');
        }

        const buffer = response.data instanceof ArrayBuffer
          ? new Uint8Array(response.data)
          : new Uint8Array(await response.data.arrayBuffer());

        await fsModule.writeFile(filePath, buffer);
        return { filename: filePath };
      }

      // Tauri 移动端或普通浏览器：优先尝试移动端分享，其次回退为下载
      const blob = response.data instanceof Blob
        ? response.data
        : new Blob([response.data], { type: 'application/zip' });

      const canUseShare = typeof navigator !== 'undefined'
        && typeof navigator.share === 'function'
        && typeof navigator.canShare === 'function';

      if (canUseShare) {
        try {
          const file = new File([blob], filename, { type: 'application/zip' });
          if (navigator.canShare({ files: [file] })) {
            await navigator.share({
              files: [file],
              title: 'Booth Tool 制品包',
              text: '导出的制品包，可在设备上保存或分享',
            });
            return { filename };
          }
        } catch (shareErr) {
          console.warn('Web Share API failed, falling back to download', shareErr);
        }
      }

      // 回退：自动下载到浏览器默认目录
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = filename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);
      return { filename };
    } catch (err) {
      console.error(err);
      lastError.value = err;
      throw new Error('导出失败，请稍后重试');
    } finally {
      isExporting.value = false;
    }
  }

  async function importProducts(file) {
    if (!file) throw new Error('请选择要导入的文件');
    isImporting.value = true;
    lastError.value = null;
    try {
      const formData = new FormData();
      formData.append('file', file);
      const response = await api.post('/sync/import-products', formData);
      return response.data;
    } catch (err) {
      console.error(err);
      lastError.value = err;
      const msg = err.response?.data?.error || err.response?.data?.message || '导入失败，请检查文件格式或稍后重试';
      throw new Error(msg);
    } finally {
      isImporting.value = false;
    }
  }

  // Tauri 桌面/移动：从本地路径读取后复用导入接口
  // skipConfirm: 如果为 true，跳过确认（因为调用方已经确认过了）
  async function importProductsFromPath(filePath, skipConfirm = false) {
    if (!filePath) throw new Error('未提供文件路径');
    if (!isTauri) throw new Error('仅在 Tauri 环境可用');
    
    isImporting.value = true;
    lastError.value = null;
    try {
      const fsModule = await import('@tauri-apps/plugin-fs');
      
      // [修复点 1] v2 中使用 readFile，直接返回 Uint8Array
      const data = await fsModule.readFile(filePath); 
      
      // data 已经是 Uint8Array，直接转 Blob 即可
      const blob = new Blob([data], { type: 'application/zip' });
      const fileName = filePath.split(/[/\\]/).pop() || 'import.boothpack';
      const file = new File([blob], fileName, { type: 'application/zip' });
      return await importProducts(file);
    } catch (err) {
      console.error('[Tauri] importProductsFromPath error:', err);
      lastError.value = err;
      const msg = err.message || '导入失败';
      throw new Error(msg);
    } finally {
      isImporting.value = false;
    }
  }

  return {
    isExporting,
    isImporting,
    lastError,
    exportProducts,
    importProducts,
    importProductsFromPath,
  };
});
