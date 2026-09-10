# Loyalty-app integration (booking channel + stay hook)

**Status: implemented, SHIPPED DARK** (2026-07-10, branch `feat/loyalty-channel`).
The loyalty app (separate repo/deployment) becomes (a) a **first-party booking
channel** into this PMS and (b) a **loyalty program fed by PMS checkouts**.
Interface contracts below were locked in the joint design session — do not
change field names or shapes without coordinating with the loyalty app.

## Property ↔ branch mapping

The contract identifies properties as **`hf`** (The Harbour Front Hotel) and
**`hfville`** (HF Ville). This repo's equivalent is `Branch`
(`routes::mode::Branch::{Hfhotel, Hfville}`) → per-site PG pools via
`AppState::write_pool` / `resolve_write_services`. External ids are
property-prefixed (`pms_booking_id = "hf-12345"`, `pms_stay_id = "hf-98765"`)
because the two per-site databases have overlapping SERIAL sequences.

## Feature flags / env (all fail closed)

| Env var | Purpose | Default |
|---|---|---|
| `LOYALTY_CHANNEL_ENABLED` | Master switch for the inbound `/api/channel/*` surface | **off** — all channel requests answer 503 |
| `LOYALTY_CHANNEL_TOKEN` | Shared bearer the loyalty app presents (`Authorization: Bearer …`) | unset — fail closed even when the flag is on |
| `LOYALTY_APP_URL` | Loyalty app base URL for the checkout stay hook | unset — hook off |
| `LOYALTY_SERVICE_TOKEN` | Bearer for the outbound stay hook | unset — hook off |

