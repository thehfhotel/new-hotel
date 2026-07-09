//! Loyalty-app booking channel — `/api/channel/*` (docs/loyalty-channel.md).
//!
//! Machine-to-machine surface consumed by the loyalty app (a first-party
//! booking channel). Mounted in `main.rs` OUTSIDE `require_auth`, behind its
//! own shared-bearer gate (`middleware::channel_token` — ships DARK behind
//! `LOYALTY_CHANNEL_ENABLED` + `LOYALTY_CHANNEL_TOKEN`).
//!
//! ## Locked interface contract (agreed with the loyalty app)
//!
//! * `GET  /api/channel/availability?property=hf|hfville&check_in=YYYY-MM-DD&check_out=YYYY-MM-DD&guests=N`
//! * `POST /api/channel/bookings` → **201** `{pms_booking_id, total,
//!   amount_due_now, hold_expires_at}` — creates a TENTATIVE hold
//!   (`pending`, room assigned, expires in 2h).
//! * `POST /api/channel/bookings/{pms_booking_id}/payment-verified`
//!   (body `{"amount": <THB>}`) — hold → confirmed, deposit recorded.
//!   Replay-tolerant.
//! * `POST /api/channel/bookings/{pms_booking_id}/release` — cancels the
//!   hold. Replay-tolerant. (The scheduler sweep is the belt-and-braces.)
//!
//! Wire shapes are snake_case verbatim from the contract — this file
//! deliberately does NOT use `rename_all = "camelCase"`.
//!
//! ## Property ↔ branch mapping
//!
//! The contract identifies properties as `"hf"` (The Harbour Front Hotel)
//! and `"hfville"` (HF Ville); this repo's concept is `Branch`
//! (`Hfhotel`/`Hfville` → per-site PG pools via `AppState::write_pool`).
//! `pms_booking_id` is minted as `{property}-{book_id}` because the two
//! per-site databases have overlapping SERIAL sequences — the prefix routes
//! the follow-up calls back to the right pool.
//!
//! HF Ville mutations additionally require `HFVILLE_WRITES_ENABLED` (the
//! channel router sits outside the main router's `ville_write_guard`, which
//! keys on `?branch=` — so the same policy is enforced here explicitly).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::ApiError;
use crate::outbox::event::EventSource;
use crate::service::{ChannelService, CreateHoldCommand, PaymentPlan, ServiceError};

// ---------------------------------------------------------------------------
// Property ↔ branch mapping + pms_booking_id codec (pure, unit-tested)
// ---------------------------------------------------------------------------

/// Contract property literal → this repo's `Branch`.
///
/// These helpers return the ready-to-send error `Response` in the `Err` arm —
/// deliberate on this machine surface (exact status codes are part of the
/// contract), so the `clippy::result_large_err` size lint is waived.
#[allow(clippy::result_large_err)]
fn parse_property(property: &str) -> Result<(Branch, &'static str), Response> {
    match property.trim() {
        "hf" => Ok((Branch::Hfhotel, "hf")),
        "hfville" => Ok((Branch::Hfville, "hfville")),
        other => Err(error_response(
            StatusCode::BAD_REQUEST,
            format!("unknown property '{other}' (expected 'hf' or 'hfville')"),
        )),
    }
}

/// Mint the externally-stable booking id: `{property}-{book_id}`.
fn format_pms_booking_id(property: &str, book_id: i32) -> String {
    format!("{property}-{book_id}")
}

/// Parse `{property}-{book_id}` back into (branch, property, book_id).
#[allow(clippy::result_large_err)] // see parse_property
fn parse_pms_booking_id(id: &str) -> Result<(Branch, &'static str, i32), Response> {
    let bad = || {
        error_response(
            StatusCode::BAD_REQUEST,
            format!("malformed pms_booking_id '{id}' (expected e.g. 'hf-12345')"),
        )
    };
    let (prop, num) = id.trim().rsplit_once('-').ok_or_else(bad)?;
    let (branch, property) = parse_property(prop)?;
    let book_id: i32 = num.parse().map_err(|_| bad())?;
    if book_id <= 0 {
        return Err(bad());
    }
    Ok((branch, property, book_id))
}

#[allow(clippy::result_large_err)] // see parse_property
fn parse_date(raw: &str, field: &str) -> Result<NaiveDate, Response> {
    NaiveDate::parse_from_str(raw.trim(), "%Y-%m-%d").map_err(|_| {
        error_response(
            StatusCode::BAD_REQUEST,
            format!("invalid {field} '{raw}' (expected YYYY-MM-DD)"),
        )
    })
}

