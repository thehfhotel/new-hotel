//! Hotel Management System Backend
//!
//! A Rust/Axum backend server for the hotel management system.
//! Replaces the Next.js API routes with a high-performance Rust implementation.
//!
//! Supports dual-database architecture:
//! - Legacy database (SQL Server, shared with legacy application) - Optional
//! - New HotelNew database (PostgreSQL, owned by this application) - Primary
//!
//! When SYSTEM_MODE=new, the app can run without the legacy database.
//! Legacy routes will return 503 Service Unavailable when legacy DB is unavailable.

// Modules live in `lib.rs` so they're reachable from integration tests under
// `tests/`. The binary brings them into scope via `use hotel_backend::*`.
// Phase 1b added `repository`; Phase 3b moved declaration to lib.rs — kept here
// so integration tests can reach all 11 top-level modules.
use hotel_backend::{config, db, middleware as app_middleware, routes, scheduler};

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    middleware as axum_middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{auth_enabled_from_env, AppConfig};
use crate::db::{create_pg_pool, create_pool, pg_pool_options};
use crate::routes::events::{spawn_domain_event_listener, EventSite};
use crate::routes::mode::{AppState, SystemMode};
use crate::scheduler::init_scheduler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hydrate env vars from /run/secrets/<file> BEFORE anything reads env.
    // Backend container has DB_PASSWORD/POSTGRES_PASSWORD/etc removed from
    // its environment: block in favor of Docker secrets (see CLAUDE.md
    // "Credentials & Docker secrets"). Without this call,
    // AppConfig::from_env() panics on missing DB_PASSWORD.
    hotel_backend::secrets::hydrate_env_from_secret_files();

    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hotel_backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env();
    tracing::info!("Configuration loaded");
    tracing::info!(
        "Legacy Database: {} / {}",
        config.db.server,
        config.db.database
    );
    tracing::info!(
        "New Database (PostgreSQL): {} / {}",
        config.new_db.server,
        config.new_db.database
    );
    tracing::info!(
        "HF Ville Database: {} / {} (enabled: {})",
        config.ville_db.server,
        config.ville_db.database,
        config.ville_db.enabled
    );
    tracing::info!("System Mode: {:?}", config.mode);
    tracing::info!("Server: {}", config.server.addr());

    // Determine system mode
    let system_mode = match config.mode {
        config::SystemMode::Legacy => SystemMode::Legacy,
        config::SystemMode::New => SystemMode::New,
    };

    // Try to create the new HotelNew PostgreSQL pool first (required for New mode)
    let new_pool = match create_pg_pool(&config.new_db).await {
        Ok(pool) => {
            tracing::info!("HotelNew PostgreSQL pool created successfully");
            Some(pool)
        }
        Err(e) => {
            if system_mode == SystemMode::New {
                // In New mode, HotelNew database is required
                return Err(format!(
                    "Failed to connect to HotelNew PostgreSQL database (required in New mode): {}",
                    e
                )
                .into());
            }
            tracing::warn!("Failed to create HotelNew pool: {}", e);
            None
        }
    };

    // Try to create legacy database connection pool (SQL Server)
    let legacy_pool = match create_pool(&config.db).await {
        Ok(pool) => {
            tracing::info!("Legacy database pool created successfully");
            Some(pool)
        }
        Err(e) => {
            if system_mode == SystemMode::Legacy {
                // In Legacy mode, legacy database is required
                return Err(format!(
                    "Failed to connect to legacy database (required in Legacy mode): {}",
                    e
                )
                .into());
            }
            tracing::warn!(
                "Failed to create legacy pool (legacy routes will be unavailable): {}",
                e
            );
            None
        }
    };

    // Create HF Ville pool. Phase 5 Ville cutover (#76, 2026-04-30) repointed
    // this from the legacy `?options=-csearch_path%3Dville` (ville schema in
    // hotelnew) to the new `hotelville` database (fed by sync-hfville CT
    // watcher). Now uses `VilleDbConfig` verbatim — VILLE_DB_SERVER / _PORT /
    // _NAME / _USER / _PASSWORD env vars drive the connection.
    let ville_pool = if config.ville_db.enabled {
        let ville_conn = config.ville_db.connection_string();
        // `pg_pool_options` (not a bare `PgPoolOptions::new()`) so this pool
        // carries the same explicit acquire bound as the HotelNew one — see
        // `db::PG_ACQUIRE_TIMEOUT` and the 2026-07-29 SSE incident.
        match pg_pool_options(config.ville_db.pool_max)
            .connect(&ville_conn)
            .await
        {
            Ok(pool) => {
                tracing::info!(
                    "HF Ville pool created ({}:{}/{})",
                    config.ville_db.server,
                    config.ville_db.port,
                    config.ville_db.database
                );
                Some(pool)
            }
            Err(e) => {
                tracing::warn!("Failed to create HF Ville pool: {}", e);
                None
            }
        }
    } else {
        tracing::info!("HF Ville database disabled (VILLE_DB_ENABLED not set)");
        None
    };

    // Phase 4 PR2: read AUTH_ENABLED once at startup. Defaults to false
    // so existing deployments stay unauthenticated until the operator
    // provisions an admin via `cargo run --bin create_user` and flips
    // the flag. The `/api/auth/*` endpoints are mounted regardless so
    // the frontend can probe for "auth is on" via `/api/auth/me`.
    let auth_enabled = auth_enabled_from_env();
    tracing::info!("Auth middleware: enabled={}", auth_enabled);

    // ADR 0002 coexistence: HF Ville writes from the new app are OFF by default.
    // When false, the ville-write guard middleware 403s every `branch=hfville`
    // mutation so a Ville write can never misroute into the HF Hotel pool.
    let hfville_writes = std::env::var("HFVILLE_WRITES_ENABLED")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    tracing::info!("HF Ville writes: enabled={}", hfville_writes);

    // Loyalty-app integration (docs/loyalty-channel.md). Inbound channel
    // ships DARK (flag + token both required); outbound stay hook is off
    // unless LOYALTY_APP_URL + LOYALTY_SERVICE_TOKEN are set.
    let loyalty_config = config::LoyaltyConfig::from_env();
    tracing::info!(
        "Loyalty channel: enabled={} (token set: {}); stay hook configured: {}",
        loyalty_config.channel_enabled,
        loyalty_config.channel_token.is_some(),
        loyalty_config.stay_hook_configured()
    );

    // Create AppState — canonical PG only. As of the 2026-06-11 coexistence
    // audit AppState carries no MSSQL handle: routes/repositories never touch
    // the legacy DB (docs/architecture.md "critical rule"); MSSQL is reserved
    // for the adapter workers (sync/writeback bins) and the scheduler's
    // reconcile backstop, which receives the tiberius pool directly below.
    let legacy_available = legacy_pool.is_some();
    let (final_app_state, new_available) = match new_pool {
        Some(new_hotel) => {
            let mut state = AppState::with_mode(new_hotel, system_mode)
                .with_auth_enabled(auth_enabled)
                .with_hfville_writes(hfville_writes);
            if let Some(vp) = ville_pool {
                state = state.with_ville(vp);
            }
            (Some(state), true)
        }
        None if legacy_available => {
            tracing::warn!(
                "Legacy-only mode: HotelNew database unavailable, new routes will not work"
            );
            (None, false)
        }
        None => {
            return Err("No database connections available".into());
        }
    };

    // Shared `domain_events` LISTEN fan-out — ONE dedicated connection per
    // canonical database, forwarding into the per-site broadcast channels the
    // `/api/events` SSE handler subscribes to (`routes::events`).
    //
    // These are STANDALONE connections (`PgListener::connect(dsn)`), NOT pool
    // checkouts: before 2026-07-29 the SSE handler opened a listener per
    // client with `connect_with(pool)`, so every open /v2 tab held 1–2 real
    // pool slots for its whole lifetime and tab churn exhausted the pool (30s
    // acquire hangs → 500s on /api/stats|checkins|bookings, ~90s zero-byte SSE
    // setup → Cloudflare 524 → EventSource reconnect spiral). Post-fix
    // invariant: `pg_stat_activity` shows exactly two `LISTEN "domain_events"`
    // connections total, no matter how many browser tabs are open.
    //
    // The DSNs are the same `connection_string()`s the pools above were built
    // from, so the listener can never drift onto a different database.
    if let Some(ref state) = final_app_state {
        spawn_domain_event_listener(
            config.new_db.connection_string(),
            EventSite::Hfhotel,
            state.event_fanout.hfhotel_sender(),
        );
        // `hfville_sender()` is Some exactly when `ville_pool` is (AppState::
        // with_ville), so a disabled/unreachable Ville DB spawns no task and
        // `branch=hfville|all` degrades to hfhotel-only as before.
        match state.event_fanout.hfville_sender() {
            Some(ville_tx) => {
                spawn_domain_event_listener(
                    config.ville_db.connection_string(),
                    EventSite::Hfville,
                    ville_tx,
                );
            }
            None => tracing::info!(
                "HF Ville domain-event listener not started (Ville pool unavailable)"
            ),
        }
    }

    // Initialize scheduler for background jobs (only if legacy pool is available)
    // Pass PgPool if available for legacy-to-PG sync job
    if let Some(ref pool) = legacy_pool {
        let pg_for_scheduler = final_app_state.as_ref().map(|s| s.new_pool.clone());
        // Ville canonical pool for the pure-PG stale-checkin tripwire (task #66) —
        // the backend is the only place both canonical pools are co-resident.
        let ville_pg_for_scheduler = final_app_state.as_ref().and_then(|s| s.ville_pool.clone());
        if let Err(e) = init_scheduler(
            pool.clone(),
            pg_for_scheduler,
            ville_pg_for_scheduler,
            config.slack.clone(),
            config.site.clone(),
        )
        .await
        {
            tracing::warn!("Failed to initialize scheduler: {}", e);
        }
    } else {
        tracing::info!("Scheduler disabled (legacy database unavailable)");
    }

    // Configure CORS — origins are locked to a curated allowlist read from
    // `BACKEND_ALLOWED_ORIGINS` (comma-separated, locked in v2.59.2).
    // Phase 7 audit M-3 (2026-05-10): tightened methods + headers from
    // wildcard `Any` to explicit lists so an origin-spoofed pre-flight
    // can't probe for arbitrary verbs/headers, and so credentialed
    // requests (cookie session) round-trip cleanly — `Any` is forbidden
    // by the CORS spec when `Access-Control-Allow-Credentials: true`.
    let cors = CorsLayer::new()
        .allow_origin(parse_allowed_origins())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    // Build all routes (use AppState for dual-database access).
    // User-facing legacy-read routes (rooms, bookings, customers, stats, etc.)
    // are PG-only as of Phase 8 — they read from `ht_*_legacy` mirror tables
    // populated by the CT mappers + drift-reconcile job. MSSQL is write-only
    // (writeback worker) for the legacy .NET app.
    let new_routes = if let Some(ref app_state) = final_app_state {
        build_new_routes(app_state.clone())
    } else {
        Router::new()
    };

    // Healthcheck (`/health`) — task #69 (site id) + task #78 (CT
    // watermark). Carries the site id so an operator / external monitor
    // can tell which deployment responded when HF Hotel + HF Ville
    // share a Slack webhook + log sink, AND the canonical PG pool so
    // the handler can include the current `legacy_ct_state` snapshot.
    // The PG pool is optional: legacy-only mode (no PG configured) is
    // still supported, and the handler renders `null` for the
    // watermark fields in that case.
    let health_pg_pool = final_app_state.as_ref().map(|s| s.new_pool.clone());
    let health_state = routes::health::HealthState {
        site_id: config.site.id.clone(),
        pg_pool: health_pg_pool,
    };
    let health_routes = Router::new()
        .route("/health", get(routes::health::health))
        .with_state(health_state);

    // Public installer downloads (`/api/downloads/middleware/{platform}`).
    // Stateless file serving from a bind-mounted assets dir — mounted PUBLIC
    // (like /health, outside `require_auth`) so reception can fetch the
    // card-reader middleware from our own origin without a GitHub login gate.
    let downloads_routes = routes::downloads::router();

    // Phase 4 PR2: mount the public `/api/auth/*` endpoints. These are
    // ALWAYS reachable — they're how unauthenticated callers acquire a
    // session, and `/api/auth/me` is how the frontend probes whether
    // auth is enabled at all (returns 401 + `{"error":"unauthenticated"}`
    // when no cookie is present, regardless of `AUTH_ENABLED`).
    //
    // Phase 7 audit M-2 (2026-05-10): the `/api/auth/login` route is
    // wrapped with the in-process per-IP rate limiter (10 attempts per
    // 15-minute sliding window). The limiter is mounted ONLY on login
    // — `/api/auth/me` and `/api/auth/logout` need to stay free of
    // throttling so a stuck client can always rotate its session
    // without first solving a 429. The limiter is wired here rather
    // than inside `routes::auth::router()` so the route module stays
    // free of cross-cutting concerns.
    let auth_routes = match &final_app_state {
        Some(state) => {
            let login_limiter = app_middleware::LoginRateLimitState::new();
            let login_layer = axum_middleware::from_fn_with_state(
                login_limiter,
                app_middleware::login_rate_limit,
            );
            Router::new()
                .route(
                    "/api/auth/login",
                    post(routes::auth::login).layer(login_layer),
                )
                .route("/api/auth/logout", post(routes::auth::logout))
                .route("/api/auth/me", get(routes::auth::me))
                // Cloudflare Access auto-login: verifies the CF edge
                // JWT (Cf-Access-Jwt-Assertion / CF_Authorization) and
                // mints a normal cookie session for the mapped user —
                // no second password login. NOT behind the login rate
                // limiter (verification is a signature check, not a
                // guessable credential); kill-switch CF_AUTO_LOGIN
                // (default on) makes it answer 404 when disabled.
                .route("/api/auth/cf-login", post(routes::auth::cf_login))
                // NFC staff-card session mint: consumes the one-time
                // login_token produced by GET /api/reader/wait (after it
                // verified the central HF-ID tap assertion) and mints the same cookie session
                // as password / CF login. PUBLIC — the login screen has no
                // session yet. NOT behind the login rate limiter (the token
                // is a 256-bit server-minted one-time value, not a guessable
                // credential).
                .route("/api/auth/card-login", post(routes::auth::card_login))
                .with_state(state.clone())
        }
        None => Router::new(),
    };

    // NFC staff-card login — PUBLIC reader endpoints (a login screen has no
    // session cookie). Mounted OUTSIDE `require_auth`, alongside /api/auth/*.
    // The reader now taps CENTRAL HF-ID, so there is no local `scan` route:
    // `claim` claims a pairing via HF-ID and mints the `reader_claim` cookie;
    // `wait` long-polls HF-ID, verifies the returned assertion, and hands back
    // the login_token. See `routes::reader`. The paired card-login mint is in
    // the auth block above.
    let reader_routes = match &final_app_state {
        Some(state) => Router::new()
            .route("/api/reader/claim", post(routes::reader::claim))
            .route("/api/reader/wait", get(routes::reader::wait))
            .with_state(state.clone()),
        None => Router::new(),
    };

    // Maid-facing housekeeping surface (`/api/hk/*`, employee-login plan
    // Phase 4). OWN edge gate instead of the cookie session: every request
    // must carry a `Cf-Access-Jwt-Assertion` for the /hk Cloudflare Access
    // application (env `CF_ACCESS_HK_AUD`; grant key `housekeeping`),
    // verified by `middleware::hk_access` which injects the maid's
    // `HkIdentity`. FAIL CLOSED — env unset ⇒ every request 401, so the
    // surface ships dark until the Access wiring lands. Mounted OUTSIDE
    // `require_auth` (maids have no PMS session) but WITH its own
    // `ville_write_guard` layer, exactly like the main router — with ONE
    // exemption: `POST /api/hk/rooms/{id}/cleaning` is admitted for
    // `branch=hfville` even while `HFVILLE_WRITES_ENABLED` is off, so the
    // admitted set matches `HFVILLE_WRITEBACK_INTENTS=mark_room_clean` and
    // canonical PG cannot diverge from Ville's iHOTEL (see
    // `middleware::ville_guard`).
    //
    // The whole stack is built by `routes::hk::router` so the integration
    // tests mount the shipped wiring rather than a replica.
    let hk_routes = match &final_app_state {
        Some(state) => routes::hk::router(state.clone()),
        None => Router::new(),
    };

    // Loyalty-app booking channel (`/api/channel/*` — docs/loyalty-channel.md).
    // Machine-to-machine surface for the loyalty app: availability quote,
    // tentative hold create, payment-verified confirm, release. Mounted
    // OUTSIDE `require_auth` (the caller is a service, not a session user)
    // behind its OWN shared-bearer gate, which fails closed: the whole
    // surface answers 503 until `LOYALTY_CHANNEL_ENABLED=true` AND
    // `LOYALTY_CHANNEL_TOKEN` are provisioned (ship-dark — a channel hold
    // writes back to iHOTEL as `จอง` via the normal booking-create recipe, so
    // the flag flip is a coordinated go-live step, invariant #6). HF Ville
    // channel mutations are additionally rejected until
    // `HFVILLE_WRITES_ENABLED` (enforced in `routes::channel` — this router
    // sits outside the main router's `ville_write_guard`, which keys on the
    // `?branch=` query param the channel API doesn't use).
    let channel_routes = match &final_app_state {
        Some(state) => {
            let channel_token_state = app_middleware::ChannelTokenState::new(&loyalty_config);
            Router::new()
                .route(
                    "/api/channel/availability",
                    get(routes::channel::availability),
                )
                .route(
                    "/api/channel/bookings",
                    post(routes::channel::create_booking),
                )
                .route(
                    "/api/channel/bookings/{pms_booking_id}/payment-verified",
                    post(routes::channel::payment_verified),
                )
                .route(
                    "/api/channel/bookings/{pms_booking_id}/release",
                    post(routes::channel::release),
                )
                .layer(axum_middleware::from_fn_with_state(
                    channel_token_state,
                    app_middleware::require_channel_token,
                ))
                .with_state(state.clone())
        }
        None => Router::new(),
    };

    // Phase 4 PR4: mount the protected `/api/admin/*` endpoints. The
    // subrouter is wrapped with the same `require_auth` middleware
    // PR2 added to the canonical routes so an authenticated `User` extension
    // is injected on every request. Each handler then performs an
    // explicit role check (admin vs receptionist) — see
    // `routes::admin_users::require_admin`.
    let admin_routes = match &final_app_state {
        Some(state) => {
            let auth_layer =
                axum_middleware::from_fn_with_state(state.clone(), app_middleware::require_auth);
            routes::admin_users::router()
                .with_state(state.clone())
                .layer(auth_layer)
        }
        None => Router::new(),
    };

    // Merge all routes. Public routers (auth, health) MUST be merged
    // alongside the protected `new_routes`, never inside it — the
    // `require_auth` middleware below is applied only to `new_routes`,
    // so anything mounted there gets gated when `AUTH_ENABLED=true`.
    let app = Router::new()
        .merge(new_routes)
        .merge(auth_routes)
        .merge(reader_routes)
        .merge(hk_routes)
        .merge(channel_routes)
        .merge(admin_routes)
        .merge(health_routes)
        .merge(downloads_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // Outermost: measures the full round trip, including the layers above.
        .layer(axum_middleware::from_fn(access_log));

    // Log database availability status
    match (legacy_available, new_available) {
        (true, true) => tracing::info!("Full dual-database mode: All features available"),
        (true, false) => tracing::warn!("Legacy-only mode: New features disabled"),
        (false, true) => {
            tracing::info!("New-only mode: Legacy routes disabled, using HotelNew database")
        }
        (false, false) => unreachable!(), // We return early if both are unavailable
    }

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server.addr()).await?;
    tracing::info!("Server running on http://{}", config.server.addr());

    axum::serve(listener, app).await?;

    Ok(())
}

