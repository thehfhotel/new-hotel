//! One-shot operator repair: re-point `ht_rooms_new.legacy_room_id_int` /
//! `legacy_room_no` at the legacy room they actually describe.
//!
//! ## Why this exists (wave-4 housekeeping stream, work item B0)
//!
//! HF Ville's legacy `HT_Rooms` interleaves `id` and `Room_no` — legacy `id=2`
//! is `Room_no='116'`, `id=3` is `'102'`, `id=21` is `'203'`. Canonical
//! `hotelville.ht_rooms_new` was populated as if the two ran in step, so **30 of
//! its 34 rooms carry a `legacy_room_no` that belongs to a different room**
//! (HF Hotel: 0 of 58 — its two orderings genuinely agree).
//!
//! Which column is truthful was settled against live data, not guessed:
//! `ht_checkin_rooms.cr_legacy_ds_id` → legacy `HT_CheckIn_Ds.cin_room_no`
//! matched canonical `room_no` for 8 of 8 in-house Ville stays and
//! `legacy_room_no` for 0 of 8. `sync::resolve::resolve_room_id` agrees — the
//! room-NUMBER path is correct; the id path is corrupt.
//!
//! Both directions are affected:
//!
//! 1. **Inbound (wrong TODAY).** `sync::mappers::room` resolves
//!    `WHERE legacy_room_id_int = $1 OR room_no = $2` ordered so the id match
//!    WINS, so Change Tracking writes `room_clean` / `room_maintenance` /
//!    `room_notes` / layout onto the wrong canonical Ville room. Observable
//!    live: legacy says Ville **203** is dirty, canonical says **205** is.
//! 2. **Outbound (latent).** `bin/writeback.rs` resolves `legacy_room_id_int`
//!    for `MarkRoomClean` / `MarkRoomDirty` / `SetRoomMaintenance` and passes it
//!    straight to the recipes, which key on `HT_Rooms.id`. The first Ville
//!    mark-clean/dirty would flip the WRONG iHOTEL room. `writeback_jobs` shows
//!    HF Ville has never run one — the landmine is unarmed, and shipping a
//!    Ville-capable `/hk` is exactly what would arm it.
//!
//! HF Ville therefore stays out of `HK_BRANCHES` until this bin has been run
//! with `--apply` and the result verified (`PENDING-VERIFICATIONS` V13).
//!
//! ## What it does
//!
//! Reads `(id, Room_no)` from legacy `HT_Rooms` — **READ-ONLY, no MSSQL writes
//! whatsoever** — and for each legacy row stamps that pair onto the canonical
//! room whose `room_no` equals the legacy `Room_no`:
//!
//! ```sql
//! UPDATE ht_rooms_new
//!    SET legacy_room_id_int = $1, legacy_room_no = $2, updated_at = NOW()
//!  WHERE room_no = $2
//!    AND (legacy_room_id_int IS DISTINCT FROM $1 OR legacy_room_no IS DISTINCT FROM $2)
//! ```
//!
//! The `IS DISTINCT FROM` guard makes it idempotent and makes a re-run (or a run
//! racing the live CT watcher) a no-op. The `legacy_room_no` half of the guard is
//! deliberate: the acceptance criterion for this repair is
//! `count(*) FILTER (WHERE room_no IS DISTINCT FROM legacy_room_no) = 0`, which a
//! guard on the id alone could not reach for a row whose id was already right and
//! whose number was not.
//!
//! ## What it deliberately does NOT touch
//!
//! * `room_id` — our primary key, and `aggregate_id` is DERIVED from it
//!   (`aggregate_uuid(AggregateKind::Room, room_id)`), so re-pointing it would
//!   rewrite every outbox/event identity for the room.
//! * `room_no` — the truthful column; that is the whole finding.
//! * `aggregate_id` — untouched for the same reason.
//! * Anything in legacy MSSQL.
//!
//! Only the two DENORMALISED pointers move, which is sufficient: the writeback
//! resolver reads `legacy_room_id_int`, never `room_id`.
//!
//! Safe as a bulk swap: live `pg_indexes` on `ht_rooms_new` are PK `room_id`,
//! UNIQUE `room_no`, partial UNIQUE `aggregate_id`, and a PLAIN index on
//! `legacy_room_no`. There is **no unique index on `legacy_room_id_int`**, so
//! two rows may transiently share one mid-transaction without tripping a
//! constraint.
//!
//! Rooms present on only one side are **reported, never guessed** — a legacy
//! `Room_no` with no canonical row, a canonical room with no legacy counterpart,
//! and (fatal to matching) duplicate legacy `Room_no` values are all listed for
//! the operator instead of being resolved by heuristic.
//!
//! ## Three operator guards (wave-4 review F1/F2/F3)
//!
//! 1. **Target binding.** The PG half (`DATABASE_URL`) and the MSSQL half
//!    (`DbConfig::from_env`) are wired from INDEPENDENT env vars, and
//!    `dotenvy` happily fills in an unset one from `hotel-backend/.env`. A
//!    `SITE_ID=hfville` run that inherited HF Hotel's `DATABASE_URL` would read
//!    Ville's legacy rooms and rewrite HF HOTEL's canonical pointers — and its
//!    report would look exactly like a correct run. So both resolved targets
//!    (`select current_database()` and `server:port/db`) are printed in the
//!    report header, and `--apply` is REFUSED when the PG database does not
//!    match the one `SITE_ID` implies (`hfhotel`→`hotelnew`,
//!    `hfville`→`hotelville`), or when the legacy DB name contradicts a site
//!    whose value is derivable from the deploy topology.
//! 2. **Partial plans abort.** `--apply` REFUSES to write when any room is
//!    unmatched on either side or ambiguous, because a canonical room the bin
//!    could not match KEEPS its stale `legacy_room_id_int` — and a stale
//!    pointer that now collides with a repaired one makes the inbound CT
//!    resolver (`sync/mappers/room.rs`, `legacy_room_id_int = $1 OR
//!    room_no = $2`, no tiebreak) non-deterministic. Pass `--allow-partial`
//!    only when the leftovers have been reviewed and are understood.
//! 3. **Built-in re-verify.** A successful `--apply` immediately re-reads BOTH
//!    sides and re-plans. The CT room mapper reads outside this bin's lock, so
//!    an in-flight tick can re-stamp a row microseconds after the repair
//!    commits; the residual plan is the only thing that proves it did not. A
//!    non-empty residual exits NON-ZERO — the fix is simply to re-run.
//!    Quieter still: stop the site's sync worker for the ~1 s the apply takes.
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//! # DRY RUN IS THE DEFAULT — it reads both databases and writes nothing.
//! SITE_ID=hfville DATABASE_URL=postgres://…/hotelville \
//!   DB_SERVER=… MSSQL_PORT=1436 DB_NAME=HOTEL DB_USER=sa DB_PASSWORD=… \
//!   cargo run --release --bin repair_room_legacy_keys
//!
//! # Live run — `--apply` is REQUIRED; `--dry-run` is accepted but redundant.
//! …  cargo run --release --bin repair_room_legacy_keys -- --apply
//!
//! # Live run over a plan that still has unmatched/ambiguous rooms — only
//! # after reading WHY they are unmatched in the dry-run report.
//! …  cargo run --release --bin repair_room_legacy_keys -- --apply --allow-partial
//! ```
//!
//! Exit status: `0` only when nothing was refused and (after `--apply`) the
//! re-verify found ZERO residual repairs. Any refusal or residual is non-zero.
//!
//! Run once per site. HF Hotel is the bin's own correctness proof: a dry run
//! there must report **0 rooms to repair**.
//!
//! ## Verification after `--apply` (HF Ville)
//!
//! The bin's own re-verify already asserts `Rooms still to repair: 0`. The
//! independent SQL check, for the runbook:
//!
//! ```sql
//! SELECT count(*) FILTER (WHERE room_no IS DISTINCT FROM legacy_room_no) AS mismatched,
//!        count(*) AS total
//!   FROM ht_rooms_new;                     -- mismatched must be 0
//! ```
//! then, within two sync ticks, the single dirty legacy room (`id=21`,
//! `Room_no='203'`) must show `room_clean = false` on canonical `room_no='203'`
//! and `'205'` must return to `true`.

