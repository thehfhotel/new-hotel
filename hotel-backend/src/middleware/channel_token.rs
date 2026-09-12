//! Shared-bearer service-token gate for the loyalty-app booking channel
//! (`/api/channel/*` — see `routes::channel` and `docs/loyalty-channel.md`).
//!
//! Machine-to-machine auth, deliberately NOT the cookie session: the loyalty
//! app presents `Authorization: Bearer <LOYALTY_CHANNEL_TOKEN>` on every
//! request. Structural template: `middleware::hk_access` (header extraction →
//! verify → 401/403), reduced to a shared-secret compare.
//!
//! ## Fail-closed matrix
//!
//! | state | response |
//! |---|---|
//! | `LOYALTY_CHANNEL_ENABLED` off (default) | `503` `reason: channel_disabled` — surface ships DARK |
//! | flag on, `LOYALTY_CHANNEL_TOKEN` unset/empty | `503` `reason: channel_disabled` — misconfigured, fail closed |
//! | header missing / not `Bearer` / mismatch | `401` `reason: unauthorized` |
//! | match | pass through |
//!
//! Both refusals carry the SAME machine `reason` vocabulary the handlers use
//! (`routes::channel::reason`), and that matters most for the 503: the channel
//! also answers `503` `reason: inventory_lock_timeout` for momentary write
//! contention, which a client SHOULD retry (it carries `Retry-After`). A dark
//! channel must not be retried at all. Without the code on both, the two are
//! indistinguishable on the wire — and the dark one is what production returns
//! today.
//!
//! The token compare is constant-time over the byte contents (XOR-fold, no
//! early exit) so a mismatch's response time doesn't leak a prefix-match
//! oracle. Length is compared first — leaking the token *length* is accepted
//! (standard practice absent a hashing step; the token is high-entropy).

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::config::LoyaltyConfig;
use crate::routes::channel::reason;

/// State for [`require_channel_token`]. Built once at startup in `main.rs`
/// from [`LoyaltyConfig::from_env`]; cheap to clone (Arc).
#[derive(Clone)]
pub struct ChannelTokenState {
    enabled: bool,
    token: Option<Arc<str>>,
}

impl ChannelTokenState {
    pub fn new(config: &LoyaltyConfig) -> Self {
        Self {
            enabled: config.channel_enabled,
            token: config.channel_token.as_deref().map(Arc::from),
        }
    }

    /// Test constructor.
    #[cfg(test)]
    fn for_test(enabled: bool, token: Option<&str>) -> Self {
        Self {
            enabled,
            token: token.map(Arc::from),
        }
    }
}

/// Axum `from_fn_with_state` middleware guarding the channel subrouter.
pub async fn require_channel_token(
    State(state): State<ChannelTokenState>,
    request: Request,
    next: Next,
) -> Response {
    match check_channel_access(
        state.enabled,
        state.token.as_deref(),
        request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok()),
    ) {
        access => match refusal(access) {
            None => next.run(request).await,
            Some((status, reason, message)) => (
                status,
                Json(json!({
                    "success": false,
                    "reason": reason,
                    "error": message,
                })),
            )
                .into_response(),
        },
    }
}

/// The wire shape of a refusal: status, machine `reason`, human message.
///
/// Split out as a pure function (rather than inlined into the match above) so
/// the reason codes can be asserted without standing up a router — the same
/// reason [`check_channel_access`] is pure. `None` = not a refusal.
fn refusal(access: ChannelAccess) -> Option<(StatusCode, &'static str, &'static str)> {
    match access {
        ChannelAccess::Allowed => None,
        ChannelAccess::Disabled => Some((
            StatusCode::SERVICE_UNAVAILABLE,
            reason::CHANNEL_DISABLED,
            "loyalty channel is disabled",
        )),
        ChannelAccess::Unauthorized => Some((
            StatusCode::UNAUTHORIZED,
            reason::UNAUTHORIZED,
            "invalid or missing bearer token",
        )),
    }
}

