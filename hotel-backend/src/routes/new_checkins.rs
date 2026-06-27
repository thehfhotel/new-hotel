//! New Check-in API routes for HotelNew database
//!
//! - GET /api/new/checkins - List check-ins from HT_CheckIns
//! - GET /api/new/checkins/:id - Get single check-in
//! - POST /api/new/checkins - Create check-in (walk-in or from booking)
//! - PUT /api/new/checkins/:id/checkout - Process check-out
//! - PUT /api/new/checkins/:id/extend - Extend stay (Track G1 / T4 HIGH-2)
//!
//! Guest Registry endpoints:
//! - GET /api/new/checkins/:id/guests - List guests for check-in
//! - POST /api/new/checkins/:id/guests - Add guest to check-in
//! - DELETE /api/new/checkins/:id/guests/:guestId - Remove guest from check-in
//!
//! Per `docs/architecture.md` §1, §6 (Phase 2.5) the create / checkout / extend
//! handlers delegate to `state.checkins_service`. Reads + the guest-registry
//! handlers stay on the repository for now (no service method exists yet).

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::domain::shared::{DateRange, Money};
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::outbox::event::EventSource;
use crate::repository::checkin::{
    CheckInDetailRow, CheckInListRow, GuestInsert, GuestRow,
};
use crate::service::{
    ChangeRoomCommand, ChangeRoomOutcome, CheckInToBookingCommand, CheckInWritebackContext,
    CheckOutCommand, ExtendStayCommand, ServiceError, WalkInCommand,
};

/// Check-in status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckInStatus {
    Active,
    CheckedOut,
    Cancelled,
}

impl CheckInStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "checkedout" | "checked_out" => CheckInStatus::CheckedOut,
            "cancelled" | "canceled" => CheckInStatus::Cancelled,
            _ => CheckInStatus::Active,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CheckInStatus::Active => "active",
            CheckInStatus::CheckedOut => "checkedout",
            CheckInStatus::Cancelled => "cancelled",
        }
    }
}

