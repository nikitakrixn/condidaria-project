use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Object, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Category name cannot be empty".to_string());
        }
        Ok(())
    }
}