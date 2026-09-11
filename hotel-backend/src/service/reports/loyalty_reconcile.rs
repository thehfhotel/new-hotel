//! Direct-booking program B8f — the morning reconciliation.
//!
//! The five-minute read reception runs at shift open, before the desk gets
//! busy, to answer one question: **is there an app booking that iHOTEL and our
//! app disagree about?** Task B8's checklist line **L6**
//! (`hf-tasks/tasks/direct-booking-designs/b8-overbooking-analysis.md` §4) —
//! a PRE-FLIP control, so it must work while the channel is still dark.
//!
//! Pure read-path, per `docs/architecture.md` §1: no outbox, no writeback, no
//! legacy touch. Nothing here writes anything anywhere. It reads **only
//! canonical PostgreSQL** — which is also why it keeps working when the legacy
//! leg is the thing that is broken (the same reasoning that puts the F5
//! tripwire in the scheduler rather than in the writeback worker).
//!
//! ## Why every predicate here is borrowed, not invented
//!
//! A reconciliation report that defines "not applied" or "expired" slightly
//! differently from the machinery that *acts* on those rows is worse than no
//! report: it teaches the desk to distrust the tripwire ("the alert fired but
//! the morning list is empty"). So each of the four row kinds reuses, verbatim,
//! the predicate of the component that already owns that judgement:
//!
//! | Kind | Predicate borrowed from | Where |
//! |---|---|---|
//! | [`ReconcileKind::WritebackStalled`] | Track F5's stall detector `fetch_stalled_loyalty_writebacks` | `scheduler/sync.rs` |
//! | [`ReconcileKind::LegacyHoldOrphan`] | the same F5 query, split out by intent — F5 documents cancels as in-scope precisely because "a cancel that never reaches iHOTEL leaves a phantom `จอง` on the room board" | `scheduler/sync.rs` |
//! | [`ReconcileKind::SweepLag`] | the hold expiry sweep's input query `repository::channel::expired_hold_ids` | `repository/channel.rs` |
//! | [`ReconcileKind::UnlinkedCheckin`] | the check-in overlap leg of `FREE_ROOM_PREDICATE` | `repository/channel.rs` |
//! | [`ReconcileKind::DepositDivergence`] | the B7 accepted divergence, `docs/loyalty-channel.md` §"Dual-write policy for holds" | — |
//!
//! The two F5 constants are imported rather than re-spelled
//! ([`WRITEBACK_APPLIED_STATUS`], [`DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES`]),
//! so a change to the tripwire's definition of "applied" moves this report with
//! it. The two `repository::channel` predicates live inside private `const`s in
//! a file this module must not edit, so they are restated here with their
//! source named inline and a unit test pinning the literals.
//!
//! ## What "no iHOTEL twin" means without reading MSSQL
//!
//! `writeback_jobs.status = 'done'` is the canonical record that the legacy
//! write landed — the worker sets it only after the MSSQL transaction commits.
//! So "has an unapplied writeback job" *is* "has no iHOTEL twin", established
//! from PG alone. That is the whole reason this report can be a PG-only read,
//! which B8 §4 L6 flagged as the open question.
//!
//! **Stated blind spot:** a loyalty booking with a room but *no `writeback_jobs`
//! row at all* is invisible to this report, exactly as it is to F5. Both are
//! anchored on a job row that EXISTS, so neither can ever mis-report "the hold
//! was never created" as a stall. The enqueue is in the same transaction as the
//! canonical insert (`service/booking.rs`, gated on `!cmd.rooms.is_empty()`),
//! so a roomed loyalty booking without a job implies a torn commit, which
//! PostgreSQL does not produce. The runbook carries the SQL to check it by hand
//! anyway.
//!
//! ## Timezone
//!
//! The report date defaults to **today in Asia/Bangkok**, because a shift opens
//! on a Thai business day, not a UTC one. Timestamps are returned as stored:
//! `book_hold_expires_at` is `TIMESTAMPTZ` (a real instant), while
//! `cin_checkin_time` is a naive `TIMESTAMP` carrying **local Thai wall time**
//! mirrored from MSSQL — it is emitted unconverted, and any renderer must
//! format it with `timeZone: 'UTC'` to show the stored value as-is. Never
//! `Asia/Bangkok`: that would shift it a second time.

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, Row};

use crate::scheduler::sync::{DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES, WRITEBACK_APPLIED_STATUS};
use crate::service::channel::LOYALTY_CHANNEL;

/// `ht_bookings.book_status` of an unpaid loyalty hold.
///
/// Same literal the expiry sweep is guarded on
/// (`repository::channel::expired_hold_ids`, `release_hold`) and that Track F5
/// pins as `CHANNEL_HOLD_STATUS`. Restated (not imported) because the sweep's
/// copy is inline SQL inside a file B8e owns.
const CHANNEL_HOLD_STATUS: &str = "pending";

/// `ht_bookings.book_status` written by the release/expiry path.
/// `repository::channel::release_hold` sets exactly this.
const CANCELLED_STATUS: &str = "cancelled";

/// `writeback_jobs.intent` for the cancel leg — `WritebackIntent::CancelBooking`
/// serialises to this (`outbox/intent.rs`). The hold expiry sweep releases
/// through the normal cancel path, so an unapplied job with this intent is the
/// phantom-`จอง` case.
const CANCEL_BOOKING_INTENT: &str = "cancel_booking";

/// How many days past the report date the deposit look-ahead covers.
///
/// `1` = today **and** tomorrow, per B8f: the desk must know about an app
/// deposit before the guest is standing at the counter, and a morning routine
/// run at 07:00 is the last chance to brief the shift about a tomorrow arrival
/// whose reception may be a different person.
pub const DEPOSIT_HORIZON_DAYS: i64 = 1;

/// Hard cap on rows returned per kind.
///
/// A morning read, not an export. On a healthy morning every one of these
/// queries returns zero rows; a four-figure result means something systemic is
/// broken and the desk needs the runbook, not a longer list. Matches the
/// `LIMIT 100` discipline of the sweep and the F5 detector, doubled because
/// this report has no cooldown to hide behind.
pub const MAX_ROWS_PER_KIND: i64 = 200;

// ---------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------

