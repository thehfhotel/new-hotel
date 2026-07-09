//! Maid-facing housekeeping surface (`/api/hk/*`) — employee-login plan
//! Phase 4 (HF-erp `docs/employee-login-plan.md`).
//!
//! Serves the mobile `/hk` pages maids open from their LINE Role Menu:
//!
//! - `GET  /api/hk/me`                          — the verified maid identity.
//! - `GET  /api/hk/rooms`                       — room list + today's progress.
//! - `GET  /api/hk/rooms/{id}`                  — one room + events + reports.
//! - `POST /api/hk/rooms/{id}/cleaning`         — report progress (`started`/`done`).
//! - `POST /api/hk/rooms/{id}/broken-items`     — report a broken/damaged item.
//! - `GET  /api/hk/broken-items/{id}/photo`     — stream a report's photo.
//!
//! ## Identity & auth
//!
//! The router is wrapped by [`crate::middleware::hk_access::require_hk_access`]
//! (its own Cloudflare Access application, HF ID silent login, grant key
//! `housekeeping`) which injects the verified [`HkIdentity`] — handlers NEVER
//! trust client-supplied reporter fields; the badge/name stamped into rows
//! come exclusively from the verified assertion. Fail closed: no valid
//! identity ⇒ the middleware already answered 401/403.
//!
//! ## Coexistence stance (ADR 0002 / invariant #6)
//!
//! Everything here is PG-canonical-only NEW data (`ht_hk_cleaning_events`,
//! `ht_hk_broken_reports`, migration 077): NO legacy writeback, NO sync
//! mapper, and deliberately NO write to `ht_rooms_new.room_clean` (that flag
//! belongs to the front-desk board + its legacy mirror via
//! `service::housekeeping`). A maid's progress report cannot touch iHOTEL.
//!
//! Branch-aware via `?branch=` resolved through the unified
//! [`AppState::write_pool`] chokepoint (Ship-B gate); `branch=hfville`
//! mutations stay blocked by the `ville_write_guard` layer until
//! `HFVILLE_WRITES_ENABLED` flips — v1 of the frontend pins HF Hotel.
//!
//! All SQL is RUNTIME `sqlx::query` (no compile-time macro), so this module
//! needs no `.sqlx/` cache regeneration — same policy as
//! `routes::guest_documents` / `routes::new_maintenance`.

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Extension, Json,
};
use base64::Engine as _;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};
use crate::middleware::hk_access::HkIdentity;

/// Cleaning-progress statuses a maid can report. `started` =
/// เริ่มทำความสะอาด, `done` = เสร็จแล้ว. Matches the CHECK constraint on
/// `ht_hk_cleaning_events.hkev_status` (migration 077).
pub const VALID_CLEANING_STATUSES: [&str; 2] = ["started", "done"];

/// Upper bound on a decoded broken-item photo (bytes). Phones downscale
/// client-side before upload; this is the server-side backstop.
const MAX_PHOTO_BYTES: usize = 5 * 1024 * 1024;

/// Upper bound on a broken-item description (chars).
const MAX_DESCRIPTION_CHARS: usize = 4000;

/// Branch selector shared by every hk route (`?branch=`; absent ⇒ HF Hotel).
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HkBranchQuery {
    pub branch: Option<Branch>,
}

/// Resolve the per-site canonical pool via the unified write chokepoint —
/// same idiom as `routes::new_maintenance::resolve_pool`.
fn resolve_pool(state: &AppState, branch: Option<Branch>) -> ApiResult<&PgPool> {
    state.write_pool(branch)
}

// ============================================================================
// Response types
// ============================================================================

/// `GET /api/hk/me` — the verified identity, echoed for the header bar.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub success: bool,
    pub badge: String,
    pub display_name: Option<String>,
}

/// Today's latest cleaning event for a room (the room's current progress).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningProgress {
    /// `started` | `done`.
    pub status: String,
    pub badge: String,
    pub name: Option<String>,
    pub at: DateTime<Utc>,
}

