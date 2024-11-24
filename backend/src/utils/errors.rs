use serde::{Deserialize, Serialize};
use poem_openapi::Object;

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(code: u16, message: String) -> Self {
        ApiError { code, message }
    }
}