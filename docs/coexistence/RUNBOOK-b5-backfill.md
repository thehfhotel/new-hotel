# Runbook — Apply Track B5 `ht_checkin_rooms` backfill

> **Status:** UNAPPLIED on both sites as of 2026-05-13.
> **Owner of execution:** the human operator (receptionist coordination required).
> **Estimated downtime:** none. The backfill UPSERTs into a canonical
> PG table the legacy iHOTEL app never reads, so the legacy UI is
> unaffected. The receptionist coordination is precautionary — pick a
> window where no big multi-room walk-in is in progress so the
> dry-run + apply numbers line up cleanly.

## Why this is needed

Track B2 (commit 79f8276) made the canonical sync mapper emit one
`ht_checkin_rooms` row per legacy `HT_CheckIn_Ds` row whenever the CT
watcher re-syncs a folio. But folios that haven't been edited since
the B2 deploy still carry only the deprecated header-level
`ht_checkins.cin_room_id` and NO junction rows. The B3 dashboard reads
correctly fall back to `cin_room_id` for those, so single-room folios
display correctly — but secondary rooms of any multi-room folio that
hasn't been edited since B2 remain invisible on the dashboard until
the next legacy edit naturally triggers a CT re-sync.

The B5 backfill sweeps every still-active legacy folio once and
materialises the junction via the same mapper the CT watcher uses
(`sync::mappers::checkin::apply_checkin_aggregate`). After B5 runs:

* Every ACTIVE legacy `HT_CheckIn_H` folio (`Cin_status = N'ปกติ'` with
  at least one `HT_CheckIn_Ds` row whose `Cin_Room_Status <>
  N'Check-Out'`) has matching `ht_checkin_rooms` rows.
* The dashboard reads (B3) show all rooms correctly for every active
  folio, including the multi-room ones that haven't been touched since
  the B2 deploy.

## Pre-flight checks

1. **Receptionist coordination.** Pick a window where no big
   multi-room walk-in is mid-edit on the iHOTEL UI. The bin doesn't
   touch the legacy DB (read-only) so iHOTEL itself is unaffected,
   but the dry-run summary is easier to validate against a stable
   active-folio count.

2. **Backup verification.** PG nightly backup is the rollback of last
   resort. Confirm the most recent successful backup before the apply
   run. The bin's own rollback procedure (below) is the first line of
   defence and almost always sufficient.

