//! Direct-booking program D3 — integration tests for the channel rollup.
//!
//! Seeds real `ht_customers` / `ht_rooms_new` / `ht_bookings` /
//! `ht_booking_rooms` rows into a live PG pool, runs
//! `service::reports::channel_rollup` against them and asserts the bucketing,
//! the room-night multiplier and the cancelled-vs-sellable split.
//!
//! Isolation has two layers, because this report aggregates EVERY booking in
//! its window and would otherwise pick up any row another test left behind:
//!
//! 1. The window is far-future (`2099-03-*`), so no other suite's fixtures or
//!    any production-shaped seed can land inside it.
//! 2. Cleanup uses an EXACT-match marker on `book_notes` / `cust_notes` /
//!    `room_notes` per `tests/common/mod.rs` discipline — never a `LIKE`.
//!
//! Follows `test_rr4_export.rs`, the repo's other `/api/reports/*` test.

mod common;

use chrono::NaiveDate;
use hotel_backend::service::reports::channel_rollup::{load_channel_rollup, rollup, ChannelRollup};
use sqlx::PgPool;

/// `cleanup_d3` sweeps every row carrying the shared marker and each test
/// calls it on entry, so parallel tests would race each other's fixtures
/// (observed in the G8 suite as an FK violation). Serialise the file.
static D3_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

const ROOM_MARKER: &str = "TEST_d3_rollup_room";
const CUST_MARKER: &str = "TEST_d3_rollup_cust";
const BOOK_MARKER: &str = "TEST_d3_rollup_book";

/// Far-future window nothing else in the suite touches.
const WINDOW_FROM: (i32, u32, u32) = (2099, 3, 1);
const WINDOW_TO: (i32, u32, u32) = (2099, 3, 31);

fn window() -> (NaiveDate, NaiveDate) {
    (
        NaiveDate::from_ymd_opt(WINDOW_FROM.0, WINDOW_FROM.1, WINDOW_FROM.2).unwrap(),
        NaiveDate::from_ymd_opt(WINDOW_TO.0, WINDOW_TO.1, WINDOW_TO.2).unwrap(),
    )
}

/// Drop everything these tests authored. Children before parents.
async fn cleanup_d3(pool: &PgPool) {
    sqlx::query(
        "DELETE FROM ht_booking_rooms WHERE br_book_id IN \
         (SELECT book_id FROM ht_bookings WHERE book_notes = $1)",
    )
    .bind(BOOK_MARKER)
    .execute(pool)
    .await
    .ok();
    sqlx::query("DELETE FROM ht_bookings WHERE book_notes = $1")
        .bind(BOOK_MARKER)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_notes = $1")
        .bind(ROOM_MARKER)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_notes = $1")
        .bind(CUST_MARKER)
        .execute(pool)
        .await
        .ok();
}

async fn seed_customer(pool: &PgPool) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_notes) \
         VALUES ('D3', 'Rollup', $1) RETURNING cust_id",
    )
    .bind(CUST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_customer failed")
}

async fn seed_room(pool: &PgPool, room_no: &str) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_rooms_new (room_no, room_notes) VALUES ($1, $2) RETURNING room_id",
    )
    .bind(room_no)
    .bind(ROOM_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_room failed")
}

/// Seed one booking. `book_no` must be unique and fit `VARCHAR(20)`.
/// `nights` drives `book_checkout`, which drives the generated `book_nights`.
#[allow(clippy::too_many_arguments)]
async fn seed_booking(
    pool: &PgPool,
    book_no: &str,
    cust_id: i32,
    checkin: NaiveDate,
    nights: i64,
    channel: Option<&str>,
    source: Option<&str>,
    status: &str,
    total: f64,
) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_bookings \
            (book_no, book_cust_id, book_checkin, book_checkout, book_channel, \
             book_source, book_status, book_total_amount, book_notes) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8::float8, $9) RETURNING book_id",
    )
    .bind(book_no)
    .bind(cust_id)
    .bind(checkin)
    .bind(checkin + chrono::Duration::days(nights))
    .bind(channel)
    .bind(source)
    .bind(status)
    .bind(total)
    .bind(BOOK_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_booking failed")
}

async fn assign_room(pool: &PgPool, book_id: i32, room_id: i32) {
    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(book_id)
        .bind(room_id)
        .execute(pool)
        .await
        .expect("assign_room failed");
}

