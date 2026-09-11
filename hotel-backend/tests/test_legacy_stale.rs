//! ADR 0006 integration — the `legacy_stale` transport, end to end through
//! real PostgreSQL.
//!
//! The unit tests cover the two ends in isolation (label building + coalescing
//! in `outbox::legacy_stale`; classification + SSE framing in `routes::events`).
//! What only a live PG can prove is the middle: that a `pg_notify` on the
//! `legacy_stale` channel is actually picked up by the SAME standalone listener
//! that serves `domain_events`, and lands on the site's broadcast fan-out —
//! i.e. that the second `listen()` call is really wired.
//!
//! ## Running
//!
//! Needs a PG reachable via `DATABASE_URL`. Skipped (not failed) when that is
//! unset, matching `tests/test_permissions_g7.rs` — CI supplies it.

use std::time::Duration;

use hotel_backend::outbox::legacy_stale::{
    publish, LegacyStaleSignal, LEGACY_STALE_CHANNEL, LEGACY_STALE_EVENT,
};
use hotel_backend::routes::events::{
    spawn_domain_event_listener, BroadcastEvent, EventFanout, EventSite, RESYNC_EVENT,
};
use hotel_backend::routes::mode::Branch;
use sqlx::PgPool;
use tokio::sync::broadcast;

/// How long to wait for a notification to travel PG → listener → broadcast.
/// The documented budget for this hop is ~30 ms (`architecture.md` §3.6f);
/// two seconds is pure CI slack.
const DELIVERY_TIMEOUT: Duration = Duration::from_secs(2);

fn dsn() -> Option<String> {
    std::env::var("DATABASE_URL").ok()
}

/// Wait for the real `legacy_stale` frame, discarding any `RESYNC_EVENT`
/// ("refresh") frames that land first.
///
/// The listener publishes a connect-time resync burst asynchronously — if
/// that connect happens to land AFTER a fixed pre-test sleep+drain instead of
/// before it, the drain misses it, and the first frame `rx.recv()` yields
/// once the real NOTIFY fires is that stale "refresh" frame, not
/// `legacy_stale`. A fixed sleep can't rule that race out; discarding resync
/// frames right where we wait for the real one can, and the whole loop still
/// respects `DELIVERY_TIMEOUT` so a listener that truly never delivers still
/// fails instead of hanging.
async fn recv_legacy_stale(rx: &mut broadcast::Receiver<BroadcastEvent>) -> BroadcastEvent {
    tokio::time::timeout(DELIVERY_TIMEOUT, async {
        loop {
            let event = rx.recv().await.expect("fan-out channel is open");
            if event.name != RESYNC_EVENT {
                return event;
            }
        }
    })
    .await
    .expect("legacy_stale notification should arrive within the delivery budget")
}

fn sample_signal() -> LegacyStaleSignal {
    LegacyStaleSignal::new(
        "hfhotel",
        2,
        "เช็คอิน ห้อง 302",
        vec![
            "เช็คอิน ห้อง 302".to_string(),
            "ห้อง 302 รอ ทำความสะอาด".to_string(),
        ],
    )
}

/// Raw `pg_notify` on the `legacy_stale` channel reaches a subscriber of the
/// site's fan-out, under the `legacy_stale` event name, payload verbatim.
///
/// Deliberately fired as raw SQL rather than through `legacy_stale::publish`:
/// this asserts the SUBSCRIBER half against the channel contract, so it would
/// still fail loudly if the publisher and the listener ever drifted onto
/// different channel names.
#[tokio::test]
async fn pg_notify_on_legacy_stale_reaches_the_fanout() {
    let Some(dsn) = dsn() else {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    };
    let pool = PgPool::connect(&dsn).await.expect("connect");

    let fanout = EventFanout::new();
    let (mut rx, _) = fanout.receivers_for(Branch::Hfhotel);
    let handle =
        spawn_domain_event_listener(dsn.clone(), EventSite::Hfhotel, fanout.hfhotel_sender());

    // The listener LISTENs asynchronously; a NOTIFY fired before it subscribes
    // is simply lost (that is PG's contract, and why reconnects publish a
    // resync). Give it a moment before firing ours — the connect-time resync
    // burst itself is discarded deterministically below, not by a drain here.
    tokio::time::sleep(Duration::from_millis(500)).await;

    let signal = sample_signal();
    let payload = serde_json::to_string(&signal).expect("serialize");
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(LEGACY_STALE_CHANNEL)
        .bind(&payload)
        .execute(&pool)
        .await
        .expect("pg_notify");

    let event = recv_legacy_stale(&mut rx).await;

    assert_eq!(event.name, LEGACY_STALE_EVENT);
    assert_eq!(event.site, EventSite::Hfhotel);
    assert_eq!(&*event.payload, payload.as_str());

    // The payload the browser bridge receives round-trips back into the same
    // type the middleware will parse.
    let parsed: LegacyStaleSignal = serde_json::from_str(&event.payload).expect("round-trip");
    assert_eq!(parsed.count(), 2);
    assert_eq!(parsed.items().len(), 2);

    handle.abort();
}

/// `legacy_stale::publish` (autocommit, no surrounding transaction) is
/// observable by the listener, and a malformed payload on the same channel is
/// dropped WITHOUT killing the listener — the next good signal still arrives.
#[tokio::test]
async fn publish_is_observable_and_garbage_does_not_kill_the_listener() {
    let Some(dsn) = dsn() else {
        eprintln!("skipping: DATABASE_URL unset");
        return;
    };
    let pool = PgPool::connect(&dsn).await.expect("connect");

    let fanout = EventFanout::new();
    let (mut rx, _) = fanout.receivers_for(Branch::Hfhotel);
    let handle =
        spawn_domain_event_listener(dsn.clone(), EventSite::Hfhotel, fanout.hfhotel_sender());

    // See `recv_legacy_stale` — the connect-time resync burst is discarded
    // deterministically at the point we wait for the real signal, not here.
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Garbage first — must be swallowed by `classify_notification`.
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(LEGACY_STALE_CHANNEL)
        .bind("{\"not\":\"a signal\"}")
        .execute(&pool)
        .await
        .expect("pg_notify garbage");

    // …then a real one through the production publisher.
    let signal = sample_signal();
    publish(&pool, &signal).await.expect("publish");

    let event = recv_legacy_stale(&mut rx).await;

    assert_eq!(event.name, LEGACY_STALE_EVENT);
    let parsed: LegacyStaleSignal = serde_json::from_str(&event.payload).expect("round-trip");
    assert_eq!(
        parsed.id(),
        signal.id(),
        "the malformed payload must have been dropped, not forwarded",
    );

    handle.abort();
}
