use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

use crate::db::PgPool;
use crate::handlers::dtos::{CreateCartRequest, UpdateCartRequest};
use crate::handlers::cart_handlers;
use crate::routes::{json_body, with_pool};

pub fn cart_routes(
    pool: PgPool,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let base = warp::path("carts");

    // POST /carts
    let create = warp::post()
        .and(base.clone())
        .and(with_pool(pool.clone()))
        .and(json_body::<CreateCartRequest>())
        .and_then(|pool, req| async move {
            cart_handlers::create(pool, req)
                .await
                .map_err(warp::reject::custom)
        });

    // PUT /carts/:id
    let update = warp::put()
        .and(base.clone())
        .and(warp::path::param::<Uuid>())
        .and(with_pool(pool.clone()))
        .and(json_body::<UpdateCartRequest>())
        .and_then(|id, pool, req| async move {
            cart_handlers::update(pool, id, req)
                .await
                .map_err(warp::reject::custom)
        });

    // GET /carts/:id
    let get_one = warp::get()
        .and(base.clone().and(warp::path::param::<Uuid>()))
        .and(with_pool(pool.clone()))
        .and_then(|id, pool| async move {
            cart_handlers::get(pool, id)
                .await
                .map_err(warp::reject::custom)
        });

    // DELETE /carts/:id
    let delete = warp::delete()
        .and(base.and(warp::path::param::<Uuid>()))
        .and(with_pool(pool))
        .and_then(|id, pool| async move {
            cart_handlers::delete(pool, id)
                .await
                .map_err(warp::reject::custom)
        });


    create.or(get_one).or(update).or(delete)
}
