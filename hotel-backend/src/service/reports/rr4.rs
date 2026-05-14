//! Track G8 — RR.4 / ตม.30 Thai immigration foreign-guest export.
//!
//! ## Why this exists
//!
//! Thai law (RR.4 / ตม.30) requires every hotel to file foreign-guest
//! check-ins with the district immigration office on a fixed schedule.
//! Until Track G8 the new app had no equivalent of legacy
//! `FrmReportRR4` (Crystal Reports), so receptionists fell back to
//! iHOTEL for monthly filing. That blocked standalone cutover (audit
//! 2026-05-13 T4 CRIT-2).
//!
//! ## Source
//!
//! Per `docs/legacy-app/REPORTS_INVENTORY.md` §3.9, the legacy report
//! reads from `HT_CheckIn_H`, `HT_CheckIn_Other_People`, `HT_Customers`,
//! and `TB_SETTINGS`. Track E1 (migration 034) mirrors
//! `HT_CheckIn_Other_People` into canonical `ht_guest_registry` and
//! Track E2 (migration 035) carries the full `HT_Customers` address
//! plus nationality columns onto `ht_customers`. We therefore assemble
//! RR.4 rows from a single canonical join:
//!
//! ```text
//!   ht_guest_registry  ↔  ht_checkins  ↔  ht_customers
//!                              ↕
//!                          ht_rooms_new
//! ```
//!
//! The `ht_guest_registry` join is a LEFT — the primary guest's data
//! lives on `ht_customers`, and `ht_guest_registry` carries companion
//! rows (non-primary). We UNION both shapes so every foreign individual
//! who stayed in the window gets a row, regardless of whether they
//! were the primary check-in or a companion.
//!
//! ## Foreign-guest filter
//!
//! Per `docs/legacy-app/COMPAT_CHEATSHEET.md` line 99, the legacy
//! boolean `Cin_foreign` is `'True'` / `'False'` (capital). The
//! canonical mirror does not carry a boolean foreign flag — we derive
//! it from `nationality`: any non-empty value other than `'TH'` or
//! `'Thailand'` (case-insensitive) qualifies as foreign. This matches
//! the legacy report's intent (filing covers every guest of any
//! non-Thai nationality).
//!
//! ## Window semantics
//!
//! Inclusive of both endpoints on the **check-in date** axis (i.e.
//! `cin_checkin_time::date BETWEEN from AND to`). This matches the
//! legacy `FrmReportRR4` filter which keyed on `Cin_Date_in BETWEEN`.
//! Guests whose check-in date is in the window are filed, regardless
//! of when they check out. (The form is "arrivals" not "stayed
//! during" — confirmed against the official immigration template.)
//!
//! ## Multi-room handling
//!
//! Per the task spec: "for multi-room foreign families this matters …
//! lean toward joining ht_checkin_rooms". As of this commit
//! `ht_checkin_rooms` does NOT exist (Track B1 only landed schema for
//! `ht_checkins.cin_room_id`, a single-room reference). One canonical
//! check-in → one room → one row per guest. When Track B2 adds the
//! junction, the SQL here will swap `ci.cin_room_id` for
//! `JOIN ht_checkin_rooms` and the row count rises naturally for
//! foreign families in 2+ rooms. The shape (one row per guest+room)
//! is already correct under either schema.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::service::error::{ServiceError, ServiceResult};

/// Export file format. The regulator expects xlsx (matches the legacy
/// Crystal Reports A4 layout); CSV is a developer convenience.
///
/// `Default` is xlsx per REPORTS_INVENTORY.md §3.9 (the legacy form
/// emits an A4 portrait .xlsx-equivalent).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Rr4ExportFormat {
    Csv,
    #[default]
    Xlsx,
}

impl Rr4ExportFormat {
    /// File-name extension (no leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Xlsx => "xlsx",
        }
    }

    /// MIME type for the HTTP response.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Csv => "text/csv; charset=utf-8",
            Self::Xlsx => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
        }
    }

    /// Persisted token used in `ht_rr4_exports.format`.
    pub fn as_db_token(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Xlsx => "xlsx",
        }
    }
}

