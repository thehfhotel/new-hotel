//! Direct-booking program B8f — integration tests for the morning
//! reconciliation (`GET /api/reports/loyalty-reconcile`, checklist L6).
//!
//! Seeds one real row per reconcile kind into a live PG pool — `ht_customers`
//! / `ht_rooms_new` / `ht_bookings` / `ht_booking_rooms` / `ht_checkins` /
//! `writeback_jobs` — runs `service::reports::loyalty_reconcile` against them
//! and asserts each lands in the kind its borrowed predicate says it should.
//!
//! Follows `test_channel_rollup_report.rs`, the repo's other `/api/reports/*`
//! service-level test.
//!
//! ## Isolation, and the one place it is necessarily weaker
//!
//! Two of the five kinds are **not date-scoped**, by design: the writeback-gap
//! query (Track F5's predicate) and the sweep-lag query
//! (`expired_hold_ids`'s predicate) both scan every loyalty booking in the
//! database regardless of stay dates, because a stalled writeback or an
//! unswept hold is urgent whenever it happened. A far-future window therefore
//! cannot fence them off the way it fences the D3 rollup.
//!
//! So the assertions split:
//!
//! * **date-scoped kinds** (`deposit_divergence`, `unlinked_checkin`) — exact
//!   counts, fenced by a far-future report date (`2099-04-*`) no other suite
//!   or production-shaped seed touches;
//! * **undated kinds** (`writeback_stalled`, `legacy_hold_orphan`,
//!   `sweep_lag`) — assert THIS test's `book_no` is present with the right
//!   kind, age and action, and that counts are at least ours. Asserting an
//!   exact global count there would make the suite fail because some other
//!   test left a stuck job behind, which is a true statement about the
//!   database and a useless test failure.
//!
//! Cleanup uses EXACT-match markers on `book_notes` / `cust_notes` /
//! `room_notes` / `cin_notes` per `tests/common/mod.rs` discipline — never a
//! `LIKE`.

mod common;

use chrono::NaiveDate;
use hotel_backend::service::reports::loyalty_reconcile::{
    load_loyalty_reconcile, LoyaltyReconcile, ReconcileRow, DEPOSIT_HORIZON_DAYS,
};
use sqlx::PgPool;
use uuid::Uuid;

/// `cleanup_b8f` sweeps every row carrying the shared marker and each test
/// calls it on entry, so parallel tests in this file would race each other's
/// fixtures. Serialise it.
static B8F_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

const ROOM_MARKER: &str = "TEST_b8f_recon_room";
const CUST_MARKER: &str = "TEST_b8f_recon_cust";
const BOOK_MARKER: &str = "TEST_b8f_recon_book";
const CIN_MARKER: &str = "TEST_b8f_recon_cin";

/// Far-future report date nothing else in the suite touches. Deliberately in
/// April, a month clear of `test_channel_rollup_report.rs`'s `2099-03-*`.
const REPORT_DATE: (i32, u32, u32) = (2099, 4, 10);

/// The stall threshold the tests bind explicitly, rather than reading
/// `stall_threshold_minutes()` — a `LOYALTY_WRITEBACK_STALL_ALERT_MINUTES`
/// set in the test environment must not silently change what these assert.
const THRESHOLD_MINUTES: i32 = 10;

fn report_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(REPORT_DATE.0, REPORT_DATE.1, REPORT_DATE.2).unwrap()
}

fn day(offset: i64) -> NaiveDate {
    report_date() + chrono::Duration::days(offset)
}

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

