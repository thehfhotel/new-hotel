//! One-shot backfill: legacy MSSQL `HT_Rooms` + `HT_SET_RoomType`
//! → PostgreSQL `ht_rooms_new` + `ht_room_types`.
//!
//! Why this exists
//! ---------------
//! The PG `ht_rooms_new` table has been empty in production. The frontend's
//! room picker (booking creation UI) shows nothing, and the writeback worker
//! cannot resolve `room_id → legacy_room_no` lookups for `MarkRoomClean`.
//! This binary mirrors the 58 legacy rooms + 8 room types into PG so the
//! frontend works AND so the writeback adapter has the data it needs.
//!
//! Idempotent. Safe to re-run after legacy room edits.
//!
//! Usage
//! -----
//! ```text
//! docker compose --profile backfill run --rm backfill-rooms
//! ```
//!
//! Or locally:
//! ```text
//! cd hotel-backend
//! DATABASE_URL=postgres://postgres:NewHotel%402026%21@localhost:5439/hotelnew \
//!   DB_SERVER=192.168.100.222 DB_USER=sa DB_PASSWORD=... DB_NAME=db \
//!   cargo run --release --bin backfill_rooms
//! ```
//!
//! Column mapping (verified against `docs/legacy-spike/schema/01-baseline-schema.txt`)
//! ----------------------------------------------------------------------------------
//!
//! `HT_SET_RoomType` (8 rows) → `ht_room_types`:
//!
//! | Legacy column        | PG column           | Notes                                 |
//! |----------------------|---------------------|---------------------------------------|
//! | `id`         (int)   | `type_id`           | preserved — FK target for room.type_id |
//! | `id_full`    (vchar) | `type_code`         | UNIQUE — fall back to `T<id>` if null |
//! | `name`       (vchar) | `type_name`         | also copied into `type_name_en`       |
//! | `Room_PriceA`(float) | `type_base_price`   | weekday/standard rate                 |
//! | (no source col)      | `type_size_sqm`     | NULL — legacy doesn't track sqm      |
//! | (no source col)      | `type_max_guests`   | leaves PG default (2)                 |
//!
//! `HT_Rooms` (58 rows) → `ht_rooms_new`:
//!
//! | Legacy column          | PG column             | Notes                                          |
//! |------------------------|-----------------------|------------------------------------------------|
//! | `id`        (int)      | `room_id`             | preserved — writeback resolves room_id→room_no |
//! | `Room_no`   (vchar)    | `room_no`             | UNIQUE in PG                                   |
//! | `Room_Type` (vchar)    | `room_type_id`        | resolved by name match against ht_room_types   |
//! | `Room_PriceA`(float)   | `room_price_weekday`  |                                                |
//! | `Room_PriceB`(float)   | `room_price_weekend`  |                                                |
//! | `Room_PriceC`(float)   | `room_price_special`  |                                                |
//! | `Room_Use`  ('yes'/'no')| `room_status`        | 'yes' → 'occupied', else → 'available'         |
//! | `Room_Clean`('yes'/'no')| `room_clean`         | INVERTED per spike §3i: 'yes' = needs cleaning |
//! | `Room_Manternace` (sic) ('yes'/'no') | `room_maintenance` | true iff legacy = 'yes'        |
//! | `Room_Group`(vchar)    | `room_building`       | spike notes Room_Group is the building/wing    |
//! | `Room_Details`(vchar)  | `room_notes`          |                                                |
//! | (derived)              | `room_floor`          | first digit of room_no (e.g. "201" → 2)        |
//! | (no source col)        | `room_view`           | NULL — legacy doesn't track view               |
//! | (no source col)        | `room_features`       | NULL                                           |
//! | (no source col)        | `room_active`         | leaves PG default (true)                       |
//!
//! Legacy columns deliberately NOT copied (no canonical PG equivalent — these
//! are operational state owned by the legacy app, not master data):
//!   - `Room_X`, `Room_Y`         — UI grid coordinates for legacy app's map view
//!   - `Room_Use_Count`           — lifetime occupancy counter
//!   - `Room_Polity`              — legacy policy id; semantics unclear
//!   - `Room_Book`, `Room_Book_Name`, `Room_Book_Time`, `Room_Book_ds`
//!                                — current booking ref; comes from booking sync
//!   - `Room_Power_OPEN/CLOSE/STATUS`, `Room_Clean_Time`
//!                                — physical-device state; out of scope for booking UI

