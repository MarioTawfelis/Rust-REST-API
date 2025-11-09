use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCartItemRequest {
    pub item_id: Uuid,
    pub quantity: i32,
    pub unit_price: BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartItemRequest {
    pub quantity: Option<i32>,
    pub unit_price: Option<BigDecimal>,
}
