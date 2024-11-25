use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{category::Category, product::Product};

pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        CategoryRepository { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create(&self, category: Category) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(
            "INSERT INTO categories (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(&category.name)
        .bind(&category.description)
        .fetch_one(&self.pool)
        .await;
        category
    }

    pub async fn update(&self, id: Uuid, category: &Category) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE categories SET name = $1, description = $2 WHERE id = $3",
        )
        .bind(&category.name)
        .bind(&category.description)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_products_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE category_id = $1")
            .bind(category_id)
            .fetch_all(&self.pool)
            .await
    }

    /*pub async fn get_all_paginated(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        sqlx::query_as::<_, Category>(
            "SELECT * FROM categories ORDER BY name LIMIT $1 OFFSET $2"
        )
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_filtered(
        &self,
        name: Option<String>,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        let query = if let Some(name) = name {
            sqlx::query_as::<_, Category>(
                "SELECT * FROM categories WHERE name ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
            )
            .bind(format!("%{}%", name))
            .bind(per_page as i64)
            .bind(offset as i64)
        } else {
            sqlx::query_as::<_, Category>(
                "SELECT * FROM categories ORDER BY name LIMIT $1 OFFSET $2"
            )
            .bind(per_page as i64)
            .bind(offset as i64)
        };
        query.fetch_all(&self.pool).await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_products_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE category_id = $1")
            .bind(category_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn exists(&self, category_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM categories WHERE id = $1"
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }*/
}