use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::products;

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: uuid::Uuid,
    pub product_name: String,
    pub product_description: Option<String>,
    pub price: bigdecimal::BigDecimal,
    pub stock: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}