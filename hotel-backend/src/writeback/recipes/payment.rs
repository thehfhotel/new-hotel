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
    /// recipe emits the `raw/invoice-20260424-100827/writes.txt:3` (spike §3h) statement
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
    /// "Now" timestamp threaded in by `execute()` so `build_statements`
    /// stays PURE — coexistence audit T6 HIGH-1. Drives
    /// `HT_CheckIn_Pay.Cin_Pay_Date` and `HT_Receipt_H.Receipt_Date` from a
    /// single source; the prior `Utc::now()` recompute could straddle BKK
    /// midnight between the two rows. Tests pin a fixed instant.
    pub created_at: DateTime<Utc>,
    /// Wave 5c — VAT percentage to apply to this receipt. Plumbed in from
    /// `ht_settings.vat_percent` by the service layer so the hotel can
    /// toggle between 0%/7% without a code change. The prior hardcoded
    /// [`RECEIPT_VAT_PERCENT`] constant is now only the fallback when the
    /// settings row is missing (e.g. older queued intents).
    pub vat_percent: i32,
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
    let now_q = sql_quote(&format_legacy_datetime(inputs.created_at));
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

    // Tender column: cash → Cin_Pay_Cash; credit → Cin_Pay_Credit;
    // transfer → Cin_Pay_Tran (bank-transfer / PromptPay-wire).
    // Cin_Pay_web is the online / QR / app-payment column (Wave 5c —
    // routed via PaymentMethod::Web). Verified across 24 captured rows
    // in /tmp/legacy-events-full.log.
    let (cash_2dp, credit_2dp, transfer_2dp, web_2dp) = match inputs.method {
        PaymentMethod::Cash => (
            money_2dp(amount)?,
            money_2dp(0.0)?,
            money_2dp(0.0)?,
            money_2dp(0.0)?,
        ),
        PaymentMethod::Credit => (
            money_2dp(0.0)?,
            money_2dp(amount)?,
            money_2dp(0.0)?,
            money_2dp(0.0)?,
        ),
        PaymentMethod::Transfer => (
            money_2dp(0.0)?,
            money_2dp(0.0)?,
            money_2dp(amount)?,
            money_2dp(0.0)?,
        ),
        PaymentMethod::Web => (
            money_2dp(0.0)?,
            money_2dp(0.0)?,
            money_2dp(0.0)?,
            money_2dp(amount)?,
        ),
    };
    let free_2dp = money_2dp(0.0)?;

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
    // Wave 5c: VAT percent threaded in from `ht_settings.vat_percent`
    // (service-layer plumbing). At 0% the split degenerates to
    // `(amount, 0)` so receipts come out VAT-free; at 7% it matches the
    // legacy capture math `Total/1.07`.
    let (before_vat, vat) = vat_inclusive_split(amount, inputs.vat_percent);
    let before_vat_2dp = money_2dp(before_vat)?;
    let vat_2dp = money_2dp(vat)?;

    let mut statements: Vec<String> = Vec::with_capacity(8);

    // NOTE (2026-06-11 coexistence audit, P0-2): earlier revisions emitted a
    // `delete from HT_CheckIn_Product where Cin_no=…` "cart clear" here,
    // attributed to the §3h capture. The §3h capture contains NO such
    // statement (it is 4 statements: pay INSERT, totals UPDATE, receipt
    // header, receipt line) — the delete was a Phase-1 artifact. Worse, it
    // erased iHOTEL-entered minibar/product lines WITHOUT the mandatory
    // `HT_Products.Pro_Amt` stock-restore pairing (COMPAT_CHEATSHEET §6.3),
    // corrupting both the folio and product stock on every payment against a
    // folio with product charges. Removed — do not reintroduce without a
    // fresh capture showing iHOTEL doing it.

    // 1. Per-room apportionment — spike §3h `invoice/writes.txt:3`. Fires
    //    immediately before the HT_CheckIn_Pay INSERT. Only emitted when the
    //    route resolved a specific `HT_CheckIn_Ds.id`.
    // Wave 6 LOW item 4: 2dp for consistency with the HT_CheckIn_Pay /
    // HT_Receipt_H formatting throughout this recipe.
    if let Some(ds_id) = inputs.checkin_ds_id {
        statements.push(format!(
            "update [HT_CheckIn_Ds] SET  [Cin_Room_Pay_Total]={amount_2dp},[Cin_note]='' where id={ds_id}"
        ));
    }

    // 2. HT_CheckIn_Pay — payment row. 20 columns in the legacy app's
    //    canonical order (verified from 24 captures in
    //    /tmp/legacy-events-full.log; every row uses this exact column
    //    sequence). Note `[Cin_Pay_Ds_PriceTotal]` precedes
    //    `[Cin_Pay_Ds_PriceOne]` — opposite of the natural English reading.
    //    `[Cin_Pay_Ds]` carries the room number (NOT empty); the unit
    //    column emits the literal string `'รายการ'` not the integer 1.
    //
    //    Audit H3: `Cin_Pay_Ds_Price` and `Cin_Pay_Ds_PriceTotal` are the
    //    actual tender amount (not the nightly total). The legacy invariant
    //    (`COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay` (A)"
    //    "Cin_Pay_Cash+Cin_Pay_Credit+Cin_Pay_Free+Cin_Pay_Tran+Cin_Pay_web")
    //    requires
    //    `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web`. The unit
    //    price (`Cin_Pay_Ds_PriceOne`) and quantity (`Cin_Pay_Ds_Num`)
    //    remain verbatim so the printed receipt line still shows the
    //    per-night breakdown.
    //
    //    Idempotency MED: wrap the INSERT in `WHERE NOT EXISTS` on
    //    `Pay_no`. The Pay_no was allocated under TABLOCKX so a duplicate
    //    can only arise from a retry after a network drop on the COMMIT
    //    of a prior attempt. Without the guard, the subsequent
    //    `Total_Price_Pay = ISNULL(...) + amount` UPDATE would
    //    double-count the payment.
    let cash_v = cash_2dp.parse::<f64>().unwrap_or(0.0);
    let credit_v = credit_2dp.parse::<f64>().unwrap_or(0.0);
    let free_v = free_2dp.parse::<f64>().unwrap_or(0.0);
    let transfer_v = transfer_2dp.parse::<f64>().unwrap_or(0.0);
    let web_v = web_2dp.parse::<f64>().unwrap_or(0.0);
    let amount_check = cash_v + credit_v + free_v + transfer_v + web_v;
    debug_assert!(
        (amount_check - amount).abs() < 0.005,
        "split tender (cash={cash_v} credit={credit_v} free={free_v} \
         tran={transfer_v} web={web_v}) must equal amount={amount}"
    );
    statements.push(format!(
        "INSERT INTO [HT_CheckIn_Pay](  [Cin_No],[Cin_Pay_Ds],[Cin_Pay_Cash],[Cin_Pay_Credit],\
         [Cin_Pay_Date],[Cin_Pay_Ds_Name],[Cin_Pay_Ds_Price],[Cin_Pay_Ds_unit],[Pay_No],\
         [Cin_Cust_no],[Cin_Pay_Ds_ID],[Cin_Pay_Ds_Num],[Cin_Pay_Ds_PriceTotal],\
         [Cin_Pay_Ds_PriceOne],[Cin_Pay_Note],[Pay_by],[Cin_Pay_Free],[Cin_Pay_Tran],\
         [Branch],[Cin_Pay_web])\
         SELECT {cin_no_q},{room_no_q},{cash_2dp},{credit_2dp},{now_q},{pay_ds_name_q},\
         {amount_2dp},{pay_ds_unit_q},{pay_no_q},{cust_no_q},{pay_ds_id_q},{nights},\
         {amount_2dp},{price_per_night_2dp},'',{operator_q},{free_2dp},{transfer_2dp},\
         {branch_q},{web_2dp} \
         WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay WHERE Pay_no={pay_no_q})"
    ));

    // 3. HT_CheckIn_H — Total_Price_Pay + Total_Price_Balance re-aggregated
    //    from `HT_CheckIn_Pay` rows under UPDLOCK+HOLDLOCK
    //    (Track C — T5 CRIT-1, `docs/coexistence/audit-2026-05-13.md`).
    //
    //    Why not the previous additive UPDATE?
    //    The earlier `Total_Price_Pay = ISNULL(...,0) + amount` shape only
    //    held the header row's X-lock for the duration of one UPDATE
    //    statement (RC default) — released at COMMIT. iHOTEL's save-payment
    //    uses an absolute `SET Total_Price_Pay=<precomputed>`; if our
    //    writeback commits first, iHOTEL's subsequent absolute write
    //    overwrites the header with its pre-computed total (which never
    //    saw our amount) → payment silently disappears from the legacy view.
    //
    //    Race-safe pattern: SELECT-aggregate from `HT_CheckIn_Pay` rows with
    //    UPDLOCK+HOLDLOCK; the lock is held *through COMMIT* (HOLDLOCK
    //    promotes to serializable-range), blocking iHOTEL's read-modify-write
    //    until we finish. The absolute SET we then write is correct by
    //    re-aggregation, so we no longer race the iHOTEL absolute SET. The
    //    `cin_status <> 'ยกเลิก'` filter excludes cancelled tender rows
    //    per T2 CRIT-2 (`Cin_Status` column on HT_CheckIn_Pay carries
    //    `'1'` for active or `'ยกเลิก'` for cancelled — `COMPAT_CHEATSHEET.md`
    //    §"1.5 Boolean conventions" "`Cin_Pay_Status` (HT_CheckIn_Pay)").
    //
    //    Pattern matches the booking-edit race-safety recipe validated in
    //    spike §6.
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

    // 4. VAT accumulator — spike §3h `invoice/writes.txt:6`. This hotel uses
    //    no VAT (the captured value is the gross amount), but the .NET app
    //    still increments `Total_Price_vat` by the payment amount on every
    //    invoice. Emit it for parity so reports that aggregate this column
    //    match the legacy app's running total.
    //
    //    Track C — T5 CRIT-1: same race-safe pattern as Total_Price_Pay
    //    above. Re-aggregate `Total_Price_vat` from `HT_CheckIn_Pay` rows
    //    under UPDLOCK+HOLDLOCK held through COMMIT instead of additively
    //    incrementing — concurrent iHOTEL absolute-write on the column
    //    would otherwise clobber our contribution.
    statements.push(format!(
        "UPDATE HT_CheckIn_H WITH (UPDLOCK, HOLDLOCK) SET \
         Total_Price_vat=(SELECT ISNULL(SUM(ISNULL(Cin_Pay_Cash,0)+ISNULL(Cin_Pay_Credit,0)\
         +ISNULL(Cin_Pay_Tran,0)+ISNULL(Cin_Pay_Free,0)+ISNULL(Cin_Pay_web,0)),0) \
         FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK) \
         WHERE Cin_No={cin_no_q} AND ISNULL(Cin_Status,'1') <> 'ยกเลิก') \
         where Cin_no={cin_no_q}"
    ));

    // 5. HT_Receipt_H — receipt header. 20-column canonical order
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
        vat_per = inputs.vat_percent,
    ));

    // 6. HT_Receipt_Ds — receipt line for the room charge. Audit H4: the
    //    legacy capture (`invoice-20260424-100827/writes.txt:8`) emits
    //    `S_Unit=1.00, S_Price=711.00, S_Total=711.00, S_PriceDiscount=0.00`
    //    — every numeric column rendered with two decimals. The prior code
    //    emitted bare integers (1, 711, 0) which diverged from the
    //    capture and printed receipts with inconsistent decimal styling.
    let unit_one_2dp = money_2dp(1.0)?;
    let zero_2dp = money_2dp(0.0)?;
    statements.push(format!(
        "INSERT INTO [HT_Receipt_Ds]([S_Sale_id],[S_Product_no],[S_Product_name],[S_Unit],\
         [S_UnitName],[S_Price],[S_Total],[S_PriceDiscount_per],[S_PriceDiscount])\
         VALUES({receipt_id},{service_code_q},{receipt_label_q},{unit_one_2dp},{unit_name_q},\
         {amount_2dp},{amount_2dp},'',{zero_2dp})"
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
    price_per_night_baht: Option<f64>,
    nights: Option<i32>,
    vat_percent: Option<i32>,
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

    // Coexistence audit T6 HIGH-1: capture `Utc::now()` once so
    // `Cin_Pay_Date` and `Receipt_Date` share the same instant.
    let created_at = Utc::now();

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
        // Wave 5a item 2: per-night rate + nights are now plumbed from the
        // service via the intent payload. None falls back to the recipe's
        // amount/nights default (defensive only — kept so older queued
        // intents still drain).
        price_per_night_baht,
        nights,
        stay_check_in: None,
        stay_check_out: None,
        from_booking: false,
        receipt_tax: "",
        created_at,
        // Wave 5c: fall back to the legacy hardcoded constant when the
        // route didn't plumb a value through (older queued intents that
        // pre-date the ht_settings.vat_percent lookup).
        vat_percent: vat_percent.unwrap_or(RECEIPT_VAT_PERCENT),
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
            // T6 HIGH-1: fixed instant so Cin_Pay_Date and Receipt_Date are
            // deterministic for byte-parity assertions.
            created_at: Utc.with_ymd_and_hms(2026, 4, 25, 4, 35, 47).unwrap(),
            // Wave 5c: hotel's standard Thai 7% VAT — matches the captured
            // legacy rows (so byte-for-byte tests below stay green).
            vat_percent: 7,
        }
    }

    /// Coexistence audit T6 HIGH-1: `build_statements` is PURE — calling it
    /// twice with the same inputs must produce byte-identical output, with
    /// `Cin_Pay_Date` and `Receipt_Date` derived from `inputs.created_at`.
    #[test]
    fn build_statements_is_pure_with_fixed_instant() {
        let inputs = sample_inputs();
        let first = build_statements(&inputs).unwrap();
        let second = build_statements(&inputs).unwrap();
        assert_eq!(first, second, "build_statements must be deterministic");
    }

    /// Byte-for-byte test of the HT_CheckIn_Pay statement against a
    /// captured legacy row. Source: `/tmp/legacy-events-full.log`
    /// timestamped 2026-04-25T04:35:47 — `R2604-0250` payment for
    /// `CH26-005236` room `414`.
    ///
    /// The recipe wraps the legacy INSERT in
    /// `INSERT INTO … SELECT … WHERE NOT EXISTS …` for retry idempotency
    /// (audit MED — payment idempotency). The column list and value tuple
    /// remain byte-for-byte identical to the legacy capture — we just
    /// gate on `Pay_no`.
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
                    SELECT 'CH26-005236','414',801.00,0.00,";
        assert!(
            pay.starts_with(head),
            "HT_CheckIn_Pay must start with the legacy column list + tender values; got:\n{pay}"
        );
        let tail = "'ค่าห้อง',801.00,'รายการ','R2604-0250','C21624','P001',1,801.00,801.00,'',\
                    'Admin',0.00,0.00,'สำนักงานใหญ่',0.00 \
                    WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay WHERE Pay_no='R2604-0250')";
        assert!(
            pay.ends_with(tail),
            "HT_CheckIn_Pay must end with the legacy value tail + idempotency guard; got:\n{pay}"
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
    fn produces_six_statements_when_per_room_apportionment_set() {
        // 1 per-room update + 1 HT_CheckIn_Pay + 1 HT_CheckIn_H totals
        // + 1 VAT accumulator + 1 HT_Receipt_H + 1 HT_Receipt_Ds = 6
        let s = build_statements(&sample_inputs()).unwrap();
        assert_eq!(s.len(), 6);
    }

    #[test]
    fn skips_per_room_update_when_checkin_ds_id_none() {
        let mut inputs = sample_inputs();
        inputs.checkin_ds_id = None;
        let s = build_statements(&inputs).unwrap();
        assert_eq!(s.len(), 5);
        assert!(!s.iter().any(|s| s.contains("[Cin_Room_Pay_Total]=")));
    }

    /// 2026-06-11 coexistence audit P0-2: the recipe must NOT delete
    /// `HT_CheckIn_Product` rows. The §3h capture has no cart-clear, and a
    /// bare DELETE would erase iHOTEL-entered product lines without the
    /// mandatory `Pro_Amt` stock-restore pairing (cheatsheet §6.3).
    #[test]
    fn never_deletes_checkin_product_rows() {
        let s = build_statements(&sample_inputs()).unwrap();
        assert!(
            !s.iter()
                .any(|s| s.to_ascii_lowercase().contains("delete from ht_checkin_product")),
            "payment recipe must not touch HT_CheckIn_Product; got: {s:#?}"
        );
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
        // The INSERT … SELECT shape (idempotency guard) ends with the
        // Web=0.00 value followed by the WHERE NOT EXISTS clause — no
        // trailing `)` after the value list.
        assert!(pay.contains(",0.00,0.00,"));
        assert!(pay.contains(",0.00,801.00,'สำนักงานใหญ่',0.00 WHERE NOT EXISTS"));
    }

    /// Track C — T5 CRIT-1: the VAT accumulator is re-aggregated from
    /// `HT_CheckIn_Pay` rows under UPDLOCK+HOLDLOCK held through COMMIT
    /// (was additive `Total_Price_vat=Total_Price_vat+amount`, which
    /// raced iHOTEL's absolute-write).
    #[test]
    fn vat_accumulator_re_aggregates_from_rows_under_updlock() {
        let s = build_statements(&sample_inputs()).unwrap();
        let vat = s
            .iter()
            .find(|s| s.contains("UPDATE HT_CheckIn_H") && s.contains("Total_Price_vat="))
            .expect("VAT accumulator UPDATE must be emitted");
        // Must NOT use the additive shape (legacy race-prone pattern).
        assert!(
            !vat.contains("Total_Price_vat=Total_Price_vat+"),
            "VAT must be re-aggregated (absolute), not additive; got:\n{vat}"
        );
        // Must promote UPDLOCK+HOLDLOCK on HT_CheckIn_Pay (held through
        // COMMIT — blocks iHOTEL's read-modify-write on the same rows).
        assert!(
            vat.contains("FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK)"),
            "VAT aggregate SELECT must take UPDLOCK+HOLDLOCK on HT_CheckIn_Pay; got:\n{vat}"
        );
        // Must filter cancelled rows (T2 CRIT-2).
        assert!(
            vat.contains("ISNULL(Cin_Status,'1') <> 'ยกเลิก'"),
            "VAT aggregate must exclude cancelled (Cin_Status='ยกเลิก') tender rows; got:\n{vat}"
        );
        // Must filter by current Cin_No.
        assert!(vat.contains("WHERE Cin_No='CH26-005236'"));
        assert!(vat.contains("where Cin_no='CH26-005236'"));
    }

    /// Track C — T5 CRIT-1: HT_CheckIn_H totals UPDATE re-aggregates from
    /// `HT_CheckIn_Pay` rows under UPDLOCK+HOLDLOCK held through COMMIT
    /// (was additive — concurrent iHOTEL absolute SET silently overwrote
    /// our contribution).
    #[test]
    fn checkin_h_pay_total_re_aggregates_from_rows_under_updlock() {
        let s = build_statements(&sample_inputs()).unwrap();
        let upd = s
            .iter()
            .find(|s| s.contains("UPDATE [HT_CheckIn_H]"))
            .expect("HT_CheckIn_H totals UPDATE must be emitted");
        // Header must take UPDLOCK+HOLDLOCK (held through COMMIT).
        assert!(
            upd.contains("UPDATE [HT_CheckIn_H] WITH (UPDLOCK, HOLDLOCK)"),
            "HT_CheckIn_H UPDATE must take UPDLOCK+HOLDLOCK on the header; got:\n{upd}"
        );
        // Must NOT use the legacy additive shape.
        assert!(
            !upd.contains("ISNULL([Total_Price_Pay],0)+"),
            "Total_Price_Pay must be absolute re-aggregate, not additive; got:\n{upd}"
        );
        // SELECT re-aggregation must hold UPDLOCK+HOLDLOCK on HT_CheckIn_Pay.
        assert!(
            upd.contains("FROM HT_CheckIn_Pay WITH (UPDLOCK, HOLDLOCK)"),
            "Pay aggregate SELECT must take UPDLOCK+HOLDLOCK on HT_CheckIn_Pay; got:\n{upd}"
        );
        // Aggregate sums every tender column.
        for col in [
            "Cin_Pay_Cash",
            "Cin_Pay_Credit",
            "Cin_Pay_Tran",
            "Cin_Pay_Free",
            "Cin_Pay_web",
        ] {
            assert!(
                upd.contains(col),
                "aggregate must include {col}; got:\n{upd}"
            );
        }
        // Cancelled tender rows excluded (T2 CRIT-2).
        assert!(
            upd.contains("ISNULL(Cin_Status,'1') <> 'ยกเลิก'"),
            "aggregate must exclude cancelled (Cin_Status='ยกเลิก') tender rows; got:\n{upd}"
        );
        // Balance = Net - aggregate (relative to canonical aggregate).
        assert!(
            upd.contains("[Total_Price_Balance]=ISNULL([Total_Price_Net],0)-"),
            "balance must be Net - aggregate; got:\n{upd}"
        );
        // Untouched columns: Room / Net / Product.
        assert!(!upd.contains("[Total_Price_Room]="));
        assert!(!upd.contains("[Total_Price_Net]=ISNULL"));
        assert!(!upd.contains("[Total_Price_Product]="));
        // Filtered by current Cin_no.
        assert!(upd.contains("where [Cin_no]='CH26-005236'"));
    }

    /// Track C — T5 CRIT-1: locking pattern must match the booking-edit
    /// recipe validated in spike §6. UPDLOCK+HOLDLOCK on both the header
    /// UPDATE and the SELECT-from-rows guarantees the lock survives
    /// through COMMIT and blocks any concurrent iHOTEL absolute write
    /// on the same Cin_no until our transaction commits.
    #[test]
    fn aggregate_from_rows_with_updlock_blocks_concurrent_absolute_write() {
        let s = build_statements(&sample_inputs()).unwrap();
        let combined = s.join("\n");
        // Both updates (Pay/Balance and Vat) must promote UPDLOCK+HOLDLOCK.
        let updlock_holdlock_count = combined.matches("WITH (UPDLOCK, HOLDLOCK)").count();
        assert!(
            updlock_holdlock_count >= 4,
            "expected ≥4 UPDLOCK+HOLDLOCK hints (header UPDATE × 2 + SELECT aggregate × 2); \
             got {updlock_holdlock_count}:\n{combined}"
        );
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

    /// Fix for audit H3: `Cin_Pay_Ds_Price` and `Cin_Pay_Ds_PriceTotal`
    /// must equal the actual tender amount, satisfying the legacy invariant
    /// `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web`
    /// (`COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay` (A)"
    /// "Cin_Pay_Cash+Cin_Pay_Credit+Cin_Pay_Free+Cin_Pay_Tran+Cin_Pay_web").
    /// With the prior (nightly_total) wiring a
    /// partial payment broke the invariant: a 400-baht prepayment against a
    /// 711-baht nightly stay would write Ds_Price=711 but Cash+...=400.
    #[test]
    fn ds_price_equals_amount_not_nightly_total_on_partial_payment() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = 400.0; // partial — less than one night's room rate
        inputs.price_per_night_baht = Some(711.0);
        inputs.nights = Some(1);
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // The Ds_Price (7th VALUE) and Ds_PriceTotal (13th VALUE) must both
        // be the partial-payment amount (400.00), not the nightly total
        // (711.00). Cin_Pay_Ds_PriceOne (14th VALUE, unit price) stays at 711.00
        // and Cin_Pay_Ds_Num (12th VALUE, quantity) stays at 1.
        assert!(
            pay.contains("'ค่าห้อง',400.00,'รายการ'"),
            "Cin_Pay_Ds_Price must be 400.00 (the tender), not nightly total; got:\n{pay}"
        );
        // Cash column (3rd VALUE) and Ds_PriceTotal (13th VALUE) — both 400.00.
        assert!(
            pay.contains(",400.00,0.00,"),
            "Cash tender column must be 400.00; got:\n{pay}"
        );
        assert!(
            pay.contains(",1,400.00,711.00,"),
            "Cin_Pay_Ds_Num=1, Cin_Pay_Ds_PriceTotal=400.00, Cin_Pay_Ds_PriceOne=711.00; got:\n{pay}"
        );
    }

    /// Fix for audit H4: HT_Receipt_Ds emits S_Unit, S_Price, S_Total,
    /// S_PriceDiscount with `.00` decimals — matches live capture
    /// `invoice-20260424-100827/writes.txt:8`: `1.00, 711.00, 711.00, 0.00`.
    #[test]
    fn receipt_ds_emits_two_decimal_format() {
        let s = build_statements(&sample_inputs()).unwrap();
        let line = s.iter().find(|s| s.contains("[HT_Receipt_Ds]")).unwrap();
        // VALUES tail (after Sale_id, product_no, product_name, …):
        //   …,1.00,'คืน',801.00,801.00,'',0.00)
        assert!(
            line.contains(",1.00,'คืน',801.00,801.00,'',0.00)"),
            "HT_Receipt_Ds must format S_Unit/S_Price/S_Total/S_PriceDiscount with .00; got:\n{line}"
        );
        assert!(
            !line.contains(",1,'คืน'"),
            "S_Unit must be 1.00, not bare 1; got:\n{line}"
        );
    }

    /// Fix for audit H5 (also exercised at domain level): when the recipe
    /// chooses tender columns for a Transfer payment, the amount lands in
    /// Cin_Pay_Tran (column 18, immediately before Branch).
    #[test]
    fn transfer_routes_to_cin_pay_tran_column() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Transfer;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // Tail with Cash=0, Credit=0, Free=0, Tran=801.00, Branch, Web=0.
        // Idempotency guard appends `WHERE NOT EXISTS …` after the values.
        assert!(
            pay.contains(",0.00,801.00,'สำนักงานใหญ่',0.00 WHERE NOT EXISTS"),
            "Transfer amount must land in Cin_Pay_Tran (before Branch); got:\n{pay}"
        );
    }

    /// Fix for payment idempotency MED: the INSERT INTO HT_CheckIn_Pay must
    /// be guarded by `WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay WHERE
    /// Pay_no=<pay_no_q>)` so a retry after a network-drop-on-COMMIT
    /// doesn't double-write the payment (which would double Total_Price_Pay
    /// via the subsequent UPDATE).
    #[test]
    fn checkin_pay_insert_is_guarded_against_duplicate_pay_no() {
        let s = build_statements(&sample_inputs()).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        assert!(
            pay.contains(
                "WHERE NOT EXISTS (SELECT 1 FROM HT_CheckIn_Pay WHERE Pay_no='R2604-0250')"
            ),
            "HT_CheckIn_Pay INSERT must be idempotent on Pay_no; got:\n{pay}"
        );
    }

    #[test]
    fn rejects_non_finite_amount() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = f64::NAN;
        let err = build_statements(&inputs).expect_err("NaN must be rejected");
        assert!(err.to_string().contains("non-finite"));
    }

    /// Wave 5a item 2: when the route plumbs the canonical per-night rate
    /// through to the recipe, that value MUST land verbatim in
    /// `Cin_Pay_Ds_PriceOne` — the recipe's `amount/nights` fallback is
    /// defensive only.
    #[test]
    fn price_per_night_from_payload_used_when_supplied() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = 1780.0; // partial settle on a 2-night room
        inputs.price_per_night_baht = Some(890.0);
        inputs.nights = Some(2);
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // VALUE position 14 is Cin_Pay_Ds_PriceOne; quantity Cin_Pay_Ds_Num
        // (position 12) carries nights. Format: ",1780.00,890.00,'',..."
        // (amount_2dp,price_per_night_2dp,Cin_Pay_Note,...) — note the
        // num precedes Ds_PriceTotal.
        assert!(
            pay.contains(",2,1780.00,890.00,"),
            "supplied price_per_night must land in Cin_Pay_Ds_PriceOne; got:\n{pay}"
        );
    }

    /// Wave 5a item 2 regression — the defensive `amount/nights` fallback
    /// still fires when the payload omits `price_per_night_baht`, so older
    /// queued intents that pre-date the service-layer plumbing keep
    /// behaving the same.
    #[test]
    fn price_per_night_falls_back_to_amount_over_nights_when_none() {
        let mut inputs = sample_inputs();
        inputs.amount_baht = 1780.0;
        inputs.price_per_night_baht = None;
        inputs.nights = Some(2);
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // amount/nights = 890.0
        assert!(
            pay.contains(",2,1780.00,890.00,"),
            "fallback price_per_night must be amount/nights; got:\n{pay}"
        );
    }

    /// Wave 5c — QR / online payments must land in the `Cin_Pay_web`
    /// column (the rightmost tender slot, after Branch). The other four
    /// tender columns stay zero so the legacy invariant
    /// `Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web` still
    /// equals the tendered amount.
    #[test]
    fn payment_recipe_emits_to_cin_pay_web_for_qr_method() {
        let mut inputs = sample_inputs();
        inputs.method = PaymentMethod::Web;
        let s = build_statements(&inputs).unwrap();
        let pay = s.iter().find(|s| s.contains("[HT_CheckIn_Pay]")).unwrap();
        // Cash + Credit columns (positions 3-4 of the SELECT tuple) both 0.
        assert!(
            pay.contains(",801.00,'รายการ',"),
            "Cin_Pay_Ds_Price still equals the tender amount; got:\n{pay}"
        );
        // Cash=0, Credit=0 immediately after the 'CH26-005236','414' prefix.
        assert!(
            pay.contains("SELECT 'CH26-005236','414',0.00,0.00,"),
            "Cash + Credit must both be 0.00 for Web tender; got:\n{pay}"
        );
        // Branch then Cin_Pay_web — Web=801.00 lands at the tail.
        assert!(
            pay.contains(",0.00,0.00,'สำนักงานใหญ่',801.00 WHERE NOT EXISTS"),
            "Web tender amount must land in Cin_Pay_web (last column before \
             idempotency guard); got:\n{pay}"
        );
        // Free + Tran columns also 0 (the 17th + 18th values).
        // The shape after PriceOne is `,'',OPERATOR,0.00,0.00,'สำนักงานใหญ่',…`
        // — Free + Tran are the two `0.00` flanking Branch.
        let tail = ",'Admin',0.00,0.00,'สำนักงานใหญ่',801.00";
        assert!(
            pay.contains(tail),
            "Free + Tran columns must both be 0.00 when method is Web; got:\n{pay}"
        );
        // Confirm split-tender invariant via debug_assert (panics on mismatch
        // in build_statements). If the build returned, the invariant held.
    }

    /// Wave 5c — VAT split is driven by `inputs.vat_percent` not the
    /// hardcoded constant. At 0% the split degenerates to
    /// `(amount, 0)` so the receipt header carries the gross amount as
    /// BeforeVat with zero VAT; the legacy `Receipt_VatPer` column also
    /// echoes the configured percent.
    #[test]
    fn vat_split_respects_settings_vat_percent() {
        // 7% — matches the legacy capture (sanity).
        let inputs_7 = sample_inputs();
        let s_7 = build_statements(&inputs_7).unwrap();
        let h_7 = s_7.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        // 801.00 / 1.07 → BeforeVat=748.60, Vat=52.40, VatPer=7.
        assert!(
            h_7.contains(",801.00,52.40,748.60,'True',7,"),
            "at 7% VAT, Receipt_H must carry the legacy split; got:\n{h_7}"
        );

        // 0% — VAT disabled; BeforeVat == amount, Vat == 0, VatPer=0.
        let mut inputs_0 = sample_inputs();
        inputs_0.vat_percent = 0;
        let s_0 = build_statements(&inputs_0).unwrap();
        let h_0 = s_0.iter().find(|s| s.contains("[HT_Receipt_H]")).unwrap();
        assert!(
            h_0.contains(",801.00,0.00,801.00,'True',0,"),
            "at 0% VAT, Receipt_H must carry (Total, 0, Total, 'True', 0); \
             got:\n{h_0}"
        );
    }
}
