//! One-shot catch-up backfill: legacy MSSQL `HT_Room_Status` -> canonical PG
//! `ht_room_calendar` (issue #273 remainder — see
//! `docs/coexistence/room-calendar-deficit-design.md`, the design this bin
//! implements).
//!
//! ## Why this exists
//!
//! `ht_room_calendar` (migration 039, `sync/mappers/room_calendar.rs`)
//! shipped **forward-only** on 2026-05-13: the CT watcher has correctly
//! captured every `HT_Room_Status` change since, but nothing ever backfilled
//! the pre-coverage history, unlike every sibling entity (`backfill_rooms`,
//! `backfill_legacy_checkins`, `backfill_legacy_bookings`,
//! `backfill_payment_ledger`, `backfill_receipt_payments`, …). The live
//! deficit measured 2026-07-31: HF Hotel 3,137 missing nights, HF Ville 513
//! (99.7% pre-2026-05-13 departure-marker history; a handful of ordinary,
//! already-closed sync stragglers make up the rest). Full characterisation,
//! root cause, and the remediation shape below are in the design doc — this
//! file does not re-derive any of it.
//!
//! **This is NOT an active-bug fix.** Nothing reads `ht_room_calendar` today
//! (`routes/calendar.rs` / `routes/rooms.rs` reconstruct availability from
//! `ht_bookings` + `ht_checkins`); the only consumer is the Phase 6-C mirror
//! probe, itself dark behind `RECONCILE_MIRROR_PROBE_ENABLED=false`. This bin
//! is operability hygiene ahead of enabling that probe (design §5), not a
//! user-facing gap to rush.
//!
//! ## Legacy access is READ-ONLY — no dark flag needed
//!
//! Same reasoning as `backfill_receipt_payments`: this bin never writes to
//! MSSQL, only `SELECT`s against `HT_Room_Status` (routed through
//! `db::mssql_timeout`'s timeout+poison wrappers). Every write lands in
//! canonical PostgreSQL only, so invariant #6 ("new legacy writes ship dark")
//! does not apply and no `_ENABLED` flag or reception-coordinated live test
//! is needed to run this.
//!
//! ## What it does
//!
//! 1. Determines the legacy scan floor: the canonical `MIN(rcal_date)`
//!    (auto-derived, same "never configured" contract as
//!    `scheduler::sync::room_calendar_business_key_legacy_sql`'s era floor —
//!    though not the same SQL: [`CANONICAL_FLOOR_SQL`] is unfiltered, while
//!    the probe's floor (issue #273, 2026-07-31) is `MIN` over MIRRORED rows
//!    only, `WHERE rcal_legacy_id IS NOT NULL`. A detached formerly-mirrored
//!    tile can predate the mirrored floor, so this bin's floor can only be
//!    EARLIER, never later — it scans a wider legacy window, never a
//!    narrower one. Safe in that direction: anything this bin inserts is
//!    stamped with a fresh `rcal_legacy_id` through the same mapper the CT
//!    watcher uses, which only ever pulls the probe's mirrored-only floor
//!    DOWN toward this one, never leaves a gap the probe's narrower floor
//!    would have skipped),
//!    overridden by `--since=YYYY-MM-DD`, or disabled entirely by `--all`
//!    (unfloored — expensive, scans all `HT_Room_Status` history).
//! 2. Reads the legacy work-list: one row per **business key**
//!    `(room_no, CAST(room_date AS DATE))` in era, deduplicated to a single
//!    deterministic representative (`MAX(id)`) when `HT_Room_Status` carries
//!    duplicate rows for the same night (observed live for HF Ville room
//!    201) — see [`dedup_worklist_sql`].
//! 3. Reads the canonical business-key set (`ht_room_calendar` JOIN
//!    `ht_rooms_new`, **unfiltered** — deliberately includes app-authored
//!    surplus rows with `rcal_legacy_id IS NULL` so they read as
//!    "already present" and are never touched by this bin; see
//!    [`fetch_canonical_keys`]).
//! 4. Diffs (2) against (3) in memory. Only legacy keys **absent** from the
//!    canonical set are candidates — this is the `missing_pg` deficit.
//! 5. For each candidate, re-drives the SAME code the live CT watcher runs:
//!    [`RoomCalendarMapper::apply`] (`ChangeOp::Insert` — `Insert` and
//!    `Update` hit the identical `apply_upsert` branch in this mapper, so
//!    the backfill doesn't need to know or care which case it is). This bin
//!    never hand-writes a bespoke `INSERT` against `ht_room_calendar`; that
//!    divergence-from-the-mapper is the exact bug class that caused #204's
//!    customer drift.
//! 6. **Idempotent**, two ways: the pre-diff in step 4 means an
//!    already-mirrored key is never even attempted twice (no-op, not a
//!    duplicate write); and even if a race re-attempted one, the mapper's
//!    own `ON CONFLICT (rcal_room_id, rcal_date) DO UPDATE` makes a repeat
//!    apply a true no-op in effect. Safe to run twice, or after the mirror
//!    probe (once enabled) has already narrowed the gap.
//! 7. `--dry-run` resolves everything (room/booking/checkin lookups via the
//!    SAME `sync::resolve` helpers the mapper itself calls) but commits no
//!    PG transaction — writes NOTHING. Given the volume (thousands of rows
//!    per site), output is a per-site SUMMARY plus a bounded sample of
//!    individual lines ([`SAMPLE_PRINT_LIMIT`]), not one line per row.
//!    Errors (e.g. a genuinely unknown room) are always printed in full —
//!    per the design, this bin must surface them loudly, never silently
//!    skip them, even though §2's evidence says this should not happen.
//! 8. Domain events are intentionally NOT published — matches the
//!    established precedent (`backfill_legacy_checkins`,
//!    `backfill_receipt_payments`): a backfilled historical row is not a
//!    new event from a domain perspective. `RoomCalendarMapper` doesn't
//!    return one today anyway (F1 ships canonical projection only).
//! 9. This bin does NOT mark any `ht_reconcile_log` row resolved — there
//!    isn't a per-key row to mark (the calendar probe records one
//!    *aggregate* row per site, `per_pk: false`). Once
//!    `RECONCILE_MIRROR_PROBE_ENABLED` is flipped on, the next mirror-probe
//!    tick re-hashes both sides fresh and closes any open
//!    `mirror_ht_room_calendar` row itself when business-key counts
//!    converge — the designed, self-verifying path this bin deliberately
//!    does not race or duplicate.
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//!
//! # ALWAYS start here — reports what would be written, writes nothing.
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_room_calendar -- --dry-run
//!
//! # Live run — drains the current era-floored deficit.
//! cargo run --release --bin backfill_room_calendar
//!
//! # Narrower manual re-run from an explicit date:
//! cargo run --release --bin backfill_room_calendar -- --dry-run --since=2026-01-01
//!
//! # Full unfloored history scan (expensive — HF Hotel's unfloored
//! # HT_Room_Status is unbounded history):
//! cargo run --release --bin backfill_room_calendar -- --dry-run --all
//! ```
//!
//! Run once per site — same env-driven site selection as every sibling
//! backfill bin (`DATABASE_URL`/`NEW_DATABASE_URL` for the PG side,
//! `DB_SERVER`/`DB_NAME`/… for the legacy side, `SITE_ID` for the log tag).
//! No new site-selection mechanism.
//!
//! ## Flags
//!
//! * `--dry-run`      — resolve everything, commit nothing. Reports what
//!                       WOULD be written.
//! * `--since=YYYY-MM-DD` — override the auto-derived floor. An unparsable
//!                       value is logged and ignored (falls back to the
//!                       derived floor), matching the permissive parsing
//!                       style of the other backfill bins' `--days=N`.
//! * `--all`           — no floor at all; scans the full legacy table.
//!                       Wins over `--since=` if both are given.

