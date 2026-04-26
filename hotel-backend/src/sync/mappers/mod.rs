//! Per-table CT mappers.
//!
//! Each submodule owns the translation from one MSSQL legacy table's
//! Change Tracking rows to canonical PG mutations + `DomainEvent`s. The
//! watcher binary (`bin/sync.rs`) builds a `Vec<Box<dyn MssqlChangeMapper>>`
//! by composing one entry per CT-enabled table — see
//! `CT_ENABLED_TABLES` in that binary for the full set.
//!
//! ## Phase 5.2 scope
//!
//! - [`customer`] — full I/U/D coverage of `HT_Customers`.
//! - [`room`] — `HT_Rooms` master mapper (room_clean / room_use mirror)
//!   and `HT_Room_Status` stub (deferred to 5.3 / 5.4 where the booking
//!   / checkin mappers own the per-night occupancy table).
//!
//! Booking / checkin tables (`HT_Book_*`, `HT_CheckIn_*`, `HT_Receipt_H`)
//! still ride on `NoopMapper` until 5.3 lands.

pub mod customer;
pub mod room;

pub use customer::CustomerMapper;
pub use room::{RoomMasterMapper, RoomStatusMapper};
