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

use chrono::Utc;
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
/// (or `None` if the room has never been occupied).
pub fn build_statements(
    room_id: i32,
    room_no: &str,
    by: &str,
    prior: Option<&PriorOccupant>,
) -> Vec<String> {
    let now_str = format_legacy_datetime(Utc::now());
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
        // 2. Audit row in HT_Housewife
        format!(
            "INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) \
             VALUES ({by_q}, {room_no_q}, {now_q}, '',{h_cin_q},{h_name_q})"
        ),
    ]
}

/// SELECT the prior **non-cancelled** occupant of `room_no`. Per spike §3j:
///
/// ```sql
/// SELECT TOP 1 h.Cin_no, c.Cust_name + ' ' + c.Cust_name2 AS h_cin_name
///   FROM HT_CheckIn_Ds d
///   JOIN HT_CheckIn_H h ON h.Cin_no = d.Cin_No
///   JOIN HT_Customers c ON c.Cust_no = h.Cin_cust_no
///  WHERE d.Cin_Room_No = @room_no
///    AND h.cin_status NOT IN ('ยกเลิก')
///  ORDER BY d.Cin_Room_Out DESC
/// ```
///
/// Returns `None` if no prior real occupant exists.
pub async fn fetch_prior_occupant(
    conn: &mut LegacyConn<'_>,
    room_no: &str,
) -> WritebackResult<Option<PriorOccupant>> {
    let room_no_q = sql_quote(room_no);
    let sql = format!(
        "SELECT TOP 1 h.Cin_no, ISNULL(c.Cust_name, '') + ' ' + ISNULL(c.Cust_name2, '') AS h_cin_name \
         FROM HT_CheckIn_Ds d \
         JOIN HT_CheckIn_H h ON h.Cin_no = d.Cin_No \
         JOIN HT_Customers c ON c.Cust_no = h.Cin_cust_no \
         WHERE d.Cin_Room_No = {room_no_q} \
           AND h.cin_status NOT IN (N'ยกเลิก') \
         ORDER BY d.Cin_Room_Out DESC"
    );
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
    let statements = build_statements(room_id_int, room_no, by, prior.as_ref());
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

    /// Matches `mark-clean-20260424-115026/writes.txt` lines 1-2 (modulo the
    /// dynamic timestamp which we render with `format_legacy_datetime(now)`).
    #[test]
    fn build_statements_matches_spike_capture_with_prior() {
        let prior = PriorOccupant {
            cin_no: "CH26-005159".into(),
            customer_full_name: "<REDACTED-real-guest-name>".into(),
        };
        let statements = build_statements(6, "306", "Admin", Some(&prior));
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

    #[test]
    fn build_statements_uses_empty_strings_when_no_prior_occupant() {
        let statements = build_statements(6, "306", "Admin", None);
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
        let statements = build_statements(50, "403", "Admin", None);
        assert!(statements[0].contains("where id=50"));
        assert!(!statements[0].contains("room_no"));
    }
}
