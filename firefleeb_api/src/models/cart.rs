use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::carts;
use crate::models::user::User;

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = carts)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cart_status: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cart_status: String,
    pub created_at: Option<DateTime<Utc>>,
}