/// Check-in from HT_CheckIns table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckIn {
    pub id: i32,
    pub cin_no: String,
    pub booking_id: Option<i32>,
    pub booking_no: Option<String>,
    pub customer_id: i32,
    pub customer_name: Option<String>,
    pub room_id: i32,
    pub room_no: Option<String>,
    pub room_type_name: Option<String>,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub expected_checkout: Option<NaiveDateTime>,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub status: String,
    pub rate_per_night: Option<f64>,
    pub total_amount: Option<f64>,
    pub payment_status: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewCheckIn {
    fn from_list_row(row: CheckInListRow) -> Self {
        Self {
            id: row.cin_id,
            cin_no: row.cin_no,
            booking_id: row.cin_book_id,
            booking_no: row.book_no,
            customer_id: row.cin_cust_id,
            customer_name: row.customer_name,
            room_id: row.cin_room_id,
            room_no: row.room_no,
            room_type_name: row.type_name,
            check_in_time: row.cin_checkin_time,
            check_out_time: row.cin_checkout_time,
            expected_checkout: row.cin_expected_checkout,
            adults: row.cin_adults,
            children: row.cin_children,
            status: row.cin_status,
            rate_per_night: row.cin_rate_per_night,
            total_amount: row.cin_total_amount,
            payment_status: row.cin_payment_status,
            notes: row.cin_notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }

    fn from_detail_row(row: CheckInDetailRow) -> Self {
        Self {
            id: row.cin_id,
            cin_no: row.cin_no,
            booking_id: row.cin_book_id,
            booking_no: Some(row.book_no),
            customer_id: row.cin_cust_id,
            customer_name: row.customer_name,
            room_id: row.cin_room_id,
            room_no: Some(row.room_no),
            room_type_name: Some(row.type_name),
            check_in_time: Some(row.cin_checkin_time),
            check_out_time: row.cin_checkout_time,
            expected_checkout: Some(row.cin_expected_checkout.and_hms_opt(0, 0, 0).unwrap()),
            adults: row.cin_adults,
            children: row.cin_children,
            status: row.cin_status.unwrap_or_else(|| "active".to_string()),
            rate_per_night: row.cin_rate_per_night,
            total_amount: row.cin_total_amount,
            payment_status: row.cin_payment_status,
            notes: row.cin_notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Query parameters for check-ins list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckInsQuery {
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub room_id: Option<i32>,
    pub customer_id: Option<i32>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    /// Branch selector: 'hfhotel' | 'hfville' | 'all' (HotelNew only contains hfhotel data)
    pub branch: Option<Branch>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }

/// Response for check-ins list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckInsResponse {
    pub success: bool,
    pub data: Vec<NewCheckIn>,
    pub pagination: Pagination,
}

/// Response for single check-in
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckInResponse {
    pub success: bool,
    pub checkin: NewCheckIn,
}

/// Request body for creating check-in
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCheckInRequest {
    /// Booking ID if checking in from booking (optional for walk-ins)
    pub booking_id: Option<i32>,
    /// Customer ID (required for walk-ins, optional if booking_id provided)
    pub customer_id: Option<i32>,
    /// Room ID to check into
    pub room_id: i32,
    /// Check-in time (defaults to now)
    pub check_in_time: Option<String>,
    /// Expected check-out date/time
    pub expected_checkout: String,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub rate_per_night: Option<f64>,
    pub notes: Option<String>,
}

/// Request body for checkout
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckOutRequest {
    pub check_out_time: Option<String>,
    pub total_amount: Option<f64>,
    pub payment_status: Option<String>,
    pub notes: Option<String>,
}

/// Request body for extend stay.
///
/// Track G1 / T4 HIGH-2 (`docs/coexistence/audit-2026-05-13.md`). The
/// receptionist supplies the new (later) departure date and an optional
/// free-form reason. The route validates that the new date is strictly
/// later than the existing `cin_expected_checkout` before delegating to
/// [`crate::service::CheckInService::extend`] — shortening a stay is a
/// separate flow (refunds / partial-stay charges) tracked under Track G2.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendStayRequest {
    /// New (later) departure date, ISO `YYYY-MM-DD`. Must be strictly after
    /// the existing `cin_expected_checkout`.
    pub new_checkout_date: NaiveDate,
    /// Optional human-readable reason recorded against the writeback. Not
    /// persisted in the canonical PG state today (no `extend_reason`
    /// column); reserved for future audit-trail surface.
    #[serde(default)]
    pub reason: Option<String>,
}

/// Response for create/checkout operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cin_no: Option<String>,
}

/// GET /api/new/checkins - List check-ins from HT_CheckIns
pub async fn list_checkins(
    State(state): State<AppState>,
    Query(params): Query<NewCheckInsQuery>,
) -> ApiResult<Json<NewCheckInsResponse>> {
    // Branch-aware: HF Hotel reads new_pool, HF Ville reads ville_pool
    // (hotelville's canonical ht_checkins is populated), All unions both —
    // mirroring routes/rooms.rs::list_rooms.
    let (rows, total) = match params.branch.unwrap_or_default() {
        Branch::Hfhotel => state.checkins.list_with_count(&state.new_pool, &params).await?,
        Branch::Hfville => state.checkins.list_with_count(state.ville_pool()?, &params).await?,
        Branch::All => {
            let (mut r, mut t) = state.checkins.list_with_count(&state.new_pool, &params).await?;
            if let Ok(vp) = state.ville_pool() {
                let (vr, vt) = state.checkins.list_with_count(vp, &params).await?;
                r.extend(vr);
                t += vt;
            }
            (r, t)
        }
    };

    let checkins: Vec<NewCheckIn> = rows.into_iter().map(NewCheckIn::from_list_row).collect();

    Ok(Json(NewCheckInsResponse {
        success: true,
        data: checkins,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/new/checkins/:id - Get single check-in
pub async fn get_checkin(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<NewCheckInResponse>> {
    let row = state
        .checkins
        .get(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    Ok(Json(NewCheckInResponse {
        success: true,
        checkin: NewCheckIn::from_detail_row(row),
    }))
}

/// POST /api/new/checkins - Create check-in (walk-in or from booking)
///
/// Delegates to either [`crate::service::CheckInService::walk_in`] or
/// [`crate::service::CheckInService::check_in_to_booking`] depending on
/// whether a `booking_id` was provided.
pub async fn create_checkin(
    State(state): State<AppState>,
    Json(body): Json<CreateCheckInRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let cin_no = generate_cin_no(&state).await?;
    let expected_checkout = parse_expected_checkout(&body.expected_checkout)?;
    let check_in_time = parse_check_in_time(body.check_in_time.as_deref())?;
    // Walk-ins use `body.customer_id`; booking-linked check-ins resolve the
    // customer through the booking. Both feed the writeback recipe (`walkin`
    // or `checkin_to_booking`) which copies `guest_name_for_registry` into
    // both `HT_Customers.Cust_name` and `HT_CheckIn_Other_People.Cin_name`.
    let resolved_customer_id = match body.booking_id {
        Some(booking_id) => state
            .checkins
            .get_booking_customer_id(&state.new_pool, booking_id)
            .await?,
        None => body.customer_id,
    };
    let writeback_context =
        build_check_in_writeback_context(&state, &body, expected_checkout, resolved_customer_id)
            .await?;
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);
    // TODO: wire user_id from auth middleware
    let source = EventSource::our_app(Uuid::nil(), Uuid::new_v4());

    let outcome = match body.booking_id {
        Some(booking_id) => state
            .checkins_service
            .check_in_to_booking(CheckInToBookingCommand {
                cin_no: cin_no.clone(),
                booking_id,
                room_id: body.room_id,
                check_in_time,
                expected_checkout,
                adults,
                children,
                rate_per_night: body.rate_per_night,
                notes: body.notes.clone(),
                writeback_context,
                source,
            })
            .await
            .map_err(map_create_checkin_error)?,
        None => {
            let customer_id = body.customer_id.ok_or_else(|| {
                ApiError::BadRequest("Customer ID is required for walk-ins".to_string())
            })?;
            state
                .checkins_service
                .walk_in(WalkInCommand {
                    cin_no: cin_no.clone(),
                    customer_id,
                    room_id: body.room_id,
                    check_in_time,
                    expected_checkout,
                    adults,
                    children,
                    rate_per_night: body.rate_per_night,
                    notes: body.notes.clone(),
                    writeback_context,
                    source,
                })
                .await
                .map_err(map_create_checkin_error)?
        }
    };

    Ok(Json(MutationResponse {
        success: true,
        message: "Check-in created successfully".to_string(),
        id: Some(outcome.check_in_id),
        cin_no: Some(cin_no),
    }))
}

/// `200` body for the checkout quote (spike Phase 2).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutQuote {
    pub success: bool,
    /// Nights billed = `(COALESCE(checkout, expected_checkout)::date − checkin::date)`,
    /// min 1 — the canonical server computation that backs the receipt + writeback.
    pub nights: i32,
    pub rate_per_night: f64,
    /// `nights × rate` (room charge). Products/VAT/deposits are not yet plumbed
    /// server-side — an iHOTEL `FrmCheckOut` parity follow-up.
    pub room_total: f64,
}

/// GET /api/checkins/{id}/checkout-quote — the server-authoritative checkout
/// total (read-only). Returns the same `check_in_billing` numbers the receipt
/// and the iHOTEL writeback already use, so the UI can DISPLAY the canonical
/// total instead of computing `ceil(now − checkin) × rate` client-side. Gated
/// on `CHECKOUT_SERVER_TOTAL_ENABLED` at the UI layer (spike Phase 2, ship-dark).
pub async fn checkout_quote(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<CheckoutQuote>> {
    let billing = state
        .payments
        .check_in_billing(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;
    let nights = billing.nights.unwrap_or(1).max(1);
    let rate = billing.cin_rate_per_night.unwrap_or(0.0);
    Ok(Json(CheckoutQuote {
        success: true,
        nights,
        rate_per_night: rate,
        room_total: rate * nights as f64,
    }))
}

/// PUT /api/new/checkins/:id/checkout - Process check-out
pub async fn checkout(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<CheckOutRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // The service runs the same status/active check, but we still need
    // `cin_no` for the response (the service outcome only carries the id).
    let status_snap = state
        .checkins
        .find_status(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;
    let cin_no = status_snap.cin_no;

    let check_out_time = parse_check_out_time(body.check_out_time.as_deref())?;

    // Audit H1: thread real revenue + nights into the writeback intent so
    // the MSSQL `Total_Price_*` UPDATEs stop wiping legacy revenue. Numbers
    // come from the canonical PG state via the payment-billing repo (same
    // query that backs the receipt totals API).
    let billing = state
        .payments
        .check_in_billing(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;
    let nights = billing.nights.unwrap_or(1).max(1) as f64;
    let rate = billing.cin_rate_per_night.unwrap_or(0.0);
    let room_price_total = rate * nights;
    let pay_total = body.total_amount.or(billing.cin_total_amount).unwrap_or(0.0);
    let net_total = room_price_total; // No product/extras plumbing yet.
    let balance = (net_total - pay_total).max(0.0);

    // Spike Phase 2 SHADOW (CHECKOUT_SERVER_TOTAL_ENABLED ships dark): log when
    // the client-submitted total diverges from the server-authoritative room
    // total, so the delta (mostly the actual- vs expected-nights basis) can be
    // sized on real folios before flipping the flag. The client value is still
    // what's charged here; this is observation only.
    if let Some(client_total) = body.total_amount {
        if (client_total - room_price_total).abs() > 0.01 {
            tracing::info!(
                target: "shadow.checkout_total",
                cin_id,
                cin_no = %cin_no,
                nights,
                rate,
                client_total,
                server_room_total = room_price_total,
                delta = client_total - room_price_total,
                "checkout-total shadow: client vs server-authoritative total diverge (charged client value — flag off)"
            );
        }
    }

    let outcome = state
        .checkins_service
        .check_out(CheckOutCommand {
            check_in_id: cin_id,
            check_out_time,
            total_amount: body.total_amount,
            payment_status: body
                .payment_status
                .clone()
                .unwrap_or_else(|| "paid".to_string()),
            notes: body.notes.clone(),
            // TODO: wire user_id from auth middleware
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
            nights,
            room_price_total,
            product_total: 0.0,
            net_total,
            pay_total,
            balance,
        })
        .await
        .map_err(map_checkout_error)?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Check-out completed successfully".to_string(),
        id: Some(outcome.check_in_id),
        cin_no: Some(cin_no),
    }))
}

/// PUT /api/new/checkins/:id/extend - Extend an active stay.
///
/// Track G1 / T4 HIGH-2 (`docs/coexistence/audit-2026-05-13.md`). Wraps
/// the pre-existing [`crate::service::CheckInService::extend`] (service
/// landed in v2.x; recipe `writeback::recipes::extend_stay` already
/// emits the legacy `HT_Book_Date` / `HT_CheckIn_Ds` / `HT_Room_Status`
/// diff per spike §3f). This handler is the "open the door" wiring so
/// receptionists no longer fall back to iHOTEL for the common
/// "one-more-night" request.
///
/// Validation: the new departure date MUST be strictly after the existing
/// `cin_expected_checkout`. Shortening a stay routes through a separate
/// refund / partial-stay flow (Track G2) which doesn't exist yet — a 400
/// here keeps the rule explicit instead of silently dropping nights.
pub async fn extend(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<ExtendStayRequest>,
) -> ApiResult<Json<NewCheckInResponse>> {
    // Pull the detail row up front so we can (a) validate the date,
    // (b) compute the recipe inputs (stay_start / rate / nights /
    // guest_label), and (c) return the post-extend payload.
    let detail = state
        .checkins
        .get(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    validate_new_checkout_after_existing(
        body.new_checkout_date,
        detail.cin_expected_checkout,
    )?;

    let command = build_extend_stay_command(&detail, &body);
    state
        .checkins_service
        .extend(command)
        .await
        .map_err(map_extend_error)?;

    // Re-fetch so the response carries any state the service may have
    // updated (today the service emits a writeback only and does not touch
    // PG `cin_expected_checkout` — that lives on the worker side via the
    // recipe. A follow-up wave will land a PG `apply_extend` so the
    // canonical state and the response stay in lock-step).
    let updated = state
        .checkins
        .get(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    Ok(Json(NewCheckInResponse {
        success: true,
        checkin: NewCheckIn::from_detail_row(updated),
    }))
}

// ---------- create/checkout helpers ----------

async fn generate_cin_no(state: &AppState) -> ApiResult<String> {
    let last_cin_no = state.checkins.latest_cin_no_today(&state.new_pool).await?;
    let next_seq = last_cin_no
        .as_deref()
        .and_then(|s| s.split('-').nth(2))
        .and_then(|n| n.parse::<i32>().ok())
        .map(|n| n + 1)
        .unwrap_or(1);
    let today = state.checkins.today_yyyymmdd(&state.new_pool).await?;
    Ok(format!("CIN-{}-{:04}", today, next_seq))
}

fn parse_expected_checkout(raw: &str) -> ApiResult<NaiveDate> {
    NaiveDate::parse_from_str(raw, "%Y-%m-%d").map_err(|_| {
        ApiError::BadRequest(
            "Invalid expected checkout date format (expected YYYY-MM-DD)".to_string(),
        )
    })
}

fn parse_check_in_time(raw: Option<&str>) -> ApiResult<Option<NaiveDateTime>> {
    let Some(s) = raw else { return Ok(None) };
    let parsed = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms_opt(14, 0, 0).unwrap())
        })
        .map_err(|_| ApiError::BadRequest("Invalid check-in time format".to_string()))?;
    Ok(Some(parsed))
}

fn parse_check_out_time(raw: Option<&str>) -> ApiResult<Option<NaiveDateTime>> {
    let Some(s) = raw else { return Ok(None) };
    let parsed = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms_opt(12, 0, 0).unwrap())
        })
        .map_err(|_| ApiError::BadRequest("Invalid check-out time format".to_string()))?;
    Ok(Some(parsed))
}

/// Build a [`CheckInWritebackContext`] from the request body + PG lookups.
///
/// The walk-in recipe (spike §3a) uses `guest_name_for_registry` for both
/// `HT_Customers.Cust_name` and `HT_CheckIn_Other_People.Cin_name`. The
/// linked-to-booking recipe (§3d) uses it for the `HT_CheckIn_Ds.room_Details`
/// label that shows on the room grid. Either way, it MUST be populated or the
/// .NET app shows the row with blank guest. Same story for `room_no` /
/// `room_type` (which lands on `HT_CheckIn_Ds.Cin_Room_No` / `Cin_Room_Type`)
/// and `price_per_night` (`HT_CheckIn_Ds.Cin_Room_Price`).
async fn build_check_in_writeback_context(
    state: &AppState,
    body: &CreateCheckInRequest,
    expected_checkout: NaiveDate,
    resolved_customer_id: Option<i32>,
) -> ApiResult<CheckInWritebackContext> {
    use chrono::{NaiveTime, TimeZone, Utc};

    let midnight = NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight is valid");
    let now = chrono::Local::now().date_naive();
    let stay_start = Utc.from_utc_datetime(&now.and_time(midnight));
    let stay_end = Utc.from_utc_datetime(&expected_checkout.and_time(midnight));
    let nights = (expected_checkout - now).num_days().max(1) as i32;

    // Look up the room first — its weekday price is the fallback when the
    // request omits `rate_per_night`. Ditto for room_no / room_type which
    // the recipe inserts verbatim into HT_CheckIn_Ds.
    let room = state.rooms.get(&state.new_pool, body.room_id).await?;
    let (room_no, room_type, default_weekday) = match room {
        Some(r) => (
            r.room_no,
            r.type_name.unwrap_or_default(),
            r.room_price_weekday,
        ),
        None => (String::new(), String::new(), None),
    };

    let price_per_night = body
        .rate_per_night
        .or(default_weekday)
        .map(money_from_baht_f64)
        .unwrap_or(Money::ZERO);
    let price_total =
        Money::from_satang(price_per_night.as_satang().saturating_mul(nights as i64));

    // Customer lookup. The walkin recipe also INSERTs HT_Customers from this
    // value when legacy_cust_no is None, so an empty name here results in a
    // blank-named customer in legacy. We try hard, but fall back to empty
    // rather than failing the check-in (matches build_writeback_context for
    // bookings — degrade gracefully on missing FKs).
    //
    // Wave 5a item 1: also surface the customer phone so the linked-to-booking
    // writeback recipe doesn't wipe `HT_Customers.Cust_Add_tel` on every
    // check-in (the prior code passed `None` unconditionally to
    // `checkin_to_booking::execute`).
    let (guest_name, customer_phone) = match resolved_customer_id {
        Some(cust_id) => match state.customers.get(&state.new_pool, cust_id).await? {
            Some(c) => (
                full_customer_name(&c.cust_firstname, c.cust_lastname.as_deref()),
                c.cust_phone,
            ),
            None => (String::new(), None),
        },
        None => (String::new(), None),
    };

    Ok(CheckInWritebackContext {
        legacy_cust_no: None,
        linked_legacy_book_id: None,
        room_no,
        room_type,
        stay: DateRange::new(stay_start, stay_end),
        price_per_night,
        nights,
        price_total,
        created_by: String::new(),
        guest_name_for_registry: guest_name,
        guest_country: String::new(),
        customer_phone,
    })
}

/// Join `cust_firstname` + `cust_lastname` with a single space, dropping the
/// trailing space when the last name is missing. Mirrors the
/// `HT_Customers.Cust_name` shape (legacy stores the joined value).
fn full_customer_name(first: &str, last: Option<&str>) -> String {
    match last {
        Some(last) if !last.is_empty() => format!("{} {}", first, last),
        _ => first.to_string(),
    }
}

fn money_from_baht_f64(baht: f64) -> Money {
    let satang = (baht * 100.0).round() as i64;
    Money::from_satang(satang)
}

/// Translate the service's `Conflict` outcome (room already occupied) to
/// the route's prior 400 wording so the wire contract is preserved.
fn map_create_checkin_error(err: ServiceError) -> ApiError {
    match err {
        ServiceError::Conflict(_) => ApiError::BadRequest("Room is currently occupied".to_string()),
        other => other.into(),
    }
}

/// Translate the service's `Conflict` outcome (check-in not active) to
/// the route's prior 400 wording.
fn map_checkout_error(err: ServiceError) -> ApiError {
    match err {
        ServiceError::Conflict(_) => ApiError::BadRequest("Check-in is not active".to_string()),
        other => other.into(),
    }
}

// ---------- extend-stay helpers ----------

/// Reject requests that shorten or no-op the stay. Refund / partial-stay
/// flows route through Track G2; until that lands, the route rejects loudly
/// instead of silently dropping nights from the canonical state.
fn validate_new_checkout_after_existing(
    new_checkout_date: NaiveDate,
    existing_expected_checkout: NaiveDate,
) -> ApiResult<()> {
    if new_checkout_date <= existing_expected_checkout {
        return Err(ApiError::BadRequest(format!(
            "New checkout date ({}) must be strictly after existing expected \
             checkout ({}). Shortening a stay is not supported by this endpoint.",
            new_checkout_date, existing_expected_checkout
        )));
    }
    Ok(())
}

/// Build the [`ExtendStayCommand`] from the canonical PG detail row + the
/// HTTP body. Pure (no IO, no clock reads) so the test suite can pin the
/// translation contract without spinning up an `AppState`.
///
/// Totals mirror the checkout convention (audit H1): rate × nights for
/// room price + net, current `cin_total_amount` for pay, max(0) for the
/// remaining balance. The writeback recipe re-aggregates from
/// `HT_CheckIn_Pay` anyway (Wave 5a item 2) — these values are only the
/// fallback that lands on `HT_CheckIn_H` if no payments exist legacy-side.
pub(crate) fn build_extend_stay_command(
    detail: &CheckInDetailRow,
    body: &ExtendStayRequest,
) -> ExtendStayCommand {
    use chrono::{NaiveTime, TimeZone, Utc};

    // Same UTC-midnight convention as the create flow (see
    // `build_check_in_writeback_context`). The audit's BKK-midnight
    // theme will refactor both call sites in a single Track A wave.
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight is valid");
    let stay_start = Utc.from_utc_datetime(
        &detail.cin_checkin_time.date().and_time(midnight),
    );
    let new_end = Utc.from_utc_datetime(&body.new_checkout_date.and_time(midnight));

    let nights_new = (body.new_checkout_date - detail.cin_checkin_time.date())
        .num_days()
        .max(1) as f64;
    let rate = detail.cin_rate_per_night.unwrap_or(0.0);
    let room_price_total_baht = rate * nights_new;
    let pay_total_baht = detail.cin_total_amount.unwrap_or(0.0);
    let balance_total_baht = (room_price_total_baht - pay_total_baht).max(0.0);

    let new_room_price_total = money_from_baht_f64(room_price_total_baht);
    let new_net_total = new_room_price_total; // no product/extras plumbing yet
    let new_pay_total = money_from_baht_f64(pay_total_baht);
    let new_balance_total = money_from_baht_f64(balance_total_baht);

    // The `reason` field is reserved for a future audit-trail surface; the
    // canonical PG schema doesn't have a column for it yet (no migration
    // landed for `cin_extend_reason`). Keeping it on the wire so the UI
    // can start collecting the value today and we don't need a v2 of the
    // request shape when the column lands.
    let _ = &body.reason;

    ExtendStayCommand {
        check_in_id: detail.cin_id,
        new_end,
        stay_start,
        guest_label: detail.customer_name.clone().unwrap_or_default(),
        new_room_price_total,
        new_net_total,
        new_pay_total,
        new_balance_total,
        // TODO: wire user_id from auth middleware (parity with checkout/create)
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    }
}

/// Translate service outcomes to the route's HTTP semantics. Mirrors the
/// shape of `map_checkout_error` so receptionists see the same wording
/// across both "not active" guard paths.
fn map_extend_error(err: ServiceError) -> ApiError {
    match err {
        ServiceError::Conflict(_) => ApiError::BadRequest("Check-in is not active".to_string()),
        other => other.into(),
    }
}

// =============================================================================
// Change-room endpoint — Track G4 / T4 HIGH-3
// =============================================================================

/// Request body for `POST /api/new/checkins/:id/change-room`.
///
/// Track G4 / T4 HIGH-3. The receptionist supplies the room they're
/// moving from (must currently sit in the folio's junction) and the
/// room they're moving to (must be free of any other active check-in).
/// `reason` is an optional free-text note that lands in the audit row
/// (`ht_room_changes.rc_reason` + legacy `HT_Changed_Room.Note`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRoomRequest {
    pub from_room_id: i32,
    pub to_room_id: i32,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Response for `POST /api/new/checkins/:id/change-room`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRoomResponse {
    pub success: bool,
    pub message: String,
    pub check_in_id: i32,
    pub rc_id: i64,
    pub from_room_id: i32,
    pub to_room_id: i32,
}

/// `POST /api/new/checkins/:id/change-room` — move an active guest
/// from one room to another mid-stay.
///
/// Track G4 / T4 HIGH-3 (`docs/coexistence/audit-2026-05-13.md`). The
/// route delegates the full validation + transaction to
/// [`crate::service::CheckInService::change_room`]. Returns the freshly
/// allocated `rc_id` so the UI can link directly to the audit row.
pub async fn change_room(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<ChangeRoomRequest>,
) -> ApiResult<Json<ChangeRoomResponse>> {
    let command = build_change_room_command(cin_id, &body);
    let outcome = state
        .checkins_service
        .change_room(command)
        .await
        .map_err(map_change_room_error)?;
    Ok(Json(build_change_room_response(&body, &outcome)))
}

/// PURE — translate the path id + JSON body into a service command.
/// Lives at module scope so the unit tests can pin the contract without
/// spinning up an [`AppState`].
pub(crate) fn build_change_room_command(
    cin_id: i32,
    body: &ChangeRoomRequest,
) -> ChangeRoomCommand {
    ChangeRoomCommand {
        check_in_id: cin_id,
        from_room_id: body.from_room_id,
        to_room_id: body.to_room_id,
        reason: body.reason.clone(),
        // Operator name lands here when G7 auth middleware wires
        // session → state. Empty string is the canonical "unknown"
        // sentinel (mirrors how walkin handles `created_by`).
        actor: String::new(),
        // TODO: wire user_id from auth middleware (parity with extend/checkout).
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    }
}

/// PURE — translate the service outcome + request body into the
/// HTTP response shape.
pub(crate) fn build_change_room_response(
    body: &ChangeRoomRequest,
    outcome: &ChangeRoomOutcome,
) -> ChangeRoomResponse {
    ChangeRoomResponse {
        success: true,
        message: "Room change recorded".to_string(),
        check_in_id: outcome.check_in_id,
        rc_id: outcome.rc_id,
        from_room_id: body.from_room_id,
        to_room_id: body.to_room_id,
    }
}

/// Translate service outcomes to HTTP. Validation failures (same room,
/// from-room not in folio) surface as 400; conflict (target room
/// occupied, source check-in not active) surfaces as 400 with explicit
/// wording so the modal can show the receptionist what went wrong.
fn map_change_room_error(err: ServiceError) -> ApiError {
    match err {
        ServiceError::Conflict(msg) => ApiError::BadRequest(msg),
        ServiceError::Validation(msg) => ApiError::BadRequest(msg),
        other => other.into(),
    }
}

// =============================================================================
// POS / sales-to-room endpoints — Track G6 (MVP)
// =============================================================================

/// Request body for `POST /api/new/checkins/:id/pos-sale`.
///
/// Track G6. The cashier picks a product from F3's catalog
/// (`ht_products`) and rings up `qty` units against the active
/// folio. `unit_price_override` is optional — pass `null` to use the
/// catalog price; pass a non-NULL value to override (e.g. happy-hour
/// pricing). `note` is an optional free-text annotation captured in
/// `ht_pos_sales.sale_note` and propagated to legacy
/// `HT_CheckIn_Product.Cin_Pro_note`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PosSaleRequest {
    pub product_id: i64,
    pub qty: f64,
    #[serde(default)]
    pub unit_price_override: Option<f64>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Response for `POST /api/new/checkins/:id/pos-sale`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PosSaleResponse {
    pub success: bool,
    pub message: String,
    pub sale_id: i64,
    pub aggregate_id: Uuid,
    pub check_in_id: i32,
    pub product_id: i64,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub total_baht: f64,
    pub sold_at: chrono::DateTime<chrono::Utc>,
}

/// Single POS sale row surfaced by
/// `GET /api/new/checkins/:id/pos-sales`. Joined with `ht_products`
/// so the receptionist UI doesn't need a second round-trip to render
/// product name / unit.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PosSaleEntry {
    pub sale_id: i64,
    pub product_id: i64,
    pub product_legacy_no: String,
    pub product_name: String,
    pub product_unit: Option<String>,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub total_baht: f64,
    pub note: Option<String>,
    pub status: String,
    pub source: String,
    pub sold_at: chrono::DateTime<chrono::Utc>,
    pub sold_by: Option<String>,
    pub legacy_id: Option<i32>,
    pub aggregate_id: Uuid,
}

/// Response envelope for the per-folio sales list.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PosSalesListResponse {
    pub success: bool,
    pub data: Vec<PosSaleEntry>,
    pub total: i32,
    pub grand_total_baht: f64,
}

