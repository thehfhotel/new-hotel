//! Room & staff sticky-notes API (โน้ตห้อง / โน้ตพนักงาน) — task #47 / migration 062.
//!
//! - `GET  /api/notes?branch=&kind=&target=&unread=` — list notes, unread first.
//! - `POST /api/notes`                                — add a note (room or staff).
//! - `POST /api/notes/{id}/read`                      — mark a note read.
//!
//! Branch-aware: `?branch=hfville` reads/writes the `hotelville` canonical pool,
//! otherwise the primary `hotelnew` pool — the same per-site resolution as
//! [`crate::routes::new_cash`]. Site scoping is connection-level (each logical
//! DB holds exactly one site's notes), so the pool selection *is* the site
//! filter. All queries are LITERAL strings bound with `.bind()` (runtime
//! `sqlx::query`, no `query!` macro / no `.sqlx` cache).
//!
//! ## Writeback (SHIPPED DARK — `NOTES_WRITEBACK_ENABLED`, default off)
//!
//! Coexistence (ADR 0002): a note added here SHOULD also land in the legacy
//! `HT_Room_SMS` / `HT_EMP_SMS` table so iHOTEL sees it. The verified recipe
//! lives in [`crate::writeback::recipes::sticky_note`] and is wired through
//! [`WritebackIntent::CreateNote`] / [`WritebackIntent::MarkNoteRead`], but the
//! emitter below only enqueues when `NOTES_WRITEBACK_ENABLED` is set — default
//! off (zero legacy writes pending a reception-coordinated live test, mirroring
//! the round-bill `ROUND_WRITEBACK_ENABLED` precedent). When off these handlers
//! write canonical PG only; the inbound poll
//! ([`crate::bin`]`/sync.rs::sync_sticky_notes`) still mirrors iHOTEL's notes in.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::outbox::{generate_idempotency_key, NoteTargetKind, OutboxRepository, WritebackIntent};

// =============================================================================
// Query / request / response shapes
// =============================================================================

/// `GET /api/notes` query string.
#[derive(Debug, Deserialize)]
pub struct NoteListQuery {
    pub branch: Option<Branch>,
    /// Optional target-kind filter (`room` | `staff`).
    pub kind: Option<String>,
    /// Optional exact target-key filter (room number / staff username).
    pub target: Option<String>,
    /// When `true`, return only unread notes.
    pub unread: Option<bool>,
}

/// Branch selector for the create + mark-read endpoints.
#[derive(Debug, Deserialize)]
pub struct NoteBranchQuery {
    pub branch: Option<Branch>,
}

/// Body for `POST /api/notes`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest {
    /// `room` → HT_Room_SMS, `staff` → HT_EMP_SMS.
    pub target_kind: NoteTargetKind,
    /// Room number (room notes) or staff username (staff notes).
    pub target_key: String,
    /// The note body.
    pub body: String,
    /// Author login. Optional; defaults to empty.
    #[serde(default)]
    pub created_by: Option<String>,
}

/// One note row in API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteDto {
    pub note_id: i64,
    pub target_kind: String,
    pub target_key: String,
    pub body: String,
    pub created_by: Option<String>,
    pub is_read: bool,
    pub legacy_id: Option<i32>,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// `200` body for `GET /api/notes`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteListResponse {
    pub success: bool,
    pub notes: Vec<NoteDto>,
    pub unread_count: i64,
}

/// Body for a successful create / mark-read.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteMutationResponse {
    pub success: bool,
    pub note: NoteDto,
}

// =============================================================================
// Helpers
// =============================================================================

/// Whether note writeback to the legacy SMS tables is enabled (default off).
/// Read at request time (cheap) — same ship-dark pattern as the `/api/mode`
/// flags. Mirrors `ROUND_WRITEBACK_ENABLED`.
fn notes_writeback_enabled() -> bool {
    std::env::var("NOTES_WRITEBACK_ENABLED")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false)
}

/// Resolve the per-site pool for a branch (HF Ville → ville pool, else primary).
/// `All` collapses to the primary pool — notes are per-site and the board picks
/// one branch at a time.
fn resolve_pool<'a>(state: &'a AppState, branch: Option<Branch>) -> ApiResult<&'a crate::db::PgPool> {
    match branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool(),
        Branch::Hfhotel | Branch::All => Ok(&state.new_pool),
    }
}

/// Map a decode failure on a note read to a 500 (schema drift, not a client bug).
fn map_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map note row: {e}"))
}

