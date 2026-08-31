//! `?branch=` is REQUIRED on every `/hk` room endpoint — wave-4 A.
//!
//! ## The bug this pins
//!
//! `?branch=` used to be optional on the maid surface and `None` fell through
//! to `Branch::default()` = HF Hotel. `app/hk/page.tsx` never sent one and its
//! comment claimed "v1 pins the primary property" — it pinned nothing, it
//! omitted the parameter. A HF Ville maid's cleaning report was therefore filed
//! against HF Hotel: canonical `hotelnew` rows, and (once `done` started
//! flipping `room_clean`) an HF Hotel `MarkRoomClean` writeback.
//!
//! Making the parameter mandatory is a BREAKING API change, taken deliberately
//! while the break window is empty: `ht_hk_cleaning_events` held 0 rows at both
//! sites and the แม่บ้าน tile was off the Employee Hub, so there was no live
//! `/hk` client to break. A stale cached bundle now fails loudly with 400
//! instead of silently mutating the wrong hotel.
//!
//! ## Two routers, on purpose
//!
//! The branch gate lives in the HANDLERS, i.e. INSIDE `require_hk_access`, so
//! an unauthenticated caller can never probe branch configuration. That makes
//! the gate's own status codes unobservable through the shipped router — with
//! no `Cf-Access-Jwt-Assertion` header every probe is 401, by design. So:
//!
//! * [`inner`] mounts `routes::hk::routes_inside_access` (the SAME handler
//!   table the shipped router wraps) and injects an `HkIdentity` where the
//!   Access layer would have put one. That is where 400/403 are asserted.
//! * [`shipped`] mounts the real `routes::hk::router` and asserts that EVERY
//!   probe — valid branch, invalid branch, no branch — is 401. That is the
//!   proof the gate sits behind auth and leaks nothing.
//!
//! `tests/test_hk_ville_guard.rs` (unchanged) covers the third layer: the Ville
//! admission guard still admits `POST /api/hk/rooms/{id}/cleaning` and still
//! refuses everything else for `branch=hfville`.
//!
//! ## The reception viewer (2026-09)
//!
//! The suite also owns the read-only `reception` role, because it is a gate in
//! the same handlers and observable only from the same inner router: a viewer
//! reads the board, is `403` on both mutations, and reads `canReport: false` /
//! `markDirtyEnabled: false` from `/api/hk/me`. Every viewer row is DB-free,
//! and each is paired with the equivalent maid row so the two roles cannot
//! drift into each other. An identity holding NEITHER grant never reaches this
//! table at all — `middleware::hk_access` answers 403 first, which is pinned by
//! that module's own unit tests and by `shipped_router_answers_401_*` below.
//!
//! ## Running
//! Rows 1-10 need NO database (the gate answers before any pool is touched;
//! the pool is lazy). The two-site pool-routing proof needs `DATABASE_URL` and
//! `VILLE_DATABASE_URL` and SKIPS when the latter is unset — same convention as
//! `tests/test_ville_write_routing.rs`.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Extension;
use hotel_backend::middleware::hk_access::HkIdentity;
use hotel_backend::routes::hk::{
    HkPolicy, BRANCH_NOT_ENABLED_ERROR, BRANCH_REQUIRED_ERROR, MARK_DIRTY_DISABLED_ERROR,
    REPORT_NOT_PERMITTED_ERROR,
};
use hotel_backend::routes::mode::{AppState, Branch};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

/// A pool that never connects — rows 1-10 are answered before any SQL runs, so
/// this suite does not need a database at all.
fn lazy_pool() -> PgPool {
    PgPool::connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
        .expect("a lazy pool needs no live server")
}

async fn live_pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
    });
    PgPool::connect(&url).await.ok()
}

fn maid() -> HkIdentity {
    HkIdentity {
        badge: "Q1001".to_string(),
        display_name: None,
        email: None,
        // The `housekeeping` grant. Every row in this suite is a MAID row, so
        // the reception-viewer work must not move a single status code here.
        can_report: true,
    }
}

