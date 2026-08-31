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
//! ## The DERIVED facts (second half of this file)
//!
//! `occupancy` merges the same way from `HT_Rooms.Room_Use`, but its FALLBACK
//! is derived per fetch from active checkins rather than read from a mirror
//! column — and `expectedArrival` / `expectedDeparture` are derived-only, with
//! no legacy counterpart at all. Those are SQL claims, so the second half of
//! this file seeds real rows (customer, rooms, folios, junction lines,
//! bookings) instead of scripting a reader.
//!
//! ## No MSSQL, ever
//!
//! The legacy read is injected as a scripted [`RoomFlagsSource`], so CI needs
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
use hotel_backend::legacy_room_status::{LegacyRoomFlags, RoomFlagsOutcome, RoomFlagsSource};
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
struct ScriptedIhotel(RoomFlagsOutcome);

#[async_trait]
impl RoomFlagsSource for ScriptedIhotel {
    async fn room_flags(&self) -> RoomFlagsOutcome {
        self.0.clone()
    }
}

/// iHOTEL answering the CLEANLINESS fact only — `Room_Use` UNKNOWN, so the
/// pre-existing suites keep testing the cleanliness rules in isolation.
fn ihotel_says(room_no: &str, is_clean: bool) -> RoomFlagsOutcome {
    let mut map = HashMap::new();
    map.insert(
        room_no.to_string(),
        LegacyRoomFlags {
            is_clean: Some(is_clean),
            occupied: None,
        },
    );
    RoomFlagsOutcome::Available(map)
}

/// iHOTEL answering the OCCUPANCY fact only (`Room_Use`), cleanliness UNKNOWN.
fn ihotel_occupancy(room_no: &str, occupied: bool) -> RoomFlagsOutcome {
    let mut map = HashMap::new();
    map.insert(
        room_no.to_string(),
        LegacyRoomFlags {
            is_clean: None,
            occupied: Some(occupied),
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
        display_name: None,
        email: None,
        can_report: true,
    }
}

/// The handler table with a verified identity injected where the Cloudflare
/// Access layer would have put one, and a scripted iHOTEL reader behind it.
fn app(state: AppState, outcome: RoomFlagsOutcome) -> axum::Router {
    let policy = HkPolicy {
        branches: vec![Branch::Hfhotel],
        ..HkPolicy::default()
    }
    .with_legacy_room_flags(Branch::Hfhotel, Arc::new(ScriptedIhotel(outcome)));
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
        app(state.clone(), RoomFlagsOutcome::Unavailable),
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
        app(state, RoomFlagsOutcome::Unavailable),
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

    // No `with_legacy_room_flags` at all.
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

// ============================================================================
// Occupancy + arrival/departure — the DERIVED canonical facts, through real PG
// ============================================================================
//
// `occupancy` falls back to a value DERIVED from active checkins, and the two
// planning tags are derived-only with no legacy counterpart at all. None of
// that can be proven against a scripted reader: it is SQL, so it needs rows.
//
// Every test here seeds its own fixture under `OCC_MARKER` and tears it down at
// both ends, so a crashed run self-heals on the next one (the `--test-threads=1`
// shared-schema convention).

/// Marker stamped on every note column this section writes, so cleanup is
/// exact-match and can never touch another suite's rows.
const OCC_MARKER: &str = "ZT-CR1-OCC";

/// Room numbers (`VARCHAR(10)`), one per scenario.
const R_STAY: &str = "ZO-STAY";
const R_OUT: &str = "ZO-OUT";
const R_NOCR: &str = "ZO-NOCR";
const R_ARR: &str = "ZO-ARR";
const R_DEP: &str = "ZO-DEP";
const OCC_ROOMS: [&str; 5] = [R_STAY, R_OUT, R_NOCR, R_ARR, R_DEP];

/// TODAY as the SQL under test computes it.
///
/// Deliberately NOT `chrono::Utc::now().date_naive()` (nor `Local`): between
/// 17:00 and 24:00 UTC the Bangkok civil day is already tomorrow, and a test
/// that built its fixture off the wrong one would go red every evening — or,
/// worse, go green against a `CURRENT_DATE` implementation that is wrong for
/// those seven hours. Asking PG the same question the predicate asks IS the
/// midnight-boundary assertion.
async fn bkk_today(pool: &PgPool) -> chrono::NaiveDate {
    sqlx::query_scalar("SELECT (NOW() AT TIME ZONE 'Asia/Bangkok')::date")
        .fetch_one(pool)
        .await
        .expect("PG answers its own date expression")
}

/// FK order: checkins (cascades `ht_checkin_rooms`) → bookings (cascades
/// `ht_booking_rooms`) → rooms → customer.
async fn occ_cleanup(pool: &PgPool) {
    for sql in [
        "DELETE FROM ht_checkins WHERE cin_notes = $1",
        "DELETE FROM ht_bookings WHERE book_notes = $1",
        "DELETE FROM ht_rooms_new WHERE room_notes = $1",
        "DELETE FROM ht_customers WHERE cust_notes = $1",
    ] {
        let _ = sqlx::query(sql).bind(OCC_MARKER).execute(pool).await;
    }
}

/// Customer + the five marker rooms, all active and canonically CLEAN.
/// Returns `(cust_id, room_no → room_id)`.
async fn occ_fixture(pool: &PgPool) -> (i32, HashMap<&'static str, i32>) {
    occ_cleanup(pool).await;
    let cust_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname, cust_notes) \
         VALUES ('ZT_CR1_OCC', $1) RETURNING cust_id",
    )
    .bind(OCC_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed customer");

    let mut rooms = HashMap::new();
    for room_no in OCC_ROOMS {
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active, room_notes) \
             VALUES ($1, true, true, $2) RETURNING room_id",
        )
        .bind(room_no)
        .bind(OCC_MARKER)
        .fetch_one(pool)
        .await
        .expect("seed room");
        rooms.insert(room_no, room_id);
    }
    (cust_id, rooms)
}

