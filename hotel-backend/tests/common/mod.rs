//! Shared test infrastructure for integration tests.
//!
//! Provides helpers for creating a database pool and cleaning up test data.
//! All test data uses a `TEST_` prefix in identifiable fields so cleanup
//! queries can target them without affecting real data.
//!
//! ## Cleanup scope
//!
//! `cleanup()` MUST only delete rows owned by the test that calls it.
//! Wildcard markers like `'TEST_%'` were the root cause of the
//! `customer_search_by_name` flake (CI run 24957295146): the lifecycle
//! test's `cleanup` ran concurrently with the search test and matched
//! the search test's `'TEST_search_by_name'` rows, so the second
//! assertion saw 0 rows instead of 2.
//!
//! Rule: cleanup queries here use exact-match markers tied to the
//! lifecycle test only. Tests that own additional markers do their
//! own targeted DELETE (see `customer_search_by_name`'s pre/post
//! cleanup in `test_customers.rs`).
//! CI also runs `cargo test -- --test-threads=1` as defense in depth
//! against shared-DB races we haven't yet enumerated; see
//! `.github/workflows/docker-build.yml` (`Run backend tests` step).

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

/// Delete rows created by the `customer_crud_lifecycle` test only.
///
/// Each query uses an EXACT-match marker (not `LIKE 'TEST_%'`) so this
/// helper never touches data owned by another concurrently-running test.
/// The queries are ordered to respect foreign-key constraints
/// (children first). `.ok()` means a missing table or zero-affected-rows
/// does not abort the remaining cleanup.
#[allow(dead_code)]
pub async fn cleanup(pool: &PgPool) {
    // The marker every row owned by `customer_crud_lifecycle` carries.
    // Wildcard `'TEST_%'` was the flake root cause — see module docs.
    const LIFECYCLE_MARKER: &str = "TEST_customer_crud";

    // --- payments (child of checkins) ---
    sqlx::query(
        "DELETE FROM ht_payments WHERE pay_cin_id IN \
         (SELECT cin_id FROM ht_checkins WHERE cin_notes = $1)",
    )
    .bind(LIFECYCLE_MARKER)
    .execute(pool)
    .await
    .ok();

    // --- guest registry (child of checkins) ---
    sqlx::query(
        "DELETE FROM ht_guest_registry WHERE guest_cin_id IN \
         (SELECT cin_id FROM ht_checkins WHERE cin_notes = $1)",
    )
    .bind(LIFECYCLE_MARKER)
    .execute(pool)
    .await
    .ok();

    // --- checkins ---
    sqlx::query("DELETE FROM ht_checkins WHERE cin_notes = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- booking rooms (child of bookings) ---
    sqlx::query(
        "DELETE FROM ht_booking_rooms WHERE br_book_id IN \
         (SELECT book_id FROM ht_bookings WHERE book_source = $1)",
    )
    .bind(LIFECYCLE_MARKER)
    .execute(pool)
    .await
    .ok();

    // --- bookings ---
    sqlx::query("DELETE FROM ht_bookings WHERE book_source = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- booking notes ---
    sqlx::query("DELETE FROM ht_booking_notes WHERE book_no = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- inventory transactions ---
    sqlx::query("DELETE FROM ht_inventory_transactions WHERE trans_notes = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- room inventory (references rooms and items) ---
    sqlx::query(
        "DELETE FROM ht_room_inventory WHERE ri_room_id IN \
         (SELECT room_id FROM ht_rooms_new WHERE room_notes = $1)",
    )
    .bind(LIFECYCLE_MARKER)
    .execute(pool)
    .await
    .ok();

    // --- inventory items ---
    sqlx::query("DELETE FROM ht_inventory_items WHERE item_code = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- inventory categories ---
    sqlx::query("DELETE FROM ht_inventory_categories WHERE cat_name = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- maintenance requests (references rooms) ---
    sqlx::query("DELETE FROM ht_maintenance_requests WHERE mreq_title = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- rates ---
    sqlx::query("DELETE FROM ht_rates WHERE rate_name = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- rooms (references room_types) ---
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_notes = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- customers ---
    sqlx::query("DELETE FROM ht_customers WHERE cust_notes = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();

    // --- room types ---
    sqlx::query("DELETE FROM ht_room_types WHERE type_code = $1")
        .bind(LIFECYCLE_MARKER)
        .execute(pool)
        .await
        .ok();
}
