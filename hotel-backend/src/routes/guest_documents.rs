//! Guest documents API (check-in registration Phase 2) — `ht_guest_documents`.
//!
//! - `POST /api/guest-documents`            — store a captured document (Thai ID
//!   card / passport / face photo) as `bytea`; return `{ docId, tmpNo }`.
//! - `GET  /api/checkins/{id}/documents`    — list a folio's document metadata
//!   (NO blob).
//! - `GET  /api/guest-documents/{docId}`    — stream one document's raw bytes.
//!
//! Branch-aware via `?branch=` (each logical DB holds one site's documents, so
//! pool selection *is* the site filter — same idiom as [`crate::routes::new_notes`]).
//! ALL queries are runtime `sqlx::query(...)` bound with `.bind()` (no `query!`
//! macro / no `.sqlx` cache) — required because `ht_guest_documents` and its
//! `bytea` column are outside the offline cache.
//!
//! ## Legacy mirror (SHIPPED DARK — `GUEST_DOCUMENT_STORAGE_ENABLED`, default off)
//!
//! When the flag is on, `POST` enqueues a [`WritebackIntent::MirrorGuestImage`]
//! in the same transaction so the writeback worker mirrors the image into the
//! legacy `Tb_Save_Image` table (provisional row keyed by `tmp_no`; the check-in
//! writeback stamps `cin_no`/`cust_no` later). When off, the document is stored
//! canonically only — no legacy write (invariant #6: new legacy writes ship dark
//! pending a reception-coordinated live test).

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use base64::Engine as _;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Row as _;
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::outbox::{generate_idempotency_key, OutboxRepository, WritebackIntent};
use crate::render::thai_id_card::{render_card, ThaiIdCardFields};

/// The canonical `doc_type` values this endpoint accepts. Mirrored to the legacy
/// `Tb_Save_Image.ttype` Thai literal by `writeback::recipes::save_image`.
const VALID_DOC_TYPES: [&str; 3] = ["thai_id_card", "passport", "face_photo"];

/// Branch selector (`?branch=`) for the guest-document endpoints.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocBranchQuery {
    pub branch: Option<Branch>,
}

/// Body for `POST /api/guest-documents`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGuestDocumentRequest {
    /// `thai_id_card` | `passport` | `face_photo`.
    pub doc_type: String,
    /// MIME type; defaults to `image/jpeg`.
    #[serde(default)]
    pub mime: Option<String>,
    /// Base64-encoded image bytes (standard alphabet, with or without padding).
    pub image_base64: String,
    /// Capture source (`chip` | `scanner` | `webcam`). Free text; optional.
    #[serde(default)]
    pub source: Option<String>,
    /// Optional canonical customer id this document belongs to.
    #[serde(default)]
    pub cust_id: Option<i32>,
    /// Optional canonical check-in (folio) id this document belongs to.
    #[serde(default)]
    pub cin_id: Option<i32>,
}

/// `200` body for a successful `POST /api/guest-documents`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGuestDocumentResponse {
    pub success: bool,
    pub doc_id: i32,
    /// Provisional `Tb_Save_Image.tmp_no` — pass this back as `photoTmpNo` on
    /// the check-in POST so the check-in writeback links the photo.
    pub tmp_no: String,
}

/// One document's metadata (NO blob) for the folio-documents list.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestDocumentMeta {
    pub doc_id: i32,
    pub cust_id: Option<i32>,
    pub cin_id: Option<i32>,
    pub doc_type: String,
    pub mime: String,
    pub source: Option<String>,
    pub tmp_no: Option<String>,
    pub legacy_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

/// `200` body for `GET /api/checkins/{id}/documents`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestDocumentListResponse {
    pub success: bool,
    pub documents: Vec<GuestDocumentMeta>,
}

/// Resolve the per-site pool (HF Ville → ville pool, else primary) via the
/// unified write chokepoint — same as `routes::new_notes::resolve_pool`.
fn resolve_pool<'a>(
    state: &'a AppState,
    branch: Option<Branch>,
) -> ApiResult<&'a crate::db::PgPool> {
    state.write_pool(branch)
}

