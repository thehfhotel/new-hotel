//! Repository layer — PG-only data access behind aggregate-scoped traits.
//!
//! Per `docs/architecture.md` §1, §2, §6. Each aggregate (Customer, Booking,
//! CheckIn, Room, Payment, Inventory, Outbox, EventLog) gets:
//!
//! 1. A `pub trait <Aggregate>Repository` declaring the data operations.
//! 2. A `pub struct Pg<Aggregate>Repository` providing the PostgreSQL impl.
//!
//! **Rules** (per §2 separation of concerns):
//!
//! - Repository methods talk to PostgreSQL only — never MSSQL.
//! - Methods accept `impl sqlx::PgExecutor<'_>` so a caller can pass either a
//!   `&PgPool` (single-shot reads) or `&mut *tx` (composed transactional writes
//!   in the upcoming service layer — Phase 2).
//! - When a single repo method must execute multiple statements atomically, it
//!   takes `&mut sqlx::Transaction<'_, Postgres>` directly so it can perform
//!   the BEGIN/COMMIT itself or run as part of a larger transaction.
//! - Repositories know about domain types and `sqlx`. They do NOT import
//!   `axum`, HTTP error types, or HTTP DTOs.
//!
//! Routes obtain repositories via `AppState` (see `routes::mode::AppState`),
//! held as `Arc<dyn ...Repository + Send + Sync>` so that test doubles can be
//! injected without changing route code.

pub mod booking;
// Loyalty-channel cross-aggregate queries (availability / holds / stay
// snapshot). Free functions, not a trait — see the module doc for why.
pub mod channel;
pub mod checkin;
pub mod customer;
pub mod inventory;
pub mod payment;
pub mod room;
pub mod session;
pub mod settings;
pub mod user;

pub use booking::{BookingRepository, PgBookingRepository};
pub use checkin::{CheckInRepository, PgCheckInRepository};
pub use customer::{CustomerRepository, PgCustomerRepository};
pub use inventory::{InventoryRepository, PgInventoryRepository};
pub use payment::{PaymentRepository, PgPaymentRepository};
pub use room::{PgRoomRepository, RoomRepository};
pub use session::{PgSessionRepository, SessionRepository};
pub use user::{PgUserRepository, UserRepository};

// Outbox + EventBus impls live under `crate::outbox` (Phase 3b) — concrete
// stateless structs, not trait+impl. Reach via `crate::outbox::{OutboxRepository, EventBus}`.
