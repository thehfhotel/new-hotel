//! Periodic-poll mapper for the legacy `HT_Products` master table.
//!
//! Track F3 / `docs/coexistence/audit-2026-05-13.md` T1 CRIT-3.
//!
//! ## Why a poll, not a CT mapper
//!
//! `HT_Products` is **not** currently CT-enabled in legacy MSSQL — the
//! Phase 5 / E1 PK+CT migrations
//! (`migrations/legacy-mssql/`) widened CT coverage to 16 tables but
//! deliberately stopped short of master-data tables that mutate
//! infrequently (the legacy app's FrmManageProduct UI is the only
//! source of edits, and stock-counter updates piggy-back on every
//! `HT_CheckIn_Product` write — also not CT-enabled). A sibling
//! `migrations/legacy-mssql/NNN_enable_ct_ht_products.sql` would be
//! needed to flip CT on, and that requires a vendor maintenance
//! window.
//!
//! Until that lands, this module exposes [`poll_products_once`] — a
//! periodic scan that:
//!
//! 1. Reads every `HT_Products` row from MSSQL (a few hundred at most;
//!    `HT_Products` is product master, not a transactional table).
//! 2. Projects each row through the shared [`project`] helper into a
//!    [`ProductProjection`] (same column → field translation an
//!    eventual CT mapper would use).
//! 3. UPSERTs into canonical `ht_products` keyed on
//!    `prod_legacy_no = Pro_no` (the legacy business key).
//!
//! Wired into [`crate::scheduler::mirror::reload_mirror_dimensions`],
//! so it rides the same reconcile cadence as the `legacy_mirror.*`
//! reloads and `ht_rate_tiers` (15 min on the backend cron; the
//! per-site sync worker's `WORKER_RECONCILE_INTERVAL_SECS` on HF
//! Ville). `--bootstrap` picks it up for free — that path calls
//! `run_sync` too.
//!
//! ## Why this poll is load-bearing, not just nice-to-have
//!
//! `sync/mappers/mirror.rs::upsert_canonical_pos_sale` INNER JOINs
//! `ht_products` to resolve a `HT_CheckIn_Product` folio line, and
//! returns `Err` when the product is absent — deliberately, to hold the
//! watermark for a loud retry. Since the global watermark gates on
//! `!errored` (any table erroring pins the version for ALL CT tables),
//! an unmirrored product does not merely stall POS lines: it freezes
//! that site's entire CT sync. Before this poll existed, `ht_products`
//! was written only by the admin route `POST /api/new/products` (an
//! operator hand-typing `prod_legacy_no`), so one product created and
//! sold in iHOTEL jammed everything until a human noticed — against a
//! 2-day CT retention window. See the prune note below for why the fix
//! is an upsert-only reload rather than a mirror-the-table reload.
//!
//! ## No prune — deliberate, unlike the `rate_tiers` sibling
//!
//! `rate_tiers::reload_rate_tiers` is UPSERT-only with no prune, which
//! is a known open defect there (a tier deleted in legacy keeps
//! serving). This module reaches the same shape for *different* and
//! *sound* reasons — three independent blockers, any one sufficient:
//!
//! 1. **A prune cannot execute.** `prod_id` is the target of two
//!    `NOT NULL` foreign keys with no cascade —
//!    `ht_pos_sales.sale_product_id` (migration 052) and
//!    `ht_booking_products.bp_product_id` (migration 061), plus
//!    nullable refs from `ht_pos_receipts.line_product_id` (065) and
//!    `ht_inventory_items.inv_product_id` (041). `DELETE FROM
//!    ht_products WHERE …` on any product that was ever sold aborts the
//!    whole reload transaction on an FK violation, every tick, forever.
//! 2. **Absence from legacy is not deletion.** iHOTEL's
//!    `FrmManageProduct` edits master data by DELETE-then-REINSERT
//!    (`COMPAT_CHEATSHEET.md` §3.25 "On master-data edit: HT_Rooms, HT_Products",
//!    the hard-delete list). A poll that read
//!    between those two statements would prune a live SKU — and pruning
//!    a live SKU re-creates exactly the FK-miss jam this module exists
//!    to prevent.
//! 3. **Not every canonical row has a legacy counterpart.**
//!    `routes/new_products.rs::create_product` lets an operator create a
//!    canonical-only product; those rows are legitimately absent from
//!    `HT_Products`.
//!
//! A *soft* prune (`prod_active = FALSE`) is rejected for the same
//! third reason plus one more: `prod_active` is operator-owned local
//! state (`routes/new_products.rs::update_product` writes it) and
//! `service/pos.rs` refuses a sale on an archived product. A poll that
//! wrote it would stomp an operator's archive decision every 15 minutes
//! and could take a sellable product offline. [`upsert_product`]
//! therefore never touches `prod_active`.
//!
//! Net: a product deleted in iHOTEL and never re-inserted leaves a
//! stale canonical row. That is strictly the safe direction of the
//! trade — a stale product row is inert (nothing reads it unless a
//! folio line names it), whereas a missing one halts sync.
//!
//! ## CT-enablement TODO
//!
//! Same shape Track E1 used (PK+CT pair landing in a single
//! `migrations/legacy-mssql/` file). Once `HT_Products` is CT-enabled,
//! replace [`poll_products_once`] with a real `MssqlChangeMapper` impl —
//! the projection / UPSERT helpers in this module are deliberately
//! shaped to drop into the trait without rewrite. The poll itself
//! becomes a one-time backfill on cutover.
//!
//! ## Column mapping
//!
//! | MSSQL `HT_Products` | PG `ht_products`         |
//! |---------------------|---------------------------|
//! | `Pro_no`            | `prod_legacy_no` (UNIQUE) |
//! | `Pro_Name`          | `prod_name`               |
//! | `Pro_Unit`          | `prod_unit`               |
//! | `Pro_PriceA`        | `prod_price`              |
//! | `Pro_Amt`           | `prod_current_stock`      |
//! | `Pro_Type`          | `prod_category`           |
//!
//! Width note: `Pro_Name` is `varchar(500)` in legacy but `prod_name`
//! is `VARCHAR(250)` in PG (migration 041), so [`upsert_product`] clamps
//! — see [`PROD_NAME_MAX_CHARS`]. Every other column is same-width or
//! wider on the PG side.

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use crate::db::DbPool;
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::row::MappableRow;
use crate::sync::SyncError;
use sqlx::PgPool;
use std::time::Instant;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

