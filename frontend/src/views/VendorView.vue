<template>
  <div class="vendor-view">
    <header class="page-header">
      <div class="header-content">
        <h1>待处理订单</h1>
        <p v-if="eventName">当前展会: <strong>{{ eventName }}</strong></p>
        <p v-else>正在加载展会信息...</p>
      </div>
      <!-- 使用 Naive UI 按钮 -->
      <n-button 
        type="primary" 
        :loading="isRefreshing" 
        @click="manualRefresh"
      >
        {{ isRefreshing ? '刷新中' : '手动刷新' }}
      </n-button>
    </header>

    <main class="order-feed-container">
      <LiveStats class="live-stats-module" :event-id="props.id" />
      
      <n-alert v-if="store.pendingOrders.length" type="warning" :bordered="false" style="margin-bottom: 1rem;">
        有 {{ store.pendingOrders.length }} 条待处理订单，请及时处理。
      </n-alert>

      <div class="order-tabs">
        <n-tabs v-model:value="currentTab" type="line" animated>
          <n-tab-pane :name="'pending'" :tab="'待处理 (' + store.pendingOrders.length + ')'" />
          <n-tab-pane name="completed" tab="已完成" />
        </n-tabs>
      </div>

      <!-- 列表内容 -->
      <div v-show="currentTab === 'pending'" class="order-feed">
        <div v-if="!store.pendingOrders.length" class="no-orders-message">暂无待处理订单</div>
        <TransitionGroup name="list" tag="div">
          <OrderCard 
            v-for="order in store.pendingOrders" 
            :key="order.id"
            :order="order"
            @complete="completeOrder"
            @cancel="cancelOrder"
          />
        </TransitionGroup>
      </div>

      <div v-show="currentTab === 'completed'" class="order-feed">
         <p class="revenue-summary">今日已完成订单总额: <strong>¥{{ store.totalRevenue.toFixed(2) }}</strong></p>
         <div v-if="!store.completedOrders.length" class="no-orders-message">暂无已完成订单</div>
         <OrderCard 
            v-for="order in store.completedOrders" 
            :key="order.id"
            :order="order"
            :is-completed="true"
         />
      </div>
    </main>

    <!-- 隐藏的音频播放器，引用 public 目录下的 notify.mp3 -->
    <audio ref="audioRef" src="/notify.mp3" preload="auto"></audio>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { NButton, NTabs, NTabPane, NAlert, useDialog, useMessage } from 'naive-ui';
import { useOrderStore } from '@/stores/orderStore';
import { useEventStore } from '@/stores/eventStore';
import { useEventDetailStore } from '@/stores/eventDetailStore'; 
import LiveStats from '@/components/vendor/LiveStats.vue';
import OrderCard from '@/components/order/OrderCard.vue';

const props = defineProps({
  id: { type: String, required: true }
});

const audioRef = ref(null);
const store = useOrderStore();
const eventStore = useEventStore();
const eventDetailStore = useEventDetailStore();
const message = useMessage();
const dialog = useDialog();

const isRefreshing = ref(false);
const currentTab = ref('pending');
const isInitialized = ref(false); // 用于标记第一次加载，避免页面一打开就响

const eventName = computed(() => {
  const event = eventStore.events.find(e => e.id === parseInt(props.id, 10));
  return event ? event.name : `展会 #${props.id}`;
});

// 播放声音的函数
const playNoticeSound = () => {
  if (audioRef.value) {
    audioRef.value.currentTime = 0;
    // 现代浏览器要求用户必须点击过页面后才能自动播放声音
    audioRef.value.play().catch(err => {
      console.warn('音频播放尝试失败（用户尚未与页面交互或文件路径不正确）:', err);
    });
  }
};

async function manualRefresh() {
  isRefreshing.value = true;
  // 顺便在这里尝试播放一下声音，让浏览器“解锁”音频播放权限
  playNoticeSound(); 
  
  await Promise.all([
    store.pollPendingOrders(),
    eventDetailStore.fetchProductsForEvent(props.id),
    store.fetchCompletedOrders(),
  ]);
  isRefreshing.value = false;
}

// 核心逻辑：监听订单数量变化
watch(
  () => store.pendingOrders.length,
  (newCount, oldCount) => {
    // 只有当数量增加，且不是第一次初始化加载时才响铃
    if (isInitialized.value && newCount > oldCount) {
      playNoticeSound();
      message.info('收到新订单！', { keepAliveOnHover: true });
    }
    
    // 首次加载后标记为已初始化
    if (!isInitialized.value && newCount !== undefined) {
      isInitialized.value = true;
    }
  }
);

async function completeOrder(orderId) {
  try {
    await store.markOrderAsCompleted(orderId);
    await eventDetailStore.fetchProductsForEvent(props.id);
    await store.fetchCompletedOrders();
    message.success('订单已完成配货');
  } catch (error) {
    message.error(error?.message || '操作失败');
  }
}

async function cancelOrder(orderId) {
  dialog.warning({
    title: '确认取消',
    content: '确定要取消这个订单吗？此操作无法撤销。',
    positiveText: '确认',
    negativeText: '返回',
    async onPositiveClick() {
      try {
        await store.cancelOrder(orderId);
        message.success('订单已取消');
      } catch (error) {
        message.error(error?.message || '取消失败');
      }
    }
  });
}

onMounted(() => {
  if (eventStore.events.length === 0) {
    eventStore.fetchEvents();
  }
  store.setActiveEvent(props.id);
  eventDetailStore.fetchProductsForEvent(props.id);
});

onUnmounted(() => {
  store.stopPolling();
});
</script>

<style scoped>
.vendor-view { max-width: 600px; margin: 0 auto; padding: 1rem; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 2rem; border-bottom: 1px solid var(--border-color); padding-bottom: 1rem; }
.page-header h1 { margin: 0; color: var(--accent-color); }
.order-tabs { margin-bottom: 1.5rem; }
.no-orders-message { text-align: center; padding: 3rem; color: var(--text-muted); }
.revenue-summary { text-align: right; font-size: 1.1rem; margin-bottom: 1rem; }
.list-enter-active, .list-leave-active { transition: all 0.5s ease; }
.list-enter-from, .list-leave-to { opacity: 0; transform: translateX(30px); }
</style>