//! In-app reception verification form API (คู่มือตรวจสอบระบบใหม่) — task #58 / migration 066.
//!
//! - `POST /api/verification` — submit one completed checklist; returns its id.
//! - `GET  /api/verification` — list recent submissions (for IT review).
//!
//! The ONLINE equivalent of the printed checklist
//! `docs/coexistence/reception-verification-TH.html` (§1 screen-compare, §2 policy
//! questions, §3 last-5-bills total check, §4 coordinated live tests, §5 overall
//! readiness). Answers land in PostgreSQL so IT can query them (e.g.
//! `vr_answers->>'q2_1'`) instead of reading photos of paper forms.
//!
//! PG-CANONICAL ONLY: this is app-internal operational data — iHOTEL has NO
//! counterpart, so there is NO sync mapper and NO legacy writeback (unlike
//! [`crate::routes::new_notes`]). Every answer is stored verbatim in the
//! `vr_answers` JSONB column keyed by question id, which keeps the form flexible
//! (questions can change without a migration). All queries are LITERAL strings
//! bound with `.bind()` (runtime `sqlx::query`, no `query!` macro / no `.sqlx`
//! cache).
//!
//! Branch-aware: `?branch=hfville` reads/writes the `hotelville` canonical pool
//! via the unified per-site write chokepoint ([`AppState::write_pool`]), otherwise
//! the primary `hotelnew` pool — the same per-site resolution as `new_notes`.
//! (While `HFVILLE_WRITES_ENABLED` is off, the `ville_write_guard` middleware
//! 403s a `branch=hfville` POST before it reaches this handler — so the frontend
//! gates the submit button to writable branches, like every other /v2 write page.)

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::domain::user::User;
use crate::error::{ApiError, ApiResult};

// =============================================================================
// Query / request / response shapes
// =============================================================================

/// Branch + paging selector for both endpoints.
#[derive(Debug, Deserialize)]
pub struct VerificationQuery {
    pub branch: Option<Branch>,
    /// GET only: max rows to return (default 50, clamped to 1..=200).
    pub limit: Option<i64>,
}

/// Body for `POST /api/verification`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVerificationRequest {
    /// ผู้ตรวจ — inspector name. Optional; defaults to the session user when auth
    /// is on (see [`super::resolve_actor`]).
    #[serde(default)]
    pub inspector: Option<String>,
    /// Every choice/text answer keyed by question id (q1_1, q1_2_reasons, …).
    /// Must be a non-empty JSON object.
    pub answers: serde_json::Value,
    /// §5 overall readiness verdict ('a' | 'b' | 'c'). Optional.
    #[serde(default)]
    pub overall: Option<String>,
}

/// One stored response in `GET /api/verification`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationDto {
    pub id: i64,
    pub submitted_at: DateTime<Utc>,
    pub site: Option<String>,
    pub inspector: Option<String>,
    pub answers: serde_json::Value,
    pub overall: Option<String>,
}

/// `200` body for `GET /api/verification`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationListResponse {
    pub success: bool,
    pub responses: Vec<VerificationDto>,
}

/// `201` body for a successful `POST /api/verification`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVerificationResponse {
    pub success: bool,
    pub id: i64,
    pub submitted_at: DateTime<Utc>,
}

// =============================================================================
// Helpers
// =============================================================================

/// Resolve the per-site pool for a branch via the unified write chokepoint
/// (HF Ville → ville pool, else primary). Used by BOTH the read and the write
/// handler so the `Hfville → ville_pool` decision lives in exactly one place —
/// the same delegation pattern as [`crate::routes::new_notes::resolve_pool`].
fn resolve_pool(state: &AppState, branch: Option<Branch>) -> ApiResult<&crate::db::PgPool> {
    state.write_pool(branch)
}

/// Stable site string for `vr_site` (also the per-site routing key).
fn branch_site(branch: Option<Branch>) -> &'static str {
    match branch.unwrap_or_default() {
        Branch::Hfville => "hfville",
        Branch::All => "all",
        Branch::Hfhotel => "hfhotel",
    }
}

/// Map a decode failure on a response row to a 500 (schema drift, not a client bug).
fn map_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map verification row: {e}"))
}

/// Build a [`VerificationDto`] from a `ht_verification_responses` row.
fn row_to_dto(row: &sqlx::postgres::PgRow) -> ApiResult<VerificationDto> {
    Ok(VerificationDto {
        id: row.try_get("vr_id").map_err(map_row_err)?,
        submitted_at: row.try_get("vr_submitted_at").map_err(map_row_err)?,
        site: row.try_get("vr_site").map_err(map_row_err)?,
        inspector: row.try_get("vr_inspector").map_err(map_row_err)?,
        answers: row.try_get("vr_answers").map_err(map_row_err)?,
        overall: row.try_get("vr_overall").map_err(map_row_err)?,
    })
}

// =============================================================================
// Handlers
// =============================================================================