/// Verbatim MSSQL table name.
pub(crate) const TABLE: &str = "HT_Products";

/// Columns the poll reads from `HT_Products`. Kept in lockstep with
/// [`project`] so a typo on either side surfaces at the row boundary
/// rather than as silent NULLs.
///
/// [`poll_select_sql`] composes the SELECT projection from this list, so
/// it is the single source of truth — and the Track-J1
/// `assert_projection_slice_subset!` lock test below pins it against the
/// live HF Hotel baseline schema.
///
/// **Casing is load-bearing.** tiberius resolves `try_get("<name>")` by
/// exact byte match against the result-set metadata, so these literals
/// must match the catalog spelling (and [`project`]'s reads) verbatim —
/// `Pro_no` lowercase-n, `Pro_PriceA` / `Pro_Amt` / `Pro_Type` /
/// `Pro_Name` / `Pro_Unit` capitalised. The subset macro folds case, so
/// a dedicated exact-case test below covers the runtime path.
pub(crate) const POLL_COLUMNS: &[&str] = &[
    "Pro_no",
    "Pro_Name",
    "Pro_Unit",
    "Pro_PriceA",
    "Pro_Amt",
    "Pro_Type",
];

/// Legacy `Pro_Name` is `varchar(500)`; canonical `prod_name` is
/// `VARCHAR(250)` (migration 041). PG counts characters, MSSQL counts
/// bytes (the legacy DB stores Thai as single-byte TIS-620), so a
/// long Thai name really can exceed the canonical width. Without the
/// clamp, one over-long SKU aborts the reload transaction on
/// `value too long for type character varying(250)` — on every tick,
/// permanently, which is the same site-wide stall this module exists
/// to prevent.
pub(crate) const PROD_NAME_MAX_CHARS: usize = 250;

