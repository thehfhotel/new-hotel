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

use crate::outbox::intent::{BookingChanges, CustomerResave};
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
    /// Room number on the booking *before* this modification. Used as a
    /// fallback when the user extends the stay without moving rooms — every
    /// new HT_Book_Date row needs `Book_type` populated with a room number,
    /// otherwise the booking disappears from the .NET app's calendar grid
    /// (audit HIGH-1). Resolved by `execute()` from HT_Book_Date.
    pub existing_room_no: Option<&'a str>,
}

/// Build the targeted-UPDATE statements. PURE — no I/O.
pub fn build_statements(inputs: &ModifyBookingInputs<'_>) -> Vec<String> {
    let book_id_q = sql_quote(inputs.book_id);
    let mut statements = Vec::new();

    // 0a. Customer re-save — spike §3c capture lines 5,16,28. The .NET app
    //     re-saves the customer record on every booking modify. Without it,
    //     phone/address edits don't propagate to the customer master.
    if let Some(resave) = inputs.changes.customer_resave.as_ref() {
        statements.push(build_customer_resave_update(resave));
    }

    // 0b. Clear stale HT_Rooms display fields BEFORE the date diff —
    //     spike §3c capture lines 6,14,17. Otherwise after a date change the
    //     calendar grid keeps stale "booked" captions.
    statements.push(format!(
        "update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' \
         where room_book in (select id from ht_book_date  where Book_no={book_id_q})"
    ));

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
        // HIGH-1 fix: when only dates change, fall back to the existing
        // room_no resolved from MSSQL. Empty Book_type would make the
        // booking disappear from the .NET app's calendar grid.
        let room_no_for_inserts = inputs
            .changes
            .new_room_no
            .as_deref()
            .or(inputs.existing_room_no)
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

        // Re-write the HT_Rooms display caption with the new dates — spike
        // §3c capture line 26. Mirrors `booking_create`'s caption format
        // (commit 0179f81). We need a customer name + room number to render
        // the caption; if either is missing we skip (the .NET app would also
        // skip in that case — phone/notes are optional).
        if let (Some(customer_name), Some(room_no), Some(stay)) = (
            inputs.changes.new_customer_name.as_deref(),
            inputs.changes.new_room_no.as_deref(),
            inputs.changes.new_stay.as_ref(),
        ) {
            let stay_in_short = crate::writeback::format::format_bangkok(stay.start, "%d/%m %H:%M");
            let stay_end_actual = end_of_stay_at_almost_noon(stay.end);
            let stay_out_short = crate::writeback::format::format_bangkok(stay_end_actual, "%d/%m %H:%M");
            let phone = inputs.changes.new_customer_phone.as_deref().unwrap_or("");
            let notes = inputs.changes.new_notes.as_deref().unwrap_or("");
            let room_book_ds_q = sql_quote(&format!(
                "{name}  {phone} เวลาเข้าพัก : 00:00  1. {room}  ({in_short}) ถึง ({out_short})  หมายเหตุ : {notes} ",
                name = customer_name,
                phone = phone,
                room = room_no,
                in_short = stay_in_short,
                out_short = stay_out_short,
                notes = notes,
            ));
            let first_book_date_id = inputs.book_date_id_base;
            let room_book_q = sql_quote(&first_book_date_id.to_string());
            let room_book_name_q = sql_quote(customer_name);
            let room_book_time_q = sql_quote("00:00");
            let room_no_q = sql_quote(room_no);
            statements.push(format!(
                "update HT_Rooms set room_book_ds={room_book_ds_q}, Room_Book={room_book_q},\
                 Room_Book_Name={room_book_name_q},Room_Book_Time={room_book_time_q} \
                 where room_no={room_no_q}"
            ));
        }
    }

    statements
}