/// An ACTIVE folio (`cin_checkout_time IS NULL`) whose header room is
/// `cin_room_id`, due out on `expected_checkout`.
async fn occ_checkin(
    pool: &PgPool,
    cust_id: i32,
    room_id: i32,
    cin_no: &str,
    expected_checkout: chrono::NaiveDate,
) -> i32 {
    sqlx::query_scalar(
        "INSERT INTO ht_checkins \
           (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, \
            cin_status, cin_notes) \
         VALUES ($1, $2, $3, NOW() - INTERVAL '1 day', $4, 'active', $5) \
         RETURNING cin_id",
    )
    .bind(cin_no)
    .bind(cust_id)
    .bind(room_id)
    .bind(expected_checkout)
    .bind(OCC_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed checkin")
}

/// One `ht_checkin_rooms` line. `status` is the literal under test — the point
/// of the NOT-IN rule is that it mixes CT-mirrored legacy spellings with ours.
async fn occ_cr(pool: &PgPool, cin_id: i32, room_id: i32, status: &str) {
    sqlx::query(
        "INSERT INTO ht_checkin_rooms \
           (cr_cin_id, cr_room_id, cr_room_status, cr_rate_per_night, cr_nights, cr_room_total) \
         VALUES ($1, $2, $3, 0, 1, 0)",
    )
    .bind(cin_id)
    .bind(room_id)
    .bind(status)
    .execute(pool)
    .await
    .expect("seed checkin-room line");
}

/// A booking whose stay starts on `checkin` and runs one night.
async fn occ_booking(
    pool: &PgPool,
    cust_id: i32,
    book_no: &str,
    checkin: chrono::NaiveDate,
    status: &str,
) -> i32 {
    sqlx::query_scalar(
        "INSERT INTO ht_bookings \
           (book_no, book_cust_id, book_checkin, book_checkout, book_status, book_notes) \
         VALUES ($1, $2, $3, $3 + 1, $4, $5) RETURNING book_id",
    )
    .bind(book_no)
    .bind(cust_id)
    .bind(checkin)
    .bind(status)
    .bind(OCC_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed booking")
}

async fn occ_booking_room(pool: &PgPool, book_id: i32, room_id: i32) {
    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(book_id)
        .bind(room_id)
        .execute(pool)
        .await
        .expect("seed booking-room assignment");
}

/// Pluck one marker room out of a list payload by `roomNo`.
fn room_in<'a>(list: &'a serde_json::Value, room_no: &str) -> &'a serde_json::Value {
    list["data"]
        .as_array()
        .expect("data is an array")
        .iter()
        .find(|r| r["roomNo"] == room_no)
        .unwrap_or_else(|| panic!("room {room_no} must be listed: {list}"))
}

