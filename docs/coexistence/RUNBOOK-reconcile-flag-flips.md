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

### Redeploying with no file change

A flag flip never needs this — the compose edit is its own trigger. But when
you genuinely have no file change (rotating a secret in
`/home/deploy/secrets`, restarting a wedged worker), use the `force_deploy`
input (issue #260):

```bash
gh workflow run docker-build.yml --ref master -f force_deploy=true
```

**Verified working 2026-07-28** (run 30307168866): `deploy` ran and all five
containers restarted.

Two things about it that are easy to get wrong:

- **It DOES rebuild.** `dorny/paths-filter` has no base commit to diff against
  on a `workflow_dispatch`, so it reports every filter as changed and the full
  test + build matrix runs (~10 min), not the quick re-roll of existing
  `:latest` images you might expect. Budget for that.
- **A bare dispatch without the flag would probably also deploy**, for the same
  reason — the builds succeed, which satisfies the deploy condition on its own.
  `force_deploy` is therefore belt-and-braces rather than the only route: it
  makes the intent explicit and guarantees the deploy even if the build jobs are
  skipped, instead of relying on undocumented paths-filter behaviour that could
  change on an action bump.

An empty commit is **not** an alternative — it produces a green run that deploys
nothing.

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

## 2. Flip B — per-table CT watermarks (safe, but OFF by default)

Flag: `SYNC_PER_TABLE_WATERMARK`. Currently `false` on both sites. It is now
**safe to enable** — issue #259 closed the two defects that made it dangerous —
but nobody has needed it yet, so it stays off.

### What it buys

Each table advances its own `legacy_ct_state_per_table` row, so a row-lock wedge
on one hot table no longer gates the others. That matters because the
2026-07-27 correctness fix deliberately made one errored table hold the **whole**
global advance — no data loss, but a full stall. Per-table decouples that.

Precedent: a 74-minute `HT_Book_H` row-lock on `Book_ID='R015142'`, 2026-05-14.

### Why it's off anyway

The stall it prevents is loud (`CT watermark STUCK` within ~30 min), bounded
(~2 days of CT retention before it turns destructive), and rare (that one
instance in ~2.5 months, comfortably inside the budget). Turning per-table on
trades a well-understood, detectable failure for a mode with no production
hours. Enable it if a wedge actually happens, or when you want the extra
headroom — not as routine hygiene.

### What was fixed (#259, 2026-07-27)

1. **The global row no longer freezes.** `run_one_tick` now writes it as a
   conservative floor under per-table mode: the **minimum** `last_seen_version`
   across the tables this process polls (`global_floor_from_per_table`). Since
   `watermark::advance` is monotonic (`WHERE last_seen_version <= $1`), a floor
   below the current value is a harmless no-op. Because the floor never exceeds
   any table's real progress, resuming from it can only re-read (idempotent),
   never skip.
   This one change also repairs `/health`, `scripts/sync-status.sh`, and the
   watchdog — all three read that row, and it advances again.
2. **The watchdog names the stalest table.** When it does fire under per-table
   mode, the alert identifies the table holding the floor down (lowest version,
   tie-break on oldest poll). Output is byte-identical when the flag is off.

The startup guard that used to refuse `SYNC_PER_TABLE_WATERMARK=true` is
**removed** — it existed only because of those two defects.

Migration 078's reseed remains done: all 19 rows on both sites were force-reset
from the global watermark (hfville 9060 → 37989; hotelnew 17209 → 67695).

### If you do enable it

Flip Ville first, per §0. Then watch for 24h:

- Zero `:rotating_light: CT retention overflow` pages. Any at all ⇒ roll back;
  the reseed didn't hold.
- The **global** row still advancing — that's the floor write working, and it's
  what keeps rollback cheap. If it stops, roll back within 2 days or the startup
  gate will refuse on the next restart.
- Per-table rows clustered, none drifting far behind.
- Quiet tables (`HT_Cupon`, `HT_Deposit`, `HT_Bill_Debt_H/Ds`,
  `HT_CheckIn_Product`, `HT_Receipt_H` — all zero-traffic on Ville) advancing to
  the tick ceiling, not frozen. This is the part with the least production
  evidence: a frozen quiet table sinks below retention and pages days later.

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