**The flag flip is NOT "just config"** (coexistence invariant #6): a channel
hold writes into the shared legacy DB (as a normal booking `จอง`) the moment
it is created. Enabling the channel requires a reception-coordinated live
verification, same as every other dark-shipped legacy write.

## Provisioning (deploy plumbing)

All four keys are **declared in the deploy path, blank/false**, so the flip is
a value change (a repo variable / secret + a redeploy), never a code change.

| key | kind | where it is declared | default |
|---|---|---|---|
| `LOYALTY_CHANNEL_ENABLED` | flag | GH repo **variable** → `docker-build.yml` `vars.… \|\| 'false'` → payload `.env` → compose `${LOYALTY_CHANNEL_ENABLED:-false}` | `false` |
| `LOYALTY_APP_URL` | config | GH repo **variable** → same path → compose `${LOYALTY_APP_URL:-}` | blank |
| `LOYALTY_CHANNEL_TOKEN` | secret | GH repo **secret** → payload `.secrets.loyalty_channel_token` → `/home/deploy/secrets/loyalty_channel_token` → `/run/secrets/…` | absent (empty file) |
| `LOYALTY_SERVICE_TOKEN` | secret | GH repo **secret** → payload `.secrets.loyalty_service_token` → same file path | absent (empty file) |

Notes that matter when changing this wiring:

* The two flags/URLs follow the `ROUND_WRITEBACK_ENABLED` / `HFID_LOCATION_URL`
  family — the workflow fallback and the `docker-compose.yml` default are the
  same literal, so an unset repo variable is a no-op. They are **not** part of
  the ADR-0004 compose-owned set (reconcile flags + `OTA_BRIDGE_*`), which must
  stay out of the workflow.
* The two tokens are secrets and therefore have **no `environment:` entry**;
  `secrets.rs::SECRET_FILE_MAP` already maps both files, and `env` wins over a
  file if both are present (local dev).
* `run-deploy.sh` writes a file for **every** `.secrets` key, including empty
  values. An unset GH secret therefore yields an *empty* file, which the
  hydrator treats as absent — the gate stays closed. A compose `secrets:` entry
  pointing at a **missing** file, by contrast, aborts the entire stack start.
  Hence the standing repo idiom (`ota_bridge_token`, `hfid_resolve_secret`):
  **the payload key ships one deploy AHEAD of the compose declaration.**
* **Step 2, after the deploy that first carries the payload keys** — add to
  `docker-compose.yml`: the two top-level `secrets:` definitions
  (`file: ${SECRETS_DIR:-/home/deploy/secrets}/loyalty_channel_token`, same for
  `loyalty_service_token`) and the matching two entries under the `backend`
  service's `secrets:` list. Backend only — no worker calls the channel or the
  stay hook. Until that lands, the tokens exist on the host but are not mounted,
  which is exactly the intended dark state.

## Why `/api/channel/availability` answers 503 today

Confirmed against the code on 2026-09-10.

Every `/api/channel/*` route is wrapped by
`middleware::channel_token::require_channel_token`, whose decision function is:

```rust
let Some(expected) = expected_token.filter(|_| enabled) else {
    return ChannelAccess::Disabled;   // -> 503
};
```

`Disabled` renders `503` with body
`{"success": false, "error": "loyalty channel is disabled"}`.

That gate runs **before** the bearer is examined, so today's 503 has **two
independent causes, both currently true**:

1. `LOYALTY_CHANNEL_ENABLED` is not `true` — `config::flag_enabled` accepts only
   `true`/`1` (trimmed, case-insensitive); unset, blank, `false`, `0`, `off`,
   `no` all read as off. Before this change the key was absent from the deploy
   manifests entirely, so it could only ever be off in production.
2. `LOYALTY_CHANNEL_TOKEN` is unset/empty — `config::optional_env` maps
   unset **and** blank/whitespace to `None`, and `expected_token.filter(...)`
   turns `None` into `Disabled` even with the flag on.

Consequences for verification:

* While dark, a request with **no** bearer, a **wrong** bearer and the **right**
  bearer are indistinguishable — all three get 503. That is why the go-live
  check is "an authorised call gets **503-by-flag**, not 401": a `401`
  (`{"error": "invalid or missing bearer token"}`) can only be produced once the
  flag is on *and* a token is provisioned, so seeing 401 proves the gate opened
  and the credential is wrong.
* A `404` instead of a 503 means the channel router was never mounted (no
  `AppState` — the backend is running without a DB), not a flag state.
* The startup log line is the ops discriminator, and it prints no secret:
  `Loyalty channel: enabled=<bool> (token set: <bool>); stay hook configured: <bool>`.

## The two settings that make accrual live

Checkout accrual (`service::loyalty`, Piece 3 below) is off unless **both** of
these are set — `LoyaltyConfig::stay_hook_configured()` is
`app_url.is_some() && service_token.is_some()`, and
`LoyaltyClient::from_config` returns `None` otherwise:

1. **`LOYALTY_APP_URL`** — base URL; the hook POSTs `{LOYALTY_APP_URL}/api/loyalty/stays`.
2. **`LOYALTY_SERVICE_TOKEN`** — the outbound bearer.

Neither is the channel flag: accrual is independent of
`LOYALTY_CHANNEL_ENABLED`, so a walk-in or iHOTEL-originated stay accrues too,
as long as the guest carries a membership link. With either setting missing the
client is never built, `checkout` proceeds normally and **no stay ever accrues**
— silently, by design (the hook can never fail a checkout). Per-stay
preconditions on top of the two settings: the checkout must complete the stay
(a per-room partial checkout leaves `cin_status='active'` and is skipped) and
the guest must have `ht_customers.cust_membership_id` set.

## Piece 1 — inbound channel API (`routes/channel.rs`)

Machine-to-machine, mounted **outside** `require_auth` behind
`middleware::channel_token` (constant-time shared-bearer compare; 503 when
dark, 401 on bad credentials). HF Ville **mutations** additionally require
`HFVILLE_WRITES_ENABLED` — enforced in-route because this router sits outside
the main router's `ville_write_guard` (which keys on `?branch=`).

* `GET /api/channel/availability?property=hf|hfville&check_in=YYYY-MM-DD&check_out=YYYY-MM-DD&guests=N`
  → `{property, check_in, check_out, room_types: [{room_type_id, name,
  description, nightly_price, available_count}]}`. Real inventory: active
  non-maintenance rooms of each active type minus overlapping
  confirmed/pending bookings and non-cancelled check-ins, half-open
  `[check_in, check_out)` — the exact overlap predicate of
  `room_is_available` / `validate_booking`, plus the maintenance/active
  gates (the channel has no human picker to exclude those). Types that
  cannot sleep `guests` are omitted; sold-out types report
  `available_count: 0`. `nightly_price` = `ht_room_types.type_base_price`.
  `room_type_id` is the `type_id` SERIAL as a string (stable per property).

* `POST /api/channel/bookings` → **201**
  `{pms_booking_id, total, amount_due_now, hold_expires_at}`.
  Creates a **TENTATIVE HOLD** that consumes availability immediately:
  - match-or-create guest (exact phone + case-insensitive name; else create
    via `CustomerService::create`), attach `membership_id` when supplied
    (last-write-wins);
  - pick the lowest-numbered free room of the type (same pick race the
    booking form has; accepted);
  - ride **`BookingService::create`** with `status='pending'`,
    `book_channel='loyalty'`, `book_source='loyalty'`, one assigned room at
    the quoted nightly price, `book_hold_expires_at = now + 2h` (stamped in
    the same transaction);
  - money in integer satang: `total = nightly × nights`;
    `amount_due_now = (total+1)/2` satang for `payment:"deposit50"`
    (round half-up), or `total` for `"full"`.

* `POST /api/channel/bookings/{pms_booking_id}/payment-verified`
  body `{"amount": <THB received>}` → flips `pending → confirmed`, records
  `book_deposit_amount` + `book_deposit_date`; response carries
  `deposit_recorded` + `balance_due`. **Idempotent** — replay against an
  already-confirmed booking succeeds (`already_confirmed: true`) without
  writing. Refuses released/expired holds with 409. `FOR UPDATE` serializes
  against the sweep/release.

* `POST /api/channel/bookings/{pms_booking_id}/release` → cancels the hold.
  **Idempotent** (`already_released: true` on replay). Guarded on
  `book_status='pending'` — release can never cancel a booking
  payment-verified just confirmed (409 instead).

* **Expiry sweep** (`scheduler/jobs.rs`, every 5 min, both sites):
  auto-releases holds past `book_hold_expires_at` through the same guarded
  release path. Belt-and-braces — the loyalty app's own release call is not
  load-bearing. Registered unconditionally: it filters
  `book_channel='loyalty' AND book_status='pending'` via the partial index
  `ix_ht_bookings_hold_expiry`, a no-op while the channel is dark.

### Dual-write policy for holds (the load-bearing decision)

A hold is a **roomed `pending` booking**, and this repo's existing rule is
that roomed bookings write back to iHOTEL regardless of status (the
`booking_create` recipe gates on room presence only). We deliberately keep
that: **iHOTEL sees the hold as `จอง` immediately**, otherwise a
receptionist would double-book the room during the 2h payment window.
Consequences:

* legacy has no tentative/confirmed distinction, so **payment-verified is a
  PG-only flip** — no legacy write. The validated `booking_modify` recipe has
  no deposit (`Book_Price_Pay`) leg and inventing one would violate the
  byte-parity rule, so **the deposit is not mirrored**: iHOTEL shows the
  booking with deposit 0 until checkout (known, accepted divergence — folio
  truth lands at checkout).
* release/expiry rides the normal `CancelBooking` writeback so iHOTEL frees
  the room.
* an abandoned hold therefore appears-and-disappears in iHOTEL within ≤2h —
  churn reception should be told about at go-live.

## Piece 2 — membership link on the guest profile

* Migration **086**: `ht_customers.cust_membership_id VARCHAR(64)`
  (PG-canonical only — legacy `HT_Customers` has no membership column;
  excluded from the `UpdateCustomer` re-save and the sync mapper, same
  policy as `cust_dob`), plus `ht_bookings.book_hold_expires_at TIMESTAMPTZ`.
* Desk endpoint: `PUT /api/customers/{id}/membership`
  body `{"membershipId": "…" | null}` (null/blank clears). Dedicated
  endpoint — the general customer PUT round-trips the whole record, so a
  stale form could clobber a freshly-scanned link, and COALESCE enrichment
  can't express "clear". Branch-aware. Emits `CustomerModified`
  (`changed_fields: ["cust_membership_id"]`), **no writeback**.
* Desk UI: `components/customers/MembershipEditor.tsx` inside the customer
  edit form (staff type/scan the id from the guest's member QR); saves
  independently of the main form submit. `membershipId` is on the customer
  DTO (`GET /api/customers/{id}` and the search list).
* Guest search by phone already existed
  (`GET /api/customers/search?search=<phone>` fuzzy-matches `cust_phone`).

## Piece 3 — checkout stay hook (`service/loyalty.rs`)

On checkout commit (`routes/new_checkins.rs::checkout`), a detached task:

1. re-reads the stay (post-commit) — a per-room partial checkout that did
   NOT complete the stay is naturally skipped (`cin_status` still `active`);
2. requires a membership link on the guest;
3. POSTs `{LOYALTY_APP_URL}/api/loyalty/stays` with
   `Authorization: Bearer {LOYALTY_SERVICE_TOKEN}` and body
   `{pms_stay_id: "{property}-{cin_id}", membership_id, property, check_in,
   check_out, nights}` (nights = whole days, floored at 1; loyalty side is
   idempotent on `pms_stay_id`);
4. retries 3× with 1s/2s/4s backoff (`ureq` via `spawn_blocking` — the
   `SlackClient` idiom; 4xx other than 408/429 aborts early);
5. on persistent failure: loud `tracing::error!` + Slack page with the
   `pms_stay_id` for manual replay. **The checkout itself can never be
   blocked or failed by this hook.**

Durability note: the hook is fire-and-forget (not outbox-durable) — a
backend crash in the seconds between checkout commit and POST loses that
notification (recoverable by manual replay; the Slack page covers the
observed-failure case, not the crash case). Accepted for v1; if it ever
matters, the upgrade path is a `domain_events` subscriber with an
`event_log` cursor.

## Layering / invariants audit

* SQL lives in `repository/` (`repository/channel.rs` — free functions, no
  trait: cross-aggregate reads with a single PG impl); business logic in
  `service/channel.rs` + `service/loyalty.rs`; `routes/channel.rs` is
  shape/status-code translation only.
* Canonical writes + outbox enqueue + event publish share one transaction
  (holds via `BookingService::create`; release enqueues `CancelBooking`
  with the same deterministic idempotency key `BookingService::cancel`
  would use, so `dbo.ht_writeback_ledger` dedupes across paths).
* No new legacy write **shape**: holds reuse `booking_create`, releases
  reuse `booking_cancel`, byte-parity recipes untouched.
* Tests: `hotel-backend/tests/test_channel.rs` (integration; live PG) +
  unit suites in `middleware/channel_token.rs`, `service/channel.rs`,
  `service/loyalty.rs`, `routes/channel.rs`,
  `__tests__/components/customers/MembershipEditor.test.tsx` (Jest).

## Go-live checklist (when the loyalty app is ready)

Ordering note: the estate program board's decision **P1 supersedes the
"HF Hotel first" reading of this list** — the first live channel flip is at
**HF Ville** (lower volume, ~30% OTA), and HF follows only after two clean
weeks. See `hf-tasks/tasks/direct-booking.md` (B9–B11).

1. `gh secret set LOYALTY_CHANNEL_TOKEN` / `LOYALTY_SERVICE_TOKEN`, then
   redeploy so `run-deploy.sh` writes the two secret files.
2. Land the compose `secrets:` declaration (step 2 of *Provisioning* above) so
   the files are actually mounted, and redeploy.
3. `gh variable set LOYALTY_APP_URL` and verify the stay hook against a linked
   test guest (checkout one real stay, confirm the points transaction).
4. Reception-coordinated live test of one hold → payment-verified → release →
   checkout at **HF Ville**; verify the `จอง` appears/clears correctly in
   iHOTEL (invariant #6). HF Ville mutations additionally require
   `HFVILLE_WRITES_ENABLED=true` (ADR 0002 Ship-B gate) — check its current
   value before the test rather than assuming it.
5. `gh variable set LOYALTY_CHANNEL_ENABLED -b true` + redeploy. Confirm the
   promoted image SHA, then confirm an authorised
   `GET /api/channel/availability` returns **200** (not 503) and an
   unauthorised one returns **401** (not 503).
6. Rollback is the same one-liner in reverse
   (`gh variable set LOYALTY_CHANNEL_ENABLED -b false` + redeploy); it stops new
   holds but does not cancel existing ones — the 2h expiry sweep still runs.
