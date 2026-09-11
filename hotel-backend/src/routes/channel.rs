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
//! ## `Idempotency-Key` on the hold create
//!
//! `POST /api/channel/bookings` is the one call here that is not naturally
//! replay-tolerant — payment-verified and release converge on a state, but a
//! create mints a new booking every time. A client retry after a hung request
//! therefore used to produce TWO holds (two real iHOTEL `จอง` rows, one of
//! which nobody releases before its 2h deadline).
//!
//! Callers may now send **`Idempotency-Key: <client-generated string>`**
//! (OPTIONAL — a request without it behaves exactly as it always has). The key
//! is a HEADER and deliberately NOT a body field: the request body is a locked
//! snake_case contract that the loyalty app and this file agree on field by
//! field, and idempotency is transport concern, not booking data. It is also
//! where every client library already looks for it, including the loyalty
//! app's own backend.
//!
//! * same key + same request → the FIRST response is replayed verbatim (same
//!   status, same body, no second hold), stamped `Idempotency-Replayed: true`;
//! * same key + a materially different request → **422**;
//! * two identical requests at once → serialised; exactly one hold is created
//!   and the loser replays the winner's response.
//!
//! Mechanism and TTL live in `service::channel_idempotency` (migration 093).
//!
//! ### The gap between the two writes (B8d / issue #305)
//!
//! The key row and the booking commit in DIFFERENT transactions — they have
//! to, because the reservation must stay open across the create. A process
//! that dies between them leaves the hold COMMITTED and the key GONE, so the
//! retry reserves fresh and, before this change, minted a second hold.
//!
//! The fix is to give the hold its own copy of the key:
//! `ht_bookings.book_ext_ref = hold_ext_ref(caller, key)` alongside
//! `book_channel = 'loyalty'`, which puts the dedupe on migration 076's
//! partial UNIQUE index — INSIDE the booking's transaction, the one place a
//! crash cannot separate from the booking. A retry then finds the survivor and
//! replays it (same 201, `Idempotency-Replayed: true`, the STORED total and
//! the ORIGINAL deadline), and the same path catches two retries racing each
//! other. Unkeyed requests stamp nothing and are unchanged.
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
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::error::ApiError;
use crate::outbox::event::EventSource;
use crate::service::{
    caller_identity, fingerprint_of, hold_ext_ref, normalize_key, ChannelIdempotency,
    ChannelService, CreateHoldCommand, PaymentPlan, Reserved, ServiceError, StoredResponse,
    ENDPOINT_CREATE_BOOKING, IDEMPOTENCY_KEY_HEADER, IDEMPOTENCY_REPLAYED_HEADER,
};

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

/// Render a pre-serialized JSON payload with an explicit status, optionally
/// marking it as an idempotent replay.
///
/// Built by hand rather than through `Json(...)` because a replay must return
/// the stored body BYTE FOR BYTE — re-parsing and re-serializing it would hand
/// the retrying client a different document than the original request got.
fn json_response(status: StatusCode, body: String, replayed: bool) -> Response {
    let mut builder = Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json");
    if replayed {
        builder = builder.header(IDEMPOTENCY_REPLAYED_HEADER, "true");
    }
    builder.body(Body::from(body)).unwrap_or_else(|err| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to build channel response: {err}"),
        )
    })
}

/// Replay a stored response, stamped so the client can tell it from a fresh
/// one. A stored status that is no longer a valid HTTP code cannot happen (we
/// wrote it) — fall back to the status the create path uses rather than 500 on
/// our own record.
fn replay_response(stored: StoredResponse) -> Response {
    let status = StatusCode::from_u16(stored.status).unwrap_or(StatusCode::CREATED);
    json_response(status, stored.body, true)
}

/// A key that was already spent on a different request. **422**, not 409: the
/// request is well-formed and the server is in no conflicting state — the
/// entity is unprocessable because it contradicts what this key already means.
fn mismatch_response(key: &str) -> Response {
    error_response(
        StatusCode::UNPROCESSABLE_ENTITY,
        format!(
            "Idempotency-Key '{key}' was already used for a different booking request; \
             retry the original request unchanged, or use a new key"
        ),
    )
}

/// The bearer this request presented, for `caller_identity`.
///
/// A local re-read of the `Authorization` header rather than plumbing it out of
/// `middleware::channel_token`: that module's job is to decide 401 vs 503 and
/// it deliberately exposes no token accessor. Behind it a valid bearer is
/// guaranteed, so this is a total function over an already-verified header.
fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let raw = headers.get(header::AUTHORIZATION)?.to_str().ok()?.trim();
    let (scheme, rest) = raw.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let token = rest.trim();
    (!token.is_empty()).then_some(token)
}

