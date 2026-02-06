//! Room and room-type integration tests.
//!
//! Creates a room type (with `TEST-` prefix in `type_code`) and rooms (with
//! `TEST_` prefix in `room_notes`), then exercises read and status-update
//! operations.
//!
//! Each test uses a unique suffix in fixture data so parallel tests do not
//! interfere with each other.

mod common;

use sqlx::Row;

/// Helper: create a test room type and return its `type_id`.
/// Uses a unique `type_code` per suffix.
async fn create_test_room_type(pool: &sqlx::PgPool, suffix: &str) -> i32 {
    let code = format!("TEST-R{}", suffix);
    let row = sqlx::query(
        r#"INSERT INTO ht_room_types (type_code, type_name, type_name_en, type_base_price, type_max_guests, type_bed_type)
           VALUES ($1, 'ห้องทดสอบ', 'Test Room', 1200.00, 2, 'Double')
           ON CONFLICT (type_code) DO UPDATE SET type_name = EXCLUDED.type_name
           RETURNING type_id"#,
    )
    .bind(&code)
    .fetch_one(pool)
    .await
    .expect("INSERT room type should succeed");

    row.try_get::<i32, _>("type_id").unwrap()
}

/// Helper: create a test room and return its `room_id`.
async fn create_test_room(pool: &sqlx::PgPool, type_id: i32, room_no: &str, suffix: &str) -> i32 {
    let note = format!("TEST_room_{}", suffix);
    let row = sqlx::query(
        r#"INSERT INTO ht_rooms_new (room_no, room_type_id, room_floor, room_status, room_clean, room_maintenance, room_price_weekday, room_notes)
           VALUES ($1, $2, 1, 'available', true, false, 1200.00, $3)
           ON CONFLICT (room_no) DO UPDATE SET room_notes = EXCLUDED.room_notes
           RETURNING room_id"#,
    )
    .bind(room_no)
    .bind(type_id)
    .bind(&note)
    .fetch_one(pool)
    .await
    .expect("INSERT room should succeed");

    row.try_get::<i32, _>("room_id").unwrap()
}

/// Targeted cleanup by IDs.
async fn cleanup_room_fixture(pool: &sqlx::PgPool, room_ids: &[i32], type_id: i32) {
    for room_id in room_ids {
        sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_room_types WHERE type_id = $1")
        .bind(type_id)
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
async fn room_type_create_and_read() {
    let pool = common::create_test_pool().await;

    let type_id = create_test_room_type(&pool, "01").await;
    assert!(type_id > 0);

    // Read back
    let row = sqlx::query(
        "SELECT type_code, type_name, type_name_en, type_base_price::float8 AS base_price, \
         type_max_guests, type_active FROM ht_room_types WHERE type_id = $1",
    )
    .bind(type_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let code: String = row.try_get("type_code").unwrap();
    assert!(code.starts_with("TEST-R"));

    let name_en: String = row.try_get("type_name_en").unwrap();
    assert_eq!(name_en, "Test Room");

    let base_price: f64 = row.try_get("base_price").unwrap();
    assert!((base_price - 1200.0).abs() < 0.01);

    let max_guests: i32 = row.try_get("type_max_guests").unwrap();
    assert_eq!(max_guests, 2);

    let active: bool = row.try_get("type_active").unwrap();
    assert!(active);

    cleanup_room_fixture(&pool, &[], type_id).await;
}

#[tokio::test]
async fn room_create_read_and_update_status() {
    let pool = common::create_test_pool().await;

    let type_id = create_test_room_type(&pool, "02").await;
    let room_id = create_test_room(&pool, type_id, "T901", "02").await;
    assert!(room_id > 0);

    // ---- READ ----
    let row = sqlx::query(
        "SELECT r.room_no, r.room_status, r.room_clean, r.room_maintenance, \
         rt.type_name \
         FROM ht_rooms_new r \
         LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id \
         WHERE r.room_id = $1",
    )
    .bind(room_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let room_no: String = row.try_get("room_no").unwrap();
    assert_eq!(room_no, "T901");

    let status: String = row.try_get("room_status").unwrap();
    assert_eq!(status, "available");

    let is_clean: bool = row.try_get("room_clean").unwrap();
    assert!(is_clean);

    let is_maintenance: bool = row.try_get("room_maintenance").unwrap();
    assert!(!is_maintenance);

    // ---- UPDATE STATUS ----
    let result = sqlx::query(
        "UPDATE ht_rooms_new SET room_status = 'maintenance', updated_at = NOW() WHERE room_id = $1",
    )
    .bind(room_id)
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(result.rows_affected(), 1);

    // Verify
    let row = sqlx::query("SELECT room_status FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let status: String = row.try_get("room_status").unwrap();
    assert_eq!(status, "maintenance");

    // ---- UPDATE STATUS back to available ----
    sqlx::query(
        "UPDATE ht_rooms_new SET room_status = 'available', updated_at = NOW() WHERE room_id = $1",
    )
    .bind(room_id)
    .execute(&pool)
    .await
    .unwrap();

    cleanup_room_fixture(&pool, &[room_id], type_id).await;
}

#[tokio::test]
async fn room_unique_room_no_constraint() {
    let pool = common::create_test_pool().await;

    let type_id = create_test_room_type(&pool, "03").await;
    let room_id = create_test_room(&pool, type_id, "T999", "03").await;

    // Attempt to insert a room with the same room_no (without ON CONFLICT)
    let result = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_type_id, room_notes) \
         VALUES ('T999', $1, 'TEST_dup')",
    )
    .bind(type_id)
    .execute(&pool)
    .await;

    assert!(
        result.is_err(),
        "Inserting duplicate room_no should violate unique constraint"
    );

    cleanup_room_fixture(&pool, &[room_id], type_id).await;
}
