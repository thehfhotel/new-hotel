//! HF Ville admission gate on the maid surface — housekeeping-ops launch proof.
//!
//! ## What this pins
//!
//! Ville coequal writes are LIVE (`HFVILLE_WRITES_ENABLED=true` since
//! 2026-06-29), so in production this gate admits everything and the exemption
//! is inert. These tests pin the DISABLED posture — the one an operator can
//! return to — because that is where the exemption has teeth:
//!
//! - the maid's report routes — `cleaning`, `linen-shortage` (migration 088),
//!   `linen-shortage/resolve` (migration 090) and, since migration 091, the
//!   whole Report HK set (`rooms/{id}/report`, `reports/{id}/verify|return`,
//!   `report-photos`) — must still be admitted, so a housekeeping report is not
//!   collateral damage of a front-desk write-policy toggle;
//! - nothing else may be, so an admitted mutation can never outrun a narrowed
//!   `HFVILLE_WRITEBACK_INTENTS`, which would leave canonical PG ahead of
//!   Ville's iHOTEL. (`linen-shortage` cannot outrun anything even in
//!   principle: it is PG-only and enqueues no intent at all.)
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

/// A pool that never connects. The guard and the Access gate both answer before
/// any handler could resolve a pool — which is itself part of the assertion —
/// so the exemption tests need no database at all.
fn lazy_pool() -> PgPool {
    PgPool::connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/never")
        .expect("a lazy pool needs no live server")
}

/// Send an UNAUTHENTICATED request through the real hk router and return the
/// status. `hfville_writes` mirrors `HFVILLE_WRITES_ENABLED`.
async fn probe(pool: PgPool, method: &str, uri: &str, hfville_writes: bool) -> StatusCode {
    probe_with_body(pool, method, uri, hfville_writes, r#"{"status":"done"}"#).await
}

/// [`probe`] with an explicit body — the linen route's payload is a different
/// shape, and sending a cleaning body to it would confound "the guard admitted
/// it" with "the body was rejected" if the layer order ever regressed.
async fn probe_with_body(
    pool: PgPool,
    method: &str,
    uri: &str,
    hfville_writes: bool,
    body: &str,
) -> StatusCode {
    // No ville pool wired: the guard/auth layers must answer before any
    // handler could resolve a pool, which is itself part of the assertion.
    let state = AppState::new(pool).with_hfville_writes(hfville_writes);
    let app = hotel_backend::routes::hk::router(state);
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request builds");
    app.oneshot(req).await.expect("router responds").status()
}

const LINEN_BODY: &str = r#"{"items":[{"kind":"bath_towel","qty":2}]}"#;

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

/// The linen-shortage report (migration 088) must be admitted for
/// `branch=hfville` exactly the way the cleaning report is — 401 from the
/// Access gate, never 403 from the Ville guard.
///
/// The safety argument is STRONGER here than for cleaning, and worth stating
/// because it is why this exemption is not a widening of risk: the linen route
/// is PG-only. It inserts `ht_hk_linen_reports` rows and enqueues NO writeback
/// intent, so there is no intent for `HFVILLE_WRITEBACK_INTENTS` to park as
/// `'skipped'` and therefore no way for canonical PG to run ahead of Ville's
/// iHOTEL — the exact failure mode the narrow-exemption rule exists to prevent.
///
/// Needs no database: both layers answer before a pool is touched.
#[tokio::test]
async fn hfville_linen_shortage_is_admitted_while_ville_writes_are_disabled() {
    let status = probe_with_body(
        lazy_pool(),
        "POST",
        "/api/hk/rooms/1/linen-shortage?branch=hfville",
        false,
        LINEN_BODY,
    )
    .await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "expected the Ville guard to ADMIT the maid linen-shortage route (leaving \
         the Access gate to answer 401); a 403 means Ville maids cannot report a \
         linen shortage whenever front-desk Ville writes are turned off"
    );
}

/// The linen RESOLVE route (migration 090, เติมผ้าแล้ว) must be admitted for
/// `branch=hfville` exactly the way the report it completes is.
///
/// It is the ONE six-segment write on this surface, so this is also the proof
/// that the guard's matcher actually grew that shape rather than falling
/// through: before migration 090 this path had five-plus-one segments and the
/// matcher rejected anything past the fifth, so it would answer 403 here.
///
/// The safety argument is `linen-shortage`'s, at one remove and slightly
/// stronger: the resolve is ONE `UPDATE` closing open rows, with no writeback
/// intent, no outbox row and — unlike ADR 0008's signals — not even a domain
/// event. There is nothing for a narrowed `HFVILLE_WRITEBACK_INTENTS` to park.
///
/// Needs no database: both layers answer before a pool is touched.
#[tokio::test]
async fn hfville_linen_resolve_is_admitted_while_ville_writes_are_disabled() {
    let status = probe_with_body(
        lazy_pool(),
        "POST",
        "/api/hk/rooms/1/linen-shortage/resolve?branch=hfville",
        false,
        "",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "expected the Ville guard to ADMIT the maid linen-resolve route (leaving \
         the Access gate to answer 401); a 403 means Ville maids cannot mark a \
         room restocked whenever front-desk Ville writes are turned off"
    );
}

