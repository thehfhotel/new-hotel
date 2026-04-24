//! Customer aggregate.
//!
//! Pure domain types with no I/O. Mirrors the legacy `HT_Customers` table while
//! using a UUID as the canonical primary key (per architecture.md §4a).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A customer / guest profile.
///
/// `legacy_cust_no` is the `C\d+` identifier minted by the legacy app
/// (e.g. `"C21607"`). It is `None` for customers we created until the writeback
/// worker assigns one via the TABLOCKX MAX+1 allocation pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub legacy_cust_no: Option<String>,

    pub name: String,
    pub name2: Option<String>,

    pub customer_type: CustomerType,
    pub type_main: Option<String>,

    pub id_card: Option<String>,
    pub phone: Option<String>,

    pub address_line: Option<String>,
    pub address_district: Option<String>,
    pub address_province: Option<String>,
    pub address_postal: Option<String>,
    pub address_country: Option<String>,

    pub work_name: Option<String>,
    pub work_position: Option<String>,
    pub work_address: Option<String>,
    pub work_phone: Option<String>,

    pub last_change: Option<DateTime<Utc>>,
}

impl Customer {
    /// Validate a Thai national ID card number (13 digits, mod-11 checksum).
    ///
    /// Returns `true` for non-Thai (foreign passport, etc.) values that are not
    /// 13 digits — those are stored as-is. Only enforces the checksum when the
    /// input is exactly 13 ASCII digits.
    pub fn validate_id_card(id_card: &str) -> bool {
        let digits: Vec<u32> = id_card.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 13 {
            // Foreign passport / non-Thai identifier — accept as-is.
            return true;
        }
        if id_card.chars().any(|c| !c.is_ascii_digit()) {
            // Mixed alphanumerics — accept (e.g. passport string with embedded digits).
            return true;
        }
        let checksum: u32 = digits[..12]
            .iter()
            .enumerate()
            .map(|(index, digit)| digit * (13 - index as u32))
            .sum();
        let expected = (11 - (checksum % 11)) % 10;
        digits[12] == expected
    }

    /// Returns whether this instance's `id_card` (if any) passes
    /// [`Self::validate_id_card`].
    pub fn has_valid_id_card(&self) -> bool {
        self.id_card
            .as_deref()
            .map(Self::validate_id_card)
            .unwrap_or(true)
    }
}

/// Customer classification.
///
/// Mirrors the legacy `HT_Customers.Cust_Type` Thai-language values used by the
/// 3rd-party app. We carry both the canonical machine value and the original
/// Thai literal via [`CustomerType::legacy_literal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerType {
    Individual,
    Company,
    Government,
    Other,
}

impl CustomerType {
    /// Legacy app's Thai literal for this type (matches `HT_Customers.Cust_Type`).
    pub fn legacy_literal(self) -> &'static str {
        match self {
            // "บุคคลธรรมดา" (individual person)
            CustomerType::Individual => "บุคคลธรรมดา",
            // "บริษัท / ห้างร้าน" (company / business)
            CustomerType::Company => "บริษัท",
            CustomerType::Government => "หน่วยงานราชการ",
            CustomerType::Other => "อื่นๆ",
        }
    }
}
