//! Phase 5.5d — read-only HTTP endpoints surfacing the
//! `legacy_mirror.*` schema (coupons, in-stay POS / minibar, room
//! moves, pricing-tier reference data).
//!
//! Per architecture §11, these tables are populated by the CT watcher
//! (Phase 5.5c CT mappers) plus a one-time bootstrap snapshot
//! (Phase 5.5c-b). Our app NEVER writes to them — these are
//! opaque pass-through reads of legacy-only features so receptionists
//! can see the full picture in our UI without switching to the .NET
//! app for coupons / minibar / room-move history / pricing tiers.
//!
//! On decommission the schema is dropped and these endpoints can be
//! deleted in one PR.

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::mode::{AppState, Branch};
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct CinNoQuery {
    /// Legacy check-in number (e.g. `"CH26-005258"`). The UI passes
    /// the value it already has from the loaded check-in object —
    /// keeps this endpoint decoupled from our PG UUIDs.
    pub cin_no: String,
    /// Hotel branch selector. Routes the read to the correct PG pool
    /// (`new_pool` for HF Hotel / All / unset, `ville_pool` for HF
    /// Ville). Frontend's `useBranchFetch` auto-appends this from the
    /// active `BranchContext`.
    pub branch: Option<Branch>,
}

#[derive(Debug, Deserialize)]
pub struct BranchOnlyQuery {
    /// Hotel branch selector. Same dispatch rules as `CinNoQuery::branch`.
    pub branch: Option<Branch>,
}

// ─── Coupons ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponRow {
    pub cupon_no: i32,
    pub cupon_cin_no: Option<String>,
    pub cupon_cin_room: Option<String>,
    pub cupon_date: Option<NaiveDateTime>,
    pub cupon_gen_date: Option<NaiveDateTime>,
    pub cupon_by: Option<String>,
    pub cupon_print: i32,
}

/// `GET /api/legacy-mirror/coupons?cin_no=…` — coupons (food /
/// breakfast vouchers) attached to one check-in.
pub async fn list_coupons(
    State(state): State<AppState>,
    Query(q): Query<CinNoQuery>,
) -> ApiResult<Json<Vec<CouponRow>>> {
    let pool = match q.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        _ => &state.new_pool,
    };

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<NaiveDateTime>, Option<NaiveDateTime>, Option<String>, i32)>(
        "SELECT cupon_no, cupon_cin_no, cupon_cin_room, cupon_date, \
                cupon_gen_date, cupon_by, cupon_print \
           FROM legacy_mirror.ht_cupon \
          WHERE cupon_cin_no = $1 \
          ORDER BY cupon_no",
    )
    .bind(&q.cin_no)
    .fetch_all(pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|(no, cin, room, date, gen, by, print)| CouponRow {
                cupon_no: no,
                cupon_cin_no: cin,
                cupon_cin_room: room,
                cupon_date: date,
                cupon_gen_date: gen,
                cupon_by: by,
                cupon_print: print,
            })
            .collect(),
    ))
}

// ─── In-stay POS / Minibar charges ───────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRow {
    pub id: i32,
    pub cin_no: Option<String>,
    pub cin_room_no: Option<String>,
    pub cin_ds_date: Option<NaiveDateTime>,
    pub cin_pro_id: Option<String>,
    pub cin_pro_name: Option<String>,
    pub cin_pro_unit: Option<String>,
    pub cin_pro_num: Option<f64>,
    pub cin_pro_price: Option<f64>,
    pub cin_pro_pricetotal: Option<f64>,
    pub cin_pro_pay: Option<f64>,
    pub cin_pro_note: Option<String>,
}

/// `GET /api/legacy-mirror/products?cin_no=…` — in-stay POS / minibar
/// charges per check-in.
pub async fn list_products(
    State(state): State<AppState>,
    Query(q): Query<CinNoQuery>,
) -> ApiResult<Json<Vec<ProductRow>>> {
    let pool = match q.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        _ => &state.new_pool,
    };

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<NaiveDateTime>, Option<String>, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<String>)>(
        "SELECT id, cin_no, cin_room_no, cin_ds_date, cin_pro_id, \
                cin_pro_name, cin_pro_unit, cin_pro_num, cin_pro_price, \
                cin_pro_pricetotal, cin_pro_pay, cin_pro_note \
           FROM legacy_mirror.ht_checkin_product \
          WHERE cin_no = $1 \
          ORDER BY cin_ds_date NULLS LAST, id",
    )
    .bind(&q.cin_no)
    .fetch_all(pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|(id, cin, room, date, pid, name, unit, num, price, total, pay, note)| ProductRow {
                id,
                cin_no: cin,
                cin_room_no: room,
                cin_ds_date: date,
                cin_pro_id: pid,
                cin_pro_name: name,
                cin_pro_unit: unit,
                cin_pro_num: num,
                cin_pro_price: price,
                cin_pro_pricetotal: total,
                cin_pro_pay: pay,
                cin_pro_note: note,
            })
            .collect(),
    ))
}

