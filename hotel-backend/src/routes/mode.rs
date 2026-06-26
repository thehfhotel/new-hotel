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
    PaymentService, PosService, ShiftService,
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
    /// Track G6 — POS / sales-to-room service. Stateless once
    /// constructed; reads `ht_products` + `ht_checkins`, writes
    /// `ht_pos_sales` + outbox intent.
    pos: Arc<PosService>,
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
    // NOTE: deliberately NO legacy MSSQL pool here. Routes/repositories never
    // touch the legacy DB (docs/architecture.md "critical rule"); MSSQL is
    // reserved for the adapter workers (sync/writeback bins) and the
    // scheduler's reconcile backstop, which get their own pool in main.rs.
    // The former `legacy_pool` field was held-but-never-queried — removed by
    // the 2026-06-11 coexistence audit to make the boundary structural.
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
    /// POS service — orchestrates `ht_pos_sales` writes + stock
    /// decrement + outbox `RecordPosSale` intent. Track G6.
    pub pos_service: Arc<PosService>,

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

    /// Create new AppState with the canonical PG pool and default legacy mode.
    ///
    /// Auth is wired with PG-backed repositories but disabled by
    /// default (`auth_enabled = false`). Callers that want auth on
    /// chain `.with_auth_enabled(true)` after construction.
    pub fn new(new_pool: crate::db::PgPool) -> Self {
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
            pos_service: services.pos,
            auth_service: crate::routes::auth::build_auth_service(),
            auth_enabled: false,
        }
    }

    /// Create new AppState with specified mode.
    ///
    /// Auth is wired but disabled by default — see [`Self::new`] for
    /// the full rationale. Use [`Self::with_auth_enabled`] to flip the
    /// middleware on.
    pub fn with_mode(new_pool: crate::db::PgPool, mode: SystemMode) -> Self {
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
            pos_service: services.pos,
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
            coupons: Arc::new(CouponService::new(outbox.clone(), events.clone(), pg.clone())),
            // Track G6 — POS / sales-to-room service.
            pos: Arc::new(PosService::new(outbox, events, pg)),
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

/// HF Ville connectivity report.
///
/// `/api/mode`'s `villeAvailable` only reflects whether the pool was built at
/// *startup* (`main.rs` fail-soft: a Ville DB that was unreachable when the
/// backend last booted leaves `ville_pool = None` until the next restart, which
/// is exactly the "HF Ville is not loading" symptom). This struct backs an
/// endpoint that ACTIVELY pings the Ville DB so a post-startup outage — or a
/// recovery that needs a restart — is detectable by monitors and post-deploy
/// smoke checks.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VilleHealth {
    /// The pool exists: `VILLE_DB_ENABLED=true` AND the startup connect succeeded.
    pub enabled: bool,
    /// A live `SELECT 1` against the Ville pool just succeeded.
    pub connected: bool,
    /// Human-readable cause when not connected (omitted when healthy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Active connectivity probe, split out from the handler so the pool-missing
/// branch is unit-testable without a database. The `Some` branch is exercised
/// by the post-deploy smoke check (`scripts/smoke-ville.sh`).
pub async fn ville_health_status(pool: Option<&crate::db::PgPool>) -> VilleHealth {
    match pool {
        None => VilleHealth {
            enabled: false,
            connected: false,
            detail: Some(
                "HF Ville pool not initialized — VILLE_DB_ENABLED is off, or the connection \
                 failed at backend startup (check VILLE_DB_* and the ville tunnel, then restart \
                 the backend)"
                    .to_string(),
            ),
        },
        Some(pool) => match sqlx::query("SELECT 1").execute(pool).await {
            Ok(_) => VilleHealth {
                enabled: true,
                connected: true,
                detail: None,
            },
            Err(e) => VilleHealth {
                enabled: true,
                connected: false,
                detail: Some(format!("HF Ville pool exists but SELECT 1 failed: {e}")),
            },
        },
    }
}

/// GET /api/health/ville — active HF Ville connectivity probe.
pub async fn get_ville_health(State(state): State<AppState>) -> Json<VilleHealth> {
    Json(ville_health_status(state.ville_pool.as_ref()).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The frontend (BranchContext) reads `data.villeAvailable` (camelCase).
    /// If this serialization ever regressed to snake_case, HF Ville would
    /// silently stay disabled in BOTH UIs even with a healthy Ville DB — the
    /// exact "HF Ville is not loading" class. Lock the wire contract here.
    #[test]
    fn mode_response_serializes_ville_available_as_camel_case() {
        let resp = ModeResponse {
            success: true,
            mode: SystemMode::New,
            ville_available: true,
        };
        let v = serde_json::to_value(&resp).unwrap();
        assert_eq!(
            v.get("villeAvailable").and_then(|x| x.as_bool()),
            Some(true),
            "frontend depends on camelCase `villeAvailable`"
        );
        assert!(
            v.get("ville_available").is_none(),
            "snake_case must NOT be emitted — it would break the frontend gate"
        );
    }

    /// With no Ville pool (disabled, or startup connect failed), the probe must
    /// report enabled=false, connected=false, and a non-empty diagnostic.
    #[tokio::test]
    async fn ville_health_reports_unavailable_when_pool_missing() {
        let health = ville_health_status(None).await;
        assert!(!health.enabled);
        assert!(!health.connected);
        assert!(
            health.detail.as_deref().unwrap_or("").contains("VILLE_DB_ENABLED"),
            "diagnostic should point the operator at the likely cause"
        );
    }

    /// Connectivity report serializes camelCase and omits `detail` when healthy
    /// so monitors can key on `connected`.
    #[test]
    fn ville_health_serializes_camel_case_and_omits_detail_when_healthy() {
        let healthy = VilleHealth {
            enabled: true,
            connected: true,
            detail: None,
        };
        let v = serde_json::to_value(&healthy).unwrap();
        assert_eq!(v.get("connected").and_then(|x| x.as_bool()), Some(true));
        assert_eq!(v.get("enabled").and_then(|x| x.as_bool()), Some(true));
        assert!(v.get("detail").is_none(), "detail omitted when healthy");

        let down = VilleHealth {
            enabled: true,
            connected: false,
            detail: Some("SELECT 1 failed".to_string()),
        };
        let v = serde_json::to_value(&down).unwrap();
        assert_eq!(v.get("connected").and_then(|x| x.as_bool()), Some(false));
        assert_eq!(v.get("detail").and_then(|x| x.as_str()), Some("SELECT 1 failed"));
    }
}
