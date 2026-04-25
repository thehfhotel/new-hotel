//! `ModifyBooking` recipe — spike `findings.md` §3c.
//!
//! **Critical departure from the legacy app.** Spike §3c documents that the
//! .NET app does a destructive DELETE-everything + RE-INSERT pattern (no
//! transaction). Per spike §3c recommendation:
//!
//! > **For our writeback**: we should **NOT** replicate this destructive
//! > pattern. Use targeted UPDATEs against `HT_Book_H` / `HT_Book_Ds` instead,
//! > and add/remove `HT_Book_Date` rows as needed. The legacy app reads what's
//! > in the tables, so our targeted UPDATEs are equivalent and safer.
//!
//! ## What we DO emit (safe, targeted)
//!
//! - `UPDATE HT_Book_H SET …` — only the changed fields (dates, customer info,
//!   notes). Keeps `book_room_type=2`, `Book_Notify_Day=3`, etc. untouched
//!   (they're set at create time per §3k).
//! - `UPDATE HT_Book_Ds SET …` — when stay range or price changed.
//! - `DELETE FROM HT_Book_Date WHERE Book_no=… AND Book_date_ds NOT IN (…)`
//!   — drop nights that no longer apply.
//! - `INSERT INTO HT_Book_Date …` — add new nights that didn't exist before.
//!
//! ## What we DO NOT emit (the legacy app's destructive pattern)
//!
//! - `DELETE FROM HT_Book_H WHERE Book_ID=…` — would lose the booking ID and
//!   the customer linkage if the subsequent INSERT failed.
//! - `DELETE FROM HT_Book_Ds WHERE Book_no=…` — same risk.
//! - Mass DELETE+REINSERT of `HT_Book_Date` — only diff what changed.
//!
//! Reference SQL (the destructive legacy pattern, for context only —
//! `booking-checkin-20260424-101838/writes.txt` lines 5-13):
//!
//! ```text
//! UPDATE HT_Customers SET ... WHERE Cust_no='C21610'                 -- preserved
//! UPDATE HT_Rooms SET room_book_*='', Room_Book='' WHERE room_book IN
//!     (SELECT id FROM ht_book_date WHERE Book_no='R014810')          -- preserved
//! DELETE FROM HT_Book_Date WHERE Book_no='R014810'                   -- ❌ skipped
//! DELETE FROM HT_Book_H    WHERE Book_ID='R014810'                   -- ❌ skipped
//! DELETE FROM HT_Book_Ds   WHERE Book_no='R014810'                   -- ❌ skipped
//! [INSERT all 3 tables again with the new dates]                     -- ❌ skipped
//! ```
//!
//! Our targeted approach achieves the same end state without the data-loss
//! window the legacy app's no-transaction implementation has (§3c warning).

use crate::outbox::intent::BookingChanges;
use crate::writeback::allocate::{allocate_book_date_id, LegacyConn};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    format_legacy_date, format_legacy_datetime, midnight_of, sql_quote,
};
use chrono::{DateTime, Days, NaiveDate, Utc};

/// Inputs for the modify-booking recipe — already-resolved legacy IDs +
/// the diff from the `BookingChanges` payload.
#[derive(Debug, Clone)]
pub struct ModifyBookingInputs<'a> {
    pub book_id: &'a str,
    /// Caller pre-allocates the FIRST id to use for any new HT_Book_Date INSERTs.
    /// If no nights are added, this is unused.
    pub book_date_id_base: i32,
    pub changes: &'a BookingChanges,
    /// Calendar nights the *new* stay range covers (after applying the change).
    /// Empty if `changes.new_stay` is None — we don't touch HT_Book_Date.
    pub new_nights_calendar: Vec<NaiveDate>,
}

