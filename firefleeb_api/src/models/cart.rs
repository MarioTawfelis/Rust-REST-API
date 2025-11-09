use core::str;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;
use crate::schema::carts;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cart_status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub cart_total: BigDecimal,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: Uuid,
    pub cart_status: String,
    pub cart_total: BigDecimal,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = carts)]
pub struct UpdateCart {
    pub cart_status: Option<String>,
    pub cart_total: Option<BigDecimal>,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cart_status: String,
    pub cart_total: BigDecimal,
    pub created_at: Option<DateTime<Utc>>,
}
