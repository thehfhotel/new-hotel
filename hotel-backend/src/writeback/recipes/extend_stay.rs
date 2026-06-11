//! `ExtendStay` recipe — spike `findings.md` §3f Phase A.
//!
//! Recompute totals, replace `HT_Room_Status` rows for the changed range.
//! Targeted UPDATEs — we **skip the legacy app's destructive Phase B**
//! (DELETE+REINSERT) per spike §3f recommendation.
//!
//! Reference SQL (verbatim from `extend-20260424-101350/writes.txt`,
//! findings.md §3f Phase A — 7 statements):
//!
//! ```text
//! 0. UPDATE HT_CheckIn_H SET Cin_Work_number=539215 WHERE Cin_No='CH26-005230'
//!    -- ONE leading TM.30 touch (findings.md:276)
//!
//! 1. update HT_Rooms set room_use='no'
//!    where room_no in (select Cin_Room_No from HT_CheckIn_Ds
//!                       where Cin_no='CH26-005230' and Cin_Room_Status<>'Check-Out')
//!
//! 2. delete from HT_Room_Status where room_CheckIn_No='CH26-005230'
//!
//! 3. UPDATE [HT_CheckIn_H] SET
//!      [Total_Price_Room]=1780, [Total_Price_Net]=1780, [Total_Price_Balance]=1780
//!    where [Cin_no]='CH26-005230'
//!    -- ONLY Room / Net / Balance (findings.md:279-281). The capture does
//!    -- NOT touch Total_Price_Product or Total_Price_Pay.
//!
//! 4. update [HT_CheckIn_Ds] SET
//!      [Cin_Room_night]=2, [Cin_Room_PriceTotal]=1780, [Cin_note]='',
//!      [Cin_Room_Out]='4/26/2026 12:00:00 PM'
//!    where id=25009
//!
//! 5. update HT_Rooms set room_use='yes' where room_no='508'   -- revert
//!
//! 6+7. INSERT INTO [HT_Room_Status] (id, room_no, room_date, room_status,
//!        room_Details, room_CheckIn_No, room_date_oa)
//!      VALUES (50235, '508', '4/24/2026', 'เข้าพัก', 'SPIKE TEST WALKIN 3',
//!              'CH26-005230', 46136)
//!      VALUES (50236, '508', '4/25/2026', 'เข้าพัก', 'SPIKE TEST WALKIN 3',
//!              'CH26-005230', 46137)
//! ```
//!
//! Spike §3f notes:
//! - Old `HT_Room_Status` rows are deleted entirely, then re-INSERTed for the
//!   full new date range. (Cleaner than diffing — and matches the legacy app.)
//! - `room_use='no'` then `'yes'` flicker is intentional in the capture but
//!   functionally a no-op. We preserve it for parity.
//! - `Cin_Room_Out` is the canonical departure time the user picked. Format
//!   matches the legacy app's `12:00:00 PM` convention.
//!
//! ## Deliberate departures (2026-06-11 coexistence audit, P0-3)
//!
//! - **`Total_Price_Product` and `Total_Price_Pay` are NOT written.** An
//!   earlier revision set both — with `product_total` hardcoded `0.0` —
//!   which zeroed iHOTEL-entered product revenue on every extend and raced
//!   concurrent payment writebacks on `Total_Price_Pay`. The §3f capture
//!   never touches those columns; neither do we.
//! - **`Total_Price_Balance` is re-aggregated live**, not taken from the
//!   intent payload: `Balance = Net - SUM(active HT_CheckIn_Pay tender)`
//!   under UPDLOCK+HOLDLOCK held through COMMIT — the same discipline
//!   `checkout.rs` / `payment.rs` use (Track C T5). The capture's literal
//!   Balance is only correct because Pay was 0 at capture time; a literal
//!   would clobber concurrent payments.

