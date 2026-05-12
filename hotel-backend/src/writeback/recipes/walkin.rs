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
    allocate_checkin_ds_id, allocate_cin_no, allocate_cust_no, allocate_customer_id,
    allocate_room_status_id, LegacyConn,
};
use crate::writeback::constants::{
    power_log_note_check_in, CIN_DEP_STATUS_NONE, CIN_ROOM_STATUS_OCCUPYING, CIN_STATUS_NORMAL,
    CUST_TYPE_MAIN_NORMAL, CUST_TYPE_NORMAL, DEFAULT_OPERATOR, ROOM_STATUS_OCCUPYING,
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
    /// Pre-allocated `HT_CheckIn_Ds.id`. Despite the spike's earlier note,
    /// schema dump (2026-04-26 inspect_schema) confirms this column is NOT
    /// IDENTITY — `int NOT NULL default=NULL`. INSERTs without explicit `id`
    /// fail; the recipe must allocate via TABLOCKX MAX+1 and pass it here.
    pub checkin_ds_id: i32,
    pub nights_calendar: Vec<NaiveDate>,
    /// Optional `Tb_Save_Image.tmp_no` — see [`CreateCheckInPayload::photo_tmp_no`].
    pub photo_tmp_no: Option<&'a str>,
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
    let cust_type_main_q = sql_quote(CUST_TYPE_MAIN_NORMAL);
    let cust_last_change_q = sql_quote(&format_legacy_date(
        crate::writeback::format::bangkok_date(Utc::now()),
    ));
    let stay_start_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_q = sql_quote(&format_legacy_datetime(inputs.stay_end));
    let now_q = sql_quote(&format_legacy_datetime(Utc::now()));
    let occupying_q = sql_quote(CIN_ROOM_STATUS_OCCUPYING);
    let cin_status_q = sql_quote(CIN_STATUS_NORMAL);
    let cust_price_q = sql_quote(CUST_TYPE_NORMAL);
    let dep_status_q = sql_quote(CIN_DEP_STATUS_NONE);
    let room_status_q = sql_quote(ROOM_STATUS_OCCUPYING);
    let power_note = power_log_note_check_in(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    let registry_prefix = guest_prefix_for_country(inputs.guest_country);
    // Legacy capture pattern (verified 10 captured Other_People rows in
    // /tmp/legacy-events-full.log): prefix + ' ' + name + ' '. The
    // trailing space comes from `name + ' ' + name2` when name2 is
    // empty — we emulate the empty-name2 case directly.
    let registry_name = format!("{registry_prefix} {} ", inputs.guest_name_for_registry);
    let registry_name_q = sql_quote(&registry_name);
    let country_q = sql_quote(inputs.guest_country);
    let cust_id = inputs.customer_id_int;
    let price = inputs.price_per_night_baht;
    let total = inputs.price_total_baht;
    let nights = inputs.nights;
    // Cin_Room_ALL carries the room number with a trailing space — the
    // .NET app concatenates room numbers separated by spaces and the
    // single-room form keeps the trailing pad.
    let room_all = format!("{} ", inputs.room_no);
    let room_all_q = sql_quote(&room_all);

    let mut statements: Vec<String> = Vec::with_capacity(8 + inputs.nights_calendar.len());

    // 1. HT_Customers — new customer for the walk-in. 30 columns in the
    //    legacy app's canonical order (verified from
    //    /tmp/legacy-events-full.log captures of Cust_no C21624..C21634).
    //    `Cust_Last_Change` carries today's Bangkok-local date.
    statements.push(format!(
        "INSERT INTO [HT_Customers]([id],[Cust_no],[Cust_perfix],[Cust_name],[Cust_name2],\
         [Cust_Type],[Cust_Email],[Cust_Add_no],[Cust_Add_moo],[Cust_Add_soi],[Cust_Add_road],\
         [Cust_Add_tambon],[Cust_Add_ampore],[Cust_Add_province],[Cust_Add_code],[Cust_Add_tel],\
         [Cust_Add_fax],[Cust_Work_Name],[Cust_Work_no],[Cust_Work_moo],[Cust_Work_soi],\
         [Cust_Work_road],[Cust_Work_tambon],[Cust_Work_ampore],[Cust_Work_province],\
         [Cust_Work_code],[Cust_Work_tel],[Cust_Work_fax],[Cust_Last_Change],[Cust_Type_Main]) \
         VALUES ({cust_id},{cust_no_q},'',{cust_name_q},'',{cust_type_q},'','','','','','','','',\
         '',{cust_phone_q},'','','','','','','','','','','','',{cust_last_change_q},\
         {cust_type_main_q})"
    ));

    // 1b. Tb_Save_Image — link uploaded photo to the new check-in.
    //     The .NET app fires this on every save; UPDATE matches 0 rows
    //     when no photo was uploaded. Skip when no tmp_no was supplied
    //     (matches the legacy app's behavior on photo-less check-ins).
    //
    // Audit H14: also skip when `tmp_no` is an empty / whitespace-only
    // string. Without this filter, `UPDATE Tb_Save_Image … WHERE tmp_no=''`
    // re-stamps every orphan-pending-cleanup row (which the legacy app
    // leaves with `tmp_no=''` after a successful save) with THIS check-in's
    // identifiers — poisoning the photo audit trail for unrelated guests.
    if let Some(tmp_no) = inputs.photo_tmp_no.filter(|s| !s.trim().is_empty()) {
        let tmp_no_q = sql_quote(tmp_no);
        statements.push(format!(
            "update Tb_Save_Image set cin_no={cin_no_q},cust_no={cust_no_q},tmp_no='' \
             where tmp_no={tmp_no_q}"
        ));
    }

    // 2. Mark room occupied — by room_no per spike §3a
    statements.push(format!(
        "update HT_Rooms set room_use='yes' where room_no={room_no_q}"
    ));

    // 3. HT_CheckIn_Ds — 16 cols in legacy order (verified from
    //    /tmp/legacy-events-full.log). `[Cin_dep_status]` is lowercase
    //    d-s in the legacy SQL; `[Dep_by]` is NOT in the column list.
    //    `id` is NOT IDENTITY (schema dump 2026-04-26 confirmed).
    let ds_id = inputs.checkin_ds_id;
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Ds]([id],[Cin_No],[Cin_Room_No],[Cin_Room_Type],[Cin_Room_In],\
         [Cin_Room_Out],[Cin_Room_Status],[Cin_Room_Dep],[Cin_Room_Price],[Cin_Room_Night],\
         [Cin_Room_PriceToTal],[Cin_Room_Pay_Before],[Cin_Room_Pay_Total],[Cin_note],\
         [Cin_dep_status],[Cin_cupon])\
         VALUES( {ds_id},{cin_no_q},{room_no_q},{room_type_q},{stay_start_q},{stay_end_q},\
         {occupying_q},0,{price},{nights},{total},0,0,'',{dep_status_q},0)"
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

    // N+2. HT_CheckIn_H — 19-col canonical legacy order (verified from
    //     16 captures in /tmp/legacy-events-full.log; `Cin_by` precedes
    //     `Cin_Date_in`). Drops the obsolete `[Total_Price_vat]`,
    //     `[Cin_note]`, and `[Cin_Work_number]` columns the audit
    //     identified, and uses the lowercase `[Cin_cust_price]`,
    //     mixed-case `[Cin_Date_Out]`, `[Cin_Type]` casing the legacy
    //     app emits. `[Cin_Type]` is the integer 0; `[Cin_foreign]` is
    //     the string `'False'`. For walk-ins `[Cin_Book_no]` is empty.
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
         [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
         [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],[Cin_Car_id],\
         [Cin_Room_ALL],[Cin_by],[Cin_Date_in],[Cin_Date_Out],[Cin_Type],[Cin_foreign])\
         VALUES({cin_no_q},{now_q},'',{cust_no_q},{cust_price_q},{cin_status_q},\
         {total_2dp},0.00,{total_2dp},0.00,{total_2dp},'','',{room_all_q},{by_q},\
         {stay_start_q},{stay_end_q},0,'False')",
        total_2dp = format!("{total:.2}"),
    ));

    // N+3. HT_Cupon — mark loyalty coupon as printed (spike §3a, walkin/writes.txt:9).
    // Extracted into the shared helper so this recipe and `checkin_to_booking`
    // emit byte-identical SQL.
    statements.push(super::helpers::mark_cupon_printed(inputs.cin_no));

    let _ = NaiveDate::from_ymd_opt(2026, 1, 1); // silence unused-import lint
    let _ = Datelike::year(&Utc::now()); // silence unused-import lint
    let _ = price; // silence: kept on PaymentInputs for future per-night reporting
    statements
}