/// One room on the maid's list.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HkRoom {
    pub room_id: i32,
    pub room_no: String,
    pub floor: Option<i32>,
    pub building: Option<String>,
    /// Front-desk cleanliness flag (informational — which rooms need work).
    pub room_clean: bool,
    /// Today's latest maid-reported progress; `None` = nothing reported yet.
    pub cleaning: Option<CleaningProgress>,
    /// Open (untriaged) broken-item reports on this room.
    pub open_reports: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomsResponse {
    pub success: bool,
    pub data: Vec<HkRoom>,
}

/// One cleaning event in the room-detail log.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningEvent {
    pub event_id: i64,
    pub status: String,
    pub badge: String,
    pub name: Option<String>,
    pub at: DateTime<Utc>,
}

/// One broken-item report (metadata only — the photo streams separately).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokenReport {
    pub report_id: i64,
    pub description: String,
    pub badge: String,
    pub name: Option<String>,
    pub status: String,
    pub has_photo: bool,
    pub at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailResponse {
    pub success: bool,
    pub room: HkRoom,
    /// Today's cleaning events, recent first.
    pub events: Vec<CleaningEvent>,
    /// Recent broken-item reports on this room, recent first.
    pub reports: Vec<BrokenReport>,
}

/// Body for `POST /api/hk/rooms/{id}/cleaning`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCleaningBody {
    /// `started` | `done`.
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCleaningResponse {
    pub success: bool,
    pub room_id: i32,
    pub status: String,
}

/// Body for `POST /api/hk/rooms/{id}/broken-items`. The photo rides the same
/// base64→bytea pattern as `routes::guest_documents`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportBrokenItemBody {
    pub description: String,
    #[serde(default)]
    pub photo_base64: Option<String>,
    #[serde(default)]
    pub photo_mime: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportBrokenItemResponse {
    pub success: bool,
    pub report_id: i64,
    pub room_id: i32,
}

// ============================================================================
// Pure validation helpers (unit-tested below)
// ============================================================================

/// Normalize + validate a reported cleaning status.
fn parse_cleaning_status(raw: &str) -> Result<&'static str, ApiError> {
    let wanted = raw.trim().to_lowercase();
    VALID_CLEANING_STATUSES
        .iter()
        .find(|s| **s == wanted)
        .copied()
        .ok_or_else(|| {
            ApiError::BadRequest(format!(
                "invalid status '{raw}' (expected one of {VALID_CLEANING_STATUSES:?})"
            ))
        })
}

/// Normalize + validate a broken-item description.
fn parse_description(raw: &str) -> Result<String, ApiError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ApiError::BadRequest(
            "description must not be empty".to_string(),
        ));
    }
    if trimmed.chars().count() > MAX_DESCRIPTION_CHARS {
        return Err(ApiError::BadRequest(format!(
            "description exceeds {MAX_DESCRIPTION_CHARS} characters"
        )));
    }
    Ok(trimmed.to_string())
}

/// Decode an optional base64 photo, enforcing the size backstop. Returns
/// `(bytes, mime)` when present. Whitespace/newlines tolerated, same as
/// `routes::guest_documents`.
fn parse_photo(
    photo_base64: Option<&str>,
    photo_mime: Option<&str>,
) -> Result<Option<(Vec<u8>, String)>, ApiError> {
    let Some(raw) = photo_base64 else {
        return Ok(None);
    };
    let cleaned: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    if cleaned.is_empty() {
        return Ok(None);
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(cleaned.as_bytes())
        .map_err(|e| ApiError::BadRequest(format!("photoBase64 is not valid base64: {e}")))?;
    if bytes.is_empty() {
        return Ok(None);
    }
    if bytes.len() > MAX_PHOTO_BYTES {
        return Err(ApiError::BadRequest(format!(
            "photo exceeds {MAX_PHOTO_BYTES} bytes after decoding"
        )));
    }
    let mime = photo_mime
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("image/jpeg")
        .to_string();
    Ok(Some((bytes, mime)))
}

// ============================================================================
// SQL helpers (pool-parameterized so the DB-backed tests drive them directly)
// ============================================================================

/// The Thai-day predicate: an event "counts today" when its timestamp falls on
/// today's date in Asia/Bangkok. Both hotels run on Thai wall-clock days.
const TODAY_BKK: &str =
    "(hkev_created_at AT TIME ZONE 'Asia/Bangkok')::date = (NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// Fetch the active-room list with today's latest progress + open reports.
async fn fetch_rooms(pool: &PgPool) -> Result<Vec<HkRoom>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            COALESCE(r.room_clean, true) AS room_clean,
            ev.hkev_status,
            ev.hkev_badge,
            ev.hkev_name,
            ev.hkev_created_at,
            COALESCE(br.open_reports, 0) AS open_reports
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {TODAY_BKK}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
        LEFT JOIN LATERAL (
            SELECT COUNT(*) AS open_reports
            FROM ht_hk_broken_reports b
            WHERE b.hkbr_room_id = r.room_id AND b.hkbr_status = 'open'
        ) br ON TRUE
        WHERE COALESCE(r.room_active, true) = true
        ORDER BY r.room_no
        "#
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(room_from_row).collect())
}

