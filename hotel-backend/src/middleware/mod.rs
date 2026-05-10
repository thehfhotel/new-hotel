//! Tower-style request middleware (Phase 4 PR2).
//!
//! Each submodule exports an `async fn` suitable for
//! `axum::middleware::from_fn_with_state`. Today's inhabitants:
//!
//! * `auth` — cookie-session gate for `/api/new/*` (Phase 4 PR2).
//! * `rate_limit` — per-IP throttle on `POST /api/auth/login`
//!   (Phase 7 audit M-2).

pub mod auth;
pub mod rate_limit;

pub use auth::require_auth;
pub use rate_limit::{login_rate_limit, LoginRateLimitState};
