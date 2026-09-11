//! Sticky-note recipes — Task #47 (room & staff notes / โน้ตห้อง / โน้ตพนักงาน).
//!
//! Mirrors iHOTEL's `Room_Note.cs` / `EMP_Note.cs` against the two legacy SMS
//! tables — `docs/legacy-app/SCHEMA.sql` §"Table: dbo.HT_Room_SMS" and
//! §"Table: dbo.HT_EMP_SMS" for the shape; `docs/legacy-app/COMPAT_CHEATSHEET.md`
//! §"Table: `HT_Room_SMS` (A)", §"Table: `HT_EMP_SMS` (A)" and §3.22 for the
//! flow. Both tables share one shape —
//! `SMS_ID int IDENTITY, <key> varchar(50), SMS_Details text, SMS_By
//! varchar(250), SMS_Readed varchar(50)` — differing only in the target key
//! column (`HT_Room_SMS.SMS_Room` = room number vs `HT_EMP_SMS.SMS_TO` = staff
//! username), so one recipe parameterized by [`NoteTargetKind`] covers both.
//!
//! Two recipes (one per intent variant):
//!
//! - [`execute_create`] — `INSERT … (key, SMS_Details, SMS_By, SMS_Readed) …
//!   OUTPUT INSERTED.SMS_ID VALUES (…, 'no')`. The verified iHOTEL shape
//!   (cheatsheet §3.22) initializes `SMS_Readed='no'`. `SMS_ID` is IDENTITY, so
//!   the recipe captures the freshly-allocated id via `OUTPUT INSERTED.SMS_ID`
//!   (the [`super::execute_insert_with_output_id`] helper — same approach as
//!   `room_change`) and returns it in [`LegacyIds::sms_id`] for the writeback
//!   worker to back-populate `ht_notes.note_legacy_id`.
//! - [`execute_mark_read`] — `UPDATE <table> SET SMS_Readed='yes' WHERE
//!   SMS_ID=<id>` (iHOTEL's `Room_Note_Read.cs` / `EMP_Note_Read.cs`). Targets
//!   the legacy PK so the UPDATE is precise and naturally idempotent.
//!
//! Both follow the project recipe pattern: a pure `build_*` statement composer
//! (testable without MSSQL) + a thin `execute_*` wrapper that does the I/O.
//!
//! ## Idempotency
//!
//! `execute_create` is NOT self-idempotent (an IDENTITY INSERT can't be guarded
//! with `WHERE NOT EXISTS` on a not-yet-known id), so the dispatcher LEDGERS it
//! (`dbo.ht_writeback_ledger`) — a crash-after-commit retry is a no-op replay.
//! `execute_mark_read` is an absolute-SET UPDATE keyed on the PK, idempotent by
//! construction.

use crate::outbox::intent::NoteTargetKind;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

/// Resolve the legacy `(table, target-key column)` pair for a note kind.
/// `Room` → `HT_Room_SMS.SMS_Room`; `Staff` → `HT_EMP_SMS.SMS_TO`. PURE.
fn legacy_table_and_key_col(kind: NoteTargetKind) -> (&'static str, &'static str) {
    match kind {
        NoteTargetKind::Room => ("HT_Room_SMS", "SMS_Room"),
        NoteTargetKind::Staff => ("HT_EMP_SMS", "SMS_TO"),
    }
}

/// Build the legacy sticky-note INSERT. PURE — no I/O.
///
/// Column order mirrors the cheatsheet §3.22 shape; `SMS_ID` is omitted
/// (IDENTITY) and captured via the `OUTPUT INSERTED.SMS_ID` clause. `SMS_Readed`
/// is initialized to `'no'` (the mark-read recipe flips it to `'yes'`).
pub fn build_create_statement(
    kind: NoteTargetKind,
    target_key: &str,
    body: &str,
    created_by: &str,
) -> String {
    let (table, key_col) = legacy_table_and_key_col(kind);
    format!(
        "INSERT INTO [{table}] ([{key_col}],[SMS_Details],[SMS_By],[SMS_Readed]) \
         OUTPUT INSERTED.SMS_ID \
         VALUES ({key},{details},{by},'no')",
        key = sql_quote(target_key),
        details = sql_quote(body),
        by = sql_quote(created_by),
    )
}

/// Build the legacy mark-read UPDATE. PURE — no I/O.
///
/// Mirrors `Room_Note_Read.cs` / `EMP_Note_Read.cs`: flip `SMS_Readed='yes'`
/// keyed on the legacy PK `SMS_ID` (one note row).
pub fn build_mark_read_statement(kind: NoteTargetKind, sms_id: i32) -> String {
    let (table, _) = legacy_table_and_key_col(kind);
    format!("UPDATE {table} SET SMS_Readed='yes' WHERE SMS_ID={sms_id}")
}

