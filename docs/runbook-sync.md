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
| `SYNC_PER_TABLE_WATERMARK` | `false` = one shared `legacy_ct_state` row for all CT tables; `true` = one `legacy_ct_state_per_table` row each (R3). Per-table stops a wedge on one hot table gating every other table's advance. | `false` both sites | **Safe since #259** (the global row is now written as a min-of-per-table floor) but deliberately OFF — the wedge it prevents is loud, bounded (~2 days) and rare. Migration 078's reseed is done. Enable only if a wedge actually occurs: `docs/coexistence/RUNBOOK-reconcile-flag-flips.md` §2. |
| `WORKER_RECONCILE_ENABLED` | Runs the reconcile sweep inside the `sync` worker instead of the backend cron. This is how HF Ville gets a reconcile backstop at all (the backend cron only ever covered HF Hotel). | `false` (HF Hotel, uses backend cron) / `true` (HF Ville) | Leave as-is. HF Ville's is load-bearing — turning it off leaves that site with no backstop, which is exactly the 2026-06-28 gap. |
| `RECONCILE_FORCE_CONVERGE_ENABLED` | Auto-repairs **value** drift by re-running the mapper. **`customers` and `rooms` only** — bookings/checkins are multi-row aggregates and fall through untouched. | **`true` both sites** | Live. State lives in `docker-compose.yml` defaults, NOT GitHub variables (ADR 0004) — the GH vars were deleted. Read compose, and flip by editing the default there. |
| `RECONCILE_REINGEST_MISSING_PG_ENABLED` | Auto-repairs **`missing_pg`** rows (legacy row exists, canonical PG has none) by re-running the mapper for that key. PG-write-only; never writes legacy. Customers are processed before bookings so FK-defer ordering holds. Rows whose legacy row has *vanished* are left open for operator review. | **`true` both sites** (Ville 2026-07-27, HF Hotel 2026-07-28) | Live — this is the self-heal for a dropped legacy change whose CT delta aged past retention. State is a `docker-compose.yml` default (ADR 0004). See Section 9b. |
| `CT_POLL_INTERVAL_MS` | How often the watcher polls MSSQL CT. Lower = lower latency, higher load. | `1000` (1s) | Increase only if MSSQL load is a concern. |
| `LEGACY_SYNC_CT_KEEPALIVE_SECS` | `> 0` = a sibling task runs a read-only `CHANGE_TRACKING_CURRENT_VERSION()` on this cadence to keep the CT version machinery warm. The per-tick `SELECT 1` keeps the *connection* warm but not CT, so on a quiescent overnight iHOTEL the watchdog's first CT probe after a lull can answer slower than its 30s budget → the benign `:information_source: CT watermark idle — probe timed out`. Keeping CT hot makes that probe return fast (and classify a *real* backlog correctly instead of masking it as a timeout). | `45` (HF Hotel) / `0` (HF Ville, off) | Enabled at 45s for HF Hotel (2026-06-30). Ville wired but off — flip `HFVILLE_LEGACY_SYNC_CT_KEEPALIVE_SECS=45` in `.env` to enable (no code change). 45 sits under the 60s pool idle_timeout. Read-only; no writeback; safe to enable without reception coordination. |
| `SYNC_TEST_SKIP_MSSQL_PROBE` | Test-only. `true` = skip the bb8-tiberius probe in `tests/test_sync_phase54_integration.rs::mssql_stub`. | unset | Set when running pure-PG tests without legacy MSSQL access (saves 30s per process). |
| `DATABASE_URL` | PG DSN for the watcher's writes. | (set in compose) | Standard PG creds — same as backend. |
| `DB_SERVER` / `DB_NAME` / `DB_USER` / `DB_PASSWORD` | Legacy MSSQL connection. | (set in compose) | Match the legacy app. |
| `SLACK_WEBHOOK_URL` | Where schema drift / retention overflow / cold-replay refusal alerts go. | (set in compose) | Standard ops Slack. |
| `RUST_LOG` | Tracing filter. | `hotel_backend=info,sync=info` | Add `,debug` to tighten when investigating. |