use std::collections::HashMap;
use std::env;
use hotel_backend::service::ids::{aggregate_uuid, AggregateKind};

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use sqlx::PgPool;

/// Connection-pool size for the one-shot bin — small on purpose: we issue
/// per-row UPSERTs sequentially, not in parallel.
const POOL_MAX: u32 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backfill_rooms=info".into()),
        )
        .init();

    let legacy = connect_legacy().await?;
    let pg = connect_pg().await?;

    tracing::info!("Connected to legacy MSSQL + PostgreSQL — starting backfill");

    // Load `HT_Rooms_Price` upfront — the legacy app keys prices by
    // (Room_Type, Room_CustType) where Room_CustType is the price tier
    // (`ราคาปกติ` is the default / standard one). HT_Rooms.Room_PriceA is
    // legacy-deprecated and shipped as 0 on every row in production.
    let default_prices = load_default_prices(&legacy).await?;
    let type_stats = backfill_room_types(&legacy, &pg, &default_prices).await?;
    let room_stats = backfill_rooms(&legacy, &pg, &default_prices).await?;
    bump_sequences(&pg).await?;

    tracing::info!(
        "Backfill complete: imported {imported_types} room_types ({updated_types} updates), \
         {imported_rooms} rooms ({updated_rooms} updates)",
        imported_types = type_stats.inserted,
        updated_types = type_stats.updated,
        imported_rooms = room_stats.inserted,
        updated_rooms = room_stats.updated,
    );

    Ok(())
}

#[derive(Default, Debug)]
struct UpsertCounts {
    inserted: i32,
    updated: i32,
}

// =============================================================================
// Step 0: Default-price lookup
// =============================================================================
//
// The legacy app stores the canonical room rate in `HT_Rooms_Price`, NOT
// in `HT_Rooms.Room_PriceA` (which is shipped as 0 on every row in
// production). The schema is `(Room_Type, Room_CustType, Room_Price)`
// where `Room_CustType` is the price tier label — `ราคาปกติ` is the
// standard rate the booking form uses by default.
//
// Returns a map from room type name (e.g. `"เตียงคู่"`) to the standard
// price. Rooms whose type has no standard-price row will have None and
// the per-room `room_price_weekday` is left NULL (so the route's
// fallback chain can pick body.total_amount instead of being short-
// circuited by Some(0.0)).

/// `Room_CustType` literal that selects the "standard" price tier.
const STANDARD_PRICE_TIER: &str = "ราคาปกติ";

async fn load_default_prices(
    legacy: &Pool<ConnectionManager>,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy.get().await?;
    let rows = conn
        .simple_query(
            r#"
            SELECT Room_Type, Room_Price
            FROM HT_Rooms_Price
            WHERE Room_CustType = N'ราคาปกติ'
              AND Room_Price > 0
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut map = HashMap::with_capacity(rows.len());
    for row in &rows {
        let Some(name) = row.get::<&str, _>("Room_Type") else { continue };
        let name = name.trim();
        if name.is_empty() { continue }
        let price = row.get::<f64, _>("Room_Price").unwrap_or(0.0);
        if price <= 0.0 { continue }
        map.insert(name.to_string(), price);
    }
    tracing::info!(
        "  Default prices: loaded {n} room types with non-zero {tier} price",
        n = map.len(),
        tier = STANDARD_PRICE_TIER
    );
    Ok(map)
}

// =============================================================================
// Step 1: Room Types
// =============================================================================

async fn backfill_room_types(
    legacy: &Pool<ConnectionManager>,
    pg: &PgPool,
    default_prices: &HashMap<String, f64>,
) -> Result<UpsertCounts, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT id, id_full, name, Room_PriceA
            FROM HT_SET_RoomType
            ORDER BY id
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut counts = UpsertCounts::default();

    for row in &rows {
        let legacy_id = row.get::<i32, _>("id").ok_or("HT_SET_RoomType.id is null")?;
        let id_full = row.get::<&str, _>("id_full").map(String::from);
        let name = row
            .get::<&str, _>("name")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                format!("HT_SET_RoomType id={legacy_id}: 'name' is null/empty")
            })?
            .to_string();
        // Standard price for this type — sourced from HT_Rooms_Price (the
        // legacy app's actual price table), NOT HT_SET_RoomType.Room_PriceA
        // which is shipped as 0 in production. Falls back to 0 only when
        // there's no `ราคาปกติ` row at all for this type.
        let base_price = default_prices.get(&name).copied().unwrap_or(0.0);

        // type_code is NOT NULL UNIQUE in PG. Prefer id_full; fall back to
        // a deterministic stub so re-runs land on the same row.
        let type_code = id_full
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| format!("T{legacy_id}"));

        let result = sqlx::query(
            r#"
            INSERT INTO ht_room_types
                (type_id, type_code, type_name, type_name_en, type_base_price)
            VALUES ($1, $2, $3, $3, $4::float8)
            ON CONFLICT (type_id) DO UPDATE SET
                type_code        = EXCLUDED.type_code,
                type_name        = EXCLUDED.type_name,
                type_name_en     = EXCLUDED.type_name_en,
                type_base_price  = EXCLUDED.type_base_price,
                type_updated_at  = NOW()
            RETURNING (xmax = 0) AS was_insert
            "#,
        )
        .bind(legacy_id)
        .bind(&type_code)
        .bind(&name)
        .bind(base_price)
        .fetch_one(pg)
        .await?;

        let was_insert: bool = sqlx::Row::try_get(&result, "was_insert")?;
        if was_insert {
            counts.inserted += 1;
        } else {
            counts.updated += 1;
        }
    }

    tracing::info!(
        "  Room types: {} inserted, {} updated ({} total seen)",
        counts.inserted, counts.updated, rows.len()
    );
    Ok(counts)
}

