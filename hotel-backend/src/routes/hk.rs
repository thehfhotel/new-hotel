//! Maid-facing housekeeping surface (`/api/hk/*`) — employee-login plan
//! Phase 4 (HF-erp `docs/employee-login-plan.md`).
//!
//! Serves the mobile `/hk` pages maids open from their LINE Role Menu:
//!
//! - `GET  /api/hk/me`                          — the verified maid identity.
//! - `GET  /api/hk/rooms`                       — room list + today's progress.
//! - `GET  /api/hk/rooms/{id}`                  — one room + today's events.
//! - `POST /api/hk/rooms/{id}/cleaning`         — report progress (`started`/`done`).
//! - `POST /api/hk/rooms/{id}/broken-items`     — RETIRED, answers `410 Gone`.
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
//! **Changed 2026-08-11 (housekeeping-ops).** This surface used to be
//! PG-canonical-only with NO legacy coupling at all. It no longer is:
//!
//! - `ht_hk_cleaning_events` / `ht_hk_broken_reports` (migration 077) remain
//!   PG-canonical-only NEW data — no sync mapper, no legacy twin.
//! - **`cleaning` with `done` DOES now reach iHOTEL**: it delegates to
//!   [`crate::service::housekeeping::HousekeepingService::mark_clean_if_dirty`],
//!   which flips `ht_rooms_new.room_clean`, enqueues the proven
//!   `MarkRoomClean` writeback and publishes `RoomMarkedClean` — all in one
//!   transaction (PG first, mirror async; invariant #2). That is the whole
//!   point of the change: reception must see the finished room on iHOTEL's
//!   board without the maid touching the legacy app.
//! - `started` stays PG-only on purpose — iHOTEL's in-progress field
//!   `Room_Clean_Time` feeds its room-power countdown, so mirroring it is
//!   parity risk for no operational gain (housekeeping-ops plan, decision #3).
//! - The repeat-tap guard lives in `mark_clean_if_dirty`'s conditional UPDATE,
//!   so a maid double-tapping เสร็จแล้ว cannot double-write `HT_Housewife`.
//!
//! Branch-aware via `?branch=` resolved through the unified
//! [`AppState::write_pool`] chokepoint (Ship-B gate).
//!
//! ## HF Ville admission (housekeeping-ops launch)
//!
//! The launch environment leaves `HFVILLE_WRITES_ENABLED` UNSET — general
//! Ville mutations must stay inadmissible — and sets
//! `HFVILLE_WRITEBACK_INTENTS=mark_room_clean` on the Ville writeback worker.
//! For the maid surface to work at Ville under that env, `ville_write_guard`
//! grants ONE narrow exemption: `POST /api/hk/rooms/{id}/cleaning` is admitted
//! for `branch=hfville` regardless of the flag
//! ([`crate::middleware::ville_guard::is_ville_exempt_path`]).
//!
//! That keeps the ADMITTED set identical to the ALLOWLISTED writeback set. The
//! alternative — flipping `HFVILLE_WRITES_ENABLED` on — would admit every other
//! Ville mutation, each writing canonical PG while its writeback intent parked
//! as `'skipped'`, silently diverging PG from Ville's iHOTEL. `broken-items`
//! and every non-hk mutation stay blocked. Pinned by
//! `tests/test_hk_ville_guard.rs`.
//!
//! All SQL is RUNTIME `sqlx::query` (no compile-time macro), so this module
//! needs no `.sqlx/` cache regeneration — same policy as
//! `routes::guest_documents` / `routes::new_maintenance`.

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};
use crate::middleware::hk_access::HkIdentity;
use crate::outbox::event::EventSource;
use crate::service::housekeeping::{HousekeepingService, MarkCleanCommand};

/// Cleaning-progress statuses a maid can report. `started` =
/// เริ่มทำความสะอาด, `done` = เสร็จแล้ว. Matches the CHECK constraint on
/// `ht_hk_cleaning_events.hkev_status` (migration 077).
pub const VALID_CLEANING_STATUSES: [&str; 2] = ["started", "done"];

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

/// Build a [`HousekeepingService`] bound to the branch's pool — identical
/// construction to `routes::housekeeping::service_for`, so the maid surface
/// and the front desk share one code path into the `MarkRoomClean` writeback
/// (and one `Hfville → ville_pool` decision, via `write_pool`).
fn service_for(state: &AppState, branch: Option<Branch>) -> ApiResult<HousekeepingService> {
    let pool = state.write_pool(branch)?.clone();
    Ok(HousekeepingService::new(
        state.rooms.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool,
    ))
}

