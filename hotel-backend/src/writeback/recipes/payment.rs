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
    receipt_room_label, receipt_stay_note, BRANCH_HEAD_OFFICE, DEFAULT_OPERATOR,
    PAY_DS_ID_ROOM, PAY_DS_NAME_ROOM, PAY_DS_UNIT_ITEM, RECEIPT_NOTE_UP_BOOKING,
    RECEIPT_SERVICE_CODE_ROOM, RECEIPT_STATUS_NORMAL, RECEIPT_UNIT_NIGHT,
    RECEIPT_VAT_INCLUSIVE, RECEIPT_VAT_PERCENT,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{
    bangkok_date, format_legacy_datetime, money_2dp, sql_quote, vat_inclusive_split,
};
use chrono::{DateTime, Utc};

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
    /// Operator name attributed to the payment row (`HT_CheckIn_Pay.Pay_by`).
    /// Defaults to [`DEFAULT_OPERATOR`] when None.
    pub operator: Option<&'a str>,
    /// Per-night room price. Stored verbatim into
    /// `HT_CheckIn_Pay.Cin_Pay_Ds_PriceOne` (the unit price the receipt
    /// printer prints). Falls back to `amount_baht / nights` when None.
    pub price_per_night_baht: Option<f64>,
    /// Number of nights covered by the payment (>=1). Stored verbatim
    /// into `HT_CheckIn_Pay.Cin_Pay_Ds_Num`. Defaults to 1.
    pub nights: Option<i32>,
    /// Stay-period dates used to render the
    /// `HT_Receipt_H.Receipt_note` blurb (Thai: "stay from D/M/YY to
    /// D/M/YY"). When None we emit an empty note.
    pub stay_check_in: Option<DateTime<Utc>>,
    pub stay_check_out: Option<DateTime<Utc>>,
    /// Whether this payment is settling a check-in that originated from
    /// a booking. Drives `HT_Receipt_H.Receipt_noteUP` — `'Booking'`
    /// when true, empty string otherwise (matches the legacy capture).
    pub from_booking: bool,
    /// Tax/customer ID to write into `HT_Receipt_H.Receipt_Tax`.
    /// Captured values look like Thai national ID numbers, foreign
    /// passport-style IDs, or short codes. Empty string when unknown.
    pub receipt_tax: &'a str,
}

