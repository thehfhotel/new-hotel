//! `MarkRoomDirty` recipe — coexistence audit 2026-06-11 P2.
//!
//! Source of truth for the flag literal is the spike capture set, NOT the
//! decompile prose: `docs/legacy-spike/findings.md` §3e (check-out) and
//! §3i (cancel check-in) are the two live-captured flows in which iHOTEL
//! marks a room dirty, and both write `Room_Clean='yes'`:
//!
//! ```text
//! -- §3e check-out, Phase 2 (checkout-20260424-... /writes.txt)
//! UPDATE HT_Rooms SET room_use='no', Room_Clean='yes',
//!        Room_Use_Count=Room_Use_Count+1
//!  WHERE room_no='402'
//!
//! -- §3i cancel check-in ("now empty AND needs cleaning")
//! UPDATE HT_Rooms SET Room_Clean='yes', Room_Use='no' WHERE room_no='306'
//! ```
//!
//! **Polarity (settled — do not re-derive):** legacy `HT_Rooms.Room_Clean`
//! is a NEEDS-CLEANING flag. `'yes'` = dirty, `'no'` = clean. findings.md
//! §3j states it outright (`Room_Clean='no'` = "no clean needed") and §3i
//! annotates the cancel write as "now empty AND needs cleaning"; the CT
//! room mapper (`sync/mappers/room.rs`, settled + backfilled 2026-06-30)
//! inverts on read for exactly this reason — canonical
//! `ht_rooms_new.room_clean = true` means IS clean. The decompile prose in
//! `docs/legacy-app/COMPAT_CHEATSHEET.md` / `FEATURE_MAP.md` claimed the
//! opposite in several places and is self-contradictory; it was corrected
//! in the same commit as this fix and loses to findings.md + the mapper.
//!
//! **The bug this replaces (live 2026-06-11 → 2026-07-28):** this recipe
//! delegated to [`super::mark_clean::build_statements`] because cheatsheet
//! §3.13 labels `ClickClean` ("`Room_Clean='no', Room_Clean_Time=''`" +
//! `HT_Housewife` INSERT) as "housewife *starts* cleaning" — i.e. dirty.
//! Under the real polarity that statement is the mark-CLEAN write (it is
//! byte-identical to the spike §3j mark-clean capture, which should have been
//! the tell). So "mark dirty" in our app wrote `Room_Clean='no'`, iHOTEL's
//! board kept showing the room clean, and the CT echo of that row —
//! inverted by the mapper — flipped canonical `room_clean` back to `true`
//! within one sync tick. Net effect: mark-dirty was a self-erasing no-op
//! on both sides.
//!
//! Companion columns, verified against the §3e / §3i captures:
//!
//! * **`Room_Use` / `room_use` is NOT written.** Both captures set it to
//!   `'no'` because check-out and cancel also RELEASE the room; a
//!   standalone housekeeping flag-flip must not. A still-occupied room can
//!   legitimately be flagged dirty (mid-stay service request), and writing
//!   `room_use='no'` would evict the guest from iHOTEL's grid.
//! * **`Room_Use_Count` is NOT incremented.** §3e bumps it as part of
//!   closing a stay; incrementing it here would double-count occupancy.
//! * **`Room_Clean_Time` is NOT written.** Neither dirty capture touches
//!   it — it is the "when housekeeping last marked clean" OADate stamp,
//!   owned by the mark-clean side (`mark_clean` clears it to `''`).
//! * **Keyed by numeric `HT_Rooms.id`, not `room_no`.** The §3e/§3i dirty
//!   writes happen to key by `room_no`, but that is incidental to those
//!   whole-room flows; the housekeeping flag statements in the same
//!   captures (§3e Phase 3, §3j) key by `id`, which is the PK and the key
//!   the writeback resolver already carries
//!   (`ht_rooms_new.legacy_room_id_int` — spike §3j critical finding,
//!   §4e "Pick the right one per statement").
//!
//! Further deliberate deviations from the decompile, with reasons:
//!
//! * **`Module1.Power_set(..., "OFF", ...)` is NOT replicated.** That call
//!   flips the room's real power relay (`Room_Power_STATUS` +
//!   `HT_POWER_LOG`). Driving physical room power as a side effect of a
//!   PG housekeeping flag-flip is unsafe (a mobile-app mark-dirty must
//!   not cut power to a room a guest just left their luggage in); power
//!   is owned by the dedicated power-control flow.
//! * **The `HT_Housewife` INSERT is NOT emitted — removed 2026-07-31,
//!   issue #276.** It was carried over from `mark_clean` on the theory
//!   that `HT_Housewife` is a general "every clean / dirty / repair
//!   action" log (`COMPAT_CHEATSHEET.md` §`HT_Housewife` prose) and that
//!   the row was "the only trace of *who* flagged the room" reaching
//!   iHOTEL. Both premises were wrong:
//!
//!   1. **iHOTEL itself never inserts an `HT_Housewife` row on a
//!      standalone dirty flip.** The two live-captured dirty writes
//!      (findings.md §3e Phase 2, §3i) touch only `HT_Rooms` — no
//!      `HT_Housewife` INSERT anywhere in either capture. The decompile
//!      confirms this structurally: `COMPAT_CHEATSHEET.md`'s "Mark dirty"
//!      bullet (check-out / cancel-check-in) has no `HT_Housewife`
//!      step, and `FEATURE_MAP.md` §J7 says it outright — "the dirty
//!      flag itself, `Room_Clean='yes'`, is raised by check-out /
//!      cancel-check-in ... not by these forms" (the housewife-writing
//!      forms are `ClickClean`/`ClickCleanOK`, both mark-CLEAN). So there
//!      was never an iHOTEL "trace of who dirtied the room" to begin
//!      with — `HT_Housewife` is scoped to cleaning/repair completion,
//!      not to dirtying. Suppressing our INSERT does not remove
//!      anything iHOTEL itself provides; it removes a row our own
//!      recipe invented that has no legacy analog.
//!   2. **`h_note` does not discriminate a dirty-flag row from a real
//!      cleaning even if we kept writing it.** Live distribution query
//!      2026-07-31 (`HT_Housewife`, `db`, HF Hotel): 31,802 of ~31,922
//!      rows carry `h_note=''`, the rest are `'ปิดโดยโปรแกรม'` (62,
//!      unrelated system-close note) and `'เปลี่ยนสถานะเป็นซ่อม : '`
//!      (58, the §3.15 send-to-maintenance discriminator). There is no
//!      note pattern anywhere in the real data for a bare dirty flip,
//!      confirming there is nothing to mirror.
//!
//!   `FrmReportHousewife` (รายงานแม่บ้าน) reads `HT_Housewife(R)` +
//!   `TB_MRP_EMPLOYEE(R)` and counts/groups by `h_name`/date
//!   (`REPORTS_INVENTORY.md` §3.10, `FEATURE_MAP.md` §J7 "counts by
//!   employee") with no `h_note` filter documented anywhere in the
//!   decompile. Every row it counts is a real housekeeping action in
//!   iHOTEL's own world; a dirty-flag row from our app inflates that
//!   count with a phantom cleaning. Byte-parity to legacy now means
//!   emitting *nothing* to `HT_Housewife` here, matching iHOTEL exactly
//!   — see issue #276 for the full evidence pass and the two other
//!   options considered (and rejected: inventing a Thai discriminator
//!   literal that appears in no capture, or accepting the phantom count).
//!   `fetch_prior_occupant` / `PriorOccupant` existed solely to populate
//!   that INSERT's `h_cin`/`h_cin_name` — with the INSERT gone, this
//!   recipe no longer performs a prior-occupant lookup (one fewer legacy
//!   MSSQL round trip per dirty flip) and no longer imports them from
//!   `mark_clean`.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;