/// Drop everything these tests authored. Children before parents, and
/// `writeback_jobs` first — it is keyed on the booking's `aggregate_id`, so it
/// must go before the bookings that resolve those UUIDs.
async fn cleanup_b8f(pool: &PgPool) {
    sqlx::query(
        "DELETE FROM writeback_jobs WHERE aggregate_id IN \
         (SELECT aggregate_id FROM ht_bookings WHERE book_notes = $1 AND aggregate_id IS NOT NULL)",
    )
    .bind(BOOK_MARKER)
    .execute(pool)
    .await
    .ok();
    sqlx::query("DELETE FROM ht_checkins WHERE cin_notes = $1")
        .bind(CIN_MARKER)
        .execute(pool)
        .await
        .ok();
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
         VALUES ('B8f', 'Reconcile', $1) RETURNING cust_id",
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

/// One loyalty-channel booking. Returns `(book_id, aggregate_id)` — the UUID
/// is the ONLY key `writeback_jobs` shares with `ht_bookings`, so the tests
/// that need a job row need it back.
///
/// `hold_offset_minutes` positions `book_hold_expires_at` relative to `now()`:
/// negative puts the hold in the past (the sweep-lag shape), positive keeps it
/// live. Every channel booking carries the column — `confirm_booking_payment`
/// deliberately does NOT clear it — so the fixtures carry it too.
#[allow(clippy::too_many_arguments)]
async fn seed_loyalty_booking(
    pool: &PgPool,
    book_no: &str,
    cust_id: i32,
    check_in: NaiveDate,
    nights: i64,
    status: &str,
    deposit: f64,
    hold_offset_minutes: i64,
) -> (i32, Uuid) {
    sqlx::query_as::<_, (i32, Uuid)>(
        "INSERT INTO ht_bookings \
            (book_no, book_cust_id, book_checkin, book_checkout, book_channel, book_source, \
             book_status, book_deposit_amount, book_hold_expires_at, book_notes, aggregate_id) \
         VALUES ($1, $2, $3, $4, 'loyalty', 'loyalty', $5, $6::float8, \
                 now() + make_interval(mins => $7), $8, gen_random_uuid()) \
         RETURNING book_id, aggregate_id",
    )
    .bind(book_no)
    .bind(cust_id)
    .bind(check_in)
    .bind(check_in + chrono::Duration::days(nights))
    .bind(status)
    .bind(deposit)
    .bind(hold_offset_minutes as i32)
    .bind(BOOK_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_loyalty_booking failed")
}

async fn assign_room(pool: &PgPool, book_id: i32, room_id: i32) {
    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(book_id)
        .bind(room_id)
        .execute(pool)
        .await
        .expect("assign_room failed");
}

/// A writeback job in a NOT-`done` state, aged `age_minutes` into the past.
///
/// `idempotency_key` is `UNIQUE NOT NULL`, so each fixture mints its own.
async fn seed_writeback_job(
    pool: &PgPool,
    aggregate_id: Uuid,
    intent: &str,
    status: &str,
    age_minutes: i64,
) {
    sqlx::query(
        "INSERT INTO writeback_jobs (intent, payload, aggregate_id, idempotency_key, status, created_at) \
         VALUES ($1, '{}'::jsonb, $2, gen_random_uuid(), $3, now() - make_interval(mins => $4))",
    )
    .bind(intent)
    .bind(aggregate_id)
    .bind(status)
    .bind(age_minutes as i32)
    .execute(pool)
    .await
    .expect("seed_writeback_job failed");
}

/// A check-in on `room_id` with **no booking link** — the B7a gap shape: the
/// guest was checked in through iHOTEL (or as a walk-in on our own room
/// board), so `cin_book_id` is NULL and every app-deposit signpost that
/// resolves through it renders nothing.
async fn seed_unlinked_checkin(
    pool: &PgPool,
    cin_no: &str,
    cust_id: i32,
    room_id: i32,
    checkin_date: NaiveDate,
    expected_checkout: NaiveDate,
) {
    sqlx::query(
        "INSERT INTO ht_checkins \
            (cin_no, cin_book_id, cin_cust_id, cin_room_id, cin_checkin_time, \
             cin_expected_checkout, cin_status, cin_notes) \
         VALUES ($1, NULL, $2, $3, $4::date + TIME '14:00', $5, 'active', $6)",
    )
    .bind(cin_no)
    .bind(cust_id)
    .bind(room_id)
    .bind(checkin_date)
    .bind(expected_checkout)
    .bind(CIN_MARKER)
    .execute(pool)
    .await
    .expect("seed_unlinked_checkin failed");
}

async fn run_report(pool: &PgPool) -> LoyaltyReconcile {
    load_loyalty_reconcile(pool, report_date(), DEPOSIT_HORIZON_DAYS, THRESHOLD_MINUTES)
        .await
        .expect("load_loyalty_reconcile failed")
}

fn find<'a>(out: &'a LoyaltyReconcile, book_no: &str) -> &'a ReconcileRow {
    out.rows
        .iter()
        .find(|r| r.book_no == book_no)
        .unwrap_or_else(|| {
            panic!(
                "no row for `{book_no}`; got {:?}",
                out.rows
                    .iter()
                    .map(|r| (r.book_no.as_str(), r.kind))
                    .collect::<Vec<_>>()
            )
        })
}

