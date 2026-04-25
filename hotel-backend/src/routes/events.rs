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
//! ## Connection lifecycle
//!
//! - One dedicated `PgListener` per client. `LISTEN` holds a Postgres
//!   connection for the life of the stream — we deliberately do NOT share
//!   one listener across clients, so a slow consumer can't backpressure
//!   others.
//! - When the client disconnects (browser closes, network blip, etc.), the
//!   `Sse` future is dropped; the `stream!` block is cancelled; the
//!   `PgListener` is dropped; the underlying connection returns to / is
//!   closed by sqlx. No explicit cleanup needed.
//! - 30-second heartbeat (`KeepAlive`) emits an SSE comment line so proxies
//!   and load balancers don't idle out the long-lived connection.
//!
//! ## Error handling
//!
//! - Failure to open the listener returns HTTP 500 before the stream starts
//!   (handled in [`stream`]).
//! - A malformed payload from PG is logged and skipped — we do NOT kill the
//!   stream, since one bad event shouldn't sever every browser. Subscribers
//!   reconcile via `event_log` on reconnect (see architecture §3.6e).
//! - `listener.recv()` returning an error means the underlying connection
//!   died; we log and end the stream so the browser's `EventSource`
//!   auto-reconnects (per the spec).

use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::Stream;
use sqlx::postgres::PgListener;

use crate::outbox::event::DomainEvent;
use crate::routes::mode::AppState;

/// SSE channel name. Must match the literal used by
/// [`crate::outbox::bus::EventBus::publish`].
const DOMAIN_EVENTS_CHANNEL: &str = "domain_events";

/// Heartbeat interval. Long enough that we don't waste bandwidth, short
/// enough to beat the typical reverse-proxy idle timeout (60s on most
/// nginx/Cloudflare defaults).
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(30);

/// `GET /api/events` — long-lived SSE stream of every [`DomainEvent`]
/// published since the connection opened.
///
/// Returns `Sse<impl Stream<...>>` rather than an early `ApiResult` because
/// `Sse` wants to own the stream; we surface listener-setup failures by
/// emitting an error comment and ending the stream, letting the browser
/// auto-reconnect.
pub async fn stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let pool = state.new_pool.clone();

    // We can't use `?` here because the function returns `Sse<...>`, not a
    // `Result`. Build the listener up-front so that channel-listen failures
    // surface in the logs immediately, then move ownership into the stream.
    let listener_result = open_domain_events_listener(&pool).await;

    let event_stream = async_stream::stream! {
        let mut listener = match listener_result {
            Ok(listener) => listener,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    "Failed to open PgListener for {DOMAIN_EVENTS_CHANNEL}; closing SSE stream",
                );
                // Surface the failure to the client as a one-shot comment so
                // the browser logs something useful, then end the stream.
                yield Ok(Event::default().comment("listener-open-failed"));
                return;
            }
        };

        loop {
            match listener.recv().await {
                Ok(notification) => {
                    let payload = notification.payload();
                    match serde_json::from_str::<DomainEvent>(payload) {
                        Ok(event) => {
                            yield Ok(Event::default()
                                .event(event.type_name())
                                .data(payload));
                        }
                        Err(parse_err) => {
                            // One bad payload shouldn't sever every browser.
                            // Log and skip; the canonical record is still in
                            // event_log for any subscriber that needs to
                            // reconcile.
                            tracing::warn!(
                                error = %parse_err,
                                payload = %payload,
                                "Skipping malformed domain_events payload",
                            );
                        }
                    }
                }
                Err(recv_err) => {
                    // Connection-level error: the listener can't recover
                    // mid-stream. End the stream and let the browser's
                    // EventSource auto-reconnect (per the WHATWG spec).
                    tracing::warn!(
                        error = %recv_err,
                        "PgListener recv() failed; ending SSE stream so client reconnects",
                    );
                    return;
                }
            }
        }
    };

    Sse::new(event_stream).keep_alive(
        KeepAlive::default()
            .interval(KEEPALIVE_INTERVAL)
            .text("ping"),
    )
}

/// Acquire a dedicated `PgListener` from the shared pool and subscribe to
/// the domain-events channel.
///
/// Extracted so the call-site stays linear and the error-mapping lives in
/// one place. The returned listener owns its connection for the life of the
/// SSE stream.
async fn open_domain_events_listener(pool: &crate::db::PgPool) -> Result<PgListener, sqlx::Error> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(DOMAIN_EVENTS_CHANNEL).await?;
    Ok(listener)
}
