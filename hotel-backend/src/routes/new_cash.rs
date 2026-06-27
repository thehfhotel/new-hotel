//! Cash in/out petty-cash ledger API (รายรับ-รายจ่าย) — migration 059.
//!
//! - `GET  /api/cash?branch=&from=&to=&kind=` — list ledger entries in a date
//!   range, plus income/expense/balance totals.
//! - `POST /api/cash/income`   — record an income entry.
//! - `POST /api/cash/expense`  — record an expense entry.
//! - `GET  /api/cash/categories?branch=&level=` — the account taxonomy
//!   (mirror of TB_SET_MyType2 / _2_2 / 3) for the entry-form dropdowns.
//!
//! Branch-aware: `?branch=hfville` reads/writes the `hotelville` canonical
//! pool, otherwise the primary `hotelnew` pool — the same per-site resolution
//! as [`crate::routes::new_shifts::current_shift`]. Site scoping is
//! connection-level (each logical DB holds exactly one site's ledger), so the
//! pool selection *is* the site filter. All queries are runtime `sqlx::query`
//! (no `query!` macro) so they add nothing to the `.sqlx/` offline cache.
//!
//! ## Writeback (intentionally NOT wired — see the TODO in `create_entry`)
//!
//! Coexistence (ADR 0002) means an entry created here SHOULD also land in
//! legacy `TB_Pay_History` so iHOTEL sees it. The PURE recipe for that INSERT
//! lives in [`crate::writeback::recipes::cash_entry`], but it is deliberately
//! left unwired pending byte-shape verification of the legacy `Pay_Type` /
//! `Pay_Group` / `Pay_Account` / `Pay_Program` values against the FrmAddPay
//! decompile. Until then these handlers write canonical PG only; the inbound
//! poll (`bin/sync.rs::sync_cash_history`) still mirrors iHOTEL's entries in.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};

/// Hard cap on the list window so the date-range scan stays bounded.
const MAX_RANGE_DAYS: i64 = 366;

// =============================================================================
// Query / request / response shapes
// =============================================================================

/// `GET /api/cash` query string.
#[derive(Debug, Deserialize)]
pub struct CashListQuery {
    pub branch: Option<Branch>,
    /// RFC3339 lower bound (inclusive). Defaults to 30 days before `to`.
    pub from: Option<DateTime<Utc>>,
    /// RFC3339 upper bound (exclusive). Defaults to now.
    pub to: Option<DateTime<Utc>>,
    /// Optional `cash_kind` filter (`income` | `expense` | `unknown`).
    pub kind: Option<String>,
}

/// Branch selector for the create + category endpoints.
#[derive(Debug, Deserialize)]
pub struct CashBranchQuery {
    pub branch: Option<Branch>,
}

/// `GET /api/cash/categories` query string.
#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub branch: Option<Branch>,
    /// Optional `cat_level` filter (`2` | `2_2` | `3`).
    pub level: Option<String>,
}

/// Body for `POST /api/cash/{income,expense}`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCashEntryRequest {
    /// Entry date. Defaults to now if omitted.
    #[serde(default)]
    pub entry_date: Option<DateTime<Utc>>,
    /// Amount in baht. Must be finite and `> 0`.
    pub amount: f64,
    /// Payer / payee (legacy `Pay_Cust`).
    #[serde(default)]
    pub payee: Option<String>,
    /// Document / bill number (legacy `Pay_Bill`).
    #[serde(default)]
    pub bill_no: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    /// Account-tree id_full (legacy `Pay_Group`).
    #[serde(default)]
    pub group: Option<String>,
    /// Account-tree id_full (legacy `Pay_Account`).
    #[serde(default)]
    pub account: Option<String>,
    /// Who entered it (cashier login).
    #[serde(default)]
    pub created_by: Option<String>,
}

/// One ledger row in API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashEntryDto {
    pub cash_id: i64,
    pub legacy_id: Option<i32>,
    pub kind: String,
    pub legacy_type: Option<String>,
    pub entry_date: Option<DateTime<Utc>>,
    pub bill_no: Option<String>,
    pub payee: Option<String>,
    pub amount: f64,
    pub note: Option<String>,
    pub program_date: Option<DateTime<Utc>>,
    pub group: Option<String>,
    pub account: Option<String>,
    pub source: String,
    pub created_by: Option<String>,
}

/// Income/expense/balance roll-up over the listed window.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CashTotals {
    pub income: f64,
    pub expense: f64,
    /// `income − expense`.
    pub balance: f64,
    pub count: i64,
}

/// `200` body for `GET /api/cash`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashListResponse {
    pub success: bool,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub entries: Vec<CashEntryDto>,
    pub totals: CashTotals,
}

/// One account-tree node.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashCategoryDto {
    pub cat_id: i64,
    pub level: String,
    pub legacy_id: i32,
    pub id_full: Option<String>,
    pub name: Option<String>,
}

/// `200` body for `GET /api/cash/categories`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashCategoryListResponse {
    pub success: bool,
    pub categories: Vec<CashCategoryDto>,
}

