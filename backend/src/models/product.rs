use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Object, Serialize, Deserialize, FromRow)]
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
        if self.name.len() > 100 {
            return Err("Product name cannot exceed 100 characters".to_string());
        }
        if self.price <= 0.0 {
            return Err("Product price must be greater than 0".to_string());
        }
        if self.description.len() > 500 {
            return Err("Product description cannot exceed 500 characters".to_string());
        }   
        Ok(())
    }

    // TODO: implement formatted_price

    // pub fn formatted_price(&self) -> String {
    //     format!("{:.2} руб.", self.price)
    // }
}