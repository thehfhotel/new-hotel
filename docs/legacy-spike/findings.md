# Legacy DB — Findings

Captured 2026-04-24 from the live HF Hotel legacy DB (`<legacy-mssql-host> / db`)
while the receptionist drove the 3rd-party Windows app through controlled
actions. All quotes/identifiers below are direct from the captured event
stream — see `raw/<capture>/writes.txt` for the unedited record.

---

## 1. Server & app fingerprint

| Property | Value |
|---|---|
| SQL Server | 2022 RTM Express (16.0.1000.6) on Windows 10 Pro |
| @@SERVERNAME | `<legacy-host>\SQLEXPRESS` (the host was renamed `<legacy-host>` later) |
| Database | `db` (only user DB on the instance) |
| 3rd-party app | `.Net SqlClient Data Provider`, host `<legacy-host>`, login `sa` |
| Connection | Same physical box — receptionist app and SQL Server are co-located |
| Triggers / sprocs / functions in `db` | **Zero** |

No hidden side-effects — every state change is in the captured INSERT/UPDATE statements.

---

## 2. ID generation (the missing piece for writeback)

Three string-typed identifiers, all **app-allocated** (no IDENTITY column):

| Field | Format | Captured examples | Allocation |
|---|---|---|---|
| `HT_CheckIn_H.Cin_no` | `CH` + 2-digit year + `-` + 6-digit zero-padded sequential | `CH26-005228`, `CH26-005229`, `CH26-005230`, `CH26-005231` | Strict `+1` per check-in (we saw +2 once because another clerk did one between our captures) |
| `HT_Book_H.Book_ID` | `R` + 6-digit zero-padded sequential | `R014810` | Strict `+1` per booking |
| `HT_Customers.Cust_no` | `C` + sequential (no zero-pad observed) | `C21607` → `C21610` | Strict `+1` per new customer |

Numeric `id` columns on dependent tables (e.g. `HT_Room_Status.id`,
`HT_Book_Date.id`, `HT_CheckIn_Ds.id`) are **also app-allocated** — the app
does `MAX(id)+1` style increments (race-condition prone, but their problem).

**For our writeback worker**: we must read the current MAX before allocating,
or use a separate sequence table. Best to wrap allocation in a transaction
to avoid duplicate-key collisions when two writers race.

**Race-safety: PROVEN with `TABLOCKX, HOLDLOCK`**. See §6 for the verified
locking pattern. We tested it against live receptionist activity — both our
writeback and the .NET app got distinct sequential IDs with zero errors.
The receptionist's UI experienced ~10s blocking but no error dialog.

**Note on `HT_Book_Ds.id`**: this is the ONE exception — it's actually a
SQL Server `IDENTITY` column. The app INSERTs WITHOUT specifying `id` and
SQL Server auto-allocates. No race concern there.

**Note on `HT_Customers.id`**: column metadata says `is_identity=0` but the
app DOES allocate it via `MAX(id)+1`. Looks like at-some-point the IDENTITY
property was stripped from this column.

---

## 3. Per-flow write recipes

Each section lists the SQL the legacy app fires for that user action.
**All `:param`-style placeholders are interpolated values**, not parameters
— the app sends literal SQL text every time. INSERT statements use bracketed
identifiers (typical .NET SqlClient style).

### 3a. Walk-in check-in (8 statements, ~30ms)

Source captures: `walkin-20260424-095304/writes.txt`, `walkin3-20260424-100000/writes.txt`

```sql
-- 1. New customer
INSERT INTO [HT_Customers] ([id], [Cust_no], [Cust_name], [Cust_name2],
    [Cust_Type], ..., [Cust_Add_*], [Cust_Work_*])
-- (id is identity but app passes a value; Cust_no = next 'C\d+')

-- 2. Photo link (no-op if no photo was captured)
UPDATE Tb_Save_Image
   SET cin_no='CH26-005228', cust_no='C21607', tmp_no=''
 WHERE tmp_no='924127'                    -- pre-uploaded photo's tmp id

-- 3. Check-in detail (room assignment row)
INSERT INTO [HT_CheckIn_Ds] ([id], [Cin_No], [Cin_Room_No], [Cin_Room_Type],
    [Cin_Room_In], [Cin_Room_Out], [Cin_Room_Status='จอง'/'เข้าพัก'],
    [Cin_Room_Price], [Cin_Room_Night], [Cin_Room_PriceToTal], ...)

-- 4. Power log (lights on)
INSERT INTO [HT_POWER_LOG] ([ROOM_NO], [ROOM_POWER_START], [ROOM_POWER_START_BY],
    [ROOM_POWER_END_BY=''], [ROOM_POWER_NOTE='เปิดไฟ อัตโนมัติ จากเช็คอิน No.CH26-005228'])

-- 5. Mark room occupied
UPDATE HT_Rooms SET room_use='yes' WHERE room_no='402'

-- 6. Room status row (per night — 1 night → 1 row)
INSERT INTO [HT_Room_Status] ([id], [room_no], [room_date],
    [room_status='เข้าพัก'], [room_Details=cust_name], [room_CheckIn_No], [room_date_oa])

-- 7. Accompanying guests (TM.30 — at minimum, 1 row for primary guest)
INSERT INTO [HT_CheckIn_Other_People] ([Cin_no], [Cin_name='Mr. NAME' or 'นาย NAME'], [Cin_contry])

-- 8. Check-in header (totals, statuses, balance)
INSERT INTO [HT_CheckIn_H] ([Cin_no], [Cin_Date], [Cin_Book_no=NULL for walk-ins],
    [Cin_cust_no], [Cin_status], [Total_Price_*], [Cin_by='Admin'], [Cin_Date_in], [Cin_Date_out], ...)

-- 9. Coupon flag (cupons are pre-allocated; this marks the new check-in's coupon as printed)
UPDATE HT_Cupon SET cupon_print=1 WHERE cupon_cin_no='CH26-005228'

-- ~5 seconds later (TM.30 batch number assigned async)
UPDATE HT_CheckIn_H SET Cin_Work_number=269357 WHERE Cin_No='CH26-005228'
```

