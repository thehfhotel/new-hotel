//! System mode API route
//!
//! - GET /api/mode - Returns current system mode (legacy or new)

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::{ApiError, ApiResult};
use crate::outbox::{EventBus, OutboxRepository};

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

/// Application state for dual-database routes
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
    /// Outbox publisher for legacy MSSQL writebacks (Phase 3b — `architecture.md` §3.6c).
    /// Stateless; wrapped in `Arc` for cheap clone across handlers.
    pub outbox: Arc<OutboxRepository>,
    /// Domain-event bus publisher (Phase 3b — `architecture.md` §3.6c).
    /// Stateless; wrapped in `Arc` for cheap clone across handlers.
    pub events: Arc<EventBus>,
}

impl AppState {
    /// Create new AppState with both pools and default legacy mode
    pub fn new(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool) -> Self {
        Self {
            legacy_pool,
            new_pool,
            ville_pool: None,
            mode: Arc::new(std::sync::RwLock::new(SystemMode::Legacy)),
            outbox: Arc::new(OutboxRepository::new()),
            events: Arc::new(EventBus::new()),
        }
    }

    /// Create new AppState with specified mode
    pub fn with_mode(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool, mode: SystemMode) -> Self {
        Self {
            legacy_pool,
            new_pool,
            ville_pool: None,
            mode: Arc::new(std::sync::RwLock::new(mode)),
            outbox: Arc::new(OutboxRepository::new()),
            events: Arc::new(EventBus::new()),
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
