# HF Ville co-equal writes — implementation plan (ADR 0002 completion)

Status: **Ship A implemented (default-off, safe). Ship B = GO to build, NO-GO to flip** until the gates below are met. Derived from a design + adversarial-verification pass (2026-06-26).

Goal: let the **new app write HF Ville** so iHOTEL and our app are co-equal writers on both sites (ADR 0002), **safely** — without ever writing Ville-intended data into the HF Hotel pool.

## Architecture (verified sound)
The API backend is one process holding `new_pool` + `Option<ville_pool>`. Each service (`CheckInService`, `BookingService`, `PaymentService`, `ShiftService`, …) holds its own `pg: PgPool` and transacts on it; `OutboxRepository`/`EventBus` are **stateless zero-sized** and write into the caller's tx → i.e. into whichever pool the service was built with. So:
- A **second service bundle bound to `ville_pool`** makes all its canonical writes + `writeback_jobs` INSERT + `pg_notify` land in `hotelville` → drained by the already-running `writeback-hfville` worker → Ville MSSQL. SSE uses a per-pool `PgListener` (`routes/events.rs`), so no singleton/clobber.
- HF Hotel already runs this exact two-writer model; Ville just needs the same plumbing.

## Ship A — DONE (default-off, closes a live hazard)
- `HFVILLE_WRITES_ENABLED` env flag (default false), surfaced as `villeWritesEnabled` in `/api/mode`.
- Global mutating-route **guard middleware**: rejects `POST/PUT/PATCH/DELETE` with `branch=hfville` (robust query parse) → **403** when the flag is off, before any handler — never touches `new_pool`.
- `ApiError::Forbidden(String)` → 403.
- Frontend `canWrite = hfhotel || (hfville && villeWritesEnabled)`: v2 lifts view-only on `canWrite`; **v1 classic dashboard** gates its room-action buttons on `canWrite` (this closed the live hazard: v1 showed `hotelville` room ids and its check-in modal POSTed into `hotelnew`, where shared integer PKs hit a *different real Hotel room*).

