//! One-shot backfill: legacy MSSQL `HT_Receipt_H` -> canonical PG
//! `ht_payments` (issue #278 — the receipt-header mirror gap).
//!
//! ## Why this exists
//!
//! The HF Hotel `payments` reconcile arm (2026-07-31, `c4817d7`) surfaced
//! exactly 13 `missing_pg` receipts across 9 folios, all with
//! `ht_checkins.created_at` in a 35-hour window on 2026-05-15 — the ~19k
//! pre-bootstrap check-ins closed by `bin/backfill_legacy_checkins.rs`.
//!
//! Root cause: `sync/mappers/payment.rs::apply_receipt_upsert` requires the
//! parent check-in to resolve; when it can't it returns `Err`, correctly
//! holding the CT watermark for a loud retry (invariant #5). But that retry
//! is bounded by MSSQL Change Tracking's 2-day retention on `HT_Receipt_H`,
//! and the checkins-side backfill took up to 19 days to close the parent
//! gap — so the CT deltas for these 13 receipts aged out long before their
//! parent ever existed canonically. Nothing re-drives `HT_Receipt_H` on its
//! own (`apply_checkin_aggregate` touches only `ht_checkins` /
//! `ht_checkin_rooms`), so the gap is permanent without this bin.
//!
//! **The money is NOT missing** — every one of these receipts has a
//! matching `ht_payment_ledger` line (mirrored independently from
//! `HT_CheckIn_Pay`) and `ht_checkins.cin_paid_amount` reconciles exactly.
//! This is an audit-trail completeness gap in the receipt-header mirror,
//! not a cash shortfall. See the issue for the full triage.
//!
//! ## Legacy access is READ-ONLY — no dark flag needed
//!
//! This bin NEVER writes to the legacy MSSQL databases. It only issues a
//! single `SELECT` against `HT_Receipt_H` per candidate receipt (routed
//! through `simple_query_with_timeout_pooled`, `MssqlOpKind::Read` — see
//! `db::mssql_timeout`). Every write lands in canonical PostgreSQL only.
//! That is the whole reason this closes without a `_ENABLED` env flag or
//! reception-coordinated live verification: unlike every writeback path in
//! this repo, there is no shared-DB blast radius.
//!
//! ## What it does
//!
//! 1. Work-list: `DISTINCT legacy_pk` from `ht_reconcile_log` where
//!    `table_name = 'payments' AND divergence_kind = 'missing_pg' AND
//!     resolved_at IS NULL` ([`WORKLIST_SQL`]) — OR, if positional CLI
//!    args are given, exactly those `Receipt_no` values instead (re-run a
//!    subset; the explicit list REPLACES the reconcile-log query, it does
//!    not filter it).
//! 2. For each `Receipt_no`: re-fetch the current `HT_Receipt_H` row from
//!    legacy, then re-drive
//!    [`sync::mappers::payment::apply_receipt_upsert`] — the SAME code
//!    path the live CT watcher uses. This bin never hand-writes a bespoke
//!    `INSERT` against `ht_payments`; that divergence-from-the-mapper is
//!    the exact bug class this repo keeps hitting (see the void-monotonicity
//!    history in `sync/mappers/payment.rs`).
//! 3. **Idempotent.** Before writing, the bin checks whether `ht_payments`
//!    already carries a row for `(pay_cin_id, receipt_no)` — the SAME
//!    predicate `apply_receipt_upsert` itself resolves on
//!    (`legacy_receipt_no = $2 OR pay_reference = $2`). An already-present
//!    receipt is reported and skipped — a true no-op, not a second UPDATE
//!    and not a duplicate row. Safe to run twice, or after the reconcile
//!    tick has already converged some of the rows.
//! 4. `--dry-run` reports exactly what WOULD be written (receipt no,
//!    amount, parent `Cin_no`, resolved canonical check-in id) and writes
//!    NOTHING — no PG transaction is committed. **Always run `--dry-run`
//!    first**; it is the safe, obvious way to preview this bin.
//! 5. The parent-FK case is handled explicitly: the bin resolves the
//!    parent check-in itself (`sync::resolve::resolve_checkin_id`) before
//!    ever calling the mapper. Per the issue triage all 9 parent folios
//!    now exist canonically, so this should always succeed — but if a
//!    parent is still unresolvable, that receipt is reported clearly and
//!    skipped (`skipped_unresolvable_parent`), not treated as a run
//!    failure and not silently dropped.
//! 6. **This bin does NOT mark `ht_reconcile_log` rows resolved.** The
//!    reconcile tick re-hashes both legacy and canonical on its own
//!    schedule and closes converged rows itself — that is the designed
//!    self-verifying path (it proves the write actually took), and this
//!    bin deliberately does not race or duplicate it. After a successful
//!    run, expect the `payments` `missing_pg` backlog to clear on the next
//!    reconcile tick, not immediately.
//! 7. Domain events (`PaymentReceived`) returned by the mapper are
//!    intentionally NOT published — matches the precedent in
//!    `bin/backfill_legacy_checkins.rs`: a backfilled historical receipt
//!    is not a new event from a domain perspective.
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//!
//! # ALWAYS start here — reports what would be written, writes nothing.
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_receipt_payments -- --dry-run
//!
//! # Live run — drains every unresolved `missing_pg` payments row.
//! cargo run --release --bin backfill_receipt_payments
//!
//! # Re-run an explicit subset (e.g. to re-verify after a partial run).
//! cargo run --release --bin backfill_receipt_payments -- --dry-run \
//!   B2604-0283 B2604-0290 B2605-0008
//! ```
//!
//! ## Flags
//!
//! * `--dry-run` — resolve everything (legacy fetch, parent check-in,
//!   already-present probe) but commit no PG transaction. Reports what
//!   WOULD be written.
//! * positional args — explicit `Receipt_no` values. When given, these
//!   REPLACE the `ht_reconcile_log` work-list.

