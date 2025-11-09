use uuid::Uuid;
use warp::{Reply, http::StatusCode, reply};

use crate::db::PgPool;
use crate::errors::AppError;
use crate::handlers::dtos::{CreateCartItemRequest, UpdateCartItemRequest};
use crate::models::cart_item::{CartItemResponse, NewCartItem, UpdateCartItem};
use crate::services::cart_item_service;

pub async fn list(pool: PgPool, cart_id: Uuid) -> Result<impl Reply, AppError> {
    let items = cart_item_service::list_items(pool, cart_id).await?;
    let response: Vec<CartItemResponse> = items.into_iter().map(|i| i.to_response()).collect();
    Ok(reply::json(&response))
}

pub async fn create(
    pool: PgPool,
    cart_id: Uuid,
    req: CreateCartItemRequest,
) -> Result<impl Reply, AppError> {
    let new_item = NewCartItem {
        cart_id,
        item_id: req.item_id,
        quantity: req.quantity,
        unit_price: req.unit_price,
    };

    let item = cart_item_service::add_item(pool, new_item).await?;
    Ok(reply::with_status(
        reply::json(&item.to_response()),
        StatusCode::CREATED,
    ))
}

pub async fn update(
    pool: PgPool,
    cart_id: Uuid,
    item_id: Uuid,
    req: UpdateCartItemRequest,
) -> Result<impl Reply, AppError> {
    let updates = UpdateCartItem {
        quantity: req.quantity,
        unit_price: req.unit_price,
    };

    let response = match cart_item_service::update_item(pool, cart_id, item_id, updates).await? {
        Some(item) => reply::json(&item.to_response()).into_response(),
        None => reply::with_status(
            reply::json(&serde_json::json!({ "message": "cart item removed" })),
            StatusCode::NO_CONTENT,
        )
        .into_response(),
    };

    Ok(response)
}

pub async fn delete(pool: PgPool, cart_id: Uuid, item_id: Uuid) -> Result<impl Reply, AppError> {
    cart_item_service::remove_item(pool, cart_id, item_id).await?;
    Ok(reply::with_status(
        reply::json(&serde_json::json!({ "message": "cart item deleted" })),
        StatusCode::NO_CONTENT,
    ))
}

pub async fn clear(pool: PgPool, cart_id: Uuid) -> Result<impl Reply, AppError> {
    cart_item_service::clear_cart(pool, cart_id).await?;
    Ok(reply::with_status(
        reply::json(&serde_json::json!({ "message": "cart cleared" })),
        StatusCode::NO_CONTENT,
    ))
}
