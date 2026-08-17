//! Per-employee location enforcement on the `/hk` maid surface — wave-4 C.
//!
//! ## The bug this pins
//!
//! `GET /api/hk/me` served the GLOBAL `HK_BRANCHES` allowlist to every maid,
//! and `require_branch` checked only that allowlist. `HK_BRANCHES` answers
//! "which properties does this DEPLOYMENT serve" — it never answered "which
//! property does THIS EMPLOYEE work at". So an HF Ville maid was offered
//! "ฮาร์เบอร์ฟร้อนท์" in the picker and could file a cleaning report — and,
//! for `done`, a real `MarkRoomClean` writeback — against the WRONG PROPERTY.
//!
//! HF ID holds the authoritative answer in `Employee.location`, so with
//! `HK_LOCATION_ENFORCEMENT_ENABLED=true` the surface asks it (badge →
//! `HF` | `HF_VILLE` | null) and refuses anything that does not match.
//!
//! ## The hard property
//!
//! **Nothing falls back.** A null location, an unknown badge, an inactive or
//! pending employee, an unreachable HF ID, an unset URL and an unset secret all
//! REFUSE — none of them degrades to the allowlist or to `hfhotel`, and there
//! is no per-badge carve-out. Two refusal KINDS, deliberately distinct:
//!
//! - **403** — a definite "no" (mismatch, or no usable location). Retrying
//!   changes nothing; an admin must act.
//! - **503** — "cannot tell" (lookup unreachable / unconfigured). The request
//!   may be perfectly legitimate, so a maid must not be told she is not
//!   allowed because a LAN cable is loose.
//!
//! ## Where the rows are asserted
//!
//! Through [`inner`] — `routes::hk::routes_inside_access`, the SAME handler
//! table the shipped router wraps, with an `HkIdentity` injected where the
//! Cloudflare Access layer would have put one. Through the shipped
//! `routes::hk::router` every unauthenticated probe is 401 by design, so the
//! gate's own status codes are unobservable there (the same two-router split
//! `tests/test_hk_branch_required.rs` documents, and for the same reason).
//!
//! `tests/test_hk_ville_guard.rs` is UNCHANGED by this stream: it drives the
//! shipped router unauthenticated, where the Ville guard and the Access gate
//! still answer 403/401 before any handler — and therefore before this gate —
//! runs.
//!
//! ## Running
//! NO database required: every row here is answered before any pool is
//! touched, and the pool is lazy.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Extension;
use hotel_backend::hfid_location::{
    outcome_for_resolve_badge_body, EmployeeLocation, HfidLocationClient, LocationLookup,
    LocationOutcome,
};
use hotel_backend::middleware::hk_access::HkIdentity;
use hotel_backend::routes::hk::{
    HkPolicy, BRANCH_NOT_ENABLED_ERROR, LOCATION_LOOKUP_UNAVAILABLE_ERROR, LOCATION_MISMATCH_ERROR,
    LOCATION_UNKNOWN_ERROR, REASON_LOOKUP_UNAVAILABLE, REASON_NO_LOCATION,
};
use hotel_backend::routes::mode::{AppState, Branch};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

/// A pool that never connects. Every REFUSAL row is answered before any SQL
/// runs; the two PASSTHROUGH rows deliberately fall through to the database,
/// so the acquire timeout is clamped hard — at sqlx's 30 s default those rows
/// alone cost two minutes of CI per run.
fn lazy_pool() -> PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(250))
        .connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
        .expect("a lazy pool needs no live server")
}

/// The badge the owner has NOT yet given a location — the one real NULL in HF
/// ID today, and the reason V14 lists "set badge 421's location" as a
/// PRECONDITION of the flip rather than a follow-up.
const NULL_LOCATION_BADGE: &str = "421";

fn maid(badge: &str) -> HkIdentity {
    HkIdentity {
        badge: badge.to_string(),
        display_name: None,
        email: None,
    }
}

/// A lookup that answers per badge, so one policy can serve the whole matrix.
struct MatrixLookup;

