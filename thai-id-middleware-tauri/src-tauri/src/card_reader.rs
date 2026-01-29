//! PC/SC Card Reader Implementation for Thai National ID Cards
//!
//! This module provides functionality to read Thai National ID cards using PC/SC smart card readers.
//! It handles APDU communication, TIS-620/UTF-8 encoding, and retry logic for cold-inserted cards.

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use pcsc::{Card, Context, Protocols, Scope, ShareMode, State};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use thiserror::Error;

// ============================================================================
// Debug Mode
// ============================================================================

/// Global debug flag - set to true to enable verbose logging
static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

/// Enable debug mode for card reader operations
pub fn set_debug_mode(enabled: bool) {
    DEBUG_MODE.store(enabled, Ordering::SeqCst);
}

/// Check if debug mode is enabled
pub fn is_debug_mode() -> bool {
    DEBUG_MODE.load(Ordering::SeqCst)
}

/// Debug print macro - only prints if DEBUG_MODE is true
macro_rules! debug_print {
    ($($arg:tt)*) => {
        if DEBUG_MODE.load(std::sync::atomic::Ordering::SeqCst) {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

// ============================================================================
// APDU Commands for Thai National ID Card
// ============================================================================

/// Thai National ID Card Application ID
const SELECT_APPLET: &[u8] = &[
    0x00, 0xA4, 0x04, 0x00, 0x08, 0xA0, 0x00, 0x00, 0x00, 0x54, 0x48, 0x00, 0x01,
];

/// Read Citizen ID (13 digits)
const READ_CID: &[u8] = &[0x80, 0xB0, 0x00, 0x04, 0x02, 0x00, 0x0D];

/// Read Thai Full Name
const READ_TH_FULLNAME: &[u8] = &[0x80, 0xB0, 0x00, 0x11, 0x02, 0x00, 0x64];

/// Read English Full Name
const READ_EN_FULLNAME: &[u8] = &[0x80, 0xB0, 0x00, 0x75, 0x02, 0x00, 0x64];

/// Read Date of Birth (YYYYMMDD)
const READ_DOB: &[u8] = &[0x80, 0xB0, 0x00, 0xD9, 0x02, 0x00, 0x08];

/// Read Gender (1=Male, 2=Female)
const READ_GENDER: &[u8] = &[0x80, 0xB0, 0x00, 0xE1, 0x02, 0x00, 0x01];

/// Read Address
const READ_ADDRESS: &[u8] = &[0x80, 0xB0, 0x15, 0x79, 0x02, 0x00, 0x64];

/// Read Issue Date (YYYYMMDD)
const READ_ISSUE_DATE: &[u8] = &[0x80, 0xB0, 0x01, 0x67, 0x02, 0x00, 0x08];

/// Read Expire Date (YYYYMMDD)
const READ_EXPIRE_DATE: &[u8] = &[0x80, 0xB0, 0x01, 0x6F, 0x02, 0x00, 0x08];

/// GET RESPONSE command template (length is filled in dynamically)
const GET_RESPONSE_BASE: &[u8] = &[0x00, 0xC0, 0x00, 0x00];

/// Photo reading commands - 20 parts
/// Each part reads 255 bytes using CLA=0x80, INS=0xB0
/// The (P1, P2) addresses specify the photo data segments
const PHOTO_PARTS: [(u8, u8); 20] = [
    (0x01, 0x7B), (0x02, 0x7A), (0x03, 0x79), (0x04, 0x78),
    (0x05, 0x77), (0x06, 0x76), (0x07, 0x75), (0x08, 0x74),
    (0x09, 0x73), (0x0A, 0x72), (0x0B, 0x71), (0x0C, 0x70),
    (0x0D, 0x6F), (0x0E, 0x6E), (0x0F, 0x6D), (0x10, 0x6C),
    (0x11, 0x6B), (0x12, 0x6A), (0x13, 0x69), (0x14, 0x68),
];

/// Known Application IDs (AIDs) to test during debug
pub const KNOWN_AIDS: [(&str, &[u8]); 4] = [
    ("TH-NID Standard", &[0xA0, 0x00, 0x00, 0x00, 0x54, 0x48, 0x00, 0x01]),
    ("TH-NID Alternate", &[0xA0, 0x00, 0x00, 0x00, 0x54, 0x48, 0x00, 0x02]),
    ("TH-MOI", &[0xA0, 0x00, 0x00, 0x00, 0x84, 0x54, 0x48, 0x00, 0x01]),
    ("EMV Payment", &[0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10]),
];

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during card reading operations
#[derive(Error, Debug)]
pub enum CardReaderError {
    #[error("PC/SC context initialization failed: {0}")]
    ContextInitFailed(String),

    #[error("No card reader connected")]
    NoReaderConnected,

    #[error("No card inserted")]
    NoCardInserted,

    #[error("Failed to connect to card: {0}")]
    ConnectionFailed(String),

    #[error("Failed to select Thai ID applet. Is this a Thai National ID card?")]
    AppletSelectionFailed,

    #[error("Card communication error: {0}")]
    TransmitError(String),

    #[error("Invalid response from card: SW={sw1:02X}{sw2:02X}")]
    InvalidResponse { sw1: u8, sw2: u8 },

    #[error("Data parsing error: {0}")]
    ParseError(String),

    #[error("Connection failed after {0} attempts: {1}")]
    RetryExhausted(u32, String),
}

impl From<pcsc::Error> for CardReaderError {
    fn from(err: pcsc::Error) -> Self {
        CardReaderError::TransmitError(err.to_string())
    }
}

// ============================================================================
// Data Structures
// ============================================================================

/// Current state of the card reader
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardReaderState {
    pub reader_connected: bool,
    pub card_inserted: bool,
    pub reader_name: Option<String>,
    pub card_atr: Option<String>,
}

impl Default for CardReaderState {
    fn default() -> Self {
        Self {
            reader_connected: false,
            card_inserted: false,
            reader_name: None,
            card_atr: None,
        }
    }
}

/// Raw name data from the card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawNames {
    pub thai_full_name: String,
    pub english_full_name: String,
}

/// Complete card data read from Thai National ID card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub cid: String,
    pub thai_title: String,
    pub thai_first_name: String,
    pub thai_last_name: String,
    pub english_title: String,
    pub english_first_name: String,
    pub english_last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
    pub issue_date: String,
    pub expire_date: String,
    /// Base64-encoded JPEG photo (optional, only included when requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,
    pub raw: RawNames,
}

/// Result of testing an Application ID (AID)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AidTestResult {
    pub name: String,
    pub aid: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Raw APDU read result for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawReadResult {
    pub command: String,
    pub command_hex: String,
    pub response: String,
    pub sw1: u8,
    pub sw2: u8,
}

/// Full debug information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullDebugInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_name: Option<String>,
    pub aid_results: Vec<AidTestResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_read_result: Option<RawReadResult>,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ============================================================================
// Global State
// ============================================================================

lazy_static::lazy_static! {
    static ref READER_STATE: Mutex<CardReaderState> = Mutex::new(CardReaderState::default());
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert bytes to hex string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("")
}

/// Parse TIS-620/UTF-8 encoded Thai text from card response
///
/// Thai ID cards use TIS-620 encoding for Thai text, but many modern cards
/// use UTF-8. This function handles both cases and cleans up the text.
pub fn parse_thai_string(buffer: &[u8]) -> String {
    if buffer.len() < 2 {
        return String::new();
    }

    // Remove status bytes (last 2 bytes)
    let data = &buffer[..buffer.len() - 2];

    // Try UTF-8 first (modern cards)
    let mut text = match std::str::from_utf8(data) {
        Ok(s) => s.to_string(),
        Err(_) => {
            // Fall back to TIS-620 to UTF-8 conversion
            tis620_to_utf8(data)
        }
    };

    // Clean up: remove nulls, replace # with space, normalize whitespace
    text = text.replace('\0', "");
    text = text.replace('#', " ");
    text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    text.trim().to_string()
}

/// Convert TIS-620 encoded bytes to UTF-8 string
///
/// TIS-620 is a Thai character encoding where Thai characters are in the range 0xA1-0xFB.
/// Each TIS-620 character maps to a Unicode code point in the range U+0E01-U+0E5B.
fn tis620_to_utf8(data: &[u8]) -> String {
    let mut result = String::new();

    for &byte in data {
        if byte < 0x80 {
            // ASCII range
            result.push(byte as char);
        } else if byte >= 0xA1 && byte <= 0xFB {
            // TIS-620 Thai character range
            // TIS-620 0xA1 = Unicode U+0E01 (Thai character KO KAI)
            let unicode_point = 0x0E01 + (byte as u32 - 0xA1);
            if let Some(c) = char::from_u32(unicode_point) {
                result.push(c);
            }
        } else if byte >= 0x80 && byte < 0xA1 {
            // Extended ASCII / control range - skip or replace
            result.push(' ');
        }
        // else: skip invalid bytes
    }

    result
}

/// Parse ASCII text from card response
pub fn parse_ascii_string(buffer: &[u8]) -> String {
    if buffer.len() < 2 {
        return String::new();
    }

    // Remove status bytes (last 2 bytes)
    let data = &buffer[..buffer.len() - 2];

    // Convert to ASCII, replacing non-ASCII with spaces
    let mut text: String = data
        .iter()
        .map(|&b| {
            if b == 0 {
                ' '
            } else if b < 128 {
                b as char
            } else {
                ' '
            }
        })
        .collect();

    // Clean up: replace # with space, normalize whitespace
    text = text.replace('#', " ");
    text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    text.trim().to_string()
}

/// Parse date from YYYYMMDD format to YYYY-MM-DD
pub fn parse_date(buffer: &[u8]) -> String {
    if buffer.len() < 10 {
        // Need at least 8 bytes + 2 status bytes
        return String::new();
    }

    // Remove status bytes and convert to string
    let data = &buffer[..buffer.len() - 2];
    let date_str: String = data
        .iter()
        .take(8)
        .filter_map(|&b| {
            if b.is_ascii_digit() {
                Some(b as char)
            } else {
                None
            }
        })
        .collect();

    if date_str.len() >= 8 {
        let year = &date_str[0..4];
        let month = &date_str[4..6];
        let day = &date_str[6..8];
        format!("{}-{}-{}", year, month, day)
    } else {
        date_str
    }
}

/// Split full name into parts (title, first name, last name)
fn split_name(full_name: &str) -> (String, String, String) {
    let parts: Vec<&str> = full_name.split_whitespace().collect();
    match parts.len() {
        0 => (String::new(), String::new(), String::new()),
        1 => (parts[0].to_string(), String::new(), String::new()),
        2 => (parts[0].to_string(), parts[1].to_string(), String::new()),
        _ => (
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2..].join(" "),
        ),
    }
}

