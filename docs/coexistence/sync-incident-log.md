# Sync incident log

Running root-cause ledger for coexistence sync / monitoring alerts, newest first.
Format per entry: symptom → evidence → cause → fix. Keep entries short; link code
and prior entries instead of re-explaining. Vocabulary: "sync lag / unconverged"
for transients, "durable divergence" only for rows that resist multiple sweep cycles
(see CLAUDE.md "Vocabulary note").

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
