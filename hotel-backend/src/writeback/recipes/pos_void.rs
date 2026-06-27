//! `VoidPosSale` recipe — Task #45 / POS void.
//!
//! Reverses a folio POS line that was previously mirrored into legacy
//! `HT_CheckIn_Product` by the `pos_sale` recipe. Our app has already
//! flipped canonical `ht_pos_sales.sale_status='voided'` and restored
//! `ht_products.prod_current_stock` (additive `+qty`); this recipe undoes
//! the legacy mirror so iHOTEL's folio no longer shows the (now-voided)
//! charge and its `Pro_Amt` stock counter is restored.
//!
//! ## The reversal
//!
//! The original `pos_sale` recipe did:
//!
//! ```text
//! INSERT INTO HT_CheckIn_Product (…)  -- id = <sale_legacy_id>
//! UPDATE HT_Products SET Pro_Amt = Pro_Amt + (-qty) WHERE Pro_no=…
//! ```
//!
//! The void inverts it, keyed on the back-populated `sale_legacy_id`
//! (== `HT_CheckIn_Product.id`):
//!
//! ```text
//! DELETE FROM HT_CheckIn_Product WHERE id = <legacy_id>
//! UPDATE HT_Products SET Pro_Amt = Pro_Amt + <qty> WHERE Pro_no = <pro_no>
//! ```
//!
//! ## Idempotency
//!
//! A crash-after-commit retry (or a race with iHOTEL deleting the same
//! line) must NOT double-restore `Pro_Amt`. The recipe reads the line's
//! quantity + product code FROM THE LEGACY ROW ITSELF inside the same
//! batch and gates the DELETE + restore on the row still existing
//! (`IF @num IS NOT NULL`). A second apply finds no row → no-op. Reading
//! the restore amount from the legacy row (not the canonical qty) also
//! guarantees the restore exactly matches what was decremented, even if
//! the two sides ever disagreed.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

/// Build the single guarded T-SQL batch. PURE — no I/O.
///
/// `legacy_id` is the `HT_CheckIn_Product.id` to void (a number — embedded
/// directly, no injection surface). `prod_legacy_no` is the canonical
/// fallback `Pro_no` used only when the legacy row's `Cin_Pro_id` is
/// blank (defensive; normally the recipe restores against the row's own
/// `Cin_Pro_id`).
pub fn build_statement(legacy_id: i32, prod_legacy_no: &str) -> String {
    let prod_q = sql_quote(prod_legacy_no);
    format!(
        "DECLARE @num float, @pno varchar(50); \
         SELECT @num = Cin_Pro_num, @pno = Cin_Pro_id FROM HT_CheckIn_Product WHERE id = {legacy_id}; \
         IF @num IS NOT NULL \
         BEGIN \
           DELETE FROM HT_CheckIn_Product WHERE id = {legacy_id}; \
           UPDATE HT_Products SET Pro_Amt = ISNULL(Pro_Amt,0) + @num \
             WHERE Pro_no = ISNULL(NULLIF(LTRIM(RTRIM(@pno)),''), {prod_q}); \
         END"
    )
}

/// Execute the void recipe. Returns an empty [`LegacyIds`] — the void
/// allocates no new legacy id and the canonical row is already
/// `sale_status='voided'`, so there is nothing to back-populate.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    legacy_id: i32,
    prod_legacy_no: &str,
) -> WritebackResult<LegacyIds> {
    let stmt = build_statement(legacy_id, prod_legacy_no);
    super::execute_all(conn, std::slice::from_ref(&stmt)).await?;
    Ok(LegacyIds::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The batch deletes the line by id and restores `Pro_Amt` additively,
    /// guarded by `IF @num IS NOT NULL` for idempotency.
    #[test]
    fn build_statement_is_guarded_delete_plus_restore() {
        let sql = build_statement(4242, "B-001");
        assert!(sql.contains("SELECT @num = Cin_Pro_num, @pno = Cin_Pro_id FROM HT_CheckIn_Product WHERE id = 4242"));
        assert!(sql.contains("IF @num IS NOT NULL"));
        assert!(sql.contains("DELETE FROM HT_CheckIn_Product WHERE id = 4242"));
        // Additive restore (NOT absolute SET) — survives interleaving.
        assert!(sql.contains("SET Pro_Amt = ISNULL(Pro_Amt,0) + @num"));
        // Fallback Pro_no quoted.
        assert!(sql.contains("'B-001'"));
    }

    /// PURE: identical inputs ⇒ identical output.
    #[test]
    fn build_statement_is_deterministic() {
        assert_eq!(build_statement(1, "P001"), build_statement(1, "P001"));
    }

    /// Fallback Pro_no is SQL-quoted (defensive — apostrophe doubling).
    #[test]
    fn fallback_prod_no_is_quoted() {
        let sql = build_statement(7, "X'1");
        assert!(sql.contains("'X''1'"), "got:\n{sql}");
    }
}
