//! `RecordReceipt` recipe — Task #45 / POS walk-up (roomless) sale.
//!
//! Mirrors the iHOTEL standalone-receipt write path (`FrmAddSale` /
//! `FrmReceiptMain`, captured in `docs/legacy-spike/findings.md` §3h "print
//! invoice" + `docs/legacy-app/COMPAT_CHEATSHEET.md` §3.8 / §720-753). A
//! walk-up sale (customer NOT staying — no folio) is recorded as:
//!
//! ```text
//! 1. id = get_id("HT_Receipt_H","id")            -- TABLOCKX MAX+1
//!    Receipt_no = GetSIR()                        -- B{yyMM}-{4digit}
//!    INSERT INTO HT_Receipt_H (id, Receipt_no, …, Receipt_ref='')
//! 2. For each line:
//!    INSERT INTO HT_Receipt_Ds (S_Sale_id=<receipt id>, S_Product_no, …)
//! 3. For each line with a product:
//!    UPDATE HT_Products SET Pro_Amt = Pro_Amt + (-qty) WHERE Pro_no=…
//! ```
//!
//! ## Why NO `HT_CheckIn_Product`
//!
//! The legacy walk-up path reads sales from `HT_Receipt_H/Ds` joined by
//! `Receipt_ref=Cin_no`; a walk-up has no `Cin_no`, so `Receipt_ref=''`.
//! `COMPAT_CHEATSHEET.md` §3.8 is explicit: this path does **NOT** insert
//! `HT_CheckIn_Product` (that would double-count). Our app has already
//! decremented `ht_products.prod_current_stock` canonically; the paired
//! additive `Pro_Amt` UPDATE here keeps the legacy stock invariant — same
//! rationale as the folio `pos_sale` recipe.
//!
//! ## Column order
//!
//! The `HT_Receipt_H` 20-column order + the `HT_Receipt_Ds` 9-column order
//! are copied verbatim from `writeback/recipes/payment.rs` (which is
//! byte-pinned against `/tmp/legacy-events-full.log`). The values differ
//! (per-product lines instead of a single room-charge line, `Receipt_ref=''`
//! instead of the folio `Cin_no`).

use crate::writeback::allocate::{allocate_receipt_h_id, allocate_receipt_no, LegacyConn};
use crate::writeback::constants::{
    RECEIPT_STATUS_NORMAL, RECEIPT_VAT_INCLUSIVE,
};
use crate::writeback::dispatcher::{LegacyIds, ResolvedReceipt};
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{
    format_legacy_datetime, money_2dp, sql_quote, vat_inclusive_split,
};
use chrono::{DateTime, Utc};

/// PURE inputs for [`build_statements`]. Hydrated from [`ResolvedReceipt`]
/// plus the freshly-allocated legacy ids.
#[derive(Debug, Clone)]
pub struct ReceiptInputs<'a> {
    pub receipt_h_id: i32,
    pub receipt_no: &'a str,
    pub customer_no: &'a str,
    pub customer_name: &'a str,
    pub customer_address: &'a str,
    pub customer_tel: &'a str,
    pub tax_id: &'a str,
    pub total_baht: f64,
    pub discount_baht: f64,
    pub vat_percent: i32,
    pub note: &'a str,
    pub created_at: DateTime<Utc>,
    pub lines: &'a [ReceiptLineInput<'a>],
}

/// PURE input for one `HT_Receipt_Ds` line.
#[derive(Debug, Clone)]
pub struct ReceiptLineInput<'a> {
    pub prod_legacy_no: &'a str,
    pub prod_name: &'a str,
    pub unit_name: &'a str,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub total_baht: f64,
    pub discount_baht: f64,
}