3. **Pipeline is the only deploy path.** Ensure the `chore/track-B5-
   backfill-bin` branch (PR #TODO) is merged to master and the
   docker-build workflow has published the new image tag. The bin
   ships in the same image as `bin/sync.rs` (binaries are co-located
   per `hotel-backend/Cargo.toml`).

4. **Set the legacy DB password.** Export `DB_PASSWORD` in the shell
   you're running from. Value is in 1Password under the legacy-MSSQL
   `sa` entry:

       export DB_PASSWORD='<value>'

5. **Tunnel for HF-Ville only.** HF-Ville's MSSQL is reachable via
   the `hfville` WireGuard interface:

       sudo wg-quick up hfville
       ping -c1 192.168.11.51   # must succeed before continuing

   HF-Hotel's MSSQL is on the LAN — no tunnel needed.

6. **Dry-run first.** ALWAYS run with `--dry-run` first to validate
   the active-folio count and the expected applied count look sane
   before committing. Compare against `SELECT COUNT(*) FROM
   ht_checkins WHERE cin_status = 'active';` — should be close
   (within a few rows of CT lag).

## Apply commands

The backfill bin is published in the same image as the CT watcher
(`ghcr.io/thehfhotel/new-hotel-backend`). The standard apply path is
via `docker compose --profile backfill`, mirroring the
`backfill_rooms` precedent. Run from the project root on
`evergreen.<host>` (the production server).

### Site 1 — HF Hotel

#### Dry run

    SITE_ID=hfhotel \
      docker compose --profile backfill run --rm \
        --entrypoint ./backfill_checkin_rooms \
        backfill-checkin-rooms -- --dry-run

Expected output: a `[DRY-RUN]` summary report with:

* `Folios scanned` close to `SELECT COUNT(*) FROM ht_checkins WHERE
  cin_status = 'active'`.
* `Junction rows applied` representing the number of folios that
  would be touched (NOT the row count — one folio counts as one even
  if it has 3 rooms to insert).
* `Skipped (already match)` for folios where the post-B2 CT watcher
  already populated the junction (folios touched since the B2 deploy).
* `Skipped (no PG row)` should be 0 in a healthy deployment. A
  non-zero value here means the CT watcher hasn't caught up on
  recently-created folios; re-run after the watermark advances.
* `Errors` MUST be 0. Investigate any non-zero value before applying.

#### Apply

After dry-run looks sane, re-run without the flag:

    SITE_ID=hfhotel \
      docker compose --profile backfill run --rm \
        --entrypoint ./backfill_checkin_rooms \
        backfill-checkin-rooms

Expected: identical numbers, but the `[DRY-RUN]` banner becomes
`[APPLY]` and the junction rows are committed.

### Site 2 — HF Ville

Same commands with `SITE_ID=hfville`:

    # Dry run
    SITE_ID=hfville \
      docker compose --profile backfill run --rm \
        --entrypoint ./backfill_checkin_rooms \
        backfill-checkin-rooms -- --dry-run

    # Apply
    SITE_ID=hfville \
      docker compose --profile backfill run --rm \
        --entrypoint ./backfill_checkin_rooms \
        backfill-checkin-rooms

The Ville WG tunnel must be up (`wg-quick up hfville`) before
running.

## Verification queries

### Pre-apply baseline (run BEFORE the apply step)

Capture the current shape so the post-apply numbers are
interpretable:

    -- Active folio count.
    SELECT COUNT(*) AS active_folios
      FROM ht_checkins
     WHERE cin_status = 'active';

    -- Active folios that already carry junction rows.
    SELECT COUNT(DISTINCT c.cin_id) AS already_backfilled
      FROM ht_checkins c
      JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
     WHERE c.cin_status = 'active';

    -- Per-folio room count distribution (one row per folio).
    SELECT room_count, COUNT(*) AS folios
      FROM (
        SELECT c.cin_id, COUNT(cr.cr_id) AS room_count
          FROM ht_checkins c
          LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
         WHERE c.cin_status = 'active'
         GROUP BY c.cin_id
      ) per_folio
     GROUP BY room_count
     ORDER BY room_count;

### Post-apply checks

After the apply step on each site, re-run the queries above. Expected
shape:

* `active_folios` unchanged (the bin doesn't insert into
  `ht_checkins`).
* `already_backfilled` increased to ~= `active_folios` (modulo a
  small CT-lag tail).
* The `room_count=0` bucket should drop to (near) zero — that's the
  pre-B5 gap we just closed. The non-zero buckets shift toward higher
  per-folio counts for multi-room folios.

Spot-check a multi-room folio on the dashboard (`/`): pick one whose
legacy `HT_CheckIn_Ds` carries 2+ rows and confirm BOTH rooms now show
as occupied. Receptionist-driven verification.

## Rollback

The bin writes ONLY into `ht_checkin_rooms`. Rollback is a clean
TRUNCATE-equivalent scoped to the run:

    -- Roll back to a junction-free state for ACTIVE folios only.
    -- B2's mapper will re-create rows for any folio that the legacy
    -- side subsequently edits, so this is reversible without data
    -- loss; the cardinality gap simply re-opens for any UN-edited
    -- multi-room folio.
    DELETE FROM ht_checkin_rooms
     WHERE cr_cin_id IN (
       SELECT cin_id FROM ht_checkins
        WHERE cin_status = 'active'
     );

Use this rollback if:

* The dashboard spot-check after apply shows a wrong room set on a
  folio (i.e. the mapper projected something unexpected). Roll back,
  capture the offending `Cin_no`, debug, re-deploy a fix, re-run.
* A downstream reader reports a NOT NULL constraint trip on the
  junction (none should exist — every column is either nullable or
  defaulted at the schema level).

Note: rolling back does NOT touch the deprecated
`ht_checkins.cin_room_id` (B3's read paths still fall back to it for
junction-less folios), so the dashboard's single-room story remains
unbroken even mid-rollback.

## What success looks like

Within 5 minutes of apply on each site:

* The bin's stdout summary shows `Errors: 0` and `Junction rows
  applied = N` matching expectations.
* Post-apply verification queries show `already_backfilled ≈
  active_folios` and the `room_count=0` bucket near zero.
* Dashboard spot-check on a known multi-room folio surfaces ALL
  rooms (pre-B5 it would have surfaced one).
* No new Slack alerts from `bin/sync.rs` or `bin/writeback.rs` —
  the bin doesn't touch their queues.

## After-apply housekeeping

* No memory updates required — this is a one-shot data fix, not a
  recurring state change.
* The follow-on `ht_checkins.cin_room_id` column DROP is tracked
  separately; do NOT drop the column from this runbook. The remaining
  `cin_room_id` readers (RR.4, invoice, reports) are listed under
  B3.x follow-ups and must cut over first.
* Re-running the bin against the same site is safe and a no-op
  (idempotency by `existing_matches` + `rooms_match`). Use the
  dry-run to spot-check after future multi-room edits if a sanity
  check is needed.
