//! Machine-auth gate for the OTA booking bridge (`/api/ota/*` — see
//! `routes::ota` and `docs/ota-bridge.md`).
//!
//! Structural clone of [`crate::middleware::channel_token`] with one added
//! axis: a **permissive-then-enforce** mode, because unlike the loyalty
//! channel this surface replaces an *already-live* unauthenticated call path
//! (`ota-desk/lib/pms.ts` reaches `/api/*` over the `hotel-network` bridge
//! carrying no credential at all). The gate therefore has to be able to accept
//! the un-credentialed caller for exactly as long as it takes the other repo to
//! start sending a bearer.
//!
//! `channel_token.rs` is owned by a shipped feature; `extract_bearer` /
//! `constant_time_eq` are COPIED here verbatim rather than refactored out of
//! it, deliberately, so a change to this bridge can never move the loyalty
//! channel's auth.
//!
//! ## Mode matrix (the whole contract)
//!
//! | `enabled` | `enforce` | accepted tokens | `Authorization` | outcome | HTTP |
//! |---|---|---|---|---|---|
//! | false | * | * | * | `Disabled` | **503** |
//! | true | true | empty | * | `Disabled` | **503** (misconfigured → fail closed) |
//! | true | true | non-empty | absent / non-Bearer | `Unauthorized` | **401** |
//! | true | * | any | Bearer, no match | `Unauthorized` | **401** |
//! | true | false | any | absent / non-Bearer | `AllowedUnauthenticated` | **200** + WARN |
//! | true | * | non-empty | matches primary | `Allowed` | **200** |
//! | true | * | non-empty | matches previous | `AllowedRotating` | **200** + WARN |
//!
//! **The load-bearing invariant: a *presented but wrong* credential is 401 in
//! every mode.** Only a completely ABSENT `Authorization` header rides the
//! permissive lane. That is what makes flipping `OTA_BRIDGE_ENFORCE` provably
//! safe — once the "accepted WITHOUT a bearer token" WARN count is zero,
//! enforcing cannot break a caller — and it means a rotation mismatch can never
//! be silently masked as success.
//!
//! The token compare is constant-time over the byte contents (XOR-fold, no
//! early exit) so a mismatch's response time doesn't leak a prefix-match
//! oracle. Length is compared first — leaking the token *length* is accepted
//! (standard practice absent a hashing step; the token is high-entropy).
//!
//! Nothing here ever logs a token, a header, or a body.

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::config::OtaBridgeConfig;

/// State for [`require_ota_token`]. Built once at startup in `main.rs` from
/// [`OtaBridgeConfig::from_env`]; cheap to clone (at most two `Arc<str>`).
#[derive(Clone)]
pub struct OtaTokenState {
    enabled: bool,
    enforce: bool,
    /// `[token, previous_token]`, blanks filtered out, **primary first**.
    /// The ordering is load-bearing: index 0 is `Allowed`, anything else is
    /// `AllowedRotating` (i.e. "you are still on the old secret").
    accepted: Vec<Arc<str>>,
}

impl OtaTokenState {
    pub fn new(config: &OtaBridgeConfig) -> Self {
        let accepted = [config.token.as_deref(), config.previous_token.as_deref()]
            .into_iter()
            .flatten()
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(Arc::from)
            .collect();
        Self {
            enabled: config.enabled,
            enforce: config.enforce,
            accepted,
        }
    }

    /// Test constructor.
    #[cfg(test)]
    fn for_test(enabled: bool, enforce: bool, tokens: &[&str]) -> Self {
        Self {
            enabled,
            enforce,
            accepted: tokens.iter().map(|t| Arc::from(*t)).collect(),
        }
    }
}