/// Convert gender code to readable string
fn parse_gender(gender_code: &str) -> String {
    match gender_code.trim() {
        "1" => "Male".to_string(),
        "2" => "Female".to_string(),
        other => other.to_string(),
    }
}

// ============================================================================
// Card Communication Functions
// ============================================================================

/// Transmit APDU command and handle SW1=0x61 (more data available) responses
///
/// When SW1=0x61, the card has more data available. SW2 indicates how many bytes
/// can be retrieved using a GET RESPONSE command.
fn transmit_with_get_response(card: &Card, command: &[u8]) -> Result<Vec<u8>, CardReaderError> {
    if is_debug_mode() {
        let cmd_hex: String = command.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
        debug_print!("Transmitting: {}", cmd_hex);
    }

    let mut recv_buffer = vec![0u8; 258];
    let response = card.transmit(command, &mut recv_buffer)?;

    // Accumulate response data
    let mut full_response: Vec<u8> = response.to_vec();

    // Check for SW1=0x61 (more data available)
    while full_response.len() >= 2 {
        let sw1 = full_response[full_response.len() - 2];
        let sw2 = full_response[full_response.len() - 1];

        if sw1 == 0x61 {
            debug_print!("SW=61{:02X}, sending GET RESPONSE for {} bytes", sw2, sw2);
            // Build GET RESPONSE command with expected length
            let get_response_cmd = [GET_RESPONSE_BASE, &[sw2]].concat();

            recv_buffer = vec![0u8; 258];
            let response = card.transmit(&get_response_cmd, &mut recv_buffer)?;

            // Remove old status word and append new response
            let data_len = full_response.len() - 2;
            full_response.truncate(data_len);
            full_response.extend_from_slice(response);
        } else {
            break;
        }
    }

    if is_debug_mode() {
        let resp_hex: String = full_response.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
        let display = if resp_hex.len() > 60 { format!("{}...", &resp_hex[..60]) } else { resp_hex };
        debug_print!("Response ({} bytes): {}", full_response.len(), display);
    }

    Ok(full_response)
}

