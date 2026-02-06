//! Payment integration tests.
//!
//! Creates fixture data (customer, room type, room, check-in), then exercises
//! recording a payment and voiding it.
//!
//! Each test function uses a unique suffix in its fixture data to avoid
//! interference when tests run in parallel.  Cleanup targets only the specific
//! test's data via the `cin_id` returned by the fixture.

mod common;

use sqlx::Row;

/// Fixture: customer + room type + room + active check-in.
///
/// `suffix` is appended to unique fields (cin_no, room_no) to prevent
/// collisions between parallel tests.
///
/// Returns (cin_id, room_id, cust_id).
async fn create_checkin_fixture(pool: &sqlx::PgPool, suffix: &str) -> (i32, i32, i32) {
    let note = format!("TEST_pay_{}", suffix);

    // Customer
    let row = sqlx::query(
        "INSERT INTO ht_customers (cust_firstname, cust_notes) \
         VALUES ($1, $2) RETURNING cust_id",
    )
    .bind(format!("PayTest{}", suffix))
    .bind(&note)
    .fetch_one(pool)
    .await
    .expect("INSERT customer fixture");

    let cust_id: i32 = row.try_get("cust_id").unwrap();

    // Room type (shared across tests -- ON CONFLICT is safe)
    let row = sqlx::query(
        r#"INSERT INTO ht_room_types (type_code, type_name, type_base_price)
           VALUES ('TEST-PY', 'Payment Test Type', 2000.00)
           ON CONFLICT (type_code) DO UPDATE SET type_name = EXCLUDED.type_name
           RETURNING type_id"#,
    )
    .fetch_one(pool)
    .await
    .expect("INSERT room type fixture");

    let type_id: i32 = row.try_get("type_id").unwrap();

    // Room (unique per test)
    let room_no = format!("TP{}", suffix);
    let row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_type_id, room_floor, room_status, room_notes) \
         VALUES ($1, $2, 3, 'available', $3) \
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

    // Check-in (unique cin_no per test)
    let cin_no = format!("CIN-TEST-PY{}", suffix);
    let row = sqlx::query(
        "INSERT INTO ht_checkins \
         (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, \
          cin_adults, cin_status, cin_rate_per_night, cin_notes) \
         VALUES ($1, $2, $3, NOW(), '2026-03-10', 1, 'active', 2000.00, $4) \
         RETURNING cin_id",
    )
    .bind(&cin_no)
    .bind(cust_id)
    .bind(room_id)
    .bind(&note)
    .fetch_one(pool)
    .await
    .expect("INSERT check-in fixture");

    let cin_id: i32 = row.try_get("cin_id").unwrap();

    // Mark room as occupied
    sqlx::query("UPDATE ht_rooms_new SET room_status = 'occupied' WHERE room_id = $1")
        .bind(room_id)
        .execute(pool)
        .await
        .ok();

    (cin_id, room_id, cust_id)
}

/// Remove the data created by a single test, targeted by IDs.
async fn cleanup_fixture(pool: &sqlx::PgPool, cin_id: i32, room_id: i32, cust_id: i32) {
    sqlx::query("DELETE FROM ht_payments WHERE pay_cin_id = $1")
        .bind(cin_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_guest_registry WHERE guest_cin_id = $1")
        .bind(cin_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_checkins WHERE cin_id = $1")
        .bind(cin_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("UPDATE ht_rooms_new SET room_status = 'available' WHERE room_id = $1")
        .bind(room_id)
        .execute(pool)
        .await
        .ok();
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
async fn payment_record_and_list() {
    let pool = common::create_test_pool().await;

    let (cin_id, room_id, cust_id) = create_checkin_fixture(&pool, "01").await;

    // ---- RECORD PAYMENT ----
    let row = sqlx::query(
        r#"INSERT INTO ht_payments (pay_cin_id, pay_amount, pay_method, pay_reference, pay_notes, pay_created_by)
           VALUES ($1, 1500.00, 'cash', 'REF-001', 'TEST_payment_record', 'test-user')
           RETURNING pay_id"#,
    )
    .bind(cin_id)
    .fetch_one(&pool)
    .await
    .expect("INSERT payment should succeed");

    let pay_id: i32 = row.try_get("pay_id").unwrap();
    assert!(pay_id > 0);

    // ---- LIST PAYMENTS ----
    let rows = sqlx::query(
        "SELECT pay_id, pay_amount::float8 AS amount, pay_method, pay_voided \
         FROM ht_payments WHERE pay_cin_id = $1 ORDER BY pay_id",
    )
    .bind(cin_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(rows.len(), 1);

    let amount: f64 = rows[0].try_get("amount").unwrap();
    assert!((amount - 1500.0).abs() < 0.01);

    let method: String = rows[0].try_get("pay_method").unwrap();
    assert_eq!(method, "cash");

    let voided: bool = rows[0].try_get("pay_voided").unwrap();
    assert!(!voided);

    cleanup_fixture(&pool, cin_id, room_id, cust_id).await;
}

#[tokio::test]
async fn payment_void() {
    let pool = common::create_test_pool().await;

    let (cin_id, room_id, cust_id) = create_checkin_fixture(&pool, "02").await;

    // Record payment
    let row = sqlx::query(
        r#"INSERT INTO ht_payments (pay_cin_id, pay_amount, pay_method, pay_notes)
           VALUES ($1, 500.00, 'transfer', 'TEST_payment_void')
           RETURNING pay_id"#,
    )
    .bind(cin_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let pay_id: i32 = row.try_get("pay_id").unwrap();

    // ---- VOID PAYMENT ----
    let result = sqlx::query(
        "UPDATE ht_payments SET pay_voided = true, pay_voided_at = NOW() WHERE pay_id = $1",
    )
    .bind(pay_id)
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(result.rows_affected(), 1);

    // Verify voided
    let row = sqlx::query("SELECT pay_voided, pay_voided_at FROM ht_payments WHERE pay_id = $1")
        .bind(pay_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let voided: bool = row.try_get("pay_voided").unwrap();
    assert!(voided, "Payment should be voided");

    let voided_at: Option<chrono::NaiveDateTime> = row.try_get("pay_voided_at").ok();
    assert!(voided_at.is_some(), "pay_voided_at should be set");

    // ---- BALANCE CALCULATION (total - non-voided payments) ----
    let row = sqlx::query(
        "SELECT COALESCE(SUM(pay_amount), 0)::float8 AS total_paid \
         FROM ht_payments WHERE pay_cin_id = $1 AND pay_voided = false",
    )
    .bind(cin_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let total_paid: f64 = row.try_get("total_paid").unwrap();
    assert!(
        total_paid.abs() < 0.01,
        "After voiding the only payment, total paid should be 0 (got {})",
        total_paid
    );

    cleanup_fixture(&pool, cin_id, room_id, cust_id).await;
}

#[tokio::test]
async fn payment_fk_constraint() {
    let pool = common::create_test_pool().await;

    // Attempt to insert a payment for a non-existent check-in
    let result = sqlx::query(
        "INSERT INTO ht_payments (pay_cin_id, pay_amount, pay_method) \
         VALUES (999999, 100.00, 'cash')",
    )
    .execute(&pool)
    .await;

    assert!(
        result.is_err(),
        "Payment referencing non-existent check-in should violate FK constraint"
    );
}
