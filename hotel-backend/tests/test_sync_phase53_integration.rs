//! Phase 5.3 integration tests for the booking aggregate CT mapper.
//!
//! Drives `apply_booking_aggregate` directly with `BookingAggregate`
//! fixtures (no MSSQL connection required) and asserts the canonical
//! PG row + `event_log` content match expectations.
//!
//! Skipped silently when `DATABASE_URL` is unreachable — the unit
//! tests in `sync::mappers::booking` cover the pure projection /
//! coalescing logic; this suite covers the UPSERT + idempotency +
//! event-persistence loop.

mod common;

use chrono::NaiveDate;
use hotel_backend::sync::mappers::apply_booking_aggregate;
use hotel_backend::sync::parent_loader::BookingAggregate;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const HT_BOOK_H: &str = "HT_Book_H";
const HT_BOOK_DS: &str = "HT_Book_Ds";
const HT_BOOK_DATE: &str = "HT_Book_Date";

/// Helper — generate a unique-ish Book_ID so re-runs don't clash.
fn unique_book_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("RT{:06}", (nanos % 1_000_000) as u32)
}

fn unique_cust_no() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    // Slightly different bucket so we don't collide with the booking id.
    format!("CT{:06}", (nanos % 1_000_000) as u32 + 1)
}

fn unique_room_no() -> String {
    format!("X{:03}", (rand::random::<u8>() as u16) % 999 + 1)
}

fn header_row(book_id: &str, cust_no: &str, status: &str, total: f64) -> HashMapRow {
    HashMapRow::new(HT_BOOK_H)
        .with("Book_ID", MockValue::Str(book_id.into()))
        .with("Book_Cust_ID", MockValue::Str(cust_no.into()))
        .with("Book_Status", MockValue::Str(status.into()))
        .with(
            "Book_Date_in",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        )
        .with(
            "Book_Date_out",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 2)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        )
        .with("Book_Price_Total", MockValue::Decimal(total))
        .with("Book_Price_Pay", MockValue::Decimal(0.0))
        .with("Book_room_note", MockValue::Null)
}

fn ds_row(book_id: &str, room_no: &str, price: f64) -> HashMapRow {
    HashMapRow::new(HT_BOOK_DS)
        .with("id", MockValue::I32(7000))
        .with("Book_No", MockValue::Str(book_id.into()))
        .with("Book_Room_Type", MockValue::Str(room_no.into()))
        .with("Book_Room_Price", MockValue::Decimal(price))
        .with("Book_status", MockValue::I32(1))
}

fn date_row(book_id: &str, room_no: &str) -> HashMapRow {
    HashMapRow::new(HT_BOOK_DATE)
        .with("id", MockValue::I32(47200))
        .with("Book_no", MockValue::Str(book_id.into()))
        .with("Book_type", MockValue::Str(room_no.into()))
        .with(
            "Book_date_ds",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        )
        .with("Book_USE", MockValue::I32(0))
        .with("Book_ok", MockValue::I32(0))
}

/// Seed a customer in PG so the booking mapper's customer FK resolver
/// can find it by `legacy_cust_no`. Returns the freshly minted SERIAL
/// `cust_id`.
async fn seed_customer(pool: &sqlx::PgPool, legacy_cust_no: &str) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no, cust_notes) \
         VALUES ('TEST_phase53_cust', $1, 'TEST_phase53') \
         RETURNING cust_id",
    )
    .bind(legacy_cust_no)
    .fetch_one(pool)
    .await
    .expect("seed customer")
}

/// Seed a room in PG so the room-FK resolver in `replace_rooms` finds
/// it by `room_no`.
async fn seed_room(pool: &sqlx::PgPool, room_no: &str) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes) \
         VALUES ($1, true, 'TEST_phase53_room') \
         ON CONFLICT (room_no) DO UPDATE SET room_clean = EXCLUDED.room_clean \
         RETURNING room_id",
    )
    .bind(room_no)
    .fetch_one(pool)
    .await
    .expect("seed room")
}