/// Attempt to connect to the card with retry logic
///
/// Handles cold-inserted cards that need time to initialize after insertion.
/// Uses 5 retries with 1000ms delay (5 seconds total).
fn connect_with_retry(
    ctx: &Context,
    reader_name: &std::ffi::CStr,
    max_retries: u32,
    delay_ms: u64,
) -> Result<Card, CardReaderError> {
    let mut last_error = String::from("Unknown error");

    for attempt in 1..=max_retries {
        debug_print!("connect_with_retry: attempt {}/{} for reader {:?}", attempt, max_retries, reader_name);

        // Thai ID cards use T=0 protocol - try this first for best compatibility
        match ctx.connect(reader_name, ShareMode::Shared, Protocols::T0) {
            Ok(card) => {
                debug_print!("Successfully connected (Shared/T0) on attempt {}", attempt);
                return Ok(card);
            }
            Err(err) => {
                last_error = err.to_string();
                debug_print!("Shared/T0 failed: {}", last_error);
            }
        }

        // Fall back to T=1 protocol
        match ctx.connect(reader_name, ShareMode::Shared, Protocols::T1) {
            Ok(card) => {
                debug_print!("Successfully connected (Shared/T1) on attempt {}", attempt);
                return Ok(card);
            }
            Err(err) => {
                last_error = err.to_string();
                debug_print!("Shared/T1 failed: {}", last_error);
            }
        }

        // Fall back to ANY protocol
        match ctx.connect(reader_name, ShareMode::Shared, Protocols::ANY) {
            Ok(card) => {
                debug_print!("Successfully connected (Shared/ANY) on attempt {}", attempt);
                return Ok(card);
            }
            Err(err) => {
                last_error = err.to_string();
                debug_print!("Shared/ANY failed: {}", last_error);
            }
        }

        if attempt < max_retries {
            debug_print!("Waiting {}ms before retry...", delay_ms);
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }

    Err(CardReaderError::RetryExhausted(max_retries, last_error))
}

/// Connect to card and return both card and protocol string
fn connect_with_retry_and_protocol(
    ctx: &Context,
    reader_name: &std::ffi::CStr,
    max_retries: u32,
    delay_ms: u64,
) -> Result<(Card, String), CardReaderError> {
    let mut last_error = String::from("Unknown error");

    for attempt in 1..=max_retries {
        debug_print!("connect_with_retry_and_protocol: attempt {}/{}", attempt, max_retries);

        // Thai ID cards use T=0 protocol - try this first for best compatibility
        match ctx.connect(reader_name, ShareMode::Shared, Protocols::T0) {
            Ok(card) => {
                debug_print!("Successfully connected (Shared/T0) on attempt {}", attempt);
                return Ok((card, "T=0".to_string()));
            }
            Err(err) => {
                last_error = err.to_string();
                debug_print!("Shared/T0 failed: {}", last_error);
            }
        }

        // Fall back to T=1 protocol
        match ctx.connect(reader_name, ShareMode::Shared, Protocols::T1) {
            Ok(card) => {
                debug_print!("Successfully connected (Shared/T1) on attempt {}", attempt);
                return Ok((card, "T=1".to_string()));
            }
            Err(err) => {
                last_error = err.to_string();
                debug_print!("Shared/T1 failed: {}", last_error);
            }
        }

        // Fall back to ANY protocol
        match ctx.connect(reader_name, ShareMode::Shared, Protocols::ANY) {
            Ok(card) => {
                debug_print!("Successfully connected (Shared/ANY) on attempt {}", attempt);
                return Ok((card, "T=0/T=1".to_string()));
            }
            Err(err) => {
                last_error = err.to_string();
                debug_print!("Shared/ANY failed: {}", last_error);
            }
        }

        if attempt < max_retries {
            debug_print!("Waiting {}ms before retry...", delay_ms);
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }

    Err(CardReaderError::RetryExhausted(max_retries, last_error))
}

/// Read photo from Thai ID card (20 parts, ~5KB JPEG)
///
/// Returns base64-encoded JPEG image data
fn read_photo_from_card(card: &Card) -> Result<String, CardReaderError> {
    debug_print!("read_photo_from_card: reading photo data (20 parts)");

    let mut photo_data: Vec<u8> = Vec::with_capacity(5100); // 20 * 255 bytes max

    for (i, (p1, p2)) in PHOTO_PARTS.iter().enumerate() {
        // Build APDU command: CLA=0x80, INS=0xB0, P1, P2, Lc=0x02, Data=0x00, Le=0xFF
        let cmd = [0x80, 0xB0, *p1, *p2, 0x02, 0x00, 0xFF];

        debug_print!("Reading photo part {}/20", i + 1);

        let response = transmit_with_get_response(card, &cmd)?;

        if response.len() < 2 {
            debug_print!("Photo part {} response too short: {} bytes", i + 1, response.len());
            continue;
        }

        let sw1 = response[response.len() - 2];
        let sw2 = response[response.len() - 1];

        if sw1 != 0x90 || sw2 != 0x00 {
            debug_print!("Photo part {} failed: SW={:02X}{:02X}", i + 1, sw1, sw2);
            // Some cards may have shorter photos, so we stop on error
            break;
        }

        // Append data without status bytes
        let data = &response[..response.len() - 2];
        photo_data.extend_from_slice(data);
    }

    // Trim trailing null bytes (padding)
    while photo_data.last() == Some(&0x00) {
        photo_data.pop();
    }

    debug_print!("Photo data read: {} bytes", photo_data.len());

    if photo_data.is_empty() {
        return Err(CardReaderError::ParseError("No photo data received".to_string()));
    }

    // Encode as base64
    let base64_photo = BASE64.encode(&photo_data);

    Ok(base64_photo)
}

/// Test a specific AID on the card
fn test_aid(card: &Card, name: &str, aid: &[u8]) -> AidTestResult {
    // Build SELECT command: CLA=0x00, INS=0xA4, P1=0x04, P2=0x00, Lc=len, AID bytes
    let mut cmd = vec![0x00, 0xA4, 0x04, 0x00, aid.len() as u8];
    cmd.extend_from_slice(aid);

    let aid_hex = bytes_to_hex(aid);
    debug_print!("Testing AID {}: {}", name, aid_hex);

    let mut recv_buffer = vec![0u8; 258];
    match card.transmit(&cmd, &mut recv_buffer) {
        Ok(response) => {
            if response.len() < 2 {
                return AidTestResult {
                    name: name.to_string(),
                    aid: aid_hex,
                    success: false,
                    status_word: None,
                    response: None,
                    description: None,
                    error: Some("Response too short".to_string()),
                };
            }

            let sw1 = response[response.len() - 2];
            let sw2 = response[response.len() - 1];
            let sw = format!("{:02X}{:02X}", sw1, sw2);
            // SW1=0x90 is success, SW1=0x61 means more data available (also success)
            let success = sw1 == 0x90 || sw1 == 0x61;

            let description = match (sw1, sw2) {
                (0x90, 0x00) => Some("Success".to_string()),
                (0x61, _) => Some(format!("More data available ({} bytes)", sw2)),
                (0x6A, 0x82) => Some("File or application not found".to_string()),
                (0x6A, 0x86) => Some("Incorrect P1P2".to_string()),
                (0x6D, 0x00) => Some("Instruction not supported".to_string()),
                (0x6E, 0x00) => Some("Class not supported".to_string()),
                _ => Some(format!("Unknown status: {:02X}{:02X}", sw1, sw2)),
            };

            let response_hex = if response.len() > 2 {
                Some(bytes_to_hex(&response[..response.len() - 2]))
            } else {
                None
            };

            AidTestResult {
                name: name.to_string(),
                aid: aid_hex,
                success,
                status_word: Some(sw),
                response: response_hex,
                description,
                error: None,
            }
        }
        Err(e) => AidTestResult {
            name: name.to_string(),
            aid: aid_hex,
            success: false,
            status_word: None,
            response: None,
            description: None,
            error: Some(e.to_string()),
        },
    }
}

/// Test all known AIDs on the card
fn test_known_aids(card: &Card) -> Vec<AidTestResult> {
    KNOWN_AIDS
        .iter()
        .map(|(name, aid)| test_aid(card, name, aid))
        .collect()
}

// ============================================================================
// Public API
// ============================================================================

/// Get current status of card reader and card
///
/// Returns information about:
/// - Whether a reader is connected
/// - Whether a card is inserted
/// - Reader name
/// - Card ATR (Answer To Reset)
pub fn get_status() -> CardReaderState {
    // Try to establish PC/SC context and check for readers/cards
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(e) => {
            log::warn!("Failed to establish PC/SC context: {}", e);
            return CardReaderState::default();
        }
    };

    // Get list of readers
    let readers_buf_len = match ctx.list_readers_len() {
        Ok(len) => len,
        Err(_) => return CardReaderState::default(),
    };

    let mut readers_buf = vec![0u8; readers_buf_len];
    let readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(_) => return CardReaderState::default(),
    };

    // Check if we have any readers
    let reader_names: Vec<_> = readers.collect();
    if reader_names.is_empty() {
        return CardReaderState::default();
    }

    let reader_name = reader_names[0];
    let reader_name_str = reader_name.to_string_lossy().to_string();

    // Check card status
    let mut reader_states = vec![pcsc::ReaderState::new(reader_name, State::UNAWARE)];

    if ctx
        .get_status_change(Duration::from_millis(100), &mut reader_states)
        .is_err()
    {
        return CardReaderState {
            reader_connected: true,
            card_inserted: false,
            reader_name: Some(reader_name_str),
            card_atr: None,
        };
    }

    let state = &reader_states[0];
    let card_inserted = state.event_state().contains(State::PRESENT);

    // Get ATR if card is present
    let card_atr = if card_inserted {
        let atr = state.atr();
        if atr.is_empty() {
            None
        } else {
            Some(bytes_to_hex(atr))
        }
    } else {
        None
    };

    // Update global state
    let new_state = CardReaderState {
        reader_connected: true,
        card_inserted,
        reader_name: Some(reader_name_str),
        card_atr,
    };

    if let Ok(mut state) = READER_STATE.lock() {
        *state = new_state.clone();
    }

    new_state
}

