//! Change Tracking watcher (MSSQL → PG) — Phase 5.
//!
//! The reverse half of the writeback adapter (`crate::writeback`). Where
//! writeback pushes our PG-side mutations into legacy MSSQL, this module
//! pulls the .NET app's MSSQL-side mutations into PG so we can emit
//! `DomainEvent`s against the canonical store.
//!
//! Per `docs/architecture.md` §3.6d, §4d-tris, §10 #8.
//!
//! ## Module layout (Phase 5.1 skeleton)
//!
//! | Submodule | Responsibility |
//! |---|---|
//! | [`change_op`] | `'I'` / `'U'` / `'D'` enum from CT's `SYS_CHANGE_OPERATION` |
//! | [`watermark`] | Read / advance `legacy_ct_state.last_seen_version` |
//! | [`mapper`]    | `MssqlChangeMapper` trait + `NoopMapper` placeholder |
//!
//! ## What this module does NOT do (yet)
//!
//! - No real per-table mappers — that's 5.2+. `NoopMapper` is enough
//!   for 5.1 to exercise the polling loop, watermark, and observability
//!   plumbing without touching PG row-shapes.
//! - No bootstrap path — cold-start (`--bootstrap`) reuses
//!   `crate::scheduler::sync` and lands in 5.5.
//! - No mirror-schema writes — Phase 5.5 (`legacy_mirror`).
//!
//! ## Loop-prevention companion
//!
//! The watcher's MSSQL `SELECT … FROM CHANGETABLE(CHANGES …)` filters
//! out rows whose `SYS_CHANGE_CONTEXT = 0x4E48` — the tag stamped by
//! `crate::db::mssql_session::set_context_info` at the top of the
//! writeback dispatcher. Together they prevent a feedback loop between
//! the writeback worker and the CT watcher.

pub mod change_op;
pub mod mapper;
pub mod mappers;
pub mod parent_loader;
pub mod resolve;
pub mod row;
pub mod watermark;

pub use change_op::ChangeOp;
pub use mapper::{MssqlChangeMapper, NoopMapper};
pub use row::MappableRow;

use thiserror::Error;

/// All failure modes the CT watcher binary exposes to its caller
/// (`bin/sync.rs`).
///
/// Kept small for Phase 5.1 — the variant set will grow as 5.2+ adds
/// per-table mappers that produce richer error contexts (e.g. UUID
/// derivation failures, foreign-key resolution misses).
#[derive(Error, Debug)]
pub enum SyncError {
    /// Transient PostgreSQL I/O failure. Watcher should log + retry the
    /// tick.
    #[error("sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    /// Transient MSSQL I/O failure. Watcher should log + retry the tick.
    #[error("tiberius: {0}")]
    Tiberius(#[from] tiberius::error::Error),

    /// Transient bb8 pool failure (acquire timeout, broken connection).
    #[error("legacy connection pool: {0}")]
    Pool(#[from] bb8::RunError<bb8_tiberius::Error>),

    /// CT retention overflow — `MIN_VALID_VERSION` for some table is
    /// already higher than our last-seen watermark. We've fallen behind
    /// past the configured retention window; advancing would silently
    /// skip rows. Operator must run a bootstrap (`--bootstrap`) to
    /// re-seed canonical state before sync can resume.
    #[error("CT retention overflow on table {table}: min_valid_version={min_valid} > last_seen_version={last_seen}")]
    RetentionOverflow {
        table: String,
        min_valid: i64,
        last_seen: i64,
    },

    /// Mapper returned a domain-level error that retry won't fix — e.g.
    /// a row referenced an aggregate UUID we've never seen. The watcher
    /// records it in `legacy_sync_status.last_error` and continues with
    /// the next mapper.
    #[error("mapper error ({table}): {message}")]
    Mapper { table: &'static str, message: String },

    /// Generic configuration / environment failure.
    #[error("config: {0}")]
    Config(String),
}