/// SSE endpoint path, excluded from the access log below.
const SSE_PATH: &str = "/api/events";

/// Request-level access log for `/api/*`: method, path, status, latency — at
/// INFO, one line per request.
///
/// The backend had **no** request logging: `tower_http`'s `TraceLayer` emits
/// at DEBUG and the deployed filter is `hotel_backend=info,tower_http=info`,
/// so the 2026-07-29 500-burst was completely invisible and the diagnosis had
/// to be reconstructed from `pg_stat_activity` and the browser console. One
/// INFO line per request makes the next occurrence a grep.
///
/// A hand-rolled middleware rather than reconfiguring `TraceLayer` because
/// `DefaultMakeSpan` records the **full URI including the query string**, and
/// our query strings carry guest-identifying parameters (`?q=`, `?book_no=`,
/// customer ids). This logs `uri().path()` only — never a query string, never
/// a header, never a body, never a token.
///
/// `/api/events` is excluded outright: an SSE stream stays open for hours, so
/// its "latency" is a disconnect timestamp, not a service time — logging it
/// would bury the data-endpoint lines this exists to surface. Non-`/api`
/// paths (`/health` polls, installer downloads) are skipped as noise.
async fn access_log(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = req.uri().path().to_string();
    if !path.starts_with("/api/") || path == SSE_PATH {
        return next.run(req).await;
    }

    let method = req.method().clone();
    let started = std::time::Instant::now();
    let response = next.run(req).await;

    tracing::info!(
        method = %method,
        path = %path,
        status = response.status().as_u16(),
        latency_ms = started.elapsed().as_millis() as u64,
        "api request",
    );

    response
}

