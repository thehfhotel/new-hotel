//! `RecordPosSale` recipe — Track G6 / POS module (MVP).
//!
//! Mirrors the iHOTEL POS write path captured in
//! `docs/legacy-app/COMPAT_CHEATSHEET.md` §6.3 "HT_Products.Pro_Amt -= num".
//! When the receptionist
//! posts a F&B / laundry / amenity line to a guest's folio, the legacy
//! app fires two statements inside one transaction:
//!
//! ```text
//! 1. INSERT INTO HT_CheckIn_Product (12 cols)
//!    OUTPUT INSERTED.id
//!    VALUES (...)
//!
//! 2. UPDATE HT_Products
//!       SET Pro_Amt = Pro_Amt + (-<qty>)
//!     WHERE Pro_no = '<prod_legacy_no>'
//! ```
//!
//! Our app has already done both writes canonically (the `pos_sales`
//! INSERT and the `ht_products.prod_current_stock` decrement); this
//! recipe pushes the same pair of writes into legacy MSSQL so the
//! stock invariant holds across both DBs.
//!
//! ## Why a single recipe instead of two intents
//!
//! Splitting the pair into `RecordPosSale` + `AdjustProductStock`
//! would be cleaner conceptually but allows the queue to drain only
//! the first half — leaving `HT_CheckIn_Product` with a row but the
//! `HT_Products.Pro_Amt` counter un-decremented. The legacy app
//! never observes such a state because its POS form wraps both
//! statements in one transaction; we match the legacy invariant by
//! emitting both writes from a single recipe under the same
//! `BEGIN TRAN` the worker holds (`run_in_transaction`).
//!
//! ## Why `Pro_Amt = Pro_Amt + (-qty)` (additive, not absolute SET)
//!
//! The Track F3 `AdjustProductStock` recipe documents the rationale
//! at length (`adjust_product_stock.rs:30-44`): additive arithmetic
//! survives interleaving with concurrent iHOTEL stock moves. The
//! same rule applies here.
//!
//! ## IDENTITY capture
//!
//! `HT_CheckIn_Product.id` is `int IDENTITY NOT NULL` per the schema
//! dump (`docs/legacy-spike/schema/01-baseline-schema.txt:275-286`).
//! The recipe uses `OUTPUT INSERTED.id` so the allocated id streams
//! back as part of the INSERT response — no `SCOPE_IDENTITY()` wire
//! crossing (audit H12 — same fix as the room_change recipe).
//!
//! ## Statements emitted
//!
//! Two statements, in order:
//!
//! 1. `build_insert_statement(...)` — the 12-column INSERT against
//!    `[HT_CheckIn_Product]` with `OUTPUT INSERTED.id`.
//! 2. `build_stock_decrement_statement(...)` — the additive `Pro_Amt`
//!    UPDATE keyed on `Pro_no`.
//!
//! Both are PURE / deterministic on inputs so the test suite can pin
//! the exact wire bytes.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::{LegacyIds, ResolvedPosSale};
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{format_legacy_datetime, sql_quote};

/// PURE inputs for [`build_insert_statement`]. Hydrated from
/// [`ResolvedPosSale`] before the recipe formats the SQL so the
/// statement builder is trivially unit-testable.
#[derive(Debug, Clone)]
pub struct PosSaleInputs<'a> {
    pub cin_no: &'a str,
    pub room_no: &'a str,
    pub sold_at: chrono::DateTime<chrono::Utc>,
    pub prod_legacy_no: &'a str,
    pub prod_name: &'a str,
    pub prod_unit: &'a str,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub total_baht: f64,
    pub note: &'a str,
}

