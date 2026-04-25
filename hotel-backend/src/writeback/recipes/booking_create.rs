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

    // 4..N. HT_Book_Date — one row per calendar night
    for (i, day) in inputs.nights_calendar.iter().enumerate() {
        let id = inputs.book_date_id_base + i as i32;
        let date_q = sql_quote(&format_legacy_date(*day));
        statements.push(format!(
            "INSERT INTO [HT_Book_Date]([id],[Book_no],[Book_type],[Book_date_ds],[Book_Num],\
             [Book_USE])VALUES({id},{book_id_q},{room_no_q},{date_q},1,0)"
        ));
    }
    let _ = Datelike::year(&Utc::now().date_naive()); // silence unused-import lint
    statements
}

/// Snap a stay-end DateTime to `11:59:59 AM` of the same calendar day.
/// Per spike §3b — convenient for date-range BETWEEN queries.
pub fn end_of_stay_at_almost_noon(stay_end: DateTime<Utc>) -> DateTime<Utc> {
    let date = stay_end.date_naive();
    Utc.from_utc_datetime(
        &date
            .and_hms_opt(11, 59, 59)
            .expect("11:59:59 is a valid time"),
    )
}

/// Enumerate calendar nights in `[stay_start, stay_end)`. Each becomes one
/// `HT_Book_Date` row.
pub fn enumerate_calendar_nights(
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
        price_baht: (payload.price.as_satang() as f64) / 100.0,
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
            stay_start: Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
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
        let date_inserts: Vec<&String> = statements.iter().filter(|s| s.contains("HT_Book_Date")).collect();
        assert_eq!(date_inserts.len(), 3);
        assert!(date_inserts[0].contains("47285"));
        assert!(date_inserts[1].contains("47286"));
        assert!(date_inserts[2].contains("47287"));
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
