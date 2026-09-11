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
| `LOYALTY_CHANNEL_ENABLED` | flag | **`docker-compose.yml` default only** (ADR 0004) — deliberately absent from `docker-build.yml`; flip = edit `${LOYALTY_CHANNEL_ENABLED:-false}` | `false` |
| `LOYALTY_APP_URL` | config | GH repo **variable** → same path → compose `${LOYALTY_APP_URL:-}` | blank |
| `LOYALTY_CHANNEL_TOKEN` | secret | GH repo **secret** → payload `.secrets.loyalty_channel_token` → `/home/deploy/secrets/loyalty_channel_token` → `/run/secrets/…` | absent (empty file) |
| `LOYALTY_SERVICE_TOKEN` | secret | GH repo **secret** → payload `.secrets.loyalty_service_token` → same file path | absent (empty file) |

Notes that matter when changing this wiring:

* `LOYALTY_CHANNEL_ENABLED` **is** part of the ADR-0004 compose-owned set
  (reconcile flags + `OTA_BRIDGE_*`): it is an operational flag guarding a
  legacy write, so its state is the committed `docker-compose.yml` default and
  it is deliberately absent from `docker-build.yml`. Routing it through a repo
  variable would make that default unreachable — `run-deploy.sh` materialises
  `.env` wholesale — leaving the repo asserting the channel is dark while it
  writes `จอง` rows. The flip is therefore a one-line reviewable diff that
  `git log -S LOYALTY_CHANNEL_ENABLED` can date, **not** a `gh variable set`.
* `LOYALTY_APP_URL` is a per-environment base URL, the category ADR 0004 leaves
  on the variable path (`ROUND_WRITEBACK_ENABLED` / `HFID_LOCATION_URL` family).
  Its workflow fallback and its compose default are the same literal, so an
  unset repo variable is a no-op.
* CI job **`lint-deploy-flag-ownership`** enforces both halves: no compose-owned
  key may appear in `docker-build.yml`, and every key that rides both files must
  carry the same literal on each side.
* The two tokens are secrets and therefore have **no `environment:` entry**;
  `secrets.rs::SECRET_FILE_MAP` already maps both files, and `env` wins over a
  file if both are present (local dev).
* `run-deploy.sh` writes a file for **every** `.secrets` key, including empty
  values. An unset GH secret therefore yields an *empty* file, which the
  hydrator treats as absent — the gate stays closed. A compose `secrets:` entry
  pointing at a **missing** file, by contrast, aborts the entire stack start.
  Hence the standing repo idiom (`ota_bridge_token`, `hfid_resolve_secret`):
  **the payload key ships one deploy AHEAD of the compose declaration.**