/// Input for an export run.
///
/// `site` is operator-selected (`hfhotel` / `hfville`) and appears in
/// the filename + audit row. The canonical PG database is shared
/// across sites today (one row per stay regardless of property), so
/// `site` is currently a label, not a filter clause. When Track G3
/// adds per-site partitioning we'll thread it into the WHERE.
#[derive(Debug, Clone)]
pub struct Rr4ExportRequest {
    pub site: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub format: Rr4ExportFormat,
    pub exported_by: String,
}

impl Rr4ExportRequest {
    /// Validate the input shape. Empty site is rejected (file naming
    /// + audit trail both need a non-empty discriminator).
    pub fn validate(&self) -> ServiceResult<()> {
        if self.site.trim().is_empty() {
            return Err(ServiceError::validation("site must not be empty"));
        }
        if self.to < self.from {
            return Err(ServiceError::validation(
                "`to` date must be on or after `from`",
            ));
        }
        Ok(())
    }

    /// Compose the canonical filename: `RR4_{site}_{from}_{to}.{ext}`.
    pub fn filename(&self) -> String {
        format!(
            "RR4_{}_{}_{}.{}",
            sanitise_site(&self.site),
            self.from.format("%Y%m%d"),
            self.to.format("%Y%m%d"),
            self.format.extension(),
        )
    }
}

/// Strip path-unsafe characters from the site label before splicing
/// into the filename. The DB column stores the raw value; only the
/// HTTP-facing filename uses this.
fn sanitise_site(raw: &str) -> String {
    raw.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// One row in the RR.4 export.
///
/// Column order on this struct is **load-bearing**: serialisers
/// (CSV writer, xlsx column index) walk fields in declaration order
/// so the emitted file matches the legacy A4 layout verbatim. Do not
/// reorder without updating the test that pins `RR4_COLUMN_HEADERS`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rr4Row {
    /// `ลำดับ` — sequence number (1-based, populated at emit time).
    pub seq: i32,
    /// `ชื่อ-นามสกุล` — full name (preserved verbatim, Thai or roman).
    pub full_name: String,
    /// `สัญชาติ` — nationality.
    pub nationality: String,
    /// `เลขที่หนังสือเดินทาง` — passport number.
    pub passport_no: String,
    /// `วันที่เข้าพัก` — check-in date (YYYY-MM-DD).
    pub checkin_date: String,
    /// `วันที่ออก` — check-out date or planned check-out.
    pub checkout_date: String,
    /// `หมายเลขห้อง` — room number.
    pub room_no: String,
    /// `ที่อยู่ปัจจุบัน` — current address (joined from `ht_customers`).
    pub address: String,
}

/// Column headers in emit order. Pinned by a unit test so refactors
/// to `Rr4Row` field order can't silently break the regulator's
/// expected layout.
pub const RR4_COLUMN_HEADERS: &[&str] = &[
    "ลำดับ",
    "ชื่อ-นามสกุล",
    "สัญชาติ",
    "เลขที่หนังสือเดินทาง",
    "วันที่เข้าพัก",
    "วันที่ออก",
    "หมายเลขห้อง",
    "ที่อยู่ปัจจุบัน",
];

/// Output of an export run.
#[derive(Debug, Clone)]
pub struct Rr4ExportResult {
    /// Raw bytes of the emitted file (CSV or xlsx).
    pub bytes: Vec<u8>,
    /// Hex-encoded SHA-256 of `bytes`. Persisted to
    /// `ht_rr4_exports.file_hash` for audit non-repudiation.
    pub file_hash: String,
    /// Foreign-guest row count (`len(rows)`).
    pub row_count: i32,
    /// Suggested HTTP filename.
    pub filename: String,
    /// MIME type for the HTTP `Content-Type` header.
    pub mime_type: &'static str,
}

/// Compliance-report service. Pure read-path; owns its own pool
/// handle (cheap clone) because the route layer doesn't need to
/// thread an outbox handle through.
#[derive(Clone)]
pub struct Rr4Service {
    pool: PgPool,
}

