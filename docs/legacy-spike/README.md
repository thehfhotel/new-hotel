# Legacy DB Reverse-Engineering Spike — Capture Archive

Captured 2026-04-24 against `<legacy-mssql-host> / db` (HF Hotel) using
`scripts/legacy-spike/`. Receptionist drove the 3rd-party Windows app
(`.Net SqlClient Data Provider` from host `<legacy-host>`) while we ran an
Extended Events session capturing every SQL batch the app sent.

## What's in here

| Path | What it is |
|---|---|
| `findings.md` | **Distilled analysis.** Start here. Per-flow write patterns, ID generation, gotchas, writeback design implications. |
| `tables.md` | Table-by-table reference (purpose, key fields, write patterns from observed captures). |
| `schema/00-prereqs.txt` | Server identity, perms, login list, available DBs |
| `schema/01-baseline-schema.txt` | Full schema dump: 60+ tables with columns/PKs/FKs/indexes/views/triggers/procs/functions |
| `schema/02-snapshot-rowcounts.txt` | Per-table row count + checksum (initial baseline) |
| `raw/<capture>/02-before.txt` | Pre-action rowcount snapshot for each capture |
| `raw/<capture>/06-after.txt` | Post-action rowcount snapshot |
| `raw/<capture>/07-events.txt` | **Full XE event stream** (raw SQL with bound parameters, timestamps, session/app metadata). The .xel is cumulative, so events from earlier captures are present too — see `writes.txt` for time-windowed writes. |
| `raw/<capture>/writes.txt` | **Time-windowed writes only** (INSERT/UPDATE/DELETE) for that capture's session. Most useful for analysis. |

## Captures

| Capture dir | Action | Notes |
|---|---|---|
| `walkin-20260424-095304` | Walk-in (room 402, `SPIKE TEST WALKIN`) | First capture. ID photo was attached. |
| `walkin3-20260424-100000` | Walk-in (room 508, `SPIKE TEST WALKIN 3`) | Second sample, no ID photo (Tb_Save_Image UPDATE was a no-op). |
| `checkout-20260424-100323` | Check-out (room 402) | Includes the destructive Phase 1 + actual checkout. |
| `invoice-20260424-100827` | Print invoice (room 414, real customer) | Pay first, then receipt insert. |
| `checkout2-20260424-101023` | Check-out (room 403) | Confirms Phase 1 always fires regardless of which button is clicked. |
| `extend-20260424-101350` | Extend stay (room 508, +1 night) | Destructive Phase B fires AFTER the actual extend. |
| `booking-checkin-20260424-101838` | Create future booking + modify booking + check-in to it (R014810 → CH26-005231, room 402) | Three flows in one capture. |
| `booking-cancel-20260424-103158` | Create booking R014811 + cancel it (room 403) | Cleanest capture: cancel uses targeted UPDATEs + DELETE only, no destructive phase. |
| `checkin-cancel-20260424-114532` | Walk-in for room 306 (CH26-005233) | Confirms walk-in pattern repeats deterministically. |
| `cancel-checkin-20260424-114805` | Cancel CH26-005233 | New flow — INSERTs into `HT_Rooms_Cancel` audit table, no destructive phase. |
| `mark-clean-20260424-115026` | Mark room 306 as clean (housekeeping) | Reveals `Room_Clean='no'` = "no clean needed", and `HT_Housewife.h_cin` references PRIOR non-cancelled occupant. |

## How to revisit

To grep all captures for a specific table or column:
```bash
grep -l 'HT_CheckIn_Pay' raw/*/writes.txt
grep -h 'INSERT INTO \[HT_Customers\]' raw/*/writes.txt | head
```

To see one capture's full timeline:
```bash
less raw/walkin-20260424-095304/writes.txt
```

To see the un-truncated SQL for one event (with bound parameters):
```bash
awk -F'|' '$1 == "<full timestamp>"' raw/<capture>/07-events.txt
```

## Cleanup later

Test data from the spike that should eventually be voided in the legacy app
(do via the app's normal cancel/delete flow):

| Cust_no | Cin_no | Booking# | Room | Notes |
|---|---|---|---|---|
| C21607 | CH26-005228 | — | 402 | walked-in then checked out (already cleared) |
| C21609 | CH26-005230 | — | 508 | walked in, extended stay — still active? |
| C21608 | CH26-005229 | — | 403 | walked in then checked out (already cleared) |
| C21610 | CH26-005231 | R014810 | 402 | booking → check-in then checked out (already done) |
| C21611 | — | R014811 | 403 | booking already cancelled (status='ยกเลิก'), customer record stays |
| C21613 | — | R014812 | 402 | RACE TEST — booking, fixed, then cancelled via writeback (already cleaned) |
| C21615 | — | R014814 | 402 | RACE TEST — booking cancelled via writeback (already cleaned) |
| C21616 | — | R014815 | 402 | RACE TEST — booking cancelled via writeback (already cleaned) |
| C21618 | CH26-005233 | — | 306 | walk-in then cancelled via .NET app, room then marked clean |
