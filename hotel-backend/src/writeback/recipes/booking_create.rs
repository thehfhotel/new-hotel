//! `CreateBooking` recipe — spike `findings.md` §3b + visibility rules §3k.
//!
//! 4 INSERTs (5 if the customer is new). Allocates `Book_ID` (and `Cust_no` if
//! needed) under TABLOCKX. Critical: every booking must have the §3k fields
//! set correctly or it will be invisible in the .NET app's booking list.
//!
//! Reference SQL (verbatim from `booking-checkin-20260424-101838/writes.txt`
//! lines 1-4 + spike §3k):
//!
//! ```text
//! 1. INSERT INTO [HT_Customers] (id, Cust_no, Cust_perfix, Cust_name, ...)
//!    -- skipped if customer already exists
//!
//! 2. INSERT INTO [HT_Book_H](
//!      Book_ID='R014810', Book_Date=now, Book_Cust_ID='C21610',
//!      Book_Cust_Name='SPIKE TEST WALKIN', Book_Cust_Name2='',
//!      Book_Cust_Tel='0900000088',
//!      Book_Price_Total=890, Book_Price_Pay=0,
//!      Book_Status='', Book_Date_in='4/25/2026 12:00:00 AM' (midnight per §3k),
//!      Book_Date_out='4/26/2026 12:00:00 AM' (midnight per §3k),
//!      Book_by='Admin', Book_room_all='' (§3k),
//!      Book_room_note='', book_room_type=2 (§3k visibility!),
//!      Book_Notify_Day=3 (§3k default), Book_Notify_Note='', Book_Sale=''
//!    )
//!
//! 3. INSERT INTO [HT_Book_Ds](Book_No='R014810', Book_Room_Type='402'
//!    -- ⚠️ stores room NUMBER not type!
//!    , Book_Room_Start='4/25/2026 12:00:00 PM' (NOT midnight — actual stay)
//!    , Book_Room_End='4/26/2026 11:59:59 AM',
//!    Book_Room_Price=890, Book_Room_Night=1, Book_Room_Num=1,
//!    Book_Room_PriceToTal=890, Book_Room_Note='', Book_status=1 (§3k active)
//!    )
//!    -- HT_Book_Ds.id is IDENTITY, omit from column list
//!
//! 4..N. INSERT INTO [HT_Book_Date] for each calendar night
//!    (id, Book_no, Book_type='402', Book_date_ds='4/25/2026', Book_Num=1, Book_USE=0)
//! ```
//!
//! Spike §3k visibility checklist (all required for the .NET booking list):
//! | Field | Must be | Why |
//! |---|---|---|
//! | `book_room_type` | `2` | by-room-number; the list filters out `=1` |
//! | `Book_status` (HT_Book_Ds) | `1` | active; cancelled = 3 |
//! | `Book_Date_in/out` (HT_Book_H) | date-only (`'M/D/YYYY'`) | top-of-form date display; verified from /tmp/legacy-events-full.log R014820..R014824 |
//! | `Book_room_all` | `''` | unused header field; must be empty |
//! | `Book_Notify_Day` | `3` | .NET app default |
//! | All optional varchars | `''` | NULL crashes WinForms downstream |
//!
//! Departure on `HT_Book_Ds.Book_Room_End` is `'11:59:59 AM'` — convenient
//! for date-range BETWEEN queries (per spike §3b).

use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::outbox::intent::CreateBookingPayload;
use crate::writeback::allocate::{
    allocate_book_date_id, allocate_book_id, allocate_cust_no, allocate_customer_id,
    allocate_room_status_id, LegacyConn,
};
use crate::writeback::constants::{
    BOOK_DS_STATUS_ACTIVE, BOOK_NOTIFY_DAY_DEFAULT, BOOK_ROOM_TYPE_BY_ROOM_NUMBER,
    BOOK_STATUS_BOOKED, CUST_TYPE_MAIN_NORMAL, CUST_TYPE_NORMAL, DEFAULT_OPERATOR,
    ROOM_STATUS_RESERVED,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{
    date_to_ole_serial, end_of_stay_at_almost_noon, enumerate_calendar_nights, format_legacy_date,
    format_legacy_datetime, sql_quote,
};

/// Inputs for the booking-create recipe.
#[derive(Debug, Clone)]
pub struct CreateBookingInputs<'a> {
    pub book_id: &'a str,
    pub cust_no: &'a str,
    pub customer_id_int: Option<i32>,
    pub customer_name: &'a str,
    pub customer_phone: Option<&'a str>,
    pub created_by: &'a str,
    pub notes: Option<&'a str>,
    pub stay_start: DateTime<Utc>,
    pub stay_end: DateTime<Utc>,
    pub room_no: &'a str,
    pub price_baht: f64,
    /// Deposit (`เงินมัดจำ`) the receptionist entered on the form. Lands
    /// in `HT_Book_H.Book_Price_Pay`. Zero means no upfront payment.
    pub deposit_baht: f64,
    pub nights: i32,
    /// First `HT_Book_Date.id` to use for night INSERTs.
    pub book_date_id_base: i32,
    /// First `HT_Room_Status.id` to use for the per-night occupancy-ledger
    /// INSERTs (H7 fix — without these rows the booking is invisible in the
    /// .NET app's calendar grid AND `checkin_to_booking`'s night-0 UPDATE
    /// silently matches 0 rows).
    pub room_status_id_base: i32,
    /// Each booking covers nights[start..end). `nights` enumerates them in
    /// calendar order — recipe assigns `book_date_id_base + i`.
    pub nights_calendar: Vec<NaiveDate>,
    /// Whether the customer is new (recipe will INSERT HT_Customers if so).
    pub customer_is_new: bool,
}

