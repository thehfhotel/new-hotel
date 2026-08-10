//! HTTP Server implementation using Axum for Thai ID Middleware
//!
//! Provides REST API endpoints for card reader status and card reading operations.
//! Listens on 127.0.0.1:9898 with CORS enabled for localhost web applications.

use axum::{
    extract::{Query, State},
    http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::card_reader::{is_debug_mode, set_debug_mode, CardData, CardReader, FullDebugInfo};
use crate::ihotel;
use crate::stale::{Action, StaleLatch, StaleSignalPayload};
use crate::toast;

/// Title shown on every legacy-stale toast. Body text is per-episode (see
/// `crate::stale::compose_toast_text` via `Action::ShowToast`).
const TOAST_TITLE: &str = "iHOTEL";

/// Server port for the HTTP API
const SERVER_PORT: u16 = 9898;

/// Default CORS allowlist when `CARD_READER_ALLOWED_ORIGINS` is unset.
/// Covers the production frontend (`https://hotel.thehfhotel.org`, both HF
/// Hotel and HF Ville share this origin — branch is a header, not a host) plus
/// the Next.js dev server on the host (3003) and the in-container `web`
/// service. So a stock reception install reads cards from prod with no per-PC
/// env config; `CARD_READER_ALLOWED_ORIGINS` still overrides for other hosts.
const DEFAULT_ALLOWED_ORIGINS: &str =
    "https://hotel.thehfhotel.org,http://localhost:3003,http://web:3003";

/// Shared application state for Axum handlers
#[derive(Clone)]
pub struct AppState {
    pub card_reader: Arc<Mutex<CardReader>>,
    /// The grid-staleness latch (`docs/adr/0006-legacy-stale-notification.md`).
    /// Shared with `main.rs`'s tray menu so `POST /ihotel/refresh` and the
    /// tray's "รีเฟรช iHOTEL" item clear the same state.
    pub stale_latch: Arc<Mutex<StaleLatch>>,
}

/// Health/Status response structure
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: String,
    /// Crate version — lets the Stage-6 SSH install verify which build is
    /// actually serving (an in-place upgrade that silently failed would
    /// otherwise be indistinguishable from success).
    pub version: String,
    pub timestamp: String,
    pub reader_connected: bool,
    pub card_inserted: bool,
    pub server_running: bool,
    pub port: u16,
    pub reader_name: Option<String>,
}

/// Successful read response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadSuccessResponse {
    pub success: bool,
    pub data: CardData,
}

/// Error response for read failures
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadErrorResponse {
    pub success: bool,
    pub error: String,
}

/// 404 Not Found response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundResponse {
    pub error: String,
    pub available_endpoints: Vec<String>,
}

/// Debug mode response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugModeResponse {
    pub debug_mode: bool,
}

/// `GET /ihotel/status` response — a snapshot combining the live
/// process/window probe (`crate::ihotel::snapshot`, real on Windows,
/// honestly `false`/`false` everywhere else) with the grid staleness
/// latch's own state.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IhotelStatusResponse {
    pub process_found: bool,
    pub grid_window_found: bool,
    pub latch_state: &'static str,
    pub stale_count: u32,
    pub last_notify_at: Option<String>,
}

/// `POST /ihotel/refresh` response, for both outcomes.
///
/// `sent` is the whole answer: `true` means a click was posted to iHOTEL's
/// Refresh button, `false` means a guard stopped us and `reason`/`message`
/// say which and why (see `crate::ihotel::SkipReason`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub sent: bool,
    /// Stable machine-readable skip reason: `process-not-found`,
    /// `modal-open`, `selection-pending`, `grid-not-found`,
    /// `probe-timeout`, `post-failed`. Absent when `sent` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The same Thai sentence reception just saw on the toast, so a caller
    /// can echo it in-page instead of inventing a second wording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Query parameters for the read endpoint
#[derive(Debug, Deserialize)]
pub struct ReadParams {
    /// Include photo in the response (default: false)
    #[serde(default)]
    pub photo: Option<bool>,
}

