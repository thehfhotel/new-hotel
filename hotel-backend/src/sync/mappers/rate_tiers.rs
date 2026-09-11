//! Track F4 — periodic-poll mapper for legacy `HT_Rooms_Price`.
//!
//! `HT_Rooms_Price` is the legacy `(Room_Type, Room_CustType)` pricing
//! matrix referenced by the .NET app every time it draws the booking
//! form's "ราคา / คืน" (price per night), "ราคา / ชม." (per-hour
//! extension), and "ราคา / เดือน" (monthly) columns. Per
//! `docs/coexistence/audit-2026-05-13.md` T1 CRIT-4 the canonical PG
//! `ht_rates` table modeled the wrong axis (weekday/weekend/special)
//! and could not represent any non-default customer-type tier.
//!
//! ## Why poll, not Change Tracking
//!
//! `HT_Rooms_Price` is not CT-enabled (the legacy_mssql phases enabled
//! CT on the 10 transactional tables; this 32-row pricing matrix
//! changes on the order of weeks, not seconds). The pre-existing
//! `crate::scheduler::mirror::reload_rooms_price` already does a
//! full-table DELETE+INSERT into `legacy_mirror.ht_rooms_price` every
//! 15 minutes — F4 piggybacks that cadence with one more UPSERT into
//! the canonical `ht_rate_tiers`.
//!
//! ## Mapping
//!
//! | MSSQL `HT_Rooms_Price` | PG `ht_rate_tiers`         |
//! |------------------------|----------------------------|
//! | `id`                   | `rate_tier_legacy_id`      |
//! | `Room_Type`            | `rate_tier_room_type`      |
//! | `Room_CustType`        | `rate_tier_cust_type`      |
//! | `Room_Price`           | `rate_tier_price`          |
//! | `Room_Price_H`         | `rate_tier_price_hourly`   |
//! | `Room_Price_M`         | `rate_tier_price_monthly`  |
//!
//! The UPSERT key is `(rate_tier_room_type, rate_tier_cust_type)` — the
//! composite natural key, NOT the legacy `id`. If legacy ever renames
//! a row (deletes + re-inserts with new id), the canonical row stays
//! pinned to the same `(room_type, cust_type)` and just gets its
//! `legacy_id` updated.
//!
//! ## Rows with NULL or empty keys
//!
//! Skipped silently. The canonical UNIQUE constraint forbids NULL pair
//! columns, and legacy occasionally seeds blank rows during admin UI
//! sessions; we don't want a 100ms transient blank to pollute the
//! canonical row count.
//!
//! ## Pruning legacy-deleted tiers (issue #270)
//!
//! Until #270 this mapper was UPSERT-only: a tier deleted in iHOTEL kept
//! its canonical row forever, and since `routes/new_rates.rs::list_rates`
//! reads `WHERE rate_tier_active = true` that row kept *serving* its
//! last-known price. The prune below closes that.
//!
//! The sibling [`crate::sync::mappers::products`] poll deliberately does
//! NOT prune, for three independent reasons. All three were re-checked
//! against `ht_rate_tiers` and only one survives — hence the different
//! shape here:
//!
//! 1. **FK abort — does NOT apply.** `ht_products.prod_id` is the target
//!    of two `NOT NULL` no-cascade FKs, so a DELETE aborts the reload.
//!    Nothing anywhere references `ht_rate_tiers`: `rate_tier_id` is a
//!    PG-only `BIGSERIAL` with no inbound `REFERENCES` in
//!    `migrations/pg/` or `init-db/init-hotelnew.sql` (verified for
//!    #270), and `outbox/intent.rs` keys the rate writeback on the
//!    composite `(room_type, cust_type)`, never on the surrogate id.
//!    A DELETE here executes cleanly.
//! 2. **Absence ≠ deletion — DOES apply.** iHOTEL edits this table with
//!    delete-then-reinsert on `FrmSETRoomType` / `FrmSETCsuType` saves
//!    (`docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_Rooms_Price` +
//!    §3.25 hard-delete list). A poll landing between those statements
//!    reads an incomplete matrix. This is the real hazard, and
//!    [`PRUNE_GRACE_MINUTES`] is the answer — see below.
//! 3. **Canonical-only rows — does NOT apply to the pruned subset.**
//!    `routes/new_rates.rs::create_rate_tier` can mint a tier that has no
//!    legacy counterpart (notably `active: false`, which deliberately
//!    enqueues no writeback). Those rows are structurally distinguishable:
//!    the canonical INSERT never populates `rate_tier_legacy_id`, and only
//!    [`apply_rate_tier_rows`] ever does. Scoping the prune to
//!    `rate_tier_legacy_id IS NOT NULL` means it can only ever remove a
//!    row this mapper itself mirrored in from legacy.
//!
//! ### Hard delete, not soft-close
//!
//! `rate_tier_active` already exists (migration 042) and issue #270
//! floated a soft-close. Rejected: `rate_tier_active` is **operator-owned
//! local state** — `routes/new_rates.rs::update_rate_tier` writes it and
//! a deactivation deliberately stays canonical-only. A poll that wrote it
//! would either (a) never re-activate, so one mid-edit race disables a
//! live tier permanently, or (b) re-activate on the next UPSERT, silently
//! stomping an operator's archive decision every 15 minutes. This is the
//! same discipline `products.rs` applies to `prod_active`: the poll never
//! touches the flag. A hard DELETE, by contrast, is **self-correcting** —
//! the next tick re-inserts whatever legacy still has.
//!
//! ### Two guards
//!
//! * **Grace window** ([`PRUNE_GRACE_MINUTES`]) — a row is only pruned
//!   once it has gone unrefreshed for longer than the reload cadence, so
//!   a delete-then-reinsert window (milliseconds in iHOTEL) can never
//!   reach it. Costs no new state: the UPSERT already stamps
//!   `rate_tier_updated_at = NOW()` on every present row each tick, so a
//!   stale timestamp *is* the "absent for ≥1 full tick" signal.
//! * **Empty-batch guard** ([`PruneDecision::SkipEmptyBatch`]) — a tick
//!   that yields zero usable keys prunes NOTHING. Deliberate choice of
//!   *refuse-to-wipe* over *all-gone*: `HT_Rooms_Price` has held 32 rows
//!   since 2015 (`docs/legacy-spike/schema/01-baseline-schema.txt`) and a
//!   genuinely empty legacy pricing matrix would mean iHOTEL cannot price
//!   a room at all, so an empty read is overwhelmingly more likely to be
//!   a fault (blocked read, truncated result) than the truth. If it ever
//!   IS the truth, an operator clears the canonical table by hand.
//!
//! Rows legacy still has but that fail [`is_acceptable_row`] (e.g. a
//! transient zero price mid-edit) are counted as *present* for prune
//! purposes — [`retained_keys`] intentionally keys off the raw legacy
//! pair, not the acceptance verdict, so a bad price never escalates into
//! a deletion.

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use crate::db::DbPool;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::time::Instant;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

