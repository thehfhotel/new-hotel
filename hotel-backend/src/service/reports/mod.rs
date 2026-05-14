//! Service layer — operational + compliance reports.
//!
//! Per `docs/architecture.md` §1, reports are pure read-paths: they
//! aggregate canonical state across multiple aggregates and never emit
//! outbox events. This module groups every export the regulator,
//! accountant, or housekeeper needs into a single namespace so the
//! HTTP layer can dispatch from one place.
//!
//! Current members:
//!
//! * [`rr4`] — RR.4 / ตม.30 Thai immigration foreign-guest export
//!   (audit 2026-05-13 T4 CRIT-2). Required by law before standalone
//!   cutover.

pub mod rr4;

pub use rr4::{
    render_csv, render_xlsx, Rr4ExportFormat, Rr4ExportRequest, Rr4ExportResult, Rr4Row,
    Rr4Service, RR4_COLUMN_HEADERS,
};
