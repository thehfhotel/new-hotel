//! Booking aggregate.
//!
//! Pure domain types — no I/O, no SQL. The booking lifecycle state machine
//! (`BookingState`) is enforced here; persistence and writeback happen in the
//! repository / service / writeback layers.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::shared::{DateRange, Money, RoomNumber};

/// A future or active room booking.
///
/// `legacy_book_id` is the `R\d{6}` identifier minted by the legacy app
/// (e.g. `"R014810"`). It is `None` for bookings we created until the writeback
/// worker assigns one via the TABLOCKX MAX+1 allocation pattern (see
/// `docs/legacy-spike/findings.md` §6).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,
    pub legacy_book_id: Option<String>,

    pub customer_id: Uuid,

    pub state: BookingState,

    pub stay: DateRange,
    pub room_no: Option<RoomNumber>,

    pub price: Money,

    pub notes: Option<String>,
}

/// State machine for a booking.
///
/// Transitions (validated in the service layer, not the type):
///
/// ```text
/// Pending  ──confirm──▶  Active  ──check_in──▶  CheckedIn  ──check_out──▶  Completed
///    │                      │                        │
///    └──────────────────────┴────────cancel──────────┴──▶  Cancelled
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingState {
    /// Tentative — not yet confirmed.
    Pending,
    /// Confirmed; awaiting arrival.
    Active,
    /// Guest has checked in (booking and check-in linked via `Cin_Book_no`).
    CheckedIn,
    /// Guest has checked out — terminal success state.
    Completed,
    /// Cancelled (legacy literal: `'ยกเลิก'`). Records preserved per spike §3g-bis.
    Cancelled,
}

impl BookingState {
    /// Legacy `HT_Book_H.Book_Status` literal that mirrors this state.
    pub fn legacy_literal(self) -> &'static str {
        match self {
            BookingState::Pending => "ยังไม่ยืนยัน",
            BookingState::Active => "ยืนยัน",
            BookingState::CheckedIn => "เข้าพัก",
            BookingState::Completed => "ออกแล้ว",
            BookingState::Cancelled => "ยกเลิก",
        }
    }

    /// Numeric code used in `HT_Book_Ds.Book_status`.
    ///
    /// Per spike §3g-bis: `1` = active, `3` = cancelled, `0` = draft.
    pub fn legacy_numeric(self) -> i32 {
        match self {
            BookingState::Pending => 0,
            BookingState::Active | BookingState::CheckedIn | BookingState::Completed => 1,
            BookingState::Cancelled => 3,
        }
    }
}