/// Pure decision function — unit-testable without a router.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChannelAccess {
    Allowed,
    /// Flag off OR token unconfigured → the surface does not exist yet (503).
    Disabled,
    /// Bad/missing credentials (401).
    Unauthorized,
}

fn check_channel_access(
    enabled: bool,
    expected_token: Option<&str>,
    authorization_header: Option<&str>,
) -> ChannelAccess {
    // Ship-dark gate first: flag off or no token provisioned ⇒ the surface
    // is unavailable regardless of what the caller sends.
    let Some(expected) = expected_token.filter(|_| enabled) else {
        return ChannelAccess::Disabled;
    };

    let Some(presented) = authorization_header.and_then(extract_bearer) else {
        return ChannelAccess::Unauthorized;
    };

    if constant_time_eq(presented.as_bytes(), expected.as_bytes()) {
        ChannelAccess::Allowed
    } else {
        ChannelAccess::Unauthorized
    }
}

/// Pull the token out of an `Authorization: Bearer <token>` header value.
/// Scheme match is case-insensitive per RFC 7235; the token is trimmed.
fn extract_bearer(header_value: &str) -> Option<&str> {
    let trimmed = header_value.trim();
    let (scheme, rest) = trimmed.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let token = rest.trim();
    (!token.is_empty()).then_some(token)
}

