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

use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
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

/// Split a VAT-inclusive total into `(before_vat, vat)`, both rounded to
/// 2 decimal places. The legacy app's receipt-print path treats the
/// headline `Total` as VAT-inclusive at 7%, then renders both
/// components for the printed receipt. Mirrors the captured
/// `HT_Receipt_H` math: `Total=801.00 → BeforeVat=748.60, Vat=52.40`
/// (verified from `/tmp/legacy-events-full.log`).
///
/// `vat_percent=7` → divisor `1.07`; the Vat is the remainder so the
/// two parts sum back to exactly `total` after rounding.
pub fn vat_inclusive_split(total: f64, vat_percent: i32) -> (f64, f64) {
    let divisor = 1.0 + (vat_percent as f64) / 100.0;
    let before_vat = round_money(total / divisor);
    let vat = round_money(total - before_vat);
    (before_vat, vat)
}

/// Round a money figure to 2 decimal places using banker's rounding
/// (round-half-to-even) — the convention the legacy .NET app's
/// `Math.Round(value, 2)` uses by default
/// (`MidpointRounding.ToEven`). Audit H6: the prior implementation used
/// `f64::round` (round-half-away-from-zero) which diverges from .NET on
/// the 0.005 / 0.015 / 0.025 midpoints. Today's whole-baht prices never
/// hit those midpoints, but VAT splits and any future fractional pricing
/// would silently fork from the legacy app.
fn round_money(value: f64) -> f64 {
    (value * 100.0).round_ties_even() / 100.0
}

