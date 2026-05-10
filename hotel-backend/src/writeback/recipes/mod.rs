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

/// Read `SCOPE_IDENTITY()` from MSSQL — the IDENTITY value of the last INSERT
/// in the current scope. Use immediately after a recipe's INSERT into an
/// IDENTITY-keyed table (e.g. `HT_CheckIn_Ds`) to capture the assigned id
/// for back-population to PG.
///
/// Returns `Err(Recipe(...))` if MSSQL returns NULL — that means the prior
/// statement did not insert into an IDENTITY-keyed table within the scope,
/// which is a recipe ordering bug.
pub(crate) async fn fetch_scope_identity(
    conn: &mut LegacyConn<'_>,
) -> WritebackResult<i32> {
    let stream = conn.simple_query("SELECT CAST(SCOPE_IDENTITY() AS INT)").await?;
    let row = stream.into_row().await?;
    let row = row.ok_or_else(|| {
        crate::writeback::error::WritebackError::Recipe(
            "SCOPE_IDENTITY returned no row".into(),
        )
    })?;
    let id: Option<i32> = row.get(0);
    id.ok_or_else(|| {
        crate::writeback::error::WritebackError::Recipe(
            "SCOPE_IDENTITY returned NULL — prior statement did not insert into an IDENTITY table".into(),
        )
    })
}

/// Run statements up to and including the one matching `marker_sql_substring`,
/// capture `SCOPE_IDENTITY()`, then run the remaining statements. Used for
/// recipes that need the IDENTITY value of one INSERT to feed into either
/// later SQL or back-population (audit CRIT-3 — `HT_CheckIn_Ds.id`).
///
/// `marker_sql_substring` must match exactly one statement; panics in tests
/// (debug_assert) if 0 or >1 match.
pub(crate) async fn execute_capturing_identity_at(
    conn: &mut LegacyConn<'_>,
    statements: &[String],
    marker_sql_substring: &str,
) -> WritebackResult<i32> {
    let matches: Vec<usize> = statements
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.contains(marker_sql_substring).then_some(i))
        .collect();
    // Hard error in release too (LOW-2 hardening): if a future maintainer
    // adds a second statement matching the marker, capturing the wrong
    // SCOPE_IDENTITY would silently corrupt back-population.
    if matches.len() != 1 {
        return Err(crate::writeback::error::WritebackError::Recipe(format!(
            "execute_capturing_identity_at: marker {:?} matched {} statements; expected exactly 1",
            marker_sql_substring,
            matches.len()
        )));
    }
    let pivot = matches[0];
    execute_all(conn, &statements[..=pivot]).await?;
    let id = fetch_scope_identity(conn).await?;
    execute_all(conn, &statements[pivot + 1..]).await?;
    Ok(id)
}
