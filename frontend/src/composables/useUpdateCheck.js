import { ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { fetch } from '@tauri-apps/plugin-http';
import { open } from '@tauri-apps/plugin-shell';

// === 配置区域 ===
const GITHUB_USER = 'Academy-of-Boundary-Landscape';
const GITHUB_REPO = 'ABL-BoothApp';
const LATEST_RELEASE_URL = `https://api.github.com/repos/${GITHUB_USER}/${GITHUB_REPO}/releases/latest`;
const DOWNLOAD_PAGE_URL = `https://github.com/${GITHUB_USER}/${GITHUB_REPO}/releases/latest`;

/**
 * 简单的语义化版本比较 (无需安装 semver 库)
 * 返回 1: v1 > v2
 * 返回 -1: v1 < v2
 * 返回 0: v1 == v2
 */
function compareVersions(v1, v2) {
  const parts1 = v1.split('.').map(Number);
  const parts2 = v2.split('.').map(Number);
  const len = Math.max(parts1.length, parts2.length);

  for (let i = 0; i < len; i++) {
    const p1 = parts1[i] || 0;
    const p2 = parts2[i] || 0;
    if (p1 > p2) return 1;
    if (p1 < p2) return -1;
  }
  return 0;
}

export function useUpdateCheck() {
  const loading = ref(false);
  const error = ref(null);
  const hasUpdate = ref(false);
  const currentVersion = ref('');
  const latestVersion = ref('');
  const releaseNote = ref('');
  const releaseDate = ref('');

  // 辅助检测是否在 Tauri 环境
  const isTauri = () => typeof window !== 'undefined' && window.__TAURI_INTERNALS__ !== undefined;

  const checkUpdate = async () => {
    // 如果在浏览器中运行，直接停止，防止调用 Tauri API 报错
    if (!isTauri()) {
      error.value = "请在 App 环境中运行";
      return;
    }

    loading.value = true;
    error.value = null;
    hasUpdate.value = false;

    try {
      // 1. 获取当前应用版本
      currentVersion.value = await getVersion();

      // 2. 请求 GitHub API
      // 注意：Tauri 的 fetch 不受 CORS 限制
      const response = await fetch(LATEST_RELEASE_URL, {
        method: 'GET',
        headers: {
          'User-Agent': 'Tauri-App-Updater', // GitHub API 要求 User-Agent
        },
      });

      if (!response.ok) {
        // 处理 GitHub API 限流 (403) 或 404
        if (response.status === 403) {
          throw new Error("检查过于频繁，请稍后再试");
        }
        throw new Error(`请求失败: ${response.status} ${response.statusText}`);
      }

      const data = await response.json();
      
      // 处理版本号：去掉可能存在的 'v' 前缀
      const remoteTag = data.tag_name.replace(/^v/, '');
      
      latestVersion.value = remoteTag;
      releaseNote.value = data.body || '暂无更新日志';
      releaseDate.value = data.published_at || '';

      // 3. 对比版本
      // 如果 远程版本 > 当前版本 (结果为 1)
      if (compareVersions(remoteTag, currentVersion.value) === 1) {
        hasUpdate.value = true;
      } else {
        hasUpdate.value = false;
      }
    } catch (e) {
      console.error("更新检查出错:", e);
      error.value = e instanceof Error ? e.message : '网络连接失败';
    } finally {
      loading.value = false;
    }
  };

  const goToDownload = async () => {
    if (isTauri()) {
      // 调用系统默认浏览器打开 (支持 Windows 和 Android)
      await open(DOWNLOAD_PAGE_URL);
    } else {
      // 浏览器环境回退
      window.open(DOWNLOAD_PAGE_URL, '_blank');
    }
  };

  return {
    loading,
    error,
    hasUpdate,
    currentVersion,
    latestVersion,
    releaseNote,
    releaseDate,
    checkUpdate,
    goToDownload,
  };
}