#[async_trait]
impl LocationLookup for MatrixLookup {
    async fn lookup(&self, badge: &str) -> LocationOutcome {
        match badge {
            "HOTEL-MAID" => LocationOutcome::Resolved(EmployeeLocation::Hf),
            "VILLE-MAID" => LocationOutcome::Resolved(EmployeeLocation::HfVille),
            // Null `location`, unknown badge, inactive or pending employee —
            // HF ID answered, there is simply no usable branch.
            NULL_LOCATION_BADGE | "UNKNOWN-BADGE" | "INACTIVE" | "PENDING" => {
                LocationOutcome::NoLocation
            }
            // HF ID unreachable / non-2xx / undecodable.
            _ => LocationOutcome::Unavailable,
        }
    }
}

/// Enforcement ON, both properties allowlisted, [`MatrixLookup`] behind it.
fn enforcing(branches: Vec<Branch>) -> HkPolicy {
    HkPolicy {
        branches,
        location_enforcement_enabled: true,
        location: Some(Arc::new(HfidLocationClient::with_lookup(Arc::new(
            MatrixLookup,
        )))),
        ..HkPolicy::default()
    }
}

// ============================================================================
// The `housekeeping_admin` grant (owner-approved "admin can pick location")
// ============================================================================
//
// These rows are driven by REAL `/resolve-badge` BODIES rather than by
// hand-written outcomes, because the thing under test IS the body → outcome
// collapse: an admin with a location and an admin with a NULL location must
// reach the same place, and asserting that from a hand-written `AnyLocation`
// would assume it instead of proving it.

/// Verbatim live payload shape (fingerprint-time-logger `5b45a235`), with the
/// grant present. Badge 421 is the one employee with a NULL location today,
/// which is exactly why it is the interesting admin row: without the grant it
/// is refused outright (row 4), with it, every served property is offered.
const ADMIN_WITH_LOCATION: &str = r#"{"found":true,"badge":"A1","display_name":"หัวหน้าแม่บ้าน",
    "apps":["hotel","housekeeping","housekeeping_admin"],"active":true,"pending":false,
    "location":"HF"}"#;

const ADMIN_WITH_NULL_LOCATION: &str = r#"{"found":true,"badge":"421","display_name":"นก",
    "apps":["hotel","housekeeping","housekeeping_admin"],"active":true,"pending":false,
    "location":null}"#;

/// TODAY'S LIVE SHAPE for badge 421 — the grant absent from `apps`. This is
/// the darkness proof in test form: the payload the live lookup actually
/// returns right now must behave exactly as it did before this feature
/// existed (⇒ 403, no location on file).
const NON_ADMIN_NULL_LOCATION: &str = r#"{"found":true,"badge":"421","display_name":"นก",
    "apps":["hotel","housekeeping"],"active":true,"pending":false,"location":null}"#;

/// A plain HF Hotel maid, grant absent. The byte-unchanged control.
const NON_ADMIN_WITH_LOCATION: &str = r#"{"found":true,"badge":"HOTEL-MAID","display_name":null,
    "apps":["hotel","housekeeping"],"active":true,"pending":false,"location":"HF"}"#;

/// Badge → the `/resolve-badge` BODY HF ID would return, decoded through the
/// production interpretation. Counts calls, so the cache rows can prove a
/// grant is replayed from cache rather than re-asked.
struct PayloadLookup {
    calls: AtomicUsize,
}

impl PayloadLookup {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
        })
    }
    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl LocationLookup for PayloadLookup {
    async fn lookup(&self, badge: &str) -> LocationOutcome {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let body = match badge {
            "ADMIN-LOCATED" => ADMIN_WITH_LOCATION,
            "ADMIN-NULL" => ADMIN_WITH_NULL_LOCATION,
            "NON-ADMIN-NULL" => NON_ADMIN_NULL_LOCATION,
            "NON-ADMIN-LOCATED" => NON_ADMIN_WITH_LOCATION,
            _ => r#"{"found":false,"active":false,"location":null}"#,
        };
        outcome_for_resolve_badge_body(body)
    }
}

/// Enforcement ON with the payload-driven lookup behind it.
fn enforcing_payloads(branches: Vec<Branch>, lookup: Arc<PayloadLookup>) -> HkPolicy {
    HkPolicy {
        branches,
        location_enforcement_enabled: true,
        location: Some(Arc::new(HfidLocationClient::with_lookup(lookup))),
        ..HkPolicy::default()
    }
}

