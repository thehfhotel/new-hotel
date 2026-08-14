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
//! **Reception read (wave-4 B6):** [`list_cleaning_progress`]
//! (`GET /api/housekeeping/cleaning`) exposes today's maid-reported progress
//! from `ht_hk_cleaning_events` so the แผนกแม่บ้าน board can show who is in a
//! room and since when. Until it existed, that table was read by nothing
//! outside `routes::hk` — the maid reported progress and reception could not
//! see it.
//!
//! All SQL here and in the service is RUNTIME `sqlx::query` (no compile-time
//! macro), so these routes need no `.sqlx/` cache regeneration.

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::domain::user::User;
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
    // Delegate the Hfville→ville_pool decision to the unified write chokepoint.
    let pool = state.write_pool(branch)?.clone();
    Ok(HousekeepingService::new(
        state.rooms.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool,
    ))
}

/// Resolve the operator label: prefer the authenticated session user
/// (Task #40), then the body-supplied value, then [`DEFAULT_BY`]. Auth
/// ships dark, so `actor` is `None` and the body value wins exactly as
/// before; once auth is on the housekeeper's login is recorded instead.
fn resolve_by(actor: Option<&User>, by: Option<String>) -> String {
    super::resolve_actor(actor, by.as_deref()).unwrap_or_else(|| DEFAULT_BY.to_string())
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
    actor: Option<Extension<User>>,
    body: Option<Json<HousekeepingActionBody>>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let by = resolve_by(actor.as_deref(), body.and_then(|Json(b)| b.by));
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
    actor: Option<Extension<User>>,
    body: Option<Json<HousekeepingActionBody>>,
) -> ApiResult<Json<HousekeepingResponse>> {
    let svc = service_for(&state, query.branch)?;
    let by = resolve_by(actor.as_deref(), body.and_then(|Json(b)| b.by));
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

// ============================================================================
// Reception read — today's maid cleaning progress
// ============================================================================

/// One room's LATEST maid-reported cleaning event today (Thai day).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomCleaningProgress {
    pub room_id: i32,
    pub room_no: String,
    /// `started` | `done` | `dirty`.
    pub status: String,
    /// Verified HF ID badge — ALWAYS present.
    pub badge: String,
    /// Display-name snapshot; usually `null` today (the CF IdP forwards only
    /// `apps` + `badge`). Render `name ?? badge`.
    pub name: Option<String>,
    /// A real instant (`TIMESTAMPTZ`), NOT a naive legacy MSSQL datetime.
    /// Render it with normal local-time formatting — the `timeZone:'UTC'` rule
    /// applies ONLY to values mirrored from legacy (see `app/hk/hk-lib.ts`).
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningProgressResponse {
    pub success: bool,
    pub data: Vec<RoomCleaningProgress>,
}

/// GET /api/housekeeping/cleaning — today's cleaning progress, one row per room.
///
/// The reception (แผนกแม่บ้าน) board's live feed. Before this, `ht_hk_cleaning_events`
/// was read by NOTHING outside `routes::hk` — the maid's progress existed and
/// reception could not see it, and the board's middle "กำลังทำความสะอาด" column
/// was permanently empty because it keyed off a `status='cleaning'` literal that
/// `/api/rooms` never emits (pinned by `routes::new_rooms`'
/// `room_stored_cleaning_is_still_derived_available`).
///
/// Deliberately a SEPARATE endpoint rather than widening `/api/rooms`:
/// `routes::new_rooms` is a large shared file whose live-flags scan is pinned by
/// tests, and leaving it untouched is what let the backend and frontend halves
/// of this change be built in parallel.
///
/// Rooms with no event today are ABSENT from `data` (the frontend reads absent
/// as "no progress reported"), so the response stays tiny — 0 rows on a quiet
/// morning, at most one row per active room.
///
/// `?branch=` is OPTIONAL here, matching its sibling housekeeping routes:
/// reception's `useBranchFetch` always sends one, and an omitted branch means
/// the primary site exactly as it does for clean/dirty/maintenance. (The `/hk`
/// MAID surface is the opposite — there the branch is mandatory, because a maid
/// picks her own property and a wrong guess files against the wrong hotel.)
pub async fn list_cleaning_progress(
    State(state): State<AppState>,
    Query(query): Query<HousekeepingQuery>,
) -> ApiResult<Json<CleaningProgressResponse>> {
    // Same per-site chokepoint as the mutating siblings, so this read can never
    // disagree with the writes it renders.
    let pool = state.write_pool(query.branch)?;

    // `TODAY_BKK` is shared verbatim with `routes::hk` — ONE definition of
    // "today", so the maid's list and reception's board cannot disagree at a
    // day boundary. INNER JOIN LATERAL ⇒ only rooms with an event today.
    let sql = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            ev.hkev_status,
            ev.hkev_badge,
            ev.hkev_name,
            ev.hkev_created_at
        FROM ht_rooms_new r
        JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {today}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
        WHERE COALESCE(r.room_active, true) = true
        ORDER BY r.room_no
        "#,
        today = super::hk::TODAY_BKK
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .fetch_all(pool)
        .await?;

    let data = rows
        .iter()
        .map(|row| RoomCleaningProgress {
            room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
            room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
            status: row.try_get::<String, _>("hkev_status").unwrap_or_default(),
            badge: row.try_get::<String, _>("hkev_badge").unwrap_or_default(),
            name: row.try_get::<String, _>("hkev_name").ok(),
            at: row
                .try_get::<DateTime<Utc>, _>("hkev_created_at")
                .unwrap_or_else(|_| Utc::now()),
        })
        .collect();

    Ok(Json(CleaningProgressResponse {
        success: true,
        data,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_by_defaults_blank_and_missing() {
        // Auth off (`actor = None`): body value wins, blank/missing → sentinel.
        assert_eq!(resolve_by(None, None), DEFAULT_BY);
        assert_eq!(resolve_by(None, Some("   ".to_string())), DEFAULT_BY);
        assert_eq!(resolve_by(None, Some("Nok".to_string())), "Nok");
        assert_eq!(resolve_by(None, Some("  Nok  ".to_string())), "Nok");
    }

    #[test]
    fn resolve_by_prefers_authenticated_user() {
        use crate::domain::user::{Role, User};
        use chrono::NaiveDate;
        let u = User {
            user_id: 9,
            username: "housekeeper_a".to_string(),
            password_hash: String::new(),
            role: Role::Housekeeper,
            active: true,
            created_at: NaiveDate::from_ymd_opt(2026, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            last_login_at: None,
            email: None,
        };
        // Authenticated user overrides the body value.
        assert_eq!(resolve_by(Some(&u), Some("Nok".to_string())), "housekeeper_a");
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

    /// The reception feed must carry the LATEST event per room and must OMIT
    /// rooms with no event today (absent = "no progress reported"). DB-backed;
    /// skips gracefully without a local PG, same `try_pool` convention as
    /// `service::housekeeping::tests`.
    #[tokio::test]
    async fn cleaning_feed_returns_latest_event_per_room_and_omits_quiet_rooms() {
        use sqlx::PgPool;

        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        let Ok(pool) = PgPool::connect(&url).await else {
            eprintln!("skipping cleaning_feed_returns_latest_event_per_room — PG not reachable");
            return;
        };

        for marker in ["ZT-HKF1", "ZT-HKF2"] {
            let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
                .bind(marker)
                .execute(&pool)
                .await;
        }
        // ZT-HKF1 gets two events today; ZT-HKF2 stays quiet.
        let reported: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-HKF1', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed reported room");
        let quiet: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-HKF2', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed quiet room");

        for status in ["started", "done"] {
            sqlx::query(
                "INSERT INTO ht_hk_cleaning_events \
                     (hkev_room_id, hkev_status, hkev_badge, hkev_name) \
                 VALUES ($1, $2, 'Q1001', 'นก')",
            )
            .bind(reported)
            .bind(status)
            .execute(&pool)
            .await
            .expect("seed cleaning event");
        }

        let state = AppState::new(pool.clone());
        let Json(body) = list_cleaning_progress(
            axum::extract::State(state),
            axum::extract::Query(HousekeepingQuery { branch: None }),
        )
        .await
        .expect("cleaning feed must answer");

        assert!(body.success);
        let row = body
            .data
            .iter()
            .find(|r| r.room_id == reported)
            .expect("the room with events today must appear");
        assert_eq!(row.status, "done", "the LATEST event wins");
        assert_eq!(row.badge, "Q1001");
        assert_eq!(row.name.as_deref(), Some("นก"));
        assert_eq!(row.room_no, "ZT-HKF1");
        assert!(
            !body.data.iter().any(|r| r.room_id == quiet),
            "a room with no event today must be ABSENT, not present with a null status"
        );

        for id in [reported, quiet] {
            let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
                .bind(id)
                .execute(&pool)
                .await;
        }
    }
}