### 2a. Alert-tuning knobs (2026-07-28 hardening wave)

These were compiled-in constants before the 2026-07-28 alert-surface calibration (`1b06849`) — tuning the one alert the channel actually received required a full backend deploy. Now env-overridable; **all defaults are unchanged** from the pre-calibration constants. Every knob in this subsection follows the same resolution chain: `<VAR>_<SITE_ID_UPPER>` (e.g. `_HFHOTEL` / `_HFVILLE`) first, then the bare global `<VAR>`, then the compiled-in default. Non-numeric or non-positive values are ignored with a warning log and fall back one tier rather than to zero.

| Var | Meaning | Default | When to flip |
|---|---|---|---|
| `LEVEL_DRIFT_STALE_INTERVAL_HOURS` (+ per-site) | §9b unconverged-rows digest: a `ht_reconcile_log` row unresolved longer than this is "stale" and eligible for the `:warning:` digest. | `4` | Rarely. Lower only to shrink detection latency for a known-bad table during an active investigation. |
| `LEVEL_DRIFT_COOLDOWN_HOURS` (+ per-site) | §9b: minimum gap between `:warning:` digests for the same `(site, table)`. Burns only on a confirmed delivery (Slack `Sent` or `LoggedOnly`, never on a failed POST) — a webhook outage no longer silences the table for the full window. | `24` | Shorten if operators want faster re-nagging on a table under active repair; the paired all-clear already re-arms it early on recovery. |
| `LEVEL_DRIFT_ESCALATE_HOURS` (+ per-site) | §9b: oldest-row age at which a table moves from the `:warning:` "unconverged" tier to the distinct `:bangbang:` "STUCK — will not self-heal" tier, under its own `escalated:<table>` cooldown (same duration as `LEVEL_DRIFT_COOLDOWN_HOURS`) so the transition announces on the next tick instead of waiting out an already-open primary window. Must exceed `LEVEL_DRIFT_STALE_INTERVAL_HOURS`; if not, silently clamped to `stale_hours + 1` with a warning log line. A table lands in exactly one tier per tick. | `72` | Rarely — by 72h the auto-resolve sweep has had ~288 chances to close the row and is not going to. |
| `LEGACY_RECONCILE_BURST_COOLDOWN_HOURS` (+ per-site) | §9 edge-triggered burst alert (`>50 rows/hr/table`, `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`): minimum gap between repeat bursts for the same `(site, table)`. Before this existed the burst alert had NO cooldown and could re-fire on every 15-min tick. | `1` (hour, matches the alert's own rolling window) | Raise only if a known bulk catch-up is producing repeat bursts inside the hour; the threshold itself is a blast-radius dial, not a target — 21 days of prod data peaked at 33/hr, well under 50. |
| `LEGACY_SYNC_CT_LAG_PAGER_ENABLED` | Kill switch for the CT-lag pager (§3 "CT watcher LAG sustained"). Any value other than the literal string `false` counts as enabled. | `true` | Flip to `false` only if the pager proves noisy in production. The underlying per-tick log observation (`[Sync] CT watcher lag detected`, ~170×/day at WARN pre-calibration) keeps running unconditionally either way — this only gates the Slack page. |
| `LEGACY_SYNC_CT_LAG_PERSIST_SECS` | How long the version-lag breach must hold CONTINUOUSLY (any healthy observation resets it) before the pager fires. Level-triggered on purpose — a single tick over threshold is routine (one iHOTEL batch save, a slow poll cycle) and drains within seconds. | `1800` (30 min) | Leave alone. 30 min still leaves ~47h of the 2-day CT retention window to act in. |
| `LEGACY_SYNC_CT_LAG_PROBE_INTERVAL_SECS` | Minimum spacing between dedicated CT-current probes issued solely for the lag check (probes the stall/recovery watchdog branches already took are reused for free). A cached probe older than 2× this interval cannot drive a page or all-clear decision in either direction — uncertainty holds the current state. | `300` (5 min) | Only if the extra read-only scalar probes on the shared legacy server are a concern. |
| `LEGACY_CT_LAG_WARN_VERSIONS` (+ per-site) | Version-lag threshold shared by BOTH the log-only per-tick observation (`scheduler::sync::check_ct_watcher_lag`) and the `bin/sync` pager's decision — one resolver contract, locked by a test so the two can never disagree. Deliberately NOT prefixed `LEGACY_SYNC_` (it pre-dates the pager). Pageable when `version_lag > threshold`. | `100` | Lower only with evidence that steady-state lag regularly exceeds 100 CT versions without being a real backlog. |
| `LEGACY_CT_LAG_WARN_SECONDS` (+ per-site) | Poll-age threshold on the same shared struct. Logged for context on every page but **never pages on its own** — see §3, "why only the VERSION arm pages": a healthy caught-up watcher on a quiet legacy never advances `last_polled_at`, so poll age grows unbounded every quiet night by design. | `300` | Changing it does not change paging behaviour, only the informational log-only observation. |
| `LEGACY_SYNC_BOOT_ALERT_COOLDOWN_MINS` | Per-reason (`boot_refusal:<slug>` — live-bootstrap, schema-fingerprint, cold-replay, retention-overflow, CT-not-enabled) durable dedup window for the five refuse-to-start pages. Sized to squash a `restart: on-failure:5` burst (~6-10 min) with margin. Fails OPEN on a PG error — see §3. | `30` (min) | Widen if a flapping deploy is still producing more than one page per reason inside 30 min. A DIFFERENT reason always pages immediately regardless of this window. |
| `WRITEBACK_STARTUP_PROBE_ATTEMPTS` (`bin/writeback`, not `bin/sync`) | Attempt budget shared by all three writeback startup probes (schema fingerprint, collation, idempotency-ledger) before refusing to start; 6/12/18s backoff between attempts (~36s total). Old name `WRITEBACK_FINGERPRINT_ATTEMPTS` still honoured as a fallback (new name wins if both are set). | `4` | Raise if a slow WireGuard re-handshake or `newdb`/legacy restart blip regularly exceeds the ~36s budget and trips a false "unreachable" refusal. |

---

### 2b. Phase 6-A..D reconcile-arm flags (2026-07-28 coverage expansion)

Four new arms/probes, each dark behind its own flag, each `RECONCILE_REINGEST_MISSING_PG_ENABLED`-independent (none of these four self-heal — every finding needs operator action). Detection-only: none writes canonical state, only `ht_reconcile_log` plus (for the two reconcile arms) a small ack cache. Per-site override follows the existing prefix convention (`HFVILLE_<VAR>`, same as `LEGACY_SYNC_CT_KEEPALIVE_SECS` in §2) — the HF Hotel `backend`/`sync` services read the bare var, `sync-hfville` reads the `HFVILLE_`-prefixed one.

| Var | Meaning | Default | When to flip |
|---|---|---|---|
| `RECONCILE_PAYMENTS_ARM_ENABLED` (`HFVILLE_RECONCILE_PAYMENTS_ARM_ENABLED` for Ville) | Phase 6-A (`3c45ba6`): reconciles legacy `HT_Receipt_H` against canonical `ht_payments`, keyed on `Receipt_no`. Ack cache `ht_receipts_legacy` (migration 080). Floored at the mirror's own canonical-era start (`MIN(ht_payments.pay_date)`, derived, never configured) so the ~20k pre-CT legacy receipts at HF Hotel are never reported missing. | `false` both sites | Ville first, 48h soak, then HF Hotel — see rollout note below. Ville's canary is intentionally WEAK here (105 legacy receipts vs 4 canonical, nothing like HF Hotel's 21.5k-receipt history); the era floor, not the soak, is what bounds that. |
| `RECONCILE_GUEST_REGISTRY_ARM_ENABLED` (`HFVILLE_RECONCILE_GUEST_REGISTRY_ARM_ENABLED` for Ville) | Phase 6-B (`6fddd6b`): reconciles legacy `HT_CheckIn_Other_People` against canonical `ht_guest_registry`. Unit is the whole companion SET per folio (`Cin_no`), not a row — iHOTEL edits companions by delete-then-reinsert, so a row-level compare would report two divergent rows on every correctly-applied edit. Ack cache `ht_guest_registry_legacy` (migration 081). Floored at the mirror's coverage era plus parent-checkin existence. | `false` both sites | Ville first, 48h soak, then HF Hotel. Unlike the payments arm, Ville's canary here is REPRESENTATIVE (same shape/era boundary as HF Hotel — ~29 Ville first-enable finds predicted ~12 at HF Hotel). A TM.30 companion under-count is the exact class this arm exists to surface. |
| `RECONCILE_MIRROR_PROBE_ENABLED` (`HFVILLE_RECONCILE_MIRROR_PROBE_ENABLED` for Ville) | Phase 6-C (`1259ca4`): generic aggregate probe (COUNT/MAX/SUM, one UNION-ALL batch per side per tick, migration 082's `sync_status_mirror_probe`) over the 8 CT-mirrored `legacy_mirror.*` tables plus canonical `ht_room_calendar`; a per-key diff runs only when the aggregate disagrees. `ht_room_calendar` is OBSERVE-ONLY — the gap is real (HF Hotel 1507 in-era legacy rows vs 1298 canonical) but structural (iHOTEL's MAX+1 id-rebind permanently NULLs `rcal_legacy_id` with nothing to restore it), so it's logged with both counts and never written to `ht_reconcile_log` — no sweep or self-heal could close a row there today. Floored at `MIN(mirror pk)` per table. | `false` both sites | Ville first, 48h soak, then HF Hotel. Ville is REPRESENTATIVE for all 9 probes (exactly converged on the 8 mirror tables once floored, same as HF Hotel; the calendar gap ratio is worse at Ville, so a clean soak still predicts HF Hotel). |
| `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED` (`HFVILLE_RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED` for Ville) | Phase 6-D (`7a928fd`): reconciles legacy `HT_CheckIn_Pay` against canonical `ht_payment_ledger` — the table `round_report` reads. Unit is the folio (forced by the mapper's delete+reinsert, same shape as 6-B). Tenders deduped per receipt before summing, matching `round_report`'s own dedupe, so the reported legacy total isn't inflated ~3x by iHOTEL's per-line tender replication; `ledger_amount` itself is summed raw (itemized by design). Floored at `MIN(ledger_legacy_id)` applied via `HAVING`, not `WHERE`, so a folio straddling the era boundary isn't compared on a partial line set. | `false` both sites | Ville first, 48h soak, then HF Hotel. Ville is EXACTLY converged as of 2026-07-28 (a clean baseline, not proof the detector fires) — HF Hotel is where the 19 known `missing_pg` folios live, remediable via the `backfill_payment_ledger` bin. |

**Rollout note (all four Phase 6-A..D flags).** Enable HF Ville first, soak 48h, then HF Hotel — the same site-first sequencing as every prior reconcile arm (`RECONCILE_REINGEST_MISSING_PG_ENABLED`, `SYNC_PER_TABLE_WATERMARK`). **A first-enable backlog flood is EXPECTED, not a bug**: each arm floors its scan at its own mirror's derived canonical-era start specifically so it doesn't report pre-CT legacy history as missing — but even floored, the very first tick after enabling still surfaces every genuine gap accumulated since that era in one shot (live evidence: the 6-D probe opened exactly 19 `missing_pg` rows at HF Hotel on its first pass). Pre-set `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE>` (§9) above the expected one-time count before flipping, or flip inside an announced window, so the burst doesn't trip the §9 rate alert or read as a live incident. The `>72h` escalation tier (`LEVEL_DRIFT_ESCALATE_HOURS`, §9b) WILL fire on any first-enable finds still open after 3 days — expected for a backlog that needs operator action to close (none of these four arms self-heal), not a bug. Triage per §9's classification table; don't raise the threshold to silence it.

---

## 3. Slack alert meanings

The watcher surfaces three categories of alerts to Slack. All are
prefixed so they're triagable in one glance.

| Alert | Meaning | Operator action |
|---|---|---|
| `:warning: CT watcher REFUSED TO START` (schema fingerprint) | Legacy MSSQL columns drifted from the captured baseline. The watcher refuses to project against an unknown shape. | Run `./scripts/writeback-fingerprint.sh` and follow the README to update the baseline before restarting. Same workflow as the writeback worker — fingerprint is shared. |
| `:no_entry: CT watcher REFUSED TO START` (cold replay) | `last_seen_version=0` and `LEGACY_SYNC_ALLOW_COLD_REPLAY != true`. | Run the bootstrap procedure (Section 1). |
| `:no_entry: CT watcher REFUSED TO START` (retention overflow) | At startup, `MIN_VALID_VERSION` is higher than the watermark on at least one CT-tracked table. CT history we'd need to catch up has aged out. The pre-flight check refuses rather than silently skipping rows. | Run the bootstrap procedure (Section 1) — `--bootstrap` re-snapshots canonical PG and stamps the watermark to `CHANGE_TRACKING_CURRENT_VERSION()`. After bootstrap, restart the watcher. See Section 4b for the shadow-mode trap that triggers this. |
| `:no_entry: CT watcher REFUSED TO START — Change Tracking not enabled` | A `CT_ENABLED_TABLES` entry has no live CT subscription on the legacy server — almost always a `migrations/legacy-mssql/` prerequisite that didn't get applied before this binary shipped (the 2026-06-24 incident this guard closed). | Check `scripts/migrate-legacy-mssql.sh` output for a failed/skipped migration, apply the matching CT-enable DDL, restart. `LEGACY_SYNC_ALLOW_CT_GAP=true` runs with those tables intentionally unsynced (per-tick errors resume) — an explicit opt-in, not a fix. |
| `:no_entry: Bootstrap REFUSED — live deployment` | Operator ran `--bootstrap` while `LEGACY_SYNC_ENABLED=true`. The snapshot would race the live CT watcher's UPSERTs and clobber `mirror_source='ct'` rows. | Stop the watcher first (set `LEGACY_SYNC_ENABLED=false` and redeploy), then run `--bootstrap`, then re-enable. Set `LEGACY_SYNC_ALLOW_LIVE_BOOTSTRAP=true` ONLY if you accept the race window. |
| `:rotating_light: CT retention overflow` | A specific table's `MIN_VALID_VERSION` is higher than the watermark — CT history we needed has aged out (default retention 2 days). | Re-bootstrap (Section 1). The reconcile inside `--bootstrap` will catch us up via the canonical UPSERT path. |
| `:warning: Reconcile rows unconverged >4h` (`level_drift_alert`) | At least one `ht_reconcile_log` row for that table has been unresolved for over `LEVEL_DRIFT_STALE_INTERVAL_HOURS` (default 4h). Level-triggered, per-table cooldown (`LEVEL_DRIFT_COOLDOWN_HOURS`, default 24h) — catches a *single* stuck row, which the §9 rate alert structurally cannot. | Triage per Section 9b. Most actionable shape is `divergence_kind = missing_pg` with a still-live legacy row: a dropped CT event, healed by the re-ingest arm. Pairs with a `:white_check_mark:` all-clear that also clears the cooldown. |
| `:bangbang: Reconcile rows STUCK >72h — will not self-heal` (escalated tier, 2026-07-28) | The SAME `ht_reconcile_log` row(s) as the `:warning:` digest above, but the oldest one has now passed `LEVEL_DRIFT_ESCALATE_HOURS` (default 72h) — three consecutive daily digests ignored. A distinct title/tone (day-1 and day-16 of the 2026-07-11 16-day loss used to send byte-identical text on the primary alert's 24h rhythm, which is what trains an operator to dismiss it) and its own `escalated:<table>` cooldown key so the transition to STUCK announces on the NEXT tick rather than waiting out a primary window that may have refreshed hours earlier. A table lands in exactly one tier per tick — escalated tables do NOT also get the `:warning:` digest. | The fix is re-ingest or `--bootstrap`, not patience — the auto-resolve sweep has had ~288 chances by 72h and is not going to close it. See Section 9b. |
| `:warning: CT watcher LAG sustained Nmin` (2026-07-28, `bin/sync` watchdog) | The per-tick `[Sync] CT watcher lag detected` observation (fires ~170×/day at WARN, log-only, unchanged) now has a paging path. Pages ONLY on the **VERSION arm** (`legacy_ct_state.last_seen_version` vs MSSQL `CHANGE_TRACKING_CURRENT_VERSION()`, threshold `LEGACY_CT_LAG_WARN_VERSIONS`, default 100) after the breach holds CONTINUOUSLY for `LEGACY_SYNC_CT_LAG_PERSIST_SECS` (default 30 min) — a single tick over threshold is routine and self-drains. The **poll-age arm never pages**: `legacy_ct_state.last_polled_at` is written only by `watermark::advance`, so a healthy watcher caught up on a quiet legacy never touches the row and poll age grows unbounded every quiet night by design (shadow mode freezes it outright) — paging on it would manufacture nightly noise. Uses a durable cooldown slot (`ct_watcher_lag:global`, 24h) that the paired `:white_check_mark: CT watcher lag RECOVERED` all-clear releases, so a genuine recurrence pages immediately. Deliberately **suppressed while a watermark-STUCK alert is open** — that page already owns the operator's attention for the frozen-watermark shape; this one covers the different shape of a watermark that is still moving but steadily falling behind (the 2026-05-18 lost-UPDATE class, which this detector exists to catch). Kill switch: `LEGACY_SYNC_CT_LAG_PAGER_ENABLED=false`. | Check `legacy_sync_status.last_error` for a table stuck in a retry loop, then `/api/new/sync/status`. Grep worker logs for `[Sync] CT watcher lag detected` for the per-tick history. MSSQL CT retention is 2 days — anything still unread when a version ages out is lost silently, so treat this as urgent, not informational. |
| `:no_entry: ... REFUSED TO START` / `:no_entry: Bootstrap REFUSED` — dedup mechanic (2026-07-28) | All five refuse-to-start pages above (live-bootstrap, schema-fingerprint, cold-replay, retention-overflow, CT-not-enabled) used to be bare unthrottled POSTs. With Compose's `restart: on-failure:5` and a 60s guard sleep before each exit, one bad deploy produced up to 6 identical pages per service × 2 sites, then **silence** once the restart cap was exhausted — a storm that reads as a flood followed by a false "recovered". Now deduped per reason (`boot_refusal:<slug>`, one durable cooldown slot each — a fingerprint fix that then trips the CT gate still pages immediately, different key). Dedup fails **OPEN** here (`ClaimFallback::Send`), the opposite polarity from the retention-overflow page: these guards' failure mode is silence about a process that will not run, so a duplicate page beats a missing one. A "gave up after 5 restarts" alert is structurally impossible from inside the dying process, so every refusal body now states the restart-cap contract explicitly: *silence after this page does NOT mean recovered* — confirm with `docker compose ps` / container logs. Window: `LEGACY_SYNC_BOOT_ALERT_COOLDOWN_MINS` (default 30 min). | Don't read a quiet channel as "the deploy healed itself." Check `docker compose ps` for the container's actual state before assuming recovery. An external container-liveness monitor is the real backstop for the exhausted-restart case (tracked separately — not shipped). |
| `:warning: Writeback PG NOTIFY listener UNHEALTHY` / `:white_check_mark: ... RECOVERED` (`bin/writeback`, demoted 2026-07-28) | The listener supervisor reconnects on any `recv()` error; below `LISTENER_SUSTAINED_OUTAGE_SECS` (10 min, const, not env-overridable) a reconnect loop doing its job is a log line only — the worker keeps draining the queue via its 30s poll fallback throughout, so nothing is lost or stuck, only NOTIFY latency degrades from sub-second to ≤30s. Past 10 min continuous the page fires (re-page floor 30 min inside one outage). Fixed alongside the demotion: `consecutive_failures` used to never reset on a healthy session, so ten unrelated failures spread across days could sum past the old raw-count threshold and page on noise. A session now counts as HEALTHY (and clears the outage clock) only once the subscription has held for `LISTENER_HEALTHY_SESSION_SECS` (30s) — a connect-then-instantly-drop flap keeps accumulating toward the sustained threshold instead of silently resetting it. Pairs with an all-clear once a healthy session closes a paged outage. | Not urgent by itself — confirm the queue is still draining (`pending` count via the janitor) before treating this as anything beyond a latency degradation. Inspect `pg_stat_activity` for the LISTEN connection / role privileges / `max_connections` exhaustion per the alert body. |
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
| `pg_hash IS NULL` on most rows. | Bulk PG-miss — canonical rows never landed. Most often: CT retention overflow (Section 4b) or watcher offline window. | Cross-check `legacy_sync_status.last_processed_at` — stale on the matching MSSQL table? Run `--bootstrap` (Section 1). **Prefer the re-ingest arm over the blanket UPDATE below**: with `RECONCILE_REINGEST_MISSING_PG_ENABLED=true` the sweep re-runs the mapper per key and closes each row only once canonical actually converges. The manual fallback — `UPDATE ht_reconcile_log SET resolved_at = now() WHERE resolved_at IS NULL AND table_name = 'customers';` — closes rows *whether or not* canonical landed, so it hides any key the bootstrap missed. Use it only when the arm is off. |
| `pg_hash` and `mssql_hash` both populated, both differ on every row. | Mapper-projection bug — the CT watcher is writing canonical state but with a different shape than reconcile expects. | Pick one row, compare `mssql_row_json` to the canonical PG row, identify the diverging column. Fix the mapper in `src/sync/mappers/`, ship via CI. Mark rows resolved AFTER the next reconcile tick goes clean. |
| Single isolated row, no pattern across `table_name`. | One-off CT overflow, a dropped CT event, or a hand-edit on the legacy DB after a watcher restart. | If `pg_hash IS NULL` and the legacy row still exists, this is a **dropped CT event** — see Section 9b, the re-ingest arm heals it with no legacy write. Only if that arm is off: re-fire the CT mapper for that PK by writing a no-op UPDATE on the source MSSQL row, OR resolve by hand if the canonical PG state matches business intent. |
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

## 9b. The >4h unconverged alert (`level_drift_alert`)

Distinct from the §9 *rate* alert (50/hour/table). This one is
**level-triggered**: any table with at least one `ht_reconcile_log` row
still unresolved after **4 hours** fires it, on a **24h per-table
cooldown** (`ht_level_drift_alert_cooldowns`, migration 053). It exists
to catch the case a rate alert structurally cannot — a *single* stuck
row. Message shape:

```
[site=hfville] :warning: Reconcile rows unconverged >4h :warning:
• `bookings`: 3 unresolved row(s)
• `customers`: 1 unresolved row(s)
```

It now pairs with a `:white_check_mark:` all-clear when the table's stale
rows clear; the all-clear also drops the cooldown row, so a recurrence
alerts immediately rather than being swallowed by a stale 24h window.

### Triage order

1. **Age.** Under one sweep interval (15 min) → noise, the sweep closes
   it. Days → durable, keep going.
2. **Shape.** Pull the rows:
   ```sql
   SELECT id, table_name, legacy_pk, detected_at, now()-detected_at AS age,
          divergence_kind, legacy_row_count, pg_row_count
     FROM ht_reconcile_log
    WHERE resolved_at IS NULL ORDER BY detected_at;
   ```
   `divergence_kind = missing_pg` with `pg_row_count = 0` and a *live*
   legacy row is a **dropped CT event** — the mapper never ran for that
   key. Do NOT go hunting for a mapper bug.
3. **Confirm it's a drop, not an outage.** Check whether keys either side
   landed (`legacy_cust_no IN (...)`, `legacy_book_id IN (...)`). Sibling
   keys present ⇒ per-key miss, not a window. Then read the watcher's
   advance log for the detection window and look for **another table**
   moving the watermark past it:
   ```
   docker logs new-hotel-production-sync-hfville-1 \
     --since <local-ts> --until <local-ts> | grep -a 'Advanced CT watermark'
   ```
   Remember the host renders `--since/--until` in **Thai local time** while
   log lines are stamped **UTC**.
4. **Heal.** With `RECONCILE_REINGEST_MISSING_PG_ENABLED=true` the sweep
   re-runs the mapper for the key and closes the row once the hashes
   converge — no legacy write, no manual SQL. If the legacy row has
   vanished, the arm deliberately leaves the row open: that is a genuine
   anomaly needing a human.

### Escalation

Only page on rows that survive multiple sweep cycles. A row that clears
inside one tick is the system working. If a `missing_pg` row is older
than CT's 2-day retention, the watcher can **never** redeliver it — a
re-ingest (or `--bootstrap`) is the only path, so re-firing the alert
without acting just burns the cooldown.

Worked example: 2026-07-27 in `docs/coexistence/sync-incident-log.md`.

---

## 9c. Coverage boundary — what Phase 6-A..D do NOT reconcile (2026-07-28)

The four §2b arms close the payments/companion/mirror-table/tender-ledger gaps,
but two adjacent surfaces are still outside the reconcile system entirely —
worth stating explicitly so an operator doesn't assume they're covered.

* **Coupons (`HT_Cupon`).** Verified unused in both live legacy DBs — dormant
  since 2025-07 (`docs/adr/0003-ihotel-anchored-ux.md`, CONTEXT.md's "Verified
  unused" list). No dedicated Phase 6 arm exists for it, but it isn't a blind
  spot: `HT_Cupon` is one of the 8 `legacy_mirror.*` tables in the §2b Phase
  6-C mirror probe's set, so if the dormant feature ever came back to life, a
  coupon batch would still be caught by that probe with no code change.
* **Rate tiers and the other `legacy_mirror` dimension tables**
  (`HT_ContinueTime`, `HT_Rooms_Price`/`HT_Order_Up`/`HT_Order_Down`, and
  canonical `ht_rate_tiers`). None of these are in scope for any Phase 6-A..D
  arm or the Phase 6-C mirror probe. They're wholesale-reloaded — full
  DELETE+INSERT for the `legacy_mirror.*` dimensions, UPSERT-only for
  canonical `ht_rate_tiers` — every 15-minute tick by
  `reload_mirror_dimensions` (`scheduler/mirror.rs`), which is correct by
  construction on the reload path itself rather than reconciled after the
  fact — the same reload cadence that also carries canonical `ht_products`
  (Phase 2b, see `docs/coexistence/sync-incident-log.md`'s "Armed
  product-stall" entry). The one known gap in that design: the
  `ht_rate_tiers` UPSERT never prunes a canonical row whose legacy
  `(room_type, cust_type)` key has since been deleted in iHOTEL — it keeps
  serving its last-known price forever. Tracked as GitHub issue **#270**
  ("sync: HT_Rooms_Price mirror never prunes a legacy-deleted rate tier —
  stale price served forever"), not fixed here.

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
