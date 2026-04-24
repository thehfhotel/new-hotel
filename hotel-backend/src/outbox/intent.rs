//! `WritebackIntent` — the type-only contract for the legacy MSSQL adapter.
//!
//! Each variant maps 1:1 to a recipe in `docs/legacy-spike/findings.md` §3a–k.
//! When the writeback worker (Phase 3b / 4b — `bin/writeback.rs`) dequeues a
//! `writeback_jobs` row, it deserializes the JSON payload into one of these and
//! dispatches to the matching `writeback/<flow>.rs` recipe.
//!
//! **Type definitions only** — no SQL, no tiberius, no I/O. The exact MSSQL
//! field shapes are derived inside the writeback recipes during Wave 2.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    booking::BookingState, payment::PaymentMethod, shared::DateRange, shared::Money,
};

/// A single durable command queued in the `writeback_jobs` outbox.
///
/// Variants intentionally cover only the 11 spike-validated flows. New legacy
/// writebacks (e.g. minibar charges, refunds — gaps noted in `findings.md` §7)
/// must extend both this enum and add a recipe in `writeback/`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "intent", content = "payload", rename_all = "snake_case")]
pub enum WritebackIntent {
    /// Spike §3b — `INSERT HT_Customers (if new)` + `INSERT HT_Book_H` +
    /// `INSERT HT_Book_Ds` + `INSERT HT_Book_Date` (×nights).
    CreateBooking {
        booking_id: Uuid,
        payload: CreateBookingPayload,
    },

    /// Spike §3c — targeted UPDATEs against `HT_Book_H` / `HT_Book_Ds` and
    /// add/remove `HT_Book_Date` rows. **Skip the legacy app's destructive
    /// DELETE-then-REINSERT pattern** (per spike §3c).
    ModifyBooking {
        booking_id: Uuid,
        changes: BookingChanges,
    },

    /// Spike §3g-bis — clean cancel: 4 UPDATEs + 1 DELETE on `HT_Book_Date`.
    CancelBooking { booking_id: Uuid },

    /// Spike §3a (walk-in) or §3d (linked to existing booking).
    /// 7 INSERTs + 3 UPDATEs across 7 tables.
    CreateCheckIn {
        check_in_id: Uuid,
        payload: CreateCheckInPayload,
    },

    /// Spike §3i — clean targeted DELETEs + audit row in `HT_Rooms_Cancel`.
    /// Subtracts this room's price from `HT_CheckIn_H` totals (multi-room safe).
    CancelCheckIn {
        check_in_id: Uuid,
        reason: Option<String>,
    },

    /// Spike §3f — recompute totals + replace `HT_Room_Status` rows for the
    /// changed date range. Targeted; skip the destructive Phase B.
    ExtendStay {
        check_in_id: Uuid,
        new_end: DateTime<Utc>,
    },

    /// Spike §3e Phase 2 — 5 UPDATEs across `HT_POWER_LOG`, `HT_CheckIn_Ds`,
    /// `HT_Rooms`, `HT_Room_Status`, `HT_CheckIn_H`. **Skip Phase 1** (destructive).
    CheckOut { check_in_id: Uuid },

    /// Spike §3h — `INSERT HT_CheckIn_Pay` + `UPDATE HT_CheckIn_H` totals.
    /// Receipt INSERTs (`HT_Receipt_H` / `HT_Receipt_Ds`) follow in a separate
    /// intent when the user prints.
    RecordPayment {
        check_in_id: Uuid,
        amount: Money,
        method: PaymentMethod,
    },

    /// Spike §3j — `UPDATE HT_Rooms` (by `id`, not `room_no`!) +
    /// `INSERT HT_Housewife` with `h_cin/h_cin_name` looked up from the prior
    /// non-cancelled check-in for this room.
    MarkRoomClean { room_id: Uuid, by: String },
}