fn rows_of<'a>(out: &'a LoyaltyReconcile, kind: &str) -> Vec<&'a ReconcileRow> {
    out.rows.iter().filter(|r| r.kind == kind).collect()
}

// ---------------------------------------------------------------------------
// The pivot test — one real row per kind
// ---------------------------------------------------------------------------

/// Seed one row per reconcile kind end to end and assert each is bucketed the
/// way its borrowed predicate says it should be.
#[tokio::test]
async fn morning_reconcile_buckets_one_real_row_of_every_kind() {
    let _guard = B8F_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_b8f(&pool).await;

    let cust = seed_customer(&pool).await;
    let room_stall = seed_room(&pool, "B8F01").await;
    let room_unlinked = seed_room(&pool, "B8F02").await;
    let room_deposit = seed_room(&pool, "B8F03").await;

    // (a) writeback_stalled — a live hold whose create never reached iHOTEL.
    //     Hold deliberately still LIVE (+90m) so it cannot also trip sweep lag.
    let (stall_id, stall_agg) =
        seed_loyalty_booking(&pool, "TESTB8F-STALL", cust, day(0), 2, "pending", 0.0, 90).await;
    assign_room(&pool, stall_id, room_stall).await;
    seed_writeback_job(&pool, stall_agg, "create_booking", "pending", 30).await;

    // (b-ii) legacy_hold_orphan — the hold is cancelled in PG but the cancel
    //        writeback never landed, so iHOTEL still shows `จอง`.
    let (_orphan_id, orphan_agg) = seed_loyalty_booking(
        &pool,
        "TESTB8F-ORPHAN",
        cust,
        day(3),
        1,
        "cancelled",
        0.0,
        -240,
    )
    .await;
    seed_writeback_job(&pool, orphan_agg, "cancel_booking", "failed", 75).await;

    // (b-i) sweep_lag — still `pending` 20 minutes past its deadline, which
    //       the 5-minute sweep should already have released.
    seed_loyalty_booking(&pool, "TESTB8F-SWEEP", cust, day(5), 1, "pending", 0.0, -20).await;

    // (d) unlinked_checkin — stay covers the report date, guest is in the
    //     room, `cin_book_id` is NULL. Arrival the day BEFORE the report date
    //     so it cannot also land in the deposit look-ahead.
    let (unlinked_id, _) = seed_loyalty_booking(
        &pool,
        "TESTB8F-UNLINK",
        cust,
        day(-1),
        3,
        "confirmed",
        0.0,
        -600,
    )
    .await;
    assign_room(&pool, unlinked_id, room_unlinked).await;
    seed_unlinked_checkin(&pool, "TESTB8F-C1", cust, room_unlinked, day(-1), day(2)).await;

    // (c) deposit_divergence — arriving on the report date with money in the
    //     bank that iHOTEL will show as 0.
    let (deposit_id, _) = seed_loyalty_booking(
        &pool,
        "TESTB8F-DEP",
        cust,
        day(0),
        2,
        "confirmed",
        1500.50,
        -120,
    )
    .await;
    assign_room(&pool, deposit_id, room_deposit).await;

    let out = run_report(&pool).await;

    // ---- (a) the missing iHOTEL twin -----------------------------------
    let stall = find(&out, "TESTB8F-STALL");
    assert_eq!(stall.kind, "writeback_stalled");
    assert_eq!(
        stall.action, "call_guest",
        "a room sold in PG and free in iHOTEL needs a human, not a retry"
    );
    assert!(stall.is_defect);
    assert_eq!(stall.intent.as_deref(), Some("create_booking"));
    assert_eq!(stall.job_status.as_deref(), Some("pending"));
    assert!(
        stall.age_minutes.unwrap_or(0) >= 29,
        "age is measured from writeback_jobs.created_at, got {:?}",
        stall.age_minutes
    );
    assert_eq!(stall.age, "30m");
    assert_eq!(stall.room_no.as_deref(), Some("B8F01"));
    assert_eq!(stall.guest_name.as_deref(), Some("B8f Reconcile"));
    assert_eq!(stall.arrival, "today");

    // ---- (b-ii) the phantom จอง ----------------------------------------
    let orphan = find(&out, "TESTB8F-ORPHAN");
    assert_eq!(
        orphan.kind, "legacy_hold_orphan",
        "a stuck cancel on an already-cancelled booking is the phantom kind, \
         not the missing-twin kind"
    );
    assert_eq!(orphan.action, "resend_hold");
    assert_eq!(orphan.book_status, "cancelled");
    assert_eq!(orphan.intent.as_deref(), Some("cancel_booking"));
    assert_eq!(orphan.job_status.as_deref(), Some("failed"));
    assert_eq!(orphan.age, "1h 15m");
    assert_eq!(
        orphan.room_no, None,
        "no room assigned — the lateral must not drop the row"
    );

    // ---- (b-i) sweep lag -----------------------------------------------
    let sweep = find(&out, "TESTB8F-SWEEP");
    assert_eq!(sweep.kind, "sweep_lag");
    assert_eq!(sweep.action, "resend_hold");
    assert_eq!(sweep.book_status, "pending");
    assert!(
        sweep.hold_expires_at.is_some(),
        "sweep-lag rows must carry the deadline that made them late"
    );
    assert!(
        sweep.age_minutes.unwrap_or(0) >= 19,
        "age is minutes past book_hold_expires_at, got {:?}",
        sweep.age_minutes
    );
    assert_eq!(sweep.arrival, "+5d");

    // ---- (d) the B7a gap ------------------------------------------------
    let unlinked = find(&out, "TESTB8F-UNLINK");
    assert_eq!(unlinked.kind, "unlinked_checkin");
    assert_eq!(unlinked.action, "brief_desk");
    assert!(unlinked.is_defect, "a missing link costs money at checkout");
    assert_eq!(unlinked.cin_no.as_deref(), Some("TESTB8F-C1"));
    assert!(unlinked.checked_in_at.is_some());
    assert_eq!(unlinked.room_no.as_deref(), Some("B8F02"));
    assert_eq!(
        rows_of(&out, "unlinked_checkin").len(),
        1,
        "date-scoped: only this test's row can be in the window"
    );

    // ---- (c) the accepted deposit divergence ----------------------------
    let deposit = find(&out, "TESTB8F-DEP");
    assert_eq!(deposit.kind, "deposit_divergence");
    assert_eq!(deposit.action, "brief_desk");
    assert!(
        !deposit.is_defect,
        "iHOTEL showing deposit 0 is documented and permanent — it must not \
         make a healthy morning read red"
    );
    assert_eq!(deposit.deposit_amount, Some(1500.50));
    assert_eq!(
        deposit.age_minutes, None,
        "no clock runs on a briefing item"
    );
    assert_eq!(deposit.age, "");
    assert_eq!(deposit.arrival, "today");
    assert_eq!(
        rows_of(&out, "deposit_divergence").len(),
        1,
        "date-scoped: only this test's row can be in the window"
    );

    // ---- (e) the counts summary -----------------------------------------
    assert_eq!(out.summary.deposit_divergence, 1);
    assert_eq!(out.summary.unlinked_checkin, 1);
    assert_eq!(out.summary.deposit_total, 1500.50);
    // The three undated kinds are at-least assertions: another suite's stuck
    // job is a true statement about the database, not a failure of this test.
    assert!(out.summary.writeback_stalled >= 1);
    assert!(out.summary.legacy_hold_orphan >= 1);
    assert!(out.summary.sweep_lag >= 1);
    assert!(out.summary.defects >= 4);
    assert!(
        !out.summary.clear,
        "four defects were seeded; the morning is not clear"
    );
    assert!(out.summary.total >= 5);

    // ---- display order ---------------------------------------------------
    // Blast-radius order: the two that can cost a room outrank the two that
    // cost only paperwork, and the briefing item is last.
    let seeded: Vec<&str> = out
        .rows
        .iter()
        .filter(|r| r.book_no.starts_with("TESTB8F-"))
        .map(|r| r.kind)
        .collect();
    assert_eq!(
        seeded,
        vec![
            "writeback_stalled",
            "legacy_hold_orphan",
            "sweep_lag",
            "unlinked_checkin",
            "deposit_divergence",
        ]
    );

    cleanup_b8f(&pool).await;
}

