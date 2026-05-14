//! Phase 5.4 integration tests for the check-in + payment CT mappers.
//!
//! Drives `apply_checkin_aggregate` directly with `CheckInAggregate`
//! fixtures (no MSSQL connection required) and asserts the canonical
//! PG row + `event_log` content match expectations.
//!
//! Skipped silently when `DATABASE_URL` is unreachable — the unit
//! tests in `sync::mappers::checkin` and `sync::mappers::payment`
//! cover the pure projection logic; this suite covers the UPSERT +
//! idempotency + event-persistence loop.
//!
//! Note: The check-in mapper takes a live MSSQL pool for the optional
//! checkout side-effect (parent booking re-projection). For walk-in
//! flows that side-effect is a no-op, so we pass a thin `mssql_stub`
//! pool that's never actually used. The booking-linked side-effect
//! path is exercised end-to-end against a live MSSQL only in Phase 5.5
//! once the watcher is cut over.

mod common;

use chrono::NaiveDate;
use hotel_backend::sync::mappers::{
    apply_checkin_aggregate, resolve_customer_via_eager_mirror_for_test,
};
use hotel_backend::sync::parent_loader::CheckInAggregate;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const HT_CHECKIN_H: &str = "HT_CheckIn_H";
const HT_CHECKIN_DS: &str = "HT_CheckIn_Ds";
const HT_CUSTOMERS: &str = "HT_Customers";

/// Helper — generate a unique `Cin_no` so re-runs don't clash.
fn unique_cin_no() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("CT26-{:06}", (nanos % 1_000_000) as u32)
}

fn unique_cust_no() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("CIT{:06}", (nanos % 1_000_000) as u32 + 1)
}

fn unique_room_no() -> String {
    format!("Y{:03}", (rand::random::<u8>() as u16) % 999 + 1)
}

fn header_row(cin_no: &str, cust_no: &str, status: &str) -> HashMapRow {
    HashMapRow::new(HT_CHECKIN_H)
        .with("Cin_no", MockValue::Str(cin_no.into()))
        .with("Cin_status", MockValue::Str(status.into()))
        .with("Cin_Book_no", MockValue::Str(String::new()))
        .with("Cin_cust_no", MockValue::Str(cust_no.into()))
        .with(
            "Cin_Date_in",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 1)
                    .unwrap()
                    .and_hms_opt(14, 30, 0)
                    .unwrap(),
            ),
        )
        .with(
            "Cin_Date_Out",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 2)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            ),
        )
        .with("Total_Price_Room", MockValue::Decimal(890.0))
        .with("Total_Price_Net", MockValue::Decimal(890.0))
        .with("Total_Price_Pay", MockValue::Decimal(0.0))
        .with("Total_Price_Balance", MockValue::Decimal(890.0))
}

fn ds_row(cin_no: &str, room_no: &str, status: &str) -> HashMapRow {
    HashMapRow::new(HT_CHECKIN_DS)
        .with("id", MockValue::I32(25001))
        .with("Cin_No", MockValue::Str(cin_no.into()))
        .with("Cin_Room_No", MockValue::Str(room_no.into()))
        .with("Cin_Room_Status", MockValue::Str(status.into()))
        .with("Cin_Room_In", MockValue::Null)
        .with("Cin_Room_Out", MockValue::Null)
        // Track B2 — per-room operational columns the projection now
        // mirrors into `ht_checkin_rooms`. Defaults mirror the
        // writeback recipe's `walkin::build_statements` payload.
        .with("Cin_Room_Price", MockValue::Decimal(890.0))
        .with("Cin_Room_Night", MockValue::I32(1))
        .with("Cin_Room_PriceToTal", MockValue::Decimal(890.0))
        .with("Cin_Room_Dep", MockValue::Decimal(0.0))
        .with("Cin_Dep_Status", MockValue::Null)
        .with("Cin_Dep_return_date", MockValue::Null)
        .with("Cin_Dep_return_by", MockValue::Null)
}