/// Default CORS allowlist when `BACKEND_ALLOWED_ORIGINS` is unset.
/// Covers the Next.js dev server on the host (3003) + the in-container
/// `web` service. Production deployments should set the env var with the
/// public hostname(s) instead of relying on the default.
const DEFAULT_ALLOWED_ORIGINS: &str = "http://localhost:3003,http://web:3003";

/// Parse the CORS allowlist from `BACKEND_ALLOWED_ORIGINS` (comma-separated)
/// or fall back to `DEFAULT_ALLOWED_ORIGINS`. Empty entries are skipped
/// silently so trailing commas don't break startup. Malformed origins
/// panic loudly — config errors should fail at startup, not at request
/// time (same loud-failure stance as `config::require_secret`).
fn parse_allowed_origins() -> AllowOrigin {
    let raw = std::env::var("BACKEND_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| DEFAULT_ALLOWED_ORIGINS.to_string());

    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            HeaderValue::from_str(origin).unwrap_or_else(|err| {
                panic!(
                    "BACKEND_ALLOWED_ORIGINS contains a malformed origin {:?}: {}. \
                     Expected comma-separated absolute origins (e.g. \
                     'https://hotel.example.com,http://web:3003').",
                    origin, err
                )
            })
        })
        .collect();

    if origins.is_empty() {
        panic!(
            "BACKEND_ALLOWED_ORIGINS is set but resolved to zero origins after \
             trimming. Refusing to start with an empty CORS allowlist — set the \
             env var to a comma-separated list of absolute origins, or unset it \
             to use the default ({}).",
            DEFAULT_ALLOWED_ORIGINS
        );
    }

    tracing::info!(
        "CORS allowlist: {}",
        origins
            .iter()
            .filter_map(|value| value.to_str().ok())
            .collect::<Vec<_>>()
            .join(", ")
    );

    AllowOrigin::list(origins)
}