/// Enforcement ON but the lookup UNCONFIGURED — `HFID_LOCATION_URL` /
/// `HFID_RESOLVE_SECRET` not both set.
fn enforcing_unconfigured() -> HkPolicy {
    HkPolicy {
        branches: vec![Branch::Hfhotel, Branch::Hfville],
        location_enforcement_enabled: true,
        location: None,
        ..HkPolicy::default()
    }
}

/// The DARK default: enforcement off, but a lookup wired that would answer
/// `Unavailable` (⇒ 503) for every badge. Any non-503 answer therefore proves
/// the dark path never consulted it.
fn dark() -> HkPolicy {
    HkPolicy {
        branches: vec![Branch::Hfhotel, Branch::Hfville],
        location_enforcement_enabled: false,
        location: Some(Arc::new(HfidLocationClient::with_lookup(Arc::new(
            MatrixLookup,
        )))),
        ..HkPolicy::default()
    }
}

/// The handler table with a verified identity injected where the Cloudflare
/// Access layer would have put one.
fn inner(policy: HkPolicy, identity: HkIdentity) -> axum::Router {
    hotel_backend::routes::hk::routes_inside_access(AppState::new(lazy_pool()), policy)
        .layer(Extension(identity))
}

async fn call(
    policy: HkPolicy,
    identity: HkIdentity,
    method: &str,
    uri: &str,
    body: &str,
) -> (StatusCode, String) {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request builds");
    let response = inner(policy, identity)
        .oneshot(req)
        .await
        .expect("router responds");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body reads");
    (status, String::from_utf8_lossy(&bytes).to_string())
}

fn assert_envelope(body: &str, expected_error: &str, what: &str) {
    let json: serde_json::Value =
        serde_json::from_str(body).unwrap_or_else(|_| panic!("{what}: body must be JSON: {body}"));
    assert_eq!(
        json.get("success").and_then(|v| v.as_bool()),
        Some(false),
        "{what}: the repo-wide envelope requires success=false, body={body}"
    );
    assert_eq!(
        json.get("error").and_then(|v| v.as_str()),
        Some(expected_error),
        "{what}: stable, actionable error message, body={body}"
    );
}

