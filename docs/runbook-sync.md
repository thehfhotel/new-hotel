# Runbook — CT Watcher (Phase 5.5 production cutover)

Operator-facing reference for `bin/sync` (the Change Tracking watcher).
Companion to `docs/architecture.md` §3.6d, §3.7, §11.

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
| `LEGACY_SYNC_RECONCILE_MODE` | Mode for the demoted `scheduler::sync::run_sync` job. `diff_only` = log drift to `ht_reconcile_log`; `upsert` = legacy 5-min-style UPSERT into `ht_*_legacy`. | `diff_only` | Flip to `upsert` ONLY if the CT watcher is operationally disabled and you need the legacy safety net to keep canonical state in sync. |
| `CT_POLL_INTERVAL_MS` | How often the watcher polls MSSQL CT. Lower = lower latency, higher load. | `1000` (1s) | Increase only if MSSQL load is a concern. |
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
| `:rotating_light: CT retention overflow` | A specific table's `MIN_VALID_VERSION` is higher than the watermark — CT history we needed has aged out (default retention 2 days). | Re-bootstrap (Section 1). The reconcile inside `--bootstrap` will catch us up via the canonical UPSERT path. |
| Mapper consecutive-failure threshold (future) | Per-table `legacy_sync_status.consecutive_failures` exceeds N. | Inspect `legacy_sync_status.last_error` for that table; check mapper logs for the failing CT row payload. |

---

## 4. Cutover procedure

The supported sequence from "code deployed, default-disabled" to "live
production CT watcher". Each step gates on the prior step's success.

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
4. **Go live.** Set `LEGACY_SYNC_SHADOW_MODE=false`, recreate the
   container. Now CT changes flow into canonical PG state and the
   event bus.
5. **24h live soak.** Run the receptionist test plan (Section 7).
   Verify `event_log` rows with `source_kind='legacy_app'` accumulate.
   Verify `ht_reconcile_log` stays empty (no drift between CT watcher
   and the demoted reconcile).
6. **Demote `scheduler::sync`.** Already demoted automatically by
   v2.45.0 (`LEGACY_SYNC_RECONCILE_MODE` defaults to `diff_only`).
   Confirm by checking the next reconcile tick logs: should say
   `[Sync] Customers (DiffOnly): …`.

---

## 5. Rollback procedure

If something goes wrong during cutover, the reverse sequence:

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
