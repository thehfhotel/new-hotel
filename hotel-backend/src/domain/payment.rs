//! Payment aggregate.
//!
//! A payment is a single tender event against a check-in. Multiple payments per
//! check-in are supported (matches `ht_payments` table semantics added in v2.10
//! and the legacy `HT_CheckIn_Pay` insert per spike §3h).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::shared::Money;

/// A single payment event against a check-in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub check_in_id: Uuid,
    pub amount: Money,
    pub method: PaymentMethod,
    pub date: DateTime<Utc>,
    /// Legacy `HT_CheckIn_Pay.Pay_No` (month-scoped sequence).
    pub legacy_pay_no: Option<String>,
}

/// Tender method.
///
/// Maps to the `Cin_Pay_Cash` / `Cin_Pay_Credit` / etc. columns on
/// `HT_CheckIn_Pay` — the legacy app stores the amount in the column matching
/// the chosen tender (per spike §3h).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    Credit,
    Transfer,
    /// Online / QR / PromptPay-app payment. Wave 5c — routed to the
    /// `HT_CheckIn_Pay.Cin_Pay_web` column so iHOTEL reports surface the
    /// tender (the legacy "web" tender bucket the .NET app already
    /// supports for online payments). Canonical PG `ht_payments.pay_method`
    /// continues to carry the literal `"qr"` value so the wire contract
    /// + dashboards stay stable.
    Web,
}

impl PaymentMethod {
    /// Legacy column on `HT_CheckIn_Pay` that receives the amount for this method.
    ///
    /// Mirrors the recipe in `writeback::recipes::payment::build_statements`
    /// and `COMPAT_CHEATSHEET.md:552` — bank transfers (PromptPay / wire) land
    /// in `Cin_Pay_Tran`, not `Cin_Pay_Credit`. QR / online payments land in
    /// `Cin_Pay_web` per `audit-2026-05-13.md` Wave 5c.
    pub fn legacy_column(self) -> &'static str {
        match self {
            PaymentMethod::Cash => "Cin_Pay_Cash",
            PaymentMethod::Credit => "Cin_Pay_Credit",
            PaymentMethod::Transfer => "Cin_Pay_Tran",
            PaymentMethod::Web => "Cin_Pay_web",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cash_method_routes_to_cash_column() {
        assert_eq!(PaymentMethod::Cash.legacy_column(), "Cin_Pay_Cash");
    }

    #[test]
    fn credit_method_routes_to_credit_column() {
        assert_eq!(PaymentMethod::Credit.legacy_column(), "Cin_Pay_Credit");
    }

    #[test]
    fn transfer_method_routes_to_tran_column() {
        // Fix for audit H5: Transfer must route to Cin_Pay_Tran per
        // COMPAT_CHEATSHEET.md:552 — bank transfers are the third tender
        // column, distinct from credit cards.
        assert_eq!(PaymentMethod::Transfer.legacy_column(), "Cin_Pay_Tran");
    }

    #[test]
    fn web_method_routes_to_web_column() {
        // Wave 5c: QR / online tender routes to Cin_Pay_web so iHOTEL
        // legacy reports include it (was previously bypassing writeback
        // entirely via `insert_qr_payment_directly`).
        assert_eq!(PaymentMethod::Web.legacy_column(), "Cin_Pay_web");
    }
}