/// Request body for the `/parse-mrz` endpoint.
///
/// The caller supplies the already-OCR'd 2-line TD3 MRZ string (client-side
/// tesseract.js, a dedicated scanner, or manual paste) plus an optional base64
/// image that is echoed back so the frontend can attach it to the check-in.
#[derive(Debug, Deserialize)]
pub struct ParseMrzRequest {
    /// The two MRZ lines separated by a newline (`\n` or `\r\n`).
    pub mrz: String,
    /// Optional base64-encoded passport image, echoed back verbatim.
    #[serde(default)]
    pub image: Option<String>,
}

/// Successful MRZ parse response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseMrzResponse {
    pub success: bool,
    pub passport_number: String,
    pub surname: String,
    pub given_names: String,
    pub nationality: String,
    /// ISO 8601 (YYYY-MM-DD).
    pub date_of_birth: String,
    /// `M`, `F`, or `X`.
    pub sex: String,
    /// ISO 8601 (YYYY-MM-DD).
    pub expiry_date: String,
    /// True when all TD3 check digits validate.
    pub checksum_valid: bool,
    /// Echoed-back base64 image when one was supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

/// Build the Axum router. Split out from [`start_http_server`] so tests can
/// drive it directly (via `tower::ServiceExt::oneshot`) without binding a
/// real TCP socket — see the `test_notify_requires_json_content_type` test.
fn build_router(state: AppState) -> Router {
    // Lock CORS down to a curated allowlist. The middleware is bound to
    // 127.0.0.1 but the browser still happily proxies cross-origin
    // `fetch('http://localhost:9898/read')` requests from any tab the
    // receptionist visits — which would let any malicious page exfiltrate
    // a card-on-reader. The allowlist is sourced from
    // `CARD_READER_ALLOWED_ORIGINS` (comma-separated) and defaults to the
    // legitimate frontends only. Methods are restricted to GET (every
    // handler is a GET today) plus OPTIONS for the preflight; headers are
    // restricted to `Content-Type` since none of the endpoints inspect
    // anything else. No credentials/cookies are involved, so
    // `allow_credentials` stays off. POST is allowed for `/parse-mrz` and
    // the `/notify`/`/ihotel/*` endpoints below, which take/accept a JSON
    // body (or nothing).
    let cors = CorsLayer::new()
        .allow_origin(parse_allowed_origins())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(status_handler))
        .route("/read", get(read_handler))
        .route("/parse-mrz", post(parse_mrz_handler))
        .route("/debug", get(debug_info_handler))
        .route("/debug/enable", get(enable_debug_handler))
        .route("/debug/disable", get(disable_debug_handler))
        .route("/notify", post(notify_handler))
        .route("/ihotel/status", get(ihotel_status_handler))
        .route("/ihotel/refresh", post(ihotel_refresh_handler))
        .fallback(not_found_handler)
        .layer(cors)
        .with_state(state)
}

/// Start the HTTP server - runs until the application exits
///
/// # Arguments
/// * `card_reader` - Shared CardReader instance wrapped in Arc<Mutex<>>
/// * `stale_latch` - Shared grid-staleness latch (ADR 0006), also captured
///   by `main.rs`'s tray menu so both surfaces agree on state.
///
/// # Returns
/// * `Ok(())` when server stops
/// * `Err(String)` if server failed to start
pub async fn start_http_server(
    card_reader: Arc<Mutex<CardReader>>,
    stale_latch: Arc<Mutex<StaleLatch>>,
) -> Result<(), String> {
    let state = AppState {
        card_reader,
        stale_latch,
    };

    let app = build_router(state);

    let addr = format!("127.0.0.1:{}", SERVER_PORT);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    println!("Thai ID Middleware server running on http://{}", addr);

    // Run the server - this will block until the app exits
    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server error: {}", e))
}

