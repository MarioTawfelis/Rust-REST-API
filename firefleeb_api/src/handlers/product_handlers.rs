use crate::handlers::dtos::{CreateProductRequest, UpdateProductRequest, ProductResponse};
use crate::models::product::{NewProduct, UpdateProduct};
use crate::services::product_service;
use crate::errors::AppError;
use crate::db::PgPool;
use warp::Reply;

pub async fn create_product_handler(
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
    Ok(reply::json(ProductResposnse::from(product)))
}

pub async fn update_product_handler(
    pool: PgPool,
    product_id: uuid::Uuid,
    req: UpdateProductRequest
) -> Result<impl Reply, AppError> {
    let updated_product = UpdateProduct {
        product_name: req.product_name,
        product_description: req.product_description,
        price: req.price,
        stock: req.stock,
    };

    let product = product_service::update_product(pool, product_id, updated_product).await?;
    Ok(reply::json(ProductResponse::from(product)))
}