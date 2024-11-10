use poem::http::StatusCode;
use poem_openapi::{param::Path, payload::{Json, PlainText}, ApiResponse, OpenApi};
use sqlx::Error;
use uuid::Uuid;

use crate::{database::product_repository::ProductRepository, models::product::Product};

#[derive(ApiResponse)]
enum GetProductResponse {
    #[oai(status = 200)]
    Product(Json<Product>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}


pub struct ProductsApi {
    repository: ProductRepository,
}

impl ProductsApi {
    pub fn new(repository: ProductRepository) -> Self {
        Self { repository }
    }
}

#[OpenApi]
impl ProductApi {
    #[oai(path = "/products", method = "get")]
    async fn get_products(&self) -> Result<Json<Vec<Product>>, poem::Error> {
        match self.repository.get_all().await {
            Ok(products) => Ok(Json(products)),
            Err(e) => Err(e.into()),
        }
    }

    #[oai(path = "/products/:id", method = "get")]
    async fn get_product(&self, id: Path<String>) -> GetProductResponse {
        let result = self.repository.get_by_id(id.0).await;

        match result {
            Ok(Some(product)) => GetProductResponse::Product(Json(product)),
            Ok(None) => GetProductResponse::NotFound(PlainText(format!("Product with ID {} not found", id.0))),
            Err(e) => {
                // Логирование ошибки
                eprintln!("Database error: {}", e);
                GetProductResponse::NotFound(PlainText("Failed to fetch product".to_string()))
            },
        }
    }

    #[oai(path = "/products", method = "post")]
    async fn create_product(&self, product: Json<Product>) -> Result<Json<Product>> {
        match self.repository.create(&product).await {
            Ok(product) => Ok(Json(product)),
            Err(e) => Err(Error::from(e).into()),
        }
    }

    #[oai(path = "/products/:id", method = "put")]
    async fn update_product(&self, id: Path<String>, product: Json<Product>) -> Result<Json<Product>> {
        self.repository.update(id.0, &product).await.map_err(Error::from)?;

        // Извлекаем обновленный продукт
        let updated_product = self.repository.get_by_id(id.0).await
            .map_err(|err| Error::from(err))?
            .ok_or(poem::Error::from_string("Product not found after update", StatusCode::NOT_FOUND))?;

        Ok(Json(updated_product))
    }

    #[oai(path = "/products/:id", method = "delete")]
    async fn delete_product(&self, id: Path<String>) -> Result<()> {
        match self.repository.delete(id.0).await {
            Ok(_) => Ok(()),
            Err(sqlx::Error::RowNotFound) => Err(poem::Error::from_string("Product not found", StatusCode::NOT_FOUND)),
            Err(e) => Err(Error::from(e).into()),
        }
    }
}