//! `ServiceError` — the unified failure type for the service layer.
//!
//! Per `docs/architecture.md` §1, §2 the service layer sits between thin HTTP
//! routes and the data-access repositories. It needs to surface three families
//! of failure:
//!
//! 1. **Validation** — the inbound command is malformed (missing required field,
//!    invalid date range, unknown enum value). The HTTP layer maps this to
//!    `400 Bad Request`.
//! 2. **Not found** — an aggregate the command refers to does not exist. The
//!    HTTP layer maps this to `404 Not Found`.
//! 3. **Conflict** — preconditions for the operation are not met (booking
//!    already cancelled, room already occupied, payment already voided). The
//!    HTTP layer maps this to `409 Conflict`.
//! 4. **Infrastructure** — sqlx transaction failed, outbox enqueue collided on
//!    its idempotency key, event publish failed. The HTTP layer maps this to
//!    `500 Internal Server Error`.
//!
//! `ServiceError` does NOT depend on `axum` or HTTP types — the route layer
//! converts via `From<ServiceError> for ApiError` (added when Wave 4 thins the
//! routes; for now `From<sqlx::Error>` keeps services callable from existing
//! `ApiResult`-typed code paths).

use thiserror::Error;

use crate::error::ApiError;

/// Errors emitted by service-layer operations.
///
/// Variants are grouped by HTTP semantics so the route layer can map them
/// without inspecting the message string.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Inbound command fails domain invariants (e.g. `stay_end <= stay_start`).
    #[error("validation error: {0}")]
    Validation(String),

    /// Referenced aggregate does not exist (e.g. update a missing booking).
    #[error("not found: {0}")]
    NotFound(String),

    /// The CALLER's role forbids this operation on this aggregate — not a
    /// malformed command and not a state precondition, but "this identity may
    /// not do that at all". The HTTP layer maps it to `403 Forbidden`.
    ///
    /// Added for the room signals of ADR 0008
    /// ([`crate::service::hk_signals`]), where the rule "nobody acts on their
    /// own direction's signals except cancel-own-while-open" can only be
    /// evaluated once the signal's direction has been READ — i.e. inside the
    /// service transaction, not at the route gate the way
    /// `routes::hk::require_report_capability` works. Without this variant the
    /// refusal would have to collapse into [`Self::Validation`] and reach a
    /// maid as a 400, indistinguishable from a typo in her request.
    #[error("forbidden: {0}")]
    Forbidden(String),

    /// Preconditions not met for the requested transition (e.g. cancel an
    /// already-cancelled booking, void an already-voided payment).
    #[error("conflict: {0}")]
    Conflict(String),

    /// Underlying repository / database failure.
    #[error("repository error: {0}")]
    Repository(#[from] sqlx::Error),

    /// Outbox enqueue or event publish failed.
    ///
    /// Distinct from [`Self::Repository`] so observability can alarm
    /// specifically on outbox / event-bus failures (those typically indicate
    /// migration drift, not a bad query).
    #[error("outbox error: {0}")]
    Outbox(String),

    /// Catch-all for unexpected failures (`serde_json` encode, etc.).
    #[error("internal error: {0}")]
    Internal(String),
}

impl ServiceError {
    /// Construct a validation error from any `Display`-able message.
    pub fn validation(msg: impl Into<String>) -> Self {
        ServiceError::Validation(msg.into())
    }

    /// Construct a not-found error from any `Display`-able message.
    pub fn not_found(msg: impl Into<String>) -> Self {
        ServiceError::NotFound(msg.into())
    }

    /// Construct a role-refusal error from any `Display`-able message.
    pub fn forbidden(msg: impl Into<String>) -> Self {
        ServiceError::Forbidden(msg.into())
    }

    /// Construct a conflict error from any `Display`-able message.
    pub fn conflict(msg: impl Into<String>) -> Self {
        ServiceError::Conflict(msg.into())
    }

    /// Construct an outbox error from any `Display`-able message.
    pub fn outbox(msg: impl Into<String>) -> Self {
        ServiceError::Outbox(msg.into())
    }

    /// Map an outbox-enqueue failure to the semantically correct variant.
    ///
    /// A unique violation on `writeback_jobs.idempotency_key` means an
    /// identical intent for the same aggregate is ALREADY queued — i.e. a
    /// concurrent caller won the race. That is a precondition conflict
    /// (409), not infrastructure failure: the idempotency key is the
    /// designed last-line guard for exactly this interleaving (two
    /// concurrent transitions both passing the pre-TX status gate; the
    /// loser's TX rolls back here). Surfacing it as `Conflict` keeps the
    /// loser's contract identical whether the status-guard UPDATE or the
    /// idempotency key catches it. Everything else stays `Outbox` so
    /// observability can alarm on real enqueue failures (migration drift).
    pub fn from_enqueue_error(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db) = err {
            if db.constraint() == Some("writeback_jobs_idempotency_key_key") {
                return ServiceError::Conflict(format!(
                    "identical writeback intent already enqueued (concurrent duplicate): {db}"
                ));
            }
        }
        ServiceError::Outbox(err.to_string())
    }

    /// Construct an internal error from any `Display`-able message.
    pub fn internal(msg: impl Into<String>) -> Self {
        ServiceError::Internal(msg.into())
    }
}

/// Bridge to the existing HTTP error type so today's `ApiResult`-typed routes
/// can call services without a separate translation layer.
///
/// Wave 4 will replace this with a tighter mapping (404 vs 409 vs 400) when
/// routes are thinned to delegate exclusively through services.
impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Validation(msg) => ApiError::BadRequest(msg),
            ServiceError::NotFound(msg) => ApiError::NotFound(msg),
            ServiceError::Forbidden(msg) => ApiError::Forbidden(msg),
            // NOTE: 400, not 409. Kept as-is for the room-signal work of ADR
            // 0008 rather than "corrected": every existing service (booking
            // cancel, payment void, the `writeback_jobs` idempotency race)
            // already answers a precondition failure this way, and `ApiError::
            // Conflict` is reserved in this repo for a ship-dark FLAG refusing
            // a request (its doc comment says so). Moving the whole mapping to
            // 409 for one new feature would change five shipped surfaces'
            // status codes as a side effect. `service::hk_signals` documents
            // the choice at its own boundary.
            ServiceError::Conflict(msg) => ApiError::BadRequest(msg),
            ServiceError::Repository(err) => ApiError::Database(err.to_string()),
            ServiceError::Outbox(msg) => ApiError::Internal(format!("outbox: {msg}")),
            ServiceError::Internal(msg) => ApiError::Internal(msg),
        }
    }
}

/// Result alias used across the service layer.
pub type ServiceResult<T> = Result<T, ServiceError>;