/// One owned row read from `HT_Rooms_Price`.
///
/// Public for unit-tests in `tests/test_sync_track_f4_apply.rs` —
/// production callers build it inline from a `tiberius::Row` inside
/// [`fetch_legacy_rows`].
#[derive(Debug, Clone, PartialEq)]
pub struct RateTierRow {
    pub legacy_id: i32,
    pub room_type: String,
    pub cust_type: String,
    pub price: f64,
    pub price_hourly: Option<f64>,
    pub price_monthly: Option<f64>,
}

/// Aggregate result of one poll tick.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ReloadStats {
    /// Rows that hit the canonical UPSERT path. Either inserted or
    /// updated — sqlx doesn't tell us which after the ON CONFLICT, and
    /// we don't need to distinguish for observability today.
    pub upserted: i64,
    /// Rows the projection rejected before reaching PG: NULL pk, empty
    /// `Room_Type` or `Room_CustType`, or a non-positive `Room_Price`.
    /// Logged in aggregate so an operator can spot a sudden surge of
    /// invalid legacy rows.
    pub skipped: i64,
    /// Canonical rows deleted because legacy no longer has their
    /// `(room_type, cust_type)` pair (issue #270). Only ever counts
    /// mirror-owned rows — see the module doc's prune section. Normally
    /// 0; any non-zero value is also emitted as a per-key `warn!`.
    pub pruned: i64,
    /// `true` when the empty-batch guard tripped: the tick produced no
    /// usable legacy keys, so the prune was skipped wholesale rather than
    /// interpreted as "legacy deleted everything".
    pub prune_skipped: bool,
}

