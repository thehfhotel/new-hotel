//! Change Tracking Watcher Binary (`bin/sync.rs`).
//!
//! Per `docs/architecture.md` §3.6d, §4d-tris, §10 #8.
//!
//! ## Lifecycle
//!
//! 1. Parse env (`LEGACY_SYNC_ENABLED`, `LEGACY_SYNC_SHADOW_MODE`,
//!    `LEGACY_SYNC_TABLE_ALLOWLIST`, `CT_POLL_INTERVAL_MS`).
//! 2. If disabled (`LEGACY_SYNC_ENABLED != "true"`) → log + exit 0
//!    (intentional disable, NOT a failure).
//! 3. Open PG (sqlx) + MSSQL (tiberius/bb8) pools.
//! 4. Verify legacy schema fingerprint — abort + Slack alert on drift.
//! 5. Build `Vec<Box<dyn MssqlChangeMapper>>` — one per CT-enabled
//!    table, filtered by allowlist. Phase 5.2 ships real mappers for
//!    `HT_Customers` / `HT_Rooms` / `HT_Room_Status`; the rest stay on
//!    `NoopMapper` until 5.3 / 5.4.
//! 6. Main loop: every `CT_POLL_INTERVAL_MS` (default 1000ms):
//!    a. Read `legacy_ct_state.last_seen_version`.
//!    b. For each mapper, in panic-isolated tasks:
//!       - Verify `MIN_VALID_VERSION(<table>) <= last_seen_version`
//!         (else → CT retention overflow → Slack alert + skip).
//!       - Query `CHANGETABLE(CHANGES <table>, @last) JOIN <table>`.
//!         (No `SYS_CHANGE_CONTEXT` filter — writeback echoes are
//!         absorbed by mapper idempotency; see `build_ct_changes_sql`.)
//!       - For each row: `mapper.apply(&mut tx, op, Some(&row)).await`
//!         (or `None` for D operations). On success, INSERT into
//!         `event_log` (live mode) or log "would publish" (shadow mode).
//!       - Capture per-table max(SYS_CHANGE_VERSION).
//!       - Update `legacy_sync_status.rows_ingested` /  `rows_skipped`.
//!       - Commit (live) / rollback (shadow) in the same TX.
//!       - Advance the watermark to per-table max.
//! 7. SIGTERM → finish current tick, then exit cleanly.
//!
//! ## Watermark advance — per-table, not min-of-all
//!
//! Phase 5.2 advances the watermark per-table at the end of each
//! table's own TX. Customer / room mappers are flat tables — one CT row
//! produces at most one mapper call and at most one event, so per-table
//! advance is equivalent to min-of-all and simpler.
//!
//! **Phase 5.3 keeps per-table watermark advance** even though the
//! booking aggregate spans HT_Book_H + HT_Book_Ds + HT_Book_Date.
//! `SYS_CHANGE_VERSION` is global-monotonic across the whole database,
//! so committing the per-table max version after a successful TX
//! cannot lose rows from any other table. The "one event per
//! aggregate per tick" guarantee is enforced one level up — by
//! coalescing CT rows in `poll_table` (see the per-row vs coalesced
//! branches there), not by watermark mechanics. Net effect: a booking
//! header + 2 line + 5 night change touches all three tables, but
//! produces exactly one DomainEvent (header tick emits the
//! BookingModified; the subsequent Ds + Date ticks load the same
//! aggregate, find the canonical row already matches, and skip the
//! event).

#![allow(clippy::doc_lazy_continuation)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::Notify;

use hotel_backend::config::{DbConfig, SiteConfig, SlackConfig};
use hotel_backend::db::mssql_timeout::{
    simple_query_with_explicit_timeout, simple_query_with_timeout_pooled, MssqlOpKind,
};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::notifications::slack::{SlackClient, SlackMessage};
use hotel_backend::outbox::bus::EventBus;
use hotel_backend::outbox::event::DomainEvent;
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mappers::{
    apply_booking_aggregate, apply_checkin_aggregate, apply_payment_aggregate,
    BillDebtDsMirrorMapper, BillDebtHMirrorMapper, BookProMirrorMapper, BookingDatesMapper,
    BookingHeaderMapper, BookingRoomsMapper, ChangedRoomMirrorMapper, CheckInHeaderMapper,
    CheckInRoomsMapper, CheckinProductMirrorMapper, CuponMirrorMapper, CustomerMapper,
    DepositMirrorMapper, GuestRegistryMapper, PaymentMapper, ReceiptMapper, RoomCalendarMapper,
    RoomMasterMapper, RoomsCancelMirrorMapper,
};
use hotel_backend::sync::parent_loader::{load_booking_aggregate, load_checkin_aggregate};
use hotel_backend::sync::row::MappableRow;
use hotel_backend::sync::{MssqlChangeMapper, NoopMapper};
use hotel_backend::writeback::verify_ct_schema_fingerprint;

// ============================================================================
// Resilience PR R1 — structured error taxonomy for the CT watcher
// ============================================================================
//
// Background. The 2026-05-14 HF-Hotel CT watermark stall (74-min outage) was
// rendered forensically opaque because:
//   * The pre-restart `docker logs` rotated away when `sync-1` was recreated
//     by the deploy.
//   * `legacy_sync_status.last_error` was overwritten by post-restart healthy
//     ticks (per-table counter bumps clear the row's error fields on the
//     first 0-row tick after recovery).
//   * The `tracing::error!` lines used free-form strings — no stable
//     `event_name` to grep across pre- and post-incident log windows.
//
// The taxonomy below assigns a stable, dot-delimited identifier to every
// failure mode the tick loop can surface. Two consumers benefit:
//   1. `tracing` JSON output — `event_name` becomes a top-level field
//      operators can filter on in Loki / journalctl / `jq`.
//   2. `legacy_sync_status.last_error` — when persisted via
//      `record_table_error`, the event_name is prefixed (`"sync.foo: <msg>"`)
//      so the next operator triaging a stalled table can see the failure
//      MODE even after the original log line is gone.
//
// Adding a new event name. Define a `const EV_…: &str = "sync.…"`, hand it
// to the relevant `tracing::error!` / `record_table_error` call, and add a
// `KNOWN_SYNC_EVENT_NAMES` entry below so the registry test stays green.

/// Failed to read the CT watermark (`legacy_ct_state.last_seen_version`)
/// at the top of a tick. PG-side read error — usually transient.
pub const EV_WATERMARK_READ_FAIL: &str = "sync.watermark_read_fail";

/// Failed to advance the watermark after a successful tick. PG UPDATE
/// failure — leaves the watcher at risk of re-fetching the same CT rows
/// next tick (idempotent, but observability flag).
pub const EV_WATERMARK_ADVANCE_FAIL: &str = "sync.watermark_advance_fail";

/// Legacy MSSQL connectivity probe failed at the top of a tick. Tunnel
/// flap or pool exhaustion. Short-circuits the entire tick.
pub const EV_LEGACY_PROBE_FAIL: &str = "sync.legacy_probe_fail";

/// CT retention guard failed — could not query `CHANGE_TRACKING_MIN_VALID_VERSION`
/// for the table. NOT the same as a retention OVERFLOW (that's
/// `EV_CT_RETENTION_OVERFLOW`); this is the round-trip itself failing.
pub const EV_RETENTION_CHECK_FAIL: &str = "sync.retention_check_fail";

/// CT retention overflow detected — `MIN_VALID_VERSION > last_seen_version`.
/// Watcher cannot resume without bootstrap. Paged via Slack separately.
pub const EV_CT_RETENTION_OVERFLOW: &str = "sync.ct_retention_overflow";

/// `SELECT … FROM CHANGETABLE(CHANGES …) COUNT(*)` query failed
/// (NoopMapper path). Usually a tiberius / pool error.
pub const EV_CT_COUNT_FAIL: &str = "sync.ct_count_fail";

/// `SELECT … FROM CHANGETABLE(CHANGES …) JOIN <table>` failed (real
/// mapper path). Usually a tiberius / pool error.
pub const EV_CT_FETCH_FAIL: &str = "sync.ct_fetch_fail";

/// Failed to begin the per-table PG transaction at the top of `poll_table`.
/// Almost always pool exhaustion.
pub const EV_PG_TX_BEGIN_FAIL: &str = "sync.pg_tx_begin_fail";

/// Failed to commit the per-table PG transaction. Leaves the canonical
/// projection mid-update; the next tick re-fetches the same CT range and
/// retries idempotently.
pub const EV_PG_TX_COMMIT_FAIL: &str = "sync.pg_tx_commit_fail";

/// `parent_loader::load_booking_aggregate` / `load_checkin_aggregate`
/// returned an error. The aggregate row is skipped this tick; the next
/// CT tick on the same key will retry.
pub const EV_LOAD_AGGREGATE_FAIL: &str = "sync.load_aggregate_fail";

/// `apply_*_aggregate` (coalesced path) returned a domain-level error —
/// usually an FK resolution miss or a UUID derivation failure. Recorded
/// and skipped.
pub const EV_AGGREGATE_APPLY_FAIL: &str = "sync.aggregate_apply_fail";

/// Per-row `mapper.apply` (Phase 5.2 dispatch path: customer / room /
/// room_status) returned an error.
pub const EV_MAPPER_APPLY_FAIL: &str = "sync.mapper_apply_fail";

/// `persist_event` (event_log INSERT) failed. Canonical row already
/// applied; the missing event_log row is visible to operators as a
/// counter drift.
pub const EV_PERSIST_EVENT_FAIL: &str = "sync.persist_event_fail";

/// Failed to UPDATE `legacy_sync_status` counters at the end of a tick.
/// Pure observability degradation — canonical state is unaffected.
pub const EV_STATUS_UPDATE_FAIL: &str = "sync.status_update_fail";

/// Caught a CT row with `SYS_CHANGE_OPERATION` outside `{I,U,D}`.
/// Indicates either a CT bug or a schema drift; never observed in prod.
pub const EV_UNKNOWN_CT_OP: &str = "sync.unknown_ct_op";

/// D-event orphan recovery on `HT_CheckIn_Ds` could not resolve the
/// parent `Cin_no` via `ht_checkins.legacy_checkin_ds_id`. Canonical
/// aggregate may stay stale until the next CT tick on the parent header.
pub const EV_ORPHAN_RECOVERY_FAIL: &str = "sync.orphan_recovery_fail";

/// Shadow-mode rollback of the per-table TX failed. Cosmetic — the TX
/// was already a no-op for canonical state by design of shadow mode.
pub const EV_SHADOW_ROLLBACK_FAIL: &str = "sync.shadow_rollback_fail";

/// Registry of every event_name emitted by the watcher. Kept as a
/// const array so the unit test below can lock in the set — a refactor
/// that adds a new event must add it here too. Order is not significant
/// (the registry is membership-tested, not pattern-matched).
const KNOWN_SYNC_EVENT_NAMES: &[&str] = &[
    EV_WATERMARK_READ_FAIL,
    EV_WATERMARK_ADVANCE_FAIL,
    EV_LEGACY_PROBE_FAIL,
    EV_RETENTION_CHECK_FAIL,
    EV_CT_RETENTION_OVERFLOW,
    EV_CT_COUNT_FAIL,
    EV_CT_FETCH_FAIL,
    EV_PG_TX_BEGIN_FAIL,
    EV_PG_TX_COMMIT_FAIL,
    EV_LOAD_AGGREGATE_FAIL,
    EV_AGGREGATE_APPLY_FAIL,
    EV_MAPPER_APPLY_FAIL,
    EV_PERSIST_EVENT_FAIL,
    EV_STATUS_UPDATE_FAIL,
    EV_UNKNOWN_CT_OP,
    EV_ORPHAN_RECOVERY_FAIL,
    EV_SHADOW_ROLLBACK_FAIL,
];

/// Cap on the `legacy_sync_status.last_error` payload we persist. The
/// column itself is `TEXT` (unbounded), but a multi-MB tiberius error
/// would (a) bloat the row, (b) destroy dashboard readability, and
/// (c) potentially echo attacker-controlled bytes from a malformed
/// upstream value. 1 KiB is enough for the event_name prefix + a
/// human-readable summary; the full error stays in the log.
const LAST_ERROR_MAX_LEN: usize = 1024;

/// Defensive sanitization for the `last_error` payload before we write
/// it to PG. Two concerns:
///   1. Length — truncate at `LAST_ERROR_MAX_LEN` chars (NOT bytes; we
///      truncate at a char boundary so a multibyte glyph doesn't get
///      split mid-codepoint and break a downstream JSON consumer).
///   2. Interior `"` / newlines / control chars — replaced with their
///      JSON-style escape so an upstream MSSQL error containing
///      attacker-controlled bytes can't break the dashboard's JSON
///      render. Defensive against the prompt-injection vector flagged
///      in the 2026-05-14 post-mortem.
///
/// Pure function — exposed at module scope so the unit tests below can
/// exercise it without a PG handle.
fn sanitize_last_error(raw: &str) -> String {
    // Step 1: escape interior bytes that would break JSON / log consumers.
    // Mirrors a subset of `serde_json::to_string` for `&str` but stays
    // dependency-free and produces predictable output for the test
    // assertions.
    let mut escaped = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                // Other C0 control chars — render as Unicode escape so
                // the dashboard can't render them as glyphs.
                escaped.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => escaped.push(c),
        }
    }

    // Step 2: truncate at a char boundary so we never split a multibyte
    // glyph in half. `String::truncate` already requires a char boundary,
    // but we compute the boundary ourselves to keep the post-truncate
    // string a clean prefix rather than panic'ing.
    if escaped.chars().count() <= LAST_ERROR_MAX_LEN {
        return escaped;
    }
    let mut out = String::with_capacity(LAST_ERROR_MAX_LEN + 16);
    for (i, ch) in escaped.chars().enumerate() {
        if i >= LAST_ERROR_MAX_LEN - 1 {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}

/// Build the `legacy_sync_status.last_error` payload from an event_name
/// + raw error. Format: `"<event_name>: <sanitized_summary>"`. Operators
/// see the failure MODE first (greppable, stable) followed by the
/// (truncated, JSON-safe) details.
fn format_last_error(event_name: &str, raw: &str) -> String {
    let sanitized = sanitize_last_error(raw);
    let mut out = String::with_capacity(event_name.len() + 2 + sanitized.len());
    out.push_str(event_name);
    out.push_str(": ");
    out.push_str(&sanitized);
    // Re-truncate the combined string in case the event_name prefix
    // pushed us past the cap (it shouldn't — event_names are short —
    // but defensive against a future longer naming scheme).
    sanitize_last_error(&out)
}

const DEFAULT_CT_POLL_INTERVAL_MS: u64 = 1000;

/// How often to verify each table's CT retention window
/// (`MIN_VALID_VERSION(<table>) <= last_seen_version`).
///
/// Retention overflow is a >48h outage scenario; recovery is
/// operator-driven via Slack alert + manual `--bootstrap` reconcile,
/// so 5-min detection vs 30s detection is operationally equivalent.
/// The 10× reduction in pool pressure removes the bb8 timeout noise
/// observed on the hotter mappers (HT_CheckIn_*, HT_Receipt_H) whose
/// parent-aggregate re-loads dominate MSSQL connection demand.
///
/// Override at runtime via `LEGACY_SYNC_RETENTION_CHECK_INTERVAL_SECS`.
const DEFAULT_RETENTION_CHECK_INTERVAL_SECS: u64 = 300;

/// Mid-run pool-outage handling (v2.58.4). HF Ville's WG tunnel flaps
/// for ~2 min every couple of days; when the legacy MSSQL is
/// unreachable, every `mssql.get().await` blocks for the full
/// `POOL_CONNECTION_TIMEOUT` and returns "Timed out in bb8". Without
/// short-circuiting we'd walk the 16-table loop sequentially, each
/// table's own `fetch_ct_rows` paying its own pool-timeout. The
/// up-front `probe_legacy_connectivity` call in `run_one_tick` is the
/// short-circuit: the FIRST pool-timeout abandons the rest of the tick
/// (a WARN, not a page), and the next 1s tick re-probes and resumes the
/// moment the tunnel returns. NOTE (2026-06-30): the former
/// `LEGACY_SYNC_OUTAGE_COOLDOWN_SECS` / `LEGACY_SYNC_OUTAGE_ALERT_THRESHOLD`
/// knobs were removed — their consecutive-tick *paging* path had been
/// refactored out long ago, leaving the env vars parsed-but-dead and
/// falsely implying a paging breaker. A sustained unreachable legacy is
/// now paged solely by the watchdog's probe-outage escalation (see
/// `DEFAULT_PROBE_OUTAGE_ESCALATION_SECS`).

/// MSSQL-pool-init retry knobs (v2.63.0). Same root cause as the
/// mid-run pool-timeout short-circuit above — HF Ville's WG tunnel can be down at
/// container startup, in which case the initial `create_pool` call
/// returns a "Timed out in bb8" / TCP-refused error and the watcher
/// historically exited with code 1. Docker's `restart: on-failure:5`
/// policy then capped retries at 5 attempts before giving up, leaving
/// the watcher dead even after the tunnel recovered.
///
/// Retry on init failure with exponential backoff capped at
/// `INIT_RETRY_MAX_SECS`; never exit. Genuine non-recoverable errors
/// (panics, schema fingerprint mismatch handled separately above)
/// still propagate, so Docker's restart policy stays meaningful for
/// the cases it was designed for.
///
/// Schedule: 5s, 10s, 20s, 40s, 60s, 60s, 60s, ... — total elapsed
/// reaches 5 min at attempt ~7.
///
/// Override at runtime via `LEGACY_SYNC_INIT_RETRY_INITIAL_SECS` /
/// `LEGACY_SYNC_INIT_RETRY_MAX_SECS` /
/// `LEGACY_SYNC_INIT_RETRY_ALERT_AFTER_SECS`.
const DEFAULT_INIT_RETRY_INITIAL_SECS: u64 = 5;
const DEFAULT_INIT_RETRY_MAX_SECS: u64 = 60;
const DEFAULT_INIT_RETRY_ALERT_AFTER_SECS: u64 = 300;

/// Quiet-aware watchdog probe budget. The stall watchdog fires its
/// `CHANGE_TRACKING_CURRENT_VERSION()` probe inside the alert path, so
/// the budget MUST stay well below the 60s poll interval — but it must
/// also be generous enough to ride out an overnight-quiescent legacy
/// (slow WireGuard tunnel re-key, sluggish-but-reachable iHOTEL).
///
/// 2026-06-26: raised 5s → 12s. The old 5s was tighter than the main
/// watcher's own connectivity probe (`SELECT 1` runs on the 10s
/// `MssqlOpKind::Read` budget), so during slow-but-reachable overnight
/// windows the watcher stayed healthy while THIS probe timed out, fired
/// a self-recovering `:information_source:` page, and recovered the next
/// tick once legacy answered.
///
/// 2026-06-29: raised 12s → 30s. Even at 12s the probe kept timing out
/// during deep overnight-quiet windows (info pages with the watermark
/// idle 6171s/8113s on 2026-06-29) — iHOTEL was simply answering
/// `CHANGE_TRACKING_CURRENT_VERSION()` slower than 12s while quiescent,
/// not unreachable. 30s rides out that sluggishness and still leaves
/// ~30s of headroom in the 60s tick. A genuinely unreachable legacy
/// still fails fast (TCP/handshake errors don't wait the full budget)
/// and falls through to the conservative "fire anyway" branch; and a
/// real, sustained probe outage is still caught by the 1h
/// [`DEFAULT_PROBE_OUTAGE_ESCALATION_SECS`] escalation. Override via
/// `LEGACY_SYNC_PROBE_TIMEOUT_MS`.
const DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS: u64 = 30_000;

/// 2026-06-26 — once a probe-timeout (`:information_source:`) outage has
/// been open this long WITHOUT self-recovering, it stops being the benign
/// overnight-quiet pattern and becomes a real "we cannot reach legacy to
/// confirm whether changes are backing up" signal. Escalate it to a
/// `:rotating_light:` page (one-time per outage). This escalation is now
/// the SOLE Slack signal for a sustained probe-unreachable condition
/// (the benign informational note is Slack-suppressed as of 2026-06-30,
/// and the never-wired pool-outage breaker knobs were removed the same
/// day), so it carries the whole "self-healing failed" contract.
/// 2026-06-30 — lowered 1h → 20min (operator request): a transient slow
/// probe self-clears within 1–3 watchdog ticks (≤3min), far below this
/// window, so 20min stays noise-free while surfacing a genuinely stuck/
/// unreachable legacy ~40min sooner. Override via
/// `LEGACY_SYNC_PROBE_OUTAGE_ESCALATION_SECS`.
const DEFAULT_PROBE_OUTAGE_ESCALATION_SECS: u64 = 1200;

/// 2026-06-30 — CT-machinery keep-warm interval. The watcher's per-tick
/// `SELECT 1` connectivity probe already keeps the bb8 pool + WireGuard
/// tunnel warm, but it does NOT exercise Change Tracking. On a quiescent
/// overnight iHOTEL the FIRST `CHANGE_TRACKING_CURRENT_VERSION()` after a
/// lull hits a cold CT path and can answer slower than the 30s watchdog
/// probe budget — the benign "CT watermark idle — probe timed out" pattern
/// (the connection is fine; CT version computation is simply cold). When
/// this is > 0 a sibling task issues a read-only
/// `CHANGE_TRACKING_CURRENT_VERSION()` on this cadence to keep the CT
/// version machinery hot, so the real watchdog probe returns fast — and a
/// GENUINE backlog is classified correctly instead of masked as a timeout.
/// 0 = OFF (default). Opt-in because it adds a trivial read-only query to
/// the SHARED legacy server 24/7; flip it on per-site via env. Recommended
/// value when enabling: 45 (comfortably under the 60s pool idle_timeout and
/// the 60s watchdog interval). Override via `LEGACY_SYNC_CT_KEEPALIVE_SECS`.
const DEFAULT_CT_KEEPALIVE_SECS: u64 = 0;

/// Track D / T7 CRIT-3 — watermark-stall watchdog poll interval (60s).
/// Reads `legacy_ct_state.last_seen_version` + `last_polled_at` once
/// per interval and compares against the previous observation.
const WATERMARK_WATCHDOG_POLL_INTERVAL_SECS: u64 = 60;

/// Track D / T7 CRIT-3 — emit a Slack alert if `last_seen_version`
/// hasn't advanced for this long while shadow mode is OFF AND the
/// watcher claims to be polling (recent `last_polled_at`). Default
/// 30 min. Override via `LEGACY_SYNC_WATERMARK_STALL_ALERT_SECS`.
const DEFAULT_WATERMARK_STALL_ALERT_SECS: u64 = 1800;

/// Track D / T7 CRIT-3 — hard ceiling on how long shadow mode can run
/// before paging the operator. The MSSQL CT retention default is 2
/// days; we ceiling at 36h to leave 12h of cushion. Hardcoded so a
/// well-meaning operator can't push it past the cliff via env var.
const SHADOW_MODE_MAX_DURATION_SECS: u64 = 36 * 3600;

/// Track D / T7 CRIT-3 — minimum gap between watchdog Slack pages.
/// Flooding the channel once we detect a stall isn't actionable — the
/// operator just needs to know it's happening.
const WATCHDOG_ALERT_COOLDOWN_SECS: u64 = 1800;

/// 2026-05-22 — number of CONSECUTIVE probe failures required before
/// the watchdog pages the `:information_source:` (probe-timeout,
/// informational since 2026-06-11) class. A single
/// `CHANGE_TRACKING_CURRENT_VERSION()` timeout inside the 60s tick
/// is dominated by transient iHOTEL lock contention (`TABLOCKX,
/// HOLDLOCK` on `HT_CheckIn_Ds.get_id` cascades and report runs — see
/// `docs/legacy-app/COMPAT_CHEATSHEET.md` §4) and self-clears on the
/// next tick. Requiring 3 in a row means ~3 minutes of sustained
/// uncertainty before a page, while keeping the confirmed-quiet branch at
/// single-tick reaction speed. (The confirmed-backlog `:rotating_light:`
/// branch has its own, separate persistence gate — see
/// [`DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD`].) Override via
/// `LEGACY_SYNC_PROBE_TIMEOUT_STREAK`.
const DEFAULT_PROBE_TIMEOUT_STREAK_THRESHOLD: u32 = 3;

/// 2026-06-29 — number of CONSECUTIVE watchdog ticks a CONFIRMED backlog
/// (probe `Some(v)` with `v > watermark`) must persist before the
/// `:rotating_light:` *CT watermark STUCK* page fires.
///
/// The stuck-eligibility gate ([`watermark_stall_alert_eligible`]) only
/// measures how long the watermark has been *frozen* — which, during a
/// genuine quiet period, is just how long it's been idle at tip-of-stream
/// (correctly suppressed by [`should_fire_stall_alert`] while the probe
/// reads `== watermark`). The instant a SINGLE new change lands on legacy,
/// the very next 60s watchdog tick can observe `ct_current > watermark`
/// *before* the CT watcher (which polls on its own cadence) has consumed
/// it. Pre-2026-06-29 that paged a critical "STUCK" on the FIRST
/// observation and inherited the whole idle duration as the "stuck"
/// figure — producing absurd self-recovering pages like
/// "stuck 7807s, 1 version unprocessed" that cleared the next tick.
///
/// A backlog the watcher drains within its own poll cycle is normal lag,
/// not a stall. Requiring it to survive >= 2 consecutive 60s ticks (~60s
/// of genuinely-unconsumed changes) suppresses that race while still
/// paging a truly wedged watcher within ~60s. The monotonicity-violation
/// case (`v < watermark`) is NOT gated — it's a corruption signal and
/// fires on first observation. Override via
/// `LEGACY_SYNC_BACKLOG_PERSIST_STREAK` (1 reproduces the old hair-trigger).
const DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD: u32 = 2;

/// All CT-enabled MSSQL tables — must stay in sync with the seeds in
/// migrations 017 (canonical sync, 10 tables) + 022 (legacy_mirror, 6
/// tables) + 033 (Track E1, 2 tables) + 056 (HT_Book_Pro) and the
/// `legacy_sync_status` rows. Adding a new mapper means inserting a
/// row in the relevant seed migration (BOTH `legacy_sync_status` and a
/// `legacy_ct_state_per_table` row seeded from the current global
/// watermark — see migration 056 for the canonical example), adding
/// the table here, and wiring its mapper in `build_mappers`.
const CT_ENABLED_TABLES: &[&str] = &[
    // Phase 5 — canonical sync (10 tables, CT enabled 2026-04-25)
    "HT_Customers",
    "HT_Rooms",
    "HT_Room_Status",
    "HT_Book_H",
    "HT_Book_Ds",
    "HT_Book_Date",
    "HT_CheckIn_H",
    "HT_CheckIn_Ds",
    "HT_CheckIn_Pay",
    "HT_Receipt_H",
    // Phase 5.5b — legacy_mirror.* opaque pass-through (6 tables, CT
    // enabled 2026-04-29 on HF Hotel; Ville's CT enabled by migrations
    // 020 + 021 after the 2026-04-29 SS2025 upgrade). The mirror mappers
    // run for both sites — `SiteConfig::from_env()` selects the pool +
    // legacy connection at startup.
    "HT_Cupon",
    "HT_CheckIn_Product",
    "HT_Deposit",
    "HT_Changed_Room",
    "HT_Bill_Debt_H",
    "HT_Bill_Debt_Ds",
    // Phase 5/E1 — Track E1 (audit 2026-05-13) sync-gap closure.
    // `HT_CheckIn_Other_People` (T2 HIGH-3): newly CT-enabled by
    // legacy-mssql migration 022, mapped to canonical `ht_guest_registry`
    // for TM.30 immigration compliance.
    // `HT_Rooms_Cancel` (T2 HIGH-5): CT enabled back in Phase 5 (migration
    // 020) but had no mapper — dangling subscription closed by a new
    // mirror mapper.
    "HT_CheckIn_Other_People",
    "HT_Rooms_Cancel",
    // Phase 5/E2 — coexistence audit 2026-06-11 P2 gap closure.
    // `HT_Book_Pro` (pre-booked products attached to a booking by
    // FrmAddBook2): CT enabled by legacy-mssql migration 023, mirrored
    // into `legacy_mirror.ht_book_pro` (migration pg/056) by the new
    // `BookProMirrorMapper` so iHOTEL-entered booking products are
    // visible to the new app.
    "HT_Book_Pro",
];

/// What the `sync` binary was asked to do, parsed from argv.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliMode {
    /// `--print-ct-tables`: dump the CT-enabled table list and exit (deploy gate).
    PrintCtTables,
    /// `--bootstrap [--dry-run]`: one-shot cold-seed. `dry_run` previews
    /// (read-only) instead of writing.
    Bootstrap { dry_run: bool },
    /// No flags: run the steady-state CT watcher.
    Watcher,
}

