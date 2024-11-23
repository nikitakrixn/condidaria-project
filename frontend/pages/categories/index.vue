<template>
    <div class="p-6 bg-pink-50 min-h-screen">
      <h1 class="text-4xl font-extrabold text-center mb-8 text-pink-600">Our Sweet Categories</h1>
  
      <div v-if="error" class="text-red-500 text-center">{{ error.message }}</div>
  
      <div v-if="data && data.length === 0" class="text-center">
        No categories found.
      </div>
  
      <div v-if="data" class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
        <div
          v-for="category in data"
          :key="category.id"
          class="bg-white rounded-lg shadow-lg overflow-hidden transform hover:scale-105 transition-transform duration-300"
        >
          <div class="h-48 bg-cover bg-center" :style="{ backgroundImage: `url(${category.imageUrl || defaultImage})` }"></div>
          <div class="p-4">
            <h2 class="text-2xl font-bold text-pink-600 mb-2">{{ category.name }}</h2>
            <p class="text-gray-700 mb-4">{{ category.description || 'No description available.' }} </p>
            <NuxtLink
              :to="`/categories/${category.id}`"
              class="block text-center px-4 py-2 bg-pink-600 text-white rounded hover:bg-pink-700 transition-colors"
            >
              View Details
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
</template>
<script setup>
  import { useApi } from '~/composables/useApi';

  const { data, error } = await useApi('/categories');

  const defaultImage = 'https://via.placeholder.com/300x200?text=Category+Image';
</script>
  
<style scoped>
/* Добавление теней и округлений для карточек */
.card {
    border-radius: 0.5rem;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s;
}
.card:hover {
    transform: scale(1.05);
}
</style>