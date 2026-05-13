//! `RoomChange` recipe — Track G4 / T4 HIGH-3
//! (`docs/coexistence/audit-2026-05-13.md`).
//!
//! Mirrors legacy `Module1.Change_Room` (`docs/legacy-app/COMPAT_CHEATSHEET.md`
//! §`HT_Changed_Room`, §3.17 Change Room Mid-Stay). The receptionist moved
//! an active guest from `from_room` to `to_room`; our app has already
//! recorded the move in canonical `ht_room_changes` and updated the
//! `ht_checkin_rooms` junction. This recipe pushes the legacy mirror:
//!
//! ```text
//! 1. INSERT INTO [HT_Changed_Room]
//!      ([cin_no], [room_before], [room_after], [change_date],
//!       [room_before_price], [Note], [ToPrice])
//!    OUTPUT INSERTED.id
//!    VALUES (<cin_no>, <from>, <to>, <changed_at>,
//!            <room_before_price>, <reason>, <to_price>)
//! ```
//!
//! Then it back-populates the freshly-allocated `HT_Changed_Room.id` onto
//! the canonical `ht_room_changes.rc_legacy_id` column via the
//! [`LegacyIds::extra`] map (the writeback worker's `mark_done` picks the
//! key up and runs the UPDATE in PG).
//!
//! ## Legacy-app behavior we deliberately skip
//!
//! `Module1.Change_Room` (`COMPAT_CHEATSHEET.md` §3.17) also updates the
//! associated `HT_CheckIn_Ds.Cin_Room_No` from the old room to the new one
//! and flips `HT_Rooms.Room_Use` flags for both rooms. We do NOT emit
//! those statements from this recipe because:
//!
//! - The canonical PG state already reflects the move (the service layer
//!   UPDATEd `ht_checkin_rooms.cr_room_id` inside the same transaction).
//! - `HT_CheckIn_Ds.Cin_Room_No` lands on the legacy side via the
//!   existing sync mappers / extend-stay path when the room's stay
//!   timeline gets rewritten.
//! - `HT_Rooms.Room_Use` is housekeeping state — the `mark_clean` /
//!   walkin recipes own it. Touching it from here would risk a
//!   double-write race with the housekeeping flow.
//!
//! Keeping this recipe narrow to the single audit-row INSERT means the
//! writeback contract is a strict superset of what `HT_Changed_Room`
//! itself promises: a faithful audit ledger entry. Re-deriving the
//! occupancy delta is the canonical PG state's job, not the audit row's.
//!
//! ## Race-safety
//!
//! `HT_Changed_Room.id` is `int IDENTITY NOT NULL` (per
//! `docs/legacy-app/SCHEMA.sql:12`) — the recipe lets MSSQL allocate the
//! id and captures it via `OUTPUT INSERTED.id` so we never read
//! `SCOPE_IDENTITY()` across a wire boundary (audit H12). The wrapping
//! `BEGIN TRAN … COMMIT TRAN` in `run_in_transaction` provides
//! TABLOCKX-equivalent isolation for the `mark_done` back-population
//! that follows.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::{LegacyIds, ResolvedRoomChange};
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, sql_quote};

/// PURE inputs for [`build_statements`]. The dispatcher hydrates this from
/// [`ResolvedRoomChange`] before calling into the recipe so the statement
/// builder stays trivially unit-testable.
#[derive(Debug, Clone)]
pub struct RoomChangeInputs<'a> {
    pub cin_no: &'a str,
    pub from_room_no: &'a str,
    pub to_room_no: &'a str,
    pub change_date: chrono::DateTime<chrono::Utc>,
    /// Baht. Per legacy schema `room_before_price float NOT NULL DEFAULT 0`.
    pub room_before_price: f64,
    /// Free-form reason — empty string is treated as `''` on the wire.
    pub reason: &'a str,
    /// Legacy column is `varchar(20)`; we pass through whatever the
    /// canonical row stored. Empty string when not set.
    pub to_price: &'a str,
}

/// Build the single INSERT statement that records the mid-stay room move.
/// PURE — no I/O. The `OUTPUT INSERTED.id` clause is what lets the
/// caller capture the freshly-allocated IDENTITY via
/// `execute_insert_with_output_id`.
pub fn build_insert_statement(inputs: &RoomChangeInputs<'_>) -> String {
    let cin_no_q = sql_quote(inputs.cin_no);
    let from_q = sql_quote(inputs.from_room_no);
    let to_q = sql_quote(inputs.to_room_no);
    let date_q = sql_quote(&format_legacy_datetime(inputs.change_date));
    let reason_q = sql_quote(inputs.reason);
    let to_price_q = sql_quote(inputs.to_price);
    // Wave 6 LOW item 4: 2dp money formatting matches all other recipes.
    let price = format!("{:.2}", inputs.room_before_price);
    format!(
        "INSERT INTO [HT_Changed_Room]([cin_no],[room_before],[room_after],[change_date],\
         [room_before_price],[Note],[ToPrice]) OUTPUT INSERTED.id \
         VALUES({cin_no_q},{from_q},{to_q},{date_q},{price},{reason_q},{to_price_q})"
    )
}