// ─── Mid-stay room moves ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomChangeRow {
    pub id: i32,
    pub cin_no: String,
    pub room_before: Option<String>,
    pub room_after: Option<String>,
    pub change_date: Option<NaiveDateTime>,
    pub room_before_price: f64,
    pub note: Option<String>,
    pub toprice: Option<String>,
}

/// `GET /api/legacy-mirror/room-changes?cin_no=…` — mid-stay room-move
/// audit per check-in.
pub async fn list_room_changes(
    State(state): State<AppState>,
    Query(q): Query<CinNoQuery>,
) -> ApiResult<Json<Vec<RoomChangeRow>>> {
    let pool = match q.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        _ => &state.new_pool,
    };

    let rows = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>, Option<NaiveDateTime>, f64, Option<String>, Option<String>)>(
        "SELECT id, cin_no, room_before, room_after, change_date, \
                room_before_price, note, toprice \
           FROM legacy_mirror.ht_changed_room \
          WHERE cin_no = $1 \
          ORDER BY change_date NULLS LAST, id",
    )
    .bind(&q.cin_no)
    .fetch_all(pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|(id, cin, before, after, date, price, note, toprice)| RoomChangeRow {
                id,
                cin_no: cin,
                room_before: before,
                room_after: after,
                change_date: date,
                room_before_price: price,
                note,
                toprice,
            })
            .collect(),
    ))
}

// ─── Pricing reference data (consolidated) ──────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingReference {
    /// `HT_ContinueTime` — hourly extension price master.
    pub extension_prices: Vec<ExtensionPrice>,
    /// `HT_Rooms_Price` — per-customer-type room price overrides.
    pub room_prices: Vec<RoomPrice>,
    /// `HT_Order_Up` — pricing tier (price-up bracket).
    pub tier_up: Vec<PricingTier>,
    /// `HT_Order_Down` — pricing tier (price-down bracket).
    pub tier_down: Vec<PricingTier>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionPrice {
    pub id: i32,
    pub con_name: Option<String>,
    pub con_minute: Option<i32>,
    pub con_price: Option<f64>,
    pub con_type: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomPrice {
    pub id: i32,
    pub room_type: Option<String>,
    pub room_custtype: Option<String>,
    pub room_price: Option<f64>,
    pub room_price_h: Option<f64>,
    pub room_price_m: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingTier {
    pub id: i32,
    pub cust_type: Option<String>,
    pub cust_month: Option<i32>,
    pub cast_type: Option<String>,
}

/// `GET /api/legacy-mirror/pricing` — consolidated pricing reference
/// data. Returns all four dimension tables in one response so the
/// settings page makes one fetch.
pub async fn get_pricing_reference(
    State(state): State<AppState>,
    Query(q): Query<BranchOnlyQuery>,
) -> ApiResult<Json<PricingReference>> {
    let pool = match q.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        _ => &state.new_pool,
    };

    let extension_prices = sqlx::query_as::<_, (i32, Option<String>, Option<i32>, Option<f64>, Option<String>)>(
        "SELECT id, con_name, con_minute, con_price, con_type \
           FROM legacy_mirror.ht_continuetime ORDER BY id",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, name, minute, price, t)| ExtensionPrice {
        id,
        con_name: name,
        con_minute: minute,
        con_price: price,
        con_type: t,
    })
    .collect();

    let room_prices = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>)>(
        "SELECT id, room_type, room_custtype, room_price, room_price_h, room_price_m \
           FROM legacy_mirror.ht_rooms_price ORDER BY id",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, t, ct, p, h, m)| RoomPrice {
        id,
        room_type: t,
        room_custtype: ct,
        room_price: p,
        room_price_h: h,
        room_price_m: m,
    })
    .collect();

    let tier_up = pricing_tier(pool, "legacy_mirror.ht_order_up").await?;
    let tier_down = pricing_tier(pool, "legacy_mirror.ht_order_down").await?;

    Ok(Json(PricingReference {
        extension_prices,
        room_prices,
        tier_up,
        tier_down,
    }))
}

async fn pricing_tier(pool: &crate::db::PgPool, table: &str) -> ApiResult<Vec<PricingTier>> {
    // Both ht_order_up and ht_order_down have the identical 4-column
    // shape. Inline-build the SELECT (fixed string interpolation, no
    // user input — `table` is hardcoded by the caller).
    let sql = format!(
        "SELECT id, cust_type, cust_month, cast_type FROM {table} ORDER BY id"
    );
    let rows =
        sqlx::query_as::<_, (i32, Option<String>, Option<i32>, Option<String>)>(sqlx::AssertSqlSafe(&*sql))
            .fetch_all(pool)
            .await?;
    Ok(rows
        .into_iter()
        .map(|(id, ct, cm, cast)| PricingTier {
            id,
            cust_type: ct,
            cust_month: cm,
            cast_type: cast,
        })
        .collect())
}
