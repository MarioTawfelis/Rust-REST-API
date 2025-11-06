use std::convert::Infallible;
use warp::{Reply, Rejection};
use crate::errors::AppError;
use warp::http::StatusCode;

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let resp = if let Some(app) = err.find::<AppError>() {
        app.clone().into_response() 
    } else {
        let body = serde_json::json!({
            "error": "internal server error",
            "status": 500
        });
        warp::reply::with_status(
            warp::reply::json(&body),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response() 
    };

    Ok(resp) 
}