/// The read-only reception viewer (`reception` grant, no `housekeeping`) — the
/// identity the "viewer" rows below inject in the maid's place.
fn viewer() -> HkIdentity {
    HkIdentity {
        can_report: false,
        ..maid()
    }
}

/// Location enforcement is left at its DARK default here on purpose: this
/// suite pins the `HK_BRANCHES` gate, and the gate's status codes must be
/// unchanged by the wave-4 C build. `tests/test_hk_location_enforcement.rs`
/// owns the enforcement-on matrix.
fn policy(branches: Vec<Branch>, mark_dirty_enabled: bool) -> HkPolicy {
    HkPolicy {
        branches,
        mark_dirty_enabled,
        ..HkPolicy::default()
    }
}

/// The handler table with a verified identity injected where the Cloudflare
/// Access layer would have put one.
fn inner(state: AppState, policy: HkPolicy) -> axum::Router {
    hotel_backend::routes::hk::routes_inside_access(state, policy).layer(Extension(maid()))
}

/// Send a request and return `(status, body_json)`.
async fn call(app: axum::Router, method: &str, uri: &str, body: &str) -> (StatusCode, String) {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request builds");
    let response = app.oneshot(req).await.expect("router responds");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body reads");
    (status, String::from_utf8_lossy(&bytes).to_string())
}

/// GET through the inner router with the default (HF-Hotel-only) policy.
async fn get_inner(uri: &str) -> (StatusCode, String) {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    call(app, "GET", uri, "").await
}

fn assert_branch_400(status: StatusCode, body: &str, what: &str) {
    assert_eq!(status, StatusCode::BAD_REQUEST, "{what}: expected 400, body={body}");
    let json: serde_json::Value = serde_json::from_str(body).expect("400 body must be JSON");
    assert_eq!(
        json.get("success").and_then(|v| v.as_bool()),
        Some(false),
        "{what}: the repo-wide envelope requires success=false, body={body}"
    );
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(BRANCH_REQUIRED_ERROR),
        "{what}: stable error message, body={body}"
    );
}

// ============================================================================
// Rows 1-4 — a missing or malformed branch is 400, never a default
// ============================================================================

/// Row 1. The exact shape of the original bug: no `?branch=` at all. It must
/// NOT resolve to HF Hotel.
#[tokio::test]
async fn row1_missing_branch_is_400() {
    let (status, body) = get_inner("/api/hk/rooms").await;
    assert_branch_400(status, &body, "GET /api/hk/rooms with no branch");
}

/// Row 2. An empty value is as unusable as an absent one — a client that
/// serialised `null` must not be luckier than one that sent nothing.
#[tokio::test]
async fn row2_empty_branch_is_400() {
    let (status, body) = get_inner("/api/hk/rooms?branch=").await;
    assert_branch_400(status, &body, "GET /api/hk/rooms?branch=");
}

/// Row 3. **The load-bearing one.** `AppState::write_pool(Some(Branch::All))`
/// returns the PRIMARY pool, so accepting `all` would re-open the identical
/// wrong-hotel bug under a different query string.
#[tokio::test]
async fn row3_branch_all_is_400() {
    let (status, body) = get_inner("/api/hk/rooms?branch=all").await;
    assert_branch_400(status, &body, "GET /api/hk/rooms?branch=all");
}

/// Row 4. No case-fudging: the accepted spellings are exactly `hfhotel` and
/// `hfville`, matching every other branch-aware surface.
#[tokio::test]
async fn row4_wrong_case_branch_is_400() {
    let (status, body) = get_inner("/api/hk/rooms?branch=HFHOTEL").await;
    assert_branch_400(status, &body, "GET /api/hk/rooms?branch=HFHOTEL");
}

// ============================================================================
// Rows 5-7 — a well-formed branch passes the gate; HK_BRANCHES decides which
// ============================================================================

