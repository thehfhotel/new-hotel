//! Server-Sent Events (SSE) endpoint for real-time domain-event broadcasting.
//!
//! Per `docs/architecture.md` §3.6e (subscription path) and §3.6f
//! (latency budget — target ~30ms PG-NOTIFY → browser).
//!
//! ## Wire shape
//!
//! Browsers open `GET /api/events` and keep the connection alive. Each
//! [`crate::outbox::event::DomainEvent`] published via `EventBus::publish`
//! lands on the PG `domain_events` channel and is forwarded to every
//! connected client as one SSE message:
//!
//! ```text
//! event: BookingCreated
//! data: {"type":"BookingCreated","data":{...}}
//! ```
//!
//! The `event:` field is the variant discriminant from
//! [`DomainEvent::type_name`], so the browser can filter via
//! `EventSource.addEventListener("BookingCreated", ...)` without parsing the
//! payload.
//!
//! ## Shared LISTEN fan-out (2026-07-29 pool-exhaustion incident)
//!
//! This module used to open a **dedicated `PgListener` per client** with
//! `PgListener::connect_with(pool)`, i.e. every open browser tab held one real
//! sqlx pool slot for the whole life of its stream (two for `branch=all`).
//! Against pools of 10 (hotelnew) / 5 (hotelville) that is a handful of /v2
//! tabs before the pool is gone: `/api/stats|checkins|bookings` blocked for
//! sqlx's default 30s acquire timeout and then 500'd, while the SSE handler
//! itself needed auth + two listener acquires **serially — ~90s with zero
//! bytes written**, long enough for Cloudflare to 524 the stream, after which
//! `EventSource` reconnected onto the same starved pool. Self-sustaining under
//! tab churn.
//!
//! The listener is now a **process-wide singleton per database**:
//!
//! - [`spawn_domain_event_listener`] runs one background task per site, each
//!   holding a **standalone** connection (`PgListener::connect(dsn)` — NOT
//!   `connect_with(pool)`), so it consumes **zero pool slots**.
//! - The task parses each notification once and forwards it into a
//!   [`tokio::sync::broadcast`] channel ([`EventFanout`], one channel per
//!   site, payload tagged with its [`EventSite`]).
//! - [`stream`] performs **no pool acquire at all** after auth: it just
//!   `subscribe()`s to the branch's channel(s). Cost of a new tab is one
//!   `broadcast::Receiver`.
//!
//! Live invariant to check after deploy: `pg_stat_activity` shows exactly
//! **two connections, two channels each** (one connection per site, each
//! `LISTEN`ing on `domain_events` + `legacy_stale`), regardless of how many
//! browser tabs are open.
//!
//! ## Second channel: `legacy_stale`
//!
//! Per `docs/adr/0006-legacy-stale-notification.md`, `bin/writeback.rs`
//! publishes an ephemeral "iHOTEL's room grid is now stale" hint on its own
//! channel after a legacy MSSQL commit. It rides the SAME listener connection
//! — a second `listen()` call, not a second task — precisely to keep the
//! two-connection invariant above intact. [`classify_notification`] is the
//! branch point; everything downstream (fan-out, `branch=` routing, lag
//! handling) is shared unchanged.
//!
//! The `legacy_stale` event name is deliberately absent from
//! [`RESYNC_FANOUT_EVENTS`]: a lagged subscriber or a listener reconnect must
//! never fabricate a "go refresh iHOTEL" toast out of a local hiccup.
//!
//! ## Branch routing
//!
//! Each branch has its own PostgreSQL database with an independent
//! `domain_events` channel. The optional `?branch=` query param selects:
//!
//! - `hfhotel` (default, also `branch` unset) — the hotelnew fan-out
//! - `hfville` — the hotelville fan-out
//! - `all` — multiplex BOTH fan-outs through one stream
//!
//! If the Ville fan-out is unavailable (Ville disabled at startup) for
//! `hfville` or `all`, we degrade to hfhotel-only and emit a tracing warning
//! rather than failing the SSE connection — same contract as the previous
//! pool-based implementation. The Ville channel exists exactly when
//! `AppState::ville_pool` does (see `AppState::with_ville`).
//!
//! ## Connection lifecycle
//!
//! - The stream's first frame is an SSE comment emitted **before** anything is
//!   awaited, so a reverse proxy (Cloudflare) sees response bytes in
//!   milliseconds and can never time the handshake out.
//! - When the client disconnects (browser closes, network blip, etc.) the
//!   `Sse` future is dropped, the `stream!` block is cancelled, and the only
//!   things dropped are the `broadcast::Receiver`s. No connection, no pool
//!   slot, no task — nothing to leak.
//! - 30-second heartbeat (`KeepAlive`) emits an SSE comment line so proxies
//!   and load balancers don't idle out the long-lived connection.
//!
//! ## Error handling
//!
//! - A malformed payload from PG is logged and skipped **in the listener
//!   task** — we do NOT kill any stream, since one bad event shouldn't sever
//!   every browser. Subscribers reconcile via `event_log` (architecture §3.6e).
//! - A dead listener connection is reconnected by its own task with capped
//!   backoff; browsers are never disconnected for it. On every (re)connect —
//!   including the first — the task publishes a **resync** burst so clients
//!   refetch anything that happened while the listener was down.
//! - A slow client whose broadcast receiver falls behind gets `Lagged`; we
//!   emit the same resync burst instead of dropping its stream.

