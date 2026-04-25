//! `ExtendStay` recipe — spike `findings.md` §3f Phase A.
//!
//! Recompute totals, replace `HT_Room_Status` rows for the changed range.
//! Targeted UPDATEs — we **skip the legacy app's destructive Phase B**
//! (DELETE+REINSERT) per spike §3f recommendation.
//!
//! Reference SQL (verbatim from `extend-20260424-101350/writes.txt` lines 3-9):
//!
//! ```text
//! 1. update HT_Rooms set room_use='no'
//!    where room_no in (select Cin_Room_No from HT_CheckIn_Ds
//!                       where Cin_no='CH26-005230' and Cin_Room_Status<>'Check-Out')
//!
//! 2. delete from HT_Room_Status where room_CheckIn_No='CH26-005230'
//!
//! 3. UPDATE [HT_CheckIn_H] SET
//!      [Total_Price_Room]=1780.00, [Total_Price_Product]=0.00,
//!      [Total_Price_Net]=1780.00, [Total_Price_Pay]=0.00,
//!      [Total_Price_Balance]=1780.00
//!    where [Cin_no]='CH26-005230'
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

use chrono::{DateTime, Datelike, Days, NaiveDate, Utc};

use crate::domain::shared::Money;
use crate::writeback::allocate::{allocate_room_status_id, LegacyConn};
use crate::writeback::constants::{CIN_ROOM_STATUS_OCCUPYING, ROOM_STATUS_OCCUPYING};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    date_to_ole_serial, format_legacy_date, format_legacy_datetime, sql_quote,
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
    pub new_balance: f64,
    pub product_total: f64,
    pub pay_total: f64,
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
    /// Random TM.30 batch numbers — spike §3a + §3f capture lines 1-2.
    /// The .NET app emits two `UPDATE HT_CheckIn_H SET Cin_Work_number=<rand>`
    /// before every save. Each is a non-sequential i32. The caller generates
    /// these via `rand::random::<i32>()` so `build_statements` stays pure.
    /// An empty Vec emits zero touches (useful for tests that focus on the
    /// downstream statements).
    pub tm30_touch_ids: Vec<i32>,
}

