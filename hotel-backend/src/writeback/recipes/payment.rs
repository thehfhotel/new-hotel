//! `RecordPayment` recipe — spike `findings.md` §3h.
//!
//! 4 statements: payment + totals refresh, then receipt header + receipt line.
//! Per spike §3h the legacy app captures the print event ~26 seconds after the
//! payment (the print dialog interaction). For our writeback we collapse them
//! into one transaction — the receptionist clicks "save & print" once.
//!
//! Reference SQL (verbatim from `invoice-20260424-100827/writes.txt`):
//!
//! ```text
//! 1. INSERT INTO [HT_CheckIn_Pay]([Cin_No='CH26-005227'],[Cin_Pay_Cash],[Cin_Pay_Credit],
//!      [Cin_Pay_Date], …, [Pay_No], [Cin_Cust_no], …)
//! 2. UPDATE [HT_CheckIn_H] SET [Total_Price_Room]=711, [Total_Price_Pay]=711,
//!      [Total_Price_Balance]=0 where [Cin_no]='CH26-005227'
//! 3. INSERT INTO [HT_Receipt_H]([id], [Receipt_no], [Receipt_Date], [Receipt_Name],
//!      [Receipt_Address], [Receipt_Tel], [Receipt_Total=711], [Receipt_Vat=0],
//!      [Receipt_VatPer=0], [status_name], [Receipt_Discount=0], …)
//! 4. INSERT INTO [HT_Receipt_Ds]([S_Sale_id=20653], [S_Product_no='SEV-001'],
//!      [S_Product_name='ค่าห้องพัก [414]'], [S_Unit=1], [S_UnitName='คืน'],
//!      [S_Price=711], [S_Total=711], …)
//! ```
//!
//! Spike §3h findings:
//! - `S_Product_no='SEV-001'` is the legacy service code for room charge.
//! - `S_Product_name='ค่าห้องพัก [414]'` (Thai: room charge) with the room
//!   number in brackets — built via [`receipt_room_label`].
//! - `S_Unit=1, S_UnitName='คืน'` (Thai: night).
//! - This hotel uses no VAT (all 0). `Receipt_Vat`, `Receipt_VatPer` = 0.
//! - Receipts are **append-only**: never deleted on check-out.
//! - `Pay_No` is month-scoped (per spike §2 — month boundary on `Cin_Pay_Date`).
//! - Tender column varies: cash → `Cin_Pay_Cash`; card → `Cin_Pay_Credit`. We
//!   look this up via [`PaymentMethod::legacy_column`].

use crate::domain::payment::PaymentMethod;
use crate::domain::shared::Money;
use crate::outbox::intent::RecordPaymentReceipt;
use crate::writeback::allocate::{
    allocate_pay_no, allocate_receipt_h_id, allocate_receipt_no, LegacyConn,
};
use crate::writeback::constants::{
    receipt_room_label, RECEIPT_SERVICE_CODE_ROOM, RECEIPT_UNIT_NIGHT,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, sql_quote};
use chrono::Utc;

/// Inputs for the payment + receipt recipe.
#[derive(Debug, Clone)]
pub struct PaymentInputs<'a> {
    pub cin_no: &'a str,
    pub cust_no: &'a str,
    pub room_no: &'a str,
    /// Customer name shown on the receipt header. Use the legacy
    /// `HT_Customers.Cust_name` value to keep parity with .NET output.
    pub customer_name: &'a str,
    pub customer_address: &'a str,
    pub customer_tel: &'a str,
    pub amount_baht: f64,
    pub method: PaymentMethod,
    pub pay_no: &'a str,
    pub receipt_no: &'a str,
    pub receipt_h_id: i32,
    /// `HT_CheckIn_Ds.id` for the room being paid for. When `Some(_)` the
    /// recipe emits the spike §3h capture line 3 statement
    /// `UPDATE HT_CheckIn_Ds SET Cin_Room_Pay_Total=<amt>, Cin_note='' WHERE id=<ds_id>`
    /// to apportion the payment to a single check-in detail row. `None`
    /// skips that UPDATE — header totals still settle.
    pub checkin_ds_id: Option<i32>,
}

