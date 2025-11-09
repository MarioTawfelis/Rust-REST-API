use bigdecimal::{BigDecimal, Zero};
use uuid::Uuid;

use crate::db::{PgPool, cart_item_repository, cart_repository, with_conn};
use crate::errors::{AppError, map_diesel_error};
use crate::models::cart_item::{CartItem, NewCartItem, UpdateCartItem};

pub async fn list_items(pool: PgPool, cart_id: Uuid) -> Result<Vec<CartItem>, AppError> {
    with_conn(pool, move |conn| {
        cart_item_repository::get_items_by_cart_id(conn, cart_id)
    })
    .await
    .map_err(map_diesel_error)
}

pub async fn add_item(pool: PgPool, new_item: NewCartItem) -> Result<CartItem, AppError> {
    if let Err(msg) = new_item.validate() {
        return Err(AppError::Validation(msg));
    }

    with_conn(pool, move |conn| {
        let item = cart_item_repository::create_cart_item(conn, &new_item)?;
        recalc_cart_total(conn, new_item.cart_id)?;
        Ok(item)
    })
    .await
    .map_err(map_diesel_error)
}

pub async fn update_item(
    pool: PgPool,
    cart_id: Uuid,
    item_id: Uuid,
    mut updates: UpdateCartItem,
) -> Result<Option<CartItem>, AppError> {
    if updates.quantity.is_none() && updates.unit_price.is_none() {
        return Err(AppError::Validation(
            "At least one field must be provided".into(),
        ));
    }

    if let Some(qty) = updates.quantity {
        if qty < 0 {
            return Err(AppError::Validation(
                "Quantity must be zero or greater".into(),
            ));
        }
        if qty == 0 {
            remove_item(pool, cart_id, item_id).await?;
            return Ok(None);
        }
        updates.quantity = Some(qty);
    }

    if let Some(price) = updates.unit_price.as_ref() {
        if price < &BigDecimal::zero() {
            return Err(AppError::Validation(
                "Unit price must be zero or greater".into(),
            ));
        }
    }

    with_conn(pool, move |conn| {
        let item = cart_item_repository::update_cart_item(conn, cart_id, item_id, &updates)?;
        recalc_cart_total(conn, cart_id)?;
        Ok(item)
    })
    .await
    .map(|item| Some(item))
    .map_err(map_diesel_error)
}

pub async fn remove_item(pool: PgPool, cart_id: Uuid, item_id: Uuid) -> Result<(), AppError> {
    let deleted = with_conn(pool, move |conn| {
        let deleted = cart_item_repository::delete_item(conn, cart_id, item_id)?;
        if deleted > 0 {
            recalc_cart_total(conn, cart_id)?;
        }
        Ok(deleted)
    })
    .await
    .map_err(map_diesel_error)?;

    if deleted == 0 {
        Err(AppError::NotFound("Cart item not found".into()))
    } else {
        Ok(())
    }
}

pub async fn clear_cart(pool: PgPool, cart_id: Uuid) -> Result<(), AppError> {
    with_conn(pool, move |conn| {
        cart_item_repository::delete_all_for_cart(conn, cart_id)?;
        recalc_cart_total(conn, cart_id)?;
        Ok(())
    })
    .await
    .map_err(map_diesel_error)
}

fn recalc_cart_total(conn: &mut diesel::PgConnection, cart_id: Uuid) -> diesel::QueryResult<()> {
    use std::ops::AddAssign;

    let items = cart_item_repository::get_items_by_cart_id(conn, cart_id)?;
    let mut total = BigDecimal::zero();
    for item in items {
        let qty = BigDecimal::from(item.quantity as i64);
        total.add_assign(item.unit_price * qty);
    }

    cart_repository::update_cart_total(conn, cart_id, &total).map(|_| ())
}
