//! Coupon HTTP routes — Track G5.
//!
//! - `POST /api/new/coupons` — issue a new coupon. Gated on
//!   `coupon.issue` (admin + receptionist per migration 051).
//! - `POST /api/new/coupons/{code}/redeem` — mark a coupon as
//!   redeemed / printed. Gated on `coupon.redeem` (admin + cashier +
//!   receptionist per migration 051).

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::domain::user::User;
use crate::error::{ApiError, ApiResult};
use crate::outbox::event::EventSource;
use crate::service::{IssueCouponCommand, RedeemCouponCommand};

/// Request body for `POST /api/new/coupons`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCouponRequest {
    /// Optional customer link. The frontend resolves customer search
    /// before submitting.
    pub customer_id: Option<i32>,
    /// Coupon face value in baht. Defaults to 0 for legacy-style
    /// food/breakfast entitlement coupons (no value attached).
    #[serde(default)]
    pub value: f64,
    pub expires_at: Option<NaiveDate>,
    /// Operator who issued the coupon. Routes will eventually source
    /// this from auth context; the body field is a transitional
    /// fallback used by the modal until the auth handoff lands.
    pub issued_by: Option<String>,
    /// Optional legacy `Cin_no` the coupon is bound to. The modal
    /// passes the current folio when issuing at check-in time.
    pub for_cin_no: Option<String>,
}

/// Request body for `POST /api/new/coupons/{code}/redeem`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemCouponRequest {
    /// Canonical check-in id the redemption is applied against.
    /// Optional — coupons may be redeemed standalone.
    pub cin_id: Option<i32>,
}

/// Mutation response carrying the new/updated row's id + aggregate id.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponMutationResponse {
    pub success: bool,
    pub message: String,
    pub id: i64,
    pub aggregate_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Branch selector — `branchFetch` appends `?branch=`; absent ⇒ HF Hotel.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchQuery {
    pub branch: Option<Branch>,
}

/// `POST /api/new/coupons` — issue a new coupon.
pub async fn issue_coupon(
    State(state): State<AppState>,
    // Task #40: the authenticated operator, when present. Auth ships dark, so
    // this is `Option<_>` (absent → `None`) and the body `issuedBy` remains the
    // fallback. Must precede the `Json` body extractor.
    actor: Option<Extension<User>>,
    // Branch selector: must precede the `Json` body extractor.
    Query(query): Query<BranchQuery>,
    Json(body): Json<IssueCouponRequest>,
) -> ApiResult<Json<CouponMutationResponse>> {
    if !body.value.is_finite() || body.value < 0.0 {
        return Err(ApiError::BadRequest(
            "value must be a finite non-negative number".to_string(),
        ));
    }

    let issued_by =
        super::resolve_actor(actor.as_deref(), body.issued_by.as_deref()).unwrap_or_default();
    // Branch-aware WRITE: bind the CouponService to the per-site pool so a Ville
    // coupon's canonical row + outbox enqueue land in hotelville and mirror to
    // Ville's legacy HT_Cupon. HF Hotel / `All` returns the pre-wired Arc.
    let ws = state.resolve_write_services(query.branch)?;
    let outcome = ws
        .coupons
        .issue_coupon(IssueCouponCommand {
            customer_id: body.customer_id,
            value_baht: body.value,
            expires_at: body.expires_at,
            issued_by,
            for_cin_no: body.for_cin_no.clone(),
            // TODO: thread user_id from auth middleware once the route
            // is mounted behind `require_auth`.
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await?;

    Ok(Json(CouponMutationResponse {
        success: true,
        message: format!(
            "Coupon {code} issued",
            code = outcome.coupon_code
        ),
        id: outcome.coupon_id,
        aggregate_id: outcome.aggregate_id,
        code: Some(outcome.coupon_code),
    }))
}

/// `POST /api/new/coupons/{code}/redeem` — mark a coupon redeemed.
pub async fn redeem_coupon(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Query(query): Query<BranchQuery>,
    Json(body): Json<RedeemCouponRequest>,
) -> ApiResult<Json<CouponMutationResponse>> {
    // Branch-aware WRITE: resolve the coupon FOR UPDATE lookup + redeem stamp
    // against the per-site pool so a Ville coupon isn't matched/mutated in the
    // HF Hotel DB (coupon codes can collide across the two logical DBs).
    let ws = state.resolve_write_services(query.branch)?;
    let outcome = ws
        .coupons
        .redeem_coupon(RedeemCouponCommand {
            coupon_code: code.clone(),
            redeemed_cin_id: body.cin_id,
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await?;

    Ok(Json(CouponMutationResponse {
        success: true,
        message: format!("Coupon {code} redeemed"),
        id: outcome.coupon_id,
        aggregate_id: outcome.aggregate_id,
        code: Some(code),
    }))
}

#[cfg(test)]
mod tests {
    //! Track G5 — input-validation tests for the coupon routes. These
    //! are pure deserialization + helper tests so they run without a
    //! database; the end-to-end behavior is covered by the
    //! `service::coupon` DB-gated tests (added separately).

    use super::*;

    /// The issue request must accept the documented camelCase shape
    /// (forCinNo, expiresAt). Catches a refactor that accidentally
    /// drops the renaming.
    #[test]
    fn issue_request_deserializes_camel_case() {
        let body = r#"{
            "customerId": 42,
            "value": 200.0,
            "expiresAt": "2026-12-31",
            "issuedBy": "receptionist_test",
            "forCinNo": "CH26-005228"
        }"#;
        let parsed: IssueCouponRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.customer_id, Some(42));
        assert_eq!(parsed.value, 200.0);
        assert_eq!(parsed.expires_at, Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()));
        assert_eq!(parsed.issued_by.as_deref(), Some("receptionist_test"));
        assert_eq!(parsed.for_cin_no.as_deref(), Some("CH26-005228"));
    }

    /// `value` defaults to 0 when omitted — matches the legacy `HT_Cupon`
    /// shape where coupons are entitlement-only with no monetary face
    /// value.
    #[test]
    fn issue_request_value_defaults_to_zero() {
        let body = r#"{}"#;
        let parsed: IssueCouponRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.value, 0.0);
        assert!(parsed.customer_id.is_none());
        assert!(parsed.expires_at.is_none());
    }

    /// The redeem request body's `cinId` round-trips through serde.
    #[test]
    fn redeem_request_deserializes_optional_cin_id() {
        let body = r#"{ "cinId": 17 }"#;
        let parsed: RedeemCouponRequest = serde_json::from_str(body).expect("must parse");
        assert_eq!(parsed.cin_id, Some(17));

        let body_empty = r#"{}"#;
        let parsed_empty: RedeemCouponRequest =
            serde_json::from_str(body_empty).expect("must parse");
        assert!(parsed_empty.cin_id.is_none());
    }
}