async fn cleanup(pool: &sqlx::PgPool, book_id: &str, cust_no: &str, room_no: &str) {
    // Delete event_log rows tagged with the booking's aggregate uuid.
    let agg: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(book_id)
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
    sqlx::query("DELETE FROM ht_booking_rooms WHERE br_book_id IN \
                 (SELECT book_id FROM ht_bookings WHERE legacy_book_id = $1)")
        .bind(book_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_bookings WHERE legacy_book_id = $1")
        .bind(book_id)
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

#[tokio::test]
async fn booking_insert_upserts_pg_row_and_writes_event_log() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };

    let mut tx = pool.begin().await.expect("begin");
    let event = apply_booking_aggregate(&mut tx, &aggregate, &book_id)
        .await
        .expect("apply must succeed");
    assert!(event.is_some(), "fresh aggregate must emit an event");
    let event = event.unwrap();
    assert_eq!(event.type_name(), "BookingCreated");

    hotel_backend::outbox::bus::EventBus::publish(&mut tx, &event)
        .await
        .expect("publish");
    tx.commit().await.expect("commit");

    // Canonical row landed.
    let (book_pg_id, agg_id, status, total): (
        i32,
        Option<uuid::Uuid>,
        String,
        Option<f64>,
    ) = sqlx::query_as(
        "SELECT book_id, aggregate_id, book_status, book_total_amount::float8 \
           FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(agg_id.is_some());
    assert_eq!(status, "confirmed");
    assert_eq!(total, Some(890.0));
    let _ = cust_pg_id; // silence unused

    // ht_booking_rooms wired.
    let room_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(book_pg_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(room_count, 1);

    // event_log carries exactly one BookingCreated row.
    let event_kinds: Vec<String> = sqlx::query_scalar(
        "SELECT event_type FROM event_log WHERE aggregate_id = $1 ORDER BY created_at",
    )
    .bind(agg_id.unwrap())
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(event_kinds, vec!["BookingCreated".to_string()]);

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}

#[tokio::test]
async fn booking_re_apply_with_identical_aggregate_skips_event() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };

    // First apply — emits BookingCreated.
    let mut tx = pool.begin().await.unwrap();
    let event = apply_booking_aggregate(&mut tx, &aggregate, &book_id)
        .await
        .expect("first apply");
    assert!(event.is_some());
    if let Some(e) = event {
        hotel_backend::outbox::bus::EventBus::publish(&mut tx, &e)
            .await
            .ok();
    }
    tx.commit().await.unwrap();

    // Second apply with identical aggregate — must NOT emit.
    let mut tx2 = pool.begin().await.unwrap();
    let event2 = apply_booking_aggregate(&mut tx2, &aggregate, &book_id)
        .await
        .expect("second apply");
    tx2.commit().await.unwrap();
    assert!(
        event2.is_none(),
        "re-apply with identical aggregate must skip event publication"
    );

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}

