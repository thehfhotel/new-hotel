//! Track B5 — `ht_checkin_rooms` backfill integration tests
//! (`docs/coexistence/audit-2026-05-13.md` Theme 1, T1 CRIT-1
//! follow-on).
//!
//! ## Why this exists
//!
//! Track B2 (commit 79f8276) made the sync mapper emit one
//! `ht_checkin_rooms` row per `HT_CheckIn_Ds` row whenever the CT
//! watcher re-syncs a folio. Folios that haven't been edited since
//! the B2 deploy still carry only the deprecated header-level
//! `ht_checkins.cin_room_id` and NO junction rows. Track B5 closes
//! that window with a one-shot backfill bin.
//!
//! These tests drive
//! [`hotel_backend::sync::backfill::backfill_one_folio_with_aggregate`]
//! directly. The bin's MSSQL-load step is bypassed by passing the
//! hand-built `CheckInAggregate` instead — same shape the CT watcher
//! produces but without needing a live legacy connection.
//!
//! ## What each test asserts
//!
//! - `backfill_idempotent_on_already_backfilled_folio` — call twice,
//!   assert the second call reports `SkippedIdempotent` AND the
//!   junction row count doesn't change.
//! - `backfill_materializes_two_room_folio` — seed a 2-room legacy
//!   aggregate against a pre-B2 PG row (`cin_room_id` only, no
//!   junction). Run backfill, assert both junction rows land.
//! - `backfill_skips_folio_missing_in_pg` — seed a legacy aggregate
//!   with no matching `ht_checkins.legacy_cin_no` row, assert no
//!   junction insert AND outcome is `SkippedMissingPg`.
//! - `backfill_dry_run_emits_no_writes` — dry-run on a 2-room folio
//!   reports `Applied` but the post-run junction count stays at 0.
//!
//! ## Cleanup
//!
//! Every test uses a `TEST_B5` marker on `cust_notes` / `room_notes`
//! so cleanup is exact-match and never touches concurrent tests'
//! data. Test data is torn down even on panic via `cleanup_b5_fixture`.

mod common;

use chrono::{Duration, Local, NaiveDate, NaiveDateTime};

use hotel_backend::sync::backfill::{backfill_one_folio_with_aggregate, FolioOutcome};
use hotel_backend::sync::parent_loader::CheckInAggregate;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const TEST_MARKER: &str = "TEST_B5";

/// Yesterday in BKK wall-clock — `cin_checkin_time` is stored as
/// `TIMESTAMP` (naive) representing local Bangkok time.
fn yesterday_bkk() -> NaiveDateTime {
    (Local::now() - Duration::days(1)).naive_local()
}

fn tomorrow_bkk_date() -> NaiveDate {
    (Local::now() + Duration::days(1)).date_naive()
}

/// Generate suffixes unique enough to allow parallel test runs against
/// the same DB without colliding `room_no` / `cin_no` values.
fn unique_suffix(tag: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    // Use a 5-digit slice so the resulting room_no stays under
    // VARCHAR(10) and cin_no under VARCHAR(20).
    format!("{}{:05}", tag, (nanos % 100_000) as u32)
}

/// Build a `HT_CheckIn_H` mock header row matching the post-B2 shape
/// the CT watcher's `parent_loader::load_checkin_aggregate` produces.
fn header_row(cin_no: &str, cust_no: &str) -> HashMapRow {
    HashMapRow::new("HT_CheckIn_H")
        .with("Cin_no", MockValue::Str(cin_no.into()))
        .with("Cin_status", MockValue::Str("ปกติ".into()))
        .with("Cin_Book_no", MockValue::Str(String::new()))
        .with("Cin_cust_no", MockValue::Str(cust_no.into()))
        .with(
            "Cin_Date_in",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 13)
                    .unwrap()
                    .and_hms_opt(14, 30, 0)
                    .unwrap(),
            ),
        )
        .with(
            "Cin_Date_Out",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 14)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            ),
        )
        .with("Total_Price_Room", MockValue::Decimal(1780.0))
        .with("Total_Price_Net", MockValue::Decimal(1780.0))
        .with("Total_Price_Pay", MockValue::Decimal(0.0))
        .with("Total_Price_Balance", MockValue::Decimal(1780.0))
}

/// One `HT_CheckIn_Ds` row per room, mirroring the per-room column
/// set the B2 projection (`project_rooms`) consumes.
fn ds_row(cin_no: &str, room_no: &str, ds_id: i32) -> HashMapRow {
    HashMapRow::new("HT_CheckIn_Ds")
        .with("id", MockValue::I32(ds_id))
        .with("Cin_No", MockValue::Str(cin_no.into()))
        .with("Cin_Room_No", MockValue::Str(room_no.into()))
        .with("Cin_Room_Status", MockValue::Str("เข้าพัก".into()))
        .with("Cin_Room_In", MockValue::Null)
        .with("Cin_Room_Out", MockValue::Null)
        .with("Cin_Room_Price", MockValue::Decimal(890.0))
        .with("Cin_Room_Night", MockValue::I32(1))
        .with("Cin_Room_PriceToTal", MockValue::Decimal(890.0))
        .with("Cin_dep", MockValue::Decimal(0.0))
        .with("Cin_dep_status", MockValue::Null)
        .with("Cin_dep_returned", MockValue::Null)
        .with("Cin_dep_returned_by", MockValue::Null)
}

