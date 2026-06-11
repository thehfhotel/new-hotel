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

use chrono::{DateTime, NaiveDate, Utc};

use crate::outbox::intent::{CreateCheckInPayload, RoomLine};
use crate::writeback::allocate::{
    allocate_checkin_ds_id, allocate_cin_no, allocate_room_status_id, LegacyConn,
};
use crate::writeback::constants::{
    power_log_note_check_in, BOOK_STATUS_OCCUPYING, CIN_DEP_STATUS_NONE,
    CIN_ROOM_STATUS_OCCUPYING, CIN_STATUS_NORMAL, CUST_TYPE_NORMAL, DEFAULT_OPERATOR,
    ROOM_STATUS_OCCUPYING,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{
    date_to_ole_serial, enumerate_calendar_nights, format_legacy_date, format_legacy_datetime,
    sql_quote,
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
    /// Pre-allocated `HT_CheckIn_Ds.id`. NOT IDENTITY (schema dump
    /// 2026-04-26) — caller must allocate via TABLOCKX MAX+1.
    ///
    /// Multi-room (Track B4): when `room_lines` is non-empty this is
    /// the id for the FIRST room's `HT_CheckIn_Ds` row. Subsequent
    /// rooms use sequential ids (`checkin_ds_id + 1`, +2, …).
    pub checkin_ds_id: i32,
    /// Optional `Tb_Save_Image.tmp_no` — see [`CreateCheckInPayload::photo_tmp_no`].
    pub photo_tmp_no: Option<&'a str>,
    /// "Now" timestamp threaded in by `execute()` so `build_statements`
    /// stays PURE — Wave 5b item 4. Drives `HT_CheckIn_H.Cin_Date`. Tests
    /// pass a fixed instant for exact byte-parity assertions.
    pub created_at: DateTime<Utc>,
    /// Track B4 — per-room slice from `ht_checkin_rooms`. Empty ⇒
    /// recipe emits the legacy single-room shape. Non-empty ⇒ recipe
    /// emits one `HT_CheckIn_Ds` row per slice entry. See
    /// [`crate::writeback::recipes::walkin::WalkInInputs::room_lines`]
    /// for the full back-compat / cardinality contract.
    pub room_lines: Vec<RoomLine>,
}

/// Build statements for a check-in linked to a booking. PURE — no I/O.
pub fn build_statements(inputs: &CheckInToBookingInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let book_id_q = sql_quote(inputs.book_id);
    let by_q = sql_quote(inputs.created_by);
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_phone_q = sql_quote(inputs.customer_phone.unwrap_or(""));
    let stay_start_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_q = sql_quote(&format_legacy_datetime(inputs.stay_end));
    let now_q = sql_quote(&format_legacy_datetime(inputs.created_at));
    let occupying_q = sql_quote(CIN_ROOM_STATUS_OCCUPYING);
    let cin_status_q = sql_quote(CIN_STATUS_NORMAL);
    let cust_price_q = sql_quote(CUST_TYPE_NORMAL);
    let dep_status_q = sql_quote(CIN_DEP_STATUS_NONE);
    let room_status_q = sql_quote(ROOM_STATUS_OCCUPYING);
    let book_status_q = sql_quote(BOOK_STATUS_OCCUPYING);
    let power_note = power_log_note_check_in(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    let registry_prefix = super::helpers::guest_prefix_for_country(inputs.guest_country);
    let registry_name = format!("{registry_prefix} {} ", inputs.guest_name_for_registry);
    let registry_name_q = sql_quote(&registry_name);
    let country_q = sql_quote(inputs.guest_country);
    // Wave 6 LOW item 4: pre-format money to 2dp for consistency with the
    // HT_CheckIn_H VALUES and the rest of the recipe corpus.
    let total = format!("{:.2}", inputs.price_total_baht);

    // Track B4 — derive the per-room slice. Empty ⇒ legacy single-room
    // path falls back to the top-level fields (byte-identical pre-B4
    // SQL); non-empty ⇒ N rooms apportioned out via the same loops.
    let lines = effective_room_lines(inputs);
    let room_all = lines
        .iter()
        .map(|l| format!("{} ", l.room_no))
        .collect::<String>();
    let room_all_q = sql_quote(&room_all);

    let mut statements: Vec<String> = Vec::with_capacity(
        9 + inputs.nights_calendar.len() * lines.len() + lines.len(),
    );

    // 1. UPDATE HT_Customers — narrow re-save (2026-06-11 audit P1-9).
    //
    //    iHOTEL's re-save writes back the values it LOADED from the row,
    //    so its end state preserves every profile field. Our payload only
    //    carries name / phone / country — an earlier revision mirrored the
    //    .NET app's 31-field UPDATE shape but filled the ~25 fields the
    //    payload doesn't carry with `''` (email, addresses, ID-card,
    //    prefix, sex, …), erasing receptionist-entered iHOTEL data on
    //    every booking-linked check-in. It also force-reset the
    //    Cust_Type / Cust_Type_main tier columns to hardcoded defaults.
    //
    //    Fix: SET only what the payload actually carries. Fields the
    //    payload doesn't carry are simply absent from the SET list — the
    //    existing MSSQL values survive, which is exactly the end state
    //    iHOTEL's load-then-resave produces. Phone is skipped when the
    //    payload has none (None ⇒ preserve, not blank); country is
    //    skipped when empty for the same reason. Capture conventions kept:
    //    two spaces after `SET`, lowercase `where`.
    let mut cust_sets = vec![format!("[Cust_name]={cust_name_q}")];
    if inputs.customer_phone.is_some() {
        cust_sets.push(format!("[Cust_Add_tel]={cust_phone_q}"));
    }
    if !inputs.guest_country.is_empty() {
        cust_sets.push(format!("[Cust_Contry]={country_q}"));
    }
    statements.push(format!(
        "UPDATE [HT_Customers] SET  {sets} where Cust_no={cust_no_q}",
        sets = cust_sets.join(","),
    ));

    // 1b. Tb_Save_Image — link uploaded photo to the new check-in.
    //     UPDATE matches 0 rows when no photo was uploaded (mirrors the
    //     legacy app's behavior). Skip when no tmp_no was supplied.
    //
    // Audit H14: also skip when `tmp_no` is an empty / whitespace-only
    // string. `WHERE tmp_no=''` would re-stamp every orphan-pending-cleanup
    // row (which the legacy app leaves with `tmp_no=''` after a successful
    // save) with THIS check-in's identifiers — poisoning the photo audit
    // trail for unrelated guests.
    if let Some(tmp_no) = inputs.photo_tmp_no.filter(|s| !s.trim().is_empty()) {
        let tmp_no_q = sql_quote(tmp_no);
        statements.push(format!(
            "update Tb_Save_Image set cin_no={cin_no_q},cust_no={cust_no_q},tmp_no='' \
             where tmp_no={tmp_no_q}"
        ));
    }

    // Track B4 — per-room fan-out. Single-room (`lines.len() == 1`)
    // collapses to the legacy pre-B4 shape byte-for-byte; multi-room
    // fires the same statement template once per room.

    // 2. HT_POWER_LOG — lights on per room
    for line in &lines {
        let line_room_no_q = sql_quote(&line.room_no);
        statements.push(format!(
            "INSERT INTO [HT_POWER_LOG]([ROOM_NO],[ROOM_POWER_START],[ROOM_POWER_START_BY],\
             [ROOM_POWER_END_BY],[ROOM_POWER_NOTE],[ROOM_POWER_NOTE2])\
             VALUES({line_room_no_q},GETDATE(),{by_q},'',{power_note_q},'')"
        ));
    }

    // 3. HT_CheckIn_Ds — one row per room. 16-col canonical legacy
    //    order (verified from /tmp/legacy-events-full.log).
    //    `[Cin_dep_status]` lowercase d-s, no `[Dep_by]`. `id` is NOT
    //    IDENTITY (schema dump 2026-04-26 confirmed).
    for (room_idx, line) in lines.iter().enumerate() {
        let ds_id = inputs.checkin_ds_id + room_idx as i32;
        let line_room_no_q = sql_quote(&line.room_no);
        let line_room_type_q = sql_quote(&line.room_type);
        let line_price = format!("{:.2}", line.price_per_night);
        let line_total = format!("{:.2}", line.room_total);
        let line_nights = line.nights;
        // Per-room Thai status literal — preserve verbatim.
        let line_status_q = if line.room_status.is_empty() {
            occupying_q.clone()
        } else {
            sql_quote(&line.room_status)
        };
        statements.push(format!(
            "INSERT INTO [HT_CheckIn_Ds]([id],[Cin_No],[Cin_Room_No],[Cin_Room_Type],[Cin_Room_In],\
             [Cin_Room_Out],[Cin_Room_Status],[Cin_Room_Dep],[Cin_Room_Price],[Cin_Room_Night],\
             [Cin_Room_PriceToTal],[Cin_Room_Pay_Before],[Cin_Room_Pay_Total],[Cin_note],\
             [Cin_dep_status],[Cin_cupon])\
             VALUES( {ds_id},{cin_no_q},{line_room_no_q},{line_room_type_q},{stay_start_q},{stay_end_q},\
             {line_status_q},0,{line_price},{line_nights},{line_total},0,0,'',{dep_status_q},0)"
        ));
    }

    // 4. Mark each room occupied
    for line in &lines {
        let line_room_no_q = sql_quote(&line.room_no);
        statements.push(format!(
            "update HT_Rooms set room_use='yes' where room_no={line_room_no_q}"
        ));
    }

    // 5. UPDATE existing HT_Room_Status row(s) for night 0 — §3d: no
    //    Cin_no filter, the .NET app overwrites by (room_date, room_no).
    //    We do the same for parity, once per room.
    if let Some(first_day) = inputs.nights_calendar.first() {
        let first_date_q = sql_quote(&format_legacy_date(*first_day));
        for line in &lines {
            let line_room_no_q = sql_quote(&line.room_no);
            statements.push(format!(
                "update [HT_Room_Status] SET  [room_status]={room_status_q},\
                 [room_Details]={cust_name_q},[room_CheckIn_No]={cin_no_q} \
                 where room_date={first_date_q} and room_no={line_room_no_q}"
            ));
        }
    }

    // 6..N. HT_Room_Status for additional nights × rooms (skip night 0
    //       — handled by the UPDATE block above per spike §3d).
    //
    // Wave 3 followup: each night must be an UPSERT, not a plain INSERT
    // (booking_create pre-inserts night rows). The
    // `IF EXISTS … UPDATE … ELSE INSERT` form matches the legacy app's
    // "upsert per night" semantics (`COMPAT_CHEATSHEET.md:347-348`) and
    // makes the recipe safe for both pre-existing and net-new nights.
    //
    // Multi-room id-base layout: `room_status_id_base + (room_idx *
    // (nights-1)) + (night_idx - 1)`. The `(nights-1)` factor and
    // `-1` offset reflect that night 0 is handled by the UPDATE above
    // and consumes no id from the allocator.
    let extra_nights = inputs.nights_calendar.len().saturating_sub(1) as i32;
    for (room_idx, line) in lines.iter().enumerate() {
        let line_room_no_q = sql_quote(&line.room_no);
        for (night_idx, day) in inputs.nights_calendar.iter().enumerate().skip(1) {
            let id = inputs.room_status_id_base
                + (room_idx as i32 * extra_nights)
                + (night_idx as i32 - 1);
            let date_q = sql_quote(&format_legacy_date(*day));
            let oa = date_to_ole_serial(*day) as i64;
            statements.push(format!(
                "IF EXISTS (SELECT 1 FROM [HT_Room_Status] WHERE room_date={date_q} \
                 AND room_no={line_room_no_q}) \
                 UPDATE [HT_Room_Status] SET [room_status]={room_status_q},\
                 [room_Details]={cust_name_q},[room_CheckIn_No]={cin_no_q} \
                 WHERE room_date={date_q} AND room_no={line_room_no_q} \
                 ELSE \
                 INSERT INTO [HT_Room_Status]([id],[room_no],[room_date],[room_status],\
                 [room_Details],[room_CheckIn_No],[room_date_oa])\
                 VALUES({id},{line_room_no_q},{date_q},{room_status_q},{cust_name_q},{cin_no_q},{oa})"
            ));
        }
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

    // 10. HT_CheckIn_H — 19-col canonical legacy order (verified from
    //     /tmp/legacy-events-full.log captures of CH26-005236, line
    //     3988). Cin_Book_no carries the linked booking. Drops the
    //     obsolete `[Total_Price_vat]`, `[Cin_note]`, and
    //     `[Cin_Work_number]` columns; uses the lowercase
    //     `[Cin_cust_price]`, mixed-case `[Cin_Date_Out]` and
    //     `[Cin_Type]` casing the legacy app emits. `[Cin_Type]=0`
    //     (integer); `[Cin_foreign]='False'` (literal string).
    // Wave 6 LOW item 4: `total` is the pre-formatted 2dp string.
    let total_2dp = total.as_str();
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
         [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
         [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],[Cin_Car_id],\
         [Cin_Room_ALL],[Cin_by],[Cin_Date_in],[Cin_Date_Out],[Cin_Type],[Cin_foreign])\
         VALUES({cin_no_q},{now_q},{book_id_q},{cust_no_q},{cust_price_q},{cin_status_q},\
         {total_2dp},0.00,{total_2dp},0.00,{total_2dp},'','',{room_all_q},{by_q},\
         {stay_start_q},{stay_end_q},0,'False')",
    ));

    // 11. HT_Cupon — mark loyalty coupon as printed (spike §3d,
    //     booking-checkin/writes.txt:39). Shared helper with `walkin` so the
    //     literal SQL stays identical.
    statements.push(super::helpers::mark_cupon_printed(inputs.cin_no));

    statements
}

/// Track B4 — derive the effective per-room slice (mirrors
/// [`crate::writeback::recipes::walkin::effective_room_lines`]). Empty
/// `inputs.room_lines` synthesizes a single-line slice from the top-
/// level fields so single-room callers see no byte-level diff.
fn effective_room_lines(inputs: &CheckInToBookingInputs<'_>) -> Vec<RoomLine> {
    if !inputs.room_lines.is_empty() {
        return inputs.room_lines.clone();
    }
    vec![RoomLine {
        room_no: inputs.room_no.to_string(),
        room_type: inputs.room_type.to_string(),
        price_per_night: inputs.price_per_night_baht,
        nights: inputs.nights,
        room_total: inputs.price_total_baht,
        room_status: String::new(),
        legacy_ds_id: None,
    }]
}

/// Execute the check-in-to-booking recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &CreateCheckInPayload,
    book_id: &str,
) -> WritebackResult<LegacyIds> {
    // Audit H13: reject NaN/Infinity before any allocation or SQL formatting.
    // Same rationale as `walkin::execute` — `format!("{}", f64::NAN)` emits the
    // literal `"NaN"`, which would fail mid-transaction with partial state.
    let price_per_night_baht = (payload.price_per_night.as_satang() as f64) / 100.0;
    let price_total_baht = (payload.price_total.as_satang() as f64) / 100.0;
    super::helpers::validate_finite(&[
        ("price_per_night_baht", price_per_night_baht),
        ("price_total_baht", price_total_baht),
    ])?;

    // Wave 6 LOW item 5: hard-validate `nights >= 1` instead of silently
    // clamping via `.max(1)`. The service layer should reject this at
    // enqueue time; this is defense-in-depth so a caller bug surfaces as a
    // `Recipe` error before any TABLOCKX allocation runs.
    if payload.nights < 1 {
        return Err(WritebackError::Recipe(format!(
            "CheckIn-to-booking: nights must be >= 1 (got {})",
            payload.nights
        )));
    }

    let cust_no = payload.legacy_cust_no.clone().ok_or_else(|| {
        WritebackError::Recipe(
            "CheckIn-to-booking requires legacy_cust_no in payload (§3d: customer already exists)"
                .into(),
        )
    })?;
    let cin_no = allocate_cin_no(conn).await?;
    let room_status_id_base = allocate_room_status_id(conn).await?;
    // Track B4 — first ds id under TABLOCKX. The lock is held for the
    // full transaction so sequential ids checkin_ds_id_base + room_idx
    // are reserved for the rest of the per-room fan-out (no second
    // round-trip needed).
    let checkin_ds_id_base = allocate_checkin_ds_id(conn).await?;

    // Wave 6 LOW item 6: empty range surfaces as error rather than silently
    // injecting a phantom night; cap-truncate logs WARN.
    let nights_calendar = enumerate_calendar_nights(payload.stay.start, payload.stay.end)?;

    // Wave 5b item 4: capture `Utc::now()` once at the entry to `execute()`
    // so `build_statements` is purely a function of its inputs.
    let created_at = Utc::now();

    let inputs = CheckInToBookingInputs {
        cin_no: &cin_no,
        cust_no: &cust_no,
        book_id,
        customer_name: &payload.guest_name_for_registry,
        // Wave 5a item 1: preserve booking-time phone. Prior code passed
        // `None` unconditionally, which mapped to `[Cust_Add_tel]=''` and
        // wiped the phone on every booking-linked check-in.
        customer_phone: payload.customer_phone.as_deref(),
        guest_name_for_registry: &payload.guest_name_for_registry,
        guest_country: &payload.guest_country,
        created_by: &payload.created_by,
        room_no: &payload.room_no,
        room_type: &payload.room_type,
        stay_start: payload.stay.start,
        stay_end: payload.stay.end,
        price_per_night_baht,
        nights: payload.nights,
        price_total_baht,
        room_status_id_base,
        nights_calendar,
        checkin_ds_id: checkin_ds_id_base,
        photo_tmp_no: payload.photo_tmp_no.as_deref(),
        created_at,
        room_lines: payload.room_lines.clone(),
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;

    let _ = DEFAULT_OPERATOR; // silence unused-import lint
    // Track B4 — record per-room (room_no, ds_id) mapping.
    let lines = effective_room_lines(&inputs);
    let mut ids = LegacyIds::new()
        .with_cin_no(cin_no.clone())
        .with_cust_no(cust_no.clone())
        .with_book_id(book_id.to_string())
        .with_room_no(payload.room_no.clone())
        .with_checkin_ds_id(checkin_ds_id_base);
    for (room_idx, line) in lines.iter().enumerate() {
        ids = ids.with_room_ds_id(
            line.room_no.clone(),
            checkin_ds_id_base + room_idx as i32,
        );
    }
    ids.extra
        .insert("room_status_id_base".into(), serde_json::Value::from(room_status_id_base));
    Ok(ids)
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
            checkin_ds_id: 25009,
            photo_tmp_no: None,
            // Track B4 — empty slice ⇒ legacy single-room path.
            room_lines: Vec::new(),
            // Wave 5b item 4: fixed instant for deterministic tests.
            created_at: Utc.with_ymd_and_hms(2026, 4, 24, 10, 23, 2).unwrap(),
        }
    }

    /// Inputs that mirror the captured legacy check-in for
    /// `CH26-005236` (Cust C21624, room 414, booking R014820, single
    /// night). Used to assert byte-for-byte parity on HT_CheckIn_H.
    fn capture_inputs() -> CheckInToBookingInputs<'static> {
        CheckInToBookingInputs {
            cin_no: "CH26-005236",
            cust_no: "C21624",
            book_id: "R014820",
            customer_name: "Alberto Calvo Alvarez",
            customer_phone: None,
            guest_name_for_registry: "Alberto Calvo Alvarez",
            guest_country: "",
            created_by: "Admin",
            room_no: "414",
            room_type: "เตียงเดี่ยว",
            // Bangkok 11:32:02 AM = UTC 04:32:02
            stay_start: Utc.with_ymd_and_hms(2026, 4, 25, 4, 32, 02).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 4, 26, 4, 59, 59).unwrap(),
            price_per_night_baht: 801.0,
            nights: 1,
            price_total_baht: 801.0,
            room_status_id_base: 50300,
            nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            checkin_ds_id: 25014,
            photo_tmp_no: None,
            // Track B4 — empty slice ⇒ legacy single-room path.
            room_lines: Vec::new(),
            // Wave 5b item 4: pin `Cin_Date` to the captured wall-clock
            // (4/25/2026 11:32:02 AM Bangkok = 04:32:02 UTC) for exact
            // byte-parity below.
            created_at: Utc.with_ymd_and_hms(2026, 4, 25, 4, 32, 2).unwrap(),
        }
    }

    /// Byte-level parity for the HT_CheckIn_H statement against the
    /// captured booking-checkin for `CH26-005236` in
    /// `/tmp/legacy-events-full.log`.
    ///
    /// Wave 5b item 4: `created_at` is now an input, so this asserts the
    /// FULL line by exact-equality — no more substring-matching around
    /// `Cin_Date`.
    #[test]
    fn checkin_h_matches_legacy_capture_byte_for_byte() {
        let s = build_statements(&capture_inputs());
        let cin_h = s
            .iter()
            .find(|s| s.contains("[HT_CheckIn_H]"))
            .expect("HT_CheckIn_H INSERT must be emitted");
        let expected = "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
                        [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
                        [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],\
                        [Cin_Car_id],[Cin_Room_ALL],[Cin_by],[Cin_Date_in],[Cin_Date_Out],\
                        [Cin_Type],[Cin_foreign])\
                        VALUES('CH26-005236','4/25/2026 11:32:02 AM','R014820','C21624','ราคาปกติ',\
                        'ปกติ',801.00,0.00,801.00,0.00,801.00,'','','414 ','Admin',\
                        '4/25/2026 11:32:02 AM','4/26/2026 11:59:59 AM',0,'False')";
        assert_eq!(cin_h, expected);
    }

    /// Wave 5b item 4: `build_statements` is PURE — no `Utc::now()`, no
    /// I/O. Two invocations with identical inputs must produce identical
    /// output. Pins the purity contract.
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let inputs = sample_inputs();
        let first = build_statements(&inputs);
        let second = build_statements(&inputs);
        assert_eq!(first, second, "build_statements must be deterministic");
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
    fn checkin_h_uses_normal_status_and_normal_price_per_capture() {
        let s = build_statements(&sample_inputs());
        let cin_h = s.iter().find(|s| s.contains("HT_CheckIn_H")).unwrap();
        assert!(cin_h.contains("'ราคาปกติ'"));
        assert!(cin_h.contains("'ปกติ'"));
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
        inputs.photo_tmp_no = Some("687233");
        let s = build_statements(&inputs);
        let upd = s
            .iter()
            .find(|s| s.starts_with("update Tb_Save_Image"))
            .expect("Tb_Save_Image UPDATE must be emitted when tmp_no supplied");
        assert!(upd.contains("cin_no='CH26-005231'"));
        assert!(upd.contains("cust_no='C21610'"));
        assert!(upd.contains("tmp_no=''"));
        assert!(upd.contains("where tmp_no='687233'"));
    }

    #[test]
    fn skips_tb_save_image_when_photo_tmp_no_none() {
        let s = build_statements(&sample_inputs());
        assert!(!s.iter().any(|s| s.starts_with("update Tb_Save_Image")));
    }

    /// Audit H14: empty `tmp_no` must skip the Tb_Save_Image UPDATE — same
    /// reason as `walkin`: `WHERE tmp_no=''` re-stamps every orphan-pending-
    /// cleanup row with THIS check-in's identifiers.
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

    #[test]
    fn tmp_no_whitespace_only_skips_tb_save_image_update() {
        let mut inputs = sample_inputs();
        inputs.photo_tmp_no = Some("   ");
        let s = build_statements(&inputs);
        assert!(!s.iter().any(|s| s.starts_with("update Tb_Save_Image")));
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

    /// Wave 3 followup — after H7 made `booking_create` insert
    /// `HT_Room_Status` rows for every booked night, the nights-1..N branch
    /// of `checkin_to_booking` must NOT fire a plain INSERT for rows that
    /// already exist (multi-night booking conversion). Instead it emits a
    /// single-statement `IF EXISTS … UPDATE … ELSE INSERT` upsert that
    /// matches the legacy app's "upsert per night" semantics
    /// (`COMPAT_CHEATSHEET.md:347-348`).
    #[test]
    fn additional_nights_are_upserts_not_plain_inserts() {
        let s = build_statements(&sample_inputs());
        let nights_1_plus: Vec<&String> = s
            .iter()
            .filter(|stmt| stmt.starts_with("IF EXISTS (SELECT 1 FROM [HT_Room_Status]"))
            .collect();
        assert_eq!(
            nights_1_plus.len(),
            1,
            "expected one upsert per night-1..N (2-night sample has 1)"
        );
        let upsert = nights_1_plus[0];
        assert!(upsert.contains("room_date='4/25/2026'"));
        assert!(upsert.contains("room_no='402'"));
        assert!(upsert.contains("UPDATE [HT_Room_Status]"));
        assert!(upsert.contains("ELSE"));
        assert!(upsert.contains("INSERT INTO [HT_Room_Status]"));
        assert!(upsert.contains("(50237,"));
    }

    /// Wave 3 followup regression — a multi-night conversion must NOT emit
    /// a bare `INSERT INTO [HT_Room_Status]` (i.e. one that isn't guarded by
    /// an `IF EXISTS … ELSE` clause) for nights 1..N. The night-0 path is
    /// still an UPDATE-only (it relies on the booking-created row existing).
    #[test]
    fn multi_night_does_not_double_insert_room_status() {
        let mut inputs = sample_inputs();
        inputs.nights_calendar = vec![
            NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
        ];
        let s = build_statements(&inputs);
        // Every HT_Room_Status statement must be either an UPDATE (night 0)
        // or an `IF EXISTS … UPDATE … ELSE INSERT` upsert (nights 1..N).
        // A bare INSERT line (not preceded by `IF EXISTS …`) would risk
        // duplicate rows since the table has no unique constraint per
        // SCHEMA.sql inspection.
        for stmt in s.iter().filter(|s| s.contains("HT_Room_Status")) {
            let is_night_zero_update = stmt.starts_with("update [HT_Room_Status] SET");
            let is_guarded_upsert =
                stmt.starts_with("IF EXISTS (SELECT 1 FROM [HT_Room_Status]")
                && stmt.contains("ELSE")
                && stmt.contains("INSERT INTO [HT_Room_Status]");
            assert!(
                is_night_zero_update || is_guarded_upsert,
                "unguarded HT_Room_Status statement would double-insert:\n{stmt}"
            );
        }
        // Exactly one night-0 UPDATE + (N-1) upserts for an N-night stay.
        let night_zero_updates = s
            .iter()
            .filter(|s| s.starts_with("update [HT_Room_Status] SET"))
            .count();
        let upserts = s
            .iter()
            .filter(|s| s.starts_with("IF EXISTS (SELECT 1 FROM [HT_Room_Status]"))
            .count();
        assert_eq!(night_zero_updates, 1);
        assert_eq!(upserts, 2, "3-night stay must emit 2 upserts for nights 1+2");
    }

    #[test]
    fn emits_cupon_print_update_at_end() {
        // Spike §3d `booking-checkin/writes.txt:39` — the linked-to-booking
        // check-in fires the same HT_Cupon mark-printed UPDATE as walk-in.
        let s = build_statements(&sample_inputs());
        let cupon = s
            .iter()
            .find(|s| s.starts_with("update HT_Cupon"))
            .expect("HT_Cupon mark-printed UPDATE must be emitted");
        assert!(cupon.contains("cupon_print=1"));
        assert!(cupon.contains("cupon_cin_no='CH26-005231'"));
    }

    /// 2026-06-11 audit P1-9 — the re-save SETs ONLY the fields the
    /// payload carries. The sample payload has no phone and an empty
    /// country, so the UPDATE is name-only — byte-pinned. iHOTEL's own
    /// re-save preserves loaded values; blanking the ~25 uncarried
    /// fields (email, addresses, ID-card, prefix, sex, tier) erased
    /// receptionist-entered data.
    #[test]
    fn ht_customers_update_sets_only_payload_carried_fields() {
        let s = build_statements(&sample_inputs());
        let upd = s
            .iter()
            .find(|s| s.starts_with("UPDATE [HT_Customers]"))
            .unwrap();
        // Byte-pinned narrow shape (capture conventions: two spaces after
        // SET, lowercase `where`).
        assert_eq!(
            upd,
            "UPDATE [HT_Customers] SET  [Cust_name]='SPIKE TEST WALKIN' \
             where Cust_no='C21610'"
        );
        // None of the previously-blanked fields may appear.
        for field in [
            "[Cust_name2]",
            "[Cust_Type]",
            "[Cust_Type_main]",
            "[Cust_Email]",
            "[Cust_Add_no]",
            "[Cust_Add_moo]",
            "[Cust_Add_soi]",
            "[Cust_Add_road]",
            "[Cust_Work_Name]",
            "[Cust_Work_tax]",
            "[Cust_perfix]",
            "[Cust_sex]",
            "[Cust_IDcard]",
        ] {
            assert!(
                !upd.contains(field),
                "field {field} must NOT be blanked by the re-save: {upd}"
            );
        }
    }

    /// P1-9 — a non-empty country in the payload still lands in
    /// `Cust_Contry`; an empty one is omitted (preserve, don't blank).
    #[test]
    fn ht_customers_update_carries_country_only_when_non_empty() {
        let mut inputs = sample_inputs();
        inputs.guest_country = "Spain";
        let s = build_statements(&inputs);
        let upd = s
            .iter()
            .find(|s| s.starts_with("UPDATE [HT_Customers]"))
            .unwrap();
        assert_eq!(
            upd,
            "UPDATE [HT_Customers] SET  [Cust_name]='SPIKE TEST WALKIN',\
             [Cust_Contry]='Spain' where Cust_no='C21610'"
        );
    }

    /// Audit H13: linked-to-booking `execute()` must reject NaN before any
    /// SQL is formatted. Mirrors the walk-in guard — `format!("{}", f64::NAN)`
    /// would emit literal `"NaN"` and fail mid-transaction.
    #[test]
    fn validate_finite_blocks_nan_in_checkin_to_booking_execute_inputs() {
        let result = super::super::helpers::validate_finite(&[
            ("price_per_night_baht", f64::NAN),
            ("price_total_baht", 1780.0),
        ]);
        let err = result.expect_err("NaN must be rejected");
        assert!(err.to_string().contains("price_per_night_baht"));
    }

    #[test]
    fn validate_finite_blocks_infinity_in_checkin_to_booking_execute_inputs() {
        let result = super::super::helpers::validate_finite(&[
            ("price_per_night_baht", 890.0),
            ("price_total_baht", f64::NEG_INFINITY),
        ]);
        let err = result.expect_err("Infinity must be rejected");
        assert!(err.to_string().contains("price_total_baht"));
    }

    #[test]
    fn power_log_note_uses_check_in_template() {
        let s = build_statements(&sample_inputs());
        let pl = s.iter().find(|s| s.contains("HT_POWER_LOG")).unwrap();
        assert!(pl.contains("'เปิดไฟ อัตโนมัติ จากเช็คอิน No.CH26-005231'"));
    }

    /// Wave 5a item 1: when the payload carries a customer phone, the
    /// UPDATE HT_Customers statement must write it into `Cust_Add_tel`
    /// — not the empty string. The prior code unconditionally passed
    /// `customer_phone: None` from `execute()`, which mapped to
    /// `[Cust_Add_tel]=''` and wiped the booking-time phone every
    /// time a guest converted from booking to check-in.
    #[test]
    fn customer_phone_preserved_when_supplied() {
        let mut inputs = sample_inputs();
        inputs.customer_phone = Some("0812345678");
        let s = build_statements(&inputs);
        let upd = s
            .iter()
            .find(|s| s.starts_with("UPDATE [HT_Customers]"))
            .expect("UPDATE [HT_Customers] must be emitted");
        assert!(
            upd.contains("[Cust_Add_tel]='0812345678'"),
            "supplied phone must land verbatim in Cust_Add_tel; got:\n{upd}"
        );
    }

    /// 2026-06-11 audit P1-9: an absent phone (`None`) must OMIT
    /// `Cust_Add_tel` from the SET list entirely — preserving whatever
    /// phone iHOTEL has on file. (The pre-audit behavior wrote
    /// `[Cust_Add_tel]=''`, wiping the booking-time phone whenever the
    /// payload carried none.)
    #[test]
    fn customer_phone_omitted_when_none() {
        let s = build_statements(&sample_inputs());
        let upd = s
            .iter()
            .find(|s| s.starts_with("UPDATE [HT_Customers]"))
            .unwrap();
        assert!(
            !upd.contains("[Cust_Add_tel]"),
            "absent phone must not blank Cust_Add_tel: {upd}"
        );
    }
}
