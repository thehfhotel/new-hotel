//! Error type for the writeback worker.
//!
//! Per `docs/architecture.md` §3.6c — the writeback worker pulls jobs from
//! `writeback_jobs` and pushes them to legacy MSSQL. Failures fall into a small
//! number of buckets, each handled differently:
//!
//! * [`WritebackError::SchemaDrift`] — refuse to start. Vendor changed the
//!   legacy schema; writing now would corrupt their data.
//! * [`WritebackError::IntentMismatch`] — payload doesn't match the variant.
//!   Move job to `failed`; this is a code bug, retries won't help.
//! * [`WritebackError::Tiberius`] / [`WritebackError::Sqlx`] — transient I/O.
//!   Retry up to `WRITEBACK_MAX_ATTEMPTS` (default 3) with exponential backoff.
//! * [`WritebackError::Recipe`] — recipe-specific business-rule failure (e.g.
//!   no prior occupant for a `mark_clean`). Move job to `failed`; needs human
//!   intervention.
//! * [`WritebackError::Disabled`] — `WRITEBACK_ENABLED=false`. Worker exits
//!   cleanly at startup.

use thiserror::Error;

/// All failure modes the writeback worker exposes to its caller (the binary
/// `bin/writeback.rs`).
#[derive(Error, Debug)]
pub enum WritebackError {
    /// Schema fingerprint check on startup found drift between the captured
    /// baseline (`docs/legacy-spike/schema/01-baseline-schema.txt`) and the
    /// live MSSQL columns. Worker refuses to write.
    #[error("legacy schema drift: expected fingerprint {expected}, got {actual}")]
    SchemaDrift { expected: String, actual: String },

    /// `WRITEBACK_ENABLED=false` — Service layer keeps enqueuing for audit, but
    /// the worker exits at startup so jobs accumulate harmlessly until the
    /// toggle flips.
    #[error("writeback disabled by WRITEBACK_ENABLED env var")]
    Disabled,

    /// Job's `intent` discriminant doesn't match its `payload` shape — a
    /// service-layer bug. Logged and the job is moved to `failed`; retries
    /// won't fix it.
    #[error("intent payload mismatch: {0}")]
    IntentMismatch(String),

    /// Recipe encountered a business-rule failure that retrying won't fix
    /// (e.g. `mark_clean` couldn't find any prior occupant for the room).
    #[error("recipe error: {0}")]
    Recipe(String),

    /// Transient MSSQL I/O failure. Retry-eligible.
    #[error("tiberius: {0}")]
    Tiberius(#[from] tiberius::error::Error),

    /// Transient bb8 pool failure (acquire timeout, broken connection, etc.).
    /// Retry-eligible.
    #[error("legacy connection pool: {0}")]
    Pool(#[from] bb8::RunError<bb8_tiberius::Error>),

    /// Transient PostgreSQL I/O failure. Retry-eligible.
    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    /// JSON deserialization of `writeback_jobs.payload` into the matching
    /// `WritebackIntent` variant failed. Treated as `IntentMismatch`-equivalent.
    #[error("payload deserialize: {0}")]
    Serde(#[from] serde_json::Error),

    /// Generic configuration / environment failure.
    #[error("config: {0}")]
    Config(String),
}

impl WritebackError {
    /// Whether this error should cause the job to retry (true) or be marked
    /// `failed` immediately (false).
    ///
    /// Schema drift, intent mismatch, payload deserialize, and recipe errors
    /// are deterministic — retrying produces the same failure. I/O errors
    /// (Tiberius, Sqlx, Pool) may succeed on retry.
    pub fn is_retryable(&self) -> bool {
        match self {
            WritebackError::Tiberius(_)
            | WritebackError::Pool(_)
            | WritebackError::Sqlx(_) => true,
            WritebackError::SchemaDrift { .. }
            | WritebackError::Disabled
            | WritebackError::IntentMismatch(_)
            | WritebackError::Recipe(_)
            | WritebackError::Serde(_)
            | WritebackError::Config(_) => false,
        }
    }
}

/// Result alias used throughout the writeback module.
pub type WritebackResult<T> = Result<T, WritebackError>;
