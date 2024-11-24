use poem_openapi::param::Path;
use poem_openapi::{payload::Json, OpenApi, ApiResponse, Object, Tags};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::category::Category;
use crate::database::category_repository::CategoryRepository;
use crate::utils::errors::ApiError;

use super::products::ProductsResponse;

#[derive(Tags)]
enum ApiTags {
    Categories,
}

#[derive(ApiResponse)]
enum CategoryResponse {
    #[oai(status = 200)]
    Ok(Json<Category>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum CategoriesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Category>>),
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(Object)]
struct CreateCategoryRequest {
    name: String,
    description: Option<String>,
}

#[derive(Object)]
struct UpdateCategoryRequest {
    name: String,
    description: Option<String>,
}

pub struct CategoryApi {
    repository: CategoryRepository,
}

impl CategoryApi {
    pub fn new(pool: PgPool) -> Self {
        CategoryApi {
            repository: CategoryRepository::new(pool),
        }
    }
}

#[OpenApi]
impl CategoryApi {
    #[oai(path = "/categories", method = "get", tag = "ApiTags::Categories")]
    async fn get_categories(&self) -> CategoriesResponse {
        match self.repository.get_all().await {
            Ok(categories) => CategoriesResponse::Ok(Json(categories)),
            Err(_) => CategoriesResponse::InternalServerError,
        }
    }

    #[oai(path = "/categories/:id", method = "get", tag = "ApiTags::Categories")]
    async fn get_category(&self, id: Path<Uuid>) -> CategoryResponse {
        match self.repository.get_by_id(id.0).await {
            Ok(category) => CategoryResponse::Ok(Json(category)),
            Err(sqlx::Error::RowNotFound) => CategoryResponse::NotFound,
            Err(_) => CategoryResponse::InternalServerError,
        }
    }

    #[oai(path = "/categories", method = "post", tag = "ApiTags::Categories")]
    async fn create_category(&self, category: Json<CreateCategoryRequest>) -> CategoryResponse {

        let new_category = Category {
            id: Uuid::new_v4(),
            name: category.name.clone(),
            description: category.description.clone(),
        };

        match new_category.validate() {
            Ok(_) => match self.repository.create(new_category).await {
                Ok(category) => CategoryResponse::Ok(Json(category)),
                Err(_) => CategoryResponse::InternalServerError,
            },
            Err(e) => {
                println!("{}", e);
                CategoryResponse::InternalServerError
            }
        }
    }

    #[oai(path = "/categories/:id", method = "put", tag = "ApiTags::Categories")]
    async fn update_category(
        &self,
        id: Path<Uuid>,
        category: Json<UpdateCategoryRequest>,
    ) -> CategoryResponse {
        let updated_category = Category {
            id: id.0,
            name: category.name.clone(),
            description: category.description.clone(),
        };

        match updated_category.validate() {
            Ok(_) => match self.repository.update(id.0, &updated_category).await {
                Ok(_) => CategoryResponse::Ok(Json(updated_category)),
                Err(sqlx::Error::RowNotFound) => CategoryResponse::NotFound,
                Err(_) => CategoryResponse::InternalServerError,
            },
            Err(e) => {
                println!("{}", e);
                CategoryResponse::InternalServerError
            }
        }
    }

    #[oai(path = "/categories/:id", method = "delete", tag = "ApiTags::Categories")]
    async fn delete_category(&self, id: Path<Uuid>) -> CategoryResponse {
        match self.repository.delete(id.0).await {
            Ok(_) => CategoryResponse::Ok(Json(Category {
                id: id.0,
                name: "".to_string(),
                description: None,
            })),
            Err(sqlx::Error::RowNotFound) => CategoryResponse::NotFound,
            Err(_) => CategoryResponse::InternalServerError,
        }
    }

    #[oai(path = "/categories/:id/products", method = "get", tag = "ApiTags::Categories")]
    async fn get_products_by_category(&self, id: Path<Uuid>) -> ProductsResponse {
        match self.repository.get_products_by_category(id.0).await {
            Ok(products) => ProductsResponse::Ok(Json(products)),
            Err(_) => ProductsResponse::InternalServerError(Json(ApiError::new(500, "Failed to fetch product".to_string()))),
        }
    }
    
}