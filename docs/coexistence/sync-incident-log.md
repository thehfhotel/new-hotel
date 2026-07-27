# Sync incident log

Running root-cause ledger for coexistence sync / monitoring alerts, newest first.
Format per entry: symptom → evidence → cause → fix. Keep entries short; link code
and prior entries instead of re-explaining. Vocabulary: "sync lag / unconverged"
for transients, "durable divergence" only for rows that resist multiple sweep cycles
(see CLAUDE.md "Vocabulary note").

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