/// Build the statements for a booking. PURE — no I/O.
pub fn build_statements(inputs: &CreateBookingInputs<'_>) -> Vec<String> {
    let book_id_q = sql_quote(inputs.book_id);
    let cust_no_q = sql_quote(inputs.cust_no);
    let now_q = sql_quote(&format_legacy_datetime(Utc::now()));
    let by_q = sql_quote(inputs.created_by);
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_phone_q = sql_quote(inputs.customer_phone.unwrap_or(""));
    let notes_q = sql_quote(inputs.notes.unwrap_or(""));
    let cust_type_q = sql_quote(CUST_TYPE_NORMAL);
    let room_no_q = sql_quote(inputs.room_no);

    // §3k: HT_Book_H carries date-only forms (verified from
    // /tmp/legacy-events-full.log captures of R014820..R014824 — every
    // row emits `'4/25/2026'`-style dates, NOT midnight datetimes).
    // HT_Book_Ds keeps the actual stay times.
    let stay_start_date_q = sql_quote(&format_legacy_date(
        crate::writeback::format::bangkok_date(inputs.stay_start),
    ));
    let stay_end_date_q = sql_quote(&format_legacy_date(
        crate::writeback::format::bangkok_date(inputs.stay_end),
    ));

    // HT_Book_Ds: actual stay times. Departure is hardcoded 11:59:59 AM per spike §3b.
    let stay_start_actual_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_actual = end_of_stay_at_almost_noon(inputs.stay_end);
    let stay_end_actual_q = sql_quote(&format_legacy_datetime(stay_end_actual));

    let mut statements: Vec<String> = Vec::with_capacity(4 + inputs.nights_calendar.len());

    // 1. HT_Customers (only if new). 30 columns in canonical legacy
    //    order — verified from /tmp/legacy-events-full.log (12 captured
    //    INSERT INTO [HT_Customers] rows across Cust_no C21624..C21634).
    //    Drops the obsolete `Cust_sex`, `Cust_IDcard`, `Cust_Contry`,
    //    `Cust_Work_Tax` columns the audit identified; adds the
    //    `[Cust_Last_Change]` date emitted by every legacy INSERT.
    //    `[Cust_Type_Main]` value is `'ราคาปกติ'` (NOT
    //    `'บุคคลธรรมดา'`) — that latter form is only emitted by the
    //    UPDATE path, never INSERT.
    if inputs.customer_is_new {
        let id = inputs.customer_id_int.unwrap_or(0);
        let cust_type_main_q = sql_quote(CUST_TYPE_MAIN_NORMAL);
        let last_change_q = sql_quote(&format_legacy_date(
            crate::writeback::format::bangkok_date(Utc::now()),
        ));
        statements.push(format!(
            "INSERT INTO [HT_Customers]([id],[Cust_no],[Cust_perfix],[Cust_name],[Cust_name2],\
             [Cust_Type],[Cust_Email],[Cust_Add_no],[Cust_Add_moo],[Cust_Add_soi],[Cust_Add_road],\
             [Cust_Add_tambon],[Cust_Add_ampore],[Cust_Add_province],[Cust_Add_code],\
             [Cust_Add_tel],[Cust_Add_fax],[Cust_Work_Name],[Cust_Work_no],[Cust_Work_moo],\
             [Cust_Work_soi],[Cust_Work_road],[Cust_Work_tambon],[Cust_Work_ampore],\
             [Cust_Work_province],[Cust_Work_code],[Cust_Work_tel],[Cust_Work_fax],\
             [Cust_Last_Change],[Cust_Type_Main]) VALUES ({id},{cust_no_q},'',{cust_name_q},'',\
             {cust_type_q},'','','','','','','','','',{cust_phone_q},'','','','','','','','','',\
             '','','',{last_change_q},{cust_type_main_q})"
        ));
    }

    // 2. HT_Book_H — 17-col canonical legacy order. Verified from
    //    /tmp/legacy-events-full.log captures of R014820..R014824:
    //    drops `[Book_Notify_Note]`; emits `Book_Notify_Day,Book_sale`
    //    WITHOUT square brackets (the .NET app's column-list builder
    //    apparently switches off brackets for the trailing two cols);
    //    `Book_Date_in/out` are date-only (`'4/25/2026'`).
    //    Book_Status='จอง' is the future-date-room-view visibility
    //    key.
    let booked_q = sql_quote(BOOK_STATUS_BOOKED);
    statements.push(format!(
        "INSERT INTO [HT_Book_H]( [Book_ID],[Book_Date],[Book_Cust_ID],[Book_Cust_Name],\
         [Book_Cust_Name2],[Book_Cust_Tel],[Book_Price_Total],[Book_Price_Pay],[Book_Status],\
         [Book_Date_in],[Book_Date_out],[Book_by],[Book_room_all],[Book_room_note],\
         [book_room_type],Book_Notify_Day,Book_sale)\
         VALUES( {book_id_q},{now_q},{cust_no_q},{cust_name_q},'',{cust_phone_q},\
         {price_2dp},{deposit_2dp},{status},{stay_in},{stay_out},{by_q},'',{notes_q},{room_type_code},\
         {notify_day},'')",
        price_2dp = format!("{:.2}", inputs.price_baht),
        deposit_2dp = format!("{:.2}", inputs.deposit_baht),
        status = booked_q,
        stay_in = stay_start_date_q,
        stay_out = stay_end_date_q,
        room_type_code = BOOK_ROOM_TYPE_BY_ROOM_NUMBER,
        notify_day = BOOK_NOTIFY_DAY_DEFAULT,
    ));

    // 3. HT_Book_Ds — Book_Room_Type stores ROOM NUMBER (per §3b finding)
    //    id is IDENTITY → omit from column list.
    //
    // Wave 6 LOW item 4: money values rendered with 2dp for consistency with
    // the HT_Book_H price columns and the HT_Receipt_H formatting (Wave 2
    // H4 / `money_2dp`). The legacy app emits `890` (no decimals) here, but
    // the columns are float-typed so `890.00` is operationally identical;
    // pinning the unified shape keeps recipes mutually consistent.
    let book_ds_price_2dp = format!("{:.2}", inputs.price_baht);
    let book_ds_total_2dp = format!("{:.2}", inputs.price_baht * inputs.nights as f64);
    statements.push(format!(
        "INSERT INTO [HT_Book_Ds]([Book_No],[Book_Room_Type],[Book_Room_Start],[Book_Room_End],\
         [Book_Room_Price],[Book_Room_Night],[Book_Room_Num],[Book_Room_PriceToTal],\
         [Book_Room_Note],[Book_status])VALUES({book_id_q},{room_no_q},{start},{end},\
         {price},{nights},1,{total},'',{status})",
        start = stay_start_actual_q,
        end = stay_end_actual_q,
        price = book_ds_price_2dp,
        nights = inputs.nights,
        total = book_ds_total_2dp,
        status = BOOK_DS_STATUS_ACTIVE,
    ));

    // 4..N. HT_Book_Date — one row per calendar night.
    //
    // We do NOT emit `update HT_Book_Date set Book_ok = Book_ok + 1` here.
    // That bump was previously added based on a misread of spike capture
    // booking-checkin/writes.txt:27 — but that capture was from a
    // CHECK-IN flow, not a fresh booking-create. The legacy-monitor
    // capture of R014836 (a clean future booking-create on 2026-04-25)
    // confirms: legacy emits zero Book_ok bumps for fresh creates. The
    // bump belongs in the check-in recipe path.
    for (i, day) in inputs.nights_calendar.iter().enumerate() {
        let id = inputs.book_date_id_base + i as i32;
        let date_q = sql_quote(&format_legacy_date(*day));
        statements.push(format!(
            "INSERT INTO [HT_Book_Date]([id],[Book_no],[Book_type],[Book_date_ds],[Book_Num],\
             [Book_USE])VALUES({id},{book_id_q},{room_no_q},{date_q},1,0)"
        ));
    }

    // N+1..M. HT_Room_Status — per-room-per-day occupancy-ledger rows for each
    //   booked night. H7 fix: without these rows the .NET app's calendar grid
    //   shows the booking as empty (the grid query filters
    //   `where (room_status='จอง' or room_status='เข้าพัก')`), AND when a
    //   check-in is later created against this booking, `checkin_to_booking`'s
    //   night-0 UPDATE matches 0 rows silently.
    //   Per `COMPAT_CHEATSHEET.md` line 347: status='จอง', room_Book_No=Book_ID.
    //   room_CheckIn_No is empty until the booking converts to a check-in
    //   (mirrors legacy app's FrmBookRooms emit shape).
    let room_status_q = sql_quote(ROOM_STATUS_RESERVED);
    let cust_name_for_status_q = sql_quote(inputs.customer_name);
    for (i, day) in inputs.nights_calendar.iter().enumerate() {
        let id = inputs.room_status_id_base + i as i32;
        let date_q = sql_quote(&format_legacy_date(*day));
        let oa = date_to_ole_serial(*day) as i64;
        statements.push(format!(
            "INSERT INTO [HT_Room_Status]([id],[room_no],[room_date],[room_status],\
             [room_Details],[room_Book_No],[room_CheckIn_No],[room_date_oa])\
             VALUES({id},{room_no_q},{date_q},{room_status_q},{cust_name_for_status_q},\
             {book_id_q},'',{oa})"
        ));
    }

    // N+1. HT_Rooms display fields — only set when the booking STARTS TODAY
    // (Bangkok). The .NET app's room-list view (the "rooms now booked" panel)
    // reads HT_Rooms.Room_Book / room_book_ds to figure out which rooms are
    // currently claimed; setting them for a future booking makes the room
    // appear booked TODAY in that panel.
    //
    // Verified against legacy app capture (R014836, future booking 4/28-4/29
    // created on 4/25): legacy emits ZERO HT_Rooms statements for future
    // bookings — only HT_Book_H + HT_Book_Ds + HT_Book_Date. The room_book
    // caption gets populated later (presumably by the day-of housekeeping
    // refresh OR by the check-in flow itself). Our recipe was unconditionally
    // setting it, which corrupted the room-list view per the 2026-04-25
    // R014835 incident.
    let bkk_today = Utc::now().with_timezone(&chrono_tz::Asia::Bangkok).date_naive();
    let bkk_stay_start = inputs.stay_start.with_timezone(&chrono_tz::Asia::Bangkok).date_naive();
    if bkk_stay_start <= bkk_today {
        let stay_in_short = inputs
            .stay_start
            .with_timezone(&chrono_tz::Asia::Bangkok)
            .format("%d/%m %H:%M")
            .to_string();
        let stay_out_short = stay_end_actual
            .with_timezone(&chrono_tz::Asia::Bangkok)
            .format("%d/%m %H:%M")
            .to_string();
        let room_book_ds_q = sql_quote(&format!(
            "{name}  {phone} เวลาเข้าพัก : 00:00  1. {room}  ({in_short}) ถึง ({out_short})  หมายเหตุ : {notes} ",
            name = inputs.customer_name,
            phone = inputs.customer_phone.unwrap_or(""),
            room = inputs.room_no,
            in_short = stay_in_short,
            out_short = stay_out_short,
            notes = inputs.notes.unwrap_or(""),
        ));
        let first_book_date_id = inputs.book_date_id_base;
        let room_book_q = sql_quote(&first_book_date_id.to_string());
        let room_book_name_q = sql_quote(inputs.customer_name);
        let room_book_time_q = sql_quote("00:00");
        statements.push(format!(
            "update HT_Rooms set room_book_ds={room_book_ds_q}, Room_Book={room_book_q},\
             Room_Book_Name={room_book_name_q},Room_Book_Time={room_book_time_q} \
             where room_no={room_no_q}"
        ));
    }

    let _ = Datelike::year(&Utc::now().date_naive()); // silence unused-import lint
    statements
}