/// The stall threshold is a real gate, not decoration: a healthy in-flight
/// job must not appear on the desk's list. This is what keeps the report and
/// the Track F5 alert reporting the same set — a row here is a row that would
/// page, and vice versa.
#[tokio::test]
async fn a_job_younger_than_the_threshold_is_not_on_the_morning_list() {
    let _guard = B8F_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_b8f(&pool).await;

    let cust = seed_customer(&pool).await;
    let (_id, agg) =
        seed_loyalty_booking(&pool, "TESTB8F-FRESH", cust, day(0), 1, "pending", 0.0, 120).await;
    // One minute old — NOTIFY is sub-second and the poll fallback is 30s, so
    // this job is simply in flight.
    seed_writeback_job(&pool, agg, "create_booking", "pending", 1).await;

    let out = run_report(&pool).await;
    assert!(
        !out.rows.iter().any(|r| r.book_no == "TESTB8F-FRESH"),
        "a 1-minute-old job is healthy in-flight work; listing it would train \
         the desk to ignore the section"
    );

    cleanup_b8f(&pool).await;
}

/// An APPLIED writeback (`status = 'done'`) is the "iHOTEL has a twin" case
/// and must be silent however old it is — this pins the shared
/// `WRITEBACK_APPLIED_STATUS` literal end to end, not just as a constant.
#[tokio::test]
async fn an_applied_writeback_is_never_reported() {
    let _guard = B8F_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_b8f(&pool).await;

    let cust = seed_customer(&pool).await;
    let (_id, agg) = seed_loyalty_booking(
        &pool,
        "TESTB8F-DONE",
        cust,
        day(0),
        1,
        "confirmed",
        0.0,
        120,
    )
    .await;
    seed_writeback_job(&pool, agg, "create_booking", "done", 5000).await;

    let out = run_report(&pool).await;
    assert!(
        !out.rows.iter().any(|r| r.book_no == "TESTB8F-DONE"),
        "`done` means the MSSQL transaction committed — the twin exists"
    );

    cleanup_b8f(&pool).await;
}

