//! OTA booking bridge (`/api/ota/*`) — the MACHINE surface `ota-desk` calls.
//!
//! See `docs/ota-bridge.md` for the full contract and the rollout sequence.
//!
//! ## Why a parallel prefix instead of a gate on `/api/*`
//!
//! `ota-desk` creates PMS bookings by calling five ordinary `/api/*` routes
//! over the `hotel-network` docker bridge with **no credential of any kind**.
//! Those same routes are what the PMS browser UI reaches through the Next.js
//! rewrite (`next.config.js` — `/api/:path* → ${BACKEND_URL}/api/:path*`), so
//! requiring a bearer on them would break the whole front-desk UI.
//!
//! This module therefore re-mounts **the existing handler functions
//! unchanged** under a second prefix that IS bearer-gated. Nothing is
//! duplicated: no handler, no DTO, no query struct, no repository method. Every
//! one of the five takes only `State` + `Query`/`Json` and never reads its own
//! path, so a request that differs from today's only in its prefix produces a
//! byte-identical response. That equality is the contract — any drift silently
//! breaks `ota-desk/lib/pms.ts`.
//!
//! Deliberately NOT mounted: cancel / modify. `ota-desk` has no client code for
//! them; they belong to writeback Phase 3. Minimal surface = the actual need.
//!
//! ## Layering (outermost first): `require_ota_token` → `ville_write_guard` → routes
//!
//! * **`require_ota_token` outermost** so an unauthenticated probe is rejected
//!   before it can touch state or any other gate. It ships DARK: 503 until
//!   `OTA_BRIDGE_ENABLED`, then permissive until `OTA_BRIDGE_ENFORCE`
//!   ([`crate::middleware::ota_token`] holds the matrix).
//! * **`ville_write_guard` is MANDATORY**, not optional: it is present on the
//!   desk mounting in `main.rs` and keys on `?branch=` + method — exactly what
//!   `ota-desk` sends. Omitting it here would silently punch a HF Ville write
//!   hole around ADR 0002's admission gate.
//!   [`crate::middleware::ville_guard::is_ville_exempt_path`] matches only the
//!   maid cleaning route, so `/api/ota/*` gets no exemption. Pinned by
//!   `tests/test_ota_bridge.rs`.
//! * **`require_auth` is deliberately ABSENT** — the caller is a service with
//!   no cookie session, identical to `/api/channel/*`. The OTA token becomes
//!   load-bearing the moment `AUTH_ENABLED` is flipped true.
//!
//! `AUTHORIZATION` is deliberately NOT in the CORS `allow_headers` list
//! (`main.rs`), so a browser cannot present a bearer here at all — a free
//! structural guarantee that this stays a machine-only surface.
//!
//! ## Coexistence stance
//!
//! Re-mounting changes no write path: `create_booking` still commits to
//! canonical PG and enqueues its writeback exactly as the desk route does
//! (invariants #1/#2). [`reconcile_bookings`] is a pure PG READ — it touches no
//! legacy DB and enqueues nothing.
//!
//! All SQL here is RUNTIME `sqlx::query` (no compile-time macro), so this
//! module needs no `.sqlx/` cache regeneration — same policy as `routes::hk` /
//! `routes::guest_documents`.

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::config::OtaBridgeConfig;
use crate::error::{ApiError, ApiResult};

const BRANCH_REQUIRED_ERROR: &str =
    "branch is required and must be exactly 'hfhotel' or 'hfville' (reconciliation is per-property)";
const DATE_RANGE_ERROR: &str = "checkin_to must be on or after checkin_from";

/// Default page size — matches what `ota-desk`'s reconciler asks for.
const DEFAULT_LIMIT: i64 = 500;
/// Hard ceiling on a single page. Pagination, not a date-span cap, is what
/// bounds volume here (see [`reconcile_bookings`]).
const MAX_LIMIT: i64 = 1000;

/// Build the shipped `/api/ota/*` router, reading its config from the
/// environment. `main.rs` calls this; the integration tests call
/// [`router_with_config`] so they mount the SAME wiring with an explicit
/// config instead of a replica.
pub fn router(state: AppState) -> Router {
    router_with_config(state, OtaBridgeConfig::from_env())
}

