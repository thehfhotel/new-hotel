//! `CreateCheckIn` walk-in recipe — spike `findings.md` §3a.
//!
//! Walk-in (no prior booking): 7 INSERTs + 3 UPDATEs across 7 tables.
//! Allocates `Cust_no`, `Cin_no`, `HT_Customers.id`, and `HT_Room_Status.id`
//! under TABLOCKX.
//!
//! Reference SQL (verbatim from `walkin-20260424-095304/writes.txt`):
//!
//! ```text
//! 1. INSERT INTO [HT_Customers]([id],[Cust_no],[Cust_name],[Cust_name2],[Cust_Type], …)
//! 2. update Tb_Save_Image set cin_no='CH26-005228', cust_no='C21607', tmp_no=''
//!      where tmp_no='924127'                            -- photo link (no-op if no photo)
//! 3. update HT_Rooms set room_use='yes' where room_no='402'
//! 4. INSERT INTO [HT_CheckIn_Ds]([id],[Cin_No],[Cin_Room_No],[Cin_Room_Type],
//!      [Cin_Room_In],[Cin_Room_Out],[Cin_Room_Status='เข้าพัก'],
//!      [Cin_Room_Price],[Cin_Room_Night],[Cin_Room_PriceToTal], …)
//! 5. INSERT INTO [HT_POWER_LOG]([ROOM_NO],[ROOM_POWER_START],[ROOM_POWER_START_BY],
//!      [ROOM_POWER_END_BY=''],[ROOM_POWER_NOTE='เปิดไฟ อัตโนมัติ จากเช็คอิน No.CH26-005228'])
//! 6. INSERT INTO [HT_Room_Status](id, room_no, room_date, room_status='เข้าพัก',
//!      room_Details=cust_name, room_CheckIn_No, room_date_oa)
//! 7. INSERT INTO [HT_CheckIn_Other_People]([Cin_no],[Cin_name='Mr. NAME'],[Cin_contry])
//! 8. INSERT INTO [HT_CheckIn_H](Cin_no, Cin_Date, Cin_Book_no=NULL, Cin_cust_no, …)
//! 9. update HT_Cupon set cupon_print=1 where cupon_cin_no='CH26-005228'
//! ```
//!
//! Spike §3a critical findings:
//! - `Cin_Book_no` is NULL/empty for walk-ins — the discriminator vs §3d.
//! - `Cin_Room_Status` initial value is `'เข้าพัก'` (Thai: occupying).
//! - Person prefix is `'Mr. '` or `'นาย '` — both observed; we use `'Mr. '` as default.
//! - `Tb_Save_Image` UPDATE is a no-op when no photo was uploaded — we skip
//!   it (matches the legacy app's behavior on photo-less check-ins).
//! - `Cin_Work_number` (TM.30 batch) is assigned ~5s later by the legacy app's
//!   async batch job. Our writeback doesn't allocate it — the .NET app will
//!   set it when it next opens the check-in (or it stays 0 until then).

use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::outbox::intent::CreateCheckInPayload;
use crate::writeback::allocate::{
    allocate_cin_no, allocate_cust_no, allocate_customer_id, allocate_room_status_id, LegacyConn,
};
use crate::writeback::constants::{
    power_log_note_check_in, CIN_ROOM_STATUS_OCCUPYING, CUST_TYPE_MAIN_INDIVIDUAL,
    CUST_TYPE_NORMAL, DEFAULT_OPERATOR, ROOM_STATUS_OCCUPYING,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    date_to_ole_serial, format_legacy_date, format_legacy_datetime, sql_quote,
};

/// Inputs for the walk-in recipe.
#[derive(Debug, Clone)]
pub struct WalkInInputs<'a> {
    pub cin_no: &'a str,
    pub cust_no: &'a str,
    /// `HT_Customers.id` — required because the column is no longer IDENTITY
    /// (per spike §2). Caller allocates this with TABLOCKX.
    pub customer_id_int: i32,
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
    /// First `HT_Room_Status.id` to use for night INSERTs.
    pub room_status_id_base: i32,
    pub nights_calendar: Vec<NaiveDate>,
}

