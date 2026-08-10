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
| POST   | `/ihotel/refresh` | Post a click to iHOTEL's Refresh button (guarded)      |

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

#### The refresh action (`POST /ihotel/refresh`)

Reception-invoked only — a click on the tray item or on the endpoint. **No
writeback signal can reach it**, per ADR 0006 Decision §1.

It locates iHOTEL's own Refresh button (`ButtonX3` on `FormRoomMain`) with
UI Automation, then posts a `WM_LBUTTONDOWN`/`WM_LBUTTONUP` pair to that
button's HWND. UIA to locate, because WinForms maps a control's `Name` onto
the UIA `AutomationId`, which survives relayout; Win32 to act, because
`ButtonX` derives from `Control` rather than `ButtonBase`, so `BM_CLICK`
would be silently swallowed. Never `SendMessage` (blocks forever on a
wedged app), never `SendInput`/`SetForegroundWindow` (would hijack the
receptionist's cursor and focus).

A refresh is not free — `LoadRooms` can `UPDATE HT_Rooms`/`HT_Book_Date`
and toggle room power, and `ButtonX3_Click` calls `ClearCheck()`, which
destroys any in-progress multi-room selection
([`docs/legacy-app/ROOM_GRID_REFRESH.md`](../docs/legacy-app/ROOM_GRID_REFRESH.md)
§4-5). So a guarded skip is always preferred over a risky click. The
endpoint answers `200` either way; `sent` is the discriminator, and a skip
carries a stable `reason` plus the Thai `message` reception just saw on a
toast:

| `reason`            | Meaning                                                     |
| ------------------- | ----------------------------------------------------------- |
| `process-not-found` | No `HOTEL.exe` running                                      |
| `modal-open`        | iHOTEL's top-level shell is disabled — a `ShowDialog` is up  |
| `selection-pending` | `ButtonX6` is visible: a multi-room selection is in flight   |
| `grid-not-found`    | `ButtonX3` could not be resolved — the room grid isn't open  |
| `probe-timeout`     | The Windows probe exceeded its deadline (wedged iHOTEL)      |
| `post-failed`       | Guards passed but `PostMessage` itself failed               |

The staleness latch clears **only** on `sent: true` — a skip means the grid
was never refreshed, so the episode is still open.

All guard policy lives in a pure `ihotel::decide(IhotelSnapshot)` that
makes no OS calls, so the ordering above is unit-tested on the macOS CI leg;
the Windows half only fills the snapshot in and executes the decision.

Off Windows every probe honestly reports "not found", so `POST
/ihotel/refresh` answers `{"sent": false, "reason": "process-not-found"}`.

##### Overriding the targets

| Env var                      | Default      | Purpose                                     |
| ---------------------------- | ------------ | ------------------------------------------- |
| `IHOTEL_PROCESS_NAME`        | `HOTEL.exe`  | Process image name to look for              |
| `IHOTEL_REFRESH_BUTTON_ID`   | `ButtonX3`   | AutomationId of the Refresh button          |
| `IHOTEL_SELECTION_BUTTON_ID` | `ButtonX6`   | AutomationId of the selection-pending button |

iHOTEL is a third-party binary; a designer-renamed control should be a
five-second config edit, not a build-sign-redeploy across every reception
PC. The effective values are logged once at startup — read that line first
when diagnosing a `grid-not-found`.

`scripts/dev/ihotel-uia-harness.ps1` is a throwaway WinForms harness with
controls named `ButtonX3`/`ButtonX6` for exercising all of the above on a
scratch Windows box (point `IHOTEL_PROCESS_NAME` at `powershell.exe`)
before touching a live reception terminal. Its header comment carries the
full test matrix.

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