/// Fetch one active room (today's progress + open-report count included).
async fn fetch_room(pool: &PgPool, room_id: i32) -> Result<Option<HkRoom>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_floor,
            r.room_building,
            COALESCE(r.room_clean, true) AS room_clean,
            ev.hkev_status,
            ev.hkev_badge,
            ev.hkev_name,
            ev.hkev_created_at,
            COALESCE(br.open_reports, 0) AS open_reports
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {TODAY_BKK}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
        LEFT JOIN LATERAL (
            SELECT COUNT(*) AS open_reports
            FROM ht_hk_broken_reports b
            WHERE b.hkbr_room_id = r.room_id AND b.hkbr_status = 'open'
        ) br ON TRUE
        WHERE r.room_id = $1 AND COALESCE(r.room_active, true) = true
        "#
    );
    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(room_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.as_ref().map(room_from_row))
}

fn room_from_row(row: &sqlx::postgres::PgRow) -> HkRoom {
    let cleaning = row
        .try_get::<String, _>("hkev_status")
        .ok()
        .map(|status| CleaningProgress {
            status,
            badge: row.try_get::<String, _>("hkev_badge").unwrap_or_default(),
            name: row.try_get::<String, _>("hkev_name").ok(),
            at: row
                .try_get::<DateTime<Utc>, _>("hkev_created_at")
                .unwrap_or_else(|_| Utc::now()),
        });
    HkRoom {
        room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
        room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
        floor: row.try_get::<i32, _>("room_floor").ok(),
        building: row.try_get::<String, _>("room_building").ok(),
        room_clean: row.try_get::<bool, _>("room_clean").unwrap_or(true),
        cleaning,
        open_reports: row.try_get::<i64, _>("open_reports").unwrap_or(0),
    }
}

/// Insert a cleaning-progress event. Returns the new event id.
async fn insert_cleaning_event(
    pool: &PgPool,
    room_id: i32,
    status: &str,
    badge: &str,
    name: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO ht_hk_cleaning_events (hkev_room_id, hkev_status, hkev_badge, hkev_name) \
         VALUES ($1, $2, $3, $4) RETURNING hkev_id",
    )
    .bind(room_id)
    .bind(status)
    .bind(badge)
    .bind(name)
    .fetch_one(pool)
    .await?;
    row.try_get("hkev_id")
}

/// Today's cleaning events for one room, recent first.
async fn fetch_today_events(
    pool: &PgPool,
    room_id: i32,
) -> Result<Vec<CleaningEvent>, sqlx::Error> {
    let sql = format!(
        "SELECT hkev_id, hkev_status, hkev_badge, hkev_name, hkev_created_at \
           FROM ht_hk_cleaning_events \
          WHERE hkev_room_id = $1 AND {TODAY_BKK} \
          ORDER BY hkev_created_at DESC, hkev_id DESC \
          LIMIT 20"
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(room_id)
        .fetch_all(pool)
        .await?;
    Ok(rows
        .iter()
        .map(|row| CleaningEvent {
            event_id: row.try_get::<i64, _>("hkev_id").unwrap_or(0),
            status: row.try_get::<String, _>("hkev_status").unwrap_or_default(),
            badge: row.try_get::<String, _>("hkev_badge").unwrap_or_default(),
            name: row.try_get::<String, _>("hkev_name").ok(),
            at: row
                .try_get::<DateTime<Utc>, _>("hkev_created_at")
                .unwrap_or_else(|_| Utc::now()),
        })
        .collect())
}

/// Insert a broken-item report. Returns the new report id.
#[allow(clippy::too_many_arguments)]
async fn insert_broken_report(
    pool: &PgPool,
    room_id: i32,
    description: &str,
    badge: &str,
    name: Option<&str>,
    photo: Option<&[u8]>,
    photo_mime: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO ht_hk_broken_reports \
             (hkbr_room_id, hkbr_description, hkbr_badge, hkbr_name, hkbr_photo, hkbr_photo_mime) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING hkbr_id",
    )
    .bind(room_id)
    .bind(description)
    .bind(badge)
    .bind(name)
    .bind(photo)
    .bind(photo_mime)
    .fetch_one(pool)
    .await?;
    row.try_get("hkbr_id")
}