/// The six-segment widening admits ONE path, not a SHAPE. `resolve` is exempt
/// as the completion of `linen-shortage` and under no other action, on no other
/// collection — and appending a sixth segment to an already-exempt five-segment
/// route must not inherit its exemption.
///
/// This is the half that matters: a matcher that simply stopped checking the
/// sixth segment would pass the admit test above and silently exempt every
/// `/api/hk/rooms/{id}/{anything}/{anything}`.
#[tokio::test]
async fn the_linen_resolve_exemption_does_not_widen_on_the_shipped_router() {
    for uri in [
        // `resolve` under an action that does not own it.
        "/api/hk/rooms/1/cleaning/resolve?branch=hfville",
        "/api/hk/rooms/1/signals/resolve?branch=hfville",
        "/api/hk/rooms/1/broken-items/resolve?branch=hfville",
        // …and on the other collection.
        "/api/hk/signals/1/ack/resolve?branch=hfville",
        "/api/hk/signals/1/linen-shortage/resolve?branch=hfville",
        // Near-misses on either half of the pair.
        "/api/hk/rooms/1/linen-shortage/resolveX?branch=hfville",
        "/api/hk/rooms/1/linen-shortageX/resolve?branch=hfville",
        // Trailing slash and deeper paths still fall through.
        "/api/hk/rooms/1/linen-shortage/resolve/?branch=hfville",
        "/api/hk/rooms/1/linen-shortage/resolve/extra?branch=hfville",
        // Malformed room id.
        "/api/hk/rooms/1a/linen-shortage/resolve?branch=hfville",
        "/api/hk/rooms//linen-shortage/resolve?branch=hfville",
        // The desk tree follows front-desk write policy, at every depth.
        "/api/housekeeping/rooms/1/linen-shortage/resolve?branch=hfville",
    ] {
        let status = probe_with_body(lazy_pool(), "POST", uri, false, "").await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "{uri} must be refused by the Ville guard; 401 would mean the \
             six-segment exemption is matching a shape rather than one path"
        );
    }

    // The exempt path with a mutating non-POST method.
    for method in ["PUT", "PATCH", "DELETE"] {
        let status = probe_with_body(
            lazy_pool(),
            method,
            "/api/hk/rooms/1/linen-shortage/resolve?branch=hfville",
            false,
            "",
        )
        .await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "{method} on the linen-resolve path must NOT be exempt"
        );
    }
}

/// The exemption is POST-only and exact-match, on the SHIPPED router. A
/// near-miss path or a non-POST method on the linen route must fall through to
/// the normal gate and be refused with 403 — the unit tests pin the pure
/// matcher, this pins that the shipped stack actually consults it.
#[tokio::test]
async fn linen_exemption_does_not_widen_on_the_shipped_router() {
    // A near-miss path: not the exempt leaf, so the guard refuses it.
    let status = probe_with_body(
        lazy_pool(),
        "POST",
        "/api/hk/rooms/1/linen-shortages?branch=hfville",
        false,
        LINEN_BODY,
    )
    .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "a near-miss of the linen path must be refused by the Ville guard; 401 \
         would mean the exemption is matching on a prefix"
    );

    // The exempt path with a mutating non-POST method.
    for method in ["PUT", "PATCH", "DELETE"] {
        let status = probe_with_body(
            lazy_pool(),
            method,
            "/api/hk/rooms/1/linen-shortage?branch=hfville",
            false,
            LINEN_BODY,
        )
        .await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "{method} on the linen path must NOT be exempt"
        );
    }
}

