use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category_id: Uuid,
    pub image_url: Option<String>,
}

impl Product {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Product name cannot be empty".to_string());
        }
        if self.price <= 0.0 {
            return Err("Product price must be greater than 0".to_string());
        }   
        Ok(())
    }

    pub fn formatted_price(&self) -> String {
        format!("{:.2} руб.", self.price)
    }
}