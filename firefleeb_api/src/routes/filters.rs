use std::convert::Infallible;
use crate::db::PgPool;
use warp::{Filter, Rejection};


pub fn json_body<T: serde::de::DeserializeOwned + Send>() 
    -> impl Filter<Extract = (T,), Error = Rejection> + Clone 
{
    warp::body::content_length_limit(16 * 1024)
        .and(warp::body::json())
}

pub fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}