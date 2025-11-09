use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::types::email::Email;
use serde::{Deserialize, Serialize};

use crate::models::user::User;


#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: Email,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<Email>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: Email,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Email,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(m: User) -> Self {
        Self {
            id: m.id,
            email: m.email,
            created_at: m.created_at,
        }
    }
}