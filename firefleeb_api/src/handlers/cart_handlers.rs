use crate::handlers::dtos::{CreateCartRequest, UpdateCartRequest, CartResponse};
use crate::models::cart::{NewCart, UpdateCart};
use crate::services::cart_service;
use crate::errors::AppError;
use crate::db::PgPool;
use warp::{Reply, reply};
use uuid::Uuid;

pub async fn create(
    pool: PgPool,
    req: CreateCartRequest
) -> Result<impl Reply, AppError> {
    let new_cart = NewCart {
        user_id: req.user_id,
        cart_status: req.cart_status,
        cart_total: req.cart_total,
    };

    let cart = cart_service::create_cart(pool, new_cart).await?;
    Ok(reply::json(&CartResponse::from(cart)))
}

pub async fn update(
    pool: PgPool,
    cart_id: Uuid,
    req: UpdateCartRequest
) -> Result<impl Reply, AppError> {
    let updated_cart = UpdateCart {
        cart_status: req.cart_status,
        cart_total: req.cart_total,
    };

    let cart = cart_service::update_cart(pool, cart_id, updated_cart).await?;
    Ok(reply::json(&CartResponse::from(cart)))
}

pub async fn get(pool: PgPool, id: Uuid) -> Result<impl Reply, AppError> {
    let cart = cart_service::get_cart_by_user_id(pool, id).await?;
    Ok(warp::reply::json(&CartResponse::from(cart)))
}

pub async fn delete(pool: PgPool, id: Uuid) -> Result<impl Reply, AppError> {
    cart_service::delete_cart(pool, id).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"message": "deleted"})),
        warp::http::StatusCode::NO_CONTENT,
    ))
}