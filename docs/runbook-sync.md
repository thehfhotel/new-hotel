# Runbook — CT Watcher (Phase 5.5 production cutover)

Operator-facing reference for `bin/sync` (the Change Tracking watcher).
Companion to `docs/architecture.md` §3.6d, §3.7, §11.

This binary also runs one **non-CT** job: a read-only per-tick *poll* of
the legacy `HT_Round_Bill` cashier-round ledger into canonical
`ht_shifts` (shipped 2026-06-26). Because it is a plain poll and not
Change Tracking, it has no watermark and never appears in
`legacy_sync_status` / `legacy_ct_state` — don't triage it via the CT
observability below. See Section 10.

This runbook assumes the reader is the on-call operator for HF Hotel.
Receptionist test plan (Section 7) can be handed to the receptionist
team verbatim once the operator is ready to verify.

> **Status as of v2.45.0 (2026-04-26):** code shipped, default-disabled.
> Cutover requires explicit operator action — see Section 4.

---

## 1. Bootstrap procedure

The watcher refuses to start with `last_seen_version = 0` (the migration's
seed value). This protects against the "process every CT row from
time-zero" footgun. Bootstrap seeds canonical PG state from MSSQL via the
existing reconcile path AND stamps the watermark to the current
`CHANGE_TRACKING_CURRENT_VERSION()` so the next watcher start resumes
from sub-second tip-of-stream.

Run **once** on production before flipping `LEGACY_SYNC_ENABLED=true`:

```bash
# From the production server, in the deployment directory:
docker compose --profile legacy run --rm sync ./sync --bootstrap
```

Expected log output:

```
[INFO] Phase 5.5 bootstrap — cold-seeding canonical PG + CT watermark
[INFO] [bootstrap] Connected to PostgreSQL
[INFO] [bootstrap] Connected to legacy MSSQL
[INFO] [bootstrap] Running reconcile (UPSERT mode)…
[INFO] [Sync] Customers (Upsert): … added, … updated, … unchanged in …ms
[INFO] [Sync] Rooms (Upsert): … added, … updated, … unchanged in …ms
[INFO] [Sync] Bookings (Upsert): … added, … updated, … unchanged in …ms
[INFO] [Sync] Check-ins (Upsert): … added, … updated, … unchanged in …ms
[INFO] [bootstrap] Reconcile complete
[INFO] [bootstrap] Read CHANGE_TRACKING_CURRENT_VERSION() from MSSQL current_version=…
[INFO] [bootstrap] CT watermark stamped — bootstrap complete. Operator may now flip LEGACY_SYNC_ENABLED=true.
```

Re-running is safe — the reconcile is idempotent (UPSERT-by-hash) and
the watermark stamp overwrites unconditionally.

---

## 2. Env-var matrix

Every variable consumed by `bin/sync`. Defaults are what
`docker-compose.yml` ships.

