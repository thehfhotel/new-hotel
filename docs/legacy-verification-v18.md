# V18 — is `HT_Book_Ds.Book_Room_Num` a QUANTITY? (long form)

The canonical one-line ledger entry is the **V18** row in
[`docs/coexistence/PENDING-VERIFICATIONS.md`](coexistence/PENDING-VERIFICATIONS.md)
(§Code-gated); this file is its long form. Docs only — nothing here changes behaviour.

**Correction carried from V17:** the column is on **`HT_Book_Ds`**, not `HT_Book_H` — the
header has no `Book_Room_Num` at all. It is `float` (`docs/legacy-app/SCHEMA.sql`;
`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Book_Ds` (A)").

## 1. The question

V18 asks whether `HT_Book_Ds.Book_Room_Num` is a **quantity** — "how many rooms of this
type the guest wants" — rather than a room number or an ordinal. It matters because a
PARKED booking (legacy mode 1: `HT_Book_H.Book_room_type = 1`, header-only, no specific
rooms) is counted by `repository::channel` as a *claim* on inventory, and the claim size
drives both terms of the availability answer: `surplus = max(free_total − ALL parked
claims, 0)` and `available(type) = min(max(free(type) − parked_typed(type), 0), surplus)`.
Today every parked booking contributes exactly **1** to both. If `Book_Room_Num` is a
quantity greater than 1, one legacy header row is really claiming more than one room, our
inventory maths under-counts the pressure it exerts, and the loyalty channel can sell a
room iHOTEL has already promised away. V17's live read already found the shape: HF Hotel
`R003911` carries `Book_Room_Num = 2.0`.

## 2. What the CT mapper assumes today

* **Fetched, then never read.** `BOOK_DS_PROJECTION`
  (`hotel-backend/src/sync/parent_loader.rs:124-135`) lists `"Book_Room_Num"`,
  so it arrives from MSSQL on every `HT_Book_Ds` row — but
  `hotel-backend/src/sync/mappers/booking.rs` has **zero read sites** for it (a
  case-insensitive grep matches only the test name
  `project_aggregate_missing_book_room_type_defaults_to_room_numbers`).
  `BOOK_H_PROJECTION` does not list it, consistent with the header having
  no such column.
* **The mode branch.** `apply_booking_aggregate` (`booking.rs:493`) → `project_aggregate`
  (`booking.rs:661`): when `Book_room_type != 1` each live Ds line's
  (misleadingly named) `Book_Room_Type` string is taken as a **room number**; when it **is**
  1, `rooms` stays empty (header-only → parked) and the first live line's `Book_Room_Type`
  becomes `book_room_type_code`, resolved by `resolve_room_type_code` (`booking.rs:1484`) —
  type lookup first, then V17's `ht_rooms_new.room_no` fallback. Cancelled lines
  (`Book_status = 3`) are skipped in both arms. **Both the line COUNT and the per-line
  `Book_Room_Num` are discarded.** One header → one `ht_bookings` row (`insert_new`
  `:1270` / `update_existing` `:1222`), and zero `ht_booking_rooms` rows when parked
  (`replace_rooms` `:1510`).
* **Where "one header = one claim" actually lives:** `hotel-backend/src/repository/channel.rs`.
  `PARKED_CLAIM_PREDICATE` selects live, date-overlapping bookings with
  `NOT EXISTS (SELECT 1 FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id)`, and
  `inventory_ctes()` counts them by **row cardinality**:

  ```sql
  parked_claims AS (
      SELECT COUNT(*)::int8 AS n FROM ht_bookings b WHERE {PARKED_CLAIM_PREDICATE}
  ),
  parked_claims_typed AS (
      SELECT b.book_room_type_id AS type_id, COUNT(*)::int8 AS n
        FROM ht_bookings b
       WHERE {PARKED_CLAIM_PREDICATE} AND b.book_room_type_id IS NOT NULL
       GROUP BY b.book_room_type_id
  ),
  ```

  **Plainly: yes — one `HT_Book_H` header is exactly ONE parked claim regardless of
  `Book_Room_Num`.** `COUNT(*)`, never `SUM(...)`; no quantity column is consulted anywhere
  on the read path.
* Symmetric outbound assumption: `writeback/recipes/booking_create.rs:204` writes
  `Book_Room_Num` as a hardcoded `1` ("single-room assumption undocumented",
  `docs/legacy-spike/writeback-audit-2026-05-12.md` §"LOW (~25 items)") — correct for us, but it means the
  >1 case has never been exercised from either side.

## 3. Which legacy rows would disprove it

For a mode-1 (`HT_Book_H.Book_room_type = 1`) row:

| observation | meaning |
|---|---|
| `Book_Room_Num > 1` on a line whose `Book_Room_Type` resolves to a room **TYPE** | **QUANTITY** — "N rooms of this type". The `R003911` shape (`เตียงเดี่ยว`, `2.0`). |
| `Book_Room_Num` values matching no `HT_Rooms.Room_no` (e.g. `2.0`, when rooms run 301-313 / 401-418 / 501-518 / A-block / V-codes) | cannot be a room number — kills the ROOM-NUMBER reading outright. |
| `Book_Room_Num` equals the COUNT of the header's live `HT_Book_Ds` lines | a per-header count denormalised onto each line — a quantity, but the claim size is the value, **not** the sum. |
| `Book_Room_Num` disagrees with that count and is > 1 | a per-LINE quantity — claim size is `SUM(Book_Room_Num)` over live lines. This is the case that changes the aggregate SQL. |
| distribution clusters on small integers (1.0, 2.0, 3.0) | QUANTITY. |
| distribution spreads across room-number shapes (301…518) | ROOM NUMBER — V18 is void and the mapper is right to ignore it. |

**Corroboration already in the repo**, short of a per-row proof: iHOTEL's own room-grid
summary runs `select book_room_type, sum(book_room_num) as num from View_Book_Ds2 where
book_status='จอง' … group by book_room_type` — captured live in
`docs/legacy-spike/raw/checkout2-20260424-101023/07-events.txt:24` and documented as query
#3 of `LoadRooms` in `docs/legacy-app/ROOM_GRID_REFRESH.md` §"4. What a refresh actually costs". The legacy app **SUMs**
the column and groups it by type; nobody sums room numbers. Strong, but inferred from
aggregate behaviour — hence the probe.

## 4. The owner-run probe (read-only)

**The owner runs this, not the agent.** Reception boxes are owner-run and DDL is prohibited.
The access path below is **V17's** — the same one that produced its evidence: from the
deploy host out to the legacy MSSQL instance, *not* an interactive session on the reception
PC. Use it as written rather than improvising a local `sqlcmd -E` on the box.
Both statements are strictly `SELECT` — no temp tables, no writes, no DDL — bounded by
`TOP`, under `READ UNCOMMITTED` so they cannot block live reception. The password is read
from the secret file inside the remote shell and never transcribed. Datetimes in these
tables are local Thai (GMT+7) stored naive; nothing here converts them.

**Q1 — the mode-1 rows, with the two falsifiers (HF Hotel):**

```
ssh evergreen 'docker run --rm --network host -e SQLCMDPASSWORD="$(cat /home/deploy/secrets/db_password)" \
  mcr.microsoft.com/mssql-tools /opt/mssql-tools/bin/sqlcmd -S 192.168.100.222,1433 -U sa -d db -C -W -s"|" \
  -Q "SET NOCOUNT ON; SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED; SELECT TOP 200 h.Book_ID, h.Book_room_type, d.Book_Room_Type, d.Book_Room_Num, d.Book_status, (SELECT COUNT(*) FROM HT_Book_Ds x WHERE x.Book_No = h.Book_ID AND x.Book_status <> 3) AS live_ds_lines FROM HT_Book_H h JOIN HT_Book_Ds d ON d.Book_No = h.Book_ID WHERE h.Book_room_type = 1 ORDER BY h.Book_Date_in DESC;"'
```

**Q2 — the value distribution over all live Ds lines (the decisive discriminator):**

```
ssh evergreen 'docker run --rm --network host -e SQLCMDPASSWORD="$(cat /home/deploy/secrets/db_password)" \
  mcr.microsoft.com/mssql-tools /opt/mssql-tools/bin/sqlcmd -S 192.168.100.222,1433 -U sa -d db -C -W -s"|" \
  -Q "SET NOCOUNT ON; SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED; SELECT TOP 50 d.Book_Room_Num, COUNT(*) AS rows_with_that_value FROM HT_Book_Ds d WHERE d.Book_status <> 3 GROUP BY d.Book_Room_Num ORDER BY COUNT(*) DESC;"'
```

For **HF Ville**, the same two commands with `-S 192.168.11.51,1436 -d HOTEL` (same
`db_password` secret). V17 found 0 mode-1 headers there out of 2,460, so Q1 is expected to
return nothing and Q2 is the only informative one.

**How to read it.** Q2 settles it: values clustered on small integers with `1.0` dominating
⇒ **QUANTITY**, V18 is real; values spread across room-number shapes (`301`, `402`, `518`)
⇒ **ROOM NUMBER**, V18 is void. In Q1, any row with `Book_Room_Num > 1` confirms a
multi-room claim, and comparing it against `live_ds_lines` says whether the claim size is
the value itself (they match) or the sum over lines (they do not).

## 5. What we would change if it is a quantity — NOT in this PR

* **Migration** — nullable `ht_bookings.book_room_claim_qty SMALLINT` (NULL = "no opinion"
  = 1, so every existing row stays correct). ALTER-only ⇒ `migrations/pg/` +
  `init-db/init-hotelnew.sql` + a `migrations/README.md` row, and an annotation on the
  existing `ht_bookings` row in `CARDINALITY_MAP.md` rather than a new one.
* **Mapper** (`sync/mappers/booking.rs`) — read `Book_Room_Num` off the live
  (`Book_status <> 3`) Ds lines in the mode-1 arm, clamp to `>= 1`, store it, and make it an
  unguarded gate term like `book_room_type_id` so a quantity edit in iHOTEL re-applies and
  converges. First-line value vs `SUM` over lines is exactly what Q1's `live_ds_lines`
  column decides — do not guess it.
* **Repository** (`repository/channel.rs`) — `parked_claims` and `parked_claims_typed`
  become `SUM(COALESCE(b.book_room_claim_qty, 1))::int8` instead of `COUNT(*)::int8`. That
  is the whole behaviour change; `inventory_surplus` and `type_availability` keep their shape.
* **No legacy write, no dark flag** — inbound-only. `booking_create`'s hardcoded
  `Book_Room_Num = 1` stays (byte parity; our app creates single-room bookings only) and
  should be documented rather than "fixed". Invariant #6 is not engaged.
* **Scope/value check first.** At today's volume this buys little: 4 mode-1 Ds rows at HF
  Hotel, 0 at HF Ville, only 2 of the 4 `Book_status = 1` — and neither active row is the
  multi-room one. `hotel-backend/tests/test_channel_parked_inventory.rs` would need
  extending for a case production has not yet produced. Probe first, then decide.