/// Build the `UPDATE [HT_Customers] SET ... WHERE Cust_no=…` statement that
/// re-saves the customer record on every booking modify. Spike §3c capture
/// line 28 — the .NET app writes the full address/work field set even when
/// only the phone changed. We mirror that for parity (NULLs would also be
/// safe but empty strings match the WinForms-friendly default).
fn build_customer_resave_update(r: &CustomerResave) -> String {
    let cust_no_q = sql_quote(&r.legacy_cust_no);
    format!(
        "UPDATE [HT_Customers] SET  [Cust_name]={name},[Cust_name2]={name2},\
         [Cust_Type]={ctype},[Cust_Type_main]={ctype_main},[Cust_Email]={email},\
         [Cust_Add_no]={add_no},[Cust_Add_moo]={add_moo},[Cust_Add_soi]={add_soi},\
         [Cust_Add_road]={add_road},[Cust_Add_tambon]={add_tambon},\
         [Cust_Add_ampore]={add_ampore},[Cust_Add_province]={add_province},\
         [Cust_Add_code]={add_code},[Cust_Add_tel]={add_tel},[Cust_Add_fax]={add_fax},\
         [Cust_Work_Name]={work_name},[Cust_Work_no]={work_no},[Cust_Work_moo]={work_moo},\
         [Cust_Work_soi]={work_soi},[Cust_Work_road]={work_road},\
         [Cust_Work_tambon]={work_tambon},[Cust_Work_ampore]={work_ampore},\
         [Cust_Work_province]={work_province},[Cust_Work_code]={work_code},\
         [Cust_Work_tel]={work_tel},[Cust_Work_fax]={work_fax} \
         WHERE Cust_no={cust_no_q}",
        name = sql_quote(&r.cust_name),
        name2 = sql_quote(&r.cust_name2),
        ctype = sql_quote(&r.cust_type),
        ctype_main = sql_quote(&r.cust_type_main),
        email = sql_quote(&r.cust_email),
        add_no = sql_quote(&r.cust_add_no),
        add_moo = sql_quote(&r.cust_add_moo),
        add_soi = sql_quote(&r.cust_add_soi),
        add_road = sql_quote(&r.cust_add_road),
        add_tambon = sql_quote(&r.cust_add_tambon),
        add_ampore = sql_quote(&r.cust_add_ampore),
        add_province = sql_quote(&r.cust_add_province),
        add_code = sql_quote(&r.cust_add_code),
        add_tel = sql_quote(&r.cust_add_tel),
        add_fax = sql_quote(&r.cust_add_fax),
        work_name = sql_quote(&r.cust_work_name),
        work_no = sql_quote(&r.cust_work_no),
        work_moo = sql_quote(&r.cust_work_moo),
        work_soi = sql_quote(&r.cust_work_soi),
        work_road = sql_quote(&r.cust_work_road),
        work_tambon = sql_quote(&r.cust_work_tambon),
        work_ampore = sql_quote(&r.cust_work_ampore),
        work_province = sql_quote(&r.cust_work_province),
        work_code = sql_quote(&r.cust_work_code),
        work_tel = sql_quote(&r.cust_work_tel),
        work_fax = sql_quote(&r.cust_work_fax),
    )
}

fn end_of_stay_at_almost_noon(stay_end: DateTime<Utc>) -> DateTime<Utc> {
    use chrono::TimeZone;
    use chrono_tz::Asia::Bangkok;
    let bkk_date = stay_end.with_timezone(&Bangkok).date_naive();
    let bkk_target = bkk_date
        .and_hms_opt(11, 59, 59)
        .expect("11:59:59 is a valid time");
    Bangkok
        .from_local_datetime(&bkk_target)
        .single()
        .expect("11:59:59 is unambiguous in Asia/Bangkok (no DST)")
        .with_timezone(&Utc)
}

/// Enumerate the new calendar nights for the modified stay range.
/// Self-contained per recipe spec. Uses Bangkok calendar day boundaries —
/// see `format::bangkok_date` for why UTC `date_naive()` would be wrong.
fn enumerate_calendar_nights(
    stay_start: DateTime<Utc>,
    stay_end: DateTime<Utc>,
) -> Vec<NaiveDate> {
    let start = crate::writeback::format::bangkok_date(stay_start);
    let end = crate::writeback::format::bangkok_date(stay_end);
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
    // If the stay is changing without moving rooms, we need the current
    // room_no for the new HT_Book_Date INSERTs (audit HIGH-1).
    let existing_room_no = if changes.new_stay.is_some() && changes.new_room_no.is_none() {
        fetch_existing_room_no(conn, book_id).await?
    } else {
        None
    };
    let inputs = ModifyBookingInputs {
        book_id,
        book_date_id_base,
        changes,
        new_nights_calendar: new_nights,
        existing_room_no: existing_room_no.as_deref(),
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_book_id(book_id.to_string()))
}

