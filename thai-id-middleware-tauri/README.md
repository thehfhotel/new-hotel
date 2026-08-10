# Thai ID Middleware (Tauri)

Local helper app that exposes a Thai national ID card reader (PC/SC) over a
small HTTP API on `127.0.0.1:9898`. The hotel frontend calls
`GET /read` to pull card data when a guest checks in.

## Endpoints

| Method | Path              | Description                                           |
| ------ | ----------------- | ------------------------------------------------------ |
| GET    | `/health`         | Server + reader status                                 |
| GET    | `/status`         | Alias for `/health`                                    |
| GET    | `/read`           | Read card data (`?photo=true` optional)                |
| POST   | `/parse-mrz`      | Parse a TD3 passport MRZ string                        |
| GET    | `/debug`          | Full debug info (ATR, protocol, AID)                   |
| GET    | `/debug/enable`   | Enable verbose debug logging                           |
| GET    | `/debug/disable`  | Disable verbose debug logging                          |
| POST   | `/notify`         | Feed a `legacy_stale` signal into the staleness latch  |
| GET    | `/ihotel/status`  | iHOTEL process/window snapshot + latch state           |
| POST   | `/ihotel/refresh` | Trigger iHOTEL's refresh (`501` until Stage 4)         |

The server binds to `127.0.0.1` only, so it is unreachable from the
network.

### Legacy-stale notification (ADR 0006)

`POST /notify`, `GET /ihotel/status`, and `POST /ihotel/refresh` implement
the reception-facing half of
[`docs/adr/0006-legacy-stale-notification.md`](../docs/adr/0006-legacy-stale-notification.md):
when a writeback commits into the shared legacy MSSQL at a moment iHOTEL's
own room grid can't pick up on its own, the reception browser tab relays a
`legacy_stale` signal here, which feeds a one-toast-per-episode latch
(`src/stale.rs`) and — on Windows — raises a native toast
(`src/toast.rs`, via `tauri-winrt-notification`). The tray icon also gets
two menu items, "รีเฟรช iHOTEL" and "สถานะ iHOTEL", wired to the same code
paths as the HTTP endpoints (`src/ihotel/mod.rs`).

`POST /ihotel/refresh` and iHOTEL process/window probing
(`src/ihotel/mod.rs`) are intentionally stubbed in this build — they always
report unavailable / not-found. A later stage fills in the real Windows UI
Automation probing and refresh action; this build ships the notification
half only, per the ADR's decision to notify rather than automate.

`POST /notify` requires `Content-Type: application/json` — this isn't
incidental. A "simple" cross-origin POST (any other content type, or none)
never triggers the browser's CORS preflight, so the allowlist below would
never even be consulted for it.

### Autostart

The middleware registers `tauri-plugin-autostart` and enables it by
default on first launch (idempotent — it won't rewrite the OS autostart
entry on every subsequent launch). This exists because nothing else starts
the tray app on a reception PC today, and the `legacy_stale` notification
path depends on it already running before a browser tab needs to relay a
signal to it.

## CORS allowlist (`CARD_READER_ALLOWED_ORIGINS`)

Even with a 127.0.0.1 bind, any web page the receptionist's browser
loads can issue `fetch('http://localhost:9898/read')` and exfiltrate the
card-on-reader unless CORS is locked down. To prevent that, the server
restricts cross-origin requests to a curated allowlist sourced from the
environment.

- **Env var:** `CARD_READER_ALLOWED_ORIGINS`
- **Format:** comma-separated absolute origins
  (e.g. `https://hotel.example.com,http://web:3003`)
- **Default (when unset):** `https://hotel.thehfhotel.org,http://localhost:3003,http://web:3003`
  — covers the production frontend plus the Next.js dev server on the host
  and the in-container `web` service, so a stock reception install reads
  cards / relays `legacy_stale` signals from prod with no per-PC env config.
- **Allowed methods:** `GET`, `POST`, `OPTIONS`
- **Allowed headers:** `Content-Type`
- **Credentials:** disabled — no cookies cross the boundary.

### Production deployments

Production deployments **MUST** set `CARD_READER_ALLOWED_ORIGINS`
explicitly to include the public hostname of the frontend that should
be allowed to read cards. For example:

```
CARD_READER_ALLOWED_ORIGINS=https://hotel.example.com
```

A misconfigured value (malformed origin, or a non-empty env var that
trims to zero entries) will panic the process at startup rather than
silently fall back to a permissive policy.

## Build

```
cd src-tauri
cargo check     # fast type check
cargo build     # full build (pulls webkit/wry on first run)
```
