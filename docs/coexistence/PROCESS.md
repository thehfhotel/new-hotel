# Coexistence audit process

These are the processes adopted on 2026-05-13 after the post-mortem on the
`ht_checkins` single-room schema gap. Their purpose is to ensure that any
PG canonical schema, sync mapper, read path, or writeback recipe stays
**aligned with the iHOTEL legacy app across all four layers** — schema,
sync, read, write — so that the two apps can operate against the same
data without divergence.

## Why this exists

The multi-room check-in gap was rediscovered three months after the
schema decision was already shipped. The root cause was **not** that
references were ignored — the cheatsheet, FEATURE_MAP, and SCHEMA.sql
didn't yet exist when the schema landed (2026-02-06, commit `f446341`).
The deeper failure mode was that the references arrived later
(2026-05-11, commit `3cc0bee`) without triggering a schema re-audit, and
that downstream symptoms ("Book_Room_Num hardcoded 1", "mark_clean
single-room blind spot") were filed as recipe-level MEDs rather than
escalated to the source-of-truth schema.

The processes below address each link in that chain.

## P1 — Pre-schema legacy-mapping checklist

Every migration that introduces a new `ht_*` canonical table MUST come
with a row in [`CARDINALITY_MAP.md`](CARDINALITY_MAP.md) containing:

- The legacy counterpart table(s) in MSSQL `HT_*` (or "none" with
  rationale)
- The cardinality relationship: `1:1`, `1:N`, `N:1`, `N:N`, or `N/A`
- Source of truth: `PG canonical`, `MSSQL legacy`, or `shared` (with
  notes on conflict resolution)
- Sync mapper path (file + function) if applicable, or "none"
- Read path (route + query) — at least the primary consumer
- Write path (recipe or service) — at least the primary writer

A PR that adds a `CREATE TABLE ht_*` without updating
`CARDINALITY_MAP.md` should be blocked by review (and eventually by CI).

The lesson: `ht_booking_rooms` got a junction because the booking UX
made multi-room obvious. `ht_checkins` was modeled later under the
"the room they sleep in tonight" mental model. Forcing a written
cardinality column for every table makes asymmetric assumptions
visible to the reviewer.

## P2 — Re-audit when reference docs land

When new authoritative reference docs land (cheatsheet, FEATURE_MAP,
SCHEMA.sql regeneration, decompile refresh), a coexistence re-audit
task MUST be filed within the same PR. The audit's job is to diff the
new reference against the current schema/mappers/queries and surface
gaps.

The lesson: `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Ds`" "one row per **room** in the check-in"
says explicitly that `HT_CheckIn_Ds` is one row per room (a single check-in
can cover multiple rooms). This landed on 2026-05-11 and triggered no schema
re-audit. The gap was caught only when the dashboard migration
2026-05-11/12 surfaced the symptom.

### P2.1 — Rust changes that depend on a legacy-mssql migration must apply in the same change-window

When a Rust sync mapper, writeback recipe, or service-layer code change
depends on a legacy-mssql migration (Change Tracking enablement, PK
addition, column expansion), the migration **MUST be applied in the
same change-window as the Rust deploy**.

The lesson: Track E1's `GuestRegistryMapper` (mirrors
`HT_CheckIn_Other_People` rows into `ht_guest_registry`) shipped in
`v2.63.12` on 2026-05-12. It depends on Change Tracking being enabled
on `HT_CheckIn_Other_People`, which requires
`migrations/legacy-mssql/022_phase5e_other_people_rooms_cancel.sql` to
be applied first. The migration was not applied until 2026-05-13 —
a 14-hour gap during which the `bin/sync.rs` watcher emitted a
`Change tracking is not enabled for table HT_CheckIn_Other_People`
error log roughly **once per second on both sites**, and
`ht_guest_registry` did not accrue companion-guest rows. TM.30
immigration reporting under-counted foreign guests for the duration.

#### Runbook template for legacy-mssql migration apply

