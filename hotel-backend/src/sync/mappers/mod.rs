//! Per-table CT mappers.
//!
//! Each submodule owns the translation from one MSSQL legacy table's
//! Change Tracking rows to canonical PG mutations + `DomainEvent`s. The
//! watcher binary (`bin/sync.rs`) builds a `Vec<Box<dyn MssqlChangeMapper>>`
//! by composing one entry per CT-enabled table — see
//! `CT_ENABLED_TABLES` in that binary for the full set.
//!
//! ## Phase scope
//!
//! - [`customer`] — full I/U/D coverage of `HT_Customers` (5.2).
//! - [`room`] — `HT_Rooms` master mapper (room_clean / room_use mirror)
//!   plus the `HT_Room_Status` *retired* stub (5.4: occupancy is owned
//!   by the check-in aggregate; the stub stays for observability).
//! - [`booking`] — `HT_Book_H` + `HT_Book_Ds` + `HT_Book_Date` aggregate
//!   mappers with shared parent re-load + per-tick coalescing (5.3).
//! - [`checkin`] — `HT_CheckIn_H` + `HT_CheckIn_Ds` aggregate mappers
//!   (5.4). Drives `ht_checkins` UPSERT + `CheckIn{Created,Cancelled}`
//!   / `CheckOutCompleted` events. Triggers a parent-booking
//!   re-projection on full check-out so `Book_Status='ออกแล้ว'`
//!   propagates atomically.
//! - [`payment`] — `HT_CheckIn_Pay` + `HT_Receipt_H` (5.4). The
//!   payment row coalesces back into the check-in aggregate sweep
//!   (totals re-projection); receipts UPSERT into `ht_payments` and
//!   emit `PaymentReceived`.
//!
//! Phase 5.4 completes 10-table CT coverage: every CT-enabled table
//! now has a real mapper or an intentional retired stub.

pub mod booking;
pub mod checkin;
pub mod coupon;
pub mod customer;
pub mod guest_registry;
pub mod mirror;
pub mod payment;
pub mod products;
pub mod rate_tiers;
pub mod room;
pub mod room_calendar;

pub use booking::{
    apply_booking_aggregate, BookingDatesMapper, BookingHeaderMapper, BookingRoomsMapper,
};
pub use checkin::{
    apply_checkin_aggregate, resolve_customer_via_eager_mirror_for_test, CheckInHeaderMapper,
    CheckInRoomsMapper,
};
pub use coupon::apply_canonical_cupon_event;
pub use customer::CustomerMapper;
pub use guest_registry::GuestRegistryMapper;
pub use mirror::{
    BillDebtDsMirrorMapper, BillDebtHMirrorMapper, ChangedRoomMirrorMapper,
    CheckinProductMirrorMapper, CuponMirrorMapper, DepositMirrorMapper, RoomsCancelMirrorMapper,
};
pub use payment::{apply_payment_aggregate, PaymentMapper, ReceiptMapper};
pub use room::{RoomMasterMapper, RoomStatusMapper};
pub use room_calendar::RoomCalendarMapper;