// =============================================================================
// Step 2: Rooms
// =============================================================================

async fn backfill_rooms(
    legacy: &Pool<ConnectionManager>,
    pg: &PgPool,
    default_prices: &HashMap<String, f64>,
) -> Result<UpsertCounts, Box<dyn std::error::Error + Send + Sync>> {
    // Build name → type_id lookup from PG (room_types must be backfilled first).
    let type_lookup = load_type_name_index(pg).await?;
    tracing::debug!("Loaded {} room-type names for FK resolution", type_lookup.len());

    let mut conn = legacy.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT id, Room_no, Room_Type, Room_Details,
                   Room_PriceA, Room_PriceB, Room_PriceC,
                   Room_Use, Room_Clean, Room_Manternace,
                   Room_Group
            FROM HT_Rooms
            ORDER BY id
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut counts = UpsertCounts::default();

    for row in &rows {
        let legacy_id = row.get::<i32, _>("id").ok_or("HT_Rooms.id is null")?;
        let room_no = row
            .get::<&str, _>("Room_no")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| format!("HT_Rooms id={legacy_id}: Room_no is null/empty"))?
            .to_string();

        let room_type_name = row.get::<&str, _>("Room_Type").map(|s| s.trim().to_string());
        let room_type_id = room_type_name
            .as_deref()
            .filter(|s| !s.is_empty())
            .and_then(|name| type_lookup.get(name).copied());

        if room_type_id.is_none() && room_type_name.as_deref().is_some_and(|s| !s.is_empty()) {
            tracing::warn!(
                "  Room {} (id={}): legacy Room_Type '{}' has no matching ht_room_types.type_name — leaving room_type_id NULL",
                room_no, legacy_id, room_type_name.as_deref().unwrap_or("")
            );
        }

        let room_details = row.get::<&str, _>("Room_Details").map(String::from);
        // HT_Rooms.Room_PriceA/B/C are legacy-deprecated and ship as 0 in
        // production. Real prices live in HT_Rooms_Price keyed by Room_Type
        // — we preloaded them into `default_prices`. We assign the standard
        // tier (`ราคาปกติ`) to room_price_weekday; weekend/special are left
        // None until we wire the multi-tier table (HT_Rooms_Price has
        // Room_Price_H / Room_Price_M variants but we don't surface tiers
        // in our UI yet).
        //
        // Leaving NULL (None) when the type has no `ราคาปกติ` entry — the
        // route's `r.room_price_weekday.or(body.total_amount)` chain then
        // correctly falls through to the user-entered total.
        let price_weekday: Option<f64> = room_type_name
            .as_deref()
            .filter(|s| !s.is_empty())
            .and_then(|name| default_prices.get(name).copied())
            .filter(|p| *p > 0.0);
        let price_b: Option<f64> = None;
        let price_c: Option<f64> = None;
        let room_use = row.get::<&str, _>("Room_Use").unwrap_or("no");
        let room_clean_legacy = row.get::<&str, _>("Room_Clean").unwrap_or("no");
        let room_maint_legacy = row.get::<&str, _>("Room_Manternace").unwrap_or("no");
        let room_group = row.get::<&str, _>("Room_Group").map(String::from);

        // Status mapping: legacy Room_Use='yes' → guest is in the room.
        let room_status = if eq_ignore_case_yes(room_use) {
            "occupied"
        } else {
            "available"
        };
        // Cleanliness inversion: spike §3i + §3j confirm Room_Clean='yes'
        // means "needs cleaning". PG's `room_clean` is the positive form
        // (true = clean), so we negate.
        let room_clean = !eq_ignore_case_yes(room_clean_legacy);
        let room_maintenance = eq_ignore_case_yes(room_maint_legacy);

        // Floor: first digit of room_no, e.g. "201" → 2. Skips non-digit-leading
        // room codes (e.g. "VIP1") rather than guessing.
        let room_floor: Option<i32> = room_no
            .chars()
            .next()
            .and_then(|c| c.to_digit(10))
            .map(|d| d as i32);

        // Migration 014: also populate the writeback resolver columns. The
        // legacy room_no IS the canonical room_no (HT_Rooms.room_no in MSSQL),
        // and legacy_room_id_int IS the SERIAL room_id (we preserve HT_Rooms.id
        // → ht_rooms_new.room_id end-to-end). aggregate_id is the deterministic
        // UUID derived from room_id so MarkRoomClean can resolve.
        let aggregate_id = aggregate_uuid(AggregateKind::Room, legacy_id);

        let result = sqlx::query(
            r#"
            INSERT INTO ht_rooms_new (
                room_id, room_no, room_type_id, room_floor, room_building,
                room_status, room_clean, room_maintenance,
                room_price_weekday, room_price_weekend, room_price_special,
                room_notes,
                legacy_room_no, legacy_room_id_int, aggregate_id
            )
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8,
                $9::float8, $10::float8, $11::float8,
                $12,
                $13, $14, $15
            )
            ON CONFLICT (room_id) DO UPDATE SET
                room_no              = EXCLUDED.room_no,
                room_type_id         = EXCLUDED.room_type_id,
                room_floor           = EXCLUDED.room_floor,
                room_building        = EXCLUDED.room_building,
                room_status          = EXCLUDED.room_status,
                room_clean           = EXCLUDED.room_clean,
                room_maintenance     = EXCLUDED.room_maintenance,
                room_price_weekday   = EXCLUDED.room_price_weekday,
                room_price_weekend   = EXCLUDED.room_price_weekend,
                room_price_special   = EXCLUDED.room_price_special,
                room_notes           = EXCLUDED.room_notes,
                legacy_room_no       = EXCLUDED.legacy_room_no,
                legacy_room_id_int   = EXCLUDED.legacy_room_id_int,
                aggregate_id         = EXCLUDED.aggregate_id,
                updated_at           = NOW()
            RETURNING (xmax = 0) AS was_insert
            "#,
        )
        .bind(legacy_id)
        .bind(&room_no)
        .bind(room_type_id)
        .bind(room_floor)
        .bind(&room_group)
        .bind(room_status)
        .bind(room_clean)
        .bind(room_maintenance)
        .bind(price_weekday)
        .bind(price_b)
        .bind(price_c)
        .bind(&room_details)
        .bind(&room_no)
        .bind(legacy_id)
        .bind(aggregate_id)
        .fetch_one(pg)
        .await?;

        let was_insert: bool = sqlx::Row::try_get(&result, "was_insert")?;
        if was_insert {
            counts.inserted += 1;
        } else {
            counts.updated += 1;
        }
    }

    tracing::info!(
        "  Rooms: {} inserted, {} updated ({} total seen)",
        counts.inserted, counts.updated, rows.len()
    );
    Ok(counts)
}

