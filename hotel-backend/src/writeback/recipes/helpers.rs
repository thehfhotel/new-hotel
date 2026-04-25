//! Cross-recipe SQL helpers — small statement builders shared across more
//! than one recipe so the literal SQL stays identical at every call site.
//!
//! Per the recipe spec (`recipes/mod.rs`), recipes are normally
//! self-contained. This module exists for the few statements the legacy
//! .NET app fires from multiple flows — extracting them keeps the SQL
//! literal in lock-step (drift across recipes would surface as parity
//! errors against the legacy DB only after deployment).

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
}
