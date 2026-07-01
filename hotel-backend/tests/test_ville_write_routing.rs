//! HF Ville write-routing e2e — task #42 pre-flip proof.
//!
//! Proves that, with both canonical pools wired, a `Branch::Hfville` write lands
//! ONLY in the `hotelville` pool (never `hotelnew`), the per-site write
//! chokepoint (`write_pool`) resolves the correct physical DB, and the Ville
//! round binds `shift_site_id = 'hfville'`. This is the canonical-PG half of the
//! routing hardened in commit 55f4ec8 — it does NOT exercise the legacy-MSSQL
//! mirror leg (that is the coordinated live test in
//! `docs/coexistence/ville-write-e2e-runbook.md`).
//!
//! ## Running
//! Needs TWO databases (same schema), pointed at by:
//!   - `DATABASE_URL`       → the HF Hotel pool (`hotelnew`)
//!   - `VILLE_DATABASE_URL` → the HF Ville pool (`hotelville`)
//! and `VILLE_DATABASE_URL` MUST be a dedicated test DB (the test deletes its own
//! `TEST_villeroute_*` rows from both). If `VILLE_DATABASE_URL` is unset the test
//! SKIPS (does not fail), so the single-DB CI harness is unaffected until a
//! second DB is provisioned.

use hotel_backend::outbox::event::EventSource;
use hotel_backend::routes::mode::{AppState, Branch};
use hotel_backend::service::{CreateCustomerCommand, IssueCouponCommand, OpenShiftCommand};
use sqlx::PgPool;
use uuid::Uuid;

/// Connect both pools, or `None` (→ skip) when `VILLE_DATABASE_URL` is unset.
async fn two_pools() -> Option<(PgPool, PgPool)> {
    let ville_url = std::env::var("VILLE_DATABASE_URL").ok()?;
    let new_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
    });
    let new_pool = PgPool::connect(&new_url).await.expect("connect hotelnew");
    let ville_pool = PgPool::connect(&ville_url)
        .await
        .expect("connect hotelville");
    Some((new_pool, ville_pool))
}

async fn current_db(pool: &PgPool) -> String {
    sqlx::query_scalar::<_, String>("SELECT current_database()")
        .fetch_one(pool)
        .await
        .expect("SELECT current_database()")
}

async fn count(pool: &PgPool, sql: &'static str, marker: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(sql)
        .bind(marker)
        .fetch_one(pool)
        .await
        .unwrap_or(-1)
}

/// Delete this test family's rows (`TEST_villeroute_*`) from a pool so reruns and
/// the per-site open-round mutual-exclusion check start clean.
async fn purge(pool: &PgPool) {
    for sql in [
        "DELETE FROM writeback_jobs WHERE payload::text LIKE '%TEST_villeroute_%'",
        "DELETE FROM ht_shifts WHERE shift_opened_by LIKE 'TEST_villeroute_%'",
        "DELETE FROM ht_coupons WHERE coupon_issued_by LIKE 'TEST_villeroute_%'",
        "DELETE FROM ht_customers WHERE cust_firstname LIKE 'TEST_villeroute_%'",
    ] {
        sqlx::query(sql).execute(pool).await.ok();
    }
}

