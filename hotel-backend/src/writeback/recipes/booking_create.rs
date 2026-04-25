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
//! | `Book_Date_in/out` (HT_Book_H) | midnight | top-of-form date display |
//! | `Book_room_all` | `''` | unused header field; must be empty |
//! | `Book_Notify_Day` | `3` | .NET app default |
//! | All optional varchars | `''` | NULL crashes WinForms downstream |
//!
//! Departure on `HT_Book_Ds.Book_Room_End` is `'11:59:59 AM'` — convenient
//! for date-range BETWEEN queries (per spike §3b).

use chrono::{DateTime, Datelike, Days, NaiveDate, TimeZone, Utc};

use crate::outbox::intent::CreateBookingPayload;
use crate::writeback::allocate::{
    allocate_book_date_id, allocate_book_id, allocate_cust_no, allocate_customer_id, LegacyConn,
};
use crate::writeback::constants::{
    BOOK_DS_STATUS_ACTIVE, BOOK_NOTIFY_DAY_DEFAULT, BOOK_ROOM_TYPE_BY_ROOM_NUMBER, CUST_TYPE_NORMAL,
    DEFAULT_OPERATOR,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_date, format_legacy_datetime, midnight_of, sql_quote};

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
    pub nights: i32,
    /// First `HT_Book_Date.id` to use for night INSERTs.
    pub book_date_id_base: i32,
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

    // §3k: midnight on HT_Book_H, actual stay times on HT_Book_Ds.
    let stay_start_midnight_q =
        sql_quote(&format_legacy_datetime(midnight_of(inputs.stay_start)));
    let stay_end_midnight_q = sql_quote(&format_legacy_datetime(midnight_of(inputs.stay_end)));

    // HT_Book_Ds: actual stay times. Departure is hardcoded 11:59:59 AM per spike §3b.
    let stay_start_actual_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_actual = end_of_stay_at_almost_noon(inputs.stay_end);
    let stay_end_actual_q = sql_quote(&format_legacy_datetime(stay_end_actual));

    let mut statements: Vec<String> = Vec::with_capacity(4 + inputs.nights_calendar.len());

    // 1. HT_Customers (only if new)
    if inputs.customer_is_new {
        let id = inputs.customer_id_int.unwrap_or(0);
        statements.push(format!(
            "INSERT INTO [HT_Customers]([id],[Cust_no],[Cust_perfix],[Cust_name],[Cust_name2],\
             [Cust_sex],[Cust_IDcard],[Cust_Type],[Cust_Email],[Cust_Add_no],[Cust_Add_moo],\
             [Cust_Add_soi],[Cust_Add_road],[Cust_Add_tambon],[Cust_Add_ampore],\
             [Cust_Add_province],[Cust_Add_code],[Cust_Add_tel],[Cust_Add_fax],[Cust_Work_Name],\
             [Cust_Work_no],[Cust_Work_moo],[Cust_Work_soi],[Cust_Work_road],[Cust_Work_tambon],\
             [Cust_Work_ampore],[Cust_Work_province],[Cust_Work_code],[Cust_Work_tel],\
             [Cust_Work_fax],[Cust_Type_Main],[Cust_Contry],[Cust_Work_Tax])\
             VALUES({id},{cust_no_q},'',{cust_name_q},'','','',{cust_type_q},'','','','','',\
             '','','','',{cust_phone_q},'','','','','','','','','','','',\
             '','บุคคลธรรมดา','','')"
        ));
    }

    // 2. HT_Book_H — full §3k visibility set
    statements.push(format!(
        "INSERT INTO [HT_Book_H]( [Book_ID],[Book_Date],[Book_Cust_ID],[Book_Cust_Name],\
         [Book_Cust_Name2],[Book_Cust_Tel],[Book_Price_Total],[Book_Price_Pay],[Book_Status],\
         [Book_Date_in],[Book_Date_out],[Book_by],[Book_room_all],[Book_room_note],\
         [book_room_type],[Book_Notify_Day],[Book_Notify_Note],[Book_Sale])\
         VALUES({book_id_q},{now_q},{cust_no_q},{cust_name_q},'',{cust_phone_q},\
         {price},0,'',{stay_in},{stay_out},{by_q},'',{notes_q},{room_type_code},\
         {notify_day},'','')",
        price = inputs.price_baht,
        stay_in = stay_start_midnight_q,
        stay_out = stay_end_midnight_q,
        room_type_code = BOOK_ROOM_TYPE_BY_ROOM_NUMBER,
        notify_day = BOOK_NOTIFY_DAY_DEFAULT,
    ));

    // 3. HT_Book_Ds — Book_Room_Type stores ROOM NUMBER (per §3b finding)
    //    id is IDENTITY → omit from column list.
    statements.push(format!(
        "INSERT INTO [HT_Book_Ds]([Book_No],[Book_Room_Type],[Book_Room_Start],[Book_Room_End],\
         [Book_Room_Price],[Book_Room_Night],[Book_Room_Num],[Book_Room_PriceToTal],\
         [Book_Room_Note],[Book_status])VALUES({book_id_q},{room_no_q},{start},{end},\
         {price},{nights},1,{total},'',{status})",
        start = stay_start_actual_q,
        end = stay_end_actual_q,
        price = inputs.price_baht,
        nights = inputs.nights,
        total = inputs.price_baht * inputs.nights as f64,
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

/// Snap a stay-end DateTime to `11:59:59 AM` of the same Bangkok calendar day.
/// Per spike §3b — convenient for date-range BETWEEN queries. Uses Bangkok
/// time to determine the calendar day (a 17:00Z instant is the *next* day in
/// Bangkok, so `date_naive()` on UTC would return the wrong day).
pub fn end_of_stay_at_almost_noon(stay_end: DateTime<Utc>) -> DateTime<Utc> {
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

/// Enumerate Bangkok calendar nights in `[stay_start, stay_end)`. Each becomes
/// one `HT_Book_Date` row. The boundaries are the Bangkok calendar day, not
/// the UTC one — a 17:00Z instant is already the next day in Bangkok.
pub fn enumerate_calendar_nights(
    stay_start: DateTime<Utc>,
    stay_end: DateTime<Utc>,
) -> Vec<NaiveDate> {
    use chrono_tz::Asia::Bangkok;
    let start = stay_start.with_timezone(&Bangkok).date_naive();
    let end = stay_end.with_timezone(&Bangkok).date_naive();
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
        // Single-day stay (departure same calendar day) — still record one night
        nights.push(start);
    }
    nights
}

/// Execute the create-booking recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &CreateBookingPayload,
) -> WritebackResult<LegacyIds> {
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

    let nights_calendar = enumerate_calendar_nights(payload.stay.start, payload.stay.end);
    let nights = payload.nights.max(1);
    let price_baht = (payload.price.as_satang() as f64) / 100.0;

    // HIGH-4: defense-in-depth NaN/Infinity guard. Money-derived f64s are
    // always finite today, but the night-total interpolation
    // (`price * nights`) could overflow to infinity for extreme inputs.
    super::helpers::validate_finite(&[
        ("price_baht", price_baht),
        ("nightly_total", price_baht * (nights as f64)),
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
        nights,
        book_date_id_base,
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
    if let Some(id) = cust_id_int {
        ids.extra.insert("customer_id_int".into(), serde_json::Value::from(id));
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            nights: 1,
            book_date_id_base: 47285,
            nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            customer_is_new: false,
        }
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
    fn book_h_dates_are_midnight_per_section_3k() {
        let statements = build_statements(&sample_inputs());
        let book_h = statements.iter().find(|s| s.contains("HT_Book_H")).unwrap();
        // Midnight = "12:00:00 AM"
        assert!(book_h.contains("'4/25/2026 12:00:00 AM'"));
        assert!(book_h.contains("'4/26/2026 12:00:00 AM'"));
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

    #[test]
    fn enumerate_calendar_nights_handles_one_night() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
        );
        assert_eq!(nights, vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()]);
    }

    #[test]
    fn enumerate_calendar_nights_handles_two_nights() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 27, 12, 0, 0).unwrap(),
        );
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
}