/// Parse the `sync` CLI mode from argv (pure; argv[0] is skipped).
///
/// Rejects any unrecognized argument with an error rather than ignoring it.
/// This is deliberate: before this guard, `sync --bootstrap --dry-run` (or any
/// typo'd flag) was SILENTLY IGNORED, so an operator expecting a no-op preview
/// would instead run a full destructive bootstrap (DELETE+reinsert of every
/// mirror table + a watermark stamp). `--dry-run` only means something for
/// `--bootstrap` (the watcher and `--print-ct-tables` never write), so it is an
/// error to pass it alone. `--print-ct-tables` is exclusive (it is the
/// dependency-free deploy probe).
fn parse_cli_mode<I: IntoIterator<Item = String>>(args: I) -> Result<CliMode, String> {
    let (mut bootstrap, mut dry_run, mut print_ct) = (false, false, false);
    for a in args.into_iter().skip(1) {
        match a.as_str() {
            "--bootstrap" => bootstrap = true,
            "--dry-run" => dry_run = true,
            "--print-ct-tables" => print_ct = true,
            other => {
                return Err(format!(
                    "unrecognized argument `{other}`. Supported: \
                     `--bootstrap [--dry-run]`, `--print-ct-tables`, or no args (watcher)"
                ));
            }
        }
    }
    if print_ct {
        if bootstrap || dry_run {
            return Err(
                "--print-ct-tables is exclusive and cannot be combined with other flags".into(),
            );
        }
        return Ok(CliMode::PrintCtTables);
    }
    if dry_run && !bootstrap {
        return Err(
            "--dry-run only applies to --bootstrap (the watcher and --print-ct-tables never \
             write). Did you mean `--bootstrap --dry-run`?"
                .into(),
        );
    }
    if bootstrap {
        return Ok(CliMode::Bootstrap { dry_run });
    }
    Ok(CliMode::Watcher)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    hotel_backend::secrets::hydrate_env_from_secret_files();
    dotenvy::dotenv().ok();

    // Parse argv up front so an unrecognized flag fails LOUD (see
    // parse_cli_mode) instead of silently falling through to a write.
    let mode = match parse_cli_mode(env::args().collect::<Vec<_>>()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[sync] argument error: {e}");
            return Err(e.into());
        }
    };

    // Single source of truth for the deploy-time CT gate
    // (scripts/deploy/run-deploy.sh): emit the tables this binary expects
    // Change Tracking on, one per line, then exit. Kept dependency-free (no
    // DB connection, no SITE_ID parse) so the deploy can run it straight from
    // the backend image to learn EXACTLY what this build requires — no
    // Rust-vs-shell list drift. Added after the 2026-06-24 HT_Book_Pro
    // incident (binary shipped ahead of its CT-enable migration).
    if mode == CliMode::PrintCtTables {
        for t in CT_ENABLED_TABLES {
            println!("{t}");
        }
        return Ok(());
    }

    // Security audit 2026-05-14: hydrate sensitive env vars (DB_PASSWORD,
    // POSTGRES_PASSWORD, SLACK_WEBHOOK_URL) from Docker secret files at
    // `/run/secrets/<name>` when present. Also reconstructs DATABASE_URL
    // from POSTGRES_USER + secret-file POSTGRES_PASSWORD + NEW_DB_* parts
    // if it isn't pre-baked. See `hotel_backend::secrets` for details.
    let hydrated = hotel_backend::secrets::hydrate_env_from_secret_files();
    if hydrated > 0 {
        eprintln!(
            "[secrets] sync: hydrated {hydrated} env var(s) from secret files at /run/secrets/"
        );
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hotel_backend=info,sync=info".into()),
        )
        .init();

    // Task #69: parse SITE_ID once at startup. Panics on a typo so a
    // misconfigured deploy fails loud before the watcher starts pulling
    // CT rows.
    let site = SiteConfig::from_env();
    tracing::info!(site = %site.id, "CT watcher: site identity resolved");

    let (bootstrap_requested, bootstrap_dry_run) = match mode {
        CliMode::Bootstrap { dry_run } => (true, dry_run),
        _ => (false, false),
    };

    let enabled = env::var("LEGACY_SYNC_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false);

    // `--bootstrap` is an explicit one-shot operator action: cold-seed
    // canonical PG state from MSSQL via the legacy reconcile path, then
    // record the current `CHANGE_TRACKING_CURRENT_VERSION()` as the
    // watermark so the next run can resume from sub-second tip-of-stream.
    // It runs INDEPENDENTLY of LEGACY_SYNC_ENABLED — operators bootstrap
    // first, then flip the flag (per docs/runbook-sync.md cutover sequence).
    //
    // Audit finding N1 (Phase 5.5 codebase audit, 2026-04-29) — refuse
    // a live bootstrap unless the operator opts in. When
    // `LEGACY_SYNC_ENABLED=true`, the watcher is (or will shortly be)
    // pulling CT rows and UPSERTing into `legacy_mirror.<table>` with
    // `mirror_source='ct'`. The bootstrap snapshot path runs
    // `DELETE FROM legacy_mirror.<table>` then re-inserts every row
    // with `mirror_source='reconcile'` — that DELETE races the
    // watcher's UPSERTs and can clobber `mirror_source='ct'` rows the
    // mapper just wrote (and the snapshot doesn't carry, e.g. legacy
    // INSERTs newer than the snapshot SELECT). To proceed against a
    // live deployment, set `LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP=true`
    // (matches the cold-replay / overflow override pattern below).
    if bootstrap_requested {
        // A dry run writes NOTHING (read-only preview), so it is always safe
        // against a live watcher — skip the live-bootstrap refusal entirely.
        if enabled && !bootstrap_dry_run {
            let allow_live_bootstrap = env::var("LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP")
                .map(|v| v == "true")
                .unwrap_or(false);
            if !allow_live_bootstrap {
                let slack_config = SlackConfig::from_env();
                let slack: Option<SlackClient> = if slack_config.is_configured() {
                    Some(SlackClient::new(slack_config))
                } else {
                    None
                };
                let msg = build_live_bootstrap_refusal_message();
                tracing::error!(site = %site.id, "{msg}");
                if let Some(s) = &slack {
                    let payload = SlackMessage::with_site_text(
                        &site.id,
                        format!(
                            ":no_entry: *Bootstrap REFUSED — live deployment* :no_entry:\n{msg}"
                        ),
                    );
                    let _ = s.send_message(&payload).await;
                }
                // Sleep before exit so Docker `restart: unless-stopped`
                // doesn't turn this into a tight loop + alert flood.
                tracing::warn!(
                    site = %site.id,
                    "Sleeping 60s before exit to throttle Docker restart cadence"
                );
                tokio::time::sleep(Duration::from_secs(60)).await;
                return Err(msg.into());
            }
            tracing::warn!(
                site = %site.id,
                "LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP=true — proceeding with bootstrap \
                 against a live deployment. Watcher CT writes during the snapshot \
                 window may be clobbered by the snapshot DELETE."
            );
        }
        return run_bootstrap(&site, bootstrap_dry_run).await;
    }

    if !enabled {
        tracing::info!(
            "LEGACY_SYNC_ENABLED!=true — CT watcher exiting cleanly without polling"
        );
        return Ok(());
    }

    let poll_interval_ms = env::var("CT_POLL_INTERVAL_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CT_POLL_INTERVAL_MS);

    let retention_check_interval = Duration::from_secs(
        env::var("LEGACY_SYNC_RETENTION_CHECK_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_RETENTION_CHECK_INTERVAL_SECS),
    );

    let shadow_mode = env::var("LEGACY_SYNC_SHADOW_MODE")
        .map(|v| v == "true")
        .unwrap_or(false);

    // Resilience PR R3 (2026-05-14) — per-table CT watermark feature
    // flag. OFF by default to keep the current global-watermark path
    // operational; flip to `true` per-site after migration 050 has
    // landed to decouple per-table progress. See module docs in
    // `crate::sync::watermark` for the dual-mode contract.
    let per_table_watermark = env::var("SYNC_PER_TABLE_WATERMARK")
        .map(|v| v == "true")
        .unwrap_or(false);

    let allowlist = parse_allowlist(env::var("LEGACY_SYNC_TABLE_ALLOWLIST").ok());

    tracing::info!(
        poll_interval_ms,
        retention_check_interval_secs = retention_check_interval.as_secs(),
        shadow_mode,
        per_table_watermark,
        allowlist = ?allowlist,
        "Starting CT watcher"
    );

    let pg_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pg = PgPoolOptions::new()
        .max_connections(8)
        .connect(&pg_url)
        .await?;
    tracing::info!("Connected to PostgreSQL");

    let mssql_config = DbConfig::from_env();

    // Slack client must be initialised BEFORE the MSSQL pool retry loop
    // so the loop can fire an alert when total elapsed crosses the
    // `INIT_RETRY_ALERT_AFTER_SECS` threshold (v2.63.0). Was previously
    // initialised AFTER `create_pool` because the only failure mode
    // there was exit-1; now that we retry forever instead of exiting,
    // the alert plumbing has to be live during retry.
    let slack_config = SlackConfig::from_env();
    let slack: Option<SlackClient> = if slack_config.is_configured() {
        tracing::info!("Slack notifications enabled for CT watcher");
        Some(SlackClient::new(slack_config))
    } else {
        tracing::warn!(
            "Slack notifications NOT configured (set SLACK_WEBHOOK_URL); \
             schema drift / retention overflow will only surface in logs"
        );
        None
    };

    let init_retry_config = InitRetryConfig::from_env();
    let mssql = create_pool_with_retry(
        &mssql_config,
        &init_retry_config,
        slack.as_ref(),
        &site.id,
        "ct-watcher",
    )
    .await;
    tracing::info!(server = %mssql_config.server, "Connected to legacy MSSQL");

    // Track D / T7 HIGH-3 — verify the CT-side fingerprint (writeback
    // baseline + 5 CT-only extras) instead of just the writeback set.
    // A vendor change on HT_CheckIn_Product / HT_Deposit / HT_Bill_Debt_*
    // / HT_CheckIn_Other_People would silently corrupt CT-mapped rows
    // without the CT-extra guard.
    if let Err(e) = verify_ct_schema_fingerprint(&mssql).await {
        tracing::error!(
            site = %site.id,
            error = %e,
            "Schema fingerprint check failed — refusing to start"
        );
        if let Some(slack) = &slack {
            let msg = SlackMessage::with_site_text(
                &site.id,
                format!(
                    ":warning: *CT watcher REFUSED TO START* :warning:\n\
                     Legacy MSSQL schema fingerprint mismatch.\n\
                     *Error:* `{e}`\n\
                     _The legacy DB columns drifted from the captured baseline. \
                     Run_ `./scripts/writeback-fingerprint.sh` _and follow the \
                     README to update the baseline before restarting._"
                ),
            );
            let _ = slack.send_message(&msg).await;
        }
        tracing::warn!(site = %site.id, "Sleeping 60s before exit to throttle Docker restart cadence");
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(format!("Schema fingerprint check failed: {e}").into());
    }

    // Cold-replay refusal (Phase 5.5). If the watermark is still at the
    // seed value (`last_seen_version = 0`) AND the operator hasn't
    // explicitly opted into a cold replay via env var, refuse to start
    // and point at `--bootstrap`. Without this guard, a fresh deploy
    // would attempt to process every CT row from time-zero, which
    // either fails immediately on retention overflow (long-lived
    // databases) or floods downstream subscribers with months of
    // historical events.
    let cold_replay_allowed = env::var("LEGACY_SYNC_ALLOW_COLD_REPLAY")
        .map(|v| v == "true")
        .unwrap_or(false);
    let current_watermark = hotel_backend::sync::watermark::read_last_seen(&pg)
        .await
        .map_err(|e| format!("Failed to read CT watermark: {e}"))?;
    if current_watermark == 0 && !cold_replay_allowed {
        let msg = "CT watermark is 0 (cold start) and \
                   LEGACY_SYNC_ALLOW_COLD_REPLAY != true — refusing to start. \
                   Run `bin/sync --bootstrap` first to seed canonical state \
                   and the watermark, OR set LEGACY_SYNC_ALLOW_COLD_REPLAY=true \
                   to override (will replay all CT history). \
                   See docs/runbook-sync.md for the full cutover procedure.";
        tracing::error!(site = %site.id, "{msg}");
        if let Some(s) = &slack {
            let payload = SlackMessage::with_site_text(
                &site.id,
                format!(":no_entry: *CT watcher REFUSED TO START* :no_entry:\n{msg}"),
            );
            let _ = s.send_message(&payload).await;
        }
        // Sleep before exit so Docker `restart: unless-stopped` doesn't
        // turn this into a tight loop + alert flood.
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(msg.into());
    }

    // Retention-overflow refusal. If the watermark is older than the CT
    // MIN_VALID_VERSION on ANY tracked table, the row history we'd need
    // to catch up has aged out — incremental replay would silently miss
    // changes since the watermark. Canonical scenario is the
    // shadow-mode 2-day trap (watermark frozen by TX rollback while
    // MIN_VALID_VERSION marches forward), but a long worker outage
    // hits the same wall. Force the operator to --bootstrap.
    let allow_overflow = env::var("LEGACY_SYNC_ALLOW_OVERFLOW")
        .map(|v| v == "true")
        .unwrap_or(false);
    let allowed_tables: Vec<&'static str> = CT_ENABLED_TABLES
        .iter()
        .filter(|t| {
            allowlist
                .as_ref()
                .map(|a| a.contains(**t))
                .unwrap_or(true)
        })
        .copied()
        .collect();
    let mut overflowed: Vec<String> = Vec::new();
    for table in &allowed_tables {
        match check_retention(&mssql, table, current_watermark).await {
            Ok(()) => {}
            Err(err) if err.contains("retention overflow") => {
                overflowed.push(format!("{table}: {err}"));
            }
            Err(err) => {
                tracing::warn!(
                    table,
                    error = %err,
                    "Pre-flight retention probe failed; treating as transient"
                );
            }
        }
    }
    if !overflowed.is_empty() && !allow_overflow {
        let msg = format!(
            "CT retention overflow on {} table(s) — refusing to start.\n  \
             Affected:\n    - {}\n  \
             The CT row history we need has aged out. Common cause: \
             shadow-mode soak >= MSSQL CT retention (default 2 days) — \
             watermark gets rolled back every tick while \
             MIN_VALID_VERSION marches forward. Long worker outage hits \
             the same wall.\n  \
             Recover with `bin/sync --bootstrap` to re-snapshot \
             canonical PG and reset the watermark to the current SQL \
             Server CT version. After bootstrap, restart this binary.\n  \
             Set LEGACY_SYNC_ALLOW_OVERFLOW=true ONLY if you accept \
             that incremental rows since the watermark are silently \
             skipped (data loss). See docs/runbook-sync.md.",
            overflowed.len(),
            overflowed.join("\n    - "),
        );
        tracing::error!(site = %site.id, "{msg}");
        if let Some(s) = &slack {
            let payload = SlackMessage::with_site_text(
                &site.id,
                format!(
                    ":no_entry: *CT watcher REFUSED TO START — retention overflow* :no_entry:\n{msg}"
                ),
            );
            let _ = s.send_message(&payload).await;
        }
        // Sleep before exit so Docker `restart: unless-stopped` doesn't
        // turn this into a tight loop + alert flood.
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(msg.into());
    }

    // CT-enablement gate (incident 2026-06-24). A table in CT_ENABLED_TABLES
    // whose legacy CT subscription is missing makes the per-tick CHANGETABLE
    // query error ~1/sec forever — isolated (other tables keep syncing) but
    // noisy, and that table never syncs. `check_retention` can't catch it:
    // CHANGE_TRACKING_MIN_VALID_VERSION returns NULL when CT is off and the
    // function treats NULL as healthy. So probe enablement explicitly and
    // refuse to start, naming the tables — almost always a
    // migrations/legacy-mssql/ prerequisite that wasn't applied before this
    // binary shipped. The deploy now runs scripts/migrate-legacy-mssql.sh
    // before starting workers, so in the normal flow CT is already enabled by
    // the time we get here; this is the backstop for runtime CT loss
    // (server failover, manual DISABLE). A failed probe (connectivity) is
    // transient — only an explicit Ok(false) refuses.
    let allow_ct_gap = env::var("LEGACY_SYNC_ALLOW_CT_GAP")
        .map(|v| v == "true")
        .unwrap_or(false);
    let mut ct_probes: Vec<(&'static str, Option<bool>)> = Vec::new();
    for table in &allowed_tables {
        match check_ct_enabled(&mssql, table).await {
            Ok(on) => ct_probes.push((*table, Some(on))),
            Err(err) => {
                tracing::warn!(
                    table,
                    error = %err,
                    "Pre-flight CT-enablement probe failed; treating as transient"
                );
                ct_probes.push((*table, None));
            }
        }
    }
    let ct_missing: Vec<&'static str> = ct_tables_definitely_missing(&ct_probes);
    if !ct_missing.is_empty() && !allow_ct_gap {
        let msg = format!(
            "Change Tracking NOT enabled on {} expected table(s) — refusing to start.\n  \
             Affected:\n    - {}\n  \
             The watcher would error ~1/sec on each and never sync them. Almost \
             always a migrations/legacy-mssql/ prerequisite that did not get \
             applied before this binary shipped. The deploy runs \
             scripts/migrate-legacy-mssql.sh automatically — check its output for \
             a failed/ skipped migration, apply the matching CT-enable DDL, then \
             restart this binary. Set LEGACY_SYNC_ALLOW_CT_GAP=true ONLY to run \
             with those tables intentionally unsynced (the per-tick errors will \
             resume). See docs/runbook-sync.md.",
            ct_missing.len(),
            ct_missing.join("\n    - "),
        );
        tracing::error!(site = %site.id, "{msg}");
        if let Some(s) = &slack {
            let payload = SlackMessage::with_site_text(
                &site.id,
                format!(
                    ":no_entry: *CT watcher REFUSED TO START — Change Tracking not enabled* :no_entry:\n{msg}"
                ),
            );
            let _ = s.send_message(&payload).await;
        }
        // Sleep before exit so Docker `restart: unless-stopped` doesn't
        // turn this into a tight loop + alert flood.
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(msg.into());
    }

    let mappers = build_mappers(&allowlist);
    tracing::info!(
        count = mappers.len(),
        watermark = current_watermark,
        "Mappers initialised (10 canonical sync + 6 legacy_mirror = 16 \
         CT-enabled tables; every table has a real mapper)"
    );

    let shutdown = Arc::new(Notify::new());
    let shutdown_clone = shutdown.clone();
    tokio::spawn(async move {
        let signal_kind = tokio::signal::unix::SignalKind::terminate();
        let mut sigterm = match tokio::signal::unix::signal(signal_kind) {
            Ok(s) => s,
            Err(err) => {
                tracing::warn!(error = %err, "Could not register SIGTERM handler");
                return;
            }
        };
        sigterm.recv().await;
        tracing::info!("SIGTERM received — finishing current tick then exiting");
        shutdown_clone.notify_waiters();
    });

    // Track D / T7 CRIT-3 — watermark-stall watchdog. Background task
    // that polls `legacy_ct_state` every 60s and pages on:
    // (a) live-mode watermark not advancing for `stall_alert_secs`
    //     (default 30 min, configurable via env), or
    // (b) shadow mode running past the 36h hardcoded ceiling (below
    //     the 48h MSSQL CT retention cliff).
    // Cooldown 30min per alert kind so we don't flood.
    let stall_alert_secs = env::var("LEGACY_SYNC_WATERMARK_STALL_ALERT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_WATERMARK_STALL_ALERT_SECS);
    let watchdog_pg = pg.clone();
    let watchdog_mssql = mssql.clone();
    let watchdog_slack = slack.clone();
    let watchdog_site_id = site.id.clone();
    let watchdog_shutdown = shutdown.clone();
    tokio::spawn(async move {
        run_watermark_watchdog(
            watchdog_pg,
            watchdog_mssql,
            watchdog_slack,
            watchdog_site_id,
            stall_alert_secs,
            watchdog_shutdown,
        )
        .await;
    });

    // 2026-06-30 — CT keep-warm task. See `DEFAULT_CT_KEEPALIVE_SECS` for
    // the full rationale. Opt-in (default OFF): when
    // `LEGACY_SYNC_CT_KEEPALIVE_SECS` > 0, periodically run a read-only
    // `CHANGE_TRACKING_CURRENT_VERSION()` so the CT version machinery stays
    // hot on a quiescent legacy and the watchdog's stall probe stops timing
    // out during overnight quiet windows. Read-only; no writeback. Logs at
    // debug so a healthy ping is silent at the default info level.
    let ct_keepalive_secs = env::var("LEGACY_SYNC_CT_KEEPALIVE_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(DEFAULT_CT_KEEPALIVE_SECS);
    if ct_keepalive_secs > 0 {
        let keepalive_mssql = mssql.clone();
        let keepalive_site_id = site.id.clone();
        let keepalive_shutdown = shutdown.clone();
        // Reuse the watchdog's probe budget so keep-warm and the real probe
        // agree on what "too slow" means.
        let keepalive_timeout = Duration::from_millis(
            env::var("LEGACY_SYNC_PROBE_TIMEOUT_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS),
        );
        tracing::info!(
            site = %site.id,
            interval_secs = ct_keepalive_secs,
            timeout_ms = keepalive_timeout.as_millis() as u64,
            "[Sync] CT keep-warm ENABLED — periodic CHANGE_TRACKING_CURRENT_VERSION() keeps the CT machinery hot"
        );
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(ct_keepalive_secs));
            // Consume the immediate first tick — startup already touched legacy.
            ticker.tick().await;
            let notified = keepalive_shutdown.notified();
            tokio::pin!(notified);
            loop {
                tokio::select! {
                    _ = &mut notified => {
                        tracing::info!(
                            site = %keepalive_site_id,
                            "[Sync] CT keep-warm task exiting (SIGTERM)"
                        );
                        break;
                    }
                    _ = ticker.tick() => {
                        match probe_change_tracking_current_version(&keepalive_mssql, keepalive_timeout).await {
                            Ok(v) => tracing::debug!(
                                site = %keepalive_site_id,
                                ct_current = v,
                                "[Sync] CT keep-warm ping ok"
                            ),
                            Err(err) => tracing::debug!(
                                site = %keepalive_site_id,
                                error = %err,
                                "[Sync] CT keep-warm ping failed (benign — warmer next tick)"
                            ),
                        }
                    }
                }
            }
        });
    }

    // P1 (task #67) — per-site reconcile safety net. The backend's
    // `init_scheduler` runs the 15-min diff-only reconcile ONLY for the
    // primary site (HF Hotel, against `hotelnew`); HF Ville had NO reconcile
    // backstop at all, so a dropped CT event there went undetected for weeks
    // (the 2026-06-28 room-114 / cin 19906 incident). This worker already
    // holds ITS OWN site's MSSQL + canonical PG pools, so it is the natural
    // home for that site's reconcile. Gated by `WORKER_RECONCILE_ENABLED`
    // (default off) and set true ONLY on the sync-hfville container, so HF
    // Hotel keeps its single backend-side reconcile — no double-run / double
    // drift-alerting. Runs in whatever `LEGACY_SYNC_RECONCILE_MODE` resolves
    // to (default `diff_only`: logs to `ht_reconcile_log`, never mutates —
    // the CT watcher above owns canonical writes).
    if env::var("WORKER_RECONCILE_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
    {
        let interval_secs = env::var("WORKER_RECONCILE_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|s| *s >= 60)
            .unwrap_or(900);
        let reconcile_pg = pg.clone();
        let reconcile_mssql = mssql.clone();
        let reconcile_slack = slack.clone();
        let reconcile_site_id = site.id.clone();
        let reconcile_shutdown = shutdown.clone();
        tracing::info!(
            site = %site.id,
            interval_secs,
            "[Sync] Worker reconcile ENABLED — per-site diff-only safety net"
        );
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
            // Consume the immediate first tick: startup already converged
            // canonical, and we don't want a full sweep racing the CT watcher
            // before it settles (or hammering MSSQL on every worker restart).
            ticker.tick().await;
            let notified = reconcile_shutdown.notified();
            tokio::pin!(notified);
            loop {
                tokio::select! {
                    _ = &mut notified => {
                        tracing::info!(
                            site = %reconcile_site_id,
                            "[Sync] Worker reconcile task exiting (SIGTERM)"
                        );
                        break;
                    }
                    _ = ticker.tick() => {
                        let span = tracing::info_span!(
                            "worker_reconcile_tick", site = %reconcile_site_id
                        );
                        let _enter = span.enter();
                        hotel_backend::scheduler::sync::run_sync(
                            &reconcile_mssql,
                            &reconcile_pg,
                            reconcile_slack.as_ref(),
                            &reconcile_site_id,
                        )
                        .await;
                    }
                }
            }
        });
    } else {
        tracing::debug!(
            site = %site.id,
            "[Sync] Worker reconcile disabled (set WORKER_RECONCILE_ENABLED=true to enable)"
        );
    }

    // Per-table retention check timestamps. The first tick after
    // startup runs the check unconditionally (no prior `Instant`); after
    // that, each table re-checks at most once per
    // `retention_check_interval`. Holding this map in the main loop
    // keeps state local to the watcher process — no PG round-trip
    // needed and the map dies cleanly with the worker on SIGTERM.
    let mut retention_last_checked: HashMap<String, Instant> = HashMap::new();

    // Task #69: wrap the main loop in a tracing span so every log line
    // emitted from inside the watcher (mapper warnings, watermark
    // advances, retention probes) carries `site=<id>`. With both HF
    // Hotel and HF Ville sending logs to the same sink, this is the
    // only thing that lets an operator filter by site.
    let watcher_span = tracing::info_span!("ct_watcher", site = %site.id);
    let _watcher_guard = watcher_span.enter();

    loop {
        run_one_tick(
            &pg,
            &mssql,
            &mappers,
            &slack,
            shadow_mode,
            per_table_watermark,
            &mut retention_last_checked,
            retention_check_interval,
            &site.id,
        )
        .await;

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(poll_interval_ms)) => {
                tracing::trace!("CT poll-interval tick");
            }
            _ = shutdown.notified() => {
                tracing::info!("Shutdown signaled — exiting main loop");
                break;
            }
        }
    }

    tracing::info!("CT watcher exited cleanly");
    Ok(())
}

/// One-shot operator action: cold-seed canonical PG state from MSSQL
/// (legacy reconcile) and record the current
/// `CHANGE_TRACKING_CURRENT_VERSION()` as the CT watermark.
///
/// Per docs/architecture.md §3.6d and docs/runbook-sync.md, this is
/// the prerequisite for the watcher's cutover step. After bootstrap
/// completes, the operator flips `LEGACY_SYNC_ENABLED=true` and the
/// watcher resumes from sub-second tip-of-stream — no cold-replay
/// catch-up, no retention-overflow risk.
///
/// Bootstrap is intentionally NOT idempotent in shape (the reconcile
/// it invokes IS idempotent — UPSERT-by-hash). Re-running just re-runs
/// the reconcile and re-stamps the watermark to the new tip.
async fn run_bootstrap(
    site: &SiteConfig,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!(
        site = %site.id,
        dry_run,
        "Phase 5.5 bootstrap — cold-seeding canonical PG + CT watermark"
    );

    let pg_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pg = PgPoolOptions::new()
        .max_connections(4)
        .connect(&pg_url)
        .await?;
    tracing::info!("[bootstrap] Connected to PostgreSQL");

    let mssql_config = DbConfig::from_env();

    // Bootstrap retries on initial MSSQL pool failure too (v2.63.0).
    // Bootstrap is an operator action, so a transient WG flap shouldn't
    // force the operator to babysit the command — the retry loop will
    // hold the line until the tunnel comes back. The bootstrap path
    // initialises its own Slack client (slack_config_for_bootstrap)
    // because the rest of bootstrap intentionally passes `None` into
    // `run_sync` to suppress drift alerts; the retry alert is a
    // different concern (operator paging) and belongs ON.
    let slack_config_for_bootstrap = SlackConfig::from_env();
    let slack_for_bootstrap: Option<SlackClient> = if slack_config_for_bootstrap.is_configured() {
        Some(SlackClient::new(slack_config_for_bootstrap))
    } else {
        None
    };
    let init_retry_config = InitRetryConfig::from_env();
    let mssql = create_pool_with_retry(
        &mssql_config,
        &init_retry_config,
        slack_for_bootstrap.as_ref(),
        &site.id,
        "bootstrap",
    )
    .await;
    tracing::info!(server = %mssql_config.server, "[bootstrap] Connected to legacy MSSQL");

    // Schema fingerprint guard — same gate the watcher main loop uses.
    // Refusing to bootstrap on drift prevents seeding canonical state
    // from a DB shape we don't understand. Track D / T7 HIGH-3 uses
    // the CT-side fingerprint so a drift on a CT-only mirror table
    // blocks the cold-seed reconcile too.
    if let Err(e) = verify_ct_schema_fingerprint(&mssql).await {
        return Err(format!(
            "[bootstrap] Schema fingerprint check failed; refusing to bootstrap: {e}"
        )
        .into());
    }

    // Audit finding N2 (Phase 5.5 codebase audit, 2026-04-29) — capture
    // CHANGE_TRACKING_CURRENT_VERSION() BEFORE the reconcile + snapshot
    // and use that captured value as the watermark stamped at the end.
    //
    // Why: `CHANGETABLE(CHANGES <table>, @version)` returns rows
    // strictly greater than `@version`. If we read the version AFTER
    // the snapshots, any CT row produced between snapshot SELECT and
    // version read is silently skipped on the next watcher tick. For
    // canonical tables this self-heals on the next update (idempotent
    // UPSERT-by-hash), but for the legacy_mirror tables an INSERT in
    // that window with no follow-up update would never land until
    // somebody touches the row again — silent data loss.
    //
    // Capturing the version at the START guarantees the next watcher
    // tick replays everything from snapshot-time-onward. The overlap
    // (rows we both snapshotted AND will replay via CT) is harmless:
    // the CT mappers UPSERT idempotently and `mirror_source` is the
    // only column that changes (snapshot writes 'reconcile', CT writes
    // 'ct' — the latest write wins, which is what we want anyway).
    let snapshot_version = read_change_tracking_current_version(&mssql).await?;
    tracing::info!(
        snapshot_version,
        "[bootstrap] Captured CHANGE_TRACKING_CURRENT_VERSION() before snapshot"
    );

    // `--dry-run`: everything above is read-only (connect, fingerprint, version
    // capture). STOP HERE and report what the real run WOULD write — never
    // touch PG or MSSQL. This is the safe preview an operator reaches for; it
    // works against a live watcher (no writes to race) and is the reason
    // `--dry-run` bypasses the live-bootstrap refusal in `main`.
    if dry_run {
        tracing::info!(
            "[bootstrap:dry-run] read-only preview — NO writes will be performed"
        );

        // The global watermark the live bootstrap would OVERWRITE (unguarded)
        // with `snapshot_version`. Surfacing both makes the rewind/advance
        // explicit before the operator commits to a real run.
        let current_watermark: i64 =
            sqlx::query_scalar("SELECT last_seen_version FROM legacy_ct_state WHERE id = 1")
                .fetch_one(&pg)
                .await
                .unwrap_or(-1);
        tracing::info!(
            current_watermark,
            would_stamp_watermark = snapshot_version,
            "[bootstrap:dry-run] watermark: a real bootstrap would overwrite the global watermark"
        );

        // Per transactional mirror table: legacy source count vs current PG
        // mirror count. The real snapshot would DELETE the PG side and
        // re-INSERT every source row as mirror_source='reconcile'.
        for (mssql_table, pg_table) in hotel_backend::scheduler::mirror::MIRROR_TRANSACTIONAL_TABLES
        {
            let legacy_source_rows: i64 = {
                let mut conn = mssql.get().await?;
                let rows = conn
                    .simple_query(format!("SELECT COUNT_BIG(*) FROM {mssql_table}"))
                    .await?
                    .into_first_result()
                    .await?;
                rows.first().and_then(|r| r.get::<i64, _>(0)).unwrap_or(-1)
            };
            // `pg_table` is a compile-time constant from
            // MIRROR_TRANSACTIONAL_TABLES (never user input); AssertSqlSafe
            // documents the audited interpolation sqlx 0.9 otherwise rejects.
            let pg_mirror_rows: i64 = sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
                "SELECT count(*) FROM {pg_table}"
            )))
            .fetch_one(&pg)
            .await
            .unwrap_or(-1);
            tracing::info!(
                table = %mssql_table,
                legacy_source_rows,
                pg_mirror_rows,
                delta = legacy_source_rows - pg_mirror_rows,
                "[bootstrap:dry-run] snapshot would replace the PG mirror with the legacy source"
            );
        }

        tracing::info!(
            "[bootstrap:dry-run] preview complete. The canonical reconcile \
             (run_sync, UPSERT-by-hash) was NOT executed — it converges canonical \
             tables idempotently and is not enumerated here. No writes performed."
        );
        return Ok(());
    }

    // Phase 1: run the existing reconcile path ONCE to bring canonical
    // PG state up to date with MSSQL. The legacy `scheduler::sync::run_sync`
    // (5-min full-sync) already does this work via UPSERT-by-hash. After
    // Phase 5.5 the steady-state cron version of `run_sync` is demoted
    // to diff-only, but the bootstrap path uses it in upsert mode here
    // by temporarily overriding the env var.
    tracing::info!("[bootstrap] Running reconcile (UPSERT mode)…");
    let prior_mode = env::var("LEGACY_SYNC_RECONCILE_MODE").ok();
    // SAFETY: setting an env var pre-tokio-runtime is safe here because
    // we own the entire process state; the watcher binary doesn't fork.
    env::set_var("LEGACY_SYNC_RECONCILE_MODE", "upsert");
    // Pass `None` for the slack client: bootstrap uses the Slack
    // channel for its own progress + refusal messages and must not
    // emit a Phase 6 drift alert during a fresh seed (canonical state
    // hasn't existed yet, so every legacy row is a "PG miss" by
    // construction — a drift alert would fire unconditionally).
    hotel_backend::scheduler::sync::run_sync(&mssql, &pg, None, &site.id).await;
    match prior_mode {
        Some(v) => env::set_var("LEGACY_SYNC_RECONCILE_MODE", v),
        None => env::remove_var("LEGACY_SYNC_RECONCILE_MODE"),
    }
    tracing::info!("[bootstrap] Reconcile complete");

    // Phase 5.5c-b: snapshot the 6 transactional legacy_mirror tables.
    // These are CT-tracked (5.5b) and the CT mappers (5.5c) maintain
    // them incrementally going forward, but CT history only carries
    // changes from MIN_VALID_VERSION onward — pre-existing rows would
    // never appear in the mirror without this one-shot snapshot.
    // Bootstrap-only path; the regular reconcile cycle does NOT touch
    // these tables (would defeat the point of CT real-time mirroring).
    tracing::info!("[bootstrap] Snapshotting legacy_mirror transactional tables…");
    hotel_backend::scheduler::mirror::snapshot_mirror_transactional_tables(&mssql, &pg).await;
    tracing::info!("[bootstrap] Mirror transactional snapshot complete");

    // Phase 2: stamp the watermark to the version captured BEFORE the
    // reconcile + snapshot (audit finding N2). Use a direct UPDATE
    // (NOT `watermark::advance`) because advance has a guard
    // `last_seen_version <= $1` that blocks moving backward; bootstrap
    // is allowed to OVERWRITE the watermark even if a prior partial run
    // bumped it past `snapshot_version`.
    //
    // The watcher's next tick will resume from `snapshot_version`,
    // re-applying every CT row produced from snapshot-time onward. The
    // overlap with the snapshot itself is harmless (idempotent UPSERT
    // / DELETE — see the comment above the version capture).
    sqlx::query(
        "UPDATE legacy_ct_state \
            SET last_seen_version = $1, \
                last_polled_at    = now() \
          WHERE id = 1",
    )
    .bind(snapshot_version)
    .execute(&pg)
    .await?;
    tracing::info!(
        watermark = snapshot_version,
        "[bootstrap] CT watermark stamped — bootstrap complete. \
         Operator may now flip LEGACY_SYNC_ENABLED=true."
    );

    Ok(())
}

/// One-shot reachability probe against legacy MSSQL. Acquires a pool
/// connection and runs `SELECT 1` — both must succeed.
///
/// Used at the top of `run_one_tick` to short-circuit the whole tick
/// when the legacy tunnel is down. Without this, every CT-enabled
/// table's fetch sequentially burns one `POOL_CONNECTION_TIMEOUT`
/// (5s as of R2 / 2026-05-14) before bb8 gives up, so a 2-minute WG
/// flap still fans out into ~16× that budget before the watcher
/// catches up. One probe up front collapses that to a single WARN
/// per tick + the next tick (1s later) re-probes and resumes
/// immediately on recovery.
async fn probe_legacy_connectivity(
    mssql: &DbPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    // Probe runs read-budget: detects a stuck `SELECT 1` (e.g. a
    // global tempdb contention spike) without inheriting it for the
    // rest of the tick.
    let _ = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT 1",
        MssqlOpKind::Read,
    )
    .await?;
    Ok(())
}

/// Read `SELECT CHANGE_TRACKING_CURRENT_VERSION()` from MSSQL — the
/// global-monotonic version every CT row carries. Returns the watermark
/// the watcher should resume from after `run_bootstrap`.
async fn read_change_tracking_current_version(
    mssql: &DbPool,
) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT CHANGE_TRACKING_CURRENT_VERSION() AS v",
        MssqlOpKind::Read,
    )
    .await?;
    let row = rows.first().ok_or_else(|| {
        "CHANGE_TRACKING_CURRENT_VERSION() returned no rows".to_string()
    })?;
    let v: Option<i64> = row.get("v");
    Ok(v.unwrap_or(0))
}

/// Watchdog-only probe of `CHANGE_TRACKING_CURRENT_VERSION()` with an
/// explicit `timeout` (default [`DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS`],
/// 12s; tunable via `LEGACY_SYNC_PROBE_TIMEOUT_MS`). Used solely by
/// [`run_watermark_watchdog`] to gate the stall alert on whether legacy
/// MSSQL actually has new CT versions to offer. The budget is passed in
/// (rather than read from the const directly) so the watchdog resolves
/// the env override once at startup and both the stall-branch and
/// recovery-branch probes share it. Failing fast and falling through to
/// "fire alert" (per [`should_fire_stall_alert`]) is the desired
/// behaviour on a genuinely unreachable legacy.
async fn probe_change_tracking_current_version(
    mssql: &DbPool,
    timeout: Duration,
) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    let rows = simple_query_with_explicit_timeout(
        &mut conn,
        "SELECT CHANGE_TRACKING_CURRENT_VERSION() AS v",
        timeout,
    )
    .await?;
    let row = rows.first().ok_or_else(|| {
        "CHANGE_TRACKING_CURRENT_VERSION() returned no rows".to_string()
    })?;
    let v: Option<i64> = row.get("v");
    Ok(v.unwrap_or(0))
}

