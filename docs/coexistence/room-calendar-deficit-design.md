# `ht_room_calendar` night-deficit remediation design (issue #273 remainder)

Status: **design only, no code written**. Companion to the closure/detection work
shipped in `60deef0` (business-key resolve + detection re-key, `observe_only: false`
now sitting behind the still-`false` `RECONCILE_MIRROR_PROBE_ENABLED` flag).

All data below is live, read-only evidence pulled 2026-07-31 via
`ssh evergreen` against both production databases. Raw key dumps used for the
diff live in `/tmp/calendar_deficit/` on the machine this was run from (not
committed — regenerate with the queries in this doc if needed).

---

## 1. What `ht_room_calendar` is for — impact if it stays short

**Nothing reads it today.** Confirmed by exhaustive grep across the backend and
frontend:

```
grep -rln "ht_room_calendar\|room_calendar\|RoomCalendar" hotel-backend/src app components
```

The only hits are the migration (`migrations/pg/039_create_ht_room_calendar.sql`),
`init-db/init-hotelnew.sql`, the sync mapper (`sync/mappers/room_calendar.rs`),
its regression tests, and the Phase 6-C probe/resolve code in
`scheduler/{mirror_probe,sync}.rs`. `routes/calendar.rs` and
`routes/new_calendar.rs` (382 + N lines, the actual endpoints the v2 booking
calendar and room grid call) have **zero** references to `ht_room_calendar` —
they reconstruct availability from `ht_bookings` + `ht_checkins` joins, exactly
as the mapper's own doc comment says:

> Read path: Out of scope for F1 — existing `routes/calendar.rs` /
> `routes/rooms.rs` keep working off the bookings+checkins reconstruction. A
> follow-up track will switch to canonical reads.

That switch never happened. `ht_room_calendar` is a write-only shadow
projection: Track F1 (audit 2026-05-13, T1 HIGH-4) built it because the
bookings+checkins reconstruction silently misses direct iHOTEL edits to
`HT_Room_Status` (mark-clean, walk-in, extend-stay, mid-stay room moves), but
the read-side migration to actually consume it was deferred and nothing has
picked it up since.

**User-visible impact of the deficit today: none.** No route, report, or v2 UI
page queries this table, so a missing night here cannot desync a screen a
receptionist looks at. The only consumer is the Phase 6-C mirror probe itself
(`scheduler/mirror_probe.rs` / `scheduler::sync::probe_room_calendar_business_key`),
which is dark behind `RECONCILE_MIRROR_PROBE_ENABLED=false` on every service —
so as of today the deficit is invisible even to the sync-health dashboard.

**This changes the priority calculus directly.** There is no live user harm to
stop; the only stakeholder is the future read-path migration this table was
built for, and the alert-noise budget once the mirror probe flips on (see §5).
Remediation here is *operability hygiene before enabling a new alert*, not an
active-bug fix — closer in urgency to `HT_Rooms_Cancel`'s coverage-floor case
than to the #278 receipt-payments money-adjacent gap.

---

## 2. Deficit characterisation (live, read-only, both sites, 2026-07-31)

### Numbers moved since the 2026-07-28 measurement in `mirror_probe.rs`'s doc
comment — the era floor is *derived*, and canonical's `MIN(rcal_date)` moved
significantly earlier on HF Hotel between then and now (a legitimate legacy
write landed with an old `room_date`, discussed below), which pulled far more
legacy history into scope. Restated cleanly:

| Site | Canonical floor (`MIN(rcal_date)`) | Canonical nights (business key) | Legacy nights in-era (floored, DISTINCT `room_no`+night) | Deficit (`missing_pg`) | Surplus (`missing_mssql`) |
|---|---|---|---|---|---|
| HF Hotel | 2025-04-13 | 1,529 | 4,542 | **3,137** | 124 |
| HF Ville | 2026-03-26 | 1,157 | 1,631 | **513** | 39 |

(Business-key definitions and SQL match `room_calendar_business_key_legacy_sql`
/ `ROOM_CALENDAR_BUSINESS_KEY_PG_SQL` in `scheduler/sync.rs` exactly — this is
the same comparison the closure arm already resolves on, just run by hand with
the exact key list dumped for a per-night diff instead of only the aggregate.)