/// `201` body for a successful create.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCashEntryResponse {
    pub success: bool,
    pub entry: CashEntryDto,
}

// =============================================================================
// Row mapping
// =============================================================================

/// Map a decode failure on a ledger read to a 500 (schema drift, not a client
/// mistake).
fn map_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map cash row: {e}"))
}

/// Build a [`CashEntryDto`] from a ledger row. The SELECT MUST cast
/// `cash_amount::float8 AS cash_amount` (NUMERIC → f64).
fn row_to_entry(row: &sqlx::postgres::PgRow) -> ApiResult<CashEntryDto> {
    Ok(CashEntryDto {
        cash_id: row.try_get("cash_id").map_err(map_row_err)?,
        legacy_id: row.try_get("cash_legacy_id").map_err(map_row_err)?,
        kind: row.try_get("cash_kind").map_err(map_row_err)?,
        legacy_type: row.try_get("cash_legacy_type").map_err(map_row_err)?,
        entry_date: row.try_get("cash_entry_date").map_err(map_row_err)?,
        bill_no: row.try_get("cash_bill_no").map_err(map_row_err)?,
        payee: row.try_get("cash_payee").map_err(map_row_err)?,
        amount: row.try_get("cash_amount").map_err(map_row_err)?,
        note: row.try_get("cash_note").map_err(map_row_err)?,
        program_date: row.try_get("cash_program_date").map_err(map_row_err)?,
        group: row.try_get("cash_group").map_err(map_row_err)?,
        account: row.try_get("cash_account").map_err(map_row_err)?,
        source: row.try_get("cash_source").map_err(map_row_err)?,
        created_by: row.try_get("cash_created_by").map_err(map_row_err)?,
    })
}

/// Columns selected by both the list and create paths (so the DTO mapping is
/// identical). `cash_amount` is cast to `float8`.
const ENTRY_COLUMNS: &str = "cash_id, cash_legacy_id, cash_kind, cash_legacy_type, \
     cash_entry_date, cash_bill_no, cash_payee, cash_amount::float8 AS cash_amount, \
     cash_note, cash_program_date, cash_group, cash_account, cash_source, cash_created_by";

/// Resolve the per-site pool for a branch (HF Ville → ville pool, else
/// primary). `All` collapses to the primary pool — a cash ledger is per-site
/// and the page picks one branch at a time.
fn resolve_pool<'a>(state: &'a AppState, branch: Option<Branch>) -> ApiResult<&'a crate::db::PgPool> {
    match branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool(),
        Branch::Hfhotel | Branch::All => Ok(&state.new_pool),
    }
}

// =============================================================================
// Handlers
// =============================================================================

/// `GET /api/cash` — list ledger entries in a date range, with income/expense
/// totals. Branch-aware. Range defaults to the last 30 days and is capped at
/// [`MAX_RANGE_DAYS`].
pub async fn list_cash(
    State(state): State<AppState>,
    Query(query): Query<CashListQuery>,
) -> ApiResult<Json<CashListResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    let to = query.to.unwrap_or_else(Utc::now);
    let mut from = query.from.unwrap_or_else(|| to - chrono::Duration::days(30));
    let max_from = to - chrono::Duration::days(MAX_RANGE_DAYS);
    if from < max_from {
        from = max_from;
    }

    // Optional kind filter. Validate against the canonical CHECK domain so a
    // typo'd filter is a 400, not a silent empty result.
    let kind = match query.kind.as_deref() {
        None | Some("") => None,
        Some(k @ ("income" | "expense" | "unknown")) => Some(k.to_string()),
        Some(other) => {
            return Err(ApiError::BadRequest(format!(
                "invalid kind '{other}' (expected income | expense | unknown)"
            )))
        }
    };

    let mut sql = format!(
        "SELECT {ENTRY_COLUMNS} FROM ht_cash_ledger \
          WHERE cash_entry_date >= $1 AND cash_entry_date < $2"
    );
    if kind.is_some() {
        sql.push_str(" AND cash_kind = $3");
    }
    sql.push_str(" ORDER BY cash_entry_date DESC NULLS LAST, cash_id DESC");

    let mut q = sqlx::query(&sql).bind(from).bind(to);
    if let Some(ref k) = kind {
        q = q.bind(k);
    }
    let rows = q
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to list cash entries: {e}")))?;

    let round2 = |v: f64| (v * 100.0).round() / 100.0;
    let mut entries = Vec::with_capacity(rows.len());
    let mut totals = CashTotals::default();
    for row in &rows {
        let entry = row_to_entry(row)?;
        match entry.kind.as_str() {
            "income" => totals.income += entry.amount,
            "expense" => totals.expense += entry.amount,
            _ => {}
        }
        totals.count += 1;
        entries.push(entry);
    }
    totals.income = round2(totals.income);
    totals.expense = round2(totals.expense);
    totals.balance = round2(totals.income - totals.expense);

    Ok(Json(CashListResponse {
        success: true,
        from,
        to,
        entries,
        totals,
    }))
}

