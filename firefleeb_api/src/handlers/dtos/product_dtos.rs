use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub product_name: String,
    pub product_description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub price: Option<BigDecimal>,
    pub stock: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub product_name: String,
    pub product_description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub created_at: Option<DateTime<Utc>>,
}