### Clustering by month — the deficit is almost entirely pre-coverage history

HF Hotel `missing_pg`, by month:

```
2025-04   88     2025-11  180     2026-04  283
2025-05  100     2025-12  254     2026-05  297  (all ≤ 05-16)
2025-06  107     2026-01  330     2026-07    3  (all = 07-01)
2025-07  140     2026-02  333
2025-08  264     2026-03  296
2025-09  154
2025-10  308
```

HF Ville `missing_pg`, by month:

```
2026-03   35     2026-05  203  (all ≤ 05-13)
2026-04  273     2026-06    2  (06-04, 06-30)
```

**The cliff is exact and sited exactly where the mapper's own history starts.**
Migration 039 (`ht_room_calendar` table) and `sync/mappers/room_calendar.rs`
landed 2026-05-13; nothing before that date was ever forward-synced, and —
critically — **no backfill bin was ever written for it**. Every sibling
entity that needed one has one:

```
$ ls hotel-backend/src/bin/ | grep backfill
backfill_checkin_rooms.rs
backfill_customer_legacy_ids.rs
backfill_legacy_bookings.rs
backfill_legacy_checkins.rs
backfill_payment_ledger.rs
backfill_receipt_payments.rs
backfill_rooms.rs
```

`ht_room_calendar` is the only CT-mirrored entity in this repo that shipped a
forward-only mapper with an explicit "read path deferred" note and never got a
companion `backfill_room_calendar` bin. That gap is the root cause of 3,131 of
HF Hotel's 3,137 missing nights (99.8%) and 511 of HF Ville's 513 (99.6%).

One more texture worth recording: the pre-coverage legacy rows are almost all
literally the terminal `Check Out` / `Check-Out` marker (3,197 + 27 = 3,224 of
the HF Hotel pre-2026-05-13 rows are one of those two literals; **zero** are
`เข้าพัก` or `จอง`). Whether that's because `HT_Room_Status` genuinely only
kept the departure-day marker for old, fully-cycled stays, or iHOTEL prunes the
occupied-night rows behind a booking once it's long closed, wasn't chased
further — it doesn't change the remediation shape (the backfill re-drives
whatever legacy rows currently exist, verbatim, same as the live mapper does),
but it does mean a chunk of this backlog is inherently low-value: departure
markers for stays that closed over a year ago, for a table nothing reads yet.

### The residual — 5 nights, isolated, ~30 days old, not growing

Outside the pre-coverage cliff, exactly **5** nights are missing across both
sites, all landing on one of two dates roughly a month ago:

* **HF Hotel, 2026-07-01** (3 nights): rooms `A2-1`, `A3-3`, `A4-3` — **the
  same `Cin_No` `CH26-005947`** checked out all three simultaneously
  (legacy ids 52472/52474/52475, consecutive). A multi-room checkout, entirely
  dropped from `ht_room_calendar` while every other date for those same three
  rooms (before and after) landed fine, including under different `Cin_No`s.
  Not an FK-resolve defer: all three rooms already existed in `ht_rooms_new`
  well before this date.
* **HF Ville, 2026-06-04** (1 night): room `103`, `Cin_No CH26-001278`.
* **HF Ville, 2026-06-30** (1 night): room `201` — 4 legacy rows share this
  business key under different `Cin_No`s (`CH26-001420/1421/1422/1424`),
  consistent with the documented allocator-duplicate behaviour; the business
  key itself is still simply missing from canonical.

**Not growing.** The newest missing night on either site is 30–31 days old
(HF Hotel 2026-07-01, HF Ville 2026-06-30, against "today" 2026-07-31); zero
missing nights in the most recent 4 weeks on either site. The mapper has been
running clean since. This reads as a rare, already-closed occurrence of the
class below, not an active bug currently dropping events.

**Surplus side (`missing_mssql`) is separately explained and out of scope.**
All 124 HF Hotel surplus rows have `rcal_legacy_id IS NULL` (verified by
direct query) — app-authored tiles with no legacy counterpart, exactly the
"122 of 1420" figure the `MirrorProbe.mirror_filter` doc comment already
documents. Not a defect; the business-key comparison correctly counts them as
present-only-in-PG, and remediation here does not touch them.

