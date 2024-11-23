<template>
    <nav class="bg-blue-500 text-white p-4">
      <div class="container mx-auto flex justify-between items-center">
        <NuxtLink to="/" class="text-lg font-bold hover:underline">Home</NuxtLink>
        <div>
          <NuxtLink to="/products" class="mr-4 hover:underline">Products</NuxtLink>
          <NuxtLink to="/categories" class="hover:underline">Categories</NuxtLink>
        </div>
      </div>
    </nav>
  </template>