use std::collections::{HashMap, HashSet};
use std::env;
use std::time::Instant;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::Row as _;

const PG_POOL_MAX: u32 = 4;

// =============================================================================
// Pure planning core (unit-tested below — no DB, no env, no I/O)
// =============================================================================

/// One legacy `HT_Rooms` row, as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyRoom {
    pub id: i32,
    pub room_no: String,
}

/// One canonical `ht_rooms_new` row, as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalRoom {
    pub room_id: i32,
    pub room_no: String,
    pub legacy_room_id_int: Option<i32>,
    pub legacy_room_no: Option<String>,
}

/// One room whose denormalised legacy pointers must move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repair {
    pub room_no: String,
    pub canonical_room_id: i32,
    pub before_legacy_id: Option<i32>,
    pub before_legacy_room_no: Option<String>,
    pub after_legacy_id: i32,
    pub after_legacy_room_no: String,
}

/// The full before/after picture, computed before anything is written so a dry
/// run and a live run report identically.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Plan {
    pub repairs: Vec<Repair>,
    /// Canonical rooms whose pointers already match the legacy row.
    pub already_correct: Vec<String>,
    /// Legacy `Room_no` values with no canonical room (reported, not created).
    pub legacy_without_canonical: Vec<LegacyRoom>,
    /// Canonical rooms with no legacy counterpart (reported, not cleared).
    pub canonical_without_legacy: Vec<String>,
    /// Legacy `Room_no` values appearing more than once — matching by number is
    /// ambiguous for these, so they are SKIPPED entirely.
    pub ambiguous_legacy_room_nos: Vec<String>,
    /// Legacy rows with a NULL / blank `Room_no` (unmatchable by definition).
    pub skipped_blank_legacy_room_no: u64,
}

impl Plan {
    pub fn is_noop(&self) -> bool {
        self.repairs.is_empty()
    }
}

