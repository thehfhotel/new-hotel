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
use crate::repository::{
    BookingRepository, CheckInRepository, CustomerRepository, EventLogRepository,
    InventoryRepository, OutboxRepository, PaymentRepository, PgBookingRepository,
    PgCheckInRepository, PgCustomerRepository, PgEventLogRepository, PgInventoryRepository,
    PgOutboxRepository, PgPaymentRepository, PgRoomRepository, RoomRepository,
};

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
    /// Stub today; Agent D ships the real impl in parallel (Phase 1b).
    pub outbox: Arc<dyn OutboxRepository>,
    /// Stub today; Agent D ships the real impl in parallel (Phase 1b).
    pub events: Arc<dyn EventLogRepository>,
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
        Arc<dyn OutboxRepository>,
        Arc<dyn EventLogRepository>,
    ) {
        (
            Arc::new(PgCustomerRepository::new()),
            Arc::new(PgBookingRepository::new()),
            Arc::new(PgCheckInRepository::new()),
            Arc::new(PgRoomRepository::new()),
            Arc::new(PgPaymentRepository::new()),
            Arc::new(PgInventoryRepository::new()),
            Arc::new(PgOutboxRepository::new()),
            Arc::new(PgEventLogRepository::new()),
        )
    }

    /// Create new AppState with both pools and default legacy mode
    pub fn new(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool) -> Self {
        let (customers, bookings, checkins, rooms, payments, inventory, outbox, events) =
            Self::default_repositories();
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
        }
    }

    /// Create new AppState with specified mode
    pub fn with_mode(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool, mode: SystemMode) -> Self {
        let (customers, bookings, checkins, rooms, payments, inventory, outbox, events) =
            Self::default_repositories();
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