/// `GET /api/cash/categories` — the account taxonomy for the entry-form
/// dropdowns. Branch-aware; optional `level` filter.
pub async fn list_categories(
    State(state): State<AppState>,
    Query(query): Query<CategoryQuery>,
) -> ApiResult<Json<CashCategoryListResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    let level = match query.level.as_deref() {
        None | Some("") => None,
        Some(l @ ("2" | "2_2" | "3")) => Some(l.to_string()),
        Some(other) => {
            return Err(ApiError::BadRequest(format!(
                "invalid level '{other}' (expected 2 | 2_2 | 3)"
            )))
        }
    };

    let mut sql = String::from(
        "SELECT cash_cat_id, cat_level, cat_legacy_id, cat_id_full, cat_name \
           FROM ht_cash_categories",
    );
    if level.is_some() {
        sql.push_str(" WHERE cat_level = $1");
    }
    sql.push_str(" ORDER BY cat_level, cat_name NULLS LAST, cat_legacy_id");

    let mut q = sqlx::query(&sql);
    if let Some(ref l) = level {
        q = q.bind(l);
    }
    let rows = q
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to list cash categories: {e}")))?;

    let mut categories = Vec::with_capacity(rows.len());
    for row in &rows {
        categories.push(CashCategoryDto {
            cat_id: row.try_get("cash_cat_id").map_err(map_row_err)?,
            level: row.try_get("cat_level").map_err(map_row_err)?,
            legacy_id: row.try_get("cat_legacy_id").map_err(map_row_err)?,
            id_full: row.try_get("cat_id_full").map_err(map_row_err)?,
            name: row.try_get("cat_name").map_err(map_row_err)?,
        });
    }

    Ok(Json(CashCategoryListResponse {
        success: true,
        categories,
    }))
}

/// `POST /api/cash/income` — record an income entry.
pub async fn create_income(
    State(state): State<AppState>,
    Query(query): Query<CashBranchQuery>,
    Json(body): Json<CreateCashEntryRequest>,
) -> ApiResult<(StatusCode, Json<CreateCashEntryResponse>)> {
    create_entry(state, query.branch, body, "income").await
}

/// `POST /api/cash/expense` — record an expense entry.
pub async fn create_expense(
    State(state): State<AppState>,
    Query(query): Query<CashBranchQuery>,
    Json(body): Json<CreateCashEntryRequest>,
) -> ApiResult<(StatusCode, Json<CreateCashEntryResponse>)> {
    create_entry(state, query.branch, body, "expense").await
}

/// Shared create path for income + expense. Inserts a canonical
/// `cash_source='app'` row on the resolved per-site pool.
///
/// TODO(cash-writeback): mirror this entry into legacy `TB_Pay_History` so
/// iHOTEL sees it (coexistence — ADR 0002). The PURE INSERT recipe is ready in
/// `crate::writeback::recipes::cash_entry`, but wiring it (allocate a
/// `WritebackIntent`, emit it in this transaction, add a dispatcher arm) is
/// gated on verifying the exact legacy `Pay_Type` / `Pay_Group` / `Pay_Account`
/// / `Pay_Program` byte shape against the FrmAddPay decompile. Until then app
/// entries live in canonical PG only.
async fn create_entry(
    state: AppState,
    branch: Option<Branch>,
    body: CreateCashEntryRequest,
    kind: &str,
) -> ApiResult<(StatusCode, Json<CreateCashEntryResponse>)> {
    let pool = resolve_pool(&state, branch)?;

    if !body.amount.is_finite() || body.amount <= 0.0 {
        return Err(ApiError::BadRequest(
            "amount must be a finite number greater than 0".to_string(),
        ));
    }

    let entry_date = body.entry_date.unwrap_or_else(Utc::now);
    // Best-effort legacy `Pay_Type` marker to store alongside (and, once
    // writeback is wired, write back as). The EXACT legacy strings are
    // unverified — see the TODO above; the normalized `cash_kind` is the
    // authoritative direction for our app.
    let legacy_type = if kind == "income" { "รายรับ" } else { "รายจ่าย" };

    let sql = format!(
        "INSERT INTO ht_cash_ledger ( \
             cash_kind, cash_legacy_type, cash_entry_date, cash_bill_no, \
             cash_payee, cash_amount, cash_note, cash_group, cash_account, \
             cash_source, cash_created_by \
         ) VALUES ($1, $2, $3, $4, $5, $6::numeric, $7, $8, $9, 'app', $10) \
         RETURNING {ENTRY_COLUMNS}"
    );

    let row = sqlx::query(&sql)
        .bind(kind)
        .bind(legacy_type)
        .bind(entry_date)
        .bind(&body.bill_no)
        .bind(&body.payee)
        .bind(body.amount)
        .bind(&body.note)
        .bind(&body.group)
        .bind(&body.account)
        .bind(&body.created_by)
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to create cash entry: {e}")))?;

    let entry = row_to_entry(&row)?;
    Ok((
        StatusCode::CREATED,
        Json(CreateCashEntryResponse {
            success: true,
            entry,
        }),
    ))
}
