//! `RefundPayment` recipe — Track G2 / T4 CRIT-1.
//!
//! The legacy iHOTEL app records refunds as a `HT_CheckIn_Pay` row with a
//! NEGATIVE `Cin_Pay_Cash` (or `Credit`/`Tran`/`web`) amount per
//! `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay` (A)"
//! "can be negative (refunds use negation)":
//!
//! > Cin_Pay_Cash/Cin_Pay_Credit: split tender amounts, can be negative
//! > (refunds use negation).
//!
//! The recipe mirrors `recipes::payment` byte-for-byte EXCEPT:
//!
//! 1. The tender column carries `-amount` (negative).
//! 2. `Cin_Pay_Ds_Price` / `Cin_Pay_Ds_PriceTotal` also carry the
//!    negative amount so the legacy invariant
//!    `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web` holds.
//! 3. `Cin_Pay_Note` carries a `REFUND of Pay_no=<original>` prefix plus
//!    the operator-supplied reason — so iHOTEL's audit traces the refund
//!    back to the original payment row.
//! 4. NO `HT_Receipt_H` / `HT_Receipt_Ds` rows are emitted. Refunds are
//!    a tender adjustment, not a new receipt; reprinting refund receipts
//!    is a Track G follow-on if needed.
//!
//! The `HT_CheckIn_H` totals UPDATE uses the same UPDLOCK+HOLDLOCK
//! re-aggregation pattern as the original payment recipe (Track C
//! T5 CRIT-1) — re-summing `HT_CheckIn_Pay` rows under serializable-range
//! locks held through COMMIT, so concurrent iHOTEL absolute SETs cannot
//! clobber our contribution.

use crate::domain::payment::PaymentMethod;
use crate::domain::shared::Money;
use crate::writeback::allocate::{allocate_pay_no, LegacyConn};
use crate::writeback::constants::{
    BRANCH_HEAD_OFFICE, DEFAULT_OPERATOR, PAY_DS_ID_ROOM, PAY_DS_NAME_ROOM, PAY_DS_UNIT_ITEM,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, money_2dp, sql_quote};
use chrono::{DateTime, Utc};

/// Inputs for the refund + totals re-aggregation recipe.
///
/// `amount_baht` is POSITIVE — the recipe negates it before emitting SQL.
/// Keeping the carrier positive lets `money_2dp` continue to validate
/// finiteness against an unsigned magnitude and the negation lives in
/// exactly one place (this recipe).
#[derive(Debug, Clone)]
pub struct RefundInputs<'a> {
    pub cin_no: &'a str,
    pub cust_no: &'a str,
    pub room_no: &'a str,
    /// Magnitude of the refund in baht. Must be finite + non-negative.
    pub amount_baht: f64,
    pub method: PaymentMethod,
    /// New `HT_CheckIn_Pay.Pay_No` allocated for the refund row.
    pub pay_no: &'a str,
    /// Operator name attributed to the row (`Pay_by`). Defaults to
    /// [`DEFAULT_OPERATOR`] when None.
    pub operator: Option<&'a str>,
    /// `Pay_no` of the original payment this refund offsets. Embedded in
    /// `Cin_Pay_Note` so the legacy audit trail can join refunds back
    /// to their original payments. `None` when the worker hasn't
    /// back-populated the original's `legacy_pay_no` yet — the note
    /// then degrades to a generic `REFUND` prefix.
    pub original_legacy_pay_no: Option<&'a str>,
    /// Free-text operator reason. Captured verbatim into the note after
    /// the `REFUND of Pay_no=…` prefix. Empty string emits just the
    /// prefix.
    pub refund_reason: &'a str,
    /// "Now" timestamp threaded in by `execute()` so `build_statements`
    /// stays PURE — same contract as the `payment` recipe.
    pub created_at: DateTime<Utc>,
}

