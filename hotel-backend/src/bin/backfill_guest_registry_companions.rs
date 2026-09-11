//! One-shot backfill: legacy MSSQL `HT_CheckIn_Other_People` -> canonical PG
//! `ht_guest_registry` (companion-guest gap found by the `guest_registry`
//! reconcile arm on its HF Ville live debut, 2026-07-31).
//!
//! ## Why this exists
//!
//! Enabling the `guest_registry` reconcile arm on HF Ville (`68a30cc`, T2
//! step 3/8) immediately surfaced 30 open `value` divergences: canonical
//! carries ZERO companions for 29 of the 30 flagged folios, and only 1 of 2
//! for the 30th (`CH26-001524`). Net shortfall: 30 companion records
//! (legacy 31 rows vs canonical 1 across those folios).
//!
//! **Root cause is two ALREADY-FIXED sync bugs**, not an active leak:
//! `30d2d72` (2026-06-11, FK-defer silent-drop) covers the 2026-05-13 →
//! 06-07 cluster, and `42dc2c0` (2026-07-27, global watermark clobber)
//! covers the 06-13 → 07-11 tail. Canonical's mirror tracks legacy's max
//! companion id (24918) exactly with nothing missing after 07-11 — the leak
//! is closed. This bin repairs the historical damage those two bugs already
//! left behind; it fixes nothing that is still actively happening.
//!
//! ## Why `guest_registry` cannot self-heal (and never will, without this)
//!
//! `guest_registry` is deliberately absent from BOTH
//! `scheduler::sync::FORCE_CONVERGE_VALUE_DRIFT_TABLES` (its self-heal
//! would have to DELETE canonical companion rows legacy no longer has —
//! destroying TM.30 registry state from a sweep) and
//! `REINGEST_MISSING_PG_TABLES` (see the comments beside both consts,
//! `scheduler/sync.rs` ~4453-4477). More fundamentally, this class of
//! divergence classifies as `value` — not `missing_pg` — because the
//! reconcile unit is the FOLIO (all companions sharing one `Cin_no`, keyed
//! by `legacy_pk = Cin_no`), and every one of these folios' parent
//! check-in already exists canonically; only some (or all) of its
//! companions are missing. The folio's hash differs, which is a `value`
//! divergence, and neither self-heal arm is wired for `guest_registry` at
//! all. **These 30 rows will never auto-heal — a backfill is the only path
//! to closure.**
//!
//! ## Legacy access is READ-ONLY — no dark flag needed
//!
//! This bin NEVER writes to legacy MSSQL. It only issues plain `SELECT`s
//! against `HT_CheckIn_Other_People` (routed through
//! `simple_query_with_timeout_pooled`, `MssqlOpKind::Read` — see
//! `db::mssql_timeout`). Every write lands in canonical PostgreSQL only.
//! That is why this closes without a `_ENABLED` env flag or a
//! reception-coordinated live test, same reasoning as
//! `backfill_receipt_payments` / `backfill_room_calendar`.
//!
//! ## What it does
//!
//! 1. **Work-list**: a full `HT_CheckIn_Other_People` scan (`id, Cin_no,
//!    Cin_name, Cin_contry` — small table, ~20k rows per site, one query),
//!    diffed IN MEMORY against two canonical sets:
//!    * the in-era `Cin_no` folio keys (`ht_checkins.legacy_cin_no` whose
//!      `cin_checkin_time` is at/after the SAME era floor the
//!      `guest_registry` reconcile arm itself uses — see "Era floor"
//!      below), and
//!    * every `guest_legacy_id` canonical already carries.
//!
//!    A legacy row is a candidate iff its `Cin_no` is in-era AND its `id`
//!    is NOT in the canonical `guest_legacy_id` set. Diffing at the
//!    per-ROW `id` level (not per-folio) is what makes the duplicate-twin
//!    case (`CH26-001524`, two legacy rows with duplicate name/blank
//!    country, one already mirrored) come out right automatically: the
//!    already-mirrored row's `id` is excluded, the missing twin's `id`
//!    is not, and nothing needs folio-aware special-casing.
//!
//!    Positional CLI args (`Cin_no` values) REPLACE the era-floored scope
//!    with an explicit folio set — re-run a subset. The legacy scan itself
//!    is unchanged (still one full-table read); only the in-scope key set
//!    changes. Because this bypasses the floor entirely, if an explicitly-
//!    passed `Cin_no` resolves to a parent check-in whose `cin_checkin_time`
//!    falls OUTSIDE the era floor, this bin prints a non-fatal WARNING (never
//!    a failure — the override is legitimate and stays fully functional):
//!    the row still gets written, but the `guest_registry` reconcile arm's
//!    own scope is floor-gated, so it will never auto-close that row via the
//!    designed self-verifying path (point 7 below). See
//!    [`resolved_parent_is_outside_era_floor`].
//!
//! 2. **Era floor**: read-only. Combines the SAME two sources
//!    `scheduler::sync::guest_registry_era_floor` clamps together —
//!    the persisted, monotonically-non-decreasing watermark in
//!    `ht_reconcile_era_floor` (table_name = `'guest_registry'`) and the
//!    derived `date_trunc('day', MIN(cin_checkin_time))` over already-
//!    mirrored, non-primary companions — via the identical `GREATEST`
//!    semantics ([`clamped_era_floor`]). This bin reads that watermark but
//!    **never writes it** — persisting/advancing the floor is exclusively
//!    the reconcile arm's job; a backfill reading the same basis must not
//!    perturb it. If neither source yields a floor (no canonical coverage
//!    established yet — e.g. HF Hotel before its arm enables, ~2026-08-02)
//!    and no explicit `Cin_no` args were given, the run reports a clean
//!    zero and exits rather than scanning unfloored (same "no coverage,
//!    nothing to reconcile against" posture `sync_guest_registry` itself
//!    takes).
//!
//! 3. **Re-drives the mapper's own upsert path.** For each candidate this
//!    bin calls [`GuestRegistryMapper::apply`] (`ChangeOp::Insert`) — the
//!    SAME trait method the live CT watcher calls, with its full
//!    echo-adoption / echo-preserve / upsert logic intact. It does NOT
//!    hand-write a bespoke `INSERT` against `ht_guest_registry`; that
//!    divergence-from-the-mapper is the exact bug class that caused #204's
//!    customer drift (see the mapper module's own docs). Concretely this
//!    means: `RoomCalendarMapper` in `backfill_room_calendar` is the
//!    precedent followed here — the mapper's trait `apply` is directly
//!    callable from a bin, so there is no free function to reuse instead
//!    (unlike `backfill_receipt_payments`, which re-drives
//!    `payment::apply_receipt_upsert`).
//!
//! 4. **Idempotent**, two ways: an explicit pre-check
//!    (`SELECT guest_id FROM ht_guest_registry WHERE guest_legacy_id = $1`)
//!    reports an already-mirrored row as a true no-op before ever calling
//!    the mapper; and even if a race slipped past that check, the mapper's
//!    own `UPSERT_COMPANION_SQL` is `ON CONFLICT (guest_legacy_id) DO
//!    UPDATE` (migration 034's UNIQUE constraint), so a repeat apply
//!    converges in place rather than duplicating. Safe to run twice.
//!
//! 5. **Parent-FK handling.** All 30 known parent check-ins already exist
//!    canonically (per the investigation), but this bin resolves the
//!    parent itself first (`sync::resolve::resolve_checkin_id`, the SAME
//!    lookup the mapper performs internally) purely so it can report the
//!    resolved id and make an explicit, loud SKIP decision — never a run
//!    failure and never a silent drop — if a parent is ever unresolvable.
//!    The live CT watcher's mapper instead returns a hard `Err` on this
//!    (correct for it: it must hold the watermark for a retry). A backfill
//!    has nothing to retry against, so skip-and-report is the right
//!    response here.
//!
//! 6. `--dry-run` resolves everything (parent lookup, idempotency
//!    pre-check) but commits no PG transaction — writes NOTHING. Reports
//!    legacy id, `Cin_no`, name, country, and the resolved canonical
//!    check-in id for every candidate.
//!
//! 7. **This bin does NOT mark any `ht_reconcile_log` row resolved.** The
//!    generic auto-resolve sweep re-hashes both sides on its own schedule
//!    and closes converged rows itself once the companion counts and
//!    content match — that is the designed, self-verifying path (it proves
//!    the write actually took, rather than this bin asserting it did), and
//!    this bin deliberately does not race or duplicate it. After a
//!    successful run, expect the `guest_registry` `value` backlog to clear
//!    on the next reconcile tick, not immediately.
//!
//! 8. Domain events: `GuestRegistryMapper::apply` never returns one on the
//!    Insert/Update path (all three branches return `Ok(None)` — see the
//!    mapper source), so there is nothing to suppress here. Matches the
//!    established precedent anyway (`backfill_room_calendar`,
//!    `backfill_receipt_payments`): a backfilled historical row is not a
//!    new event from a domain perspective.
//!
//! 9. **Site-parameterized** — no new mechanism. Like every sibling
//!    backfill bin, this one reads `DATABASE_URL`/`NEW_DATABASE_URL` for
//!    the PG side and `DB_SERVER`/`DB_NAME`/… (`DbConfig::from_env`) for
//!    the legacy side; `SITE_ID` (`SiteConfig::from_env`) only tags log
//!    lines. Running it against HF Hotel's env once its arm enables
//!    (~2026-08-02, step 4/8) needs no code change.
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//!
//! # ALWAYS start here — reports what would be written, writes nothing.
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_guest_registry_companions -- --dry-run
//!
//! # Live run — drains the current era-floored companion gap.
//! cargo run --release --bin backfill_guest_registry_companions
//!
//! # Re-run an explicit subset of folios (e.g. to re-verify CH26-001524):
//! cargo run --release --bin backfill_guest_registry_companions -- --dry-run \
//!   CH26-001524
//! ```
//!
//! ## Flags
//!
//! * `--dry-run` — resolve everything, commit nothing. Reports what WOULD
//!   be written.
//! * positional args — explicit `Cin_no` (folio) values. When given, these
//!   REPLACE the era-floored scope, not filter it.

