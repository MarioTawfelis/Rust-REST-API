use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::models::product::Product;

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

impl From<Product> for ProductResponse {
    fn from(m: Product) -> Self {
        Self {
            id: m.id,
            product_name: m.product_name,
            product_description: m.product_description,
            price: m.price,
            stock: m.stock,
            created_at: m.created_at,
        }
    }
}