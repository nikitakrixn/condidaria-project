use poem_openapi::param::Path;
use poem_openapi::{payload::Json, OpenApi, ApiResponse, Tags};
use uuid::Uuid;
use crate::models::product::{CreateProductRequest, Product, UpdateProductRequest};
use crate::database::product_repository::ProductRepository;
use crate::utils::errors::ApiError;
use sqlx::PgPool;


#[derive(Tags)]
enum ApiTags {
    Products,
}

#[derive(ApiResponse)]
enum ProductResponse {
    #[oai(status = 200)]
    Ok(Json<Product>),
    #[oai(status = 201)]
    Created(Json<Product>),
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 401)]
    BadRequest(Json<ApiError>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}


#[derive(ApiResponse)]
pub enum ProductsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Product>>),
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

pub struct ProductApi {
    repository: ProductRepository,
}

impl ProductApi {
    pub fn new(pool: PgPool) -> Self {
        ProductApi { repository: ProductRepository::new(pool) }
    }
}


#[OpenApi]
impl ProductApi {
    #[oai(path = "/products", method = "get", tag = "ApiTags::Products")]
    async fn get_products(&self) -> ProductsResponse {
        match self.repository.get_all().await {
            Ok(products) => ProductsResponse::Ok(Json(products)),
            Err(_) => ProductsResponse::InternalServerError(Json(ApiError::new(500, "Failed to fetch products".to_string()))),
        }
    }

    #[oai(path = "/products/:id", method = "get", tag = "ApiTags::Products")]
    async fn get_product(&self, id: Path<Uuid>) -> ProductResponse {
        match self.repository.get_by_id(id.0).await {
            Ok(product) => ProductResponse::Ok(Json(product)),
            Err(sqlx::Error::RowNotFound) => ProductResponse::NotFound,
            Err(_) => ProductResponse::InternalServerError(Json(ApiError::new(500, "Failed to fetch product".to_string()))),
        }
    }



    #[oai(path = "/products", method = "post", tag = "ApiTags::Products")]
    async fn create_product(&self, product: Json<CreateProductRequest>) -> ProductResponse {
        let product = Product {
            id: Uuid::new_v4(),
            name: product.name.clone(),
            description: product.description.clone(),
            price: product.price,
            category_id: product.category_id,
            image_url: product.image_url.clone(),
        };

        match product.validate() {
            Ok(_) => {
                match self.repository.create(product).await {
                    Ok(product) => ProductResponse::Created(Json(product)),
                    Err(_) => ProductResponse::InternalServerError(Json(ApiError::new(500, "Failed to create product".to_string()))),
                }
            }
            Err(e) => {
                println!("{}", e);
                ProductResponse::BadRequest(Json(ApiError::new(400, e)))
            }
        }
    }


    #[oai(path = "/products/:id", method = "put", tag = "ApiTags::Products")]
    async fn update_product(&self, id: Path<Uuid>, product: Json<UpdateProductRequest>) -> ProductResponse {
        let product = Product {
            id: id.0, // Use the ID from the path
            name: product.name.clone(),
            description: product.description.clone(),
            price: product.price,
            category_id: product.category_id,
            image_url: product.image_url.clone(),
        };

        match product.validate() {
            Ok(_) => {
                match self.repository.update(id.0, &product).await {
                    Ok(_) => ProductResponse::Ok(Json(product)), // Return the updated product
                    Err(sqlx::Error::RowNotFound) => ProductResponse::NotFound,
                    Err(_) => ProductResponse::InternalServerError(Json(ApiError::new(500, "Failed to update product".to_string()))),
                }
            }
            Err(e) => {
                ProductResponse::BadRequest(Json(ApiError::new(400, e)))
            }
        }
    }

    #[oai(path = "/products/:id", method = "delete", tag = "ApiTags::Products")]
    async fn delete_product(&self, id: Path<Uuid>) -> ProductResponse {
        match self.repository.delete(id.0).await {
            Ok(_) => ProductResponse::NoContent,
            Err(sqlx::Error::RowNotFound) => ProductResponse::NotFound,
            Err(_) => ProductResponse::InternalServerError(Json(ApiError::new(500, "Failed to delete product".to_string()))),
        }
    }
}