use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    extract::{Query, State},
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::Stream;
use serde::Deserialize;
use sqlx::postgres::PgListener;
use tokio::sync::broadcast;

use crate::outbox::event::DomainEvent;
use crate::outbox::legacy_stale::{LegacyStaleSignal, LEGACY_STALE_CHANNEL, LEGACY_STALE_EVENT};
use crate::routes::mode::{AppState, Branch};

/// SSE channel name. Must match the literal used by
/// [`crate::outbox::bus::EventBus::publish`].
const DOMAIN_EVENTS_CHANNEL: &str = "domain_events";

/// Heartbeat interval. Long enough that we don't waste bandwidth, short
/// enough to beat the typical reverse-proxy idle timeout (60s on most
/// nginx/Cloudflare defaults).
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(30);

/// Ring size of each site's broadcast channel. A subscriber that falls this
/// far behind gets `Lagged` and is served a resync burst instead of the
/// individual events — so the bound is a memory ceiling, not a correctness
/// knob. 512 comfortably absorbs the biggest observed burst (a multi-night
/// booking aggregate emits ~5 events; a CT catch-up sweep a few hundred).
const FANOUT_CAPACITY: usize = 512;

/// Backoff bounds for a listener task that lost its connection. Starts at
/// `MIN` after every successful connect, doubles per failure, caps at `MAX`.
const LISTENER_BACKOFF_MIN: Duration = Duration::from_secs(1);
const LISTENER_BACKOFF_MAX: Duration = Duration::from_secs(30);

/// Comment sent as the very first frame of every stream. Its only job is to
/// put bytes on the wire immediately (Cloudflare 524 prevention); no client
/// reads it.
const HELLO_COMMENT: &str = "connected";

/// Canonical name of the synthetic "you may have missed something, refetch"
/// event.
pub const RESYNC_EVENT: &str = "refresh";

/// Event name(s) a resync burst is replayed under.
///
/// **Formerly a 12-name compat tail** (one frame per [`DomainEvent::type_name`]
/// variant, [`RESYNC_EVENT`] leading): both browser consumers
/// (`lib/v2/use-live-refresh.ts` and the classic dashboard in `app/page.tsx`)
/// used to subscribe only with `EventSource.addEventListener(<domain-event
/// name>, …)` — no wildcard/`onmessage` listener anywhere — so a lone
/// `refresh` frame would have been silently dropped by every page. Replaying
/// the resync under every domain-event name guaranteed it reached whatever
/// subset a page listened for, with zero frontend change.
///
/// Both consumers now ALSO listen for [`RESYNC_EVENT`] unconditionally, on
/// top of whatever domain-event subset each page cares about (see the
/// `useLiveRefresh` doc comment and the SSE effect in `app/page.tsx`), so the
/// tail collapsed to this one canonical name — also the self-documenting name
/// for operators (`curl -N /api/events` shows `event: refresh`).
const RESYNC_FANOUT_EVENTS: &[&str] = &[RESYNC_EVENT];

/// Which canonical database an event came from. Carried on every broadcast
/// item so a `branch=all` stream stays attributable in logs, and so the two
/// site channels can never be confused for one another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSite {
    Hfhotel,
    Hfville,
}

impl EventSite {
    /// Stable lowercase tag — matches the `?branch=` values and `SITE_ID`.
    pub fn as_str(self) -> &'static str {
        match self {
            EventSite::Hfhotel => "hfhotel",
            EventSite::Hfville => "hfville",
        }
    }
}

/// One item on a site's fan-out channel: an already-validated notification
/// payload plus the `event:` name to publish it under.
///
/// `payload` is an `Arc<str>` so fanning one notification out to N subscribers
/// is N refcount bumps, not N string copies.
#[derive(Debug, Clone)]
pub struct BroadcastEvent {
    pub site: EventSite,
    pub name: &'static str,
    pub payload: Arc<str>,
}

