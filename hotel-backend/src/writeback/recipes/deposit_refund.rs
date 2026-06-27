//! `RefundDeposit` recipe — Task #49 (deposit refund / คืนเงินมัดจำ).
//!
//! Mirrors iHOTEL `FormShowDEPBack.cs:536`
//! (`docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_CheckIn_Ds`, lines 466-467):
//!
//! ```text
//! update HT_CheckIn_Ds
//!    set Cin_Dep_return_date=getdate(),
//!        Cin_Dep_Status='คืนเงินแล้ว',
//!        Cin_Dep_return_by='<emp>'
//!  where id=<HT_CheckIn_Ds.id>
//! ```
//!
//! The single legacy UPDATE flips the deposit-lifecycle flag on the per-room
//! folio line. The canonical side (`ht_checkin_rooms.cr_dep_status` +
//! `cr_dep_returned_at` / `cr_dep_returned_by`) was already committed by
//! `service::checkin::CheckInService::refund_deposit` in the same PG
//! transaction that enqueued this intent.
//!
//! ## Keying — `cr_legacy_ds_id`
//!
//! The legacy row is targeted by `HT_CheckIn_Ds.id`, which the writeback worker
//! back-populated onto `ht_checkin_rooms.cr_legacy_ds_id` when the parent
//! check-in's `CreateCheckIn` writeback landed (Track B4). The dispatcher
//! resolves it from PG (`SELECT cr_legacy_ds_id … WHERE cr_id = …`) and passes
//! it in. When it is still `None` — the check-in's own writeback hasn't
//! completed yet, or the folio originated entirely legacy-side and was never
//! mirrored — the recipe **no-ops with a WARN** rather than failing the job:
//! the canonical refund already committed, and the next Change-Tracking
//! round-trip converges iHOTEL once the id materializes.
//!
//! ## Deviations from the decompile
//!
//! * `Cin_Dep_return_date=getdate()` is preserved verbatim (the iHOTEL
//!   statement uses the server clock). The canonical `cr_dep_returned_at`
//!   carries the authoritative app-side instant; the legacy column is
//!   informational and re-mirrored back through the CT sync. Keeping
//!   `getdate()` makes the legacy write byte-identical to iHOTEL and the
//!   statement builder fully deterministic from `(id, by)` alone.
//! * The recipe is idempotent on retry: re-running the UPDATE re-sets the same
//!   flag values, so it is deliberately NOT gated by the legacy idempotency
//!   ledger (it allocates no sequential id).

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

/// The Thai literal iHOTEL writes into `HT_CheckIn_Ds.Cin_Dep_Status` once a
/// deposit is refunded (cheatsheet §`HT_CheckIn_Ds`, "Refund deposit").
pub const DEP_STATUS_REFUNDED: &str = "คืนเงินแล้ว";

/// Build the single deposit-refund UPDATE. PURE — no I/O.
///
/// `legacy_ds_id` is the legacy `HT_CheckIn_Ds.id` (== canonical
/// `cr_legacy_ds_id`). `by` is the operator, escaped via [`sql_quote`].
/// Bare-identifier lowercase `update` matches the legacy app's UPDATE shape
/// (findings §4d) and the byte-pinned mark-clean / room-change companions.
pub fn build_statement(legacy_ds_id: i32, by: &str) -> String {
    let by_q = sql_quote(by);
    let status_q = sql_quote(DEP_STATUS_REFUNDED);
    format!(
        "update HT_CheckIn_Ds set Cin_Dep_return_date=getdate(),\
         Cin_Dep_Status={status_q},Cin_Dep_return_by={by_q} where id={legacy_ds_id}"
    )
}

/// Execute the deposit-refund recipe.
///
/// `legacy_ds_id` is `None` when the dispatcher could not resolve the room's
/// `cr_legacy_ds_id` from PG (the parent check-in's writeback hasn't landed, or
/// the folio is legacy-only). In that case the recipe logs a WARN and returns
/// an empty [`LegacyIds`] — a deliberate no-op (the canonical refund already
/// committed; CT converges iHOTEL later). No legacy id is allocated, so nothing
/// is back-populated on success either.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    legacy_ds_id: Option<i32>,
    by: &str,
) -> WritebackResult<LegacyIds> {
    let Some(legacy_ds_id) = legacy_ds_id else {
        tracing::warn!(
            event = "deposit_refund_no_legacy_ds_id",
            "RefundDeposit: room has no resolved cr_legacy_ds_id \
             (HT_CheckIn_Ds.id) yet — skipping legacy UPDATE. Canonical refund \
             already committed; Change-Tracking will converge iHOTEL once the \
             check-in's CreateCheckIn writeback back-populates the id."
        );
        return Ok(LegacyIds::new());
    };

    let statement = build_statement(legacy_ds_id, by);
    super::execute_all(conn, std::slice::from_ref(&statement)).await?;

    // Nothing to back-populate — the recipe targets an existing legacy row by a
    // pre-resolved id and allocates none.
    Ok(LegacyIds::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Byte-pin the statement against cheatsheet §`HT_CheckIn_Ds`
    /// "Refund deposit" (FormShowDEPBack.cs:536): `getdate()` for the return
    /// date, the `'คืนเงินแล้ว'` Thai literal verbatim, operator in
    /// `Cin_Dep_return_by`, keyed on numeric `HT_CheckIn_Ds.id`.
    #[test]
    fn build_statement_matches_form_show_depback() {
        let sql = build_statement(4231, "Front Desk");
        assert_eq!(
            sql,
            "update HT_CheckIn_Ds set Cin_Dep_return_date=getdate(),\
             Cin_Dep_Status='คืนเงินแล้ว',Cin_Dep_return_by='Front Desk' where id=4231"
        );
    }

    /// The status flag is the exact Thai "returned" literal (not the
    /// collected / no-deposit ones) — guard against an accidental swap.
    #[test]
    fn build_statement_uses_returned_status_literal() {
        let sql = build_statement(1, "Nok");
        assert!(sql.contains("Cin_Dep_Status='คืนเงินแล้ว'"));
        assert!(!sql.contains("ยังไม่คืนค่ามัดจำ"));
        assert!(!sql.contains("ไม่เก็บค่ามัดจำ"));
    }

    /// Operator names round-trip through `sql_quote` — an embedded apostrophe
    /// must be doubled (same escaping every recipe relies on).
    #[test]
    fn build_statement_doubles_embedded_quotes_in_operator() {
        let sql = build_statement(99, "O'Brien");
        assert!(sql.contains("Cin_Dep_return_by='O''Brien'"));
        assert!(sql.contains("where id=99"));
    }

    /// PURE — repeated calls with the same inputs produce byte-identical
    /// output (the statement is fully determined by `(id, by)`; `getdate()`
    /// is a server-side literal, not a captured instant).
    #[test]
    fn build_statement_is_pure() {
        assert_eq!(
            build_statement(7, "Admin"),
            build_statement(7, "Admin"),
        );
    }
}