| Var | Meaning | Default | When to flip |
|---|---|---|---|
| `LEGACY_SYNC_ENABLED` | Master switch. `true` = main loop runs; `false` = binary logs + exits 0. | `false` | After successful bootstrap (Section 1) AND `LEGACY_SYNC_SHADOW_MODE=false` soak. |
| `LEGACY_SYNC_SHADOW_MODE` | `true` = run mappers, log "would publish", roll back PG TX (canonical state untouched). | `true` | Flip to `false` after 24h shadow-mode soak shows zero mapper errors. |
| `LEGACY_SYNC_TABLE_ALLOWLIST` | Comma-separated CT-enabled table names. Empty = all 10 tables. | `` (all) | Set during incremental rollout (e.g. `HT_Customers,HT_Rooms` first). |
| `LEGACY_SYNC_ALLOW_COLD_REPLAY` | `true` = allow start with `last_seen_version=0` (replay all CT history). | `false` | NEVER in production. Test-only escape hatch — bootstrap is the supported path. |
| `LEGACY_SYNC_ALLOW_OVERFLOW` | `true` = start even when the watermark is already past CT retention on one or more tables (incremental rows since the watermark are silently skipped — DATA LOSS). | `false` | NEVER in production. Bootstrap is the supported path. See Section 4b for the shadow-mode trap that makes overflow a foreseeable scenario. |
| `LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP` | `true` = allow `--bootstrap` to run while `LEGACY_SYNC_ENABLED=true` (i.e. against a live deployment). The bootstrap snapshot's `DELETE FROM legacy_mirror.<table>` races the watcher's `mirror_source='ct'` UPSERTs and can clobber real-time CT writes that landed during the snapshot window. | `false` | NEVER in production. Supported procedure: stop the watcher first (`LEGACY_SYNC_ENABLED=false` + redeploy), bootstrap, re-enable. |
| `LEGACY_SYNC_RECONCILE_MODE` | Mode for the demoted `scheduler::sync::run_sync` job. `diff_only` = log drift to `ht_reconcile_log`; `upsert` = legacy 5-min-style UPSERT into `ht_*_legacy`. | `diff_only` | Flip to `upsert` ONLY if the CT watcher is operationally disabled and you need the legacy safety net to keep canonical state in sync. |
| `CT_POLL_INTERVAL_MS` | How often the watcher polls MSSQL CT. Lower = lower latency, higher load. | `1000` (1s) | Increase only if MSSQL load is a concern. |
| `LEGACY_SYNC_CT_KEEPALIVE_SECS` | `> 0` = a sibling task runs a read-only `CHANGE_TRACKING_CURRENT_VERSION()` on this cadence to keep the CT version machinery warm. The per-tick `SELECT 1` keeps the *connection* warm but not CT, so on a quiescent overnight iHOTEL the watchdog's first CT probe after a lull can answer slower than its 30s budget → the benign `:information_source: CT watermark idle — probe timed out`. Keeping CT hot makes that probe return fast (and classify a *real* backlog correctly instead of masking it as a timeout). | `0` (off) | Flip to `45` per-site (under the 60s pool idle_timeout) if probe-timeout pages are noisy / you want the real-backlog probe to be reliable. Read-only; no writeback; safe to enable without reception coordination. |
| `SYNC_TEST_SKIP_MSSQL_PROBE` | Test-only. `true` = skip the bb8-tiberius probe in `tests/test_sync_phase54_integration.rs::mssql_stub`. | unset | Set when running pure-PG tests without legacy MSSQL access (saves 30s per process). |
| `DATABASE_URL` | PG DSN for the watcher's writes. | (set in compose) | Standard PG creds — same as backend. |
| `DB_SERVER` / `DB_NAME` / `DB_USER` / `DB_PASSWORD` | Legacy MSSQL connection. | (set in compose) | Match the legacy app. |
| `SLACK_WEBHOOK_URL` | Where schema drift / retention overflow / cold-replay refusal alerts go. | (set in compose) | Standard ops Slack. |
| `RUST_LOG` | Tracing filter. | `hotel_backend=info,sync=info` | Add `,debug` to tighten when investigating. |

---

## 3. Slack alert meanings

The watcher surfaces three categories of alerts to Slack. All are
prefixed so they're triagable in one glance.

| Alert | Meaning | Operator action |
|---|---|---|
| `:warning: CT watcher REFUSED TO START` (schema fingerprint) | Legacy MSSQL columns drifted from the captured baseline. The watcher refuses to project against an unknown shape. | Run `./scripts/writeback-fingerprint.sh` and follow the README to update the baseline before restarting. Same workflow as the writeback worker — fingerprint is shared. |
| `:no_entry: CT watcher REFUSED TO START` (cold replay) | `last_seen_version=0` and `LEGACY_SYNC_ALLOW_COLD_REPLAY != true`. | Run the bootstrap procedure (Section 1). |
| `:no_entry: CT watcher REFUSED TO START` (retention overflow) | At startup, `MIN_VALID_VERSION` is higher than the watermark on at least one CT-tracked table. CT history we'd need to catch up has aged out. The pre-flight check refuses rather than silently skipping rows. | Run the bootstrap procedure (Section 1) — `--bootstrap` re-snapshots canonical PG and stamps the watermark to `CHANGE_TRACKING_CURRENT_VERSION()`. After bootstrap, restart the watcher. See Section 4b for the shadow-mode trap that triggers this. |
| `:no_entry: Bootstrap REFUSED — live deployment` | Operator ran `--bootstrap` while `LEGACY_SYNC_ENABLED=true`. The snapshot would race the live CT watcher's UPSERTs and clobber `mirror_source='ct'` rows. | Stop the watcher first (set `LEGACY_SYNC_ENABLED=false` and redeploy), then run `--bootstrap`, then re-enable. Set `LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP=true` ONLY if you accept the race window. |
| `:rotating_light: CT retention overflow` | A specific table's `MIN_VALID_VERSION` is higher than the watermark — CT history we needed has aged out (default retention 2 days). | Re-bootstrap (Section 1). The reconcile inside `--bootstrap` will catch us up via the canonical UPSERT path. |
| Mapper consecutive-failure threshold (future) | Per-table `legacy_sync_status.consecutive_failures` exceeds N. | Inspect `legacy_sync_status.last_error` for that table; check mapper logs for the failing CT row payload. |

