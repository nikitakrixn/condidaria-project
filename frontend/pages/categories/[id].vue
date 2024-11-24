<template>
  <div class="p-6 bg-pink-50 min-h-screen">
    <div v-if="error" class="text-red-500 text-center">{{ error.message }}</div>
    <div v-if="pending" class="text-center">Loading...</div>

    <div v-if="data" class="max-w-4xl mx-auto bg-white rounded-lg shadow-lg p-6">
      <div class="mb-8">
        <h1 class="text-4xl font-bold text-pink-600">{{ data.name }}</h1>
        <p class="text-gray-700 mt-2">{{ data.description || 'This category has no description.' }}</p>
      </div>

      <div v-if="data.imageUrl" class="mb-8">
        <img :src="data.imageUrl" alt="Category Image" class="w-full h-64 object-cover rounded-lg shadow-md" />
      </div>

      <div v-if="products && products.length > 0" class="mt-8">
        <h2 class="text-2xl font-semibold mb-4">Products in this Category</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
          <div
            v-for="product in products"
            :key="product.id"
            class="bg-white rounded-lg shadow-lg overflow-hidden transform hover:scale-105 transition-transform duration-300"
          >
            <div class="h-48 bg-cover bg-center" :style="{ backgroundImage: `url(${product.imageUrl || defaultProductImage})` }"></div>
            <div class="p-4">
              <h3 class="text-xl font-bold text-pink-600">{{ product.name }}</h3>
              <p class="text-gray-700 mb-4">{{ product.description || 'No description available.' }}</p>
              <p class="text-lg font-semibold text-gray-800">{{ product.price }} руб.</p>
              <NuxtLink
                :to="`/products/${product.id}`"
                class="block text-center mt-2 px-4 py-2 bg-pink-600 text-white rounded hover:bg-pink-700 transition-colors"
              >
                View Product
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="text-center mt-8 text-gray-600">
        No products found in this category.
      </div>

      <div class="mt-8">
        <NuxtLink
          to="/categories"
          class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Back to Categories
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useRoute } from 'vue-router';
import { useApi } from '~/composables/useApi';

const route = useRoute();
const { data, pending, error } = await useApi(`/categories/${route.params.id}`);

const { data: products } = await useApi(`/categories/${route.params.id}/products`);

const defaultProductImage = 'https://via.placeholder.com/300x200?text=Product+Image';
</script>