/// A check-in that DOES carry its booking link is the healthy B7a shape and
/// must not be reported — otherwise every correctly-linked app arrival would
/// land on the morning list and the routine would be abandoned in a week.
#[tokio::test]
async fn a_linked_checkin_is_not_the_b7a_gap() {
    let _guard = B8F_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_b8f(&pool).await;

    let cust = seed_customer(&pool).await;
    let room = seed_room(&pool, "B8F09").await;
    let (book_id, _) = seed_loyalty_booking(
        &pool,
        "TESTB8F-LINKED",
        cust,
        day(-1),
        3,
        "checkedin",
        0.0,
        -600,
    )
    .await;
    assign_room(&pool, book_id, room).await;

    sqlx::query(
        "INSERT INTO ht_checkins \
            (cin_no, cin_book_id, cin_cust_id, cin_room_id, cin_checkin_time, \
             cin_expected_checkout, cin_status, cin_notes) \
         VALUES ('TESTB8F-C9', $1, $2, $3, $4::date + TIME '14:00', $5, 'active', $6)",
    )
    .bind(book_id)
    .bind(cust)
    .bind(room)
    .bind(day(-1))
    .bind(day(2))
    .bind(CIN_MARKER)
    .execute(&pool)
    .await
    .expect("seed linked check-in failed");

    let out = run_report(&pool).await;
    assert!(
        !out.rows.iter().any(|r| r.book_no == "TESTB8F-LINKED"),
        "cin_book_id is set — the deposit signposts resolve and nothing is wrong"
    );

    cleanup_b8f(&pool).await;
}