/// The DERIVED occupancy fallback, all three shapes at once, with iHOTEL
/// UNREACHABLE so the canonical value reaches the wire untouched.
///
/// This is also delta (e): a `legacyStatusStale: true` response must still
/// carry the derived occupancy, not a blank or a default. The fallback is the
/// state this surface ships in for any branch without a reader.
#[tokio::test]
async fn stale_legacy_leaves_occupancy_on_the_derived_value() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping stale_legacy_leaves_occupancy_on_the_derived_value — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;

    // 1. Active folio, this room's line is the legacy in-house literal.
    let cin_stay = occ_checkin(&pool, cust_id, rooms[R_STAY], "ZOCC-STAY", today).await;
    occ_cr(&pool, cin_stay, rooms[R_STAY], "เข้าพัก").await;

    // 2. Active folio (someone else is still in it), but THIS room's line has
    //    checked out. The room is free even though the folio is not closed —
    //    the case `LIVE_ROOM_FLAGS_SQL` gets wrong.
    let cin_out = occ_checkin(&pool, cust_id, rooms[R_OUT], "ZOCC-OUT", today).await;
    occ_cr(&pool, cin_out, rooms[R_OUT], "Check-Out").await;
    occ_cr(&pool, cin_out, rooms[R_STAY], "เข้าพัก").await;

    // 3. Pre-B5 shape: an active folio with NO junction rows at all, so
    //    `cin_room_id` is the only pointer to the occupied room.
    occ_checkin(&pool, cust_id, rooms[R_NOCR], "ZOCC-NOCR", today).await;

    let (status, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{list}");
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(true),
        "iHOTEL is unreachable in this test: {list}"
    );

    assert_eq!(
        room_in(&list, R_STAY)["occupancy"],
        serde_json::json!("occupied"),
        "an active folio line in 'เข้าพัก' is OCCUPIED: {list}"
    );
    assert_eq!(
        room_in(&list, R_OUT)["occupancy"],
        serde_json::json!("vacant"),
        "a 'Check-Out' line is VACANT even under a still-active folio: {list}"
    );
    assert_eq!(
        room_in(&list, R_NOCR)["occupancy"],
        serde_json::json!("occupied"),
        "a folio with no junction rows still occupies its cin_room_id room: {list}"
    );

    occ_cleanup(&pool).await;
}

/// Both alternative checkout spellings free the room. `'Check Out'` (no
/// hyphen) is FrmCheckOut.cs:6246's known iHOTEL inconsistency; `'ยกเลิก'` is
/// the cancel literal. Missing either would show a maid an occupied room she
/// could have cleaned.
#[tokio::test]
async fn every_checkout_spelling_frees_the_room() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping every_checkout_spelling_frees_the_room — PG not reachable");
        return;
    };
    for literal in ["Check-Out", "Check Out", "ยกเลิก"] {
        let (cust_id, rooms) = occ_fixture(&pool).await;
        let today = bkk_today(&pool).await;
        let cin = occ_checkin(&pool, cust_id, rooms[R_OUT], "ZOCC-SPELL", today).await;
        occ_cr(&pool, cin, rooms[R_OUT], literal).await;

        let (_, list) = get_json(
            app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
            "/api/hk/rooms?branch=hfhotel",
        )
        .await;
        assert_eq!(
            room_in(&list, R_OUT)["occupancy"],
            serde_json::json!("vacant"),
            "cr_room_status {literal:?} must free the room: {list}"
        );
        occ_cleanup(&pool).await;
    }
}

