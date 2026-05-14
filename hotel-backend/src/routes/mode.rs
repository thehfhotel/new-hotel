//! System mode API route + shared `AppState`.
//!
//! - GET /api/mode - Returns current system mode (legacy or new)
//!
//! `AppState` carries:
//! - the two database pools (legacy SQL Server + new PostgreSQL),
//! - the optional HF Ville mirror pool,
//! - one trait-object handle per repository so routes call `state.customers.get(...)`
//!   instead of inline `sqlx::query!()` (per `docs/architecture.md` §1, §6).

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::{ApiError, ApiResult};
use crate::outbox::{EventBus, OutboxRepository};
use crate::repository::{
    BookingRepository, CheckInRepository, CustomerRepository, InventoryRepository,
    PaymentRepository, PgBookingRepository, PgCheckInRepository, PgCustomerRepository,
    PgInventoryRepository, PgPaymentRepository, PgRoomRepository, RoomRepository,
};
use crate::routes::auth::ProdAuthService;
use crate::config::SiteConfig;
use crate::service::{
    BookingService, CheckInService, CouponService, CustomerService, HousekeepingService,
    PaymentService, ShiftService,
};

/// Bundle of fully-wired Phase-2 services produced by `AppState::wire_services`.
///
/// Returned by the helper so the two public constructors (`new` and
/// `with_mode`) can spread the fields into [`AppState`] without repeating the
/// service graph wiring twice.
struct WiredServices {
    customers: Arc<CustomerService>,
    bookings: Arc<BookingService>,
    checkins: Arc<CheckInService>,
    payments: Arc<PaymentService>,
    housekeeping: Arc<HousekeepingService>,
    /// Track F2 / T1 HIGH-5 — shift service bound to this binary's
    /// site (read from `SITE_ID` at startup). Wired into
    /// `PaymentService::with_shifts` so `record_payment` refuses to
    /// insert unless an `ht_shifts` row is open.
    shifts: Arc<ShiftService>,
    /// Track G5 — coupon issuing canonical service.
    coupons: Arc<CouponService>,
}

/// System operating mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SystemMode {
    /// Legacy mode - use only the legacy database
    #[default]
    Legacy,
    /// New mode - use the new_hotel database (hybrid reads from both)
    New,
}

impl SystemMode {
    /// Parse mode from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "new" => SystemMode::New,
            _ => SystemMode::Legacy,
        }
    }
}

/// Hotel branch selector for multi-branch support
#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Branch {
    /// HF Hotel (default) - main branch
    #[default]
    Hfhotel,
    /// HF Ville - สุราษฎร์ธานี branch
    Hfville,
    /// All branches combined
    All,
}

/// Application state for dual-database routes.
///
/// Repositories are stored as `Arc<dyn ...Repository>` so test setups can swap
/// them for in-memory fakes without touching route code (per
/// `docs/architecture.md` §1).
#[derive(Clone)]
pub struct AppState {
    /// Connection pool for legacy database (SQL Server via tiberius)
    pub legacy_pool: crate::db::DbPool,
    /// Connection pool for new_hotel database (PostgreSQL via sqlx)
    pub new_pool: crate::db::PgPool,
    /// Connection pool for HF Ville mirror database (PostgreSQL via sqlx, optional)
    pub ville_pool: Option<crate::db::PgPool>,
    /// Current system operating mode
    pub mode: Arc<std::sync::RwLock<SystemMode>>,

    // ----- Repository handles (per architecture.md §1, §6) -----
    pub customers: Arc<dyn CustomerRepository>,
    pub bookings: Arc<dyn BookingRepository>,
    pub checkins: Arc<dyn CheckInRepository>,
    pub rooms: Arc<dyn RoomRepository>,
    pub payments: Arc<dyn PaymentRepository>,
    pub inventory: Arc<dyn InventoryRepository>,

    // ----- Outbox + event bus (per architecture.md §3.6c) -----
    /// Outbox publisher for legacy MSSQL writebacks. Stateless; cheap Arc clone.
    pub outbox: Arc<OutboxRepository>,
    /// Domain-event bus (event_log + pg_notify). Stateless; cheap Arc clone.
    pub events: Arc<EventBus>,