/// Look up the current room_no on the booking by reading any existing
/// `HT_Book_Date.Book_type` row. Returns `None` if no rows match (e.g.
/// brand-new booking that hasn't allocated its first night yet).
async fn fetch_existing_room_no(
    conn: &mut LegacyConn<'_>,
    book_id: &str,
) -> WritebackResult<Option<String>> {
    let book_id_q = sql_quote(book_id);
    let sql = format!(
        "SELECT TOP 1 Book_type FROM HT_Book_Date \
         WHERE Book_no={book_id_q} AND Book_type IS NOT NULL AND Book_type<>'' \
         ORDER BY id DESC"
    );
    let stream = conn.simple_query(sql).await?;
    let row = stream.into_row().await?;
    match row {
        Some(r) => Ok(r.get::<&str, _>(0).map(|s| s.to_string())),
        None => Ok(None),
    }
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
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
            existing_room_no: None,
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
    fn empty_changes_only_emits_room_book_clear() {
        // Spike §3c capture lines 6/14/17: the .NET app fires the
        // `update HT_Rooms set room_book_*=''` clear on EVERY save, even
        // when no fields change. Mirroring keeps the calendar grid in sync.
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        assert_eq!(s.len(), 1);
        assert!(s[0].starts_with("update HT_Rooms set room_book_ds=''"));
        assert!(s[0].contains("Book_no='R014810'"));
    }

    #[test]
    fn book_date_inserts_use_existing_room_no_when_only_dates_change() {
        // HIGH-1 regression: previously, when new_stay was Some but
        // new_room_no was None, every HT_Book_Date INSERT was issued with
        // Book_type='', making the booking invisible in the .NET app's
        // calendar grid.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 27, 5, 0, 0).unwrap(),
            )),
            new_room_no: None, // user only changed dates, not the room
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014812",
            book_date_id_base: 47301,
            changes: &changes,
            new_nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ],
            existing_room_no: Some("402"),
        };
        let s = build_statements(&inputs);
        let inserts: Vec<_> = s.iter().filter(|x| x.contains("INSERT INTO [HT_Book_Date]")).collect();
        assert_eq!(inserts.len(), 2, "expected one INSERT per night");
        for stmt in &inserts {
            assert!(
                stmt.contains(",'402',"),
                "Book_type must use existing_room_no, got: {stmt}"
            );
            assert!(
                !stmt.contains(",'',"),
                "Book_type must NOT be empty (HIGH-1): {stmt}"
            );
        }
    }

    #[test]
    fn book_date_inserts_use_new_room_no_when_room_changed() {
        // When the user moves rooms, new_room_no wins over existing_room_no.
        let changes = BookingChanges {
            new_stay: Some(DateRange::new(
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            )),
            new_room_no: Some("510".to_string()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014812",
            book_date_id_base: 47302,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: Some("402"), // would be the wrong choice
        };
        let s = build_statements(&inputs);
        let insert = s.iter().find(|x| x.contains("INSERT INTO [HT_Book_Date]")).unwrap();
        assert!(insert.contains(",'510',"), "got: {insert}");
        assert!(!insert.contains(",'402',"));
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
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
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
                // 5 AM UTC = noon Bangkok.
                Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        let d = s.iter().find(|s| s.contains("[HT_Book_Ds]")).unwrap();
        assert!(d.contains("'4/25/2026 12:00:00 PM'"));
        assert!(d.contains("'4/26/2026 11:59:59 AM'"));
    }

    #[test]
    fn updates_book_h_with_phone_when_phone_changed() {
        // After fix #7 we always emit the room_book clear (statement 0). The
        // phone change adds the [Book_Cust_Tel] UPDATE on HT_Book_H.
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: Some("0900000099".into()),
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        // Statement 0: room_book clear; statement 1: HT_Book_H UPDATE.
        assert_eq!(s.len(), 2);
        let book_h = s.iter().find(|s| s.contains("[HT_Book_H]")).unwrap();
        assert!(book_h.contains("[Book_Cust_Tel]='0900000099'"));
    }

    #[test]
    fn clear_room_book_display_fires_before_date_diff() {
        // Spike §3c capture lines 6,14,17 — the clear must come BEFORE any
        // HT_Book_Date INSERT, otherwise the .NET calendar grid keeps stale
        // captions briefly even after a date change.
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
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        let clear_idx = s
            .iter()
            .position(|s| s.starts_with("update HT_Rooms set room_book_ds=''"))
            .unwrap();
        let date_insert_idx = s
            .iter()
            .position(|s| s.contains("INSERT INTO [HT_Book_Date]"))
            .unwrap();
        assert!(clear_idx < date_insert_idx, "clear must precede date INSERT");
    }

    #[test]
    fn customer_resave_emits_full_field_update() {
        // Spike §3c capture line 28 — the .NET app re-saves the full address
        // + work field set on every booking modify (most blanks).
        let resave = CustomerResave {
            legacy_cust_no: "C21610".into(),
            cust_name: "SPIKE TEST WALKIN".into(),
            cust_type: "ราคาปกติ".into(),
            cust_type_main: "บุคคลธรรมดา".into(),
            cust_add_tel: "0900000088".into(),
            ..CustomerResave::default()
        };
        let changes = BookingChanges {
            new_stay: None,
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: Some(resave),
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 0,
            changes: &changes,
            new_nights_calendar: vec![],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        let upd = s
            .iter()
            .find(|s| s.starts_with("UPDATE [HT_Customers]"))
            .expect("customer re-save UPDATE must be emitted when payload set");
        assert!(upd.contains("[Cust_name]='SPIKE TEST WALKIN'"));
        assert!(upd.contains("[Cust_Type_main]='บุคคลธรรมดา'"));
        assert!(upd.contains("[Cust_Add_tel]='0900000088'"));
        assert!(upd.contains("WHERE Cust_no='C21610'"));
        // The customer re-save must come BEFORE the room_book clear.
        let resave_idx = s
            .iter()
            .position(|s| s.starts_with("UPDATE [HT_Customers]"))
            .unwrap();
        let clear_idx = s
            .iter()
            .position(|s| s.starts_with("update HT_Rooms"))
            .unwrap();
        assert!(resave_idx < clear_idx);
    }

    #[test]
    fn caption_rewrite_uses_new_dates_when_full_context_supplied() {
        // Spike §3c capture line 26: re-write HT_Rooms display caption with
        // the new dates after a modify. Mirrors `booking_create`'s format.
        let stay = DateRange::new(
            Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
        );
        let changes = BookingChanges {
            new_stay: Some(stay),
            new_room_no: Some("402".into()),
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: Some("vip".into()),
            new_customer_phone: Some("0900000088".into()),
            new_customer_name: Some("SPIKE TEST WALKIN".into()),
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        // Caption rewrite is the LAST statement.
        let caption = s
            .iter()
            .rev()
            .find(|s| s.contains("Room_Book_Name="))
            .expect("caption rewrite must be emitted when full context set");
        assert!(caption.contains("'SPIKE TEST WALKIN'"));
        assert!(caption.contains("where room_no='402'"));
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
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
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
            new_customer_name: None,
            customer_resave: None,
        };
        let inputs = ModifyBookingInputs {
            book_id: "R014810",
            book_date_id_base: 47300,
            changes: &changes,
            new_nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            existing_room_no: None,
        };
        let s = build_statements(&inputs);
        let del = s
            .iter()
            .find(|s| s.starts_with("DELETE FROM HT_Book_Date"))
            .unwrap();
        assert!(del.contains("NOT IN ('4/25/2026')"));
    }
}
