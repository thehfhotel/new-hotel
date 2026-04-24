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
}

impl PaymentMethod {
    /// Legacy column on `HT_CheckIn_Pay` that receives the amount for this method.
    pub fn legacy_column(self) -> &'static str {
        match self {
            PaymentMethod::Cash => "Cin_Pay_Cash",
            PaymentMethod::Credit => "Cin_Pay_Credit",
            // Bank transfers are recorded under the credit column historically;
            // the spike has not yet captured a dedicated transfer flow.
            PaymentMethod::Transfer => "Cin_Pay_Credit",
        }
    }
}