/// Build the single INSERT statement for `HT_CheckIn_Product`.
/// PURE — no I/O. The `OUTPUT INSERTED.id` clause is what lets the
/// caller capture the freshly-allocated IDENTITY via
/// `execute_insert_with_output_id`.
///
/// Column order matches the legacy schema column order (see
/// `docs/legacy-spike/schema/01-baseline-schema.txt:275-286`):
/// `id, Cin_No, Cin_Room_no, Cin_Ds_date, Cin_Pro_id, Cin_Pro_name,
///  Cin_Pro_Unit, Cin_Pro_num, Cin_Pro_price, Cin_Pro_priceTotal,
///  Cin_Pro_pay, Cin_Pro_note`. We omit `id` (IDENTITY) and let the
/// server allocate it.
///
/// `Cin_Pro_pay` is stamped at 0 today — POS-to-folio means the
/// line item is unpaid until the round-bill at checkout. The legacy
/// app permits this column to be 0 / NULL for posted-to-folio lines.
/// Direct-pay POS is deferred (see CHANGELOG `### Deferred`).
pub fn build_insert_statement(inputs: &PosSaleInputs<'_>) -> String {
    let cin_no_q = sql_quote(inputs.cin_no);
    let room_q = sql_quote(inputs.room_no);
    let date_q = sql_quote(&format_legacy_datetime(inputs.sold_at));
    let prod_id_q = sql_quote(inputs.prod_legacy_no);
    let prod_name_q = sql_quote(inputs.prod_name);
    let prod_unit_q = sql_quote(inputs.prod_unit);
    let note_q = sql_quote(inputs.note);
    // 3dp for qty (matches AdjustProductStock recipe — fractional
    // units like litres land at 3dp). 2dp for money columns (matches
    // every other recipe — cancel, checkout, room_change, payment).
    let qty = format!("{:.3}", inputs.qty);
    let unit_price = format!("{:.2}", inputs.unit_price_baht);
    let total = format!("{:.2}", inputs.total_baht);
    // Cin_Pro_pay = 0.00 — see docstring. Posted-to-folio sales are
    // unpaid until round-bill.
    let pay = "0.00";
    format!(
        "INSERT INTO [HT_CheckIn_Product]\
         ([Cin_No],[Cin_Room_no],[Cin_Ds_date],[Cin_Pro_id],[Cin_Pro_name],\
         [Cin_Pro_Unit],[Cin_Pro_num],[Cin_Pro_price],[Cin_Pro_priceTotal],\
         [Cin_Pro_pay],[Cin_Pro_note]) OUTPUT INSERTED.id \
         VALUES({cin_no_q},{room_q},{date_q},{prod_id_q},{prod_name_q},\
         {prod_unit_q},{qty},{unit_price},{total},{pay},{note_q})"
    )
}

/// Build the paired `HT_Products.Pro_Amt` decrement. Mirrors the
/// `AdjustProductStock` recipe shape (additive, 3dp) so a code reader
/// scanning either file sees the same wire pattern.
///
/// `qty_sold` is positive (number of units leaving stock); the
/// statement negates it inline so the UPDATE shape stays
/// `Pro_Amt = Pro_Amt + <delta>`.
pub fn build_stock_decrement_statement(prod_legacy_no: &str, qty_sold: f64) -> String {
    let pro_no_q = sql_quote(prod_legacy_no);
    // Negate so we always emit the additive `+` form. The
    // AdjustProductStock recipe documents why absolute SET would
    // race with concurrent iHOTEL sales.
    let delta = -qty_sold;
    let delta_str = format!("{:.3}", delta);
    format!(
        "UPDATE HT_Products SET Pro_Amt = Pro_Amt + {delta_str} \
         WHERE Pro_no={pro_no_q}"
    )
}

