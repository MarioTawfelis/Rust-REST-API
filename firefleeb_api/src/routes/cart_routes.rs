use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

use crate::db::PgPool;
use crate::handlers::dtos::{
    CreateCartItemRequest, CreateCartRequest, UpdateCartItemRequest, UpdateCartRequest,
};
use crate::handlers::{cart_handlers, cart_item_handlers};
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
        .and(base.clone().and(warp::path::param::<Uuid>()))
        .and(with_pool(pool.clone()))
        .and_then(|id, pool| async move {
            cart_handlers::delete(pool, id)
                .await
                .map_err(warp::reject::custom)
        });

    // Common prefix: /carts/:cart_id/items
    let items_base = base
        .and(warp::path::param::<Uuid>())
        .and(warp::path("items"));

    // GET /carts/:cart_id/items
    let list_items = warp::get()
        .and(items_base.clone())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and_then(|cart_id, pool| async move {
            cart_item_handlers::list(pool, cart_id)
                .await
                .map_err(warp::reject::custom)
        });

    // POST /carts/:cart_id/items
    let add_item = warp::post()
        .and(items_base.clone())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<CreateCartItemRequest>())
        .and_then(|cart_id, pool, req| async move {
            cart_item_handlers::create(pool, cart_id, req)
                .await
                .map_err(warp::reject::custom)
        });

    // PUT /carts/:cart_id/items/:item_id
    let update_item = warp::put()
        .and(items_base.clone())
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<UpdateCartItemRequest>())
        .and_then(|cart_id, item_id, pool, req| async move {
            cart_item_handlers::update(pool, cart_id, item_id, req)
                .await
                .map_err(warp::reject::custom)
        });

    // DELETE /carts/:cart_id/items/:item_id
    let delete_item = warp::delete()
        .and(items_base.clone())
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and_then(|cart_id, item_id, pool| async move {
            cart_item_handlers::delete(pool, cart_id, item_id)
                .await
                .map_err(warp::reject::custom)
        });

    // DELETE /carts/:cart_id/items
    let clear_items = warp::delete()
        .and(items_base)
        .and(warp::path::end())
        .and(with_pool(pool))
        .and_then(|cart_id, pool| async move {
            cart_item_handlers::clear(pool, cart_id)
                .await
                .map_err(warp::reject::custom)
        });

    create
        .or(get_one)
        .or(update)
        .or(delete)
        .or(list_items)
        .or(add_item)
        .or(update_item)
        .or(delete_item)
        .or(clear_items)
}