/// Create recipe — INSERT the legacy SMS row and capture its IDENTITY `SMS_ID`.
///
/// Returns [`LegacyIds`] carrying `sms_id` so the writeback worker's
/// `back_populate_legacy_ids` step can stamp it onto
/// `ht_notes.note_legacy_id`.
pub async fn execute_create(
    conn: &mut LegacyConn<'_>,
    kind: NoteTargetKind,
    target_key: &str,
    body: &str,
    created_by: &str,
) -> WritebackResult<LegacyIds> {
    let stmt = build_create_statement(kind, target_key, body, created_by);
    let sms_id = super::execute_insert_with_output_id(conn, &stmt).await?;
    Ok(LegacyIds::new().with_sms_id(sms_id))
}

/// Mark-read recipe — flip `SMS_Readed='yes'` on the legacy row.
pub async fn execute_mark_read(
    conn: &mut LegacyConn<'_>,
    kind: NoteTargetKind,
    sms_id: i32,
) -> WritebackResult<LegacyIds> {
    let stmt = build_mark_read_statement(kind, sms_id);
    super::execute_all(conn, std::slice::from_ref(&stmt)).await?;
    Ok(LegacyIds::new().with_sms_id(sms_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The room-note INSERT targets `HT_Room_SMS.SMS_Room`, captures the
    /// IDENTITY via `OUTPUT INSERTED.SMS_ID`, and initializes `SMS_Readed='no'`
    /// — pinning the cheatsheet §3.22 shape.
    #[test]
    fn create_room_note_statement_shape() {
        let sql = build_create_statement(NoteTargetKind::Room, "402", "ห้องเสีย", "Admin");
        assert!(sql.contains("INSERT INTO [HT_Room_SMS]"), "{sql}");
        assert!(sql.contains("[SMS_Room]"), "room key column: {sql}");
        assert!(sql.contains("OUTPUT INSERTED.SMS_ID"), "must capture IDENTITY: {sql}");
        assert!(sql.contains("'402'"), "target key: {sql}");
        assert!(sql.contains("'ห้องเสีย'"), "body verbatim: {sql}");
        assert!(sql.contains("'Admin'"), "author: {sql}");
        assert!(sql.trim_end().ends_with(",'no')"), "SMS_Readed initialized to no: {sql}");
    }

    /// The staff-note INSERT targets `HT_EMP_SMS.SMS_TO` (the only difference
    /// from the room variant).
    #[test]
    fn create_staff_note_statement_targets_emp_table() {
        let sql = build_create_statement(NoteTargetKind::Staff, "somchai", "ฝากเวร", "Admin");
        assert!(sql.contains("INSERT INTO [HT_EMP_SMS]"), "{sql}");
        assert!(sql.contains("[SMS_TO]"), "staff key column: {sql}");
        assert!(!sql.contains("SMS_Room"), "must not reference the room column: {sql}");
        assert!(sql.contains("'somchai'"), "target key: {sql}");
    }

    /// Single quotes in the body / key are doubled so the literal can't break
    /// out (the SMS body is free operator text).
    #[test]
    fn create_statement_escapes_apostrophes() {
        let sql = build_create_statement(NoteTargetKind::Room, "402", "guest's key", "O'Neil");
        assert!(sql.contains("'guest''s key'"), "body apostrophe doubled: {sql}");
        assert!(sql.contains("'O''Neil'"), "author apostrophe doubled: {sql}");
    }

    /// Mark-read flips `SMS_Readed='yes'` keyed on the legacy PK, per table.
    #[test]
    fn mark_read_statement_shape_per_kind() {
        assert_eq!(
            build_mark_read_statement(NoteTargetKind::Room, 17),
            "UPDATE HT_Room_SMS SET SMS_Readed='yes' WHERE SMS_ID=17"
        );
        assert_eq!(
            build_mark_read_statement(NoteTargetKind::Staff, 99),
            "UPDATE HT_EMP_SMS SET SMS_Readed='yes' WHERE SMS_ID=99"
        );
    }

    /// `build_create_statement` is deterministic for fixed inputs — keeps it a
    /// pure function the dispatcher can test without an MSSQL connection.
    #[test]
    fn create_statement_is_pure() {
        let a = build_create_statement(NoteTargetKind::Room, "402", "x", "Admin");
        let b = build_create_statement(NoteTargetKind::Room, "402", "x", "Admin");
        assert_eq!(a, b);
    }
}