/// A cancelled app booking must not show up in the deposit look-ahead: the
/// guest is not arriving, so there is nothing to brief the desk about. Pins
/// the `IS DISTINCT FROM` choice — a NULL-status booking must still be listed,
/// which a plain `<>` would silently drop from both sides.
#[tokio::test]
async fn a_cancelled_booking_is_not_in_the_deposit_lookahead() {
    let _guard = B8F_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_b8f(&pool).await;

    let cust = seed_customer(&pool).await;
    seed_loyalty_booking(
        &pool,
        "TESTB8F-DEPCXL",
        cust,
        day(0),
        1,
        "cancelled",
        900.0,
        -300,
    )
    .await;

    // Same shape, but with a NULL status — must still be reported.
    sqlx::query(
        "INSERT INTO ht_bookings \
            (book_no, book_cust_id, book_checkin, book_checkout, book_channel, book_source, \
             book_status, book_deposit_amount, book_notes, aggregate_id) \
         VALUES ('TESTB8F-DEPNULL', $1, $2, $3, 'loyalty', 'loyalty', NULL, 700.0, $4, \
                 gen_random_uuid())",
    )
    .bind(cust)
    .bind(day(1))
    .bind(day(2))
    .bind(BOOK_MARKER)
    .execute(&pool)
    .await
    .expect("seed NULL-status booking failed");

    let out = run_report(&pool).await;

    assert!(
        !out.rows.iter().any(|r| r.book_no == "TESTB8F-DEPCXL"),
        "a cancelled guest is not arriving"
    );
    let null_status = find(&out, "TESTB8F-DEPNULL");
    assert_eq!(null_status.kind, "deposit_divergence");
    assert_eq!(null_status.arrival, "tomorrow");
    assert_eq!(
        null_status.book_status, "",
        "book_status is nullable, and a row the predicate deliberately INCLUDES          must also decode — every loader COALESCEs it to ''"
    );
    assert_eq!(out.summary.deposit_total, 700.0);

    cleanup_b8f(&pool).await;
}

/// A clean morning — no loyalty rows in the window at all — must come back
/// `clear`, with an empty list and zeroed counts. This is the shape reception
/// should see on most days, and the one the five-minute routine is sized for.
#[tokio::test]
async fn a_quiet_morning_reports_clear_for_the_date_scoped_kinds() {
    let _guard = B8F_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_b8f(&pool).await;

    let out = run_report(&pool).await;

    assert_eq!(
        rows_of(&out, "deposit_divergence").len(),
        0,
        "nothing seeded in the far-future window"
    );
    assert_eq!(rows_of(&out, "unlinked_checkin").len(), 0);
    assert_eq!(out.summary.deposit_divergence, 0);
    assert_eq!(out.summary.unlinked_checkin, 0);
    assert_eq!(out.summary.deposit_total, 0.0);

    cleanup_b8f(&pool).await;
}