---

## 3. Root cause

**Two distinct causes, not one:**

1. **The 3,131 / 511-row bulk (99.7% combined) — never-backfilled pre-coverage
   history.** Not a sync defect at all: the mapper has correctly captured
   every `HT_Room_Status` CT event since it started polling (2026-05-13 HF
   Hotel; Ville's floor sits earlier only because a stay's *nights* can predate
   the date CT started watching that stay's later edits). Nobody wrote the
   one-time catch-up job every other entity got. This is a **backfill gap**,
   not a **retention-outrun** bug.

2. **The 5-row residual — NOT the #278 payments class.** Issue #278's root
   cause was specific: `apply_receipt_upsert` requires the parent check-in to
   resolve and returns `Err` (correctly holding the watermark) when it can't;
   the parent took up to 19 days to backfill elsewhere, long enough for the
   receipt's own CT delta to age past the 2-day retention. `RoomCalendarMapper`
   is explicitly built differently — it does **not** defer on a missing
   booking/checkin FK (`sync/mappers/room_calendar.rs`, "Crucially we do NOT
   defer the entire row when the booking/check-in hasn't landed yet"); only a
   missing **room** defers, and all rooms involved in the residual 5 already
   existed canonically. So the parent-FK-retry mechanism is ruled out by the
   mapper's own design and directly confirmed by the room lookups.

   What's left is the general **silent-drop class** this repo has hit
   repeatedly (`docs/coexistence/sync-incident-log.md`: the 2026-05-18
   FK-defer incident, the 2026-07-27 global-watermark clobber, the 2026-07-06
   idempotency-gate gap) — some per-key CT apply step failed or was skipped
   without holding the watermark loudly enough, and by the time anyone
   noticed, the 2-day CT retention window had already closed it off from
   redelivery. The HF Hotel case (3 rooms, 1 `Cin_No`, consecutive legacy ids,
   single missing night each) is consistent with one multi-row iHOTEL
   transaction landing mid-tick and only partially surviving — the same shape
   as the 2026-07-27 incident, though the actual sync logs from a month ago
   have long since rotated out, so this can't be pinned to one specific
   mechanism with certainty. Given the tiny count (5 rows, both sites, 30 days
   silent since) this reads as a rare, closed occurrence, not a live pattern
   worth a separate hardening track on its own — the general silent-drop
   defenses already shipped (per-table watermark ceiling, gate_guard
   contract) are the standing mitigation.

**Conclusion: this is fundamentally the same shape as `backfill_payment_ledger`
and `backfill_legacy_checkins`, not `backfill_receipt_payments`** — a one-time
catch-up for a mapper that only ever ran forward, plus a handful of ordinary
sync stragglers the catch-up sweeps up for free because it re-drives by
current legacy state, not by replaying history.

---

## 4. Remediation design

### Shape: `bin/backfill_room_calendar.rs`, modelled on `backfill_receipt_payments.rs`

Closer to `backfill_receipt_payments` than `backfill_payment_ledger` in every
way that matters for this repo's conventions:

* **Legacy access is read-only** — one `SELECT` per candidate key (or a
  bounded batch), routed through `simple_query_with_timeout_pooled`
  (`MssqlOpKind::Read`), same as every bin post-#275/#274. No env flag needed:
  this bin never writes to MSSQL, so invariant #6 (new legacy writes ship
  dark) does not apply — same reasoning `backfill_receipt_payments`'s doc
  comment gives for skipping the `_ENABLED` flag entirely.
* **Re-drives the exact mapper path**, not a bespoke `INSERT`. Unlike
  `backfill_payment_ledger` (which calls a lower-level `mirror_payment_ledger`
  helper) or `backfill_receipt_payments` (which calls
  `apply_receipt_upsert` directly), the room-calendar mapper's write logic
  lives behind the `MssqlChangeMapper` trait, and `RoomCalendarMapper` is
  `pub`, its `apply()` method is the trait's `pub async fn`, and
  `MappableRow` is implemented for `tiberius::Row` — so the bin can call:

  ```rust
  RoomCalendarMapper.apply(&mut tx, ChangeOp::Insert, Some(&row)).await
  ```

  directly against a freshly-fetched `tiberius::Row`, using **exactly** the
  same code the live CT watcher runs on every `HT_Room_Status` insert/update.
  `ChangeOp::Insert` and `ChangeOp::Update` hit the identical `apply_upsert`
  branch in this mapper (the enum only forks on `Delete`), so the backfill
  does not need to know or care which case it is — every candidate key is
  driven through `apply_upsert`'s idempotent `ON CONFLICT (rcal_room_id,
  rcal_date) DO UPDATE`, which is naturally safe to call twice.

* **Work-list query**: unlike `backfill_receipt_payments` (which reads
  `ht_reconcile_log` because the payments arm is live and already recording
  rows), the room-calendar business-key probe writes only an **aggregate**
  sentinel row (`per_pk: false` — see `mirror_probe.rs`), never a per-night
  key. There is no per-key worklist to read from `ht_reconcile_log` even once
  the probe is enabled. The work-list must instead be derived directly from a
  legacy/canonical key diff, computed by the bin itself on each run:

  ```sql
  -- Legacy side (MSSQL), matching room_calendar_business_key_legacy_sql
  -- exactly, but returning KEYS not an aggregate:
  SELECT DISTINCT id, room_no, room_date, room_status, room_Details,
         room_Book_No, room_CheckIn_No
    FROM HT_Room_Status
   WHERE room_no IS NOT NULL AND room_date IS NOT NULL
     [AND CAST(room_date AS DATE) >= @floor]   -- era floor, see below
  ```

  ```sql
  -- Canonical side (PG) — the existing business-key set:
  SELECT r.room_no, c.rcal_date
    FROM ht_room_calendar c JOIN ht_rooms_new r ON r.room_id = c.rcal_room_id
  ```

  Diff by `(room_no, CAST(room_date AS DATE))` in Rust (a `HashSet`, same
  shape as `mirror_probe::diff_mirror_rows` but keyed on the business pair
  instead of an integer pk) and re-drive only the legacy rows whose key is
  **not** in the canonical set. Reuse the era floor the resolve arm already
  derives (`fetch_calendar_business_key_pg`'s `MIN(rcal_date)`) as the
  default lower bound — same "never configured, always derived" contract as
  `PAYMENTS_ERA_FLOOR_SQL` and the mirror probe's `MIN(pk)` — with a
  `--since=YYYY-MM-DD` override for a narrower manual re-run, and `--all` to
  scan unfloored (useful for a first exploratory dry run, expensive
  otherwise: HF Hotel's unfloored `HT_Room_Status` is unbounded history, not
  measured here).

  When `HT_Room_Status` has duplicate rows for one `(room_no, night)` business
  key (observed live for HF Ville room 201, 4 legacy ids), pick a single
  deterministic representative (e.g. `MAX(id)`, matching how the live mapper's
  UPSERT would simply apply whichever row CT delivered last) rather than
  driving the mapper once per duplicate — the target row converges to the
  same canonical state either way, so driving it multiple times only adds
  redundant writes.

* **Idempotency predicate**: the mapper's own `ON CONFLICT (rcal_room_id,
  rcal_date) DO UPDATE` already makes a re-apply of the same key a no-op in
  effect (same values in, same values out), so unlike
  `backfill_receipt_payments` (which pre-checks and explicitly skips already-
  present rows to avoid a second write) this bin can safely upsert every key
  in its work-list without a separate existence check — the mapper path *is*
  the idempotency guard. A `--dry-run` still short-circuits before the PG
  transaction, so a preview never even reaches that upsert.

* **`--dry-run` output**: report, per key, what would be written — room_no,
  night, `room_status`, resolved `room_id`, and whether `room_Book_No` /
  `room_CheckIn_No` resolved to a canonical booking/checkin (both optional;
  a miss is not a skip, matching the live mapper's "tile matters more than FK
  precision" stance) — plus a summary table: candidates found, would-write,
  errored (e.g. genuinely unknown room — should not happen per §2's evidence
  but the mapper's own loud-fail-and-retry contract on a missing room means
  this bin must also surface it loudly, not silently skip). Mirror the
  `backfill_receipt_payments` summary block shape (`Attempted` /
  `Would write:` or `Written:` / `Skipped` / `Errored`) for operator
  familiarity.

* **Verification — same self-verifying pattern as #278, adapted for an
  aggregate-only probe.** `backfill_receipt_payments` deliberately does NOT
  mark `ht_reconcile_log` rows resolved — it leaves that to the next reconcile
  tick re-hashing both sides, "the designed self-verifying path (it proves
  the write actually took)". This bin should do the same in spirit, but the
  mechanics differ because the calendar probe writes one aggregate row per
  site, not one row per key: after a live run, the **next mirror-probe tick**
  (`probe_room_calendar_business_key`) re-derives the business-key hash fresh
  on both sides and will find `legacy_hash == pg_hash` if the backfill closed
  the era-floored gap completely, auto-resolving any open aggregate row via
  the same `should_auto_resolve` path every other probe uses. Because this is
  an aggregate NET comparison (equal-and-opposite gaps can cancel — the known
  limitation the closure-arm doc already records), the bin's own printed
  summary (candidates found vs. written) is the primary evidence of
  correctness for a human reviewing the run; the reconcile row converging is
  the automated confirmation that canonical's business-key state now matches
  legacy's, not proof that every individual key round-tripped byte-for-byte.
  For a stronger per-key guarantee, re-run this doc's manual key-diff (§2) with
  `comm -23` after the live run and confirm `missing_pg` collapses to (ideally)
  the empty set within the floor scanned.

### Deployment gotcha — Dockerfile needs three edits or the bin ships absent

Confirmed by reading `hotel-backend/Dockerfile` directly — every existing
`backfill_*` bin appears in exactly three places, and a new bin must be added
to all three or CI goes green while the binary is silently missing from the
runtime image:

1. **Build stage, `cargo build --release --locked --offline` bin list**
   (`Dockerfile:169-180`) — add `--bin backfill_room_calendar`.
2. **Build stage, `/out/` staging copy** (`Dockerfile:183-193`) — add
   `cp target/release/backfill_room_calendar /out/;`.
3. **Runtime stage, final image `COPY --from=builder`** (`Dockerfile:232-242`)
   — add `COPY --from=builder /out/backfill_room_calendar /app/backfill_room_calendar`.

This is the exact gotcha the #278 closing comment flagged for the next reader
("a new bin must be added to `hotel-backend/Dockerfile` in three places… or
the deploy goes green with the binary absent from the image") and it is not
hypothetical — the sibling agent working `scheduler/sync.rs` /
`mirror_probe.rs` this round owns those files, not `Dockerfile`; whichever
follow-up actually implements this bin needs to touch `Dockerfile` explicitly,
since none of the three edit sites are colocated with the mapper or scheduler
code a reviewer would naturally be looking at.

No dedicated `docker-compose.yml` service is strictly required — several
existing backfill bins (`backfill_receipt_payments`, `backfill_legacy_checkins`,
`backfill_legacy_bookings`) have no `profiles: [backfill]` entry of their own
and are invoked ad hoc (reusing the `backend` service's built image/env,
matching how #278's live run was executed). Adding one (`backfill-room-calendar`
/ `backfill-room-calendar-hfville`, mirroring the `backfill-payment-ledger`
pair at `docker-compose.yml:1006-1052`) would be a nice-to-have for
repeatability but is not on the critical path.

---

## 5. Sequencing against the T2 / mirror-probe rollout

**Land the remediation before flipping `RECONCILE_MIRROR_PROBE_ENABLED`, for
two independent reasons:**

1. **Alert-noise reality.** The moment the flag flips true, both sites will
   open exactly one durable `mirror_ht_room_calendar` aggregate row apiece
   (`observe_only: false` since `60deef0`) — and per the module doc, this row
   is deliberately excluded from every self-heal list
   (`FORCE_CONVERGE_VALUE_DRIFT_TABLES`, `REINGEST_MISSING_PG_TABLES`), so
   `check_level_drift_and_alert` / `send_stale_level_digest` will escalate it
   to the `:bangbang:` 72h tier on schedule, with genuinely nothing an
   operator can point at except "yes, we know, it's on a list somewhere" — a
   fresh instance of exactly the alert-training-dismissal pattern the Phase 5
   alert-surface audit (2026-07-28, same incident log) already spent a whole
   entry fixing. Landing the backfill first means the flag flips onto a table
   that is at worst 5 nights short (the residual, itself probably swept up by
   the same run since the bin doesn't distinguish "old" from "recent" missing
   keys) instead of ~3,650 nights short across both sites.
2. **Impact finding from §1 removes the only argument for urgency.** Nothing
   reads this table yet, so there is no live-correctness reason to rush the
   flag ahead of the backfill — the only cost of sequencing
   backfill-then-flag is a few more days before the mirror probe arm goes
   live, and that cost is much smaller than the cost of a chronically-open,
   unactionable page once it does.

Recommended order:

1. Implement `bin/backfill_room_calendar.rs` per §4 (own PR, own review — this
   doc does not implement it).
2. Run `--dry-run` on both sites, confirm the candidate count roughly matches
   this doc's `missing_pg` figures (3,137 / 513, modulo whatever days pass
   between now and the run and any organic convergence).
3. Run live on both sites. Re-run `--dry-run` immediately after to confirm
   idempotent convergence (candidates should now be ~0, or just the
   irreducible surplus-side / genuinely-unresolvable rows, matching the
   `backfill_receipt_payments` precedent of re-running dry-run to prove
   idempotency).
4. Re-run this doc's manual key-diff (§2) once more to confirm `missing_pg`
   has collapsed at the current era floor.
5. Only then flip `RECONCILE_MIRROR_PROBE_ENABLED` (a separate, coordinated
   step already gated behind T2's own soak sequencing — mirror-probe is
   explicitly called out in the task brief as "the LAST arm to enable").

---

## Appendix — queries used (read-only, reproducible)

```sql
-- Canonical floor/count (per site)
SELECT COUNT(*), MIN(rcal_date), MAX(rcal_date) FROM ht_room_calendar;

