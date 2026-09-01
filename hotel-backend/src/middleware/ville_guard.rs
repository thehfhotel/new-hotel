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
//! ## The exemptions (housekeeping-ops, 2026-08-11; linen 2026-08-31;
//! room signals 2026-09-01; linen resolve 2026-09-01; Report HK 2026-09-02)
//!
//! [`is_ville_exempt_path`] admits, regardless of the flag, exactly the maid
//! surface's own write routes — and nothing else:
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
//! * `POST /api/hk/rooms/{id}/linen-shortage/resolve` — the maid's เติมผ้าแล้ว
//!   (migration 090), which closes every open row for the room. **PG-ONLY on
//!   exactly the same terms as the report it completes**, and if anything more
//!   narrowly: one `UPDATE`, no writeback intent, no outbox row, and not even a
//!   domain event. It is the ONE **six-segment** exemption on this surface —
//!   [`is_ville_exempt_path`] admits a single optional trailing segment,
//!   matched as a `(action, sub-action)` PAIR against
//!   [`VILLE_EXEMPT_ROOM_SUBACTIONS`], never as a prefix. So `resolve` is
//!   exempt as the completion of `linen-shortage` and nowhere else.
//! * `POST /api/hk/rooms/{id}/signals` and
//!   `POST /api/hk/signals/{id}/{ack|done|cancel|answer}` — the room signals of
//!   ADR 0008 (migration 089). **PG-ONLY on the same terms as linen, and
//!   irreversibly so**: `ht_hk_room_signals` has no legacy counterpart to
//!   mirror to, so there is no writeback recipe to write, no intent to
//!   allowlist, and no dark flag waiting to enable one. They DO publish domain
//!   events — but those are `event_log` + `pg_notify` rows feeding this app's
//!   own SSE fan-out, not a legacy write, and the writeback worker dispatches
//!   `WritebackIntent`s, not events.
//!
//!   The DESK half of the same feature (`/api/housekeeping/signals/*`) is
//!   deliberately NOT exempt. The exemption exists so a MAID's work is not
//!   collateral damage of a front-desk write-policy toggle; a front-desk
//!   mutation is exactly what that toggle governs.
//! * `POST /api/hk/rooms/{id}/report`, `POST /api/hk/reports/{id}/verify`,
//!   `POST /api/hk/reports/{id}/return` and `POST /api/hk/report-photos` — the
//!   Report HK sheet of migration 091. **PG-ONLY on the same absolute terms as
//!   the room signals**: `ht_hk_room_reports` / `_items` / `_photos` have no
//!   legacy counterpart at all — iHOTEL knows nothing about the Report HK
//!   sheet — so there is no writeback recipe to write, no intent to allowlist,
//!   and no dark flag waiting to enable one. The submit publishes domain events
//!   only through the `item_missing` / `item_damaged` room signals it raises,
//!   which are already exempt on their own path; an event is not a legacy
//!   write.
//!
//!   The VERDICTS are exempt too, unlike every other reception-side write on
//!   this estate, and that is deliberate rather than an oversight: they are
//!   served from the MAID surface's own router (`/api/hk/*`) by a receptionist
//!   standing in the room with her phone, not from the desk tree. A 403 there
//!   would strand a maid's submitted report unjudged for as long as the toggle
//!   is off — exactly the collateral damage the exemption exists to prevent.
//!   The desk tree (`/api/housekeeping/*`) has no report routes at all.
//!
//!   `POST /api/hk/report-photos` is the ONE exempt path with **no id**: a
//!   photo is uploaded BEFORE the report it will belong to exists, so there is
//!   nothing to address yet. It is matched as a whole-path equality, never as
//!   a prefix — `/api/hk/report-photos/9` (the READ) and
//!   `/api/hk/report-photosX` both fall through.
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
//! closed list of leaf names (and, for the one six-segment shape, a closed list
//! of leaf PAIRS), POST only, numeric room id. They do NOT cover
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
///
/// `signals` (migration 089, ADR 0008) is PG-only in the same absolute sense,
/// and more so: `ht_hk_room_signals` has no legacy counterpart to mirror TO —
/// iHOTEL knows nothing about room signals — so there is no writeback recipe,
/// no intent, and no future one to gate. It raises one row and publishes a
/// domain event for the SSE fan-out; an event is not a legacy write.
///
/// `report` (migration 091, Report HK) is PG-only in the same absolute sense:
/// `ht_hk_room_reports` + its item and photo tables have no legacy counterpart
/// — iHOTEL knows nothing about the Report HK sheet — so there is no writeback
/// recipe, no intent, and no future one to gate. The submit DOES publish
/// domain events, but only via the `item_missing` / `item_damaged` room signals
/// it raises, which were already exempt on their own path; an event is not a
/// legacy write, and the writeback worker dispatches `WritebackIntent`s.
const VILLE_EXEMPT_ROOM_ACTIONS: [&str; 4] = ["cleaning", "linen-shortage", "signals", "report"];

