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
use hotel_backend::sync::mappers::apply_checkin_aggregate;
use hotel_backend::sync::parent_loader::CheckInAggregate;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const HT_CHECKIN_H: &str = "HT_CheckIn_H";
const HT_CHECKIN_DS: &str = "HT_CheckIn_Ds";

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
        .with("Cin_Room_Out", MockValue::Null)
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