/// Read + validate the optional `Idempotency-Key` header.
///
/// `Ok(None)` = the caller opted out and gets exactly today's behaviour. A
/// header that IS present but unusable is a 400, not a silent opt-out —
/// dropping idempotency protection quietly is the failure this feature exists
/// to remove.
#[allow(clippy::result_large_err)] // see parse_property
fn idempotency_key(headers: &HeaderMap) -> Result<Option<String>, Response> {
    let Some(raw) = headers.get(IDEMPOTENCY_KEY_HEADER) else {
        return Ok(None);
    };
    let raw = raw.to_str().map_err(|_| {
        error_response(
            StatusCode::BAD_REQUEST,
            "Idempotency-Key must be printable ASCII without spaces (a UUID is ideal)".to_string(),
        )
    })?;
    normalize_key(raw).map(Some).map_err(service_error_response)
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

impl ChannelPayment {
    /// The contract literal, for the idempotency fingerprint. Deliberately not
    /// `Serialize`: this enum is request-only and the literals are already
    /// pinned by `payment_plan_wire_literals`.
    fn wire_value(self) -> &'static str {
        match self {
            ChannelPayment::Deposit50 => "deposit50",
            ChannelPayment::Full => "full",
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

/// Canonical fingerprint of a hold-create request, for the `Idempotency-Key`
/// mismatch check.
///
/// Hashes NORMALISED fields in a fixed order, not the raw bytes: a retry that
/// re-serialises its JSON with different key order or whitespace, or sends
/// `" 3 "` where the first attempt sent `"3"`, is the SAME request and must
/// replay rather than 422. Everything that changes what gets booked is in
/// here; nothing else is.
fn create_booking_fingerprint(
    property: &str,
    room_type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    body: &CreateChannelBookingRequest,
) -> String {
    fingerprint_of(&[
        property,
        &room_type_id.to_string(),
        &check_in.format("%Y-%m-%d").to_string(),
        &check_out.format("%Y-%m-%d").to_string(),
        &body.guests.to_string(),
        body.guest.name.trim(),
        body.guest.phone.trim(),
        body.membership_id.as_deref().unwrap_or("").trim(),
        body.payment.wire_value(),
    ])
}

/// Create the hold and render the 201 payload as a STRING — the exact bytes
/// that get sent and, on a keyed request, stored for replay.
///
/// `ext_ref` is the hold's own copy of the caller-idempotency key (B8d /
/// issue #305). Passing it makes migration 076's `(book_channel,
/// book_ext_ref)` UNIQUE index dedupe the create INSIDE the booking
/// transaction, which is the only place that survives a crash between the
/// booking's commit and `ht_channel_idempotency`'s. `None` for an unkeyed
/// request, which keeps that path byte-for-byte what it always was.
///
/// Returns the rendered body, the `book_id`, and whether the service answered
/// with an EXISTING hold — or a ready-to-send error `Response`.
#[allow(clippy::result_large_err)] // see parse_property
#[allow(clippy::too_many_arguments)] // one machine-surface request, unpacked
async fn perform_create_hold(
    state: &AppState,
    service: &ChannelService,
    branch: Branch,
    property: &'static str,
    room_type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    body: &CreateChannelBookingRequest,
    ext_ref: Option<String>,
) -> Result<(String, i32, bool), Response> {
    // Same daily allocator as the booking form (per-branch pool).
    let pool = state.write_pool(Some(branch)).map_err(api_error_response)?;
    let book_no = super::new_bookings::generate_book_no(state, pool)
        .await
        .map_err(api_error_response)?;

    let outcome = service
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
            ext_ref,
            source: channel_event_source(),
        })
        .await
        .map_err(service_error_response)?;

    let payload = CreateChannelBookingResponse {
        pms_booking_id: format_pms_booking_id(property, outcome.book_id),
        total: outcome.total_baht,
        amount_due_now: outcome.amount_due_baht,
        hold_expires_at: outcome.hold_expires_at.to_rfc3339(),
    };
    let body = serde_json::to_string(&payload).map_err(|err| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to render channel booking response: {err}"),
        )
    })?;

    Ok((body, outcome.book_id, outcome.replayed))
}