impl Rr4Service {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Fetch every foreign-guest row in the window.
    ///
    /// Returns rows ordered by (checkin_date ASC, room_no ASC,
    /// full_name ASC) so successive exports of the same window
    /// produce byte-identical output (load-bearing for `file_hash`
    /// non-repudiation).
    pub async fn fetch_rows(&self, req: &Rr4ExportRequest) -> ServiceResult<Vec<Rr4Row>> {
        req.validate()?;

        // The query unions two shapes:
        //
        //   (A) PRIMARY guest — derives identity from `ht_customers`
        //       directly via `ht_checkins.cin_cust_id`. No
        //       `ht_guest_registry` row required (the primary is
        //       always implied by `cin_cust_id`).
        //
        //   (B) COMPANION guest — derives identity from a non-primary
        //       `ht_guest_registry` row. Nationality and passport
        //       come from `ht_guest_registry` (Track E1 mapper
        //       populates these from `HT_CheckIn_Other_People`);
        //       address falls back to the primary's address (the
        //       legacy form does the same — companions share the
        //       primary's room and therefore his/her on-file address).
        //
        // Foreign filter (same in both branches): nationality is
        // non-empty AND not Thai. Empty/NULL nationality is treated
        // as Thai (legacy default — `Cin_foreign='False'`) so the
        // export does NOT spam the regulator with locals.
        let sql = r#"
            WITH primary_rows AS (
                SELECT
                    cu.cust_firstname AS firstname,
                    COALESCE(cu.cust_lastname, '') AS lastname,
                    COALESCE(cu.cust_nationality, '') AS nationality,
                    COALESCE(cu.cust_passport, '') AS passport_no,
                    ci.cin_checkin_time::date AS checkin_date,
                    COALESCE(ci.cin_checkout_time::date, ci.cin_expected_checkout) AS checkout_date,
                    COALESCE(r.room_no, '') AS room_no,
                    COALESCE(cu.cust_address, '') AS address
                FROM ht_checkins ci
                JOIN ht_customers cu ON cu.cust_id = ci.cin_cust_id
                LEFT JOIN ht_rooms_new r ON r.room_id = ci.cin_room_id
                WHERE ci.cin_status <> 'cancelled'
                  AND ci.cin_checkin_time::date BETWEEN $1 AND $2
                  AND COALESCE(cu.cust_nationality, '') <> ''
                  AND LOWER(COALESCE(cu.cust_nationality, '')) NOT IN ('th', 'thai', 'thailand', 'ไทย')
            ),
            companion_rows AS (
                SELECT
                    COALESCE(g.guest_firstname, '') AS firstname,
                    COALESCE(g.guest_lastname, '') AS lastname,
                    COALESCE(g.guest_nationality, '') AS nationality,
                    COALESCE(g.guest_passport, '') AS passport_no,
                    ci.cin_checkin_time::date AS checkin_date,
                    COALESCE(ci.cin_checkout_time::date, ci.cin_expected_checkout) AS checkout_date,
                    COALESCE(r.room_no, '') AS room_no,
                    COALESCE(cu.cust_address, '') AS address
                FROM ht_guest_registry g
                JOIN ht_checkins ci ON ci.cin_id = g.guest_cin_id
                LEFT JOIN ht_customers cu ON cu.cust_id = ci.cin_cust_id
                LEFT JOIN ht_rooms_new r ON r.room_id = ci.cin_room_id
                WHERE COALESCE(g.guest_is_primary, false) = false
                  AND ci.cin_status <> 'cancelled'
                  AND ci.cin_checkin_time::date BETWEEN $1 AND $2
                  AND COALESCE(g.guest_nationality, '') <> ''
                  AND LOWER(COALESCE(g.guest_nationality, '')) NOT IN ('th', 'thai', 'thailand', 'ไทย')
            )
            SELECT firstname, lastname, nationality, passport_no,
                   checkin_date::text AS checkin_date,
                   checkout_date::text AS checkout_date,
                   room_no, address
              FROM primary_rows
            UNION ALL
            SELECT firstname, lastname, nationality, passport_no,
                   checkin_date::text AS checkin_date,
                   checkout_date::text AS checkout_date,
                   room_no, address
              FROM companion_rows
             ORDER BY checkin_date ASC, room_no ASC, firstname ASC, lastname ASC
        "#;

        use sqlx::Row;
        let rows = sqlx::query(sql)
            .bind(req.from)
            .bind(req.to)
            .fetch_all(&self.pool)
            .await?;

        let mut out: Vec<Rr4Row> = Vec::with_capacity(rows.len());
        for (idx, row) in rows.iter().enumerate() {
            let firstname: String = row.try_get("firstname").unwrap_or_default();
            let lastname: String = row.try_get("lastname").unwrap_or_default();
            let full_name = if lastname.trim().is_empty() {
                firstname.trim().to_string()
            } else {
                format!("{} {}", firstname.trim(), lastname.trim())
            };
            out.push(Rr4Row {
                seq: (idx as i32) + 1,
                full_name,
                nationality: row.try_get("nationality").unwrap_or_default(),
                passport_no: row.try_get("passport_no").unwrap_or_default(),
                checkin_date: row.try_get("checkin_date").unwrap_or_default(),
                checkout_date: row.try_get("checkout_date").unwrap_or_default(),
                room_no: row.try_get("room_no").unwrap_or_default(),
                address: row.try_get("address").unwrap_or_default(),
            });
        }
        Ok(out)
    }