/// Parse the CORS allowlist from `CARD_READER_ALLOWED_ORIGINS`
/// (comma-separated) or fall back to `DEFAULT_ALLOWED_ORIGINS`. Empty
/// entries are skipped silently so trailing commas don't break startup.
/// Malformed origins panic loudly — config errors should fail at startup,
/// not at request time, so a misconfigured deployment is impossible to
/// miss instead of silently letting cards leak.
fn parse_allowed_origins() -> AllowOrigin {
    let raw = std::env::var("CARD_READER_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| DEFAULT_ALLOWED_ORIGINS.to_string());

    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            HeaderValue::from_str(origin).unwrap_or_else(|err| {
                panic!(
                    "CARD_READER_ALLOWED_ORIGINS contains a malformed origin {:?}: {}. \
                     Expected comma-separated absolute origins (e.g. \
                     'https://hotel.example.com,http://web:3003').",
                    origin, err
                )
            })
        })
        .collect();

    if origins.is_empty() {
        panic!(
            "CARD_READER_ALLOWED_ORIGINS is set but resolved to zero origins after \
             trimming. Refusing to start with an empty CORS allowlist — set the \
             env var to a comma-separated list of absolute origins, or unset it \
             to use the default ({}).",
            DEFAULT_ALLOWED_ORIGINS
        );
    }

    println!(
        "CORS allowlist: {}",
        origins
            .iter()
            .filter_map(|value| value.to_str().ok())
            .collect::<Vec<_>>()
            .join(", ")
    );

    AllowOrigin::list(origins)
}

/// GET /health - Returns server and reader status
async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let reader = state.card_reader.lock().await;
    let status = reader.get_status();

    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: iso_timestamp(),
        reader_connected: status.reader_connected,
        card_inserted: status.card_inserted,
        server_running: true,
        port: SERVER_PORT,
        reader_name: status.reader_name,
    })
}

/// GET /status - Alias for /health
async fn status_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    health_handler(State(state)).await
}

/// GET /read - Read Thai ID card data
///
/// Query parameters:
/// - `photo=true` - Include photo in the response (adds ~2 seconds)
async fn read_handler(
    State(state): State<AppState>,
    Query(params): Query<ReadParams>,
) -> Result<Json<ReadSuccessResponse>, (StatusCode, Json<ReadErrorResponse>)> {
    let include_photo = params.photo.unwrap_or(false);

    println!(
        "Read card request received (include_photo={})",
        include_photo
    );

    let mut reader = state.card_reader.lock().await;

    match reader.read_card_with_options(include_photo).await {
        Ok(data) => {
            println!("Card read successfully");
            Ok(Json(ReadSuccessResponse {
                success: true,
                data,
            }))
        }
        Err(e) => {
            println!("Read card error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ReadErrorResponse {
                    success: false,
                    error: e.to_string(),
                }),
            ))
        }
    }
}

/// POST /parse-mrz - Parse a TD3 passport MRZ string.
///
/// Body: `{ "mrz": "<line1>\n<line2>", "image": "<base64?>" }`. The MRZ must be
/// supplied by the caller (no OCR happens here). Returns the extracted fields
/// plus a `checksumValid` flag; the optional image is echoed back untouched.
/// A malformed MRZ (wrong line count / length) returns 400 with the standard
/// error shape.
async fn parse_mrz_handler(
    Json(payload): Json<ParseMrzRequest>,
) -> Result<Json<ParseMrzResponse>, (StatusCode, Json<ReadErrorResponse>)> {
    println!(
        "Parse MRZ request received (image={})",
        payload.image.is_some()
    );

    match crate::mrz::parse_td3(&payload.mrz) {
        Ok(data) => {
            println!(
                "MRZ parsed (passport={}, checksumValid={})",
                data.passport_number, data.checksum_valid
            );
            Ok(Json(ParseMrzResponse {
                success: true,
                passport_number: data.passport_number,
                surname: data.surname,
                given_names: data.given_names,
                nationality: data.nationality,
                date_of_birth: data.date_of_birth,
                sex: data.sex,
                expiry_date: data.expiry_date,
                checksum_valid: data.checksum_valid,
                image: payload.image,
            }))
        }
        Err(e) => {
            println!("Parse MRZ error: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ReadErrorResponse {
                    success: false,
                    error: e,
                }),
            ))
        }
    }
}

