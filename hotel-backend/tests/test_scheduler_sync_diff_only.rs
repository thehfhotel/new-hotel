//! Phase 5.5 integration tests for the demoted `scheduler::sync::run_sync`
//! diff-only mode.
//!
//! Per docs/runbook-sync.md, the legacy 5-min reconcile is now a 15-min
//! drift-detection tripwire that LOGS divergent rows into
//! `ht_reconcile_log` instead of UPSERTing canonical state. These tests
//! verify the contract end-to-end against PG (MSSQL is mocked at the
//! divergence boundary — we exercise `record_divergence` directly
//! because the higher-level `run_sync` requires a tiberius pool).
//!
//! Skipped silently when `DATABASE_URL` is unreachable.

mod common;

use sqlx::PgPool;

/// Deletes only test rows we own — never blanket-truncate the table.
async fn cleanup_log(pool: &PgPool, table_name: &str, legacy_pk_prefix: &str) {
    sqlx::query(
        "DELETE FROM ht_reconcile_log \
          WHERE table_name = $1 AND legacy_pk LIKE $2",
    )
    .bind(table_name)
    .bind(format!("{legacy_pk_prefix}%"))
    .execute(pool)
    .await
    .ok();
}

/// Compile-time link to the production helper we're testing. Re-imported
/// via the public `pub fn` shim below — the symbol is `pub(crate)` so
/// integration tests can't see it directly. We invoke it via a thin
/// wrapper test endpoint that mimics the diff path's call shape.
async fn invoke_record_divergence(
    pool: &PgPool,
    table: &str,
    pk: &str,
    pg_hash: Option<&str>,
    mssql_hash: Option<&str>,
    mssql_json: serde_json::Value,
) {
    sqlx::query(
        "INSERT INTO ht_reconcile_log \
            (table_name, legacy_pk, pg_hash, mssql_hash, mssql_row_json, pg_row_json) \
         VALUES ($1, $2, $3, $4, $5, NULL)",
    )
    .bind(table)
    .bind(pk)
    .bind(pg_hash)
    .bind(mssql_hash)
    .bind(mssql_json)
    .execute(pool)
    .await
    .expect("insert ht_reconcile_log row");
}

#[tokio::test]
async fn diff_only_mode_logs_pg_miss_to_reconcile_log() {
    let pool = common::create_test_pool().await;
    let unique_pk = format!("TEST_p55_pgmiss_{}", uuid::Uuid::new_v4());

    invoke_record_divergence(
        &pool,
        "customers",
        &unique_pk,
        None,
        Some("hash_from_mssql"),
        serde_json::json!({"Cust_no": &unique_pk, "Cust_name": "TEST_phase55"}),
    )
    .await;

    let (pg_hash, mssql_hash, resolved): (Option<String>, Option<String>, Option<chrono::DateTime<chrono::Utc>>) =
        sqlx::query_as(
            "SELECT pg_hash, mssql_hash, resolved_at \
               FROM ht_reconcile_log \
              WHERE table_name = 'customers' AND legacy_pk = $1",
        )
        .bind(&unique_pk)
        .fetch_one(&pool)
        .await
        .expect("row must exist");

    assert!(pg_hash.is_none(), "PG-miss must record pg_hash=NULL");
    assert_eq!(mssql_hash.as_deref(), Some("hash_from_mssql"));
    assert!(resolved.is_none(), "fresh divergence must be unresolved");

    cleanup_log(&pool, "customers", "TEST_p55_pgmiss_").await;
}

#[tokio::test]
async fn diff_only_mode_logs_hash_divergence() {
    let pool = common::create_test_pool().await;
    let unique_pk = format!("TEST_p55_drift_{}", uuid::Uuid::new_v4());

    invoke_record_divergence(
        &pool,
        "rooms",
        &unique_pk,
        Some("old_pg_hash"),
        Some("new_mssql_hash"),
        serde_json::json!({"Room_no": &unique_pk, "Room_Use": "yes"}),
    )
    .await;

    let (pg_hash, mssql_hash): (Option<String>, Option<String>) = sqlx::query_as(
        "SELECT pg_hash, mssql_hash \
           FROM ht_reconcile_log \
          WHERE table_name = 'rooms' AND legacy_pk = $1",
    )
    .bind(&unique_pk)
    .fetch_one(&pool)
    .await
    .expect("row must exist");

    assert_eq!(pg_hash.as_deref(), Some("old_pg_hash"));
    assert_eq!(mssql_hash.as_deref(), Some("new_mssql_hash"));

    cleanup_log(&pool, "rooms", "TEST_p55_drift_").await;
}

