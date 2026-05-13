//! Track F4 — integration tests for `ht_rate_tiers`.
//!
//! Covers:
//! * `sync::mappers::rate_tiers::apply_rate_tier_rows` — UPSERT semantics
//!   on the composite `(room_type, cust_type)` key.
//! * `routes::new_rates::lookup_by_room_and_cust_type` — service-layer
//!   helper used by booking/check-in price resolution.
//!
//! Each test uses a unique `room_type` suffix in fixture data so parallel
//! tests do not collide on the UNIQUE `(rate_tier_room_type,
//! rate_tier_cust_type)` constraint.

mod common;

use hotel_backend::routes::new_rates::lookup_by_room_and_cust_type;
use hotel_backend::sync::mappers::rate_tiers::{apply_rate_tier_rows, RateTierRow};
use sqlx::Row;

const STANDARD_TIER: &str = "ราคาปกติ";
const CORPORATE_TIER: &str = "บริษัท";

fn fixture_row(room_type: &str, cust_type: &str, legacy_id: i32, price: f64) -> RateTierRow {
    RateTierRow {
        legacy_id,
        room_type: room_type.to_string(),
        cust_type: cust_type.to_string(),
        price,
        price_hourly: Some(price / 4.0),
        price_monthly: Some(price * 20.0),
    }
}

async fn cleanup_room_type(pool: &sqlx::PgPool, room_type: &str) {
    sqlx::query("DELETE FROM ht_rate_tiers WHERE rate_tier_room_type = $1")
        .bind(room_type)
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
async fn apply_inserts_new_rows_and_returns_upsert_count() {
    let pool = common::create_test_pool().await;
    let room_type = "TEST_F4_apply_insert";
    cleanup_room_type(&pool, room_type).await;

    let rows = vec![
        fixture_row(room_type, STANDARD_TIER, 9001, 800.0),
        fixture_row(room_type, CORPORATE_TIER, 9002, 1200.0),
    ];

    let stats = apply_rate_tier_rows(&pool, &rows)
        .await
        .expect("apply should succeed");

    assert_eq!(stats.upserted, 2);
    assert_eq!(stats.skipped, 0);

    let count = sqlx::query(
        "SELECT COUNT(*)::int AS cnt FROM ht_rate_tiers WHERE rate_tier_room_type = $1",
    )
    .bind(room_type)
    .fetch_one(&pool)
    .await
    .unwrap()
    .try_get::<i32, _>("cnt")
    .unwrap();
    assert_eq!(count, 2, "expected two rows persisted");

    cleanup_room_type(&pool, room_type).await;
}

#[tokio::test]
async fn apply_updates_existing_composite_key_in_place() {
    let pool = common::create_test_pool().await;
    let room_type = "TEST_F4_apply_update";
    cleanup_room_type(&pool, room_type).await;

    // Initial seed
    let initial = vec![fixture_row(room_type, STANDARD_TIER, 7001, 500.0)];
    apply_rate_tier_rows(&pool, &initial).await.unwrap();

    // Second pass with a different price — must UPDATE, not duplicate.
    let updated = vec![fixture_row(room_type, STANDARD_TIER, 7001, 750.0)];
    let stats = apply_rate_tier_rows(&pool, &updated).await.unwrap();
    assert_eq!(stats.upserted, 1);

    let row = sqlx::query(
        "SELECT rate_tier_price::float8 AS p FROM ht_rate_tiers \
         WHERE rate_tier_room_type = $1 AND rate_tier_cust_type = $2",
    )
    .bind(room_type)
    .bind(STANDARD_TIER)
    .fetch_one(&pool)
    .await
    .expect("seeded row should be present");
    let price: f64 = row.try_get("p").unwrap();
    assert!(
        (price - 750.0).abs() < f64::EPSILON,
        "price should reflect the latest UPSERT, got {price}"
    );

    let cnt = sqlx::query(
        "SELECT COUNT(*)::int AS c FROM ht_rate_tiers WHERE rate_tier_room_type = $1",
    )
    .bind(room_type)
    .fetch_one(&pool)
    .await
    .unwrap()
    .try_get::<i32, _>("c")
    .unwrap();
    assert_eq!(cnt, 1, "UPSERT must not duplicate the composite key");

    cleanup_room_type(&pool, room_type).await;
}

#[tokio::test]
async fn apply_skips_rows_with_blank_keys_or_invalid_price() {
    let pool = common::create_test_pool().await;
    let room_type = "TEST_F4_skip";
    cleanup_room_type(&pool, room_type).await;

    let rows = vec![
        // valid
        fixture_row(room_type, STANDARD_TIER, 1, 600.0),
        // blank room_type
        fixture_row("", STANDARD_TIER, 2, 600.0),
        // blank cust_type
        fixture_row(room_type, "   ", 3, 600.0),
        // zero price
        fixture_row(room_type, CORPORATE_TIER, 4, 0.0),
    ];

    let stats = apply_rate_tier_rows(&pool, &rows).await.unwrap();
    assert_eq!(stats.upserted, 1);
    assert_eq!(stats.skipped, 3);

    cleanup_room_type(&pool, room_type).await;
}

#[tokio::test]
async fn lookup_returns_row_for_active_composite_key() {
    let pool = common::create_test_pool().await;
    let room_type = "TEST_F4_lookup_hit";
    cleanup_room_type(&pool, room_type).await;

    let rows = vec![fixture_row(room_type, STANDARD_TIER, 5001, 900.0)];
    apply_rate_tier_rows(&pool, &rows).await.unwrap();

    let found = lookup_by_room_and_cust_type(&pool, room_type, STANDARD_TIER)
        .await
        .expect("lookup should not error");

    let tier = found.expect("expected a row for the seeded composite key");
    assert_eq!(tier.room_type, room_type);
    assert_eq!(tier.cust_type, STANDARD_TIER);
    assert!((tier.price - 900.0).abs() < f64::EPSILON);
    assert_eq!(tier.legacy_id, Some(5001));

    cleanup_room_type(&pool, room_type).await;
}

#[tokio::test]
async fn lookup_returns_none_for_missing_composite_key() {
    let pool = common::create_test_pool().await;
    let room_type = "TEST_F4_lookup_miss";
    cleanup_room_type(&pool, room_type).await;

    let found = lookup_by_room_and_cust_type(&pool, room_type, STANDARD_TIER)
        .await
        .expect("lookup should not error");
    assert!(found.is_none(), "expected None for unseeded composite key");
}

#[tokio::test]
async fn lookup_ignores_inactive_rows() {
    let pool = common::create_test_pool().await;
    let room_type = "TEST_F4_lookup_inactive";
    cleanup_room_type(&pool, room_type).await;

    let rows = vec![fixture_row(room_type, STANDARD_TIER, 4001, 550.0)];
    apply_rate_tier_rows(&pool, &rows).await.unwrap();

    // Deactivate
    sqlx::query(
        "UPDATE ht_rate_tiers SET rate_tier_active = false \
         WHERE rate_tier_room_type = $1 AND rate_tier_cust_type = $2",
    )
    .bind(room_type)
    .bind(STANDARD_TIER)
    .execute(&pool)
    .await
    .unwrap();

    let found = lookup_by_room_and_cust_type(&pool, room_type, STANDARD_TIER)
        .await
        .expect("lookup should not error");
    assert!(found.is_none(), "inactive rows must not be returned");

    cleanup_room_type(&pool, room_type).await;
}
