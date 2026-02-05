//! Booking note models (HT_Booking_Notes table - app-owned)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Booking note from HT_Booking_Notes table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i32,
    pub text: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Notes list response
#[derive(Debug, Serialize)]
pub struct NotesResponse {
    pub success: bool,
    pub notes: Vec<Note>,
}

/// Request body for creating a note
#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub text: String,
}

/// Response after creating a note
#[derive(Debug, Serialize)]
pub struct CreateNoteResponse {
    pub success: bool,
    pub note: Note,
}

/// Response after deleting a note
#[derive(Debug, Serialize)]
pub struct DeleteNoteResponse {
    pub success: bool,
    pub message: String,
}