use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use hotel_backend::config::{DbConfig, SiteConfig};
use hotel_backend::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::mappers::payment::apply_receipt_upsert;
use hotel_backend::sync::resolve;
use hotel_backend::sync::row::MappableRow;
use hotel_backend::sync::SyncError;

const PG_POOL_MAX: u32 = 4;
const MSSQL_POOL_MAX: u32 = 4;

/// Legacy projection for the one-shot re-fetch by `Receipt_no`. Mirrors
/// `sync::mappers::payment::RECEIPT_SELECT_COLS` field-for-field, but kept
/// as an independent copy rather than imported: that const is table-alias
/// qualified (`t.id, t.Receipt_no, …`) for the CT JOIN and private to the
/// mapper module — same independence idiom `sync::parent_loader` uses for
/// its own local `sql_quote_inline` rather than reaching into `writeback`.
const RECEIPT_COLS: &str =
    "id, Receipt_no, Receipt_Date, Receipt_Total, Receipt_ref, Receipt_c_no, status_name";

/// Work-list SQL — issue #278 point 1. `payments` is deliberately outside
/// both self-heal lists (`REINGEST_MISSING_PG_TABLES` /
/// `should_auto_resolve`, see `scheduler::sync::PAYMENTS_ERA_FLOOR_SQL`
/// doc), so `missing_pg` rows here sit until an operator acts — this bin
/// is that action. `value` divergence for `payments` (e.g. a legacy void
/// not yet mirrored) is a different remediation path and is NOT drained
/// by this bin.
const WORKLIST_SQL: &str = "SELECT DISTINCT legacy_pk FROM ht_reconcile_log \
     WHERE table_name = 'payments' AND divergence_kind = 'missing_pg' \
       AND resolved_at IS NULL \
     ORDER BY legacy_pk";