use std::collections::HashSet;
use std::env;

use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tiberius::Query;

use hotel_backend::config::{DbConfig, SiteConfig};
use hotel_backend::db::mssql_timeout::{
    query_with_timeout_pooled, simple_query_with_timeout_pooled, MssqlOpKind,
};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::RoomCalendarMapper;
use hotel_backend::sync::resolve;
use hotel_backend::sync::row::MappableRow;
use hotel_backend::sync::SyncError;

const PG_POOL_MAX: u32 = 4;
const MSSQL_POOL_MAX: u32 = 4;
const TABLE: &str = "HT_Room_Status";

/// Bound on individually-printed candidate lines per outcome kind in
/// `--dry-run` / live output (design §4: "a per-site SUMMARY plus a bounded
/// sample, not 3,650 lines"). Errors are exempt from this bound — they are
/// expected to be rare (design §2) and must always surface loudly.
const SAMPLE_PRINT_LIMIT: usize = 25;

/// Canonical business-key set query — deliberately unfiltered on
/// `rcal_legacy_id`: app-authored surplus rows (`rcal_legacy_id IS NULL`)
/// must read as "already present" so this bin never attempts to touch them
/// (requirement: leave the surplus alone).
const CANONICAL_KEYS_SQL: &str = "SELECT r.room_no, c.rcal_date \
     FROM ht_room_calendar c \
     JOIN ht_rooms_new r ON r.room_id = c.rcal_room_id";