use std::collections::HashSet;
use std::env;

use chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use hotel_backend::config::{DbConfig, SiteConfig};
use hotel_backend::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::GuestRegistryMapper;
use hotel_backend::sync::resolve;
use hotel_backend::sync::row::MappableRow;
use hotel_backend::sync::SyncError;

const PG_POOL_MAX: u32 = 4;
const MSSQL_POOL_MAX: u32 = 4;
const TABLE: &str = "HT_CheckIn_Other_People";

/// `ht_reconcile_era_floor.table_name` / `ht_reconcile_log.table_name` for
/// this entity — same literal `scheduler::sync` reports under.
const ERA_FLOOR_KEY: &str = "guest_registry";

/// Full legacy scan. No WHERE clause — the era/explicit scoping happens in
/// memory against the canonical key sets (see module docs point 1); the
/// table is small enough (~20k rows/site, per `scheduler::sync`'s own
/// bulk-scan comment) that this is cheaper than the per-PK loads other
/// arms use. Columns match `sync::mappers::guest_registry::
/// GUEST_REGISTRY_SELECT_COLS` field-for-field (kept independent — that
/// const is table-alias qualified for the CT JOIN and private to the
/// mapper module, same independence idiom `backfill_receipt_payments`
/// uses for `RECEIPT_COLS`). Preserves the iHOTEL typo `Cin_contry`
/// verbatim per the user's standing constraint.
const LEGACY_SCAN_SQL: &str =
    "SELECT id, Cin_no, Cin_name, Cin_contry FROM HT_CheckIn_Other_People";

/// Every legacy companion id canonical already carries. `guest_legacy_id`
/// is UNIQUE (migration 034), so this set is exactly "already mirrored".
const CANONICAL_MIRRORED_IDS_SQL: &str =
    "SELECT guest_legacy_id FROM ht_guest_registry WHERE guest_legacy_id IS NOT NULL";

/// In-era folio keys. Byte-identical query shape to
/// `scheduler::sync::IN_ERA_CHECKIN_KEYS_SQL` (independent copy — that
/// const is private to the scheduler module).
const IN_ERA_CHECKIN_KEYS_SQL: &str = "SELECT legacy_cin_no FROM ht_checkins \
     WHERE legacy_cin_no IS NOT NULL AND cin_checkin_time >= $1";

/// The persisted half of the era-floor clamp — read-only. Independent copy
/// of the SELECT half of `scheduler::sync::RECONCILE_ERA_FLOOR_UPSERT_SQL`;
/// this bin never runs the upsert half (see module docs point 2).
const PERSISTED_ERA_FLOOR_SQL: &str =
    "SELECT era_floor FROM ht_reconcile_era_floor WHERE table_name = $1";

/// The derived half of the era-floor clamp. Byte-identical basis to
/// `scheduler::sync::guest_registry_era_floor_sql()` (independent copy —
/// that fn is private to the scheduler module).
const DERIVED_ERA_FLOOR_SQL: &str = "SELECT date_trunc('day', MIN(ht_checkins.cin_checkin_time)) \
     FROM ht_guest_registry \
     JOIN ht_checkins ON ht_checkins.cin_id = ht_guest_registry.guest_cin_id \
    WHERE COALESCE(guest_is_primary, false) = false \
      AND ht_guest_registry.guest_legacy_id IS NOT NULL";

/// Idempotency pre-check — the SAME conflict target the mapper's own
/// `UPSERT_COMPANION_SQL` resolves on (`guest_legacy_id` UNIQUE).
const EXISTING_MIRRORED_PROBE_SQL: &str =
    "SELECT guest_id FROM ht_guest_registry WHERE guest_legacy_id = $1";

/// Gap-2 hardening — read-only lookup used ONLY to power the out-of-era
/// WARNING for explicit `Cin_no` overrides (module docs point 1). Never
/// queried on the default era-floored path.
const PARENT_CHECKIN_TIME_SQL: &str = "SELECT cin_checkin_time FROM ht_checkins WHERE cin_id = $1";

