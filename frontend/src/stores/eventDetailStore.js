import { defineStore } from 'pinia';
import { ref } from 'vue';
import api from '@/services/api';

export const useEventDetailStore = defineStore('eventDetail', () => {
  // --- State ---
  const event = ref(null); // 存储当前展会的详细信息 (暂时未使用，但可扩展)
  const products = ref([]); // 存储该展会上架的商品列表
  const isLoading = ref(false);
  const error = ref(null);
  const allOrders = ref([]); 

  // --- Actions ---
  // 获取指定展会的商品列表
  async function fetchProductsForEvent(eventId) {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await api.get(`/events/${eventId}/products`);
      products.value = response.data;
    } catch (err) {
      error.value = '无法加载展会商品列表。';
      console.error(err);
    } finally {
      isLoading.value = false;
    }
  }

  // 为展会添加一个商品
  async function addProductToEvent(eventId, productData) {
    try {
      const response = await api.post(`/events/${eventId}/products`, productData);
      products.value.unshift(response.data);
      return response.data;
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '上架商品失败。');
    }
  }
  
  // 更新展会商品的库存或价格
  async function updateEventProduct(productId, productData) {
    try {
      const response = await api.put(`/products/${productId}`, productData);
      const index = products.value.findIndex(p => p.id === productId);
      if (index !== -1) {
        products.value[index] = response.data;
      }
      return response.data;
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '更新商品失败。');
    }
  }

  // 从展会中移除一个商品
  async function deleteEventProduct(productId) {
    try {
      await api.delete(`/products/${productId}`);
      products.value = products.value.filter(p => p.id !== productId);
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '下架商品失败。');
    }
  }
  async function fetchAllOrdersForEvent(eventId) {
    isLoading.value = true;
    error.value = null;
    try {
      // 不带 status 参数，获取所有订单
      const response = await api.get(`/events/${eventId}/orders`);
      allOrders.value = response.data;
    } catch (err) {
      error.value = '无法加载订单列表。';
      console.error(err);
    } finally {
      isLoading.value = false;
    }
  }
  async function adminUpdateOrderStatus(eventId, orderId, newStatus) {
    try {
      const response = await api.put(`/events/${eventId}/orders/${orderId}/status`, { status: newStatus });
      // 更新成功后，在本地 allOrders 列表中找到并更新该订单
      const index = allOrders.value.findIndex(o => o.id === orderId);
      if (index !== -1) {
        allOrders.value[index] = response.data;
      }
      return response.data;
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '更新订单状态失败。');
    }
  }
  // 重置状态，以便在切换不同展会详情页时清空旧数据
  function resetStore() {
      event.value = null;
      products.value = [];
      error.value = null;
      allOrders.value = [];
  }

  return {
    event,
    products,
    isLoading,
    error,
    fetchProductsForEvent,
    addProductToEvent,
    updateEventProduct,
    deleteEventProduct,
    resetStore,
    adminUpdateOrderStatus,
    fetchAllOrdersForEvent,
    allOrders
  };
});