use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::domain::shared::Money;
use crate::writeback::allocate::{allocate_room_status_id, LegacyConn};
use crate::writeback::constants::{CIN_ROOM_STATUS_OCCUPYING, ROOM_STATUS_OCCUPYING};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    date_to_ole_serial, enumerate_calendar_nights, format_legacy_date, format_legacy_datetime,
    sql_quote,
};

/// Inputs for the extend-stay recipe.
#[derive(Debug, Clone)]
pub struct ExtendStayInputs<'a> {
    pub cin_no: &'a str,
    pub room_no: &'a str,
    pub checkin_ds_id: i32,
    pub new_end: DateTime<Utc>,
    /// New nights total after extending. Per spike §3f the capture set this
    /// from 1 → 2 nights.
    pub new_nights: i32,
    /// New total room price (`Cin_Room_Price * new_nights`).
    pub new_room_price_total: f64,
    pub new_net_total: f64,
    /// Customer name to display in `HT_Room_Status.room_Details` (mirrors the
    /// legacy app's behavior of showing the guest name on the calendar grid).
    pub guest_label: &'a str,
    /// Calendar nights the stay now covers. Each becomes one
    /// `HT_Room_Status` row. Pass in calendar order; the recipe allocates
    /// `id`s sequentially (caller supplies the starting id via
    /// `room_status_id_base` + offset).
    pub nights: Vec<NaiveDate>,
    /// First `HT_Room_Status.id` to use. Recipe assigns
    /// `room_status_id_base + i` for the i-th night.
    pub room_status_id_base: i32,
    /// Random TM.30 batch number — spike §3a + §3f capture line 1
    /// (`findings.md:276`). The .NET app emits ONE
    /// `UPDATE HT_CheckIn_H SET Cin_Work_number=<rand>` at the start of the
    /// extend flow. It is a non-sequential i32; the caller generates it via
    /// `rand::random::<i32>()` so `build_statements` stays pure. `None`
    /// emits no touch (useful for tests that focus on the downstream
    /// statements).
    pub tm30_touch_id: Option<i32>,
}