/// Build the statements for a walk-in. PURE — no I/O.
pub fn build_statements(inputs: &WalkInInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let by_q = sql_quote(inputs.created_by);
    let room_no_q = sql_quote(inputs.room_no);
    let room_type_q = sql_quote(inputs.room_type);
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_phone_q = sql_quote(inputs.customer_phone.unwrap_or(""));
    let cust_type_q = sql_quote(CUST_TYPE_NORMAL);
    let cust_type_main_q = sql_quote(CUST_TYPE_MAIN_INDIVIDUAL);
    let stay_start_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_q = sql_quote(&format_legacy_datetime(inputs.stay_end));
    let now_q = sql_quote(&format_legacy_datetime(Utc::now()));
    let occupying_q = sql_quote(CIN_ROOM_STATUS_OCCUPYING);
    let room_status_q = sql_quote(ROOM_STATUS_OCCUPYING);
    let power_note = power_log_note_check_in(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    // Spike §3a: prefix is 'Mr. ' (English) or 'นาย ' (Thai). We default to
    // 'Mr. ' because both forms are observed and Mr. is non-Thai-locale-safe.
    let registry_name = format!("Mr. {}", inputs.guest_name_for_registry);
    let registry_name_q = sql_quote(&registry_name);
    let country_q = sql_quote(inputs.guest_country);
    let cust_id = inputs.customer_id_int;
    let price = inputs.price_per_night_baht;
    let total = inputs.price_total_baht;
    let nights = inputs.nights;

    let mut statements: Vec<String> = Vec::with_capacity(7 + inputs.nights_calendar.len());

    // 1. HT_Customers — new customer for the walk-in
    statements.push(format!(
        "INSERT INTO [HT_Customers]([id],[Cust_no],[Cust_perfix],[Cust_name],[Cust_name2],\
         [Cust_sex],[Cust_IDcard],[Cust_Type],[Cust_Email],[Cust_Add_no],[Cust_Add_moo],\
         [Cust_Add_soi],[Cust_Add_road],[Cust_Add_tambon],[Cust_Add_ampore],[Cust_Add_province],\
         [Cust_Add_code],[Cust_Add_tel],[Cust_Add_fax],[Cust_Work_Name],[Cust_Work_no],\
         [Cust_Work_moo],[Cust_Work_soi],[Cust_Work_road],[Cust_Work_tambon],[Cust_Work_ampore],\
         [Cust_Work_province],[Cust_Work_code],[Cust_Work_tel],[Cust_Work_fax],[Cust_Type_Main],\
         [Cust_Contry],[Cust_Work_Tax])\
         VALUES({cust_id},{cust_no_q},'',{cust_name_q},'','','',{cust_type_q},'','','','','',\
         '','','','',{cust_phone_q},'','','','','','','','','','','',\
         {cust_type_main_q},{country_q},'')"
    ));

    // 2. Mark room occupied — by room_no per spike §3a
    statements.push(format!(
        "update HT_Rooms set room_use='yes' where room_no={room_no_q}"
    ));

    // 3. HT_CheckIn_Ds — id is IDENTITY, omit from column list per spike §2
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Ds]([Cin_No],[Cin_Room_No],[Cin_Room_Type],[Cin_Room_In],\
         [Cin_Room_Out],[Cin_Room_Status],[Cin_Room_Dep],[Cin_Room_Price],[Cin_Room_Night],\
         [Cin_Room_PriceToTal],[Cin_Room_Pay_Before],[Cin_Room_Pay_Total],[Cin_note],\
         [Cin_Dep_Status],[Dep_by],[Cin_cupon])\
         VALUES({cin_no_q},{room_no_q},{room_type_q},{stay_start_q},{stay_end_q},\
         {occupying_q},0,{price},{nights},{total},0,0,'','','',0)"
    ));

    // 4. HT_POWER_LOG — lights on with check-in note
    statements.push(format!(
        "INSERT INTO [HT_POWER_LOG]([ROOM_NO],[ROOM_POWER_START],[ROOM_POWER_START_BY],\
         [ROOM_POWER_END_BY],[ROOM_POWER_NOTE],[ROOM_POWER_NOTE2])\
         VALUES({room_no_q},GETDATE(),{by_q},'',{power_note_q},'')"
    ));

    // 5..N. HT_Room_Status — one row per calendar night
    for (i, day) in inputs.nights_calendar.iter().enumerate() {
        let id = inputs.room_status_id_base + i as i32;
        let date_q = sql_quote(&format_legacy_date(*day));
        let oa = date_to_ole_serial(*day) as i64;
        statements.push(format!(
            "INSERT INTO [HT_Room_Status]([id],[room_no],[room_date],[room_status],\
             [room_Details],[room_CheckIn_No],[room_date_oa])\
             VALUES({id},{room_no_q},{date_q},{room_status_q},{cust_name_q},{cin_no_q},{oa})"
        ));
    }

    // N+1. HT_CheckIn_Other_People — TM.30 primary guest row
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])\
         VALUES({cin_no_q},{registry_name_q},{country_q})"
    ));

    // N+2. HT_CheckIn_H — header (Cin_Book_no is empty for walk-ins per §3a)
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
         [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
         [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],[Cin_Car_id],\
         [Cin_Room_ALL],[Total_Price_vat],[Cin_by],[Cin_Date_in],[Cin_Date_out],[Cin_type],\
         [Cin_note],[Cin_foreign],[Cin_Work_number])\
         VALUES({cin_no_q},{now_q},'',{cust_no_q},'',{occupying_q},{total},0,{total},0,{total},\
         '','',{room_no_q},0,{by_q},{stay_start_q},{stay_end_q},0,'','',0)"
    ));

    // N+3. HT_Cupon — mark loyalty coupon as printed (spike §3a, walkin/writes.txt:9).
    // Extracted into the shared helper so this recipe and `checkin_to_booking`
    // emit byte-identical SQL.
    statements.push(super::helpers::mark_cupon_printed(inputs.cin_no));

    let _ = NaiveDate::from_ymd_opt(2026, 1, 1); // silence unused-import lint
    let _ = Datelike::year(&Utc::now()); // silence unused-import lint
    statements
}