#[derive(Debug, Default)]
struct Summary {
    legacy_scanned: usize,
    in_scope: usize,
    already_mirrored: usize,
    candidates: usize,
    written: usize,
    skipped_already_present: usize,
    skipped_unresolvable_parent: usize,
    errored: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(
            |_| "backfill_guest_registry_companions=info,hotel_backend=info".into(),
        ))
        .init();

    let (dry_run, explicit_cin_nos) = parse_args(env::args().skip(1));
    let site = SiteConfig::from_env();

    tracing::info!(
        site = %site.id, dry_run, explicit_count = explicit_cin_nos.len(),
        "guest-registry companion backfill — closing the 2026-05-13..07-11 sync-bug gap"
    );

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let mirrored_ids = fetch_mirrored_legacy_ids(&pg).await?;

    let explicit_scope = !explicit_cin_nos.is_empty();
    let (in_scope, scope_desc, era_floor_for_warning): (
        HashSet<String>,
        String,
        Option<NaiveDateTime>,
    ) = if !explicit_scope {
        match fetch_era_floor(&pg).await? {
            Some(floor) => {
                let keys = fetch_in_era_checkin_keys(&pg, floor).await?;
                (
                    keys,
                    format!(
                        "{floor} (derived — same basis the guest_registry reconcile arm uses)"
                    ),
                    Some(floor),
                )
            }
            None => {
                println!();
                println!("=== backfill_guest_registry_companions ===");
                println!("Site:    {}", site.id);
                println!(
                    "No canonical companion coverage established yet — ht_guest_registry \
                     holds no mirrored companion and ht_reconcile_era_floor has no \
                     'guest_registry' row. Nothing to backfill against on this site; run \
                     the guest_registry reconcile arm at least once, or pass explicit \
                     Cin_no values."
                );
                tracing::info!(site = %site.id, "no era-floor coverage — nothing to do");
                return Ok(());
            }
        }
    } else {
        let keys: HashSet<String> = explicit_cin_nos.iter().cloned().collect();
        let desc = format!("none — explicit Cin_no list ({} folios)", keys.len());
        // Gap-2 hardening: read-only, best-effort. This does NOT filter or
        // replace the explicit scope above (module docs point 1, unchanged)
        // — it only powers the out-of-era WARNING in `process_candidate`. A
        // failure here must not abort an otherwise-valid explicit override
        // run, so it degrades to "no warning check" rather than propagating.
        let floor = match fetch_era_floor(&pg).await {
            Ok(f) => f,
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "failed to read era floor for the explicit-Cin_no out-of-era warning \
                     check — continuing without it (non-fatal, does not affect scope)"
                );
                None
            }
        };
        (keys, desc, floor)
    };

    tracing::info!(
        site = %site.id, scope = %scope_desc, in_scope_folios = in_scope.len(),
        "scan scope resolved"
    );

    let legacy_rows = fetch_legacy_companions(&mssql).await?;

    let mut s = Summary { legacy_scanned: legacy_rows.len(), ..Default::default() };
    let mut candidates: Vec<(&tiberius::Row, LegacyCompanionRow)> = Vec::new();

    for row in &legacy_rows {
        let preview = match extract_legacy_row(row) {
            Ok(p) => p,
            Err(err) => {
                tracing::warn!(error = %err, "failed to read legacy companion row — skipping");
                println!("[ERROR] failed to read legacy HT_CheckIn_Other_People row: {err}");
                s.errored += 1;
                continue;
            }
        };

        match classify(&preview, &in_scope, &mirrored_ids) {
            ScopeVerdict::OutOfScope => {}
            ScopeVerdict::AlreadyMirrored => {
                s.in_scope += 1;
                s.already_mirrored += 1;
            }
            ScopeVerdict::Candidate => {
                s.in_scope += 1;
                s.candidates += 1;
                candidates.push((row, preview));
            }
        }
    }

    for (row, preview) in &candidates {
        process_candidate(
            &pg,
            row,
            preview,
            dry_run,
            explicit_scope,
            era_floor_for_warning,
            &mut s,
        )
        .await;
    }

    println!();
    println!("=== backfill_guest_registry_companions ===");
    println!("Site:                           {}", site.id);
    println!(
        "Mode:                           {}",
        if dry_run { "DRY RUN (writes nothing)" } else { "LIVE" }
    );
    println!("Scope:                          {scope_desc}");
    println!("Legacy companions scanned:      {}", s.legacy_scanned);
    println!("In scope:                       {}", s.in_scope);
    println!("Already mirrored (no-op):       {}", s.already_mirrored);
    println!("Candidates:                     {}", s.candidates);
    println!(
        "{:<32}{}",
        if dry_run { "Would write:" } else { "Written:" },
        s.written
    );
    println!("Skipped (already present):      {}", s.skipped_already_present);
    println!("Skipped (unresolvable parent):  {}", s.skipped_unresolvable_parent);
    println!("Errored:                        {}", s.errored);
    if !dry_run && s.written > 0 {
        println!();
        println!(
            "Note: ht_reconcile_log rows are NOT marked resolved by this bin. The next \
             guest_registry reconcile tick re-hashes both sides and closes converged \
             folios itself — that is the designed, self-verifying path."
        );
    }

    tracing::info!(
        site = %site.id,
        dry_run,
        legacy_scanned = s.legacy_scanned,
        in_scope = s.in_scope,
        already_mirrored = s.already_mirrored,
        candidates = s.candidates,
        written = s.written,
        skipped_already_present = s.skipped_already_present,
        skipped_unresolvable_parent = s.skipped_unresolvable_parent,
        errored = s.errored,
        "backfill_guest_registry_companions — done"
    );

    Ok(())
}

/// Parse CLI args into `(dry_run, explicit_cin_nos)`. Pure — no I/O — so
/// it is unit-testable without touching `env::args()`. Positional args
/// (anything not starting with `--`) are explicit `Cin_no` (folio) values
/// to re-run a subset (module docs); when given they REPLACE the
/// era-floored scope rather than filtering it. Unrecognised `--flags` are
/// ignored, matching the permissive parsing style of the other backfill
/// bins.
fn parse_args<I: IntoIterator<Item = String>>(args: I) -> (bool, Vec<String>) {
    let mut dry_run = false;
    let mut explicit = Vec::new();
    for a in args {
        if a == "--dry-run" {
            dry_run = true;
        } else if !a.starts_with("--") {
            explicit.push(a);
        }
    }
    (dry_run, explicit)
}

/// The clamp semantics, as a pure function — byte-identical rule to
/// `scheduler::sync::clamped_era_floor` (independent copy; that fn is
/// private to the scheduler module). See that fn's doc for the full
/// rationale; summary: both present → the LATER one (a derived floor
/// dragged backwards by one historical companion edit must never widen the
/// scope); persisted only → keep the watermark; neither → `None`.
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

/// The persisted half of the era-floor clamp — a single query, split out of
/// `fetch_era_floor` so it is independently testable against a temp-table
/// fixture (see tests below), mirroring the `fetch_canonical_keys` /
/// `fetch_canonical_floor` split in `backfill_room_calendar`. `impl
/// sqlx::PgExecutor<'_>` (rather than `&PgPool`) so both `fetch_era_floor`
/// (a `&PgPool`, which is `Copy`) and the unit tests (a `&mut
/// Transaction`, reborrowed) can call it.
async fn fetch_persisted_era_floor(
    pg: impl sqlx::PgExecutor<'_>,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    sqlx::query_scalar(PERSISTED_ERA_FLOOR_SQL).bind(ERA_FLOOR_KEY).fetch_optional(pg).await
}

/// The derived half of the era-floor clamp — a single query, split out of
/// `fetch_era_floor` for the same reason as `fetch_persisted_era_floor`.
async fn fetch_derived_era_floor(
    pg: impl sqlx::PgExecutor<'_>,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    sqlx::query_scalar(DERIVED_ERA_FLOOR_SQL).fetch_one(pg).await
}

/// Read-only era floor: the SAME clamp the `guest_registry` reconcile arm
/// applies, but this bin never writes `ht_reconcile_era_floor` — see module
/// docs point 2. `&PgPool` is `Copy`, so it can be passed to both halves
/// below without an explicit reborrow.
async fn fetch_era_floor(pg: &PgPool) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    let persisted = fetch_persisted_era_floor(pg).await?;
    let derived = fetch_derived_era_floor(pg).await?;
    Ok(clamped_era_floor(persisted, derived))
}

async fn fetch_in_era_checkin_keys(
    pg: impl sqlx::PgExecutor<'_>,
    floor: NaiveDateTime,
) -> Result<HashSet<String>, sqlx::Error> {
    let rows: Vec<String> =
        sqlx::query_scalar(IN_ERA_CHECKIN_KEYS_SQL).bind(floor).fetch_all(pg).await?;
    Ok(rows.into_iter().collect())
}

async fn fetch_mirrored_legacy_ids(
    pg: impl sqlx::PgExecutor<'_>,
) -> Result<HashSet<i32>, sqlx::Error> {
    let rows: Vec<i32> = sqlx::query_scalar(CANONICAL_MIRRORED_IDS_SQL).fetch_all(pg).await?;
    Ok(rows.into_iter().collect())
}

/// Idempotency pre-check as a named, directly-testable function (rather than
/// left inline in `process_candidate`) — the SAME conflict target the
/// mapper's own `UPSERT_COMPANION_SQL` resolves on (`guest_legacy_id`
/// UNIQUE). See tests below.
async fn fetch_existing_mirrored_guest_id(
    pg: impl sqlx::PgExecutor<'_>,
    legacy_id: i32,
) -> Result<Option<i32>, sqlx::Error> {
    sqlx::query_scalar(EXISTING_MIRRORED_PROBE_SQL).bind(legacy_id).fetch_optional(pg).await
}