/// Build all statements for an extend-stay. PURE — no I/O.
///
/// `tm30_touch_id` injects the single leading TM.30 touch (spike §3a + §3f
/// `extend/writes.txt:1`, findings.md:276 — the capture shows exactly ONE).
/// It is a random `i32` per spike §3a — the caller (`execute()`) generates
/// it via `rand::random()` so this function stays pure and trivially
/// unit-testable.
pub fn build_statements(inputs: &ExtendStayInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let room_no_q = sql_quote(inputs.room_no);
    let new_end_q = sql_quote(&format_legacy_datetime(inputs.new_end));
    let occupying_q = sql_quote(ROOM_STATUS_OCCUPYING); // == CIN_ROOM_STATUS_OCCUPYING
    let _ = CIN_ROOM_STATUS_OCCUPYING; // silence the unused-constant warning if any
    let label_q = sql_quote(inputs.guest_label);

    let mut statements: Vec<String> = Vec::new();

    // 0. TM.30 touch — spike §3a (`Cin_Work_number` is random + async) and
    //    §3f extend capture line 1 (findings.md:276): the .NET app fires
    //    exactly ONE such UPDATE at the start of the extend flow. Random
    //    i32, supplied by caller. (An earlier revision emitted two —
    //    corrected per the 2026-06-11 coexistence audit.)
    if let Some(tm30_id) = inputs.tm30_touch_id {
        statements.push(format!(
            "update HT_CheckIn_H set Cin_Work_number={tm30_id} where Cin_No={cin_no_q}"
        ));
    }

    // 1. Temp clear room_use (will be reverted)
    statements.push(format!(
        "update HT_Rooms set room_use='no' where room_no in (select Cin_Room_No from HT_CheckIn_Ds where Cin_no={cin_no_q} and Cin_Room_Status<>'Check-Out')"
    ));
    // 2. Wipe all room_status rows for this check-in
    statements.push(format!(
        "delete from HT_Room_Status where room_CheckIn_No={cin_no_q}"
    ));
    // 3. Update HT_CheckIn_H totals — ONLY Room / Net / Balance, matching
    //    the §3f capture (findings.md:279-281). Total_Price_Product and
    //    Total_Price_Pay are deliberately untouched (see module docs —
    //    2026-06-11 audit P0-3).
    //
    //    Balance is `Net - SUM(active tender rows)` re-aggregated live from
    //    `HT_CheckIn_Pay` under UPDLOCK+HOLDLOCK held through COMMIT, the
    //    same race-safe pattern as `checkout.rs` / `payment.rs` (Track C
    //    T5): a Balance literal computed from PG state at intent-emit time
    //    would clobber any payment that committed in between. The
    //    `Cin_Status <> 'ยกเลิก'` filter excludes cancelled tender rows
    //    (T2 CRIT-2).
    //
    //    Wave 6 LOW item 4: 2dp for consistency with the HT_CheckIn_H
    //    create path (`walkin::build_statements`).
    statements.push(format!(
        "UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK) SET \
         [Total_Price_Room]={room_price:.2},\
         [Total_Price_Net]={net:.2},\
         [Total_Price_Balance]={net:.2}-(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)\
         +ISNULL(Cin_Pay_Credit,0)+ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)\
         +ISNULL(Cin_Pay_web,0)),0) FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
         WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก') \
         where [Cin_no]={cin_no_q}",
        room_price = inputs.new_room_price_total,
        net = inputs.new_net_total,
    ));
    // 4. Update HT_CheckIn_Ds (by id) with new nights + price + departure
    statements.push(format!(
        "update [HT_CheckIn_Ds] SET  [Cin_Room_night]={nights},\
         [Cin_Room_PriceTotal]={price_total:.2},[Cin_note]='',\
         [Cin_Room_Out]={new_end_q} where id={ds_id}",
        nights = inputs.new_nights,
        price_total = inputs.new_room_price_total,
        ds_id = inputs.checkin_ds_id,
    ));
    // 5. Revert room_use back to 'yes' — Wave 5b item 2: mirror step-1's
    //    subquery so multi-room check-ins get every non-checked-out room
    //    re-flipped, not just `inputs.room_no`. Prior single-room shape left
    //    sibling rooms stuck `room_use='no'` after the step-1 wipe.
    statements.push(format!(
        "update HT_Rooms set room_use='yes' where room_no in (select Cin_Room_No from HT_CheckIn_Ds where Cin_no={cin_no_q} and Cin_Room_Status<>'Check-Out')"
    ));

    // 6..N. Re-insert HT_Room_Status, one per night
    for (i, day) in inputs.nights.iter().enumerate() {
        let id = inputs.room_status_id_base + i as i32;
        let date_q = sql_quote(&format_legacy_date(*day));
        let oa = date_to_ole_serial(*day) as i64;
        statements.push(format!(
            "INSERT INTO [HT_Room_Status]([id],[room_no],[room_date],[room_status],\
             [room_Details],[room_CheckIn_No],[room_date_oa])VALUES({id},{room_no_q},{date_q},\
             {occupying_q},{label_q},{cin_no_q},{oa})"
        ));
    }
    statements
}

