//! `AdjustProductStock` recipe — Track F3 / `audit-2026-05-13.md` T1 CRIT-3.
//!
//! Single targeted UPDATE that adjusts the legacy `HT_Products.Pro_Amt`
//! stock counter for a product identified by its business key
//! (`Pro_no` = our `prod_legacy_no`).
//!
//! ## Background
//!
//! `docs/legacy-app/COMPAT_CHEATSHEET.md:560-564` documents the stock
//! invariant — every `HT_CheckIn_Product` INSERT/UPDATE/DELETE on the
//! legacy side is paired with
//! `UPDATE HT_Products SET Pro_Amt = Pro_Amt ± num WHERE Pro_no='<id>'`
//! so the running stock counter stays consistent with consumption.
//!
//! Today our app increments `ht_inventory_items.item_current_stock` for
//! housekeeping / minibar receipts but never decrements `Pro_Amt`. The
//! invariant breaks end-to-end: iHOTEL keeps shipping decrements on its
//! own sales while our stock-in receipts never propagate. This recipe
//! closes the gap from our side — when our app records a stock movement
//! against a product that has a `prod_legacy_no`, we emit one
//! `AdjustProductStock` intent and the writeback worker fires the same
//! `Pro_Amt = Pro_Amt ± delta` UPDATE the legacy app would.
//!
//! ## Byte-shape contract
//!
//! Exactly one statement is built, matching the legacy app's pattern:
//!
//! ```text
//! UPDATE HT_Products SET Pro_Amt = Pro_Amt + <delta> WHERE Pro_no='<no>'
//! ```
//!
//! Notes:
//! - `delta` is an `f64` so callers can pass fractional units
//!   (`Pro_Amt` is `numeric` in legacy; `Pro_Unit` may be `'litre'`).
//!   We format with 3 decimal places so the wire shape stays
//!   deterministic for tests.
//! - The recipe is additive (`Pro_Amt + delta`) so negative deltas
//!   decrement. This mirrors the legacy pattern verbatim and means a
//!   concurrent iHOTEL stock movement does NOT clobber our update.
//! - `Pro_no` is single-quoted via `sql_quote` (apostrophe-safe).
//! - The recipe does **not** assert the row exists. A missing
//!   `Pro_no` yields `0 rows affected` — the dispatcher does not treat
//!   that as an error today (mirrors `mark_clean`'s `WHERE NOT EXISTS`
//!   tolerance). The route layer's pre-validation against `ht_products`
//!   ensures we never dispatch an `AdjustProductStock` with a bogus
//!   legacy_no.
//! - The `reason` audit string is intentionally NOT written to legacy
//!   (no audit column on `HT_Products`). It is captured in the canonical
//!   `ht_inventory_transactions.trans_notes` by the route layer before
//!   the intent is enqueued — that record remains the audit trail.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

/// Build the single UPDATE statement. PURE — no I/O, deterministic on
/// inputs. Exposed so the test suite can lock the exact wire shape.
pub fn build_statements(prod_legacy_no: &str, delta: f64) -> Vec<String> {
    let pro_no_q = sql_quote(prod_legacy_no);
    // Format with 3dp so the wire bytes are stable across runs and the
    // test asserting `Pro_Amt = Pro_Amt + 5` (integer delta) renders as
    // `+ 5.000`. Integer-deltas still serialize identically across hosts
    // — `.3` truncates / pads consistently regardless of locale.
    let delta_str = format!("{:.3}", delta);
    vec![format!(
        "UPDATE HT_Products SET Pro_Amt = Pro_Amt + {delta_str} \
         WHERE Pro_no={pro_no_q}"
    )]
}

/// Execute the recipe. Returns an empty `LegacyIds` (no new legacy id
/// is allocated — the row already exists, we just shift its stock
/// counter). The `prod_legacy_no` is stashed in `LegacyIds.extra` so
/// the writeback worker's `mark_done` log line can correlate.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    prod_legacy_no: &str,
    delta: f64,
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(prod_legacy_no, delta);
    super::execute_all(conn, &statements).await?;
    let mut ids = LegacyIds::new();
    ids.extra.insert(
        "prod_legacy_no".into(),
        serde_json::Value::from(prod_legacy_no.to_string()),
    );
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Locks the exact wire shape called out in the Track F3 brief:
    /// `UPDATE HT_Products SET Pro_Amt = Pro_Amt + 5 WHERE Pro_no='P001'`
    /// (modulo the `.3` decimal padding from `format!("{:.3}")`).
    #[test]
    fn build_statements_matches_compat_cheatsheet_shape() {
        let statements = build_statements("P001", 5.0);
        assert_eq!(statements.len(), 1, "exactly one UPDATE");
        assert_eq!(
            statements[0],
            "UPDATE HT_Products SET Pro_Amt = Pro_Amt + 5.000 \
             WHERE Pro_no='P001'"
        );
    }

    /// A negative delta still uses additive `+` — `+ -3.000` is the
    /// expected SQL Server-accepted shape and preserves concurrency
    /// safety vs a separate `-` arm.
    #[test]
    fn build_statements_uses_additive_form_for_negative_delta() {
        let statements = build_statements("B-001", -3.0);
        assert_eq!(
            statements[0],
            "UPDATE HT_Products SET Pro_Amt = Pro_Amt + -3.000 \
             WHERE Pro_no='B-001'"
        );
    }

    /// Apostrophe in `prod_legacy_no` (defensive — pro_no is a
    /// vendor-controlled field) must be doubled by `sql_quote`.
    #[test]
    fn build_statements_quotes_prod_legacy_no_safely() {
        let statements = build_statements("X'01", 1.0);
        assert!(
            statements[0].contains("WHERE Pro_no='X''01'"),
            "apostrophe must be doubled: {}",
            statements[0]
        );
    }

    /// Fractional units (e.g. minibar measured in litres) round-trip
    /// through the 3dp formatter without loss.
    #[test]
    fn build_statements_preserves_fractional_delta() {
        let statements = build_statements("L-001", 0.250);
        assert!(
            statements[0].contains("Pro_Amt + 0.250"),
            "fractional delta must serialize at 3dp: {}",
            statements[0]
        );
    }

    /// PURE: repeated calls with the same inputs produce byte-identical
    /// output. The recipe captures no Utc::now() and consults no global
    /// state — locks T6 HIGH-1's purity expectation.
    #[test]
    fn build_statements_is_pure_for_identical_inputs() {
        let first = build_statements("P001", 5.0);
        let second = build_statements("P001", 5.0);
        assert_eq!(first, second, "build_statements must be deterministic");
    }

    /// Track F3 invariant smoke test: starting from a known PG stock X,
    /// applying delta N via this recipe yields the same `Pro_Amt = Pro_Amt + N`
    /// shape the legacy app would issue. The wire-roundtrip side is
    /// covered by the worker's integration suite; here we lock the
    /// algebraic shape so a future refactor can't accidentally switch
    /// to absolute SET (which would clobber concurrent iHOTEL writes).
    #[test]
    fn stock_invariant_uses_additive_update_not_absolute_set() {
        let statements = build_statements("P001", 7.0);
        assert!(
            statements[0].contains("Pro_Amt = Pro_Amt + "),
            "must use additive `Pro_Amt = Pro_Amt + N` shape (NOT `SET Pro_Amt = N`); \
             absolute SET would clobber concurrent iHOTEL stock movements. Got: {}",
            statements[0]
        );
        assert!(
            !statements[0].contains("Pro_Amt = 7"),
            "must NOT use absolute-SET shape: {}",
            statements[0]
        );
    }
}