/// Row 5. A valid, enabled branch clears the gate. Asserted negatively (not
/// 400, not 403) because what happens next depends on whether a database is
/// reachable — the point is only that the BRANCH gate did not fire.
#[tokio::test]
async fn row5_valid_branch_passes_the_gate() {
    let (status, body) = get_inner("/api/hk/rooms?branch=hfhotel").await;
    assert_ne!(status, StatusCode::BAD_REQUEST, "branch was valid: {body}");
    assert_ne!(status, StatusCode::FORBIDDEN, "branch was enabled: {body}");
}

/// Row 6. `HK_BRANCHES=hfhotel` (the shipping default) ⇒ HF Ville is a
/// well-formed but UNOFFERED property: 403, not 400. This is what keeps the
/// Ville legacy-key landmine unarmed until `repair_room_legacy_keys --apply`.
#[tokio::test]
async fn row6_hfville_is_403_when_not_in_hk_branches() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(app, "GET", "/api/hk/rooms?branch=hfville", "").await;
    assert_eq!(status, StatusCode::FORBIDDEN, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(BRANCH_NOT_ENABLED_ERROR)
    );
}

/// Row 7. Adding `hfville` to `HK_BRANCHES` admits it — the ONE config change
/// that opens HF Ville, and the reason V13 gates it.
#[tokio::test]
async fn row7_hfville_passes_when_hk_branches_lists_it() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel, Branch::Hfville], false),
    );
    let (status, body) = call(app, "GET", "/api/hk/rooms?branch=hfville", "").await;
    assert_ne!(status, StatusCode::BAD_REQUEST, "body={body}");
    assert_ne!(
        status,
        StatusCode::FORBIDDEN,
        "hfville must be admitted once HK_BRANCHES lists it: {body}"
    );
}

// ============================================================================
// Rows 8-10 — the rule covers mutations and the photo read; `me` is exempt
// ============================================================================

/// Row 8. The mutation is gated too, and the branch is checked BEFORE the body:
/// a request with no branch must never be answered on the strength of its
/// payload. (`tests/test_hk_ville_guard.rs` separately proves the Ville guard
/// still ADMITS this exact path — the gate here is inside, not instead.)
#[tokio::test]
async fn row8_cleaning_post_without_branch_is_400() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/cleaning",
        r#"{"status":"done"}"#,
    )
    .await;
    assert_branch_400(status, &body, "POST cleaning with no branch");
}

/// Row 8b. The linen-shortage report (migration 088) is gated identically to
/// the cleaning report, and the branch is likewise checked BEFORE the body: a
/// request with no branch must never be answered on the strength of its items.
#[tokio::test]
async fn row8b_linen_shortage_post_without_branch_is_400() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/linen-shortage",
        r#"{"items":[{"kind":"bath_towel","qty":2}]}"#,
    )
    .await;
    assert_branch_400(status, &body, "POST linen-shortage with no branch");
}

/// Row 8c. A body so malformed it could not possibly be recorded STILL yields
/// the branch 400 first. This is the ordering half of row 8b: it proves the
/// branch gate is not merely present but ahead of validation, so an
/// unauthorised caller cannot use body errors to probe the surface.
#[tokio::test]
async fn row8c_linen_branch_is_checked_before_the_body() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(app, "POST", "/api/hk/rooms/1/linen-shortage", r#"{}"#).await;
    assert_branch_400(status, &body, "linen POST with neither branch nor items");
}