/// Recent broken-item reports for one room (metadata only), recent first.
async fn fetch_room_reports(pool: &PgPool, room_id: i32) -> Result<Vec<BrokenReport>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT hkbr_id, hkbr_description, hkbr_badge, hkbr_name, hkbr_status, \
                (hkbr_photo IS NOT NULL) AS has_photo, hkbr_created_at \
           FROM ht_hk_broken_reports \
          WHERE hkbr_room_id = $1 \
          ORDER BY hkbr_created_at DESC, hkbr_id DESC \
          LIMIT 10",
    )
    .bind(room_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .iter()
        .map(|row| BrokenReport {
            report_id: row.try_get::<i64, _>("hkbr_id").unwrap_or(0),
            description: row
                .try_get::<String, _>("hkbr_description")
                .unwrap_or_default(),
            badge: row.try_get::<String, _>("hkbr_badge").unwrap_or_default(),
            name: row.try_get::<String, _>("hkbr_name").ok(),
            status: row
                .try_get::<String, _>("hkbr_status")
                .unwrap_or_else(|_| "open".to_string()),
            has_photo: row.try_get::<bool, _>("has_photo").unwrap_or(false),
            at: row
                .try_get::<DateTime<Utc>, _>("hkbr_created_at")
                .unwrap_or_else(|_| Utc::now()),
        })
        .collect())
}

/// 404-checking room existence probe (active rooms only).
async fn require_room(pool: &PgPool, room_id: i32) -> ApiResult<()> {
    let exists = sqlx::query(
        "SELECT 1 FROM ht_rooms_new WHERE room_id = $1 AND COALESCE(room_active, true) = true",
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await?;
    if exists.is_none() {
        return Err(ApiError::NotFound(format!("room {room_id} not found")));
    }
    Ok(())
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/hk/me — echo the verified identity for the page header.
pub async fn me(Extension(identity): Extension<HkIdentity>) -> ApiResult<Json<MeResponse>> {
    Ok(Json(MeResponse {
        success: true,
        badge: identity.badge,
        display_name: identity.display_name,
    }))
}

/// GET /api/hk/rooms — the maid's room list with today's progress.
pub async fn list_rooms(
    State(state): State<AppState>,
    Query(query): Query<HkBranchQuery>,
) -> ApiResult<Json<RoomsResponse>> {
    let pool = resolve_pool(&state, query.branch)?;
    let data = fetch_rooms(pool).await?;
    Ok(Json(RoomsResponse {
        success: true,
        data,
    }))
}

/// GET /api/hk/rooms/{id} — one room with today's events + recent reports.
pub async fn room_detail(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
) -> ApiResult<Json<RoomDetailResponse>> {
    let pool = resolve_pool(&state, query.branch)?;
    let room = fetch_room(pool, room_id)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("room {room_id} not found")))?;
    let events = fetch_today_events(pool, room_id).await?;
    let reports = fetch_room_reports(pool, room_id).await?;
    Ok(Json(RoomDetailResponse {
        success: true,
        room,
        events,
        reports,
    }))
}

/// POST /api/hk/rooms/{id}/cleaning — report cleaning progress.
pub async fn report_cleaning(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(identity): Extension<HkIdentity>,
    Json(body): Json<ReportCleaningBody>,
) -> ApiResult<Json<ReportCleaningResponse>> {
    let status = parse_cleaning_status(&body.status)?;
    let pool = resolve_pool(&state, query.branch)?;
    require_room(pool, room_id).await?;
    insert_cleaning_event(
        pool,
        room_id,
        status,
        &identity.badge,
        identity.display_name.as_deref(),
    )
    .await?;
    Ok(Json(ReportCleaningResponse {
        success: true,
        room_id,
        status: status.to_string(),
    }))
}

