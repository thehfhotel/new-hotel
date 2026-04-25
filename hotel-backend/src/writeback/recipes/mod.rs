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
        tracing::trace!(sql = %stmt, "Writeback statement");
        let _ = conn.simple_query(stmt.as_str()).await?;
    }
    Ok(())
}