/// Build the per-branch [`ChannelService`] + enforce the HF Ville write gate
/// for mutating calls. Read paths pass `mutating=false` (availability serves
/// HF Ville data even while Ville writes stay dark).
#[allow(clippy::result_large_err)] // see parse_property
fn channel_service_for(
    state: &AppState,
    branch: Branch,
    mutating: bool,
) -> Result<ChannelService, Response> {
    if mutating && branch == Branch::Hfville && !state.hfville_writes_enabled {
        return Err(error_response(
            StatusCode::FORBIDDEN,
            "HF Ville writes are disabled (HFVILLE_WRITES_ENABLED=false); the hfville property \
             cannot accept channel bookings yet"
                .to_string(),
        ));
    }
    let ws = state
        .resolve_write_services(Some(branch))
        .map_err(api_error_response)?;
    let pool = state.write_pool(Some(branch)).map_err(api_error_response)?;
    Ok(ChannelService::new(
        pool.clone(),
        ws.bookings,
        ws.customers,
        state.customers.clone(),
    ))
}

// ---------------------------------------------------------------------------
// Error → wire mapping (Conflict must surface as 409 on this machine surface;
// the app-wide `From<ServiceError> for ApiError` flattens it to 400)
// ---------------------------------------------------------------------------

fn error_response(status: StatusCode, message: String) -> Response {
    (
        status,
        Json(serde_json::json!({ "success": false, "error": message })),
    )
        .into_response()
}

fn api_error_response(err: ApiError) -> Response {
    err.into_response()
}

fn service_error_response(err: ServiceError) -> Response {
    match err {
        ServiceError::Validation(msg) => error_response(StatusCode::BAD_REQUEST, msg),
        ServiceError::NotFound(msg) => error_response(StatusCode::NOT_FOUND, msg),
        ServiceError::Conflict(msg) => error_response(StatusCode::CONFLICT, msg),
        other => api_error_response(ApiError::from(other)),
    }
}

fn channel_event_source() -> EventSource {
    // Machine caller — no session user. Same shape the existing routes use
    // pre-auth (`our_app(nil, correlation)`).
    EventSource::our_app(Uuid::nil(), Uuid::new_v4())
}

// ---------------------------------------------------------------------------
// GET /api/channel/availability
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct AvailabilityQuery {
    pub property: String,
    pub check_in: String,
    pub check_out: String,
    /// Party size; defaults to 1 when omitted.
    pub guests: Option<i32>,
}

/// Wire shape per the locked contract (snake_case, `room_type_id` string).
#[derive(Debug, Serialize)]
pub struct AvailabilityRoomType {
    pub room_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub nightly_price: f64,
    pub available_count: i64,
}

#[derive(Debug, Serialize)]
pub struct AvailabilityResponse {
    pub property: String,
    pub check_in: String,
    pub check_out: String,
    pub room_types: Vec<AvailabilityRoomType>,
}

