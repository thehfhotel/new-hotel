//! Domain layer — pure types and business invariants.
//!
//! **Rules** (per `docs/architecture.md` §1, §2):
//!
//! 1. NO I/O. No `tokio`, no `sqlx`, no `axum`, no `tiberius`.
//! 2. NO database SQL. Persistence lives in `repository/`.
//! 3. NO HTTP. Wire shapes live in `routes/`.
//! 4. Pure types + state machines + invariants. Testable in isolation.
//!
//! These types are the *vocabulary* spoken by every other layer.
//! When the legacy MSSQL is decommissioned (state C in `architecture.md` §3),
//! this module does not change at all.

pub mod booking;
pub mod checkin;
pub mod customer;
pub mod payment;
pub mod room;
pub mod shared;

pub use booking::{Booking, BookingState};
pub use checkin::{CheckIn, CheckInState};
pub use customer::{Customer, CustomerType};
pub use payment::{Payment, PaymentMethod};
pub use room::{CleanState, Room, RoomStatus};
pub use shared::{DateRange, Money, RoomNumber};