/// Gap-2 hardening — fetch the resolved parent's `cin_checkin_time` purely
/// to power the out-of-era WARNING for explicit `Cin_no` overrides. Only
/// called when `explicit_scope` is true in `process_candidate`; never on the
/// default floored path.
async fn fetch_parent_checkin_time(
    pg: impl sqlx::PgExecutor<'_>,
    cin_id: i32,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    sqlx::query_scalar(PARENT_CHECKIN_TIME_SQL).bind(cin_id).fetch_optional(pg).await
}

/// Gap-2 hardening — pure decision: should we WARN that an explicitly-
/// passed `Cin_no` resolved to a parent check-in whose `cin_checkin_time`
/// sits OUTSIDE the `guest_registry` era floor? `true` means "print the
/// warning". Boundary matches `IN_ERA_CHECKIN_KEYS_SQL`'s own `>=` — a
/// check-in exactly AT the floor is in-era, not outside it. This is a
/// warning decision only; it never blocks the override (module docs point
/// 1) and never changes the default floored path's behaviour.
fn resolved_parent_is_outside_era_floor(checkin_time: NaiveDateTime, floor: NaiveDateTime) -> bool {
    checkin_time < floor
}

/// Full legacy work-list. READ-ONLY — routed through
/// `simple_query_with_timeout_pooled` (`MssqlOpKind::Read`) per the
/// standing guardrail (issue #275 / #279): a raw, unwrapped tiberius query
/// gets neither the per-op timeout nor the poison-on-timeout flag. No bind
/// params needed — the scan is unfiltered; scoping happens in memory.
async fn fetch_legacy_companions(
    mssql: &DbPool,
) -> Result<Vec<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    let rows =
        simple_query_with_timeout_pooled(&mut conn, LEGACY_SCAN_SQL, MssqlOpKind::Read).await?;
    Ok(rows)
}

/// Minimal legacy-row projection: just the fields needed to scope/diff and
/// print a human-readable line. The real, authoritative projection stays
/// inside `GuestRegistryMapper::apply` (never duplicated here) — mirrors
/// the `ReceiptPreview` / `CandidatePreview` idiom in the sibling bins.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LegacyCompanionRow {
    legacy_id: i32,
    cin_no: Option<String>,
    cin_name: Option<String>,
    cin_country: Option<String>,
}

fn extract_legacy_row(row: &dyn MappableRow) -> Result<LegacyCompanionRow, SyncError> {
    let legacy_id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
        table: TABLE,
        message: "id NULL — IDENTITY column should never be NULL post-migration 022".into(),
    })?;
    let cin_no = row.try_get_str("Cin_no")?.map(str::to_string);
    let cin_name = row.try_get_str("Cin_name")?.map(str::to_string);
    let cin_country = row.try_get_str("Cin_contry")?.map(str::to_string);
    Ok(LegacyCompanionRow { legacy_id, cin_no, cin_name, cin_country })
}

/// Scoping verdict for one legacy companion row — pure, so the duplicate-
/// twin case (`CH26-001524`) and the orphan-row case are directly
/// unit-testable without a database.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScopeVerdict {
    /// `Cin_no` NULL/empty (an orphan row the mapper itself would skip
    /// with a warning), or not in the in-scope key set.
    OutOfScope,
    /// In scope, but `id` already carries a canonical companion —
    /// already mirrored, a true no-op.
    AlreadyMirrored,
    /// In scope and `id` absent from canonical — a backfill candidate.
    Candidate,
}

fn classify(
    row: &LegacyCompanionRow,
    in_scope: &HashSet<String>,
    mirrored_ids: &HashSet<i32>,
) -> ScopeVerdict {
    let Some(cin_no) = row.cin_no.as_deref().filter(|s| !s.is_empty()) else {
        return ScopeVerdict::OutOfScope;
    };
    if !in_scope.contains(cin_no) {
        return ScopeVerdict::OutOfScope;
    }
    if mirrored_ids.contains(&row.legacy_id) {
        return ScopeVerdict::AlreadyMirrored;
    }
    ScopeVerdict::Candidate
}

/// Process one candidate end-to-end: resolve parent, idempotency-check,
/// then either report (dry-run) or re-drive the mapper (live). Every
/// branch updates `s` and prints a one-line report.
///
/// `explicit_scope` / `era_floor_for_warning` power the Gap-2 out-of-era
/// WARNING only: when `explicit_scope` is true and the resolved parent's
/// `cin_checkin_time` is outside `era_floor_for_warning`, this prints a
/// non-fatal warning (never a failure) — see
/// [`resolved_parent_is_outside_era_floor`]. Both are no-ops on the default
/// floored path (`explicit_scope == false`).
async fn process_candidate(
    pg: &PgPool,
    row: &tiberius::Row,
    preview: &LegacyCompanionRow,
    dry_run: bool,
    explicit_scope: bool,
    era_floor_for_warning: Option<NaiveDateTime>,
    s: &mut Summary,
) {
    // Guaranteed Some by `classify` admitting this row into `candidates`.
    let cin_no = preview.cin_no.as_deref().unwrap_or_default();
    let name = preview.cin_name.as_deref().unwrap_or_default();
    let country = preview.cin_country.as_deref().unwrap_or_default();

    let mut tx = match pg.begin().await {
        Ok(t) => t,
        Err(err) => {
            tracing::warn!(legacy_id = preview.legacy_id, error = %err, "pg.begin failed — skipping");
            println!("[ERROR] legacy_id={}: pg.begin failed: {err}", preview.legacy_id);
            s.errored += 1;
            return;
        }
    };

    // Resolve the parent check-in ourselves (same lookup
    // GuestRegistryMapper::apply performs internally) purely so we can
    // report the resolved id and make an explicit skip decision — never a
    // run failure — rather than let the mapper's own loud Err (correct for
    // the live CT watcher, which must hold its watermark) bubble up here.
    let parent = match resolve::resolve_checkin_id(&mut tx, Some(cin_no)).await {
        Ok(v) => v,
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(
                legacy_id = preview.legacy_id, cin_no, error = %err,
                "resolve_checkin_id failed — skipping"
            );
            println!(
                "[ERROR] legacy_id={} cin_no={cin_no}: resolve_checkin_id failed: {err}",
                preview.legacy_id
            );
            s.errored += 1;
            return;
        }
    };
    let Some((cin_id, _agg)) = parent else {
        let _ = tx.rollback().await;
        tracing::warn!(
            legacy_id = preview.legacy_id, cin_no,
            "parent check-in unresolvable in canonical ht_checkins — skipping. Per the \
             investigation all 30 known parent folios exist canonically; if this fires \
             the parent check-in itself still needs backfilling \
             (bin/backfill_legacy_checkins) first."
        );
        println!(
            "[SKIP] legacy_id={} cin_no={cin_no}: parent check-in unresolvable in \
             canonical ht_checkins",
            preview.legacy_id
        );
        s.skipped_unresolvable_parent += 1;
        return;
    };

    // Gap-2 hardening: explicit Cin_no overrides REPLACE the era-floored
    // scope (module docs point 1) — legitimate, kept fully functional. But
    // a row written outside the floor is permanently invisible to the
    // guest_registry reconcile arm's own floor-gated auto-close, so warn
    // loudly (never fail) when that's about to happen.
    if explicit_scope {
        if let Some(floor) = era_floor_for_warning {
            match fetch_parent_checkin_time(&mut *tx, cin_id).await {
                Ok(Some(checkin_time))
                    if resolved_parent_is_outside_era_floor(checkin_time, floor) =>
                {
                    tracing::warn!(
                        legacy_id = preview.legacy_id, cin_no, %checkin_time, %floor,
                        "explicit Cin_no resolves outside the guest_registry era floor — \
                         the reconcile arm will never auto-close this row"
                    );
                    println!(
                        "[WARNING] legacy_id={} cin_no={cin_no}: parent check-in {checkin_time} \
                         is OUTSIDE the guest_registry era floor ({floor}) — this row WILL be \
                         written, but the guest_registry reconcile arm's own scope is \
                         floor-gated, so it will NEVER auto-close via the normal re-hash path. \
                         Confirm this explicit Cin_no override is intentional.",
                        preview.legacy_id
                    );
                }
                Ok(_) => {}
                Err(err) => {
                    tracing::warn!(
                        legacy_id = preview.legacy_id, cin_no, error = %err,
                        "failed to read parent cin_checkin_time for the era-floor warning \
                         check — continuing without it (non-fatal, does not block the write)"
                    );
                }
            }
        }
    }

    // Idempotency pre-check — the SAME conflict target the mapper's own
    // UPSERT resolves on. A hit means a concurrent write (or a stale
    // in-memory diff snapshot) already landed this companion: true no-op.
    let existing: Result<Option<i32>, sqlx::Error> =
        fetch_existing_mirrored_guest_id(&mut *tx, preview.legacy_id).await;
    let existing = match existing {
        Ok(v) => v,
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(
                legacy_id = preview.legacy_id, error = %err,
                "existing-row probe failed — skipping"
            );
            println!(
                "[ERROR] legacy_id={}: existing-row probe failed: {err}",
                preview.legacy_id
            );
            s.errored += 1;
            return;
        }
    };
    if let Some(guest_id) = existing {
        let _ = tx.rollback().await;
        tracing::info!(
            legacy_id = preview.legacy_id, guest_id,
            "already present in ht_guest_registry — no-op"
        );
        println!(
            "[SKIP] legacy_id={} cin_no={cin_no}: already present (guest_id={guest_id}) — no-op",
            preview.legacy_id
        );
        s.skipped_already_present += 1;
        return;
    }

    if dry_run {
        let _ = tx.rollback().await;
        println!(
            "[DRY RUN] would write legacy_id={} cin_no={cin_no} name={name:?} \
             country={country:?} canonical_checkin_id={cin_id}",
            preview.legacy_id
        );
        s.written += 1;
        return;
    }

    match GuestRegistryMapper.apply(&mut tx, ChangeOp::Insert, Some(row)).await {
        Ok(_event) => match tx.commit().await {
            Ok(()) => {
                tracing::info!(
                    legacy_id = preview.legacy_id, cin_no, canonical_checkin_id = cin_id,
                    "written to ht_guest_registry"
                );
                println!(
                    "[WRITTEN] legacy_id={} cin_no={cin_no} name={name:?} country={country:?} \
                     canonical_checkin_id={cin_id}",
                    preview.legacy_id
                );
                s.written += 1;
            }
            Err(err) => {
                tracing::warn!(legacy_id = preview.legacy_id, error = %err, "commit failed");
                println!("[ERROR] legacy_id={}: commit failed: {err}", preview.legacy_id);
                s.errored += 1;
            }
        },
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(
                legacy_id = preview.legacy_id, cin_no, error = %err,
                "GuestRegistryMapper::apply failed"
            );
            println!(
                "[ERROR] legacy_id={} cin_no={cin_no}: GuestRegistryMapper::apply failed: {err}",
                preview.legacy_id
            );
            s.errored += 1;
        }
    }
}

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new().max_connections(PG_POOL_MAX).connect(&url).await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Connected to PostgreSQL");
    Ok(pool)
}

