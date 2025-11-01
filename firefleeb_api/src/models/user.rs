use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;
use crate::types::email::Email;

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: Email,
    pub password_hash: String,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Email,
    pub created_at: Option<DateTime<Utc>>,
}