/// Execute the create-booking recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &CreateBookingPayload,
) -> WritebackResult<LegacyIds> {
    // Wave 6 LOW item 5: hard-validate `nights >= 1` instead of silently
    // clamping via `.max(1)`. The service layer should reject this at
    // enqueue time; this is defense-in-depth so a caller bug surfaces as a
    // `Recipe` error before any TABLOCKX allocation runs.
    if payload.nights < 1 {
        return Err(WritebackError::Recipe(format!(
            "CreateBooking: nights must be >= 1 (got {})",
            payload.nights
        )));
    }

    // Allocate IDs under TABLOCKX, in dependency order.
    let cust_no = match payload.legacy_cust_no.as_deref() {
        Some(existing) => existing.to_string(),
        None => allocate_cust_no(conn).await?,
    };
    let cust_id_int = if payload.legacy_cust_no.is_none() {
        Some(allocate_customer_id(conn).await?)
    } else {
        None
    };
    let book_id = allocate_book_id(conn).await?;
    let book_date_id_base = allocate_book_date_id(conn).await?;
    // H7: per-night HT_Room_Status rows need a base id allocated under
    // TABLOCKX, same lock pattern as the other counters.
    let room_status_id_base = allocate_room_status_id(conn).await?;

    // Wave 6 LOW item 6: empty range now surfaces as an error rather than
    // silently injecting a phantom night. A cap-truncate logs a WARN.
    let nights_calendar = enumerate_calendar_nights(payload.stay.start, payload.stay.end)?;
    let nights = payload.nights;
    let price_baht = (payload.price.as_satang() as f64) / 100.0;

    // HIGH-4: defense-in-depth NaN/Infinity guard. Money-derived f64s are
    // always finite today, but the night-total interpolation
    // (`price * nights`) could overflow to infinity for extreme inputs.
    let deposit_baht = (payload.deposit.as_satang() as f64) / 100.0;
    super::helpers::validate_finite(&[
        ("price_baht", price_baht),
        ("nightly_total", price_baht * (nights as f64)),
        ("deposit_baht", deposit_baht),
    ])?;

    let inputs = CreateBookingInputs {
        book_id: &book_id,
        cust_no: &cust_no,
        customer_id_int: cust_id_int,
        customer_name: &payload.customer_name,
        customer_phone: payload.customer_phone.as_deref(),
        created_by: payload.created_by.as_str(),
        notes: payload.notes.as_deref(),
        stay_start: payload.stay.start,
        stay_end: payload.stay.end,
        room_no: &payload.room_no,
        price_baht,
        deposit_baht,
        nights,
        book_date_id_base,
        room_status_id_base,
        nights_calendar,
        customer_is_new: payload.legacy_cust_no.is_none(),
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new()
        .with_book_id(book_id.clone())
        .with_cust_no(cust_no.clone());
    let _ = DEFAULT_OPERATOR; // silence unused-import lint when no fallback path used
    ids.extra
        .insert("book_date_id_base".into(), serde_json::Value::from(book_date_id_base));
    ids.extra
        .insert("room_status_id_base".into(), serde_json::Value::from(room_status_id_base));
    if let Some(id) = cust_id_int {
        ids.extra.insert("customer_id_int".into(), serde_json::Value::from(id));
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_inputs() -> CreateBookingInputs<'static> {
        CreateBookingInputs {
            book_id: "R014810",
            cust_no: "C21610",
            customer_id_int: Some(21610),
            customer_name: "SPIKE TEST WALKIN",
            customer_phone: Some("0900000088"),
            created_by: "Admin",
            notes: None,
            // 5 AM UTC = noon Bangkok (legacy app's wall-clock view).
            stay_start: Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            room_no: "402",
            price_baht: 890.0,
            deposit_baht: 0.0,
            nights: 1,
            book_date_id_base: 47285,
            room_status_id_base: 50237,
            nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            customer_is_new: false,
        }
    }

    /// Deposit (`เงินมัดจำ`) lands in `HT_Book_H.Book_Price_Pay` formatted
    /// to 2 decimal places. Was hardcoded `0.00` until 2.23.0.
    #[test]
    fn book_h_book_price_pay_carries_deposit_amount() {
        let mut inputs = sample_inputs();
        inputs.deposit_baht = 500.0;
        let statements = build_statements(&inputs);
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        // 7th and 8th positional values are Total then Pay (deposit). Capture
        // the comma-separated VALUES tuple to assert exact placement.
        assert!(
            book_h.contains(",890.00,500.00,"),
            "expected '...,Total,Pay,...' = '...,890.00,500.00,...' in {book_h}"
        );
    }

    #[test]
    fn book_h_book_price_pay_zero_when_no_deposit() {
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        assert!(book_h.contains(",890.00,0.00,"), "expected '...,890.00,0.00,...' in {book_h}");
    }

    #[test]
    fn book_h_uses_book_room_type_2_for_visibility() {
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        // The §3k visibility-required value
        assert!(book_h.contains(",2,"));
    }

    #[test]
    fn book_ds_uses_status_1_for_visibility() {
        let statements = build_statements(&sample_inputs());
        let book_ds = statements.iter().find(|s| s.contains("HT_Book_Ds")).unwrap();
        // Last positional value before the closing paren is Book_status=1
        assert!(book_ds.ends_with(",1)"));
    }

    #[test]
    fn book_h_dates_are_date_only_per_legacy_capture() {
        // Verified from /tmp/legacy-events-full.log captures of
        // R014820..R014824: HT_Book_H carries date-only forms
        // (`'4/25/2026'`), not midnight datetimes. The .NET app's
        // booking-list view binds to the date string directly.
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        assert!(book_h.contains("'4/25/2026'"));
        assert!(book_h.contains("'4/26/2026'"));
        assert!(!book_h.contains("'4/25/2026 12:00:00 AM'"));
    }

    /// Byte-level parity for HT_Book_H against the captured booking
    /// `R014820` (Cust C21624). Source: `/tmp/legacy-events-full.log`.
    #[test]
    fn book_h_matches_legacy_capture_byte_for_byte() {
        let inputs = CreateBookingInputs {
            book_id: "R014820",
            cust_no: "C21624",
            customer_id_int: Some(21624),
            customer_name: "Alberto Calvo Alvarez",
            customer_phone: Some(""),
            created_by: "Admin",
            notes: Some("เก็บเงิน "),
            // Bangkok 4/25..4/26
            stay_start: Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            room_no: "414",
            price_baht: 801.0,
            deposit_baht: 0.0,
            nights: 1,
            book_date_id_base: 47200,
            room_status_id_base: 50200,
            nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            customer_is_new: false,
        };
        let s = build_statements(&inputs);
        let book_h = s.iter().find(|s| s.contains("[HT_Book_H]")).unwrap();
        let head = "INSERT INTO [HT_Book_H]( [Book_ID],[Book_Date],[Book_Cust_ID],\
                    [Book_Cust_Name],[Book_Cust_Name2],[Book_Cust_Tel],[Book_Price_Total],\
                    [Book_Price_Pay],[Book_Status],[Book_Date_in],[Book_Date_out],[Book_by],\
                    [Book_room_all],[Book_room_note],[book_room_type],Book_Notify_Day,\
                    Book_sale)VALUES( 'R014820',";
        assert!(
            book_h.starts_with(head),
            "HT_Book_H column list must match legacy capture; got:\n{book_h}"
        );
        let tail = ",'C21624','Alberto Calvo Alvarez','','',801.00,0.00,'จอง','4/25/2026',\
                    '4/26/2026','Admin','','เก็บเงิน ',2,3,'')";
        assert!(
            book_h.ends_with(tail),
            "HT_Book_H value tail must match legacy capture; got:\n{book_h}"
        );
    }

    #[test]
    fn book_h_drops_obsolete_book_notify_note_column() {
        let s = build_statements(&sample_inputs());
        let book_h = s.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        assert!(!book_h.contains("Book_Notify_Note"));
    }

    /// Byte-level parity for the new-customer HT_Customers INSERT
    /// against captured C21624 in `/tmp/legacy-events-full.log`.
    /// Cust_Last_Change is determined from `Utc::now()` so we
    /// substring-match around the column list and value head.
    #[test]
    fn ht_customers_insert_matches_legacy_capture_byte_for_byte() {
        let mut inputs = sample_inputs();
        inputs.customer_is_new = true;
        inputs.cust_no = "C21624";
        inputs.customer_id_int = Some(21624);
        inputs.customer_name = "Alberto Calvo Alvarez";
        inputs.customer_phone = Some("");
        let s = build_statements(&inputs);
        let cust = s
            .iter()
            .find(|s| s.starts_with("INSERT INTO [HT_Customers]"))
            .expect("HT_Customers INSERT must be emitted for new customer");
        let head = "INSERT INTO [HT_Customers]([id],[Cust_no],[Cust_perfix],[Cust_name],\
                    [Cust_name2],[Cust_Type],[Cust_Email],[Cust_Add_no],[Cust_Add_moo],\
                    [Cust_Add_soi],[Cust_Add_road],[Cust_Add_tambon],[Cust_Add_ampore],\
                    [Cust_Add_province],[Cust_Add_code],[Cust_Add_tel],[Cust_Add_fax],\
                    [Cust_Work_Name],[Cust_Work_no],[Cust_Work_moo],[Cust_Work_soi],\
                    [Cust_Work_road],[Cust_Work_tambon],[Cust_Work_ampore],[Cust_Work_province],\
                    [Cust_Work_code],[Cust_Work_tel],[Cust_Work_fax],[Cust_Last_Change],\
                    [Cust_Type_Main]) VALUES (21624,'C21624','','Alberto Calvo Alvarez','',\
                    'ราคาปกติ','','','','','','','','','','','','','','','','','','','','','',\
                    '',";
        assert!(
            cust.starts_with(head),
            "HT_Customers INSERT must match legacy column list + leading values; got:\n{cust}"
        );
        // Tail: 'M/D/YYYY' last_change + 'ราคาปกติ' type_main + ')'.
        // The date depends on Utc::now() so we just assert the type_main ending.
        assert!(
            cust.ends_with(",'ราคาปกติ')"),
            "HT_Customers Cust_Type_Main must be 'ราคาปกติ'; got:\n{cust}"
        );
        // Drops the four obsolete columns the audit identified.
        assert!(!cust.contains("[Cust_sex]"));
        assert!(!cust.contains("[Cust_IDcard]"));
        assert!(!cust.contains("[Cust_Contry]"));
        assert!(!cust.contains("[Cust_Work_Tax]"));
    }

    #[test]
    fn book_ds_uses_actual_stay_times_with_almost_noon_end() {
        let statements = build_statements(&sample_inputs());
        let book_ds = statements.iter().find(|s| s.contains("HT_Book_Ds")).unwrap();
        // Actual start time stays at noon, end is snapped to 11:59:59 AM
        assert!(book_ds.contains("'4/25/2026 12:00:00 PM'"));
        assert!(book_ds.contains("'4/26/2026 11:59:59 AM'"));
    }

    #[test]
    fn book_ds_room_type_column_stores_room_number() {
        let statements = build_statements(&sample_inputs());
        let book_ds = statements.iter().find(|s| s.contains("HT_Book_Ds")).unwrap();
        // ⚠️ This is the misleading column name — it stores room number not type
        assert!(book_ds.contains("'402'"));
    }

    #[test]
    fn book_h_room_all_field_is_empty_per_section_3k() {
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        // Book_room_all must be '' or .NET hides the booking
        assert!(book_h.contains(",'',")); // there will be many '' but at minimum present
    }

    /// Book_Status must be 'จอง' (Thai for "booked") — visibility key for
    /// the .NET app's future-date room view. The view query is
    /// `SELECT * FROM View_Book_Date WHERE book_status='จอง' AND
    /// Book_Date_ds=...`. Empty string (the prior value) hid bookings from
    /// that view — the 2026-04-26 R014838/R014839 incident.
    #[test]
    fn book_h_status_is_jong_for_visibility_in_future_room_view() {
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        assert!(
            book_h.contains("'จอง'"),
            "Book_Status must be 'จอง' for future-room-view visibility: {book_h}"
        );
    }

    #[test]
    fn book_h_notify_day_defaults_to_three() {
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        assert!(book_h.contains(",3,"));
    }

    #[test]
    fn book_date_one_row_per_calendar_night() {
        let mut inputs = sample_inputs();
        inputs.nights_calendar = vec![
            NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 27).unwrap(),
        ];
        let statements = build_statements(&inputs);
        let date_inserts: Vec<&String> = statements
            .iter()
            .filter(|s| s.starts_with("INSERT INTO [HT_Book_Date]"))
            .collect();
        assert_eq!(date_inserts.len(), 3);
        assert!(date_inserts[0].contains("47285"));
        assert!(date_inserts[1].contains("47286"));
        assert!(date_inserts[2].contains("47287"));
    }

    /// Reverse of the prior (now-deleted) Book_ok regression test. The
    /// legacy app's monitor capture of R014836 (clean future booking-create
    /// on 2026-04-25) shows ZERO Book_ok bumps — only the 3 INSERTs.
    /// Earlier code added a bump per night based on a misread of a
    /// CHECK-IN flow capture; this test guards against re-introducing it.
    #[test]
    fn book_date_inserts_do_not_bump_book_ok() {
        let mut inputs = sample_inputs();
        inputs.nights_calendar = vec![
            NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
        ];
        let statements = build_statements(&inputs);
        let book_ok_updates: Vec<&String> = statements
            .iter()
            .filter(|s| s.to_lowercase().contains("book_ok"))
            .collect();
        assert!(
            book_ok_updates.is_empty(),
            "booking_create must NOT touch Book_ok — that bump belongs to \
             the check-in recipe path. Got: {book_ok_updates:?}"
        );
    }

    /// HT_Rooms display-field UPDATE must be SKIPPED for future bookings.
    /// Setting Room_Book / room_book_ds for a booking starting tomorrow (or
    /// later) makes the room appear "currently booked" in the .NET app's
    /// room-list view today — the 2026-04-25 R014835 incident. Legacy app
    /// (R014836 capture) emits zero HT_Rooms statements for future creates.
    #[test]
    fn skips_ht_rooms_update_for_future_booking() {
        let mut inputs = sample_inputs();
        // Push the stay 30 days into the future relative to "today" (Bangkok
        // wall-clock at test time). A 30-day delta dwarfs any plausible
        // test-clock skew (the Bangkok timezone offset is +7h, so 30 days is
        // safely > today no matter what UTC minute the test runs at).
        let today_bkk = Utc::now().with_timezone(&chrono_tz::Asia::Bangkok).date_naive();
        let future = today_bkk
            .checked_add_days(chrono::Days::new(30))
            .expect("today + 30d is a valid date");
        let future_noon = chrono_tz::Asia::Bangkok
            .from_local_datetime(&future.and_hms_opt(12, 0, 0).unwrap())
            .single()
            .unwrap()
            .with_timezone(&Utc);
        inputs.stay_start = future_noon;
        inputs.stay_end = future_noon + chrono::Duration::days(1);
        inputs.nights_calendar = vec![future];
        let statements = build_statements(&inputs);
        let ht_rooms_updates: Vec<&String> = statements
            .iter()
            .filter(|s| s.contains("update HT_Rooms"))
            .collect();
        assert!(
            ht_rooms_updates.is_empty(),
            "future booking must NOT touch HT_Rooms display fields. Got: {ht_rooms_updates:?}"
        );
    }

    /// Inverse: bookings starting TODAY DO get the HT_Rooms display update.
    /// (For the same-day walk-in-via-booking flow, since it's a today-start
    /// booking that would replace whatever caption was there.)
    #[test]
    fn emits_ht_rooms_update_for_today_booking() {
        let mut inputs = sample_inputs();
        let today_bkk = Utc::now().with_timezone(&chrono_tz::Asia::Bangkok).date_naive();
        let today_noon = chrono_tz::Asia::Bangkok
            .from_local_datetime(&today_bkk.and_hms_opt(12, 0, 0).unwrap())
            .single()
            .unwrap()
            .with_timezone(&Utc);
        inputs.stay_start = today_noon;
        inputs.stay_end = today_noon + chrono::Duration::days(1);
        inputs.nights_calendar = vec![today_bkk];
        let statements = build_statements(&inputs);
        let ht_rooms_updates: Vec<&String> = statements
            .iter()
            .filter(|s| s.contains("update HT_Rooms"))
            .collect();
        assert_eq!(
            ht_rooms_updates.len(),
            1,
            "today booking must emit exactly one HT_Rooms display UPDATE"
        );
    }

    #[test]
    fn skips_customer_insert_when_existing() {
        let mut inputs = sample_inputs();
        inputs.customer_is_new = false;
        let statements = build_statements(&inputs);
        assert!(!statements.iter().any(|s| s.contains("HT_Customers")));
    }

    #[test]
    fn includes_customer_insert_when_new() {
        let mut inputs = sample_inputs();
        inputs.customer_is_new = true;
        let statements = build_statements(&inputs);
        let cust = statements.iter().find(|s| s.contains("HT_Customers")).unwrap();
        assert!(cust.contains("INSERT INTO"));
        assert!(cust.contains("'C21610'"));
        assert!(cust.contains("'SPIKE TEST WALKIN'"));
        assert!(cust.contains("'0900000088'"));
    }

    /// Wave 6 LOW items 1 + 6: enumerate_calendar_nights now lives in
    /// `writeback::format` and returns a Result so the empty-range and
    /// cap-truncate guards surface. The functional cases below confirm the
    /// per-recipe import still produces the right nights for booking-create.
    #[test]
    fn enumerate_calendar_nights_handles_one_night() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(nights, vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()]);
    }

    #[test]
    fn enumerate_calendar_nights_handles_two_nights() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 27, 12, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(
            nights,
            vec![
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap()
            ]
        );
    }

    #[test]
    fn end_of_stay_snaps_to_11_59_59_am() {
        let snapped =
            end_of_stay_at_almost_noon(Utc.with_ymd_and_hms(2026, 4, 26, 14, 30, 0).unwrap());
        assert_eq!(format_legacy_datetime(snapped), "4/26/2026 11:59:59 AM");
    }

    /// H7 — booking-create must insert one `HT_Room_Status` row per booked
    /// night with `status='จอง'`, `room_Book_No=Book_ID`. Without these
    /// rows, the .NET app's calendar grid shows the night as empty AND
    /// `checkin_to_booking`'s night-0 UPDATE matches 0 rows silently.
    /// Per `COMPAT_CHEATSHEET.md` line 347.
    #[test]
    fn inserts_ht_room_status_per_booked_night() {
        let mut inputs = sample_inputs();
        inputs.nights_calendar = vec![
            NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 27).unwrap(),
        ];
        inputs.nights = 3;
        inputs.room_status_id_base = 50300;
        let statements = build_statements(&inputs);
        let room_status_inserts: Vec<&String> = statements
            .iter()
            .filter(|s| s.starts_with("INSERT INTO [HT_Room_Status]"))
            .collect();
        assert_eq!(
            room_status_inserts.len(),
            3,
            "expected one HT_Room_Status row per booked night"
        );
        for stmt in &room_status_inserts {
            assert!(
                stmt.contains("'จอง'"),
                "room_status must be 'จอง' for booking nights: {stmt}"
            );
            assert!(
                stmt.contains("'R014810'"),
                "room_Book_No must carry Book_ID: {stmt}"
            );
        }
        // Distinct room_date per night.
        assert!(room_status_inserts[0].contains("'4/25/2026'"));
        assert!(room_status_inserts[1].contains("'4/26/2026'"));
        assert!(room_status_inserts[2].contains("'4/27/2026'"));
        // ids increment from base.
        assert!(room_status_inserts[0].contains("50300"));
        assert!(room_status_inserts[1].contains("50301"));
        assert!(room_status_inserts[2].contains("50302"));
    }

    /// HT_Room_Status rows for a booking carry the room number (denormalized)
    /// and an empty `room_CheckIn_No` (the booking has no check-in yet).
    #[test]
    fn ht_room_status_for_booking_uses_room_no_and_empty_checkin() {
        let mut inputs = sample_inputs();
        inputs.room_status_id_base = 50400;
        let statements = build_statements(&inputs);
        let row = statements
            .iter()
            .find(|s| s.starts_with("INSERT INTO [HT_Room_Status]"))
            .expect("HT_Room_Status row must be emitted");
        // room_no = '402'
        assert!(row.contains("'402'"));
        // room_Book_No carries Book_ID, room_CheckIn_No is empty for a booking
        assert!(row.contains("'R014810'"));
    }
}