/// Payload for [`WritebackIntent::CreateBooking`].
///
/// Carries everything the recipe needs to build the 4 INSERTs without
/// re-querying PG. Customer fields are denormalized (the booking writeback may
/// be the customer's first appearance in MSSQL — recipe inserts both atomically).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookingPayload {
    pub customer_id: Uuid,
    pub legacy_cust_no: Option<String>,
    pub customer_name: String,
    pub customer_phone: Option<String>,

    pub stay: DateRange,
    pub room_no: String,
    pub room_type: String,
    pub price: Money,
    pub nights: i32,

    pub created_by: String,
    pub notes: Option<String>,
}

/// Payload for [`WritebackIntent::CreateCheckIn`].
///
/// Discriminates between walk-in and linked-to-booking via `linked_booking_id`
/// (per spike §3a vs §3d).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckInPayload {
    pub customer_id: Uuid,
    pub legacy_cust_no: Option<String>,

    /// `Some(_)` for "check-in to existing booking" (§3d).
    /// `None` for walk-in (§3a) — recipe also inserts a new customer.
    pub linked_booking_id: Option<Uuid>,
    pub linked_legacy_book_id: Option<String>,

    pub room_no: String,
    pub room_type: String,
    pub stay: DateRange,

    pub price_per_night: Money,
    pub nights: i32,
    pub price_total: Money,

    pub created_by: String,
    pub guest_name_for_registry: String,
    pub guest_country: String,
}

impl WritebackIntent {
    /// Stable string identifier for this variant — matches the `intent` discriminant
    /// produced by `serde(tag = "intent", rename_all = "snake_case")`.
    ///
    /// Persisted into `writeback_jobs.intent` so the worker can dispatch to the
    /// matching recipe without having to deserialize the whole payload.
    pub fn intent_name(&self) -> &'static str {
        match self {
            WritebackIntent::CreateBooking { .. } => "create_booking",
            WritebackIntent::ModifyBooking { .. } => "modify_booking",
            WritebackIntent::CancelBooking { .. } => "cancel_booking",
            WritebackIntent::CreateCheckIn { .. } => "create_check_in",
            WritebackIntent::CancelCheckIn { .. } => "cancel_check_in",
            WritebackIntent::ExtendStay { .. } => "extend_stay",
            WritebackIntent::CheckOut { .. } => "check_out",
            WritebackIntent::RecordPayment { .. } => "record_payment",
            WritebackIntent::MarkRoomClean { .. } => "mark_room_clean",
        }
    }

    /// The aggregate root id this intent mutates.
    ///
    /// Persisted into `writeback_jobs.aggregate_id` so jobs for the same entity
    /// can be located by the index `ix_writeback_jobs_aggregate`.
    pub fn aggregate_id(&self) -> Uuid {
        match self {
            WritebackIntent::CreateBooking { booking_id, .. }
            | WritebackIntent::ModifyBooking { booking_id, .. }
            | WritebackIntent::CancelBooking { booking_id } => *booking_id,
            WritebackIntent::CreateCheckIn { check_in_id, .. }
            | WritebackIntent::CancelCheckIn { check_in_id, .. }
            | WritebackIntent::ExtendStay { check_in_id, .. }
            | WritebackIntent::CheckOut { check_in_id }
            | WritebackIntent::RecordPayment { check_in_id, .. } => *check_in_id,
            WritebackIntent::MarkRoomClean { room_id, .. } => *room_id,
        }
    }
}

/// Diff payload for [`WritebackIntent::ModifyBooking`].
///
/// Only the fields a user actually edits round-trip through here — fields
/// retained as `None` are left untouched. The writeback recipe applies these
/// as targeted UPDATEs (not the legacy app's DELETE-then-REINSERT).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingChanges {
    pub new_stay: Option<DateRange>,
    pub new_room_no: Option<String>,
    pub new_room_type: Option<String>,
    pub new_price: Option<Money>,
    pub new_state: Option<BookingState>,
    pub new_notes: Option<String>,
    pub new_customer_phone: Option<String>,
}
