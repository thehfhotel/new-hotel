//! `CreateCheckIn` linked-to-booking recipe — spike `findings.md` §3d.
//!
//! Check-in against an existing booking (5 differences from walk-in §3a):
//! 1. **No** `INSERT INTO HT_Customers` — customer already exists from booking.
//! 2. `UPDATE HT_Customers` instead (re-save profile).
//! 3. `UPDATE HT_Book_H SET Book_Status='เข้าพัก' WHERE Book_ID=…` — booking
//!    marked as occupying.
//! 4. `UPDATE HT_Rooms SET room_book_*='', room_book_name=''` — clear booking
//!    display so room shows occupied not booked (subquery on `View_HT_ROOM`).
//! 5. **Existing** `HT_Room_Status` rows get UPDATEd (no `room_CheckIn_No`
//!    filter — overwrites whatever matches `(room_date, room_no)`). Missing
//!    nights still get INSERTed. Per spike §3d this is "current-state" behavior.
//! 6. `Cin_Book_no='R…'` set in `HT_CheckIn_H` — the linkage.
//!
//! Reference SQL (verbatim from `booking-checkin-20260424-101838/writes.txt`
//! lines 25-37 — the 10:23:02 block):
//!
//! ```text
//! UPDATE [HT_Customers] SET [Cust_name]='SPIKE TEST WALKIN', … WHERE Cust_no='C21610'
//! update Tb_Save_Image set cin_no='CH26-005231', cust_no='C21610', tmp_no=''
//!     where tmp_no='221643'
//! INSERT INTO [HT_POWER_LOG]([ROOM_NO], …, ROOM_POWER_NOTE='เปิดไฟ … No.CH26-005231')
//! INSERT INTO [HT_CheckIn_Ds]([id],[Cin_No],[Cin_Room_No], …)
//! update HT_Rooms set room_use='yes' where room_no='402'
//! update [HT_Room_Status] SET [room_status]='เข้าพัก',[room_Details]='SPIKE TEST WALKIN',
//!        [room_CheckIn_No]='CH26-005231'
//!     where room_date='4/24/2026' and room_no='402'                  -- the overwrite
//! INSERT INTO [HT_CheckIn_Other_People] (…)
//! INSERT INTO [HT_Room_Status] (…)                                   -- next night
//! update HT_Book_H set Book_Status='เข้าพัก' where Book_ID='R014810'
//! update HT_Rooms set room_book_ds='',room_book='',room_book_name='',room_book_time=''
//!     where room_no in (select room_no from View_HT_ROOM where book_no='R014810')
//! INSERT INTO [HT_CheckIn_H] (…, Cin_Book_no='R014810', …)
//! ```

use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::outbox::intent::CreateCheckInPayload;
use crate::writeback::allocate::{allocate_cin_no, allocate_room_status_id, LegacyConn};
use crate::writeback::constants::{
    power_log_note_check_in, BOOK_STATUS_OCCUPYING, CIN_ROOM_STATUS_OCCUPYING, DEFAULT_OPERATOR,
    ROOM_STATUS_OCCUPYING,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    date_to_ole_serial, format_legacy_date, format_legacy_datetime, sql_quote,
};

/// Inputs for the check-in-to-booking recipe.
#[derive(Debug, Clone)]
pub struct CheckInToBookingInputs<'a> {
    pub cin_no: &'a str,
    pub cust_no: &'a str,
    pub book_id: &'a str,
    pub customer_name: &'a str,
    pub customer_phone: Option<&'a str>,
    pub guest_name_for_registry: &'a str,
    pub guest_country: &'a str,
    pub created_by: &'a str,
    pub room_no: &'a str,
    pub room_type: &'a str,
    pub stay_start: DateTime<Utc>,
    pub stay_end: DateTime<Utc>,
    pub price_per_night_baht: f64,
    pub nights: i32,
    pub price_total_baht: f64,
    /// First `HT_Room_Status.id` to use for new-night INSERTs.
    pub room_status_id_base: i32,
    pub nights_calendar: Vec<NaiveDate>,
}