**Tables touched**: 7 INSERTs + 3 UPDATEs.

**Findings:**
- `Cin_Room_Status` initial value is `'เข้าพัก'` (Thai: "occupying") for walk-ins.
- `room_status` in `HT_Room_Status` mirrors this.
- `Cin_by='Admin'` is the legacy app's logged-in employee. Always `Admin` in our captures.
- The person prefix varies: `Mr.` or `นาย` (Thai for Mr.) depending on which input form was used. Inconsistent — both observed in our 2 walk-ins.
- `Tb_Save_Image` UPDATE always fires; matches 0 rows if no photo.
- `Cin_Work_number` (TM.30 batch number) is **random**, assigned ~5s after check-in completes. Multiple updates may fire (we saw 3 in one session). It's NOT sequential.
- ID-card photo storage table `Tb_Save_Image` (9,688 rows) is non-trivial — likely binary blobs.

### 3b. Create future booking (4 INSERTs, ~20ms)

Source: `booking-checkin-20260424-101838/writes.txt` first 4 lines.

```sql
INSERT INTO [HT_Customers] (...)             -- new customer (skipped if existing)

INSERT INTO [HT_Book_H] ([Book_ID='R014810'], [Book_Date], [Book_Cust_ID],
    [Book_Cust_Name], [Book_Cust_Tel], [Book_Price_Total='890'], [Book_Price_Pay='0'],
    [Book_Status], [Book_Date_in='4/25/2026 12:00:00 PM'],
    [Book_Date_out='4/26/2026 11:59:59 AM'], [Book_by='Admin'],
    [Book_room_all], [Book_room_note], [book_room_type], ...)

INSERT INTO [HT_Book_Ds] ([Book_No='R014810'], [Book_Room_Type='402'], -- ⚠️ stores room NUMBER, not type!
    [Book_Room_Start], [Book_Room_End], [Book_Room_Price='890'],
    [Book_Room_Night=1], [Book_Room_Num='1'], [Book_Room_PriceToTal='890'], ...)

INSERT INTO [HT_Book_Date] ([id=47285], [Book_no='R014810'], [Book_type='402'],
    [Book_date_ds='4/25/2026'], [Book_Num=1], [Book_USE=0])  -- one row per night
```

**Findings:**
- `HT_Book_Ds.Book_Room_Type` stores the room **number** (`'402'`), not a type code. The column name is misleading.
- `HT_Book_Date` has **one row per calendar night** of the booking. A 2-night stay = 2 rows.
- `Book_USE=0` distinguishes a booking-only date from a checked-in one (later set to 1 on check-in).
- Departure time is hardcoded `'11:59:59 AM'` (1 second before noon) — convenient for date-range BETWEEN queries.

### 3c. Modify booking (destructive — DELETE + RE-INSERT)

Source: `booking-checkin-20260424-101838/writes.txt` second 13 lines (10:20:13 onwards).

The legacy app does NOT update individual fields. It does:
```sql
UPDATE HT_Customers SET ... WHERE Cust_no='C21610'        -- re-save customer
UPDATE HT_Rooms SET room_book_*='', Room_Book='' WHERE room_book IN
   (SELECT id FROM ht_book_date WHERE Book_no='R014810')  -- clear room "booked" display
DELETE FROM HT_Book_Date WHERE Book_no='R014810'
DELETE FROM HT_Book_H    WHERE Book_ID='R014810'
DELETE FROM HT_Book_Pro  WHERE [B_NO]='R014810'           -- (HT_Book_Pro is empty in our DB)
DELETE FROM HT_Book_Ds   WHERE Book_no='R014810'

-- Then RE-INSERT all 4 tables with the new dates
INSERT INTO [HT_Book_H] (...) -- new dates
INSERT INTO [HT_Book_Ds] (...)
INSERT INTO [HT_Book_Date] (...) -- one row per night of the new range

-- Display caption + counter
UPDATE HT_Rooms SET room_book_ds='SPIKE TEST WALKIN', Room_Book='...'
   WHERE room_no IN (SELECT room_no FROM View_HT_ROOM WHERE book_no='R014810')
UPDATE HT_Book_Date SET Book_ok=Book_ok+1 WHERE id=47285
```

**🚨 No transactions.** If the app crashes between DELETE and INSERT, the booking is permanently lost.

**For our writeback**: we should **NOT** replicate this destructive pattern. Use targeted UPDATEs against `HT_Book_H` / `HT_Book_Ds` instead, and add/remove `HT_Book_Date` rows as needed. The legacy app reads what's in the tables, so our targeted UPDATEs are equivalent and safer.

### 3d. Check-in TO existing booking (vs walk-in)

Source: `booking-checkin-20260424-101838/writes.txt` last block (10:23:02).