/// Build the receipt header + line + stock-decrement statements. PURE — no
/// I/O. Returns `Err` on any non-finite money figure.
pub fn build_statements(inputs: &ReceiptInputs<'_>) -> WritebackResult<Vec<String>> {
    let receipt_id = inputs.receipt_h_id;
    let receipt_no_q = sql_quote(inputs.receipt_no);
    let now_q = sql_quote(&format_legacy_datetime(inputs.created_at));
    let cust_name_q = sql_quote(inputs.customer_name);
    let cust_addr_q = sql_quote(inputs.customer_address);
    let cust_tel_q = sql_quote(inputs.customer_tel);
    let cust_no_q = sql_quote(inputs.customer_no);
    let tax_q = sql_quote(inputs.tax_id);
    let note_q = sql_quote(inputs.note);
    let status_q = sql_quote(RECEIPT_STATUS_NORMAL);
    let vat_in_q = sql_quote(RECEIPT_VAT_INCLUSIVE);

    let total_2dp = money_2dp(inputs.total_baht)?;
    let discount_2dp = money_2dp(inputs.discount_baht)?;
    // VAT-inclusive split — at 0% it degenerates to (total, 0) so a
    // VAT-free hotel emits a clean receipt; at 7% it matches the
    // legacy capture math `Total/1.07` (same helper the payment recipe uses).
    let (before_vat, vat) = vat_inclusive_split(inputs.total_baht, inputs.vat_percent);
    let before_vat_2dp = money_2dp(before_vat)?;
    let vat_2dp = money_2dp(vat)?;

    let mut statements: Vec<String> = Vec::with_capacity(1 + inputs.lines.len() * 2);

    // 1. HT_Receipt_H — header. 20-column canonical order (verbatim from
    //    payment.rs). `Receipt_ref=''` (no folio — walk-up sale),
    //    `Receipt_noteUP=''` (not a booking).
    statements.push(format!(
        "INSERT INTO [HT_Receipt_H]([id],[Receipt_no],[Receipt_Date],[Receipt_Name],\
         [Receipt_Address],[Receipt_Tel],[Receipt_Fax],[Receipt_Total],[Receipt_Vat],\
         [Receipt_BeforeVat],[Receipt_VatIn],[Receipt_VatPer],[status_name],[Receipt_Discount],\
         [Receipt_ref],[Receipt_c_no],[Receipt_cin_vat_before],[Receipt_note],[Receipt_Tax],\
         [Receipt_noteUP])VALUES({receipt_id},{receipt_no_q},{now_q},{cust_name_q},{cust_addr_q},\
         {cust_tel_q},'',{total_2dp},{vat_2dp},{before_vat_2dp},{vat_in_q},\
         {vat_per},{status_q},{discount_2dp},'',{cust_no_q},{total_2dp},{note_q},\
         {tax_q},'')",
        vat_per = inputs.vat_percent,
    ));

    // 2. HT_Receipt_Ds — one line per product. Money columns at 2dp
    //    (matches the payment recipe's audit-H4 fix).
    for line in inputs.lines {
        let prod_no_q = sql_quote(line.prod_legacy_no);
        let prod_name_q = sql_quote(line.prod_name);
        let unit_name_q = sql_quote(line.unit_name);
        let qty_2dp = money_2dp(line.qty)?;
        let price_2dp = money_2dp(line.unit_price_baht)?;
        let line_total_2dp = money_2dp(line.total_baht)?;
        let line_discount_2dp = money_2dp(line.discount_baht)?;
        statements.push(format!(
            "INSERT INTO [HT_Receipt_Ds]([S_Sale_id],[S_Product_no],[S_Product_name],[S_Unit],\
             [S_UnitName],[S_Price],[S_Total],[S_PriceDiscount_per],[S_PriceDiscount])\
             VALUES({receipt_id},{prod_no_q},{prod_name_q},{qty_2dp},{unit_name_q},\
             {price_2dp},{line_total_2dp},'',{line_discount_2dp})"
        ));
    }

    // 3. Paired additive stock decrement per product line (skip ad-hoc
    //    lines with no legacy product code). Reuses the `pos_sale` recipe's
    //    wire shape so a code reader sees one additive `Pro_Amt + (-qty)`
    //    form everywhere.
    for line in inputs.lines {
        if line.prod_legacy_no.is_empty() {
            continue;
        }
        statements.push(super::pos_sale::build_stock_decrement_statement(
            line.prod_legacy_no,
            line.qty,
        ));
    }

    Ok(statements)
}

