//! Coupon HTTP routes — Track G5.
//!
//! - `POST /api/new/coupons` — issue a new coupon. Gated on
//!   `coupon.issue` (admin + receptionist per migration 051).
//! - `POST /api/new/coupons/{code}/redeem` — mark a coupon as
//!   redeemed / printed. Gated on `coupon.redeem` (admin + cashier +
//!   receptionist per migration 051).
//! - `GET /api/new/coupons` — list coupons with optional `status`
//!   query parameter (defaults to `issued` for the
//!   "outstanding coupons" dashboard panel). Unrestricted.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::outbox::event::EventSource;
use crate::service::{IssueCouponCommand, RedeemCouponCommand};

/// Coupon DTO surfaced on the wire. Field naming follows the
/// project's camelCase convention (`#[serde(rename_all = "camelCase")]`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponView {
    pub id: i64,
    pub code: String,
    pub status: String,
    pub value: f64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<NaiveDate>,
    pub issued_by: Option<String>,
    pub for_cin_no: Option<String>,
    pub redeemed_at: Option<DateTime<Utc>>,
    pub redeemed_cin_id: Option<i64>,
    pub legacy_cupon_no: Option<i32>,
}

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

/// Query parameters for `GET /api/new/coupons`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCouponsQuery {
    /// Filter by status. Defaults to `issued` (the "outstanding
    /// coupons" view used by the dashboard panel).
    pub status: Option<String>,
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

/// Wrapped response for `GET` queries.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponsResponse {
    pub success: bool,
    pub data: Vec<CouponView>,
}

/// `POST /api/new/coupons` — issue a new coupon.
pub async fn issue_coupon(
    State(state): State<AppState>,
    Json(body): Json<IssueCouponRequest>,
) -> ApiResult<Json<CouponMutationResponse>> {
    if !body.value.is_finite() || body.value < 0.0 {
        return Err(ApiError::BadRequest(
            "value must be a finite non-negative number".to_string(),
        ));
    }

    let outcome = state
        .coupons_service
        .issue_coupon(IssueCouponCommand {
            customer_id: body.customer_id,
            value_baht: body.value,
            expires_at: body.expires_at,
            issued_by: body.issued_by.clone().unwrap_or_default(),
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
    Json(body): Json<RedeemCouponRequest>,
) -> ApiResult<Json<CouponMutationResponse>> {
    let outcome = state
        .coupons_service
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

/// `GET /api/new/coupons?status=issued` — list coupons.
pub async fn list_coupons(
    State(state): State<AppState>,
    Query(params): Query<ListCouponsQuery>,
) -> ApiResult<Json<CouponsResponse>> {
    let status_filter = params.status.unwrap_or_else(|| "issued".to_string());
    let rows = sqlx::query(
        "SELECT \
            coupon_id, coupon_code, coupon_status, coupon_value::float8 AS coupon_value, \
            coupon_issued_at, coupon_expires_at, coupon_issued_by, coupon_for_cin_no, \
            coupon_redeemed_at, coupon_redeemed_cin_id, legacy_cupon_no \
         FROM ht_coupons \
         WHERE coupon_status = $1 \
         ORDER BY coupon_issued_at DESC \
         LIMIT 500",
    )
    .bind(&status_filter)
    .fetch_all(&state.new_pool)
    .await?;

    let data: Vec<CouponView> = rows
        .into_iter()
        .map(|row| CouponView {
            id: row.try_get("coupon_id").unwrap_or(0),
            code: row.try_get("coupon_code").unwrap_or_default(),
            status: row.try_get("coupon_status").unwrap_or_default(),
            value: row.try_get("coupon_value").unwrap_or(0.0),
            issued_at: row
                .try_get("coupon_issued_at")
                .unwrap_or_else(|_| Utc::now()),
            expires_at: row.try_get("coupon_expires_at").ok(),
            issued_by: row.try_get("coupon_issued_by").ok(),
            for_cin_no: row.try_get("coupon_for_cin_no").ok(),
            redeemed_at: row.try_get("coupon_redeemed_at").ok(),
            redeemed_cin_id: row.try_get("coupon_redeemed_cin_id").ok(),
            legacy_cupon_no: row.try_get("legacy_cupon_no").ok(),
        })
        .collect();

    Ok(Json(CouponsResponse {
        success: true,
        data,
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

    /// The list query's `status` defaults to `issued` at the handler
    /// level. The deserialized query treats a missing param as None;
    /// the handler applies the default.
    #[test]
    fn list_query_status_optional() {
        let no_status: ListCouponsQuery =
            serde_json::from_str("{}").expect("must parse empty");
        assert!(no_status.status.is_none());

        let redeemed: ListCouponsQuery =
            serde_json::from_str(r#"{"status":"redeemed"}"#).expect("must parse");
        assert_eq!(redeemed.status.as_deref(), Some("redeemed"));
    }
}
