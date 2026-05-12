//! `MarkRoomClean` recipe — spike `findings.md` §3j.
//!
//! 2 statements: clear the room's "needs cleaning" flag, then INSERT a
//! housekeeping audit row. The audit row's `h_cin` / `h_cin_name` reference
//! the **prior non-cancelled occupant** of the room, NOT necessarily the most
//! recent (which might be a cancelled check-in).
//!
//! Reference SQL (from `mark-clean-20260424-115026/writes.txt`):
//!
//! ```text
//! 1. update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=6
//!    -- by HT_Rooms.id (numeric internal PK), NOT room_no
//!
//! 2. INSERT INTO HT_Housewife (h_name, h_room, h_date, h_note, h_cin, h_cin_name)
//!    VALUES ('Admin', '306', '4/24/2026 6:50:59 PM', '',
//!            'CH26-005159',                           -- prior real check-in
//!            '<REDACTED-real-guest-name>')                    -- prior customer name
//! ```
//!
//! Spike §3j critical findings:
//! - `HT_Rooms` is updated by **`id` (numeric)**, not `room_no`. Per spike §4e,
//!   different statements pick different lookup keys — be precise.
//! - `Room_Clean='no'` means "no cleaning needed" (already cleaned).
//! - The recipe issues a **lookup query** before the INSERT to find the prior
//!   occupant — see [`fetch_prior_occupant`].
//! - If no prior occupant exists (brand-new room), `h_cin` and `h_cin_name`
//!   are `''` (empty) — matches the legacy app's behavior on day-one rooms.

use chrono::{DateTime, Utc};
use tiberius::Row;

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, sql_quote};

/// Result of the prior-occupant lookup.
#[derive(Debug, Clone, Default)]
pub struct PriorOccupant {
    pub cin_no: String,
    pub customer_full_name: String,
}

/// Build the 2 statements that mark a room clean. PURE — no I/O.
///
/// `room_id` is `HT_Rooms.id` (numeric PK). `room_no` is the display value
/// (e.g. `"306"`). `prior` is the lookup result from [`fetch_prior_occupant`]
/// (or `None` if the room has never been occupied). `now` is threaded in by
/// `execute()` so the function stays purely deterministic for tests —
/// coexistence audit T6 HIGH-1.
pub fn build_statements(
    room_id: i32,
    room_no: &str,
    by: &str,
    prior: Option<&PriorOccupant>,
    now: DateTime<Utc>,
) -> Vec<String> {
    let now_str = format_legacy_datetime(now);
    let now_q = sql_quote(&now_str);
    let by_q = sql_quote(by);
    let room_no_q = sql_quote(room_no);
    let (h_cin_q, h_name_q) = match prior {
        Some(p) => (sql_quote(&p.cin_no), sql_quote(&p.customer_full_name)),
        None => ("''".to_string(), "''".to_string()),
    };

    vec![
        // 1. Clear the cleaning flag — by HT_Rooms.id (numeric)
        format!(
            "update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id={room_id}"
        ),
        // 2. Audit row in HT_Housewife — Track C T5 HIGH-3 dedup guard
        //    (`docs/coexistence/audit-2026-05-13.md`).
        //
        //    HT_Housewife has no UNIQUE constraint we control (the legacy
        //    schema is read-only — we cannot add one without breaking the
        //    legacy app's INSERT path). Without a guard, two concurrent
        //    mark-clean events (housekeeper marks clean in iHOTEL at T0,
        //    our mobile app fires the same intent at T0+50ms) both succeed
        //    and the audit log over-counts.
        //
        //    The `WHERE NOT EXISTS` guard skips the INSERT when a matching
        //    audit row was written for the same (room, prior cin) pair in
        //    the last 5 minutes. The window matches realistic concurrent
        //    housekeeping scenarios:
        //    - A receptionist marks the room clean in iHOTEL and a
        //      housekeeper marks it clean on the mobile app within minutes
        //      of each other — coexistence path.
        //    - The writeback worker retries after a transient network
        //      failure on COMMIT — idempotency path.
        //    Anything beyond 5 minutes is treated as a legitimate
        //    re-clean (e.g. the room was re-occupied and freshly cleaned
        //    again the same shift).
        format!(
            "INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) \
             SELECT {by_q}, {room_no_q}, {now_q}, '',{h_cin_q},{h_name_q} \
             WHERE NOT EXISTS (\
                 SELECT 1 FROM HT_Housewife \
                  WHERE h_room={room_no_q} AND h_cin={h_cin_q} \
                    AND h_date > DATEADD(minute, -5, GETDATE())\
             )"
        ),
    ]
}