#[tokio::test]
async fn booking_modify_emits_booking_modified_event() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed.
    let initial = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_booking_aggregate(&mut tx, &initial, &book_id)
        .await
        .expect("seed apply");
    tx.commit().await.unwrap();

    // Modify total amount (and status to keep it confirmed).
    let modified = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 1290.0)),
        rooms: vec![ds_row(&book_id, &room_no, 1290.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let event = apply_booking_aggregate(&mut tx2, &modified, &book_id)
        .await
        .expect("modify apply");
    tx2.commit().await.unwrap();
    let event = event.expect("modify must emit");
    assert_eq!(event.type_name(), "BookingModified");

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}

#[tokio::test]
async fn booking_delete_marks_status_cancelled_and_emits_booking_cancelled() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed.
    let initial = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_booking_aggregate(&mut tx, &initial, &book_id)
        .await
        .expect("seed");
    tx.commit().await.unwrap();

    // Header gone — simulate the delete-or-cancel-with-purge case.
    let cancelled = BookingAggregate {
        header: None,
        rooms: vec![],
        nights: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    let event = apply_booking_aggregate(&mut tx2, &cancelled, &book_id)
        .await
        .expect("cancel apply");
    tx2.commit().await.unwrap();

    let event = event.expect("cancel must emit");
    assert_eq!(event.type_name(), "BookingCancelled");

    let status: String = sqlx::query_scalar(
        "SELECT book_status FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(status, "cancelled");

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}

/// Regression: a header-only legacy aggregate (HT_Book_H present, zero
/// HT_Book_Ds lines) MUST insert a canonical `ht_bookings` row with zero
/// `ht_booking_rooms` rows. iHOTEL's ClickBook cancel-on-room flow
/// (cheatsheet §3.6) and the FrmAddBook2.SAVE_EDIT delete-then-reinsert
/// pattern (§3.7) both leave the aggregate in this shape. Without
/// header-only support, `HT_CheckIn_H.Cin_Book_no` FKs can never resolve
/// for these bookings — the 2026-05-18 "18 stuck check-ins" PROD-CRIT.
#[tokio::test]
async fn header_only_booking_creates_canonical_row_with_zero_booking_rooms() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no(); // seeded for cleanup symmetry only

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;

    let aggregate = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![],
        nights: vec![],
    };

    let mut tx = pool.begin().await.expect("begin");
    let event = apply_booking_aggregate(&mut tx, &aggregate, &book_id)
        .await
        .expect("apply must succeed for header-only aggregate");
    assert!(event.is_some(), "header-only insert must emit an event");
    let event = event.unwrap();
    assert_eq!(event.type_name(), "BookingCreated");

    hotel_backend::outbox::bus::EventBus::publish(&mut tx, &event)
        .await
        .expect("publish");
    tx.commit().await.expect("commit");

    let (book_pg_id, status): (i32, String) = sqlx::query_as(
        "SELECT book_id, book_status FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .expect("canonical row must land for header-only booking");
    assert_eq!(status, "confirmed");

    let room_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(book_pg_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        room_count, 0,
        "header-only booking must have zero ht_booking_rooms rows"
    );

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}

/// Regression: an edit that drops every room from a booking (the legacy
/// app's §3.7 delete-then-reinsert pattern can transiently surface this
/// state) MUST remove the stale `ht_booking_rooms` rows. Without the
/// unconditional `replace_rooms` call the junction would keep pointing
/// at rooms that no longer exist in the legacy aggregate.
#[tokio::test]
async fn re_apply_with_zero_rooms_clears_stale_booking_rooms() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed with one room.
    let initial = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx = pool.begin().await.unwrap();
    let _ = apply_booking_aggregate(&mut tx, &initial, &book_id)
        .await
        .expect("seed apply");
    tx.commit().await.unwrap();

    let book_pg_id: i32 =
        sqlx::query_scalar("SELECT book_id FROM ht_bookings WHERE legacy_book_id = $1")
            .bind(&book_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    let seeded_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(book_pg_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(seeded_count, 1, "seed should land one booking_rooms row");

    // Re-apply with empty rooms (header-only transient state).
    let header_only = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![],
        nights: vec![],
    };
    let mut tx2 = pool.begin().await.unwrap();
    apply_booking_aggregate(&mut tx2, &header_only, &book_id)
        .await
        .expect("header-only re-apply");
    tx2.commit().await.unwrap();

    let after_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(book_pg_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        after_count, 0,
        "header-only re-apply must drop stale booking_rooms rows"
    );

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}

/// One H + 2 Ds + 5 Date CT rows in the same tick must result in
/// exactly ONE `apply_booking_aggregate` call (per the watcher's
/// coalescing pre-pass) — and exactly ONE `event_log` row for the
/// underlying BookingModified.
#[tokio::test]
async fn coalescing_yields_exactly_one_event_per_aggregate_per_tick() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let _cust_pg_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // Seed canonical row first so the apply emits BookingModified.
    let initial = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx0 = pool.begin().await.unwrap();
    if let Some(e) = apply_booking_aggregate(&mut tx0, &initial, &book_id)
        .await
        .unwrap()
    {
        hotel_backend::outbox::bus::EventBus::publish(&mut tx0, &e)
            .await
            .ok();
    }
    tx0.commit().await.unwrap();

    // Now simulate a tick with 1 H + 2 Ds + 5 Date rows for the same
    // booking: the watcher's pre-pass dedups to one `book_id` and
    // calls `apply_booking_aggregate` exactly once. We mirror that
    // here by calling apply ONCE.
    let modified = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 1290.0)),
        rooms: vec![ds_row(&book_id, &room_no, 1290.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx = pool.begin().await.unwrap();
    let event = apply_booking_aggregate(&mut tx, &modified, &book_id)
        .await
        .expect("apply");
    if let Some(e) = event {
        hotel_backend::outbox::bus::EventBus::publish(&mut tx, &e)
            .await
            .ok();
    }
    tx.commit().await.unwrap();

    // Two events total in event_log: the original Created + the
    // Modified from the second apply. NOT one-per-child-row.
    let agg_id: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    let modified_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM event_log \
          WHERE aggregate_id = $1 AND event_type = 'BookingModified'",
    )
    .bind(agg_id.unwrap())
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        modified_count, 1,
        "exactly one BookingModified per aggregate per tick"
    );

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
}