/// EVERY room endpoint, reads and mutations alike. Reads are in the list on
/// purpose: a maid who can see the other property's room board is already
/// looking at the wrong hotel, one tap before she writes to it.
const ROOM_ENDPOINTS: &[(&str, &str, &str)] = &[
    ("GET", "/api/hk/rooms", ""),
    ("GET", "/api/hk/rooms/1", ""),
    ("POST", "/api/hk/rooms/1/cleaning", r#"{"status":"done"}"#),
    ("GET", "/api/hk/broken-items/1/photo", ""),
];

fn with_branch(uri: &str, branch: &str) -> String {
    format!("{uri}?branch={branch}")
}

// ============================================================================
// Row 1 — flag OFF is byte-identical to the pre-enforcement build
// ============================================================================

/// The dark posture. Enforcement off ⇒ NO lookup is issued at all, so a maid
/// whose badge would resolve to `Unavailable` still passes the gate. Asserted
/// negatively (not 403, not 503) because what happens next depends on the
/// database — the point is only that the LOCATION gate did not fire.
#[tokio::test]
async fn row1_flag_off_is_a_passthrough_for_every_endpoint_and_branch() {
    for (method, uri, body) in ROOM_ENDPOINTS {
        for branch in ["hfhotel", "hfville"] {
            let (status, resp) = call(
                dark(),
                // A badge the lookup would refuse, if it were ever consulted.
                maid(NULL_LOCATION_BADGE),
                method,
                &with_branch(uri, branch),
                body,
            )
            .await;
            assert_ne!(
                status,
                StatusCode::FORBIDDEN,
                "{method} {uri}?branch={branch} must not be location-refused while dark: {resp}"
            );
            assert_ne!(
                status,
                StatusCode::SERVICE_UNAVAILABLE,
                "{method} {uri}?branch={branch} must not 503 while dark: {resp}"
            );
        }
    }
}

/// `GET /api/hk/me` while dark: the whole allowlist, in `HK_BRANCHES` order,
/// and `branchesUnavailableReason` present-but-null. The field is ADDITIVE —
/// every pre-existing field keeps its value and shape.
#[tokio::test]
async fn row1b_flag_off_me_serves_the_whole_allowlist_unchanged() {
    let (status, body) = call(dark(), maid(NULL_LOCATION_BADGE), "GET", "/api/hk/me", "").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");

    assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(
        json.get("badge").and_then(|v| v.as_str()),
        Some(NULL_LOCATION_BADGE)
    );
    assert_eq!(
        json.get("markDirtyEnabled").and_then(|v| v.as_bool()),
        Some(false)
    );

    let branches = json
        .get("branches")
        .and_then(|v| v.as_array())
        .expect("branches");
    let ids: Vec<&str> = branches
        .iter()
        .filter_map(|b| b.get("id").and_then(|v| v.as_str()))
        .collect();
    assert_eq!(ids, vec!["hfhotel", "hfville"], "the allowlist, verbatim");

    assert!(
        json.get("branchesUnavailableReason").is_some(),
        "the field is always serialized so the client branches on its VALUE: {body}"
    );
    assert!(
        json["branchesUnavailableReason"].is_null(),
        "…and it is null while dark: {body}"
    );
}

// ============================================================================
// Rows 2-3 — flag ON: an exact match passes, a mismatch is 403
// ============================================================================

/// Row 2. The employee's OWN branch clears the gate on every endpoint.
#[tokio::test]
async fn row2_matching_branch_passes_on_every_endpoint() {
    for (badge, branch) in [("HOTEL-MAID", "hfhotel"), ("VILLE-MAID", "hfville")] {
        for (method, uri, body) in ROOM_ENDPOINTS {
            let (status, resp) = call(
                enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
                maid(badge),
                method,
                &with_branch(uri, branch),
                body,
            )
            .await;
            assert_ne!(
                status,
                StatusCode::FORBIDDEN,
                "{badge} on their OWN branch must pass: {method} {uri} → {resp}"
            );
            assert_ne!(
                status,
                StatusCode::SERVICE_UNAVAILABLE,
                "{badge} on their OWN branch must not 503: {method} {uri} → {resp}"
            );
        }
    }
}

/// Row 3. **The bug, closed.** An HF Ville maid asking for HF Hotel — and the
/// mirror image — is 403 on every endpoint, reads included, with an actionable
/// Thai message.
#[tokio::test]
async fn row3_mismatched_branch_is_403_on_every_endpoint() {
    for (badge, wrong_branch) in [("VILLE-MAID", "hfhotel"), ("HOTEL-MAID", "hfville")] {
        for (method, uri, body) in ROOM_ENDPOINTS {
            let (status, resp) = call(
                enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
                maid(badge),
                method,
                &with_branch(uri, wrong_branch),
                body,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{badge} must not act on {wrong_branch}: {method} {uri} → {resp}"
            );
            assert_envelope(
                &resp,
                LOCATION_MISMATCH_ERROR,
                &format!("{badge} → {method} {uri}?branch={wrong_branch}"),
            );
        }
    }
}

// ============================================================================
// Rows 4-6 — the three refusal classes
// ============================================================================

/// Row 4. A NULL location is 403 with the "no branch set — contact an admin"
/// message. It must NOT be coerced to a default branch: that coercion is
/// precisely the wrong-property bug, one layer down.
#[tokio::test]
async fn row4_null_location_is_403_on_every_endpoint_and_branch() {
    for (method, uri, body) in ROOM_ENDPOINTS {
        for branch in ["hfhotel", "hfville"] {
            let (status, resp) = call(
                enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
                maid(NULL_LOCATION_BADGE),
                method,
                &with_branch(uri, branch),
                body,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "a NULL location must refuse: {method} {uri}?branch={branch} → {resp}"
            );
            assert_envelope(
                &resp,
                LOCATION_UNKNOWN_ERROR,
                &format!("null location → {method} {uri}?branch={branch}"),
            );
        }
    }
}

/// Row 5. An unreachable lookup is 503 — NOT 403, and NOT a passthrough. The
/// distinction is the point: a transport blip must not read to a maid as "you
/// are not allowed here", and must not silently admit her either.
#[tokio::test]
async fn row5_unreachable_lookup_is_503_on_every_endpoint() {
    for (method, uri, body) in ROOM_ENDPOINTS {
        let (status, resp) = call(
            enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
            maid("SOME-OTHER-BADGE"), // MatrixLookup ⇒ Unavailable
            method,
            &with_branch(uri, "hfhotel"),
            body,
        )
        .await;
        assert_eq!(
            status,
            StatusCode::SERVICE_UNAVAILABLE,
            "an unreachable lookup must 503: {method} {uri} → {resp}"
        );
        assert_envelope(
            &resp,
            LOCATION_LOOKUP_UNAVAILABLE_ERROR,
            &format!("unreachable → {method} {uri}"),
        );
    }
}

/// Row 6. An INACTIVE (or pending-approval) employee is refused — 403, the
/// same definite-no class as a null location. HF ID has not vouched for them,
/// so their location is not an authority to file against a property.
#[tokio::test]
async fn row6_inactive_or_pending_employee_is_refused() {
    for badge in ["INACTIVE", "PENDING", "UNKNOWN-BADGE"] {
        let (status, resp) = call(
            enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
            maid(badge),
            "POST",
            "/api/hk/rooms/1/cleaning?branch=hfhotel",
            r#"{"status":"done"}"#,
        )
        .await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "{badge} must be refused: {resp}"
        );
        assert_envelope(&resp, LOCATION_UNKNOWN_ERROR, badge);
    }
}