**Watermark watchdog — what reaches Slack (2026-06-30).** The watchdog
no longer Slacks the benign `:information_source: CT watermark idle —
probe timed out (informational)` note or its paired `:white_check_mark:
CT watermark RECOVERED` all-clear — that pattern self-clears within a tick
and was pure overnight noise. It is still logged (`tracing::info`,
`"informational (Slack-suppressed); escalation still armed"`) and visible
on `/api/new/sync/status`. The watchdog STILL pages for the actionable
cases, each of which keeps its all-clear: a **sustained** probe outage
(`:rotating_light: legacy probe unreachable Nmin`, after
`LEGACY_SYNC_PROBE_OUTAGE_ESCALATION_SECS`, default 20 min), a **confirmed
backlog** (`:rotating_light: CT watermark STUCK`, probe shows
`ct_current > watermark`), and a **monotonicity violation**
(`:rotating_light: CT watermark anomaly`, `ct_current < watermark`). To
reduce the underlying probe timeouts, see `LEGACY_SYNC_CT_KEEPALIVE_SECS`
in Section 2.

---

## 4. Cutover procedure

The supported sequence from "code deployed, default-disabled" to "live
production CT watcher". Each step gates on the prior step's success.

> **Read first:** Section 4a documents an operator-flip-revert pitfall
> that affects every step below where the procedure says "edit `.env`".
> Skipping it will silently revert your flag flips on the next
> `git push master`.

1. **Deploy code.** `git push master` triggers the pipeline.
   `docker-compose.yml` ships `LEGACY_SYNC_ENABLED=false` and
   `LEGACY_SYNC_SHADOW_MODE=true`. The `sync` container starts and
   immediately exits 0 (intentional — see env matrix).
2. **Bootstrap.** Run `docker compose --profile legacy run --rm sync
   ./sync --bootstrap` (Section 1). Verify watermark > 0:
   ```bash
   docker exec new-hotel-db psql -U postgres -p 5439 -d hotelnew \
     -c 'SELECT * FROM legacy_ct_state;'
   ```
3. **Shadow soak (24h).** Set `LEGACY_SYNC_ENABLED=true`,
   `LEGACY_SYNC_SHADOW_MODE=true` in the `.env` on the deploy host,
   then `docker compose --profile legacy up -d --force-recreate sync`.
   Tail logs: `docker logs -f new-hotel-production-sync-1`. Look for
   `would publish (shadow mode)` lines; verify no mapper errors. Check
   `legacy_sync_status.last_error` is NULL for every row after 24h.
   ⚠️ **Editing `.env` directly will be reverted on the next master
   push — see Section 4a.**
4. **Go live.** Set `LEGACY_SYNC_SHADOW_MODE=false`, recreate the
   container. Now CT changes flow into canonical PG state and the
   event bus. ⚠️ **Same `.env`-revert pitfall — see Section 4a.**
5. **24h live soak.** Run the receptionist test plan (Section 7).
   Verify `event_log` rows with `source_kind='legacy_app'` accumulate.
   Verify `ht_reconcile_log` stays empty (no drift between CT watcher
   and the demoted reconcile).
6. **Demote `scheduler::sync`.** Already demoted automatically by
   v2.45.0 (`LEGACY_SYNC_RECONCILE_MODE` defaults to `diff_only`).
   Confirm by checking the next reconcile tick logs: should say
   `[Sync] Customers (DiffOnly): …`.

---

## 4a. Known operator pitfall — `.env` is rewritten by every CI deploy

**Symptom.** You SSH to evergreen, edit `~/new-hotel-production/.env` to
flip `LEGACY_SYNC_ENABLED=true`, recreate the `sync` container, watch
events flow. A few hours (or days) later someone merges an unrelated
PR, the CI deploy job runs, and the watcher silently goes back to
`exited` state because `.env` was reset to `LEGACY_SYNC_ENABLED=false`.

**Why.** The deploy job in `.github/workflows/docker-build.yml` (search
for the `Deploy` step under `jobs.deploy.steps`) writes `.env` from
scratch on every push, sourcing values from GitHub Secrets. The current
heredoc only contains the secrets the production runtime requires
(`DB_*`, `POSTGRES_*`, `SLACK_WEBHOOK_URL`); it does NOT carry
`LEGACY_SYNC_ENABLED` / `LEGACY_SYNC_SHADOW_MODE`. Anything you write
into `.env` by hand that isn't in that heredoc is lost the next time
master is pushed.

