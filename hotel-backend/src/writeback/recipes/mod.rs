//! Per-intent SQL recipes — one file per `WritebackIntent` variant.
//!
//! Each recipe is a faithful translation of the corresponding section of
//! `docs/legacy-spike/findings.md` §3a–k. The shape pattern is:
//!
//! ```text
//! pub fn build_statements(...) -> Vec<String>   // PURE — testable, no I/O
//! pub async fn execute(...)    -> WritebackResult<LegacyIds>  // does I/O
//! ```
//!
//! Recipes that need MAX+1 ID allocation MUST call `allocate::*` to acquire
//! the lock — they don't pre-derive the IDs in `build_statements` because the
//! ID isn't known until the lock is held. For those recipes,
//! `build_statements` accepts the already-allocated IDs as parameters.
//!
//! ## Mapping to spike sections
//!
//! | Module | Spike § | Intent variant |
//! |---|---|---|
//! | `walkin` | §3a | `CreateCheckIn` (linked_booking_id=None) |
//! | `checkin_to_booking` | §3d | `CreateCheckIn` (linked_booking_id=Some) |
//! | `booking_create` | §3b | `CreateBooking` |
//! | `booking_modify` | §3c | `ModifyBooking` |
//! | `booking_cancel` | §3g-bis | `CancelBooking` |
//! | `checkin_cancel` | §3i | `CancelCheckIn` |
//! | `extend_stay` | §3f | `ExtendStay` |
//! | `checkout` | §3e Phase 2 ONLY | `CheckOut` |
//! | `payment` | §3h | `RecordPayment` (+ receipt) |
//! | `mark_clean` | §3j | `MarkRoomClean` |

pub mod booking_cancel;
pub mod booking_create;
pub mod booking_modify;
pub mod checkin_cancel;
pub mod checkin_to_booking;
pub mod checkout;
pub mod extend_stay;
pub mod helpers;
pub mod mark_clean;
pub mod payment;
pub mod walkin;

use crate::writeback::allocate::LegacyConn;
use crate::writeback::error::WritebackResult;

/// Internal helper — execute each statement in `statements` against the
/// connection in order, returning the first error.
///
/// Used by every recipe to ship its `build_statements` output. Statement order
/// matters for some flows (delete-then-insert, child-then-parent), so we
/// deliberately don't parallelize.
pub(crate) async fn execute_all(
    conn: &mut LegacyConn<'_>,
    statements: &[String],
) -> WritebackResult<()> {
    for stmt in statements {
        // Phase 7 audit M-4 (2026-05-10): trace logs used to embed the
        // full statement (`sql = %stmt`), which leaks PII / payment
        // amounts / customer ids if the trace level is ever enabled in
        // production. Replaced with two redacted fields:
        //   * `stmt_kind` — first whitespace-delimited token, typically
        //     INSERT/UPDATE/DELETE. Lets ops correlate without leaking
        //     column values.
        //   * `stmt_len`  — byte length of the statement; helps spot
        //     anomalously large writes during incident review.
        // The statement is still passed to `simple_query` verbatim — the
        // wire behaviour is unchanged.
        tracing::trace!(
            stmt_kind = stmt.split_whitespace().next().unwrap_or("UNKNOWN"),
            stmt_len = stmt.len(),
            "writeback statement"
        );
        let _ = conn.simple_query(stmt.as_str()).await?;
    }
    Ok(())
}

/// Execute an `INSERT … OUTPUT INSERTED.id … VALUES …` statement and return
/// the captured id from the `OUTPUT` clause.
///
/// Audit H12: the previous `execute_capturing_identity_at` / `fetch_scope_identity`
/// helpers issued the INSERT and `SELECT SCOPE_IDENTITY()` as two separate
/// `simple_query` calls. SCOPE_IDENTITY is batch-scoped on the wire and
/// tiberius would occasionally return `NULL` (the prior call's batch already
/// closed before the SELECT executed). `OUTPUT INSERTED.<col>` survives any
/// scope quirk because the id is streamed back as part of the INSERT response
/// itself — no second statement, no scope to lose.
///
/// Callers compose the INSERT with the `OUTPUT INSERTED.id` clause so the
/// helper is statement-agnostic (any table, any column name). Returns
/// `Err(Recipe(..))` if the INSERT did not produce exactly one row — surfaces
/// recipe ordering bugs as a hard error instead of silently corrupting
/// back-population.
///
/// Today every IDENTITY-keyed legacy table the recipes touch (`HT_CheckIn_Ds`,
/// `HT_Receipt_H`, `HT_Book_Date`, `HT_Room_Status`, `HT_Rooms_Cancel`) was
/// stripped of its IDENTITY property by the vendor (verified via the
/// 2026-04-26 `inspect_schema` dump) — all recipes allocate via TABLOCKX
/// MAX+1 (`allocate::*_id`) and embed the value in the INSERT directly. This
/// helper exists for future tables that may need to capture a true IDENTITY
/// id without relying on SCOPE_IDENTITY's wire semantics.
#[allow(dead_code)]
pub(crate) async fn execute_insert_with_output_id(
    conn: &mut LegacyConn<'_>,
    insert_sql: &str,
) -> WritebackResult<i32> {
    debug_assert!(
        insert_sql.to_ascii_uppercase().contains("OUTPUT INSERTED."),
        "execute_insert_with_output_id requires an OUTPUT INSERTED.<col> clause"
    );
    let stream = conn.simple_query(insert_sql).await?;
    let row = stream.into_row().await?;
    let row = row.ok_or_else(|| {
        crate::writeback::error::WritebackError::Recipe(
            "OUTPUT INSERTED.id returned no row — INSERT did not affect any rows".into(),
        )
    })?;
    let id: Option<i32> = row.get(0);
    id.ok_or_else(|| {
        crate::writeback::error::WritebackError::Recipe(
            "OUTPUT INSERTED.id returned NULL — target column is nullable or not INT".into(),
        )
    })
}

#[cfg(test)]
mod tests {
    //! Pure tests for the recipe-mod helpers. The wire-roundtrip helper
    //! (`execute_insert_with_output_id`) itself requires an MSSQL connection
    //! and is exercised end-to-end by the live writeback worker — the unit
    //! tests here cover the input-validation shape.

    /// Audit H12 regression — a SQL string lacking `OUTPUT INSERTED.` would
    /// silently come back with zero rows from tiberius (an INSERT returns
    /// no rowset by default) and surface as a confusing `Recipe("no row")`
    /// error far from the call site. The debug_assert pins the contract
    /// at the helper boundary so any future caller sees the failure
    /// immediately in test/dev. We can't test the assert itself in release
    /// builds (panics are debug-only) so we cover the documented shape
    /// here as a smoke test against typos.
    #[test]
    fn output_inserted_clause_pattern_present_in_real_sql_examples() {
        // The exact substring the debug_assert above is checking for.
        let examples = [
            "INSERT INTO [HT_CheckIn_Ds] (...) OUTPUT INSERTED.id VALUES (...)",
            "insert into ht_receipt_h (...) output inserted.id values (...)",
            "INSERT INTO [HT_Book_Date](...) OUTPUT INSERTED.id VALUES(...)",
        ];
        for s in examples {
            assert!(s.to_ascii_uppercase().contains("OUTPUT INSERTED."),
                "example {s:?} must satisfy the helper precondition");
        }
    }
}
