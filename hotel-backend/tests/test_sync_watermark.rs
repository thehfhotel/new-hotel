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
//! Resilience PR R3 (2026-05-14) adds per-table watermark round-trip
//! coverage. `read_per_table` / `advance_per_table` / `touch_per_table` /
//! `record_per_table_error` are exercised end-to-end so a refactor
//! can't silently regress the per-row UPSERT.
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

/// R3 — `read_per_table` returns the full backfilled set (migration
/// 050 seeds one row per CT-tracked table). The map MUST contain
/// `HT_Book_H` because the global stall observed 2026-05-14 was
/// caused by a wedge on exactly that table; without per-table
/// granularity, the wedge gates every other table's advance.
#[tokio::test]
async fn read_per_table_returns_backfilled_rows_including_hot_tables() {
    let pool = common::create_test_pool().await;
    let map = watermark::read_per_table(&pool)
        .await
        .expect("read_per_table against seeded legacy_ct_state_per_table must succeed");

    // Migration 050 seeds 18 tables. A long-lived dev DB might have
    // had rows manually inserted on top, so assert lower bound.
    assert!(
        map.len() >= 18,
        "expected at least 18 backfilled rows, got {}",
        map.len()
    );
    for hot_table in [
        "HT_Book_H",
        "HT_Book_Ds",
        "HT_Book_Date",
        "HT_CheckIn_H",
        "HT_Receipt_H",
    ] {
        assert!(
            map.contains_key(hot_table),
            "{hot_table} must be present in per-table watermark map"
        );
    }
}

/// R3 — `advance_per_table` round-trips through `read_per_table` and
/// is monotonic (a stale tick can't roll a sibling table's
/// watermark backward). Uses a dedicated `__test_table` row so the
/// real seeded tables stay untouched across re-runs.
#[tokio::test]
async fn advance_per_table_round_trips_and_is_monotonic() {
    let pool = common::create_test_pool().await;

    let test_table = "__test_table_per_table_advance";

    // Cleanup any leftover state from a prior failed run.
    sqlx::query("DELETE FROM legacy_ct_state_per_table WHERE table_name = $1")
        .bind(test_table)
        .execute(&pool)
        .await
        .expect("setup: clear leftover row");

    watermark::advance_per_table(&pool, test_table, 42)
        .await
        .expect("first advance_per_table must succeed (INSERT branch)");

    let map = watermark::read_per_table(&pool)
        .await
        .expect("read after first advance");
    assert_eq!(map.get(test_table).copied(), Some(42));

    // Forward advance: the per-table row UPDATEs cleanly.
    watermark::advance_per_table(&pool, test_table, 100)
        .await
        .expect("forward advance must succeed (UPDATE branch)");
    let map = watermark::read_per_table(&pool)
        .await
        .expect("read after forward advance");
    assert_eq!(map.get(test_table).copied(), Some(100));

    // Backward advance is rejected by the WHERE clause — the row
    // stays at 100 even though we asked for 50. This is the
    // monotonic guard.
    watermark::advance_per_table(&pool, test_table, 50)
        .await
        .expect("backward advance call returns Ok (UPDATE just no-ops)");
    let map = watermark::read_per_table(&pool)
        .await
        .expect("read after backward advance attempt");
    assert_eq!(
        map.get(test_table).copied(),
        Some(100),
        "backward advance must be silently rejected by the monotonic WHERE clause"
    );

    // Teardown.
    sqlx::query("DELETE FROM legacy_ct_state_per_table WHERE table_name = $1")
        .bind(test_table)
        .execute(&pool)
        .await
        .expect("teardown: remove test row");
}

/// R3 — `record_per_table_error` writes `last_error` /
/// `last_error_at` so a per-table watchdog can age stuck rows
/// without joining `legacy_sync_status`.
#[tokio::test]
async fn record_per_table_error_persists_message_and_timestamp() {
    let pool = common::create_test_pool().await;

    let test_table = "__test_table_per_table_error";
    sqlx::query("DELETE FROM legacy_ct_state_per_table WHERE table_name = $1")
        .bind(test_table)
        .execute(&pool)
        .await
        .expect("setup: clear leftover row");

    let message = "tiberius: row lock on Book_ID='R015142'";
    watermark::record_per_table_error(&pool, test_table, message)
        .await
        .expect("record_per_table_error must succeed");

    let row: (Option<String>, Option<chrono::DateTime<chrono::Utc>>) = sqlx::query_as(
        "SELECT last_error, last_error_at FROM legacy_ct_state_per_table WHERE table_name = $1",
    )
    .bind(test_table)
    .fetch_one(&pool)
    .await
    .expect("read after record_per_table_error");

    assert_eq!(row.0.as_deref(), Some(message));
    assert!(row.1.is_some(), "last_error_at must be stamped");

    // Subsequent successful advance clears the error fields so the
    // watchdog stops paging on a transient blip.
    watermark::advance_per_table(&pool, test_table, 1)
        .await
        .expect("recovery advance must succeed");
    let row: (Option<String>, Option<chrono::DateTime<chrono::Utc>>) = sqlx::query_as(
        "SELECT last_error, last_error_at FROM legacy_ct_state_per_table WHERE table_name = $1",
    )
    .bind(test_table)
    .fetch_one(&pool)
    .await
    .expect("read after recovery advance");
    assert!(
        row.0.is_none() && row.1.is_none(),
        "successful advance_per_table must clear last_error / last_error_at"
    );

    sqlx::query("DELETE FROM legacy_ct_state_per_table WHERE table_name = $1")
        .bind(test_table)
        .execute(&pool)
        .await
        .expect("teardown: remove test row");
}
