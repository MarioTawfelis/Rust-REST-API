use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::carts;

use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[table_name = "carts"]
pub struct Cart {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub cart_status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}