/// `POST /api/guest-documents` — store a captured document and (when
/// `GUEST_DOCUMENT_STORAGE_ENABLED`) enqueue the legacy `Tb_Save_Image` mirror.
pub async fn create_guest_document(
    State(state): State<AppState>,
    Query(query): Query<DocBranchQuery>,
    Json(body): Json<CreateGuestDocumentRequest>,
) -> ApiResult<(StatusCode, Json<CreateGuestDocumentResponse>)> {
    let pool = resolve_pool(&state, query.branch)?;

    let doc_type = body.doc_type.trim();
    if !VALID_DOC_TYPES.contains(&doc_type) {
        return Err(ApiError::BadRequest(format!(
            "invalid docType '{doc_type}' (expected one of {VALID_DOC_TYPES:?})"
        )));
    }
    let mime = body
        .mime
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("image/jpeg")
        .to_string();

    // Decode the base64 image → bytea. Tolerate whitespace/newlines from a
    // data-URL-less client by stripping them first.
    let cleaned: String = body
        .image_base64
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    if cleaned.is_empty() {
        return Err(ApiError::BadRequest(
            "imageBase64 must not be empty".to_string(),
        ));
    }
    let image = base64::engine::general_purpose::STANDARD
        .decode(cleaned.as_bytes())
        .map_err(|e| ApiError::BadRequest(format!("imageBase64 is not valid base64: {e}")))?;
    if image.is_empty() {
        return Err(ApiError::BadRequest("decoded image is empty".to_string()));
    }

    // Provisional Tb_Save_Image.tmp_no linkage key (compact, unique).
    let tmp_no = Uuid::new_v4().simple().to_string();
    let source = body
        .source
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to open tx: {e}")))?;

    // Dynamic sqlx — bytea bound via `.bind(Vec<u8>)`.
    let row = sqlx::query(
        "INSERT INTO ht_guest_documents ( \
             doc_cust_id, doc_cin_id, doc_type, doc_mime, doc_image, doc_source, \
             doc_legacy_tmp_no \
         ) VALUES ($1, $2, $3, $4, $5, $6, $7) \
         RETURNING doc_id",
    )
    .bind(body.cust_id)
    .bind(body.cin_id)
    .bind(doc_type)
    .bind(&mime)
    .bind(&image)
    .bind(source)
    .bind(&tmp_no)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to store guest document: {e}")))?;
    let doc_id: i32 = row
        .try_get("doc_id")
        .map_err(|e| ApiError::Internal(format!("doc_id missing after insert: {e}")))?;

    // Legacy mirror — SHIPPED DARK behind GUEST_DOCUMENT_STORAGE_ENABLED.
    if crate::config::guest_document_storage_enabled() {
        // Best-effort diagnostics — the provisional Tb_Save_Image row is written
        // with empty cust_no/cin_no regardless (the check-in writeback stamps
        // them later by tmp_no), so a NULL here is harmless. Literal SQL bound
        // with `.bind()` (runtime `sqlx::query_scalar`, no `query!` macro).
        let cust_legacy_no: Option<String> = match body.cust_id {
            Some(id) => sqlx::query_scalar(
                "SELECT NULLIF(legacy_cust_no, '') FROM ht_customers WHERE cust_id = $1",
            )
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to resolve legacy cust_no: {e}")))?
            .flatten(),
            None => None,
        };
        let cin_legacy_no: Option<String> = match body.cin_id {
            Some(id) => sqlx::query_scalar(
                "SELECT NULLIF(legacy_cin_no, '') FROM ht_checkins WHERE cin_id = $1",
            )
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to resolve legacy cin_no: {e}")))?
            .flatten(),
            None => None,
        };

        let intent = WritebackIntent::MirrorGuestImage {
            doc_id: doc_id as i64,
            doc_type: doc_type.to_string(),
            cust_legacy_no,
            cin_legacy_no,
            tmp_no: tmp_no.clone(),
        };
        let key = generate_idempotency_key(&intent, Uuid::new_v4());
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to enqueue image writeback: {e}")))?;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::Internal(format!("failed to commit guest document: {e}")))?;

    Ok((
        StatusCode::CREATED,
        Json(CreateGuestDocumentResponse {
            success: true,
            doc_id,
            tmp_no,
        }),
    ))
}

/// `GET /api/checkins/{id}/documents` — list a folio's document metadata (no blob).
pub async fn list_checkin_documents(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Query(query): Query<DocBranchQuery>,
) -> ApiResult<Json<GuestDocumentListResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    let rows = sqlx::query(
        "SELECT doc_id, doc_cust_id, doc_cin_id, doc_type, doc_mime, doc_source, \
                doc_legacy_tmp_no, doc_legacy_id, doc_created_at \
           FROM ht_guest_documents \
          WHERE doc_cin_id = $1 \
          ORDER BY doc_created_at ASC, doc_id ASC",
    )
    .bind(cin_id)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to list guest documents: {e}")))?;

    let documents = rows
        .iter()
        .map(|row| {
            Ok(GuestDocumentMeta {
                doc_id: row.try_get("doc_id").map_err(map_row_err)?,
                cust_id: row.try_get("doc_cust_id").map_err(map_row_err)?,
                cin_id: row.try_get("doc_cin_id").map_err(map_row_err)?,
                doc_type: row.try_get("doc_type").map_err(map_row_err)?,
                mime: row.try_get("doc_mime").map_err(map_row_err)?,
                source: row.try_get("doc_source").map_err(map_row_err)?,
                tmp_no: row.try_get("doc_legacy_tmp_no").map_err(map_row_err)?,
                legacy_id: row.try_get("doc_legacy_id").map_err(map_row_err)?,
                created_at: row.try_get("doc_created_at").map_err(map_row_err)?,
            })
        })
        .collect::<ApiResult<Vec<_>>>()?;

    Ok(Json(GuestDocumentListResponse {
        success: true,
        documents,
    }))
}