/// Build a [`NoteDto`] from a `ht_notes` row (the SELECT/RETURNING column set).
fn row_to_dto(row: &sqlx::postgres::PgRow) -> ApiResult<NoteDto> {
    Ok(NoteDto {
        note_id: row.try_get("note_id").map_err(map_row_err)?,
        target_kind: row.try_get("note_target_kind").map_err(map_row_err)?,
        target_key: row.try_get("note_target_key").map_err(map_row_err)?,
        body: row.try_get("note_body").map_err(map_row_err)?,
        created_by: row.try_get("note_created_by").map_err(map_row_err)?,
        is_read: row.try_get("note_is_read").map_err(map_row_err)?,
        legacy_id: row.try_get("note_legacy_id").map_err(map_row_err)?,
        source: row.try_get("note_source").map_err(map_row_err)?,
        created_at: row.try_get("note_created_at").map_err(map_row_err)?,
        updated_at: row.try_get("note_updated_at").map_err(map_row_err)?,
    })
}

/// Parse the canonical `note_target_kind` string back into a [`NoteTargetKind`]
/// (for building the writeback intent). Defensive default to `Staff` is
/// unreachable in practice — the column has a CHECK domain of room/staff.
fn parse_kind(s: &str) -> NoteTargetKind {
    match s {
        "room" => NoteTargetKind::Room,
        _ => NoteTargetKind::Staff,
    }
}

// =============================================================================
// Handlers
// =============================================================================

/// `GET /api/notes` — list notes (unread first), branch-aware, with optional
/// kind / target / unread-only filters.
pub async fn list_notes(
    State(state): State<AppState>,
    Query(query): Query<NoteListQuery>,
) -> ApiResult<Json<NoteListResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    // Validate the optional kind filter against the canonical CHECK domain so a
    // typo is a 400, not a silent empty result.
    let kind: Option<&str> = match query.kind.as_deref() {
        None | Some("") => None,
        Some(k @ ("room" | "staff")) => Some(k),
        Some(other) => {
            return Err(ApiError::BadRequest(format!(
                "invalid kind '{other}' (expected room | staff)"
            )))
        }
    };
    let target = query.target.filter(|t| !t.is_empty());
    // unread=true → only unread (note_is_read = false); else no filter.
    let is_read_filter: Option<bool> = if query.unread == Some(true) { Some(false) } else { None };

    // LITERAL query; every value is bound. Optional filters use the
    // `($N::type IS NULL OR col = $N)` idiom so the string never changes.
    let rows = sqlx::query(
        "SELECT note_id, note_target_kind, note_target_key, note_body, \
                note_created_by, note_is_read, note_legacy_id, note_source, \
                note_created_at, note_updated_at \
           FROM ht_notes \
          WHERE ($1::text IS NULL OR note_target_kind = $1) \
            AND ($2::text IS NULL OR note_target_key = $2) \
            AND ($3::bool IS NULL OR note_is_read = $3) \
          ORDER BY note_is_read ASC, note_created_at DESC, note_id DESC",
    )
    .bind(kind)
    .bind(&target)
    .bind(is_read_filter)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to list notes: {e}")))?;

    let mut notes = Vec::with_capacity(rows.len());
    let mut unread_count = 0i64;
    for row in &rows {
        let dto = row_to_dto(row)?;
        if !dto.is_read {
            unread_count += 1;
        }
        notes.push(dto);
    }

    Ok(Json(NoteListResponse {
        success: true,
        notes,
        unread_count,
    }))
}

/// `POST /api/notes` — add a sticky note. Inserts a canonical
/// `note_source='app'` row on the resolved per-site pool, and (when
/// `NOTES_WRITEBACK_ENABLED`) enqueues the legacy-mirror writeback in the same
/// transaction.
pub async fn create_note(
    State(state): State<AppState>,
    Query(query): Query<NoteBranchQuery>,
    Json(body): Json<CreateNoteRequest>,
) -> ApiResult<(StatusCode, Json<NoteMutationResponse>)> {
    let pool = resolve_pool(&state, query.branch)?;

    let target_key = body.target_key.trim().to_string();
    let note_body = body.body.trim().to_string();
    if target_key.is_empty() {
        return Err(ApiError::BadRequest("targetKey is required".to_string()));
    }
    if note_body.is_empty() {
        return Err(ApiError::BadRequest("body is required".to_string()));
    }
    let created_by = body.created_by.clone().unwrap_or_default();
    let aggregate_id = Uuid::new_v4();

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to open tx: {e}")))?;

    let row = sqlx::query(
        "INSERT INTO ht_notes ( \
             note_target_kind, note_target_key, note_body, note_created_by, \
             note_source, aggregate_id \
         ) VALUES ($1, $2, $3, $4, 'app', $5) \
         RETURNING note_id, note_target_kind, note_target_key, note_body, \
                   note_created_by, note_is_read, note_legacy_id, note_source, \
                   note_created_at, note_updated_at",
    )
    .bind(body.target_kind.as_str())
    .bind(&target_key)
    .bind(&note_body)
    .bind(if created_by.is_empty() { None } else { Some(&created_by) })
    .bind(aggregate_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to create note: {e}")))?;

    let dto = row_to_dto(&row)?;

    // Coexistence writeback — SHIPPED DARK behind NOTES_WRITEBACK_ENABLED.
    if notes_writeback_enabled() {
        let intent = WritebackIntent::CreateNote {
            note_aggregate_id: aggregate_id,
            target_kind: body.target_kind,
            target_key: target_key.clone(),
            body: note_body.clone(),
            created_by: created_by.clone(),
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to enqueue note writeback: {e}")))?;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to commit note: {e}")))?;

    Ok((
        StatusCode::CREATED,
        Json(NoteMutationResponse { success: true, note: dto }),
    ))
}

