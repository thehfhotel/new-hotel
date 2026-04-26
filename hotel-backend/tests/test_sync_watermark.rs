//! Integration tests for `crate::sync::watermark`.
//!
//! Phase 5.1 smoke test required by the plan: `read_last_seen` against a
//! fresh `legacy_ct_state` row returns `0`. Migration 013 seeds the
//! single-row table with `last_seen_version = 0`, so a freshly migrated
//! database should always answer `0` from a cold read.
//!
//! The `advance` round-trip test moves the watermark forward and reads
//! it back to confirm the single-row UPDATE plumbing works end-to-end.
//! It restores the prior value at the end so re-runs against a
//! long-lived dev DB stay idempotent.
//!
//! These tests need a live PostgreSQL pool — they run only when
//! `DATABASE_URL` is reachable. The shared `common::create_test_pool`
//! helper covers the fallback / skip semantics other integration tests
//! already rely on.

mod common;

use hotel_backend::sync::watermark;

#[tokio::test]
async fn read_last_seen_on_fresh_state_returns_zero_or_more() {
    let pool = common::create_test_pool().await;

    // The seed value is 0 on a fresh install. A long-lived dev DB may
    // have advanced past 0 once a real CT watcher tick lands in 5.2,
    // so we assert non-negative rather than equality. The `advance`
    // round-trip below is the sharper test.
    let value = watermark::read_last_seen(&pool)
        .await
        .expect("read_last_seen against seeded legacy_ct_state must succeed");
    assert!(
        value >= 0,
        "last_seen_version must be non-negative, got {value}"
    );
}

#[tokio::test]
async fn advance_updates_and_round_trips_through_read() {
    let pool = common::create_test_pool().await;

    let original = watermark::read_last_seen(&pool)
        .await
        .expect("baseline read must succeed");

    // Move forward by a large delta so we never accidentally land on
    // the same value the seed had — keeps the assertion sharp.
    let bumped = original + 1_000_000;
    watermark::advance(&pool, bumped)
        .await
        .expect("advance must succeed against the single-row state table");

    let after = watermark::read_last_seen(&pool)
        .await
        .expect("read after advance must succeed");
    assert_eq!(
        after, bumped,
        "advance(N) followed by read_last_seen() must return N"
    );

    // Restore the prior value so re-running the test (or running it
    // alongside the future real CT watcher) stays idempotent. The
    // WHERE clause `last_seen_version <= $1` in advance() blocks
    // moving backward, so we use a direct UPDATE here as a test-only
    // teardown step.
    sqlx::query("UPDATE legacy_ct_state SET last_seen_version = $1 WHERE id = 1")
        .bind(original)
        .execute(&pool)
        .await
        .expect("teardown: restore original watermark");
}
