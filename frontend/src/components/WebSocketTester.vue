<template>
  <div class="tester-container">
    <h3>WebSocket 连接测试器</h3>
    <div class="controls">
      <button @click="connectSocket" :disabled="isConnected">连接</button>
      <button @click="disconnectSocket" :disabled="!isConnected">断开</button>
      <button @click="sendPing" :disabled="!isConnected">发送 Ping</button>
    </div>
    <div class="status">
      <strong>状态:</strong>
      <span :class="statusClass">{{ statusMessage }}</span>
    </div>
    <div class="logs">
      <h4>日志:</h4>
      <ul>
        <li v-for="(log, index) in logs" :key="index">{{ log }}</li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onUnmounted } from 'vue';
import { socket } from '@/services/socketService';

const isConnected = ref(socket.connected);
const logs = ref([]);

const statusMessage = computed(() => isConnected.value ? '已连接' : '已断开');
const statusClass = computed(() => isConnected.value ? 'connected' : 'disconnected');

function addLog(message) {
  const time = new Date().toLocaleTimeString();
  logs.value.unshift(`[${time}] ${message}`); // unshift 会将新日志放在最上面
}

function setupListeners() {
  socket.off(); // 清理所有旧监听器

  socket.on('connect', () => {
    isConnected.value = true;
    addLog('✅ 连接成功！');
  });

  socket.on('disconnect', () => {
    isConnected.value = false;
    addLog('🔌 连接已断开。');
  });

  socket.on('connection_successful', (data) => {
    addLog(`🤝 收到后端欢迎消息: ${data.message}`);
  });

  socket.on('pong_test', (data) => {
    addLog(`🎉 收到 PONG 响应: ${data.message}`);
  });
}

// 在组件加载时就设置好监听器
setupListeners();

function connectSocket() {
  addLog('正在尝试连接...');
  socket.connect();
}

function disconnectSocket() {
  addLog('正在断开连接...');
  socket.disconnect();
}

function sendPing() {
  addLog("🚀 发送 'ping_test' 事件...");
  socket.emit('ping_test', { from: 'WebSocketTester' });
}

// 确保组件卸载时断开连接
onUnmounted(() => {
  if (socket.connected) {
    socket.disconnect();
  }
});
</script>

<style scoped>
.tester-container {
  position: fixed;
  bottom: 20px;
  right: 20px;
  background-color: var(--card-bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  width: 350px;
  z-index: 9999;
  font-size: 0.9rem;
}
.controls { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
.controls button { padding: 5px 10px; /* ... */ }
.status { margin-bottom: 1rem; }
.status span { font-weight: bold; padding: 2px 6px; border-radius: 4px; }
.status .connected { color: #4CAF50; background-color: rgba(76, 175, 80, 0.1); }
.status .disconnected { color: #F44336; background-color: rgba(244, 67, 54, 0.1); }
.logs { max-height: 200px; overflow-y: auto; background-color: var(--bg-color); padding: 0.5rem; border-radius: 4px; }
.logs ul { list-style-type: none; padding: 0; margin: 0; }
</style>