/// `POST /api/notes/{id}/read` — mark a note read. Flips `note_is_read` and
/// (when `NOTES_WRITEBACK_ENABLED`) enqueues the legacy `SMS_Readed='yes'`
/// writeback in the same transaction. Idempotent.
pub async fn mark_read(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
    Query(query): Query<NoteBranchQuery>,
) -> ApiResult<Json<NoteMutationResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to open tx: {e}")))?;

    // Stamp an aggregate_id if the row lacks one (legacy-synced notes are
    // mirrored without one) so the writeback resolver can find it by
    // aggregate_id. COALESCE preserves an existing id.
    let fresh_aggregate = Uuid::new_v4();
    let row = sqlx::query(
        "UPDATE ht_notes SET \
             note_is_read = TRUE, \
             aggregate_id = COALESCE(aggregate_id, $2), \
             note_updated_at = NOW() \
         WHERE note_id = $1 \
         RETURNING note_id, note_target_kind, note_target_key, note_body, \
                   note_created_by, note_is_read, note_legacy_id, note_source, \
                   note_created_at, note_updated_at, aggregate_id",
    )
    .bind(note_id)
    .bind(fresh_aggregate)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to mark note read: {e}")))?
    .ok_or_else(|| ApiError::NotFound(format!("note {note_id} not found")))?;

    let dto = row_to_dto(&row)?;
    let aggregate_id: Uuid = row
        .try_get("aggregate_id")
        .map_err(|e| ApiError::Internal(format!("aggregate_id missing after update: {e}")))?;

    // Coexistence writeback — SHIPPED DARK behind NOTES_WRITEBACK_ENABLED.
    if notes_writeback_enabled() {
        let intent = WritebackIntent::MarkNoteRead {
            note_aggregate_id: aggregate_id,
            target_kind: parse_kind(&dto.target_kind),
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to enqueue note-read writeback: {e}")))?;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to commit note read: {e}")))?;

    Ok(Json(NoteMutationResponse { success: true, note: dto }))
}

#[cfg(test)]
mod tests {
    //! Pure deserialization + helper tests (run without a database). The
    //! end-to-end DB behavior is covered by the writeback recipe tests
    //! (`writeback::recipes::sticky_note`) and the sync poll.

    use super::*;

    /// The create request accepts the documented camelCase shape and parses
    /// `targetKind` into the `NoteTargetKind` enum.
    #[test]
    fn create_request_deserializes_camel_case() {
        let body = r#"{
            "targetKind": "room",
            "targetKey": "402",
            "body": "ห้องน้ำเสีย",
            "createdBy": "reception_a"
        }"#;
        let parsed: CreateNoteRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.target_kind, NoteTargetKind::Room);
        assert_eq!(parsed.target_key, "402");
        assert_eq!(parsed.body, "ห้องน้ำเสีย");
        assert_eq!(parsed.created_by.as_deref(), Some("reception_a"));
    }

    /// `staff` target kind round-trips and `createdBy` is optional.
    #[test]
    fn create_request_staff_kind_and_optional_author() {
        let body = r#"{ "targetKind": "staff", "targetKey": "somchai", "body": "ฝากเวร" }"#;
        let parsed: CreateNoteRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.target_kind, NoteTargetKind::Staff);
        assert!(parsed.created_by.is_none());
    }

    /// `parse_kind` round-trips the canonical column strings.
    #[test]
    fn parse_kind_round_trips() {
        assert_eq!(parse_kind("room"), NoteTargetKind::Room);
        assert_eq!(parse_kind("staff"), NoteTargetKind::Staff);
        // Defensive default (unreachable given the CHECK domain).
        assert_eq!(parse_kind("???"), NoteTargetKind::Staff);
    }
}
