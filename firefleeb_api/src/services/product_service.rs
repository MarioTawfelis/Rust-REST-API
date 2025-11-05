use diesel::result::{DatabaseErroKind, Error as DieseError};

use uuid::Uuid;

use crate::db::product_repository;
use crate::db::{with_conn, PgPool};

use crate::errors::AppError;

use crate::models::product::{NewProduct, UpdateProduct, Product};

pub async fn create_product(
    pool: PgPool,
    new_product: NewProduct
) -> Result<Product, AppError> {
    with_conn(pool, |conn| {
        product_repository::create_product(conn, &new_product)
    })
    .await
    .map_err(map_diesel_error)
}

pub async fn get_product_by_id(
    pool: PgPool,
    product_id: Uuid,
) -> Result<Product, AppError> {
    let maybe_product = with_conn(pool, |conn| {
        product_repository::get_product_by_id(conn, product_id)
    })
    .await
    .map_err(map_diesel_error)?;

    maybe_product.ok_or_else(|| AppError::NotFound("Product not found".into()))
}

pub async fn update_product(
    pool: PgPool,
    product_id: Uuid,
    updated: UpdateProduct
) -> Result<Product, AppError> {
    with_conn(pool, |conn| {
        product_repository::update_product(conn, product_id, &updated)
    })
    .await
    .map_err(map_diesel_error)
}

pub async fn delete_product(
    pool: PgPool,
    product_id: Uuid,
) -> Result<(), AppError> {
    with_conn(pool, |conn| {
        product_repository::delete_product(conn, product_id)
    })
    .await
    .map_err(map_diesel_error)
    .and_then(|rows_deleted| {
        if rows_deleted == 0 {
            Err(AppError::NotFound("Product not found".into()))
        } else {
            Ok(())
        }
    })
}