/// `GET /api/guest-documents/{docId}` — stream one document's raw bytes with its
/// stored MIME type.
pub async fn get_guest_document(
    State(state): State<AppState>,
    Path(doc_id): Path<i32>,
    Query(query): Query<DocBranchQuery>,
) -> ApiResult<Response> {
    let pool = resolve_pool(&state, query.branch)?;

    let row = sqlx::query("SELECT doc_mime, doc_image FROM ht_guest_documents WHERE doc_id = $1")
        .bind(doc_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to load guest document: {e}")))?
        .ok_or_else(|| ApiError::NotFound(format!("guest document {doc_id} not found")))?;

    let mime: String = row.try_get("doc_mime").map_err(map_row_err)?;
    let image: Vec<u8> = row.try_get("doc_image").map_err(map_row_err)?;

    // Content-Length is set automatically by hyper from the known-size body.
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        // Private: a guest ID/passport scan must never be shared-cached.
        .header(header::CACHE_CONTROL, "private, no-store")
        .body(Body::from(image))
        .map_err(|e| ApiError::Internal(format!("failed to build image response: {e}")))
}

/// Body for `POST /api/guest-documents/render-thai-id`.
///
/// A superset of the raw card fields plus the chip face photo (base64 JPEG) and
/// the optional canonical linkage ids. All name/date/address fields are
/// optional (default blank) — a partial chip read still composites a card with
/// whatever it captured.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderThaiIdRequest {
    pub cid: String,
    #[serde(default)]
    pub thai_title: String,
    #[serde(default)]
    pub thai_first_name: String,
    #[serde(default)]
    pub thai_last_name: String,
    #[serde(default)]
    pub english_title: String,
    #[serde(default)]
    pub english_first_name: String,
    #[serde(default)]
    pub english_last_name: String,
    #[serde(default)]
    pub date_of_birth: String,
    #[serde(default)]
    pub issue_date: String,
    #[serde(default)]
    pub address: String,
    /// Base64-encoded chip face photo (JPEG; standard alphabet, padding
    /// optional, whitespace tolerated).
    pub photo_base64: String,
    #[serde(default)]
    pub cust_id: Option<i32>,
    #[serde(default)]
    pub cin_id: Option<i32>,
}