/// Pick a Thai-or-English personal-prefix string for the
/// `HT_CheckIn_Other_People.Cin_name` field based on the guest's
/// country. Heuristic: country starting with "TH" → `'นาย'` (Thai
/// "Mr."), otherwise `'Mr.'`. The legacy capture shows mixed forms
/// (`'นาย'`, `'น.ส.'`, `'นาง'`, `'Mr.'`, `'Mrs.'`, custom IDs like
/// `'925'`); plumbing the actual `Cust_perfix` through the payload
/// is a separate task — this heuristic is the minimum that
/// preserves the Thai-vs-foreign distinction. Empty country falls
/// back to `'Mr.'` (the safer non-Thai-locale default).
fn guest_prefix_for_country(country: &str) -> &'static str {
    let trimmed = country.trim();
    if !trimmed.is_empty()
        && (trimmed.eq_ignore_ascii_case("TH") || trimmed.to_ascii_uppercase().starts_with("TH"))
    {
        "นาย"
    } else {
        "Mr."
    }
}

/// Execute the walk-in recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &CreateCheckInPayload,
) -> WritebackResult<LegacyIds> {
    // Audit H13: reject NaN/Infinity before any allocation or SQL formatting.
    // `format!("{}", f64::NAN)` emits the literal string `"NaN"`, which would
    // produce invalid SQL like `[Cin_Room_Price]=NaN` and fail mid-transaction
    // leaving partial state. Today the values come from `Money::as_satang()`
    // (always finite) — this is defense-in-depth against a future code path
    // introducing a non-Money f64 source.
    let price_per_night_baht = (payload.price_per_night.as_satang() as f64) / 100.0;
    let price_total_baht = (payload.price_total.as_satang() as f64) / 100.0;
    super::helpers::validate_finite(&[
        ("price_per_night_baht", price_per_night_baht),
        ("price_total_baht", price_total_baht),
    ])?;

    // Allocate IDs under TABLOCKX, in dependency order.
    let cust_no = match payload.legacy_cust_no.as_deref() {
        Some(existing) => existing.to_string(),
        None => allocate_cust_no(conn).await?,
    };
    let cust_id_int = allocate_customer_id(conn).await?;
    let cin_no = allocate_cin_no(conn).await?;
    let room_status_id_base = allocate_room_status_id(conn).await?;
    // Allocate HT_CheckIn_Ds.id under TABLOCKX. Schema dump 2026-04-26
    // confirmed this column is NOT IDENTITY despite the spike's earlier
    // note — INSERT must provide an explicit value.
    let checkin_ds_id = allocate_checkin_ds_id(conn).await?;

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
        price_per_night_baht,
        nights: payload.nights.max(1),
        price_total_baht,
        room_status_id_base,
        nights_calendar,
        checkin_ds_id,
        photo_tmp_no: payload.photo_tmp_no.as_deref(),
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;

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
            checkin_ds_id: 25001,
            photo_tmp_no: None,
        }
    }

    /// Walk-in inputs that mirror the captured legacy walk-in for
    /// `CH26-005242` (Cust C21636, room 407, single night). Used to
    /// assert byte-for-byte parity on the HT_CheckIn_H statement.
    fn capture_inputs() -> WalkInInputs<'static> {
        WalkInInputs {
            cin_no: "CH26-005242",
            cust_no: "C21636",
            customer_id_int: 21636,
            customer_name: "phetreudi",
            customer_phone: None,
            guest_name_for_registry: "phetreudi",
            guest_country: "",
            created_by: "Admin",
            room_no: "407",
            room_type: "เตียงเดี่ยว",
            // Bangkok 4:12:18 PM = UTC 09:12:18 — and the captured
            // Cin_Date is `'4/25/2026 4:12:18 PM'`. We test the
            // stay_start/Cin_Date_in column independently because
            // Cin_Date uses Utc::now() in build_statements.
            stay_start: Utc.with_ymd_and_hms(2026, 4, 25, 9, 12, 18).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            price_per_night_baht: 890.0,
            nights: 1,
            price_total_baht: 890.0,
            room_status_id_base: 50300,
            nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            checkin_ds_id: 25025,
            photo_tmp_no: None,
        }
    }

    /// Byte-level parity for the HT_CheckIn_H statement against the
    /// captured walk-in for `CH26-005242` in
    /// `/tmp/legacy-events-full.log`. We assert the column list +
    /// value tail (Cin_Date is the only field that depends on
    /// `Utc::now()` at build time, so we substring-match around it).
    #[test]
    fn checkin_h_matches_legacy_capture_byte_for_byte() {
        let s = build_statements(&capture_inputs());
        let cin_h = s
            .iter()
            .find(|s| s.contains("[HT_CheckIn_H]"))
            .expect("HT_CheckIn_H INSERT must be emitted");
        let head = "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
                    [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
                    [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],\
                    [Cin_Car_id],[Cin_Room_ALL],[Cin_by],[Cin_Date_in],[Cin_Date_Out],\
                    [Cin_Type],[Cin_foreign])\
                    VALUES('CH26-005242',";
        assert!(
            cin_h.starts_with(head),
            "HT_CheckIn_H must use legacy column order; got:\n{cin_h}"
        );
        let tail = ",'','C21636','ราคาปกติ','ปกติ',890.00,0.00,890.00,0.00,890.00,'','',\
                    '407 ','Admin','4/25/2026 4:12:18 PM','4/26/2026 12:00:00 PM',0,'False')";
        assert!(
            cin_h.ends_with(tail),
            "HT_CheckIn_H value tail must match legacy capture; got:\n{cin_h}"
        );
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
        // Cin_Book_no = '' for walk-ins per spike §3a — third VALUES() position
        // (after cin_no and Cin_Date which carries Utc::now()).
        assert!(
            cin_h.contains(",'','C21607',"),
            "Cin_Book_no must be empty for walkin: {cin_h}"
        );
    }

    #[test]
    fn checkin_h_uses_normal_status_for_active_stay() {
        // The header status is 'ปกติ' (normal), distinct from the
        // HT_CheckIn_Ds.Cin_Room_Status which is 'เข้าพัก'. Verified
        // from /tmp/legacy-events-full.log captures.
        let s = build_statements(&sample_inputs());
        let cin_h = s.iter().find(|s| s.contains("HT_CheckIn_H")).unwrap();
        assert!(cin_h.contains("'ปกติ'"));
        assert!(!cin_h.contains("'เข้าพัก'"));
    }

    #[test]
    fn checkin_h_uses_balance_equal_to_net_per_capture() {
        // Pre-payment, Total_Price_Balance equals Total_Price_Net (not 0).
        let s = build_statements(&sample_inputs());
        let cin_h = s.iter().find(|s| s.contains("HT_CheckIn_H")).unwrap();
        // Net=890.00, Pay=0.00, Balance=890.00
        assert!(cin_h.contains(",890.00,0.00,890.00,"));
    }

    #[test]
    fn checkin_h_room_all_has_trailing_space() {
        let s = build_statements(&sample_inputs());
        let cin_h = s.iter().find(|s| s.contains("HT_CheckIn_H")).unwrap();
        assert!(cin_h.contains("'402 '"));
    }

    #[test]
    fn checkin_h_drops_obsolete_columns_per_audit() {
        let s = build_statements(&sample_inputs());
        let cin_h = s.iter().find(|s| s.contains("HT_CheckIn_H")).unwrap();
        assert!(!cin_h.contains("[Total_Price_vat]"));
        assert!(!cin_h.contains("[Cin_note]"));
        assert!(!cin_h.contains("[Cin_Work_number]"));
    }

    #[test]
    fn checkin_ds_uses_lowercase_dep_status_column_and_no_dep_by() {
        let s = build_statements(&sample_inputs());
        let ds = s.iter().find(|s| s.contains("HT_CheckIn_Ds")).unwrap();
        assert!(ds.contains("[Cin_dep_status]"));
        assert!(!ds.contains("[Cin_Dep_Status]"));
        assert!(!ds.contains("[Dep_by]"));
        assert!(ds.contains("'ไม่เก็บค่ามัดจำ'"));
    }

    #[test]
    fn fires_tb_save_image_update_when_photo_tmp_no_supplied() {
        let mut inputs = sample_inputs();
        inputs.photo_tmp_no = Some("924127");
        let s = build_statements(&inputs);
        let upd = s
            .iter()
            .find(|s| s.starts_with("update Tb_Save_Image"))
            .expect("Tb_Save_Image UPDATE must be emitted when tmp_no supplied");
        assert!(upd.contains("cin_no='CH26-005228'"));
        assert!(upd.contains("cust_no='C21607'"));
        assert!(upd.contains("tmp_no=''"));
        assert!(upd.contains("where tmp_no='924127'"));
    }

    #[test]
    fn skips_tb_save_image_when_photo_tmp_no_none() {
        let s = build_statements(&sample_inputs());
        assert!(!s.iter().any(|s| s.starts_with("update Tb_Save_Image")));
    }

    /// Audit H14: an empty-string `tmp_no` must not fire the Tb_Save_Image
    /// UPDATE. Otherwise `WHERE tmp_no=''` matches every orphan-pending-
    /// cleanup row (legacy app leaves `tmp_no=''` after a successful save)
    /// and re-stamps them with THIS check-in's cin_no / cust_no.
    #[test]
    fn tmp_no_empty_string_skips_tb_save_image_update() {
        let mut inputs = sample_inputs();
        inputs.photo_tmp_no = Some("");
        let s = build_statements(&inputs);
        assert!(
            !s.iter().any(|s| s.starts_with("update Tb_Save_Image")),
            "empty tmp_no must not emit a Tb_Save_Image UPDATE"
        );
    }

    /// Audit H14 extended: whitespace-only `tmp_no` is also treated as
    /// absent — same poisoning blast radius.
    #[test]
    fn tmp_no_whitespace_only_skips_tb_save_image_update() {
        let mut inputs = sample_inputs();
        inputs.photo_tmp_no = Some("   ");
        let s = build_statements(&inputs);
        assert!(!s.iter().any(|s| s.starts_with("update Tb_Save_Image")));
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
    fn registry_name_uses_mr_prefix_for_non_thai_country() {
        let s = build_statements(&sample_inputs());
        let people = s
            .iter()
            .find(|s| s.contains("HT_CheckIn_Other_People"))
            .unwrap();
        // Trailing space comes from `prefix + ' ' + name + ' '` — matches
        // captured pattern (e.g. 'Mr. Thomas Meininghaus  ').
        assert!(people.contains("'Mr. SPIKE TEST WALKIN '"));
    }

    #[test]
    fn registry_name_uses_thai_prefix_when_country_is_th() {
        let mut inputs = sample_inputs();
        inputs.guest_country = "TH";
        inputs.guest_name_for_registry = "อุทัย สุขผล";
        let s = build_statements(&inputs);
        let people = s
            .iter()
            .find(|s| s.contains("HT_CheckIn_Other_People"))
            .unwrap();
        assert!(people.contains("'นาย อุทัย สุขผล '"));
    }

    #[test]
    fn checkin_ds_includes_explicit_id_column() {
        // Schema dump 2026-04-26 confirmed HT_CheckIn_Ds.id is NOT IDENTITY
        // (`int NOT NULL, default=NULL`). The earlier "IDENTITY, omit" comment
        // was wrong — INSERT must provide an explicit value or fail.
        let s = build_statements(&sample_inputs());
        let ds = s.iter().find(|s| s.contains("HT_CheckIn_Ds")).unwrap();
        assert!(ds.contains("[id]"), "[id] must be in column list: {ds}");
        assert!(ds.contains("[Cin_No]"));
        // Legacy capture begins `VALUES( {id},` — note the leading space.
        assert!(
            ds.contains("VALUES( 25001,"),
            "[id] value must be the pre-allocated checkin_ds_id (25001 in sample): {ds}"
        );
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
        // First positional value is the explicit id (21607). Legacy
        // capture wraps the column-list with `) VALUES (` (trailing
        // space before the open paren).
        assert!(cust.contains("VALUES (21607,'C21607',"));
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

    /// Audit H13: walk-in `execute()` must reject NaN before any SQL is
    /// formatted. `format!("{}", f64::NAN)` would emit literal `"NaN"` into
    /// `[Cin_Room_Price]=NaN` and fail the whole transaction. This test pins
    /// the labels we wire up so the error message stays operator-grep-able.
    #[test]
    fn validate_finite_blocks_nan_in_walkin_execute_inputs() {
        let result = super::super::helpers::validate_finite(&[
            ("price_per_night_baht", f64::NAN),
            ("price_total_baht", 890.0),
        ]);
        let err = result.expect_err("NaN must be rejected");
        assert!(err.to_string().contains("price_per_night_baht"));
    }

    #[test]
    fn validate_finite_blocks_infinity_in_walkin_execute_inputs() {
        let result = super::super::helpers::validate_finite(&[
            ("price_per_night_baht", 890.0),
            ("price_total_baht", f64::INFINITY),
        ]);
        let err = result.expect_err("Infinity must be rejected");
        assert!(err.to_string().contains("price_total_baht"));
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