The `docker-compose.yml` for the `sync` service falls back to
`LEGACY_SYNC_ENABLED=false` and `LEGACY_SYNC_SHADOW_MODE=true` when the
`.env` doesn't supply them — which is why the watcher silently
"reverts" to disabled-shadow rather than failing loudly.

### Recommended remediation — promote the flags to GitHub Secrets

This is the lowest-friction, smallest-blast-radius fix. Do this BEFORE
you flip the watcher live for the first time.

1. **Set the secrets on the repo:**
   ```bash
   gh secret set LEGACY_SYNC_ENABLED      --body "true"
   gh secret set LEGACY_SYNC_SHADOW_MODE  --body "true"   # or "false" once soaked
   ```

2. **Add them to the deploy job's heredoc.** In
   `.github/workflows/docker-build.yml`, locate the `Deploy` step under
   `jobs.deploy.steps` (the block that begins
   `# Write .env file from GitHub Secrets`). Add:
   ```yaml
   env:
     # …existing env vars…
     LEGACY_SYNC_ENABLED:     ${{ secrets.LEGACY_SYNC_ENABLED }}
     LEGACY_SYNC_SHADOW_MODE: ${{ secrets.LEGACY_SYNC_SHADOW_MODE }}
   ```
   And inside the `.env` heredoc body:
   ```bash
   echo "LEGACY_SYNC_ENABLED='${LEGACY_SYNC_ENABLED}'"
   echo "LEGACY_SYNC_SHADOW_MODE='${LEGACY_SYNC_SHADOW_MODE}'"
   ```

3. **Add them to the secret-validation loop** (the `for var in DB_SERVER
   DB_NAME …` block) so a missing secret fails the deploy loudly
   instead of producing an empty value compose silently treats as the
   default.

4. **Push the workflow change.** From now on, flipping the flag means
   `gh secret set LEGACY_SYNC_ENABLED --body "true"` plus a re-run of
   the deploy workflow (or any push to master). No more SSH-to-edge
   drift.

> The wiring is intentionally NOT shipped in the same commit as this
> documentation update — the operator should decide WHEN to flip the
> flag for the first time before exposing it via secrets.

### Alternative — move the gate to a PG flag table

If you want operator changes to take effect mid-tick without a deploy
or a container restart, move the flag out of the env into a PG row.

1. **Add a migration** under `migrations/pg/` (e.g.
   `020_legacy_sync_control.sql`):
   ```sql
   CREATE TABLE IF NOT EXISTS legacy_sync_control (
       id              BIGINT       PRIMARY KEY DEFAULT 1 CHECK (id = 1),
       enabled         BOOLEAN      NOT NULL DEFAULT false,
       shadow_mode     BOOLEAN      NOT NULL DEFAULT true,
       updated_at      TIMESTAMPTZ  NOT NULL DEFAULT now(),
       updated_by      TEXT
   );
   INSERT INTO legacy_sync_control (id) VALUES (1) ON CONFLICT DO NOTHING;
   ```

2. **Re-point `bin/sync.rs`** to read this row at the top of every poll
   tick instead of reading the env once at startup. The watcher
   evaluates the flag fresh, so the operator can flip it via:
   ```sql
   UPDATE legacy_sync_control
      SET enabled = true, shadow_mode = false, updated_at = now(),
          updated_by = 'on-call-operator'
    WHERE id = 1;
   ```

3. **Audit log.** Because every flip is a row update, you get a free
   audit trail (extend the table with a history side-table if you want
   per-flip records). Survives `.env` resets, no deploy needed, more
   granular control (you can disable a single table via additional
   columns later).

The PG-flag approach trades a small amount of code complexity for
operational flexibility. Pick it if you expect to flip the flag more
than a couple of times in the cutover window; pick the GH-secrets
approach if you'll flip it twice and forget about it.

### Rollback path — re-enabling shadow mode mid-incident

If a Phase 5.5 mapper bug is detected in production and you need to
switch the watcher back to shadow mode WITHOUT taking it offline:

**Using the GH-secrets approach (recommended above):**
```bash
gh secret set LEGACY_SYNC_SHADOW_MODE --body "true"
gh workflow run docker-build.yml --ref master   # re-runs deploy with new .env
# OR for a faster path that skips image rebuild:
ssh deploy@evergreen \
  "cd /home/nut/new-hotel-production && \
   sed -i \"s/^LEGACY_SYNC_SHADOW_MODE=.*/LEGACY_SYNC_SHADOW_MODE='true'/\" .env && \
   docker compose --profile legacy up -d --force-recreate sync"
# IMPORTANT: the sed-on-evergreen path is only stable until the next
# master push; immediately follow with the `gh secret set` so the
# next deploy carries the flag forward.
```

**Using the PG-flag approach:**
```sql
UPDATE legacy_sync_control SET shadow_mode = true, updated_at = now() WHERE id = 1;
```
No restart needed — the next poll tick reads the new value. Use this
if you wired the alternative remediation above.

---

## 4b. The shadow-mode 2-day CT-retention trap

**Symptom.** After 1–2 days of shadow-mode soak, every CT-tracked
table simultaneously starts firing `:rotating_light: CT retention
overflow` alerts. On restart, the watcher refuses to start with
`:no_entry: CT watcher REFUSED TO START — retention overflow`. The
soak metrics looked perfect right up until the moment everything went
red.

**Why.** Shadow mode rolls back the PG transaction at the end of every
tick. The watermark UPDATE on `legacy_ct_state.last_seen_version`
lives inside that transaction, so it gets rolled back too — the
watermark is **frozen** for the entire shadow soak. Meanwhile SQL
Server's CT garbage collector keeps running on its own schedule
(default retention: **2 days**), so `MIN_VALID_VERSION` marches
forward. Once `MIN_VALID_VERSION > last_seen_version`, the row history
we'd need to catch up incrementally is gone.

The per-table `last_processed_at` counters DO update during the soak
(`bump_skipped` writes outside the transaction for observability),
which is why the dashboard's per-table freshness checks stay green —
masking that the watermark itself is stale.

**The startup guardrail.** As of v2.49.4 the watcher runs a pre-flight
`check_retention()` against every CT-tracked table at boot. If any
table has overflowed, the watcher refuses to start (parallel to the
cold-replay guardrail) — surfacing the trap at restart time instead of
letting it silently skip rows on the next live tick.

**Recovery (this sequence is what we did on 2026-04-28):**

1. Flip live first: `gh secret set LEGACY_SYNC_SHADOW_MODE -b "false"`
   then `gh workflow run docker-build.yml`. Without this the trap will
   re-arm in another 2 days.
2. Wait for deploy. The new container will refuse to start with
   `retention overflow` because the watermark is still frozen.
3. Run bootstrap from the deploy host:
   ```bash
   cd /home/nut/new-hotel-production
   docker compose --profile legacy run --rm sync ./sync --bootstrap
   ```
   Snapshots all 10 tables into canonical PG and stamps the watermark
   to `CHANGE_TRACKING_CURRENT_VERSION()`.
4. The next deploy or container restart now passes the guardrail and
   live polling resumes from the fresh watermark.

**Prevention for future shadow soaks.** Either:
- Keep shadow soaks shorter than the legacy DB's CT retention period,
  or
- Increase CT retention on the legacy DB to comfortably exceed the
  longest planned shadow soak. To check current setting:
  ```sql
  SELECT retention_period, retention_period_units_desc, is_auto_cleanup_on
    FROM sys.change_tracking_databases
   WHERE database_id = DB_ID('HotelNew');
  ```
  To extend (example: 7 days) — coordinate with the legacy app vendor
  before changing:
  ```sql
  ALTER DATABASE HotelNew
    SET CHANGE_TRACKING (CHANGE_RETENTION = 7 DAYS, AUTO_CLEANUP = ON);
  ```

---

## 5. Rollback procedure

If something goes wrong during cutover, the reverse sequence:

> **Reminder:** every step that says "set X in `.env`" is subject to
> the operator-flip-revert pitfall in Section 4a. Follow each `.env`
> edit with a `gh secret set` for the matching key (once the GH-secrets
> remediation is wired) so the next CI deploy doesn't undo it.

1. **Disable the watcher.** Set `LEGACY_SYNC_ENABLED=false` in `.env`.
   `docker compose --profile legacy up -d --force-recreate sync` —
   the container will exit cleanly.
2. **Re-enable the legacy reconcile UPSERT path** (the safety net).
   Set `LEGACY_SYNC_RECONCILE_MODE=upsert` in `.env`. Restart backend
   to pick up the change: `docker compose restart backend`.