/// Read all data from Thai National ID card
///
/// This function:
/// 1. Establishes PC/SC context
/// 2. Connects to the card (with retry for cold-inserted cards)
/// 3. Selects the Thai ID applet
/// 4. Reads all personal data fields
/// 5. Optionally reads the photo
/// 6. Parses and returns the data
///
/// # Arguments
/// * `include_photo` - Whether to read the photo (adds ~2 seconds)
///
/// # Errors
///
/// Returns an error if:
/// - No card reader is connected
/// - No card is inserted
/// - The card is not a Thai National ID card
/// - Communication with the card fails
pub fn read_card() -> Result<CardData, CardReaderError> {
    read_card_with_options(false)
}

/// Read all data from Thai National ID card with options
///
/// # Arguments
/// * `include_photo` - Whether to read the photo (adds ~2 seconds)
pub fn read_card_with_options(include_photo: bool) -> Result<CardData, CardReaderError> {
    debug_print!("read_card: starting card read operation (include_photo={})", include_photo);

    // Establish PC/SC context
    let ctx = Context::establish(Scope::User)
        .map_err(|e| {
            debug_print!("Failed to establish PC/SC context: {}", e);
            CardReaderError::ContextInitFailed(e.to_string())
        })?;

    debug_print!("PC/SC context established");

    // Get list of readers
    let readers_buf_len = ctx
        .list_readers_len()
        .map_err(|e| {
            debug_print!("Failed to get readers list length: {:?}", e);
            CardReaderError::NoReaderConnected
        })?;

    let mut readers_buf = vec![0u8; readers_buf_len];
    let readers = ctx
        .list_readers(&mut readers_buf)
        .map_err(|e| {
            debug_print!("Failed to list readers: {:?}", e);
            CardReaderError::NoReaderConnected
        })?;

    let reader_names: Vec<_> = readers.collect();
    debug_print!("Found {} reader(s)", reader_names.len());
    if reader_names.is_empty() {
        return Err(CardReaderError::NoReaderConnected);
    }

    let reader_name = reader_names[0];
    debug_print!("Using reader: {:?}", reader_name);

    // Connect to card with retry logic (5 retries, 500ms delay)
    let card = connect_with_retry(&ctx, reader_name, 5, 500)?;

    debug_print!("Connected to card, selecting applet...");

    // Select Thai ID applet
    let select_response = transmit_with_get_response(&card, SELECT_APPLET)?;

    if select_response.len() < 2 {
        debug_print!("Applet selection response too short: {} bytes", select_response.len());
        return Err(CardReaderError::AppletSelectionFailed);
    }

    let sw1 = select_response[select_response.len() - 2];
    let sw2 = select_response[select_response.len() - 1];

    if sw1 != 0x90 || sw2 != 0x00 {
        debug_print!("Applet selection failed: SW={:02X}{:02X}", sw1, sw2);
        return Err(CardReaderError::AppletSelectionFailed);
    }

    debug_print!("Applet selected successfully, reading data...");

    // Read all data fields
    let cid_response = transmit_with_get_response(&card, READ_CID)?;
    let th_name_response = transmit_with_get_response(&card, READ_TH_FULLNAME)?;
    let en_name_response = transmit_with_get_response(&card, READ_EN_FULLNAME)?;
    let dob_response = transmit_with_get_response(&card, READ_DOB)?;
    let gender_response = transmit_with_get_response(&card, READ_GENDER)?;
    let address_response = transmit_with_get_response(&card, READ_ADDRESS)?;
    let issue_date_response = transmit_with_get_response(&card, READ_ISSUE_DATE)?;
    let expire_date_response = transmit_with_get_response(&card, READ_EXPIRE_DATE)?;

    // Parse responses
    let cid = parse_ascii_string(&cid_response);
    let thai_full_name = parse_thai_string(&th_name_response);
    let english_full_name = parse_ascii_string(&en_name_response);
    let date_of_birth = parse_date(&dob_response);
    let gender_code = parse_ascii_string(&gender_response);
    let address = parse_thai_string(&address_response);
    let issue_date = parse_date(&issue_date_response);
    let expire_date = parse_date(&expire_date_response);

    // Split names
    let (thai_title, thai_first_name, thai_last_name) = split_name(&thai_full_name);
    let (english_title, english_first_name, english_last_name) = split_name(&english_full_name);

    // Convert gender
    let gender = parse_gender(&gender_code);

    // Optionally read photo
    let photo = if include_photo {
        debug_print!("Reading photo...");
        match read_photo_from_card(&card) {
            Ok(photo_data) => {
                debug_print!("Photo read successfully ({} chars base64)", photo_data.len());
                Some(photo_data)
            }
            Err(e) => {
                debug_print!("Failed to read photo: {}", e);
                // Don't fail the entire read if photo fails
                None
            }
        }
    } else {
        None
    };

    log::info!("read_card: successfully read card data for CID: {}", cid);

    Ok(CardData {
        cid,
        thai_title,
        thai_first_name,
        thai_last_name,
        english_title,
        english_first_name,
        english_last_name,
        date_of_birth,
        gender,
        address,
        issue_date,
        expire_date,
        photo,
        raw: RawNames {
            thai_full_name,
            english_full_name,
        },
    })
}

