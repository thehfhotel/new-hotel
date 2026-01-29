use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::card_reader::{
    self, CardData, CardReaderState, FullDebugInfo,
    set_debug_mode, is_debug_mode, read_card_with_options, get_debug_info,
};
use serde::{Deserialize, Serialize};

/// Application state wrapper for Tauri
/// Uses a Mutex to serialize access to card reader operations
pub struct AppState {
    /// Mutex to prevent concurrent card operations (smart cards are sequential devices)
    pub card_lock: Arc<Mutex<()>>,
}

/// Response structure for read_card command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadCardResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Get the current status of the card reader and card
#[tauri::command]
pub async fn get_status(_state: State<'_, AppState>) -> Result<CardReaderState, String> {
    // Status check doesn't need the lock as it's read-only
    Ok(card_reader::get_status())
}

/// Get the application version
#[tauri::command]
pub async fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Read data from the Thai ID card
///
/// # Arguments
/// * `include_photo` - Optional flag to include photo data (default: false)
#[tauri::command]
pub async fn read_card(
    state: State<'_, AppState>,
    include_photo: Option<bool>,
) -> Result<ReadCardResponse, String> {
    // Acquire lock to prevent concurrent card operations
    let _lock = state.card_lock.lock().await;

    let include_photo = include_photo.unwrap_or(false);

    // Run blocking card read operation in a separate thread
    let result = tokio::task::spawn_blocking(move || read_card_with_options(include_photo)).await;

    match result {
        Ok(Ok(data)) => Ok(ReadCardResponse {
            success: true,
            data: Some(data),
            error: None,
        }),
        Ok(Err(e)) => Ok(ReadCardResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
        Err(e) => Ok(ReadCardResponse {
            success: false,
            data: None,
            error: Some(format!("Task join error: {}", e)),
        }),
    }
}

/// Get full debug information about the card including ATR, protocol, and AID test results
#[tauri::command]
pub async fn debug_card(state: State<'_, AppState>) -> Result<FullDebugInfo, String> {
    // Acquire lock to prevent concurrent card operations
    let _lock = state.card_lock.lock().await;

    // Run blocking debug info collection in a separate thread
    let result = tokio::task::spawn_blocking(get_debug_info).await;

    match result {
        Ok(debug_info) => Ok(debug_info),
        Err(e) => Ok(FullDebugInfo {
            atr: None,
            protocol: None,
            reader_name: None,
            aid_results: vec![],
            raw_read_result: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            error: Some(format!("Task join error: {}", e)),
        }),
    }
}

/// Enable or disable debug mode for verbose logging
#[tauri::command]
pub async fn set_debug(enabled: bool) -> bool {
    set_debug_mode(enabled);
    is_debug_mode()
}

/// Get current debug mode status
#[tauri::command]
pub async fn get_debug() -> bool {
    is_debug_mode()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_card_response_serialization() {
        let response = ReadCardResponse {
            success: true,
            data: None,
            error: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        // Optional fields should be skipped when None
        assert!(!json.contains("\"data\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_read_card_response_with_error() {
        let response = ReadCardResponse {
            success: false,
            data: None,
            error: Some("No card inserted".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"error\":\"No card inserted\""));
    }
}
