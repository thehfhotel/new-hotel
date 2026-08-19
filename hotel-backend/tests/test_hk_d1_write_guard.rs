//! Defect D1 — the maid's TAP must be judged by the same truth her SCREEN was.
//!
//! ## The defect this pins closed
//!
//! `/hk`'s display has been iHOTEL-wins since CR-1, but the write guard judged
//! canonical `ht_rooms_new.room_clean` — the CT mirror. When the mirror lagged
//! iHOTEL, the two disagreed in the one place it mattered:
//!
//! 1. iHOTEL says room 104 is DIRTY (guest checked out), the CT mirror has not
//!    caught up and still says CLEAN.
//! 2. The maid's list shows DIRTY (iHOTEL wins) — correctly.
//! 3. She cleans it and taps เสร็จแล้ว.
//! 4. The guard reads CANONICAL, sees "already clean", enqueues nothing,
//!    publishes nothing, and answers `success: true` → บันทึกแล้ว.
//! 5. Nothing ever reaches iHOTEL. Reception's board still shows the room
//!    dirty; the maid believes she recorded it. Silent loss.
//!
//! The 0a30079 read-sync fix made this EASIER to hit, not harder: the display
//! now refreshes promptly, so the maid acts on fresh iHOTEL truth sooner —
//! deeper inside the window where canonical still disagrees.
//!
//! ## What is asserted here, over real HTTP
//!
//! The unit tests in `service::housekeeping` pin the DECISION and the ones in
//! `routes::hk` pin the HINT. This suite pins the WIRING: that a real
//! `POST /api/hk/rooms/{id}/cleaning`, through the real handler stack, against
//! a real PostgreSQL row, actually enqueues the writeback under a lagging
//! mirror — and still does not double-write, still degrades to the pre-D1
//! behaviour when iHOTEL cannot be reached, and never fails a maid's tap.
//!
//! ## No MSSQL, ever
//!
//! The legacy read is injected as a scripted [`RoomFlagsSource`], exactly as
//! `tests/test_hk_ihotel_status.rs` does for the read path — CI needs no SQL
//! Server and cannot flake on one. The recipes are untouched by this change, so
//! there is no new byte-parity surface to verify live.
//!
//! ## Running
//! Needs `DATABASE_URL`; SKIPS cleanly when PG is unreachable.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Extension;
use hotel_backend::legacy_room_status::{LegacyRoomFlags, RoomFlagsOutcome, RoomFlagsSource};
use hotel_backend::middleware::hk_access::HkIdentity;
use hotel_backend::routes::hk::HkPolicy;
use hotel_backend::routes::mode::{AppState, Branch};
use hotel_backend::service::ids::{aggregate_uuid, AggregateKind};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

/// Marker room number, distinct from every other suite's (the `--test-threads=1`
/// shared-schema convention).
const ROOM_NO: &str = "ZT-D1W";

#[derive(Debug)]
struct ScriptedIhotel(RoomFlagsOutcome);

#[async_trait]
impl RoomFlagsSource for ScriptedIhotel {
    async fn room_flags(&self) -> RoomFlagsOutcome {
        self.0.clone()
    }
}

/// The CLEANLINESS fact only — `Room_Use` is left UNKNOWN, so this suite
/// keeps proving that the D1 write guard decides on cleanliness alone and the
/// widened CR-1 read gave it no new input.
fn ihotel_says(is_clean: bool) -> RoomFlagsOutcome {
    let mut map = HashMap::new();
    map.insert(
        ROOM_NO.to_string(),
        LegacyRoomFlags {
            is_clean: Some(is_clean),
            occupied: None,
        },
    );
    RoomFlagsOutcome::Available(map)
}

async fn try_pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").ok()?;
    PgPool::connect(&url).await.ok()
}

fn maid() -> HkIdentity {
    HkIdentity {
        badge: "Q1001".to_string(),
        display_name: Some("นก".to_string()),
        email: None,
    }
}

/// The handler table with a verified identity injected where Cloudflare Access
/// would have put one, and a scripted iHOTEL behind it.
///
/// `mark_dirty_enabled` is passed explicitly: the `dirty` pole stays behind
/// `HK_MARK_DIRTY_ENABLED` (invariant #6) and D1 does not widen that gate.
fn app(state: AppState, outcome: RoomFlagsOutcome, mark_dirty_enabled: bool) -> axum::Router {
    let policy = HkPolicy {
        branches: vec![Branch::Hfhotel],
        mark_dirty_enabled,
        ..HkPolicy::default()
    }
    .with_legacy_room_flags(Branch::Hfhotel, Arc::new(ScriptedIhotel(outcome)));
    hotel_backend::routes::hk::routes_inside_access(state, policy).layer(Extension(maid()))
}