/// Max chars kept from a maid's display name for `HT_Housewife.h_name`.
///
/// `h_name` is `varchar(150)` in the legacy schema. Thai is single-byte under
/// the DB's TIS-620 collation but 3 bytes as UTF-8, and we cannot be certain
/// which budget a given driver/collation path spends, so bound BOTH: at most
/// [`MAX_H_NAME_CHARS`] characters AND at most [`MAX_H_NAME_BYTES`] UTF-8
/// bytes. That is comfortably inside 150 under either reading.
const MAX_H_NAME_CHARS: usize = 45;
const MAX_H_NAME_BYTES: usize = 140;

/// The label recorded as the housekeeper in `HT_Housewife.h_name`.
///
/// Prefers the verified HF ID display name so iHOTEL's housekeeping log names
/// the actual maid; falls back to the badge, which is ALWAYS present (the
/// middleware 401s without one). Today the CF Access IdP forwards only
/// `["apps", "badge"]`, so this resolves to the badge in production — adding
/// `name` to the forwarded claims upgrades the audit row with no code change
/// here. Never client-supplied: both fields come from the verified assertion.
///
/// The name is IdP-supplied and therefore unbounded, while `h_name` is
/// `varchar(150)`. An over-long name would make the legacy INSERT fail with
/// MSSQL 8152 (string truncation) and the writeback job would retry to
/// `exhausted` — a stuck queue caused purely by someone's long display name.
/// Truncation on a CHAR boundary avoids that and never splits a UTF-8
/// sequence (a byte-sliced Thai name would corrupt the literal).
fn maid_label(identity: &HkIdentity) -> String {
    let raw = identity
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or(&identity.badge);
    truncate_h_name(raw)
}

/// Clamp to [`MAX_H_NAME_CHARS`] chars and [`MAX_H_NAME_BYTES`] UTF-8 bytes,
/// always on a char boundary. PURE — unit-tested below.
fn truncate_h_name(raw: &str) -> String {
    let mut out = String::new();
    for (index, ch) in raw.chars().enumerate() {
        if index >= MAX_H_NAME_CHARS || out.len() + ch.len_utf8() > MAX_H_NAME_BYTES {
            break;
        }
        out.push(ch);
    }
    out
}

/// `EventSource` for a maid-originated cleaning event. Mirrors
/// `routes::housekeeping::http_source`; a real `user_id` would land here if
/// maids ever became PMS accounts (they are CF Access identities today).
fn hk_source() -> EventSource {
    EventSource::our_app(uuid::Uuid::nil(), uuid::Uuid::new_v4())
}

// ============================================================================
// Router
// ============================================================================

/// Build the fully-layered `/api/hk/*` router.
///
/// Lives here (rather than inline in `main`) so `main.rs` and the integration
/// tests mount the SAME stack — a test that rebuilt the layer order by hand
/// would be testing its own replica, not the shipped wiring. Same idiom as
/// `routes::admin_users::router()`.
///
/// Layer order is load-bearing, outermost last:
/// 1. `ville_write_guard` (OUTERMOST) — a disabled-Ville mutation is refused
///    up front, before any auth work, EXCEPT the exempt cleaning route.
/// 2. `require_hk_access` — Cloudflare Access gate; fails closed (401) when
///    `CF_ACCESS_HK_AUD` is unset, so the surface ships dark.
/// 3. body limit, then the handlers.
///
/// That order is what `tests/test_hk_ville_guard.rs` asserts against: an
/// unauthenticated `branch=hfville` request returns 403 when the guard refuses
/// it and 401 when the guard admits it and auth then refuses — which
/// distinguishes the two layers without needing a valid Access assertion.
pub fn router(state: AppState) -> Router {
    let ville_guard = axum::middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::ville_guard::ville_write_guard,
    );
    Router::new()
        .route("/api/hk/me", get(me))
        .route("/api/hk/rooms", get(list_rooms))
        .route("/api/hk/rooms/{room_id}", get(room_detail))
        .route("/api/hk/rooms/{room_id}/cleaning", post(report_cleaning))
        .route(
            "/api/hk/rooms/{room_id}/broken-items",
            post(report_broken_item),
        )
        .route(
            "/api/hk/broken-items/{report_id}/photo",
            get(broken_item_photo),
        )
        .layer(axum::extract::DefaultBodyLimit::max(8 * 1024 * 1024))
        .layer(axum::middleware::from_fn(
            crate::middleware::hk_access::require_hk_access,
        ))
        .layer(ville_guard)
        .with_state(state)
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDetailResponse {
    pub success: bool,
    pub room: HkRoom,
    /// Today's cleaning events, recent first.
    pub events: Vec<CleaningEvent>,
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
    /// `true` when this call performed the dirty→clean transition and so
    /// enqueued the `MarkRoomClean` writeback. `false` for `started`, and for
    /// a repeat `done` on an already-clean room (idempotent no-op).
    ///
    /// ENQUEUED, NOT DELIVERED. This says a `writeback_jobs` row was committed
    /// alongside the canonical flip — nothing about iHOTEL having been written.
    /// The worker drains asynchronously and may retry, park (`'skipped'`, if
    /// the HF Ville allowlist excludes the intent) or exhaust. Callers must not
    /// render this as "the front desk can see it now".
    pub writeback_enqueued: bool,
}