/// Compute the repair plan. PURE.
///
/// Matching is by ROOM NUMBER, in the direction the live evidence supports:
/// canonical `room_no` is truthful, the legacy pointers are not. A legacy
/// `Room_no` that appears twice is ambiguous and is skipped rather than
/// resolved by a tiebreak nobody verified.
pub fn plan_repairs(legacy: &[LegacyRoom], canonical: &[CanonicalRoom]) -> Plan {
    let mut plan = Plan::default();

    // Legacy room numbers that appear more than once cannot be matched safely.
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for room in legacy {
        *seen.entry(room.room_no.as_str()).or_insert(0) += 1;
    }
    let ambiguous: HashSet<&str> = seen
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(no, _)| *no)
        .collect();
    plan.ambiguous_legacy_room_nos = {
        let mut v: Vec<String> = ambiguous.iter().map(|s| (*s).to_string()).collect();
        v.sort();
        v
    };

    let by_room_no: HashMap<&str, &CanonicalRoom> = canonical
        .iter()
        .map(|room| (room.room_no.as_str(), room))
        .collect();

    let mut matched: HashSet<&str> = HashSet::new();

    for legacy_room in legacy {
        if ambiguous.contains(legacy_room.room_no.as_str()) {
            continue;
        }
        let Some(target) = by_room_no.get(legacy_room.room_no.as_str()) else {
            plan.legacy_without_canonical.push(legacy_room.clone());
            continue;
        };
        matched.insert(target.room_no.as_str());

        let id_matches = target.legacy_room_id_int == Some(legacy_room.id);
        let no_matches = target.legacy_room_no.as_deref() == Some(legacy_room.room_no.as_str());
        if id_matches && no_matches {
            plan.already_correct.push(target.room_no.clone());
            continue;
        }

        plan.repairs.push(Repair {
            room_no: target.room_no.clone(),
            canonical_room_id: target.room_id,
            before_legacy_id: target.legacy_room_id_int,
            before_legacy_room_no: target.legacy_room_no.clone(),
            after_legacy_id: legacy_room.id,
            after_legacy_room_no: legacy_room.room_no.clone(),
        });
    }

    for room in canonical {
        if !matched.contains(room.room_no.as_str()) {
            plan.canonical_without_legacy.push(room.room_no.clone());
        }
    }

    plan.repairs.sort_by(|a, b| a.room_no.cmp(&b.room_no));
    plan.already_correct.sort();
    plan.canonical_without_legacy.sort();
    plan.legacy_without_canonical
        .sort_by(|a, b| a.room_no.cmp(&b.room_no));
    plan
}

// =============================================================================
// Operator guards (pure — unit-tested below)
// =============================================================================

/// The canonical PG database a site's app owns (ADR 0001's per-site logical
/// DB topology; `secrets.rs` reconstructs the URL from `NEW_DB_NAME`, which
/// docker-compose pins to `hotelville` for every hfville service).
pub fn expected_pg_database(site_id: &str) -> Option<&'static str> {
    match site_id {
        "hfhotel" => Some("hotelnew"),
        "hfville" => Some("hotelville"),
        _ => None,
    }
}

/// The legacy MSSQL database a site talks to, WHERE IT IS DERIVABLE. HF
/// Ville's is pinned in docker-compose (`DB_NAME=HOTEL` on every hfville
/// service); HF Hotel's comes from a repo secret (`DB_NAME: ${{ secrets.DB_NAME }}`),
/// so this bin cannot know it — it is printed for the operator, never enforced.
pub fn expected_mssql_database(site_id: &str) -> Option<&'static str> {
    match site_id {
        "hfville" => Some("HOTEL"),
        _ => None,
    }
}

/// Refuse to write when the resolved databases contradict `SITE_ID`.
///
/// The two halves of this bin are wired from independent env vars, so a
/// half-supplied environment (e.g. `SITE_ID`/`DB_*` exported for Ville but
/// `DATABASE_URL` inherited from `.env`) reads one site and writes the other,
/// producing a report indistinguishable from a correct run. MSSQL database
/// names are case-insensitive, so that half compares case-insensitively.
pub fn check_target_binding(
    site_id: &str,
    pg_database: &str,
    mssql_database: &str,
) -> Result<(), String> {
    let mut problems: Vec<String> = Vec::new();

    match expected_pg_database(site_id) {
        Some(expected) if expected != pg_database => problems.push(format!(
            "PostgreSQL database is {pg_database:?} but SITE_ID={site_id} implies {expected:?}"
        )),
        None => problems.push(format!(
            "SITE_ID={site_id:?} is not one of the known sites (hfhotel, hfville), \
             so the PostgreSQL target cannot be verified"
        )),
        _ => {}
    }

    if let Some(expected) = expected_mssql_database(site_id) {
        if !expected.eq_ignore_ascii_case(mssql_database) {
            problems.push(format!(
                "legacy MSSQL database is {mssql_database:?} but SITE_ID={site_id} implies \
                 {expected:?}"
            ));
        }
    }

    if problems.is_empty() {
        return Ok(());
    }
    Err(format!(
        "REFUSING to write: {}. The PG half (DATABASE_URL/NEW_DATABASE_URL) and the legacy \
         half (DB_SERVER/MSSQL_PORT/DB_NAME) are wired from INDEPENDENT env vars and dotenvy \
         fills in whichever one you left unset from hotel-backend/.env — a half-supplied \
         environment reads one site and rewrites the other. Export every variable for the \
         site you mean, then re-run.",
        problems.join("; ")
    ))
}

/// Refuse to write a plan that leaves rooms unmatched or ambiguous.
///
/// A canonical room the bin could not match KEEPS whatever
/// `legacy_room_id_int` it already had. If that stale value collides with an
/// id this run just assigned to another room, `sync/mappers/room.rs` resolves
/// `legacy_room_id_int = $1 OR room_no = $2` with both rows satisfying the id
/// predicate and NO tiebreak column — inbound `room_clean` / `room_maintenance`
/// / `room_notes` then land on whichever row PG happens to return, and the
/// `room_no IS DISTINCT FROM legacy_room_no` acceptance query does not catch it.
pub fn apply_block_reason(plan: &Plan, allow_partial: bool) -> Option<String> {
    if allow_partial {
        return None;
    }
    let mut gaps: Vec<String> = Vec::new();
    if !plan.legacy_without_canonical.is_empty() {
        gaps.push(format!(
            "{} legacy room(s) with no canonical row",
            plan.legacy_without_canonical.len()
        ));
    }
    if !plan.canonical_without_legacy.is_empty() {
        gaps.push(format!(
            "{} canonical room(s) with no legacy row (each KEEPS its current, possibly stale, \
             legacy_room_id_int)",
            plan.canonical_without_legacy.len()
        ));
    }
    if !plan.ambiguous_legacy_room_nos.is_empty() {
        gaps.push(format!(
            "{} duplicated legacy Room_no value(s)",
            plan.ambiguous_legacy_room_nos.len()
        ));
    }
    if gaps.is_empty() {
        return None;
    }
    Some(format!(
        "REFUSING to apply a PARTIAL plan: {}. An unmatched canonical room keeps its stale \
         pointer, which can collide with an id this run assigns elsewhere and make the inbound \
         CT room resolver non-deterministic. Fix the gaps in iHOTEL / run backfill_rooms, or \
         re-run with --allow-partial once you have read WHY each room is unmatched.",
        gaps.join("; ")
    ))
}

