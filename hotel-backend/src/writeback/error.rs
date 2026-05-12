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
    /// (Tiberius, Pool) may succeed on retry.
    ///
    /// For `sqlx::Error::Database` (audit Wave 6 LOW item 9): we
    /// **inspect the SQLSTATE** and only retry codes that PG classifies as
    /// transient. Earlier this branch returned `true` unconditionally,
    /// which meant a `23505` (unique_violation) on `writeback_jobs` —
    /// possible if a producer races two NOTIFYs for the same row — would
    /// retry forever and pin a worker thread on a deterministic failure.
    ///
    /// Transient SQLSTATEs we retry:
    /// - `40001` serialization_failure
    /// - `40P01` deadlock_detected
    /// - `57P01` admin_shutdown
    /// - `57P02` crash_shutdown
    /// - `57P03` cannot_connect_now
    /// - `08000` / `08003` / `08006` / `08001` connection-class errors
    /// - `53300` too_many_connections
    ///
    /// Everything else (`23xxx` integrity violations, `42xxx` syntax /
    /// schema errors, `P0001` recipe-raised exceptions, etc.) is a
    /// deterministic failure → false.
    pub fn is_retryable(&self) -> bool {
        match self {
            WritebackError::Tiberius(_) | WritebackError::Pool(_) => true,
            WritebackError::Sqlx(err) => sqlx_error_is_retryable(err),
            WritebackError::SchemaDrift { .. }
            | WritebackError::Disabled
            | WritebackError::IntentMismatch(_)
            | WritebackError::Recipe(_)
            | WritebackError::Serde(_)
            | WritebackError::Config(_) => false,
        }
    }
}

/// Classify a `sqlx::Error` for retry. Only PG SQLSTATEs documented as
/// transient (serialization / deadlock / connection-class) cause a retry;
/// integrity violations (`23505`, `23503`, etc.) and syntax errors are
/// deterministic and surface as permanent failures.
///
/// I/O-class errors that don't carry a SQLSTATE (e.g. `sqlx::Error::Io`,
/// `PoolTimedOut`, `PoolClosed`) are also treated as transient — those
/// are the wire-level failures that retry was designed for.
fn sqlx_error_is_retryable(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db) => is_transient_pg_sqlstate(db.code().as_deref()),
        // Wire-level / pool-level failures — retry.
        sqlx::Error::Io(_)
        | sqlx::Error::Tls(_)
        | sqlx::Error::PoolTimedOut
        | sqlx::Error::PoolClosed
        | sqlx::Error::WorkerCrashed => true,
        // Everything else (RowNotFound, ColumnNotFound, Decode, Encode,
        // Configuration, Protocol, etc.) is deterministic.
        _ => false,
    }
}

/// PG SQLSTATE classification — `true` only for codes that PG's docs flag
/// as transient (a retry could plausibly succeed). Source:
/// <https://www.postgresql.org/docs/current/errcodes-appendix.html>.
fn is_transient_pg_sqlstate(code: Option<&str>) -> bool {
    matches!(
        code,
        Some(
            "40001"     // serialization_failure
            | "40P01"   // deadlock_detected
            | "57P01"   // admin_shutdown
            | "57P02"   // crash_shutdown
            | "57P03"   // cannot_connect_now
            | "08000"   // connection_exception
            | "08001"   // sqlclient_unable_to_establish_sqlconnection
            | "08003"   // connection_does_not_exist
            | "08006"   // connection_failure
            | "53300"   // too_many_connections
        )
    )
}

/// Result alias used throughout the writeback module.
pub type WritebackResult<T> = Result<T, WritebackError>;

#[cfg(test)]
mod tests {
    use super::*;

    // --- Wave 6 LOW item 9 — PG SQLSTATE classification ---

    #[test]
    fn transient_pg_sqlstates_are_retryable() {
        for code in [
            "40001", "40P01", "57P01", "57P02", "57P03", "08000", "08001", "08003", "08006",
            "53300",
        ] {
            assert!(
                is_transient_pg_sqlstate(Some(code)),
                "SQLSTATE {code} must be classified as transient"
            );
        }
    }

    #[test]
    fn integrity_violations_are_not_retryable() {
        // These were silently retried forever under the old impl —
        // unique_violation on `writeback_jobs.id` or a constraint break
        // would pin a worker on a deterministic failure.
        for code in ["23505", "23503", "23502", "23514", "23P01"] {
            assert!(
                !is_transient_pg_sqlstate(Some(code)),
                "SQLSTATE {code} (integrity violation) must NOT retry"
            );
        }
    }

    #[test]
    fn syntax_and_schema_errors_are_not_retryable() {
        // Syntax errors, missing tables/columns, etc. — retrying won't help.
        for code in ["42601", "42703", "42P01", "42883"] {
            assert!(
                !is_transient_pg_sqlstate(Some(code)),
                "SQLSTATE {code} (schema/syntax) must NOT retry"
            );
        }
    }

    #[test]
    fn recipe_user_exceptions_are_not_retryable() {
        // PG `RAISE EXCEPTION ...` defaults to P0001; recipe-raised
        // failures must not retry.
        assert!(!is_transient_pg_sqlstate(Some("P0001")));
    }

    #[test]
    fn missing_sqlstate_is_not_retryable() {
        // No SQLSTATE → we don't know what happened → don't retry.
        assert!(!is_transient_pg_sqlstate(None));
    }

    // --- WritebackError dispatch ---

    #[test]
    fn schema_drift_is_not_retryable() {
        let e = WritebackError::SchemaDrift {
            expected: "x".into(),
            actual: "y".into(),
        };
        assert!(!e.is_retryable());
    }

    #[test]
    fn recipe_error_is_not_retryable() {
        assert!(!WritebackError::Recipe("boom".into()).is_retryable());
    }

    #[test]
    fn intent_mismatch_is_not_retryable() {
        assert!(!WritebackError::IntentMismatch("boom".into()).is_retryable());
    }

    #[test]
    fn config_error_is_not_retryable() {
        assert!(!WritebackError::Config("boom".into()).is_retryable());
    }

    #[test]
    fn disabled_is_not_retryable() {
        assert!(!WritebackError::Disabled.is_retryable());
    }
}
