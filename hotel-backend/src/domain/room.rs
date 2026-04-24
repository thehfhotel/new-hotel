//! Room aggregate.
//!
//! Mirrors the legacy `HT_Rooms` table (per `docs/legacy-spike/findings.md`).
//! Note that the legacy schema exposes both an internal numeric `id` and a
//! string `room_no`; we use a UUID as the canonical PK and store the legacy
//! numeric `id` as `legacy_room_id` for cross-system references.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::shared::RoomNumber;

/// A physical hotel room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    /// Legacy `HT_Rooms.id` (internal numeric, e.g. `15` for room 402).
    pub legacy_room_id: Option<i64>,

    pub room_no: RoomNumber,
    pub room_type: String,

    pub status: RoomStatus,
    pub clean_state: CleanState,
}

/// Whether the room is currently occupied.
///
/// Mirrors the legacy `HT_Rooms.room_use` (`'yes'` / `'no'`) flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomStatus {
    /// `room_use='no'` — the room is unoccupied.
    Available,
    /// `room_use='yes'` — the room is occupied (active check-in).
    Occupied,
    /// Reserved by an active booking but not yet checked in.
    Booked,
    /// Out of service for maintenance (`HT_Rooms.room_manternace`).
    OutOfService,
}

/// Housekeeping state of the room.
///
/// **Confusing legacy mapping** (per spike §3i / §3j):
/// `HT_Rooms.Room_Clean='yes'` actually means *"yes, this room needs cleaning"*.
/// `Room_Clean='no'` means *"no, the room does not need cleaning — it is clean"*.
/// Our enum uses sane names; conversion to the legacy literal happens at the
/// writeback boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanState {
    /// Ready for next guest. Maps to `Room_Clean='no'`.
    Clean,
    /// Needs housekeeping. Maps to `Room_Clean='yes'`.
    NeedsCleaning,
    /// Currently being cleaned.
    InProgress,
}

impl CleanState {
    /// Legacy `HT_Rooms.Room_Clean` literal — note the inverted semantics.
    pub fn legacy_literal(self) -> &'static str {
        match self {
            CleanState::Clean => "no",
            CleanState::NeedsCleaning => "yes",
            // The legacy app does not distinguish "in progress" — treat as needs-cleaning.
            CleanState::InProgress => "yes",
        }
    }
}