#[derive(Debug, Default)]
struct Summary {
    attempted: usize,
    written: usize,
    skipped_already_present: usize,
    skipped_unresolvable_parent: usize,
    errored: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "backfill_receipt_payments=info,hotel_backend=info".into()
            }),
        )
        .init();

    let (dry_run, explicit) = parse_args(env::args().skip(1));
    let site = SiteConfig::from_env();

    tracing::info!(
        site = %site.id, dry_run, explicit_count = explicit.len(),
        "issue #278 — backfilling ht_payments from legacy HT_Receipt_H"
    );

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let receipt_nos = if explicit.is_empty() {
        fetch_candidate_receipt_nos(&pg).await?
    } else {
        explicit
    };
    tracing::info!(
        site = %site.id, candidates = receipt_nos.len(),
        "fetched candidate receipt numbers"
    );

    let mut s = Summary::default();
    for receipt_no in &receipt_nos {
        s.attempted += 1;
        process_receipt(&pg, &mssql, receipt_no, dry_run, &mut s).await;
    }

    println!();
    println!("=== backfill_receipt_payments — issue #278 ===");
    println!("Site:                          {}", site.id);
    println!(
        "Mode:                          {}",
        if dry_run { "DRY RUN (writes nothing)" } else { "LIVE" }
    );
    println!("Attempted:                     {}", s.attempted);
    println!(
        "{:<31}{}",
        if dry_run { "Would write:" } else { "Written:" },
        s.written
    );
    println!("Skipped (already present):     {}", s.skipped_already_present);
    println!("Skipped (unresolvable parent): {}", s.skipped_unresolvable_parent);
    println!("Errored:                       {}", s.errored);
    if !dry_run && s.written > 0 {
        println!();
        println!(
            "Note: ht_reconcile_log rows are NOT marked resolved by this bin. \
             The next reconcile tick re-hashes both sides and closes converged \
             rows itself — that is the designed, self-verifying path."
        );
    }

    tracing::info!(
        site = %site.id,
        dry_run,
        attempted = s.attempted,
        written = s.written,
        skipped_already_present = s.skipped_already_present,
        skipped_unresolvable_parent = s.skipped_unresolvable_parent,
        errored = s.errored,
        "backfill_receipt_payments — done"
    );

    Ok(())
}

/// Parse CLI args into `(dry_run, explicit_receipt_nos)`. Pure — no I/O —
/// so it is unit-testable without touching `env::args()`. Positional args
/// (anything not starting with `--`) are explicit `Receipt_no` values to
/// re-run a subset (module docs, point 1); when given they REPLACE the
/// `ht_reconcile_log` work-list rather than filtering it. Unrecognised
/// `--flags` are ignored, matching the permissive parsing style of the
/// other backfill bins.
fn parse_args<I: IntoIterator<Item = String>>(args: I) -> (bool, Vec<String>) {
    let mut dry_run = false;
    let mut explicit = Vec::new();
    for a in args {
        if a == "--dry-run" {
            dry_run = true;
        } else if !a.starts_with("--") {
            explicit.push(a);
        }
    }
    (dry_run, explicit)
}

/// Minimal legacy-row preview: just the two fields this bin needs before
/// deciding whether to call [`apply_receipt_upsert`] (which does the real,
/// authoritative projection — `payment::project_receipt` stays private per
/// the "don't restructure the mapper" guardrail). Takes `&dyn MappableRow`
/// rather than `&tiberius::Row` specifically so it is unit-testable against
/// `sync::row::test_support::HashMapRow` — `tiberius::Row`'s constructors
/// are private (see `sync/row.rs` module docs).
struct ReceiptPreview {
    legacy_cin_no: Option<String>,
    receipt_total: f64,
}

fn preview_receipt(row: &dyn MappableRow) -> Result<ReceiptPreview, SyncError> {
    let legacy_cin_no = row
        .try_get_str("Receipt_ref")?
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let receipt_total = row.try_get_decimal("Receipt_Total")?.unwrap_or(0.0);
    Ok(ReceiptPreview { legacy_cin_no, receipt_total })
}

/// SQL-quote a value for inline interpolation (single-quote doubling).
/// Local copy of the same idiom used by `sync::parent_loader`'s
/// `sql_quote_inline` and `writeback::format::sql_quote` — kept
/// independent for the same reason `parent_loader` gives: "we cannot
/// reach for a parameterised tiberius query without losing the streaming
/// shape that `simple_query` provides."
fn sql_quote_inline(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Re-fetch one `HT_Receipt_H` row by `Receipt_no`. READ-ONLY — the only
/// MSSQL call site in this bin, routed through
/// `simple_query_with_timeout_pooled` (`MssqlOpKind::Read`) per the
/// standing guardrail (issue #275 / #274): a raw, unwrapped tiberius query
/// gets neither the per-op timeout nor the poison-on-timeout flag.
async fn fetch_legacy_receipt(
    mssql: &DbPool,
    receipt_no: &str,
) -> Result<Option<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    let sql = format!(
        "SELECT {RECEIPT_COLS} FROM HT_Receipt_H WHERE Receipt_no = {}",
        sql_quote_inline(receipt_no)
    );
    let mut rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;
    if rows.len() > 1 {
        // `Receipt_no` is unique in the legacy app (per the mapper's own
        // reconcile-hash contract docs) — defensive only.
        tracing::warn!(
            receipt_no,
            count = rows.len(),
            "multiple HT_Receipt_H rows matched Receipt_no — using the first"
        );
    }
    Ok(if rows.is_empty() { None } else { Some(rows.remove(0)) })
}

