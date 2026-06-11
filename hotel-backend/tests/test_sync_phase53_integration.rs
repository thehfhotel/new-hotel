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

/// Process-unique residue (2026-06-11): bare `nanos % N` slices collide
/// for parallel tests in the same residue window; atomic counter + pid
/// make this unique per process and de-correlated across runs.
fn unique_residue() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32;
    nanos
        .wrapping_add(std::process::id())
        .wrapping_add(seq.wrapping_mul(7919))
}

/// Helper — generate a unique-ish Book_ID so re-runs don't clash.
fn unique_book_id() -> String {
    format!("RT{:06}", unique_residue() % 1_000_000)
}

fn unique_cust_no() -> String {
    // Slightly different bucket so we don't collide with the booking id.
    format!("CT{:06}", unique_residue() % 1_000_000 + 1)
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
    let event = apply_booking_aggregate(&mut tx, None, &aggregate, &book_id)
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
    let event = apply_booking_aggregate(&mut tx, None, &aggregate, &book_id)
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
    let event2 = apply_booking_aggregate(&mut tx2, None, &aggregate, &book_id)
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
    let _ = apply_booking_aggregate(&mut tx, None, &initial, &book_id)
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
    let event = apply_booking_aggregate(&mut tx2, None, &modified, &book_id)
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
    let _ = apply_booking_aggregate(&mut tx, None, &initial, &book_id)
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
    let event = apply_booking_aggregate(&mut tx2, None, &cancelled, &book_id)
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
    let event = apply_booking_aggregate(&mut tx, None, &aggregate, &book_id)
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
    let _ = apply_booking_aggregate(&mut tx, None, &initial, &book_id)
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
    apply_booking_aggregate(&mut tx2, None, &header_only, &book_id)
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
    if let Some(e) = apply_booking_aggregate(&mut tx0, None, &initial, &book_id)
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
    let event = apply_booking_aggregate(&mut tx, None, &modified, &book_id)
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

// =============================================================================
// 2026-06-11 — June-3 silent-drop regression suite (audit P0 #1).
//
// On 2026-06-03 a customer+booking created in iHOTEL (C22209 / R015290)
// was permanently lost: the booking apply returned Ok(None) when the
// customer FK missed, the watcher counted it `skipped` (not `errored`),
// and the watermark advanced past the CT row. These tests lock the new
// contract: an unresolvable customer FK is an ERROR (watermark hold),
// and the eager-mirror path makes the miss self-healing when MSSQL is
// reachable (covered by the shared
// `checkin::resolve_customer_via_eager_mirror_for_test` seam — the
// booking mapper routes through the exact same helper).
// =============================================================================

#[tokio::test]
async fn booking_apply_errors_when_customer_unresolvable() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no(); // intentionally NOT seeded
    let room_no = unique_room_no();

    let _room_id = seed_room(&pool, &room_no).await;

    let aggregate = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };

    let mut tx = pool.begin().await.expect("begin");
    // `mssql=None` -> eager-mirror impossible -> MUST error so the
    // watcher's per-key handler sets errored=true and HOLDS the
    // watermark. Pre-fix this returned Ok(None) ("skipped") and the
    // booking was unrecoverable after the 2-day CT retention.
    let result = apply_booking_aggregate(&mut tx, None, &aggregate, &book_id).await;
    tx.rollback().await.ok();
    let err = result.expect_err(
        "unresolvable customer FK must error (June-3 regression) — \
         Ok(None) here is the silent-drop class",
    );
    assert!(
        err.to_string().contains("customer FK unresolvable"),
        "error must name the unresolvable FK: {err}"
    );

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 0, "no row must be inserted on the error path");

    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(&room_no)
        .execute(&pool)
        .await
        .ok();
}

