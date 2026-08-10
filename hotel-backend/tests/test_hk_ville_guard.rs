//! HF Ville admission gate on the maid surface — housekeeping-ops launch proof.
//!
//! ## What this pins
//!
//! Ville coequal writes are LIVE (`HFVILLE_WRITES_ENABLED=true` since
//! 2026-06-29), so in production this gate admits everything and the exemption
//! is inert. These tests pin the DISABLED posture — the one an operator can
//! return to — because that is where the exemption has teeth:
//!
//! - the maid's cleaning route must still be admitted, so a housekeeping
//!   report is not collateral damage of a front-desk write-policy toggle;
//! - nothing else may be, so an admitted mutation can never outrun a narrowed
//!   `HFVILLE_WRITEBACK_INTENTS`, which would leave canonical PG ahead of
//!   Ville's iHOTEL.
//!
//! ## How the layer is identified without a valid Access assertion
//!
//! `routes::hk::router` stacks the Ville guard OUTSIDE the Cloudflare Access
//! gate. With no `Cf-Access-Jwt-Assertion` header and `CF_ACCESS_HK_AUD`
//! unset, the two layers answer with distinguishable statuses:
//!
//! - **403** ⇒ the Ville guard refused the request (never reached auth).
//! - **401** ⇒ the guard ADMITTED it and the Access gate refused it.
//!
//! So 401 is the positive signal for "the exemption works", and no JWT
//! plumbing is needed. The router under test is the shipped one — `main.rs`
//! mounts the very same `routes::hk::router`, so this cannot drift from
//! production wiring.
//!
//! ## Running
//! `DATABASE_URL` → `hotelnew`. `VILLE_DATABASE_URL` → `hotelville` enables the
//! Ville-effect test; when unset that one SKIPS (does not fail), matching the
//! convention in `test_ville_write_routing.rs`.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use hotel_backend::outbox::event::EventSource;
use hotel_backend::routes::mode::AppState;
use hotel_backend::service::housekeeping::{HousekeepingService, MarkCleanCommand};
use sqlx::{PgPool, Row};
use tower::ServiceExt; // for `oneshot`

async fn new_pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
    });
    PgPool::connect(&url).await.ok()
}

/// Send an UNAUTHENTICATED request through the real hk router and return the
/// status. `hfville_writes` mirrors `HFVILLE_WRITES_ENABLED`.
async fn probe(pool: PgPool, method: &str, uri: &str, hfville_writes: bool) -> StatusCode {
    // No ville pool wired: the guard/auth layers must answer before any
    // handler could resolve a pool, which is itself part of the assertion.
    let state = AppState::new(pool).with_hfville_writes(hfville_writes);
    let app = hotel_backend::routes::hk::router(state);
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(r#"{"status":"done"}"#))
        .expect("request builds");
    app.oneshot(req).await.expect("router responds").status()
}

/// The admitted flow: with Ville writes DISABLED, the maid's cleaning report
/// for `branch=hfville` must pass the Ville guard. It then
/// stops at the Access gate (401), which is exactly how we know the guard let
/// it through rather than short-circuiting with 403.
#[tokio::test]
async fn hfville_cleaning_is_admitted_while_ville_writes_are_disabled() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping hfville_cleaning_is_admitted — PG not reachable");
        return;
    };
    let status = probe(
        pool,
        "POST",
        "/api/hk/rooms/1/cleaning?branch=hfville",
        false,
    )
    .await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "expected the Ville guard to ADMIT the maid cleaning route (leaving the \
         Access gate to answer 401); a 403 means the exemption regressed and \
         Ville maids cannot report cleaning at launch"
    );
}

/// The other half of the boundary: a NON-exempt hk mutation on
/// `branch=hfville` must still be refused by the guard (403), never reaching
/// auth. `broken-items` is the neighbouring route on the same router, so this
/// is the tightest possible check that the exemption did not widen.
#[tokio::test]
async fn other_hfville_mutations_are_still_refused() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping other_hfville_mutations_are_still_refused — PG not reachable");
        return;
    };
    let status = probe(
        pool,
        "POST",
        "/api/hk/rooms/1/broken-items?branch=hfville",
        false,
    )
    .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "a non-exempt Ville mutation must be refused by the Ville guard BEFORE \
         auth; 401 would mean the exemption leaked to the whole hk router"
    );
}

/// HF Hotel is unaffected by the gate, and reads are never gated.
#[tokio::test]
async fn hf_hotel_writes_and_all_reads_are_ungated() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping hf_hotel_writes_and_all_reads_are_ungated — PG not reachable");
        return;
    };
    for (method, uri) in [
        ("POST", "/api/hk/rooms/1/cleaning?branch=hfhotel"),
        ("POST", "/api/hk/rooms/1/cleaning"),
        ("POST", "/api/hk/rooms/1/broken-items?branch=hfhotel"),
        ("GET", "/api/hk/rooms?branch=hfville"),
    ] {
        let status = probe(pool.clone(), method, uri, false).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "{method} {uri} must reach the Access gate, not the Ville guard"
        );
    }
}