/// One outbound SSE frame.
///
/// Exists as its own type because `axum::response::sse::Event` is
/// write-only — it exposes no getters, so a stream of `Event`s cannot be
/// asserted on. Composing the stream over `Frame` keeps ordering (hello frame
/// first), lag handling and branch fan-in unit-testable without a browser, a
/// database, or an HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    /// SSE comment line (`: text`). Never delivered to an `addEventListener`.
    Comment(&'static str),
    /// A named SSE message: `event: <name>` + `data: <data>`.
    Message { name: &'static str, data: Arc<str> },
}

impl Frame {
    fn into_event(self) -> Event {
        match self {
            Frame::Comment(text) => Event::default().comment(text),
            Frame::Message { name, data } => Event::default().event(name).data(&*data),
        }
    }
}

/// Process-wide fan-out handles: one broadcast channel per canonical database.
///
/// Held on [`AppState`] (cheap to clone — one `Arc` bump per sender). The
/// senders are created up front and live for the process lifetime, so
/// `subscribe()` always succeeds even before the listener task has connected
/// and even while zero clients are attached.
///
/// The Ville channel is `Some` exactly when `AppState::ville_pool` is — see
/// [`AppState::with_ville`] — which keeps the `branch=hfville|all`
/// degrade-to-hfhotel contract identical to the pre-fan-out implementation.
#[derive(Clone)]
pub struct EventFanout {
    hfhotel: broadcast::Sender<BroadcastEvent>,
    hfville: Option<broadcast::Sender<BroadcastEvent>>,
}

impl EventFanout {
    /// HF Hotel only. Call [`enable_hfville`](Self::enable_hfville) when the
    /// Ville pool is available.
    pub fn new() -> Self {
        Self {
            hfhotel: broadcast::channel(FANOUT_CAPACITY).0,
            hfville: None,
        }
    }

    /// Create the HF Ville channel. Idempotent: a second call keeps the
    /// existing channel so already-issued receivers stay attached.
    pub fn enable_hfville(&mut self) {
        if self.hfville.is_none() {
            self.hfville = Some(broadcast::channel(FANOUT_CAPACITY).0);
        }
    }

    /// Publisher handle for the HF Hotel listener task.
    pub fn hfhotel_sender(&self) -> broadcast::Sender<BroadcastEvent> {
        self.hfhotel.clone()
    }

    /// Publisher handle for the HF Ville listener task, if Ville is available.
    pub fn hfville_sender(&self) -> Option<broadcast::Sender<BroadcastEvent>> {
        self.hfville.clone()
    }

    /// Pick which fan-out(s) this SSE connection subscribes to, from the
    /// `branch` query param.
    ///
    /// Returns `(primary, Option<secondary>)`; `secondary` is only `Some` for
    /// `Branch::All` when Ville is available — in that case the caller
    /// multiplexes the two. `Branch::Hfville` with no Ville channel falls back
    /// to HF Hotel with a tracing warning rather than 500-ing, matching the
    /// contract documented in the module preamble.
    pub fn receivers_for(
        &self,
        branch: Branch,
    ) -> (
        broadcast::Receiver<BroadcastEvent>,
        Option<broadcast::Receiver<BroadcastEvent>>,
    ) {
        match branch {
            Branch::Hfhotel => (self.hfhotel.subscribe(), None),
            Branch::Hfville => match self.hfville.as_ref() {
                Some(ville) => (ville.subscribe(), None),
                None => {
                    tracing::warn!(
                        "SSE branch=hfville requested but ville fan-out unavailable; falling back to hfhotel"
                    );
                    (self.hfhotel.subscribe(), None)
                }
            },
            Branch::All => match self.hfville.as_ref() {
                Some(ville) => (self.hfhotel.subscribe(), Some(ville.subscribe())),
                None => {
                    tracing::warn!(
                        "SSE branch=all requested but ville fan-out unavailable; falling back to hfhotel-only"
                    );
                    (self.hfhotel.subscribe(), None)
                }
            },
        }
    }
}

impl Default for EventFanout {
    fn default() -> Self {
        Self::new()
    }
}

/// Query string for the SSE endpoint. `branch` is optional so existing
/// callers (no query string) keep their hfhotel-only behavior.
#[derive(Debug, Deserialize, Default)]
pub struct EventStreamQuery {
    pub branch: Option<Branch>,
}

/// `GET /api/events` — long-lived SSE stream of every [`DomainEvent`]
/// published since the connection opened.
///
/// Acquires **no database connection**: after the auth middleware has run, the
/// only work here is `subscribe()` on the branch's broadcast channel(s), which
/// is why a burst of new tabs can no longer starve the pool.
///
/// Returns `Sse<impl Stream<...>>` rather than an `ApiResult` because `Sse`
/// wants to own the stream; there is no fallible setup left to report.
pub async fn stream(
    State(state): State<AppState>,
    Query(query): Query<EventStreamQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let branch = query.branch.unwrap_or_default();
    let (primary, secondary) = state.event_fanout.receivers_for(branch);
    let frames = frame_stream(primary, secondary);

    let event_stream = async_stream::stream! {
        for await frame in frames {
            yield Ok::<Event, Infallible>(frame.into_event());
        }
    };

    Sse::new(event_stream).keep_alive(
        KeepAlive::default()
            .interval(KEEPALIVE_INTERVAL)
            .text("ping"),
    )
}

/// Compose the outbound frame sequence for one client.
///
/// Split out from [`stream`] (and typed over [`Frame`] rather than
/// `sse::Event`) so the three behaviours that matter are testable without a
/// database: the hello frame precedes every event, a `branch=all` stream
/// interleaves both sites, and a lagged subscriber gets a resync burst instead
/// of a disconnect.
fn frame_stream(
    primary: broadcast::Receiver<BroadcastEvent>,
    secondary: Option<broadcast::Receiver<BroadcastEvent>>,
) -> impl Stream<Item = Frame> {
    async_stream::stream! {
        // FIRST, before any await: bytes on the wire in milliseconds so a
        // reverse proxy never times out the handshake (the 524 class).
        yield Frame::Comment(HELLO_COMMENT);

        let mut primary = primary;
        let mut secondary = secondary;

        loop {
            // With two receivers, race them and forward whichever fires
            // first; `broadcast::Receiver::recv` is cancel-safe, so the
            // losing arm keeps its place in the ring. The single-receiver
            // branch (default) stays on the simple `.recv().await` path.
            let (from_primary, result) = match secondary.as_mut() {
                Some(sec) => {
                    tokio::select! {
                        result = primary.recv() => (true, result),
                        result = sec.recv() => (false, result),
                    }
                }
                None => (true, primary.recv().await),
            };

            match result {
                Ok(event) => {
                    yield Frame::Message { name: event.name, data: event.payload };
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    // This client couldn't keep up with the ring. The events
                    // it missed are gone, but the canonical state isn't — tell
                    // it to refetch rather than severing the stream.
                    tracing::warn!(
                        skipped,
                        from_primary,
                        "SSE subscriber lagged behind the domain-event fan-out; emitting resync",
                    );
                    for frame in resync_frames(None, "subscriber-lagged") {
                        yield frame;
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    // A closed receiver is permanently ready, so it MUST be
                    // dropped or the select! below would spin hot on it.
                    // Senders live on AppState for the process lifetime, so in
                    // practice this only happens at shutdown.
                    if from_primary {
                        match secondary.take() {
                            Some(sec) => {
                                tracing::warn!(
                                    "primary domain-event fan-out closed; continuing on secondary",
                                );
                                primary = sec;
                            }
                            None => {
                                tracing::warn!(
                                    "domain-event fan-out closed; ending SSE stream so client reconnects",
                                );
                                return;
                            }
                        }
                    } else {
                        tracing::warn!(
                            "secondary domain-event fan-out closed; continuing on primary",
                        );
                        secondary = None;
                    }
                }
            }
        }
    }
}

/// JSON body carried by every frame of a resync burst.
///
/// Deliberately NOT shaped like a real [`DomainEvent`]: the burst is replayed
/// under the canonical resync name(s) (see [`RESYNC_FANOUT_EVENTS`]), so
/// anything that does parse the body must see immediately that this is a
/// refetch marker and not a booking/check-in that actually happened.
fn resync_payload(site: Option<EventSite>, reason: &str) -> String {
    serde_json::json!({
        "type": "Resync",
        "data": {
            "site": site.map(EventSite::as_str),
            "reason": reason,
        }
    })
    .to_string()
}

/// Resync burst as stream frames — used for the per-client `Lagged` path.
fn resync_frames(site: Option<EventSite>, reason: &str) -> Vec<Frame> {
    let data: Arc<str> = Arc::from(resync_payload(site, reason).as_str());
    RESYNC_FANOUT_EVENTS
        .iter()
        .map(|name| Frame::Message {
            name,
            data: data.clone(),
        })
        .collect()
}

/// Resync burst on the fan-out — used by a listener task after every
/// (re)connect, so every attached client refetches whatever it missed while
/// the LISTEN connection was down.
///
/// Sends are best-effort: `broadcast::Sender::send` fails when nobody is
/// subscribed (no browser tabs open), which is the normal steady state.
fn publish_resync(tx: &broadcast::Sender<BroadcastEvent>, site: EventSite, reason: &str) {
    let payload: Arc<str> = Arc::from(resync_payload(Some(site), reason).as_str());
    for name in RESYNC_FANOUT_EVENTS.iter().copied() {
        let _ = tx.send(BroadcastEvent {
            site,
            name,
            payload: payload.clone(),
        });
    }
}

/// Spawn the process-wide `domain_events` listener for one database.
///
/// `dsn` must be the same connection info the site's pool was built from —
/// callers pass `NewDbConfig::connection_string()` /
/// `VilleDbConfig::connection_string()` (`main.rs`), which is the canonical
/// source used to build the pools themselves.
///
/// The task owns a **standalone** connection (`PgListener::connect`), NOT one
/// borrowed from the sqlx pool — that is the whole point of the fan-out (see
/// module preamble). It runs for the life of the process, reconnecting with
/// capped backoff, and never panics.
pub fn spawn_domain_event_listener(
    dsn: String,
    site: EventSite,
    tx: broadcast::Sender<BroadcastEvent>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move { run_domain_event_listener(dsn, site, tx).await })
}

async fn run_domain_event_listener(
    dsn: String,
    site: EventSite,
    tx: broadcast::Sender<BroadcastEvent>,
) {
    let mut backoff = LISTENER_BACKOFF_MIN;

    loop {
        match open_domain_events_listener(&dsn).await {
            Ok(mut listener) => {
                backoff = LISTENER_BACKOFF_MIN;
                tracing::info!(
                    site = site.as_str(),
                    channels = %format_args!("{DOMAIN_EVENTS_CHANNEL},{LEGACY_STALE_CHANNEL}"),
                    "domain-event listener connected; publishing resync to attached clients",
                );
                // Every (re)connect, INCLUDING the first: whatever was
                // published while we were not listening is unrecoverable from
                // here, so tell attached clients to refetch.
                publish_resync(&tx, site, "listener-connect");

                loop {
                    match listener.recv().await {
                        Ok(notification) => {
                            // One bad payload must not sever every browser —
                            // `classify_notification` logs and returns None,
                            // and the canonical record is still in event_log
                            // for any subscriber that needs to reconcile.
                            if let Some(event) = classify_notification(
                                notification.channel(),
                                notification.payload(),
                                site,
                            ) {
                                // Err == nobody subscribed right now.
                                let _ = tx.send(event);
                            }
                        }
                        Err(recv_err) => {
                            tracing::warn!(
                                site = site.as_str(),
                                error = %recv_err,
                                "domain-event listener connection lost; reconnecting",
                            );
                            break;
                        }
                    }
                }
            }
            Err(err) => {
                tracing::warn!(
                    site = site.as_str(),
                    error = %err,
                    backoff_secs = backoff.as_secs(),
                    "Failed to open domain-event listener; retrying after backoff",
                );
            }
        }

        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(LISTENER_BACKOFF_MAX);
    }
}

/// Open a standalone `PgListener` (its own connection, outside every pool)
/// and subscribe to BOTH channels this module fans out.
///
/// Two `listen()` calls on ONE connection, not two listeners: the 2026-07-29
/// pool-exhaustion incident's fix is "one standalone connection per site", and
/// a second task would quietly double that count (see the module preamble's
/// live invariant).
async fn open_domain_events_listener(dsn: &str) -> Result<PgListener, sqlx::Error> {
    let mut listener = PgListener::connect(dsn).await?;
    listener.listen(DOMAIN_EVENTS_CHANNEL).await?;
    listener.listen(LEGACY_STALE_CHANNEL).await?;
    Ok(listener)
}

/// Turn one raw PG notification into the broadcast item it should fan out as,
/// or `None` if it must be dropped.
///
/// Extracted from the listener's recv loop for the same reason [`Frame`] is a
/// type of its own: it makes the fan-in decision assertable without a browser,
/// a database, or an HTTP server. The only side effect is a `tracing` line on
/// the reject paths, so it stays callable from a plain `#[test]`.
///
/// `site` is the database the notification arrived on — NOT anything read out
/// of the payload. A `legacy_stale` signal carries its own `site` string for
/// diagnostics, but trusting it would let a mis-targeted writeback worker
/// deliver Ville toasts to HF Hotel's reception.
fn classify_notification(channel: &str, payload: &str, site: EventSite) -> Option<BroadcastEvent> {
    match channel {
        DOMAIN_EVENTS_CHANNEL => match serde_json::from_str::<DomainEvent>(payload) {
            Ok(event) => Some(BroadcastEvent {
                site,
                name: event.type_name(),
                payload: Arc::from(payload),
            }),
            Err(parse_err) => {
                tracing::warn!(
                    site = site.as_str(),
                    error = %parse_err,
                    payload = %payload,
                    "Skipping malformed domain_events payload",
                );
                None
            }
        },
        // Validated, then forwarded VERBATIM: the browser bridge and the
        // Tauri middleware downstream parse the same JSON, so re-serializing
        // here would be a second place for the wire shape to drift.
        LEGACY_STALE_CHANNEL => match serde_json::from_str::<LegacyStaleSignal>(payload) {
            Ok(_) => Some(BroadcastEvent {
                site,
                name: LEGACY_STALE_EVENT,
                payload: Arc::from(payload),
            }),
            Err(parse_err) => {
                tracing::warn!(
                    site = site.as_str(),
                    error = %parse_err,
                    payload = %payload,
                    "Skipping malformed legacy_stale payload",
                );
                None
            }
        },
        other => {
            // Unreachable unless someone adds a `listen()` without a branch
            // here — in which case dropping is right and the log says why.
            tracing::warn!(
                site = site.as_str(),
                channel = %other,
                "Notification on an unhandled channel; dropping",
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::pin;
    use std::task::Poll;

    fn sample(site: EventSite, name: &'static str) -> BroadcastEvent {
        BroadcastEvent {
            site,
            name,
            payload: Arc::from(format!(r#"{{"type":"{name}"}}"#).as_str()),
        }
    }

    /// Pull the next frame without pulling in `futures_util::StreamExt` —
    /// `futures-util` is declared `default-features = false` here, so only the
    /// bare `Stream` trait is guaranteed available.
    async fn next_frame<S: Stream<Item = Frame>>(stream: &mut std::pin::Pin<&mut S>) -> Frame {
        std::future::poll_fn(|cx| stream.as_mut().poll_next(cx))
            .await
            .expect("frame stream must not terminate")
    }

    /// A frame is available right now (no waiting).
    fn poll_now<S: Stream<Item = Frame>>(stream: &mut std::pin::Pin<&mut S>) -> Poll<Option<Frame>> {
        let waker = std::task::Waker::noop();
        let mut cx = std::task::Context::from_waker(waker);
        stream.as_mut().poll_next(&mut cx)
    }

    /// The resync burst is down to the one canonical name: both browser
    /// consumers (`lib/v2/use-live-refresh.ts`, `app/page.tsx`) now listen
    /// for [`RESYNC_EVENT`] unconditionally, on top of whatever domain-event
    /// subset each page cares about — see the doc comment on
    /// [`RESYNC_FANOUT_EVENTS`] for the history of the 12-name compat tail
    /// this replaced. If this test starts failing because someone widened the
    /// list again, confirm the widening is actually needed (a frontend
    /// consumer that doesn't listen for `refresh`) before accepting it.
    #[test]
    fn resync_burst_is_the_single_canonical_event() {
        assert_eq!(
            RESYNC_FANOUT_EVENTS,
            &[RESYNC_EVENT],
            "resync burst should be exactly one frame now that every frontend \
             consumer listens for the canonical `refresh` event",
        );
    }

    /// A listener (re)connect publishes the resync burst onto the site's
    /// channel so every attached client refetches what it missed.
    #[test]
    fn listener_reconnect_publishes_resync_to_subscribers() {
        let fanout = EventFanout::new();
        let (mut rx, _) = fanout.receivers_for(Branch::Hfhotel);

        publish_resync(&fanout.hfhotel_sender(), EventSite::Hfhotel, "listener-connect");

        let names: Vec<&str> = (0..RESYNC_FANOUT_EVENTS.len())
            .map(|_| rx.try_recv().expect("resync frame").name)
            .collect();
        assert_eq!(names, RESYNC_FANOUT_EVENTS.to_vec());
        assert!(rx.try_recv().is_err(), "burst must be exactly one pass");

        let payload = resync_payload(Some(EventSite::Hfhotel), "listener-connect");
        assert!(payload.contains("\"reason\":\"listener-connect\""));
        assert!(payload.contains("\"site\":\"hfhotel\""));
        assert!(
            payload.contains("\"type\":\"Resync\""),
            "body must self-identify as a refetch marker, not a real domain event",
        );
    }

    /// `branch=hfhotel` must never observe a Ville event. The two sites are
    /// separate channels, so this is structural — assert it stays that way.
    #[tokio::test]
    async fn hfhotel_subscriber_never_sees_ville_events() {
        let mut fanout = EventFanout::new();
        fanout.enable_hfville();

        let (hotel_rx, hotel_secondary) = fanout.receivers_for(Branch::Hfhotel);
        assert!(hotel_secondary.is_none(), "hfhotel is a single-channel stream");

        // Keep a Ville receiver alive so the send below actually lands on the
        // Ville ring (a broadcast send with zero subscribers is a no-op).
        let (_ville_keepalive, _) = fanout.receivers_for(Branch::Hfville);
        fanout
            .hfville_sender()
            .expect("ville enabled")
            .send(sample(EventSite::Hfville, "CheckInCreated"))
            .expect("ville subscriber attached");

        let mut stream = pin!(frame_stream(hotel_rx, None));
        assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));
        assert!(
            matches!(poll_now(&mut stream), Poll::Pending),
            "a ville-tagged event must not reach an hfhotel subscriber",
        );

        // …and the hotel channel still delivers its own events.
        fanout
            .hfhotel_sender()
            .send(sample(EventSite::Hfhotel, "RoomMarkedClean"))
            .expect("hfhotel subscriber attached");
        match next_frame(&mut stream).await {
            Frame::Message { name, .. } => assert_eq!(name, "RoomMarkedClean"),
            other => panic!("expected RoomMarkedClean message, got {other:?}"),
        }
    }

    /// `branch=all` multiplexes BOTH sites through one stream.
    #[tokio::test]
    async fn all_branch_sees_both_sites() {
        let mut fanout = EventFanout::new();
        fanout.enable_hfville();

        let (primary, secondary) = fanout.receivers_for(Branch::All);
        assert!(secondary.is_some(), "branch=all must subscribe to both sites");

        fanout
            .hfhotel_sender()
            .send(sample(EventSite::Hfhotel, "BookingCreated"))
            .expect("hfhotel subscriber attached");
        fanout
            .hfville_sender()
            .expect("ville enabled")
            .send(sample(EventSite::Hfville, "CheckOutCompleted"))
            .expect("hfville subscriber attached");

        let mut stream = pin!(frame_stream(primary, secondary));
        assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));

        let mut seen = Vec::new();
        for _ in 0..2 {
            match next_frame(&mut stream).await {
                Frame::Message { name, .. } => seen.push(name),
                other => panic!("expected message frame, got {other:?}"),
            }
        }
        seen.sort_unstable();
        assert_eq!(seen, vec!["BookingCreated", "CheckOutCompleted"]);
    }

    /// Ville unavailable: `hfville` and `all` degrade to hfhotel-only instead
    /// of failing the connection (unchanged from the pool-based behaviour).
    #[test]
    fn missing_ville_degrades_to_hfhotel() {
        let fanout = EventFanout::new();

        let (_, secondary) = fanout.receivers_for(Branch::All);
        assert!(secondary.is_none(), "branch=all degrades to hfhotel-only");

        let (mut rx, secondary) = fanout.receivers_for(Branch::Hfville);
        assert!(secondary.is_none());
        fanout
            .hfhotel_sender()
            .send(sample(EventSite::Hfhotel, "BookingCreated"))
            .expect("fallback subscriber is on the hfhotel channel");
        assert_eq!(rx.try_recv().expect("fallback delivery").name, "BookingCreated");
    }

    /// The hello comment is emitted before anything is awaited — that is what
    /// gives Cloudflare first bytes in milliseconds and prevents the 524 that
    /// started the reconnect spiral.
    #[tokio::test]
    async fn hello_frame_precedes_any_event() {
        let fanout = EventFanout::new();
        let (rx, _) = fanout.receivers_for(Branch::Hfhotel);
        fanout
            .hfhotel_sender()
            .send(sample(EventSite::Hfhotel, "BookingCreated"))
            .expect("subscriber attached");

        let mut stream = pin!(frame_stream(rx, None));
        assert_eq!(
            poll_now(&mut stream),
            Poll::Ready(Some(Frame::Comment(HELLO_COMMENT))),
            "hello frame must be ready without awaiting the channel",
        );
        match next_frame(&mut stream).await {
            Frame::Message { name, .. } => assert_eq!(name, "BookingCreated"),
            other => panic!("expected the queued event next, got {other:?}"),
        }
    }

    /// A slow client that overruns the ring gets a resync burst, NOT a
    /// disconnect — the stream keeps serving afterwards.
    #[tokio::test]
    async fn lagged_subscriber_gets_resync_and_stream_survives() {
        // Own channel with a tiny ring so overrun is deterministic.
        let (tx, rx) = broadcast::channel::<BroadcastEvent>(2);
        let mut stream = pin!(frame_stream(rx, None));
        assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));

        for _ in 0..5 {
            tx.send(sample(EventSite::Hfhotel, "BookingCreated"))
                .expect("subscriber attached");
        }

        // First wake-up after the overrun is the resync burst.
        for &expected in RESYNC_FANOUT_EVENTS {
            match next_frame(&mut stream).await {
                Frame::Message { name, data } => {
                    assert_eq!(name, expected);
                    assert!(data.contains("subscriber-lagged"), "resync must say why");
                }
                other => panic!("expected resync frame, got {other:?}"),
            }
        }

        // Stream is alive: the two events that survived the ring still flow…
        for _ in 0..2 {
            match next_frame(&mut stream).await {
                Frame::Message { name, .. } => assert_eq!(name, "BookingCreated"),
                other => panic!("expected buffered event, got {other:?}"),
            }
        }

        // …and so does one published after the lag.
        tx.send(sample(EventSite::Hfhotel, "RoomMarkedDirty"))
            .expect("subscriber still attached");
        match next_frame(&mut stream).await {
            Frame::Message { name, .. } => assert_eq!(name, "RoomMarkedDirty"),
            other => panic!("stream must keep delivering after a Lagged resync, got {other:?}"),
        }
    }

    /// Both fan-out channels closing ends the stream so the browser's
    /// `EventSource` reconnects (per the WHATWG spec).
    #[tokio::test]
    async fn closed_fanout_ends_stream() {
        let (tx, rx) = broadcast::channel::<BroadcastEvent>(4);
        let mut stream = pin!(frame_stream(rx, None));
        assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));
        drop(tx);
        assert_eq!(
            std::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await,
            None,
        );
    }

    /// A closed *secondary* must be dropped rather than re-polled — otherwise
    /// the `select!` spins hot on a permanently-ready arm.
    #[tokio::test]
    async fn closed_secondary_falls_back_to_primary() {
        let (primary_tx, primary_rx) = broadcast::channel::<BroadcastEvent>(4);
        let (secondary_tx, secondary_rx) = broadcast::channel::<BroadcastEvent>(4);

        let mut stream = pin!(frame_stream(primary_rx, Some(secondary_rx)));
        assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));

        drop(secondary_tx);
        primary_tx
            .send(sample(EventSite::Hfhotel, "CustomerModified"))
            .expect("primary subscriber attached");

        match next_frame(&mut stream).await {
            Frame::Message { name, .. } => assert_eq!(name, "CustomerModified"),
            other => panic!("expected primary event after secondary closed, got {other:?}"),
        }
    }

    /// Client disconnect (browser closed, network blip) drops the `Sse`
    /// future, which drops this stream. The ONLY thing that must be released
    /// is the broadcast receiver — there is no connection, pool slot or task
    /// behind a subscription any more. `receiver_count` returning to zero is
    /// the observable proof.
    #[tokio::test]
    async fn dropping_the_stream_releases_the_subscription() {
        let fanout = EventFanout::new();
        let tx = fanout.hfhotel_sender();
        assert_eq!(tx.receiver_count(), 0, "no clients attached yet");

        let (rx, _) = fanout.receivers_for(Branch::Hfhotel);
        assert_eq!(tx.receiver_count(), 1, "subscribe() is the whole cost of a tab");

        {
            let mut stream = pin!(frame_stream(rx, None));
            assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));
            assert_eq!(tx.receiver_count(), 1);
        }

        assert_eq!(
            tx.receiver_count(),
            0,
            "a dropped stream must release its receiver — otherwise every tab churn leaks a slot",
        );
    }

    /// `Frame` → `sse::Event` conversion must not panic on either variant
    /// (axum has no getters, so this is the reachable assertion).
    #[test]
    fn frames_convert_to_sse_events() {
        let _ = Frame::Comment(HELLO_COMMENT).into_event();
        let _ = Frame::Message {
            name: RESYNC_EVENT,
            data: Arc::from(r#"{"type":"Resync"}"#),
        }
        .into_event();
    }

    /// Site tags are the same lowercase strings as `?branch=` / `SITE_ID`.
    #[test]
    fn site_tags_match_branch_vocabulary() {
        assert_eq!(EventSite::Hfhotel.as_str(), "hfhotel");
        assert_eq!(EventSite::Hfville.as_str(), "hfville");
    }

    /// A real `DomainEvent` payload, serialized the way `EventBus::publish`
    /// puts it on the wire.
    fn domain_event_payload() -> String {
        let event = DomainEvent::RoomMarkedDirty {
            room_id: uuid::Uuid::nil(),
            source: crate::outbox::event::EventSource::System {
                reason: "test".into(),
            },
        };
        serde_json::to_string(&event).expect("serialize domain event")
    }

    /// A real `legacy_stale` payload, built through the publisher's own
    /// constructor so a wire-shape change breaks this test too.
    fn legacy_stale_payload() -> String {
        let signal = crate::outbox::legacy_stale::LegacyStaleSignal::new(
            "hfhotel",
            3,
            "เช็คอิน ห้อง 302",
            vec!["เช็คอิน ห้อง 302".to_string()],
        );
        serde_json::to_string(&signal).expect("serialize legacy stale signal")
    }

    /// `domain_events` keeps its existing behaviour: the SSE `event:` name is
    /// the variant discriminant and the payload is forwarded verbatim.
    #[test]
    fn classify_forwards_domain_events_under_their_variant_name() {
        let payload = domain_event_payload();
        let event = classify_notification(DOMAIN_EVENTS_CHANNEL, &payload, EventSite::Hfhotel)
            .expect("valid domain event must classify");

        assert_eq!(event.name, "RoomMarkedDirty");
        assert_eq!(event.site, EventSite::Hfhotel);
        assert_eq!(&*event.payload, payload.as_str());
    }

    /// `legacy_stale` fans out under its own name, tagged with the database
    /// it arrived on — NOT with the `site` string inside the payload.
    #[test]
    fn classify_forwards_legacy_stale_under_its_own_name() {
        let payload = legacy_stale_payload();
        let event = classify_notification(LEGACY_STALE_CHANNEL, &payload, EventSite::Hfville)
            .expect("valid legacy_stale signal must classify");

        assert_eq!(event.name, LEGACY_STALE_EVENT);
        assert_eq!(event.name, "legacy_stale");
        assert_eq!(
            event.site,
            EventSite::Hfville,
            "the listener's database decides the site, never the payload's own field",
        );
        assert_eq!(&*event.payload, payload.as_str());
    }

    /// Garbage on either channel is dropped, not forwarded and not panicked
    /// on — one bad payload must never sever every attached browser.
    #[test]
    fn classify_drops_malformed_payloads_without_panicking() {
        for channel in [DOMAIN_EVENTS_CHANNEL, LEGACY_STALE_CHANNEL] {
            for payload in ["", "not json at all", "{}", r#"{"type":"NopeNotAVariant"}"#] {
                assert!(
                    classify_notification(channel, payload, EventSite::Hfhotel).is_none(),
                    "channel {channel} must drop payload {payload:?}",
                );
            }
        }
        // …and a payload that is valid for the OTHER channel is still garbage
        // for this one.
        assert!(
            classify_notification(DOMAIN_EVENTS_CHANNEL, &legacy_stale_payload(), EventSite::Hfhotel)
                .is_none(),
        );
        assert!(
            classify_notification(LEGACY_STALE_CHANNEL, &domain_event_payload(), EventSite::Hfhotel)
                .is_none(),
        );
    }

    /// A channel nobody wired a branch for is dropped rather than guessed at.
    #[test]
    fn classify_drops_unknown_channels() {
        assert!(
            classify_notification("some_future_channel", "{}", EventSite::Hfhotel).is_none(),
        );
    }

    /// A classified `legacy_stale` item survives the shared fan-out path and
    /// comes out of the stream as an SSE frame under its own event name —
    /// the half of the chain the DB-backed test in `tests/test_legacy_stale.rs`
    /// hands off to.
    #[tokio::test]
    async fn legacy_stale_reaches_a_subscriber_as_its_own_frame() {
        let fanout = EventFanout::new();
        let (rx, _) = fanout.receivers_for(Branch::Hfhotel);
        let payload = legacy_stale_payload();

        let event = classify_notification(LEGACY_STALE_CHANNEL, &payload, EventSite::Hfhotel)
            .expect("classify");
        fanout.hfhotel_sender().send(event).expect("subscriber attached");

        let mut stream = pin!(frame_stream(rx, None));
        assert_eq!(next_frame(&mut stream).await, Frame::Comment(HELLO_COMMENT));
        match next_frame(&mut stream).await {
            Frame::Message { name, data } => {
                assert_eq!(name, LEGACY_STALE_EVENT);
                assert_eq!(&*data, payload.as_str());
            }
            other => panic!("expected a legacy_stale frame, got {other:?}"),
        }
    }

    /// `legacy_stale` must NOT be replayed by a resync burst. A lagged
    /// subscriber or a listener reconnect is a local hiccup in OUR stream —
    /// it says nothing about whether iHOTEL's grid is stale, and fabricating
    /// a toast from it is exactly the alarm-fatigue failure ADR 0006 §5 is
    /// built to avoid.
    #[test]
    fn resync_burst_never_fabricates_a_legacy_stale_signal() {
        assert!(
            !RESYNC_FANOUT_EVENTS.contains(&LEGACY_STALE_EVENT),
            "legacy_stale must not ride the resync burst",
        );
        for frame in resync_frames(None, "subscriber-lagged") {
            match frame {
                Frame::Message { name, .. } => assert_ne!(name, LEGACY_STALE_EVENT),
                Frame::Comment(_) => {}
            }
        }
    }
}