/// GET /debug - Get full debug information about the card
///
/// Returns comprehensive debug info including:
/// - ATR (Answer To Reset)
/// - Protocol (T=0 or T=1)
/// - Reader name
/// - AID test results for known Thai ID card applications
/// - Raw read result
async fn debug_info_handler(State(state): State<AppState>) -> Json<FullDebugInfo> {
    println!("Debug info request received");

    let reader = state.card_reader.lock().await;
    let debug_info = reader.get_debug_info().await;

    println!(
        "Debug info collected: ATR={:?}, Protocol={:?}",
        debug_info.atr, debug_info.protocol
    );

    Json(debug_info)
}

/// GET /debug/enable - Enable debug mode
async fn enable_debug_handler() -> Json<DebugModeResponse> {
    set_debug_mode(true);
    println!("Debug mode ENABLED");
    Json(DebugModeResponse {
        debug_mode: is_debug_mode(),
    })
}

/// GET /debug/disable - Disable debug mode
async fn disable_debug_handler() -> Json<DebugModeResponse> {
    set_debug_mode(false);
    println!("Debug mode DISABLED");
    Json(DebugModeResponse {
        debug_mode: is_debug_mode(),
    })
}

/// POST /notify - Feed a `legacy_stale` signal into the grid-staleness latch
///
/// Per `docs/adr/0006-legacy-stale-notification.md` §3: the reception
/// browser tab (already SSE-connected to the backend) relays the signal
/// here after receiving it. Body is the `LegacyStaleSignal` JSON shape
/// (`{id, site, count, summary, items, emitted_at}` — see
/// `hotel-backend/src/outbox/legacy_stale.rs` and `crate::stale`).
///
/// **Requires `Content-Type: application/json`.** This isn't just
/// pedantry: a "simple" cross-origin POST (e.g. `Content-Type: text/plain`,
/// or no body at all) never triggers the browser's CORS preflight, which
/// means the `CARD_READER_ALLOWED_ORIGINS` allowlist above would never even
/// be consulted for it — any page could fire one blind. Axum's `Json<T>`
/// extractor already rejects a request whose `Content-Type` isn't
/// `application/json` (or an `+json` suffix) with `415 Unsupported Media
/// Type` before this handler body ever runs (see
/// `test_notify_requires_json_content_type` below), which is what actually
/// forces the preflight and makes the allowlist gate this endpoint for
/// real.
async fn notify_handler(State(state): State<AppState>, Json(payload): Json<StaleSignalPayload>) -> StatusCode {
    let action = {
        let mut latch = state.stale_latch.lock().await;
        latch.on_notify(&payload, Utc::now())
    };
    dispatch_action(action);
    StatusCode::NO_CONTENT
}

/// GET /ihotel/status - Snapshot of iHOTEL's observed state + the latch
///
/// `processFound`/`gridWindowFound` come from `crate::ihotel::snapshot()`:
/// on Windows that is a real probe (is `HOTEL.exe` running, does its
/// Refresh button resolve); on every other target it honestly reports
/// `false`/`false` rather than guessing.
async fn ihotel_status_handler(State(state): State<AppState>) -> Json<IhotelStatusResponse> {
    let probe = ihotel::snapshot();
    let snap = {
        let latch = state.stale_latch.lock().await;
        latch.snapshot()
    };

    Json(IhotelStatusResponse {
        process_found: probe.process_found,
        grid_window_found: probe.grid_window_found,
        latch_state: snap.state,
        stale_count: snap.stale_count,
        last_notify_at: snap.last_notify_at.map(|t| t.to_rfc3339()),
    })
}

