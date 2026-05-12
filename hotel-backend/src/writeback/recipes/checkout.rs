//! `CheckOut` recipe — spike `findings.md` §3e **Phase 2 only**.
//!
//! 5 UPDATEs, no destructive Phase 1. Per spike §3e:
//! > **For our writeback**: we should **NOT** replicate this destructive
//! > pattern. Use targeted UPDATEs against `HT_Book_H` / `HT_Book_Ds` instead,
//! > and add/remove `HT_Book_Date` rows as needed.
//!
//! Reference SQL (verbatim from `checkout-20260424-100323/writes.txt` lines 19-23):
//!
//! ```text
//! 1. update HT_POWER_LOG SET ROOM_POWER_END=GETDATE(),
//!      ROOM_POWER_END_BY='Admin',
//!      ROOM_POWER_NOTE2='ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.CH26-005228'
//!    where room_no='402' and ROOM_POWER_END_BY=''
//!
//! 2. update [HT_CheckIn_Ds] SET
//!      [Cin_Room_Out]='4/24/2026 5:05:04 PM',
//!      [Cin_Room_Status]='Check-Out',                  -- English with HYPHEN
//!      [Cin_Room_Pay_Total]=0,
//!      [Cin_Room_night]=1,
//!      [Cin_Room_PriceTotal]=0,
//!      [Cin_note]=''
//!    where id=25007                                    -- by HT_CheckIn_Ds.id
//!
//! 3. update HT_Rooms set room_use='no', Room_Clean='yes', Room_Use_Count=Room_Use_Count+1
//!    where room_no='402'
//!
//! 4. update HT_Room_Status SET room_status='Check Out'  -- English with SPACE
//!    where room_no='402' and room_CheckIn_No='CH26-005228'
//!
//! 5. UPDATE [HT_CheckIn_H] SET
//!      [Total_Price_Room]=0.00, [Total_Price_Product]=0.00, [Total_Price_Net]=0.00,
//!      [Total_Price_Pay]=0.00, [Total_Price_Balance]=0.00, [Cin_note]=''
//!    where [Cin_no]='CH26-005228'
//! ```
//!
//! Spike §3e critical findings:
//! - `'Check-Out'` with HYPHEN on `HT_CheckIn_Ds.Cin_Room_Status`
//! - `'Check Out'` with SPACE on `HT_Room_Status.room_status`
//! - `Room_Use_Count` is incremented, NOT replaced
//! - The capture zeros all totals — this matches the receptionist's flow where
//!   payment was already taken (balance=0). The recipe takes totals as inputs
//!   so the worker can pass real values.
//! - Power-log note format: `ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.{cin_no}`

use chrono::Utc;

use crate::writeback::allocate::LegacyConn;
use crate::writeback::constants::{
    power_log_note_check_out, CIN_ROOM_STATUS_CHECKED_OUT, DEFAULT_OPERATOR,
    ROOM_STATUS_CHECKED_OUT,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, sql_quote};

/// Inputs for the check-out recipe.
#[derive(Debug, Clone, Copy)]
pub struct CheckOutInputs<'a> {
    pub cin_no: &'a str,
    pub room_no: &'a str,
    /// `HT_CheckIn_Ds.id` — the row to UPDATE. Numeric internal PK, distinct
    /// from `Cin_no`. Per spike §3e the legacy app filters by `id`.
    pub checkin_ds_id: i32,
    pub room_price_total: f64,
    pub product_total: f64,
    pub net_total: f64,
    pub pay_total: f64,
    pub balance: f64,
    pub nights: f64,
}