/// What is wrong with one row, and therefore what the desk does about it.
///
/// The derived `Ord` is **the report's display order and it is load-bearing**:
/// it ranks by blast radius, worst first, so a receptionist working top-down is
/// working in the right order. Reordering the variants reorders the response.
///
/// The ranking, and the reasoning behind it:
///
/// 1. [`WritebackStalled`](ReconcileKind::WritebackStalled) — **we sold a room
///    iHOTEL believes is free.** The only kind that can produce a double-sold
///    room-night, which is the harm B8 exists to prevent.
/// 2. [`LegacyHoldOrphan`](ReconcileKind::LegacyHoldOrphan) — **iHOTEL holds a
///    room that is actually free.** Inverted harm: not a double-sell, but the
///    desk will turn away a walk-in for a guest who no longer exists.
/// 3. [`SweepLag`](ReconcileKind::SweepLag) — a dead hold still blocking *our*
///    inventory. Self-healing within 5 minutes in the normal case, so it ranks
///    below the two that are not self-healing.
/// 4. [`UnlinkedCheckin`](ReconcileKind::UnlinkedCheckin) — the guest is in the
///    room; only the paperwork is wrong. Costs money at checkout (the deposit
///    signposts do not render), costs nobody a room.
/// 5. [`DepositDivergence`](ReconcileKind::DepositDivergence) — nothing is
///    broken at all. A briefing item, deliberately last so it can never push a
///    real defect below the fold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReconcileKind {
    /// (a) A loyalty booking whose writeback job has not applied — iHOTEL has
    /// no twin, so the room reads FREE on the room board.
    WritebackStalled,
    /// (b, second half) A released/expired hold whose `cancel_booking`
    /// writeback has not applied — iHOTEL still shows `จอง` for a room that is
    /// free. The "legacy `จอง` row that outlived its hold".
    LegacyHoldOrphan,
    /// (b, first half) A hold past `book_hold_expires_at` still sitting
    /// `pending` — the 5-minute expiry sweep has not caught it.
    SweepLag,
    /// (d) An app booking checked in without the canonical link
    /// (`cin_book_id IS NULL` on a check-in matching the booking's room and
    /// dates) — the B7a gap, typically an iHOTEL-side or walk-in check-in.
    UnlinkedCheckin,
    /// (c) An app booking arriving today/tomorrow carrying a deposit that
    /// iHOTEL will show as 0 — B7's accepted divergence, not a defect.
    DepositDivergence,
}

impl ReconcileKind {
    /// Stable machine key. Wire contract — the runbook's SQL headings and any
    /// future desk chip key off these strings.
    pub fn key(&self) -> &'static str {
        match self {
            ReconcileKind::WritebackStalled => "writeback_stalled",
            ReconcileKind::LegacyHoldOrphan => "legacy_hold_orphan",
            ReconcileKind::SweepLag => "sweep_lag",
            ReconcileKind::UnlinkedCheckin => "unlinked_checkin",
            ReconcileKind::DepositDivergence => "deposit_divergence",
        }
    }

    /// What the desk does, as a machine key the runbook maps to a paragraph.
    ///
    /// Three verbs only, because a five-minute routine cannot carry more:
    /// `call_guest` (the guest's room is at risk — speak to a human),
    /// `resend_hold` (our side is stale — re-drive the leg, do not hand-write
    /// legacy rows), `brief_desk` (nothing to fix; tell the shift).
    pub fn action(&self) -> &'static str {
        match self {
            // The room is sold in our app and free in iHOTEL. Until the leg
            // recovers the only protection is a human knowing about it.
            ReconcileKind::WritebackStalled => "call_guest",
            // Nobody's stay is at risk; a stale row needs re-driving.
            ReconcileKind::LegacyHoldOrphan | ReconcileKind::SweepLag => "resend_hold",
            // The guest is already in the room; the desk just needs to know
            // the deposit exists before it takes payment.
            ReconcileKind::UnlinkedCheckin | ReconcileKind::DepositDivergence => "brief_desk",
        }
    }

    /// Whether this kind means something is genuinely wrong.
    ///
    /// [`DepositDivergence`](ReconcileKind::DepositDivergence) is the one
    /// expected-by-design kind: iHOTEL showing deposit 0 for an app booking is
    /// the documented consequence of the `booking_modify` recipe having no
    /// `Book_Price_Pay` leg, not a failure. Counting it as a defect would make
    /// the summary read red on a perfectly healthy morning, which is the
    /// fastest way to get a daily routine abandoned.
    pub fn is_defect(&self) -> bool {
        !matches!(self, ReconcileKind::DepositDivergence)
    }
}

// ---------------------------------------------------------------------------
// Pure bucketing + age logic
// ---------------------------------------------------------------------------

/// Bucket one unapplied writeback job into (a) or (b-second-half).
///
/// Both come out of the SAME F5 query — one indexed round trip — and are told
/// apart here, on the pure side, so the split is unit-testable without a
/// database.
///
/// The rule, and why it is `intent` **and** `book_status` rather than either
/// alone:
///
/// * `cancel_booking` + `cancelled` → [`ReconcileKind::LegacyHoldOrphan`]. The
///   canonical row is already dead; the only thing outstanding is telling
///   iHOTEL, so iHOTEL is holding a `จอง` for a hold that no longer exists.
/// * anything else → [`ReconcileKind::WritebackStalled`]. Includes
///   `create_booking` and `modify_booking` at any status.
///
/// Checking `intent` alone would be wrong: a `cancel_booking` job on a booking
/// that is NOT `cancelled` means the cancel raced a payment-verified and lost
/// (`release_hold` is guarded on `book_status='pending'`, so it writes 0 rows
/// and the booking stays `confirmed`). That booking is live and paid, so its
/// outstanding legacy work is the dangerous "no twin" kind, not a phantom.
///
/// Checking `book_status` alone would be equally wrong: a cancelled booking
/// with a stuck *create* still has no iHOTEL twin at all, so there is no
/// phantom `จอง` to chase — the create simply became moot.
pub fn classify_writeback_gap(intent: &str, book_status: &str) -> ReconcileKind {
    if intent == CANCEL_BOOKING_INTENT && book_status == CANCELLED_STATUS {
        return ReconcileKind::LegacyHoldOrphan;
    }
    ReconcileKind::WritebackStalled
}

/// Render an age in whole minutes as the compact label the desk reads.
///
/// `None` → `""`: the deposit-divergence kind has no age (nothing is late; the
/// row is a briefing item), and rendering `0m` there would imply a clock that
/// is not running.
///
/// Negative input clamps to `0m` rather than rendering `-7m`. A negative age is
/// only reachable through clock skew between the DB and the report, and
/// "expires in -7m" at 07:00 is a puzzle, not an instruction — the same call
/// Track F5's `format_hold_suffix` makes for the same reason.
pub fn age_label(minutes: Option<i64>) -> String {
    let Some(minutes) = minutes else {
        return String::new();
    };
    let minutes = minutes.max(0);
    if minutes < 60 {
        return format!("{minutes}m");
    }
    let (hours, mins) = (minutes / 60, minutes % 60);
    if hours < 24 {
        return format!("{hours}h {mins:02}m");
    }
    format!("{}d {}h", hours / 24, hours % 24)
}

