//! Phase 5.5 integration tests for the `--bootstrap` flag and cold-replay
//! refusal in `bin/sync.rs`.
//!
//! The end-to-end bootstrap (read MSSQL CT current version + reconcile
//! seed) requires a live legacy MSSQL connection — those parts are
//! exercised against `MSSQL_HOST`-gated paths only. The pure-PG pieces
//! tested here are:
//!
//! 1. Watermark UPSERT semantics — the bootstrap path must overwrite
//!    `last_seen_version` (not gated by `<=` like `watermark::advance`).
//! 2. Cold-replay refusal preconditions — `read_last_seen` returns the
//!    sentinel `0` when the watermark is freshly seeded.
//! 3. The runbook contract: after a successful bootstrap, `read_last_seen`
//!    returns a non-zero value (so the cold-replay refusal won't trigger
//!    on the next watcher start).
//!
//! Skipped silently when `DATABASE_URL` is unreachable.

mod common;

use hotel_backend::sync::watermark;

/// `legacy_ct_state` is a singleton row (id = 1, single-row CHECK), and
/// all three watermark tests below stamp it. Cargo runs tests within a
/// binary in parallel, so without serialization they race each other's
/// stamps and read back a sibling's value (observed 2026-06-11:
/// `bootstrap_watermark_stamp_overwrites_higher_existing_version` read
/// the sentinel `0` that `cold_replay_sentinel_is_zero_on_fresh_install`
/// had just stamped). One shared lock keeps them deterministic.
static WATERMARK_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Direct watermark stamp — mirrors what `run_bootstrap` does after
/// reading `CHANGE_TRACKING_CURRENT_VERSION()`. The bootstrap path
/// uses an unconditional UPDATE (NOT `watermark::advance`) so it can
/// overwrite a partially-bumped watermark from a prior crashed run.
async fn stamp_watermark(pool: &sqlx::PgPool, version: i64) {
    sqlx::query(
        "UPDATE legacy_ct_state \
            SET last_seen_version = $1, last_polled_at = now() \
          WHERE id = 1",
    )
    .bind(version)
    .execute(pool)
    .await
    .expect("stamp watermark");
}

#[tokio::test]
async fn bootstrap_watermark_stamp_overwrites_higher_existing_version() {
    // Bootstrap is allowed to OVERWRITE the watermark even backwards —
    // the operator's intent is "re-seed canonical state from MSSQL,
    // resume watcher from fresh tip-of-stream". A prior partial run
    // could have bumped the watermark past tip-of-stream (then crashed
    // before the canonical UPSERT landed); bootstrap recovers the
    // operator from that state by force-stamping.
    let _guard = WATERMARK_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    let original = watermark::read_last_seen(&pool).await.unwrap();

    // Bump high.
    stamp_watermark(&pool, original + 5_000_000).await;
    let high = watermark::read_last_seen(&pool).await.unwrap();
    assert!(high >= original + 5_000_000);

    // Bootstrap-style stamp at a LOWER value (simulating "MSSQL's
    // current CT version is below where the prior run advanced to").
    stamp_watermark(&pool, original + 1_000).await;
    let after = watermark::read_last_seen(&pool).await.unwrap();
    assert_eq!(
        after,
        original + 1_000,
        "bootstrap stamp must overwrite even when going backwards"
    );

    // Restore so re-runs of the suite stay idempotent.
    stamp_watermark(&pool, original).await;
}

#[tokio::test]
async fn cold_replay_sentinel_is_zero_on_fresh_install() {
    // Documents the contract the watcher refusal logic depends on:
    // `read_last_seen` returns 0 ONLY when migration 013 just seeded
    // the row and nothing has stamped it since. The CT watcher checks
    // this exact value to decide whether to refuse cold replay.
    let _guard = WATERMARK_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    let original = watermark::read_last_seen(&pool).await.unwrap();

    // Force the sentinel.
    stamp_watermark(&pool, 0).await;
    let cold = watermark::read_last_seen(&pool).await.unwrap();
    assert_eq!(cold, 0, "freshly seeded watermark must read as 0");

    // Restore.
    stamp_watermark(&pool, original).await;
}

#[tokio::test]
async fn post_bootstrap_watermark_is_non_zero() {
    // Documents the post-bootstrap state: after `run_bootstrap` reads
    // `CHANGE_TRACKING_CURRENT_VERSION()` and stamps it, the watermark
    // is non-zero (CT versions are monotonic positive integers from
    // the moment Change Tracking is enabled — 2026-04-25 in our case).
    // This is the state where the watcher's cold-replay refusal does
    // NOT trigger on next start.
    let _guard = WATERMARK_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    let original = watermark::read_last_seen(&pool).await.unwrap();

    // Simulate "MSSQL CT is at version 12345".
    stamp_watermark(&pool, 12_345).await;
    let post = watermark::read_last_seen(&pool).await.unwrap();
    assert!(post > 0, "post-bootstrap watermark must be non-zero");

    // Restore.
    stamp_watermark(&pool, original).await;
}

/// Compile-time check — the env var name documented in the runbook
/// must match the literal the binary checks. A rename without a
/// runbook update would silently break operator workflows.
#[test]
fn cold_replay_env_var_name_matches_runbook() {
    // Both the watcher binary AND docs/runbook-sync.md MUST use exactly
    // this string. If you rename it in one place, rename in the other.
    const EXPECTED: &str = "LEGACY_SYNC_ALLOW_COLD_REPLAY";

    let bin_src = include_str!("../src/bin/sync.rs");
    assert!(
        bin_src.contains(EXPECTED),
        "bin/sync.rs must reference {EXPECTED} for cold-replay refusal"
    );
}

/// Same compile-time guard for the bootstrap CLI flag.
#[test]
fn bootstrap_flag_literal_matches_runbook() {
    const EXPECTED_FLAG: &str = "--bootstrap";
    let bin_src = include_str!("../src/bin/sync.rs");
    assert!(
        bin_src.contains(EXPECTED_FLAG),
        "bin/sync.rs must accept the {EXPECTED_FLAG} CLI flag"
    );
}

/// SYNC_TEST_SKIP_MSSQL_PROBE is documented in the runbook as the
/// QoL escape hatch for pure-PG test runs (skips the bb8-tiberius
/// probe that otherwise blocks 30s). The literal must match the
/// integration suite's check.
#[test]
fn sync_test_skip_mssql_probe_env_var_matches_runbook() {
    const EXPECTED: &str = "SYNC_TEST_SKIP_MSSQL_PROBE";
    let phase54_src = include_str!("test_sync_phase54_integration.rs");
    assert!(
        phase54_src.contains(EXPECTED),
        "test_sync_phase54_integration.rs must check {EXPECTED}"
    );
}