// =============================================================================
// Runner
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "repair_room_legacy_keys=info,hotel_backend=info".into()),
        )
        .init();

    // DRY RUN IS THE DEFAULT. Writing requires an explicit `--apply`; there is
    // no way to write by forgetting a flag.
    let apply = env::args().any(|a| a == "--apply");
    let allow_partial = env::args().any(|a| a == "--allow-partial");
    let site_id = env::args()
        .find_map(|a| a.strip_prefix("--site-id=").map(str::to_string))
        .or_else(|| env::var("SITE_ID").ok())
        .unwrap_or_else(|| "hfhotel".to_string());

    tracing::info!(
        site = %site_id,
        mode = if apply { "APPLY" } else { "DRY RUN" },
        allow_partial,
        "repair_room_legacy_keys — starting"
    );

    // Resolve BOTH targets before anything is read or written, so the report
    // header can name the databases actually used and the SITE_ID binding can
    // be checked before a single row moves (F1).
    let (pg, pg_database) = connect_pg().await?;
    let db_config = hotel_backend::config::DbConfig::from_env();
    let targets = Targets {
        pg_database,
        mssql_server: db_config.server.clone(),
        mssql_port: db_config.port,
        mssql_database: db_config.database.clone(),
    };
    let binding = check_target_binding(&site_id, &targets.pg_database, &targets.mssql_database);
    if let Err(reason) = &binding {
        if apply {
            print_targets(&site_id, &targets, Some(reason));
            return Err(reason.clone().into());
        }
    }

    let legacy = fetch_legacy_rooms(&db_config).await?;
    let canonical = fetch_canonical_rooms(&pg).await?;
    tracing::info!(
        site = %site_id,
        legacy_rows = legacy.rooms.len(),
        canonical_rows = canonical.len(),
        "fetched both sides"
    );

    let mut plan = plan_repairs(&legacy.rooms, &canonical);
    plan.skipped_blank_legacy_room_no = legacy.skipped_blank;

    // A partial plan is refused BEFORE the transaction opens — nothing is
    // written, and the operator still gets the full report to act on (F2).
    let blocked = if apply {
        apply_block_reason(&plan, allow_partial)
    } else {
        None
    };

    let start = Instant::now();
    let mut applied = 0u64;
    let mut residual: Option<Plan> = None;
    if apply && blocked.is_none() && !plan.is_noop() {
        applied = apply_repairs(&pg, &plan).await?;

        // Built-in re-verify (F3): the CT room mapper reads outside this bin's
        // lock, so a tick in flight during the apply can re-stamp a row the
        // instant we commit. Re-read BOTH sides and re-plan — a non-empty
        // residual is the only proof that happened, and the caller's fix is
        // simply to re-run.
        let legacy_after = fetch_legacy_rooms(&db_config).await?;
        let canonical_after = fetch_canonical_rooms(&pg).await?;
        let mut after = plan_repairs(&legacy_after.rooms, &canonical_after);
        after.skipped_blank_legacy_room_no = legacy_after.skipped_blank;
        residual = Some(after);
    }
    let duration = start.elapsed();

    let mode = match (apply, blocked.is_some()) {
        (true, true) => Mode::Refused,
        (true, false) => Mode::Apply,
        (false, _) => Mode::DryRun,
    };
    print_report(
        &site_id,
        &targets,
        binding.as_ref().err().map(String::as_str),
        mode,
        &plan,
        applied,
        residual.as_ref(),
        duration,
    );

    if let Some(reason) = blocked {
        return Err(reason.into());
    }
    if let Some(after) = &residual {
        if !after.is_noop() {
            let reason = format!(
                "APPLIED {applied} row(s), but the re-verify still plans {} repair(s) — the live \
                 CT room mapper re-stamped a row during the apply. Nothing is broken; re-run \
                 --apply (ideally with the site's sync worker stopped) until this reports 0.",
                after.repairs.len()
            );
            return Err(reason.into());
        }
    }

    if !apply && !plan.is_noop() {
        tracing::warn!(
            rooms = plan.repairs.len(),
            "DRY RUN — nothing was written. Re-run with --apply to repair."
        );
    }

    Ok(())
}

/// What the run actually did — kept distinct from `apply` so a refused apply
/// can never print the word "APPLY" over a report where nothing was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    DryRun,
    Apply,
    Refused,
}

/// The databases this process actually resolved — printed so a report can be
/// matched to the site it came from (F1).
struct Targets {
    pg_database: String,
    mssql_server: String,
    mssql_port: u16,
    mssql_database: String,
}

