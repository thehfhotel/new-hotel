//! Track G8 — RR.4 Thai immigration export HTTP route.
//!
//! `GET /api/new/reports/rr4?from=YYYY-MM-DD&to=YYYY-MM-DD&site=...&format=xlsx|csv`
//!
//! Returns a binary attachment (xlsx by default, csv when requested)
//! with `Content-Disposition: attachment` so browsers trigger a save
//! dialog directly. The bytes are exactly what the regulator receives;
//! a SHA-256 of those same bytes is written to `ht_rr4_exports` for
//! audit non-repudiation.

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::Deserialize;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::service::reports::{Rr4ExportFormat, Rr4ExportRequest, Rr4Service};

/// Inbound query string. `site` is operator-selected; `format`
/// defaults to xlsx (regulator delivery format).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rr4Query {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub site: Option<String>,
    #[serde(default)]
    pub format: Option<Rr4ExportFormat>,
}

/// `GET /api/new/reports/rr4` — emit the RR.4 / ตม.30 export.
pub async fn get_rr4_export(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<Rr4Query>,
) -> ApiResult<impl IntoResponse> {
    let from = parse_date(&params.from, "from")?;
    let to = parse_date(&params.to, "to")?;
    let format = params.format.unwrap_or_default();
    let site = params
        .site
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("hfhotel")
        .to_string();
    let exported_by = exported_by_from_headers(&headers);

    let service = Rr4Service::new(state.new_pool.clone());
    let result = service
        .export(Rr4ExportRequest {
            site,
            from,
            to,
            format,
            exported_by,
        })
        .await?;

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(result.mime_type),
    );
    response_headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", result.filename))
            .map_err(|e| ApiError::Internal(format!("invalid content-disposition: {e}")))?,
    );
    // Surface the SHA-256 of the emitted bytes so callers (and the
    // frontend) can prove byte-exact match with the audit row.
    response_headers.insert(
        "X-RR4-File-Hash",
        HeaderValue::from_str(&result.file_hash)
            .map_err(|e| ApiError::Internal(format!("invalid hash header: {e}")))?,
    );
    response_headers.insert(
        "X-RR4-Row-Count",
        HeaderValue::from(result.row_count),
    );

    Ok((StatusCode::OK, response_headers, result.bytes))
}

/// Parse an inbound `YYYY-MM-DD` date string. Empty / malformed values
/// surface as `400 Bad Request`.
fn parse_date(raw: &str, field: &str) -> ApiResult<NaiveDate> {
    if raw.trim().is_empty() {
        return Err(ApiError::BadRequest(format!(
            "query parameter `{field}` is required (YYYY-MM-DD)"
        )));
    }
    NaiveDate::parse_from_str(raw.trim(), "%Y-%m-%d")
        .map_err(|e| ApiError::BadRequest(format!("invalid `{field}` date `{raw}`: {e}")))
}

/// Best-effort identification of the operator triggering the export.
///
/// The auth middleware (Phase 4 PR2) stamps the username into a
/// downstream request extension, but the route signature here keeps
/// the dependency minimal by also accepting an `X-Exported-By` header
/// (used in tests + curl convenience). Falls back to `'system'`.
fn exported_by_from_headers(headers: &HeaderMap) -> String {
    if let Some(hv) = headers.get("x-exported-by").and_then(|h| h.to_str().ok()) {
        let trimmed = hv.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    "system".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_rejects_empty() {
        let err = parse_date("", "from").unwrap_err();
        match err {
            ApiError::BadRequest(msg) => assert!(msg.contains("`from`")),
            _ => panic!("expected BadRequest"),
        }
    }

    #[test]
    fn parse_date_rejects_malformed() {
        let err = parse_date("2026/05/13", "from").unwrap_err();
        match err {
            ApiError::BadRequest(_) => {}
            _ => panic!("expected BadRequest for `2026/05/13`"),
        }
    }

    #[test]
    fn parse_date_accepts_valid_iso() {
        let d = parse_date("2026-05-13", "from").expect("valid iso must parse");
        assert_eq!(d, NaiveDate::from_ymd_opt(2026, 5, 13).unwrap());
    }

    #[test]
    fn exported_by_defaults_to_system_when_header_missing() {
        let h = HeaderMap::new();
        assert_eq!(exported_by_from_headers(&h), "system");
    }

    #[test]
    fn exported_by_reads_header_when_present() {
        let mut h = HeaderMap::new();
        h.insert(
            "x-exported-by",
            HeaderValue::from_static("receptionist-1"),
        );
        assert_eq!(exported_by_from_headers(&h), "receptionist-1");
    }

    #[test]
    fn rr4_query_format_optional_defaults_to_xlsx_on_service() {
        // Confirms the Default impl is the regulator-default.
        let format: Rr4ExportFormat = None.unwrap_or_default();
        assert_eq!(format, Rr4ExportFormat::Xlsx);
    }
}