/// The exempt leaf segments under `/api/hk/signals/{signal_id}/` — ADR 0008's
/// lifecycle actions. A closed list, matched by exact equality.
///
/// These are NOT under `/rooms/`, which is why they need their own matcher
/// rather than an entry in [`VILLE_EXEMPT_ROOM_ACTIONS`]: acting on a signal
/// addresses the signal, which already knows its room. Same PG-only argument
/// as `signals` above — each one flips `sig_status` and publishes an event, and
/// `answer` additionally inserts child signal rows. All canonical PG, none of
/// it legacy.
///
/// `answer` is included deliberately: without it a HF Ville maid could open a
/// ขอเช็คห้อง's answer screen and be 403'd at the one moment a guest is waiting
/// at the counter — the exact "housekeeping is not collateral damage of a
/// front-desk write-policy toggle" case the exemption exists for.
const VILLE_EXEMPT_SIGNAL_ACTIONS: [&str; 4] = ["ack", "done", "cancel", "answer"];

/// The exempt leaf segments under `/api/hk/reports/{report_id}/` — Report HK's
/// two verdicts (migration 091). A closed list, matched by exact equality.
///
/// These are NOT under `/rooms/`, for exactly [`VILLE_EXEMPT_SIGNAL_ACTIONS`]'s
/// reason: a verdict addresses the report, which already knows its room, so
/// repeating the room id in the path would only be a second thing that can
/// disagree with the row. Hence a THIRD collection rather than an entry in
/// [`VILLE_EXEMPT_ROOM_ACTIONS`] — and, deliberately, a third LIST rather than
/// a shared one, so `ack` can never become a report verdict nor `verify` a
/// signal action.
///
/// PG-only on the same terms as everything else here: each one flips
/// `rr_status`, stamps the countersignature, and (for `verify`) attaches the
/// verifier's photo rows. No writeback intent, no outbox row, and no domain
/// event of its own.
///
/// `verify` is included deliberately: without it a HF Ville receptionist could
/// open a maid's report and be 403'd at the moment she is standing in the room
/// with her phone out — the exact "housekeeping is not collateral damage of a
/// front-desk write-policy toggle" case the exemption exists for.
const VILLE_EXEMPT_REPORT_ACTIONS: [&str; 2] = ["verify", "return"];

/// The ONE exempt FOUR-segment path: `POST /api/hk/report-photos` (migration
/// 091), the Report HK photo intake.
///
/// Matched as a WHOLE PATH by exact equality — it carries no id at all, because
/// a photo is uploaded BEFORE the report it will belong to exists, so there is
/// nothing yet to address. That makes it the only exemption on this surface
/// that the segment matcher below cannot express, and giving it its own
/// constant keeps the segment matcher from having to grow an "id optional"
/// mode that would loosen every other shape.
///
/// PG-only, and about as narrowly as anything here: one INSERT of an
/// UNATTACHED row. It cannot even reach a report until a submit/verify names
/// it, and those are separately exempt on their own merits.
const VILLE_EXEMPT_REPORT_PHOTOS: &str = "/api/hk/report-photos";

