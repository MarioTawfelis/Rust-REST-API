use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

use crate::db::PgPool;
use crate::handlers::dtos::{
    CreateUserRequest, LoginRequest, UpdatePasswordRequest, UpdateUserRequest,
};
use crate::handlers::user_handlers;
use crate::routes::{json_body, with_pool};

pub fn user_routes(
    pool: PgPool,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let base = warp::path("users");

    // POST /users
    let create = warp::post()
        .and(base.clone())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<CreateUserRequest>())
        .and_then(|pool, req| async move {
            user_handlers::register(pool, req)
                .await
                .map_err(warp::reject::custom)
        });

    // PUT /users/:id
    let update = warp::put()
        .and(base.clone())
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<UpdateUserRequest>())
        .and_then(|id, pool, req| async move {
            user_handlers::update_user(pool, id, req)
                .await
                .map_err(warp::reject::custom)
        });

    // PUT /users/:id/password
    let update_password = warp::put()
        .and(base.clone())
        .and(warp::path("password-reset"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<UpdatePasswordRequest>())
        .and_then(|id, pool, req| async move {
            user_handlers::change_password(pool, id, req)
                .await
                .map_err(warp::reject::custom)
        });

    // POST /users/login
    let login = warp::post()
        .and(base.clone())
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and(json_body::<LoginRequest>())
        .and_then(|pool, req| async move {
            user_handlers::login(pool, req)
                .await
                .map_err(warp::reject::custom)
        });

    create.or(update).or(update_password).or(login)
}
