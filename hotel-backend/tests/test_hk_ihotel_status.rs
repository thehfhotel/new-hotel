//! CR-1 — **iHOTEL WINS** the room-clean status the maid sees on `/hk`.
//!
//! ## What this pins
//!
//! The owner's decision (locked 2026-08-15): reception works the iHOTEL board,
//! so a maid must be shown legacy `HT_Rooms.Room_Clean`, with canonical
//! `ht_rooms_new.room_clean` demoted to a mirror. The unit tests in
//! `routes::hk` pin the merge RULES; this suite pins the WIRING — that
//! `GET /api/hk/rooms` and `GET /api/hk/rooms/{id}` actually serve the merged
//! value and the `legacyStatusStale` flag over real HTTP, through the real
//! handler stack, out of a real PostgreSQL row.
//!
//! Three properties, each a distinct way the feature could ship broken:
//!
//! 1. iHOTEL's value REPLACES the canonical one on both endpoints.
//! 2. An unreachable iHOTEL degrades to the canonical value with
//!    `legacyStatusStale: true` — and is **200, never 5xx**. A maid on a
//!    stairwell must never get an error page because a legacy server blinked.
//! 3. Divergence never reaches the maid: the payload carries no second
//!    opinion, only the one value she should act on.
//!
//! ## No MSSQL, ever
//!
//! The legacy read is injected as a scripted [`RoomCleanSource`], so CI needs
//! no SQL Server and cannot flake on one. The live-MSSQL path is covered by
//! the live verification in `docs/coexistence/PENDING-VERIFICATIONS.md`.
//!
//! ## Running
//! Needs `DATABASE_URL` (the handlers read `ht_rooms_new` before the merge
//! runs); SKIPS cleanly when PG is unreachable — same convention as
//! `tests/test_hk_branch_required.rs`.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Extension;
use hotel_backend::legacy_room_status::{RoomCleanOutcome, RoomCleanSource};
use hotel_backend::middleware::hk_access::HkIdentity;
use hotel_backend::routes::hk::HkPolicy;
use hotel_backend::routes::mode::{AppState, Branch};
use sqlx::{PgPool, Row as _};
use tower::ServiceExt; // for `oneshot`

/// Marker room number. Distinct from every other suite's markers so the
/// `--test-threads=1` shared-schema convention still leaves no cross-talk.
const ROOM_NO: &str = "ZT-CR1";

/// A scripted iHOTEL answer — the whole reason CI needs no SQL Server.
#[derive(Debug)]
struct ScriptedIhotel(RoomCleanOutcome);

#[async_trait]
impl RoomCleanSource for ScriptedIhotel {
    async fn room_clean(&self) -> RoomCleanOutcome {
        self.0.clone()
    }
}

fn ihotel_says(room_no: &str, is_clean: bool) -> RoomCleanOutcome {
    let mut map = HashMap::new();
    map.insert(room_no.to_string(), is_clean);
    RoomCleanOutcome::Available(map)
}

async fn try_pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").ok()?;
    PgPool::connect(&url).await.ok()
}

fn maid() -> HkIdentity {
    HkIdentity {
        badge: "Q1001".to_string(),
        display_name: None,
        email: None,
    }
}

/// The handler table with a verified identity injected where the Cloudflare
/// Access layer would have put one, and a scripted iHOTEL reader behind it.
fn app(state: AppState, outcome: RoomCleanOutcome) -> axum::Router {
    let policy = HkPolicy {
        branches: vec![Branch::Hfhotel],
        ..HkPolicy::default()
    }
    .with_legacy_room_clean(Branch::Hfhotel, Arc::new(ScriptedIhotel(outcome)));
    hotel_backend::routes::hk::routes_inside_access(state, policy).layer(Extension(maid()))
}

async fn get_json(app: axum::Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .expect("request builds");
    let response = app.oneshot(req).await.expect("router responds");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body reads");
    let json = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| panic!("body must be JSON: {}", String::from_utf8_lossy(&bytes)));
    (status, json)
}

/// Seed ONE active room with an explicit canonical `room_clean`, returning its
/// id. `room_clean` is deliberately the OPPOSITE of what the scripted iHOTEL
/// will say in the merge tests, so a merge that silently no-ops is a failure
/// rather than an accidental pass.
async fn seed_room(pool: &PgPool, canonical_clean: bool) -> i32 {
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(ROOM_NO)
        .execute(pool)
        .await;
    let row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
         VALUES ($1, $2, true) RETURNING room_id",
    )
    .bind(ROOM_NO)
    .bind(canonical_clean)
    .fetch_one(pool)
    .await
    .expect("seed insert must succeed");
    row.try_get("room_id").expect("room_id")
}

async fn cleanup(pool: &PgPool) {
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(ROOM_NO)
        .execute(pool)
        .await;
}

