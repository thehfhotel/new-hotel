//! Shared domain primitives.
//!
//! Pure value types that have no I/O, no SQL, and no framework dependencies.
//! Reusable across every aggregate (Customer, Booking, CheckIn, Room, Payment).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A half-open date/time range `[start, end)`.
///
/// Used by bookings (`stay_start`, `stay_end`) and check-ins (`Cin_Room_In`,
/// `Cin_Room_Out`). Departure is exclusive — the legacy app stores `'11:59:59 AM'`
/// for the same intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    /// Returns `true` when `start < end`.
    pub fn is_valid(&self) -> bool {
        self.start < self.end
    }

    /// Number of overnight stays the range spans.
    ///
    /// Counts calendar nights, not 24h windows. A 1-night stay (Mon afternoon →
    /// Tue noon) returns `1`. Returns `0` if the range is degenerate.
    pub fn nights(&self) -> i64 {
        let days = (self.end.date_naive() - self.start.date_naive()).num_days();
        if days < 0 {
            0
        } else {
            days
        }
    }
}

/// Money represented as integer minor units (Thai baht satang, i.e. 1/100 baht).
///
/// **Why integer cents instead of `BigDecimal`:** Thai baht charged by hotels are
/// integer-friendly (room rates of 890, 711, 1780 in our spike captures — never
/// fractional). The `bigdecimal` feature was dropped from sqlx in v2.8.1 (see
/// CLAUDE.md). i64 cents avoids floating-point drift entirely while still
/// supporting future per-satang precision if a branch ever adds it.
///
/// Range: ±9.2 × 10^16 satang ≈ ±9.2 × 10^14 baht — comfortably above any
/// realistic hotel transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money(pub i64);

impl Money {
    pub const ZERO: Money = Money(0);

    /// Construct from whole baht (no satang).
    pub fn from_baht(baht: i64) -> Self {
        Money(baht.saturating_mul(100))
    }

    /// Construct from satang (1/100 baht).
    pub fn from_satang(satang: i64) -> Self {
        Money(satang)
    }

    /// Whole baht component (truncates satang).
    pub fn whole_baht(self) -> i64 {
        self.0 / 100
    }

    /// Satang component (always 0..=99 for non-negative values).
    pub fn satang_component(self) -> i64 {
        self.0 % 100
    }

    /// Total satang value.
    pub fn as_satang(self) -> i64 {
        self.0
    }
}

/// A hotel room number as displayed to staff (e.g. "402", "508").
///
/// Stored as a string because the legacy app uses string columns
/// (`HT_Rooms.room_no VARCHAR`) and some branches use non-numeric suffixes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoomNumber(pub String);

impl RoomNumber {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RoomNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