/// Row 7. Enforcement ON with the lookup UNCONFIGURED (`HFID_LOCATION_URL` /
/// `HFID_RESOLVE_SECRET` not both set) fails CLOSED with 503. "Unconfigured"
/// and "allowed" must never be the same state — this is the row that would
/// have made a half-finished flip silently permissive.
#[tokio::test]
async fn row7_unconfigured_lookup_fails_closed_not_open() {
    for (method, uri, body) in ROOM_ENDPOINTS {
        let (status, resp) = call(
            enforcing_unconfigured(),
            maid("HOTEL-MAID"),
            method,
            &with_branch(uri, "hfhotel"),
            body,
        )
        .await;
        assert_eq!(
            status,
            StatusCode::SERVICE_UNAVAILABLE,
            "an unconfigured lookup must fail closed: {method} {uri} → {resp}"
        );
    }
}

// ============================================================================
// Row 8 — the allowlist still binds, and answers FIRST
// ============================================================================

/// The two gates compose, and `HK_BRANCHES` answers first. A Ville maid asking
/// for `hfville` while the deployment serves HF Hotel only gets the ALLOWLIST
/// 403 (`branch not enabled`), not the location 403 — so the operator reads
/// "widen HK_BRANCHES", which is the actual fix (V13).
#[tokio::test]
async fn row8_allowlist_answers_before_the_location_gate() {
    let (status, body) = call(
        enforcing(vec![Branch::Hfhotel]),
        maid("VILLE-MAID"),
        "GET",
        "/api/hk/rooms?branch=hfville",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
    assert_envelope(
        &body,
        BRANCH_NOT_ENABLED_ERROR,
        "an unserved property is an allowlist refusal, not a location one",
    );
}

// ============================================================================
// Rows 9-11 — `GET /api/hk/me` filtering + the reason field
// ============================================================================

fn me_branch_ids(json: &serde_json::Value) -> Vec<String> {
    json.get("branches")
        .and_then(|v| v.as_array())
        .expect("branches is an array")
        .iter()
        .filter_map(|b| b.get("id").and_then(|v| v.as_str()).map(str::to_string))
        .collect()
}

/// Row 9. `/me` serves the INTERSECTION: a Ville maid sees only วิลล์, a HF
/// Hotel maid only ฮาร์เบอร์ฟร้อนท์ — a single entry, which the client
/// auto-selects with no picker at all. No reason, because nothing is missing.
#[tokio::test]
async fn row9_me_filters_branches_to_the_employees_own_location() {
    for (badge, expected) in [("HOTEL-MAID", "hfhotel"), ("VILLE-MAID", "hfville")] {
        let (status, body) = call(
            enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
            maid(badge),
            "GET",
            "/api/hk/me",
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
        assert_eq!(me_branch_ids(&json), vec![expected.to_string()], "{body}");
        assert!(
            json["branchesUnavailableReason"].is_null(),
            "a resolved maid has no reason: {body}"
        );
        // The pre-existing fields are untouched.
        assert_eq!(json.get("success").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(json.get("badge").and_then(|v| v.as_str()), Some(badge));
    }
}

/// Row 10. No usable location ⇒ `branches: []` + `no_location`. `/me` still
/// answers 200: it is a status report, not a gate, and the client needs the
/// reason in order to render an actionable message instead of an empty picker.
#[tokio::test]
async fn row10_me_reports_no_location_with_an_empty_branch_list() {
    let (status, body) = call(
        enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
        maid(NULL_LOCATION_BADGE),
        "GET",
        "/api/hk/me",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
    assert!(
        me_branch_ids(&json).is_empty(),
        "no branch may be offered: {body}"
    );
    assert_eq!(
        json.get("branchesUnavailableReason")
            .and_then(|v| v.as_str()),
        Some(REASON_NO_LOCATION),
        "{body}"
    );
}

/// Row 11. An unreachable — or unconfigured — lookup ⇒ `branches: []` +
/// `lookup_unavailable`. The client must be able to tell this from
/// `no_location`: one says "contact an admin", the other says "try again".
#[tokio::test]
async fn row11_me_reports_lookup_unavailable_distinctly() {
    for policy in [
        enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
        enforcing_unconfigured(),
    ] {
        let (status, body) = call(policy, maid("SOME-OTHER-BADGE"), "GET", "/api/hk/me", "").await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
        assert!(me_branch_ids(&json).is_empty(), "{body}");
        assert_eq!(
            json.get("branchesUnavailableReason")
                .and_then(|v| v.as_str()),
            Some(REASON_LOOKUP_UNAVAILABLE),
            "{body}"
        );
    }
}

/// Row 12. Today's production shape, spelled out: an `HF_VILLE` employee while
/// `HK_BRANCHES` is hfhotel-only has NO admitted branch — `[]` + `no_location`,
/// never `hfhotel`. This is exactly why V13 (widen `HK_BRANCHES`) must follow
/// V14 (flip enforcement) rather than precede it, and why flipping enforcement
/// before Ville is admitted leaves Ville staff unable to work by DESIGN.
#[tokio::test]
async fn row12_ville_employee_has_no_branch_until_hk_branches_widens() {
    let (status, body) = call(
        enforcing(vec![Branch::Hfhotel]),
        maid("VILLE-MAID"),
        "GET",
        "/api/hk/me",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
    assert!(
        me_branch_ids(&json).is_empty(),
        "hfhotel must NOT be offered as a consolation branch: {body}"
    );
    assert_eq!(
        json.get("branchesUnavailableReason")
            .and_then(|v| v.as_str()),
        Some(REASON_NO_LOCATION),
        "a real but unserved property is not a retryable outage: {body}"
    );
}

// ============================================================================
// Row 13 — the surface is still behind auth
// ============================================================================

/// The gate must not become a way to probe employee configuration. Through the
/// SHIPPED router with no Access assertion, every probe is still 401/403 from
/// the outer layers — the location gate never runs, so no badge, branch or
/// reason leaks to an unauthenticated caller.
#[tokio::test]
async fn row13_unauthenticated_probes_never_reach_the_location_gate() {
    let state = AppState::new(lazy_pool());
    let app = hotel_backend::routes::hk::router_with_policy(
        state,
        enforcing(vec![Branch::Hfhotel, Branch::Hfville]),
    );
    for uri in [
        "/api/hk/me",
        "/api/hk/rooms?branch=hfhotel",
        "/api/hk/rooms?branch=hfville",
    ] {
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
            .expect("request builds");
        let status = app.clone().oneshot(req).await.expect("responds").status();
        assert!(
            status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN,
            "{uri}: unauthenticated probe must be refused by the outer layers, got {status}"
        );
        assert_ne!(
            status,
            StatusCode::SERVICE_UNAVAILABLE,
            "{uri}: the location gate must never run for an unauthenticated caller"
        );
    }
}

// ============================================================================
// Rows 14-19 — the `housekeeping_admin` grant
// ============================================================================

/// Row 14. An admin WITH a location is offered every served property, and may
/// act on all of them. The location on file is not consulted — the grant
/// replaces it rather than being intersected with it.
#[tokio::test]
async fn row14_admin_with_a_location_gets_every_served_branch() {
    let lookup = PayloadLookup::new();
    let (status, body) = call(
        enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], lookup.clone()),
        maid("ADMIN-LOCATED"),
        "GET",
        "/api/hk/me",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
    assert_eq!(
        me_branch_ids(&json),
        vec!["hfhotel".to_string(), "hfville".to_string()],
        "an admin whose location is HF must still be offered BOTH: {body}"
    );
    assert!(
        json["branchesUnavailableReason"].is_null(),
        "nothing is missing, so nothing to explain: {body}"
    );

    // …and every endpoint admits her on either branch.
    for (method, uri, req_body) in ROOM_ENDPOINTS {
        for branch in ["hfhotel", "hfville"] {
            let (status, resp) = call(
                enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], lookup.clone()),
                maid("ADMIN-LOCATED"),
                method,
                &with_branch(uri, branch),
                req_body,
            )
            .await;
            assert_ne!(
                status,
                StatusCode::FORBIDDEN,
                "an admin must not be location-refused: {method} {uri}?branch={branch} → {resp}"
            );
            assert_ne!(
                status,
                StatusCode::SERVICE_UNAVAILABLE,
                "an admin must not 503: {method} {uri}?branch={branch} → {resp}"
            );
        }
    }
}

/// Row 15. **The load-bearing row.** An admin whose `location` is NULL gets
/// every served property too — the grant overrides location ENTIRELY.
///
/// This is deliberately NOT the silent-default hole this stream closed. The
/// difference is where the answer comes from: a null location alone is an
/// ABSENCE and still refuses (row 4, and row 17 below on the very same badge),
/// whereas the grant is an explicit tick against a named employee in HF ID,
/// auditable there. Row 17 is what keeps the two apart.
#[tokio::test]
async fn row15_admin_with_a_null_location_still_gets_every_served_branch() {
    let lookup = PayloadLookup::new();
    let (status, body) = call(
        enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], lookup.clone()),
        maid("ADMIN-NULL"),
        "GET",
        "/api/hk/me",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
    assert_eq!(
        me_branch_ids(&json),
        vec!["hfhotel".to_string(), "hfville".to_string()],
        "a NULL location must not narrow a grant-holder: {body}"
    );

    for (method, uri, req_body) in ROOM_ENDPOINTS {
        for branch in ["hfhotel", "hfville"] {
            let (status, resp) = call(
                enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], lookup.clone()),
                maid("ADMIN-NULL"),
                method,
                &with_branch(uri, branch),
                req_body,
            )
            .await;
            assert_ne!(
                status,
                StatusCode::FORBIDDEN,
                "a NULL-location admin must be admitted: {method} {uri}?branch={branch} → {resp}"
            );
            assert_ne!(
                status,
                StatusCode::SERVICE_UNAVAILABLE,
                "a NULL-location admin must not 503: {method} {uri}?branch={branch} → {resp}"
            );
        }
    }
}