/// Same router with an explicit [`OtaBridgeConfig`].
///
/// Layer order is load-bearing: in axum the LAST `.layer()` is the OUTERMOST,
/// so `ota_gate` must be applied after `ville_guard`.
pub fn router_with_config(state: AppState, config: OtaBridgeConfig) -> Router {
    let ville_guard = axum::middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::ville_guard::ville_write_guard,
    );
    let ota_gate = axum::middleware::from_fn_with_state(
        crate::middleware::ota_token::OtaTokenState::new(&config),
        crate::middleware::ota_token::require_ota_token,
    );

    Router::new()
        // ── The five EXISTING handlers, re-mounted verbatim ──────────────
        .route(
            "/api/ota/customers/search",
            get(super::new_customers::list_customers),
        )
        .route(
            "/api/ota/customers",
            post(super::new_customers::create_customer),
        )
        .route("/api/ota/rooms", get(super::new_rooms::list_rooms))
        .route(
            "/api/ota/bookings/validate",
            post(super::new_bookings::validate_booking),
        )
        .route(
            "/api/ota/bookings",
            post(super::new_bookings::create_booking),
        )
        // ── The one NEW shape ────────────────────────────────────────────
        .route("/api/ota/reconcile/bookings", get(reconcile_bookings))
        .with_state(state)
        .layer(ville_guard)
        .layer(ota_gate)
}

