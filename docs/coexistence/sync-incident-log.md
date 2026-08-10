# Sync incident log

Running root-cause ledger for coexistence sync / monitoring alerts, newest first.
Format per entry: symptom → evidence → cause → fix. Keep entries short; link code
and prior entries instead of re-explaining. Vocabulary: "sync lag / unconverged"
for transients, "durable divergence" only for rows that resist multiple sweep cycles
(see CLAUDE.md "Vocabulary note").

## 2026-08-01 — payment-ledger probe: 404 false `missing_pg` folios from a backfill dragging its coverage floor

**Symptom.** The Phase 6-D payment-ledger probe (`payment_ledger_probe`, enabled
on HF Ville 2026-08-01 as T2 rollout step 5) reported **404 `missing_pg` folios**
at HF Ville, all of them pre-coverage. Zero were genuine mirror gaps — the money
in every one of them was never in scope for the mirror in the first place.

**Evidence.** The probe floors its legacy scan at
`PG_FLOOR_SQL = SELECT MIN(ledger_legacy_id) FROM ht_payment_ledger` and applies
it as `HAVING MIN(id) >= floor` over legacy `HT_CheckIn_Pay`. That floor had
moved from ~40470 to **39113** — about seven months backwards — between the
2026-07-28 read-only baseline (1,016 in-era folios, exactly converged) and the
first enabled tick. The single row responsible was a payment line dated 2025-08
belonging to `CH25-000076`, a monthly-billed long-stay, mirrored by the
`backfill_payment_ledger --days=212` run of 2026-07-30.

**Cause.** A raw `MIN(ledger_legacy_id)` is a valid coverage boundary **only if
coverage is an id-contiguous SUFFIX** of the legacy table. Mirroring does not
preserve that property: `sync/mappers/payment.rs::mirror_payment_ledger` DELETEs
a `Cin_No`'s lines and re-INSERTs the loader's whole current set, so the folio —
not the line — is the atomic unit, and anything that selects folios by DATE
lands whole folios with their old line ids attached. A date-windowed backfill and
an id-derived floor are therefore **not interchangeable definitions of coverage**.
One long-stay whose folio spans a monthly billing boundary is enough to collapse
the floor by the length of that stay, and every never-mirrored folio between the
old and new floor is then swept into the scan and reported as `missing_pg`. The
same shape reaches the probe with no backfill at all: iHOTEL editing a payment on
a pre-era folio makes the CT tick mirror that folio whole. The module docs named
this risk and consciously did not fix it — the reasoning was that
`PAYMENT_LEDGER_MAX_FOLIO_FINDINGS` (50) bounds the blast radius to one aggregate
row, and that a clamped floor needed a schema change because
`ht_reconcile_era_floor.era_floor` is a `TIMESTAMP` and cannot hold an id. The
cap did contain it. It did not prevent it, and 404 folios is well past the point
where an operator has to reason about whether the money is real.

**Fix — two parts.**

1. *Remediation — DONE at Ville, PENDING at HF Hotel.* Ville's
   `backfill_payment_ledger --all` ran the same day (2026-08-01), making its
   canonical coverage genuinely complete rather than merely date-windowed:
   Ville's derived floor is now 39014, the absolute legacy minimum, so the in-era
   set is the whole legacy table and the boundary can no longer move at all.
   **HF Hotel's `--all` has NOT run** — it is scheduled for the night of
   2026-08-01. Until it completes, HF Hotel's canonical coverage is still the
   Track J7e window, its derived floor is that window's boundary, and the 19
   `missing_pg` folios from the 2026-07-28 baseline
   (`CH26-004952`…`CH26-004971` minus `CH26-004960`) are still uncovered. The
   probe therefore stays DARK at HF Hotel until the run completes — see the
   enable-order constraint below, which is not a preference but a correctness
   requirement of the new ratchet.

2. *Class fix — migration 084 + the era-floor ratchet.*
   `ht_reconcile_era_floor` gains `era_floor_id BIGINT` (with `era_floor` going
   nullable and a `CHECK` that a row carries a floor in at least one basis), and
   `scheduler/payment_ledger_probe.rs::payment_ledger_era_floor` now computes
   `effective = GREATEST(persisted era_floor_id, MIN(ledger_legacy_id))`,
   persisting the max back through the same `GREATEST` upsert the Phase 6-B
   `guest_registry` arm uses. The floor can only ratchet FORWARD; a later
   poisoning row moves the derived value and leaves the effective one alone.
   `clamped_era_floor` was generalized over the floor basis so both arms share
   ONE rule rather than two copies, and the arms key their own rows
   (`payment_ledger_probe` vs `guest_registry`) so they cannot collide — per-site
   separation is free, each site's probe running against its own logical PG
   database.

**Enable-order constraint the ratchet creates — read before flipping the flag at
any site.** The ratchet makes the floor monotonic, which means the FIRST enabled
tick's reading is durable. Seeding it from narrow coverage is therefore its own
failure mode, and it is the mirror image of the incident above:

* **Do not enable the probe at a site until that site's
  `backfill_payment_ledger --all` has completed.** A floor seeded from a
  date-windowed coverage window would keep holding after a later `--all` widens
  coverage, permanently excluding the folios that backfill lands — they would
  never be compared, and a real gap in them would be invisible.
* **If a watermark was seeded before a coverage-widening backfill**, the only
  way down is to delete the row:
  `DELETE FROM ht_reconcile_era_floor WHERE table_name = 'payment_ledger_probe';`
  on that site's canonical database (`hotelnew` / `hotelville`). The next tick
  re-derives from live data. A hand `UPDATE` can only move a floor FORWARD — the
  upsert clamps with `GREATEST`.