/// Row 8d. Past the branch gate, a bad body is a 400 in the REPO's envelope —
/// not axum's serde rejection, which the maid's `hkFetch` cannot parse. This is
/// what `items: Option<…>` and the untyped `qty` buy; asserting the envelope
/// (not just the status) is the part that would catch a regression to a
/// required serde field.
#[tokio::test]
async fn linen_body_errors_use_the_repo_envelope() {
    for (payload, needle, what) in [
        (r#"{}"#, "items is required", "items absent"),
        (r#"{"items":[]}"#, "items is required", "items empty"),
        (
            r#"{"items":[{"kind":"blanket","qty":1}]}"#,
            "invalid linen kind",
            "unknown kind",
        ),
        (
            r#"{"items":[{"kind":"bath_towel","qty":2},{"kind":"bath_towel","qty":1}]}"#,
            "duplicate linen kind",
            "duplicate kind",
        ),
        (
            r#"{"items":[{"kind":"bath_towel","qty":0}]}"#,
            "invalid qty",
            "qty 0",
        ),
        (
            r#"{"items":[{"kind":"bath_towel","qty":21}]}"#,
            "invalid qty",
            "qty 21",
        ),
        (
            r#"{"items":[{"kind":"bath_towel","qty":-3}]}"#,
            "invalid qty",
            "negative qty",
        ),
        (
            r#"{"items":[{"kind":"bath_towel","qty":"2"}]}"#,
            "invalid qty",
            "qty as a string",
        ),
        (
            r#"{"items":[{"kind":"bath_towel","qty":1.5}]}"#,
            "invalid qty",
            "fractional qty",
        ),
        (
            r#"{"items":[{"kind":"bed_sheet","qty":1},{"kind":"pillowcase","qty":1},
                        {"kind":"duvet_cover","qty":1},{"kind":"bath_towel","qty":1},
                        {"kind":"face_towel","qty":1},{"kind":"foot_towel","qty":1},
                        {"kind":"pillowcase","qty":1}]}"#,
            "too many linen entries",
            "seven entries",
        ),
    ] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(
            app,
            "POST",
            "/api/hk/rooms/1/linen-shortage?branch=hfhotel",
            payload,
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{what}: body={body}");
        let json: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or_else(|_| panic!("{what}: 400 body must be JSON, got {body}"));
        assert_eq!(
            json.get("success").and_then(|v| v.as_bool()),
            Some(false),
            "{what}: the repo envelope requires success=false, body={body}"
        );
        let error = json
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("{what}: 400 body must carry `error`, got {body}"));
        assert!(
            error.contains(needle),
            "{what}: expected the error to mention '{needle}', got '{error}'"
        );
    }
}

/// Row 9. `GET /api/hk/me` NEVER 400s on a missing branch — it is what tells
/// the client which branches exist, so requiring one would be circular.
#[tokio::test]
async fn row9_me_never_requires_a_branch() {
    let (status, body) = get_inner("/api/hk/me").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("me body must be JSON");
    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(json.get("badge").and_then(|v| v.as_str()), Some("Q1001"));
    // Additive contract fields the picker depends on.
    let branches = json.get("branches").and_then(|v| v.as_array()).expect("branches");
    assert_eq!(branches.len(), 1, "the shipping default offers one branch");
    assert_eq!(branches[0].get("id").and_then(|v| v.as_str()), Some("hfhotel"));
    assert_eq!(
        branches[0].get("labelTh").and_then(|v| v.as_str()),
        Some("ฮาร์เบอร์ฟร้อนท์")
    );
    assert_eq!(
        json.get("markDirtyEnabled").and_then(|v| v.as_bool()),
        Some(false),
        "mark-dirty must ship dark"
    );
}

/// Row 10. The photo read is per-site too — a Ville report's bytes must not be
/// served out of the HF Hotel pool.
#[tokio::test]
async fn row10_broken_item_photo_without_branch_is_400() {
    let (status, body) = get_inner("/api/hk/broken-items/1/photo").await;
    assert_branch_400(status, &body, "GET broken-items photo with no branch");
}

// ============================================================================
// HK_MARK_DIRTY_ENABLED — invariant #6
// ============================================================================

/// `status:"dirty"` is refused while the flag is off, in Thai, with the repo's
/// envelope. The frontend hides the button, so this is the stale-bundle path.
#[tokio::test]
async fn mark_dirty_is_403_while_the_flag_is_off() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/cleaning?branch=hfhotel",
        r#"{"status":"dirty"}"#,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(MARK_DIRTY_DISABLED_ERROR)
    );
}

