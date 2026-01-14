<template>
  <n-config-provider :theme="naiveTheme" :theme-overrides="themeOverrides">
    <n-message-provider>
      <n-dialog-provider>
        <MainHeader />
        <RouterView />
        <GlobalAlert />
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup>
import { computed } from 'vue';
import { RouterView } from 'vue-router';
import GlobalAlert from '@/components/GlobalAlert.vue';
import MainHeader from '@/components/shared/MainHeader.vue';
import { NConfigProvider, NMessageProvider, NDialogProvider, darkTheme } from 'naive-ui';
import { useThemeStore } from '@/stores/themeStore';

const themeStore = useThemeStore();

// 让 Naive UI 随 store 的深色/浅色状态切换
const naiveTheme = computed(() => (themeStore.isDark ? darkTheme : null));

// 覆盖色板随用户自定义主色实时更新
// 注意：在 Pinia setup store 中，naiveThemeOverrides 已经是解包后的值，
// 这里不需要再取 .value，直接包一层 computed 保持响应式即可。
const themeOverrides = computed(() => themeStore.naiveThemeOverrides);
</script>
<style scoped>
  body {
  /* 基础样式 */
  margin: 0;
  padding: 0;
  
  /* 关键代码：利用 padding 避开安全区 */
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
  
  /* 确保背景色铺满全屏，而不仅仅是 padding 内部 */
  min-height: 100vh;
  box-sizing: border-box;
}
</style>