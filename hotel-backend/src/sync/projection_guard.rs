//! Track J1 — projection-lock guard for legacy-MSSQL projections.
//!
//! tiberius (our MSSQL driver) does not validate column names at compile
//! time the way sqlx-macros do for PostgreSQL. A typo'd column in a
//! `SELECT t.<col>, …` projection produces `Invalid column name` only
//! at runtime — by which point the CT watcher has already advanced its
//! watermark past the failing tick and silently dropped every row.
//!
//! Two prod incidents on 2026-05-14 had the same root cause:
//!
//! - **PR #90 (H1 hotfix):** `HT_CheckIn_Ds` projection used four
//!   non-existent column names (`Cin_dep`, `Cin_dep_status`,
//!   `Cin_dep_returned`, `Cin_dep_returned_by`). Every check-in CT row
//!   was dropped for nine hours before the prod-alert fired.
//! - **PR #101 (customer reconcile drift):** `sync_customers` projected
//!   `Cust_Type` (the rate-tier label) instead of `Cust_Type_Main` (the
//!   customer-category literal the CT mapper writes). 54 customer rows
//!   surfaced as systematic value drift that the auto-resolve sweep
//!   could never clear.
//!
//! Both fixes added a single ad-hoc lock test (`#[test]` that hard-codes
//! the legacy column list against the projection const). This module
//! generalises that pattern so every CT mapper + every reconcile-job
//! projection gets the same guarantee without duplicating the parser.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::assert_projection_subset;
//!
//! const BOOK_H_SELECT_COLS: &str = "t.Book_ID, t.Book_Cust_ID, t.Book_Status";
//!
//! #[test]
//! fn book_h_select_cols_are_subset_of_baseline() {
//!     assert_projection_subset!(BOOK_H_SELECT_COLS, "HT_Book_H");
//! }
//! ```
//!
//! The macro parses the projection string (stripping `t.` table-alias
//! prefixes, splitting on `,`, trimming whitespace) and asserts every
//! token appears in the `==== Columns ====` section of
//! `docs/legacy-spike/schema/01-baseline-schema.txt` for the given
//! `<TABLE>`.
//!
//! The baseline file is captured against the live HF Hotel database via
//! `INFORMATION_SCHEMA.COLUMNS` — the authoritative source of truth for
//! what columns exist. A typo'd projection fails the test at CI time,
//! never reaching the watcher.

use std::collections::HashSet;

/// Verbatim contents of the legacy-spike baseline schema dump.
///
/// Loaded via `include_str!` so the schema text travels with the binary
/// and every test sees the same bytes. Lines of interest are in the
/// `==== Columns ====` section in `dbo.<TABLE>|<ord>|<col>|<type>|…`
/// format.
pub const LEGACY_BASELINE_SCHEMA: &str =
    include_str!("../../../docs/legacy-spike/schema/01-baseline-schema.txt");

/// Parse the `==== Columns ====` section of the baseline schema for a
/// single table and return the set of legal column names.
///
/// `table` is the bare table name without the `dbo.` schema prefix
/// (e.g. `"HT_Book_H"`).
///
/// Column names are normalised to **lowercase** before being inserted.
/// SQL Server is case-insensitive by default (the live HF Hotel
/// database uses the SQL_Latin1_General_CP1_CI_AS collation), so
/// `Cin_Date_Out` and `Cin_Date_out` resolve to the same column at
/// query time. The guard mirrors that semantics: a projection that
/// uses `Pay_No` (which the live DB happily resolves to `Pay_no`) is
/// still considered correct. The guard only flags columns whose
/// **letters** don't exist on the table — exactly the bug class that
/// drove PR #90 and #101 (`Cin_dep` doesn't case-insensitively match
/// any column in `HT_CheckIn_Ds`; `Cust_Type` is case-insensitively
/// distinct from `Cust_Type_Main`).
///
/// Returns an empty set when the table has no rows in the dump — caller
/// should treat that as a hard error (every CT-enabled table MUST be
/// present in the baseline).
pub fn parse_baseline_columns(table: &str) -> HashSet<String> {
    let prefix = format!("dbo.{table}|");
    let mut cols = HashSet::new();
    let mut in_columns_section = false;
    for raw_line in LEGACY_BASELINE_SCHEMA.lines() {
        if raw_line.starts_with("==== Columns") {
            in_columns_section = true;
            continue;
        }
        if raw_line.starts_with("==== ") && in_columns_section {
            // Crossed into the next section (PKs / FKs / etc.) — stop.
            break;
        }
        if !in_columns_section {
            continue;
        }
        if !raw_line.starts_with(&prefix) {
            continue;
        }
        // Line shape: `dbo.<TABLE>|<ord>|<col>|<type>|<max_len>|…`.
        // We need field index 2 (the column name).
        let mut parts = raw_line.split('|');
        let _table = parts.next();
        let _ord = parts.next();
        if let Some(col) = parts.next() {
            cols.insert(col.to_lowercase());
        }
    }
    cols
}

