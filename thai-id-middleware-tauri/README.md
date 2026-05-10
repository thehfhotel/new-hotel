# Thai ID Middleware (Tauri)

Local helper app that exposes a Thai national ID card reader (PC/SC) over a
small HTTP API on `127.0.0.1:9898`. The hotel frontend calls
`GET /read` to pull card data when a guest checks in.

## Endpoints

| Method | Path             | Description                              |
| ------ | ---------------- | ---------------------------------------- |
| GET    | `/health`        | Server + reader status                   |
| GET    | `/status`        | Alias for `/health`                      |
| GET    | `/read`          | Read card data (`?photo=true` optional)  |
| GET    | `/debug`         | Full debug info (ATR, protocol, AID)    |
| GET    | `/debug/enable`  | Enable verbose debug logging             |
| GET    | `/debug/disable` | Disable verbose debug logging            |

The server binds to `127.0.0.1` only, so it is unreachable from the
network.

## CORS allowlist (`CARD_READER_ALLOWED_ORIGINS`)

Even with a 127.0.0.1 bind, any web page the receptionist's browser
loads can issue `fetch('http://localhost:9898/read')` and exfiltrate the
card-on-reader unless CORS is locked down. To prevent that, the server
restricts cross-origin requests to a curated allowlist sourced from the
environment.

- **Env var:** `CARD_READER_ALLOWED_ORIGINS`
- **Format:** comma-separated absolute origins
  (e.g. `https://hotel.example.com,http://web:3003`)
- **Default (when unset):** `http://localhost:3003,http://web:3003`
  — covers the Next.js dev server on the host plus the in-container
  `web` service. Same default as the backend's `BACKEND_ALLOWED_ORIGINS`.
- **Allowed methods:** `GET`, `OPTIONS`
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