/// Report HK (migration 091) must be admitted for `branch=hfville` on all FOUR
/// of its write shapes — the maid's submission, reception's two verdicts, and
/// the photo intake.
///
/// The safety argument is the room signals', not the cleaning report's: the
/// three `ht_hk_room_report*` tables have no legacy counterpart AT ALL, so
/// there is no writeback recipe to write, no intent for a narrowed
/// `HFVILLE_WRITEBACK_INTENTS` to park as `'skipped'`, and no dark flag waiting
/// to enable one. Canonical PG cannot run ahead of Ville's iHOTEL because
/// nothing here has an iHOTEL side.
///
/// The two VERDICTS are the notable inclusion: they are reception-side writes,
/// and every other reception-side write on this estate follows front-desk write
/// policy. These do not, because they are served from the MAID surface's own
/// router by a receptionist standing in the room — a 403 there strands a maid's
/// submitted report unjudged for as long as the toggle is off.
///
/// Needs no database: both layers answer before a pool is touched.
#[tokio::test]
async fn hfville_report_hk_is_admitted_while_ville_writes_are_disabled() {
    for (uri, body) in [
        (
            "/api/hk/rooms/1/report?branch=hfville",
            r#"{"roomStatus":"vc","allItemsOk":true,"items":[],"photoIds":[1]}"#,
        ),
        ("/api/hk/reports/9/verify?branch=hfville", r#"{"photoIds":[1]}"#),
        (
            "/api/hk/reports/9/return?branch=hfville",
            r#"{"reason":"not_clean"}"#,
        ),
        ("/api/hk/report-photos?branch=hfville", ""),
    ] {
        let status = probe_with_body(lazy_pool(), "POST", uri, false, body).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "expected the Ville guard to ADMIT {uri} (leaving the Access gate to \
             answer 401); a 403 means Ville housekeeping cannot file or judge a \
             room report whenever front-desk Ville writes are turned off"
        );
    }
}

/// The Report HK widening admits FOUR paths, not four shapes. The three
/// collections' action lists are separate, the photo intake is a whole-path
/// equality (so its READ endpoint is NOT exempt), and no near-miss inherits an
/// exemption.
///
/// This is the half that matters: a matcher that shared one action list across
/// collections, or that used `starts_with` for the photo path, would pass the
/// admit test above while silently exempting a much larger surface.
#[tokio::test]
async fn the_report_hk_exemption_does_not_widen_on_the_shipped_router() {
    for uri in [
        // A verdict leaf under the wrong collection, and vice versa.
        "/api/hk/rooms/1/verify?branch=hfville",
        "/api/hk/rooms/1/return?branch=hfville",
        "/api/hk/reports/9/cleaning?branch=hfville",
        "/api/hk/reports/9/signals?branch=hfville",
        "/api/hk/reports/9/ack?branch=hfville",
        "/api/hk/signals/7/verify?branch=hfville",
        "/api/hk/signals/7/report?branch=hfville",
        // Near-misses on the submission path.
        "/api/hk/rooms/1/reportX?branch=hfville",
        "/api/hk/rooms/1/reports?branch=hfville",
        "/api/hk/rooms/1/report/extra?branch=hfville",
        "/api/hk/rooms/1a/report?branch=hfville",
        // Near-misses on the verdict paths.
        "/api/hk/reports/9/verifyX?branch=hfville",
        "/api/hk/reports/9/verify/extra?branch=hfville",
        "/api/hk/reports/9a/verify?branch=hfville",
        "/api/hk/reports/9?branch=hfville",
        // The photo intake is an EXACT path: its READ endpoint and every
        // near-miss must fall through to the normal gate.
        "/api/hk/report-photos/9?branch=hfville",
        "/api/hk/report-photosX?branch=hfville",
        "/api/hk/report-photos/9/delete?branch=hfville",
        // The desk tree follows front-desk write policy at every depth.
        "/api/housekeeping/reports/9/verify?branch=hfville",
        "/api/housekeeping/report-photos?branch=hfville",
    ] {
        let status = probe_with_body(lazy_pool(), "POST", uri, false, "{}").await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "{uri} must be refused by the Ville guard; 401 would mean the Report \
             HK exemption is matching a shape rather than four paths"
        );
    }

    // The exempt paths with a mutating non-POST method.
    for path in [
        "/api/hk/rooms/1/report?branch=hfville",
        "/api/hk/reports/9/verify?branch=hfville",
        "/api/hk/reports/9/return?branch=hfville",
        "/api/hk/report-photos?branch=hfville",
    ] {
        for method in ["PUT", "PATCH", "DELETE"] {
            let status = probe_with_body(lazy_pool(), method, path, false, "{}").await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{method} on {path} must NOT be exempt"
            );
        }
    }
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
        ("POST", "/api/hk/rooms/1/linen-shortage?branch=hfhotel"),
        ("POST", "/api/hk/rooms/1/linen-shortage"),
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

