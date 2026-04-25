//! Legacy SQL value formatting helpers.
//!
//! The legacy app emits **literal SQL text** with no parameter binding
//! (per `docs/legacy-spike/findings.md` §3 preface). To match its output
//! exactly we render values into the same shapes:
//!
//! * Dates: `M/D/YYYY h:mm:ss tt` — US-style with no leading zeros and
//!   12-hour AM/PM. Spike §4b. Example: `4/24/2026 5:05:04 PM`.
//! * Booleans / status: copied verbatim from `constants.rs`.
//! * Strings: SQL-quoted with single-quote doubling (`O'Brien` → `O''Brien`).
//! * NULL: rendered as `''` (empty string) per spike §3k — many legacy
//!   `varchar` columns are nullable but the .NET WinForms controls misbehave
//!   on NULL, so we always send empty string.

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

/// Format a `DateTime<Utc>` into the legacy app's `M/D/YYYY h:mm:ss tt`
/// representation (spike §4b).
///
/// Examples (verified against captures):
/// * `2026-04-24T17:05:04Z` → `"4/24/2026 5:05:04 PM"`
/// * `2026-04-26T11:59:59Z` → `"4/26/2026 11:59:59 AM"`
/// * `2026-04-25T00:00:00Z` → `"4/25/2026 12:00:00 AM"` (midnight = 12 AM in 12-hour clock)
/// * `2026-04-25T12:00:00Z` → `"4/25/2026 12:00:00 PM"` (noon = 12 PM)
///
/// We deliberately treat the input as **wall-clock Thai time** (matching the
/// legacy app's `CultureInfo.InvariantCulture` / `en-US` `DateTime.ToString()`
/// — see spike §4b). All hotel data lives in GMT+7 by convention; the
/// timezone offset is dropped at the format layer.
pub fn format_legacy_datetime(dt: DateTime<Utc>) -> String {
    format_legacy_naive(dt.naive_utc())
}

/// Like [`format_legacy_datetime`] but for `NaiveDateTime` (no offset).
pub fn format_legacy_naive(dt: NaiveDateTime) -> String {
    let hour_24 = dt.hour();
    let (hour_12, period) = to_12_hour(hour_24);
    format!(
        "{}/{}/{} {}:{:02}:{:02} {}",
        dt.month(),
        dt.day(),
        dt.year(),
        hour_12,
        dt.minute(),
        dt.second(),
        period,
    )
}

/// Date-only form: `M/D/YYYY` (no time component). Used for `HT_Book_Date.Book_date_ds`
/// and `HT_Room_Status.room_date` (per spike captures).
pub fn format_legacy_date(date: NaiveDate) -> String {
    format!("{}/{}/{}", date.month(), date.day(), date.year())
}

/// Convert 24-hour clock to (12-hour, "AM"/"PM"). Matches .NET's
/// `CultureInfo.InvariantCulture` `tt` format specifier.
fn to_12_hour(hour_24: u32) -> (u32, &'static str) {
    if hour_24 == 0 {
        (12, "AM")
    } else if hour_24 < 12 {
        (hour_24, "AM")
    } else if hour_24 == 12 {
        (12, "PM")
    } else {
        (hour_24 - 12, "PM")
    }
}

/// OLE Automation Date serial — days since 1899-12-30. Used by
/// `HT_Room_Status.room_date_oa` (spike §4b).
///
/// Examples: `2026-04-24` → `46136`, `2026-04-25` → `46137`.
pub fn date_to_ole_serial(date: NaiveDate) -> f64 {
    let epoch = NaiveDate::from_ymd_opt(1899, 12, 30).expect("OLE epoch is a valid date");
    (date - epoch).num_days() as f64
}

/// Quote a string for inline SQL — escapes embedded single quotes and wraps
/// in single quotes. Mirrors what the legacy `.Net SqlClient` produces when
/// it interpolates parameter values into the captured statements.
///
/// # Examples
/// ```
/// use hotel_backend::writeback::format::sql_quote;
/// assert_eq!(sql_quote("O'Brien"), "'O''Brien'");
/// assert_eq!(sql_quote(""), "''");
/// ```
pub fn sql_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// SQL-quote a `Some(_)` value, otherwise `''` (empty string). Per spike §3k:
/// the legacy app stores `''` rather than NULL for optional `varchar` columns
/// because some .NET WinForms controls crash on NULL string concatenation.
pub fn sql_quote_or_empty(value: Option<&str>) -> String {
    sql_quote(value.unwrap_or(""))
}

/// Convert a `DateTime<Utc>` into its midnight (00:00:00) equivalent for the
/// **same calendar day**. Spike §3k requires `HT_Book_H.Book_Date_in/out` at
/// midnight so the booking-list view renders correctly.
pub fn midnight_of(dt: DateTime<Utc>) -> DateTime<Utc> {
    let date = dt.date_naive();
    Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).expect("hms 0 0 0 is valid"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn format_datetime_matches_spike_capture_5pm() {
        // Spike checkout-20260424-100323/writes.txt:20 — Cin_Room_Out='4/24/2026 5:05:04 PM'
        let dt = Utc.with_ymd_and_hms(2026, 4, 24, 17, 5, 4).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/24/2026 5:05:04 PM");
    }

    #[test]
    fn format_datetime_matches_spike_booking_checkin() {
        // Spike booking-checkin-20260424-101838/writes.txt:3 — Book_Date_in='4/25/2026 12:00:00 PM'
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/25/2026 12:00:00 PM");
    }

    #[test]
    fn format_datetime_matches_spike_booking_checkout() {
        // Spike — Book_Date_out='4/26/2026 11:59:59 AM'
        let dt = Utc.with_ymd_and_hms(2026, 4, 26, 11, 59, 59).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/26/2026 11:59:59 AM");
    }

    #[test]
    fn format_datetime_midnight_renders_as_12_am() {
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 0, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/25/2026 12:00:00 AM");
    }

    #[test]
    fn format_datetime_noon_renders_as_12_pm() {
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/25/2026 12:00:00 PM");
    }

    #[test]
    fn format_date_no_leading_zeros() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();
        assert_eq!(format_legacy_date(date), "4/5/2026");
    }

    #[test]
    fn ole_serial_matches_spike_captures() {
        // Spike walkin-20260424-095304/writes.txt:6 — room_date_oa=46136 for 4/24/2026
        let day_4_24 = NaiveDate::from_ymd_opt(2026, 4, 24).unwrap();
        assert_eq!(date_to_ole_serial(day_4_24) as i64, 46136);
        // Spike booking-checkin-20260424-101838/writes.txt:35 — 46137 for 4/25/2026
        let day_4_25 = NaiveDate::from_ymd_opt(2026, 4, 25).unwrap();
        assert_eq!(date_to_ole_serial(day_4_25) as i64, 46137);
    }

    #[test]
    fn sql_quote_doubles_embedded_quotes() {
        assert_eq!(sql_quote("O'Brien"), "'O''Brien'");
        assert_eq!(sql_quote(""), "''");
        assert_eq!(sql_quote("plain"), "'plain'");
    }

    #[test]
    fn sql_quote_or_empty_handles_none() {
        assert_eq!(sql_quote_or_empty(None), "''");
        assert_eq!(sql_quote_or_empty(Some("hi")), "'hi'");
    }

    #[test]
    fn midnight_of_preserves_calendar_day() {
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 14, 30, 45).unwrap();
        assert_eq!(midnight_of(dt), Utc.with_ymd_and_hms(2026, 4, 25, 0, 0, 0).unwrap());
    }
}
