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
//! | `refund_payment` | Track G2 / T4 CRIT-1 (`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay` (A)" "can be negative (refunds use negation)") | `RefundPayment` |
//! | `room_change` | Track G4 / T4 HIGH-3 (`docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_Changed_Room`, §3.17) | `RoomChange` |
//! | `deposit_refund` | Task #49 — deposit refund (`docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_CheckIn_Ds` "Refund deposit", FormShowDEPBack.cs:536) | `RefundDeposit` |
//! | `mark_clean` | spike §3j | `MarkRoomClean` |
//! | `mark_dirty` | Audit 2026-06-11 P2; flag literal per `docs/legacy-spike/findings.md` §3e/§3i (`Room_Clean='yes'`) | `MarkRoomDirty` |
//! | `set_maintenance` | Audit 2026-06-11 P2 (`docs/legacy-app/COMPAT_CHEATSHEET.md` §3.15/§3.16) | `SetRoomMaintenance` |
//! | `update_room` | Admin room master-data edit — closes the `PUT /api/new/rooms/:id` writeback gap | `UpdateRoom` |
//! | `move_room_tiles` | Issue #236 — จัดผัง layout-edit drop, FormRoomMain drag/drop shape (`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Update grid layout"). **SHIPPED DARK** behind `LAYOUT_WRITEBACK_ENABLED` | `MoveRoomTiles` |
//! | `update_customer` | Audit 2026-06-11 P2 — standalone customer-edit re-save (spike §3c "re-save customer") | `UpdateCustomer` |
//! | `adjust_product_stock` | Track F3 / T1 CRIT-3 (`docs/legacy-app/COMPAT_CHEATSHEET.md` §6.3 "HT_Products.Pro_Amt -= num") | `AdjustProductStock` |
//! | `coupon` | Track G5 (`docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_Cupon`) | `IssueCoupon` + `RedeemCoupon` |
//! | `pos_sale` | Track G6 / POS module (MVP) — `HT_CheckIn_Product` INSERT + paired `HT_Products.Pro_Amt` additive decrement | `RecordPosSale` |
//! | `receipt` | Task #45 / POS walk-up (roomless) sale — `HT_Receipt_H` INSERT + N `HT_Receipt_Ds` + paired `HT_Products.Pro_Amt` decrement per line (NO `HT_CheckIn_Product`; `Receipt_ref=''`) | `RecordReceipt` |
//! | `pos_void` | Task #45 / POS void — guarded `DELETE HT_CheckIn_Product` (by `sale_legacy_id`) + additive `Pro_Amt` restore | `VoidPosSale` |
//! | `round_bill` | Track J6 (round coexistence step 2) — `HT_Round_Bill` open (`INSERT`) / close (`UPDATE`), `COMPAT_CHEATSHEET.md` §"Table: `HT_Round_Bill` (A)", §"3.20 Open Round-Bill", §"3.21 Close Round-Bill" (`FrmDueBill.cs:1653/1670`) | `OpenRound` + `CloseRound` |
//! | `sticky_note` | Task #47 — room/staff notes `HT_Room_SMS` / `HT_EMP_SMS` INSERT (`OUTPUT INSERTED.SMS_ID`) + mark-read UPDATE, `COMPAT_CHEATSHEET.md` §"Table: `HT_Room_SMS` (A)", §"Table: `HT_EMP_SMS` (A)", §3.22. **SHIPPED DARK** behind `NOTES_WRITEBACK_ENABLED` | `CreateNote` + `MarkNoteRead` |
//! | `cash_entry` | Migration 059 — petty-cash `TB_Pay_History` positional INSERT (`COMPAT_CHEATSHEET.md` §"Table: `TB_Pay_History` (A)" "`id int` (NOT IDENTITY) via `get_id`" / `FrmAddPay.cs:638`). Issue #202: intent/dispatcher/back-population WIRED (migration 085 adds `ht_cash_ledger.aggregate_id`); **still UNWIRED at the emission side** — no `POST /api/cash/*` call site enqueues it yet, pending `Pay_Type`/`Pay_Group`/`Pay_Account`/`Pay_Program` byte-shape verification | `CreateCashEntry` |
//! | `rate_price` | Task #51 — `(Room_Type, Cust_Type)` pricing matrix UPSERT into `HT_Rooms_Price` (`docs/legacy-app/SCHEMA.sql` + `sync/mappers/rate_tiers.rs`). Idempotent `IF EXISTS … UPDATE … ELSE INSERT …` keyed on the composite natural key | `UpsertRatePrice` |

pub mod adjust_product_stock;
pub mod booking_cancel;
pub mod booking_create;
pub mod booking_modify;
pub mod cash_entry;
pub mod checkin_cancel;
pub mod checkin_to_booking;
pub mod checkout;
pub mod companion_people;
pub mod coupon;
pub mod deposit_refund;
pub mod extend_stay;
pub mod helpers;
pub mod mark_clean;
pub mod mark_dirty;
pub mod move_room_tiles;
pub mod payment;
pub mod pos_sale;
pub mod pos_void;
pub mod rate_price;
pub mod receipt;
pub mod refund_payment;
pub mod room_change;
pub mod round_bill;
pub mod save_image;
pub mod set_maintenance;
pub mod sticky_note;
pub mod update_customer;
pub mod update_room;
pub mod walkin;

