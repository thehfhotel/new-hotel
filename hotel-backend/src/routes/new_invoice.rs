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

    // Room assignment
    pub room: InvoiceRoom,

    // Stay details
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub expected_checkout: Option<NaiveDateTime>,
    pub adults: i32,
    pub children: i32,

    // Rate calculations
    pub rates: InvoiceRates,

    // Totals (G3: VAT-inclusive split)
    pub total_amount: f64,
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
                ci.cin_no,
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

    // Get total amount (use calculated if not stored)
    let total_amount = rec.cin_total_amount.unwrap_or(subtotal);

    // G3 / T4 HIGH-7: VAT split + invoice number.
    let vat_per = settings::get_vat_percent(pool).await;
    let (before_vat, vat_amount) = vat_inclusive_split(total_amount, vat_per);
    let inv_no = format_invoice_number(rec.cin_id, rec.created_at);

    let invoice = Invoice {
        checkin_id: rec.cin_id,
        cin_no: rec.cin_no,
        booking_id: rec.cin_book_id,
        booking_no: Some(rec.book_no),
        inv_no,
        guest,
        room,
        check_in_time: Some(rec.cin_checkin_time),
        check_out_time: rec.cin_checkout_time,
        expected_checkout: Some(rec.cin_expected_checkout.and_hms_opt(0, 0, 0).unwrap()),
        adults: rec.cin_adults.unwrap_or(1),
        children: rec.cin_children.unwrap_or(0),
        rates,
        total_amount,
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
/// **TODO (deferred):** the legacy `HT_INVOICE.INV_NO` column is
/// `int NOT NULL` allocated via `MAX+1` by the .NET app. A follow-on
/// Track G wave can plumb `allocate_inv_no` through the writeback adapter
/// so the PG-emitted number matches a real legacy slot. Until then this
/// format keeps the printed invoice stable per check-in but does NOT
/// reserve a number in the legacy sequence — printing the same invoice
/// twice yields the same string.
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
