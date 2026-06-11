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

use chrono::{DateTime, NaiveDate, Utc};

use crate::outbox::intent::{CreateCheckInPayload, RoomLine};
use crate::writeback::allocate::{
    allocate_checkin_ds_id, allocate_cin_no, allocate_customer_id, cust_no_from_customer_id,
    allocate_room_status_id, LegacyConn,
};
use crate::writeback::constants::{
    power_log_note_check_in, CIN_DEP_STATUS_NONE, CIN_ROOM_STATUS_OCCUPYING, CIN_STATUS_NORMAL,
    CUST_TYPE_MAIN_NORMAL, CUST_TYPE_NORMAL, DEFAULT_OPERATOR, ROOM_STATUS_OCCUPYING,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{
    date_to_ole_serial, enumerate_calendar_nights, format_legacy_date, format_legacy_datetime,
    sql_quote,
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
    ///
    /// Multi-room (Track B4): when `room_lines` is non-empty this is the
    /// id for the FIRST room's `HT_CheckIn_Ds` row. Subsequent rows use
    /// sequential ids (`checkin_ds_id + 1`, +2, …) — `execute()` allocates
    /// them under a single TABLOCKX MAX+1 sweep so the values stay
    /// contiguous and race-safe.
    pub checkin_ds_id: i32,
    pub nights_calendar: Vec<NaiveDate>,
    /// Optional `Tb_Save_Image.tmp_no` — see [`CreateCheckInPayload::photo_tmp_no`].
    pub photo_tmp_no: Option<&'a str>,
    /// "Now" timestamp threaded in by `execute()` so `build_statements`
    /// stays PURE — Wave 5b item 4. Drives `HT_CheckIn_H.Cin_Date` and
    /// `HT_Customers.Cust_Last_Change` (the latter as a Bangkok-local date).
    /// Tests pass a fixed instant for exact byte-parity assertions.
    pub created_at: DateTime<Utc>,
    /// Track B4 — per-room slice from `ht_checkin_rooms`. Empty ⇒
    /// recipe emits the legacy single-room shape using the top-level
    /// `room_no` / `room_type` / `price_per_night_baht` / `nights` /
    /// `price_total_baht` fields. Non-empty ⇒ recipe emits one
    /// `HT_CheckIn_Ds` + `HT_POWER_LOG` row per slice entry, allocates
    /// `room_lines.len()` sequential ds_ids starting at `checkin_ds_id`,
    /// and concatenates room numbers into a single `HT_CheckIn_H.Cin_Room_ALL`.
    pub room_lines: Vec<RoomLine>,
}

/// Build the statements for a walk-in. PURE — no I/O.
pub fn build_statements(inputs: &WalkInInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let by_q = sql_quote(inputs.created_by);
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_phone_q = sql_quote(inputs.customer_phone.unwrap_or(""));
    let cust_type_q = sql_quote(CUST_TYPE_NORMAL);
    let cust_type_main_q = sql_quote(CUST_TYPE_MAIN_NORMAL);
    let cust_last_change_q = sql_quote(&format_legacy_date(
        crate::writeback::format::bangkok_date(inputs.created_at),
    ));
    let stay_start_q = sql_quote(&format_legacy_datetime(inputs.stay_start));
    let stay_end_q = sql_quote(&format_legacy_datetime(inputs.stay_end));
    let now_q = sql_quote(&format_legacy_datetime(inputs.created_at));
    let occupying_q = sql_quote(CIN_ROOM_STATUS_OCCUPYING);
    let cin_status_q = sql_quote(CIN_STATUS_NORMAL);
    let cust_price_q = sql_quote(CUST_TYPE_NORMAL);
    let dep_status_q = sql_quote(CIN_DEP_STATUS_NONE);
    let room_status_q = sql_quote(ROOM_STATUS_OCCUPYING);
    let power_note = power_log_note_check_in(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    let registry_prefix = super::helpers::guest_prefix_for_country(inputs.guest_country);
    // Legacy capture pattern (verified 10 captured Other_People rows in
    // /tmp/legacy-events-full.log): prefix + ' ' + name + ' '. The
    // trailing space comes from `name + ' ' + name2` when name2 is
    // empty — we emulate the empty-name2 case directly.
    let registry_name = format!("{registry_prefix} {} ", inputs.guest_name_for_registry);
    let registry_name_q = sql_quote(&registry_name);
    let country_q = sql_quote(inputs.guest_country);
    let cust_id = inputs.customer_id_int;
    // Wave 6 LOW item 4: pre-format money to 2dp for consistency with the
    // HT_CheckIn_H VALUES and the rest of the recipe corpus.
    let total = format!("{:.2}", inputs.price_total_baht);

    // Track B4 — derive the per-room slice. Empty `room_lines` falls back
    // to the legacy single-room shape using the top-level fields so the
    // recipe stays byte-identical for callers that haven't migrated to
    // the junction yet (legacy spike captures, pre-B4 outbox events).
    let lines = effective_room_lines(inputs);
    let primary_room_no = lines[0].room_no.as_str();
    let primary_room_no_q = sql_quote(primary_room_no);
    // `Cin_Room_ALL`: legacy capture pattern is space-separated room
    // numbers with a trailing space (single-room `'402 '`; multi-room
    // `'508 509 '`). The same byte-for-byte shape covers both.
    let room_all = lines
        .iter()
        .map(|l| format!("{} ", l.room_no))
        .collect::<String>();
    let room_all_q = sql_quote(&room_all);

    let mut statements: Vec<String> = Vec::with_capacity(
        8 + inputs.nights_calendar.len() * lines.len() + lines.len(),
    );

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

    // Track B4 — per-room fan-out. For a single-room folio `lines.len()
    // == 1` and the loops collapse to one HT_Rooms / HT_CheckIn_Ds /
    // HT_POWER_LOG / N×HT_Room_Status statements — byte-identical to the
    // pre-B4 single-room recipe. For multi-room (`lines.len() == N`) the
    // recipe emits N of each room-scoped statement.
    //
    // Sequential ds_id allocation: the caller (`execute()`) acquires a
    // single `MAX(id)+1` under TABLOCKX, then we assign +0, +1, … +(N-1)
    // to each room. Holding the lock once per folio keeps the race-safety
    // guarantee from the spike §6 proof and avoids N round-trips for the
    // common 1- or 2-room walk-in.

    // 2. Mark each room occupied — by room_no per spike §3a
    for line in &lines {
        let line_room_no_q = sql_quote(&line.room_no);
        statements.push(format!(
            "update HT_Rooms set room_use='yes' where room_no={line_room_no_q}"
        ));
    }

    // 3. HT_CheckIn_Ds — one row per room. 16 cols in legacy order
    //    (verified from /tmp/legacy-events-full.log). `[Cin_dep_status]`
    //    is lowercase d-s; `[Dep_by]` is NOT in the column list. `id` is
    //    NOT IDENTITY (schema dump 2026-04-26 confirmed). Per-room
    //    `Cin_Room_Status` is passed through verbatim from the junction
    //    so Thai literals (`'เข้าพัก'`, `'จอง'`) preserve byte-for-byte.
    for (room_idx, line) in lines.iter().enumerate() {
        let ds_id = inputs.checkin_ds_id + room_idx as i32;
        let line_room_no_q = sql_quote(&line.room_no);
        let line_room_type_q = sql_quote(&line.room_type);
        let line_price = format!("{:.2}", line.price_per_night);
        let line_total = format!("{:.2}", line.room_total);
        let line_nights = line.nights;
        // Per-room Thai status literal — preserve verbatim. Falls back
        // to the recipe-level `'เข้าพัก'` when the slice omits the field
        // (single-room legacy path).
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

    // 4. HT_POWER_LOG — lights on per room with check-in note
    for line in &lines {
        let line_room_no_q = sql_quote(&line.room_no);
        statements.push(format!(
            "INSERT INTO [HT_POWER_LOG]([ROOM_NO],[ROOM_POWER_START],[ROOM_POWER_START_BY],\
             [ROOM_POWER_END_BY],[ROOM_POWER_NOTE],[ROOM_POWER_NOTE2])\
             VALUES({line_room_no_q},GETDATE(),{by_q},'',{power_note_q},'')"
        ));
    }

    // 5..N. HT_Room_Status — one row per (room, calendar night). Ids are
    //       sequential across the full Cartesian product: room0×night0,
    //       room0×night1, …, room0×nightN-1, room1×night0, … (caller
    //       allocates `nights × rooms` ids in one TABLOCKX sweep).
    for (room_idx, line) in lines.iter().enumerate() {
        let line_room_no_q = sql_quote(&line.room_no);
        for (night_idx, day) in inputs.nights_calendar.iter().enumerate() {
            let id = inputs.room_status_id_base
                + (room_idx as i32 * inputs.nights_calendar.len() as i32)
                + night_idx as i32;
            let date_q = sql_quote(&format_legacy_date(*day));
            let oa = date_to_ole_serial(*day) as i64;
            statements.push(format!(
                "INSERT INTO [HT_Room_Status]([id],[room_no],[room_date],[room_status],\
                 [room_Details],[room_CheckIn_No],[room_date_oa])\
                 VALUES({id},{line_room_no_q},{date_q},{room_status_q},{cust_name_q},{cin_no_q},{oa})"
            ));
        }
    }
    // Silence: `primary_room_no_q` is read below in the header build for
    // back-compat clarity; suppress the unused-var lint when the header
    // path doesn't reference it directly.
    let _ = &primary_room_no_q;
    let _ = primary_room_no;

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
    // Wave 6 LOW item 4: `total` is the pre-formatted 2dp string.
    let total_2dp = total.as_str();
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_H]([Cin_no],[Cin_Date],[Cin_Book_no],[Cin_cust_no],\
         [Cin_cust_price],[Cin_status],[Total_Price_Room],[Total_Price_Product],\
         [Total_Price_Net],[Total_Price_Pay],[Total_Price_Balance],[Cin_Car_type],[Cin_Car_id],\
         [Cin_Room_ALL],[Cin_by],[Cin_Date_in],[Cin_Date_Out],[Cin_Type],[Cin_foreign])\
         VALUES({cin_no_q},{now_q},'',{cust_no_q},{cust_price_q},{cin_status_q},\
         {total_2dp},0.00,{total_2dp},0.00,{total_2dp},'','',{room_all_q},{by_q},\
         {stay_start_q},{stay_end_q},0,'False')",
    ));

    // N+3. HT_Cupon — mark loyalty coupon as printed (spike §3a, walkin/writes.txt:9).
    // Extracted into the shared helper so this recipe and `checkin_to_booking`
    // emit byte-identical SQL.
    statements.push(super::helpers::mark_cupon_printed(inputs.cin_no));

    statements
}