/// The report header: WHICH databases this run resolved. Printed by both the
/// dry run and the live run, and on its own when `--apply` is refused, so no
/// report can be read without knowing what it was read from (F1).
fn print_targets(site_id: &str, targets: &Targets, binding_error: Option<&str>) {
    println!();
    println!("=== Room legacy-key repair (ht_rooms_new.legacy_room_id_int / legacy_room_no) ===");
    println!("Site (SITE_ID):          {site_id}");
    println!(
        "PostgreSQL database:     {} (select current_database(){})",
        targets.pg_database,
        match expected_pg_database(site_id) {
            Some(expected) => format!("; SITE_ID implies {expected}"),
            None => "; SITE_ID implies nothing — unknown site".to_string(),
        }
    );
    println!(
        "Legacy MSSQL:            {}:{}/{}{}",
        targets.mssql_server,
        targets.mssql_port,
        targets.mssql_database,
        match expected_mssql_database(site_id) {
            Some(expected) => format!(" (SITE_ID implies db {expected})"),
            None => " (db not derivable from SITE_ID — not verified)".to_string(),
        }
    );
    if let Some(error) = binding_error {
        println!();
        println!("*** TARGET MISMATCH ***");
        println!("{error}");
    }
}

/// Emit the operator-facing before/after table + summary (stdout, so it can be
/// pasted straight into the runbook).
#[allow(clippy::too_many_arguments)]
fn print_report(
    site_id: &str,
    targets: &Targets,
    binding_error: Option<&str>,
    mode: Mode,
    plan: &Plan,
    applied: u64,
    residual: Option<&Plan>,
    duration: std::time::Duration,
) {
    print_targets(site_id, targets, binding_error);
    println!(
        "Mode:                    {}",
        match mode {
            Mode::Apply => "APPLY",
            Mode::DryRun => "DRY RUN (no writes)",
            Mode::Refused => "APPLY REFUSED (no writes)",
        }
    );
    println!();

    if plan.repairs.is_empty() {
        println!("No rooms need repair — canonical pointers already match legacy HT_Rooms.");
    } else {
        println!(
            "{:<10} {:>10}  {:>22}  {:>22}",
            "room_no", "room_id", "before (id / room_no)", "after (id / room_no)"
        );
        println!("{}", "-".repeat(70));
        for repair in &plan.repairs {
            println!(
                "{:<10} {:>10}  {:>22}  {:>22}",
                repair.room_no,
                repair.canonical_room_id,
                format!(
                    "{} / {}",
                    repair
                        .before_legacy_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "NULL".to_string()),
                    repair.before_legacy_room_no.as_deref().unwrap_or("NULL")
                ),
                format!("{} / {}", repair.after_legacy_id, repair.after_legacy_room_no),
            );
        }
    }

    println!();
    println!(
        "Rooms to repair:         {}{}",
        plan.repairs.len(),
        if mode == Mode::Apply {
            ""
        } else {
            " (would repair)"
        }
    );
    if mode == Mode::Apply {
        println!("Rows updated:            {applied}");
    }
    println!("Already correct:         {}", plan.already_correct.len());
    println!(
        "Legacy without canonical:{} {}",
        plan.legacy_without_canonical.len(),
        summarize(
            &plan
                .legacy_without_canonical
                .iter()
                .map(|r| format!("{}(id={})", r.room_no, r.id))
                .collect::<Vec<_>>()
        )
    );
    println!(
        "Canonical without legacy:{} {}",
        plan.canonical_without_legacy.len(),
        summarize(&plan.canonical_without_legacy)
    );
    println!(
        "Ambiguous legacy Room_no:{} {}",
        plan.ambiguous_legacy_room_nos.len(),
        summarize(&plan.ambiguous_legacy_room_nos)
    );
    println!(
        "Blank legacy Room_no:    {}",
        plan.skipped_blank_legacy_room_no
    );
    println!("Duration:                {duration:?}");

    // Built-in post-apply re-verify (F3) — both sides re-read, plan recomputed.
    if let Some(after) = residual {
        println!();
        println!("--- re-verify (both sides re-read after the apply) ---");
        println!("Rooms still to repair:   {}", after.repairs.len());
        if after.is_noop() {
            println!("Re-verify:               PASS (0 residual repairs)");
        } else {
            println!(
                "Re-verify:               FAIL — {}",
                summarize(
                    &after
                        .repairs
                        .iter()
                        .map(|r| r.room_no.clone())
                        .collect::<Vec<_>>()
                )
            );
            println!(
                "The live CT room mapper re-stamped a row during the apply (it reads outside \
                 this bin's lock). Nothing is broken — re-run --apply, ideally with the site's \
                 sync worker stopped, until this reports 0."
            );
        }
    }

    if !plan.ambiguous_legacy_room_nos.is_empty() {
        println!();
        println!(
            "WARNING: {} legacy Room_no value(s) appear MORE THAN ONCE. Matching by \
             room number is ambiguous for them, so they were skipped entirely — \
             resolve them in iHOTEL before trusting this repair.",
            plan.ambiguous_legacy_room_nos.len()
        );
    }
    if !plan.legacy_without_canonical.is_empty() || !plan.canonical_without_legacy.is_empty() {
        println!();
        println!(
            "NOTE: rooms present on only one side are REPORTED, never guessed. A legacy \
             room with no canonical row needs `backfill_rooms`; a canonical room with no \
             legacy row keeps whatever pointers it already had."
        );
    }
}

fn summarize(items: &[String]) -> String {
    if items.is_empty() {
        return String::new();
    }
    const MAX: usize = 12;
    if items.len() <= MAX {
        format!("[{}]", items.join(", "))
    } else {
        format!("[{}, … +{}]", items[..MAX].join(", "), items.len() - MAX)
    }
}