* **Step 2 — LANDED** (PR #298), one deploy
  after the payload keys. `docker-compose.yml` now carries the two top-level
  `secrets:` definitions (`file:
  ${SECRETS_DIR:-/home/deploy/secrets}/loyalty_channel_token`, same for
  `loyalty_service_token`) and the matching two entries under the `backend`
  service's `secrets:` list. Backend only — no worker calls the channel or the
  stay hook. Mounting them opens nothing: `LOYALTY_CHANNEL_ENABLED` is still
  `false` and both files are empty until the GH secrets are minted, which the
  hydrator treats as absent (`config::tests::
  loyalty_tokens_stay_unprovisioned_when_the_secret_file_is_missing_or_empty`
  pins that a missing *or* empty file still boots the backend with the channel
  dark, flag forced on included).
* `run-deploy.sh` needs no per-secret change: its single loop installs **every**
  `.secrets` key with `install -m 0444 -o deploy -g docker`, so the two loyalty
  files land with exactly the same mode and owner as `db_password` and friends.
  **0444, not 0400** — the backend container's non-root `appuser` must be able
  to read the bind-mounted file (v2.66.2 operator note); the surviving `0400`
  claims in prose were stale. Tightening the mode is a tracked follow-up with
  its own reasoning block in the script — do not "fix" it in passing.

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
  bearer are indistinguishable — all three get 503. **This makes HTTP useless as
  the acceptance test for B4 (mint the tokens).** A 503 is returned whether the
  secret was minted, minted with a typo'd payload key, or never minted at all;
  whether the compose `secrets:` follow-up landed or not. A check that cannot
  fail certifies nothing, and the missing mount would surface only at step 5 of
  the checklist — inside the reception-coordinated live window.
* **The acceptance test for B4 is the startup log line**, which is the only
  secret-free observable that discriminates. After the redeploy, in the
  `backend` container log:

  ```
  Loyalty channel: enabled=false (token set: true); stay hook configured: false
  ```

  `token set:` is `true` **exactly when** the token actually reached the process
  — i.e. the compose `secrets:` declaration landed and the file is non-empty. If
  it reads `false` after B4, the mount or the secret is missing; do not proceed.
  (`hotel-backend/src/main.rs`, `LoyaltyConfig::from_env`.) The same line is the
  B12 check for accrual: `stay hook configured: true`.
* A `401` (`{"error": "invalid or missing bearer token"}`) can only be produced
  once the flag is on *and* a token is provisioned — so once the channel is
  live, an unauthorised call returning 401 rather than 503 proves the gate
  opened. That is a step-5 observation, not a B4 one.
* A `404` instead of a 503 means the channel router was never mounted (no
  `AppState` — the backend is running without a DB), not a flag state.

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
  `{pms_booking_id, total, amount_due_now, hold_expires_at}`. Accepts an
  OPTIONAL **`Idempotency-Key`** header so a client retry replays the first
  response instead of creating a second hold — see §Idempotency below.
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
  already-settled booking succeeds (`already_confirmed: true`) without writing.
  The settled set accepts **both spellings of each state**
  (`service::channel::is_settled_status`): this app writes `'checkedin'` while
  the CT sync mapper writes `'checked_in'` for iHOTEL's `เข้าพัก`, and the
  underscored spelling is the steady state — so a late payment-verified retry
  for a guest the desk has since checked in through iHOTEL replays instead of
  answering 409. `'checkedout'` / `'checked_out'` / `'completed'` (legacy
  `ออกแล้ว`) are settled for the same reason. **What the mapper writes is
  unchanged** — only what we accept. Refuses released/expired holds with 409.
  `FOR UPDATE` serializes against the sweep/release.

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

### Idempotency on the hold create (`Idempotency-Key`)

**Header name: `Idempotency-Key`. Response marker: `Idempotency-Replayed: true`.**
Optional — a request without the header behaves exactly as it always has.

`payment-verified` and `release` are naturally replay-tolerant (they converge on
a state). `POST /api/channel/bookings` is not: it mints a NEW hold every time it
runs. A loyalty-app client whose request hung and was retried therefore ended up
with **two** holds → two `ht_bookings` rows → **two real iHOTEL `จอง` bookings**,
one of which nobody releases before its 2h deadline. The loyalty app worked
around this with a 20 s Redis lock — a timing heuristic, not a guarantee.

Migration **093** (`ht_channel_idempotency`) closes it properly. Migration 076's
`(book_channel, book_ext_ref)` natural key does NOT apply here: a loyalty hold
has no channel-native booking id at request time — the id the app knows is the
one we mint. What the app CAN supply is a client-generated key.

**It is a HEADER, not a body field.** The request body is a locked snake_case
contract the two systems agree on field by field, and a `idempotencyKey` member
would both break that shape and put transport policy inside booking data. It is
also where every client library already looks, including the loyalty app's own
backend (`services/idempotency.rs`).

| case | response |
|---|---|
| no `Idempotency-Key` | unchanged behaviour; no row is written |
| first request with key K | **201**, the hold is created, K records the response |
| retry of the SAME request with K | the stored response, **verbatim** — same status, same body, same `pms_booking_id` — plus `Idempotency-Replayed: true`. No second hold. |
| a DIFFERENT request with K | **422** `{"success": false, "error": "Idempotency-Key '…' was already used for a different booking request; …"}` |
| two identical requests at once | serialised — exactly one hold; the loser replays the winner's response |
| key present but blank / with spaces / > 255 chars | **400** (a broken key is loud, never a silent opt-out) |
| retry more than 24 h later | treated as a first-time request (TTL) |

Key facts a caller needs:

* **Any printable-ASCII string, 1..=255 characters**; a UUID v4 per booking
  attempt is the intended usage. Keep the key across retries of the SAME
  attempt; mint a new one for a new booking.
* **The key space is per caller and per property.** `idem_caller` is the
  SHA-256 of the presented bearer, so rotating `LOYALTY_CHANNEL_TOKEN` starts a
  fresh key space (a rotated token is a different client), and the row lives in
  the property's own database — which is correct, because a retry always targets
  the property the original request did.
* **"Different request" is judged on a canonicalised fingerprint**, not raw
  bytes: property, room type, both dates, guests, trimmed guest name and phone,
  trimmed membership id, payment plan. Re-serialising the JSON with different
  whitespace or key order, or sending `" 3 "` where the first attempt sent
  `"3"`, still REPLAYS. Changing anything that changes what gets booked is 422.
* **Errors are never cached.** A request that failed (no room available, bad
  dates, HF Ville writes disabled) frees its key immediately, so the client may
  retry the same key once the cause is fixed.

How the concurrent case is actually safe, since it is the part that is easy to
get wrong: the reserving `INSERT` runs inside a transaction that is held open
across the whole create. A simultaneous duplicate blocks on the uncommitted
unique-index entry (PostgreSQL speculative insertion) until the winner commits,
then finds nothing to insert and reads a COMPLETE row. There is no advisory
lock and no Redis, and the stored response commits in the same transaction that
reserved the key — so a rollback loses both, never one without the other.

**Residual gap, documented rather than papered over:** the hold and the
idempotency record are two transactions (the hold rides `BookingService::create`,
which owns its own). If the process dies in the gap between them, the hold is
committed and the key is not, so a retry re-enters as a fresh request and creates
a second hold — exactly the pre-existing behaviour, not a regression. Closing it
would mean threading the key through `BookingService::create` as a second
`book_ext_ref`-style natural key; that is a deliberate follow-up, not something
to bolt on.

Implementation: `service::channel_idempotency` (policy, fingerprinting, the
reservation guard) + `repository::channel_idempotency` (SQL) + the keyed branch
of `routes::channel::create_booking`. Integration tests:
`hotel-backend/tests/test_channel.rs::hold_create_is_idempotent_per_key` and
`::concurrent_identical_hold_creates_produce_one_hold`.

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

### Checking an app booking in — which app the desk uses

A booking made in the guest app exists in BOTH systems from the moment the hold
is created (previous subsection), so the desk can check the arriving guest in
from either one. The two paths are not equivalent, and the difference is
invisible at the counter — which is why it is written down here.

| | iHOTEL (`FormCheckIn`) | our app (Task B7a: reservations list / reservation detail / room board `จองแล้ว` → **เช็คอิน**) |
|---|---|---|
| creates | `HT_CheckIn_H` + `HT_CheckIn_Ds` directly | `ht_checkins` with `cin_book_id` set, then the `create_check_in` writeback mirrors it |
| booking link | `HT_CheckIn_H.Cin_Book_ID`, arrives canonical via the CT sync mapper | written in the same PG transaction as the stay |
| `ht_bookings.book_status` | `เข้าพัก` → mapped to `checked_in` by the sync | `checkedin`, written by `set_booking_checkedin` |
| app-deposit signposts | appear once the sync round-trip completes | appear immediately |

**Both are supported and neither is being removed** (ADR 0002 — iHOTEL is not
being decommissioned, and per ADR 0003 no capability reception has today may
become unreachable). The channel API already tolerates both spellings of the
checked-in state for exactly this reason, so a late `payment_verified` retry for
a guest the desk checked in through iHOTEL replays instead of answering 409.

**Which one the desk should use for an app booking: ours.** Not because iHOTEL
is wrong, but because the link is immediate there and the deposit signpost is
the whole point:

* An app guest's deposit is **not mirrored** (previous subsection), so iHOTEL's
  own check-in screen shows `0` with nothing to explain it. Our from-reservation
  check-in carries the `AppDepositNotice` into the check-in modal, the printed
  registration slip, the folio, the payment dialog and the checkout modal.
* The link is what every one of those signposts resolves through
  (`GET /api/checkins/:id/deposits` joins `ht_bookings` via `ci.cin_book_id`).
  Checking the same guest in as a **walk-in** from our room board leaves
  `cin_book_id` NULL, and then the notice never appears at all — which is the
  gap B7a closed.
* An iHOTEL check-in still ends up correct; the signposts simply lag by the
  sync round-trip, and the booking-linked state is only as good as what the CT
  mapper carried back.

Two consequences worth saying out loud at go-live:

* **The desk deposit field is not the booking deposit.** Our check-in modal
  leaves `เงินมัดจำที่รับที่เคาน์เตอร์` BLANK for a from-reservation check-in,
  deliberately: `ht_bookings.book_deposit_amount` is money in the bank,
  `ht_checkin_rooms.cr_dep_amount` (→ legacy `HT_CheckIn_Ds.Cin_Room_Dep`) is
  money in the drawer, and pre-filling one from the other would refund the
  guest in cash at checkout for a transfer they made in the app.
* **Multi-room app bookings still go through iHOTEL.** The single-room guard in
  `CheckInService::check_in_to_booking` rejects them with a Thai-facing message
  saying so; the loyalty channel creates single-room holds, so this only bites a
  desk-grown booking.

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

### Two facts that fix the ordering

**1. There is one flag and it opens both properties.** The estate program
board's decision **P1** wants the first live channel at **HF Ville** (lower
volume, ~30% OTA), with HF Hotel following after two clean weeks. There is one
`backend` service and one `LOYALTY_CHANNEL_ENABLED`, and
`routes::channel::channel_service_for` gates only *Ville* mutations (on
`HFVILLE_WRITES_ENABLED`) — **HF Hotel has no second gate.** So the flip makes
`POST /api/channel/holds` live for `property=hf` in the same deploy, and a
channel hold writes a `จอง` into HF's shared legacy DB on creation.