/// Constant-time byte-slice equality (XOR-fold, no early exit on content).
/// Returns false immediately on length mismatch — length is not secret here.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str = "test-loyalty-channel-token";

    #[test]
    fn disabled_flag_wins_over_valid_token() {
        assert_eq!(
            check_channel_access(false, Some(TOKEN), Some(&format!("Bearer {TOKEN}"))),
            ChannelAccess::Disabled,
            "surface must stay dark while LOYALTY_CHANNEL_ENABLED is off"
        );
    }

    #[test]
    fn enabled_without_token_fails_closed() {
        assert_eq!(
            check_channel_access(true, None, Some(&format!("Bearer {TOKEN}"))),
            ChannelAccess::Disabled,
            "flag on but no LOYALTY_CHANNEL_TOKEN provisioned must NOT accept anything"
        );
    }

    #[test]
    fn missing_header_is_unauthorized() {
        assert_eq!(
            check_channel_access(true, Some(TOKEN), None),
            ChannelAccess::Unauthorized
        );
    }

    #[test]
    fn wrong_token_is_unauthorized() {
        assert_eq!(
            check_channel_access(true, Some(TOKEN), Some("Bearer wrong-token")),
            ChannelAccess::Unauthorized
        );
    }

    #[test]
    fn wrong_scheme_is_unauthorized() {
        assert_eq!(
            check_channel_access(true, Some(TOKEN), Some(&format!("Basic {TOKEN}"))),
            ChannelAccess::Unauthorized
        );
    }

    #[test]
    fn matching_bearer_is_allowed_case_insensitive_scheme() {
        for scheme in ["Bearer", "bearer", "BEARER"] {
            assert_eq!(
                check_channel_access(true, Some(TOKEN), Some(&format!("{scheme} {TOKEN}"))),
                ChannelAccess::Allowed,
                "scheme {scheme} should be accepted"
            );
        }
    }

    #[test]
    fn extract_bearer_trims_and_rejects_empty() {
        assert_eq!(extract_bearer("Bearer  abc "), Some("abc"));
        assert_eq!(extract_bearer("Bearer "), None);
        assert_eq!(extract_bearer("abc"), None);
    }

    /// Every response this middleware can emit carries a machine `reason`,
    /// and the two it emits are the ones the handlers CANNOT emit — so the
    /// "every `/api/channel/*` error body carries a reason" claim is only true
    /// end to end if these two do too.
    ///
    /// The 503 assertion is the load-bearing one: `channel_disabled` must not
    /// equal `inventory_lock_timeout` (retryable, `Retry-After`) nor collapse
    /// onto the generic `for_status(503)` fallback, or a client cannot tell
    /// "the channel is off, stop" from "try again in a second".
    #[test]
    fn every_refusal_carries_a_reason_distinct_from_the_retryable_503() {
        assert_eq!(
            refusal(ChannelAccess::Allowed),
            None,
            "a pass-through is not a refusal"
        );

        let (disabled_status, disabled_reason, disabled_msg) =
            refusal(ChannelAccess::Disabled).expect("Disabled is a refusal");
        assert_eq!(disabled_status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(disabled_reason, reason::CHANNEL_DISABLED);
        assert!(!disabled_msg.is_empty());

        let (unauth_status, unauth_reason, unauth_msg) =
            refusal(ChannelAccess::Unauthorized).expect("Unauthorized is a refusal");
        assert_eq!(unauth_status, StatusCode::UNAUTHORIZED);
        assert_eq!(unauth_reason, reason::UNAUTHORIZED);
        assert!(!unauth_msg.is_empty());

        assert_ne!(
            disabled_reason,
            reason::INVENTORY_LOCK_TIMEOUT,
            "the dark-channel 503 must stay distinguishable from the retryable one"
        );
        assert_ne!(
            disabled_reason,
            reason::for_status(503),
            "it must be an explicit code, not the generic 503 fallback"
        );
    }

    /// The production-today shape, end to end: flag off, no token ⇒ every
    /// caller gets `503 channel_disabled`, never the retryable 503 and never
    /// a 401.
    #[test]
    fn the_dark_default_answers_channel_disabled_for_every_caller() {
        let state = ChannelTokenState::new(&LoyaltyConfig::default());
        for header in [
            None,
            Some(format!("Bearer {TOKEN}")),
            Some("Bearer x".to_string()),
        ] {
            let access =
                check_channel_access(state.enabled, state.token.as_deref(), header.as_deref());
            let (status, code, _) = refusal(access).expect("a dark channel refuses everything");
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(code, reason::CHANNEL_DISABLED);
        }
    }

    #[test]
    fn constant_time_eq_semantics() {
        assert!(constant_time_eq(b"same", b"same"));
        assert!(!constant_time_eq(b"same", b"sam1"));
        assert!(!constant_time_eq(b"short", b"longer"));
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn state_builder_reads_config() {
        let cfg = LoyaltyConfig {
            channel_enabled: true,
            channel_token: Some(TOKEN.to_string()),
            app_url: None,
            service_token: None,
        };
        let state = ChannelTokenState::new(&cfg);
        assert!(state.enabled);
        assert_eq!(state.token.as_deref(), Some(TOKEN));
        // for_test parity so router tests can construct states directly.
        let t = ChannelTokenState::for_test(true, Some(TOKEN));
        assert_eq!(t.token.as_deref(), Some(TOKEN));
    }

    /// The shape `LoyaltyConfig::from_env()` produces in production today, and
    /// the shape the B3 dark declaration keeps producing: flag off (unset or
    /// blank `LOYALTY_CHANNEL_ENABLED`) and no token (unset or empty
    /// `/run/secrets/loyalty_channel_token`). Every request — credentialled or
    /// not — must land on `Disabled` (503), never `Unauthorized` (401), which
    /// is what makes "503-by-flag, not 401" a meaningful go-live check.
    ///
    /// `config::flag_enabled` / `optional_env` are what map unset-or-blank onto
    /// these `false` / `None` values; that mapping is pinned by
    /// `config::tests::loyalty_channel_stays_dark_when_the_flag_is_unset_or_blank`.
    #[test]
    fn default_config_keeps_the_surface_dark_for_every_caller() {
        let state = ChannelTokenState::new(&LoyaltyConfig::default());
        assert!(!state.enabled);
        assert!(state.token.is_none());

        for header in [
            None,
            Some(format!("Bearer {TOKEN}")),
            Some("Bearer wrong-token".to_string()),
            Some(String::new()),
        ] {
            assert_eq!(
                check_channel_access(state.enabled, state.token.as_deref(), header.as_deref()),
                ChannelAccess::Disabled,
                "a dark channel must answer 503 for header {header:?}, never 401"
            );
        }
    }
}
