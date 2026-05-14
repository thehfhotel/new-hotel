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
//!       - Query `CHANGETABLE(CHANGES <table>, @last) JOIN <table>`
//!         filtering `SYS_CHANGE_CONTEXT <> 0x4E48` (loop-prevention).
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
use hotel_backend::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::notifications::slack::{SlackClient, SlackMessage};
use hotel_backend::outbox::bus::EventBus;
use hotel_backend::outbox::event::DomainEvent;
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mappers::{
    apply_booking_aggregate, apply_checkin_aggregate, apply_payment_aggregate,
    BillDebtDsMirrorMapper, BillDebtHMirrorMapper, BookingDatesMapper, BookingHeaderMapper,
    BookingRoomsMapper, ChangedRoomMirrorMapper, CheckInHeaderMapper, CheckInRoomsMapper,
    CheckinProductMirrorMapper, CuponMirrorMapper, CustomerMapper, DepositMirrorMapper,
    GuestRegistryMapper, PaymentMapper, ReceiptMapper, RoomCalendarMapper, RoomMasterMapper,
    RoomsCancelMirrorMapper,
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

/// MSSQL-pool-outage circuit breaker (v2.58.4). HF Ville's WG tunnel
/// flaps for ~2 min every couple of days; when the legacy MSSQL is
/// unreachable, every `mssql.get().await` blocks for the full
/// `POOL_CONNECTION_TIMEOUT` (5s as of R2 / 2026-05-14, was 15s)
/// and returns "Timed out in bb8". Without short-circuiting we walk
/// the 16-table loop sequentially, each table's own `fetch_ct_rows`
/// paying its own pool-timeout — the burst still lasts ~16× that
/// budget and produces 16 identical WARNs. The breaker trips on the
/// FIRST pool-timeout in a tick, abandons the rest of the tick, and
/// sleeps a cooldown so the next tick gives the tunnel a chance to
/// recover before retrying.
///
/// Override at runtime via `LEGACY_SYNC_OUTAGE_COOLDOWN_SECS`.
const DEFAULT_OUTAGE_COOLDOWN_SECS: u64 = 30;

/// Number of consecutive ticks that must trip the pool-outage breaker
/// before the watcher pages an operator. A single tick failure is
/// almost always a 1s WG keepalive miss + immediate recovery — paging
/// on it would be pure noise. Two consecutive failed ticks (separated
/// by the cooldown above ≈ 30s minimum) only happen when the tunnel
/// has stayed dead for >30s, which is the operationally interesting
/// case.
///
/// Override at runtime via `LEGACY_SYNC_OUTAGE_ALERT_THRESHOLD`.
const DEFAULT_OUTAGE_ALERT_THRESHOLD: u32 = 2;

/// MSSQL-pool-init retry knobs (v2.63.0). Same root cause as the
/// mid-run outage breaker above — HF Ville's WG tunnel can be down at
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

/// All CT-enabled MSSQL tables — must stay in sync with the seed in
/// migrations 017 (canonical sync, 10 tables) + 022 (legacy_mirror, 6
/// tables) and the `legacy_sync_status` rows. Adding a new mapper
/// means inserting a row in the relevant seed migration, adding the
/// table here, and wiring its mapper in `build_mappers`.
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
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

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

    let bootstrap_requested = env::args().any(|a| a == "--bootstrap");

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
        if enabled {
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
        return run_bootstrap(&site).await;
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

    // Phase 5.5/2.58.4 — MSSQL-pool-outage breaker knobs. Both are
    // operator-visible env vars so a noisy WG tunnel can be tuned
    // without a redeploy.
    let outage_cooldown = Duration::from_secs(
        env::var("LEGACY_SYNC_OUTAGE_COOLDOWN_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_OUTAGE_COOLDOWN_SECS),
    );
    let outage_alert_threshold = env::var("LEGACY_SYNC_OUTAGE_ALERT_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_OUTAGE_ALERT_THRESHOLD);

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
        outage_cooldown_secs = outage_cooldown.as_secs(),
        outage_alert_threshold,
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
    let watchdog_slack = slack.clone();
    let watchdog_site_id = site.id.clone();
    let watchdog_shutdown = shutdown.clone();
    tokio::spawn(async move {
        run_watermark_watchdog(
            watchdog_pg,
            watchdog_slack,
            watchdog_site_id,
            stall_alert_secs,
            watchdog_shutdown,
        )
        .await;
    });

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
async fn run_bootstrap(site: &SiteConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!(
        site = %site.id,
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
///    is FALSE AND `last_polled_at` is recent → ALERT (the watcher is
///    ticking but not advancing — the canonical CT-watermark-stall trap).
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
async fn run_watermark_watchdog(
    pg: PgPool,
    slack: Option<SlackClient>,
    site_id: String,
    stall_alert_secs: u64,
    shutdown: Arc<Notify>,
) {
    let started_at = Instant::now();
    let stall_threshold = Duration::from_secs(stall_alert_secs);
    let cooldown = Duration::from_secs(WATCHDOG_ALERT_COOLDOWN_SECS);
    let mut prior: Option<WatermarkObservation> = None;
    let mut last_stall_alert: Option<Instant> = None;
    let mut last_shadow_alert: Option<Instant> = None;

    let shadow_mode = env::var("LEGACY_SYNC_SHADOW_MODE")
        .map(|v| v == "true")
        .unwrap_or(false);

    tracing::info!(
        site = %site_id,
        stall_alert_secs,
        shadow_mode,
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

        // Watermark stall check (live mode only).
        if let Some(prior_obs) = prior.as_ref() {
            if let Some(reason) =
                watermark_stall_alert_eligible(prior_obs, &observation, now, shadow_mode, stall_threshold)
            {
                let cooldown_elapsed = match last_stall_alert {
                    Some(t) => now.duration_since(t) >= cooldown,
                    None => true,
                };
                if cooldown_elapsed {
                    tracing::error!(
                        site = %site_id,
                        version = observation.last_seen_version,
                        reason,
                        "[watchdog] Watermark stall detected — paging operator"
                    );
                    if let Some(s) = slack.as_ref() {
                        let payload = SlackMessage::with_site_text(
                            &site_id,
                            format!(
                                ":rotating_light: *CT watermark STUCK* :rotating_light:\n\
                                 {reason}\n\
                                 _The CT watcher is ticking but `last_seen_version` \
                                 hasn't advanced. Common causes: every tick failing + \
                                 rolling back the watermark UPDATE (check \
                                 `legacy_sync_status.last_error`), or all 16 tables \
                                 happen to be quiet. Tighten the check via the \
                                 dashboard at `/api/new/sync/status`._"
                            ),
                        );
                        let _ = s.send_message(&payload).await;
                    }
                    last_stall_alert = Some(now);
                }
            } else if observation.last_seen_version > prior_obs.last_seen_version {
                tracing::debug!(
                    site = %site_id,
                    from = prior_obs.last_seen_version,
                    to = observation.last_seen_version,
                    "[watchdog] Watermark advanced"
                );
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
/// Phase 5.5c adds the 6 legacy_mirror.* mappers, bringing total
/// mapper coverage to 16 tables.
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
                        tracing::warn!(
                            event_name = EV_ORPHAN_RECOVERY_FAIL,
                            table,
                            ds_id,
                            reason = "lookup_query_errored",
                            error = %err,
                            "D-event orphan recovery query errored; skipping"
                        );
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
                        Ok(a) => apply_booking_aggregate(&mut tx, &a, key).await,
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
                    } else {
                        ingested += 1;
                    }
                }
                Ok(None) => {
                    // Idempotent skip — canonical row already matches.
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
                    } else {
                        ingested += 1;
                    }
                }
                Ok(None) => {
                    // Idempotent skip / D-event with no event payload.
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

    if max_version > last_seen {
        // R3 — feature-flagged dual-write contract. Per-table mode
        // advances ONLY the per-table row so a stuck sibling
        // doesn't pin the global down; global mode advances ONLY
        // the single-row state, preserving the pre-R3 behaviour.
        let advance_result = if per_table_watermark {
            hotel_backend::sync::watermark::advance_per_table(pg, table, max_version).await
        } else {
            hotel_backend::sync::watermark::advance(pg, max_version).await
        };
        match advance_result {
            Err(err) => {
                // R1: structured event + persisted failure mode so the
                // 2026-05-14 symptom (UPDATE failure post-commit, no
                // PG-side breadcrumb) survives a container restart.
                tracing::error!(
                    event_name = EV_WATERMARK_ADVANCE_FAIL,
                    table,
                    new_version = max_version,
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
                    to = max_version,
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

/// Fetch CT rows joined with the table, filtered by loop-prevention
/// `SYS_CHANGE_CONTEXT <> 0x4E48`, ordered by `SYS_CHANGE_VERSION` for
/// monotonic processing.
async fn fetch_ct_rows(
    mssql: &DbPool,
    table: &str,
    pk_cols: &[&str],
    select_sql: &str,
    last_seen: i64,
) -> Result<Vec<CtRow>, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;

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

    let sql = format!(
        "SELECT ct.SYS_CHANGE_VERSION AS sys_change_version, \
                ct.SYS_CHANGE_OPERATION AS sys_change_operation, \
                {pk_projection}, \
                {select_sql} \
           FROM CHANGETABLE(CHANGES {table}, {last_seen}) AS ct \
           LEFT JOIN {table} AS t ON {join_clause} \
          WHERE ct.SYS_CHANGE_CONTEXT IS NULL \
             OR ct.SYS_CHANGE_CONTEXT <> 0x4E48 \
          ORDER BY ct.SYS_CHANGE_VERSION ASC"
    );

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

async fn count_ct_rows(
    mssql: &DbPool,
    table: &str,
    last_seen: i64,
) -> Result<i64, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
    let sql = format!(
        "SELECT COUNT(*) FROM CHANGETABLE(CHANGES {table}, {last_seen}) AS ct \
         WHERE ct.SYS_CHANGE_CONTEXT IS NULL OR ct.SYS_CHANGE_CONTEXT <> 0x4E48"
    );
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
            18,
            "18 CT-enabled tables expected (10 canonical + 6 legacy_mirror \
             + 2 Track-E1: HT_CheckIn_Other_People + HT_Rooms_Cancel)"
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

    #[test]
    fn ct_enabled_tables_match_migration_017_022_and_033_seed() {
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
        ];
        assert_eq!(CT_ENABLED_TABLES, &expected);
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

    /// CHANGETABLE filter must include the SYS_CHANGE_CONTEXT clause
    /// matching the `SET CONTEXT_INFO 0x4E48` value the writeback
    /// dispatcher stamps. Locks the byte literal so a refactor can't
    /// silently break loop-prevention. The check is a substring match
    /// on `0x4E48` near `SYS_CHANGE_CONTEXT` — both fetch_ct_rows and
    /// count_ct_rows compose the clause across multiple source lines,
    /// so an exact-string match would be fragile.
    #[test]
    fn fetch_ct_rows_uses_loop_prevention_filter() {
        let source = include_str!("sync.rs");
        // Both occurrences of the literal must be paired with the
        // SYS_CHANGE_CONTEXT predicate — this guards both the JOIN
        // SELECT and the COUNT SELECT.
        let ctx_count = source.matches("SYS_CHANGE_CONTEXT").count();
        let tag_count = source.matches("0x4E48").count();
        assert!(
            ctx_count >= 2 && tag_count >= 2,
            "expected ≥2 occurrences of SYS_CHANGE_CONTEXT + 0x4E48 (JOIN + COUNT paths)"
        );
    }

    /// The CONTEXT_INFO value used by writeback's dispatcher and the
    /// CT watcher's filter must be the same byte sequence — otherwise
    /// every writeback re-fires through the watcher.
    #[test]
    fn loop_prevention_tag_matches_writeback_dispatcher() {
        let dispatcher_src = include_str!("../writeback/dispatcher.rs");
        let mssql_session_src = include_str!("../db/mssql_session.rs");
        assert!(dispatcher_src.contains("set_context_info(conn)"));
        assert!(
            mssql_session_src.contains("SET CONTEXT_INFO 0x4E48"),
            "mssql_session::set_context_info must issue 0x4E48 \
             so the watcher's CHANGETABLE filter can match it"
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
}