async fn connect_legacy() -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    let mut config = DbConfig::from_env();
    config.pool_max = MSSQL_POOL_MAX;
    let server = config.server.clone();
    let pool = create_pool(&config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    {
        // Even the startup liveness probe routes through the timeout+poison
        // wrapper — "ALL legacy MSSQL reads" per the standing guardrail,
        // no carve-out for a one-off SELECT 1.
        let mut conn = pool.get().await?;
        let _ = simple_query_with_timeout_pooled(&mut conn, "SELECT 1", MssqlOpKind::Read).await?;
    }
    tracing::info!(server = %server, port = config.port, "Connected to legacy MSSQL (read-only)");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    //! Pure-logic + string-content unit tests need no database. The
    //! DB-backed tests (`fetch_persisted_era_floor_*`,
    //! `fetch_derived_era_floor_*`, `fetch_in_era_checkin_keys_*`,
    //! `fetch_mirrored_legacy_ids_*`, `fetch_existing_mirrored_guest_id_*`,
    //! `fetch_parent_checkin_time_*`, `duplicate_twin_folio_classifies_*`)
    //! all self-skip without `DATABASE_URL` — `cargo test --lib`-equivalent
    //! bin tests must stay green on a machine with no database. Each opens
    //! its own connection + TEMP-table-shadowed transaction and rolls back,
    //! mirroring the `temp_fixture_conn` / `TEMP_FIXTURE_DDL` idiom in
    //! `backfill_room_calendar` exactly — this file's five ad-hoc SQL
    //! statements (`PERSISTED_ERA_FLOOR_SQL`, `DERIVED_ERA_FLOOR_SQL`,
    //! `IN_ERA_CHECKIN_KEYS_SQL`, `CANONICAL_MIRRORED_IDS_SQL`,
    //! `EXISTING_MIRRORED_PROBE_SQL`) plus the Gap-2 addition
    //! (`PARENT_CHECKIN_TIME_SQL`) are runtime `sqlx::query()` calls with no
    //! compile-time check, so a real-schema round-trip is the only thing
    //! that actually validates them.

    use super::*;
    use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

    // -------------------------------------------------------------------
    // Source-scan pin — issue #275 / #279. This bin's only MSSQL reads
    // (`fetch_legacy_companions`, the `connect_legacy` liveness probe) must
    // route through `simple_query_with_timeout_pooled`, never a raw
    // unwrapped tiberius call. `scheduler/mod.rs`'s equivalent pin
    // deliberately excludes `bin/*` (operator-invoked, no scheduled
    // exposure) — this is this bin's own guard so a future edit can't
    // silently reintroduce the raw call.
    //
    // Scanned region is PRODUCTION code only (everything before this
    // `#[cfg(test)]` module) — `include_str!` reads this whole file,
    // itself included, so scanning past this point would trip on the
    // needle text these very assertions must contain to name the pattern
    // they're checking for.
    // -------------------------------------------------------------------
    fn production_source() -> &'static str {
        let src = include_str!("backfill_guest_registry_companions.rs");
        let boundary = src.find("#[cfg(test)]").expect("test module marker must exist");
        &src[..boundary]
    }

    #[test]
    fn bin_contains_no_raw_mssql_bypass_calls() {
        for needle in [".simple_query(", ".query(&mut", ".execute(&mut"] {
            assert!(
                !production_source().contains(needle),
                "backfill_guest_registry_companions.rs calls `{needle}` directly — route \
                 it through simple_query_with_timeout_pooled (db::mssql_timeout) instead \
                 (issue #275 / #279)"
            );
        }
    }

    #[test]
    fn bin_imports_the_timeout_wrapper() {
        assert!(
            production_source().contains("simple_query_with_timeout_pooled"),
            "backfill_guest_registry_companions.rs no longer uses \
             simple_query_with_timeout_pooled — if it stopped reading MSSQL entirely \
             this test should be removed, not left stale"
        );
    }

    // -------------------------------------------------------------------
    // parse_args
    // -------------------------------------------------------------------
    #[test]
    fn parse_args_recognises_dry_run_flag() {
        let (dry_run, explicit) = parse_args(vec!["--dry-run".to_string()]);
        assert!(dry_run);
        assert!(explicit.is_empty());
    }

    #[test]
    fn parse_args_collects_positional_cin_nos() {
        let (dry_run, explicit) =
            parse_args(vec!["CH26-001524".to_string(), "CH26-000999".to_string()]);
        assert!(!dry_run);
        assert_eq!(explicit, vec!["CH26-001524", "CH26-000999"]);
    }

    #[test]
    fn parse_args_mixes_flags_and_positional_args() {
        let (dry_run, explicit) =
            parse_args(vec!["--dry-run".to_string(), "CH26-001524".to_string()]);
        assert!(dry_run);
        assert_eq!(explicit, vec!["CH26-001524"]);
    }

    #[test]
    fn parse_args_ignores_unknown_flags() {
        let (dry_run, explicit) = parse_args(vec!["--bogus-flag".to_string()]);
        assert!(!dry_run);
        assert!(explicit.is_empty());
    }

    #[test]
    fn parse_args_empty_input_defaults_live_and_empty() {
        let (dry_run, explicit) = parse_args(Vec::<String>::new());
        assert!(!dry_run);
        assert!(explicit.is_empty());
    }

    // -------------------------------------------------------------------
    // clamped_era_floor — byte-identical rule to
    // scheduler::sync::clamped_era_floor.
    // -------------------------------------------------------------------
    fn dt(y: i32, m: u32, d: u32) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }

    #[test]
    fn clamp_takes_the_later_of_both_when_both_present() {
        let persisted = dt(2026, 5, 13);
        let derived = dt(2023, 1, 1); // dragged backwards by one old edit
        assert_eq!(clamped_era_floor(Some(persisted), Some(derived)), Some(persisted));
    }

    #[test]
    fn clamp_prefers_a_later_derived_over_an_older_persisted() {
        let persisted = dt(2026, 5, 13);
        let derived = dt(2026, 7, 1);
        assert_eq!(clamped_era_floor(Some(persisted), Some(derived)), Some(derived));
    }

    #[test]
    fn clamp_keeps_the_watermark_when_derived_goes_none() {
        let persisted = dt(2026, 5, 13);
        assert_eq!(clamped_era_floor(Some(persisted), None), Some(persisted));
    }

    #[test]
    fn clamp_is_none_when_neither_source_has_coverage() {
        assert_eq!(clamped_era_floor(None, None), None);
    }

    #[test]
    fn clamp_falls_back_to_derived_when_nothing_persisted_yet() {
        let derived = dt(2026, 5, 13);
        assert_eq!(clamped_era_floor(None, Some(derived)), Some(derived));
    }

    // -------------------------------------------------------------------
    // extract_legacy_row — pure projection, testable via HashMapRow since
    // `tiberius::Row`'s constructors are private (sync/row.rs docs).
    // -------------------------------------------------------------------
    fn make_row(id: i32, cin_no: Option<&str>, name: Option<&str>, country: Option<&str>) -> HashMapRow {
        let mut row = HashMapRow::new(TABLE).with("id", MockValue::I32(id));
        row = row.with(
            "Cin_no",
            cin_no.map(|s| MockValue::Str(s.into())).unwrap_or(MockValue::Null),
        );
        row = row.with(
            "Cin_name",
            name.map(|s| MockValue::Str(s.into())).unwrap_or(MockValue::Null),
        );
        row = row.with(
            "Cin_contry",
            country.map(|s| MockValue::Str(s.into())).unwrap_or(MockValue::Null),
        );
        row
    }

    #[test]
    fn extract_legacy_row_reads_all_four_fields() {
        let row = make_row(101, Some("CH26-001524"), Some("Somsri Kaew"), Some("TH"));
        let p = extract_legacy_row(&row).expect("extract must succeed");
        assert_eq!(p.legacy_id, 101);
        assert_eq!(p.cin_no.as_deref(), Some("CH26-001524"));
        assert_eq!(p.cin_name.as_deref(), Some("Somsri Kaew"));
        assert_eq!(p.cin_country.as_deref(), Some("TH"));
    }

    #[test]
    fn extract_legacy_row_errors_when_id_is_null() {
        let row = make_row(0, Some("CH26-001524"), None, None).with("id", MockValue::Null);
        let err = extract_legacy_row(&row).expect_err("NULL id must error");
        assert!(err.to_string().contains("id"));
    }

    #[test]
    fn extract_legacy_row_accepts_null_name_and_country() {
        let row = make_row(101, Some("CH26-001524"), None, None);
        let p = extract_legacy_row(&row).expect("extract must succeed");
        assert_eq!(p.cin_name, None);
        assert_eq!(p.cin_country, None);
    }

    // -------------------------------------------------------------------
    // classify — the scoping/diff decision. Pins the duplicate-twin case
    // (CH26-001524) and the orphan-row case explicitly.
    // -------------------------------------------------------------------
    #[test]
    fn classify_out_of_scope_when_cin_no_is_null() {
        let row = LegacyCompanionRow {
            legacy_id: 1,
            cin_no: None,
            cin_name: None,
            cin_country: None,
        };
        let in_scope: HashSet<String> = HashSet::new();
        let mirrored: HashSet<i32> = HashSet::new();
        assert_eq!(classify(&row, &in_scope, &mirrored), ScopeVerdict::OutOfScope);
    }

    #[test]
    fn classify_out_of_scope_when_cin_no_is_empty_string() {
        let row = LegacyCompanionRow {
            legacy_id: 1,
            cin_no: Some(String::new()),
            cin_name: None,
            cin_country: None,
        };
        let in_scope: HashSet<String> = ["CH26-001524".to_string()].into_iter().collect();
        let mirrored: HashSet<i32> = HashSet::new();
        assert_eq!(classify(&row, &in_scope, &mirrored), ScopeVerdict::OutOfScope);
    }

    #[test]
    fn classify_out_of_scope_when_cin_no_not_in_era() {
        let row = LegacyCompanionRow {
            legacy_id: 1,
            cin_no: Some("CH26-999999".to_string()),
            cin_name: None,
            cin_country: None,
        };
        let in_scope: HashSet<String> = ["CH26-001524".to_string()].into_iter().collect();
        let mirrored: HashSet<i32> = HashSet::new();
        assert_eq!(classify(&row, &in_scope, &mirrored), ScopeVerdict::OutOfScope);
    }

    #[test]
    fn classify_candidate_when_in_scope_and_not_mirrored() {
        let row = LegacyCompanionRow {
            legacy_id: 42,
            cin_no: Some("CH26-001524".to_string()),
            cin_name: Some("Somsri Kaew".to_string()),
            cin_country: None,
        };
        let in_scope: HashSet<String> = ["CH26-001524".to_string()].into_iter().collect();
        let mirrored: HashSet<i32> = HashSet::new();
        assert_eq!(classify(&row, &in_scope, &mirrored), ScopeVerdict::Candidate);
    }

    #[test]
    fn classify_already_mirrored_when_id_already_canonical() {
        let row = LegacyCompanionRow {
            legacy_id: 42,
            cin_no: Some("CH26-001524".to_string()),
            cin_name: Some("Somsri Kaew".to_string()),
            cin_country: None,
        };
        let in_scope: HashSet<String> = ["CH26-001524".to_string()].into_iter().collect();
        let mirrored: HashSet<i32> = [42].into_iter().collect();
        assert_eq!(classify(&row, &in_scope, &mirrored), ScopeVerdict::AlreadyMirrored);
    }

    /// The `CH26-001524` shape from the live investigation: two legacy
    /// rows sharing one `Cin_no`, duplicate name/blank country, one
    /// (`id=100`) already mirrored and one (`id=101`) missing. Per-row `id`
    /// diffing must surface exactly the missing twin.
    #[test]
    fn duplicate_twin_folio_surfaces_only_the_missing_row() {
        let mirrored_twin = LegacyCompanionRow {
            legacy_id: 100,
            cin_no: Some("CH26-001524".to_string()),
            cin_name: Some("Somsri Kaew".to_string()),
            cin_country: Some(String::new()),
        };
        let missing_twin = LegacyCompanionRow {
            legacy_id: 101,
            cin_no: Some("CH26-001524".to_string()),
            cin_name: Some("Somsri Kaew".to_string()),
            cin_country: Some(String::new()),
        };
        let in_scope: HashSet<String> = ["CH26-001524".to_string()].into_iter().collect();
        let mirrored: HashSet<i32> = [100].into_iter().collect();

        assert_eq!(
            classify(&mirrored_twin, &in_scope, &mirrored),
            ScopeVerdict::AlreadyMirrored,
            "the already-mirrored twin must be a no-op, not re-flagged"
        );
        assert_eq!(
            classify(&missing_twin, &in_scope, &mirrored),
            ScopeVerdict::Candidate,
            "the missing twin must still surface despite the duplicate name/country"
        );
    }

    // -------------------------------------------------------------------
    // SQL shape guards (no database needed).
    // -------------------------------------------------------------------
    #[test]
    fn legacy_scan_sql_projects_the_four_columns_and_preserves_the_typo() {
        assert!(LEGACY_SCAN_SQL.contains("id"));
        assert!(LEGACY_SCAN_SQL.contains("Cin_no"));
        assert!(LEGACY_SCAN_SQL.contains("Cin_name"));
        assert!(LEGACY_SCAN_SQL.contains("Cin_contry"));
        assert!(
            !LEGACY_SCAN_SQL.contains("Cin_country"),
            "must NOT silently 'fix' the legacy spelling"
        );
        assert!(LEGACY_SCAN_SQL.contains("FROM HT_CheckIn_Other_People"));
    }

    #[test]
    fn canonical_mirrored_ids_sql_filters_null_legacy_ids() {
        assert!(CANONICAL_MIRRORED_IDS_SQL.contains("guest_legacy_id IS NOT NULL"));
        assert!(CANONICAL_MIRRORED_IDS_SQL.contains("FROM ht_guest_registry"));
    }

    #[test]
    fn in_era_checkin_keys_sql_matches_the_scheduler_arm_basis() {
        assert!(IN_ERA_CHECKIN_KEYS_SQL.contains("FROM ht_checkins"));
        assert!(IN_ERA_CHECKIN_KEYS_SQL.contains("legacy_cin_no IS NOT NULL"));
        assert!(IN_ERA_CHECKIN_KEYS_SQL.contains("cin_checkin_time >= $1"));
    }

    #[test]
    fn derived_era_floor_sql_matches_the_scheduler_arm_basis() {
        assert!(DERIVED_ERA_FLOOR_SQL.contains("date_trunc('day', MIN(ht_checkins.cin_checkin_time))"));
        assert!(DERIVED_ERA_FLOOR_SQL.contains("guest_is_primary"));
        assert!(DERIVED_ERA_FLOOR_SQL.contains("guest_legacy_id IS NOT NULL"));
    }

    #[test]
    fn persisted_era_floor_sql_targets_the_guest_registry_key() {
        assert!(PERSISTED_ERA_FLOOR_SQL.contains("FROM ht_reconcile_era_floor"));
        assert!(PERSISTED_ERA_FLOOR_SQL.contains("table_name = $1"));
    }

    #[test]
    fn era_floor_key_is_guest_registry() {
        assert_eq!(ERA_FLOOR_KEY, "guest_registry");
    }

    #[test]
    fn existing_mirrored_probe_sql_keys_on_guest_legacy_id() {
        assert!(EXISTING_MIRRORED_PROBE_SQL.contains("guest_legacy_id = $1"));
        assert!(EXISTING_MIRRORED_PROBE_SQL.contains("FROM ht_guest_registry"));
    }

    #[test]
    fn parent_checkin_time_sql_targets_cin_id() {
        assert!(PARENT_CHECKIN_TIME_SQL.contains("cin_checkin_time"));
        assert!(PARENT_CHECKIN_TIME_SQL.contains("FROM ht_checkins"));
        assert!(PARENT_CHECKIN_TIME_SQL.contains("cin_id = $1"));
    }

    // -------------------------------------------------------------------
    // resolved_parent_is_outside_era_floor — Gap-2 warning decision, pure.
    // Boundary must match IN_ERA_CHECKIN_KEYS_SQL's own `>=`.
    // -------------------------------------------------------------------
    #[test]
    fn outside_era_floor_true_when_checkin_strictly_before_floor() {
        assert!(resolved_parent_is_outside_era_floor(dt(2020, 1, 1), dt(2026, 5, 13)));
    }

    #[test]
    fn outside_era_floor_false_when_checkin_exactly_at_floor() {
        // The floor itself is IN era (>=), matching IN_ERA_CHECKIN_KEYS_SQL
        // — must not warn on a boundary row.
        assert!(!resolved_parent_is_outside_era_floor(dt(2026, 5, 13), dt(2026, 5, 13)));
    }

    #[test]
    fn outside_era_floor_false_when_checkin_after_floor() {
        assert!(!resolved_parent_is_outside_era_floor(dt(2026, 7, 1), dt(2026, 5, 13)));
    }

    // -------------------------------------------------------------------
    // DB-backed tests — TEMP-table-shadowed fixtures, self-skipping
    // without a reachable DATABASE_URL. Mirrors `temp_fixture_conn` /
    // `TEMP_FIXTURE_DDL` in `backfill_room_calendar` exactly: each test
    // opens its own connection, begins a transaction, creates simplified
    // TEMP tables (only the columns these queries actually read) under
    // `SET LOCAL search_path = pg_temp, public` so they shadow the real
    // tables for the duration of the transaction, seeds rows, asserts,
    // then rolls back — production data can never be touched.
    // -------------------------------------------------------------------
    async fn temp_fixture_conn() -> Option<sqlx::PgConnection> {
        use sqlx::Connection;
        let url = std::env::var("DATABASE_URL").ok()?;
        match sqlx::PgConnection::connect(&url).await {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!(
                    "backfill_guest_registry_companions temp-fixture probe SKIPPED — cannot \
                     connect to PG: {e}"
                );
                None
            }
        }
    }

    /// Simplified schema — only the columns `PERSISTED_ERA_FLOOR_SQL`,
    /// `DERIVED_ERA_FLOOR_SQL`, `IN_ERA_CHECKIN_KEYS_SQL`,
    /// `CANONICAL_MIRRORED_IDS_SQL`, `EXISTING_MIRRORED_PROBE_SQL`, and
    /// `PARENT_CHECKIN_TIME_SQL` actually read. No FKs — this bin never
    /// relies on referential integrity, only column shapes.
    const TEMP_FIXTURE_DDL: &str = "\
        CREATE TEMP TABLE ht_checkins ( \
            cin_id INTEGER PRIMARY KEY, \
            legacy_cin_no VARCHAR(20), \
            cin_checkin_time TIMESTAMP NOT NULL \
        ) ON COMMIT DROP; \
        CREATE TEMP TABLE ht_guest_registry ( \
            guest_id SERIAL PRIMARY KEY, \
            guest_cin_id INTEGER NOT NULL, \
            guest_is_primary BOOLEAN DEFAULT false, \
            guest_legacy_id INTEGER \
        ) ON COMMIT DROP; \
        CREATE TEMP TABLE ht_reconcile_era_floor ( \
            table_name VARCHAR(50) PRIMARY KEY, \
            era_floor TIMESTAMP NOT NULL \
        ) ON COMMIT DROP;";

    async fn begin_fixture_tx(conn: &mut sqlx::PgConnection) -> sqlx::Transaction<'_, sqlx::Postgres> {
        use sqlx::Connection;
        let mut tx = conn.begin().await.expect("begin");
        sqlx::raw_sql(TEMP_FIXTURE_DDL)
            .execute(&mut *tx)
            .await
            .expect("create temp fixture tables");
        sqlx::query("SET LOCAL search_path = pg_temp, public")
            .execute(&mut *tx)
            .await
            .expect("set search_path");
        tx
    }

    #[tokio::test]
    async fn fetch_persisted_era_floor_reads_the_seeded_row_for_its_key() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_reconcile_era_floor (table_name, era_floor) VALUES \
             ('guest_registry', '2026-05-13 00:00:00'), \
             ('payments', '2020-01-01 00:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed era floor rows");

        let floor = fetch_persisted_era_floor(&mut *tx).await.expect("fetch_persisted_era_floor");
        assert_eq!(floor, Some(dt(2026, 5, 13)));

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_persisted_era_floor_is_none_when_key_absent() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_reconcile_era_floor (table_name, era_floor) \
             VALUES ('payments', '2020-01-01 00:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed unrelated row");

        let floor = fetch_persisted_era_floor(&mut *tx).await.expect("fetch_persisted_era_floor");
        assert_eq!(floor, None, "no 'guest_registry' row seeded — must be None, not the payments row");

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_derived_era_floor_computes_min_checkin_time_for_mirrored_non_primary_companions() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_checkins (cin_id, legacy_cin_no, cin_checkin_time) VALUES \
             (1, 'CH26-000001', '2026-05-15 09:00:00'), \
             (2, 'CH26-000002', '2020-01-01 09:00:00'), \
             (3, 'CH26-000003', '2019-01-01 09:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed checkins");

        // guest 1: non-primary, mirrored (guest_legacy_id set) — counts,
        // cin_id=1 (2026-05-15, the LATEST of the three — proves the query
        // isn't accidentally picking the earliest checkin overall).
        // guest 2: non-primary but NOT mirrored (guest_legacy_id NULL) —
        // must be excluded even though its checkin (2020) is earlier.
        // guest 3: primary AND mirrored — must be excluded despite being
        // the earliest checkin (2019); guest_is_primary disqualifies it.
        sqlx::query(
            "INSERT INTO ht_guest_registry (guest_cin_id, guest_is_primary, guest_legacy_id) \
             VALUES (1, false, 100), (2, false, NULL), (3, true, 200)",
        )
        .execute(&mut *tx)
        .await
        .expect("seed guest_registry");

        let floor = fetch_derived_era_floor(&mut *tx).await.expect("fetch_derived_era_floor");
        assert_eq!(
            floor,
            Some(dt(2026, 5, 15)),
            "must be the checkin time of the mirrored non-primary companion only"
        );

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_in_era_checkin_keys_filters_by_floor_inclusive_and_excludes_null() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_checkins (cin_id, legacy_cin_no, cin_checkin_time) VALUES \
             (1, 'CH26-001524', '2026-05-15 10:00:00'), \
             (2, 'CH26-000999', '2026-05-13 00:00:00'), \
             (3, 'CH26-000111', '2020-01-01 00:00:00'), \
             (4, NULL, '2026-06-01 00:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed checkins");

        let keys = fetch_in_era_checkin_keys(&mut *tx, dt(2026, 5, 13))
            .await
            .expect("fetch_in_era_checkin_keys");

        let expected: HashSet<String> =
            ["CH26-001524".to_string(), "CH26-000999".to_string()].into_iter().collect();
        assert_eq!(
            keys, expected,
            "must include the boundary row (>=) and exclude both the pre-floor and NULL rows"
        );

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_mirrored_legacy_ids_returns_only_non_null_ids() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_checkins (cin_id, legacy_cin_no, cin_checkin_time) \
             VALUES (1, 'CH26-001524', '2026-05-15 10:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed checkin");
        sqlx::query(
            "INSERT INTO ht_guest_registry (guest_cin_id, guest_is_primary, guest_legacy_id) \
             VALUES (1, false, 100), (1, false, 101), (1, false, NULL)",
        )
        .execute(&mut *tx)
        .await
        .expect("seed guest_registry");

        let ids = fetch_mirrored_legacy_ids(&mut *tx).await.expect("fetch_mirrored_legacy_ids");
        let expected: HashSet<i32> = [100, 101].into_iter().collect();
        assert_eq!(ids, expected);

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_existing_mirrored_guest_id_reports_present_and_absent() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_checkins (cin_id, legacy_cin_no, cin_checkin_time) \
             VALUES (1, 'CH26-001524', '2026-05-15 10:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed checkin");
        sqlx::query(
            "INSERT INTO ht_guest_registry (guest_cin_id, guest_is_primary, guest_legacy_id) \
             VALUES (1, false, 100)",
        )
        .execute(&mut *tx)
        .await
        .expect("seed mirrored companion");

        let present = fetch_existing_mirrored_guest_id(&mut *tx, 100)
            .await
            .expect("probe for a mirrored id must succeed");
        assert!(present.is_some(), "guest_legacy_id=100 was seeded — must report present");

        let absent = fetch_existing_mirrored_guest_id(&mut *tx, 999)
            .await
            .expect("probe for a missing id must succeed");
        assert_eq!(absent, None, "guest_legacy_id=999 was never seeded — must report absent");

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_parent_checkin_time_reads_the_seeded_row_and_is_none_when_absent() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_checkins (cin_id, legacy_cin_no, cin_checkin_time) \
             VALUES (1, 'CH26-001524', '2026-05-15 10:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed checkin");

        let found = fetch_parent_checkin_time(&mut *tx, 1)
            .await
            .expect("fetch_parent_checkin_time for a seeded cin_id must succeed");
        assert_eq!(
            found,
            Some(
                chrono::NaiveDate::from_ymd_opt(2026, 5, 15)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            )
        );

        let missing = fetch_parent_checkin_time(&mut *tx, 999)
            .await
            .expect("fetch_parent_checkin_time for an unseeded cin_id must succeed");
        assert_eq!(missing, None);

        tx.rollback().await.expect("rollback");
    }

    /// Requirement (c): the `CH26-001524` duplicate-twin shape end-to-end
    /// against the real schema — `fetch_mirrored_legacy_ids` (DB) feeding
    /// `classify` (pure) must surface exactly ONE candidate (the missing
    /// twin), not zero and not two.
    #[tokio::test]
    async fn duplicate_twin_folio_classifies_as_exactly_one_candidate_against_real_schema() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
        let mut tx = begin_fixture_tx(&mut conn).await;

        sqlx::query(
            "INSERT INTO ht_checkins (cin_id, legacy_cin_no, cin_checkin_time) \
             VALUES (1, 'CH26-001524', '2026-05-15 10:00:00')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed checkin");
        // Only the id=100 twin is already mirrored — id=101 is the missing
        // twin this bin exists to backfill.
        sqlx::query(
            "INSERT INTO ht_guest_registry (guest_cin_id, guest_is_primary, guest_legacy_id) \
             VALUES (1, false, 100)",
        )
        .execute(&mut *tx)
        .await
        .expect("seed the already-mirrored twin");

        let mirrored_ids =
            fetch_mirrored_legacy_ids(&mut *tx).await.expect("fetch_mirrored_legacy_ids");
        let in_scope: HashSet<String> = ["CH26-001524".to_string()].into_iter().collect();

        let mirrored_twin = LegacyCompanionRow {
            legacy_id: 100,
            cin_no: Some("CH26-001524".to_string()),
            cin_name: Some("Somsri Kaew".to_string()),
            cin_country: Some(String::new()),
        };
        let missing_twin = LegacyCompanionRow {
            legacy_id: 101,
            cin_no: Some("CH26-001524".to_string()),
            cin_name: Some("Somsri Kaew".to_string()),
            cin_country: Some(String::new()),
        };

        let verdicts = [
            classify(&mirrored_twin, &in_scope, &mirrored_ids),
            classify(&missing_twin, &in_scope, &mirrored_ids),
        ];
        assert_eq!(verdicts[0], ScopeVerdict::AlreadyMirrored);
        assert_eq!(verdicts[1], ScopeVerdict::Candidate);
        assert_eq!(
            verdicts.iter().filter(|v| **v == ScopeVerdict::Candidate).count(),
            1,
            "exactly one twin (the missing one) must classify as a Candidate"
        );

        tx.rollback().await.expect("rollback");
    }
}