3. **Clear the watermark IF the watcher state is suspect.** Only do
   this if you intend to re-bootstrap before re-enabling:
   ```bash
   docker exec new-hotel-db psql -U postgres -p 5439 -d hotelnew \
     -c 'UPDATE legacy_ct_state SET last_seen_version = 0 WHERE id = 1;'
   ```
4. **Re-bootstrap before re-enabling.** Repeat Section 1 then Section 4.

---

## 6. Known limitations

* **Payment-cancel cascade race window.** When the .NET app cancels a
  payment in `HT_CheckIn_Pay`, the cascade to `cin_paid_amount` in our
  `ht_checkins` (via `apply_payment_aggregate`) converges within a few
  CT ticks (seconds), not the same tick. Cutover testing should
  include a payment-cancel scenario (Section 7, scenario 5b) to verify
  the cascade lands within ~3 ticks. If the canonical row stays stale
  for >10 seconds after the cancel, file a bug — that's beyond the
  designed convergence window.

* **Receipt `pay_method='cash'` default.** The `ReceiptMapper` defaults
  every emitted `PaymentReceived` event's `pay_method` to `'cash'`.
  Actual tender breakdown (cash / credit / transfer) lives in the
  legacy `HT_Receipt_H.pay_*` columns and isn't projected yet — Phase
  5.6 will enrich the receipt mapper to read those columns. Until
  then, downstream reporting that depends on tender method should
  query `ht_payments` directly for the breakdown, NOT the projected
  `pay_method` field.

* **MSSQL probe in test runs.** `tests/test_sync_phase54_integration.rs`'s
  `mssql_stub` opens a tiberius pool to satisfy
  `apply_checkin_aggregate`'s `Option<&DbPool>` parameter. When MSSQL
  is unreachable the bb8-tiberius probe blocks ~30s before timing out,
  adding 30s per test process. Set `SYNC_TEST_SKIP_MSSQL_PROBE=true`
  to short-circuit the probe.

---

## 7. Receptionist test plan

Hand this section to the receptionist team for the 24h live soak
(Section 4 step 5). All five scenarios should propagate to our app
within ~2 seconds of the .NET-app action.

> **Setup.** Open the legacy .NET app on one terminal and our app on
> a second screen. Have a watch with a second hand visible.

### Scenario 1 — Create booking

1. In the .NET app, create a new booking for a future date (any room,
   any guest).
2. Note the time. Within ~2 seconds, the booking should appear in our
   app's bookings list.
3. ✅ PASS = appears within 5s. ❌ FAIL = doesn't appear within 30s.

### Scenario 2 — Cancel booking

1. In the .NET app, cancel an existing booking (one you can recreate).
2. Within ~2 seconds, the same booking in our app should show
   "ยกเลิก" / cancelled status.
3. ✅ PASS = status flips within 5s.

### Scenario 3 — Check-in

1. In the .NET app, check a guest in (walk-in or against a booking).
2. Within ~2 seconds, the check-in should appear in our app's active
   stays / dashboard.
3. ✅ PASS = appears within 5s.

### Scenario 4 — Check-out

1. In the .NET app, fully check the guest out (every room on the
   folio).
2. Within ~2 seconds, our app should:
   - Show the booking status as "ออกแล้ว" / completed (if there was
     a parent booking).
   - Move the check-in row out of "active stays" into history.
3. ✅ PASS = both visible within 5s.

### Scenario 5a — Add payment

1. In the .NET app, add a payment to an active check-in (any tender —
   cash / credit / transfer).
2. Within ~2 seconds, our app should show:
   - The check-in's outstanding balance dropped by the payment amount.
   - A new row in payments / receipts.
3. ✅ PASS = balance updates within 5s.

### Scenario 5b — Cancel payment (race-window check, see Section 6)

1. In the .NET app, cancel the payment you just added.
2. Within ~10 seconds (note: longer window than the others), our app's
   check-in balance should restore to the pre-payment value.
3. ✅ PASS = balance restores within 10s. ❌ FAIL = balance stays
   stale for >30s — file a bug.

---

## 8. Observability dashboard pointers

Where to look for "is the watcher healthy?" data.

* **Per-table CT watcher progress + errors:**
  ```sql
  SELECT * FROM legacy_sync_status ORDER BY last_processed_at DESC;
  ```
  Look for: `last_error` should be NULL; `consecutive_failures`
  should be 0; `last_processed_at` should be recent (within
  ~`CT_POLL_INTERVAL_MS`).