/// Fixture handle returned by `seed_pre_b2_two_room_folio` — carries
/// every primary key the cleanup helper needs to roll the seed back.
struct Fixture {
    cust_id: i32,
    room_id_a: i32,
    room_id_b: i32,
    room_no_a: String,
    room_no_b: String,
    cin_id: i32,
    cin_no: String,
    legacy_cust_no: String,
}

/// Seed the pre-B2 shape of a multi-room folio:
///   1 customer, 2 rooms, 1 `ht_checkins` row pointing at room A via
///   `cin_room_id`, NO `ht_checkin_rooms` rows.
///
/// This is the shape backfill should fix: room B is invisible to
/// dashboard reads because it's not in the junction. Post-B5 the
/// junction has 2 rows.
async fn seed_pre_b2_two_room_folio(pool: &sqlx::PgPool, tag: &str) -> Fixture {
    let suffix = unique_suffix(tag);
    let room_no_a = format!("TA{}", &suffix[2..]);
    let room_no_b = format!("TB{}", &suffix[2..]);
    let cin_no = format!("CB5-{}", &suffix[..]);
    let legacy_cust_no = format!("CB5{}", &suffix[..]);

    let cust_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no, cust_notes) \
         VALUES ('TEST_B5_cust', $1, $2) \
         RETURNING cust_id",
    )
    .bind(&legacy_cust_no)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed customer");

    let room_id_a: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes, room_active) \
         VALUES ($1, true, $2, true) \
         RETURNING room_id",
    )
    .bind(&room_no_a)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed room A");

    let room_id_b: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes, room_active) \
         VALUES ($1, true, $2, true) \
         RETURNING room_id",
    )
    .bind(&room_no_b)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed room B");

    // Pre-B2 shape: `cin_room_id` points at the FIRST room only —
    // exactly what the old mapper wrote. The junction is empty; B5
    // must populate it.
    let cin_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_checkins ( \
             cin_no, legacy_cin_no, cin_cust_id, cin_room_id, \
             legacy_cust_no, legacy_room_no, \
             cin_checkin_time, cin_expected_checkout, cin_status, cin_notes \
         ) VALUES ($1, $1, $2, $3, $4, $5, $6, $7, 'active', $8) \
         RETURNING cin_id",
    )
    .bind(&cin_no)
    .bind(cust_id)
    .bind(room_id_a)
    .bind(&legacy_cust_no)
    .bind(&room_no_a)
    .bind(yesterday_bkk())
    .bind(tomorrow_bkk_date())
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed checkin");

    Fixture {
        cust_id,
        room_id_a,
        room_id_b,
        room_no_a,
        room_no_b,
        cin_id,
        cin_no,
        legacy_cust_no,
    }
}

/// Build a 2-room aggregate referencing the seeded room numbers /
/// customer. The aggregate shape matches what
/// `parent_loader::load_checkin_aggregate` returns from MSSQL.
fn build_two_room_aggregate(fx: &Fixture) -> CheckInAggregate {
    CheckInAggregate {
        header: Some(header_row(&fx.cin_no, &fx.legacy_cust_no)),
        rooms: vec![
            ds_row(&fx.cin_no, &fx.room_no_a, 70001),
            ds_row(&fx.cin_no, &fx.room_no_b, 70002),
        ],
        payments: vec![],
    }
}

async fn count_junction_rows(pool: &sqlx::PgPool, cin_id: i32) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)::bigint FROM ht_checkin_rooms WHERE cr_cin_id = $1",
    )
    .bind(cin_id)
    .fetch_one(pool)
    .await
    .unwrap_or(0)
}