/// How long a canonical row must go unrefreshed before the prune may
/// remove it.
///
/// Must exceed one reload cadence. The reload rides `run_sync`, i.e.
/// `WORKER_RECONCILE_INTERVAL_SECS` (default 900s = 15 min, floored at
/// 60s in `bin/sync.rs`), so 30 minutes is ≥2 consecutive misses at the
/// default and many more at the floor. The UPSERT stamps
/// `rate_tier_updated_at = NOW()` on every row legacy still has, so only
/// a row genuinely absent across that whole window goes stale — which is
/// what makes an iHOTEL delete-then-reinsert (milliseconds) invisible to
/// the prune.
pub(crate) const PRUNE_GRACE_MINUTES: i32 = 30;

/// The prune statement. Hoisted to a `&'static str` const so the shape
/// tests below can pin its guards without a database — and so it stays
/// on sqlx's `SqlSafeStr` fast path (no `AssertSqlSafe` wrapper).
///
/// Three predicates, each load-bearing:
/// * `rate_tier_legacy_id IS NOT NULL` — mirror-owned rows only, so an
///   operator's canonical-only tier is structurally out of reach.
/// * the grace window — absorbs iHOTEL's delete-then-reinsert edit.
/// * `NOT EXISTS (… UNNEST …)` — the actual "absent from this tick's
///   legacy batch" test, over the raw composite key.
///
/// `rate_tier_active` appears nowhere: the flag is operator-owned and
/// this mapper never writes it (locked by a test below).
pub(crate) const PRUNE_SQL: &str = "DELETE FROM ht_rate_tiers AS t \
     WHERE t.rate_tier_legacy_id IS NOT NULL \
       AND t.rate_tier_updated_at < NOW() - ($1::int * INTERVAL '1 minute') \
       AND NOT EXISTS ( \
             SELECT 1 FROM UNNEST($2::text[], $3::text[]) AS k(room_type, cust_type) \
              WHERE k.room_type = t.rate_tier_room_type \
                AND k.cust_type = t.rate_tier_cust_type \
           ) \
     RETURNING t.rate_tier_room_type, t.rate_tier_cust_type, t.rate_tier_legacy_id";

/// What the prune pass should do with one tick's batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PruneDecision {
    /// Delete mirror-owned rows whose composite key is absent from this
    /// (non-empty, de-duplicated) set.
    Prune(Vec<(String, String)>),
    /// Empty-batch guard — the tick yielded no usable legacy key, so
    /// nothing is deleted. See the module doc: refuse-to-wipe.
    SkipEmptyBatch,
}

/// The composite keys this tick proves legacy still has.
///
/// PURE. Two deliberate properties:
///
/// * **Raw, not trimmed.** Legacy stores padded values (`'เตียงคู่ '` —
///   see `docs/legacy-spike/raw/walkin-*/07-events.txt`) and the UPSERT
///   binds them verbatim, so the canonical key includes the padding.
///   Trimming here would fail to match the row we just wrote and delete
///   it one grace window later.
/// * **Acceptance-independent.** A row legacy has but whose price fails
///   [`is_acceptable_row`] is still *present*; only a blank composite key
///   is dropped (it cannot match any canonical row anyway, since neither
///   write path can produce one).
///
/// Order-preserving and de-duplicated — legacy has no UNIQUE on the pair.
pub(crate) fn retained_keys(rows: &[RateTierRow]) -> Vec<(String, String)> {
    let mut seen: HashSet<(&str, &str)> = HashSet::with_capacity(rows.len());
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        if row.room_type.trim().is_empty() || row.cust_type.trim().is_empty() {
            continue;
        }
        if seen.insert((row.room_type.as_str(), row.cust_type.as_str())) {
            out.push((row.room_type.clone(), row.cust_type.clone()));
        }
    }
    out
}

