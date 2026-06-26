# Spike: Encapsulate hotel logic in the backend; frontend = display + interaction

**Date:** 2026-06-27 · **Status:** recommendation (not yet scheduled) · **Trigger:** the
`/v2/rooms` room-status bug (statuses didn't match iHOTEL or the classic dashboard).

## Question

Should we move room/hotel **business logic** out of the frontend and into the
backend API, so the frontend is responsible only for **display and interaction**
— not for deriving room state, computing money, or encoding hotel rules?

## TL;DR — Yes, strongly.

The room-status bug fixed today is a textbook symptom: the **derived state lived
in the frontend**, two UIs derived it differently, and the backend served a raw
(stale) column. Move all *derivation/computation* to the backend and have it
return **display-ready view-models**; the frontend maps those to pixels. Do it
**incrementally** — the layered backend (`domain/ → repository/ → service/ →
routes/`) already gives derivation a natural home. Start with the room endpoint
consolidation, which is already half-done and directly prevents a recurrence.

## Evidence — hotel logic currently in the frontend

1. **Room status is derived in ≥4 frontend places, and they diverged.**
   - `app/page.tsx::getRoomStatus` (classic) computes status from raw legacy
     fields (`Room_Use`/`Room_Book`/`Room_Clean`/`Room_Manternace`) + a separate
     `/api/rooms/checkouts-today` call + a **"checkout today AND local hour ≥ 6"**
     rule — all client-side.
   - `lib/v2/status.ts::roomStatusView` (v2) had its *own* precedence and a
     `isClean===false → dirty` rule.
   - Also re-implemented in `components/RoomGrid.tsx`, `components/v2/RoomActionSheet.tsx`.
   - They disagreed → occupied rooms (402–405) showed "available", and every
     free room showed "รอทำความสะอาด". The backend wasn't the source of truth
     for *status*, only for *rows*.

2. **Billing math runs in the client.** `components/modals/CheckOutModal.tsx`:
   `nights = Math.ceil(dateDiff)` (min 1) `× ratePerNight = totalAmount`. Rate
   selection (weekday/weekend/special), rounding, and VAT logic live in JS across
   `billing/`, `BookingForm`, `PaymentModal`, document templates — none of it
   server-authoritative.

3. **Date / availability / timezone rules are scattered.** The "after 6am"
   checkout window, `isSameStoredDay`, and the `Asia/Bangkok` / fake-`Z`
   normalization are repeated across many components (`lib/v2/status.ts`,
   `StayTimeline`, `RoundReport`, calendar, reservations…).

4. **Two parallel room endpoints with different semantics.** `/api/rooms`
   (canonical, camelCase, *was* a stale stored column) vs `/api/rooms/board`
   (legacy-shaped, live-derived). The frontend had to know which to call and how
   to interpret each. This split *is* the bug surface.

## Cost of the status quo

- **Correctness/drift:** rules duplicated across two UIs inevitably diverge →
  bugs. The backend owns *data* but not *derived state*, so each client re-derives.
- **2× surface:** classic + v2 each re-implement the same rules; every change is
  done (and can be gotten wrong) twice.
- **Not testable / not reusable:** pricing, occupancy, and checkout-window rules
  in JS can't be unit-tested with the domain or reused by reports/exports, and a
  client can compute a "wrong" total with nothing to enforce the right one.
- **Coexistence parity is harder:** matching iHOTEL exactly (e.g. its
  `Room_Clean='yes' → cleaning`, MAX+1 ids, the 6am window) must currently be
  replicated in *every* UI. Centralizing makes parity a **backend** concern,
  asserted once with tests against the decompile recipes.

## Target architecture

- **Backend returns view-models (DTOs) with derived, display-ready state:**
  resolved `status`, computed totals/folio, validation verdicts, flags. The
  derivation lives in `domain/`/`service/` (routes stay thin, per
  `docs/architecture.md`).
- **Frontend renders + interacts only:** layout, responsive, animation, form UX,
  optimistic updates, and *pure presentation* (currency/date **formatting**,
  i18n labels, status→tone color maps). No domain derivation, no money math, no
  availability rules.
- **Litmus test for "where does it go?":** *Would two clients compute it
  differently, or does it affect correctness/money/parity?* → backend. *Is it
  purely how something looks?* → frontend.

## What to move (priority)

1. **Room status — START HERE (half-done).** `/api/rooms` now derives live status
   server-side (today's fix). Next: consolidate the two room endpoints into one
   canonical live endpoint, delete `getRoomStatus`/`roomStatusView` derivation
   from the classic + v2, and have both render the backend's `status`.
2. **Billing/folio totals:** a server-computed checkout/folio endpoint (nights,
   rate tier, VAT, rounding). `CheckOutModal`/billing *display* it. Extends the
   server-side money already in `ht_payment_ledger` + the round report.
3. **Booking validation & availability** (overlap, date rules): a server
   `validate`/`quote` endpoint; forms call it instead of computing locally.
4. **Booking/checkin status mappings** (`bookingStatusView`, etc.): backend
   returns status (+ optional label); frontend keeps only the tone map.
5. **Timezone normalization:** backend returns proper instants or pre-formatted
   Thai values; frontend stops the fake-`Z` gymnastics.

## What stays in the frontend

Pure presentation: currency/number/date **formatting**, i18n strings, a thin
`status → color/tone` map, layout/responsive/animation, optimistic UI, and
client-side form *hints* (with the server as the authority).

## Risks / tradeoffs

- More DTO shaping + a chattier/purpose-built API surface; mitigate with
  view-specific endpoints rather than generic CRUD + client assembly.
- Migration spans two UIs; do it **endpoint-by-endpoint**, keeping both working.
- Don't over-centralize genuine presentation (label/tone). Keep routes thin —
  derivation belongs in `domain/`/`service/`, not fat handlers.

## Phased plan (low-risk, each phase shippable)

- **Phase 0 (done):** room status derived live in `/api/rooms`.
- **Phase 1:** one canonical room endpoint (live status + flags); remove
  `getRoomStatus`/`roomStatusView` derivation from both UIs; delete the redundant
  `/api/rooms/board`. *Directly prevents this class of bug.*
- **Phase 2:** server-computed folio/checkout total; stop client billing math.
- **Phase 3:** server-side booking validation/availability (`quote`/`validate`).
- **Phase 4:** retire remaining legacy-shaped endpoints + frontend interpretation;
  standardize on canonical view-models, verifying iHOTEL parity each step.

## Recommendation

Adopt **"backend owns domain logic and returns view-models; frontend renders."**
Schedule **Phase 1** next — it's already half-built and removes the duplicated
room-status derivation that caused today's incident.