/// POST /api/hk/rooms/{id}/broken-items — submit a broken-item report.
pub async fn report_broken_item(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Query(query): Query<HkBranchQuery>,
    Extension(identity): Extension<HkIdentity>,
    Json(body): Json<ReportBrokenItemBody>,
) -> ApiResult<(StatusCode, Json<ReportBrokenItemResponse>)> {
    let description = parse_description(&body.description)?;
    let photo = parse_photo(body.photo_base64.as_deref(), body.photo_mime.as_deref())?;
    let pool = resolve_pool(&state, query.branch)?;
    require_room(pool, room_id).await?;
    let report_id = insert_broken_report(
        pool,
        room_id,
        &description,
        &identity.badge,
        identity.display_name.as_deref(),
        photo.as_ref().map(|(bytes, _)| bytes.as_slice()),
        photo.as_ref().map(|(_, mime)| mime.as_str()),
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(ReportBrokenItemResponse {
            success: true,
            report_id,
            room_id,
        }),
    ))
}

/// GET /api/hk/broken-items/{id}/photo — stream a report's photo bytes.
pub async fn broken_item_photo(
    State(state): State<AppState>,
    Path(report_id): Path<i64>,
    Query(query): Query<HkBranchQuery>,
) -> ApiResult<Response> {
    let pool = resolve_pool(&state, query.branch)?;
    let row = sqlx::query(
        "SELECT hkbr_photo, hkbr_photo_mime FROM ht_hk_broken_reports WHERE hkbr_id = $1",
    )
    .bind(report_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("report {report_id} not found")))?;

    let photo: Option<Vec<u8>> = row.try_get("hkbr_photo").ok();
    let Some(bytes) = photo else {
        return Err(ApiError::NotFound(format!(
            "report {report_id} has no photo"
        )));
    };
    let mime: String = row
        .try_get::<String, _>("hkbr_photo_mime")
        .unwrap_or_else(|_| "image/jpeg".to_string());

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CACHE_CONTROL, "private, max-age=3600")
        .body(Body::from(bytes))
        .map_err(|e| ApiError::Internal(format!("failed to build photo response: {e}")))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- pure validation ----------------------------------------------

    #[test]
    fn cleaning_status_accepts_only_started_and_done() {
        assert_eq!(parse_cleaning_status("started").unwrap(), "started");
        assert_eq!(parse_cleaning_status(" DONE ").unwrap(), "done");
        for bad in ["", "cleaning", "finished", "เสร็จแล้ว"] {
            assert!(
                parse_cleaning_status(bad).is_err(),
                "'{bad}' must be rejected"
            );
        }
    }

    #[test]
    fn description_is_trimmed_and_bounded() {
        assert_eq!(parse_description("  ก๊อกน้ำพัง  ").unwrap(), "ก๊อกน้ำพัง");
        assert!(parse_description("   ").is_err());
        let too_long = "ก".repeat(MAX_DESCRIPTION_CHARS + 1);
        assert!(parse_description(&too_long).is_err());
        let just_fits = "ก".repeat(MAX_DESCRIPTION_CHARS);
        assert!(parse_description(&just_fits).is_ok());
    }

    #[test]
    fn photo_decoding_handles_absent_blank_valid_and_garbage() {
        // Absent / blank ⇒ no photo, no error.
        assert!(parse_photo(None, None).unwrap().is_none());
        assert!(parse_photo(Some("   \n"), None).unwrap().is_none());
        // Valid base64 round-trips, default mime applied.
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"jpegbytes");
        let (bytes, mime) = parse_photo(Some(&encoded), None).unwrap().unwrap();
        assert_eq!(bytes, b"jpegbytes");
        assert_eq!(mime, "image/jpeg");
        // Explicit mime wins.
        let (_, mime) = parse_photo(Some(&encoded), Some("image/png"))
            .unwrap()
            .unwrap();
        assert_eq!(mime, "image/png");
        // Garbage is a BadRequest.
        assert!(parse_photo(Some("!!!not-base64!!!"), None).is_err());
    }

    #[test]
    fn oversized_photo_is_rejected() {
        // A base64 payload that decodes to MAX_PHOTO_BYTES + 3 bytes.
        let raw = vec![0u8; MAX_PHOTO_BYTES + 3];
        let encoded = base64::engine::general_purpose::STANDARD.encode(&raw);
        assert!(parse_photo(Some(&encoded), None).is_err());
    }

    // ---- DB-backed (skip gracefully when no local PG — same `try_pool`
    //      convention as `service::housekeeping::tests`) ------------------

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    /// Seed a marker room, exercise the full progress + broken-report flow,
    /// and verify the "today" room list reflects the LATEST event.
    #[tokio::test]
    async fn cleaning_and_broken_report_round_trip() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping cleaning_and_broken_report_round_trip — PG not reachable");
            return;
        };
        // Reset marker rows from prior (possibly aborted) runs. Events and
        // reports cascade with the room delete (ON DELETE CASCADE).
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-HK1'")
            .execute(&pool)
            .await;
        let row = sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
             VALUES ('ZT-HK1', false, true) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let room_id: i32 = row.try_get("room_id").unwrap();

        // started → done: the list must surface the LATEST event.
        insert_cleaning_event(&pool, room_id, "started", "Q1001", Some("Nok"))
            .await
            .expect("started event must insert");
        insert_cleaning_event(&pool, room_id, "done", "Q1001", Some("Nok"))
            .await
            .expect("done event must insert");

        let rooms = fetch_rooms(&pool).await.expect("room list must fetch");
        let room = rooms
            .iter()
            .find(|r| r.room_id == room_id)
            .expect("seeded room must be listed");
        let progress = room.cleaning.as_ref().expect("today's progress present");
        assert_eq!(progress.status, "done");
        assert_eq!(progress.badge, "Q1001");
        assert_eq!(room.open_reports, 0);

        // Broken-item report (with photo) → open_reports increments, detail
        // queries return it, photo bytes round-trip.
        let report_id = insert_broken_report(
            &pool,
            room_id,
            "ก๊อกน้ำรั่ว",
            "Q1001",
            Some("Nok"),
            Some(b"fakejpeg"),
            Some("image/jpeg"),
        )
        .await
        .expect("broken report must insert");
        assert!(report_id > 0);

        let room = fetch_room(&pool, room_id)
            .await
            .expect("room fetch must succeed")
            .expect("room must exist");
        assert_eq!(room.open_reports, 1);

        let events = fetch_today_events(&pool, room_id).await.unwrap();
        assert_eq!(events.len(), 2, "both events land in today's log");
        assert_eq!(events[0].status, "done", "recent-first ordering");

        let reports = fetch_room_reports(&pool, room_id).await.unwrap();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].description, "ก๊อกน้ำรั่ว");
        assert!(reports[0].has_photo);
        assert_eq!(reports[0].status, "open");

        // CHECK constraint rejects an invalid status at the DB layer too.
        let bad = insert_cleaning_event(&pool, room_id, "finished", "Q1001", None).await;
        assert!(bad.is_err(), "CHECK constraint must reject bad status");

        // Cleanup (cascades the events + reports).
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }

    /// The room-existence probe 404s inactive/unknown rooms.
    #[tokio::test]
    async fn require_room_rejects_unknown_and_inactive() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping require_room_rejects_unknown_and_inactive — PG not reachable");
            return;
        };
        // Unknown id.
        assert!(require_room(&pool, -424242).await.is_err());

        // Inactive room.
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-HK2'")
            .execute(&pool)
            .await;
        let row = sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_active) \
             VALUES ('ZT-HK2', false) RETURNING room_id",
        )
        .fetch_one(&pool)
        .await
        .expect("seed insert must succeed");
        let room_id: i32 = row.try_get("room_id").unwrap();
        assert!(require_room(&pool, room_id).await.is_err());
        assert!(
            fetch_room(&pool, room_id).await.unwrap().is_none(),
            "inactive rooms are hidden from the maid list"
        );
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(&pool)
            .await;
    }
}
