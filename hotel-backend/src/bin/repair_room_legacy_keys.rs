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
//! constraint. Safe to run while sync is live: once the ids are right the
//! mapper's preferred id-match resolves the correct row, and its
//! `legacy_room_no = COALESCE($4, legacy_room_no)` re-stamps the correct number.
//!
//! Rooms present on only one side are **reported, never guessed** — a legacy
//! `Room_no` with no canonical row, a canonical room with no legacy counterpart,
//! and (fatal to matching) duplicate legacy `Room_no` values are all listed for
//! the operator instead of being resolved by heuristic.
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
//! ```
//!
//! Run once per site. HF Hotel is the bin's own correctness proof: a dry run
//! there must report **0 rooms to repair**.
//!
//! ## Verification after `--apply` (HF Ville)
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
    let site_id = env::args()
        .find_map(|a| a.strip_prefix("--site-id=").map(str::to_string))
        .or_else(|| env::var("SITE_ID").ok())
        .unwrap_or_else(|| "hfhotel".to_string());

    tracing::info!(
        site = %site_id,
        mode = if apply { "APPLY" } else { "DRY RUN" },
        "repair_room_legacy_keys — starting"
    );

    let pg = connect_pg().await?;
    let legacy = fetch_legacy_rooms().await?;
    let canonical = fetch_canonical_rooms(&pg).await?;
    tracing::info!(
        site = %site_id,
        legacy_rows = legacy.rooms.len(),
        canonical_rows = canonical.len(),
        "fetched both sides"
    );

    let mut plan = plan_repairs(&legacy.rooms, &canonical);
    plan.skipped_blank_legacy_room_no = legacy.skipped_blank;

    let start = Instant::now();
    let mut applied = 0u64;
    if apply && !plan.is_noop() {
        applied = apply_repairs(&pg, &plan).await?;
    }
    let duration = start.elapsed();

    print_report(&site_id, apply, &plan, applied, duration);

    if !apply && !plan.is_noop() {
        tracing::warn!(
            rooms = plan.repairs.len(),
            "DRY RUN — nothing was written. Re-run with --apply to repair."
        );
    }

    Ok(())
}

/// Emit the operator-facing before/after table + summary (stdout, so it can be
/// pasted straight into the runbook).
fn print_report(
    site_id: &str,
    apply: bool,
    plan: &Plan,
    applied: u64,
    duration: std::time::Duration,
) {
    println!();
    println!("=== Room legacy-key repair (ht_rooms_new.legacy_room_id_int / legacy_room_no) ===");
    println!("Site:                    {site_id}");
    println!(
        "Mode:                    {}",
        if apply { "APPLY" } else { "DRY RUN (no writes)" }
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
        if apply { "" } else { " (would repair)" }
    );
    if apply {
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
async fn fetch_legacy_rooms() -> Result<LegacyRooms, Box<dyn std::error::Error + Send + Sync>> {
    let config = hotel_backend::config::DbConfig::from_env();
    let server = config.server.clone();
    let port = config.port;
    let database = config.database.clone();
    let pool = hotel_backend::db::create_pool(&config)
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

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(PG_POOL_MAX)
        .connect(&url)
        .await?;
    tracing::info!("Connected to PostgreSQL");
    Ok(pool)
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
}
