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

    /// 503 + `Retry-After` — a TRANSIENT, self-clearing contention stopped the
    /// request; nothing is wrong and nothing was written.
    ///
    /// Distinct from [`ApiError::ServiceUnavailable`] (a dependency is DOWN and
    /// the caller must degrade) and emphatically distinct from
    /// [`ApiError::BadRequest`]: the request was perfect. Introduced for the
    /// booking-inventory lock (B8e / L3), where a caller that waits out a
    /// concurrent desk save must be told to retry — folding that into a 400
    /// would turn a millisecond of contention into a permanently dropped
    /// booking on the OTA bridge, which has no human to notice.
    #[error("Busy: {0}")]
    Busy(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

/// `Retry-After` (seconds) on [`ApiError::Busy`]. One second: the contention
/// this signals clears in milliseconds, and the lock itself gives up after
/// five, so anything larger would tell a client to wait longer than the
/// condition can last.
pub const BUSY_RETRY_AFTER_SECONDS: u32 = 1;

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
        // `Busy` is the one variant that carries a header, so it returns early
        // rather than widening the tuple every other arm builds.
        if let ApiError::Busy(msg) = &self {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                [(
                    axum::http::header::RETRY_AFTER,
                    BUSY_RETRY_AFTER_SECONDS.to_string(),
                )],
                Json(json!({"success": false, "error": msg})),
            )
                .into_response();
        }

        let (status, message) = match &self {
            ApiError::Busy(_) => unreachable!("handled above"),
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