/// Rejection ORDER: the branch gate fires before the mark-dirty gate, so a
/// request with neither is a 400 about the branch — never a 403 that would tell
/// an unauthorised caller which flags exist.
#[tokio::test]
async fn branch_is_checked_before_the_mark_dirty_flag() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/cleaning",
        r#"{"status":"dirty"}"#,
    )
    .await;
    assert_branch_400(status, &body, "dirty POST with no branch");
}

/// An unknown status is still a 400 from `parse_cleaning_status`, and it is
/// checked AFTER the branch — so the widened status set did not loosen
/// anything.
#[tokio::test]
async fn unknown_status_is_still_400() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], true),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/cleaning?branch=hfhotel",
        r#"{"status":"clean"}"#,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "body={body}");
    assert!(
        body.contains("invalid status"),
        "the status error must survive the branch gate: {body}"
    );
}

// ============================================================================
// The reception viewer — read-only on the SAME surface
// ============================================================================
//
// `reception` opens this surface as a viewer: the desk gets the room board,
// and neither mutation. `canReport` in `/api/hk/me` is UX only — these rows
// are the server-side enforcement behind it, which is what a stale bundle
// hits.
//
// All DB-free: every assertion is answered before a pool is touched (the
// refusals by the capability gate, `me` by the policy alone).

/// The handler table with a READ-ONLY reception identity injected where the
/// Access layer would have put one.
fn inner_viewer(state: AppState, policy: HkPolicy) -> axum::Router {
    hotel_backend::routes::hk::routes_inside_access(state, policy).layer(Extension(viewer()))
}

/// [`lazy_pool`] with the acquire timeout clamped, for the rows that
/// deliberately fall THROUGH to the database. At sqlx's 30 s default the three
/// viewer read endpoints alone cost 90 s of CI per run — the same clamp, and
/// the same reason, as `tests/test_hk_location_enforcement.rs`.
fn clamped_lazy_pool() -> PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(250))
        .connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
        .expect("a lazy pool needs no live server")
}

fn assert_report_403(status: StatusCode, body: &str, what: &str) {
    assert_eq!(status, StatusCode::FORBIDDEN, "{what}: expected 403, body={body}");
    let json: serde_json::Value = serde_json::from_str(body)
        .unwrap_or_else(|_| panic!("{what}: 403 body must be JSON, got {body}"));
    assert_eq!(
        json.get("success").and_then(|v| v.as_bool()),
        Some(false),
        "{what}: the repo-wide envelope requires success=false, body={body}"
    );
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(REPORT_NOT_PERMITTED_ERROR),
        "{what}: stable error message, body={body}"
    );
}

/// **The headline row.** A viewer is refused on BOTH mutations, with the repo
/// envelope and the read-only message — never a 401 (the identity is proven),
/// never a 404, and never a success.
#[tokio::test]
async fn viewer_is_403_on_both_mutations() {
    for (uri, payload, what) in [
        (
            "/api/hk/rooms/1/cleaning?branch=hfhotel",
            r#"{"status":"done"}"#,
            "cleaning: done",
        ),
        (
            "/api/hk/rooms/1/cleaning?branch=hfhotel",
            r#"{"status":"started"}"#,
            "cleaning: started",
        ),
        (
            "/api/hk/rooms/1/cleaning?branch=hfhotel",
            r#"{"status":"dirty"}"#,
            "cleaning: dirty",
        ),
        (
            "/api/hk/rooms/1/linen-shortage?branch=hfhotel",
            r#"{"items":[{"kind":"bath_towel","qty":2}]}"#,
            "linen shortage",
        ),
    ] {
        // `mark_dirty_enabled: true` on purpose: the capability refusal must
        // not depend on a flag that only ever narrows what a MAID may do.
        let app = inner_viewer(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], true),
        );
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_report_403(status, &body, what);
    }
}

