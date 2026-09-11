//! `SetRoomMaintenance` recipe — coexistence audit 2026-06-11 P2,
//! `docs/legacy-app/COMPAT_CHEATSHEET.md` §3.15 (send to maintenance,
//! ClickClean.ButtonX6) / §3.16 (return from maintenance, ClickManternance).
//!
//! One statement:
//!
//! ```text
//! update HT_Rooms set Room_Manternace='yes' where id=<id>   -- to maintenance
//! update HT_Rooms set Room_Manternace='no' where id=<id>    -- back in service
//! ```
//!
//! * `Room_Manternace` is the **verbatim legacy typo**
//!   (`hotel-backend/schema-baseline.txt:510` — `varchar(50) DEFAULT 'no'`).
//! * Value vocabulary is lowercase `'yes'` / `'no'` (cheatsheet
//!   §"1.5 Boolean conventions" "`Room_Manternace` (HT_Rooms)"; the CT room
//!   mapper reads it back through `legacy_yesno_to_bool`). iHOTEL's grid treats
//!   `Room_Manternace='yes'` as "under repair, not rentable" regardless of
//!   the other flags, so this single column is sufficient to take the room
//!   off iHOTEL's market.
//! * Keyed by **numeric `HT_Rooms.id`** (the cheatsheet's `where id=<id>`
//!   form), resolved by the writeback worker from
//!   `ht_rooms_new.legacy_room_id_int` — same resolution as `mark_clean`
//!   (spike §3j critical finding: not `room_no`).
//!
//! Deliberate deviations from the decompile, with reasons:
//!
//! * **`Room_Clean='no'` is NOT bundled** even though the cheatsheet §3.15 /
//!   §3.16 handlers set it alongside the maintenance flag. `Room_Clean` is owned
//!   by the `mark_clean` / `mark_dirty` recipes mirroring canonical
//!   `ht_rooms_new.room_clean`; the canonical maintenance toggle does NOT
//!   flip `room_clean`, so bundling the legacy write would diverge
//!   `Room_Clean` from canonical and trip the room reconcile hash
//!   (`scheduler::sync::room_canonical_hash` includes the clean flag, and
//!   room rows need manual ack — see memory `reconcile_projection_ghosts`).
//! * **`HT_Rooms_Repair` / `HT_Housewife` audit INSERTs are NOT
//!   replicated.** Canonical `ht_maintenance_requests` is the audit trail
//!   for our flow; `HT_Rooms_Repair` is iHOTEL-internal reporting and is
//!   not in the writeback-verified fingerprint set — adding a new write
//!   target needs its own verification pass, deferred until a report
//!   consumer actually requires it.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;

/// Build the single maintenance-flag statement. PURE — no I/O.
///
/// `room_id` is `HT_Rooms.id` (numeric internal PK).
pub fn build_statements(room_id: i32, maintenance: bool) -> Vec<String> {
    let flag = if maintenance { "yes" } else { "no" };
    vec![format!(
        "update HT_Rooms set Room_Manternace='{flag}' where id={room_id}"
    )]
}

/// Execute the maintenance-flag recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    room_id_int: i32,
    maintenance: bool,
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(room_id_int, maintenance);
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new();
    ids.extra
        .insert("room_id".into(), serde_json::Value::from(room_id_int));
    ids.extra.insert(
        "room_manternace".into(),
        serde_json::Value::from(if maintenance { "yes" } else { "no" }),
    );
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Byte-pin the to-maintenance statement (cheatsheet §3.15 /
    /// §"Table: `HT_Rooms` (A)" operations:
    /// `update HT_Rooms set Room_Manternace='yes' where id=<id>`).
    /// The column name keeps the legacy typo verbatim.
    #[test]
    fn build_statements_to_maintenance_matches_cheatsheet() {
        let statements = build_statements(6, true);
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_Manternace='yes' where id=6"
        );
    }

    /// Byte-pin the return-from-maintenance statement (cheatsheet §3.16,
    /// minus the deliberately-omitted `Room_Clean='no'` — see module doc).
    #[test]
    fn build_statements_back_in_service_matches_cheatsheet() {
        let statements = build_statements(50, false);
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_Manternace='no' where id=50"
        );
    }

    /// Spike §3j critical-finding parity: keyed by numeric `HT_Rooms.id`,
    /// never by `room_no`; never `N'…'` literals; never touches the
    /// clean-flag columns owned by mark_clean / mark_dirty.
    #[test]
    fn statement_targets_numeric_id_and_only_the_maintenance_column() {
        for maintenance in [true, false] {
            let stmt = &build_statements(123, maintenance)[0];
            assert!(stmt.contains("where id=123"));
            assert!(!stmt.contains("room_no"), "must not key by room_no");
            assert!(!stmt.contains("N'"), "must not emit N'…' literals");
            assert!(!stmt.contains("Room_Clean"), "Room_Clean is owned by mark_clean/mark_dirty");
        }
    }
}