/// `POST /api/verification` — save one completed checklist. Branch-aware; inserts
/// a single canonical row on the resolved per-site pool and returns the new id.
pub async fn create_verification(
    State(state): State<AppState>,
    Query(query): Query<VerificationQuery>,
    // Task #40 convention: prefer the authenticated inspector over the body value
    // (auth ships dark, so this is `None` and the body value wins). Must precede
    // the `Json` body extractor.
    actor: Option<Extension<User>>,
    Json(body): Json<CreateVerificationRequest>,
) -> ApiResult<(StatusCode, Json<CreateVerificationResponse>)> {
    let pool = resolve_pool(&state, query.branch)?;

    // Reject an empty submission so we never store a meaningless row. The column
    // is NOT NULL; require a non-empty JSON object of answers.
    let non_empty_object = body
        .answers
        .as_object()
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    if !non_empty_object {
        return Err(ApiError::BadRequest(
            "answers must be a non-empty JSON object".to_string(),
        ));
    }

    let inspector = super::resolve_actor(actor.as_deref(), body.inspector.as_deref());
    let overall = body
        .overall
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let site = branch_site(query.branch);

    // LITERAL string; every value bound. `vr_answers` (JSONB) takes a
    // `serde_json::Value` directly — same bind shape as `ht_shifts.shift_cash_count`
    // in `service::shifts`.
    let row = sqlx::query(
        "INSERT INTO ht_verification_responses \
             (vr_site, vr_inspector, vr_answers, vr_overall) \
         VALUES ($1, $2, $3, $4) \
         RETURNING vr_id, vr_submitted_at",
    )
    .bind(site)
    .bind(inspector.as_deref())
    .bind(&body.answers)
    .bind(overall)
    .fetch_one(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to save verification response: {e}")))?;

    let id: i64 = row.try_get("vr_id").map_err(map_row_err)?;
    let submitted_at: DateTime<Utc> = row.try_get("vr_submitted_at").map_err(map_row_err)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateVerificationResponse {
            success: true,
            id,
            submitted_at,
        }),
    ))
}

/// `GET /api/verification` — list recent submissions (most-recent first) for the
/// IT review screen. Branch-aware.
pub async fn list_verifications(
    State(state): State<AppState>,
    Query(query): Query<VerificationQuery>,
) -> ApiResult<Json<VerificationListResponse>> {
    let pool = resolve_pool(&state, query.branch)?;
    let limit = query.limit.unwrap_or(50).clamp(1, 200);

    let rows = sqlx::query(
        "SELECT vr_id, vr_submitted_at, vr_site, vr_inspector, vr_answers, vr_overall \
           FROM ht_verification_responses \
          ORDER BY vr_submitted_at DESC, vr_id DESC \
          LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to list verification responses: {e}")))?;

    let responses = rows
        .iter()
        .map(row_to_dto)
        .collect::<ApiResult<Vec<_>>>()?;

    Ok(Json(VerificationListResponse {
        success: true,
        responses,
    }))
}

#[cfg(test)]
mod tests {
    //! Pure deserialization + helper tests (run without a database). The
    //! end-to-end DB behavior is exercised by the integration suite.

    use super::*;

    /// The create request accepts the documented camelCase shape and keeps the
    /// answers as an arbitrary JSON object.
    #[test]
    fn create_request_deserializes_camel_case_with_json_answers() {
        let body = r#"{
            "inspector": "Nok",
            "answers": {
                "q1_1": "match",
                "q1_2": "mismatch",
                "q1_2_reasons": ["cash", "card"],
                "q2_1": "a",
                "q3_bills": [{ "no": "B001", "ihotel": "1200", "newsys": "1200", "match": "match" }],
                "q5": "a"
            },
            "overall": "a"
        }"#;
        let parsed: CreateVerificationRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.inspector.as_deref(), Some("Nok"));
        assert_eq!(parsed.overall.as_deref(), Some("a"));
        assert!(parsed.answers.is_object());
        assert_eq!(parsed.answers["q1_1"], serde_json::json!("match"));
        assert_eq!(parsed.answers["q1_2_reasons"], serde_json::json!(["cash", "card"]));
    }

    /// `inspector` and `overall` are optional; `answers` is the only required field.
    #[test]
    fn create_request_inspector_and_overall_optional() {
        let body = r#"{ "answers": { "q5": "c" } }"#;
        let parsed: CreateVerificationRequest = serde_json::from_str(body).expect("must parse");
        assert!(parsed.inspector.is_none());
        assert!(parsed.overall.is_none());
        assert_eq!(parsed.answers["q5"], serde_json::json!("c"));
    }

    /// Each branch maps to its stable `vr_site` string; unset defaults to HF Hotel.
    #[test]
    fn branch_site_maps_each_branch() {
        assert_eq!(branch_site(Some(Branch::Hfhotel)), "hfhotel");
        assert_eq!(branch_site(Some(Branch::Hfville)), "hfville");
        assert_eq!(branch_site(Some(Branch::All)), "all");
        assert_eq!(branch_site(None), "hfhotel");
    }
}
