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

    /// 409 carrying a **machine `reason`** and, when the refusal points at an
    /// existing row, that row's id.
    ///
    /// Same status as [`ApiError::Conflict`], different contract. `Conflict`
    /// asks callers to parse a `SCREAMING_SNAKE` prefix out of a
    /// human-readable message; this variant puts the code in its own field,
    /// which is the convention the newer surfaces already use — `/api/channel/*`
    /// (`routes::channel::reason`) and this router's own
    /// [`ApiError::Busy`] body. New refusals should prefer it.
    ///
    /// The id matters as much as the code: a refusal that says only "already
    /// done" leaves the desk hunting for the row that did it. `conflicting_id`
    /// is rendered as `conflictingId` — for
    /// [`BOOKING_ALREADY_CHECKED_IN_REASON`] it is the open check-in's
    /// `ht_checkins.cin_id`, so the UI can offer that folio instead of a dead
    /// end.
    #[error("Conflict ({reason}): {message}")]
    ConflictWithReason {
        reason: &'static str,
        message: String,
        conflicting_id: Option<i32>,
    },

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

/// Machine `reason` carried by every [`ApiError::Busy`] body, on BOTH the
/// desk/OTA router and `/api/channel/*` (which re-exports it as
/// `routes::channel::reason::INVENTORY_LOCK_TIMEOUT`). Declared here, not
/// there, so the low-level error type does not have to reach up into a route
/// module for its own body — and so the two surfaces cannot drift into
/// describing one condition two ways.
///
/// It covers BOTH shapes of `InventoryLockError::Busy`: another writer held
/// the lock (`LockHeld`) and the connection pool had nothing to lend
/// (`PoolExhausted`). They are one condition to a caller — transient, nothing
/// written, retry the identical request — and splitting them would hand
/// loyalty-app a distinction it cannot act on differently.
pub const BUSY_REASON: &str = "inventory_lock_timeout";

/// Machine `reason` for the booking-level double-check-in refusal (B7b).
///
/// `POST /api/new/checkins` with a `booking_id` whose booking already has as
/// many OPEN check-ins as it has assigned rooms. Emitted as
/// [`ApiError::ConflictWithReason`] → **409**, with `conflictingId` set to the
/// open check-in's `cin_id`.
///
/// Declared here beside [`BUSY_REASON`] for the same reason: the code is a
/// wire contract, and a contract that lives inside the one handler that emits
/// it drifts the moment a second surface needs it. **Renaming it is a contract
/// change, not a refactor.**
pub const BOOKING_ALREADY_CHECKED_IN_REASON: &str = "booking_already_checked_in";

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
                // `reason` alongside the message so the desk/OTA body mirrors
                // the channel router's — one condition, one machine code, two
                // surfaces (B8e round-2 review).
                Json(json!({"success": false, "reason": BUSY_REASON, "error": msg})),
            )
                .into_response();
        }

        // Same early-return reason as `Busy`: this variant renders extra body
        // fields (`reason`, `conflictingId`) rather than the uniform
        // `{success, error}` shape every arm below shares.
        if let ApiError::ConflictWithReason {
            reason,
            message,
            conflicting_id,
        } = &self
        {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "success": false,
                    "reason": reason,
                    "error": message,
                    "conflictingId": conflicting_id,
                })),
            )
                .into_response();
        }

        let (status, message) = match &self {
            ApiError::Busy(_) | ApiError::ConflictWithReason { .. } => {
                unreachable!("handled above")
            }
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
