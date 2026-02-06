//! System mode API route
//!
//! - GET /api/mode - Returns current system mode (legacy or new)

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::ApiResult;

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

/// Application state for dual-database routes
#[derive(Clone)]
pub struct AppState {
    /// Connection pool for legacy database (SQL Server via tiberius)
    pub legacy_pool: crate::db::DbPool,
    /// Connection pool for new_hotel database (PostgreSQL via sqlx)
    pub new_pool: crate::db::PgPool,
    /// Current system operating mode
    pub mode: Arc<std::sync::RwLock<SystemMode>>,
}

impl AppState {
    /// Create new AppState with both pools and default legacy mode
    pub fn new(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool) -> Self {
        Self {
            legacy_pool,
            new_pool,
            mode: Arc::new(std::sync::RwLock::new(SystemMode::Legacy)),
        }
    }

    /// Create new AppState with specified mode
    pub fn with_mode(legacy_pool: crate::db::DbPool, new_pool: crate::db::PgPool, mode: SystemMode) -> Self {
        Self {
            legacy_pool,
            new_pool,
            mode: Arc::new(std::sync::RwLock::new(mode)),
        }
    }

    /// Get current mode
    pub fn current_mode(&self) -> SystemMode {
        *self.mode.read().unwrap()
    }

    /// Set mode
    pub fn set_mode(&self, mode: SystemMode) {
        *self.mode.write().unwrap() = mode;
    }
}

/// Mode response
#[derive(Debug, Serialize)]
pub struct ModeResponse {
    pub success: bool,
    pub mode: SystemMode,
}

/// GET /api/mode - Returns current system mode
pub async fn get_mode(State(state): State<AppState>) -> ApiResult<Json<ModeResponse>> {
    let mode = state.current_mode();

    Ok(Json(ModeResponse {
        success: true,
        mode,
    }))
}