/// Build the refund statements. PURE — no I/O.
///
/// Emits 3 statements:
///
/// 1. `INSERT INTO HT_CheckIn_Pay` with negative tender amount and a
///    `WHERE NOT EXISTS` idempotency guard on `Pay_no` (matches the
///    `payment` recipe's audit-MED retry guard).
/// 2. `UPDATE HT_CheckIn_H WITH (UPDLOCK, HOLDLOCK)` re-aggregating
///    `Total_Price_Pay` + `Total_Price_Balance` from `HT_CheckIn_Pay`
///    rows (Track C pattern).
/// 3. `UPDATE HT_CheckIn_H WITH (UPDLOCK, HOLDLOCK)` re-aggregating
///    `Total_Price_vat` for the same reason.
///
/// Returns `Err` if `amount_baht` is non-finite or negative.
pub fn build_statements(
    inputs: &RefundInputs<'_>,
) -> Result<Vec<String>, crate::writeback::error::WritebackError> {
    if !inputs.amount_baht.is_finite() || inputs.amount_baht < 0.0 {
        return Err(crate::writeback::error::WritebackError::Recipe(format!(
            "refund amount must be finite and non-negative; got {}",
            inputs.amount_baht
        )));
    }

    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let room_no_q = sql_quote(inputs.room_no);
    let pay_no_q = sql_quote(inputs.pay_no);
    let now_q = sql_quote(&format_legacy_datetime(inputs.created_at));
    let amount = inputs.amount_baht;
    let neg_amount = -amount;
    let neg_amount_2dp = money_2dp(neg_amount)?;
    let pay_ds_name_q = sql_quote(PAY_DS_NAME_ROOM);
    let pay_ds_unit_q = sql_quote(PAY_DS_UNIT_ITEM);
    let pay_ds_id_q = sql_quote(PAY_DS_ID_ROOM);
    let operator_q = sql_quote(inputs.operator.unwrap_or(DEFAULT_OPERATOR));
    let branch_q = sql_quote(BRANCH_HEAD_OFFICE);
    let zero_2dp = money_2dp(0.0)?;

    // The same per-column tender routing as the `payment` recipe, but
    // the matched column carries `-amount` instead of `+amount`. The
    // legacy invariant
    // `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web` therefore
    // sums to the negative refund amount (matches `COMPAT_CHEATSHEET.md`
    // §"Table: `HT_CheckIn_Pay` (A)"
    // "Cin_Pay_Cash+Cin_Pay_Credit+Cin_Pay_Free+Cin_Pay_Tran+Cin_Pay_web").
    let (cash_2dp, credit_2dp, transfer_2dp, web_2dp) = match inputs.method {
        PaymentMethod::Cash => (
            money_2dp(neg_amount)?,
            zero_2dp.clone(),
            zero_2dp.clone(),
            zero_2dp.clone(),
        ),
        PaymentMethod::Credit => (
            zero_2dp.clone(),
            money_2dp(neg_amount)?,
            zero_2dp.clone(),
            zero_2dp.clone(),
        ),
        PaymentMethod::Transfer => (
            zero_2dp.clone(),
            zero_2dp.clone(),
            money_2dp(neg_amount)?,
            zero_2dp.clone(),
        ),
        PaymentMethod::Web => (
            zero_2dp.clone(),
            zero_2dp.clone(),
            zero_2dp.clone(),
            money_2dp(neg_amount)?,
        ),
    };

    // Compose the audit note: `"REFUND of Pay_no=<original>: <reason>"`.
    // When the original Pay_no isn't yet resolved (worker's
    // back-population still pending) fall back to a generic `"REFUND"`
    // prefix — the canonical `ht_payments.refund_of_payment_id` still
    // carries the link, so this is purely the legacy-side audit trail.
    let note = match inputs.original_legacy_pay_no {
        Some(orig) if !orig.is_empty() => {
            if inputs.refund_reason.is_empty() {
                format!("REFUND of Pay_no={orig}")
            } else {
                format!("REFUND of Pay_no={orig}: {reason}", reason = inputs.refund_reason)
            }
        }
        _ => {
            if inputs.refund_reason.is_empty() {
                "REFUND".to_string()
            } else {
                format!("REFUND: {reason}", reason = inputs.refund_reason)
            }
        }
    };
    let note_q = sql_quote(&note);

    let mut statements: Vec<String> = Vec::with_capacity(3);

    // 1. INSERT the refund row. Mirrors the `payment` recipe's 20-column
    //    canonical order verified against 24 captures in
    //    `/tmp/legacy-events-full.log`; the only differences are:
    //
    //    - Cash/Credit/Tran/Web columns carry NEGATIVE refund amount
    //    - Cin_Pay_Ds_Price + Cin_Pay_Ds_PriceTotal carry NEGATIVE refund
    //      (so the legacy invariant `Cin_Pay_Ds_Price = Cash + Credit +
    //      Free + Tran + Web` still holds)
    //    - Cin_Pay_Note carries the REFUND prefix + reason
    //    - Cin_Pay_Ds_PriceOne carries the positive unit "tender unit"
    //      (negative-of-negative cancellation: |refund| / 1 = |refund|).
    //      Same shape as the payment recipe when nights=1.
    //    - Cin_Pay_Ds_Num = 1 (refund is a single tender adjustment;
    //      multi-night apportionment is out of scope per the audit doc).
    //
    //    Audit MED — payment idempotency: wrap INSERT in
    //    `WHERE NOT EXISTS` on the freshly-allocated `Pay_no` so a retry
    //    after a network drop on COMMIT doesn't double-write.
    let unit_one_2dp = money_2dp(amount)?;
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Pay](  [Cin_No],[Cin_Pay_Ds],[Cin_Pay_Cash],[Cin_Pay_Credit],\
         [Cin_Pay_Date],[Cin_Pay_Ds_Name],[Cin_Pay_Ds_Price],[Cin_Pay_Ds_unit],[Pay_No],\
         [Cin_Cust_no],[Cin_Pay_Ds_ID],[Cin_Pay_Ds_Num],[Cin_Pay_Ds_PriceTotal],\
         [Cin_Pay_Ds_PriceOne],[Cin_Pay_Note],[Pay_by],[Cin_Pay_Free],[Cin_Pay_Tran],\
         [Branch],[Cin_Pay_web])\
         SELECT {cin_no_q},{room_no_q},{cash_2dp},{credit_2dp},{now_q},{pay_ds_name_q},\
         {neg_amount_2dp},{pay_ds_unit_q},{pay_no_q},{cust_no_q},{pay_ds_id_q},1,\
         {neg_amount_2dp},{unit_one_2dp},{note_q},{operator_q},{zero_2dp},{transfer_2dp},\
         {branch_q},{web_2dp} \
         WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay WHERE Pay_no={pay_no_q})"
    ));

    // 2. HT_CheckIn_H — re-aggregate `Total_Price_Pay` + `Total_Price_Balance`
    //    from `HT_CheckIn_Pay` rows under UPDLOCK+HOLDLOCK (Track C — T5
    //    CRIT-1). Pattern identical to the `payment` recipe: the new
    //    negative refund row is naturally included in the SUM, so the
    //    header totals settle to `prior_pay_total - refund` without an
    //    additive UPDATE. Cancelled tender rows excluded via the
    //    `ISNULL(Cin_Status,'1') <> 'ยกเลิก'` filter (T2 CRIT-2).
    statements.push(format!(
        "UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK) SET \
         [Total_Price_Pay]=(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)+ISNULL(Cin_Pay_Credit,0)\
         +ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)+ISNULL(Cin_Pay_web,0)),0) \
         FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
         WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก'),\
         [Total_Price_Balance]=ISNULL([Total_Price_Net],0)-(SELECT ISNULL(SUM(\
         ISNULL(Cin_Pay_Cash,0)+ISNULL(Cin_Pay_Credit,0)+ISNULL(Cin_Pay_Tran,0)\
         +ISNULL(Cin_Pay_Free,0)+ISNULL(Cin_Pay_web,0)),0) \
         FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
         WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก') \
         where [Cin_no]={cin_no_q}"
    ));

    // 3. VAT accumulator — same race-safe re-aggregation. The refund
    //    decreases `Total_Price_vat` accordingly because the new
    //    negative tender row participates in the same SUM.
    statements.push(format!(
        "UPDATE HT_CheckIn_H WITH (UPDLOCK, HOLDLOCK) SET \
         Total_Price_vat=(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)+ISNULL(Cin_Pay_Credit,0)\
         +ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)+ISNULL(Cin_Pay_web,0)),0) \
         FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
         WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก') \
         where Cin_no={cin_no_q}"
    ));

    // Touch the unused-on-this-path branding so future maintenance
    // catches `BRANCH_HEAD_OFFICE` etc. via grep.
    let _ = (&cust_no_q, &branch_q);

    Ok(statements)
}