/// Auto-derived era floor — the same "never configured" contract as
/// `scheduler::sync::room_calendar_business_key_legacy_sql`'s floor. `None`
/// means the canonical calendar is currently empty (unfloored is then the
/// correct behaviour — see `dedup_worklist_sql`'s doc).
const CANONICAL_FLOOR_SQL: &str = "SELECT MIN(rcal_date) FROM ht_room_calendar";

/// Columns projected from `HT_Room_Status` — matches
/// `sync::mappers::room_calendar::SELECT_COLS` field-for-field (kept as an
/// independent copy; that const is private to the mapper module, same
/// independence idiom `backfill_receipt_payments` uses for its own
/// `RECEIPT_COLS`).
const LEGACY_ROOM_STATUS_COLS: &str = "t.id, t.room_no, t.room_date, t.room_status, \
     t.room_Details, t.room_Book_No, t.room_CheckIn_No";

#[derive(Debug, Default)]
struct Summary {
    legacy_in_era: usize,
    already_present: usize,
    candidates: usize,
    written: usize,
    errored: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Args {
    dry_run: bool,
    all: bool,
    since_raw: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backfill_room_calendar=info,hotel_backend=info".into()),
        )
        .init();

    let args = parse_args(env::args().skip(1));
    let explicit_since = args.since_raw.as_deref().and_then(|s| {
        match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(d) => Some(d),
            Err(err) => {
                tracing::warn!(
                    value = s, error = %err,
                    "--since= value did not parse as YYYY-MM-DD — ignoring, \
                     falling back to the derived floor"
                );
                None
            }
        }
    });
    let site = SiteConfig::from_env();

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let canonical_floor = fetch_canonical_floor(&pg).await?;
    let (floor, floor_desc): (Option<NaiveDate>, String) = if args.all {
        (None, "none (--all: full unfloored legacy scan)".to_string())
    } else if let Some(d) = explicit_since {
        (Some(d), format!("{d} (--since override)"))
    } else {
        match canonical_floor {
            Some(d) => (Some(d), format!("{d} (derived from canonical MIN(rcal_date))")),
            None => (
                None,
                "none (canonical ht_room_calendar is currently empty — full scan)".to_string(),
            ),
        }
    };

    tracing::info!(
        site = %site.id, dry_run = args.dry_run, floor = %floor_desc,
        "issue #273 — backfilling ht_room_calendar from legacy HT_Room_Status"
    );

    let canonical_keys = fetch_canonical_keys(&pg).await?;
    let legacy_rows = fetch_legacy_worklist(&mssql, floor).await?;

    tracing::info!(
        site = %site.id,
        canonical_nights = canonical_keys.len(),
        legacy_in_era = legacy_rows.len(),
        "fetched canonical business-key set and legacy work-list"
    );

    let mut s = Summary { legacy_in_era: legacy_rows.len(), ..Default::default() };
    let mut sample_written = SAMPLE_PRINT_LIMIT;

    for row in &legacy_rows {
        let preview = match preview_candidate(row) {
            Ok(p) => p,
            Err(err) => {
                tracing::warn!(error = %err, "failed to preview legacy row — skipping");
                println!("[ERROR] failed to preview legacy HT_Room_Status row: {err}");
                s.errored += 1;
                continue;
            }
        };

        let key = (preview.room_no.clone(), preview.night);
        if canonical_keys.contains(&key) {
            s.already_present += 1;
            continue;
        }
        s.candidates += 1;

        process_candidate(&pg, row, &preview, args.dry_run, &mut sample_written, &mut s).await;
    }

    println!();
    println!("=== backfill_room_calendar — issue #273 ===");
    println!("Site:                          {}", site.id);
    println!(
        "Mode:                          {}",
        if args.dry_run { "DRY RUN (writes nothing)" } else { "LIVE" }
    );
    println!("Floor:                         {floor_desc}");
    println!("Legacy nights in era (deduped): {}", s.legacy_in_era);
    println!("Already present (no-op):       {}", s.already_present);
    println!("Candidates (missing_pg):       {}", s.candidates);
    println!(
        "{:<31}{}",
        if args.dry_run { "Would write:" } else { "Written:" },
        s.written
    );
    println!("Errored:                       {}", s.errored);
    if !args.dry_run && s.written > 0 {
        println!();
        println!(
            "Note: this bin does not mark any ht_reconcile_log row resolved — the \
             calendar mirror probe records one aggregate row per site, not a \
             per-key row. Once RECONCILE_MIRROR_PROBE_ENABLED is on, the next \
             mirror-probe tick re-hashes both sides and closes any open \
             mirror_ht_room_calendar row itself when business-key counts \
             converge — that is the designed, self-verifying path."
        );
    }

    tracing::info!(
        site = %site.id,
        dry_run = args.dry_run,
        legacy_in_era = s.legacy_in_era,
        already_present = s.already_present,
        candidates = s.candidates,
        written = s.written,
        errored = s.errored,
        "backfill_room_calendar — done"
    );

    Ok(())
}