/// Query params for [`reconcile_bookings`].
///
/// Every field is `Option<_>` so axum's own `Query` rejection never fires on a
/// missing param — the validation below returns this repo's `ApiError`
/// envelope instead, so a client sees one consistent error shape.
#[derive(Debug, Deserialize)]
pub struct ReconcileBookingsQuery {
    pub branch: Option<String>,
    pub checkin_from: Option<String>,
    pub checkin_to: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

/// One reconciliation row. **All five fields are non-nullable strings.**
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconcileBookingRow {
    pub book_no: String,
    /// **Exactly `YYYY-MM-DD`** — produced by `::text` on a `DATE` column, NOT
    /// by a `NaiveDate`/`NaiveDateTime` serde field. See the SQL note.
    pub check_in: String,
    /// Exactly `YYYY-MM-DD`, same reason.
    pub check_out: String,
    pub guest_name: String,
    pub notes: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconcileBookingsResponse {
    pub success: bool,
    pub data: Vec<ReconcileBookingRow>,
    /// The last row's `bookNo` when the page is FULL, otherwise `null`.
    pub next_cursor: Option<String>,
}

/// The reconciler read, byte-parity with the direct `mcp_ro` PG query it
/// replaces (`ota-desk/ingest/reconcile.ts`). Three details are load-bearing
/// and must NOT be "cleaned up":
///
/// * **The `::text` casts.** The client compares `checkIn` to its own
///   `YYYY-MM-DD` strings. A `NaiveDate`/`NaiveDateTime` serde field would
///   render `"2026-08-14T00:00:00"` and take the matcher to zero matches
///   **silently** — rows would just stay `unmatched`. Highest-value assertion
///   in the suite (`tests/test_ota_bridge.rs`).
/// * **`coalesce(book_status,'') <> 'cancelled'`.** A NULL status is
///   *included*. Do not rewrite as `IS DISTINCT FROM` + NOT NULL, nor as a
///   status whitelist.
/// * **`guestName` / `notes` are composed in SQL**, one expression in one
///   place, so the two repos cannot drift. A LEFT JOIN miss (or a nameless
///   customer) yields `""` via `trim(' ')`, never null — the fields are
///   non-optional `String`. Note `notes` keeps its interior single space and
///   its possible TRAILING space: the client substring-searches this value.
const RECONCILE_SQL: &str = "\
SELECT b.book_no, \
       b.book_checkin::text  AS book_checkin, \
       b.book_checkout::text AS book_checkout, \
       trim(coalesce(c.cust_firstname,'') || ' ' || coalesce(c.cust_lastname,'')) AS guest_name, \
       coalesce(b.book_notes,'') || ' ' || coalesce(b.book_special_requests,'')   AS notes \
FROM ht_bookings b \
LEFT JOIN ht_customers c ON c.cust_id = b.book_cust_id \
WHERE b.book_checkin BETWEEN $1::date AND $2::date \
  AND coalesce(b.book_status,'') <> 'cancelled' \
  AND ($3::text IS NULL OR b.book_no > $3) \
ORDER BY b.book_no ASC \
LIMIT $4";

/// `GET /api/ota/reconcile/bookings` — the purpose-built reconciler read.
///
/// The existing `GET /api/bookings` cannot serve this: its date filter is
/// stay-OVERLAP not check-in-BETWEEN, its DTO has no `book_special_requests`,
/// its status filter is exact-match rather than `<> 'cancelled'`, and it
/// serializes `check_in` as a datetime.
///
/// Pagination is KEYSET on `book_no` (`VARCHAR(20) NOT NULL UNIQUE` — a total
/// order), so pages can neither duplicate nor skip within a snapshot. The date
/// predicate rides `ix_ht_bookings_checkin`.
///
/// There is deliberately **no cap on the date span**: silently clamping it
/// would produce false `unmatched` rows at the client, and rejecting a
/// legitimately wide window would stall reconciliation. Volume is bounded by
/// pagination instead.
pub async fn reconcile_bookings(
    State(state): State<AppState>,
    Query(params): Query<ReconcileBookingsQuery>,
) -> ApiResult<Json<ReconcileBookingsResponse>> {
    let branch = parse_reconcile_branch(params.branch.as_deref())?;
    let checkin_from = parse_checkin_date("checkin_from", params.checkin_from.as_deref())?;
    let checkin_to = parse_checkin_date("checkin_to", params.checkin_to.as_deref())?;
    if checkin_to < checkin_from {
        return Err(ApiError::BadRequest(DATE_RANGE_ERROR.to_string()));
    }
    let limit = parse_limit(params.limit)?;
    let cursor = params
        .cursor
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .map(str::to_string);

    // READ pool per property, mirroring `new_bookings::list_bookings`.
    // `All` is refused above — a union would break the client's per-property
    // `consumed` bookkeeping.
    let pool = match branch {
        Branch::Hfville => state.ville_pool()?,
        _ => &state.new_pool,
    };

    let rows = sqlx::query(RECONCILE_SQL)
        .bind(checkin_from)
        .bind(checkin_to)
        .bind(cursor.as_deref())
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let mut data = Vec::with_capacity(rows.len());
    for row in &rows {
        data.push(ReconcileBookingRow {
            book_no: row.try_get("book_no")?,
            check_in: row.try_get("book_checkin")?,
            check_out: row.try_get("book_checkout")?,
            guest_name: row.try_get("guest_name")?,
            notes: row.try_get("notes")?,
        });
    }

    // A FULL page means "there may be more"; a short page is the end.
    let next_cursor = if data.len() as i64 == limit {
        data.last().map(|row| row.book_no.clone())
    } else {
        None
    };

    Ok(Json(ReconcileBookingsResponse {
        success: true,
        data,
        next_cursor,
    }))
}

/// Require an exactly-spelled per-property branch. PURE.
///
/// `all` is refused on purpose: reconciliation is per-property, and
/// `write_pool(Some(All))` resolves to the PRIMARY pool — accepting it would
/// silently mean "HF Hotel" while reading as "both".
fn parse_reconcile_branch(raw: Option<&str>) -> ApiResult<Branch> {
    match raw.map(str::trim) {
        Some("hfhotel") => Ok(Branch::Hfhotel),
        Some("hfville") => Ok(Branch::Hfville),
        _ => Err(ApiError::BadRequest(BRANCH_REQUIRED_ERROR.to_string())),
    }
}

/// Require an exactly-`YYYY-MM-DD` date. PURE.
fn parse_checkin_date(field: &str, raw: Option<&str>) -> ApiResult<NaiveDate> {
    let raw = raw.map(str::trim).filter(|v| !v.is_empty());
    let Some(raw) = raw else {
        return Err(ApiError::BadRequest(format!(
            "{field} is required and must be YYYY-MM-DD"
        )));
    };
    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest(format!("{field} must be YYYY-MM-DD")))
}

/// Clamp-free page size: default 500, `1..=1000`, out-of-range is a 400 rather
/// than a silent clamp (a silently shrunk page would desync the client's
/// cursor loop). PURE.
fn parse_limit(raw: Option<i64>) -> ApiResult<i64> {
    match raw {
        None => Ok(DEFAULT_LIMIT),
        Some(n) if (1..=MAX_LIMIT).contains(&n) => Ok(n),
        Some(n) => Err(ApiError::BadRequest(format!(
            "limit must be between 1 and {MAX_LIMIT} (got {n})"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bad_request_message(err: ApiError) -> String {
        match err {
            ApiError::BadRequest(msg) => msg,
            other => panic!("expected BadRequest, got {other:?}"),
        }
    }

    #[test]
    fn branch_must_be_exactly_one_property() {
        assert_eq!(
            parse_reconcile_branch(Some("hfhotel")).unwrap(),
            Branch::Hfhotel
        );
        assert_eq!(
            parse_reconcile_branch(Some(" hfville ")).unwrap(),
            Branch::Hfville
        );
        for raw in [None, Some(""), Some("all"), Some("HFHOTEL"), Some("hf")] {
            let err = parse_reconcile_branch(raw).expect_err("{raw:?} must be refused");
            assert_eq!(bad_request_message(err), BRANCH_REQUIRED_ERROR);
        }
    }

    #[test]
    fn dates_must_be_yyyy_mm_dd() {
        assert_eq!(
            parse_checkin_date("checkin_from", Some("2026-08-14")).unwrap(),
            NaiveDate::from_ymd_opt(2026, 8, 14).unwrap()
        );
        for raw in [
            None,
            Some(""),
            Some("14/08/2026"),
            Some("2026-08-14T00:00:00"),
            Some("2026-13-01"),
        ] {
            parse_checkin_date("checkin_from", raw).expect_err("{raw:?} must be refused");
        }
    }

    #[test]
    fn limit_defaults_and_rejects_out_of_range() {
        assert_eq!(parse_limit(None).unwrap(), DEFAULT_LIMIT);
        assert_eq!(parse_limit(Some(1)).unwrap(), 1);
        assert_eq!(parse_limit(Some(MAX_LIMIT)).unwrap(), MAX_LIMIT);
        for n in [0_i64, -1, MAX_LIMIT + 1] {
            parse_limit(Some(n)).expect_err("limit {n} must be refused, not clamped");
        }
    }

    /// The response envelope must be camelCase — `lib/pms-bookings.ts` reads
    /// `bookNo` / `checkIn` / `checkOut` / `guestName` / `nextCursor`.
    #[test]
    fn response_serializes_camel_case_with_null_cursor() {
        let body = serde_json::to_value(ReconcileBookingsResponse {
            success: true,
            data: vec![ReconcileBookingRow {
                book_no: "B260814001".to_string(),
                check_in: "2026-08-14".to_string(),
                check_out: "2026-08-16".to_string(),
                guest_name: String::new(),
                notes: "paid ".to_string(),
            }],
            next_cursor: None,
        })
        .expect("serializes");

        assert_eq!(body["success"], serde_json::json!(true));
        assert_eq!(body["data"][0]["bookNo"], serde_json::json!("B260814001"));
        assert_eq!(body["data"][0]["checkIn"], serde_json::json!("2026-08-14"));
        assert_eq!(body["data"][0]["checkOut"], serde_json::json!("2026-08-16"));
        assert_eq!(body["data"][0]["guestName"], serde_json::json!(""));
        assert_eq!(
            body["data"][0]["notes"],
            serde_json::json!("paid "),
            "the trailing space is part of the value the client substring-searches"
        );
        assert!(
            body["nextCursor"].is_null(),
            "a short page must emit an explicit null, not omit the key"
        );
    }

    /// The SQL is the reconciler's contract; these clauses are the ones a
    /// well-meaning cleanup would break.
    #[test]
    fn reconcile_sql_keeps_its_load_bearing_clauses() {
        assert!(
            RECONCILE_SQL.contains("b.book_checkin::text"),
            "dropping the ::text cast silently zeroes the client's date matching"
        );
        assert!(RECONCILE_SQL.contains("b.book_checkout::text"));
        assert!(
            RECONCILE_SQL.contains("coalesce(b.book_status,'') <> 'cancelled'"),
            "a NULL book_status must stay INCLUDED"
        );
        assert!(
            RECONCILE_SQL.contains("LEFT JOIN ht_customers"),
            "an inner join would drop rows the reconciler expects to see"
        );
        assert!(RECONCILE_SQL.contains("ORDER BY b.book_no ASC"));
        assert!(
            RECONCILE_SQL.contains("($3::text IS NULL OR b.book_no > $3)"),
            "keyset cursor must be an EXCLUSIVE lower bound on the ordered key"
        );
    }
}
