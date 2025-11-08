use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::models::cart::Cart;

#[derive(Debug, Deserialize)]
pub struct CreateCartRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartRequest {
    pub cart_status: Option<String>,
    pub cart_total: Option<BigDecimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartResponse {
    pub cart_id: Uuid,
    pub user_id: Uuid,
    pub cart_status: String,
    pub cart_total: BigDecimal,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<Cart> for CartResponse {
    fn from(m: Cart) -> Self {
        Self {
            cart_id: m.id,
            user_id: m.user_id,
            cart_status: m.cart_status,
            cart_total: m.cart_total,
            created_at: m.created_at,
        }
    }
}