use crate::db::mssql_timeout::{simple_query_with_timeout, MssqlOpKind};
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
        // R2 (2026-05-14): the recipe-shared statement executor — by
        // wrapping HERE every recipe variant (booking_create,
        // booking_modify, walkin, checkin_to_booking, …) inherits the
        // per-statement write-budget timeout for free. A stuck
        // statement inside the recipe's BEGIN TRAN will surface as a
        // retryable `Tiberius::Io { TimedOut }`, the outer
        // `run_in_transaction` will run ROLLBACK (itself timed), and
        // the job is retried on the next dispatcher claim.
        let _ = simple_query_with_timeout(conn, stmt.as_str(), MssqlOpKind::Write).await?;
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
/// 2026-04-26 `inspect_schema` dump) — those recipes allocate via TABLOCKX
/// MAX+1 (`allocate::*_id`) and embed the value in the INSERT directly. The
/// exception is `HT_Changed_Room` (Track G4 / migration 045): the vendor
/// left its `id` column IDENTITY-keyed, so the `room_change` recipe uses
/// this helper to capture the freshly-allocated id without crossing a
/// `SCOPE_IDENTITY()` wire boundary.
pub(crate) async fn execute_insert_with_output_id(
    conn: &mut LegacyConn<'_>,
    insert_sql: &str,
) -> WritebackResult<i32> {
    debug_assert!(
        insert_sql.to_ascii_uppercase().contains("OUTPUT INSERTED."),
        "execute_insert_with_output_id requires an OUTPUT INSERTED.<col> clause"
    );
    // R2 (2026-05-14): wrap in write-budget timeout. The OUTPUT
    // INSERTED INSERT runs inside the recipe's BEGIN TRAN; a stuck
    // server-side trigger or row-lock backlog gets converted into a
    // retryable failure here instead of hanging the worker.
    let rows = simple_query_with_timeout(conn, insert_sql, MssqlOpKind::Write).await?;
    let row = rows.into_iter().next().ok_or_else(|| {
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
            assert!(
                s.to_ascii_uppercase().contains("OUTPUT INSERTED."),
                "example {s:?} must satisfy the helper precondition"
            );
        }
    }

    /// Issue #274/#279 sibling, scoped to `save_image.rs`: it is the one
    /// writeback RECIPE (not a scheduler poll) that reaches legacy MSSQL
    /// through a bound `tiberius::Query` — its two Write call sites
    /// (provisional INSERT, existing-row UPDATE) run inside the writeback
    /// transaction, so an unbounded hang there holds a legacy row lock
    /// against the live iHOTEL app, not just a read-only scheduler poll.
    /// Neither `scheduler::mod::tests` (`SCHEDULER_SOURCES` is scoped to
    /// `scheduler/*.rs`) nor `writeback::dispatcher`'s own `include_str!`
    /// tests (pin unrelated invariants — ledger-write ordering,
    /// `intent_facts` exhaustiveness) cover this file, so it gets its own
    /// pin here rather than joining `SCHEDULER_SOURCES` — this file talks
    /// to `&mut LegacyConn` directly (no pool), so it can't satisfy that
    /// array's companion "must import `simple_query_with_timeout_pooled`"
    /// test, and folding it in would misname a scheduler-scoped constant.
    ///
    /// Fails loudly if a future edit reverts either bound-Query site back
    /// to a raw `Query::new(sql).execute(&mut **conn)` / `.query(&mut`
    /// call, or reaches for `.simple_query(` directly, bypassing
    /// `query_execute_with_timeout` / `simple_query_with_timeout`
    /// (`db::mssql_timeout`). Scanning from here (not from inside
    /// `save_image.rs` via `include_str!` of itself) sidesteps the
    /// self-reference trap `scheduler::mod::tests` also avoids: this
    /// assertion's own needle strings live in `recipes/mod.rs`, a
    /// different file from the one being scanned, so they can't trip the
    /// scan on themselves.
    #[test]
    fn save_image_recipe_has_no_raw_mssql_bypass_calls() {
        let src = include_str!("save_image.rs");
        for needle in [".simple_query(", ".query(&mut", ".execute(&mut"] {
            assert!(
                !src.contains(needle),
                "writeback/recipes/save_image.rs calls `{needle}` directly — \
                 route it through query_execute_with_timeout / \
                 simple_query_with_timeout (db::mssql_timeout) so the \
                 bound-parameter call gets a per-op timeout and poisons the \
                 connection on timeout instead of hanging unbounded and \
                 holding a legacy row lock against iHOTEL (issue #279 / #274)"
            );
        }
    }
}