-- Legacy floored aggregate (matches room_calendar_business_key_legacy_sql)
SET NOCOUNT ON;
SELECT COUNT_BIG(*) AS night_count,
       CONVERT(varchar(10), MIN(night_date), 23) AS min_date,
       CONVERT(varchar(10), MAX(night_date), 23) AS max_date
  FROM (SELECT DISTINCT room_no, CAST(room_date AS DATE) AS night_date
          FROM HT_Room_Status
         WHERE room_no IS NOT NULL AND room_date IS NOT NULL
           AND CAST(room_date AS DATE) >= CAST('<canonical MIN(rcal_date)>' AS DATE)
       ) AS nights;

-- Full key lists, diffed locally with `comm -23` / `comm -13` after sort -u:
--   canonical: SELECT room_no || '|' || rcal_date FROM ht_room_calendar
--              JOIN ht_rooms_new ON room_id = rcal_room_id;
--   legacy:    SELECT DISTINCT LTRIM(RTRIM(room_no)) || '|' ||
--              CONVERT(varchar(10), room_date, 23) FROM HT_Room_Status
--              WHERE ... CAST(room_date AS DATE) >= <floor>;
```

Access pattern used throughout (per task instructions):

```
ssh evergreen 'docker exec new-hotel-db psql -U postgres -d hotelnew -tAc "..."'
ssh evergreen 'docker exec new-hotel-db psql -U postgres -d hotelville -tAc "..."'
ssh evergreen 'docker run --rm --network host -e SQLCMDPASSWORD="$(cat /home/deploy/secrets/db_password)" \
  mcr.microsoft.com/mssql-tools /opt/mssql-tools/bin/sqlcmd -S 192.168.100.222,1433 -U sa -d db -C -W -s"|" -Q "..."'
# Ville: -S 192.168.11.51,1436 -d HOTEL, same db_password secret
```