/// Execute the refund recipe.
///
/// Allocates a fresh `Pay_no` via [`allocate_pay_no`] (TABLOCKX MAX+1 —
/// same idempotency contract as the `payment` recipe), captures a single
/// `Utc::now()` for the row's date column, builds the statements, and
/// dispatches them serially against the open MSSQL connection.
#[allow(clippy::too_many_arguments)]
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    cust_no: &str,
    room_no: &str,
    amount: Money,
    method: PaymentMethod,
    original_legacy_pay_no: Option<&str>,
    refund_reason: &str,
) -> WritebackResult<LegacyIds> {
    let amount_f64 = (amount.as_satang() as f64) / 100.0;
    super::helpers::validate_finite(&[("refund_amount_baht", amount_f64)])?;

    let pay_no = allocate_pay_no(conn).await?;
    let created_at = Utc::now();

    let inputs = RefundInputs {
        cin_no,
        cust_no,
        room_no,
        amount_baht: amount_f64,
        method,
        pay_no: &pay_no,
        operator: None,
        original_legacy_pay_no,
        refund_reason,
        created_at,
    };
    let statements = build_statements(&inputs)?;
    super::execute_all(conn, &statements).await?;

    Ok(LegacyIds::new()
        .with_cin_no(cin_no.to_string())
        .with_pay_no(pay_no))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_inputs() -> RefundInputs<'static> {
        RefundInputs {
            cin_no: "CH26-005236",
            cust_no: "C21624",
            room_no: "414",
            amount_baht: 500.0,
            method: PaymentMethod::Cash,
            pay_no: "R2604-0260",
            operator: Some("Admin"),
            original_legacy_pay_no: Some("R2604-0250"),
            refund_reason: "wrong room rate",
            created_at: Utc.with_ymd_and_hms(2026, 4, 25, 4, 35, 47).unwrap(),
        }
    }

    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let inputs = sample_inputs();
        let first = build_statements(&inputs).unwrap();
        let second = build_statements(&inputs).unwrap();
        assert_eq!(first, second, "build_statements must be deterministic");
    }

    #[test]
    fn emits_three_statements() {
        // INSERT HT_CheckIn_Pay + UPDATE HT_CheckIn_H (Pay/Balance) +
        // UPDATE HT_CheckIn_H (VAT) — no receipt rows.
        let s = build_statements(&sample_inputs()).unwrap();
        assert_eq!(s.len(), 3, "refund recipe must emit exactly 3 stmts; got {}", s.len());
    }

    #[test]
    fn refund_emits_negative_cash_amount_for_cash_method() {
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // For cash method: Cin_Pay_Cash=-500.00, Cin_Pay_Credit=0.00
        // The Cin_No='CH26-005236','414' prefix lands first, then the
        // two tender columns (positions 3-4 of the SELECT tuple).
        assert!(
            pay.contains("SELECT 'CH26-005236','414',-500.00,0.00,"),
            "cash refund must carry negative Cin_Pay_Cash with positive 0.00 Credit; got:\n{pay}"
        );
    }

    #[test]
    fn refund_emits_negative_credit_amount_for_credit_method() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Credit;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        assert!(
            pay.contains("SELECT 'CH26-005236','414',0.00,-500.00,"),
            "credit refund must carry negative Cin_Pay_Credit; got:\n{pay}"
        );
    }

    #[test]
    fn refund_emits_negative_transfer_amount_for_transfer_method() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Transfer;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // Tran is the 18th column — sits between Free and Branch, and
        // immediately before Branch in the value tail.
        assert!(
            pay.contains(",0.00,-500.00,'สำนักงานใหญ่',0.00 WHERE NOT EXISTS"),
            "transfer refund must carry negative Cin_Pay_Tran (before Branch); got:\n{pay}"
        );
    }

    #[test]
    fn refund_emits_negative_web_amount_for_web_method() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Web;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // Web is the 20th (final) column — after Branch.
        assert!(
            pay.contains(",0.00,0.00,'สำนักงานใหญ่',-500.00 WHERE NOT EXISTS"),
            "web refund must carry negative Cin_Pay_web (last column); got:\n{pay}"
        );
    }

    #[test]
    fn refund_emits_negative_ds_price_for_invariant() {
        // Legacy invariant (`COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay` (A)"
        // "Cin_Pay_Cash+Cin_Pay_Credit+Cin_Pay_Free+Cin_Pay_Tran+Cin_Pay_web"):
        //   Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web
        // For a -500 cash refund: -500 = -500 + 0 + 0 + 0 + 0 ✓
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // Cin_Pay_Ds_Price is the 7th SELECT value, right before 'รายการ' (PAY_DS_UNIT_ITEM).
        assert!(
            pay.contains("'ค่าห้อง',-500.00,'รายการ'"),
            "Cin_Pay_Ds_Price must be the negated refund amount; got:\n{pay}"
        );
        // Cin_Pay_Ds_PriceTotal is the 13th value (right after Cin_Pay_Ds_Num=1).
        assert!(
            pay.contains(",1,-500.00,500.00,"),
            "Cin_Pay_Ds_PriceTotal must be -500.00; Cin_Pay_Ds_PriceOne must be the magnitude 500.00; got:\n{pay}"
        );
    }

    #[test]
    fn cin_pay_note_carries_refund_prefix_with_original_pay_no_and_reason() {
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        assert!(
            pay.contains("'REFUND of Pay_no=R2604-0250: wrong room rate'"),
            "Cin_Pay_Note must carry the REFUND prefix + original Pay_no + reason; got:\n{pay}"
        );
    }

    #[test]
    fn cin_pay_note_falls_back_to_generic_refund_when_original_unknown() {
        let mut inputs = sample_inputs();
        inputs.original_legacy_pay_no = None;
        inputs.refund_reason = "";
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        assert!(
            pay.contains("'REFUND'"),
            "Cin_Pay_Note must fall back to the generic REFUND prefix when original \
             Pay_no isn't known yet; got:\n{pay}"
        );
    }

    #[test]
    fn pay_insert_guarded_by_idempotent_where_not_exists() {
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        assert!(
            pay.contains(
                "WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay WHERE Pay_no='R2604-0260')"
            ),
            "HT_CheckIn_Pay refund INSERT must be idempotent on Pay_no; got:\n{pay}"
        );
    }

    /// Track C — T5 CRIT-1: the totals UPDATE must take UPDLOCK+HOLDLOCK
    /// and re-aggregate from rows (never additive).
    #[test]
    fn totals_updates_are_re_aggregated_under_updlock_holdlock() {
        let s = build_statements(&sample_inputs()).unwrap();
        let totals = s
            .iter()
            .find(|s| s.contains("UPDATE [HT_CheckIn_H]") && s.contains("Total_Price_Pay"))
            .expect("HT_CheckIn_H totals UPDATE must be emitted");
        assert!(
            totals.contains("UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK)"),
            "header UPDATE must take UPDLOCK+HOLDLOCK; got:\n{totals}"
        );
        assert!(
            totals.contains("FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK)"),
            "SELECT aggregate must take UPDLOCK+HOLDLOCK on HT_CheckIn_Pay; got:\n{totals}"
        );
        assert!(
            !totals.contains("Total_Price_Pay=ISNULL([Total_Price_Pay],0)+"),
            "Total_Price_Pay must be absolute re-aggregate, not additive; got:\n{totals}"
        );
        assert!(
            totals.contains("ISNULL(Cin_Status,'1') <> 'ยกเลิก'"),
            "aggregate must exclude cancelled tender rows; got:\n{totals}"
        );
    }

    #[test]
    fn vat_accumulator_re_aggregates_from_rows_under_updlock() {
        let s = build_statements(&sample_inputs()).unwrap();
        let vat = s
            .iter()
            .find(|s| s.contains("Total_Price_vat="))
            .expect("VAT accumulator UPDATE must be emitted");
        assert!(
            !vat.contains("Total_Price_vat=Total_Price_vat-"),
            "VAT must be re-aggregated (absolute), not additive; got:\n{vat}"
        );
        assert!(
            vat.contains("FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK)"),
            "VAT aggregate SELECT must take UPDLOCK+HOLDLOCK; got:\n{vat}"
        );
    }

    /// Refunds do NOT emit receipt rows (no HT_Receipt_H / HT_Receipt_Ds).
    /// Reprinting refund receipts is a Track G follow-on if needed.
    #[test]
    fn refund_emits_no_receipt_rows() {
        let s = build_statements(&sample_inputs()).unwrap();
        for stmt in &s {
            assert!(
                !stmt.contains("HT_Receipt_H"),
                "refund recipe must not emit HT_Receipt_H; got:\n{stmt}"
            );
            assert!(
                !stmt.contains("HT_Receipt_Ds"),
                "refund recipe must not emit HT_Receipt_Ds; got:\n{stmt}"
            );
        }
    }

    #[test]
    fn rejects_non_finite_amount() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = f64::NAN;
        let err = build_statements(&inputs).expect_err("NaN must be rejected");
        assert!(
            err.to_string().contains("finite"),
            "error message must mention finiteness; got: {err}"
        );
    }

    #[test]
    fn rejects_negative_amount() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = -10.0;
        let err = build_statements(&inputs).expect_err("negative carrier must be rejected");
        let msg = err.to_string();
        assert!(
            msg.contains("non-negative") || msg.contains("negative"),
            "error message must mention non-negativity; got: {msg}"
        );
    }
}