/// Parse CLI args. Pure — no I/O — so it is unit-testable without touching
/// `env::args()`. `--since=` is captured as a raw string here; the
/// YYYY-MM-DD parse (and its warn-and-fallback on failure) happens in
/// `main` so this function stays a pure string-in/struct-out mapping.
/// Unrecognised `--flags` are ignored, matching the permissive parsing
/// style of the other backfill bins.
fn parse_args<I: IntoIterator<Item = String>>(args: I) -> Args {
    let mut a = Args::default();
    for arg in args {
        if arg == "--dry-run" {
            a.dry_run = true;
        } else if arg == "--all" {
            a.all = true;
        } else if let Some(v) = arg.strip_prefix("--since=") {
            a.since_raw = Some(v.to_string());
        }
    }
    a
}

/// Legacy work-list query: one row per `(room_no, night)` business key in
/// era, deduplicated to a single deterministic representative (`MAX(id)`,
/// matching how the live mapper's UPSERT would simply apply whichever row
/// CT delivered last — driving the mapper once per duplicate would only
/// add redundant writes converging on the same canonical state).
///
/// `floored = false` scans the whole legacy table on purpose — with an
/// EMPTY canonical calendar, "legacy has N nights and we have none" IS the
/// finding (same contract as
/// `scheduler::sync::room_calendar_business_key_legacy_sql`).
fn dedup_worklist_sql(floored: bool) -> String {
    let floor_clause =
        if floored { " AND CAST(room_date AS DATE) >= CAST(@P1 AS DATE)" } else { "" };
    format!(
        "SELECT {LEGACY_ROOM_STATUS_COLS} \
           FROM HT_Room_Status t \
           INNER JOIN ( \
             SELECT room_no, CAST(room_date AS DATE) AS night_date, MAX(id) AS max_id \
               FROM HT_Room_Status \
              WHERE room_no IS NOT NULL AND room_date IS NOT NULL{floor_clause} \
              GROUP BY room_no, CAST(room_date AS DATE) \
           ) rep ON t.id = rep.max_id"
    )
}

/// Fetch the deduplicated legacy work-list. READ-ONLY — routed through
/// `query_with_timeout_pooled` (`MssqlOpKind::Read`) per the standing
/// guardrail (issue #275 / #279): a raw, unwrapped bound-`Query` call gets
/// neither the per-op timeout nor the poison-on-timeout flag.
async fn fetch_legacy_worklist(
    mssql: &DbPool,
    floor: Option<NaiveDate>,
) -> Result<Vec<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let sql = dedup_worklist_sql(floor.is_some());
    let floor_text = floor.map(|d| d.to_string());

    let mut conn = mssql.get().await?;
    let mut q = Query::new(sql.as_str());
    if let Some(f) = floor_text.as_deref() {
        q.bind(f);
    }
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    Ok(rows)
}

/// Canonical-side business-key set. Read-only against PG; `impl
/// sqlx::PgExecutor<'_>` so both `main` (a `&PgPool`) and the temp-table
/// unit test below (a transaction) can call it, matching the convention
/// documented in `repository/mod.rs`.
async fn fetch_canonical_keys(
    pg: impl sqlx::PgExecutor<'_>,
) -> Result<HashSet<(String, NaiveDate)>, sqlx::Error> {
    let rows: Vec<(String, NaiveDate)> = sqlx::query_as(CANONICAL_KEYS_SQL).fetch_all(pg).await?;
    Ok(rows.into_iter().collect())
}

/// Canonical coverage floor (`MIN(rcal_date)`). `None` means the calendar
/// is currently empty.
async fn fetch_canonical_floor(
    pg: impl sqlx::PgExecutor<'_>,
) -> Result<Option<NaiveDate>, sqlx::Error> {
    sqlx::query_scalar(CANONICAL_FLOOR_SQL).fetch_one(pg).await
}

/// Minimal legacy-row preview: just the fields needed to compute the
/// business-key diff and print a human-readable line. The real,
/// authoritative projection stays inside `RoomCalendarMapper::apply` (never
/// duplicated here) — this is display/diff-key extraction only, mirroring
/// the `ReceiptPreview` idiom in `backfill_receipt_payments`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidatePreview {
    room_no: String,
    night: NaiveDate,
    room_status: String,
    room_book_no: Option<String>,
    room_checkin_no: Option<String>,
}