/// The canonical `ht_products` UPSERT, hoisted to a const so the shape
/// test below can assert the written column set matches
/// [`ProductProjection`]'s fields 1:1 (and that `prod_active` stays out
/// of it). `&'static str` keeps it on sqlx's `SqlSafeStr` fast path — no
/// `AssertSqlSafe` wrapper needed.
const UPSERT_SQL: &str = "INSERT INTO ht_products \
     (prod_legacy_no, prod_name, prod_unit, prod_price, \
      prod_current_stock, prod_category) \
 VALUES ($1, $2, $3, $4::float8, $5::float8, $6) \
 ON CONFLICT (prod_legacy_no) DO UPDATE SET \
     prod_name          = EXCLUDED.prod_name, \
     prod_unit          = EXCLUDED.prod_unit, \
     prod_price         = EXCLUDED.prod_price, \
     prod_current_stock = EXCLUDED.prod_current_stock, \
     prod_category      = EXCLUDED.prod_category, \
     prod_updated_at    = NOW() \
 RETURNING prod_id, (xmax = 0) AS was_insert";

/// Aggregate result of one poll tick. Mirrors
/// [`crate::sync::mappers::rate_tiers::ReloadStats`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ReloadStats {
    /// Rows that reached the canonical UPSERT. Inserted or updated —
    /// `ON CONFLICT` doesn't tell us which and observability doesn't
    /// need the split.
    pub upserted: i64,
    /// Rows rejected before PG: a NULL `Pro_no` (projection error) or a
    /// blank-after-trim business key. Logged in aggregate so an operator
    /// can spot a sudden surge of malformed legacy rows.
    pub skipped: i64,
}

/// Compose the poll's SELECT from [`POLL_COLUMNS`] + [`TABLE`]. PURE —
/// no I/O, so the test below can pin the emitted SQL without a legacy
/// connection.
pub(crate) fn poll_select_sql() -> String {
    format!("SELECT {} FROM {}", POLL_COLUMNS.join(", "), TABLE)
}

/// Truncate `s` to at most `max` **characters**, on a char boundary.
///
/// Pure helper behind [`PROD_NAME_MAX_CHARS`]. Returns a borrow, so the
/// overwhelmingly common (short-name) case allocates nothing.
pub(crate) fn clamp_chars(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((byte_idx, _)) => &s[..byte_idx],
        None => s,
    }
}

/// Reject rows the canonical schema can't usefully hold.
///
/// Only the business key is checked: `prod_legacy_no` is `NOT NULL
/// UNIQUE` and is the join target
/// (`upsert_canonical_pos_sale`'s `p.prod_legacy_no = <Cin_Pro_id>`).
/// A blank key can never satisfy that join, and every blank legacy row
/// would collide on the single `''` slot in the UNIQUE index — each
/// tick silently overwriting the last. Skip instead.
///
/// A blank `prod_name` is deliberately NOT rejected: an unnamed product
/// is odd but still a valid FK target, and skipping it would re-create
/// the very jam this poll prevents.
///
/// Pure so the tests below can pin the contract without a database.
pub(crate) fn is_acceptable_projection(p: &ProductProjection) -> bool {
    !p.prod_legacy_no.trim().is_empty()
}

/// Owned snapshot of a single `HT_Products` row.
///
/// `prod_legacy_no` is required (Pro_no is NOT NULL in legacy);
/// everything else is `Option` because the legacy schema permits
/// NULLs on display / category columns.
#[derive(Debug, Clone, PartialEq)]
pub struct ProductProjection {
    pub prod_legacy_no: String,
    pub prod_name: String,
    pub prod_unit: Option<String>,
    pub prod_price: f64,
    pub prod_current_stock: f64,
    pub prod_category: Option<String>,
}