/// Axum `from_fn_with_state` middleware guarding the `/api/ota/*` subrouter.
///
/// Mounted OUTERMOST (outside `ville_write_guard`) so an unauthenticated probe
/// is rejected before it can touch state or any other gate.
pub async fn require_ota_token(
    State(state): State<OtaTokenState>,
    request: Request,
    next: Next,
) -> Response {
    let accepted: Vec<&str> = state.accepted.iter().map(|t| t.as_ref()).collect();
    let decision = check_ota_access(
        state.enabled,
        state.enforce,
        &accepted,
        request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok()),
    );

    match decision {
        OtaAccess::Allowed => next.run(request).await,
        OtaAccess::AllowedRotating => {
            tracing::warn!(
                target: "ota_bridge",
                "ota bridge authenticated with the PREVIOUS token — finish the rotation"
            );
            next.run(request).await
        }
        OtaAccess::AllowedUnauthenticated => {
            // Borrow the path BEFORE the request is consumed by `next`.
            let path = request.uri().path().to_string();
            tracing::warn!(
                target: "ota_bridge",
                path = %path,
                "ota bridge request accepted WITHOUT a bearer token (OTA_BRIDGE_ENFORCE=false)"
            );
            next.run(request).await
        }
        OtaAccess::Disabled => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "success": false,
                "error": "ota bridge is disabled",
            })),
        )
            .into_response(),
        OtaAccess::Unauthorized => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": "invalid or missing bearer token",
            })),
        )
            .into_response(),
    }
}

/// Pure decision function — the whole mode matrix, unit-testable without a
/// router.
#[derive(Debug, PartialEq, Eq)]
enum OtaAccess {
    /// Matched the primary token.
    Allowed,
    /// Matched the PREVIOUS (rotation) token — served, with a WARN.
    AllowedRotating,
    /// No credential presented and `OTA_BRIDGE_ENFORCE` is off — served, with
    /// a WARN. The migration lane, and the only permissive outcome.
    AllowedUnauthenticated,
    /// Flag off, or enforcing with nothing provisioned → the surface does not
    /// exist (503).
    Disabled,
    /// A credential WAS presented and did not match, or one was required and
    /// absent (401).
    Unauthorized,
}

fn check_ota_access(
    enabled: bool,
    enforce: bool,
    accepted: &[&str],
    authorization_header: Option<&str>,
) -> OtaAccess {
    // Ship-dark gate first: the surface is unavailable regardless of what the
    // caller sends.
    if !enabled {
        return OtaAccess::Disabled;
    }
    // Enforcing with no token provisioned is a misconfiguration, not an open
    // door — fail closed rather than fall back to the permissive lane.
    if enforce && accepted.is_empty() {
        return OtaAccess::Disabled;
    }

    match authorization_header.and_then(extract_bearer) {
        Some(presented) => {
            let matched = accepted
                .iter()
                .position(|expected| constant_time_eq(presented.as_bytes(), expected.as_bytes()));
            match matched {
                Some(0) => OtaAccess::Allowed,
                // Anything after index 0 is the rotation slot.
                Some(_) => OtaAccess::AllowedRotating,
                // A PRESENTED-BUT-WRONG credential is 401 even while
                // permissive. Do not "fall back" to AllowedUnauthenticated
                // here — that would mask a botched rotation as success.
                None => OtaAccess::Unauthorized,
            }
        }
        // Absent / malformed / non-Bearer / empty-token header.
        None if enforce => OtaAccess::Unauthorized,
        None => OtaAccess::AllowedUnauthenticated,
    }
}

/// Pull the token out of an `Authorization: Bearer <token>` header value.
/// Scheme match is case-insensitive per RFC 7235; the token is trimmed.
/// (Copied from `channel_token.rs` — see the module docs on why.)
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
/// (Copied from `channel_token.rs` — see the module docs on why.)
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

