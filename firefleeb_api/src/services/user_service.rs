use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::db::user_repository;
use crate::db::{with_conn, PgPool};
use crate::errors::AppError;
use crate::models::user::{NewUser, UpdateUser, User};
use crate::types::email::Email;
use crate::errors::map_diesel_error;

pub async fn register_user(
    pool: PgPool,
    email: Email,
    password_plain: String
) -> Result<User, AppError> {
    validate_password(&password_plain)?;

    let password_hash = hash(&password_plain, DEFAULT_COST)
        .map_err(|_| AppError::Internal("Failed to hash password".into()))?;

    let new_user = NewUser { email, password_hash };

    with_conn(pool, move |conn| user_repository::create_user(conn, &new_user))
        .await
        .map_err(map_diesel_error)
}

pub async fn authenticate_user(
    pool: PgPool,
    email: Email,
    password_plain: String
) -> Result<User, AppError> {
    let maybe_user = with_conn(pool.clone(), move |conn| {
        user_repository::get_user_by_email(conn, &email)
    })
    .await
    .map_err(map_diesel_error)?;

    let user = maybe_user.ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    let ok = verify(&password_plain, &user.password_hash)
        .map_err(|_| AppError::Internal("Failed to verify password".into()))?;
    if !ok {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    Ok(user)
}

pub async fn get_user_by_id(
    pool: PgPool,
    user_id: Uuid,
) -> Result<User, AppError> {
    let maybe_user = with_conn(pool, move |conn| {
        user_repository::get_user_by_id(conn, user_id)
    })
    .await
    .map_err(map_diesel_error)?;

    maybe_user.ok_or_else(|| AppError::NotFound("User not found".into()))
}

/// Forgot my password
pub async fn update_user_password(
    pool: PgPool,
    user_id: Uuid,
    old_password_plain: String,
    new_password_plain: String,
    update: UpdateUser
) -> Result<User, AppError> {
    validate_password(&new_password_plain)?;

    let user = get_user_by_id(pool.clone(), user_id).await?;

    let ok = verify(&old_password_plain, &user.password_hash)
        .map_err(|_| AppError::Internal("Failed to verify current password".into()))?;
    if !ok {
        return Err(AppError::Unauthorized("Invalid current password".into()));
    }

    let updated_user = with_conn(pool, move |conn| {
        user_repository::update_user(conn, user_id, &update)
    })
    .await
    .map_err(map_diesel_error)?;

    Ok(updated_user)
}

pub async fn delete_user(
    pool: PgPool,
    user_id: Uuid
) -> Result<(), AppError> {
    let rows = with_conn(pool, move |conn| {
        user_repository::delete_user(conn, user_id)
    })
    .await
    .map_err(map_diesel_error)?;

    if rows == 0 {
        return Err(AppError::NotFound("User not found".into()));
    } 

    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation("Password must be at least 8 characters long".into()));
    }
    Ok(())
}
