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
//! byte-identical to the §3j mark-clean capture, which should have been
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
//!   §4e "different statements pick different lookup keys").
//!
//! Further deliberate deviations from the decompile, with reasons:
//!
//! * **`Module1.Power_set(..., "OFF", ...)` is NOT replicated.** That call
//!   flips the room's real power relay (`Room_Power_STATUS` +
//!   `HT_POWER_LOG`). Driving physical room power as a side effect of a
//!   PG housekeeping flag-flip is unsafe (a mobile-app mark-dirty must
//!   not cut power to a room a guest just left their luggage in); power
//!   is owned by the dedicated power-control flow.
//! * **The `HT_Housewife` INSERT is retained**, shared byte-for-byte with
//!   `mark_clean` via [`super::mark_clean::build_housewife_audit_insert`].
//!   `HT_Housewife` is iHOTEL's general housekeeping log — "every clean /
//!   dirty / repair action" (`COMPAT_CHEATSHEET.md` §`HT_Housewife`), with
//!   §3.15 send-to-maintenance writing one too — so a row here is in
//!   keeping with the table's contract, and it is the only trace of *who*
//!   flagged the room that reaches iHOTEL. It carries the Track C T5
//!   HIGH-3 dedup guard (5-minute `WHERE NOT EXISTS` window on
//!   `(h_room, h_cin)`) instead of the legacy bare `VALUES` — same
//!   retry/concurrency rationale as `mark_clean`. Known artifact: a
//!   dirty→clean (or clean→dirty) pair on the same room within 5 minutes
//!   logs only ONE audit row, because both flows guard on the same
//!   `(h_room, h_cin)` key. The `HT_Rooms` flag statement always runs, so
//!   grid state stays correct; only the audit log under-counts in that
//!   window.
//! * `h_note` is `''` — iHOTEL lets the housekeeper type a free-text
//!   note; our mark-dirty command has no note field today. (iHOTEL's own
//!   `h_note` is the discriminator between start-cleaning / end-cleaning /
//!   sent-to-repair rows, so an empty note is indistinguishable from a
//!   clean row in `FrmReportHousewife`. Filed as a follow-up rather than
//!   inventing a Thai literal that appears in no capture.)

use chrono::{DateTime, Utc};

use super::mark_clean::{build_housewife_audit_insert, fetch_prior_occupant, PriorOccupant};
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;

/// Build the 2 statements that mark a room dirty. PURE — no I/O.
///
/// `room_id` is `HT_Rooms.id` (numeric internal PK — spike §3j critical
/// finding: NOT `room_no`). Statement 1 is this recipe's OWN flag write
/// (`Room_Clean='yes'`, findings.md §3e/§3i); statement 2 is the audit row
/// shared with `mark_clean`. The two recipes must never delegate to each
/// other's flag statement — that was the 2026-06-11 → 2026-07-28 bug.
pub fn build_statements(
    room_id: i32,
    room_no: &str,
    by: &str,
    prior: Option<&PriorOccupant>,
    now: DateTime<Utc>,
) -> Vec<String> {
    vec![
        // 1. Raise the needs-cleaning flag — by HT_Rooms.id (numeric).
        //    findings.md §3e Phase 2 / §3i: DIRTY is `Room_Clean='yes'`.
        format!("update HT_Rooms set Room_Clean='yes' where id={room_id}"),
        // 2. Audit row in HT_Housewife — shared literal (see module doc).
        build_housewife_audit_insert(room_no, by, prior, now),
    ]
}