pub async fn availability(
    State(state): State<AppState>,
    Query(query): Query<AvailabilityQuery>,
) -> Response {
    let (branch, property) = match parse_property(&query.property) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let check_in = match parse_date(&query.check_in, "check_in") {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let check_out = match parse_date(&query.check_out, "check_out") {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let service = match channel_service_for(&state, branch, false) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    match service
        .availability(check_in, check_out, query.guests.unwrap_or(1))
        .await
    {
        Ok(rows) => Json(AvailabilityResponse {
            property: property.to_string(),
            check_in: check_in.format("%Y-%m-%d").to_string(),
            check_out: check_out.format("%Y-%m-%d").to_string(),
            room_types: rows
                .into_iter()
                .map(|r| AvailabilityRoomType {
                    room_type_id: r.type_id.to_string(),
                    name: r.name,
                    description: r.description.filter(|d| !d.trim().is_empty()),
                    nightly_price: r.nightly_price,
                    available_count: r.available_count,
                })
                .collect(),
        })
        .into_response(),
        Err(err) => service_error_response(err),
    }
}

// ---------------------------------------------------------------------------
// POST /api/channel/bookings
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ChannelGuest {
    pub name: String,
    pub phone: String,
}

/// `"deposit50"` | `"full"` per the locked contract.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ChannelPayment {
    #[serde(rename = "deposit50")]
    Deposit50,
    #[serde(rename = "full")]
    Full,
}

impl From<ChannelPayment> for PaymentPlan {
    fn from(p: ChannelPayment) -> Self {
        match p {
            ChannelPayment::Deposit50 => PaymentPlan::Deposit50,
            ChannelPayment::Full => PaymentPlan::Full,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelBookingRequest {
    pub property: String,
    pub room_type_id: String,
    pub check_in: String,
    pub check_out: String,
    pub guests: i32,
    pub guest: ChannelGuest,
    #[serde(default)]
    pub membership_id: Option<String>,
    pub payment: ChannelPayment,
}

#[derive(Debug, Serialize)]
pub struct CreateChannelBookingResponse {
    pub pms_booking_id: String,
    /// Total stay price, THB.
    pub total: f64,
    /// 50% (rounded to the satang, half-up) or 100% of `total`, THB.
    pub amount_due_now: f64,
    /// ISO-8601 UTC instant; the hold auto-releases past this.
    pub hold_expires_at: String,
}

pub async fn create_booking(
    State(state): State<AppState>,
    Json(body): Json<CreateChannelBookingRequest>,
) -> Response {
    let (branch, property) = match parse_property(&body.property) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let check_in = match parse_date(&body.check_in, "check_in") {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let check_out = match parse_date(&body.check_out, "check_out") {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let room_type_id: i32 = match body.room_type_id.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                format!(
                    "invalid room_type_id '{}' (use the id from /api/channel/availability)",
                    body.room_type_id
                ),
            )
        }
    };
    let service = match channel_service_for(&state, branch, true) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    // Same daily allocator as the booking form (per-branch pool).
    let pool = match state.write_pool(Some(branch)) {
        Ok(p) => p,
        Err(e) => return api_error_response(e),
    };
    let book_no = match super::new_bookings::generate_book_no(&state, pool).await {
        Ok(n) => n,
        Err(e) => return api_error_response(e),
    };

    match service
        .create_hold(CreateHoldCommand {
            book_no,
            room_type_id,
            check_in,
            check_out,
            guests: body.guests,
            guest_name: body.guest.name.clone(),
            guest_phone: body.guest.phone.clone(),
            membership_id: body.membership_id.clone(),
            payment: body.payment.into(),
            source: channel_event_source(),
        })
        .await
    {
        Ok(outcome) => (
            StatusCode::CREATED,
            Json(CreateChannelBookingResponse {
                pms_booking_id: format_pms_booking_id(property, outcome.book_id),
                total: outcome.total_baht,
                amount_due_now: outcome.amount_due_baht,
                hold_expires_at: outcome.hold_expires_at.to_rfc3339(),
            }),
        )
            .into_response(),
        Err(err) => service_error_response(err),
    }
}

// ---------------------------------------------------------------------------
// POST /api/channel/bookings/{pms_booking_id}/payment-verified
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct PaymentVerifiedRequest {
    /// Amount received, THB. (The contract left the body open; this is the
    /// one field the PMS needs — the loyalty app knows what it collected.)
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct PaymentVerifiedResponse {
    pub success: bool,
    pub pms_booking_id: String,
    pub status: &'static str,
    /// Deposit recorded on the booking, THB.
    pub deposit_recorded: f64,
    /// Remaining balance due at the property, THB.
    pub balance_due: f64,
    /// `true` on an idempotent replay (already confirmed earlier).
    pub already_confirmed: bool,
}

pub async fn payment_verified(
    State(state): State<AppState>,
    Path(pms_booking_id): Path<String>,
    body: Option<Json<PaymentVerifiedRequest>>,
) -> Response {
    let (branch, property, book_id) = match parse_pms_booking_id(&pms_booking_id) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let Some(Json(body)) = body else {
        return error_response(
            StatusCode::BAD_REQUEST,
            "missing JSON body: {\"amount\": <THB received>}".to_string(),
        );
    };
    let service = match channel_service_for(&state, branch, true) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    match service.confirm_payment(book_id, body.amount).await {
        Ok(outcome) => Json(PaymentVerifiedResponse {
            success: true,
            pms_booking_id: format_pms_booking_id(property, book_id),
            status: "confirmed",
            deposit_recorded: outcome.deposit_baht,
            balance_due: outcome.balance_due_baht,
            already_confirmed: outcome.already_confirmed,
        })
        .into_response(),
        Err(err) => service_error_response(err),
    }
}

// ---------------------------------------------------------------------------
// POST /api/channel/bookings/{pms_booking_id}/release
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct ReleaseResponse {
    pub success: bool,
    pub pms_booking_id: String,
    pub status: &'static str,
    /// `true` on an idempotent replay (already cancelled earlier).
    pub already_released: bool,
}

pub async fn release(
    State(state): State<AppState>,
    Path(pms_booking_id): Path<String>,
) -> Response {
    let (branch, property, book_id) = match parse_pms_booking_id(&pms_booking_id) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    let service = match channel_service_for(&state, branch, true) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    match service
        .release(book_id, "loyalty payment window lapsed (channel release)")
        .await
    {
        Ok(outcome) => Json(ReleaseResponse {
            success: true,
            pms_booking_id: format_pms_booking_id(property, book_id),
            status: "cancelled",
            already_released: outcome.already_released,
        })
        .into_response(),
        Err(err) => service_error_response(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_mapping_matches_contract() {
        assert!(matches!(parse_property("hf"), Ok((Branch::Hfhotel, "hf"))));
        assert!(matches!(
            parse_property("hfville"),
            Ok((Branch::Hfville, "hfville"))
        ));
        assert!(matches!(parse_property(" hf "), Ok((Branch::Hfhotel, "hf"))));
        assert!(parse_property("hfhotel").is_err(), "internal site ids are NOT wire values");
        assert!(parse_property("").is_err());
    }

    #[test]
    fn pms_booking_id_round_trips() {
        let id = format_pms_booking_id("hf", 12345);
        assert_eq!(id, "hf-12345");
        let (branch, property, book_id) = parse_pms_booking_id(&id).unwrap();
        assert_eq!(branch, Branch::Hfhotel);
        assert_eq!(property, "hf");
        assert_eq!(book_id, 12345);

        let (branch, property, book_id) = parse_pms_booking_id("hfville-7").unwrap();
        assert_eq!(branch, Branch::Hfville);
        assert_eq!(property, "hfville");
        assert_eq!(book_id, 7);
    }

    #[test]
    fn pms_booking_id_rejects_garbage() {
        for bad in ["12345", "hf-", "hf-abc", "mars-1", "hf--3", "hf-0", "hf--1"] {
            assert!(parse_pms_booking_id(bad).is_err(), "{bad} must be rejected");
        }
    }

    #[test]
    fn payment_plan_wire_literals() {
        let d: ChannelPayment = serde_json::from_str("\"deposit50\"").unwrap();
        assert!(matches!(PaymentPlan::from(d), PaymentPlan::Deposit50));
        let f: ChannelPayment = serde_json::from_str("\"full\"").unwrap();
        assert!(matches!(PaymentPlan::from(f), PaymentPlan::Full));
        assert!(serde_json::from_str::<ChannelPayment>("\"half\"").is_err());
    }

    #[test]
    fn availability_response_uses_contract_keys() {
        let resp = AvailabilityResponse {
            property: "hf".into(),
            check_in: "2026-08-01".into(),
            check_out: "2026-08-03".into(),
            room_types: vec![AvailabilityRoomType {
                room_type_id: "3".into(),
                name: "Deluxe".into(),
                description: None,
                nightly_price: 1200.0,
                available_count: 4,
            }],
        };
        let v = serde_json::to_value(&resp).unwrap();
        // Locked contract: snake_case keys, room_type_id as a string.
        assert_eq!(v["room_types"][0]["room_type_id"], "3");
        assert_eq!(v["room_types"][0]["nightly_price"], 1200.0);
        assert_eq!(v["room_types"][0]["available_count"], 4);
        assert!(v["room_types"][0].get("roomTypeId").is_none());
    }

    #[test]
    fn create_response_uses_contract_keys() {
        let resp = CreateChannelBookingResponse {
            pms_booking_id: "hf-1".into(),
            total: 2400.0,
            amount_due_now: 1200.0,
            hold_expires_at: "2026-08-01T10:00:00+00:00".into(),
        };
        let v = serde_json::to_value(&resp).unwrap();
        assert_eq!(v["pms_booking_id"], "hf-1");
        assert_eq!(v["total"], 2400.0);
        assert_eq!(v["amount_due_now"], 1200.0);
        assert!(v["hold_expires_at"].is_string());
    }
}