[`RUNBOOK-mssql-022-apply.md`](RUNBOOK-mssql-022-apply.md) is the
established template for future legacy-mssql migration apply runbooks.
The structure to copy:

1. **Status banner.** Applied / Unapplied on each site, with date.
2. **Why this is needed.** Cite the dependent Rust commit/version and
   the per-second error log message (so future operators can grep
   their alerts).
3. **Pre-flight checks.** Receptionist coordination window, backup
   verification, locking-impact estimate (Sch-M lock duration).
4. **Apply procedure.** The `sqlcmd` invocation (or equivalent),
   including the `.rollback.sql` sibling path.
5. **Post-apply verification.** `sys.change_tracking_tables` SELECT
   confirming CT is enabled; tail of `bin/sync.rs` log confirming the
   per-second error stopped.
6. **Rollback.** Reference the `.rollback.sql` and any cleanup of
   `ht_*` rows that mistakenly accrued before apply.

The rule of thumb: if a Rust commit lands that REFERENCES a
legacy-mssql migration in its commit message or CHANGELOG, the same PR
must include (or link to a same-day-applied) runbook. Never ship a
mapper "expecting" a migration to be applied later — the lag manifests
as silent under-counting plus log spam, and the human operator may not
notice for hours.

## P3 — Promote spike captures to fixtures

Every raw spike capture under `docs/legacy-spike/raw/` MUST be
promoted to a regression fixture on the day it's archived. The fixture
exercises the full end-to-end path that the capture represents (state
setup → action → assertions on resulting MSSQL state).

The lesson: `walkin3-20260424-100000/07-events.txt` contained a 2-room
walk-in capture. `booking-checkin-20260424-101838/07-events.txt`
contained a 2-room booking-linked check-in. Both went un-promoted to
tests for three weeks. The Apr 24 open question in
`docs/legacy-spike/findings.md` §"7. What we still don't know" ("Whether multi-room
check-ins use the same flow") was already answered by the captures
themselves — the answer just never made it to writing or testing.

## P4 — Multi-layer end-to-end fixture template

For every cross-system flow, a fixture must exercise ALL four layers:

1. **Setup:** insert canonical PG state
2. **Write:** trigger writeback recipe → MSSQL
3. **Verify MSSQL:** check legacy table rows directly (or against a
   recorded fixture for byte parity)
4. **Reverse sync:** simulate a Change Tracking tick on the MSSQL row
5. **Verify PG:** confirm canonical state matches expectations
6. **Read:** call the dashboard/API endpoint that surfaces the state
7. **Verify display:** confirm the response reflects step 5

A fixture that only exercises 1–3 (typical writeback test today) can
ship a feature that passes CI but breaks coexistence. The cardinality
map ensures the fixture knows how many entities to assert against at
each layer.

## P5 — Cardinality matrix as source of truth

[`CARDINALITY_MAP.md`](CARDINALITY_MAP.md) is the single source of
truth for cross-system mapping. Every coexistence audit (this one and
future passes) reads it first.

When a finding identifies a cardinality bug (like the `ht_checkins`
one), the fix MUST update the matrix as part of the remediation PR.
The matrix is never the trailing artifact — it's the lever.

## CI hook proposal (design — not yet implemented)

A pre-PR check that:

1. Lists every `CREATE TABLE ht_*` in migrations added in the PR
2. Requires a matching new row in `CARDINALITY_MAP.md`
3. Lints the row: legacy counterpart cited, cardinality is a valid
   value, sync mapper file exists if claimed
4. Fails the PR if any check is missing

To be filed as a separate engineering task once the audit reveals
priorities.

## Audit cadence

- **On schema PR:** P1 enforced.
- **On reference-doc PR:** P2 triggered automatically.
- **On every quarterly engineering review:** full cardinality matrix
  walk-through with cross-team owners.
- **On observed coexistence divergence (e.g. ihotel vs our app shows
  different state):** ad-hoc audit on the affected layer + the matrix
  row.