fn preview_candidate(row: &dyn MappableRow) -> Result<CandidatePreview, SyncError> {
    let room_no = row
        .try_get_str("room_no")?
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "room_no is NULL — required business key".into(),
        })?
        .to_string();
    let night = row
        .try_get_datetime("room_date")?
        .ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "room_date is NULL — required business key".into(),
        })?
        .date();
    let room_status = row
        .try_get_str("room_status")?
        .ok_or_else(|| SyncError::Mapper { table: TABLE, message: "room_status is NULL".into() })?
        .to_string();
    let room_book_no = row.try_get_str("room_Book_No")?.map(str::to_string);
    let room_checkin_no = row.try_get_str("room_CheckIn_No")?.map(str::to_string);

    Ok(CandidatePreview { room_no, night, room_status, room_book_no, room_checkin_no })
}

/// Decide whether to print this candidate's individual line and, if so,
/// consume one unit of the sample budget. Extracted as a pure helper so the
/// bounded-sample behaviour is directly unit-testable.
fn take_sample_slot(remaining: &mut usize) -> bool {
    if *remaining > 0 {
        *remaining -= 1;
        true
    } else {
        false
    }
}

/// Process one missing candidate: `--dry-run` resolves (room required,
/// booking/checkin optional) via the SAME `sync::resolve` helpers the
/// mapper itself calls, then rolls back — nothing is committed. Live mode
/// re-drives `RoomCalendarMapper::apply` (`ChangeOp::Insert`) and commits.
/// Every branch updates `s`; errors always print in full (never bounded by
/// the sample budget — design §4: "must also surface it loudly").
async fn process_candidate(
    pg: &PgPool,
    row: &tiberius::Row,
    preview: &CandidatePreview,
    dry_run: bool,
    sample_remaining: &mut usize,
    s: &mut Summary,
) {
    let mut tx = match pg.begin().await {
        Ok(t) => t,
        Err(err) => {
            tracing::warn!(
                room_no = %preview.room_no, night = %preview.night, error = %err,
                "pg.begin failed — skipping"
            );
            println!(
                "[ERROR] room={} night={}: pg.begin failed: {err}",
                preview.room_no, preview.night
            );
            s.errored += 1;
            return;
        }
    };

    if dry_run {
        let room_id = resolve::resolve_room_id(&mut tx, Some(preview.room_no.as_str())).await;
        let book_resolved =
            resolve::resolve_booking_id(&mut tx, preview.room_book_no.as_deref())
                .await
                .map(|v| v.is_some());
        let checkin_resolved =
            resolve::resolve_checkin_id(&mut tx, preview.room_checkin_no.as_deref())
                .await
                .map(|v| v.is_some());
        let _ = tx.rollback().await;

        match room_id {
            Ok(Some(room_id)) => {
                s.written += 1;
                if take_sample_slot(sample_remaining) {
                    println!(
                        "[DRY RUN] would write room={} night={} status={} room_id={} \
                         book_resolved={} checkin_resolved={}",
                        preview.room_no,
                        preview.night,
                        preview.room_status,
                        room_id,
                        book_resolved.unwrap_or(false),
                        checkin_resolved.unwrap_or(false)
                    );
                }
            }
            Ok(None) => {
                tracing::warn!(
                    room_no = %preview.room_no, night = %preview.night,
                    "unknown room in canonical ht_rooms_new — live apply would error"
                );
                println!(
                    "[ERROR] room={} night={}: unknown room in canonical ht_rooms_new — \
                     a live run would hold/retry this row (see RoomCalendarMapper docs)",
                    preview.room_no, preview.night
                );
                s.errored += 1;
            }
            Err(err) => {
                tracing::warn!(
                    room_no = %preview.room_no, night = %preview.night, error = %err,
                    "resolve_room_id failed"
                );
                println!(
                    "[ERROR] room={} night={}: resolve_room_id failed: {err}",
                    preview.room_no, preview.night
                );
                s.errored += 1;
            }
        }
        return;
    }

    match RoomCalendarMapper.apply(&mut tx, ChangeOp::Insert, Some(row)).await {
        Ok(_event) => match tx.commit().await {
            Ok(()) => {
                s.written += 1;
                if take_sample_slot(sample_remaining) {
                    println!(
                        "[WRITTEN] room={} night={} status={}",
                        preview.room_no, preview.night, preview.room_status
                    );
                }
            }
            Err(err) => {
                tracing::warn!(
                    room_no = %preview.room_no, night = %preview.night, error = %err,
                    "commit failed"
                );
                println!(
                    "[ERROR] room={} night={}: commit failed: {err}",
                    preview.room_no, preview.night
                );
                s.errored += 1;
            }
        },
        Err(err) => {
            let _ = tx.rollback().await;
            tracing::warn!(
                room_no = %preview.room_no, night = %preview.night, error = %err,
                "RoomCalendarMapper::apply failed"
            );
            println!(
                "[ERROR] room={} night={}: RoomCalendarMapper::apply failed: {err}",
                preview.room_no, preview.night
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
    //! DB-backed test (`fetch_canonical_keys_and_floor_read_temp_fixture`)
    //! self-skips without `DATABASE_URL` — `cargo test --lib`-equivalent bin
    //! tests must stay green on a machine with no database. Mirrors the
    //! `worklist_probe_conn` idiom in `backfill_receipt_payments`.

    use super::*;
    use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

    // -------------------------------------------------------------------
    // Source-scan pin — issue #275 / #279. This bin's only MSSQL reads
    // (`fetch_legacy_worklist`, the `connect_legacy` liveness probe) must
    // route through the timeout+poison wrappers in `db::mssql_timeout`,
    // never a raw unwrapped tiberius call. `scheduler/mod.rs`'s equivalent
    // pin deliberately excludes `bin/*` (operator-invoked, no scheduled
    // exposure) — this is this bin's own guard so a future edit can't
    // silently reintroduce a raw call.
    //
    // Scanned region is PRODUCTION code only (everything before this
    // `#[cfg(test)]` module) — `include_str!` reads this whole file,
    // itself included, so scanning past this point would trip on the
    // needle text these very assertions must contain to name the pattern
    // they're checking for.
    // -------------------------------------------------------------------
    fn production_source() -> &'static str {
        let src = include_str!("backfill_room_calendar.rs");
        let boundary = src.find("#[cfg(test)]").expect("test module marker must exist");
        &src[..boundary]
    }

    #[test]
    fn bin_contains_no_raw_mssql_bypass_calls() {
        for needle in [".simple_query(", ".query(&mut", ".execute(&mut"] {
            assert!(
                !production_source().contains(needle),
                "backfill_room_calendar.rs calls `{needle}` directly — route it \
                 through simple_query_with_timeout_pooled / query_with_timeout_pooled \
                 (db::mssql_timeout) instead (issue #275 / #279)"
            );
        }
    }

    #[test]
    fn bin_imports_the_timeout_wrappers() {
        for needle in ["simple_query_with_timeout_pooled", "query_with_timeout_pooled"] {
            assert!(
                production_source().contains(needle),
                "backfill_room_calendar.rs no longer uses {needle} — if it stopped \
                 reading MSSQL that way entirely, remove the corresponding import \
                 rather than leaving this pin stale"
            );
        }
    }

    // -------------------------------------------------------------------
    // parse_args
    // -------------------------------------------------------------------
    #[test]
    fn parse_args_recognises_dry_run_flag() {
        let a = parse_args(vec!["--dry-run".to_string()]);
        assert!(a.dry_run);
        assert!(!a.all);
        assert_eq!(a.since_raw, None);
    }

    #[test]
    fn parse_args_recognises_all_flag() {
        let a = parse_args(vec!["--all".to_string()]);
        assert!(a.all);
        assert!(!a.dry_run);
    }

    #[test]
    fn parse_args_captures_since_raw_value() {
        let a = parse_args(vec!["--since=2026-01-01".to_string()]);
        assert_eq!(a.since_raw.as_deref(), Some("2026-01-01"));
    }

    #[test]
    fn parse_args_mixes_flags() {
        let a = parse_args(vec![
            "--dry-run".to_string(),
            "--since=2025-04-13".to_string(),
        ]);
        assert!(a.dry_run);
        assert!(!a.all);
        assert_eq!(a.since_raw.as_deref(), Some("2025-04-13"));
    }

    #[test]
    fn parse_args_ignores_unknown_flags() {
        let a = parse_args(vec!["--bogus-flag".to_string()]);
        assert_eq!(a, Args::default());
    }

    #[test]
    fn parse_args_empty_input_defaults_live_and_floored() {
        let a = parse_args(Vec::<String>::new());
        assert_eq!(a, Args::default());
    }

    // -------------------------------------------------------------------
    // dedup_worklist_sql — shape guards (no database needed).
    // -------------------------------------------------------------------
    #[test]
    fn dedup_worklist_sql_groups_by_business_key_and_picks_max_id() {
        let sql = dedup_worklist_sql(false);
        assert!(sql.contains("GROUP BY room_no, CAST(room_date AS DATE)"));
        assert!(sql.contains("MAX(id) AS max_id"));
        assert!(sql.contains("INNER JOIN"));
        assert!(sql.contains("FROM HT_Room_Status"));
    }

    #[test]
    fn dedup_worklist_sql_floored_variant_binds_floor_param() {
        let sql = dedup_worklist_sql(true);
        assert!(sql.contains("@P1"));
        assert!(sql.contains("CAST(room_date AS DATE) >= CAST(@P1 AS DATE)"));
    }

    #[test]
    fn dedup_worklist_sql_unfloored_variant_has_no_floor_param() {
        let sql = dedup_worklist_sql(false);
        assert!(!sql.contains("@P1"));
    }

    #[test]
    fn dedup_worklist_sql_projects_every_column_the_mapper_needs() {
        let sql = dedup_worklist_sql(true);
        for col in
            ["t.id", "t.room_no", "t.room_date", "t.room_status", "t.room_Details", "t.room_Book_No", "t.room_CheckIn_No"]
        {
            assert!(sql.contains(col), "dedup_worklist_sql must project {col}; got: {sql}");
        }
    }

    #[test]
    fn dedup_worklist_sql_excludes_null_business_keys() {
        let sql = dedup_worklist_sql(false);
        assert!(sql.contains("room_no IS NOT NULL"));
        assert!(sql.contains("room_date IS NOT NULL"));
    }

    // -------------------------------------------------------------------
    // CANONICAL_KEYS_SQL / CANONICAL_FLOOR_SQL — shape guards.
    // -------------------------------------------------------------------
    #[test]
    fn canonical_keys_sql_is_unfiltered_on_legacy_id() {
        // Deliberately must NOT filter rcal_legacy_id — app-authored
        // surplus rows must read as "already present" so this bin never
        // touches them.
        assert!(!CANONICAL_KEYS_SQL.contains("rcal_legacy_id"));
        assert!(CANONICAL_KEYS_SQL.contains("FROM ht_room_calendar"));
        assert!(CANONICAL_KEYS_SQL.contains("JOIN ht_rooms_new"));
    }

    #[test]
    fn canonical_floor_sql_selects_min_rcal_date() {
        assert!(CANONICAL_FLOOR_SQL.contains("MIN(rcal_date)"));
        assert!(CANONICAL_FLOOR_SQL.contains("FROM ht_room_calendar"));
    }

    // -------------------------------------------------------------------
    // preview_candidate — pure projection, testable via HashMapRow since
    // `tiberius::Row`'s constructors are private (sync/row.rs docs).
    // -------------------------------------------------------------------
    fn make_row(
        room_no: &str,
        room_date: chrono::NaiveDateTime,
        room_status: &str,
    ) -> HashMapRow {
        HashMapRow::new(TABLE)
            .with("room_no", MockValue::Str(room_no.into()))
            .with("room_date", MockValue::DateTime(room_date))
            .with("room_status", MockValue::Str(room_status.into()))
            .with("room_Book_No", MockValue::Null)
            .with("room_CheckIn_No", MockValue::Null)
    }

    fn naive(y: i32, m: u32, d: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }

    #[test]
    fn preview_candidate_extracts_business_key_and_status() {
        let row = make_row("203", naive(2025, 6, 1), "เข้าพัก");
        let p = preview_candidate(&row).expect("preview must succeed");
        assert_eq!(p.room_no, "203");
        assert_eq!(p.night, NaiveDate::from_ymd_opt(2025, 6, 1).unwrap());
        assert_eq!(p.room_status, "เข้าพัก");
    }

    #[test]
    fn preview_candidate_truncates_room_date_to_calendar_day() {
        let dt = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap().and_hms_opt(15, 45, 0).unwrap();
        let row = make_row("203", dt, "Check Out");
        let p = preview_candidate(&row).expect("preview must succeed");
        assert_eq!(p.night, NaiveDate::from_ymd_opt(2025, 6, 1).unwrap());
    }

    #[test]
    fn preview_candidate_accepts_null_optional_fk_columns() {
        let row = make_row("203", naive(2025, 6, 1), "จอง");
        let p = preview_candidate(&row).expect("preview must succeed");
        assert_eq!(p.room_book_no, None);
        assert_eq!(p.room_checkin_no, None);
    }

    #[test]
    fn preview_candidate_extracts_optional_fk_columns_when_present() {
        let row = make_row("203", naive(2025, 6, 1), "จอง")
            .with("room_Book_No", MockValue::Str("R014838".into()))
            .with("room_CheckIn_No", MockValue::Str("C123456".into()));
        let p = preview_candidate(&row).expect("preview must succeed");
        assert_eq!(p.room_book_no.as_deref(), Some("R014838"));
        assert_eq!(p.room_checkin_no.as_deref(), Some("C123456"));
    }

    #[test]
    fn preview_candidate_errors_when_room_no_is_null() {
        let row = make_row("203", naive(2025, 6, 1), "จอง").with("room_no", MockValue::Null);
        let err = preview_candidate(&row).expect_err("NULL room_no must error");
        assert!(err.to_string().contains("room_no"));
    }

    #[test]
    fn preview_candidate_errors_when_room_date_is_null() {
        let row = make_row("203", naive(2025, 6, 1), "จอง").with("room_date", MockValue::Null);
        let err = preview_candidate(&row).expect_err("NULL room_date must error");
        assert!(err.to_string().contains("room_date"));
    }

    #[test]
    fn preview_candidate_errors_when_room_status_is_null() {
        let row = make_row("203", naive(2025, 6, 1), "จอง").with("room_status", MockValue::Null);
        let err = preview_candidate(&row).expect_err("NULL room_status must error");
        assert!(err.to_string().contains("room_status"));
    }

    // -------------------------------------------------------------------
    // take_sample_slot — pure bounded-sample gate.
    // -------------------------------------------------------------------
    #[test]
    fn take_sample_slot_allows_up_to_the_budget_then_stops() {
        let mut remaining = 2usize;
        assert!(take_sample_slot(&mut remaining));
        assert!(take_sample_slot(&mut remaining));
        assert!(!take_sample_slot(&mut remaining));
        assert!(!take_sample_slot(&mut remaining));
        assert_eq!(remaining, 0);
    }

    #[test]
    fn take_sample_slot_zero_budget_never_allows() {
        let mut remaining = 0usize;
        assert!(!take_sample_slot(&mut remaining));
    }

    // -------------------------------------------------------------------
    // fetch_canonical_keys / fetch_canonical_floor — live behavioural test
    // against a TEMP-shadowed `ht_room_calendar` + `ht_rooms_new`,
    // self-skipping without a reachable DATABASE_URL. Simplified schema
    // (no booking/checkin FKs) — this bin never reads those columns.
    // -------------------------------------------------------------------
    async fn temp_fixture_conn() -> Option<sqlx::PgConnection> {
        use sqlx::Connection;
        let url = std::env::var("DATABASE_URL").ok()?;
        match sqlx::PgConnection::connect(&url).await {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!(
                    "backfill_room_calendar temp-fixture probe SKIPPED — cannot connect to PG: {e}"
                );
                None
            }
        }
    }

    const TEMP_FIXTURE_DDL: &str = "\
        CREATE TEMP TABLE ht_rooms_new ( \
            room_id SERIAL PRIMARY KEY, \
            room_no TEXT NOT NULL \
        ) ON COMMIT DROP; \
        CREATE TEMP TABLE ht_room_calendar ( \
            rcal_id BIGSERIAL PRIMARY KEY, \
            rcal_room_id INTEGER NOT NULL, \
            rcal_date DATE NOT NULL, \
            rcal_status VARCHAR(50) NOT NULL, \
            rcal_legacy_id INTEGER \
        ) ON COMMIT DROP;";

    #[tokio::test]
    async fn fetch_canonical_keys_and_floor_read_temp_fixture() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
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

        sqlx::query(
            "INSERT INTO ht_rooms_new (room_no) VALUES ('101'), ('102'), ('999-surplus-only')",
        )
        .execute(&mut *tx)
        .await
        .expect("seed rooms");

        // 101/2026-01-01 = mapper-authored; 102/2026-01-02 = app-authored
        // surplus (rcal_legacy_id NULL) — both must read as canonical
        // "present" regardless of legacy_id.
        sqlx::query(
            "INSERT INTO ht_room_calendar \
                (rcal_room_id, rcal_date, rcal_status, rcal_legacy_id) \
             VALUES \
                ((SELECT room_id FROM ht_rooms_new WHERE room_no = '101'), \
                 '2026-01-01', 'เข้าพัก', 555), \
                ((SELECT room_id FROM ht_rooms_new WHERE room_no = '102'), \
                 '2026-01-02', 'จอง', NULL)",
        )
        .execute(&mut *tx)
        .await
        .expect("seed calendar rows");

        let keys = fetch_canonical_keys(&mut *tx).await.expect("fetch_canonical_keys");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&("101".to_string(), NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())));
        assert!(
            keys.contains(&("102".to_string(), NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())),
            "app-authored surplus row (rcal_legacy_id NULL) must still count as canonical-present"
        );
        assert!(!keys.contains(&("999-surplus-only".to_string(), NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())));

        let floor = fetch_canonical_floor(&mut *tx).await.expect("fetch_canonical_floor");
        assert_eq!(floor, Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()));

        tx.rollback().await.expect("rollback");
    }

    #[tokio::test]
    async fn fetch_canonical_floor_is_none_when_calendar_empty() {
        let Some(mut conn) = temp_fixture_conn().await else {
            return;
        };
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

        let floor = fetch_canonical_floor(&mut *tx).await.expect("fetch_canonical_floor");
        assert_eq!(floor, None);

        let keys = fetch_canonical_keys(&mut *tx).await.expect("fetch_canonical_keys");
        assert!(keys.is_empty());

        tx.rollback().await.expect("rollback");
    }
}