/// Get full debug information including ATR, protocol, and AID tests
pub fn get_debug_info() -> FullDebugInfo {
    debug_print!("get_debug_info: starting debug info collection");

    let timestamp = chrono::Utc::now().to_rfc3339();

    // Establish PC/SC context
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(e) => {
            return FullDebugInfo {
                atr: None,
                protocol: None,
                reader_name: None,
                aid_results: vec![],
                raw_read_result: None,
                timestamp,
                error: Some(format!("Failed to establish PC/SC context: {}", e)),
            };
        }
    };

    // Get list of readers
    let readers_buf_len = match ctx.list_readers_len() {
        Ok(len) => len,
        Err(e) => {
            return FullDebugInfo {
                atr: None,
                protocol: None,
                reader_name: None,
                aid_results: vec![],
                raw_read_result: None,
                timestamp,
                error: Some(format!("Failed to get readers list: {}", e)),
            };
        }
    };

    let mut readers_buf = vec![0u8; readers_buf_len];
    let readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(e) => {
            return FullDebugInfo {
                atr: None,
                protocol: None,
                reader_name: None,
                aid_results: vec![],
                raw_read_result: None,
                timestamp,
                error: Some(format!("Failed to list readers: {}", e)),
            };
        }
    };

    let reader_names: Vec<_> = readers.collect();
    if reader_names.is_empty() {
        return FullDebugInfo {
            atr: None,
            protocol: None,
            reader_name: None,
            aid_results: vec![],
            raw_read_result: None,
            timestamp,
            error: Some("No card reader connected".to_string()),
        };
    }

    let reader_name = reader_names[0];
    let reader_name_str = reader_name.to_string_lossy().to_string();

    // Check card status and get ATR
    let mut reader_states = vec![pcsc::ReaderState::new(reader_name, State::UNAWARE)];
    if ctx
        .get_status_change(Duration::from_millis(100), &mut reader_states)
        .is_err()
    {
        return FullDebugInfo {
            atr: None,
            protocol: None,
            reader_name: Some(reader_name_str),
            aid_results: vec![],
            raw_read_result: None,
            timestamp,
            error: Some("Failed to get card status".to_string()),
        };
    }

    let state = &reader_states[0];
    let card_inserted = state.event_state().contains(State::PRESENT);

    if !card_inserted {
        return FullDebugInfo {
            atr: None,
            protocol: None,
            reader_name: Some(reader_name_str),
            aid_results: vec![],
            raw_read_result: None,
            timestamp,
            error: Some("No card inserted".to_string()),
        };
    }

    // Get ATR
    let atr = state.atr();
    let atr_hex = if atr.is_empty() {
        None
    } else {
        Some(bytes_to_hex(atr))
    };

    // Connect to card and get protocol
    let (card, protocol) = match connect_with_retry_and_protocol(&ctx, reader_name, 3, 500) {
        Ok((card, protocol)) => (card, protocol),
        Err(e) => {
            return FullDebugInfo {
                atr: atr_hex,
                protocol: None,
                reader_name: Some(reader_name_str),
                aid_results: vec![],
                raw_read_result: None,
                timestamp,
                error: Some(format!("Failed to connect to card: {}", e)),
            };
        }
    };

    // Test all known AIDs
    let aid_results = test_known_aids(&card);

    // Try a raw read command (READ CID) for debug
    let raw_read_result = {
        // First select the applet
        let select_result = transmit_with_get_response(&card, SELECT_APPLET);
        if select_result.is_ok() {
            // Try reading CID
            match transmit_with_get_response(&card, READ_CID) {
                Ok(response) if response.len() >= 2 => {
                    let sw1 = response[response.len() - 2];
                    let sw2 = response[response.len() - 1];
                    Some(RawReadResult {
                        command: "READ_CID".to_string(),
                        command_hex: bytes_to_hex(READ_CID),
                        response: bytes_to_hex(&response),
                        sw1,
                        sw2,
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    };

    FullDebugInfo {
        atr: atr_hex,
        protocol: Some(protocol),
        reader_name: Some(reader_name_str),
        aid_results,
        raw_read_result,
        timestamp,
        error: None,
    }
}

// ============================================================================
// CardReader Wrapper Struct
// ============================================================================

/// CardReader struct wrapper for object-oriented access to card reader functions
///
/// This struct wraps the free functions to provide a consistent interface
/// for use with async runtimes and shared state patterns.
pub struct CardReader;

impl CardReader {
    /// Create a new CardReader instance
    pub fn new() -> Self {
        CardReader
    }

    /// Get current status of card reader and card
    pub fn get_status(&self) -> CardReaderState {
        get_status()
    }

    /// Read all data from Thai National ID card (without photo)
    ///
    /// This is an async wrapper that runs the blocking read operation
    /// in a separate thread to avoid blocking the async runtime.
    pub async fn read_card(&mut self) -> Result<CardData, CardReaderError> {
        // Run blocking PC/SC operations in a separate thread
        tokio::task::spawn_blocking(|| read_card())
            .await
            .map_err(|e| CardReaderError::TransmitError(format!("Task join error: {}", e)))?
    }

    /// Read all data from Thai National ID card with options
    ///
    /// # Arguments
    /// * `include_photo` - Whether to read the photo (adds ~2 seconds)
    pub async fn read_card_with_options(&mut self, include_photo: bool) -> Result<CardData, CardReaderError> {
        // Run blocking PC/SC operations in a separate thread
        tokio::task::spawn_blocking(move || read_card_with_options(include_photo))
            .await
            .map_err(|e| CardReaderError::TransmitError(format!("Task join error: {}", e)))?
    }

    /// Get full debug information
    pub async fn get_debug_info(&self) -> FullDebugInfo {
        // Run blocking PC/SC operations in a separate thread
        tokio::task::spawn_blocking(get_debug_info)
            .await
            .unwrap_or_else(|e| FullDebugInfo {
                atr: None,
                protocol: None,
                reader_name: None,
                aid_results: vec![],
                raw_read_result: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
                error: Some(format!("Task join error: {}", e)),
            })
    }
}

impl Default for CardReader {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ascii_string() {
        // Test with status bytes
        let data = b"JOHN#DOE\x00\x00\x90\x00";
        let result = parse_ascii_string(data);
        assert_eq!(result, "JOHN DOE");
    }

    #[test]
    fn test_parse_ascii_string_empty() {
        let data = b"\x90\x00";
        let result = parse_ascii_string(data);
        assert_eq!(result, "");
    }

    #[test]
    fn test_parse_date() {
        let data = b"19900115\x90\x00";
        let result = parse_date(data);
        assert_eq!(result, "1990-01-15");
    }

    #[test]
    fn test_parse_date_invalid() {
        let data = b"123\x90\x00";
        let result = parse_date(data);
        assert_eq!(result, "123");
    }

    #[test]
    fn test_split_name_full() {
        let (title, first, last) = split_name("Mr. John Doe");
        assert_eq!(title, "Mr.");
        assert_eq!(first, "John");
        assert_eq!(last, "Doe");
    }

    #[test]
    fn test_split_name_with_multiple_last_names() {
        let (title, first, last) = split_name("Mr. John Van Der Berg");
        assert_eq!(title, "Mr.");
        assert_eq!(first, "John");
        assert_eq!(last, "Van Der Berg");
    }

    #[test]
    fn test_split_name_empty() {
        let (title, first, last) = split_name("");
        assert_eq!(title, "");
        assert_eq!(first, "");
        assert_eq!(last, "");
    }

    #[test]
    fn test_parse_gender() {
        assert_eq!(parse_gender("1"), "Male");
        assert_eq!(parse_gender("2"), "Female");
        assert_eq!(parse_gender("3"), "3");
    }

    #[test]
    fn test_tis620_to_utf8() {
        // Test ASCII passthrough
        let ascii = b"Hello";
        let result = tis620_to_utf8(ascii);
        assert_eq!(result, "Hello");

        // TIS-620 0xA1 should map to U+0E01 (Thai KO KAI)
        let thai = [0xA1u8]; // ก in TIS-620
        let result = tis620_to_utf8(&thai);
        assert_eq!(result, "\u{0E01}");
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes = [0x3B, 0x67, 0x00, 0x00, 0x00, 0xC1, 0x04, 0x31, 0x80, 0x00];
        let result = bytes_to_hex(&bytes);
        assert_eq!(result, "3B67000000C1043180");
    }

    #[test]
    fn test_card_reader_state_default() {
        let state = CardReaderState::default();
        assert!(!state.reader_connected);
        assert!(!state.card_inserted);
        assert!(state.reader_name.is_none());
        assert!(state.card_atr.is_none());
    }

    #[test]
    fn test_card_reader_error_display() {
        let err = CardReaderError::NoReaderConnected;
        assert_eq!(err.to_string(), "No card reader connected");

        let err = CardReaderError::InvalidResponse { sw1: 0x6A, sw2: 0x82 };
        assert_eq!(err.to_string(), "Invalid response from card: SW=6A82");
    }
}