/// Tunable knobs for [`create_pool_with_retry`]. All three default to
/// the `DEFAULT_INIT_RETRY_*` constants and can be overridden at
/// runtime via the matching `LEGACY_SYNC_INIT_RETRY_*` env vars.
#[derive(Debug, Clone, Copy)]
struct InitRetryConfig {
    initial_backoff: Duration,
    max_backoff: Duration,
    alert_after: Duration,
}

impl InitRetryConfig {
    fn from_env() -> Self {
        let initial = env::var("LEGACY_SYNC_INIT_RETRY_INITIAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_INIT_RETRY_INITIAL_SECS);
        let max = env::var("LEGACY_SYNC_INIT_RETRY_MAX_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_INIT_RETRY_MAX_SECS);
        let alert = env::var("LEGACY_SYNC_INIT_RETRY_ALERT_AFTER_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_INIT_RETRY_ALERT_AFTER_SECS);
        Self {
            initial_backoff: Duration::from_secs(initial),
            max_backoff: Duration::from_secs(max),
            alert_after: Duration::from_secs(alert),
        }
    }
}

/// Compute the next backoff interval for the MSSQL pool-init retry
/// loop. Doubles `current` (capped at `max`). Pulled into a free
/// function so the schedule is unit-testable without a tokio runtime.
///
/// Expected progression with the defaults (initial=5s, max=60s):
/// `5s → 10s → 20s → 40s → 60s → 60s → 60s → ...`
fn next_backoff(current: Duration, max: Duration) -> Duration {
    let doubled = current.saturating_mul(2);
    if doubled > max {
        max
    } else {
        doubled
    }
}

/// Retry [`create_pool`] with exponential backoff (capped) until it
/// succeeds. Never returns `Err` — the container should ride out a
/// transient MSSQL outage instead of crash-looping under Docker's
/// `restart: on-failure:5` policy.
///
/// Fires a Slack alert once the total elapsed retry time crosses
/// `config.alert_after` so an operator gets paged when the outage is
/// long enough to be operationally interesting (default 5 min). The
/// alert is one-shot per `create_pool_with_retry` call — we don't want
/// to spam Slack on every retry attempt during a multi-hour outage.
///
/// `caller_tag` is a short identifier ("ct-watcher" / "bootstrap")
/// that appears in the Slack message + log lines so a single
/// shared-binary instance can attribute its alerts correctly.
async fn create_pool_with_retry(
    config: &DbConfig,
    retry_config: &InitRetryConfig,
    slack: Option<&SlackClient>,
    site_id: &str,
    caller_tag: &str,
) -> DbPool {
    let started_at = Instant::now();
    let mut backoff = retry_config.initial_backoff;
    let mut attempt: u32 = 0;
    let mut alerted = false;

    loop {
        attempt += 1;
        match create_pool(config).await {
            Ok(pool) => {
                if attempt > 1 {
                    tracing::info!(
                        site = %site_id,
                        caller = caller_tag,
                        attempt,
                        elapsed_secs = started_at.elapsed().as_secs(),
                        "MSSQL pool init succeeded after retry"
                    );
                }
                return pool;
            }
            Err(err) => {
                let elapsed = started_at.elapsed();
                tracing::warn!(
                    site = %site_id,
                    caller = caller_tag,
                    attempt,
                    elapsed_secs = elapsed.as_secs(),
                    next_retry_secs = backoff.as_secs(),
                    error = %err,
                    "MSSQL pool init failed — retrying with backoff"
                );

                if !alerted && elapsed >= retry_config.alert_after {
                    alerted = true;
                    if let Some(s) = slack {
                        let elapsed_secs = elapsed.as_secs();
                        let payload = SlackMessage::with_site_text(
                            site_id,
                            format!(
                                ":warning: *MSSQL pool init stalled* :warning:\n\
                                 Caller: `{caller_tag}`\n\
                                 Attempts: {attempt}\n\
                                 Elapsed: {elapsed_secs}s\n\
                                 Latest error: `{err}`\n\
                                 _The container is retrying with exponential \
                                 backoff (capped) and will NOT exit. Common \
                                 cause: legacy MSSQL unreachable (WG tunnel \
                                 flap, MSSQL restart, network partition)._"
                            ),
                        );
                        let _ = s.send_message(&payload).await;
                    }
                }

                tokio::time::sleep(backoff).await;
                backoff = next_backoff(backoff, retry_config.max_backoff);
            }
        }
    }
}

/// Track D / T7 CRIT-3 — observation read from `legacy_ct_state`.
/// Used by [`watermark_stall_watchdog_alerts_when_version_stuck`] to
/// keep state across polls without a DB round-trip for the prior
/// version.
#[derive(Debug, Clone, Copy)]
struct WatermarkObservation {
    last_seen_version: i64,
    /// `last_polled_at` from PG — surfaces whether the watcher is
    /// actively ticking. If it's stale, the issue isn't a stuck
    /// watermark — it's a stalled tick loop, handled by other alerts.
    last_polled_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When we observed this snapshot (process-local, not from PG).
    /// Used to compute how long the watermark has been stuck across
    /// the watchdog's own polls.
    observed_at: Instant,
}

/// Pure decision function for the watermark-stall alert. Returns
/// `Some(reason)` when the operator should be paged, `None` otherwise.
///
/// Inputs are intentionally explicit (no env reads, no clock) so the
/// unit test can drive the truth table without spinning a tokio
/// runtime or mocking PG.
///
/// Rules:
/// 1. If `current.last_seen_version > prior.last_seen_version` → watermark
///    advanced. Reset the timer; no alert.
/// 2. If `current.last_seen_version == prior.last_seen_version` AND
///    the stuck duration is below the threshold → no alert (steady-state
///    idle is normal).
/// 3. If the version is stuck for `>= stall_alert_secs` AND shadow_mode
///    is FALSE → ALERT (the watcher is alive — this watchdog tick is the
///    liveness proof — but the watermark isn't advancing; the canonical
///    CT-watermark-stall trap). NOTE: this function does NOT read
///    `last_polled_at`; only the version and `prior.observed_at` matter.
///    Whether legacy is genuinely idle vs. wedged is decided downstream
///    by the CT probe in [`should_fire_stall_alert`].
/// 4. Shadow mode stalls don't fire from this rule — they're caught by
///    [`shadow_mode_pager_eligible`] separately.
fn watermark_stall_alert_eligible(
    prior: &WatermarkObservation,
    current: &WatermarkObservation,
    now: Instant,
    shadow_mode: bool,
    stall_threshold: Duration,
) -> Option<String> {
    if current.last_seen_version > prior.last_seen_version {
        return None;
    }
    if shadow_mode {
        // Shadow stall is loud-by-design — see shadow_mode_pager_eligible.
        return None;
    }
    let stuck_for = now.duration_since(prior.observed_at);
    if stuck_for < stall_threshold {
        return None;
    }
    Some(format!(
        "CT watermark stuck at {} for {}s (threshold {}s)",
        current.last_seen_version,
        stuck_for.as_secs(),
        stall_threshold.as_secs(),
    ))
}

/// Pure decision function for the quiet-aware stall alert override.
///
/// After [`watermark_stall_alert_eligible`] has already concluded a stall
/// is alert-worthy by duration, this gates the actual Slack page on
/// whether legacy MSSQL has anything new to offer. We compare the
/// canonical PG watermark against the live
/// `CHANGE_TRACKING_CURRENT_VERSION()` probe.
///
/// Rules (conservative — uncertainty defaults to firing):
/// 1. Probe returned `Some(v)` AND `v == watermark` → legacy CT is quiet
///    at the same version the watermark already holds. Watcher is
///    correctly tracking tip-of-stream — suppress alert (`false`).
/// 2. Probe returned `Some(v)` AND `v > watermark` → legacy has changes
///    the watcher has NOT processed. Real stall — alert (`true`).
/// 3. Probe returned `Some(v)` AND `v < watermark` → impossible in a
///    healthy CT instance (CT version is monotonic), but defensively
///    treat as a real problem worth paging on (`true`).
/// 4. Probe returned `None` (failure / timeout) → uncertainty. Don't
///    suppress on a probe failure — fire the alert (`true`) so an
///    operator can investigate (matches the prompt's "conservative —
///    don't suppress on uncertainty" requirement).
///
/// Background: 2026-05-14 off-peak quiet periods triggered three
/// false-positive Slack pages on hfhotel + two on hfville. The
/// `legacy_ct_state.last_seen_version` correctly matched
/// `CHANGE_TRACKING_CURRENT_VERSION()` (verified 17209 == 17209), so
/// there was nothing to advance to. The original watchdog had no way
/// to tell idle from wedged.
fn should_fire_stall_alert(watermark: i64, ct_current_probe: Option<i64>) -> bool {
    match ct_current_probe {
        Some(v) if v == watermark => false,
        _ => true,
    }
}

/// Streak gate for the probe-timeout (`:information_source:`) class. When the
/// watchdog probe fails (`None`), a single observation is noise — a
/// timeout inside a 60s tick is usually transient iHOTEL lock
/// contention that clears on the next tick. This function suppresses
/// the page until N consecutive failures have been observed
/// (`consecutive_failures >= threshold`), giving the signal time to
/// stabilise.
///
/// Probe successes (`Some(_)`) bypass the gate entirely — the
/// confirmed-backlog and confirmed-quiet branches in
/// [`should_fire_stall_alert`] still fire / suppress on the first
/// observation. The gate only changes behaviour for the uncertainty
/// class.
///
/// Returns `true` when the streak has crossed the threshold (or the
/// probe succeeded — caller is then expected to defer to
/// [`should_fire_stall_alert`] for the actual decision). Returns
/// `false` when the page should be suppressed this tick.
///
/// Background: 2026-05-22 four self-recovering `:warning:` pages on
/// hfhotel within ~8h, all from single 5s probe timeouts during
/// overnight quiet periods. With threshold=3, the same workload would
/// have produced zero pages because the next tick's probe succeeded
/// each time (verified by the `:white_check_mark: RECOVERED` message
/// firing within 1-3 min on every alert).
fn probe_failure_streak_passes_gate(
    probe: Option<i64>,
    consecutive_failures: u32,
    threshold: u32,
) -> bool {
    match probe {
        Some(_) => true,
        None => consecutive_failures >= threshold,
    }
}

/// Persistence gate for the confirmed-backlog (`:rotating_light:` *STUCK*)
/// class (2026-06-29). A backlog (`ct_current > watermark`) the CT watcher
/// drains within its own poll cycle is normal lag, not a stall — so the
/// critical page is suppressed until the backlog has been observed on
/// `consecutive_observations >= threshold` consecutive watchdog ticks.
///
/// Returns `true` once the streak crosses the threshold (page allowed),
/// `false` while it's still below (suppress this tick). Mirrors
/// [`probe_failure_streak_passes_gate`] so the loop body stays small and
/// the rule stays unit-testable.
///
/// Background: a long quiet window followed by a single new change let the
/// watchdog observe `ct_current > watermark` for one tick before the
/// watcher consumed it, paging a self-recovering critical "STUCK" that
/// inherited the idle duration as its "stuck" figure (e.g. "stuck 7807s,
/// 1 version unprocessed" → RECOVERED 60s later). See
/// [`DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD`].
fn backlog_persist_passes_gate(consecutive_observations: u32, threshold: u32) -> bool {
    consecutive_observations >= threshold
}

/// Pure decision for the probe-timeout-outage escalation (2026-06-26).
///
/// The `:information_source:` probe-timeout page is benign WHEN it
/// self-recovers within a tick or two (legacy was just briefly slow).
/// But if the probe keeps timing out and the alert never recovers, the
/// info class becomes a blind spot: a real legacy outage (or a genuine
/// backlog we can't see because the probe can't reach legacy) would keep
/// re-firing `:information_source: …no action needed` forever and never
/// escalate to `:rotating_light:` — because the critical branch requires
/// a SUCCESSFUL probe returning `v > watermark`.
///
/// This closes that gap: once an info-class alert has been open for
/// `threshold` without recovering, escalate ONCE (the caller flips
/// `already_escalated` so we don't spam).
///
/// * `info_outage_since` — when the current unrecovered info-outage run
///   began (set on the first info page, cleared on recovery / probe
///   success / no-stall).
/// * `already_escalated` — whether this outage run has already escalated.
fn probe_outage_escalation_eligible(
    info_outage_since: Option<Instant>,
    now: Instant,
    threshold: Duration,
    already_escalated: bool,
) -> bool {
    match info_outage_since {
        Some(since) if !already_escalated => now.duration_since(since) >= threshold,
        _ => false,
    }
}

/// Severity-aware cooldown gate for stall pages (2026-06-11, paired
/// with the probe-timeout demotion to `:information_source:`).
///
/// `last` is the previous page as `(when, was_critical)`;
/// `is_critical` describes the page being considered now (critical =
/// the probe SUCCEEDED and confirmed a backlog or monotonicity
/// violation; non-critical = the probe-timeout informational class).
///
/// Rules:
/// 1. No prior page → pass.
/// 2. Cooldown elapsed → pass.
/// 3. Inside cooldown, but the new page is CRITICAL and the prior page
///    was only informational → pass (escalation bypass). Without this,
///    a probe-timeout info note would shadow a confirmed-backlog
///    `:rotating_light:` for up to [`WATCHDOG_ALERT_COOLDOWN_SECS`].
/// 4. Otherwise → suppress.
fn stall_page_passes_cooldown(
    last: Option<(Instant, bool)>,
    now: Instant,
    cooldown: Duration,
    is_critical: bool,
) -> bool {
    match last {
        None => true,
        Some((paged_at, was_critical)) => {
            now.duration_since(paged_at) >= cooldown || (is_critical && !was_critical)
        }
    }
}

/// Tone-aware Slack message for a watermark stall page. Severity is
/// derived from the probe outcome:
///
/// * `probe = None` (timeout / DB failure) → `:information_source:` —
///   uncertainty, NOT a confirmed backlog. The 2026-05-19 false
///   positives (3 pages on hfhotel) all fell in this bucket: iHOTEL
///   was busy/slow, probe timed out, every alert self-recovered
///   within minutes. Originally paged as `:warning:`; demoted to
///   informational on 2026-06-11 after three more self-recovering
///   pages in one day (hfhotel ×2, hfville ×1) confirmed the
///   quiet-period + slow-probe combination is the expected overnight
///   pattern, not an operator signal. A later probe that confirms a
///   real backlog escalates to `:rotating_light:` and bypasses the
///   cooldown (see [`stall_page_passes_cooldown`]).
/// * `probe = Some(c)` AND `c > watermark` → `:rotating_light:` —
///   legacy CT is ahead of the watermark by `c - w` versions.
///   Confirmed backlog. Real stall.
/// * `probe = Some(c)` AND `c < watermark` → `:rotating_light:` —
///   monotonicity violation. Impossible in healthy CT.
///
/// `probe = Some(c)` where `c == watermark` is already suppressed
/// upstream by [`should_fire_stall_alert`] — this function should
/// never be called in that case, but if it is we fall through to the
/// monotonicity branch's wording (defensive).
fn format_stall_alert_message(
    watermark: i64,
    probe: Option<i64>,
    stuck_for: Duration,
    threshold: Duration,
) -> String {
    let stuck_secs = stuck_for.as_secs();
    let threshold_secs = threshold.as_secs();
    let dashboard_hint =
        "Check `legacy_sync_status.last_error` or the dashboard at `/api/new/sync/status`.";

    match probe {
        None => format!(
            ":information_source: *CT watermark idle — probe timed out* (informational)\n\
             Watermark at v{watermark} for {stuck_secs}s (threshold {threshold_secs}s); \
             legacy probe timed out so we cannot confirm a real backlog. This is the \
             expected quiet-period pattern (iHOTEL busy/slow while nothing changes) — \
             no action needed. Escalates to a critical page automatically if a later \
             probe confirms a real backlog.\n\
             _{dashboard_hint}_"
        ),
        Some(ct_current) if ct_current > watermark => {
            let delta = ct_current - watermark;
            format!(
                ":rotating_light: *CT watermark STUCK* :rotating_light:\n\
                 Watermark at v{watermark} but legacy current is v{ct_current} \
                 ({delta} versions unprocessed, stuck {stuck_secs}s, threshold \
                 {threshold_secs}s). Real changes are not being processed.\n\
                 _{dashboard_hint}_"
            )
        }
        Some(ct_current) => format!(
            ":rotating_light: *CT watermark anomaly* :rotating_light:\n\
             Watermark v{watermark} exceeds legacy current v{ct_current} (stuck \
             {stuck_secs}s, threshold {threshold_secs}s). Monotonicity violated \
             — investigate immediately.\n\
             _{dashboard_hint}_"
        ),
    }
}

/// Slack message for an ESCALATED probe-timeout outage (2026-06-26).
/// Fired when the benign `:information_source:` info page has stayed open
/// past [`probe_outage_escalation_eligible`]'s threshold without
/// recovering — at which point "no action needed" is no longer true: we
/// have been unable to reach legacy CT for an extended window and cannot
/// rule out a real backlog.
fn format_probe_outage_escalation_message(
    watermark: i64,
    outage_for: Duration,
    threshold: Duration,
) -> String {
    let outage_mins = outage_for.as_secs() / 60;
    let threshold_mins = threshold.as_secs() / 60;
    format!(
        ":rotating_light: *CT watermark — legacy probe unreachable {outage_mins}min* \
         :rotating_light:\n\
         Watermark stuck at v{watermark} and the legacy CT probe has been timing out \
         for {outage_mins}min (escalation threshold {threshold_mins}min). This is NO \
         LONGER the benign quiet-period pattern — we cannot confirm whether real \
         changes are backing up toward the CT retention cliff. Investigate legacy \
         MSSQL reachability now (WireGuard tunnel, iHOTEL load).\n\
         _Check `legacy_sync_status.last_error` or the dashboard at \
         `/api/new/sync/status`._"
    )
}

/// Why the recovery message should fire. Carries enough context for
/// the caller to format the Slack message without re-deriving state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecoveryDecision {
    /// The watermark version at the time the original stall was paged.
    paged_version: i64,
    /// When the original stall alert was paged.
    paged_at: Instant,
    /// The watermark version observed right now (post-recovery).
    current_version: i64,
    /// Which condition triggered recovery — useful for the log line.
    reason: RecoveryReason,
}

/// Discriminator on which of the two recovery rules triggered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecoveryReason {
    /// Watermark advanced past the version we paged on.
    WatermarkAdvanced,
    /// Watermark didn't advance, but the legacy probe now confirms
    /// legacy CT is idle at our watermark (so there's nothing to
    /// process, which means the stall was really an idle window).
    ProbeConfirmsQuiet,
}

/// Pure decision function for the recovery notification. Mirrors the
/// pattern of [`watermark_stall_alert_eligible`] /
/// [`should_fire_stall_alert`] so the watchdog loop body stays small
/// and the rules stay unit-testable without a tokio runtime.
///
/// Inputs:
/// * `pending` — the open-alert state (`Some((paged_at, paged_version))`
///   set by the caller after a successful page, `None` otherwise).
/// * `current_version` — `observation.last_seen_version`.
/// * `probe` — the result of a fresh
///   [`probe_change_tracking_current_version`] call this iteration.
///   `None` means the probe failed or wasn't attempted.
///
/// Returns `Some(RecoveryDecision)` when the watchdog should fire ONE
/// recovery message and clear the pending state.
///
/// Recovery fires when there's an open alert AND **either**:
/// 1. The watermark has advanced past `paged_version` (the watcher
///    caught up).
/// 2. The probe returned `Some(v)` where `v == current_version` —
///    legacy CT is idle at our watermark (same condition
///    [`should_fire_stall_alert`] uses to suppress new alerts). This
///    covers the "iHOTEL was just quiet, the watermark was already
///    correct" case from 2026-05-19.
///
/// Recovery does NOT fire on a probe failure during the recovery
/// check — that's uncertainty, and we'd rather hold the open alert
/// than declare a premature all-clear.
fn recovery_alert_eligible(
    pending: Option<(Instant, i64)>,
    current_version: i64,
    probe: Option<i64>,
) -> Option<RecoveryDecision> {
    let (paged_at, paged_version) = pending?;

    if current_version > paged_version {
        return Some(RecoveryDecision {
            paged_version,
            paged_at,
            current_version,
            reason: RecoveryReason::WatermarkAdvanced,
        });
    }
    if matches!(probe, Some(v) if v == current_version) {
        return Some(RecoveryDecision {
            paged_version,
            paged_at,
            current_version,
            reason: RecoveryReason::ProbeConfirmsQuiet,
        });
    }
    None
}

/// Format the human-readable "alert duration" for the recovery
/// message. No `humantime` dep in the project, so we round to the
/// coarsest sensible unit (`s` / `min` / `h`) — simplicity over
/// polish, as the prompt directs.
fn format_alert_duration(elapsed: Duration) -> String {
    let secs = elapsed.as_secs();
    if secs < 60 {
        format!("{secs}s ago")
    } else if secs < 3600 {
        format!("{}min ago", secs / 60)
    } else {
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        if mins == 0 {
            format!("{hours}h ago")
        } else {
            format!("{hours}h{mins}min ago")
        }
    }
}

/// Format the Slack recovery message body. Pulled into a helper for
/// the same testability reason as [`format_stall_alert_message`].
fn format_recovery_message(decision: &RecoveryDecision, now: Instant) -> String {
    let elapsed = now.duration_since(decision.paged_at);
    let duration_ago = format_alert_duration(elapsed);
    // Wording must match the actual reason. Before 2026-06-11 both
    // branches claimed "resumed advancing", which read as nonsense
    // when the version in the recovery equalled the paged version
    // (the ProbeConfirmsQuiet case — nothing ever advanced).
    let detail = match decision.reason {
        RecoveryReason::WatermarkAdvanced => format!(
            "Watermark resumed advancing (now at v{}).",
            decision.current_version
        ),
        RecoveryReason::ProbeConfirmsQuiet => format!(
            "Legacy CT confirmed idle at v{} — no backlog existed; the watermark \
             was already at tip-of-stream.",
            decision.current_version
        ),
    };
    format!(
        ":white_check_mark: *CT watermark RECOVERED*\n\
         {detail} Prior alert from {duration_ago} cleared.\n\
         _Dashboard: `/api/new/sync/status`._"
    )
}

/// Pure decision function for the shadow-mode-too-long alert. Returns
/// `Some(reason)` when shadow mode has been running for longer than
/// the hardcoded ceiling ([`SHADOW_MODE_MAX_DURATION_SECS`], 36h).
///
/// The MSSQL CT retention default is 2 days; staying in shadow mode
/// past 36h leaves <12h before the retention cliff silently drops
/// changes the next tick would have replayed.
fn shadow_mode_pager_eligible(
    shadow_mode: bool,
    started_at: Instant,
    now: Instant,
) -> Option<String> {
    if !shadow_mode {
        return None;
    }
    let duration = now.duration_since(started_at);
    let max = Duration::from_secs(SHADOW_MODE_MAX_DURATION_SECS);
    if duration < max {
        return None;
    }
    Some(format!(
        "Shadow mode has been running for {}s (ceiling {}s, ≈36h). \
         MSSQL CT retention is 2 days; staying in shadow much longer \
         risks the watermark dropping behind MIN_VALID_VERSION.",
        duration.as_secs(),
        max.as_secs(),
    ))
}

/// Track D / T7 CRIT-3 — spawn the watermark-stall watchdog. Runs as a
/// detached background task; reads `legacy_ct_state` every 60s and
/// fires Slack alerts on either (a) version stuck >= `stall_alert_secs`
/// in live mode, or (b) shadow mode running past the 36h ceiling.
/// Cooldown 30min per alert kind so we don't flood.
///
/// 2026-05-14 quiet-aware refinement: when the duration-based stall
/// rule fires in live mode, the watchdog now probes
/// `CHANGE_TRACKING_CURRENT_VERSION()` on legacy MSSQL with its own
/// 5-second timeout budget (reusing R2's
/// `simple_query_with_explicit_timeout`) and suppresses the page when
/// legacy is just idle at the same version our watermark already
/// holds. Probe failures fall through to firing — see
/// [`should_fire_stall_alert`].
async fn run_watermark_watchdog(
    pg: PgPool,
    mssql: DbPool,
    slack: Option<SlackClient>,
    site_id: String,
    stall_alert_secs: u64,
    shutdown: Arc<Notify>,
) {
    let started_at = Instant::now();
    let stall_threshold = Duration::from_secs(stall_alert_secs);
    let cooldown = Duration::from_secs(WATCHDOG_ALERT_COOLDOWN_SECS);
    let mut prior: Option<WatermarkObservation> = None;
    // `(when, was_critical)` — severity travels with the timestamp so
    // a confirmed-backlog page can bypass the cooldown left by a mere
    // probe-timeout informational note (see `stall_page_passes_cooldown`).
    let mut last_stall_alert: Option<(Instant, bool)> = None;
    let mut last_shadow_alert: Option<Instant> = None;
    // Open-alert tracking for the recovery notification (PR D,
    // 2026-05-19). Parallel to `last_stall_alert` — that one encodes
    // cooldown, this one encodes "we paged and haven't yet declared
    // all-clear". Cleared after firing the recovery message. Tuple:
    // `(paged_at, watermark_at_page_time)`.
    let mut pending_stall_alert: Option<(Instant, i64)> = None;
    // 2026-05-22 — streak gate for the probe-timeout (informational)
    // class. Counts CONSECUTIVE probe failures in the stall branch.
    // Reset on probe success, watermark advance, or any tick that
    // doesn't run a probe (no stall this tick).
    let mut probe_failure_streak: u32 = 0;
    // 2026-06-29 — persistence gate for the confirmed-backlog
    // (`:rotating_light:` STUCK) class. `backlog_streak` counts
    // CONSECUTIVE ticks where the probe confirmed `ct_current > watermark`;
    // `backlog_since` anchors when the backlog was first observed so the
    // critical page reports the BACKLOG age, not the (potentially huge,
    // idle-dominated) watermark-freeze age. Both reset on a caught-up
    // probe (`== watermark`), a watermark advance, or a no-stall tick.
    let mut backlog_streak: u32 = 0;
    let mut backlog_since: Option<Instant> = None;
    // 2026-06-26 — probe-timeout-outage escalation state. `since` anchors
    // the first info page of the current unrecovered outage; `escalated`
    // makes the escalation one-time per outage. Both clear on recovery,
    // probe success, or a no-stall tick (same lifecycle as the streak).
    let mut info_outage_since: Option<Instant> = None;
    let mut info_outage_escalated: bool = false;

    let shadow_mode = env::var("LEGACY_SYNC_SHADOW_MODE")
        .map(|v| v == "true")
        .unwrap_or(false);

    let probe_timeout_streak_threshold = env::var("LEGACY_SYNC_PROBE_TIMEOUT_STREAK")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PROBE_TIMEOUT_STREAK_THRESHOLD);