/// Build the targeted-UPDATE statements. PURE — no I/O.
pub fn build_statements(inputs: &ModifyBookingInputs<'_>) -> Vec<String> {
    let book_id_q = sql_quote(inputs.book_id);
    let mut statements = Vec::new();

    // 1. UPDATE HT_Book_H — set only the changed fields. Builds the SET clause
    //    incrementally; if no header changes, we skip the UPDATE entirely.
    let mut header_sets: Vec<String> = Vec::new();
    if let Some(stay) = &inputs.changes.new_stay {
        // §3k: Book_Date_in/out on HT_Book_H render at midnight
        let in_q = sql_quote(&format_legacy_datetime(midnight_of(stay.start)));
        let out_q = sql_quote(&format_legacy_datetime(midnight_of(stay.end)));
        header_sets.push(format!("[Book_Date_in]={in_q}"));
        header_sets.push(format!("[Book_Date_out]={out_q}"));
    }
    if let Some(price) = &inputs.changes.new_price {
        let baht = (price.as_satang() as f64) / 100.0;
        header_sets.push(format!("[Book_Price_Total]={baht}"));
    }
    if let Some(phone) = &inputs.changes.new_customer_phone {
        let q = sql_quote(phone);
        header_sets.push(format!("[Book_Cust_Tel]={q}"));
    }
    if let Some(notes) = &inputs.changes.new_notes {
        let q = sql_quote(notes);
        header_sets.push(format!("[Book_room_note]={q}"));
    }
    if !header_sets.is_empty() {
        statements.push(format!(
            "UPDATE [HT_Book_H] SET {sets} WHERE Book_ID={book_id_q}",
            sets = header_sets.join(",")
        ));
    }

    // 2. UPDATE HT_Book_Ds — set only the changed detail fields.
    let mut ds_sets: Vec<String> = Vec::new();
    if let Some(stay) = &inputs.changes.new_stay {
        // HT_Book_Ds carries actual stay times — start as picked, end snapped
        // to 11:59:59 AM per spike §3b convention.
        let start_q = sql_quote(&format_legacy_datetime(stay.start));
        let end_actual = end_of_stay_at_almost_noon(stay.end);
        let end_q = sql_quote(&format_legacy_datetime(end_actual));
        let nights = inputs.new_nights_calendar.len().max(1) as i32;
        ds_sets.push(format!("[Book_Room_Start]={start_q}"));
        ds_sets.push(format!("[Book_Room_End]={end_q}"));
        ds_sets.push(format!("[Book_Room_Night]={nights}"));
    }
    if let Some(room_no) = &inputs.changes.new_room_no {
        // Misleading column name: stores the room NUMBER (per spike §3b).
        let q = sql_quote(room_no);
        ds_sets.push(format!("[Book_Room_Type]={q}"));
    }
    if let Some(price) = &inputs.changes.new_price {
        let baht = (price.as_satang() as f64) / 100.0;
        let nights = inputs.new_nights_calendar.len().max(1) as i32;
        ds_sets.push(format!("[Book_Room_Price]={baht}"));
        ds_sets.push(format!(
            "[Book_Room_PriceToTal]={total}",
            total = baht * nights as f64
        ));
    }
    if !ds_sets.is_empty() {
        statements.push(format!(
            "UPDATE [HT_Book_Ds] SET {sets} WHERE Book_No={book_id_q}",
            sets = ds_sets.join(",")
        ));
    }

    // 3. HT_Book_Date diff — only when stay range changed.
    if inputs.changes.new_stay.is_some() {
        // Drop nights that aren't in the new calendar
        let kept_dates_q = inputs
            .new_nights_calendar
            .iter()
            .map(|d| sql_quote(&format_legacy_date(*d)))
            .collect::<Vec<_>>()
            .join(",");
        if kept_dates_q.is_empty() {
            statements.push(format!(
                "DELETE FROM HT_Book_Date WHERE Book_no={book_id_q}"
            ));
        } else {
            statements.push(format!(
                "DELETE FROM HT_Book_Date WHERE Book_no={book_id_q} \
                 AND Book_date_ds NOT IN ({kept_dates_q})"
            ));
        }
        // Re-insert any nights that don't exist yet. We can't know what's in
        // MSSQL without a SELECT, so we use INSERT with NOT EXISTS to stay
        // idempotent.
        let room_no_for_inserts = inputs
            .changes
            .new_room_no
            .as_deref()
            .unwrap_or("");
        let room_no_q = sql_quote(room_no_for_inserts);
        for (i, day) in inputs.new_nights_calendar.iter().enumerate() {
            let id = inputs.book_date_id_base + i as i32;
            let date_q = sql_quote(&format_legacy_date(*day));
            statements.push(format!(
                "INSERT INTO [HT_Book_Date]([id],[Book_no],[Book_type],[Book_date_ds],\
                 [Book_Num],[Book_USE]) \
                 SELECT {id},{book_id_q},{room_no_q},{date_q},1,0 \
                 WHERE NOT EXISTS (SELECT 1 FROM HT_Book_Date \
                 WHERE Book_no={book_id_q} AND Book_date_ds={date_q})"
            ));
        }
    }

    statements
}

