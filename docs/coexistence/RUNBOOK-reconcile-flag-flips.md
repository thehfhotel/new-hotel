# Runbook — reconcile flag flips

Covers the two self-heal / watermark flags on the reconcile sweep. **One is a
live procedure; the other is BLOCKED and must not be turned on.** Read §2 before
touching per-table watermarks.

Background: `docs/coexistence/sync-incident-log.md` → 2026-07-27.
Alert triage: `docs/runbook-sync.md` §9b.
Flag mechanism: `docs/adr/0004-flag-state-in-compose.md`.

---

## 0. How these flags are flipped

**Not** via `gh variable set`, and **not** via an empty commit. Both fail
silently:

- GitHub repo variables no longer feed these keys (ADR 0004). Setting one has
  no effect.
- An empty commit changes no paths, so the `changes` paths-filter job reports
  nothing, `build-*` skip, and the `deploy` job's final condition —
  `build-frontend == 'success' || build-backend == 'success' || (changes.outputs.deploy == 'true' && …)` —
  is false on every disjunct. **The run goes green without deploying anything.**
  This bit us on 2026-07-27: a flag was "flipped", CI was green, and the
  container never restarted.

The correct procedure is to **edit the default in `docker-compose.yml` and
push**. `docker-compose.yml` is in the workflow's `deploy` paths filter, so the
edit is its own deploy trigger.

```bash
# 1. Edit the flag's default in docker-compose.yml, e.g.
#      - RECONCILE_REINGEST_MISSING_PG_ENABLED=${HFVILLE_..._ENABLED:-false}
#    →  - RECONCILE_REINGEST_MISSING_PG_ENABLED=${HFVILLE_..._ENABLED:-true}
git commit -am "chore(sync): <what and why>"
git push origin master

# 2. Confirm the deploy job actually RAN (not skipped)
gh run list --workflow=docker-build.yml --limit 1 --json databaseId --jq '.[0].databaseId' \
  | xargs -I{} gh run view {} --json jobs --jq '.jobs[] | "\(.conclusion // .status)\t\(.name)"'

# 3. Confirm the container restarted AND carries the new value
ssh evergreen "docker ps --format '{{.Names}}\t{{.Status}}' | grep sync-hfville"
ssh evergreen "docker inspect new-hotel-production-sync-hfville-1 \
  --format '{{range .Config.Env}}{{println .}}{{end}}' | grep -iE 'REINGEST|FORCE_CONVERGE|PER_TABLE'"
```

**Step 3 is not optional.** A green run is not proof a flag is live; "Up N
hours" on the container means it never restarted and the old value still holds.

---

## 1. Flip A — `missing_pg` re-ingest self-heal (LIVE PROCEDURE)

Repairs reconcile-log rows where the legacy row exists but canonical has none —
a **dropped legacy change** (see `CONTEXT.md`). Re-runs the normal mapper for
that key. PG-write-only; legacy MSSQL is read, never written.

Flag: `RECONCILE_REINGEST_MISSING_PG_ENABLED` (`HFVILLE_` sibling for Ville).
Ville canaries first.

### Preconditions

```bash
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelville -At -F'|' -c \
  \"SELECT id, table_name, legacy_pk, now()-detected_at AS age, divergence_kind,
            legacy_row_count, pg_row_count
      FROM ht_reconcile_log WHERE resolved_at IS NULL ORDER BY detected_at;\""
```

Rows must be older than `FORCE_CONVERGE_MIN_AGE_SECS` (1h, a compile-time const
in `scheduler/sync.rs` — not an env var) to be eligible.

### Verify after the deploy

The sweep runs every 15 minutes (`WORKER_RECONCILE_INTERVAL_SECS`).

```bash
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelville -At -F'|' -c \
  \"SELECT id, table_name, legacy_pk, resolved_at FROM ht_reconcile_log
     WHERE resolved_at IS NULL;\""
```

Expect the rows to close, the canonical records to appear, and a
`:white_check_mark:` all-clear in Slack naming the site and tables.

### Reading the outcome from logs