/// The capability is checked BEFORE the branch gate and before body
/// VALIDATION: a viewer's branch-less, nonsense-bodied POST is still the
/// capability 403, never a 400. That ordering is what keeps a read-only badge
/// from spending an HF ID location lookup per rejected write — and from using
/// body or branch errors to probe the surface.
///
/// "Before validation", not "before deserialization": axum runs the
/// `Json<…>` extractor ahead of every handler line, so a body that cannot
/// DESERIALIZE is a 422 from axum for a viewer and a maid alike. That is
/// pre-existing (`ReportCleaningBody::status` is a required serde field —
/// which is exactly why row 8c pins the ordering on the linen route, whose
/// `items` is an `Option`), so the shapeless bodies below are on that route.
#[tokio::test]
async fn viewer_capability_is_checked_before_branch_and_body() {
    for (uri, payload, what) in [
        (
            "/api/hk/rooms/1/cleaning",
            r#"{"status":"done"}"#,
            "no branch at all",
        ),
        (
            "/api/hk/rooms/1/cleaning?branch=all",
            r#"{"status":"nonsense"}"#,
            "branch=all, bad status",
        ),
        (
            "/api/hk/rooms/1/linen-shortage",
            r#"{}"#,
            "no branch, and no items either",
        ),
        (
            "/api/hk/rooms/1/linen-shortage?branch=hfville",
            r#"{"items":[{"kind":"blanket","qty":99}]}"#,
            "unoffered branch, invalid items",
        ),
    ] {
        let app = inner_viewer(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], true),
        );
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_report_403(status, &body, what);
    }
}

/// The other half: the READS are unchanged for a viewer. Asserted negatively
/// (not 401/403) because what happens next depends on a database — the point
/// is only that no auth or capability gate fired.
#[tokio::test]
async fn viewer_passes_the_gates_on_every_read() {
    for uri in [
        "/api/hk/rooms?branch=hfhotel",
        "/api/hk/rooms/1?branch=hfhotel",
        "/api/hk/broken-items/1/photo?branch=hfhotel",
    ] {
        let app = inner_viewer(
            AppState::new(clamped_lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "GET", uri, "").await;
        assert_ne!(
            status,
            StatusCode::UNAUTHORIZED,
            "GET {uri}: the viewer identity is verified: {body}"
        );
        assert_ne!(
            status,
            StatusCode::FORBIDDEN,
            "GET {uri}: reads are the whole point of the viewer role: {body}"
        );
    }
}

/// `HK_BRANCHES` still binds for a viewer — the role is read-only, not
/// unbounded. An unoffered property is the same 403 a maid gets.
#[tokio::test]
async fn viewer_is_still_bound_by_hk_branches_on_reads() {
    let app = inner_viewer(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(app, "GET", "/api/hk/rooms?branch=hfville", "").await;
    assert_eq!(status, StatusCode::FORBIDDEN, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(BRANCH_NOT_ENABLED_ERROR),
        "the refusal must be the ALLOWLIST's, not the capability's: {body}"
    );
}

/// `GET /api/hk/me` for a viewer: `canReport: false`, and `markDirtyEnabled`
/// forced to `false` EVEN THOUGH `HK_MARK_DIRTY_ENABLED` is on — a report the
/// identity may not file is a dead tap however the env is set.
#[tokio::test]
async fn me_reports_a_viewer_as_read_only_regardless_of_the_mark_dirty_flag() {
    let app = inner_viewer(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], true),
    );
    let (status, body) = call(app, "GET", "/api/hk/me", "").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("me body must be JSON");
    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(json.get("badge").and_then(|v| v.as_str()), Some("Q1001"));
    assert_eq!(
        json.get("canReport").and_then(|v| v.as_bool()),
        Some(false),
        "the desk's board is read-only: {body}"
    );
    assert_eq!(
        json.get("markDirtyEnabled").and_then(|v| v.as_bool()),
        Some(false),
        "HK_MARK_DIRTY_ENABLED is ON here — the capability must still win: {body}"
    );
    // The viewer still gets a board to look at.
    let branches = json.get("branches").and_then(|v| v.as_array()).expect("branches");
    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].get("id").and_then(|v| v.as_str()), Some("hfhotel"));
    assert!(
        json.get("branchesUnavailableReason")
            .expect("the key is always present")
            .is_null(),
        "nothing is empty, so there is nothing to explain: {body}"
    );
}