/// Tear down a fixture by its primary keys — children CASCADE from
/// `ht_checkins`, so the junction goes with it.
async fn cleanup_b5_fixture(pool: &sqlx::PgPool, fx: &Fixture) {
    sqlx::query("DELETE FROM ht_checkins WHERE cin_id = $1")
        .bind(fx.cin_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_id IN ($1, $2)")
        .bind(fx.room_id_a)
        .bind(fx.room_id_b)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(fx.cust_id)
        .execute(pool)
        .await
        .ok();
}

// ═════════════════════════════════════════════════════════════════════
// Test 1: materialises both junction rows for a 2-room folio
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn backfill_materializes_two_room_folio() {
    let pool = common::create_test_pool().await;
    let fx = seed_pre_b2_two_room_folio(&pool, "mt").await;

    // Pre-condition: junction is empty (pre-B2 shape).
    assert_eq!(
        count_junction_rows(&pool, fx.cin_id).await,
        0,
        "pre-condition: pre-B2 folio must have zero junction rows"
    );

    let aggregate = build_two_room_aggregate(&fx);
    let outcome =
        backfill_one_folio_with_aggregate(&pool, None, &fx.cin_no, &aggregate, false).await;

    let post_count = count_junction_rows(&pool, fx.cin_id).await;
    cleanup_b5_fixture(&pool, &fx).await;

    assert_eq!(
        outcome,
        FolioOutcome::Applied,
        "Track B5 / T1 CRIT-1: 2-room pre-B2 folio must report Applied"
    );
    assert_eq!(
        post_count, 2,
        "Track B5 / T1 CRIT-1: must materialise one junction row per legacy Ds row (got {})",
        post_count
    );
}

// ═════════════════════════════════════════════════════════════════════
// Test 2: re-running on an already-backfilled folio is a no-op
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn backfill_idempotent_on_already_backfilled_folio() {
    let pool = common::create_test_pool().await;
    let fx = seed_pre_b2_two_room_folio(&pool, "id").await;

    let aggregate = build_two_room_aggregate(&fx);

    // First run — populates the junction.
    let first =
        backfill_one_folio_with_aggregate(&pool, None, &fx.cin_no, &aggregate, false).await;
    let after_first = count_junction_rows(&pool, fx.cin_id).await;

    // Second run — must be a no-op.
    let second =
        backfill_one_folio_with_aggregate(&pool, None, &fx.cin_no, &aggregate, false).await;
    let after_second = count_junction_rows(&pool, fx.cin_id).await;

    cleanup_b5_fixture(&pool, &fx).await;

    assert_eq!(first, FolioOutcome::Applied, "first run must apply");
    assert_eq!(after_first, 2, "first run must materialise both rooms");
    assert_eq!(
        second,
        FolioOutcome::SkippedIdempotent,
        "Track B5 idempotency: second run must report SkippedIdempotent (got {:?})",
        second
    );
    assert_eq!(
        after_second, after_first,
        "junction row count must not change on a no-op re-run"
    );
}

// ═════════════════════════════════════════════════════════════════════
// Test 3: folio with no PG row is skipped (warning, not error)
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn backfill_skips_folio_missing_in_pg() {
    let pool = common::create_test_pool().await;
    // Build an aggregate for a cin_no that does NOT exist in
    // `ht_checkins`. The bin should treat this as a "CT watcher
    // hasn't caught up" case — warn + skip, not error.
    let phantom_cin_no = format!("CB5-PHANTOM-{}", unique_suffix("ph"));
    let aggregate = CheckInAggregate {
        header: Some(header_row(&phantom_cin_no, "PHANTOMCUST")),
        rooms: vec![ds_row(&phantom_cin_no, "999", 70099)],
        payments: vec![],
    };

    // Sanity: no PG row exists for this cin_no.
    let pg_present: Option<(i32,)> = sqlx::query_as(
        "SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1 LIMIT 1",
    )
    .bind(&phantom_cin_no)
    .fetch_optional(&pool)
    .await
    .expect("phantom cin presence probe");
    assert!(
        pg_present.is_none(),
        "test pre-condition: phantom cin must not exist in ht_checkins"
    );

    let outcome =
        backfill_one_folio_with_aggregate(&pool, None, &phantom_cin_no, &aggregate, false)
            .await;

    // Confirm no junction row was inserted via a cross-cin probe.
    let phantom_junction_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1",
    )
    .bind(&phantom_cin_no)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    assert_eq!(
        outcome,
        FolioOutcome::SkippedMissingPg,
        "Track B5: folio missing in PG must report SkippedMissingPg \
         (got {:?}) — the CT watcher must catch up before backfill",
        outcome
    );
    assert_eq!(
        phantom_junction_count, 0,
        "no junction row may be inserted when the parent ht_checkins row is missing"
    );
}

// ═════════════════════════════════════════════════════════════════════
// Test 4: --dry-run reports would-Apply but writes nothing
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn backfill_dry_run_emits_no_writes() {
    let pool = common::create_test_pool().await;
    let fx = seed_pre_b2_two_room_folio(&pool, "dr").await;

    let aggregate = build_two_room_aggregate(&fx);
    let outcome =
        backfill_one_folio_with_aggregate(&pool, None, &fx.cin_no, &aggregate, true).await;

    let post_count = count_junction_rows(&pool, fx.cin_id).await;
    cleanup_b5_fixture(&pool, &fx).await;

    assert_eq!(
        outcome,
        FolioOutcome::Applied,
        "dry-run on a divergent folio must still report Applied so the operator \
         sees that an apply run would do work"
    );
    assert_eq!(
        post_count, 0,
        "Track B5 --dry-run must write nothing (junction count must stay at 0)"
    );
}
