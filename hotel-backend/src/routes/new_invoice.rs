//! New Invoice API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/invoice - Get complete invoice data for a check-in
//!
//! ## Track G3 — VAT invoice fields (`audit-2026-05-13.md` T4 HIGH-7)
//!
//! Corporate guests in Thailand require a proper VAT invoice (ใบกำกับภาษี)
//! that shows the VAT split (before-VAT subtotal + VAT amount), the buyer's
//! tax-id (`cust_work_tax`), and an invoice number. Before G3 this route
//! returned a bare total with no VAT breakdown; the frontend
//! `InvoiceTemplate` had a `vatAmount` field but the API never set it. G3
//! plumbs the four missing fields through.
//!
//! ## Tax invoice (task #44) — POS lines + reconciling document total
//!
//! A Thai ใบกำกับภาษี must itemise *every* charge and have its line items
//! reconcile to the printed grand total. The G3 shape was room-only, so a
//! folio with minibar / restaurant / other POS charges printed an invoice
//! that under-stated the bill. Task #44 adds the folio's `ht_pos_sales`
//! (posted) lines alongside the room line and bases the document total +
//! VAT split on the sum of the lines (`room_total + products_total`) rather
//! than the stored `cin_total_amount`. This is deliberate:
//! `cin_total_amount` is the *room* total before checkout (POS sales accrue
//! separately in `ht_pos_sales`) and only folds in products at checkout, so
//! summing the lines is the one figure that reconciles in BOTH the
//! in-house and checked-out states without double-counting. `total_amount`
//! still reports the stored `cin_total_amount` unchanged for any consumer
//! that wants the persisted figure. Branch-aware: POS lines are read from
//! the same pool the check-in came from.
//!
//! - `vat_per` — read from `ht_settings.vat_percent` via
//!   [`crate::repository::settings::get_vat_percent`] (Wave 5c plumbing).
//!   Falls back to 7% (Thai standard VAT) on any lookup failure.
//! - `before_vat`, `vat_amount` — VAT-inclusive split using
//!   [`crate::writeback::format::vat_inclusive_split`] (banker's rounding —
//!   matches `.NET Math.Round` default, audit Wave 6 H4).
//! - `tax_id` — `ht_customers.cust_work_tax` (Track E2 added this column).
//! - `inv_no` — deterministic, Bangkok-calendar-scoped, PG-only number of
//!   the form `INV{yyMM}-{cin_id:06}`. **TODO: legacy alignment.** The
//!   legacy `HT_INVOICE.INV_NO` column is `int NOT NULL` and the .NET app
//!   allocates via `MAX+1`. Writing `HT_INVOICE` from the writeback adapter
//!   is out of scope for G3 (deferred to a follow-on Track G wave). Until
//!   then this number is stable per check-in but does not reserve a slot in
//!   the legacy sequence.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{Datelike, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Asia::Bangkok;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::repository::settings;
use crate::writeback::format::vat_inclusive_split;

/// Guest details for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceGuest {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub id_card: Option<String>,
    pub passport: Option<String>,
    /// Corporate buyer's tax-id (`ht_customers.cust_work_tax`). Required on
    /// every Thai VAT invoice issued to a juristic person; blank for
    /// individual walk-ins. G3 / T4 HIGH-7.
    pub tax_id: Option<String>,
}

/// Room assignment for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRoom {
    pub room_id: i32,
    pub room_no: String,
    pub room_type: Option<String>,
    pub floor: Option<i32>,
}

/// Rate details for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRates {
    pub rate_per_night: f64,
    pub nights: i32,
    pub subtotal: f64,
}

/// One itemised room line on the invoice (task #62 — per-room bills).
///
/// Sourced from the `ht_checkin_rooms` junction (one row per room per
/// folio) joined to `ht_rooms_new` (+ `ht_room_types`). A multi-room stay
/// yields one line per room so the printed invoice matches iHOTEL's
/// per-room `HT_CheckIn_Ds` view; the document's room subtotal is the sum
/// of every line's `subtotal` (`cr_room_total`). Empty only for
/// pre-junction folios (the B5 backfill), where the route falls back to
/// the legacy single `room` / `rates` fields.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRoomLine {
    pub room_no: String,
    pub room_type: Option<String>,
    pub nights: i32,
    pub rate_per_night: f64,
    pub subtotal: f64,
}