/// Execute the walk-up receipt recipe.
///
/// Allocates `HT_Receipt_H.id` + `Receipt_no` under TABLOCKX, builds the
/// header + lines + stock decrements, ships them on the worker's
/// transaction, and returns [`LegacyIds`] with `receipt_no` + the
/// `receipt_id` / `receipt_h_id` extras so `mark_done` back-populates
/// `ht_pos_receipts.receipt_legacy_id` / `receipt_legacy_no`.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    resolved: &ResolvedReceipt,
) -> WritebackResult<LegacyIds> {
    super::helpers::validate_finite(&[
        ("total_baht", resolved.total_baht),
        ("discount_baht", resolved.discount_baht),
    ])?;
    if resolved.lines.is_empty() {
        return Err(WritebackError::Recipe(
            "RecordReceipt requires at least one line".into(),
        ));
    }
    for (i, line) in resolved.lines.iter().enumerate() {
        if line.qty <= 0.0 {
            return Err(WritebackError::Recipe(format!(
                "RecordReceipt line {i} qty must be > 0 (got {})",
                line.qty
            )));
        }
    }

    let receipt_no = allocate_receipt_no(conn).await?;
    let receipt_h_id = allocate_receipt_h_id(conn).await?;

    let line_inputs: Vec<ReceiptLineInput<'_>> = resolved
        .lines
        .iter()
        .map(|l| ReceiptLineInput {
            prod_legacy_no: &l.prod_legacy_no,
            prod_name: &l.prod_name,
            unit_name: &l.unit_name,
            qty: l.qty,
            unit_price_baht: l.unit_price_baht,
            total_baht: l.total_baht,
            discount_baht: l.discount_baht,
        })
        .collect();

    let inputs = ReceiptInputs {
        receipt_h_id,
        receipt_no: &receipt_no,
        customer_no: &resolved.customer_no,
        customer_name: &resolved.customer_name,
        customer_address: &resolved.customer_address,
        customer_tel: &resolved.customer_tel,
        tax_id: &resolved.tax_id,
        total_baht: resolved.total_baht,
        discount_baht: resolved.discount_baht,
        vat_percent: resolved.vat_percent,
        note: &resolved.note,
        created_at: resolved.sold_at,
        lines: &line_inputs,
    };
    let statements = build_statements(&inputs)?;
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new().with_receipt_no(receipt_no.clone());
    ids.extra
        .insert("receipt_id".into(), serde_json::Value::from(resolved.receipt_id));
    ids.extra
        .insert("receipt_h_id".into(), serde_json::Value::from(receipt_h_id));
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_inputs() -> ReceiptInputs<'static> {
        // 5 AM UTC = noon BKK (the wall-clock the legacy app sees).
        const LINES: &[ReceiptLineInput<'static>] = &[
            ReceiptLineInput {
                prod_legacy_no: "B-001",
                prod_name: "Coca-Cola 330ml",
                unit_name: "ขวด",
                qty: 2.0,
                unit_price_baht: 25.0,
                total_baht: 50.0,
                discount_baht: 0.0,
            },
            ReceiptLineInput {
                prod_legacy_no: "S-009",
                prod_name: "Lay's",
                unit_name: "ถุง",
                qty: 1.0,
                unit_price_baht: 20.0,
                total_baht: 20.0,
                discount_baht: 0.0,
            },
        ];
        ReceiptInputs {
            receipt_h_id: 20700,
            receipt_no: "B2606-0042",
            customer_no: "C0000",
            customer_name: "Walk-up",
            customer_address: "",
            customer_tel: "",
            tax_id: "",
            total_baht: 70.0,
            discount_baht: 0.0,
            vat_percent: 0,
            note: "",
            created_at: Utc.with_ymd_and_hms(2026, 6, 27, 5, 0, 0).unwrap(),
            lines: LINES,
        }
    }

    /// PURE: identical inputs ⇒ byte-identical output (no `Utc::now()`).
    #[test]
    fn build_statements_is_deterministic() {
        let a = build_statements(&sample_inputs()).unwrap();
        let b = build_statements(&sample_inputs()).unwrap();
        assert_eq!(a, b);
    }

    /// Header + N lines + N stock decrements. 1 + 2 + 2 = 5 statements.
    #[test]
    fn emits_header_lines_and_stock_decrements() {
        let s = build_statements(&sample_inputs()).unwrap();
        assert_eq!(s.len(), 5);
        assert!(s[0].starts_with("INSERT INTO [HT_Receipt_H]"));
        assert!(s.iter().filter(|x| x.contains("[HT_Receipt_Ds]")).count() == 2);
        assert!(s.iter().filter(|x| x.contains("UPDATE HT_Products")).count() == 2);
    }

    /// Walk-up receipts carry `Receipt_ref=''` (no folio) and
    /// `Receipt_noteUP=''` (not a booking). The header's customer-no is
    /// `Receipt_c_no`.
    #[test]
    fn header_has_empty_receipt_ref_and_note_up() {
        let s = build_statements(&sample_inputs()).unwrap();
        let h = &s[0];
        // VAT 0% ⇒ Total, 0 Vat, Total BeforeVat, 'True', 0 VatPer.
        assert!(h.contains(",70.00,0.00,70.00,'True',0,"), "got:\n{h}");
        assert!(h.contains("'ปกติ'"));
        // ...,Receipt_Discount=0.00,Receipt_ref='',Receipt_c_no='C0000',...
        assert!(h.contains(",0.00,'','C0000',70.00,"), "got:\n{h}");
        // Receipt_noteUP='' is the last value.
        assert!(h.ends_with(",'')"));
    }

    /// At 7% VAT the inclusive split matches the legacy capture math.
    #[test]
    fn header_vat_split_at_seven_percent() {
        let mut inputs = sample_inputs();
        inputs.total_baht = 107.0;
        inputs.vat_percent = 7;
        let s = build_statements(&inputs).unwrap();
        let h = &s[0];
        // 107.00 / 1.07 = 100.00 before VAT, 7.00 VAT, VatPer=7.
        assert!(h.contains(",107.00,7.00,100.00,'True',7,"), "got:\n{h}");
    }

    /// A receipt line carries the product business key, name, unit, qty and
    /// money columns at 2dp (matches the HT_Receipt_Ds capture convention).
    #[test]
    fn line_emits_product_columns_two_decimals() {
        let s = build_statements(&sample_inputs()).unwrap();
        let line = s.iter().find(|x| x.contains("'B-001'")).unwrap();
        assert!(line.contains("[HT_Receipt_Ds]"));
        assert!(line.contains("'Coca-Cola 330ml'"));
        assert!(line.contains("'ขวด'"));
        // S_Sale_id, prod, name, S_Unit=2.00, unit, S_Price=25.00, S_Total=50.00
        assert!(line.contains("VALUES(20700,'B-001','Coca-Cola 330ml',2.00,'ขวด',25.00,50.00,'',0.00)"), "got:\n{line}");
    }

    /// Build a header with a custom line set (own the lines locally so the
    /// borrow outlives the `build_statements` call).
    fn inputs_with<'a>(lines: &'a [ReceiptLineInput<'a>], total: f64) -> ReceiptInputs<'a> {
        ReceiptInputs {
            receipt_h_id: 20700,
            receipt_no: "B2606-0042",
            customer_no: "C0000",
            customer_name: "Walk-up",
            customer_address: "",
            customer_tel: "",
            tax_id: "",
            total_baht: total,
            discount_baht: 0.0,
            vat_percent: 0,
            note: "",
            created_at: Utc.with_ymd_and_hms(2026, 6, 27, 5, 0, 0).unwrap(),
            lines,
        }
    }

    /// Per-line discount lands in `S_PriceDiscount`.
    #[test]
    fn line_discount_is_emitted() {
        let lines = [ReceiptLineInput {
            prod_legacy_no: "B-001",
            prod_name: "Coke",
            unit_name: "ขวด",
            qty: 1.0,
            unit_price_baht: 30.0,
            total_baht: 25.0,
            discount_baht: 5.0,
        }];
        let inputs = inputs_with(&lines, 25.0);
        let s = build_statements(&inputs).unwrap();
        let line = s.iter().find(|x| x.contains("[HT_Receipt_Ds]")).unwrap();
        assert!(line.ends_with(",25.00,'',5.00)"), "got:\n{line}");
    }

    /// Ad-hoc lines (no legacy product code) skip the stock decrement.
    #[test]
    fn adhoc_line_skips_stock_decrement() {
        let lines = [ReceiptLineInput {
            prod_legacy_no: "",
            prod_name: "Misc service",
            unit_name: "",
            qty: 1.0,
            unit_price_baht: 100.0,
            total_baht: 100.0,
            discount_baht: 0.0,
        }];
        let inputs = inputs_with(&lines, 100.0);
        let s = build_statements(&inputs).unwrap();
        // 1 header + 1 line, NO stock decrement.
        assert_eq!(s.len(), 2);
        assert!(!s.iter().any(|x| x.contains("UPDATE HT_Products")));
    }

    /// Non-finite total is rejected before SQL formatting.
    #[test]
    fn rejects_non_finite_total() {
        let mut inputs = sample_inputs();
        inputs.total_baht = f64::NAN;
        let err = build_statements(&inputs).expect_err("NaN must be rejected");
        assert!(err.to_string().contains("non-finite"));
    }

    /// Apostrophes in product names are doubled (SQL-quote).
    #[test]
    fn quotes_embedded_apostrophes() {
        let lines = [ReceiptLineInput {
            prod_legacy_no: "X'1",
            prod_name: "Beer Lao 5'",
            unit_name: "ขวด",
            qty: 1.0,
            unit_price_baht: 50.0,
            total_baht: 50.0,
            discount_baht: 0.0,
        }];
        let inputs = inputs_with(&lines, 50.0);
        let s = build_statements(&inputs).unwrap();
        assert!(s.iter().any(|x| x.contains("'Beer Lao 5'''")));
        assert!(s.iter().any(|x| x.contains("WHERE Pro_no='X''1'")));
    }
}