/// How the arrival date reads relative to the report date: `"today"`,
/// `"tomorrow"`, `"+3d"`, or `"-2d"` for a past arrival.
///
/// Pure (report date injected) so the boundary cases are testable without
/// waiting for midnight in Bangkok.
pub fn arrival_label(check_in: NaiveDate, report_date: NaiveDate) -> String {
    match (check_in - report_date).num_days() {
        0 => "today".to_string(),
        1 => "tomorrow".to_string(),
        d if d > 0 => format!("+{d}d"),
        d => format!("{d}d"),
    }
}

// ---------------------------------------------------------------------------
// Row + summary shapes
// ---------------------------------------------------------------------------

/// One line of the morning report.
///
/// Flat and kind-tagged rather than an enum-per-shape, because the consumer is
/// a desk list (and a runbook SQL result) that scans top-to-bottom: every row
/// must be readable without branching on its kind first. Fields that do not
/// apply to a kind are `None` — see each field.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconcileRow {
    /// [`ReconcileKind::key`].
    pub kind: &'static str,
    /// [`ReconcileKind::action`] — `call_guest` | `resend_hold` | `brief_desk`.
    pub action: &'static str,
    /// False only for `deposit_divergence`; see [`ReconcileKind::is_defect`].
    pub is_defect: bool,
    /// `ht_bookings.book_id` — the canonical key, for a follow-up API call.
    pub book_id: i32,
    /// `ht_bookings.book_no` — the reference reception types into iHOTEL.
    pub book_no: String,
    /// `ht_bookings.book_status`.
    pub book_status: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    /// `"today"` / `"tomorrow"` / `"+3d"`, relative to the report date.
    pub arrival: String,
    /// The booked room, when one is assigned. `None` for a parked (roomless)
    /// booking — which a loyalty hold never is, but an OTA-grown one can be.
    pub room_no: Option<String>,
    /// `cust_firstname` + `cust_lastname`, blank-collapsed to `None`.
    pub guest_name: Option<String>,
    /// Kind-dependent, and deliberately `Option` so "no clock is running" is
    /// distinguishable from "0 minutes late":
    ///
    /// * `writeback_stalled` / `legacy_hold_orphan` — minutes since the
    ///   `writeback_jobs` row was enqueued;
    /// * `sweep_lag` — minutes since `book_hold_expires_at` passed;
    /// * `unlinked_checkin` — minutes since the guest checked in;
    /// * `deposit_divergence` — `None`.
    pub age_minutes: Option<i64>,
    /// [`age_label`] of `age_minutes`.
    pub age: String,
    /// `book_hold_expires_at`. Present on any channel booking (it is NOT
    /// cleared on confirmation), so it is only *meaningful* while
    /// `book_status = 'pending'` — the exact trap Track F5 documents.
    pub hold_expires_at: Option<DateTime<Utc>>,
    /// `book_deposit_amount` in baht, on `deposit_divergence` and on any
    /// `unlinked_checkin` whose booking carries one. `None` elsewhere.
    pub deposit_amount: Option<f64>,
    /// `writeback_jobs.intent`, on the two writeback kinds.
    pub intent: Option<String>,
    /// `writeback_jobs.status` (never `done`), on the two writeback kinds.
    pub job_status: Option<String>,
    /// `ht_checkins.cin_no` of the unlinked stay, on `unlinked_checkin`.
    pub cin_no: Option<String>,
    /// `ht_checkins.cin_checkin_time` — **naive local Thai wall time**, as
    /// stored. Render with `timeZone: 'UTC'`; never `Asia/Bangkok`.
    pub checked_in_at: Option<NaiveDateTime>,
}

/// The counts line — the part reception actually reads most mornings.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconcileSummary {
    pub total: i64,
    /// `total` minus the expected-by-design deposit rows. This is the number
    /// that decides whether the morning is clean.
    pub defects: i64,
    pub writeback_stalled: i64,
    pub legacy_hold_orphan: i64,
    pub sweep_lag: i64,
    pub unlinked_checkin: i64,
    pub deposit_divergence: i64,
    /// Sum of `deposit_amount` over the `deposit_divergence` rows, in baht —
    /// the money the desk must not ask for twice.
    pub deposit_total: f64,
    /// Largest `age_minutes` across the DEFECT rows (deposit rows have no
    /// age). `None` when there are no defects.
    pub oldest_defect_minutes: Option<i64>,
    /// `defects == 0`. The five-second answer: a clear morning needs no
    /// further reading.
    pub clear: bool,
}

/// The whole report body.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoyaltyReconcile {
    pub summary: ReconcileSummary,
    pub rows: Vec<ReconcileRow>,
}

/// Fold rows into the summary and put them in desk order.
///
/// Order is `(kind, age desc, book_no)`: kinds rank by blast radius via the
/// derived `Ord` on [`ReconcileKind`], and within a kind the oldest row is the
/// most urgent. `book_no` is the final tiebreak purely so the output is
/// deterministic — two rows of the same kind and age must not swap places
/// between refreshes, or the desk cannot tell "same list" from "new problem".
///
/// Pure — this is what the unit tests exercise.
pub fn reconcile(mut rows: Vec<ReconcileRow>) -> LoyaltyReconcile {
    rows.sort_by(|a, b| {
        kind_rank(a.kind)
            .cmp(&kind_rank(b.kind))
            .then(b.age_minutes.unwrap_or(0).cmp(&a.age_minutes.unwrap_or(0)))
            .then(a.book_no.cmp(&b.book_no))
    });

    let mut summary = ReconcileSummary::default();
    for row in &rows {
        summary.total += 1;
        match row.kind {
            "writeback_stalled" => summary.writeback_stalled += 1,
            "legacy_hold_orphan" => summary.legacy_hold_orphan += 1,
            "sweep_lag" => summary.sweep_lag += 1,
            "unlinked_checkin" => summary.unlinked_checkin += 1,
            "deposit_divergence" => summary.deposit_divergence += 1,
            _ => {}
        }
        if row.is_defect {
            summary.defects += 1;
            if let Some(age) = row.age_minutes {
                summary.oldest_defect_minutes =
                    Some(summary.oldest_defect_minutes.map_or(age, |o| o.max(age)));
            }
        } else if let Some(amount) = row.deposit_amount {
            summary.deposit_total += amount;
        }
    }
    summary.clear = summary.defects == 0;
    // Baht, not satang: keep the total off a float cliff after summing.
    summary.deposit_total = (summary.deposit_total * 100.0).round() / 100.0;

    LoyaltyReconcile { summary, rows }
}

