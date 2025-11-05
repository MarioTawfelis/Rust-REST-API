use uuid::Uuid;
use bigdecimal::BigDecimal;

use diesel::prelude::*;
use diesel::{QueryResult, PgConnection};

use crate::models::product::{Product, NewProduct, UpdateProduct};
use crate::schema::products;

pub fn create_product(
    conn: &mut PgConnection,
    new_product: &NewProduct
) -> QueryResult<Product> {
    diesel::insert_into(products::table)
        .values(new_product)
        .get_result(conn)
}

pub fn get_product_by_id(
    conn: &mut PgConnection, 
    product_id: Uuid
) -> QueryResult<Option<Product>> {
    products::table
        .filter(products::id.eq(product_id))
        .first::<Product>(conn)
        .optional()
}

pub fn update_product(
    conn: &mut PgConnection, 
    product_id: Uuid, 
    updated: &UpdateProduct
) -> QueryResult<Product> {
    diesel::update(products::table.find(product_id))
        .set(updated)
        .get_result(conn)
}

pub fn delete_product(
    conn: &mut PgConnection, 
    product_id: Uuid
) -> QueryResult<usize> {
    diesel::delete(products::table.find(product_id))
        .execute(conn)
}