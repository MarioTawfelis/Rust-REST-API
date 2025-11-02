use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;
use crate::types::email::Email;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: Email,
    pub password_hash: String,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Email,
    pub created_at: Option<DateTime<Utc>>,
}