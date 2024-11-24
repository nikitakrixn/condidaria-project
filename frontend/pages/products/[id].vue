<template>
    <div class="container mx-auto p-4">
      <h1 class="text-2xl font-bold mb-4">{{ product.name }}</h1>
      <img v-if="product.image_url" :src="product.image_url" alt="Product Image" class="w-full h-96 object-cover mb-4" />
      <p class="text-gray-700 mb-4">{{ product.description }}</p>
      <p class="text-lg font-bold text-gray-800">{{ product.formatted_price }}</p>
      <NuxtLink to="/products" class="mt-4 bg-gray-500 text-white px-4 py-2 rounded">Назад к списку продуктов</NuxtLink>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted } from 'vue';
  import { useRoute } from '#app';
  import { useProducts } from '~/composables/useProducts';
  
  const route = useRoute();
  const { getProductById } = useProducts();
  const product = ref(null);
  
  const fetchProduct = async () => {
    const { data } = await getProductById(route.params.id);
    if (data.value) {
      product.value = data.value;
    }
  };
  
  onMounted(fetchProduct);
  </script>