/// Apply the empty-batch guard to [`retained_keys`]. PURE, so the
/// refuse-to-wipe contract is pinned without a database.
pub(crate) fn prune_decision(rows: &[RateTierRow]) -> PruneDecision {
    let keys = retained_keys(rows);
    if keys.is_empty() {
        PruneDecision::SkipEmptyBatch
    } else {
        PruneDecision::Prune(keys)
    }
}

/// Top-level entry point — fetch from MSSQL, project, UPSERT one TX.
///
/// Wired into [`crate::scheduler::mirror::reload_mirror_dimensions`]
/// so it runs on the same 15-minute reconcile cadence as the
/// `legacy_mirror.*` reloads. Errors propagate to the caller, which
/// already logs `[Mirror] reload ...` failures without aborting the
/// other dimension reloads.
pub async fn reload_rate_tiers(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let start = Instant::now();
    let rows = fetch_legacy_rows(legacy_pool).await?;
    let stats = apply_rate_tier_rows(pg_pool, &rows).await?;
    tracing::info!(
        table = "HT_Rooms_Price",
        upserted = stats.upserted,
        skipped = stats.skipped,
        pruned = stats.pruned,
        prune_skipped = stats.prune_skipped,
        duration_ms = start.elapsed().as_millis(),
        "[Mirror] reloaded ht_rate_tiers"
    );
    Ok(())
}

/// Pull every row from legacy `HT_Rooms_Price` and project into owned
/// `RateTierRow`s. The full-table scan is fine — this table tops out at
/// ~32 rows in production (per `docs/legacy-app/SCHEMA.sql`).
async fn fetch_legacy_rows(legacy_pool: &DbPool) -> Result<Vec<RateTierRow>, AnyError> {
    let mut conn = legacy_pool.get().await?;
    // R2 (2026-05-14): wrap in per-op read timeout. Table is tiny
    // (~32 rows) and the call is mirror-job-only, but a stuck poll
    // would otherwise pin the mirror scheduler.
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M \
         FROM HT_Rooms_Price",
        MssqlOpKind::Read,
    )
    .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        let Some(legacy_id): Option<i32> = r.get(0) else {
            continue;
        };
        let room_type: Option<&str> = r.try_get(1).ok().flatten();
        let cust_type: Option<&str> = r.try_get(2).ok().flatten();
        let price: Option<f64> = r.try_get(3).ok().flatten();
        let price_hourly: Option<f64> = r.try_get(4).ok().flatten();
        let price_monthly: Option<f64> = r.try_get(5).ok().flatten();

        out.push(RateTierRow {
            legacy_id,
            room_type: room_type.unwrap_or_default().to_string(),
            cust_type: cust_type.unwrap_or_default().to_string(),
            price: price.unwrap_or(0.0),
            price_hourly,
            price_monthly,
        });
    }
    Ok(out)
}