async fn post_cleaning(app: axum::Router, room_id: i32, status: &str) -> (StatusCode, serde_json::Value) {
    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/hk/rooms/{room_id}/cleaning?branch=hfhotel"))
        .header("content-type", "application/json")
        .body(Body::from(format!(r#"{{"status":"{status}"}}"#)))
        .expect("request builds");
    let response = app.oneshot(req).await.expect("router responds");
    let status_code = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body reads");
    let json = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| panic!("body must be JSON: {}", String::from_utf8_lossy(&bytes)));
    (status_code, json)
}

/// Seed ONE active room with an explicit canonical `room_clean`.
async fn seed_room(pool: &PgPool, canonical_clean: bool) -> i32 {
    cleanup(pool).await;
    sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
         VALUES ($1, $2, true) RETURNING room_id",
    )
    .bind(ROOM_NO)
    .bind(canonical_clean)
    .fetch_one(pool)
    .await
    .expect("seed insert must succeed")
}

async fn job_count(pool: &PgPool, room_id: i32, intent: &str) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM writeback_jobs WHERE aggregate_id = $1 AND intent = $2")
        .bind(aggregate_uuid(AggregateKind::Room, room_id))
        .bind(intent)
        .fetch_one(pool)
        .await
        .expect("job count")
}

async fn canonical_clean(pool: &PgPool, room_id: i32) -> Option<bool> {
    sqlx::query_scalar("SELECT room_clean FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .fetch_one(pool)
        .await
        .expect("room row")
}

async fn cleanup(pool: &PgPool) {
    let ids: Vec<i32> = sqlx::query_scalar("SELECT room_id FROM ht_rooms_new WHERE room_no = $1")
        .bind(ROOM_NO)
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    for room_id in ids {
        let agg = aggregate_uuid(AggregateKind::Room, room_id);
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(agg)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(agg)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_hk_cleaning_events WHERE hkev_room_id = $1")
            .bind(room_id)
            .execute(pool)
            .await;
    }
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(ROOM_NO)
        .execute(pool)
        .await;
}

/// THE DEFECT, end to end. Canonical CLEAN (mirror behind), iHOTEL DIRTY (what
/// the maid's screen showed her). Her tap must now produce a real
/// `mark_room_clean` writeback instead of a silent no-op with a บันทึกแล้ว.
#[tokio::test]
async fn a_tap_on_a_room_ihotel_calls_dirty_reaches_ihotel() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_tap_on_a_room_ihotel_calls_dirty_reaches_ihotel — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, true).await;
    let state = AppState::new(pool.clone());

    let (status, body) = post_cleaning(
        app(state, ihotel_says(false), false),
        room_id,
        "done",
    )
    .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body["writebackEnqueued"],
        serde_json::json!(true),
        "the maid acted on iHOTEL's DIRTY — her tap must reach iHOTEL: {body}"
    );
    assert_eq!(
        job_count(&pool, room_id, "mark_room_clean").await,
        1,
        "exactly one MarkRoomClean job must be queued"
    );
    assert_eq!(
        canonical_clean(&pool, room_id).await,
        Some(true),
        "canonical already matched the tap — a repair flips nothing"
    );

    cleanup(&pool).await;
}

/// The fallback, end to end: iHOTEL unreachable ⇒ canonical-only judgement,
/// i.e. EXACTLY the pre-D1 behaviour — same failure surface as today, never
/// worse. The tap is still 200 and still `success: true`: a maid's tap is never
/// failed, never blocked, and never made to wait on a legacy server being up.
#[tokio::test]
async fn an_unreachable_ihotel_degrades_to_todays_behaviour_and_never_fails_the_tap() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping an_unreachable_ihotel_degrades_to_todays_behaviour — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, true).await;
    let state = AppState::new(pool.clone());

    let (status, body) = post_cleaning(
        app(state, RoomFlagsOutcome::Unavailable, false),
        room_id,
        "done",
    )
    .await;

    assert_eq!(
        status,
        StatusCode::OK,
        "a legacy outage must never become an error on a maid's tap: {body}"
    );
    assert_eq!(body["success"], serde_json::json!(true), "{body}");
    assert_eq!(
        body["writebackEnqueued"],
        serde_json::json!(false),
        "with no iHOTEL opinion the guard judges canonical alone: {body}"
    );
    assert_eq!(job_count(&pool, room_id, "mark_room_clean").await, 0);

    cleanup(&pool).await;
}

