//! `CancelBooking` recipe — spike `findings.md` §3g-bis.
//!
//! Clean cancel: 4 UPDATEs + 1 DELETE. The customer + booking shells are
//! preserved; only `HT_Book_Date` rows are hard-deleted (frees the room).
//!
//! Reference SQL (verbatim from the spike capture
//! `booking-cancel-20260424-103158/writes.txt` lines 6-10):
//!
//! ```text
//! update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time=''
//!     where room_book in (select id from ht_book_date where Book_no='R014811')
//! update HT_Book_H set Book_Status='ยกเลิก' where Book_ID='R014811'
//! update HT_Book_ds set Book_status=3        where Book_No='R014811'
//! delete from HT_Book_Date                   where Book_no='R014811'
//! update HT_Book_H set book_status='ยกเลิก'  where book_id='R014811'   -- duplicate
//! ```
//!
//! Note the duplicated `UPDATE HT_Book_H ... book_status='ยกเลิก'` at the end
//! — it appears in the legacy capture and we preserve it. It's harmless (sets
//! the same value) and matters for byte-for-byte parity if anyone diff-checks
//! against the .NET app's output.
//!
//! Spike §3g-bis quote: *"This is the cleanest flow we've captured."*

use crate::writeback::allocate::LegacyConn;
use crate::writeback::constants::{BOOK_DS_STATUS_CANCELLED, BOOK_STATUS_CANCELLED};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

/// Build the 5 statements that cancel a booking. PURE — no I/O.
///
/// `book_id` is the legacy `HT_Book_H.Book_ID` (e.g. `"R014810"`).
pub fn build_statements(book_id: &str) -> Vec<String> {
    let book_id_q = sql_quote(book_id);
    vec![
        // 1. Clear the room "booked" display (subquery on HT_Book_Date)
        format!(
            "update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' \
             where room_book in (select id from ht_book_date  where Book_no={book_id_q})"
        ),
        // 2. Soft-cancel HT_Book_H (varchar status)
        format!(
            "update HT_Book_H set Book_Status={status} where Book_ID={book_id_q}",
            status = sql_quote(BOOK_STATUS_CANCELLED),
        ),
        // 3. Soft-cancel HT_Book_Ds (numeric status; column is mixed-case `Book_status`)
        format!(
            "update HT_Book_ds set Book_status={status} where Book_No={book_id_q}",
            status = BOOK_DS_STATUS_CANCELLED,
        ),
        // 4. Hard-delete HT_Book_Date — frees the room nights.
        //
        // Functionally equivalent to the legacy app's per-room form
        // (verified from /tmp/legacy-events-full.log):
        //   delete from HT_Book_Date where book_type='509' and book_no='R014835'
        //   delete from HT_Book_Date where book_type='V.201' and book_no='R014833'
        // The .NET app emits one DELETE per room because its UI tracks
        // them individually; ours doesn't carry that info today, so
        // the bulk DELETE removes all rows for the booking in one
        // statement. Per-room emission is a TODO when multi-room
        // tracking lands; the end state is identical either way.
        //
        // ⚠️ INTENTIONAL: `delete from  HT_Book_Date` has TWO spaces between
        // `from` and the table name. This is verbatim from the spike capture
        // (`booking-cancel-20260424-103158/writes.txt:9`) and pinned by the
        // byte-parity test below. Do NOT normalize to a single space —
        // future formatter / clippy autofixes that "clean up" the whitespace
        // would break the parity assertion and silently fork our SQL from
        // the legacy .NET app's emitted form. Wave 6 LOW item 3 documents
        // this pin.
        format!("delete from  HT_Book_Date where Book_no={book_id_q}"),
        // 5. Duplicate UPDATE HT_Book_H (lowercase column names — preserved for parity)
        format!(
            "update HT_Book_H set book_status={status} where book_id={book_id_q}",
            status = sql_quote(BOOK_STATUS_CANCELLED),
        ),
    ]
}

/// Execute the cancel-booking recipe against a live MSSQL connection.
pub async fn execute(conn: &mut LegacyConn<'_>, book_id: &str) -> WritebackResult<LegacyIds> {
    let statements = build_statements(book_id);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_book_id(book_id.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compares emitted SQL against the spike capture
    /// `booking-cancel-20260424-103158/writes.txt` (lines 6-10).
    #[test]
    fn build_statements_matches_spike_capture() {
        let statements = build_statements("R014811");
        assert_eq!(statements.len(), 5);
        assert_eq!(
            statements[0],
            "update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' \
             where room_book in (select id from ht_book_date  where Book_no='R014811')"
        );
        assert_eq!(
            statements[1],
            "update HT_Book_H set Book_Status='ยกเลิก' where Book_ID='R014811'"
        );
        assert_eq!(statements[2], "update HT_Book_ds set Book_status=3 where Book_No='R014811'");
        assert_eq!(statements[3], "delete from  HT_Book_Date where Book_no='R014811'");
        assert_eq!(
            statements[4],
            "update HT_Book_H set book_status='ยกเลิก' where book_id='R014811'"
        );
    }

    #[test]
    fn embedded_quotes_in_book_id_are_escaped() {
        let statements = build_statements("R'014811");
        assert!(statements[1].contains("'R''014811'"));
    }
}
