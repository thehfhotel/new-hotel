//! Booking integration tests.
//!
//! Creates the required fixture data (customer, room type, room), then
//! exercises booking create, booking-room assignment, and cancellation.
//!
//! Each test uses a unique `suffix` for its fixtures so parallel tests do not
//! interfere with each other.  Cleanup targets specific IDs, not broad LIKE
//! patterns.

mod common;

use sqlx::Row;

/// Fixture: customer + room type + room.  Returns (cust_id, room_id, type_id).
///
/// `suffix` is appended to unique fields to prevent collisions.
async fn create_fixtures(pool: &sqlx::PgPool, suffix: &str) -> (i32, i32, i32) {
    let note = format!("TEST_book_{}", suffix);

    // Customer
    let row = sqlx::query(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_notes) \
         VALUES ($1, 'Customer', $2) RETURNING cust_id",
    )
    .bind(format!("BookTest{}", suffix))
    .bind(&note)
    .fetch_one(pool)
    .await
    .expect("INSERT customer fixture");

    let cust_id: i32 = row.try_get("cust_id").unwrap();

    // Room type (shared -- ON CONFLICT safe)
    let row = sqlx::query(
        r#"INSERT INTO ht_room_types (type_code, type_name, type_base_price)
           VALUES ('TEST-BK', 'Booking Test Type', 1500.00)
           ON CONFLICT (type_code) DO UPDATE SET type_name = EXCLUDED.type_name
           RETURNING type_id"#,
    )
    .fetch_one(pool)
    .await
    .expect("INSERT room type fixture");

    let type_id: i32 = row.try_get("type_id").unwrap();

    // Room (unique per test)
    let room_no = format!("TB{}", suffix);
    let row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_type_id, room_floor, room_status, room_notes) \
         VALUES ($1, $2, 2, 'available', $3) \
         ON CONFLICT (room_no) DO UPDATE SET room_notes = EXCLUDED.room_notes \
         RETURNING room_id",
    )
    .bind(&room_no)
    .bind(type_id)
    .bind(&note)
    .fetch_one(pool)
    .await
    .expect("INSERT room fixture");

    let room_id: i32 = row.try_get("room_id").unwrap();

    (cust_id, room_id, type_id)
}

/// Targeted cleanup for a single test's data.
async fn cleanup_booking_fixture(
    pool: &sqlx::PgPool,
    book_id: Option<i32>,
    room_id: i32,
    cust_id: i32,
) {
    if let Some(bid) = book_id {
        sqlx::query("DELETE FROM ht_booking_rooms WHERE br_book_id = $1")
            .bind(bid)
            .execute(pool)
            .await
            .ok();
        sqlx::query("DELETE FROM ht_bookings WHERE book_id = $1")
            .bind(bid)
            .execute(pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
async fn booking_create_and_add_rooms() {
    let pool = common::create_test_pool().await;

    let (cust_id, room_id, _type_id) = create_fixtures(&pool, "01").await;

    // ---- CREATE BOOKING ----
    let book_no = "TEST-20260207-0001";
    let row = sqlx::query(
        r#"INSERT INTO ht_bookings
           (book_no, book_cust_id, book_checkin, book_checkout, book_adults, book_children,
            book_status, book_source, book_total_amount, book_deposit_amount, book_notes)
           VALUES ($1, $2, '2026-03-01', '2026-03-03', 2, 0,
                   'confirmed', 'TEST_booking_create', 3000.00, 1000.00, 'Integration test booking')
           RETURNING book_id"#,
    )
    .bind(book_no)
    .bind(cust_id)
    .fetch_one(&pool)
    .await
    .expect("INSERT booking should succeed");

    let book_id: i32 = row.try_get("book_id").unwrap();
    assert!(book_id > 0);

    // Verify computed nights column  (checkout - checkin = 2 days)
    let row = sqlx::query(
        "SELECT book_nights, book_status FROM ht_bookings WHERE book_id = $1",
    )
    .bind(book_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let nights: i32 = row.try_get("book_nights").unwrap();
    assert_eq!(nights, 2, "Computed nights should be checkout - checkin = 2");

    let status: String = row.try_get("book_status").unwrap();
    assert_eq!(status, "confirmed");

    // ---- ADD BOOKING ROOM ----
    let result = sqlx::query(
        "INSERT INTO ht_booking_rooms (br_book_id, br_room_id, br_price_per_night) \
         VALUES ($1, $2, 1500.00)",
    )
    .bind(book_id)
    .bind(room_id)
    .execute(&pool)
    .await
    .expect("INSERT booking room should succeed");

    assert_eq!(result.rows_affected(), 1);

    // Verify booking room was linked
    let row = sqlx::query(
        "SELECT COUNT(*)::int AS cnt FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(book_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let room_count: i32 = row.try_get("cnt").unwrap();
    assert_eq!(room_count, 1);

    cleanup_booking_fixture(&pool, Some(book_id), room_id, cust_id).await;
}

#[tokio::test]
async fn booking_cancel() {
    let pool = common::create_test_pool().await;

    let (cust_id, room_id, _type_id) = create_fixtures(&pool, "02").await;

    // Create booking
    let book_no = "TEST-20260207-0002";
    let row = sqlx::query(
        r#"INSERT INTO ht_bookings
           (book_no, book_cust_id, book_checkin, book_checkout, book_status, book_source)
           VALUES ($1, $2, '2026-04-01', '2026-04-05', 'confirmed', 'TEST_booking_cancel')
           RETURNING book_id"#,
    )
    .bind(book_no)
    .bind(cust_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let book_id: i32 = row.try_get("book_id").unwrap();

    // ---- CANCEL ----
    let result = sqlx::query(
        "UPDATE ht_bookings SET book_status = 'cancelled', updated_at = NOW() \
         WHERE book_id = $1 AND book_status NOT IN ('completed', 'cancelled')",
    )
    .bind(book_id)
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(result.rows_affected(), 1);

    // Verify status
    let row = sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
        .bind(book_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let status: String = row.try_get("book_status").unwrap();
    assert_eq!(status, "cancelled");

    // ---- CANCEL AGAIN should be a no-op (0 rows affected) ----
    let result = sqlx::query(
        "UPDATE ht_bookings SET book_status = 'cancelled', updated_at = NOW() \
         WHERE book_id = $1 AND book_status NOT IN ('completed', 'cancelled')",
    )
    .bind(book_id)
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(
        result.rows_affected(),
        0,
        "Cancelling an already-cancelled booking should be a no-op"
    );

    cleanup_booking_fixture(&pool, Some(book_id), room_id, cust_id).await;
}

#[tokio::test]
async fn booking_checkout_date_constraint() {
    let pool = common::create_test_pool().await;

    let (cust_id, room_id, _type_id) = create_fixtures(&pool, "03").await;

    // Attempt to create a booking where checkout <= checkin
    let result = sqlx::query(
        r#"INSERT INTO ht_bookings
           (book_no, book_cust_id, book_checkin, book_checkout, book_source)
           VALUES ('TEST-20260207-BAD', $1, '2026-05-10', '2026-05-10', 'TEST_constraint')"#,
    )
    .bind(cust_id)
    .execute(&pool)
    .await;

    assert!(
        result.is_err(),
        "Booking with checkout == checkin should violate CHECK constraint"
    );

    cleanup_booking_fixture(&pool, None, room_id, cust_id).await;
}
