# Phase 3 — mark_dirty polarity fix: 15-minute reception verification runsheet

Fix commit: branch `phase3-mark-dirty` (own `build_statements` writing
`Room_Clean='yes'`; polarity evidence findings.md §3e/§3i + live all-58-rooms
read). Ship ONLY with this window. Rollback = one revert; blast radius = one
room's clean flag + one `HT_Housewife` audit row.

## Pre-flight (before the window; no reception time needed)

- **P1.** Merge `phase3-mark-dirty` → master and push. CI deploys backend +
  writeback worker. Confirm the `deploy` job actually RAN (not skipped) and the
  backend + writeback containers show "Up N seconds". Green CI alone is not
  enough.
- **P2.** Ask reception for ONE test room that is EMPTY tonight and not booked.
  Record its `room_no` `<R>` and `HT_Rooms.id`.
- **P3.** Baseline read (READ-ONLY), keep the output:
  ```
  ssh evergreen 'docker run --rm --network host -e SQLCMDPASSWORD="$(cat /home/deploy/secrets/db_password)" \
    mcr.microsoft.com/mssql-tools /opt/mssql-tools/bin/sqlcmd -S <legacy-mssql-host>,1433 -U sa -d db -C -h -1 -y 8000 \
    -Q "SET NOCOUNT ON; SELECT id, Room_no, Room_Clean, Room_Use, Room_Use_Count, Room_Clean_Time FROM HT_Rooms WHERE Room_no=<R>;"'
  ```
  Expect `Room_Clean='no'`. Canonical: `SELECT room_no, room_clean FROM
  ht_rooms_new WHERE room_no=<R>` → expect `true`.
- **P4.** Reception has iHOTEL open on the room grid, on the screen they
  normally use.

## T+0 .. T+3 — mark dirty in our app

1. In our app: /v2/rooms (or /housekeeping) → select room `<R>` → "Mark dirty"
   (`POST /api/housekeeping/rooms/{aggregate_id}/dirty`).
2. Canonical must flip: `ht_rooms_new.room_clean = false`.
3. Writeback job for intent `mark_room_dirty` must be done with no retry/error.

## T+3 .. T+6 — legacy side + iHOTEL board

4. Re-run the P3 sqlcmd. **PASS** =
   - `Room_Clean`: `'no'` → `'yes'`
   - `Room_Use`, `Room_Use_Count`, `Room_Clean_Time`: IDENTICAL to baseline
     (companion-column assertion — those belong to check-out/cancel, not to a
     standalone dirty flip).
5. Reception refreshes / reopens the iHOTEL room grid: room `<R>` must read as
   NEEDS-CLEANING. Have reception say it in their own words — don't infer from
   colour.
6. Audit-row sanity (read-only):
   `SELECT TOP 3 h_date, h_name, h_room, h_note, h_cin FROM HT_Housewife WHERE
   h_room=<R> ORDER BY h_date DESC` — expect exactly ONE new row, `h_name` =
   the operator who clicked.

## T+6 .. T+10 — THE ECHO TEST (the whole point of the window)

7. Wait at least 3 minutes (≥ 2 sync ticks) without touching the room anywhere.
8. Canonical re-read: `ht_rooms_new.room_clean` **MUST STILL BE `false`**.
   If it flipped back to `true`, the CT echo + mapper inversion is still
   winning → STOP, go to Rollback. This is the exact failure the fix targets.
9. Legacy re-read: `Room_Clean` still `'yes'`.
10. `ht_reconcile_log`: no rooms row for `<R>`, or one that self-closed on the
    next tick.

## T+10 .. T+14 — clean via the normal flow, both sides converge

11. Our app: "Mark clean" on `<R>`. Expect legacy `Room_Clean='no'` AND
    `Room_Clean_Time=''`; canonical `room_clean=true`; iHOTEL grid back to
    clean.
12. Inbound direction unaffected: reception marks a DIFFERENT room clean/dirty
    inside iHOTEL; our app must follow within one tick.

## T+14 — decision

- **PASS** → leave deployed. Then: entry in
  `docs/coexistence/sync-incident-log.md`, update auto-memory
  (reconcile_coverage / app_deployment_status).
- **FAIL at any step** → Rollback below, and record which step failed.

## Rollback / blast radius

- Rollback: `git revert <merge commit>`; push; confirm the deploy job ran and
  containers restarted. Single commit — no migration, no schema change, no
  flag, no `.sqlx` change, no data backfill.
- Restore the test room by clicking mark-clean in iHOTEL's own housekeeping
  flow — never by a direct UPDATE on the shared legacy DB.
- Blast radius: the `Room_Clean` varchar on ONE `HT_Rooms` row, plus one
  `HT_Housewife` audit row. No money, occupancy, booking or check-in data is
  touched. `Room_Use` / `Room_Use_Count` / `Room_Clean_Time` are provably
  untouched (unit test `statement_one_touches_only_the_clean_flag`).
- Worst case if the fix were wrong and left in: one room reads needs-cleaning
  in iHOTEL while actually clean → a housekeeper re-cleans a clean room.

## Why this needs a live window at all

Step 5 (iHOTEL's board) is only observable on reception's screen, and steps
1/11 mutate a room in their live shift loop.

## Known follow-up (not a gate)

Our mark-dirty `HT_Housewife` row has `h_note=''`, which is also iHOTEL's
start-cleaning note value, so `FrmReportHousewife` will count it as a cleaning
by that operator. Fixing it means minting a Thai discriminator literal that
appears in no capture — needs its own evidence pass.