/// Execute the POS-sale recipe.
///
/// `resolved` carries the fully-hydrated canonical row joined with
/// `ht_products`. The recipe:
///
/// 1. Validates qty / unit_price / total are finite (audit H4).
/// 2. INSERTs the `HT_CheckIn_Product` line, capturing the legacy
///    `id` via `OUTPUT INSERTED.id`.
/// 3. UPDATEs `HT_Products.Pro_Amt` additively so the legacy stock
///    invariant holds.
/// 4. Returns [`LegacyIds`] with `cin_no` + `extra.sale_id` +
///    `extra.legacy_id` so the writeback worker's `mark_done`
///    back-populates `ht_pos_sales.sale_legacy_id`.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    resolved: &ResolvedPosSale,
) -> WritebackResult<LegacyIds> {
    super::helpers::validate_finite(&[
        ("qty", resolved.qty),
        ("unit_price_baht", resolved.unit_price_baht),
        ("total_baht", resolved.total_baht),
    ])?;
    if resolved.qty <= 0.0 {
        return Err(crate::writeback::error::WritebackError::Recipe(format!(
            "RecordPosSale qty must be > 0 (got {})",
            resolved.qty
        )));
    }

    let inputs = PosSaleInputs {
        cin_no: &resolved.legacy_cin_no,
        room_no: &resolved.legacy_room_no,
        sold_at: resolved.sold_at,
        prod_legacy_no: &resolved.prod_legacy_no,
        prod_name: &resolved.prod_name,
        prod_unit: &resolved.prod_unit,
        qty: resolved.qty,
        unit_price_baht: resolved.unit_price_baht,
        total_baht: resolved.total_baht,
        note: &resolved.note,
    };
    let insert_sql = build_insert_statement(&inputs);
    let legacy_id = super::execute_insert_with_output_id(conn, &insert_sql).await?;

    // Paired stock decrement — same connection, same transaction.
    let stock_sql = build_stock_decrement_statement(&resolved.prod_legacy_no, resolved.qty);
    super::execute_all(conn, &[stock_sql]).await?;

    let mut ids = LegacyIds::new().with_cin_no(resolved.legacy_cin_no.clone());
    ids.extra
        .insert("sale_id".into(), serde_json::Value::from(resolved.sale_id));
    ids.extra.insert(
        "ht_checkin_product_id".into(),
        serde_json::Value::from(legacy_id),
    );
    ids.extra.insert(
        "prod_legacy_no".into(),
        serde_json::Value::from(resolved.prod_legacy_no.clone()),
    );
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_inputs() -> PosSaleInputs<'static> {
        PosSaleInputs {
            cin_no: "CH26-005230",
            room_no: "303",
            // 5 AM UTC = noon BKK (the wall-clock the legacy app sees).
            sold_at: chrono::Utc.with_ymd_and_hms(2026, 5, 14, 5, 0, 0).unwrap(),
            prod_legacy_no: "B-001",
            prod_name: "Coca-Cola 330ml",
            prod_unit: "ขวด",
            qty: 2.0,
            unit_price_baht: 25.0,
            total_baht: 50.0,
            note: "minibar",
        }
    }

    /// The INSERT statement matches the 11-explicit-column shape of
    /// `HT_CheckIn_Product` (id is IDENTITY, omitted; the legacy
    /// schema has 12 cols, we write 11) with `OUTPUT INSERTED.id` so
    /// the recipe can back-populate the legacy id.
    #[test]
    fn build_insert_statement_emits_eleven_column_insert_with_output() {
        let inputs = sample_inputs();
        let sql = build_insert_statement(&inputs);

        assert!(sql.starts_with("INSERT INTO [HT_CheckIn_Product]"));
        assert!(sql.contains("OUTPUT INSERTED.id"));
        // Column order matches the legacy schema verbatim. Locked
        // here so a future refactor can't accidentally swap order
        // (legacy positional rules don't apply to a column list, but
        // a parity test against capture files would diff lexically).
        assert!(sql.contains(
            "([Cin_No],[Cin_Room_no],[Cin_Ds_date],[Cin_Pro_id],[Cin_Pro_name],\
             [Cin_Pro_Unit],[Cin_Pro_num],[Cin_Pro_price],[Cin_Pro_priceTotal],\
             [Cin_Pro_pay],[Cin_Pro_note])"
        ));
        // Identifier escaping passes through.
        assert!(sql.contains("'CH26-005230'"));
        assert!(sql.contains("'303'"));
        assert!(sql.contains("'B-001'"));
        // Thai literal preserved verbatim — Track C constraint.
        assert!(sql.contains("'ขวด'"));
        // 3dp qty, 2dp money. Cin_Pro_pay is hardcoded 0.00 (folio).
        assert!(sql.contains(",2.000,25.00,50.00,0.00,'minibar'"));
        // Wall-clock matches the legacy app's `M/d/yyyy h:mm:ss tt` format.
        assert!(sql.contains("'5/14/2026 12:00:00 PM'"));
    }

    /// Empty optional fields (room_no, note) round-trip as `''` per
    /// the recipe's docstring. Legacy schema accepts NULL but our
    /// app stores the empty string sentinel (same convention as the
    /// room_change recipe).
    #[test]
    fn build_insert_statement_handles_empty_room_and_note() {
        let mut inputs = sample_inputs();
        inputs.room_no = "";
        inputs.note = "";
        let sql = build_insert_statement(&inputs);
        // The empty-string room_no lands as `''` between the cin_no
        // and the date literal.
        assert!(sql.contains("'CH26-005230','',"));
        // ...and the empty note lands as `''` immediately before the
        // closing paren.
        assert!(sql.ends_with(",0.00,'')"));
    }

    /// Apostrophe-doubling — the legacy app stamps Thai product
    /// names with embedded apostrophes ("Beer Lao 5'") and the
    /// recipe must round-trip them per `sql_quote`'s convention.
    #[test]
    fn build_insert_statement_doubles_embedded_quotes() {
        let mut inputs = sample_inputs();
        inputs.prod_name = "Beer Lao 5'";
        inputs.note = "Guest's tab";
        let sql = build_insert_statement(&inputs);
        assert!(sql.contains("'Beer Lao 5'''"));
        assert!(sql.contains("'Guest''s tab'"));
    }

    /// The stock-decrement statement matches the `AdjustProductStock`
    /// recipe's wire shape. Negation happens inline so we always
    /// emit the additive `+` form (NOT a separate `-` arm).
    #[test]
    fn build_stock_decrement_statement_emits_additive_negative_delta() {
        let sql = build_stock_decrement_statement("B-001", 2.0);
        assert_eq!(
            sql,
            "UPDATE HT_Products SET Pro_Amt = Pro_Amt + -2.000 \
             WHERE Pro_no='B-001'"
        );
    }

    /// Fractional qty round-trips at 3dp (matches `AdjustProductStock`).
    /// Locks the cross-recipe consistency rule — selling 1.5L of soft
    /// drink decrements `Pro_Amt` by `-1.500` not `-1.5`.
    #[test]
    fn build_stock_decrement_statement_preserves_fractional_qty() {
        let sql = build_stock_decrement_statement("L-001", 1.5);
        assert!(sql.contains("Pro_Amt + -1.500"), "got: {sql}");
    }

    /// SQL injection — apostrophe inside `Pro_no` must be doubled.
    /// Defensive: legacy `Pro_no` is vendor-controlled.
    #[test]
    fn build_stock_decrement_statement_quotes_prod_no_safely() {
        let sql = build_stock_decrement_statement("X'01", 1.0);
        assert!(sql.contains("WHERE Pro_no='X''01'"), "got: {sql}");
    }

    /// PURE: identical inputs ⇒ byte-identical output. Locks the
    /// no-`Utc::now()` / no-global-state invariant that lets the
    /// recipe be retried safely.
    #[test]
    fn build_statements_are_deterministic_on_identical_inputs() {
        let inputs = sample_inputs();
        let first_insert = build_insert_statement(&inputs);
        let second_insert = build_insert_statement(&inputs);
        assert_eq!(first_insert, second_insert);

        let first_update = build_stock_decrement_statement("B-001", 2.0);
        let second_update = build_stock_decrement_statement("B-001", 2.0);
        assert_eq!(first_update, second_update);
    }

    /// Audit H4 — validation guard. NaN / Infinity in any of the
    /// three money fields must surface as a recipe error before SQL
    /// formatting (`format!("{:.2}", f64::NAN)` would emit `NaN` and
    /// break the legacy parse).
    #[test]
    fn validate_finite_rejects_nan_qty() {
        let result = crate::writeback::recipes::helpers::validate_finite(&[
            ("qty", f64::NAN),
            ("unit_price_baht", 25.0),
            ("total_baht", 50.0),
        ]);
        let err = result.expect_err("NaN qty must be rejected");
        assert!(err.to_string().contains("qty"), "got: {err}");
    }

    /// Locks the wire-shape commitment for the `Cin_Pro_pay = 0.00`
    /// constant: MVP posts to folio, never direct-pay. A future
    /// direct-pay code path will need to extend `PosSaleInputs`
    /// with a pay field rather than re-purposing this constant.
    #[test]
    fn build_insert_statement_pegs_cin_pro_pay_at_zero_for_folio_charge() {
        let sql = build_insert_statement(&sample_inputs());
        // The `0.00` sits between the price-total and the note. Use
        // the joined slice to avoid matching the unrelated `50.00`
        // total earlier in the statement.
        assert!(sql.contains(",50.00,0.00,'minibar'"));
    }
}
