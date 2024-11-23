use poem_openapi::param::Path;
use poem_openapi::{payload::Json, OpenApi, ApiResponse, Object, Tags};
use uuid::Uuid;
use crate::models::product::Product;
use crate::database::product_repository::ProductRepository;
use sqlx::PgPool;


#[derive(Tags)]
enum ApiTags {
    Products,
}

#[derive(ApiResponse)]
enum ProductResponse {
    #[oai(status = 200)]
    Ok(Json<Product>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError,
}


#[derive(ApiResponse)]
enum ProductsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Product>>),
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(Object)]
struct CreateProductRequest {
    name: String,
    description: String,
    price: f64,
    category_id: Uuid,
    image_url: Option<String>,
}

#[derive(Object)]
struct UpdateProductRequest {
    name: String,
    description: String,
    price: f64,
    category_id: Uuid,
    image_url: Option<String>,
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
            Err(_) => ProductsResponse::InternalServerError,
        }
    }

    #[oai(path = "/products/:id", method = "get", tag = "ApiTags::Products")]
    async fn get_product(&self, id: Path<Uuid>) -> ProductResponse {
        match self.repository.get_by_id(id.0).await {
            Ok(product) => ProductResponse::Ok(Json(product)),
            Err(sqlx::Error::RowNotFound) => ProductResponse::NotFound,
            Err(_) => ProductResponse::InternalServerError,
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
                    Ok(product) => ProductResponse::Ok(Json(product)),
                    Err(_) => ProductResponse::InternalServerError,
                }
            }
            Err(e) => {
                println!("{}", e);
                ProductResponse::InternalServerError
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
                    Err(_) => ProductResponse::InternalServerError,
                }
            }
            Err(e) => {
                println!("{}", e);
                ProductResponse::InternalServerError
            }
        }
    }

    #[oai(path = "/products/:id", method = "delete", tag = "ApiTags::Products")]
    async fn delete_product(&self, id: Path<Uuid>) -> ProductResponse {
        match self.repository.delete(id.0).await {
            Ok(_) => ProductResponse::Ok(Json(Product { // Return a placeholder product since the real one is deleted.  Consider returning just a 204 No Content.
                id: Uuid::nil(), 
                name: "".to_string(),
                description: "".to_string(),
                price: 0.0,
                category_id: Uuid::nil(),
                image_url: None,
            })),
            Err(sqlx::Error::RowNotFound) => ProductResponse::NotFound,
            Err(_) => ProductResponse::InternalServerError,
        }
    }
}