/// Rank a row's `kind` key back onto [`ReconcileKind`]'s `Ord`.
///
/// Rows carry the `&'static str` key (it is the wire contract) rather than the
/// enum, so the sort maps back. An unknown key sorts last instead of panicking:
/// a future kind added to the loader but not here should land at the bottom of
/// the list, not take the report down at 07:00.
fn kind_rank(key: &str) -> u8 {
    match key {
        "writeback_stalled" => 0,
        "legacy_hold_orphan" => 1,
        "sweep_lag" => 2,
        "unlinked_checkin" => 3,
        "deposit_divergence" => 4,
        _ => u8::MAX,
    }
}

/// Build a row from the common booking columns, filling the kind-specific
/// fields through the closure. Keeps the five loaders from each re-spelling
/// fifteen field initialisers.
#[allow(clippy::too_many_arguments)]
fn base_row(
    kind: ReconcileKind,
    report_date: NaiveDate,
    book_id: i32,
    book_no: String,
    book_status: String,
    check_in: NaiveDate,
    check_out: NaiveDate,
    room_no: Option<String>,
    guest_name: Option<String>,
    age_minutes: Option<i64>,
) -> ReconcileRow {
    ReconcileRow {
        kind: kind.key(),
        action: kind.action(),
        is_defect: kind.is_defect(),
        book_id,
        book_no,
        book_status,
        check_in,
        check_out,
        arrival: arrival_label(check_in, report_date),
        room_no,
        guest_name,
        age_minutes,
        age: age_label(age_minutes),
        hold_expires_at: None,
        deposit_amount: None,
        intent: None,
        job_status: None,
        cin_no: None,
        checked_in_at: None,
    }
}

// ---------------------------------------------------------------------------
// SQL fragments shared by the loaders
// ---------------------------------------------------------------------------

/// The booked room, as a correlated lateral. `LEFT JOIN LATERAL … ON TRUE` (not
/// a plain join) so a parked/roomless booking still produces its row with
/// `room_no = NULL` instead of vanishing — the same "never silently drop a
/// roomless booking" discipline the D3 rollup's `GREATEST(…, 1)` multiplier
/// applies. `ORDER BY` + `LIMIT 1` because a desk-grown multi-room booking has
/// several and the report shows the first; `uq_ht_br_bookroom` guarantees no
/// duplicate of the same room within a booking.
const ROOM_LATERAL: &str = r#"
    LEFT JOIN LATERAL (
        SELECT rn.room_no
          FROM ht_booking_rooms br
          JOIN ht_rooms_new rn ON rn.room_id = br.br_room_id
         WHERE br.br_book_id = b.book_id
         ORDER BY rn.room_no
         LIMIT 1
    ) rm ON TRUE
"#;

/// Guest display name. `cust_lastname` is nullable, so concatenating without
/// `COALESCE` would NULL the whole expression for a single-name guest — a very
/// common shape in the Thai registry. The outer `NULLIF(BTRIM(…), '')` collapses
/// a whitespace-only result back to SQL NULL so the row reads `None`, not `""`.
const GUEST_NAME_EXPR: &str = "NULLIF(BTRIM(COALESCE(cu.cust_firstname, '') || ' ' \
     || COALESCE(cu.cust_lastname, '')), '')";

/// Minutes elapsed since a timestamp column, as `bigint`.
fn age_minutes_expr(column: &str) -> String {
    format!("(EXTRACT(EPOCH FROM (now() - {column})) / 60)::bigint")
}

// ---------------------------------------------------------------------------
// Loaders
// ---------------------------------------------------------------------------

/// (a) + (b, second half) — loyalty bookings whose writeback has not applied.
///
/// **Predicate reused verbatim from Track F5's
/// `scheduler::sync::fetch_stalled_loyalty_writebacks`:**
///
/// ```sql
/// FROM writeback_jobs j JOIN ht_bookings b ON b.aggregate_id = j.aggregate_id
/// WHERE j.status <> 'done'
///   AND j.created_at <= now() - make_interval(mins => $threshold)
///   AND b.book_channel = 'loyalty'
/// ```
///
/// Every clause is load-bearing and none of it is re-derived here:
///
/// * **the join key is `aggregate_id`** — the only key the two tables share.
///   `writeback_jobs.aggregate_id` carries the booking UUID for all three
///   booking intents, and `ht_bookings.aggregate_id` is uniquely indexed
///   (`ux_ht_bookings_aggregate_id`). Selecting booking intents *through the
///   join* rather than with an `intent IN (…)` filter means a booking intent
///   added later is covered without touching this query — F5's decision,
///   inherited.
/// * **`status <> 'done'`, not an allow-list of bad statuses.** `pending`,
///   `in_progress`, `failed` and `exhausted` all mean iHOTEL has not seen the
///   row, and a future status should default to *appearing in the morning
///   report*, not to silence. `in_progress` matters most: a worker that dies
///   mid-claim leaves the row claimed, which is exactly the outage this
///   catches.
/// * **the same age threshold as the alert.** Sharing
///   [`DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES`] (10) is the point: a row on
///   this list is a row that would page, and a row that pages is on this list.
///   Without the floor the report would show healthy in-flight jobs seconds
///   old and train the desk to ignore the whole section.
///
/// The `threshold_minutes` bind is `i32` because it goes straight into
/// `make_interval(mins => $2)`, which PostgreSQL overloads only on `int`; a
/// `bigint` bind raises `function make_interval(mins => bigint) does not exist`
/// and would silently take this section of the report offline.
///
/// Runs on the existing `ix_writeback_jobs_claim` partial index plus a lookup
/// per row on `ux_ht_bookings_aggregate_id` — no new index, no migration.
pub async fn load_writeback_gaps(
    pool: &PgPool,
    report_date: NaiveDate,
    threshold_minutes: i32,
) -> Result<Vec<ReconcileRow>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT b.book_id,
               b.book_no,
               b.book_status,
               b.book_checkin,
               b.book_checkout,
               b.book_hold_expires_at,
               j.intent,
               j.status AS job_status,
               {age} AS age_minutes,
               rm.room_no,
               {guest} AS guest_name
          FROM writeback_jobs j
          JOIN ht_bookings b ON b.aggregate_id = j.aggregate_id
          LEFT JOIN ht_customers cu ON cu.cust_id = b.book_cust_id
          {room_lateral}
         WHERE j.status <> $1
           AND j.created_at <= now() - make_interval(mins => $2)
           AND b.book_channel = $3
         ORDER BY j.created_at
         LIMIT $4
        "#,
        age = age_minutes_expr("j.created_at"),
        guest = GUEST_NAME_EXPR,
        room_lateral = ROOM_LATERAL,
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(WRITEBACK_APPLIED_STATUS)
        .bind(threshold_minutes)
        .bind(LOYALTY_CHANNEL)
        .bind(MAX_ROWS_PER_KIND)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .iter()
        .map(|r| {
            let intent: String = r.get("intent");
            let book_status: String = r.get("book_status");
            let kind = classify_writeback_gap(&intent, &book_status);
            let mut row = base_row(
                kind,
                report_date,
                r.get("book_id"),
                r.get("book_no"),
                book_status,
                r.get("book_checkin"),
                r.get("book_checkout"),
                r.try_get("room_no").unwrap_or(None),
                r.try_get("guest_name").unwrap_or(None),
                Some(r.try_get::<i64, _>("age_minutes").unwrap_or(0)),
            );
            row.hold_expires_at = r.try_get("book_hold_expires_at").unwrap_or(None);
            row.job_status = Some(r.get("job_status"));
            row.intent = Some(intent);
            row
        })
        .collect())
}

