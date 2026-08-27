# ADR 0006 — Notify reception on legacy writeback commit; don't auto-refresh iHOTEL

**Status:** Accepted — 2026-08-04.
**Scope:** what happens on the reception PC after a `bin/writeback.rs` job commits a row
into legacy MSSQL — specifically, whether/how iHOTEL's room grid (`FormRoomMain`) is made
to reflect it. Does **not** cover the toast-delivery implementation itself (Tauri plugin
wiring, the SSE→localhost hop) — that is separately-scoped build work this ADR licenses
but does not design.

## Context

Every writeback (mark-dirty, check-in, check-out, booking, payment, ...) commits into the
shared legacy MSSQL, which iHOTEL's own UI does not know to re-read. Full mechanics are now
recorded in [`docs/legacy-app/ROOM_GRID_REFRESH.md`](../legacy-app/ROOM_GRID_REFRESH.md)
(decompile-derived, 2026-08-03/04); the load-bearing facts for this decision:

- iHOTEL's room grid polls itself on a ≈60.6 s timer (`FormRoomMain.Timer1`), but that timer
  is gated on `!MSSQL.CodeErr`, which goes `true` the instant the form loses focus
  (`FormRoomMain_Deactivate`) and only clears when it regains it. **While reception is
  working in our app instead of iHOTEL — which is exactly when a writeback fires — iHOTEL's
  grid does not auto-refresh at all**, for any duration
  (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"1. The refresh timer" "FormRoomMain.Timer1.Interval = 60560"
  and `docs/legacy-app/ROOM_GRID_REFRESH.md` §"2. The focus gate").
- The only externally-reachable refresh is iHOTEL's own "Refresh" button, which also calls
  `ClearCheck()` and destroys any in-progress multi-room tap-selection
  (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"5. The only external lever").
- A refresh is not read-only: `LoadRooms` can write `HT_Rooms`/`HT_Book_Date` and toggle
  room power via a serial relay (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"4. What a refresh actually costs").
- Exhaustive search of the decompiled binary found no socket, named pipe, file watch, SQL
  notification, or window-message hook of any kind — **iHOTEL cannot be told to refresh by
  anything outside its own process**
  (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"6. The negative finding"). This also forecloses a
  DB-side fix (trigger, Service Broker): the legacy DB has zero triggers/sprocs today and
  iHOTEL's whole app assumes that stays true
  (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"11. Why not fix this DB-side").