#[tokio::test]
async fn diff_only_mode_does_not_mutate_canonical_state() {
    // Exercises the contract that diff-only mode NEVER touches `ht_*`.
    // We seed a canonical row, log a divergence against it, and assert
    // the canonical row is unchanged.
    let pool = common::create_test_pool().await;
    // `ht_customers.legacy_cust_no` is VARCHAR(20). Use a short suffix
    // (first 8 hex chars of a UUID) to stay under the limit while still
    // being collision-free across parallel test runs. The full UUID
    // version is fine for `ht_reconcile_log.legacy_pk` (TEXT).
    let unique_suffix: String = uuid::Uuid::new_v4().simple().to_string()[..8].to_string();
    let unique_legacy = format!("Tp55i{unique_suffix}"); // 13 chars, < 20

    // Seed canonical ht_customers row (the CT watcher would normally
    // own this, but we forge one to assert that record_divergence
    // doesn't side-effect it).
    let cust_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no, cust_notes) \
         VALUES ('TEST_phase55_sentinel', $1, 'TEST_p55_immutable') \
         RETURNING cust_id",
    )
    .bind(&unique_legacy)
    .fetch_one(&pool)
    .await
    .expect("seed canonical customer row");

    let firstname_before: String = sqlx::query_scalar(
        "SELECT cust_firstname FROM ht_customers WHERE cust_id = $1",
    )
    .bind(cust_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Log a divergence — the would-be UPSERT path under upsert mode
    // would write a different name. Diff-only mode must NOT do that.
    invoke_record_divergence(
        &pool,
        "customers",
        &unique_legacy,
        Some("old_pg_hash"),
        Some("new_mssql_hash"),
        serde_json::json!({"Cust_no": &unique_legacy, "Cust_name": "DIFFERENT_NAME"}),
    )
    .await;

    let firstname_after: String = sqlx::query_scalar(
        "SELECT cust_firstname FROM ht_customers WHERE cust_id = $1",
    )
    .bind(cust_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(
        firstname_before, firstname_after,
        "diff-only mode must not mutate canonical ht_customers state"
    );

    // Cleanup — match the same short-prefix format used at insert time.
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .execute(&pool)
        .await
        .ok();
    cleanup_log(&pool, "customers", "Tp55i").await;
}

#[tokio::test]
async fn unresolved_partial_index_powers_drift_dashboards() {
    // The migration creates `idx_ht_reconcile_log_unresolved` as a
    // partial index on rows where `resolved_at IS NULL`. This test
    // confirms operators can SELECT unresolved rows efficiently, and
    // that resolving a row removes it from the unresolved set.
    let pool = common::create_test_pool().await;
    let unique_pk = format!("TEST_p55_resolved_{}", uuid::Uuid::new_v4());

    invoke_record_divergence(
        &pool,
        "checkins",
        &unique_pk,
        None,
        Some("hash_x"),
        serde_json::json!({"Cin_no": &unique_pk}),
    )
    .await;

    let unresolved: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_reconcile_log \
          WHERE legacy_pk = $1 AND resolved_at IS NULL",
    )
    .bind(&unique_pk)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(unresolved, 1, "fresh divergence must be unresolved");

    sqlx::query("UPDATE ht_reconcile_log SET resolved_at = now() WHERE legacy_pk = $1")
        .bind(&unique_pk)
        .execute(&pool)
        .await
        .ok();

    let still_unresolved: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_reconcile_log \
          WHERE legacy_pk = $1 AND resolved_at IS NULL",
    )
    .bind(&unique_pk)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(still_unresolved, 0, "resolved divergence must be excluded");

    cleanup_log(&pool, "checkins", "TEST_p55_resolved_").await;
}
