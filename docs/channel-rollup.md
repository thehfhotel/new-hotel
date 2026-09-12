# Channel rollup — where our bookings come from

Direct-booking program **D3**
(`hf-tasks/tasks/direct-booking.md`, plan `direct-booking-designs/program-plan.md`).

`ht_bookings.book_channel` (migration 076) and `ht_bookings.book_source` have been free
text wired to no reporting route, so "what share of our business is direct?" was
unanswerable from the PMS. **B6** (PR #300) put `bookChannel` on the booking DTOs so
reception can *see* it per row. This route makes it *countable*.

Read-only. No outbox, no writeback, no legacy MSSQL touch, no new table, no flag.

## Surface

| Method + path | Handler | Change |
|---|---|---|
| `GET /api/reports/channel-rollup?from=&to=&branch=` | `routes::new_reports::get_channel_rollup` | **NEW** |

Gating and site handling are identical to every other `/api/reports/*` route:

* **Auth** — the blanket `require_auth` cookie-session layer wrapping the whole
  `build_new_routes()` router (`main.rs`). No per-route permission gate; no report
  route has one. `require_auth` is a no-op while `AUTH_ENABLED=false`.
* **Site** — `?branch=hfhotel` (default) | `hfville`. HF and HF Ville are separate
  logical PG databases, so the branch selects the connection pool; it is not a row
  filter. `branch=all` resolves to HF Hotel, matching the other reports — there is no
  cross-site union anywhere in this file.

### Query parameters

| Name | Type | Default | Notes |
|---|---|---|---|
| `from` | `YYYY-MM-DD` | `to - 29 days` | Inclusive. Matched against the booking's **check-in (arrival) date**. |
| `to` | `YYYY-MM-DD` | today in Bangkok (GMT+7) | Inclusive. |
| `branch` | `hfhotel` \| `hfville` \| `all` | `hfhotel` | Pool selector. |

Both dates are optional — a bare `GET /api/reports/channel-rollup` answers for the last
30 days rather than `400`. A malformed date, a reversed range, or a window wider than
**366 days** is a `400`. The cap exists because this is a dashboard read: without it a
refresh scans the whole booking history.

Today defaults roll at **Thai midnight**, not UTC midnight, so "the last 30 days" does
not silently mean "yesterday" for the first seven hours of each Thai day.

### Response

```jsonc
{
  "success": true,
  "from": "2026-08-12",
  "to": "2026-09-10",
  "buckets": [
    { "key": "app",     "kind": "app",     "label": "App",
      "bookings": 12, "cancelled": 1, "holdsExpired": 1, "roomNights": 30, "grossRevenue": 45000.0 },
    { "key": "agoda",   "kind": "ota",     "label": "Agoda",     "...": "..." },
    { "key": "ota",     "kind": "ota",     "label": "OTA",       "...": "..." },
    { "key": "direct",  "kind": "direct",  "label": "Direct",    "...": "..." },
    { "key": "unknown", "kind": "unknown", "label": "Unknown",   "...": "..." }
  ],
  "totals": { "bookings": 260, "cancelled": 41, "holdsExpired": 6, "roomNights": 612, "grossRevenue": 918000.0 },
  "directShare": 38.5,
  "directShareByBookings": 41.2,
  "directShareByRevenue": 36.9
}
```

Buckets come back in a stable order — app, then OTAs alphabetically, then direct, then
unknown — and a bucket with no bookings in the window is simply absent.

`directShare` is **(app + direct) ÷ all**, as a percentage rounded to one decimal, on
the **room-night** basis, which is what KPI **K5** is written against. The other two
bases are exposed alongside it rather than forcing the caller to pick. All three are
`0.0`, never `NaN`, on an empty window.

## What each number is, exactly

| Field | Source |
|---|---|
| `bookings` | `COUNT(*)` over `ht_bookings` with `book_checkin` in the window — **every status, cancellations included**. |
| `cancelled` | Of those, `book_status = 'cancelled'`. `cancelled / bookings` is the cancellation rate KPI **K7** wants. |
| `holdsExpired` | Of those cancellations, the ones with `book_hold_auto_released_at IS NOT NULL` — loyalty holds the expiry sweep **auto-released** because the payment window lapsed (migration 096, **B13**). A strict SUBSET of `cancelled`. Structurally `0` outside the `app` bucket: only a loyalty hold has a payment window to lapse. Note the deliberate name split: the column and the Rust field are named for the mechanism (`..._auto_released`), the wire field for the business question (`holdsExpired`), pinned together by an explicit `serde(rename)`. |
| `roomNights` | `SUM(book_nights × rooms)` over **non-cancelled** bookings. `book_nights` is the stored generated column (`book_checkout - book_checkin`); `rooms` is `COUNT(*)` over `ht_booking_rooms` for that booking, **floored at 1**. |
| `grossRevenue` | `SUM(book_total_amount)` over **non-cancelled** bookings, in baht. |

Two choices worth defending:

* **The room floor.** A booking whose rooms have not been assigned yet has zero
  `ht_booking_rooms` rows. Counting it as zero room-nights would quietly under-report
  every channel's forward book, so it counts as one room. The multiplier itself matters
  because multi-room stays exist — they are iHOTEL-created, since our app rejects
  multi-room walk-ins.
* **Cancelled bookings sell no nights and earn no baht,** so they are excluded from
  `roomNights` and `grossRevenue` while staying in `bookings`. Any other split makes one
  of the two KPIs unreadable.

### Why `holdsExpired` is a typed column, named for the auto-release, and shares the arrival-date basis

Two decisions, both deliberate (**B13**).

**Typed, not textual.** Before migration 096 a swept hold was indistinguishable from
any other cancellation except by matching the free-text `book_cancel_reason`. That
match is not merely brittle — it is *wrong*. The sweep writes `loyalty hold expired
(auto-release)`; the channel's own release endpoint writes `loyalty payment window
lapsed (channel release)`. Both sentences say the payment window ran out, yet only the
first is an expiry whose TTL we control — the second is the loyalty app handing a room
back for reasons of its own — a guest abandonment, which must never be counted as a TTL
expiry. Counting them together would inflate the exact rate B13 exists to read, and
would make a `HOLD_TTL` change look effective (or useless) for reasons that have nothing
to do with `HOLD_TTL`. So the sweep stamps `book_hold_auto_released_at` in the same
`UPDATE` that cancels, and this report counts that column. A reason-string rename can no
longer silently zero the number.

**Named for the event, not the verdict.** The column records an *auto-release*, because
that is the one act that sets it. The rejected `..._expired_at` spelling sat one letter
from migration 086's `book_hold_expires_at` and was also `TIMESTAMPTZ`, so a typo would
compile and return plausible timestamps — measuring the deadline instead of the event, in
the column whose whole purpose is measurement. It also keeps its meaning if a later
`HOLD_TTL` change alters what "expired" means. The metric stays `holdsExpired` because
that is the question being asked.

**Same window basis as everything else here.** Every figure in this report is
attributed by `book_checkin` (arrival), so a hold that expired in January for a March
stay is counted in March — not in January when it actually died. An expiry-time basis
(`book_cancelled_at`) would read more naturally on its own, but it would break the
property that makes the number useful: counted this way `holdsExpired` is a strict
subset of `cancelled`, over exactly the same rows, so `holdsExpired ÷ bookings` inside
the `app` bucket is a real rate whose numerator and denominator describe the same
population. Split the bases and that ratio silently compares two different sets of
bookings. If you need "holds that expired *during* a calendar period" — a different and
also legitimate question — that is a new field with its own basis, not a redefinition
of this one.

### Why `grossRevenue` is `book_total_amount` and not the reports' revenue column

The other five report routes share `CHECKIN_REVENUE_EXPR`
(`COALESCE(ci.cin_total_amount, ci.cin_rate_per_night × nights)`), which reads
`ht_checkins`. That column is unusable here:

* a **cancelled** booking has no `ht_checkins` row at all, and cancellations are half
  the point of this report;
* a booking that has **not arrived yet** has no folio either, so the forward book would
  read as zero revenue;
* `book_total_amount` is the column the OTA bridge writes the OTA gross into, which is
  what makes the commission arithmetic in **K6** possible.

### Why `roomNights` will not equal `/api/reports/occupancy`

It is a different question, not a discrepancy, and it cannot be fixed by reusing joins:

| | `/api/reports/occupancy` | `/api/reports/channel-rollup` |
|---|---|---|
| Table | `ht_checkins` | `ht_bookings` + `ht_booking_rooms` |
| Measures | **realised** room-nights, clipped to the window | **booked** room-nights, attributed whole to the arrival date |
| Cancellations | invisible (no check-in row exists) | counted |
| Walk-ins | included | included |
| Channel | not available (`ht_checkins` has no channel column, and `cin_book_id` is nullable) | the whole point |

A stay that starts on the last day of the window contributes one night to occupancy and
its whole length to the rollup. Report the two side by side; do not expect them to tie.

## Bucketing rules

Applied in order to the `(book_channel, book_source)` pair, both trimmed and
lower-cased, with an empty string treated as absent
(`service::reports::channel_rollup::classify`):

| # | Condition | Bucket |
|---|---|---|
| 1 | `channel = 'loyalty'` | `app` |
| 2 | `channel` set, and a direct spelling (`walkin`, `walk-in`, `walk_in`, `phone`, `direct`, `desk`) | `direct` |
| 3 | `channel` set, anything else | `ota`, keyed on the slug |
| 4 | no channel, `source = 'loyalty'` | `app` |
| 5 | no channel, `source = 'ota'` | `ota`, keyed `ota`, labelled "OTA" |
| 6 | no channel, `source` names a known OTA | `ota`, keyed on that name |
| 7 | no channel, `source` a known direct spelling (`walk-in`, `phone`, `line`, `online`, `website`, `email`, …) | `direct` |
| 8 | no channel, `source` absent / `legacy_app` / anything else | `unknown` |

Rules 1-3 and 5-6 are deliberately the same decision the reservations chip makes
(`bookingChannelView` in `components/v2/BookingChannelChip.tsx`), so a row that reads
"Agoda" at the desk counts as Agoda here. An **unmapped OTA slug still buckets as that
OTA** under rule 3 — a new OTA is a channel we have not labelled, not an unknown one;
only the display label falls back to the raw slug.

> The OTA display-label map exists twice — `OTA_LABELS` here and in
> `BookingChannelChip.tsx`. Keep them in step, or the weekly pack and the reservations
> list will name the same OTA differently. Folding them into one shared contract is a
> follow-up.

### Read `unknown` before you read `directShare`

The D3 brief defined direct as "`book_channel` null and `book_source` not ota". Rule 8
deviates: `legacy_app` and a null source go to **`unknown`**, not `direct`.

The reason is that `book_source = 'legacy_app'` is hardcoded by the CT sync mapper on
every booking it discovers in iHOTEL — and while daily ops still run in iHOTEL, that is
very nearly every production booking. iHOTEL records no channel anywhere (its
`HT_Book_H.Book_Sale` is written blank by our own byte-parity recipe and is not
populated by reception either), so those rows carry no provenance at all. Calling them
"direct" would report a direct share near 100% that means nothing.

So expect this route to answer, today, something close to *"unknown 95%"*. That is the
honest answer, and it is the finding, not a bug: **the direct share becomes measurable
as `book_channel` gets populated**, not before. A large `unknown` bucket is also a
visible bug report — an unrecognised `book_source` spelling lands there rather than
silently inflating `direct`, which is why rule 8's fallback is `unknown` and the direct
sources in rule 7 are a closed allowlist.

Reverting to the literal brief is a one-line change to rule 8 in
`hotel-backend/src/service/reports/channel_rollup.rs`.

## Where the code lives

| File | Role |
|---|---|
| `hotel-backend/src/service/reports/channel_rollup.rs` | `classify` (pure bucketing), `rollup` (pure fold), `load_channel_rollup` (the one grouped query) |
| `hotel-backend/src/routes/new_reports.rs` | `get_channel_rollup` — thin: pool, window, delegate |
| `hotel-backend/src/main.rs` | route registration |
| `hotel-backend/tests/test_channel_rollup_report.rs` | integration tests against the test PG database |

The SQL is a static string with bound dates via dynamic `sqlx::query()` — the same
idiom as `/api/reports/vat-summary` — so there is no `.sqlx/` offline entry to
regenerate.

## Neighbouring report routes

Documented here because these five had no doc page anywhere. All are branch-aware,
read-only, and live in `routes/new_reports.rs`.

| Method + path | Basis |
|---|---|
| `GET /api/reports/revenue?from=&to=&groupBy=day\|week\|month` | `ht_checkins`, `cin_status='checkedout'`, `CHECKIN_REVENUE_EXPR` |
| `GET /api/reports/occupancy?from=&to=` | `ht_checkins` overlapping the window, `cin_status IN ('active','checkedout')`; rooms from `ht_rooms_new` |
| `GET /api/reports/revenue-by-room-type?from=&to=` | as `revenue`, joined to `ht_rooms_new` + `ht_room_types` |
| `GET /api/reports/vat-summary?from=&to=&groupBy=` | as `revenue`, split inclusive at the branch VAT rate |
| `GET /api/reports/sales-by-customer?from=&to=&limit=` | as `revenue`, grouped by `ht_customers` |
| `GET /api/reports/rr4?...` | RR.4 / ตม.30 immigration export — `service::reports::rr4` |
