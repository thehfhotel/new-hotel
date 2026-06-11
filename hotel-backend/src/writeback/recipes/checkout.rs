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

use chrono::{DateTime, Utc};
use tiberius::Row;

use crate::db::mssql_timeout::{simple_query_with_timeout, MssqlOpKind};
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
    /// "Now" timestamp threaded in by `execute()` so `build_statements`
    /// stays PURE — coexistence audit T6 HIGH-1. Drives
    /// `HT_CheckIn_Ds.Cin_Room_Out` (the per-room checkout stamp).
    pub created_at: DateTime<Utc>,
}

/// Build the 5 statements that complete a check-out. PURE — no I/O.
pub fn build_statements(inputs: &CheckOutInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let room_no_q = sql_quote(inputs.room_no);
    let by_q = sql_quote(DEFAULT_OPERATOR);
    let now_str = format_legacy_datetime(inputs.created_at);
    let now_q = sql_quote(&now_str);
    let power_note = power_log_note_check_out(inputs.cin_no);
    let power_note_q = sql_quote(&power_note);
    let cin_status_q = sql_quote(CIN_ROOM_STATUS_CHECKED_OUT);
    let room_status_q = sql_quote(ROOM_STATUS_CHECKED_OUT);
    let ds_id = inputs.checkin_ds_id;
    // Wave 6 LOW item 4: pre-format money to 2dp for consistency with the
    // HT_CheckIn_H / HT_Receipt_H formatting that already uses 2dp.
    let room_price = format!("{:.2}", inputs.room_price_total);
    let product_total = format!("{:.2}", inputs.product_total);
    let net = format!("{:.2}", inputs.net_total);
    // `pay` is still used on the HT_CheckIn_Ds per-room totals UPDATE
    // (the per-room column reflects the snapshot at checkout time; only
    // the header HT_CheckIn_H Total_Price_Pay is re-aggregated live —
    // see the §5 statement in this function for the rationale).
    let pay = format!("{:.2}", inputs.pay_total);
    // Track C — T5 HIGH-4: `inputs.balance` is no longer emitted as a
    // SQL literal — the `[Total_Price_Balance]` value in the
    // HT_CheckIn_H UPDATE is now `Net - (live aggregate)`. The input
    // remains in `CheckOutInputs` for the legacy-event sanity-check
    // surface and is referenced by `validate_finite` in `execute()`.
    let _balance_unused = inputs.balance;
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
        //
        //    Track C — T5 HIGH-4 (`docs/coexistence/audit-2026-05-13.md`):
        //    `Total_Price_Pay` is re-aggregated from `HT_CheckIn_Pay` rows
        //    under UPDLOCK+HOLDLOCK held through COMMIT — NOT trusted from
        //    the intent payload's `pay_total`. The intent's `pay_total` was
        //    computed from PG state at intent-emit time; if a second
        //    payment writeback commits between emit and this checkout's
        //    BEGIN TRAN, the stale `pay_total` would clobber the second
        //    payment from the legacy view. Re-aggregating live and locking
        //    the rows through COMMIT closes the window — same pattern as
        //    payment.rs (T5 CRIT-1) and the booking-edit recipe validated
        //    in spike §6.
        //
        //    `inputs.pay_total` is kept as a sanity-check input — `execute()`
        //    logs a WARN if the live aggregate drifts from the payload (see
        //    `pay_total_drift_threshold_baht`). The other totals
        //    (Room/Product/Net) remain authoritative from the payload —
        //    payment doesn't touch those columns so no concurrent-write
        //    risk exists there.
        //
        //    `cin_status <> 'ยกเลิก'` filter on the Pay rows excludes
        //    cancelled tender rows (T2 CRIT-2 — COMPAT_CHEATSHEET line 106).
        format!(
            "UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK) SET \
             [Total_Price_Room]={room_price},\
             [Total_Price_Product]={product_total},\
             [Total_Price_Net]={net},\
             [Total_Price_Pay]=(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)+ISNULL(Cin_Pay_Credit,0)\
             +ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)+ISNULL(Cin_Pay_web,0)),0) \
             FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
             WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก'),\
             [Total_Price_Balance]={net}-(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)\
             +ISNULL(Cin_Pay_Credit,0)+ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)\
             +ISNULL(Cin_Pay_web,0)),0) FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
             WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก'),\
             [Cin_note]='' where [Cin_no]={cin_no_q}"
        ),
    ]
}