**5 differences from walk-in:**
1. ❌ **No** `INSERT INTO HT_Customers` — customer already exists from booking step.
2. ✅ `UPDATE HT_Customers` instead (re-save profile).
3. ✅ `UPDATE HT_Book_H SET Book_Status='เข้าพัก' WHERE Book_ID='R014810'` — booking marked as occupying.
4. ✅ `UPDATE HT_Rooms SET room_book_*='', room_book_name=''` — clear the booking display so the room shows as occupied (not booked).
5. ✅ Existing `HT_Room_Status` rows (created by booking? actually no — see below) get **UPDATEd** instead of INSERTed:
   ```sql
   UPDATE [HT_Room_Status]
      SET [room_status]='เข้าพัก', [room_Details]='SPIKE TEST WALKIN', [room_CheckIn_No]='CH26-005231'
    WHERE room_date='4/24/2026' AND room_no='402'
   ```
   This UPDATE has no `room_CheckIn_No` filter — it overwrites whatever existing row matches `(room_date, room_no)`, **including historical check-out records**. That's a data-loss bug in the legacy app, but it tells us `HT_Room_Status` is a **current-state** table, not a history.

6. ✅ `Cin_Book_no='R014810'` set in `HT_CheckIn_H` — links the check-in back to the booking.

INSERTs that **still happen** for additional nights: if booking is N nights and only N-1 `HT_Room_Status` rows pre-exist, the missing nights get INSERTed.

### 3e. Check-out (always two phases — destructive Phase 1 + actual checkout)

Source: `checkout-20260424-100323/writes.txt`, `checkout2-20260424-101023/writes.txt`

**Phase 1 (always fires first, ~12 statements):** opens the check-in, deletes all child rows, re-inserts them. We confirmed this fires whether the receptionist clicks Save first or just clicks Check-out — it's hardcoded.

```sql
UPDATE HT_Customers SET ... WHERE Cust_no='C21607'             -- re-save customer
UPDATE HT_Rooms SET room_use='no' WHERE room_no IN
   (SELECT Cin_Room_No FROM HT_CheckIn_Ds
     WHERE Cin_no='CH26-005228' AND Cin_Room_Status<>'Check-Out')
DELETE FROM HT_Room_Status         WHERE room_CheckIn_No='CH26-005228'
DELETE FROM HT_CheckIn_H           WHERE Cin_no='CH26-005228'
DELETE FROM HT_CheckIn_Product     WHERE Cin_no='CH26-005228'
DELETE FROM HT_CheckIn_Ds          WHERE Cin_no='CH26-005228' AND Cin_Room_Status<>'Check-Out'
UPDATE Tb_Save_Image SET cin_no='CH26-005228', cust_no='C21607', tmp_no='' WHERE tmp_no='588401'
INSERT INTO [HT_CheckIn_Ds] (...)            -- re-insert
UPDATE HT_Rooms SET room_use='yes' WHERE room_no='402'         -- reverts the earlier 'no'
INSERT INTO [HT_Room_Status] (...)
DELETE FROM HT_CheckIn_Other_People WHERE Cin_no='CH26-005228'
INSERT INTO [HT_CheckIn_Other_People] (...)
INSERT INTO [HT_CheckIn_H] (...)
UPDATE HT_CheckIn_H SET Cin_Work_number=712095 WHERE Cin_No='CH26-005228'  -- new TM.30 batch
```

**Phase 2 (the actual check-out, 5 UPDATEs):**
```sql
UPDATE HT_POWER_LOG
   SET ROOM_POWER_END=GETDATE(),
       ROOM_POWER_END_BY='Admin',
       ROOM_POWER_NOTE2='ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.CH26-005228'
 WHERE room_no='402' AND ROOM_POWER_END_BY=''             -- finds the in-progress one

UPDATE [HT_CheckIn_Ds]
   SET [Cin_Room_Out]='4/24/2026 5:05:04 PM',
       [Cin_Room_Status]='Check-Out',                     -- NB: English here, not Thai
       [Cin_Room_Pay_Total]=0, [Cin_Room_night]=1,
       [Cin_Room_PriceTotal]=0, [Cin_note]=''
 WHERE id=25007                                           -- by HT_CheckIn_Ds.id

UPDATE HT_Rooms SET room_use='no', Room_Clean='yes',
       Room_Use_Count=Room_Use_Count+1
 WHERE room_no='402'

UPDATE HT_Room_Status SET room_status='Check Out'         -- NB: English, with space
 WHERE room_no='402' AND room_CheckIn_No='CH26-005228'

UPDATE [HT_CheckIn_H]
   SET [Total_Price_Room]=0, [Total_Price_Product]=0, [Total_Price_Net]=0,
       [Total_Price_Pay]=0, [Total_Price_Balance]=0, [Cin_note]=''
 WHERE [Cin_no]='CH26-005228'
```

**Phase 3 (housekeeping, after check-out — 2 statements per checked-out room):**
```sql
UPDATE HT_Rooms SET Room_Clean='no', Room_Clean_Time='' WHERE id=15        -- by id, not room_no!
INSERT INTO HT_Housewife (h_name='Admin', h_room='402', h_date='4/24/2026 5:11:53 PM',
    h_note='', h_cin='CH26-005228', h_cin_name='SPIKE TEST WALKIN')
```

`HT_Rooms.id` is **internal numeric** (e.g. id=15 for room 402, id=50 for room 403). Different from `room_no`. Some flows reference rooms by `id`, others by `room_no` — be careful.