Given that, the practical options are: do nothing (status quo — reception occasionally
looks at a stale grid and doesn't know it); make our app drive iHOTEL's refresh for her; or
tell her a refresh would help and let her decide. This ADR picks the third.

## Decision

On writeback commit, the reception PC's local Tauri tray middleware
(`thai-id-middleware-tauri` — already deployed as the card-reader HTTP bridge on
`127.0.0.1:9898`, tray icon already present per `capabilities/default.json`'s
`core:tray:default`) shows a **Windows toast** with an opt-in **"Refresh iHOTEL now"**
button. **The refresh is never triggered automatically.**

### 1. Notify, don't automate

Not just a preference — automating the refresh would mean *our app* causes writes into
legacy MSSQL at moments nobody chose (`Power_set2` room-power toggles,
`AutoAddBookingRooms`'s `HT_Rooms`/`HT_Book_Date` UPDATEs —
`docs/legacy-app/ROOM_GRID_REFRESH.md` §"4. What a refresh actually costs"), and
those writes round-trip straight back through Change Tracking into canonical PostgreSQL.
It would also blow away whatever multi-room selection reception has mid-flight
(`ClearCheck()`, `docs/legacy-app/ROOM_GRID_REFRESH.md` §"5. The only external lever") — from her point of view, our app would be
silently clearing her in-progress work on a screen she isn't even looking at. Someone will
propose full automation within a month of this shipping ("just click Refresh for her,
what's the harm") — this ADR is the answer, and the reasoning above is why "it's just a
button click" undersells the cost.

### 2. Ephemeral channel, not `EventBus`/`event_log`

The signal is worthless the instant it's stale or nobody's listening — a durable, indexed
row with real retention is the wrong shape for it, and it would mislabel an adapter-level
fact ("a legacy row now exists") as a domain fact, which is exactly what `EventBus`/
`event_log` is for elsewhere in this codebase. Use a plain `pg_notify` on its own channel,
fired from the writeback worker at commit — same shape as the existing
`writeback_jobs_notify_trigger` precedent (`migrations/pg/016_writeback_notify_trigger.sql`),
which already establishes "trigger-fired NOTIFY inside the committing transaction, no
durable row" as the pattern for exactly this kind of internal wake-up signal.

Name it `legacy_stale` — explicitly **not** `refresh`. That name is already taken:
`routes/events.rs:133` defines `pub const RESYNC_EVENT: &str = "refresh"`, which means
"refetch our own UI's data" and fires on SSE listener-reconnect resyncs that have nothing
to do with a writeback landing in legacy. Reusing the string would make two structurally
different signals indistinguishable on the wire.

### 3. Browser-mediated transport, not a middleware-direct SSE subscription

The Tauri process does not hold its own connection to the backend's SSE stream. Instead,
the reception PC's browser tab (already SSE-connected via `routes/events.rs` for the v2 UI)
receives the signal and relays it to the local Tauri middleware over its existing
`127.0.0.1:9898` HTTP bridge, which then raises the toast.

The alternative — the Tauri process subscribing to backend SSE directly — was rejected
because it needs three things this project has already paid to avoid once:

- A **Cloudflare Access service token** distributed to every reception PC so the desktop
  process can authenticate cross-network: a long-lived credential on machines reception
  physically controls, whose rotation means reinstalling on every PC.
- **Per-PC branch identity** (which site's events this PC should get), a piece of config
  that doesn't exist today outside the browser session.
- A **permanently-held connection from a Windows desktop process**, the exact shape
  `docs/adr/0001-phase5-ville-multi-site.md` §"Decision summary" "fragile (Windows userspace, sleeps with RDP logout)"
  already rejected once for `desktop-0be5led`'s own Tailscale daemon (Q3 row of that table).

**Honest cost, recorded rather than glossed over:** delivery requires BOTH the tray app
running AND a browser tab open on the v2 UI on that PC. If either is missing, the failure
mode is silent — reception just sees today's status quo (no notification at all), which
means this is strictly additive risk, never worse than not building it. **Revisit trigger:**
if a measured population of iHOTEL-only PCs emerges (terminals where nobody ever opens the
v2 browser UI), this transport delivers nothing to them and the direct-SSE alternative
above should be re-evaluated for that population specifically, accepting its cost.

### 4. Deployment posture change

Installing the middleware onto the legacy hosts is a real, deliberate widening of blast
radius, recorded here instead of discovered later. The SSH key `~/.ssh/front2_diag`
(comment `front2-diag-readonly`, created on evergreen 2026-07-31 17:57 +07) was created
for **read-only** diagnostics only (the Task-Scheduler/Event-Viewer investigation behind
issue #277's bb8-timeout root-cause work).
Using that same access path to install and run toast-notification middleware turns it into
"run code as an admin console user on both legacy DB hosts":

| Host | Tailscale IP | Console user | Notes |
|---|---|---|---|
| `FRONT2` | `100.79.67.0` | `front2` | Runs `HOTEL.exe`; **also hosts the live SQL Express instance** — the shared legacy MSSQL itself. |
| `DESKTOP-0BE5LED` | `100.109.224.37` | `admin` | Runs `HOTEL.exe`; HF Ville's MSSQL host (per `docs/adr/0001-phase5-ville-multi-site.md`). |

Both are verified as the interactive console user, both currently running `HOTEL.exe`. That
`FRONT2` is simultaneously the SQL Server host means anything that goes wrong with the
installed middleware process shares a machine with the database itself — a materially
different risk than a diagnostics-only read.

**Rollback per host**, if this needs to be undone:

```powershell
Stop-Service sshd; Set-Service sshd -StartupType Disabled
# then remove the OpenSSH-evergreen-only firewall rule
```

### 5. Alarm fatigue is the primary failure mode

A toast on every writeback would retrain reception to ignore toasts within a shift — the
same failure class this project already treats as a first-order design constraint for
Slack alerts (CLAUDE.md's alerting guardrail: page only on confirmed/unrecoverable
failures). Mitigation is a **latch**, not a counter:

- Staleness is modeled as one open "stale episode" per room grid. The first writeback while
  iHOTEL is unfocused/stale opens it and fires **one** toast; every subsequent writeback
  while the episode is still open just increments a counter — no new toast.
- Suppressed entirely while iHOTEL is the foreground window (nothing to notify about if
  she's already looking at it — and per `docs/legacy-app/ROOM_GRID_REFRESH.md` §"2. The focus gate", focus alone clears
  `MSSQL.CodeErr` and lets the normal 60.6 s timer resume).
- Cleared either by the receptionist clicking "Refresh iHOTEL now," or by iHOTEL holding
  foreground continuously for ≥65 s (long enough for at least one natural timer tick to
  have run — the ≈60.6 s interval in
  `docs/legacy-app/ROOM_GRID_REFRESH.md` §"1. The refresh timer" "FormRoomMain.Timer1.Interval = 60560", plus slack).

**Pilot acceptance gate:** ≤1 toast per 30 minutes sustained over a full shift, and a direct
question to the receptionist afterward — did she use the button, or ignore it? A pilot that
passes the rate gate but gets ignored is still a failure; both signals are needed before
this graduates past a pilot.

## Consequences

**Gained.** Reception gets a signal that iHOTEL is showing stale room state, at the moment
it starts being stale, without our app ever writing into legacy MSSQL on its own initiative.
The decision is opt-in at the point of use (she clicks, or she doesn't), consistent with
iHOTEL-anchored UX's improvement invariant (CONTEXT.md) — nothing becomes automatic that
wasn't a deliberate action before.

**Given up.** No fully-automatic fix to the staleness problem — it's marketing an alert, not
resolving the underlying gap between the two apps. Delivery has a real dependency (browser
tab + tray app both running) whose absence is silent, and reception must still take a
manual action to actually refresh. Two legacy Windows hosts, one of them the SQL Server
host itself, now carry an admin-capable middleware install where before the same SSH access
was read-only diagnostics.

## Alternatives considered

- **Automate the refresh entirely** (call iHOTEL's own refresh path, or drive
  `ButtonX3_Click` via UI automation, on every writeback). Rejected — see Decision §1.
- **DB-side notification** (a trigger or Service Broker on the shared legacy MSSQL).
  Rejected — legacy DDL beyond the CT prerequisite carve-out is prohibited, and the legacy
  DB's zero-trigger invariant is load-bearing for the rest of this project's byte-parity
  methodology (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"11. Why not fix this DB-side").
- **Tauri middleware subscribes to backend SSE directly**, skipping the browser hop.
  Rejected — see Decision §3.
- **Poll `IsListroom`-style from outside the process.** Not viable at all:
  `Module1.IsListroom` is an in-process static field, never persisted anywhere external
  (`docs/legacy-app/ROOM_GRID_REFRESH.md` §"7. The app's own refresh flag is unreachable")
  — there is nothing to poll from outside `HOTEL.exe`.

## References

- `docs/legacy-app/ROOM_GRID_REFRESH.md` — the decompile-derived findings this decision
  rests on; read that first for anything claimed above about `FormRoomMain`'s behavior.
- `migrations/pg/016_writeback_notify_trigger.sql` — the `pg_notify`-inside-transaction
  precedent this design's channel reuses the shape of.
- `hotel-backend/src/routes/events.rs:133` — `RESYNC_EVENT = "refresh"`, the name this
  design deliberately does not reuse.
- `docs/adr/0001-phase5-ville-multi-site.md` §"Decision summary" "fragile (Windows userspace, sleeps with RDP logout)"
  (the Q3 row) — the prior rejection of a permanently-held Windows-desktop connection this
  ADR's transport choice avoids repeating.
- `thai-id-middleware-tauri/` — the existing local middleware this design extends; see its
  `README.md` for the current card-reader HTTP bridge it already runs.
- Issue #277 — the bb8 pool-checkout timeout investigation that motivated the
  `front2_diag` read-only SSH key this ADR's deployment-posture section is about.
- CLAUDE.md — "New legacy writes ship DARK" and the alerting guardrail (page only on
  confirmed/unrecoverable failures), both directly load-bearing for this decision.
