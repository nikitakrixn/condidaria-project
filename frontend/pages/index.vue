<template>
    <div class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-blue-500 to-blue-700 text-white">
      <h1 class="text-5xl font-bold mb-4">Welcome to My Nuxt 3 App</h1>
      <p class="text-lg mb-8">This is a sample application built with Nuxt 3 and Rust backend.</p>
  
      <div class="flex space-x-4">
        <NuxtLink
          to="/products"
          class="px-6 py-3 bg-white text-blue-500 font-semibold rounded shadow hover:bg-gray-100"
        >
          View Products
        </NuxtLink>
        <NuxtLink
          to="/categories"
          class="px-6 py-3 bg-white text-blue-500 font-semibold rounded shadow hover:bg-gray-100"
        >
          View Categories
        </NuxtLink>
      </div>
    </div>
  </template>