/// Build the single statement that marks a room dirty. PURE — no I/O.
///
/// `room_id` is `HT_Rooms.id` (numeric internal PK — spike §3j critical
/// finding: NOT `room_no`). This is this recipe's OWN flag write
/// (`Room_Clean='yes'`, findings.md §3e/§3i). It must never delegate to
/// `mark_clean`'s flag statement — that was the 2026-06-11 → 2026-07-28
/// bug — and, since issue #276, it must never emit an `HT_Housewife`
/// audit row either (see module doc).
pub fn build_statements(room_id: i32) -> Vec<String> {
    vec![
        // Raise the needs-cleaning flag — by HT_Rooms.id (numeric).
        // findings.md §3e Phase 2 / §3i: DIRTY is `Room_Clean='yes'`.
        format!("update HT_Rooms set Room_Clean='yes' where id={room_id}"),
    ]
}

/// Execute the mark-dirty recipe.
///
/// `room_no` and `by` are accepted to match the dispatcher's uniform
/// `MarkRoomClean`/`MarkRoomDirty` call shape, but are unused here: the
/// sole statement is keyed by `room_id` alone, and — since issue #276 —
/// this recipe no longer writes an `HT_Housewife` audit row, so there is
/// no `h_room`/`h_name` to populate and no prior-occupant lookup to run.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    _room_no: &str,
    room_id_int: i32,
    _by: &str,
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(room_id_int);
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new();
    ids.extra
        .insert("room_id".into(), serde_json::Value::from(room_id_int));
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};

    fn pinned_now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 11, 9, 30, 0).unwrap()
    }

    /// Byte-pin the (sole) statement against the findings.md §3e check-out
    /// capture (`Room_Clean='yes'` = NEEDS cleaning) rendered in the
    /// housekeeping family's lowercase, `where id=` form (§3e Phase 3 /
    /// §3j). Keyed by numeric `HT_Rooms.id`, never `room_no` (spike §3j
    /// critical finding).
    #[test]
    fn statement_one_raises_needs_cleaning_flag_by_numeric_id() {
        let statements = build_statements(6);
        assert_eq!(statements.len(), 1, "mark-dirty must be single-statement — see issue #276");
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_Clean='yes' where id=6"
        );
        assert!(!statements[0].contains("room_no"));
    }

    /// The regression guard for the 2026-06-11 → 2026-07-28 bug: mark-dirty
    /// delegated to `mark_clean::build_statements` and therefore wrote
    /// `Room_Clean='no'`. The CT room mapper inverts on read
    /// (`sync/mappers/room.rs`), so the echo of that row flipped canonical
    /// `room_clean` back to `true` within one tick. If the flag statement
    /// ever equals mark-clean's again, this fails.
    #[test]
    fn statement_one_is_not_the_mark_clean_flag_write() {
        let dirty = build_statements(6);
        let clean = crate::writeback::recipes::mark_clean::build_statements(
            6,
            "306",
            "Admin",
            None,
            pinned_now(),
        );
        assert_ne!(
            dirty[0], clean[0],
            "mark_dirty must NOT reuse mark_clean's flag statement"
        );
        assert!(
            !dirty[0].contains("Room_Clean='no'"),
            "mark_dirty must never write the CLEAN literal; got: {}",
            dirty[0]
        );
        assert!(
            !dirty[0].contains("Room_Clean_Time"),
            "Room_Clean_Time is owned by mark_clean (findings.md §3j); got: {}",
            dirty[0]
        );
    }

    /// Companion-column verification against findings.md §3e/§3i: the
    /// captures also set `room_use='no'` and bump `Room_Use_Count`, but
    /// those belong to the check-out / cancel flows that RELEASE the room.
    /// A standalone housekeeping flag-flip must touch `Room_Clean` only.
    #[test]
    fn statement_one_touches_only_the_clean_flag() {
        let stmt = &build_statements(6)[0];
        assert!(!stmt.to_lowercase().contains("room_use"));
        assert!(!stmt.contains("Room_Use_Count"));
        assert!(!stmt.contains("Room_Manternace"));
        assert!(!stmt.contains("N'"), "must not emit N'…' literals");
    }

    /// Issue #276 regression guard: mark-dirty must NOT insert an
    /// `HT_Housewife` audit row. Evidence pass (2026-07-31): iHOTEL's own
    /// dirty-flip writes (findings.md §3e Phase 2, §3i) never touch
    /// `HT_Housewife` — that table is written only by the cleaning/repair
    /// actions (`ClickClean`/`ClickCleanOK`/send-to-maintenance —
    /// cheatsheet §3.13-§3.15, `FEATURE_MAP.md` §J7). A standalone dirty
    /// flip must mirror that and emit exactly the flag UPDATE, nothing
    /// else — replaces the old `statement_two_inserts_housewife_audit_row`
    /// / `housewife_insert_keeps_5_minute_dedup_guard` /
    /// `housewife_audit_row_stays_in_lockstep_with_mark_clean` tests,
    /// which asserted the shape of a statement that no longer exists.
    #[test]
    fn mark_dirty_emits_no_housewife_audit_row() {
        let statements = build_statements(6);
        assert_eq!(statements.len(), 1);
        assert!(
            !statements.iter().any(|s| s.contains("HT_Housewife")),
            "mark-dirty must not touch HT_Housewife — see issue #276; got: {statements:?}"
        );
    }

    /// PURE — repeated calls with the same input produce byte-identical
    /// output (T6 HIGH-1 convention). No longer time-threaded: with the
    /// `HT_Housewife` INSERT gone, the sole statement has no `now` input.
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let first = build_statements(6);
        let second = build_statements(6);
        assert_eq!(first, second);
    }
}
