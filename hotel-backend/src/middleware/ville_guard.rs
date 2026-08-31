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
//! ## The exemptions (housekeeping-ops, 2026-08-11; linen 2026-08-31)
//!
//! [`is_ville_exempt_path`] admits exactly TWO routes regardless of the flag,
//! both maid reports on the `/hk` surface:
//!
//! * `POST /api/hk/rooms/{id}/cleaning` — the cleaning report.
//! * `POST /api/hk/rooms/{id}/linen-shortage` — the linen-shortage (ขาดผ้า)
//!   report (migration 088). **PG-ONLY**: it inserts `ht_hk_linen_reports` rows
//!   and nothing else — no writeback intent, no outbox row, no domain event,
//!   because iHOTEL has no linen counterpart to mirror to. So the
//!   admission-vs-delivery argument below does not merely hold for it, it is
//!   vacuous: there is no intent that `HFVILLE_WRITEBACK_INTENTS` could park as
//!   `'skipped'`, hence no way for canonical PG to run ahead of Ville's iHOTEL.
//!   This is the safest possible thing to exempt.
//!
//! ### Current production state
//!
//! `HFVILLE_WRITES_ENABLED=true` has been set since 2026-06-29 — Ville coequal
//! writes are LIVE — so this gate admits everything and the exemption is
//! **inert in production today**. It exists for the case where an operator
//! turns Ville writes back off: the maid surface is a different concern from
//! front-desk write policy, and a housekeeping report should not be collateral
//! damage of that toggle.
//!
//! The exemption does NOT punch a hole in the real kill switch for Ville legacy
//! writes. This gate is HTTP ADMISSION only; legacy delivery is stopped by
//! `HFVILLE_WRITEBACK_ENABLED=false` on the Ville worker (or narrowed by
//! `HFVILLE_WRITEBACK_INTENTS`). With writes disabled, an admitted cleaning
//! report flips canonical PG and its writeback waits in the durable outbox.
//!
//! ### Why an exemption rather than flipping the flag
//!
//! If Ville writes were ever disabled and we re-enabled them wholesale just to
//! let maids through, every other Ville mutation would be admitted too. Should
//! `HFVILLE_WRITEBACK_INTENTS` be narrow at that moment, those mutations would
//! write canonical PG while their intents parked as `'skipped'` — canonical PG
//! and Ville's iHOTEL silently diverging. Keeping the admitted set narrow keeps
//! admission and delivery consistent under every combination of the two knobs.
//!
//! The exemptions are deliberately NARROW: exact segment match against a
//! closed list of leaf names, POST only, numeric room id. They do NOT cover
//! `/broken-items` (retired, 410) or any other hk or non-hk mutation.

use axum::extract::State;
use axum::http::Method;
use axum::response::{IntoResponse, Response};

use crate::error::ApiError;
use crate::routes::mode::AppState;

/// The exempt leaf segments under `/api/hk/rooms/{room_id}/`. A closed list,
/// matched by exact equality — never a prefix or `starts_with`.
///
/// `linen-shortage` is the PG-only one (migration 088): it writes
/// `ht_hk_linen_reports` and enqueues nothing, so admitting it cannot desync
/// canonical PG from Ville's iHOTEL under any setting of
/// `HFVILLE_WRITEBACK_INTENTS`. See the module docs.
const VILLE_EXEMPT_ROOM_ACTIONS: [&str; 2] = ["cleaning", "linen-shortage"];

/// Is this request path one of the Ville-exempt routes? PURE.
///
/// Matches exactly `/api/hk/rooms/{room_id}/{action}` with a numeric `room_id`
/// and `action` in [`VILLE_EXEMPT_ROOM_ACTIONS`] — five segments, no more. A
/// trailing slash yields a trailing empty segment and therefore does NOT match,
/// and no prefix/substring test is used, so `/api/hk/rooms/1/cleaning/anything`,
/// `/api/hk/rooms/1/linen-shortageX` and `/api/hk/rooms/1/broken-items` all fall
/// through to the normal gate.
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
        (Some("api"), Some("hk"), Some("rooms"), Some(room_id), Some(action))
            if VILLE_EXEMPT_ROOM_ACTIONS.contains(&action) =>
        {
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
    const LINEN: &str = "/api/hk/rooms/42/linen-shortage";
    const BROKEN: &str = "/api/hk/rooms/42/broken-items";
    const BOOKING: &str = "/api/new/bookings";
    const VILLE_Q: Option<&str> = Some("branch=hfville");

    // ---- path matcher ---------------------------------------------------

    #[test]
    fn exempt_path_matches_only_the_two_maid_report_routes() {
        assert!(is_ville_exempt_path(CLEANING));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/cleaning"));
        assert!(is_ville_exempt_path(LINEN));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/linen-shortage"));

        for not_exempt in [
            BROKEN,
            BOOKING,
            "/api/hk/rooms/42",
            "/api/hk/rooms/42/cleaning/",            // trailing slash
            "/api/hk/rooms/42/cleaning/extra",       // deeper path
            "/api/hk/rooms//cleaning",               // empty room id
            "/api/hk/rooms/4a2/cleaning",            // non-numeric room id
            "/api/hk/rooms/42/cleaningX",            // near-miss suffix
            "/x/api/hk/rooms/42/cleaning",           // prefixed
            "api/hk/rooms/42/cleaning",              // no leading slash
            "/api/hk/rooms/42/linen-shortage/",      // trailing slash
            "/api/hk/rooms/42/linen-shortage/extra", // deeper path
            "/api/hk/rooms//linen-shortage",         // empty room id
            "/api/hk/rooms/4a2/linen-shortage",      // non-numeric room id
            "/api/hk/rooms/42/linen-shortageX",      // near-miss suffix
            "/api/hk/rooms/42/linen_shortage",       // underscore, not the route
            "/api/hk/rooms/42/linen",                // truncated leaf
            "/x/api/hk/rooms/42/linen-shortage",     // prefixed
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

    // ---- the admission matrix -------------------------------------------

    /// With Ville writes DISABLED (not the current production state, but the
    /// posture an operator can return to): the maid's cleaning report is
    /// admitted for `branch=hfville`, and NOTHING else is. Keeping the
    /// admitted set this narrow is what stops an admitted mutation from
    /// outrunning a narrowed writeback allowlist.
    #[test]
    fn writes_disabled_admits_only_the_maid_reports() {
        // Admitted.
        assert!(
            !ville_write_blocked(&Method::POST, CLEANING, VILLE_Q, false),
            "hfville cleaning must be admitted while Ville writes are disabled"
        );
        // The linen report is PG-only — nothing it writes can reach legacy, so
        // admitting it cannot desync canonical PG from Ville's iHOTEL.
        assert!(
            !ville_write_blocked(&Method::POST, LINEN, VILLE_Q, false),
            "hfville linen-shortage must be admitted while Ville writes are disabled"
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

    /// The exemptions are POST-only: they exist for the maid's REPORTS, so a
    /// DELETE/PUT aimed at either path must not slip through.
    #[test]
    fn exemption_is_post_only() {
        for path in [CLEANING, LINEN] {
            for method in [Method::PUT, Method::PATCH, Method::DELETE] {
                assert!(
                    ville_write_blocked(&method, path, VILLE_Q, false),
                    "{method} on {path} must NOT be exempt"
                );
            }
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
        for path in [CLEANING, LINEN, BROKEN, BOOKING] {
            assert!(!ville_write_blocked(&Method::POST, path, VILLE_Q, true));
        }
    }
}