`resolved_at` is the only status column, so outcome detail lives in structured
logs. Greppable:

| log line | meaning |
|---|---|
| `…marking resolved` | healed |
| `Re-ingest (missing_pg): legacy row absent at re-fetch` | legacy row deleted since detection — **genuine anomaly**, deliberately left open for a human |
| `…canonical re-ingested but hashes still unconverged` | applied but did not converge — investigate the mapper projection |

### Ordering invariant

Customers are swept before bookings so the FK-defer path holds
(`apply_booking_aggregate` needs the customer present). If a customer heals but
its bookings stay open past two sweeps, that guarantee has regressed.

### Rollback

Edit the default back to `:-false`, push. Already-healed rows stay healed — the
canonical write is correct and idempotent; only further healing stops.

---

## 2. Flip B — per-table CT watermarks (**BLOCKED — DO NOT ENABLE**)

Flag: `SYNC_PER_TABLE_WATERMARK`. **Leave at `false` on both sites.**

Turning this on today causes a permanent alert storm and a rollback cliff. Two
prerequisites are missing:

1. **The global row freezes.** `bin/sync.rs` gates the once-per-tick global
   advance on `!per_table_watermark`, so under per-table mode
   `legacy_ct_state.last_seen_version` / `last_polled_at` are never written.
2. **The watchdog is not per-table aware.** `run_watermark_watchdog` has zero
   per-table handling, and `read_ct_state` reads only
   `SELECT last_seen_version, last_polled_at FROM legacy_ct_state WHERE id = 1`.

Together: the watchdog reads a frozen row, probes
`CHANGE_TRACKING_CURRENT_VERSION()`, sees `ct_current > watermark`, and fires
`:rotating_light: CT watermark STUCK` every 30 minutes **forever** — the
recovery condition compares against that same frozen row, so no all-clear can
ever fire. After ~2 days the frozen value falls below
`CHANGE_TRACKING_MIN_VALID_VERSION`, at which point the startup gate legitimately
refuses to start and recovery needs `--bootstrap`.

### What IS already done

Migration 078's reseed — the expensive prerequisite — is complete and verified.
All 19 `legacy_ct_state_per_table` rows on both sites were force-reset from the
global watermark (hfville 9060 → 37989; hotelnew 17209 → 67695), clearing the
retention-overflow storm stale rows would otherwise cause. That work does not
expire; it simply isn't sufficient alone.

### Why the other two are not being fixed now (decision, 2026-07-27)

Per-table watermarks mitigate one wedged table gating the others. After the
2026-07-27 fix that scenario is **loud** (`CT watermark STUCK` within ~30 min),
**bounded** (~2 days of CT retention before it turns destructive), and **rare**
(one instance in ~2.5 months — a 74-minute `HT_Book_H` row-lock on 2026-05-14,
comfortably inside the budget).

Before that fix, a wedged table caused *silent, unbounded* data loss. The change
converted silent loss into a detectable, recoverable stall. Building the two
prerequisites means adding fresh code to the most correctness-critical path in
the system to mitigate a risk that just became materially safer. The owner
accepted the residual exposure.

Revisit if a wedge actually occurs.

### Guard

A startup guard refuses to start when `SYNC_PER_TABLE_WATERMARK=true`, naming
these prerequisites — joining the existing refuse-to-start guards (cold replay,
retention overflow, live bootstrap, CT-enablement gap). The flag cannot be
turned on by accident.

---

## 3. HF Hotel

HF Hotel currently holds **zero** unresolved `ht_reconcile_log` rows, so enabling
the re-ingest arm there should be a no-op — which makes it a clean test of the
flag itself rather than of the heal. Do it only after HF Ville has soaked.

Edit the non-prefixed default on the **`backend`** service. HF Hotel's reconcile
runs in `backend`, not in the `sync` worker (`WORKER_RECONCILE_ENABLED` is false
there, so the `sync` copy of the key is inert):

```yaml
- RECONCILE_REINGEST_MISSING_PG_ENABLED=${RECONCILE_REINGEST_MISSING_PG_ENABLED:-true}
```