/// Distinct `Receipt_no` values the payments reconcile arm has flagged
/// `missing_pg` and not yet resolved. READ-ONLY against canonical PG.
async fn fetch_candidate_receipt_nos(pg: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(WORKLIST_SQL).fetch_all(pg).await
}

/// Process one receipt end-to-end: fetch, preview, resolve parent,
/// idempotency-check, then either report (dry-run) or write (live).
/// Every branch updates `s` and prints a one-line report so a human
/// scanning stdout can follow along without grepping tracing output.
async fn process_receipt(
    pg: &PgPool,
    mssql: &DbPool,
    receipt_no: &str,
    dry_run: bool,
    s: &mut Summary,
) {
    let row = match fetch_legacy_receipt(mssql, receipt_no).await {
        Ok(Some(r)) => r,
        Ok(None) => {
            tracing::warn!(
                receipt_no,
                "HT_Receipt_H row not found in legacy for this Receipt_no — \
                 row may have vanished since the reconcile scan; skipping"
            );
            println!("[ERROR] receipt_no={receipt_no}: not found in legacy HT_Receipt_H");
            s.errored += 1;
            return;
        }
        Err(err) => {
            tracing::warn!(receipt_no, error = %err, "legacy fetch failed — skipping");
            println!("[ERROR] receipt_no={receipt_no}: legacy fetch failed: {err}");
            s.errored += 1;
            return;
        }
    };

    let preview = match preview_receipt(&row) {
        Ok(p) => p,
        Err(err) => {
            tracing::warn!(receipt_no, error = %err, "failed to preview legacy row — skipping");
            println!("[ERROR] receipt_no={receipt_no}: preview failed: {err}");
            s.errored += 1;
            return;
        }
    };

    // Should not happen in practice — `PAYMENTS_RECONCILE_SCAN_FILTER`
    // already excludes ref-less receipts from ever landing in
    // `ht_reconcile_log`, and `apply_receipt_upsert` itself treats this as
    // a deliberate skip, not an error. Defensive only (also covers the
    // explicit-CLI-args path, which bypasses that filter).
    let Some(legacy_cin_no) = preview.legacy_cin_no else {
        tracing::info!(
            receipt_no,
            "receipt carries no Receipt_ref — out of mapper scope (sale without \
             a check-in); skipping"
        );
        println!("[SKIP] receipt_no={receipt_no}: no Receipt_ref (out of scope)");
        s.skipped_unresolvable_parent += 1;
        return;
    };

    let mut tx = match pg.begin().await {
        Ok(t) => t,
        Err(err) => {
            tracing::warn!(receipt_no, error = %err, "pg.begin failed — skipping");
            println!("[ERROR] receipt_no={receipt_no}: pg.begin failed: {err}");
            s.errored += 1;
            return;
        }
    };

    let cin_id = match resolve::resolve_checkin_id(&mut tx, Some(legacy_cin_no.as_str())).await {
        Ok(Some((cid, _agg))) => cid,
        Ok(None) => {
            let _ = tx.rollback().await;
            tracing::warn!(
                receipt_no,
                %legacy_cin_no,
                "parent check-in unresolvable in canonical ht_checkins — skipping. \
                 Per issue #278 all known parent folios now exist canonically; if \
                 this fires, the parent check-in itself still needs backfilling \
                 (bin/backfill_legacy_checkins) before this receipt can land."
            );
            println!(
                "[SKIP] receipt_no={receipt_no}: parent check-in {legacy_cin_no} \
                 unresolvable in canonical ht_checkins"
            );
            s.skipped_unresolvable_parent += 1;
            return;
        }
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(receipt_no, error = %err, "resolve_checkin_id failed — skipping");
            println!("[ERROR] receipt_no={receipt_no}: resolve_checkin_id failed: {err}");
            s.errored += 1;
            return;
        }
    };

    // Idempotency pre-check — the SAME identity predicate
    // `apply_receipt_upsert` resolves on. A hit means the receipt is
    // already mirrored: true no-op, we never call the mapper at all.
    let existing_pay_id: Result<Option<i32>, sqlx::Error> = sqlx::query_scalar(
        "SELECT pay_id FROM ht_payments \
          WHERE pay_cin_id = $1 AND (legacy_receipt_no = $2 OR pay_reference = $2) \
          LIMIT 1",
    )
    .bind(cin_id)
    .bind(receipt_no)
    .fetch_optional(&mut *tx)
    .await;

    let existing_pay_id = match existing_pay_id {
        Ok(v) => v,
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(receipt_no, error = %err, "existing-row probe failed — skipping");
            println!("[ERROR] receipt_no={receipt_no}: existing-row probe failed: {err}");
            s.errored += 1;
            return;
        }
    };

    if let Some(pay_id) = existing_pay_id {
        let _ = tx.rollback().await;
        tracing::info!(receipt_no, pay_id, "already present in ht_payments — no-op");
        println!("[SKIP] receipt_no={receipt_no}: already present (pay_id={pay_id}) — no-op");
        s.skipped_already_present += 1;
        return;
    }

    if dry_run {
        let _ = tx.rollback().await;
        println!(
            "[DRY RUN] would write receipt_no={receipt_no} amount={:.2} \
             parent_cin_no={legacy_cin_no} canonical_checkin_id={cin_id}",
            preview.receipt_total
        );
        s.written += 1;
        return;
    }

    match apply_receipt_upsert(&mut tx, &row).await {
        Ok(_event) => match tx.commit().await {
            Ok(()) => {
                tracing::info!(
                    receipt_no,
                    amount = preview.receipt_total,
                    parent_cin_no = %legacy_cin_no,
                    canonical_checkin_id = cin_id,
                    "written to ht_payments"
                );
                println!(
                    "[WRITTEN] receipt_no={receipt_no} amount={:.2} \
                     parent_cin_no={legacy_cin_no} canonical_checkin_id={cin_id}",
                    preview.receipt_total
                );
                s.written += 1;
            }
            Err(err) => {
                tracing::warn!(receipt_no, error = %err, "commit failed");
                println!("[ERROR] receipt_no={receipt_no}: commit failed: {err}");
                s.errored += 1;
            }
        },
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(receipt_no, error = %err, "apply_receipt_upsert failed");
            println!("[ERROR] receipt_no={receipt_no}: apply_receipt_upsert failed: {err}");
            s.errored += 1;
        }
    }
}

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new().max_connections(PG_POOL_MAX).connect(&url).await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Connected to PostgreSQL");
    Ok(pool)
}

