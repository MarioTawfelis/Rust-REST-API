use crate::errors::AppError;
use std::convert::Infallible;
use warp::{Rejection, Reply, filters::body::BodyDeserializeError, http::StatusCode};

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let resp = if let Some(app) = err.find::<AppError>() {
        app.clone().into_response()
    } else if let Some(body_err) = err.find::<BodyDeserializeError>() {
        let body = serde_json::json!({
            "error": format!("invalid request body: {body_err}"),
            "status": 400
        });
        warp::reply::with_status(warp::reply::json(&body), StatusCode::BAD_REQUEST).into_response()
    } else {
        let body = serde_json::json!({
            "error": "internal server error",
            "status": 500
        });
        warp::reply::with_status(warp::reply::json(&body), StatusCode::INTERNAL_SERVER_ERROR)
            .into_response()
    };

    Ok(resp)
}