fn end_of_stay_at_almost_noon(stay_end: DateTime<Utc>) -> DateTime<Utc> {
    use chrono::TimeZone;
    let date = stay_end.date_naive();
    Utc.from_utc_datetime(
        &date
            .and_hms_opt(11, 59, 59)
            .expect("11:59:59 is a valid time"),
    )
}

/// Enumerate the new calendar nights for the modified stay range.
/// Self-contained per recipe spec.
fn enumerate_calendar_nights(
    stay_start: DateTime<Utc>,
    stay_end: DateTime<Utc>,
) -> Vec<NaiveDate> {
    let start = stay_start.date_naive();
    let end = stay_end.date_naive();
    let mut nights = Vec::new();
    let mut day = start;
    while day < end {
        nights.push(day);
        day = match day.checked_add_days(Days::new(1)) {
            Some(d) => d,
            None => break,
        };
        if nights.len() > 365 {
            break;
        }
    }
    if nights.is_empty() {
        nights.push(start);
    }
    nights
}

/// Execute the modify-booking recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
    changes: &BookingChanges,
) -> WritebackResult<LegacyIds> {
    // Only allocate book_date IDs if stay changed (we may insert new nights).
    let book_date_id_base = if changes.new_stay.is_some() {
        allocate_book_date_id(conn).await?
    } else {
        0 // unused
    };
    let new_nights = if let Some(stay) = &changes.new_stay {
        enumerate_calendar_nights(stay.start, stay.end)
    } else {
        Vec::new()
    };
    let inputs = ModifyBookingInputs {
        book_id,
        book_date_id_base,
        changes,
        new_nights_calendar: new_nights,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_book_id(book_id.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shared::{DateRange, Money};
    use chrono::TimeZone;

    #[test]
    fn no_destructive_delete_on_book_h_or_book_ds() {
        // Spike §3c rule: never DELETE from HT_Book_H / HT_Book_Ds — that's the
        // destructive pattern we explicitly skip.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 27, 12, 0, 0).unwrap(),
            )),
            new_room_no: Some("403".into()),
            new_room_type: None,
            new_price: Some(Money::from_baht(950)),
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
        };
        let statements = build_statements(&inputs);
        for s in &statements {
            let lower = s.to_ascii_lowercase();
            assert!(
                !lower.contains("delete from ht_book_h")
                    && !lower.contains("delete from ht_book_ds"),
                "destructive pattern leaked: {s}"
            );
        }
    }

    #[test]
    fn empty_changes_produces_no_statements() {
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
        };
        assert!(build_statements(&inputs).is_empty());
    }

    #[test]
    fn updates_book_h_dates_at_midnight_per_section_3k() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 14, 30, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 11, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
        };
        let s = build_statements(&inputs);
        let h = s.iter().find(|s| s.contains("[HT_Book_H]")).unwrap();
        assert!(h.contains("'4/25/2026 12:00:00 AM'"));
        assert!(h.contains("'4/26/2026 12:00:00 AM'"));
    }

    #[test]
    fn updates_book_ds_with_actual_stay_times_and_almost_noon_end() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
        };
        let s = build_statements(&inputs);
        let d = s.iter().find(|s| s.contains("[HT_Book_Ds]")).unwrap();
        assert!(d.contains("'4/25/2026 12:00:00 PM'"));
        assert!(d.contains("'4/26/2026 11:59:59 AM'"));
    }

    #[test]
    fn updates_room_phone_only_when_phone_changed() {
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: Some("0900000099".into()),
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
        };
        let s = build_statements(&inputs);
        assert_eq!(s.len(), 1);
        assert!(s[0].contains("[Book_Cust_Tel]='0900000099'"));
    }

    #[test]
    fn book_date_diff_uses_idempotent_insert() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            )),
            new_room_no: Some("402".into()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
        };
        let s = build_statements(&inputs);
        // The night INSERT should be guarded by NOT EXISTS for idempotency
        let insert = s
            .iter()
            .find(|s| s.contains("INSERT INTO [HT_Book_Date]"))
            .unwrap();
        assert!(insert.contains("WHERE NOT EXISTS"));
    }

    #[test]
    fn book_date_keeps_only_new_calendar_dates() {
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
        };
        let s = build_statements(&inputs);
        let del = s
            .iter()
            .find(|s| s.starts_with("DELETE FROM HT_Book_Date"))
            .unwrap();
        assert!(del.contains("NOT IN ('4/25/2026')"));
    }
}