/// `POST /api/new/checkins/:id/pos-sale` — ring up one POS line.
///
/// Track G6 (MVP). Delegates the full validation + PG transaction +
/// outbox enqueue to [`crate::service::PosService::record_sale`].
pub async fn create_pos_sale(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<PosSaleRequest>,
) -> ApiResult<Json<PosSaleResponse>> {
    let command = build_pos_sale_command(cin_id, &body);
    let outcome = state
        .pos_service
        .record_sale(command)
        .await
        .map_err(map_pos_sale_error)?;
    Ok(Json(build_pos_sale_response(&outcome)))
}

/// `GET /api/new/checkins/:id/pos-sales` — list the folio's POS sales.
///
/// Read path — no service method; we stay on the repository pattern.
/// Ordered most-recent first because the cashier UI shows the
/// running tab in reverse-chronological order.
pub async fn list_pos_sales(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<PosSalesListResponse>> {
    if !state.checkins.exists(&state.new_pool, cin_id).await? {
        return Err(ApiError::NotFound(format!("check-in {cin_id} not found")));
    }
    let rows = sqlx::query(
        "SELECT s.sale_id, s.sale_product_id, s.sale_qty::float8 AS qty, \
                s.sale_unit_price::float8 AS unit_price, \
                s.sale_total::float8 AS total, s.sale_note, \
                s.sale_status, s.source, s.sale_sold_at, s.sale_sold_by, \
                s.sale_legacy_id, s.aggregate_id, \
                p.prod_legacy_no, p.prod_name, p.prod_unit \
           FROM ht_pos_sales s \
           JOIN ht_products  p ON p.prod_id = s.sale_product_id \
          WHERE s.sale_cin_id = $1 \
       ORDER BY s.sale_sold_at DESC, s.sale_id DESC",
    )
    .bind(cin_id)
    .fetch_all(&state.new_pool)
    .await?;

    let entries: Vec<PosSaleEntry> = rows
        .into_iter()
        .map(|row| {
            use sqlx::Row;
            PosSaleEntry {
                sale_id: row.try_get::<i64, _>("sale_id").unwrap_or(0),
                product_id: row.try_get::<i64, _>("sale_product_id").unwrap_or(0),
                product_legacy_no: row
                    .try_get::<String, _>("prod_legacy_no")
                    .unwrap_or_default(),
                product_name: row.try_get::<String, _>("prod_name").unwrap_or_default(),
                product_unit: row.try_get::<Option<String>, _>("prod_unit").unwrap_or(None),
                qty: row.try_get::<f64, _>("qty").unwrap_or(0.0),
                unit_price_baht: row.try_get::<f64, _>("unit_price").unwrap_or(0.0),
                total_baht: row.try_get::<f64, _>("total").unwrap_or(0.0),
                note: row.try_get::<Option<String>, _>("sale_note").unwrap_or(None),
                status: row.try_get::<String, _>("sale_status").unwrap_or_default(),
                source: row.try_get::<String, _>("source").unwrap_or_default(),
                sold_at: row
                    .try_get::<chrono::DateTime<chrono::Utc>, _>("sale_sold_at")
                    .unwrap_or_else(|_| chrono::Utc::now()),
                sold_by: row
                    .try_get::<Option<String>, _>("sale_sold_by")
                    .unwrap_or(None),
                legacy_id: row
                    .try_get::<Option<i32>, _>("sale_legacy_id")
                    .unwrap_or(None),
                aggregate_id: row
                    .try_get::<Uuid, _>("aggregate_id")
                    .unwrap_or_else(|_| Uuid::nil()),
            }
        })
        .collect();

    let grand_total_baht = entries
        .iter()
        .filter(|e| e.status == "posted")
        .map(|e| e.total_baht)
        .sum();
    let total = i32::try_from(entries.len()).unwrap_or(i32::MAX);
    Ok(Json(PosSalesListResponse {
        success: true,
        data: entries,
        total,
        grand_total_baht,
    }))
}

/// PURE — translate path id + body into a service command. Module-
/// scoped so unit tests can pin the contract without building
/// `AppState`.
pub(crate) fn build_pos_sale_command(
    cin_id: i32,
    body: &PosSaleRequest,
) -> crate::service::RecordSaleCommand {
    crate::service::RecordSaleCommand {
        check_in_id: cin_id,
        product_id: body.product_id,
        qty: body.qty,
        unit_price_override_baht: body.unit_price_override,
        note: body.note.clone(),
        // TODO: wire operator from auth middleware once Phase 4 PR3
        //       threads the session into request extensions. Empty
        //       string is the canonical "unknown" sentinel (mirrors
        //       `change_room` / walkin convention).
        actor: String::new(),
    }
}

/// PURE — translate service outcome to HTTP response body.
pub(crate) fn build_pos_sale_response(
    outcome: &crate::service::RecordSaleOutcome,
) -> PosSaleResponse {
    PosSaleResponse {
        success: true,
        message: "Sale recorded".to_string(),
        sale_id: outcome.sale_id,
        aggregate_id: outcome.aggregate_id,
        check_in_id: outcome.check_in_id,
        product_id: outcome.product_id,
        qty: outcome.qty,
        unit_price_baht: outcome.unit_price_baht,
        total_baht: outcome.total_baht,
        sold_at: outcome.sold_at,
    }
}

/// Translate service outcomes to HTTP. Validation + Conflict surface
/// as 400 with explicit wording so the cashier UI can show
/// receptionists what went wrong (`product not active`, `check-in
/// not active`, `qty must be positive`, etc.).
fn map_pos_sale_error(err: ServiceError) -> ApiError {
    match err {
        ServiceError::Conflict(msg) => ApiError::BadRequest(msg),
        ServiceError::Validation(msg) => ApiError::BadRequest(msg),
        other => other.into(),
    }
}

// =============================================================================
// Guest Registry Types and Endpoints
//
// Stays on the repository: no service method exists yet for guest registry
// CRUD. When a `GuestService` lands, these handlers delegate the same way.
// =============================================================================

/// Guest from HT_Guest_Registry table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guest {
    pub id: i32,
    pub cin_id: i32,
    pub cust_id: Option<i32>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub id_card: Option<String>,
    pub passport: Option<String>,
    pub nationality: Option<String>,
    pub is_primary: bool,
    pub created_at: Option<NaiveDateTime>,
}

