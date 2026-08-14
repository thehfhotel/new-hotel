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

Secret-file hydration entries exist (`loyalty_channel_token`,
`loyalty_service_token` in `secrets.rs::SECRET_FILE_MAP`); the
`docker-compose.yml` `secrets:` block and deploy-payload wiring are
**deliberately deferred** to the coordinated go-live (declaring a compose
secret whose file the deploy payload doesn't write yet would break the stack
start). Until then, plain env vars are the provisioning path.

**The flag flip is NOT "just config"** (coexistence invariant #6): a channel
hold writes into the shared legacy DB (as a normal booking `จอง`) the moment
it is created. Enabling the channel requires a reception-coordinated live
verification, same as every other dark-shipped legacy write.

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

1. Provision `LOYALTY_CHANNEL_TOKEN` / `LOYALTY_SERVICE_TOKEN` (+ compose
   `secrets:` wiring + deploy-payload `.secrets` entries if using files).
2. Set `LOYALTY_APP_URL`; verify the stay hook against a linked test guest.
3. Reception-coordinated live test of one hold → payment-verified →
   checkout at HF Hotel; verify the `จอง` appears/clears correctly in
   iHOTEL (invariant #6).
4. Flip `LOYALTY_CHANNEL_ENABLED=true` (HF Hotel). HF Ville additionally
   waits on `HFVILLE_WRITES_ENABLED` (ADR 0002 Ship-B gate).