/// Row 16. **The allowlist is the outer bound and the grant cannot cross it.**
/// An admin asking for a branch `HK_BRANCHES` does not list is still 403 — and
/// with the ALLOWLIST message, not a location one, so the operator reads
/// "widen HK_BRANCHES" rather than going to look at an employee record that is
/// not the problem.
#[tokio::test]
async fn row16_admin_is_still_bounded_by_the_hk_branches_allowlist() {
    for badge in ["ADMIN-LOCATED", "ADMIN-NULL"] {
        for (method, uri, req_body) in ROOM_ENDPOINTS {
            let (status, resp) = call(
                enforcing_payloads(vec![Branch::Hfhotel], PayloadLookup::new()),
                maid(badge),
                method,
                &with_branch(uri, "hfville"),
                req_body,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{badge} must not reach an unserved property: {method} {uri} → {resp}"
            );
            assert_envelope(
                &resp,
                BRANCH_NOT_ENABLED_ERROR,
                &format!("{badge} → {method} {uri}?branch=hfville"),
            );
        }
        // …and `/me` offers only what the deployment serves.
        let (status, body) = call(
            enforcing_payloads(vec![Branch::Hfhotel], PayloadLookup::new()),
            maid(badge),
            "GET",
            "/api/hk/me",
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
        assert_eq!(
            me_branch_ids(&json),
            vec!["hfhotel".to_string()],
            "{badge}: a grant widens to the allowlist, never past it: {body}"
        );
    }
}

/// Row 17. **The darkness proof.** Today's LIVE payload for badge 421 — same
/// employee, same null location, `apps` WITHOUT the grant — behaves exactly as
/// it did before this feature existed: `[]` + `no_location` from `/me`, and
/// 403 `LOCATION_UNKNOWN_ERROR` on every room endpoint and branch.
///
/// Pair this with row 15: the two rows differ ONLY by one string in `apps`.
/// That is the whole capability, and it is why nothing changes until the owner
/// ticks the box.
#[tokio::test]
async fn row17_the_same_badge_without_the_grant_is_byte_unchanged() {
    let (status, body) = call(
        enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], PayloadLookup::new()),
        maid("NON-ADMIN-NULL"),
        "GET",
        "/api/hk/me",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
    assert!(
        me_branch_ids(&json).is_empty(),
        "no grant ⇒ no branch for a null location: {body}"
    );
    assert_eq!(
        json.get("branchesUnavailableReason")
            .and_then(|v| v.as_str()),
        Some(REASON_NO_LOCATION),
        "{body}"
    );

    for (method, uri, req_body) in ROOM_ENDPOINTS {
        for branch in ["hfhotel", "hfville"] {
            let (status, resp) = call(
                enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], PayloadLookup::new()),
                maid("NON-ADMIN-NULL"),
                method,
                &with_branch(uri, branch),
                req_body,
            )
            .await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "a non-holder with a null location must still refuse: \
                 {method} {uri}?branch={branch} → {resp}"
            );
            assert_envelope(
                &resp,
                LOCATION_UNKNOWN_ERROR,
                &format!("non-holder null location → {method} {uri}?branch={branch}"),
            );
        }
    }
}

