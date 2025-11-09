use bigdecimal::BigDecimal;
use uuid::Uuid;

use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

use crate::models::cart::{Cart, NewCart, UpdateCart};
use crate::schema::carts;

pub fn create_default_cart(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Cart> {
    let new_cart = NewCart {
        user_id,
        cart_status: "active".into(),
        cart_total: BigDecimal::from(0),
    };

    diesel::insert_into(carts::table)
        .values(&new_cart)
        .get_result::<Cart>(conn)
}

pub fn get_cart_by_id(conn: &mut PgConnection, cart_id: Uuid) -> QueryResult<Option<Cart>> {
    carts::table.find(cart_id).first::<Cart>(conn).optional()
}

pub fn get_active_by_user_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Option<Cart>> {
    carts::table
        .filter(carts::user_id.eq(user_id))
        .filter(carts::cart_status.eq("active"))
        .first::<Cart>(conn)
        .optional()
}

pub fn update_cart(
    conn: &mut PgConnection,
    cart_id: Uuid,
    updated: &UpdateCart,
) -> QueryResult<Cart> {
    diesel::update(carts::table.find(cart_id))
        .set(updated)
        .get_result(conn)
}

pub fn update_cart_total(
    conn: &mut PgConnection,
    cart_id: Uuid,
    new_total: &BigDecimal,
) -> QueryResult<Cart> {
    diesel::update(carts::table.find(cart_id))
        .set(carts::cart_total.eq(new_total))
        .get_result(conn)
}

pub fn delete_cart(conn: &mut PgConnection, cart_id: Uuid) -> QueryResult<usize> {
    diesel::delete(carts::table.find(cart_id)).execute(conn)
}