/// Track B4 — derive the effective per-room slice. When `inputs.room_lines`
/// is non-empty we trust the caller's canonical slice from
/// `ht_checkin_rooms` verbatim. When empty (single-room legacy path)
/// we synthesize a one-line slice from the top-level fields so every
/// downstream emitter loops the same shape.
fn effective_room_lines(inputs: &WalkInInputs<'_>) -> Vec<RoomLine> {
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

    // Wave 6 LOW item 5: hard-validate `nights >= 1` instead of silently
    // clamping via `.max(1)`. The service layer should reject this at
    // enqueue time; this is defense-in-depth so a caller bug surfaces as a
    // `Recipe` error before any TABLOCKX allocation runs.
    if payload.nights < 1 {
        return Err(WritebackError::Recipe(format!(
            "Walk-in CheckIn: nights must be >= 1 (got {})",
            payload.nights
        )));
    }

    // Allocate IDs under TABLOCKX, in dependency order.
    //
    // 2026-06-11 audit: Cust_no derives from the SAME `MAX(id)+1` value as
    // the `id` column, matching iHOTEL's convention (cheatsheet §1.6 #3 —
    // `Cust_no = 'C' + (id)`). The earlier separate suffix-parsing
    // allocator was numerically aligned today but would fork from iHOTEL
    // under any id/Cust_no divergence.
    let cust_id_int = allocate_customer_id(conn).await?;
    let cust_no = match payload.legacy_cust_no.as_deref() {
        Some(existing) => existing.to_string(),
        None => cust_no_from_customer_id(cust_id_int),
    };
    let cin_no = allocate_cin_no(conn).await?;
    let room_status_id_base = allocate_room_status_id(conn).await?;
    // Track B4 — allocate the FIRST `HT_CheckIn_Ds.id` under TABLOCKX
    // and use it as the base for sequential per-room ids. The single
    // TABLOCKX sweep keeps the spike §6 race-safety guarantee even when
    // the recipe emits N rows in one folio (the tail ids are guaranteed
    // free because TABLOCKX is held until commit).
    //
    // Schema dump 2026-04-26 confirmed `HT_CheckIn_Ds.id` is NOT IDENTITY
    // despite the spike's earlier note — INSERT must provide an explicit
    // value.
    let checkin_ds_id_base = allocate_checkin_ds_id(conn).await?;

    // Wave 6 LOW item 6: empty range surfaces as error rather than silently
    // injecting a phantom night; cap-truncate logs WARN.
    let nights_calendar = enumerate_calendar_nights(payload.stay.start, payload.stay.end)?;

    // Wave 5b item 4: capture `Utc::now()` once at the entry to `execute()` so
    // `build_statements` is purely a function of its inputs. Tests can swap in
    // a fixed instant for exact byte-parity assertions.
    let created_at = Utc::now();

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

    // Track B4 — record the per-room (room_no, ds_id) mapping for
    // `ht_checkin_rooms.cr_legacy_ds_id` back-population. Single-room
    // folios still surface the legacy `checkin_ds_id` field (back-compat
    // with ExtendStay / CheckOut single-row consumers); multi-room
    // folios populate both `checkin_ds_id` (= first room's id) AND the
    // full mapping.
    let lines = effective_room_lines(&inputs);
    let mut ids = LegacyIds::new()
        .with_cin_no(cin_no.clone())
        .with_cust_no(cust_no.clone())
        .with_room_no(payload.room_no.clone())
        .with_checkin_ds_id(checkin_ds_id_base);
    for (room_idx, line) in lines.iter().enumerate() {
        ids = ids.with_room_ds_id(
            line.room_no.clone(),
            checkin_ds_id_base + room_idx as i32,
        );
    }
    ids.extra
        .insert("customer_id_int".into(), serde_json::Value::from(cust_id_int));
    ids.extra
        .insert("room_status_id_base".into(), serde_json::Value::from(room_status_id_base));
    Ok(ids)
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
            // Wave 5b item 4: fixed instant for byte-parity tests.
            // 9:56:20 UTC = 4:56:20 PM Bangkok.
            created_at: Utc.with_ymd_and_hms(2026, 4, 24, 9, 56, 20).unwrap(),
            // Track B4 — empty slice ⇒ legacy single-room path. Multi-
            // room tests construct their own inputs with `room_lines` set.
            room_lines: Vec::new(),
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
            // Wave 5b item 4: pin `Cin_Date` to the captured wall-clock
            // (4/25/2026 4:12:18 PM Bangkok = 09:12:18 UTC) so the legacy
            // capture-byte-parity assertion below can use exact equality.
            created_at: Utc.with_ymd_and_hms(2026, 4, 25, 9, 12, 18).unwrap(),
            room_lines: Vec::new(),
        }
    }

    /// Byte-level parity for the HT_CheckIn_H statement against the
    /// captured walk-in for `CH26-005242` in `/tmp/legacy-events-full.log`.
    ///
    /// Wave 5b item 4: `created_at` is now an input, so we can express this
    /// as an exact-equality check against the full legacy line — no more
    /// substring-matching around `Cin_Date`.
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
                        VALUES('CH26-005242','4/25/2026 4:12:18 PM','','C21636','ราคาปกติ','ปกติ',\
                        890.00,0.00,890.00,0.00,890.00,'','','407 ','Admin','4/25/2026 4:12:18 PM',\
                        '4/26/2026 12:00:00 PM',0,'False')";
        assert_eq!(cin_h, expected);
    }

    /// Wave 5b item 4: `build_statements` is PURE — no `Utc::now()`, no
    /// other I/O. Calling it twice with the same inputs (including a fixed
    /// `created_at`) must produce byte-identical output. This pins the
    /// purity contract that the doc-comment claims and makes regressions
    /// (e.g. someone re-adding `Utc::now()` inside) visible immediately.
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let inputs = sample_inputs();
        let first = build_statements(&inputs);
        let second = build_statements(&inputs);
        assert_eq!(first, second, "build_statements must be deterministic");
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

    /// Wave 6: enumerate_calendar_nights now lives in `writeback::format`
    /// and returns Result. This test keeps the walkin-specific coverage
    /// (overnight stay) — the empty-range / cap-truncate guard rails are
    /// tested in `format.rs`.
    #[test]
    fn enumerate_calendar_nights_handles_overnight() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 25, 11, 59, 59).unwrap(),
        )
        .unwrap();
        assert_eq!(nights, vec![NaiveDate::from_ymd_opt(2026, 4, 24).unwrap()]);
    }
}
