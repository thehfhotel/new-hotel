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
//! * [`channel_rollup`] — direct-booking program D3. Bookings, room-nights,
//!   gross revenue and cancellations bucketed by `ht_bookings.book_channel` /
//!   `book_source`, so "what is our direct share?" has an answer.
//! * [`loyalty_reconcile`] — direct-booking program B8f (checklist L6). The
//!   five-minute morning read reception runs at shift open: app bookings that
//!   iHOTEL and PostgreSQL disagree about. PG-only, so it keeps working when
//!   the legacy leg is the thing that is broken.

pub mod channel_rollup;
pub mod loyalty_reconcile;
pub mod rr4;

pub use channel_rollup::{
    classify, load_channel_rollup, rollup, ChannelBucket, ChannelBucketRow, ChannelRollup,
    ChannelSourceRow, ChannelTotals, DEFAULT_RANGE_DAYS, MAX_RANGE_DAYS,
};
pub use loyalty_reconcile::{
    age_label, arrival_label, classify_writeback_gap, load_loyalty_reconcile, reconcile,
    stall_threshold_minutes, LoyaltyReconcile, ReconcileKind, ReconcileRow, ReconcileSummary,
    DEPOSIT_HORIZON_DAYS, MAX_ROWS_PER_KIND,
};
pub use rr4::{
    render_csv, render_xlsx, Rr4ExportFormat, Rr4ExportRequest, Rr4ExportResult, Rr4Row,
    Rr4Service, RR4_COLUMN_HEADERS,
};