    let backlog_persist_threshold = env::var("LEGACY_SYNC_BACKLOG_PERSIST_STREAK")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD);

    let probe_timeout = Duration::from_millis(
        env::var("LEGACY_SYNC_PROBE_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS),
    );

    let probe_outage_escalation = Duration::from_secs(
        env::var("LEGACY_SYNC_PROBE_OUTAGE_ESCALATION_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_PROBE_OUTAGE_ESCALATION_SECS),
    );

    tracing::info!(
        site = %site_id,
        stall_alert_secs,
        shadow_mode,
        probe_timeout_streak_threshold,
        backlog_persist_threshold,
        probe_timeout_ms = probe_timeout.as_millis() as u64,
        probe_outage_escalation_secs = probe_outage_escalation.as_secs(),
        "[watchdog] Watermark-stall watchdog starting"
    );

    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(WATERMARK_WATCHDOG_POLL_INTERVAL_SECS)) => {}
            _ = shutdown.notified() => {
                tracing::info!(site = %site_id, "[watchdog] Shutdown — exiting");
                return;
            }
        }

        let now = Instant::now();

        // Read both watermark + last_polled_at in one round-trip.
        let observation = match read_ct_state(&pg).await {
            Ok(o) => o,
            Err(err) => {
                tracing::warn!(
                    site = %site_id,
                    error = %err,
                    "[watchdog] Failed to read legacy_ct_state — observability degraded"
                );
                continue;
            }
        };

        // Recovery check (PR D, 2026-05-19). If we have an open stall
        // alert, decide whether to fire the all-clear THIS iteration.
        // Recovery short-circuits on the cheap "watermark advanced"
        // rule and only spends a probe call on the "still stuck but
        // legacy might be idle now" path — we don't want to burn the
        // probe budget on every idle loop iteration.
        if let Some((paged_at, paged_version)) = pending_stall_alert {
            let mut recovery_probe: Option<i64> = None;
            // Cheap branch first — if the watermark moved past the
            // paged version we don't need to probe at all.
            let advanced = observation.last_seen_version > paged_version;
            if !advanced {
                // Watermark didn't advance — legacy might be idle and
                // matching us now. Spend one probe to find out. On
                // failure we DON'T declare recovery (uncertainty
                // holds the open alert).
                recovery_probe = match probe_change_tracking_current_version(&mssql, probe_timeout).await {
                    Ok(v) => Some(v),
                    Err(err) => {
                        tracing::warn!(
                            site = %site_id,
                            error = %err,
                            "[watchdog] CT-current probe failed during recovery check \
                             — holding open alert"
                        );
                        None
                    }
                };
            }
            if let Some(decision) = recovery_alert_eligible(
                Some((paged_at, paged_version)),
                observation.last_seen_version,
                recovery_probe,
            ) {
                tracing::info!(
                    site = %site_id,
                    paged_version = decision.paged_version,
                    current_version = decision.current_version,
                    reason = ?decision.reason,
                    "[watchdog] Watermark recovered — firing all-clear"
                );
                if let Some(s) = slack.as_ref() {
                    let payload = SlackMessage::with_site_text(
                        &site_id,
                        format_recovery_message(&decision, now),
                    );
                    let _ = s.send_message(&payload).await;
                }
                pending_stall_alert = None;
                // Outage is over — reset the escalation anchor so the
                // next unrecovered run starts fresh.
                info_outage_since = None;
                info_outage_escalated = false;
            }
        }

        // Watermark stall check (live mode only).
        if let Some(prior_obs) = prior.as_ref() {
            if let Some(reason) =
                watermark_stall_alert_eligible(prior_obs, &observation, now, shadow_mode, stall_threshold)
            {
                // 2026-05-14 quiet-aware gate. The duration rule has
                // already concluded the watermark hasn't moved in
                // `stall_threshold`. Probe legacy CT to distinguish
                // "wedged" (CT current > watermark) from "idle"
                // (CT current == watermark, nothing to advance to).
                // 2026-05-22: probe failures are gated by a consecutive-
                // failure streak (see `probe_failure_streak_passes_gate`)
                // — a single timeout is dominated by transient iHOTEL
                // lock contention and self-clears on the next tick.
                let probe = match probe_change_tracking_current_version(&mssql, probe_timeout).await {
                    Ok(v) => {
                        probe_failure_streak = 0;
                        // Probe reachable → any in-flight probe-timeout
                        // outage is over; clear the escalation anchor.
                        info_outage_since = None;
                        info_outage_escalated = false;
                        Some(v)
                    }
                    Err(err) => {
                        probe_failure_streak = probe_failure_streak.saturating_add(1);
                        tracing::warn!(
                            site = %site_id,
                            error = %err,
                            streak = probe_failure_streak,
                            threshold = probe_timeout_streak_threshold,
                            "[watchdog] CT-current probe failed — streak incremented"
                        );
                        None
                    }
                };
                if !should_fire_stall_alert(observation.last_seen_version, probe) {
                    // Probe confirms legacy CT is idle at our watermark —
                    // caught up. Any backlog we were tracking is drained.
                    backlog_streak = 0;
                    backlog_since = None;
                    tracing::info!(
                        site = %site_id,
                        watermark = observation.last_seen_version,
                        ct_current = probe.unwrap_or_default(),
                        "[watchdog] legacy CT quiet — watermark correctly tracking current"
                    );
                } else if !probe_failure_streak_passes_gate(
                    probe,
                    probe_failure_streak,
                    probe_timeout_streak_threshold,
                ) {
                    tracing::info!(
                        site = %site_id,
                        watermark = observation.last_seen_version,
                        streak = probe_failure_streak,
                        threshold = probe_timeout_streak_threshold,
                        "[watchdog] probe timeout — suppressing informational page until streak crosses threshold"
                    );
                } else if probe.is_none() {
                    // Probe TIMED OUT — the informational class. Anchor the
                    // outage on the first info page so a sustained
                    // probe-unreachable window can escalate (2026-06-26).
                    if info_outage_since.is_none() {
                        info_outage_since = Some(now);
                    }
                    let stuck_for = now.duration_since(prior_obs.observed_at);
                    if probe_outage_escalation_eligible(
                        info_outage_since,
                        now,
                        probe_outage_escalation,
                        info_outage_escalated,
                    ) {
                        // Open too long without recovering — this is no
                        // longer benign. Escalate ONCE, bypassing cooldown.
                        let outage_for = info_outage_since
                            .map(|t| now.duration_since(t))
                            .unwrap_or_default();
                        tracing::error!(
                            site = %site_id,
                            version = observation.last_seen_version,
                            outage_secs = outage_for.as_secs(),
                            "[watchdog] probe-timeout outage exceeded escalation threshold — paging operator"
                        );
                        if let Some(s) = slack.as_ref() {
                            let payload = SlackMessage::with_site_text(
                                &site_id,
                                format_probe_outage_escalation_message(
                                    observation.last_seen_version,
                                    outage_for,
                                    probe_outage_escalation,
                                ),
                            );
                            let _ = s.send_message(&payload).await;
                        }
                        info_outage_escalated = true;
                        // Record the cooldown anchor as NON-critical so a
                        // later *confirmed-backlog* critical (probe succeeds,
                        // v > watermark) can still bypass this cooldown via
                        // the `is_critical && !was_critical` rule — we don't
                        // want an escalation to shadow a precise backlog page
                        // for up to a full cooldown. Subsequent *info* pages
                        // are still cooldown-suppressed (info vs false → no
                        // bypass), so this doesn't reintroduce spam.
                        last_stall_alert = Some((now, false));
                        pending_stall_alert = Some((now, observation.last_seen_version));
                    } else {
                        // Probe TIMED OUT — formerly the "informational" page.
                        // 2026-06-30 (operator request): we no longer Slack this
                        // benign quiet-period pattern. It self-clears on the next
                        // reachable probe, and the only actionable variant — a
                        // SUSTAINED probe outage — still pages via the escalation
                        // branch above. We deliberately do NOT arm
                        // `pending_stall_alert` here, so the paired all-clear
                        // ("CT watermark RECOVERED") is suppressed too: nothing was
                        // announced to Slack, so there is nothing to clear. The
                        // escalation path stays armed because `info_outage_since`
                        // is anchored above, independently of this branch. The log
                        // line preserves full forensics for the dashboard / tracing.
                        tracing::info!(
                            site = %site_id,
                            version = observation.last_seen_version,
                            stuck_secs = stuck_for.as_secs(),
                            outage_secs = info_outage_since
                                .map(|t| now.duration_since(t).as_secs())
                                .unwrap_or_default(),
                            escalation_secs = probe_outage_escalation.as_secs(),
                            reason,
                            "[watchdog] Watermark idle, probe timed out — informational (Slack-suppressed); escalation still armed"
                        );
                    }
                } else if let Some(ct_current) = probe {
                    // Probe SUCCEEDED and disagreed with the watermark (the
                    // `== watermark` case was suppressed above). Two sub-cases,
                    // both the critical class — bypasses the cooldown left by a
                    // prior informational page.
                    if ct_current < observation.last_seen_version {
                        // Monotonicity violation (watermark > legacy current):
                        // impossible in healthy CT — a corruption / CT-reset
                        // signal. Fire on FIRST observation; NOT streak-gated.
                        backlog_streak = 0;
                        backlog_since = None;
                        if stall_page_passes_cooldown(last_stall_alert, now, cooldown, true) {
                            let stuck_for = now.duration_since(prior_obs.observed_at);
                            tracing::error!(
                                site = %site_id,
                                version = observation.last_seen_version,
                                ct_current,
                                reason,
                                "[watchdog] Watermark monotonicity violation — paging operator"
                            );
                            if let Some(s) = slack.as_ref() {
                                let payload = SlackMessage::with_site_text(
                                    &site_id,
                                    format_stall_alert_message(
                                        observation.last_seen_version,
                                        probe,
                                        stuck_for,
                                        stall_threshold,
                                    ),
                                );
                                let _ = s.send_message(&payload).await;
                            }
                            last_stall_alert = Some((now, true));
                            pending_stall_alert = Some((now, observation.last_seen_version));
                        }
                    } else {
                        // `ct_current > watermark` — confirmed backlog.
                        // Persistence gate (2026-06-29): a backlog the watcher
                        // drains within its own poll cycle is normal lag, not a
                        // stall. Require it to survive >= N consecutive ticks,
                        // and report the BACKLOG age (since first observed), not
                        // the idle-dominated watermark-freeze age — that's the
                        // fix for the self-recovering "stuck 7807s, 1 version"
                        // pages.
                        backlog_streak = backlog_streak.saturating_add(1);
                        if backlog_since.is_none() {
                            backlog_since = Some(now);
                        }
                        let backlog_for = backlog_since
                            .map(|t| now.duration_since(t))
                            .unwrap_or_default();
                        if !backlog_persist_passes_gate(backlog_streak, backlog_persist_threshold) {
                            tracing::info!(
                                site = %site_id,
                                watermark = observation.last_seen_version,
                                ct_current,
                                delta = ct_current - observation.last_seen_version,
                                streak = backlog_streak,
                                threshold = backlog_persist_threshold,
                                "[watchdog] backlog observed — suppressing critical page until it persists past the streak gate"
                            );
                        } else if stall_page_passes_cooldown(last_stall_alert, now, cooldown, true) {
                            tracing::error!(
                                site = %site_id,
                                version = observation.last_seen_version,
                                ct_current,
                                backlog_secs = backlog_for.as_secs(),
                                streak = backlog_streak,
                                reason,
                                "[watchdog] Watermark stall detected (backlog persisted) — paging operator"
                            );
                            if let Some(s) = slack.as_ref() {
                                let payload = SlackMessage::with_site_text(
                                    &site_id,
                                    format_stall_alert_message(
                                        observation.last_seen_version,
                                        probe,
                                        backlog_for,
                                        stall_threshold,
                                    ),
                                );
                                let _ = s.send_message(&payload).await;
                            }
                            last_stall_alert = Some((now, true));
                            // Open-alert state for the recovery notification
                            // (PR D, 2026-05-19). Always set after a page.
                            pending_stall_alert = Some((now, observation.last_seen_version));
                        }
                    }
                }
            } else {
                // No stall this tick — no probe ran, so the streak's
                // "consecutive failures during the current stall" reading
                // is no longer meaningful. Reset so the next stall starts
                // with a fresh observation budget. The outage anchor clears
                // too — a non-stalling tick means the watermark is fine.
                // The backlog-persistence gate resets for the same reason:
                // the watermark is advancing again, so any prior backlog
                // streak no longer reflects a wedged watcher.
                probe_failure_streak = 0;
                backlog_streak = 0;
                backlog_since = None;
                info_outage_since = None;
                info_outage_escalated = false;
                if observation.last_seen_version > prior_obs.last_seen_version {
                    tracing::debug!(
                        site = %site_id,
                        from = prior_obs.last_seen_version,
                        to = observation.last_seen_version,
                        "[watchdog] Watermark advanced"
                    );
                }
            }
        }

        // Persist the LATEST advance moment, not the current poll —
        // so a sustained stall keeps reporting its true duration.
        let new_prior = if let Some(prior_obs) = prior.as_ref() {
            if observation.last_seen_version > prior_obs.last_seen_version {
                WatermarkObservation {
                    last_seen_version: observation.last_seen_version,
                    last_polled_at: observation.last_polled_at,
                    observed_at: now,
                }
            } else {
                // Same version — keep the original observed_at so the
                // stuck-duration grows monotonically.
                WatermarkObservation {
                    last_seen_version: observation.last_seen_version,
                    last_polled_at: observation.last_polled_at,
                    observed_at: prior_obs.observed_at,
                }
            }
        } else {
            // First observation — anchor the stuck-timer here.
            WatermarkObservation {
                observed_at: now,
                ..observation
            }
        };
        prior = Some(new_prior);

        // Shadow-mode-too-long check.
        if let Some(reason) = shadow_mode_pager_eligible(shadow_mode, started_at, now) {
            let cooldown_elapsed = match last_shadow_alert {
                Some(t) => now.duration_since(t) >= cooldown,
                None => true,
            };
            if cooldown_elapsed {
                tracing::error!(
                    site = %site_id,
                    reason,
                    "[watchdog] Shadow mode exceeded ceiling — paging operator"
                );
                if let Some(s) = slack.as_ref() {
                    let payload = SlackMessage::with_site_text(
                        &site_id,
                        format!(
                            ":no_entry: *Shadow mode exceeded {SHADOW_MODE_MAX_DURATION_SECS}s ceiling* :no_entry:\n\
                             {reason}\n\
                             _Flip `LEGACY_SYNC_SHADOW_MODE=false` and redeploy, or \
                             accept the retention-overflow risk for another tick. \
                             See docs/runbook-sync.md._"
                        ),
                    );
                    let _ = s.send_message(&payload).await;
                }
                last_shadow_alert = Some(now);
            }
        }
    }
}

/// Read both `last_seen_version` and `last_polled_at` from
/// `legacy_ct_state` in one query.
async fn read_ct_state(pg: &PgPool) -> Result<WatermarkObservation, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT last_seen_version, last_polled_at FROM legacy_ct_state WHERE id = 1",
    )
    .fetch_one(pg)
    .await?;
    Ok(WatermarkObservation {
        last_seen_version: row.0,
        last_polled_at: row.1,
        observed_at: Instant::now(),
    })
}

/// Operator-facing refusal message for the N1 live-bootstrap guard.
/// Pulled into a helper so the unit test in `mod tests` can pin the
/// wording (operators triage by message text in Slack alerts and
/// docs/runbook-sync.md cross-references it).
fn build_live_bootstrap_refusal_message() -> &'static str {
    "LEGACY_SYNC_ENABLED=true and --bootstrap was requested, but \
     LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP != true — refusing to bootstrap. \
     The bootstrap snapshot runs `DELETE FROM legacy_mirror.<table>` \
     before re-inserting `mirror_source='reconcile'` rows, which races \
     the live CT watcher's `mirror_source='ct'` UPSERTs and can clobber \
     real-time changes that landed during the snapshot window. \
     Stop the watcher first (set LEGACY_SYNC_ENABLED=false and redeploy), \
     then run --bootstrap, then re-enable. \
     Set LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP=true ONLY if you accept the \
     race window. See docs/runbook-sync.md."
}

fn parse_allowlist(raw: Option<String>) -> Option<HashSet<String>> {
    let raw = raw?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let set: HashSet<String> = trimmed
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if set.is_empty() {
        None
    } else {
        Some(set)
    }
}

/// Build the per-table mapper list, filtered by the allowlist. Phase
/// 5.4 finished the 10-table canonical sync (every CT-enabled table
/// has a real mapper or an intentional retired stub `HT_Room_Status`).
/// Phase 5.5c added the 6 legacy_mirror.* mappers, Track E1 the two
/// sync-gap closures, and Phase 5/E2 the `HT_Book_Pro` mirror —
/// bringing total mapper coverage to 19 tables.
fn build_mappers(allowlist: &Option<HashSet<String>>) -> Vec<Box<dyn MssqlChangeMapper>> {
    let allowed = |t: &str| allowlist.as_ref().map(|a| a.contains(t)).unwrap_or(true);

    let mut out: Vec<Box<dyn MssqlChangeMapper>> = Vec::with_capacity(CT_ENABLED_TABLES.len());

    for table in CT_ENABLED_TABLES {
        if !allowed(table) {
            continue;
        }
        let mapper: Box<dyn MssqlChangeMapper> = match *table {
            "HT_Customers" => Box::new(CustomerMapper),
            "HT_Rooms" => Box::new(RoomMasterMapper),
            // Track F1 (audit 2026-05-13 T1 HIGH-4) — swap the 5.4
            // retired-stub `RoomStatusMapper` for the new
            // `RoomCalendarMapper` that projects to canonical
            // `ht_room_calendar`. `RoomStatusMapper` stays available
            // for back-compat / drift-detection tests but no longer
            // owns CT dispatch.
            "HT_Room_Status" => Box::new(RoomCalendarMapper),
            "HT_Book_H" => Box::new(BookingHeaderMapper),
            "HT_Book_Ds" => Box::new(BookingRoomsMapper),
            "HT_Book_Date" => Box::new(BookingDatesMapper),
            "HT_CheckIn_H" => Box::new(CheckInHeaderMapper),
            "HT_CheckIn_Ds" => Box::new(CheckInRoomsMapper),
            "HT_CheckIn_Pay" => Box::new(PaymentMapper),
            "HT_Receipt_H" => Box::new(ReceiptMapper),
            // Phase 5.5c — legacy_mirror.* opaque pass-through mappers.
            // No DomainEvent emission, no aggregate coalescing — flat
            // per-row dispatch UPSERTs into legacy_mirror.<table>.
            "HT_Cupon" => Box::new(CuponMirrorMapper),
            "HT_CheckIn_Product" => Box::new(CheckinProductMirrorMapper),
            "HT_Deposit" => Box::new(DepositMirrorMapper),
            "HT_Changed_Room" => Box::new(ChangedRoomMirrorMapper),
            "HT_Bill_Debt_H" => Box::new(BillDebtHMirrorMapper),
            "HT_Bill_Debt_Ds" => Box::new(BillDebtDsMirrorMapper),
            // Track E1 — companion-guest projection (canonical) +
            // cancelled-room mirror (legacy_mirror pass-through).
            "HT_CheckIn_Other_People" => Box::new(GuestRegistryMapper),
            "HT_Rooms_Cancel" => Box::new(RoomsCancelMirrorMapper),
            // Phase 5/E2 — pre-booked products mirror (legacy_mirror
            // pass-through; coexistence audit 2026-06-11 P2).
            "HT_Book_Pro" => Box::new(BookProMirrorMapper),
            other => Box::new(NoopMapper { table_name: other }),
        };
        out.push(mapper);
    }
    out
}

/// Process one watcher tick. Per-mapper failures are logged but don't
/// abort the tick — one bad table never blocks the others.
///
/// `per_table_watermark` selects between the legacy single-row
/// `legacy_ct_state` (false, default) and the per-table
/// `legacy_ct_state_per_table` (true, Resilience PR R3). Per-table
/// mode lets a row-lock wedge on one table freeze only that row
/// rather than gating every CT-enabled table's advance.
#[allow(clippy::too_many_arguments)]
async fn run_one_tick(
    pg: &PgPool,
    mssql: &DbPool,
    mappers: &[Box<dyn MssqlChangeMapper>],
    slack: &Option<SlackClient>,
    shadow_mode: bool,
    per_table_watermark: bool,
    retention_last_checked: &mut HashMap<String, Instant>,
    retention_check_interval: Duration,
    site_id: &str,
) {
    // Two paths converge on a `HashMap<&str, i64>` of per-table
    // resume points. Global path: every table sees the same single
    // `last_seen_version`. Per-table path: each table reads its own
    // row from `legacy_ct_state_per_table` (default 0 for an
    // unseeded table — same semantics as a fresh global install).
    let per_table_resume: HashMap<String, i64> = if per_table_watermark {
        match hotel_backend::sync::watermark::read_per_table(pg).await {
            Ok(m) => m,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    "Failed to read per-table CT watermarks; skipping tick"
                );
                return;
            }
        }
    } else {
        HashMap::new()
    };

    let global_last_seen = match hotel_backend::sync::watermark::read_last_seen(pg).await {
        Ok(v) => v,
        Err(err) => {
            tracing::error!(
                event_name = EV_WATERMARK_READ_FAIL,
                site = %site_id,
                error = %err,
                "Failed to read CT watermark; skipping tick"
            );
            return;
        }
    };

    // Connectivity probe — when the legacy WG tunnel flaps the bb8
    // pool's 15s connection_timeout fires once per CT-enabled table,
    // turning a 2-min outage into a ~4-min (16×15s) sequential WARN
    // sweep. Probing once up front lets us bail the entire tick on a
    // single failure; the next tick (1s later) re-probes and resumes
    // the moment the tunnel comes back. See `probe_legacy_connectivity`
    // doc for the burn-in evidence.
    if let Err(err) = probe_legacy_connectivity(mssql).await {
        tracing::warn!(
            event_name = EV_LEGACY_PROBE_FAIL,
            site = %site_id,
            error = %err,
            "Legacy MSSQL probe failed; skipping tick"
        );
        return;
    }

    let now = Instant::now();
    for mapper in mappers {
        let table = mapper.table();
        let pk_cols = mapper.primary_key_cols();
        let select_sql = mapper.select_sql();

        // Resolve this table's resume point. Per-table mode looks up
        // the row from `legacy_ct_state_per_table` (default 0 if
        // unseeded). Global mode passes the shared `last_seen`.
        let table_last_seen = if per_table_watermark {
            per_table_resume.get(table).copied().unwrap_or(0)
        } else {
            global_last_seen
        };

        // Gate the retention guard to once per
        // `retention_check_interval` per table. The first tick (no
        // recorded timestamp) always checks; subsequent ticks within
        // the window skip the MSSQL round-trip entirely, slashing pool
        // pressure on the hot mappers.
        let should_check_retention = retention_last_checked
            .get(table)
            .map(|last| now.duration_since(*last) >= retention_check_interval)
            .unwrap_or(true);
        if should_check_retention {
            retention_last_checked.insert(table.to_string(), now);
        }

        // Run each table inside its own future; panics are isolated
        // via `tokio::spawn` further down for the per-row dispatch.
        if let Err(err) = poll_table(
            pg,
            mssql,
            slack,
            mapper.as_ref(),
            table,
            pk_cols,
            select_sql,
            table_last_seen,
            shadow_mode,
            per_table_watermark,
            should_check_retention,
            site_id,
        )
        .await
        {
            // Top-level fallback — `poll_table` already records granular
            // event_names for failures it can attribute to a specific
            // stage. A bubble-up to here is rare (only the panic-free
            // `Result` shape escaped the inner fn) but we still attach
            // an event_name so log filters never see a "naked" error.
            tracing::error!(
                event_name = EV_MAPPER_APPLY_FAIL,
                site = %site_id,
                table,
                error = %err,
                "poll_table failed"
            );
            let _ = record_table_error(pg, table, EV_MAPPER_APPLY_FAIL, &err.to_string()).await;
            if per_table_watermark {
                // R3 mirror: keep the per-table sibling row in sync so a
                // per-table watchdog can age the `last_polled_at` on
                // this specific table. Prefix with the R1 event_name so
                // operators grepping per-table errors get the same
                // taxonomy as `legacy_sync_status.last_error`.
                let payload = format!("[{EV_MAPPER_APPLY_FAIL}] {err}");
                let _ = hotel_backend::sync::watermark::record_per_table_error(
                    pg, table, &payload,
                )
                .await;
            }
        }
    }

    // Round-bill (cashier-session) sync — read-only poll of the legacy
    // `HT_Round_Bill` ledger into canonical `ht_shifts`. This is
    // deliberately OUTSIDE the CT-mapper loop: `HT_Round_Bill` has no
    // Change Tracking (and adding it would be legacy DDL we don't own —
    // see CLAUDE.md), so it can't ride the watermark path. It runs LAST
    // because the CT mappers are the load-bearing sync; a round-bill
    // failure must never abort the tick (`sync_round_bills` logs at WARN
    // and returns instead of propagating). Shadow mode skips the
    // canonical write, mirroring the CT mappers.
    sync_round_bills(pg, mssql, shadow_mode, site_id).await;

    // Guest-image sync-IN — read-only poll of the legacy `Tb_Save_Image`
    // reconstructed Thai-ID card / face-photo blobs into canonical
    // `ht_guest_documents` (doc_source='legacy'), so the registration form
    // serves the SAME image iHOTEL prints without the API ever touching
    // legacy. Same read-only per-tick poll spirit as `sync_round_bills`
    // (NOT Change-Tracking — no legacy DDL): runs LAST, logs at WARN and
    // returns on any error so an image failure never aborts the tick, and
    // skips the canonical write in shadow mode. Read-only from legacy, so it
    // defaults ENABLED (kill-switch `GUEST_DOC_SYNC_ENABLED=false`/`0`).
    sync_guest_documents(pg, mssql, shadow_mode, site_id).await;

    // Cash in/out petty-cash ledger (รายรับ-รายจ่าย) sync — same read-only
    // per-tick poll pattern as `sync_round_bills` (low-volume legacy ledger
    // + config trees, NOT Change-Tracking, so no legacy DDL). Mirrors
    // `TB_Pay_History` → `ht_cash_ledger` and the account taxonomy
    // (`TB_SET_MyType2`/`_2_2`/`3`) → `ht_cash_categories`. Both functions
    // log at WARN and return on any error — a failure here must never abort
    // the tick, and shadow mode skips the canonical writes.
    sync_cash_categories(pg, mssql, shadow_mode, site_id).await;
    sync_cash_history(pg, mssql, shadow_mode, site_id).await;

    // Room & staff sticky-notes sync (task #47) — read-only poll of the legacy
    // `HT_Room_SMS` + `HT_EMP_SMS` SMS tables into canonical `ht_notes`. Same
    // NON-CT poll pattern as the cash sync (the tables have no PK/CT
    // prerequisite, so no `migrations/legacy-mssql/` DDL). Captures both new
    // notes AND read-flag (`SMS_Readed`) flips made in iHOTEL. Logs at WARN and
    // returns on any error; shadow mode skips the canonical write.
    sync_sticky_notes(pg, mssql, shadow_mode, site_id).await;
}

/// Poll one table for CT changes since `last_seen`. Per the lifecycle:
/// retention check → SELECT CT changes → for each row, dispatch to
/// mapper → INSERT event_log → bump counters → advance watermark.
///
/// `should_check_retention` is throttled by the caller to
/// `LEGACY_SYNC_RETENTION_CHECK_INTERVAL_SECS` (default 300s) per
/// table. When false the retention round-trip to MSSQL is skipped
/// entirely — the safety net runs at most once every 5 min, which is
/// well within the operator-driven recovery window for a >48h
/// retention overflow.
#[allow(clippy::too_many_arguments)]
async fn poll_table(
    pg: &PgPool,
    mssql: &DbPool,
    slack: &Option<SlackClient>,
    mapper: &dyn MssqlChangeMapper,
    table: &str,
    pk_cols: &[&str],
    select_sql: &str,
    last_seen: i64,
    shadow_mode: bool,
    per_table_watermark: bool,
    should_check_retention: bool,
    site_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Retention guard (throttled — see fn doc comment).
    if should_check_retention {
        if let Err(err) = check_retention(mssql, table, last_seen).await {
            // `check_retention` returns a String that already encodes the
            // failure shape — both "retention overflow" (a hard stop the
            // operator must address) and "round-trip failed" (transient).
            // Discriminate on the substring `check_retention` itself
            // uses; same predicate the Slack page below uses.
            let event_name = if err.contains("retention") {
                EV_CT_RETENTION_OVERFLOW
            } else {
                EV_RETENTION_CHECK_FAIL
            };
            tracing::error!(
                event_name,
                site = %site_id,
                table,
                error = %err,
                "Retention check failed"
            );
            let _ = record_table_error(pg, table, event_name, &err).await;
            if let Some(s) = slack {
                if err.contains("retention") {
                    let msg = SlackMessage::with_site_text(
                        site_id,
                        format!(
                            ":rotating_light: *CT retention overflow* :rotating_light:\n\
                             Table: `{table}`\n\
                             Watermark fell behind CT retention; \
                             row history beyond `MIN_VALID_VERSION` is gone.\n\
                             _Recover with_ `bin/sync --bootstrap` _(Phase 5.5)_."
                        ),
                    );
                    let _ = s.send_message(&msg).await;
                }
            }
            return Ok(()); // intentional skip — retention can't be repaired by retry
        }
    }

    // 2. NoopMapper short-circuit — when select_sql is empty there's
    //    nothing to project, so just count rows for observability.
    if select_sql.is_empty() {
        let row_count = match count_ct_rows(mssql, table, last_seen).await {
            Ok(n) => n,
            Err(err) => {
                tracing::warn!(
                    event_name = EV_CT_COUNT_FAIL,
                    table,
                    error = %err,
                    "CT count query failed"
                );
                let _ = record_table_error(pg, table, EV_CT_COUNT_FAIL, &err).await;
                return Ok(());
            }
        };
        if let Err(err) = bump_skipped(pg, table, row_count, false).await {
            tracing::warn!(
                event_name = EV_STATUS_UPDATE_FAIL,
                table,
                error = %err,
                "Failed to update legacy_sync_status — observability degraded"
            );
        } else if row_count > 0 {
            tracing::info!(
                table,
                row_count,
                "CT rows observed (NoopMapper — skipped, awaiting real mapper)"
            );
        }
        return Ok(());
    }

    // 3. Real-mapper path: fetch CT rows joined with the table.
    let rows = match fetch_ct_rows(mssql, table, pk_cols, select_sql, last_seen).await {
        Ok(rs) => rs,
        Err(err) => {
            tracing::warn!(
                event_name = EV_CT_FETCH_FAIL,
                table,
                error = %err,
                "CT fetch failed"
            );
            let _ = record_table_error(pg, table, EV_CT_FETCH_FAIL, &err).await;
            return Ok(());
        }
    };

    if rows.is_empty() {
        // Empty-fetch success: no CT changes since `last_seen`. Bump the
        // skipped/processed counters with 0 so `last_processed_at` ticks
        // forward and any prior `last_error` / `consecutive_failures`
        // accumulated from a transient bb8 timeout get cleared. Without
        // this, low-traffic / empty tables (HF Ville's HT_Cupon,
        // HT_Deposit, HT_Bill_Debt_*, HT_Receipt_H — all 0-row or
        // CT-history-empty on Ville) would stay permanently stuck in
        // `legacy_sync_status` showing "Timed out in bb8" with high
        // `consecutive_failures` even though their fetches now succeed,
        // because the counter-clearing path was only reachable via a
        // non-empty `bump_counters` call. Mirrors what the NoopMapper
        // short-circuit at the top of this function already does on a
        // 0-row count. Fixes the 5-stuck-table observability bug
        // (v2.58.3, fix/hfville-stuck-ct-tables).
        let _ = bump_skipped(pg, table, 0, false).await;
        if per_table_watermark {
            // R3 — touch `last_polled_at` so the per-table watchdog
            // can distinguish "healthy but quiet" from "wedged" by
            // comparing now() - last_polled_at across rows.
            let _ = hotel_backend::sync::watermark::touch_per_table(pg, table).await;
        }
        return Ok(());
    }

    let row_count = rows.len() as i64;
    let mut max_version: i64 = last_seen;
    let mut ingested: i64 = 0;
    let mut skipped: i64 = 0;
    // Tracks whether ANY per-row / aggregate path errored this tick.
    // The end-of-tick counter bumps must NOT clear last_error /
    // consecutive_failures when this is set, otherwise per-row
    // increments from `record_table_error` get wiped before the
    // dashboard can see them — exactly the silent-failure mode that
    // hid the Cin_Pay_Status schema drift through a 16h soak.
    let mut errored = false;

    // 4. Open one PG TX per table-tick. Shadow mode rolls back; live
    //    mode commits.
    let mut tx = match pg.begin().await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(
                event_name = EV_PG_TX_BEGIN_FAIL,
                table,
                error = %err,
                "Failed to begin PG TX"
            );
            let _ = record_table_error(pg, table, EV_PG_TX_BEGIN_FAIL, &err.to_string()).await;
            return Ok(());
        }
    };

    // 4a. Aggregate-coalesced path (Phase 5.3): when the mapper opts
    //     in via `coalesce_key`, group rows by the aggregate root and
    //     dispatch each unique key exactly once per tick. Currently
    //     only the booking mappers (HT_Book_H / HT_Book_Ds /
    //     HT_Book_Date) opt in — see `sync::mappers::booking`.
    //
    //     The dispatch path branches on whether ANY row in the batch
    //     produced a Some-key. If so, the entire batch goes through
    //     the coalesced path; rows without a key (e.g. D rows on
    //     child tables where the parent FK isn't projected) are
    //     skipped with a debug log — a sibling header / line CT row
    //     in the same tick almost always covers the same booking and
    //     drives the canonical re-load.
    let any_coalesce_key = rows
        .iter()
        .any(|(_, _, r)| mapper.coalesce_key(r).is_some());

    // Track E1 / T2 HIGH-6 — for `HT_CheckIn_Ds` we MUST use the
    // aggregate path even when every row is a D-only event (no
    // sibling I/U row to surface the parent Cin_No). The orphan-
    // recovery branch below back-queries `ht_checkins` to find the
    // parent. Per-row dispatch would route to `CheckInRoomsMapper::
    // apply` which intentionally returns `Ok(None)` (coalesced
    // semantics) — leaving the canonical aggregate stale forever.
    let force_coalesce_for_orphan_recovery = table == "HT_CheckIn_Ds" && !rows.is_empty();

    if any_coalesce_key || force_coalesce_for_orphan_recovery {
        for (version, op_char, _row) in &rows {
            // We still parse the op code to surface unknown operations
            // loudly (matches the per-row path's behaviour). Beyond
            // that, the op itself is informational for the aggregate
            // path — the parent re-load supersedes per-row I/U/D
            // semantics.
            if let Err(err) = ChangeOp::try_from(op_char.as_str()) {
                tracing::warn!(
                    event_name = EV_UNKNOWN_CT_OP,
                    table,
                    sys_change_operation = %op_char,
                    error = %err,
                    "Unknown CT operation code — skipping row"
                );
                skipped += 1;
                continue;
            }
            if *version > max_version {
                max_version = *version;
            }
        }

        // Group all rows by aggregate root key (de-dup by HashSet).
        let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (_v, _op, row) in &rows {
            if let Some(k) = mapper.coalesce_key(row) {
                keys.insert(k);
            }
        }

        // Track E1 / T2 HIGH-6 — D-event orphan recovery for
        // `HT_CheckIn_Ds`. The CT projection on a D row nulls every
        // `t.<col>` (LEFT JOIN), so `coalesce_key` (which reads
        // `Cin_No`) returns None. If there's no sibling header CT row
        // in the same tick, the parent `Cin_no` is never collected
        // and the canonical row stays stale forever — a pure D-only
        // batch (rare under iHOTEL's normal flow but possible after a
        // cancel-detail-row-only edit) gets silently dropped.
        //
        // Recover by back-querying `ht_checkins.legacy_checkin_ds_id`
        // for each PK in the batch that produced no key. The PG row
        // is the canonical record of which `HT_CheckIn_Ds.id` belongs
        // to which `Cin_no`, written during the original Insert sync.
        // If the lookup misses (mirror never had the row), log a WARN
        // and let the next CT tick on the parent header handle it.
        if table == "HT_CheckIn_Ds" {
            for (_v, op_char, row) in &rows {
                if mapper.coalesce_key(row).is_some() {
                    continue; // already collected via I/U row
                }
                // Only D rows can produce None (per CheckInRoomsMapper
                // contract); other op codes were already warned about
                // upstream.
                if op_char != "D" {
                    continue;
                }
                let Some(ds_id) = row.try_get_i32("id").ok().flatten() else {
                    tracing::warn!(
                        event_name = EV_ORPHAN_RECOVERY_FAIL,
                        table,
                        reason = "missing_pk_alias",
                        "D-event row missing `id` PK alias — cannot recover \
                         parent Cin_no; canonical row may stay stale"
                    );
                    continue;
                };
                match sqlx::query_scalar::<_, Option<String>>(
                    "SELECT legacy_cin_no FROM ht_checkins \
                       WHERE legacy_checkin_ds_id = $1 LIMIT 1",
                )
                .bind(ds_id)
                .fetch_optional(&mut *tx)
                .await
                {
                    Ok(Some(Some(cin_no))) => {
                        tracing::debug!(
                            table,
                            ds_id,
                            cin_no = %cin_no,
                            "D-event orphan recovery: resolved parent Cin_no \
                             via ht_checkins.legacy_checkin_ds_id"
                        );
                        keys.insert(cin_no);
                    }
                    Ok(_) => {
                        tracing::warn!(
                            event_name = EV_ORPHAN_RECOVERY_FAIL,
                            table,
                            ds_id,
                            reason = "no_matching_pg_row",
                            "D-event orphan recovery FAILED: no ht_checkins \
                             row carries legacy_checkin_ds_id={ds_id}; \
                             canonical aggregate may stay stale until next \
                             CT tick on the parent header",
                        );
                    }
                    Err(err) => {
                        // 2026-06-11 adversarial review: a TRANSIENT PG
                        // error here must hold the watermark — otherwise
                        // a momentary hiccup during the recovery lookup
                        // consumes the D row permanently (the exact
                        // silent-drop class the per-key `errored` gating
                        // exists to prevent). Contrast `no_matching_pg_row`
                        // above: there the canonical row genuinely doesn't
                        // exist, so a retry can never learn more and
                        // warn-skip is correct.
                        tracing::warn!(
                            event_name = EV_ORPHAN_RECOVERY_FAIL,
                            table,
                            ds_id,
                            reason = "lookup_query_errored",
                            error = %err,
                            "D-event orphan recovery query errored — holding \
                             watermark for retry"
                        );
                        errored = true;
                    }
                }
            }
        }

        for key in &keys {
            // Route the coalesced apply by table. Three aggregate
            // shapes ship today:
            //
            // * booking_*  → load_booking_aggregate + apply_booking_aggregate
            // * checkin_*  → load_checkin_aggregate + apply_checkin_aggregate
            // * payment_*  → apply_payment_aggregate (loads internally)
            let result = match table {
                "HT_Book_H" | "HT_Book_Ds" | "HT_Book_Date" => {
                    match load_booking_aggregate(mssql, key).await {
                        Ok(a) => apply_booking_aggregate(&mut tx, Some(mssql), &a, key).await,
                        Err(err) => {
                            tracing::warn!(
                                event_name = EV_LOAD_AGGREGATE_FAIL,
                                table,
                                aggregate = "booking",
                                key = %key,
                                error = %err,
                                "Failed to load booking aggregate; recording and continuing"
                            );
                            let _ = record_table_error(
                                pg,
                                table,
                                EV_LOAD_AGGREGATE_FAIL,
                                &err.to_string(),
                            )
                            .await;
                            errored = true;
                            skipped += 1;
                            continue;
                        }
                    }
                }
                "HT_CheckIn_H" | "HT_CheckIn_Ds" => {
                    match load_checkin_aggregate(mssql, key).await {
                        Ok(a) => apply_checkin_aggregate(&mut tx, Some(mssql), &a, key).await,
                        Err(err) => {
                            tracing::warn!(
                                event_name = EV_LOAD_AGGREGATE_FAIL,
                                table,
                                aggregate = "checkin",
                                key = %key,
                                error = %err,
                                "Failed to load checkin aggregate; recording and continuing"
                            );
                            let _ = record_table_error(
                                pg,
                                table,
                                EV_LOAD_AGGREGATE_FAIL,
                                &err.to_string(),
                            )
                            .await;
                            errored = true;
                            skipped += 1;
                            continue;
                        }
                    }
                }
                "HT_CheckIn_Pay" => apply_payment_aggregate(&mut tx, mssql, key).await,
                other => {
                    tracing::warn!(
                        event_name = EV_AGGREGATE_APPLY_FAIL,
                        table = other,
                        reason = "unknown_aggregate_table",
                        "Unknown coalesced aggregate table — skipping"
                    );
                    skipped += 1;
                    continue;
                }
            };

            match result {
                Ok(Some(event)) => {
                    if shadow_mode {
                        tracing::info!(
                            table,
                            key = %key,
                            event_type = event.type_name(),
                            "would publish (shadow mode)"
                        );
                        skipped += 1;
                    } else if let Err(err) = persist_event(&mut tx, &event).await {
                        tracing::error!(
                            event_name = EV_PERSIST_EVENT_FAIL,
                            table,
                            key = %key,
                            error = %err,
                            "Failed to persist event_log row"
                        );
                        skipped += 1;
                        // Same silent-drop class as a failed apply: the
                        // canonical row is already in `tx` and will commit,
                        // but the LISTEN/NOTIFY event never fires. Holding
                        // the watermark lets the next tick re-run the
                        // aggregate (idempotent — returns Ok(None) on
                        // already-applied state) and re-attempt persist_event.
                        errored = true;
                    } else {
                        ingested += 1;
                    }
                }
                Ok(None) => {
                    // Idempotent skip — canonical row already matches.
                    // Since 2026-06-11 mappers MUST NOT return Ok(None)
                    // for an unresolved FK (they eager-mirror or Err so
                    // the watermark holds) — `skipped` therefore counts
                    // only genuine no-ops, never deferred rows. See
                    // sync::resolve module doc (June-3 incident).
                    skipped += 1;
                }
                Err(err) => {
                    tracing::warn!(
                        event_name = EV_AGGREGATE_APPLY_FAIL,
                        table,
                        key = %key,
                        error = %err,
                        "aggregate apply error — recording and continuing"
                    );
                    let _ = record_table_error(
                        pg,
                        table,
                        EV_AGGREGATE_APPLY_FAIL,
                        &err.to_string(),
                    )
                    .await;
                    errored = true;
                    skipped += 1;
                }
            }
        }
    } else {
        // 4b. Legacy per-row dispatch path (Phase 5.2). Customer / room /
        //     room_status mappers stay here.
        for (version, op_char, row) in &rows {
            let op = match ChangeOp::try_from(op_char.as_str()) {
                Ok(o) => o,
                Err(err) => {
                    tracing::warn!(
                        event_name = EV_UNKNOWN_CT_OP,
                        table,
                        sys_change_operation = %op_char,
                        error = %err,
                        "Unknown CT operation code — skipping row"
                    );
                    skipped += 1;
                    continue;
                }
            };

            // For Delete, the joined row is NULL but the PK columns are
            // still in the projection (CT carries them). Pass `Some(&row)`
            // either way and let the mapper decide.
            let result = mapper.apply(&mut tx, op, Some(row)).await;

            match result {
                Ok(Some(event)) => {
                    if shadow_mode {
                        tracing::info!(
                            table,
                            version,
                            op = ?op,
                            event_type = event.type_name(),
                            "would publish (shadow mode)"
                        );
                        skipped += 1;
                    } else if let Err(err) = persist_event(&mut tx, &event).await {
                        tracing::error!(
                            event_name = EV_PERSIST_EVENT_FAIL,
                            table,
                            version,
                            op = ?op,
                            error = %err,
                            "Failed to persist event_log row"
                        );
                        skipped += 1;
                        // See sibling path: persist_event failure is a
                        // silent-drop class (canonical commits, event never
                        // fires). Hold watermark to retry next tick.
                        errored = true;
                    } else {
                        ingested += 1;
                    }
                }
                Ok(None) => {
                    // Idempotent skip / D-event with no event payload.
                    // Same 2026-06-11 contract as the aggregate path:
                    // mappers Err on unresolved FKs, so this never
                    // counts a silently-deferred row.
                    skipped += 1;
                }
                Err(err) => {
                    tracing::warn!(
                        event_name = EV_MAPPER_APPLY_FAIL,
                        table,
                        version,
                        op = ?op,
                        error = %err,
                        "Mapper error — recording and continuing"
                    );
                    let _ = record_table_error(
                        pg,
                        table,
                        EV_MAPPER_APPLY_FAIL,
                        &err.to_string(),
                    )
                    .await;
                    errored = true;
                    skipped += 1;
                }
            }

            if *version > max_version {
                max_version = *version;
            }
        }
    }

    // 5. Commit (live) or rollback (shadow).
    if shadow_mode {
        if let Err(err) = tx.rollback().await {
            tracing::warn!(
                event_name = EV_SHADOW_ROLLBACK_FAIL,
                table,
                error = %err,
                "Shadow-mode rollback failed"
            );
        }
        // Bump skipped counter to mirror the noop path's behavior.
        let _ = bump_skipped(pg, table, row_count, errored).await;
        return Ok(());
    }

    if let Err(err) = tx.commit().await {
        tracing::error!(
            event_name = EV_PG_TX_COMMIT_FAIL,
            table,
            error = %err,
            "PG TX commit failed"
        );
        let _ = record_table_error(pg, table, EV_PG_TX_COMMIT_FAIL, &err.to_string()).await;
        return Ok(());
    }

    // 6. Counters + watermark advance (live mode only).
    if let Err(err) = bump_counters(pg, table, ingested, skipped, errored).await {
        tracing::warn!(
            event_name = EV_STATUS_UPDATE_FAIL,
            table,
            error = %err,
            "Failed to bump counters"
        );
    }

    // Gate the watermark advance on whether ANY per-key apply failed
    // this tick. When `errored` is true, hold at `last_seen` so the
    // next tick re-fetches the same CT rows — UPSERT semantics make
    // already-succeeded keys idempotent on retry. Closes the
    // silent-drop bug where a transient per-key failure (deploy
    // mid-tick, MSSQL hiccup) advanced the watermark past the failed
    // key's CT version, losing the event after CT's 2-day retention.
    if errored {
        tracing::warn!(
            table,
            last_seen,
            max_version,
            applied = ingested,
            skipped,
            per_table = per_table_watermark,
            "[CT] Tick had per-key failures — holding watermark at last_seen for retry next tick"
        );
        // Still touch `last_polled_at` in per-table mode so the
        // watchdog distinguishes "actively retrying" from "wedged".
        if per_table_watermark {
            let _ = hotel_backend::sync::watermark::touch_per_table(pg, table).await;
        }
    } else if let Some(target_version) = next_watermark_after_tick(max_version, last_seen, errored)
    {
        // R3 — feature-flagged dual-write contract. Per-table mode
        // advances ONLY the per-table row so a stuck sibling
        // doesn't pin the global down; global mode advances ONLY
        // the single-row state, preserving the pre-R3 behaviour.
        let advance_result = if per_table_watermark {
            hotel_backend::sync::watermark::advance_per_table(pg, table, target_version).await
        } else {
            hotel_backend::sync::watermark::advance(pg, target_version).await
        };
        match advance_result {
            Err(err) => {
                // R1: structured event + persisted failure mode so the
                // 2026-05-14 symptom (UPDATE failure post-commit, no
                // PG-side breadcrumb) survives a container restart.
                tracing::error!(
                    event_name = EV_WATERMARK_ADVANCE_FAIL,
                    table,
                    new_version = target_version,
                    per_table = per_table_watermark,
                    error = %err,
                    "Failed to advance CT watermark"
                );
                // Persist into legacy_sync_status (global single row) in
                // its OWN sqlx auto-TX — the canonical state UPDATE
                // already committed above is unaffected.
                let _ = record_table_error(
                    pg,
                    table,
                    EV_WATERMARK_ADVANCE_FAIL,
                    &err.to_string(),
                )
                .await;
                // R3 mirror: when per-table mode is active, also persist
                // into the per-table row so a per-table watchdog can
                // attribute the wedge to this specific table.
                if per_table_watermark {
                    let payload = format!("[{EV_WATERMARK_ADVANCE_FAIL}] {err}");
                    let _ = hotel_backend::sync::watermark::record_per_table_error(
                        pg, table, &payload,
                    )
                    .await;
                }
            }
            Ok(()) => {
                tracing::info!(
                    table,
                    from = last_seen,
                    to = target_version,
                    ingested,
                    skipped,
                    per_table = per_table_watermark,
                    "Advanced CT watermark"
                );
            }
        }
    } else if per_table_watermark {
        // Live tick with no new CT version (rows were all stale /
        // coalesced away). Still touch `last_polled_at` so the
        // per-table watchdog doesn't flag the row as wedged.
        let _ = hotel_backend::sync::watermark::touch_per_table(pg, table).await;
    }

    Ok(())
}