/// Render an `f64` money amount with exactly 2 decimal places (e.g.
/// `801.00`, `52.40`) — the format the legacy app emits for VAT split
/// columns on `HT_Receipt_H`. Use [`f64_sql`] elsewhere where the
/// legacy app emits trailing-zero-stripped numerics.
pub fn money_2dp(value: f64) -> Result<String, crate::writeback::error::WritebackError> {
    if !value.is_finite() {
        return Err(crate::writeback::error::WritebackError::Recipe(format!(
            "non-finite f64 cannot be rendered as money: {value}"
        )));
    }
    Ok(format!("{value:.2}"))
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

/// Snap a stay-end `DateTime<Utc>` to `11:59:59 AM` of the same Bangkok
/// calendar day. Per spike §3b — convenient for date-range BETWEEN queries
/// (`HT_Book_Ds.Book_Room_End`, etc.). Uses Bangkok time to determine the
/// calendar day (a 17:00Z instant is the *next* day in Bangkok, so
/// `date_naive()` on the raw UTC instant would return the wrong day).
///
/// Wave 6 de-duplication target — was previously defined identically in
/// `booking_create.rs` and `booking_modify.rs`; the shared definition lives
/// here next to [`midnight_of`].
pub fn end_of_stay_at_almost_noon(stay_end: DateTime<Utc>) -> DateTime<Utc> {
    let bkk_date = stay_end.with_timezone(&Bangkok).date_naive();
    let bkk_target = bkk_date
        .and_hms_opt(11, 59, 59)
        .expect("11:59:59 is a valid time");
    Bangkok
        .from_local_datetime(&bkk_target)
        .single()
        .expect("11:59:59 is unambiguous in Asia/Bangkok (no DST)")
        .with_timezone(&Utc)
}

/// Enumerate Bangkok calendar nights spanning `[stay_start, stay_end)`. Each
/// night becomes one `HT_Book_Date` / `HT_Room_Status` row in the recipes
/// that consume this. Boundaries are the Bangkok calendar day, not the UTC
/// one (a 17:00Z instant is already the next day in Bangkok).
///
/// Wave 6 de-duplication target — was defined identically (with minor
/// trivial differences) in `booking_create.rs`, `booking_modify.rs`,
/// `walkin.rs`, `checkin_to_booking.rs`, and `extend_stay.rs`.
///
/// **Safety nets** (Wave 6 LOW item 6 — was silent in earlier copies):
/// * The range is capped at 365 nights. Hitting the cap logs a WARN; the
///   truncated vec is still returned (preserves the prior behaviour of
///   never aborting allocation of `HT_Room_Status` ids).
/// * An empty range (`stay_end <= stay_start` in Bangkok) returns an error
///   so a caller bug surfaces immediately. Earlier copies silently injected
///   a phantom single-night row, which was masking the bug at the cost of
///   writing one ghost calendar row to MSSQL.
pub fn enumerate_calendar_nights(
    stay_start: DateTime<Utc>,
    stay_end: DateTime<Utc>,
) -> Result<Vec<NaiveDate>, crate::writeback::error::WritebackError> {
    const NIGHT_CAP: usize = 365;
    let start = bangkok_date(stay_start);
    let end = bangkok_date(stay_end);
    if end <= start {
        return Err(crate::writeback::error::WritebackError::Recipe(format!(
            "enumerate_calendar_nights: empty range — stay_start {start} must be \
             strictly before stay_end {end} (Bangkok calendar)"
        )));
    }
    let mut nights = Vec::new();
    let mut day = start;
    while day < end {
        nights.push(day);
        if nights.len() >= NIGHT_CAP {
            tracing::warn!(
                stay_start = %start,
                stay_end = %end,
                cap = NIGHT_CAP,
                "enumerate_calendar_nights hit night cap — truncating",
            );
            break;
        }
        day = match day.checked_add_days(Days::new(1)) {
            Some(d) => d,
            None => break,
        };
    }
    Ok(nights)
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
    fn vat_inclusive_split_matches_legacy_801_capture() {
        // Receipt_H 20663 from /tmp/legacy-events-full.log:
        //   Total=801.00, BeforeVat=748.60, Vat=52.40, VatPer=7
        let (before, vat) = vat_inclusive_split(801.00, 7);
        assert_eq!(before, 748.60);
        assert_eq!(vat, 52.40);
    }

    #[test]
    fn vat_inclusive_split_matches_legacy_3560_capture() {
        // Receipt_H 20662 from /tmp/legacy-events-full.log:
        //   Total=3560.00, BeforeVat=3327.10, Vat=232.90, VatPer=7
        let (before, vat) = vat_inclusive_split(3560.00, 7);
        assert_eq!(before, 3327.10);
        assert_eq!(vat, 232.90);
    }

    #[test]
    fn vat_inclusive_split_matches_legacy_1390_capture() {
        // Receipt_H 20664 from /tmp/legacy-events-full.log:
        //   Total=1390.00, BeforeVat=1299.07, Vat=90.93, VatPer=7
        let (before, vat) = vat_inclusive_split(1390.00, 7);
        assert_eq!(before, 1299.07);
        assert_eq!(vat, 90.93);
    }

    #[test]
    fn money_2dp_renders_two_decimals() {
        assert_eq!(money_2dp(801.0).unwrap(), "801.00");
        assert_eq!(money_2dp(52.4).unwrap(), "52.40");
        assert_eq!(money_2dp(0.0).unwrap(), "0.00");
    }

    #[test]
    fn round_money_uses_banker_s_rounding() {
        // .NET's `Math.Round(value, 2)` defaults to MidpointRounding.ToEven
        // (banker's rounding). Round-half-away-from-zero would give 0.01 / 0.02 / 0.03;
        // banker's rounding gives 0.00 / 0.02 / 0.02 (round to nearest even at the
        // exact midpoint). Verified against .NET behavior.
        assert_eq!(round_money(0.005), 0.00, "0.005 should round to 0.00 (even)");
        assert_eq!(round_money(0.015), 0.02, "0.015 should round to 0.02 (even)");
        assert_eq!(round_money(0.025), 0.02, "0.025 should round to 0.02 (even)");
        assert_eq!(round_money(0.035), 0.04, "0.035 should round to 0.04 (even)");
        // Non-midpoint values still round normally.
        assert_eq!(round_money(0.014), 0.01);
        assert_eq!(round_money(0.016), 0.02);
    }

    #[test]
    fn midnight_of_handles_late_utc_that_is_next_bangkok_day() {
        // UTC 18:00 on 4/24 = Bangkok 01:00 on 4/25
        // Midnight of that Bangkok day = Bangkok 00:00 on 4/25 = UTC 17:00 on 4/24
        let dt = Utc.with_ymd_and_hms(2026, 4, 24, 18, 0, 0).unwrap();
        assert_eq!(format_legacy_datetime(midnight_of(dt)), "4/25/2026 12:00:00 AM");
    }

    /// Wave 6: shared helper snaps to the Bangkok 11:59:59 of the same day.
    /// Mirrors the prior per-recipe implementations exactly.
    #[test]
    fn end_of_stay_at_almost_noon_snaps_to_bangkok_almost_noon() {
        // Bangkok 14:30 on 4/26 → Bangkok 11:59:59 on 4/26 = UTC 04:59:59
        let dt = Utc.with_ymd_and_hms(2026, 4, 26, 7, 30, 0).unwrap();
        let snapped = end_of_stay_at_almost_noon(dt);
        assert_eq!(format_legacy_datetime(snapped), "4/26/2026 11:59:59 AM");
    }

    /// Wave 6: enumerate_calendar_nights one-night case (overnight stay).
    #[test]
    fn enumerate_calendar_nights_handles_overnight() {
        // Bangkok noon 4/24 → Bangkok noon 4/25 = one night [4/24, 4/25)
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 24, 5, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(nights, vec![NaiveDate::from_ymd_opt(2026, 4, 24).unwrap()]);
    }

    /// Wave 6: enumerate_calendar_nights multi-night case.
    #[test]
    fn enumerate_calendar_nights_handles_three_nights() {
        let nights = enumerate_calendar_nights(
            Utc.with_ymd_and_hms(2026, 4, 24, 5, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 4, 27, 5, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(
            nights,
            vec![
                NaiveDate::from_ymd_opt(2026, 4, 24).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 25).unwrap(),
                NaiveDate::from_ymd_opt(2026, 4, 26).unwrap(),
            ]
        );
    }

    /// Wave 6 LOW item 6: an empty range surfaces as a Recipe error rather
    /// than silently injecting a phantom night. Earlier per-recipe copies
    /// pushed `start` onto the result and pressed on — that masked caller
    /// bugs at the cost of one ghost `HT_Book_Date` row in MSSQL.
    #[test]
    fn enumerate_calendar_nights_empty_range_errors() {
        // stay_end == stay_start → no nights → error
        let now = Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap();
        let err = enumerate_calendar_nights(now, now).unwrap_err();
        assert!(
            err.to_string().contains("empty range"),
            "expected 'empty range' in error, got: {err}"
        );
    }

    /// Wave 6 LOW item 6: stay_end strictly before stay_start also errors
    /// (same root cause — caller passed an inverted range).
    #[test]
    fn enumerate_calendar_nights_inverted_range_errors() {
        let later = Utc.with_ymd_and_hms(2026, 4, 26, 5, 0, 0).unwrap();
        let earlier = Utc.with_ymd_and_hms(2026, 4, 25, 5, 0, 0).unwrap();
        let err = enumerate_calendar_nights(later, earlier).unwrap_err();
        assert!(err.to_string().contains("empty range"));
    }

    /// Wave 6 LOW item 6: the 365-night cap holds. Returns 365 entries with
    /// the cap-truncate WARN logged (test only asserts the count).
    #[test]
    fn enumerate_calendar_nights_caps_at_365() {
        // 400 calendar days in Bangkok between these two instants.
        let start = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2027, 2, 5, 0, 0, 0).unwrap();
        let nights = enumerate_calendar_nights(start, end).unwrap();
        assert_eq!(nights.len(), 365, "must truncate at 365-night cap");
    }
}
