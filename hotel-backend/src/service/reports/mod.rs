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

pub mod channel_rollup;
pub mod rr4;

pub use channel_rollup::{
    classify, load_channel_rollup, rollup, ChannelBucket, ChannelBucketRow, ChannelRollup,
    ChannelSourceRow, ChannelTotals, DEFAULT_RANGE_DAYS, MAX_RANGE_DAYS,
};
pub use rr4::{
    render_csv, render_xlsx, Rr4ExportFormat, Rr4ExportRequest, Rr4ExportResult, Rr4Row,
    Rr4Service, RR4_COLUMN_HEADERS,
};
