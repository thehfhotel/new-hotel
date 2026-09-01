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
    REPORT_NOT_PERMITTED_ERROR, VERIFY_NOT_PERMITTED_ERROR,
};
use hotel_backend::routes::mode::{AppState, Branch};
use sqlx::{PgPool, Row};
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

/// Send a `multipart/form-data` request and return `(status, body_json)`.
///
/// `POST /api/hk/report-photos` takes its image as multipart, and axum's
/// `Multipart` extractor rejects a body whose content type is not multipart
/// BEFORE the handler runs — so a JSON-shaped probe would assert axum's
/// rejection rather than this module's gates. This helper sends a real
/// one-part body so the branch and role gates are what answer.
async fn call_multipart(app: axum::Router, uri: &str, part: Option<(&str, &str)>) -> (StatusCode, String) {
    const BOUNDARY: &str = "ZZTESTBOUNDARY";
    let body = match part {
        Some((field, bytes)) => format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{field}\"; \
             filename=\"p.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n{bytes}\r\n--{BOUNDARY}--\r\n"
        ),
        None => format!("--{BOUNDARY}--\r\n"),
    };
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", format!("multipart/form-data; boundary={BOUNDARY}"))
        .body(Body::from(body))
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

/// Row 8e. The linen RESOLVE route (migration 090, เติมผ้าแล้ว) answers the
/// SAME required-branch gate as the report it completes. It takes no body at
/// all, so this is the whole pre-pool surface: without `?branch=` it must be
/// the stable 400, never a default to HF Hotel.
#[tokio::test]
async fn row8e_linen_resolve_post_without_branch_is_400() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/linen-shortage/resolve",
        "",
    )
    .await;
    assert_branch_400(status, &body, "POST linen-shortage/resolve with no branch");
}

/// Row 8f. …and a branch this deployment does not offer is refused too, on the
/// same route, before any pool is resolved.
#[tokio::test]
async fn row8f_linen_resolve_with_an_unoffered_branch_is_403() {
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/linen-shortage/resolve?branch=hfville",
        "",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "an unoffered branch must be 403 on the resolve route too, body={body}"
    );
    let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(BRANCH_NOT_ENABLED_ERROR),
        "stable error message, body={body}"
    );
}

