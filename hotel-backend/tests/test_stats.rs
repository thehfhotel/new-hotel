//! Stats query integration tests.
//!
//! Verifies that the dashboard stats queries execute successfully and return
//! sensible values.  These tests do not create fixtures -- they simply run the
//! same queries used by `GET /api/new/stats` and check that no errors occur.

mod common;

use sqlx::Row;

#[tokio::test]
async fn stats_room_counts_query() {
    let pool = common::create_test_pool().await;

    let row = sqlx::query(
        r#"SELECT
            COUNT(*)::int AS total_rooms,
            SUM(CASE WHEN room_status = 'available' THEN 1 ELSE 0 END)::int AS available_rooms,
            SUM(CASE WHEN room_status = 'occupied' THEN 1 ELSE 0 END)::int AS occupied_rooms,
            SUM(CASE WHEN room_status = 'maintenance' THEN 1 ELSE 0 END)::int AS maintenance_rooms,
            SUM(CASE WHEN room_status = 'cleaning' THEN 1 ELSE 0 END)::int AS cleaning_rooms
        FROM ht_rooms_new"#,
    )
    .fetch_one(&pool)
    .await
    .expect("Room stats query should succeed");

    let total: i32 = row.try_get("total_rooms").unwrap_or(0);
    let available: i32 = row.try_get("available_rooms").unwrap_or(0);
    let occupied: i32 = row.try_get("occupied_rooms").unwrap_or(0);
    let maintenance: i32 = row.try_get("maintenance_rooms").unwrap_or(0);
    let cleaning: i32 = row.try_get("cleaning_rooms").unwrap_or(0);

    // The sum of statuses should equal total (modulo any rooms with unexpected status values).
    // We only check non-negative as a sanity guard -- if the table is empty all will be 0.
    assert!(total >= 0, "total_rooms should be non-negative");
    assert!(available >= 0, "available_rooms should be non-negative");
    assert!(occupied >= 0, "occupied_rooms should be non-negative");
    assert!(maintenance >= 0, "maintenance_rooms should be non-negative");
    assert!(cleaning >= 0, "cleaning_rooms should be non-negative");
}

#[tokio::test]
async fn stats_checkin_counts_query() {
    let pool = common::create_test_pool().await;

    let row = sqlx::query(
        r#"SELECT
            SUM(CASE WHEN cin_checkin_time::date = NOW()::date THEN 1 ELSE 0 END)::int AS today_check_ins,
            SUM(CASE WHEN cin_checkout_time::date = NOW()::date THEN 1 ELSE 0 END)::int AS today_check_outs
        FROM ht_checkins"#,
    )
    .fetch_one(&pool)
    .await
    .expect("Check-in stats query should succeed");

    // These may be NULL if the table is empty -- unwrap_or(0) handles that
    let today_in: i32 = row.try_get("today_check_ins").unwrap_or(0);
    let today_out: i32 = row.try_get("today_check_outs").unwrap_or(0);

    assert!(today_in >= 0, "today_check_ins should be non-negative");
    assert!(today_out >= 0, "today_check_outs should be non-negative");
}

#[tokio::test]
async fn stats_occupancy_rate_calculation() {
    let pool = common::create_test_pool().await;

    let row = sqlx::query(
        r#"SELECT
            COUNT(*)::int AS total_rooms,
            SUM(CASE WHEN room_status = 'occupied' THEN 1 ELSE 0 END)::int AS occupied_rooms
        FROM ht_rooms_new"#,
    )
    .fetch_one(&pool)
    .await
    .expect("Occupancy query should succeed");

    let total: i32 = row.try_get("total_rooms").unwrap_or(0);
    let occupied: i32 = row.try_get("occupied_rooms").unwrap_or(0);

    let occupancy_rate = if total > 0 {
        (occupied as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    assert!(
        (0.0..=100.0).contains(&occupancy_rate),
        "Occupancy rate should be between 0 and 100 (got {})",
        occupancy_rate
    );
}