/// (b, first half) — holds past their payment deadline that the sweep has not
/// released.
///
/// **Predicate reused verbatim from the expiry sweep's own input query,
/// `repository::channel::expired_hold_ids`:**
///
/// ```sql
/// WHERE book_channel = 'loyalty'
///   AND book_status = 'pending'
///   AND book_hold_expires_at IS NOT NULL
///   AND book_hold_expires_at < NOW()
/// ```
///
/// All four clauses, in the same order, because the set this report shows must
/// be *exactly* the set the 5-minute sweep would pick up on its next tick. A
/// row here means the sweep has not run, has failed on this row, or is behind —
/// and the desk is looking at inventory our own app is still holding against a
/// guest who has not paid.
///
/// `book_status = 'pending'` (not "non-terminal") is the same guard
/// `release_hold` uses, so the report can never list a hold that a racing
/// payment-verified has already confirmed — telling the desk a paid guest lost
/// their room is worse than saying nothing, the same failure Track F5's
/// `format_hold_suffix` guards against.
///
/// Served by `ix_ht_bookings_hold_expiry` (migration 086), the partial index
/// the sweep itself runs on.
pub async fn load_sweep_lag(
    pool: &PgPool,
    report_date: NaiveDate,
) -> Result<Vec<ReconcileRow>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT b.book_id,
               b.book_no,
               b.book_status,
               b.book_checkin,
               b.book_checkout,
               b.book_hold_expires_at,
               COALESCE(b.book_deposit_amount, 0)::float8 AS deposit_amount,
               {age} AS age_minutes,
               rm.room_no,
               {guest} AS guest_name
          FROM ht_bookings b
          LEFT JOIN ht_customers cu ON cu.cust_id = b.book_cust_id
          {room_lateral}
         WHERE b.book_channel = $1
           AND b.book_status = $2
           AND b.book_hold_expires_at IS NOT NULL
           AND b.book_hold_expires_at < NOW()
         ORDER BY b.book_hold_expires_at
         LIMIT $3
        "#,
        age = age_minutes_expr("b.book_hold_expires_at"),
        guest = GUEST_NAME_EXPR,
        room_lateral = ROOM_LATERAL,
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(LOYALTY_CHANNEL)
        .bind(CHANNEL_HOLD_STATUS)
        .bind(MAX_ROWS_PER_KIND)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .iter()
        .map(|r| {
            let mut row = base_row(
                ReconcileKind::SweepLag,
                report_date,
                r.get("book_id"),
                r.get("book_no"),
                r.get("book_status"),
                r.get("book_checkin"),
                r.get("book_checkout"),
                r.try_get("room_no").unwrap_or(None),
                r.try_get("guest_name").unwrap_or(None),
                Some(r.try_get::<i64, _>("age_minutes").unwrap_or(0)),
            );
            row.hold_expires_at = r.try_get("book_hold_expires_at").unwrap_or(None);
            row.deposit_amount = r
                .try_get::<f64, _>("deposit_amount")
                .ok()
                .filter(|d| *d > 0.0);
            row
        })
        .collect())
}

/// (c) — app bookings arriving today/tomorrow that carry a deposit iHOTEL will
/// show as 0.
///
/// **Not a defect and not a reused predicate — an accepted divergence**, from
/// `docs/loyalty-channel.md` §"Dual-write policy for holds": payment-verified
/// is a PG-only flip, because the validated `booking_modify` recipe has no
/// deposit (`Book_Price_Pay`) leg and inventing one would break byte-parity.
/// iHOTEL therefore shows the booking with deposit 0 until checkout. That is
/// deliberate and permanent.
///
/// It is on the morning list because the *consequence* is not permanent: a
/// receptionist reading `0` off the iHOTEL screen will ask a guest to pay money
/// they have already transferred. So the desk needs the number before the guest
/// arrives, and B7's folio signposts only render on a check-in that carries the
/// booking link — which bucket (d) exists to say is sometimes missing.
///
/// Cancelled bookings are excluded with `IS DISTINCT FROM 'cancelled'` rather
/// than `<>`: `book_status` is nullable, and `<>` would silently drop a
/// NULL-status booking from BOTH sides of the test. Same call the D3 rollup
/// makes for the same column.
pub async fn load_deposit_divergence(
    pool: &PgPool,
    report_date: NaiveDate,
    horizon_days: i64,
) -> Result<Vec<ReconcileRow>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT b.book_id,
               b.book_no,
               b.book_status,
               b.book_checkin,
               b.book_checkout,
               b.book_hold_expires_at,
               COALESCE(b.book_deposit_amount, 0)::float8 AS deposit_amount,
               rm.room_no,
               {guest} AS guest_name
          FROM ht_bookings b
          LEFT JOIN ht_customers cu ON cu.cust_id = b.book_cust_id
          {room_lateral}
         WHERE b.book_channel = $1
           AND COALESCE(b.book_deposit_amount, 0) > 0
           AND b.book_checkin >= $2::date
           AND b.book_checkin <= $3::date
           AND b.book_status IS DISTINCT FROM $4
         ORDER BY b.book_checkin, b.book_no
         LIMIT $5
        "#,
        guest = GUEST_NAME_EXPR,
        room_lateral = ROOM_LATERAL,
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(LOYALTY_CHANNEL)
        .bind(report_date)
        .bind(report_date + chrono::Duration::days(horizon_days))
        .bind(CANCELLED_STATUS)
        .bind(MAX_ROWS_PER_KIND)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .iter()
        .map(|r| {
            let mut row = base_row(
                ReconcileKind::DepositDivergence,
                report_date,
                r.get("book_id"),
                r.get("book_no"),
                r.get("book_status"),
                r.get("book_checkin"),
                r.get("book_checkout"),
                r.try_get("room_no").unwrap_or(None),
                r.try_get("guest_name").unwrap_or(None),
                // No clock is running on this kind — see `age_label`.
                None,
            );
            row.hold_expires_at = r.try_get("book_hold_expires_at").unwrap_or(None);
            row.deposit_amount = Some(r.try_get::<f64, _>("deposit_amount").unwrap_or(0.0));
            row
        })
        .collect())
}