/// Parse a SELECT projection string into its bare column tokens.
///
/// Strips the `t.` table-alias prefix (CT mappers all use the alias
/// `t` for the joined CHANGES table) and any surrounding whitespace.
/// Lines wrapped via Rust string-continuation (`\`) flatten into a
/// single comma-separated string already, so a simple `split(',')` is
/// the right granularity.
///
/// Empty tokens are dropped — operators sometimes leave a trailing
/// comma during edits and we shouldn't fail on that.
pub fn parse_select_cols(select: &str) -> Vec<String> {
    select
        .split(',')
        .map(str::trim)
        .filter(|tok| !tok.is_empty())
        .map(|tok| tok.strip_prefix("t.").unwrap_or(tok).to_string())
        .collect()
}

/// Assert every column in a SELECT projection exists on the named legacy
/// table per the baseline schema dump.
///
/// Use in `#[cfg(test)]` modules to lock a projection const against the
/// authoritative HF Hotel schema. Failure shape:
///
/// ```text
/// projection for HT_Book_H references column `Book_Curst_ID` which is
/// not in the HF Hotel legacy schema baseline
/// (docs/legacy-spike/schema/01-baseline-schema.txt). Typo'd projections
/// silently drop every CT row at runtime — see PR #90 / PR #101
/// post-mortems.
/// ```
///
/// The macro intentionally takes the projection as `&str` (works for
/// both `const FOO: &str = "…"` and `const FOO: &[&str] = &[…]` via
/// the second arm) so it can guard the two existing projection shapes
/// without forcing a refactor.
#[macro_export]
macro_rules! assert_projection_subset {
    ($projection:expr, $table:literal) => {{
        let allowed = $crate::sync::projection_guard::parse_baseline_columns($table);
        assert!(
            !allowed.is_empty(),
            "no baseline columns found for table `{}` — \
             docs/legacy-spike/schema/01-baseline-schema.txt may be \
             missing this table or the parser regressed",
            $table,
        );
        for col in $crate::sync::projection_guard::parse_select_cols($projection) {
            // Case-insensitive match — SQL Server default collation is
            // CI; the live HF Hotel DB resolves `Cin_Date_Out` and
            // `Cin_Date_out` to the same column.
            assert!(
                allowed.contains(&col.to_lowercase()),
                "projection for {} references column `{}` which is not \
                 in the HF Hotel legacy schema baseline \
                 (docs/legacy-spike/schema/01-baseline-schema.txt). \
                 Typo'd projections silently drop every CT row at \
                 runtime — see PR #90 / PR #101 post-mortems.",
                $table,
                col,
            );
        }
    }};
}

/// Same as [`assert_projection_subset!`] but for projections expressed
/// as `&[&'static str]` (the `parent_loader.rs` shape) rather than a
/// flat comma-separated `&str`.
///
/// Kept as a separate macro to avoid runtime type-shenanigans; the two
/// callers between them cover every projection in the codebase.
#[macro_export]
macro_rules! assert_projection_slice_subset {
    ($projection:expr, $table:literal) => {{
        let allowed = $crate::sync::projection_guard::parse_baseline_columns($table);
        assert!(
            !allowed.is_empty(),
            "no baseline columns found for table `{}` — \
             docs/legacy-spike/schema/01-baseline-schema.txt may be \
             missing this table or the parser regressed",
            $table,
        );
        for col in $projection.iter() {
            let bare = col.strip_prefix("t.").unwrap_or(col);
            // Case-insensitive — see assert_projection_subset! rationale.
            assert!(
                allowed.contains(&bare.to_lowercase()),
                "projection for {} references column `{}` which is not \
                 in the HF Hotel legacy schema baseline \
                 (docs/legacy-spike/schema/01-baseline-schema.txt). \
                 Typo'd projections silently drop every CT row at \
                 runtime — see PR #90 / PR #101 post-mortems.",
                $table,
                col,
            );
        }
    }};
}