/// iHOTEL customer-delete cascade end-to-end (cheatsheet §3.24): the
/// cascade rewrites `Book_Cust_ID='C0000'` — a sentinel with NO real
/// `HT_Customers` row. The apply must (a) NOT idempotency-skip the
/// cust_no-only change, and (b) resolve `C0000` to the canonical
/// tombstone placeholder instead of erroring/wedging.
#[tokio::test]
async fn booking_reapply_after_c0000_cascade_repoints_to_sentinel() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();
    let room_no = unique_room_no();

    let original_cust_id = seed_customer(&pool, &cust_no).await;
    let _room_id = seed_room(&pool, &room_no).await;

    // 1. Normal apply against the real customer.
    let initial = BookingAggregate {
        header: Some(header_row(&book_id, &cust_no, "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx = pool.begin().await.unwrap();
    apply_booking_aggregate(&mut tx, None, &initial, &book_id)
        .await
        .expect("initial apply")
        .expect("fresh aggregate emits an event");
    tx.commit().await.unwrap();

    // 2. iHOTEL deletes the customer: header now carries C0000. Status
    //    and amounts are UNCHANGED — pre-fix `existing_matches` skipped
    //    this entirely.
    let cascaded = BookingAggregate {
        header: Some(header_row(&book_id, "C0000", "จอง", 890.0)),
        rooms: vec![ds_row(&book_id, &room_no, 890.0)],
        nights: vec![date_row(&book_id, &room_no)],
    };
    let mut tx2 = pool.begin().await.unwrap();
    apply_booking_aggregate(&mut tx2, None, &cascaded, &book_id)
        .await
        .expect("cascade re-apply must succeed (sentinel path, no MSSQL needed)");
    tx2.commit().await.unwrap();

    let (book_cust_id, legacy_cust_no): (i32, Option<String>) = sqlx::query_as(
        "SELECT book_cust_id, legacy_cust_no FROM ht_bookings WHERE legacy_book_id = $1",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_ne!(
        book_cust_id, original_cust_id,
        "booking must be re-pointed away from the deleted customer"
    );
    assert_eq!(
        legacy_cust_no.as_deref(),
        Some("C0000"),
        "denormalised pointer must mirror the cascade sentinel"
    );
    let sentinel_no: Option<String> = sqlx::query_scalar(
        "SELECT legacy_cust_no FROM ht_customers WHERE cust_id = $1",
    )
    .bind(book_cust_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        sentinel_no.as_deref(),
        Some("C0000"),
        "FK must land on the canonical C0000 tombstone placeholder"
    );

    cleanup(&pool, &book_id, &cust_no, &room_no).await;
    // The sentinel row is shared global state — leave it in place
    // (other tests / production rows may reference it).
}

// =============================================================================
// Book_room_type=1 idempotency (audit P2 / task 7c).
//
// Type-1 bookings ("ระบุประเภทห้อง", cheatsheet §3.3) carry room-TYPE
// codes in HT_Book_Ds.Book_Room_Type. Pre-fix each line failed the
// room_no lookup, warn-skipped, and the rooms_count mismatch re-emitted
// BookingModified on EVERY CT touch forever.
// =============================================================================

#[tokio::test]
async fn type1_booking_applies_header_only_and_reapply_is_idempotent() {
    let pool = common::create_test_pool().await;
    let book_id = unique_book_id();
    let cust_no = unique_cust_no();

    let _cust_id = seed_customer(&pool, &cust_no).await;

    let header = header_row(&book_id, &cust_no, "จอง", 890.0)
        .with("Book_room_type", MockValue::I32(1));
    // Ds line carries a room-TYPE code, not a room number.
    let aggregate = BookingAggregate {
        header: Some(header),
        rooms: vec![ds_row(&book_id, "4", 890.0)],
        nights: vec![],
    };

    let mut tx = pool.begin().await.unwrap();
    let event = apply_booking_aggregate(&mut tx, None, &aggregate, &book_id)
        .await
        .expect("type-1 apply must succeed");
    assert!(event.is_some(), "fresh aggregate emits BookingCreated");
    tx.commit().await.unwrap();

    // Zero room assignments — the type code must NOT be treated as a
    // room number.
    let rooms_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_booking_rooms WHERE br_book_id IN \
         (SELECT book_id FROM ht_bookings WHERE legacy_book_id = $1)",
    )
    .bind(&book_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(rooms_count, 0, "type-1 Ds lines must not become room rows");

    // Re-apply with the identical aggregate: MUST be the idempotent
    // skip. Pre-fix the unresolvable-line count mismatch made this
    // emit BookingModified forever.
    let mut tx2 = pool.begin().await.unwrap();
    let event2 = apply_booking_aggregate(&mut tx2, None, &aggregate, &book_id)
        .await
        .expect("re-apply must succeed");
    tx2.commit().await.unwrap();
    assert!(
        event2.is_none(),
        "identical type-1 aggregate must idempotency-skip (no \
         BookingModified re-emission loop)"
    );

    cleanup(&pool, &book_id, &cust_no, "no-room-seeded").await;
}
