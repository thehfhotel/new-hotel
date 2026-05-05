# Runbook — HF Ville Phase 5 Cutover (#76)

**Status**: drafted 2026-04-29 during shadow soak. Execute earliest 2026-05-01 ~18:20 ICT (after 48 h soak per #75).

This is the playbook for flipping HF Ville from "ville_sync FreeTDS polling" to "central CT watcher" — task #76 in the Phase 5 Ville sequence. After this runs, HF Ville reads canonical state from the new `hotelville` PG database (fed by `sync-hfville` over the WG path to the upgraded SS2025 MSSQL), and the legacy `ville_sync` polling path is permanently retired.

> **Roll back is at the bottom.** If anything looks wrong, jump there.

---

## Prerequisites (verify before starting)

- [ ] Shadow soak (#75) ran ≥ 48 h with **zero** Slack alerts prefixed `[site=hfville]`
- [ ] `scripts/sync-status.sh --site hfville` shows healthy state, no consecutive-failure trip
- [ ] HF Hotel sync still healthy (drift-reconcile ticks clean, no `[site=hfhotel]` alerts during the soak window)
- [ ] All Phase 5 (Ville) prep tasks complete: #67-#73, #74, #83
- [ ] WG path verified live: `ssh evergreen "timeout 5 bash -c '</dev/tcp/<ville-mssql-host>/1436'"` returns OK
- [ ] Receptionist comms ready (see `docs/runbook-cutover-hfville-thai.md` for the template)
- [ ] At least 30 min reserved on the calendar
- [ ] You are on a stable internet connection (not on mobile data)

## Pre-cutover smoke test (5 min before the maintenance window)

Run a final state snapshot so you have a "before" picture:

```bash
ssh evergreen 'docker exec new-hotel-db psql -U postgres -d hotelville -c "
  SELECT
    (SELECT COUNT(*) FROM ht_customers) AS customers,
    (SELECT COUNT(*) FROM ht_rooms_new) AS rooms,
    (SELECT COUNT(*) FROM ht_bookings) AS bookings,
    (SELECT COUNT(*) FROM ht_checkins) AS checkins,
    (SELECT last_seen_version FROM legacy_ct_state WHERE id=1) AS ct_watermark"'
```

Save the output. After cutover, the canonical state should keep advancing — these row counts should hold or grow, never shrink.

---

## Cutover sequence

### Step 1: Receptionist confirms low-traffic window (T-5 min)

Use the Thai template in `runbook-cutover-hfville-thai.md`. Wait for receptionist's "OK" before proceeding.

### Step 2: Flip `sync-hfville` out of shadow mode (T+0)

Edit `/home/nut/new-hotel-production/.env` on evergreen and set:

```bash
ssh evergreen 'echo "HFVILLE_LEGACY_SYNC_SHADOW_MODE='\''false'\''" | sudo tee -a /home/nut/new-hotel-production/.env'
```

Then force-recreate the sync worker so it picks up the new env:

```bash
ssh evergreen "cd /home/nut/new-hotel-production && docker compose --profile hfville up -d --force-recreate sync-hfville"
```

Verify shadow_mode=false in the startup log:

```bash
ssh evergreen "docker logs new-hotel-production-sync-hfville-1 --tail 5 | grep shadow_mode"
```

Expected: `shadow_mode=false` (was `shadow_mode=true` during soak).

### Step 3: Wait one CT poll cycle (~10 sec) and confirm watermark advances

```bash
ssh evergreen 'docker exec new-hotel-db psql -U postgres -d hotelville -tAc "SELECT last_seen_version, last_polled_at FROM legacy_ct_state WHERE id=1"'
```

Expected: `last_seen_version` is now greater than the bootstrap-stamped value (was 9 at bootstrap, would be 9 + change-count). `last_polled_at` is within last 10 sec.

If watermark NOT advancing → **abort**, jump to rollback.

### Step 4: Repoint frontend `ville_pool` to hotelville (T+1 min)

The HF Hotel backend's `ville_pool` currently reads from the local `ville` schema in `hotelnew` (fed by old ville_sync). Now repoint to the new `hotelville` DB.

Edit `/home/nut/new-hotel-production/.env`:

```bash
ssh evergreen 'sudo sed -i.bak \
  -e "s|^VILLE_DB_NAME=.*$|VILLE_DB_NAME='\''hotelville'\''|" \
  -e "s|^VILLE_DB_HOST=.*$|VILLE_DB_HOST='\''newdb'\''|" \
  /home/nut/new-hotel-production/.env'
```

(Adjust env var names to match what `VilleDbConfig::from_env()` actually reads — verify with `grep VilleDbConfig hotel-backend/src/config.rs` first if unsure.)

If `VILLE_DB_*` env vars don't already exist in `.env`, append them:

```bash
ssh evergreen 'cat <<'\''EOF'\'' | sudo tee -a /home/nut/new-hotel-production/.env
VILLE_DB_HOST='\''newdb'\''
VILLE_DB_NAME='\''hotelville'\''
VILLE_DB_USER='\''postgres'\''
VILLE_DB_PASSWORD='\''NewHotel@2026!'\''
VILLE_DB_PORT='\''5439'\''
EOF'
```

Force-recreate the backend container so the new pool config takes effect:

```bash
ssh evergreen "cd /home/nut/new-hotel-production && docker compose up -d --force-recreate backend"
```

Verify backend is healthy + the new pool is connected:

```bash
ssh evergreen 'docker logs new-hotel-production-backend-1 --tail 10 | grep -iE "ville|pool|connected"'
```

### Step 5: Smoke test from frontend (T+2 min)

In a browser, hit the production frontend, switch the BranchContext selector to "HF Ville", and verify:
- [ ] Rooms page shows Ville's 34 rooms (not blank, not HF Hotel's 58)
- [ ] Bookings page shows recent Ville bookings
- [ ] Customer list shows Ville customers
- [ ] Calendar / occupancy shows current Ville check-ins

Also confirm the HF Hotel branch is unaffected:
- [ ] Switch back to "HF Hotel" — rooms/bookings/customers all match expected HF Hotel state

### Step 6: Confirm with receptionist

The .NET app at Ville should be unchanged (we didn't touch its data path). The new app is now reading from a fresh PG that's CT-synced. Have the receptionist do one quick test transaction at Ville (test booking or a checkin status update) and verify it appears in the new app within ~10 seconds.

### Step 7: Stop & remove old ville_sync container on the Ville jumpbox (T+5 min)

`ville-sync` was already stopped pre-cutover (during the shadow soak). Now remove it cleanly so it can never restart accidentally:

```bash
ssh evergreen 'ssh <ville-jumpbox> "cd ~/hfville && docker compose down 2>&1 | tail -5"'
```

The local `hfville-db` PG container can stop too — its data was a hash-polled mirror, no longer needed:

```bash
ssh evergreen 'ssh <ville-jumpbox> "docker stop hfville-db 2>&1 | tail -3"'
```

Don't `docker rm` yet — keep the volumes for 7 days as a fallback, then task #77 cleans them up properly.

### Step 8: Mark cutover complete

```bash
# Bump version, update CHANGELOG ## [2.55.0] 2026-04-29 cutover entry,
# then commit + push (not this runbook — the actual cutover commit).
```

Slack post-mortem:
- Total elapsed time
- Any anomalies during the steps
- Receptionist feedback

---

## Rollback (if any step above fails)

The strangler pattern means **rollback is reversing the env edits + force-recreate**. No data loss, no DDL undo.

### Rollback step 1: Re-enable shadow mode

```bash
ssh evergreen "sudo sed -i 's|^HFVILLE_LEGACY_SYNC_SHADOW_MODE=.*|HFVILLE_LEGACY_SYNC_SHADOW_MODE=\\\"true\\\"|' /home/nut/new-hotel-production/.env && cd /home/nut/new-hotel-production && docker compose --profile hfville up -d --force-recreate sync-hfville"
```

This puts `sync-hfville` back to TX-rollback, no canonical state mutation.

### Rollback step 2: Re-point ville_pool to old ville schema

Revert the `VILLE_DB_NAME` line in `.env` back to its pre-cutover value (likely `hotelnew` with `?options=-csearch_path%3Dville`). Then:

```bash
ssh evergreen "cd /home/nut/new-hotel-production && docker compose up -d --force-recreate backend"
```

### Rollback step 3: Restart old ville_sync on jumpbox

```bash
ssh evergreen 'ssh <ville-jumpbox> "cd ~/hfville && docker compose up -d ville-sync"'
```

(Note: `ville-sync` was failing in restart-loop because of the MSSQL upgrade port change. The OLD ville-sync code reads from port 1433. To make it work as a rollback fallback, you'd need to ALSO patch the OLD ville-sync .env on the jumpbox to use port 1436. Or accept that rollback gives you "fresh hotelnew read of the old `ville` schema" which is stale to whatever extent ville-sync has been broken. If shadow soak was clean for 48 h pre-cutover and cutover introduced a problem, rollback is to a known-stale state, not catastrophic.)

### Rollback step 4: Communicate

Tell receptionist the .NET app is fine (we didn't touch it) and the new-app's HF Ville view is rolled back to the previous state. Reschedule cutover.

---

## What this runbook does NOT cover

- **Phase 5.5 (Ville)** mirror feature — that's task #80, separate maintenance window weeks later.
- **Backups + DR drill** (#79) — set up after cutover stabilizes.
- **Permanent retirement of ville_sync code + deploy/hfville/ directory** — task #77, done after 1 week of clean cutover state.
- **Move MSSQL off the network segment an internal segment at Ville** — separate security concern, defer to Phase 8 ops hardening.

## References

- ADR 0001 — `docs/adr/0001-phase5-ville-multi-site.md`
- Memory — `~/.claude/projects/-Users-nut-new-hotel/memory/ville_constraint.md`
- Pre-cutover task chain: #67 → #74 (all complete pre-shadow-soak) → #83 (backfill_rooms fix)
