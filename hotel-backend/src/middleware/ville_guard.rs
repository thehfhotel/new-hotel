//! HF Ville write admission gate (ADR 0002 / Ship-A).
//!
//! When `HFVILLE_WRITES_ENABLED` is off, every mutating request carrying
//! `?branch=hfville` is rejected with 403 BEFORE it can reach a handler that
//! would write the HF Hotel pool. The `branch` query param is parsed properly
//! (URL-decoded), never substring-matched.
//!
//! Lives in the lib (not `main.rs`) so the decision is unit-testable and so
//! integration tests can drive the REAL layered router — see
//! `tests/test_hk_ville_guard.rs`.
//!
//! ## The one exemption (housekeeping-ops, 2026-08-11)
//!
//! [`is_ville_exempt_path`] admits exactly ONE route regardless of the flag:
//! `POST /api/hk/rooms/{id}/cleaning`, the maid's cleaning report.
//!
//! Why an exemption is the SAFE choice here, and blanket-enabling is not:
//!
//! - Launch intent is `HFVILLE_WRITES_ENABLED` UNSET (general Ville mutations
//!   stay inadmissible) + `HFVILLE_WRITEBACK_INTENTS=mark_room_clean` (only the
//!   housekeeping intent reaches Ville's iHOTEL).
//! - Those two must agree. If we instead flipped `HFVILLE_WRITES_ENABLED` on to
//!   let maids through, every other Ville mutation would be admitted, write
//!   canonical PG, and then have its writeback intent PARKED as `'skipped'` by
//!   the allowlist. Canonical PG and Ville's iHOTEL would silently diverge —
//!   precisely the failure this project must not create.
//! - So the admitted set (this route) is kept identical to the allowlisted
//!   writeback set (`mark_room_clean`), and divergence stays impossible.
//!
//! The exemption is deliberately NARROW: exact segment match, POST only,
//! numeric room id. It does NOT cover `/broken-items` (retired, 410) or any
//! other hk or non-hk mutation — those stay blocked until the coequal-writes
//! program flips the flag on its own schedule.

use axum::extract::State;
use axum::http::Method;
use axum::response::{IntoResponse, Response};

use crate::error::ApiError;
use crate::routes::mode::AppState;

/// Is this request path the ONE Ville-exempt route? PURE.
///
/// Matches exactly `/api/hk/rooms/{room_id}/cleaning` with a numeric
/// `room_id` — five segments, no more. A trailing slash yields a trailing
/// empty segment and therefore does NOT match, and no prefix/substring test is
/// used, so `/api/hk/rooms/1/cleaning/anything` and
/// `/api/hk/rooms/1/broken-items` both fall through to the normal gate.
pub fn is_ville_exempt_path(path: &str) -> bool {
    let mut segments = path.split('/');
    // A path starts with '/', so the first split segment is empty.
    if segments.next() != Some("") {
        return false;
    }
    match (
        segments.next(),
        segments.next(),
        segments.next(),
        segments.next(),
        segments.next(),
    ) {
        (Some("api"), Some("hk"), Some("rooms"), Some(room_id), Some("cleaning")) => {
            // Nothing may follow the fifth segment.
            segments.next().is_none()
                && !room_id.is_empty()
                && room_id.bytes().all(|b| b.is_ascii_digit())
        }
        _ => false,
    }
}

/// Does the `branch` query param select HF Ville? PURE. Properly URL-decoded
/// rather than substring-matched, so `?note=hfville` can never admit or block
/// a request by accident.
pub fn targets_ville(query: Option<&str>) -> bool {
    query
        .map(|q| {
            form_urlencoded::parse(q.as_bytes()).any(|(k, v)| k == "branch" && v == "hfville")
        })
        .unwrap_or(false)
}

/// The full admission decision. PURE — the whole matrix is unit-tested below.
///
/// Returns `true` when the request must be rejected with 403.
pub fn ville_write_blocked(
    method: &Method,
    path: &str,
    query: Option<&str>,
    writes_enabled: bool,
) -> bool {
    if writes_enabled {
        return false;
    }
    let mutating = matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );
    if !mutating {
        return false;
    }
    if !targets_ville(query) {
        return false;
    }
    // The single housekeeping exemption — POST only.
    if *method == Method::POST && is_ville_exempt_path(path) {
        return false;
    }
    true
}

