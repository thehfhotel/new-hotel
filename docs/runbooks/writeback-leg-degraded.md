# Runbook — writeback leg degraded: loyalty-channel bookings not reaching iHOTEL

> **Alert:** `:satellite_antenna: Loyalty-channel writeback stalled — iHOTEL cannot see N app booking(s)`
> **Fires from:** the backend/scheduler container (`scheduler::sync::check_loyalty_writeback_stall_and_alert`), every 2 minutes.
> **Severity:** warning tier — no `<!channel>`. Real and actionable; not a wake-the-engineer page.
> **Clock:** you have the remainder of a **2-hour hold TTL** to act (`docs/loyalty-channel.md:195`).
> **Owner at 02:00:** the night receptionist (§6). Engineering owns the leg; the desk owns the room.

---

## 1. What this alert means, and the one thing it is NOT

A guest booked a room in the loyalty app. The booking **committed to PostgreSQL** — it is
real, it holds inventory in our app, and the guest may have paid a 50% deposit. Its
writeback job to the legacy iHOTEL database has **not applied** for more than the
configured threshold (default 10 minutes).

**Why that matters more than an ordinary stuck writeback.** A loyalty hold is the one
canonical write whose *value* depends on the legacy leg being up. From
`docs/loyalty-channel.md:225-227`:

> A hold is a **roomed `pending` booking** … We deliberately keep that: **iHOTEL sees the
> hold as `จอง` immediately**, otherwise a receptionist would double-book the room during
> the 2h payment window.

The `booking_create` recipe writes one `HT_Room_Status` row per night with
`room_status='จอง'` (`hotel-backend/src/writeback/recipes/booking_create.rs:243,250-252`)
— **that** is what puts the room on the iHOTEL room board, which is the only surface a
night receptionist actually looks at. When the writeback leg is down, that row never
lands. The room reads FREE in iHOTEL while our app has it sold.

**This is NOT the "PMS unreachable" failure.** Those are opposite shapes and it is worth
being precise, because they call for opposite responses:

| | PMS API unreachable | **This alert** |
|---|---|---|
| What broke | the loyalty app cannot reach *our* backend | our backend is fine; the **legacy leg** is down |
| What the guest gets | an error — no booking (fails closed, §2) | **a successful booking** |
| What reception sees | nothing, correctly | **nothing, incorrectly** — a sold room reading FREE |
| Who is exposed | nobody | the next walk-in, and the app guest who paid |

Only the second one can double-sell a room. That asymmetry — the guest is told "yes"
while the desk is told nothing — is the entire reason this detector exists.

**Why nothing else catches it.** Every other detector on this surface is slower than the
hold's own lifetime or blind to a single row:

| Detector | Threshold | Why it cannot fire in time |
|---|---|---|
| level-drift digest (`scheduler/sync.rs`, `DEFAULT_LEVEL_DRIFT_STALE_INTERVAL_HOURS`) | row unconverged **4 h** | 4 h > the 2 h TTL — the hold expires and vanishes first |
| burst page (`DEFAULT_DRIFT_ALERT_THRESHOLD`) | **50 rows/hour** | one stuck hold is 1 |
| queue-depth janitor (`bin/writeback.rs`, `QUEUE_PENDING_ALERT_THRESHOLD`) | **500 pending** | one stuck hold is 1 |

**Where the detector runs, and why it is not in the writeback worker.** It runs in the
**backend/scheduler container** and reads **only canonical PostgreSQL**. The most likely
cause of this alert is that the writeback worker container itself is down — a detector
living inside that worker would share its fate and go silent in exactly the case it
exists for. This is also why the alert can reach you while the legacy leg is completely
unreachable: nothing in its path touches MSSQL.

### When it fires

All three must hold:

1. the booking carries `book_channel = 'loyalty'`;
2. a `writeback_jobs` row for it **exists** (so PG committed — never "the hold was never
   created") and is **at least N minutes old** (default 10);
3. that job is in **any status other than `done`** — `pending`, `in_progress`, `failed`
   or `exhausted`. All four mean iHOTEL has not seen the booking. `in_progress` is
   included on purpose: a worker that dies mid-claim leaves the row claimed forever.

Non-loyalty writebacks are ignored however old — a stuck desk or OTA writeback is a real
problem but carries no 2-hour fuse and is covered by the digest and queue-depth alerts.

One alert per site per cooldown (default 30 min), and a `:white_check_mark:` all-clear
when the backlog drains. **If you see the alert and then silence, that is not recovery** —
recovery has its own message. Silence means the cooldown is holding, or the backend is
also down.

---

## 2. Fail-closed behaviour today — what the guest actually sees

Two different outages, two different guest experiences. Read both before telling a guest
anything.

### 2a. When our PMS is unreachable from the loyalty app — it does fail closed

The app holds **no inventory of its own** (`loyalty-app/docs/adr/0003-booking-channel-into-pms.md`),
so with the PMS unreachable it cannot invent an answer. The path:

`POST /api/bookings/channel`
→ `loyalty-app/backend-rust/src/routes/bookings.rs:1525` (`create_channel_booking`)
→ `:1578` builds `PmsChannelClient::from_settings`
→ `loyalty-app/backend-rust/src/services/pms_channel.rs:112` (`create_booking`)
→ `:124` `.map_err(pms_unreachable)?`
→ `:273-279`:

```rust
fn pms_unreachable(e: reqwest::Error) -> AppError {
    if e.is_timeout() {
        AppError::ExternalServiceTimeout(format!("PMS channel API timed out: {e}"))
    } else {
        AppError::ExternalServiceUnavailable(format!("PMS channel API unreachable: {e}"))
    }
}
```

→ HTTP **503** (`loyalty-app/backend-rust/src/error.rs:325`) or **504** on timeout (`:326`).

**No booking is created. Good — that half is correct.**

### 2b. Three honest caveats about that copy (verified 2026-09-11, do not assume otherwise)

These are gaps, recorded here so nobody at 02:00 believes the app is saying something it
is not. **None of them is fixed by this PR** — they belong to the loyalty app.

1. **There is no Thai "call the desk" message on this path.** The guest-facing string is
   the generic `"ไม่สามารถสร้างการจองได้"` ("unable to create booking",
   `loyalty-app/frontend/src/i18n/locales/th/translation.json:1730`). The only
   `กรุณาติดต่อแผนกต้อนรับ` copy in the app is in the **membership-QR** flow
   (`:1879-1880`), not the booking flow. If you want the guest to phone reception during
   an outage, that copy does not exist yet.
2. **The guest may see a machine slug, not a sentence.** The axios interceptor resolves
   `data?.error ?? data?.message` (`loyalty-app/frontend/src/utils/axiosInterceptor.ts:31-32`),
   and the backend's JSON field named `error` holds the **machine code**
   (`"external_service_unavailable"`, `loyalty-app/backend-rust/src/error.rs:252`), not
   the human text from `:401-403`. So the toast can render `external_service_unavailable`.
3. **There is no request timeout.** `PmsChannelClient` is built with
   `reqwest::Client::new()` and no `.timeout(...)`
   (`loyalty-app/backend-rust/src/services/pms_channel.rs:84`) — contrast the SlipOK
   client, which sets 30 s. A black-holed TCP connection therefore **hangs** rather than
   failing fast, and the `is_timeout()` branch above is effectively unreachable for that
   case. A guest who gives up and retries can create a second hold (no idempotency key
   either, `:41-53`). Tracked as control **L5** in the b8 overbooking analysis.

### 2c. When the legacy leg is down — the app does NOT fail closed, and cannot

**This is the case this runbook is about.** Our PMS is healthy, so it answers normally:
availability is computed from PostgreSQL, the hold is created, PG commits, and the guest
is told **yes**. The legacy write is asynchronous by design (PG first, then mirror) and is
never in the request path.

That is the correct architecture and must not be "fixed" by making the request path wait
on MSSQL. It does mean **the only thing standing between this outage and a double-sold
room is reception knowing** — i.e. this alert and §6.

---

## 3. The alert text

Both messages are prefixed by the site tag, e.g. `[site=hfhotel] ` / `[site=hfville] `.

### 3a. The alert

```
:satellite_antenna: *Loyalty-channel writeback stalled — iHOTEL cannot see 2 app booking(s)* :satellite_antenna:
2 booking(s) made in the guest app committed to PostgreSQL but their legacy writeback job has not applied for more than 10 minute(s). iHOTEL does NOT show these rooms as `จอง`, so the desk can double-book them — and a hold that expires before the leg recovers disappears without ever reaching the room board:
• `BK26-000412` — `create_booking` pending for 12m, hold expires in 108m
• `BK26-000413` — `create_booking` in_progress for 31m, HOLD ALREADY EXPIRED
_Runbook:_ `docs/runbooks/writeback-leg-degraded.md` _— check the writeback worker and the legacy leg first. Per-site cooldown 30 min; a_ `:white_check_mark:` _all-clear fires once the backlog drains._
```

Per row: `• <book_no> — <intent> <status> for <age>m[, hold expires in <n>m | , HOLD ALREADY EXPIRED]`.
At most 15 rows are listed, then `…and N more`.

Read the row, not just the headline:

- **`hold expires in <n>m`** — you have `n` minutes to get the leg back before the hold
  self-cancels. Below ~20m, go to §6 and block the room by hand rather than waiting.
- **`HOLD ALREADY EXPIRED`** — the hold died during the outage. The room is free in our
  app; the guest may still believe they have it. See §5.
- **`cancel_booking` as the intent** — the inverse harm: a cancellation that never
  reached iHOTEL leaves a **phantom `จอง`** and reception is holding a room that is
  actually free. Do not skip these because "nothing was sold".

### 3b. The recovery (paired all-clear)

```
:white_check_mark: *Loyalty-channel writeback RECOVERED* :white_check_mark:
Every loyalty-channel booking's writeback job has applied — no job is older than 10 minute(s) without reaching iHOTEL. The app's holds are back on the iHOTEL room board.
_Closure of the_ `:satellite_antenna:` _stall alert sent earlier. Check `docs/runbooks/writeback-leg-degraded.md` §5 for the bookings that were invisible during the outage — a hold that expired mid-outage may have left a stale_ `จอง` _row._
```

It fires **once**, only after a real alert, and only when the backlog is fully drained.
A partial drain is not a recovery. A site that has never stalled never emits one.

---

## 4. Recovery recipe — getting the legacy leg back

> **Terminology correction, verified in-repo.** The legacy MSSQL leg is **WireGuard**, not
> Tailscale. HF Ville's MSSQL is reached over the `hfville` WG interface
> (`evergreen → MikroTik DNAT → 192.168.11.51:1436`, `docker-compose.yml:987,1019`), a path
> chosen **over** Tailscale subnet routing on purpose
> (`docs/adr/0001-phase5-ville-multi-site.md:21,43`). **HF Hotel's MSSQL is on the LAN and
> needs no tunnel at all.** Tailscale still matters as a *liveness signal* — when a box
> drops off Tailscale *and* the WG ping dies *and* SQL probes fail, the box itself is
> down, not the tunnel (`docs/coexistence/sync-incident-log.md:721-722`). Do not spend
> 02:00 re-dialling a tunnel to a box that is hung.

Work the ladder in order. Stop at the first rung that explains the symptom.

**Rung 1 — is the worker even running?** This is the most likely cause, and the cheapest
to check. The workers are behind opt-in compose profiles, so a deploy that lost its
profile flag leaves them simply absent:

```
docker compose ps          # look for `writeback` and `writeback-hfville`
```

Service definitions: `docker-compose.yml:765` (`writeback`, `profiles: [legacy]`) and
`:1250` (`writeback-hfville`, `profiles: [hfville]`). Bring back with
`docker compose --profile legacy up -d` / `--profile hfville up -d`.

**Note the restart-cap trap** (`docs/runbook-sync.md` §3): with `restart: on-failure:5`, a
worker that refuses to start pages a few times and then goes **permanently silent**. A
quiet channel is not recovery — always confirm with `docker compose ps`.

**Rung 2 — did the worker refuse to start?** Check its logs for a
`:no_entry:`/`:warning: REFUSED TO START` reason: schema fingerprint drift, legacy
collation check, or the writeback-ledger check
(`bin/writeback.rs` startup probes; budget `WRITEBACK_STARTUP_PROBE_ATTEMPTS`, default 4,
~36 s total). A fingerprint mismatch is re-captured with
`./scripts/writeback-fingerprint.sh` — read `docs/runbook-sync.md` §3 first; a
fingerprint change usually means somebody altered the legacy schema, which is its own
incident.

**Rung 3 — HF Ville only: is the WG tunnel up?** The established recipe
(`docs/coexistence/RUNBOOK-mssql-022-apply.md:49-55`, identical at
`docs/coexistence/RUNBOOK-b5-backfill.md:59-64`):

```
sudo wg-quick up hfville
ping -c1 192.168.11.51     # must succeed before continuing
```

**Precedent — transient, self-healing.** A `HF Ville DEGRADED→UP` pair fired 2026-08-07
09:26→09:28 ICT: WireGuard down during a **PPPoE re-dial**, self-healed in 2 minutes, app
path stayed up, **needed no action** (`docs/coexistence/sync-incident-log.md:642-643`).
If the leg returns on its own inside a few minutes and the all-clear arrives, that is this
shape. Do not escalate it.

**Rung 4 — is the legacy box itself alive?** If Tailscale, the WG ping and the SQL probes
are *all* dead at once, the box is hung, not the network. Precedent: 2026-08-11, an SSD
I/O failure on HF Ville's system drive froze iHOTEL and the whole box; the CT watermark
pinned onset to 11:36 and only a manual reboot at ~13:21 restored it
(`docs/coexistence/sync-incident-log.md:719-755`). A reboot is an owner action at the
physical site — wake the owner, not an engineer.

**Rung 5 — after any legacy-box reboot, check what did not come back.** iHOTEL's
middleware **did not autostart** after that reboot — its HKCU `Run` entry is UNQUOTED
unlike every sibling entry (`docs/coexistence/sync-incident-log.md:753-755`). A box that
is "up" is not necessarily a leg that is working. Re-check `docker compose ps`, the
watermark, and wait for the `:white_check_mark:`.

**What NOT to do, at any rung:**

- **Do not hand-write the missing rows into iHOTEL.** The writeback recipes are
  byte-parity contracts (`'…'` not `N'…'`, `M/D/YYYY H:MM:SS AM/PM`, app-allocated ids
  under `TABLOCKX, HOLDLOCK`). A hand-typed row will not match and will generate a
  reconcile divergence that cannot auto-close. Block the room in iHOTEL the way reception
  normally would (§6) and let the queue drain.
- **Do not delete or re-enqueue `writeback_jobs` rows** to "unstick" them. Create
  writebacks are idempotency-keyed through `dbo.ht_writeback_ledger`; the retry path is
  designed for exactly this and re-enqueueing can double-write.
- **Do not turn the loyalty channel off as a first move.** It fails *closed* for new
  bookings only if the PMS is unreachable, which is not this outage; switching it off
  strands the holds already outstanding without fixing any of them.

---

## 5. After recovery — reconcile what was invisible

The all-clear means the **queue** drained. It does not mean the **room board** is correct.
Check these before closing the incident:

1. **Holds that expired mid-outage.** Any row the alert showed as `HOLD ALREADY EXPIRED`.
   Our app auto-cancelled it via the 5-minute expiry sweep, which enqueues a normal
   `CancelBooking` writeback — so once the leg is back, iHOTEL may briefly show a `จอง`
   appear and then disappear. That churn is **expected**
   (`docs/loyalty-channel.md:237-239` flags it as a go-live communication item), but a
   guest who paid inside that window has paid for a room they no longer hold. Escalate to
   the owner for a refund-or-rebook decision; do not silently re-create the booking.
2. **Stale `จอง` from stalled `cancel_booking` intents.** Confirm the room reads free in
   iHOTEL now.
3. **Rooms sold twice during the outage.** For each `book_no` in the alert, check whether
   iHOTEL acquired a *different* booking or walk-in for the same room-night while it read
   free. That is the double-sell this alert exists to prevent; if one happened, it is a
   desk conversation, immediately.
4. **Deposit reads 0 in iHOTEL — expected, not a symptom.** Payment-verified is a PG-only
   flip; the validated `booking_modify` recipe has no deposit leg and inventing one would
   break byte-parity, so iHOTEL shows deposit 0 until checkout
   (`docs/loyalty-channel.md:230-234`). Do not "correct" it by hand.

---

## 6. Who does what at 02:00

**The night receptionist acts; nobody wakes an engineer for a single held room.**

**Receptionist — as soon as the alert lands:**

1. Read the `book_no` and the room from the alert row.
2. **Assume iHOTEL is wrong about that room.** Treat it as OCCUPIED. If a walk-in asks
   for it, give a different room.
3. Block the room on the iHOTEL room board the way you normally would for a phone
   booking, noting the `book_no`. This is a *manual* hold — it is not a writeback and will
   not conflict with one.
4. If the row says `HOLD ALREADY EXPIRED`, the app guest no longer has the room. Do not
   turn away a walk-in for an expired hold; note it for the morning.

**Receptionist — if an app guest phones about their booking during the outage:** their
booking is **real** in our system. Confirm it from the app/PMS side, not from the iHOTEL
room board, which is the surface that is wrong.

**Whoever is on call (morning, or immediately if the alert repeats past one cooldown):**
work §4 rung by rung. Rungs 1–3 are safe to do at any hour. **Rung 4 (a legacy-box reboot)
is an owner action at the site** — that is the one escalation worth a phone call, and only
once rungs 1–3 are ruled out.

**Nobody, at any hour:** hand-writes legacy rows, re-enqueues writeback jobs, or flips a
writeback flag to "make it go". New legacy writes ship dark and behind coordinated
verification; an alert at 02:00 is not that coordination.

**Closing the incident:** the `:white_check_mark:` all-clear plus §5 items 1–3 checked.
Log anything that reached a guest in `docs/coexistence/sync-incident-log.md`.

---

## 7. Tuning knobs

| Var | Meaning | Default |
|---|---|---|
| `LOYALTY_WRITEBACK_STALL_ALERT_MINUTES` | Age at which a not-yet-applied loyalty writeback job counts as a confirmed stall. | `10` |
| `LOYALTY_WRITEBACK_STALL_COOLDOWN_MINUTES` | Minimum gap between repeats of the alert for one site. | `30` |

Both are global (no per-site suffix). Non-numeric, zero and negative values are ignored
and fall back to the default — a `0`-minute threshold would alert on every job the instant
it is enqueued.

**Sizing, so nobody "tunes it down until it fires".** The floor on the threshold is what
the worker does on its own before a human should be involved: NOTIFY delivery is
sub-second, the poll fallback is 30 s, and retry backoff plus the janitor's stuck-claim
steal both settle inside 5 minutes. **Below ~6 minutes you will page on healthy retries.**
The ceiling is the 2 h hold TTL — the alert must land with enough of the window left for
reception to act. 10 minutes is ~20× healthy latency with ~1h50m to spare.

The cooldown is deliberately far shorter than the 24 h level-drift cooldown because the
condition is bounded by that same 2 h TTL: at 24 h the whole incident would collapse into
one message. 30 minutes gives an unattended overnight outage ~4 reminders inside a hold's
life.

The alert stays on the **warning tier** (`with_site_text`, no `<!channel>`). Do not
promote it to the pager tier without a decision record: a channel-mention at 02:00 for one
held room is the fastest way to get the whole channel muted.

---

## Related

- `hf-tasks/tasks/direct-booking-designs/b8-overbooking-analysis.md` §3, §4 — the analysis
  that specified this detector (defect 3, control **L7**), and the other controls (**L2**
  last-room guard, **L3** serialized pick→create, **L4** late-slip status filter, **L5**
  client timeout + idempotency key) that this one does *not* replace.
- `docs/loyalty-channel.md` — the channel contract: hold TTL, why holds write back
  immediately, the deposit-0 divergence, the churn note.
- `docs/runbook-sync.md` §2a (alert-tuning knobs), §3 (Slack alert meanings).
- `docs/coexistence/sync-incident-log.md` — the WG re-dial (`:642-643`) and box-hang
  (`:719-755`) precedents cited in §4.