/// Once the coequal-writes program flips `HFVILLE_WRITES_ENABLED`, the gate
/// stops refusing anything — the exemption must not have changed that world.
#[tokio::test]
async fn enabling_ville_writes_ungates_everything() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping enabling_ville_writes_ungates_everything — PG not reachable");
        return;
    };
    let status = probe(
        pool,
        "POST",
        "/api/hk/rooms/1/broken-items?branch=hfville",
        true,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

/// The EFFECT half of the launch config: a maid's `done` on a Ville room must
/// flip the Ville canonical flag and enqueue `mark_room_clean` **in Ville's
/// own database** — the intent the Ville worker's allowlist admits. Repeating
/// it must stay a no-op.
///
/// Skips when `VILLE_DATABASE_URL` is unset (same convention as
/// `test_ville_write_routing.rs`).
#[tokio::test]
async fn ville_cleaning_flips_ville_pg_and_enqueues_only_mark_room_clean() {
    let Some(ville_url) = std::env::var("VILLE_DATABASE_URL").ok() else {
        eprintln!("skipping ville_cleaning_flips_ville_pg — VILLE_DATABASE_URL unset");
        return;
    };
    let Some(hf_pool) = new_pool().await else {
        eprintln!("skipping ville_cleaning_flips_ville_pg — PG not reachable");
        return;
    };
    let ville_pool = PgPool::connect(&ville_url).await.expect("connect hotelville");

    // Seed a DIRTY marker room in the VILLE database only.
    for pool in [&hf_pool, &ville_pool] {
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = 'ZT-VG1'")
            .execute(pool)
            .await;
    }
    let row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
         VALUES ('ZT-VG1', false, true) RETURNING room_id",
    )
    .fetch_one(&ville_pool)
    .await
    .expect("seed the Ville room");
    let room_id: i32 = row.try_get("room_id").expect("room_id");

    // Resolve the branch-bound service exactly as `routes::hk` does.
    let state = AppState::new(hf_pool.clone())
        .with_ville(ville_pool.clone())
        .with_hfville_writes(true);
    let pool_for_branch = state
        .write_pool(Some(hotel_backend::routes::mode::Branch::Hfville))
        .expect("ville pool resolves")
        .clone();
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT current_database()")
            .fetch_one(&pool_for_branch)
            .await
            .unwrap(),
        "hotelville",
        "branch=hfville must resolve to the hotelville database"
    );

    let svc = HousekeepingService::new(
        state.rooms.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool_for_branch,
    );
    let cmd = || MarkCleanCommand {
        room_id,
        by: "นก".into(),
        source: EventSource::System { reason: "test".into() },
    };

    assert!(
        svc.mark_clean_if_dirty(cmd()).await.unwrap().is_some(),
        "first done must perform the transition"
    );
    assert!(
        svc.mark_clean_if_dirty(cmd()).await.unwrap().is_none(),
        "a repeat done must not enqueue a second Ville writeback"
    );

    // Ville PG carries the flip.
    let clean: bool = sqlx::query_scalar("SELECT room_clean FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .fetch_one(&ville_pool)
        .await
        .unwrap();
    assert!(clean, "Ville canonical room_clean must be true");

    // Exactly one job, in VILLE's outbox, with the allowlisted intent.
    let agg: uuid::Uuid = sqlx::query_scalar(
        "SELECT aggregate_id FROM writeback_jobs WHERE intent = 'mark_room_clean' \
          ORDER BY id DESC LIMIT 1",
    )
    .fetch_one(&ville_pool)
    .await
    .expect("a mark_room_clean job must exist in the Ville outbox");
    let jobs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM writeback_jobs WHERE aggregate_id = $1 AND intent = 'mark_room_clean'",
    )
    .bind(agg)
    .fetch_one(&ville_pool)
    .await
    .unwrap();
    assert_eq!(jobs, 1, "exactly one Ville writeback across two done taps");

    // And nothing leaked into the HF Hotel outbox or room table.
    let leaked: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM writeback_jobs WHERE aggregate_id = $1",
    )
    .bind(agg)
    .fetch_one(&hf_pool)
    .await
    .unwrap();
    assert_eq!(leaked, 0, "the Ville writeback must NOT appear in hotelnew");

    // Cleanup.
    let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
        .bind(agg)
        .execute(&ville_pool)
        .await;
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(&ville_pool)
        .await;
}