/// Execute the extend-stay recipe.
///
/// Consumes the [`WritebackIntent::ExtendStay`] payload (spike §3f) —
/// `stay_start`, `guest_label`, and the totals the service layer enriches
/// before enqueuing. Calendar nights span the full `[stay_start, new_end)`
/// range so `HT_Room_Status` rows cover the entire stay (not just
/// `[today, new_end)` as the prior implementation did).
///
/// `new_pay_total` / `new_balance_total` are still accepted (older queued
/// intents carry them, and we validate finiteness) but are NOT written to
/// MSSQL — the §3f capture never touches `Total_Price_Pay`, and Balance is
/// re-aggregated live inside the UPDATE (2026-06-11 audit P0-3).
///
/// The single leading TM.30 touch (capture line 1, findings.md:276) is
/// generated via `rand::random::<i32>()` per spike §3a — `Cin_Work_number`
/// is a non-sequential random i32 the .NET app assigns after each save.
#[allow(clippy::too_many_arguments)]
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    room_no: &str,
    checkin_ds_id: i32,
    stay_start: DateTime<Utc>,
    new_end: DateTime<Utc>,
    guest_label: &str,
    new_room_price_total: Money,
    new_net_total: Money,
    new_pay_total: Money,
    new_balance_total: Money,
) -> WritebackResult<LegacyIds> {
    // Allocate the starting room_status id under TABLOCKX so concurrent
    // writers can't collide.
    let id_base = allocate_room_status_id(conn).await?;

    // Wave 6 LOW item 6: empty range surfaces as error rather than silently
    // injecting a phantom night; cap-truncate logs WARN.
    let nights = enumerate_calendar_nights(stay_start, new_end)?;
    let _ = (Datelike::year(&new_end.date_naive()),); // silence unused-import lint

    // HIGH-4: reject NaN/Infinity before SQL formatting.
    super::helpers::validate_finite(&[
        ("new_room_price_total", money_to_baht_f64(new_room_price_total)),
        ("new_net_total", money_to_baht_f64(new_net_total)),
        ("new_pay_total", money_to_baht_f64(new_pay_total)),
        ("new_balance_total", money_to_baht_f64(new_balance_total)),
    ])?;

    let inputs = ExtendStayInputs {
        cin_no,
        room_no,
        checkin_ds_id,
        new_end,
        new_nights: nights.len() as i32,
        new_room_price_total: money_to_baht_f64(new_room_price_total),
        new_net_total: money_to_baht_f64(new_net_total),
        guest_label,
        nights,
        room_status_id_base: id_base,
        // Spike §3a: TM.30 batch numbers are non-sequential random i32
        // assigned by the .NET app's async post-save job. We mirror ONE
        // touch per the extend capture (line 1, findings.md:276). MED-3:
        // clamp to the positive i32 range — the .NET app's Cin_Work_number
        // column is signed but no spike capture has ever observed a
        // negative value, and a negative number may trip the WinForms grid
        // control.
        tm30_touch_id: Some(positive_i32()),
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_cin_no(cin_no.to_string()))
}

fn money_to_baht_f64(m: Money) -> f64 {
    (m.as_satang() as f64) / 100.0
}