/// Build the new routes router with AppState.
///
/// Phase 4 PR2: the entire returned router is wrapped with the
/// `require_auth` middleware. The middleware itself short-circuits to
/// a no-op pass-through when `AppState::auth_enabled` is `false`
/// (the production default), so there is no per-request cost until an
/// operator opts in. The middleware is applied to THIS subrouter only
/// — `/api/auth/*` and `/health` are mounted separately in `main()`
/// and stay public.
fn build_new_routes(app_state: AppState) -> Router {
    let auth_layer =
        axum_middleware::from_fn_with_state(app_state.clone(), app_middleware::require_auth);

    // ADR 0002: reject HF Ville mutations while the new app isn't a Ville writer.
    // Default-off safety net so a `branch=hfville` write can never reach a handler
    // (and thus never misroute into the HF Hotel pool). No-op once the flag is on.
    let ville_write_guard_layer =
        axum_middleware::from_fn_with_state(app_state.clone(), ville_write_guard);

    // Track G7 — permission gates layered on top of `require_auth`. Each
    // gate is a no-op when `AUTH_ENABLED=false`, so this stays free until
    // an operator flips the flag. Mounted via `route_layer` on the
    // individual routes that need them so we don't have to split this
    // router into per-permission subrouters.
    let perm_payment_refund =
        app_middleware::require_permission("payment.refund", app_state.clone());
    let perm_inventory_consume_check =
        app_middleware::require_permission("inventory.consume", app_state.clone());
    let perm_inventory_consume_replenish =
        app_middleware::require_permission("inventory.consume", app_state.clone());
    let perm_inventory_consume_adjustments =
        app_middleware::require_permission("inventory.consume", app_state.clone());
    // Track G5 — coupon issuing.
    let perm_coupon_issue = app_middleware::require_permission("coupon.issue", app_state.clone());
    let perm_coupon_redeem = app_middleware::require_permission("coupon.redeem", app_state.clone());
    // Track G6 — POS / sales-to-room route gate. Admin + cashier
    // hold the grant (migration 052 seeds the role grid). Each route gets
    // its own layer instance because `route_layer` consumes the layer by
    // value (Task #45 added the walk-up sale + void routes — same grant).
    let perm_pos_sell = app_middleware::require_permission("pos.sell", app_state.clone());
    let perm_pos_sell_walkup = app_middleware::require_permission("pos.sell", app_state.clone());
    let perm_pos_sell_void = app_middleware::require_permission("pos.sell", app_state.clone());

    Router::new()
        // ── One /api/* suite (no "/new" prefix anywhere) ──
        // Canonical CRUD on /api/rooms, /api/checkins, /api/bookings,
        // /api/customers/search; a few branch-aware legacy reads remain on
        // distinct paths (/api/checkins/board, /api/bookings/by-number).
        //
        // /api/rooms now derives live room status server-side (occupancy/booking/
        // checkout/maintenance) — the legacy /api/rooms/board +
        // /api/rooms/checkouts-today were removed 2026-06-27 once both the
        // dashboard and the v2 grid consumed the canonical status. See
        // docs/spikes/2026-06-27-frontend-backend-encapsulation.md.
        .route(
            "/api/bookings/by-number/{book_no}",
            get(routes::bookings::get_booking),
        )
        .route(
            "/api/bookings/by-number/{book_no}/notes",
            get(routes::bookings::get_notes)
                .post(routes::bookings::create_note)
                .delete(routes::bookings::delete_note),
        )
        .route("/api/checkins/board", get(routes::checkins::list_checkins))
        .route(
            "/api/customers",
            get(routes::customers::list_customers).post(routes::new_customers::create_customer),
        )
        // Canonical single-customer CRUD — branch-aware (`?branch=`). GET
        // returns the full editable record for the edit form; PUT delegates to
        // the customer service (canonical UPDATE + legacy re-save enqueue);
        // DELETE soft-deletes (`cust_active=false`, no legacy writeback by
        // design). HF Ville mutations stay gated by `ville_write_guard`.
        .route(
            "/api/customers/{id}",
            get(routes::new_customers::get_customer)
                .put(routes::new_customers::update_customer)
                .delete(routes::new_customers::delete_customer),
        )
        .route(
            "/api/customers/{id}/bookings",
            get(routes::customers::get_customer_bookings),
        )
        .route(
            "/api/customers/{id}/stats",
            get(routes::customers::get_customer_stats),
        )
        // Loyalty membership link (migration 086) — desk set/clear. A
        // dedicated PUT (not a field on the general customer update) so a
        // stale edit form can never clobber a freshly-scanned link, and so
        // clearing is expressible. PG-canonical only, no legacy writeback.
        .route(
            "/api/customers/{id}/membership",
            put(routes::new_customers::set_membership),
        )
        // Branch-aware stats (HF Hotel + HF Ville) — the sole stats path.
        .route("/api/stats", get(routes::stats::get_stats))
        // Mode and calendar routes
        .route("/api/mode", get(routes::mode::get_mode))
        // Active HF Ville connectivity probe (pings the ville pool with SELECT 1)
        // so post-startup outages are detectable — see scripts/smoke-ville.sh.
        .route("/api/health/ville", get(routes::mode::get_ville_health))
        .route("/api/calendar", get(routes::calendar::get_calendar))
        // Booking reminders (task #53 / migration 064) — feed for the in-shell
        // notification bell: upcoming arrivals + balance-due bookings, branch-
        // aware. Dismiss suppresses a booking's reminder (PG-canonical only, no
        // legacy writeback). Static `/reminders` is matched ahead of any param.
        .route(
            "/api/calendar/reminders",
            get(routes::new_calendar::list_reminders),
        )
        .route(
            "/api/calendar/reminders/{id}/dismiss",
            post(routes::new_calendar::dismiss_reminder),
        )
        // Phase 5.5d — legacy_mirror.* read-only endpoints (coupons,
        // minibar, room moves). Surfaces legacy-only features so
        // receptionists don't switch to the .NET app.
        .route(
            "/api/legacy-mirror/coupons",
            get(routes::legacy_mirror::list_coupons),
        )
        .route(
            "/api/legacy-mirror/products",
            get(routes::legacy_mirror::list_products),
        )
        .route(
            "/api/legacy-mirror/room-changes",
            get(routes::legacy_mirror::list_room_changes),
        )
        // Task #51 — consolidated read-only pricing reference (HT_Rooms_Price /
        // HT_ContinueTime / HT_Order_Up / HT_Order_Down) for the pricing page.
        .route(
            "/api/legacy-mirror/pricing",
            get(routes::legacy_mirror::get_pricing_reference),
        )
        // Customers canonical search (create is a POST on /api/customers above).
        .route(
            "/api/customers/search",
            get(routes::new_customers::list_customers),
        )
        // Rooms CRUD (canonical)
        .route(
            "/api/rooms",
            get(routes::new_rooms::list_rooms).post(routes::new_rooms::create_room),
        )
        // Layout-edit mode (จัดผัง, #236) — one drop = canonical room_x/room_y
        // UPDATE + MoveRoomTiles writeback (shared board with iHOTEL
        // FormRoomMain). 409s while LAYOUT_WRITEBACK_ENABLED is off (ship-dark).
        // Static segment, so it never shadows the `{id}` capture below.
        .route(
            "/api/rooms/layout",
            put(routes::new_rooms::update_room_layout),
        )
        .route(
            "/api/rooms/{id}",
            get(routes::new_rooms::get_room).put(routes::new_rooms::update_room),
        )
        .route(
            "/api/rooms/{id}/status",
            patch(routes::new_rooms::update_room_status),
        )
        // Housekeeping — clean / dirty / maintenance toggles. Route the
        // previously-unwired HousekeepingService so "mark clean" / "mark dirty"
        // actually flip canonical `room_clean` AND mirror to legacy HT_Rooms +
        // HT_Housewife (was a no-op via PATCH /status), and the out-of-service
        // toggle mirrors to HT_Rooms.Room_Manternace so iHOTEL stops renting
        // the room. Branch-aware via `?branch=`; HF Ville stays gated by the
        // ville_write_guard layer below until HFVILLE_WRITES_ENABLED is on.
        .route(
            "/api/housekeeping/rooms/{id}/clean",
            post(routes::housekeeping::mark_clean),
        )
        .route(
            "/api/housekeeping/rooms/{id}/dirty",
            post(routes::housekeeping::mark_dirty),
        )
        .route(
            "/api/housekeeping/rooms/{id}/maintenance",
            post(routes::housekeeping::set_maintenance),
        )
        // Bookings CRUD (canonical)
        .route(
            "/api/bookings",
            get(routes::new_bookings::list_bookings).post(routes::new_bookings::create_booking),
        )
        // Spike Phase 3: read-only server-side booking date + availability check
        // (the form surfaces this verdict when BOOKING_VALIDATION_ENABLED is on).
        // Static `/validate` is matched ahead of the `/{id}` param route.
        .route(
            "/api/bookings/validate",
            post(routes::new_bookings::validate_booking),
        )
        .route(
            "/api/bookings/{id}",
            get(routes::new_bookings::get_booking).put(routes::new_bookings::update_booking),
        )
        .route(
            "/api/bookings/{id}/cancel",
            put(routes::new_bookings::cancel_booking),
        )
        // Check-ins CRUD (canonical)
        .route(
            "/api/checkins",
            get(routes::new_checkins::list_checkins).post(routes::new_checkins::create_checkin),
        )
        .route(
            "/api/checkins/{id}/checkout",
            put(routes::new_checkins::checkout),
        )
        // Spike Phase 2: read-only server-authoritative checkout total (the UI
        // displays this instead of computing nights × rate client-side when
        // CHECKOUT_SERVER_TOTAL_ENABLED is on).
        .route(
            "/api/checkins/{id}/checkout-quote",
            get(routes::new_checkins::checkout_quote),
        )
        // #75 — per-room (partial) checkout picker: all rooms of a multi-room stay.
        .route(
            "/api/checkins/{id}/rooms",
            get(routes::new_checkins::list_checkin_rooms),
        )
        // Track G1 / T4 HIGH-2: extend an active stay (one-more-night flow).
        .route(
            "/api/checkins/{id}/extend",
            put(routes::new_checkins::extend),
        )
        // Task #54 item 3 / Track G2: general checkout date-edit (shorten OR
        // extend) — reuses the ExtendStay canonical-update + writeback path.
        .route(
            "/api/checkins/{id}/change-dates",
            put(routes::new_checkins::change_dates),
        )
        // Track G4 / T4 HIGH-3: mid-stay room change (move guest A → B).
        .route(
            "/api/checkins/{id}/change-room",
            post(routes::new_checkins::change_room),
        )
        // Task #54 item 5: printable room-change receipt for a folio.
        .route(
            "/api/checkins/{id}/room-change-receipt",
            get(routes::new_checkins::room_change_receipt),
        )
        // #29: printable registration slip (ใบลงทะเบียนเข้าพัก) reprint for any
        // check-in — iHOTEL FrmRegMain equivalent. Branch-aware read.
        .route(
            "/api/checkins/{id}/registration-slip",
            get(routes::new_checkins::registration_slip),
        )
        // Task #49 — deposit refund (คืนเงินมัดจำ / iHOTEL FormShowDEPBack).
        // GET lists per-room deposit lines (backs the folio refund button);
        // POST marks a room's deposit refunded. Both branch-aware.
        .route(
            "/api/checkins/{id}/deposits",
            get(routes::new_checkins::list_deposits),
        )
        .route(
            "/api/checkins/{id}/deposit-refund",
            post(routes::new_checkins::deposit_refund),
        )
        // Track G6 — POS / sales-to-room (MVP). Ring up a product line
        // against an active folio. Gated on `pos.sell` (cashier + admin).
        .route(
            "/api/checkins/{id}/pos-sale",
            post(routes::new_checkins::create_pos_sale).route_layer(perm_pos_sell),
        )
        // GET stays ungated — receptionists need to see the running tab
        // even when they can't ring up new sales.
        .route(
            "/api/checkins/{id}/pos-sales",
            get(routes::new_checkins::list_pos_sales),
        )
        // Task #45 — POS walk-up (roomless) sale → standalone receipt.
        // Same `pos.sell` grant as folio POS; branch-aware via ?branch=.
        .route(
            "/api/new/pos/walkup-sale",
            post(routes::new_pos::create_walkup_sale).route_layer(perm_pos_sell_walkup),
        )
        // Task #45 — void a folio POS line (canonical + legacy reversal).
        .route(
            "/api/new/pos/sales/{id}/void",
            post(routes::new_pos::void_sale).route_layer(perm_pos_sell_void),
        )
        // Guest registry
        .route(
            "/api/checkins/{id}/guests",
            get(routes::new_checkins::list_guests).post(routes::new_checkins::create_guest),
        )
        .route(
            "/api/checkins/{id}/guests/{guest_id}",
            delete(routes::new_checkins::delete_guest),
        )
        // Guest documents (check-in registration Phase 2 — ht_guest_documents;
        // legacy Tb_Save_Image mirror SHIPPED DARK behind GUEST_DOCUMENT_STORAGE_ENABLED)
        .route(
            "/api/guest-documents",
            post(routes::guest_documents::create_guest_document),
        )
        // Server-side Thai national-ID card compositing (render/thai_id_card.rs):
        // reconstruct the iHOTEL ReportReg card face from a chip capture + store
        // the PNG canonically. Degrades to {success:false} when the template/font
        // asset is absent (frontend then uploads the raw face photo). Static
        // `render-thai-id` is matched ahead of the `{doc_id}` param below.
        .route(
            "/api/guest-documents/render-thai-id",
            post(routes::guest_documents::render_thai_id_card),
        )
        .route(
            "/api/guest-documents/{doc_id}",
            get(routes::guest_documents::get_guest_document),
        )
        .route(
            "/api/checkins/{id}/documents",
            get(routes::guest_documents::list_checkin_documents),
        )
        // Invoice
        .route(
            "/api/checkins/{id}/invoice",
            get(routes::new_invoice::get_invoice),
        )
        // Payments
        .route(
            "/api/checkins/{id}/payments",
            get(routes::new_payments::list_payments).post(routes::new_payments::create_payment),
        )
        // ── Single /api/* suite (no "/new" prefix) for non-colliding families ──
        // Room types CRUD
        .route(
            "/api/room-types",
            get(routes::new_room_types::list_room_types)
                .post(routes::new_room_types::create_room_type),
        )
        .route(
            "/api/room-types/{id}",
            get(routes::new_room_types::get_room_type)
                .put(routes::new_room_types::update_room_type)
                .delete(routes::new_room_types::delete_room_type),
        )
        // Rates CRUD (DEPRECATED ht_rates write path — kept for back-compat).
        .route(
            "/api/rates",
            get(routes::new_rates::list_rates).post(routes::new_rates::create_rate),
        )
        .route(
            "/api/rates/{id}",
            get(routes::new_rates::get_rate)
                .put(routes::new_rates::update_rate)
                .delete(routes::new_rates::delete_rate),
        )
        // Task #51 — canonical (Room_Type, Cust_Type) pricing matrix on
        // ht_rate_tiers (the live mirror of legacy HT_Rooms_Price). Writes
        // enqueue a HT_Rooms_Price writeback. Branch-aware (HF Ville mutations
        // gated by ville_write_guard).
        .route(
            "/api/rate-tiers",
            get(routes::new_rates::list_rate_tiers).post(routes::new_rates::create_rate_tier),
        )
        .route(
            "/api/rate-tiers/{id}",
            put(routes::new_rates::update_rate_tier),
        )
        // Reports
        .route(
            "/api/reports/revenue",
            get(routes::new_reports::get_revenue),
        )
        .route(
            "/api/reports/occupancy",
            get(routes::new_reports::get_occupancy),
        )
        .route(
            "/api/reports/revenue-by-room-type",
            get(routes::new_reports::get_revenue_by_room_type),
        )
        // Financial reports (task #55) — output-tax (VAT) period summary +
        // revenue grouped by customer. Read-only, branch-aware.
        .route(
            "/api/reports/vat-summary",
            get(routes::new_reports::get_vat_summary),
        )
        .route(
            "/api/reports/sales-by-customer",
            get(routes::new_reports::get_sales_by_customer),
        )
        // Track G8 — RR.4 Thai immigration foreign-guest export (legal CRIT)
        .route("/api/reports/rr4", get(routes::rr4_export::get_rr4_export))
        // Daily guest rosters / desk paperwork (task #43). FULL lists (no silent
        // cap, unlike the limit=100 dashboard glances), branch-aware. Read-only.
        .route(
            "/api/rosters/summary",
            get(routes::new_rosters::roster_summary),
        )
        .route(
            "/api/rosters/in-house",
            get(routes::new_rosters::in_house_roster),
        )
        .route(
            "/api/rosters/arrivals",
            get(routes::new_rosters::arrivals_roster),
        )
        .route(
            "/api/rosters/departures",
            get(routes::new_rosters::departures_roster),
        )
        .route(
            "/api/rosters/continuing",
            get(routes::new_rosters::continuing_roster),
        )
        // Track G2 / T4 CRIT-1 — refund (negative payment) against an existing payment row.
        // Track G7 gates this on `payment.refund` (cashier + admin only).
        .route(
            "/api/payments/{id}/refund",
            post(routes::new_payments::refund_payment).route_layer(perm_payment_refund),
        )
        // Track G5 — canonical coupon issuing (HT_Cupon mirror).
        // `coupon.issue` gates admin + receptionist (migration 051).
        .route(
            "/api/coupons",
            post(routes::new_coupons::issue_coupon).route_layer(perm_coupon_issue),
        )
        // Mark a coupon redeemed/printed. `coupon.redeem` gates admin +
        // cashier + receptionist (migration 051).
        .route(
            "/api/coupons/{code}/redeem",
            post(routes::new_coupons::redeem_coupon).route_layer(perm_coupon_redeem),
        )
        // Shifts (Track F2 / T1 HIGH-5 — cashier-shift gate for payments)
        .route(
            "/api/shifts/current",
            get(routes::new_shifts::current_shift),
        )
        // Track J6 — open/close a cashier round (mirrored to iHOTEL's
        // HT_Round_Bill via the writeback worker). Mounted unconditionally;
        // the `ShiftService.round_writeback` flag (from ROUND_WRITEBACK_ENABLED)
        // gates behaviour — both reject when off (ServiceError::Conflict → HTTP
        // 400, see error.rs), so iHOTEL stays the sole round-opener until an
        // operator flips the flag. `branch=hfville`
        // open/close is additionally blocked by the Ship-A ville_write_guard
        // until HF Ville gets its own write bundle (Ship B).
        .route("/api/shifts/open", post(routes::new_shifts::open_shift))
        .route("/api/shifts/close", post(routes::new_shifts::close_shift))
        // Track J7b — round reconciliation report (income by tender + sales
        // summary) from canonical ht_payment_ledger. Read-only; branch-aware.
        .route(
            "/api/shifts/{shift_id}/report",
            get(routes::new_shifts::round_report),
        )
        // Track J7f — round summary/analytics: closed rounds over a date range
        // (iHOTEL FrmDueBill history) + period totals + by-cashier. Read-only.
        .route(
            "/api/shifts/summary",
            get(routes::new_shifts::round_summary),
        )
        // Cash in/out petty-cash ledger (รายรับ-รายจ่าย) — migration 059.
        // Branch-aware list + create (income/expense) + the account taxonomy
        // dropdown source. Inbound mirror of legacy TB_Pay_History rides the
        // sync poll (bin/sync.rs::sync_cash_history); writeback to TB_Pay_History
        // is pending byte-shape verification (writeback::recipes::cash_entry).
        .route("/api/cash", get(routes::new_cash::list_cash))
        .route("/api/cash/income", post(routes::new_cash::create_income))
        .route("/api/cash/expense", post(routes::new_cash::create_expense))
        .route(
            "/api/cash/categories",
            get(routes::new_cash::list_categories),
        )
        // Room & staff sticky notes (โน้ตห้อง / โน้ตพนักงาน) — task #47 /
        // migration 062. Branch-aware list + add + mark-read. Inbound mirror of
        // legacy HT_Room_SMS / HT_EMP_SMS rides the sync poll
        // (bin/sync.rs::sync_sticky_notes); writeback to the SMS tables is
        // SHIPPED DARK behind NOTES_WRITEBACK_ENABLED (writeback::recipes::sticky_note).
        .route(
            "/api/notes",
            get(routes::new_notes::list_notes).post(routes::new_notes::create_note),
        )
        .route("/api/notes/{id}/read", post(routes::new_notes::mark_read))
        // In-app reception verification form (คู่มือตรวจสอบระบบใหม่) — task #58 /
        // migration 066. Online equivalent of
        // docs/coexistence/reception-verification-TH.html. Branch-aware submit +
        // recent-list for IT review. PG-CANONICAL ONLY (no legacy counterpart, no
        // sync, no writeback); answers stored in the vr_answers JSONB column.
        .route(
            "/api/verification",
            get(routes::new_verification::list_verifications)
                .post(routes::new_verification::create_verification),
        )
        // Data-driven feedback/re-verification form definitions (Tier 1 / migration
        // 067). Generic renderer fetches a form's schema from here, so question
        // edits are a DB write — no frontend rebuild. Read-only; answers still POST
        // to /api/verification. PG-CANONICAL ONLY (primary pool).
        .route(
            "/api/feedback/forms",
            get(routes::new_feedback::list_feedback_forms),
        )
        .route(
            "/api/feedback/forms/{key}",
            get(routes::new_feedback::get_feedback_form),
        )
        // Inventory Management
        .route(
            "/api/inventory/items",
            get(routes::new_inventory::list_items).post(routes::new_inventory::create_item),
        )
        .route(
            "/api/inventory/items/{id}",
            get(routes::new_inventory::get_item)
                .put(routes::new_inventory::update_item)
                .delete(routes::new_inventory::delete_item),
        )
        .route(
            "/api/inventory/rooms",
            get(routes::new_inventory::list_inventory_rooms),
        )
        .route(
            "/api/inventory/rooms/{room_id}",
            get(routes::new_inventory::get_room_inventory)
                .put(routes::new_inventory::update_room_inventory),
        )
        // Track G7 gates housekeeping/POS stock mutations on `inventory.consume`.
        .route(
            "/api/inventory/rooms/{room_id}/check",
            axum::routing::post(routes::new_inventory::check_room_inventory)
                .route_layer(perm_inventory_consume_check),
        )
        .route(
            "/api/inventory/rooms/{room_id}/replenish",
            axum::routing::post(routes::new_inventory::replenish_room_inventory)
                .route_layer(perm_inventory_consume_replenish),
        )
        .route(
            "/api/inventory/adjustments",
            axum::routing::post(routes::new_inventory::create_stock_adjustment)
                .route_layer(perm_inventory_consume_adjustments),
        )
        .route(
            "/api/inventory/transactions",
            get(routes::new_inventory::list_transactions),
        )
        .route(
            "/api/inventory/stats",
            get(routes::new_inventory::get_stats),
        )
        // Products (Track F3 — `audit-2026-05-13.md` T1 CRIT-3; Task #51
        // added master CRUD + detail + the stock-adjust route mount).
        .route(
            "/api/products",
            get(routes::new_products::list_products).post(routes::new_products::create_product),
        )
        .route(
            "/api/products/{id}",
            get(routes::new_products::get_product).put(routes::new_products::update_product),
        )
        .route(
            "/api/products/{id}/stock-adjust",
            post(routes::new_products::adjust_stock),
        )
        // Maintenance Management
        .route(
            "/api/maintenance/categories",
            get(routes::new_maintenance::list_categories),
        )
        .route(
            "/api/maintenance/requests",
            get(routes::new_maintenance::list_requests)
                .post(routes::new_maintenance::create_request),
        )
        .route(
            "/api/maintenance/requests/{id}",
            get(routes::new_maintenance::get_request).put(routes::new_maintenance::update_request),
        )
        .route(
            "/api/maintenance/requests/{id}/status",
            put(routes::new_maintenance::update_request_status),
        )
        // Sync status
        .route("/api/sync/status", get(routes::new_sync::get_sync_status))
        // Real-time domain-event stream (Phase 4a per architecture.md §3.6e).
        // Long-lived SSE connection, but POOL-FREE after auth: it subscribes
        // to the per-site broadcast fan-out fed by the startup listener tasks
        // above (was one PgListener — i.e. one pool slot — per client until
        // the 2026-07-29 exhaustion incident).
        .route("/api/events", get(routes::events::stream))
        .with_state(app_state)
        // Phase 4 PR2: gate every route above behind the cookie-session
        // auth middleware. The middleware itself is a no-op when
        // `AUTH_ENABLED=false` (the production default), so this is
        // free until an operator opts in. Applied AFTER `with_state`
        // so the inner routes already have their state attached when
        // the layer wraps them.
        .layer(auth_layer)
        // Outermost: runs before auth so a disabled-Ville mutation is rejected
        // up front. Inspects only method + query string (never the body).
        .layer(ville_write_guard_layer)
}

// `ville_write_guard` now lives in the lib
// (`hotel_backend::middleware::ville_guard`) so its decision matrix is
// unit-testable and integration tests can mount the real layered router.
use hotel_backend::middleware::ville_guard::ville_write_guard;
