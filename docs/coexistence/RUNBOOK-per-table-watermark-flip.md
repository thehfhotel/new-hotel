# Runbook — per-table CT watermark flip + `missing_pg` re-ingest flip

Two independent flags ship dark in the same release. **Flip them one at a
time, HF Ville first.** Both are reversible.

Background: `docs/coexistence/sync-incident-log.md` → 2026-07-27.
Alert triage: `docs/runbook-sync.md` §9b.

---

## 0. Preconditions (verify BEFORE flipping anything)

The root-cause fix — the global watermark now advancing once per tick to a
pre-loop `CHANGE_TRACKING_CURRENT_VERSION()` ceiling — is **not behind a
flag**. It is live the moment the release deploys. Neither flip below is
required to stop new losses; they are self-heal and defence-in-depth.

```bash
# 1. Release is actually serving (green CI != live).
ssh evergreen "docker inspect new-hotel-production-sync-hfville-1 \
  --format '{{.Config.Image}} {{.State.StartedAt}}'"

# 2. Migration 078 applied on BOTH logical DBs.
for db in hotelnew hotelville; do
  ssh evergreen "docker exec new-hotel-db psql -U postgres -d $db -At \
    -c \"SELECT * FROM schema_migrations WHERE version = '078';\""
done

# 3. Per-table rows are no longer stale — each should now be at/near the
#    global watermark, NOT the pre-078 values (hotelville 9060 / hotelnew 17209).
for db in hotelnew hotelville; do
  echo "== $db =="
  ssh evergreen "docker exec new-hotel-db psql -U postgres -d $db -At -F'|' -c \
    \"SELECT (SELECT last_seen_version FROM legacy_ct_state) AS global,
             min(last_seen_version) AS per_table_min,
             max(last_seen_version) AS per_table_max,
             count(*) AS tables
        FROM legacy_ct_state_per_table;\""
done
```

**Gate:** `per_table_min` must be within a few hundred versions of `global`
on both DBs. If it is still 9060 / 17209, migration 078 did not run — stop,
do not flip. Flipping against stale rows fires one uncooled
`:rotating_light: CT retention overflow` page per table (19 of them) and
then replays an unbounded CT backlog.

Also confirm live flag state from the source of truth — **not** from
compose, whose defaults read `false` even when production is `true`:

```bash
gh variable list | grep -E 'RECONCILE|PER_TABLE|WORKER'
```

---

## 1. Flip A — `missing_pg` re-ingest self-heal (do this first)

This is the flip that heals the 4 stranded HF Ville rows (customer `C2413`,
bookings `R002066|110|112|217`). It is PG-write-only and never writes legacy.

```bash
gh variable set HFVILLE_RECONCILE_REINGEST_MISSING_PG_ENABLED -b true
git commit --allow-empty -m "chore(sync): enable missing_pg re-ingest on HF Ville" && git push
```

### Verify (within ~15 min — one sweep interval + the 1h age gate)

The 4 rows are all far past `FORCE_CONVERGE_MIN_AGE_SECS` (3600s), so they
are eligible on the first sweep.

```bash
# Rows should close. Expect 0.
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelville -At -F'|' -c \
  \"SELECT id, table_name, legacy_pk, resolved_at FROM ht_reconcile_log
     WHERE id IN (4757,4758,4759,4760);\""

# Canonical rows should now EXIST.
ssh evergreen "docker exec new-hotel-db psql -U postgres -d hotelville -At -F'|' -c \
  \"SELECT 'cust', count(*) FROM ht_customers WHERE legacy_cust_no='C2413';
    SELECT 'book', count(*) FROM ht_bookings  WHERE legacy_book_id='R002066';\""
```

Expect `cust|1` and `book|1`. The booking is a **live future reservation**
(2026-11-27 → 11-29, rooms 110/112/217) — after the heal it should appear in
HF Ville occupancy for those nights.

Then confirm the paired all-clear landed in Slack
(`:white_check_mark:` naming `hfville` / `bookings` + `customers`).

### Ordering invariant to watch

Customers are swept before bookings so the FK-defer path holds. If you see
the customer heal but the bookings stay open for more than two sweeps, the
ordering guarantee regressed — grep for the re-ingest log lines and check
whether `booking.rs` returned `Ok(None)` on a missing FK.

### Rollback

`gh variable set HFVILLE_RECONCILE_REINGEST_MISSING_PG_ENABLED -b false` +
redeploy. Already-healed rows stay healed (the canonical write is correct and
idempotent); only further healing stops.

---

## 2. Flip B — per-table watermarks (defence in depth)

Do **not** start this until Flip A has soaked cleanly.

This does not fix the 2026-07-11 class — the once-per-tick ceiling already
did. It fixes a *different* failure mode: one wedged table gating every other
table's advance (canonical case: a 74-min row-lock stall on `HT_Book_H`
`Book_ID='R015142'`, 2026-05-14).

```bash
gh variable set HFVILLE_SYNC_PER_TABLE_WATERMARK -b true
git commit --allow-empty -m "chore(sync): per-table CT watermark on HF Ville" && git push
```

### Watch for 24h

```bash
scripts/sync-status.sh --site hfville          # now reports the stalest per-table row
```

Green means **all** of:

- Zero `:rotating_light: CT retention overflow` pages. Any at all ⇒ roll back
  immediately, the reseed did not take.
- Zero `:rotating_light: CT watermark STUCK` pages.
- Per-table rows all advancing and clustered — no single table drifting far
  behind the others.
- The global `legacy_ct_state` row still advancing (it is kept as a
  conservative floor so `/health`, the watchdog and `sync-status.sh` keep
  working, and so a rollback stays safe).
- Quiet tables (`HT_Cupon`, `HT_Deposit`, `HT_Bill_Debt_H`, `HT_Bill_Debt_Ds`,
  `HT_CheckIn_Product`, `HT_Receipt_H` — all zero-traffic on HF Ville) are
  advancing to the tick ceiling, **not** frozen. A frozen quiet table will
  eventually fall below `CHANGE_TRACKING_MIN_VALID_VERSION` and page.

### Rollback — mind the 2-day cliff

Rolling back within ~2 days is safe: the global floor is still above
`MIN_VALID_VERSION`, so the watcher resumes and replays the per-table-mode
window (idempotent UPSERTs, so correct, but expect an event burst and a
refetch storm on open `/v2` screens via `useLiveRefresh`).

Rolling back **after** the global floor has fallen below `MIN_VALID_VERSION`
means the startup gate legitimately refuses to start, and recovery needs
`--bootstrap`. The global-floor write exists precisely to keep this window
open — verify it is advancing (above) before assuming rollback is cheap.

---

## 3. Then HF Hotel

Only after HF Ville has soaked 24h clean on both flags:

```bash
gh variable set RECONCILE_REINGEST_MISSING_PG_ENABLED -b true
# soak, then:
gh variable set SYNC_PER_TABLE_WATERMARK -b true
```

HF Hotel currently has **zero** unresolved `ht_reconcile_log` rows, so Flip A
should be a no-op there — which makes it a clean canary for the flag itself.