/// Apply the plan in ONE transaction — the pointers move together or not at all.
async fn apply_repairs(
    pg: &PgPool,
    plan: &Plan,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let mut tx = pg.begin().await?;
    let mut updated = 0u64;
    for repair in &plan.repairs {
        // Idempotent + non-destructive: matches on the TRUTHFUL column
        // (`room_no`) and only writes when a pointer actually differs, so a
        // re-run — or a run racing the live CT watcher, which may have already
        // re-stamped the row — is a no-op rather than a fight.
        let affected = sqlx::query(
            "UPDATE ht_rooms_new \
                SET legacy_room_id_int = $1, legacy_room_no = $2, updated_at = NOW() \
              WHERE room_no = $2 \
                AND (legacy_room_id_int IS DISTINCT FROM $1 \
                     OR legacy_room_no IS DISTINCT FROM $2)",
        )
        .bind(repair.after_legacy_id)
        .bind(&repair.after_legacy_room_no)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if affected == 0 {
            tracing::info!(
                room_no = %repair.room_no,
                "row already carried the target pointers (CT watcher got there first) — no-op"
            );
        }
        updated += affected;
    }
    tx.commit().await?;
    Ok(updated)
}

struct LegacyRooms {
    rooms: Vec<LegacyRoom>,
    skipped_blank: u64,
}

/// Read `(id, Room_no)` from legacy `HT_Rooms` — READ-ONLY.
///
/// Takes the config the caller already resolved (and printed in the report
/// header) so the rows can never come from a different server than the one the
/// report names.
async fn fetch_legacy_rooms(
    config: &hotel_backend::config::DbConfig,
) -> Result<LegacyRooms, Box<dyn std::error::Error + Send + Sync>> {
    let server = config.server.clone();
    let port = config.port;
    let database = config.database.clone();
    let pool = hotel_backend::db::create_pool(config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    tracing::info!("Connecting to legacy SQL Server at {server}:{port} (db {database})");

    let mut conn = pool.get().await?;
    let rows = conn
        .simple_query("SELECT id, Room_no FROM HT_Rooms ORDER BY id")
        .await?
        .into_first_result()
        .await?;

    let mut out = Vec::with_capacity(rows.len());
    let mut skipped_blank = 0u64;
    for row in &rows {
        let Some(id) = row.get::<i32, _>("id") else {
            skipped_blank += 1;
            continue;
        };
        let room_no = row
            .get::<&str, _>("Room_no")
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let Some(room_no) = room_no else {
            skipped_blank += 1;
            continue;
        };
        out.push(LegacyRoom {
            id,
            room_no: room_no.to_string(),
        });
    }
    if skipped_blank > 0 {
        tracing::warn!(
            skipped_blank,
            "legacy HT_Rooms rows with NULL id / blank Room_no skipped (unmatchable)"
        );
    }
    Ok(LegacyRooms {
        rooms: out,
        skipped_blank,
    })
}

/// Read the canonical room pointers. Runtime `sqlx::query` — no `.sqlx/` cache.
async fn fetch_canonical_rooms(
    pg: &PgPool,
) -> Result<Vec<CanonicalRoom>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query(
        "SELECT room_id, room_no, legacy_room_id_int, legacy_room_no \
           FROM ht_rooms_new ORDER BY room_no",
    )
    .fetch_all(pg)
    .await?;
    Ok(rows
        .iter()
        .map(|row| CanonicalRoom {
            room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
            room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
            legacy_room_id_int: row.try_get::<Option<i32>, _>("legacy_room_id_int").ok().flatten(),
            legacy_room_no: row.try_get::<Option<String>, _>("legacy_room_no").ok().flatten(),
        })
        .collect())
}

/// Connect to canonical PG and ask the SERVER which database it landed in —
/// never parsed out of the URL, which can carry a stale path, an override, or
/// a `?dbname=` the driver ignores. The answer is what the report header shows
/// and what the `SITE_ID` binding check is made against (F1).
async fn connect_pg() -> Result<(PgPool, String), Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(PG_POOL_MAX)
        .connect(&url)
        .await?;
    let database: String = sqlx::query("select current_database()")
        .fetch_one(&pool)
        .await?
        .try_get::<String, _>(0)?;
    tracing::info!(database = %database, "Connected to PostgreSQL");
    Ok((pool, database))
}