/// POST /ihotel/refresh - Trigger iHOTEL's reception-invoked refresh
///
/// The same code path the tray's "รีเฟรช iHOTEL" item calls
/// (`crate::ihotel::trigger_refresh`), so the two surfaces can never
/// disagree about what "refresh" does. Per ADR 0006 Decision §1 this is
/// always reception-invoked — no writeback signal can reach it.
///
/// **Both outcomes answer `200`.** A guard skip is not a server error: the
/// request was understood, a decision was made, and reception has already
/// been told why on a toast. Encoding "we deliberately did nothing" as a
/// 4xx/5xx would light up browser consoles for the single most common and
/// most *correct* outcome (`selection-pending` — refusing to destroy her
/// in-progress room selection). `sent` is the discriminator.
///
/// The staleness latch is cleared **only** on a real send: a skip means the
/// grid was never refreshed, so the episode is still open and reception
/// should still be told about the next writeback.
async fn ihotel_refresh_handler(State(state): State<AppState>) -> impl IntoResponse {
    match ihotel::trigger_refresh() {
        Ok(()) => {
            let mut latch = state.stale_latch.lock().await;
            latch.on_refresh_performed(Utc::now());
            (
                StatusCode::OK,
                Json(RefreshResponse {
                    sent: true,
                    reason: None,
                    message: None,
                }),
            )
                .into_response()
        }
        Err(reason) => (
            StatusCode::OK,
            Json(RefreshResponse {
                sent: false,
                reason: Some(reason.to_string()),
                message: Some(reason.toast_text().to_string()),
            }),
        )
            .into_response(),
    }
}

/// Turn a latch [`Action`] into a real toast call. Factored out of
/// `notify_handler` so its body reads as "update state, then react".
fn dispatch_action(action: Action) {
    match action {
        Action::ShowToast(text) => toast::show(TOAST_TITLE, &text),
        Action::UpdateCount(count) => {
            // No update-in-place toast API is wired yet (see `crate::toast`
            // module docs) — a count bump while a toast is already showing
            // is logged, not re-toasted, to honor the "one toast per
            // episode" rule (ADR 0006 §5).
            println!(
                "[legacy-stale] stale count now {count} (toast already shown for this episode)"
            );
        }
        Action::Nothing => {}
    }
}

/// 404 handler - Returns available endpoints
async fn not_found_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(NotFoundResponse {
            error: "Not Found".to_string(),
            available_endpoints: vec![
                "GET /health - Check server and reader status".to_string(),
                "GET /status - Alias for /health".to_string(),
                "GET /read - Read Thai ID card data".to_string(),
                "GET /read?photo=true - Read card data with photo (~3 sec)".to_string(),
                "POST /parse-mrz - Parse a TD3 passport MRZ string".to_string(),
                "GET /debug - Get full debug info (ATR, protocol, AID tests)".to_string(),
                "GET /debug/enable - Enable debug mode".to_string(),
                "GET /debug/disable - Disable debug mode".to_string(),
                "POST /notify - Feed a legacy_stale signal into the grid-staleness latch"
                    .to_string(),
                "GET /ihotel/status - iHOTEL process/window snapshot + latch state".to_string(),
                "POST /ihotel/refresh - Post a click to iHOTEL's Refresh button (guarded)"
                    .to_string(),
            ],
        }),
    )
}

/// Generate ISO 8601 timestamp in UTC
fn iso_timestamp() -> String {
    use std::time::SystemTime;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    // Format as ISO 8601 without external chrono dependency
    let secs = now.as_secs();
    let millis = now.subsec_millis();

    // Calculate date/time components
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let mins = (time_secs % 3600) / 60;
    let secs_of_min = time_secs % 60;

    // Simple date calculation
    let mut year = 1970i32;
    let mut remaining_days = days as i32;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for days_in_month in month_days.iter() {
        if remaining_days < *days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, month, day, hours, mins, secs_of_min, millis
    )
}