/// Row 18. A located NON-holder is unchanged too: her own branch passes, the
/// other is 403 with the MISMATCH message. Re-asserted here against the
/// payload-driven lookup (rows 2-3 assert it against hand-written outcomes) so
/// the `apps`-consuming decode path is proven not to have disturbed the
/// ordinary maid — which is every maid, today.
#[tokio::test]
async fn row18_a_located_non_holder_is_unchanged_through_the_payload_path() {
    let policy =
        || enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], PayloadLookup::new());

    let (status, body) = call(policy(), maid("NON-ADMIN-LOCATED"), "GET", "/api/hk/me", "").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("body is JSON");
    assert_eq!(
        me_branch_ids(&json),
        vec!["hfhotel".to_string()],
        "a plain HF maid still sees exactly her own property: {body}"
    );

    for (method, uri, req_body) in ROOM_ENDPOINTS {
        let (status, resp) = call(
            policy(),
            maid("NON-ADMIN-LOCATED"),
            method,
            &with_branch(uri, "hfhotel"),
            req_body,
        )
        .await;
        assert_ne!(
            status,
            StatusCode::FORBIDDEN,
            "her own branch must pass: {method} {uri} → {resp}"
        );

        let (status, resp) = call(
            policy(),
            maid("NON-ADMIN-LOCATED"),
            method,
            &with_branch(uri, "hfville"),
            req_body,
        )
        .await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "the other property must still refuse: {method} {uri} → {resp}"
        );
        assert_envelope(
            &resp,
            LOCATION_MISMATCH_ERROR,
            &format!("non-holder → {method} {uri}?branch=hfville"),
        );
    }
}

