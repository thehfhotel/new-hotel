//! Shared test infrastructure for integration tests.
//!
//! Provides helpers for creating a database pool and cleaning up test data.
//! All test data uses a `TEST_` prefix in identifiable fields so cleanup
//! queries can target them without affecting real data.

use sqlx::PgPool;

/// Create a PgPool for integration tests.
///
/// Uses the `DATABASE_URL` environment variable, falling back to the default
/// local development connection string if the variable is not set.
pub async fn create_test_pool() -> PgPool {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:NewHotel%402026%21@localhost:5439/hotelnew".to_string()
    });
    PgPool::connect(&url)
        .await
        .expect("Failed to connect to test database")
}

/// Delete all rows that were created by the test suite.
///
/// The queries are ordered to respect foreign-key constraints (children first).
/// Each statement uses `.ok()` so that a missing table or zero-affected-rows
/// does not abort the remaining cleanup.
#[allow(dead_code)]
pub async fn cleanup(pool: &PgPool) {
    // --- payments (child of checkins) ---
    sqlx::query(
        "DELETE FROM ht_payments WHERE pay_cin_id IN \
         (SELECT cin_id FROM ht_checkins WHERE cin_notes LIKE 'TEST_%')",
    )
    .execute(pool)
    .await
    .ok();

    // --- guest registry (child of checkins) ---
    sqlx::query(
        "DELETE FROM ht_guest_registry WHERE guest_cin_id IN \
         (SELECT cin_id FROM ht_checkins WHERE cin_notes LIKE 'TEST_%')",
    )
    .execute(pool)
    .await
    .ok();

    // --- checkins ---
    sqlx::query("DELETE FROM ht_checkins WHERE cin_notes LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- booking rooms (child of bookings) ---
    sqlx::query(
        "DELETE FROM ht_booking_rooms WHERE br_book_id IN \
         (SELECT book_id FROM ht_bookings WHERE book_source LIKE 'TEST_%')",
    )
    .execute(pool)
    .await
    .ok();

    // --- bookings ---
    sqlx::query("DELETE FROM ht_bookings WHERE book_source LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- booking notes ---
    sqlx::query("DELETE FROM ht_booking_notes WHERE book_no LIKE 'TEST-%'")
        .execute(pool)
        .await
        .ok();

    // --- inventory transactions ---
    sqlx::query("DELETE FROM ht_inventory_transactions WHERE trans_notes LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- room inventory (references rooms and items) ---
    sqlx::query(
        "DELETE FROM ht_room_inventory WHERE ri_room_id IN \
         (SELECT room_id FROM ht_rooms_new WHERE room_notes LIKE 'TEST_%')",
    )
    .execute(pool)
    .await
    .ok();

    // --- inventory items ---
    sqlx::query("DELETE FROM ht_inventory_items WHERE item_code LIKE 'TEST-%'")
        .execute(pool)
        .await
        .ok();

    // --- inventory categories ---
    sqlx::query("DELETE FROM ht_inventory_categories WHERE cat_name LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- maintenance requests (references rooms) ---
    sqlx::query("DELETE FROM ht_maintenance_requests WHERE mreq_title LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- rates ---
    sqlx::query("DELETE FROM ht_rates WHERE rate_name LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- rooms (references room_types) ---
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_notes LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- customers ---
    sqlx::query("DELETE FROM ht_customers WHERE cust_notes LIKE 'TEST_%'")
        .execute(pool)
        .await
        .ok();

    // --- room types ---
    sqlx::query("DELETE FROM ht_room_types WHERE type_code LIKE 'TEST-%'")
        .execute(pool)
        .await
        .ok();
}
