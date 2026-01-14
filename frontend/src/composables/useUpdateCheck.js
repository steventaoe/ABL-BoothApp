// src/composables/useUpdateCheck.js
import { ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { fetch } from '@tauri-apps/plugin-http';
import { open } from '@tauri-apps/plugin-shell';
import semver from 'semver';

// 配置你的 GitHub 仓库信息
// 请将下面的占位符替换为你实际的 GitHub 用户名和仓库名
const GITHUB_USER = 'Academy-of-Boundary-Landscape'; // 示例: 'torvalds'
const GITHUB_REPO = 'ABL-BoothApp'; // 示例: 'linux'

// API 地址
const LATEST_RELEASE_URL = `https://api.github.com/repos/${GITHUB_USER}/${GITHUB_REPO}/releases/latest`;
// 跳转下载页地址
const DOWNLOAD_PAGE_URL = `https://github.com/${GITHUB_USER}/${GITHUB_REPO}/releases/latest`;

export function useUpdateCheck() {
  const loading = ref(false);
  const error = ref(null);
  const hasUpdate = ref(false);
  const currentVersion = ref('');
  const latestVersion = ref('');
  const releaseNote = ref('');
  const releaseDate = ref('');

  const checkUpdate = async () => {
    loading.value = true;
    error.value = null;
    hasUpdate.value = false;

    try {
      // 1. 获取当前应用版本
      currentVersion.value = await getVersion();

      // 2. 请求 GitHub API
      const response = await fetch(LATEST_RELEASE_URL, {
        method: 'GET',
        // 建议加上 User-Agent，虽然不是必须，但符合 GitHub API 规范
        headers: {
          'User-Agent': 'Tauri-App-Updater',
        },
      });

      if (!response.ok) {
        throw new Error(`检查更新失败: ${response.statusText}`);
      }

      const data = await response.json();
      
      // 处理版本号：去掉可能存在的 'v' 前缀
      const remoteTag = data.tag_name.replace(/^v/, '');
      latestVersion.value = remoteTag;
      releaseNote.value = data.body || '暂无更新日志';
      releaseDate.value = data.published_at || '';

      // 3. 对比版本
      // 如果 远程版本 > 当前版本
      if (semver.gt(remoteTag, currentVersion.value)) {
        hasUpdate.value = true;
      } else {
        hasUpdate.value = false;
      }
    } catch (e) {
      console.error(e);
      error.value = e?.message || '网络连接失败';
    } finally {
      loading.value = false;
    }
  };

  const goToDownload = async () => {
    // 调用系统浏览器打开
    await open(DOWNLOAD_PAGE_URL);
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