#[tokio::test]
async fn ville_writes_route_to_hotelville_only() {
    let Some((new_pool, ville_pool)) = two_pools().await else {
        eprintln!(
            "SKIP ville_writes_route_to_hotelville_only: set VILLE_DATABASE_URL \
             (and DATABASE_URL) to two dedicated test DBs to run this."
        );
        return;
    };

    // open_shift refuses unless ROUND_WRITEBACK_ENABLED; the Hfhotel bundle must
    // bind the binary's SITE_ID. These are read by wire_services at AppState::new.
    std::env::set_var("ROUND_WRITEBACK_ENABLED", "true");
    std::env::set_var("SITE_ID", "hfhotel");

    purge(&new_pool).await;
    purge(&ville_pool).await;

    let marker = format!("TEST_villeroute_{}", Uuid::new_v4().simple());

    let state = AppState::new(new_pool.clone())
        .with_ville(ville_pool.clone())
        .with_hfville_writes(true);

    // ---- 1. the chokepoint resolves the correct PHYSICAL database ----
    let v_db = current_db(state.write_pool(Some(Branch::Hfville)).unwrap()).await;
    let h_db = current_db(state.write_pool(Some(Branch::Hfhotel)).unwrap()).await;
    let n_db = current_db(state.write_pool(None).unwrap()).await;
    assert_ne!(v_db, h_db, "Hfville and Hfhotel must resolve different DBs");
    assert_eq!(
        h_db, n_db,
        "unset ?branch must resolve the same DB as Hfhotel"
    );
    assert_eq!(
        v_db,
        current_db(&ville_pool).await,
        "Hfville must resolve the hotelville pool"
    );

    // ---- 2. site_id threading (the one non-mechanical change) ----
    let ws_v = state.resolve_write_services(Some(Branch::Hfville)).unwrap();
    let ws_h = state.resolve_write_services(Some(Branch::Hfhotel)).unwrap();
    assert_eq!(
        ws_v.shifts.site_id(),
        "hfville",
        "Ville bundle must bind site_id=hfville"
    );
    assert_eq!(
        ws_h.shifts.site_id(),
        "hfhotel",
        "HF Hotel bundle keeps the binary SITE_ID"
    );

    // outbox baseline on the ville pool (only this test touches hotelville_test)
    let ville_jobs_before: i64 = sqlx::query_scalar("SELECT count(*) FROM writeback_jobs")
        .fetch_one(&ville_pool)
        .await
        .unwrap();

    // ---- 3. a Ville round: canonical row + outbox land in hotelville only ----
    ws_v.shifts
        .open_shift(OpenShiftCommand {
            opened_by: marker.clone(),
            opening_float: 100.0,
            notes: Some(marker.clone()),
        })
        .await
        .expect("open Ville round");

    let v_shift = count(
        &ville_pool,
        "SELECT count(*) FROM ht_shifts WHERE shift_opened_by=$1 AND shift_site_id='hfville'",
        &marker,
    )
    .await;
    let n_shift = count(
        &new_pool,
        "SELECT count(*) FROM ht_shifts WHERE shift_opened_by=$1",
        &marker,
    )
    .await;
    assert_eq!(
        v_shift, 1,
        "Ville round must be in hotelville with shift_site_id=hfville"
    );
    assert_eq!(n_shift, 0, "Ville round must NOT leak into hotelnew");

    // ---- 4. a Ville customer create lands in hotelville only ----
    ws_v.customers
        .create(CreateCustomerCommand {
            first_name: marker.clone(),
            last_name: None,
            phone: None,
            email: None,
            id_card: None,
            address: None,
            customer_type: None,
            notes: Some(marker.clone()),
            enrichment: Default::default(),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .expect("create Ville customer");

    let v_cust = count(
        &ville_pool,
        "SELECT count(*) FROM ht_customers WHERE cust_firstname=$1",
        &marker,
    )
    .await;
    let n_cust = count(
        &new_pool,
        "SELECT count(*) FROM ht_customers WHERE cust_firstname=$1",
        &marker,
    )
    .await;
    assert_eq!(v_cust, 1, "Ville customer must be in hotelville");
    assert_eq!(n_cust, 0, "Ville customer must NOT leak into hotelnew");

    // ---- 5. a Ville coupon issue lands in hotelville only ----
    ws_v.coupons
        .issue_coupon(IssueCouponCommand {
            customer_id: None,
            value_baht: 0.0,
            expires_at: None,
            issued_by: marker.clone(),
            for_cin_no: Some(marker.clone()),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .expect("issue Ville coupon");

    let v_coupon = count(
        &ville_pool,
        "SELECT count(*) FROM ht_coupons WHERE coupon_issued_by=$1",
        &marker,
    )
    .await;
    let n_coupon = count(
        &new_pool,
        "SELECT count(*) FROM ht_coupons WHERE coupon_issued_by=$1",
        &marker,
    )
    .await;
    assert_eq!(v_coupon, 1, "Ville coupon must be in hotelville");
    assert_eq!(n_coupon, 0, "Ville coupon must NOT leak into hotelnew");

    // ---- 6. the writeback outbox rode the ville pool's tx (round + coupon) ----
    let ville_jobs_after: i64 = sqlx::query_scalar("SELECT count(*) FROM writeback_jobs")
        .fetch_one(&ville_pool)
        .await
        .unwrap();
    assert!(
        ville_jobs_after - ville_jobs_before >= 2,
        "Ville round + coupon must enqueue their writeback jobs into hotelville's outbox \
         (before={ville_jobs_before}, after={ville_jobs_after})"
    );

    purge(&new_pool).await;
    purge(&ville_pool).await;
}