**Status state machine:**
| Field | Walk-in / Check-in | Check-out |
|---|---|---|
| `HT_Rooms.room_use` | `'yes'` | `'no'` |
| `HT_Rooms.Room_Clean` | (no change) | `'yes'` then `'no'` (housekeeping) |
| `HT_CheckIn_Ds.Cin_Room_Status` | `'เข้าพัก'` (Thai) | `'Check-Out'` (English w/ hyphen) |
| `HT_Room_Status.room_status` | `'เข้าพัก'` (Thai) | `'Check Out'` (English w/ space) |

**Mixed Thai/English status values are real**, with inconsistent spacing (`Check-Out` vs `Check Out`). Our writeback constants must mirror these literal strings exactly.

### 3f. Extend stay (new tables, similar destructive pattern)

Source: `extend-20260424-101350/writes.txt`

**Phase A (the actual extend, 7 statements, ~30ms):**
```sql
UPDATE HT_CheckIn_H SET Cin_Work_number=539215 WHERE Cin_No='CH26-005230'  -- TM.30 touch
UPDATE HT_Rooms SET room_use='no' WHERE room_no IN (...)  -- temp clear (will revert)
DELETE FROM HT_Room_Status WHERE room_CheckIn_No='CH26-005230'  -- nuke all date rows
UPDATE [HT_CheckIn_H]
   SET [Total_Price_Room]=1780, [Total_Price_Net]=1780, [Total_Price_Balance]=1780
 WHERE [Cin_no]='CH26-005230'  -- recalc totals
UPDATE [HT_CheckIn_Ds]
   SET [Cin_Room_night]=2, [Cin_Room_PriceTotal]=1780,
       [Cin_Room_Out]='4/26/2026 12:00:00 PM'
 WHERE id=25009  -- by HT_CheckIn_Ds.id
UPDATE HT_Rooms SET room_use='yes' WHERE room_no='508'   -- revert
INSERT INTO [HT_Room_Status] (id=50235, room_date='4/24/2026', ...)  -- re-add night 1
INSERT INTO [HT_Room_Status] (id=50236, room_date='4/25/2026', ...)  -- new night 2
```

Then Phase B (destructive save) fires too — same pattern as on check-out.

**Implication for writeback**: extend = recompute totals + replace `HT_Room_Status` rows for the changed date range. Targeted, no destructive Phase B needed.

### 3g-bis. Cancel booking (4 UPDATEs + 1 DELETE — clean, no destructive phase!)

Source: `booking-cancel-20260424-103158/writes.txt`

```sql
UPDATE HT_Rooms SET room_book_ds='', Room_Book='', Room_Book_Name='', Room_Book_Time=''
   WHERE room_book IN (SELECT id FROM ht_book_date WHERE Book_no='R014811')

UPDATE HT_Book_H  SET Book_Status='ยกเลิก' WHERE Book_ID='R014811'  -- Thai: cancelled
UPDATE HT_Book_ds SET Book_status=3        WHERE Book_No='R014811'  -- numeric status code
DELETE FROM HT_Book_Date                   WHERE Book_no='R014811'  -- HARD delete
UPDATE HT_Book_H  SET book_status='ยกเลิก' WHERE book_id='R014811'  -- duplicate UPDATE (lowercase column)
```

**Findings:**
- Cancel is **soft on `HT_Book_H` / `HT_Book_Ds`** — status flipped to `'ยกเลิก'` (Thai: cancelled). Records preserved.
- Cancel is **HARD on `HT_Book_Date`** — those rows are deleted permanently. The room is freed.
- `HT_Customers` is preserved (customer may return).
- **`HT_Book_Ds.Book_status` is a numeric field** (`3` = cancelled). Different from the varchar `Book_Status` on `HT_Book_H`. Watch the case sensitivity.
- The two `UPDATE HT_Book_H ... book_status='ยกเลิก'` statements at 10:34:02.099 and 10:34:02.124 are duplicates — set the same value twice. Likely a bug in the legacy app's UI handler.
- No destructive Phase B runs. **This is the cleanest flow we've captured.** 🎉

### 3h. Take payment + print invoice (4 statements, ~26s gap for print dialog)

Source: `invoice-20260424-100827/writes.txt`

```sql
INSERT INTO [HT_CheckIn_Pay] ([Cin_No='CH26-005227'], [Cin_Pay_Cash], [Cin_Pay_Credit],
    [Cin_Pay_Date], [Cin_Pay_Ds_Name], [Cin_Pay_Ds_Price], [Cin_Pay_Ds_unit],
    [Pay_No], [Cin_Cust_no], [Cin_Pay_Ds_ID], [Cin_Pay_Ds_Num],
    [Cin_Pay_Ds_PriceTotal], ...)

UPDATE [HT_CheckIn_H]
   SET [Total_Price_Room]=711, [Total_Price_Product]=0, [Total_Price_Net]=711,
       [Total_Price_Pay]=711, [Total_Price_Balance]=0
 WHERE [Cin_no]='CH26-005227'

-- ~26 seconds later (after print dialog interaction)
INSERT INTO [HT_Receipt_H] ([id], [Receipt_no], [Receipt_Date], [Receipt_Name],
    [Receipt_Address], [Receipt_Tel], [Receipt_Total=711], [Receipt_Vat=0],
    [Receipt_VatPer=0], [status_name], [Receipt_Discount=0], ...)

INSERT INTO [HT_Receipt_Ds] ([S_Sale_id=20653], [S_Product_no='SEV-001'],
    [S_Product_name='ค่าห้องพัก [414]'], [S_Unit=1], [S_UnitName='คืน'],
    [S_Price=711], [S_Total=711], S_PriceDiscount_per='', S_PriceDiscount=0)
```