/// Random positive i32 (1..=i32::MAX) for TM.30 `Cin_Work_number`. See MED-3
/// in the call site.
fn positive_i32() -> i32 {
    (rand::random::<u32>() & 0x7FFF_FFFF).max(1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Verifies the structure against the spike capture's statements.
    #[test]
    fn build_statements_matches_spike_structure() {
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            // 5 AM UTC = noon Bangkok (the wall-clock the legacy app sees).
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            new_nights: 2,
            new_room_price_total: 1780.0,
            new_net_total: 1780.0,
            guest_label: "SPIKE TEST WALKIN 3",
            nights: vec![
                NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            ],
            room_status_id_base: 50235,
            tm30_touch_id: None,
        };
        let statements = build_statements(&inputs);
        // 5 fixed + 2 night rows
        assert_eq!(statements.len(), 7);

        // 1: temp room_use clear
        assert!(statements[0].contains("Cin_no='CH26-005230'"));
        assert!(statements[0].contains("Cin_Room_Status<>'Check-Out'"));
        // 2: wipe room_status
        assert_eq!(
            statements[1],
            "delete from HT_Room_Status where room_CheckIn_No='CH26-005230'"
        );
        // 3: totals on HT_CheckIn_H — Room + Net literal, Balance live
        //    re-aggregate (byte-pinned in
        //    `totals_update_matches_capture_columns_with_live_balance`).
        assert!(statements[2].contains("[Total_Price_Room]=1780"));
        assert!(statements[2].contains("[Total_Price_Net]=1780"));
        assert!(statements[2].contains("[Total_Price_Balance]=1780.00-(SELECT"));
        // 4: HT_CheckIn_Ds (by id)
        assert!(statements[3].contains("[Cin_Room_night]=2"));
        assert!(statements[3].contains("[Cin_Room_PriceTotal]=1780"));
        assert!(statements[3].contains("[Cin_Room_Out]='4/26/2026 12:00:00 PM'"));
        assert!(statements[3].contains("where id=25009"));
        // 5: revert room_use — Wave 5b item 2: subquery over all
        // non-checked-out rooms of this check-in (multi-room safe).
        assert_eq!(
            statements[4],
            "update HT_Rooms set room_use='yes' where room_no in (select Cin_Room_No from HT_CheckIn_Ds where Cin_no='CH26-005230' and Cin_Room_Status<>'Check-Out')"
        );
        // 6: night 1 INSERT
        assert!(statements[5].contains("VALUES(50235,'508','4/24/2026','เข้าพัก'"));
        assert!(statements[5].contains(",46136)"));
        // 7: night 2 INSERT
        assert!(statements[6].contains("VALUES(50236,'508','4/25/2026','เข้าพัก'"));
        assert!(statements[6].contains(",46137)"));
    }

    #[test]
    fn single_tm30_touch_leads_when_provided() {
        // Spike §3f capture line 1 (findings.md:276): exactly ONE leading
        // TM.30 UPDATE before any other statement. (An earlier revision
        // emitted two — corrected per the 2026-06-11 coexistence audit.)
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            new_nights: 2,
            new_room_price_total: 1780.0,
            new_net_total: 1780.0,
            guest_label: "SPIKE TEST WALKIN 3",
            nights: vec![],
            room_status_id_base: 50235,
            tm30_touch_id: Some(539215),
        };
        let statements = build_statements(&inputs);
        // First statement is the TM.30 touch — byte-pinned to the capture
        // shape (modulo the random batch number).
        assert_eq!(
            statements[0],
            "update HT_CheckIn_H set Cin_Work_number=539215 where Cin_No='CH26-005230'"
        );
        // And exactly one touch in the whole recipe.
        let touches = statements
            .iter()
            .filter(|s| s.contains("Cin_Work_number="))
            .count();
        assert_eq!(touches, 1, "extend must emit exactly one TM.30 touch");
    }

    /// 2026-06-11 audit P0-3 — byte-pin the HT_CheckIn_H totals UPDATE:
    /// only Room / Net / Balance (the §3f capture's column set,
    /// findings.md:279-281); Balance is the live re-aggregate; Product and
    /// Pay are never written.
    #[test]
    fn totals_update_matches_capture_columns_with_live_balance() {
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            new_nights: 2,
            new_room_price_total: 1780.0,
            new_net_total: 1780.0,
            guest_label: "SPIKE TEST WALKIN 3",
            nights: vec![],
            room_status_id_base: 50235,
            tm30_touch_id: None,
        };
        let statements = build_statements(&inputs);
        let totals = statements
            .iter()
            .find(|s| s.contains("[Total_Price_Room]"))
            .expect("totals UPDATE must be emitted");
        assert_eq!(
            *totals,
            "UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK) SET \
             [Total_Price_Room]=1780.00,\
             [Total_Price_Net]=1780.00,\
             [Total_Price_Balance]=1780.00-(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)\
             +ISNULL(Cin_Pay_Credit,0)+ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)\
             +ISNULL(Cin_Pay_web,0)),0) FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
             WHERE Cin_No='CH26-005230' AND ISNULL(Cin_Status,'1') <> 'ยกเลิก') \
             where [Cin_no]='CH26-005230'"
        );
        // The two columns the capture never touches must NOT appear.
        assert!(
            !totals.contains("[Total_Price_Product]"),
            "extend must not clobber Total_Price_Product: {totals}"
        );
        assert!(
            !totals.contains("[Total_Price_Pay]"),
            "extend must not clobber Total_Price_Pay: {totals}"
        );
    }

    /// Wave 6: enumerate_calendar_nights now lives in `writeback::format`
    /// and returns Result. Keep the extend-stay-specific coverage; the
    /// empty-range / cap-truncate guards are tested in `format.rs`.
    #[test]
    fn enumerate_calendar_nights_covers_full_extended_range() {
        // Stay started Apr 24, extend to Apr 27 → 3 nights (24, 25, 26).
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 27, 12, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(
            nights,
            vec![
                NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ]
        );
    }

    /// Wave 5b item 2: step-5's `room_use='yes'` revert must use the same
    /// subquery shape as step-1's `room_use='no'` clear, otherwise a
    /// multi-room check-in leaves every sibling room stuck `'no'` after the
    /// extend. Symmetric subqueries guarantee step-1 / step-5 cover the
    /// exact same set of rows.
    #[test]
    fn step_5_uses_subquery_for_multi_room() {
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            new_nights: 2,
            new_room_price_total: 1780.0,
            new_net_total: 1780.0,
            guest_label: "MULTI",
            nights: vec![],
            room_status_id_base: 50235,
            tm30_touch_id: None,
        };
        let statements = build_statements(&inputs);
        let step5 = statements
            .iter()
            .find(|s| {
                s.starts_with("update HT_Rooms set room_use='yes'") && s.contains("select")
            })
            .expect("step-5 must use subquery for multi-room parity");
        assert!(step5.contains("Cin_no='CH26-005230'"));
        assert!(step5.contains("Cin_Room_Status<>'Check-Out'"));
        // And there must be no single-room form left over (`room_no='508'`
        // suffix on the same update statement).
        assert!(
            !step5.ends_with("where room_no='508'"),
            "single-room form leaked: {step5}"
        );
    }

    /// Wave 5b item 2: a single-room extend (the common case) still works —
    /// the subquery resolves to exactly that one room, so behaviour stays
    /// unchanged for the 99% path.
    #[test]
    fn single_room_input_still_works() {
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap(),
            new_nights: 1,
            new_room_price_total: 890.0,
            new_net_total: 890.0,
            guest_label: "SOLO",
            nights: vec![NaiveDate::from_ymd_opt(2026, 4, 25).unwrap()],
            room_status_id_base: 50235,
            tm30_touch_id: None,
        };
        let statements = build_statements(&inputs);
        // Step-1 and step-5 must share the EXACT same subquery / WHERE shape
        // (only differ on `room_use='no'` vs `'yes'`).
        let step1 = statements.iter().find(|s| s.contains("room_use='no'")).unwrap();
        let step5 = statements.iter().find(|s| s.contains("room_use='yes'")).unwrap();
        let step1_tail = step1.trim_start_matches("update HT_Rooms set room_use='no'");
        let step5_tail = step5.trim_start_matches("update HT_Rooms set room_use='yes'");
        assert_eq!(
            step1_tail, step5_tail,
            "step-1 and step-5 must share the WHERE / subquery shape"
        );
        // And the per-night HT_Room_Status INSERT still references the
        // single room number — proving room_no is not vestigial.
        let night = statements
            .iter()
            .find(|s| s.contains("INSERT INTO [HT_Room_Status]"))
            .unwrap();
        assert!(night.contains("'508'"));
    }

    #[test]
    fn no_destructive_phase_b_in_emitted_statements() {
        // Spike §3f explicitly says skip Phase B (delete+reinsert HT_CheckIn_*).
        // Verify our recipe doesn't emit any DELETE on HT_CheckIn_*.
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            new_nights: 2,
            new_room_price_total: 1780.0,
            new_net_total: 1780.0,
            guest_label: "",
            nights: vec![],
            room_status_id_base: 50235,
            tm30_touch_id: None,
        };
        let statements = build_statements(&inputs);
        for s in &statements {
            assert!(
                !s.contains("delete from HT_CheckIn_H")
                    && !s.contains("delete from HT_CheckIn_Ds")
                    && !s.contains("delete from HT_CheckIn_Product")
                    && !s.contains("delete from HT_CheckIn_Other_People"),
                "destructive Phase B statement leaked: {s}"
            );
        }
    }
}