    // ----- Service layer handles (per architecture.md §1, §6 — Phase 2) -----
    /// Customer service — orchestrates `ht_customers` writes + outbox + events.
    pub customers_service: Arc<CustomerService>,
    /// Booking service — orchestrates `ht_bookings` writes + outbox + events.
    pub bookings_service: Arc<BookingService>,
    /// Check-in service — orchestrates `ht_checkins` writes + outbox + events.
    pub checkins_service: Arc<CheckInService>,
    /// Payment service — orchestrates `ht_payments` writes + outbox + events.
    pub payments_service: Arc<PaymentService>,
    /// Housekeeping service — orchestrates room cleanliness flips + events.
    pub housekeeping_service: Arc<HousekeepingService>,
    /// Shift service — owns `ht_shifts` open/close/lookup. Track F2 /
    /// T1 HIGH-5. Bound to this binary's `SITE_ID` so per-site cashier
    /// rounds are isolated (`hfhotel` and `hfville` each have their
    /// own running counter).
    pub shifts_service: Arc<ShiftService>,
    /// Coupon service — Track G5 (`ht_coupons`). Orchestrates issue +
    /// redeem against the canonical table plus outbox enqueues so the
    /// writeback worker can mirror onto legacy `HT_Cupon`.
    pub coupons_service: Arc<CouponService>,

    // ----- Auth (Phase 4 PR2) -----
    /// Cookie-session auth service — wired with PG-backed user + session
    /// repositories. Cheap to clone (Arcs all the way down). Reachable
    /// from both the `/api/auth/*` route handlers and the
    /// `middleware::auth::require_auth` layer wrapping `/api/new/*`.
    pub auth_service: Arc<ProdAuthService>,
    /// Master switch for the auth middleware. Read from `AUTH_ENABLED`
    /// in `main.rs`; defaults to `false` so production stays exactly
    /// as-is until an operator provisions an admin user and flips the
    /// flag. The `/api/auth/*` endpoints REMAIN reachable in either
    /// state — only the gate on `/api/new/*` is toggled.
    pub auth_enabled: bool,
}

impl AppState {
    /// Build the default repository wiring (PostgreSQL impls of every aggregate).
    fn default_repositories() -> (
        Arc<dyn CustomerRepository>,
        Arc<dyn BookingRepository>,
        Arc<dyn CheckInRepository>,
        Arc<dyn RoomRepository>,
        Arc<dyn PaymentRepository>,
        Arc<dyn InventoryRepository>,
    ) {
        (
            Arc::new(PgCustomerRepository::new()),
            Arc::new(PgBookingRepository::new()),
            Arc::new(PgCheckInRepository::new()),
            Arc::new(PgRoomRepository::new()),
            Arc::new(PgPaymentRepository::new()),
            Arc::new(PgInventoryRepository::new()),
        )
    }

    /// Create new AppState with both pools and default legacy mode.
    ///
    /// Auth is wired with PG-backed repositories but disabled by
    /// default (`auth_enabled = false`). Callers that want auth on
    /// chain `.with_auth_enabled(true)` after construction.
    pub fn new(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool) -> Self {
        let (customers, bookings, checkins, rooms, payments, inventory) =
            Self::default_repositories();
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        let services = Self::wire_services(
            customers.clone(),
            bookings.clone(),
            checkins.clone(),
            rooms.clone(),
            payments.clone(),
            outbox.clone(),
            events.clone(),
            new_pool.clone(),
        );
        Self {
            legacy_pool,
            new_pool,
            ville_pool: None,
            mode: Arc::new(std::sync::RwLock::new(SystemMode::Legacy)),
            customers,
            bookings,
            checkins,
            rooms,
            payments,
            inventory,
            outbox,
            events,
            customers_service: services.customers,
            bookings_service: services.bookings,
            checkins_service: services.checkins,
            payments_service: services.payments,
            housekeeping_service: services.housekeeping,
            shifts_service: services.shifts,
            coupons_service: services.coupons,
            auth_service: crate::routes::auth::build_auth_service(),
            auth_enabled: false,
        }
    }

    /// Create new AppState with specified mode.
    ///
    /// Auth is wired but disabled by default — see [`Self::new`] for
    /// the full rationale. Use [`Self::with_auth_enabled`] to flip the
    /// middleware on.
    pub fn with_mode(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool, mode: SystemMode) -> Self {
        let (customers, bookings, checkins, rooms, payments, inventory) =
            Self::default_repositories();
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        let services = Self::wire_services(
            customers.clone(),
            bookings.clone(),
            checkins.clone(),
            rooms.clone(),
            payments.clone(),
            outbox.clone(),
            events.clone(),
            new_pool.clone(),
        );
        Self {
            legacy_pool,
            new_pool,
            ville_pool: None,
            mode: Arc::new(std::sync::RwLock::new(mode)),
            customers,
            bookings,
            checkins,
            rooms,
            payments,
            inventory,
            outbox,
            events,
            customers_service: services.customers,
            bookings_service: services.bookings,
            checkins_service: services.checkins,
            payments_service: services.payments,
            housekeeping_service: services.housekeeping,
            shifts_service: services.shifts,
            coupons_service: services.coupons,
            auth_service: crate::routes::auth::build_auth_service(),
            auth_enabled: false,
        }
    }

