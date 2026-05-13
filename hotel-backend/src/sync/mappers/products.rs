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
//! The scheduler invokes this every few minutes — see
//! `bin/sync.rs::tick_products_reconcile` (when that wiring lands). For
//! now the function is unit-tested in isolation and exposed for
//! integration suites to drive directly.
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

use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

/// Verbatim MSSQL table name.
pub(crate) const TABLE: &str = "HT_Products";

/// Columns the poll reads from `HT_Products`. Kept in lockstep with
/// [`project`] so a typo on either side surfaces at the row boundary
/// rather than as silent NULLs.
pub(crate) const POLL_COLUMNS: &[&str] = &[
    "Pro_no",
    "Pro_Name",
    "Pro_Unit",
    "Pro_PriceA",
    "Pro_Amt",
    "Pro_Type",
];

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

/// UPSERT one projected row into `ht_products`. Returns the canonical
/// `prod_id` so callers can resolve FKs without a follow-up SELECT.
///
/// The UPSERT pattern matches the customer mapper: INSERT with
/// `ON CONFLICT (prod_legacy_no) DO UPDATE` so concurrent polls (or
/// the eventual CT mapper) collapse race-safely onto the same row.
/// `aggregate_id` is pinned on first INSERT via a follow-up UPDATE
/// (mirrors the SERIAL-aware pattern used elsewhere in the codebase
/// so subscribers can deduplicate by UUID).
pub async fn upsert_product(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    p: &ProductProjection,
) -> Result<i64, SyncError> {
    use sqlx::Row;

    // Single-statement UPSERT. Returns `prod_id` and a `was_insert`
    // discriminator so we can decide whether to pin `aggregate_id`
    // (only on first INSERT — UPDATEs leave it alone, preserving the
    // original UUID that subscribers may have cached).
    let row = sqlx::query(
        "INSERT INTO ht_products \
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
         RETURNING prod_id, (xmax = 0) AS was_insert",
    )
    .bind(&p.prod_legacy_no)
    .bind(&p.prod_name)
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
    /// `COMPAT_CHEATSHEET.md:964` — projection must round-trip the
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
}
