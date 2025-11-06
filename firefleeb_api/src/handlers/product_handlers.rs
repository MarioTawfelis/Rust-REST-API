use crate::handlers::dtos::{CreateProductRequest, UpdateProductRequest, ProductResponse};
use crate::models::product::{NewProduct, UpdateProduct};
use crate::services::product_service;
use crate::errors::AppError;
use crate::db::PgPool;
use warp::{Reply, reply};
use uuid::Uuid;

pub async fn create(
    pool: PgPool,
    req: CreateProductRequest
) -> Result<impl Reply, AppError> {
    let new_product = NewProduct {
        product_name: req.product_name,
        product_description: req.product_description,
        price: req.price,
        stock: req.stock,
    };

    let product = product_service::create_product(pool, new_product).await?;
    Ok(reply::json(&ProductResponse::from(product)))
}

pub async fn update(
    pool: PgPool,
    product_id: Uuid,
    req: UpdateProductRequest
) -> Result<impl Reply, AppError> {
    let updated_product = UpdateProduct {
        product_name: req.product_name,
        product_description: req.product_description,
        price: req.price,
        stock: req.stock,
    };

    let product = product_service::update_product(pool, product_id, updated_product).await?;
    Ok(reply::json(&ProductResponse::from(product)))
}

pub async fn get(pool: PgPool, id: Uuid) -> Result<impl Reply, AppError> {
    let product = product_service::get_product_by_id(pool, id).await?;
    Ok(warp::reply::json(&ProductResponse::from(product)))
}

pub async fn delete(pool: PgPool, id: Uuid) -> Result<impl Reply, AppError> {
    product_service::delete_product(pool, id).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({"message": "deleted"})),
        warp::http::StatusCode::NO_CONTENT,
    ))
}