/// A `cr_room_status` we do not recognise, under an ACTIVE folio, biases
/// OCCUPIED. A maid walking in on a guest is the failure to avoid; a room shown
/// occupied that is actually empty costs one door knock.
#[tokio::test]
async fn an_unrecognised_room_status_under_an_active_folio_reads_occupied() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping an_unrecognised_room_status_under_an_active_folio_reads_occupied — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;
    let cin = occ_checkin(&pool, cust_id, rooms[R_STAY], "ZOCC-JUNK", today).await;
    occ_cr(&pool, cin, rooms[R_STAY], "active").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        room_in(&list, R_STAY)["occupancy"],
        serde_json::json!("occupied"),
        "our own 'active' literal — and anything else — must not free the room: {list}"
    );

    occ_cleanup(&pool).await;
}

/// CR-1 rule 1 for the SECOND fact, end to end: iHOTEL says `Room_Use='no'`
/// for a room PG derived as OCCUPIED, and the maid sees `vacant` on BOTH
/// endpoints.
///
/// This is a live shape, not a hypothetical: HF Ville room 106 on 2026-08-19
/// had exactly this disagreement (active folio in PG, `Room_Use='no'` in
/// iHOTEL). Reception works the iHOTEL board, so iHOTEL wins.
#[tokio::test]
async fn ihotel_room_use_beats_the_derived_occupancy_on_both_endpoints() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping ihotel_room_use_beats_the_derived_occupancy_on_both_endpoints — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;
    let cin = occ_checkin(&pool, cust_id, rooms[R_STAY], "ZOCC-WINS", today).await;
    occ_cr(&pool, cin, rooms[R_STAY], "เข้าพัก").await;
    let room_id = rooms[R_STAY];
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state.clone(), ihotel_occupancy(R_STAY, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(false),
        "iHOTEL answered — nothing is stale: {list}"
    );
    assert_eq!(
        room_in(&list, R_STAY)["occupancy"],
        serde_json::json!("vacant"),
        "iHOTEL Room_Use='no' must beat the canonical derived OCCUPIED: {list}"
    );

    let (status, detail) = get_json(
        app(state, ihotel_occupancy(R_STAY, false)),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        detail["room"]["occupancy"],
        serde_json::json!("vacant"),
        "detail and list must never tell the maid different stories: {detail}"
    );

    occ_cleanup(&pool).await;
}

/// The arrival tag: a booking on this room whose stay starts TODAY and is
/// still `'confirmed'` / `'pending'`.
///
/// Every negative is a distinct way the tag could have been built wrong —
/// wrong status set, wrong date comparison, or reading `ht_bookings` without
/// the room assignment (which would tag every room on the floor).
#[tokio::test]
async fn the_arrival_tag_is_todays_unconsumed_booking_for_this_room() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_arrival_tag_is_todays_unconsumed_booking_for_this_room — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;
    let yesterday = today - chrono::Duration::days(1);
    let tomorrow = today + chrono::Duration::days(1);

    // The one that must tag.
    let b = occ_booking(&pool, cust_id, "ZOB-CONF", today, "confirmed").await;
    occ_booking_room(&pool, b, rooms[R_ARR]).await;

    // Every non-tagging shape, all pointed at R_DEP so a single assertion
    // covers them: already checked in, cancelled, tomorrow, yesterday.
    for (book_no, checkin, status) in [
        ("ZOB-CIN", today, "checked_in"),
        ("ZOB-CANC", today, "cancelled"),
        ("ZOB-TOM", tomorrow, "confirmed"),
        ("ZOB-YEST", yesterday, "confirmed"),
    ] {
        let id = occ_booking(&pool, cust_id, book_no, checkin, status).await;
        occ_booking_room(&pool, id, rooms[R_DEP]).await;
    }

    // A today-confirmed booking with NO room assignment must tag NOTHING.
    occ_booking(&pool, cust_id, "ZOB-NOROOM", today, "confirmed").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        room_in(&list, R_ARR)["expectedArrival"],
        serde_json::json!(true),
        "a confirmed booking starting today tags an arrival: {list}"
    );
    assert_eq!(
        room_in(&list, R_DEP)["expectedArrival"],
        serde_json::json!(false),
        "checked_in / cancelled / tomorrow / yesterday must NOT tag: {list}"
    );
    assert_eq!(
        room_in(&list, R_STAY)["expectedArrival"],
        serde_json::json!(false),
        "an unassigned booking must not tag every room: {list}"
    );

    // A 'pending' hold is an arrival too — same tag, second admitted status.
    let p = occ_booking(&pool, cust_id, "ZOB-PEND", today, "pending").await;
    occ_booking_room(&pool, p, rooms[R_NOCR]).await;
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        room_in(&list, R_NOCR)["expectedArrival"],
        serde_json::json!(true),
        "'pending' is an admitted arrival status: {list}"
    );

    occ_cleanup(&pool).await;
}