/// Row 19. The 60s cache carries the GRANT, not just a location: a second
/// request for the same admin badge is served from cache and still offers both
/// properties, on ONE lookup. (The revocation direction — a widening must not
/// outlive the grant — is pinned by the TTL test in `src/hfid_location.rs`,
/// which can shorten the TTL; here the point is that the cached entry replays
/// the grant rather than decaying into a location or into nothing.)
#[tokio::test]
async fn row19_the_cache_carries_the_grant_alongside_the_location() {
    let lookup = PayloadLookup::new();
    let policy = enforcing_payloads(vec![Branch::Hfhotel, Branch::Hfville], lookup.clone());

    for attempt in 1..=3 {
        // One policy, therefore one client and one cache, across all three.
        let req = Request::builder()
            .method("GET")
            .uri("/api/hk/me")
            .body(Body::empty())
            .expect("request builds");
        let response = inner(policy.clone(), maid("ADMIN-NULL"))
            .oneshot(req)
            .await
            .expect("router responds");
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .expect("body reads");
        let json: serde_json::Value = serde_json::from_slice(&bytes).expect("body is JSON");
        assert_eq!(
            me_branch_ids(&json),
            vec!["hfhotel".to_string(), "hfville".to_string()],
            "attempt {attempt}: a cached grant must replay as the full allowlist"
        );
    }
    assert_eq!(
        lookup.calls(),
        1,
        "one LAN round-trip, then the cache — the grant is cached like a location"
    );
}