/// Threshold (in baht) at which `execute()` logs a WARN if the
/// intent payload's `pay_total` drifts from the live MSSQL aggregate
/// computed in the recipe's `HT_CheckIn_H` UPDATE.
///
/// Track C — T5 HIGH-4: the intent's `pay_total` is no longer used as
/// the authoritative value (we re-aggregate from rows under
/// UPDLOCK+HOLDLOCK) but we keep it as a sanity-check sentinel. A drift
/// at or above this threshold means a payment slipped in between intent
/// emit and checkout writeback — useful for monitoring but no longer a
/// correctness problem since the live aggregate wins.
const PAY_TOTAL_DRIFT_THRESHOLD_BAHT: f64 = 0.005;

/// SELECT the live sum of every tender column from `HT_CheckIn_Pay` for
/// the given `Cin_no`, excluding cancelled rows. Used by `execute()` to
/// sanity-check the intent payload's `pay_total` (Track C — T5 HIGH-4).
///
/// Returns `Ok(None)` if MSSQL returns no row (e.g. transient stream
/// error materialised as an empty result). The caller treats a missing
/// row as "skip the drift check" rather than aborting — the
/// authoritative aggregate is the in-UPDATE subquery in
/// `build_statements`, which always reads live.
async fn fetch_live_pay_total(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
) -> WritebackResult<Option<f64>> {
    let cin_no_q = sql_quote(cin_no);
    let sql = format!(
        "SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)+ISNULL(Cin_Pay_Credit,0)\
         +ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)+ISNULL(Cin_Pay_web,0)),0) \
         AS live_pay_total \
         FROM HT_CheckIn_Pay \
         WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก'"
    );
    // R2 (2026-05-14): payment-sum SELECT inside the checkout recipe
    // BEGIN TRAN — write budget for the consistent transaction
    // envelope.
    let rows: Vec<Row> = simple_query_with_timeout(conn, &sql, MssqlOpKind::Write).await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    // The aggregate is rendered as `float` by SQL Server — tiberius
    // returns it as f64. `live_pay_total` is the column alias.
    let v: Option<f64> = row.get(0);
    Ok(v)
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

    // Coexistence audit T6 HIGH-1: capture `Utc::now()` once at the entry to
    // `execute()` so `build_statements` is purely a function of its inputs.
    let created_at = Utc::now();

    // Track C — T5 HIGH-4: sanity-check the intent payload's `pay_total`
    // against the live MSSQL aggregate. The actual SQL UPDATE re-aggregates
    // from rows under UPDLOCK+HOLDLOCK so the intent's value is no longer
    // authoritative; this log surfaces the drift for observability.
    if let Some(live_pay_total) = fetch_live_pay_total(conn, cin_no).await? {
        if (live_pay_total - pay_total).abs() >= PAY_TOTAL_DRIFT_THRESHOLD_BAHT {
            tracing::warn!(
                cin_no,
                intent_pay_total = pay_total,
                live_pay_total,
                drift_baht = (live_pay_total - pay_total).abs(),
                "checkout intent payload pay_total drifted from live HT_CheckIn_Pay \
                 aggregate (concurrent payment writeback between intent emit and \
                 checkout commit). Live aggregate wins — Track C T5 HIGH-4."
            );
        }
    }

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
        created_at,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;
    Ok(LegacyIds::new().with_cin_no(cin_no.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Fixed instant used by every CheckOutInputs test literal so
    /// `Cin_Room_Out` renders to a stable wall-clock string. T6 HIGH-1.
    fn pinned_now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 4, 24, 10, 5, 4).unwrap()
    }

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
            created_at: pinned_now(),
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
            created_at: pinned_now(),
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
            created_at: pinned_now(),
        };
        let statements = build_statements(&inputs);
        assert!(statements[3].contains("'Check Out'"));
        assert!(!statements[3].contains("'Check-Out'"));
    }

    /// Track C — T5 HIGH-4: only the non-Pay totals (Room, Product, Net)
    /// remain literal pass-throughs from the intent payload. `Total_Price_Pay`
    /// and `Total_Price_Balance` are now SELECT subqueries aggregating from
    /// `HT_CheckIn_Pay` rows under UPDLOCK+HOLDLOCK — see
    /// `pay_total_re_aggregated_from_rows_not_input_payload` below.
    #[test]
    fn non_pay_totals_are_passed_through() {
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
            created_at: pinned_now(),
        };
        let statements = build_statements(&inputs);
        assert!(statements[4].contains("[Total_Price_Room]=1780"));
        assert!(statements[4].contains("[Total_Price_Net]=1780"));
        assert!(statements[4].contains("[Total_Price_Product]=0"));
        // Pay is no longer a literal — it's a SELECT subquery (T5 HIGH-4).
        assert!(
            !statements[4].contains("[Total_Price_Pay]=1780"),
            "Total_Price_Pay must NOT be the input literal — it must re-aggregate; got:\n{}",
            statements[4]
        );
        assert!(
            statements[4].contains("[Total_Price_Pay]=(SELECT"),
            "Total_Price_Pay must be a SELECT subquery; got:\n{}",
            statements[4]
        );
    }

    /// Coexistence audit T6 HIGH-1: `build_statements` is PURE — calling it
    /// twice with the same inputs produces byte-identical output (the
    /// `Cin_Room_Out` stamp derives from `inputs.created_at`, not a fresh
    /// `Utc::now()`).
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
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
            created_at: pinned_now(),
        };
        let first = build_statements(&inputs);
        let second = build_statements(&inputs);
        assert_eq!(first, second, "build_statements must be deterministic");
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
            created_at: pinned_now(),
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
        // HT_CheckIn_H row (statement 5): Room and Net remain literal
        // pass-throughs (no concurrent-write risk on those columns).
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
        // Track C — T5 HIGH-4: Pay is re-aggregated, not literal.
        assert!(
            statements[4].contains("[Total_Price_Pay]=(SELECT"),
            "Total_Price_Pay must be the re-aggregate subquery (T5 HIGH-4); got:\n{}",
            statements[4]
        );
    }

    /// Track C — T5 HIGH-4: the HT_CheckIn_H totals UPDATE must re-aggregate
    /// `Total_Price_Pay` from `HT_CheckIn_Pay` rows under UPDLOCK+HOLDLOCK
    /// rather than trusting the intent payload's `pay_total`. The payload's
    /// `pay_total` was computed from PG state at intent-emit time; if a
    /// payment writeback commits between emit and the checkout's
    /// BEGIN TRAN, the stale `pay_total` would clobber the second payment.
    /// Re-aggregating live and locking through COMMIT closes the window.
    #[test]
    fn pay_total_re_aggregated_from_rows_not_input_payload() {
        // Pass a deliberately wrong `pay_total` (9999) — the SQL must not
        // contain that literal because the value comes from a subquery.
        let inputs = CheckOutInputs {
            cin_no: "CH26-005228",
            room_no: "402",
            checkin_ds_id: 25007,
            room_price_total: 2670.0,
            product_total: 0.0,
            net_total: 2670.0,
            pay_total: 9999.0, // intent-payload value — must NOT appear in SQL
            balance: -7329.0,  // intent-payload balance — also must NOT appear
            nights: 3.0,
            created_at: pinned_now(),
        };
        let statements = build_statements(&inputs);
        let h = &statements[4];

        // The intent's pay_total literal must NOT be embedded.
        assert!(
            !h.contains("[Total_Price_Pay]=9999"),
            "intent payload pay_total must NOT be emitted as a literal; got:\n{h}"
        );
        // Header UPDATE must take UPDLOCK+HOLDLOCK (held through COMMIT).
        assert!(
            h.contains("UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK)"),
            "HT_CheckIn_H UPDATE must take UPDLOCK+HOLDLOCK; got:\n{h}"
        );
        // Pay-from-rows aggregate must hold UPDLOCK+HOLDLOCK on HT_CheckIn_Pay.
        assert!(
            h.contains("FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK)"),
            "Pay aggregate SELECT must take UPDLOCK+HOLDLOCK on HT_CheckIn_Pay; got:\n{h}"
        );
        // Aggregate must sum every tender column (Cash + Credit + Tran + Free + web).
        for col in [
            "Cin_Pay_Cash",
            "Cin_Pay_Credit",
            "Cin_Pay_Tran",
            "Cin_Pay_Free",
            "Cin_Pay_web",
        ] {
            assert!(
                h.contains(col),
                "Pay aggregate must include {col}; got:\n{h}"
            );
        }
        // Cancelled tender rows filtered (T2 CRIT-2).
        assert!(
            h.contains("ISNULL(Cin_Status,'1') <> 'ยกเลิก'"),
            "Pay aggregate must exclude cancelled (Cin_Status='ยกเลิก'); got:\n{h}"
        );
        // Balance must be Net - aggregate (not the intent's balance literal).
        assert!(
            !h.contains("[Total_Price_Balance]=-7329"),
            "intent payload balance must NOT be emitted as a literal; got:\n{h}"
        );
        assert!(
            h.contains("[Total_Price_Balance]=2670.00-(SELECT"),
            "Total_Price_Balance must be Net - aggregate subquery; got:\n{h}"
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
            created_at: pinned_now(),
        };
        let statements = build_statements(&inputs);
        assert!(
            statements[2].contains("Room_Use_Count=Room_Use_Count+3"),
            "Room_Use_Count must be bumped by nights (3), not +1; got:\n{}",
            statements[2]
        );
    }
}