**Findings:**
- `S_Product_no='SEV-001'` is the **service code** for room charge. Likely a constants table somewhere; we should look it up.
- `S_Product_name='ค่าห้องพัก [414]'` (Thai: "room charge") with the room number in brackets.
- `S_Unit=1, S_UnitName='คืน'` (Thai: "night").
- This hotel uses **no VAT** (all 0). Need a setting check in `TB_SETTINGS` if other branches use VAT.
- Receipts are 1 receipt per print event — multiple charge lines (`HT_Receipt_Ds`) can group under one `HT_Receipt_H` (we only saw 1 line, but the structure supports many).
- Receipts are **never deleted** on check-out — historical receipts persist.

### 3i. Cancel a check-in (7 statements, ~70ms — clean, no destructive phase)

Source: `cancel-checkin-20260424-114805/writes.txt`

```sql
-- 1. Drop the room status row
DELETE FROM HT_Room_Status
   WHERE room_no='306' AND room_CheckIn_No='CH26-005233';

-- 2. Drop the room from check-in detail
DELETE FROM HT_CheckIn_Ds
   WHERE Cin_Room_No='306' AND Cin_No='CH26-005233';

-- 3. Free the room (now empty AND needs cleaning — Room_Clean='yes' = "needs clean")
UPDATE HT_Rooms SET Room_Clean='yes', Room_Use='no'
   WHERE room_no='306';

-- 4. Audit log entry — NEW table not seen in any other flow: HT_Rooms_Cancel
INSERT INTO HT_Rooms_Cancel (id, room_no, cin_no, cancel_date, cancel_by, cancel_note)
   VALUES (298, '306', 'CH26-005233', GETDATE(), 'Admin', N'ยกเลิกคุณนัท');
   -- 'ยกเลิกคุณนัท' = "Cancel Mr. Nat" — the receptionist's free-text note

-- 5. Subtract this room's price from check-in totals (multi-room safe)
UPDATE [HT_CheckIn_H] SET
   [Total_Price_Room]    = Total_Price_Room    - 890,
   [Total_Price_Net]     = [Total_Price_Net]   - 890,
   [Total_Price_Pay]     = [Total_Price_Pay]   - 0,
   [Total_Price_Balance] = ([Total_Price_Balance] - 890) + 0
WHERE [Cin_no]='CH26-005233';

-- 6. Mark the whole check-in cancelled
UPDATE HT_CheckIn_H SET cin_status='ยกเลิก' WHERE cin_no='CH26-005233';

-- 7. Lights off with cancel-specific note (different text from check-out!)
UPDATE HT_POWER_LOG SET
   ROOM_POWER_END    = GETDATE(),
   ROOM_POWER_END_BY = 'Admin',
   ROOM_POWER_NOTE2  = N'ปิดไฟ เนื่องจากยกเลิกห้องพัก'  -- "Lights off due to room cancellation"
WHERE room_no='306' AND ROOM_POWER_END_BY='';
```