* **The tripwire that says this happened.** `effective > derived` sustained
  across 4 consecutive ticks (~1h at the 15-min cadence) raises a Slack alert
  under the cooldown key `era_floor_held:payment_ledger_probe`, naming both
  floors, the site, and the DELETE. It is deliberately not pager-tier: the scan
  never widens, so nothing is broken — but the two explanations (a genuine
  pre-coverage mirror, or a stale watermark) need different responses. A hold
  lasting a single tick is the ratchet working and is logged, not alerted.

**Also fixed here — the resolve path no longer writes the watermark.** The
auto-resolve sweep dispatches on `ht_reconcile_log.table_name` and is NOT gated
on `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED`, so the first version of the ratchet
let a sweep at a DARK site persist a floor from that site's pre-backfill
coverage — exactly the stale-watermark state above, arrived at without anyone
enabling anything. Both `<aggregate>` resolve arms now go through
`payment_ledger_era_floor_readonly` (same derived `MIN`, same clamp, plain
`SELECT` of the persisted row). The probe's own tick is the single writer.

**And both scans are floored, not just the legacy one.** The first version
floored only `HT_CheckIn_Pay`. Whenever the watermark holds, every canonical
folio below it then has no legacy counterpart in scope and reads as
`missing_mssql`: per-tick row churn while the count is small (the probe re-mints
what the sweep just closed through the unfloored single-folio projection) and, once
past `PAYMENT_LEDGER_MAX_FOLIO_FINDINGS`, ONE `<aggregate>` row that can never
resolve, because resolution would compare unfloored-canonical totals against
floored-legacy ones — paging every 72h forever. Both scans now carry the same
whole-folio `HAVING MIN(id) >= floor`, and both are pinned to the EFFECTIVE
value by test.

**Deliberately NOT done: switching the floor to a date basis.** The obvious
"floor on `ledger_pay_date` instead" is worse, not simpler: that column is
`TIMESTAMPTZ` while the legacy side stores naive local Thai time, so it
reintroduces exactly the Thai→UTC boundary reasoning the integer IDENTITY basis
was chosen to avoid.

**Made visible, not just fixed.** `payment_ledger_era_floor` logs whenever the
persisted watermark is holding the scan forward, the tick's summary lines carry
`derived_floor` alongside `floor`, an `<aggregate>` divergence row records both in
its JSON, and a hold that PERSISTS raises the Slack alert described above. A
future drag attempt now shows up as a log line naming both numbers instead of
silently widening the scan — and a hold that is really a stale watermark now
reaches an operator instead of sitting in a log. Detection and the auto-resolve
sweep both go through the clamped helper (test-pinned), or a row written from one
folio set could be re-projected from another and never close.

## 2026-07-29 — mark_dirty polarity fix: live-verified PASS (room 302, HF Hotel)

**Not an incident** — a live verification entry for a fix shipped and confirmed
the same day. Merge commit `6848a22` (fix commit `5ce6be1`, branch
`phase3-mark-dirty`), deployed via CI: deploy job ran, all five containers
restarted, backend healthy. Runsheet: `docs/coexistence/phase3-mark-dirty-runsheet.md`.

**The bug (live until today).** Marking a room dirty in our app reused
`mark_clean`'s statement builder, so it wrote the CLEAN value to legacy
`HT_Rooms.Room_Clean`. Two consequences: iHOTEL was told the room was clean —
the opposite of the receptionist's intent — and the CT watcher then read that
back and reverted canonical to clean within a tick, silently undoing the
receptionist's action.

**Fix.** `mark_dirty` now owns its own `build_statements` and writes
`Room_Clean='yes'` (needs cleaning), leaving `Room_Use` / `Room_Use_Count` /
`Room_Clean_Time` untouched.

**Live verification, room 302 (legacy `HT_Rooms.id=2`), HF Hotel, all
timestamps UTC.**
- Baseline pre-test: canonical `room_clean=true`, available; legacy
  `Room_Clean='no'`, `Room_Use='no'`, `Room_Use_Count=355`, `Room_Clean_Time`
  blank.
- 10:22:31 reception marked 302 dirty in our app. Writeback job #40
  `mark_room_dirty` → done, 1 attempt, no error, ~65ms.
- Canonical flipped to `room_clean=false` at 10:22:33.
- Legacy row: `Room_Clean` `'no'` → `'yes'`. Companion columns PROVABLY
  untouched: `Room_Use='no'`, `Room_Use_Count=355`, `Room_Clean_Time` still
  zero-length.
- Exactly one `HT_Housewife` audit row minted: `h_name=winut`, `h_room=302`,
  `h_cin=CH26-006285`, `h_note=''` (17:22:31 Thai local).
- Reception confirmed iHOTEL's own room grid displayed 302 as needing
  cleaning — the cross-app assertion that only a live window can make.
- Echo test (the point of the window): 4 minutes untouched. Canonical stayed
  `room_clean=false` with `updated_at` frozen at 10:22:33 (nothing rewrote
  it). Crucially the watcher was demonstrably awake and DID see the change —
  sync-1 logged `CT rows applied table="HT_Rooms" from=69696
  applied_through=69697 ingested=0 skipped=1` and advanced the global
  watermark 69696→69697. `ingested=0/skipped=1` is the echo-before-stamp
  adoption correctly declining to re-apply our own write. Under the old bug
  this would have ingested the row and inverted canonical back to clean.
- 10:27:32 reception marked 302 clean again. Writeback job #41
  `mark_room_clean` → done, 1 attempt, no error. Both sides converged back to
  the exact baseline (canonical `room_clean=true`; legacy `Room_Clean='no'`,
  `Room_Use='no'`, `Room_Use_Count=355`, `Room_Clean_Time` zero-length).
  `ht_reconcile_log`: 0 unresolved rows throughout.