/// Build all statements for an extend-stay. PURE — no I/O.
///
/// `tm30_touch_ids` injects the two leading TM.30 touches (spike §3a + §3f
/// `extend/writes.txt:1,2`). Each is a random `i32` per spike §3a — the
/// caller (`execute()`) generates two via `rand::random()` so this function
/// stays pure and trivially unit-testable.
pub fn build_statements(inputs: &ExtendStayInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let room_no_q = sql_quote(inputs.room_no);
    let new_end_q = sql_quote(&format_legacy_datetime(inputs.new_end));
    let occupying_q = sql_quote(ROOM_STATUS_OCCUPYING); // == CIN_ROOM_STATUS_OCCUPYING
    let _ = CIN_ROOM_STATUS_OCCUPYING; // silence the unused-constant warning if any
    let label_q = sql_quote(inputs.guest_label);

    let mut statements: Vec<String> = Vec::new();

    // 0a/0b. TM.30 touches — spike §3a (`Cin_Work_number` is random + async)
    //        and §3f extend capture lines 1-2: the .NET app fires two such
    //        UPDATEs at the start of every Save action that touches a
    //        check-in. Random i32 per touch, supplied by caller.
    for tm30_id in &inputs.tm30_touch_ids {
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
    // 3. Update HT_CheckIn_H totals
    statements.push(format!(
        "UPDATE [HT_CheckIn_H] SET  [Total_Price_Room]={room_price},\
         [Total_Price_Product]={product},\
         [Total_Price_Net]={net},\
         [Total_Price_Pay]={pay},\
         [Total_Price_Balance]={balance} \
         where [Cin_no]={cin_no_q}",
        room_price = inputs.new_room_price_total,
        product = inputs.product_total,
        net = inputs.new_net_total,
        pay = inputs.pay_total,
        balance = inputs.new_balance,
    ));
    // 4. Update HT_CheckIn_Ds (by id) with new nights + price + departure
    statements.push(format!(
        "update [HT_CheckIn_Ds] SET  [Cin_Room_night]={nights},\
         [Cin_Room_PriceTotal]={price_total},[Cin_note]='',\
         [Cin_Room_Out]={new_end_q} where id={ds_id}",
        nights = inputs.new_nights,
        price_total = inputs.new_room_price_total,
        ds_id = inputs.checkin_ds_id,
    ));
    // 5. Revert room_use back to 'yes' (parity with legacy capture)
    statements.push(format!(
        "update HT_Rooms set room_use='yes' where room_no={room_no_q}"
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
/// Now consumes the full payload (spike §3f) — `stay_start`, `guest_label`,
/// and the four totals (`new_room_price_total`, `new_net_total`, `new_pay`,
/// `new_balance`) come from the [`WritebackIntent::ExtendStay`] variant the
/// service layer enriches before enqueuing. Calendar nights span the full
/// `[stay_start, new_end)` range so `HT_Room_Status` rows cover the entire
/// stay (not just `[today, new_end)` as the prior implementation did).
///
/// The two leading TM.30 touches (capture lines 1-2) are generated via
/// `rand::random::<i32>()` per spike §3a — `Cin_Work_number` is a
/// non-sequential random i32 the .NET app assigns ~5s after each save.
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

    let nights = enumerate_calendar_nights(stay_start, new_end);
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
        new_balance: money_to_baht_f64(new_balance_total),
        product_total: 0.0,
        pay_total: money_to_baht_f64(new_pay_total),
        guest_label,
        nights,
        room_status_id_base: id_base,
        // Spike §3a: TM.30 batch numbers are non-sequential random i32
        // assigned by the .NET app's async post-save job. We mirror two
        // touches per the extend capture (lines 1-2). MED-3: clamp to the
        // positive i32 range — the .NET app's Cin_Work_number column is
        // signed but no spike capture has ever observed a negative value,
        // and a negative number may trip the WinForms grid control.
        tm30_touch_ids: vec![positive_i32(), positive_i32()],
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_cin_no(cin_no.to_string()))
}

/// Enumerate calendar nights in `[stay_start, new_end)` — one
/// `HT_Room_Status` row per night. Capped at 365 nights as a safety net.
fn enumerate_calendar_nights(
    stay_start: DateTime<Utc>,
    new_end: DateTime<Utc>,
) -> Vec<NaiveDate> {
    let start = crate::writeback::format::bangkok_date(stay_start);
    let end = crate::writeback::format::bangkok_date(new_end);
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

    /// Verifies the structure against the spike capture's first 9 statements.
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
            new_balance: 1780.0,
            product_total: 0.0,
            pay_total: 0.0,
            guest_label: "SPIKE TEST WALKIN 3",
            nights: vec![
                NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
            ],
            room_status_id_base: 50235,
            tm30_touch_ids: vec![],
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
        // 3: totals on HT_CheckIn_H
        assert!(statements[2].contains("[Total_Price_Room]=1780"));
        assert!(statements[2].contains("[Total_Price_Net]=1780"));
        assert!(statements[2].contains("[Total_Price_Balance]=1780"));
        // 4: HT_CheckIn_Ds (by id)
        assert!(statements[3].contains("[Cin_Room_night]=2"));
        assert!(statements[3].contains("[Cin_Room_PriceTotal]=1780"));
        assert!(statements[3].contains("[Cin_Room_Out]='4/26/2026 12:00:00 PM'"));
        assert!(statements[3].contains("where id=25009"));
        // 5: revert room_use
        assert_eq!(
            statements[4],
            "update HT_Rooms set room_use='yes' where room_no='508'"
        );
        // 6: night 1 INSERT
        assert!(statements[5].contains("VALUES(50235,'508','4/24/2026','เข้าพัก'"));
        assert!(statements[5].contains(",46136)"));
        // 7: night 2 INSERT
        assert!(statements[6].contains("VALUES(50236,'508','4/25/2026','เข้าพัก'"));
        assert!(statements[6].contains(",46137)"));
    }

    #[test]
    fn tm30_touches_lead_when_provided() {
        // Spike §3f capture lines 1-2: two leading TM.30 UPDATEs before any
        // other statement.
        let inputs = ExtendStayInputs {
            cin_no: "CH26-005230",
            room_no: "508",
            checkin_ds_id: 25009,
            new_end: Utc.with_ymd_and_hms(2026, 4, 26, 12, 0, 0).unwrap(),
            new_nights: 2,
            new_room_price_total: 1780.0,
            new_net_total: 1780.0,
            new_balance: 1780.0,
            product_total: 0.0,
            pay_total: 0.0,
            guest_label: "SPIKE TEST WALKIN 3",
            nights: vec![],
            room_status_id_base: 50235,
            tm30_touch_ids: vec![539215, 539216],
        };
        let statements = build_statements(&inputs);
        // First two statements must be TM.30 touches in order.
        assert!(statements[0].contains("Cin_Work_number=539215"));
        assert!(statements[0].contains("Cin_No='CH26-005230'"));
        assert!(statements[1].contains("Cin_Work_number=539216"));
        assert!(statements[1].contains("Cin_No='CH26-005230'"));
    }

    #[test]
    fn enumerate_calendar_nights_covers_full_extended_range() {
        // Stay started Apr 24, extend to Apr 27 → 3 nights (24, 25, 26).
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 27, 12, 0, 0).unwrap(),
        );
        assert_eq!(
            nights,
            vec![
                NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ]
        );
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
            new_balance: 1780.0,
            product_total: 0.0,
            pay_total: 0.0,
            guest_label: "",
            nights: vec![],
            room_status_id_base: 50235,
            tm30_touch_ids: vec![],
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