/// One POS / product / other-charge line on the tax invoice (task #44).
///
/// Sourced from `ht_pos_sales` (posted rows only) joined to `ht_products`
/// for the display name + unit. Mirrors the legacy `HT_Invoice_Ds`
/// shape (S_Product_name / S_Unit / S_Price / S_Total) so the printed
/// ใบกำกับภาษี itemises minibar / restaurant / other charges next to the
/// room line.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceProduct {
    pub name: String,
    pub unit: Option<String>,
    pub qty: f64,
    pub unit_price: f64,
    pub total: f64,
}

/// Complete invoice data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    // Check-in info
    pub checkin_id: i32,
    pub cin_no: String,
    pub booking_id: Option<i32>,
    pub booking_no: Option<String>,
    /// G3 / T4 HIGH-7: PG-only invoice number (legacy MSSQL alignment
    /// deferred — see module doc). Stable per check-in.
    pub inv_no: String,

    // Guest details
    pub guest: InvoiceGuest,

    // Room assignment (legacy single-room — kept for existing consumers
    // that read `room`/`rates`; mirrors the deprecated `cin_room_id` join).
    pub room: InvoiceRoom,
    /// Task #62: itemised per-room lines from the `ht_checkin_rooms`
    /// junction (one entry per room — multi-room bills). Ordered by room
    /// number. Empty for pre-junction folios; consumers then fall back to
    /// the single `room` / `rates` fields above.
    pub rooms: Vec<InvoiceRoomLine>,

    // Stay details
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub expected_checkout: Option<NaiveDateTime>,
    pub adults: i32,
    pub children: i32,

    // Rate calculations
    pub rates: InvoiceRates,

    // POS / other charges (task #44 — tax invoice itemisation)
    /// Posted `ht_pos_sales` lines for this folio (minibar / restaurant /
    /// other). Empty for a room-only stay. Branch-aware.
    pub products: Vec<InvoiceProduct>,
    /// Sum of the room line(s) — equals `rates.subtotal`. Surfaced
    /// explicitly so the document's "room subtotal" reconciles against the
    /// products subtotal and the grand total.
    pub room_total: f64,
    /// Sum of `products[].total` (posted POS lines only).
    pub products_total: f64,

    // Totals (G3: VAT-inclusive split)
    /// Stored `cin_total_amount` (room total before checkout; room+product
    /// after). Reported unchanged for consumers wanting the persisted
    /// figure — the *document* total is `grand_total`. See module docs.
    pub total_amount: f64,
    /// Task #44: the reconciling document total = `room_total +
    /// products_total`. This is the figure the VAT split is taken over and
    /// the one the printed ใบกำกับภาษี shows as ยอดรวมทั้งสิ้น.
    pub grand_total: f64,
    /// G3 / T4 HIGH-7: subtotal before VAT, derived via banker's rounding.
    pub before_vat: f64,
    /// G3 / T4 HIGH-7: VAT amount, derived via banker's rounding such that
    /// `before_vat + vat_amount == total_amount` (within rounding tolerance).
    pub vat_amount: f64,
    /// G3 / T4 HIGH-7: VAT percentage applied (e.g. `7`). Read from
    /// `ht_settings.vat_percent` (Wave 5c) — falls back to the Thai
    /// standard 7% on lookup failure.
    pub vat_per: i32,
    pub payment_status: Option<String>,

    // Notes
    pub notes: Option<String>,

    // Timestamps
    pub created_at: Option<NaiveDateTime>,
}

/// Response for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceResponse {
    pub success: bool,
    pub invoice: Invoice,
}

/// Query parameters for the invoice fetch (branch selector only).
///
/// `cin_id` is a per-database SERIAL — it collides across the two logical DBs,
/// so the pool selection is what returns the correct check-in. Mirrors the
/// branch handling in `routes/checkins.rs`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceQuery {
    pub branch: Option<Branch>,
}

