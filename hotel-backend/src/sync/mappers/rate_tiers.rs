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

use crate::db::DbPool;
use sqlx::PgPool;
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
    let rows = conn
        .simple_query(
            "SELECT id, Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M \
             FROM HT_Rooms_Price",
        )
        .await?
        .into_first_result()
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
    }
}
