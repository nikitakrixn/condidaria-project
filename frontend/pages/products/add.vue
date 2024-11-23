<!-- pages/products/add.vue -->
<template>
    <div class="max-w-4xl mx-auto p-6 bg-white rounded-lg shadow-lg mt-8">
      <h1 class="text-3xl font-bold mb-6 text-center text-pink-600">Add New Product</h1>
  
      <form @submit.prevent="submitForm" novalidate>
        <!-- Name -->
        <div class="mb-4">
          <label for="name" class="block text-gray-700 font-semibold mb-2">Product Name</label>
          <input
            id="name"
            v-model="product.name"
            type="text"
            class="w-full p-3 border rounded focus:outline-none focus:ring-2 focus:ring-pink-500"
            :class="{ 'border-red-500': errors.name }"
            placeholder="Enter product name"
          />
          <p v-if="errors.name" class="text-red-500 mt-1">{{ errors.name }}</p>
        </div>
  
        <!-- Description -->
        <div class="mb-4">
          <label for="description" class="block text-gray-700 font-semibold mb-2">Description</label>
          <textarea
            id="description"
            v-model="product.description"
            class="w-full p-3 border rounded focus:outline-none focus:ring-2 focus:ring-pink-500"
            :class="{ 'border-red-500': errors.description }"
            placeholder="Enter product description"
          ></textarea>
          <p v-if="errors.description" class="text-red-500 mt-1">{{ errors.description }}</p>
        </div>
  
        <!-- Price -->
        <div class="mb-4">
          <label for="price" class="block text-gray-700 font-semibold mb-2">Price (in руб.)</label>
          <input
            id="price"
            v-model.number="product.price"
            type="number"
            class="w-full p-3 border rounded focus:outline-none focus:ring-2 focus:ring-pink-500"
            :class="{ 'border-red-500': errors.price }"
            placeholder="Enter product price"
          />
          <p v-if="errors.price" class="text-red-500 mt-1">{{ errors.price }}</p>
        </div>
  
        <!-- Category -->
        <div class="mb-4">
          <label for="category" class="block text-gray-700 font-semibold mb-2">Category</label>
          <select
            id="category"
            v-model="product.category_id"
            class="w-full p-3 border rounded focus:outline-none focus:ring-2 focus:ring-pink-500"
            :class="{ 'border-red-500': errors.category_id }"
            >
            <option value="" disabled>Select a category</option>
            <option
                v-for="category in categories.value"
                :key="category.id"
                :value="category.id"
            >
                {{ category.name }}
            </option>
            </select>
          <p v-if="errors.category_id" class="text-red-500 mt-1">{{ errors.category_id }}</p>
        </div>
  
        <!-- Image URL -->
        <div class="mb-4">
          <label for="image_url" class="block text-gray-700 font-semibold mb-2">Image URL</label>
          <input
            id="image_url"
            v-model="product.image_url"
            type="text"
            class="w-full p-3 border rounded focus:outline-none focus:ring-2 focus:ring-pink-500"
            placeholder="Enter image URL (optional)"
          />
        </div>
  
        <!-- Submit Button -->
        <div class="text-center">
          <button
            type="submit"
            class="px-6 py-3 bg-pink-600 text-white rounded hover:bg-pink-700 transition-colors"
          >
            Add Product
          </button>
        </div>
      </form>
  
      <!-- Success Message -->
      <p v-if="successMessage" class="mt-4 text-green-500 text-center">{{ successMessage }}</p>
  
      <!-- Error Message -->
      <p v-if="formError" class="mt-4 text-red-500 text-center">{{ formError }}</p>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted } from 'vue';
  import { useApi } from '~/composables/useApi';
  
  const product = ref({
    name: '',
    description: '',
    price: 0,
    category_id: '',
    image_url: '',
  });
  
  const categories = ref([]);
  const errors = ref({});
  const successMessage = ref('');
  const formError = ref('');
  
  // Fetch categories on component mount
  onNuxtReady(async () => {
    try {
      const { data } = await useApi('/categories');
      categories.value = data;
      console.log('Fetched categories:', categories.value);
    } catch (error) {
      console.error('Failed to fetch categories:', error);
    }
  });
  
  // Form submission
  async function submitForm() {
    // Clear previous messages and errors
    successMessage.value = '';
    formError.value = '';
    errors.value = {};
  
    // Basic validation
    if (!product.value.name) {
      errors.value.name = 'Product name is required.';
    }
    if (!product.value.description) {
      errors.value.description = 'Description is required.';
    }
    if (!product.value.price || product.value.price <= 0) {
      errors.value.price = 'Price must be greater than 0.';
    }
    if (!product.value.category_id) {
      errors.value.category_id = 'Please select a category.';
    }
  
    // If there are validation errors, stop submission
    if (Object.keys(errors.value).length > 0) {
      return;
    }
  
    try {
      // Send product data to the server
      const { data, error } = await useApi('/products', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(product.value),
      });

      console.log('Response data:', data);
      console.log('Response error:', error);

  
      if (error) {
        formError.value = 'Failed to add product. Please try again later.';
        console.error('Error:', error);
        return;
      } else {
        successMessage.value = 'Product added successfully!';
        // Reset the form
        product.value = {
          name: '',
          description: '',
          price: 0,
          category_id: '',
          image_url: '',
        };
      }
    } catch (err) {
      formError.value = 'An error occurred while adding the product.';
      console.error(err);
    }
  }
  </script>
  
  <style scoped>
  /* Дополнительные стили для формы */
  input, textarea, select {
    border: 1px solid #ccc;
    transition: border-color 0.3s;
  }
  input:focus, textarea:focus, select:focus {
    border-color: #ff66b2;
  }
  </style>