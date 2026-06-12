//! `MarkRoomDirty` recipe — coexistence audit 2026-06-11 P2,
//! `docs/legacy-app/COMPAT_CHEATSHEET.md` §3.13 (ClickClean.cs:493-540,
//! "housewife start cleaning").
//!
//! Decompile reference (§3.13):
//!
//! ```text
//! Read latest checkout: SELECT TOP 1 cin_no, cin_cust_name FROM View_CheckIn_Ds
//!                       WHERE Cin_room_status='Check-Out' AND cin_room_no=<r>
//!                       ORDER BY cin_room_out DESC
//! UPDATE HT_Rooms SET Room_Clean='no', Room_Clean_Time='' WHERE id=<roomid>
//! INSERT HT_Housewife (h_name=<emp>, h_room=<r>, h_date=now, h_note=<note>,
//!                      h_cin=<latest_cin>, h_cin_name=<latest_custname>)
//! Module1.Power_set(<r>, "OFF", "", "ปิดไฟจากปุ่มทำความสะอาดเรียบร้อย")
//! ```
//!
//! These writes are **byte-identical to the spike §3j mark-clean capture**
//! (`mark-clean-20260424-115026/writes.txt`): the legacy app's distinction
//! between "start cleaning" (dirty) and "cleaning done" lives in the
//! ClickCleanOK follow-up (`Room_Clean_Time=<OADate>`, §3.14), not in the
//! flag statement itself. The recipe therefore **delegates to
//! [`super::mark_clean::build_statements`]** per the `helpers` charter
//! (shared legacy statements stay in lock-step) — and the byte-pinned
//! tests below pin this module's own output independently so any future
//! mark-clean divergence surfaces here too.
//!
//! Deliberate deviations from the decompile, with reasons:
//!
//! * **`Module1.Power_set(..., "OFF", ...)` is NOT replicated.** That call
//!   flips the room's real power relay (`Room_Power_STATUS` +
//!   `HT_POWER_LOG`). Driving physical room power as a side effect of a
//!   PG housekeeping flag-flip is unsafe (a mobile-app mark-dirty must
//!   not cut power to a room a guest just left their luggage in); power
//!   is owned by the dedicated power-control flow.
//! * **The `HT_Housewife` INSERT carries the Track C T5 HIGH-3 dedup
//!   guard** (5-minute `WHERE NOT EXISTS` window on `(h_room, h_cin)`)
//!   instead of the legacy bare `VALUES` — same retry/concurrency
//!   rationale as `mark_clean`. Known artifact: a dirty→clean (or
//!   clean→dirty) pair on the same room within 5 minutes logs only ONE
//!   audit row, because both flows guard on the same `(h_room, h_cin)`
//!   key. The `HT_Rooms` flag statement always runs, so grid state stays
//!   correct; only the audit log under-counts in that window.
//! * `h_note` is `''` — iHOTEL lets the housekeeper type a free-text
//!   note; our mark-dirty command has no note field today.

use chrono::{DateTime, Utc};

use super::mark_clean::{fetch_prior_occupant, PriorOccupant};
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;

/// Build the 2 statements that mark a room dirty. PURE — no I/O.
///
/// `room_id` is `HT_Rooms.id` (numeric internal PK — spike §3j critical
/// finding: NOT `room_no`). Delegates to `mark_clean::build_statements`
/// because the legacy flag + audit statements are byte-identical across
/// the two flows (see module doc).
pub fn build_statements(
    room_id: i32,
    room_no: &str,
    by: &str,
    prior: Option<&PriorOccupant>,
    now: DateTime<Utc>,
) -> Vec<String> {
    super::mark_clean::build_statements(room_id, room_no, by, prior, now)
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

    /// Byte-pin statement 1 against cheatsheet §3.13:
    /// `UPDATE HT_Rooms SET Room_Clean='no', Room_Clean_Time='' WHERE id=`
    /// rendered in the spike capture's lowercase form. Keyed by numeric
    /// `HT_Rooms.id`, never `room_no` (spike §3j critical finding).
    #[test]
    fn statement_one_flips_room_clean_flag_by_numeric_id() {
        let statements = build_statements(6, "306", "Admin", None, pinned_now());
        assert_eq!(statements.len(), 2);
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=6"
        );
        assert!(!statements[0].contains("room_no"));
    }

    /// §3.13 "start cleaning" audit row: h_name=<by>, h_room, h_date,
    /// h_note='', h_cin/h_cin_name = latest checked-out occupant.
    #[test]
    fn statement_two_inserts_housewife_start_cleaning_row() {
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

    /// Lock the byte-identity claim from the module doc: §3.13 mark-dirty
    /// writes are the same statements as the spike §3j mark-clean capture.
    /// If `mark_clean` ever diverges (e.g. a fresh capture shows the
    /// ClickCleanOK OADate write belongs there), this test forces a
    /// deliberate decision here instead of silently changing mark-dirty.
    #[test]
    fn mark_dirty_statements_match_mark_clean_statements_today() {
        let prior = PriorOccupant {
            cin_no: "CH26-005159".into(),
            customer_full_name: "Jane Doe".into(),
        };
        assert_eq!(
            build_statements(6, "306", "Admin", Some(&prior), pinned_now()),
            crate::writeback::recipes::mark_clean::build_statements(
                6,
                "306",
                "Admin",
                Some(&prior),
                pinned_now()
            ),
        );
    }
}