> **The P1 canary is therefore a caller-side discipline, not a server-side
> gate.** During the canary the loyalty app must send only `property=hfville`;
> nothing in this PMS will stop an `hf` hold. HF Hotel reception must be in the
> loop for the same flip, and the HF-side agreement is "no `property=hf` calls
> yet", not "the surface is closed". If that is not good enough, add the gate
> first — a property allowlist in `channel_service_for` modelled on
> `HK_BRANCHES` is the smallest version.

**2. Nothing can be live-tested before the flip.** Every `/api/channel/*`
request answers 503 while the flag is off (see the 503 section above), so a
hold → checkout cycle is not merely inconvenient before the flip — it is
impossible. The flip must come *first*, and it is safe to put first: **the flip
alone writes nothing to the legacy DB.** A `จอง` appears only when the loyalty
app actually calls hold-create. Rollback is one line and ~10 min.

### Steps

1. `gh secret set LOYALTY_CHANNEL_TOKEN` / `LOYALTY_SERVICE_TOKEN`, then
   redeploy so `run-deploy.sh` writes the two secret files.
2. ~~Land the compose `secrets:` declaration~~ **DONE** (step 2 of
   *Provisioning* above): `docker-compose.yml` carries both top-level
   `secrets:` definitions and both entries under the `backend` service (PR
   #298), so step 1's redeploy is all that is left. **Acceptance (board item B4): the backend startup line reads `token
   set: true`** — not "an HTTP call returns 503", which it does either way.
3. `gh variable set LOYALTY_APP_URL` and verify the stay hook against a linked
   test guest (checkout one real stay, confirm the points transaction).
   Acceptance: startup line reads `stay hook configured: true`.
4. Confirm `HFVILLE_WRITES_ENABLED=true` (ADR 0002 Ship-B gate) by reading its
   current value — Ville mutations need it and it is a separate knob. Confirm
   with **both** receptions that the loyalty app will send only
   `property=hfville` until P1's two clean weeks are up (fact 1 above).
5. **Flip the flag:** edit `docker-compose.yml`'s
   `LOYALTY_CHANNEL_ENABLED=${LOYALTY_CHANNEL_ENABLED:-false}` to `:-true` and
   merge — `docker-compose.yml` is in the workflow's `deploy` paths filter, so
   the edit ships itself (ADR 0004). Confirm the promoted image SHA, then that
   an authorised `GET /api/channel/availability` returns **200** (not 503) and
   an unauthorised one **401** (not 503). No legacy row exists yet.
6. **Now** the reception-coordinated live cycle, with reception watching iHOTEL
   and the rollback diff staged: one hold → payment-verified → release →
   checkout at **HF Ville**; verify the `จอง` appears and clears correctly
   (invariant #6).
7. Rollback is the same one-line diff in reverse (`:-true` → `:-false`, merge,
   ~10 min). It stops new holds but does **not** cancel holds already created —
   the 2h expiry sweep still runs, and any `จอง` already written stays written
   and must be cleared in iHOTEL by hand.