/// Apply a batch of projected rows to PG in one transaction. Public so
/// integration tests can feed pre-built `RateTierRow`s without needing
/// a live MSSQL connection.
pub async fn apply_rate_tier_rows(
    pg_pool: &PgPool,
    rows: &[RateTierRow],
) -> Result<ReloadStats, AnyError> {
    let mut tx = pg_pool.begin().await?;
    let mut stats = ReloadStats::default();

    for row in rows {
        if !is_acceptable_row(row) {
            stats.skipped += 1;
            continue;
        }
        sqlx::query(
            "INSERT INTO ht_rate_tiers \
                (rate_tier_room_type, rate_tier_cust_type, rate_tier_price, \
                 rate_tier_price_hourly, rate_tier_price_monthly, rate_tier_legacy_id) \
             VALUES ($1, $2, $3::numeric, $4::numeric, $5::numeric, $6) \
             ON CONFLICT (rate_tier_room_type, rate_tier_cust_type) DO UPDATE SET \
                rate_tier_price         = EXCLUDED.rate_tier_price, \
                rate_tier_price_hourly  = EXCLUDED.rate_tier_price_hourly, \
                rate_tier_price_monthly = EXCLUDED.rate_tier_price_monthly, \
                rate_tier_legacy_id     = EXCLUDED.rate_tier_legacy_id, \
                rate_tier_updated_at    = NOW()",
        )
        .bind(&row.room_type)
        .bind(&row.cust_type)
        .bind(row.price)
        .bind(row.price_hourly)
        .bind(row.price_monthly)
        .bind(row.legacy_id)
        .execute(&mut *tx)
        .await?;
        stats.upserted += 1;
    }

    // Issue #270 — prune tiers legacy no longer has. Runs INSIDE the same
    // TX as the UPSERT pass, and deliberately AFTER it: the UPSERT has
    // just stamped `rate_tier_updated_at = NOW()` on every row legacy
    // still has, so the grace-window predicate can only match a row this
    // batch did not refresh. Atomic with the upsert — a reader never sees
    // a half-reconciled matrix.
    match prune_decision(rows) {
        PruneDecision::SkipEmptyBatch => {
            stats.prune_skipped = true;
            tracing::warn!(
                table = "HT_Rooms_Price",
                fetched = rows.len(),
                "[Mirror] rate-tier prune SKIPPED — legacy batch yielded no usable \
                 (room_type, cust_type) key; refusing to wipe canonical ht_rate_tiers"
            );
        }
        PruneDecision::Prune(keys) => {
            let (room_types, cust_types): (Vec<String>, Vec<String>) = keys.into_iter().unzip();
            let deleted = sqlx::query(PRUNE_SQL)
                .bind(PRUNE_GRACE_MINUTES)
                .bind(room_types)
                .bind(cust_types)
                .fetch_all(&mut *tx)
                .await?;

            // Deletion is the one destructive thing this mapper does and
            // it should be rare (weeks-scale table), so name every key.
            for row in &deleted {
                let room_type: String = row.try_get("rate_tier_room_type").unwrap_or_default();
                let cust_type: String = row.try_get("rate_tier_cust_type").unwrap_or_default();
                let legacy_id: Option<i32> = row.try_get("rate_tier_legacy_id").unwrap_or(None);
                tracing::warn!(
                    table = "HT_Rooms_Price",
                    room_type = room_type.as_str(),
                    cust_type = cust_type.as_str(),
                    legacy_id,
                    grace_minutes = PRUNE_GRACE_MINUTES,
                    "[Mirror] pruned ht_rate_tiers row — absent from legacy \
                     HT_Rooms_Price for longer than the grace window"
                );
            }
            stats.pruned = deleted.len() as i64;
        }
    }

    tx.commit().await?;
    Ok(stats)
}

