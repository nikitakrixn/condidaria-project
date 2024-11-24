import { ref } from "vue";
import { useApi } from "./useApi";

export function useProducts() {
    const products = ref([]);

    const fetchProducts = async () => {
        const { data } = await useApi('/products');
        if (data.value) {
            products.value = data.value
        }
    };

    const getProductById = async (id) => {
        return await useApi(`/products/${id}`);
    };

    const createProduct = async (product) => {
        return await useApi('/products', {
            method: 'POST',
            data: product
        });
    };

    const updateProduct = async (product) => {
        return await useApi(`/products/${product.id}`, {
            method: 'PUT',
            data: product
        });
    };

    const deleteProduct = async (id) => {
        return await useApi(`/products/${id}`, {
            method: 'DELETE'
        });
    };

    return {
        products,
        fetchProducts,
        getProductById,
        createProduct,
        updateProduct,
        deleteProduct
    };
}