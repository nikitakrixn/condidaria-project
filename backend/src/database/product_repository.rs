use sqlx::PgPool;
use uuid::Uuid;

use crate::models::product::Product;



pub struct ProductRepository {
    pool: PgPool
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        ProductRepository { pool}
    }

    pub async fn get_all(&self) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM products").fetch_all(&self.pool).await
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1").bind(id).fetch_one(&self.pool).await
    }

    pub async fn create(&self, product: Product) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>("Insert Into products (name, description, price, category_id, image_url) VALUES ($1, $2, $3, $4, $5) RETURNING *")
        .bind(&product.name)
        .bind(&product.description)
        .bind(&product.price)
        .bind(&product.category_id)
        .bind(&product.image_url)
        .fetch_one(&self.pool).await
    }

    pub async fn update(&self, id: Uuid, product: &Product) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE products SET name = $1, description = $2, price = $3, category_id = $4, image_url = $5 WHERE id = $6"
        )
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.category_id)
        .bind(&product.image_url)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }


}