/// Execute the walk-in recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &CreateCheckInPayload,
) -> WritebackResult<LegacyIds> {
    // Allocate IDs under TABLOCKX, in dependency order.
    let cust_no = match payload.legacy_cust_no.as_deref() {
        Some(existing) => existing.to_string(),
        None => allocate_cust_no(conn).await?,
    };
    let cust_id_int = allocate_customer_id(conn).await?;
    let cin_no = allocate_cin_no(conn).await?;
    let room_status_id_base = allocate_room_status_id(conn).await?;

    let nights_calendar = enumerate_calendar_nights(payload.stay.start, payload.stay.end);

    let inputs = WalkInInputs {
        cin_no: &cin_no,
        cust_no: &cust_no,
        customer_id_int: cust_id_int,
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
    // Capture SCOPE_IDENTITY() right after the HT_CheckIn_Ds INSERT so the
    // writeback worker's mark_done can back-populate
    // ht_checkins.legacy_checkin_ds_id (used by ExtendStay / CheckOut).
    let checkin_ds_id =
        super::execute_capturing_identity_at(conn, &statements, "INSERT INTO [HT_CheckIn_Ds]")
            .await?;

    let _ = DEFAULT_OPERATOR; // silence unused-import lint
    let mut ids = LegacyIds::new()
        .with_cin_no(cin_no.clone())
        .with_cust_no(cust_no.clone())
        .with_room_no(payload.room_no.clone())
        .with_checkin_ds_id(checkin_ds_id);
    ids.extra
        .insert("customer_id_int".into(), serde_json::Value::from(cust_id_int));
    ids.extra
        .insert("room_status_id_base".into(), serde_json::Value::from(room_status_id_base));
    Ok(ids)
}

/// Enumerate calendar nights spanning `[stay_start, stay_end)`.
/// Mirrors the helper in `booking_create.rs` (kept here to avoid a cross-recipe
/// dependency — recipes should be self-contained per spec).
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

    fn sample_inputs() -> WalkInInputs<'static> {
        WalkInInputs {
            cin_no: "CH26-005228",
            cust_no: "C21607",
            customer_id_int: 21607,
            customer_name: "SPIKE TEST WALKIN",
            customer_phone: None,
            guest_name_for_registry: "SPIKE TEST WALKIN",
            guest_country: "",
            created_by: "Admin",
            room_no: "402",
            room_type: "Standard",
            stay_start: Utc.with_ymd_and_hms(2026, 4, 24, 9, 56, 20).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 25, 11, 59, 59).unwrap(),
            price_per_night_baht: 890.0,
            nights: 1,
            price_total_baht: 890.0,
            room_status_id_base: 50230,
            nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 24).unwrap()],
        }
    }

    #[test]
    fn produces_seven_or_more_statements() {
        // 1 HT_Customers + 1 HT_Rooms + 1 HT_CheckIn_Ds + 1 HT_POWER_LOG +
        // N HT_Room_Status + 1 HT_CheckIn_Other_People + 1 HT_CheckIn_H
        let s = build_statements(&sample_inputs());
        assert!(s.len() >= 7);
    }

    #[test]
    fn checkin_h_has_empty_book_no_for_walkin() {
        let s = build_statements(&sample_inputs());
        let cin_h = s.iter().find(|s| s.contains("HT_CheckIn_H")).unwrap();
        // Cin_Book_no = '' for walk-ins per spike §3a
        // Position after Cin_Date: VALUES(cin_no, now, '', cust_no, ...)
        assert!(cin_h.contains(",'',"));
    }

    #[test]
    fn cin_room_status_uses_thai_occupying_literal() {
        let s = build_statements(&sample_inputs());
        let ds = s.iter().find(|s| s.contains("HT_CheckIn_Ds")).unwrap();
        assert!(ds.contains("'เข้าพัก'"));
    }

    #[test]
    fn room_status_uses_thai_occupying_literal() {
        let s = build_statements(&sample_inputs());
        let rs = s.iter().find(|s| s.contains("HT_Room_Status")).unwrap();
        assert!(rs.contains("'เข้าพัก'"));
    }

    #[test]
    fn power_log_note_uses_check_in_template() {
        let s = build_statements(&sample_inputs());
        let pl = s.iter().find(|s| s.contains("HT_POWER_LOG")).unwrap();
        assert!(pl.contains("'เปิดไฟ อัตโนมัติ จากเช็คอิน No.CH26-005228'"));
    }

    #[test]
    fn marks_room_occupied_by_room_no_not_id() {
        let s = build_statements(&sample_inputs());
        let upd = s
            .iter()
            .find(|s| s.starts_with("update HT_Rooms"))
            .unwrap();
        assert!(upd.contains("room_no='402'"));
        assert!(!upd.contains("where id="));
    }

    #[test]
    fn registry_name_uses_mr_prefix() {
        let s = build_statements(&sample_inputs());
        let people = s
            .iter()
            .find(|s| s.contains("HT_CheckIn_Other_People"))
            .unwrap();
        assert!(people.contains("'Mr. SPIKE TEST WALKIN'"));
    }

    #[test]
    fn checkin_ds_omits_id_column_for_identity() {
        let s = build_statements(&sample_inputs());
        let ds = s.iter().find(|s| s.contains("HT_CheckIn_Ds")).unwrap();
        // id is IDENTITY (spike §2) — verify it's not in the column list
        assert!(!ds.contains("[id]"));
        assert!(ds.contains("[Cin_No]"));
    }

    #[test]
    fn room_status_ids_are_sequential() {
        let mut inputs = sample_inputs();
        inputs.nights_calendar = vec![
            NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
        ];
        let s = build_statements(&inputs);
        let rs: Vec<&String> = s.iter().filter(|s| s.contains("HT_Room_Status")).collect();
        assert_eq!(rs.len(), 2);
        assert!(rs[0].contains("(50230,"));
        assert!(rs[1].contains("(50231,"));
    }

    #[test]
    fn customer_id_is_passed_explicitly() {
        let s = build_statements(&sample_inputs());
        let cust = s.iter().find(|s| s.contains("HT_Customers")).unwrap();
        // First positional value is the explicit id (21607)
        assert!(cust.contains("VALUES(21607,'C21607',"));
    }

    #[test]
    fn emits_cupon_print_update_after_checkin_h() {
        // Spike §3a `walkin/writes.txt:9` — every walk-in fires this.
        let s = build_statements(&sample_inputs());
        let cupon = s
            .iter()
            .find(|s| s.starts_with("update HT_Cupon"))
            .expect("HT_Cupon mark-printed UPDATE must be emitted");
        assert!(cupon.contains("cupon_print=1"));
        assert!(cupon.contains("cupon_cin_no='CH26-005228'"));
    }

    #[test]
    fn enumerate_calendar_nights_handles_overnight() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 25, 11, 59, 59).unwrap(),
        );
        assert_eq!(nights, vec![NaiveDate::from_ymd_opt(2026, 4, 24).unwrap()]);
    }
}
