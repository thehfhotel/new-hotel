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
//! * `hfid_assertion` — HF-ID card-tap assertion verification (a SECOND
//!   JWKS + RS256 source, same pattern as `cf_access`) backing the
//!   central-pairing card login in `GET /api/reader/wait`.
//! * `hk_access` — Cloudflare Access gate for the maid-facing `/api/hk/*`
//!   surface (a THIRD consumer of the team JWKS, different Access app AUD;
//!   employee-login plan Phase 4). IS a tower layer, unlike `cf_access`.
//! * `ville_guard` — HF Ville write admission gate (ADR 0002 / Ship-A):
//!   403s `?branch=hfville` mutations while `HFVILLE_WRITES_ENABLED` is off,
//!   with one narrow housekeeping exemption. Lives here (not `main.rs`) so
//!   the decision is unit-testable.
//! * `channel_token` — shared-bearer service-token gate for the loyalty-app
//!   booking channel `/api/channel/*` (machine-to-machine; ships dark behind
//!   `LOYALTY_CHANNEL_ENABLED`). IS a tower layer.

pub mod auth;
pub mod cf_access;
pub mod channel_token;
pub mod hfid_assertion;
pub mod hk_access;
pub mod permissions;
pub mod rate_limit;
pub mod ville_guard;

pub use auth::require_auth;
pub use channel_token::{require_channel_token, ChannelTokenState};
pub use permissions::{permissions_for_user, require_permission};
pub use rate_limit::{login_rate_limit, LoginRateLimitState};
pub use ville_guard::{is_ville_exempt_path, ville_write_blocked, ville_write_guard};
