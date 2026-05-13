//! Schema validation tests.
//!
//! Verify that all expected tables exist in the HotelNew database and that the
//! `schema_migrations` tracking table is present and contains at least the
//! baseline row.

mod common;

use sqlx::Row;

#[tokio::test]
async fn schema_migrations_table_exists() {
    let pool = common::create_test_pool().await;

    let row = sqlx::query("SELECT COUNT(*)::int AS cnt FROM schema_migrations")
        .fetch_one(&pool)
        .await
        .expect("schema_migrations table should exist");

    let count: i32 = row.try_get("cnt").unwrap_or(0);
    assert!(count >= 1, "schema_migrations should have at least the baseline row");
}

/// List of every table the application expects.
const EXPECTED_TABLES: &[&str] = &[
    "ht_customers",
    "ht_room_types",
    "ht_rooms_new",
    "ht_bookings",
    "ht_booking_rooms",
    "ht_checkins",
    "ht_guest_registry",
    "ht_rates",
    "ht_rate_tiers",
    "ht_settings",
    "ht_booking_notes",
    "ht_inventory_categories",
    "ht_inventory_items",
    "ht_inventory_transactions",
    "ht_room_inventory",
    "ht_payments",
    "ht_maintenance_categories",
    "ht_maintenance_requests",
    "schema_migrations",
];

#[tokio::test]
async fn all_expected_tables_exist() {
    let pool = common::create_test_pool().await;

    for table in EXPECTED_TABLES {
        let sql = format!(
            "SELECT EXISTS (
                SELECT 1 FROM information_schema.tables
                WHERE table_schema = 'public' AND table_name = '{}'
            ) AS exists",
            table
        );

        let row = sqlx::query(&sql)
            .fetch_one(&pool)
            .await
            .unwrap_or_else(|e| panic!("Query failed for table '{}': {}", table, e));

        let exists: bool = row.try_get("exists").unwrap_or(false);
        assert!(exists, "Table '{}' should exist in the database", table);
    }
}

#[tokio::test]
async fn settings_table_has_defaults() {
    let pool = common::create_test_pool().await;

    let row = sqlx::query("SELECT COUNT(*)::int AS cnt FROM ht_settings")
        .fetch_one(&pool)
        .await
        .expect("ht_settings table should be queryable");

    let count: i32 = row.try_get("cnt").unwrap_or(0);
    assert!(
        count >= 6,
        "ht_settings should have at least 6 default rows (got {})",
        count
    );
}
