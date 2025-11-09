use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::products;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: Uuid,
    pub product_name: String,
    pub product_description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub product_name: String,
    pub product_description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
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