/// The maid's side of the same payload, so the two cannot drift: `canReport`
/// is `true` and `markDirtyEnabled` still tracks the env exactly as before.
#[tokio::test]
async fn me_reports_a_maid_as_able_to_report() {
    for mark_dirty in [false, true] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], mark_dirty),
        );
        let (status, body) = call(app, "GET", "/api/hk/me", "").await;
        assert_eq!(status, StatusCode::OK, "body={body}");
        let json: serde_json::Value = serde_json::from_str(&body).expect("me body must be JSON");
        assert_eq!(
            json.get("canReport").and_then(|v| v.as_bool()),
            Some(true),
            "a housekeeping badge keeps the full capability: {body}"
        );
        assert_eq!(
            json.get("markDirtyEnabled").and_then(|v| v.as_bool()),
            Some(mark_dirty),
            "for a maid the flag is still the ONLY input: {body}"
        );
    }
}

/// A maid is untouched by the capability gate: `dirty` with the flag ON still
/// reaches the room lookup, and `dirty` with the flag OFF is still the
/// mark-dirty 403 — never the read-only one.
#[tokio::test]
async fn the_capability_gate_is_invisible_to_maids() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/cleaning?branch=hfhotel",
        r#"{"status":"dirty"}"#,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(MARK_DIRTY_DISABLED_ERROR),
        "a maid's refusal must still name the FLAG, not the capability: {body}"
    );
}

// ============================================================================
// The shipped stack leaks nothing before auth
// ============================================================================

/// Through the REAL router every probe is 401 — valid branch, invalid branch,
/// no branch, `me`. That is the proof the branch gate sits INSIDE
/// `require_hk_access`: an unauthenticated caller cannot use the difference
/// between 400 and 403 to enumerate which properties this deployment serves.
#[tokio::test]
async fn shipped_router_answers_401_before_the_branch_gate() {
    let Some(pool) = live_pool().await else {
        eprintln!("skipping shipped_router_answers_401 — PG not reachable");
        return;
    };
    for (method, uri, body) in [
        ("GET", "/api/hk/rooms", ""),
        ("GET", "/api/hk/rooms?branch=", ""),
        ("GET", "/api/hk/rooms?branch=all", ""),
        ("GET", "/api/hk/rooms?branch=hfhotel", ""),
        ("GET", "/api/hk/rooms?branch=hfville", ""),
        ("GET", "/api/hk/me", ""),
        ("GET", "/api/hk/broken-items/1/photo", ""),
        (
            "POST",
            "/api/hk/rooms/1/cleaning?branch=hfhotel",
            r#"{"status":"dirty"}"#,
        ),
        (
            "POST",
            "/api/hk/rooms/1/linen-shortage?branch=hfhotel",
            r#"{"items":[{"kind":"bath_towel","qty":2}]}"#,
        ),
    ] {
        let app = hotel_backend::routes::hk::router(
            AppState::new(pool.clone()).with_hfville_writes(true),
        );
        let (status, got) = call(app, method, uri, body).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "{method} {uri} must be refused by the Access gate before any branch \
             or flag logic can answer; got {status} {got}"
        );
    }
}

