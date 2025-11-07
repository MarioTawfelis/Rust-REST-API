use uuid::Uuid;

use diesel::prelude::*;
use diesel::{QueryResult, PgConnection};

use crate::models::cart::{Cart, NewCart, UpdateCart};
use crate::schema::carts;

pub fn create_cart(
    conn: &mut PgConnection,
    new_cart: &NewCart,
) -> QueryResult<Cart> {
    diesel::insert_into(carts::table)
        .values(new_cart)
        .get_result(conn)
    }

pub fn get_cart_by_id(
    conn: &mut PgConnection,
    cart_id: Uuid,
) -> QueryResult<Option<Cart>> {
    carts::table
        .filter(carts::id.eq(cart_id))
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

pub fn delete_cart(
    conn: &mut PgConnection,
    cart_id: Uuid,
) -> QueryResult<usize> {
    diesel::delete(carts::table.find(cart_id))
        .execute(conn)
}