fn ds_row_checked_out(cin_no: &str, room_no: &str) -> HashMapRow {
    ds_row(cin_no, room_no, "Check-Out").with(
        "Cin_Room_Out",
        MockValue::DateTime(
            NaiveDate::from_ymd_opt(2026, 5, 2)
                .unwrap()
                .and_hms_opt(11, 30, 0)
                .unwrap(),
        ),
    )
}

/// Seed a customer in PG so the check-in mapper's customer FK
/// resolver can find it.
async fn seed_customer(pool: &sqlx::PgPool, legacy_cust_no: &str) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no, cust_notes) \
         VALUES ('TEST_phase54_cust', $1, 'TEST_phase54') \
         RETURNING cust_id",
    )
    .bind(legacy_cust_no)
    .fetch_one(pool)
    .await
    .expect("seed customer")
}

/// Seed a room in PG so the room-FK resolver finds it by `room_no`.
async fn seed_room(pool: &sqlx::PgPool, room_no: &str) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes) \
         VALUES ($1, true, 'TEST_phase54_room') \
         ON CONFLICT (room_no) DO UPDATE SET room_clean = EXCLUDED.room_clean \
         RETURNING room_id",
    )
    .bind(room_no)
    .fetch_one(pool)
    .await
    .expect("seed room")
}