/// One CT row returned by [`fetch_ct_rows`]: the version, the
/// single-character op code, and the joined data wrapped in our test
/// fixture (which trivially backs onto `tiberius::Row`).
type CtRow = (
    i64,
    String,
    hotel_backend::sync::row::test_support::HashMapRow,
);
// Note: above type alias intentionally points at HashMapRow only because
// that's the test impl; we re-shape tiberius rows into the same
// HashMap-backed type below so the dispatch path works against
// `MappableRow` uniformly. This keeps a single code path for both
// production and tests, at the cost of one extra copy per row (small
// — column counts are <16, all cells are short strings or i32s).

#[cfg(test)]
mod sync_row_alias_compile_check {
    // Compile-time guard that the alias above stays in sync with the
    // public `test_support` re-export the watcher binary depends on.
    use super::CtRow;
    fn _assert_send(_v: CtRow) {}
}

/// Build the CT polling query.
///
/// ## No `SYS_CHANGE_CONTEXT` filter — deliberately (2026-06-11)
///
/// Until 2026-06-11 both this query and [`build_ct_count_sql`] carried
/// `WHERE ct.SYS_CHANGE_CONTEXT IS NULL OR ct.SYS_CHANGE_CONTEXT <>
/// 0x4E48`, believed to filter out echoes of our own writeback (which
/// issues `SET CONTEXT_INFO 0x4E48` per session). That predicate was
/// INERT: `SET CONTEXT_INFO` never populates `SYS_CHANGE_CONTEXT` —
/// only the per-statement `WITH CHANGE_TRACKING_CONTEXT (...)` table
/// hint does, and nothing in the codebase uses it. Every writeback CT
/// row arrived with `SYS_CHANGE_CONTEXT IS NULL` and sailed through;
/// echo absorption has always come from the mappers' idempotent
/// UPSERTs (re-applying our own write converges to a no-op and emits
/// no event).
///
/// Do NOT "fix" this by adding `WITH CHANGE_TRACKING_CONTEXT` to the
/// writeback and re-instating the filter: CT coalesces per-PK to the
/// LATEST change's context, so a genuine iHOTEL edit racing a writeback
/// touch on the same row would inherit our tag and be filtered out —
/// recreating the June-3 silent-loss class at the SQL layer.
fn build_ct_changes_sql(
    table: &str,
    pk_cols: &[&str],
    select_sql: &str,
    last_seen: i64,
) -> Result<String, String> {
    // Build the JOIN condition: `t.pk1 = ct.pk1 AND t.pk2 = ct.pk2 …`.
    // For a single-PK table (our 5.2 mappers) this is trivial.
    let join_clause = if pk_cols.is_empty() {
        // Fallback that should never happen for a real mapper; the
        // NoopMapper short-circuit upstream owns that path.
        return Err("real mapper must declare primary_key_cols".into());
    } else {
        pk_cols
            .iter()
            .map(|c| format!("t.{c} = ct.{c}"))
            .collect::<Vec<_>>()
            .join(" AND ")
    };

    // Build the PK projection list for the SELECT (we always include
    // the PK columns from CT itself so D rows still carry them even
    // when `t.*` is NULL).
    let pk_projection = pk_cols
        .iter()
        .map(|c| format!("ct.{c} AS pk_{c}"))
        .collect::<Vec<_>>()
        .join(", ");

    Ok(format!(
        "SELECT ct.SYS_CHANGE_VERSION AS sys_change_version, \
                ct.SYS_CHANGE_OPERATION AS sys_change_operation, \
                {pk_projection}, \
                {select_sql} \
           FROM CHANGETABLE(CHANGES {table}, {last_seen}) AS ct \
           LEFT JOIN {table} AS t ON {join_clause} \
          ORDER BY ct.SYS_CHANGE_VERSION ASC"
    ))
}

/// Companion COUNT query for [`build_ct_changes_sql`] — same CHANGETABLE
/// window, no filter (see that function's doc for why the historical
/// `SYS_CHANGE_CONTEXT` predicate was removed).
fn build_ct_count_sql(table: &str, last_seen: i64) -> String {
    format!("SELECT COUNT(*) FROM CHANGETABLE(CHANGES {table}, {last_seen}) AS ct")
}

/// Fetch CT rows joined with the table, ordered by `SYS_CHANGE_VERSION`
/// for monotonic processing. Echo absorption for our own writeback's CT
/// rows happens in the idempotent mappers, NOT here — see
/// [`build_ct_changes_sql`].
async fn fetch_ct_rows(
    mssql: &DbPool,
    table: &str,
    pk_cols: &[&str],
    select_sql: &str,
    last_seen: i64,
) -> Result<Vec<CtRow>, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;

    let sql = build_ct_changes_sql(table, pk_cols, select_sql, last_seen)?;

    let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read)
        .await
        .map_err(|e| e.to_string())?;

    let mut out: Vec<CtRow> = Vec::with_capacity(rows.len());
    for r in rows {
        let version: i64 = r.get("sys_change_version").unwrap_or(0);
        // SYS_CHANGE_OPERATION is always one of 'I' / 'U' / 'D' (single
        // char); tiberius surfaces it as `&str`.
        let op_char: String = r
            .get::<&str, _>("sys_change_operation")
            .unwrap_or("?")
            .to_string();
        out.push((version, op_char, materialise_row(&r, pk_cols, select_sql)));
    }
    Ok(out)
}

/// Copy a tiberius row's columns into a `HashMapRow` so the rest of the
/// dispatch path can use the `MappableRow` trait uniformly. Strictly a
/// boundary translator — both sides of the column list (PK + projection)
/// are addressed by the original column names the mapper requested.
fn materialise_row(
    row: &tiberius::Row,
    pk_cols: &[&str],
    select_sql: &str,
) -> hotel_backend::sync::row::test_support::HashMapRow {
    let projection: Vec<String> = extract_projection_columns(select_sql);
    build_materialised_row(pk_cols, &projection, |col| read_cell(row, col))
}

/// Inner pure helper extracted from [`materialise_row`] so the loop
/// ordering — which is correctness-critical on D rows — is unit-
/// testable without constructing a `tiberius::Row`.
///
/// **Loop ordering rationale.** The PK columns can appear in BOTH
/// `pk_cols` AND the projection (e.g. mirror mappers like
/// `CuponMirrorMapper` declare `cupon_no` as PK *and* project
/// `t.cupon_no`). On D rows the LEFT JOIN nulls every `t.<col>`
/// projection while CT's `pk_<col>` aliases stay populated. If the PK
/// loop runs first and the projection loop runs second, the
/// projection's `MockValue::Null` overwrite silently clobbers the
/// real PK and the mapper crashes with "PK NULL — should not happen"
/// on every D event. Running the projection FIRST and the PK loop
/// LAST lets the PK overwrite the projection's NULL on D rows
/// without affecting I/U semantics (where both reads agree).
fn build_materialised_row<F>(
    pk_cols: &[&str],
    projection_cols: &[String],
    mut read: F,
) -> hotel_backend::sync::row::test_support::HashMapRow
where
    F: FnMut(&str) -> Option<hotel_backend::sync::row::test_support::MockValue>,
{
    use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

    let mut h = HashMapRow::new("ct_row");

    // 1. Projection columns FIRST. The mapper specified them as
    //    `t.<col>` in select_sql; tiberius exposes them under just
    //    `<col>` in the result row. NULL gets recorded explicitly so
    //    the mapper sees `None` instead of "missing column".
    for col in projection_cols {
        match read(col) {
            Some(v) => h.cells.insert(col.clone(), v),
            None => h.cells.insert(col.clone(), MockValue::Null),
        };
    }

    // 2. PK columns LAST. Surfaced under both `pk_<name>` (the SELECT
    //    alias) and `<name>` (what the mapper looks up). For D rows
    //    the joined table column is NULL but `pk_<name>` is
    //    populated — running this AFTER projection lets the PK
    //    overwrite the NULL the projection loop just wrote.
    for col in pk_cols {
        let pk_alias = format!("pk_{col}");
        if let Some(v) = read(&pk_alias) {
            h.cells.insert((*col).to_string(), v);
        }
    }

    h
}

/// Pull `t.<column>` identifiers out of a select clause like
/// `"t.Cust_no, t.Cust_name, t.Cust_perfix"`. Tolerant of whitespace
/// and trailing commas; ignores anything that isn't `t.<ident>`.
fn extract_projection_columns(select_sql: &str) -> Vec<String> {
    select_sql
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            p.strip_prefix("t.").map(|s| s.trim().to_string())
        })
        .collect()
}

/// Read one cell from a tiberius row, choosing a wrapper based on the
/// type tiberius surfaces. Probes types in the order our 5.2 mappers
/// actually use (str → i32 → i64 → f64 → datetime). Returns `None` if
/// the cell is SQL NULL.
fn read_cell(
    row: &tiberius::Row,
    col: &str,
) -> Option<hotel_backend::sync::row::test_support::MockValue> {
    use hotel_backend::sync::row::test_support::MockValue;

    if let Ok(Some(s)) = tiberius::Row::try_get::<&str, _>(row, col) {
        return Some(MockValue::Str(s.to_string()));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i32, _>(row, col) {
        return Some(MockValue::I32(n));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i64, _>(row, col) {
        return Some(MockValue::I64(n));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<f64, _>(row, col) {
        return Some(MockValue::F64(n));
    }
    if let Ok(Some(d)) = tiberius::Row::try_get::<chrono::NaiveDateTime, _>(row, col) {
        return Some(MockValue::DateTime(d));
    }
    None
}

/// Convert a legacy MSSQL `datetime` — Bangkok wall-clock stored
/// without timezone info — to a true UTC instant for the canonical
/// `TIMESTAMPTZ` columns on `ht_shifts`.
///
/// Mirrors `sync::mappers::checkin::naive_dt_to_utc` and
/// `booking::naive_date_to_utc` (the 2026-06-11 audit-P2 convention):
/// canonical timestamptz columns hold the real instant, so Bangkok
/// 08:00 is stored as 01:00 UTC. `+07:00` is a fixed offset (no DST),
/// so `single()` always yields exactly one instant. Keeping the same
/// convention as the CT mappers means `/api/shifts/current` lines up
/// with the rest of the canonical timeline (and with native shifts
/// opened via `ShiftService`, which stamps `NOW()` on a timestamptz
/// column).
fn naive_thai_to_utc(dt: chrono::NaiveDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    let bangkok = chrono::FixedOffset::east_opt(7 * 3600).expect("+07:00 is a valid offset");
    bangkok
        .from_local_datetime(&dt)
        .single()
        .expect("fixed offsets have no DST gaps/folds")
        .with_timezone(&chrono::Utc)
}

/// Read-only sync of the legacy cashier-session ledger
/// (`HT_Round_Bill`) into the canonical `ht_shifts` table.
///
/// **Why**: co-existence (ADR 0002) means a receptionist may open or
/// close the cashier round in EITHER iHOTEL or our app. iHOTEL records
/// the round in `HT_Round_Bill`; our checkout/payment gate consults
/// canonical `ht_shifts`. Without this poll, a round opened in iHOTEL
/// would be invisible to our app, so our gate would wrongly report "no
/// open shift". This makes iHOTEL the source of truth that our
/// canonical state follows (read-only), so both apps agree on "is a
/// round open?".
///
/// **Shape**: every tick we SELECT the currently-open row
/// (`round_end IS NULL`) plus any row touched in the last 2 days, and
/// UPSERT each into `ht_shifts` keyed on `(shift_site_id, shift_no)`
/// where `shift_no` = the legacy app-allocated `id`. The 2-day window
/// keeps a just-closed round converging (its `round_end` lands on a
/// later tick) without scanning the full history every second.
///
/// **Ordering**: closed rows are applied before the (at most one) open
/// row. iHOTEL's invariant is "≤1 row with `round_end IS NULL`", and PG
/// enforces the same via the partial unique index
/// `ht_shifts_one_open_per_site`. Applying the prior round's closure
/// before the new round's open avoids transiently tripping that index.
///
/// **Resilience**: a failure here must NEVER abort the tick — every
/// error path logs at WARN and returns/continues. In shadow mode the
/// canonical UPSERT is skipped (read-and-log only).
async fn sync_round_bills(pg: &PgPool, mssql: &DbPool, shadow_mode: bool, site_id: &str) {
    // Closed rows first (CASE … = 0), the single open row last (= 1), so
    // a round-rollover tick lands the prior closure before the new open
    // and never trips the one-open-per-site partial unique index.
    const ROUND_BILL_SELECT: &str = "SELECT id, round_no, round_price, round_by, round_start, round_end \
         FROM HT_Round_Bill \
         WHERE round_end IS NULL OR round_start >= DATEADD(DAY, -2, GETDATE()) \
         ORDER BY CASE WHEN round_end IS NULL THEN 1 ELSE 0 END, id";

    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!(
                event_name = "round_bill_sync_conn_fail",
                site = %site_id,
                error = %err,
                "round-bill sync: could not acquire MSSQL connection; skipping"
            );
            return;
        }
    };
    let rows = match simple_query_with_timeout_pooled(&mut conn, ROUND_BILL_SELECT, MssqlOpKind::Read)
        .await
    {
        Ok(r) => r,
        Err(err) => {
            tracing::warn!(
                event_name = "round_bill_sync_query_fail",
                site = %site_id,
                error = %err,
                "round-bill sync: SELECT failed; skipping"
            );
            return;
        }
    };
    drop(conn); // release the pooled MSSQL connection before PG work

    let mut upserted = 0usize;
    for row in &rows {
        let Some(legacy_id) = tiberius::Row::try_get::<i32, _>(row, "id").ok().flatten() else {
            tracing::warn!(
                event_name = "round_bill_sync_row_skip",
                site = %site_id,
                "round-bill sync: row missing id; skipping"
            );
            continue;
        };
        let Some(round_start) =
            tiberius::Row::try_get::<chrono::NaiveDateTime, _>(row, "round_start")
                .ok()
                .flatten()
        else {
            tracing::warn!(
                event_name = "round_bill_sync_row_skip",
                site = %site_id,
                legacy_id,
                "round-bill sync: row missing round_start; skipping"
            );
            continue;
        };
        let round_price = tiberius::Row::try_get::<f64, _>(row, "round_price")
            .ok()
            .flatten()
            .unwrap_or(0.0);
        let round_by = tiberius::Row::try_get::<&str, _>(row, "round_by")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let round_end = tiberius::Row::try_get::<chrono::NaiveDateTime, _>(row, "round_end")
            .ok()
            .flatten();

        let opened_at = naive_thai_to_utc(round_start);
        let closed_at = round_end.map(naive_thai_to_utc);
        // `round_by` names the cashier; on close iHOTEL overwrites it
        // with the closing employee. A single-row snapshot can't separate
        // opener from closer, so we use it for `shift_opened_by` and —
        // only when the round is closed — `shift_closed_by` too.
        let opened_by = round_by.clone().unwrap_or_else(|| "ihotel".to_string());
        let closed_by = if closed_at.is_some() { round_by } else { None };

        if shadow_mode {
            tracing::info!(
                event_name = "round_bill_sync_shadow",
                site = %site_id,
                legacy_id,
                open = closed_at.is_none(),
                "round-bill sync (shadow): would upsert canonical shift"
            );
            continue;
        }

        // Runtime `sqlx::query` (not the compile-time `query!` macro) so
        // this adds nothing to the `.sqlx/` offline cache. `$3::numeric`
        // mirrors `ShiftService::open_shift` (binds f64, server casts to
        // NUMERIC). Conflict target is the `(shift_site_id, shift_no)`
        // unique constraint — re-running a tick is idempotent.
        let res = sqlx::query(
            "INSERT INTO ht_shifts ( \
                 shift_site_id, shift_no, shift_opening_float, shift_opened_by, \
                 shift_opened_at, shift_closed_at, shift_closed_by, shift_legacy_round_id \
             ) VALUES ($1, $2, $3::numeric, $4, $5, $6, $7, $8) \
             ON CONFLICT (shift_site_id, shift_no) DO UPDATE SET \
                 shift_opening_float   = EXCLUDED.shift_opening_float, \
                 shift_opened_by       = EXCLUDED.shift_opened_by, \
                 shift_opened_at       = EXCLUDED.shift_opened_at, \
                 shift_closed_at       = EXCLUDED.shift_closed_at, \
                 shift_closed_by       = EXCLUDED.shift_closed_by, \
                 shift_legacy_round_id = EXCLUDED.shift_legacy_round_id",
        )
        .bind(site_id)
        .bind(legacy_id)
        .bind(round_price)
        .bind(&opened_by)
        .bind(opened_at)
        .bind(closed_at)
        .bind(&closed_by)
        .bind(legacy_id)
        .execute(pg)
        .await;

        match res {
            Ok(_) => upserted += 1,
            Err(err) => {
                // Most likely a partial-index trip during a rollover the
                // closure hasn't caught up on yet — self-heals next tick.
                tracing::warn!(
                    event_name = "round_bill_sync_upsert_fail",
                    site = %site_id,
                    legacy_id,
                    error = %err,
                    "round-bill sync: UPSERT into ht_shifts failed; continuing"
                );
            }
        }
    }

    if upserted > 0 {
        tracing::debug!(
            event_name = "round_bill_sync_ok",
            site = %site_id,
            upserted,
            "round-bill sync: upserted canonical shifts from HT_Round_Bill"
        );
    }
}

/// Guest-image sync-IN — read-only per-tick poll of the legacy `Tb_Save_Image`
/// blobs into canonical `ht_guest_documents` (`doc_source='legacy'`).
///
/// iHOTEL stores a reconstructed Thai-ID card image (`ttype 'บัตรประชาชน'`) and a
/// face photo (`'รูปลูกค้า'`) per check-in; the registration form must show the SAME
/// image iHOTEL prints. The API is forbidden from touching legacy (architecture
/// rule), so the sync worker mirrors the bytes into `ht_guest_documents` and the
/// existing API/form path serves them from there.
///
/// **Resilience**: identical spirit to `sync_round_bills` — every error path logs
/// at WARN and returns; shadow mode logs at INFO and skips the canonical write.
/// NOT a Change-Tracking mapper (low-volume, no legacy DDL), so it runs LAST in
/// the tick and a failure here never aborts the load-bearing CT sync.
///
/// **Idempotency**: the UPSERT conflict target is the partial UNIQUE index on
/// `doc_legacy_id` (`= Tb_Save_Image.id`, migration 071), so a re-poll or
/// crash-after-commit replay never lands the same legacy image twice.
///
/// Read-only from legacy, so it defaults ENABLED — kill-switch
/// `GUEST_DOC_SYNC_ENABLED=false`/`0` disables it.
async fn sync_guest_documents(pg: &PgPool, mssql: &DbPool, shadow_mode: bool, site_id: &str) {
    // Kill-switch: DEFAULT ENABLED; only skip when explicitly "false"/"0".
    let enabled = match std::env::var("GUEST_DOC_SYNC_ENABLED") {
        Ok(v) => {
            let v = v.trim().to_ascii_lowercase();
            v != "false" && v != "0"
        }
        Err(_) => true,
    };
    if !enabled {
        return;
    }

    if shadow_mode {
        tracing::info!(
            event_name = "guest_doc_sync_shadow",
            site = %site_id,
            "guest-image sync (shadow): skipping legacy read + canonical write"
        );
        return;
    }

    // Bounded batch (newest first) of check-ins that still have NO image at all.
    let batch: Vec<(i32, Option<i32>, String)> =
        match sqlx::query_as::<_, (i32, Option<i32>, String)>(
            "SELECT ci.cin_id, ci.cin_cust_id, ci.legacy_cin_no FROM ht_checkins ci \
              WHERE ci.legacy_cin_no IS NOT NULL AND ci.legacy_cin_no <> '' \
                AND NOT EXISTS ( \
                    SELECT 1 FROM ht_guest_documents gd WHERE gd.doc_cin_id = ci.cin_id) \
              ORDER BY ci.cin_id DESC LIMIT 20",
        )
        .fetch_all(pg)
        .await
        {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    event_name = "guest_doc_sync_query_fail",
                    site = %site_id,
                    error = %err,
                    "guest-image sync: PG batch SELECT failed; skipping"
                );
                return;
            }
        };
    if batch.is_empty() {
        return;
    }

    // legacy_cin_no -> (cin_id, cin_cust_id)
    let mut by_cin_no: HashMap<String, (i32, Option<i32>)> = HashMap::new();
    for (cin_id, cust_id, legacy_cin_no) in &batch {
        by_cin_no.insert(legacy_cin_no.clone(), (*cin_id, *cust_id));
    }

    // Safely single-quote the cin_nos for the IN list. These are our own
    // CHyy-nnnnnn values (no MSSQL literal-encoding hazard — plain `'…'`, never
    // `N'…'`), but escape doubled single-quotes defensively.
    let in_list = by_cin_no
        .keys()
        .map(|k| format!("'{}'", k.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(", ");

    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!(
                event_name = "guest_doc_sync_conn_fail",
                site = %site_id,
                error = %err,
                "guest-image sync: could not acquire MSSQL connection; skipping"
            );
            return;
        }
    };

    // Step 1 — metadata only (NO blob): pick which image to mirror per cin_no.
    let meta_sql =
        format!("SELECT id, cin_no, ttype FROM Tb_Save_Image WHERE cin_no IN ({in_list})");
    let meta_rows =
        match simple_query_with_timeout_pooled(&mut conn, &meta_sql, MssqlOpKind::Read).await {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    event_name = "guest_doc_sync_query_fail",
                    site = %site_id,
                    error = %err,
                    "guest-image sync: Tb_Save_Image metadata SELECT failed; skipping"
                );
                return;
            }
        };

    // Classify ttype IN RUST by substring (avoids Thai-literal encoding issues in
    // SQL) and keep the lowest-rank per cin_no (prefer id card / passport over the
    // face photo).
    let mut chosen: HashMap<String, (i32, &'static str, i32)> = HashMap::new();
    for row in &meta_rows {
        let Some(legacy_id) = tiberius::Row::try_get::<i32, _>(row, "id").ok().flatten() else {
            continue;
        };
        let Some(cin_no) = tiberius::Row::try_get::<&str, _>(row, "cin_no")
            .ok()
            .flatten()
        else {
            continue;
        };
        let ttype = tiberius::Row::try_get::<&str, _>(row, "ttype")
            .ok()
            .flatten()
            .unwrap_or("");
        let (doc_type, rank): (&'static str, i32) = if ttype.contains("บัตร") {
            ("thai_id_card", 0)
        } else if ttype.contains("ลูกค้า") {
            ("face_photo", 1)
        } else if ttype.contains("เดินทาง") {
            ("passport", 0)
        } else {
            continue;
        };
        // Defensive: only mirror images that belong to a cin_no in our batch.
        if !by_cin_no.contains_key(cin_no) {
            continue;
        }
        let entry = chosen
            .entry(cin_no.to_string())
            .or_insert((legacy_id, doc_type, rank));
        if rank < entry.2 {
            *entry = (legacy_id, doc_type, rank);
        }
    }
    if chosen.is_empty() {
        return;
    }

    // Step 2 — pull the blobs for exactly the chosen ids.
    let id_list = chosen
        .values()
        .map(|(id, _, _)| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let blob_sql = format!("SELECT id, pic FROM Tb_Save_Image WHERE id IN ({id_list})");
    let blob_rows =
        match simple_query_with_timeout_pooled(&mut conn, &blob_sql, MssqlOpKind::Read).await {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    event_name = "guest_doc_sync_blob_fail",
                    site = %site_id,
                    error = %err,
                    "guest-image sync: Tb_Save_Image blob SELECT failed; skipping"
                );
                return;
            }
        };
    drop(conn); // release the pooled MSSQL connection before PG work

    let mut blobs: HashMap<i32, Vec<u8>> = HashMap::new();
    for row in &blob_rows {
        let Some(id) = tiberius::Row::try_get::<i32, _>(row, "id").ok().flatten() else {
            continue;
        };
        if let Some(pic) = tiberius::Row::try_get::<&[u8], _>(row, "pic")
            .ok()
            .flatten()
        {
            blobs.insert(id, pic.to_vec());
        }
    }

    let mut upserted = 0usize;
    for (cin_no, &(legacy_id, doc_type, _rank)) in &chosen {
        let Some(image) = blobs.get(&legacy_id) else {
            tracing::warn!(
                event_name = "guest_doc_sync_row_skip",
                site = %site_id,
                legacy_id,
                "guest-image sync: no blob returned for chosen id; skipping"
            );
            continue;
        };
        // Detect mime from the blob magic bytes.
        let mime = if image.len() >= 4
            && image[0] == 0x89
            && image[1] == b'P'
            && image[2] == b'N'
            && image[3] == b'G'
        {
            "image/png"
        } else if image.len() >= 2 && image[0] == 0xFF && image[1] == 0xD8 {
            "image/jpeg"
        } else {
            "image/jpeg"
        };
        let Some((cin_id, cust_id)) = by_cin_no.get(cin_no).copied() else {
            continue;
        };

        // Runtime `sqlx::query` (not the `query!` macro) — adds nothing to the
        // `.sqlx/` offline cache. Idempotent UPSERT keyed on the legacy id
        // (migration 071 partial UNIQUE `ux_ht_guest_documents_legacy_id`).
        let res = sqlx::query(
            "INSERT INTO ht_guest_documents ( \
                 doc_cust_id, doc_cin_id, doc_type, doc_mime, doc_image, doc_source, doc_legacy_id \
             ) VALUES ($1, $2, $3, $4, $5, 'legacy', $6) \
             ON CONFLICT (doc_legacy_id) DO UPDATE SET \
                 doc_cust_id = EXCLUDED.doc_cust_id, \
                 doc_cin_id  = EXCLUDED.doc_cin_id, \
                 doc_type    = EXCLUDED.doc_type, \
                 doc_mime    = EXCLUDED.doc_mime, \
                 doc_image   = EXCLUDED.doc_image, \
                 doc_source  = EXCLUDED.doc_source",
        )
        .bind(cust_id)
        .bind(cin_id)
        .bind(doc_type)
        .bind(mime)
        .bind(image)
        .bind(legacy_id)
        .execute(pg)
        .await;

        match res {
            Ok(_) => upserted += 1,
            Err(err) => {
                tracing::warn!(
                    event_name = "guest_doc_sync_upsert_fail",
                    site = %site_id,
                    legacy_id,
                    error = %err,
                    "guest-image sync: UPSERT into ht_guest_documents failed; continuing"
                );
            }
        }
    }

    if upserted > 0 {
        tracing::debug!(
            event_name = "guest_doc_sync_ok",
            site = %site_id,
            upserted,
            "guest-image sync: mirrored legacy Tb_Save_Image rows into ht_guest_documents"
        );
    }
}