impl Guest {
    fn from_row(row: GuestRow) -> Self {
        Self {
            id: row.guest_id,
            cin_id: row.guest_cin_id,
            cust_id: row.guest_cust_id,
            first_name: row.guest_firstname,
            last_name: row.guest_lastname,
            id_card: row.guest_idcard,
            passport: row.guest_passport,
            nationality: row.guest_nationality,
            is_primary: row.guest_is_primary.unwrap_or(false),
            created_at: row.guest_created_at,
        }
    }
}

/// Response for guests list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestsResponse {
    pub success: bool,
    pub data: Vec<Guest>,
    pub total: i32,
}

/// Response for single guest
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestResponse {
    pub success: bool,
    pub guest: Guest,
}

/// Request body for creating a guest
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGuestRequest {
    /// Customer ID if linking to existing customer (optional)
    pub cust_id: Option<i32>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub id_card: Option<String>,
    pub passport: Option<String>,
    pub nationality: Option<String>,
    pub is_primary: Option<bool>,
}

/// Response for guest mutation operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestMutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/checkins/:id/guests - List guests for check-in
pub async fn list_guests(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<GuestsResponse>> {
    if !state.checkins.exists(&state.new_pool, cin_id).await? {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    let rows = state.checkins.list_guests(&state.new_pool, cin_id).await?;
    let guests: Vec<Guest> = rows.into_iter().map(Guest::from_row).collect();
    let total = guests.len() as i32;

    Ok(Json(GuestsResponse {
        success: true,
        data: guests,
        total,
    }))
}

/// POST /api/new/checkins/:id/guests - Add guest to check-in
pub async fn create_guest(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<CreateGuestRequest>,
) -> ApiResult<Json<GuestMutationResponse>> {
    let first_name = body.first_name.trim();
    if first_name.is_empty() {
        return Err(ApiError::BadRequest("Guest first name is required".to_string()));
    }

    let status_snap = state
        .checkins
        .find_status(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = status_snap.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot add guests to a non-active check-in".to_string()));
    }

    let is_primary = body.is_primary.unwrap_or(false);

    let mut tx = state.new_pool.begin().await?;

    // If this guest is marked as primary, unset any existing primary guest
    if is_primary {
        state.checkins.unset_primary_guests(&mut tx, cin_id).await?;
    }

    let guest_id = state
        .checkins
        .insert_guest(
            &mut tx,
            GuestInsert {
                cin_id,
                cust_id: body.cust_id,
                first_name,
                last_name: body.last_name.as_deref(),
                id_card: body.id_card.as_deref(),
                passport: body.passport.as_deref(),
                nationality: body.nationality.as_deref(),
                is_primary,
            },
        )
        .await?;

    tx.commit().await?;

    Ok(Json(GuestMutationResponse {
        success: true,
        message: "Guest added successfully".to_string(),
        id: Some(guest_id),
    }))
}

/// Path parameters for guest operations
#[derive(Debug, Deserialize)]
pub struct GuestPath {
    pub id: i32,
    pub guest_id: i32,
}

/// DELETE /api/new/checkins/:id/guests/:guestId - Remove guest from check-in
pub async fn delete_guest(
    State(state): State<AppState>,
    Path(path): Path<GuestPath>,
) -> ApiResult<Json<GuestMutationResponse>> {
    let status_snap = state
        .checkins
        .find_status(&state.new_pool, path.id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = status_snap.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot remove guests from a non-active check-in".to_string()));
    }

    let exists = state
        .checkins
        .find_guest_in_checkin(&state.new_pool, path.id, path.guest_id)
        .await?;
    if exists.is_none() {
        return Err(ApiError::NotFound("Guest not found for this check-in".to_string()));
    }

    let mut tx = state.new_pool.begin().await?;
    state
        .checkins
        .delete_guest(&mut tx, path.id, path.guest_id)
        .await?;
    tx.commit().await?;

    Ok(Json(GuestMutationResponse {
        success: true,
        message: "Guest removed successfully".to_string(),
        id: Some(path.guest_id),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    /// Build a minimal `CheckInDetailRow` fixture for the pure builder/
    /// validator tests. Real `get()` rows carry more fields populated;
    /// this stub fills the columns the extend builder actually reads.
    fn detail_fixture(
        cin_id: i32,
        checkin_date: NaiveDate,
        expected_checkout: NaiveDate,
        rate_per_night: Option<f64>,
        total_amount: Option<f64>,
        customer_name: Option<&str>,
    ) -> CheckInDetailRow {
        let noon = NaiveTime::from_hms_opt(12, 0, 0).expect("hardcoded noon is valid");
        CheckInDetailRow {
            cin_id,
            cin_no: format!("CIN-20260513-{:04}", cin_id),
            cin_book_id: None,
            book_no: String::new(),
            cin_cust_id: 1,
            customer_name: customer_name.map(str::to_string),
            cin_room_id: 1,
            room_no: "201".to_string(),
            type_name: "Standard".to_string(),
            cin_checkin_time: NaiveDateTime::new(checkin_date, noon),
            cin_checkout_time: None,
            cin_expected_checkout: expected_checkout,
            cin_adults: Some(1),
            cin_children: Some(0),
            cin_status: Some("active".to_string()),
            cin_rate_per_night: rate_per_night,
            cin_total_amount: total_amount,
            cin_payment_status: None,
            cin_notes: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Track G1 / T4 HIGH-2: shortening the stay is a different flow.
    /// Reject `new_checkout_date <= existing_expected_checkout` with a 400
    /// so receptionists see an explicit error instead of a silent no-op.
    #[test]
    fn extend_route_rejects_earlier_date() {
        let existing = NaiveDate::from_ymd_opt(2026, 5, 15).expect("valid date");

        // Strictly earlier — rejected.
        let earlier = NaiveDate::from_ymd_opt(2026, 5, 14).expect("valid date");
        let err = validate_new_checkout_after_existing(earlier, existing)
            .expect_err("earlier date must reject");
        match err {
            ApiError::BadRequest(msg) => {
                assert!(msg.contains("must be strictly after"), "got: {msg}");
            }
            other => panic!("expected BadRequest, got {other:?}"),
        }

        // Equal — also rejected (no-op extend is not a valid request).
        let err = validate_new_checkout_after_existing(existing, existing)
            .expect_err("same-day extend must reject");
        assert!(matches!(err, ApiError::BadRequest(_)));

        // One day later — accepted.
        let later = NaiveDate::from_ymd_opt(2026, 5, 16).expect("valid date");
        validate_new_checkout_after_existing(later, existing)
            .expect("a strictly later date must be accepted");
    }

    /// Track G1 / T4 HIGH-2: happy path — the route's command builder must
    /// translate the detail row + body into an [`ExtendStayCommand`] the
    /// service can execute without further lookups. This pins the
    /// contract that the route, not the service, owns:
    /// - `check_in_id` propagates verbatim.
    /// - `new_end` carries the body's `new_checkout_date` (UTC midnight).
    /// - `stay_start` carries the existing `cin_checkin_time` at UTC midnight.
    /// - `guest_label` carries the canonical customer name.
    /// - The four total fields are computed as
    ///   `(rate × extended_nights, current_total, max(0, room_total − pay))`,
    ///   matching the checkout-route convention (audit H1).
    #[test]
    fn extend_route_calls_service() {
        use chrono::{TimeZone, Utc};

        let checkin = NaiveDate::from_ymd_opt(2026, 5, 13).expect("valid date");
        let existing_checkout = NaiveDate::from_ymd_opt(2026, 5, 15).expect("valid date");
        let new_checkout = NaiveDate::from_ymd_opt(2026, 5, 17).expect("valid date");

        let detail = detail_fixture(
            42,
            checkin,
            existing_checkout,
            Some(1200.0),
            Some(2400.0),
            Some("Somchai Jaidee"),
        );
        let body = ExtendStayRequest {
            new_checkout_date: new_checkout,
            reason: Some("Guest extended one more night".to_string()),
        };

        let command = build_extend_stay_command(&detail, &body);

        assert_eq!(command.check_in_id, 42, "check_in_id must propagate");

        let midnight = NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight is valid");
        let expected_new_end = Utc.from_utc_datetime(&new_checkout.and_time(midnight));
        let expected_stay_start = Utc.from_utc_datetime(&checkin.and_time(midnight));
        assert_eq!(command.new_end, expected_new_end);
        assert_eq!(command.stay_start, expected_stay_start);

        assert_eq!(command.guest_label, "Somchai Jaidee");

        // 4 nights × 1200 baht = 4800.00 baht = 480000 satang.
        assert_eq!(command.new_room_price_total.as_satang(), 480_000);
        // No product/extras plumbing — net equals room total.
        assert_eq!(command.new_net_total.as_satang(), 480_000);
        // Existing total amount = 2400 baht = 240000 satang.
        assert_eq!(command.new_pay_total.as_satang(), 240_000);
        // Balance = max(0, room − pay) = 2400 baht = 240000 satang.
        assert_eq!(command.new_balance_total.as_satang(), 240_000);
    }

    /// Defensive: an already-overpaid stay (pay > room) must not surface a
    /// negative balance to the legacy writeback. Mirrors the checkout
    /// route's `(net − pay).max(0)` clamp.
    #[test]
    fn extend_command_clamps_negative_balance_to_zero() {
        let checkin = NaiveDate::from_ymd_opt(2026, 5, 13).expect("valid date");
        let existing_checkout = NaiveDate::from_ymd_opt(2026, 5, 14).expect("valid date");
        let new_checkout = NaiveDate::from_ymd_opt(2026, 5, 15).expect("valid date");

        let detail = detail_fixture(
            7,
            checkin,
            existing_checkout,
            Some(1000.0),
            Some(5000.0), // Overpaid relative to 2 nights × 1000.
            Some("Test Guest"),
        );
        let body = ExtendStayRequest {
            new_checkout_date: new_checkout,
            reason: None,
        };

        let command = build_extend_stay_command(&detail, &body);
        assert_eq!(
            command.new_balance_total.as_satang(),
            0,
            "balance must clamp at zero for overpaid stays"
        );
    }

    /// Track G4 / T4 HIGH-3 — the change-room route's command builder is
    /// PURE: it propagates the path id + body into a `ChangeRoomCommand`
    /// with no DB or clock reads. This locks the wire-shape contract so
    /// any future refactor of the body type surfaces here as a compile
    /// or assertion failure.
    #[test]
    fn change_room_route_builds_service_command() {
        let body = ChangeRoomRequest {
            from_room_id: 11,
            to_room_id: 22,
            reason: Some("AC broken".into()),
        };
        let cmd = build_change_room_command(99, &body);
        assert_eq!(cmd.check_in_id, 99);
        assert_eq!(cmd.from_room_id, 11);
        assert_eq!(cmd.to_room_id, 22);
        assert_eq!(cmd.reason.as_deref(), Some("AC broken"));
        // Actor stays empty until G7 auth middleware lands.
        assert_eq!(cmd.actor, "");
    }

    /// Track G4 / T4 HIGH-3 — empty / missing reason round-trips as
    /// `None` so the service's optional-string handling stays intact.
    #[test]
    fn change_room_route_passes_through_missing_reason() {
        let body = ChangeRoomRequest {
            from_room_id: 1,
            to_room_id: 2,
            reason: None,
        };
        let cmd = build_change_room_command(7, &body);
        assert!(cmd.reason.is_none());
    }

    /// Track G4 / T4 HIGH-3 — response builder reflects the service
    /// outcome verbatim (no surprise transforms). Pins the JSON shape
    /// the frontend expects.
    #[test]
    fn change_room_route_builds_response_shape() {
        let body = ChangeRoomRequest {
            from_room_id: 303,
            to_room_id: 403,
            reason: Some("upgrade".into()),
        };
        let outcome = ChangeRoomOutcome {
            check_in_id: 42,
            aggregate_id: Uuid::nil(),
            rc_id: 1234,
        };
        let response = build_change_room_response(&body, &outcome);
        assert!(response.success);
        assert_eq!(response.check_in_id, 42);
        assert_eq!(response.rc_id, 1234);
        assert_eq!(response.from_room_id, 303);
        assert_eq!(response.to_room_id, 403);
        assert!(response.message.contains("Room change"));
    }

    /// Missing `cin_rate_per_night` must not panic — fall back to zero so
    /// the recipe still runs (matches the checkout-route fallback).
    #[test]
    fn extend_command_tolerates_missing_rate() {
        let checkin = NaiveDate::from_ymd_opt(2026, 5, 13).expect("valid date");
        let existing_checkout = NaiveDate::from_ymd_opt(2026, 5, 14).expect("valid date");
        let new_checkout = NaiveDate::from_ymd_opt(2026, 5, 16).expect("valid date");

        let detail = detail_fixture(
            7,
            checkin,
            existing_checkout,
            None,
            None,
            None,
        );
        let body = ExtendStayRequest {
            new_checkout_date: new_checkout,
            reason: None,
        };

        let command = build_extend_stay_command(&detail, &body);
        assert_eq!(command.new_room_price_total.as_satang(), 0);
        assert_eq!(command.guest_label, "", "no customer name → empty label");
    }
}