/// The departure tag: an ACTIVE checkin on this room due out today or EARLIER.
///
/// `<=`, not `=`: a guest past their date is still due to leave, and that is
/// the maid's planning question. Production carries zero folios more than 7
/// days overdue at either site (checked 2026-08-19), so the widened rule adds
/// no noise.
#[tokio::test]
async fn the_departure_tag_covers_today_and_overstay_but_not_tomorrow() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_departure_tag_covers_today_and_overstay_but_not_tomorrow — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;
    let yesterday = today - chrono::Duration::days(1);
    let tomorrow = today + chrono::Duration::days(1);

    // Due out TODAY.
    let c_today = occ_checkin(&pool, cust_id, rooms[R_DEP], "ZOCC-DTODAY", today).await;
    occ_cr(&pool, c_today, rooms[R_DEP], "เข้าพัก").await;
    // OVERSTAY — due out yesterday, still in house.
    let c_over = occ_checkin(&pool, cust_id, rooms[R_STAY], "ZOCC-DOVER", yesterday).await;
    occ_cr(&pool, c_over, rooms[R_STAY], "เข้าพัก").await;
    // Staying on.
    let c_tom = occ_checkin(&pool, cust_id, rooms[R_ARR], "ZOCC-DTOM", tomorrow).await;
    occ_cr(&pool, c_tom, rooms[R_ARR], "เข้าพัก").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        room_in(&list, R_DEP)["expectedDeparture"],
        serde_json::json!(true),
        "due out today: {list}"
    );
    assert_eq!(
        room_in(&list, R_STAY)["expectedDeparture"],
        serde_json::json!(true),
        "an overstay is still due to leave: {list}"
    );
    assert_eq!(
        room_in(&list, R_ARR)["expectedDeparture"],
        serde_json::json!(false),
        "a guest staying another night is not departing: {list}"
    );

    // A REAL checkout clears both the tag and the occupancy — no separate
    // "departed" state to maintain, which is the whole reason this is derived.
    sqlx::query("UPDATE ht_checkins SET cin_checkout_time = NOW(), cin_status = 'completed' WHERE cin_id = $1")
        .bind(c_today)
        .execute(&pool)
        .await
        .expect("close the folio");
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        room_in(&list, R_DEP)["expectedDeparture"],
        serde_json::json!(false),
        "after a real checkout the tag is gone: {list}"
    );
    assert_eq!(
        room_in(&list, R_DEP)["occupancy"],
        serde_json::json!("vacant"),
        "…and so is the occupancy: {list}"
    );

    occ_cleanup(&pool).await;
}

