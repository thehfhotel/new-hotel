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
//! | `LOYALTY_CHANNEL_ENABLED` off (default) | `503` — surface ships DARK |
//! | flag on, `LOYALTY_CHANNEL_TOKEN` unset/empty | `503` — misconfigured, fail closed |
//! | header missing / not `Bearer` / mismatch | `401` |
//! | match | pass through |
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
        ChannelAccess::Allowed => next.run(request).await,
        ChannelAccess::Disabled => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "success": false,
                "error": "loyalty channel is disabled",
            })),
        )
            .into_response(),
        ChannelAccess::Unauthorized => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": "invalid or missing bearer token",
            })),
        )
            .into_response(),
    }
}

/// Pure decision function — unit-testable without a router.
#[derive(Debug, PartialEq, Eq)]
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
}
