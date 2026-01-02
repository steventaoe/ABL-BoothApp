import { io } from 'socket.io-client';

const URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:5000';

export const socket = io(URL, {
  path: '/socket.io',
  autoConnect: false,
  namespace: '/sale',
  // 【核心改动】强制只使用 'polling'，禁用 'websocket'
  transports: ['websocket', 'polling'], 
});