/// (d) — app bookings whose guest is checked in with no canonical link.
///
/// The B7a gap. A guest checked in through iHOTEL (or as a walk-in on our own
/// room board) leaves `ht_checkins.cin_book_id` NULL, and every B7 app-deposit
/// signpost resolves through that link — `GET /api/checkins/:id/deposits` joins
/// `ht_bookings` via `ci.cin_book_id`. With it NULL the notice never appears,
/// and the desk takes payment at checkout for a deposit the guest already
/// transferred in the app. Nobody at the counter can see that this has
/// happened, which is exactly why it needs a morning list.
///
/// **Overlap predicate reused verbatim from the check-in leg of
/// `repository::channel::FREE_ROOM_PREDICATE`** — the same expression the
/// availability counter and the room picker share, so "a check-in occupying
/// this booking's room over these dates" means the same thing here as it does
/// when the channel decides a room is free:
///
/// ```sql
/// ci.cin_status <> 'cancelled'
/// AND (ci.cin_room_id = <room> OR EXISTS (
///       SELECT 1 FROM ht_checkin_rooms cr
///        WHERE cr.cr_cin_id = ci.cin_id AND cr.cr_room_id = <room>))
/// AND ci.cin_checkin_time::date < <booking check-out>
/// AND COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date > <booking check-in>
/// ```
///
/// The `ht_checkin_rooms` arm is not optional decoration: a multi-room stay
/// records its rooms there, not in `cin_room_id`, so dropping it would miss
/// every app booking whose guest iHOTEL checked in as part of a multi-room
/// party — which is the ONLY way a multi-room stay can exist, since our app
/// rejects multi-room check-in.
///
/// Scope: bookings whose stay overlaps `[report_date, report_date + horizon]`,
/// so the list covers in-house guests and today's/tomorrow's arrivals without
/// dragging in history. `LEFT JOIN … WHERE ci.cin_id IS NOT NULL` is expressed
/// as a plain join; the "no link" half is `ci.cin_book_id IS NULL`.
///
/// One row per (booking, unlinked check-in) pair. A booking cannot normally
/// match two, but if it does the desk should see both.
pub async fn load_unlinked_checkins(
    pool: &PgPool,
    report_date: NaiveDate,
    horizon_days: i64,
) -> Result<Vec<ReconcileRow>, sqlx::Error> {
    let sql = format!(
        r#"
        SELECT b.book_id,
               b.book_no,
               b.book_status,
               b.book_checkin,
               b.book_checkout,
               COALESCE(b.book_deposit_amount, 0)::float8 AS deposit_amount,
               ci.cin_no,
               ci.cin_checkin_time,
               {age} AS age_minutes,
               rn.room_no,
               {guest} AS guest_name
          FROM ht_bookings b
          JOIN ht_booking_rooms br ON br.br_book_id = b.book_id
          JOIN ht_rooms_new rn ON rn.room_id = br.br_room_id
          JOIN ht_checkins ci
            ON ci.cin_status <> 'cancelled'
           AND (ci.cin_room_id = br.br_room_id OR EXISTS (
                   SELECT 1 FROM ht_checkin_rooms cr
                    WHERE cr.cr_cin_id = ci.cin_id
                      AND cr.cr_room_id = br.br_room_id))
           AND ci.cin_checkin_time::date < b.book_checkout
           AND COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date > b.book_checkin
          LEFT JOIN ht_customers cu ON cu.cust_id = b.book_cust_id
         WHERE b.book_channel = $1
           AND b.book_status IS DISTINCT FROM $2
           AND ci.cin_book_id IS NULL
           AND b.book_checkin <= $3::date
           AND b.book_checkout > $4::date
         ORDER BY ci.cin_checkin_time, b.book_no
         LIMIT $5
        "#,
        age = age_minutes_expr("ci.cin_checkin_time"),
        guest = GUEST_NAME_EXPR,
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(LOYALTY_CHANNEL)
        .bind(CANCELLED_STATUS)
        .bind(report_date + chrono::Duration::days(horizon_days))
        .bind(report_date)
        .bind(MAX_ROWS_PER_KIND)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .iter()
        .map(|r| {
            let mut row = base_row(
                ReconcileKind::UnlinkedCheckin,
                report_date,
                r.get("book_id"),
                r.get("book_no"),
                r.get("book_status"),
                r.get("book_checkin"),
                r.get("book_checkout"),
                r.try_get("room_no").unwrap_or(None),
                r.try_get("guest_name").unwrap_or(None),
                Some(r.try_get::<i64, _>("age_minutes").unwrap_or(0)),
            );
            row.cin_no = r.try_get("cin_no").unwrap_or(None);
            row.checked_in_at = r.try_get("cin_checkin_time").unwrap_or(None);
            row.deposit_amount = r
                .try_get::<f64, _>("deposit_amount")
                .ok()
                .filter(|d| *d > 0.0);
            row
        })
        .collect())
}

/// Run all four loaders and fold the result.
///
/// Sequential, not `try_join!`: each query is an indexed lookup that returns
/// zero rows on a healthy morning, and they share one pool whose connections
/// are the scarce resource. Four round trips at shift open is not a budget
/// worth optimising, and a serial call stack makes a failure attributable to
/// one kind instead of one of four.
///
/// `age_minutes` is computed by PostgreSQL's `now()` inside each query rather
/// than in Rust against a captured clock, so every age is measured against the
/// same authority that wrote the timestamps — no drift between the API
/// container's clock and the database's.
pub async fn load_loyalty_reconcile(
    pool: &PgPool,
    report_date: NaiveDate,
    horizon_days: i64,
    stall_threshold_minutes: i32,
) -> Result<LoyaltyReconcile, sqlx::Error> {
    let mut rows = load_writeback_gaps(pool, report_date, stall_threshold_minutes).await?;
    rows.extend(load_sweep_lag(pool, report_date).await?);
    rows.extend(load_unlinked_checkins(pool, report_date, horizon_days).await?);
    rows.extend(load_deposit_divergence(pool, report_date, horizon_days).await?);
    Ok(reconcile(rows))
}