## Ship B — build behind the flag (do NOT flip yet)
1. **Per-site bundle** (`routes/mode.rs`): lift `site_id` to a param of `wire_services`; **hardcode** the hotel bundle to `"hfhotel"` and build a ville bundle in `with_ville` pinned to `"hfville"` + `ville_pool` (never `SiteConfig::from_env()` — the API process serves both sites; a stray `SITE_ID=hfville` would collide the per-site shift index → **R1**). Store `ville_services: Option<WiredServices>`.
2. **Resolver chokepoint** `resolve_write_services(branch) -> ApiResult<WriteServices>`: `Hfhotel` → hotel bundle + `new_pool`; `Hfville` → `Forbidden` if flag off, else ville bundle + `ville_pool`; `All` → `BadRequest`.
3. **Convert every mutating handler** to use `ws.<service>` / `ws.pool`. **Load-bearing completeness rule:** inside a converted handler, EVERY `state.new_pool` (and every pre-write read/helper — `generate_cin_no`, `build_check_in_writeback_context`, `get_booking_customer_id`, `check_in_billing`, `get_vat_percent`, room/customer lookups, inline `fetch_all`) must move to `ws.pool`. A miss compiles fine and writes a ville row into `hotelnew` (or reads Hotel's id/billing into a Ville write) → **silent corruption (R2)**. Repo-direct `tx.begin()` sites to convert: `new_rooms.rs:202,278,487`, `new_inventory.rs` (many), `new_checkins.rs:1190,1254`, `bookings.rs:490,535,580`, `new_room_types`, `new_rates`, `new_maintenance`.
4. **CI grep-gate**: fail the build if any mounted mutating handler still references `state.new_pool`.
5. No sqlx regen (pool is a runtime arg; `git diff hotel-backend/.sqlx/` must stay empty).

## NO-GO-to-flip gates (all required before `HFVILLE_WRITES_ENABLED=true`)
- [ ] Full `new_pool→ws.pool` sweep verified (incl. pre-write reads/helpers) + CI grep-gate green.
- [ ] **Shift-open path solved.** Today only `GET /api/shifts/current` is mounted; `open_shift`/`close_shift` exist but are unmounted, and the checkout/payment **round-bill gate hard-rejects when no shift is open** → Ville checkout/POS/payment would dead-end. Also `current_shift` is not branch-aware (returns the Hotel shift for `?branch=hfville`). **Prerequisite question: how are HF Hotel shifts opened in production today?** (no mounted route / no `bin` opener found) — resolve before assuming Ville can mirror it.
- [ ] **R3 prereqs on `hotelville`:** migration 016 NOTIFY trigger + `writeback_jobs` + `event_log` + 024 idempotency ledger present, and `writeback-hfville` draining (`NEW_DB_NAME=hotelville`). (Operationally true today; make it a pre-flip checklist item. Missing-table fails inside the tx → rollback, no partial write.)
- [ ] One **receptionist-coordinated e2e**: flip flag in a window, do one low-risk Ville write (room-status flip or test booking on a known test room), verify the full loop `hotelville` canonical → `hotelville.writeback_jobs` → `writeback-hfville` → iHOTEL on Ville MSSQL → CT watcher does **not** echo it back; avoid concurrent same-table iHOTEL save (MAX+1 id PK race); clean the test row via iHOTEL.

## Residual risks
- **R1** site-id collision (mitigated by hardcoding bundle site ids).
- **R2** conversion completeness (mitigated by CI grep-gate + flag stays off until verified).
- **R3** writeback prerequisite on hotelville (pre-flip checklist).
- **R4** `branch=all` writes — resolver rejects with 400; UI never sends it.

---

## Cashier rounds (shifts) — coexistence (2026-06-26)

**Live `HT_Round_Bill` investigation (read-only, via evergreen):** both sites run **active, disciplined rounds** — HF Hotel 4,777 rounds since 2021-12 (3 `round_by`), HF Ville 813 since 2024-12 (1 `round_by`); **~3 rounds/day** (~06:00/12:00/22:00 shift boundaries), durations ~6–10h, and **exactly one open round per site at all times** (`open_now=1` both). So: mimic iHOTEL strictly — **no gate leniency**; one-open-per-site matches `ht_shifts`. `round_no` is unused (NULL) → key on legacy `id`; `round_by` is a shared "Admin" account (coarse attribution).

**Captured write shapes** (`COMPAT_CHEATSHEET.md` §946-956, §3.20-3.21, from `FrmDueBill.cs:1653/1670`):
- Open: `INSERT HT_Round_Bill (id=get_id [MAX+1], round_start=now, round_price=<float>, round_by=loginName)` (round_end NULL)
- Close: `UPDATE HT_Round_Bill SET round_end=now, round_by=<emp> WHERE round_end IS NULL`
- Gate: `SELECT id FROM HT_Round_Bill WHERE round_end IS NULL`; `get_id` collision risk rated **Low** (only one open at a time).

**(1)+(3) — SHIPPED (read-only on iHOTEL):** `bin/sync.rs::sync_round_bills` polls `HT_Round_Bill` (open + last-2-days) and upserts `ht_shifts` per-site (`shift_no = shift_legacy_round_id = legacy id`, Thai→UTC, runtime sqlx so no `.sqlx` churn, closed-before-open ordering, shadow-aware, never aborts the tick). `GET /api/shifts/current` is now branch-aware. iHOTEL is the source of truth; our gate follows. This unblocks our-app checkout/payment at both sites.

**(2) — TODO (writes iHOTEL's live shared table):** writeback recipe for round open/close (shapes above) + mount open/close routes + per-site routing (needs the Ship-B bundle for Ville). **Open design point:** our open must allocate `id` without colliding with iHOTEL's `MAX+1` (mitigated by one-open-at-a-time + check-then-write against the synced open round). Gated + a one-time coordinated round test before enabling.
