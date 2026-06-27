//! Housekeeping API routes — clean / dirty / maintenance toggles.
//!
//! Thin wrappers over [`crate::service::HousekeepingService`], which owns the
//! canonical `ht_rooms_new.room_clean` / `room_maintenance` flips, the legacy
//! writeback enqueue (`MarkRoomClean` / `MarkRoomDirty` / `SetRoomMaintenance`
//! — spike §3j + coexistence audit 2026-06-11 P2), and the domain-event
//! publish. Until this module the service was wired into `AppState` but
//! **unrouted**: the housekeeping board's "mark dirty" / "mark clean" actions
//! PATCHed `/api/rooms/:id/status` with a string that never touched
//! `room_clean` and never reached iHOTEL (a no-op), and the out-of-service
//! toggle likewise never mirrored to legacy `HT_Rooms.Room_Manternace`, so the
//! legacy grid kept renting a room taken out of service in the new app.
//!
//! **Branch-aware:** `?branch=hfville` targets the `hotelville` canonical pool,
//! everything else the primary HF Hotel pool — mirroring `routes::new_shifts`.
//! A fresh `HousekeepingService` is built per request bound to the resolved
//! pool (the AppState-wired instance is hardwired to the primary pool at
//! startup); construction is cheap (Arc clones + a pool handle clone). HF Ville
//! mutations are still gated by the `ville_write_guard` middleware in `main.rs`
//! until `HFVILLE_WRITES_ENABLED` is flipped — this module relies on that
//! existing safety net rather than re-checking the flag.
//!
//! All SQL the service runs is RUNTIME `sqlx::query` (no compile-time macro),
//! so adding these routes needs no `.sqlx/` cache regeneration.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::ApiResult;
use crate::outbox::event::EventSource;
use crate::service::{
    HousekeepingService, MarkCleanCommand, MarkDirtyCommand, MarkMaintenanceCommand,
};

/// Operator label stamped into the legacy `HT_Housewife` audit row when the
/// caller doesn't supply one. Auth context isn't wired into these routes yet
/// (`AUTH_ENABLED` is off in production), so we fall back to a stable sentinel
/// rather than tripping the service's non-empty `by` validation.
const DEFAULT_BY: &str = "Front Desk";

/// Branch selector shared by every housekeeping route.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HousekeepingQuery {
    pub branch: Option<Branch>,
}

/// Body for the clean / dirty actions. `by` lands in the legacy `HT_Housewife`
/// audit row (the housekeeper / operator). Optional — defaults to
/// [`DEFAULT_BY`] when absent or blank.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct HousekeepingActionBody {
    pub by: Option<String>,
}

/// Body for the maintenance toggle.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceBody {
    /// `true` ⇒ take the room out of service (legacy `Room_Manternace='yes'`);
    /// `false` ⇒ return it to service.
    pub maintenance: bool,
}

/// Uniform success envelope for the housekeeping mutations.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HousekeepingResponse {
    pub success: bool,
    pub message: String,
    pub room_id: i32,
}

/// Build a [`HousekeepingService`] bound to the branch's pool. Reuses the
/// AppState repository / outbox / event-bus handles (cheap Arc clones) and the
/// resolved pool handle — same construction shape as `AppState::wire_services`.
fn service_for(state: &AppState, branch: Option<Branch>) -> ApiResult<HousekeepingService> {
    let pool = match branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?.clone(),
        Branch::Hfhotel | Branch::All => state.new_pool.clone(),
    };
    Ok(HousekeepingService::new(
        state.rooms.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool,
    ))
}

/// Resolve the operator label, defaulting blank/missing input to [`DEFAULT_BY`].
fn resolve_by(by: Option<String>) -> String {
    by.map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_BY.to_string())
}

/// `EventSource` for an HTTP-originated housekeeping mutation. Mirrors
/// `routes::new_bookings` — a real `user_id` lands here once auth is wired.
fn http_source() -> EventSource {
    EventSource::our_app(Uuid::nil(), Uuid::new_v4())
}

/// POST /api/housekeeping/rooms/{id}/clean — mark a room clean.
pub async fn mark_clean(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HousekeepingQuery>,
    body: Option<Json<HousekeepingActionBody>>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let by = resolve_by(body.and_then(|Json(b)| b.by));
    let outcome = svc
        .mark_clean(MarkCleanCommand {
            room_id,
            by,
            source: http_source(),
        })
        .await?;
    Ok(Json(HousekeepingResponse {
        success: true,
        message: "Room marked clean".to_string(),
        room_id: outcome.room_id,
    }))
}

/// POST /api/housekeeping/rooms/{id}/dirty — mark a room dirty.
pub async fn mark_dirty(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HousekeepingQuery>,
    body: Option<Json<HousekeepingActionBody>>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let by = resolve_by(body.and_then(|Json(b)| b.by));
    let outcome = svc
        .mark_dirty(MarkDirtyCommand {
            room_id,
            by,
            source: http_source(),
        })
        .await?;
    Ok(Json(HousekeepingResponse {
        success: true,
        message: "Room marked dirty".to_string(),
        room_id: outcome.room_id,
    }))
}

/// POST /api/housekeeping/rooms/{id}/maintenance — toggle out-of-service.
pub async fn set_maintenance(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HousekeepingQuery>,
    Json(body): Json<MaintenanceBody>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let outcome = svc
        .set_maintenance(MarkMaintenanceCommand {
            room_id,
            maintenance: body.maintenance,
            source: http_source(),
        })
        .await?;
    let message = if body.maintenance {
        "Room sent to maintenance".to_string()
    } else {
        "Room returned to service".to_string()
    };
    Ok(Json(HousekeepingResponse {
        success: true,
        message,
        room_id: outcome.room_id,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_by_defaults_blank_and_missing() {
        assert_eq!(resolve_by(None), DEFAULT_BY);
        assert_eq!(resolve_by(Some("   ".to_string())), DEFAULT_BY);
        assert_eq!(resolve_by(Some("Nok".to_string())), "Nok");
        assert_eq!(resolve_by(Some("  Nok  ".to_string())), "Nok");
    }

    /// The clean/dirty body must accept an empty object (`{}`) — the frontend
    /// may omit `by` — so serde `default` keeps the `by` field optional.
    #[test]
    fn action_body_accepts_empty_object() {
        let body: HousekeepingActionBody = serde_json::from_str("{}").unwrap();
        assert!(body.by.is_none());
        let body: HousekeepingActionBody =
            serde_json::from_str(r#"{"by":"Nok"}"#).unwrap();
        assert_eq!(body.by.as_deref(), Some("Nok"));
    }

    /// Maintenance body round-trips the boolean in both directions.
    #[test]
    fn maintenance_body_parses_bool() {
        let on: MaintenanceBody = serde_json::from_str(r#"{"maintenance":true}"#).unwrap();
        assert!(on.maintenance);
        let off: MaintenanceBody = serde_json::from_str(r#"{"maintenance":false}"#).unwrap();
        assert!(!off.maintenance);
    }
}
