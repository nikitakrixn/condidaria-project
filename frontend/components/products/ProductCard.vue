<template>
    <div class="bg-white rounded-lg shadow-md overflow-hidden">
      <img v-if="product.image_url" :src="product.image_url" alt="Product Image" class="w-full h-48 object-cover" />
      <div class="p-4">
        <h2 class="text-xl font-bold mb-2">{{ product.name }}</h2>
        <p class="text-gray-700 mb-4">{{ product.description }}</p>
        <p class="text-lg font-bold text-gray-800">{{ product.formatted_price }}</p>
        <NuxtLink :to="`/products/${product.id}`" class="mt-4 bg-blue-500 text-white px-4 py-2 rounded">Подробнее</NuxtLink>
      </div>
    </div>
  </template>
  
  <script setup>
  defineProps({
    product: {
      type: Object,
      required: true,
    },
  });
  </script>