* **Watermark progress:**
  ```sql
  SELECT * FROM legacy_ct_state;
  ```
  `last_seen_version` should monotonically increase; `last_polled_at`
  should be recent.

* **CT-published events (the watcher's output):**
  ```sql
  SELECT event_type, count(*)
    FROM event_log
   WHERE created_at > now() - interval '1 hour'
     AND payload->>'source_kind' = 'legacy_app'
   GROUP BY 1 ORDER BY 2 DESC;
  ```

* **Drift tripwire (should be empty in steady state):**
  ```sql
  SELECT table_name, count(*)
    FROM ht_reconcile_log
   WHERE resolved_at IS NULL
   GROUP BY 1 ORDER BY 2 DESC;
  ```
  Any non-zero count means the demoted `scheduler::sync` reconcile
  saw a row whose hash didn't match — investigate the listed
  `(table_name, legacy_pk)` against the canonical PG row to find
  what the CT watcher missed.

---

## 9. Phase 6 — drift-reconcile safety net

**As of v2.54.0**, the demoted reconcile job is wired with an alerting
loop. The CT watcher remains the real-time path; the reconcile is a
slower safety net that surfaces rows the watcher missed (CT retention
overflow, transient mapper bug, schema regression).

### Cadence

* **15 minutes**, on the quarter-hour (`0 */15 * * * *`).
* Polling more often would only add legacy-MSSQL load without changing
  the recovery posture — operator response time to a Slack alert
  dominates real-time latency, and the CT watcher's sub-second path
  already covers the latency-sensitive case.

### Alert mechanic

At the end of each tick the job runs:

```sql
SELECT table_name, count(*)
  FROM ht_reconcile_log
 WHERE resolved_at IS NULL
   AND detected_at > now() - interval '1 hour'
 GROUP BY table_name;
```

If any `table_name` count exceeds the configured threshold (default
**50**, override via `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`), one
Slack message is fired listing every offending table:

```
:rotating_light: *Reconcile drift threshold exceeded* :rotating_light:
The drift-reconcile job recorded more than 50 unresolved
ht_reconcile_log rows for the following table(s) in the last hour:
• `customers`: 73 unresolved rows in last hour
• `bookings`: 412 unresolved rows in last hour
_Investigate via docs/runbook-sync.md §9 (Phase 6 drift alert)._
```

Logs always carry the same data even when Slack isn't configured —
look for `[Sync] Drift alert: table exceeds reconcile-log threshold`
in the backend container logs.

### Investigating a drift alert

Two probable causes — distinguish them before deciding the action.

**Step 1 — pull the offending rows** for the alerting `table_name`:

```sql
SELECT detected_at, legacy_pk, pg_hash, mssql_hash,
       mssql_row_json, pg_row_json
  FROM ht_reconcile_log
 WHERE resolved_at IS NULL
   AND table_name = 'customers'   -- replace with the alerting table
   AND detected_at > now() - interval '1 hour'
 ORDER BY detected_at DESC;
```

**Step 2 — classify**:

| Pattern | Likely cause | Action |
|---|---|---|
| `pg_hash IS NULL` on most rows. | Bulk PG-miss — canonical rows never landed. Most often: CT retention overflow (Section 4b) or watcher offline window. | Cross-check `legacy_sync_status.last_processed_at` — stale on the matching MSSQL table? Run `--bootstrap` (Section 1). After bootstrap, mark the rows resolved: `UPDATE ht_reconcile_log SET resolved_at = now() WHERE resolved_at IS NULL AND table_name = 'customers';` |
| `pg_hash` and `mssql_hash` both populated, both differ on every row. | Mapper-projection bug — the CT watcher is writing canonical state but with a different shape than reconcile expects. | Pick one row, compare `mssql_row_json` to the canonical PG row, identify the diverging column. Fix the mapper in `src/sync/mappers/`, ship via CI. Mark rows resolved AFTER the next reconcile tick goes clean. |
| Single isolated row, no pattern across `table_name`. | One-off CT overflow or a hand-edit on the legacy DB after a watcher restart. | Re-fire the CT mapper for that PK by writing a no-op UPDATE on the source MSSQL row, OR resolve by hand if the canonical PG state matches business intent. |
| Counts climb monotonically each tick on the same `table_name`. | The watcher is offline for that table, OR a mapper consistently rejects the CT row. | Check `legacy_sync_status` for the matching MSSQL table — `last_error` non-NULL or `consecutive_failures` climbing means a mapper crash; restart the watcher after fixing. If `last_processed_at` is recent and `last_error` is NULL, the watcher is processing but the mapper silently swallows the row — file a bug. |

**Step 3 — resolve.** Once the underlying cause is fixed (or
classified as a non-issue), mark the rows resolved so they stop
counting toward the alert threshold:

```sql
UPDATE ht_reconcile_log
   SET resolved_at = now()
 WHERE resolved_at IS NULL
   AND table_name = 'customers'      -- the alerting table
   AND detected_at < now() - interval '5 minutes';
```

The 5-minute window leaves rows from the in-flight reconcile tick
alone in case the operator is still investigating.

### Tuning the threshold

The default of 50/hour/table is well above steady-state noise
(should be 0) and below the volume a genuine bulk catch-up would
produce. If a particular table has chatty steady-state drift you
genuinely don't care about (e.g. a clock-skew column that never
matters), prefer fixing the mapper or hash input over raising the
threshold — the threshold is a global blast-radius dial, not a
per-table mute. To tune:

```bash
# Production: set on the deploy host (subject to the .env-revert
# pitfall in Section 4a — promote to GH secrets if you want it to
# survive deploys).
LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD=100
```

Setting `0` or a negative value falls back to the default with a
warning log line; non-numeric values do the same.

### Retiring `record_success` / `record_error`

`scheduler/sync.rs::record_success` and `record_error` continue to
update `sync_status` rows for the operator dashboard. They are NOT
control-flow critical to the diff-only path but observability still
depends on them. Do not delete without first updating any dashboards
that read `sync_status`. The doc-comments in those functions carry
this contract.

---

## 10. Round-bill poll (cashier sessions, non-CT)

Shipped 2026-06-26. The `sync` binary polls the legacy `HT_Round_Bill`
cashier-round ledger into canonical `ht_shifts` so a round opened or
closed in iHOTEL is visible to our app's checkout/payment gate.
Coexistence (ADR 0002) means a receptionist may open/close the round in
EITHER app, so canonical state has to follow iHOTEL.

**Not Change Tracking.** This is a plain per-tick `SELECT`
(`sync_round_bills` in `bin/sync.rs`), deliberately OUTSIDE the CT-mapper
loop. `HT_Round_Bill` is **not** in `CT_ENABLED_TABLES`, carries no
watermark, and does not appear in `legacy_sync_status` or
`legacy_ct_state` — adding CT to it would be legacy DDL we don't own
(CLAUDE.md). Don't triage it via the CT observability in Section 8.

**When it runs.** Last in every `run_one_tick`, so only while
`LEGACY_SYNC_ENABLED=true` and at the `CT_POLL_INTERVAL_MS` cadence. It
honours `LEGACY_SYNC_SHADOW_MODE` (shadow = read-and-log "would upsert",
no canonical write) and ignores `LEGACY_SYNC_TABLE_ALLOWLIST` (not a CT
table). Each tick it SELECTs the one open row (`round_end IS NULL`) plus
any round touched in the last 2 days and UPSERTs into `ht_shifts` keyed
on `(shift_site_id, shift_no)`, where `shift_no` = the legacy
app-allocated `HT_Round_Bill.id`.

**Direction today.** Read-only iHOTEL → canonical only. The reverse path
(our app co-equally opening/closing `HT_Round_Bill`) is shipped DARK
behind `ROUND_WRITEBACK_ENABLED` (default false) and is NOT enabled —
see `docs/coexistence/ville-coequal-writes-plan.md`.

**Resilience.** A failure here never aborts the tick — every error path
logs at WARN and returns/continues, so the load-bearing CT mappers are
unaffected. Triage via the structured log `event_name`s:
`round_bill_sync_conn_fail`, `round_bill_sync_query_fail`,
`round_bill_sync_row_skip`, `round_bill_sync_upsert_fail`. A transient
`round_bill_sync_upsert_fail` during a round rollover self-heals next
tick — usually the one-open-per-site partial index catching up.

**Verify it's working.**

```sql
SELECT shift_site_id, shift_no, shift_legacy_round_id,
       shift_opened_at, shift_closed_at
  FROM ht_shifts
 WHERE shift_closed_at IS NULL
 ORDER BY shift_site_id;
```

Exactly one open (`shift_closed_at IS NULL`) row per site is the healthy
steady state — the same "≤1 open round per site" invariant iHOTEL holds,
enforced canonically by the partial unique index
`ht_shifts_one_open_per_site`.

---