/// Build the 4 payment + receipt statements. PURE — no I/O.
pub fn build_statements(inputs: &PaymentInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let _room_no_q = sql_quote(inputs.room_no); // reserved for future per-receipt usage
    let pay_no_q = sql_quote(inputs.pay_no);
    let receipt_no_q = sql_quote(inputs.receipt_no);
    let now_q = sql_quote(&format_legacy_datetime(Utc::now()));
    let amount = inputs.amount_baht;
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_addr_q = sql_quote(inputs.customer_address);
    let cust_tel_q = sql_quote(inputs.customer_tel);
    let receipt_label = receipt_room_label(inputs.room_no);
    let receipt_label_q = sql_quote(&receipt_label);
    let unit_name_q = sql_quote(RECEIPT_UNIT_NIGHT);
    let service_code_q = sql_quote(RECEIPT_SERVICE_CODE_ROOM);
    let receipt_id = inputs.receipt_h_id;

    // Tender column: amount goes into Cin_Pay_Cash or Cin_Pay_Credit, the other is 0
    let (cash, credit) = match inputs.method {
        PaymentMethod::Cash => (amount, 0.0),
        PaymentMethod::Credit | PaymentMethod::Transfer => (0.0, amount),
    };

    let mut statements: Vec<String> = Vec::with_capacity(8);

    // 1. Defensive cart clear — spike §3h `invoice/writes.txt:2`. The .NET app
    //    drops any in-progress `HT_CheckIn_Product` rows before settling. We
    //    don't write that table today, so this is almost always a no-op, but
    //    we emit it for byte-level parity with the legacy capture.
    statements.push(format!(
        "delete from HT_CheckIn_Product where Cin_no={cin_no_q}"
    ));

    // 2. Per-room apportionment — spike §3h `invoice/writes.txt:3`. Fires
    //    immediately before the HT_CheckIn_Pay INSERT. Only emitted when the
    //    route resolved a specific `HT_CheckIn_Ds.id`.
    if let Some(ds_id) = inputs.checkin_ds_id {
        statements.push(format!(
            "update [HT_CheckIn_Ds] SET  [Cin_Room_Pay_Total]={amount},[Cin_note]='' where id={ds_id}"
        ));
    }

    // 3. HT_CheckIn_Pay — payment row. Now includes [Cin_Pay_Ds] (column 4)
    //    per spike §3h `invoice/writes.txt:4`. Empty-string default mirrors
    //    the .NET capture (no extra description text).
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Pay]([Cin_No],[Cin_Pay_Ds],[Cin_Pay_Cash],[Cin_Pay_Credit],\
         [Cin_Pay_Date],[Cin_Pay_Ds_Name],[Cin_Pay_Ds_Price],[Cin_Pay_Ds_unit],[Pay_No],\
         [Cin_Cust_no],[Cin_Pay_Ds_ID],[Cin_Pay_Ds_Num],[Cin_Pay_Ds_PriceTotal])\
         VALUES({cin_no_q},'',{cash},{credit},{now_q},{receipt_label_q},{amount},1,{pay_no_q},\
         {cust_no_q},{service_code_q},1,{amount})"
    ));

    // 4. HT_CheckIn_H — accumulate Pay, recompute Balance from Net (HIGH-3).
    //    The spike capture (§3h) was a full-settle so it set Pay=amount and
    //    Balance=0 verbatim. Replaying that on a partial payment would lose
    //    prior payments and clobber Total_Price_Room / Total_Price_Net (which
    //    the booking_create / extend_stay recipes own). We additively update
    //    Pay and recompute Balance = Net - Pay, leaving Room/Net/Product
    //    alone.
    statements.push(format!(
        "UPDATE [HT_CheckIn_H] SET [Total_Price_Pay]=ISNULL([Total_Price_Pay],0)+{amount},\
         [Total_Price_Balance]=ISNULL([Total_Price_Net],0)-(ISNULL([Total_Price_Pay],0)+{amount}) \
         where [Cin_no]={cin_no_q}"
    ));

    // 5. VAT accumulator — spike §3h `invoice/writes.txt:6`. This hotel uses
    //    no VAT (the captured value is the gross amount), but the .NET app
    //    still increments `Total_Price_vat` by the payment amount on every
    //    invoice. Emit it for parity so reports that aggregate this column
    //    match the legacy app's running total.
    statements.push(format!(
        "update HT_CheckIn_H set Total_Price_vat=Total_Price_vat+{amount} where Cin_no={cin_no_q}"
    ));

    // 6. HT_Receipt_H — receipt header (id is IDENTITY but we still pass one;
    //    legacy capture provides a value, so we mirror that for parity).
    //    Includes [Receipt_c_no] per the schema (column 4, varchar(50),
    //    nullable). Storing `cin_no` here is the .NET app's convention so the
    //    receipt can be traced back to the originating check-in.
    statements.push(format!(
        "INSERT INTO [HT_Receipt_H]([id],[Receipt_no],[Receipt_Date],[Receipt_c_no],\
         [Receipt_Name],[Receipt_Address],[Receipt_Tel],[Receipt_Fax],[Receipt_Discount],\
         [Receipt_Total],[Receipt_Vat],[Receipt_BeforeVat],[Receipt_VatIn],[Receipt_VatPer],\
         [status_name],[Receipt_Ref],[Receipt_cin_vat_before],[Receipt_Tax])\
         VALUES({receipt_id},{receipt_no_q},{now_q},{cin_no_q},{cust_name_q},{cust_addr_q},\
         {cust_tel_q},'',0,{amount},0,{amount},'',0,'','',0,'')"
    ));

    // 7. HT_Receipt_Ds — receipt line for the room charge
    statements.push(format!(
        "INSERT INTO [HT_Receipt_Ds]([S_Sale_id],[S_Product_no],[S_Product_name],[S_Unit],\
         [S_UnitName],[S_Price],[S_Total],[S_PriceDiscount_per],[S_PriceDiscount])\
         VALUES({receipt_id},{service_code_q},{receipt_label_q},1,{unit_name_q},\
         {amount},{amount},'',0)"
    ));

    statements
}

