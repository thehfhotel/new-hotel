//! Error types for the Hotel Backend API
//!
//! Follows the thiserror pattern from the Tauri middleware.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// API errors that can occur during request handling
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 409 — the request is valid but the server-side state / configuration
    /// refuses it (e.g. a ship-dark write flag is off). The message should
    /// LEAD with a stable machine-checkable code (`SCREAMING_SNAKE`) so
    /// clients can branch on it without a separate `code` field.
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 503 — the request is well-formed and permitted, but a DEPENDENCY this
    /// handler needs could not be reached, so no authoritative answer exists
    /// right now. Distinct from [`ApiError::Forbidden`] on purpose: 403 means
    /// "the answer is no", 503 means "there is no answer yet, retrying may
    /// help". Collapsing the two would let a client cache a refusal that was
    /// really an outage — or, worse, invite a fallback. The message is
    /// user-facing (`/hk` renders it to a maid in Thai).
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl From<tiberius::error::Error> for ApiError {
    fn from(err: tiberius::error::Error) -> Self {
        ApiError::Database(err.to_string())
    }
}

impl From<bb8::RunError<bb8_tiberius::Error>> for ApiError {
    fn from(err: bb8::RunError<bb8_tiberius::Error>) -> Self {
        ApiError::Database(err.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Database(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            // The message is preserved (not swallowed like Internal/Database):
            // it is the actionable text the caller must show, and it names no
            // internal detail — the dependency's own error stays in the logs.
            ApiError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            ApiError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            ApiError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({"success": false, "error": message}))).into_response()
    }
}

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;
