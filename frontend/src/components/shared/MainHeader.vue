<template>
  <header class="app-header">
    <div class="app-header-left">
      <span class="app-logo">出摊工具导航栏</span>
    </div>
    <nav class="app-header-nav">
      <RouterLink
        to="/"
        class="nav-link"
        active-class="nav-link--active"
        exact-active-class="nav-link--active"
      >
        用户页面
      </RouterLink>
      <RouterLink
        to="/vendor"
        class="nav-link"
        active-class="nav-link--active"
      >
        摊主页面
      </RouterLink>
      <RouterLink
        to="/admin"
        class="nav-link"
        active-class="nav-link--active"
      >
        管理员页面
      </RouterLink>
    </nav>
  </header>
</template>

<script setup>
import { RouterLink } from 'vue-router';
</script>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  border-bottom: 1px solid #2c2c2c;
  background-color: #121212;
}

.app-logo {
  font-size: 18px;
  font-weight: 600;
  color: #ffffff;
}

.app-header-nav {
  display: flex;
  gap: 12px;
}

.nav-link {
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 14px;
  color: #c8c8c8;
  text-decoration: none;
  transition: background-color 0.15s ease, color 0.15s ease;
}

.nav-link:hover {
  background-color: #1f1f1f;
  color: #ffffff;
}

.nav-link--active {
  background-color: #03dac6;
  color: #000000;
}
</style>
