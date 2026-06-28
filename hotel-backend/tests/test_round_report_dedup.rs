//! Round-report income dedup regression test (task #63).
//!
//! iHOTEL stores the `HT_CheckIn_Pay` tender split (cash/credit/transfer/…)
//! REPLICATED on every line of a multi-line receipt — one line per room for a
//! multi-room stay, plus a line per product. Summing all lines therefore
//! double/triple-counts the money a receipt actually took (HF Ville round 816:
//! raw-sum transfer 17,255 vs iHOTEL 11,005). The round report must collapse to
//! one representative line per `ledger_pay_no` before summing the tenders.
//!
//! This test seeds that exact shape into `ht_payment_ledger` and asserts the
//! REAL handler query [`ROUND_INCOME_BY_TENDER_SQL`] dedups correctly, while a
//! naive (pre-fix) sum over-counts. It binds an explicit `[from, to)` window in
//! the far future (2031) so it only ever sees its own fixture rows — the query
//! itself has no per-test marker filter (it sums everything in the window).

mod common;

use chrono::{DateTime, Utc};
use hotel_backend::routes::new_shifts::ROUND_INCOME_BY_TENDER_SQL;
use sqlx::Row;

/// Far-future window — no real or other-test data lives here.
const WINDOW_FROM: &str = "2031-06-27T00:00:00Z";
const WINDOW_TO: &str = "2031-06-28T00:00:00Z";
const PAY_DATE: &str = "2031-06-27T15:00:00Z";
/// Legacy-id base well above any real (~41k) or other-test id, to avoid the
/// `ledger_legacy_id` UNIQUE constraint colliding.
const LEGACY_BASE: i32 = 1_900_500_000;

fn ts(s: &str) -> DateTime<Utc> {
    s.parse().expect("parse rfc3339")
}

async fn cleanup(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM ht_payment_ledger WHERE ledger_legacy_id BETWEEN $1 AND $2")
        .bind(LEGACY_BASE)
        .bind(LEGACY_BASE + 99)
        .execute(pool)
        .await
        .ok();
}

/// Insert one ledger line. `cash`/`tran` are the REPLICATED receipt tender;
/// `amount` is the itemized per-line amount.
#[allow(clippy::too_many_arguments)]
async fn line(
    pool: &sqlx::PgPool,
    legacy_off: i32,
    pay_no: &str,
    ds_id: &str,
    cash: f64,
    tran: f64,
    amount: f64,
) {
    sqlx::query(
        "INSERT INTO ht_payment_ledger \
            (ledger_legacy_id, ledger_pay_no, ledger_ds_id, ledger_cash, ledger_credit, \
             ledger_tran, ledger_amount, ledger_status, ledger_pay_date, ledger_note) \
         VALUES ($1, $2, $3, $4::float8, 0, $5::float8, $6::float8, '1', $7, 'TEST_round_report_dedup')",
    )
    .bind(LEGACY_BASE + legacy_off)
    .bind(pay_no)
    .bind(ds_id)
    .bind(cash)
    .bind(tran)
    .bind(amount)
    .bind(ts(PAY_DATE))
    .execute(pool)
    .await
    .expect("INSERT ledger fixture line");
}

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

#[tokio::test]
async fn round_income_dedups_replicated_tenders_per_receipt() {
    let pool = common::create_test_pool().await;
    cleanup(&pool).await; // clear any leftovers from a prior failed run

    // Receipt 1 — single room, transfer 890.
    line(&pool, 1, "TESTRDD-1", "P001", 0.0, 890.0, 890.0).await;
    // Receipt 2 — TWO rooms; tender 2780 REPLICATED on both lines, amount itemized 1390 each.
    line(&pool, 2, "TESTRDD-2", "P001", 0.0, 2780.0, 1390.0).await;
    line(&pool, 3, "TESTRDD-2", "P001", 0.0, 2780.0, 1390.0).await;
    // Receipt 3 — room + product; tender 1690 REPLICATED, amount itemized (room 1390 / product 300).
    line(&pool, 4, "TESTRDD-3", "P001", 0.0, 1690.0, 1390.0).await;
    line(&pool, 5, "TESTRDD-3", "SEV-016", 0.0, 1690.0, 300.0).await;
    // Receipt 4 — single room, split tender cash 695 + transfer 695 on ONE line.
    line(&pool, 6, "TESTRDD-4", "P001", 695.0, 695.0, 1390.0).await;

    // --- the REAL handler query (deduped) ---
    let row = sqlx::query(ROUND_INCOME_BY_TENDER_SQL)
        .bind(ts(WINDOW_FROM))
        .bind(ts(WINDOW_TO))
        .fetch_one(&pool)
        .await
        .expect("run ROUND_INCOME_BY_TENDER_SQL");

    let cash_received: f64 = row.try_get("cash_received").unwrap();
    let cash_paid: f64 = row.try_get("cash_paid").unwrap();
    let credit: f64 = row.try_get("credit").unwrap();
    let transfer: f64 = row.try_get("transfer").unwrap();
    let free: f64 = row.try_get("free").unwrap();
    let web: f64 = row.try_get("web").unwrap();
    let line_count: i64 = row.try_get("line_count").unwrap();
    let payment_count: i64 = row.try_get("payment_count").unwrap();

    // Deduped: one representative line per pay_no (DISTINCT ON … ORDER BY ledger_id
    // takes the lowest-id line of each receipt).
    //   transfer = 890 + 2780 + 1690 + 695 = 6055   (NOT the raw 10525)
    //   cash     = 695 (receipt 4 only)
    assert!(approx(transfer, 6055.0), "transfer should dedup to 6055, got {transfer}");
    assert!(approx(cash_received, 695.0), "cash_received should be 695, got {cash_received}");
    assert!(approx(cash_paid, 0.0), "cash_paid {cash_paid}");
    assert!(approx(credit, 0.0) && approx(free, 0.0) && approx(web, 0.0));
    // line_count stays raw (6 lines); payment_count is the deduped receipt count (4).
    assert_eq!(line_count, 6, "line_count is the raw line count");
    assert_eq!(payment_count, 4, "payment_count is the deduped receipt count");

    // Reconciliation: deduped tender total == itemized SUM(ledger_amount).
    // 695 + 6055 = 6750  ==  890 + (1390+1390) + (1390+300) + 1390 = 6750.
    let amount_total: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(ledger_amount),0)::float8 FROM ht_payment_ledger \
         WHERE ledger_status='1' AND ledger_pay_date >= $1 AND ledger_pay_date < $2",
    )
    .bind(ts(WINDOW_FROM))
    .bind(ts(WINDOW_TO))
    .fetch_one(&pool)
    .await
    .unwrap();
    let tender_total = cash_received - cash_paid + credit + transfer + free + web;
    assert!(
        approx(tender_total, amount_total) && approx(amount_total, 6750.0),
        "deduped tenders ({tender_total}) must reconcile with itemized amount ({amount_total})",
    );

    // Guard: the PRE-FIX naive sum over-counts (proves the bug the fix prevents).
    let naive_transfer: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(ledger_tran) FILTER (WHERE ledger_tran > 0),0)::float8 \
           FROM ht_payment_ledger \
          WHERE ledger_status='1' AND ledger_pay_date >= $1 AND ledger_pay_date < $2",
    )
    .bind(ts(WINDOW_FROM))
    .bind(ts(WINDOW_TO))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        approx(naive_transfer, 10525.0),
        "naive (un-deduped) transfer should be the inflated 10525, got {naive_transfer}",
    );

    cleanup(&pool).await;
}
