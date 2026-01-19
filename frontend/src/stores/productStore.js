import { defineStore } from 'pinia';
import api from '@/services/api';
import { ref, computed } from 'vue';
import { getImageUrl } from '@/services/url';

export const useProductStore = defineStore('masterProduct', () => {
  // --- State ---
  const masterProducts = ref([]);
  const isLoading = ref(false);
  const error = ref(null);
  const searchTerm = ref('');
  const showInactive = ref(false);
  
  // --- Getters (Computed) ---
  const filteredProducts = computed(() => {
    if (!searchTerm.value.trim()) {
      return masterProducts.value;
    }
    const lowerCaseSearchTerm = searchTerm.value.toLowerCase();
    return masterProducts.value.filter(product => {
      const nameMatch = product.name.toLowerCase().includes(lowerCaseSearchTerm);
      const codeMatch = product.product_code.toLowerCase().includes(lowerCaseSearchTerm);
      return nameMatch || codeMatch;
    });
  });

  // --- Actions ---
  
  // fetchMasterProducts 无需修改
  async function fetchMasterProducts() {
    isLoading.value = true;
    error.value = null;
    try {
      const params = showInactive.value ? { all: true } : {};
      const response = await api.get('/master-products', { params });
      masterProducts.value = response.data.map(product => ({
        ...product,
        image_url: getImageUrl(product.image_url)
      }));
    } catch (err) {
      error.value = '无法加载商品库列表。';
      console.error(err);
    } finally {
      isLoading.value = false;
    }
  }

  // 【已修改】createMasterProduct 现在直接接收一个 FormData 对象
  async function createMasterProduct(formData) {
    try {
      // 组件已经准备好了 FormData，我们直接发送即可
      // Axios 会自动设置正确的 Content-Type
      const response = await api.post('/master-products', formData);
      const product = { ...response.data, image_url: getImageUrl(response.data.image_url) };
      masterProducts.value.unshift(product);
      return product;
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '创建商品失败，请检查输入。');
    }
  }

  // 【已修改】updateMasterProduct 现在接收 productId 和 FormData 两个参数
  async function updateMasterProduct(productId, formData) {
    try {
      // 使用 POST 发送 FormData 来更新，以获得更好的兼容性
      const response = await api.post(`/master-products/${productId}`, formData);
      const product = { ...response.data, image_url: getImageUrl(response.data.image_url) };
      const index = masterProducts.value.findIndex(p => p.id === productId);
      if (index !== -1) {
        // 使用新数据替换旧数据，以确保响应性
        masterProducts.value[index] = product;
      }
      return product;
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '更新商品失败，请重试。');
    }
  }
  
  // toggleProductStatus 无需修改
  async function toggleProductStatus(product) {
    try {
      const newStatus = !product.is_active;
      const response = await api.put(`/master-products/${product.id}/status`, { is_active: newStatus });
      const updatedProduct = { ...response.data, image_url: getImageUrl(response.data.image_url) };
      const index = masterProducts.value.findIndex(p => p.id === product.id);
      if (index !== -1) {
        masterProducts.value[index] = updatedProduct;
      }
    } catch (err) {
      console.error(err);
      throw new Error(err.response?.data?.error || '更新商品状态失败。');
    }
  }

  // --- Return ---
  return {
    masterProducts,
    isLoading,
    error,
    searchTerm,
    filteredProducts,
    fetchMasterProducts,
    createMasterProduct,
    updateMasterProduct,
    toggleProductStatus,
    showInactive,
  };
});