/// Execute the payment + receipt recipe.
///
/// `receipt` carries the customer name/address/tel for `HT_Receipt_H`. The
/// route layer populates these by looking up the customer attached to the
/// check-in; if the lookup fails (e.g. orphaned check-in) the route still
/// enqueues the intent with empty strings — receipts get printed with blank
/// header fields rather than dropping the payment. We log a warning when all
/// three fields are empty so the fact that the receipt header is unpopulated
/// is visible in the writeback worker logs.
#[allow(clippy::too_many_arguments)]
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    cust_no: &str,
    room_no: &str,
    amount: Money,
    method: PaymentMethod,
    receipt: &RecordPaymentReceipt,
    checkin_ds_id: Option<i32>,
) -> WritebackResult<LegacyIds> {
    if receipt.customer_name.is_empty()
        && receipt.customer_address.is_empty()
        && receipt.customer_tel.is_empty()
    {
        tracing::warn!(
            cin_no,
            cust_no,
            "RecordPayment receipt header is fully empty — printed receipt \
             will have blank Receipt_Name/Address/Tel. Route enrichment from \
             ht_customers may have failed."
        );
    }

    // HIGH-4: reject NaN/Infinity before SQL formatting.
    let amount_f64 = (amount.as_satang() as f64) / 100.0;
    super::helpers::validate_finite(&[("amount_baht", amount_f64)])?;

    let pay_no = allocate_pay_no(conn).await?;
    let receipt_no = allocate_receipt_no(conn).await?;
    let receipt_h_id = allocate_receipt_h_id(conn).await?;

    let inputs = PaymentInputs {
        cin_no,
        cust_no,
        room_no,
        customer_name: receipt.customer_name.as_str(),
        customer_address: receipt.customer_address.as_str(),
        customer_tel: receipt.customer_tel.as_str(),
        amount_baht: amount_f64,
        method,
        pay_no: &pay_no,
        receipt_no: &receipt_no,
        receipt_h_id,
        checkin_ds_id,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new()
        .with_cin_no(cin_no.to_string())
        .with_pay_no(pay_no.clone())
        .with_receipt_no(receipt_no.clone());
    ids.extra
        .insert("receipt_h_id".into(), serde_json::Value::from(receipt_h_id));
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_inputs() -> PaymentInputs<'static> {
        PaymentInputs {
            cin_no: "CH26-005227",
            cust_no: "C21605",
            room_no: "414",
            customer_name: "Walk-in",
            customer_address: "",
            customer_tel: "",
            amount_baht: 711.0,
            method: PaymentMethod::Cash,
            pay_no: "P2604-000001",
            receipt_no: "RC2604-000001",
            receipt_h_id: 20653,
            checkin_ds_id: Some(25006),
        }
    }

    #[test]
    fn produces_seven_statements_when_per_room_apportionment_set() {
        // 1 cart-clear + 1 per-room update + 1 HT_CheckIn_Pay + 1 HT_CheckIn_H
        // totals + 1 VAT accumulator + 1 HT_Receipt_H + 1 HT_Receipt_Ds = 7
        let s = build_statements(&sample_inputs());
        assert_eq!(s.len(), 7);
    }

    #[test]
    fn skips_per_room_update_when_checkin_ds_id_none() {
        let mut inputs = sample_inputs();
        inputs.checkin_ds_id = None;
        let s = build_statements(&inputs);
        // Without ds_id we drop the per-room UPDATE — 6 statements total.
        assert_eq!(s.len(), 6);
        assert!(!s.iter().any(|s| s.contains("[Cin_Room_Pay_Total]=")));
    }

    #[test]
    fn includes_defensive_cart_clear_per_spike_capture_line_2() {
        let s = build_statements(&sample_inputs());
        let clear = s
            .iter()
            .find(|s| s.starts_with("delete from HT_CheckIn_Product"))
            .expect("cart clear must be emitted");
        assert!(clear.contains("Cin_no='CH26-005227'"));
    }

    #[test]
    fn per_room_apportionment_matches_capture_line_3() {
        let s = build_statements(&sample_inputs());
        let upd = s
            .iter()
            .find(|s| s.contains("[Cin_Room_Pay_Total]"))
            .expect("per-room UPDATE must be emitted when ds_id is Some");
        assert!(upd.contains("[Cin_Room_Pay_Total]=711"));
        assert!(upd.contains("[Cin_note]=''"));
        assert!(upd.contains("where id=25006"));
    }

    #[test]
    fn checkin_pay_includes_cin_pay_ds_column() {
        let s = build_statements(&sample_inputs());
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        assert!(pay.contains("[Cin_Pay_Ds]"));
    }

    #[test]
    fn includes_vat_accumulator_increment_per_capture_line_6() {
        let s = build_statements(&sample_inputs());
        let vat = s
            .iter()
            .find(|s| s.contains("Total_Price_vat=Total_Price_vat+"))
            .expect("VAT accumulator UPDATE must be emitted");
        assert!(vat.contains("Total_Price_vat=Total_Price_vat+711"));
        assert!(vat.contains("Cin_no='CH26-005227'"));
    }

    #[test]
    fn cash_payment_routes_to_cash_column() {
        let s = build_statements(&sample_inputs());
        let pay = s.iter().find(|s| s.contains("HT_CheckIn_Pay")).unwrap();
        // Cash: Cin_Pay_Cash=711, Cin_Pay_Credit=0
        assert!(pay.contains(",711,0,")); // cash, credit, then date
    }

    #[test]
    fn credit_payment_routes_to_credit_column() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Credit;
        let s = build_statements(&inputs);
        let pay = s.iter().find(|s| s.contains("HT_CheckIn_Pay")).unwrap();
        // Credit: Cin_Pay_Cash=0, Cin_Pay_Credit=711
        assert!(pay.contains(",0,711,"));
    }

    #[test]
    fn transfer_payment_uses_credit_column_per_payment_method_mapping() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Transfer;
        let s = build_statements(&inputs);
        let pay = s.iter().find(|s| s.contains("HT_CheckIn_Pay")).unwrap();
        // Transfer maps to credit column (per PaymentMethod::legacy_column)
        assert!(pay.contains(",0,711,"));
    }

    #[test]
    fn checkin_h_totals_accumulate_payment_per_high3() {
        // HIGH-3: the previous recipe set Total_Price_Pay={amount}, which lost
        // any prior partial payment and clobbered Room/Net. Now we additively
        // update Pay and recompute Balance from Net.
        let s = build_statements(&sample_inputs());
        let upd = s.iter().find(|s| s.contains("UPDATE [HT_CheckIn_H]")).unwrap();
        // additive Pay
        assert!(
            upd.contains("[Total_Price_Pay]=ISNULL([Total_Price_Pay],0)+711"),
            "Pay must accumulate: {upd}"
        );
        // Balance = Net - new Pay
        assert!(
            upd.contains("[Total_Price_Balance]=ISNULL([Total_Price_Net],0)-(ISNULL([Total_Price_Pay],0)+711)"),
            "Balance must recompute from Net: {upd}"
        );
        // Must NOT touch Room or Net (those belong to booking_create / extend_stay)
        assert!(!upd.contains("[Total_Price_Room]="), "must not touch Room: {upd}");
        assert!(!upd.contains("[Total_Price_Net]=ISNULL"), "must not write Net: {upd}");
        assert!(upd.contains("[Cin_no]='CH26-005227'"));
    }

    #[test]
    fn receipt_h_uses_no_vat_per_spike_capture() {
        let s = build_statements(&sample_inputs());
        let r = s.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        // Spike §3h: Receipt_Vat=0, Receipt_VatPer=0
        assert!(r.contains(",0,711,0,711,'',0,")); // discount, total, vat, beforeVat, vatIn, vatPer
    }

    #[test]
    fn receipt_line_uses_room_label_format() {
        let s = build_statements(&sample_inputs());
        let line = s.iter().find(|s| s.contains("[HT_Receipt_Ds]")).unwrap();
        // S_Product_name='ค่าห้องพัก [414]' per spike §3h
        assert!(line.contains("'ค่าห้องพัก [414]'"));
        // S_UnitName='คืน' per spike §3h
        assert!(line.contains("'คืน'"));
        // S_Product_no='SEV-001' per spike §3h
        assert!(line.contains("'SEV-001'"));
    }

    #[test]
    fn receipt_h_and_ds_share_sale_id() {
        let s = build_statements(&sample_inputs());
        let h = s.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        let d = s.iter().find(|s| s.contains("[HT_Receipt_Ds]")).unwrap();
        assert!(h.contains("VALUES(20653,"));
        assert!(d.contains("VALUES(20653,"));
    }
}