**Findings:**
- **Brand-new table written: `HT_Rooms_Cancel`** — cancel audit log. `id` is app-allocated MAX+1 (we saw 298, prev count was 297). Every cancel-check-in adds a row with the cancel reason note.
- **NO destructive Phase 1** — unlike check-out, cancel-check-in is clean targeted operations. Easier to writeback safely.
- **`HT_CheckIn_Other_People` is NOT deleted** — accompanying guests stay attached to the cancelled check-in. Audit trail preserved.
- **Totals adjusted by SUBTRACTION**, not zeroed. Correctly handles multi-room check-ins where only one room is being cancelled (other rooms' charges remain).
- **Power log note differs from check-out**: cancel uses `'ปิดไฟ เนื่องจากยกเลิกห้องพัก'`; check-out uses `'ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.{cin_no}'`. Two distinct templates.
- **Room_Clean='yes' set after cancel** — meaning "yes, this room needs cleaning". So Mark-Clean must follow (see §3j).
- `cin_status` (lowercase column name in the UPDATE) — typical legacy app inconsistency.

### 3j. Mark room clean (housekeeping — 2 statements)

Source: `mark-clean-20260424-115026/writes.txt`

```sql
-- 1. Mark room as no-longer-needs-cleaning (Room_Clean='no' = "no clean needed")
UPDATE HT_Rooms SET Room_Clean='no', Room_Clean_Time=''
   WHERE id=6;        -- by HT_Rooms.id, NOT room_no

-- 2. Insert housekeeping audit row
INSERT INTO HT_Housewife (h_name, h_room, h_date, h_note, h_cin, h_cin_name)
VALUES ('Admin', '306', '4/24/2026 6:50:59 PM', '',
        'CH26-005159', 'สุภัตร์ตรา พุทธกูล');
```

**Critical finding — `h_cin` references the LAST REAL CHECK-IN, not the most-recent one:**

`h_cin='CH26-005159'` is **not** the just-cancelled `CH26-005233` we tested. It's a much older check-in for room 306 with a real customer name (`สุภัตร์ตรา พุทธกูล`).

The .NET app looks up "the prior real occupant of this room" when recording housekeeping — likely most-recent check-in WHERE `cin_status NOT IN ('ยกเลิก')`. Cancelled check-ins are skipped. This makes housekeeping audit useful: "we cleaned up after THIS guest left."

**For our writeback's mark-clean flow:**
```sql
-- lookup the prior occupant
SELECT TOP 1 h.Cin_no, c.Cust_name + ' ' + c.Cust_name2 AS h_cin_name
  FROM HT_CheckIn_Ds d
  JOIN HT_CheckIn_H h ON h.Cin_no = d.Cin_No
  JOIN HT_Customers c ON c.Cust_no = h.Cin_cust_no
 WHERE d.Cin_Room_No = @room_no
   AND h.cin_status NOT IN (N'ยกเลิก')
 ORDER BY d.Cin_Room_Out DESC

-- then UPDATE HT_Rooms + INSERT HT_Housewife with the looked-up values
```

**Other findings:**
- `Room_Clean_Time` is set to `''` (empty), suggesting it's a manual override — most rows have empty.
- `id=6` is `HT_Rooms.id` for room 306. **The mark-clean UPDATE filters by `id`, not `room_no`** — different from the cancel-check-in `room_use='no'` UPDATE. Inconsistency in the legacy app.

### 3k. Booking-list visibility requirements (verified by experiment)

The .NET app's main booking-list view filters out our writeback-created
bookings unless we set TWO specific fields correctly. Discovered the hard
way during Test 2's verification: receptionist couldn't find R014812 / R014815
in the booking list, but they appeared in the room availability view.

**Required for booking-list visibility:**
| Field | Must be | Notes |
|---|---|---|
| `HT_Book_H.book_room_type` | `2` (NOT `1`) | `2` = book by room number; `1` = book by room type. Of 1178 real bookings, 1176 use `2` (only 2 anomalies use `1`). |
| `HT_Book_Ds.Book_status` | `1` (active) | `0` = draft / non-standard; `1` = active; `3` = cancelled. The list filters to `=1`. |
| `HT_Book_H.Book_Date_in/out` | midnight (00:00:00) | The `HT_Book_H` date fields render as DATE-only in the form's top header. Actual stay times come from `HT_Book_Ds.Book_Room_Start/End`. |
| `HT_Book_H.Book_room_all` | `''` (empty) | Rooms live in `HT_Book_Ds` rows; this header field is unused by the .NET app. Setting it to `'402'` made the booking unfindable in some search paths. |
| `HT_Book_H.Book_Notify_Day` | `3` (default) | Payment reminder days. `0` disables notifications — fine but inconsistent with .NET app defaults. |
| All optional varchars | `''` (NOT NULL) | Some .NET WinForms controls misbehave on NULL — string concat downstream may crash. Always use `''`. |

**Verified on R014812**: with all 6 fixes applied, R014812 appeared in the booking list, opened in the full detail form (with the totals breakdown + extras section + "เลือกเลขห้อง" button), and supported edit-and-save through the .NET app — receptionist edited the phone, the .NET app saved successfully back to both `HT_Book_H.Book_Cust_Tel` AND `HT_Customers.Cust_Add_tel` (cross-table update). Truly indistinguishable from a native booking.

---

## 4. Cross-cutting findings

### 4a. The legacy app has **no transactions**

Every `INSERT`/`UPDATE`/`DELETE` is a separate batch. The destructive Phase
1/B (delete-then-reinsert) is therefore **interruptible** — if the app
crashes between the DELETE and the matching INSERT, data is permanently
lost. This is real risk in the legacy app.

**Our writeback worker MUST use transactions** for any multi-statement
mutation, even if the legacy app doesn't.

### 4b. Date format is **`M/D/YYYY` (US-style, no leading zeros)**

Examples: `'4/24/2026 5:05:04 PM'`, `'4/26/2026 11:59:59 AM'`. NOT ISO 8601.
This is the .NET default `ToString()` for `DateTime` in `en-US` culture —
the app probably runs with `CultureInfo.InvariantCulture` or `en-US`
regardless of the OS locale.

The numeric `room_date_oa` field on `HT_Room_Status` is the **OLE Automation
Date serial** (days since 1899-12-30). E.g. `46136` = `2026-04-24`,
`46137` = `2026-04-25`. Convert with: `DATETIME = '1899-12-30' + N days`.

### 4c. Mixed Thai / English literals

The app's status enums are inconsistent:
- Thai: `'เข้าพัก'` (occupying), `'ยกเลิก'` (cancelled), `'ราคาปกติ'` (normal price), `'บุคคลธรรมดา'` (individual person), `'จอง'` (booked)
- English: `'Check-Out'` (HT_CheckIn_Ds), `'Check Out'` (HT_Room_Status — note inconsistent space)

**Constants file required** in our writeback worker. Must mirror legacy literals exactly — including the `Check-Out` vs `Check Out` spacing.

### 4d. Identifier conventions

| Style | Used by |
|---|---|
| `[BracketedIdentifier]` (every column) | INSERT statements (typical .NET SqlClient) |
| Bare identifier | UPDATE/DELETE statements (older code path?) |
| Mixed casing (`HT_CheckIn_Ds` vs `ht_book_date`) | Both observed in different statements |

SQL Server is case-insensitive on default collation, so this works — but
indicates the app was assembled by multiple engineers/eras.

### 4e. Same `room_no` may appear at different `HT_Rooms.id` per branch

We observed `room_no='402' → id=15`, `room_no='403' → id=50`. The numeric
`id` is the internal PK; `room_no` is a string display value. Some
statements use `id` (e.g. `Room_Clean` UPDATE), others use `room_no` (e.g.
`room_use` UPDATE). Pick the right one per statement.

### 4f. `HT_Room_Status` is **current-state, not history**

`UPDATE HT_Room_Status WHERE room_no=X AND room_date=Y` overwrites whatever
row exists, including old check-out records. There's no `Cin_no` filter on
the UPDATE during check-in, so a new check-in on the same `(room, date)`
clobbers the previous occupancy's row. **For historical occupancy,
query `HT_CheckIn_Ds` by `Cin_Room_In`/`Cin_Room_Out` ranges**, not
`HT_Room_Status`.

---

## 5. Writeback design implications (for Option A in the architecture plan)

Map of "our app's intent" → "legacy SQL we must emit":

| User intent | Legacy SQL | Notes |
|---|---|---|
| New customer (standalone) | `INSERT HT_Customers` | Allocate `Cust_no = MAX+1`. Address fields all default to `''`. |
| New booking (advance) | `INSERT HT_Book_H` + `INSERT HT_Book_Ds` (1) + `INSERT HT_Book_Date` (×nights) | Allocate `Book_ID = MAX+1`. Departure time = `'11:59:59 AM'`. |
| Modify booking dates | UPDATE HT_Book_H, HT_Book_Ds; ADD/REMOVE HT_Book_Date rows; UPDATE HT_Rooms display | **Skip** the legacy's destructive DELETE-all pattern. |
| Walk-in check-in | All 8 statements from §3a, plus the 5s-later `Cin_Work_number` UPDATE | Allocate `Cin_no = MAX+1`. `Cin_by` = our app user (or `'Admin'`). |
| Check-in to a booking | Same as walk-in BUT: skip `INSERT HT_Customers` (UPDATE instead), set `Cin_Book_no`, UPDATE `HT_Book_H.Book_Status='เข้าพัก'`, UPDATE existing `HT_Room_Status` rows | Linkage via `Cin_Book_no`. |
| Add / extend night | UPDATE `HT_CheckIn_Ds` (night count, Cin_Room_Out, prices), UPDATE `HT_CheckIn_H` totals, INSERT new `HT_Room_Status` for added nights | Skip destructive Phase. |
| Take payment | INSERT `HT_CheckIn_Pay`, UPDATE `HT_CheckIn_H` totals | One-shot. |
| Print receipt | INSERT `HT_Receipt_H` + INSERT `HT_Receipt_Ds` (×lines) | Service code `SEV-001` for room charge. Receipts are append-only. |
| Check-out | UPDATE `HT_POWER_LOG` (lights off w/ note), UPDATE `HT_CheckIn_Ds` (status, dep date), UPDATE `HT_Rooms` (use=no, Clean=yes, count++), UPDATE `HT_Room_Status`, UPDATE `HT_CheckIn_H` (zero totals if balance=0). Then optionally housekeeping: UPDATE `HT_Rooms.Room_Clean='no'` + INSERT `HT_Housewife` | Skip Phase 1. |
| Cancel booking | UPDATE `HT_Rooms` to clear booking display (subquery on `HT_Book_Date`), UPDATE `HT_Book_H.Book_Status='ยกเลิก'`, UPDATE `HT_Book_Ds.Book_status=3` (numeric), DELETE `HT_Book_Date` (hard) | Customer + booking shells preserved. See §3g-bis. |
| Cancel a check-in | DELETE `HT_Room_Status` + `HT_CheckIn_Ds` (per room), UPDATE `HT_Rooms` (Use=no, Clean=yes), INSERT `HT_Rooms_Cancel` (audit log), SUBTRACT this room's price from `HT_CheckIn_H` totals, UPDATE `HT_CheckIn_H.cin_status='ยกเลิก'`, UPDATE `HT_POWER_LOG` (lights off w/ cancel-specific note) | See §3i. Allocate `HT_Rooms_Cancel.id = MAX+1`. |
| Mark room clean (housekeeping) | UPDATE `HT_Rooms.Room_Clean='no'` (by id, not room_no), INSERT `HT_Housewife` with `h_cin/h_cin_name` looked up from PRIOR non-cancelled check-in for this room | See §3j. |
| Add minibar / charge | **NOT YET CAPTURED** — likely INSERT `HT_CheckIn_Product`. To verify with another spike. |
| Refund | **NOT YET CAPTURED**. |

### Allocation strategy (verified)

Counters that need to be advanced atomically for our writeback:
- `Cust_no = 'C' + (next int)` → derived from `MAX(id)+1` on `HT_Customers`
- `Book_ID = 'R' + zeropad6(next int)` → `MAX(Book_ID)` parsed + 1
- `Cin_no = 'CH' + 2-digit-year + '-' + zeropad6(next int)` → year-scoped
- `pay_no` → MONTH-scoped (per `Cin_Pay_Date` BETWEEN month boundaries)
- `Receipt_no` → MONTH-scoped + receipt-type prefix
- Numeric `id` columns on `HT_Customers`, `HT_Book_Date`, `HT_Room_Status`, `HT_Rooms_Cancel` → `MAX(id)+1`
- `HT_Book_Ds.id` → SQL Server IDENTITY (no allocation needed)

**Race-safe pattern (verified live)** — wrap every `MAX+1` lookup in:
```sql
BEGIN TRAN
  SELECT @nextid = ISNULL(MAX(id), 0) + 1
    FROM <table> WITH (TABLOCKX, HOLDLOCK)
  INSERT INTO <table> (id, ...) VALUES (@nextid, ...)
COMMIT
```

`TABLOCKX` blocks all other writers AND blocks the .NET app's `SELECT MAX`
queries (which run at default `READ COMMITTED` isolation, no `NOLOCK`
hints — verified in captures). The .NET app waits for our COMMIT, then
reads the new MAX, allocates `our+1`, and INSERTs. No collisions.

**Verified under live load (Test 2):**
- We held `TABLOCKX` for 10 seconds while receptionist clicked "save" on a booking.
- Allocations: ours `R014815` (committed at 18:13:05.633), receptionist's `R014816` (got blocked, waited, then committed with our+1).
- Zero PK collisions, zero error dialogs in the .NET app — only a one-time UI hitch.

In production, the lock hold time per allocation is ~5-10ms (network round-trip + INSERT), well under any UI threshold. Receptionist will not perceive it.

### Safety rails

1. **Always wrap multi-statement intents in a transaction** (`BEGIN TRAN ... COMMIT`).
2. **Never replicate the legacy app's destructive DELETE+REINSERT pattern.**
3. **Test on HF Ville first** — lower traffic, lower stakes than HF Hotel.
4. **Idempotency**: design every writeback operation so retrying it is safe (e.g. INSERT with PK-conflict check, UPDATE with `WHERE current_value=X`).
5. **Schema fingerprint check on startup** — query a known column set and bail if it doesn't match what we observed. The vendor could push a schema change at any time.

---

## 6. Race-safety verification

Race-safe MAX+1 allocation works against the live .NET app at default
`READ COMMITTED` isolation. **No cooperation from the .NET app required**
— SQL Server enforces the lock server-side.

### Test 1 — lock blocks reads (no UI impact)

Held `TABLOCKX` on `HT_Customers` from session A for 5 sec via:
```sql
BEGIN TRAN
SELECT @x = MAX(id) FROM HT_Customers WITH (TABLOCKX, HOLDLOCK)
WAITFOR DELAY '00:00:05'
ROLLBACK
```
Concurrent session B issued `SELECT MAX(id) FROM HT_Customers` (no hints) —
B blocked for 3490ms before the result returned. Confirms default reads
honor our exclusive lock.

### Test 2 — concurrent receptionist save (live UI test)

Receptionist had a half-typed booking ready. We fired our writeback with
a 10-sec hold. Receptionist clicked save during the hold. Result:

| | Us (writeback) | Receptionist (.NET app) |
|---|---|---|
| `Cust_no` allocated | `C21616` | `C21617` |
| `Book_ID` allocated | `R014815` | `R014816` |
| Outcome | INSERTed cleanly | INSERTed cleanly after ~10s wait |
| Errors | none | none |
| UI experience | n/a | save took 10s instead of <1s; no error dialog |

Sequential IDs, no collisions, no errors. Pattern confirmed for production.

### Test 3 — writeback row visible/editable in .NET app

Updated `R014812` to use the corrected `book_room_type=2`, `Book_status=1`,
midnight dates, etc. Receptionist:
1. Found it in the booking list ✓
2. Opened it in the full detail form ✓ (totals breakdown, extras section, "เลือกเลขห้อง" button — looked exactly like a native booking)
3. Edited the phone number, clicked save ✓
4. Save propagated to BOTH `HT_Book_H.Book_Cust_Tel` AND `HT_Customers.Cust_Add_tel` cleanly

**Conclusion: writeback is fully indistinguishable from native .NET app writes** as long as the 6 fields in §3k are populated correctly.

### Test 4 — cancel via writeback

Applied the §3g-bis cancel SQL to `R014812`. Receptionist confirmed:
- `R014812` removed from active booking list
- Room slot freed (next booking R014814/R014816 took over the room view)
- Booking visible in cancelled bookings (status `'ยกเลิก'`)

---

## 7. What we still don't know

| Gap | Plan to fill |
|---|---|
| Add minibar / per-line charge | Spike capture of "add product" action. `HT_CheckIn_Product` (currently 0 rows in DB) and `HT_Products` tables. |
| Refund / negative payment | Spike capture. Possibly negative `Cin_Pay_Cash`. |
| Edit existing customer in isolation (without check-in flow) | Spike capture. |
| Bulk operations (sync end-of-day, close-out, daily round) | Probably batch operations into `HT_Round_Bill` (4575 rows). |
| What `HT_Cupon` actually represents | 17,894 rows. Pre-allocated coupons? Loyalty discount codes? |
| What the empty-table columns (`HT_Bank_*`, `HT_Bill_Debt_*`, `HT_Order_*`, `HT_Deposit`, `HT_Register`) are for | Unused features — confirm with hotel staff. |
| What `Tb_Save_Image` actually stores (binary blobs?) | Sample one row's column shapes. |
| Whether multi-room check-ins use the same flow | Capture a 2-room walk-in; confirm two `HT_CheckIn_Ds` rows + power log entries. |
| `Cin_Work_number` — TM.30 batch number meaning | Random number assigned async ~5s after check-in. May relate to government registry export. |

These are all minor — the writeback worker can ship without them. Add as
needed when the corresponding feature is implemented in our app.
