//! Check-in aggregate.
//!
//! Pure types. The service layer drives the state machine; the writeback layer
//! translates state transitions into legacy `HT_CheckIn_H` / `HT_CheckIn_Ds`
//! mutations per `docs/legacy-spike/findings.md` §3a, §3d, §3e, §3i.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::shared::{DateRange, Money, RoomNumber};

/// An active or completed check-in (one per stay, one or more rooms).
///
/// `legacy_cin_no` is the `CH\d{2}-\d{6}` identifier minted by the legacy app
/// (e.g. `"CH26-005228"`). `None` until the writeback worker assigns one.
///
/// `booking_id` is `None` for walk-ins (per spike §3a) and `Some(_)` for
/// check-ins linked to an advance booking (per spike §3d).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckIn {
    pub id: Uuid,
    pub legacy_cin_no: Option<String>,

    pub booking_id: Option<Uuid>,
    pub customer_id: Uuid,

    pub status: CheckInState,
    pub room_no: RoomNumber,

    pub stay: DateRange,

    pub total_price_room: Money,
    pub total_price_product: Money,
    pub total_price_net: Money,
    pub total_price_pay: Money,
    pub total_price_balance: Money,
}

/// State machine for a check-in.
///
/// ```text
/// Active  ──checkout──▶  CheckedOut
///    │
///    └──cancel──▶  Cancelled
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckInState {
    /// Guest is currently occupying a room.
    Active,
    /// Guest has departed; balance settled.
    CheckedOut,
    /// Check-in was reversed before/during the stay (per spike §3i).
    Cancelled,
}

impl CheckInState {
    /// Legacy `HT_CheckIn_Ds.Cin_Room_Status` literal.
    ///
    /// Note the inconsistency captured in the spike: `Cin_Room_Status` uses
    /// `"Check-Out"` (hyphen, English) while `HT_Room_Status.room_status` uses
    /// `"Check Out"` (space, English). See [`Self::legacy_room_status_literal`].
    pub fn legacy_literal(self) -> &'static str {
        match self {
            CheckInState::Active => "เข้าพัก",
            CheckInState::CheckedOut => "Check-Out",
            CheckInState::Cancelled => "ยกเลิก",
        }
    }

    /// Legacy `HT_Room_Status.room_status` literal (note the spacing
    /// inconsistency vs [`Self::legacy_literal`]).
    pub fn legacy_room_status_literal(self) -> &'static str {
        match self {
            CheckInState::Active => "เข้าพัก",
            CheckInState::CheckedOut => "Check Out",
            CheckInState::Cancelled => "ยกเลิก",
        }
    }
}