/// The same proof for the linen-shortage route, WITHOUT needing a database.
///
/// `router()` answers 401 from `require_hk_access` before any handler can
/// resolve a pool, so a never-connecting lazy pool is sufficient — which means
/// this one runs everywhere, including a CI job with no PG. A new mutation that
/// forgot the Access layer would be a silently unauthenticated write endpoint,
/// so this assertion should never be gated behind an optional dependency.
///
/// Every valid body and branch combination is exercised: the refusal must come
/// from auth, never from the branch gate or body validation.
#[tokio::test]
async fn shipped_router_answers_401_for_linen_shortage_without_a_db() {
    for uri in [
        "/api/hk/rooms/1/linen-shortage",
        "/api/hk/rooms/1/linen-shortage?branch=hfhotel",
        "/api/hk/rooms/1/linen-shortage?branch=hfville",
        "/api/hk/rooms/1/linen-shortage?branch=all",
    ] {
        let app =
            hotel_backend::routes::hk::router(AppState::new(lazy_pool()).with_hfville_writes(true));
        let (status, got) = call(
            app,
            "POST",
            uri,
            r#"{"items":[{"kind":"bath_towel","qty":2}]}"#,
        )
        .await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "POST {uri} must be refused by the Access gate before any branch, body \
             or pool logic can answer; got {status} {got}"
        );
    }
}

// ============================================================================
// The test that would have caught the original bug: two-site pool routing
// ============================================================================

/// `?branch=hfhotel` must read the `hotelnew` pool and `?branch=hfville` the
/// `hotelville` pool — proven with marker rooms that exist in exactly ONE
/// database each. A regression that resolved both branches to the primary pool
/// would list `ZT-HKB1` under both.
///
/// Skips when `VILLE_DATABASE_URL` is unset (same convention as
/// `tests/test_ville_write_routing.rs`).
#[tokio::test]
async fn branch_selects_the_right_site_pool() {
    let Ok(ville_url) = std::env::var("VILLE_DATABASE_URL") else {
        eprintln!("skipping branch_selects_the_right_site_pool — VILLE_DATABASE_URL unset");
        return;
    };
    let Some(hf_pool) = live_pool().await else {
        eprintln!("skipping branch_selects_the_right_site_pool — PG not reachable");
        return;
    };
    let ville_pool = PgPool::connect(&ville_url).await.expect("connect hotelville");

    // Clean both sides, then seed ONE marker per database.
    for pool in [&hf_pool, &ville_pool] {
        for marker in ["ZT-HKB1", "ZT-HKB2"] {
            let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
                .bind(marker)
                .execute(pool)
                .await;
        }
    }
    sqlx::query("INSERT INTO ht_rooms_new (room_no, room_clean, room_active) VALUES ('ZT-HKB1', true, true)")
        .execute(&hf_pool)
        .await
        .expect("seed the HF Hotel marker");
    sqlx::query("INSERT INTO ht_rooms_new (room_no, room_clean, room_active) VALUES ('ZT-HKB2', true, true)")
        .execute(&ville_pool)
        .await
        .expect("seed the HF Ville marker");

    let state = AppState::new(hf_pool.clone())
        .with_ville(ville_pool.clone())
        .with_hfville_writes(true);

    for (branch, expected, forbidden) in [
        ("hfhotel", "ZT-HKB1", "ZT-HKB2"),
        ("hfville", "ZT-HKB2", "ZT-HKB1"),
    ] {
        let app = inner(
            state.clone(),
            policy(vec![Branch::Hfhotel, Branch::Hfville], false),
        );
        let (status, body) = call(app, "GET", &format!("/api/hk/rooms?branch={branch}"), "").await;
        assert_eq!(status, StatusCode::OK, "branch={branch} body={body}");
        assert!(
            body.contains(expected),
            "branch={branch} must list {expected} (its own site's marker): {body}"
        );
        assert!(
            !body.contains(forbidden),
            "branch={branch} must NOT list {forbidden} — that is the other site's \
             marker, and seeing it means both branches resolved to one pool"
        );
    }

    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-HKB1'")
        .execute(&hf_pool)
        .await;
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-HKB2'")
        .execute(&ville_pool)
        .await;
}