/// How many days back the cash-ledger poll scans `TB_Pay_History` each tick.
/// Petty-cash entries are low-volume and almost always dated "today"; bounding
/// the scan to a recent window keeps the per-tick cost ~constant as the legacy
/// ledger grows over the years (parity with `sync_round_bills`' 2-day window,
/// just wider because cash entries can be back-dated a little).
const CASH_WINDOW_DAYS: i64 = 120;

/// Convert a legacy OLE-Automation date serial (`TB_Pay_History.Pay_Date` /
/// `Pay_Program` are `float` OADates — `DateTime.ToOADate()`, days since
/// 1899-12-30 with the fractional part = fraction of a 24h day) into a true UTC
/// instant. The serial encodes a **Thai-local wall clock** (the legacy app is
/// tz-naive), so we attach +07:00 before converting — same convention as
/// `naive_thai_to_utc` everywhere else.
///
/// Returns `None` for non-finite / non-positive / absurd serials (garbage or a
/// genuine "no date" zero) so the caller can store NULL rather than a bogus
/// 1899 timestamp. PURE.
fn ole_serial_to_utc(serial: f64) -> Option<chrono::DateTime<chrono::Utc>> {
    if !serial.is_finite() || serial <= 0.0 || serial > 400_000.0 {
        return None; // garbage, "no date", or implausibly far future (>~2994)
    }
    let epoch = chrono::NaiveDate::from_ymd_opt(1899, 12, 30)?;
    let whole_days = serial.trunc();
    let date = epoch.checked_add_days(chrono::Days::new(whole_days as u64))?;
    let frac = serial - whole_days; // [0, 1)
    let secs = ((frac * 86_400.0).round() as i64).clamp(0, 86_399);
    let naive = date.and_hms_opt(0, 0, 0)? + chrono::Duration::seconds(secs);
    Some(naive_thai_to_utc(naive))
}

/// Normalize the raw legacy `TB_Pay_History.Pay_Type` marker into our
/// canonical `cash_kind`. BEST-EFFORT: the income/expense screen is
/// "รายรับ-รายจ่าย", so the marker almost always contains the Thai word
/// "รับ" (receive → income) or "จ่าย" (pay → expense). Anything we can't
/// classify is stored as `'unknown'` — the RAW `Pay_Type` is always preserved
/// verbatim in `ht_cash_ledger.cash_legacy_type`, which stays authoritative.
/// PURE.
fn cash_kind_from_pay_type(pay_type: &str) -> &'static str {
    if pay_type.contains("รับ") {
        "income"
    } else if pay_type.contains("จ่าย") {
        "expense"
    } else {
        "unknown"
    }
}

/// Read-only sync of the legacy account taxonomy (`TB_SET_MyType2` /
/// `TB_SET_MyType2_2` / `TB_SET_MyType3`) into canonical `ht_cash_categories`.
///
/// These three legacy tables share the same `(id IDENTITY, id_full, name)`
/// shape and classify cash-ledger entries; we mirror all three in one
/// `UNION ALL` round-trip, tagging each row with its `cat_level`
/// ('2' | '2_2' | '3'). They are essentially static config (≤~40 rows total),
/// so a full poll each tick is cheap. Same resilience contract as
/// `sync_round_bills`: every error path logs at WARN and returns; shadow mode
/// skips the canonical write.
async fn sync_cash_categories(pg: &PgPool, mssql: &DbPool, shadow_mode: bool, site_id: &str) {
    const CATEGORY_SELECT: &str = "\
         SELECT '2' AS lvl, id, id_full, name FROM TB_SET_MyType2 \
         UNION ALL SELECT '2_2' AS lvl, id, id_full, name FROM TB_SET_MyType2_2 \
         UNION ALL SELECT '3' AS lvl, id, id_full, name FROM TB_SET_MyType3";

    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!(
                event_name = "cash_category_sync_conn_fail",
                site = %site_id,
                error = %err,
                "cash-category sync: could not acquire MSSQL connection; skipping"
            );
            return;
        }
    };
    let rows =
        match simple_query_with_timeout_pooled(&mut conn, CATEGORY_SELECT, MssqlOpKind::Read).await {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    event_name = "cash_category_sync_query_fail",
                    site = %site_id,
                    error = %err,
                    "cash-category sync: SELECT failed; skipping"
                );
                return;
            }
        };
    drop(conn); // release the pooled MSSQL connection before PG work

    let mut upserted = 0usize;
    for row in &rows {
        let Some(level) = tiberius::Row::try_get::<&str, _>(row, "lvl").ok().flatten() else {
            continue;
        };
        let Some(legacy_id) = tiberius::Row::try_get::<i32, _>(row, "id").ok().flatten() else {
            continue;
        };
        let id_full = tiberius::Row::try_get::<&str, _>(row, "id_full")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let name = tiberius::Row::try_get::<&str, _>(row, "name")
            .ok()
            .flatten()
            .map(|s| s.to_string());

        if shadow_mode {
            continue;
        }

        // Runtime `sqlx::query` (NOT the `query!` macro) so this adds nothing
        // to the `.sqlx/` offline cache. Idempotent UPSERT keyed on the
        // (level, legacy id) unique constraint.
        let res = sqlx::query(
            "INSERT INTO ht_cash_categories \
                 (cat_level, cat_legacy_id, cat_id_full, cat_name) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (cat_level, cat_legacy_id) DO UPDATE SET \
                 cat_id_full   = EXCLUDED.cat_id_full, \
                 cat_name      = EXCLUDED.cat_name, \
                 cat_synced_at = NOW()",
        )
        .bind(level)
        .bind(legacy_id)
        .bind(&id_full)
        .bind(&name)
        .execute(pg)
        .await;

        match res {
            Ok(_) => upserted += 1,
            Err(err) => {
                tracing::warn!(
                    event_name = "cash_category_sync_upsert_fail",
                    site = %site_id,
                    level,
                    legacy_id,
                    error = %err,
                    "cash-category sync: UPSERT into ht_cash_categories failed; continuing"
                );
            }
        }
    }

    if upserted > 0 {
        tracing::debug!(
            event_name = "cash_category_sync_ok",
            site = %site_id,
            upserted,
            "cash-category sync: upserted canonical cash categories"
        );
    }
}

/// Read-only sync of the legacy petty-cash ledger (`TB_Pay_History`) into
/// canonical `ht_cash_ledger`.
///
/// **Why**: coexistence (ADR 0002) means a receptionist may record a cash
/// in/out entry in EITHER iHOTEL or our app. iHOTEL writes `TB_Pay_History`;
/// our income/expense page reads canonical `ht_cash_ledger`. Without this poll,
/// entries made in iHOTEL would be invisible to our app.
///
/// **Shape**: each tick we SELECT rows dated within the last
/// [`CASH_WINDOW_DAYS`] (the `Pay_Date` OADate float compared against a
/// pre-computed cutoff serial) plus any null-dated rows, and UPSERT each into
/// `ht_cash_ledger` keyed on `cash_legacy_id` = `TB_Pay_History.id`. The
/// `Pay_Date` / `Pay_Program` OADate floats are converted to UTC instants and
/// the raw `Pay_Type` is preserved verbatim alongside the normalized
/// `cash_kind`.
///
/// **Resilience**: identical to `sync_round_bills` — every error path logs at
/// WARN and returns/continues; shadow mode skips the canonical write.
async fn sync_cash_history(pg: &PgPool, mssql: &DbPool, shadow_mode: bool, site_id: &str) {
    // Pre-compute the OADate cutoff serial for (today − CASH_WINDOW_DAYS) in
    // Bangkok local time, so the legacy comparison stays a cheap float >=.
    let epoch = match chrono::NaiveDate::from_ymd_opt(1899, 12, 30) {
        Some(d) => d,
        None => return,
    };
    let today_bkk = chrono::Utc::now()
        .with_timezone(
            &chrono::FixedOffset::east_opt(7 * 3600).expect("+07:00 is a valid offset"),
        )
        .date_naive();
    let cutoff_date = today_bkk - chrono::Duration::days(CASH_WINDOW_DAYS);
    let cutoff_serial = (cutoff_date - epoch).num_days();

    // Note the explicit column list (the legacy table is positional but we read
    // by name) and the recent-window filter. `Pay_Date IS NULL` rows are always
    // included (cheap; never lose an undated entry).
    let select_sql = format!(
        "SELECT id, Pay_Date, Pay_Bill, Pay_Cust, Pay_Type, Pay_Total, \
                Pay_Note, Pay_Program, Pay_Group, Pay_Account \
           FROM TB_Pay_History \
          WHERE Pay_Date >= {cutoff_serial} OR Pay_Date IS NULL"
    );

    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!(
                event_name = "cash_history_sync_conn_fail",
                site = %site_id,
                error = %err,
                "cash-history sync: could not acquire MSSQL connection; skipping"
            );
            return;
        }
    };
    let rows =
        match simple_query_with_timeout_pooled(&mut conn, &select_sql, MssqlOpKind::Read).await {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    event_name = "cash_history_sync_query_fail",
                    site = %site_id,
                    error = %err,
                    "cash-history sync: SELECT failed; skipping"
                );
                return;
            }
        };
    drop(conn); // release the pooled MSSQL connection before PG work

    let mut upserted = 0usize;
    for row in &rows {
        let Some(legacy_id) = tiberius::Row::try_get::<i32, _>(row, "id").ok().flatten() else {
            tracing::warn!(
                event_name = "cash_history_sync_row_skip",
                site = %site_id,
                "cash-history sync: row missing id; skipping"
            );
            continue;
        };

        let entry_date = tiberius::Row::try_get::<f64, _>(row, "Pay_Date")
            .ok()
            .flatten()
            .and_then(ole_serial_to_utc);
        let program_date = tiberius::Row::try_get::<f64, _>(row, "Pay_Program")
            .ok()
            .flatten()
            .and_then(ole_serial_to_utc);
        let pay_type = tiberius::Row::try_get::<&str, _>(row, "Pay_Type")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let kind = cash_kind_from_pay_type(pay_type.as_deref().unwrap_or(""));
        let amount_raw = tiberius::Row::try_get::<f64, _>(row, "Pay_Total")
            .ok()
            .flatten()
            .unwrap_or(0.0);
        // Coerce a non-finite legacy float to 0 so the ::numeric cast can't
        // poison the batch (mirrors the ht_payment_ledger projection).
        let amount = if amount_raw.is_finite() { amount_raw } else { 0.0 };
        let bill_no = tiberius::Row::try_get::<&str, _>(row, "Pay_Bill")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let payee = tiberius::Row::try_get::<&str, _>(row, "Pay_Cust")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let note = tiberius::Row::try_get::<&str, _>(row, "Pay_Note")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let group = tiberius::Row::try_get::<&str, _>(row, "Pay_Group")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let account = tiberius::Row::try_get::<&str, _>(row, "Pay_Account")
            .ok()
            .flatten()
            .map(|s| s.to_string());

        if shadow_mode {
            tracing::info!(
                event_name = "cash_history_sync_shadow",
                site = %site_id,
                legacy_id,
                kind,
                "cash-history sync (shadow): would upsert canonical cash entry"
            );
            continue;
        }

        // Runtime `sqlx::query` (NOT the `query!` macro) so this adds nothing
        // to the `.sqlx/` offline cache. `$6::numeric` mirrors the f64→NUMERIC
        // convention used by sync_round_bills. Idempotent UPSERT keyed on
        // cash_legacy_id; cash_source is forced to 'legacy' (this is the
        // legacy mirror path).
        //
        // ECHO-SAFETY (issue #202): this UPSERT dedups on `cash_legacy_id`, so it
        // is echo-safe ONLY if an app-originated cash entry already carries its
        // allocated legacy id in `cash_legacy_id` by the time this mirror re-reads
        // it. Cash-OUTBOUND writeback (`writeback/recipes/cash_entry.rs`) is
        // currently DARK, so no echo occurs today. BEFORE enabling it, the worker
        // MUST back-populate `cash_legacy_id` onto the canonical row after the
        // writeback allocates the legacy id (the same pattern payments use:
        // `back_populate_legacy_ids` → `legacy_receipt_no`, which the HT_Receipt_H
        // importer then dedups on — see `sync/mappers/payment.rs`). Without that
        // back-population, our own cash write re-imports here as a phantom
        // duplicate (`cash_source='legacy'`, app's original keeps `cash_legacy_id`
        // NULL). Verify before flipping cash-outbound writeback.
        let res = sqlx::query(
            "INSERT INTO ht_cash_ledger ( \
                 cash_legacy_id, cash_kind, cash_legacy_type, cash_entry_date, \
                 cash_bill_no, cash_payee, cash_amount, cash_note, \
                 cash_program_date, cash_group, cash_account, cash_source \
             ) VALUES ($1, $2, $3, $4, $5, $6, $7::numeric, $8, $9, $10, $11, 'legacy') \
             ON CONFLICT (cash_legacy_id) DO UPDATE SET \
                 cash_kind         = EXCLUDED.cash_kind, \
                 cash_legacy_type  = EXCLUDED.cash_legacy_type, \
                 cash_entry_date   = EXCLUDED.cash_entry_date, \
                 cash_bill_no      = EXCLUDED.cash_bill_no, \
                 cash_payee        = EXCLUDED.cash_payee, \
                 cash_amount       = EXCLUDED.cash_amount, \
                 cash_note         = EXCLUDED.cash_note, \
                 cash_program_date = EXCLUDED.cash_program_date, \
                 cash_group        = EXCLUDED.cash_group, \
                 cash_account      = EXCLUDED.cash_account, \
                 cash_source       = 'legacy', \
                 cash_synced_at    = NOW()",
        )
        .bind(legacy_id)
        .bind(kind)
        .bind(&pay_type)
        .bind(entry_date)
        .bind(&bill_no)
        .bind(&payee)
        .bind(amount)
        .bind(&note)
        .bind(program_date)
        .bind(&group)
        .bind(&account)
        .execute(pg)
        .await;

        match res {
            Ok(_) => upserted += 1,
            Err(err) => {
                tracing::warn!(
                    event_name = "cash_history_sync_upsert_fail",
                    site = %site_id,
                    legacy_id,
                    error = %err,
                    "cash-history sync: UPSERT into ht_cash_ledger failed; continuing"
                );
            }
        }
    }

    if upserted > 0 {
        tracing::debug!(
            event_name = "cash_history_sync_ok",
            site = %site_id,
            upserted,
            "cash-history sync: upserted canonical cash entries from TB_Pay_History"
        );
    }
}

/// Normalize the legacy `SMS_Readed` varchar marker ('yes' / 'no', lowercase
/// per `docs/legacy-app/COMPAT_CHEATSHEET.md:98`) into the canonical
/// `note_is_read` bool. Anything other than a case-insensitive "yes" is treated
/// as unread (the legacy default on insert is 'no'). PURE.
fn sms_readed_to_bool(readed: &str) -> bool {
    readed.trim().eq_ignore_ascii_case("yes")
}

/// Read-only sync of the legacy sticky-note tables (`HT_Room_SMS` +
/// `HT_EMP_SMS`) into canonical `ht_notes` (task #47).
///
/// **Why**: coexistence (ADR 0002) means a note may be added — or marked read —
/// in EITHER iHOTEL or our app. iHOTEL writes the SMS tables; our board reads
/// canonical `ht_notes`. Without this poll, notes/read-flips made in iHOTEL
/// would be invisible to our app.
///
/// **Shape**: both legacy tables share the same columns (differing only in the
/// target-key column — `SMS_Room` vs `SMS_TO`), so one `UNION ALL` round-trip
/// mirrors both, tagging each row with `note_target_kind` ('room' | 'staff').
/// Each row UPSERTs into `ht_notes` keyed on `(note_target_kind, note_legacy_id)`
/// = the per-table IDENTITY `SMS_ID` (the two tables have independent IDENTITY
/// sequences, so the pair is the key). `note_source` is forced to 'legacy'. The
/// DO UPDATE carries a `WHERE … IS DISTINCT FROM …` guard so a steady-state
/// tick (nothing changed) performs ZERO row writes — only genuinely-changed
/// notes (new body / author / read-flip) cost a write.
///
/// **Resilience**: identical to `sync_cash_history` — every error path logs at
/// WARN and returns/continues; shadow mode skips the canonical write.
async fn sync_sticky_notes(pg: &PgPool, mssql: &DbPool, shadow_mode: bool, site_id: &str) {
    // Both SMS tables, one round-trip. The target-key column is aliased to a
    // common name so the row reader is uniform.
    const NOTES_SELECT: &str = "\
         SELECT 'room' AS kind, SMS_ID, SMS_Room AS target_key, SMS_Details, SMS_By, SMS_Readed \
           FROM HT_Room_SMS \
         UNION ALL \
         SELECT 'staff' AS kind, SMS_ID, SMS_TO AS target_key, SMS_Details, SMS_By, SMS_Readed \
           FROM HT_EMP_SMS";

    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!(
                event_name = "sticky_note_sync_conn_fail",
                site = %site_id,
                error = %err,
                "sticky-note sync: could not acquire MSSQL connection; skipping"
            );
            return;
        }
    };
    let rows =
        match simple_query_with_timeout_pooled(&mut conn, NOTES_SELECT, MssqlOpKind::Read).await {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    event_name = "sticky_note_sync_query_fail",
                    site = %site_id,
                    error = %err,
                    "sticky-note sync: SELECT failed; skipping"
                );
                return;
            }
        };
    drop(conn); // release the pooled MSSQL connection before PG work

    let mut upserted = 0usize;
    for row in &rows {
        let Some(kind) = tiberius::Row::try_get::<&str, _>(row, "kind").ok().flatten() else {
            continue;
        };
        let Some(legacy_id) = tiberius::Row::try_get::<i32, _>(row, "SMS_ID").ok().flatten() else {
            tracing::warn!(
                event_name = "sticky_note_sync_row_skip",
                site = %site_id,
                "sticky-note sync: row missing SMS_ID; skipping"
            );
            continue;
        };
        let target_key = tiberius::Row::try_get::<&str, _>(row, "target_key")
            .ok()
            .flatten()
            .unwrap_or("")
            .to_string();
        let body = tiberius::Row::try_get::<&str, _>(row, "SMS_Details")
            .ok()
            .flatten()
            .unwrap_or("")
            .to_string();
        let created_by = tiberius::Row::try_get::<&str, _>(row, "SMS_By")
            .ok()
            .flatten()
            .map(|s| s.to_string());
        let is_read = sms_readed_to_bool(
            tiberius::Row::try_get::<&str, _>(row, "SMS_Readed")
                .ok()
                .flatten()
                .unwrap_or(""),
        );

        if shadow_mode {
            continue;
        }

        // Runtime `sqlx::query` (NOT the `query!` macro) so this adds nothing to
        // the `.sqlx/` offline cache. Idempotent UPSERT keyed on
        // (note_target_kind, note_legacy_id); the DO UPDATE `WHERE` guard makes
        // an unchanged row a no-op (zero steady-state writes).
        let res = sqlx::query(
            "INSERT INTO ht_notes ( \
                 note_target_kind, note_target_key, note_body, note_created_by, \
                 note_is_read, note_legacy_id, note_source \
             ) VALUES ($1, $2, $3, $4, $5, $6, 'legacy') \
             ON CONFLICT (note_target_kind, note_legacy_id) DO UPDATE SET \
                 note_target_key = EXCLUDED.note_target_key, \
                 note_body       = EXCLUDED.note_body, \
                 note_created_by = EXCLUDED.note_created_by, \
                 note_is_read    = EXCLUDED.note_is_read, \
                 note_source     = 'legacy', \
                 note_updated_at = NOW(), \
                 note_synced_at  = NOW() \
             WHERE ht_notes.note_target_key IS DISTINCT FROM EXCLUDED.note_target_key \
                OR ht_notes.note_body       IS DISTINCT FROM EXCLUDED.note_body \
                OR ht_notes.note_created_by IS DISTINCT FROM EXCLUDED.note_created_by \
                OR ht_notes.note_is_read    IS DISTINCT FROM EXCLUDED.note_is_read",
        )
        .bind(kind)
        .bind(&target_key)
        .bind(&body)
        .bind(&created_by)
        .bind(is_read)
        .bind(legacy_id)
        .execute(pg)
        .await;

        match res {
            Ok(_) => upserted += 1,
            Err(err) => {
                tracing::warn!(
                    event_name = "sticky_note_sync_upsert_fail",
                    site = %site_id,
                    kind,
                    legacy_id,
                    error = %err,
                    "sticky-note sync: UPSERT into ht_notes failed; continuing"
                );
            }
        }
    }

    if upserted > 0 {
        tracing::debug!(
            event_name = "sticky_note_sync_ok",
            site = %site_id,
            upserted,
            "sticky-note sync: upserted canonical notes from HT_Room_SMS/HT_EMP_SMS"
        );
    }
}

#[cfg(test)]
mod sticky_note_sync_tests {
    use super::sms_readed_to_bool;

    /// `SMS_Readed` normalization: only a case-insensitive "yes" is read; the
    /// legacy 'no' default + any garbage is unread.
    #[test]
    fn sms_readed_normalizes_yes_no() {
        assert!(sms_readed_to_bool("yes"));
        assert!(sms_readed_to_bool("YES"));
        assert!(sms_readed_to_bool(" Yes "));
        assert!(!sms_readed_to_bool("no"));
        assert!(!sms_readed_to_bool(""));
        assert!(!sms_readed_to_bool("maybe"));
    }
}

#[cfg(test)]
mod cash_sync_tests {
    use super::{cash_kind_from_pay_type, ole_serial_to_utc};
    use chrono::{TimeZone, Utc};

    #[test]
    fn ole_serial_round_trips_a_known_date() {
        // 46136 = 2026-04-24 (matches writeback::format::date_to_ole_serial's
        // spike-verified value). Integer serial = midnight Bangkok = 17:00 UTC
        // the previous day.
        let dt = ole_serial_to_utc(46136.0).expect("valid serial");
        assert_eq!(dt, Utc.with_ymd_and_hms(2026, 4, 23, 17, 0, 0).unwrap());
    }

    #[test]
    fn ole_serial_decodes_the_time_fraction() {
        // 0.5 of a day = 12:00 Bangkok = 05:00 UTC.
        let dt = ole_serial_to_utc(46136.5).expect("valid serial");
        assert_eq!(dt, Utc.with_ymd_and_hms(2026, 4, 24, 5, 0, 0).unwrap());
    }

    #[test]
    fn ole_serial_rejects_garbage_and_zero() {
        assert!(ole_serial_to_utc(0.0).is_none(), "zero = no date");
        assert!(ole_serial_to_utc(-5.0).is_none(), "negative");
        assert!(ole_serial_to_utc(f64::NAN).is_none(), "NaN");
        assert!(ole_serial_to_utc(f64::INFINITY).is_none(), "Inf");
        assert!(ole_serial_to_utc(9_000_000.0).is_none(), "absurd future");
    }

    #[test]
    fn pay_type_classifies_income_expense_unknown() {
        assert_eq!(cash_kind_from_pay_type("รายรับ"), "income");
        assert_eq!(cash_kind_from_pay_type("รับเงิน"), "income");
        assert_eq!(cash_kind_from_pay_type("รายจ่าย"), "expense");
        assert_eq!(cash_kind_from_pay_type("จ่ายค่าน้ำ"), "expense");
        assert_eq!(cash_kind_from_pay_type(""), "unknown");
        assert_eq!(cash_kind_from_pay_type("misc"), "unknown");
    }
}

/// Persist a `DomainEvent` into `event_log` inside the caller's TX.
/// Wraps `EventBus::publish` so the watcher and the service layer take
/// the same code path.
async fn persist_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &DomainEvent,
) -> Result<(), sqlx::Error> {
    let _id = EventBus::publish(tx, event).await?;
    Ok(())
}

async fn check_retention(
    mssql: &DbPool,
    table: &str,
    last_seen: i64,
) -> Result<(), String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
    let sql = format!(
        "SELECT CHANGE_TRACKING_MIN_VALID_VERSION(OBJECT_ID(N'{table}')) AS min_valid"
    );
    let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read)
        .await
        .map_err(|e| e.to_string())?;
    let row = match rows.first() {
        Some(r) => r,
        None => return Ok(()),
    };
    let min_valid: Option<i64> = row.get(0);
    let Some(min_valid) = min_valid else {
        return Ok(());
    };
    if min_valid > last_seen {
        return Err(format!(
            "retention overflow: min_valid_version={min_valid} > last_seen={last_seen}"
        ));
    }
    Ok(())
}

/// Pure helper for the startup CT-enablement gate: from per-table probe
/// outcomes, return the tables that are DEFINITIVELY missing Change Tracking.
/// `Some(true)` = CT on, `Some(false)` = CT confirmed off (refuse),
/// `None` = probe errored (transient — must NOT count as missing, or a
/// connectivity blip would refuse startup). Pulled out so the
/// "transient errors never refuse" invariant is unit-testable without MSSQL.
fn ct_tables_definitely_missing<'a>(probes: &[(&'a str, Option<bool>)]) -> Vec<&'a str> {
    probes
        .iter()
        .filter_map(|(t, outcome)| match outcome {
            Some(false) => Some(*t),
            _ => None,
        })
        .collect()
}

/// Probe whether Change Tracking is enabled on `table`.
/// `CHANGE_TRACKING_MIN_VALID_VERSION(OBJECT_ID(N'...'))` returns NULL exactly
/// when CT is not enabled (or the object doesn't exist), and a version number
/// otherwise — so `Ok(false)` is a definitive "CT is off" answer while `Err`
/// is a connectivity/query failure. The caller MUST keep these distinct: a
/// transient probe failure must never be mistaken for a missing migration
/// (that would refuse startup on a blip). Mirrors `check_retention`'s pooled
/// read path. Added 2026-06-24 as the startup CT-enablement gate.
async fn check_ct_enabled(mssql: &DbPool, table: &str) -> Result<bool, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
    let sql = format!(
        "SELECT CASE WHEN CHANGE_TRACKING_MIN_VALID_VERSION(OBJECT_ID(N'{table}')) IS NULL \
         THEN 0 ELSE 1 END AS ct_on"
    );
    let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read)
        .await
        .map_err(|e| e.to_string())?;
    let row = rows
        .first()
        .ok_or_else(|| "ct-enablement probe returned no rows".to_string())?;
    let on: i32 = row.get(0).unwrap_or(0);
    Ok(on == 1)
}

async fn count_ct_rows(
    mssql: &DbPool,
    table: &str,
    last_seen: i64,
) -> Result<i64, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
    let sql = build_ct_count_sql(table, last_seen);
    let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read)
        .await
        .map_err(|e| e.to_string())?;
    let row = rows
        .first()
        .ok_or_else(|| "COUNT(*) returned no rows".to_string())?;
    let n: i32 = row.get(0).unwrap_or(0);
    Ok(n as i64)
}

async fn bump_skipped(
    pg: &PgPool,
    table: &str,
    n: i64,
    errored: bool,
) -> Result<(), sqlx::Error> {
    // When the tick produced any per-row error, leave last_error /
    // consecutive_failures alone so the increments from
    // `record_table_error` survive — otherwise a 100%-failing tick that
    // also reaches this counter bump would silently reset the failure
    // count to 0 and the dashboard would never see the failure.
    let sql = if errored {
        "UPDATE legacy_sync_status \
            SET rows_skipped      = rows_skipped + $2, \
                last_processed_at = now() \
          WHERE table_name = $1"
    } else {
        "UPDATE legacy_sync_status \
            SET rows_skipped         = rows_skipped + $2, \
                last_processed_at    = now(), \
                last_error           = NULL, \
                last_error_at        = NULL, \
                consecutive_failures = 0 \
          WHERE table_name = $1"
    };
    sqlx::query(sql).bind(table).bind(n).execute(pg).await?;
    Ok(())
}

async fn bump_counters(
    pg: &PgPool,
    table: &str,
    ingested: i64,
    skipped: i64,
    errored: bool,
) -> Result<(), sqlx::Error> {
    let sql = if errored {
        "UPDATE legacy_sync_status \
            SET rows_ingested     = rows_ingested + $2, \
                rows_skipped      = rows_skipped + $3, \
                last_processed_at = now() \
          WHERE table_name = $1"
    } else {
        "UPDATE legacy_sync_status \
            SET rows_ingested        = rows_ingested + $2, \
                rows_skipped         = rows_skipped + $3, \
                last_processed_at    = now(), \
                last_error           = NULL, \
                last_error_at        = NULL, \
                consecutive_failures = 0 \
          WHERE table_name = $1"
    };
    sqlx::query(sql)
        .bind(table)
        .bind(ingested)
        .bind(skipped)
        .execute(pg)
        .await?;
    Ok(())
}

/// Decide whether (and to what version) the CT watermark should advance
/// at the end of a per-table tick.
///
/// Returns `Some(target_version)` when the caller should advance, or
/// `None` when the watermark must stay pinned at `last_seen` so the
/// next tick re-fetches the same CT rows.
///
/// Rules:
/// * If `errored` is true, hold the watermark at `last_seen` regardless
///   of how far `max_version` advanced this tick. UPSERT semantics in the
///   aggregate/per-row appliers make re-applying the already-succeeded
///   keys idempotent, so it's safe to re-process the whole batch; the
///   alternative — advancing past the failed key's CT version — silently
///   drops the failed event after MSSQL's 2-day CT retention expires.
/// * If no error occurred AND `max_version > last_seen`, advance to
///   `max_version`. This is the original pre-fix happy path.
/// * Otherwise (success but no version progress), return `None` — no
///   work to do.
///
/// Closes the silent-drop bug observed on HF Hotel 2026-05-11..15: 30+
/// `new-hotel-production-sync-1` container kills landed mid-tick after
/// some keys had committed; the watermark advanced past the failed
/// keys' versions and the corresponding `ht_reconcile_log` divergences
/// (28 rows, `divergence_kind='value'`) were stuck until manual
/// intervention.
fn next_watermark_after_tick(
    max_version: i64,
    last_seen: i64,
    errored: bool,
) -> Option<i64> {
    if errored {
        return None;
    }
    if max_version > last_seen {
        Some(max_version)
    } else {
        None
    }
}