/// The ONE exempt SIX-segment shape: `/api/hk/rooms/{id}/{action}/{sub}`, as a
/// closed list of `(action, sub)` PAIRS — migration 090's
/// `POST /api/hk/rooms/{id}/linen-shortage/resolve` (เติมผ้าแล้ว, the maid
/// marking a room restocked).
///
/// PG-only on exactly `linen-shortage`'s terms, which is what makes it
/// exemptible: it closes `ht_hk_linen_reports` rows with one `UPDATE` and
/// enqueues nothing — no writeback intent, no outbox row, and (unlike the room
/// signals) not even a domain event. There is no intent a narrowed
/// `HFVILLE_WRITEBACK_INTENTS` could park as `'skipped'`, hence no way for
/// canonical PG to run ahead of Ville's iHOTEL.
///
/// A PAIR list rather than a second leaf list, deliberately: `resolve` is
/// exempt only as the completion of `linen-shortage`, never as a leaf under
/// `cleaning`, `signals`, or a future action. Widening this list means naming
/// both halves.
const VILLE_EXEMPT_ROOM_SUBACTIONS: [(&str, &str); 1] = [("linen-shortage", "resolve")];

/// Is this request path one of the Ville-exempt routes? PURE.
///
/// FIVE shapes. Four require a numeric id and nothing after the leaf; the fifth
/// is a whole-path equality with no id at all:
///
/// * `/api/hk/rooms/{room_id}/{action}` with `action` in
///   [`VILLE_EXEMPT_ROOM_ACTIONS`] (five segments);
/// * `/api/hk/signals/{signal_id}/{action}` with `action` in
///   [`VILLE_EXEMPT_SIGNAL_ACTIONS`] (five segments);
/// * `/api/hk/reports/{report_id}/{action}` with `action` in
///   [`VILLE_EXEMPT_REPORT_ACTIONS`] (five segments — migration 091's verdicts);
/// * `/api/hk/rooms/{room_id}/{action}/{sub}` with `(action, sub)` in
///   [`VILLE_EXEMPT_ROOM_SUBACTIONS`] (SIX segments — migration 090's linen
///   resolve, and nothing else);
/// * exactly [`VILLE_EXEMPT_REPORT_PHOTOS`] (four segments, no id — migration
///   091's photo intake, which is uploaded before any report exists).
///
/// The three collections keep SEPARATE action lists on purpose: a leaf is
/// exempt under the collection it belongs to and nowhere else, so
/// `/api/hk/rooms/1/verify` and `/api/hk/reports/1/cleaning` are both refused.
///
/// The sixth segment is OPTIONAL and, when present, is matched as a pair with
/// the fifth against a closed list — so that widening admits exactly one more
/// path. A six-segment path under `/signals/` or `/reports/` matches nothing,
/// and neither does an exempt five-segment leaf with an arbitrary sixth segment
/// appended.
///
/// A trailing slash yields a trailing empty segment and therefore does NOT
/// match, and no prefix/substring test is used, so
/// `/api/hk/rooms/1/cleaning/anything`, `/api/hk/rooms/1/linen-shortageX`,
/// `/api/hk/signals/1/ackX`, `/api/hk/report-photosX` and
/// `/api/hk/rooms/1/broken-items` all fall through to the normal gate.
///
/// `GET /api/hk/signals`, `GET /api/hk/events`, `GET /api/hk/reports` and
/// `GET /api/hk/report-photos/{id}` need no entry at all — the gate only ever
/// fires on mutating methods.
pub fn is_ville_exempt_path(path: &str) -> bool {
    // The one shape with no id: an exact whole-path match, so
    // `/api/hk/report-photos/9` (the READ) and `/api/hk/report-photosX` both
    // fall through to the segment matcher and then to the normal gate.
    if path == VILLE_EXEMPT_REPORT_PHOTOS {
        return true;
    }

    let mut segments = path.split('/');
    // A path starts with '/', so the first split segment is empty.
    if segments.next() != Some("") {
        return false;
    }
    let (collection, id, action) = match (
        segments.next(),
        segments.next(),
        segments.next(),
        segments.next(),
        segments.next(),
    ) {
        (Some("api"), Some("hk"), Some(collection), Some(id), Some(action)) => {
            (collection, id, action)
        }
        _ => return false,
    };
    // The id must be a bare number, on every shape.
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    // The OPTIONAL sixth segment. Nothing may follow IT — so a trailing slash
    // (which yields an empty sixth or seventh segment) and any deeper path
    // still fall through, exactly as before.
    let sub_action = segments.next();
    if segments.next().is_some() {
        return false;
    }
    match (collection, sub_action) {
        ("rooms", None) => VILLE_EXEMPT_ROOM_ACTIONS.contains(&action),
        ("signals", None) => VILLE_EXEMPT_SIGNAL_ACTIONS.contains(&action),
        ("reports", None) => VILLE_EXEMPT_REPORT_ACTIONS.contains(&action),
        // The one six-segment exemption, matched as a PAIR: `resolve` is exempt
        // only under `linen-shortage`, never as a leaf under another action.
        ("rooms", Some(sub)) => VILLE_EXEMPT_ROOM_SUBACTIONS.contains(&(action, sub)),
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
    const LINEN_RESOLVE: &str = "/api/hk/rooms/42/linen-shortage/resolve";
    const SIGNALS: &str = "/api/hk/rooms/42/signals";
    const SIGNAL_ACK: &str = "/api/hk/signals/7/ack";
    const SIGNAL_DONE: &str = "/api/hk/signals/7/done";
    const SIGNAL_CANCEL: &str = "/api/hk/signals/7/cancel";
    const SIGNAL_ANSWER: &str = "/api/hk/signals/7/answer";
    // Report HK (migration 091).
    const REPORT_SUBMIT: &str = "/api/hk/rooms/42/report";
    const REPORT_VERIFY: &str = "/api/hk/reports/9/verify";
    const REPORT_RETURN: &str = "/api/hk/reports/9/return";
    const REPORT_PHOTOS: &str = "/api/hk/report-photos";
    const BROKEN: &str = "/api/hk/rooms/42/broken-items";
    const BOOKING: &str = "/api/new/bookings";
    const VILLE_Q: Option<&str> = Some("branch=hfville");

    // ---- path matcher ---------------------------------------------------

    #[test]
    fn exempt_path_matches_only_the_maid_tree_write_routes() {
        assert!(is_ville_exempt_path(CLEANING));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/cleaning"));
        assert!(is_ville_exempt_path(LINEN));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/linen-shortage"));
        // Migration 090's เติมผ้าแล้ว — the ONE six-segment shape.
        assert!(is_ville_exempt_path(LINEN_RESOLVE));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/linen-shortage/resolve"));
        // ADR 0008 room signals: the raise hangs off /rooms/, the lifecycle
        // actions off /signals/ — two shapes, one matcher.
        assert!(is_ville_exempt_path(SIGNALS));
        assert!(is_ville_exempt_path("/api/hk/rooms/1/signals"));
        for action in [SIGNAL_ACK, SIGNAL_DONE, SIGNAL_CANCEL, SIGNAL_ANSWER] {
            assert!(is_ville_exempt_path(action), "{action} must be exempt");
        }
        // Report HK (migration 091): the submission hangs off /rooms/, the two
        // verdicts off /reports/, and the photo intake is the one shape with no
        // id at all.
        for report_path in [REPORT_SUBMIT, REPORT_VERIFY, REPORT_RETURN, REPORT_PHOTOS] {
            assert!(is_ville_exempt_path(report_path), "{report_path} must be exempt");
        }
        assert!(is_ville_exempt_path("/api/hk/rooms/1/report"));
        assert!(is_ville_exempt_path("/api/hk/reports/1/verify"));
        assert!(is_ville_exempt_path("/api/hk/reports/1/return"));

        for not_exempt in [
            BROKEN,
            // The two collections' action lists are NOT interchangeable: a
            // leaf is exempt under the collection it belongs to and nowhere
            // else, which is what keeps the widening narrow.
            "/api/hk/rooms/42/ack",
            "/api/hk/rooms/42/answer",
            "/api/hk/signals/7/cleaning",
            "/api/hk/signals/7/linen-shortage",
            "/api/hk/signals/7/signals",
            // Near-misses and malformed ids on the new shape.
            "/api/hk/signals/7/ackX",
            "/api/hk/signals/7/ack/",
            "/api/hk/signals/7/ack/extra",
            "/api/hk/signals//ack",
            "/api/hk/signals/7a/ack",
            "/api/hk/signals/7",
            "/api/hk/signals",
            "/x/api/hk/signals/7/ack",
            "api/hk/signals/7/ack",
            "/api/hk/rooms/42/signalsX",
            "/api/hk/rooms/42/signals/",
            "/api/hk/rooms/42/signals/extra",
            // ---- Report HK: the three collections' action lists are NOT
            // interchangeable, and the widening must admit exactly the four
            // paths above rather than a shape.
            "/api/hk/rooms/42/verify",
            "/api/hk/rooms/42/return",
            "/api/hk/reports/9/cleaning",
            "/api/hk/reports/9/signals",
            "/api/hk/reports/9/linen-shortage",
            "/api/hk/reports/9/ack",
            "/api/hk/reports/9/done",
            "/api/hk/reports/9/cancel",
            "/api/hk/reports/9/answer",
            "/api/hk/signals/7/verify",
            "/api/hk/signals/7/return",
            "/api/hk/signals/7/report",
            // Near-misses and malformed ids on the report shapes.
            "/api/hk/rooms/42/reportX",
            "/api/hk/rooms/42/reports",
            "/api/hk/rooms/42/report/",
            "/api/hk/rooms/42/report/extra",
            "/api/hk/rooms//report",
            "/api/hk/rooms/4a2/report",
            "/api/hk/reports/9/verifyX",
            "/api/hk/reports/9/verify/",
            "/api/hk/reports/9/verify/extra",
            "/api/hk/reports//verify",
            "/api/hk/reports/9a/verify",
            "/api/hk/reports/9",
            "/api/hk/reports",
            "/x/api/hk/reports/9/verify",
            "api/hk/reports/9/verify",
            "/api/housekeeping/reports/9/verify",
            // The photo intake is a WHOLE-PATH equality, never a prefix — so
            // the READ endpoint and every near-miss fall through. This is the
            // half that matters: a `starts_with` here would exempt every
            // `/api/hk/report-photos*` path including a future mutation.
            "/api/hk/report-photos/9",
            "/api/hk/report-photos/",
            "/api/hk/report-photosX",
            "/api/hk/report-photos/9/delete",
            "/x/api/hk/report-photos",
            "api/hk/report-photos",
            "/api/housekeeping/report-photos",
            // A collection this matcher does not know must never be exempt,
            // however plausible its leaf looks.
            "/api/hk/notes/7/ack",
            "/api/housekeeping/signals/7/ack",
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
            // ---- the six-segment widening must admit ONE path, not a shape.
            // `resolve` is exempt as the completion of `linen-shortage` and
            // under no other action, on no other collection.
            "/api/hk/rooms/42/cleaning/resolve",
            "/api/hk/rooms/42/signals/resolve",
            "/api/hk/rooms/42/broken-items/resolve",
            "/api/hk/signals/7/linen-shortage/resolve",
            "/api/hk/signals/7/ack/resolve",
            "/api/hk/signals/7/done/resolve",
            // Near-misses and malformed ids on the six-segment shape.
            "/api/hk/rooms/42/linen-shortage/resolveX",
            "/api/hk/rooms/42/linen-shortage/resolve/",      // trailing slash
            "/api/hk/rooms/42/linen-shortage/resolve/extra", // deeper path
            "/api/hk/rooms/42/linen-shortageX/resolve",
            "/api/hk/rooms/42/linen-shortage//resolve",
            "/api/hk/rooms//linen-shortage/resolve",  // empty room id
            "/api/hk/rooms/4a2/linen-shortage/resolve", // non-numeric room id
            "/x/api/hk/rooms/42/linen-shortage/resolve", // prefixed
            "api/hk/rooms/42/linen-shortage/resolve", // no leading slash
            "/api/housekeeping/rooms/42/linen-shortage/resolve",
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
        // …and so is its completion (migration 090), by the same argument at
        // one remove: the UPDATE enqueues nothing and publishes nothing.
        assert!(
            !ville_write_blocked(&Method::POST, LINEN_RESOLVE, VILLE_Q, false),
            "hfville linen-shortage resolve must be admitted while Ville writes are disabled"
        );
        // Room signals (ADR 0008) are PG-only in the strongest sense: iHOTEL
        // has no counterpart at all, so there is not even a writeback recipe
        // that a narrowed HFVILLE_WRITEBACK_INTENTS could park.
        for signal_path in [SIGNALS, SIGNAL_ACK, SIGNAL_DONE, SIGNAL_CANCEL, SIGNAL_ANSWER] {
            assert!(
                !ville_write_blocked(&Method::POST, signal_path, VILLE_Q, false),
                "hfville {signal_path} must be admitted while Ville writes are disabled"
            );
        }
        // Report HK (migration 091) is PG-only in the strongest sense too: the
        // three tables have no legacy counterpart, so there is no recipe a
        // narrowed HFVILLE_WRITEBACK_INTENTS could park. The VERDICTS are
        // included because they are served from the MAID surface's own router
        // by a receptionist standing in the room — a 403 there strands a
        // submitted report unjudged.
        for report_path in [REPORT_SUBMIT, REPORT_VERIFY, REPORT_RETURN, REPORT_PHOTOS] {
            assert!(
                !ville_write_blocked(&Method::POST, report_path, VILLE_Q, false),
                "hfville {report_path} must be admitted while Ville writes are disabled"
            );
        }

        // The DESK half of the same feature is NOT exempt: an exemption exists
        // so a maid's report is not collateral damage of a FRONT-DESK write
        // policy toggle, and a front-desk mutation is what that toggle is for.
        assert!(
            ville_write_blocked(&Method::POST, "/api/housekeeping/signals/7/ack", VILLE_Q, false),
            "the reception signal routes must follow front-desk write policy"
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
        for path in [
            CLEANING,
            LINEN,
            LINEN_RESOLVE,
            SIGNALS,
            SIGNAL_ACK,
            SIGNAL_DONE,
            SIGNAL_CANCEL,
            SIGNAL_ANSWER,
            REPORT_SUBMIT,
            REPORT_VERIFY,
            REPORT_RETURN,
            REPORT_PHOTOS,
        ] {
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
        for path in [
            CLEANING,
            LINEN,
            LINEN_RESOLVE,
            SIGNALS,
            SIGNAL_ACK,
            SIGNAL_DONE,
            SIGNAL_CANCEL,
            SIGNAL_ANSWER,
            REPORT_SUBMIT,
            REPORT_VERIFY,
            REPORT_RETURN,
            REPORT_PHOTOS,
            BROKEN,
            BOOKING,
        ] {
            assert!(!ville_write_blocked(&Method::POST, path, VILLE_Q, true));
        }
    }
}