/// Project one MSSQL row into a [`ProductProjection`]. PURE — no I/O.
pub fn project(row: &dyn MappableRow) -> Result<ProductProjection, SyncError> {
    let prod_legacy_no = row
        .try_get_str("Pro_no")?
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "Pro_no is NULL — required business key".into(),
        })?
        .to_string();

    // `Pro_Name` is `varchar NOT NULL` in legacy but defensively fall
    // back to empty string so a malformed row doesn't abort the whole
    // poll batch.
    let prod_name = row.try_get_str("Pro_Name")?.unwrap_or("").to_string();

    Ok(ProductProjection {
        prod_legacy_no,
        prod_name,
        prod_unit: row.try_get_str("Pro_Unit")?.map(str::to_string),
        prod_price: row.try_get_decimal("Pro_PriceA")?.unwrap_or(0.0),
        prod_current_stock: row.try_get_decimal("Pro_Amt")?.unwrap_or(0.0),
        prod_category: row.try_get_str("Pro_Type")?.map(str::to_string),
    })
}

/// Top-level entry point — fetch every `HT_Products` row from MSSQL,
/// project, UPSERT in one PG transaction.
///
/// Wired into [`crate::scheduler::mirror::reload_mirror_dimensions`],
/// which logs a failure without aborting the sibling dimension reloads.
/// Returns the tick's [`ReloadStats`] so integration suites can assert
/// counts.
pub async fn poll_products_once(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<ReloadStats, AnyError> {
    let start = Instant::now();
    let (rows, projection_errors) = fetch_legacy_rows(legacy_pool).await?;
    let mut stats = apply_product_rows(pg_pool, &rows).await?;
    // Rows that never projected count as skipped too — an operator
    // watching the aggregate shouldn't have to know which of the two
    // reject paths a malformed row took.
    stats.skipped += projection_errors;
    tracing::info!(
        table = TABLE,
        upserted = stats.upserted,
        skipped = stats.skipped,
        duration_ms = start.elapsed().as_millis(),
        "[Mirror] reloaded ht_products"
    );
    Ok(stats)
}

/// Pull every row from legacy `HT_Products` and project into owned
/// [`ProductProjection`]s. Returns `(projected, projection_errors)`.
///
/// Full-table scan is fine: `HT_Products` is SKU master (hundreds of
/// rows), not transactional.
///
/// A row that fails [`project`] (NULL `Pro_no`) is logged and skipped
/// rather than aborting the batch — one malformed legacy row must not
/// cost every other product its mirror, because a missing product is
/// what stalls CT sync in the first place.
async fn fetch_legacy_rows(
    legacy_pool: &DbPool,
) -> Result<(Vec<ProductProjection>, i64), AnyError> {
    let mut conn = legacy_pool.get().await?;
    // Per-op read timeout, same rationale as `rate_tiers::fetch_legacy_rows`:
    // the table is tiny and this is a mirror-job-only read, but a stuck
    // poll would otherwise pin the whole dimension-reload scheduler.
    let sql = poll_select_sql();
    let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;

    let mut out = Vec::with_capacity(rows.len());
    let mut projection_errors = 0i64;
    for r in &rows {
        // `tiberius::Row` implements `MappableRow`, so the poll and the
        // eventual CT mapper share ONE projection path — exactly the
        // drop-in shape this module was written for.
        match project(r) {
            Ok(p) => out.push(p),
            Err(e) => {
                projection_errors += 1;
                tracing::warn!(
                    table = TABLE,
                    error = %e,
                    "[Mirror] skipping malformed HT_Products row"
                );
            }
        }
    }
    Ok((out, projection_errors))
}

/// Apply a batch of projected rows to PG in one transaction.
///
/// Public so integration tests can feed pre-built [`ProductProjection`]s
/// without a live MSSQL connection (the `rate_tiers::apply_rate_tier_rows`
/// idiom).
///
/// UPSERT-only — see the module docstring for why pruning absent rows is
/// unsafe here.
pub async fn apply_product_rows(
    pg_pool: &PgPool,
    rows: &[ProductProjection],
) -> Result<ReloadStats, AnyError> {
    let mut tx = pg_pool.begin().await?;
    let mut stats = ReloadStats::default();

    for p in rows {
        if !is_acceptable_projection(p) {
            stats.skipped += 1;
            continue;
        }
        upsert_product(&mut tx, p).await?;
        stats.upserted += 1;
    }

    tx.commit().await?;
    Ok(stats)
}

/// UPSERT one projected row into `ht_products`. Returns the canonical
/// `prod_id` so callers can resolve FKs without a follow-up SELECT.
///
/// The UPSERT pattern matches the customer mapper: INSERT with
/// `ON CONFLICT (prod_legacy_no) DO UPDATE` so concurrent polls (or
/// the eventual CT mapper) collapse race-safely onto the same row.
/// `aggregate_id` is pinned on first INSERT via a follow-up UPDATE
/// (mirrors the SERIAL-aware pattern used elsewhere in the codebase
/// so subscribers can deduplicate by UUID).
///
/// `prod_active` is intentionally absent from both the column list and
/// the `DO UPDATE SET` list — it is operator-owned local state with no
/// legacy counterpart (see the module docstring's prune note).
pub async fn upsert_product(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    p: &ProductProjection,
) -> Result<i64, SyncError> {
    use sqlx::Row;

    // Single-statement UPSERT. Returns `prod_id` and a `was_insert`
    // discriminator so we can decide whether to pin `aggregate_id`
    // (only on first INSERT — UPDATEs leave it alone, preserving the
    // original UUID that subscribers may have cached).
    let row = sqlx::query(UPSERT_SQL)
        .bind(&p.prod_legacy_no)
        .bind(clamp_chars(&p.prod_name, PROD_NAME_MAX_CHARS))
        .bind(p.prod_unit.as_deref())
        .bind(p.prod_price)
        .bind(p.prod_current_stock)
        .bind(p.prod_category.as_deref())
        .fetch_one(&mut **tx)
        .await?;

    let prod_id: i64 = row.try_get("prod_id")?;
    let was_insert: bool = row.try_get("was_insert").unwrap_or(false);

    if was_insert {
        // Pin aggregate_id on first INSERT. SERIAL `prod_id` is i64
        // (BIGSERIAL on the table) so we narrow to i32 for the
        // aggregate-UUID helper — `HT_Products` row counts are
        // bounded by legacy SKU count (hundreds), comfortably under
        // i32::MAX. Migration 041's `BIGSERIAL` is future-proofing
        // for the unlikely case product churn exceeds 2B over the
        // app's lifetime; the narrow is safe today.
        if let Ok(narrow) = i32::try_from(prod_id) {
            let agg_id = aggregate_uuid(AggregateKind::Product, narrow);
            sqlx::query("UPDATE ht_products SET aggregate_id = $1 WHERE prod_id = $2")
                .bind(agg_id)
                .bind(prod_id)
                .execute(&mut **tx)
                .await?;
        }
    }

    Ok(prod_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    /// Smoke test — full row projects every field through. Pins the
    /// column → field mapping so a typo in [`project`] surfaces here
    /// before it costs an integration run.
    #[test]
    fn project_extracts_all_columns_from_full_row() {
        let row = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Str("B-001".into()))
            .with("Pro_Name", MockValue::Str("Coca-Cola".into()))
            .with("Pro_Unit", MockValue::Str("bottle".into()))
            .with("Pro_PriceA", MockValue::Decimal(25.0))
            .with("Pro_Amt", MockValue::Decimal(120.0))
            .with("Pro_Type", MockValue::Str("B".into()));

        let p = project(&row).expect("full row must project");
        assert_eq!(p.prod_legacy_no, "B-001");
        assert_eq!(p.prod_name, "Coca-Cola");
        assert_eq!(p.prod_unit.as_deref(), Some("bottle"));
        assert_eq!(p.prod_price, 25.0);
        assert_eq!(p.prod_current_stock, 120.0);
        assert_eq!(p.prod_category.as_deref(), Some("B"));
    }

    /// The sentinel "room rent line" Pro_no per
    /// `COMPAT_CHEATSHEET.md` §"Table: `HT_Products` (A)" "Sentinel: `'P001'`"
    /// — projection must round-trip the
    /// literal `'P001'` unchanged so the writeback recipe can target
    /// it by name.
    #[test]
    fn project_round_trips_sentinel_p001_pro_no() {
        let row = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Str("P001".into()))
            .with("Pro_Name", MockValue::Str("ค่าห้องพัก".into()))
            .with("Pro_Unit", MockValue::Null)
            .with("Pro_PriceA", MockValue::Null)
            .with("Pro_Amt", MockValue::Decimal(0.0))
            .with("Pro_Type", MockValue::Null);
        let p = project(&row).expect("sentinel row must project");
        assert_eq!(p.prod_legacy_no, "P001");
        assert_eq!(p.prod_name, "ค่าห้องพัก");
    }

    /// `Pro_no` NULL = malformed legacy row → must surface as a loud
    /// error so the poll batch can log + skip rather than insert a
    /// row with an empty business key (which would violate the
    /// `UNIQUE` constraint on the next NULL row).
    #[test]
    fn project_errors_when_pro_no_is_null() {
        let row = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Null)
            .with("Pro_Name", MockValue::Str("x".into()))
            .with("Pro_Unit", MockValue::Null)
            .with("Pro_PriceA", MockValue::Null)
            .with("Pro_Amt", MockValue::Null)
            .with("Pro_Type", MockValue::Null);
        let err = project(&row).expect_err("NULL Pro_no must abort");
        assert!(err.to_string().contains("Pro_no"));
    }

    /// Nullable columns project to None — the canonical PG schema
    /// matches (prod_unit / prod_category are nullable).
    #[test]
    fn project_tolerates_null_optional_columns() {
        let row = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Str("X-1".into()))
            .with("Pro_Name", MockValue::Str("X".into()))
            .with("Pro_Unit", MockValue::Null)
            .with("Pro_PriceA", MockValue::Null)
            .with("Pro_Amt", MockValue::Null)
            .with("Pro_Type", MockValue::Null);
        let p = project(&row).expect("nullable cols → None");
        assert!(p.prod_unit.is_none());
        assert!(p.prod_category.is_none());
        // Numeric defaults to 0.0 — matches the `DEFAULT 0` in PG.
        assert_eq!(p.prod_price, 0.0);
        assert_eq!(p.prod_current_stock, 0.0);
    }

    /// Stock invariant smoke (Track F3 brief item 4): if PG stock = X
    /// and legacy increments stock by N, the poll's next pass projects
    /// X+N. The full upsert is exercised end-to-end by integration; here
    /// we lock the projection's pass-through arithmetic so a future
    /// projection refactor can't silently coerce the f64 through an
    /// integer truncation.
    #[test]
    fn stock_invariant_passes_through_projection() {
        let row_before = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Str("B-001".into()))
            .with("Pro_Name", MockValue::Str("Coke".into()))
            .with("Pro_Unit", MockValue::Null)
            .with("Pro_PriceA", MockValue::Decimal(25.0))
            .with("Pro_Amt", MockValue::Decimal(100.0))
            .with("Pro_Type", MockValue::Null);
        let p_before = project(&row_before).expect("first pass projects");
        assert_eq!(p_before.prod_current_stock, 100.0);

        // Simulate legacy incrementing Pro_Amt by 5 (e.g. iHOTEL
        // restock).
        let row_after = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Str("B-001".into()))
            .with("Pro_Name", MockValue::Str("Coke".into()))
            .with("Pro_Unit", MockValue::Null)
            .with("Pro_PriceA", MockValue::Decimal(25.0))
            .with("Pro_Amt", MockValue::Decimal(105.0))
            .with("Pro_Type", MockValue::Null);
        let p_after = project(&row_after).expect("second pass projects");
        assert_eq!(
            p_after.prod_current_stock,
            p_before.prod_current_stock + 5.0,
            "stock invariant must pass through projection without loss",
        );
    }

    /// Locks the poll's column list so the SELECT projection and the
    /// projection's column reads stay in lockstep. A divergence would
    /// surface as a missing-column error at the row boundary inside
    /// `project` — the test catches it at build time.
    #[test]
    fn poll_columns_match_projection_reads() {
        for col in ["Pro_no", "Pro_Name", "Pro_Unit", "Pro_PriceA", "Pro_Amt", "Pro_Type"] {
            assert!(
                POLL_COLUMNS.contains(&col),
                "POLL_COLUMNS missing '{col}' — projection reads it via \
                 try_get_*, so the SELECT must list it"
            );
        }
    }

    /// Track J1 projection lock — every column the poll SELECTs must
    /// exist on `HT_Products` per the live HF Hotel baseline schema
    /// dump. A typo'd column here would surface only at runtime as
    /// tiberius `Invalid column name`, which (via the FK-miss path in
    /// `upsert_canonical_pos_sale`) stalls the whole site's CT sync.
    #[test]
    fn poll_columns_are_subset_of_baseline() {
        crate::assert_projection_slice_subset!(POLL_COLUMNS, "HT_Products");
    }

    /// Stricter sibling of the macro above. `assert_projection_slice_subset!`
    /// folds case (SQL Server's collation is case-insensitive), but
    /// tiberius resolves `try_get("<name>")` by **exact byte match**
    /// against the result-set metadata — so a case-only drift between
    /// `POLL_COLUMNS` and the catalog would compile, pass the subset
    /// macro, and then return `None` for every column at runtime.
    #[test]
    fn poll_columns_match_baseline_casing_exactly() {
        let baseline = crate::sync::projection_guard::LEGACY_BASELINE_SCHEMA;
        for col in POLL_COLUMNS {
            let needle = format!("|{col}|");
            let found = baseline
                .lines()
                .filter(|l| l.starts_with("dbo.HT_Products|"))
                .any(|l| l.contains(&needle));
            assert!(
                found,
                "POLL_COLUMNS entry `{col}` does not appear with this exact \
                 casing on dbo.HT_Products in the baseline schema dump. \
                 tiberius column lookup is case-sensitive — a case-only \
                 mismatch reads NULL for every row at runtime."
            );
        }
    }

    /// The emitted SELECT is composed from `POLL_COLUMNS` + `TABLE`, so
    /// adding a column to the const is the ONLY edit needed. Pins the
    /// exact text an operator would see in an MSSQL trace.
    #[test]
    fn poll_select_sql_is_composed_from_the_column_const() {
        let sql = poll_select_sql();
        assert_eq!(
            sql,
            "SELECT Pro_no, Pro_Name, Pro_Unit, Pro_PriceA, Pro_Amt, Pro_Type \
             FROM HT_Products"
        );
        for col in POLL_COLUMNS {
            assert!(sql.contains(col), "SELECT must list `{col}`");
        }
    }

    /// A legacy product row maps to the canonical `ht_products` shape:
    /// every `ProductProjection` field has exactly one target column in
    /// the UPSERT, and the write never touches operator-owned
    /// `prod_active`.
    #[test]
    fn product_row_maps_to_ht_products_shape() {
        let row = HashMapRow::new(TABLE)
            .with("Pro_no", MockValue::Str("B-002".into()))
            .with("Pro_Name", MockValue::Str("น้ำดื่ม".into()))
            .with("Pro_Unit", MockValue::Str("ขวด".into()))
            .with("Pro_PriceA", MockValue::Decimal(15.0))
            .with("Pro_Amt", MockValue::Decimal(48.5))
            .with("Pro_Type", MockValue::Str("B".into()));
        let p = project(&row).expect("row must project");

        // Every projected value round-trips unchanged into the bind set.
        assert_eq!(p.prod_legacy_no, "B-002");
        assert_eq!(p.prod_name, "น้ำดื่ม");
        assert_eq!(p.prod_unit.as_deref(), Some("ขวด"));
        assert_eq!(p.prod_price, 15.0);
        assert_eq!(p.prod_current_stock, 48.5);
        assert_eq!(p.prod_category.as_deref(), Some("B"));

        // Each `ProductProjection` field has exactly one target column,
        // and every non-key column is refreshed on conflict (otherwise a
        // legacy price/stock edit would never reach canonical). Collapse
        // the const's alignment padding so the check isn't whitespace-
        // sensitive.
        let sql: String = UPSERT_SQL.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(sql.contains("ON CONFLICT (prod_legacy_no) DO UPDATE SET"));
        for col in [
            "prod_name",
            "prod_unit",
            "prod_price",
            "prod_current_stock",
            "prod_category",
        ] {
            assert!(
                sql.contains(&format!("{col} = EXCLUDED.{col}")),
                "`{col}` must be refreshed on conflict — a legacy edit \
                 would otherwise never reach the canonical row"
            );
        }

        // `prod_active` is operator-owned: legacy has no counterpart and
        // `service/pos.rs` refuses sales on an archived product, so a
        // poll that wrote it could take a sellable SKU offline every
        // reconcile tick.
        assert!(
            !UPSERT_SQL.contains("prod_active"),
            "the poll must never write prod_active — it is operator-owned \
             local state with no legacy counterpart"
        );
    }

    /// Blank business keys are rejected: `prod_legacy_no` is the UNIQUE
    /// FK join target for POS sales, so every blank row would collide on
    /// the single `''` slot and none of them could ever resolve a sale.
    #[test]
    fn rejects_blank_and_whitespace_business_key() {
        let base = ProductProjection {
            prod_legacy_no: "B-001".into(),
            prod_name: "Coke".into(),
            prod_unit: None,
            prod_price: 25.0,
            prod_current_stock: 0.0,
            prod_category: None,
        };
        assert!(is_acceptable_projection(&base));
        assert!(!is_acceptable_projection(&ProductProjection {
            prod_legacy_no: String::new(),
            ..base.clone()
        }));
        assert!(!is_acceptable_projection(&ProductProjection {
            prod_legacy_no: "   ".into(),
            ..base.clone()
        }));
        // A blank NAME is fine — an unnamed product is still a valid FK
        // target, and skipping it would re-create the sync jam.
        assert!(is_acceptable_projection(&ProductProjection {
            prod_name: String::new(),
            ..base
        }));
    }

    /// Legacy `Pro_Name` is varchar(500), canonical `prod_name` is
    /// VARCHAR(250). Without the clamp one over-long SKU aborts the
    /// reload TX on every tick, forever.
    #[test]
    fn clamps_over_long_product_name_on_a_char_boundary() {
        let short = "Coca-Cola";
        assert_eq!(clamp_chars(short, PROD_NAME_MAX_CHARS), short);

        // Thai is 3 bytes/char in UTF-8 — a naive byte slice here would
        // panic on a non-char-boundary index.
        let long: String = "ก".repeat(400);
        let clamped = clamp_chars(&long, PROD_NAME_MAX_CHARS);
        assert_eq!(clamped.chars().count(), PROD_NAME_MAX_CHARS);
        assert!(clamped.chars().all(|c| c == 'ก'));

        // Exactly at the limit is untouched (no off-by-one truncation).
        let exact: String = "x".repeat(PROD_NAME_MAX_CHARS);
        assert_eq!(clamp_chars(&exact, PROD_NAME_MAX_CHARS), exact.as_str());
    }

    /// Pins the deliberate no-prune decision (module docstring §"No
    /// prune"). `prod_id` is the target of two NOT NULL, no-cascade FKs
    /// (`ht_pos_sales.sale_product_id`, `ht_booking_products.bp_product_id`),
    /// and iHOTEL edits master data by DELETE-then-REINSERT — so a
    /// prune would either abort the reload TX permanently or delete a
    /// live SKU mid-edit and re-create the FK-miss stall. If someone
    /// later adds a prune to "match" the legacy_mirror reloads, this
    /// fails and sends them to the docstring.
    #[test]
    fn reload_never_prunes_canonical_products() {
        let src = include_str!("products.rs");
        let marker = src
            .find("#[cfg(test)]")
            .expect("test module marker must exist");
        // Strip doc/line comments — the prune rationale legitimately
        // NAMES both forbidden tokens in prose. Only executable code and
        // SQL consts are in scope here.
        let code: String = src[..marker]
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in ["DELETE FROM ht_products", "prod_active"] {
            assert!(
                !code.contains(forbidden),
                "`{forbidden}` appeared in the products poll. Pruning \
                 ht_products is unsafe (NOT NULL FKs from ht_pos_sales / \
                 ht_booking_products; iHOTEL edits via DELETE-then-REINSERT) \
                 and prod_active is operator-owned. See the module docstring."
            );
        }
    }
}