/// Reject blank-key and non-positive-price rows.
///
/// Pure function so [`tests`] below can pin the contract without
/// touching a database.
fn is_acceptable_row(row: &RateTierRow) -> bool {
    if row.room_type.trim().is_empty() {
        return false;
    }
    if row.cust_type.trim().is_empty() {
        return false;
    }
    if !row.price.is_finite() || row.price <= 0.0 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_row() -> RateTierRow {
        RateTierRow {
            legacy_id: 1,
            room_type: "เตียงคู่".to_string(),
            cust_type: "ราคาปกติ".to_string(),
            price: 800.0,
            price_hourly: Some(200.0),
            price_monthly: Some(15000.0),
        }
    }

    #[test]
    fn accepts_well_formed_row() {
        assert!(is_acceptable_row(&sample_row()));
    }

    #[test]
    fn rejects_blank_room_type() {
        let row = RateTierRow {
            room_type: String::new(),
            ..sample_row()
        };
        assert!(!is_acceptable_row(&row));
    }

    #[test]
    fn rejects_whitespace_only_cust_type() {
        let row = RateTierRow {
            cust_type: "   ".to_string(),
            ..sample_row()
        };
        assert!(!is_acceptable_row(&row));
    }

    #[test]
    fn rejects_zero_price() {
        let row = RateTierRow {
            price: 0.0,
            ..sample_row()
        };
        assert!(!is_acceptable_row(&row));
    }

    #[test]
    fn rejects_negative_price() {
        let row = RateTierRow {
            price: -100.0,
            ..sample_row()
        };
        assert!(!is_acceptable_row(&row));
    }

    #[test]
    fn rejects_nan_price() {
        let row = RateTierRow {
            price: f64::NAN,
            ..sample_row()
        };
        assert!(!is_acceptable_row(&row));
    }

    #[test]
    fn accepts_missing_optional_columns() {
        let row = RateTierRow {
            price_hourly: None,
            price_monthly: None,
            ..sample_row()
        };
        assert!(is_acceptable_row(&row));
    }

    #[test]
    fn reload_stats_default_is_zero() {
        let stats = ReloadStats::default();
        assert_eq!(stats.upserted, 0);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.pruned, 0);
        assert!(!stats.prune_skipped);
    }

    // ------------------------------------------------------------------
    // Issue #270 — prune of legacy-deleted tiers.
    // ------------------------------------------------------------------

    fn row(room_type: &str, cust_type: &str, legacy_id: i32, price: f64) -> RateTierRow {
        RateTierRow {
            legacy_id,
            room_type: room_type.to_string(),
            cust_type: cust_type.to_string(),
            price,
            price_hourly: None,
            price_monthly: None,
        }
    }

    /// The core #270 contract: a tier legacy dropped is NOT in the
    /// retained set, so the `NOT EXISTS` arm deletes it — while every
    /// tier legacy still has IS retained and therefore untouched.
    #[test]
    fn deleted_tier_drops_out_of_retained_keys_and_survivors_stay() {
        let before = vec![
            row("เตียงคู่", "ราคาปกติ", 1, 800.0),
            row("เตียงคู่", "บริษัท", 2, 700.0),
            row("เตียงเดี่ยว", "ราคาปกติ", 3, 600.0),
        ];
        // iHOTEL deletes the corporate tier for เตียงคู่.
        let after: Vec<RateTierRow> = before
            .iter()
            .filter(|r| r.cust_type != "บริษัท")
            .cloned()
            .collect();

        let keys = retained_keys(&after);
        assert!(
            !keys.contains(&("เตียงคู่".to_string(), "บริษัท".to_string())),
            "deleted tier must fall out of the retained set so the prune removes it"
        );
        assert!(keys.contains(&("เตียงคู่".to_string(), "ราคาปกติ".to_string())));
        assert!(keys.contains(&("เตียงเดี่ยว".to_string(), "ราคาปกติ".to_string())));
        assert_eq!(keys.len(), 2, "surviving tiers must all be retained");
    }

    /// A legacy row whose price the projection rejects is still PRESENT
    /// in legacy — a transient zero price mid-edit must never escalate
    /// into a deletion.
    #[test]
    fn retained_keys_keep_price_rejected_rows() {
        let rows = vec![row("เตียงคู่", "ราคาปกติ", 1, 0.0)];
        assert!(!is_acceptable_row(&rows[0]), "fixture must be UPSERT-rejected");
        assert_eq!(
            retained_keys(&rows),
            vec![("เตียงคู่".to_string(), "ราคาปกติ".to_string())],
            "a bad price is not a deletion"
        );
    }

    /// Legacy stores padded values (`'เตียงคู่ '`) and the UPSERT binds
    /// them verbatim, so the retained key must carry the same padding or
    /// the prune would delete the row the same tick wrote.
    #[test]
    fn retained_keys_preserve_untrimmed_legacy_padding() {
        let rows = vec![row("เตียงคู่ ", "ราคาปกติ", 1, 800.0)];
        assert_eq!(
            retained_keys(&rows),
            vec![("เตียงคู่ ".to_string(), "ราคาปกติ".to_string())]
        );
    }

    #[test]
    fn retained_keys_drop_blank_key_rows() {
        let rows = vec![
            row("", "ราคาปกติ", 1, 800.0),
            row("เตียงคู่", "   ", 2, 800.0),
            row("เตียงคู่", "ราคาปกติ", 3, 800.0),
        ];
        assert_eq!(
            retained_keys(&rows),
            vec![("เตียงคู่".to_string(), "ราคาปกติ".to_string())]
        );
    }

    #[test]
    fn retained_keys_dedupe_and_preserve_order() {
        let rows = vec![
            row("B", "ปกติ", 1, 1.0),
            row("A", "ปกติ", 2, 1.0),
            row("B", "ปกติ", 3, 1.0),
        ];
        assert_eq!(
            retained_keys(&rows),
            vec![
                ("B".to_string(), "ปกติ".to_string()),
                ("A".to_string(), "ปกติ".to_string())
            ]
        );
    }

    /// Empty-batch guard, decided deliberately: refuse-to-wipe. A tick
    /// that reads zero rows must NOT be read as "legacy deleted the whole
    /// pricing matrix".
    #[test]
    fn empty_legacy_batch_refuses_to_prune() {
        assert_eq!(prune_decision(&[]), PruneDecision::SkipEmptyBatch);
    }

    /// Same guard on the pathological all-blank read — no usable key is
    /// indistinguishable from no rows, and both must be inert.
    #[test]
    fn all_blank_key_batch_refuses_to_prune() {
        let rows = vec![row("", "", 1, 800.0), row("  ", "ราคาปกติ", 2, 800.0)];
        assert_eq!(prune_decision(&rows), PruneDecision::SkipEmptyBatch);
    }

    #[test]
    fn non_empty_batch_prunes_against_its_keys() {
        let rows = vec![row("เตียงคู่", "ราคาปกติ", 1, 800.0)];
        assert_eq!(
            prune_decision(&rows),
            PruneDecision::Prune(vec![("เตียงคู่".to_string(), "ราคาปกติ".to_string())])
        );
    }

    // --- PRUNE_SQL shape locks ---------------------------------------

    /// Scoping to `rate_tier_legacy_id IS NOT NULL` is what keeps an
    /// operator's canonical-only tier (created via `POST /api/rate-tiers`,
    /// which never populates `rate_tier_legacy_id`) out of reach.
    #[test]
    fn prune_sql_is_scoped_to_mirror_owned_rows() {
        assert!(
            PRUNE_SQL.contains("rate_tier_legacy_id IS NOT NULL"),
            "prune must never reach canonical-only rows"
        );
    }

    #[test]
    fn prune_sql_has_grace_window_and_key_absence_test() {
        assert!(PRUNE_SQL.contains("rate_tier_updated_at <"));
        assert!(PRUNE_SQL.contains("INTERVAL '1 minute'"));
        assert!(PRUNE_SQL.contains("NOT EXISTS"));
        assert!(PRUNE_SQL.contains("UNNEST($2::text[], $3::text[])"));
    }

    /// Mirrors `products.rs`'s `prod_active` lock: `rate_tier_active` is
    /// operator-owned state and the poll must never write it.
    #[test]
    fn prune_sql_never_touches_the_operator_owned_active_flag() {
        assert!(!PRUNE_SQL.contains("rate_tier_active"));
    }

    /// Bounded, key-scoped DELETE — never a TRUNCATE or an unfiltered
    /// wipe. A missing WHERE here would empty the pricing matrix.
    #[test]
    fn prune_sql_is_a_bounded_delete() {
        assert!(PRUNE_SQL.starts_with("DELETE FROM ht_rate_tiers"));
        assert!(!PRUNE_SQL.contains("TRUNCATE"));
        assert!(PRUNE_SQL.contains(" WHERE "));
    }

    /// The grace window only absorbs iHOTEL's delete-then-reinsert if it
    /// spans more than one reload cadence (`WORKER_RECONCILE_INTERVAL_SECS`,
    /// default 900s = 15 min).
    #[test]
    fn prune_grace_exceeds_one_reload_cadence() {
        assert!(
            PRUNE_GRACE_MINUTES > 15,
            "grace must exceed the 15-minute default reload cadence"
        );
    }
}
