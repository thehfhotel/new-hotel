//! Writeback adapter — pushes our PG-side mutations to the legacy MSSQL DB.
//!
//! Per `docs/architecture.md` §3.6c (publication path) + §6 (worked example) +
//! §8 Phase 4b. Lives **outside** the decommission boundary — when legacy is
//! turned off (State C), `WRITEBACK_ENABLED=false` and this module's worker
//! exits at startup. The rest of the application keeps running.
//!
//! ## Module layout
//!
//! | File | Responsibility |
//! |---|---|
//! | [`error`] | `WritebackError` enum + retryability classification |
//! | [`fingerprint`] | Schema-drift guard — refuses to start if MSSQL columns drifted |
//! | [`format`] | Date / quote helpers matching the .NET app's literal output |
//! | [`constants`] | Verbatim Thai/English status literals + power-log templates |
//! | [`allocate`] | TABLOCKX + HOLDLOCK MAX+1 helpers per ID counter |
//! | [`dispatcher`] | Routes a `WritebackIntent` to its recipe |
//! | [`recipes`] | One file per intent, faithful translation of spike findings |
//!
//! ## Usage by the worker (`bin/writeback.rs`)
//!
//! 1. On startup → [`fingerprint::verify_schema_fingerprint`]. Bail on drift.
//! 2. `LISTEN writeback_channel` (PG) + 30-sec poll fallback.
//! 3. For each pending job: claim row → MSSQL `BEGIN TRAN` → call
//!    [`dispatcher::dispatch`] → COMMIT → mark done. On failure: rollback,
//!    increment `attempts`, retry up to `WRITEBACK_MAX_ATTEMPTS`.
//!
//! The worker binary owns the loop + retry policy; this module owns the SQL
//! generation and ID allocation. Keeps recipes pure of orchestration concerns.

pub mod allocate;
pub mod constants;
pub mod dispatcher;
pub mod error;
pub mod fingerprint;
pub mod format;
pub mod recipes;

pub use dispatcher::{dispatch, DispatchContext, LegacyIds, ResolvedJob};
pub use error::{WritebackError, WritebackResult};
pub use fingerprint::{
    verify_legacy_collation_safety, verify_schema_fingerprint, EXPECTED_FINGERPRINT,
};
