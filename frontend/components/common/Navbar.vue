<template>
  <nav class="bg-white shadow-md">
    <div class="container mx-auto px-4 py-4 flex justify-between items-center">
      <NuxtLink to="/" class="text-xl font-bold text-gray-800">КондиДария</NuxtLink>
      <div class="space-x-4">
        <NuxtLink to="/products" class="text-gray-800 hover:text-gray-600">Продукты</NuxtLink>
        <NuxtLink to="/admin/products" class="text-gray-800 hover:text-gray-600">Админ</NuxtLink>
      </div>
    </div>
  </nav>
</template>