async fn cleanup(pool: &sqlx::PgPool, cin_no: &str, cust_no: &str, room_no: &str) {
    let agg: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(cin_no)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();
    if let Some(a) = agg {
        sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(a)
            .execute(pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_payments WHERE pay_cin_id IN \
                 (SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1)")
        .bind(cin_no)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_checkins WHERE legacy_cin_no = $1")
        .bind(cin_no)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(cust_no)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(room_no)
        .execute(pool)
        .await
        .ok();
}

/// `apply_checkin_aggregate` requires a `&DbPool` (legacy MSSQL) for
/// the optional checkout side-effect that re-projects a parent
/// booking. The walk-in flows here never trigger that side-effect (no
/// `Cin_Book_no`), so the pool is never actually borrowed. We construct
/// it lazily and tolerate failures so the suite runs anywhere PG is
/// available, even without legacy MSSQL (the booking-linked side-effect
/// path is exercised end-to-end against a live MSSQL only in Phase 5.5
/// once the watcher is cut over).
///
/// Phase 5.5 QoL: `SYNC_TEST_SKIP_MSSQL_PROBE=true` skips the pool-init
/// probe entirely and returns `None`. Use this for pure-PG test runs that
/// don't need MSSQL (the bb8-tiberius probe otherwise blocks ~30s when
/// MSSQL is unreachable, before the test can even start). See
/// `docs/runbook-sync.md` env-var matrix.
async fn mssql_stub() -> Option<hotel_backend::db::DbPool> {
    if std::env::var("SYNC_TEST_SKIP_MSSQL_PROBE")
        .map(|v| v == "true")
        .unwrap_or(false)
    {
        return None;
    }
    // `DbConfig::from_env` panics when `DB_PASSWORD` is unset (the
    // 2.54.6 fail-loud removed the hardcoded fallback). In CI / pure-PG
    // runs that env var is intentionally not provided, so treat its
    // absence as the same signal as `SYNC_TEST_SKIP_MSSQL_PROBE=true`:
    // skip the probe rather than panic the test process.
    if std::env::var("DB_PASSWORD")
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        return None;
    }
    let config = hotel_backend::config::DbConfig::from_env();
    hotel_backend::db::create_pool(&config).await.ok()
}

#[tokio::test]
async fn checkin_walkin_upserts_pg_row_and_writes_event_log() {
    let pool = common::create_test_pool().await;
    // Walk-in flows never trigger the parent-booking re-projection
    // side-effect, so MSSQL is genuinely optional here.
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };

    let mut tx = pool.begin().await.expect("begin");
    let event = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .expect("apply must succeed");
    assert!(event.is_some(), "fresh aggregate must emit an event");
    let event = event.unwrap();
    assert_eq!(event.type_name(), "CheckInCreated");

    hotel_backend::outbox::bus::EventBus::publish(&mut tx, &event)
        .await
        .expect("publish");
    tx.commit().await.expect("commit");

    // Canonical row landed.
    let (cin_id, agg_id, status, total): (
        i32,
        Option<uuid::Uuid>,
        String,
        Option<f64>,
    ) = sqlx::query_as(
        "SELECT cin_id, aggregate_id, cin_status, cin_total_amount::float8 \
           FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(agg_id.is_some());
    assert_eq!(status, "active");
    assert_eq!(total, Some(890.0));
    let _ = cin_id;

    // event_log carries exactly one CheckInCreated row.
    let event_kinds: Vec<String> = sqlx::query_scalar(
        "SELECT event_type FROM event_log WHERE aggregate_id = $1 ORDER BY created_at",
    )
    .bind(agg_id.unwrap())
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(event_kinds, vec!["CheckInCreated".to_string()]);

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

#[tokio::test]
async fn checkin_re_apply_with_identical_aggregate_skips_event() {
    if std::env::var("MSSQL_HOST").is_err() {
        return;
    }
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };

    // First apply — emits CheckInCreated.
    let mut tx = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .unwrap();
    assert!(event.is_some());
    if let Some(e) = event {
        hotel_backend::outbox::bus::EventBus::publish(&mut tx, &e)
            .await
            .ok();
    }
    tx.commit().await.unwrap();

    // Second apply — must NOT emit.
    let mut tx2 = pool.begin().await.unwrap();
    let event2 = apply_checkin_aggregate(&mut tx2, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .unwrap();
    tx2.commit().await.unwrap();
    assert!(
        event2.is_none(),
        "re-apply with identical aggregate must skip event publication"
    );

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

#[tokio::test]
async fn checkin_status_update_to_yokleek_emits_checkin_cancelled() {
    if std::env::var("MSSQL_HOST").is_err() {
        return;
    }
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed.
    let initial = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &initial, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Header flips to ยกเลิก.
    let cancelled = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ยกเลิก")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx2, mssql.as_ref(), &cancelled, &cin_no)
        .await
        .unwrap();
    tx2.commit().await.unwrap();
    let event = event.expect("status flip must emit");
    assert_eq!(event.type_name(), "CheckInCancelled");

    let status: String = sqlx::query_scalar(
        "SELECT cin_status FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status, "cancelled");

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

#[tokio::test]
async fn checkin_delete_marks_status_cancelled_and_emits_cancelled_event() {
    if std::env::var("MSSQL_HOST").is_err() {
        return;
    }
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed.
    let initial = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &initial, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Header gone — simulate the legacy delete path.
    let gone = CheckInAggregate {
        header: None,
        rooms: vec![],
        payments: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx2, mssql.as_ref(), &gone, &cin_no)
        .await
        .unwrap();
    tx2.commit().await.unwrap();
    let event = event.expect("delete must emit");
    assert_eq!(event.type_name(), "CheckInCancelled");

    let status: String = sqlx::query_scalar(
        "SELECT cin_status FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status, "cancelled");

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

#[tokio::test]
async fn full_checkout_flips_status_to_checked_out_and_emits_checkout_completed() {
    if std::env::var("MSSQL_HOST").is_err() {
        return;
    }
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed an active stay.
    let active = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &active, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Every Ds row flips to Check-Out.
    let checked_out = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row_checked_out(&cin_no, &room_no)],
        payments: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx2, mssql.as_ref(), &checked_out, &cin_no)
        .await
        .unwrap();
    tx2.commit().await.unwrap();
    let event = event.expect("checkout must emit");
    assert_eq!(event.type_name(), "CheckOutCompleted");

    let (status, checkout_time): (String, Option<chrono::NaiveDateTime>) = sqlx::query_as(
        "SELECT cin_status, cin_checkout_time \
           FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status, "checked_out");
    assert!(checkout_time.is_some(), "checkout_time must be set");

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

#[tokio::test]
async fn checkin_apply_defers_when_customer_not_yet_mirrored() {
    if std::env::var("MSSQL_HOST").is_err() {
        return;
    }
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no(); // intentionally NOT seeded
    let room_no = unique_room_no();

    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .unwrap();
    tx.rollback().await.ok();
    assert!(
        event.is_none(),
        "apply must defer (Ok(None)) when customer FK isn't resolvable yet"
    );

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 0, "no row must be inserted on defer");

    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(&room_no)
        .execute(&pool)
        .await
        .ok();
}

/// Regression for the v2.63.0 strand bug.
///
/// Production scenario: legacy iHOTEL cancels a check-in by setting
/// `HT_CheckIn_H.Cin_status='ยกเลิก'` AND deleting every `HT_CheckIn_Ds`
/// row. The next CT tick brings a header-present aggregate with
/// `rooms.is_empty()`. `derive_room_state` returns `first_room_no=None`,
/// so the normal FK-resolution path defers indefinitely
/// (`resolve_room_id(None) -> None`) — leaving the canonical row stuck
/// at `cin_status='active'` forever.
///
/// Expected behaviour after the fix: when an existing canonical row is
/// found AND the projection's `cin_status` is `'cancelled'`, the mapper
/// short-circuits to a `cin_status='cancelled'` UPDATE on the existing
/// row WITHOUT requiring a resolvable room FK. No domain event is
/// emitted (recovery, not transition).
#[tokio::test]
async fn checkin_cancelled_with_deleted_ds_rows_lands_via_short_circuit() {
    // No MSSQL gate — the short-circuit path never triggers the parent
    // booking re-projection side-effect (walk-in: empty Cin_Book_no),
    // so the mssql_stub is genuinely optional.
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed: original active check-in lands normally.
    let active = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &active, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Sanity: the canonical row is 'active'.
    let status_before: String = sqlx::query_scalar(
        "SELECT cin_status FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status_before, "active");

    // Legacy: header flipped to ยกเลิก AND every HT_CheckIn_Ds row
    // deleted. Header is still present — so `is_present()` is true and
    // the aggregate goes through `project_aggregate` (NOT through the
    // header-gone `apply_cancelled` path).
    let cancelled_with_deleted_rooms = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ยกเลิก")),
        rooms: vec![], // <- legacy deleted all Ds rows
        payments: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(
        &mut tx2,
        mssql.as_ref(),
        &cancelled_with_deleted_rooms,
        &cin_no,
    )
    .await
    .unwrap();
    tx2.commit().await.unwrap();

    // No event emitted — short-circuit path is a recovery, not a
    // transition. The cancellation transition was already captured by
    // the header-status flip (and is observed on the canonical row).
    assert!(
        event.is_none(),
        "short-circuit cancellation must not emit a domain event"
    );

    // Canonical row must now be cancelled — the bug was that it would
    // stay 'active' forever because room FK resolution deferred.
    let status_after: String = sqlx::query_scalar(
        "SELECT cin_status FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        status_after, "cancelled",
        "cin_status must flip to 'cancelled' even with no resolvable room"
    );

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

/// Companion regression: when NO canonical row exists yet, a cancelled
/// header (with deleted Ds rows) must still defer — the original INSERT
/// CT row hasn't landed yet, so there's nothing to update. The
/// short-circuit is guarded by `existing.is_some()`.
#[tokio::test]
async fn checkin_cancelled_with_no_existing_row_still_defers() {
    // Same MSSQL exemption as the sibling test above.
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // No prior INSERT — first CT tick we ever see for this cin_no is
    // the cancellation itself (with all Ds rows already deleted).
    let cancelled = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ยกเลิก")),
        rooms: vec![],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &cancelled, &cin_no)
        .await
        .unwrap();
    tx.rollback().await.ok();

    assert!(
        event.is_none(),
        "no canonical row exists yet — short-circuit must not fire blind"
    );
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 0, "no row must be inserted by the short-circuit path");

    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&cust_no)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(&room_no)
        .execute(&pool)
        .await
        .ok();
}

// =============================================================================
// Eager-mirror regression — checkin references an un-mirrored customer.
//
// Before this fix the checkin mapper logged a WARN and returned Ok(None)
// when canonical `ht_customers` didn't yet carry the referenced
// `legacy_cust_no`. The CT watermark advanced past the deferred row, so
// recovery depended on a later CT update for the same checkin re-firing
// the aggregate load — defer-forever in production catch-up scenarios.
//
// The mapper now eagerly fetches the missing `HT_Customers` row from
// MSSQL (via the test-injected supplier here) and INSERTs it into
// canonical `ht_customers` in the same TX, then retries the FK lookup.
// =============================================================================

/// Construct a stub `HT_Customers` row in the shape the eager fetcher
/// expects. Mirrors `parent_loader::fetch_rows` materialisation: every
/// column from `customer::EAGER_FETCH_COLUMNS` is present, either as a
/// Str/Null value.
fn stub_customers_row(cust_no: &str, name: &str) -> HashMapRow {
    // Track E2 (T1 HIGH-2 / T2 HIGH-4) widened
    // `customer::EAGER_FETCH_COLUMNS` from 8 to 33. Every column must
    // be present in the stub row or the projection's `try_get_str`
    // probe errors out at apply time.
    HashMapRow::new(HT_CUSTOMERS)
        .with("Cust_no", MockValue::Str(cust_no.into()))
        .with("Cust_name", MockValue::Str(name.into()))
        .with("Cust_name2", MockValue::Null)
        .with("Cust_perfix", MockValue::Str("นาย".into()))
        .with("Cust_sex", MockValue::Null)
        .with("Cust_IDcard", MockValue::Null)
        .with("Cust_Type", MockValue::Null)
        .with(
            "Cust_Type_Main",
            MockValue::Str("บุคคลธรรมดา".into()),
        )
        .with("Cust_Email", MockValue::Null)
        .with("Cust_Add_no", MockValue::Null)
        .with("Cust_Add_moo", MockValue::Null)
        .with("Cust_Add_soi", MockValue::Null)
        .with("Cust_Add_road", MockValue::Null)
        .with("Cust_Add_tambon", MockValue::Null)
        .with("Cust_Add_ampore", MockValue::Null)
        .with("Cust_Add_province", MockValue::Null)
        .with("Cust_Add_code", MockValue::Null)
        .with("Cust_Add_tel", MockValue::Str("0801112222".into()))
        .with("Cust_Add_fax", MockValue::Null)
        .with("Cust_Work_Name", MockValue::Null)
        .with("Cust_Work_no", MockValue::Null)
        .with("Cust_Work_moo", MockValue::Null)
        .with("Cust_Work_soi", MockValue::Null)
        .with("Cust_Work_road", MockValue::Null)
        .with("Cust_Work_tambon", MockValue::Null)
        .with("Cust_Work_ampore", MockValue::Null)
        .with("Cust_Work_province", MockValue::Null)
        .with("Cust_Work_code", MockValue::Null)
        .with("Cust_Work_tel", MockValue::Null)
        .with("Cust_Work_fax", MockValue::Null)
        .with("Cust_Work_Tax", MockValue::Null)
        .with("Cust_Last_Change", MockValue::Null)
        .with("Cust_Contry", MockValue::Null)
        .with("Cust_Price_Over", MockValue::Null)
}

#[tokio::test]
async fn eager_mirror_inserts_customer_when_canonical_row_missing() {
    let pool = common::create_test_pool().await;
    let cust_no = unique_cust_no();

    // Pre-condition: canonical ht_customers has NO row for this cust_no.
    let count_before: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count_before, 0, "test pre-condition: no canonical row");

    // Drive the eager-mirror helper with a stub supplier that returns
    // the synthetic HT_Customers row the legacy MSSQL would have.
    let mut tx = pool.begin().await.unwrap();
    let stub_cust_no = cust_no.clone();
    let cust_id = resolve_customer_via_eager_mirror_for_test(
        &mut tx,
        &cust_no,
        move |requested| {
            assert_eq!(requested, stub_cust_no, "supplier called with wrong cust_no");
            Some(stub_customers_row(&stub_cust_no, "ทดสอบ EagerMirror"))
        },
    )
    .await
    .expect("eager mirror must succeed when supplier returns a row");
    let cust_id = cust_id.expect("eager mirror must yield a resolved cust_id");
    tx.commit().await.unwrap();

    // Post-condition: canonical row landed with the projected columns.
    let (returned_cust_id, firstname, title, phone, cust_type): (
        i32,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
    ) = sqlx::query_as(
        "SELECT cust_id, cust_firstname, cust_title, cust_phone, cust_type \
           FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(returned_cust_id, cust_id);
    assert_eq!(firstname, "ทดสอบ EagerMirror");
    assert_eq!(title.as_deref(), Some("นาย"));
    assert_eq!(phone.as_deref(), Some("0801112222"));
    assert_eq!(cust_type.as_deref(), Some("บุคคลธรรมดา"));

    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&cust_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn eager_mirror_path_lets_checkin_apply_succeed_without_pre_seeded_customer() {
    // Full end-to-end: drive `apply_checkin_aggregate` against an
    // aggregate whose Cin_cust_no is NOT yet in canonical ht_customers.
    // The eager mirror plumbing pulls the row in-band via the stub
    // supplier, so the checkin INSERT succeeds with a resolved FK
    // rather than deferring with the old defer-forever WARN.
    let pool = common::create_test_pool().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    // Pre-seed the room so room-FK resolution succeeds; deliberately
    // do NOT seed the customer — that's the path under test.
    let _room_id = seed_room(&pool, &room_no).await;

    // First exercise the eager-mirror helper to land the customer row.
    // Production wires this up via the live MSSQL pool; here we use the
    // test-injection seam (see `eager_mirror_inserts_customer_when_...`).
    {
        let mut tx = pool.begin().await.unwrap();
        let stub_cust_no = cust_no.clone();
        let cust_id = resolve_customer_via_eager_mirror_for_test(
            &mut tx,
            &cust_no,
            move |_| Some(stub_customers_row(&stub_cust_no, "EagerWalkInGuest")),
        )
        .await
        .unwrap()
        .expect("supplier returned a row -> eager mirror must yield cust_id");
        assert!(cust_id > 0);
        tx.commit().await.unwrap();
    }

    // With the canonical customer now present, the standard apply path
    // resolves the FK without invoking the eager-mirror branch.
    let mssql = mssql_stub().await;
    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let event = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .expect("apply must succeed once customer is mirrored");
    tx.commit().await.unwrap();
    assert!(
        event.is_some(),
        "checkin apply must emit an event when FK resolution succeeds"
    );
    assert_eq!(event.unwrap().type_name(), "CheckInCreated");

    // Canonical checkin row landed with a non-null cin_cust_id.
    let cin_cust_id: i32 = sqlx::query_scalar(
        "SELECT cin_cust_id FROM ht_checkins WHERE legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(cin_cust_id > 0, "cin_cust_id must point at the eager-mirrored row");

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}

#[tokio::test]
async fn eager_mirror_defers_when_supplier_returns_no_row() {
    // Defensive fallback: when the customer is truly missing in MSSQL
    // too (should never happen under the legacy app's ordering), the
    // helper returns Ok(None) so the apply path defers cleanly rather
    // than crashing. Distinct from the old defer-forever — this case
    // is logged with a different message so production can tell the
    // two apart.
    let pool = common::create_test_pool().await;
    let cust_no = unique_cust_no();

    let mut tx = pool.begin().await.unwrap();
    let result = resolve_customer_via_eager_mirror_for_test(
        &mut tx,
        &cust_no,
        |_| None, // simulate "row not in MSSQL either"
    )
    .await
    .expect("missing-in-MSSQL must not error, just defer");
    tx.rollback().await.ok();

    assert!(
        result.is_none(),
        "supplier returning None must defer (Ok(None)), not insert anything"
    );
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 0, "no canonical row may be inserted on the defer path");
}

// =============================================================================
// Track B2 — multi-room sync mapper (T2 CRIT-1, audit-2026-05-13)
//
// The headline correctness fix: every `HT_CheckIn_Ds` row in the legacy
// aggregate must land as its own `ht_checkin_rooms` row. Pre-B2 the
// mapper collapsed everything to the first room — rooms 2..N were
// invisible to canonical state.
// =============================================================================

/// Helper: read every `(room_no, cr_room_status)` pair for a folio
/// identified by its `legacy_cin_no`. Empty when the folio is missing.
async fn checkin_rooms_for_folio(pool: &sqlx::PgPool, cin_no: &str) -> Vec<(String, String)> {
    sqlx::query_as::<_, (String, String)>(
        "SELECT r.room_no, cr.cr_room_status \
           FROM ht_checkin_rooms cr \
           JOIN ht_rooms_new r ON r.room_id = cr.cr_room_id \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1 \
          ORDER BY r.room_no",
    )
    .bind(cin_no)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

/// Track B2 / T2 CRIT-1 — headline test. A 2-room walk-in aggregate
/// MUST emit exactly TWO `ht_checkin_rooms` rows. Pre-B2 only the
/// first room landed (T2 CRIT-1).
#[tokio::test]
async fn apply_checkin_aggregate_creates_one_ht_checkin_rooms_row_per_legacy_ds_row() {
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no_a = unique_room_no();
    let room_no_b = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_a = seed_room(&pool, &room_no_a).await;
    let _room_b = seed_room(&pool, &room_no_b).await;

    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![
            ds_row(&cin_no, &room_no_a, "เข้าพัก"),
            // Second Ds row needs a distinct legacy id so the partial
            // index `ht_checkin_rooms_legacy_ds_id` doesn't collide
            // with a NULL-ambiguity in re-runs.
            ds_row(&cin_no, &room_no_b, "เข้าพัก")
                .with("id", hotel_backend::sync::row::test_support::MockValue::I32(25002)),
        ],
        payments: vec![],
    };

    let mut tx = pool.begin().await.expect("begin");
    let event = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .expect("apply must succeed");
    assert!(event.is_some(), "fresh multi-room aggregate must emit an event");
    if let Some(e) = event {
        hotel_backend::outbox::bus::EventBus::publish(&mut tx, &e)
            .await
            .expect("publish");
    }
    tx.commit().await.expect("commit");

    let rooms = checkin_rooms_for_folio(&pool, &cin_no).await;
    assert_eq!(
        rooms.len(),
        2,
        "multi-room folio must materialise as TWO ht_checkin_rooms rows \
         (pre-B2 only the first room landed) — got {rooms:?}"
    );
    let room_nos: Vec<&str> = rooms.iter().map(|(no, _)| no.as_str()).collect();
    assert!(
        room_nos.contains(&room_no_a.as_str()),
        "first room must be present"
    );
    assert!(
        room_nos.contains(&room_no_b.as_str()),
        "second room must be present (pre-B2 this was the bug)"
    );

    cleanup(&pool, &cin_no, &cust_no, &room_no_a).await;
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(&room_no_b)
        .execute(&pool)
        .await
        .ok();
}

/// Re-applying the exact same multi-room aggregate must NOT change the
/// junction state (idempotency under Track B2 / T2 HIGH-2).
#[tokio::test]
async fn apply_checkin_aggregate_upserts_idempotently_on_re_apply() {
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no_a = unique_room_no();
    let room_no_b = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_a = seed_room(&pool, &room_no_a).await;
    let _room_b = seed_room(&pool, &room_no_b).await;

    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![
            ds_row(&cin_no, &room_no_a, "เข้าพัก"),
            ds_row(&cin_no, &room_no_b, "เข้าพัก")
                .with("id", hotel_backend::sync::row::test_support::MockValue::I32(25002)),
        ],
        payments: vec![],
    };

    // First apply lands the two rows.
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let after_first = checkin_rooms_for_folio(&pool, &cin_no).await;
    assert_eq!(after_first.len(), 2);

    // Re-apply with identical aggregate.
    let mut tx2 = pool.begin().await.unwrap();
    let event2 = apply_checkin_aggregate(&mut tx2, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .unwrap();
    tx2.commit().await.unwrap();
    assert!(
        event2.is_none(),
        "idempotent re-apply must skip the event publish path"
    );

    // Junction state unchanged.
    let after_second = checkin_rooms_for_folio(&pool, &cin_no).await;
    assert_eq!(
        after_second, after_first,
        "junction state must be byte-equal across idempotent re-apply"
    );

    cleanup(&pool, &cin_no, &cust_no, &room_no_a).await;
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(&room_no_b)
        .execute(&pool)
        .await
        .ok();
}

/// Dropping a room from a multi-room folio (3 → 2) MUST DELETE the
/// junction row for the dropped room. Track B2 dropped-room semantics.
#[tokio::test]
async fn apply_checkin_aggregate_removes_dropped_rooms() {
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no_a = unique_room_no();
    let room_no_b = unique_room_no();
    let room_no_c = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_a = seed_room(&pool, &room_no_a).await;
    let _room_b = seed_room(&pool, &room_no_b).await;
    let _room_c = seed_room(&pool, &room_no_c).await;

    // Initial 3-room aggregate.
    let three_room = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![
            ds_row(&cin_no, &room_no_a, "เข้าพัก"),
            ds_row(&cin_no, &room_no_b, "เข้าพัก")
                .with("id", hotel_backend::sync::row::test_support::MockValue::I32(25002)),
            ds_row(&cin_no, &room_no_c, "เข้าพัก")
                .with("id", hotel_backend::sync::row::test_support::MockValue::I32(25003)),
        ],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &three_room, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(checkin_rooms_for_folio(&pool, &cin_no).await.len(), 3);

    // iHOTEL edit removes room C from the folio. The legacy app's
    // shape is: HT_CheckIn_Ds row for room C is hard-deleted. Re-load
    // the aggregate and observe two rows only.
    let two_room = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![
            ds_row(&cin_no, &room_no_a, "เข้าพัก"),
            ds_row(&cin_no, &room_no_b, "เข้าพัก")
                .with("id", hotel_backend::sync::row::test_support::MockValue::I32(25002)),
        ],
        payments: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx2, mssql.as_ref(), &two_room, &cin_no)
        .await
        .unwrap();
    tx2.commit().await.unwrap();

    let rooms = checkin_rooms_for_folio(&pool, &cin_no).await;
    assert_eq!(
        rooms.len(),
        2,
        "dropped-room cleanup must DELETE the room-C junction row \
         (3-room → 2-room edit on legacy)"
    );
    let still_present: Vec<&str> = rooms.iter().map(|(n, _)| n.as_str()).collect();
    assert!(still_present.contains(&room_no_a.as_str()));
    assert!(still_present.contains(&room_no_b.as_str()));
    assert!(
        !still_present.contains(&room_no_c.as_str()),
        "room C must be gone from the junction"
    );

    cleanup(&pool, &cin_no, &cust_no, &room_no_a).await;
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = ANY($1)")
        .bind(&[room_no_b.clone(), room_no_c.clone()])
        .execute(&pool)
        .await
        .ok();
}

/// Thai literals (`'เข้าพัก'`) must round-trip verbatim through the
/// junction — locks the user's standing legacy-literal constraint
/// against an accidental enum translation.
#[tokio::test]
async fn apply_checkin_aggregate_preserves_thai_literals_in_junction() {
    let pool = common::create_test_pool().await;
    let mssql = mssql_stub().await;
    let cin_no = unique_cin_no();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = CheckInAggregate {
        header: Some(header_row(&cin_no, &cust_no, "ปกติ")),
        rooms: vec![ds_row(&cin_no, &room_no, "เข้าพัก")],
        payments: vec![],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_checkin_aggregate(&mut tx, mssql.as_ref(), &aggregate, &cin_no)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let status: String = sqlx::query_scalar(
        "SELECT cr.cr_room_status \
           FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1",
    )
    .bind(&cin_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        status, "เข้าพัก",
        "Thai literal must round-trip verbatim into ht_checkin_rooms.cr_room_status"
    );

    cleanup(&pool, &cin_no, &cust_no, &room_no).await;
}