/// Guard: reject a disabled-Ville mutation with 403 before it reaches a
/// handler. Inspects only method + path + query string — never the body.
pub async fn ville_write_guard(
    State(state): State<AppState>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    if ville_write_blocked(
        req.method(),
        req.uri().path(),
        req.uri().query(),
        state.hfville_writes_enabled,
    ) {
        return ApiError::Forbidden(
            "HF Ville writes are disabled (HFVILLE_WRITES_ENABLED=false); manage HF Ville via iHOTEL"
                .to_string(),
        )
        .into_response();
    }
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEANING: &str = "/api/hk/rooms/42/cleaning";
    const BROKEN: &str = "/api/hk/rooms/42/broken-items";
    const BOOKING: &str = "/api/new/bookings";
    const VILLE_Q: Option<&str> = Some("branch=hfville");

    // ---- path matcher ---------------------------------------------------

    #[test]
    fn exempt_path_matches_only_the_cleaning_route() {
        assert!(is_ville_exempt_path(CLEANING));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/cleaning"));

        for not_exempt in [
            BROKEN,
            BOOKING,
            "/api/hk/rooms/42",
            "/api/hk/rooms/42/cleaning/",       // trailing slash
            "/api/hk/rooms/42/cleaning/extra",  // deeper path
            "/api/hk/rooms//cleaning",          // empty room id
            "/api/hk/rooms/4a2/cleaning",       // non-numeric room id
            "/api/hk/rooms/42/cleaningX",       // near-miss suffix
            "/x/api/hk/rooms/42/cleaning",      // prefixed
            "api/hk/rooms/42/cleaning",         // no leading slash
        ] {
            assert!(
                !is_ville_exempt_path(not_exempt),
                "{not_exempt} must NOT be exempt"
            );
        }
    }

    // ---- branch parsing -------------------------------------------------

    #[test]
    fn ville_branch_is_parsed_not_substring_matched() {
        assert!(targets_ville(Some("branch=hfville")));
        assert!(targets_ville(Some("foo=1&branch=hfville&bar=2")));
        assert!(!targets_ville(Some("branch=hfhotel")));
        assert!(!targets_ville(None));
        // A value that merely CONTAINS the word must not trigger the gate.
        assert!(!targets_ville(Some("note=hfville")));
        assert!(!targets_ville(Some("branch=hfvillex")));
    }

    // ---- the launch matrix ----------------------------------------------

    /// LAUNCH CONFIG (`HFVILLE_WRITES_ENABLED` unset): the maid's cleaning
    /// report is admitted for `branch=hfville`, and NOTHING else is. This is
    /// the invariant that keeps the admitted set identical to the allowlisted
    /// writeback set, so canonical PG cannot diverge from Ville's iHOTEL.
    #[test]
    fn writes_disabled_admits_only_hfville_cleaning() {
        // Admitted.
        assert!(
            !ville_write_blocked(&Method::POST, CLEANING, VILLE_Q, false),
            "hfville cleaning must be admitted while Ville writes are disabled"
        );

        // Everything else on branch=hfville stays blocked.
        assert!(
            ville_write_blocked(&Method::POST, BROKEN, VILLE_Q, false),
            "broken-items must stay blocked"
        );
        assert!(
            ville_write_blocked(&Method::POST, BOOKING, VILLE_Q, false),
            "a general Ville mutation must stay blocked"
        );
        for method in [Method::PUT, Method::PATCH, Method::DELETE] {
            assert!(
                ville_write_blocked(&method, BOOKING, VILLE_Q, false),
                "{method} on a Ville mutation must stay blocked"
            );
        }
    }

    /// The exemption is POST-only: it exists for the maid's cleaning REPORT,
    /// so a DELETE/PUT aimed at the same path must not slip through.
    #[test]
    fn exemption_is_post_only() {
        for method in [Method::PUT, Method::PATCH, Method::DELETE] {
            assert!(
                ville_write_blocked(&method, CLEANING, VILLE_Q, false),
                "{method} on the cleaning path must NOT be exempt"
            );
        }
    }

    /// The gate only ever fires on mutating methods against branch=hfville.
    #[test]
    fn reads_and_non_ville_branches_are_never_blocked() {
        assert!(!ville_write_blocked(&Method::GET, BOOKING, VILLE_Q, false));
        assert!(!ville_write_blocked(
            &Method::POST,
            BOOKING,
            Some("branch=hfhotel"),
            false
        ));
        assert!(!ville_write_blocked(&Method::POST, BOOKING, None, false));
    }

    /// With the flag ON nothing is blocked — the exemption changes no
    /// behavior in the eventual coequal-writes world.
    #[test]
    fn writes_enabled_blocks_nothing() {
        for path in [CLEANING, BROKEN, BOOKING] {
            assert!(!ville_write_blocked(&Method::POST, path, VILLE_Q, true));
        }
    }
}
