//! Tower-style request middleware (Phase 4 PR2).
//!
//! Each submodule exports an `async fn` suitable for
//! `axum::middleware::from_fn_with_state`. Today's inhabitants:
//!
//! * `auth` — cookie-session gate for `/api/new/*` (Phase 4 PR2).
//! * `rate_limit` — per-IP throttle on `POST /api/auth/login`
//!   (Phase 7 audit M-2).
//! * `permissions` — role/permission grid layered on top of `auth`
//!   for Track G features (refunds, room change, round-bill, etc.).
//! * `cf_access` — Cloudflare Access JWT verification (JWKS + RS256)
//!   backing the `POST /api/auth/cf-login` auto-login route. Not a
//!   tower layer — a verification helper the route calls.

pub mod auth;
pub mod cf_access;
pub mod permissions;
pub mod rate_limit;

pub use auth::require_auth;
pub use permissions::{permissions_for_user, require_permission};
pub use rate_limit::{login_rate_limit, LoginRateLimitState};