/// Build the 5 statements that complete a check-out. PURE — no I/O.
pub fn build_statements(inputs: &CheckOutInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let room_no_q = sql_quote(inputs.room_no);
    let by_q = sql_quote(DEFAULT_OPERATOR);
    let now_str = format_legacy_datetime(Utc::now());
    let now_q = sql_quote(&now_str);
    let power_note = power_log_note_check_out(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    let cin_status_q = sql_quote(CIN_ROOM_STATUS_CHECKED_OUT);
    let room_status_q = sql_quote(ROOM_STATUS_CHECKED_OUT);
    let ds_id = inputs.checkin_ds_id;
    let room_price = inputs.room_price_total;
    let product_total = inputs.product_total;
    let net = inputs.net_total;
    let pay = inputs.pay_total;
    let balance = inputs.balance;
    let nights = inputs.nights;
    // Audit H2: Room_Use_Count must be bumped by the real nights count
    // (COMPAT_CHEATSHEET.md:289 / 1164), not always +1. Cast to i64 — the
    // legacy column is INT; floor to integer nights (the payload may carry
    // fractional values from rate math, but the usage counter is whole).
    let nights_int = nights.max(0.0) as i64;

    vec![
        // 1. Lights off — finds the in-progress entry by ROOM_POWER_END_BY=''
        format!(
            "update HT_POWER_LOG SET ROOM_POWER_END=GETDATE(),ROOM_POWER_END_BY={by_q},\
             ROOM_POWER_NOTE2={power_note_q} where room_no={room_no_q} and ROOM_POWER_END_BY=''"
        ),
        // 2. Stamp check-out on HT_CheckIn_Ds (by id) — Cin_Room_Status='Check-Out' (HYPHEN)
        format!(
            "update [HT_CheckIn_Ds] SET  [Cin_Room_Out]={now_q},[Cin_Room_Status]={cin_status_q},\
             [Cin_Room_Pay_Total]={pay},[Cin_Room_night]={nights},\
             [Cin_Room_PriceTotal]={room_price},[Cin_note]='' where id={ds_id}"
        ),
        // 3. Free the room + flag dirty + bump usage counter by nights (audit H2)
        format!(
            "update HT_Rooms set room_use='no',Room_Clean='yes',\
             Room_Use_Count=Room_Use_Count+{nights_int} where room_no={room_no_q}"
        ),
        // 4. Stamp check-out on HT_Room_Status — room_status='Check Out' (SPACE)
        format!(
            "update HT_Room_Status SET room_status={room_status_q} where room_no={room_no_q} \
             and room_CheckIn_No={cin_no_q}"
        ),
        // 5. Final totals on HT_CheckIn_H
        format!(
            "UPDATE [HT_CheckIn_H] SET  [Total_Price_Room]={room_price},\
             [Total_Price_Product]={product_total},[Total_Price_Net]={net},\
             [Total_Price_Pay]={pay},[Total_Price_Balance]={balance},[Cin_note]='' \
             where [Cin_no]={cin_no_q}"
        ),
    ]
}

/// Execute the check-out recipe.
///
/// Audit H1: callers must thread the real revenue totals + nights from
/// the canonical PG state. The legacy-event fallback (when the payload
/// pre-dates the H1 fix and the totals are not yet on the intent) passes
/// zeros — matching the prior buggy behavior — and logs a WARN at the
/// dispatcher so the partial sync is visible in worker logs.
#[allow(clippy::too_many_arguments)]
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    room_no: &str,
    checkin_ds_id: i32,
    nights: f64,
    room_price_total: f64,
    product_total: f64,
    net_total: f64,
    pay_total: f64,
    balance: f64,
) -> WritebackResult<LegacyIds> {
    // Audit H13: reject NaN/Infinity for every monetary input before any SQL
    // formatting. `format!("{}", f64::NAN)` emits literal `"NaN"`, which
    // would cause MSSQL to reject the UPDATE mid-transaction and leave the
    // check-out partially applied (power log already stamped off but totals
    // never overwritten). Unlike walkin/checkin_to_booking these values flow
    // straight from the caller as f64 — there is no Money type to guarantee
    // finiteness — so the check is necessary, not just defense-in-depth.
    super::helpers::validate_finite(&[
        ("nights", nights),
        ("room_price_total", room_price_total),
        ("product_total", product_total),
        ("net_total", net_total),
        ("pay_total", pay_total),
        ("balance", balance),
    ])?;

    let inputs = CheckOutInputs {
        cin_no,
        room_no,
        checkin_ds_id,
        room_price_total,
        product_total,
        net_total,
        pay_total,
        balance,
        nights,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_cin_no(cin_no.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_statements_matches_spike_capture_structure() {
        // Mirror the values from checkout-20260424-100323/writes.txt:19-23
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 0.0,
            product_total: 0.0,
            net_total: 0.0,
            pay_total: 0.0,
            balance: 0.0,
            nights: 1.0,
        };
        let statements = build_statements(&inputs);
        assert_eq!(statements.len(), 5);

        // 1. Power log
        assert!(statements[0].contains("HT_POWER_LOG"));
        assert!(statements[0].contains("ROOM_POWER_END_BY=''"));
        assert!(statements[0].contains("ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.CH26-005228"));

        // 2. CheckIn_Ds — CRITICAL: Cin_Room_Status='Check-Out' WITH HYPHEN
        assert!(statements[1].contains("[Cin_Room_Status]='Check-Out'"));
        assert!(statements[1].contains("where id=25007"));

        // 3. HT_Rooms
        assert!(statements[2].contains("Room_Use_Count=Room_Use_Count+1"));
        assert!(statements[2].contains("room_use='no'"));
        assert!(statements[2].contains("Room_Clean='yes'"));

        // 4. Room_Status — CRITICAL: 'Check Out' WITH SPACE
        assert!(statements[3].contains("room_status='Check Out'"));
        assert!(statements[3].contains("room_CheckIn_No='CH26-005228'"));

        // 5. HT_CheckIn_H
        assert!(statements[4].contains("[Cin_no]='CH26-005228'"));
    }

    #[test]
    fn check_out_status_uses_hyphen_not_space_on_checkin_ds() {
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 0.0,
            product_total: 0.0,
            net_total: 0.0,
            pay_total: 0.0,
            balance: 0.0,
            nights: 1.0,
        };
        let statements = build_statements(&inputs);
        assert!(statements[1].contains("'Check-Out'"));
        assert!(!statements[1].contains("'Check Out'"));
    }

    #[test]
    fn room_status_uses_space_not_hyphen_on_room_status() {
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 0.0,
            product_total: 0.0,
            net_total: 0.0,
            pay_total: 0.0,
            balance: 0.0,
            nights: 1.0,
        };
        let statements = build_statements(&inputs);
        assert!(statements[3].contains("'Check Out'"));
        assert!(!statements[3].contains("'Check-Out'"));
    }

    #[test]
    fn totals_are_passed_through() {
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 1780.0,
            product_total: 0.0,
            net_total: 1780.0,
            pay_total: 1780.0,
            balance: 0.0,
            nights: 2.0,
        };
        let statements = build_statements(&inputs);
        assert!(statements[4].contains("[Total_Price_Room]=1780"));
        assert!(statements[4].contains("[Total_Price_Net]=1780"));
        assert!(statements[4].contains("[Total_Price_Pay]=1780"));
        assert!(statements[4].contains("[Total_Price_Balance]=0"));
    }

    /// Fix for audit H1: a 3-night checkout with real revenue totals
    /// (2670 baht) must write those values to MSSQL — not zeros. The prior
    /// `execute()` hardcoded all totals to 0 regardless of the actual stay,
    /// wiping real revenue from MSSQL on every checkout.
    #[test]
    fn build_statements_uses_real_revenue_totals() {
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 2670.0,
            product_total: 0.0,
            net_total: 2670.0,
            pay_total: 2670.0,
            balance: 0.0,
            nights: 3.0,
        };
        let statements = build_statements(&inputs);
        // HT_CheckIn_Ds row (statement 2): Cin_Room_PriceTotal must equal
        // the real room-revenue total, not 0.
        assert!(
            statements[1].contains("[Cin_Room_PriceTotal]=2670"),
            "Cin_Room_PriceTotal must reflect real room revenue; got:\n{}",
            statements[1]
        );
        // Cin_Room_night must be the real nights count.
        assert!(
            statements[1].contains("[Cin_Room_night]=3"),
            "Cin_Room_night must reflect real nights; got:\n{}",
            statements[1]
        );
        // HT_CheckIn_H row (statement 5): Total_Price_* must reflect real
        // figures.
        assert!(
            statements[4].contains("[Total_Price_Room]=2670"),
            "Total_Price_Room must reflect real room revenue; got:\n{}",
            statements[4]
        );
        assert!(
            statements[4].contains("[Total_Price_Net]=2670"),
            "Total_Price_Net must reflect real net; got:\n{}",
            statements[4]
        );
        assert!(
            statements[4].contains("[Total_Price_Pay]=2670"),
            "Total_Price_Pay must reflect real pay total; got:\n{}",
            statements[4]
        );
    }

    /// Audit H13: checkout `execute()` must reject NaN/Infinity for every
    /// monetary input before any SQL is formatted. Unlike walkin /
    /// checkin_to_booking these values flow straight from the caller as
    /// f64 (not derived from `Money`), so the check is necessary not just
    /// defense-in-depth: a NaN reaching `[Total_Price_Pay]={pay}` would
    /// emit `[Total_Price_Pay]=NaN` and fail the transaction after the
    /// power log was already stamped off. This test pins the labels.
    #[test]
    fn validate_finite_blocks_nan_in_checkout_execute_inputs() {
        let result = super::super::helpers::validate_finite(&[
            ("nights", f64::NAN),
            ("room_price_total", 2670.0),
            ("product_total", 0.0),
            ("net_total", 2670.0),
            ("pay_total", 2670.0),
            ("balance", 0.0),
        ]);
        let err = result.expect_err("NaN nights must be rejected");
        assert!(err.to_string().contains("nights"));
    }

    #[test]
    fn validate_finite_blocks_infinity_in_checkout_execute_inputs() {
        for (label, payload) in [
            ("room_price_total", f64::INFINITY),
            ("net_total", f64::NEG_INFINITY),
            ("pay_total", f64::INFINITY),
            ("balance", f64::INFINITY),
            ("product_total", f64::INFINITY),
        ] {
            let result = super::super::helpers::validate_finite(&[
                ("nights", 3.0),
                ("room_price_total", if label == "room_price_total" { payload } else { 2670.0 }),
                ("product_total", if label == "product_total" { payload } else { 0.0 }),
                ("net_total", if label == "net_total" { payload } else { 2670.0 }),
                ("pay_total", if label == "pay_total" { payload } else { 2670.0 }),
                ("balance", if label == "balance" { payload } else { 0.0 }),
            ]);
            let err = result.expect_err("Infinity must be rejected");
            assert!(err.to_string().contains(label), "label {label} not in: {err}");
        }
    }

    /// Fix for audit H2: Room_Use_Count is bumped by the real nights count
    /// (per COMPAT_CHEATSHEET.md:289), not always +1. Spike captures were
    /// 1-night stays so the bug was hidden.
    #[test]
    fn room_use_count_increments_by_nights_not_one() {
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 2670.0,
            product_total: 0.0,
            net_total: 2670.0,
            pay_total: 2670.0,
            balance: 0.0,
            nights: 3.0,
        };
        let statements = build_statements(&inputs);
        assert!(
            statements[2].contains("Room_Use_Count=Room_Use_Count+3"),
            "Room_Use_Count must be bumped by nights (3), not +1; got:\n{}",
            statements[2]
        );
    }
}
