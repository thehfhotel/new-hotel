//! Cross-recipe SQL helpers — small statement builders shared across more
//! than one recipe so the literal SQL stays identical at every call site.
//!
//! Per the recipe spec (`recipes/mod.rs`), recipes are normally
//! self-contained. This module exists for the few statements the legacy
//! .NET app fires from multiple flows — extracting them keeps the SQL
//! literal in lock-step (drift across recipes would surface as parity
//! errors against the legacy DB only after deployment).

use crate::outbox::intent::CustomerResave;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::sql_quote;

/// Reject any non-finite f64 (NaN/Infinity) before it gets interpolated into
/// SQL. `format!("{}", f64::NAN)` produces the literal string `"NaN"`, which
/// would emit invalid SQL like `Total_Price=NaN` and fail the entire
/// transaction (audit HIGH-4). Callers should validate every monetary /
/// price / count value at the entry of `execute()` so the failure surfaces
/// before any allocate / INSERT runs.
///
/// Pass `(label, value)` pairs so the error message identifies the offending
/// field.
pub fn validate_finite(values: &[(&str, f64)]) -> WritebackResult<()> {
    for (label, value) in values {
        if !value.is_finite() {
            return Err(WritebackError::Recipe(format!(
                "non-finite {label} cannot be written to legacy DB: {value}"
            )));
        }
    }
    Ok(())
}

/// `UPDATE HT_Cupon SET cupon_print=1 WHERE cupon_cin_no=<cin_no>` — fired
/// after every walk-in (spike `walkin/writes.txt:9`) and after the linked-
/// to-booking check-in (spike `booking-checkin/writes.txt:39`). Marks the
/// pre-allocated coupon row as "printed" in the legacy app's loyalty table.
///
/// Both `walkin` and `checkin_to_booking` emit this as the final statement
/// in their recipe — extracting it here guarantees byte-identical SQL.
pub fn mark_cupon_printed(cin_no: &str) -> String {
    format!(
        "update HT_Cupon set cupon_print=1 where cupon_cin_no={cin_no_q}",
        cin_no_q = sql_quote(cin_no),
    )
}

/// Pick a Thai-or-English personal-prefix string for the
/// `HT_CheckIn_Other_People.Cin_name` field based on the guest's country.
///
/// Heuristic: country starting with "TH" (case-insensitive) → `'นาย'`
/// (Thai "Mr."), otherwise `'Mr.'`. The legacy capture shows mixed forms
/// (`'นาย'`, `'น.ส.'`, `'นาง'`, `'Mr.'`, `'Mrs.'`, custom IDs like `'925'`);
/// plumbing the actual `Cust_perfix` through the payload is a separate task
/// — this heuristic is the minimum that preserves the Thai-vs-foreign
/// distinction. Empty country falls back to `'Mr.'` (the safer non-Thai
/// default).
///
/// Wave 6 de-duplication target — was previously defined identically in
/// `walkin.rs` and `checkin_to_booking.rs`.
pub fn guest_prefix_for_country(country: &str) -> &'static str {
    let trimmed = country.trim();
    if !trimmed.is_empty()
        && (trimmed.eq_ignore_ascii_case("TH") || trimmed.to_ascii_uppercase().starts_with("TH"))
    {
        "นาย"
    } else {
        "Mr."
    }
}

