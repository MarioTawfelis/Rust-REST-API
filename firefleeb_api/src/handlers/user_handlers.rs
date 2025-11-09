use crate::db::PgPool;
use crate::errors::AppError;
use crate::handlers::dtos::{
    CreateUserRequest, LoginRequest, UpdatePasswordRequest, UpdateUserRequest, UserResponse,
};
use crate::models::user::UpdateUser;
use crate::services::user_service;
use uuid::Uuid;
use warp::{Reply, reply};

pub async fn register(pool: PgPool, req: CreateUserRequest) -> Result<impl Reply, AppError> {
    let user = user_service::register_user(pool, req.email, req.password).await?;
    Ok(reply::json(&UserResponse::from(user)))
}

pub async fn login(pool: PgPool, req: LoginRequest) -> Result<impl Reply, AppError> {
    let user = user_service::authenticate_user(pool, req.email, req.password).await?;
    Ok(reply::json(&UserResponse::from(user)))
}

pub async fn get(pool: PgPool, user_id: Uuid) -> Result<impl Reply, AppError> {
    let user = user_service::get_user_by_id(pool, user_id).await?;
    Ok(warp::reply::json(&UserResponse::from(user)))
}

pub async fn update_user(
    pool: PgPool,
    user_id: Uuid,
    req: UpdateUserRequest,
) -> Result<impl Reply, AppError> {
    let update = UpdateUser {
        email: req.email,
        password_hash: None,
    };

    let user = user_service::update_user(pool, user_id, update).await?;
    Ok(reply::json(&UserResponse::from(user)))
}

pub async fn change_password(
    pool: PgPool,
    user_id: Uuid,
    req: UpdatePasswordRequest,
) -> Result<impl Reply, AppError> {
    user_service::update_user_password(pool, user_id, req.old_password, req.new_password).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"message": "password updated"})),
        warp::http::StatusCode::OK,
    ))
}
