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
use chrono_tz::Asia::Bangkok;

/// Format a `DateTime<Utc>` into the legacy app's `M/D/YYYY h:mm:ss tt`
/// representation (spike §4b), converting from UTC to Asia/Bangkok wall-clock
/// time first.
///
/// **Why the timezone conversion:** the .NET app stores Thai-local naive
/// datetimes (no offset). Our PG `TIMESTAMPTZ` columns store real UTC
/// instants. Without the conversion, every value we write to MSSQL would be
/// 7h behind the wall-clock time the receptionist actually entered.
///
/// Examples (the input is the real UTC instant; the output is the Bangkok
/// wall clock that the .NET app expects):
/// * `2026-04-24T10:05:04Z` (= 17:05 Bangkok) → `"4/24/2026 5:05:04 PM"`
/// * `2026-04-26T04:59:59Z` (= 11:59 Bangkok) → `"4/26/2026 11:59:59 AM"`
/// * `2026-04-24T17:00:00Z` (= 00:00 Bangkok next day) → `"4/25/2026 12:00:00 AM"`
pub fn format_legacy_datetime(dt: DateTime<Utc>) -> String {
    format_legacy_naive(dt.with_timezone(&Bangkok).naive_local())
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

/// Convert a UTC instant to its Bangkok calendar day. Use this everywhere we
/// derive a `NaiveDate` from a `DateTime<Utc>` — `dt.date_naive()` returns
/// the UTC day, which is wrong for any instant after 17:00Z (already the
/// next day in Bangkok). Spike captures and the .NET app's date logic both
/// use the Bangkok calendar.
pub fn bangkok_date(dt: DateTime<Utc>) -> NaiveDate {
    dt.with_timezone(&Bangkok).date_naive()
}

/// Format a UTC instant using a chrono `strftime` pattern in Bangkok-local
/// wall-clock. Use for non-spike-captured display strings (e.g. the
/// `HT_Rooms.room_book_ds` summary text).
pub fn format_bangkok(dt: DateTime<Utc>, fmt: &str) -> String {
    dt.with_timezone(&Bangkok).format(fmt).to_string()
}

/// Render an `f64` as a SQL numeric literal. Errors loudly on NaN/Infinity
/// rather than letting `format!("{}", f64::NAN)` produce the literal string
/// `"NaN"`, which would silently emit invalid SQL like `Total_Price=NaN` and
/// fail the entire transaction (audit HIGH-4).
///
/// Use this instead of bare `format!("{amount}")` everywhere a recipe
/// interpolates a money / count / price value into SQL.
pub fn f64_sql(value: f64) -> Result<String, crate::writeback::error::WritebackError> {
    if !value.is_finite() {
        return Err(crate::writeback::error::WritebackError::Recipe(format!(
            "non-finite f64 cannot be rendered as SQL: {value}"
        )));
    }
    Ok(value.to_string())
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

/// Convert a `DateTime<Utc>` into midnight (00:00:00) of its **Bangkok
/// calendar day**. Spike §3k requires `HT_Book_H.Book_Date_in/out` at midnight
/// so the booking-list view renders correctly. The day boundary must be the
/// Bangkok one — a 17:00Z instant is 00:00 the *next* day in Bangkok, so
/// `date_naive()` on the raw UTC instant returns the wrong calendar day.
pub fn midnight_of(dt: DateTime<Utc>) -> DateTime<Utc> {
    let bkk_date = dt.with_timezone(&Bangkok).date_naive();
    let bkk_midnight = bkk_date
        .and_hms_opt(0, 0, 0)
        .expect("hms 0 0 0 is valid");
    Bangkok
        .from_local_datetime(&bkk_midnight)
        .single()
        .expect("midnight is unambiguous in Asia/Bangkok (no DST)")
        .with_timezone(&Utc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn format_datetime_matches_spike_capture_5pm() {
        // Spike checkout-20260424-100323/writes.txt:20 — Cin_Room_Out='4/24/2026 5:05:04 PM'
        // Bangkok 17:05:04 = UTC 10:05:04
        let dt = Utc.with_ymd_and_hms(2026, 4, 24, 10, 5, 4).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/24/2026 5:05:04 PM");
    }

    #[test]
    fn format_datetime_matches_spike_booking_checkin() {
        // Spike booking-checkin-20260424-101838/writes.txt:3 — Book_Date_in='4/25/2026 12:00:00 PM'
        // Bangkok noon = UTC 05:00
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/25/2026 12:00:00 PM");
    }

    #[test]
    fn format_datetime_matches_spike_booking_checkout() {
        // Spike — Book_Date_out='4/26/2026 11:59:59 AM'
        // Bangkok 11:59:59 = UTC 04:59:59
        let dt = Utc.with_ymd_and_hms(2026, 4, 26, 4, 59, 59).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/26/2026 11:59:59 AM");
    }

    #[test]
    fn format_datetime_midnight_renders_as_12_am() {
        // Bangkok midnight on 4/25 = UTC 17:00 on 4/24
        let dt = Utc.with_ymd_and_hms(2026, 4, 24, 17, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/25/2026 12:00:00 AM");
    }

    #[test]
    fn format_datetime_noon_renders_as_12_pm() {
        // Bangkok noon = UTC 05:00
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(dt), "4/25/2026 12:00:00 PM");
    }

    #[test]
    fn format_datetime_converts_utc_to_bangkok() {
        // Regression for CRIT-2: a real UTC instant from PG must be shifted
        // +7h before formatting. UTC noon = 19:00 Bangkok, NOT 12:00.
        let utc_noon = Utc.with_ymd_and_hms(2026, 4, 25, 12, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(utc_noon), "4/25/2026 7:00:00 PM");
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
    fn midnight_of_preserves_bangkok_calendar_day() {
        // Bangkok 14:30:45 on 4/25 = UTC 07:30:45 on 4/25
        // Bangkok midnight on 4/25 = UTC 17:00:00 on 4/24
        let dt = Utc.with_ymd_and_hms(2026, 4, 25, 7, 30, 45).unwrap();
        assert_eq!(midnight_of(dt), Utc.with_ymd_and_hms(2026, 4, 24, 17, 0, 0).unwrap());
    }

    #[test]
    fn midnight_of_handles_late_utc_that_is_next_bangkok_day() {
        // UTC 18:00 on 4/24 = Bangkok 01:00 on 4/25
        // Midnight of that Bangkok day = Bangkok 00:00 on 4/25 = UTC 17:00 on 4/24
        let dt = Utc.with_ymd_and_hms(2026, 4, 24, 18, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(midnight_of(dt)), "4/25/2026 12:00:00 AM");
    }
}
