//! HTTP Server implementation using Axum for Thai ID Middleware
//!
//! Provides REST API endpoints for card reader status and card reading operations.
//! Listens on 127.0.0.1:9898 with CORS enabled for localhost web applications.

use axum::{
    extract::{Query, State},
    http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::card_reader::{CardData, CardReader, FullDebugInfo, set_debug_mode, is_debug_mode};

/// Server port for the HTTP API
const SERVER_PORT: u16 = 9898;

/// Default CORS allowlist when `CARD_READER_ALLOWED_ORIGINS` is unset.
/// Mirrors the backend's `BACKEND_ALLOWED_ORIGINS` default — covers the
/// Next.js dev server on the host (3003) plus the in-container `web`
/// service. Production deployments MUST set the env var explicitly with
/// the public hostname(s) of the frontend(s) allowed to read cards.
const DEFAULT_ALLOWED_ORIGINS: &str = "http://localhost:3003,http://web:3003";

/// Shared application state for Axum handlers
#[derive(Clone)]
pub struct AppState {
    pub card_reader: Arc<Mutex<CardReader>>,
}

/// Health/Status response structure
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: String,
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

/// Query parameters for the read endpoint
#[derive(Debug, Deserialize)]
pub struct ReadParams {
    /// Include photo in the response (default: false)
    #[serde(default)]
    pub photo: Option<bool>,
}

/// Start the HTTP server - runs until the application exits
///
/// # Arguments
/// * `card_reader` - Shared CardReader instance wrapped in Arc<Mutex<>>
///
/// # Returns
/// * `Ok(())` when server stops
/// * `Err(String)` if server failed to start
pub async fn start_http_server(card_reader: Arc<Mutex<CardReader>>) -> Result<(), String> {
    let state = AppState { card_reader };

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
    // `allow_credentials` stays off.
    let cors = CorsLayer::new()
        .allow_origin(parse_allowed_origins())
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE]);

    // Build the router with all endpoints
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(status_handler))
        .route("/read", get(read_handler))
        .route("/debug", get(debug_info_handler))
        .route("/debug/enable", get(enable_debug_handler))
        .route("/debug/disable", get(disable_debug_handler))
        .fallback(not_found_handler)
        .layer(cors)
        .with_state(state);

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

    println!("Read card request received (include_photo={})", include_photo);

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

/// GET /debug - Get full debug information about the card
///
/// Returns comprehensive debug info including:
/// - ATR (Answer To Reset)
/// - Protocol (T=0 or T=1)
/// - Reader name
/// - AID test results for known Thai ID card applications
/// - Raw read result
async fn debug_info_handler(
    State(state): State<AppState>,
) -> Json<FullDebugInfo> {
    println!("Debug info request received");

    let reader = state.card_reader.lock().await;
    let debug_info = reader.get_debug_info().await;

    println!("Debug info collected: ATR={:?}, Protocol={:?}",
             debug_info.atr, debug_info.protocol);

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
                "GET /debug - Get full debug info (ATR, protocol, AID tests)".to_string(),
                "GET /debug/enable - Enable debug mode".to_string(),
                "GET /debug/disable - Disable debug mode".to_string(),
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