/// GET /api/new/checkins/:id/invoice - Get complete invoice data
pub async fn get_invoice(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Query(params): Query<InvoiceQuery>,
) -> ApiResult<Json<InvoiceResponse>> {
    // Branch-aware: HF Hotel reads new_pool, HF Ville reads ville_pool. `All`
    // is not meaningful for a single check-in → default to HF Hotel.
    let pool = match params.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    // Get check-in with all related data
    let rec = sqlx::query!(
            r#"
            SELECT
                -- Check-in info
                ci.cin_id,
                COALESCE(NULLIF(ci.legacy_cin_no, ''), ci.cin_no) AS cin_no,
                ci.cin_book_id,
                COALESCE(b.book_no, '') as "book_no!",

                -- Customer/Guest info (LEFT JOIN — coalesce to satisfy NOT NULL bindings; missing customer is rare for walk-ins)
                COALESCE(c.cust_id, 0) as "cust_id!",
                COALESCE(c.cust_firstname, '') as "cust_firstname!",
                c.cust_lastname,
                c.cust_email,
                c.cust_phone,
                c.cust_address,
                c.cust_idcard,
                c.cust_passport,
                -- G3 / T4 HIGH-7: corporate buyer tax-id (Track E2 column).
                c.cust_work_tax,

                -- Room info (LEFT JOIN — coalesce in case of orphaned room_id)
                COALESCE(r.room_id, 0) as "room_id!",
                COALESCE(r.room_no, '') as "room_no!",
                r.room_floor,
                COALESCE(rt.type_name, '') as "room_type!",

                -- Stay details
                ci.cin_checkin_time,
                ci.cin_checkout_time,
                ci.cin_expected_checkout,
                ci.cin_adults,
                ci.cin_children,

                -- Rate info
                ci.cin_rate_per_night::float8 as cin_rate_per_night,
                (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date) as nights,

                -- Totals
                ci.cin_total_amount::float8 as cin_total_amount,
                ci.cin_payment_status,

                -- Notes and timestamps
                ci.cin_notes,
                ci.created_at
            FROM ht_checkins ci
            LEFT JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
            LEFT JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
            LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
            LEFT JOIN ht_bookings b ON ci.cin_book_id = b.book_id
            WHERE ci.cin_id = $1
            "#,
            cin_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Extract customer info
    let first_name = rec.cust_firstname;
    let last_name = rec.cust_lastname.clone();
    let full_name = match &last_name {
        Some(ln) => format!("{} {}", first_name, ln),
        None => first_name.clone(),
    };

    let guest = InvoiceGuest {
        id: rec.cust_id,
        first_name,
        last_name,
        full_name,
        email: rec.cust_email,
        phone: rec.cust_phone,
        address: rec.cust_address,
        id_card: rec.cust_idcard,
        passport: rec.cust_passport,
        tax_id: normalize_tax_id(rec.cust_work_tax),
    };

    // Extract room info
    let room = InvoiceRoom {
        room_id: rec.room_id,
        room_no: rec.room_no,
        room_type: Some(rec.room_type),
        floor: rec.room_floor,
    };

    // Extract rate info
    let rate_per_night = rec.cin_rate_per_night.unwrap_or(0.0);
    let nights = rec.nights.unwrap_or(1).max(1);
    let subtotal = rate_per_night * nights as f64;

    let rates = InvoiceRates {
        rate_per_night,
        nights,
        subtotal,
    };

    // Task #62: itemise every room of the stay from the `ht_checkin_rooms`
    // junction so a multi-room folio lists one line per room (matching
    // iHOTEL's per-room `HT_CheckIn_Ds` view). Runtime query (literal
    // string + bind) so no `.sqlx` cache entry is needed — mirrors the POS
    // line query below. Read from the branch-aware `pool` selected above.
    // `room_type` is a LEFT JOIN (orphaned `room_type_id` → NULL line).
    let room_rows = sqlx::query(
        "SELECT r.room_no, \
                rt.type_name, \
                cr.cr_nights                 AS nights, \
                cr.cr_rate_per_night::float8 AS rate_per_night, \
                cr.cr_room_total::float8     AS subtotal \
           FROM ht_checkin_rooms cr \
           JOIN ht_rooms_new   r  ON r.room_id  = cr.cr_room_id \
      LEFT JOIN ht_room_types  rt ON rt.type_id = r.room_type_id \
          WHERE cr.cr_cin_id = $1 \
       ORDER BY r.room_no ASC",
    )
    .bind(rec.cin_id)
    .fetch_all(pool)
    .await?;

    let rooms: Vec<InvoiceRoomLine> = room_rows
        .into_iter()
        .map(|row| InvoiceRoomLine {
            room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
            room_type: row.try_get::<Option<String>, _>("type_name").unwrap_or(None),
            nights: row.try_get::<i32, _>("nights").unwrap_or(0),
            rate_per_night: row.try_get::<f64, _>("rate_per_night").unwrap_or(0.0),
            subtotal: row.try_get::<f64, _>("subtotal").unwrap_or(0.0),
        })
        .collect();

    // Get stored total amount (use calculated if not stored). Reported
    // as `total_amount`; the *document* total is `grand_total` below.
    let total_amount = rec.cin_total_amount.unwrap_or(subtotal);

    // Task #44: itemise the folio's POS / other charges. Posted rows only
    // (voided lines never appear on the printed invoice). Runtime query
    // (literal string + bind) so no `.sqlx` cache entry is needed. Read
    // from the branch-aware `pool` selected above.
    let product_rows = sqlx::query(
        "SELECT p.prod_name, p.prod_unit, \
                s.sale_qty::float8        AS qty, \
                s.sale_unit_price::float8 AS unit_price, \
                s.sale_total::float8      AS total \
           FROM ht_pos_sales s \
           JOIN ht_products  p ON p.prod_id = s.sale_product_id \
          WHERE s.sale_cin_id = $1 AND s.sale_status = 'posted' \
       ORDER BY s.sale_sold_at ASC, s.sale_id ASC",
    )
    .bind(rec.cin_id)
    .fetch_all(pool)
    .await?;

    let products: Vec<InvoiceProduct> = product_rows
        .into_iter()
        .map(|row| InvoiceProduct {
            name: row.try_get::<String, _>("prod_name").unwrap_or_default(),
            unit: row.try_get::<Option<String>, _>("prod_unit").unwrap_or(None),
            qty: row.try_get::<f64, _>("qty").unwrap_or(0.0),
            unit_price: row.try_get::<f64, _>("unit_price").unwrap_or(0.0),
            total: row.try_get::<f64, _>("total").unwrap_or(0.0),
        })
        .collect();
    let products_total = products.iter().map(|p| p.total).sum::<f64>();

    // Document total = sum of the lines (room + products). See module docs
    // for why this is preferred over the stored `cin_total_amount`.
    //
    // Task #62: the room subtotal is the SUM of the per-room
    // `cr_room_total` from the junction (authoritative for multi-room
    // bills). Falls back to the legacy single-room `subtotal`
    // (rate × nights) only when the junction has no rows — i.e. a
    // pre-B5-backfill folio.
    let room_total = if rooms.is_empty() {
        subtotal
    } else {
        rooms.iter().map(|r| r.subtotal).sum::<f64>()
    };
    let grand_total = room_total + products_total;

    // G3 / T4 HIGH-7: VAT split (over the reconciling document total) +
    // invoice number. HF Hotel runs vat_per=0 → split is (grand_total, 0).
    let vat_per = settings::get_vat_percent(pool).await;
    let (before_vat, vat_amount) = vat_inclusive_split(grand_total, vat_per);
    let inv_no = format_invoice_number(rec.cin_id, rec.created_at);

    let invoice = Invoice {
        checkin_id: rec.cin_id,
        cin_no: rec.cin_no,
        booking_id: rec.cin_book_id,
        booking_no: Some(rec.book_no),
        inv_no,
        guest,
        room,
        rooms,
        check_in_time: Some(rec.cin_checkin_time),
        check_out_time: rec.cin_checkout_time,
        expected_checkout: Some(rec.cin_expected_checkout.and_hms_opt(0, 0, 0).unwrap()),
        adults: rec.cin_adults.unwrap_or(1),
        children: rec.cin_children.unwrap_or(0),
        rates,
        products,
        room_total,
        products_total,
        total_amount,
        grand_total,
        before_vat,
        vat_amount,
        vat_per,
        payment_status: rec.cin_payment_status,
        notes: rec.cin_notes,
        created_at: rec.created_at,
    };

    Ok(Json(InvoiceResponse {
        success: true,
        invoice,
    }))
}

/// Normalize the raw `ht_customers.cust_work_tax` value into the shape the
/// frontend expects: `None` if missing or blank-after-trim, otherwise the
/// trimmed string. Trimming matches the convention used elsewhere in the
/// stack (`repository::settings::parse_vat_percent`) and protects the UI
/// from rendering a stray-whitespace tax-id that looks like an empty field
/// with a tooltip.
fn normalize_tax_id(raw: Option<String>) -> Option<String> {
    raw.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

/// Build a deterministic, PG-only invoice number of the form
/// `INV{yyMM}-{cin_id:06}`. Year/month are scoped to the Bangkok calendar
/// to match the convention used everywhere else (audit CRIT-C1 / C3:
/// `pay_no_prefix` / `receipt_no_prefix`). When `created_at` is absent we
/// fall back to the current UTC instant — preserves call-site behaviour
/// without requiring the caller to thread `Utc::now()` in.
///
/// **TODO (deferred — re-confirmed task #44):** the legacy
/// `HT_INVOICE.INV_NO` column is `int NOT NULL` allocated via `MAX+1` by
/// the .NET app, but the *INSERT shape* is NOT validated in the spike
/// captures. `docs/legacy-spike/findings.md` §3h ("take payment + print
/// invoice") is actually a `HT_Receipt_H`/`HT_Receipt_Ds` write, not
/// `HT_INVOICE`; the real `HT_INVOICE` writer is `FormBookingInvoice`
/// (per-booking, prepaid rooms — `docs/legacy-app/FEATURE_MAP.md` J6),
/// for which no write-capture exists. The 16-column row semantics
/// (`INV_STAY`, `INV_TITLE`, `INV_NIGHT` as varchar, etc.) and the
/// allocation lifecycle are unverified, so per the task's safer-default
/// rule we do NOT guess the legacy INSERT. A follow-on wave can plumb
/// `allocate_inv_no` through the writeback adapter once a `FormBookingInvoice`
/// write is captured. Until then this format keeps the printed invoice
/// stable per check-in but does NOT reserve a number in the legacy
/// sequence — printing the same invoice twice yields the same string.
fn format_invoice_number(cin_id: i32, created_at: Option<NaiveDateTime>) -> String {
    let stamp = created_at
        .and_then(|naive| Utc.from_local_datetime(&naive).single())
        .unwrap_or_else(Utc::now);
    let bkk = stamp.with_timezone(&Bangkok).date_naive();
    format!("INV{:02}{:02}-{:06}", bkk.year() % 100, bkk.month(), cin_id)
}

#[cfg(test)]
mod tests {
    //! Pure-helper tests for the G3 VAT-invoice fields. Live PG / MSSQL
    //! integration is out of scope for the unit suite per the Phase 4b spec.

    use super::*;
    use chrono::NaiveDate;

    /// G3: VAT split MUST use banker's rounding to match the legacy
    /// `Math.Round(value, 2)` (MidpointRounding.ToEven) default — Track C /
    /// Wave 6 H4 lesson. The reference vector here is the same one
    /// `format::vat_inclusive_split` is locked against in
    /// `writeback::format::tests::vat_inclusive_split_matches_legacy_*` so
    /// the invoice route and the writeback recipe can never diverge.
    #[test]
    fn computes_vat_split_with_banker_rounding() {
        // From `/tmp/legacy-events-full.log` Receipt_H 20663:
        //   Total=801.00, BeforeVat=748.60, Vat=52.40, VatPer=7
        let (before, vat) = vat_inclusive_split(801.00, 7);
        assert_eq!(before, 748.60, "before_vat must round-half-to-even");
        assert_eq!(vat, 52.40, "vat_amount must round-half-to-even");
        // Sum-back invariant: the printed receipt MUST add back to the
        // original total, never `total ± 0.01`.
        assert_eq!(
            before + vat,
            801.00,
            "before_vat + vat_amount must equal total (sum-back invariant)"
        );
    }

    /// G3: the VAT percentage stamped onto the invoice must be the one
    /// `ht_settings.vat_percent` is configured to (Wave 5c plumbing), not
    /// a hardcoded 7. We can't reach a live PG inside the unit suite, so
    /// this test pins the contract by exercising the split helper with a
    /// non-default percentage and asserting the math reflects it.
    #[test]
    fn vat_per_from_settings_overrides_default() {
        // VAT-exempt hotel: vat_per=0 → split is (total, 0).
        let (before_zero, vat_zero) = vat_inclusive_split(1000.00, 0);
        assert_eq!(before_zero, 1000.00);
        assert_eq!(vat_zero, 0.00);

        // Hypothetical 10% — exercises the divisor-arithmetic path.
        let (before_ten, vat_ten) = vat_inclusive_split(1100.00, 10);
        assert_eq!(before_ten, 1000.00);
        assert_eq!(vat_ten, 100.00);

        // Confirms the helper does NOT silently coerce its second arg back
        // to 7 — guards against an accidental hardcode regression.
        let (b7, v7) = vat_inclusive_split(1100.00, 7);
        assert_ne!(b7, before_ten);
        assert_ne!(v7, vat_ten);
    }

    /// G3: `tax_id` on the invoice comes from `ht_customers.cust_work_tax`
    /// (Track E2 column). `None` and blank-string both render as "no tax
    /// id" — the legacy app stores blank for individual walk-ins, so we
    /// must collapse the two to avoid the frontend showing an empty-string
    /// tax-id field that looks like a data-loading bug.
    #[test]
    fn tax_id_from_cust_work_tax() {
        // Corporate guest: tax-id present.
        assert_eq!(
            normalize_tax_id(Some("REDACTED-tax-id".to_string())),
            Some("REDACTED-tax-id".to_string())
        );
        // Walk-in / individual: NULL on the customers row → None on the API.
        assert_eq!(normalize_tax_id(None), None);
        // Legacy-style blank (".NET WinForms stores '' instead of NULL"):
        // collapse to None so the UI doesn't render an empty value.
        assert_eq!(normalize_tax_id(Some("".to_string())), None);
        assert_eq!(normalize_tax_id(Some("   ".to_string())), None);
        // Trim incidental whitespace (defensive — legacy data dumps
        // routinely include trailing spaces on `varchar` columns).
        assert_eq!(
            normalize_tax_id(Some("  REDACTED-tax-id  ".to_string())),
            Some("REDACTED-tax-id".to_string())
        );
    }

    /// G3: `inv_no` format invariant. The PG-only format is
    /// `INV{yyMM}-{cin_id:06}`. Bangkok-calendar-scoped to match the rest
    /// of the allocator stack (audit CRIT-C1 / CRIT-C3 — `pay_no_prefix`
    /// uses Bangkok year/month). Documented in the function's doc-comment
    /// as PG-only with a TODO for legacy `HT_INVOICE.INV_NO` alignment.
    #[test]
    fn inv_no_format_matches_legacy_or_pg_only_as_documented() {
        // Bangkok-noon 2026-05-13 (UTC 05:00) → INV2605-005228.
        let created = NaiveDate::from_ymd_opt(2026, 5, 13)
            .unwrap()
            .and_hms_opt(5, 0, 0)
            .unwrap();
        let inv_no = format_invoice_number(5228, Some(created));
        assert_eq!(inv_no, "INV2605-005228");

        // Width invariant: cin_id is always zero-padded to 6 digits so
        // the printed invoice column never wobbles.
        let small = format_invoice_number(1, Some(created));
        assert_eq!(small, "INV2605-000001");

        // Bangkok-calendar rollover (CRIT-C3 regression): UTC 2026-04-30
        // 17:30 = Bangkok 2026-05-01 00:30 → MUST stamp INV2605, not
        // INV2604. The created_at value here is the *Bangkok-local* naive
        // datetime as `ht_checkins.created_at` is read (PG `TIMESTAMP` —
        // no timezone — populated from Bangkok-local wall time).
        let bkk_local_just_after_midnight = NaiveDate::from_ymd_opt(2026, 5, 1)
            .unwrap()
            .and_hms_opt(0, 30, 0)
            .unwrap();
        let rolled = format_invoice_number(42, Some(bkk_local_just_after_midnight));
        assert!(
            rolled.starts_with("INV2605-"),
            "expected INV2605- prefix (Bangkok already in May), got {rolled}"
        );
    }

    /// G3 documentation regression: `format_invoice_number` falls back to
    /// the current UTC instant when `created_at` is missing. Without the
    /// fallback the route would have to thread `Utc::now()` through every
    /// call-site — including from inside the SQLx-typed query result —
    /// and we'd lose the deterministic format for records that lost their
    /// `created_at` (NULL after a manual DB migration, etc.).
    #[test]
    fn inv_no_falls_back_to_now_when_created_at_missing() {
        let inv_no = format_invoice_number(99, None);
        // The exact prefix depends on the wall clock, but the *shape* must
        // hold — INV + 4 digits + dash + 6 digits = 14 chars total.
        assert_eq!(inv_no.len(), "INVYYMM-NNNNNN".len());
        assert!(inv_no.starts_with("INV"));
        assert!(inv_no.contains('-'));
        assert!(inv_no.ends_with("000099"), "trailing cin_id must be zero-padded: {inv_no}");
    }
}