/// Pluck our marker room out of the list payload.
fn seeded(list: &serde_json::Value) -> &serde_json::Value {
    list["data"]
        .as_array()
        .expect("data is an array")
        .iter()
        .find(|r| r["roomNo"] == ROOM_NO)
        .expect("the seeded room must be listed")
}

/// Property 1 — canonical says CLEAN, iHOTEL says DIRTY, the maid sees DIRTY.
///
/// This is the live-observable shape of the change: a room checked out in
/// iHOTEL whose CT mirror has not landed yet. Before CR-1 the maid saw
/// "clean" and walked past it.
#[tokio::test]
async fn ihotel_dirty_beats_canonical_clean_on_both_endpoints() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping ihotel_dirty_beats_canonical_clean_on_both_endpoints — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, true).await;
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state.clone(), ihotel_says(ROOM_NO, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        seeded(&list)["roomClean"],
        serde_json::json!(false),
        "the list must show iHOTEL's answer, not the canonical mirror: {list}"
    );
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(false),
        "iHOTEL answered — nothing is stale: {list}"
    );

    let (status, detail) = get_json(
        app(state, ihotel_says(ROOM_NO, false)),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        detail["room"]["roomClean"],
        serde_json::json!(false),
        "detail and list must never tell the maid different stories: {detail}"
    );
    assert_eq!(detail["legacyStatusStale"], serde_json::json!(false));

    cleanup(&pool).await;
}

/// Property 1, the other direction — canonical says DIRTY, iHOTEL says CLEAN.
/// Pinned separately so a merge that just forced everything to `false` (or
/// inverted the polarity) cannot pass.
#[tokio::test]
async fn ihotel_clean_beats_canonical_dirty() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping ihotel_clean_beats_canonical_dirty — PG not reachable");
        return;
    };
    seed_room(&pool, false).await;
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state, ihotel_says(ROOM_NO, true)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(seeded(&list)["roomClean"], serde_json::json!(true), "{list}");
    assert_eq!(list["legacyStatusStale"], serde_json::json!(false));

    cleanup(&pool).await;
}

/// Property 2 — the fallback the owner locked. An unreachable iHOTEL must
/// produce a USABLE list: 200, canonical values untouched, and the flag that
/// makes the client render its Thai note. Stale-but-shown beats dead screen.
#[tokio::test]
async fn unreachable_ihotel_serves_the_canonical_mirror_with_the_stale_flag() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping unreachable_ihotel_serves_the_canonical_mirror_with_the_stale_flag — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, false).await;
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state.clone(), RoomCleanOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "a legacy outage must NEVER become an error page: {list}"
    );
    assert_eq!(list["success"], serde_json::json!(true));
    assert_eq!(
        seeded(&list)["roomClean"],
        serde_json::json!(false),
        "the canonical value must be served verbatim: {list}"
    );
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(true),
        "the maid must be TOLD she is looking at the mirror: {list}"
    );

    let (status, detail) = get_json(
        app(state, RoomCleanOutcome::Unavailable),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(detail["legacyStatusStale"], serde_json::json!(true), "{detail}");

    cleanup(&pool).await;
}

/// A branch with NO reader attached behaves exactly like an unreachable one.
/// This is the state the surface SHIPS in for any branch `main.rs` could not
/// wire (no legacy pool, no Ville password) — it must be a degraded display,
/// never a 500.
#[tokio::test]
async fn a_branch_without_a_reader_is_the_same_degraded_display() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_branch_without_a_reader_is_the_same_degraded_display — PG not reachable");
        return;
    };
    seed_room(&pool, true).await;

    // No `with_legacy_room_clean` at all.
    let policy = HkPolicy {
        branches: vec![Branch::Hfhotel],
        ..HkPolicy::default()
    };
    let router = hotel_backend::routes::hk::routes_inside_access(AppState::new(pool.clone()), policy)
        .layer(Extension(maid()));

    let (status, list) = get_json(router, "/api/hk/rooms?branch=hfhotel").await;
    assert_eq!(status, StatusCode::OK, "{list}");
    assert_eq!(list["legacyStatusStale"], serde_json::json!(true), "{list}");
    assert_eq!(seeded(&list)["roomClean"], serde_json::json!(true), "{list}");

    cleanup(&pool).await;
}

/// Property 3 — divergence is an operator signal and must not reach the wire.
/// The maid gets ONE value to act on; a second opinion on her screen is a
/// question she cannot answer.
#[tokio::test]
async fn the_maid_never_receives_the_canonical_second_opinion() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_maid_never_receives_the_canonical_second_opinion — PG not reachable");
        return;
    };
    seed_room(&pool, true).await;
    let state = AppState::new(pool.clone());

    let (_, list) = get_json(
        app(state, ihotel_says(ROOM_NO, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    let body = list.to_string().to_lowercase();
    for leaked in ["diverg", "pmsclean", "canonicalclean", "ihotelclean"] {
        assert!(
            !body.contains(leaked),
            "the payload must carry no second opinion ({leaked}): {body}"
        );
    }

    cleanup(&pool).await;
}
