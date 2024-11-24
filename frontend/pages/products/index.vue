<template>
  <div class="container mx-auto p-4">
    <h1 class="text-2x1 font-bold mb-4">Список продуктов</h1>
    <NuxtLink to="/products/add" class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">Добавить продукт</NuxtLink>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 mt-4">
    </div>
  </div>
</template>