**Not run.** Runsheet step 12, the inbound-direction spot check (reception
marking a DIFFERENT room in iHOTEL and our app following) — the window ended
after the outbound path passed. The inbound path is long-standing shipped
behaviour and was not modified by this change, but it was not re-exercised
today.

**Known follow-up carried forward (not a gate).** Our mark-dirty
`HT_Housewife` row has `h_note=''`, which `FrmReportHousewife` counts as a
cleaning by that operator. New evidence from today's read: iHOTEL's OWN rows
for room 302 (`h_name=Admin`, dated 2026-07-26, 2026-07-05, 2026-06-11) also
carry `h_note=''`, so an empty note does not discriminate even within
iHOTEL's own data; any fix needs a discriminator that appears in no existing
capture. Being filed as its own issue.

> **Resolved 2026-07-31 (#276, `ccf88c3`).** Not fixed by a discriminator
> note — re-scoped instead. A live scan of all ~31,922 `HT_Housewife` rows
> found no dirty-flip note pattern at all (only `ปิดโดยโปรแกรม` system
> auto-close and `เปลี่ยนสถานะเป็นซ่อม :` send-to-maintenance are non-empty),
> and both housewife-writing decompile handlers (`ClickClean`,
> `ClickCleanOK`) are clean-side; findings.md §3e/§3i show check-out /
> cancel-check-in raise `Room_Clean='yes'` with zero `HT_Housewife` touches.
> iHOTEL itself never writes `HT_Housewife` on a dirty flip, so this audit
> row had no legacy analog — `mark_dirty` now writes no `HT_Housewife` row at
> all. `mark_clean` is unchanged. The room-302 row described above (minted
> under the pre-#276 build) is historical and stands as recorded; a repeat of
> this runsheet today would see no such row — see
> `docs/coexistence/phase3-mark-dirty-runsheet.md` Step 6.

## 2026-07-29 — /v2 slow tabs + 500s + SSE 524: PG pool exhausted by per-tab LISTEN connections

**Symptom.** Long loads switching /v2 tabs; console 500s on
`/api/stats|checkins|bookings` and a Cloudflare 524 on `/api/events?branch=all`.

**Evidence.** Origin answered unauthenticated probes in <5ms; backend logged
NOTHING at the failure instant (it had no request-level access logging); web
container: `Failed to proxy … socket hang up` for exactly the three endpoints in
one 60ms burst; PG "idle"; Ville pool held residual `UNLISTEN *` connections —
released per-client PgListeners. Legacy MSSQL was ruled out with measurements
(reads <100ms, zero blocking; the `Timed out in bb8` noise is the separate,
sync-only #274).

**Cause.** Every /v2 tab's SSE stream held 1-2 REAL pool slots for its lifetime
(`PgListener::connect_with(pool)`, two acquires — serial — for `branch=all`)
against pools of max 10/5 with sqlx's 30s default acquire timeout never
overridden, while `AUTH_ENABLED=true` added a session-validate acquire in front
of every request and the in-process reconcile parked ~9 more. Saturation ⇒ data
endpoints hang 30s → 500; SSE spends ~90s in serial acquires emitting zero
bytes → CF 524 at ~100s → EventSource reconnects onto the starved pool. NOT a
regression — no route/pool file had changed in 3+ weeks; a concurrency
threshold was crossed.

**Fix (`517b907`).** One standalone listener connection per database
(zero pool slots) fanning out via per-site broadcast channels; handler is
pool-free after auth; immediate hello frame (CF gets bytes in ms); 5s
`acquire_timeout` on both PG pools; pool headroom 10→20 / 5→10 (compose
defaults, ADR 0004); request-level access log (path+status+latency ONLY — the
existing TraceLayer stays at DEBUG because it records query strings carrying
guest-identifying params). Verified live: exactly 2 `LISTEN "domain_events"`
connections regardless of tab count.

**Planning premise falsified during implementation, repaired not shipped:** the
planned synthetic `refresh` event would have been silently dropped by ALL 11
subscribing pages — both consumers filter strictly by event name with no
`onmessage`. Resync is instead a burst under every subscribed name, debounced
client-side to one refetch, with a compile-time exhaustiveness guard on the
name list.

**Rules of thumb.** (1) A pool can be exhausted while `pg_stat_activity` looks
idle — LISTEN connections ARE idle; count them, don't eyeball state. (2) An SSE
handler must emit first bytes before any slow work or the CDN's timeout becomes
your failure mode. (3) "socket hang up" with a silent backend means you lack
access logging, and that absence is itself the first finding.

## 2026-07-28 — Sync/alert hardening wave (Phases 2–5): armed product-stall, checkout double-count, gate_guard contract, bookings-hash disproof, alert-surface calibration

**Context.** Not a single live incident — five investigations run the day after the
16-day loss below, each hunting the same shape of defect that bug exposed: a control
that looks like it works and doesn't. Commits `5e1d014`..`1b06849`; `42dc2c0` (the
entry directly below) is the root-cause fix this whole wave follows from and is
included here only as the anchor.

### Armed product-stall (Phase 2b, `7b57edc`)

**Symptom.** None yet — found by code audit, not a live failure. `poll_products_once`
was referenced in its own module doc and never defined; `upsert_product` had zero
callers.

**Evidence.** `ht_products` was populated only by an operator hand-typing a legacy id.
`upsert_canonical_pos_sale` INNER JOINs `ht_products` and returns `Err` on a miss —
and since `42dc2c0` that `Err` holds the GLOBAL watermark for all 19 CT tables, not
just one.

**Cause.** The inbound product mirror was designed (documented in
`CARDINALITY_MAP.md`) but never wired into `reload_mirror_dimensions`. Products
tables were never actually empty (hotelnew 5 rows, hotelville 3), so the gap never
fired live — but the first sale of any NEW legacy product would have frozen the
entire site's sync, not just products. An armed stall with a wide blast radius,
dormant only because no new product had been added since the FK-miss path shipped.

**Fix.** Wired `poll_products_once`, first in `reload_mirror_dimensions`'s cycle.
This narrows rather than closes the race: the reload rides the 15-min reconcile
tick, not the sub-second CT path, so a product created and sold inside one interval
still misses on the first attempt — but the miss now holds the watermark (bounded,
self-heals next reload) instead of silently dropping the event. Also found while
wiring: legacy `Pro_Name` is `varchar(500)` BYTES (TIS-620) vs canonical
`VARCHAR(250)` CHARACTERS — an over-long Thai SKU name would have aborted the reload
TX every tick, forever (same stall class); now char-clamped. Deliberately no prune
(FK aborts on any ever-sold SKU, iHOTEL's delete-then-reinsert edit pattern, and
canonical-only rows all make a prune unsafe). Residual: an eager per-miss load at
the FK site would close the race fully (filed as an issue, tracked in the
continuation plan).

### Checkout double-count (Phase 2a, `7b57edc`, migration 079)

**Symptom.** None live — caught before shipping the product sync that would have
unmasked it. Verified unreachable today only because `ht_pos_sales`/`ht_products`
are 0 on both sites; shipping the product sync WITHOUT this fix first would have
turned on live double-charging on checkout.

**Evidence.** The originally planned fix assumed `cin_total_amount`
(`HT_CheckIn_H.Total_Price_Net`) was room-only before checkout and Room+Product only
after. False: iHOTEL rewrites `Total_Price_*` on every payment/sale change, so
checkout is a re-write, not the first write — there is no "already folded vs not"
state to discriminate on.

**Cause.** No discriminator existed for "has this folio's product total already been
folded into the room total". Compounding it, checkout writeback passed
`room_price_total` (computed from the product-inclusive `cin_total_amount`) to
`CheckOutCommand` even with `CHECKOUT_SERVER_TOTAL_ENABLED=true` (the prod default),
which would have stamped a product-inclusive value into the shared legacy
`Total_Price_Room` column — corrupting the exact column the next CT tick reads back.

**Fix.** Migration 079 adds `ht_checkins.cin_room_amount`, fed from
`Total_Price_Room` (already present in the check-in header projection — no legacy
DDL, no CT change). `folio_breakdown`'s basis becomes
`cin_room_amount + product_total`, state-free: iHOTEL-folded lines are excluded from
the room basis by construction, app-originated lines are added once. Deliberately
**no `DEFAULT 0`** on the new column — legacy `Total_Price_Room` is
`float NOT NULL DEFAULT 0`, so a product-only folio can legitimately have a zero
room leg; a column default would have collapsed "never projected" into "genuinely
zero" and double-billed that folio's products. `NULL` falls back to the pre-079
`cin_total_amount` basis (exact everywhere no POS line exists — i.e. everywhere
today) and self-heals on the first CT read-back. All six checkout money fields now
derive from one `folio_breakdown` on the flag-on path, so `room + product == net`
holds by construction.

### gate_guard executable contract + golden byte pins (Phase 4a, `c647619`)

**Symptom.** None new — closes the CLASS of bug behind three separate incidents:
2026-07-06 checkins (`d09e756`, entry below), plus a bookings and a customers repeat
of the same shape found this week. Each was fixed with a hand-written one-off test;
nothing stopped the next occurrence.

**Evidence.** The pattern every time: a field feeds the reconcile hash but is
invisible to the mapper's idempotency gate (`existing_matches`), so a legacy edit
touching ONLY that field is silently skipped, the watermark advances anyway, and the
CT delta ages past the 2-day retention window. A naive guard (two string consts plus
a superset `assert!`) does not catch this — nothing binds the declared field list to
the boolean chain the gate actually executes at runtime, which is exactly how the
prior one-off tests decayed.

**Cause.** No mechanical link between "fields the hash covers" and "fields the gate
compares" — both were prose-documented lists that could silently fall out of step
with each other as either was edited independently.

**Fix.** Both halves made EXECUTABLE, proven by breaking the code in both
directions. The gate: `existing_matches` IS the table now —
`FIELDS.iter().all(|f| (f.matches)(..))` — so deleting a field's name deletes its
comparison; there is no second list to fall out of sync. The hash: each
`HashInput` carries the segment-rendering closure, and a golden test pins
`sha256(join(segments))` to the literal pre-change format string. Deleting the
`cin_checkin_time` gate term failed the superset test, the mutation test, AND the
pre-existing `d09e756` regression test; "fixing" that by instead deleting
`cust_idcard` from `HASH_INPUTS` then failed the customers golden-vector test. The
teeth are the per-entity behavioural mutation tests — mutate a hash input on a
fixture, assert the segment moved AND the production gate now returns `false`; this
can't be satisfied by editing a list because it executes the real gate against a
real mutation. Also adds `RECONCILE_RESOLVABLE_TABLES` + `debug_assert!` on the two
dispatch wildcards so a future reconcile arm can't ship without its
`compute_current_*` branches (the exact 2026-05-18 rooms mistake). Byte
compatibility independently verified: all four `format!` templates replicated in
Python, 5/5 match — no stored `ht_reconcile_log.mssql_hash` or
`ht_*_legacy.sync_hash` ack was invalidated, so no re-diff storm.

Two residual gaps reported rather than papered over (filed as issues, not fixed
here): a multi-room-checkin gate blind spot (`legacy_room_no` hashes the FIRST
`HT_CheckIn_Ds` row; the gate compares the room SET; a delete-then-reinsert reorder
moves neither) pinned by
`multi_room_first_room_reorder_is_a_known_gate_blind_spot` (closed 2026-07-28 by #264 — the pin test is now `multi_room_first_room_reorder_defeats_the_gate`); and a guarded-term
Some→None weakness (`legacy_cust_no` is COALESCE'd, so a legacy NULL is
deliberately not treated as a mismatch) recorded per entity.

### Bookings-hash dual-source DISPROOF (Phase 4b, `35b9986`)

**Symptom.** None — a suspected gap raised BY the gate_guard work above, resolved
the same day before it could become an incident.

**Evidence.** `c647619` flagged a possible gate ⊂ hash violation on bookings: the
gate compares header-derived dates from `HT_Book_H` while the reconcile hash reads
dates off the representative `View_Booking_Ds` line — if those were two different
sources, an iHOTEL SAVE_EDIT could move the hash without moving any gated field,
invisible to a name-level check since both sides name the same field
(`book_checkin`/`book_checkout`). Could not be settled from the repo —
`docs/legacy-spike/schema/01-baseline-schema.txt:682` truncates the view's `SELECT`.
Queried `sys.sql_modules` on the LIVE HF Hotel server instead:
`View_Booking_Ds` takes both `Book_Date_in`/`Book_Date_out` from the JOINED HEADER
(`HT_Book_H`), not from `HT_Book_Ds` itself — `HT_Book_Ds` has no such columns at
all; its per-line dates are `Book_Room_Start`/`Book_Room_End`, which this
projection doesn't touch.

**Cause.** N/A — the suspected defect does not exist. Gate and hash read the same
source.

**Fix.** No code change. Phase 4b (hash-source unification) dropped as unnecessary.
Evidence recorded as a comment on `BOOKINGS_RECONCILE_PROJECTION` in
`scheduler/sync.rs` so it isn't re-raised from the same truncated schema dump.

### Watermark sampled-ceiling root-cause (`42dc2c0`)

Root cause of the 16-day dropped-booking incident (customer `C2413` + booking
`R002066`) — full write-up is the entry directly below this one. Cited here only as
the anchor this whole wave follows from: every fix above was found by auditing the
systems adjacent to that bug (the reconcile-hash/gate coupling, and the alert
channel that let it re-fire silently for 16 days) for the same "looks like coverage,
isn't" shape.

### Alert-surface calibration (Phase 5, `1b06849`)

**Symptom.** Not a failure — an audit finding. All 30 Slack alert templates were
inventoried against 7 days of production output: the channel sent ~8 messages in 7
days, 7 of them the SAME `>4h` unconverged-rows digest (day 1 and day 16 of the
16-day loss below sent byte-identical text). Meanwhile ~900 WARN/ERROR log events
per day had no paging path at all — `[Sync] CT watcher lag detected` alone fires
~170×/day at WARN with no Slack. The channel was quiet because the detectors were
mute, not because the system was clean.

**Evidence / cause.** Per-detector shipped behaviour and knobs are in
`docs/runbook-sync.md` §2a/§3. Highlights: the unconverged-rows digest had one voice
regardless of age (day-1 == day-16 text, which trains dismissal fastest); cooldowns
burned on ATTEMPT rather than confirmed delivery, so a failed POST on the all-clear
path could permanently lose the closure record; the CT-lag observation had no
paging path at all; refuse-to-start guards had no dedup (up to 6 identical pages ×
2 sites per failed deploy, then silence once the restart cap hit — a storm followed
by a false "recovered"); the writeback listener's `consecutive_failures` never reset
on a healthy session, so unrelated failures spread across days could sum into a
page; the shadow-mode ceiling guard measured process uptime and reset on every
deploy, so at 6 deploys/day it could structurally never reach its 36h trigger; and
the four business-notification flags defaulted `true` in code, compose, AND the
workflow (only the repo-variable layer held them `false`), so a fork or a deleted
repo variable would have turned the alert channel into a booking feed.

**Fix.** Escalation tier added to the unconverged-rows digest (`:bangbang:` at
`LEVEL_DRIFT_ESCALATE_HOURS`, default 72h, own `escalated:<table>` cooldown key,
exactly one tier per table per tick); all four threshold families made
env-overridable with unchanged defaults. Cooldowns now burn only on confirmed
delivery. CT-lag pager added, VERSION arm only (the poll-age arm is structurally
false on a quiet night — paging on it would have manufactured nightly noise), 30-min
persistence gate, durable 24h slot released by its own all-clear, suppressed while
a watermark-STUCK alert is open. Refuse-to-start pages deduped per reason
(`boot_refusal:<slug>`), fail-OPEN (the opposite polarity from the retention page on
purpose — these guards' failure mode is silence about a dead process, not a storm),
every refusal body now states the restart-cap contract explicitly so silence is
never read as recovery. Writeback listener alert demoted to a 10-min-sustained page
with the healthy-session reset fixed and a paired all-clear. Shadow-mode ceiling
re-anchored to `legacy_ct_state.last_polled_at` (frozen for the soak's duration,
durable across restarts, zero extra queries). Notification flags now fail closed at
every layer (code default, compose default, workflow fallback) — verified as a
production no-op by tracing the actual precedence chain, not by assuming it. One
cross-file defect found independently by two agents in the same PR: `tables_recovered`
filtered only the bare `stale_active_checkin` sentinel, but `bin/sync.rs` parks
namespaced keys (`ct_retention_overflow:`, `boot_refusal:`, etc.) in the SAME shared
`ht_level_drift_alert_cooldowns` table — the reconcile all-clear could treat a
namespaced key as a converged table and delete it, silently un-throttling the other
binary's pages. Fixed structurally (`is_reconcile_table_key`: any key containing the
namespace separator is non-reconcile) rather than by extending a list that would go
stale on the next key added.

**Outcome.** Gates: 1241 lib + 158 bin/sync + 57 bin/writeback pass; `cargo check
--release` clean with and without `SQLX_OFFLINE`; clippy zero-new on every touched
file (baselines measured by stash/HEAD-copy comparison, not eyeballed).

**Rule of thumb for next time.** A quiet Slack channel is not evidence of a healthy
system — check whether the detectors underneath it can actually reach Slack before
trusting the silence. And when auditing one incident's blast radius, check the
adjacent control surfaces (here: the gate/hash coupling and the alert channel
itself) for the same failure shape — three of the six findings in this wave were
caught that way, not by a new failure.

### Coverage expansion — payments, guest-registry, mirror probe, payment-ledger probe (Phase 6-A..D, `3c45ba6`..`7a928fd`)

**Not an incident** — four new reconcile arms/probes landed the same day, each shipped
dark behind its own flag (`RECONCILE_PAYMENTS_ARM_ENABLED`,
`RECONCILE_GUEST_REGISTRY_ARM_ENABLED`, `RECONCILE_MIRROR_PROBE_ENABLED`,
`RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED`, all default `false` both sites). They close
the last structural coverage gap the Phase 5 alert-surface audit above exposed: the
existing reconcile only ever hashed `customers`/`rooms`/`bookings`/`checkins`, so a
dropped CT event on a receipt, a TM.30 companion, one of the 8 opaque
`legacy_mirror.*` pass-through tables, or a line in the per-folio tender ledger the
round report reads had no watcher at all — unconverged and unnoticed by construction,
not because nothing was ever wrong. Each arm floors its scan at its own mirror's
derived canonical-era start so it never reports the tens of thousands of pre-CT
legacy rows as missing (unfloored, the payments arm alone would have opened >20,400
permanently unclosable rows on its first tick); even floored, live read-only evidence
the same day found real gaps worth acting on — 19 missing payment-ledger folios at
HF Hotel (money the round report under-counts today) and a `ht_room_calendar` gap
(1507 legacy vs 1298 canonical rows, HF Hotel) structural enough to ship
observe-only rather than alert on a row nothing can currently close. Rollout
sequencing and coverage boundary in `docs/runbook-sync.md` §2b / §9c.

## 2026-07-27 — HF Ville bookings+customers unconverged 16 days (global CT watermark clobbered mid-loop)

**Symptom.** `level_drift_alert` fired for hfville: `bookings` 3 rows, `customers`
1 row. Alert had been re-firing on its 24h cooldown since 2026-07-11 — ~16
consecutive days, never acted on.

**Evidence.** All 4 rows `divergence_kind = missing_pg`, `pg_hash IS NULL`,
`legacy_row_count=1 / pg_row_count=0`, detected within 1.5s of each other at
2026-07-11 03:51:30Z. One logical iHOTEL save: customer `C2413` +
booking `R002066` (3 rooms 110/112/217, stay 2026-11-27→29), `Book_Date
2026-07-11 10:49:15` Thai. Neighbours on both sides landed fine — `C2411`,
`C2412`, `C2414`, `C2415` and `R002064`, `R002065`, `R002067`, `R002068` are all
in PG — so not an outage window; a per-key miss. Sync was healthy throughout
(149 customers / 131 bookings ingested since). `legacy_sync_status`: 0 errors.
The decisive line: in the whole 03:47–03:59Z window the CT watcher logged
**exactly one** advance — `table="HT_Book_Pro" from=30788 to=30801 ingested=0
skipped=2 per_table=false`. `HT_Customers` and `HT_Book_H` logged nothing. The
auto-resolve sweep has recomputed `current_legacy_hash=Some(...)` /
`current_pg_hash=None` every tick for 16 days, i.e. the legacy rows still exist
and canonical is still empty.

**Cause.** Global-watermark cross-table clobber. `watermark.rs`'s module doc
specifies `advance` is called *once per tick, after all mappers commit, with
`min(per-table-max)`*. The implementation called it **inside `poll_table`, once
per table, with that table's own `max_version`**, and `advance` is monotonic-max
(`WHERE last_seen_version <= $1`) — so the shared watermark ended each tick at
**max**, not min. One iHOTEL save writes many CT tables in a single transaction;
the watcher polls them sequentially in `CT_ENABLED_TABLES` order, where
`HT_Customers` is index 0 (first) and `HT_Book_Pro` is index 18 (last). The save
landed mid-loop: `HT_Customers` had already polled and seen nothing, then
`HT_Book_Pro` polled after the write and advanced the *shared* watermark
30788→30801. Every table resumed the next tick at 30801, so `C2413`'s v≈30789
was never fetched. CT's 2-day retention then made it unrecoverable. Nothing
errored, so the `!errored` watermark gate (2026-06-11 silent-drop fix) never
engaged. **This is the unfixed half of the 2026-06-03 incident**
(`C22209`/`R015290`), whose write-up already named it — "global-watermark mode
also stranded the customer's un-polled CT row" — but only the FK-defer half was
fixed then. Ongoing exposure was real, not theoretical: single advances on
hfville jump up to 832 versions (`HT_CheckIn_H` 369 jumps >3, `HT_Room_Status`
218, `HT_Book_H` 182).

**Why it never self-healed.** `should_auto_resolve` requires *both* hashes
present; `(Some(legacy), None)` — a dropped ingest — falls to `_ => false`.
`force_converge_reconcile_row` covers only customers/rooms and only value drift.
So a `missing_pg` row whose CT event aged out had **no automated path to
closure**, and `level_drift_alert` had no paired recovery notification, so it
just re-fired forever.

**Fix.** Four layers:
1. *Root cause* — the tick now samples `CHANGE_TRACKING_CURRENT_VERSION()` **once,
   before** the mapper loop, and advances the global watermark to that ceiling
   **once, after** the loop, only if no table errored. Anything landing mid-loop
   has a version above the ceiling and survives to the next tick. `poll_table` no
   longer advances the global watermark. Honours the documented contract.
2. *Safety net* — new `missing_pg` re-ingest arm in the auto-resolve sweep: if
   the legacy row still exists and the row is past `FORCE_CONVERGE_MIN_AGE_SECS`,
   re-run the normal mapper for that key (PG-write-only, never writes legacy),
   customers before bookings so the FK-defer ordering holds. Ships dark behind
   `RECONCILE_REINGEST_MISSING_PG_ENABLED`. This is what heals `C2413`/`R002066`
   — no touch-UPDATE on iHOTEL required, unlike the 2026-06-03 heal.
3. *Alert hygiene* — `level_drift_alert` now pairs with a `:white_check_mark:`
   all-clear that also clears the cooldown row, matching the writeback and
   CT-watermark alert precedents. Also fixed the sweep's `LIMIT 500` with no
   `ORDER BY`, which could starve rows past 4h indefinitely under a backlog.
4. *Defence in depth* — per-table watermarks (`SYNC_PER_TABLE_WATERMARK`, R3) made
   flippable: migration 078 reseeds `legacy_ct_state_per_table` (050/056 used
   `ON CONFLICT DO NOTHING`, which is why rows froze at 9060 on hfville vs a
   global of 37843), quiet tables now advance to the tick ceiling instead of
   freezing into retention overflow, `--bootstrap` stamps per-table rows, and the
   flag is plumbed per-site. Ships default-off; HF Ville canaries first.

**Outcome (confirmed 2026-07-27 09:58:48Z).** The re-ingest arm healed all four
rows on its first eligible sweep, 16.25 days after detection
(`age_secs=1404437`). Canonical now carries customer `C2413`
(สุภาวดี เมียนเมือง / 0828261756) and booking `R002066` (2026-11-27 → 11-29,
`confirmed`, cust `C2413`) with all three room legs (110 @ 1150, 112 @ 1150,
217 @ 1250) — byte-matching legacy. The FK ordering guarantee held: the
customer closed at `…48.664`, the bookings at `…48.733/.749/.763`. The new
all-clear fired for both tables and cleared the level-alert cooldown, and the
digest now reports no rows older than 4h. **No write was made to iHOTEL** —
unlike the 2026-06-03 heal, which needed touch-UPDATEs on the legacy rows.

**Two mechanism bugs found while shipping this**, both the same shape as the
incident itself — a control that looks like it works and doesn't:
1. `HFVILLE_WORKER_RECONCILE_ENABLED` was in compose but no workflow, so
   `run-deploy.sh`'s wholesale `.env` rewrite discarded any host-set value. It
   was un-flippable and worked only because its default happened to be `true`.
2. A flag flip via `gh variable set` + an **empty commit** produced a green CI
   run that deployed nothing — an empty commit changes no paths, so the
   `changes` filter is false and the deploy job's final disjunct never holds.
   The flag read `false` in the container for an hour while CI showed success.
Both are closed by ADR 0004 (flag state committed to compose defaults; the
compose edit is itself the deploy trigger). See
`docs/coexistence/RUNBOOK-reconcile-flag-flips.md` §0.

**Rule of thumb for next time.** `missing_pg` with a *live* legacy row and
`pg_hash IS NULL`, where sibling keys either side landed fine, means a **dropped
CT event**, not a mapper bug — go straight to the watcher's advance log for that
window and check whether some *other* table moved the watermark past it. If the
gap is older than CT's 2-day retention the watcher can never redeliver it; only a
re-ingest heals it.

## 2026-07-06 — checkins unconverged >4h, both sites (Cin_Date_in edits eaten by idempotency gate)

**Symptom.** `level_drift_alert` fired for `checkins`: hfville 1 row (self-healed
before investigation), hfhotel 2 rows (`CH26-006020`, `CH26-006039`) unconverged
1–2 days. Same signature on both rows: PG and legacy agreed on room / customer /
expected checkout / status, but legacy `Cin_Date_in` was *later* than canonical
`cin_checkin_time` by 5h24m and 2h33m (non-constant offsets → not timezone).

**Evidence.** Sync logs showed the CT watcher consumed both legacy edits as
`ingested=0 skipped=1` at exactly the edit timestamps (+8s / +16s) — delivered,
loaded, then idempotency-skipped. `legacy_sync_status`: 0 failures, watermark live.
No writeback jobs for either key (not an echo). PG `updated_at` frozen strictly
before each edit; event log had zero events after the edits — exactly what an
`Ok(None)` skip predicts. 22 prior HF Hotel occurrences in 14 days, 100%
self-resolved when checkout (a *compared* field change) forced a full re-apply.

**Cause.** Idempotency-gate blind spot in the checkin CT mapper —
`existing_matches()` (`hotel-backend/src/sync/mappers/checkin.rs`) compared only
status / amounts / checkout-time / guarded cust_no; `fetch_existing()` didn't even
SELECT `cin_checkin_time` or `cin_expected_checkout`. A legacy edit touching only
`Cin_Date_in` (edit-date dialog / re-save) or only `Cin_Room_Out` (stay extension)
changed no compared field → skip → watermark advanced → CT delta aged past 2-day
retention. Both fields ARE reconcile-hash inputs, so the sweep flagged rows it
could never close (force-converge covers customers/rooms only). Same structural
class as the 2026-07-01 write-once `legacy_room_no`/`legacy_cust_no` fix: a
hash-input field invisible to the skip comparator. The booking mapper was checked
and is NOT affected (it already compares `book_checkin`/`book_checkout`).

**Fix.** Added `cin_checkin_time` + `cin_expected_checkout` to
`fetch_existing()` / `ExistingCheckIn` / `existing_matches()`; `update_existing`
already wrote both through plainly, so a mismatch now converges in one re-apply.
Two regression tests (`existing_matches_is_false_when_only_checkin_time_differs`,
`..._expected_checkout_differs`). No data heal needed: the 2 open rows converge at
checkout like all 22 priors, and the auto-resolve sweep closes them. **Alert
verdict: true positive, correctly calibrated** — it caught genuine durable
divergence the sweep could not close; no threshold change.

**Rule of thumb for next time.** If a `checkins`-family unconverged row shows a
single-field diff in the pg/mssql JSONs and the sync logs show `skipped=1` at the
legacy edit time with zero errors, suspect a comparator gap before a dropped
event: diff the fields in `existing_matches()` against the reconcile-hash inputs
for that entity — they must be a superset.

---

## 2026-08-10 — HF Ville calendar: 4 nights dropped, CT aged out (true positive)

**Alert.** `level_drift_alert` at `site=hfville`, `mirror_ht_room_calendar`,
1 unresolved row, detected 2026-08-09 08:10Z (15:10 ICT), still open 21h later
across every sweep. `missing_pg`, legacy 1663 vs pg 1659 (delta 4). A separate
`HF Ville DEGRADED→UP` pair fired 2026-08-07 09:26→09:28 ICT (WireGuard down
during a PPPoE re-dial, self-healed in 2 min) — **not** the same event: two days
apart, and the app path stayed up (`ts=0 wg=1`). That one needed no action.

**Why the first hypothesis was wrong.** The leading guess was the #282 class —
a false `missing_pg` where the night is present but `rcal_legacy_id` is NULL, so
the `WHERE rcal_legacy_id IS NOT NULL` count cannot see it. That was killed by
direct query: the four `(107, night)` tiles were **absent from
`ht_room_calendar` entirely**, not merely unbound. Do not assume a known
false-positive class explains a new row on the same probe.

**Evidence.** Set-diff of `DISTINCT (room_no, night)` over the era window
resolved the delta to exactly four pairs, nothing in the opposite direction:
room **107**, nights `2026-07-28`, `08-05`, `08-06`, `08-08` (legacy ids 4692,
4799, 4815, 4832). Ruled out in order: room-mapping artifact (room 107 present,
`room_id=7`, 56 tiles, 0 orphan calendar rows); FK defer (all five referenced
check-ins present in canonical); hidden characters in `room_no` (byte-identical
`313037` on the row that synced, 4648, and the four that did not); stalled
watermark (**current at 43660 = `ct_current`**, never held). Canonical held 169
tiles with ids *above* the last synced one and its max id equalled legacy's —
so this was a **selective silent drop, not a stall**. A full id-level diff found
zero canonical ids absent from legacy, and the only other in-era misses
(2361, 2568, 2569, 4223/4224/4225, 4567, 4609) are legacy **duplicate rows for
one `(room, night)`** — legacy permits them, canonical is unique on
`(rcal_room_id, rcal_date)`, so the upsert keeps one and orphans the rest by
design. Benign, and worth not re-deriving.

**Cause (strong hypothesis, not proven).** Transient loss of the legacy link
mid-tick, with the affected keys skipped rather than retried. The Ville uplink
is genuinely unstable — **211 `Timed out in bb8` plus 109 connection errors in
72h** on `sync-hfville` — and the watermark advanced throughout. Room 107 is
coincidence: it is whichever room happened to have activity during a blip. The
CT versions for all four are now far below `CHANGE_TRACKING_MIN_VALID_VERSION`
(42589 vs current 43660), so **no tick can ever redeliver them** — precisely the
case the alert text describes. This is the same structural family as the
2026-05-18 silent-drop incident (per-key failures must gate the watermark on
`!errored`); whether an un-gated path survives that fix is **not established
here** and is tracked separately.

**Fix.** `backfill_room_calendar --since=2026-03-26` against `sync-hfville`
(re-drives the mapper from a direct legacy SELECT, so it bypasses the aged-out
CT window). Dry run independently reported the same 4 candidates; live run wrote
4, errored 0, all with `checkin_resolved=true`. The mirror probe re-hashes and
closes its own aggregate row — the bin deliberately does not touch
`ht_reconcile_log`. **Alert verdict: true positive, correctly calibrated.** It
caught durable, unrecoverable divergence the sweep could not close. No threshold
change.

**Rule of thumb for next time.** On a `mirror_ht_room_calendar` `missing_pg`
row, the first query is whether the `(room, night)` tiles **exist at all** —
that single check separates the #282 false-positive class (tile present,
`rcal_legacy_id` NULL) from a genuine drop, and the two demand opposite
responses. Then check `ct_min` against the event's version: below it, CT can
never redeliver and only a backfill will close the gap.

**Correction (2026-08-10, same day, issue #283 audit).** The "cause" paragraph
above is WRONG and is retained only as a record of the misdiagnosis. The
connection-failure hypothesis was falsified by bucketing all 11 bb8/transport
failure windows over 240h against the six CT events for the dropped rows —
none coincide; the watcher was healthy at every drop, and during the real
outages the 2026-05-18 `!errored` gating held the watermark correctly. The
actual mechanism is the **unverified tick ceiling**: the watcher sampled
`CHANGE_TRACKING_CURRENT_VERSION()` and advanced the watermark to it in the
same tick, but without snapshot isolation a transaction can commit with a
version at-or-below the sampled ceiling while remaining invisible to the
`CHANGETABLE` read taken moments later — and the skipped range is never
re-read. Production proof: tick `2026-08-05T07:20:06Z` applied row 4798
(room 105) but not its sibling 4799 (room 107) — both written by the same
iHOTEL check-in `CH26-001753` — then advanced 41503→41516 over it. Fix: the
CT ceiling settle gate in `bin/sync.rs` (a sampled ceiling only becomes an
advance target on a later tick, once every CHANGETABLE read provably
post-dates it), plus hard-error handling of CT control columns and gating of
three un-gated per-key skips. The `MAX+1`-style "hole below a current max"
diagnostic in the rule-of-thumb above still stands — that part was right.
