//! Tower-style request middleware (Phase 4 PR2).
//!
//! Each submodule exports an `async fn` suitable for
//! `axum::middleware::from_fn_with_state`. The auth middleware is the
//! only inhabitant today; future per-route concerns (rate limit,
//! request id, etc.) belong here too.

pub mod auth;

pub use auth::require_auth;
