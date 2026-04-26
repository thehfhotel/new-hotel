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
//!   and `HT_Room_Status` stub (deferred to 5.4 where the checkin
//!   mapper owns the per-night occupancy table) (5.2).
//! - [`booking`] — `HT_Book_H` + `HT_Book_Ds` + `HT_Book_Date` aggregate
//!   mappers with shared parent re-load + per-tick coalescing (5.3).
//!
//! Checkin tables (`HT_CheckIn_*`, `HT_Receipt_H`) still ride on
//! `NoopMapper` until 5.4 lands.

pub mod booking;
pub mod customer;
pub mod room;

pub use booking::{
    apply_booking_aggregate, BookingDatesMapper, BookingHeaderMapper, BookingRoomsMapper,
};
pub use customer::CustomerMapper;
pub use room::{RoomMasterMapper, RoomStatusMapper};