    /// Builder-style toggle for the auth middleware. Called from
    /// `main.rs` once it has read `AUTH_ENABLED` from the environment.
    /// Does NOT affect the `/api/auth/*` endpoints — those stay reachable
    /// regardless so the frontend can probe the auth state.
    pub fn with_auth_enabled(mut self, enabled: bool) -> Self {
        self.auth_enabled = enabled;
        self
    }

    /// Wire all Phase-2 services from the shared repository / outbox /
    /// event-bus handles. Kept as a single helper so both `new()` and
    /// `with_mode()` produce identical service graphs.
    ///
    /// Track F2 / T1 HIGH-5: this is also where `ShiftService` is bound
    /// to the binary's `SITE_ID` and threaded into
    /// `PaymentService::with_shifts(...)` so `record_payment` refuses
    /// the cash-drawer write when no shift is open.
    #[allow(clippy::too_many_arguments)]
    fn wire_services(
        customers: Arc<dyn CustomerRepository>,
        bookings: Arc<dyn BookingRepository>,
        checkins: Arc<dyn CheckInRepository>,
        rooms: Arc<dyn RoomRepository>,
        payments: Arc<dyn PaymentRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: crate::db::PgPool,
    ) -> WiredServices {
        let site_id = SiteConfig::from_env().id;
        let shifts = Arc::new(ShiftService::new(pg.clone(), site_id));
        WiredServices {
            customers: Arc::new(CustomerService::new(
                customers,
                outbox.clone(),
                events.clone(),
                pg.clone(),
            )),
            bookings: Arc::new(BookingService::new(
                bookings,
                outbox.clone(),
                events.clone(),
                pg.clone(),
            )),
            checkins: Arc::new(
                CheckInService::new(
                    checkins,
                    outbox.clone(),
                    events.clone(),
                    pg.clone(),
                )
                // Track G9 / T4 HIGH-8 — wire the same `ShiftService`
                // instance the payment gate uses so `check_out`
                // (round-bill fold) refuses to run unless a shift is
                // open AND can stamp the resolved `shift_id` onto
                // `ht_checkins.cin_round_bill_shift_id` for per-shift
                // revenue attribution.
                .with_shifts(shifts.clone()),
            ),
            payments: Arc::new(
                PaymentService::new(
                    payments,
                    outbox.clone(),
                    events.clone(),
                    pg.clone(),
                )
                .with_shifts(shifts.clone()),
            ),
            housekeeping: Arc::new(HousekeepingService::new(
                rooms,
                outbox.clone(),
                events.clone(),
                pg.clone(),
            )),
            shifts,
            coupons: Arc::new(CouponService::new(outbox, events, pg)),
        }
    }

    /// Create new AppState with ville pool
    pub fn with_ville(mut self, ville_pool: crate::db::PgPool) -> Self {
        self.ville_pool = Some(ville_pool);
        self
    }

    /// Get current mode
    pub fn current_mode(&self) -> SystemMode {
        *self.mode.read().unwrap()
    }

    /// Set mode
    pub fn set_mode(&self, mode: SystemMode) {
        *self.mode.write().unwrap() = mode;
    }

    /// Get ville pool or return error
    pub fn ville_pool(&self) -> ApiResult<&crate::db::PgPool> {
        self.ville_pool
            .as_ref()
            .ok_or_else(|| ApiError::Internal("HF Ville database is not available".to_string()))
    }
}

/// Mode response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeResponse {
    pub success: bool,
    pub mode: SystemMode,
    pub ville_available: bool,
}

/// GET /api/mode - Returns current system mode
pub async fn get_mode(State(state): State<AppState>) -> ApiResult<Json<ModeResponse>> {
    let mode = state.current_mode();

    Ok(Json(ModeResponse {
        success: true,
        mode,
        ville_available: state.ville_pool.is_some(),
    }))
}
