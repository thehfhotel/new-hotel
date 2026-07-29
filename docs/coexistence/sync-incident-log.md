# Sync incident log

Running root-cause ledger for coexistence sync / monitoring alerts, newest first.
Format per entry: symptom → evidence → cause → fix. Keep entries short; link code
and prior entries instead of re-explaining. Vocabulary: "sync lag / unconverged"
for transients, "durable divergence" only for rows that resist multiple sweep cycles
(see CLAUDE.md "Vocabulary note").

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
