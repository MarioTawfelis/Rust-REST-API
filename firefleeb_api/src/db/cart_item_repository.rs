use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use uuid::Uuid;

use crate::models::cart_item::{CartItem, NewCartItem, UpdateCartItem};
use crate::schema::cart_items;

/// Insert a new cart item row (no merging). Fails if (cart_id, item_id) already exists.
pub fn create_cart_item(
    conn: &mut PgConnection,
    new_cart_item: &NewCartItem,
) -> QueryResult<CartItem> {
    dbg!(new_cart_item);
    diesel::insert_into(cart_items::table)
        .values(new_cart_item)
        .get_result::<CartItem>(conn)
        .inspect_err(|e| eprintln!("insert cart_item failed: {e:?}"))
}

/// Return all items for a cart
pub fn get_items_by_cart_id(conn: &mut PgConnection, cart_id: Uuid) -> QueryResult<Vec<CartItem>> {
    cart_items::table
        .filter(cart_items::cart_id.eq(cart_id))
        .order_by(cart_items::created_at.asc().nulls_last())
        .load::<CartItem>(conn)
}

/// Set quantity (0 removes the item).
pub fn set_item_quantity(
    conn: &mut PgConnection,
    cart_id: Uuid,
    product_id: Uuid,
    qty: i32,
) -> QueryResult<CartItem> {
    use crate::schema::cart_items::dsl as ci;

    if qty < 0 {
        return Err(diesel::result::Error::RollbackTransaction);
    }
    if qty == 0 {
        // Delete and return NotFound if it didn't exist
        let deleted = diesel::delete(
            ci::cart_items
                .filter(ci::cart_id.eq(cart_id))
                .filter(ci::item_id.eq(product_id)),
        )
        .execute(conn)?;
        if deleted == 0 {
            return Err(diesel::result::Error::NotFound);
        }
        // No row to return; pattern is to re-query items/total at the service layer
        return Err(diesel::result::Error::NotFound);
    }

    diesel::update(
        ci::cart_items
            .filter(ci::cart_id.eq(cart_id))
            .filter(ci::item_id.eq(product_id)),
    )
    .set(ci::quantity.eq(qty))
    .get_result::<CartItem>(conn)
}

/// Update quantity/unit price for a specific cart item.
pub fn update_cart_item(
    conn: &mut PgConnection,
    cart_id: Uuid,
    product_id: Uuid,
    updated: &UpdateCartItem,
) -> QueryResult<CartItem> {
    use crate::schema::cart_items::dsl as ci;

    diesel::update(
        ci::cart_items
            .filter(ci::cart_id.eq(cart_id))
            .filter(ci::item_id.eq(product_id)),
    )
    .set(updated)
    .get_result::<CartItem>(conn)
}

/// Remove one item row
pub fn delete_item(conn: &mut PgConnection, cart_id: Uuid, product_id: Uuid) -> QueryResult<usize> {
    use crate::schema::cart_items::dsl as ci;

    diesel::delete(
        ci::cart_items
            .filter(ci::cart_id.eq(cart_id))
            .filter(ci::item_id.eq(product_id)),
    )
    .execute(conn)
}

/// Remove all items for a cart
pub fn delete_all_for_cart(conn: &mut PgConnection, cart_id: Uuid) -> QueryResult<usize> {
    use crate::schema::cart_items::dsl as ci;

    diesel::delete(ci::cart_items.filter(ci::cart_id.eq(cart_id))).execute(conn)
}