/// Returns: legacy `name` → PG `type_id`. Includes the rows we just inserted.
async fn load_type_name_index(pg: &PgPool) -> Result<HashMap<String, i32>, sqlx::Error> {
    let pairs: Vec<(i32, String)> =
        sqlx::query_as("SELECT type_id, type_name FROM ht_room_types")
            .fetch_all(pg)
            .await?;

    Ok(pairs.into_iter().map(|(id, name)| (name, id)).collect())
}

/// Case-insensitive `'yes'` check tolerant of whitespace.
fn eq_ignore_case_yes(s: &str) -> bool {
    s.trim().eq_ignore_ascii_case("yes")
}

// =============================================================================
// Step 3: Bump SERIAL sequences past the largest preserved id
// =============================================================================
// Without this, the next "create new room from the UI" would collide with a
// preserved legacy id and PG would raise a duplicate-key error on the SERIAL
// default.
async fn bump_sequences(pg: &PgPool) -> Result<(), sqlx::Error> {
    bump_sequence_to_max(pg, "ht_room_types_type_id_seq", "ht_room_types", "type_id").await?;
    bump_sequence_to_max(pg, "ht_rooms_new_room_id_seq", "ht_rooms_new", "room_id").await?;
    Ok(())
}

async fn bump_sequence_to_max(
    pg: &PgPool,
    sequence: &str,
    table: &str,
    pk_col: &str,
) -> Result<(), sqlx::Error> {
    // setval(seq, max(pk)) — third arg defaults to true ("is_called"), so
    // nextval() returns max+1. COALESCE protects against an empty table.
    let sql = format!(
        "SELECT setval('{sequence}', COALESCE((SELECT MAX({pk_col}) FROM {table}), 1))"
    );
    sqlx::query(&sql).execute(pg).await?;
    tracing::info!("  Bumped sequence {} past max({}.{})", sequence, table, pk_col);
    Ok(())
}