/// Row 8g. A MAID with a valid branch passes every pure gate on the resolve
/// route and falls through to the database.
///
/// Asserted NEGATIVELY (the pool never connects, so what comes back is a
/// server-side failure) because the point is only that no gate fired: a 400,
/// 401, 403 or 404 here would mean the branch gate, the Access layer, the
/// capability check or the routing table refused a legitimate เติมผ้าแล้ว. This
/// is the row that would catch the route simply not being mounted.
#[tokio::test]
async fn row8g_a_maid_reaches_the_database_on_the_resolve_route() {
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/linen-shortage/resolve?branch=hfhotel",
        "",
    )
    .await;
    assert!(
        !matches!(
            status,
            StatusCode::BAD_REQUEST
                | StatusCode::UNAUTHORIZED
                | StatusCode::FORBIDDEN
                | StatusCode::NOT_FOUND
                | StatusCode::METHOD_NOT_ALLOWED
        ),
        "a maid's เติมผ้าแล้ว must pass every gate and reach the pool; got {status} {body}"
    );
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
        // Migration 090. Deliberately NOT the room-signals rule (where
        // `can_report` picks a SIDE and both sides act): completing a shortage
        // is a maid's physical act, so the viewer sees the backlog and cannot
        // close it.
        (
            "/api/hk/rooms/1/linen-shortage/resolve?branch=hfhotel",
            "",
            "linen shortage: resolve",
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
        // The resolve route has no body to be wrong, so the capability check
        // is pinned against the branch gate alone: no branch, and a branch this
        // deployment does not offer, are both still the capability 403.
        (
            "/api/hk/rooms/1/linen-shortage/resolve",
            "",
            "resolve with no branch at all",
        ),
        (
            "/api/hk/rooms/1/linen-shortage/resolve?branch=hfville",
            "",
            "resolve on an unoffered branch",
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

/// The same proof for the linen RESOLVE route (migration 090), WITHOUT needing
/// a database.
///
/// A new mutation that forgot the Access layer would be a silently
/// unauthenticated write endpoint — and this one is additionally the first
/// SIX-segment write on the surface, so it also proves the Ville guard ADMITS
/// it (401 from auth) rather than short-circuiting (403) even on `hfville`,
/// through the shipped stack that `main.rs` mounts.
#[tokio::test]
async fn shipped_router_answers_401_for_linen_resolve_without_a_db() {
    for uri in [
        "/api/hk/rooms/1/linen-shortage/resolve",
        "/api/hk/rooms/1/linen-shortage/resolve?branch=hfhotel",
        "/api/hk/rooms/1/linen-shortage/resolve?branch=hfville",
        "/api/hk/rooms/1/linen-shortage/resolve?branch=all",
    ] {
        let app =
            hotel_backend::routes::hk::router(AppState::new(lazy_pool()).with_hfville_writes(true));
        let (status, got) = call(app, "POST", uri, "").await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "POST {uri} must be refused by the Access gate before any branch or \
             pool logic can answer; got {status} {got}"
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

// ============================================================================
// Room signals (ADR 0008) — the role gate, the room_check carve-out, and the
// same branch gate every other room endpoint answers
// ============================================================================
//
// All DB-free: every assertion below is answered by a pure gate before a pool
// is touched. The DB-backed paths (a real raise → ack → done round trip, the
// เสร็จแล้ว auto-complete, the answer's child signals) belong to the live-PG
// suite the orchestrator runs afterwards; what is pinned HERE is the set of
// refusals a stale bundle or a hand-rolled request can provoke.
//
// The role gate is the interesting one and it is SYMMETRIC: `can_report`
// decides which SIDE an identity is on, not whether it may write at all. So —
// unlike the cleaning and linen mutations — a viewer is NOT refused outright on
// `POST /signals`; it is refused for the maid's TYPES, and the maid is refused
// for the desk's. Pinning both halves is what stops one role's rules being
// quietly applied to the other.

/// The signal type an identity of the OTHER side would send.
const A_MAID_TYPE: &str = "item_missing";
const A_DESK_TYPE: &str = "priority_clean";

fn assert_forbidden(status: StatusCode, body: &str, what: &str) {
    assert_eq!(status, StatusCode::FORBIDDEN, "{what}: expected 403, body={body}");
    let json: serde_json::Value = serde_json::from_str(body)
        .unwrap_or_else(|_| panic!("{what}: 403 body must be JSON, got {body}"));
    assert_eq!(
        json.get("success").and_then(|v| v.as_bool()),
        Some(false),
        "{what}: the repo-wide envelope requires success=false, body={body}"
    );
    assert!(
        json.get("error").and_then(|v| v.as_str()).is_some_and(|e| !e.is_empty()),
        "{what}: a 403 must carry an actionable message, body={body}"
    );
}

fn assert_bad_request(status: StatusCode, body: &str, what: &str) {
    assert_eq!(status, StatusCode::BAD_REQUEST, "{what}: expected 400, body={body}");
    let json: serde_json::Value = serde_json::from_str(body)
        .unwrap_or_else(|_| panic!("{what}: 400 body must be JSON, got {body}"));
    assert_eq!(
        json.get("success").and_then(|v| v.as_bool()),
        Some(false),
        "{what}: the repo-wide envelope requires success=false, body={body}"
    );
}

/// A maid may not speak for reception. `priority_clean` is a REAL, correctly
/// spelled type — it just belongs to the other direction — so this must be a
/// permission refusal (403), not a "bad body" (400): the client sent something
/// valid that this identity may not say.
#[tokio::test]
async fn signals_a_maid_cannot_send_a_desk_to_maid_type() {
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/signals?branch=hfhotel",
        &format!(r#"{{"type":"{A_DESK_TYPE}"}}"#),
    )
    .await;
    assert_forbidden(status, &body, "maid POSTing a desk→maid type");
}

/// …and the mirror image: the read-only reception viewer IS the desk on this
/// surface, so it may not raise a maid→desk signal either.
#[tokio::test]
async fn signals_a_viewer_cannot_send_a_maid_to_desk_type() {
    let app = inner_viewer(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/signals?branch=hfhotel",
        &format!(r#"{{"type":"{A_MAID_TYPE}"}}"#),
    )
    .await;
    assert_forbidden(status, &body, "viewer POSTing a maid→desk type");
}

/// A code in NEITHER vocabulary is a malformed request, not an authorization
/// problem — otherwise a client typo reads to an operator as a permissions bug.
#[tokio::test]
async fn signals_an_unknown_type_is_400_not_403() {
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/signals?branch=hfhotel",
        r#"{"type":"gossip"}"#,
    )
    .await;
    assert_bad_request(status, &body, "POST /signals with an unknown type");
}

/// **The checkout carve-out.** ขอเช็คห้อง can never be completed by a bare done
/// tap — its completion is the maid's judgement — and the refusal must NAME the
/// answer endpoint so a client can route the user there instead of retrying.
///
/// The signal id is fabricated, and that is the point: the type-level refusal
/// is produced by the transition table AFTER the row is read, so this row
/// requires a database. It runs against the live pool when one is reachable and
/// skips otherwise, like every other DB-backed row in this suite.
#[tokio::test]
async fn signals_done_on_a_room_check_points_at_the_answer_endpoint() {
    let Some(pool) = live_pool().await else {
        eprintln!("skipping signals_done_on_a_room_check — PG not reachable");
        return;
    };
    // Seed one desk→maid room_check against any active room.
    let Some(room_id) = sqlx::query_scalar::<_, i32>(
        "SELECT room_id FROM ht_rooms_new WHERE COALESCE(room_active, true) LIMIT 1",
    )
    .fetch_optional(&pool)
    .await
    .expect("room probe")
    else {
        eprintln!("skipping signals_done_on_a_room_check — no rooms seeded");
        return;
    };
    let signal_id: i64 = sqlx::query_scalar(
        "INSERT INTO ht_hk_room_signals \
             (sig_room_id, sig_direction, sig_type, sig_created_badge) \
         VALUES ($1, 'desk_to_maid', 'room_check', 'TEST') RETURNING sig_id",
    )
    .bind(room_id)
    .fetch_one(&pool)
    .await
    .expect("seed a room_check");

    let app = inner(
        AppState::new(pool.clone()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        &format!("/api/hk/signals/{signal_id}/done?branch=hfhotel"),
        "",
    )
    .await;
    assert_bad_request(status, &body, "done tap on a room_check");
    assert!(
        body.contains("answer"),
        "the refusal must point at the answer endpoint, got {body}"
    );

    sqlx::query("DELETE FROM ht_hk_room_signals WHERE sig_id = $1")
        .bind(signal_id)
        .execute(&pool)
        .await
        .expect("cleanup");
}

/// Every signal endpoint answers the SAME branch gate as the room endpoints —
/// a stale bundle that forgot `?branch=` fails loudly instead of silently
/// acting on HF Hotel.
#[tokio::test]
async fn signals_endpoints_all_require_a_branch() {
    for (method, uri, body) in [
        ("GET", "/api/hk/signals", ""),
        ("GET", "/api/hk/events", ""),
        ("POST", "/api/hk/rooms/1/signals", r#"{"type":"item_missing"}"#),
        ("POST", "/api/hk/signals/1/ack", ""),
        ("POST", "/api/hk/signals/1/done", ""),
        ("POST", "/api/hk/signals/1/cancel", ""),
        ("POST", "/api/hk/signals/1/answer", r#"{"outcome":"clear"}"#),
    ] {
        let app = inner(
            AppState::new(clamped_lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, got) = call(app, method, uri, body).await;
        assert_branch_400(status, &got, &format!("{method} {uri} with no branch"));
    }
}

/// `branch=all` must be refused here too. `write_pool(Some(All))` returns the
/// PRIMARY pool, so accepting it would let a Ville signal land in HF Hotel's
/// database — the identical wrong-hotel bug row 3 pins for the room list.
#[tokio::test]
async fn signals_reject_branch_all() {
    for (method, uri, body) in [
        ("GET", "/api/hk/signals?branch=all", ""),
        ("GET", "/api/hk/events?branch=all", ""),
        (
            "POST",
            "/api/hk/rooms/1/signals?branch=all",
            r#"{"type":"item_missing"}"#,
        ),
    ] {
        let app = inner(
            AppState::new(clamped_lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, got) = call(app, method, uri, body).await;
        assert_branch_400(status, &got, &format!("{method} {uri}"));
    }
}

/// A branch outside `HK_BRANCHES` is 403 on the signal endpoints too — the
/// deployment allowlist binds the maid stream exactly as it binds the reads.
#[tokio::test]
async fn signals_respect_the_hk_branches_allowlist() {
    for uri in [
        "/api/hk/signals?branch=hfville",
        "/api/hk/events?branch=hfville",
    ] {
        let app = inner(
            AppState::new(clamped_lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "GET", uri, "").await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "GET {uri} must be 403 when HK_BRANCHES omits hfville, body={body}"
        );
        assert!(
            body.contains(BRANCH_NOT_ENABLED_ERROR),
            "GET {uri}: stable error message, body={body}"
        );
    }
}

/// **Branch isolation on the stream.** A `hfville` request with no Ville
/// fan-out wired must be `503`, NEVER a silent fall back to HF Hotel's signal
/// stream. Reception's `/api/events` degrades that way on purpose; a Ville
/// maid's phone may not, because the fallback would show her another
/// property's room numbers, maids and guest-accountability notices.
#[tokio::test]
async fn the_maid_stream_503s_rather_than_serving_the_other_branch() {
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel, Branch::Hfville], false),
    );
    let (status, body) = call(app, "GET", "/api/hk/events?branch=hfville", "").await;
    assert_eq!(
        status,
        StatusCode::SERVICE_UNAVAILABLE,
        "an unwired Ville fan-out must answer 503, not HF Hotel's stream; body={body}"
    );
}

/// The branch gate runs BEFORE the body is judged, so a request with no branch
/// is never answered on the strength of its payload — the same ordering the
/// cleaning and linen routes pin.
#[tokio::test]
async fn signals_check_the_branch_before_the_body() {
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/signals",
        r#"{"type":"totally-not-a-signal"}"#,
    )
    .await;
    assert_branch_400(status, &body, "POST /signals, no branch, bad type");
}

/// An unrecognised `outcome` on the answer endpoint is a 400 with the repo
/// envelope — not a serde rejection in a foreign shape the maid's client
/// cannot parse.
#[tokio::test]
async fn the_answer_outcome_is_validated_with_the_repo_envelope() {
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/signals/1/answer?branch=hfhotel",
        r#"{"outcome":"maybe"}"#,
    )
    .await;
    assert_bad_request(status, &body, "answer with an unknown outcome");
    assert!(
        body.contains("clear") && body.contains("problems"),
        "the message must name the accepted outcomes, got {body}"
    );
}

/// Through the SHIPPED router every signal endpoint is 401 without an Access
/// assertion. A new mutation that forgot the auth layer would be a silently
/// unauthenticated write endpoint, so this runs with a never-connecting pool
/// and therefore everywhere — including CI with no PG.
#[tokio::test]
async fn shipped_router_answers_401_for_every_signal_endpoint() {
    for (method, uri, body) in [
        ("GET", "/api/hk/signals?branch=hfhotel", ""),
        ("GET", "/api/hk/events?branch=hfhotel", ""),
        (
            "POST",
            "/api/hk/rooms/1/signals?branch=hfhotel",
            r#"{"type":"item_missing"}"#,
        ),
        ("POST", "/api/hk/signals/1/ack?branch=hfhotel", ""),
        ("POST", "/api/hk/signals/1/done?branch=hfhotel", ""),
        ("POST", "/api/hk/signals/1/cancel?branch=hfhotel", ""),
        (
            "POST",
            "/api/hk/signals/1/answer?branch=hfhotel",
            r#"{"outcome":"clear"}"#,
        ),
    ] {
        let app = hotel_backend::routes::hk::router(
            AppState::new(clamped_lazy_pool()).with_hfville_writes(true),
        );
        let (status, got) = call(app, method, uri, body).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "{method} {uri} must be refused by the Access gate before any signal \
             logic can answer; got {status} {got}"
        );
    }
}

// ============================================================================
// The DESK read — `answeredRoomChecks` (the documented v1 gap, ADR 0008)
// ============================================================================
//
// `GET /api/housekeeping/signals` is reception's twin of `GET /api/hk/signals`
// and lives in this suite for the same reason the `/hk` signal rows do: it is
// the other half of one feature, and the two must not drift.
//
// The gap being closed (`components/v2/signals/RoomCheckPanel`'s header): the
// live list is `open` + `acked` by contract, so a maid's เคลียร์ answer leaves
// NOTHING behind — the panel could only infer it from a transition its own tab
// watched, and a tab reload showed "not requested" for a room already cleared.
// The desk response now carries a THIRD field: the newest ANSWERED room_check
// per room for the Bangkok civil day.
//
// The maid tree is deliberately untouched — `GET /api/hk/signals` keeps its
// two-field body, which `shipped_router_answers_401_for_every_signal_endpoint`
// and the `/hk` rows above still describe.

/// The desk handler mounted on its shipped path. Reception's surface has no
/// Access layer (its auth is the cookie session in `main.rs`, a no-op while
/// `AUTH_ENABLED=false`), so unlike [`inner`] there is no identity to inject —
/// the role is constantly `SignalRole::Desk` inside the handler.
fn desk(state: AppState) -> axum::Router {
    axum::Router::new()
        .route(
            "/api/housekeeping/signals",
            axum::routing::get(hotel_backend::routes::housekeeping::list_signals),
        )
        .with_state(state)
}

/// The field is ADDITIVE and ALWAYS serialized, in the agreed camelCase
/// spelling — DB-free, so it runs everywhere.
///
/// The VALUE is what the client branches on: `RoomCheckPanel` reads an ABSENT
/// key as an older backend and falls back to its module-memory inference, so a
/// build that emitted no key on a quiet morning would silently ship the bug
/// this change fixes while every other test still passed.
#[test]
fn the_desk_envelope_always_carries_answered_room_checks() {
    let body = serde_json::to_string(
        &hotel_backend::routes::housekeeping::DeskSignalListResponse {
            success: true,
            signals: vec![],
            answered_room_checks: vec![],
        },
    )
    .expect("the desk envelope serializes");
    assert!(body.contains(r#""answeredRoomChecks":[]"#), "{body}");
    assert!(body.contains(r#""signals":[]"#), "{body}");
    assert!(body.contains(r#""success":true"#), "{body}");
}

/// The answered read rides the SAME per-site chokepoint as the live list.
///
/// With no `hotelville` pool wired, `?branch=hfville` must FAIL rather than
/// answer from the HF Hotel pool — otherwise reception's checkout screen could
/// show a Ville room as เคลียร์ on the strength of an HF Hotel maid's answer,
/// which is the two-site bug this whole suite exists to pin. DB-free: the
/// chokepoint answers before any pool is touched.
#[tokio::test]
async fn the_desk_answered_read_is_branch_routed_not_defaulted() {
    let app = desk(AppState::new(clamped_lazy_pool()));
    let (status, body) = call(app, "GET", "/api/housekeeping/signals?branch=hfville", "").await;
    assert_eq!(
        status,
        StatusCode::INTERNAL_SERVER_ERROR,
        "an unwired HF Ville must be an error, never the primary site's rows: {body}"
    );
    assert_ne!(status, StatusCode::OK, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body must be JSON");
    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(false));
    assert!(
        json.get("answeredRoomChecks").is_none(),
        "a refused branch carries no signals at all: {body}"
    );
}

/// The live path, end to end against PG: today's answered ขอเช็คห้อง comes back
/// on the new field, newest-per-room, while the cancelled one and the still-open
/// one do not. Skips gracefully without a local database, same convention as
/// [`branch_selects_the_right_site_pool`].
#[tokio::test]
async fn desk_signals_serve_todays_newest_answered_room_check_per_room() {
    let Some(pool) = live_pool().await else {
        eprintln!("skipping desk_signals_serve_todays_newest_answered_room_check — PG not reachable");
        return;
    };

    // Marker rooms, torn down first in case a previous run died mid-test.
    // Deleting the room CASCADEs its signals (migration 089's FK).
    for marker in ["ZT-HKA1", "ZT-HKA2"] {
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
            .bind(marker)
            .execute(&pool)
            .await;
    }
    let mut rooms = Vec::new();
    for marker in ["ZT-HKA1", "ZT-HKA2"] {
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ($1, true, true) RETURNING room_id",
        )
        .bind(marker)
        .fetch_one(&pool)
        .await
        .expect("seed marker room");
        rooms.push(room_id);
    }
    let (room_a, room_b) = (rooms[0], rooms[1]);

    // Seeded directly rather than through the service: this row is about what
    // the READ serves, and the write paths are pinned by their own suites.
    //
    // `sig_done_at` is built as an offset from TODAY'S BANGKOK MIDNIGHT, not
    // from `NOW()`: the fixture must land inside the same civil day the query
    // filters on no matter which UTC hour CI runs in — which is the very
    // property `TODAY_BKK_SIGNAL_DONE` exists to get right.
    let seed = |room_id: i32,
                status: &'static str,
                outcome: Option<&'static str>,
                done_source: Option<&'static str>,
                since_bkk_midnight: Option<&'static str>| {
        let pool = pool.clone();
        async move {
            let id: i64 = sqlx::query_scalar(
                "INSERT INTO ht_hk_room_signals \
                     (sig_room_id, sig_direction, sig_type, sig_status, sig_outcome, \
                      sig_created_badge, sig_done_badge, sig_done_source, sig_done_at) \
                 VALUES ($1, 'desk_to_maid', 'room_check', $2, $3, 'Front Desk', \
                         CASE WHEN $4::text IS NULL THEN NULL ELSE 'Q1001' END, $4, \
                         CASE WHEN $5::text IS NULL THEN NULL ELSE \
                              ((date_trunc('day', NOW() AT TIME ZONE 'Asia/Bangkok') \
                                + $5::interval) AT TIME ZONE 'Asia/Bangkok') END) \
                 RETURNING sig_id",
            )
            .bind(room_id)
            .bind(status)
            .bind(outcome)
            .bind(done_source)
            .bind(since_bkk_midnight)
            .fetch_one(&pool)
            .await
            .expect("seed signal");
            id
        }
    };

    // Room A: an early เคลียร์, then a later มีของหาย answer today — only the
    // LATER one may appear (one entry per room, newest wins).
    let early = seed(
        room_a,
        "done",
        Some("clear"),
        Some("room_check_answer"),
        Some("1 hour"),
    )
    .await;
    let latest_a = seed(
        room_a,
        "done",
        Some("problems"),
        Some("room_check_answer"),
        Some("9 hours"),
    )
    .await;
    // Room A also has a CANCELLED check — a withdrawn ขอเช็คห้อง must never
    // read as an answer.
    let cancelled = seed(room_a, "cancelled", None, None, None).await;
    // Room B: a เคลียร์ answered today, plus a still-open check that belongs on
    // the LIVE list only.
    let latest_b = seed(
        room_b,
        "done",
        Some("clear"),
        Some("room_check_answer"),
        Some("8 hours"),
    )
    .await;
    let open_b = seed(room_b, "open", None, None, None).await;

    let app = desk(AppState::new(pool.clone()));
    let (status, body) = call(app, "GET", "/api/housekeeping/signals?branch=hfhotel", "").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body must be JSON");
    let answered = json["answeredRoomChecks"]
        .as_array()
        .unwrap_or_else(|| panic!("answeredRoomChecks must be an array: {body}"));

    let ids: Vec<i64> = answered
        .iter()
        .filter_map(|s| s["signalId"].as_i64())
        .collect();
    assert!(
        ids.contains(&latest_a) && ids.contains(&latest_b),
        "today's answered checks for both marker rooms must appear: {body}"
    );
    assert!(
        !ids.contains(&early),
        "one entry per room: the EARLIER answer must be superseded, got {ids:?}"
    );
    assert!(
        !ids.contains(&cancelled),
        "a cancelled check must never be served as an answer: {ids:?}"
    );
    assert!(
        !ids.contains(&open_b),
        "a still-open check belongs to `signals`, not `answeredRoomChecks`: {ids:?}"
    );

    // The DTO carries the ANSWER — the fact the panel could not recover.
    let entry = answered
        .iter()
        .find(|s| s["signalId"].as_i64() == Some(latest_b))
        .expect("room B's answered check");
    assert_eq!(entry["outcome"], "clear");
    assert_eq!(entry["status"], "done");
    assert_eq!(entry["doneSource"], "room_check_answer");
    assert_eq!(entry["roomNo"], "ZT-HKA2");
    assert_eq!(entry["doneBy"]["badge"], "Q1001");
    assert!(entry["doneAt"].is_string(), "doneAt must be populated: {entry}");

    // Ordering is doneAt DESCENDING across rooms — 09:00 (room A) before 08:00.
    let marker_order: Vec<i64> = ids
        .iter()
        .copied()
        .filter(|id| *id == latest_a || *id == latest_b)
        .collect();
    assert_eq!(
        marker_order,
        vec![latest_a, latest_b],
        "newest answer first: {body}"
    );

    // …and the live list is untouched: the open check is still there and no
    // terminal row leaked into it.
    let live: Vec<i64> = json["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .filter_map(|s| s["signalId"].as_i64())
        .collect();
    assert!(live.contains(&open_b), "the open check stays on the board: {body}");
    for terminal in [latest_a, latest_b, cancelled, early] {
        assert!(
            !live.contains(&terminal),
            "signal {terminal} is terminal and must not be on the live board: {body}"
        );
    }

    for marker in ["ZT-HKA1", "ZT-HKA2"] {
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
            .bind(marker)
            .execute(&pool)
            .await;
    }
}

// ============================================================================
// Report HK — migration 091
// ============================================================================
//
// The seven endpoints of the room report, gated by the SAME `?branch=` rule as
// everything else on this surface plus a role rule that runs in BOTH
// directions: a viewer may not SUBMIT and a maid may not JUDGE. Those two are
// different refusals with different messages, and the pairs below keep them
// from collapsing into one.
//
// All DB-free unless a row says otherwise: every assertion here is answered
// before a pool is touched.

/// The three JSON WRITE endpoints, with a body each handler will accept if it
/// ever gets that far. Used by the branch rows, where the point is that the
/// gate answers BEFORE the body is judged.
///
/// `POST /api/hk/report-photos` is deliberately NOT here: it is multipart, and
/// axum's `Multipart` extractor answers a non-multipart body before the handler
/// runs, so it needs [`call_multipart`] rather than [`call`]. Its own rows are
/// below.
const REPORT_WRITES: [(&str, &str); 3] = [
    ("/api/hk/rooms/1/report", V2_SUBMIT_BODY),
    ("/api/hk/reports/9/verify", r#"{"photoIds":[1]}"#),
    ("/api/hk/reports/9/return", r#"{"reason":"not_clean"}"#),
];

/// The three READ endpoints (`/meta` added by migration 092).
const REPORT_READS: [&str; 2] = ["/api/hk/reports", "/api/hk/reports/9"];

/// The 22 checklist items, in `report-vocab.ts` order — the ONLY accepted tick
/// set. Duplicated here rather than imported so this suite pins the WIRE
/// contract: if the backend's vocabulary moves without the frontend's, these
/// bodies stop being accepted and this file says so.
const REPORT_ITEM_CODES: [&str; 22] = [
    "water_glass",
    "coffee_tray",
    "coffee_cup",
    "coffee_sachet_jar",
    "kettle",
    "bathroom_bin",
    "hairdryer",
    "bath_amenity_tray",
    "aircon_remote",
    "tv_remote",
    "mirror_bin",
    "hangers",
    "bath_towel",
    "face_towel",
    "foot_towel",
    "duvet",
    "bed_sheet",
    "pillowcase",
    "duvet_cover",
    "pillow",
    "ashtray",
    "bathrobe",
];

/// A COMPLETE v2 submission: 22 `ok` ticks over four photos (one per capture
/// zone), which is what a perfect room produces.
///
/// Built rather than spelled so the branch/role rows stay readable; the rows
/// that are ABOUT the body shape spell their own.
fn v2_submit_body() -> String {
    let ticks: Vec<String> = REPORT_ITEM_CODES
        .iter()
        .enumerate()
        .map(|(index, item)| {
            format!(
                r#"{{"item":"{item}","state":"ok","photoId":{}}}"#,
                index % 4 + 1
            )
        })
        .collect();
    format!(r#"{{"roomStatus":"vc","ticks":[{}]}}"#, ticks.join(","))
}

/// A v2 submission with a caller-chosen photo per tick and an explicit extras
/// list — the shape the photo-total rows need.
fn submit_body_with(photo_for: impl Fn(usize) -> i64, extras: &[i64]) -> String {
    let ticks: Vec<String> = REPORT_ITEM_CODES
        .iter()
        .enumerate()
        .map(|(index, item)| {
            format!(
                r#"{{"item":"{item}","state":"ok","photoId":{}}}"#,
                photo_for(index)
            )
        })
        .collect();
    let extras: Vec<String> = extras.iter().map(i64::to_string).collect();
    format!(
        r#"{{"roomStatus":"vc","ticks":[{}],"extraPhotoIds":[{}]}}"#,
        ticks.join(","),
        extras.join(",")
    )
}

/// The same body as a `&'static str`, for the const tables above.
static V2_SUBMIT_BODY: &str = concat!(
    r#"{"roomStatus":"vc","ticks":["#,
    r#"{"item":"water_glass","state":"ok","photoId":1},"#,
    r#"{"item":"coffee_tray","state":"ok","photoId":2},"#,
    r#"{"item":"coffee_cup","state":"ok","photoId":3},"#,
    r#"{"item":"coffee_sachet_jar","state":"ok","photoId":4},"#,
    r#"{"item":"kettle","state":"ok","photoId":1},"#,
    r#"{"item":"bathroom_bin","state":"ok","photoId":2},"#,
    r#"{"item":"hairdryer","state":"ok","photoId":3},"#,
    r#"{"item":"bath_amenity_tray","state":"ok","photoId":4},"#,
    r#"{"item":"aircon_remote","state":"ok","photoId":1},"#,
    r#"{"item":"tv_remote","state":"ok","photoId":2},"#,
    r#"{"item":"mirror_bin","state":"ok","photoId":3},"#,
    r#"{"item":"hangers","state":"ok","photoId":4},"#,
    r#"{"item":"bath_towel","state":"ok","photoId":1},"#,
    r#"{"item":"face_towel","state":"ok","photoId":2},"#,
    r#"{"item":"foot_towel","state":"ok","photoId":3},"#,
    r#"{"item":"duvet","state":"ok","photoId":4},"#,
    r#"{"item":"bed_sheet","state":"ok","photoId":1},"#,
    r#"{"item":"pillowcase","state":"ok","photoId":2},"#,
    r#"{"item":"duvet_cover","state":"ok","photoId":3},"#,
    r#"{"item":"pillow","state":"ok","photoId":4},"#,
    r#"{"item":"ashtray","state":"ok","photoId":1},"#,
    r#"{"item":"bathrobe","state":"ok","photoId":2}"#,
    r#"]}"#
);

/// The static table and the builder must stay the same body — otherwise a row
/// that uses one and a row that uses the other are testing different things.
#[test]
fn the_static_and_built_submit_bodies_agree() {
    assert_eq!(v2_submit_body(), V2_SUBMIT_BODY);
    let json: serde_json::Value = serde_json::from_str(V2_SUBMIT_BODY).expect("valid JSON");
    assert_eq!(
        json["ticks"].as_array().map(Vec::len),
        Some(22),
        "the checklist is 22 items and a submission ticks all of them"
    );
}

/// Every Report HK endpoint — read and write — 400s without a `?branch=`.
///
/// The photo endpoints matter most here: a photo carries no room and no report,
/// so the branch is the ONLY thing that decides which site's database it lands
/// in. A default would file HF Ville's evidence in HF Hotel with nothing else
/// in the request to contradict it.
#[tokio::test]
async fn report_endpoints_all_require_a_branch() {
    for uri in REPORT_READS {
        let (status, body) = get_inner(uri).await;
        assert_branch_400(status, &body, &format!("GET {uri} with no branch"));
    }
    for uri in [
        "/api/hk/reports?date=2026-09-02",
        "/api/hk/report-photos/9",
        // Migration 092's metadata probe — a photo carries no room and no
        // report, so the branch is the ONLY thing that says which site to look
        // in.
        "/api/hk/report-photos/9/meta",
    ] {
        let (status, body) = get_inner(uri).await;
        assert_branch_400(status, &body, &format!("GET {uri} with no branch"));
    }

    // …and migration 092's DELETE, which is a WRITE with the same problem: a
    // default branch would let a Ville maid delete an HF Hotel photo.
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(app, "DELETE", "/api/hk/report-photos/9", "").await;
    assert_branch_400(status, &body, "DELETE /api/hk/report-photos/9 with no branch");
    for (uri, payload) in REPORT_WRITES {
        // The MAID identity for the submission, the VIEWER for the verdicts —
        // otherwise the role gate would answer 403 first and this row would be
        // asserting the wrong thing.
        let app = if uri.starts_with("/api/hk/rooms/") {
            inner(
                AppState::new(lazy_pool()),
                policy(vec![Branch::Hfhotel], false),
            )
        } else {
            inner_viewer(
                AppState::new(lazy_pool()),
                policy(vec![Branch::Hfhotel], false),
            )
        };
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_branch_400(status, &body, &format!("POST {uri} with no branch"));
    }

    // The photo intake, through a REAL multipart body so the branch gate is
    // what answers rather than the extractor.
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call_multipart(app, "/api/hk/report-photos", Some(("photo", "JPEGBYTES"))).await;
    assert_branch_400(status, &body, "POST /api/hk/report-photos with no branch");
}

/// `?branch=all` is refused on every Report HK endpoint, exactly as on the
/// room and signal routes: `write_pool(Some(Branch::All))` returns the PRIMARY
/// pool, so accepting it would re-open the wrong-hotel bug under a different
/// query string.
#[tokio::test]
async fn report_endpoints_reject_branch_all() {
    for uri in REPORT_READS {
        let (status, body) = get_inner(&format!("{uri}?branch=all")).await;
        assert_branch_400(status, &body, &format!("GET {uri}?branch=all"));
    }
    let app = inner(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel, Branch::Hfville], false),
    );
    let (status, body) = call(app, "POST", "/api/hk/rooms/1/report?branch=all", V2_SUBMIT_BODY).await;
    assert_branch_400(status, &body, "POST report?branch=all");
}

/// A well-formed branch this deployment does not offer is 403 on the report
/// endpoints too — the `HK_BRANCHES` allowlist, not a per-feature list.
#[tokio::test]
async fn report_endpoints_respect_the_hk_branches_allowlist() {
    for uri in REPORT_READS {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "GET", &format!("{uri}?branch=hfville"), "").await;
        assert_eq!(status, StatusCode::FORBIDDEN, "GET {uri}: body={body}");
        let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
        assert_eq!(
            json.get("error").and_then(|v| v.as_str()),
            Some(BRANCH_NOT_ENABLED_ERROR)
        );
    }
}

/// **The maid-side role row.** A read-only reception viewer may not SUBMIT a
/// report — the same 403 and the same message it gets on the other maid
/// mutations, so the surface has ONE read-only story rather than three.
#[tokio::test]
async fn a_viewer_cannot_submit_a_report() {
    let app = inner_viewer(
        AppState::new(lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "POST",
        "/api/hk/rooms/1/report?branch=hfhotel",
        V2_SUBMIT_BODY,
    )
    .await;
    assert_report_403(status, &body, "viewer submitting a room report");
}

/// **The reception-side role row, and the one this feature exists for.** A
/// MAID may not verify or return a report — including a maid who also holds the
/// `reception` grant, because `can_report` is the maid side (CONTEXT.md
/// §Housekeeping: "A maid never verifies").
///
/// The message must be the VERIFY refusal, not the read-only one: a maid told
/// "this account can only view room status" would be looking for the wrong
/// problem entirely.
#[tokio::test]
async fn a_maid_cannot_verify_or_return_a_report() {
    for (uri, payload) in [
        ("/api/hk/reports/9/verify?branch=hfhotel", r#"{"photoIds":[1]}"#),
        (
            "/api/hk/reports/9/return?branch=hfhotel",
            r#"{"reason":"not_clean"}"#,
        ),
    ] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "POST {uri} from a maid: expected 403, body={body}"
        );
        let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
        assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(false));
        assert_eq!(
            json.get("error").and_then(|v| v.as_str()),
            Some(VERIFY_NOT_PERMITTED_ERROR),
            "a maid must be told the verdict is reception's, not that she is read-only"
        );
    }
}

/// Both capability gates run BEFORE the branch gate and before the body — so a
/// wrong-role caller is refused without a location lookup and without a
/// database round-trip, whatever nonsense the rest of the request carries.
///
/// Asserted in BOTH directions in one test, because the ordering is the same
/// property and a fix that reordered one gate would probably reorder the other.
#[tokio::test]
async fn the_report_capability_gates_run_before_branch_and_body() {
    // The bodies below are syntactically valid JSON and semantically wrong in
    // every field: an unknown branch, an unknown room-status code, a
    // contradicted attestation, an out-of-range photo count. A role refusal
    // must outrank all of it.
    //
    // (Syntactically valid because `Json` is an EXTRACTOR: axum rejects an
    // unparseable body before any handler code runs, so a "not json at all"
    // probe would assert axum's rejection, not this module's gate ordering.
    // Same reason `viewer_capability_is_checked_before_branch_and_body` sends
    // well-formed nonsense.)
    for (uri, payload, what) in [
        (
            "/api/hk/rooms/1/report",
            r#"{"roomStatus":"vacant","ticks":[{"item":"nope","state":"gone","qty":0}]}"#,
            "viewer submit: no branch, every field wrong",
        ),
        (
            "/api/hk/rooms/1/report?branch=all",
            r#"{}"#,
            "viewer submit: branch=all, empty body",
        ),
        (
            "/api/hk/rooms/1/report?branch=hfville",
            V2_SUBMIT_BODY,
            "viewer submit: unoffered branch, valid body",
        ),
        (
            // The RETIRED v1 body must not become a way past the role gate
            // either: capability outranks body shape, in both directions.
            "/api/hk/rooms/1/report?branch=hfhotel",
            r#"{"roomStatus":"vc","allItemsOk":true,"items":[],"photoIds":[1]}"#,
            "viewer submit: retired v1 body",
        ),
    ] {
        let app = inner_viewer(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_report_403(status, &body, what);
    }

    // Maid judging: the same shape, the other direction and the other message.
    for (uri, payload, what) in [
        (
            "/api/hk/reports/9/verify",
            r#"{"photoIds":[]}"#,
            "maid verify: no branch, zero photos",
        ),
        (
            "/api/hk/reports/9/return?branch=all",
            r#"{"reason":"dirty"}"#,
            "maid return: branch=all, unknown reason",
        ),
    ] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{what}: body={body}");
        let json: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or_else(|_| panic!("{what}: 403 body must be JSON, got {body}"));
        assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(false));
        assert_eq!(
            json.get("error").and_then(|v| v.as_str()),
            Some(VERIFY_NOT_PERMITTED_ERROR),
            "{what}"
        );
    }
}

/// The photo INTAKE is open to both roles — the side is derived from the role,
/// so neither can manufacture the other's evidence and neither needs to be
/// refused at the door. Both identities must therefore clear the capability
/// gates and be stopped by the BRANCH gate instead.
///
/// The same holds for the two reads: reception must see the maid's photos to
/// judge them, and the maid must see what came back with a return.
#[tokio::test]
async fn the_photo_endpoints_are_open_to_both_roles() {
    for maid_side in [true, false] {
        let state = AppState::new(lazy_pool());
        let pol = policy(vec![Branch::Hfhotel], false);
        let app = if maid_side {
            inner(state, pol)
        } else {
            inner_viewer(state, pol)
        };
        // No branch ⇒ the BRANCH gate answers, which proves no capability gate
        // fired first.
        let (status, body) =
            call_multipart(app, "/api/hk/report-photos", Some(("photo", "JPEGBYTES"))).await;
        assert_branch_400(
            status,
            &body,
            &format!("photo upload (can_report={maid_side})"),
        );
    }

    for maid_side in [true, false] {
        let state = AppState::new(lazy_pool());
        let pol = policy(vec![Branch::Hfhotel], false);
        let app = if maid_side {
            inner(state, pol)
        } else {
            inner_viewer(state, pol)
        };
        let (status, body) = call(app, "GET", "/api/hk/report-photos/9", "").await;
        assert_branch_400(
            status,
            &body,
            &format!("photo read (can_report={maid_side})"),
        );
    }
}

/// The VERIFY side keeps its 1..=4 per-transition bound, and the SUBMIT side
/// now has a DISTINCT-photo total of 4..=24 across ticks and extras. Both
/// answer in the repo's envelope — never a serde rejection and never a 500 from
/// the attach statement coming up short.
#[tokio::test]
async fn report_photo_counts_are_400_on_both_sides() {
    let too_many = r#"[1,2,3,4,5]"#;

    // Every tick on ONE photo: three short of the four-zone floor.
    let one_photo = submit_body_with(|_| 1, &[]);
    // …and the standard four-photo body plus 21 extras: 25 distinct, one over
    // the ceiling.
    let extras: Vec<i64> = (100..121).collect();
    let over_ceiling = submit_body_with(|index| index as i64 % 4 + 1, &extras);
    for (uri, payload, what) in [
        (
            "/api/hk/rooms/1/report?branch=hfhotel",
            one_photo.as_str(),
            "submit backed by 1 distinct photo (below the four-zone floor)",
        ),
        (
            "/api/hk/rooms/1/report?branch=hfhotel",
            over_ceiling.as_str(),
            "submit backed by 25 distinct photos",
        ),
        (
            "/api/hk/rooms/1/report?branch=hfhotel",
            r#"{"roomStatus":"vc","ticks":[]}"#,
            "submit with no ticks at all",
        ),
        (
            "/api/hk/rooms/1/report?branch=hfhotel",
            r#"{"roomStatus":"vc"}"#,
            "submit with ticks omitted",
        ),
    ] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "POST", uri, payload).await;
        assert_bad_request(status, &body, what);
    }

    for (payload, what) in [
        (r#"{"photoIds":[]}"#, "verify with 0 photos"),
        (r#"{}"#, "verify with photoIds omitted"),
        (
            &format!(r#"{{"photoIds":{too_many}}}"#),
            "verify with 5 photos",
        ),
    ] {
        let app = inner_viewer(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(
            app,
            "POST",
            "/api/hk/reports/9/verify?branch=hfhotel",
            payload,
        )
        .await;
        assert_bad_request(status, &body, what);
    }
}

/// Body validation answers in the REPO's envelope for every malformed shape —
/// the reason each parser takes its field as an `Option`/`Value` rather than a
/// typed field serde would reject with a foreign body shape.
///
/// **v2**: the checklist is a tick list, so the rows are about coverage,
/// per-tick state/qty/photo rules, and the retired v1 shape.
#[tokio::test]
async fn report_body_errors_use_the_repo_envelope() {
    // A tick body with ONE item replaced by `entry`.
    let swap = |index: usize, entry: &str| -> String {
        let ticks: Vec<String> = REPORT_ITEM_CODES
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if i == index {
                    entry.to_string()
                } else {
                    format!(r#"{{"item":"{item}","state":"ok","photoId":{}}}"#, i % 4 + 1)
                }
            })
            .collect();
        format!(r#"{{"roomStatus":"vc","ticks":[{}]}}"#, ticks.join(","))
    };
    // The full body minus its last tick — 21 of 22.
    let short = {
        let ticks: Vec<String> = REPORT_ITEM_CODES
            .iter()
            .enumerate()
            .take(21)
            .map(|(i, item)| format!(r#"{{"item":"{item}","state":"ok","photoId":{}}}"#, i % 4 + 1))
            .collect();
        format!(r#"{{"roomStatus":"vc","ticks":[{}]}}"#, ticks.join(","))
    };
    // 22 entries, one item ticked twice and another absent — the count alone
    // is never the test.
    let duplicated = swap(0, r#"{"item":"tv_remote","state":"ok","photoId":1}"#);

    let rows: Vec<(String, &str)> = vec![
        (
            swap(9, r#"{"item":"tv_remote","state":"ok","photoId":2}"#)
                .replace(r#""roomStatus":"vc""#, r#""roomStatus":"vacant""#),
            "unknown roomStatus",
        ),
        (
            V2_SUBMIT_BODY.replace(r#""roomStatus":"vc","#, ""),
            "missing roomStatus",
        ),
        (short, "21 of 22 ticks — the checklist must be complete"),
        (duplicated, "one item ticked twice and another absent"),
        (
            swap(9, r#"{"item":"remote","state":"ok","photoId":2}"#),
            "unknown item code",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"broken","qty":1,"photoId":2}"#),
            "unknown state code (the code is 'damaged')",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"missing","photoId":2}"#),
            "a problem tick with no qty",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"ok","qty":1,"photoId":2}"#),
            "an ok tick carrying a qty",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"missing","qty":0,"photoId":2}"#),
            "qty below the bound",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"missing","qty":100,"photoId":2}"#),
            "qty above the bound",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"missing","qty":"2","photoId":2}"#),
            "qty as a string",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"ok"}"#),
            "a tick with no photoId — every tick is photo-backed",
        ),
        (
            swap(9, r#"{"item":"tv_remote","state":"ok","photoId":0}"#),
            "a non-positive photoId",
        ),
        (
            V2_SUBMIT_BODY.replace(
                r#"{"roomStatus":"vc""#,
                r#"{"date":"2026-9-2","roomStatus":"vc""#,
            ),
            "unpadded date",
        ),
        (
            V2_SUBMIT_BODY.replace(
                r#"{"roomStatus":"vc""#,
                r#"{"date":"02/09/2026","roomStatus":"vc""#,
            ),
            "locale-format date",
        ),
        (
            V2_SUBMIT_BODY.replace(r#"]}"#, r#"],"extraPhotoIds":[9,9]}"#),
            "the same extra photo twice",
        ),
    ];

    for (payload, what) in rows {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "POST", "/api/hk/rooms/1/report?branch=hfhotel", &payload)
            .await;
        assert_bad_request(status, &body, what);
    }

    // The return reason is canned: an unknown one, and an absent one, are 400.
    for (payload, what) in [
        (r#"{"reason":"dirty"}"#, "unknown return reason"),
        (r#"{}"#, "missing return reason"),
        (r#"{"reason":"ยังไม่สะอาด"}"#, "the Thai LABEL, not the code"),
    ] {
        let app = inner_viewer(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(
            app,
            "POST",
            "/api/hk/reports/9/return?branch=hfhotel",
            payload,
        )
        .await;
        assert_bad_request(status, &body, what);
    }
}

/// **The cross-version row.** A stale bundle sends the RETIRED v1 body —
/// `allItemsOk` + `items` — and must get a 400 whose message NAMES `ticks`.
///
/// Both halves of Report HK v2 deploy as one atomic pair, so the only client
/// that can send v1 is a cached one; the only useful thing to tell it is what
/// the field is called now. A generic "ticks is required" would read like a
/// client bug and send someone looking in the wrong place.
///
/// Every v1-shaped variant is covered, including a body that sends BOTH shapes:
/// a client that does not know which one the server believes is a client we
/// must not guess for.
#[tokio::test]
async fn a_v1_report_body_is_400_naming_ticks() {
    let both_shapes = V2_SUBMIT_BODY.replace(
        r#"{"roomStatus":"vc""#,
        r#"{"allItemsOk":true,"roomStatus":"vc""#,
    );
    for (payload, what) in [
        (
            r#"{"roomStatus":"vc","allItemsOk":true,"items":[],"photoIds":[1]}"#,
            "the exact v1 clean-room body",
        ),
        (
            r#"{"roomStatus":"vc","allItemsOk":false,"items":[{"item":"tv_remote","problem":"missing","qty":1}],"photoIds":[1]}"#,
            "the v1 exception body",
        ),
        (
            r#"{"roomStatus":"vc","allItemsOk":true}"#,
            "allItemsOk alone",
        ),
        (
            r#"{"roomStatus":"vc","items":[]}"#,
            "items alone — either field is the v1 shape",
        ),
        (
            both_shapes.as_str(),
            "BOTH shapes at once — refused, never silently resolved",
        ),
    ] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, "POST", "/api/hk/rooms/1/report?branch=hfhotel", payload)
            .await;
        assert_bad_request(status, &body, what);
        let json: serde_json::Value = serde_json::from_str(&body).expect("400 body must be JSON");
        let error = json.get("error").and_then(|v| v.as_str()).unwrap_or_default();
        assert!(
            error.contains("ticks"),
            "{what}: the refusal must NAME the field that replaced them, got {error}"
        );
    }
}

/// A malformed `?date=` is 400 on the overview, and an absent one is not — the
/// day defaults to today in Bangkok, which is the one place this surface is
/// allowed to pick a value for the client.
#[tokio::test]
async fn the_overview_date_is_optional_but_validated() {
    for bad in ["yesterday", "2026-9-2", "2026-13-01", "2026-09-02T00:00:00Z"] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(
            app,
            "GET",
            &format!("/api/hk/reports?branch=hfhotel&date={bad}"),
            "",
        )
        .await;
        assert_bad_request(status, &body, &format!("date={bad}"));
    }

    // A well-formed date clears every PURE gate and falls through to the
    // database (which is not reachable here) — asserted negatively, exactly
    // like row 5, because the point is only that no gate fired.
    let app = inner(
        AppState::new(clamped_lazy_pool()),
        policy(vec![Branch::Hfhotel], false),
    );
    let (status, body) = call(
        app,
        "GET",
        "/api/hk/reports?branch=hfhotel&date=2026-09-02",
        "",
    )
    .await;
    assert_ne!(status, StatusCode::BAD_REQUEST, "the date was valid: {body}");
    assert_ne!(status, StatusCode::FORBIDDEN, "the branch was enabled: {body}");
}

/// Through the REAL router every Report HK probe is 401 — the proof that all
/// seven endpoints sit INSIDE `require_hk_access`, and (for the writes) that
/// the Ville guard ADMITS them rather than short-circuiting with 403 even on
/// `branch=hfville`.
///
/// A new endpoint that forgot the Access layer would be a silently
/// unauthenticated write surface holding photographs of guest rooms, so this
/// runs without a database and is never gated behind an optional dependency.
#[tokio::test]
async fn shipped_router_answers_401_for_every_report_endpoint() {
    let mut probes: Vec<(&str, String, &str)> = Vec::new();
    for uri in REPORT_READS {
        for suffix in ["", "?branch=hfhotel", "?branch=hfville", "?branch=all"] {
            probes.push(("GET", format!("{uri}{suffix}"), ""));
        }
    }
    probes.push(("GET", "/api/hk/report-photos/9?branch=hfhotel".to_string(), ""));
    probes.push((
        "GET",
        "/api/hk/reports?branch=hfhotel&date=2026-09-02".to_string(),
        "",
    ));
    // Migration 092's two new photo endpoints. The DELETE matters most: it is
    // the first non-POST write on this surface, so it is the first that could
    // have been mounted outside the Access layer by mistake — an
    // unauthenticated DELETE against photographs of guest rooms.
    for suffix in ["", "?branch=hfhotel", "?branch=hfville", "?branch=all"] {
        probes.push(("GET", format!("/api/hk/report-photos/9/meta{suffix}"), ""));
        probes.push(("DELETE", format!("/api/hk/report-photos/9{suffix}"), ""));
    }
    for (uri, payload) in REPORT_WRITES {
        for suffix in ["", "?branch=hfhotel", "?branch=hfville", "?branch=all"] {
            probes.push(("POST", format!("{uri}{suffix}"), payload));
        }
    }

    for (method, uri, payload) in probes {
        let app =
            hotel_backend::routes::hk::router(AppState::new(lazy_pool()).with_hfville_writes(true));
        let (status, got) = call(app, method, &uri, payload).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "{method} {uri} must be refused by the Access gate before any branch, \
             role, body or pool logic can answer; got {status} {got}"
        );
    }

    // The multipart intake, with a REAL body — the Access layer must answer
    // before the extractor ever reads a byte of it.
    for suffix in ["", "?branch=hfhotel", "?branch=hfville", "?branch=all"] {
        let app =
            hotel_backend::routes::hk::router(AppState::new(lazy_pool()).with_hfville_writes(true));
        let uri = format!("/api/hk/report-photos{suffix}");
        let (status, got) = call_multipart(app, &uri, Some(("photo", "JPEGBYTES"))).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "POST {uri} must be refused by the Access gate; got {status} {got}"
        );
    }
}

// ============================================================================
// Report HK v2 — the photo DELETE and /meta (migration 092)
// ============================================================================

/// Both photo endpoints are open to BOTH roles, exactly like the intake and the
/// byte read: the SIDE is derived from the uploader, so each role can only ever
/// reach its own pictures and neither needs refusing at the door.
///
/// A maid must be able to drop a blurred zone shot and a receptionist a bad
/// verify shot; a capability gate here would refuse one of them for holding the
/// wrong grant rather than for owning the wrong photo. Both identities must
/// therefore clear the capability gates and be stopped by the BRANCH gate.
///
/// DB-free: the branch gate answers before a pool is touched.
#[tokio::test]
async fn the_photo_delete_and_meta_are_open_to_both_roles() {
    for maid_side in [true, false] {
        for (method, uri) in [
            ("DELETE", "/api/hk/report-photos/9"),
            ("GET", "/api/hk/report-photos/9/meta"),
        ] {
            let state = AppState::new(lazy_pool());
            let pol = policy(vec![Branch::Hfhotel], false);
            let app = if maid_side {
                inner(state, pol)
            } else {
                inner_viewer(state, pol)
            };
            let (status, body) = call(app, method, uri, "").await;
            assert_branch_400(
                status,
                &body,
                &format!("{method} {uri} (can_report={maid_side})"),
            );
        }
    }
}

/// The `HK_BRANCHES` allowlist applies to both new endpoints too — a
/// well-formed branch this deployment does not offer is 403, not a fallback to
/// the primary pool.
#[tokio::test]
async fn the_photo_delete_and_meta_respect_the_hk_branches_allowlist() {
    for (method, uri) in [
        ("DELETE", "/api/hk/report-photos/9?branch=hfville"),
        ("GET", "/api/hk/report-photos/9/meta?branch=hfville"),
    ] {
        let app = inner(
            AppState::new(lazy_pool()),
            policy(vec![Branch::Hfhotel], false),
        );
        let (status, body) = call(app, method, uri, "").await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}: body={body}");
        let json: serde_json::Value = serde_json::from_str(&body).expect("403 body must be JSON");
        assert_eq!(
            json.get("error").and_then(|v| v.as_str()),
            Some(BRANCH_NOT_ENABLED_ERROR)
        );
    }
}

/// **The DELETE's whole rule matrix, against a real database.**
///
/// `DELETE /api/hk/report-photos/{id}` is the maid's manage-pictures-before-
/// submit primitive, and it is the ONE place in this app that removes photo
/// evidence — so every boundary it draws is worth a row:
///
/// * her OWN UNATTACHED photo is deleted, and the row is really gone;
/// * someone ELSE's photo is **403**, and survives;
/// * an ATTACHED photo is **400**, and survives — the owner's keep-forever
///   decision means nothing here can delete a picture a report names;
/// * an unknown id is **404**;
/// * a repeat of a successful delete is **404**, not a second success.
///
/// The reception VIEWER gets the same treatment on its own photos, which is why
/// the `reception`-side row is here rather than in a separate test: one rule,
/// two roles.
///
/// Skips when PG is unreachable, the convention this suite already follows.
#[tokio::test]
async fn deleting_a_report_photo_enforces_owner_and_unattached() {
    let Some(pool) = live_pool().await else {
        eprintln!("skipping deleting_a_report_photo_enforces_owner_and_unattached — PG not reachable");
        return;
    };

    // Seed photos directly: the intake is multipart and this row is about the
    // DELETE's rules, not the upload's.
    async fn seed_photo(
        pool: &PgPool,
        badge: &str,
        side: &str,
        attached_to: Option<i64>,
    ) -> i64 {
        let row = sqlx::query(
            "INSERT INTO ht_hk_room_report_photos \
                 (rrp_report_id, rrp_side, rrp_photo, rrp_photo_mime, rrp_badge, rrp_zone, rrp_bytes) \
             VALUES ($1, $2, $3, 'image/jpeg', $4, 'bed', 8) RETURNING rrp_id",
        )
        .bind(attached_to)
        .bind(side)
        .bind(b"fakejpeg".as_slice())
        .bind(badge)
        .fetch_one(pool)
        .await
        .expect("seed photo must insert");
        row.try_get("rrp_id").expect("rrp_id")
    }

    async fn exists(pool: &PgPool, photo_id: i64) -> bool {
        sqlx::query("SELECT 1 FROM ht_hk_room_report_photos WHERE rrp_id = $1")
            .bind(photo_id)
            .fetch_optional(pool)
            .await
            .expect("existence probe")
            .is_some()
    }

    // A room and a report to attach one photo to. Cleaned up at the end; the
    // report and its photos cascade with the room.
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-RP2'")
        .execute(&pool)
        .await;
    let room_row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
         VALUES ('ZT-RP2', true, true) RETURNING room_id",
    )
    .fetch_one(&pool)
    .await
    .expect("marker room must insert");
    let room_id: i32 = room_row.try_get("room_id").expect("room_id");

    let report_row = sqlx::query(
        "INSERT INTO ht_hk_room_reports \
             (rr_room_id, rr_date, rr_status, rr_room_status, rr_all_items_ok, rr_submitted_badge) \
         VALUES ($1, (NOW() AT TIME ZONE 'Asia/Bangkok')::date, 'submitted', 'vc', true, 'Q1001') \
         RETURNING rr_id",
    )
    .bind(room_id)
    .fetch_one(&pool)
    .await
    .expect("marker report must insert");
    let report_id: i64 = report_row.try_get("rr_id").expect("rr_id");

    // `maid()` is badge Q1001; `viewer()` shares it, so the not-yours row needs
    // a THIRD badge that neither identity holds.
    let mine = seed_photo(&pool, "Q1001", "maid", None).await;
    let theirs = seed_photo(&pool, "Z9999", "maid", None).await;
    let attached = seed_photo(&pool, "Q1001", "maid", Some(report_id)).await;
    let reception_photo = seed_photo(&pool, "Q1001", "reception", None).await;

    let del = |photo_id: i64, as_maid: bool| {
        let state = AppState::new(pool.clone());
        let pol = policy(vec![Branch::Hfhotel], false);
        let app = if as_maid {
            inner(state, pol)
        } else {
            inner_viewer(state, pol)
        };
        async move {
            call(
                app,
                "DELETE",
                &format!("/api/hk/report-photos/{photo_id}?branch=hfhotel"),
                "",
            )
            .await
        }
    };

    // 1. Her own unattached photo: gone.
    let (status, body) = del(mine, true).await;
    assert_eq!(status, StatusCode::OK, "own unattached delete: body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body must be JSON");
    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(true));
    assert!(!exists(&pool, mine).await, "the row must really be gone");

    // 2. A repeat is a 404, not a second success.
    let (status, body) = del(mine, true).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "repeat delete: body={body}");

    // 3. Someone else's photo: 403, and it survives.
    let (status, body) = del(theirs, true).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "another badge's photo: body={body}");
    assert!(
        exists(&pool, theirs).await,
        "a refused delete must not remove the row"
    );

    // 4. An ATTACHED photo: 400 (the repo's conflict mapping), and it survives.
    //    This is the keep-forever guarantee: no path here deletes evidence.
    let (status, body) = del(attached, true).await;
    assert_bad_request(status, &body, "deleting a photo a report already names");
    assert!(
        exists(&pool, attached).await,
        "an attached photo must survive its own delete attempt"
    );

    // 5. An unknown id: 404.
    let (status, body) = del(-4242, true).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "unknown photo: body={body}");

    // 6. The reception VIEWER deletes its own photo on the same terms.
    let (status, body) = del(reception_photo, false).await;
    assert_eq!(status, StatusCode::OK, "viewer's own photo: body={body}");
    assert!(!exists(&pool, reception_photo).await);

    // Cleanup (the report and the attached photo cascade with the room).
    let _ = sqlx::query("DELETE FROM ht_hk_room_report_photos WHERE rrp_badge = 'Z9999'")
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(&pool)
        .await;
}

/// `GET /api/hk/report-photos/{id}/meta` — the client's resume-after-reload
/// probe. It must answer the one bit that decides what the maid may still do
/// with an id she is holding: `attached`.
///
/// Skips when PG is unreachable.
#[tokio::test]
async fn report_photo_meta_reports_zone_size_and_attachment() {
    let Some(pool) = live_pool().await else {
        eprintln!("skipping report_photo_meta_reports_zone_size_and_attachment — PG not reachable");
        return;
    };

    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-RP3'")
        .execute(&pool)
        .await;
    let room_row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
         VALUES ('ZT-RP3', true, true) RETURNING room_id",
    )
    .fetch_one(&pool)
    .await
    .expect("marker room must insert");
    let room_id: i32 = room_row.try_get("room_id").expect("room_id");
    let report_row = sqlx::query(
        "INSERT INTO ht_hk_room_reports \
             (rr_room_id, rr_date, rr_status, rr_room_status, rr_all_items_ok, rr_submitted_badge) \
         VALUES ($1, (NOW() AT TIME ZONE 'Asia/Bangkok')::date, 'submitted', 'vc', true, 'Q1001') \
         RETURNING rr_id",
    )
    .bind(room_id)
    .fetch_one(&pool)
    .await
    .expect("marker report must insert");
    let report_id: i64 = report_row.try_get("rr_id").expect("rr_id");

    let seed = |zone: Option<&'static str>, attached_to: Option<i64>| {
        let pool = pool.clone();
        async move {
            let row = sqlx::query(
                "INSERT INTO ht_hk_room_report_photos \
                     (rrp_report_id, rrp_side, rrp_photo, rrp_photo_mime, rrp_badge, rrp_zone, rrp_bytes) \
                 VALUES ($1, 'maid', $2, 'image/jpeg', 'Q1001', $3, 8) RETURNING rrp_id",
            )
            .bind(attached_to)
            .bind(b"fakejpeg".as_slice())
            .bind(zone)
            .fetch_one(&pool)
            .await
            .expect("seed photo must insert");
            row.try_get::<i64, _>("rrp_id").expect("rrp_id")
        }
    };

    let unattached = seed(Some("bathroom"), None).await;
    let filed = seed(None, Some(report_id)).await;

    let meta = |photo_id: i64| {
        let app = inner(
            AppState::new(pool.clone()),
            policy(vec![Branch::Hfhotel], false),
        );
        async move {
            call(
                app,
                "GET",
                &format!("/api/hk/report-photos/{photo_id}/meta?branch=hfhotel"),
                "",
            )
            .await
        }
    };

    let (status, body) = meta(unattached).await;
    assert_eq!(status, StatusCode::OK, "meta of an unattached photo: body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body must be JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["photo"]["photoId"], unattached);
    assert_eq!(json["photo"]["side"], "maid");
    assert_eq!(json["photo"]["zone"], "bathroom");
    assert_eq!(json["photo"]["bytes"], 8);
    assert_eq!(
        json["photo"]["attached"], false,
        "an unattached photo is the one she may still delete or tick against"
    );
    assert!(json["photo"]["uploadedAt"].is_string());

    let (status, body) = meta(filed).await;
    assert_eq!(status, StatusCode::OK, "meta of a filed photo: body={body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body must be JSON");
    assert_eq!(json["photo"]["attached"], true);
    assert!(
        json["photo"]["zone"].is_null(),
        "a photo with no zone answers an explicit null, never an omitted key"
    );

    let (status, _) = meta(-4242).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "an unknown photo is a 404");

    let _ = sqlx::query("DELETE FROM ht_hk_room_report_photos WHERE rrp_id = $1")
        .bind(unattached)
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(&pool)
        .await;
}
