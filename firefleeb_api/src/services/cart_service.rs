use uuid::Uuid;

use crate::db::cart_repository;
use crate::db::{with_conn, PgPool};
use crate::errors::AppError;
use crate::errors::map_diesel_error;
use crate::models::cart::{NewCart, UpdateCart, Cart};

pub async fn create_cart(
    pool: PgPool,
    new_cart: NewCart,
) -> Result<Cart, AppError> {
    with_conn(pool, move |conn| {
        cart_repository::create_cart(conn, &new_cart)
    })
    .await
    .map_err(map_diesel_error)
}

pub async fn get_cart_by_user_id(
    pool: PgPool,
    user_id: Uuid,
) -> Result<Cart, AppError> {
    let maybe_cart = with_conn(pool, move |conn|{
        cart_repository::get_cart_by_user_id(conn, user_id)
    })
    .await
    .map_err(map_diesel_error)?;

    maybe_cart.ok_or_else(|| AppError::NotFound("Cart not found".into()))
}

pub async fn update_cart(
    pool: PgPool,
    cart_id: Uuid,
    updated: UpdateCart,
) -> Result<Cart, AppError> {
    with_conn(pool, move |conn| {
        cart_repository::update_cart(conn, cart_id, &updated)
    })
    .await
    .map_err(map_diesel_error)
}

pub async fn delete_cart(
    pool: PgPool,
    cart_id: Uuid,
) -> Result<(), AppError> {
    with_conn(pool, move |conn| {
        cart_repository::delete_cart(conn, cart_id)
    })
    .await
    .map_err(map_diesel_error)
    .and_then(|rows_deleted| {
        if rows_deleted == 0 {
            Err(AppError::NotFound("Cart not found".into()))
        } else {
            Ok(())
        }
    })
}