/// Build the payment + receipt statements. PURE — no I/O.
///
/// Column order, value forms, and Thai literals are byte-for-byte
/// matched against `/tmp/legacy-events-full.log` (24 captured
/// `INSERT INTO [HT_CheckIn_Pay]` rows + matching `HT_Receipt_H` rows).
/// Returns an `Err` if any money figure (amount, per-night price,
/// VAT split, etc.) is non-finite.
pub fn build_statements(
    inputs: &PaymentInputs<'_>,
) -> Result<Vec<String>, crate::writeback::error::WritebackError> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let cust_no_q = sql_quote(inputs.cust_no);
    let room_no_q = sql_quote(inputs.room_no);
    let pay_no_q = sql_quote(inputs.pay_no);
    let receipt_no_q = sql_quote(inputs.receipt_no);
    let now_q = sql_quote(&format_legacy_datetime(Utc::now()));
    let amount = inputs.amount_baht;
    let amount_2dp = money_2dp(amount)?;
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_addr_q = sql_quote(inputs.customer_address);
    let cust_tel_q = sql_quote(inputs.customer_tel);
    let receipt_label = receipt_room_label(inputs.room_no);
    let receipt_label_q = sql_quote(&receipt_label);
    let pay_ds_name_q = sql_quote(PAY_DS_NAME_ROOM);
    let pay_ds_unit_q = sql_quote(PAY_DS_UNIT_ITEM);
    let pay_ds_id_q = sql_quote(PAY_DS_ID_ROOM);
    let unit_name_q = sql_quote(RECEIPT_UNIT_NIGHT);
    let service_code_q = sql_quote(RECEIPT_SERVICE_CODE_ROOM);
    let operator_q = sql_quote(inputs.operator.unwrap_or(DEFAULT_OPERATOR));
    let branch_q = sql_quote(BRANCH_HEAD_OFFICE);
    let receipt_id = inputs.receipt_h_id;
    let nights = inputs.nights.unwrap_or(1).max(1);
    let price_per_night = inputs
        .price_per_night_baht
        .unwrap_or(amount / nights as f64);
    let price_per_night_2dp = money_2dp(price_per_night)?;
    let nightly_total_2dp = money_2dp(price_per_night * nights as f64)?;

    // Tender column: cash → Cin_Pay_Cash; credit/transfer → Cin_Pay_Credit.
    // Cin_Pay_Tran is the bank-transfer column — only filled when neither
    // cash nor credit was tendered (e.g. PromptPay / wire). Cin_Pay_web is
    // online-payment; today always 0.00. Verified across 24 captured rows
    // in /tmp/legacy-events-full.log.
    let (cash_2dp, credit_2dp, transfer_2dp) = match inputs.method {
        PaymentMethod::Cash => (money_2dp(amount)?, money_2dp(0.0)?, money_2dp(0.0)?),
        PaymentMethod::Credit => (money_2dp(0.0)?, money_2dp(amount)?, money_2dp(0.0)?),
        PaymentMethod::Transfer => (money_2dp(0.0)?, money_2dp(0.0)?, money_2dp(amount)?),
    };
    let free_2dp = money_2dp(0.0)?;
    let web_2dp = money_2dp(0.0)?;

    let receipt_tax_q = sql_quote(inputs.receipt_tax);
    let stay_note = match (inputs.stay_check_in, inputs.stay_check_out) {
        (Some(ci), Some(co)) => receipt_stay_note(bangkok_date(ci), bangkok_date(co)),
        _ => String::new(),
    };
    let stay_note_q = sql_quote(&stay_note);
    let note_up_q = sql_quote(if inputs.from_booking {
        RECEIPT_NOTE_UP_BOOKING
    } else {
        ""
    });
    let receipt_status_q = sql_quote(RECEIPT_STATUS_NORMAL);
    let vat_in_q = sql_quote(RECEIPT_VAT_INCLUSIVE);
    let (before_vat, vat) = vat_inclusive_split(amount, RECEIPT_VAT_PERCENT);
    let before_vat_2dp = money_2dp(before_vat)?;
    let vat_2dp = money_2dp(vat)?;

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

    // 3. HT_CheckIn_Pay — payment row. 20 columns in the legacy app's
    //    canonical order (verified from 24 captures in
    //    /tmp/legacy-events-full.log; every row uses this exact column
    //    sequence). Note `[Cin_Pay_Ds_PriceTotal]` precedes
    //    `[Cin_Pay_Ds_PriceOne]` — opposite of the natural English reading.
    //    `[Cin_Pay_Ds]` carries the room number (NOT empty); the unit
    //    column emits the literal string `'รายการ'` not the integer 1.
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Pay](  [Cin_No],[Cin_Pay_Ds],[Cin_Pay_Cash],[Cin_Pay_Credit],\
         [Cin_Pay_Date],[Cin_Pay_Ds_Name],[Cin_Pay_Ds_Price],[Cin_Pay_Ds_unit],[Pay_No],\
         [Cin_Cust_no],[Cin_Pay_Ds_ID],[Cin_Pay_Ds_Num],[Cin_Pay_Ds_PriceTotal],\
         [Cin_Pay_Ds_PriceOne],[Cin_Pay_Note],[Pay_by],[Cin_Pay_Free],[Cin_Pay_Tran],\
         [Branch],[Cin_Pay_web])\
         VALUES( {cin_no_q},{room_no_q},{cash_2dp},{credit_2dp},{now_q},{pay_ds_name_q},\
         {nightly_total_2dp},{pay_ds_unit_q},{pay_no_q},{cust_no_q},{pay_ds_id_q},{nights},\
         {nightly_total_2dp},{price_per_night_2dp},'',{operator_q},{free_2dp},{transfer_2dp},\
         {branch_q},{web_2dp})"
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

    // 6. HT_Receipt_H — receipt header. 20-column canonical order
    //    matching all captured rows in /tmp/legacy-events-full.log
    //    (e.g. Receipt_H 20663). VAT-inclusive math via
    //    [`vat_inclusive_split`]. `[Receipt_ref]` (lowercase r) carries
    //    the originating `Cin_No`; `[Receipt_c_no]` carries the
    //    customer number. `Receipt_noteUP='Booking'` flags receipts
    //    derived from a booking, empty otherwise. `[Receipt_Tax]`
    //    holds an external tax/customer ID supplied by the caller.
    statements.push(format!(
        "INSERT INTO [HT_Receipt_H]([id],[Receipt_no],[Receipt_Date],[Receipt_Name],\
         [Receipt_Address],[Receipt_Tel],[Receipt_Fax],[Receipt_Total],[Receipt_Vat],\
         [Receipt_BeforeVat],[Receipt_VatIn],[Receipt_VatPer],[status_name],[Receipt_Discount],\
         [Receipt_ref],[Receipt_c_no],[Receipt_cin_vat_before],[Receipt_note],[Receipt_Tax],\
         [Receipt_noteUP])VALUES({receipt_id},{receipt_no_q},{now_q},{cust_name_q},{cust_addr_q},\
         {cust_tel_q},'',{amount_2dp},{vat_2dp},{before_vat_2dp},{vat_in_q},\
         {vat_per},{receipt_status_q},0,{cin_no_q},{cust_no_q},{amount_2dp},{stay_note_q},\
         {receipt_tax_q},{note_up_q})",
        vat_per = RECEIPT_VAT_PERCENT,
    ));

    // 7. HT_Receipt_Ds — receipt line for the room charge
    statements.push(format!(
        "INSERT INTO [HT_Receipt_Ds]([S_Sale_id],[S_Product_no],[S_Product_name],[S_Unit],\
         [S_UnitName],[S_Price],[S_Total],[S_PriceDiscount_per],[S_PriceDiscount])\
         VALUES({receipt_id},{service_code_q},{receipt_label_q},1,{unit_name_q},\
         {amount},{amount},'',0)"
    ));

    Ok(statements)
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
        // Service-layer plumbing for these fields will follow in a
        // separate task (the route currently doesn't carry them);
        // defaults match the most-common legacy capture shape.
        operator: None,
        price_per_night_baht: None,
        nights: None,
        stay_check_in: None,
        stay_check_out: None,
        from_booking: false,
        receipt_tax: "",
    };
    let statements = build_statements(&inputs)?;
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
    use chrono::TimeZone;

    fn sample_inputs() -> PaymentInputs<'static> {
        PaymentInputs {
            cin_no: "CH26-005236",
            cust_no: "C21624",
            room_no: "414",
            customer_name: "Mr.Alberto Calvo Alvarez ",
            customer_address: "Espanola",
            customer_tel: "",
            amount_baht: 801.0,
            method: PaymentMethod::Cash,
            pay_no: "R2604-0250",
            receipt_no: "B2604-0265",
            receipt_h_id: 20663,
            checkin_ds_id: Some(25014),
            operator: Some("Admin"),
            price_per_night_baht: Some(801.0),
            nights: Some(1),
            stay_check_in: Some(Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap()),
            stay_check_out: Some(Utc.with_ymd_and_hms(2026, 4, 26, 4, 59, 59).unwrap()),
            from_booking: true,
            receipt_tax: "XDD619524",
        }
    }

    /// Byte-for-byte test of the HT_CheckIn_Pay statement against a
    /// captured legacy row. Source: `/tmp/legacy-events-full.log`
    /// timestamped 2026-04-25T04:35:47 — `R2604-0250` payment for
    /// `CH26-005236` room `414`.
    #[test]
    fn checkin_pay_matches_legacy_capture_byte_for_byte() {
        // The captured row uses a different timestamp than Utc::now()
        // would produce, so we substring-check around the date column.
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s
            .iter()
            .find(|s| s.contains("[HT_CheckIn_Pay]"))
            .expect("HT_CheckIn_Pay INSERT must be emitted");
        let head = "INSERT INTO [HT_CheckIn_Pay](  [Cin_No],[Cin_Pay_Ds],[Cin_Pay_Cash],\
                    [Cin_Pay_Credit],[Cin_Pay_Date],[Cin_Pay_Ds_Name],[Cin_Pay_Ds_Price],\
                    [Cin_Pay_Ds_unit],[Pay_No],[Cin_Cust_no],[Cin_Pay_Ds_ID],[Cin_Pay_Ds_Num],\
                    [Cin_Pay_Ds_PriceTotal],[Cin_Pay_Ds_PriceOne],[Cin_Pay_Note],[Pay_by],\
                    [Cin_Pay_Free],[Cin_Pay_Tran],[Branch],[Cin_Pay_web])\
                    VALUES( 'CH26-005236','414',801.00,0.00,";
        assert!(
            pay.starts_with(head),
            "HT_CheckIn_Pay must start with the legacy column list + tender values; got:\n{pay}"
        );
        let tail = "'ค่าห้อง',801.00,'รายการ','R2604-0250','C21624','P001',1,801.00,801.00,'',\
                    'Admin',0.00,0.00,'สำนักงานใหญ่',0.00)";
        assert!(
            pay.ends_with(tail),
            "HT_CheckIn_Pay must end with the legacy value tail; got:\n{pay}"
        );
    }

    /// Byte-for-byte test of the HT_Receipt_H statement against a
    /// captured legacy row. Source: `/tmp/legacy-events-full.log`
    /// `Receipt_H` id=20663, `B2604-0265` for `CH26-005236`.
    #[test]
    fn receipt_h_matches_legacy_capture_byte_for_byte() {
        let s = build_statements(&sample_inputs()).unwrap();
        let h = s
            .iter()
            .find(|s| s.contains("[HT_Receipt_H]"))
            .expect("HT_Receipt_H INSERT must be emitted");
        let head = "INSERT INTO [HT_Receipt_H]([id],[Receipt_no],[Receipt_Date],[Receipt_Name],\
                    [Receipt_Address],[Receipt_Tel],[Receipt_Fax],[Receipt_Total],[Receipt_Vat],\
                    [Receipt_BeforeVat],[Receipt_VatIn],[Receipt_VatPer],[status_name],\
                    [Receipt_Discount],[Receipt_ref],[Receipt_c_no],[Receipt_cin_vat_before],\
                    [Receipt_note],[Receipt_Tax],[Receipt_noteUP])\
                    VALUES(20663,'B2604-0265',";
        assert!(
            h.starts_with(head),
            "Receipt_H must start with legacy column list; got:\n{h}"
        );
        let tail = "'Mr.Alberto Calvo Alvarez ','Espanola','','',801.00,52.40,748.60,'True',7,\
                    'ปกติ',0,'CH26-005236','C21624',801.00,\
                    'เข้าพัก วันที่ 25/04/26 ถึง 26/04/26','XDD619524','Booking')";
        assert!(
            h.ends_with(tail),
            "Receipt_H must end with the legacy value tail (with VAT split + Booking note);\
             got:\n{h}"
        );
    }

    #[test]
    fn produces_seven_statements_when_per_room_apportionment_set() {
        // 1 cart-clear + 1 per-room update + 1 HT_CheckIn_Pay + 1 HT_CheckIn_H
        // totals + 1 VAT accumulator + 1 HT_Receipt_H + 1 HT_Receipt_Ds = 7
        let s = build_statements(&sample_inputs()).unwrap();
        assert_eq!(s.len(), 7);
    }

    #[test]
    fn skips_per_room_update_when_checkin_ds_id_none() {
        let mut inputs = sample_inputs();
        inputs.checkin_ds_id = None;
        let s = build_statements(&inputs).unwrap();
        assert_eq!(s.len(), 6);
        assert!(!s.iter().any(|s| s.contains("[Cin_Room_Pay_Total]=")));
    }

    #[test]
    fn includes_defensive_cart_clear_per_spike_capture_line_2() {
        let s = build_statements(&sample_inputs()).unwrap();
        let clear = s
            .iter()
            .find(|s| s.starts_with("delete from HT_CheckIn_Product"))
            .expect("cart clear must be emitted");
        assert!(clear.contains("Cin_no='CH26-005236'"));
    }

    #[test]
    fn per_room_apportionment_matches_capture_line_3() {
        let s = build_statements(&sample_inputs()).unwrap();
        let upd = s
            .iter()
            .find(|s| s.contains("[Cin_Room_Pay_Total]"))
            .expect("per-room UPDATE must be emitted when ds_id is Some");
        assert!(upd.contains("[Cin_Room_Pay_Total]=801"));
        assert!(upd.contains("[Cin_note]=''"));
        assert!(upd.contains("where id=25014"));
    }

    #[test]
    fn cash_payment_routes_to_cash_column_with_two_decimals() {
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s.iter().find(|s| s.contains("HT_CheckIn_Pay")).unwrap();
        // Cash: Cin_Pay_Cash=801.00, Cin_Pay_Credit=0.00, Cin_Pay_Tran=0.00
        assert!(pay.contains(",801.00,0.00,"));
    }

    #[test]
    fn credit_payment_routes_to_credit_column_with_two_decimals() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Credit;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("HT_CheckIn_Pay")).unwrap();
        assert!(pay.contains(",0.00,801.00,"));
    }

    #[test]
    fn transfer_payment_uses_tran_column_per_capture() {
        // Capture row 1 (CH26-005216 R2604-0249) shows
        //   Cash=0.00, Credit=0.00, ..., Cin_Pay_Tran=3560.00
        // when the tender was bank transfer.
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Transfer;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("HT_CheckIn_Pay")).unwrap();
        // Cash, Credit, then later Tran are all f64-rendered with 2dp.
        assert!(pay.contains(",0.00,0.00,"));
        assert!(pay.contains(",0.00,801.00,'สำนักงานใหญ่',0.00)"));
    }

    #[test]
    fn includes_vat_accumulator_increment_per_capture_line_6() {
        let s = build_statements(&sample_inputs()).unwrap();
        let vat = s
            .iter()
            .find(|s| s.contains("Total_Price_vat=Total_Price_vat+"))
            .expect("VAT accumulator UPDATE must be emitted");
        assert!(vat.contains("Total_Price_vat=Total_Price_vat+801"));
        assert!(vat.contains("Cin_no='CH26-005236'"));
    }

    #[test]
    fn checkin_h_totals_accumulate_payment_per_high3() {
        let s = build_statements(&sample_inputs()).unwrap();
        let upd = s.iter().find(|s| s.contains("UPDATE [HT_CheckIn_H]")).unwrap();
        assert!(upd.contains("[Total_Price_Pay]=ISNULL([Total_Price_Pay],0)+801"));
        assert!(upd.contains(
            "[Total_Price_Balance]=ISNULL([Total_Price_Net],0)-\
             (ISNULL([Total_Price_Pay],0)+801)"
        ));
        assert!(!upd.contains("[Total_Price_Room]="));
        assert!(!upd.contains("[Total_Price_Net]=ISNULL"));
        assert!(upd.contains("[Cin_no]='CH26-005236'"));
    }

    #[test]
    fn receipt_line_uses_room_label_format_and_unit_night() {
        let s = build_statements(&sample_inputs()).unwrap();
        let line = s.iter().find(|s| s.contains("[HT_Receipt_Ds]")).unwrap();
        // S_Product_name='ค่าห้องพัก [414]' (receipt-line form, distinct
        // from the payment row's 'ค่าห้อง'). S_UnitName='คืน' (night).
        assert!(line.contains("'ค่าห้องพัก [414]'"));
        assert!(line.contains("'คืน'"));
        assert!(line.contains("'SEV-001'"));
    }

    #[test]
    fn receipt_h_and_ds_share_sale_id() {
        let s = build_statements(&sample_inputs()).unwrap();
        let h = s.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        let d = s.iter().find(|s| s.contains("[HT_Receipt_Ds]")).unwrap();
        assert!(h.contains("VALUES(20663,"));
        assert!(d.contains("VALUES(20663,"));
    }

    #[test]
    fn from_booking_false_emits_empty_note_up() {
        // Walk-in receipts emit `Receipt_noteUP=''` rather than 'Booking'.
        let mut inputs = sample_inputs();
        inputs.from_booking = false;
        let s = build_statements(&inputs).unwrap();
        let h = s.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        assert!(h.ends_with(",'')"));
        assert!(!h.contains("'Booking'"));
    }

    #[test]
    fn missing_stay_dates_emit_empty_receipt_note() {
        let mut inputs = sample_inputs();
        inputs.stay_check_in = None;
        inputs.stay_check_out = None;
        let s = build_statements(&inputs).unwrap();
        let h = s.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        // Receipt_note is the 18th value — sandwiched between
        // [Receipt_cin_vat_before] and [Receipt_Tax]. Both flank with
        // commas; an empty note is `''` between them.
        assert!(h.contains("801.00,'','XDD619524'"));
    }

    #[test]
    fn rejects_non_finite_amount() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = f64::NAN;
        let err = build_statements(&inputs).expect_err("NaN must be rejected");
        assert!(err.to_string().contains("non-finite"));
    }
}