/// Body of a `410 Gone` answer from a retired endpoint. Same
/// `{success:false, error}` shape the frontend's `hkFetch` already expects.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoneResponse {
    pub success: bool,
    pub error: String,
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

// ============================================================================
// SQL helpers (pool-parameterized so the DB-backed tests drive them directly)
// ============================================================================

/// The Thai-day predicate: an event "counts today" when its timestamp falls on
/// today's date in Asia/Bangkok. Both hotels run on Thai wall-clock days.
const TODAY_BKK: &str =
    "(hkev_created_at AT TIME ZONE 'Asia/Bangkok')::date = (NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// Fetch the active-room list with today's latest cleaning progress.
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
            ev.hkev_created_at
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {TODAY_BKK}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
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
            ev.hkev_created_at
        FROM ht_rooms_new r
        LEFT JOIN LATERAL (
            SELECT e.hkev_status, e.hkev_badge, e.hkev_name, e.hkev_created_at
            FROM ht_hk_cleaning_events e
            WHERE e.hkev_room_id = r.room_id AND {TODAY_BKK}
            ORDER BY e.hkev_created_at DESC, e.hkev_id DESC
            LIMIT 1
        ) ev ON TRUE
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

/// GET /api/hk/rooms/{id} — one room with today's cleaning events.
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
    Ok(Json(RoomDetailResponse {
        success: true,
        room,
        events,
    }))
}

/// POST /api/hk/rooms/{id}/cleaning — report cleaning progress.
///
/// `started` is PG-only (see the module header). `done` ADDITIONALLY drives
/// the canonical clean flip + the `MarkRoomClean` legacy writeback through
/// [`HousekeepingService::mark_clean_if_dirty`], so the front desk sees the
/// maid's finished room in iHOTEL.
///
/// The maid's append-only event is recorded FIRST and unconditionally: it is
/// the maid's own record of work done, and it must survive even if the
/// writeback leg is unavailable (e.g. the Ville pool is down). The
/// `writeback_enqueued` flag reports whether this call was the one that
/// performed the dirty→clean transition — `false` on a repeat tap.
///
/// Consequence, accepted by design: because the event insert is a separate
/// statement from the service transaction, a client retry after a 5xx can
/// append a SECOND `ht_hk_cleaning_events` row for the same tap. That log is
/// append-only by nature (`started`/`done` may legitimately repeat in a day)
/// and the room's "current progress" reads only the LATEST row, so a duplicate
/// is cosmetic. The thing that must not double — the legacy write — is guarded
/// separately by `mark_clean_if_dirty`'s conditional UPDATE.
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

    // `done` ⇒ canonical flip + outbox enqueue + domain event in ONE tx.
    // Idempotent: a repeat `done` on an already-clean room returns `None` and
    // enqueues nothing (invariant #4).
    let writeback_enqueued = if status == "done" {
        let svc = service_for(&state, query.branch)?;
        svc.mark_clean_if_dirty(MarkCleanCommand {
            room_id,
            by: maid_label(&identity),
            source: hk_source(),
        })
        .await?
        .is_some()
    } else {
        false
    };

    Ok(Json(ReportCleaningResponse {
        success: true,
        room_id,
        status: status.to_string(),
        writeback_enqueued,
    }))
}

