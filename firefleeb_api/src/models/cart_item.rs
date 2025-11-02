use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{cart::Cart, product::Product};
use crate::schema::cart_items;

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product, foreign_key = item_id))]
#[diesel(table_name = cart_items)]
pub struct CartItem {
    pub id: Uuid,
    pub item_id: Uuid,
    pub cart_id: Uuid,
    pub quantity: i32,
    pub unit_price: BigDecimal,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub item_id : Uuid,
    pub cart_id : Uuid,
    pub quantity : i32,
    pub unit_price : BigDecimal
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = cart_items)]
pub struct UpdateCartItem {
    pub quantity : Option<i32>,
    pub unit_price : Option<BigDecimal>
}

#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub item_id: Uuid,
    pub cart_id: Uuid,
    pub quantity: i32,
    pub unit_price: BigDecimal,
    pub total_price: BigDecimal, // quantity * unit_price
    pub created_at: Option<DateTime<Utc>>,
}

impl CartItem {
    /// Helper: compute total price = unit_price * qunaitiy
    pub fn total_price(&self) -> BigDecimal {
        &self.unit_price * BigDecimal::from(self.quantity as i64)
    }

    // Convert to response struct
    pub fn to_response(&self) -> CartItemResponse {
        CartItemResponse {
            id: self.id,
            item_id: self.item_id,
            cart_id: self.cart_id,
            quantity: self.quantity,
            unit_price: self.unit_price.clone(),
            total_price: self.total_price(),
            created_at: self.created_at,
        }
    }
}

impl NewCartItem {
    /// Validation: ensure quantity > 0 and unit_price >= 0
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= 0 {
            return Err("Quantity must be greater than 0".into());
        }
        let zero = BigDecimal::from(0);
        if self.unit_price < zero {
            return Err("Unit price must be non-negative".into());
        }
        Ok(())
    }
}