/// Operator-facing token fingerprint: the first 6 hex chars of
/// `sha256(value)`, or `"unset"`.
///
/// This is how an operator confirms new-hotel's `OTA_BRIDGE_TOKEN` and
/// ota-desk's `PMS_BRIDGE_TOKEN` hold the IDENTICAL string without either side
/// printing a secret — both repos print this fingerprint and the two are
/// compared by eye. 24 bits is not a security boundary; it is a typo detector.
pub fn token_fingerprint(value: Option<&str>) -> String {
    match value.map(str::trim).filter(|v| !v.is_empty()) {
        Some(v) => {
            let digest = Sha256::digest(v.as_bytes());
            digest
                .iter()
                .take(3)
                .map(|byte| format!("{byte:02x}"))
                .collect()
        }
        None => "unset".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str = "test-ota-bridge-token";
    const PREVIOUS: &str = "test-ota-bridge-token-previous";

    fn bearer(token: &str) -> String {
        format!("Bearer {token}")
    }

    // ---- the seven rows of the mode matrix ----------------------------

    /// Row 1: flag off wins over everything, including a perfect credential.
    #[test]
    fn row1_disabled_flag_wins_over_valid_token() {
        assert_eq!(
            check_ota_access(false, false, &[TOKEN], Some(&bearer(TOKEN))),
            OtaAccess::Disabled,
            "the surface must stay dark while OTA_BRIDGE_ENABLED is off"
        );
        assert_eq!(
            check_ota_access(false, true, &[TOKEN], Some(&bearer(TOKEN))),
            OtaAccess::Disabled
        );
        assert_eq!(check_ota_access(false, false, &[], None), OtaAccess::Disabled);
    }

    /// Row 2: enforcing with nothing provisioned is a misconfiguration —
    /// fail CLOSED (503), never fall through to the permissive lane.
    #[test]
    fn row2_enforcing_without_any_token_fails_closed() {
        assert_eq!(
            check_ota_access(true, true, &[], None),
            OtaAccess::Disabled,
            "enforce=true with no token must not accept an unauthenticated call"
        );
        assert_eq!(
            check_ota_access(true, true, &[], Some(&bearer(TOKEN))),
            OtaAccess::Disabled
        );
    }

    /// Row 3: enforcing, provisioned, no credential presented → 401.
    #[test]
    fn row3_enforcing_without_a_header_is_unauthorized() {
        assert_eq!(
            check_ota_access(true, true, &[TOKEN], None),
            OtaAccess::Unauthorized
        );
        assert_eq!(
            check_ota_access(true, true, &[TOKEN], Some(&format!("Basic {TOKEN}"))),
            OtaAccess::Unauthorized,
            "a non-Bearer scheme is 'no credential presented'"
        );
    }

    /// Row 4 — THE load-bearing one. A presented-but-wrong credential is 401
    /// in EVERY mode, including permissive. If this ever returns
    /// `AllowedUnauthenticated`, a botched token rotation would be masked as
    /// success and the enforce flip would be unsafe.
    #[test]
    fn row4_wrong_token_is_unauthorized_even_in_permissive_mode() {
        for enforce in [false, true] {
            assert_eq!(
                check_ota_access(true, enforce, &[TOKEN], Some(&bearer("wrong-token"))),
                OtaAccess::Unauthorized,
                "enforce={enforce}: a WRONG bearer must never ride the permissive lane"
            );
        }
        // Even with nothing provisioned: a presented credential cannot match.
        assert_eq!(
            check_ota_access(true, false, &[], Some(&bearer(TOKEN))),
            OtaAccess::Unauthorized
        );
    }

    /// Row 5: the migration lane — absent header, permissive mode → served,
    /// with a WARN the operator counts down to zero before enforcing.
    #[test]
    fn row5_absent_header_is_allowed_while_permissive() {
        assert_eq!(
            check_ota_access(true, false, &[TOKEN], None),
            OtaAccess::AllowedUnauthenticated
        );
        assert_eq!(
            check_ota_access(true, false, &[], None),
            OtaAccess::AllowedUnauthenticated
        );
        assert_eq!(
            check_ota_access(true, false, &[TOKEN], Some("NotBearer whatever")),
            OtaAccess::AllowedUnauthenticated,
            "a non-Bearer scheme presents no credential, so it rides the lane"
        );
    }

    /// Row 6: the primary token, in both modes, with a case-insensitive scheme.
    #[test]
    fn row6_primary_token_is_allowed() {
        for enforce in [false, true] {
            for scheme in ["Bearer", "bearer", "BEARER"] {
                assert_eq!(
                    check_ota_access(
                        true,
                        enforce,
                        &[TOKEN, PREVIOUS],
                        Some(&format!("{scheme} {TOKEN}"))
                    ),
                    OtaAccess::Allowed,
                    "enforce={enforce} scheme={scheme} should be accepted"
                );
            }
        }
    }

    /// Row 7: the rotation slot is accepted but flagged, so a half-finished
    /// rotation is visible rather than silent.
    #[test]
    fn row7_previous_token_is_allowed_with_a_rotation_warning() {
        for enforce in [false, true] {
            assert_eq!(
                check_ota_access(true, enforce, &[TOKEN, PREVIOUS], Some(&bearer(PREVIOUS))),
                OtaAccess::AllowedRotating,
                "enforce={enforce}: the previous token must be served, but distinguishably"
            );
        }
    }

    // ---- copied helpers keep their `channel_token.rs` semantics --------

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

    // ---- state assembly ------------------------------------------------

    #[test]
    fn state_builder_filters_blanks_and_keeps_primary_first() {
        let cfg = OtaBridgeConfig {
            enabled: true,
            enforce: false,
            token: Some(format!("  {TOKEN}  ")),
            previous_token: Some("   ".to_string()),
        };
        let state = OtaTokenState::new(&cfg);
        assert!(state.enabled);
        assert!(!state.enforce);
        assert_eq!(
            state.accepted.len(),
            1,
            "a whitespace-only previous token must be dropped, not accepted"
        );
        assert_eq!(state.accepted[0].as_ref(), TOKEN, "the token is trimmed");

        let rotating = OtaTokenState::new(&OtaBridgeConfig {
            enabled: true,
            enforce: true,
            token: Some(TOKEN.to_string()),
            previous_token: Some(PREVIOUS.to_string()),
        });
        let accepted: Vec<&str> = rotating.accepted.iter().map(|t| t.as_ref()).collect();
        assert_eq!(accepted, vec![TOKEN, PREVIOUS], "primary must sort first");

        // for_test parity so router tests can construct states directly.
        let t = OtaTokenState::for_test(true, true, &[TOKEN]);
        assert_eq!(t.accepted[0].as_ref(), TOKEN);
    }

    #[test]
    fn empty_config_yields_a_dark_closed_surface() {
        let state = OtaTokenState::new(&OtaBridgeConfig::default());
        assert!(!state.enabled);
        assert!(state.accepted.is_empty());
    }

    // ---- operator fingerprint -----------------------------------------

    #[test]
    fn fingerprint_is_six_hex_chars_and_never_the_value() {
        let fp = token_fingerprint(Some(TOKEN));
        assert_eq!(fp.len(), 6, "6 hex chars = first 3 bytes of sha256");
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(!fp.contains(TOKEN));
        // Stable and value-derived: same input ⇒ same fingerprint, and a
        // different secret ⇒ (overwhelmingly likely) a different one.
        assert_eq!(fp, token_fingerprint(Some(&format!(" {TOKEN} "))));
        assert_ne!(fp, token_fingerprint(Some(PREVIOUS)));
    }

    #[test]
    fn fingerprint_of_absent_or_blank_is_unset() {
        assert_eq!(token_fingerprint(None), "unset");
        assert_eq!(token_fingerprint(Some("")), "unset");
        assert_eq!(token_fingerprint(Some("   ")), "unset");
    }
}
