<template>
  <Transition name="alert-fade">
    <div v-if="alertStore.isVisible" class="alert-overlay" @click.self="alertStore.hide">
      <div class="alert-box" :class="`alert-${alertStore.type}`">
        <header class="alert-header">
          <h3>{{ alertStore.title }}</h3>
          <button class="close-btn" @click="alertStore.hide">&times;</button>
        </header>
        <main class="alert-body">
          <p>{{ alertStore.message }}</p>
        </main>
        <footer class="alert-footer">
          <button class="ok-btn" @click="alertStore.hide">确认</button>
        </footer>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { useAlertStore } from '@/stores/alertStore';

const alertStore = useAlertStore();
</script>

<style scoped>
.alert-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999999;
}

.alert-box {
  background-color: #2c2c3e; /* 深色背景 */
  border-radius: 8px;
  width: 90%;
  max-width: 400px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
  border-top: 4px solid #4a90e2; /* 默认信息蓝 */
  color: #e0e0e0;
  transform: scale(1);
}

/* 根据类型改变边框颜色 */
.alert-box.alert-success { border-top-color: #50e3c2; }
.alert-box.alert-warning { border-top-color: #f5a623; }
.alert-box.alert-error { border-top-color: #d0021b; }

.alert-header {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid #444;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.alert-header h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 500;
}
.close-btn {
  background: none;
  border: none;
  font-size: 2rem;
  color: #888;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.alert-body {
  padding: 1.5rem;
  font-size: 1rem;
  line-height: 1.6;
}

.alert-footer {
  padding: 1rem 1.5rem;
  background-color: rgba(0, 0, 0, 0.2);
  display: flex;
  justify-content: flex-end;
  border-radius: 0 0 8px 8px;
}

.ok-btn {
  background-color: #4a90e2; /* 默认信息蓝 */
  color: white;
  border: none;
  padding: 0.75rem 2rem;
  border-radius: 5px;
  font-weight: bold;
  cursor: pointer;
  transition: background-color 0.2s;
}
.ok-btn:hover {
  background-color: #357abd;
}

/* 根据类型改变按钮颜色 */
.alert-success .ok-btn { background-color: #50e3c2; }
.alert-success .ok-btn:hover { background-color: #38a88f; }
.alert-warning .ok-btn { background-color: #f5a623; }
.alert-warning .ok-btn:hover { background-color: #d48e1a; }
.alert-error .ok-btn { background-color: #d0021b; }
.alert-error .ok-btn:hover { background-color: #a00114; }

/* 过渡动画 */
.alert-fade-enter-active,
.alert-fade-leave-active {
  transition: opacity 0.3s ease;
}
.alert-fade-enter-active .alert-box,
.alert-fade-leave-active .alert-box {
  transition: transform 0.3s ease;
}

.alert-fade-enter-from,
.alert-fade-leave-to {
  opacity: 0;
}
.alert-fade-enter-from .alert-box,
.alert-fade-leave-to .alert-box {
  transform: scale(0.9);
}
</style>