pub async fn create_booking(
    State(state): State<AppState>,
    headers: HeaderMap,
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
    // Validated (and the HF Ville write gate applied) BEFORE the key is
    // reserved: a request that could never have created a hold must not spend
    // one.
    let service = match channel_service_for(&state, branch, true) {
        Ok(s) => s,
        Err(resp) => return resp,
    };
    let key = match idempotency_key(&headers) {
        Ok(k) => k,
        Err(resp) => return resp,
    };

    // No key — exactly the pre-existing behaviour, no row written and no
    // `book_ext_ref` stamped, so every call mints a new hold.
    let Some(key) = key else {
        return match perform_create_hold(
            &state,
            &service,
            branch,
            property,
            room_type_id,
            check_in,
            check_out,
            &body,
            None,
        )
        .await
        {
            Ok((rendered, _, _)) => json_response(StatusCode::CREATED, rendered, false),
            Err(resp) => resp,
        };
    };

    // Keyed: the reservation lives in the same per-branch database as the
    // booking it protects, so the key and the hold commit or roll back together.
    let pool = match state.write_pool(Some(branch)) {
        Ok(p) => p.clone(),
        Err(e) => return api_error_response(e),
    };
    let idempotency = ChannelIdempotency::new(pool);
    let caller = caller_identity(bearer_token(&headers));
    let fingerprint =
        create_booking_fingerprint(property, room_type_id, check_in, check_out, &body);

    let reservation = match idempotency
        .reserve(&caller, &key, ENDPOINT_CREATE_BOOKING, &fingerprint)
        .await
    {
        Ok(Reserved::Fresh(reservation)) => reservation,
        Ok(Reserved::Replay(stored)) => {
            tracing::info!(
                idempotency_key = %key,
                book_id = ?stored.book_id,
                "loyalty channel hold create replayed from a stored response"
            );
            return replay_response(stored);
        }
        Ok(Reserved::Mismatch) => return mismatch_response(&key),
        Err(err) => return service_error_response(err),
    };

    match perform_create_hold(
        &state,
        &service,
        branch,
        property,
        room_type_id,
        check_in,
        check_out,
        &body,
        // B8d: the hold carries the key itself, so the (book_channel,
        // book_ext_ref) index dedupes even when this reservation never commits.
        Some(hold_ext_ref(&caller, &key)),
    )
    .await
    {
        Ok((rendered, book_id, replayed)) => {
            if let Err(err) = reservation
                .complete(StatusCode::CREATED.as_u16(), &rendered, Some(book_id))
                .await
            {
                // The hold IS committed (its own transaction); only the record
                // of the key failed. Report the created booking — refusing it
                // would tell the client nothing happened when a real iHOTEL
                // `จอง` exists. Since B8d a retry of this key no longer creates
                // a second hold: it re-enters as a fresh reservation, the
                // service recognises the booking by its `book_ext_ref`, and the
                // SAME hold comes back marked as a replay.
                tracing::error!(
                    error = %err,
                    idempotency_key = %key,
                    book_id,
                    "hold created but its idempotency key could not be recorded; a retry of this \
                     key replays the hold via its book_ext_ref"
                );
            }
            // `replayed` here means the BOOKING was the survivor of an earlier
            // attempt (crash between the two writes, or a lost concurrent
            // race) — the response is the original hold, so say so.
            json_response(StatusCode::CREATED, rendered, replayed)
        }
        Err(resp) => {
            // Errors are never cached: free the key so the client may retry it.
            reservation.abandon().await;
            resp
        }
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
        assert!(matches!(
            parse_property(" hf "),
            Ok((Branch::Hfhotel, "hf"))
        ));
        assert!(
            parse_property("hfhotel").is_err(),
            "internal site ids are NOT wire values"
        );
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

    fn sample_request() -> CreateChannelBookingRequest {
        serde_json::from_value(serde_json::json!({
            "property": "hf",
            "room_type_id": "3",
            "check_in": "2026-09-20",
            "check_out": "2026-09-22",
            "guests": 2,
            "guest": { "name": "Somchai Jaidee", "phone": "0812345678" },
            "payment": "deposit50"
        }))
        .expect("sample request parses")
    }

    fn headers_with(name: &'static str, value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(name, value.parse().expect("header value"));
        headers
    }

    #[test]
    fn idempotency_key_is_optional_and_validated() {
        // Absent → opt out, today's behaviour.
        assert!(idempotency_key(&HeaderMap::new()).unwrap().is_none());
        // Present → trimmed. Header name lookup is case-insensitive.
        assert_eq!(
            idempotency_key(&headers_with("Idempotency-Key", " abc-123 "))
                .unwrap()
                .as_deref(),
            Some("abc-123")
        );
        assert_eq!(
            idempotency_key(&headers_with("idempotency-key", "abc-123"))
                .unwrap()
                .as_deref(),
            Some("abc-123")
        );
        // Present but unusable is a 400, NOT a silent opt-out.
        assert!(idempotency_key(&headers_with("Idempotency-Key", "   ")).is_err());
        assert!(idempotency_key(&headers_with("Idempotency-Key", "has space")).is_err());
        assert!(idempotency_key(&headers_with("Idempotency-Key", &"a".repeat(256))).is_err());
    }

    #[test]
    fn bearer_token_is_read_case_insensitively() {
        for scheme in ["Bearer", "bearer", "BEARER"] {
            let headers = headers_with("authorization", &format!("{scheme} tok-123"));
            assert_eq!(bearer_token(&headers), Some("tok-123"), "scheme {scheme}");
        }
        assert_eq!(
            bearer_token(&headers_with("authorization", "Basic tok-123")),
            None
        );
        assert_eq!(
            bearer_token(&headers_with("authorization", "Bearer  ")),
            None
        );
        assert_eq!(bearer_token(&HeaderMap::new()), None);
    }

    #[test]
    fn fingerprint_ignores_formatting_but_not_content() {
        let base = sample_request();
        let baseline = create_booking_fingerprint("hf", 3, d("2026-09-20"), d("2026-09-22"), &base);

        // Same booking, sloppier client: padded name/phone, no membership vs
        // an empty one. Must REPLAY, not 422.
        let mut sloppy = sample_request();
        sloppy.guest.name = "  Somchai Jaidee  ".into();
        sloppy.guest.phone = " 0812345678 ".into();
        sloppy.membership_id = Some("   ".into());
        assert_eq!(
            baseline,
            create_booking_fingerprint("hf", 3, d("2026-09-20"), d("2026-09-22"), &sloppy)
        );

        // Anything that changes what gets booked must differ.
        let mut other_guests = sample_request();
        other_guests.guests = 3;
        assert_ne!(
            baseline,
            create_booking_fingerprint("hf", 3, d("2026-09-20"), d("2026-09-22"), &other_guests)
        );
        let mut other_payment = sample_request();
        other_payment.payment = ChannelPayment::Full;
        assert_ne!(
            baseline,
            create_booking_fingerprint("hf", 3, d("2026-09-20"), d("2026-09-22"), &other_payment)
        );
        let mut member = sample_request();
        member.membership_id = Some("HF-0001".into());
        assert_ne!(
            baseline,
            create_booking_fingerprint("hf", 3, d("2026-09-20"), d("2026-09-22"), &member)
        );
        // Property, room type and dates are the caller's normalised values.
        assert_ne!(
            baseline,
            create_booking_fingerprint("hfville", 3, d("2026-09-20"), d("2026-09-22"), &base)
        );
        assert_ne!(
            baseline,
            create_booking_fingerprint("hf", 4, d("2026-09-20"), d("2026-09-22"), &base)
        );
        assert_ne!(
            baseline,
            create_booking_fingerprint("hf", 3, d("2026-09-21"), d("2026-09-22"), &base)
        );
    }

    #[test]
    fn a_reused_key_with_a_different_body_is_422() {
        let resp = mismatch_response("abc-123");
        assert_eq!(
            resp.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "422 — well-formed request, but it contradicts what the key already means"
        );
    }

    #[test]
    fn only_a_replay_carries_the_replay_header() {
        let fresh = json_response(StatusCode::CREATED, "{\"a\":1}".to_string(), false);
        assert_eq!(fresh.status(), StatusCode::CREATED);
        assert_eq!(
            fresh.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
        assert!(fresh.headers().get(IDEMPOTENCY_REPLAYED_HEADER).is_none());

        let replay = replay_response(StoredResponse {
            status: 201,
            body: "{\"a\":1}".to_string(),
            book_id: Some(7),
        });
        assert_eq!(replay.status(), StatusCode::CREATED);
        assert_eq!(
            replay.headers().get(IDEMPOTENCY_REPLAYED_HEADER).unwrap(),
            "true"
        );
    }

    fn d(raw: &str) -> NaiveDate {
        NaiveDate::parse_from_str(raw, "%Y-%m-%d").expect("test date")
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
