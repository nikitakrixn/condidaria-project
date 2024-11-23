export const useApi = (endpoint, opts) => {
    const config = useRuntimeConfig();

    return useFetch(endpoint, {
        baseURL: config.public.backendUrl,
        ...opts
    });
};