    /// Run the full export pipeline: fetch rows, serialise to the
    /// requested format, hash the bytes, append an audit row.
    pub async fn export(&self, req: Rr4ExportRequest) -> ServiceResult<Rr4ExportResult> {
        req.validate()?;
        let rows = self.fetch_rows(&req).await?;
        let row_count: i32 = i32::try_from(rows.len()).map_err(|_| {
            ServiceError::internal("RR.4 row count exceeds i32::MAX (this should never happen)")
        })?;
        let bytes = match req.format {
            Rr4ExportFormat::Csv => render_csv(&rows)?,
            Rr4ExportFormat::Xlsx => render_xlsx(&rows)?,
        };
        let file_hash = sha256_hex(&bytes);

        // Append the audit row. We deliberately do NOT wrap the file
        // emission + audit insert in a transaction — if the audit
        // insert fails we still want the receptionist to receive the
        // bytes (the regulator's deadline is a hard wall). The audit
        // failure is logged for ops follow-up.
        let audit_result = sqlx::query(
            "INSERT INTO ht_rr4_exports \
                (site, range_from, range_to, format, row_count, exported_by, file_hash) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&req.site)
        .bind(req.from)
        .bind(req.to)
        .bind(req.format.as_db_token())
        .bind(row_count)
        .bind(&req.exported_by)
        .bind(&file_hash)
        .execute(&self.pool)
        .await;

        if let Err(err) = audit_result {
            tracing::error!(
                error = %err,
                site = %req.site,
                from = %req.from,
                to = %req.to,
                "RR.4 audit row insert failed — file bytes still returned to caller"
            );
        }

        Ok(Rr4ExportResult {
            filename: req.filename(),
            mime_type: req.format.mime_type(),
            bytes,
            file_hash,
            row_count,
        })
    }
}

/// Hex SHA-256 of an arbitrary byte slice. 64 lowercase chars.
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Serialise rows to CSV (UTF-8, RFC 4180 quoting via the `csv` crate).
///
/// The header row is `RR4_COLUMN_HEADERS` verbatim — keeping the same
/// header text whether the operator picks CSV or xlsx is a forward
/// compatibility hedge (audit comparisons across formats).
pub fn render_csv(rows: &[Rr4Row]) -> ServiceResult<Vec<u8>> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer
        .write_record(RR4_COLUMN_HEADERS)
        .map_err(|e| ServiceError::internal(format!("csv header write failed: {e}")))?;
    for row in rows {
        writer
            .write_record([
                row.seq.to_string().as_str(),
                row.full_name.as_str(),
                row.nationality.as_str(),
                row.passport_no.as_str(),
                row.checkin_date.as_str(),
                row.checkout_date.as_str(),
                row.room_no.as_str(),
                row.address.as_str(),
            ])
            .map_err(|e| ServiceError::internal(format!("csv row write failed: {e}")))?;
    }
    writer
        .flush()
        .map_err(|e| ServiceError::internal(format!("csv flush failed: {e}")))?;
    writer
        .into_inner()
        .map_err(|e| ServiceError::internal(format!("csv finalise failed: {e}")))
}

