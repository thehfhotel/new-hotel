//! Data-driven feedback / re-verification form definitions (Tier 1, migration 067).
//!
//! - `GET /api/feedback/forms`        — list active form definitions (optionally
//!   `?site=hfhotel|hfville`), most-prominent first. The generic frontend renderer
//!   builds a form from the returned `schema` (the question list), so editing a
//!   question is a DB write to `ht_feedback_forms` — no frontend rebuild/deploy.
//! - `GET /api/feedback/forms/{key}`  — one form definition by `form_key`.
//!
//! Submitted answers still go to `POST /api/verification` (see
//! [`crate::routes::new_verification`]) and land in `ht_verification_responses`,
//! tagged with `kind = form_kind`. This module is READ-ONLY (Tier 1 edits the
//! schema via a seed/DB write; an admin-editor UI is deferred to Tier 2).
//!
//! PG-CANONICAL ONLY: app-internal config, no iHOTEL counterpart — no sync, no
//! writeback. Served from the PRIMARY pool (`state.new_pool`): the form schema is
//! global config (seeded in both logical DBs by the migration, read from primary),
//! the same model `new_verification` uses for the responses store. All queries are
//! LITERAL strings bound with `.bind()` (runtime `sqlx::query`, no `.sqlx` cache).

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};

/// Query selector for the list endpoint.
#[derive(Debug, Deserialize)]
pub struct FeedbackFormQuery {
    /// Restrict to forms for one site. A form with `form_site` NULL or `'all'`
    /// is always included (site-agnostic). Omit to return every active form.
    pub site: Option<String>,
}

/// One form definition returned to the renderer.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackFormDto {
    pub key: String,
    pub site: Option<String>,
    pub kind: String,
    pub title: String,
    pub intro: Option<String>,
    /// The question list (JSONB) — rendered generically by the frontend.
    pub schema: serde_json::Value,
    pub sort: i32,
}

/// `200` body for the list endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackFormListResponse {
    pub success: bool,
    pub forms: Vec<FeedbackFormDto>,
}

fn map_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map feedback form row: {e}"))
}

fn row_to_dto(row: &sqlx::postgres::PgRow) -> ApiResult<FeedbackFormDto> {
    Ok(FeedbackFormDto {
        key: row.try_get("form_key").map_err(map_row_err)?,
        site: row.try_get("form_site").map_err(map_row_err)?,
        kind: row.try_get("form_kind").map_err(map_row_err)?,
        title: row.try_get("form_title").map_err(map_row_err)?,
        intro: row.try_get("form_intro").map_err(map_row_err)?,
        schema: row.try_get("form_schema").map_err(map_row_err)?,
        sort: row.try_get("form_sort").map_err(map_row_err)?,
    })
}

/// `GET /api/feedback/forms?site=` — active form definitions. Site filter keeps
/// site-agnostic forms (NULL / 'all') visible everywhere.
pub async fn list_feedback_forms(
    State(state): State<AppState>,
    Query(query): Query<FeedbackFormQuery>,
) -> ApiResult<Json<FeedbackFormListResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
        "SELECT form_key, form_site, form_kind, form_title, form_intro, \
                form_schema, form_sort \
           FROM ht_feedback_forms \
          WHERE form_active = TRUE \
            AND ($1::text IS NULL OR form_site IS NULL OR form_site = 'all' OR form_site = $1) \
          ORDER BY form_sort ASC, form_key ASC",
    )
    .bind(query.site.as_deref())
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to list feedback forms: {e}")))?;

    let forms = rows.iter().map(row_to_dto).collect::<ApiResult<Vec<_>>>()?;
    Ok(Json(FeedbackFormListResponse {
        success: true,
        forms,
    }))
}

/// `GET /api/feedback/forms/{key}` — one active form definition by `form_key`.
pub async fn get_feedback_form(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> ApiResult<Json<FeedbackFormDto>> {
    let pool = &state.new_pool;

    let row = sqlx::query(
        "SELECT form_key, form_site, form_kind, form_title, form_intro, \
                form_schema, form_sort \
           FROM ht_feedback_forms \
          WHERE form_active = TRUE AND form_key = $1",
    )
    .bind(&key)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to read feedback form: {e}")))?
    .ok_or_else(|| ApiError::NotFound(format!("feedback form '{key}' not found")))?;

    Ok(Json(row_to_dto(&row)?))
}
