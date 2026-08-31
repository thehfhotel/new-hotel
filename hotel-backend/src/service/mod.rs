//! Service layer — business logic + outbox emission + event publish.
//!
//! Per `docs/architecture.md` §1, §6 (Phase 2). Services sit between thin
//! HTTP routes and the per-aggregate repositories:
//!
//! ```text
//!     routes/  (Wave 4 — thinned)
//!         │
//!         ▼
//!     service/   ← this module
//!         │
//!         ├──▶ repository/   (PG-only data access)
//!         ├──▶ outbox/       (writeback queue + event bus)
//!         └──▶ domain/       (pure types)
//! ```
//!
//! ## Conventions every service follows
//!
//! 1. Constructor takes `Arc<dyn AggregateRepository>`,
//!    `Arc<OutboxRepository>`, `Arc<EventBus>`, and `PgPool`.
//! 2. Each public method opens a single `pg.begin()` transaction.
//! 3. Inside the TX: canonical write → outbox enqueue → event publish.
//! 4. Commit. On any error before commit, the TX rolls back and **no**
//!    side-effects (NOTIFY, writeback row) are observed by subscribers.
//! 5. Methods take **command structs** (one per public method) so the
//!    route layer can construct them from request DTOs without leaking the
//!    repository's borrowed-string write shapes upward.
//! 6. Methods return rich outcome structs containing both the SERIAL `i32`
//!    repository id and the deterministic `Uuid` aggregate id used in
//!    events / intents (see [`ids`] for the bridge).
//!
//! Wave 4 will refactor `routes/*` to delegate through these services.
//! Today they are wired into [`crate::routes::mode::AppState`] and ready to
//! call but unused by HTTP handlers — installing them now keeps the Wave 4
//! diff small + greppable.

pub mod auth;
pub mod booking;
pub mod channel;
pub mod checkin;
pub mod coupon;
pub mod customer;
pub mod error;
pub mod hk_signals;
pub mod housekeeping;
pub mod ids;
pub mod loyalty;
pub mod payment;
pub mod pos;
pub mod reader;
pub mod reports;
pub mod shifts;

pub use auth::{AuthError, AuthService, DEFAULT_SESSION_TTL};
pub use booking::{
    BookingOutcome, BookingProductCommand, BookingRoomCommand, BookingService,
    BookingSnapshotInputs, BookingWritebackContext, CancelBookingCommand, CreateBookingCommand,
    ModifyBookingCommand,
};
pub use channel::{
    ChannelService, ConfirmOutcome, CreateHoldCommand, HoldOutcome, PaymentPlan, ReleaseOutcome,
};
pub use checkin::{
    CancelCheckInCommand, ChangeRoomCommand, ChangeRoomOutcome, CheckInOutcome, CheckInService,
    CheckInToBookingCommand, CheckInWritebackContext, CheckOutCommand, ExtendStayCommand,
    RefundDepositCommand, RefundDepositOutcome, WalkInCommand,
};
pub use coupon::{
    CouponService, IssueCouponCommand, IssueCouponOutcome, RedeemCouponCommand, RedeemCouponOutcome,
};
pub use customer::{
    CreateCustomerCommand, CustomerEnrichmentInput, CustomerOutcome, CustomerService,
    UpdateCustomerCommand,
};
pub use error::{ServiceError, ServiceResult};
pub use hk_signals::{
    ActOnSignalCommand, AnswerOutcome, AnswerRoomCheckCommand, HkSignalService,
    RaiseSignalCommand, SignalOutcome,
};
pub use housekeeping::{
    HousekeepingOutcome, HousekeepingService, MarkCleanCommand, MarkDirtyCommand,
    MarkMaintenanceCommand,
};
pub use ids::{aggregate_uuid, AggregateKind};
pub use loyalty::{LoyaltyClient, LoyaltyStayPayload};
pub use payment::{
    GenerateReceiptCommand, GenerateReceiptOutcome, PaymentService, RecordPaymentCommand,
    RecordPaymentOutcome, RefundPaymentCommand, RefundPaymentOutcome,
};
pub use pos::{
    PosService, RecordSaleCommand, RecordSaleOutcome, RecordWalkupSaleCommand,
    RecordWalkupSaleOutcome, VoidSaleCommand, VoidSaleOutcome, WalkupSaleLine,
    WalkupSaleLineOutcome,
};
pub use reader::{
    find_or_provision_user_by_badge, HfIdClient, ReaderState, ReaderStore, WaitOutcome,
};
pub use shifts::{
    CloseShiftCommand, OpenShiftCommand, OpenShiftOutcome, Shift, ShiftService, ShiftSummary,
};