/// Serialise rows to a one-sheet xlsx workbook.
///
/// Sheet name: `"RR4"`. Header row in bold. Cell-by-cell writes —
/// no external Excel binding required.
pub fn render_xlsx(rows: &[Rr4Row]) -> ServiceResult<Vec<u8>> {
    use rust_xlsxwriter::{Format, Workbook};

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet
        .set_name("RR4")
        .map_err(|e| ServiceError::internal(format!("xlsx sheet name failed: {e}")))?;

    let header_format = Format::new().set_bold();

    for (col_idx, header) in RR4_COLUMN_HEADERS.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col_idx as u16, *header, &header_format)
            .map_err(|e| ServiceError::internal(format!("xlsx header write failed: {e}")))?;
    }

    for (row_idx, row) in rows.iter().enumerate() {
        let r = (row_idx + 1) as u32;
        worksheet
            .write_number(r, 0, row.seq as f64)
            .map_err(|e| ServiceError::internal(format!("xlsx seq write failed: {e}")))?;
        worksheet
            .write_string(r, 1, &row.full_name)
            .map_err(|e| ServiceError::internal(format!("xlsx name write failed: {e}")))?;
        worksheet
            .write_string(r, 2, &row.nationality)
            .map_err(|e| ServiceError::internal(format!("xlsx nationality write failed: {e}")))?;
        worksheet
            .write_string(r, 3, &row.passport_no)
            .map_err(|e| ServiceError::internal(format!("xlsx passport write failed: {e}")))?;
        worksheet
            .write_string(r, 4, &row.checkin_date)
            .map_err(|e| ServiceError::internal(format!("xlsx checkin write failed: {e}")))?;
        worksheet
            .write_string(r, 5, &row.checkout_date)
            .map_err(|e| ServiceError::internal(format!("xlsx checkout write failed: {e}")))?;
        worksheet
            .write_string(r, 6, &row.room_no)
            .map_err(|e| ServiceError::internal(format!("xlsx room write failed: {e}")))?;
        worksheet
            .write_string(r, 7, &row.address)
            .map_err(|e| ServiceError::internal(format!("xlsx address write failed: {e}")))?;
    }

    workbook
        .save_to_buffer()
        .map_err(|e| ServiceError::internal(format!("xlsx serialise failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rr4_column_headers_match_legacy_layout() {
        // Per docs/legacy-app/REPORTS_INVENTORY.md §3.9 the legacy
        // form is the "Thai government RR.4 guest-registry form
        // (sent to อำเภอ)". The official template has these eight
        // columns in this exact order. Any divergence would force
        // the receptionist to manually re-shuffle before filing.
        let expected: &[&str] = &[
            "ลำดับ",
            "ชื่อ-นามสกุล",
            "สัญชาติ",
            "เลขที่หนังสือเดินทาง",
            "วันที่เข้าพัก",
            "วันที่ออก",
            "หมายเลขห้อง",
            "ที่อยู่ปัจจุบัน",
        ];
        assert_eq!(RR4_COLUMN_HEADERS, expected);
    }

    #[test]
    fn filename_uses_iso_date_compact_format() {
        let req = Rr4ExportRequest {
            site: "hfhotel".into(),
            from: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 5, 13).unwrap(),
            format: Rr4ExportFormat::Xlsx,
            exported_by: "tester".into(),
        };
        assert_eq!(req.filename(), "RR4_hfhotel_20260501_20260513.xlsx");
    }

    #[test]
    fn filename_sanitises_unsafe_site_chars() {
        let req = Rr4ExportRequest {
            site: "hf hotel/ville".into(),
            from: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
            format: Rr4ExportFormat::Csv,
            exported_by: "tester".into(),
        };
        // space + slash both flatten to underscore
        assert_eq!(req.filename(), "RR4_hf_hotel_ville_20260101_20260131.csv");
    }

    #[test]
    fn validate_rejects_inverted_range() {
        let req = Rr4ExportRequest {
            site: "hfhotel".into(),
            from: NaiveDate::from_ymd_opt(2026, 5, 13).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            format: Rr4ExportFormat::Xlsx,
            exported_by: "tester".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn validate_rejects_empty_site() {
        let req = Rr4ExportRequest {
            site: "  ".into(),
            from: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 5, 13).unwrap(),
            format: Rr4ExportFormat::Xlsx,
            exported_by: "tester".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn csv_header_row_is_first_line() {
        let bytes = render_csv(&[]).expect("render_csv on empty rows must succeed");
        let text = std::str::from_utf8(&bytes).expect("csv must be utf-8");
        let first_line = text.lines().next().expect("csv must have header");
        // CSV crate writes UTF-8 verbatim — assert one of the Thai
        // headers makes it through.
        assert!(
            first_line.contains("ลำดับ"),
            "csv header line must carry Thai literal; got: {first_line}"
        );
    }

    #[test]
    fn csv_emits_one_row_per_input() {
        let rows = vec![
            Rr4Row {
                seq: 1,
                full_name: "John Smith".into(),
                nationality: "USA".into(),
                passport_no: "P12345".into(),
                checkin_date: "2026-05-01".into(),
                checkout_date: "2026-05-03".into(),
                room_no: "101".into(),
                address: "Bangkok".into(),
            },
            Rr4Row {
                seq: 2,
                full_name: "Yamada Taro".into(),
                nationality: "JP".into(),
                passport_no: "TZ9999".into(),
                checkin_date: "2026-05-02".into(),
                checkout_date: "2026-05-04".into(),
                room_no: "102".into(),
                address: "Tokyo".into(),
            },
        ];
        let bytes = render_csv(&rows).expect("render_csv must succeed");
        let text = std::str::from_utf8(&bytes).expect("csv must be utf-8");
        let line_count = text.lines().count();
        // 1 header + 2 data
        assert_eq!(line_count, 3, "expected header + 2 rows, got:\n{text}");
        assert!(text.contains("John Smith"));
        assert!(text.contains("Yamada Taro"));
    }

    #[test]
    fn csv_preserves_thai_address_verbatim() {
        // Per the user's standing constraint: legacy literals (Thai
        // addresses, Thai names) must round-trip unchanged.
        let rows = vec![Rr4Row {
            seq: 1,
            full_name: "นายสมชาย ใจดี".into(),
            nationality: "USA".into(),
            passport_no: "P12345".into(),
            checkin_date: "2026-05-01".into(),
            checkout_date: "2026-05-03".into(),
            room_no: "101".into(),
            address: "123 ถนนสุขุมวิท กรุงเทพฯ".into(),
        }];
        let bytes = render_csv(&rows).expect("render_csv must succeed");
        let text = std::str::from_utf8(&bytes).expect("csv must be utf-8");
        assert!(text.contains("นายสมชาย ใจดี"));
        assert!(text.contains("ถนนสุขุมวิท"));
    }

    #[test]
    fn xlsx_render_produces_valid_zip_signature() {
        // xlsx files are zip containers; the first 4 bytes are the
        // local file header signature `PK\x03\x04`. Cheapest possible
        // sanity check that rust_xlsxwriter actually produced an xlsx.
        let bytes = render_xlsx(&[]).expect("render_xlsx must succeed on empty");
        assert!(
            bytes.len() >= 4,
            "xlsx output must be at least 4 bytes; got {}",
            bytes.len()
        );
        assert_eq!(&bytes[..4], b"PK\x03\x04", "xlsx must start with ZIP signature");
    }

    #[test]
    fn sha256_hex_is_64_lowercase_chars() {
        let hash = sha256_hex(b"hello");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
        // Known SHA-256("hello"):
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn format_extension_matches_db_token() {
        assert_eq!(Rr4ExportFormat::Csv.extension(), "csv");
        assert_eq!(Rr4ExportFormat::Csv.as_db_token(), "csv");
        assert_eq!(Rr4ExportFormat::Xlsx.extension(), "xlsx");
        assert_eq!(Rr4ExportFormat::Xlsx.as_db_token(), "xlsx");
    }

    #[test]
    fn default_format_is_xlsx() {
        // Regulator delivery format per REPORTS_INVENTORY.md §3.9.
        assert_eq!(Rr4ExportFormat::default(), Rr4ExportFormat::Xlsx);
    }
}