/// Persist a per-table failure mode to `legacy_sync_status` for
/// cross-restart visibility. Runs in its OWN sqlx auto-TX (the caller's
/// failed-tick TX, if any, is rolled back independently — wrapping this
/// write in that TX would lose the failure mode along with the canonical
/// state mutation we're trying to record).
///
/// `event_name` is one of the `EV_*` constants above and is greppable
/// across both the log stream and the persisted row. `err` is the raw
/// upstream error; it gets JSON-sanitized + truncated by
/// `format_last_error` before persistence.
async fn record_table_error(
    pg: &PgPool,
    table: &str,
    event_name: &str,
    err: &str,
) -> Result<(), sqlx::Error> {
    let payload = format_last_error(event_name, err);
    sqlx::query(
        "UPDATE legacy_sync_status \
            SET last_error           = $2, \
                last_error_at        = now(), \
                consecutive_failures = consecutive_failures + 1 \
          WHERE table_name = $1",
    )
    .bind(table)
    .bind(payload)
    .execute(pg)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // CLI mode parsing (parse_cli_mode) — the --dry-run / unknown-flag guard
    // ========================================================================

    fn argv(rest: &[&str]) -> Vec<String> {
        // parse_cli_mode skips argv[0]; supply a realistic program name.
        std::iter::once("./sync")
            .chain(rest.iter().copied())
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn parse_cli_mode_no_args_is_watcher() {
        assert_eq!(parse_cli_mode(argv(&[])), Ok(CliMode::Watcher));
    }

    #[test]
    fn parse_cli_mode_bootstrap_plain() {
        assert_eq!(
            parse_cli_mode(argv(&["--bootstrap"])),
            Ok(CliMode::Bootstrap { dry_run: false })
        );
    }

    #[test]
    fn parse_cli_mode_bootstrap_dry_run_either_order() {
        assert_eq!(
            parse_cli_mode(argv(&["--bootstrap", "--dry-run"])),
            Ok(CliMode::Bootstrap { dry_run: true })
        );
        assert_eq!(
            parse_cli_mode(argv(&["--dry-run", "--bootstrap"])),
            Ok(CliMode::Bootstrap { dry_run: true })
        );
    }

    #[test]
    fn parse_cli_mode_print_ct_tables() {
        assert_eq!(
            parse_cli_mode(argv(&["--print-ct-tables"])),
            Ok(CliMode::PrintCtTables)
        );
    }

    #[test]
    fn parse_cli_mode_dry_run_alone_is_rejected() {
        // The regression that motivated this: --dry-run without --bootstrap
        // must NOT silently become a watcher (or worse, be ignored on a write
        // path). It is a hard error.
        let err = parse_cli_mode(argv(&["--dry-run"])).unwrap_err();
        assert!(err.contains("--dry-run only applies to --bootstrap"), "{err}");
    }

    #[test]
    fn parse_cli_mode_unknown_flag_is_rejected() {
        // The footgun: a typo'd or unsupported flag must fail loud, never be
        // ignored (which previously let `--bootstrap --dyr-run` run a full write).
        let err = parse_cli_mode(argv(&["--bootstrap", "--dyr-run"])).unwrap_err();
        assert!(err.contains("unrecognized argument"), "{err}");
        assert!(err.contains("--dyr-run"), "{err}");
    }

    #[test]
    fn parse_cli_mode_print_ct_tables_is_exclusive() {
        assert!(parse_cli_mode(argv(&["--print-ct-tables", "--bootstrap"])).is_err());
        assert!(parse_cli_mode(argv(&["--print-ct-tables", "--dry-run"])).is_err());
    }

    // ========================================================================
    // Resilience PR R1 — error taxonomy + last_error sanitization
    // ========================================================================

    /// Every event_name in the registry follows the `sync.<snake_case>`
    /// shape. Locks the format so a log-aggregation rule that matches on
    /// the `sync.` prefix can rely on coverage. (Operators in the
    /// 2026-05-14 post-mortem proposed using `event_name =~ /^sync\./`
    /// as the Loki dashboard filter.)
    #[test]
    fn known_sync_event_names_use_sync_prefix() {
        for name in KNOWN_SYNC_EVENT_NAMES {
            assert!(
                name.starts_with("sync."),
                "event_name `{name}` must start with `sync.` so dashboards \
                 can filter on it; got: {name}"
            );
            assert!(
                name.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c == '.'),
                "event_name `{name}` must be lowercase snake.case ASCII \
                 (greppable from any locale)"
            );
        }
    }

    /// Registry must enumerate every `EV_*` constant. The polish here is
    /// detecting a NEW event_name added at a call site but forgotten in
    /// the registry — the existing audit `include_str!` test would miss
    /// that because grep'ing `EV_` matches the call-site mentions.
    #[test]
    fn known_sync_event_names_match_source_constants() {
        // We can't reflect on `pub const` names at runtime, so the
        // counter-test is: registry contains the constants we know
        // about. Adding a new EV_ implies adding it to the registry,
        // which this test mechanically enforces by counting unique
        // entries.
        let unique: std::collections::HashSet<&str> =
            KNOWN_SYNC_EVENT_NAMES.iter().copied().collect();
        assert_eq!(
            unique.len(),
            KNOWN_SYNC_EVENT_NAMES.len(),
            "KNOWN_SYNC_EVENT_NAMES has duplicate entries — each event_name \
             must appear exactly once"
        );
        // Sanity floor — if a refactor accidentally deletes the array we
        // want the test to fail loudly.
        assert!(
            KNOWN_SYNC_EVENT_NAMES.len() >= 15,
            "registry shrank suspiciously to {} entries; expected ≥15",
            KNOWN_SYNC_EVENT_NAMES.len()
        );
    }

    /// `sanitize_last_error` passes through a plain ASCII summary
    /// unchanged. Establishes the happy-path baseline so the
    /// subsequent edge-case tests can assert deviations.
    #[test]
    fn sanitize_last_error_passes_through_plain_ascii() {
        let raw = "Timed out in bb8";
        let out = sanitize_last_error(raw);
        assert_eq!(out, raw);
    }

    /// Interior `"` must be escaped so a downstream JSON-emitter
    /// (Loki, our `/api/new/sync/status` route) doesn't see an
    /// unterminated string. Defensive against the prompt-injection
    /// vector flagged in the 2026-05-14 post-mortem (an MSSQL error
    /// message could echo attacker-controlled bytes from a malformed
    /// upstream value).
    #[test]
    fn sanitize_last_error_escapes_double_quotes() {
        let raw = r#"bad row: name="Robert""#;
        let out = sanitize_last_error(raw);
        assert!(
            out.contains(r#"\""#),
            "interior `\"` must be backslash-escaped; got: {out}"
        );
        assert!(!out.contains(r#"name="Robert""#));
    }

    /// Newlines collapse to `\n` so a multi-line tiberius backtrace
    /// doesn't shred the dashboard layout. Same rationale as `"` —
    /// a single `last_error` column should render on one line.
    #[test]
    fn sanitize_last_error_escapes_newlines_and_tabs() {
        let raw = "line1\nline2\twith tab\r\n";
        let out = sanitize_last_error(raw);
        assert!(out.contains("\\n"));
        assert!(out.contains("\\t"));
        assert!(out.contains("\\r"));
        // None of the raw control chars should survive.
        assert!(!out.contains('\n'));
        assert!(!out.contains('\t'));
        assert!(!out.contains('\r'));
    }

    /// Other C0 control chars (e.g. `0x07` BEL, `0x1B` ESC) render as
    /// `\u00XX` rather than passing through and risking a dashboard
    /// terminal interpreting them. The 2026-05-14 post-mortem
    /// specifically flagged ANSI-escape injection as a concern.
    #[test]
    fn sanitize_last_error_escapes_other_control_chars() {
        let raw = "alert\x07esc\x1bdone";
        let out = sanitize_last_error(raw);
        assert!(out.contains("\\u0007"), "BEL must be \\u-escaped: {out}");
        assert!(out.contains("\\u001b"), "ESC must be \\u-escaped: {out}");
        assert!(!out.contains('\x07'));
        assert!(!out.contains('\x1b'));
    }

    /// `LAST_ERROR_MAX_LEN` enforces a hard cap so a multi-MB tiberius
    /// error can't bloat the row. The cap counts CHARS (not bytes) so
    /// multibyte glyphs survive intact.
    #[test]
    fn sanitize_last_error_truncates_at_cap() {
        let raw: String = "x".repeat(LAST_ERROR_MAX_LEN * 4);
        let out = sanitize_last_error(&raw);
        let char_count = out.chars().count();
        assert!(
            char_count <= LAST_ERROR_MAX_LEN,
            "sanitized length {char_count} exceeds LAST_ERROR_MAX_LEN {LAST_ERROR_MAX_LEN}"
        );
        // Truncation marker should be present so the operator knows the
        // payload was cut.
        assert!(
            out.ends_with('…'),
            "truncated payload must end with `…` marker; got tail: {}",
            &out[out.len().saturating_sub(8)..]
        );
    }

    /// Truncation must not split a multibyte UTF-8 codepoint in half —
    /// `String::truncate` would panic on a non-char-boundary, but our
    /// chars() iteration approach should be safe. This test pins it.
    #[test]
    fn sanitize_last_error_truncates_at_char_boundary_for_multibyte() {
        // Thai script — 3 bytes per character. Fill past the cap.
        let raw: String = "ก".repeat(LAST_ERROR_MAX_LEN * 2);
        let out = sanitize_last_error(&raw);
        // No panic; output is valid UTF-8 (Rust guarantees String) and
        // under cap.
        assert!(out.chars().count() <= LAST_ERROR_MAX_LEN);
    }

    /// `format_last_error` prepends the event_name with a `:` separator
    /// — operators can `grep ^sync.foo` in the dashboard / `psql` and
    /// see every row that hit that failure mode.
    #[test]
    fn format_last_error_prepends_event_name() {
        let out = format_last_error(EV_WATERMARK_ADVANCE_FAIL, "Timed out in bb8");
        assert!(
            out.starts_with("sync.watermark_advance_fail: "),
            "must start with `<event>: ` prefix; got: {out}"
        );
        assert!(out.contains("Timed out in bb8"));
    }

    /// Even with a long upstream error, `format_last_error` stays
    /// under the cap (the inner `sanitize_last_error` truncates; this
    /// test guards against a future refactor that builds the string
    /// without re-running sanitization).
    #[test]
    fn format_last_error_respects_cap_even_for_huge_payload() {
        let raw: String = "y".repeat(LAST_ERROR_MAX_LEN * 4);
        let out = format_last_error(EV_LOAD_AGGREGATE_FAIL, &raw);
        assert!(out.chars().count() <= LAST_ERROR_MAX_LEN);
    }

    /// All error-emission sites in the tick path must attach an
    /// `event_name = EV_...` attribute. Locks in the structured-log
    /// contract so a future refactor can't silently regress to
    /// free-form `tracing::error!("…")`.
    ///
    /// Strategy: every `tracing::error!` call inside the tick path
    /// (between `run_one_tick` and the end of `poll_table`) must
    /// reference one of the known constants. The source-text test
    /// is necessarily approximate but catches the common regression
    /// shape.
    #[test]
    fn every_tick_path_tracing_error_references_an_event_name() {
        let source = include_str!("sync.rs");
        let tick_start = source
            .find("async fn run_one_tick(")
            .expect("run_one_tick must exist");
        let tick_end_marker = "async fn fetch_ct_rows(";
        let tick_end = source[tick_start..]
            .find(tick_end_marker)
            .map(|i| tick_start + i)
            .unwrap_or(source.len());
        let region = &source[tick_start..tick_end];

        // Count `tracing::error!(` openings in the region.
        let error_macro_count = region.matches("tracing::error!(").count();
        // Count `event_name` attribute mentions in the same region.
        // Accept both the literal `event_name = EV_FOO` and the shorthand
        // `event_name,` (used when a local variable already holds the
        // resolved constant, e.g. the retention discriminator).
        let event_name_eq_count = region.matches("event_name = EV_").count();
        let event_name_short_count = region.matches("event_name,").count();
        let event_name_attr_count = event_name_eq_count + event_name_short_count;

        assert!(
            error_macro_count >= 5,
            "expected ≥5 tracing::error! sites in tick path; found {error_macro_count} \
             — region may have been refactored away from this file"
        );
        assert!(
            event_name_attr_count >= error_macro_count,
            "every tracing::error! in the tick path must include an `event_name = EV_…` \
             attribute (or shorthand `event_name,` binding a local EV_* constant); \
             got {error_macro_count} error macros vs {event_name_attr_count} \
             event_name attributes. The shortfall is the regression."
        );
    }

    /// Every `record_table_error` call site must pass an `EV_*`
    /// constant. The function's contract is "persist a FAILURE MODE",
    /// and the post-mortem's whole point is that a raw-string payload
    /// loses the mode. Lock the call shape.
    #[test]
    fn every_record_table_error_call_passes_an_event_const() {
        let source = include_str!("sync.rs");
        // Count call sites (skip doc comments / strings — but the
        // matcher is precise enough: `record_table_error(pg,`).
        let call_count = source.matches("record_table_error(pg,").count();
        // Of those, how many pair with an `EV_` constant nearby? Search
        // both backward (the retention path declares
        // `let event_name = if … { EV_FOO } else { EV_BAR };` just above
        // the call) AND forward (most call sites pass `EV_*` inline) so
        // either pattern counts.
        let mut paired = 0;
        for (idx, _) in source.match_indices("record_table_error(pg,") {
            let window_start = idx.saturating_sub(400);
            let window_end = (idx + 240).min(source.len());
            let window = &source[window_start..window_end];
            if window.contains("EV_") {
                paired += 1;
            }
        }
        assert!(
            call_count >= 8,
            "expected ≥8 record_table_error call sites in the tick path; \
             got {call_count}"
        );
        assert_eq!(
            paired, call_count,
            "every record_table_error(pg, …) call must reference an EV_ constant; \
             {paired}/{call_count} did. The shortfall is the regression."
        );
    }

    // ========================================================================
    // Silent-drop fix — CT watermark decision after a per-table tick.
    //
    // The 2026-05-11..15 production incident: `new-hotel-production-sync-1`
    // was killed 30+ times mid-tick during deploys; per-key aggregate
    // applies failed transiently; the watermark advanced past the failed
    // keys' SYS_CHANGE_VERSION; the CT row aged out of MSSQL's 2-day
    // retention; canonical PG silently lagged. These tests pin the
    // hold-on-error contract so the regression can't sneak back.
    // ========================================================================

    /// Happy path: all keys applied cleanly, batch saw a new
    /// `max_version`. Advance to `max_version`.
    #[test]
    fn next_watermark_after_tick_advances_to_max_when_no_error() {
        assert_eq!(
            next_watermark_after_tick(100, 90, false),
            Some(100),
            "successful tick with progress must advance to max_version"
        );
    }

    /// The bug fix: ANY per-key failure pins the watermark at
    /// `last_seen` so the next tick re-fetches the same CT rows. This
    /// is the exact case the 28 stuck `ht_reconcile_log` rows on HF
    /// Hotel were created by — fixing this one assertion is the whole
    /// point of the PR.
    #[test]
    fn next_watermark_after_tick_holds_at_last_seen_on_error() {
        assert_eq!(
            next_watermark_after_tick(100, 90, true),
            None,
            "errored tick MUST hold watermark — re-fetch is the retry mechanism"
        );
    }

    /// Even a wildly-advanced `max_version` must not leak past the
    /// hold when `errored` is true. Defends against a future
    /// "optimization" that tries to advance to `max_version - 1` on
    /// error (which would still drop the failed key's event).
    #[test]
    fn next_watermark_after_tick_holds_even_when_max_version_is_far_ahead() {
        assert_eq!(
            next_watermark_after_tick(10_000, 5, true),
            None,
            "no advance permitted on error regardless of how far max_version moved"
        );
    }

    /// Success but no version progress (all rows were stale /
    /// coalesced away with no key delta) — no advance. Mirrors the
    /// pre-fix `max_version > last_seen` guard.
    #[test]
    fn next_watermark_after_tick_no_advance_when_no_progress() {
        assert_eq!(
            next_watermark_after_tick(90, 90, false),
            None,
            "max_version == last_seen → no advance"
        );
    }

    /// Defensive: `max_version < last_seen` shouldn't happen in
    /// practice (the per-table loop only ratchets `max_version`
    /// upward), but if it does — e.g. a future refactor accidentally
    /// reset the local — we must NOT regress the watermark. The
    /// previous gate (`max_version > last_seen`) already covered this;
    /// the helper preserves the semantic.
    #[test]
    fn next_watermark_after_tick_no_advance_when_max_below_last_seen() {
        assert_eq!(
            next_watermark_after_tick(50, 100, false),
            None,
            "max_version < last_seen must not advance (no regression of watermark)"
        );
    }

    /// Combined: error + no progress — still hold. Trivially follows
    /// from the rules but worth a test so the truth table is complete.
    #[test]
    fn next_watermark_after_tick_holds_on_error_with_no_progress() {
        assert_eq!(next_watermark_after_tick(90, 90, true), None);
    }

    #[test]
    fn parse_allowlist_returns_none_when_unset() {
        assert!(parse_allowlist(None).is_none());
    }

    #[test]
    fn parse_allowlist_returns_none_when_blank() {
        assert!(parse_allowlist(Some(String::new())).is_none());
        assert!(parse_allowlist(Some("   ".into())).is_none());
        assert!(parse_allowlist(Some(",,,".into())).is_none());
    }

    #[test]
    fn parse_allowlist_splits_and_trims() {
        let set = parse_allowlist(Some(" HT_Customers , HT_Rooms ,HT_Book_H ".into()))
            .expect("non-empty allowlist returns Some");
        assert!(set.contains("HT_Customers"));
        assert!(set.contains("HT_Rooms"));
        assert!(set.contains("HT_Book_H"));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn build_mappers_no_allowlist_returns_all_enabled_tables() {
        let mappers = build_mappers(&None);
        assert_eq!(mappers.len(), CT_ENABLED_TABLES.len());
        assert_eq!(
            mappers.len(),
            19,
            "19 CT-enabled tables expected (10 canonical + 6 legacy_mirror \
             + 2 Track-E1: HT_CheckIn_Other_People + HT_Rooms_Cancel \
             + 1 Phase 5/E2: HT_Book_Pro)"
        );
    }

    #[test]
    fn build_mappers_filters_by_allowlist() {
        let mut allow = HashSet::new();
        allow.insert("HT_Customers".to_string());
        allow.insert("HT_Rooms".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 2);
    }

    /// Phase 5.2: customer + room are now real mappers; the rest are
    /// still NoopMapper. The mapper for `HT_Customers` must not be a
    /// NoopMapper anymore — locks the wiring so a refactor doesn't
    /// silently regress the customer mapper to no-op.
    #[test]
    fn build_mappers_wires_customer_to_customer_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Customers".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        // The CustomerMapper has primary_key_cols == &["id"]; NoopMapper
        // has &[]. Use that as the structural assertion.
        assert_eq!(
            mappers[0].primary_key_cols(),
            &["id"],
            "HT_Customers must be wired to CustomerMapper, not NoopMapper"
        );
    }

    #[test]
    fn build_mappers_wires_rooms_to_room_master_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Rooms".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        assert!(mappers[0].select_sql().contains("Room_Clean"));
    }

    /// Track F1 (audit 2026-05-13 T1 HIGH-4) — `HT_Room_Status` is
    /// wired to the new `RoomCalendarMapper` (projects to canonical
    /// `ht_room_calendar`) instead of the retired 5.4 stub
    /// `RoomStatusMapper`. The mapper must project the columns
    /// needed for the per-night ledger (`room_date`, `room_status`,
    /// `room_no`, plus the `room_Book_No` / `room_CheckIn_No` /
    /// `room_Details` FK + label trio).
    #[test]
    fn build_mappers_wires_room_status_to_room_calendar_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Room_Status".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        let select = mappers[0].select_sql();
        for col in &[
            "room_no",
            "room_date",
            "room_status",
            "room_Details",
            "room_Book_No",
            "room_CheckIn_No",
        ] {
            assert!(
                select.contains(col),
                "RoomCalendarMapper SELECT must project {col}; got: {select}"
            );
        }
    }

    /// Phase 5.3: HT_Book_H + HT_Book_Ds + HT_Book_Date are now real
    /// mappers (BookingHeaderMapper / BookingRoomsMapper /
    /// BookingDatesMapper). Locks the wiring so a refactor doesn't
    /// silently regress them to NoopMapper.
    #[test]
    fn build_mappers_wires_booking_header_to_booking_header_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_H".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(
            mappers[0].primary_key_cols(),
            &["Book_ID"],
            "HT_Book_H must be wired to BookingHeaderMapper (PK=Book_ID), not NoopMapper"
        );
    }

    #[test]
    fn build_mappers_wires_booking_ds_to_booking_rooms_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_Ds".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        assert!(mappers[0].select_sql().contains("Book_Room_Type"));
    }

    #[test]
    fn build_mappers_wires_booking_date_to_booking_dates_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_Date".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        assert!(mappers[0].select_sql().contains("Book_ok"));
    }

    /// 5.4 wired the four checkin/receipt tables to real mappers.
    /// Locks the wiring so a refactor can't silently regress them
    /// back to NoopMapper.
    #[test]
    fn build_mappers_wires_5_4_tables_to_real_mappers() {
        let cases: &[(&str, &[&str])] = &[
            ("HT_CheckIn_H", &["Cin_no"]),
            ("HT_CheckIn_Ds", &["id"]),
            ("HT_CheckIn_Pay", &["id"]),
            ("HT_Receipt_H", &["id"]),
        ];
        for (t, expected_pk) in cases {
            let mut allow = HashSet::new();
            allow.insert((*t).to_string());
            let mappers = build_mappers(&Some(allow));
            assert_eq!(mappers.len(), 1, "{t}: expected one mapper");
            assert_eq!(
                mappers[0].primary_key_cols(),
                *expected_pk,
                "{t} must be wired to its real 5.4 mapper, not NoopMapper"
            );
        }
    }

    /// Track E1 (audit 2026-05-13) — `HT_CheckIn_Other_People` +
    /// `HT_Rooms_Cancel` get real mappers. Locks the wiring so a
    /// refactor can't silently regress them to NoopMapper (which would
    /// re-open the TM.30 stale-registry bug and the dangling-CT
    /// retention leak respectively).
    #[test]
    fn build_mappers_wires_track_e1_tables_to_real_mappers() {
        let cases: &[(&str, &[&str], &str)] = &[
            ("HT_CheckIn_Other_People", &["id"], "Cin_contry"),
            ("HT_Rooms_Cancel", &["id"], "cancel_note"),
        ];
        for (t, expected_pk, projection_marker) in cases {
            let mut allow = HashSet::new();
            allow.insert((*t).to_string());
            let mappers = build_mappers(&Some(allow));
            assert_eq!(mappers.len(), 1, "{t}: expected one mapper");
            assert_eq!(
                mappers[0].primary_key_cols(),
                *expected_pk,
                "{t} must be wired to its real Track-E1 mapper, not NoopMapper"
            );
            assert!(
                mappers[0].select_sql().contains(projection_marker),
                "{t} projection must include {projection_marker}; \
                 got: {}",
                mappers[0].select_sql()
            );
        }
    }

    /// Phase 5/E2 (coexistence audit 2026-06-11 P2) — `HT_Book_Pro`
    /// gets a real mirror mapper. Locks the wiring so a refactor can't
    /// silently regress it to NoopMapper (which would re-open the
    /// invisible-booking-products gap).
    #[test]
    fn build_mappers_wires_book_pro_to_mirror_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_Pro".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1, "HT_Book_Pro: expected one mapper");
        assert_eq!(
            mappers[0].primary_key_cols(),
            &["id"],
            "HT_Book_Pro must be wired to BookProMirrorMapper, not NoopMapper"
        );
        assert!(
            mappers[0].select_sql().contains("B_PRICE_TOTAL"),
            "HT_Book_Pro projection must include B_PRICE_TOTAL; got: {}",
            mappers[0].select_sql()
        );
    }

    #[test]
    fn ct_enabled_tables_match_migration_017_022_033_and_056_seed() {
        let expected = [
            // Phase 5 — canonical sync (migration 017)
            "HT_Customers",
            "HT_Rooms",
            "HT_Room_Status",
            "HT_Book_H",
            "HT_Book_Ds",
            "HT_Book_Date",
            "HT_CheckIn_H",
            "HT_CheckIn_Ds",
            "HT_CheckIn_Pay",
            "HT_Receipt_H",
            // Phase 5.5b — legacy_mirror.* (migration 022)
            "HT_Cupon",
            "HT_CheckIn_Product",
            "HT_Deposit",
            "HT_Changed_Room",
            "HT_Bill_Debt_H",
            "HT_Bill_Debt_Ds",
            // Phase 5/E1 — Track E1 sync-gap closure (migration 033)
            "HT_CheckIn_Other_People",
            "HT_Rooms_Cancel",
            // Phase 5/E2 — pre-booked products mirror (migration 056)
            "HT_Book_Pro",
        ];
        assert_eq!(CT_ENABLED_TABLES, &expected);
    }

    #[test]
    fn ct_gate_flags_only_confirmed_off_tables() {
        // Some(false) → missing; Some(true) → fine; None (probe errored) →
        // transient, must NOT be flagged (a connectivity blip can't refuse
        // startup). Order preserved.
        let probes = [
            ("HT_Customers", Some(true)),
            ("HT_Book_Pro", Some(false)),
            ("HT_Rooms", None),
            ("HT_Deposit", Some(false)),
        ];
        assert_eq!(
            ct_tables_definitely_missing(&probes),
            vec!["HT_Book_Pro", "HT_Deposit"]
        );
    }

    #[test]
    fn ct_gate_all_healthy_or_transient_flags_nothing() {
        let probes = [
            ("HT_Customers", Some(true)),
            ("HT_Rooms", None), // transient probe failure — not a refusal
        ];
        assert!(ct_tables_definitely_missing(&probes).is_empty());
    }

    #[test]
    fn extract_projection_columns_strips_t_prefix() {
        let cols = extract_projection_columns(
            "t.Cust_no, t.Cust_name, t.Cust_Add_tel",
        );
        assert_eq!(cols, vec!["Cust_no", "Cust_name", "Cust_Add_tel"]);
    }

    #[test]
    fn extract_projection_columns_tolerates_whitespace_and_trailing_comma() {
        let cols = extract_projection_columns(
            "  t.id,  t.Room_no ,  t.Room_Type ,",
        );
        assert_eq!(cols, vec!["id", "Room_no", "Room_Type"]);
    }

    #[test]
    fn extract_projection_columns_skips_non_t_qualified_entries() {
        let cols = extract_projection_columns(
            "t.id, ct.SYS_CHANGE_VERSION AS v, t.Room_no",
        );
        assert_eq!(cols, vec!["id", "Room_no"]);
    }

    /// Regression: on D rows the LEFT JOIN nulls the projection's
    /// `t.<pk>` cell while CT's `pk_<pk>` alias stays populated. The
    /// PK loop MUST run after the projection loop so it overwrites
    /// the projection's NULL with the real CT-side PK value;
    /// otherwise mirror mappers (whose PK column is also projected)
    /// crash with "PK NULL — should not happen" on every D event.
    /// Discovered 2026-04-29 in the codebase audit; fix is the loop
    /// reorder in `build_materialised_row`.
    #[test]
    fn d_row_pk_survives_null_projection_overwrite() {
        use hotel_backend::sync::row::test_support::MockValue;
        use std::collections::HashMap;

        // Simulate the tiberius row a D event produces for HT_Cupon:
        // pk_cupon_no is populated (CT-side authoritative PK), the
        // projection's plain "cupon_no" alias is NULL (LEFT JOIN
        // nulled the row), and other projected columns are NULL too.
        let mut cells: HashMap<&'static str, MockValue> = HashMap::new();
        cells.insert("pk_cupon_no", MockValue::I32(12345));
        // Note: "cupon_no" (without pk_) is NOT inserted — read() returns None,
        // which the projection loop records as MockValue::Null.

        let projection: Vec<String> = vec![
            "cupon_no".into(),
            "cupon_cin_no".into(),
            "cupon_cin_room".into(),
        ];
        let h = build_materialised_row(&["cupon_no"], &projection, |col| {
            cells.get(col).cloned()
        });

        // Post-fix: the PK loop runs after projection and the real
        // CT-side `12345` survives.
        match h.cells.get("cupon_no") {
            Some(MockValue::I32(12345)) => {}
            other => panic!(
                "expected cupon_no to be I32(12345) (PK from CT alias), got {other:?}"
            ),
        }
        // Other projection columns stay NULL (D-row JOIN behavior).
        assert!(matches!(h.cells.get("cupon_cin_no"), Some(MockValue::Null)));
        assert!(matches!(h.cells.get("cupon_cin_room"), Some(MockValue::Null)));
    }

    /// I/U rows must continue to work — projection writes the actual
    /// `t.<col>` value and the PK loop overwrites with the same value
    /// (CT-side and table-side agree on PK on I/U).
    #[test]
    fn iu_row_pk_value_consistent_after_loop_swap() {
        use hotel_backend::sync::row::test_support::MockValue;
        use std::collections::HashMap;

        let mut cells: HashMap<&'static str, MockValue> = HashMap::new();
        cells.insert("pk_cupon_no", MockValue::I32(7777));
        cells.insert("cupon_no", MockValue::I32(7777)); // table-side projection
        cells.insert("cupon_cin_no", MockValue::Str("CH26-005258".into()));

        let projection: Vec<String> = vec!["cupon_no".into(), "cupon_cin_no".into()];
        let h = build_materialised_row(&["cupon_no"], &projection, |col| {
            cells.get(col).cloned()
        });

        assert!(matches!(h.cells.get("cupon_no"), Some(MockValue::I32(7777))));
        match h.cells.get("cupon_cin_no") {
            Some(MockValue::Str(s)) => assert_eq!(s, "CH26-005258"),
            other => panic!("unexpected cupon_cin_no: {other:?}"),
        }
    }

    /// The CT polling queries must NOT filter on `SYS_CHANGE_CONTEXT`
    /// (2026-06-11, audit P1 #5). The historical predicate
    /// `SYS_CHANGE_CONTEXT <> 0x4E48` was inert — `SET CONTEXT_INFO`
    /// never populates `SYS_CHANGE_CONTEXT` (only the per-statement
    /// `WITH CHANGE_TRACKING_CONTEXT` hint does, which nothing uses) —
    /// and re-instating it alongside that hint would CREATE June-3-style
    /// loss, because CT coalesces per-PK to the latest change's context
    /// and would eat genuine iHOTEL edits racing a writeback touch.
    /// Echo absorption is mapper idempotency; this test keeps the SQL
    /// honest about that.
    #[test]
    fn ct_queries_carry_no_sys_change_context_filter() {
        let changes = build_ct_changes_sql(
            "HT_Customers",
            &["id"],
            "t.Cust_no, t.Cust_name",
            42,
        )
        .expect("valid mapper config must build");
        assert!(
            !changes.contains("SYS_CHANGE_CONTEXT"),
            "CHANGES query must not filter on SYS_CHANGE_CONTEXT: {changes}"
        );
        assert!(
            changes.contains("ORDER BY ct.SYS_CHANGE_VERSION ASC"),
            "monotonic ordering must survive the filter removal: {changes}"
        );
        assert!(changes.contains("CHANGETABLE(CHANGES HT_Customers, 42)"));

        let count = build_ct_count_sql("HT_Customers", 42);
        assert!(
            !count.contains("SYS_CHANGE_CONTEXT"),
            "COUNT query must not filter on SYS_CHANGE_CONTEXT: {count}"
        );
        assert!(count.contains("CHANGETABLE(CHANGES HT_Customers, 42)"));
    }

    /// Empty PK list must refuse to build (NoopMapper short-circuits
    /// upstream; reaching here without PKs is a config bug).
    #[test]
    fn ct_changes_sql_requires_primary_keys() {
        assert!(build_ct_changes_sql("HT_Customers", &[], "t.Cust_no", 0).is_err());
    }

    /// The writeback session tag survives as a session-observability
    /// marker (`sys.dm_exec_sessions.context_info`), NOT as a CT
    /// filter input. This test pins that the dispatcher still sets it
    /// (operators rely on it for session triage) while the watcher no
    /// longer pretends to consume it.
    #[test]
    fn writeback_session_tag_still_set_for_observability() {
        let dispatcher_src = include_str!("../writeback/dispatcher.rs");
        let mssql_session_src = include_str!("../db/mssql_session.rs");
        assert!(dispatcher_src.contains("set_context_info(conn)"));
        assert!(
            mssql_session_src.contains("SET CONTEXT_INFO 0x4E48"),
            "mssql_session::set_context_info must keep issuing the 0x4E48 \
             session tag (observability via sys.dm_exec_sessions)"
        );
    }

    /// Audit finding N1 (Phase 5.5 codebase audit, 2026-04-29) — pin
    /// the live-bootstrap refusal message so operators triaging Slack
    /// alerts and following docs/runbook-sync.md can rely on the
    /// wording. The runbook cross-references the env var name; if the
    /// message drifts, the operator playbook silently rots.
    #[test]
    fn live_bootstrap_refusal_names_the_override_env_var() {
        let msg = build_live_bootstrap_refusal_message();
        assert!(
            msg.contains("LEGACY_SYNC_ENABLED=true"),
            "refusal must explain the live-deployment trigger"
        );
        assert!(
            msg.contains("LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP"),
            "refusal must name the override env var so operators can find it"
        );
        assert!(
            msg.contains("--bootstrap"),
            "refusal must reference the operator action being refused"
        );
        assert!(
            msg.contains("docs/runbook-sync.md"),
            "refusal must point at the runbook for full procedure"
        );
    }

    /// The refusal must explain WHY (race window) so operators don't
    /// just slap the override on without understanding the risk.
    #[test]
    fn live_bootstrap_refusal_explains_the_race() {
        let msg = build_live_bootstrap_refusal_message();
        assert!(
            msg.contains("DELETE") && msg.contains("clobber"),
            "refusal must explain the snapshot-DELETE-vs-CT-UPSERT race"
        );
    }

    /// Regression: the empty-fetch path in `poll_table` must call
    /// `bump_skipped(.., 0, false)` before its early return so a prior
    /// `last_error` (e.g. transient bb8 timeout from a tunnel flap)
    /// gets cleared on the next successful 0-row fetch. Without this,
    /// low-traffic / empty tables (HF Ville's HT_Cupon, HT_Deposit,
    /// HT_Bill_Debt_*, HT_Receipt_H — all 0-row or CT-history-empty
    /// on Ville post-Phase-5.5b) accumulate `consecutive_failures`
    /// indefinitely and never recover their healthy status, even
    /// though the watcher is in fact polling them successfully.
    /// Discovered 2026-05-09; fix in v2.58.3.
    #[test]
    fn empty_fetch_clears_error_via_bump_skipped() {
        let source = include_str!("sync.rs");
        // Slice the file at the empty-rows guard so the assertion only
        // sees the relevant region (avoids false positives from the
        // NoopMapper-path bump_skipped a few dozen lines earlier).
        let marker = "if rows.is_empty() {";
        let idx = source
            .find(marker)
            .expect("poll_table must contain the empty-rows guard");
        let region_end = idx
            + source[idx..]
                .find("return Ok(())")
                .expect("empty-rows guard must early-return")
            + "return Ok(())".len();
        let region = &source[idx..region_end];
        assert!(
            region.contains("bump_skipped(pg, table, 0, false)"),
            "empty-fetch path must clear last_error / consecutive_failures \
             via bump_skipped(.., 0, false) before returning, otherwise \
             low-traffic tables stay stuck on stale bb8-timeout errors. \
             Got region:\n{region}"
        );
    }

    /// v2.63.0: exponential backoff schedule for MSSQL pool-init retry.
    /// Pins the expected progression (5s → 10s → 20s → 40s → 60s → 60s)
    /// so a future refactor can't silently change the cadence and
    /// either page operators too aggressively or let an outage stall
    /// the watcher quietly.
    #[test]
    fn next_backoff_doubles_until_capped() {
        let max = Duration::from_secs(60);
        let mut current = Duration::from_secs(5);

        let expected_schedule: &[u64] = &[10, 20, 40, 60, 60, 60];
        for (i, expected) in expected_schedule.iter().enumerate() {
            current = next_backoff(current, max);
            assert_eq!(
                current.as_secs(),
                *expected,
                "step {i}: expected {expected}s, got {}s",
                current.as_secs()
            );
        }
    }

    /// `saturating_mul(2)` plus the explicit cap means even a
    /// pathological starting value at the boundary can't overflow.
    #[test]
    fn next_backoff_saturates_on_giant_input() {
        let max = Duration::from_secs(60);
        let huge = Duration::from_secs(u64::MAX / 4);
        let capped = next_backoff(huge, max);
        assert_eq!(capped, max);
    }

    /// The cap also clamps a `current` already at the max — the loop
    /// never amplifies past `max_backoff`.
    #[test]
    fn next_backoff_stays_at_max_once_reached() {
        let max = Duration::from_secs(60);
        let at_max = Duration::from_secs(60);
        assert_eq!(next_backoff(at_max, max), max);
    }

    /// Pre-cap doubling: 5→10, 10→20, 20→40 — exact, no rounding.
    #[test]
    fn next_backoff_pre_cap_is_exact_doubling() {
        let max = Duration::from_secs(60);
        assert_eq!(next_backoff(Duration::from_secs(5), max).as_secs(), 10);
        assert_eq!(next_backoff(Duration::from_secs(10), max).as_secs(), 20);
        assert_eq!(next_backoff(Duration::from_secs(20), max).as_secs(), 40);
    }

    /// `InitRetryConfig::from_env` falls back to the documented defaults
    /// when no env var is set. Guards against a typo in the env-var
    /// name silently dropping operator overrides.
    #[test]
    fn init_retry_config_defaults_match_documented_constants() {
        // Snapshot the env vars and clear them so the test is
        // independent of the developer's shell state. SAFETY: tests in
        // this module run with `cargo test` which serialises them when
        // they manipulate process-global state via `--test-threads=1`
        // is NOT required because we only touch env vars that don't
        // collide with other tests in this file.
        let keys = [
            "LEGACY_SYNC_INIT_RETRY_INITIAL_SECS",
            "LEGACY_SYNC_INIT_RETRY_MAX_SECS",
            "LEGACY_SYNC_INIT_RETRY_ALERT_AFTER_SECS",
        ];
        let saved: Vec<(&'static str, Option<String>)> =
            keys.iter().map(|k| (*k, env::var(k).ok())).collect();
        for k in &keys {
            env::remove_var(k);
        }

        let cfg = InitRetryConfig::from_env();
        assert_eq!(
            cfg.initial_backoff,
            Duration::from_secs(DEFAULT_INIT_RETRY_INITIAL_SECS),
        );
        assert_eq!(
            cfg.max_backoff,
            Duration::from_secs(DEFAULT_INIT_RETRY_MAX_SECS),
        );
        assert_eq!(
            cfg.alert_after,
            Duration::from_secs(DEFAULT_INIT_RETRY_ALERT_AFTER_SECS),
        );

        // Restore so a follow-up test relying on these env vars sees
        // the original developer-shell state.
        for (k, v) in saved {
            match v {
                Some(val) => env::set_var(k, val),
                None => env::remove_var(k),
            }
        }
    }

    /// Documented schedule sanity check — the default config plus the
    /// `next_backoff` schedule should fire the alert within ~10 retry
    /// attempts so an operator gets paged inside roughly 5-6 minutes.
    /// If somebody bumps either knob without thinking, this test
    /// forces them to re-derive the alert cadence.
    ///
    /// Schedule with defaults (initial=5s, max=60s, alert=300s):
    /// attempt 1 fails (elapsed=0s) → sleep 5s
    /// attempt 2 fails (elapsed≈5s) → sleep 10s
    /// attempt 3 fails (elapsed≈15s) → sleep 20s
    /// attempt 4 fails (elapsed≈35s) → sleep 40s
    /// attempt 5 fails (elapsed≈75s) → sleep 60s
    /// attempt 6 fails (elapsed≈135s) → sleep 60s
    /// attempt 7 fails (elapsed≈195s) → sleep 60s
    /// attempt 8 fails (elapsed≈255s) → sleep 60s
    /// attempt 9 fails (elapsed≈315s) → ALERT fires (≥300s)
    ///
    /// Total time-to-page ≈ 315s = 5m15s. Acceptable for a tunnel
    /// outage where the first 5 min of retries are still in
    /// "give it a chance to recover" territory.
    #[test]
    fn default_backoff_schedule_paging_threshold_documented() {
        let max = Duration::from_secs(DEFAULT_INIT_RETRY_MAX_SECS);
        let mut current = Duration::from_secs(DEFAULT_INIT_RETRY_INITIAL_SECS);
        let mut elapsed: u64 = 0;
        let mut attempts: u32 = 1;
        let alert_threshold = DEFAULT_INIT_RETRY_ALERT_AFTER_SECS;

        // Simulate failed attempts until elapsed crosses the alert
        // threshold. Cap at 20 attempts so a buggy schedule can't loop
        // forever.
        while elapsed < alert_threshold && attempts < 20 {
            elapsed += current.as_secs();
            current = next_backoff(current, max);
            attempts += 1;
        }

        assert!(
            elapsed >= alert_threshold,
            "default schedule never reaches alert threshold inside 20 \
             attempts (elapsed={elapsed}s, threshold={alert_threshold}s)"
        );
        // Paging must happen within 10 minutes so a real outage doesn't
        // get masked by an overly conservative cadence.
        assert!(
            elapsed < 600,
            "time-to-page = {elapsed}s; should fire well under 10 min \
             (attempts taken: {attempts})"
        );
    }

    /// Integration-light check that `create_pool_with_retry` does in
    /// fact retry on an unreachable MSSQL: point at a TCP-refused port
    /// (127.0.0.1:1 — always refused on any host), wait for two
    /// attempts to fire, then abort the loop via `tokio::time::timeout`.
    /// Asserts the wrapper does NOT propagate the error (never returns
    /// Err — exact opposite of the pre-fix behaviour) and DOES log at
    /// least one retry attempt.
    ///
    /// We intentionally don't drive this past the timeout — the loop
    /// is infinite by design.
    #[tokio::test]
    async fn create_pool_with_retry_loops_instead_of_exiting() {
        let unreachable = DbConfig {
            server: "127.0.0.1".to_string(),
            // bb8's connection_timeout is 5s (R2, lowered from 15s)
            // — port 1 is unbound, so tiberius gets a TCP RST
            // immediately rather than waiting the full timeout. That
            // means we can drive two retry attempts well under the
            // 5s test budget.
            port: 1,
            database: "stub".to_string(),
            user: "stub".to_string(),
            password: "stub".to_string(),
            pool_max: 1,
        };
        let retry_cfg = InitRetryConfig {
            // Tiny backoff so the test finishes inside a few hundred ms.
            initial_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_millis(100),
            // Alert never fires in this test — way above the timeout.
            alert_after: Duration::from_secs(3600),
        };

        // The pool-init call never succeeds, so we cap the test budget
        // with a timeout. If the wrapper exited (the bug), the future
        // would resolve before the timeout with a panic from `.unwrap`;
        // if it loops correctly, the timeout fires first.
        let result = tokio::time::timeout(
            Duration::from_millis(500),
            create_pool_with_retry(
                &unreachable,
                &retry_cfg,
                None,
                "test-site",
                "test-caller",
            ),
        )
        .await;

        // The timeout MUST fire — meaning the loop is still retrying.
        assert!(
            result.is_err(),
            "create_pool_with_retry should never return on a permanently \
             unreachable MSSQL; expected timeout, got {result:?}"
        );
    }

    // -------------------------------------------------------------------
    // Track D / T7 CRIT-3 — watermark-stall watchdog
    // -------------------------------------------------------------------

    fn obs(version: i64, observed_offset_secs: u64, anchor: Instant) -> WatermarkObservation {
        WatermarkObservation {
            last_seen_version: version,
            last_polled_at: Some(chrono::Utc::now()),
            observed_at: anchor + Duration::from_secs(observed_offset_secs),
        }
    }

    /// Track D / T7 CRIT-3 — when the version advances between two
    /// polls, no alert fires (the steady-state happy path).
    #[test]
    fn watermark_stall_watchdog_no_alert_when_version_advancing() {
        let anchor = Instant::now();
        let prior = obs(100, 0, anchor);
        let current = obs(150, 60, anchor);
        let now = anchor + Duration::from_secs(60);
        let result = watermark_stall_alert_eligible(
            &prior,
            &current,
            now,
            false,
            Duration::from_secs(1800),
        );
        assert!(result.is_none(), "advancing watermark must never alert");
    }

    // -------------------------------------------------------------------
    // Quiet-aware watchdog gate (2026-05-14 incident)
    // -------------------------------------------------------------------

    /// When legacy CT is idle at the same version the watermark holds,
    /// the watchdog must NOT fire — the watcher is correctly tracking
    /// tip-of-stream, there's just nothing new to advance to. This is
    /// the canonical false-positive case from 2026-05-14 (5 pages
    /// across hfhotel + hfville during off-peak lunch/post-checkout
    /// lulls).
    #[test]
    fn watchdog_silent_when_legacy_ct_quiet() {
        let fire = should_fire_stall_alert(17209, Some(17209));
        assert!(
            !fire,
            "watermark == CT current must suppress the alert (legacy is quiet)"
        );
    }

    /// When `CHANGE_TRACKING_CURRENT_VERSION()` is ahead of the
    /// watermark, the watcher is failing to process — real stall.
    #[test]
    fn watchdog_alerts_when_legacy_ct_ahead() {
        let fire = should_fire_stall_alert(17209, Some(17250));
        assert!(
            fire,
            "CT current > watermark means the watcher is wedged — must alert"
        );
    }

    /// Probe failure / timeout must fall through to alert — uncertainty
    /// is paged, not suppressed. Matches the "conservative" requirement
    /// in the post-mortem.
    #[test]
    fn watchdog_alerts_when_probe_fails() {
        let fire = should_fire_stall_alert(17209, None);
        assert!(
            fire,
            "probe failure must fall through to alert (don't suppress on uncertainty)"
        );
    }

    /// Defensive: CT current < watermark is impossible in healthy CT
    /// (versions are monotonic), but if observed we should still alert.
    #[test]
    fn watchdog_alerts_when_probe_below_watermark() {
        let fire = should_fire_stall_alert(17209, Some(100));
        assert!(
            fire,
            "CT current < watermark is anomalous — must alert defensively"
        );
    }

    // -------------------------------------------------------------------
    // Probe-failure streak gate (2026-05-22 — 4 self-recovering
    // `:warning:` pages on hfhotel within 8h overnight, all single-tick
    // probe timeouts during quiet periods).
    // -------------------------------------------------------------------

    /// One probe failure inside the watchdog tick is dominated by
    /// transient iHOTEL lock contention — must NOT page until the
    /// signal stabilises across the threshold.
    #[test]
    fn probe_failure_streak_below_threshold_suppresses() {
        let pass = probe_failure_streak_passes_gate(None, 1, 3);
        assert!(
            !pass,
            "single probe failure must be suppressed; threshold protects against transient locks"
        );
        let pass = probe_failure_streak_passes_gate(None, 2, 3);
        assert!(!pass, "two consecutive failures still below threshold");
    }

    /// Once the streak crosses the threshold, the informational page
    /// fires. This is the canonical "real legacy outage" case (probe
    /// keeps timing out across multiple ticks).
    #[test]
    fn probe_failure_streak_at_threshold_fires() {
        let pass = probe_failure_streak_passes_gate(None, 3, 3);
        assert!(
            pass,
            "streak == threshold must page; that's the contract"
        );
    }

    /// Defensive: streaks above the threshold (e.g., between the page
    /// firing and the cooldown clearing) keep returning true so the
    /// cooldown-gated re-page logic upstream still works.
    #[test]
    fn probe_failure_streak_above_threshold_keeps_firing() {
        let pass = probe_failure_streak_passes_gate(None, 99, 3);
        assert!(
            pass,
            "streaks above threshold must keep returning true; upstream cooldown controls re-page spacing"
        );
    }

    /// A successful probe bypasses the gate entirely — the
    /// confirmed-backlog (`:rotating_light:`) branch in
    /// `should_fire_stall_alert` must fire on a single observation.
    /// The streak gate only changes behaviour for the uncertainty
    /// class.
    #[test]
    fn probe_success_bypasses_streak_gate() {
        let pass = probe_failure_streak_passes_gate(Some(17250), 0, 3);
        assert!(
            pass,
            "Some(v) must bypass the gate — confirmed backlog fires immediately"
        );
        // Even with a non-zero residual streak (shouldn't happen, but
        // defensive), a probe success should still pass.
        let pass = probe_failure_streak_passes_gate(Some(17250), 5, 3);
        assert!(pass, "Some(v) bypasses gate regardless of residual streak");
    }

    /// Setting `threshold = 1` recovers the pre-2026-05-22 hair-trigger
    /// behaviour (page on the first probe timeout). This is the escape
    /// hatch for operators who want the old behaviour and the
    /// regression test that confirms the gate doesn't accidentally
    /// suppress when configured to be permissive.
    #[test]
    fn probe_failure_streak_threshold_one_fires_immediately() {
        let pass = probe_failure_streak_passes_gate(None, 1, 1);
        assert!(
            pass,
            "threshold=1 must reproduce the pre-streak-gate hair-trigger"
        );
    }

    /// Lock the default threshold against accidental regression — the
    /// 3-tick choice is documented at the constant site and matches
    /// the 2026-05-22 incident's self-recovery window (alerts cleared
    /// in 1-3 min, so 3 ticks × 60s ≈ 3 min of patience eliminates
    /// the false-positive class entirely).
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn default_probe_timeout_streak_threshold_matches_incident_window() {
        assert_eq!(
            DEFAULT_PROBE_TIMEOUT_STREAK_THRESHOLD, 3,
            "default streak threshold should be 3 ticks (~3 min) — matches the 2026-05-22 self-recovery window"
        );
    }

    // -------------------------------------------------------------------
    // Backlog-persistence gate (2026-06-29 — self-recovering critical
    // "STUCK" pages: a long quiet window followed by a single new change
    // paged ":rotating_light: 1 version unprocessed, stuck 7807s" and
    // RECOVERED the next tick because the watcher consumed it within its
    // own poll cycle).
    // -------------------------------------------------------------------

    /// The first observation of a backlog must NOT page — the watcher
    /// very likely consumes it on its next poll. This is the exact
    /// 10:42 hfville / 3:35 hfhotel false-positive shape.
    #[test]
    fn backlog_persist_first_observation_suppresses() {
        assert!(
            !backlog_persist_passes_gate(1, DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD),
            "a single-tick backlog must be suppressed — it's normal lag, not a stall"
        );
    }

    /// Once the backlog survives the threshold number of consecutive
    /// ticks, the watcher is genuinely wedged — page critically.
    #[test]
    fn backlog_persist_at_threshold_fires() {
        assert!(
            backlog_persist_passes_gate(2, DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD),
            "a backlog persisting past the threshold is a real stall — must page"
        );
    }

    /// Streaks beyond the threshold keep returning true so the upstream
    /// cooldown (not the gate) controls re-page spacing.
    #[test]
    fn backlog_persist_above_threshold_keeps_firing() {
        assert!(
            backlog_persist_passes_gate(99, DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD),
            "streaks above threshold must keep firing; cooldown controls spacing"
        );
    }

    /// `threshold = 1` recovers the pre-2026-06-29 hair-trigger (page on
    /// first backlog observation) — the operator escape hatch and a
    /// regression guard that the gate doesn't over-suppress.
    #[test]
    fn backlog_persist_threshold_one_fires_immediately() {
        assert!(
            backlog_persist_passes_gate(1, 1),
            "threshold=1 must reproduce the pre-gate first-observation page"
        );
    }

    /// Lock the default: 2 ticks (~60s) is the minimum that outlasts the
    /// CT watcher's own poll cycle, so a backlog the watcher would have
    /// drained never pages.
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn default_backlog_persist_threshold_is_two() {
        assert_eq!(
            DEFAULT_BACKLOG_PERSIST_STREAK_THRESHOLD, 2,
            "default backlog-persistence threshold should be 2 ticks (~60s) — outlasts the watcher's poll cycle"
        );
    }

    // -------------------------------------------------------------------
    // 2026-06-26 — probe budget widened past the watcher's own read budget
    // so a slow-but-reachable overnight legacy stops producing
    // self-recovering false-positive info pages.
    // -------------------------------------------------------------------

    /// The watchdog probe budget MUST exceed the main watcher's 10s
    /// `MssqlOpKind::Read` budget — otherwise the watcher rides out a slow
    /// legacy while THIS probe times out and pages. Also must stay well
    /// under the 60s poll interval. Raised 12s → 30s on 2026-06-29 after
    /// info pages kept firing at 12s during deep overnight-quiet windows.
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn watchdog_probe_budget_exceeds_read_budget_and_fits_tick() {
        assert_eq!(DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS, 30_000);
        assert!(
            DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS > 10_000,
            "probe budget must exceed the watcher's 10s read budget"
        );
        assert!(
            DEFAULT_WATCHDOG_CT_PROBE_TIMEOUT_MS
                < WATERMARK_WATCHDOG_POLL_INTERVAL_SECS * 1000,
            "probe budget must stay under the 60s poll interval"
        );
    }

    /// The escalation default is 20 min — several watchdog ticks above a
    /// self-recovering flap, so a genuine probe-unreachable outage pages
    /// promptly while transient blips stay silent.
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn probe_outage_escalation_default_is_twenty_minutes() {
        // 2026-06-30 — lowered 1h → 20min. This escalation is the SOLE
        // Slack signal for a sustained probe-unreachable legacy (the
        // informational note is suppressed), so it must surface a real
        // outage promptly without paging on a self-recovering flap.
        assert_eq!(DEFAULT_PROBE_OUTAGE_ESCALATION_SECS, 1200);
        assert!(
            DEFAULT_PROBE_OUTAGE_ESCALATION_SECS
                >= 4 * WATERMARK_WATCHDOG_POLL_INTERVAL_SECS,
            "escalation must sit several watchdog ticks above a transient flap, \
             so a 1–3 tick self-recovering blip never pages"
        );
    }

    /// No open outage → never escalate (the benign path: probe succeeds,
    /// or no stall at all).
    #[test]
    fn probe_outage_escalation_none_when_no_open_outage() {
        let now = Instant::now();
        let thr = Duration::from_secs(3600);
        assert!(!probe_outage_escalation_eligible(None, now, thr, false));
    }

    /// Open but not yet past the threshold → hold (this is the normal
    /// self-recovering flap window — info page, recover in 2-3 min).
    #[test]
    fn probe_outage_escalation_holds_inside_threshold() {
        let since = Instant::now();
        let now = since + Duration::from_secs(600); // 10 min < 1h
        let thr = Duration::from_secs(3600);
        assert!(!probe_outage_escalation_eligible(Some(since), now, thr, false));
    }

    /// Open past the threshold and not yet escalated → escalate.
    #[test]
    fn probe_outage_escalation_fires_past_threshold() {
        let since = Instant::now();
        let now = since + Duration::from_secs(3600);
        let thr = Duration::from_secs(3600);
        assert!(probe_outage_escalation_eligible(Some(since), now, thr, false));
    }

    /// Already escalated → never escalate again (one-time per outage; the
    /// caller flips the flag so we don't spam `:rotating_light:`).
    #[test]
    fn probe_outage_escalation_is_one_time() {
        let since = Instant::now();
        let now = since + Duration::from_secs(7200); // way past threshold
        let thr = Duration::from_secs(3600);
        assert!(
            !probe_outage_escalation_eligible(Some(since), now, thr, true),
            "an already-escalated outage must not re-escalate"
        );
    }

    /// The escalation message must read as a real alert, not the benign
    /// "no action needed" info note, and must carry the version + duration.
    #[test]
    fn probe_outage_escalation_message_is_actionable() {
        let msg = format_probe_outage_escalation_message(
            44660,
            Duration::from_secs(3600),
            Duration::from_secs(3600),
        );
        assert!(msg.contains("v44660"), "must name the stuck version; got: {msg}");
        assert!(msg.contains(":rotating_light:"), "must be a critical page; got: {msg}");
        assert!(msg.contains("60min"), "must state the outage duration; got: {msg}");
        assert!(
            !msg.contains("no action needed"),
            "escalation must NOT reuse the benign info wording; got: {msg}"
        );
    }

    // -------------------------------------------------------------------
    // 2026-06-11 — severity-aware cooldown (paired with the probe-timeout
    // demotion to `:information_source:`).
    // -------------------------------------------------------------------

    /// No prior page → any page passes.
    #[test]
    fn stall_cooldown_passes_with_no_prior_page() {
        let now = Instant::now();
        let cd = Duration::from_secs(1800);
        assert!(stall_page_passes_cooldown(None, now, cd, false));
        assert!(stall_page_passes_cooldown(None, now, cd, true));
    }

    /// Inside the cooldown window, a same-or-lower-severity page is
    /// suppressed: info after info, info after critical, critical
    /// after critical.
    #[test]
    fn stall_cooldown_suppresses_non_escalating_pages_inside_window() {
        let paged_at = Instant::now();
        let now = paged_at + Duration::from_secs(60);
        let cd = Duration::from_secs(1800);
        assert!(!stall_page_passes_cooldown(Some((paged_at, false)), now, cd, false));
        assert!(!stall_page_passes_cooldown(Some((paged_at, true)), now, cd, false));
        assert!(!stall_page_passes_cooldown(Some((paged_at, true)), now, cd, true));
    }

    /// THE point of the helper: a confirmed-backlog critical page must
    /// bypass the cooldown left by a probe-timeout informational note.
    /// Otherwise the info demotion could shadow a real stall for up to
    /// 30 minutes.
    #[test]
    fn stall_cooldown_lets_critical_escalate_past_info_page() {
        let paged_at = Instant::now();
        let now = paged_at + Duration::from_secs(60);
        let cd = Duration::from_secs(1800);
        assert!(
            stall_page_passes_cooldown(Some((paged_at, false)), now, cd, true),
            "critical page must bypass the cooldown set by an informational page"
        );
    }

    /// After the cooldown elapses any severity re-pages.
    #[test]
    fn stall_cooldown_passes_after_window_elapses() {
        let paged_at = Instant::now();
        let now = paged_at + Duration::from_secs(1800);
        let cd = Duration::from_secs(1800);
        assert!(stall_page_passes_cooldown(Some((paged_at, true)), now, cd, false));
        assert!(stall_page_passes_cooldown(Some((paged_at, false)), now, cd, false));
    }

    /// Recovery wording must match the actual reason — the
    /// ProbeConfirmsQuiet case previously claimed the watermark
    /// "resumed advancing" at the very version it was paged on
    /// (observed 2026-06-11: "resumed advancing (now at v35775)"
    /// after paging on v35775).
    #[test]
    fn recovery_message_wording_matches_reason() {
        let now = Instant::now();
        let paged_at = now - Duration::from_secs(120);

        let advanced = format_recovery_message(
            &RecoveryDecision {
                paged_version: 35775,
                paged_at,
                current_version: 35800,
                reason: RecoveryReason::WatermarkAdvanced,
            },
            now,
        );
        assert!(
            advanced.contains("resumed advancing") && advanced.contains("v35800"),
            "advanced case must say so; got: {advanced}"
        );

        let quiet = format_recovery_message(
            &RecoveryDecision {
                paged_version: 35775,
                paged_at,
                current_version: 35775,
                reason: RecoveryReason::ProbeConfirmsQuiet,
            },
            now,
        );
        assert!(
            quiet.contains("confirmed idle") && quiet.contains("v35775"),
            "quiet case must say legacy was idle, not 'resumed advancing'; got: {quiet}"
        );
        assert!(
            !quiet.contains("resumed advancing"),
            "quiet case must NOT claim the watermark advanced; got: {quiet}"
        );
    }

    /// Track D / T7 CRIT-3 — the canonical case the watchdog exists for:
    /// version stuck longer than the threshold in live mode fires.
    #[test]
    fn watermark_stall_watchdog_alerts_when_version_stuck() {
        let anchor = Instant::now();
        let prior = obs(100, 0, anchor);
        let current = obs(100, 1801, anchor); // same version, 30m+1s later
        let now = anchor + Duration::from_secs(1801);
        let result = watermark_stall_alert_eligible(
            &prior,
            &current,
            now,
            false,
            Duration::from_secs(1800),
        );
        assert!(result.is_some(), "stuck >threshold must alert in live mode");
        let msg = result.unwrap();
        assert!(msg.contains("stuck at 100"));
        assert!(msg.contains("1801"));
    }

    /// Track D / T7 CRIT-3 — same input under shadow mode does NOT fire
    /// (shadow stall is handled by `shadow_mode_pager_eligible`).
    #[test]
    fn watermark_stall_watchdog_suppressed_in_shadow_mode() {
        let anchor = Instant::now();
        let prior = obs(100, 0, anchor);
        let current = obs(100, 7200, anchor);
        let now = anchor + Duration::from_secs(7200);
        let result = watermark_stall_alert_eligible(
            &prior,
            &current,
            now,
            true, // shadow mode
            Duration::from_secs(1800),
        );
        assert!(result.is_none(), "shadow stall must be handled by the other rule");
    }

    /// Track D / T7 CRIT-3 — stuck for less than the threshold must not
    /// alert. Steady-state low-traffic periods are normal.
    #[test]
    fn watermark_stall_watchdog_no_alert_below_threshold() {
        let anchor = Instant::now();
        let prior = obs(100, 0, anchor);
        let current = obs(100, 300, anchor); // 5 min stuck
        let now = anchor + Duration::from_secs(300);
        let result = watermark_stall_alert_eligible(
            &prior,
            &current,
            now,
            false,
            Duration::from_secs(1800), // 30 min threshold
        );
        assert!(result.is_none(), "below threshold must not alert");
    }

    /// Track D / T7 CRIT-3 — shadow mode older than the ceiling fires.
    #[test]
    fn shadow_mode_pager_fires_past_ceiling() {
        let started_at = Instant::now();
        let now = started_at + Duration::from_secs(SHADOW_MODE_MAX_DURATION_SECS + 1);
        let result = shadow_mode_pager_eligible(true, started_at, now);
        assert!(result.is_some(), "shadow > ceiling must page");
    }

    /// Track D / T7 CRIT-3 — shadow mode younger than the ceiling does
    /// not fire (12h is below the 36h threshold).
    #[test]
    fn shadow_mode_pager_silent_inside_ceiling() {
        let started_at = Instant::now();
        let now = started_at + Duration::from_secs(12 * 3600);
        let result = shadow_mode_pager_eligible(true, started_at, now);
        assert!(result.is_none(), "12h shadow run must NOT page");
    }

    /// Track D / T7 CRIT-3 — live mode never triggers the
    /// shadow-too-long alert regardless of elapsed time.
    #[test]
    fn shadow_mode_pager_silent_in_live_mode() {
        let started_at = Instant::now();
        let now = started_at + Duration::from_secs(SHADOW_MODE_MAX_DURATION_SECS + 10_000);
        let result = shadow_mode_pager_eligible(false, started_at, now);
        assert!(result.is_none(), "live mode must never fire the shadow pager");
    }

    /// Track D / T7 CRIT-3 — the ceiling sits below the 48h MSSQL CT
    /// retention cliff with at least 12h of cushion. Locks the constant
    /// so a future refactor can't push it past the cliff without
    /// failing this test.
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn shadow_mode_ceiling_below_ct_retention_cliff() {
        const CT_RETENTION_CLIFF_SECS: u64 = 48 * 3600;
        assert!(
            SHADOW_MODE_MAX_DURATION_SECS < CT_RETENTION_CLIFF_SECS,
            "shadow ceiling must be < 48h MSSQL CT retention"
        );
        assert!(
            CT_RETENTION_CLIFF_SECS - SHADOW_MODE_MAX_DURATION_SECS >= 12 * 3600,
            "must leave >=12h cushion before the cliff"
        );
    }

    // -------------------------------------------------------------------
    // PR D — tone-aware stall alerts + recovery notification
    // (2026-05-19 false-positive incident: 3 self-recovering pages at
    // 10:16, 12:01, 13:27 Thailand time, all probe-timeout cases).
    // -------------------------------------------------------------------

    /// Probe timeout / failure should yield the informational-tier
    /// message (demoted from `:warning:` on 2026-06-11 — the quiet-
    /// period + slow-probe combination is expected behavior), NOT the
    /// critical-tier "real changes are not being processed" wording.
    #[test]
    fn format_stall_alert_message_uses_info_tier_when_probe_is_none() {
        let msg = format_stall_alert_message(
            17209,
            None,
            Duration::from_secs(1801),
            Duration::from_secs(1800),
        );
        assert!(
            msg.contains(":information_source:"),
            "probe-failure path must use the informational tier; got: {msg}"
        );
        assert!(
            !msg.contains(":warning:") && !msg.contains(":rotating_light:"),
            "probe-failure path must NOT use :warning: or :rotating_light:; got: {msg}"
        );
        assert!(
            msg.contains("cannot confirm a real backlog"),
            "warning text must clarify there's no confirmed backlog; got: {msg}"
        );
        assert!(
            msg.contains("v17209"),
            "warning text must include the watermark version; got: {msg}"
        );
    }

    /// Probe ahead of the watermark is the canonical "wedged" case.
    /// Must use the critical tier AND surface the delta so operators
    /// can size the backlog at a glance.
    #[test]
    fn format_stall_alert_message_uses_critical_with_delta_when_probe_ahead() {
        let msg = format_stall_alert_message(
            17209,
            Some(17250),
            Duration::from_secs(1800),
            Duration::from_secs(1800),
        );
        assert!(
            msg.contains(":rotating_light:"),
            "probe-ahead path must use :rotating_light:; got: {msg}"
        );
        assert!(
            msg.contains("v17209"),
            "must include the watermark version; got: {msg}"
        );
        assert!(
            msg.contains("v17250"),
            "must include the legacy current version; got: {msg}"
        );
        assert!(
            msg.contains("41 versions unprocessed"),
            "must surface the {{c - w}} delta (17250 - 17209 = 41); got: {msg}"
        );
        assert!(
            msg.contains("Real changes are not being processed"),
            "critical wording must reflect confirmed backlog; got: {msg}"
        );
    }

    /// Probe below the watermark is an anomaly — monotonicity is
    /// guaranteed by SQL Server CT. Must still alert critically.
    #[test]
    fn format_stall_alert_message_uses_critical_for_monotonicity_violation() {
        let msg = format_stall_alert_message(
            17209,
            Some(100),
            Duration::from_secs(1800),
            Duration::from_secs(1800),
        );
        assert!(
            msg.contains(":rotating_light:"),
            "monotonicity-violation path must use :rotating_light:; got: {msg}"
        );
        assert!(
            msg.contains("anomaly"),
            "wording must call out the anomaly; got: {msg}"
        );
        assert!(
            msg.contains("Monotonicity"),
            "wording must mention monotonicity for operator clarity; got: {msg}"
        );
        assert!(
            msg.contains("v17209") && msg.contains("v100"),
            "must include both versions; got: {msg}"
        );
    }

    /// Idle-loop case: no open alert, no recovery decision.
    #[test]
    fn recovery_alert_eligible_returns_none_when_no_pending_alert() {
        let decision = recovery_alert_eligible(None, 17209, Some(17209));
        assert!(
            decision.is_none(),
            "no open alert means no recovery to declare"
        );
    }

    /// Watermark advanced past the paged version → fire recovery,
    /// regardless of probe outcome (the cheap branch).
    #[test]
    fn recovery_alert_eligible_fires_when_watermark_advanced_past_paged_version() {
        let paged_at = Instant::now();
        let decision = recovery_alert_eligible(
            Some((paged_at, 17209)),
            17220, // advanced 11 versions
            None,  // probe not even attempted on the cheap branch
        )
        .expect("watermark advance must trigger recovery");
        assert_eq!(decision.paged_version, 17209);
        assert_eq!(decision.current_version, 17220);
        assert_eq!(decision.reason, RecoveryReason::WatermarkAdvanced);
    }

    /// Watermark didn't advance, but the recovery-check probe now
    /// confirms legacy CT is idle at our watermark — fire recovery.
    /// This is the 2026-05-19 case (iHOTEL idle, probe timed out
    /// during stall window but succeeded a few minutes later).
    #[test]
    fn recovery_alert_eligible_fires_when_probe_confirms_quiet_at_watermark() {
        let paged_at = Instant::now();
        let decision = recovery_alert_eligible(
            Some((paged_at, 17209)),
            17209,        // watermark unchanged
            Some(17209),  // legacy idle at our version
        )
        .expect("probe == watermark must trigger recovery");
        assert_eq!(decision.paged_version, 17209);
        assert_eq!(decision.current_version, 17209);
        assert_eq!(decision.reason, RecoveryReason::ProbeConfirmsQuiet);
    }

    /// Defensive: probe failure during recovery check must NOT
    /// declare recovery. We'd rather hold the open alert than fire a
    /// premature all-clear on uncertainty.
    #[test]
    fn recovery_alert_eligible_returns_none_when_probe_failed_during_recovery_check() {
        let paged_at = Instant::now();
        let decision = recovery_alert_eligible(
            Some((paged_at, 17209)),
            17209,
            None, // probe failed
        );
        assert!(
            decision.is_none(),
            "probe failure during recovery check must hold the open alert"
        );
    }

    /// Idle stall persists: watermark unchanged AND probe still
    /// shows legacy is ahead (or unreachable in a different way).
    /// Recovery must not fire.
    #[test]
    fn recovery_alert_eligible_returns_none_when_state_unchanged_and_probe_still_silent() {
        let paged_at = Instant::now();
        // Probe says legacy is still ahead — stall is real and
        // continuing. Recovery should NOT declare all-clear.
        let decision = recovery_alert_eligible(
            Some((paged_at, 17209)),
            17209,
            Some(17250), // legacy still ahead
        );
        assert!(
            decision.is_none(),
            "ongoing stall (probe still ahead) must not fire recovery"
        );
    }
}