/// Execute the mark-dirty recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    room_no: &str,
    room_id_int: i32,
    by: &str,
) -> WritebackResult<LegacyIds> {
    let prior = fetch_prior_occupant(conn, room_no).await?;
    // Capture `Utc::now()` once so the `h_date` stamp is deterministic
    // relative to the rest of the recipe (T6 HIGH-1 convention).
    let now = Utc::now();
    let statements = build_statements(room_id_int, room_no, by, prior.as_ref(), now);
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new();
    ids.extra
        .insert("room_id".into(), serde_json::Value::from(room_id_int));
    if let Some(p) = prior {
        ids.extra
            .insert("prior_cin_no".into(), serde_json::Value::from(p.cin_no));
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn pinned_now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 11, 9, 30, 0).unwrap()
    }

    /// Byte-pin statement 1 against the findings.md §3e check-out capture
    /// (`Room_Clean='yes'` = NEEDS cleaning) rendered in the housekeeping
    /// family's lowercase, `where id=` form (§3e Phase 3 / §3j). Keyed by
    /// numeric `HT_Rooms.id`, never `room_no` (spike §3j critical finding).
    #[test]
    fn statement_one_raises_needs_cleaning_flag_by_numeric_id() {
        let statements = build_statements(6, "306", "Admin", None, pinned_now());
        assert_eq!(statements.len(), 2);
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
        let dirty = build_statements(6, "306", "Admin", None, pinned_now());
        let clean =
            crate::writeback::recipes::mark_clean::build_statements(6, "306", "Admin", None, pinned_now());
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
        let stmt = &build_statements(6, "306", "Admin", None, pinned_now())[0];
        assert!(!stmt.to_lowercase().contains("room_use"));
        assert!(!stmt.contains("Room_Use_Count"));
        assert!(!stmt.contains("Room_Manternace"));
        assert!(!stmt.contains("N'"), "must not emit N'…' literals");
    }

    /// Housekeeping audit row: h_name=<by>, h_room, h_date, h_note='',
    /// h_cin/h_cin_name = latest checked-out occupant.
    #[test]
    fn statement_two_inserts_housewife_audit_row() {
        let prior = PriorOccupant {
            cin_no: "CH26-005159".into(),
            customer_full_name: "Jane Doe".into(),
        };
        let statements = build_statements(6, "306", "Admin", Some(&prior), pinned_now());
        let insert = &statements[1];
        assert!(insert.starts_with("INSERT INTO HT_Housewife"));
        assert!(insert.contains("'Admin'"));
        assert!(insert.contains("'306'"));
        assert!(insert.contains("'CH26-005159'"));
        assert!(insert.contains("'Jane Doe'"));
    }

    /// The Track C dedup guard must carry over (retry / concurrent-event
    /// idempotency — see module doc for the accepted dirty↔clean
    /// cross-suppression artifact inside the 5-minute window).
    #[test]
    fn housewife_insert_keeps_5_minute_dedup_guard() {
        let statements = build_statements(6, "306", "Admin", None, pinned_now());
        let insert = &statements[1];
        assert!(insert.contains("WHERE NOT EXISTS"));
        assert!(insert.contains("h_date > DATEADD(minute, -5, GETDATE())"));
        assert!(insert.contains("h_cin=''"), "no prior occupant ⇒ h_cin=''");
    }

    /// PURE — repeated calls with the same inputs produce byte-identical
    /// output (T6 HIGH-1 convention).
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let first = build_statements(6, "306", "Admin", None, pinned_now());
        let second = build_statements(6, "306", "Admin", None, pinned_now());
        assert_eq!(first, second);
    }

    /// The AUDIT row (statement 2) — and only the audit row — stays
    /// byte-identical to `mark_clean`'s, because both call
    /// `mark_clean::build_housewife_audit_insert`. If that shared literal
    /// ever diverges between the two recipes, this forces a deliberate
    /// decision here instead of silent drift.
    #[test]
    fn housewife_audit_row_stays_in_lockstep_with_mark_clean() {
        let prior = PriorOccupant {
            cin_no: "CH26-005159".into(),
            customer_full_name: "Jane Doe".into(),
        };
        let dirty = build_statements(6, "306", "Admin", Some(&prior), pinned_now());
        let clean = crate::writeback::recipes::mark_clean::build_statements(
            6,
            "306",
            "Admin",
            Some(&prior),
            pinned_now(),
        );
        assert_eq!(dirty[1], clean[1]);
    }
}