async fn connect_legacy() -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    let mut config = DbConfig::from_env();
    config.pool_max = MSSQL_POOL_MAX;
    let server = config.server.clone();
    let pool = create_pool(&config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    {
        // Even the startup liveness probe routes through the timeout+poison
        // wrapper — "ALL legacy MSSQL reads" per the standing guardrail,
        // no carve-out for a one-off SELECT 1.
        let mut conn = pool.get().await?;
        let _ = simple_query_with_timeout_pooled(&mut conn, "SELECT 1", MssqlOpKind::Read).await?;
    }
    tracing::info!(server = %server, port = config.port, "Connected to legacy MSSQL (read-only)");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    //! Pure-logic + string-content unit tests need no database. The one
    //! DB-backed test (`worklist_sql_selects_only_unresolved_missing_pg_payments`)
    //! self-skips without `DATABASE_URL` — `cargo test --lib`-equivalent
    //! bin tests must stay green on a machine with no database. Mirrors
    //! the `void_probe_conn` idiom in `sync::mappers::payment`'s own test
    //! module.

    use super::*;
    use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

    // -------------------------------------------------------------------
    // Source-scan pin — issue #275 / #274. This bin's only MSSQL reads
    // (`fetch_legacy_receipt`, the `connect_legacy` liveness probe) must
    // route through `simple_query_with_timeout_pooled`, never a raw
    // unwrapped tiberius call. `scheduler/mod.rs`'s equivalent pin
    // deliberately excludes `bin/*` (operator-invoked, no scheduled
    // exposure) — this is this bin's own guard so a future edit can't
    // silently reintroduce the raw call.
    //
    // Scanned region is PRODUCTION code only (everything before this
    // `#[cfg(test)]` module) — `include_str!` reads this whole file,
    // itself included, so scanning past this point would trip on the
    // needle text these very assertions must contain to name the pattern
    // they're checking for.
    // -------------------------------------------------------------------
    fn production_source() -> &'static str {
        let src = include_str!("backfill_receipt_payments.rs");
        let boundary = src.find("#[cfg(test)]").expect("test module marker must exist");
        &src[..boundary]
    }

    #[test]
    fn bin_contains_no_raw_simple_query_calls() {
        // Built from fragments so neither this line nor the panic message
        // below reproduces the exact needle contiguously — seeing it here
        // is not itself a violation.
        let needle = format!("{}{}", ".simple_", "query(");
        assert!(
            !production_source().contains(&needle),
            "backfill_receipt_payments.rs calls the raw tiberius query method \
             directly — route it through simple_query_with_timeout_pooled \
             (db::mssql_timeout) instead (issue #275 / #274)"
        );
    }

    #[test]
    fn bin_imports_the_timeout_wrapper() {
        assert!(
            production_source().contains("simple_query_with_timeout_pooled"),
            "backfill_receipt_payments.rs no longer uses \
             simple_query_with_timeout_pooled — if it stopped reading MSSQL \
             entirely this test should be removed, not left stale"
        );
    }

    // -------------------------------------------------------------------
    // parse_args
    // -------------------------------------------------------------------
    #[test]
    fn parse_args_recognises_dry_run_flag() {
        let (dry_run, explicit) = parse_args(vec!["--dry-run".to_string()]);
        assert!(dry_run);
        assert!(explicit.is_empty());
    }

    #[test]
    fn parse_args_collects_positional_receipt_numbers() {
        let (dry_run, explicit) =
            parse_args(vec!["B2604-0283".to_string(), "B2605-0008".to_string()]);
        assert!(!dry_run);
        assert_eq!(explicit, vec!["B2604-0283", "B2605-0008"]);
    }

    #[test]
    fn parse_args_mixes_flags_and_positional_args() {
        let (dry_run, explicit) =
            parse_args(vec!["--dry-run".to_string(), "B2604-0283".to_string()]);
        assert!(dry_run);
        assert_eq!(explicit, vec!["B2604-0283"]);
    }

    #[test]
    fn parse_args_ignores_unknown_flags() {
        let (dry_run, explicit) = parse_args(vec!["--bogus-flag".to_string()]);
        assert!(!dry_run);
        assert!(explicit.is_empty());
    }

    #[test]
    fn parse_args_empty_input_defaults_live_and_empty() {
        let (dry_run, explicit) = parse_args(Vec::<String>::new());
        assert!(!dry_run);
        assert!(explicit.is_empty());
    }

    // -------------------------------------------------------------------
    // sql_quote_inline
    // -------------------------------------------------------------------
    #[test]
    fn sql_quote_inline_wraps_plain_values() {
        assert_eq!(sql_quote_inline("B2604-0283"), "'B2604-0283'");
    }

    #[test]
    fn sql_quote_inline_doubles_embedded_single_quotes() {
        assert_eq!(sql_quote_inline("O'Brien"), "'O''Brien'");
    }

    #[test]
    fn sql_quote_inline_handles_empty_string() {
        assert_eq!(sql_quote_inline(""), "''");
    }

    // -------------------------------------------------------------------
    // preview_receipt — pure projection, testable via HashMapRow since
    // `tiberius::Row`'s constructors are private (sync/row.rs docs).
    // -------------------------------------------------------------------
    #[test]
    fn preview_receipt_extracts_ref_and_total() {
        let row = HashMapRow::new("HT_Receipt_H")
            .with("Receipt_ref", MockValue::Str("CH26-005258".into()))
            .with("Receipt_Total", MockValue::Decimal(1980.0));
        let p = preview_receipt(&row).expect("preview must succeed");
        assert_eq!(p.legacy_cin_no.as_deref(), Some("CH26-005258"));
        assert_eq!(p.receipt_total, 1980.0);
    }

    #[test]
    fn preview_receipt_treats_empty_ref_as_none() {
        // Mirrors `payment::project_receipt`'s own `.filter(|s|
        // !s.is_empty())` — an empty Receipt_ref means "no check-in",
        // same as a NULL one, and must not be mistaken for a real key.
        let row = HashMapRow::new("HT_Receipt_H")
            .with("Receipt_ref", MockValue::Str(String::new()))
            .with("Receipt_Total", MockValue::Decimal(400.0));
        let p = preview_receipt(&row).expect("preview must succeed");
        assert_eq!(p.legacy_cin_no, None);
    }

    #[test]
    fn preview_receipt_treats_null_ref_as_none() {
        let row = HashMapRow::new("HT_Receipt_H")
            .with("Receipt_ref", MockValue::Null)
            .with("Receipt_Total", MockValue::Decimal(400.0));
        let p = preview_receipt(&row).expect("preview must succeed");
        assert_eq!(p.legacy_cin_no, None);
    }

    #[test]
    fn preview_receipt_defaults_missing_total_to_zero() {
        let row = HashMapRow::new("HT_Receipt_H")
            .with("Receipt_ref", MockValue::Str("CH26-005258".into()))
            .with("Receipt_Total", MockValue::Null);
        let p = preview_receipt(&row).expect("preview must succeed");
        assert_eq!(p.receipt_total, 0.0);
    }

    // -------------------------------------------------------------------
    // WORKLIST_SQL — shape guard (no database needed).
    // -------------------------------------------------------------------
    #[test]
    fn worklist_sql_selects_unresolved_missing_pg_payments() {
        assert!(WORKLIST_SQL.contains("DISTINCT legacy_pk"));
        assert!(WORKLIST_SQL.contains("table_name = 'payments'"));
        assert!(WORKLIST_SQL.contains("divergence_kind = 'missing_pg'"));
        assert!(WORKLIST_SQL.contains("resolved_at IS NULL"));
    }

    // -------------------------------------------------------------------
    // WORKLIST_SQL — live behavioural test against a TEMP-shadowed
    // `ht_reconcile_log`, self-skipping without a reachable DATABASE_URL.
    // -------------------------------------------------------------------
    async fn worklist_probe_conn() -> Option<sqlx::PgConnection> {
        use sqlx::Connection;
        let url = std::env::var("DATABASE_URL").ok()?;
        match sqlx::PgConnection::connect(&url).await {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("backfill_receipt_payments worklist probe SKIPPED — cannot connect to PG: {e}");
                None
            }
        }
    }

    const WORKLIST_PROBE_DDL: &str = "CREATE TEMP TABLE ht_reconcile_log ( \
             id               BIGSERIAL PRIMARY KEY, \
             table_name       TEXT NOT NULL, \
             legacy_pk        TEXT NOT NULL, \
             divergence_kind  TEXT, \
             resolved_at      TIMESTAMPTZ \
         ) ON COMMIT DROP";

    #[tokio::test]
    async fn worklist_sql_selects_only_unresolved_missing_pg_payments() {
        let Some(mut conn) = worklist_probe_conn().await else {
            return;
        };
        use sqlx::Connection;
        let mut tx = conn.begin().await.expect("begin");
        sqlx::query(WORKLIST_PROBE_DDL)
            .execute(&mut *tx)
            .await
            .expect("create temp ht_reconcile_log");
        sqlx::query("SET LOCAL search_path = pg_temp, public")
            .execute(&mut *tx)
            .await
            .expect("set search_path");
        sqlx::query(
            "INSERT INTO ht_reconcile_log (table_name, legacy_pk, divergence_kind, resolved_at) \
             VALUES \
             ('payments', 'B2604-0283', 'missing_pg', NULL), \
             ('payments', 'B2604-0290', 'missing_pg', NULL), \
             ('payments', 'B2604-9999', 'missing_pg', NOW()), \
             ('payments', 'B2604-8888', 'value', NULL), \
             ('checkins', 'CH26-000001', 'missing_pg', NULL)",
        )
        .execute(&mut *tx)
        .await
        .expect("seed rows");

        let mut found: Vec<String> = sqlx::query_scalar(WORKLIST_SQL)
            .fetch_all(&mut *tx)
            .await
            .expect("worklist query must execute");
        found.sort();

        assert_eq!(
            found,
            vec!["B2604-0283".to_string(), "B2604-0290".to_string()],
            "must select only unresolved missing_pg rows for table_name='payments' \
             — not the resolved row, the value-drift row, or the checkins row"
        );

        tx.rollback().await.expect("rollback");
    }
}