/// Check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn test_app_state() -> AppState {
        AppState {
            card_reader: Arc::new(Mutex::new(CardReader::new())),
            stale_latch: Arc::new(Mutex::new(StaleLatch::new())),
        }
    }

    fn stale_signal_body() -> String {
        serde_json::json!({
            "id": "9f5c1e2a-0000-0000-0000-000000000000",
            "site": "hfhotel",
            "count": 1,
            "summary": "เช็คอิน ห้อง 302",
            "items": ["เช็คอิน ห้อง 302"],
            "emitted_at": "2026-08-10T09:00:00Z"
        })
        .to_string()
    }

    #[tokio::test]
    async fn test_notify_requires_json_content_type() {
        // The load-bearing assertion in this file: a "simple" content type
        // must be rejected with 415 before the handler ever touches the
        // latch — see notify_handler's doc comment for why this is what
        // makes the CORS allowlist actually gate this endpoint.
        let app = build_router(test_app_state());

        let request = Request::builder()
            .method("POST")
            .uri("/notify")
            .header("content-type", "text/plain")
            .body(Body::from(stale_signal_body()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_notify_without_content_type_is_rejected() {
        let app = build_router(test_app_state());

        let request = Request::builder()
            .method("POST")
            .uri("/notify")
            .body(Body::from(stale_signal_body()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_notify_with_json_content_type_returns_204() {
        let app = build_router(test_app_state());

        let request = Request::builder()
            .method("POST")
            .uri("/notify")
            .header("content-type", "application/json")
            .body(Body::from(stale_signal_body()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_notify_feeds_the_latch() {
        let state = test_app_state();
        let latch = state.stale_latch.clone();
        let app = build_router(state);

        let request = Request::builder()
            .method("POST")
            .uri("/notify")
            .header("content-type", "application/json")
            .body(Body::from(stale_signal_body()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let snap = latch.lock().await.snapshot();
        assert_eq!(snap.state, "stale");
        assert_eq!(snap.stale_count, 1);
    }

    #[tokio::test]
    async fn test_ihotel_status_reports_clear_latch_and_unavailable_probe() {
        let app = build_router(test_app_state());

        let request = Request::builder()
            .method("GET")
            .uri("/ihotel/status")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["processFound"], false);
        assert_eq!(body["gridWindowFound"], false);
        assert_eq!(body["latchState"], "clear");
        assert_eq!(body["staleCount"], 0);
        assert!(body["lastNotifyAt"].is_null());
    }

    async fn post_refresh(app: Router) -> serde_json::Value {
        let request = Request::builder()
            .method("POST")
            .uri("/ihotel/refresh")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        // Both outcomes are 200 — see ihotel_refresh_handler's doc comment.
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn test_ihotel_refresh_reports_the_decision_on_the_wire() {
        // `process-not-found` is the honest answer on this crate's macOS CI
        // leg (and on any Windows box that isn't running iHOTEL), so it is
        // the one decision shape reachable from an end-to-end HTTP test.
        let body = post_refresh(build_router(test_app_state())).await;
        assert_eq!(body["sent"], false);
        assert_eq!(body["reason"], "process-not-found");
        assert!(
            body["message"].as_str().is_some_and(|m| !m.is_empty()),
            "a skip must carry the Thai explanation reception just saw"
        );
    }

    #[tokio::test]
    async fn test_ihotel_refresh_skip_does_not_clear_the_latch() {
        // The load-bearing assertion for ADR 0006 §5: nothing was
        // refreshed, so the stale episode must still be open. Clearing here
        // would silently swallow the next writeback's toast.
        let state = test_app_state();
        let latch = state.stale_latch.clone();
        let app = build_router(state);

        let request = Request::builder()
            .method("POST")
            .uri("/notify")
            .header("content-type", "application/json")
            .body(Body::from(stale_signal_body()))
            .unwrap();
        assert_eq!(
            app.clone().oneshot(request).await.unwrap().status(),
            StatusCode::NO_CONTENT
        );

        let body = post_refresh(app).await;
        assert_eq!(body["sent"], false);

        let snap = latch.lock().await.snapshot();
        assert_eq!(snap.state, "stale");
        assert_eq!(snap.stale_count, 1);
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "ok".to_string(),
            timestamp: "2024-01-01T00:00:00.000Z".to_string(),
            reader_connected: true,
            card_inserted: false,
            server_running: true,
            port: 9898,
            reader_name: Some("Test Reader".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"ok\""));
        assert!(json.contains("\"readerConnected\":true"));
        assert!(json.contains("\"cardInserted\":false"));
        assert!(json.contains("\"serverRunning\":true"));
        assert!(json.contains("\"port\":9898"));
        assert!(json.contains("\"readerName\":\"Test Reader\""));
    }

    #[test]
    fn test_read_error_response_serialization() {
        let response = ReadErrorResponse {
            success: false,
            error: "No card inserted".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"error\":\"No card inserted\""));
    }

    #[test]
    fn test_parse_mrz_response_serialization() {
        let response = ParseMrzResponse {
            success: true,
            passport_number: "L898902C3".to_string(),
            surname: "ERIKSSON".to_string(),
            given_names: "ANNA MARIA".to_string(),
            nationality: "UTO".to_string(),
            date_of_birth: "1974-08-12".to_string(),
            sex: "F".to_string(),
            expiry_date: "2012-04-15".to_string(),
            checksum_valid: true,
            image: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"passportNumber\":\"L898902C3\""));
        assert!(json.contains("\"givenNames\":\"ANNA MARIA\""));
        assert!(json.contains("\"dateOfBirth\":\"1974-08-12\""));
        assert!(json.contains("\"expiryDate\":\"2012-04-15\""));
        assert!(json.contains("\"checksumValid\":true"));
        // Image is omitted entirely when absent.
        assert!(!json.contains("\"image\""));
    }

    #[test]
    fn test_parse_mrz_request_deserialization() {
        let req: ParseMrzRequest =
            serde_json::from_str("{\"mrz\":\"L1\\nL2\",\"image\":\"AAAA\"}").unwrap();
        assert_eq!(req.mrz, "L1\nL2");
        assert_eq!(req.image, Some("AAAA".to_string()));

        // image is optional.
        let req2: ParseMrzRequest = serde_json::from_str("{\"mrz\":\"x\"}").unwrap();
        assert_eq!(req2.image, None);
    }

    #[test]
    fn test_not_found_response_serialization() {
        let response = NotFoundResponse {
            error: "Not Found".to_string(),
            available_endpoints: vec![
                "GET /health".to_string(),
                "GET /status".to_string(),
                "GET /read".to_string(),
            ],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"error\":\"Not Found\""));
        assert!(json.contains("\"availableEndpoints\":"));
    }

    #[test]
    fn test_iso_timestamp_format() {
        let timestamp = iso_timestamp();
        // Should match ISO 8601 format: YYYY-MM-DDTHH:MM:SS.sssZ
        assert!(timestamp.ends_with('Z'));
        assert!(timestamp.contains('T'));
        assert!(timestamp.contains('-'));
        assert!(timestamp.contains(':'));
        assert_eq!(timestamp.len(), 24); // "2024-01-01T00:00:00.000Z"
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2000)); // Divisible by 400
        assert!(!is_leap_year(1900)); // Divisible by 100 but not 400
        assert!(is_leap_year(2024)); // Divisible by 4 but not 100
        assert!(!is_leap_year(2023)); // Not divisible by 4
    }

    #[test]
    fn test_read_params_default() {
        let params: ReadParams = serde_json::from_str("{}").unwrap();
        assert_eq!(params.photo, None);
    }

    #[test]
    fn test_read_params_with_photo() {
        let params: ReadParams = serde_json::from_str("{\"photo\": true}").unwrap();
        assert_eq!(params.photo, Some(true));
    }

    #[test]
    fn test_default_allowed_origins_constant() {
        // Default must cover the legitimate frontends only — never `*`.
        // Sentinel test so a future "let's just open it back up" change
        // trips a failing assertion.
        assert_eq!(
            DEFAULT_ALLOWED_ORIGINS,
            "http://localhost:3003,http://web:3003"
        );
        assert!(!DEFAULT_ALLOWED_ORIGINS.contains('*'));
    }

    #[test]
    fn test_parse_allowed_origins_uses_default_when_unset() {
        // Avoid leaking env state into other tests in this process.
        std::env::remove_var("CARD_READER_ALLOWED_ORIGINS");
        // Should not panic — default origins are valid HeaderValues.
        let _ = parse_allowed_origins();
    }

    #[test]
    fn test_parse_allowed_origins_accepts_custom_list() {
        std::env::set_var(
            "CARD_READER_ALLOWED_ORIGINS",
            "https://hotel.example.com, http://localhost:3003",
        );
        // Should not panic — both are valid origins; whitespace is trimmed.
        let _ = parse_allowed_origins();
        std::env::remove_var("CARD_READER_ALLOWED_ORIGINS");
    }
}