/// Build statements for a check-in linked to a booking. PURE — no I/O.
pub fn build_statements(inputs: &CheckInToBookingInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let book_id_q = sql_quote(inputs.book_id);
    let by_q = sql_quote(inputs.created_by);
    let room_no_q = sql_quote(inputs.room_no);
    let room_type_q = sql_quote(inputs.room_type);
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_phone_q = sql_quote(inputs.customer_phone.unwrap_or(""));
    let stay_start_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_q = sql_quote(&format_legacy_datetime(inputs.stay_end));
    let now_q = sql_quote(&format_legacy_datetime(Utc::now()));
    let occupying_q = sql_quote(CIN_ROOM_STATUS_OCCUPYING);
    let room_status_q = sql_quote(ROOM_STATUS_OCCUPYING);
    let book_status_q = sql_quote(BOOK_STATUS_OCCUPYING);
    let power_note = power_log_note_check_in(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    let registry_name = format!("Mr. {}", inputs.guest_name_for_registry);
    let registry_name_q = sql_quote(&registry_name);
    let country_q = sql_quote(inputs.guest_country);
    let price = inputs.price_per_night_baht;
    let total = inputs.price_total_baht;
    let nights = inputs.nights;

    let mut statements: Vec<String> = Vec::with_capacity(8 + inputs.nights_calendar.len());

    // 1. UPDATE HT_Customers (re-save profile — §3d difference vs walk-in)
    statements.push(format!(
        "UPDATE [HT_Customers] SET  [Cust_name]={cust_name_q},[Cust_name2]='',\
         [Cust_Type]='ราคาปกติ',[Cust_Type_Main]='บุคคลธรรมดา',[Cust_Add_tel]={cust_phone_q} \
         WHERE Cust_no={cust_no_q}"
    ));

    // 2. HT_POWER_LOG — lights on
    statements.push(format!(
        "INSERT INTO [HT_POWER_LOG]([ROOM_NO],[ROOM_POWER_START],[ROOM_POWER_START_BY],\
         [ROOM_POWER_END_BY],[ROOM_POWER_NOTE],[ROOM_POWER_NOTE2])\
         VALUES({room_no_q},GETDATE(),{by_q},'',{power_note_q},'')"
    ));

    // 3. HT_CheckIn_Ds — id is IDENTITY, omit
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Ds]([Cin_No],[Cin_Room_No],[Cin_Room_Type],[Cin_Room_In],\
         [Cin_Room_Out],[Cin_Room_Status],[Cin_Room_Dep],[Cin_Room_Price],[Cin_Room_Night],\
         [Cin_Room_PriceToTal],[Cin_Room_Pay_Before],[Cin_Room_Pay_Total],[Cin_note],\
         [Cin_Dep_Status],[Dep_by],[Cin_cupon])\
         VALUES({cin_no_q},{room_no_q},{room_type_q},{stay_start_q},{stay_end_q},\
         {occupying_q},0,{price},{nights},{total},0,0,'','','',0)"
    ));

    // 4. Mark room occupied
    statements.push(format!(
        "update HT_Rooms set room_use='yes' where room_no={room_no_q}"
    ));

    // 5. UPDATE existing HT_Room_Status row(s) — §3d: no Cin_no filter, the
    //    .NET app overwrites by (room_date, room_no). We do the same for parity.
    if let Some(first_day) = inputs.nights_calendar.first() {
        let first_date_q = sql_quote(&format_legacy_date(*first_day));
        statements.push(format!(
            "update [HT_Room_Status] SET  [room_status]={room_status_q},\
             [room_Details]={cust_name_q},[room_CheckIn_No]={cin_no_q} \
             where room_date={first_date_q} and room_no={room_no_q}"
        ));
    }

    // 6..N. HT_Room_Status INSERTs for additional nights (skip the first —
    //       handled by UPDATE above per spike §3d).
    for (i, day) in inputs.nights_calendar.iter().enumerate().skip(1) {
        let id = inputs.room_status_id_base + (i as i32 - 1);
        let date_q = sql_quote(&format_legacy_date(*day));
        let oa = date_to_ole_serial(*day) as i64;
        statements.push(format!(
            "INSERT INTO [HT_Room_Status]([id],[room_no],[room_date],[room_status],\
             [room_Details],[room_CheckIn_No],[room_date_oa])\
             VALUES({id},{room_no_q},{date_q},{room_status_q},{cust_name_q},{cin_no_q},{oa})"
        ));
    }

    // 7. HT_CheckIn_Other_People — TM.30 primary guest
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])\
         VALUES({cin_no_q},{registry_name_q},{country_q})"
    ));

    // 8. UPDATE HT_Book_H — booking now occupying
    statements.push(format!(
        "update HT_Book_H set Book_Status={book_status_q} where Book_ID={book_id_q}"
    ));

    // 9. Clear room_book_* display columns — booking has become a check-in
    statements.push(format!(
        "update HT_Rooms set room_book_ds='',room_book='',room_book_name='',room_book_time='' \
         where room_no in (select room_no from View_HT_ROOM where book_no={book_id_q})"
    ));

    // 10. HT_CheckIn_H — Cin_Book_no set to the linked booking
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
         [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
         [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],[Cin_Car_id],\
         [Cin_Room_ALL],[Total_Price_vat],[Cin_by],[Cin_Date_in],[Cin_Date_out],[Cin_type],\
         [Cin_note],[Cin_foreign],[Cin_Work_number])\
         VALUES({cin_no_q},{now_q},{book_id_q},{cust_no_q},'',{occupying_q},{total},0,{total},0,\
         {total},'','',{room_no_q},0,{by_q},{stay_start_q},{stay_end_q},0,'','',0)"
    ));

    let _ = NaiveDate::from_ymd_opt(2026, 1, 1); // silence unused-import lint
    let _ = Datelike::year(&Utc::now()); // silence unused-import lint
    statements
}

