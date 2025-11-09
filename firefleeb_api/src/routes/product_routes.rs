use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

use crate::db::PgPool;
use crate::handlers::dtos::{CreateProductRequest, UpdateProductRequest};
use crate::handlers::product_handlers;
use crate::routes::{json_body, with_pool};

pub fn product_routes(
    pool: PgPool,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // POST /products
    let create = warp::post()
        .and(warp::path("products"))
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<CreateProductRequest>())
        .and_then(|pool, req| async move {
            product_handlers::create(pool, req)
                .await
                .map_err(warp::reject::custom)
        });

    // PUT /products/:id
    let update = warp::put()
        .and(warp::path("products"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<UpdateProductRequest>())
        .and_then(|id, pool, req| async move {
            product_handlers::update(pool, id, req)
                .await
                .map_err(warp::reject::custom)
        });

    // GET /products/:id
    let get_one = warp::get()
        .and(
            warp::path("products")
                .and(warp::path::param::<Uuid>())
                .and(warp::path::end()),
        )
        .and(with_pool(pool.clone()))
        .and_then(|id, pool| async move {
            product_handlers::get(pool, id)
                .await
                .map_err(warp::reject::custom)
        });

    // DELETE /products/:id
    let delete = warp::delete()
        .and(
            warp::path("products")
                .and(warp::path::param::<Uuid>())
                .and(warp::path::end()),
        )
        .and(with_pool(pool))
        .and_then(|id, pool| async move {
            product_handlers::delete(pool, id)
                .await
                .map_err(warp::reject::custom)
        });

    create.or(get_one).or(update).or(delete)
}