/// Execute the room-change recipe.
///
/// `resolved` carries the fully-hydrated canonical row. The recipe:
///
/// 1. Validates the baht price is finite.
/// 2. Builds + runs the INSERT with `OUTPUT INSERTED.id` to capture the
///    legacy `HT_Changed_Room.id`.
/// 3. Returns [`LegacyIds`] with `cin_no` + `extra.rc_id` + `extra.legacy_id`
///    so the writeback worker's `mark_done` back-populates
///    `ht_room_changes.rc_legacy_id`.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    resolved: &ResolvedRoomChange,
) -> WritebackResult<LegacyIds> {
    super::helpers::validate_finite(&[(
        "room_before_price_baht",
        resolved.room_before_price_baht,
    )])?;

    let inputs = RoomChangeInputs {
        cin_no: &resolved.legacy_cin_no,
        from_room_no: &resolved.from_room_no,
        to_room_no: &resolved.to_room_no,
        change_date: resolved.changed_at,
        room_before_price: resolved.room_before_price_baht,
        reason: &resolved.reason,
        to_price: &resolved.to_price,
    };
    let insert_sql = build_insert_statement(&inputs);
    let legacy_id = super::execute_insert_with_output_id(conn, &insert_sql).await?;

    let mut ids = LegacyIds::new().with_cin_no(resolved.legacy_cin_no.clone());
    ids.extra
        .insert("rc_id".into(), serde_json::Value::from(resolved.rc_id));
    ids.extra.insert(
        "ht_changed_room_id".into(),
        serde_json::Value::from(legacy_id),
    );
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Match the contract documented at the top of this file: a single
    /// INSERT with `OUTPUT INSERTED.id` so the caller captures the freshly
    /// allocated IDENTITY. All 7 columns of `HT_Changed_Room` populated.
    #[test]
    fn build_insert_statement_emits_seven_column_insert_with_output() {
        let inputs = RoomChangeInputs {
            cin_no: "CH26-005230",
            from_room_no: "303",
            to_room_no: "403",
            // 5 AM UTC = noon BKK (the wall-clock the legacy app sees).
            change_date: chrono::Utc.with_ymd_and_hms(2026, 5, 14, 5, 0, 0).unwrap(),
            room_before_price: 890.0,
            reason: "ลูกค้าบ่นเรื่องเสียงดัง",
            to_price: "890",
        };
        let sql = build_insert_statement(&inputs);

        assert!(sql.starts_with("INSERT INTO [HT_Changed_Room]"));
        assert!(sql.contains("OUTPUT INSERTED.id"));
        // Column order matches the cheatsheet shape.
        assert!(sql.contains(
            "([cin_no],[room_before],[room_after],[change_date],\
             [room_before_price],[Note],[ToPrice])"
        ));
        // Identifier escaping passes through.
        assert!(sql.contains("'CH26-005230'"));
        assert!(sql.contains("'303'"));
        assert!(sql.contains("'403'"));
        // Thai literals preserved verbatim — Track C constraint.
        assert!(sql.contains("'ลูกค้าบ่นเรื่องเสียงดัง'"));
        // 2dp money formatting (Wave 6 LOW item 4).
        assert!(sql.contains(",890.00,"));
        assert!(sql.contains("'890'"));
        // Wall-clock matches the legacy app's `M/d/yyyy h:mm:ss tt` format.
        assert!(sql.contains("'5/14/2026 12:00:00 PM'"));
    }

    /// Empty reason / to_price round-trip as `''` (legacy schema accepts
    /// the columns as nullable, but the legacy app stores empty string per
    /// the §3.17 capture — we mirror that to avoid a parity diff).
    #[test]
    fn build_insert_statement_handles_empty_optional_fields() {
        let inputs = RoomChangeInputs {
            cin_no: "CH26-005231",
            from_room_no: "201",
            to_room_no: "202",
            change_date: chrono::Utc.with_ymd_and_hms(2026, 5, 14, 5, 0, 0).unwrap(),
            room_before_price: 0.0,
            reason: "",
            to_price: "",
        };
        let sql = build_insert_statement(&inputs);
        // Each empty string lands as the legacy 2-char literal `''`.
        // 2dp price formatting kicks in even for zero (parity with cancel /
        // checkout recipes).
        assert!(sql.contains(",0.00,''")); // ...,room_before_price,Note,...
        assert!(sql.ends_with(",'','')"));
    }

    /// SQL injection / quote escaping: an operator-supplied reason
    /// containing an apostrophe must round-trip as `''` per the
    /// `sql_quote` doubling convention. This is the same shape every
    /// recipe relies on; locking it here protects against a future
    /// refactor accidentally swapping the helper for a different
    /// escaping rule.
    #[test]
    fn build_insert_statement_doubles_embedded_quotes_in_reason() {
        let inputs = RoomChangeInputs {
            cin_no: "CH26-005232",
            from_room_no: "101",
            to_room_no: "102",
            change_date: chrono::Utc.with_ymd_and_hms(2026, 5, 14, 5, 0, 0).unwrap(),
            room_before_price: 500.0,
            reason: "Guest's complaint about A/C",
            to_price: "500",
        };
        let sql = build_insert_statement(&inputs);
        // Apostrophe doubled per sql_quote convention.
        assert!(sql.contains("'Guest''s complaint about A/C'"));
    }

    /// HIGH-4 validation guard: NaN/Infinity in `room_before_price_baht`
    /// must be rejected before SQL formatting (`format!("{:.2}", f64::NAN)`
    /// would emit `NaN` and break the legacy parse). Locks the contract
    /// `execute()` enforces.
    #[test]
    fn validate_finite_rejects_nan_room_before_price() {
        let result = super::super::helpers::validate_finite(&[(
            "room_before_price_baht",
            f64::NAN,
        )]);
        let err = result.expect_err("NaN must be rejected");
        let msg = err.to_string();
        assert!(msg.contains("room_before_price_baht"), "msg: {msg}");
    }
}