/// Execute the check-in-to-booking recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &CreateCheckInPayload,
    book_id: &str,
) -> WritebackResult<LegacyIds> {
    let cust_no = payload.legacy_cust_no.clone().ok_or_else(|| {
        crate::writeback::error::WritebackError::Recipe(
            "CheckIn-to-booking requires legacy_cust_no in payload (§3d: customer already exists)"
                .into(),
        )
    })?;
    let cin_no = allocate_cin_no(conn).await?;
    let room_status_id_base = allocate_room_status_id(conn).await?;

    let nights_calendar = enumerate_calendar_nights(payload.stay.start, payload.stay.end);

    let inputs = CheckInToBookingInputs {
        cin_no: &cin_no,
        cust_no: &cust_no,
        book_id,
        customer_name: &payload.guest_name_for_registry,
        customer_phone: None,
        guest_name_for_registry: &payload.guest_name_for_registry,
        guest_country: &payload.guest_country,
        created_by: &payload.created_by,
        room_no: &payload.room_no,
        room_type: &payload.room_type,
        stay_start: payload.stay.start,
        stay_end: payload.stay.end,
        price_per_night_baht: (payload.price_per_night.as_satang() as f64) / 100.0,
        nights: payload.nights.max(1),
        price_total_baht: (payload.price_total.as_satang() as f64) / 100.0,
        room_status_id_base,
        nights_calendar,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;

    let _ = DEFAULT_OPERATOR; // silence unused-import lint
    let mut ids = LegacyIds::new()
        .with_cin_no(cin_no.clone())
        .with_cust_no(cust_no.clone())
        .with_book_id(book_id.to_string());
    ids.extra
        .insert("room_status_id_base".into(), serde_json::Value::from(room_status_id_base));
    Ok(ids)
}

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
        day = match day.succ_opt() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_inputs() -> CheckInToBookingInputs<'static> {
        CheckInToBookingInputs {
            cin_no: "CH26-005231",
            cust_no: "C21610",
            book_id: "R014810",
            customer_name: "SPIKE TEST WALKIN",
            customer_phone: None,
            guest_name_for_registry: "SPIKE TEST WALKIN",
            guest_country: "",
            created_by: "Admin",
            room_no: "402",
            room_type: "Standard",
            stay_start: Utc.with_ymd_and_hms(2026, 4, 24, 10, 23, 02).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 26, 11, 59, 59).unwrap(),
            price_per_night_baht: 890.0,
            nights: 2,
            price_total_baht: 1780.0,
            room_status_id_base: 50237,
            nights_calendar: vec![
                NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            ],
        }
    }

    #[test]
    fn does_not_insert_into_ht_customers() {
        // Per spike §3d difference #1: no INSERT, only UPDATE
        let s = build_statements(&sample_inputs());
        assert!(!s.iter().any(|s| s.starts_with("INSERT INTO [HT_Customers]")));
    }

    #[test]
    fn updates_ht_customers_to_resave_profile() {
        let s = build_statements(&sample_inputs());
        assert!(s.iter().any(|s| s.starts_with("UPDATE [HT_Customers]")));
    }

    #[test]
    fn marks_booking_as_occupying() {
        let s = build_statements(&sample_inputs());
        let book_h = s.iter().find(|s| s.contains("update HT_Book_H")).unwrap();
        assert!(book_h.contains("Book_Status='เข้าพัก'"));
        assert!(book_h.contains("Book_ID='R014810'"));
    }

    #[test]
    fn clears_room_book_display_via_view_subquery() {
        let s = build_statements(&sample_inputs());
        let upd = s
            .iter()
            .find(|s| s.contains("room_book_ds=''") && s.contains("View_HT_ROOM"))
            .unwrap();
        assert!(upd.contains("book_no='R014810'"));
    }

    #[test]
    fn checkin_h_carries_book_no_linkage() {
        let s = build_statements(&sample_inputs());
        let cin_h = s
            .iter()
            .find(|s| s.starts_with("INSERT INTO [HT_CheckIn_H]"))
            .unwrap();
        assert!(cin_h.contains("'R014810'"));
        // Should not have Cin_Book_no='' for linked-to-booking case
    }

    #[test]
    fn first_room_status_row_is_updated_not_inserted() {
        let s = build_statements(&sample_inputs());
        // The UPDATE on existing HT_Room_Status row by (room_date, room_no)
        let upd = s
            .iter()
            .find(|s| s.starts_with("update [HT_Room_Status]"))
            .unwrap();
        assert!(upd.contains("room_date='4/24/2026'"));
        assert!(upd.contains("room_no='402'"));
        assert!(upd.contains("[room_CheckIn_No]='CH26-005231'"));
    }

    #[test]
    fn additional_nights_are_inserted() {
        let s = build_statements(&sample_inputs());
        // 2 nights total, first is UPDATE, second is INSERT
        let inserts: Vec<&String> = s
            .iter()
            .filter(|s| s.contains("INSERT INTO [HT_Room_Status]"))
            .collect();
        assert_eq!(inserts.len(), 1); // only the second night
        assert!(inserts[0].contains("'4/25/2026'"));
        assert!(inserts[0].contains("(50237,"));
    }

    #[test]
    fn power_log_note_uses_check_in_template() {
        let s = build_statements(&sample_inputs());
        let pl = s.iter().find(|s| s.contains("HT_POWER_LOG")).unwrap();
        assert!(pl.contains("'เปิดไฟ อัตโนมัติ จากเช็คอิน No.CH26-005231'"));
    }
}