// ============================================================================
// Room signals (ADR 0008 / migration 089) — the two exempt shapes
// ============================================================================
//
// Same proof, same mechanics: 401 = the Ville guard ADMITTED the request and
// the Access gate refused it; 403 = the guard refused it. All DB-free — both
// layers answer before a pool is touched.
//
// The safety argument here is the strongest of the three exemptions.
// `linen-shortage` is PG-only because iHOTEL has no linen table TODAY; room
// signals are PG-only because iHOTEL has no concept of a room signal AT ALL, so
// there is not merely no intent enqueued — there is no writeback recipe that
// could ever be written, and nothing for a narrowed `HFVILLE_WRITEBACK_INTENTS`
// to park. The domain events the routes publish are `event_log` +
// `pg_notify` rows feeding this app's own SSE fan-out; the writeback worker
// dispatches `WritebackIntent`s, not events.

const SIGNAL_BODY: &str = r#"{"type":"item_missing"}"#;
const ANSWER_BODY: &str = r#"{"outcome":"clear"}"#;

/// The maid tree's signal writes must all be admitted while Ville writes are
/// disabled — the raise (under `/rooms/`) and each lifecycle action (under
/// `/signals/`). A 403 on any of them means a HF Ville maid cannot coordinate a
/// checkout whenever front-desk Ville writes are turned off, which is exactly
/// the collateral damage the exemption exists to prevent.
#[tokio::test]
async fn hfville_room_signals_are_admitted_while_ville_writes_are_disabled() {
    for (uri, body) in [
        ("/api/hk/rooms/1/signals?branch=hfville", SIGNAL_BODY),
        ("/api/hk/signals/1/ack?branch=hfville", ""),
        ("/api/hk/signals/1/done?branch=hfville", ""),
        ("/api/hk/signals/1/cancel?branch=hfville", ""),
        ("/api/hk/signals/1/answer?branch=hfville", ANSWER_BODY),
    ] {
        let status = probe_with_body(lazy_pool(), "POST", uri, false, body).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "expected the Ville guard to ADMIT {uri} (leaving the Access gate to \
             answer 401); a 403 means the room-signal exemption regressed"
        );
    }
}

/// The exemption is POST-only, exact-match, and per-COLLECTION — the shipped
/// router must consult the matcher, not a prefix.
///
/// The per-collection half matters most: `ack` is exempt under `/signals/` and
/// must NOT be under `/rooms/`, and `signals` is exempt under `/rooms/` and
/// must not be a magic leaf under `/signals/`. Cross-admitting them would turn
/// a closed list into a fuzzy one.
#[tokio::test]
async fn the_signal_exemption_does_not_widen_on_the_shipped_router() {
    for uri in [
        // Near-miss leaves.
        "/api/hk/signals/1/ackX?branch=hfville",
        "/api/hk/rooms/1/signalsX?branch=hfville",
        // Deeper paths and trailing slashes.
        "/api/hk/signals/1/ack/extra?branch=hfville",
        "/api/hk/signals/1/ack/?branch=hfville",
        // Non-numeric / empty ids.
        "/api/hk/signals/1a/ack?branch=hfville",
        "/api/hk/signals//ack?branch=hfville",
        // The two collections' action lists are NOT interchangeable.
        "/api/hk/rooms/1/ack?branch=hfville",
        "/api/hk/signals/1/cleaning?branch=hfville",
        // The DESK half of the same feature follows front-desk write policy.
        "/api/housekeeping/signals/1/ack?branch=hfville",
        "/api/housekeeping/rooms/1/signals?branch=hfville",
    ] {
        let status = probe_with_body(lazy_pool(), "POST", uri, false, SIGNAL_BODY).await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "{uri} must be refused by the Ville guard; 401 would mean the \
             room-signal exemption is matching too broadly"
        );
    }

    // The exempt paths with a mutating non-POST method.
    for uri in [
        "/api/hk/rooms/1/signals?branch=hfville",
        "/api/hk/signals/1/ack?branch=hfville",
    ] {
        for method in ["PUT", "PATCH", "DELETE"] {
            let status = probe_with_body(lazy_pool(), method, uri, false, SIGNAL_BODY).await;
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "{method} on {uri} must NOT be exempt"
            );
        }
    }
}

/// Reads are never blocked by this gate, so the maid's board and her live
/// stream keep working on `branch=hfville` regardless of the write flag — they
/// reach the Access gate (401) rather than the Ville guard (403).
#[tokio::test]
async fn hfville_signal_reads_are_never_touched_by_the_write_gate() {
    for uri in [
        "/api/hk/signals?branch=hfville",
        "/api/hk/events?branch=hfville",
    ] {
        let status = probe_with_body(lazy_pool(), "GET", uri, false, "").await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "GET {uri} must pass the write gate untouched"
        );
    }
}