// =============================================================================
// Connection helpers
// =============================================================================

async fn connect_legacy() -> Result<Pool<ConnectionManager>, Box<dyn std::error::Error + Send + Sync>>
{
    let server = env::var("DB_SERVER").unwrap_or_else(|_| "192.168.100.222".to_string());
    let database = env::var("DB_NAME").unwrap_or_else(|_| "db".to_string());
    let user = env::var("DB_USER").unwrap_or_else(|_| "sa".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "12345678".to_string());

    let mut config = tiberius::Config::new();
    config.host(&server);
    config.port(1433);
    config.database(&database);
    config.authentication(tiberius::AuthMethod::sql_server(&user, &password));
    config.trust_cert();

    let manager = ConnectionManager::new(config);
    let pool = Pool::builder().max_size(POOL_MAX).build(manager).await?;

    {
        let mut conn = pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
    }
    tracing::info!("Connected to legacy SQL Server at {}", server);

    Ok(pool)
}

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL must be set (postgres://user:pass@host:port/db)")?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(POOL_MAX)
        .connect(&url)
        .await?;

    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Connected to PostgreSQL");

    Ok(pool)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_matcher_is_case_and_whitespace_tolerant() {
        assert!(eq_ignore_case_yes("yes"));
        assert!(eq_ignore_case_yes("YES"));
        assert!(eq_ignore_case_yes(" Yes "));
        assert!(!eq_ignore_case_yes("no"));
        assert!(!eq_ignore_case_yes(""));
        assert!(!eq_ignore_case_yes("y"));
    }

    #[test]
    fn floor_extraction_first_digit() {
        let cases = [
            ("201", Some(2)),
            ("508", Some(5)),
            ("VIP1", None),
            ("", None),
            ("9", Some(9)),
        ];
        for (room_no, expected) in cases {
            let actual: Option<i32> = room_no
                .chars()
                .next()
                .and_then(|c| c.to_digit(10))
                .map(|d| d as i32);
            assert_eq!(actual, expected, "room_no={room_no:?}");
        }
    }

    #[test]
    fn room_status_inverts_legacy_room_use() {
        // 'yes' = guest in room → 'occupied'
        assert_eq!(
            if eq_ignore_case_yes("yes") { "occupied" } else { "available" },
            "occupied"
        );
        assert_eq!(
            if eq_ignore_case_yes("no") { "occupied" } else { "available" },
            "available"
        );
    }

    #[test]
    fn room_clean_inverts_legacy_room_clean() {
        // Spike §3i: legacy 'yes' = needs cleaning → PG room_clean=false.
        assert!(!eq_ignore_case_yes("yes") == false); // sanity
        let needs_cleaning_legacy = "yes";
        let pg_room_clean = !eq_ignore_case_yes(needs_cleaning_legacy);
        assert!(!pg_room_clean, "Room_Clean='yes' must map to room_clean=false");

        let already_clean_legacy = "no";
        let pg_room_clean = !eq_ignore_case_yes(already_clean_legacy);
        assert!(pg_room_clean, "Room_Clean='no' must map to room_clean=true");
    }
}
