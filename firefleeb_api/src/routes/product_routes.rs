use std::convert::Infallible;

use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

use crate::db::PgPool;
use crate::handlers::dtos::{CreateProductRequest, UpdateProductRequest};
use crate::handlers::product_handlers;

pub fn product_routes(
    pool: PgPool,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let base = warp::path("products");

    // POST /products
    let create = warp::post()
        .and(base.clone())
        .and(with_pool(pool.clone()))
        .and(json_body::<CreateProductRequest>())
        .and_then(|pool, req| async move {
            product_handlers::create(pool, req)
                .await
                .map_err(warp::reject::custom)
        });

    // PUT /products/:id
    let update = warp::put()
        .and(base.clone())
        .and(warp::path::param::<Uuid>())
        .and(with_pool(pool.clone()))
            .and(json_body::<UpdateProductRequest>())
            .and_then(|id, pool, req| async move {
                product_handlers::update(pool, id, req)
                    .await
                    .map_err(warp::reject::custom)
            });

    // GET /products/:id
    let get_one = warp::get()
        .and(base.clone().and(warp::path::param::<Uuid>()))
        .and(with_pool(pool.clone()))
        .and_then(|id, pool| async move {
            product_handlers::get(pool, id)
                .await
                .map_err(warp::reject::custom)
        });


    // DELETE /products/:id
    let delete = warp::delete()
        .and(base.and(warp::path::param::<Uuid>()))
        .and(with_pool(pool))
        .and_then(|id, pool| async move {
            product_handlers::delete(pool, id)
                .await
                .map_err(warp::reject::custom)
        });

    create.or(get_one).or(update).or(delete)
}

fn json_body<T: serde::de::DeserializeOwned + Send>() 
    -> impl Filter<Extract = (T,), Error = Rejection> + Clone 
{
    warp::body::content_length_limit(16 * 1024)
        .and(warp::body::json())
}

fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
