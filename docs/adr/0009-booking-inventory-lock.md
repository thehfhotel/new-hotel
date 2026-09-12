# ADR 0009 — Booking-inventory lock: an idle guard transaction, not a threaded one

**Status:** Accepted (2026-09-11, B8e / PR #311)
**Scope:** `hotel-backend/src/repository/inventory_lock.rs`, `service::channel::create_hold`,
`service::booking::create`

## Context

`pick_free_room` (channel) and `room_is_available` (desk form) are plain SELECTs, and the
INSERT that consumes what they found runs in a *later* transaction — with a guest
match-or-create round trip in between. Two writers could pick the same last room and both
commit. Nothing in the canonical schema rejects the second write: `uq_ht_br_bookroom` only
stops one booking listing a room twice, and channel rows on the loyalty side carry
`room_id = NULL`, so that repo's `EXCLUDE USING gist` cannot cover them either.

We need mutual exclusion that spans **pick → insert**, across two paths that do not share a
transaction.

## Decision

Take a **transaction-scoped advisory lock** (`pg_advisory_xact_lock`) on a transaction that a
guard object owns, holds open and **never writes through**, and keep that guard alive across
the caller's own (separate) create transaction.

Key: `(classid = "BKIV" as i32, objid = fnv1a32(property))` — one lock per property. Taken by
exactly three paths: `service::channel::create_hold`; `service::booking::create` when
`CreateBookingCommand::inventory_lock` is `Some`; and, since B8g (#325),
`service::booking::modify` when `ModifyBookingCommand::inventory_lock` is `Some` **and** the
edit changes the booking's room set.

Since B8h, `modify` takes it *inside* its own transaction, after locking the booking row —
the predicate and the legacy promote decision are then read from one snapshot instead of two.
That inverts the lock order relative to the other two paths, and is sound only because no
holder of this advisory lock ever locks a pre-existing `ht_bookings` row. The argument, and
the change that would invalidate it, live in the "Lock order" section of `service::checkin`'s
module doc.

## The alternative we did not take

The structurally correct design is to make `pg_advisory_xact_lock` the **first statement of
the same transaction that does the INSERT**, threading `&mut Transaction` through
`pick_free_room` and `inventory_snapshot`. It has no idle transaction at all, so none of the
caveat below applies.

It was rejected **for now** on blast radius, not on merit: `BookingService::create` owns its
transaction, and letting callers pass one in changes the signature for the desk form, the OTA
bridge, the loyalty channel, the scheduler and every test that builds a
`CreateBookingCommand` — a refactor several times the size of the fix it carries. Revisit when
another change is already opening that seam.

## Consequence to remember

**The guard's transaction sits IDLE while the caller works on other connections**, so a
server-side `idle_in_transaction_session_timeout` would terminate that backend mid-critical-
section and silently release the lock. There would be no error near the caller: the pick and
the insert would simply stop excluding, and the double-sell would return looking like a
heisenbug.

PostgreSQL ships that setting as `0` (disabled) and this repo never sets it, on the server or
per role. Before ever enabling it:

- it must exceed the whole create span (hundreds of ms), **and**
- prefer moving to the threaded-transaction design above instead.

The same caveat is repeated as a loud comment at the top of
`repository/inventory_lock.rs`; keep the two in step.

## Scope limit (do not over-read the lock)

Only the three paths named above take it. Walk-in check-in, room change, stay extension, a
booking edit that only re-dates an unchanged room set, and the CT sync mappers all still race
exactly as they did before. What keeps the *channel* clear of those is the B8e **last-room floor** (L2), which
holds back a buffer of sellable rooms, not this lock. Widening the lock to those paths is a
separate decision — they take real row locks inside their own transactions.

## Kill switch

`BOOKING_INVENTORY_LOCK_ENABLED` (compose-owned, **default on**) turns the lock into a no-op
guard. Opposite polarity to every ship-dark flag in `config.rs`, because this one closes a
window rather than opening a legacy write. It is an incident tool: setting it false re-opens
the hold-vs-hold and hold-vs-desk double-sell (races 2.1 and 2.3 of the B8 overbooking
analysis, in hf-tasks).
