<template>
    <div>
      <h1 class="text-3xl font-bold mb-4">Products</h1>
      <div v-if="error" class="text-red-500">{{ error.message }}</div>
      <div v-if="pending" class="text-center">Loading...</div>
      <div v-if="data">
        <div v-if="data.length === 0" class="text-center">No products found.</div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div v-for="product in data" :key="product.id" class="p-4 border rounded">
            <h2 class="text-xl font-bold">{{ product.name }}</h2>
            <p class="text-gray-700 mb-2">{{ product.description }}</p>
            <p class="text-lg font-semibold">{{ product.price }} руб.</p>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup>
  import { useApi } from '~/composables/useApi';
  
  const { data, error } = await useApi('/products');
  </script>