/// Invariant #4 under the NEW logic: iHOTEL keeps answering "dirty" until our
/// own job drains, so a maid double-tapping เสร็จแล้ว would force a second
/// `HT_Housewife` audit row if the repair were unguarded. It must not.
#[tokio::test]
async fn a_double_tap_under_a_lagging_mirror_still_enqueues_exactly_one_job() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_double_tap_under_a_lagging_mirror — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, true).await;
    let state = AppState::new(pool.clone());

    for tap in 1..=4 {
        let (status, body) = post_cleaning(
            app(state.clone(), ihotel_says(false), false),
            room_id,
            "done",
        )
        .await;
        assert_eq!(status, StatusCode::OK, "tap {tap}: {body}");
        assert_eq!(
            body["writebackEnqueued"],
            serde_json::json!(tap == 1),
            "only the FIRST tap may enqueue (tap {tap}): {body}"
        );
    }

    assert_eq!(
        job_count(&pool, room_id, "mark_room_clean").await,
        1,
        "exactly one writeback across four taps"
    );
    // The maid's log is append-only — every tap is still recorded, only the
    // legacy write is deduplicated.
    let events: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ht_hk_cleaning_events WHERE hkev_room_id = $1")
            .bind(room_id)
            .fetch_one(&pool)
            .await
            .expect("event count");
    assert_eq!(events, 4, "all four taps stay in the maid's own audit trail");

    cleanup(&pool).await;
}

/// The ordinary path must be untouched: a genuinely dirty room still
/// transitions and still mirrors — even when iHOTEL already believes it is
/// clean. A real canonical transition is never suppressed by the new logic.
#[tokio::test]
async fn a_genuinely_dirty_room_still_transitions_and_mirrors() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_genuinely_dirty_room_still_transitions — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, false).await;
    let state = AppState::new(pool.clone());

    let (status, body) = post_cleaning(app(state, ihotel_says(true), false), room_id, "done").await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["writebackEnqueued"], serde_json::json!(true), "{body}");
    assert_eq!(job_count(&pool, room_id, "mark_room_clean").await, 1);
    assert_eq!(canonical_clean(&pool, room_id).await, Some(true));

    cleanup(&pool).await;
}

/// The symmetric pole, end to end. Canonical already DIRTY, iHOTEL still shows
/// the room CLEAN — so ห้องยังไม่สะอาด must reach iHOTEL's grid rather than
/// no-op. Still gated by `HK_MARK_DIRTY_ENABLED`: D1 makes the tap honest, it
/// does not widen the dark-ship gate.
#[tokio::test]
async fn the_dirty_pole_is_repaired_symmetrically_and_stays_behind_its_flag() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_dirty_pole_is_repaired_symmetrically — PG not reachable");
        return;
    };
    let room_id = seed_room(&pool, false).await;
    let state = AppState::new(pool.clone());

    // Flag OFF ⇒ 403 before any of this logic is reached, unchanged by D1.
    let (status, body) = post_cleaning(
        app(state.clone(), ihotel_says(true), false),
        room_id,
        "dirty",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "the mark-dirty gate must still hold: {body}"
    );
    assert_eq!(job_count(&pool, room_id, "mark_room_dirty").await, 0);

    // Flag ON ⇒ the mirror-image repair fires.
    let (status, body) = post_cleaning(
        app(state.clone(), ihotel_says(true), true),
        room_id,
        "dirty",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body["writebackEnqueued"],
        serde_json::json!(true),
        "iHOTEL showed the room clean — the tap must reach its grid: {body}"
    );
    assert_eq!(job_count(&pool, room_id, "mark_room_dirty").await, 1);
    assert_eq!(canonical_clean(&pool, room_id).await, Some(false));

    // And it is idempotent on the same double-tap argument.
    let (_, body) = post_cleaning(app(state, ihotel_says(true), true), room_id, "dirty").await;
    assert_eq!(body["writebackEnqueued"], serde_json::json!(false), "{body}");
    assert_eq!(job_count(&pool, room_id, "mark_room_dirty").await, 1);

    cleanup(&pool).await;
}
