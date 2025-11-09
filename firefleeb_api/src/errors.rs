use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::convert::Infallible;
use std::fmt;
use warp::Reply;
use warp::http::StatusCode;
use warp::reply::{Response, json, with_status};

#[derive(Debug, Clone)]
pub enum AppError {
    Validation(String),
    Unauthorized(String),
    Conflict(String),
    NotFound(String),
    Db(String),
    Internal(String),
}

// Map Diesel errors into our AppError type consistently.
pub fn map_diesel_error(err: DieselError) -> AppError {
    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            AppError::Conflict("Unique constraint violation".into())
        }
        other => AppError::Db(format!("Database error: {other}")),
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Validation(msg)
            | AppError::Unauthorized(msg)
            | AppError::Conflict(msg)
            | AppError::NotFound(msg)
            | AppError::Db(msg)
            | AppError::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl warp::reject::Reject for AppError {}

/// Convert AppError → HTTP JSON reply for Warp
impl Reply for AppError {
    fn into_response(self) -> Response {
        let code = match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Db(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = serde_json::json!({
            "error": self.to_string(),
            "status": code.as_u16()
        });
        with_status(json(&body), code).into_response()
    }
}

/// Convenience result alias used in handlers/services
pub type AppResult<T> = Result<T, AppError>;

/// Map Diesel errors → AppError::Db
impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        AppError::Db(e.to_string())
    }
}

/// Map R2D2 errors → AppError::Db
impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        AppError::Db(e.to_string())
    }
}

/// Map Infallible (used in Warp filters) → AppError::Internal
impl From<Infallible> for AppError {
    fn from(_: Infallible) -> Self {
        AppError::Internal("infallible error".into())
    }
}