/// Variant of [`assert_projection_slice_subset!`] for projections that
/// read from a `View_*` join over two underlying tables (e.g.
/// `View_Booking_Ds` = `HT_Book_H` ⋈ `HT_Book_Ds`).
///
/// A projection token is accepted iff it exists in either table's
/// baseline column list. This is the cheapest viable guardrail given
/// the baseline-schema dump truncates view definitions — a column
/// referenced by the view must exist on at least one of the two
/// underlying tables.
///
/// `view_label` is only used in failure messages — pass the view name
/// for grep-ability.
#[macro_export]
macro_rules! assert_projection_slice_subset_of_two_tables {
    ($projection:expr, $table_a:literal, $table_b:literal, $view_label:literal) => {{
        $crate::assert_projection_slice_subset_of_two_tables!(
            $projection, $table_a, $table_b, $view_label, &[] as &[&str]
        )
    }};
    (
        $projection:expr,
        $table_a:literal,
        $table_b:literal,
        $view_label:literal,
        $view_extras:expr $(,)?
    ) => {{
        let allowed_a = $crate::sync::projection_guard::parse_baseline_columns($table_a);
        let allowed_b = $crate::sync::projection_guard::parse_baseline_columns($table_b);
        assert!(
            !allowed_a.is_empty(),
            "no baseline columns found for {} — schema dump may be missing it",
            $table_a,
        );
        assert!(
            !allowed_b.is_empty(),
            "no baseline columns found for {} — schema dump may be missing it",
            $table_b,
        );
        // View-aliased / view-computed columns that don't exist as
        // bare columns on either base table but ARE projected by the
        // view. Lowercased to match the baseline parser's case-folding.
        let view_extras: std::collections::HashSet<String> = $view_extras
            .iter()
            .map(|s: &&str| s.to_lowercase())
            .collect();
        for col in $projection.iter() {
            let bare = col.strip_prefix("t.").unwrap_or(col);
            // Case-insensitive — see assert_projection_subset! rationale.
            let lowered = bare.to_lowercase();
            assert!(
                allowed_a.contains(&lowered)
                    || allowed_b.contains(&lowered)
                    || view_extras.contains(&lowered),
                "projection for {} references column `{}` which is not on \
                 either {} or {} (and not in the view's explicit \
                 view-extras allow-list) per the HF Hotel legacy schema \
                 baseline (docs/legacy-spike/schema/01-baseline-schema.txt). \
                 Typo'd projections silently drop every reconcile row \
                 at runtime — see PR #90 / PR #101 post-mortems.",
                $view_label,
                col,
                $table_a,
                $table_b,
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ht_customers_columns_from_baseline() {
        let cols = parse_baseline_columns("HT_Customers");
        // Spot-check the columns the customer CT mapper writes — names
        // are lowercased per the SQL-Server case-insensitive convention.
        assert!(cols.contains("cust_no"), "cust_no must be in baseline");
        assert!(cols.contains("cust_name"), "cust_name must be in baseline");
        assert!(
            cols.contains("cust_type_main"),
            "cust_type_main must be in baseline",
        );
        assert!(
            cols.contains("cust_idcard"),
            "cust_idcard must be in baseline",
        );
        // Total column count for HT_Customers per the baseline (~35).
        assert!(
            cols.len() >= 30,
            "expected ≥30 HT_Customers columns, got {}",
            cols.len(),
        );
    }

    #[test]
    fn returns_empty_set_for_unknown_table() {
        // A table that doesn't exist in the dump must not silently match
        // — callers rely on the empty-set check to surface typos.
        let cols = parse_baseline_columns("HT_NotARealTable_XYZ");
        assert!(cols.is_empty());
    }

    #[test]
    fn parses_comma_separated_select_with_table_alias() {
        let tokens = parse_select_cols("t.Book_ID, t.Book_Cust_ID, t.Book_Status");
        assert_eq!(tokens, vec!["Book_ID", "Book_Cust_ID", "Book_Status"]);
    }

    #[test]
    fn parses_multiline_select_with_string_continuation() {
        // Rust's `\` line-continuation flattens into a single comma-
        // separated string before this function sees it. Confirm
        // whitespace doesn't break tokenisation.
        let select = "t.id,    t.Cust_no,\n     t.Cust_name";
        let tokens = parse_select_cols(select);
        assert_eq!(tokens, vec!["id", "Cust_no", "Cust_name"]);
    }

    #[test]
    fn parse_select_cols_strips_only_t_dot_prefix() {
        // Bare columns (no alias) survive unchanged — used by the
        // reconcile-job projections which SELECT from a single table
        // without a JOIN alias.
        let tokens = parse_select_cols("Cust_no, Cust_name, Cust_Type_Main");
        assert_eq!(tokens, vec!["Cust_no", "Cust_name", "Cust_Type_Main"]);
    }

    #[test]
    fn parse_select_cols_drops_empty_tokens() {
        // Trailing comma / extra whitespace shouldn't insert empty
        // tokens that would fail the contains-check spuriously.
        let tokens = parse_select_cols("t.id, t.Cust_no,, ");
        assert_eq!(tokens, vec!["id", "Cust_no"]);
    }
}