// =============================================================================
// Tests — the planner is pure, so the DRY-RUN semantics are fully testable
// without either database.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy(id: i32, room_no: &str) -> LegacyRoom {
        LegacyRoom {
            id,
            room_no: room_no.to_string(),
        }
    }

    fn canonical(
        room_id: i32,
        room_no: &str,
        legacy_id: Option<i32>,
        legacy_no: Option<&str>,
    ) -> CanonicalRoom {
        CanonicalRoom {
            room_id,
            room_no: room_no.to_string(),
            legacy_room_id_int: legacy_id,
            legacy_room_no: legacy_no.map(str::to_string),
        }
    }

    /// HF Hotel's shape: the two orderings genuinely agree, so a dry run must
    /// report ZERO repairs. This is the bin's own correctness proof — if it
    /// wanted to "fix" HF Hotel, its matching rule would be wrong.
    #[test]
    fn aligned_site_needs_no_repair() {
        let legacy_rooms = vec![legacy(1, "101"), legacy(2, "102"), legacy(3, "103")];
        let canonical_rooms = vec![
            canonical(1, "101", Some(1), Some("101")),
            canonical(2, "102", Some(2), Some("102")),
            canonical(3, "103", Some(3), Some("103")),
        ];
        let plan = plan_repairs(&legacy_rooms, &canonical_rooms);
        assert!(plan.is_noop(), "aligned site must need no repair: {plan:?}");
        assert_eq!(plan.already_correct.len(), 3);
        assert!(plan.legacy_without_canonical.is_empty());
        assert!(plan.canonical_without_legacy.is_empty());
    }

    /// The live HF Ville shape (§0.2 of the design): legacy `id=2` is
    /// `Room_no='116'`, `id=3` is `'102'`, `id=21` is `'203'`, while canonical
    /// assumed sequential ids. Every mismatched room must be re-pointed at the
    /// legacy row that carries ITS OWN number.
    #[test]
    fn interleaved_site_repoints_by_room_number() {
        let legacy_rooms = vec![
            legacy(1, "101"),
            legacy(2, "116"),
            legacy(3, "102"),
            legacy(21, "203"),
        ];
        let canonical_rooms = vec![
            canonical(1, "101", Some(1), Some("101")),
            canonical(2, "102", Some(2), Some("116")),
            canonical(3, "103", Some(3), Some("102")),
            canonical(21, "205", Some(21), Some("203")),
            canonical(16, "116", Some(16), Some("999")),
        ];
        let plan = plan_repairs(&legacy_rooms, &canonical_rooms);

        // 101 already agrees.
        assert_eq!(plan.already_correct, vec!["101".to_string()]);

        // 102 must point at legacy id 3 (the row whose Room_no IS '102'),
        // and 116 at legacy id 2 — the swap that is wrong live today.
        let by_no: HashMap<&str, &Repair> = plan
            .repairs
            .iter()
            .map(|r| (r.room_no.as_str(), r))
            .collect();
        assert_eq!(by_no["102"].after_legacy_id, 3);
        assert_eq!(by_no["102"].after_legacy_room_no, "102");
        assert_eq!(by_no["102"].before_legacy_id, Some(2));
        assert_eq!(by_no["116"].after_legacy_id, 2);
        assert_eq!(by_no["116"].after_legacy_room_no, "116");

        // Legacy 203 belongs to canonical room '203', which does not exist here
        // — reported, NOT silently re-pointed at canonical '205'.
        assert_eq!(
            plan.legacy_without_canonical,
            vec![legacy(21, "203")],
            "an unmatched legacy room must be reported, never guessed"
        );
        assert!(
            !plan.repairs.iter().any(|r| r.room_no == "205"),
            "canonical 205 must NOT be re-pointed at legacy 203"
        );
        assert!(plan.canonical_without_legacy.contains(&"205".to_string()));
        assert!(plan.canonical_without_legacy.contains(&"103".to_string()));
    }

    /// A repair is also needed when the id is already right but the
    /// denormalised number is not — the acceptance criterion is
    /// `room_no IS DISTINCT FROM legacy_room_no` reaching zero, which a
    /// guard on the id alone could never deliver.
    #[test]
    fn stale_legacy_room_no_alone_is_repaired() {
        let plan = plan_repairs(
            &[legacy(7, "207")],
            &[canonical(7, "207", Some(7), Some("107"))],
        );
        assert_eq!(plan.repairs.len(), 1);
        assert_eq!(plan.repairs[0].after_legacy_room_no, "207");
        assert_eq!(plan.repairs[0].before_legacy_room_no.as_deref(), Some("107"));
    }

    /// NULL pointers (never-stamped rows) are a repair, not an error.
    #[test]
    fn null_pointers_are_repaired() {
        let plan = plan_repairs(&[legacy(5, "105")], &[canonical(5, "105", None, None)]);
        assert_eq!(plan.repairs.len(), 1);
        assert_eq!(plan.repairs[0].before_legacy_id, None);
        assert_eq!(plan.repairs[0].after_legacy_id, 5);
    }

    /// A duplicated legacy `Room_no` makes number-matching ambiguous. Skip and
    /// report — never pick one by a tiebreak nobody verified.
    #[test]
    fn duplicate_legacy_room_no_is_skipped_not_guessed() {
        let plan = plan_repairs(
            &[legacy(1, "101"), legacy(9, "101"), legacy(2, "102")],
            &[
                canonical(1, "101", Some(99), Some("999")),
                canonical(2, "102", Some(88), Some("888")),
            ],
        );
        assert_eq!(plan.ambiguous_legacy_room_nos, vec!["101".to_string()]);
        assert_eq!(
            plan.repairs.len(),
            1,
            "only the unambiguous room may be repaired: {plan:?}"
        );
        assert_eq!(plan.repairs[0].room_no, "102");
    }

    /// Re-running after a successful apply must plan nothing — the guard in the
    /// UPDATE mirrors this, so a dry run and the SQL agree on "no-op".
    #[test]
    fn plan_is_idempotent() {
        let legacy_rooms = vec![legacy(2, "116"), legacy(3, "102")];
        let before = vec![
            canonical(2, "102", Some(2), Some("116")),
            canonical(16, "116", Some(16), Some("999")),
        ];
        let first = plan_repairs(&legacy_rooms, &before);
        assert_eq!(first.repairs.len(), 2);

        // Simulate the apply.
        let after: Vec<CanonicalRoom> = before
            .iter()
            .map(|room| {
                let repair = first.repairs.iter().find(|r| r.room_no == room.room_no);
                match repair {
                    Some(r) => CanonicalRoom {
                        legacy_room_id_int: Some(r.after_legacy_id),
                        legacy_room_no: Some(r.after_legacy_room_no.clone()),
                        ..room.clone()
                    },
                    None => room.clone(),
                }
            })
            .collect();
        assert!(
            plan_repairs(&legacy_rooms, &after).is_noop(),
            "a second run must be a no-op"
        );
    }

    // -------------------------------------------------------------------
    // F1 — SITE_ID ↔ resolved-database binding
    // -------------------------------------------------------------------

    #[test]
    fn site_id_implies_its_own_pg_database() {
        assert_eq!(expected_pg_database("hfhotel"), Some("hotelnew"));
        assert_eq!(expected_pg_database("hfville"), Some("hotelville"));
        assert_eq!(expected_pg_database("hfvilel"), None);
    }

    #[test]
    fn matching_targets_are_accepted() {
        assert!(check_target_binding("hfville", "hotelville", "HOTEL").is_ok());
        // HF Hotel's legacy DB_NAME comes from a repo secret — not derivable,
        // so whatever it is must not be second-guessed.
        assert!(check_target_binding("hfhotel", "hotelnew", "anything").is_ok());
    }

    /// THE failure this gate exists for: `SITE_ID`/`DB_*` exported for Ville
    /// but `DATABASE_URL` left to dotenvy's `.env` (HF Hotel's). The run would
    /// read Ville's legacy rooms and rewrite HF HOTEL's canonical pointers,
    /// and its report would be indistinguishable from a correct Ville run.
    #[test]
    fn ville_site_against_hotel_pg_database_is_refused() {
        let error = check_target_binding("hfville", "hotelnew", "HOTEL")
            .expect_err("a Ville run against hotelnew must be refused");
        assert!(error.contains("hotelnew"), "{error}");
        assert!(error.contains("hotelville"), "{error}");
        assert!(error.starts_with("REFUSING to write"), "{error}");
    }

    #[test]
    fn hotel_site_against_ville_pg_database_is_refused() {
        assert!(check_target_binding("hfhotel", "hotelville", "db").is_err());
    }

    /// MSSQL database names are case-insensitive, so a legitimately-cased
    /// value must not be refused — but a genuinely different database must be.
    #[test]
    fn mssql_database_is_checked_case_insensitively_where_derivable() {
        assert!(check_target_binding("hfville", "hotelville", "hotel").is_ok());
        let error = check_target_binding("hfville", "hotelville", "HOTELNEW")
            .expect_err("a Ville run against a non-HOTEL legacy db must be refused");
        assert!(error.contains("HOTELNEW"), "{error}");
    }

    /// An unknown `SITE_ID` cannot imply anything, so nothing can be verified
    /// — that is a refusal, not a pass.
    #[test]
    fn unknown_site_id_cannot_be_verified_and_is_refused() {
        let error = check_target_binding("hfvilel", "hotelville", "HOTEL")
            .expect_err("an unknown SITE_ID must not be silently trusted");
        assert!(error.contains("not one of the known sites"), "{error}");
    }

    // -------------------------------------------------------------------
    // F2 — a partial plan must not be applied
    // -------------------------------------------------------------------

    fn partial_plan_pieces() -> Vec<(&'static str, Plan)> {
        vec![
            (
                "legacy_without_canonical",
                Plan {
                    repairs: vec![],
                    legacy_without_canonical: vec![legacy(21, "203")],
                    ..Plan::default()
                },
            ),
            (
                "canonical_without_legacy",
                Plan {
                    canonical_without_legacy: vec!["210".to_string()],
                    ..Plan::default()
                },
            ),
            (
                "ambiguous_legacy_room_nos",
                Plan {
                    ambiguous_legacy_room_nos: vec!["101".to_string()],
                    ..Plan::default()
                },
            ),
        ]
    }

    #[test]
    fn complete_plan_is_not_blocked() {
        let plan = plan_repairs(
            &[legacy(3, "102"), legacy(2, "116")],
            &[
                canonical(2, "102", Some(2), Some("116")),
                canonical(16, "116", Some(16), Some("999")),
            ],
        );
        assert_eq!(plan.repairs.len(), 2);
        assert!(
            apply_block_reason(&plan, false).is_none(),
            "a plan with no gaps must apply without --allow-partial"
        );
    }

    /// Each gap on its own blocks the apply: an unmatched canonical room keeps
    /// a stale `legacy_room_id_int` that can collide with an id this run
    /// assigns elsewhere, and the CT room resolver has no tiebreak.
    #[test]
    fn each_gap_blocks_the_apply() {
        for (label, plan) in partial_plan_pieces() {
            let reason = apply_block_reason(&plan, false)
                .unwrap_or_else(|| panic!("{label} must block the apply"));
            assert!(
                reason.starts_with("REFUSING to apply a PARTIAL plan"),
                "{label}: {reason}"
            );
        }
    }

    #[test]
    fn allow_partial_overrides_every_gap() {
        for (label, plan) in partial_plan_pieces() {
            assert!(
                apply_block_reason(&plan, true).is_none(),
                "{label} must be applyable under an explicit --allow-partial"
            );
        }
    }

    /// The live Ville shape reported at dry-run time (24 canonical rooms with
    /// no legacy row) must NOT slip through as an unremarkable NOTE.
    #[test]
    fn ville_dry_run_shape_blocks_without_allow_partial() {
        let plan = plan_repairs(
            &[legacy(2, "116"), legacy(3, "102")],
            &[
                canonical(2, "102", Some(2), Some("116")),
                canonical(16, "116", Some(16), Some("999")),
                canonical(10, "210", Some(12), Some("210")),
            ],
        );
        assert_eq!(plan.canonical_without_legacy, vec!["210".to_string()]);
        let reason = apply_block_reason(&plan, false).expect("unmatched canonical must block");
        assert!(reason.contains("canonical room(s) with no legacy row"), "{reason}");
    }
}