/// Build the `UPDATE [HT_Customers] SET ... where Cust_no=…` statement that
/// re-saves the customer record exactly like the legacy .NET app does.
///
/// 31 SET fields in the canonical legacy order — verified from
/// `/tmp/legacy-events-full.log` capture for `C21624` (line 3988):
/// name, name2, Type, Type_main, Email, Add_*, Work_*, Work_tax,
/// perfix, sex, IDcard, Contry. `[Cust_Type_main]` is lowercase m
/// (distinct from the INSERT path's `[Cust_Type_Main]`); the WHERE
/// clause uses lowercase `where`. Empty strings are written for fields
/// the payload doesn't supply (legacy `''`-over-NULL convention,
/// spike §3k).
///
/// Fired from two flows — extracted here per this module's charter so the
/// SQL literal stays byte-identical at both call sites:
/// * `booking_modify` (spike §3c lines 5/16/28 — the .NET app re-saves the
///   customer on every booking modify);
/// * `update_customer` (coexistence audit 2026-06-11 P2 — standalone
///   customer-edit writeback).
pub fn build_customer_resave_update(r: &CustomerResave) -> String {
    let cust_no_q = sql_quote(&r.legacy_cust_no);
    format!(
        "UPDATE [HT_Customers] SET  [Cust_name]={name},[Cust_name2]={name2},\
         [Cust_Type]={ctype},[Cust_Type_main]={ctype_main},[Cust_Email]={email},\
         [Cust_Add_no]={add_no},[Cust_Add_moo]={add_moo},[Cust_Add_soi]={add_soi},\
         [Cust_Add_road]={add_road},[Cust_Add_tambon]={add_tambon},\
         [Cust_Add_ampore]={add_ampore},[Cust_Add_province]={add_province},\
         [Cust_Add_code]={add_code},[Cust_Add_tel]={add_tel},[Cust_Add_fax]={add_fax},\
         [Cust_Work_Name]={work_name},[Cust_Work_no]={work_no},[Cust_Work_moo]={work_moo},\
         [Cust_Work_soi]={work_soi},[Cust_Work_road]={work_road},\
         [Cust_Work_tambon]={work_tambon},[Cust_Work_ampore]={work_ampore},\
         [Cust_Work_province]={work_province},[Cust_Work_code]={work_code},\
         [Cust_Work_tel]={work_tel},[Cust_Work_fax]={work_fax},[Cust_Work_tax]={work_tax},\
         [Cust_perfix]={perfix},[Cust_sex]={sex},[Cust_IDcard]={idcard},\
         [Cust_Contry]={contry} where Cust_no={cust_no_q}",
        name = sql_quote(&r.cust_name),
        name2 = sql_quote(&r.cust_name2),
        ctype = sql_quote(&r.cust_type),
        ctype_main = sql_quote(&r.cust_type_main),
        email = sql_quote(&r.cust_email),
        add_no = sql_quote(&r.cust_add_no),
        add_moo = sql_quote(&r.cust_add_moo),
        add_soi = sql_quote(&r.cust_add_soi),
        add_road = sql_quote(&r.cust_add_road),
        add_tambon = sql_quote(&r.cust_add_tambon),
        add_ampore = sql_quote(&r.cust_add_ampore),
        add_province = sql_quote(&r.cust_add_province),
        add_code = sql_quote(&r.cust_add_code),
        add_tel = sql_quote(&r.cust_add_tel),
        add_fax = sql_quote(&r.cust_add_fax),
        work_name = sql_quote(&r.cust_work_name),
        work_no = sql_quote(&r.cust_work_no),
        work_moo = sql_quote(&r.cust_work_moo),
        work_soi = sql_quote(&r.cust_work_soi),
        work_road = sql_quote(&r.cust_work_road),
        work_tambon = sql_quote(&r.cust_work_tambon),
        work_ampore = sql_quote(&r.cust_work_ampore),
        work_province = sql_quote(&r.cust_work_province),
        work_code = sql_quote(&r.cust_work_code),
        work_tel = sql_quote(&r.cust_work_tel),
        work_fax = sql_quote(&r.cust_work_fax),
        work_tax = sql_quote(&r.cust_work_tax),
        perfix = sql_quote(&r.cust_perfix),
        sex = sql_quote(&r.cust_sex),
        idcard = sql_quote(&r.cust_idcard),
        contry = sql_quote(&r.cust_contry),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Matches the spike capture lines verbatim:
    /// - `walkin-20260424-095304/writes.txt:9`
    /// - `walkin3-20260424-100000/writes.txt:9`
    /// - `booking-checkin-20260424-101838/writes.txt:39`
    #[test]
    fn mark_cupon_printed_matches_spike_capture_format() {
        let stmt = mark_cupon_printed("CH26-005228");
        assert_eq!(
            stmt,
            "update HT_Cupon set cupon_print=1 where cupon_cin_no='CH26-005228'"
        );
    }

    #[test]
    fn embedded_quote_in_cin_no_is_escaped() {
        let stmt = mark_cupon_printed("CH'26-005228");
        assert!(stmt.contains("'CH''26-005228'"));
    }

    #[test]
    fn validate_finite_passes_normal_values() {
        assert!(validate_finite(&[("amount", 890.0), ("nights", 2.0)]).is_ok());
        assert!(validate_finite(&[("zero", 0.0)]).is_ok());
        assert!(validate_finite(&[("negative", -100.5)]).is_ok());
    }

    #[test]
    fn validate_finite_rejects_nan() {
        let err = validate_finite(&[("amount", f64::NAN)]).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("amount"), "msg: {msg}");
        assert!(msg.contains("NaN"), "msg: {msg}");
    }

    #[test]
    fn validate_finite_rejects_infinity() {
        assert!(validate_finite(&[("price", f64::INFINITY)]).is_err());
        assert!(validate_finite(&[("price", f64::NEG_INFINITY)]).is_err());
    }

    #[test]
    fn guest_prefix_for_country_picks_thai_prefix_for_th_countries() {
        assert_eq!(guest_prefix_for_country("TH"), "นาย");
        assert_eq!(guest_prefix_for_country("Th"), "นาย");
        assert_eq!(guest_prefix_for_country("th"), "นาย");
        assert_eq!(guest_prefix_for_country("THA"), "นาย");
        assert_eq!(guest_prefix_for_country("THAILAND"), "นาย");
    }

    #[test]
    fn guest_prefix_for_country_falls_back_to_mr_for_others() {
        assert_eq!(guest_prefix_for_country(""), "Mr.");
        assert_eq!(guest_prefix_for_country("US"), "Mr.");
        assert_eq!(guest_prefix_for_country("UK"), "Mr.");
        assert_eq!(guest_prefix_for_country("JP"), "Mr.");
        // Whitespace-only treated as empty.
        assert_eq!(guest_prefix_for_country("   "), "Mr.");
    }
}