/// The stall threshold this report shares with Track F5's alert, resolved from
/// the same `LOYALTY_WRITEBACK_STALL_ALERT_MINUTES` override.
///
/// Re-resolved here (rather than calling F5's private `loyalty_writeback_stall_minutes`)
/// with identical semantics: clamp to a floor of 1, fall back to
/// [`DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES`] on missing/unparseable. A
/// `0`-minute threshold would list every job the instant it is enqueued.
pub fn stall_threshold_minutes() -> i32 {
    std::env::var("LOYALTY_WRITEBACK_STALL_ALERT_MINUTES")
        .ok()
        .and_then(|v| v.trim().parse::<i32>().ok())
        .filter(|m| *m >= 1)
        .unwrap_or(DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn row(kind: ReconcileKind, book_no: &str, age: Option<i64>) -> ReconcileRow {
        base_row(
            kind,
            date(2099, 3, 10),
            1,
            book_no.to_string(),
            "pending".to_string(),
            date(2099, 3, 10),
            date(2099, 3, 12),
            Some("301".to_string()),
            Some("Somchai T".to_string()),
            age,
        )
    }

    // ---- classify_writeback_gap: the (a) / (b) split -------------------

    #[test]
    fn a_cancelled_booking_with_a_stuck_cancel_is_a_phantom_jong() {
        // The hold is dead in PG; only iHOTEL still shows `จอง`.
        assert_eq!(
            classify_writeback_gap("cancel_booking", "cancelled"),
            ReconcileKind::LegacyHoldOrphan
        );
        assert_eq!(
            ReconcileKind::LegacyHoldOrphan.action(),
            "resend_hold",
            "nobody's stay is at risk — the desk re-drives the leg"
        );
    }

    #[test]
    fn a_stuck_create_is_the_missing_twin_kind() {
        assert_eq!(
            classify_writeback_gap("create_booking", "pending"),
            ReconcileKind::WritebackStalled
        );
        assert_eq!(
            classify_writeback_gap("create_booking", "confirmed"),
            ReconcileKind::WritebackStalled
        );
        assert_eq!(
            classify_writeback_gap("modify_booking", "confirmed"),
            ReconcileKind::WritebackStalled
        );
    }

    #[test]
    fn a_cancel_that_lost_its_race_is_still_the_dangerous_kind() {
        // `release_hold` is guarded on `book_status='pending'`, so a cancel
        // that raced a payment-verified writes 0 rows and the booking stays
        // `confirmed`. That guest has PAID: the outstanding legacy work is a
        // missing twin, not a phantom to be cleaned up.
        assert_eq!(
            classify_writeback_gap("cancel_booking", "confirmed"),
            ReconcileKind::WritebackStalled
        );
        assert_eq!(ReconcileKind::WritebackStalled.action(), "call_guest");
    }

    #[test]
    fn a_cancelled_booking_with_a_stuck_create_is_not_a_phantom() {
        // No twin was ever written, so there is no `จอง` in iHOTEL to chase.
        assert_eq!(
            classify_writeback_gap("create_booking", "cancelled"),
            ReconcileKind::WritebackStalled
        );
    }

    // ---- age_label -----------------------------------------------------

    #[test]
    fn age_label_renders_minutes_hours_and_days() {
        assert_eq!(age_label(Some(0)), "0m");
        assert_eq!(age_label(Some(59)), "59m");
        assert_eq!(age_label(Some(60)), "1h 00m");
        assert_eq!(age_label(Some(134)), "2h 14m");
        assert_eq!(age_label(Some(1439)), "23h 59m");
        assert_eq!(age_label(Some(1440)), "1d 0h");
        assert_eq!(age_label(Some(3000)), "2d 2h");
    }

    #[test]
    fn age_label_is_empty_when_no_clock_is_running() {
        // The deposit kind is a briefing item, not a late row. `0m` there
        // would imply something is being timed.
        assert_eq!(age_label(None), "");
    }

    #[test]
    fn age_label_never_renders_a_negative_countdown() {
        // Reachable only through clock skew. "-7m" at 07:00 is a puzzle.
        assert_eq!(age_label(Some(-7)), "0m");
        assert_eq!(age_label(Some(-9999)), "0m");
    }

    // ---- arrival_label -------------------------------------------------

    #[test]
    fn arrival_label_names_today_and_tomorrow() {
        let today = date(2026, 9, 11);
        assert_eq!(arrival_label(date(2026, 9, 11), today), "today");
        assert_eq!(arrival_label(date(2026, 9, 12), today), "tomorrow");
        assert_eq!(arrival_label(date(2026, 9, 14), today), "+3d");
        assert_eq!(arrival_label(date(2026, 9, 9), today), "-2d");
    }

    #[test]
    fn arrival_label_crosses_a_month_boundary() {
        assert_eq!(
            arrival_label(date(2026, 10, 1), date(2026, 9, 30)),
            "tomorrow"
        );
    }

    // ---- kind ranking + action mapping ---------------------------------

    #[test]
    fn kinds_rank_by_blast_radius() {
        // A double-sold room outranks a phantom hold outranks sweep lag
        // outranks paperwork outranks a briefing item.
        let mut kinds = vec![
            ReconcileKind::DepositDivergence,
            ReconcileKind::SweepLag,
            ReconcileKind::WritebackStalled,
            ReconcileKind::UnlinkedCheckin,
            ReconcileKind::LegacyHoldOrphan,
        ];
        kinds.sort();
        assert_eq!(
            kinds,
            vec![
                ReconcileKind::WritebackStalled,
                ReconcileKind::LegacyHoldOrphan,
                ReconcileKind::SweepLag,
                ReconcileKind::UnlinkedCheckin,
                ReconcileKind::DepositDivergence,
            ]
        );
        // `kind_rank` must agree with the derived Ord, since the rows carry
        // the string key and not the enum.
        let ranks: Vec<u8> = kinds.iter().map(|k| kind_rank(k.key())).collect();
        assert_eq!(ranks, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn only_the_deposit_kind_is_not_a_defect() {
        assert!(ReconcileKind::WritebackStalled.is_defect());
        assert!(ReconcileKind::LegacyHoldOrphan.is_defect());
        assert!(ReconcileKind::SweepLag.is_defect());
        assert!(ReconcileKind::UnlinkedCheckin.is_defect());
        assert!(
            !ReconcileKind::DepositDivergence.is_defect(),
            "iHOTEL showing deposit 0 is documented and permanent — counting \
             it as a defect makes a healthy morning read red"
        );
    }

    #[test]
    fn every_kind_maps_to_one_of_the_three_desk_verbs() {
        for kind in [
            ReconcileKind::WritebackStalled,
            ReconcileKind::LegacyHoldOrphan,
            ReconcileKind::SweepLag,
            ReconcileKind::UnlinkedCheckin,
            ReconcileKind::DepositDivergence,
        ] {
            assert!(
                ["call_guest", "resend_hold", "brief_desk"].contains(&kind.action()),
                "{} has no desk verb",
                kind.key()
            );
        }
    }

    // ---- reconcile: ordering + summary ---------------------------------

    #[test]
    fn rows_sort_by_kind_then_oldest_first() {
        let out = reconcile(vec![
            row(ReconcileKind::DepositDivergence, "D1", None),
            row(ReconcileKind::SweepLag, "S1", Some(12)),
            row(ReconcileKind::WritebackStalled, "W1", Some(30)),
            row(ReconcileKind::WritebackStalled, "W2", Some(120)),
            row(ReconcileKind::LegacyHoldOrphan, "L1", Some(45)),
            row(ReconcileKind::UnlinkedCheckin, "U1", Some(600)),
        ]);

        let order: Vec<&str> = out.rows.iter().map(|r| r.book_no.as_str()).collect();
        // W2 (120m) before W1 (30m) — oldest first within a kind.
        assert_eq!(order, vec!["W2", "W1", "L1", "S1", "U1", "D1"]);
    }

    #[test]
    fn equal_rows_order_deterministically_by_book_no() {
        // Two refreshes must not swap rows, or the desk cannot tell "same
        // list" from "new problem".
        let out = reconcile(vec![
            row(ReconcileKind::WritebackStalled, "AB-02", Some(30)),
            row(ReconcileKind::WritebackStalled, "AB-01", Some(30)),
        ]);
        let order: Vec<&str> = out.rows.iter().map(|r| r.book_no.as_str()).collect();
        assert_eq!(order, vec!["AB-01", "AB-02"]);
    }

    #[test]
    fn summary_counts_each_kind_and_separates_defects() {
        let mut deposit = row(ReconcileKind::DepositDivergence, "D1", None);
        deposit.deposit_amount = Some(1200.50);
        let mut deposit2 = row(ReconcileKind::DepositDivergence, "D2", None);
        deposit2.deposit_amount = Some(800.25);

        let out = reconcile(vec![
            row(ReconcileKind::WritebackStalled, "W1", Some(30)),
            row(ReconcileKind::WritebackStalled, "W2", Some(45)),
            row(ReconcileKind::LegacyHoldOrphan, "L1", Some(90)),
            row(ReconcileKind::SweepLag, "S1", Some(7)),
            row(ReconcileKind::UnlinkedCheckin, "U1", Some(300)),
            deposit,
            deposit2,
        ]);

        assert_eq!(out.summary.total, 7);
        assert_eq!(out.summary.writeback_stalled, 2);
        assert_eq!(out.summary.legacy_hold_orphan, 1);
        assert_eq!(out.summary.sweep_lag, 1);
        assert_eq!(out.summary.unlinked_checkin, 1);
        assert_eq!(out.summary.deposit_divergence, 2);
        // The deposit rows are not defects.
        assert_eq!(out.summary.defects, 5);
        assert!(!out.summary.clear);
        // Oldest across DEFECTS only — the 300m unlinked check-in.
        assert_eq!(out.summary.oldest_defect_minutes, Some(300));
        assert_eq!(out.summary.deposit_total, 2000.75);
    }

    #[test]
    fn a_deposit_only_morning_is_clear() {
        // The common healthy shape once the channel is live: app guests are
        // arriving with deposits and nothing is broken. This MUST read clear,
        // or the routine gets abandoned inside a week.
        let mut deposit = row(ReconcileKind::DepositDivergence, "D1", None);
        deposit.deposit_amount = Some(990.0);

        let out = reconcile(vec![deposit]);
        assert_eq!(out.summary.total, 1);
        assert_eq!(out.summary.defects, 0);
        assert!(out.summary.clear);
        assert_eq!(out.summary.oldest_defect_minutes, None);
        assert_eq!(out.summary.deposit_total, 990.0);
    }

    #[test]
    fn an_empty_morning_is_clear_and_totals_zero() {
        let out = reconcile(vec![]);
        assert_eq!(out.rows, vec![]);
        assert_eq!(
            out.summary,
            ReconcileSummary {
                clear: true,
                ..Default::default()
            }
        );
        assert!(out.summary.clear);
        assert_eq!(out.summary.deposit_total, 0.0);
    }

    #[test]
    fn deposit_total_stays_on_the_satang() {
        // Three baht amounts that sum off a float cliff without rounding.
        let amounts = [1234.56, 78.91, 0.03];
        let rows: Vec<ReconcileRow> = amounts
            .iter()
            .enumerate()
            .map(|(i, amt)| {
                let mut r = row(ReconcileKind::DepositDivergence, &format!("D{i}"), None);
                r.deposit_amount = Some(*amt);
                r
            })
            .collect();
        assert_eq!(reconcile(rows).summary.deposit_total, 1313.50);
    }

    #[test]
    fn base_row_fills_the_derived_display_fields() {
        let r = row(ReconcileKind::WritebackStalled, "AB-01", Some(134));
        assert_eq!(r.kind, "writeback_stalled");
        assert_eq!(r.action, "call_guest");
        assert!(r.is_defect);
        assert_eq!(r.age, "2h 14m");
        assert_eq!(r.arrival, "today");
    }

    // ---- shared-constant pins ------------------------------------------

    #[test]
    fn the_writeback_applied_status_is_shared_with_f5() {
        // Imported, not re-spelled — this pins the import against a rename.
        assert_eq!(WRITEBACK_APPLIED_STATUS, "done");
    }

    #[test]
    fn the_sweep_predicate_literals_match_the_sweep() {
        // `repository::channel::expired_hold_ids` keeps these inline in SQL
        // inside a file this module must not edit, so they are restated here.
        // If that query changes, this test is the tripwire.
        assert_eq!(CHANNEL_HOLD_STATUS, "pending");
        assert_eq!(CANCELLED_STATUS, "cancelled");
        assert_eq!(LOYALTY_CHANNEL, "loyalty");
    }

    #[test]
    fn the_stall_threshold_matches_the_f5_alert_default() {
        // A row on this list must be a row that would page, and vice versa.
        assert_eq!(DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES, 10);
    }
}