/// POST /api/hk/rooms/{id}/broken-items — **RETIRED (410 Gone)**.
///
/// Breakage intake moved to the Housekeeping ops app
/// (`housekeeping.thehfhotel.org/staff/report`), which opens a real Work Order
/// with photos, status lifecycle and reception ownership — see the
/// housekeeping-ops plan, Module 2. `410` (not `404`) is deliberate: the
/// resource existed and is permanently gone, which tells a stale cached
/// client to stop retrying rather than treat it as a transient routing error.
///
/// `ht_hk_broken_reports` and its rows are KEPT as history, and
/// [`broken_item_photo`] still serves their photos.
pub async fn report_broken_item() -> (StatusCode, Json<GoneResponse>) {
    (
        StatusCode::GONE,
        Json(GoneResponse {
            success: false,
            error: "endpoint ย้ายไปที่แอปแม่บ้านแล้ว: แจ้งซ่อมที่ housekeeping.thehfhotel.org/staff/report"
                .to_string(),
        }),
    )
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

    /// The maid label stamped into `HT_Housewife.h_name` prefers the verified
    /// display name and falls back to the badge — never blank, because
    /// `mark_clean_if_dirty` rejects an empty `by`.
    #[test]
    fn maid_label_prefers_display_name_then_badge() {
        let with_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: Some("นก".into()),
            email: None,
        };
        assert_eq!(maid_label(&with_name), "นก");

        // Production reality today: the CF IdP forwards only `apps` + `badge`.
        let no_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: None,
            email: None,
        };
        assert_eq!(maid_label(&no_name), "Q1001");

        // A whitespace-only name must not produce a blank `by`.
        let blank_name = HkIdentity {
            badge: "Q1001".into(),
            display_name: Some("   ".into()),
            email: None,
        };
        assert_eq!(maid_label(&blank_name), "Q1001");
    }

    /// `h_name` is `varchar(150)`. An unbounded IdP display name would make
    /// the legacy INSERT fail with MSSQL 8152 and the writeback job retry to
    /// `exhausted` — a queue stuck on somebody's long name. The clamp must
    /// hold under BOTH byte budgets and never split a UTF-8 sequence.
    #[test]
    fn maid_label_is_clamped_for_ht_housewife_varchar_150() {
        // Thai is the realistic worst case: 3 UTF-8 bytes per char.
        let long_thai = "น".repeat(500);
        let clamped = maid_label(&HkIdentity {
            badge: "Q1001".into(),
            display_name: Some(long_thai),
            email: None,
        });
        assert_eq!(clamped.chars().count(), MAX_H_NAME_CHARS);
        assert!(
            clamped.len() <= MAX_H_NAME_BYTES,
            "{} bytes exceeds the byte budget",
            clamped.len()
        );
        assert!(clamped.len() < 150, "must fit varchar(150) as raw bytes too");
        // Truncation landed on a char boundary — the string is still valid
        // UTF-8 and contains only whole Thai characters.
        assert!(clamped.chars().all(|c| c == 'น'));

        // ASCII hits the char cap before the byte cap.
        let long_ascii = "a".repeat(500);
        let clamped_ascii = maid_label(&HkIdentity {
            badge: "Q1001".into(),
            display_name: Some(long_ascii),
            email: None,
        });
        assert_eq!(clamped_ascii.len(), MAX_H_NAME_CHARS);

        // Normal names are untouched.
        assert_eq!(
            maid_label(&HkIdentity {
                badge: "Q1001".into(),
                display_name: Some("นก สมใจ".into()),
                email: None,
            }),
            "นก สมใจ"
        );

        // 4-byte chars must not overflow the byte budget either.
        let emoji_ish = "𝔘".repeat(100); // 4 bytes each
        let clamped_wide = truncate_h_name(&emoji_ish);
        assert!(clamped_wide.len() <= MAX_H_NAME_BYTES);
        assert!(clamped_wide.chars().count() <= MAX_H_NAME_CHARS);
    }

    /// The retired broken-item intake answers 410 (permanently gone), not 404
    /// (transient/unknown) — so stale cached clients stop retrying.
    #[tokio::test]
    async fn retired_broken_item_endpoint_answers_410() {
        let (status, Json(body)) = report_broken_item().await;
        assert_eq!(status, StatusCode::GONE);
        assert!(!body.success);
        assert!(
            body.error.contains("housekeeping.thehfhotel.org/staff/report"),
            "the 410 must point maids at the new intake; got: {}",
            body.error
        );
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

        // Historical broken-item rows must stay READABLE by the surviving
        // photo route even though the list read path and its `reports` /
        // `openReports` response fields are gone (intake retired, 410; the
        // Housekeeping ops app owns new reports). Seeded via raw SQL because
        // no intake helper remains.
        let report_row = sqlx::query(
            "INSERT INTO ht_hk_broken_reports \
                 (hkbr_room_id, hkbr_description, hkbr_badge, hkbr_name, hkbr_photo, hkbr_photo_mime) \
             VALUES ($1, 'ก๊อกน้ำรั่ว', 'Q1001', 'Nok', $2, 'image/jpeg') RETURNING hkbr_id",
        )
        .bind(room_id)
        .bind(b"fakejpeg".as_slice())
        .fetch_one(&pool)
        .await
        .expect("historical broken report must insert");
        let report_id: i64 = report_row.try_get("hkbr_id").unwrap();
        assert!(report_id > 0);

        let stored: Vec<u8> =
            sqlx::query_scalar("SELECT hkbr_photo FROM ht_hk_broken_reports WHERE hkbr_id = $1")
                .bind(report_id)
                .fetch_one(&pool)
                .await
                .expect("the photo route's row must still be readable");
        assert_eq!(stored, b"fakejpeg", "history rows survive the retirement");

        let events = fetch_today_events(&pool, room_id).await.unwrap();
        assert_eq!(events.len(), 2, "both events land in today's log");
        assert_eq!(events[0].status, "done", "recent-first ordering");

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
