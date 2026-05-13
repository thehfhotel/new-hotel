//! Settings repository — PostgreSQL data access for `ht_settings`.
//!
//! `ht_settings` is the canonical key-value store for hotel-wide configuration
//! (VAT percent, hotel name, default check-in/out times, ID prefixes, …). The
//! schema is intentionally string-typed (`setting_value TEXT`) so callers parse
//! their own values. This module surfaces the lookups the application needs
//! today; add new helpers as features land rather than exposing a generic
//! `get(key) -> Option<String>` that bypasses the type-aware parsing tier.
//!
//! Wave 5c (`docs/coexistence/audit-2026-05-13.md`) introduced the
//! `vat_percent` key so the legacy receipt header (`HT_Receipt_H.Receipt_VatPer`)
//! reads from a configurable source instead of the hardcoded
//! `RECEIPT_VAT_PERCENT` constant. Defaults to 7% (Thai standard VAT) when the
//! row is missing or unparseable.

use sqlx::PgPool;

/// `ht_settings.setting_key` for the receipt VAT percentage.
pub const VAT_PERCENT_KEY: &str = "vat_percent";

/// Default VAT percent applied when `ht_settings.vat_percent` is missing or
/// unparseable. Matches `writeback::constants::RECEIPT_VAT_PERCENT` so this
/// module can be used independently without importing the writeback layer.
pub const DEFAULT_VAT_PERCENT: i32 = 7;

/// Read the configured VAT percent from `ht_settings`.
///
/// Returns [`DEFAULT_VAT_PERCENT`] when:
/// - The row is missing.
/// - `setting_value` is NULL.
/// - `setting_value` does not parse as an integer.
/// - The PG query errors (logged at WARN — payment flow must not block on a
///   settings lookup hiccup).
///
/// The function takes a `PgPool` directly because the route layer calls it
/// outside any transaction (the writeback intent is enqueued after this
/// resolves). Pass the application's shared `new_pool` handle.
pub async fn get_vat_percent(pool: &PgPool) -> i32 {
    match sqlx::query_scalar::<_, Option<String>>(
        "SELECT setting_value FROM ht_settings WHERE setting_key = $1",
    )
    .bind(VAT_PERCENT_KEY)
    .fetch_optional(pool)
    .await
    {
        Ok(Some(Some(raw))) => parse_vat_percent(&raw).unwrap_or(DEFAULT_VAT_PERCENT),
        Ok(Some(None)) | Ok(None) => DEFAULT_VAT_PERCENT,
        Err(err) => {
            tracing::warn!(
                error = %err,
                key = VAT_PERCENT_KEY,
                "ht_settings lookup failed; falling back to DEFAULT_VAT_PERCENT"
            );
            DEFAULT_VAT_PERCENT
        }
    }
}

/// Parse the `setting_value` string into an integer VAT percent.
///
/// Accepts either `"7"` or `"7.0"` shapes — the migration seeds `'7.0'` for
/// human readability but legacy receipts emit a bare integer (`Receipt_VatPer=7`)
/// so we floor to i32. Returns `None` on garbage input — the caller falls back
/// to the default.
fn parse_vat_percent(raw: &str) -> Option<i32> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Try a direct integer parse first (cheap path for "7", "0", "10").
    if let Ok(n) = trimmed.parse::<i32>() {
        return Some(n);
    }
    // Fall back to float-then-truncate for "7.0", "0.0".
    trimmed.parse::<f64>().ok().and_then(|f| {
        if f.is_finite() && (0.0..=100.0).contains(&f) {
            Some(f as i32)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integer_string() {
        assert_eq!(parse_vat_percent("7"), Some(7));
        assert_eq!(parse_vat_percent("0"), Some(0));
        assert_eq!(parse_vat_percent("10"), Some(10));
    }

    #[test]
    fn parses_decimal_string_via_truncation() {
        // The migration seeds "7.0" for readability; legacy receipts only
        // emit integer percents so we floor.
        assert_eq!(parse_vat_percent("7.0"), Some(7));
        assert_eq!(parse_vat_percent("0.0"), Some(0));
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!(parse_vat_percent(""), None);
        assert_eq!(parse_vat_percent("   "), None);
        assert_eq!(parse_vat_percent("abc"), None);
        assert_eq!(parse_vat_percent("NaN"), None);
    }

    #[test]
    fn rejects_out_of_range_percent() {
        // 200% VAT is nonsense — clamp would mask config errors, so reject.
        assert_eq!(parse_vat_percent("200.5"), None);
        assert_eq!(parse_vat_percent("-1.5"), None);
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(parse_vat_percent("  7  "), Some(7));
    }
}