/// SELECT the prior occupant of `room_no` whose per-room check-out
/// completed (Wave 5b item 1). Filter mirrors `COMPAT_CHEATSHEET.md:864-866`
/// (`View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no=…
/// order by cin_room_out desc`) so a multi-room check-in where one room is
/// already out and another still occupied returns the freshly-out row, not
/// the still-occupying sibling. Returns `None` if no prior real occupant
/// exists. See [`build_prior_occupant_sql`] for the exact SQL shape.
pub async fn fetch_prior_occupant(
    conn: &mut LegacyConn<'_>,
    room_no: &str,
) -> WritebackResult<Option<PriorOccupant>> {
    let sql = build_prior_occupant_sql(room_no);
    let stream = conn.simple_query(sql).await?;
    let rows: Vec<Row> = stream.into_first_result().await?;
    let Some(row) = rows.first() else { return Ok(None) };
    let cin_no: &str = row.get(0).unwrap_or("");
    let name: &str = row.get(1).unwrap_or("");
    if cin_no.is_empty() {
        return Ok(None);
    }
    Ok(Some(PriorOccupant {
        cin_no: cin_no.to_string(),
        customer_full_name: name.trim().to_string(),
    }))
}

/// Execute the mark-clean recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    room_no: &str,
    room_id_int: i32,
    by: &str,
) -> WritebackResult<LegacyIds> {
    let prior = fetch_prior_occupant(conn, room_no).await?;
    // Coexistence audit T6 HIGH-1: capture `Utc::now()` once so the
    // `h_date` audit stamp is deterministic relative to the rest of the
    // recipe (no longer recomputed inside build_statements).
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

/// Build the prior-occupant lookup SQL — exposed for unit tests so we can
/// assert the WHERE / ORDER BY shape without a live MSSQL.
fn build_prior_occupant_sql(room_no: &str) -> String {
    use crate::writeback::constants::{CIN_ROOM_STATUS_CHECKED_OUT, CIN_STATUS_CANCELLED};
    let room_no_q = sql_quote(room_no);
    let checked_out_q = sql_quote(CIN_ROOM_STATUS_CHECKED_OUT);
    let cancelled_q = sql_quote(CIN_STATUS_CANCELLED);
    // Wave 5b item 1: per-room status filter (`d.Cin_Room_Status='Check-Out'`)
    // matches `COMPAT_CHEATSHEET.md:864-866`. The earlier whole-check-in filter
    // (`h.cin_status NOT IN ('ยกเลิก')`) is kept as belt-and-suspenders so a
    // partially-cancelled multi-room check-in (where one room shows
    // `Check-Out` but the header status was force-set to `ยกเลิก`) doesn't
    // surface as a stale prior occupant.
    //
    // Also pin NULLs-last ordering on `Cin_Room_Out` (audit LOW-4) so a
    // mid-stay row with no checkout time can never out-sort a real prior
    // occupant.
    format!(
        "SELECT TOP 1 h.Cin_no, ISNULL(c.Cust_name, '') + ' ' + ISNULL(c.Cust_name2, '') AS h_cin_name \
         FROM HT_CheckIn_Ds d \
         JOIN HT_CheckIn_H h ON h.Cin_no = d.Cin_No \
         JOIN HT_Customers c ON c.Cust_no = h.Cin_cust_no \
         WHERE d.Cin_Room_No = {room_no_q} \
           AND d.Cin_Room_Status = N{checked_out_q} \
           AND h.cin_status NOT IN (N{cancelled_q}) \
         ORDER BY CASE WHEN d.Cin_Room_Out IS NULL THEN 1 ELSE 0 END, d.Cin_Room_Out DESC"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Fixed instant used by every mark-clean test for the `h_date` stamp.
    /// T6 HIGH-1.
    fn pinned_now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 4, 24, 11, 50, 26).unwrap()
    }

    /// Matches `mark-clean-20260424-115026/writes.txt` lines 1-2 (modulo the
    /// dynamic timestamp which we render with `format_legacy_datetime(now)`).
    #[test]
    fn build_statements_matches_spike_capture_with_prior() {
        let prior = PriorOccupant {
            cin_no: "CH26-005159".into(),
            customer_full_name: "<REDACTED-real-guest-name>".into(),
        };
        let statements = build_statements(6, "306", "Admin", Some(&prior), pinned_now());
        assert_eq!(statements.len(), 2);
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=6"
        );
        assert!(statements[1].contains("INSERT INTO HT_Housewife"));
        assert!(statements[1].contains("'CH26-005159'"));
        assert!(statements[1].contains("'<REDACTED-real-guest-name>'"));
        assert!(statements[1].contains("'Admin'"));
        assert!(statements[1].contains("'306'"));
    }

    /// Coexistence audit T6 HIGH-1: `build_statements` is PURE — repeated
    /// calls with the same inputs produce byte-identical output, including
    /// the `h_date` stamp.
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let first = build_statements(6, "306", "Admin", None, pinned_now());
        let second = build_statements(6, "306", "Admin", None, pinned_now());
        assert_eq!(first, second, "build_statements must be deterministic");
    }

    #[test]
    fn build_statements_uses_empty_strings_when_no_prior_occupant() {
        let statements = build_statements(6, "306", "Admin", None, pinned_now());
        // Both h_cin and h_cin_name should be '' (empty)
        // INSERT format ends with ", h_cin_q, h_name_q)"
        // We include 4 commas after the 4 known values: ('Admin', '306', '<date>', '',
        // then ,'',''")
        assert!(
            statements[1].contains(",'',''"),
            "expected empty cin/name pair, got: {}",
            statements[1]
        );
    }

    #[test]
    fn updates_ht_rooms_by_numeric_id_not_room_no() {
        let statements = build_statements(50, "403", "Admin", None, pinned_now());
        assert!(statements[0].contains("where id=50"));
        assert!(!statements[0].contains("room_no"));
    }

    /// Wave 5b item 1: the prior-occupant lookup must filter by **per-room**
    /// `Cin_Room_Status='Check-Out'`, not whole-check-in status. Otherwise a
    /// multi-room check-in where room A is checked out and room B is still
    /// occupied would return room B's still-occupying detail row as the
    /// "prior occupant" for mark-clean on room A. Per
    /// `COMPAT_CHEATSHEET.md:864-866`.
    #[test]
    fn filters_by_per_room_status_not_whole_checkin() {
        let sql = build_prior_occupant_sql("306");
        assert!(
            sql.contains("d.Cin_Room_Status = N'Check-Out'"),
            "per-room status filter missing: {sql}"
        );
    }

    /// Wave 5b item 1 (defense-in-depth): keep the whole-check-in
    /// `cin_status NOT IN ('ยกเลิก')` guard so a partially-cancelled check-in
    /// with a stuck `Cin_Room_Status='Check-Out'` row can't surface as a
    /// stale prior occupant either.
    #[test]
    fn keeps_cancellation_guard_on_whole_checkin_status() {
        let sql = build_prior_occupant_sql("306");
        assert!(
            sql.contains("h.cin_status NOT IN (N'ยกเลิก')"),
            "whole-check-in cancellation guard missing: {sql}"
        );
    }

    /// Wave 5b item 1 (audit LOW-4): NULL `Cin_Room_Out` rows must sort last,
    /// otherwise a row with a `NULL` checkout time can out-rank a real prior
    /// occupant under SQL Server's NULLS-FIRST default.
    #[test]
    fn orders_nulls_last_for_cin_room_out_desc() {
        let sql = build_prior_occupant_sql("306");
        assert!(
            sql.contains(
                "ORDER BY CASE WHEN d.Cin_Room_Out IS NULL THEN 1 ELSE 0 END, d.Cin_Room_Out DESC"
            ),
            "NULLs-last ordering missing: {sql}"
        );
    }

    #[test]
    fn prior_occupant_sql_quotes_room_no_safely() {
        // Apostrophe in room_no (synthetic edge case) must be doubled by sql_quote.
        let sql = build_prior_occupant_sql("3'06");
        assert!(sql.contains("d.Cin_Room_No = '3''06'"));
    }

    /// Track C — T5 HIGH-3: the HT_Housewife INSERT must be guarded by a
    /// 5-minute dedup window so concurrent iHOTEL + our-app mark-clean
    /// events don't double-write the audit log. Without a UNIQUE
    /// constraint we control on the legacy schema (read-only), this
    /// `WHERE NOT EXISTS` guard is the only mechanism to enforce
    /// idempotency on a per-(room, prior-cin) basis.
    ///
    /// The 5-minute window is chosen for the realistic concurrent
    /// scenario: receptionist marks clean in iHOTEL + housekeeper marks
    /// clean on the mobile app within minutes of each other. Beyond
    /// 5 minutes the system treats subsequent marks as a legitimate
    /// re-clean (e.g. room re-occupied and cleaned again the same shift).
    #[test]
    fn ht_housewife_dedup_5_minute_window() {
        let prior = PriorOccupant {
            cin_no: "CH26-005159".into(),
            customer_full_name: "Jane Doe".into(),
        };
        let statements = build_statements(6, "306", "Admin", Some(&prior), pinned_now());
        let insert = &statements[1];
        // Must use INSERT ... SELECT ... WHERE NOT EXISTS (NOT INSERT ... VALUES).
        assert!(
            insert.starts_with("INSERT INTO HT_Housewife"),
            "must INSERT INTO HT_Housewife; got:\n{insert}"
        );
        assert!(
            insert.contains(" SELECT "),
            "must use INSERT … SELECT … WHERE NOT EXISTS guard, not VALUES; got:\n{insert}"
        );
        assert!(
            !insert.contains(" VALUES "),
            "must NOT use the legacy VALUES shape (no dedup window); got:\n{insert}"
        );
        // Dedup guard fires the WHERE NOT EXISTS subquery on (h_room, h_cin).
        assert!(
            insert.contains("WHERE NOT EXISTS"),
            "must guard with WHERE NOT EXISTS; got:\n{insert}"
        );
        assert!(
            insert.contains("FROM HT_Housewife"),
            "guard must scan HT_Housewife; got:\n{insert}"
        );
        assert!(
            insert.contains("h_room='306'"),
            "guard must match on h_room; got:\n{insert}"
        );
        assert!(
            insert.contains("h_cin='CH26-005159'"),
            "guard must match on h_cin (prior occupant); got:\n{insert}"
        );
        // 5-minute window.
        assert!(
            insert.contains("h_date > DATEADD(minute, -5, GETDATE())"),
            "guard must scope to the last 5 minutes via DATEADD(minute, -5, GETDATE()); got:\n{insert}"
        );
    }

    /// Track C — T5 HIGH-3: when the room has no prior occupant
    /// (brand-new room or fully-cancelled history), `h_cin=''`. The
    /// dedup window still applies — back-to-back day-one mark-cleans
    /// within 5 minutes should not double-log.
    #[test]
    fn ht_housewife_dedup_applies_even_when_no_prior_occupant() {
        let statements = build_statements(6, "306", "Admin", None, pinned_now());
        let insert = &statements[1];
        assert!(
            insert.contains("WHERE NOT EXISTS"),
            "guard must still fire when no prior occupant; got:\n{insert}"
        );
        assert!(
            insert.contains("h_cin=''"),
            "guard must compare h_cin to '' when no prior; got:\n{insert}"
        );
        assert!(
            insert.contains("h_date > DATEADD(minute, -5, GETDATE())"),
            "5-minute window must still apply; got:\n{insert}"
        );
    }
}