async fn run_rollup(pool: &PgPool) -> ChannelRollup {
    let (from, to) = window();
    let rows = load_channel_rollup(pool, from, to)
        .await
        .expect("load_channel_rollup failed");
    rollup(&rows)
}

fn bucket<'a>(
    out: &'a ChannelRollup,
    key: &str,
) -> &'a hotel_backend::service::reports::channel_rollup::ChannelBucketRow {
    out.buckets
        .iter()
        .find(|b| b.key == key)
        .unwrap_or_else(|| panic!("no `{key}` bucket in {:?}", out.buckets))
}

/// The pivot test: one booking per bucket, seeded end to end, must come back
/// bucketed the way `classify` says — including the two legacy classes
/// (`source='ota'` with no channel, and a `legacy_app` sync row).
#[tokio::test]
async fn channel_rollup_buckets_real_rows_by_channel_and_source() {
    let _guard = D3_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_d3(&pool).await;

    let cust = seed_customer(&pool).await;
    let checkin = NaiveDate::from_ymd_opt(2099, 3, 10).unwrap();

    // app: 2 nights, 1 room, THB 4,000
    seed_booking(
        &pool,
        "TESTD3-APP",
        cust,
        checkin,
        2,
        Some("loyalty"),
        Some("loyalty"),
        "confirmed",
        4000.0,
    )
    .await;
    // OTA by name: 3 nights, 1 room, THB 9,000
    seed_booking(
        &pool,
        "TESTD3-AGO",
        cust,
        checkin,
        3,
        Some("agoda"),
        None,
        "confirmed",
        9000.0,
    )
    .await;
    // unmapped OTA slug — still an OTA bucket, keyed on the raw slug
    seed_booking(
        &pool,
        "TESTD3-HW",
        cust,
        checkin,
        1,
        Some("hostelworld"),
        None,
        "confirmed",
        1500.0,
    )
    .await;
    // direct: walk-in, 1 night, THB 1,200
    seed_booking(
        &pool,
        "TESTD3-WI",
        cust,
        checkin,
        1,
        None,
        Some("walk-in"),
        "confirmed",
        1200.0,
    )
    .await;
    // pre-076 OTA row: source says ota, channel never written
    seed_booking(
        &pool,
        "TESTD3-PRE",
        cust,
        checkin,
        2,
        None,
        Some("ota"),
        "confirmed",
        3000.0,
    )
    .await;
    // legacy sync row: provenance not recorded → unknown, NOT direct
    seed_booking(
        &pool,
        "TESTD3-LEG",
        cust,
        checkin,
        4,
        None,
        Some("legacy_app"),
        "confirmed",
        6000.0,
    )
    .await;

    let out = run_rollup(&pool).await;

    let keys: Vec<&str> = out.buckets.iter().map(|b| b.key.as_str()).collect();
    assert_eq!(
        keys,
        vec!["app", "agoda", "hostelworld", "ota", "direct", "unknown"],
        "bucket set + display order"
    );

    assert_eq!(bucket(&out, "app").kind, "app");
    assert_eq!(bucket(&out, "app").totals.room_nights, 2);
    assert_eq!(bucket(&out, "app").totals.gross_revenue, 4000.0);

    assert_eq!(bucket(&out, "agoda").label, "Agoda");
    assert_eq!(bucket(&out, "agoda").totals.room_nights, 3);

    assert_eq!(bucket(&out, "hostelworld").kind, "ota");
    assert_eq!(bucket(&out, "hostelworld").label, "hostelworld");

    assert_eq!(bucket(&out, "ota").kind, "ota");
    assert_eq!(bucket(&out, "ota").label, "OTA");
    assert_eq!(bucket(&out, "ota").totals.room_nights, 2);

    assert_eq!(bucket(&out, "direct").totals.bookings, 1);
    assert_eq!(bucket(&out, "unknown").totals.room_nights, 4);

    // 13 room-nights total; app (2) + direct (1) = 3 → 23.1%
    assert_eq!(out.totals.bookings, 6);
    assert_eq!(out.totals.room_nights, 13);
    assert_eq!(out.direct_share, 23.1);

    cleanup_d3(&pool).await;
}

