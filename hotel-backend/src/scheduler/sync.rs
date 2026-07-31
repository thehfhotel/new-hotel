//! Background sync job: drift tripwire comparing legacy SQL Server state
//! against canonical PostgreSQL state.
//!
//! ## Two modes (Phase 5.5+)
//!
//! Controlled by env var `LEGACY_SYNC_RECONCILE_MODE`:
//!
//! | Mode        | Behaviour                                                 | Default? |
//! |-------------|-----------------------------------------------------------|----------|
//! | `diff_only` | Hash legacy + canonical rows, log divergence to `ht_reconcile_log`. | ✅ yes   |
//! | `upsert`    | Pre-5.5 escape hatch: UPSERT into `ht_*_legacy` mirror.   | dead code path |
//!
//! Phase 5.5 cutover: the CT watcher (`bin/sync.rs`) is now authoritative
//! for canonical PG state, so this job is demoted from a 5-min full-sync
//! UPSERT to a 15-min drift-detection tripwire. If the watcher misses a
//! row (CT retention overflow, transient mapper bug, schema regression),
//! the next reconcile tick lands a row in `ht_reconcile_log` for an
//! operator to investigate. `upsert` mode is preserved purely for forensic
//! rollback flexibility — it is **not exercised by any deployed code path**
//! post v2.63.0.
//!
//! Per docs/architecture.md §3.6d, §8 (Phase 5.5 row).
//!
//! ## What this job compares (v2.63.0+)
//!
//! 1. Customers: `View_Customers`  vs canonical `ht_customers`   (JOIN `legacy_cust_no`)
//! 2. Rooms:     `HT_Rooms`        vs canonical `ht_rooms_new`   (JOIN `legacy_room_no` / `room_no`)
//! 3. Bookings:  `View_Booking_Ds` vs canonical `ht_bookings`    (JOIN `legacy_book_id`)
//! 4. Check-ins: `View_CheckIn_Ds` vs canonical `ht_checkins`    (JOIN `legacy_cin_no`)
//! 5. Payments: `HT_Receipt_H`    vs canonical `ht_payments`    (JOIN `legacy_receipt_no`
//!    / `pay_reference`) — Phase 6-A, keyed on `Receipt_no`, and **DARK by
//!    default**: it runs only when `RECONCILE_PAYMENTS_ARM_ENABLED=true`.
//! 6. Guest registry: `HT_CheckIn_Other_People` vs canonical
//!    `ht_guest_registry` (JOIN `ht_checkins.legacy_cin_no`) — Phase 6-B,
//!    keyed on `Cin_no` and reconciled per FOLIO (the whole companion set of
//!    one check-in), not per row: legacy edits are DELETE+reinsert with id
//!    churn. Also **DARK by default**:
//!    `RECONCILE_GUEST_REGISTRY_ARM_ENABLED=true`.
//! 7. Mirror tables: the 8 CT-mirrored `legacy_mirror.*` tables plus
//!    `ht_room_calendar`, compared by aggregate (COUNT / MAX(pk) / SUM of the
//!    money total) with a per-PK diff only on mismatch — Phase 6-C, see
//!    [`crate::scheduler::mirror_probe`]. Also **DARK by default**:
//!    `RECONCILE_MIRROR_PROBE_ENABLED=true`.
//! 8. Payment ledger: `HT_CheckIn_Pay` vs canonical `ht_payment_ledger`,
//!    compared per FOLIO (`Cin_No`) on line count + itemized amount +
//!    receipt-deduped active tender — Phase 6-D, see
//!    [`crate::scheduler::payment_ledger_probe`]. Also **DARK by default**:
//!    `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED=true`.
//!
//! Pre-v2.63.0 this job compared MSSQL hashes against `ht_*_legacy.sync_hash`
//! (the demoted mirror tables). After the 2026-04-28 cutover those mirrors
//! stopped getting their data columns refreshed — only `sync_hash` /
//! `synced_at` tick — so MSSQL-vs-mirror drift became cosmetic noise
//! (~2300+ unresolved entries observed). v2.63.0 switches the PG side
//! to canonical-table hashes so drift IS actionable (= real CT-mapper gap).
//!
//! The `ht_*_legacy` tables are retained as a **per-PK ack cache**: once a
//! divergence is logged, the current `mssql_hash` is written to
//! `ht_*_legacy.sync_hash` so the next tick short-circuits the same drift
//! instead of re-firing it. The mirror's data columns are intentionally
//! left stale — the CT watcher owns canonical state.

use chrono::{NaiveDate, NaiveDateTime};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::time::Instant;

use tiberius::Query;

use crate::db::mssql_timeout::{
    query_with_timeout_pooled, simple_query_with_timeout_pooled, MssqlOpKind,
};
use crate::db::{DbPool, PgPool};
// Issue #204 (bug #2): the durable self-healing arm of the auto-resolve
// sweep re-drives the EXISTING CT upsert path, so it reaches for the same
// mappers / row-abstraction / op-enum the watcher uses rather than writing
// canonical fields by hand.
use crate::notifications::slack::{SlackClient, SlackMessage};
use crate::outbox::event::DomainEvent;
// Phase 6-D: one literal for the probe's `ht_reconcile_log.table_name`, its
// `sync_status.entity_type` and its `RECONCILE_RESOLVABLE_TABLES` entry, so
// the three can never drift apart.
use crate::scheduler::payment_ledger_probe::PAYMENT_LEDGER_PROBE_KEY;
use crate::sync::change_op::ChangeOp;
// Single-sourced `|` separator, shared with the mapper-side descriptor
// tables that pin the gate ⊇ reconcile-hash invariant.
use crate::sync::gate_guard::join_hash_segments;
use crate::sync::mapper::MssqlChangeMapper;
// Phase 6-B: the companion-folio projection is shared with the CT mapper so
// the reconcile arm and the mapper cannot disagree about what a folio is,
// and the canonical name re-concatenation is the SAME bytes the mapper's
// echo-adoption match uses.
use crate::sync::mappers::guest_registry::{
    RegistryFolioProjection, CANONICAL_COMPANION_NAME_SQL,
};
use crate::sync::mappers::{CustomerMapper, RoomMasterMapper};
use crate::sync::row::MappableRow;

// =============================================================================
// Scheduler-side structured-event registry (issue #267)
// =============================================================================
//
// `bin/sync.rs` owns `KNOWN_SYNC_EVENT_NAMES`, the registry for the CT
// watcher's `sync.*` failure taxonomy. That registry is BINARY-local and
// unreachable from here — a library module cannot depend on a `bin` target —
// so the scheduler keeps its own list rather than one merged cross-binary
// registry. Issue #267 asked for that call to be made explicitly: it stays
// SPLIT, and not only because the merge is mechanically impossible. The two
// populations have different contracts:
//
//   * watcher names are dot-namespaced (`sync.…`) and are additionally
//     PERSISTED, prefixed, into `legacy_sync_status.last_error`, so an
//     operator triaging a stalled table still sees the failure MODE after the
//     log line has aged out;
//   * scheduler names are log-only tripwires consumed by `/diagnose-alert`
//     greps and dashboards. They are deliberately UNPREFIXED — renaming the
//     live `force_converge_gate_skip` to fit someone else's namespace would
//     break the exact grep contract this registry exists to protect.
//
// The two namespaces stay disjoint (no dots here — pinned below), so a Loki
// filter of `^sync\.` still means "the CT watcher" and nothing else.
//
// Adding a new scheduler event: declare an `EV_…` const HERE, hand that const
// (never a bare string literal) to the `tracing::…!` call site, and add it to
// `KNOWN_SCHEDULER_EVENT_NAMES`. All three steps are mechanically enforced by
// the registry lock tests at the bottom of this file — a raw literal at a
// call site, or a const missing from the registry, fails the test gate.

/// The auto-resolve sweep re-drove a row through the CT mapper, the mapper
/// reported "nothing to do" AND canonical did not move, yet the row is still
/// unconverged — i.e. the mapper's idempotency gate covers FEWER fields than
/// the reconcile hash (a gate ⊄ hash violation). Such a row can never
/// self-heal, and the watcher will never see a CT event for it either.
/// Emitted by BOTH self-heal arms; the `arm` field discriminates
/// (`value_drift` / `missing_pg`).
pub(crate) const EV_FORCE_CONVERGE_GATE_SKIP: &str = "force_converge_gate_skip";

/// Registry of every structured event this module emits. Membership-tested,
/// not pattern-matched — order is not significant.
///
/// `#[allow(dead_code)]`: the array itself is referenced only by the registry
/// lock tests (the `EV_*` constants it holds are used at their call sites).
/// Same shape as `bin/sync.rs`'s `KNOWN_SYNC_EVENT_NAMES`.
#[allow(dead_code)]
const KNOWN_SCHEDULER_EVENT_NAMES: &[&str] = &[EV_FORCE_CONVERGE_GATE_SKIP];

/// Default per-table drift-count threshold above which a Slack alert is
/// fired on the next reconcile tick. 50 unresolved rows for a single
/// `table_name` in the last hour is well above the steady-state noise
/// floor (which should be 0) and below the noise level a genuine bulk
/// catch-up scenario would produce. Override at deploy time with
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`.
///
/// **This is a blast-radius dial, not a target** (2026-07-28 alert
/// inventory). 21 days of production data peak at 33 unresolved rows/hr
/// for a single table, so the burst alert has never fired — that is the
/// designed state. Do NOT "tune it down until it fires": the level
/// digest below already covers the single-row / slow-burn case, and this
/// threshold exists purely to catch a bulk regression (mapper crash,
/// schema break, retention overflow) before it floods the log. Lowering
/// it converts a silent-by-design tripwire into a recurring digest.
pub const DEFAULT_DRIFT_ALERT_THRESHOLD: i64 = 50;

/// Default cooldown for the edge-triggered burst alert
/// ([`check_drift_and_alert`]), per `(site, table)`.
///
/// 2026-07-28 alert inventory, defect C5: the burst alert had NO cooldown
/// at all. Above threshold it re-fired on every 15-min reconcile tick,
/// and HF Ville runs a second independent emitter (the worker reconcile
/// behind `WORKER_RECONCILE_ENABLED`), so a sustained burst could produce
/// ~8 identical messages/hour/table. One hour matches the alert's own
/// rolling observation window, so each surviving message covers a
/// distinct hour of observations instead of restating the same window
/// four times. Override with `LEGACY_RECONCILE_BURST_COOLDOWN_HOURS`
/// (per-site: `..._<SITE_ID_UPPER>`).
pub const DEFAULT_BURST_ALERT_COOLDOWN_HOURS: i64 = 1;

/// Track D / T7 HIGH-1 — level-triggered drift digest cooldown (per
/// table). The edge-triggered alert above fires on a rolling-window
/// volume threshold (50 rows/hr); the level-triggered digest below
/// fires when a table has ANY unresolved divergence older than
/// `LEVEL_DRIFT_STALE_INTERVAL`, capped at one alert per table per
/// `LEVEL_DRIFT_COOLDOWN`. The two are complementary: the edge alert
/// catches bulk regressions, the level alert catches single-row
/// divergences that never trip 50/hr but still represent stuck state.
///
/// **Env-overridable since 2026-07-28** (alert inventory, defect A2):
/// these were compiled-in `const`s, so retuning the ONE alert this
/// channel actually receives required a full backend deploy. Resolved
/// per tick by [`level_drift_thresholds_from_env`] with the standard
/// per-site → global → default chain. Defaults are unchanged.
pub const DEFAULT_LEVEL_DRIFT_STALE_INTERVAL_HOURS: i64 = 4;
pub const DEFAULT_LEVEL_DRIFT_COOLDOWN_HOURS: i64 = 24;

/// Second, higher staleness threshold at which the level digest changes
/// its title and tone from "unconverged" to "will not self-heal".
///
/// 2026-07-28 alert inventory, defect A1: day 1 and day 16 of the
/// 16-day incident produced byte-identical Slack text on a fixed 24h
/// rhythm — the fastest way to train an operator to dismiss an alert.
/// Past this threshold the digest escalates under its own cooldown key
/// ([`escalated_cooldown_key`]) so the transition is announced
/// immediately instead of waiting out the primary 24h window.
///
/// 72h = three consecutive daily digests ignored. By then the
/// auto-resolve sweep has had ~288 chances to close the row; it is not
/// going to, and the fix is a re-ingest / `--bootstrap`, not patience.
/// Override with `LEVEL_DRIFT_ESCALATE_HOURS` (per-site: `..._<SITE>`).
pub const DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS: i64 = 72;

/// Default per-tick CT watcher lag thresholds. Resolved at runtime via
/// [`ct_lag_thresholds_from_env`]. The version threshold catches a
/// watcher that's fallen behind the legacy CT stream (each MSSQL row
/// update bumps `CHANGE_TRACKING_CURRENT_VERSION()` by 1; 100 versions
/// is well above steady-state idle noise and well below a "the watcher
/// is wedged" scenario). The seconds threshold catches a watcher whose
/// `last_polled_at` row hasn't refreshed in a while — orthogonal to
/// version lag because a silent CT-stream gap also leaves
/// `last_polled_at` advancing while no new versions arrive.
///
/// Background: production incident 2026-05-18 — the CT watcher missed
/// 28+ UPDATEs to `HT_CheckIn_H` / `HT_CheckIn_Ds` between 2026-05-11
/// and 2026-05-15. The only detector was the reconcile sweep itself
/// (which was also broken at the time). This per-tick observation
/// closes the gap: a stuck watermark surfaces as a `[Sync] CT watcher
/// lag detected` log line every 15 minutes.
pub const DEFAULT_CT_LAG_WARN_VERSIONS: i64 = 100;
pub const DEFAULT_CT_LAG_WARN_SECONDS: i64 = 300;

/// Reconcile mode selected by env var `LEGACY_SYNC_RECONCILE_MODE`.
/// Default is `DiffOnly` per Phase 5.5 cutover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconcileMode {
    /// Phase 5.5 default. Compare row hashes; LOG divergent rows into
    /// `ht_reconcile_log` for operator investigation. Does NOT mutate
    /// canonical `ht_*` state — the CT watcher owns that.
    DiffOnly,
    /// Pre-5.5 behaviour, kept as an escape hatch. UPSERTs into
    /// `ht_*_legacy` on every divergence (the legacy reconcile path).
    /// Set explicitly only when the CT watcher is operationally
    /// disabled and we need the safety net to keep canonical state in
    /// sync.
    Upsert,
}

impl ReconcileMode {
    /// Read the mode from the env var. Default `DiffOnly`. Anything
    /// other than the two recognised strings logs a warning and falls
    /// back to the safe default.
    pub fn from_env() -> Self {
        match env::var("LEGACY_SYNC_RECONCILE_MODE")
            .as_deref()
            .map(str::trim)
        {
            Ok("upsert") => Self::Upsert,
            Ok("diff_only") | Err(_) => Self::DiffOnly,
            Ok(other) => {
                tracing::warn!(
                    value = other,
                    "Unknown LEGACY_SYNC_RECONCILE_MODE; defaulting to diff_only"
                );
                Self::DiffOnly
            }
        }
    }
}

/// Run a full sync cycle across all entity types.
/// Order: Customers -> Rooms -> Bookings -> CheckIns (respects dependencies).
///
/// Behaviour depends on `LEGACY_SYNC_RECONCILE_MODE` (see [`ReconcileMode`]).
///
/// Phase 6: pass an optional `SlackClient` to enable drift alerting.
/// When the per-table unresolved-drift count in the last hour exceeds
/// [`DEFAULT_DRIFT_ALERT_THRESHOLD`] (override:
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`, or per-site
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE_ID_UPPER>`), a Slack
/// message is fired at the end of the cycle. Pass `None` to silence
/// alerts (e.g. from `bin/sync --bootstrap` where the Slack channel is
/// reserved for CT-watcher errors and bootstrap progress).
///
/// `site_id` is plumbed through so the drift-alert message names which
/// deployment fired (HF Hotel vs HF Ville share a Slack webhook from
/// Phase 5 onward) and so the site-specific threshold env var can be
/// looked up.
pub async fn run_sync(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) {
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, site = %site_id, "[Sync] Starting sync cycle...");

    if let Err(e) = sync_customers(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Customer sync failed: {}", e);
        record_error(pg_pool, "customers", &e.to_string()).await;
    }

    if let Err(e) = sync_rooms(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Room sync failed: {}", e);
        record_error(pg_pool, "rooms", &e.to_string()).await;
    }

    if let Err(e) = sync_bookings(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Booking sync failed: {}", e);
        record_error(pg_pool, "bookings", &e.to_string()).await;
    }

    if let Err(e) = sync_checkins(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Check-in sync failed: {}", e);
        record_error(pg_pool, "checkins", &e.to_string()).await;
    }

    // Phase 6-A: payments (`HT_Receipt_H` ↔ `ht_payments`). SHIPPED DARK —
    // `RECONCILE_PAYMENTS_ARM_ENABLED` defaults false on every service, and
    // the check lives HERE (not inside `sync_payments`) so a disabled arm
    // issues literally zero MSSQL/PG queries. Runs AFTER check-ins: a
    // payment's canonical parent is `ht_checkins`, so any parent repair a
    // tick performs lands first. See `reconcile_payments_arm_enabled`.
    if reconcile_payments_arm_enabled() {
        if let Err(e) = sync_payments(legacy_pool, pg_pool).await {
            tracing::error!(site = %site_id, "[Sync] Payment sync failed: {}", e);
            record_error(pg_pool, "payments", &e.to_string()).await;
        }
    }

    // Phase 6-B: guest registry / companion folios (`HT_CheckIn_Other_People`
    // ↔ `ht_guest_registry`). SHIPPED DARK —
    // `RECONCILE_GUEST_REGISTRY_ARM_ENABLED` defaults false on every service,
    // and the check lives HERE (not inside `sync_guest_registry`) so a
    // disabled arm issues literally zero MSSQL/PG queries. Runs AFTER
    // check-ins for the same reason payments does: the parent is
    // `ht_checkins`. See `reconcile_guest_registry_arm_enabled`.
    if reconcile_guest_registry_arm_enabled() {
        if let Err(e) = sync_guest_registry(legacy_pool, pg_pool, slack, site_id).await {
            tracing::error!(site = %site_id, "[Sync] Guest-registry sync failed: {}", e);
            record_error(pg_pool, "guest_registry", &e.to_string()).await;
        }
    }

    // Phase 6-C: generic mirror-table probe (the 8 CT-mirrored
    // `legacy_mirror.*` tables + `ht_room_calendar`). SHIPPED DARK —
    // `RECONCILE_MIRROR_PROBE_ENABLED` defaults false on every service, and
    // the check lives HERE (not inside `run_mirror_probe`) so a disabled
    // probe issues literally zero MSSQL/PG queries. Runs BEFORE
    // `reload_mirror_dimensions` on purpose: the probe set and the reload
    // set are disjoint (the reload owns the 4 wholesale dimension mirrors),
    // so ordering carries no data dependency, and keeping the probe next to
    // the other reconcile arms is what makes the "one aggregate batch per
    // side" cost visible in one place.
    //
    // The `record_error` counter needs `sync_status.entity_type =
    // 'mirror_probe'` to EXIST — it is an `UPDATE … WHERE entity_type = $2`
    // and would otherwise update zero rows, leaving only the log line.
    // Migration 082 seeds it (and `init-db/init-hotelnew.sql` for a fresh
    // database), exactly as 080/081 did for the payments and guest-registry
    // arms.
    //
    // BOTH outcomes are written, as `payments` and `guest_registry` do:
    // `record_success` is the ONLY thing that zeroes `consecutive_failures`
    // and stamps `last_sync_at` (`last_error`/`last_error_at` keep the most
    // recent failure). With only the error arm wired, `consecutive_failures`
    // for `mirror_probe` would be a monotonic LIFETIME failure count (unique
    // among the entity_types) and `last_sync_at` would stay NULL forever — a
    // reading that is actively misleading rather than merely absent.
    if reconcile_mirror_probe_enabled() {
        match crate::scheduler::mirror_probe::run_mirror_probe(legacy_pool, pg_pool).await {
            Ok(outcome) => {
                // added=0 (the probe writes no canonical rows),
                // updated=`recorded` (divergence rows written this tick),
                // unchanged=`converged` (probes whose aggregates agreed).
                record_success(
                    pg_pool,
                    "mirror_probe",
                    0,
                    outcome.recorded as i32,
                    outcome.converged as i32,
                    outcome.duration_ms,
                )
                .await;
            }
            Err(e) => {
                tracing::error!(site = %site_id, "[Sync] Mirror probe failed: {}", e);
                record_error(pg_pool, "mirror_probe", &e.to_string()).await;
            }
        }
    }

    // Phase 6-D: per-FOLIO payment-ledger probe (`HT_CheckIn_Pay` ↔
    // `ht_payment_ledger`). SHIPPED DARK —
    // `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED` defaults false on every
    // service, and the check lives HERE (not inside the probe) so a disabled
    // probe issues literally zero MSSQL/PG queries. Runs AFTER the 6-A
    // `payments` arm on purpose: that arm reconciles the RECEIPT artefact
    // (`ht_payments`), this one the per-line tender ledger underneath it, so
    // an operator reading a tick's log sees the receipt-level answer before
    // the line-level one.
    //
    // Same `sync_status` requirement as 6-C: `record_error` is an
    // `UPDATE … WHERE entity_type = $2`, so without the
    // `entity_type = 'payment_ledger_probe'` row (migration 083, and
    // `init-db/init-hotelnew.sql` for a fresh database) a probe failure
    // updates zero rows and leaves only a log line. BOTH outcomes are
    // written, because `record_success` is the ONLY thing that zeroes
    // `consecutive_failures` and stamps `last_sync_at` (`last_error`/
    // `last_error_at` keep the most recent failure).
    if reconcile_payment_ledger_probe_enabled() {
        match crate::scheduler::payment_ledger_probe::run_payment_ledger_probe(
            legacy_pool,
            pg_pool,
        )
        .await
        {
            Ok(outcome) => {
                // added=0 (the probe writes no canonical rows),
                // updated=`recorded` (divergence rows written this tick),
                // unchanged=`converged` (folios that agreed).
                record_success(
                    pg_pool,
                    PAYMENT_LEDGER_PROBE_KEY,
                    0,
                    outcome.recorded as i32,
                    outcome.converged as i32,
                    outcome.duration_ms,
                )
                .await;
            }
            Err(e) => {
                tracing::error!(site = %site_id, "[Sync] Payment-ledger probe failed: {}", e);
                record_error(pg_pool, PAYMENT_LEDGER_PROBE_KEY, &e.to_string()).await;
            }
        }
    }

    // Phase 5.5a: full-table reload of legacy-only dimension tables into
    // legacy_mirror.*. Independent of the canonical reconcile above —
    // these tables are slow-changing reference data (pricing tiers,
    // hourly extension prices), reloaded in their own TX, and don't
    // interact with ReconcileMode (always full-reload).
    crate::scheduler::mirror::reload_mirror_dimensions(legacy_pool, pg_pool).await;

    // Track D / T7 follow-up — auto-resolve previously-recorded
    // divergences whose freshly-projected legacy hash matches the
    // freshly-projected canonical PG hash. BOTH sides are re-hashed
    // under the CURRENT projection so a change to a
    // `*_RECONCILE_PROJECTION` constant self-heals: the next sweep
    // tick re-evaluates rows whose stored `mssql_hash` was computed
    // under the now-superseded projection. Runs before the alert
    // queries so the alerts only surface drift that still persists.
    // Best-effort: a PG failure logs a warning and the alerts
    // proceed with stale state.
    if let Err(e) = auto_resolve_reconcile_log(legacy_pool, pg_pool, site_id).await {
        tracing::warn!(
            site = %site_id,
            error = %e,
            "[Sync] Auto-resolve sweep failed — alerts may include rows that have since converged"
        );
    }

    // Phase 6: drift-alert tripwire. Best-effort — degraded observability
    // never aborts the reconcile loop.
    check_drift_and_alert(pg_pool, slack, site_id).await;

    // Track D / T7 HIGH-1: level-triggered drift digest. Catches
    // long-lived single-row divergences that never breach the
    // edge-triggered 50/hr volume threshold. Per-table cooldown keeps
    // Slack from drowning during a known-bad cardinality migration
    // window.
    check_level_drift_and_alert(pg_pool, slack, site_id).await;

    // Incident 2026-05-18 follow-up — per-tick CT watcher health
    // observation. Compares the PG-side `legacy_ct_state.last_seen_version`
    // against MSSQL's `CHANGE_TRACKING_CURRENT_VERSION()` and warns on
    // lag. Log-only for now (no Slack); operators grep for the warning
    // text. Best-effort: any MSSQL/PG failure logs a warning and returns
    // without aborting the reconcile loop.
    check_ct_watcher_lag(legacy_pool, pg_pool, site_id).await;

    tracing::info!(site = %site_id, "[Sync] Sync cycle complete");
}

/// Phase 6: read the configured drift-alert threshold (default
/// [`DEFAULT_DRIFT_ALERT_THRESHOLD`]). Anything that doesn't parse to a
/// positive integer falls back to the safe default with a warning.
///
/// Task #69 — per-site override. Resolution order, first match wins:
///   1. `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE_ID_UPPER>` —
///      site-specific knob (e.g. `..._HFVILLE=20` to set a tighter
///      threshold for the smaller property).
///   2. `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD` — global, applies to
///      all sites that don't have a per-site override.
///   3. [`DEFAULT_DRIFT_ALERT_THRESHOLD`] — compiled-in fallback (50).
fn drift_alert_threshold_from_env(site_id: &str) -> i64 {
    threshold_from_env(
        "LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD",
        site_id,
        DEFAULT_DRIFT_ALERT_THRESHOLD,
    )
}

/// Per-`(site, table)` cooldown for the edge-triggered burst alert.
/// Same resolution chain as [`drift_alert_threshold_from_env`], default
/// [`DEFAULT_BURST_ALERT_COOLDOWN_HOURS`].
fn burst_cooldown_hours_from_env(site_id: &str) -> i64 {
    threshold_from_env(
        "LEGACY_RECONCILE_BURST_COOLDOWN_HOURS",
        site_id,
        DEFAULT_BURST_ALERT_COOLDOWN_HOURS,
    )
}

/// Inner helper: parse a single env var into a positive `i64` threshold.
/// Returns `None` if the var is unset OR parses to a non-positive /
/// non-numeric value (with a warning logged for the latter so an operator
/// notices a typo). Pulled out so the per-site / global fallback chain
/// in [`drift_alert_threshold_from_env`] reads as a single `or_else`.
fn parse_threshold_env(var_name: &str) -> Option<i64> {
    match env::var(var_name) {
        Ok(raw) => match raw.trim().parse::<i64>() {
            Ok(n) if n > 0 => Some(n),
            _ => {
                tracing::warn!(
                    var = var_name,
                    value = %raw,
                    "Invalid drift-alert threshold env var; ignoring"
                );
                None
            }
        },
        Err(_) => None,
    }
}

/// Generalisation of the resolution order baked into
/// [`drift_alert_threshold_from_env`]: per-site override, then global,
/// then the compiled-in default. Kept as one helper so every knob in
/// this module resolves identically instead of each one re-spelling the
/// `or_else` chain.
fn threshold_from_env(base_var: &str, site_id: &str, default: i64) -> i64 {
    let per_site_var = format!("{base_var}_{}", site_id.to_uppercase());
    parse_threshold_env(&per_site_var)
        .or_else(|| parse_threshold_env(base_var))
        .unwrap_or(default)
}

/// Resolved level-drift digest thresholds for one reconcile tick.
/// Produced by [`level_drift_thresholds_from_env`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelDriftThresholds {
    /// A row unresolved for longer than this is "stale" and eligible for
    /// the `:warning:` digest. `LEVEL_DRIFT_STALE_INTERVAL_HOURS`.
    pub stale_hours: i64,
    /// One digest per `(site, table)` per this many hours.
    /// `LEVEL_DRIFT_COOLDOWN_HOURS`.
    pub cooldown_hours: i64,
    /// Oldest-row age at which the digest escalates to
    /// "will not self-heal". `LEVEL_DRIFT_ESCALATE_HOURS`.
    pub escalate_hours: i64,
}

impl LevelDriftThresholds {
    /// Cooldown as a `Duration`, for [`cooldown_elapsed`].
    fn cooldown(&self) -> std::time::Duration {
        std::time::Duration::from_secs((self.cooldown_hours * 3600) as u64)
    }
}

/// Resolve the level-drift digest thresholds from env (defect A2 of the
/// 2026-07-28 alert inventory: these used to be compiled-in `const`s, so
/// retuning the only alert this channel actually receives cost a deploy).
///
/// Same per-site → global → default chain as
/// [`drift_alert_threshold_from_env`], via [`threshold_from_env`]:
///   1. `LEVEL_DRIFT_STALE_INTERVAL_HOURS_<SITE_ID_UPPER>` etc.
///   2. `LEVEL_DRIFT_STALE_INTERVAL_HOURS` etc.
///   3. the `DEFAULT_LEVEL_DRIFT_*` constants.
///
/// Non-numeric / non-positive values are ignored with a warning by
/// [`parse_threshold_env`], so an operator typo degrades to the previous
/// tier rather than to zero.
pub fn level_drift_thresholds_from_env(site_id: &str) -> LevelDriftThresholds {
    let stale_hours = threshold_from_env(
        "LEVEL_DRIFT_STALE_INTERVAL_HOURS",
        site_id,
        DEFAULT_LEVEL_DRIFT_STALE_INTERVAL_HOURS,
    );
    let cooldown_hours = threshold_from_env(
        "LEVEL_DRIFT_COOLDOWN_HOURS",
        site_id,
        DEFAULT_LEVEL_DRIFT_COOLDOWN_HOURS,
    );
    let mut escalate_hours = threshold_from_env(
        "LEVEL_DRIFT_ESCALATE_HOURS",
        site_id,
        DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS,
    );
    // An escalation threshold at or below the stale threshold would make
    // EVERY stale table escalate on its first digest, collapsing the two
    // tiers back into one voice — the exact defect this is fixing. Clamp
    // loudly rather than silently degrade.
    if escalate_hours <= stale_hours {
        tracing::warn!(
            site = %site_id,
            escalate_hours,
            stale_hours,
            "[Sync] LEVEL_DRIFT_ESCALATE_HOURS must exceed the stale interval; \
             clamping to stale + 1h"
        );
        escalate_hours = stale_hours + 1;
    }
    LevelDriftThresholds {
        stale_hours,
        cooldown_hours,
        escalate_hours,
    }
}

/// Severity tier of one table's level-drift digest, decided purely from
/// the age of its OLDEST unresolved `ht_reconcile_log` row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelDriftSeverity {
    /// Past the stale interval, below the escalation threshold. The
    /// familiar `:warning:` digest — "the sweep has not closed this yet".
    Stale,
    /// At or past the escalation threshold. Re-titled and re-toned: the
    /// row is not going to converge on its own and needs a re-ingest or
    /// `--bootstrap`. Rides its own cooldown key so the transition is
    /// announced immediately rather than waiting out the primary window.
    Escalated,
}

/// Pure classifier for [`LevelDriftSeverity`].
///
/// Boundary is inclusive (`>=`): an oldest-row age of exactly
/// `escalate_hours` escalates. The caller floors the age to whole hours
/// in SQL, so `>= 72` means "at least 72h old" with no rounding-up risk.
pub fn level_drift_severity(oldest_age_hours: i64, escalate_hours: i64) -> LevelDriftSeverity {
    if oldest_age_hours >= escalate_hours {
        LevelDriftSeverity::Escalated
    } else {
        LevelDriftSeverity::Stale
    }
}

/// Namespace separator for cooldown keys in
/// `ht_level_drift_alert_cooldowns` that are NOT canonical entity names.
///
/// The cooldown table is keyed `(site_id, table_name)` and is shared by
/// four emitters: the reconcile digest (real entity names — `bookings`,
/// `customers`), the stale-checkin tripwire (the bare
/// [`STALE_CHECKIN_COOLDOWN_KEY`] sentinel), `bin/sync.rs`'s
/// retention-overflow pages (`ct_retention_overflow:<table>`), and now
/// the burst alert and escalation tier here. Anything carrying this
/// separator is by construction not an entity name, which is what makes
/// the collision impossible rather than merely unlikely.
const COOLDOWN_KEY_NAMESPACE_SEP: char = ':';

/// Cooldown key for the ESCALATED tier of the level digest. Mirrors the
/// `ct_retention_overflow:<table>` shape in `bin/sync.rs` so escalation
/// keys can never collide with a canonical entity name — critically,
/// `escalated:bookings` must never be mistaken for the table `bookings`
/// by the all-clear path.
pub fn escalated_cooldown_key(table: &str) -> String {
    format!("escalated{COOLDOWN_KEY_NAMESPACE_SEP}{table}")
}

/// Cooldown key for the edge-triggered burst alert
/// ([`check_drift_and_alert`]). Same namespacing rationale as
/// [`escalated_cooldown_key`].
pub fn burst_cooldown_key(table: &str) -> String {
    format!("burst{COOLDOWN_KEY_NAMESPACE_SEP}{table}")
}

/// Cooldown key for the per-tick divergence-cap page — the "this arm was
/// about to enqueue an implausible number of findings, so it wrote nothing"
/// alert (see [`divergence_cap_exceeded`]).
///
/// Namespaced like its siblings so the sync-lag all-clear can never mistake
/// `reconcile_cap:guest_registry` for the table `guest_registry` and delete
/// its cooldown, which would un-throttle the page to once per tick. The
/// `reconcile_cap` family is new and shares no prefix with
/// `ct_retention_overflow:` / `escalated:` / `burst:` / `ct_watcher_lag:` /
/// `shadow_mode:` / `boot_refusal:`.
pub fn reconcile_cap_cooldown_key(table: &str) -> String {
    format!("reconcile_cap{COOLDOWN_KEY_NAMESPACE_SEP}{table}")
}

/// How an alert actually reached (or failed to reach) an operator on a
/// given tick. Feeds [`cooldown_should_be_marked`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertDelivery {
    /// Slack accepted the POST.
    Sent,
    /// Slack is not configured for this deployment; the `tracing` line IS
    /// the delivery channel, so the cooldown still applies (otherwise the
    /// log repeats every 15 minutes).
    LoggedOnly,
    /// Slack was configured and the POST failed.
    Failed,
}

impl AlertDelivery {
    /// Classify a `SlackClient::send_message` outcome. `None` = no client
    /// configured.
    fn from_send(result: Option<bool>) -> Self {
        match result {
            None => AlertDelivery::LoggedOnly,
            Some(true) => AlertDelivery::Sent,
            Some(false) => AlertDelivery::Failed,
        }
    }
}

/// Pure decision function: should this tick burn the cooldown?
///
/// 2026-07-28 alert inventory, defect A3: every cooldown in this module
/// was marked BEFORE the Slack POST (and before the `slack.is_some()`
/// check), so a webhook outage silenced the table for a full 24h anyway
/// — and the paired all-clear could later fire as the closure of an
/// alert nobody ever received. A failed send must leave the cooldown
/// untouched so the next 15-min tick retries.
pub fn cooldown_should_be_marked(delivery: AlertDelivery) -> bool {
    match delivery {
        AlertDelivery::Sent | AlertDelivery::LoggedOnly => true,
        AlertDelivery::Failed => false,
    }
}

/// Pure decision function: given per-table unresolved-drift counts in
/// the alerting window, return the `(table_name, count)` pairs that
/// breached the threshold (count strictly greater than threshold).
///
/// Pulled out for unit testing — no PG dependency.
pub fn tables_breaching_threshold(counts: &[(String, i64)], threshold: i64) -> Vec<(String, i64)> {
    counts
        .iter()
        .filter(|(_, n)| *n > threshold)
        .cloned()
        .collect()
}

/// Slack body for the edge-triggered sync-lag burst page. Pure — pulled
/// out of [`check_drift_and_alert`] so the composition is unit-testable
/// without a PG pool. Pager tier (issue #261): the caller wraps this in
/// [`SlackMessage::with_site_text_paged`], not `with_site_text`.
fn format_burst_alert_message(threshold: i64, body: &str, cooldown_hours: i64) -> String {
    format!(
        ":rotating_light: *Sync lag burst — threshold exceeded* :rotating_light:\n\
         The reconcile sweep observed more than {threshold} unconverged \
         `ht_reconcile_log` row(s) for the following table(s) in the last hour. \
         Most clear on their own as the CT watcher / writeback catch up; this \
         alert surfaces a burst that may indicate a real backlog:\n\
         {body}\n\
         _Investigate via `docs/runbook-sync.md` §9 (Phase 6 drift alert). \
         Per-table cooldown {cooldown_hours}h — the log line still fires every \
         tick._"
    )
}

/// Phase 6 alerting: count unresolved `ht_reconcile_log` rows added in
/// the last hour, grouped by `table_name`. If any table breaches the
/// configured threshold, emit ONE Slack message listing the offenders.
/// Best-effort — a failed Slack POST or PG query only logs a warning.
///
/// Uses `idx_ht_reconcile_log_table_unresolved` (migration 019) for
/// the partial-index group-by.
///
/// **Cardinality excluded** (2026-05-18, post-incident #128): the
/// `cardinality` divergence kind is unack-able by design (see
/// `DivergenceKind::Cardinality` docstring + Track D / T7 CRIT-1) and
/// re-fires from `sync_checkins` every 15-min tick for every multi-room
/// folio that still exists in legacy MSSQL. On HF Hotel that's currently
/// ~766 unique PKs producing ~3000 inserts/hour, drowning any genuine
/// silenceable-drift signal under fixed-volume noise. Cardinality is
/// already covered by `check_level_drift_and_alert` (4h window, 24h
/// per-table cooldown — won't spam), so dropping it from the hourly
/// edge-trigger loses no observability. The hourly alert is now scoped
/// to kinds where re-detection actually means something changed:
/// `value`, `missing_pg`, `missing_mssql`.
///
/// **Per-table cooldown** (2026-07-28 alert inventory, defect C5): this
/// alert previously had none, so while a table stayed above threshold it
/// re-fired every 15-min tick — doubled on HF Ville, whose worker
/// reconcile is a second independent emitter of the same message. The
/// cooldown reuses the durable `ht_level_drift_alert_cooldowns` table
/// under the namespaced [`burst_cooldown_key`], defaulting to
/// [`DEFAULT_BURST_ALERT_COOLDOWN_HOURS`]. Note the eligibility read and
/// the mark are not one atomic claim (unlike `bin/sync.rs`'s retention
/// pages): the two emitters tick independently minutes apart, so the
/// residual race is a single duplicate message — cheaper than trading
/// away the mark-on-successful-send property below.
async fn check_drift_and_alert(pg_pool: &PgPool, slack: Option<&SlackClient>, site_id: &str) {
    let threshold = drift_alert_threshold_from_env(site_id);

    let rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT table_name, count(*) \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
            AND divergence_kind <> 'cardinality' \
            AND detected_at > now() - interval '1 hour' \
          GROUP BY table_name",
    )
    .fetch_all(pg_pool)
    .await;

    let counts = match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to query ht_reconcile_log for drift alert — observability degraded"
            );
            return;
        }
    };

    let breaches = tables_breaching_threshold(&counts, threshold);
    if breaches.is_empty() {
        tracing::debug!(
            site = %site_id,
            tables_observed = counts.len(),
            threshold,
            "[Sync] Drift alert: no tables breach threshold"
        );
        return;
    }

    // Always log every breach; the cooldown below throttles Slack only.
    // An operator grepping the logs during an incident wants each tick.
    for (table, count) in &breaches {
        tracing::warn!(
            site = %site_id,
            table,
            count,
            threshold,
            "[Sync] Drift alert: table exceeds reconcile-log threshold in last hour"
        );
    }

    // Defect C5 — per-table cooldown so a sustained burst doesn't restate
    // the same rolling window every 15 minutes (x2 emitters on HF Ville).
    let cooldown_hours = burst_cooldown_hours_from_env(site_id);
    let cooldown = std::time::Duration::from_secs((cooldown_hours * 3600) as u64);
    let mut to_alert: Vec<(String, i64)> = Vec::new();
    for (table, count) in &breaches {
        if level_alert_eligible_pg(pg_pool, site_id, &burst_cooldown_key(table), cooldown).await {
            to_alert.push((table.clone(), *count));
        } else {
            tracing::debug!(
                site = %site_id,
                table,
                count,
                cooldown_hours,
                "[Sync] Drift burst alert suppressed by cooldown"
            );
        }
    }
    if to_alert.is_empty() {
        return;
    }

    let delivery = if let Some(slack) = slack {
        let body = to_alert
            .iter()
            .map(|(t, n)| format!("• `{t}`: {n} unresolved rows in last hour"))
            .collect::<Vec<_>>()
            .join("\n");
        let msg = SlackMessage::with_site_text_paged(
            site_id,
            format_burst_alert_message(threshold, &body, cooldown_hours),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; drift alert logged only ({} table(s) breaching)",
            to_alert.len()
        );
        AlertDelivery::LoggedOnly
    };

    // Defect A3 — burn the cooldown only if the alert actually landed.
    if cooldown_should_be_marked(delivery) {
        for (table, _) in &to_alert {
            mark_level_alert_sent_pg(pg_pool, site_id, &burst_cooldown_key(table)).await;
        }
    } else {
        tracing::warn!(
            site = %site_id,
            tables = to_alert.len(),
            "[Sync] Drift burst alert POST failed — leaving cooldown unset so the \
             next tick retries"
        );
    }
}

/// Track D / T7 HIGH-1 — pure decision function for the level-triggered
/// cooldown gate. Returns true iff the cooldown has elapsed (or there
/// is no prior alert record).
///
/// **Persistence (2026-05-16):** the cooldown state was migrated from
/// a process-local `Mutex<HashMap>` to the PG table
/// `ht_level_drift_alert_cooldowns` (migration 053). The in-memory
/// version was wiped on every backend restart — typical 2-5x/day during
/// active deploy cycles — and the next reconcile tick re-fired the
/// `:warning:` alert despite the documented 24h cooldown. The Slack
/// channel got the same alert repeatedly for the same unchanged
/// backlog. Production incident 2026-05-16.
///
/// This function stays pure (no PG dependency) so the unit tests can
/// inject `now` and `last_alerted_at` directly. The PG layer is in
/// `level_alert_eligible_pg` / `mark_level_alert_sent_pg`.
fn cooldown_elapsed(
    last_alerted_at: Option<chrono::DateTime<chrono::Utc>>,
    now: chrono::DateTime<chrono::Utc>,
    cooldown: std::time::Duration,
) -> bool {
    match last_alerted_at {
        None => true,
        // `signed_duration_since.to_std()` errors when `now < last` — a
        // clock-skew edge case. Treat as "eligible to alert" so we
        // don't get stuck refusing forever on a clock anomaly.
        Some(t) => now
            .signed_duration_since(t)
            .to_std()
            .map_or(true, |elapsed| elapsed >= cooldown),
    }
}

/// PG-backed eligibility check. Reads `last_alerted_at` for
/// `(site_id, table_name)` from `ht_level_drift_alert_cooldowns`,
/// applies the [`cooldown_elapsed`] decision. Returns `true` on PG
/// error so a transient DB blip doesn't silence a legitimate alert.
async fn level_alert_eligible_pg(
    pg_pool: &PgPool,
    site_id: &str,
    table_name: &str,
    cooldown: std::time::Duration,
) -> bool {
    let last = sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(
        "SELECT last_alerted_at FROM ht_level_drift_alert_cooldowns \
          WHERE site_id = $1 AND table_name = $2",
    )
    .bind(site_id)
    .bind(table_name)
    .fetch_optional(pg_pool)
    .await;

    match last {
        Ok(opt) => cooldown_elapsed(opt, chrono::Utc::now(), cooldown),
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                table = %table_name,
                error = %e,
                "[Sync] Failed to read level-drift cooldown row — \
                 defaulting to eligible to avoid silencing real alerts"
            );
            true
        }
    }
}

/// PG-backed cooldown mark. UPSERTs `(site_id, table_name) → NOW()`
/// in `ht_level_drift_alert_cooldowns`. Best-effort: a PG failure logs
/// a warning but doesn't block the alert from going out — the worst
/// case is one extra refire in 15 minutes.
async fn mark_level_alert_sent_pg(pg_pool: &PgPool, site_id: &str, table_name: &str) {
    let result = sqlx::query(
        "INSERT INTO ht_level_drift_alert_cooldowns \
            (site_id, table_name, last_alerted_at) \
         VALUES ($1, $2, NOW()) \
         ON CONFLICT (site_id, table_name) \
         DO UPDATE SET last_alerted_at = EXCLUDED.last_alerted_at",
    )
    .bind(site_id)
    .bind(table_name)
    .execute(pg_pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(
            site = %site_id,
            table = %table_name,
            error = %e,
            "[Sync] Failed to persist level-drift cooldown — \
             next tick may refire the alert"
        );
    }
}

/// Read every cooldown key currently recorded for `site_id` in
/// `ht_level_drift_alert_cooldowns`. This is the "we alerted about this
/// at some point and never told anyone it cleared" set that the recovery
/// notification (see [`check_level_drift_recovery_and_notify`]) diffs
/// against the tables that are still unconverged.
///
/// Fail-soft in the same style as [`level_alert_eligible_pg`]: a PG error
/// yields an empty set (no all-clear fired this tick) plus a warning, and
/// never aborts the sweep.
async fn level_alert_cooldown_keys_pg(pg_pool: &PgPool, site_id: &str) -> Vec<String> {
    let rows = sqlx::query_scalar::<_, String>(
        "SELECT table_name FROM ht_level_drift_alert_cooldowns WHERE site_id = $1",
    )
    .bind(site_id)
    .fetch_all(pg_pool)
    .await;

    match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to read level-drift cooldown keys — \
                 skipping the sync-lag all-clear this tick"
            );
            Vec::new()
        }
    }
}

/// Does a cooldown row exist for this exact `(site_id, key)`? i.e. "did
/// we alert about this at some point and never announce that it
/// cleared?" — the precondition for firing a paired all-clear.
///
/// Used by the tripwires that own a single sentinel key rather than a set
/// of entity names (currently the stale-checkin tripwire), where the
/// bulk [`level_alert_cooldown_keys_pg`] read would be wasteful.
///
/// **Fails CLOSED** (`false` on PG error), the opposite of
/// [`level_alert_eligible_pg`]: failing open there avoids silencing a
/// real alert, whereas failing open here would invent an all-clear for a
/// condition we cannot confirm ever alerted.
async fn cooldown_row_exists_pg(pg_pool: &PgPool, site_id: &str, key: &str) -> bool {
    let found = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM ht_level_drift_alert_cooldowns \
          WHERE site_id = $1 AND table_name = $2",
    )
    .bind(site_id)
    .bind(key)
    .fetch_optional(pg_pool)
    .await;

    match found {
        Ok(opt) => opt.is_some(),
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                key = %key,
                error = %e,
                "[Sync] Failed to read cooldown row — skipping the paired all-clear \
                 this tick"
            );
            false
        }
    }
}

/// Drop the `(site_id, table_name)` cooldown row so a RECURRENCE alerts
/// on the very next tick instead of being swallowed by a stale 24h
/// window. Called only after the paired `:white_check_mark:` all-clear
/// has been emitted for that table.
///
/// Best-effort: a PG failure logs a warning and leaves the row in place —
/// the worst case is one duplicate all-clear on the next tick.
async fn clear_level_alert_cooldown_pg(pg_pool: &PgPool, site_id: &str, table_name: &str) {
    let result = sqlx::query(
        "DELETE FROM ht_level_drift_alert_cooldowns \
          WHERE site_id = $1 AND table_name = $2",
    )
    .bind(site_id)
    .bind(table_name)
    .execute(pg_pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(
            site = %site_id,
            table = %table_name,
            error = %e,
            "[Sync] Failed to clear level-drift cooldown after all-clear — \
             a recurrence may stay suppressed until the 24h window lapses"
        );
    }
}

/// Cooldown keys that live in `ht_level_drift_alert_cooldowns` but are
/// NOT `ht_reconcile_log` table names. The cooldown table is shared: the
/// stale-checkin tripwire parks its own key there
/// ([`STALE_CHECKIN_COOLDOWN_KEY`]) so it inherits the same 24h
/// per-site window. Those keys have no unconverged-row count to compare
/// against, so the sync-lag all-clear must never claim them recovered or
/// clear their cooldown — doing so would let the stale-checkin alert
/// refire every 15 minutes.
///
/// This list holds only the BARE sentinels. Namespaced keys (anything
/// containing [`COOLDOWN_KEY_NAMESPACE_SEP`] — `escalated:…`, `burst:…`,
/// and `bin/sync.rs`'s `ct_retention_overflow:…`) are excluded
/// structurally by [`is_reconcile_table_key`] and don't need enumerating
/// here; that is the whole point of the namespace.
///
/// The stale-checkin key stays here even though it now HAS its own
/// all-clear (2026-07-28, defect C8): the closure is owned by
/// [`check_stale_active_checkins_and_alert`], which is the only caller
/// that can evaluate the condition. The reconcile sweep must keep its
/// hands off it.
const NON_RECONCILE_COOLDOWN_KEYS: &[&str] = &[STALE_CHECKIN_COOLDOWN_KEY];

/// Is this cooldown key a canonical `ht_reconcile_log` table name — i.e.
/// something the sync-lag all-clear is entitled to declare recovered?
///
/// Two exclusions: the bare sentinels in [`NON_RECONCILE_COOLDOWN_KEYS`],
/// and anything namespaced with [`COOLDOWN_KEY_NAMESPACE_SEP`]. The
/// second was a latent bug before 2026-07-28 — `bin/sync.rs` has parked
/// `ct_retention_overflow:<table>` rows in this shared table since the
/// retention-page work, and the all-clear would happily list one as a
/// "converged" reconcile table and DELETE its cooldown, un-throttling
/// the retention pages. Adding `escalated:` / `burst:` keys here makes
/// that structural rather than a list to remember to update.
fn is_reconcile_table_key(key: &str) -> bool {
    !NON_RECONCILE_COOLDOWN_KEYS.contains(&key) && !key.contains(COOLDOWN_KEY_NAMESPACE_SEP)
}

/// Pure decision helper for the sync-lag all-clear. Given the cooldown
/// keys recorded for a site and the tables that STILL have unconverged
/// `ht_reconcile_log` rows past the stale threshold, return the tables
/// that have recovered — i.e. we alerted about them at some point and
/// they now have zero stale rows.
///
/// Non-reconcile cooldown keys (see [`is_reconcile_table_key`]) are
/// excluded: they are parked in the same table by other tripwires and
/// carry no reconcile-row semantics. Output is de-duplicated and sorted
/// so the Slack body and the log lines are deterministic.
///
/// Kept free of PG so the state-transition tests are plain unit tests.
fn tables_recovered(cooldown_keys: &[String], still_stale_tables: &[String]) -> Vec<String> {
    let mut recovered: Vec<String> = cooldown_keys
        .iter()
        .filter(|k| is_reconcile_table_key(k))
        .filter(|k| !still_stale_tables.iter().any(|s| s == *k))
        .cloned()
        .collect();
    recovered.sort();
    recovered.dedup();
    recovered
}

/// Slack body for the level-drift all-clear. Pure — unit-testable
/// without a PG pool. All-clear tier (issue #261) — stays on
/// `with_site_text`, never the pager mention.
fn format_level_drift_all_clear_message(stale_hours: i64, body: &str, cooldown_hours: i64) -> String {
    format!(
        ":white_check_mark: *Reconcile rows CONVERGED* :white_check_mark:\n\
         Every `ht_reconcile_log` row older than \
         {stale_hours}h has converged for:\n\
         {body}\n\
         _Closure of the_ `:warning:` _unconverged alert sent earlier. The \
         per-table {cooldown_hours}h cooldown is reset, so a \
         recurrence alerts on the next tick instead of waiting out a stale \
         window._"
    )
}

/// Paired recovery notification for [`check_level_drift_and_alert`].
///
/// The level-triggered alert was fire-and-forget: an operator who fixed
/// the lag got silence, and a recurrence inside the 24h cooldown was
/// silent too. This closes both gaps — when a table that currently HAS a
/// cooldown row no longer has any unconverged rows older than the stale
/// threshold, emit ONE `:white_check_mark:` all-clear naming the site and
/// the table(s), then drop those cooldown rows so a recurrence alerts
/// immediately.
///
/// Matches the two recovery-notification precedents in this codebase:
/// `bin/writeback.rs::send_resolved_alert` (exhausted job RESOLVED) and
/// `bin/sync.rs::format_recovery_message` (CT watermark RECOVERED).
///
/// Alert hygiene only — reads and writes NOTHING but the cooldown table,
/// and is deliberately NOT behind any data-write feature flag.
/// Best-effort throughout: a PG or Slack failure logs and continues.
async fn check_level_drift_recovery_and_notify(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
    still_stale_tables: &[String],
    thresholds: LevelDriftThresholds,
) {
    let cooldown_keys = level_alert_cooldown_keys_pg(pg_pool, site_id).await;
    let recovered = tables_recovered(&cooldown_keys, still_stale_tables);

    if recovered.is_empty() {
        tracing::debug!(
            site = %site_id,
            cooldown_keys = cooldown_keys.len(),
            still_stale = still_stale_tables.len(),
            "[Sync] Sync-lag all-clear: nothing recovered this tick"
        );
        return;
    }

    let stale_hours = thresholds.stale_hours;
    let cooldown_hours = thresholds.cooldown_hours;

    for table in &recovered {
        tracing::info!(
            site = %site_id,
            table,
            stale_hours,
            "[Sync] Sync-lag all-clear: table has no unconverged rows past threshold — \
             clearing level-alert cooldown"
        );
    }

    let delivery = if let Some(slack) = slack {
        let body = recovered
            .iter()
            .map(|t| format!("• `{t}`"))
            .collect::<Vec<_>>()
            .join("\n");
        let msg = SlackMessage::with_site_text(
            site_id,
            format_level_drift_all_clear_message(stale_hours, &body, cooldown_hours),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; sync-lag all-clear logged only ({} table(s))",
            recovered.len()
        );
        AlertDelivery::LoggedOnly
    };

    // Defect A3, all-clear side: clearing the cooldown is the act that
    // ERASES the record that we ever alerted. Doing it after a failed
    // POST loses the closure permanently — the operator never hears the
    // `:warning:` was resolved and nothing will ever say so again. Keep
    // the rows; the next tick re-detects recovery and retries.
    if !cooldown_should_be_marked(delivery) {
        tracing::warn!(
            site = %site_id,
            tables = recovered.len(),
            "[Sync] Sync-lag all-clear POST failed — keeping cooldown rows so the \
             next tick retries the closure"
        );
        return;
    }

    for table in &recovered {
        clear_level_alert_cooldown_pg(pg_pool, site_id, table).await;
        // The escalated tier parks its own namespaced key
        // ([`escalated_cooldown_key`]); it is invisible to
        // `tables_recovered` by construction, so clear it alongside its
        // parent table or a recurrence would stay escalation-suppressed
        // for up to a full cooldown window. Unconditional DELETE — a
        // missing row is a no-op.
        clear_level_alert_cooldown_pg(pg_pool, site_id, &escalated_cooldown_key(table)).await;
    }
}

/// One row of the level-drift digest query: a table, how many of its
/// `ht_reconcile_log` rows are still unresolved past the stale interval,
/// and the whole-hour age of the OLDEST of them.
///
/// The age is what makes day 1 distinguishable from day 16 — see
/// [`level_drift_severity`] and [`humanize_hours`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleTable {
    pub table: String,
    pub count: i64,
    pub oldest_age_hours: i64,
}

/// Render an hour count for an operator. Under two days, plain hours
/// (`7h`) — the familiar shape. Past that, lead with days because "388h"
/// does not read as "this has been broken for sixteen days".
pub fn humanize_hours(hours: i64) -> String {
    if hours < 48 {
        return format!("{hours}h");
    }
    let days = hours / 24;
    let rem = hours % 24;
    if rem == 0 {
        format!("{days}d ({hours}h)")
    } else {
        format!("{days}d {rem}h ({hours}h)")
    }
}

/// Split the digest rows into the `:warning:` tier and the escalated
/// tier, preserving input order within each. Pure — the PG-free half of
/// [`check_level_drift_and_alert`]'s decision.
pub fn partition_level_drift(
    rows: &[StaleTable],
    escalate_hours: i64,
) -> (Vec<StaleTable>, Vec<StaleTable>) {
    let mut stale = Vec::new();
    let mut escalated = Vec::new();
    for row in rows {
        match level_drift_severity(row.oldest_age_hours, escalate_hours) {
            LevelDriftSeverity::Stale => stale.push(row.clone()),
            LevelDriftSeverity::Escalated => escalated.push(row.clone()),
        }
    }
    (stale, escalated)
}

/// Track D / T7 HIGH-1 — level-triggered drift digest. Complements the
/// edge-triggered `check_drift_and_alert` above: that one fires on
/// volume (50 rows/hr in a single table), this one fires on persistence
/// (ANY unresolved row older than 4 hours per table). The edge alert
/// catches bulk regressions; the level alert catches single-row
/// divergences that never trip the volume threshold.
///
/// Behaviour:
/// - Counts unresolved rows per `table_name` where `detected_at` is older
///   than the stale interval (`LEVEL_DRIFT_STALE_INTERVAL_HOURS`,
///   default 4h), along with the age of the oldest such row.
/// - Below the escalation threshold, emits the `:warning:` digest if the
///   per-table cooldown (`LEVEL_DRIFT_COOLDOWN_HOURS`, default 24h) has
///   elapsed since the last level alert for that table+site. The body
///   now carries the oldest-row age, so consecutive digests are visibly
///   different messages rather than the same text on a 24h metronome.
/// - At or past `LEVEL_DRIFT_ESCALATE_HOURS` (default 72h) the table
///   moves to the escalated tier: different title, different ask ("this
///   will not self-heal — re-ingest or bootstrap"), and its OWN cooldown
///   key ([`escalated_cooldown_key`]) so the transition is announced on
///   the next tick instead of waiting out the primary window. An
///   escalated table does NOT also get the `:warning:` digest — one
///   voice per table per tick.
/// - Fires the paired all-clear for any table that HAS a cooldown row but
///   no longer has stale rows (see
///   [`check_level_drift_recovery_and_notify`]) — the alert used to be
///   fire-and-forget, so an operator who fixed the lag got silence and a
///   recurrence inside the 24h window was silent too.
/// - Best-effort: a failed PG query or Slack POST only logs a warning,
///   and a failed POST leaves the cooldown UNSET so the next tick retries.
async fn check_level_drift_and_alert(pg_pool: &PgPool, slack: Option<&SlackClient>, site_id: &str) {
    let thresholds = level_drift_thresholds_from_env(site_id);
    let stale_hours = thresholds.stale_hours;

    // `stale_hours` is an i64 validated `> 0` by `parse_threshold_env`,
    // so the interpolation cannot carry operator input into the SQL.
    let rows = sqlx::query_as::<_, (String, i64, i64)>(sqlx::AssertSqlSafe(format!(
        "SELECT table_name, count(*), \
                floor(extract(epoch from (now() - min(detected_at))) / 3600)::bigint \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
            AND detected_at < now() - interval '{stale_hours} hours' \
          GROUP BY table_name"
    )))
    .fetch_all(pg_pool)
    .await;

    let counts: Vec<StaleTable> = match rows {
        Ok(r) => r
            .into_iter()
            .map(|(table, count, oldest_age_hours)| StaleTable {
                table,
                count,
                oldest_age_hours,
            })
            .collect(),
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to query ht_reconcile_log for level-triggered drift digest"
            );
            return;
        }
    };

    // Paired recovery notification — MUST run before the `counts.is_empty()`
    // early return below, because "no table has stale rows any more" is
    // exactly the everything-recovered case an operator needs to hear about.
    let still_stale_tables: Vec<String> = counts.iter().map(|r| r.table.clone()).collect();
    check_level_drift_recovery_and_notify(
        pg_pool,
        slack,
        site_id,
        &still_stale_tables,
        thresholds,
    )
    .await;

    if counts.is_empty() {
        tracing::debug!(
            site = %site_id,
            stale_hours,
            "[Sync] Level drift digest: no tables with unresolved rows past the stale interval"
        );
        return;
    }

    let (stale_tier, escalated_tier) = partition_level_drift(&counts, thresholds.escalate_hours);

    // Escalated tier first: it rides its own cooldown key, so a table
    // crossing the threshold is announced even if its primary 24h window
    // is still open.
    send_escalated_level_digest(pg_pool, slack, site_id, &escalated_tier, thresholds).await;
    send_stale_level_digest(pg_pool, slack, site_id, &stale_tier, thresholds).await;
}

/// Slack body for the routine `:warning:` level-drift digest. Pure —
/// unit-testable without a PG pool. Routine tier (issue #261): stays on
/// `with_site_text`, never the pager mention — this is the alert the
/// escalated `:bangbang:` tier exists precisely to distinguish itself
/// from.
fn format_stale_level_digest_message(
    stale_hours: i64,
    body: &str,
    cooldown_hours: i64,
    escalate_hours: i64,
) -> String {
    format!(
        ":warning: *Reconcile rows unconverged >{stale_hours}h* :warning:\n\
         `ht_reconcile_log` row(s) the auto-resolve sweep has not closed in \
         over {stale_hours} hours. This is NOT sync lag — \
         past this threshold it will not clear on its own:\n\
         {body}\n\
         _Check `divergence_kind` first. `missing_pg` with a live legacy row is a \
         *dropped legacy change*: the record is absent from our app entirely and \
         no tick will fix it. Do NOT blanket-set `resolved_at` — that closes rows \
         whether or not canonical landed. Triage: docs/runbook-sync.md §9b. \
         Per-table cooldown {cooldown_hours}h; an all-clear fires \
         when the table clears. Past {escalate_hours}h this escalates._"
    )
}

/// The familiar `:warning:` tier of [`check_level_drift_and_alert`].
/// Cooldown-gated per table on the bare entity name (the key the
/// all-clear diffs against).
async fn send_stale_level_digest(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
    tables: &[StaleTable],
    thresholds: LevelDriftThresholds,
) {
    let stale_hours = thresholds.stale_hours;
    let cooldown_hours = thresholds.cooldown_hours;
    let escalate_hours = thresholds.escalate_hours;
    let cooldown = thresholds.cooldown();

    let mut to_alert: Vec<&StaleTable> = Vec::new();
    for row in tables {
        if level_alert_eligible_pg(pg_pool, site_id, &row.table, cooldown).await {
            to_alert.push(row);
        } else {
            tracing::debug!(
                site = %site_id,
                table = %row.table,
                count = row.count,
                oldest_age_hours = row.oldest_age_hours,
                "[Sync] Level drift alert suppressed by cooldown"
            );
        }
    }

    if to_alert.is_empty() {
        return;
    }

    for row in &to_alert {
        tracing::warn!(
            site = %site_id,
            table = %row.table,
            count = row.count,
            oldest_age_hours = row.oldest_age_hours,
            stale_hours,
            "[Sync] Level drift alert: table has unresolved divergence older than threshold"
        );
    }

    let delivery = if let Some(slack) = slack {
        let body = to_alert
            .iter()
            .map(|r| {
                format!(
                    "• `{}`: {} unresolved row(s), oldest {}",
                    r.table,
                    r.count,
                    humanize_hours(r.oldest_age_hours)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let msg = SlackMessage::with_site_text(
            site_id,
            format_stale_level_digest_message(stale_hours, &body, cooldown_hours, escalate_hours),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; level drift digest logged only ({} table(s))",
            to_alert.len()
        );
        AlertDelivery::LoggedOnly
    };

    // Defect A3 — mark AFTER a confirmed delivery. Marking first meant a
    // webhook outage silenced the table for a full cooldown window and
    // let the all-clear later close an alert nobody ever saw.
    if cooldown_should_be_marked(delivery) {
        for row in &to_alert {
            mark_level_alert_sent_pg(pg_pool, site_id, &row.table).await;
        }
    } else {
        tracing::warn!(
            site = %site_id,
            tables = to_alert.len(),
            "[Sync] Level drift digest POST failed — leaving cooldown unset so the \
             next tick retries"
        );
    }
}

/// Slack body for the escalated `:bangbang:` digest. Pure — pulled out
/// of [`send_escalated_level_digest`] so it's unit-testable without a PG
/// pool. Pager tier (issue #261, re-scoped 2026-07-29): the caller wraps
/// this in [`SlackMessage::with_site_text_paged`], the same condition
/// that picks this `:bangbang:` framing over the `:warning:` digest.
fn format_escalated_level_digest_message(escalate_hours: i64, body: &str) -> String {
    format!(
        ":bangbang: *Reconcile rows STUCK >{escalate_hours}h — will not self-heal* \
         :bangbang:\n\
         These `ht_reconcile_log` row(s) have survived every auto-resolve sweep \
         for more than {escalate_hours} hours (a sweep runs every reconcile \
         tick). Waiting is no longer a strategy — nothing in the pipeline is \
         going to close them:\n\
         {body}\n\
         _The fix is re-ingest, not patience: for `missing_pg` re-drive the \
         record through the CT path or run `sync --bootstrap` for the table; \
         for `value` divergence re-apply from legacy. If the legacy change is \
         past the 2-day CT retention window, bootstrap is the ONLY path. Do NOT \
         blanket-set `resolved_at` — that hides the gap without landing the \
         data. Triage: docs/runbook-sync.md §9b._"
    )
}

/// The escalated tier of [`check_level_drift_and_alert`] (defect A1).
///
/// Fires for tables whose oldest unresolved row has passed
/// `LEVEL_DRIFT_ESCALATE_HOURS`. Two things change versus the
/// `:warning:` digest: the copy stops implying the sweep might still get
/// there, and the cooldown lives under [`escalated_cooldown_key`] so
/// crossing the threshold is not swallowed by a primary window that was
/// refreshed hours earlier.
///
/// On a successful send this marks BOTH the escalation key and the bare
/// table key. The bare key is what [`tables_recovered`] diffs against —
/// a table that escalated on its very first digest (e.g. after a long
/// worker outage) would otherwise never have a primary cooldown row and
/// so would never get an all-clear.
async fn send_escalated_level_digest(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
    tables: &[StaleTable],
    thresholds: LevelDriftThresholds,
) {
    if tables.is_empty() {
        return;
    }

    let escalate_hours = thresholds.escalate_hours;
    let cooldown = thresholds.cooldown();

    let mut to_alert: Vec<&StaleTable> = Vec::new();
    for row in tables {
        let key = escalated_cooldown_key(&row.table);
        if level_alert_eligible_pg(pg_pool, site_id, &key, cooldown).await {
            to_alert.push(row);
        } else {
            tracing::debug!(
                site = %site_id,
                table = %row.table,
                count = row.count,
                oldest_age_hours = row.oldest_age_hours,
                "[Sync] Escalated level drift alert suppressed by cooldown"
            );
        }
    }

    if to_alert.is_empty() {
        return;
    }

    for row in &to_alert {
        tracing::error!(
            site = %site_id,
            table = %row.table,
            count = row.count,
            oldest_age_hours = row.oldest_age_hours,
            escalate_hours,
            "[Sync] Level drift ESCALATED: unresolved divergence will not self-heal"
        );
    }

    let delivery = if let Some(slack) = slack {
        let body = to_alert
            .iter()
            .map(|r| {
                format!(
                    "• `{}`: {} unresolved row(s), oldest *{}*",
                    r.table,
                    r.count,
                    humanize_hours(r.oldest_age_hours)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let msg = SlackMessage::with_site_text_paged(
            site_id,
            format_escalated_level_digest_message(escalate_hours, &body),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; escalated level drift digest logged only ({} table(s))",
            to_alert.len()
        );
        AlertDelivery::LoggedOnly
    };

    if cooldown_should_be_marked(delivery) {
        for row in &to_alert {
            mark_level_alert_sent_pg(pg_pool, site_id, &escalated_cooldown_key(&row.table)).await;
            // Keep the all-clear reachable — see the fn docstring.
            mark_level_alert_sent_pg(pg_pool, site_id, &row.table).await;
        }
    } else {
        tracing::warn!(
            site = %site_id,
            tables = to_alert.len(),
            "[Sync] Escalated level drift digest POST failed — leaving cooldown unset \
             so the next tick retries"
        );
    }
}

/// Default days-past-expected-checkout before an `active` check-in is
/// flagged stale. Overridable via `STALE_CHECKIN_ALERT_DAYS`.
const STALE_CHECKIN_ALERT_DAYS_DEFAULT: i32 = 2;

/// Cooldown key (in `ht_level_drift_alert_cooldowns`, the `table_name`
/// column) for the stale-checkin tripwire — reuses the level-drift cooldown
/// table so the alert fires at most once per `LEVEL_DRIFT_COOLDOWN_HOURS`
/// (default 24h) per site, even while a backlog persists.
///
/// A bare sentinel, not namespaced with [`COOLDOWN_KEY_NAMESPACE_SEP`],
/// because it predates the namespace and is already enumerated in
/// [`NON_RECONCILE_COOLDOWN_KEYS`]. Renaming it would strand every live
/// cooldown row and re-fire the alert on both sites once.
const STALE_CHECKIN_COOLDOWN_KEY: &str = "stale_active_checkin";

/// Resolve the stale-checkin threshold (days) from `STALE_CHECKIN_ALERT_DAYS`,
/// clamped to a sane floor of 1 day. Falls back to the default on a missing
/// or unparseable value.
fn stale_checkin_alert_days() -> i32 {
    std::env::var("STALE_CHECKIN_ALERT_DAYS")
        .ok()
        .and_then(|v| v.trim().parse::<i32>().ok())
        .filter(|d| *d >= 1)
        .unwrap_or(STALE_CHECKIN_ALERT_DAYS_DEFAULT)
}

/// P0 stale-checkin tripwire (task #66). A pure-PG safety net, INDEPENDENT
/// of the MSSQL-backed reconcile sweep: a dropped checkout leaves the
/// canonical check-in `active` with `cin_expected_checkout` in the past
/// indefinitely (the CT event that would flip it is gone after retention,
/// and the reconcile hash historically didn't cover the per-room checkout
/// flip — the 2026-06-28 room-114 / cin 19906 incident, see
/// [`checkin_canonical_hash`]). This catches that class on ANY site, even
/// one whose reconcile sweep doesn't run or whose legacy MSSQL is
/// unreachable — because it only reads canonical PG.
///
/// Flags `cin_status='active'` rows whose `cin_expected_checkout` is more
/// than `STALE_CHECKIN_ALERT_DAYS` (default 2) days in the past, fires ONE
/// Slack alert per site gated by the shared 24h level-drift cooldown, and is
/// best-effort throughout (a PG or Slack failure only logs a warning).
///
/// **Paired all-clear** (2026-07-28 alert inventory, defect C8): this was
/// the one actionable alert in the channel with no closure signal — it
/// is deliberately excluded from the reconcile sweep's all-clear via
/// [`NON_RECONCILE_COOLDOWN_KEYS`], and nothing else told the operator
/// their manual reconcile had taken. It now owns its closure: when the
/// query comes back empty and a [`STALE_CHECKIN_COOLDOWN_KEY`] cooldown
/// row exists, emit `:white_check_mark:` and drop the row.
///
/// That is the right half of the "give it one / say it has none" choice
/// because recovery here is *directly observable from the same pure-PG
/// query that raises the alert* — an empty result set IS the recovered
/// state, no MSSQL round-trip, no hash comparison, no ambiguity. The
/// alternative (documenting "no all-clear will come, check PG yourself")
/// would write down a gap we can close in a dozen lines, on precisely
/// the alert whose remedy is a hand-edited row an operator most needs
/// confirmed. It also matches the three existing recovery-notification
/// precedents in this codebase.
pub async fn check_stale_active_checkins_and_alert(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) {
    let days = stale_checkin_alert_days();

    let rows = sqlx::query_as::<_, (String, chrono::DateTime<chrono::Utc>)>(
        "SELECT cin_no, cin_expected_checkout \
           FROM ht_checkins \
          WHERE cin_status = 'active' \
            AND cin_expected_checkout IS NOT NULL \
            AND cin_expected_checkout < now() - make_interval(days => $1) \
          ORDER BY cin_expected_checkout \
          LIMIT 100",
    )
    .bind(days)
    .fetch_all(pg_pool)
    .await;

    let stale = match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to query ht_checkins for stale-checkin tripwire — observability degraded"
            );
            return;
        }
    };

    let cooldown_hours = level_drift_thresholds_from_env(site_id).cooldown_hours;
    let cooldown = std::time::Duration::from_secs((cooldown_hours * 3600) as u64);

    if stale.is_empty() {
        tracing::debug!(
            site = %site_id,
            threshold_days = days,
            "[Sync] Stale-checkin tripwire: no active check-ins past expected checkout"
        );
        // Defect C8 — paired all-clear. Fires only if we actually alerted
        // at some point (a cooldown row exists), so a site that has never
        // had a stale check-in stays silent forever.
        notify_stale_checkin_all_clear(pg_pool, slack, site_id, days).await;
        return;
    }

    tracing::warn!(
        site = %site_id,
        count = stale.len(),
        threshold_days = days,
        "[Sync] Stale-checkin tripwire: active check-ins long past expected checkout (likely dropped checkout)"
    );

    // Cooldown-gate the Slack alert (reuse the level-drift cooldown table so
    // a persistent backlog doesn't refire every tick).
    if !level_alert_eligible_pg(pg_pool, site_id, STALE_CHECKIN_COOLDOWN_KEY, cooldown).await {
        tracing::debug!(
            site = %site_id,
            "[Sync] Stale-checkin alert suppressed by cooldown"
        );
        return;
    }

    let delivery = if let Some(slack) = slack {
        let now = chrono::Utc::now();
        let shown = stale.len().min(15);
        let body = stale
            .iter()
            .take(shown)
            .map(|(cin_no, exp)| {
                let overdue_days = (now - *exp).num_days();
                format!("• `{cin_no}` — {overdue_days}d past expected checkout")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let more = if stale.len() > shown {
            format!("\n…and {} more", stale.len() - shown)
        } else {
            String::new()
        };

        let msg = SlackMessage::with_site_text(
            site_id,
            format!(
                ":hourglass_flowing_sand: *Stale active check-in(s) — likely dropped checkout* :hourglass_flowing_sand:\n\
                 {count} canonical check-in(s) are still `active` more than {days} day(s) past their \
                 expected checkout. This usually means a checkout CT event was dropped (past MSSQL \
                 retention, so it won't self-heal) — the room shows occupied in the new app while \
                 iHOTEL has it checked out:\n\
                 {body}{more}\n\
                 _Reconcile the row(s) to match iHOTEL (see the 2026-06-28 cin 19906 / room 114 \
                 playbook). Pure-PG tripwire; per-site cooldown {cooldown_h}h. A \
                 `:white_check_mark:` all-clear fires once no check-in is past threshold._",
                count = stale.len(),
                cooldown_h = cooldown_hours,
            ),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; stale-checkin tripwire logged only ({} row(s))",
            stale.len()
        );
        AlertDelivery::LoggedOnly
    };

    // Defect A3 — mark only on a confirmed delivery, so a webhook outage
    // doesn't silence a dropped-checkout backlog for a full day.
    if cooldown_should_be_marked(delivery) {
        mark_level_alert_sent_pg(pg_pool, site_id, STALE_CHECKIN_COOLDOWN_KEY).await;
    } else {
        tracing::warn!(
            site = %site_id,
            "[Sync] Stale-checkin alert POST failed — leaving cooldown unset so the \
             next tick retries"
        );
    }
}

/// Paired all-clear for [`check_stale_active_checkins_and_alert`]
/// (defect C8). Called on the tick where the tripwire query comes back
/// empty; emits nothing unless a cooldown row proves we alerted earlier.
///
/// Clearing the cooldown row is what erases the "we alerted" record, so
/// — as in [`check_level_drift_recovery_and_notify`] — it happens only
/// after the closure has actually been delivered.
async fn notify_stale_checkin_all_clear(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
    days: i32,
) {
    if !cooldown_row_exists_pg(pg_pool, site_id, STALE_CHECKIN_COOLDOWN_KEY).await {
        return;
    }

    tracing::info!(
        site = %site_id,
        threshold_days = days,
        "[Sync] Stale-checkin all-clear: no active check-ins past expected checkout — \
         clearing tripwire cooldown"
    );

    let delivery = if let Some(slack) = slack {
        let msg = SlackMessage::with_site_text(
            site_id,
            format!(
                ":white_check_mark: *Stale active check-in(s) CLEARED* :white_check_mark:\n\
                 No canonical check-in is `active` more than {days} day(s) past its \
                 expected checkout any more.\n\
                 _Closure of the_ `:hourglass_flowing_sand:` _dropped-checkout alert sent \
                 earlier. The per-site cooldown is reset, so a recurrence alerts on the \
                 next tick instead of waiting out a stale window._"
            ),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; stale-checkin all-clear logged only"
        );
        AlertDelivery::LoggedOnly
    };

    if cooldown_should_be_marked(delivery) {
        clear_level_alert_cooldown_pg(pg_pool, site_id, STALE_CHECKIN_COOLDOWN_KEY).await;
    } else {
        tracing::warn!(
            site = %site_id,
            "[Sync] Stale-checkin all-clear POST failed — keeping the cooldown row so \
             the next tick retries the closure"
        );
    }
}

/// Resolved CT-lag thresholds (versions + seconds) for a reconcile tick.
/// Produced by [`ct_lag_thresholds_from_env`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CtLagThresholds {
    /// `current_version - last_seen_version` greater than this triggers
    /// a warn-level log instead of debug.
    version_lag: i64,
    /// `now() - last_polled_at` (in seconds) greater than this triggers
    /// a warn-level log instead of debug.
    poll_age_seconds: i64,
}

/// Resolve the per-tick CT lag thresholds from env. Mirrors the per-site
/// override pattern used by [`drift_alert_threshold_from_env`]:
/// `LEGACY_CT_LAG_WARN_VERSIONS_<SITE_ID_UPPER>` and
/// `LEGACY_CT_LAG_WARN_SECONDS_<SITE_ID_UPPER>` take precedence over the
/// global `LEGACY_CT_LAG_WARN_VERSIONS` / `LEGACY_CT_LAG_WARN_SECONDS`,
/// which in turn fall back to the compiled-in defaults
/// ([`DEFAULT_CT_LAG_WARN_VERSIONS`] / [`DEFAULT_CT_LAG_WARN_SECONDS`]).
fn ct_lag_thresholds_from_env(site_id: &str) -> CtLagThresholds {
    let site_upper = site_id.to_uppercase();
    let per_site_version = format!("LEGACY_CT_LAG_WARN_VERSIONS_{site_upper}");
    let per_site_seconds = format!("LEGACY_CT_LAG_WARN_SECONDS_{site_upper}");
    let version_lag = parse_threshold_env(&per_site_version)
        .or_else(|| parse_threshold_env("LEGACY_CT_LAG_WARN_VERSIONS"))
        .unwrap_or(DEFAULT_CT_LAG_WARN_VERSIONS);
    let poll_age_seconds = parse_threshold_env(&per_site_seconds)
        .or_else(|| parse_threshold_env("LEGACY_CT_LAG_WARN_SECONDS"))
        .unwrap_or(DEFAULT_CT_LAG_WARN_SECONDS);
    CtLagThresholds {
        version_lag,
        poll_age_seconds,
    }
}

/// Pure decision function for [`check_ct_watcher_lag`]. Returns `true`
/// when the observed lag breaches either threshold (strictly greater
/// than) — i.e. the operator should see a `warn` log instead of `debug`.
///
/// Inputs are kept explicit (no env reads, no clock) so the unit tests
/// can drive the truth table without spinning a tokio runtime or mocking
/// PG/MSSQL.
fn ct_lag_is_warning(version_lag: i64, poll_age_seconds: i64, thresholds: CtLagThresholds) -> bool {
    version_lag > thresholds.version_lag || poll_age_seconds > thresholds.poll_age_seconds
}

/// Snapshot of the PG-side CT watermark for the per-tick health check.
#[derive(Debug, Clone, Copy)]
struct PgCtWatermark {
    last_seen_version: i64,
    last_polled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Read `legacy_ct_state.last_seen_version` + `last_polled_at` from PG.
/// Single-row table (`WHERE id = 1`). A missing row (pre-bootstrap) is
/// surfaced as a `sqlx::Error::RowNotFound` for the caller to log and
/// skip — the watcher hasn't started yet so there's nothing to compare.
async fn read_pg_ct_watermark(pg_pool: &PgPool) -> Result<PgCtWatermark, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT last_seen_version, last_polled_at FROM legacy_ct_state WHERE id = 1",
    )
    .fetch_one(pg_pool)
    .await?;
    Ok(PgCtWatermark {
        last_seen_version: row.0,
        last_polled_at: row.1,
    })
}

/// Resilience PR R3 — is the CT watcher holding per-table watermarks?
///
/// Mirrors the exact idiom `bin/sync.rs` uses to read the same flag
/// (strict `== "true"`, default OFF) so the reconcile-tick health check
/// and the watcher binary can never disagree about which watermark table
/// is authoritative. When this is on, the single-row `legacy_ct_state`
/// stops advancing and `legacy_ct_state_per_table` carries the real
/// progress — see `crate::sync::watermark` for the dual-mode contract.
fn per_table_watermark_enabled() -> bool {
    env::var("SYNC_PER_TABLE_WATERMARK")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// One row of `legacy_ct_state_per_table`, as read by the per-tick CT
/// health check in per-table-watermark mode.
#[derive(Debug, Clone)]
struct PerTableWatermark {
    table_name: String,
    last_seen_version: i64,
    last_polled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Read every `legacy_ct_state_per_table` row. An empty result means the
/// per-table watermarks haven't been seeded yet (pre-migration-050 or
/// pre-bootstrap) — the caller logs and skips, exactly like the global
/// path's `RowNotFound` branch.
async fn read_pg_ct_watermarks_per_table(
    pg_pool: &PgPool,
) -> Result<Vec<PerTableWatermark>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, i64, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT table_name, last_seen_version, last_polled_at \
           FROM legacy_ct_state_per_table",
    )
    .fetch_all(pg_pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(
            |(table_name, last_seen_version, last_polled_at)| PerTableWatermark {
                table_name,
                last_seen_version,
                last_polled_at,
            },
        )
        .collect())
}

/// Pure picker for the per-table CT health check: given every per-table
/// watermark row, the current legacy CT version, and `now`, return the
/// STALEST table together with its `(version_lag, poll_age_seconds)`.
///
/// Ranking, in order: largest `version_lag` (i.e. smallest
/// `last_seen_version`) first, then largest poll age (i.e. oldest
/// `last_polled_at`), then `table_name` so the pick is deterministic when
/// every table is equally healthy. A `NULL` `last_polled_at` is treated as
/// infinitely old (`i64::MAX`), matching the global path's never-polled
/// handling.
///
/// Returns `None` only for an empty input (per-table watermarks not seeded
/// yet). Kept free of PG / env / clock so the unit tests can drive the
/// ranking directly.
fn stalest_per_table_watermark(
    rows: &[PerTableWatermark],
    current_version: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<(&PerTableWatermark, i64, i64)> {
    rows.iter()
        .map(|w| {
            // A watermark AHEAD of `current_version` is a CT anomaly (the
            // version is monotonic). `saturating_sub` alone does not help
            // here — on a signed `i64` it saturates at `i64::MIN`, not at
            // zero — so clamp explicitly. Without the clamp the log line
            // would report a nonsensical negative lag. (The global arm keeps
            // its pre-existing unclamped form so its behaviour stays
            // byte-identical; it is unreachable in practice for the same
            // monotonicity reason.)
            let version_lag = current_version.saturating_sub(w.last_seen_version).max(0);
            let poll_age_seconds = w
                .last_polled_at
                .map(|polled| now.signed_duration_since(polled).num_seconds().max(0))
                .unwrap_or(i64::MAX);
            (w, version_lag, poll_age_seconds)
        })
        .max_by(|a, b| {
            a.1.cmp(&b.1)
                .then_with(|| a.2.cmp(&b.2))
                // `max_by` keeps the LAST maximum, so invert the name
                // comparison to land on the alphabetically-first table.
                .then_with(|| b.0.table_name.cmp(&a.0.table_name))
        })
}

/// Read `SELECT CHANGE_TRACKING_CURRENT_VERSION()` from legacy MSSQL.
/// Returns `Ok(None)` when CT is not enabled on the database (the
/// function returns NULL in that case); returns `Err` for connection /
/// query failures. The caller logs both as a warning and skips the
/// comparison — neither is fatal to the reconcile tick.
async fn read_mssql_ct_current_version(
    legacy_pool: &DbPool,
) -> Result<Option<i64>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = "SELECT CHANGE_TRACKING_CURRENT_VERSION() AS v";
    let rows =
        query_with_timeout_pooled(&mut conn, sql, Query::new(sql), MssqlOpKind::Read).await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    Ok(row.get::<i64, _>("v"))
}

/// Incident 2026-05-18 follow-up — per-reconcile-tick CT watcher health
/// observation. Compares the PG-side `legacy_ct_state.last_seen_version`
/// against MSSQL's `CHANGE_TRACKING_CURRENT_VERSION()` and emits a
/// `warn`-level log line if either the version lag or the poll-age
/// exceeds the configured thresholds.
///
/// Log-only by design — no Slack alert. The CT-watcher binary
/// (`bin/sync.rs`) already pages on stuck watermarks via the
/// `watermark_stall_alert_eligible` path; this reconcile-tick check is
/// the SECOND line of defence for the case where the watcher process
/// itself is silent (no logs, no alerts) yet falling behind. Operators
/// grep the backend logs for `[Sync] CT watcher lag detected` to find
/// it. If repeated warns become a pattern we can layer a Slack alert
/// on top later.
///
/// Best-effort: a failed MSSQL or PG query logs a warning and returns
/// without aborting the reconcile loop. Same posture as
/// [`check_drift_and_alert`] / [`check_level_drift_and_alert`] —
/// degraded observability must never take down the rest of the tick.
///
/// **Per-table watermark mode (R3).** `SYNC_PER_TABLE_WATERMARK` selects
/// which watermark table is authoritative. With the flag OFF (default)
/// this reads the global `legacy_ct_state WHERE id = 1` row — behaviour
/// unchanged. With the flag ON the global row stops advancing (it freezes
/// at its bootstrap value), so reading it would emit a permanently-stuck
/// `version_lag` warning every tick and mask the real per-table lag;
/// instead we read every `legacy_ct_state_per_table` row and report the
/// STALEST table by name (see [`stalest_per_table_watermark`]). The
/// sibling gap in `bin/sync.rs::watermark_stall_alert_eligible` lives in
/// the CT-watcher binary and is tracked with that file.
async fn check_ct_watcher_lag(legacy_pool: &DbPool, pg_pool: &PgPool, site_id: &str) {
    if per_table_watermark_enabled() {
        check_ct_watcher_lag_per_table(legacy_pool, pg_pool, site_id).await;
    } else {
        check_ct_watcher_lag_global(legacy_pool, pg_pool, site_id).await;
    }
}

/// Global-watermark arm of [`check_ct_watcher_lag`] — the default path,
/// byte-identical to the pre-R3 behaviour.
async fn check_ct_watcher_lag_global(legacy_pool: &DbPool, pg_pool: &PgPool, site_id: &str) {
    let watermark = match read_pg_ct_watermark(pg_pool).await {
        Ok(w) => w,
        Err(sqlx::Error::RowNotFound) => {
            tracing::debug!(
                site = %site_id,
                "[Sync] CT watcher health: legacy_ct_state has no row yet — \
                 watcher pre-bootstrap, skipping lag check"
            );
            return;
        }
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to read legacy_ct_state — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    let current_version = match read_mssql_ct_current_version(legacy_pool).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            tracing::warn!(
                site = %site_id,
                "[Sync] CT watcher health: CHANGE_TRACKING_CURRENT_VERSION() \
                 returned NULL — CT not enabled on legacy DB?"
            );
            return;
        }
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to probe \
                 CHANGE_TRACKING_CURRENT_VERSION() on legacy MSSQL — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    let last_seen_version = watermark.last_seen_version;
    // Saturating sub: a watermark ahead of `current_version` is a CT
    // anomaly (the version is monotonic) but a defensive zero-lag here
    // is better than a panic on underflow.
    let version_lag = current_version.saturating_sub(last_seen_version);
    let poll_age_seconds = watermark
        .last_polled_at
        .map(|polled| {
            chrono::Utc::now()
                .signed_duration_since(polled)
                .num_seconds()
                .max(0)
        })
        // No `last_polled_at` row yet → watcher hasn't ticked. Treat as
        // "infinitely old" (i64::MAX) so the warn branch fires and the
        // operator sees the never-polled state in the logs.
        .unwrap_or(i64::MAX);

    let thresholds = ct_lag_thresholds_from_env(site_id);
    if ct_lag_is_warning(version_lag, poll_age_seconds, thresholds) {
        tracing::warn!(
            site = %site_id,
            current_version,
            last_seen_version,
            version_lag,
            poll_age_seconds,
            version_threshold = thresholds.version_lag,
            seconds_threshold = thresholds.poll_age_seconds,
            "[Sync] CT watcher lag detected"
        );
    } else {
        tracing::debug!(
            site = %site_id,
            current_version,
            last_seen_version,
            version_lag,
            poll_age_seconds,
            "[Sync] CT watcher healthy"
        );
    }
}

/// Per-table-watermark arm of [`check_ct_watcher_lag`]. Reads every
/// `legacy_ct_state_per_table` row, ranks them with
/// [`stalest_per_table_watermark`], and reports the worst one WITH ITS
/// TABLE NAME so an operator can see which table is behind rather than a
/// meaningless global figure. Same warn/debug message text as the global
/// arm (plus a `table` field) so existing log greps keep working.
async fn check_ct_watcher_lag_per_table(legacy_pool: &DbPool, pg_pool: &PgPool, site_id: &str) {
    let watermarks = match read_pg_ct_watermarks_per_table(pg_pool).await {
        Ok(w) => w,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to read legacy_ct_state_per_table — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    if watermarks.is_empty() {
        tracing::debug!(
            site = %site_id,
            "[Sync] CT watcher health: legacy_ct_state_per_table has no rows yet — \
             watcher pre-bootstrap, skipping lag check"
        );
        return;
    }

    let current_version = match read_mssql_ct_current_version(legacy_pool).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            tracing::warn!(
                site = %site_id,
                "[Sync] CT watcher health: CHANGE_TRACKING_CURRENT_VERSION() \
                 returned NULL — CT not enabled on legacy DB?"
            );
            return;
        }
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to probe \
                 CHANGE_TRACKING_CURRENT_VERSION() on legacy MSSQL — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    let Some((stalest, version_lag, poll_age_seconds)) =
        stalest_per_table_watermark(&watermarks, current_version, chrono::Utc::now())
    else {
        // Unreachable — `watermarks` is non-empty by the guard above.
        return;
    };

    let last_seen_version = stalest.last_seen_version;
    let thresholds = ct_lag_thresholds_from_env(site_id);
    if ct_lag_is_warning(version_lag, poll_age_seconds, thresholds) {
        tracing::warn!(
            site = %site_id,
            table = %stalest.table_name,
            tables_tracked = watermarks.len(),
            current_version,
            last_seen_version,
            version_lag,
            poll_age_seconds,
            version_threshold = thresholds.version_lag,
            seconds_threshold = thresholds.poll_age_seconds,
            "[Sync] CT watcher lag detected"
        );
    } else {
        tracing::debug!(
            site = %site_id,
            table = %stalest.table_name,
            tables_tracked = watermarks.len(),
            current_version,
            last_seen_version,
            version_lag,
            poll_age_seconds,
            "[Sync] CT watcher healthy"
        );
    }
}

/// Compute SHA256 hash of a string
pub(crate) fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// =============================================================================
// Multi-row PK aggregation (Phase 6 hotfix 2026-04-29)
// =============================================================================
//
// `View_CheckIn_Ds` and `View_Booking_Ds` both project multiple rows
// per logical PK (one per booked detail/room). Hashing each row
// independently against the single-row-per-PK `ht_*_legacy` cache
// caused the reconcile job to flag the same PKs as drifted on every
// tick (~22-24k/hour Slack spam observed 2026-04-29).
//
// Fix: aggregate rows by PK into a deterministic group, sort the group
// by stable discriminating fields, and hash a deterministic
// concatenation of the entire group. One hash per PK, one
// `record_divergence` per PK, one cache UPDATE per PK.
//
// Helpers below are pure (no PG, no env, no time) so they're
// unit-testable in `mod tests`.

/// One detail-row of a booking. `View_Booking_Ds` returns up to 3 of
/// these per composite PK `(Book_No, Book_Room_Type)`.
#[derive(Debug, Clone)]
struct BookingDetail {
    book_date: Option<NaiveDateTime>,
    book_date_in: Option<NaiveDateTime>,
    book_date_out: Option<NaiveDateTime>,
    book_cust_name: Option<String>,
    book_cust_id: Option<String>,
    book_status: Option<i32>,
    book_room_type: Option<String>,
}

/// Render an `Option<NaiveDateTime>` deterministically. Empty string
/// for `None` so it's distinguishable from the `Debug` `"None"` literal
/// only by absence — both encodings are stable, but using a fixed
/// sentinel avoids the `Some(...)` wrapper noise and matches what the
/// JSON projection emits.
fn fmt_dt(dt: &Option<NaiveDateTime>) -> String {
    dt.map(|d| d.to_string()).unwrap_or_default()
}

/// Render an `Option<String>` deterministically.
fn fmt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// Aggregate all detail rows for one `(Book_No, Book_Room_Type)` PK
/// into a single deterministic SHA256 hash. Sorts `details` in place
/// by `(book_date, book_date_in, book_date_out, book_cust_name,
/// book_cust_id, book_status)` — every non-key field — so the hash is
/// independent of the order MSSQL returned the rows.
///
/// **Retired in v2.63.0** — the multi-row aggregate hash is no longer
/// used by either reconcile mode (the bookings sweep takes a single
/// representative row per composite PK). Kept for the unit tests in
/// `mod tests` which still pin the determinism contract, useful both
/// if the helper is ever resurrected and as a reference for the
/// post-v2.63.0 `sort_booking_details` extraction.
#[allow(dead_code)]
fn aggregate_booking_hash(
    book_no: &str,
    room_type_key: &str,
    details: &mut Vec<BookingDetail>,
) -> String {
    details.sort_by(|a, b| {
        (
            fmt_dt(&a.book_date),
            fmt_dt(&a.book_date_in),
            fmt_dt(&a.book_date_out),
            fmt_str(&a.book_cust_name),
            fmt_str(&a.book_cust_id),
            a.book_status.unwrap_or(i32::MIN),
        )
            .cmp(&(
                fmt_dt(&b.book_date),
                fmt_dt(&b.book_date_in),
                fmt_dt(&b.book_date_out),
                fmt_str(&b.book_cust_name),
                fmt_str(&b.book_cust_id),
                b.book_status.unwrap_or(i32::MIN),
            ))
    });

    let body = details
        .iter()
        .map(|d| {
            format!(
                "{}|{}|{}|{}|{}|{:?}|{}",
                fmt_dt(&d.book_date),
                fmt_dt(&d.book_date_in),
                fmt_dt(&d.book_date_out),
                fmt_str(&d.book_cust_name),
                fmt_str(&d.book_cust_id),
                d.book_status,
                fmt_str(&d.book_room_type),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    sha256(&format!("{book_no}|{room_type_key}|{body}"))
}

/// Count DISTINCT non-empty `Cin_Room_No` values across a check-in's
/// `HT_CheckIn_Ds` rows.
///
/// Why: `ht_checkin_rooms` carries a `UNIQUE (cr_cin_id, cr_room_id)`
/// constraint, so "distinct rooms in folio" IS the canonical truth.
/// Raw `aggregate.rooms.len()` overcounts whenever iHOTEL records
/// multiple HT_CheckIn_Ds detail rows for the same room (extends,
/// re-keys, deposit returns), which then surfaces as spurious
/// `cardinality` drift in `ht_reconcile_log`.
///
/// Concrete trigger (CH22-000722, 2026-05-19): 3 HT_CheckIn_Ds rows,
/// all `Cin_Room_No='417'`, same `Cin_cust_no`, only `Cin_Room_Out`
/// differs across rows. Raw len → 3, junction count → 1, classifier
/// → `cardinality` (false positive). With this helper: distinct → 1,
/// matches the junction.
///
/// Skip semantics mirror [`crate::sync::mappers::checkin::project_rooms`]
/// (file `mappers/checkin.rs`, around the `Cin_Room_No` guards) —
/// rows whose `Cin_Room_No` is NULL or empty are dropped from the
/// junction projection, so they must also be dropped here so the two
/// counts compare apples-to-apples.
///
/// Errors from `try_get_str` are swallowed (the row is treated as
/// having no room number) rather than bubbled. Rationale: this helper
/// runs inside the reconcile loop where a hard error on a single
/// malformed row would abort the entire tick. The mapper-side
/// projection (`project_rooms`) propagates the same errors loudly so
/// real schema regressions still surface; here, conservative
/// "skip-on-probe-failure" beats blocking the tick.
fn count_distinct_legacy_checkin_rooms(
    rooms: &[crate::sync::row::test_support::HashMapRow],
) -> i32 {
    use crate::sync::row::MappableRow;
    use std::collections::HashSet;

    let mut distinct: HashSet<String> = HashSet::new();
    for r in rooms {
        let Ok(Some(room_no)) = r.try_get_str("Cin_Room_No") else {
            continue;
        };
        if room_no.is_empty() {
            continue;
        }
        distinct.insert(room_no.to_string());
    }
    distinct.len() as i32
}

/// Build the `mssql_row_json` payload for a check-in PK aggregate.
/// Reads the legacy header (`HT_CheckIn_H`) once for the cross-row
/// fields (`Cin_Date_in`, `Cin_cust_no`, `Cin_status`) and pairs them
/// with each `HT_CheckIn_Ds` row's `Cin_Room_No` / `Cin_Room_Out` so
/// an operator triaging the divergence sees what changed across every
/// booked room — not just the first.
///
/// JSON shape is preserved across the 2026-05-18 unification refactor
/// (v2.63.x → mapper-backed) so dashboards that parse the column keys
/// continue to work. Every detail row repeats the header-derived
/// fields verbatim (same redundancy the per-row JOIN projection used
/// to produce), and `Cin_cust_name` is left `null` because the
/// column was always populated from a view-derived alias that the
/// parent loader does not project (kept under the same key for shape
/// stability — see the original `CheckinDetail.cust_name = None`
/// rationale in the prior projection's doc comment).
fn checkin_aggregate_json(
    cin_no: &str,
    aggregate: &crate::sync::parent_loader::CheckInAggregate,
) -> serde_json::Value {
    use crate::sync::row::MappableRow;

    let header_date_in = aggregate
        .header
        .as_ref()
        .and_then(|h| h.try_get_datetime("Cin_Date_in").ok().flatten())
        .map(|dt| dt.to_string());
    let header_cust_no = aggregate
        .header
        .as_ref()
        .and_then(|h| h.try_get_str("Cin_cust_no").ok().flatten())
        .map(str::to_string);
    let header_status = aggregate
        .header
        .as_ref()
        .and_then(|h| h.try_get_str("Cin_status").ok().flatten())
        .map(str::to_string);

    let rows: Vec<serde_json::Value> = aggregate
        .rooms
        .iter()
        .map(|r| {
            let room_no = r
                .try_get_str("Cin_Room_No")
                .ok()
                .flatten()
                .map(str::to_string);
            let room_out = r
                .try_get_datetime("Cin_Room_Out")
                .ok()
                .flatten()
                .map(|dt| dt.to_string());
            json!({
                "Cin_Room_No": room_no,
                // JSON key reflects the hashed source column
                // (`HT_CheckIn_H.Cin_Date_in`, not the per-room
                // `HT_CheckIn_Ds.Cin_Room_In`) so operators see what
                // actually fed the hash.
                "Cin_Date_in": header_date_in,
                "Cin_Room_Out": room_out,
                "Cin_cust_name": serde_json::Value::Null,
                "Cin_cust_no": header_cust_no,
                "Cin_status": header_status,
            })
        })
        .collect();
    json!({
        "Cin_no": cin_no,
        "details": rows,
    })
}

/// Build the `mssql_row_json` payload for a booking PK group. Includes
/// the full sorted details array.
fn booking_group_json(book_no: &str, details: &[BookingDetail]) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = details
        .iter()
        .map(|d| {
            json!({
                "Book_Date": d.book_date.map(|t| t.to_string()),
                "Book_Date_in": d.book_date_in.map(|t| t.to_string()),
                "Book_Date_out": d.book_date_out.map(|t| t.to_string()),
                "Book_Cust_Name": d.book_cust_name,
                "Book_Cust_ID": d.book_cust_id,
                "Book_Status": d.book_status,
                "Book_Room_Type": d.book_room_type,
            })
        })
        .collect();
    json!({
        "Book_No": book_no,
        "details": rows,
    })
}

// =============================================================================
// Canonical-projection helpers (v2.63.0)
// =============================================================================
//
// The DiffOnly hot path computes TWO hashes per PK and compares them:
//   * `mssql_hash`     — legacy row(s) projected into canonical shape
//   * `canonical_hash` — canonical `ht_*` row(s) projected into the same shape
//
// Both hashes use the SAME field set + SAME serialisation template, so they
// match iff the CT watcher's mapper has faithfully mirrored MSSQL into
// canonical. Drift = actionable mapper gap.
//
// Field set is deliberately narrowed to columns the CT mapper actually
// projects to canonical. Untracked legacy fields (e.g. `Room_Group`,
// `Room_Book_Time`, `Book_Cust_Name` denormalisations) are excluded from
// both hashes — including them would create systematic drift on rows
// where canonical has no corresponding storage, defeating the tripwire's
// signal-to-noise ratio.

/// Reverse of `sync::mappers::room::legacy_yesno_to_bool`. Project a
/// canonical `BOOLEAN` back into the legacy `'yes' | 'no' | ""` literal
/// for hashing. `None → ""` matches what the legacy MSSQL projection
/// emits for a NULL column via `.as_deref().unwrap_or("")`.
fn bool_to_yesno(b: Option<bool>) -> &'static str {
    match b {
        Some(true) => "yes",
        Some(false) => "no",
        None => "",
    }
}

/// Project canonical `room_clean` (true = IS clean) back to the legacy
/// `Room_Clean` NEEDS-CLEANING literal for drift hashing. Legacy semantics are
/// inverted from canonical: 'no' = clean, 'yes' = dirty (needs cleaning). MUST
/// match the CT room mapper's `new_clean` inversion or every checked-out /
/// cleaned room would hash as drifted. Maintenance/use keep `bool_to_yesno`
/// ('yes' = the named state).
fn clean_bool_to_legacy_yesno(b: Option<bool>) -> &'static str {
    match b {
        Some(true) => "no",   // canonical clean -> legacy "no cleaning needed"
        Some(false) => "yes", // canonical dirty -> legacy "needs cleaning"
        None => "",
    }
}

/// Map an MSSQL legacy `Room_Clean`/`Room_Manternace` literal to the
/// canonical-shape `'yes' | 'no' | ""` token. Mirrors the CT room
/// mapper's `legacy_yesno_to_bool`: anything other than the two known
/// literals collapses to `""` so the hashes line up.
fn legacy_yesno_canonical(s: Option<&str>) -> &'static str {
    match s {
        Some("yes") => "yes",
        Some("no") => "no",
        _ => "",
    }
}

/// Hash inputs for the canonical-shape customer projection. Single-row
/// per PK on both sides. Order + separator must stay byte-identical
/// between the MSSQL and PG paths or the hashes won't line up.
///
/// Segments are joined by [`join_hash_segments`] rather than a
/// `format!` template so the separator is single-sourced with the
/// mapper-side descriptor table (`sync::mappers::customer::HASH_INPUTS`)
/// that pins the gate ⊇ hash invariant. Byte-identical to the template
/// it replaced — pinned by
/// `customers_hash_bytes_unchanged_for_golden_inputs`.
pub(crate) fn customer_canonical_hash(
    legacy_cust_no: &str,
    cust_firstname: &str,
    cust_type: Option<&str>,
    cust_phone: Option<&str>,
    cust_idcard: Option<&str>,
    cust_address: Option<&str>,
) -> String {
    sha256(&join_hash_segments(&[
        legacy_cust_no.to_string(),
        cust_firstname.to_string(),
        cust_type.unwrap_or("").to_string(),
        cust_phone.unwrap_or("").to_string(),
        cust_idcard.unwrap_or("").to_string(),
        cust_address.unwrap_or("").to_string(),
    ]))
}

/// Hash inputs for the canonical-shape room projection. Narrowed to
/// fields the CT room mapper actually writes back (room_clean,
/// room_maintenance, room_notes) — prices and other legacy-only
/// columns are excluded because canonical doesn't mirror them.
pub(crate) fn room_canonical_hash(
    room_no: &str,
    room_clean_yesno: &str,
    room_maintenance_yesno: &str,
    room_notes: Option<&str>,
) -> String {
    sha256(&join_hash_segments(&[
        room_no.to_string(),
        room_clean_yesno.to_string(),
        room_maintenance_yesno.to_string(),
        room_notes.unwrap_or("").to_string(),
    ]))
}

/// Hash inputs for one canonical-shape booking row. Single-row per
/// `legacy_book_id` (canonical doesn't multi-row by `Book_Room_Type`).
///
/// `book_status` is deliberately excluded: legacy `View_Booking_Ds.Book_Status`
/// is an integer ledger code while canonical `ht_bookings.book_status` is a
/// translated English literal sourced from `HT_Book_H.Book_Status` — different
/// fields. Status changes are surfaced by the CT watcher's domain events.
pub(crate) fn booking_canonical_hash(
    legacy_book_id: &str,
    book_checkin_date: Option<&str>,
    book_checkout_date: Option<&str>,
    legacy_cust_no: Option<&str>,
) -> String {
    sha256(&join_hash_segments(&[
        legacy_book_id.to_string(),
        book_checkin_date.unwrap_or("").to_string(),
        book_checkout_date.unwrap_or("").to_string(),
        legacy_cust_no.unwrap_or("").to_string(),
    ]))
}

/// Hash inputs for one canonical-shape check-in row. The CT checkin
/// mapper denormalises only the FIRST room (`legacy_room_no`) into
/// `ht_checkins`, so we hash that single representative row here too.
///
/// Full `cin_status` is deliberately excluded from the active-stay hash
/// shape: `View_CheckIn_Ds.Cin_status` is a per-room ledger state,
/// whereas canonical `ht_checkins.cin_status` is the header-derived
/// aggregate — different fields. The `checked_out` boolean below is the
/// ONE status bit we DO hash (see next paragraph).
///
/// **Checked-out flag (task #68, 2026-06-28).** `cin_checkout_time` alone
/// does NOT distinguish active-from-checked-out, because the hash feeds it
/// the *effective* checkout date — `actual` when checked out, else
/// `expected` — and the two coincide whenever a guest departs on the
/// booked date. A dropped checkout then leaves canonical `active`
/// (effective = expected) while legacy is checked-out (effective = actual)
/// with IDENTICAL dates → identical hashes → invisible drift. That is
/// exactly the 2026-06-28 room-114 / cin 19906 incident (out 05-16 =
/// expected 05-16). The `checked_out` bit — `cin_checkout_time.is_some()`
/// on BOTH sides — makes the active↔checked-out transition a first-class
/// hash input without importing the non-comparable full `cin_status`.
/// Agreeing rows still match (both `true` or both `false`); only a genuine
/// status divergence (dropped checkout/checkin) flips it.
///
/// **Cancelled folios (2026-05-19 — reconcile cleanup PR B).** When
/// iHOTEL cancels a check-in (`HT_CheckIn_H.Cin_status='ยกเลิก'`) it
/// also deletes the per-room `HT_CheckIn_Ds` rows. The CT mapper's
/// `derive_room_state` honours that by emitting
/// `canonical_status='cancelled'` with `first_room_no=None`.
/// Canonical PG, however, retains the original `legacy_room_no` on
/// the existing `ht_checkins` row so operators can still see WHICH
/// room was cancelled. Hashing room context on both sides therefore
/// diverges forever: legacy emits `""`, PG emits `Some("301")`. Six
/// stuck `value` drifts in production (CH26-005252, CH26-005270,
/// CH26-005487, CH26-005524, CH26-005527, CH26-005543) are the
/// live manifestation as of the 2026-05-19 audit.
///
/// When `cancelled = true`, this function returns a sentinel
/// `sha256("CANCELLED|{legacy_cin_no}")` and IGNORES every other
/// input. Both the legacy and canonical reconcile paths call with
/// the same `cancelled` decision (`cin_status == "cancelled"`), so
/// the sentinel collapses the parity gap deterministically. When
/// `cancelled = false`, the active-stay 5-field shape is unchanged
/// — pre-2026-05-19 hash bytes are preserved bit-for-bit.
pub(crate) fn checkin_canonical_hash(
    legacy_cin_no: &str,
    legacy_room_no: Option<&str>,
    cin_checkin_time: Option<&str>,
    cin_checkout_time: Option<&str>,
    legacy_cust_no: Option<&str>,
    checked_out: bool,
    cancelled: bool,
) -> String {
    // Shape selector — stays a pre-join early return so the sentinel
    // never picks up segment bytes.
    if cancelled {
        return sha256(&format!("CANCELLED|{}", legacy_cin_no));
    }
    // The `co=` prefix belongs to the checked-out SEGMENT, not to the
    // separator — see `sync::mappers::checkin::HASH_INPUTS`.
    sha256(&join_hash_segments(&[
        legacy_cin_no.to_string(),
        legacy_room_no.unwrap_or("").to_string(),
        cin_checkin_time.unwrap_or("").to_string(),
        cin_checkout_time.unwrap_or("").to_string(),
        legacy_cust_no.unwrap_or("").to_string(),
        format!("co={}", checked_out),
    ]))
}

/// Render a money value into its reconcile-hash segment.
///
/// Two decimals on BOTH sides: canonical `ht_payments.pay_amount` is
/// `DECIMAL(12,2)` while legacy `HT_Receipt_H.Receipt_Total` is a bare
/// `float`, so the fixed precision is what makes the two comparable at
/// all (a float `890.0000000001` must hash like `890.00`).
///
/// The `-0.0` normalisation is not cosmetic: IEEE `-0.0` renders as
/// `"-0.00"` while `0.0` renders as `"0.00"`, and `-0.0 == 0.0` is true —
/// so a zero-total receipt could otherwise hash differently on the two
/// sides forever with nothing observable to fix.
pub(crate) fn money_hash_segment(amount: f64) -> String {
    let normalised = if amount == 0.0 { 0.0 } else { amount };
    format!("{:.2}", normalised)
}

/// Render the void bit into its reconcile-hash segment. The `voided=`
/// prefix belongs to the SEGMENT, not the separator — same convention as
/// the check-in hash's `co=`.
pub(crate) fn voided_hash_segment(voided: bool) -> String {
    format!("voided={}", voided)
}

/// Hash inputs for one canonical-shape payment (legacy `HT_Receipt_H`)
/// row. Phase 6-A; keyed on `Receipt_no`.
///
/// **Why `Receipt_no` and NOT `Pay_No`:** `Pay_No` is a pointer into the
/// per-line `HT_CheckIn_Pay` ledger (many lines share one), whereas
/// `Receipt_no` is the receipt artefact's own unique business key — and
/// it is what `apply_receipt_upsert` resolves canonical rows by
/// (`legacy_receipt_no` / `pay_reference`).
///
/// **Deliberately excluded** (each would be permanent, unfixable sync
/// lag rather than signal):
/// * `pay_date` — `RECEIPT_UPSERT_UPDATE_SQL` COALESCEs it, so an
///   app-originated payment keeps its own creation instant and can never
///   converge on legacy `Receipt_Date`;
/// * `pay_method` — `HT_Receipt_H` doesn't carry the tender (that lives
///   in the matching `HT_CheckIn_Pay` line), so the mapper defaults the
///   column to `'cash'` and never mirrors it in either direction.
///
/// **Known one-way asymmetry, deliberately hashed:** canonical void is
/// PG-only (`repository/payment.rs::void`, no writeback recipe) and the
/// mapper's `pay_voided` fold is MONOTONIC, so a canonically-voided
/// payment whose legacy `status_name` is still `'ปกติ'` diverges here and
/// CANNOT self-heal. That is a genuine cross-app money-reporting
/// disagreement (iHOTEL's shift report still counts the receipt, ours
/// doesn't), so it is signal — but expect such rows to need operator
/// action, not patience. This is one reason `payments` is deliberately
/// absent from [`FORCE_CONVERGE_VALUE_DRIFT_TABLES`].
///
/// **That shape needs an explicit carve-out to stay OBSERVABLE, and has
/// one** (2026-07-28 review). A PG-only void never moves the LEGACY hash,
/// and `sync_payments` short-circuits on `acked == mssql_hash` *before* it
/// fetches the canonical row — so without help, only the canonical-only
/// voids that already existed at first-enable would ever be reported, and
/// a void performed in our app after a receipt was acked as converged
/// would be invisible to the arm forever.
/// [`load_canonically_voided_receipt_keys`] closes that: it lifts the
/// currently-voided canonical receipt keys in ONE batched read per tick,
/// and [`payment_ack_short_circuit_bypassed`] re-opens the comparison for
/// exactly the asymmetric pair (canonical voided ∧ legacy not cancelled).
/// Do not "simplify" that bypass back into a plain ack short-circuit — the
/// money path is only continuously monitored because of it.
pub(crate) fn payment_canonical_hash(
    receipt_no: &str,
    amount: f64,
    voided: bool,
    legacy_cin_no: Option<&str>,
) -> String {
    sha256(&join_hash_segments(&[
        receipt_no.to_string(),
        money_hash_segment(amount),
        voided_hash_segment(voided),
        legacy_cin_no.unwrap_or("").to_string(),
    ]))
}

/// Hash inputs for one canonical-shape companion FOLIO (legacy
/// `HT_CheckIn_Other_People` rows sharing a `Cin_no`). Phase 6-B; keyed on
/// `Cin_no`.
///
/// **Why the folio is the unit.** iHOTEL edits companions by
/// DELETE-then-REINSERT (`FrmCheckIn.cs:9975`), minting a new IDENTITY per
/// edit, and the CT mapper mirrors that faithfully. A per-ROW arm keyed on
/// that id would therefore report two divergences on every correctly-applied
/// edit — one for the retired id (which can never converge) and one for the
/// new one — while a folio hash is invariant under the churn and moves only
/// when the companion CONTENT does. See
/// [`crate::sync::mappers::guest_registry::RegistryFolioProjection`].
///
/// The body is `cin_no | <sorted "{name}|{country}" lines joined by \n>`,
/// with ids on both sides excluded. An EMPTY folio (a check-in with no
/// companions) is a real, hashable state, not an absent row — that is what
/// lets a folio whose companions were legitimately deleted on both sides
/// auto-resolve rather than sit open forever.
pub(crate) fn guest_registry_canonical_hash(
    folio: &crate::sync::mappers::guest_registry::RegistryFolioProjection,
) -> String {
    sha256(&join_hash_segments(&[
        folio.legacy_cin_no.clone(),
        folio.companions_segment(),
    ]))
}

/// Track D / T7 CRIT-1 — discriminator for `ht_reconcile_log.divergence_kind`.
/// Pure enum so the reconcile loop and the (never-silenced) ack guard
/// agree on the same vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivergenceKind {
    /// Same row count on both sides; content drift (a CT-mapper bug
    /// projected a column wrong, an operator hand-edited canonical,
    /// etc.). Acked by hash — subsequent ticks short-circuit once the
    /// `mssql_hash` stops changing.
    Value,
    /// Row counts differ between MSSQL and PG. Canonical example: a
    /// multi-room folio (3 rows in `View_CheckIn_Ds`) collapsed into 1
    /// `ht_checkins` row by the CT mapper's `first_room_no`
    /// denormalisation. Hashes will never match while the cardinality
    /// asymmetry exists — Track D / T7 CRIT-1 says this case must
    /// NEVER be acked, so every tick re-fires until operator action
    /// (Track B junction-table migration, or a hand-applied fix).
    Cardinality,
    /// Canonical PG row is missing entirely (`pg_hash IS NULL`). The CT
    /// watcher hasn't yet projected this PK into canonical — could be
    /// a transient watermark lag, a mapper bug, or a CT retention
    /// overflow. Highest-signal divergence; never silenced.
    MissingPg,
    /// Legacy MSSQL row is missing for a PK that exists in canonical.
    /// Should be impossible under normal flow (writeback owns the PG-to-
    /// MSSQL direction) but kept for symmetry — if it ever fires, the
    /// writeback is broken and operator needs to know.
    MissingMssql,
}

impl DivergenceKind {
    /// String literal stored in `ht_reconcile_log.divergence_kind`.
    /// Schema constraint isn't enforced via CHECK so the column can
    /// hold any TEXT — but readers (alerts, dashboards) expect these
    /// exact values.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Cardinality => "cardinality",
            Self::MissingPg => "missing_pg",
            Self::MissingMssql => "missing_mssql",
        }
    }

    /// Track D / T7 CRIT-1 invariant: the cache ack must NEVER silence
    /// a cardinality drift or a canonical-missing divergence. Acking
    /// these would write the MSSQL hash into `ht_*_legacy.sync_hash`,
    /// short-circuiting subsequent ticks — but the underlying drift is
    /// not repaired by hash alone, so the alert would go quiet while
    /// canonical stayed wrong forever.
    pub fn is_silenceable(self) -> bool {
        matches!(self, Self::Value | Self::MissingMssql)
    }
}

/// Track D / T7 CRIT-1 — classify a divergence by hash + row-count
/// comparison. Pure function so the unit tests can exercise the truth
/// table without a PG pool.
///
/// * `pg_hash = None` ⇒ `MissingPg` (highest signal).
/// * `mssql_hash = None` ⇒ `MissingMssql`.
/// * `legacy_row_count != pg_row_count` ⇒ `Cardinality` (multi-room
///   folio collapse, junction-table gap).
/// * otherwise ⇒ `Value` (hash mismatch with matching cardinality).
///
/// The caller has already determined that the two hashes differ; this
/// helper just sub-classifies the drift for ack-silencing purposes.
pub fn classify_divergence(
    pg_hash: Option<&str>,
    mssql_hash: Option<&str>,
    legacy_row_count: i32,
    pg_row_count: i32,
) -> DivergenceKind {
    if pg_hash.is_none() {
        return DivergenceKind::MissingPg;
    }
    if mssql_hash.is_none() {
        return DivergenceKind::MissingMssql;
    }
    if legacy_row_count != pg_row_count {
        return DivergenceKind::Cardinality;
    }
    DivergenceKind::Value
}

/// Phase 5.5 diff-only path: record a divergence into `ht_reconcile_log`
/// instead of mutating canonical state. Best-effort — a failed insert
/// only degrades observability, so we never bubble it up to abort the
/// reconcile loop.
///
/// Track D / T7 CRIT-1: every row now carries `divergence_kind` +
/// `legacy_row_count` + `pg_row_count` so cardinality drift (e.g.
/// multi-room folio collapsed by the CT mapper) is distinguishable
/// from value drift, and the ack-cache (`ht_*_legacy.sync_hash`)
/// silencing rule can refuse to silence non-`value` kinds.
/// Insert a fresh `ht_reconcile_log` row UNLESS an unresolved row
/// already exists for the same `(table_name, legacy_pk, divergence_kind,
/// mssql_hash)` tuple. This dedupe prevents unbounded log growth for
/// kinds that re-fire every tick (most notably `cardinality`, which by
/// design never silences via the cache-ack path — see
/// `DivergenceKind::is_silenceable`).
///
/// **Why mssql_hash is part of the dedupe key:** for `value` drift, a
/// new MSSQL UPDATE produces a new `mssql_hash`, so a fresh row IS
/// recorded (new state worth investigating) — that's the intended
/// semantic. For `cardinality`, the MSSQL hash is stable across ticks
/// while the asymmetry persists, so only one unresolved row per PK
/// accumulates. For `missing_pg`, the canonical-missing predicate
/// produces a stable mssql_hash (the actual legacy hash for the PK),
/// same dedupe behavior.
///
/// 2026-05-18 incident driver: after PR #128 restored sync_checkins,
/// HF Hotel was inserting ~766 cardinality rows per 15-min tick (one
/// per multi-room folio in `HT_CheckIn_Ds`), projecting `ht_reconcile_log`
/// past 100k unresolved rows within 18-24 hours and degrading the
/// partial-index `idx_ht_reconcile_log_table_unresolved` query plan.
/// Cardinality is real drift but it doesn't get materially more drifted
/// with every re-detection — one row per (PK, hash) is enough.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn record_divergence(
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
    pg_hash: Option<&str>,
    mssql_hash: Option<&str>,
    mssql_row_json: serde_json::Value,
    pg_row_json: Option<serde_json::Value>,
    divergence_kind: DivergenceKind,
    legacy_row_count: i32,
    pg_row_count: i32,
) {
    // Conditional INSERT: skip when an unresolved row with the same
    // (table_name, legacy_pk, divergence_kind, mssql_hash) already
    // exists. NULL-safe comparison via IS NOT DISTINCT FROM so
    // missing_pg rows (mssql_hash = legacy hash) and missing_mssql
    // (mssql_hash NULL) both dedupe correctly.
    let result = sqlx::query(
        "INSERT INTO ht_reconcile_log \
            (table_name, legacy_pk, pg_hash, mssql_hash, \
             mssql_row_json, pg_row_json, \
             divergence_kind, legacy_row_count, pg_row_count) \
         SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9 \
         WHERE NOT EXISTS ( \
             SELECT 1 FROM ht_reconcile_log \
              WHERE table_name = $1 \
                AND legacy_pk = $2 \
                AND divergence_kind = $7 \
                AND mssql_hash IS NOT DISTINCT FROM $4 \
                AND resolved_at IS NULL \
         )",
    )
    .bind(table_name)
    .bind(legacy_pk)
    .bind(pg_hash)
    .bind(mssql_hash)
    .bind(mssql_row_json)
    .bind(pg_row_json)
    .bind(divergence_kind.as_str())
    .bind(legacy_row_count)
    .bind(pg_row_count)
    .execute(pg_pool)
    .await;
    if let Err(e) = result {
        tracing::warn!(
            table_name,
            legacy_pk,
            error = %e,
            "[Sync] Failed to record divergence in ht_reconcile_log — observability degraded"
        );
    }
}

/// Pure decision helper for the auto-resolve sweep. A row in
/// `ht_reconcile_log` may be auto-resolved when:
///
/// 1. **Primary convergence** — a freshly-projected legacy (MSSQL) hash
///    and a freshly-computed canonical PG hash are both present, non-empty,
///    and equal. A `None` on either side is normally an intentional skip
///    (missing-PG cases must persist until canonical actually catches up;
///    missing-MSSQL cases need operator review). This helper NEVER resolves
///    a missing-PG row on its own — the 2026-07-27 re-ingest arm makes
///    canonical appear FIRST (see [`reingest_missing_pg_eligible`]) and only
///    then re-tests through this same function, so the convergence contract
///    below stays the single place a row can be closed.
///
/// 2. **Stale-ghost convergence (bookings only)** — the legacy composite
///    key has since *disappeared* (`current_legacy_hash == None`) yet
///    canonical PG now exists and matches the legacy state recorded at
///    detection time (`recorded_mssql_hash`). A booking's legacy PK is
///    `{book_no}|{room_type}`, and the room-type half churns routinely
///    over a booking's life (room reassignment, multi-room edits). A row
///    first logged as `missing_pg` during the brief window before a
///    NEW-app booking links its `legacy_book_id` becomes permanently
///    unresolvable once that initial room-type line is swapped out: the
///    legacy detail row for that exact key is gone, so the primary arm —
///    which needs BOTH sides present — can never fire. Because the
///    booking-level hash is room-type-independent, a match against the
///    recorded `mssql_hash` proves the booking itself is fully reconciled;
///    the per-room-type key simply churned away. This arm is restricted
///    to bookings: a vanished legacy key for rooms / customers / checkins
///    is a genuine anomaly that must stay open for operator review.
///    (Live evidence 2026-06-24: bookings row `R015423|501`.)
///
/// The auto-resolve sweep only reaches this helper with a `None`
/// `current_legacy_hash` when the legacy re-fetch returned `Ok(None)`
/// (row genuinely absent) — a re-fetch *error* skips the row earlier, so
/// a transient MSSQL outage can never be mistaken for a churned key.
///
/// Pulled into a free function so the unit tests can exercise the
/// truth table without a live PG pool.
fn should_auto_resolve(
    table_name: &str,
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
    recorded_mssql_hash: Option<&str>,
) -> bool {
    match (current_legacy_hash, current_pg_hash) {
        (Some(legacy), Some(pg)) if !legacy.is_empty() && !pg.is_empty() => legacy == pg,
        (None, Some(pg)) if table_name == "bookings" && !pg.is_empty() => {
            matches!(recorded_mssql_hash, Some(rec) if !rec.is_empty() && rec == pg)
        }
        _ => false,
    }
}

/// Parse the composite booking PK as stored in `ht_reconcile_log.legacy_pk`.
/// Bookings serialise as `"{book_no}|{room_type_key}"`; pre-Phase-6-hotfix
/// rows lack the separator and are interpreted as `(legacy_pk, "")`. Pure
/// so the booking-fetch dispatch + the canonical-fetch dispatch agree on
/// the parse rule.
fn parse_booking_legacy_pk(legacy_pk: &str) -> (&str, &str) {
    legacy_pk.split_once('|').unwrap_or((legacy_pk, ""))
}

/// Every `ht_reconcile_log.table_name` that BOTH resolve dispatches
/// below must handle.
///
/// A detected entity with no resolve arm is undetectably broken: the
/// sweep falls through to `_ => Ok(None)`, `current_legacy_hash` and
/// `current_pg_hash` both come back `None`, and every row for that
/// entity sits open forever. That is exactly what happened to `rooms`
/// (live evidence 2026-05-18). The `debug_assert!` in each wildcard arm
/// turns "someone added detection without a resolve arm" into a test
/// failure instead of a silent backlog, and
/// `gate_guard::tests::resolvable_tables_const_covers_every_contract_entity`
/// pins this list against the entity registry.
///
/// Phase 6-C appends the mirror-probe keys. They are NOT entity contracts
/// (no CT mapper idempotency gate to be a superset of — the mirror mappers
/// DELETE+INSERT unconditionally); they are resolvable because
/// [`crate::scheduler::mirror_probe::probe_for_table`] backs both dispatch
/// arms below. Leaving them OUT of this list would not trip any
/// `debug_assert!` — the assert only fires for listed-but-undispatched
/// names — and that is precisely why it was not done: the rows would sit
/// open forever and, being selected by age alone, would eventually own the
/// sweep's whole 500-row batch. Phase 6-D appends the payment-ledger probe
/// key on exactly the same terms (`payment_ledger_probe::is_payment_ledger_probe`
/// backs both dispatch arms).
/// `gate_guard::tests::every_resolvable_table_is_a_contract_entity_or_a_probe`
/// keeps the two populations explicit.
pub(crate) const RECONCILE_RESOLVABLE_TABLES: &[&str] = &[
    "customers",
    "bookings",
    "checkins",
    "rooms",
    "payments",
    "guest_registry",
    // Phase 6-C mirror probes — pinned against
    // `mirror_probe::mirror_probe_keys()` by a unit test below.
    "mirror_ht_cupon",
    "mirror_ht_checkin_product",
    "mirror_ht_deposit",
    "mirror_ht_changed_room",
    "mirror_ht_bill_debt_h",
    "mirror_ht_bill_debt_ds",
    "mirror_ht_rooms_cancel",
    "mirror_ht_book_pro",
    // Issue #273 (remainder): DETECTION is re-keyed off `rcal_legacy_id`
    // onto the BUSINESS key `(room, night)` — see
    // `probe_room_calendar_business_key` — the SAME key RESOLUTION
    // (`compute_room_calendar_business_key_{pg,legacy}_hash`) already
    // resolves on since the closure arm. The two can no longer disagree
    // about "converged", which is what makes recording safe: a row this
    // arm opens measures a gap a re-drive CAN close, not an id-binding
    // artefact that nothing can. No remediation/re-drive path ships in
    // this change — a recorded row stays open (real sync lag, same as
    // `guest_registry` / `payment_ledger_probe`) until one does. Pinned by
    // `ROOM_CALENDAR_PROBE_KEY`; see the closure-arm section.
    "mirror_ht_room_calendar",
    // Phase 6-D payment-ledger probe. Same population as the 6-C probes (not
    // an entity contract — `mirror_payment_ledger` DELETEs the folio and
    // re-INSERTs it unconditionally, so there is no idempotency gate for a
    // hash to be a superset of), and resolvable for the same reason: its rows
    // ARE closeable (re-drive with the `backfill_payment_ledger` bin, then the
    // sweep sees equal hashes), so they must never be left un-dispatched.
    // Pinned against `PAYMENT_LEDGER_PROBE_KEY` by a unit test below.
    "payment_ledger_probe",
];

/// Re-compute the canonical PG hash for a single `ht_reconcile_log`
/// row's `(table_name, legacy_pk)` pair. Returns `Ok(None)` if no
/// canonical row exists today (still drifted), or `Ok(Some(hash))`
/// when canonical has converged.
///
/// Dispatches on the same table-name vocabulary the reconcile loop
/// writes into `ht_reconcile_log.table_name` ("customers", "bookings",
/// "checkins", "rooms" — see [`RECONCILE_RESOLVABLE_TABLES`]). Other
/// table names return `Ok(None)` so the row stays in the queue for
/// operator review.
async fn compute_current_pg_hash(
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
) -> Result<Option<String>, sqlx::Error> {
    match table_name {
        "customers" => {
            let canonical = fetch_canonical_customer(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                customer_canonical_hash(
                    legacy_pk,
                    &c.cust_firstname,
                    c.cust_type.as_deref(),
                    c.cust_phone.as_deref(),
                    c.cust_idcard.as_deref(),
                    c.cust_address.as_deref(),
                )
            }))
        }
        "bookings" => {
            // Composite PK serialised as "{book_no}|{room_type_key}";
            // canonical hash is keyed by `book_no` (the legacy_book_id)
            // — matches `sync_bookings`' canonical-side hash inputs.
            let (book_no, _room_type_key) = parse_booking_legacy_pk(legacy_pk);
            let canonical = fetch_canonical_booking(pg_pool, book_no).await?;
            Ok(canonical.map(|c| {
                let checkin_str = c.book_checkin.map(|d| d.to_string());
                let checkout_str = c.book_checkout.map(|d| d.to_string());
                booking_canonical_hash(
                    book_no,
                    checkin_str.as_deref(),
                    checkout_str.as_deref(),
                    c.legacy_cust_no.as_deref(),
                )
            }))
        }
        "checkins" => {
            let canonical = fetch_canonical_checkin(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                let effective_checkout = c.effective_checkout_date().map(|d| d.to_string());
                let cancelled = c.is_cancelled();
                let checked_out = c.is_checked_out();
                checkin_canonical_hash(
                    legacy_pk,
                    c.legacy_room_no.as_deref(),
                    c.cin_checkin_time.map(|t| t.to_string()).as_deref(),
                    effective_checkout.as_deref(),
                    c.legacy_cust_no.as_deref(),
                    checked_out,
                    cancelled,
                )
            }))
        }
        "rooms" => {
            // Canonical-side re-hash from `ht_rooms_new` under the SAME
            // narrowed projection `sync_rooms` uses on the legacy side
            // (room_no + clean + maintenance + notes). Without this arm
            // every rooms drift row sat open forever — the post-detection
            // sweep fell through to `_ => Ok(None)` even after the
            // underlying state had converged (live A2-1 evidence,
            // 2026-05-18: current_legacy_hash=None current_pg_hash=None).
            let canonical = fetch_canonical_room(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                room_canonical_hash(
                    legacy_pk,
                    clean_bool_to_legacy_yesno(c.room_clean),
                    bool_to_yesno(c.room_maintenance),
                    c.room_notes.as_deref(),
                )
            }))
        }
        "payments" => {
            // Phase 6-A. `legacy_pk` is the receipt's `Receipt_no`; the
            // canonical probe mirrors `apply_receipt_upsert`'s own
            // `(legacy_receipt_no = $1 OR pay_reference = $1)` shape so
            // the sweep resolves the SAME row the mapper would write.
            let canonical = fetch_canonical_payment(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                payment_canonical_hash(
                    legacy_pk,
                    c.pay_amount,
                    c.is_voided(),
                    c.legacy_cin_no.as_deref(),
                )
            }))
        }
        "guest_registry" => {
            // Phase 6-B. `legacy_pk` is the folio's `Cin_no`. `Ok(None)`
            // ONLY when the parent check-in is absent from canonical — a
            // folio that exists but holds no companions hashes as the
            // EMPTY folio, so a companion set deleted on both sides
            // converges instead of sitting open forever.
            Ok(fetch_canonical_registry_folio(pg_pool, legacy_pk)
                .await?
                .as_ref()
                .map(guest_registry_canonical_hash))
        }
        // Issue #273 — the calendar's `<aggregate>` row resolves on the
        // BUSINESS key `(room, night)`. Deliberately AHEAD of the generic
        // probe arm below: that arm recomputes the `rcal_legacy_id`-keyed
        // aggregate, which is never-equal by construction (the mapper NULLs
        // the id on an allocator rebind and nothing restores it), so a
        // calendar row dispatched there could never converge. Per-PK calendar
        // rows — which the probe cannot produce, it is `per_pk: false` — fall
        // through to the generic arm unchanged.
        t if t == ROOM_CALENDAR_PROBE_KEY
            && legacy_pk == crate::scheduler::mirror_probe::MIRROR_AGGREGATE_PK =>
        {
            Ok(Some(
                compute_room_calendar_business_key_pg_hash(pg_pool).await?,
            ))
        }
        // Phase 6-C. `legacy_pk` is either a real mirrored key or the
        // `<aggregate>` sentinel. Never `Ok(None)` for a registered probe:
        // an ABSENT key hashes to `mirror_absent_hash` so a row deleted on
        // both sides converges instead of sitting open forever.
        t if crate::scheduler::mirror_probe::probe_for_table(t).is_some() => {
            let probe = crate::scheduler::mirror_probe::probe_for_table(t)
                .expect("guard just matched");
            crate::scheduler::mirror_probe::resolve_pg_hash(pg_pool, probe, legacy_pk).await
        }
        // Phase 6-D. `legacy_pk` is either a `Cin_No` or the `<aggregate>`
        // sentinel. Never `Ok(None)`: an absent folio hashes to
        // `folio_absent_hash` so a folio deleted on both sides converges.
        t if crate::scheduler::payment_ledger_probe::is_payment_ledger_probe(t) => {
            crate::scheduler::payment_ledger_probe::resolve_pg_hash(pg_pool, legacy_pk).await
        }
        _ => {
            debug_assert!(
                !RECONCILE_RESOLVABLE_TABLES.contains(&table_name),
                "resolve arm missing for {table_name} in compute_current_pg_hash \
                 — the entity is listed as resolvable but falls through to the \
                 wildcard, so every reconcile row for it stays open forever \
                 (2026-05-18, rooms)"
            );
            Ok(None)
        }
    }
}

/// Re-project a single MSSQL row/group by PK under the CURRENT
/// `*_RECONCILE_PROJECTION` constants and produce a fresh `mssql_hash`.
/// Returns `Ok(None)` when the row no longer exists on the legacy side
/// (treated as "still drifted — leave for operator review"), or when
/// `table_name` is outside the dispatched set (future entities that
/// don't yet have a legacy-side fetch path).
///
/// Mirrors `compute_current_pg_hash` so the auto-resolve sweep can
/// compare like-for-like under the current projection. The whole point
/// of re-fetching MSSQL — rather than trusting `ht_reconcile_log.mssql_hash`
/// — is that a projection change invalidates every pre-fix stored hash.
///
/// **Checkins (2026-05-16):** uses the CT mapper's `project_aggregate`
/// directly — loads the full MSSQL aggregate via `load_checkin_aggregate`
/// then projects through `crate::sync::mappers::project_checkin_aggregate`,
/// producing a `CanonicalCheckIn` whose hash inputs are identical to what
/// the mapper would write into PG. By construction, the resulting hash is
/// what canonical SHOULD be, so a mismatch with the actual canonical row
/// can only signal real exogenous drift (CT miss, hand-edited PG), not
/// parallel-projection-pipeline divergence. Bug B / C / D class becomes
/// impossible at this layer.
///
/// Customers + bookings + rooms still use their light-weight per-PK
/// projections (`fetch_legacy_customer_hash`, `fetch_legacy_booking_hash`,
/// `fetch_legacy_room_hash`) — the same unification is a follow-on for
/// those entities.
///
/// **Why this takes `pg_pool`** (Phase 6-C): a mirror probe's `<aggregate>`
/// row is only comparable inside the mirror's own coverage floor, and that
/// floor is a `MIN(pk)` over the MIRROR side. Re-deriving it here each sweep
/// — rather than freezing it onto the reconcile row — means a floor that
/// MOVES because the mirror finally received its missing history is picked
/// up on the next tick. No other arm reads the pool.
async fn compute_current_legacy_hash(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    match table_name {
        "customers" => fetch_legacy_customer_hash(legacy_pool, legacy_pk).await,
        "bookings" => {
            let (book_no, room_type_key) = parse_booking_legacy_pk(legacy_pk);
            fetch_legacy_booking_hash(legacy_pool, book_no, room_type_key).await
        }
        "checkins" => compute_legacy_checkin_hash_via_mapper(legacy_pool, legacy_pk).await,
        "rooms" => fetch_legacy_room_hash(legacy_pool, legacy_pk).await,
        "payments" => fetch_legacy_payment_hash(legacy_pool, legacy_pk).await,
        "guest_registry" => fetch_legacy_registry_folio_hash(legacy_pool, legacy_pk).await,
        // Issue #273 — calendar business-key arm. Sibling of the
        // `compute_current_pg_hash` arm and ordered ahead of the generic
        // probe arm for the same reason (the id-keyed aggregate is
        // never-equal by construction). Reads `pg_pool` for the era floor:
        // the coverage boundary is a `MIN(rcal_date)` over the MIRROR.
        t if t == ROOM_CALENDAR_PROBE_KEY
            && legacy_pk == crate::scheduler::mirror_probe::MIRROR_AGGREGATE_PK =>
        {
            Ok(Some(
                compute_room_calendar_business_key_legacy_hash(legacy_pool, pg_pool).await?,
            ))
        }
        // Phase 6-C — mirror probes. Sibling of the `compute_current_pg_hash`
        // arm; same absent-is-a-real-hash contract.
        t if crate::scheduler::mirror_probe::probe_for_table(t).is_some() => {
            let probe = crate::scheduler::mirror_probe::probe_for_table(t)
                .expect("guard just matched");
            crate::scheduler::mirror_probe::resolve_legacy_hash(
                legacy_pool,
                pg_pool,
                probe,
                legacy_pk,
            )
            .await
        }
        // Phase 6-D — payment-ledger probe. Sibling of the
        // `compute_current_pg_hash` arm; same absent-is-a-real-hash contract,
        // and it reads `pg_pool` for the same reason (the `<aggregate>` row's
        // coverage floor is a `MIN` over the CANONICAL side).
        t if crate::scheduler::payment_ledger_probe::is_payment_ledger_probe(t) => {
            crate::scheduler::payment_ledger_probe::resolve_legacy_hash(
                legacy_pool,
                pg_pool,
                legacy_pk,
            )
            .await
        }
        _ => {
            debug_assert!(
                !RECONCILE_RESOLVABLE_TABLES.contains(&table_name),
                "resolve arm missing for {table_name} in \
                 compute_current_legacy_hash — the entity is listed as \
                 resolvable but falls through to the wildcard, so every \
                 reconcile row for it stays open forever (2026-05-18, rooms)"
            );
            Ok(None)
        }
    }
}

/// Unified-projection MSSQL hash for a single check-in PK. Loads the
/// full legacy aggregate (header + Ds + Pay) and runs it through the
/// CT mapper's `project_aggregate`, then hashes from the resulting
/// `CanonicalCheckIn` using the same `effective_checkout_date` rule
/// the canonical-side projection uses (see
/// `CanonicalCheckinRow::effective_checkout_date`).
///
/// Cost: 3 MSSQL queries per call (header / Ds / Pay), versus 1 for
/// the old `fetch_legacy_checkin_hash`. The sweep is bounded to 500
/// rows per 15-min tick, so the steady-state load is ~1.7 MSSQL
/// queries/sec — well below saturation on the same-LAN legacy DB.
/// The architectural correctness payoff (no parallel projection
/// pipeline) outweighs the per-call cost.
async fn compute_legacy_checkin_hash_via_mapper(
    legacy_pool: &DbPool,
    cin_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let aggregate = crate::sync::parent_loader::load_checkin_aggregate(legacy_pool, cin_no).await?;
    if !aggregate.is_present() {
        return Ok(None);
    }
    let canonical = crate::sync::mappers::project_checkin_aggregate(&aggregate, cin_no)?;
    // Effective checkout: prefer actual departure when set (Bug D
    // alignment with canonical-side `effective_checkout_date()`).
    let effective_checkout = canonical
        .cin_checkout_time
        .map(|dt| dt.date())
        .unwrap_or(canonical.cin_expected_checkout)
        .to_string();
    let hash = checkin_canonical_hash(
        &canonical.legacy_cin_no,
        canonical.legacy_room_no.as_deref(),
        Some(canonical.cin_checkin_time.to_string()).as_deref(),
        Some(effective_checkout).as_deref(),
        canonical.legacy_cust_no.as_deref(),
        canonical.cin_checkout_time.is_some(),
        canonical.cin_status == "cancelled",
    );
    Ok(Some(hash))
}

/// Single-PK MSSQL re-projection for customers. Mirrors `sync_customers`'
/// per-row hash construction so a projection change in
/// `CUSTOMERS_RECONCILE_PROJECTION` self-heals on the next sweep tick.
async fn fetch_legacy_customer_hash(
    legacy_pool: &DbPool,
    cust_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Customers WHERE Cust_no = @P1",
        projection = CUSTOMERS_RECONCILE_PROJECTION,
    );
    let mut q = Query::new(sql.as_str());
    q.bind(cust_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    let row_cust_no = row
        .get::<&str, _>("Cust_no")
        .unwrap_or_default()
        .to_string();
    let cust_name = row.get::<&str, _>("Cust_name").map(String::from);
    let cust_type = row.get::<&str, _>("Cust_Type_Main").map(String::from);
    let cust_phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
    let cust_idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
    let cust_address = row.get::<&str, _>("Cust_Add_no").map(String::from);
    Ok(Some(customer_canonical_hash(
        &row_cust_no,
        cust_name.as_deref().unwrap_or(""),
        cust_type.as_deref(),
        cust_phone.as_deref(),
        cust_idcard.as_deref(),
        cust_address.as_deref(),
    )))
}

/// Single-composite-PK MSSQL re-projection for bookings. `View_Booking_Ds`
/// returns up to 3 rows per `(Book_No, Book_Room_Type)`; we group exactly
/// the same way `sync_bookings` does and take the deterministic
/// representative so the hash inputs match. `Ok(None)` when no matching
/// group exists today (legacy-side deletion since detection).
async fn fetch_legacy_booking_hash(
    legacy_pool: &DbPool,
    book_no: &str,
    room_type_key: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM View_Booking_Ds WHERE Book_No = @P1",
        projection = BOOKINGS_RECONCILE_PROJECTION.join(", "),
    );
    let mut q = Query::new(sql.as_str());
    q.bind(book_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;

    let mut groups: BTreeMap<(String, String), Vec<BookingDetail>> = BTreeMap::new();
    for row in &rows {
        let row_book_no = row
            .get::<&str, _>("Book_No")
            .unwrap_or_default()
            .to_string();
        let row_room_type = row.get::<&str, _>("Book_Room_Type").map(String::from);
        let detail = BookingDetail {
            book_date: row.try_get("Book_Date").unwrap_or(None),
            book_date_in: row.try_get("Book_Date_in").unwrap_or(None),
            book_date_out: row.try_get("Book_Date_out").unwrap_or(None),
            book_cust_name: row.get::<&str, _>("Book_Cust_Name").map(String::from),
            book_cust_id: row.get::<&str, _>("Book_Cust_ID").map(String::from),
            book_status: row.get::<i32, _>("Book_Status"),
            book_room_type: row_room_type.clone(),
        };
        let key = row_room_type.unwrap_or_default();
        groups.entry((row_book_no, key)).or_default().push(detail);
    }

    let Some(mut details) = groups.remove(&(book_no.to_string(), room_type_key.to_string())) else {
        return Ok(None);
    };
    sort_booking_details(&mut details);
    let representative = details.first();
    let book_checkin_date =
        representative.and_then(|d| d.book_date_in.map(|dt| dt.date().to_string()));
    let book_checkout_date =
        representative.and_then(|d| d.book_date_out.map(|dt| dt.date().to_string()));
    let book_cust_id_owned = representative.and_then(|d| d.book_cust_id.clone());
    Ok(Some(booking_canonical_hash(
        book_no,
        book_checkin_date.as_deref(),
        book_checkout_date.as_deref(),
        book_cust_id_owned.as_deref(),
    )))
}

/// Single-PK MSSQL re-projection for rooms. Mirrors `sync_rooms`'
/// per-row hash construction (room_no + clean + maintenance + notes,
/// with `legacy_yesno_canonical` collapsing legacy literals) so the
/// auto-resolve sweep can re-fetch a room's CURRENT legacy hash and
/// compare against canonical. `Ok(None)` when the row no longer
/// exists on the legacy side (treated as "still drifted — leave for
/// operator review").
async fn fetch_legacy_room_hash(
    legacy_pool: &DbPool,
    room_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Rooms WHERE Room_no = @P1",
        projection = ROOMS_RECONCILE_PROJECTION.join(", "),
    );
    let mut q = Query::new(sql.as_str());
    q.bind(room_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    let row_room_no = row
        .get::<&str, _>("Room_no")
        .unwrap_or_default()
        .to_string();
    let room_clean = row.get::<&str, _>("Room_Clean").map(String::from);
    let room_manternace = row.get::<&str, _>("Room_Manternace").map(String::from);
    let room_details = row.get::<&str, _>("Room_Details").map(String::from);
    Ok(Some(room_canonical_hash(
        &row_room_no,
        legacy_yesno_canonical(room_clean.as_deref()),
        legacy_yesno_canonical(room_manternace.as_deref()),
        room_details.as_deref(),
    )))
}

/// Single-PK MSSQL re-projection for payments (Phase 6-A). Mirrors
/// `sync_payments`' per-row hash construction so the auto-resolve sweep
/// compares like-for-like under the CURRENT projection. `Ok(None)` when
/// the receipt no longer exists on the legacy side, or when it has lost
/// its `Receipt_ref` (the bulk scan excludes those rows too — a
/// no-check-in receipt is a deliberate mapper skip, not sync lag).
///
/// The canonical-era floor ([`PAYMENTS_ERA_FLOOR_SQL`]) is deliberately NOT
/// applied here: this path re-projects a receipt the scan ALREADY admitted
/// and logged, so it must reproduce that row's hash unconditionally. Adding
/// the floor would make an in-flight row un-re-projectable if the floor ever
/// moved forward.
async fn fetch_legacy_payment_hash(
    legacy_pool: &DbPool,
    receipt_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Receipt_H WHERE Receipt_no = @P1",
        projection = PAYMENTS_RECONCILE_PROJECTION.join(", "),
    );
    let mut q = Query::new(sql.as_str());
    q.bind(receipt_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    let Some(projected) = project_legacy_receipt_row(row) else {
        return Ok(None);
    };
    Ok(Some(projected.hash()))
}

/// Single-PK MSSQL re-projection for a companion FOLIO (Phase 6-B).
/// Mirrors `sync_guest_registry`'s per-folio hash construction so the
/// auto-resolve sweep compares like-for-like under the CURRENT projection.
///
/// Deliberately returns `Ok(Some(<empty-folio hash>))` — never `Ok(None)` —
/// when the `Cin_no` has no companion rows left. Unlike the flat entities,
/// "no rows" here is a legitimate FOLIO STATE (most check-ins have no
/// companions), not a vanished row: iHOTEL's DELETE+reinsert edit passes
/// through it, and a companion set deleted on BOTH sides has genuinely
/// converged. Returning `None` would make that convergence unrepresentable
/// and every such row would sit open forever. The scope gate stays honest
/// because the CANONICAL arm still returns `None` when the parent check-in
/// is absent, so a bogus `legacy_pk` can never "converge" as empty/empty.
///
/// The canonical-era floor is deliberately NOT applied here, same as
/// payments: this path re-projects a folio the scan ALREADY admitted.
async fn fetch_legacy_registry_folio_hash(
    legacy_pool: &DbPool,
    cin_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_CheckIn_Other_People WHERE Cin_no = @P1",
        projection = GUEST_REGISTRY_RECONCILE_PROJECTION.join(", "),
    );
    let mut q = Query::new(sql.as_str());
    q.bind(cin_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    let mut folio = RegistryFolioProjection::empty(cin_no);
    for row in &rows {
        push_legacy_companion(&mut folio, row);
    }
    Ok(Some(guest_registry_canonical_hash(&folio)))
}

// =============================================================================
// Calendar closure arm (issue #273) — business-key resolve for
// `mirror_ht_room_calendar`
// =============================================================================
//
// ## What was broken
//
// Phase 6-C shipped the generic mirror probe over `ht_room_calendar` and
// dispatched its resolve through `mirror_probe::resolve_{pg,legacy}_hash`,
// like every sibling. For this ONE table that dispatch can never converge:
// the generic resolve is keyed on `rcal_legacy_id`, and `RoomCalendarMapper`
// deliberately NULLs that column whenever iHOTEL's `MAX(id)+1` allocator
// rebinds an id onto a different `(room, night)` slot. Nothing restores it,
// so the mirror's non-NULL id population is STRUCTURALLY below the legacy row
// count and the two aggregates are never-equal by construction. A calendar
// row reaching that arm would sit open forever no matter what an operator
// did — detection with no possible closure, which is why the probe shipped
// `observe_only`.
//
// ## What this arm changes
//
// The `<aggregate>` row now resolves on the BUSINESS key `(room, night)` —
// the same key the mapper UPSERTs on — so the comparison measures the gap
// that a re-drive can actually close, instead of an id-binding artefact that
// nothing can. Canonical is UNIQUE on `(rcal_room_id, rcal_date)`, so its
// `COUNT(*)` IS a distinct-night count; the legacy side must therefore count
// DISTINCT `(room_no, night)` pairs, because `HT_Room_Status` carries no such
// constraint and the app-side allocator can and does duplicate them.
//
// Canonical-only tiles (`rcal_legacy_id IS NULL`) are COUNTED here, unlike in
// the id-keyed probe which filters them out. That reversal is the point: a
// row whose id the mapper NULLed still occupies a real `(room, night)` slot
// that legacy also has, and excluding it would re-introduce exactly the
// structural undercount the business key exists to dodge. Note that
// `rcal_legacy_id IS NULL` cannot distinguish "app-authored tile" from
// "rebind-NULLed tile" — that indistinguishability IS the id key's defect,
// and the business key is deliberately blind to it.
//
// ## Known limitation of the aggregate shape
//
// Counts + boundaries are a NET comparison: equal-and-opposite gaps (N nights
// only in legacy, N only in canonical) would cancel and read as converged.
// That is the same trade every sibling probe makes, and the directional
// answer belongs to DETECTION (`mirror_probe::diff_mirror_rows`), not to a
// resolve arm — so it lands with the detection re-key below, not here. The
// `MIN`/`MAX` night boundaries already catch the common one-sided case where
// the missing nights sit at an edge of the window.
//
// ## What is still deferred (issue #273, deliverables 2 and 3)
//
// This is the CLOSURE half only. Detection stays `observe_only` in
// `scheduler::mirror_probe` and MUST stay there until:
//
//   1. a remediation path exists — a re-drive/backfill that actually closes a
//      genuine night gap (the live deficit survives a business-key comparison
//      today: 1546 legacy nights vs 1420 canonical at HF Hotel), and
//   2. the probe's DETECTION aggregate is re-keyed onto the business key too.
//
// (2) is not cosmetic: flipping `observe_only` while detection still counts
// id-keyed rows would record a row on the never-equal id gap that THIS arm
// then closes on the business key the very next tick — a record/resolve churn
// loop. Detection and resolution must agree on the key before the flag moves.

/// `ht_reconcile_log.table_name` of the Phase 6-C calendar mirror probe.
///
/// One literal shared by the resolve dispatches and the tests so they cannot
/// drift from the probe registry; pinned against
/// `mirror_probe::probe_for_table` by
/// `room_calendar_probe_key_matches_the_registered_mirror_probe`.
pub(crate) const ROOM_CALENDAR_PROBE_KEY: &str = "mirror_ht_room_calendar";

/// Canonical side of the calendar business-key comparison: the distinct
/// night count plus the coverage boundaries.
///
/// `MIN(rcal_date)` rides along because it IS the era floor pushed into the
/// legacy scan — the same lesson as [`PAYMENTS_ERA_FLOOR_SQL`] and the mirror
/// probe's `MIN(pk)`. A mirror that was never backfilled cannot be held
/// responsible for legacy history predating it, and reporting that history
/// builds a permanently unresolvable backlog that starves every other entity
/// out of `auto_resolve_reconcile_log`'s age-ordered 500-row batch. The floor
/// is DERIVED, never configured, and re-derived on every sweep so a floor
/// that MOVES (because the mirror finally received its missing history) is
/// picked up on the next tick instead of pinning the row to a stale boundary.
///
/// The canonical side needs no `WHERE` of its own: every canonical row is by
/// construction at or after its own `MIN(rcal_date)`.
const ROOM_CALENDAR_BUSINESS_KEY_PG_SQL: &str = "SELECT COUNT(*)::bigint AS night_count, \
     MIN(rcal_date) AS min_date, \
     MAX(rcal_date) AS max_date \
       FROM ht_room_calendar";

/// Legacy side of the same comparison, floored at the canonical `MIN`.
///
/// `floored = false` (an EMPTY canonical calendar) scans the whole legacy
/// table on purpose — with no coverage at all, "legacy has N nights and we
/// have none" IS the finding, and it lands as one bounded aggregate row.
/// Same contract as `mirror_probe::legacy_floor_filter`.
///
/// `CAST(room_date AS DATE)` normalises the legacy `datetime` (naive local
/// Thai) down to the night, which is what the canonical `DATE` column holds.
/// The boundaries come back as ISO `varchar(10)` (style 23) rather than a
/// driver-mapped date so both sides hash byte-identical `YYYY-MM-DD` text.
pub(crate) fn room_calendar_business_key_legacy_sql(floored: bool) -> String {
    let floor = if floored {
        " AND CAST(room_date AS DATE) >= CAST(@P1 AS DATE)"
    } else {
        ""
    };
    format!(
        "SELECT COUNT_BIG(*) AS night_count, \
         CONVERT(varchar(10), MIN(night_date), 23) AS min_date, \
         CONVERT(varchar(10), MAX(night_date), 23) AS max_date \
           FROM (SELECT DISTINCT room_no, CAST(room_date AS DATE) AS night_date \
                   FROM HT_Room_Status \
                  WHERE room_no IS NOT NULL AND room_date IS NOT NULL{floor}) AS nights"
    )
}

/// Hash of one side's in-era calendar business-key aggregate.
///
/// The `business_key` discriminator makes this provably incapable of
/// colliding with `mirror_probe::mirror_aggregate_hash` for the same probe
/// key — the two comparisons measure different things and must never be
/// mistaken for one another mid-migration.
///
/// Absent boundaries hash as the empty segment, so an EMPTY calendar on both
/// sides produces equal non-empty hashes and `should_auto_resolve` closes the
/// row. Absent-on-both is a real converged state for a mirror, exactly as in
/// `mirror_probe::mirror_absent_hash`; returning "no hash" instead would
/// leave such a row open forever.
pub(crate) fn room_calendar_business_key_hash(
    night_count: i64,
    min_date: Option<&str>,
    max_date: Option<&str>,
) -> String {
    sha256(&join_hash_segments(&[
        ROOM_CALENDAR_PROBE_KEY.to_string(),
        crate::scheduler::mirror_probe::MIRROR_AGGREGATE_PK.to_string(),
        "business_key".to_string(),
        night_count.to_string(),
        min_date.unwrap_or_default().to_string(),
        max_date.unwrap_or_default().to_string(),
    ]))
}

/// One PG round-trip: `(distinct night count, MIN(night), MAX(night))`.
/// Shared by both resolve halves AND the detection probe below — the legacy
/// half needs the `MIN` as its coverage floor.
async fn fetch_calendar_business_key_pg(
    pg_pool: &PgPool,
) -> Result<(i64, Option<NaiveDate>, Option<NaiveDate>), sqlx::Error> {
    sqlx::query_as::<_, (i64, Option<NaiveDate>, Option<NaiveDate>)>(
        ROOM_CALENDAR_BUSINESS_KEY_PG_SQL,
    )
    .fetch_one(pg_pool)
    .await
}

/// Canonical-side business-key hash. Never `None`: an empty calendar is a
/// real, hashable state (see [`room_calendar_business_key_hash`]).
async fn compute_room_calendar_business_key_pg_hash(
    pg_pool: &PgPool,
) -> Result<String, sqlx::Error> {
    let (night_count, min_date, max_date) = fetch_calendar_business_key_pg(pg_pool).await?;
    Ok(room_calendar_business_key_hash(
        night_count,
        min_date.map(|d| d.to_string()).as_deref(),
        max_date.map(|d| d.to_string()).as_deref(),
    ))
}

/// Raw legacy-side business-key aggregate — `(night_count, min_date,
/// max_date)` as ISO `YYYY-MM-DD` text, floored at `floor` (the canonical
/// `MIN(rcal_date)`, or `None` to scan the whole legacy table).
///
/// Extracted so the RESOLVE hash below and the DETECTION probe (issue #273,
/// [`probe_room_calendar_business_key`]) run the identical query and share
/// one interpretation of the result — the two can therefore never disagree
/// about what "converged" means, which is the property that makes flipping
/// detection safe (see the module-level "Calendar closure arm" docs).
async fn fetch_room_calendar_business_key_legacy_raw(
    legacy_pool: &DbPool,
    floor: Option<NaiveDate>,
) -> Result<(i64, Option<String>, Option<String>), Box<dyn std::error::Error + Send + Sync>> {
    let floor_text = floor.map(|d| d.to_string());
    let sql = room_calendar_business_key_legacy_sql(floor_text.is_some());

    let mut conn = legacy_pool.get().await?;
    let mut q = Query::new(sql.as_str());
    if let Some(f) = floor_text.as_deref() {
        q.bind(f);
    }
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    drop(conn);

    // An aggregate SELECT always returns exactly one row; treat a missing
    // one as "no legacy nights in era" rather than erroring the sweep row.
    let Some(row) = rows.first() else {
        return Ok((0, None, None));
    };
    Ok((
        row.try_get::<i64, _>("night_count").ok().flatten().unwrap_or(0),
        row.try_get::<&str, _>("min_date")
            .ok()
            .flatten()
            .map(str::to_string),
        row.try_get::<&str, _>("max_date")
            .ok()
            .flatten()
            .map(str::to_string),
    ))
}

/// Legacy-side business-key hash, floored at the canonical coverage
/// boundary.
///
/// Cost: ONE PG aggregate + ONE MSSQL aggregate per sweep, and only when an
/// open calendar aggregate row exists — `record_divergence`'s dedupe allows
/// at most one per site, so this cannot grow with table size.
///
/// Errors propagate to `auto_resolve_reconcile_log`, which already logs and
/// `continue`s per row: one arm's failure costs one row this tick, never the
/// cycle (the sibling error-isolation contract).
async fn compute_room_calendar_business_key_legacy_hash(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (_, floor, _) = fetch_calendar_business_key_pg(pg_pool).await?;
    let (night_count, min_date, max_date) =
        fetch_room_calendar_business_key_legacy_raw(legacy_pool, floor).await?;
    Ok(room_calendar_business_key_hash(
        night_count,
        min_date.as_deref(),
        max_date.as_deref(),
    ))
}

// =============================================================================
// Calendar DETECTION (issue #273 remainder) — business-key probe, re-keyed
// off `rcal_legacy_id`
// =============================================================================
//
// The closure arm above made the calendar's `<aggregate>` row CLOSEABLE, but
// left DETECTION untouched: until now, `mirror_probe::run_mirror_probe` kept
// measuring `ht_room_calendar` inside the generic id-keyed UNION-ALL batch
// (`rcal_legacy_id` vs `HT_Room_Status.id`) and, because that probe entry
// carried `observe_only: true`, logged the mismatch without ever calling
// `record_divergence`.
//
// Flipping `observe_only` on its own — while detection stayed id-keyed —
// would have opened a record/resolve CHURN LOOP: detection would open a row
// on the never-equal id-keyed gap (structural: `RoomCalendarMapper` NULLs
// `rcal_legacy_id` on every allocator rebind and nothing restores it), and
// the business-key resolve arm above would close that SAME row the moment
// the unrelated business-key counts happened to agree, only for detection to
// re-open it the very next tick. Two arms measuring two different things can
// never stay in agreement about "converged".
//
// So detection is re-keyed here too, and it deliberately goes through the
// SAME raw fetches the resolve arm uses
// ([`fetch_calendar_business_key_pg`], [`fetch_room_calendar_business_key_legacy_raw`])
// and the SAME [`room_calendar_business_key_hash`] — "converged" can now only
// mean one thing for this table, so a row this function opens is EXACTLY the
// row the sweep above can close, and nothing else can re-open it once it
// does. `probe_room_calendar_business_key` is called directly by
// `mirror_probe::run_mirror_probe`, in place of folding `ht_room_calendar`
// into the generic per-probe loop — the generic aggregate shape (integer
// `MAX`/`MIN` on one PK column) has no way to express "distinct
// `(room, night)` pairs, floored at a date", so this table gets its own
// tiny two-query pass instead of a UNION-ALL arm.
//
// Remediation (a re-drive/backfill that actually closes a genuine gap) is
// still NOT part of this change — a recorded row is real, honest sync-lag
// (see the "Vocabulary note" in CLAUDE.md) that will sit open, visible, and
// escalating until an operator or a future re-drive path closes it. That is
// the intended behaviour: detection catching a genuine gap and saying so is
// the whole point of re-keying it.

/// Outcome of one calendar business-key probe tick, folded into
/// [`crate::scheduler::mirror_probe::MirrorProbeOutcome`] by the caller
/// exactly like a generic probe's per-key result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RoomCalendarProbeOutcome {
    /// Both sides agree on the in-era business key — no row written.
    Converged,
    /// The business key disagreed; a divergence was written (or an
    /// already-open one deduped, per `record_divergence`'s NOT EXISTS
    /// guard) — never a per-PK row, always the `<aggregate>` sentinel.
    Diverged,
}

/// Pure decision: has the calendar business-key comparison converged, and if
/// not, which direction? `None` means converged (the two hashes are equal —
/// BYTE-IDENTICAL to `should_auto_resolve`'s primary-convergence test, see
/// `detection_and_resolution_agree_on_convergence_for_every_hash_pair`).
///
/// Extracted into a free function purely so it is unit-testable without a
/// live DB — the same treatment `mirror_probe::aggregate_divergence_kind`
/// gets. `legacy_count`/`pg_count` are needed only to pick the DIRECTION once
/// the hashes have already told us there IS a divergence; they play no part
/// in the converged/diverged decision itself, which is `legacy_hash ==
/// pg_hash` and nothing else.
///
/// Never returns `Cardinality` — same reasoning as
/// `mirror_probe::aggregate_divergence_kind`: the hourly drift digest
/// filters that kind out, so a count mismatch must be classified by
/// direction to stay alert-visible.
fn room_calendar_business_key_divergence(
    legacy_hash: &str,
    pg_hash: &str,
    legacy_count: i64,
    pg_count: i64,
) -> Option<DivergenceKind> {
    if legacy_hash == pg_hash {
        return None;
    }
    Some(if legacy_count > pg_count {
        DivergenceKind::MissingPg
    } else if pg_count > legacy_count {
        DivergenceKind::MissingMssql
    } else {
        // Counts agree but a boundary moved — money-shaped tables call
        // this `Value`; the calendar has no money column, so this reads as
        // "the same number of nights, different nights".
        DivergenceKind::Value
    })
}

/// Issue #273 — calendar DETECTION, re-keyed onto the business key the
/// closure arm above resolves on. See the section docs for why this can't
/// share the generic id-keyed UNION-ALL batch and why the churn-loop hazard
/// requires this to ship in the SAME change as the closure arm's flip.
///
/// Recording uses the same STABLE SENTINEL convention as every other
/// aggregate probe row
/// ([`crate::scheduler::mirror_probe::mirror_aggregate_sentinel`]), not the
/// live business-key hash: `should_auto_resolve` never reads the stored
/// hash for this row (closure re-runs this exact comparison fresh — see
/// `compute_current_pg_hash` / `compute_current_legacy_hash` above), so the
/// stored hash only gates `record_divergence`'s dedupe. A LIVE hash would
/// move every time the legacy table grew and mint a fresh row every tick for
/// a table with a known backlog — precisely the failure mode the sentinel
/// convention exists to avoid.
pub(crate) async fn probe_room_calendar_business_key(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<RoomCalendarProbeOutcome, Box<dyn std::error::Error + Send + Sync>> {
    let (pg_count, pg_min, pg_max) = fetch_calendar_business_key_pg(pg_pool).await?;
    let pg_min_s = pg_min.map(|d| d.to_string());
    let pg_max_s = pg_max.map(|d| d.to_string());
    let pg_hash = room_calendar_business_key_hash(pg_count, pg_min_s.as_deref(), pg_max_s.as_deref());

    let (legacy_count, legacy_min, legacy_max) =
        fetch_room_calendar_business_key_legacy_raw(legacy_pool, pg_min).await?;
    let legacy_hash =
        room_calendar_business_key_hash(legacy_count, legacy_min.as_deref(), legacy_max.as_deref());

    let Some(kind) = room_calendar_business_key_divergence(
        &legacy_hash,
        &pg_hash,
        legacy_count,
        pg_count,
    ) else {
        return Ok(RoomCalendarProbeOutcome::Converged);
    };

    let sentinel = crate::scheduler::mirror_probe::mirror_aggregate_sentinel(ROOM_CALENDAR_PROBE_KEY);
    let (mssql_hash, pg_row_hash) = match kind {
        DivergenceKind::MissingPg => (Some(sentinel.clone()), None),
        DivergenceKind::MissingMssql => (None, Some(sentinel.clone())),
        _ => (Some(sentinel.clone()), Some(sentinel.clone())),
    };

    tracing::warn!(
        probe = ROOM_CALENDAR_PROBE_KEY,
        legacy_nights = legacy_count,
        pg_nights = pg_count,
        delta = legacy_count - pg_count,
        floor = ?pg_min_s,
        kind = kind.as_str(),
        "[Sync] Mirror probe: calendar business-key divergence recorded \
         (re-keyed off rcal_legacy_id — issue #273)"
    );

    record_divergence(
        pg_pool,
        ROOM_CALENDAR_PROBE_KEY,
        crate::scheduler::mirror_probe::MIRROR_AGGREGATE_PK,
        pg_row_hash.as_deref(),
        mssql_hash.as_deref(),
        json!({
            "scope": "aggregate",
            "key_kind": "business_key",
            "legacy_table": "HT_Room_Status",
            "night_count": legacy_count,
            "min_date": legacy_min,
            "max_date": legacy_max,
        }),
        Some(json!({
            "scope": "aggregate",
            "key_kind": "business_key",
            "mirror_table": "ht_room_calendar",
            "night_count": pg_count,
            "min_date": pg_min_s,
            "max_date": pg_max_s,
        })),
        kind,
        legacy_count.min(i32::MAX as i64) as i32,
        pg_count.min(i32::MAX as i64) as i32,
    )
    .await;

    Ok(RoomCalendarProbeOutcome::Diverged)
}

/// Issue #204 (bug #2) — is the durable self-healing arm of the
/// auto-resolve sweep enabled?
///
/// Default **OFF** (ship dark). When unset / not `"true"`, the sweep stays
/// observational-only and performs ZERO canonical writes — behaviour is
/// byte-for-byte identical to its pre-#204 form. Flip to `"true"` only after
/// reception-coordinated verification (see MEMORY: flag flips are never "just
/// config").
fn reconcile_force_converge_enabled() -> bool {
    env::var("RECONCILE_FORCE_CONVERGE_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Is the `missing_pg` re-ingest arm of the auto-resolve sweep enabled?
///
/// Deliberately a SEPARATE flag from [`reconcile_force_converge_enabled`]:
/// that one is already `true` in production on `sync-hfville`, so folding a
/// brand-new canonical-write class into it would ship this ON with no
/// coordinated flip. Re-ingesting a whole booking aggregate is materially
/// more consequential than the customers/rooms value-converge that flag was
/// scoped to, so it ships dark on its own switch.
///
/// Default **OFF**. The `== "true"` comparison is strict on purpose —
/// `"TRUE"`, `"1"` and `" true"` all evaluate false, matching every other
/// feature flag in the sync path. Flip to `"true"` only after
/// reception-coordinated verification (a flag flip is never "just config").
fn reconcile_reingest_missing_pg_enabled() -> bool {
    env::var("RECONCILE_REINGEST_MISSING_PG_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Phase 6-A — is the `payments` reconcile arm enabled?
///
/// Default **OFF** (ship dark). When unset / not `"true"`, [`run_sync`]
/// never calls `sync_payments`, so the arm issues ZERO MSSQL and ZERO PG
/// queries and `ht_reconcile_log` can never gain a `payments` row —
/// behaviour is byte-for-byte identical to before the arm existed. The
/// resolve dispatches and the ack table are inert without detection.
///
/// Rollout is Ville-first → 48h soak → HF Hotel, in an announced window.
/// The first enabled tick re-hashes every IN-ERA receipt with a
/// `Receipt_ref`, so a one-time find is expected — but only a small one:
/// [`PAYMENTS_ERA_FLOOR_SQL`] keeps the pre-mirror history (>20k receipts
/// at HF Hotel) out of scope entirely, because those rows could never
/// converge and would jam the whole sweep. Pre-set
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE>` accordingly.
///
/// Be honest about what lands: `payments` is in NEITHER self-heal list, so
/// a `missing_pg` find here does NOT age out on its own — it stays open
/// until an operator acts, and the >72h escalation tier will eventually
/// fire on it. Treat every one as a real dropped receipt ingest.
///
/// The `== "true"` comparison is strict on purpose, matching every other
/// feature flag in the sync path. A flag flip is never "just config".
fn reconcile_payments_arm_enabled() -> bool {
    env::var("RECONCILE_PAYMENTS_ARM_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Phase 6-B — is the `guest_registry` (companion folio) reconcile arm
/// enabled?
///
/// Default **OFF** (ship dark). When unset / not `"true"`, [`run_sync`]
/// never calls `sync_guest_registry`, so the arm issues ZERO MSSQL and ZERO
/// PG queries and `ht_reconcile_log` can never gain a `guest_registry` row —
/// behaviour is byte-for-byte identical to before the arm existed. The
/// resolve dispatches and the ack table are inert without detection.
///
/// Rollout is Ville-first → 48h soak → HF Hotel, in an announced window.
/// The first enabled tick hashes every IN-ERA folio, so a one-time find is
/// expected — a small one, because [`GUEST_REGISTRY_ERA_FLOOR_SQL`] keeps
/// the pre-mirror history out of scope. Live 2026-07-28: HF Hotel 830 in-era
/// legacy folios vs 818 canonical (≈12 finds); HF Ville 574 vs 545 (≈29).
/// Unfloored those would have been ~19.6k and ~1.6k folios that can never
/// converge.
///
/// Be honest about what lands: `guest_registry` is in NEITHER self-heal
/// list, so a find does NOT age out on its own — it stays open until an
/// operator acts, and the >72h escalation tier will eventually fire on it.
/// Every one is a real TM.30 companion-registry disagreement.
///
/// The `== "true"` comparison is strict on purpose, matching every other
/// feature flag in the sync path. A flag flip is never "just config".
fn reconcile_guest_registry_arm_enabled() -> bool {
    env::var("RECONCILE_GUEST_REGISTRY_ARM_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Phase 6-C — is the generic mirror-table probe enabled?
///
/// Default **OFF** (ship dark). When unset / not `"true"`, [`run_sync`]
/// never calls [`crate::scheduler::mirror_probe::run_mirror_probe`], so the
/// probe issues ZERO MSSQL and ZERO PG queries and `ht_reconcile_log` can
/// never gain a `mirror_*` row — behaviour is byte-for-byte identical to
/// before the probe existed. The resolve dispatches are inert without
/// detection, and the probe has no ack table of its own (nothing to seed).
///
/// Rollout is Ville-first → 48h soak → HF Hotel, in an announced window.
/// Live read-only counts 2026-07-28 say the first enabled tick is quiet on
/// 8 of the 9 probes at BOTH sites once the `MIN(mirror pk)` coverage floor
/// is applied — including `HT_Rooms_Cancel`, whose mirror was never
/// bootstrap-snapshotted (315 legacy rows → 13 in-era, matching the 13
/// mirrored).
///
/// The 9th, `ht_room_calendar`, is NOT quiet — as of issue #273 (remainder)
/// its detection is re-keyed onto the same business key
/// (`probe_room_calendar_business_key`) the closure arm resolves on, and
/// `observe_only` is `false`. The gap is genuine and, at last measurement
/// (2026-07-28), survives the business key too: HF Hotel counted 1546
/// legacy nights vs 1420 canonical (a `missing_pg` aggregate row). The
/// id-keyed figures quoted historically for this table (1507 vs 1298 at HF
/// Hotel, 1302 vs 1071 at Ville) are a DIFFERENT comparison and must not be
/// read as the business-key gap — Ville's business-key gap has not been
/// independently measured; re-check live counts before the flip rather than
/// assuming it. Expect the first enabled tick to open exactly ONE aggregate
/// `mirror_ht_room_calendar` row per site with an open business-key gap (or
/// zero if Ville's business key happens to be converged), staying open —
/// same as `guest_registry` / `payment_ledger_probe` — until a future
/// re-drive path closes it or the >72h `:bangbang:` escalation tier fires.
/// Pre-set `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE>` or flip in an
/// announced window with that expectation communicated, not a "quiet
/// ledger" one.
///
/// The `== "true"` comparison is strict on purpose, matching every other
/// feature flag in the sync path. A flag flip is never "just config".
fn reconcile_mirror_probe_enabled() -> bool {
    env::var("RECONCILE_MIRROR_PROBE_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Phase 6-D — is the per-folio payment-ledger probe enabled?
///
/// Default **OFF** (ship dark). When unset / not `"true"`, [`run_sync`]
/// never calls
/// [`crate::scheduler::payment_ledger_probe::run_payment_ledger_probe`], so
/// the probe issues ZERO MSSQL and ZERO PG queries and `ht_reconcile_log`
/// can never gain a `payment_ledger_probe` row — behaviour is byte-for-byte
/// identical to before the probe existed. The resolve dispatches are inert
/// without detection, and the probe has no ack table of its own (nothing to
/// seed).
///
/// Rollout is Ville-first → 48h soak → HF Hotel, in an announced window.
/// Live read-only counts 2026-07-28 say what to expect, once the
/// `MIN(ledger_legacy_id)` coverage floor is applied: **HF Ville is EXACTLY
/// converged** (1,016 in-era folios, identical line counts, itemized amounts
/// AND receipt-deduped tenders on both sides), and **HF Hotel opens exactly
/// 19 rows**, all `missing_pg`, all contiguous at the era boundary
/// (`CH26-004952`…`CH26-004971`, minus `CH26-004960`) — folios whose
/// payments the Track J7e backfill never reached, i.e. money
/// `round_report` under-counts today. Zero `value` and zero
/// `missing_mssql` at either site.
///
/// Those 19 are CLOSEABLE, which is why this arm records rather than merely
/// observes (contrast `mirror_ht_room_calendar`): re-drive them with
/// `cargo run --release --bin backfill_payment_ledger` and the next
/// auto-resolve sweep sees equal hashes. But be honest about the interim —
/// `payment_ledger_probe` is in NEITHER self-heal list, so nothing closes
/// them on its own and the >72h `:bangbang:` escalation tier WILL fire if
/// they are left. Pre-set `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE>`
/// or flip in an announced window.
///
/// The `== "true"` comparison is strict on purpose, matching every other
/// feature flag in the sync path. A flag flip is never "just config".
fn reconcile_payment_ledger_probe_enabled() -> bool {
    env::var("RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Issue #204 (bug #2) — minimum age (seconds) an unresolved
/// `ht_reconcile_log` row must have before the force-converge arm will touch
/// it. A younger row is likely just waiting on an in-flight CT event, so we
/// leave it for the normal converge-on-its-own path. 3600s ≈ 4 sweep ticks
/// at the default 15-min cadence, i.e. only rows that have resisted several
/// observational sweeps qualify ("seen across 2+ sweeps").
const FORCE_CONVERGE_MIN_AGE_SECS: f64 = 3600.0;

/// Issue #204 (bug #2) — re-fetch a single `HT_Customers` base row by its
/// business key (`Cust_no`, which is what the reconcile loop stores as
/// `ht_reconcile_log.legacy_pk` for customers). The projection is the full
/// CT eager-fetch column set so the returned `tiberius::Row` carries every
/// column `CustomerMapper`'s `project` / `apply_upsert` reads. `Ok(None)`
/// when the legacy row no longer exists.
async fn fetch_legacy_customer_base_row(
    legacy_pool: &DbPool,
    cust_no: &str,
) -> Result<Option<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Customers WHERE Cust_no = @P1",
        projection = crate::sync::mappers::customer::EAGER_FETCH_COLUMNS.join(", "),
    );
    let mut q = Query::new(sql.as_str());
    q.bind(cust_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    Ok(rows.into_iter().next())
}

/// Issue #204 (bug #2) — re-fetch a single `HT_Rooms` base row by `Room_no`
/// (the reconcile loop's `legacy_pk` for rooms). The column set mirrors
/// `room.rs`'s `ROOMS_SELECT_COLS` minus the CT-JOIN `t.` alias and MUST
/// cover every field `RoomMasterMapper::project_room` reads — `id` is the CT
/// PK but is also a real `HT_Rooms` column, re-projected here. `Ok(None)`
/// when the legacy row no longer exists.
async fn fetch_legacy_room_base_row(
    legacy_pool: &DbPool,
    room_no: &str,
) -> Result<Option<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = "SELECT id, Room_no, Room_Type, Room_Clean, Room_Use, \
               Room_Manternace, Room_Details, Room_Use_Count, Room_X, Room_Y, \
               Room_Group, Room_Power_OPEN, Room_Power_CLOSE, Room_Power_STATUS, \
               Room_Polity FROM HT_Rooms WHERE Room_no = @P1"
        .to_string();
    let mut q = Query::new(sql.as_str());
    q.bind(room_no);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    Ok(rows.into_iter().next())
}

/// Issue #204 (bug #2) — durable self-heal for a single `customers` / `rooms`
/// reconcile row that has resisted observational convergence.
///
/// Re-fetches the CURRENT legacy base row by its PK and re-projects it
/// through the EXISTING CT upsert path — the very same `mapper.apply(...)`
/// the watcher uses — INSIDE a fresh PG transaction. Canonical fields are
/// therefore written by the mapper, never by hand here. The mapper's I/U
/// branch is an UPSERT that updates the existing canonical row in place
/// (verified: `customer::apply_upsert` / `room::apply_room_upsert`), and its
/// own idempotency guard collapses a no-op to no write — so this is safe to
/// re-drive.
///
/// The returned `DomainEvent` is intentionally DROPPED: the value we just
/// wrote came FROM legacy, so there is nothing to write back — emitting an
/// outbox event would only produce a converging no-op echo against MSSQL.
/// The sweep is a silent backstop; a genuine subsequent CT edit re-emits the
/// normal `CustomerModified` / room event.
///
/// Returns a [`ForceConvergeOutcome`], NOT a bool — and that distinction is
/// the whole point (2026-07-28). The mapper contract makes `Ok(None)`
/// ambiguous: it is EITHER an idempotency-gate skip (the mapper decided
/// nothing changed) OR a real write that produces no domain event. The
/// previous bool collapsed both onto `Ok(true)` = "attempted and committed",
/// so a gate skip — the very blind spot that let the divergence become
/// invisible in the first place — was reported to the sweep as a successful
/// repair. The sweep then logged "repaired" and, next tick, "still not
/// converged", forever: one blind spot disabling detection AND self-heal at
/// once while reporting success. [`ForceConvergeOutcome::MapperNoop`] keeps
/// the ambiguity honest at this boundary; the sweep resolves it with
/// evidence (did the canonical hash actually move?) via
/// [`classify_force_converge`].
///
/// `SourceRowAbsent` / `UnsupportedTable` are the old `Ok(false)` cases (the
/// legacy row no longer exists, or the table is outside the supported set)
/// so there is nothing to project from.
///
/// `op` is the `ChangeOp` handed to the mapper. The value-drift caller passes
/// `ChangeOp::Update` (the canonical row exists, we're correcting its
/// fields); the `missing_pg` re-ingest caller passes `ChangeOp::Insert` (no
/// canonical row at all). Both land in the same mapper UPSERT branch — the op
/// only shapes the `DomainEvent`, which we drop — but the honest value keeps
/// the intent readable at the call site.
///
/// **Bookings (2026-07-27, `missing_pg` re-ingest).** The bookings arm
/// re-drives the CT watcher's own aggregate path: `load_booking_aggregate`
/// re-reads `HT_Book_H` + `HT_Book_Ds` + `HT_Book_Date` for the `book_no`,
/// then `apply_booking_aggregate` projects it into canonical in a fresh PG
/// tx. The legacy pool is passed through so the mapper's customer
/// eager-mirror fallback works (an unresolvable customer FK errors rather
/// than silently skipping — the 2026-06-03 silent-drop class). Still a
/// PG-write-only path: the legacy side is read exclusively.
async fn force_converge_reconcile_row(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
    op: ChangeOp,
) -> Result<ForceConvergeOutcome, Box<dyn std::error::Error + Send + Sync>> {
    match table_name {
        "customers" => {
            let Some(row) = fetch_legacy_customer_base_row(legacy_pool, legacy_pk).await? else {
                return Ok(ForceConvergeOutcome::SourceRowAbsent);
            };
            let mut tx = pg_pool.begin().await?;
            // apply runs the full UPSERT + the mapper's idempotency check, so
            // it inserts when canonical is absent and updates in place when
            // it is not.
            let evt = CustomerMapper
                .apply(&mut tx, op, Some(&row as &dyn MappableRow))
                .await?;
            tx.commit().await?;
            Ok(ForceConvergeOutcome::from_mapper_event(evt.as_ref()))
        }
        "rooms" => {
            let Some(row) = fetch_legacy_room_base_row(legacy_pool, legacy_pk).await? else {
                return Ok(ForceConvergeOutcome::SourceRowAbsent);
            };
            let mut tx = pg_pool.begin().await?;
            let evt = RoomMasterMapper
                .apply(&mut tx, op, Some(&row as &dyn MappableRow))
                .await?;
            tx.commit().await?;
            Ok(ForceConvergeOutcome::from_mapper_event(evt.as_ref()))
        }
        "bookings" => {
            // `ht_reconcile_log.legacy_pk` for bookings is the composite
            // "{book_no}|{room_type}"; the aggregate is keyed on `book_no`
            // alone, so several reconcile rows for one booking all re-drive
            // the SAME aggregate. That is safe: `apply_booking_aggregate` is
            // idempotent and returns `Ok(None)` once canonical already
            // mirrors legacy, so the 2nd and 3rd row of a multi-room-type
            // booking are no-ops that still get their convergence re-tested.
            let (book_no, _room_type_key) = parse_booking_legacy_pk(legacy_pk);
            let aggregate =
                crate::sync::parent_loader::load_booking_aggregate(legacy_pool, book_no).await?;
            if !aggregate.is_present() {
                return Ok(ForceConvergeOutcome::SourceRowAbsent);
            }
            let mut tx = pg_pool.begin().await?;
            let evt = crate::sync::mappers::apply_booking_aggregate(
                &mut tx,
                Some(legacy_pool),
                &aggregate,
                book_no,
            )
            .await?;
            tx.commit().await?;
            Ok(ForceConvergeOutcome::from_mapper_event(evt.as_ref()))
        }
        // checkins are multi-row aggregates whose self-heal is still out of
        // scope — leave them to the normal paths / operator review.
        _ => Ok(ForceConvergeOutcome::UnsupportedTable),
    }
}

/// What one [`force_converge_reconcile_row`] attempt actually did.
///
/// The two "the mapper ran" variants are deliberately NOT collapsed:
///
/// * `Wrote` ⇔ the mapper returned `Ok(Some(event))` — canonical state
///   definitely changed (the event itself is still dropped; see the
///   `force_converge_reconcile_row` doc).
/// * `MapperNoop` ⇔ `Ok(None)`, which
///   [`crate::sync::mapper::MssqlChangeMapper::apply`] defines as "nothing to
///   publish AND nothing left to do". That covers BOTH an idempotency-gate
///   skip (`customer::apply_upsert` returns before the UPSERT when every
///   compared column already matches) AND legitimate writes that produce no
///   event (`room::apply_room_upsert` always UPSERTs but only emits on a
///   `room_clean` flip; cancel / soft-delete paths). So `MapperNoop` on its
///   own is NOT evidence of a gate skip — see [`classify_force_converge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ForceConvergeOutcome {
    /// Mapper returned `Ok(Some(event))` — canonical was written.
    Wrote,
    /// Mapper returned `Ok(None)`. Ambiguous by the mapper contract.
    MapperNoop,
    /// The legacy row (or the booking aggregate header) no longer exists,
    /// so there is nothing to re-project from.
    SourceRowAbsent,
    /// `table_name` is outside the set this self-heal supports.
    UnsupportedTable,
}

impl ForceConvergeOutcome {
    /// Classify what a mapper's `apply` returned. Kept as a constructor so
    /// every arm of `force_converge_reconcile_row` maps `Ok(Some(_))` /
    /// `Ok(None)` the same way.
    fn from_mapper_event(evt: Option<&DomainEvent>) -> Self {
        match evt {
            Some(_) => Self::Wrote,
            None => Self::MapperNoop,
        }
    }

    /// True when the mapper actually ran to completion and the transaction
    /// committed (whether or not it wrote). Both shapes get the convergence
    /// re-test; only the absent/unsupported shapes skip it — matching the
    /// pre-2026-07-28 `Ok(true)` / `Ok(false)` split exactly.
    fn mapper_ran(self) -> bool {
        matches!(self, Self::Wrote | Self::MapperNoop)
    }
}

/// Pure tripwire decision for both self-heal arms: was this "successful
/// repair" actually a MAPPER GATE SKIP that wrote nothing and fixed nothing?
///
/// Returns `true` ⇔ ALL THREE hold:
/// 1. `outcome == MapperNoop` — the mapper returned `Ok(None)`;
/// 2. `!pg_hash_moved` — the reprojected canonical hash is IDENTICAL to the
///    one measured before the apply, i.e. nothing moved;
/// 3. `!converged` — the row is still unconverged.
///
/// Condition 2 is what makes this correct rather than a naive
/// "`Ok(None)` ⇒ gate-skipped" test. Legitimate event-less writes (rooms'
/// non-clean-flip UPSERT, cancel paths) also return `Ok(None)`, but they
/// move the canonical hash, so they never trip this. What remains is the
/// pathological shape: the mapper decided "already identical" while the
/// reconcile hash still says "different" — a **gate ⊂ hash violation**,
/// where the mapper's idempotency comparison covers strictly fewer fields
/// than the reconcile projection hashes. Such a row can never self-heal and
/// can never be detected by the CT watcher either; the sweep would otherwise
/// log a successful repair every tick forever.
///
/// (A `Wrote` that leaves the row unconverged is a DIFFERENT class — the
/// mapper wrote but the two projections still disagree — and is left to the
/// existing "still diverge, leaving row open for operator review" warn.)
fn classify_force_converge(
    outcome: ForceConvergeOutcome,
    pg_hash_moved: bool,
    converged: bool,
) -> bool {
    matches!(outcome, ForceConvergeOutcome::MapperNoop) && !pg_hash_moved && !converged
}

/// Tables the #204 value-drift force-converge arm will repair. Single-PK
/// mappers whose `apply` is safe to re-drive idempotently.
///
/// **`payments` is deliberately absent** (Phase 6-A): detection ships
/// first and must soak before any self-heal is wired. `payments` also has
/// nothing to gain from the gate-skip machinery — `apply_receipt_upsert`
/// is a lookup-then-unconditional UPDATE, so it cannot be gate-blinded —
/// and one of its divergence shapes (canonical-only void, see
/// [`payment_canonical_hash`]) is a genuine cross-app disagreement that
/// re-driving the legacy row would silently ERASE rather than repair.
/// Adding it here is a separate, coordinated decision (plan 6-D).
///
/// **`guest_registry` is deliberately absent too** (Phase 6-B), and its
/// self-heal would be a bigger step than payments': the folio arm's repair
/// is not a single-row re-drive at all — it would have to DELETE canonical
/// companion rows that legacy no longer has, i.e. destroy TM.30 registry
/// state from a sweep. Detection soaks first; plan 6-D decides the rest.
const FORCE_CONVERGE_VALUE_DRIFT_TABLES: &[&str] = &["customers", "rooms"];

/// Tables the `missing_pg` re-ingest arm will repair. Customers first-class
/// (flat single-PK mapper), bookings via the aggregate loader. `rooms` is
/// excluded because a room absent from canonical is a provisioning gap, not
/// a dropped CT event; `checkins` is excluded until its aggregate re-ingest
/// has been verified the same way.
const REINGEST_MISSING_PG_TABLES: &[&str] = &["customers", "bookings"];

/// Pure gate for the #204 value-drift force-converge arm — BOTH hashes
/// present (a genuine value drift, not a missing-side case), an eligible
/// table, past the min-age threshold, flag on.
///
/// Extracted so the "value drift still routes to the existing arm,
/// unchanged" invariant is pinned by a unit test rather than by reading the
/// sweep's control flow.
fn force_converge_value_drift_eligible(
    table_name: &str,
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
    age_secs: f64,
    enabled: bool,
) -> bool {
    enabled
        && FORCE_CONVERGE_VALUE_DRIFT_TABLES.contains(&table_name)
        && current_legacy_hash.is_some()
        && current_pg_hash.is_some()
        && age_secs >= FORCE_CONVERGE_MIN_AGE_SECS
}

/// Pure gate for the `missing_pg` re-ingest arm (2026-07-27).
///
/// Repairs a DROPPED INGEST: legacy still has the row, canonical never got
/// it, and CT's 2-day retention has long since aged the event out so the
/// watcher can never redeliver it. Live shape that motivated this: customer
/// `C2413` + bookings `R002066|110` / `|112` / `|217`, unconverged for 16
/// days after a 2026-07-11 cross-table watermark clobber on HF Ville.
///
/// Conditions, ALL required:
/// * flag on (`RECONCILE_REINGEST_MISSING_PG_ENABLED`, default off);
/// * `table_name` in [`REINGEST_MISSING_PG_TABLES`];
/// * legacy hash PRESENT — a VANISHED legacy row must never trip this arm.
///   That is the genuine-anomaly case [`should_auto_resolve`] deliberately
///   protects and `rooms_dispatch_missing_legacy_row_does_not_auto_resolve`
///   pins; re-ingesting nothing would be meaningless and closing the row
///   would hide a real deletion;
/// * canonical hash ABSENT — this arm only inserts what is missing; a value
///   drift belongs to [`force_converge_value_drift_eligible`];
/// * past [`FORCE_CONVERGE_MIN_AGE_SECS`] so an in-flight CT event isn't
///   raced.
fn reingest_missing_pg_eligible(
    table_name: &str,
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
    age_secs: f64,
    enabled: bool,
) -> bool {
    enabled
        && REINGEST_MISSING_PG_TABLES.contains(&table_name)
        && current_legacy_hash.is_some_and(|h| !h.is_empty())
        && current_pg_hash.is_none()
        && age_secs >= FORCE_CONVERGE_MIN_AGE_SECS
}

/// One unresolved `ht_reconcile_log` candidate as fetched by the
/// auto-resolve sweep: `(id, table_name, legacy_pk, mssql_hash, age_secs)`.
type ReconcileCandidate = (i64, String, String, Option<String>, f64);

/// FK-dependency rank for the auto-resolve sweep's candidate ordering.
/// Lower runs first, so a parent is always re-ingested before anything that
/// points at it: customers → rooms → bookings → checkins →
/// payments / guest_registry.
///
/// **This ordering is load-bearing, not cosmetic.** `apply_booking_aggregate`
/// needs the booking's customer to exist in canonical (it eager-mirrors on a
/// miss and ERRORS if even that fails); check-ins point at rooms and
/// bookings; a payment points at its check-in (`ht_payments.pay_cin_id`, and
/// `apply_receipt_upsert` ERRORS on an unresolvable parent). Healing a
/// dependent before its parent within one sweep pass turns a repairable row
/// into an error.
fn reconcile_table_fk_rank(table_name: &str) -> u8 {
    match table_name {
        "customers" => 0,
        "rooms" => 1,
        "bookings" => 2,
        "checkins" => 3,
        "payments" => 4,
        // Phase 6-B. A companion folio hangs off its check-in
        // (`ht_guest_registry.guest_cin_id`, and the CT mapper ERRORS on an
        // unresolvable parent), so it must never be healed before check-ins.
        // Sibling of `payments`; the two do not depend on each other.
        "guest_registry" => 5,
        // Phase 6-C mirror probes. They are OBSERVED, never healed — no
        // self-heal list contains them — so their rank only decides sweep
        // ordering. Last, so no probe row can delay a repairable entity
        // row's turn in the 500-row batch.
        t if crate::scheduler::mirror_probe::probe_for_table(t).is_some() => 6,
        // Phase 6-D payment-ledger probe. OBSERVED, never healed (it is in
        // neither self-heal list), so its rank only decides sweep ordering —
        // last, alongside the 6-C probes, so no probe row can delay a
        // repairable entity row's turn in the 500-row batch.
        t if crate::scheduler::payment_ledger_probe::is_payment_ledger_probe(t) => 6,
        other => {
            // A resolvable entity that falls through here is unranked, so
            // it sorts after everything and its FK parents lose their
            // guaranteed head start. Same class of omission the two
            // resolve dispatches guard.
            debug_assert!(
                !RECONCILE_RESOLVABLE_TABLES.contains(&other),
                "FK rank missing for {other} in reconcile_table_fk_rank — \
                 the entity is listed as resolvable but falls through to the \
                 wildcard, so the sweep may heal it before its parents"
            );
            7
        }
    }
}

/// Order the sweep's candidate batch: FK parents first, then oldest first,
/// then `id` as a deterministic tie-break.
///
/// "Oldest first" is expressed as DESCENDING `age_secs` because `age_secs`
/// is `NOW() - detected_at` — a larger age IS an earlier `detected_at`.
///
/// This deliberately does NOT mirror the SQL `ORDER BY` in
/// [`auto_resolve_reconcile_log`], and the split is the point. SQL selects
/// the batch purely by age, which is what fixes the unordered-`LIMIT 500`
/// starvation hazard (with a backlog >500 rows PG returned an arbitrary
/// subset, so a row could age past 4h forever without ever being retested).
/// The FK-rank leg lives here, applied AFTER the fetch, so parents are
/// processed before dependents within the batch without letting a large
/// parent backlog starve `checkins` out of the selection entirely. Keeping
/// FK rank out of the query is intentional — do not "restore" it there.
fn sort_reconcile_candidates(rows: &mut [ReconcileCandidate]) {
    rows.sort_by(|a, b| {
        reconcile_table_fk_rank(&a.1)
            .cmp(&reconcile_table_fk_rank(&b.1))
            .then_with(|| b.4.total_cmp(&a.4))
            .then_with(|| a.0.cmp(&b.0))
    });
}

/// Track D / T7 follow-up — auto-resolve sweep. Walks unresolved
/// `ht_reconcile_log` rows whose `divergence_kind` was classified by
/// the post-migration-032 reconcile loop, re-projects BOTH the legacy
/// MSSQL row and the canonical PG row under the CURRENT projections,
/// and marks the row resolved when the two fresh hashes converge.
/// Returns the number of rows resolved.
///
/// Rationale: alerts should fire only on drift that still persists.
/// Re-projecting the legacy side (rather than trusting the row's
/// stored `mssql_hash`) is what makes this sweep robust to changes
/// in a `*_RECONCILE_PROJECTION` constant — a fix that renames or
/// re-sources a hashed column would otherwise leave every pre-fix
/// row permanently unresolvable, since the stored hash was computed
/// over a now-defunct field set.
///
/// Bounded to 500 rows per tick so a backlog can't stall the
/// reconcile loop. Best-effort per row — a single MSSQL or PG
/// failure logs and continues to the next.
///
/// **No per-table fairness — a known, load-bearing property.** The batch is
/// selected by `detected_at` alone. An entity that accumulates a large
/// backlog of rows that can NEVER close (a divergence kind with no self-heal
/// arm) would therefore occupy every subsequent 500-row batch and starve all
/// other tables out of the sweep completely. That is why a new reconcile arm
/// must not be able to manufacture permanently-unresolvable rows in bulk —
/// see [`PAYMENTS_ERA_FLOOR_SQL`] for the payments case that made this
/// concrete, and the live numbers behind it.
///
/// **Issue #204 (bug #2) — durable self-healing arm (ship dark).** By
/// default this sweep is observational-only: it resolves a row ONLY when
/// legacy and canonical ALREADY agree, so a durable value drift that will
/// never get a CT event (migration-born, or a hand-edit) is observed
/// forever but never repaired. When `RECONCILE_FORCE_CONVERGE_ENABLED` is
/// `"true"`, a long-lived `customers` / `rooms` value-drift row is repaired
/// by re-projecting the CURRENT legacy row through the EXISTING CT upsert
/// path (`force_converge_reconcile_row`) before the convergence re-test.
/// With the flag OFF the behaviour is identical to the pre-#204 sweep — no
/// canonical writes.
///
/// **`missing_pg` re-ingest arm (2026-07-27, ship dark).** A second,
/// separately-flagged arm (`RECONCILE_REINGEST_MISSING_PG_ENABLED`, default
/// off) repairs the DROPPED-INGEST shape: legacy row present, canonical row
/// absent, past the min-age threshold. Those rows have no automated path to
/// closure — the observational test needs both hashes, and CT's 2-day
/// retention means the watcher can never redeliver the event — so they sit
/// unconverged forever while the >4h level alert refires every 24h. The arm
/// re-runs the NORMAL mapper for the key and closes the row only if the two
/// sides then converge. A VANISHED legacy row never trips it: that is the
/// genuine anomaly `should_auto_resolve` protects. See
/// [`reingest_missing_pg_eligible`].
async fn auto_resolve_reconcile_log(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    site_id: &str,
) -> Result<usize, sqlx::Error> {
    // `age_secs` (issue #204 bug #2) gates the durable force-converge arm —
    // only rows that have resisted convergence for a while qualify.
    //
    // The `ORDER BY` is strictly age-based, and that is deliberate. Before
    // this it was absent entirely, so `LIMIT 500` returned an arbitrary
    // (heap-order, therefore stable) subset — rows outside it were never
    // retested and could age past the 4h level alert forever. Oldest-first
    // makes the cap a fair queue instead of a lottery.
    //
    // The FK guarantee (parents before dependents — a booking whose customer
    // is still missing hits `Ok(None)` in the mapper) is supplied in Rust by
    // [`sort_reconcile_candidates`] over the fetched batch, NOT here. Ranking
    // by table in SQL would reintroduce starvation from the other side: with
    // >500 unresolved `customers`+`rooms`+`bookings` rows, `checkins` would
    // never be swept at all. Sorting after the fetch gets both properties —
    // fair selection, correct ordering within the batch. Worst case, a
    // dependent lands in a batch whose parent did not; it stays open and the
    // next tick retries it, which is the same self-correcting path the
    // FK-defer class already relies on.
    let mut rows = sqlx::query_as::<_, ReconcileCandidate>(
        "SELECT id, table_name, legacy_pk, mssql_hash, \
                EXTRACT(EPOCH FROM (NOW() - detected_at))::float8 AS age_secs \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
          ORDER BY detected_at ASC, \
                   id ASC \
          LIMIT 500",
    )
    .fetch_all(pg_pool)
    .await?;
    // Belt-and-braces: re-apply the same order in Rust so the FK guarantee
    // survives a future edit to the query above, and so it is unit-testable.
    sort_reconcile_candidates(&mut rows);

    // Read the self-heal flags ONCE per sweep. Both default OFF ⇒ the sweep
    // stays observational-only and never writes canonical state.
    let force_converge_enabled = reconcile_force_converge_enabled();
    let reingest_missing_pg_enabled = reconcile_reingest_missing_pg_enabled();

    let mut resolved = 0usize;
    for (id, table_name, legacy_pk, recorded_mssql_hash, age_secs) in rows {
        let current_legacy_hash =
            match compute_current_legacy_hash(legacy_pool, pg_pool, &table_name, &legacy_pk).await {
                Ok(opt) => opt,
                Err(e) => {
                    tracing::warn!(
                        site = %site_id,
                        id,
                        table_name = %table_name,
                        legacy_pk = %legacy_pk,
                        error = %e,
                        "[Sync] Auto-resolve sweep: failed to re-fetch legacy hash, skipping row"
                    );
                    continue;
                }
            };

        let current_pg_hash = match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await
        {
            Ok(opt) => opt,
            Err(e) => {
                tracing::warn!(
                    site = %site_id,
                    id,
                    table_name = %table_name,
                    legacy_pk = %legacy_pk,
                    error = %e,
                    "[Sync] Auto-resolve sweep: failed to re-fetch canonical hash, skipping row"
                );
                continue;
            }
        };

        if !should_auto_resolve(
            &table_name,
            current_legacy_hash.as_deref(),
            current_pg_hash.as_deref(),
            recorded_mssql_hash.as_deref(),
        ) {
            // ---------------------------------------------------------------
            // Issue #204 (bug #2) — durable self-healing arm.
            //
            // The convergence test above is observational-only: it resolves a
            // row ONLY when legacy and canonical ALREADY agree. A value drift
            // that will never receive a CT event (migration-born, or a
            // hand-edit on one side) is therefore observed forever but never
            // repaired. When the flag is ON and the row is a long-lived
            // `customers` / `rooms` *value* drift (both sides present, just
            // unequal), re-project the CURRENT legacy row through the EXISTING
            // CT upsert path, then re-hash; if it now converges, fall through
            // to mark it resolved.
            //
            // Conservative guards:
            //   * flag OFF → branch never taken; the `else` below logs and
            //     leaves the row open exactly as before — ZERO canonical writes.
            //   * customers/rooms only — single-PK mappers whose apply is safe
            //     to re-drive idempotently. bookings/checkins are multi-row
            //     aggregates, out of scope for #204.
            //   * row older than FORCE_CONVERGE_MIN_AGE_SECS so we don't race
            //     an in-flight CT event for a fresh divergence.
            //   * BOTH hashes present — a genuine value drift, not a
            //     missing_pg / missing_mssql case (those stay open for the
            //     normal paths / operator review).
            if force_converge_value_drift_eligible(
                &table_name,
                current_legacy_hash.as_deref(),
                current_pg_hash.as_deref(),
                age_secs,
                force_converge_enabled,
            ) {
                match force_converge_reconcile_row(
                    legacy_pool,
                    pg_pool,
                    &table_name,
                    &legacy_pk,
                    // The canonical row already exists — this is a value
                    // drift, not a miss.
                    ChangeOp::Update,
                )
                .await
                {
                    Ok(outcome) if outcome.mapper_ran() => {
                        // The mapper re-projected the current legacy row into
                        // canonical. The legacy hash is unchanged (we projected
                        // FROM it), so only the canonical hash can have moved —
                        // re-fetch it and re-test convergence.
                        let reprojected_pg_hash =
                            match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await {
                                Ok(opt) => opt,
                                Err(e) => {
                                    tracing::warn!(
                                        site = %site_id,
                                        id,
                                        table_name = %table_name,
                                        legacy_pk = %legacy_pk,
                                        error = %e,
                                        "[Sync] Force-converge (#204): re-hash of canonical \
                                         failed after re-projection, leaving row open"
                                    );
                                    continue;
                                }
                            };
                        // Did the apply actually move canonical? This is the
                        // evidence that separates a mapper gate skip from a
                        // legitimate event-less write — see
                        // [`classify_force_converge`].
                        let pg_hash_moved =
                            reprojected_pg_hash.as_deref() != current_pg_hash.as_deref();
                        let converged = should_auto_resolve(
                            &table_name,
                            current_legacy_hash.as_deref(),
                            reprojected_pg_hash.as_deref(),
                            recorded_mssql_hash.as_deref(),
                        );
                        let gate_skip = classify_force_converge(outcome, pg_hash_moved, converged);
                        if converged {
                            tracing::info!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                "[Sync] Force-converge (#204): re-projected current legacy \
                                 row into canonical; hashes now converge — marking resolved"
                            );
                            // Fall through (no `continue`) to the resolved UPDATE.
                        } else if gate_skip {
                            // Live tripwire for a gate ⊂ hash violation: the
                            // mapper's idempotency check said "identical" while
                            // the reconcile projection still says "different",
                            // so this row can NEVER self-heal and the watcher
                            // will never see a CT event for it either. Emitted
                            // INSTEAD of the generic warn below (one line per
                            // row per tick — no alert storm), with a stable
                            // event name for `/diagnose-alert` to grep —
                            // [`EV_FORCE_CONVERGE_GATE_SKIP`], registered in
                            // [`KNOWN_SCHEDULER_EVENT_NAMES`] (issue #267).
                            tracing::warn!(
                                event_name = EV_FORCE_CONVERGE_GATE_SKIP,
                                arm = "value_drift",
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                age_secs,
                                current_legacy_hash = ?current_legacy_hash,
                                current_pg_hash = ?current_pg_hash,
                                "[Sync] Force-converge (#204): mapper skipped the write \
                                 (Ok(None)) and canonical did not move, yet the row is \
                                 still unconverged — the mapper's idempotency gate covers \
                                 fewer fields than the reconcile hash; self-heal cannot \
                                 repair this row"
                            );
                            continue;
                        } else {
                            tracing::warn!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                mapper_outcome = ?outcome,
                                pg_hash_moved,
                                current_legacy_hash = ?current_legacy_hash,
                                reprojected_pg_hash = ?reprojected_pg_hash,
                                "[Sync] Force-converge (#204): canonical re-projected but \
                                 hashes still diverge — leaving row open for operator review"
                            );
                            continue;
                        }
                    }
                    Ok(_) => {
                        // Legacy row no longer exists (or unsupported table) —
                        // nothing to project from.
                        tracing::debug!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            "[Sync] Force-converge (#204): skipped (legacy row absent), \
                             leaving row open"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::warn!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            error = %e,
                            "[Sync] Force-converge (#204): re-projection failed, leaving \
                             row open"
                        );
                        continue;
                    }
                }
            } else if reingest_missing_pg_eligible(
                &table_name,
                current_legacy_hash.as_deref(),
                current_pg_hash.as_deref(),
                age_secs,
                reingest_missing_pg_enabled,
            ) {
                // ---------------------------------------------------------------
                // `missing_pg` re-ingest arm (2026-07-27, ship dark behind
                // RECONCILE_REINGEST_MISSING_PG_ENABLED).
                //
                // Legacy still has the row, canonical never got it: a DROPPED
                // INGEST. CT's 2-day retention aged the event out, so the
                // watcher can never redeliver it and no observational sweep
                // can ever close the row — `should_auto_resolve` needs both
                // hashes and `(Some(legacy), None)` falls to `_ => false`.
                // Repair by re-running the NORMAL mapper for that key (the
                // same `apply` the CT watcher uses), then re-hash and only
                // close the row if the two sides actually converge.
                //
                // PG-write-only: legacy MSSQL is read, never written.
                // FK ordering is guaranteed by the sweep's candidate sort —
                // every `customers` row is healed before any `bookings` row
                // in the same pass.
                match force_converge_reconcile_row(
                    legacy_pool,
                    pg_pool,
                    &table_name,
                    &legacy_pk,
                    // No canonical row exists — this is an insert-if-absent.
                    ChangeOp::Insert,
                )
                .await
                {
                    Ok(outcome) if outcome.mapper_ran() => {
                        // Only the canonical side can have moved (we projected
                        // FROM the legacy row), so re-fetch it and re-test.
                        let reprojected_pg_hash =
                            match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await {
                                Ok(opt) => opt,
                                Err(e) => {
                                    tracing::warn!(
                                        site = %site_id,
                                        id,
                                        table_name = %table_name,
                                        legacy_pk = %legacy_pk,
                                        error = %e,
                                        "[Sync] Re-ingest (missing_pg): re-hash of canonical \
                                         failed after re-ingest, leaving row open"
                                    );
                                    continue;
                                }
                            };
                        // `current_pg_hash` is `None` on this arm by
                        // construction (that is what "missing_pg" means), so
                        // "moved" here reads as "canonical now exists / has a
                        // hash at all".
                        let pg_hash_moved =
                            reprojected_pg_hash.as_deref() != current_pg_hash.as_deref();
                        let converged = should_auto_resolve(
                            &table_name,
                            current_legacy_hash.as_deref(),
                            reprojected_pg_hash.as_deref(),
                            recorded_mssql_hash.as_deref(),
                        );
                        let gate_skip = classify_force_converge(outcome, pg_hash_moved, converged);
                        if converged {
                            tracing::info!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                age_secs,
                                "[Sync] Re-ingest (missing_pg): re-ran the mapper for a \
                                 dropped ingest; canonical now present and hashes \
                                 converge — marking resolved"
                            );
                            // Fall through (no `continue`) to the resolved UPDATE.
                        } else if gate_skip {
                            // Same tripwire as the value-drift arm, and a much
                            // sharper signal here: the canonical row was ABSENT,
                            // so a mapper that wrote nothing and moved nothing
                            // means the re-ingest silently did not happen.
                            tracing::warn!(
                                event_name = EV_FORCE_CONVERGE_GATE_SKIP,
                                arm = "missing_pg",
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                age_secs,
                                current_legacy_hash = ?current_legacy_hash,
                                current_pg_hash = ?current_pg_hash,
                                "[Sync] Re-ingest (missing_pg): mapper returned Ok(None) and \
                                 canonical did not move, yet the row is still unconverged — \
                                 the re-ingest wrote nothing; self-heal cannot repair this row"
                            );
                            continue;
                        } else {
                            tracing::warn!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                mapper_outcome = ?outcome,
                                pg_hash_moved,
                                current_legacy_hash = ?current_legacy_hash,
                                reprojected_pg_hash = ?reprojected_pg_hash,
                                "[Sync] Re-ingest (missing_pg): canonical re-ingested but \
                                 hashes still unconverged — leaving row open for operator \
                                 review"
                            );
                            continue;
                        }
                    }
                    Ok(_) => {
                        // The legacy row vanished between the hash probe and
                        // this re-fetch (or the aggregate header is gone).
                        // Distinct message on purpose — `/diagnose-alert`
                        // greps this to tell "legacy row genuinely absent"
                        // apart from "heal attempted and failed".
                        tracing::warn!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            "[Sync] Re-ingest (missing_pg): legacy row absent at re-fetch \
                             — nothing to project from, leaving row open"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::warn!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            error = %e,
                            "[Sync] Re-ingest (missing_pg): re-ingest failed, leaving row \
                             open"
                        );
                        continue;
                    }
                }
            } else {
                // Observational-only behaviour (also the flag-OFF path).
                //
                // Per-row visibility into stuck rows (prod debug 2026-05-18):
                // operators need to see WHY each persistent reconcile_log
                // row isn't converging — (a) legacy hash missing, (b)
                // canonical hash missing, or (c) hashes computed but
                // genuinely don't match. Kept at debug level so it doesn't
                // flood at info; the same field-style as the converged-row
                // debug! below for grep symmetry.
                //
                // `outcome` classifies the three shapes explicitly because
                // `ht_reconcile_log` has no status column beyond
                // `resolved_at` — any outcome distinction has to live in
                // structured logs. `legacy_row_absent` in particular is the
                // genuine-anomaly case that no self-heal arm will ever touch.
                let outcome = match (current_legacy_hash.as_deref(), current_pg_hash.as_deref()) {
                    (None, _) => "legacy_row_absent",
                    (Some(_), None) => "canonical_row_absent",
                    _ => "hashes_unconverged",
                };
                tracing::debug!(
                    site = %site_id,
                    id,
                    table_name = %table_name,
                    legacy_pk = %legacy_pk,
                    outcome,
                    current_legacy_hash = ?current_legacy_hash,
                    current_pg_hash = ?current_pg_hash,
                    recorded_mssql_hash = ?recorded_mssql_hash,
                    "[Sync] Auto-resolve sweep: hashes did not converge, leaving row open"
                );
                continue;
            }
        }

        let update = sqlx::query(
            "UPDATE ht_reconcile_log SET resolved_at = NOW() \
              WHERE id = $1 AND resolved_at IS NULL",
        )
        .bind(id)
        .execute(pg_pool)
        .await;

        match update {
            Ok(r) if r.rows_affected() == 1 => {
                resolved += 1;
                tracing::debug!(
                    site = %site_id,
                    id,
                    table_name = %table_name,
                    legacy_pk = %legacy_pk,
                    "[Sync] Auto-resolve sweep: hashes converged, marked resolved"
                );
            }
            Ok(_) => {
                // Row was already resolved by a concurrent path — fine.
            }
            Err(e) => {
                tracing::warn!(
                    site = %site_id,
                    id,
                    error = %e,
                    "[Sync] Auto-resolve sweep: UPDATE failed, leaving row unresolved"
                );
            }
        }
    }

    if resolved > 0 {
        tracing::info!(site = %site_id, resolved, "[Sync] Auto-resolve sweep: rows reconciled");
    }
    Ok(resolved)
}

/// Record a sync error in the sync_status table.
///
/// Phase 6 status: kept for the bootstrap path (`bin/sync --bootstrap`
/// in `Upsert` mode) and for the rare operator-toggled
/// `LEGACY_SYNC_RECONCILE_MODE=upsert` rollback path. The `DiffOnly`
/// hot path also calls it on hard errors so dashboards still surface a
/// per-entity failure counter — observability, not control flow.
async fn record_error(pg_pool: &PgPool, entity: &str, error: &str) {
    let _ = sqlx::query(
        r#"
        UPDATE sync_status
        SET last_error = $1,
            last_error_at = NOW(),
            consecutive_failures = consecutive_failures + 1
        WHERE entity_type = $2
        "#,
    )
    .bind(error)
    .bind(entity)
    .execute(pg_pool)
    .await;
}

/// Update sync_status after a successful sync.
///
/// Phase 6 status: continues to update `sync_status` rows (records
/// counts + duration) for both `Upsert` and `DiffOnly` modes — the
/// dashboard that reads `sync_status` to show "last reconcile tick
/// touched N rows in Mms" depends on this. Do NOT remove without first
/// updating the operator dashboard.
async fn record_success(
    pg_pool: &PgPool,
    entity: &str,
    added: i32,
    updated: i32,
    unchanged: i32,
    duration_ms: i32,
) {
    let total = added + updated + unchanged;
    let _ = sqlx::query(
        r#"
        UPDATE sync_status
        SET last_sync_at = NOW(),
            records_synced = $1,
            records_added = $2,
            records_updated = $3,
            records_unchanged = $4,
            sync_duration_ms = $5,
            consecutive_failures = 0
        WHERE entity_type = $6
        "#,
    )
    .bind(total)
    .bind(added)
    .bind(updated)
    .bind(unchanged)
    .bind(duration_ms)
    .bind(entity)
    .execute(pg_pool)
    .await;
}

// =============================================================================
// Customer Sync
// =============================================================================

/// Canonical-side projection of a customer row for hashing. Loaded by
/// `legacy_cust_no` join on `ht_customers`. Returns `None` when the CT
/// watcher hasn't yet projected this PK into canonical — that PG-miss
/// case is itself a drift signal recorded with `pg_hash=NULL`.
struct CanonicalCustomerRow {
    cust_firstname: String,
    cust_type: Option<String>,
    cust_phone: Option<String>,
    cust_idcard: Option<String>,
    cust_address: Option<String>,
}

async fn fetch_canonical_customer(
    pg_pool: &PgPool,
    legacy_cust_no: &str,
) -> Result<Option<CanonicalCustomerRow>, sqlx::Error> {
    sqlx::query_as::<
        _,
        (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(
        "SELECT cust_firstname, cust_type, cust_phone, cust_idcard, cust_address \
           FROM ht_customers \
          WHERE legacy_cust_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cust_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(
            |(firstname, type_, phone, idcard, address)| CanonicalCustomerRow {
                cust_firstname: firstname,
                cust_type: type_,
                cust_phone: phone,
                cust_idcard: idcard,
                cust_address: address,
            },
        )
    })
}

/// Best-effort cache UPDATE: ack the most recent `mssql_hash` for this
/// PK so subsequent ticks short-circuit before re-querying canonical.
/// Cache-only — does NOT mutate canonical state. A failed write merely
/// re-fires the alert next tick.
async fn ack_customer_mirror(pg_pool: &PgPool, cust_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_customers_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE cust_no = $2",
    )
    .bind(mssql_hash)
    .bind(cust_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_customers_legacy (cust_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (cust_no) DO UPDATE SET sync_hash = EXCLUDED.sync_hash, \
                                                   synced_at = EXCLUDED.synced_at",
        )
        .bind(cust_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

/// Escape-hatch path (mode=upsert): UPSERT the cache mirror's data
/// columns from the MSSQL projection. Not exercised in production after
/// v2.63.0 — preserved for forensic rollback flexibility only.
#[allow(clippy::too_many_arguments)]
async fn upsert_customer_mirror(
    pg_pool: &PgPool,
    cust_no: &str,
    cust_name: &Option<String>,
    cust_type: &Option<String>,
    cust_phone: &Option<String>,
    cust_idcard: &Option<String>,
    cust_address: &Option<String>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_customers_legacy WHERE cust_no = $1",
    )
    .bind(cust_no)
    .fetch_optional(pg_pool)
    .await?;

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_customers_legacy
                SET cust_name = $1, cust_type = $2, cust_phone = $3,
                    cust_idcard = $4, cust_address = $5,
                    sync_hash = $6, synced_at = NOW()
                WHERE cust_no = $7
                "#,
            )
            .bind(cust_name)
            .bind(cust_type)
            .bind(cust_phone)
            .bind(cust_idcard)
            .bind(cust_address)
            .bind(mssql_hash)
            .bind(cust_no)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_customers_legacy
                    (cust_no, cust_name, cust_type, cust_phone, cust_idcard, cust_address, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(cust_no)
            .bind(cust_name)
            .bind(cust_type)
            .bind(cust_phone)
            .bind(cust_idcard)
            .bind(cust_address)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

/// Legacy projection for the customer reconcile hash. Must mirror the
/// CT mapper's column semantics (see `sync::mappers::customer`) so the
/// MSSQL-side hash and the canonical-side hash project the SAME six
/// fields. Drift between this projection and the mapper's writes
/// produces systematic false-positive `ht_reconcile_log` rows that the
/// auto-resolve sweep cannot ever clear.
///
/// **Why `HT_Customers` and NOT `View_Customers`:** the legacy view
/// concatenates `Cust_perfix + Cust_name + ' ' + Cust_name2` into a
/// single `Cust_name` column and builds `C_Address` from eight address
/// components. The CT mapper writes only the base `Cust_name` into
/// `cust_firstname` and only the door number (`Cust_Add_no`) into
/// `cust_address`. Hashing the view's collapsed values against the
/// mapper's component values guarantees a mismatch on every customer
/// that has either a prefix, a secondary name, or a multi-line
/// address.
///
/// **Why `Cust_Type_Main` and NOT `Cust_Type`:** the CT mapper writes
/// `Cust_Type_Main` (the customer-category literal, e.g.
/// `'บุคคลธรรมดา'`) into PG `cust_type`. The view's `Cust_Type` column
/// is actually the *rate-tier* label (e.g. `'ราคาปกติ'`) which the
/// mapper writes into the separate `cust_price_tier` column. The two
/// frequently differ.
const CUSTOMERS_RECONCILE_PROJECTION: &str = "Cust_no, \
                                              Cust_name, \
                                              Cust_Type_Main, \
                                              Cust_Add_tel, \
                                              Cust_IDcard, \
                                              Cust_Add_no";

async fn sync_customers(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing customers...");

    let mut conn = legacy_pool.get().await?;

    let select_sql = format!(
        "SELECT {projection} FROM HT_Customers",
        projection = CUSTOMERS_RECONCILE_PROJECTION,
    );
    let rows = simple_query_with_timeout_pooled(&mut conn, &select_sql, MssqlOpKind::Read).await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cust_no = row
            .get::<&str, _>("Cust_no")
            .unwrap_or_default()
            .to_string();
        let cust_name = row.get::<&str, _>("Cust_name").map(String::from);
        // `Cust_Type_Main` (not `Cust_Type`) is the column the CT mapper
        // mirrors into PG `cust_type`. See doc on
        // `CUSTOMERS_RECONCILE_PROJECTION` for the rate-tier vs.
        // customer-category distinction.
        let cust_type = row.get::<&str, _>("Cust_Type_Main").map(String::from);
        let cust_phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
        let cust_idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
        // `Cust_Add_no` (the door number, not the view's concatenated
        // `C_Address`) is what the CT mapper mirrors into PG
        // `cust_address`. See doc on `CUSTOMERS_RECONCILE_PROJECTION`.
        let cust_address = row.get::<&str, _>("Cust_Add_no").map(String::from);

        // v2.63.0: canonical-shape hash of the MSSQL projection. Same
        // field set + serialisation as `customer_canonical_hash`, so a
        // faithful CT-mapper projection lands an identical hash on the
        // canonical side.
        let mssql_hash = customer_canonical_hash(
            &cust_no,
            cust_name.as_deref().unwrap_or(""),
            cust_type.as_deref(),
            cust_phone.as_deref(),
            cust_idcard.as_deref(),
            cust_address.as_deref(),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_customer_mirror(
                    pg_pool,
                    &cust_no,
                    &cust_name,
                    &cust_type,
                    &cust_phone,
                    &cust_idcard,
                    &cust_address,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_customers_legacy WHERE cust_no = $1",
                )
                .bind(&cust_no)
                .fetch_optional(pg_pool)
                .await?;

                // Dedupe: identical `mssql_hash` as last acknowledged
                // means drift (if any) is already in `ht_reconcile_log`.
                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_customer(pg_pool, &cust_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    customer_canonical_hash(
                        &cust_no,
                        &c.cust_firstname,
                        c.cust_type.as_deref(),
                        c.cust_phone.as_deref(),
                        c.cust_idcard.as_deref(),
                        c.cust_address.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    // Canonical matches legacy. Ack so we skip the
                    // SELECT-join next tick on this stable PK.
                    ack_customer_mirror(pg_pool, &cust_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                // Drift: canonical PG-miss (None) or hash mismatch.
                let mssql_json = json!({
                    "Cust_no": cust_no,
                    "Cust_name": cust_name,
                    "Cust_Type": cust_type,
                    "Cust_Add_tel": cust_phone,
                    "Cust_IDcard": cust_idcard,
                    "C_Address": cust_address,
                });
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "cust_firstname": c.cust_firstname,
                        "cust_type": c.cust_type,
                        "cust_phone": c.cust_phone,
                        "cust_idcard": c.cust_idcard,
                        "cust_address": c.cust_address,
                    })
                });
                // Track D / T7 CRIT-1: customers are flat 1:1 PKs on
                // both sides — `legacy_row_count` and `pg_row_count`
                // are 0 or 1 by construction. The pg_row_count == 0
                // case is the `canonical.is_none()` MissingPg path.
                let legacy_row_count: i32 = 1;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "customers",
                    &cust_no,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                // Track D / T7 CRIT-1: only silence the alert via the
                // cache UPDATE when the kind is silenceable
                // (value-drift). Cardinality + missing_pg refire every
                // tick until operator action.
                if kind.is_silenceable() {
                    ack_customer_mirror(pg_pool, &cust_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Customers ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode,
        added,
        updated,
        unchanged,
        duration_ms
    );
    record_success(pg_pool, "customers", added, updated, unchanged, duration_ms).await;

    Ok(())
}

// =============================================================================
// Room Sync
// =============================================================================

/// Canonical-side projection of a room row for hashing. Resolved by
/// `legacy_room_no` first (writeback's preferred key), falling back to
/// `room_no` so rooms that predate the writeback resolver still get
/// compared.
struct CanonicalRoomRow {
    room_clean: Option<bool>,
    room_maintenance: Option<bool>,
    room_notes: Option<String>,
}

async fn fetch_canonical_room(
    pg_pool: &PgPool,
    legacy_room_no: &str,
) -> Result<Option<CanonicalRoomRow>, sqlx::Error> {
    sqlx::query_as::<_, (Option<bool>, Option<bool>, Option<String>)>(
        "SELECT room_clean, room_maintenance, room_notes \
           FROM ht_rooms_new \
          WHERE legacy_room_no = $1 \
             OR room_no = $1 \
          ORDER BY (legacy_room_no = $1) DESC \
          LIMIT 1",
    )
    .bind(legacy_room_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(clean, maintenance, notes)| CanonicalRoomRow {
            room_clean: clean,
            room_maintenance: maintenance,
            room_notes: notes,
        })
    })
}

async fn ack_room_mirror(pg_pool: &PgPool, room_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_rooms_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE room_no = $2",
    )
    .bind(mssql_hash)
    .bind(room_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_rooms_legacy (room_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (room_no) DO UPDATE SET sync_hash = EXCLUDED.sync_hash, \
                                                  synced_at = EXCLUDED.synced_at",
        )
        .bind(room_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn upsert_room_mirror(
    pg_pool: &PgPool,
    room_no: &str,
    room_type: &Option<String>,
    room_details: &Option<String>,
    room_clean: &Option<String>,
    room_use: &Option<String>,
    room_book: &Option<String>,
    room_manternace: &Option<String>,
    room_price_a: Option<f64>,
    room_price_b: Option<f64>,
    room_price_c: Option<f64>,
    room_group: &Option<String>,
    room_book_name: &Option<String>,
    room_book_time: &Option<NaiveDateTime>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_rooms_legacy WHERE room_no = $1",
    )
    .bind(room_no)
    .fetch_optional(pg_pool)
    .await?;

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_rooms_legacy
                SET room_type = $1, room_details = $2, room_clean = $3,
                    room_use = $4, room_book = $5, room_manternace = $6,
                    room_price_a = $7::float8, room_price_b = $8::float8,
                    room_price_c = $9::float8, room_group = $10,
                    room_book_name = $11, room_book_time = $12,
                    sync_hash = $13, synced_at = NOW()
                WHERE room_no = $14
                "#,
            )
            .bind(room_type)
            .bind(room_details)
            .bind(room_clean)
            .bind(room_use)
            .bind(room_book)
            .bind(room_manternace)
            .bind(room_price_a)
            .bind(room_price_b)
            .bind(room_price_c)
            .bind(room_group)
            .bind(room_book_name)
            .bind(room_book_time)
            .bind(mssql_hash)
            .bind(room_no)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_rooms_legacy
                    (room_no, room_type, room_details, room_clean, room_use,
                     room_book, room_manternace, room_price_a, room_price_b,
                     room_price_c, room_group, room_book_name, room_book_time, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8::float8, $9::float8,
                        $10::float8, $11, $12, $13, $14)
                "#,
            )
            .bind(room_no)
            .bind(room_type)
            .bind(room_details)
            .bind(room_clean)
            .bind(room_use)
            .bind(room_book)
            .bind(room_manternace)
            .bind(room_price_a)
            .bind(room_price_b)
            .bind(room_price_c)
            .bind(room_group)
            .bind(room_book_name)
            .bind(room_book_time)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

/// Legacy `HT_Rooms` projection for the room reconcile hash. Held as a
/// module-private const so Track J1's projection-lock test can pin
/// every column against the authoritative HF Hotel schema dump.
///
/// `Room_Manternace` is the legacy schema's verbatim spelling (sic —
/// typo for "Maintenance"). Renaming it client-side would break the
/// SELECT.
const ROOMS_RECONCILE_PROJECTION: &[&str] = &[
    "Room_no",
    "Room_Type",
    "Room_Details",
    "Room_Clean",
    "Room_Use",
    "Room_Book",
    "Room_Manternace",
    "Room_PriceA",
    "Room_PriceB",
    "Room_PriceC",
    "Room_Group",
    "Room_Book_Name",
    "Room_Book_Time",
];

async fn sync_rooms(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing rooms...");

    let mut conn = legacy_pool.get().await?;

    let rooms_select_sql = format!(
        "SELECT {projection} FROM HT_Rooms ORDER BY Room_no",
        projection = ROOMS_RECONCILE_PROJECTION.join(", "),
    );
    let rows =
        simple_query_with_timeout_pooled(&mut conn, &rooms_select_sql, MssqlOpKind::Read).await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let room_no = row
            .get::<&str, _>("Room_no")
            .unwrap_or_default()
            .to_string();
        // MSSQL projection captured even for fields excluded from the
        // canonical-shape hash — operators reading `ht_reconcile_log`
        // want the full row payload to investigate.
        let room_type = row.get::<&str, _>("Room_Type").map(String::from);
        let room_details = row.get::<&str, _>("Room_Details").map(String::from);
        let room_clean = row.get::<&str, _>("Room_Clean").map(String::from);
        let room_use = row.get::<&str, _>("Room_Use").map(String::from);
        let room_book = row.get::<&str, _>("Room_Book").map(String::from);
        let room_manternace = row.get::<&str, _>("Room_Manternace").map(String::from);
        let room_price_a = row.get::<f64, _>("Room_PriceA");
        let room_price_b = row.get::<f64, _>("Room_PriceB");
        let room_price_c = row.get::<f64, _>("Room_PriceC");
        let room_group = row.get::<&str, _>("Room_Group").map(String::from);
        let room_book_name = row.get::<&str, _>("Room_Book_Name").map(String::from);
        let room_book_time: Option<NaiveDateTime> = row.try_get("Room_Book_Time").unwrap_or(None);

        // v2.63.0: canonical-shape hash over the CT-tracked fields only
        // (room_clean, room_maintenance, room_notes/details). Legacy
        // yes/no literals are folded through `legacy_yesno_canonical`
        // so they line up with the canonical `BOOLEAN` columns via
        // `bool_to_yesno`.
        let mssql_hash = room_canonical_hash(
            &room_no,
            legacy_yesno_canonical(room_clean.as_deref()),
            legacy_yesno_canonical(room_manternace.as_deref()),
            room_details.as_deref(),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_room_mirror(
                    pg_pool,
                    &room_no,
                    &room_type,
                    &room_details,
                    &room_clean,
                    &room_use,
                    &room_book,
                    &room_manternace,
                    room_price_a,
                    room_price_b,
                    room_price_c,
                    &room_group,
                    &room_book_name,
                    &room_book_time,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_rooms_legacy WHERE room_no = $1",
                )
                .bind(&room_no)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_room(pg_pool, &room_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    room_canonical_hash(
                        &room_no,
                        // `room_clean` is INVERTED between the two sides —
                        // legacy 'yes' means NEEDS cleaning, canonical `true`
                        // means IS clean. Commit 0303c98 added
                        // `clean_bool_to_legacy_yesno` for exactly this but
                        // only applied it to `compute_current_pg_hash`'s copy
                        // of this projection, leaving THIS one on the
                        // uninverted `bool_to_yesno`. The two disagreed, so
                        // every room with a non-NULL `Room_Clean` was detected
                        // as `value` drift and then immediately closed by the
                        // auto-resolve sweep (which uses the corrected
                        // inverse) — pure churn, and a row that misses the
                        // sweep's LIMIT 500 can age past the 4h level alert.
                        // Maintenance is NOT inverted and keeps `bool_to_yesno`.
                        clean_bool_to_legacy_yesno(c.room_clean),
                        bool_to_yesno(c.room_maintenance),
                        c.room_notes.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    ack_room_mirror(pg_pool, &room_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                let mssql_json = json!({
                    "Room_no": room_no,
                    "Room_Type": room_type,
                    "Room_Details": room_details,
                    "Room_Clean": room_clean,
                    "Room_Use": room_use,
                    "Room_Book": room_book,
                    "Room_Manternace": room_manternace,
                    "Room_PriceA": room_price_a,
                    "Room_PriceB": room_price_b,
                    "Room_PriceC": room_price_c,
                    "Room_Group": room_group,
                    "Room_Book_Name": room_book_name,
                });
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "room_clean": c.room_clean,
                        "room_maintenance": c.room_maintenance,
                        "room_notes": c.room_notes,
                    })
                });
                // Track D / T7 CRIT-1: rooms are flat 1:1 PKs.
                let legacy_row_count: i32 = 1;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "rooms",
                    &room_no,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                if kind.is_silenceable() {
                    ack_room_mirror(pg_pool, &room_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
        // `room_book_time` is only consumed by the Upsert branch; the
        // explicit reference silences any over-zealous "unused
        // binding" lint in DiffOnly-only execution paths.
        let _ = &room_book_time;
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Rooms ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode,
        added,
        updated,
        unchanged,
        duration_ms
    );
    record_success(pg_pool, "rooms", added, updated, unchanged, duration_ms).await;

    Ok(())
}

// =============================================================================
// Booking Sync
// =============================================================================

/// Legacy `View_Booking_Ds` projection for the booking reconcile hash.
/// Held as a module-private const so Track J1's projection-lock test
/// can pin every column.
///
/// `View_Booking_Ds` joins `HT_Book_H` (header) with `HT_Book_Ds`
/// (per-room detail), so every column in this projection must exist on
/// one of those two base tables.
/// NOT a dual-source hash — checked on the live server 2026-07-28, closing a
/// concern raised (plausibly, but wrongly) from the truncated view definition
/// in `docs/legacy-spike/schema/01-baseline-schema.txt:682`.
///
/// The worry was that the bookings idempotency gate compares header-derived
/// dates (`derive_stay_range` off `HT_Book_H`) while this reconcile hash reads
/// dates off the representative `View_Booking_Ds` LINE — which would let
/// iHOTEL's `SAVE_EDIT` move the hash without moving any gated field, i.e. a
/// gate ⊂ hash violation invisible to `sync::gate_guard`'s name-level check.
///
/// `sys.sql_modules` says otherwise — the view takes these two columns from the
/// JOINED HEADER, not from the detail rows:
///
/// ```text
/// CREATE VIEW [View_Booking_Ds] AS SELECT HT_Book_Ds.Book_No, …,
///   HT_Book_H.Book_Date_in, HT_Book_H.Book_Date_out, …
/// FROM HT_Book_Ds INNER JOIN HT_Book_H ON HT_Book_Ds.Book_No = HT_Book_H.Book_ID
/// ```
///
/// So gate and hash read the SAME source and no unification is needed.
/// (`HT_Book_Ds` has no `Book_Date_in`/`Book_Date_out` columns at all — its
/// per-line dates are `Book_Room_Start`/`Book_Room_End`, which this projection
/// deliberately does not read.)
const BOOKINGS_RECONCILE_PROJECTION: &[&str] = &[
    "Book_No",
    "Book_Date",
    "Book_Date_in",
    "Book_Date_out",
    "Book_Cust_Name",
    "Book_Cust_ID",
    "Book_Status",
    "Book_Room_Type",
];

async fn sync_bookings(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing bookings...");

    let mut conn = legacy_pool.get().await?;

    let bookings_select_sql = format!(
        "SELECT {projection} FROM View_Booking_Ds",
        projection = BOOKINGS_RECONCILE_PROJECTION.join(", "),
    );
    let rows = simple_query_with_timeout_pooled(&mut conn, &bookings_select_sql, MssqlOpKind::Read)
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    // Phase 6 hotfix (2026-04-29): `View_Booking_Ds` returns up to 3
    // rows per `(Book_No, Book_Room_Type)` composite PK. The previous
    // per-row loop computed a different hash for each iteration of the
    // same PK and re-flagged divergence forever. Aggregate by composite
    // PK first, then hash the whole group deterministically — one
    // record_divergence + one cache UPDATE per PK.
    let mut groups: BTreeMap<(String, String), Vec<BookingDetail>> = BTreeMap::new();
    for row in &rows {
        let book_no = row
            .get::<&str, _>("Book_No")
            .unwrap_or_default()
            .to_string();
        let book_room_type = row.get::<&str, _>("Book_Room_Type").map(String::from);
        let detail = BookingDetail {
            book_date: row.try_get("Book_Date").unwrap_or(None),
            book_date_in: row.try_get("Book_Date_in").unwrap_or(None),
            book_date_out: row.try_get("Book_Date_out").unwrap_or(None),
            book_cust_name: row.get::<&str, _>("Book_Cust_Name").map(String::from),
            book_cust_id: row.get::<&str, _>("Book_Cust_ID").map(String::from),
            book_status: row.get::<i32, _>("Book_Status"),
            book_room_type: book_room_type.clone(),
        };
        let room_type_key = book_room_type.unwrap_or_default();
        groups
            .entry((book_no, room_type_key))
            .or_default()
            .push(detail);
    }

    for ((book_no, room_type_key), mut details) in groups {
        // Deterministic legacy multi-row → single-row collapse: sort by
        // every non-key field (matching `aggregate_booking_hash`'s sort
        // contract) and pick the first detail row as the canonical-shape
        // representative. Canonical `ht_bookings` has one row per
        // `legacy_book_id`, so a single representative is enough.
        sort_booking_details(&mut details);
        let representative = details.first();

        // v2.63.0: canonical-shape hash of the legacy projection.
        // `book_status` intentionally omitted (see `booking_canonical_hash`
        // docs). Dates: legacy returns DATETIME; canonical stores DATE.
        // Drop the time component so both sides hash the same YYYY-MM-DD
        // string (legacy `Book_Date_in/out` are stored at midnight per
        // the booking-create recipe — see `sync::mappers::booking` docs).
        let book_checkin_date =
            representative.and_then(|d| d.book_date_in.map(|dt| dt.date().to_string()));
        let book_checkout_date =
            representative.and_then(|d| d.book_date_out.map(|dt| dt.date().to_string()));
        let book_cust_id_owned = representative.and_then(|d| d.book_cust_id.clone());
        let mssql_hash = booking_canonical_hash(
            &book_no,
            book_checkin_date.as_deref(),
            book_checkout_date.as_deref(),
            book_cust_id_owned.as_deref(),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_booking_mirror(
                    pg_pool,
                    &book_no,
                    &room_type_key,
                    representative,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_bookings_legacy \
                      WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2",
                )
                .bind(&book_no)
                .bind(&room_type_key)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_booking(pg_pool, &book_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    let checkin_str = c.book_checkin.map(|d| d.to_string());
                    let checkout_str = c.book_checkout.map(|d| d.to_string());
                    booking_canonical_hash(
                        &book_no,
                        checkin_str.as_deref(),
                        checkout_str.as_deref(),
                        c.legacy_cust_no.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    ack_booking_mirror(pg_pool, &book_no, &room_type_key, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                let mssql_json = booking_group_json(&book_no, &details);
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "book_checkin": c.book_checkin.map(|d| d.to_string()),
                        "book_checkout": c.book_checkout.map(|d| d.to_string()),
                        "legacy_cust_no": c.legacy_cust_no,
                    })
                });
                let composite_pk = format!("{book_no}|{room_type_key}");
                // Track D / T7 CRIT-1: bookings — `View_Booking_Ds` can
                // return up to 3 rows per composite PK (booking with
                // multiple room types within the same Book_No);
                // canonical `ht_bookings` collapses to one row per
                // `legacy_book_id`. legacy_row_count exposes the raw
                // legacy cardinality so a 3-row Book_No vs 1-row PG
                // mismatch lights up as `Cardinality` instead of being
                // silenced as a value drift.
                let legacy_row_count: i32 = details.len() as i32;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "bookings",
                    &composite_pk,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                if kind.is_silenceable() {
                    ack_booking_mirror(pg_pool, &book_no, &room_type_key, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Bookings ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode,
        added,
        updated,
        unchanged,
        duration_ms
    );
    record_success(pg_pool, "bookings", added, updated, unchanged, duration_ms).await;

    Ok(())
}

/// Sort a booking PK group's detail rows deterministically. Mirrors the
/// sort contract in [`aggregate_booking_hash`] so the "first" row is
/// stable across reconcile ticks regardless of the order MSSQL returned
/// the detail rows.
fn sort_booking_details(details: &mut [BookingDetail]) {
    details.sort_by(|a, b| {
        (
            fmt_dt(&a.book_date),
            fmt_dt(&a.book_date_in),
            fmt_dt(&a.book_date_out),
            fmt_str(&a.book_cust_name),
            fmt_str(&a.book_cust_id),
            a.book_status.unwrap_or(i32::MIN),
        )
            .cmp(&(
                fmt_dt(&b.book_date),
                fmt_dt(&b.book_date_in),
                fmt_dt(&b.book_date_out),
                fmt_str(&b.book_cust_name),
                fmt_str(&b.book_cust_id),
                b.book_status.unwrap_or(i32::MIN),
            ))
    });
}

/// Canonical-side projection of a booking row for hashing. Resolved by
/// `legacy_book_id`. `legacy_cust_no` is read directly from
/// `ht_bookings.legacy_cust_no` (denormalised by the writeback
/// resolver) rather than joined through `ht_customers` — keeps the
/// drift check resilient to a transient FK gap between bookings and
/// customers in the canonical store.
struct CanonicalBookingRow {
    book_checkin: Option<chrono::NaiveDate>,
    book_checkout: Option<chrono::NaiveDate>,
    legacy_cust_no: Option<String>,
}

async fn fetch_canonical_booking(
    pg_pool: &PgPool,
    legacy_book_id: &str,
) -> Result<Option<CanonicalBookingRow>, sqlx::Error> {
    sqlx::query_as::<
        _,
        (
            Option<chrono::NaiveDate>,
            Option<chrono::NaiveDate>,
            Option<String>,
        ),
    >(
        "SELECT book_checkin, book_checkout, legacy_cust_no \
           FROM ht_bookings \
          WHERE legacy_book_id = $1 \
          LIMIT 1",
    )
    .bind(legacy_book_id)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(checkin, checkout, legacy_cust_no)| CanonicalBookingRow {
            book_checkin: checkin,
            book_checkout: checkout,
            legacy_cust_no,
        })
    })
}

async fn ack_booking_mirror(
    pg_pool: &PgPool,
    book_no: &str,
    room_type_key: &str,
    mssql_hash: &str,
) {
    let updated = sqlx::query(
        "UPDATE ht_bookings_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE book_no = $2 AND COALESCE(book_room_type, '') = $3",
    )
    .bind(mssql_hash)
    .bind(book_no)
    .bind(room_type_key)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_bookings_legacy (book_no, book_room_type, sync_hash, synced_at) \
             VALUES ($1, $2, $3, NOW()) \
             ON CONFLICT (book_no, book_room_type) DO UPDATE \
               SET sync_hash = EXCLUDED.sync_hash, synced_at = EXCLUDED.synced_at",
        )
        .bind(book_no)
        .bind(room_type_key)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn upsert_booking_mirror(
    pg_pool: &PgPool,
    book_no: &str,
    room_type_key: &str,
    representative: Option<&BookingDetail>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_bookings_legacy \
          WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2",
    )
    .bind(book_no)
    .bind(room_type_key)
    .fetch_optional(pg_pool)
    .await?;

    // Empty groups should not occur (we only enter the loop with at
    // least one detail row), but guard so the Upsert path never panics
    // under a degenerate input.
    let Some(rep) = representative else {
        return Ok(());
    };

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_bookings_legacy
                SET book_date = $1, book_date_in = $2, book_date_out = $3,
                    book_cust_name = $4, book_cust_id = $5, book_status = $6,
                    sync_hash = $7, synced_at = NOW()
                WHERE book_no = $8 AND COALESCE(book_room_type, '') = $9
                "#,
            )
            .bind(rep.book_date)
            .bind(rep.book_date_in)
            .bind(rep.book_date_out)
            .bind(&rep.book_cust_name)
            .bind(&rep.book_cust_id)
            .bind(rep.book_status)
            .bind(mssql_hash)
            .bind(book_no)
            .bind(room_type_key)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_bookings_legacy
                    (book_no, book_date, book_date_in, book_date_out,
                     book_cust_name, book_cust_id, book_status,
                     book_room_type, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#,
            )
            .bind(book_no)
            .bind(rep.book_date)
            .bind(rep.book_date_in)
            .bind(rep.book_date_out)
            .bind(&rep.book_cust_name)
            .bind(&rep.book_cust_id)
            .bind(rep.book_status)
            .bind(&rep.book_room_type)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

// =============================================================================
// Check-in Sync
// =============================================================================

/// Compute the MSSQL-side reconcile hash for a check-in aggregate via
/// the CT mapper's authoritative projection. Mirrors
/// [`compute_legacy_checkin_hash_via_mapper`] (auto-resolve sweep path)
/// so the bulk reconcile loop and the per-PK auto-resolver agree on
/// `(legacy_room_no, cin_checkin_time, effective_checkout_date,
/// legacy_cust_no)` to the byte.
///
/// **Why factored out:** the bulk sweep and the auto-resolver used to
/// project the legacy aggregate independently — the auto-resolver via
/// the mapper, the sweep via an in-Rust group/sort/representative pick
/// over a `HT_CheckIn_H ⋈ HT_CheckIn_Ds` join. The two pipelines
/// disagreed on multi-room folios (mapper picks the lowest-`Cin_Ds.id`
/// row per `derive_room_state`; the sweep picked the alphabetical-first
/// `Cin_Room_No`). Production audit 2026-05-18: 4 of 5 sampled
/// multi-room PKs picked different first rooms, producing ~545 spurious
/// `value` drifts on HF Hotel per tick. Unifying on the mapper here
/// makes "Bug B / C / D class" structurally impossible in the bulk
/// path too (cf. `compute_legacy_checkin_hash_via_mapper`'s doc).
fn checkin_hash_from_canonical(
    canonical: &crate::sync::mappers::checkin::CanonicalCheckIn,
) -> String {
    let effective_checkout: chrono::NaiveDate = canonical
        .cin_checkout_time
        .map(|dt: NaiveDateTime| dt.date())
        .unwrap_or(canonical.cin_expected_checkout);
    let effective_checkout_str = effective_checkout.to_string();
    checkin_canonical_hash(
        &canonical.legacy_cin_no,
        canonical.legacy_room_no.as_deref(),
        Some(canonical.cin_checkin_time.to_string()).as_deref(),
        Some(effective_checkout_str).as_deref(),
        canonical.legacy_cust_no.as_deref(),
        canonical.cin_checkout_time.is_some(),
        canonical.cin_status == "cancelled",
    )
}

/// Pure decision helper for the bulk reconcile loop: given the legacy
/// (MSSQL) hash freshly projected via the mapper, the canonical PG
/// hash, and the legacy/canonical row counts, classify the drift kind
/// (or return `None` if the two hashes match and there's nothing to
/// record).
///
/// Pulled out as a free function so the unit tests can pin the
/// "multi-room folio whose mapper-projected hash matches canonical
/// must NOT report value drift" contract without standing up a PG
/// pool or running the full async loop.
fn classify_checkin_drift(
    canonical_hash: Option<&str>,
    mssql_hash: &str,
    legacy_row_count: i32,
    pg_row_count: i32,
) -> Option<DivergenceKind> {
    if canonical_hash == Some(mssql_hash) && legacy_row_count == pg_row_count {
        return None;
    }
    Some(classify_divergence(
        canonical_hash,
        Some(mssql_hash),
        legacy_row_count,
        pg_row_count,
    ))
}

async fn sync_checkins(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing check-ins...");

    // 1. Pull the PK list with a cheap single-column scan. The
    //    previous shape did one bulk JOIN that brought 41-45 rows back
    //    per folio (~800k rows on HF Hotel), then re-implemented the
    //    mapper's first-room projection in Rust. That parallel
    //    projection disagreed with the CT mapper on multi-room folios
    //    (the CT mapper picks the lowest-`Cin_Ds.id` row via
    //    `derive_room_state`; the bulk loop picked the alphabetical-
    //    first `Cin_Room_No`), producing ~545 spurious `value` drift
    //    rows per tick on HF Hotel (2026-05-18 audit). Unifying on the
    //    mapper eliminates the bug class — see
    //    `compute_legacy_checkin_hash_via_mapper`'s doc comment.
    // The EXISTS filter restores the row-set semantics of the pre-#134
    // bulk JOIN query (`FROM HT_CheckIn_Ds INNER JOIN HT_CheckIn_H`),
    // which implicitly excluded Ds-less header rows. 2026-05-18 prod
    // audit found 201 such rows on HF Hotel — iHOTEL "ghosts" from
    // failed walk-ins / cancelled-before-room-assignment / test data
    // that aren't representable in canonical (`ht_checkins.cin_room_id`
    // is NOT NULL). Without this filter the reconcile sweep flags every
    // ghost as `missing_pg`, producing 200+ false-positive drift rows
    // per tick.
    let mut conn = legacy_pool.get().await?;
    let pk_rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT DISTINCT h.Cin_no FROM HT_CheckIn_H h \
          WHERE EXISTS ( \
              SELECT 1 FROM HT_CheckIn_Ds d WHERE d.Cin_No = h.Cin_no \
          ) \
          ORDER BY h.Cin_no",
        MssqlOpKind::Read,
    )
    .await?;
    let cin_nos: Vec<String> = pk_rows
        .iter()
        .filter_map(|r| r.get::<&str, _>("Cin_no").map(String::from))
        .collect();
    // Free the pool slot before per-PK loads grab their own connection.
    drop(conn);

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;
    let mut load_errors = 0i32;
    let mut project_errors = 0i32;

    // 2. For each PK: load the legacy aggregate (header + Ds + Pay)
    //    and run it through the CT mapper's projection function. The
    //    resulting `CanonicalCheckIn` is what canonical PG SHOULD look
    //    like for this PK, so a hash mismatch with the actual canonical
    //    row can only signal real exogenous drift (CT miss, hand-edited
    //    PG) — never parallel-projection-pipeline divergence.
    //
    //    Cost: ~3 MSSQL queries per PK × 19k PKs (HF Hotel) ≈ 57k
    //    queries per tick, vs 1 bulk query previously. Still well
    //    within the 15-min cron cadence (steady-state ~63 q/s, well
    //    below same-LAN saturation).
    for cin_no in &cin_nos {
        let aggregate =
            match crate::sync::parent_loader::load_checkin_aggregate(legacy_pool, cin_no).await {
                Ok(a) => a,
                Err(e) => {
                    load_errors += 1;
                    tracing::warn!(
                        cin_no = %cin_no,
                        error = %e,
                        "[Sync] sync_checkins: load_checkin_aggregate failed; skipping PK"
                    );
                    continue;
                }
            };
        // Cancelled / deleted folio — the CT watcher emits
        // `CheckInCancelled` on its own; nothing for the sweep to do.
        if !aggregate.is_present() {
            continue;
        }
        let canonical_proj =
            match crate::sync::mappers::project_checkin_aggregate(&aggregate, cin_no) {
                Ok(p) => p,
                Err(e) => {
                    project_errors += 1;
                    tracing::warn!(
                        cin_no = %cin_no,
                        error = %e,
                        "[Sync] sync_checkins: project_checkin_aggregate failed; skipping PK"
                    );
                    continue;
                }
            };

        // Hash inputs are byte-identical to
        // `compute_legacy_checkin_hash_via_mapper` (auto-resolve sweep)
        // so the two paths produce the same string for the same
        // aggregate. The ack-cache (`ht_checkins_legacy.sync_hash`)
        // was stamped under the prior per-row projection, but the
        // chosen first-room row is determined by the mapper's
        // `derive_room_state` (lowest `Cin_Ds.id`) — which is also
        // what canonical-side `ht_checkins.legacy_room_no` holds. So
        // for any folio where canonical is correct, the new hash
        // equals what canonical hashes to today, and the ack cache
        // converges on the next tick.
        let mssql_hash = checkin_hash_from_canonical(&canonical_proj);

        match mode {
            ReconcileMode::Upsert => {
                upsert_checkin_mirror(
                    pg_pool,
                    cin_no,
                    &canonical_proj,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_checkins_legacy WHERE cin_no = $1",
                )
                .bind(cin_no)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_checkin(pg_pool, cin_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    let checkin_str = c.cin_checkin_time.map(|t| t.to_string());
                    let effective_checkout_str = c.effective_checkout_date().map(|d| d.to_string());
                    checkin_canonical_hash(
                        cin_no,
                        c.legacy_room_no.as_deref(),
                        checkin_str.as_deref(),
                        effective_checkout_str.as_deref(),
                        c.legacy_cust_no.as_deref(),
                        c.is_checked_out(),
                        c.is_cancelled(),
                    )
                });

                // Track D / T7 CRIT-1 / Track B last-mile (2026-05-18):
                // `pg_row_count` reads the ACTUAL per-room junction
                // count (`ht_checkin_rooms`). On a query failure fall
                // back to `1` (pre-Track-B observation) rather than
                // `0` — degraded observability beats spurious
                // `missing_pg` misclassification — and warn-log.
                //
                // 2026-05-19 dedup fix: count DISTINCT `Cin_Room_No`
                // values instead of raw `aggregate.rooms.len()`.
                // iHOTEL routinely writes >1 HT_CheckIn_Ds row per
                // room (extends / re-keys / deposit returns) — see
                // CH22-000722 trigger documented on
                // [`count_distinct_legacy_checkin_rooms`]. The
                // `ht_checkin_rooms` junction enforces
                // `UNIQUE (cr_cin_id, cr_room_id)`, so "distinct
                // rooms" IS the canonical truth that `pg_row_count`
                // already reflects.
                let legacy_row_count: i32 = count_distinct_legacy_checkin_rooms(&aggregate.rooms);
                let pg_row_count: i32 = if canonical.is_some() {
                    match count_canonical_checkin_rooms(pg_pool, cin_no).await {
                        Ok(n) => n,
                        Err(e) => {
                            tracing::warn!(
                                cin_no = %cin_no,
                                error = %e,
                                "[Sync] count_canonical_checkin_rooms failed; \
                                 falling back to pre-Track-B pg_row_count=1 \
                                 to avoid spurious missing_pg classification"
                            );
                            1
                        }
                    }
                } else {
                    0
                };

                let drift_kind = classify_checkin_drift(
                    canonical_hash.as_deref(),
                    &mssql_hash,
                    legacy_row_count,
                    pg_row_count,
                );

                let Some(kind) = drift_kind else {
                    // Hashes match AND row counts agree — converged.
                    // Ack the cache so the next tick short-circuits at
                    // the `last_ack` compare above.
                    ack_checkin_mirror(pg_pool, cin_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                };

                let mssql_json = checkin_aggregate_json(cin_no, &aggregate);
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "legacy_room_no": c.legacy_room_no,
                        "cin_checkin_time": c.cin_checkin_time.map(|t| t.to_string()),
                        // Bug D fix (2026-05-16) — surface BOTH the actual
                        // and expected checkout dates for operator triage.
                        // Hash uses `effective_checkout_date()` (actual if
                        // set, else expected); JSON exposes both so the
                        // operator can see which one matched / mismatched
                        // the legacy `Cin_Room_Out` shown in mssql_row_json.
                        "cin_checkout_time": c.cin_checkout_time.map(|t| t.to_string()),
                        "cin_expected_checkout": c.cin_expected_checkout.map(|d| d.to_string()),
                        "legacy_cust_no": c.legacy_cust_no,
                    })
                });
                record_divergence(
                    pg_pool,
                    "checkins",
                    cin_no,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                if kind.is_silenceable() {
                    ack_checkin_mirror(pg_pool, cin_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    if load_errors > 0 || project_errors > 0 {
        tracing::warn!(
            load_errors,
            project_errors,
            "[Sync] sync_checkins: {} aggregate loads / {} projections failed (per-PK skipped, see warn logs)",
            load_errors,
            project_errors,
        );
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Check-ins ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode,
        added,
        updated,
        unchanged,
        duration_ms
    );
    record_success(pg_pool, "checkins", added, updated, unchanged, duration_ms).await;

    Ok(())
}

/// Canonical-side projection of a check-in row for hashing. Resolved
/// by `legacy_cin_no`. `legacy_room_no` is the writeback-resolved
/// denormalised FIRST room (matches the CT mapper's `first_room_no`
/// denormalisation in `derive_room_state`).
///
/// **Checkout-date projection** is two-phase to mirror legacy
/// `Cin_Room_Out`'s dual semantic:
/// * Active stay: `cin_checkout_time IS NULL`, `cin_expected_checkout`
///   = max(Cin_Room_Out among non-checked-out rooms) per
///   `derive_stay_range`. Legacy `Cin_Room_Out` carries the booked
///   future date.
/// * Checked-out stay: `cin_checkout_time` = max(Cin_Room_Out across
///   all rooms) per `derive_room_state` (the actual departure time).
///   Legacy `Cin_Room_Out` is now the actual departure too — extended
///   then completed stays land here.
///
/// Hashing on `cin_expected_checkout` ALONE (Bug B fix 2026-05-15)
/// matches active stays but produces systematic false-positive value
/// drift for the 3,382 completed-extended stays in the HF Hotel
/// audit 2026-05-16: canonical's `cin_expected_checkout` froze at
/// the original booked date while legacy's `Cin_Room_Out` advanced
/// to the actual departure. Bug D fix: hash on
/// `COALESCE(cin_checkout_time::date, cin_expected_checkout)` so
/// the canonical side mirrors legacy's "best-known checkout date"
/// for the stay regardless of status.
struct CanonicalCheckinRow {
    legacy_room_no: Option<String>,
    cin_checkin_time: Option<NaiveDateTime>,
    cin_expected_checkout: Option<chrono::NaiveDate>,
    cin_checkout_time: Option<NaiveDateTime>,
    legacy_cust_no: Option<String>,
    /// Header-derived aggregate status. PG enum literals
    /// (`'active'` / `'checkedout'` / `'cancelled'`) — see
    /// `crate::sync::mappers::checkin::legacy_status_to_pg`.
    ///
    /// Used as the gate for the `cancelled` sentinel branch of
    /// `checkin_canonical_hash` so the legacy and canonical hashes
    /// converge on cancelled folios (whose per-room `HT_CheckIn_Ds`
    /// rows iHOTEL deletes — see the function-level docs on
    /// `checkin_canonical_hash` for the six stuck CH26-* PKs that
    /// motivated this).
    cin_status: Option<String>,
}

impl CanonicalCheckinRow {
    /// Effective checkout date used in the reconcile hash. Mirrors
    /// MSSQL `Cin_Room_Out.date()`: prefers actual departure
    /// (`cin_checkout_time`) when set, else falls back to expected
    /// (`cin_expected_checkout`).
    fn effective_checkout_date(&self) -> Option<chrono::NaiveDate> {
        self.cin_checkout_time
            .map(|dt| dt.date())
            .or(self.cin_expected_checkout)
    }

    /// `true` when the header-derived aggregate status is `'cancelled'`
    /// — gate for the `checkin_canonical_hash` cancelled sentinel.
    /// Treats NULL/missing as not cancelled (the active-stay 5-field
    /// hash path is strictly safer than the sentinel path: it
    /// preserves all the existing field-by-field drift detection).
    fn is_cancelled(&self) -> bool {
        self.cin_status.as_deref() == Some("cancelled")
    }

    /// `true` when an ACTUAL checkout has been recorded (`cin_checkout_time`
    /// is set) — the `checked_out` bit hashed by [`checkin_canonical_hash`].
    /// Mirrors the legacy side's `cin_checkout_time.is_some()` so a dropped
    /// checkout (canonical still active, legacy checked-out) diverges even
    /// when the actual/expected dates coincide (task #68).
    fn is_checked_out(&self) -> bool {
        self.cin_checkout_time.is_some()
    }
}

async fn fetch_canonical_checkin(
    pg_pool: &PgPool,
    legacy_cin_no: &str,
) -> Result<Option<CanonicalCheckinRow>, sqlx::Error> {
    sqlx::query_as::<
        _,
        (
            Option<String>,
            Option<NaiveDateTime>,
            Option<chrono::NaiveDate>,
            Option<NaiveDateTime>,
            Option<String>,
            Option<String>,
        ),
    >(
        "SELECT legacy_room_no, cin_checkin_time, cin_expected_checkout, \
                cin_checkout_time, legacy_cust_no, cin_status \
           FROM ht_checkins \
          WHERE legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cin_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(
            |(room, checkin, expected_checkout, actual_checkout, cust, status)| {
                CanonicalCheckinRow {
                    legacy_room_no: room,
                    cin_checkin_time: checkin,
                    cin_expected_checkout: expected_checkout,
                    cin_checkout_time: actual_checkout,
                    legacy_cust_no: cust,
                    cin_status: status,
                }
            },
        )
    })
}

/// Count canonical check-in room rows for a given `legacy_cin_no`,
/// joining `ht_checkin_rooms` (the Track B junction table created in
/// migration `043_create_ht_checkin_rooms.sql`) against `ht_checkins`.
///
/// **Why this exists:** the reconcile sweep historically hardcoded
/// `pg_row_count = 1` whenever `ht_checkins` had a row (since canonical
/// only stored the first room denormalised). After Track B landed the
/// per-room junction, the CT mapper (`mappers/checkin.rs`) and the
/// `backfill_checkin_rooms` bin populate `ht_checkin_rooms` with one
/// row per booked room. This helper reads that actual count so
/// `classify_divergence` can compare like-for-like with
/// `View_CheckIn_Ds.details.len()`, collapsing the 756 multi-room
/// cardinality drifts observed on HF Hotel on 2026-05-18 from
/// "always Cardinality" to "Value (matching) ⇒ silent ack".
///
/// Follow-up (deferred): the canonical-side hash still hashes only the
/// FIRST room (see `checkin_canonical_hash`). Once row counts match,
/// drift on a SECONDARY room would still be invisible to the hash. A
/// future PR that rotates the canonical hash format to hash ALL
/// junction rows (mirroring `aggregate_booking_hash`'s shape) would
/// close that blind spot, but is out of scope here because rotating
/// the hash
/// invalidates every `ht_checkins_legacy.sync_hash` entry and would
/// trigger a deploy-time alert flood (cf. 2026-05-15).
async fn count_canonical_checkin_rooms(
    pg_pool: &PgPool,
    legacy_cin_no: &str,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1",
    )
    .bind(legacy_cin_no)
    .fetch_one(pg_pool)
    .await
    .map(|n| n as i32)
}

async fn ack_checkin_mirror(pg_pool: &PgPool, cin_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_checkins_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE cin_no = $2",
    )
    .bind(mssql_hash)
    .bind(cin_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_checkins_legacy (cin_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (cin_no) DO UPDATE \
               SET sync_hash = EXCLUDED.sync_hash, synced_at = EXCLUDED.synced_at",
        )
        .bind(cin_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

/// Pre-Phase-5.5 escape-hatch path: UPSERT the `ht_checkins_legacy`
/// mirror's data columns from the CT mapper's canonical projection.
/// Not exercised in production after v2.63.0 — preserved for forensic
/// rollback flexibility only.
///
/// Field mapping mirrors the prior per-row `CheckinDetail`-backed
/// shape so the mirror schema is unchanged:
/// * `cin_room_no`  ← `canonical.legacy_room_no` (mapper's first-room
///                     denormalisation per `derive_room_state`)
/// * `cin_room_in`  ← `canonical.cin_checkin_time` (header `Cin_Date_in`)
/// * `cin_room_out` ← `canonical.cin_checkout_time` when set,
///                     else midnight of `cin_expected_checkout`
///                     (preserves the `NaiveDateTime` column type — the
///                     mapper's date-only `cin_expected_checkout`
///                     would require a column-type change otherwise).
/// * `cin_cust_name` ← `NULL` (this column was always populated from
///                     a view-derived alias the parent loader does
///                     not project; left NULL pre-2026-05-18 too).
/// * `cin_cust_no`  ← `canonical.legacy_cust_no`
/// * `cin_status`   ← `canonical.cin_status` (mapper's translated
///                     literal — `active` / `checkedout` / `cancelled`).
#[allow(clippy::too_many_arguments)]
async fn upsert_checkin_mirror(
    pg_pool: &PgPool,
    cin_no: &str,
    canonical: &crate::sync::mappers::checkin::CanonicalCheckIn,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_checkins_legacy WHERE cin_no = $1",
    )
    .bind(cin_no)
    .fetch_optional(pg_pool)
    .await?;

    let room_in: Option<NaiveDateTime> = Some(canonical.cin_checkin_time);
    let room_out: Option<NaiveDateTime> = canonical.cin_checkout_time.or_else(|| {
        chrono::NaiveTime::from_hms_opt(0, 0, 0)
            .map(|t| canonical.cin_expected_checkout.and_time(t))
    });
    let cust_name: Option<&str> = None;

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_checkins_legacy
                SET cin_room_no = $1, cin_room_in = $2, cin_room_out = $3,
                    cin_cust_name = $4, cin_cust_no = $5, cin_status = $6,
                    sync_hash = $7, synced_at = NOW()
                WHERE cin_no = $8
                "#,
            )
            .bind(&canonical.legacy_room_no)
            .bind(room_in)
            .bind(room_out)
            .bind(cust_name)
            .bind(&canonical.legacy_cust_no)
            .bind(&canonical.cin_status)
            .bind(mssql_hash)
            .bind(cin_no)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_checkins_legacy
                    (cin_no, cin_room_no, cin_room_in, cin_room_out,
                     cin_cust_name, cin_cust_no, cin_status, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
            )
            .bind(cin_no)
            .bind(&canonical.legacy_room_no)
            .bind(room_in)
            .bind(room_out)
            .bind(cust_name)
            .bind(&canonical.legacy_cust_no)
            .bind(&canonical.cin_status)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

// =============================================================================
// Payment (receipt) Sync — Phase 6-A, DARK behind RECONCILE_PAYMENTS_ARM_ENABLED
// =============================================================================

/// Legacy `HT_Receipt_H` projection for the payment reconcile hash.
///
/// Held as a slice const so Track J1's projection-lock test can pin every
/// column against the authoritative schema dump, and so the bulk scan and
/// the per-PK auto-resolve re-fetch cannot drift apart.
///
/// `Receipt_Date` and the VAT columns are deliberately NOT here — see
/// [`payment_canonical_hash`] for why `pay_date` / `pay_method` are
/// excluded from the hash. (`Receipt_Date` IS used in the scan's WHERE
/// clause as the canonical-era floor — see [`PAYMENTS_ERA_FLOOR_SQL`] —
/// which is a scope filter, not a hash input.)
const PAYMENTS_RECONCILE_PROJECTION: &[&str] = &[
    "Receipt_no",
    "Receipt_Total",
    "Receipt_ref",
    "status_name",
];

/// The scan filter for the bulk payments sweep.
///
/// A receipt with no `Receipt_ref` carries no `Cin_no`, and
/// `payment::apply_receipt_upsert` skips it deliberately (canonical
/// `ht_payments.pay_cin_id` is NOT NULL — there is nowhere to land it).
/// Including those rows would manufacture a permanent `missing_pg` row per
/// no-check-in sale: a deliberate design skip reported as sync lag, which
/// is exactly the false-positive class the arm exists to avoid.
const PAYMENTS_RECONCILE_SCAN_FILTER: &str = "Receipt_ref IS NOT NULL AND Receipt_ref <> ''";

/// Derive the canonical-coverage floor for the payments scan, in
/// LEGACY-LOCAL time (Thai / GMT+7, stored naive) — the era boundary below
/// which a legacy receipt provably has no canonical counterpart and never
/// will. `NULL` when `ht_payments` is empty (no coverage at all).
///
/// **Why this exists — 2026-07-28 review, BLOCKING find.** `ht_payments` is
/// populated by the CT watcher, which only ever saw receipts from the day CT
/// was enabled on `HT_Receipt_H`; there is no historical backfill. Live
/// read-only counts that day: HF Hotel legacy `HT_Receipt_H` had 21,566 rows
/// passing [`PAYMENTS_RECONCILE_SCAN_FILTER`] (2021 → 2026) against 1,154
/// canonical payments, ALL dated 2026-04-27 or later. Unfloored, the first
/// enabled tick would classify >20,400 pre-era receipts as
/// [`DivergenceKind::MissingPg`] — a kind that is NOT `is_silenceable()`, so
/// it is never acked, while `payments` is deliberately absent from
/// [`REINGEST_MISSING_PG_TABLES`], so [`should_auto_resolve`] can never close
/// it either. That backlog is PERMANENT, not transient. It would:
///
/// * re-issue ~20.4k [`CANONICAL_PAYMENT_PROBE_SQL`] probes plus ~20.4k
///   dedupe INSERTs every tick, forever — not the advertised one-MSSQL-query
///   + a-few-batched-PG-reads steady state;
/// * pin the 4h `check_level_drift_and_alert` digest and the >72h escalation
///   tier on `payments` permanently;
/// * starve every OTHER entity out of [`auto_resolve_reconcile_log`], whose
///   500-row batch is selected by `detected_at` alone with no per-table
///   fairness — once the payments rows own the oldest band, customers /
///   rooms / bookings / checkins stop being swept at all.
///
/// The prescribed Ville-first canary could NOT have surfaced this: HF Ville
/// has 105 ref-carrying legacy receipts against 4 canonical payments, so its
/// 48h soak lands ~101 rows and stays green by construction.
///
/// Pre-era receipts are out of the mirror's scope in exactly the same sense
/// as a receipt with no `Receipt_ref`: a deliberate design skip, not sync
/// lag. With the floor applied the same live data yields 1,167 in-era legacy
/// receipts against 1,154 canonical rows — a ~13-row first-enable find an
/// operator can actually act on.
///
/// The floor is DERIVED, never configured. `MIN(pay_date)` is by
/// construction the oldest receipt the mirror has ever landed
/// (`apply_receipt_upsert` seeds `pay_date` from `Receipt_Date` and COALESCEs
/// it forever after), so nothing that could still converge sorts below it.
/// `date_trunc('day', …)` widens to the start of that day so the boundary
/// includes the whole first day rather than cutting mid-afternoon.
///
/// **NO timezone shift is applied, and adding one would be a bug** (2026-07-28
/// review, second pass). `ht_payments.pay_date` is a bare `TIMESTAMP` carrying
/// the legacy value VERBATIM: `project_receipt` reads `Receipt_Date` with a
/// plain `try_get_datetime` and no conversion, and `apply_receipt_upsert`'s own
/// fallback is explicitly Bangkok wall-clock ("a `naive_utc()` fallback here
/// landed 7h early — 2026-06-11 audit"). So both sides of this comparison are
/// already the same naive Thai basis. (`naive_thai_to_utc` belongs to a
/// DIFFERENT column with a different convention: `Cin_Pay_Date` →
/// `ht_payment_ledger.ledger_pay_date`, which is `TIMESTAMPTZ`. Conflating the
/// two conventions is what put a spurious `+ INTERVAL '7 hours'` here.)
/// Verified live read-only: hotelnew `MIN(pay_date)` = `2026-04-27 16:39:21`
/// (`pay_reference` `B2604-0285`) and legacy `HT_Receipt_H.Receipt_Date` for
/// that receipt = `2026-04-27T16:39:21`, byte-identical.
///
/// A shift here would move the floor in the NARROWING direction and silently
/// drop the mirror's whole first day of coverage — precisely the
/// partial-ingest boundary where the genuine `missing_pg` finds live. It was
/// masked at both sites only because `date_trunc('day')` happened to absorb it
/// (HF Hotel 16:39 + 7h = 23:39, same day, 21 minutes of margin); any site
/// whose oldest mirrored receipt lands at or after 17:00 Thai loses a full day.
/// Unshifted is also the safe direction if an APP-created row ever became the
/// `MIN`: `repository/payment.rs` omits `pay_date` on INSERT so the column
/// `DEFAULT NOW()` applies on a UTC-basis server clock, which errs wide (floor
/// too early → extra rows scanned) rather than narrow (rows silently dropped).
const PAYMENTS_ERA_FLOOR_SQL: &str = "SELECT date_trunc('day', MIN(pay_date)) FROM ht_payments";

async fn payments_reconcile_era_floor(
    pg_pool: &PgPool,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    sqlx::query_scalar::<_, Option<NaiveDateTime>>(PAYMENTS_ERA_FLOOR_SQL)
        .fetch_one(pg_pool)
        .await
}

/// Compose the bulk scan's `WHERE` clause for a given canonical era floor.
///
/// Rows with a NULL `Receipt_Date` are deliberately KEPT: such a receipt
/// cannot be placed inside or outside the era, it is vanishingly rare (0 of
/// 21,566 at HF Hotel and 0 of 105 at HF Ville, verified 2026-07-28), and the
/// mapper WOULD land it (`p.receipt_date.unwrap_or(now)`) — so dropping it
/// would re-create the silent-skip class the floor exists to remove.
///
/// The literal is rendered `YYYY-MM-DDTHH:MM:SS`, the language-independent
/// ODBC/ISO form, so the comparison can't be re-read under a different server
/// `DATEFORMAT`. No injection surface: the value is a `NaiveDateTime`
/// PostgreSQL itself produced, formatted here.
fn payments_reconcile_scan_filter(era_floor: NaiveDateTime) -> String {
    format!(
        "{base} AND (Receipt_Date IS NULL OR Receipt_Date >= '{floor}')",
        base = PAYMENTS_RECONCILE_SCAN_FILTER,
        floor = era_floor.format("%Y-%m-%dT%H:%M:%S"),
    )
}

/// One legacy receipt as projected for reconciliation. Mirrors the CT
/// mapper's `ReceiptProjection` field-for-field on the hashed subset, so
/// the descriptor table in `sync::mappers::payment` and this loop cannot
/// disagree about the body.
struct LegacyReceiptRow {
    receipt_no: String,
    receipt_total: f64,
    legacy_cin_no: Option<String>,
    status_name: Option<String>,
}

impl LegacyReceiptRow {
    fn voided(&self) -> bool {
        // Single-sourced with the mapper's own void decision so detection
        // and application can never disagree on the cancel literal.
        crate::sync::mappers::payment::receipt_status_is_cancelled(self.status_name.as_deref())
    }

    fn hash(&self) -> String {
        payment_canonical_hash(
            &self.receipt_no,
            self.receipt_total,
            self.voided(),
            self.legacy_cin_no.as_deref(),
        )
    }

    fn json(&self) -> serde_json::Value {
        json!({
            "Receipt_no": self.receipt_no,
            "Receipt_Total": self.receipt_total,
            "Receipt_ref": self.legacy_cin_no,
            "status_name": self.status_name,
        })
    }
}

/// Project one `HT_Receipt_H` row under [`PAYMENTS_RECONCILE_PROJECTION`].
/// Returns `None` for a row that is not reconcilable — no `Receipt_no`
/// (the business key), or an empty/absent `Receipt_ref` (the deliberate
/// mapper skip). Shared by the bulk scan and the per-PK re-fetch so the
/// two apply IDENTICAL admission rules.
fn project_legacy_receipt_row(row: &tiberius::Row) -> Option<LegacyReceiptRow> {
    let receipt_no = row.get::<&str, _>("Receipt_no")?.to_string();
    if receipt_no.is_empty() {
        return None;
    }
    let legacy_cin_no = row
        .get::<&str, _>("Receipt_ref")
        .map(str::to_string)
        .filter(|s| !s.is_empty())?;
    Some(LegacyReceiptRow {
        receipt_no,
        // `Receipt_Total` is `float NOT NULL DEFAULT 0` in the live
        // schema; a NULL would only appear via a hand-edit, and 0.0 is
        // the honest projection of "no total recorded".
        receipt_total: row.get::<f64, _>("Receipt_Total").unwrap_or(0.0),
        legacy_cin_no: Some(legacy_cin_no),
        status_name: row.get::<&str, _>("status_name").map(str::to_string),
    })
}

/// Canonical-side projection of a payment row for hashing. Resolved by
/// `Receipt_no` through the SAME two columns `apply_receipt_upsert`
/// probes.
struct CanonicalPaymentRow {
    pay_amount: f64,
    pay_voided: Option<bool>,
    /// From the parent check-in (`ht_checkins.legacy_cin_no`), which is
    /// what legacy `Receipt_ref` holds. LEFT-joined: a payment whose
    /// parent row has vanished still projects (with `None`) and lands as
    /// value drift rather than being misreported as `missing_pg`.
    legacy_cin_no: Option<String>,
}

impl CanonicalPaymentRow {
    /// `pay_voided` is nullable (`BOOLEAN DEFAULT false`); NULL means
    /// "never voided", matching the `COALESCE(pay_voided, false)` the
    /// mapper's UPDATE and every `WHERE pay_voided = false` reader use.
    fn is_voided(&self) -> bool {
        self.pay_voided.unwrap_or(false)
    }
}

/// Resolve the canonical payment for a legacy `Receipt_no`.
///
/// The predicate + ORDER BY mirror
/// `payment::apply_receipt_upsert`'s existing-row probe exactly, minus its
/// `pay_cin_id` term (which the sweep does not have, and does not need —
/// `Receipt_no` is unique within the legacy app):
///
/// * `legacy_receipt_no` — stamped by OUR writeback back-population when
///   the payment originated in THIS app;
/// * `pay_reference` — set when the payment originated in iHOTEL and was
///   first imported here.
///
/// Probing only one column would report the other origin's rows as
/// `missing_pg` forever — the same defect that produced the 2026-06-30
/// HF Ville phantom-duplicate echo, in detection form. The `ORDER BY`
/// makes the app-originated row win deterministically if a legacy orphan
/// duplicate still exists.
///
/// Hoisted to a const so the shape guard executes the EXACT statement the
/// sweep runs, rather than a re-typed copy that could drift (same reason
/// `payment::RECEIPT_UPSERT_UPDATE_SQL` is a const).
const CANONICAL_PAYMENT_PROBE_SQL: &str =
    "SELECT p.pay_amount::float8, p.pay_voided, c.legacy_cin_no \
       FROM ht_payments p \
       LEFT JOIN ht_checkins c ON c.cin_id = p.pay_cin_id \
      WHERE p.legacy_receipt_no = $1 OR p.pay_reference = $1 \
      ORDER BY (p.legacy_receipt_no = $1) DESC NULLS LAST, p.pay_id ASC \
      LIMIT 1";

async fn fetch_canonical_payment(
    pg_pool: &PgPool,
    receipt_no: &str,
) -> Result<Option<CanonicalPaymentRow>, sqlx::Error> {
    sqlx::query_as::<_, (f64, Option<bool>, Option<String>)>(CANONICAL_PAYMENT_PROBE_SQL)
        .bind(receipt_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(amount, voided, cin_no)| CanonicalPaymentRow {
            pay_amount: amount,
            pay_voided: voided,
            legacy_cin_no: cin_no,
        })
    })
}

/// Best-effort ack: record the `mssql_hash` we last reconciled for this
/// receipt so the next tick short-circuits before the per-PK canonical
/// fetch. Cache-only — never mutates canonical state; a failed write just
/// re-fires the same comparison next tick.
async fn ack_receipt_mirror(pg_pool: &PgPool, receipt_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_receipts_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE receipt_no = $2",
    )
    .bind(mssql_hash)
    .bind(receipt_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_receipts_legacy (receipt_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (receipt_no) DO UPDATE SET sync_hash = EXCLUDED.sync_hash, \
                                                    synced_at = EXCLUDED.synced_at",
        )
        .bind(receipt_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

/// Read the WHOLE ack cache in ONE query.
///
/// The efficiency contract for this arm is: one bulk MSSQL SELECT + one
/// batched ack read + a per-PK canonical fetch ONLY for keys whose hash
/// moved. Per-PK ack SELECTs (what `sync_customers` / `sync_checkins` do)
/// would add one PG round-trip per receipt — tens of thousands per tick
/// for a table that is append-only and therefore almost entirely acked in
/// steady state.
async fn load_receipt_ack_cache(
    pg_pool: &PgPool,
) -> Result<std::collections::HashMap<String, String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, Option<String>)>(
        "SELECT receipt_no, sync_hash FROM ht_receipts_legacy WHERE sync_hash IS NOT NULL",
    )
    .fetch_all(pg_pool)
    .await?;
    Ok(rows
        .into_iter()
        .filter_map(|(k, v)| v.map(|hash| (k, hash)))
        .collect())
}

/// Receipt keys whose CANONICAL payment currently carries the void bit,
/// read in ONE batched query per tick.
///
/// This is what keeps the canonical-only void shape observable past the
/// first enabled tick — see [`payment_canonical_hash`] for the full
/// argument and [`payment_ack_short_circuit_bypassed`] for the rule.
///
/// BOTH lookup columns are unioned because [`CANONICAL_PAYMENT_PROBE_SQL`]
/// resolves on either (`legacy_receipt_no = $1 OR pay_reference = $1`) and
/// one row can carry two different values. Over-inclusion is harmless: the
/// worst case is one extra canonical probe for a receipt that then agrees.
///
/// Voided payments are a small minority of a small table, so this stays
/// well inside the arm's efficiency contract (one bulk MSSQL SELECT + a
/// couple of batched PG reads per tick).
async fn load_canonically_voided_receipt_keys(
    pg_pool: &PgPool,
) -> Result<std::collections::HashSet<String>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, String>(
        "SELECT legacy_receipt_no FROM ht_payments \
          WHERE COALESCE(pay_voided, false) AND legacy_receipt_no IS NOT NULL \
         UNION \
         SELECT pay_reference FROM ht_payments \
          WHERE COALESCE(pay_voided, false) AND pay_reference IS NOT NULL",
    )
    .fetch_all(pg_pool)
    .await?;
    Ok(rows.into_iter().collect())
}

/// Must the ack short-circuit be BYPASSED for this receipt?
///
/// True for exactly one divergence shape: canonical says voided, legacy
/// still says normal. That shape cannot move the legacy hash, so an acked
/// receipt would otherwise never be re-compared and the divergence would be
/// invisible forever (see [`payment_canonical_hash`]).
///
/// Deliberately NOT `canonical_voided != legacy_voided`: when legacy is the
/// one carrying the cancel literal, the legacy hash HAS moved, so the plain
/// ack comparison already re-opens the receipt on its own.
fn payment_ack_short_circuit_bypassed(canonical_voided: bool, legacy_voided: bool) -> bool {
    canonical_voided && !legacy_voided
}

/// Phase 6-A payments reconcile arm. Compares legacy `HT_Receipt_H`
/// against canonical `ht_payments`, keyed on `Receipt_no`.
///
/// Only ever called when [`reconcile_payments_arm_enabled`] is true — with
/// the flag off (the shipped default on every service) this function is
/// never entered, so the arm issues no queries at all.
///
/// Shape (see [`load_receipt_ack_cache`] for the efficiency contract):
/// 0. ONE PG read for the canonical era floor ([`PAYMENTS_ERA_FLOOR_SQL`]) —
///    receipts older than the mirror's own coverage are out of scope, exactly
///    like ref-less ones, and scanning them would build a permanently
///    unresolvable `missing_pg` backlog;
/// 1. ONE bulk MSSQL SELECT over the filtered receipt set;
/// 2. ONE batched read of the `ht_receipts_legacy` ack cache, plus ONE
///    batched read of the canonically-voided receipt keys
///    ([`load_canonically_voided_receipt_keys`]);
/// 3. per-PK canonical fetch ONLY for receipts whose legacy hash differs
///    from the acked one, or which hit the canonical-only-void carve-out.
///
/// `ReconcileMode::Upsert` is not honoured here: that pre-5.5 escape hatch
/// mirrored data columns into `ht_*_legacy`, and `ht_receipts_legacy` is a
/// pure ack cache with no data columns to mirror. The arm is diff-only by
/// construction.
async fn sync_payments(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing payments (receipts)...");

    // Canonical coverage floor FIRST. With `ht_payments` empty the arm has
    // no coverage at all, and scanning legacy would classify the ENTIRE
    // receipt history as `missing_pg` — permanently unresolvable, since
    // `payments` is in neither self-heal list. Report a clean zero tick
    // instead of manufacturing a backlog. See `PAYMENTS_ERA_FLOOR_SQL`.
    let Some(era_floor) = payments_reconcile_era_floor(pg_pool).await? else {
        tracing::warn!(
            "[Sync] sync_payments: ht_payments is empty — no canonical coverage \
             to reconcile against; skipping the legacy scan this tick"
        );
        let duration_ms = start.elapsed().as_millis() as i32;
        record_success(pg_pool, "payments", 0, 0, 0, duration_ms).await;
        return Ok(());
    };

    let mut conn = legacy_pool.get().await?;
    let select_sql = format!(
        "SELECT {projection} FROM HT_Receipt_H WHERE {filter} ORDER BY Receipt_no",
        projection = PAYMENTS_RECONCILE_PROJECTION.join(", "),
        filter = payments_reconcile_scan_filter(era_floor),
    );
    let rows = simple_query_with_timeout_pooled(&mut conn, &select_sql, MssqlOpKind::Read).await?;
    // Free the pool slot — nothing below touches MSSQL again.
    drop(conn);

    let acked = load_receipt_ack_cache(pg_pool).await?;
    let canonically_voided = load_canonically_voided_receipt_keys(pg_pool).await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;
    let mut skipped = 0i32;

    for row in &rows {
        let Some(legacy) = project_legacy_receipt_row(row) else {
            // Not reconcilable (no business key / no `Receipt_ref`). The
            // filter above already excludes the ref-less case; this is the
            // belt-and-braces arm.
            skipped += 1;
            continue;
        };
        let mssql_hash = legacy.hash();

        // Dedupe: identical hash as last acknowledged means the drift (if
        // any) is already in `ht_reconcile_log`. This is what keeps the
        // steady-state cost at one MSSQL query + a few batched PG reads.
        //
        // ONE carve-out: a canonical-only void never moves the LEGACY hash,
        // so an acked receipt would never be re-compared and the money-path
        // divergence would go unseen forever. Re-open exactly that pair.
        let void_carve_out = payment_ack_short_circuit_bypassed(
            canonically_voided.contains(&legacy.receipt_no),
            legacy.voided(),
        );
        if !void_carve_out && acked.get(&legacy.receipt_no) == Some(&mssql_hash) {
            unchanged += 1;
            continue;
        }

        let canonical = fetch_canonical_payment(pg_pool, &legacy.receipt_no).await?;
        let canonical_hash = canonical.as_ref().map(|c| {
            payment_canonical_hash(
                &legacy.receipt_no,
                c.pay_amount,
                c.is_voided(),
                c.legacy_cin_no.as_deref(),
            )
        });

        if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
            ack_receipt_mirror(pg_pool, &legacy.receipt_no, &mssql_hash).await;
            unchanged += 1;
            continue;
        }

        // Receipts are 1:1 on both sides (`Receipt_no` is unique in the
        // legacy app, and the canonical probe resolves at most one row),
        // so the counts are 0/1 by construction and `Cardinality` is not
        // reachable — the `pg_row_count == 0` case IS the `missing_pg`
        // path.
        let legacy_row_count: i32 = 1;
        let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
        let kind = classify_divergence(
            canonical_hash.as_deref(),
            Some(&mssql_hash),
            legacy_row_count,
            pg_row_count,
        );
        let pg_json = canonical.as_ref().map(|c| {
            json!({
                "pay_amount": c.pay_amount,
                "pay_voided": c.is_voided(),
                "legacy_cin_no": c.legacy_cin_no,
            })
        });
        record_divergence(
            pg_pool,
            "payments",
            &legacy.receipt_no,
            canonical_hash.as_deref(),
            Some(&mssql_hash),
            legacy.json(),
            pg_json,
            kind,
            legacy_row_count,
            pg_row_count,
        )
        .await;
        // Track D / T7 CRIT-1: value drift acks (one row per distinct
        // legacy state); `missing_pg` never does, so it re-fires every
        // tick until canonical actually catches up.
        if kind.is_silenceable() {
            ack_receipt_mirror(pg_pool, &legacy.receipt_no, &mssql_hash).await;
        }
        if canonical.is_none() {
            added += 1;
        } else {
            updated += 1;
        }
    }

    if skipped > 0 {
        tracing::warn!(
            skipped,
            "[Sync] sync_payments: {} receipts were not reconcilable \
             (missing Receipt_no / Receipt_ref despite the scan filter)",
            skipped,
        );
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        era_floor = %era_floor,
        scanned = rows.len(),
        "[Sync] Payments: {} missing-canonical, {} drifted, {} unchanged in {}ms \
         (in-era scan from {})",
        added,
        updated,
        unchanged,
        duration_ms,
        era_floor,
    );
    record_success(pg_pool, "payments", added, updated, unchanged, duration_ms).await;

    Ok(())
}

// =============================================================================
// Guest-registry (companion folio) Sync — Phase 6-B, DARK behind
// RECONCILE_GUEST_REGISTRY_ARM_ENABLED
// =============================================================================

/// Legacy `HT_CheckIn_Other_People` projection for the folio reconcile
/// hash. Same three columns the CT mapper reads, minus the IDENTITY `id`
/// (excluded from the hash on purpose — see
/// [`guest_registry_canonical_hash`]).
///
/// Held as a slice const so Track J1's projection-lock test can pin every
/// column against the authoritative schema dump, and so the bulk scan and
/// the per-PK auto-resolve re-fetch cannot drift apart. The iHOTEL typo
/// `Cin_contry` (sic) is preserved verbatim, and `Cin_no` is LOWERCASE-n
/// here (`HT_CheckIn_Ds` is the one with `Cin_No`).
const GUEST_REGISTRY_RECONCILE_PROJECTION: &[&str] = &["Cin_no", "Cin_name", "Cin_contry"];

/// Canonical-side companion filter: mirrored companions only.
///
/// `HT_CheckIn_Other_People` holds ONLY companions — the primary guest
/// lives on the check-in header — and the CT mapper inserts
/// `guest_is_primary = false` for every row it lands. The check-in
/// registration feature (migration 070, Thai-ID capture) writes PRIMARY
/// rows into the same canonical table, so without this filter every
/// registered primary guest would look like an extra companion the legacy
/// side is missing. `COALESCE` because the column is nullable
/// (`BOOLEAN DEFAULT false`) and a bare `= false` would silently drop a
/// NULL row out of the canonical folio, reporting it as a legacy-only
/// companion forever.
const CANONICAL_COMPANION_PRIMARY_FILTER: &str = "COALESCE(guest_is_primary, false) = false";

/// The two canonical companion fields, projected into the shape the legacy
/// side produces: the re-concatenated display name and the COALESCEd
/// country.
///
/// The name expression is single-sourced from
/// [`crate::sync::mappers::guest_registry::CANONICAL_COMPANION_NAME_SQL`] —
/// the SAME bytes the mapper's echo-adoption match uses. If the two ever
/// diverged, a companion our app created and the writeback echoed back
/// would be adopted correctly by the mapper yet hash differently here:
/// permanent, unfixable sync lag on a legally load-bearing table.
fn canonical_companion_projection() -> String {
    format!(
        "{name} AS companion_name, COALESCE(guest_nationality, '') AS companion_country",
        name = CANONICAL_COMPANION_NAME_SQL,
    )
}

/// Derive the canonical-coverage floor for the guest-registry scan, as a
/// check-in timestamp. `NULL` when canonical holds no mirrored companion at
/// all (no coverage).
///
/// **Why this exists** — the same BLOCKING class the payments arm hit.
/// `ht_guest_registry` is CT-populated with no historical backfill (Track
/// E1 enabled CT on `HT_CheckIn_Other_People` in May 2026), while
/// `ht_checkins` IS fully backfilled to 2021. So every pre-CT folio has a
/// canonical parent check-in but no canonical companions, and would be
/// reported as a divergence that can NEVER close. Live counts 2026-07-28:
/// HF Hotel 20,434 legacy companion rows across 20,423 folios vs 819
/// canonical companions (oldest parent check-in 2026-05-13); HF Ville 2,185
/// / 2,184 vs 545 (oldest 2026-05-13). Unfloored, the first enabled tick
/// would manufacture ~19.6k + ~1.6k permanently-open rows, re-log them on
/// every tick, pin the 4h digest and the >72h escalation tier on
/// `guest_registry`, and starve every other entity out of
/// [`auto_resolve_reconcile_log`]'s age-only 500-row batch. Floored, the
/// same live data yields 830 in-era legacy folios vs 818 canonical at HF
/// Hotel (≈12 actionable finds) and 574 vs 545 at Ville (≈29).
///
/// The floor is DERIVED, never configured: `MIN(cin_checkin_time)` over the
/// check-ins that actually carry a MIRRORED companion IS the oldest folio
/// the mirror has ever landed, so nothing that could still converge sorts
/// below it. `date_trunc('day', …)` widens to the start of that day so the
/// boundary includes the mirror's whole first day rather than cutting
/// mid-afternoon.
///
/// **`guest_legacy_id IS NOT NULL` is load-bearing, not decoration.**
/// "Mirrored" means *stamped with a legacy IDENTITY by the CT mapper*.
/// Canonical holds non-primary companions with NO legacy counterpart —
/// `POST /api/checkins/{id}/guests` and the migration-070 registration
/// capture both write them, and `TM30_COMPANION_WRITEBACK_ENABLED` is
/// compose-default false, so nothing pushes them to legacy. Counting those
/// as "coverage" would claim an era the mirror never actually covered.
///
/// **The result is CLAMPED to a persisted, non-decreasing watermark** — see
/// [`clamped_era_floor`] and [`RECONCILE_ERA_FLOOR_UPSERT_SQL`]. A raw
/// `MIN()` is a low-water mark on the PARENT's check-in time, and it can be
/// dragged backwards by ONE row: iHOTEL's DELETE+REINSERT companion edit
/// (`FrmCheckIn.cs:9975`) applied to any historical folio makes the CT
/// mapper mirror one companion whose parent check-in is e.g. 2023 (the
/// mapper resolves the parent by `legacy_cin_no` with no era restriction,
/// and `ht_checkins` is backfilled to 2021). That single row would move the
/// floor to 2023, admit ~all 20,423 legacy folios instead of 830, and make
/// the next tick enqueue ~19.6k permanently-open rows — the exact flood
/// this floor exists to prevent. The persisted watermark makes the scope
/// monotonically NARROWING; [`divergence_cap_exceeded`] is the second belt,
/// for the case where the very first (bootstrap) reading is already wrong.
///
/// **No timezone shift, by construction** — unlike the payments floor this
/// one never crosses a DB boundary: it is derived from
/// `ht_checkins.cin_checkin_time` and compared against that same column, so
/// both sides are the same naive Thai basis whatever that basis is. The
/// legacy side is filtered by KEY membership (`Cin_no` ∈ the in-era
/// canonical set), never by a legacy date, so there is no second clock.
fn guest_registry_era_floor_sql() -> String {
    format!(
        "SELECT date_trunc('day', MIN(ht_checkins.cin_checkin_time)) \
           FROM ht_guest_registry \
           JOIN ht_checkins ON ht_checkins.cin_id = ht_guest_registry.guest_cin_id \
          WHERE {primary} \
            AND ht_guest_registry.guest_legacy_id IS NOT NULL",
        primary = CANONICAL_COMPANION_PRIMARY_FILTER,
    )
}

/// `ht_reconcile_era_floor` key for this arm. Same literal as the
/// `ht_reconcile_log.table_name` / `sync_status.entity_type` the arm reports
/// under, so one operator query joins all three.
const GUEST_REGISTRY_ERA_FLOOR_KEY: &str = "guest_registry";

/// Persist-and-clamp in ONE statement: the durable floor only ever moves
/// FORWARD.
///
/// `GREATEST` lives in SQL rather than in Rust on purpose — the backend
/// scheduler and `bin/sync` can both run a tick against the same database,
/// so the monotonic guarantee has to hold under concurrency, not just
/// within one process. `RETURNING` hands back the post-clamp value, so the
/// read and the write are the same round trip.
///
/// An operator CAN still move the floor forward by hand
/// (`UPDATE ht_reconcile_era_floor SET era_floor = … WHERE table_name =
/// 'guest_registry'`) — that is the documented remedy when a bootstrap
/// reading came out too low — and `GREATEST` makes the edit stick.
const RECONCILE_ERA_FLOOR_UPSERT_SQL: &str =
    "INSERT INTO ht_reconcile_era_floor (table_name, era_floor) VALUES ($1, $2) \
     ON CONFLICT (table_name) DO UPDATE \
        SET era_floor = GREATEST(ht_reconcile_era_floor.era_floor, EXCLUDED.era_floor), \
            updated_at = NOW() \
     RETURNING era_floor";

/// Read the durable floor without writing — used only when the derived
/// floor is NULL (nothing to clamp with).
const RECONCILE_ERA_FLOOR_SELECT_SQL: &str =
    "SELECT era_floor FROM ht_reconcile_era_floor WHERE table_name = $1";

/// The clamp semantics, as a pure function (the SQL above enforces the same
/// rule atomically; this is the spec the tests pin).
///
/// * both present → the LATER one. A derived floor that dropped below the
///   watermark is exactly the one-old-companion drag described on
///   [`guest_registry_era_floor_sql`]; ignore it.
/// * persisted only (derived went NULL — every mirrored companion deleted,
///   or the table truncated) → KEEP the watermark. Widening back to "no
///   coverage" would be the same flood by another route; the in-era folios
///   then all read as divergent and [`divergence_cap_exceeded`] raises a
///   page, which is the correct response to a mirror that vanished.
/// * neither → `None`: no coverage was ever established, and the arm skips
///   the legacy scan entirely.
fn clamped_era_floor(
    persisted: Option<NaiveDateTime>,
    derived: Option<NaiveDateTime>,
) -> Option<NaiveDateTime> {
    match (persisted, derived) {
        (Some(p), Some(d)) => Some(p.max(d)),
        (Some(p), None) => Some(p),
        (None, d) => d,
    }
}

async fn guest_registry_era_floor(
    pg_pool: &PgPool,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    // `AssertSqlSafe`: the statement is assembled from compile-time consts
    // only (no runtime value reaches it) — same audit note as the sibling
    // canonical-folio statements below.
    let derived = sqlx::query_scalar::<_, Option<NaiveDateTime>>(sqlx::AssertSqlSafe(
        guest_registry_era_floor_sql(),
    ))
    .fetch_one(pg_pool)
    .await?;

    let persisted = match derived {
        Some(d) => {
            sqlx::query_scalar::<_, NaiveDateTime>(RECONCILE_ERA_FLOOR_UPSERT_SQL)
                .bind(GUEST_REGISTRY_ERA_FLOOR_KEY)
                .bind(d)
                .fetch_optional(pg_pool)
                .await?
        }
        None => {
            sqlx::query_scalar::<_, NaiveDateTime>(RECONCILE_ERA_FLOOR_SELECT_SQL)
                .bind(GUEST_REGISTRY_ERA_FLOOR_KEY)
                .fetch_optional(pg_pool)
                .await?
        }
    };

    let effective = clamped_era_floor(persisted, derived);

    // Say it out loud when the watermark is HOLDING the scope forward: that
    // means a historical folio just gained a mirrored companion, which is a
    // legitimate iHOTEL edit but would otherwise silently widen the scan by
    // years.
    match (effective, derived) {
        (Some(e), Some(d)) if e > d => tracing::info!(
            derived_floor = %d,
            effective_floor = %e,
            "[Sync] sync_guest_registry: derived coverage floor sits BEHIND the \
             persisted watermark (a pre-era folio gained a mirrored companion) — \
             holding the watermark; scope stays monotonically narrowing"
        ),
        (Some(e), None) => tracing::warn!(
            effective_floor = %e,
            "[Sync] sync_guest_registry: canonical now holds NO mirrored companion \
             at all, but a coverage watermark exists — keeping it. Expect a \
             divergence-cap page if the mirror really was lost"
        ),
        _ => {}
    }

    Ok(effective)
}

/// The in-era folio KEY set: every canonical check-in from the coverage
/// floor onward.
///
/// This is the arm's scope gate, and it does double duty:
///
/// * it drops pre-coverage folios (see [`guest_registry_era_floor_sql`]);
/// * it drops folios whose parent check-in is absent from canonical
///   entirely. Those are a CHECK-INS problem — `sync_checkins` already
///   reports them as `missing_pg` — and the companion mapper could not land
///   them anyway (it ERRORS on an unresolvable parent FK). Reporting them
///   here too would double-count one root cause and manufacture rows this
///   arm can never close.
///
/// `cin_checkin_time` is `NOT NULL` in the canonical schema, so there is no
/// NULL arm to reason about (verified live 2026-07-28: 0 NULLs at HF Hotel).
const IN_ERA_CHECKIN_KEYS_SQL: &str = "SELECT legacy_cin_no FROM ht_checkins \
      WHERE legacy_cin_no IS NOT NULL AND cin_checkin_time >= $1";

async fn load_in_era_checkin_keys(
    pg_pool: &PgPool,
    era_floor: NaiveDateTime,
) -> Result<std::collections::HashSet<String>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, String>(IN_ERA_CHECKIN_KEYS_SQL)
        .bind(era_floor)
        .fetch_all(pg_pool)
        .await?;
    Ok(rows.into_iter().collect())
}

/// Every canonical companion folio in the coverage era, in ONE query.
///
/// Joins companion → check-in (never the reverse), so a duplicate
/// `legacy_cin_no` on `ht_checkins` cannot duplicate companion lines.
fn canonical_registry_folios_sql() -> String {
    format!(
        "SELECT ht_checkins.legacy_cin_no, {projection} \
           FROM ht_guest_registry \
           JOIN ht_checkins ON ht_checkins.cin_id = ht_guest_registry.guest_cin_id \
          WHERE {primary} \
            AND ht_checkins.legacy_cin_no IS NOT NULL \
            AND ht_checkins.cin_checkin_time >= $1",
        projection = canonical_companion_projection(),
        primary = CANONICAL_COMPANION_PRIMARY_FILTER,
    )
}

async fn load_canonical_registry_folios(
    pg_pool: &PgPool,
    era_floor: NaiveDateTime,
) -> Result<BTreeMap<String, RegistryFolioProjection>, sqlx::Error> {
    // `AssertSqlSafe`: built purely from compile-time consts
    // (`canonical_companion_projection` / `CANONICAL_COMPANION_PRIMARY_FILTER`);
    // the only runtime value is the era floor, which is BOUND as `$1`.
    let rows = sqlx::query_as::<_, (String, Option<String>, String)>(sqlx::AssertSqlSafe(
        canonical_registry_folios_sql(),
    ))
    .bind(era_floor)
    .fetch_all(pg_pool)
    .await?;
    let mut folios: BTreeMap<String, RegistryFolioProjection> = BTreeMap::new();
    for (cin_no, name, country) in rows {
        folios
            .entry(cin_no.clone())
            .or_insert_with(|| RegistryFolioProjection::empty(cin_no))
            .push_companion(name.as_deref().unwrap_or_default(), Some(country.as_str()));
    }
    Ok(folios)
}

/// Resolve the canonical check-in id for a legacy `Cin_no`.
///
/// Byte-identical to the companion mapper's own parent lookup
/// (`guest_registry.rs`), so the sweep resolves the SAME folio the mapper
/// would write into.
const CANONICAL_CHECKIN_ID_PROBE_SQL: &str =
    "SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1 LIMIT 1";

/// Per-PK canonical folio for the auto-resolve sweep.
///
/// `Ok(None)` ⇔ the parent check-in is absent from canonical: the folio is
/// out of this arm's scope (see [`IN_ERA_CHECKIN_KEYS_SQL`]) and the row
/// stays open for operator review. A folio that EXISTS but holds no
/// companions returns `Ok(Some(<empty folio>))`, which is what lets a
/// companion set deleted on both sides converge.
async fn fetch_canonical_registry_folio(
    pg_pool: &PgPool,
    cin_no: &str,
) -> Result<Option<RegistryFolioProjection>, sqlx::Error> {
    let cin_id: Option<i32> = sqlx::query_scalar(CANONICAL_CHECKIN_ID_PROBE_SQL)
        .bind(cin_no)
        .fetch_optional(pg_pool)
        .await?;
    let Some(cin_id) = cin_id else {
        return Ok(None);
    };
    let sql = format!(
        "SELECT {projection} FROM ht_guest_registry \
          WHERE guest_cin_id = $1 AND {primary}",
        projection = canonical_companion_projection(),
        primary = CANONICAL_COMPANION_PRIMARY_FILTER,
    );
    // `AssertSqlSafe`: consts only; `cin_id` is bound as `$1`.
    let rows = sqlx::query_as::<_, (Option<String>, String)>(sqlx::AssertSqlSafe(sql))
        .bind(cin_id)
        .fetch_all(pg_pool)
        .await?;
    let mut folio = RegistryFolioProjection::empty(cin_no);
    for (name, country) in rows {
        folio.push_companion(name.as_deref().unwrap_or_default(), Some(country.as_str()));
    }
    Ok(Some(folio))
}

/// Project one legacy companion row into a folio. Shared by the bulk scan
/// and the per-PK re-fetch so the two apply IDENTICAL admission rules.
/// A NULL `Cin_name` lands as the empty string — exactly what the CT mapper
/// stores (`cin_name.unwrap_or_default()`), so a blank "Other People" row
/// saved by a receptionist tabbing through hashes the same on both sides.
fn push_legacy_companion(folio: &mut RegistryFolioProjection, row: &tiberius::Row) {
    folio.push_companion(
        row.get::<&str, _>("Cin_name").unwrap_or_default(),
        row.get::<&str, _>("Cin_contry"),
    );
}

fn registry_folio_json(folio: &RegistryFolioProjection) -> serde_json::Value {
    json!({
        "Cin_no": folio.legacy_cin_no,
        "companions": folio.companion_lines(),
        "companion_count": folio.len(),
    })
}

/// Best-effort ack: record the `mssql_hash` this arm last reconciled for a
/// folio. Cache-only — never mutates canonical state; a failed write just
/// re-runs the (already in-memory) comparison next tick.
async fn ack_guest_registry_mirror(pg_pool: &PgPool, cin_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_guest_registry_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE cin_no = $2",
    )
    .bind(mssql_hash)
    .bind(cin_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_guest_registry_legacy (cin_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (cin_no) DO UPDATE SET sync_hash = EXCLUDED.sync_hash, \
                                                synced_at = EXCLUDED.synced_at",
        )
        .bind(cin_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

/// Read the WHOLE ack cache in ONE query, same efficiency contract as the
/// payments arm's.
async fn load_guest_registry_ack_cache(
    pg_pool: &PgPool,
) -> Result<std::collections::HashMap<String, String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, Option<String>)>(
        "SELECT cin_no, sync_hash FROM ht_guest_registry_legacy WHERE sync_hash IS NOT NULL",
    )
    .fetch_all(pg_pool)
    .await?;
    Ok(rows
        .into_iter()
        .filter_map(|(k, v)| v.map(|hash| (k, hash)))
        .collect())
}

/// Should this folio's ack row be (re)written?
///
/// **The ack cache in this arm suppresses WRITES; it never gates
/// DETECTION** — and that difference is deliberate. Its siblings ack to
/// skip a per-PK canonical fetch, which is why the payments arm needed an
/// explicit carve-out to keep canonical-only voids observable
/// ([`payment_ack_short_circuit_bypassed`]). Here BOTH sides are already
/// resident in memory (two batched reads), so gating the comparison on the
/// ack would buy nothing and would re-create that blind spot in a worse
/// form: any canonical-side change that leaves the LEGACY hash untouched —
/// a companion deleted from `ht_guest_registry`, a primary-flag flip, a
/// dropped CT delete — would become invisible forever on a table that
/// exists to satisfy a legal reporting obligation. So the loop compares
/// every in-scope folio on every tick and the ack row is written only when
/// the legacy hash has actually moved, which keeps the steady state at ~0
/// writes without costing a single observation.
fn guest_registry_ack_needs_write(acked: Option<&String>, mssql_hash: &str) -> bool {
    acked.map(String::as_str) != Some(mssql_hash)
}

/// Ceiling on how many divergences ONE `guest_registry` tick may enqueue.
///
/// 500 is not a round number picked for looks: it is
/// [`auto_resolve_reconcile_log`]'s per-tick `LIMIT 500`. Enqueuing more
/// findings in a tick than the sweep can even LOOK at in a tick is the
/// mechanism behind every flood incident this module has had — the backlog
/// never drains, the age-ordered batch fills with one entity, and every
/// other entity is starved out of both the sweep and the digest.
///
/// Steady-state expectation is 1–2 orders of magnitude below it: live
/// 2026-07-28, floored, HF Hotel has ~12 findings across 830 in-era folios
/// and HF Ville ~29 across 574.
const GUEST_REGISTRY_DIVERGENCE_CAP_DEFAULT: i64 = 500;

/// Resolve the per-tick divergence cap. Same per-site → global → default
/// chain as every other knob here ([`threshold_from_env`]), so a site
/// working through a genuine one-time backlog can be raised on its own:
/// `RECONCILE_GUEST_REGISTRY_MAX_DIVERGENCES_HFVILLE=…`.
fn guest_registry_divergence_cap(site_id: &str) -> i64 {
    threshold_from_env(
        "RECONCILE_GUEST_REGISTRY_MAX_DIVERGENCES",
        site_id,
        GUEST_REGISTRY_DIVERGENCE_CAP_DEFAULT,
    )
}

/// Would this tick enqueue more findings than the cap allows?
///
/// The arm compares BOTH sides in memory before it writes anything, so a
/// breach aborts the whole tick — no `record_divergence`, no ack writes —
/// instead of truncating the batch. Truncating would be worse than useless:
/// it would write an arbitrary 500 of the findings, leave the rest
/// invisible, and still pin the digest on `guest_registry`.
///
/// What a breach actually means, in order of likelihood: the coverage floor
/// has been dragged backwards (see [`guest_registry_era_floor_sql`] — the
/// persisted watermark should now prevent this), the companion mirror has
/// stopped ingesting, or the legacy table was bulk-edited. None of those are
/// fixed by writing 19.6k rows.
fn divergence_cap_exceeded(divergent: usize, cap: i64) -> bool {
    divergent as i64 > cap
}

/// Page an operator that a `guest_registry` tick was ABORTED by the
/// divergence cap, and say exactly which knob unblocks it.
///
/// Cooldown-gated through the shared `ht_level_drift_alert_cooldowns` table
/// under [`reconcile_cap_cooldown_key`] with the same per-site
/// `LEVEL_DRIFT_COOLDOWN_HOURS` window as the other reconcile pages, and —
/// like them — the cooldown is burned only on a confirmed delivery, so a
/// webhook outage cannot silence a stuck arm for a day.
async fn alert_guest_registry_divergence_cap(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
    divergent: usize,
    in_scope: usize,
    cap: i64,
    era_floor: NaiveDateTime,
) {
    let key = reconcile_cap_cooldown_key("guest_registry");
    let cooldown_hours = level_drift_thresholds_from_env(site_id).cooldown_hours;
    let cooldown = std::time::Duration::from_secs((cooldown_hours * 3600) as u64);

    if !level_alert_eligible_pg(pg_pool, site_id, &key, cooldown).await {
        tracing::debug!(
            site = %site_id,
            "[Sync] Guest-registry divergence-cap page suppressed by cooldown"
        );
        return;
    }

    let delivery = if let Some(slack) = slack {
        let msg = SlackMessage::with_site_text(
            site_id,
            format!(
                ":octagonal_sign: *Guest-registry reconcile ABORTED — divergence cap* \
                 :octagonal_sign:\n\
                 The companion-folio arm found *{divergent}* diverging folios out of \
                 {in_scope} in scope this tick, above the per-tick cap of {cap}. \
                 *Nothing was written* — no `ht_reconcile_log` rows, no ack rows — \
                 because a batch that size can never drain (the auto-resolve sweep \
                 looks at 500 rows per tick) and would starve every other entity out \
                 of the sweep and the digest.\n\
                 Coverage floor in force: `{era_floor}`.\n\
                 _Likely causes, in order: the coverage floor was dragged backwards by \
                 a companion edit on a pre-era folio (check \
                 `ht_reconcile_era_floor` where `table_name='guest_registry'` and move \
                 it FORWARD by hand — the upsert clamps with GREATEST, so the edit \
                 sticks); the companion CT mapper has stopped ingesting; or \
                 `HT_CheckIn_Other_People` was bulk-edited. Raise \
                 `RECONCILE_GUEST_REGISTRY_MAX_DIVERGENCES` only once you know the \
                 backlog is real, or set `RECONCILE_GUEST_REGISTRY_ARM_ENABLED=false` \
                 to stand the arm down. Per-site cooldown {cooldown_h}h._",
                cooldown_h = cooldown_hours,
            ),
        );
        AlertDelivery::from_send(Some(slack.send_message(&msg).await))
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; guest-registry divergence-cap abort logged only"
        );
        AlertDelivery::LoggedOnly
    };

    if cooldown_should_be_marked(delivery) {
        mark_level_alert_sent_pg(pg_pool, site_id, &key).await;
    } else {
        tracing::warn!(
            site = %site_id,
            "[Sync] Guest-registry divergence-cap page POST failed — leaving the \
             cooldown unset so the next tick retries"
        );
    }
}

/// Phase 6-B guest-registry reconcile arm. Compares legacy
/// `HT_CheckIn_Other_People` against canonical `ht_guest_registry` per
/// FOLIO (all companions sharing one `Cin_no`), keyed on `Cin_no`.
///
/// Only ever called when [`reconcile_guest_registry_arm_enabled`] is true —
/// with the flag off (the shipped default on every service) this function
/// is never entered, so the arm issues no queries at all.
///
/// Shape — 1 MSSQL query + 5 PG queries per tick, plus one ack write per
/// folio whose legacy state actually moved:
/// 0. the canonical coverage floor ([`guest_registry_era_floor_sql`]),
///    clamped against its durable watermark in a second, combined
///    read-write statement ([`RECONCILE_ERA_FLOOR_UPSERT_SQL`]);
/// 1. the in-era folio key set ([`IN_ERA_CHECKIN_KEYS_SQL`]) — the scope gate;
/// 2. every canonical companion in the era, grouped into folios;
/// 3. ONE bulk MSSQL scan of `HT_CheckIn_Other_People`, grouped into folios
///    and filtered against the key set;
/// 4. the ack cache.
///
/// The comparison runs over the UNION of both key sets, so a folio that
/// exists ONLY canonically (legacy companions all deleted, our CT delete
/// dropped or a companion we created that never reached legacy) is caught
/// too — a legacy-only scan would be blind to it.
///
/// **Compare-all-then-write, never write-as-you-go.** Both sides are
/// already in memory, so the tick decides its ENTIRE output before the
/// first row is enqueued. That is what lets [`divergence_cap_exceeded`]
/// abort a pathological tick outright instead of truncating it halfway
/// through a flood of `record_divergence` INSERTs and per-folio ack
/// round-trips.
///
/// `ReconcileMode::Upsert` is not honoured here, same as payments:
/// `ht_guest_registry_legacy` is a pure ack cache with no data columns to
/// mirror. The arm is diff-only by construction.
async fn sync_guest_registry(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    tracing::info!("[Sync] Syncing guest registry (companion folios)...");

    // Canonical coverage floor FIRST. With no mirrored companion at all the
    // arm has no coverage, and scanning legacy would classify the ENTIRE
    // companion history as divergent — permanently unresolvable, since
    // `guest_registry` is in neither self-heal list. Report a clean zero
    // tick instead of manufacturing a backlog.
    let Some(era_floor) = guest_registry_era_floor(pg_pool).await? else {
        tracing::warn!(
            "[Sync] sync_guest_registry: ht_guest_registry holds no mirrored \
             companion — no canonical coverage to reconcile against; skipping \
             the legacy scan this tick"
        );
        let duration_ms = start.elapsed().as_millis() as i32;
        record_success(pg_pool, "guest_registry", 0, 0, 0, duration_ms).await;
        return Ok(());
    };

    let in_era_keys = load_in_era_checkin_keys(pg_pool, era_floor).await?;
    let mut canonical = load_canonical_registry_folios(pg_pool, era_floor).await?;

    // ONE bulk legacy scan. `HT_CheckIn_Other_People` carries no date
    // column, so the era filter cannot be pushed into this WHERE — the
    // folios are filtered by KEY membership below instead. The table is
    // narrow and small (20,434 rows at HF Hotel, 2,185 at Ville on
    // 2026-07-28), so one full scan per 15-min tick is far cheaper than the
    // per-PK loads `sync_checkins` already runs.
    let mut conn = legacy_pool.get().await?;
    let select_sql = format!(
        "SELECT {projection} FROM HT_CheckIn_Other_People",
        projection = GUEST_REGISTRY_RECONCILE_PROJECTION.join(", "),
    );
    let rows = simple_query_with_timeout_pooled(&mut conn, &select_sql, MssqlOpKind::Read).await?;
    // Free the pool slot — nothing below touches MSSQL again.
    drop(conn);

    let mut legacy: BTreeMap<String, RegistryFolioProjection> = BTreeMap::new();
    let mut skipped = 0i32;
    let mut out_of_era = 0i32;
    for row in &rows {
        // A NULL/empty `Cin_no` is an orphan companion row the CT mapper
        // skips with a warning — there is no folio to attach it to.
        let Some(cin_no) = row.get::<&str, _>("Cin_no").filter(|s| !s.is_empty()) else {
            skipped += 1;
            continue;
        };
        if !in_era_keys.contains(cin_no) {
            out_of_era += 1;
            continue;
        }
        push_legacy_companion(
            legacy
                .entry(cin_no.to_string())
                .or_insert_with(|| RegistryFolioProjection::empty(cin_no)),
            row,
        );
    }

    let acked = load_guest_registry_ack_cache(pg_pool).await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    // Union of both key sets — see the doc comment on why a legacy-only
    // walk would be blind to a canonical-only folio. `BTreeSet` keys are
    // sorted, so the merged iteration order is deterministic.
    let keys: std::collections::BTreeSet<String> =
        legacy.keys().chain(canonical.keys()).cloned().collect();

    // Materialise the union on BOTH sides: a key present on one side only
    // gets an explicit EMPTY folio on the other. Empty is a real hashable
    // state, not an absent row (that is what lets a companion set deleted
    // everywhere converge), so this changes no hash — it just means the
    // comparison below never has to synthesise a temporary.
    for cin_no in &keys {
        legacy
            .entry(cin_no.clone())
            .or_insert_with(|| RegistryFolioProjection::empty(cin_no.as_str()));
        canonical
            .entry(cin_no.clone())
            .or_insert_with(|| RegistryFolioProjection::empty(cin_no.as_str()));
    }

    // PASS 1 — pure comparison, ZERO writes, so the tick's whole output is
    // known before any of it is committed. See `divergence_cap_exceeded`.
    let mut comparisons: Vec<(&str, String, String)> = Vec::with_capacity(keys.len());
    let mut divergent = 0usize;
    for cin_no in &keys {
        let mssql_hash = guest_registry_canonical_hash(&legacy[cin_no.as_str()]);
        let pg_hash = guest_registry_canonical_hash(&canonical[cin_no.as_str()]);
        if pg_hash != mssql_hash {
            divergent += 1;
        }
        comparisons.push((cin_no.as_str(), pg_hash, mssql_hash));
    }

    // The circuit breaker. A tick this loud is a SCOPE bug (floor dragged
    // backwards) or a dead mirror, never a backlog worth writing down —
    // abort before the first INSERT and page instead.
    let cap = guest_registry_divergence_cap(site_id);
    if divergence_cap_exceeded(divergent, cap) {
        let detail = format!(
            "guest-registry reconcile aborted: {divergent} diverging folios of \
             {in_scope} in scope exceeds the per-tick cap of {cap} (coverage floor \
             {era_floor}); nothing written",
            in_scope = comparisons.len(),
        );
        tracing::error!(
            site = %site_id,
            divergent,
            in_scope = comparisons.len(),
            cap,
            era_floor = %era_floor,
            "[Sync] {}",
            detail,
        );
        alert_guest_registry_divergence_cap(
            pg_pool,
            slack,
            site_id,
            divergent,
            comparisons.len(),
            cap,
            era_floor,
        )
        .await;
        record_error(pg_pool, "guest_registry", &detail).await;
        return Ok(());
    }

    // PASS 2 — the writes, now known to be bounded.
    for (cin_no, pg_hash, mssql_hash) in &comparisons {
        let cin_no = *cin_no;
        let legacy_folio = &legacy[cin_no];
        let canonical_folio = &canonical[cin_no];

        if guest_registry_ack_needs_write(acked.get(cin_no), mssql_hash) {
            ack_guest_registry_mirror(pg_pool, cin_no, mssql_hash).await;
        }

        if pg_hash == mssql_hash {
            unchanged += 1;
            continue;
        }

        // The FOLIO is the row: it exists on both sides by construction
        // (the key set is canonical check-ins, and an absent companion set
        // is the empty folio, not a missing row), so the counts are 1/1 and
        // [`classify_divergence`] yields `Value`. The per-side companion
        // counts an operator actually needs are in the JSON payloads.
        // `Cardinality` / `MissingPg` are unreachable here by design —
        // `missing_pg` would be a never-silenced, never-closing row for the
        // ordinary "iHOTEL added a companion we haven't ingested yet" case.
        let kind = classify_divergence(Some(&pg_hash), Some(&mssql_hash), 1, 1);
        record_divergence(
            pg_pool,
            "guest_registry",
            cin_no,
            Some(&pg_hash),
            Some(&mssql_hash),
            registry_folio_json(legacy_folio),
            Some(registry_folio_json(canonical_folio)),
            kind,
            1,
            1,
        )
        .await;

        if canonical_folio.is_empty() {
            // Legacy has companions, canonical has none: the TM.30
            // under-count shape Track E1 exists to prevent.
            added += 1;
        } else {
            updated += 1;
        }
    }

    if skipped > 0 {
        tracing::warn!(
            skipped,
            "[Sync] sync_guest_registry: {} companion rows have a NULL/empty \
             Cin_no and belong to no folio",
            skipped,
        );
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        era_floor = %era_floor,
        scanned = rows.len(),
        out_of_era,
        "[Sync] Guest registry: {} folios missing every canonical companion, \
         {} drifted, {} unchanged in {}ms (in-era folios from {})",
        added,
        updated,
        unchanged,
        duration_ms,
        era_floor,
    );
    record_success(
        pg_pool,
        "guest_registry",
        added,
        updated,
        unchanged,
        duration_ms,
    )
    .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    //! Pure unit tests for the Phase 5.5 mode-parsing logic. The
    //! integration tests that exercise the full reconcile + diff-log
    //! path live in `tests/test_scheduler_sync_diff_only.rs`.
    use super::*;

    /// Env-var manipulation across parallel cargo tests would race; we
    /// serialise these tests behind a Mutex and restore the prior value.
    /// The lock is process-wide because `set_var` is too.
    fn with_mode_env<F: FnOnce() -> ReconcileMode>(value: Option<&str>, f: F) -> ReconcileMode {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let prior = env::var("LEGACY_SYNC_RECONCILE_MODE").ok();
        match value {
            Some(v) => env::set_var("LEGACY_SYNC_RECONCILE_MODE", v),
            None => env::remove_var("LEGACY_SYNC_RECONCILE_MODE"),
        }
        let out = f();
        match prior {
            Some(v) => env::set_var("LEGACY_SYNC_RECONCILE_MODE", v),
            None => env::remove_var("LEGACY_SYNC_RECONCILE_MODE"),
        }
        out
    }

    #[test]
    fn from_env_defaults_to_diff_only_when_unset() {
        let mode = with_mode_env(None, ReconcileMode::from_env);
        assert_eq!(mode, ReconcileMode::DiffOnly);
    }

    #[test]
    fn from_env_recognises_diff_only_literal() {
        let mode = with_mode_env(Some("diff_only"), ReconcileMode::from_env);
        assert_eq!(mode, ReconcileMode::DiffOnly);
    }

    #[test]
    fn from_env_recognises_upsert_literal() {
        let mode = with_mode_env(Some("upsert"), ReconcileMode::from_env);
        assert_eq!(mode, ReconcileMode::Upsert);
    }

    #[test]
    fn from_env_falls_back_to_diff_only_on_unknown_value() {
        let mode = with_mode_env(Some("garbage"), ReconcileMode::from_env);
        assert_eq!(
            mode,
            ReconcileMode::DiffOnly,
            "unknown values must default to the safe (non-mutating) mode"
        );
    }

    // -------------------------------------------------------------------
    // Phase 6 — drift-alert threshold tests
    // -------------------------------------------------------------------

    fn counts(rows: &[(&str, i64)]) -> Vec<(String, i64)> {
        rows.iter().map(|(t, n)| ((*t).to_string(), *n)).collect()
    }

    #[test]
    fn breach_filter_returns_empty_when_no_table_exceeds_threshold() {
        let input = counts(&[("customers", 10), ("rooms", 49), ("bookings", 0)]);
        assert!(tables_breaching_threshold(&input, 50).is_empty());
    }

    #[test]
    fn breach_filter_is_strict_greater_than_not_equal() {
        // Threshold 50, count 50 → must NOT alert. Spec wording: "above
        // 50 in an hour".
        let input = counts(&[("customers", 50)]);
        assert!(
            tables_breaching_threshold(&input, 50).is_empty(),
            "exactly-at-threshold must not alert"
        );
    }

    #[test]
    fn breach_filter_returns_only_breaching_tables() {
        let input = counts(&[
            ("customers", 51),
            ("rooms", 49),
            ("bookings", 5_000),
            ("checkins", 50),
        ]);
        let mut breaches = tables_breaching_threshold(&input, 50);
        breaches.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            breaches,
            vec![
                ("bookings".to_string(), 5_000),
                ("customers".to_string(), 51)
            ]
        );
    }

    #[test]
    fn breach_filter_handles_empty_input() {
        let input: Vec<(String, i64)> = Vec::new();
        assert!(tables_breaching_threshold(&input, 50).is_empty());
    }

    #[test]
    fn breach_filter_respects_custom_threshold() {
        let input = counts(&[("customers", 5), ("rooms", 11)]);
        // Threshold 10 → only `rooms` (11 > 10).
        let breaches = tables_breaching_threshold(&input, 10);
        assert_eq!(breaches, vec![("rooms".to_string(), 11)]);
    }

    /// Env-isolation helper for the threshold-parsing tests. Same shape
    /// as `with_mode_env` above. Tracks BOTH the global var and the
    /// per-site override (task #69) so the per-site fallback chain can
    /// be exercised without leaking state across tests.
    fn with_threshold_envs<F: FnOnce() -> i64>(
        global: Option<&str>,
        per_site_var: Option<(&str, &str)>,
        f: F,
    ) -> i64 {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let prior_global = env::var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD").ok();
        let prior_per_site = per_site_var.map(|(name, _)| (name.to_string(), env::var(name).ok()));
        match global {
            Some(v) => env::set_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD", v),
            None => env::remove_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"),
        }
        if let Some((name, value)) = per_site_var {
            env::set_var(name, value);
        }
        let out = f();
        match prior_global {
            Some(v) => env::set_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD", v),
            None => env::remove_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"),
        }
        if let Some((name, prior)) = prior_per_site {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        out
    }

    /// Back-compat shim for tests that only care about the global var.
    fn with_threshold_env<F: FnOnce() -> i64>(value: Option<&str>, f: F) -> i64 {
        with_threshold_envs(value, None, f)
    }

    #[test]
    fn threshold_defaults_when_env_unset() {
        let v = with_threshold_env(None, || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    #[test]
    fn threshold_parses_custom_positive_value() {
        let v = with_threshold_env(Some("125"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, 125);
    }

    #[test]
    fn threshold_falls_back_on_zero_or_negative() {
        let v = with_threshold_env(Some("0"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
        let v = with_threshold_env(Some("-5"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    #[test]
    fn threshold_falls_back_on_garbage() {
        let v = with_threshold_env(Some("not-a-number"), || {
            drift_alert_threshold_from_env("hfhotel")
        });
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    // -------------------------------------------------------------------
    // Task #69 — per-site drift threshold overrides
    // -------------------------------------------------------------------

    /// Global env honored when the per-site override is unset (the
    /// HF Hotel back-compat path: existing
    /// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD=80` keeps working
    /// unchanged after task #69 lands).
    #[test]
    fn threshold_global_used_when_per_site_unset() {
        let v = with_threshold_envs(Some("80"), None, || {
            drift_alert_threshold_from_env("hfhotel")
        });
        assert_eq!(v, 80);
    }

    /// Per-site override wins when both are set (the HF Ville path:
    /// operator wants a tighter alert threshold for the smaller
    /// property without disturbing HF Hotel's tuning).
    #[test]
    fn threshold_per_site_overrides_global() {
        let v = with_threshold_envs(
            Some("80"),
            Some(("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE", "20")),
            || drift_alert_threshold_from_env("hfville"),
        );
        assert_eq!(v, 20, "per-site override must take precedence over global");
    }

    /// Per-site override is namespaced by the site id — an HF Hotel
    /// tick must NOT pick up `..._HFVILLE` and vice versa.
    #[test]
    fn threshold_per_site_does_not_leak_across_sites() {
        let v = with_threshold_envs(
            Some("80"),
            Some(("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE", "20")),
            || drift_alert_threshold_from_env("hfhotel"),
        );
        assert_eq!(
            v, 80,
            "HF Hotel tick must use the global threshold, not HF Ville's"
        );
    }

    /// Garbage in the per-site override falls through to the global
    /// (instead of crashing) — operator typo doesn't take down alerts.
    #[test]
    fn threshold_per_site_garbage_falls_through_to_global() {
        let v = with_threshold_envs(
            Some("80"),
            Some(("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE", "abc")),
            || drift_alert_threshold_from_env("hfville"),
        );
        assert_eq!(
            v, 80,
            "invalid per-site value must fall through to the global"
        );
    }

    // -------------------------------------------------------------------
    // Phase 6 hotfix (2026-04-29) — multi-row PK aggregation determinism
    // -------------------------------------------------------------------
    //
    // `View_CheckIn_Ds` returns 41-45 rows per `Cin_no`; `View_Booking_Ds`
    // returns up to 3 rows per `(Book_No, Book_Room_Type)`. Hashing each
    // row independently against a single-row-per-PK cache caused a
    // ~22-24k/hour drift-alert spam loop. These tests pin the
    // determinism + sensitivity contract of the aggregation helpers.

    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32) -> Option<NaiveDateTime> {
        Some(
            NaiveDate::from_ymd_opt(y, m, d)
                .unwrap()
                .and_hms_opt(h, min, 0)
                .unwrap(),
        )
    }

    // -------------------------------------------------------------------
    // sync_checkins mapper-unification (2026-05-18)
    // -------------------------------------------------------------------
    //
    // The pre-2026-05-18 bulk sweep grouped `HT_CheckIn_H ⋈ HT_CheckIn_Ds`
    // rows in Rust and picked the alphabetical-first `Cin_Room_No` as
    // the representative. The CT mapper picks the lowest-`Cin_Ds.id`
    // row (via `derive_room_state`). On multi-room folios where those
    // pick different rooms, every reconcile tick inserted a spurious
    // `value` drift. The pinned tests below cover the unified shape:
    // `classify_checkin_drift` returns `None` when the mapper-projected
    // legacy hash matches canonical, regardless of room ordering.

    /// Build the hash inputs a single-folio check-in produces when its
    /// canonical PG projection picks `legacy_room_no` as the
    /// representative room. Mirrors the byte-shape of
    /// [`checkin_hash_from_canonical`] for the given inputs without
    /// requiring a full `CanonicalCheckIn` construction (which has
    /// private fields the mapper guards). Hash inputs are stable
    /// across both the bulk sweep and the auto-resolve sweep — what
    /// this test fixture pins is the cross-pipeline equality, not a
    /// new format.
    fn projected_hash(cin_no: &str, legacy_room_no: &str, cust_no: &str) -> String {
        let checkin_dt = NaiveDate::from_ymd_opt(2026, 4, 1)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();
        let expected_out = NaiveDate::from_ymd_opt(2026, 4, 3).unwrap();
        checkin_canonical_hash(
            cin_no,
            Some(legacy_room_no),
            Some(checkin_dt.to_string()).as_deref(),
            Some(expected_out.to_string()).as_deref(),
            Some(cust_no),
            false,
            false,
        )
    }

    /// Regression guard for the headline bug class
    /// (`docs/coexistence/data-hygiene-ch25-002081.md`): a multi-room
    /// folio whose canonical `legacy_room_no` matches the mapper's
    /// `derive_room_state` first-room pick MUST classify as "no drift".
    /// Pre-fix the bulk sweep re-implemented the projection in Rust
    /// and picked the alphabetical-first `Cin_Room_No` instead of the
    /// lowest-`Cin_Ds.id` row the CT mapper picks, disagreeing with
    /// canonical on every multi-room folio whose lowest-id row wasn't
    /// also alphabetically first — ~545 spurious `value` drift rows
    /// per tick on HF Hotel (2026-05-18 audit).
    ///
    /// Post-fix: both the bulk sweep and canonical resolve via the
    /// same `project_checkin_aggregate`, so the representative pick
    /// agrees by construction and no drift fires.
    #[test]
    fn multi_room_pk_with_matching_first_room_reports_no_drift() {
        // Both sides project the same first room (the mapper's pick).
        let mapper_hash = projected_hash("CIN-1", "203", "C001");
        let canonical_hash = projected_hash("CIN-1", "203", "C001");

        let drift = classify_checkin_drift(
            Some(&canonical_hash),
            &mapper_hash,
            3, // legacy rooms
            3, // pg junction rooms
        );

        assert!(
            drift.is_none(),
            "mapper-projected hash for the first room MUST equal canonical's \
             stored first room (both are the lowest-Cin_Ds.id row); the \
             bulk sweep no longer disagrees on which room is representative — \
             pre-fix this returned DivergenceKind::Value on every multi-room \
             folio where alphabetical-first ≠ lowest-id"
        );
    }

    /// The exact bug class we eliminated: pre-fix the bulk sweep
    /// picked the alphabetical-first room ("101") while canonical PG
    /// stored "203" (the mapper's lowest-id pick). The hash mismatch
    /// re-fired `DivergenceKind::Value` on every tick. This test
    /// pins the pre-fix behaviour AS A REGRESSION SHAPE and asserts
    /// the post-fix shape (both sides project via the mapper)
    /// converges.
    #[test]
    fn pre_fix_alphabetical_vs_lowest_id_pick_diverges_but_unified_converges() {
        let alpha_first_hash = projected_hash("CIN-1", "101", "C001");
        let mapper_pick_hash = projected_hash("CIN-1", "203", "C001");

        // Pre-fix shape: sweep hashed "101", canonical stored "203" →
        // spurious value drift every tick.
        let pre_fix_drift =
            classify_checkin_drift(Some(&mapper_pick_hash), &alpha_first_hash, 3, 3);
        assert_eq!(
            pre_fix_drift,
            Some(DivergenceKind::Value),
            "pre-fix sweep picked '101' alphabetically, canonical PG stored \
             '203' from the mapper — this hash mismatch was the spurious \
             drift signal the unification eliminates"
        );

        // Post-fix shape: both sides project via the mapper → same
        // first room → no drift.
        let post_fix_drift =
            classify_checkin_drift(Some(&mapper_pick_hash), &mapper_pick_hash, 3, 3);
        assert!(
            post_fix_drift.is_none(),
            "unifying the sweep on the CT mapper eliminates the parallel- \
             projection mismatch by construction"
        );
    }

    /// Negative guard: real value drift (e.g. CT-watcher missed the
    /// update, canonical stuck on an old cust_no) must still classify
    /// as `Value`. The fix narrows false positives, not signal.
    #[test]
    fn genuine_value_drift_still_classifies_as_value() {
        // Mapper sees new cust_no in legacy; canonical PG is stale.
        let mapper_hash = projected_hash("CIN-1", "203", "C001-NEW");
        let canonical_stale_hash = projected_hash("CIN-1", "203", "C001-OLD");

        let drift = classify_checkin_drift(Some(&canonical_stale_hash), &mapper_hash, 3, 3);

        assert_eq!(
            drift,
            Some(DivergenceKind::Value),
            "hash mismatch with matching cardinality must remain Value drift"
        );
    }

    /// Cardinality drift (junction row count short of legacy) MUST
    /// keep classifying as `Cardinality` regardless of hash match.
    #[test]
    fn cardinality_drift_classifies_even_when_hashes_match() {
        let hash = projected_hash("CIN-1", "203", "C001");

        let drift = classify_checkin_drift(
            Some(&hash),
            &hash,
            3, // legacy rooms
            2, // pg junction rooms short
        );

        assert_eq!(
            drift,
            Some(DivergenceKind::Cardinality),
            "row-count asymmetry must surface as Cardinality drift, \
             even when first-room hashes converge"
        );
    }

    /// `missing_pg` drift (canonical has nothing for this PK) must
    /// surface even when the mapper produces a valid hash.
    #[test]
    fn missing_canonical_classifies_as_missing_pg() {
        let mapper_hash = projected_hash("CIN-1", "203", "C001");

        let drift = classify_checkin_drift(
            None, // canonical absent
            &mapper_hash,
            3, // legacy rooms
            0, // no canonical rooms
        );

        assert_eq!(
            drift,
            Some(DivergenceKind::MissingPg),
            "canonical-absent must remain a MissingPg signal — the highest- \
             priority drift kind operators page on"
        );
    }

    // -------------------------------------------------------------------
    // Track B last-mile (2026-05-18) — junction-aware pg_row_count
    // -------------------------------------------------------------------
    //
    // These exercise the `classify_divergence` decision the reconcile
    // loop now feeds with the actual `ht_checkin_rooms` row count
    // (instead of the pre-Track-B hardcoded 1). The wire-up itself is
    // an `await` in the loop body — the testable surface is the pure
    // classifier and its truth-table response to the new inputs.

    /// Regression guard: post-Track-B, a multi-room folio whose
    /// junction-room count matches legacy `View_CheckIn_Ds.details.len()`
    /// must classify as `Value` (and become silenceable via
    /// `is_silenceable()` once hashes align too). Pre-fix this case
    /// returned `Cardinality` for every multi-room folio because
    /// `pg_row_count` was hardcoded to 1.
    #[test]
    fn cardinality_kind_when_junction_count_matches_legacy_returns_value_not_cardinality() {
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 3, 3);
        assert_eq!(
            kind,
            DivergenceKind::Value,
            "matching junction count must drop the kind to Value so the \
             ack-cache can silence it once the first-room hashes align"
        );
    }

    /// Regression guard against an over-eager fix: if the junction
    /// has FEWER rows than legacy (e.g. backfill skipped an inactive
    /// folio), we must still classify as `Cardinality` so the
    /// operator-visible signal is preserved.
    #[test]
    fn cardinality_kind_when_junction_count_differs_returns_cardinality() {
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 3, 2);
        assert_eq!(
            kind,
            DivergenceKind::Cardinality,
            "junction count short of legacy must remain Cardinality so \
             the gap surfaces in ht_reconcile_log instead of being silenced"
        );
    }

    fn booking_detail(room_type: &str, status: i32) -> BookingDetail {
        BookingDetail {
            book_date: dt(2026, 4, 1, 10, 0),
            book_date_in: dt(2026, 4, 5, 14, 0),
            book_date_out: dt(2026, 4, 7, 12, 0),
            book_cust_name: Some("Somchai".to_string()),
            book_cust_id: Some("ID-001".to_string()),
            book_status: Some(status),
            book_room_type: Some(room_type.to_string()),
        }
    }

    #[test]
    fn aggregate_booking_hash_is_order_independent() {
        let a = booking_detail("DELUXE", 1);
        let b = BookingDetail {
            book_cust_name: Some("Anan".to_string()),
            ..booking_detail("DELUXE", 2)
        };
        let c = BookingDetail {
            book_cust_id: Some("ID-002".to_string()),
            ..booking_detail("DELUXE", 1)
        };

        let mut forward = vec![a.clone(), b.clone(), c.clone()];
        let mut reversed = vec![c, b, a];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut forward);
        let h2 = aggregate_booking_hash("BK-1", "DELUXE", &mut reversed);
        assert_eq!(h1, h2, "row order must not affect the aggregate hash");
    }

    #[test]
    fn aggregate_booking_hash_changes_when_a_field_changes() {
        let a = booking_detail("DELUXE", 1);
        let mut original = vec![a.clone()];
        let mut mutated = vec![BookingDetail {
            book_status: Some(2),
            ..a
        }];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut original);
        let h2 = aggregate_booking_hash("BK-1", "DELUXE", &mut mutated);
        assert_ne!(h1, h2, "field change must produce a different hash");
    }

    #[test]
    fn aggregate_booking_hash_handles_single_row_pk() {
        let a = booking_detail("DELUXE", 1);
        let mut once = vec![a.clone()];
        let mut twice = vec![a];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut once);
        let h2 = aggregate_booking_hash("BK-1", "DELUXE", &mut twice);
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }

    #[test]
    fn aggregate_booking_hash_distinguishes_composite_pk() {
        // (BK-1, DELUXE) vs (BK-1, STANDARD) — the room_type half of
        // the composite PK must be part of the hash so two PKs
        // sharing a Book_No don't collide.
        let a = booking_detail("DELUXE", 1);
        let b = booking_detail("STANDARD", 1);
        let mut g1 = vec![a];
        let mut g2 = vec![b];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut g1);
        let h2 = aggregate_booking_hash("BK-1", "STANDARD", &mut g2);
        assert_ne!(h1, h2, "composite PK key half must be part of the hash");
    }

    // -------------------------------------------------------------------
    // v2.63.0 — canonical-shape hash helpers (PG vs MSSQL alignment)
    // -------------------------------------------------------------------
    //
    // These tests pin the contract that an MSSQL row + a faithful
    // canonical projection of that row hash to the SAME value. If a CT
    // mapper regression breaks the projection, the canonical hash
    // diverges and the reconciler logs drift — exactly the actionable
    // signal the v2.63.0 migration is designed to surface.

    #[test]
    fn customer_canonical_hash_matches_when_canonical_mirrors_legacy() {
        let mssql = customer_canonical_hash(
            "C001",
            "Somchai",
            Some("walk-in"),
            Some("0812345678"),
            Some("1234567890123"),
            Some("123 Sukhumvit"),
        );
        // Canonical row that the CT mapper would have produced.
        let canonical = customer_canonical_hash(
            "C001",
            "Somchai",
            Some("walk-in"),
            Some("0812345678"),
            Some("1234567890123"),
            Some("123 Sukhumvit"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn customer_canonical_hash_diverges_on_phone_drift() {
        let mssql =
            customer_canonical_hash("C001", "Somchai", None, Some("0812345678"), None, None);
        // CT mapper stored an older phone (drift the operator should fix).
        let canonical =
            customer_canonical_hash("C001", "Somchai", None, Some("0899999999"), None, None);
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn customer_canonical_hash_treats_none_as_empty() {
        // `Cust_Type = NULL` on the MSSQL side hashes identically to
        // `cust_type = NULL` on the canonical side — both project to
        // empty string before hashing.
        let h1 = customer_canonical_hash("C001", "Anan", None, None, None, None);
        let h2 = customer_canonical_hash("C001", "Anan", Some(""), Some(""), Some(""), Some(""));
        assert_eq!(
            h1, h2,
            "None and empty-string must canonicalise the same way"
        );
    }

    /// Locks the legacy `HT_Customers` projection used by the reconcile
    /// hash against the CT mapper's column semantics. The six columns
    /// here must mirror `sync::mappers::customer::CustomerMapper`
    /// faithfully — otherwise every customer with a prefix, secondary
    /// name, multi-line address, or distinct rate-tier-vs-category
    /// surfaces as systematic `value` drift that the auto-resolve
    /// sweep can never clear.
    ///
    /// **Forbidden columns (regression tripwires):**
    /// - `View_Customers` concatenates `Cust_perfix + Cust_name + ' ' +
    ///   Cust_name2`, hiding the actual `Cust_name` the CT mapper
    ///   writes. Hash against `HT_Customers.Cust_name` only.
    /// - `View_Customers.C_Address` is a multi-line concatenation; PG
    ///   `cust_address` mirrors only the door number from
    ///   `Cust_Add_no`. Hash against `Cust_Add_no` only.
    /// - `View_Customers.Cust_Type` is the rate-tier label (mapped to
    ///   `cust_price_tier` in PG); the CT mapper writes
    ///   `Cust_Type_Main` into PG `cust_type`. Hash against
    ///   `Cust_Type_Main` only.
    #[test]
    fn customers_reconcile_projection_locks_mapper_compatible_columns() {
        for col in [
            "Cust_no",
            "Cust_name",
            "Cust_Type_Main",
            "Cust_Add_tel",
            "Cust_IDcard",
            "Cust_Add_no",
        ] {
            assert!(
                CUSTOMERS_RECONCILE_PROJECTION.contains(col),
                "CUSTOMERS_RECONCILE_PROJECTION missing required column '{col}' — \
                 reconcile hash will diverge from CT mapper's canonical writes"
            );
        }
        // Forbidden columns — these are the View_Customers collapsed
        // surfaces that produce systematic false-positive drift.
        for forbidden in ["C_Address", "Cust_Type "] {
            assert!(
                !CUSTOMERS_RECONCILE_PROJECTION.contains(forbidden),
                "CUSTOMERS_RECONCILE_PROJECTION must not project '{forbidden}' — \
                 hashes will not align with the CT mapper's writes"
            );
        }
        // `Cust_Type` is a substring of `Cust_Type_Main`, so the
        // contains-check above uses a trailing space. Pin the exact
        // bare-column absence via a token split.
        let has_bare_cust_type = CUSTOMERS_RECONCILE_PROJECTION
            .split(',')
            .map(str::trim)
            .any(|tok| tok == "Cust_Type");
        assert!(
            !has_bare_cust_type,
            "CUSTOMERS_RECONCILE_PROJECTION must not project bare 'Cust_Type' \
             (rate-tier, mapped to cust_price_tier) — use Cust_Type_Main instead"
        );
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards for the remaining reconcile
    // projections. `CUSTOMERS_RECONCILE_PROJECTION` keeps its hand-rolled
    // lock test above because that test additionally pins forbidden-
    // column tripwires (View_Customers's `Cust_Type` rate-tier label).
    // The three projections below have no such forbidden columns —
    // a baseline-subset check is the right granularity.
    //
    // PR #90 (`HT_CheckIn_Ds` 9h drop) and PR #101 (`Cust_Type` vs
    // `Cust_Type_Main` reconcile drift) were both single typos that
    // shipped because tiberius validates column names only at runtime.
    // A baseline-subset lock test catches that class at CI time before
    // it can ever reach the watcher / reconcile loop.
    // -------------------------------------------------------------------

    #[test]
    fn rooms_reconcile_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(ROOMS_RECONCILE_PROJECTION, "HT_Rooms");
    }

    #[test]
    fn bookings_reconcile_projection_is_subset_of_legacy_schema() {
        // `View_Booking_Ds` joins HT_Book_H (header) with HT_Book_Ds
        // (per-room detail). Every projected column must appear on
        // one of the two underlying base tables.
        crate::assert_projection_slice_subset_of_two_tables!(
            BOOKINGS_RECONCILE_PROJECTION,
            "HT_Book_H",
            "HT_Book_Ds",
            "View_Booking_Ds"
        );
    }

    // sync_checkins no longer maintains its own MSSQL projection — it
    // delegates per-PK to `parent_loader::load_checkin_aggregate` +
    // `mappers::project_checkin_aggregate` (the CT mapper's own
    // pipeline). The Track J1 projection-lock test for those columns
    // lives at `sync::parent_loader::tests::checkin_h_projection_*`
    // and `sync::mappers::checkin::tests::checkin_ds_projection_*`
    // respectively. Nothing in this module owns a parallel projection
    // any more, so the local lock test is retired.

    #[test]
    fn bool_to_yesno_round_trip_via_legacy_yesno_canonical() {
        // The two halves of the room-status translation must be each
        // other's inverse for the canonical-hash to align with the
        // MSSQL projection.
        assert_eq!(
            legacy_yesno_canonical(Some("yes")),
            bool_to_yesno(Some(true))
        );
        assert_eq!(
            legacy_yesno_canonical(Some("no")),
            bool_to_yesno(Some(false))
        );
        // NULL → "" on both sides, matching how nullable BOOLEAN
        // columns canonicalise.
        assert_eq!(legacy_yesno_canonical(None), bool_to_yesno(None));
        // Unknown legacy literals fall back to "" — matches the CT
        // mapper's behaviour (`legacy_yesno_to_bool` returns None for
        // anything other than yes/no).
        assert_eq!(legacy_yesno_canonical(Some("maybe")), "");
    }

    /// Regression: the DETECTION projection (`sync_rooms`) and the
    /// AUTO-RESOLVE projection (`compute_current_pg_hash`) must agree on
    /// `room_clean`, which is INVERTED between the two sides — legacy 'yes'
    /// means NEEDS cleaning, canonical `true` means IS clean.
    ///
    /// Commit 0303c98 introduced `clean_bool_to_legacy_yesno` for this but
    /// applied it to only one of the two call sites. The result was a false
    /// `value` divergence recorded for every room with a non-NULL
    /// `Room_Clean`, immediately closed again by the sweep — churn that can
    /// age past the 4h level alert if a row misses the sweep's LIMIT 500.
    ///
    /// The `bool_to_yesno` assertion at the bottom is the important half: it
    /// fails if anyone "simplifies" the inverted helper back to the plain one.
    #[test]
    fn room_clean_projections_agree_across_detection_and_auto_resolve() {
        // A dirty room: legacy says "needs cleaning", canonical says not clean.
        let legacy_dirty = room_canonical_hash("512", legacy_yesno_canonical(Some("yes")), "no", None);
        let canonical_dirty =
            room_canonical_hash("512", clean_bool_to_legacy_yesno(Some(false)), "no", None);
        assert_eq!(
            legacy_dirty, canonical_dirty,
            "canonical room_clean=false MUST hash like legacy Room_Clean='yes' (needs cleaning)"
        );

        // A clean room: legacy "no cleaning needed", canonical clean.
        let legacy_clean = room_canonical_hash("512", legacy_yesno_canonical(Some("no")), "no", None);
        let canonical_clean =
            room_canonical_hash("512", clean_bool_to_legacy_yesno(Some(true)), "no", None);
        assert_eq!(
            legacy_clean, canonical_clean,
            "canonical room_clean=true MUST hash like legacy Room_Clean='no'"
        );

        // And the uninverted helper must NOT agree — this is what the bug was.
        let canonical_dirty_uninverted =
            room_canonical_hash("512", bool_to_yesno(Some(false)), "no", None);
        assert_ne!(
            legacy_dirty, canonical_dirty_uninverted,
            "bool_to_yesno on room_clean reintroduces the 0303c98 half-fix"
        );
    }

    #[test]
    fn room_canonical_hash_matches_when_canonical_mirrors_legacy() {
        // room_clean is INVERTED: legacy "no" = no cleaning needed = canonical
        // true (IS clean). room_maintenance is NOT inverted. Pairing canonical
        // `true` with legacy "yes" — as this test did before 20edf18 — encodes
        // the very polarity bug that fix removed.
        let mssql = room_canonical_hash("101", "no", "no", Some("ocean view"));
        let canonical = room_canonical_hash(
            "101",
            clean_bool_to_legacy_yesno(Some(true)),
            bool_to_yesno(Some(false)),
            Some("ocean view"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn room_canonical_hash_diverges_when_canonical_clean_lags_behind() {
        // Legacy says the room NEEDS cleaning ("yes" = dirty) but the CT
        // mapper hasn't yet flipped canonical.room_clean to false, so
        // canonical still claims clean → divergence fires.
        let mssql = room_canonical_hash("101", "yes", "no", None);
        let canonical = room_canonical_hash(
            "101",
            clean_bool_to_legacy_yesno(Some(true)),
            bool_to_yesno(Some(false)),
            None,
        );
        assert_ne!(mssql, canonical);
    }

    // -------------------------------------------------------------------
    // PR C (2026-05-19) — auto-resolve sweep wires rooms into both
    // dispatch tables (`compute_current_pg_hash` and
    // `compute_current_legacy_hash`). The dispatch arms are 1-liners
    // that delegate to `fetch_canonical_room → room_canonical_hash`
    // and `fetch_legacy_room_hash` respectively; the substantive
    // behaviour lives in those helpers. The composition tests below
    // pin the hash-input contract that both arms construct, mirroring
    // the customer/booking pattern (the live A2-1 evidence of
    // 2026-05-18 was `current_legacy_hash=None current_pg_hash=None`
    // — the `_ => Ok(None)` arm firing on both sides).
    // -------------------------------------------------------------------

    /// Pinpoints the canonical-side arm's composition: a `CanonicalRoomRow`
    /// projects through `bool_to_yesno + room_canonical_hash` into
    /// exactly the same byte string the legacy-side projection emits
    /// for the converged equivalent. When the dispatch arm runs against
    /// a real PG row, this is the hash the sweep compares against
    /// `fetch_legacy_room_hash`'s output.
    #[test]
    fn rooms_pg_dispatch_composition_converges_with_legacy_projection() {
        // A canonical row the CT mapper would have written after the
        // legacy state converged: legacy Room_Clean="no" (no cleaning
        // needed) → canonical room_clean=true; maintenance "no" → false.
        let canonical_row = CanonicalRoomRow {
            room_clean: Some(true),
            room_maintenance: Some(false),
            room_notes: None,
        };
        let pg_hash = room_canonical_hash(
            "A2-1",
            clean_bool_to_legacy_yesno(canonical_row.room_clean),
            bool_to_yesno(canonical_row.room_maintenance),
            canonical_row.room_notes.as_deref(),
        );

        // The legacy side runs `legacy_yesno_canonical` over the raw
        // MSSQL literal ("yes"/"no"/NULL). After convergence those
        // collapse to the same `'yes' | 'no' | ""` tokens used above.
        let legacy_hash = room_canonical_hash(
            "A2-1",
            legacy_yesno_canonical(Some("no")),
            legacy_yesno_canonical(Some("no")),
            None,
        );

        assert_eq!(
            pg_hash, legacy_hash,
            "auto-resolve sweep relies on both arms producing byte-identical \
             hashes when the underlying state has converged"
        );
    }

    /// Round-trips every legal `(canonical bool, legacy literal)` pair for
    /// the NON-inverted fields. Lock test for `bool_to_yesno` ↔
    /// `legacy_yesno_canonical`.
    ///
    /// This pairing is correct for `room_maintenance` and WRONG for
    /// `room_clean`, which is inverted (legacy "yes" = needs cleaning =
    /// canonical false) and uses `clean_bool_to_legacy_yesno` — see
    /// `room_clean_projections_agree_across_detection_and_auto_resolve`.
    /// Do not cite this test as licence to use `bool_to_yesno` on clean.
    #[test]
    fn rooms_dispatch_yesno_round_trip_is_total() {
        for (canonical, legacy_literal) in [
            (Some(true), Some("yes")),
            (Some(false), Some("no")),
            (None, None),
        ] {
            assert_eq!(
                bool_to_yesno(canonical),
                legacy_yesno_canonical(legacy_literal),
                "rooms dispatch loses parity when canonical {:?} ↔ legacy {:?}",
                canonical,
                legacy_literal,
            );
        }
    }

    /// Operator-edited the notes in legacy but the CT mapper hasn't
    /// caught up → both dispatch arms produce different hashes and the
    /// reconcile_log row stays open. Mirrors the
    /// `room_canonical_hash_diverges_when_canonical_clean_lags_behind`
    /// pattern for the notes field, which is the third hash input.
    #[test]
    fn rooms_pg_dispatch_composition_diverges_when_canonical_notes_lag() {
        let canonical_row = CanonicalRoomRow {
            room_clean: Some(true),
            room_maintenance: Some(false),
            room_notes: Some("old note".to_string()),
        };
        let pg_hash = room_canonical_hash(
            "A2-1",
            clean_bool_to_legacy_yesno(canonical_row.room_clean),
            bool_to_yesno(canonical_row.room_maintenance),
            canonical_row.room_notes.as_deref(),
        );
        let legacy_hash = room_canonical_hash(
            "A2-1",
            legacy_yesno_canonical(Some("yes")),
            legacy_yesno_canonical(Some("no")),
            Some("new note"),
        );
        assert_ne!(
            pg_hash, legacy_hash,
            "post-detection sweep must NOT auto-resolve while canonical \
             notes still trail the legacy state"
        );
    }

    /// `fetch_legacy_room_hash` returns `Ok(None)` when the legacy
    /// row has been deleted since the drift was detected. This pure
    /// test pins the convention used by every `fetch_legacy_*_hash`
    /// helper: a missing row is "still drifted — leave for operator
    /// review" rather than "converged to absent". For non-booking
    /// entities the dispatch arm must NOT auto-resolve a vanished
    /// legacy key — even when canonical matches the recorded legacy
    /// state — because a deleted room/customer/checkin is a genuine
    /// anomaly. The stale-ghost arm is bookings-only.
    #[test]
    fn rooms_dispatch_missing_legacy_row_does_not_auto_resolve() {
        // Simulate `fetch_legacy_room_hash` returning Ok(None) for a
        // legacy-deleted row, alongside a still-present canonical hash
        // that EQUALS the recorded legacy hash. For bookings this would
        // trip the stale-ghost arm; for rooms it must stay open.
        let recorded = room_canonical_hash("A2-1", "yes", "no", None);
        let legacy_hash: Option<String> = None;
        let pg_hash: Option<String> = Some(recorded.clone());

        assert!(
            !should_auto_resolve(
                "rooms",
                legacy_hash.as_deref(),
                pg_hash.as_deref(),
                Some(recorded.as_str()),
            ),
            "a None legacy hash (room deleted) must keep the reconcile_log \
             row open for operator review, never auto-resolve — the \
             stale-ghost arm is restricted to bookings"
        );
    }

    #[test]
    fn booking_canonical_hash_aligns_legacy_datetime_and_canonical_date() {
        // Legacy stores `Book_Date_in` as DATETIME-at-midnight. After
        // `.date().to_string()` it serialises to "2026-04-01" — the
        // exact format `chrono::NaiveDate::to_string()` emits on the
        // canonical side.
        let mssql = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-03"),
            Some("C001"),
        );
        let canonical = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-03"),
            Some("C001"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn booking_canonical_hash_diverges_on_checkout_date_drift() {
        let mssql = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-03"),
            Some("C001"),
        );
        let canonical = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-05"), // canonical extended by two nights (drift)
            Some("C001"),
        );
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_aligns_legacy_first_room_and_canonical_legacy_room_no() {
        // CT checkin mapper denormalises the FIRST sorted room into
        // `ht_checkins.legacy_room_no`. The reconciler picks the same
        // first room from the sorted legacy multi-row group, so both
        // hashes use the identical room_no token.
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_diverges_on_room_drift() {
        let mssql = checkin_canonical_hash("CIN001", Some("101"), None, None, None, false, false);
        // CT mapper resolved the wrong room — drift the operator
        // should investigate via `ht_reconcile_log`.
        let canonical =
            checkin_canonical_hash("CIN001", Some("102"), None, None, None, false, false);
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_handles_open_checkin_with_no_checkout() {
        // Both-NULL is now a vacuous case (Bug B fix: canonical reads
        // `cin_expected_checkout` which is NOT NULL per derive_stay_range,
        // and legacy `Cin_Room_Out` is populated at check-in time). Kept
        // as a degenerate-input guard — the hash function itself still
        // matches when both sides pass None.
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        assert_eq!(mssql, canonical);
    }

    /// Bug B fix (2026-05-15): for an active stay, MSSQL `Cin_Room_Out` is the
    /// booked checkout *timestamp* (e.g. `2026-05-18 11:59:59`) while
    /// canonical `cin_expected_checkout` is a *date* (`2026-05-18`). The
    /// reconcile-side caller now drops the time component on the MSSQL side
    /// and reads `cin_expected_checkout` (not `cin_checkout_time`) on the
    /// canonical side, so both inputs land on the same `"YYYY-MM-DD"`
    /// string and the hashes converge. Without this alignment every active
    /// stay produced a systematic false-positive value-drift row.
    #[test]
    fn checkin_canonical_hash_aligns_booked_checkout_across_sides_after_bug_b_fix() {
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18"), // MSSQL caller now drops time via .date().to_string()
            Some("C001"),
            false,
            false,
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18"), // canonical cin_expected_checkout.to_string()
            Some("C001"),
            false,
            false,
        );
        assert_eq!(mssql, canonical);
    }

    /// Regression guard for Bug B. Pre-fix the MSSQL side hashed
    /// `Cin_Room_Out` as a full datetime while the canonical side hashed
    /// `cin_checkout_time` (NULL on active stays). Re-introducing either
    /// half of that mismatch must fail this test. The two inputs below
    /// model the pre-fix call shape; the new code never produces them.
    #[test]
    fn checkin_canonical_hash_pre_bug_b_inputs_must_diverge() {
        let pre_fix_mssql_datetime = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18 11:59:59"),
            Some("C001"),
            false,
            false,
        );
        let pre_fix_canonical_null_checkout = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        assert_ne!(
            pre_fix_mssql_datetime, pre_fix_canonical_null_checkout,
            "pre-Bug-B inputs MUST diverge — this is the bug the fix addresses"
        );
    }

    /// Task #68 regression guard for the 2026-06-28 room-114 / cin 19906
    /// dropped-checkout class. A guest who departs ON the expected date leaves
    /// `effective_checkout` IDENTICAL on both sides (actual == expected), so the
    /// only signal of the dropped checkout is the `checked_out` bit. Canonical
    /// still `active` (checked_out=false) vs legacy checked-out
    /// (checked_out=true) MUST diverge despite every OTHER field matching —
    /// otherwise the sweep is blind to this exact incident.
    #[test]
    fn checkin_canonical_hash_diverges_on_checked_out_flag_when_dates_coincide() {
        let canonical_active = checkin_canonical_hash(
            "CH26-001158",
            Some("114"),
            Some("2026-05-15 15:00:00"),
            Some("2026-05-16"), // effective = expected (active, no actual checkout)
            Some("C019906"),
            false, // checked_out: canonical still active (the dropped checkout)
            false,
        );
        let legacy_checked_out = checkin_canonical_hash(
            "CH26-001158",
            Some("114"),
            Some("2026-05-15 15:00:00"),
            Some("2026-05-16"), // effective = actual = SAME date as expected
            Some("C019906"),
            true, // checked_out: legacy departed on the expected date
            false,
        );
        assert_ne!(
            canonical_active, legacy_checked_out,
            "a dropped checkout (active vs checked-out) MUST diverge even when actual==expected"
        );
    }

    /// Bug D fix (2026-05-16): `effective_checkout_date()` prefers
    /// `cin_checkout_time::date()` over `cin_expected_checkout`. Pre-fix
    /// the hash always used `cin_expected_checkout`, which froze at the
    /// original booked date for completed-extended stays — 3,382
    /// false-positive value-drift rows on HF Hotel as of audit
    /// 2026-05-16 (e.g. CH26-005335 was booked checkout 2026-05-08 but
    /// the guest actually departed 2026-05-10; reconcile flagged
    /// canonical 2026-05-08 vs MSSQL 2026-05-10 forever).
    #[test]
    fn effective_checkout_prefers_actual_when_present() {
        use chrono::NaiveDate;
        let row = CanonicalCheckinRow {
            legacy_room_no: Some("518".into()),
            cin_checkin_time: None,
            cin_expected_checkout: Some(NaiveDate::from_ymd_opt(2026, 5, 8).unwrap()),
            cin_checkout_time: Some(
                NaiveDate::from_ymd_opt(2026, 5, 10)
                    .unwrap()
                    .and_hms_opt(12, 8, 0)
                    .unwrap(),
            ),
            legacy_cust_no: None,
            cin_status: None,
        };
        // Completed-extended stay: actual departure date wins over original
        // booking. Matches the legacy `Cin_Room_Out` value the MSSQL side
        // of the reconcile hashes.
        assert_eq!(
            row.effective_checkout_date(),
            Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap())
        );
    }

    /// Active stay: `cin_checkout_time IS NULL`, fall back to
    /// `cin_expected_checkout` so the hash matches legacy
    /// `Cin_Room_Out` (which holds the booked-future date until
    /// checkout).
    #[test]
    fn effective_checkout_falls_back_to_expected_when_not_yet_checked_out() {
        use chrono::NaiveDate;
        let row = CanonicalCheckinRow {
            legacy_room_no: Some("404".into()),
            cin_checkin_time: None,
            cin_expected_checkout: Some(NaiveDate::from_ymd_opt(2026, 5, 18).unwrap()),
            cin_checkout_time: None,
            legacy_cust_no: None,
            cin_status: None,
        };
        assert_eq!(
            row.effective_checkout_date(),
            Some(NaiveDate::from_ymd_opt(2026, 5, 18).unwrap())
        );
    }

    /// Edge case: row exists but no checkout dates yet (transient
    /// mid-edit state) — None passes through to the hash function's
    /// "" sentinel.
    #[test]
    fn effective_checkout_returns_none_when_neither_set() {
        let row = CanonicalCheckinRow {
            legacy_room_no: None,
            cin_checkin_time: None,
            cin_expected_checkout: None,
            cin_checkout_time: None,
            legacy_cust_no: None,
            cin_status: None,
        };
        assert_eq!(row.effective_checkout_date(), None);
    }

    // -------------------------------------------------------------------
    // Cancelled-folio sentinel (2026-05-19 — reconcile cleanup PR B)
    // -------------------------------------------------------------------
    //
    // When iHOTEL cancels a check-in it deletes the per-room
    // `HT_CheckIn_Ds` rows, so the CT mapper's `derive_room_state`
    // emits `canonical_status='cancelled'` with `first_room_no=None`.
    // Canonical PG keeps the original `legacy_room_no` on the
    // existing `ht_checkins` row. Hashing room context on both sides
    // therefore diverges forever; the live audit on 2026-05-19 found
    // six stuck `value` drifts on HF Hotel (CH26-005252, CH26-005270,
    // CH26-005487, CH26-005524, CH26-005527, CH26-005543) all of
    // which share that exact shape.
    //
    // The `cancelled` flag on `checkin_canonical_hash` collapses both
    // sides onto a sentinel that ignores room/time/customer context.
    // These tests pin (a) the sentinel format, (b) the cross-side
    // equality the production fix delivers, and (c) the regression
    // guarantee that the active-stay path keeps its existing
    // 5-field-shape semantics.

    #[test]
    fn checkin_canonical_hash_returns_cancelled_sentinel_independent_of_room() {
        // With `cancelled = true` the function must IGNORE every input
        // except `legacy_cin_no` — garbage in any other slot must not
        // change the bytes. Pin the exact sentinel format so a future
        // refactor that drops the `CANCELLED|` prefix or swaps the
        // PK position trips this test.
        let expected = sha256("CANCELLED|CH26-005252");

        let with_room_and_dates = checkin_canonical_hash(
            "CH26-005252",
            Some("301"),
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            true,
        );
        assert_eq!(with_room_and_dates, expected);

        // Same PK, completely different room/time/customer garbage —
        // sentinel still wins (checked_out flag is ignored too).
        let with_other_garbage = checkin_canonical_hash(
            "CH26-005252",
            Some("999"),
            Some("1999-01-01 00:00:00"),
            Some("2099-12-31"),
            Some("CXXXX"),
            true,
            true,
        );
        assert_eq!(with_other_garbage, expected);

        // All-None on the ignored slots still produces the same hash.
        let with_all_none =
            checkin_canonical_hash("CH26-005252", None, None, None, None, false, true);
        assert_eq!(with_all_none, expected);
    }

    #[test]
    fn cancelled_folio_hashes_match_across_legacy_and_pg_sides() {
        // Models the exact production scenario for the six stuck
        // CH26-* PKs: iHOTEL has deleted the per-room
        // `HT_CheckIn_Ds`, so the legacy projection lands with
        // `legacy_room_no = None`; canonical PG retains
        // `legacy_room_no = Some("301")` on the original
        // `ht_checkins` row. Pre-fix the two hashes differed on the
        // empty-vs-room slot forever. With `cancelled = true` both
        // sides collapse onto the sentinel and converge.
        let legacy_side = checkin_canonical_hash(
            "CH26-005252",
            None, // post-cancel: HT_CheckIn_Ds deleted, no first_room_no
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            true,
        );
        let pg_side = checkin_canonical_hash(
            "CH26-005252",
            Some("301"), // canonical kept the original room for triage
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            true,
        );
        assert_eq!(
            legacy_side, pg_side,
            "cancelled sentinel must collapse the legacy/PG room-context gap"
        );
        // Pin the exact sentinel bytes — a future refactor that
        // breaks the format will trip this assertion alongside the
        // cross-side equality.
        assert_eq!(legacy_side, sha256("CANCELLED|CH26-005252"));
    }

    #[test]
    fn active_folio_hash_unaffected_by_cancelled_flag_being_false() {
        // Regression guard: with `cancelled = false` the function
        // must keep its existing pre-2026-05-19 5-field hash shape
        // bit-for-bit. Recomputed here from the legacy `format!`
        // string the implementation uses so the test fails loudly if
        // the active-stay path is ever refactored to a different
        // shape.
        let actual = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            false,
        );
        let expected = sha256("CIN001|101|2026-04-01 14:00:00|2026-04-03|C001|co=false");
        assert_eq!(
            actual, expected,
            "active-stay hash shape must be preserved (5 fields + co= checked_out flag) when cancelled=false"
        );
        // And the active hash MUST NOT equal the cancelled sentinel
        // for the same PK — otherwise the cancelled-vs-active
        // distinction collapses and we lose state-change drift
        // detection on the transition itself.
        assert_ne!(actual, sha256("CANCELLED|CIN001"));
    }

    // -------------------------------------------------------------------
    // Track D / T7 CRIT-1 — cardinality-aware reconcile
    // -------------------------------------------------------------------

    #[test]
    fn classify_divergence_returns_missing_pg_when_pg_hash_absent() {
        let kind = classify_divergence(None, Some("abc"), 1, 0);
        assert_eq!(kind, DivergenceKind::MissingPg);
    }

    #[test]
    fn classify_divergence_returns_missing_mssql_when_mssql_hash_absent() {
        let kind = classify_divergence(Some("abc"), None, 0, 1);
        assert_eq!(kind, DivergenceKind::MissingMssql);
    }

    #[test]
    fn classify_divergence_returns_cardinality_when_row_counts_differ() {
        // Multi-room folio: 3 legacy `View_CheckIn_Ds` rows collapsed
        // into 1 canonical row. Hashes will differ forever — the kind
        // discriminator surfaces the actionable root cause.
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 3, 1);
        assert_eq!(kind, DivergenceKind::Cardinality);
    }

    #[test]
    fn classify_divergence_returns_value_when_counts_match_and_hashes_differ() {
        // Pre-condition: caller has already determined hashes differ.
        // Same row count on both sides ⇒ pure content drift.
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 1, 1);
        assert_eq!(kind, DivergenceKind::Value);
    }

    /// Track D / T7 CRIT-1 invariant — cardinality divergences must NEVER
    /// be silenced via the ack cache. The reconcile loop reads
    /// `is_silenceable()` before calling `ack_*_mirror`; if this test
    /// regresses, multi-room folios would re-acquire the silent-failure
    /// behaviour the post-mortem exposed.
    #[test]
    fn cardinality_kind_never_silenced() {
        assert!(
            !DivergenceKind::Cardinality.is_silenceable(),
            "Cardinality drift must never be silenced — the underlying \
             schema asymmetry isn't repaired by hash alone."
        );
    }

    /// Track D / T7 HIGH-1 corollary — `pg_hash IS NULL` (canonical
    /// missing) is the highest-signal divergence and must never silence.
    /// Compounds with `cardinality_kind_never_silenced` to lock the two
    /// non-silenceable kinds.
    #[test]
    fn missing_pg_kind_never_silenced() {
        assert!(
            !DivergenceKind::MissingPg.is_silenceable(),
            "Canonical-missing rows must never be silenced — they're the \
             highest-signal divergence the reconciler surfaces."
        );
    }

    #[test]
    fn value_and_missing_mssql_kinds_are_silenceable() {
        // Value drift acks-on-hash by design — a one-shot CT regression
        // becomes silent once the canonical mapper catches up.
        assert!(DivergenceKind::Value.is_silenceable());
        // MissingMssql means writeback dropped the row — the alert
        // fires once at detection; subsequent ticks are silent until
        // writeback regenerates the row.
        assert!(DivergenceKind::MissingMssql.is_silenceable());
    }

    #[test]
    fn divergence_kind_as_str_matches_schema_constraint_values() {
        // The migration 032 column doc lists these four exact strings;
        // the alerting layer + dashboards depend on them.
        assert_eq!(DivergenceKind::Value.as_str(), "value");
        assert_eq!(DivergenceKind::Cardinality.as_str(), "cardinality");
        assert_eq!(DivergenceKind::MissingPg.as_str(), "missing_pg");
        assert_eq!(DivergenceKind::MissingMssql.as_str(), "missing_mssql");
    }

    // -------------------------------------------------------------------
    // Track D / T7 HIGH-1 — level-triggered drift digest cooldown
    // -------------------------------------------------------------------

    #[test]
    fn cooldown_elapsed_returns_true_when_no_prior_alert() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        assert!(cooldown_elapsed(None, now, cooldown));
    }

    #[test]
    fn cooldown_elapsed_returns_false_inside_window() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        // Just fired → 0 seconds elapsed → not eligible.
        assert!(!cooldown_elapsed(Some(now), now, cooldown));
    }

    #[test]
    fn cooldown_elapsed_returns_true_after_window() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400); // 24h
                                                               // Fired 25 hours ago — window elapsed.
        let last = now - chrono::Duration::hours(25);
        assert!(cooldown_elapsed(Some(last), now, cooldown));
    }

    /// Edge case: clock-skew anomaly where `last_alerted_at` is
    /// somehow in the future relative to `now`. The `to_std()` cast
    /// errors; we treat it as eligible so we don't get stuck refusing
    /// alerts forever on a clock anomaly. Per-site/per-table
    /// uniqueness is enforced by the PG primary key, so the
    /// in-memory key-construction tests from the pre-2026-05-16
    /// implementation are no longer needed.
    #[test]
    fn cooldown_elapsed_handles_future_last_alerted_via_clock_skew() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        let future = now + chrono::Duration::hours(1);
        assert!(cooldown_elapsed(Some(future), now, cooldown));
    }

    // -------------------------------------------------------------------
    // Auto-resolve sweep — pure decision helper
    // -------------------------------------------------------------------

    /// The sweep MUST mark a row resolved when the freshly-computed
    /// canonical PG hash equals the freshly-projected legacy hash.
    /// Pre-condition for the auto-resolve UPDATE — if this regresses,
    /// the sweep silently stops draining the queue.
    #[test]
    fn auto_resolve_sweep_marks_resolved_when_hashes_match() {
        let current_legacy_hash = Some("hash-X");
        let current_pg_hash = Some("hash-X");
        assert!(should_auto_resolve(
            "bookings",
            current_legacy_hash,
            current_pg_hash,
            None,
        ));
    }

    /// The sweep MUST leave the row untouched when canonical still
    /// diverges from the freshly-projected legacy hash. Drift that
    /// persists is exactly what the alerts are supposed to surface.
    #[test]
    fn auto_resolve_sweep_skips_when_drift_persists() {
        let current_legacy_hash = Some("hash-X");
        let current_pg_hash = Some("hash-Y");
        assert!(!should_auto_resolve(
            "bookings",
            current_legacy_hash,
            current_pg_hash,
            None,
        ));
    }

    /// Stale-ghost convergence: a `bookings` row whose legacy composite
    /// key has vanished (`current_legacy_hash == None`) but whose
    /// canonical PG hash now matches the legacy state recorded at
    /// detection (`recorded_mssql_hash`) MUST auto-resolve. This is the
    /// `R015423|501` class — a `missing_pg` row logged before the
    /// new-app booking linked `legacy_book_id`, stranded once its
    /// initial room-type line was swapped out. Without this it sits
    /// open forever and trips the >4h `level_drift_alert`.
    #[test]
    fn auto_resolve_sweep_resolves_booking_stale_ghost() {
        let recorded = booking_canonical_hash(
            "R015423",
            Some("2026-07-04"),
            Some("2026-07-05"),
            Some("C22381"),
        );
        let current_pg_hash = recorded.clone();
        assert!(should_auto_resolve(
            "bookings",
            None, // legacy room-type line is gone
            Some(current_pg_hash.as_str()),
            Some(recorded.as_str()),
        ));
    }

    /// Negative: a genuine, persistent `missing_pg` booking (canonical
    /// still absent today) MUST NOT be swept closed by the stale-ghost
    /// arm. The legacy side recorded a hash, but PG never caught up —
    /// this is real, un-reconciled drift.
    #[test]
    fn auto_resolve_sweep_keeps_open_persistent_missing_pg_booking() {
        assert!(!should_auto_resolve(
            "bookings",
            None,           // legacy key gone
            None,           // canonical still missing
            Some("hash-X"), // legacy recorded a hash at detection
        ));
    }

    /// Negative: the stale-ghost arm fires only when canonical matches
    /// the RECORDED legacy state. A `bookings` row where the legacy key
    /// vanished but canonical PG diverges from what legacy had recorded
    /// is real drift (the booking's dates/customer changed in PG
    /// independently) and must stay open.
    #[test]
    fn auto_resolve_sweep_keeps_open_booking_ghost_with_mismatched_canonical() {
        assert!(!should_auto_resolve(
            "bookings",
            None,
            Some("hash-current-pg"),
            Some("hash-recorded-legacy"),
        ));
    }

    /// Composite-PK parse contract: a `"book_no|room_type"` PK splits
    /// on the first `|`. The booking-fetch dispatch + the
    /// canonical-fetch dispatch must agree on this shape or the
    /// auto-resolve sweep compares hashes from different groups.
    #[test]
    fn parse_booking_legacy_pk_splits_composite() {
        assert_eq!(parse_booking_legacy_pk("B12345|A"), ("B12345", "A"));
    }

    /// Empty `room_type_key` (the `unwrap_or_default()` case in
    /// `sync_bookings`) round-trips through the PK serialisation as
    /// `"{book_no}|"`. Must still parse cleanly so an empty-key group
    /// can be re-fetched and resolved.
    #[test]
    fn parse_booking_legacy_pk_handles_empty_room_type() {
        assert_eq!(parse_booking_legacy_pk("B12345|"), ("B12345", ""));
    }

    /// Pre-Phase-6-hotfix `ht_reconcile_log` rows lack the `|` separator
    /// (older format stored just `book_no`). Treat those as
    /// `(legacy_pk, "")` so the sweep doesn't error on legacy data.
    #[test]
    fn parse_booking_legacy_pk_legacy_format_has_no_separator() {
        assert_eq!(parse_booking_legacy_pk("B12345"), ("B12345", ""));
    }

    // -------------------------------------------------------------------
    // Incident 2026-05-18 follow-up — per-tick CT watcher lag observation
    // -------------------------------------------------------------------
    //
    // `check_ct_watcher_lag` is the live MSSQL+PG observation; the
    // tests below cover only the pure decision helper
    // (`ct_lag_is_warning`) and the env-resolution helper
    // (`ct_lag_thresholds_from_env`). The plumbing through the
    // best-effort PG/MSSQL queries is exercised by integration tests
    // when the legacy spike harness is wired up — out of scope here.

    fn default_ct_thresholds() -> CtLagThresholds {
        CtLagThresholds {
            version_lag: DEFAULT_CT_LAG_WARN_VERSIONS,
            poll_age_seconds: DEFAULT_CT_LAG_WARN_SECONDS,
        }
    }

    /// Both lag dimensions below threshold → debug-level log (warning
    /// branch must NOT fire). The hot path: a healthy watcher in the
    /// steady-state idle period between CT bumps.
    #[test]
    fn ct_lag_under_both_thresholds_does_not_warn() {
        let t = default_ct_thresholds();
        assert!(!ct_lag_is_warning(0, 0, t));
        assert!(!ct_lag_is_warning(50, 60, t));
        // Exactly at threshold ⇒ debug (strict greater-than, mirrors
        // `tables_breaching_threshold`).
        assert!(!ct_lag_is_warning(
            DEFAULT_CT_LAG_WARN_VERSIONS,
            DEFAULT_CT_LAG_WARN_SECONDS,
            t,
        ));
    }

    /// Version lag breaches alone are sufficient to warn — a CT-watcher
    /// that's behind on rows is the actionable case even if it's still
    /// polling fast enough to keep `last_polled_at` fresh.
    #[test]
    fn ct_lag_warns_when_only_version_lag_breaches() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(DEFAULT_CT_LAG_WARN_VERSIONS + 1, 0, t));
    }

    /// Poll-age breaches alone are sufficient to warn — a watcher whose
    /// poll-loop is wedged still keeps `last_seen_version` advancing
    /// from the in-flight transaction, but `last_polled_at` stops
    /// refreshing (incident-2026-05-18 root-cause shape).
    #[test]
    fn ct_lag_warns_when_only_poll_age_breaches() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(0, DEFAULT_CT_LAG_WARN_SECONDS + 1, t));
    }

    /// Both dimensions breached → warn. Defensive smoke test — if either
    /// arm of the `||` regresses to `&&`, this catches it together with
    /// the two "only X breaches" tests above.
    #[test]
    fn ct_lag_warns_when_both_dimensions_breach() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(
            DEFAULT_CT_LAG_WARN_VERSIONS + 1,
            DEFAULT_CT_LAG_WARN_SECONDS + 1,
            t
        ));
    }

    /// Custom thresholds are honored — proves the helper doesn't
    /// hardwire the defaults. Matches the runtime behaviour where
    /// `LEGACY_CT_LAG_WARN_VERSIONS` / `LEGACY_CT_LAG_WARN_SECONDS`
    /// override the compiled-in constants.
    #[test]
    fn ct_lag_respects_custom_thresholds() {
        let tight = CtLagThresholds {
            version_lag: 10,
            poll_age_seconds: 30,
        };
        assert!(!ct_lag_is_warning(10, 30, tight));
        assert!(ct_lag_is_warning(11, 0, tight));
        assert!(ct_lag_is_warning(0, 31, tight));
    }

    /// Env-isolation helper for CT-lag threshold tests. Same shape as
    /// `with_threshold_envs` above. Tracks BOTH the global vars and the
    /// per-site overrides so the fallback chain can be exercised
    /// without leaking state across tests.
    fn with_ct_lag_envs<F: FnOnce() -> CtLagThresholds>(
        global_versions: Option<&str>,
        global_seconds: Option<&str>,
        per_site_versions: Option<(&str, &str)>,
        per_site_seconds: Option<(&str, &str)>,
        f: F,
    ) -> CtLagThresholds {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();

        let prior_gv = env::var("LEGACY_CT_LAG_WARN_VERSIONS").ok();
        let prior_gs = env::var("LEGACY_CT_LAG_WARN_SECONDS").ok();
        let prior_psv = per_site_versions.map(|(n, _)| (n.to_string(), env::var(n).ok()));
        let prior_pss = per_site_seconds.map(|(n, _)| (n.to_string(), env::var(n).ok()));

        match global_versions {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_VERSIONS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_VERSIONS"),
        }
        match global_seconds {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_SECONDS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_SECONDS"),
        }
        if let Some((name, value)) = per_site_versions {
            env::set_var(name, value);
        }
        if let Some((name, value)) = per_site_seconds {
            env::set_var(name, value);
        }

        let out = f();

        match prior_gv {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_VERSIONS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_VERSIONS"),
        }
        match prior_gs {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_SECONDS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_SECONDS"),
        }
        if let Some((name, prior)) = prior_psv {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        if let Some((name, prior)) = prior_pss {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        out
    }

    #[test]
    fn ct_lag_thresholds_default_when_envs_unset() {
        let t = with_ct_lag_envs(None, None, None, None, || {
            ct_lag_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.version_lag, DEFAULT_CT_LAG_WARN_VERSIONS);
        assert_eq!(t.poll_age_seconds, DEFAULT_CT_LAG_WARN_SECONDS);
    }

    #[test]
    fn ct_lag_thresholds_use_global_envs_when_set() {
        let t = with_ct_lag_envs(Some("250"), Some("900"), None, None, || {
            ct_lag_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.version_lag, 250);
        assert_eq!(t.poll_age_seconds, 900);
    }

    /// Per-site override wins when both are set — same pattern as the
    /// drift-alert threshold per-site override (task #69).
    #[test]
    fn ct_lag_thresholds_per_site_override_wins() {
        let t = with_ct_lag_envs(
            Some("250"),
            Some("900"),
            Some(("LEGACY_CT_LAG_WARN_VERSIONS_HFVILLE", "20")),
            Some(("LEGACY_CT_LAG_WARN_SECONDS_HFVILLE", "60")),
            || ct_lag_thresholds_from_env("hfville"),
        );
        assert_eq!(t.version_lag, 20);
        assert_eq!(t.poll_age_seconds, 60);
    }

    /// Per-site override is namespaced by site id — an HF Hotel tick
    /// must NOT pick up `..._HFVILLE` (mirrors the
    /// `threshold_per_site_does_not_leak_across_sites` invariant).
    #[test]
    fn ct_lag_thresholds_per_site_does_not_leak_across_sites() {
        let t = with_ct_lag_envs(
            Some("250"),
            Some("900"),
            Some(("LEGACY_CT_LAG_WARN_VERSIONS_HFVILLE", "20")),
            Some(("LEGACY_CT_LAG_WARN_SECONDS_HFVILLE", "60")),
            || ct_lag_thresholds_from_env("hfhotel"),
        );
        assert_eq!(t.version_lag, 250);
        assert_eq!(t.poll_age_seconds, 900);
    }

    /// Garbage in any env falls through to the next layer (per-site →
    /// global → default). Operator typo on the override must not
    /// silence the observation.
    #[test]
    fn ct_lag_thresholds_garbage_falls_through_to_default() {
        let t = with_ct_lag_envs(Some("abc"), Some("-1"), None, None, || {
            ct_lag_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.version_lag, DEFAULT_CT_LAG_WARN_VERSIONS);
        assert_eq!(t.poll_age_seconds, DEFAULT_CT_LAG_WARN_SECONDS);
    }

    // -------------------------------------------------------------------
    // 2026-05-19 — `count_distinct_legacy_checkin_rooms` dedup invariants.
    //
    // Backs the CH22-000722 cardinality false-positive fix: iHOTEL's
    // HT_CheckIn_Ds table commonly carries multiple detail rows for the
    // same room (extends, re-keys, deposit returns). The
    // `ht_checkin_rooms` junction's `UNIQUE (cr_cin_id, cr_room_id)`
    // already collapses them; the cardinality check has to as well or
    // every such folio shows up as drift.
    // -------------------------------------------------------------------
    use crate::sync::row::test_support::{
        HashMapRow as TestHashMapRow, MockValue as TestMockValue,
    };

    fn ds_row_with_room(room_no: TestMockValue) -> TestHashMapRow {
        TestHashMapRow::new("HT_CheckIn_Ds").with("Cin_Room_No", room_no)
    }

    #[test]
    fn cardinality_dedups_duplicate_room_in_legacy_ds() {
        // CH22-000722 shape: three Ds rows, all room 417. Distinct
        // count must collapse to 1 to match the junction.
        let rooms = vec![
            ds_row_with_room(TestMockValue::Str("417".into())),
            ds_row_with_room(TestMockValue::Str("417".into())),
            ds_row_with_room(TestMockValue::Str("417".into())),
        ];
        assert_eq!(count_distinct_legacy_checkin_rooms(&rooms), 1);
    }

    #[test]
    fn cardinality_preserves_distinct_multi_room() {
        // A real multi-room folio (rooms 417 + 418) still surfaces as
        // distinct = 2, so a short junction (only one room mirrored)
        // still flags `cardinality` drift correctly.
        let rooms = vec![
            ds_row_with_room(TestMockValue::Str("417".into())),
            ds_row_with_room(TestMockValue::Str("418".into())),
        ];
        assert_eq!(count_distinct_legacy_checkin_rooms(&rooms), 2);
    }

    #[test]
    fn cardinality_skips_null_and_empty_room_no() {
        // Mirrors `project_rooms` skip semantic: NULL and empty
        // `Cin_Room_No` are dropped from the junction projection, so
        // they must also be dropped from the legacy cardinality count.
        let rooms = vec![
            ds_row_with_room(TestMockValue::Null),
            ds_row_with_room(TestMockValue::Str("".into())),
            ds_row_with_room(TestMockValue::Str("417".into())),
        ];
        assert_eq!(count_distinct_legacy_checkin_rooms(&rooms), 1);
    }

    // -------------------------------------------------------------------
    // 2026-07-27 — `missing_pg` re-ingest arm of the auto-resolve sweep.
    //
    // Live shape that motivated the arm: a 2026-07-11 cross-table
    // watermark clobber on HF Ville dropped a CT event, leaving customer
    // `C2413` and bookings `R002066|110` / `|112` / `|217` unconverged for
    // 16 days. Legacy still had every row; canonical had none. No existing
    // path could ever close them — `should_auto_resolve` needs BOTH hashes,
    // and CT's 2-day retention means the watcher can never redeliver.
    // -------------------------------------------------------------------

    /// Env-isolation helper for the two self-heal feature flags. Same
    /// save/restore shape as `with_mode_env` / `with_threshold_envs` above;
    /// one process-wide lock because `set_var` is process-wide.
    /// Takes a SLICE of vars rather than one, so a test that needs two flags
    /// set at once passes them together. **Do NOT nest calls** — the guard is
    /// a plain non-reentrant `Mutex`, so a nested call self-deadlocks (the
    /// test binary hangs forever rather than failing).
    fn with_env_vars<T, F: FnOnce() -> T>(vars: &[(&str, Option<&str>)], f: F) -> T {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let priors: Vec<(String, Option<String>)> = vars
            .iter()
            .map(|(name, _)| ((*name).to_string(), env::var(name).ok()))
            .collect();
        for (name, value) in vars {
            match value {
                Some(v) => env::set_var(name, v),
                None => env::remove_var(name),
            }
        }
        let out = f();
        for (name, prior) in priors {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        out
    }

    /// Single-var convenience wrapper over [`with_env_vars`].
    fn with_self_heal_flag_env<T, F: FnOnce() -> T>(
        var_name: &str,
        value: Option<&str>,
        f: F,
    ) -> T {
        with_env_vars(&[(var_name, value)], f)
    }

    const REINGEST_FLAG: &str = "RECONCILE_REINGEST_MISSING_PG_ENABLED";
    const FORCE_CONVERGE_FLAG: &str = "RECONCILE_FORCE_CONVERGE_ENABLED";

    #[test]
    fn reingest_flag_defaults_off_when_env_unset() {
        let on =
            with_self_heal_flag_env(REINGEST_FLAG, None, reconcile_reingest_missing_pg_enabled);
        assert!(
            !on,
            "the missing_pg re-ingest arm writes canonical state — it must ship dark"
        );
    }

    #[test]
    fn reingest_flag_on_for_exact_true_literal() {
        let on = with_self_heal_flag_env(REINGEST_FLAG, Some("true"), {
            reconcile_reingest_missing_pg_enabled
        });
        assert!(on);
    }

    /// The `== "true"` comparison is strict on purpose. An operator who
    /// types `TRUE` / `1` / ` true` gets the SAFE (off) behaviour rather
    /// than a silently-enabled canonical-write path.
    #[test]
    fn reingest_flag_is_strict_about_the_true_literal() {
        for value in ["TRUE", "True", "1", "yes", " true", "true ", ""] {
            let on = with_self_heal_flag_env(REINGEST_FLAG, Some(value), {
                reconcile_reingest_missing_pg_enabled
            });
            assert!(!on, "{value:?} must NOT enable the re-ingest arm");
        }
    }

    /// The re-ingest flag must be independent of the force-converge flag:
    /// `RECONCILE_FORCE_CONVERGE_ENABLED` is already `true` in production
    /// on `sync-hfville`, so sharing it would have shipped the new
    /// canonical-write class ON with no coordinated flip.
    #[test]
    fn reingest_flag_is_independent_of_force_converge_flag() {
        // Both vars go through ONE `with_env_vars` call — nesting two
        // guarded calls would self-deadlock on the shared Mutex.
        let on = with_env_vars(
            &[(FORCE_CONVERGE_FLAG, Some("true")), (REINGEST_FLAG, None)],
            reconcile_reingest_missing_pg_enabled,
        );
        assert!(
            !on,
            "force-converge being ON must never imply the missing_pg re-ingest arm is ON"
        );
    }

    /// Pre-existing coverage gap: the force-converge flag reader had no
    /// test at all. Pin its default and its strictness too.
    #[test]
    fn force_converge_flag_defaults_off_and_is_strict() {
        let unset = with_self_heal_flag_env(FORCE_CONVERGE_FLAG, None, {
            reconcile_force_converge_enabled
        });
        assert!(!unset);
        let loose = with_self_heal_flag_env(FORCE_CONVERGE_FLAG, Some("TRUE"), {
            reconcile_force_converge_enabled
        });
        assert!(!loose);
        let exact = with_self_heal_flag_env(FORCE_CONVERGE_FLAG, Some("true"), {
            reconcile_force_converge_enabled
        });
        assert!(exact);
    }

    const LEGACY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const PG_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    /// Comfortably past `FORCE_CONVERGE_MIN_AGE_SECS` (3600s) — the live
    /// rows had aged 16 days.
    const OLD_ENOUGH_SECS: f64 = FORCE_CONVERGE_MIN_AGE_SECS + 1.0;

    #[test]
    fn reingest_arm_eligible_when_legacy_present_and_canonical_absent() {
        // The exact live shape: customer C2413 / booking R002066|110.
        for table in ["customers", "bookings"] {
            assert!(
                reingest_missing_pg_eligible(table, Some(LEGACY_HASH), None, OLD_ENOUGH_SECS, true,),
                "{table}: legacy present + canonical absent + past min age must be eligible"
            );
        }
    }

    /// The genuine-anomaly guard. A VANISHED legacy row must never trip the
    /// re-ingest arm — there is nothing to project from, and closing the
    /// row would hide a real deletion. Same invariant
    /// `rooms_dispatch_missing_legacy_row_does_not_auto_resolve` pins for
    /// the observational path.
    #[test]
    fn reingest_arm_not_eligible_when_legacy_row_vanished() {
        for table in ["customers", "bookings"] {
            assert!(
                !reingest_missing_pg_eligible(table, None, None, OLD_ENOUGH_SECS, true),
                "{table}: a vanished legacy row must stay open for operator review"
            );
        }
        // An empty-string legacy hash is treated the same as absent.
        assert!(!reingest_missing_pg_eligible(
            "customers",
            Some(""),
            None,
            OLD_ENOUGH_SECS,
            true,
        ));
        // …and the row must also stay open on the observational path.
        assert!(!should_auto_resolve(
            "customers",
            None,
            None,
            Some(LEGACY_HASH)
        ));
    }

    #[test]
    fn reingest_arm_not_eligible_below_min_age() {
        assert!(
            !reingest_missing_pg_eligible(
                "customers",
                Some(LEGACY_HASH),
                None,
                FORCE_CONVERGE_MIN_AGE_SECS - 1.0,
                true,
            ),
            "a fresh divergence is probably an in-flight CT event — don't race it"
        );
        // Exactly at the threshold IS eligible (`>=`).
        assert!(reingest_missing_pg_eligible(
            "customers",
            Some(LEGACY_HASH),
            None,
            FORCE_CONVERGE_MIN_AGE_SECS,
            true,
        ));
    }

    #[test]
    fn reingest_arm_not_eligible_when_flag_off() {
        assert!(
            !reingest_missing_pg_eligible(
                "bookings",
                Some(LEGACY_HASH),
                None,
                OLD_ENOUGH_SECS,
                false,
            ),
            "flag OFF ⇒ zero canonical writes"
        );
    }

    /// `rooms` / `checkins` are outside the arm: a room absent from
    /// canonical is a provisioning gap rather than a dropped ingest, and
    /// the check-in aggregate re-ingest hasn't been verified the same way.
    #[test]
    fn reingest_arm_not_eligible_for_unsupported_tables() {
        for table in ["rooms", "checkins", "payments"] {
            assert!(
                !reingest_missing_pg_eligible(
                    table,
                    Some(LEGACY_HASH),
                    None,
                    OLD_ENOUGH_SECS,
                    true,
                ),
                "{table} must not be re-ingested by this arm"
            );
        }
    }

    /// Value drift (BOTH hashes present) keeps routing to the pre-existing
    /// force-converge arm, and must NOT be picked up by the new arm.
    #[test]
    fn value_drift_still_routes_to_force_converge_arm_unchanged() {
        for table in ["customers", "rooms"] {
            assert!(
                force_converge_value_drift_eligible(
                    table,
                    Some(LEGACY_HASH),
                    Some(PG_HASH),
                    OLD_ENOUGH_SECS,
                    true,
                ),
                "{table} value drift must still reach the force-converge arm"
            );
        }
        assert!(
            !reingest_missing_pg_eligible(
                "customers",
                Some(LEGACY_HASH),
                Some(PG_HASH),
                OLD_ENOUGH_SECS,
                true,
            ),
            "a value drift is not a dropped ingest — the re-ingest arm must ignore it"
        );
    }

    /// Symmetric guard: the force-converge arm keeps its "BOTH hashes
    /// present" precondition, so the missing_pg shape never reaches it.
    #[test]
    fn force_converge_arm_ignores_the_missing_pg_shape() {
        assert!(!force_converge_value_drift_eligible(
            "customers",
            Some(LEGACY_HASH),
            None,
            OLD_ENOUGH_SECS,
            true,
        ));
        assert!(!force_converge_value_drift_eligible(
            "bookings",
            Some(LEGACY_HASH),
            Some(PG_HASH),
            OLD_ENOUGH_SECS,
            true,
        ));
    }

    // -------------------------------------------------------------------
    // Phase 6-A — payments reconcile arm (DARK). Pure tests only; the
    // MSSQL/PG halves are pinned by shape guards + the descriptor-table
    // golden vector in `sync::mappers::payment`.
    // -------------------------------------------------------------------

    /// Ships DARK. The default MUST be off on every service, and the
    /// literal comparison is strict — `"TRUE"` / `"1"` / `" true"` are all
    /// off, matching every other flag in the sync path.
    #[test]
    fn payments_arm_flag_defaults_off_and_is_strict() {
        assert!(
            !with_env_vars(&[("RECONCILE_PAYMENTS_ARM_ENABLED", None)], || {
                reconcile_payments_arm_enabled()
            }),
            "the payments arm must default OFF — enabling is a coordinated action"
        );
        assert!(with_env_vars(
            &[("RECONCILE_PAYMENTS_ARM_ENABLED", Some("true"))],
            || { reconcile_payments_arm_enabled() }
        ));
        for sloppy in ["TRUE", "1", "yes", " true", "True"] {
            assert!(
                !with_env_vars(
                    &[("RECONCILE_PAYMENTS_ARM_ENABLED", Some(sloppy))],
                    || { reconcile_payments_arm_enabled() }
                ),
                "`{sloppy}` must NOT enable the arm"
            );
        }
    }

    /// The arm ships detection-only. Wiring it into either self-heal list
    /// is a separate, coordinated decision (plan 6-D) — and one of its
    /// divergence shapes (canonical-only void) would be ERASED rather than
    /// repaired by re-driving the legacy row.
    #[test]
    fn payments_is_not_wired_into_either_self_heal_arm() {
        assert!(
            !FORCE_CONVERGE_VALUE_DRIFT_TABLES.contains(&"payments"),
            "payments detection must soak before any force-converge is wired"
        );
        assert!(
            !REINGEST_MISSING_PG_TABLES.contains(&"payments"),
            "payments must not be re-ingested by the missing_pg arm"
        );
        // …and the pure gates agree, even with the self-heal flags ON.
        assert!(!force_converge_value_drift_eligible(
            "payments",
            Some(LEGACY_HASH),
            Some(PG_HASH),
            OLD_ENOUGH_SECS,
            true,
        ));
        assert!(!reingest_missing_pg_eligible(
            "payments",
            Some(LEGACY_HASH),
            None,
            OLD_ENOUGH_SECS,
            true,
        ));
    }

    /// A payment's canonical parent is its check-in, so the sweep must
    /// heal check-ins BEFORE payments within a batch (`apply_receipt_upsert`
    /// ERRORS on an unresolvable parent).
    #[test]
    fn payments_rank_after_their_parent_checkin() {
        assert!(reconcile_table_fk_rank("checkins") < reconcile_table_fk_rank("payments"));
        assert!(reconcile_table_fk_rank("payments") < reconcile_table_fk_rank("something_new"));
    }

    /// Hash-body pin at the scheduler boundary (the mapper-side descriptor
    /// carries the byte-for-byte golden vector). Both sides of the arm call
    /// THIS function, so a converged receipt must hash identically whether
    /// projected from `HT_Receipt_H` or from `ht_payments`.
    #[test]
    fn payment_canonical_hash_matches_when_canonical_mirrors_legacy() {
        let legacy = LegacyReceiptRow {
            receipt_no: "B2604-0265".into(),
            receipt_total: 890.0,
            legacy_cin_no: Some("CH26-005228".into()),
            status_name: Some("ปกติ".into()),
        };
        let canonical = CanonicalPaymentRow {
            pay_amount: 890.0,
            pay_voided: Some(false),
            legacy_cin_no: Some("CH26-005228".into()),
        };
        assert_eq!(
            legacy.hash(),
            payment_canonical_hash(
                &legacy.receipt_no,
                canonical.pay_amount,
                canonical.is_voided(),
                canonical.legacy_cin_no.as_deref(),
            ),
        );
    }

    /// NULL `pay_voided` (the column is nullable, `DEFAULT false`) must
    /// read as NOT voided — the same `COALESCE(pay_voided, false)` the
    /// mapper's UPDATE and every money reader use. Treating NULL as
    /// "unknown" here would manufacture drift on every pre-void row.
    #[test]
    fn canonical_null_pay_voided_hashes_as_not_voided() {
        let null_voided = CanonicalPaymentRow {
            pay_amount: 500.0,
            pay_voided: None,
            legacy_cin_no: Some("CH26-000001".into()),
        };
        let explicit_false = CanonicalPaymentRow {
            pay_amount: 500.0,
            pay_voided: Some(false),
            legacy_cin_no: Some("CH26-000001".into()),
        };
        assert_eq!(null_voided.is_voided(), explicit_false.is_voided());
    }

    /// The void bit is a real hash input: a legacy cancel that canonical
    /// hasn't applied MUST diverge, and vice versa. This is the arm's whole
    /// reason for existing on the money path.
    #[test]
    fn payment_canonical_hash_diverges_on_void_state() {
        let normal = payment_canonical_hash("B1", 890.0, false, Some("CH1"));
        let voided = payment_canonical_hash("B1", 890.0, true, Some("CH1"));
        assert_ne!(normal, voided);
    }

    /// Amount drift is hashed at 2dp — legacy `Receipt_Total` is a bare
    /// `float`, canonical `pay_amount` a `DECIMAL(12,2)`, so float noise
    /// below the satang must NOT diverge while a real satang difference
    /// must.
    #[test]
    fn payment_amount_segment_is_two_decimals_and_zero_is_signless() {
        assert_eq!(
            payment_canonical_hash("B1", 890.0, false, None),
            payment_canonical_hash("B1", 890.000000001, false, None),
            "float noise below 2dp must not manufacture drift"
        );
        assert_ne!(
            payment_canonical_hash("B1", 890.00, false, None),
            payment_canonical_hash("B1", 890.01, false, None),
            "a one-satang difference is real drift"
        );
        // IEEE -0.0 renders as "-0.00" without normalisation, which would
        // make a zero-total receipt permanently unconvergeable.
        assert_eq!(money_hash_segment(-0.0), "0.00");
        assert_eq!(money_hash_segment(0.0), "0.00");
    }

    /// `Receipt_ref` carries the parent `Cin_no`; a re-pointed receipt is
    /// real drift (iHOTEL's customer-delete cascade touches this family).
    #[test]
    fn payment_canonical_hash_diverges_on_parent_checkin() {
        assert_ne!(
            payment_canonical_hash("B1", 890.0, false, Some("CH26-000001")),
            payment_canonical_hash("B1", 890.0, false, Some("CH26-000002")),
        );
        // A canonical payment whose parent check-in row has vanished
        // (LEFT JOIN → None) must not hash like one that is correctly
        // parented.
        assert_ne!(
            payment_canonical_hash("B1", 890.0, false, Some("CH26-000001")),
            payment_canonical_hash("B1", 890.0, false, None),
        );
    }

    /// SQL-shape pin for the canonical probe. It MUST mirror
    /// `payment::apply_receipt_upsert`'s existing-row lookup: probing only
    /// `pay_reference` would report every app-originated payment as
    /// `missing_pg` forever (the detection-side form of the 2026-06-30
    /// HF Ville phantom-duplicate echo).
    #[test]
    fn canonical_payment_probe_matches_the_mapper_lookup_shape() {
        // Pins the EXACT statement `fetch_canonical_payment` executes.
        let sql = CANONICAL_PAYMENT_PROBE_SQL;
        assert!(sql.contains("p.legacy_receipt_no = $1 OR p.pay_reference = $1"));
        assert!(sql.contains("ORDER BY (p.legacy_receipt_no = $1) DESC NULLS LAST"));
        assert!(
            sql.contains("LEFT JOIN ht_checkins"),
            "an INNER JOIN would misreport a parentless payment as missing_pg"
        );
    }

    /// Scan-filter pin: no-check-in receipts are a DELIBERATE mapper skip
    /// (`ht_payments.pay_cin_id` is NOT NULL), so including them would
    /// manufacture one permanent `missing_pg` row per counter sale.
    #[test]
    fn payments_scan_filter_excludes_receipts_without_a_checkin_ref() {
        assert_eq!(
            PAYMENTS_RECONCILE_SCAN_FILTER,
            "Receipt_ref IS NOT NULL AND Receipt_ref <> ''"
        );
    }

    /// The BLOCKING find of the 2026-07-28 review: without a canonical-era
    /// floor the arm hashes 21,566 HF Hotel receipts against 1,154 canonical
    /// payments, manufacturing >20k `missing_pg` rows that can NEVER close
    /// (not silenceable, not re-ingestable) and starving the auto-resolve
    /// sweep. The composed filter must carry the floor.
    #[test]
    fn payments_scan_filter_is_floored_at_the_canonical_era() {
        let floor = chrono::NaiveDate::from_ymd_opt(2026, 4, 27)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let filter = payments_reconcile_scan_filter(floor);
        assert!(
            filter.starts_with(PAYMENTS_RECONCILE_SCAN_FILTER),
            "the era floor must NARROW the ref filter, not replace it: {filter}"
        );
        assert!(
            filter.contains("Receipt_Date >= '2026-04-27T00:00:00'"),
            "floor literal must be the language-independent ODBC/ISO form: {filter}"
        );
        assert!(
            filter.contains("Receipt_Date IS NULL OR"),
            "a dateless receipt cannot be placed in or out of the era and the \
             mapper would still land it — dropping it re-creates a silent skip"
        );
    }

    /// The floor is DERIVED from canonical coverage and compared against
    /// `Receipt_Date` with NO timezone shift, because `ht_payments.pay_date`
    /// already holds the legacy value verbatim (`project_receipt` does a plain
    /// `try_get_datetime("Receipt_Date")`; the upsert's fallback is Bangkok
    /// wall-clock on purpose). `naive_thai_to_utc` applies to a different
    /// column (`Cin_Pay_Date` → `ledger_pay_date`, TIMESTAMPTZ) — conflating
    /// them once put a `+ INTERVAL '7 hours'` here, which moved the floor in
    /// the NARROWING direction and could drop the mirror's entire first day.
    /// The day truncation IS load-bearing: without it the floor cuts
    /// mid-afternoon on the mirror's very first day.
    #[test]
    fn payments_era_floor_sql_is_canonical_derived_and_unshifted() {
        assert!(PAYMENTS_ERA_FLOOR_SQL.contains("MIN(pay_date)"));
        assert!(PAYMENTS_ERA_FLOOR_SQL.contains("FROM ht_payments"));
        assert!(
            !PAYMENTS_ERA_FLOOR_SQL.contains("INTERVAL"),
            "pay_date is already the legacy naive-Thai value verbatim — any \
             offset here narrows the floor and silently drops in-era receipts"
        );
        assert!(
            PAYMENTS_ERA_FLOOR_SQL.contains("date_trunc('day'"),
            "widen to the start of the day so the mirror's first day is fully covered"
        );
    }

    /// A canonical-only void never moves the LEGACY hash, so the ack
    /// short-circuit would hide it forever once a receipt is acked. The
    /// carve-out re-opens exactly that pair — and nothing else, because every
    /// other transition does move the legacy hash.
    #[test]
    fn ack_short_circuit_bypassed_only_for_canonical_only_void() {
        assert!(
            payment_ack_short_circuit_bypassed(true, false),
            "canonical voided + legacy normal is the one shape the legacy hash \
             cannot express — it must bypass the ack"
        );
        assert!(
            !payment_ack_short_circuit_bypassed(false, true),
            "a legacy cancel already moves the legacy hash; no bypass needed"
        );
        assert!(!payment_ack_short_circuit_bypassed(false, false));
        assert!(
            !payment_ack_short_circuit_bypassed(true, true),
            "both sides voided means the hashes agree — bypassing would only \
             buy a wasted canonical probe every tick"
        );
    }

    /// Track J1 — projection lock. `Receipt_Date` / VAT columns must stay
    /// OUT (see `payment_canonical_hash` on why `pay_date` is excluded).
    #[test]
    fn payments_reconcile_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(PAYMENTS_RECONCILE_PROJECTION, "HT_Receipt_H");
        assert!(
            !PAYMENTS_RECONCILE_PROJECTION.contains(&"Receipt_Date"),
            "pay_date is COALESCE-preserved for app rows — hashing it is permanent false drift"
        );
    }

    // -------------------------------------------------------------------
    // Phase 6-B — guest-registry (companion folio) reconcile arm (DARK).
    // Pure tests only; the byte-parity golden vector for the folio hash
    // lives with the descriptor table in `sync::mappers::guest_registry`.
    // -------------------------------------------------------------------

    fn test_folio(cin_no: &str, companions: &[(&str, Option<&str>)]) -> RegistryFolioProjection {
        let mut f = RegistryFolioProjection::empty(cin_no);
        for (name, country) in companions {
            f.push_companion(name, *country);
        }
        f
    }

    /// Ships DARK. The default MUST be off on every service, and the
    /// literal comparison is strict — `"TRUE"` / `"1"` / `" true"` are all
    /// off, matching every other flag in the sync path.
    #[test]
    fn guest_registry_arm_flag_defaults_off_and_is_strict() {
        assert!(
            !with_env_vars(&[("RECONCILE_GUEST_REGISTRY_ARM_ENABLED", None)], || {
                reconcile_guest_registry_arm_enabled()
            }),
            "the guest-registry arm must default OFF — enabling is a coordinated action"
        );
        assert!(with_env_vars(
            &[("RECONCILE_GUEST_REGISTRY_ARM_ENABLED", Some("true"))],
            || { reconcile_guest_registry_arm_enabled() }
        ));
        for sloppy in ["TRUE", "1", "yes", " true", "True"] {
            assert!(
                !with_env_vars(
                    &[("RECONCILE_GUEST_REGISTRY_ARM_ENABLED", Some(sloppy))],
                    || { reconcile_guest_registry_arm_enabled() }
                ),
                "`{sloppy}` must NOT enable the arm"
            );
        }
    }

    /// The arm ships detection-only. Wiring the folio into a self-heal arm
    /// is a bigger step than for the flat entities: repairing a folio means
    /// DELETING canonical companion rows legacy no longer has, i.e. a sweep
    /// destroying TM.30 registry state.
    #[test]
    fn guest_registry_is_not_wired_into_either_self_heal_arm() {
        assert!(!FORCE_CONVERGE_VALUE_DRIFT_TABLES.contains(&"guest_registry"));
        assert!(!REINGEST_MISSING_PG_TABLES.contains(&"guest_registry"));
        assert!(!force_converge_value_drift_eligible(
            "guest_registry",
            Some(LEGACY_HASH),
            Some(PG_HASH),
            OLD_ENOUGH_SECS,
            true,
        ));
        assert!(!reingest_missing_pg_eligible(
            "guest_registry",
            Some(LEGACY_HASH),
            None,
            OLD_ENOUGH_SECS,
            true,
        ));
    }

    /// A companion folio hangs off its check-in, so the sweep must heal
    /// check-ins BEFORE it (the CT mapper ERRORS on an unresolvable parent).
    #[test]
    fn guest_registry_ranks_after_its_parent_checkin() {
        assert!(
            reconcile_table_fk_rank("checkins") < reconcile_table_fk_rank("guest_registry")
        );
        assert!(
            reconcile_table_fk_rank("guest_registry") < reconcile_table_fk_rank("something_new"),
            "the wildcard must stay strictly after every ranked entity"
        );
    }

    /// The whole point of the folio unit: iHOTEL's DELETE+reinsert edit
    /// churns ids, so a per-row arm would false-positive on every edit.
    /// Hashing the folio must be invariant under that churn — the hash is
    /// built from names + countries only.
    #[test]
    fn folio_hash_is_invariant_under_legacy_id_churn() {
        // Same companion content, re-saved in iHOTEL (new IDENTITY, new
        // canonical guest_id): nothing in the hash body can express an id.
        let before = test_folio("CH26-005228", &[("Somchai Jaidee", Some("TH"))]);
        let after_reinsert = test_folio("CH26-005228", &[("Somchai Jaidee", Some("TH"))]);
        assert_eq!(
            guest_registry_canonical_hash(&before),
            guest_registry_canonical_hash(&after_reinsert),
        );
        // …and a genuine content edit DOES move it.
        let edited = test_folio("CH26-005228", &[("Somchai Jaidee-Suk", Some("TH"))]);
        assert_ne!(
            guest_registry_canonical_hash(&before),
            guest_registry_canonical_hash(&edited),
        );
    }

    /// A folio is one row on each side by construction, so the arm records
    /// `value` drift — never `cardinality` (never silenced, never closed)
    /// and never `missing_pg` for the ordinary "canonical hasn't ingested
    /// the new companion yet" case, which WOULD be a permanently open row.
    #[test]
    fn folio_divergence_always_classifies_as_value_drift() {
        let legacy = test_folio("CH26-005228", &[("Somchai", None)]);
        let canonical = RegistryFolioProjection::empty("CH26-005228");
        let mssql_hash = guest_registry_canonical_hash(&legacy);
        let pg_hash = guest_registry_canonical_hash(&canonical);
        assert_ne!(mssql_hash, pg_hash);
        let kind = classify_divergence(Some(&pg_hash), Some(&mssql_hash), 1, 1);
        assert_eq!(kind, DivergenceKind::Value);
        assert!(
            kind.is_silenceable(),
            "folio drift must be ackable — the arm re-compares it on every \
             tick regardless, and the log row stays unresolved either way"
        );
    }

    /// The ack cache suppresses WRITES; it must never gate detection. If it
    /// did, a canonical-side change that leaves the legacy hash untouched
    /// (a companion deleted from `ht_guest_registry`, a dropped CT delete)
    /// would be invisible forever — the payments arm needed an explicit
    /// carve-out for exactly that shape, and this arm avoids needing one by
    /// keeping both sides in memory.
    #[test]
    fn ack_is_written_only_when_the_legacy_hash_moves() {
        let hash = guest_registry_canonical_hash(&test_folio("CH1", &[("A", None)]));
        assert!(
            guest_registry_ack_needs_write(None, &hash),
            "an unseen folio must be acked"
        );
        assert!(
            !guest_registry_ack_needs_write(Some(&hash), &hash),
            "a stable folio must not re-write its ack row every tick"
        );
        let moved = guest_registry_canonical_hash(&test_folio("CH1", &[("B", None)]));
        assert!(guest_registry_ack_needs_write(Some(&hash), &moved));
    }

    /// SQL-shape pins for the canonical side. The name expression MUST be
    /// the mapper's own (single-sourced), the primary-guest filter MUST be
    /// present and NULL-safe, and the era floor MUST be canonical-derived
    /// with no timezone shift and a day truncation.
    #[test]
    fn canonical_registry_sql_shapes_are_pinned() {
        let folios = canonical_registry_folios_sql();
        assert!(
            folios.contains(CANONICAL_COMPANION_NAME_SQL),
            "the canonical projection must reuse the mapper's name expression \
             verbatim, else an app-created companion hashes differently on the \
             two sides forever: {folios}"
        );
        assert!(
            folios.contains(CANONICAL_COMPANION_PRIMARY_FILTER),
            "a registered PRIMARY guest is not a companion: {folios}"
        );
        assert_eq!(
            CANONICAL_COMPANION_PRIMARY_FILTER,
            "COALESCE(guest_is_primary, false) = false",
            "the column is nullable; a bare `= false` drops NULL rows out of \
             the canonical folio and reports them as legacy-only forever"
        );
        assert!(
            folios.contains("JOIN ht_checkins ON ht_checkins.cin_id = ht_guest_registry.guest_cin_id"),
            "join companion → check-in, never the reverse: a duplicate \
             legacy_cin_no would otherwise duplicate companion lines: {folios}"
        );
        assert!(folios.contains("ht_checkins.cin_checkin_time >= $1"), "{folios}");

        let floor = guest_registry_era_floor_sql();
        assert!(floor.contains("MIN(ht_checkins.cin_checkin_time)"), "{floor}");
        assert!(floor.contains("FROM ht_guest_registry"), "{floor}");
        assert!(
            floor.contains("date_trunc('day'"),
            "widen to the start of the day so the mirror's first day is fully \
             covered: {floor}"
        );
        assert!(
            !floor.contains("INTERVAL"),
            "the floor is derived from and compared against the SAME canonical \
             column — any offset here narrows it and silently drops in-era \
             folios: {floor}"
        );
        assert!(
            floor.contains(CANONICAL_COMPANION_PRIMARY_FILTER),
            "coverage is measured over companions, not registered primaries: {floor}"
        );
        assert!(
            floor.contains("ht_guest_registry.guest_legacy_id IS NOT NULL"),
            "\"mirrored\" means STAMPED BY THE CT MAPPER. App-authored companions \
             (POST /api/checkins/{{id}}/guests, the migration-070 registration \
             capture) have no legacy counterpart at all while \
             TM30_COMPANION_WRITEBACK_ENABLED is false, so counting them as \
             coverage claims an era the mirror never covered: {floor}"
        );
    }

    /// The floor is a low-water mark on the PARENT's check-in time, so ONE
    /// pre-era folio gaining a mirrored companion (iHOTEL's DELETE+REINSERT
    /// edit on a 2023 folio — the mapper resolves the parent by
    /// `legacy_cin_no` with no era restriction) would drag it back years and
    /// admit ~all 20,423 legacy folios instead of 830. The persisted
    /// watermark is what makes scope monotonically NARROWING.
    #[test]
    fn era_floor_is_clamped_to_a_non_decreasing_watermark() {
        let old = chrono::NaiveDate::from_ymd_opt(2023, 4, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let era = chrono::NaiveDate::from_ymd_opt(2026, 5, 13)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        assert_eq!(
            clamped_era_floor(Some(era), Some(old)),
            Some(era),
            "a derived floor BELOW the watermark is the one-old-companion drag; \
             it must not widen the scan"
        );
        assert_eq!(
            clamped_era_floor(Some(old), Some(era)),
            Some(era),
            "a derived floor ABOVE the watermark is genuine narrowing and wins"
        );
        assert_eq!(
            clamped_era_floor(None, Some(era)),
            Some(era),
            "first tick: the derived value seeds the watermark"
        );
        assert_eq!(
            clamped_era_floor(Some(era), None),
            Some(era),
            "every mirrored companion vanishing must NOT reopen the whole history \
             — hold the watermark and let the divergence cap page"
        );
        assert_eq!(
            clamped_era_floor(None, None),
            None,
            "no coverage was ever established ⇒ the arm skips the legacy scan"
        );
    }

    /// The clamp has to hold across PROCESSES (the backend scheduler and
    /// `bin/sync` can tick the same database), so `GREATEST` lives in SQL
    /// and the read is the same round trip as the write. A hand-edited
    /// floor moved FORWARD is the documented remedy for a bad bootstrap
    /// reading, and `GREATEST` is what makes that edit stick.
    #[test]
    fn era_floor_watermark_sql_is_monotonic_and_single_round_trip() {
        assert!(
            RECONCILE_ERA_FLOOR_UPSERT_SQL.contains(
                "GREATEST(ht_reconcile_era_floor.era_floor, EXCLUDED.era_floor)"
            ),
            "without GREATEST the upsert would happily write a LOWER floor: \
             {RECONCILE_ERA_FLOOR_UPSERT_SQL}"
        );
        assert!(
            RECONCILE_ERA_FLOOR_UPSERT_SQL.ends_with("RETURNING era_floor"),
            "the post-clamp value must come back from the same statement, else a \
             concurrent tick's value is silently ignored: \
             {RECONCILE_ERA_FLOOR_UPSERT_SQL}"
        );
        assert!(RECONCILE_ERA_FLOOR_SELECT_SQL.contains("FROM ht_reconcile_era_floor"));
        assert_eq!(
            GUEST_REGISTRY_ERA_FLOOR_KEY, "guest_registry",
            "same literal as ht_reconcile_log.table_name / sync_status.entity_type, \
             so one operator query joins all three"
        );
    }

    /// The circuit breaker. Anything past the cap is a scope bug or a dead
    /// mirror, not a backlog: the tick must abort whole, never truncate.
    #[test]
    fn divergence_cap_trips_only_strictly_above_the_cap() {
        assert!(!divergence_cap_exceeded(0, 500));
        assert!(!divergence_cap_exceeded(499, 500));
        assert!(
            !divergence_cap_exceeded(500, 500),
            "the cap is a ceiling the tick may reach, not one it may not touch"
        );
        assert!(divergence_cap_exceeded(501, 500));
        // The flood this exists for: floor dragged to 2023 at HF Hotel.
        assert!(divergence_cap_exceeded(19_600, 500));
    }

    /// The default is not a taste call: enqueuing more findings per tick
    /// than `auto_resolve_reconcile_log` can even LOOK at per tick is the
    /// mechanism behind every flood incident here — the backlog never
    /// drains and the age-ordered batch starves every other entity. 500 is
    /// that sweep's own `LIMIT`; if it ever moves, move this with it.
    #[test]
    fn divergence_cap_default_matches_the_auto_resolve_batch() {
        assert_eq!(GUEST_REGISTRY_DIVERGENCE_CAP_DEFAULT, 500);
        // …and it must dwarf the steady state: 830 in-era folios at HF
        // Hotel with ~12 findings, 574 with ~29 at Ville (live 2026-07-28).
        assert!(GUEST_REGISTRY_DIVERGENCE_CAP_DEFAULT > 29 * 10);
    }

    /// Resolution chain + strict positivity: a zero or negative cap would
    /// make `divergence_cap_exceeded` trip on a perfectly healthy tick and
    /// wedge the arm shut.
    #[test]
    fn divergence_cap_resolves_per_site_and_stays_positive() {
        assert!(guest_registry_divergence_cap("hfhotel") > 0);
        assert_eq!(
            guest_registry_divergence_cap("hfville"),
            GUEST_REGISTRY_DIVERGENCE_CAP_DEFAULT,
            "unset ⇒ the compiled-in default on every site"
        );
    }

    /// A cap breach is announced under its own namespaced key, so the
    /// sync-lag all-clear can never mistake it for the table
    /// `guest_registry`, delete its cooldown and un-throttle the page to
    /// once per tick.
    #[test]
    fn reconcile_cap_cooldown_key_is_namespaced_and_unique() {
        let key = reconcile_cap_cooldown_key("guest_registry");
        assert_eq!(key, "reconcile_cap:guest_registry");
        assert!(!is_reconcile_table_key(&key));
        for other in [
            "ct_retention_overflow:",
            "escalated:",
            "burst:",
            "ct_watcher_lag:",
            "shadow_mode:",
            "boot_refusal:",
        ] {
            assert!(
                !key.starts_with(other) && !other.starts_with("reconcile_cap:"),
                "the reconcile_cap family must not prefix-collide with {other}"
            );
        }
    }

    /// The scope gate. Pre-coverage folios and folios with no canonical
    /// parent are BOTH out of scope: the first can never converge (no
    /// historical backfill of `ht_guest_registry`), the second is a
    /// check-ins problem `sync_checkins` already reports.
    #[test]
    fn in_era_key_set_sql_is_the_scope_gate() {
        assert!(IN_ERA_CHECKIN_KEYS_SQL.contains("FROM ht_checkins"));
        assert!(IN_ERA_CHECKIN_KEYS_SQL.contains("legacy_cin_no IS NOT NULL"));
        assert!(IN_ERA_CHECKIN_KEYS_SQL.contains("cin_checkin_time >= $1"));
        // The per-PK canonical probe resolves its parent the same way the
        // CT mapper does, so the sweep lands on the same folio.
        assert_eq!(
            CANONICAL_CHECKIN_ID_PROBE_SQL,
            "SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1 LIMIT 1"
        );
    }

    /// Track J1 — projection lock. The IDENTITY `id` must stay OUT (it is
    /// the very thing the folio unit exists to ignore), the iHOTEL typo
    /// `Cin_contry` stays verbatim, and `Cin_no` is the lowercase-n variant.
    #[test]
    fn guest_registry_reconcile_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(
            GUEST_REGISTRY_RECONCILE_PROJECTION,
            "HT_CheckIn_Other_People"
        );
        assert!(
            !GUEST_REGISTRY_RECONCILE_PROJECTION.contains(&"id"),
            "hashing the legacy IDENTITY re-creates the DELETE+reinsert false \
             positive the folio unit exists to remove"
        );
        assert!(GUEST_REGISTRY_RECONCILE_PROJECTION.contains(&"Cin_contry"));
        assert!(!GUEST_REGISTRY_RECONCILE_PROJECTION.contains(&"Cin_country"));
        assert!(!GUEST_REGISTRY_RECONCILE_PROJECTION.contains(&"Cin_No"));
    }

    // -------------------------------------------------------------------
    // Force-converge outcome classification — the gate ⊂ hash tripwire
    // (2026-07-28). Pure, no DB.
    // -------------------------------------------------------------------

    /// The amplifier this fix exists for: the mapper's idempotency gate
    /// decided "already identical" (`Ok(None)`), canonical did not move, and
    /// the row is STILL unconverged. Pre-fix this was reported to the sweep
    /// as a successful repair (`Ok(true)`), so every tick logged "repaired"
    /// and then "still not converged", forever, while the underlying
    /// divergence stayed invisible to both detection AND self-heal.
    #[test]
    fn gate_skip_flagged_when_mapper_noop_and_hash_static() {
        assert!(classify_force_converge(
            ForceConvergeOutcome::MapperNoop,
            false, // canonical hash unchanged across the apply
            false, // and the row is still unconverged
        ));

        // Converged is the dominant condition: if the row DID converge (a
        // concurrent CT event landed between the probe and the apply), the
        // sweep closes it and there is nothing to warn about.
        assert!(
            !classify_force_converge(ForceConvergeOutcome::MapperNoop, false, true),
            "a converged row is a resolved row, never a tripwire"
        );

        // The non-mapper outcomes already have their own log lines
        // (`legacy row absent` / unsupported table) and must never be
        // reported as gate skips.
        for outcome in [
            ForceConvergeOutcome::SourceRowAbsent,
            ForceConvergeOutcome::UnsupportedTable,
        ] {
            assert!(
                !classify_force_converge(outcome, false, false),
                "{outcome:?} is not a mapper gate skip"
            );
            assert!(
                !outcome.mapper_ran(),
                "{outcome:?} must skip the convergence re-test, as the old Ok(false) did"
            );
        }
    }

    /// A mapper that DID write (`Ok(Some(event))`) but left the row
    /// unconverged is a different class — the two projections genuinely
    /// disagree — and keeps the pre-existing "leaving row open for operator
    /// review" warn. It must not be mislabelled as a gate skip, whether or
    /// not the canonical hash moved.
    #[test]
    fn wrote_but_unconverged_stays_open_without_gate_skip_flag() {
        for pg_hash_moved in [true, false] {
            assert!(
                !classify_force_converge(ForceConvergeOutcome::Wrote, pg_hash_moved, false),
                "a real write is never a gate skip (pg_hash_moved={pg_hash_moved})"
            );
        }
        // …and it still takes the convergence re-test path, so a successful
        // re-ingest continues to close its ledger row.
        assert!(ForceConvergeOutcome::Wrote.mapper_ran());
        assert!(ForceConvergeOutcome::MapperNoop.mapper_ran());
    }

    /// The subtlety that makes a bare "`Ok(None)` ⇒ gate-skipped" test
    /// wrong: several write paths legitimately return `Ok(None)` — the room
    /// mapper always UPSERTs but only emits an event on a `room_clean` flip,
    /// and the cancel / soft-delete paths write without an event. Those DO
    /// move the canonical hash, which is exactly how they are told apart
    /// from a gate skip.
    #[test]
    fn noop_with_hash_movement_is_not_a_gate_skip() {
        assert!(
            !classify_force_converge(ForceConvergeOutcome::MapperNoop, true, false),
            "an event-less write that moved canonical is a legitimate repair, \
             not a gate skip — it just hasn't converged yet"
        );
        assert!(!classify_force_converge(
            ForceConvergeOutcome::MapperNoop,
            true,
            true
        ));
        // `from_mapper_event` is the single place `Ok(Some)`/`Ok(None)` is
        // interpreted; pin the `None` half here (the `Some` half needs a
        // real `DomainEvent`, which the mapper tests already cover).
        assert_eq!(
            ForceConvergeOutcome::from_mapper_event(None),
            ForceConvergeOutcome::MapperNoop
        );
    }

    // -------------------------------------------------------------------
    // Auto-resolve sweep candidate ordering — the FK guarantee.
    // -------------------------------------------------------------------

    fn candidate(id: i64, table: &str, pk: &str, age_secs: f64) -> ReconcileCandidate {
        (id, table.to_string(), pk.to_string(), None, age_secs)
    }

    #[test]
    fn reconcile_fk_rank_orders_parents_before_dependents() {
        assert!(reconcile_table_fk_rank("customers") < reconcile_table_fk_rank("bookings"));
        assert!(reconcile_table_fk_rank("rooms") < reconcile_table_fk_rank("bookings"));
        assert!(reconcile_table_fk_rank("bookings") < reconcile_table_fk_rank("checkins"));
        assert!(reconcile_table_fk_rank("checkins") < reconcile_table_fk_rank("something_new"));
    }

    /// Phase 6-C. Every mirror probe key must be RESOLVABLE — listed here
    /// AND dispatched by both `compute_current_*_hash` — or its rows sit
    /// open forever and, being selected by age alone, eventually own the
    /// sweep's whole 500-row batch (the 2026-05-18 `rooms` failure mode).
    #[test]
    fn resolvable_tables_lists_every_mirror_probe_key() {
        for key in crate::scheduler::mirror_probe::mirror_probe_keys() {
            assert!(
                RECONCILE_RESOLVABLE_TABLES.contains(&key),
                "mirror probe `{key}` is not in RECONCILE_RESOLVABLE_TABLES"
            );
        }
    }

    /// A probe is ranked LAST but still ranked: falling through to the
    /// wildcard would fire the `debug_assert!` for a listed-but-unranked
    /// table, which is a test failure, not a silent demotion.
    #[test]
    fn reconcile_fk_rank_ranks_mirror_probes_after_every_entity() {
        for key in crate::scheduler::mirror_probe::mirror_probe_keys() {
            assert!(
                reconcile_table_fk_rank("guest_registry") < reconcile_table_fk_rank(key),
                "{key} must sort after every healable entity"
            );
            assert!(
                reconcile_table_fk_rank(key) < reconcile_table_fk_rank("something_new"),
                "{key} must still be ranked ahead of the unranked wildcard"
            );
        }
    }

    /// The probe writes nothing but `ht_reconcile_log`: no probe key may
    /// appear in either self-heal list, or the sweep would start re-driving
    /// opaque mirror rows (plan 6-D, a separate coordinated decision).
    #[test]
    fn mirror_probes_are_in_neither_self_heal_list() {
        for key in crate::scheduler::mirror_probe::mirror_probe_keys() {
            assert!(!FORCE_CONVERGE_VALUE_DRIFT_TABLES.contains(&key));
            assert!(!REINGEST_MISSING_PG_TABLES.contains(&key));
            assert!(!force_converge_value_drift_eligible(
                key,
                Some("a"),
                Some("b"),
                f64::MAX,
                true
            ));
            assert!(!reingest_missing_pg_eligible(
                key,
                Some("a"),
                None,
                f64::MAX,
                true
            ));
        }
    }

    // =====================================================================
    // Issue #273 — calendar closure arm (business-key resolve)
    // =====================================================================

    /// The key literal this module dispatches on must still be the one the
    /// probe registry uses. A rename on either side would silently route
    /// calendar rows back to the never-equal id-keyed arm.
    #[test]
    fn room_calendar_probe_key_matches_the_registered_mirror_probe() {
        let probe = crate::scheduler::mirror_probe::probe_for_table(ROOM_CALENDAR_PROBE_KEY)
            .expect("the calendar probe must still be registered under this key");
        assert_eq!(probe.mirror_table, "ht_room_calendar");
        assert_eq!(probe.legacy_table, "HT_Room_Status");
        assert!(
            RECONCILE_RESOLVABLE_TABLES.contains(&ROOM_CALENDAR_PROBE_KEY),
            "a dispatched-but-unlisted probe key is invisible to the \
             resolvable-list guards"
        );
    }

    /// The closure arm introduces NO new `ht_reconcile_log.table_name` and
    /// no new hashed entity: it RE-KEYS the existing probe's resolve. So
    /// there is nothing to register with `gate_guard` (whose contract binds
    /// CT-mapper idempotency gates to reconcile hashes — no probe has one)
    /// and `RECONCILE_RESOLVABLE_TABLES` needs no new entry. This pins that
    /// the mirror population did not grow.
    #[test]
    fn calendar_closure_arm_adds_no_new_resolvable_table() {
        let listed = RECONCILE_RESOLVABLE_TABLES
            .iter()
            .filter(|t| t.starts_with("mirror_"))
            .count();
        assert_eq!(
            listed,
            crate::scheduler::mirror_probe::mirror_probe_keys().len(),
            "the closure arm must re-key an EXISTING probe, not register a \
             new reconcile entity"
        );
    }

    /// Converged: both sides agree on the in-era night set, so the two
    /// recomputed hashes are equal and `should_auto_resolve` CLOSES the row.
    /// This is the property the id-keyed arm could never have.
    #[test]
    fn room_calendar_business_key_row_closes_when_both_sides_agree() {
        let legacy = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        let pg = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        assert_eq!(legacy, pg);
        assert!(should_auto_resolve(
            ROOM_CALENDAR_PROBE_KEY,
            Some(&legacy),
            Some(&pg),
            None
        ));
    }

    /// Still divergent: the LIVE deficit (1546 legacy nights vs 1420
    /// canonical at HF Hotel, 2026-07-28) must keep the row OPEN. The arm
    /// makes the gap closeable in principle — it does not pretend it is
    /// closed today.
    #[test]
    fn room_calendar_business_key_row_stays_open_while_the_night_deficit_survives() {
        let legacy = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        let pg = room_calendar_business_key_hash(1420, Some("2025-11-02"), Some("2026-08-14"));
        assert_ne!(legacy, pg);
        assert!(!should_auto_resolve(
            ROOM_CALENDAR_PROBE_KEY,
            Some(&legacy),
            Some(&pg),
            None
        ));
    }

    /// Equal counts with different coverage boundaries is still a
    /// divergence — a night shifted off one end of the window is exactly
    /// the shape a count-only comparison would miss.
    #[test]
    fn room_calendar_business_key_hash_covers_both_coverage_boundaries() {
        let base = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        assert_ne!(
            base,
            room_calendar_business_key_hash(1546, Some("2025-11-03"), Some("2026-08-14"))
        );
        assert_ne!(
            base,
            room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-15"))
        );
    }

    /// ABSENT-SIDE semantics: an empty calendar on BOTH sides is a real
    /// converged state, so it must hash to something equal and non-empty and
    /// close the row — the same contract as `mirror_absent_hash`. Returning
    /// "no hash" here would leave such a row open forever.
    #[test]
    fn room_calendar_business_key_absent_on_both_sides_converges() {
        let empty = room_calendar_business_key_hash(0, None, None);
        assert!(!empty.is_empty());
        assert!(should_auto_resolve(
            ROOM_CALENDAR_PROBE_KEY,
            Some(&empty),
            Some(&empty),
            None
        ));
        // Absent on ONE side only is NOT converged.
        let populated =
            room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        assert_ne!(empty, populated);
        assert!(!should_auto_resolve(
            ROOM_CALENDAR_PROBE_KEY,
            Some(&populated),
            Some(&empty),
            None
        ));
    }

    /// The business-key hash must never collide with the id-keyed aggregate
    /// hash the generic probe computes for the same key and the same counts.
    /// They measure different things; a collision would let a mixed-binary
    /// fleet close a row on the wrong comparison.
    #[test]
    fn room_calendar_business_key_hash_is_distinct_from_the_id_keyed_aggregate() {
        assert_ne!(
            room_calendar_business_key_hash(1546, None, None),
            crate::scheduler::mirror_probe::mirror_aggregate_hash(
                ROOM_CALENDAR_PROBE_KEY,
                1546,
                None,
                None
            )
        );
    }

    /// The era floor is DERIVED from the mirror (`MIN(rcal_date)`), never
    /// configured and never a rolling window — the `PAYMENTS_ERA_FLOOR_SQL`
    /// lesson. And the canonical side must NOT filter on `rcal_legacy_id`:
    /// that filter is precisely the structural undercount the business key
    /// exists to dodge.
    #[test]
    fn room_calendar_era_floor_is_derived_from_the_mirror() {
        assert!(ROOM_CALENDAR_BUSINESS_KEY_PG_SQL.contains("MIN(rcal_date)"));
        assert!(ROOM_CALENDAR_BUSINESS_KEY_PG_SQL.contains("MAX(rcal_date)"));
        assert!(ROOM_CALENDAR_BUSINESS_KEY_PG_SQL.contains("FROM ht_room_calendar"));
        assert!(
            !ROOM_CALENDAR_BUSINESS_KEY_PG_SQL.contains("INTERVAL"),
            "the floor must be the mirror's own coverage boundary, not a \
             rolling window"
        );
        assert!(
            !ROOM_CALENDAR_BUSINESS_KEY_PG_SQL.contains("rcal_legacy_id"),
            "filtering on the legacy id re-introduces the structural \
             undercount the business key exists to dodge"
        );
    }

    /// The legacy side must count DISTINCT `(room_no, night)` pairs:
    /// canonical is UNIQUE on `(rcal_room_id, rcal_date)` while
    /// `HT_Room_Status` is not, so a raw `COUNT(*)` would report the legacy
    /// allocator's duplicates as a permanent deficit. The floor is pushed
    /// into the scan as a BOUND parameter, and an empty mirror scans the
    /// whole table (that IS the finding).
    #[test]
    fn room_calendar_legacy_sql_counts_distinct_nights_and_pushes_the_floor() {
        let floored = room_calendar_business_key_legacy_sql(true);
        assert!(floored.contains("SELECT DISTINCT room_no, CAST(room_date AS DATE) AS night_date"));
        assert!(floored.contains("COUNT_BIG(*)"));
        assert!(floored.contains("CAST(room_date AS DATE) >= CAST(@P1 AS DATE)"));
        assert!(
            floored.contains("CONVERT(varchar(10), MIN(night_date), 23)"),
            "boundaries must come back as ISO text so both sides hash \
             byte-identical YYYY-MM-DD"
        );
        assert!(
            !floored.contains("N'"),
            "no `N'…'` literals against the legacy DB — TIS-620 corruption"
        );

        let unfloored = room_calendar_business_key_legacy_sql(false);
        assert!(
            !unfloored.contains("@P1"),
            "an empty mirror binds no floor — it scans the whole table on \
             purpose"
        );
        assert!(unfloored.contains("SELECT DISTINCT room_no"));
    }

    /// ORDER IS LOAD-BEARING. The calendar arm must precede the generic
    /// `probe_for_table` arm in BOTH resolve dispatches — Rust match arms
    /// are tried top-down, so a calendar row would otherwise be swallowed by
    /// the generic id-keyed arm and could never converge. It must also be
    /// scoped to the `<aggregate>` PK so a per-PK row still falls through.
    #[test]
    fn room_calendar_arm_precedes_the_generic_probe_arm_in_both_dispatches() {
        let src = scheduler_source_before_tests();
        for func in [
            "async fn compute_current_pg_hash(",
            "async fn compute_current_legacy_hash(",
        ] {
            let start = src.find(func).unwrap_or_else(|| panic!("{func} must exist"));
            let rest = &src[start..];
            let body = &rest[..rest.find("\n}\n").map(|i| i + 3).unwrap_or(rest.len())];

            let calendar_at = body
                .find("ROOM_CALENDAR_PROBE_KEY")
                .unwrap_or_else(|| panic!("{func} must dispatch the calendar closure arm"));
            let generic_at = body
                .find("mirror_probe::probe_for_table(t).is_some()")
                .unwrap_or_else(|| panic!("{func} must still dispatch the generic probe arm"));
            assert!(
                calendar_at < generic_at,
                "{func}: the calendar business-key arm must come BEFORE the \
                 generic probe arm, or the never-equal id-keyed aggregate \
                 wins and the row can never close"
            );
            assert!(
                body[calendar_at..generic_at].contains("MIRROR_AGGREGATE_PK"),
                "{func}: the calendar arm must be scoped to the `<aggregate>` \
                 PK so a per-PK row still falls through to the generic arm"
            );
        }
    }

    // =====================================================================
    // Issue #273 (remainder) — calendar DETECTION re-keyed to the business
    // key, `observe_only` flipped
    // =====================================================================

    /// THE property that eliminates the record/resolve churn loop: for the
    /// SAME pair of hashes, detection's "converged?" question
    /// (`room_calendar_business_key_divergence` returning `None`) and
    /// `should_auto_resolve`'s primary-convergence arm must agree, because
    /// both are now `legacy_hash == pg_hash` and nothing else. Before this
    /// change detection asked a DIFFERENT question (the id-keyed aggregate),
    /// so the two could disagree — resolve closing a row detection would
    /// immediately re-open. If this test ever fails, that hazard is back.
    #[test]
    fn detection_and_resolution_agree_on_convergence_for_every_hash_pair() {
        let cases: &[(i64, Option<&str>, Option<&str>, i64, Option<&str>, Option<&str>)] = &[
            // Converged: identical counts and boundaries.
            (
                1420,
                Some("2025-11-02"),
                Some("2026-08-14"),
                1420,
                Some("2025-11-02"),
                Some("2026-08-14"),
            ),
            // Diverged: the live HF Hotel deficit (2026-07-28).
            (
                1546,
                Some("2025-11-02"),
                Some("2026-08-14"),
                1420,
                Some("2025-11-02"),
                Some("2026-08-14"),
            ),
            // Converged: absent on both sides (an empty calendar).
            (0, None, None, 0, None, None),
            // Diverged: same count, boundary shifted by a day.
            (
                1420,
                Some("2025-11-02"),
                Some("2026-08-14"),
                1420,
                Some("2025-11-03"),
                Some("2026-08-14"),
            ),
        ];

        for (legacy_count, legacy_min, legacy_max, pg_count, pg_min, pg_max) in cases.iter().copied()
        {
            let legacy_hash = room_calendar_business_key_hash(legacy_count, legacy_min, legacy_max);
            let pg_hash = room_calendar_business_key_hash(pg_count, pg_min, pg_max);

            let resolution_says_converged = should_auto_resolve(
                ROOM_CALENDAR_PROBE_KEY,
                Some(&legacy_hash),
                Some(&pg_hash),
                None,
            );
            let detection_says_converged = room_calendar_business_key_divergence(
                &legacy_hash,
                &pg_hash,
                legacy_count,
                pg_count,
            )
            .is_none();

            assert_eq!(
                resolution_says_converged, detection_says_converged,
                "detection and resolution disagree for legacy=({legacy_count}, {legacy_min:?}, \
                 {legacy_max:?}) pg=({pg_count}, {pg_min:?}, {pg_max:?}) — this IS the \
                 churn-loop hazard issue #273 (remainder) exists to close"
            );
        }
    }

    /// Business-key detection correctness: converges only when the hashes
    /// agree, regardless of the raw counts passed alongside them (the counts
    /// are only consulted once a divergence is already known to exist, to
    /// pick a direction).
    #[test]
    fn room_calendar_business_key_divergence_converges_when_hashes_agree() {
        let h = room_calendar_business_key_hash(1420, Some("2025-11-02"), Some("2026-08-14"));
        assert_eq!(
            room_calendar_business_key_divergence(&h, &h, 1420, 1420),
            None
        );
    }

    /// Business-key detection correctness: direction follows which side has
    /// more nights, exactly like `mirror_probe::aggregate_divergence_kind`,
    /// and a count-equal-but-boundary-shifted pair reads as `Value` — never
    /// `Cardinality` (the hourly drift digest excludes it).
    #[test]
    fn room_calendar_business_key_divergence_classifies_by_direction() {
        let legacy = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        let pg = room_calendar_business_key_hash(1420, Some("2025-11-02"), Some("2026-08-14"));

        assert_eq!(
            room_calendar_business_key_divergence(&legacy, &pg, 1546, 1420),
            Some(DivergenceKind::MissingPg)
        );
        assert_eq!(
            room_calendar_business_key_divergence(&pg, &legacy, 1420, 1546),
            Some(DivergenceKind::MissingMssql)
        );

        let shifted = room_calendar_business_key_hash(1420, Some("2025-11-03"), Some("2026-08-14"));
        assert_eq!(
            room_calendar_business_key_divergence(&pg, &shifted, 1420, 1420),
            Some(DivergenceKind::Value)
        );

        for (l_hash, p_hash, lc, pc) in [(&legacy, &pg, 1546, 1420), (&pg, &legacy, 1420, 1546)] {
            assert_ne!(
                room_calendar_business_key_divergence(l_hash, p_hash, lc, pc),
                Some(DivergenceKind::Cardinality),
                "a probe must never emit `cardinality` — the hourly drift digest \
                 excludes it and it would go unpaged"
            );
        }
    }

    /// "Recording resumes when counts change again": the divergence check
    /// has no memory between calls — a converged pair sandwiched between two
    /// DIFFERENT diverged pairs reports exactly what each call's own inputs
    /// say, never something left over from the call before. This is what
    /// makes a recorded row's eventual convergence (a re-drive, or the gap
    /// closing) followed by a LATER new gap behave as "detect it again", not
    /// "stay silent because this table already had a row once".
    #[test]
    fn room_calendar_business_key_divergence_recomputes_fresh_every_call() {
        let legacy = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        let pg = room_calendar_business_key_hash(1420, Some("2025-11-02"), Some("2026-08-14"));
        assert!(room_calendar_business_key_divergence(&legacy, &pg, 1546, 1420).is_some());

        let converged = room_calendar_business_key_hash(1546, Some("2025-11-02"), Some("2026-08-14"));
        assert!(
            room_calendar_business_key_divergence(&converged, &converged, 1546, 1546).is_none()
        );

        // A brand-new gap, evaluated right after a converged call — nothing
        // about the converged call above leaks into this one.
        let legacy2 = room_calendar_business_key_hash(1550, Some("2025-11-02"), Some("2026-08-14"));
        assert_eq!(
            room_calendar_business_key_divergence(&legacy2, &converged, 1550, 1546),
            Some(DivergenceKind::MissingPg)
        );
    }

    /// "No re-mint of a resolved row when counts unchanged": what actually
    /// prevents `record_divergence` from inserting a second row for an
    /// UNCHANGED, still-open mismatch is that the stored `mssql_hash` /
    /// `pg_hash` are the STABLE sentinel
    /// (`mirror_probe::mirror_aggregate_sentinel`), never the live
    /// business-key hash — `record_divergence`'s `NOT EXISTS (…) AND
    /// mssql_hash IS NOT DISTINCT FROM $4` dedupe then matches the row
    /// already open from the previous tick and skips the insert. A live hash
    /// would differ from tick to tick as the legacy table merely grew and
    /// defeat that dedupe, minting a fresh row every tick for a table with a
    /// known backlog. Source-scanned because this is a property of what gets
    /// PASSED to `record_divergence`, not of any pure function's return
    /// value.
    #[test]
    fn calendar_detection_records_the_stable_sentinel_not_the_live_hash() {
        let src = scheduler_source_before_tests();
        let start = src
            .find("pub(crate) async fn probe_room_calendar_business_key(")
            .expect("detection fn must exist");
        let rest = &src[start..];
        let body = &rest[..rest.find("\n}\n").map(|i| i + 3).unwrap_or(rest.len())];

        assert!(
            body.contains("mirror_aggregate_sentinel(ROOM_CALENDAR_PROBE_KEY)"),
            "detection must record the STABLE sentinel, not `legacy_hash`/`pg_hash` \
             directly, or a merely-unchanged divergence mints a fresh row every tick"
        );
        let record_at = body
            .find("record_divergence(")
            .expect("detection must call record_divergence");
        let call = &body[record_at..(record_at + 400).min(body.len())];
        // Positive check: the two hash arguments must be the
        // sentinel-derived locals…
        assert!(
            call.contains("pg_row_hash.as_deref()") && call.contains("mssql_hash.as_deref()"),
            "record_divergence must be called with the sentinel-derived \
             `pg_row_hash` / `mssql_hash` locals: {call}"
        );
        // …and NOT the live comparison hashes (`legacy_hash` / `pg_hash`,
        // the `let`-bound results of `room_calendar_business_key_hash`
        // above) — a bare-word check, not a substring one, since
        // `pg_row_hash` and `mssql_hash` themselves must not trip it.
        let words: std::collections::HashSet<&str> = call
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .collect();
        assert!(
            !words.contains("legacy_hash") && !words.contains("pg_hash"),
            "record_divergence must not be called with the live comparison \
             hashes `legacy_hash` / `pg_hash` — a moving hash would mint a \
             fresh row every tick a diverged count merely changed: {call}"
        );
    }

    // =====================================================================
    // Issue #267 — scheduler-side event_name registry
    // =====================================================================

    /// Source of THIS module up to the test module. Scanning further would
    /// match the very literals these registry tests are built from (the
    /// `include_str!` self-reference trap, same as `scheduler::mirror`).
    fn scheduler_source_before_tests() -> &'static str {
        let full = include_str!("sync.rs");
        let cut = full
            .find("#[cfg(test)]")
            .expect("test module marker must exist");
        &full[..cut]
    }

    /// Shape lock. Names must be greppable from any locale, and the
    /// scheduler namespace must stay DISJOINT from the watcher's dotted
    /// `sync.…` taxonomy so a Loki filter of `^sync\.` still means "the CT
    /// watcher" and nothing else.
    #[test]
    fn known_scheduler_event_names_are_unique_and_greppable() {
        let unique: std::collections::HashSet<&str> =
            KNOWN_SCHEDULER_EVENT_NAMES.iter().copied().collect();
        assert_eq!(
            unique.len(),
            KNOWN_SCHEDULER_EVENT_NAMES.len(),
            "KNOWN_SCHEDULER_EVENT_NAMES has duplicate entries — each name \
             must appear exactly once"
        );
        assert!(
            !KNOWN_SCHEDULER_EVENT_NAMES.is_empty(),
            "registry emptied — a refactor deleted the array contents"
        );
        for name in KNOWN_SCHEDULER_EVENT_NAMES {
            assert!(!name.is_empty(), "empty event name in the registry");
            assert!(
                name.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "event name `{name}` must be lowercase snake_case ASCII"
            );
            assert!(
                !name.contains('.'),
                "event name `{name}` uses a dot — dots are reserved for the \
                 watcher's `sync.…` namespace in bin/sync.rs, and the two \
                 registries must stay disjoint"
            );
        }
    }

    /// THE LOCK (issue #267), half one: every `EV_…` constant declared in
    /// this module must be registered, and the registry must hold nothing
    /// else. Adding a new event constant without registering it fails here.
    #[test]
    fn every_scheduler_event_name_constant_is_registered() {
        let src = scheduler_source_before_tests();
        let mut declared: Vec<String> = Vec::new();
        for (i, _) in src.match_indices("const EV_") {
            let after = &src[i..];
            let eq = after
                .find(" = \"")
                .expect("an EV_ constant must bind a plain string literal");
            let value = &after[eq + 4..];
            let end = value.find('"').expect("unterminated EV_ literal");
            declared.push(value[..end].to_string());
        }
        assert!(
            !declared.is_empty(),
            "no EV_ constants found — the scan or the constants moved"
        );
        for name in &declared {
            assert!(
                KNOWN_SCHEDULER_EVENT_NAMES.contains(&name.as_str()),
                "event name `{name}` is declared but NOT in \
                 KNOWN_SCHEDULER_EVENT_NAMES — a grep-based consumer \
                 (/diagnose-alert, dashboards) would have no registry entry \
                 to check against"
            );
        }
        assert_eq!(
            declared.len(),
            KNOWN_SCHEDULER_EVENT_NAMES.len(),
            "registry size {} does not match the {} declared EV_ constants — \
             it holds a name nothing emits, or lost one that something does",
            KNOWN_SCHEDULER_EVENT_NAMES.len(),
            declared.len()
        );
    }

    /// THE LOCK, half two: no emission site may name its event with a raw
    /// string literal. Together with the half above this is what makes an
    /// UNREGISTERED addition fail — a literal trips this test, a new
    /// constant trips the other one.
    #[test]
    fn every_scheduler_event_emission_uses_a_registered_constant() {
        let src = scheduler_source_before_tests();
        let attr = "event_name";
        let all = src.matches(&format!("{attr} = ")).count();
        let via_const = src.matches(&format!("{attr} = EV_")).count();
        assert!(
            all >= 2,
            "expected ≥2 structured event emissions in this module; found \
             {all} — the region may have been refactored away"
        );
        assert_eq!(
            all, via_const,
            "{} emission site(s) name their event with a raw string literal \
             instead of a registered EV_ constant. A typo or rename there is \
             invisible to the registry and silently breaks the \
             /diagnose-alert grep contract (issue #267).",
            all - via_const
        );
    }

    /// Phase 6-D. The payment-ledger probe registers through the SAME three
    /// mechanisms 6-C chose — resolvable, ranked, dispatched — because a
    /// probe row that nothing can close sits open forever and, being
    /// selected by age alone, eventually owns the sweep's whole 500-row
    /// batch (the 2026-05-18 `rooms` failure mode). Note the wildcard
    /// `debug_assert!` would NOT have caught an omission here: it only fires
    /// for listed-but-undispatched names.
    #[test]
    fn payment_ledger_probe_is_resolvable_ranked_last_and_never_self_healed() {
        assert!(
            RECONCILE_RESOLVABLE_TABLES.contains(&PAYMENT_LEDGER_PROBE_KEY),
            "the payment-ledger probe is not in RECONCILE_RESOLVABLE_TABLES"
        );
        assert!(
            reconcile_table_fk_rank("guest_registry")
                < reconcile_table_fk_rank(PAYMENT_LEDGER_PROBE_KEY),
            "the probe must sort after every healable entity"
        );
        assert!(
            reconcile_table_fk_rank(PAYMENT_LEDGER_PROBE_KEY)
                < reconcile_table_fk_rank("something_new"),
            "the probe must still be ranked ahead of the unranked wildcard"
        );
        // DETECTION ONLY. Per-arm self-heal extensions come after this arm's
        // detection has soaked in production, never in the same change.
        assert!(!FORCE_CONVERGE_VALUE_DRIFT_TABLES.contains(&PAYMENT_LEDGER_PROBE_KEY));
        assert!(!REINGEST_MISSING_PG_TABLES.contains(&PAYMENT_LEDGER_PROBE_KEY));
        assert!(!force_converge_value_drift_eligible(
            PAYMENT_LEDGER_PROBE_KEY,
            Some("a"),
            Some("b"),
            f64::MAX,
            true
        ));
        assert!(!reingest_missing_pg_eligible(
            PAYMENT_LEDGER_PROBE_KEY,
            Some("a"),
            None,
            f64::MAX,
            true
        ));
    }

    /// The live FK shape: booking `R002066` references customer `C2413`.
    /// Every `customers` row must be healed before ANY `bookings` row in
    /// the same sweep pass, regardless of insertion order or age.
    #[test]
    fn reconcile_candidates_sort_customers_before_bookings() {
        let mut rows = vec![
            candidate(11, "bookings", "R002066|217", 1_400_000.0),
            candidate(12, "bookings", "R002066|110", 1_400_000.0),
            // The customer row was detected LATER (smaller age) — it must
            // still be processed first.
            candidate(13, "customers", "C2413", 1_000.0),
        ];
        sort_reconcile_candidates(&mut rows);
        let tables: Vec<&str> = rows.iter().map(|r| r.1.as_str()).collect();
        assert_eq!(
            tables,
            vec!["customers", "bookings", "bookings"],
            "FK parents must run first — a booking healed before its customer errors out"
        );
    }

    /// `age_secs` is `NOW() - detected_at`, so oldest-first == largest age
    /// first. This is what stops the `LIMIT 500` from starving old rows out
    /// of an arbitrary subset once the backlog exceeds the cap.
    #[test]
    fn reconcile_candidates_sort_oldest_detected_at_first_within_a_table() {
        let mut rows = vec![
            candidate(1, "bookings", "R000001|110", 3_600.0),
            candidate(2, "bookings", "R000002|110", 1_382_400.0), // 16 days
            candidate(3, "bookings", "R000003|110", 86_400.0),    // 1 day
        ];
        sort_reconcile_candidates(&mut rows);
        let ids: Vec<i64> = rows.iter().map(|r| r.0).collect();
        assert_eq!(
            ids,
            vec![2, 3, 1],
            "oldest detected_at (largest age_secs) must be retested first"
        );
    }

    #[test]
    fn reconcile_candidates_tie_break_on_id_is_deterministic() {
        let mut rows = vec![
            candidate(9, "customers", "C0009", 7_200.0),
            candidate(4, "customers", "C0004", 7_200.0),
            candidate(7, "customers", "C0007", 7_200.0),
        ];
        sort_reconcile_candidates(&mut rows);
        assert_eq!(rows.iter().map(|r| r.0).collect::<Vec<_>>(), vec![4, 7, 9]);
    }

    // -------------------------------------------------------------------
    // Level-drift recovery notification (paired all-clear).
    // -------------------------------------------------------------------

    fn owned(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| (*v).to_string()).collect()
    }

    #[test]
    fn level_drift_recovery_fires_when_cooldown_table_has_no_stale_rows() {
        // `bookings` alerted at some point (cooldown row exists) and now
        // has zero unconverged rows past the threshold ⇒ all-clear.
        let recovered = tables_recovered(&owned(&["bookings"]), &owned(&[]));
        assert_eq!(recovered, owned(&["bookings"]));
    }

    #[test]
    fn level_drift_recovery_suppressed_while_table_still_stale() {
        let recovered = tables_recovered(&owned(&["bookings"]), &owned(&["bookings"]));
        assert!(
            recovered.is_empty(),
            "a table that still has unconverged rows past the threshold has NOT recovered"
        );
    }

    #[test]
    fn level_drift_recovery_reports_only_the_converged_tables() {
        let recovered = tables_recovered(
            &owned(&["bookings", "customers", "checkins"]),
            &owned(&["checkins"]),
        );
        assert_eq!(
            recovered,
            owned(&["bookings", "customers"]),
            "output is sorted + deduped so the Slack body is deterministic"
        );
    }

    /// The cooldown table is shared with the stale-checkin tripwire. Its
    /// key has no unconverged-row count to compare against, so the sync-lag
    /// all-clear must never claim it recovered or clear its cooldown —
    /// doing so would let that alert refire every 15 minutes.
    #[test]
    fn level_drift_recovery_never_claims_non_reconcile_cooldown_keys() {
        let recovered = tables_recovered(
            &owned(&[STALE_CHECKIN_COOLDOWN_KEY, "customers"]),
            &owned(&[]),
        );
        assert_eq!(recovered, owned(&["customers"]));
    }

    #[test]
    fn level_drift_recovery_is_silent_without_cooldown_rows() {
        // Never alerted ⇒ nothing to close, even with zero stale rows.
        assert!(tables_recovered(&owned(&[]), &owned(&[])).is_empty());
    }

    // -------------------------------------------------------------------
    // 2026-07-28 alert inventory — escalation tier, env-tunable
    // thresholds, cooldown-on-successful-send, namespaced cooldown keys.
    // -------------------------------------------------------------------

    /// Defect A1. Below the escalation threshold the digest keeps its
    /// familiar `:warning:` voice; at or past it, the tone changes.
    /// Pinned at the boundary because an off-by-one here either fires the
    /// "will not self-heal" copy a day early (crying wolf) or never
    /// (the defect).
    #[test]
    fn escalation_does_not_fire_below_the_second_threshold() {
        for age in [0_i64, 4, 24, 48, 71] {
            assert_eq!(
                level_drift_severity(age, DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS),
                LevelDriftSeverity::Stale,
                "age {age}h is below the {DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS}h escalation \
                 threshold and must stay in the :warning: tier"
            );
        }
    }

    #[test]
    fn escalation_fires_at_and_past_the_second_threshold() {
        for age in [72_i64, 73, 388] {
            assert_eq!(
                level_drift_severity(age, DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS),
                LevelDriftSeverity::Escalated,
                "age {age}h has passed the escalation threshold"
            );
        }
    }

    /// The escalation threshold is env-tunable, so the classifier must
    /// track the passed-in value, not the compiled-in default.
    #[test]
    fn escalation_boundary_tracks_the_configured_threshold() {
        assert_eq!(level_drift_severity(11, 12), LevelDriftSeverity::Stale);
        assert_eq!(level_drift_severity(12, 12), LevelDriftSeverity::Escalated);
    }

    fn stale_row(table: &str, count: i64, oldest_age_hours: i64) -> StaleTable {
        StaleTable {
            table: table.to_string(),
            count,
            oldest_age_hours,
        }
    }

    /// A table lands in exactly ONE tier — an escalated table must not
    /// also get the `:warning:` digest, or the channel gets two messages
    /// per table per day about the same rows.
    #[test]
    fn partition_level_drift_splits_tiers_without_overlap() {
        let rows = vec![
            stale_row("bookings", 3, 388),
            stale_row("customers", 1, 6),
            stale_row("checkins", 9, 72),
        ];
        let (stale, escalated) = partition_level_drift(&rows, 72);
        assert_eq!(stale, vec![stale_row("customers", 1, 6)]);
        assert_eq!(
            escalated,
            vec![stale_row("bookings", 3, 388), stale_row("checkins", 9, 72)],
            "input order is preserved within a tier"
        );
    }

    // --- Issue #261 (re-scoped 2026-07-29) — pager-tier `<!channel>` mention ---
    //
    // No second webhook: on the shared Slack webhook, ONLY the pager tier
    // (>72h escalated digest, sync-lag burst, CT-lag pager, boot-refusal)
    // leads with `<!channel> ` so it breaks through mentions-only
    // notification prefs. Routine digests and all-clears must stay quiet.
    // These pin the ACTUAL composition each send site produces, using the
    // same `format_*` + `with_site_text[_paged]` calls as the real code.

    /// The >72h escalated digest (`:bangbang:`) is the pager tier's
    /// namesake in the issue — must lead with the exact mention.
    #[test]
    fn escalated_level_digest_composition_leads_with_channel_mention() {
        let body = format_escalated_level_digest_message(72, "• `bookings`: 3 unresolved row(s), oldest *388h*");
        let msg = SlackMessage::with_site_text_paged("hfhotel", body);
        assert!(
            msg.text.starts_with("<!channel> "),
            "escalated digest must lead with `<!channel> `; got {:?}",
            msg.text
        );
        assert!(msg.text.contains(":bangbang:"));
    }

    /// The routine `:warning:` digest is the alert the escalated tier
    /// exists to distinguish itself from — it must NEVER carry the
    /// mention, or the re-scope's whole "most sends stay quiet" premise
    /// breaks.
    #[test]
    fn stale_level_digest_composition_has_no_channel_mention() {
        let body = format_stale_level_digest_message(4, "• `customers`: 1 unresolved row(s), oldest 6h", 24, 72);
        let msg = SlackMessage::with_site_text("hfhotel", body);
        assert!(
            !msg.text.contains("<!channel>"),
            "routine :warning: digest must stay unmentioned; got {:?}",
            msg.text
        );
        assert!(msg.text.contains(":warning:"));
    }

    /// The reconcile all-clear (`:white_check_mark:`) must stay
    /// unmentioned — all-clears are explicitly excluded by the re-scope.
    #[test]
    fn level_drift_all_clear_composition_has_no_channel_mention() {
        let body = format_level_drift_all_clear_message(4, "• `customers`", 24);
        let msg = SlackMessage::with_site_text("hfhotel", body);
        assert!(!msg.text.contains("<!channel>"), "got {:?}", msg.text);
        assert!(msg.text.contains(":white_check_mark:"));
    }

    /// `:rotating_light:` sync-lag burst pages are named explicitly in
    /// the re-scope.
    #[test]
    fn burst_alert_composition_leads_with_channel_mention() {
        let body = format_burst_alert_message(50, "• `bookings`: 73 unresolved rows in last hour", 1);
        let msg = SlackMessage::with_site_text_paged("hfville", body);
        assert!(
            msg.text.starts_with("<!channel> "),
            "burst page must lead with `<!channel> `; got {:?}",
            msg.text
        );
        assert!(msg.text.contains(":rotating_light:"));
    }

    /// Day 1 and day 16 must not render identically — the whole point of
    /// carrying the oldest-row age in the body.
    #[test]
    fn humanize_hours_distinguishes_day_one_from_day_sixteen() {
        assert_eq!(humanize_hours(7), "7h");
        assert_eq!(humanize_hours(47), "47h");
        assert_eq!(humanize_hours(48), "2d (48h)");
        assert_eq!(humanize_hours(388), "16d 4h (388h)");
        assert_ne!(humanize_hours(7), humanize_hours(388));
    }

    // --- Cooldown key namespacing ---------------------------------------

    /// The escalation key must be structurally incapable of colliding
    /// with a canonical entity name in the shared
    /// `ht_level_drift_alert_cooldowns` table — the same guarantee
    /// `bin/sync.rs` gets from `ct_retention_overflow:<table>`.
    #[test]
    fn escalated_cooldown_key_cannot_collide_with_an_entity_name() {
        // `guest_registry` (Phase 6-B) is the first entity name carrying an
        // underscore — it must still read as a bare reconcile table key
        // (no `:`), and must not collide with any namespaced family.
        for table in ["bookings", "customers", "checkins", "rooms", "guest_registry"] {
            let key = escalated_cooldown_key(table);
            assert_ne!(key, table, "escalation key must not equal the entity name");
            assert!(
                key.contains(COOLDOWN_KEY_NAMESPACE_SEP),
                "escalation key must carry the namespace separator: {key}"
            );
            assert!(
                !is_reconcile_table_key(&key),
                "{key} must not be treated as a reconcile table name"
            );
            assert!(
                is_reconcile_table_key(table),
                "the bare entity name {table} IS a reconcile table name"
            );
        }
        assert_eq!(escalated_cooldown_key("bookings"), "escalated:bookings");
    }

    #[test]
    fn burst_cooldown_key_cannot_collide_with_an_entity_name() {
        let key = burst_cooldown_key("bookings");
        assert_eq!(key, "burst:bookings");
        assert!(!is_reconcile_table_key(&key));
    }

    /// The all-clear diffs cooldown keys against still-stale table names.
    /// A namespaced key never matches a table name, so without this
    /// filter it would be reported "converged" and its cooldown DELETED
    /// — silently un-throttling the alert it belongs to.
    ///
    /// The `bin/sync.rs` literals below are the other half of a
    /// cross-file contract (its `*_KEY_PREFIX` / `*_COOLDOWN_KEY`
    /// constants, private to that binary, hence literals here). They have
    /// shared this table since the retention-page work and the all-clear
    /// has been eligible to delete them the whole time.
    #[test]
    fn all_clear_never_claims_namespaced_cooldown_keys() {
        let recovered = tables_recovered(
            &owned(&[
                "customers",
                "escalated:bookings",
                "burst:checkins",
                "reconcile_cap:guest_registry",
                // Parked by bin/sync.rs in the same shared table.
                "ct_retention_overflow:HT_Customers",
                "ct_watcher_lag:global",
                "shadow_mode:ceiling",
                "boot_refusal:ct_gap",
                STALE_CHECKIN_COOLDOWN_KEY,
            ]),
            &owned(&["bookings"]),
        );
        assert_eq!(
            recovered,
            owned(&["customers"]),
            "only bare reconcile table names may be declared converged"
        );
    }

    // --- Env-overridable level-drift thresholds (defect A2) --------------

    const LEVEL_DRIFT_ENV_VARS: &[&str] = &[
        "LEVEL_DRIFT_STALE_INTERVAL_HOURS",
        "LEVEL_DRIFT_COOLDOWN_HOURS",
        "LEVEL_DRIFT_ESCALATE_HOURS",
        "LEVEL_DRIFT_STALE_INTERVAL_HOURS_HFHOTEL",
        "LEVEL_DRIFT_COOLDOWN_HOURS_HFHOTEL",
        "LEVEL_DRIFT_ESCALATE_HOURS_HFHOTEL",
        "LEVEL_DRIFT_STALE_INTERVAL_HOURS_HFVILLE",
        "LEVEL_DRIFT_COOLDOWN_HOURS_HFVILLE",
        "LEVEL_DRIFT_ESCALATE_HOURS_HFVILLE",
    ];

    /// Env-isolation helper in the `with_mode_env` / `with_threshold_envs`
    /// idiom. Clears the whole level-drift var family first so an ambient
    /// value can't flip an assertion, sets the requested ones, then
    /// restores every prior value.
    fn with_level_drift_env<T, F: FnOnce() -> T>(set: &[(&str, &str)], f: F) -> T {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let prior: Vec<(&str, Option<String>)> = LEVEL_DRIFT_ENV_VARS
            .iter()
            .map(|name| (*name, env::var(name).ok()))
            .collect();
        for name in LEVEL_DRIFT_ENV_VARS {
            env::remove_var(name);
        }
        for (name, value) in set {
            env::set_var(name, value);
        }
        let out = f();
        for (name, value) in prior {
            match value {
                Some(v) => env::set_var(name, v),
                None => env::remove_var(name),
            }
        }
        out
    }

    /// Defaults are explicitly unchanged by the env work — a deploy with
    /// no new vars set must behave exactly as it did before.
    #[test]
    fn level_drift_thresholds_default_when_env_unset() {
        let t = with_level_drift_env(&[], || level_drift_thresholds_from_env("hfhotel"));
        assert_eq!(t.stale_hours, 4);
        assert_eq!(t.cooldown_hours, 24);
        assert_eq!(t.escalate_hours, 72);
        assert_eq!(t.stale_hours, DEFAULT_LEVEL_DRIFT_STALE_INTERVAL_HOURS);
        assert_eq!(t.cooldown_hours, DEFAULT_LEVEL_DRIFT_COOLDOWN_HOURS);
        assert_eq!(t.escalate_hours, DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS);
    }

    #[test]
    fn level_drift_thresholds_read_the_global_env_vars() {
        let t = with_level_drift_env(
            &[
                ("LEVEL_DRIFT_STALE_INTERVAL_HOURS", "8"),
                ("LEVEL_DRIFT_COOLDOWN_HOURS", "12"),
                ("LEVEL_DRIFT_ESCALATE_HOURS", "96"),
            ],
            || level_drift_thresholds_from_env("hfhotel"),
        );
        assert_eq!((t.stale_hours, t.cooldown_hours, t.escalate_hours), (8, 12, 96));
    }

    /// Per-site override wins over the global, and does not leak to the
    /// other site — same contract as the drift-alert threshold (#69).
    #[test]
    fn level_drift_thresholds_per_site_override_wins_and_does_not_leak() {
        let set = [
            ("LEVEL_DRIFT_COOLDOWN_HOURS", "24"),
            ("LEVEL_DRIFT_COOLDOWN_HOURS_HFVILLE", "6"),
        ];
        let ville = with_level_drift_env(&set, || level_drift_thresholds_from_env("hfville"));
        assert_eq!(ville.cooldown_hours, 6, "per-site override must win");
        let hotel = with_level_drift_env(&set, || level_drift_thresholds_from_env("hfhotel"));
        assert_eq!(
            hotel.cooldown_hours, 24,
            "HF Hotel must not pick up HF Ville's override"
        );
    }

    /// Operator typos degrade to the next tier down, never to zero — a
    /// zero cooldown would turn the digest into a 15-minute metronome.
    #[test]
    fn level_drift_thresholds_fall_back_on_invalid_values() {
        let t = with_level_drift_env(
            &[
                ("LEVEL_DRIFT_STALE_INTERVAL_HOURS", "not-a-number"),
                ("LEVEL_DRIFT_COOLDOWN_HOURS", "0"),
                ("LEVEL_DRIFT_ESCALATE_HOURS", "-5"),
            ],
            || level_drift_thresholds_from_env("hfhotel"),
        );
        assert_eq!(t.stale_hours, DEFAULT_LEVEL_DRIFT_STALE_INTERVAL_HOURS);
        assert_eq!(t.cooldown_hours, DEFAULT_LEVEL_DRIFT_COOLDOWN_HOURS);
        assert_eq!(t.escalate_hours, DEFAULT_LEVEL_DRIFT_ESCALATE_HOURS);
    }

    #[test]
    fn level_drift_thresholds_per_site_garbage_falls_through_to_global() {
        let t = with_level_drift_env(
            &[
                ("LEVEL_DRIFT_ESCALATE_HOURS", "96"),
                ("LEVEL_DRIFT_ESCALATE_HOURS_HFVILLE", "abc"),
            ],
            || level_drift_thresholds_from_env("hfville"),
        );
        assert_eq!(t.escalate_hours, 96);
    }

    /// An escalation threshold at or below the stale interval would
    /// escalate every table on its first digest, collapsing the two tiers
    /// back into the single unchanging voice this work removes.
    #[test]
    fn level_drift_escalate_threshold_is_clamped_above_the_stale_interval() {
        let t = with_level_drift_env(
            &[
                ("LEVEL_DRIFT_STALE_INTERVAL_HOURS", "10"),
                ("LEVEL_DRIFT_ESCALATE_HOURS", "4"),
            ],
            || level_drift_thresholds_from_env("hfhotel"),
        );
        assert_eq!(t.stale_hours, 10);
        assert_eq!(t.escalate_hours, 11, "clamped to stale + 1h");
        assert_eq!(
            level_drift_severity(t.stale_hours, t.escalate_hours),
            LevelDriftSeverity::Stale,
            "a row that only just crossed the stale interval must not escalate"
        );
    }

    #[test]
    fn level_drift_cooldown_duration_matches_the_configured_hours() {
        let t = with_level_drift_env(&[("LEVEL_DRIFT_COOLDOWN_HOURS", "6")], || {
            level_drift_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.cooldown(), std::time::Duration::from_secs(6 * 3600));
    }

    // --- Cooldown burns only on a successful send (defect A3) ------------

    /// A failed webhook must NOT silence the table: the cooldown stays
    /// unset so the next 15-minute tick retries. Pre-fix the mark ran
    /// before the POST, so an outage bought 24h of silence and the
    /// all-clear could later close an alert nobody received.
    #[test]
    fn cooldown_is_not_marked_when_the_send_fails() {
        assert_eq!(AlertDelivery::from_send(Some(false)), AlertDelivery::Failed);
        assert!(!cooldown_should_be_marked(AlertDelivery::Failed));
    }

    #[test]
    fn cooldown_is_marked_when_the_send_succeeds() {
        assert_eq!(AlertDelivery::from_send(Some(true)), AlertDelivery::Sent);
        assert!(cooldown_should_be_marked(AlertDelivery::Sent));
    }

    /// No Slack client configured is not a failure — the `tracing` line
    /// IS the delivery, so the cooldown still throttles it. Otherwise a
    /// log-only deployment repeats the warning every 15 minutes.
    #[test]
    fn cooldown_is_marked_in_log_only_deployments() {
        assert_eq!(AlertDelivery::from_send(None), AlertDelivery::LoggedOnly);
        assert!(cooldown_should_be_marked(AlertDelivery::LoggedOnly));
    }

    // --- Burst-alert cooldown (defect C5) --------------------------------

    /// The burst threshold is a blast-radius dial, not a target: 21 days
    /// of production peak at 33 rows/hr, so it has never fired and should
    /// not be "tuned down until it does". Pinned so a drive-by change has
    /// to argue with a test.
    #[test]
    fn burst_alert_threshold_default_is_unchanged() {
        assert_eq!(DEFAULT_DRIFT_ALERT_THRESHOLD, 50);
        // 33 is the observed 21-day production peak. It must stay BELOW
        // the threshold: the alert is designed never to fire in normal
        // operation, and the level digest already covers the slow-burn
        // case this would otherwise duplicate.
        assert!(
            tables_breaching_threshold(&counts(&[("checkins", 33)]), DEFAULT_DRIFT_ALERT_THRESHOLD)
                .is_empty(),
            "the observed production peak must not trip the burst alert"
        );
    }

    #[test]
    fn burst_cooldown_hours_defaults_and_honours_per_site_override() {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let global = "LEGACY_RECONCILE_BURST_COOLDOWN_HOURS";
        let per_site = "LEGACY_RECONCILE_BURST_COOLDOWN_HOURS_HFVILLE";
        let prior = (env::var(global).ok(), env::var(per_site).ok());
        env::remove_var(global);
        env::remove_var(per_site);
        assert_eq!(
            burst_cooldown_hours_from_env("hfhotel"),
            DEFAULT_BURST_ALERT_COOLDOWN_HOURS
        );
        env::set_var(global, "3");
        env::set_var(per_site, "12");
        assert_eq!(burst_cooldown_hours_from_env("hfhotel"), 3);
        assert_eq!(burst_cooldown_hours_from_env("hfville"), 12);
        match prior.0 {
            Some(v) => env::set_var(global, v),
            None => env::remove_var(global),
        }
        match prior.1 {
            Some(v) => env::set_var(per_site, v),
            None => env::remove_var(per_site),
        }
    }

    // -------------------------------------------------------------------
    // Per-table CT watermark health check (R3 TODO resolution).
    // -------------------------------------------------------------------

    /// `now` is threaded in explicitly (rather than each row reaching for
    /// `Utc::now()`) so `poll_age_seconds` comes out as an exact integer —
    /// otherwise sub-second construction skew truncates 4000s to 3999s and
    /// the assertions flake.
    fn per_table_at(
        now: chrono::DateTime<chrono::Utc>,
        name: &str,
        version: i64,
        polled_secs_ago: Option<i64>,
    ) -> PerTableWatermark {
        PerTableWatermark {
            table_name: name.to_string(),
            last_seen_version: version,
            last_polled_at: polled_secs_ago.map(|s| now - chrono::Duration::seconds(s)),
        }
    }

    #[test]
    fn per_table_watermark_flag_defaults_off() {
        let on = with_self_heal_flag_env("SYNC_PER_TABLE_WATERMARK", None, {
            per_table_watermark_enabled
        });
        assert!(!on, "global-watermark mode stays the default");
        let on = with_self_heal_flag_env("SYNC_PER_TABLE_WATERMARK", Some("true"), {
            per_table_watermark_enabled
        });
        assert!(on);
    }

    #[test]
    fn stalest_per_table_watermark_returns_none_for_empty_input() {
        assert!(stalest_per_table_watermark(&[], 100, chrono::Utc::now()).is_none());
    }

    /// The point of the whole exercise: report WHICH table is behind, not a
    /// global figure that is frozen at its bootstrap value in per-table mode.
    #[test]
    fn stalest_per_table_watermark_picks_largest_version_lag() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Customers", 990, Some(10)),
            per_table_at(now, "HT_Book_H", 500, Some(10)),
            per_table_at(now, "HT_Rooms", 1_000, Some(10)),
        ];
        let (stalest, version_lag, _) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(stalest.table_name, "HT_Book_H");
        assert_eq!(version_lag, 500);
    }

    #[test]
    fn stalest_per_table_watermark_breaks_version_ties_on_oldest_poll() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Customers", 1_000, Some(30)),
            per_table_at(now, "HT_CheckIn_H", 1_000, Some(4_000)),
        ];
        let (stalest, version_lag, poll_age) =
            stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(stalest.table_name, "HT_CheckIn_H");
        assert_eq!(version_lag, 0);
        assert_eq!(poll_age, 4_000);
    }

    /// A never-polled table is infinitely old — same convention the global
    /// arm uses so the warn branch fires and the operator sees it.
    #[test]
    fn stalest_per_table_watermark_treats_never_polled_as_infinitely_old() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Customers", 1_000, Some(10)),
            per_table_at(now, "HT_Book_Pro", 1_000, None),
        ];
        let (stalest, _, poll_age) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(stalest.table_name, "HT_Book_Pro");
        assert_eq!(poll_age, i64::MAX);
        assert!(ct_lag_is_warning(0, poll_age, default_ct_thresholds()));
    }

    /// A watermark ahead of `CHANGE_TRACKING_CURRENT_VERSION()` is a CT
    /// anomaly; saturating-sub keeps it at zero lag rather than panicking.
    #[test]
    fn stalest_per_table_watermark_saturates_on_watermark_ahead_of_current() {
        let now = chrono::Utc::now();
        let rows = vec![per_table_at(now, "HT_Rooms", 5_000, Some(1))];
        let (_, version_lag, _) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(version_lag, 0);
    }

    /// Equally-healthy tables must produce a stable pick so the log line
    /// doesn't flap between tables every tick.
    #[test]
    fn stalest_per_table_watermark_is_deterministic_on_a_full_tie() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Rooms", 1_000, Some(5)),
            per_table_at(now, "HT_Customers", 1_000, Some(5)),
        ];
        let (first, _, _) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(first.table_name, "HT_Customers");
    }
}