/// `POST /api/guest-documents/render-thai-id` — composite the reconstructed Thai
/// national-ID card (template background + chip face photo + shaped text)
/// server-side and store it in `ht_guest_documents` so a check-in captured via
/// OUR card-reader gets the same style of card iHOTEL prints.
///
/// Storage reuses the [`create_guest_document`] path/columns exactly, with the
/// values fixed for a rendered card (`doc_type='thai_id_card'`,
/// `doc_source='chip'`, `doc_mime='image/png'`). PG-CANONICAL ONLY — no legacy
/// `Tb_Save_Image` writeback is enqueued here (invariant #6: this endpoint is
/// verification/shadow-only; the raw-photo `POST /api/guest-documents` path owns
/// the dark legacy mirror).
///
/// Degradation contract: a *render* failure (template/font asset absent or
/// unreadable, SVG parse/raster failure) returns **HTTP 200** with
/// `{success:false, reason:'render_unavailable'}` — NOT a 5xx — so the frontend
/// cleanly falls back to uploading the raw face photo. Genuine client errors
/// (missing/invalid `photoBase64`) still return 4xx; a DB failure still 500s.
pub async fn render_thai_id_card(
    State(state): State<AppState>,
    Query(query): Query<DocBranchQuery>,
    Json(body): Json<RenderThaiIdRequest>,
) -> ApiResult<Json<Value>> {
    let pool = resolve_pool(&state, query.branch)?;

    // Decode the chip face photo (JPEG). Strip whitespace like the raw-upload
    // path so a client that pretty-prints its base64 still works.
    let cleaned: String = body
        .photo_base64
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    if cleaned.is_empty() {
        return Err(ApiError::BadRequest(
            "photoBase64 must not be empty".to_string(),
        ));
    }
    let face_jpeg = base64::engine::general_purpose::STANDARD
        .decode(cleaned.as_bytes())
        .map_err(|e| ApiError::BadRequest(format!("photoBase64 is not valid base64: {e}")))?;
    if face_jpeg.is_empty() {
        return Err(ApiError::BadRequest("decoded photo is empty".to_string()));
    }

    let fields = ThaiIdCardFields {
        cid: body.cid.clone(),
        thai_title: body.thai_title.clone(),
        thai_first_name: body.thai_first_name.clone(),
        thai_last_name: body.thai_last_name.clone(),
        english_title: body.english_title.clone(),
        english_first_name: body.english_first_name.clone(),
        english_last_name: body.english_last_name.clone(),
        date_of_birth: body.date_of_birth.clone(),
        issue_date: body.issue_date.clone(),
        address: body.address.clone(),
    };

    let template_path = crate::config::thai_id_template_path();
    let font_path = crate::config::thai_id_font_path();

    // Composite. Any render failure (missing assets etc.) degrades to
    // success:false / HTTP 200 rather than erroring — the frontend then uploads
    // the raw face photo instead.
    let png = match render_card(&fields, &face_jpeg, &template_path, &font_path) {
        Ok(png) => png,
        Err(e) => {
            tracing::warn!("thai-id card render unavailable: {e}");
            return Ok(Json(json!({
                "success": false,
                "reason": "render_unavailable",
            })));
        }
    };

    // Store canonically — same columns as create_guest_document, fixed for a
    // rendered card. Provisional Tb_Save_Image.tmp_no minted the same way.
    let tmp_no = Uuid::new_v4().simple().to_string();
    let row = sqlx::query(
        "INSERT INTO ht_guest_documents ( \
             doc_cust_id, doc_cin_id, doc_type, doc_mime, doc_image, doc_source, \
             doc_legacy_tmp_no \
         ) VALUES ($1, $2, $3, $4, $5, $6, $7) \
         RETURNING doc_id",
    )
    .bind(body.cust_id)
    .bind(body.cin_id)
    .bind("thai_id_card")
    .bind("image/png")
    .bind(&png)
    .bind("chip")
    .bind(&tmp_no)
    .fetch_one(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to store rendered card: {e}")))?;
    let doc_id: i32 = row
        .try_get("doc_id")
        .map_err(|e| ApiError::Internal(format!("doc_id missing after insert: {e}")))?;

    Ok(Json(json!({
        "success": true,
        "docId": doc_id,
        "tmpNo": tmp_no,
    })))
}

/// Map a row-decode failure to a 500 (schema drift, not a client bug).
fn map_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map guest-document row: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_deserializes_camel_case() {
        let body = r#"{
            "docType": "passport",
            "mime": "image/png",
            "imageBase64": "aGk=",
            "source": "scanner",
            "custId": 7,
            "cinId": 42
        }"#;
        let parsed: CreateGuestDocumentRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.doc_type, "passport");
        assert_eq!(parsed.mime.as_deref(), Some("image/png"));
        assert_eq!(parsed.cust_id, Some(7));
        assert_eq!(parsed.cin_id, Some(42));
    }

    #[test]
    fn valid_doc_types_are_the_three_registration_kinds() {
        assert!(VALID_DOC_TYPES.contains(&"thai_id_card"));
        assert!(VALID_DOC_TYPES.contains(&"passport"));
        assert!(VALID_DOC_TYPES.contains(&"face_photo"));
        assert!(!VALID_DOC_TYPES.contains(&"driver_license"));
    }

    #[test]
    fn base64_roundtrips_the_standard_alphabet() {
        // "hi" → "aGk=".
        let decoded = base64::engine::general_purpose::STANDARD
            .decode("aGk=")
            .unwrap();
        assert_eq!(decoded, b"hi");
    }
}