/// A two-room folio checking out ONE room: the checked-out room drops both its
/// occupancy and its departure tag while the sibling keeps both. Occupancy and
/// departure resolve the room through the same predicate, so this is the one
/// fixture that would catch them drifting apart.
#[tokio::test]
async fn a_per_room_checkout_only_clears_that_room_of_the_folio() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_per_room_checkout_only_clears_that_room_of_the_folio — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;

    let cin = occ_checkin(&pool, cust_id, rooms[R_OUT], "ZOCC-PAIR", today).await;
    occ_cr(&pool, cin, rooms[R_OUT], "Check-Out").await;
    occ_cr(&pool, cin, rooms[R_STAY], "เข้าพัก").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    for (room_no, occupancy, departing) in [
        (R_OUT, "vacant", false),
        (R_STAY, "occupied", true),
    ] {
        assert_eq!(
            room_in(&list, room_no)["occupancy"],
            serde_json::json!(occupancy),
            "{room_no} occupancy: {list}"
        );
        assert_eq!(
            room_in(&list, room_no)["expectedDeparture"],
            serde_json::json!(departing),
            "{room_no} departure tag: {list}"
        );
    }
    // The header `cin_room_id` points at the CHECKED-OUT room, so the
    // assertion above is only meaningful if that is actually the case: the
    // `cin_room_id` fallback must NOT resurrect it, because the folio has
    // `cr` rows and those are authoritative when present.
    let header_room: i32 =
        sqlx::query_scalar("SELECT cin_room_id FROM ht_checkins WHERE cin_id = $1")
            .bind(cin)
            .fetch_one(&pool)
            .await
            .expect("read header room");
    assert_eq!(
        header_room, rooms[R_OUT],
        "fixture precondition: the folio header points at the checked-out room"
    );

    occ_cleanup(&pool).await;
}

/// Both tags on ONE row: a guest leaving today and a booking arriving into the
/// same room. This is the turnover the maid's list exists to prioritise, and
/// the tags are independent — neither may suppress the other.
#[tokio::test]
async fn one_room_can_be_both_departing_and_arriving_today() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping one_room_can_be_both_departing_and_arriving_today — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;

    let cin = occ_checkin(&pool, cust_id, rooms[R_DEP], "ZOCC-TURN", today).await;
    occ_cr(&pool, cin, rooms[R_DEP], "เข้าพัก").await;
    let b = occ_booking(&pool, cust_id, "ZOB-TURN", today, "confirmed").await;
    occ_booking_room(&pool, b, rooms[R_DEP]).await;

    // iHOTEL UNREACHABLE: the two tags are canonical-only, so they must be
    // live and truthful in exactly the response that flags everything else
    // stale.
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    let room = room_in(&list, R_DEP);
    assert_eq!(list["legacyStatusStale"], serde_json::json!(true), "{list}");
    assert_eq!(room["expectedDeparture"], serde_json::json!(true), "{list}");
    assert_eq!(room["expectedArrival"], serde_json::json!(true), "{list}");
    assert_eq!(room["occupancy"], serde_json::json!("occupied"), "{list}");

    // Same story on the detail endpoint.
    let room_id = rooms[R_DEP];
    let (_, detail) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(
        detail["room"]["expectedDeparture"],
        serde_json::json!(true),
        "{detail}"
    );
    assert_eq!(
        detail["room"]["expectedArrival"],
        serde_json::json!(true),
        "{detail}"
    );

    occ_cleanup(&pool).await;
}

/// An `Available` iHOTEL answer must not disturb the two canonical tags. The
/// merge touches cleanliness and occupancy ONLY; a merge that rebuilt the row
/// would silently blank the maid's planning tags.
#[tokio::test]
async fn an_ihotel_answer_leaves_the_arrival_and_departure_tags_alone() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping an_ihotel_answer_leaves_the_arrival_and_departure_tags_alone — PG not reachable");
        return;
    };
    let (cust_id, rooms) = occ_fixture(&pool).await;
    let today = bkk_today(&pool).await;

    let cin = occ_checkin(&pool, cust_id, rooms[R_DEP], "ZOCC-KEEP", today).await;
    occ_cr(&pool, cin, rooms[R_DEP], "เข้าพัก").await;
    let b = occ_booking(&pool, cust_id, "ZOB-KEEP", today, "confirmed").await;
    occ_booking_room(&pool, b, rooms[R_DEP]).await;

    // iHOTEL contradicts BOTH merged facts…
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), ihotel_occupancy(R_DEP, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    let room = room_in(&list, R_DEP);
    assert_eq!(room["occupancy"], serde_json::json!("vacant"), "{list}");
    // …and the canonical-only tags are untouched.
    assert_eq!(room["expectedDeparture"], serde_json::json!(true), "{list}");
    assert_eq!(room["expectedArrival"], serde_json::json!(true), "{list}");

    occ_cleanup(&pool).await;
}