/// Room-nights multiply by the number of `ht_booking_rooms` rows, and a
/// booking with no assigned rooms still counts as one room rather than zero.
#[tokio::test]
async fn channel_rollup_multiplies_nights_by_rooms_and_floors_at_one() {
    let _guard = D3_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_d3(&pool).await;

    let cust = seed_customer(&pool).await;
    let room_a = seed_room(&pool, "TESTD3A").await;
    let room_b = seed_room(&pool, "TESTD3B").await;
    let room_c = seed_room(&pool, "TESTD3C").await;
    let checkin = NaiveDate::from_ymd_opt(2099, 3, 5).unwrap();

    // A 3-room, 2-night Agoda stay (iHOTEL-created shape: our app rejects
    // multi-room walk-ins) → 6 room-nights, not 2.
    let multi = seed_booking(
        &pool,
        "TESTD3-MULTI",
        cust,
        checkin,
        2,
        Some("agoda"),
        None,
        "confirmed",
        12000.0,
    )
    .await;
    assign_room(&pool, multi, room_a).await;
    assign_room(&pool, multi, room_b).await;
    assign_room(&pool, multi, room_c).await;

    // A 5-night direct booking with NO room assigned yet → floors at 1 room,
    // so 5 room-nights rather than 0.
    seed_booking(
        &pool,
        "TESTD3-NOROOM",
        cust,
        checkin,
        5,
        None,
        Some("phone"),
        "confirmed",
        5000.0,
    )
    .await;

    let out = run_rollup(&pool).await;

    assert_eq!(bucket(&out, "agoda").totals.room_nights, 6);
    assert_eq!(bucket(&out, "direct").totals.room_nights, 5);
    assert_eq!(out.totals.room_nights, 11);

    cleanup_d3(&pool).await;
}

/// A cancelled booking is counted and reported as cancelled, but sells no
/// room-nights and earns no revenue — the split KPI K7 reads.
#[tokio::test]
async fn channel_rollup_counts_cancellations_without_selling_nights() {
    let _guard = D3_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_d3(&pool).await;

    let cust = seed_customer(&pool).await;
    let checkin = NaiveDate::from_ymd_opt(2099, 3, 20).unwrap();

    seed_booking(
        &pool,
        "TESTD3-CFM",
        cust,
        checkin,
        2,
        Some("agoda"),
        None,
        "confirmed",
        6000.0,
    )
    .await;
    seed_booking(
        &pool,
        "TESTD3-CXL",
        cust,
        checkin,
        3,
        Some("agoda"),
        None,
        "cancelled",
        9000.0,
    )
    .await;

    let out = run_rollup(&pool).await;
    let agoda = bucket(&out, "agoda");

    assert_eq!(agoda.totals.bookings, 2, "cancellations stay in the count");
    assert_eq!(agoda.totals.cancelled, 1);
    assert_eq!(agoda.totals.room_nights, 2, "cancelled nights excluded");
    assert_eq!(agoda.totals.gross_revenue, 6000.0);

    cleanup_d3(&pool).await;
}

/// The window filters on the booking's check-in (arrival) date, inclusive at
/// both ends — a booking arriving the day after `to` is out.
#[tokio::test]
async fn channel_rollup_window_is_inclusive_on_check_in_date() {
    let _guard = D3_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_d3(&pool).await;

    let cust = seed_customer(&pool).await;
    let (from, to) = window();

    seed_booking(
        &pool,
        "TESTD3-FIRST",
        cust,
        from,
        1,
        None,
        Some("walk-in"),
        "confirmed",
        1000.0,
    )
    .await;
    seed_booking(
        &pool,
        "TESTD3-LAST",
        cust,
        to,
        1,
        None,
        Some("walk-in"),
        "confirmed",
        1000.0,
    )
    .await;
    // Arrives one day past the window — must not appear.
    seed_booking(
        &pool,
        "TESTD3-AFTER",
        cust,
        to + chrono::Duration::days(1),
        1,
        None,
        Some("walk-in"),
        "confirmed",
        1000.0,
    )
    .await;

    let out = run_rollup(&pool).await;

    assert_eq!(out.totals.bookings, 2, "both ends in, the day after out");
    assert_eq!(out.direct_share, 100.0);

    cleanup_d3(&pool).await;
}

/// An empty window renders cleanly: no buckets, zero totals, 0.0 share (not
/// NaN from a divide-by-zero).
#[tokio::test]
async fn channel_rollup_empty_window_is_all_zeroes() {
    let _guard = D3_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_d3(&pool).await;

    let out = run_rollup(&pool).await;

    assert!(out.buckets.is_empty());
    assert_eq!(out.totals.bookings, 0);
    assert_eq!(out.totals.room_nights, 0);
    assert_eq!(out.direct_share, 0.0);
    assert_eq!(out.direct_share_by_revenue, 0.0);
}
