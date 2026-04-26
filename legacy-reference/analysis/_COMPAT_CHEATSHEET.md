# Hotel-2018 V.1.45 - Coexistence Compatibility Cheatsheet

> **Purpose**: This is the **contract document** that the new (rewritten) hotel-management app
> must follow so that anything it writes to the shared SQL Server DB is read/processed
> correctly by the still-running legacy VB.NET WinForms app, and vice versa.
>
> **Source of truth**:
> - Cleaned decompile: `C:\Users\nutok\Downloads\Hotel-2018- V.1.45\_decompiled_clean\iHOTEL2025\`
> - Schema: `C:\Users\nutok\Downloads\Hotel-2018- V.1.45\_decompiled\_SCHEMA.sql`
> - Live DB queried via `sqlcmd -S . -E -d db`
>
> **Treat every literal value in this document as load-bearing** - the old app does pure
> string-equality comparisons against these literals (the collation `Thai_CI_AS` is
> case-insensitive but accent-sensitive).

---

## 1. Global Conventions

### 1.1 Database engine and connection

- **Engine**: SQL Server (any modern version). Old app supports MS Access fallback but the live DB is SQL Server.
- **Database name**: `db` (yes, literally — see `Module1.ReadDB_2018`).
- **Collation**: `Thai_CI_AS` on text columns (Windows-874 / TIS-620 codepage, **NOT Unicode**).
  Every `varchar` column storing Thai text uses this codepage. The new app MUST send Thai text
  with the correct codepage conversion, or use bytes that match TIS-620 byte-for-byte.
  Sending Unicode `N'…'` literals will silently corrupt to `?` because columns are `varchar` not `nvarchar`.
- **Identifier case**: SQL Server is case-insensitive on identifiers by default. The old app
  freely mixes `HT_Customers`/`HT_customers`, `Cin_no`/`cin_no`, `tb_pay_history`/`TB_Pay_History`.
  The new app MUST use the same case-folding tolerance — DO NOT rely on case-sensitive collation.
- **Triggers**: There are **zero** SQL triggers (`SELECT name FROM sys.triggers` returned 0 rows).
  All cascade behavior is in app code. Therefore the new app must replicate every cascade.
- **Foreign keys**: The schema has no FK constraints. Referential integrity is purely
  by-convention.

### 1.2 SQL execution pattern

All writes go through `Module1.connect(string sql) -> DataSet`:
- Single-statement, single-batch.
- **No parameters; pure string concatenation**. Therefore every literal value the app
  ever writes is grep-able verbatim in the source.
- No transactions wrap multi-statement cascades. The new app must accept that the old app
  can leave an incomplete cascade (e.g. partial check-in) if it crashes mid-way; reads
  should be defensive about orphans.
- The app DOES NOT escape single quotes in user input. Any `'` inside e.g. a customer name
  will produce a SQL syntax error and silently lose the write. **One exception**: notes
  written via `INV_Note.cs`, `Room_Note.cs`, `EMP_Note.cs` do `RichTextBox1.Text.Replace("'", "\"")`
  before insert. The new app MUST do the same `'` → `"` substitution on note fields, or
  better: properly escape with `''` doubling. The old app will read `''`-escaped strings fine.

### 1.3 Empty-string vs NULL

- **Empty string `''` is the canonical "absent" marker** for `varchar` flag/status columns.
  The old app reads with `where col=''` and writes `''` literals, NOT `NULL`.
- Live data confirms: `Room_Clean`, `Room_Use`, `Room_Manternace`, `Cin_status`, `Bill_Status`,
  `room_status`, `Book_Status` are never NULL — defaults are `'no'` or `''`.
- **However**: many columns ARE nullable in schema. The old app sometimes writes NULL implicitly
  by omitting the column from INSERT. Reads are typically `IS NULL OR =''` permissive. The
  new app should default nullable status/flag columns to `''` not NULL.
- `Book_Notify_Note` is one column where the old app writes the **literal string `'ไม่แจ้งเตือน'`**
  to suppress notification (see `FrmShowBookNotify.cs:1118`). NULL means "still notify".

### 1.4 Date/time handling

- **Most date columns are SQL `datetime` type**. The app writes them as VB.NET-formatted
  strings via `Conversions.ToString(DateTime.Now)` which uses the current OS locale (typically
  Thai locale → e.g. `27/2/2025 14:30:00` in Thai short-date format), wrapped in single
  quotes: `'27/2/2025 14:30:00'`.
- **CRITICAL**: SQL Server may parse `dd/mm/yyyy` differently from `mm/dd/yyyy` based on the
  session's `DATEFORMAT` and `LANGUAGE`. The old app relies on the SQL Server connection
  picking up Thai language settings to interpret these strings correctly. The new app
  MUST either (a) always use unambiguous ISO `YYYY-MM-DD HH:MM:SS` (which SQL Server
  parses unambiguously regardless of language), or (b) `SET LANGUAGE Thai` on each connection.
  Recommendation: write ISO format. Reads work fine because reads come back as `datetime` objects.
- Date-range queries use `'M/D/YYYY 00:00:00'` US format (see `GET_DOC`, `GetSIR_PAY` etc.):
  e.g. `Cin_Pay_Date between '2/01/2025 00:00:00' and '2/28/2025 23:59:59'`. With SQL Server
  default English language this works; under Thai language it could fail. Live data shows
  it works on this install, so DATEFORMAT is set to MDY at session level somewhere.
- A small number of columns are **string-typed dates**:
  - `HT_Log.DODATE` is `varchar(50)` — written as `Strings.Format(DateTime.Now, "dd/MM/yyyy HH:mm:ss")`.
  - `TB_FOLIO.F_IN`, `F_OUT`, `F_NIGHT`, `F_PRICE`, `F_PRICE_TOTAL` are `varchar(50)` —
    written as plain `Conversions.ToString(...)` (locale-dependent).
- A `float`-typed "OADate" date pattern is used:
  - `TB_Pay_History.Pay_Date` is `float` (see `FrmAddPay.cs:638`):
    `INSERT INTO TB_Pay_History VALUES(..., DateTimePicker1.Value.ToOADate(), ...)`.
    OADate = days since 1899-12-30 as a double. Reports convert back via `FromOADate`.
  - `HT_Room_Status.room_date_oa` is `float` and stores `DateTime.Date.ToOADate()` for that row's
    `room_date`. Likely a redundant copy used to dodge SQL date-parsing in a report.
  - `HT_Rooms.Room_Clean_Time` is `varchar(30)` storing `DateTime.Now.ToOADate()` as a string
    (see `ClickClean.cs:822`, `ClickCleanOK.cs:822`). Cleared with `''`.
- **The new app must mirror these formats exactly** for any column the old app reads.

### 1.5 Boolean conventions

There is no single boolean convention. Different columns use different representations:
- `Room_Clean`, `Room_Use`, `Room_Manternace` (HT_Rooms) → `'yes'` / `'no'` (lowercase, varchar).
- `Room_Power_STATUS` (HT_Rooms) → `'on'` / `'off'` (lowercase, varchar). Default `'off'`.
- `SMS_Readed` (HT_Room_SMS, HT_EMP_SMS) → `'yes'` / `'no'` (lowercase, varchar).
- `Cin_foreign` (HT_CheckIn_H) → `'True'` / `'False'` (capitalized, VB.NET boolean ToString).
- `Receipt_VatIn` (HT_Receipt_H, HT_Invoice_H) → `'True'` / `'False'`.
- `Book_USE`, `Book_ok` (HT_Book_Date) → `int` 0/1.
- `Book_status` (HT_Book_Ds) → `int` 1=normal, 3=cancelled (set by FrmShowBookNotify cancel).
- `Book_room_type` (HT_Book_H) → `int` 1=no-specific-rooms (FrmAddBook), 2=with-specific-rooms (FrmAddBook2).
- `cupon_print` (HT_Cupon) → `int` 0=unprinted, 1=printed.
- `Cin_cupon` (HT_CheckIn_Ds) → `int` count of cupon entitlement per night.
- `Cin_Pay_Status` (HT_CheckIn_Pay) → `varchar` `'1'` (active) or `'ยกเลิก'` (cancelled).
  Default value `((1))` per schema means literal int but column is varchar — works because
  `'1'::varchar = 1::int` doesn't apply but `(1)` default in MSSQL on varchar gets stringified to `'1'`.

### 1.6 ID generation patterns

The schema has 31 IDENTITY columns (DB-managed) and 31 non-IDENTITY tables where the app
generates the PK itself. There are **6 distinct patterns**:

1. **`get_id(table, column)` = MAX(column)+1** (most common, see `Module1.get_id`):
   ```sql
   SELECT MAX([Column]) FROM [Table]
   ```
   Returns 1 if empty. **Race-prone**: two clients calling `get_id` concurrently get the same
   value and both succeed (no UNIQUE constraint on most of these "id" columns).
   Tables: `HT_Cupon.cupon_no`, `HT_Rooms_Cancel.id`, `HT_Customers.id`, `HT_Deposit.id`,
   `HT_INVOICE.INV_NO`, `HT_Receipt_H.id`, `HT_Invoice_H.id`, `HT_Book_Date.id`,
   `HT_Room_Status.id`, `HT_Round_Bill.id`, `HT_Order_Up.id`/`HT_Order_Down.id` (per-type),
   `TB_FOLIO.id`, `HT_Rooms.id`, `HT_Products.id`.

2. **Date-prefixed string with monthly counter** — used for human-facing receipt-like numbers:
   - `HT_CheckIn_H.Cin_no`: `CH{yy}-{6digit}` per **year**, max+1 within `cin_date BETWEEN
     '1/1/{year}' AND '12/31/{year} 23:59:59'`. (see `FrmCheckIn.GET_DOC`).
   - `HT_CheckIn_Pay.Pay_no`: `R{yyMM}-{4digit}` per **month**. (`Module1.GetSIR_PAY`).
   - `HT_Receipt_H.Receipt_no`: `B{yyMM}-{4digit}` (B-prefix), `SB{yyMM}-{4digit}` (SmallBill),
     or `CB{yyMM}-{4digit}` (CreditBill) per month. Selected by `ComboType` in FrmAddSale.
   - `HT_Invoice_H.Receipt_no`: same `B/SB/CB` scheme as Receipt (FrmAddInvoiceSale.GetSIR).
   - `HT_Bill_Debt_H.Bill_No`: `B{yyMM}-{4digit}` per month (FrmAddSale2_Credit.GetSIR).
   - `HT_Deposit.Dep_no`: `DEP{yyMM}-{4digit}` per month (FrmAddDep.GetSIR).

3. **Customer ID** `HT_Customers.Cust_no`: `C{4digit}` derived from `MAX(id)+1` (see
   `FrmCheckIn.SAVE_CUST`, `FrmAddBook2.SAVE_CUST`):
   ```sql
   SELECT TOP 1 * FROM HT_Customers ORDER BY id DESC  -- read row's id, +1
   ```
   Same race condition as get_id. Plus the `id` column is also written explicitly with
   `get_id("HT_Customers", "id")`. The two are computed independently in the same insert
   so they should match unless another insert lands between the two SELECTs.

4. **Booking ID** `HT_Book_H.Book_ID`: `R{6digit}` from `MAX(replace("R","")) + 1` over the
   whole `HT_Book_H` table (FrmAddBook/FrmAddBook2.GET_DOC). Race-prone.

5. **Product code** `HT_Products.Pro_no`: `{ProType.id_full}-{3digit}` derived from
   `SELECT TOP 1 * FROM HT_Products WHERE Pro_Type=… ORDER BY pro_no DESC`, +1.
   Format depends on ProductType. Sentinel: `'P001'` is reserved for "room rent line item"
   (used by `Insert_Pay` when there is no real product).

6. **App-provided positional/scoped sequence**:
   - `HT_Order_Up`/`HT_Order_Down.id`: just `num2 + 1` row-index (safe because the writer
     does `DELETE WHERE Cast_Type='X'` first and rewrites all rows for that customer type).
   - `HT_Bill_Debt_Ds.DS_ID`: positional row index `num2 + 1` within one bill.
   - `HT_Invoice_Ds.S_Sale_id` / `HT_Receipt_Ds.S_Sale_id`: copy of parent
     `HT_Invoice_H.id` / `HT_Receipt_H.id` — denormalized FK, not an auto-increment.

**For the new app**: Treat #1 (MAX+1) and #2 (date-counter) as inherently race-prone.
Recommended approaches in priority order:
- (a) Wrap each ID generation in a `SERIALIZABLE` transaction with `WITH (UPDLOCK, HOLDLOCK)`
  on the parent table.
- (b) Or use a counter table. There is no existing counter table.
- (c) Or use SQL Server `SEQUENCE` objects but the old app can't read them, so only safe
  for tables the new app writes alone.
- (d) Retry on duplicate-detection (re-read MAX, increment, try again, up to N retries).
  The old app's `Insert_Pay` has a re-read loop pattern that approximates this — see
  `Module1.cs:1812` "ไม่สามารถบันทึกการจ่าย…กด OK เพิ่มลองใหม่อีกครั้ง" → loop.

### 1.7 SQL injection / quoting

Old app does **no escaping**. Single quotes in user input WILL break inserts/updates with
SQL syntax error and the write will silently fail (the user sees a blank message box).
Known mitigations in old code:
- `INV_Note.cs`, `Room_Note.cs`, `EMP_Note.cs` do `RichTextBox1.Text.Replace("'", "\"")` —
  i.e. they replace `'` with `"` (double quote, ASCII 34).
- No other field has any escaping.

For coexistence: the new app should escape `'` as `''` (SQL standard) on all fields. The
old app reading `''`-escaped strings will see them as a single `'` correctly. **However**,
a customer name with `"` in the new app must NOT be converted to `'` because the old app
writes `"` and reads it back fine.

### 1.8 Thai-language data and number-to-text

- All text columns are `varchar` with collation `Thai_CI_AS`. Storage codepage is
  Windows-874/TIS-620. Sending UTF-8 directly will corrupt to `?`.
  - **In SQL Server T-SQL**: use plain `'…'` literals with the connection's codepage.
    The .NET SqlClient driver will convert from CLR `string` to TIS-620 automatically when
    the target column is `varchar` — provided you pass `string` parameter.
  - **DO NOT** use `N'…'` with varchar columns; that triggers nvarchar-to-varchar conversion
    that fails on characters outside TIS-620.
- `DecimalToText_TH.cs` converts numbers to Thai words (`123 → "หนึ่งร้อยยี่สิบสามบาทถ้วน"`)
  for receipt printing. It's pure code (not in DB). The new app needs equivalent logic if
  it generates Thai receipts.

### 1.9 Round-bill gate

- Every transactional handler (sale, check-in, check-out, payment) calls
  `Module1.check_round_bill()`:
  ```sql
  SELECT id FROM HT_Round_Bill WHERE round_end IS NULL
  ```
  If 0 rows → blocks the operation. The new app should respect this: do not write
  receipts/check-ins/payments while no open round exists. Or, if the new app opens its own
  round, ensure the old app sees `round_end IS NULL`.

---

## 2. Per-Table Contract

> Notation:
> - **(A)** = active (live data, code reads & writes)
> - **(R-only)** = active, read-only (master data the new app should not write)
> - **(D)** = deprecated / dead / legacy table; new app should ignore.
> - **PK Convention** lists who owns the primary key.

### Dead tables (skip these)

The following tables exist in schema but should be **ignored by the new app** (the old app
either never reads them, or reads them only from one DEAD form):

| Table | Reason |
|---|---|
| `HT_Book_H2` | Only read by `FormSearchBooking2` (deprecated). |
| `HT_Book_Ds2` | Companion to H2; not used. |
| `HT_Book_Date2` | Only `FormSearchRooms2_old` (DEAD). |
| `HT_Book_Status` | Schema present but not written by current code path; legacy. |
| `HT_Bank_Accounts`, `HT_Bank_Transfer` | Schema only; no code touches them. |
| `HT_Register` | Defined and read by `FrmAddReg`/`FrmRegMain`, but `FrmAddReg` never writes it. Legacy registration form, replaced by `Print_Report` reg printout. Treat as dead. |
| `TB_MRP_Permission_name` | Schema only; never read or written. |

> Verified by: `Grep "FROM HT_Bank_"` returns 0 hits; `Grep "FROM HT_Register"` 1 hit (read-only).

---

### Table: `HT_Rooms` (A)

- **Purpose**: Master table of physical rooms. Holds both static config (number, type,
  prices, layout x/y) and **denormalized dynamic state** (clean/dirty, in-use, current
  booking ref, current power state).
- **PK convention**: `id` is `int` (NOT IDENTITY), generated by `Module1.get_id("HT_Rooms","id")`
  on insert (FrmManageRoom). `Room_no` (varchar) is the human key and is queried as the
  business identifier in 99% of code (`WHERE Room_no='101'`).
- **Schema** (23 columns):
  ```
  id int, Room_no varchar(50), Room_Type varchar(50), Room_Details varchar(500),
  Room_PriceA float, Room_PriceB float, Room_PriceC float,
  Room_Clean varchar(50) NOT NULL DEFAULT 'no',
  Room_Use varchar(50) NOT NULL DEFAULT 'no',
  Room_Manternace varchar(50) NOT NULL DEFAULT 'no',
  Room_X int NOT NULL DEFAULT 0, Room_Y int NOT NULL DEFAULT 0,
  Room_Use_Count int NOT NULL DEFAULT 0,
  Room_Polity int NOT NULL DEFAULT 1,
  Room_Book varchar(50), Room_Book_Name varchar(250), Room_Book_Time varchar(50),
  Room_Group varchar(50),
  Room_Book_ds text,
  Room_Power_OPEN varchar(50), Room_Power_CLOSE varchar(50),
  Room_Power_STATUS varchar(50) NOT NULL DEFAULT 'off',
  Room_Clean_Time varchar(30)
  ```
- **Status-flag enumeration**:
  - `Room_Clean`: `'yes'` (clean, ready), `'no'` (dirty / freshly checked-out), default `'no'`.
  - `Room_Use`: `'yes'` (currently occupied), `'no'` (free or booked-but-not-checked-in),
    default `'no'`.
  - `Room_Manternace`: `'yes'` (under repair, not rentable), `'no'` (default).
  - `Room_Power_STATUS`: `'on'` / `'off'`, default `'off'`. Always lowercase.
  - `Room_Book` (varchar): the `HT_Book_Date.id` of the most recent active booking-day for this room,
    or `''` if not booked. NB: this is **just the integer id stored as varchar** because the
    code does `update HT_Rooms set Room_Book='' where ...` (writes empty string, not NULL).
  - `Room_Book_Name`: copy of `HT_Customers.Cust_name` for the booker (display on grid). `''` when free.
  - `Room_Book_Time`: copy of booking start datetime as string (display only). `''` when free.
  - `Room_Book_ds`: legacy free-text booking description; cleared with `''`.
  - `Room_Clean_Time`: OADate string of when housekeeping last marked the room clean. `''` to clear.
- **Operations the old app performs**:
  - **Mark clean** (ClickAvliable / ClickCleanOK):
    `update HT_Rooms set Room_Clean='yes' where id=<id>`
    `update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=<id>` (variant)
  - **Mark dirty** (ClickClean.ButtonX3):
    `update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=<id>` + INSERT HT_Housewife row.
  - **Send to maintenance** (ClickAvliable.ButtonX6 / ClickClean.ButtonX6):
    `update HT_Rooms set Room_Manternace='yes' where id=<id>` + INSERT HT_Rooms_Repair.
  - **Return from maintenance** (ClickManternance.ButtonX...):
    `update HT_Rooms set Room_Clean='no',Room_Manternace='no' where id=<id>`.
  - **Mark in use** (FrmCheckIn save):
    `update HT_Rooms set room_use='yes' where room_no='<room>'`.
  - **Mark free** (FrmCheckOut, room is now dirty):
    `update HT_Rooms set room_use='no',Room_Clean='yes',Room_Use_Count=Room_Use_Count+<nights> where room_no='<room>'`.
    NB: setting `Room_Clean='yes'` here means "ready" but at the same time `frmMain1` startup
    queries treat `Room_Use='no'` as the canonical "this room is free/dirty"; reading is
    the AND of conditions.
  - **Reverse a check-out** (FrmCheckOut, mode 2):
    `update HT_Rooms set room_use='yes',Room_Clean='no',Room_Use_Count=Room_Use_Count-<nights> where room_no='<room>'`.
  - **Set booking pointers** (FrmAddBook2, FormRoomMain):
    `update HT_Rooms set room_book=<HT_Book_Date.id>, room_book_name='<Cust_name>', room_book_time='<dt>', room_book_ds='<descr>' where ...`
  - **Clear booking pointers** (ClickBook cancel, FrmCheckIn upon converting booking to checkin):
    `update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' where id=<id>` (or `where Room_Book in (select id from HT_Book_Date where Book_no='<bookid>')`).
  - **Power state** (Module1.Power_set):
    `update HT_Rooms set Room_Power_STATUS='on' where room_no='<room>'`
    `update HT_Rooms set Room_Power_STATUS='off' where room_no='<room>'` (always lowercase).
  - **Update grid layout** (FormRoomMain drag/drop):
    `update HT_Rooms set Room_X=<x>,Room_y=<y> where Room_no='<room>'`.
  - **Insert (FrmManageRoom)**: explicit columns including `[id]` from `get_id`,
    `Room_Power_STATUS` defaulted to `'off'`. Empty string for unset varchar fields.
- **Implicit invariants** (what other forms ASSUME when reading):
  - `Room_Use='yes'` ⇒ a row exists in `HT_CheckIn_Ds` for `Room_no` with
    `Cin_Room_Status<>'Check-Out'`. `frmMain1` startup *fixes* the inverse via:
    `update HT_Room_Status set room_status='Check-Out' WHERE room_no IN (SELECT Room_no FROM HT_Rooms WHERE Room_Use='no') AND room_status='เข้าพัก'`.
  - `Room_Book<>''` ⇒ a row exists in `HT_Book_Date` with that `id`.
  - `Room_Clean='yes' AND Room_Use='no' AND Room_Manternace='no'` ⇒ rentable.
  - The grid in `FormRoomMain` resolves color/state by reading these three flags only.
- **Cleanup/reset operations**:
  - "Factory reset" button (frmMain1.cs:7553): `update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='',Room_Use_Count=0,Room_Manternace='no',Room_Use='no',Room_Clean='no'`.

---

### Table: `HT_Room_Status` (A)

- **Purpose**: Per-room-per-day occupancy ledger. One row per (room_no, room_date) pair when
  the room is reserved or in-use on that calendar date. Used to render the booking grid view.
- **PK convention**: `id` is `int` (NOT IDENTITY), via `get_id("HT_Room_Status","id")`.
  Note: `HT_Book_Date.id` and `HT_Room_Status.id` are independent counters.
- **Schema** (8 columns):
  ```
  id int, room_no varchar(50), room_date datetime, room_status varchar(50),
  room_Details varchar(500), room_Book_No varchar(50),
  room_CheckIn_No varchar(50), room_date_oa float NOT NULL DEFAULT 0
  ```
- **Status-flag enumeration** (`room_status`):
  - `'จอง'` (Thai: "booked") — set by FrmBookRooms / ClickBook when reservation created.
  - `'เข้าพัก'` (Thai: "in stay") — set by FrmCheckIn when guest arrives.
  - `'Check-Out'` — set by ClickUSE.cs:1116 (with hyphen) and frmMain1.cs:7072 (auto-fix).
  - `'Check Out'` — **(without hyphen)** set by FrmCheckOut.cs:6246 (`update HT_Room_Status SET room_status='Check Out'`).
    **This is a known inconsistency**. Live data only contains `'Check-Out'` because frmMain1
    startup deletes all `room_status='Check-Out'` rows (with hyphen) on every launch; the
    `'Check Out'` (space) writes are rapidly cleaned up before they accumulate. The new app
    should write `'Check-Out'` (with hyphen) for forward compatibility, AND must tolerate
    reading both spellings.
  - `'เช่า'` — only referenced in dead code path `FormSelectRoom.RefreshStatus`; not written.
- **Other columns**:
  - `room_Details`: customer name (denormalized).
  - `room_Book_No`: `HT_Book_H.Book_ID` if booking; `NULL` otherwise.
  - `room_CheckIn_No`: `HT_CheckIn_H.Cin_no` if checked in; `NULL` otherwise.
  - `room_date_oa`: `room_date.Date.ToOADate()` as float (redundant copy for a Crystal Report).
- **Operations**:
  - **Insert booking day**: id from get_id, status='จอง', room_Book_No=Book_ID.
  - **Insert checkin day** (or upgrade booking row): if existing row found by (room_date,room_no),
    UPDATE: `room_status='เข้าพัก', room_Details=<custname>, room_CheckIn_No=<cin>`. Else INSERT.
  - **On check-out** (per FrmCheckOut path): `update HT_Room_Status SET room_status='Check Out' where room_no='<r>' and room_CheckIn_No='<cin>'`. Note: writes `'Check Out'`.
  - **Cleanup on edit-booking**: `delete from HT_Room_Status where room_Book_No='<bookid>'` then re-insert (FrmBookRooms.SAVE_EDIT).
  - **Cleanup on edit-checkin** (FrmCheckIn delete-then-reinsert): `delete from HT_Room_Status where room_CheckIn_No='<cin>'`.
  - **Startup auto-clean** (frmMain1.cs:7070-7072):
    `DELETE FROM HT_Room_Status WHERE room_status='Check-Out'`
    `update HT_Room_Status set room_status='Check-Out' WHERE (room_no IN (SELECT Room_no FROM HT_Rooms WHERE Room_Use='no')) AND (room_status='เข้าพัก')`
- **Invariants**:
  - One row per (room_no, room_date.Date) is the ideal. Code uses `where room_date='<date>'` so the
    time portion must be midnight (00:00). The new app must store the date with no time component
    (e.g. `2025-02-27 00:00:00.000`).
  - `room_status='Check-Out'` rows are ephemeral (deleted on next app start).

---

### Table: `HT_CheckIn_H` (A) — header per check-in folio

- **Purpose**: One row per check-in document (folio header).
- **PK convention**: `Cin_no varchar(50) NOT NULL` (the only column NOT-NULL). Format
  `CH{yy}-{6digit}` per year, generated by `FrmCheckIn.GET_DOC` reading
  `select top 1 from HT_CheckIn_H where cin_date between '1/1/{yyyy}' and '12/31/{yyyy} 23:59:59' order by Cin_no desc`.
- **Schema** (22 columns):
  ```
  Cin_no varchar(50) NOT NULL, Cin_Date datetime, Cin_Book_no varchar(50),
  Cin_cust_no varchar(50), Cin_cust_price varchar(50),
  Cin_Car_type varchar(50), Cin_Car_id varchar(50), Cin_status varchar(50),
  Total_Price_Room float NOT NULL DEFAULT 0,
  Total_Price_Product float NOT NULL DEFAULT 0,
  Total_Price_Net float NOT NULL DEFAULT 0,
  Total_Price_Pay float NOT NULL DEFAULT 0,
  Total_Price_Balance float NOT NULL DEFAULT 0,
  Cin_Room_ALL varchar(500),
  Total_Price_vat float NOT NULL DEFAULT 0,
  Cin_by varchar(100), Cin_Date_in datetime, Cin_Date_out datetime,
  Cin_type int NOT NULL DEFAULT 0,
  Cin_note varchar(250),
  Cin_foreign varchar(50) NOT NULL DEFAULT 'False',
  Cin_Work_number int NOT NULL DEFAULT 0
  ```
- **Status-flag enumeration**:
  - `Cin_status`: `'ปกติ'` (normal, default) or `'ยกเลิก'` (cancelled).
    Verified live: 14624 ปกติ + 167 ยกเลิก rows. NEVER `''`.
    Setter: ClickUSE.cs:1527 `update HT_CheckIn_H set cin_status='ยกเลิก' where cin_no='<cin>'`.
  - `Cin_foreign`: `'False'` (Thai national, default) or `'True'` (foreigner).
  - `Cin_type`: `0` = day-rate (default), `1` = hourly (เป็นรายชั่วโมง), `2` = monthly
    (รายเดือน). Verified live: all three values present.
- **Other columns**:
  - `Cin_Book_no`: source `HT_Book_H.Book_ID` if checked in from booking; `''` if walk-in.
  - `Cin_cust_no`: `HT_Customers.Cust_no` (e.g. `'C0001'`); `'C0000'` is reserved for
    "deleted/orphan customer" (cascading sentinel).
  - `Cin_cust_price`: customer-type code (e.g. `'A'`, `'B'`) at time of check-in (price tier).
  - `Cin_Car_type`/`Cin_Car_id`: vehicle info (free-text Thai).
  - `Cin_Room_ALL`: comma-or-space separated room numbers for display (`"101 102 "`).
  - `Cin_by`: `Module1.loginName` (employee).
  - `Cin_Date_in`/`Cin_Date_out`: planned check-in / check-out times. `Cin_Date` is when
    the checkin record was *created* (DateTime.Now).
  - `Total_Price_*`: aggregated totals; old app updates these on every payment/sale change.
- **Operations**:
  - **Insert** (FrmCheckIn save): full row, status='ปกติ', defaults zero for totals if walk-in
    pre-payment. See `FrmCheckIn.cs:9508-9550`.
  - **Update on save-edit**: full UPDATE statement, see FrmEditDate.cs:4880.
  - **Cancel** (ClickUSE): `update HT_CheckIn_H set cin_status='ยกเลิก' where cin_no='<cin>'`.
  - **Update VAT total** (FrmAddSale linking sale to checkin):
    `update HT_CheckIn_H set Total_Price_vat=Total_Price_vat+<n> where Cin_no='<cin>'`.
  - **Update money totals** at check-out: full UPDATE setting all `Total_Price_*` and `Cin_note`.
  - **Customer rename cascade** (FrmManageCustomersNew.cs:3651): on delete-customer,
    `update HT_CheckIn_H set Cin_cust_no='C0000' where Cin_cust_no='<delcust>'`.
  - **Delete-then-reinsert pattern** (FrmCheckIn edit, see FrmCheckIn.cs:9750):
    `delete from HT_CheckIn_H where Cin_no='<cin>'`. Note: the related Ds/Pay/Product cleanup
    is also done via DELETE.
- **Invariants**:
  - For every `HT_CheckIn_H` row with `cin_status='ปกติ'`, there should be ≥1
    `HT_CheckIn_Ds` row sharing `Cin_no` and ≥0 `HT_CheckIn_Pay` rows.
  - When `cin_status='ยกเลิก'`, the matching `HT_CheckIn_Pay` rows should also have
    `cin_status='ยกเลิก'` (cancellation cascades into Pay).

---

### Table: `HT_CheckIn_Ds` (A) — folio line per room per stay

- **Purpose**: Detail rows under `HT_CheckIn_H` — one row per **room** in the check-in (a
  single check-in can cover multiple rooms).
- **PK convention**: `id int NOT NULL` (NOT IDENTITY!). Generated by
  `get_id("HT_CheckIn_Ds","id")` AT THE START of the loop, then incremented by 1 per
  iteration with `num6++` — see FrmCheckIn.cs:9320. **Race-prone** for concurrent check-ins.
- **Schema** (19 columns):
  ```
  id int NOT NULL, Cin_No varchar(50), Cin_Room_No varchar(50), Cin_Room_Type varchar(50),
  Cin_Room_In datetime, Cin_Room_Out datetime,
  Cin_Room_Status varchar(500),
  Cin_Room_Dep float, Cin_Room_Price float, Cin_Room_Night float,
  Cin_Room_PriceToTal float, Cin_Room_Pay_Before float, Cin_Room_Pay_Total float,
  Cin_note varchar(500),
  Cin_Dep_Status varchar(50),
  Dep_by varchar(100),
  Cin_cupon int NOT NULL DEFAULT 0,
  Cin_Dep_return_date datetime, Cin_Dep_return_by varchar(50)
  ```
- **Status-flag enumeration** (`Cin_Room_Status`, varchar(500)):
  - `'ยังไม่เข้าพัก'` (not yet checked in) — set when checkin row created with `Grid1[..,15]=false`.
  - `'เข้าพัก'` — set when actually arrived (CheckBox at row marked).
  - `'Check-Out'` — set on check-out (FrmCheckOut.cs:6238).
  - These also appear as the source for `HT_Room_Status.room_status` (which has its own
    enumeration overlap).
- **`Cin_Dep_Status`** values:
  - `'ยังไม่คืนค่ามัดจำ'` (deposit not yet returned) — initial when deposit collected.
  - `'คืนเงินแล้ว'` (returned) — set by FormShowDEPBack.cs:536.
  - `'ไม่เก็บค่ามัดจำ'` (no deposit collected) — set when no deposit was taken at check-in
    (e.g. `Cin_Room_Dep<=0` ⇒ this string).
- **Operations**:
  - **Insert** (FrmCheckIn save, FrmCheckIn.cs:9321): full INSERT including [id] from get_id,
    then `num6++` for next iteration.
  - **Update on check-out** (FrmCheckOut.cs:6236): sets Cin_Room_Out, Cin_Room_Status='Check-Out',
    Cin_Room_Pay_Total, Cin_Room_night, Cin_Room_PriceTotal, Cin_note.
  - **Update on payment add** (FrmPayAdd.cs:5432): updates Cin_Room_Pay_Total / Cin_Room_PriceTotal.
  - **Update on extend** (ClickUSE.cs:1146): updates Cin_Room_Out for stay extension.
  - **Cancel single line** (ClickUSE.cs:1252): `update HT_CheckIn_Ds set Cin_Room_Status='ยังไม่เข้าพัก' where Cin_no='<cin>' and Cin_room_no='<r>'`.
  - **Refund deposit** (FormShowDEPBack):
    `update HT_CheckIn_Ds set Cin_Dep_return_date=getdate(),Cin_Dep_Status='คืนเงินแล้ว',Cin_Dep_return_by='<emp>' where id=<n>`.
  - **Delete on edit** (FrmCheckIn.cs:9751): `delete from HT_CheckIn_Ds where Cin_no='<cin>' and Cin_Room_Status<>'Check-Out'`. (Preserves checked-out rows.)
- **Invariants**:
  - For every Cin_Room_Status='เข้าพัก' line, `HT_Rooms.Room_Use='yes'` for the same Room_no.
  - Cin_Room_Out should be NULL or > Cin_Room_In until Cin_Room_Status='Check-Out'.

---

### Table: `HT_CheckIn_Pay` (A) — payment ledger per check-in

- **Purpose**: All money movements (cash/credit/transfer/web) attached to a check-in or sale.
  Used as the source for cash-drawer / shift reports and customer debt balance changes.
- **PK convention**: `id` is `int NOT NULL` BUT there's a wrinkle — schema says NOT IDENTITY,
  but live data shows it's **actually IDENTITY** (column type `id int IDENTITY NOT NULL`
  per `_SCHEMA.sql` line 16, but `INFORMATION_SCHEMA.COLUMNS` says `is_identity=1` for
  `HT_CheckIn_Pay.id`). **VERIFY**: confirmed `HT_CheckIn_Pay.id IS IDENTITY` in live DB.
  → The old app's `Insert_Pay` does NOT include `[id]` in the column list (Module1.cs:1781) —
  uses identity. So this column IS DB-managed.
- **Schema** (22 columns):
  ```
  id int (IDENTITY), Pay_no varchar(50), Cin_No varchar(50),
  Cin_Pay_Ds varchar(500), Cin_Pay_Cash float, Cin_Pay_Credit float, Cin_Pay_Date datetime,
  Cin_Pay_Ds_Name varchar(500), Cin_Pay_Ds_ID varchar(50), Cin_Pay_Ds_Price float,
  Cin_Pay_Ds_unit varchar(100), Cin_Pay_Ds_Num float, Cin_Pay_Ds_PriceOne float,
  Cin_Pay_Ds_PriceTotal float,
  Cin_Cust_no varchar(50), Cin_Status varchar(50) NOT NULL DEFAULT '1',
  Cin_Pay_Note varchar(500), Pay_by varchar(100),
  Cin_Pay_Free float NOT NULL DEFAULT 0, Cin_Pay_Tran float NOT NULL DEFAULT 0,
  Branch varchar(50),
  Cin_Pay_web float NOT NULL DEFAULT 0
  ```
- **`Cin_Status` values**:
  - `'1'` (default) = active payment line (verified live).
  - `'ยกเลิก'` = cancelled (set when parent receipt or checkin cancelled).
- **Pay number convention**:
  - `Pay_no` is `R{yyMM}-{4digit}` (per month) - generated by `Module1.GetSIR_PAY()`.
    Many lines may share the same `Pay_no` (one Pay_no per receipt-of-payment, multiple lines
    per Pay_no when a single payment covers multiple line items).
- **Other columns**:
  - `Cin_No`: usually `HT_CheckIn_H.Cin_no`, but for sales w/o checkin it can be:
    - `'ออกโดยไม่อ้างอิงเลขลงทะเบียน'` Thai for "issued without checkin reference",
    - or `HT_Receipt_H.Receipt_no` value, or `HT_Bill_Debt_H.Bill_No`, or even `Book_ID`.
    The new app must set this consistently with whichever source document the payment
    refers to.
  - `Cin_Pay_Ds`: short label, often a room number (e.g. `'101'`), or the literal
    `'การจองแบบระบุห้อง'` (booking refund), or `'ค่าออกภาษีส่วนเกิน'` (over-VAT charge).
  - `Cin_Pay_Cash`/`Cin_Pay_Credit`: split tender amounts, can be negative (refunds use negation).
  - `Cin_Pay_Free`: discount/free amount (e.g. coupon discount).
  - `Cin_Pay_Tran`: bank-transfer amount.
  - `Cin_Pay_web`: web/online payment amount.
  - `Cin_Pay_Ds_Name`: human-readable description, e.g. `'ค่าห้อง'`, `'น้ำดื่ม'`.
  - `Cin_Pay_Ds_ID`: product code (e.g. `'P001'` is reserved for "room rent line").
  - `Cin_Pay_Ds_Num`: quantity (e.g. number of nights or number of bottles).
  - `Cin_Pay_Ds_Price`: total price (= Cin_Pay_Cash+Cin_Pay_Credit+Cin_Pay_Free+Cin_Pay_Tran+Cin_Pay_web).
  - `Cin_Pay_Ds_PriceOne`: unit price.
  - `Cin_Pay_Ds_PriceTotal`: same as Cin_Pay_Ds_Price (denormalized).
  - `Cin_Cust_no`: `'C0001'`-style customer code, or `''` for anonymous.
  - `Branch`: copied from `FormConfirmPay.ComboBox1.Text` — the cashier's selected branch
    name from `TB_SET_Branch.name`. Empty string if no branches configured.
  - `Pay_by`: `Module1.loginName`.
- **Operations**:
  - **Insert** (Module1.Insert_Pay): omits [id] (identity), all other columns set.
    `Cin_Status` defaults to `'1'` (NOT explicitly set in INSERT).
  - **Cancel cascade** when receipt/sale is cancelled:
    `update HT_CheckIn_Pay set cin_status='ยกเลิก' where cin_no='<receipt_no_or_cin>'`.
  - **Customer-rename cascade**: `update HT_CheckIn_Pay set Cin_Cust_no='C0000' where Cin_Cust_no='<delcust>'`.
- **Invariants**:
  - For every active payment, `Cin_Pay_Ds_PriceTotal = Cin_Pay_Cash + Cin_Pay_Credit +
    Cin_Pay_Free + Cin_Pay_Tran + Cin_Pay_Web`. The shift report relies on this.

---

### Table: `HT_CheckIn_Product` (A)

- **Purpose**: Product/service line items charged to a check-in folio (minibar, food, etc.).
- **PK convention**: `id int NOT NULL` (no IDENTITY in schema, but appears autogen). Inserts
  always omit `[id]` so the DB must autogen — implies `id` IS IDENTITY despite schema.
  Verified: `INFORMATION_SCHEMA` says `is_identity=1` for `HT_CheckIn_Product.id`.
- **Schema** (12 columns):
  ```
  id int IDENTITY, Cin_No varchar(50), Cin_Room_no varchar(50),
  Cin_Ds_date datetime, Cin_Pro_id varchar(250), Cin_Pro_name varchar(500),
  Cin_Pro_Unit varchar(250), Cin_Pro_num float,
  Cin_Pro_price float, Cin_Pro_priceTotal float, Cin_Pro_pay float,
  Cin_Pro_note varchar(500)
  ```
- **Operations**:
  - **Insert** (FrmCheckIn.cs:9446, FrmCheckOut.cs:6290, FrmPayAdd.cs:5458): omits [id].
    `Cin_Pro_priceTotal = Cin_Pro_price * Cin_Pro_num` typically (computed in app).
    `Cin_Pro_pay` is amount already paid for this product line.
  - **Update on payment add**:
    `update HT_CheckIn_Product set Cin_Pro_pay=Cin_Pro_pay-<n> where id=<id>` (refund) or `+`.
  - **Delete on edit** (FrmCheckOut.cs:6200): `delete from HT_CheckIn_Product where Cin_no='<cin>'`.
- **Invariants**:
  - Stock change cascade: every INSERT of a product line is paired with
    `update HT_Products set Pro_Amt=Pro_Amt-<num> where Pro_no='<proid>'` (decrement stock).
    On DELETE: `update HT_Products set Pro_Amt=Pro_Amt+<num> where Pro_no='<proid>'`.
    The new app MUST replicate this pairing.

---

### Table: `HT_CheckIn_Other_People` (A)

- **Purpose**: Additional guests in the same room (companions). Free-text.
- **PK convention**: `id` IS IDENTITY (schema says NOT IDENTITY but live verified IDENTITY
  per `INFORMATION_SCHEMA.COLUMNS.is_identity=1`).
- **Schema** (4 columns): `id, Cin_no, Cin_name, Cin_contry`.
- **Operations**:
  - Inserts done in a loop on save (FrmCheckIn.cs:9490). DELETE-then-reinsert on edit
    (FrmCheckIn.cs:9975).
- **Invariants**: none beyond Cin_no must reference HT_CheckIn_H.

---

### Table: `HT_Customers` (A)

- **Purpose**: Customer master.
- **PK convention**:
  - `id int NOT NULL` (NOT IDENTITY) — generated by `get_id("HT_Customers","id")`.
  - `Cust_no varchar(50)` is the **business key** referenced by all other tables. Format
    `C{4digit}` from `MAX(id)+1`. **Race-prone**.
- **Sentinel value**: `'C0000'` is reserved for "deleted customer". When a customer row is
  deleted, every FK-style column in HT_CheckIn_H/Pay, HT_Book_H, HT_Bill_Debt_H, HT_Invoice_H,
  HT_Receipt_H pointing to that Cust_no is updated to `'C0000'`. The new app should treat
  `'C0000'` as a sentinel and not as a valid customer reference. The new app should NOT
  delete customers without performing the same cascade.
- **Schema** (35 columns): see `INFORMATION_SCHEMA`. Highlights:
  - `Cust_no` (key), `Cust_perfix`, `Cust_name`, `Cust_name2` (English/secondary name),
    `Cust_sex`, `Cust_IDcard`, `Cust_Type`, `Cust_Email`,
  - Address fields prefixed `Cust_Add_*` (no, moo, soi, road, tambon, ampore, province, code, tel, fax),
  - Work address fields prefixed `Cust_Work_*` (same shape) + `Cust_Work_Tax`,
  - `Cust_Last_Change datetime`,
  - `Cust_Type_Main varchar(250)`,
  - `Cust_Price_Over float NOT NULL DEFAULT 0` — running balance / store credit (debt).
  - `Cust_Contry varchar(50)`.
- **Operations**:
  - **Insert** (FrmCheckIn.SAVE_CUST, FrmAddBook.SAVE_CUST, FrmAddBook2.SAVE_CUST,
    FrmManageCustomersNew, FrmEditDate.cs:4724): explicit columns including [id] from
    get_id, Cust_no = `"C" + MAX(id)+1` (zero-padded 4 digits). Empty fields written as `''`.
    `Cust_Last_Change = DateTime.Now.Date`.
  - **Update** (FrmManageCustomersNew, FrmCheckIn.EDIT_CUST): full SET on most varchar fields
    `where Cust_no='<no>'`.
  - **Delete cascade** (FrmManageCustomersNew.cs:3649-3656):
    1. `delete from Tb_Save_Image where cust_no in (select cust_no from HT_Customers where id=<id>)`.
    2. `delete from HT_Customers where id=<id>`.
    3. Then 5 cascading UPDATE statements that flip every FK-like reference to `'C0000'`.
  - **Update Cust_Price_Over (running balance)** via `Module1.UPDATE_MONEY`:
    `update HT_Customers set Cust_Price_Over=Cust_Price_Over+<delta> where Cust_no='<no>'`,
    plus an INSERT into `HT_Log_Debt` recording the change.
- **Invariants**:
  - `Cust_no='C0000'` is reserved and should never collide with a real customer.

---

### Table: `HT_Book_H` (A) — booking header

- **Purpose**: Header per reservation (one customer can book one or many rooms over a date range).
- **PK convention**: `Book_ID varchar(50)` — format `R{6digit}` from
  `MAX(replace(Book_ID,'R',''))+1`. NOT NULL semantically though no PK constraint.
  Generated by `FrmAddBook.GET_DOC` / `FrmAddBook2.GET_DOC`.
- **Schema** (18 columns): `Book_ID, Book_Date (created), Book_Date_in, Book_Date_out,
  Book_Cust_ID, Book_Cust_Name, Book_Cust_Name2, Book_Cust_Tel, Book_Price_Total,
  Book_Price_Pay (deposit paid), Book_Status, Book_by, Book_room_all, Book_room_note,
  Book_room_type int NOT NULL DEFAULT 1, Book_Notify_Day int NOT NULL DEFAULT 0,
  Book_Notify_Note varchar(50), Book_Sale varchar(150)`.
- **Status-flag enumeration**:
  - `Book_Status` (varchar(50)): live data shows three values:
    - `'จอง'` — initial (booked, not yet checked in). Set on insert.
    - `'เข้าพัก'` — converted to check-in (set by FrmCheckIn.cs:9505).
    - `'ยกเลิก'` — cancelled (set by FrmShowBookNotify.cs:1077).
  - `Book_room_type` (int): `1` = no specific rooms (FrmAddBook), `2` = with specific rooms
    (FrmAddBook2). Default 1.
  - `Book_Notify_Note` (varchar): `NULL` (default) or `'ไม่แจ้งเตือน'` (suppress notification).
- **Operations**:
  - **Insert** (FrmAddBook.cs:3042, FrmAddBook2.cs:3522, FrmBookRooms.cs:2322):
    Book_Status='จอง', Book_room_type=1 or 2.
  - **Convert booking → check-in** (FrmCheckIn.cs:9505):
    `update HT_Book_H set Book_Status='เข้าพัก' where Book_ID='<id>'`.
  - **Cancel** (FrmShowBookNotify.cs:1077):
    `update HT_Book_H set Book_Status='ยกเลิก' where Book_ID='<id>'` AND
    `update HT_Book_ds set Book_status=3 where Book_No='<id>'`.
  - **Suppress notify**: `update HT_Book_H set Book_Notify_Note='ไม่แจ้งเตือน' where Book_ID='<id>'`.
  - **Edit** (FrmAddBook2.cs:3789): `delete from HT_Book_H where Book_ID='<id>'` then INSERT
    (delete-then-reinsert pattern). Same for HT_Book_Date, HT_Book_Ds, HT_Book_Pro.
  - **Customer rename cascade**: `update HT_Book_H set Book_Cust_ID='C0000' where Book_Cust_ID='<delcust>'`.

---

### Table: `HT_Book_Ds` (A) — booking line per room/type

- **Purpose**: Line items under a booking — one row per (room or room-type, date range).
- **PK convention**: `id int IDENTITY` per schema. **However**: FrmBookRooms.cs:2461 INSERTs
  with explicit `[id]=get_id(...)`. This is a known oddity — the old app must have
  IDENTITY_INSERT toggled at the connection level somewhere, OR the schema is actually
  non-IDENTITY despite information_schema saying otherwise. Verified `is_identity=1` so
  this insert path **would fail** in live DB. The same form does another insert WITHOUT
  [id] (FrmAddBook2.cs:3566) — that path works. → FrmBookRooms.SAVE_NEW (with explicit id)
  is likely **broken in MSSQL mode** and only works in Access mode (which has no IDENTITY).
  **The new app should NOT include [id] in INSERTs**, mimicking FrmAddBook/FrmAddBook2.
- **Schema** (11 columns):
  ```
  id int IDENTITY, Book_No varchar(50), Book_Room_Type varchar(50),
  Book_Room_Start datetime, Book_Room_End datetime,
  Book_Room_Price float, Book_Room_Night float, Book_Room_Num float,
  Book_Room_PriceToTal float, Book_Room_Note varchar(500),
  Book_status int NOT NULL DEFAULT 1
  ```
- **Status flag**: `Book_status` int. Values: `1` (default, active), `3` (cancelled — set
  by FrmShowBookNotify.cs:1078). No code uses `2`.
- **Operations**:
  - **Insert** (FrmAddBook.cs:3086, FrmAddBook2.cs:3566): no [id].
  - **Cancel cascade** (FrmShowBookNotify): `update HT_Book_ds set Book_status=3 where Book_No='<id>'`.
  - **Delete on edit**: `delete from HT_Book_Ds where Book_No='<id>'` then re-insert.

---

### Table: `HT_Book_Date` (A) — calendar grid for bookings

- **Purpose**: One row per (booking, room-type-or-room, calendar-date). Used to populate
  the multi-day booking heatmap and to detect collisions.
- **PK convention**: `id int NOT NULL` (NOT IDENTITY) — generated by `get_id("HT_Book_Date","id")`.
- **Schema** (8 columns):
  ```
  id int NOT NULL, Book_no varchar(50), Book_type varchar(50),
  Book_date_ds datetime, Book_Num int,
  Book_USE int NOT NULL DEFAULT 0, Book_ok int NOT NULL DEFAULT 0,
  Cin_no varchar(50)
  ```
- **Status flags**:
  - `Book_USE int` → 0 (not yet used) / 1 (consumed/checked-in for that day). No code sets to 1
    explicitly (verify this is dead).
  - `Book_ok int` → 0 (default) / 1 (cancelled) — `update HT_Book_Date set Book_ok=1 where id=<id>`
    in ClickBook.cs:336 cancel path.
- **Operations**:
  - **Insert per day** (FrmAddBook2.cs:3604, FrmCheckIn checks): explicit [id] via get_id,
    one row per day in booking range.
  - **Delete on edit** (FrmAddBook2.cs:3788): `delete from HT_Book_Date where Book_no='<id>'`.
  - **Mark cancelled day** (ClickBook): `update HT_Book_Date set Book_ok=1 where id=<id>`.
  - **Startup prune** (frmMain1.cs:7075):
    `delete FROM HT_Book_Date WHERE Book_date_ds < DATEADD(dd, -60, GETDATE())`.
    The new app must accept that bookings older than 60 days will be missing detail rows.

---

### Table: `HT_Book_Pro` (A) — pre-booked products

- **Purpose**: Products attached to a booking (food, drinks pre-ordered).
- **PK**: `id int IDENTITY`. Inserts omit [id] (FrmAddBook2.cs:3638).
- **Schema** (9 cols): `id, B_NO (Book_ID), B_ROOM, B_NAME, B_UNIT, B_NUM, B_PRICE, B_PRICE_TOTAL, B_PRO_ID`.
- **Operations**: insert in loop on FrmAddBook2 save; delete-on-edit `delete from HT_Book_Pro where [B_NO]='<id>'`.

---

### Table: `HT_Receipt_H` (A) — sales receipt header

- **Purpose**: Sales receipt (POS, vatable).
- **PK convention**: `id int` (per schema NOT IDENTITY, but actually IDENTITY in live DB).
  Generated via `get_id("HT_Receipt_H","id")` and explicitly INSERTED. Risk: same as
  HT_Book_Ds — works in live DB only if IDENTITY_INSERT is enabled on the session.
  *(Check live DB: `is_identity=0` for HT_Receipt_H.id per INFORMATION_SCHEMA — confirmed
  NOT IDENTITY.)*
  - Confirmed live: `HT_Receipt_H.id is_identity=0` → safe to INSERT explicit [id].
  - `Receipt_no varchar(50)` is the human-facing key: `B/SB/CB{yyMM}-{4digit}`.
- **Schema** (20 columns): `id, Receipt_no, Receipt_Date, Receipt_c_no, Receipt_Name,
  Receipt_Address, Receipt_Tel, Receipt_Fax, Receipt_Discount, Receipt_Total, Receipt_Vat,
  Receipt_BeforeVat, Receipt_VatIn, Receipt_VatPer, status_name, Receipt_Ref,
  Receipt_cin_vat_before, Receipt_note, Receipt_Tax, Receipt_noteUP`.
- **Status-flag**: `status_name`: `'ปกติ'` (active) or `'ยกเลิก'` (cancelled). Set to ยกเลิก
  by FrmReceiptMain.cs:2681 / FrmAddSale on cancel.
- **`Receipt_VatIn`**: `'True'`/`'False'` (capitalized). Whether VAT is included in price.
- **`Receipt_Ref`**: optional `HT_CheckIn_H.Cin_no` linking the receipt to a folio. `''` if
  standalone receipt.
- **`Receipt_c_no`**: `HT_Customers.Cust_no`, or `'C0000'` after customer-delete cascade.
- **Operations**:
  - **Insert** (FrmAddSale.cs:4025): explicit [id] from get_id. status_name='ปกติ'.
  - **Cancel** (FrmReceiptMain.cs:2681): `update HT_Receipt_H set status_name='ยกเลิก' where id=<id>` +
    cascade-cancel HT_CheckIn_Pay rows where cin_no=Receipt_no, and reverse
    HT_CheckIn_H.Total_Price_vat by Receipt_cin_vat_before.

---

### Table: `HT_Receipt_Ds` (A) — sales receipt lines

- **PK**: `id int IDENTITY`.
- **Schema** (10 cols): `id, S_Sale_id (=Receipt_H.id), S_Product_no, S_Product_name, S_Unit,
  S_UnitName, S_Price, S_Total, S_PriceDiscount, S_PriceDiscount_per`.
- **Operations**: insert on FrmAddSale; delete-on-edit `delete from HT_Receipt_Ds where S_Sale_id=<id>`.

---

### Table: `HT_Invoice_H` and `HT_Invoice_Ds` (A) — VAT invoice doc

- **Purpose**: Tax-invoice document (issued separately from a receipt for VAT-registered customers).
- **PK convention**: `HT_Invoice_H.id int` (per schema, not IDENTITY) — generated by `get_id`.
  `Receipt_no varchar(50)` is `B/SB/CB{yyMM}-{4digit}` (same scheme as HT_Receipt_H, separate counter).
- **Schema**: same shape as HT_Receipt_H (20 columns) — they're parallel mini-schemas.
  `HT_Invoice_Ds` parallels `HT_Receipt_Ds` (10 cols).
- **Operations**: parallel to HT_Receipt_H/Ds. Cancel sets `status_name='ยกเลิก'`.

> **Distinct from `HT_INVOICE` (uppercase!)** which is the booking-invoice (see below).

---

### Table: `HT_INVOICE` (A — but barely used)

- **Purpose**: Invoice issued from a *booking* (not a check-in). Different schema from `HT_Invoice_H`.
- **PK**: `INV_NO int NOT NULL` — generated by `get_id("HT_INVOICE","INV_NO")` formatted
  as 4-digit zero-padded string then converted to int (FormBookingInvoice.cs:1650).
- **Schema** (16 cols): `INV_NO, INV_booking_no, INV_STAY varchar(100), INV_DATE, INV_BY,
  INV_TITLE, INV_NAME, INV_COMPANY, INV_ADDRESS, INV_TEL, INV_NIGHT varchar(20), INV_PAX,
  INV_PAX_CHILD, INV_PAYMENT, INV_DUEDATE, INV_NOTE text`.
- **Live data**: only **2 rows**. The form `FormBookingInvoice` exists but is rarely used.
  The new app should preserve compatibility but not invest heavily in this table.
- **Operations**: `delete from HT_INVOICE where INV_booking_no='<bookid>'` then INSERT
  (delete-then-reinsert pattern, FormBookingInvoice.save).

---

### Table: `HT_Invoice_Note` (A)

- **Purpose**: Free-text note attached to an invoice (cin_no key).
- **PK**: composite `(Cin_no)` (no PK declared but functionally one row per Cin_no).
- **Schema**: `Cin_no varchar(50), note text, NOTE_STATUS varchar(20)`.
- **Operations** (INV_Note.cs:253-257):
  - INSERT on first save: `INSERT INTO HT_Invoice_Note VALUES ('<cin>', '<text>', NULL)`.
    **Note** the form does `RichTextBox1.Text.Replace("'","\"")` for quoting.
  - UPDATE on subsequent: `update HT_Invoice_Note set note='<text>' where Cin_no='<cin>'`.
  - The third column `NOTE_STATUS` is never written (always NULL).

---

### Table: `HT_Bill_Debt_H` and `HT_Bill_Debt_Ds` (A) — credit-sale ledger

- **Purpose**: Sales on credit (customer owes money).
- **PK**: `Bill_No varchar(20)` — `B{yyMM}-{4digit}`. (Note: same prefix as HT_Receipt_H!).
- **Schema H** (18 cols): `Bill_No, Bill_Cust_ID, Bill_Cust_Name, Bill_Cust_Address,
  Bill_Cust_Tel, Bill_Cust_Fax, Bill_Date, Bill_Ref, Bill_Price_Type, Bill_Type,
  Bill_Total, Bill_Pay, Bill_Debt, Bill_Pay_CASH, Bill_Pay_CREDIT, Bill_Status,
  Bill_by, Bill_Note`.
- **Status**: `Bill_Status`:
  - `'ปกติ'` (active, default).
  - `'ยกเลิก'` (cancelled).
  - `'ชำระแล้ว'` (paid in full) — set when Bill_Debt reaches 0.
- **`Bill_Type`** values: `'เงินสด'` (cash), `'เครดิต'` (credit). FormConfirmPay sets the
  default tender mode. (Live data shows `''` only — table has no rows in this DB).
- **Schema Ds** (9 cols): `id IDENTITY, Bill_No, DS_ID, DS_NO (=product_no), DS_NAME,
  DS_UNIT, DS_NUM, DS_PRICE, DS_PRICE_TOTAL`.

---

### Table: `HT_Cupon` (A)

- **Purpose**: Food/breakfast coupon entitlement (one row per coupon).
- **PK**: `cupon_no int` (NOT IDENTITY) — generated by `get_id("HT_Cupon","cupon_no")`.
- **Schema**: `cupon_no, cupon_cin_no varchar(50), cupon_cin_room varchar(50), cupon_date,
  cupon_gen_date, cupon_by varchar(50), cupon_print int NOT NULL DEFAULT 0`.
- **Status**: `cupon_print`: `0` = unprinted (default), `1` = printed.
- **Operations**:
  - Inserted by `Module1.GEN_Cupon` once per night×Cin_cupon-count at check-in time.
  - Updated to `cupon_print=1` after printing (Print_Report.cs:2552).

---

### Table: `HT_Deposit` (A)

- **Purpose**: Standalone deposit ledger (deposits not tied to a single check-in folio).
- **PK**: `id int` (NOT IDENTITY) via `get_id`.
- **Schema** (8 cols): `id, Dep_no varchar(250), Dep_Date, Dep_Room, Dep_Name, Dep_Price,
  Dep_Status varchar(50), Dep_ref varchar(50)`.
- **`Dep_Status`** values:
  - `'รับเงินมัดจำแล้ว'` (deposit received) — set on insert (FrmAddDep.cs:948).
  - `'คืนเงินแล้ว'` — when refunded (likely; check usage).
- **`Dep_no`**: `DEP{yyMM}-{4digit}`.

---

### Table: `HT_Changed_Room` (A)

- **Purpose**: Audit log for mid-stay room changes (guest moved from room A → B).
- **PK**: `id int IDENTITY`. Insert omits [id] in `Module1.Change_Room`.
- **Schema** (8 cols): `id, cin_no varchar(50) NOT NULL, room_before, room_after, change_date,
  room_before_price float NOT NULL DEFAULT 0, Note varchar(255), ToPrice varchar(20)`.
- **Operations**:
  - INSERT only (Module1.cs:1351, 1370). Two variants based on `ch_p` (price-change) flag.
  - Used to drive `View_Changed_Room` for the room-change report.
- The same operation also updates `HT_CheckIn_Ds.Cin_Room_No` from the old room to the new
  (the actual room move) and updates `HT_Rooms` flags for both old/new rooms.

---

### Table: `HT_Housewife` (A)

- **Purpose**: Housekeeping log — every clean / dirty / repair action.
- **PK**: `id int IDENTITY`. INSERTs omit [id].
- **Schema**: `id, h_name varchar(150), h_room varchar(50), h_date datetime, h_note text,
  h_cin varchar(50), h_cin_name varchar(250)`.
- **Operations** (ClickClean.cs:511 etc.):
  - INSERT on every clean/dirty/repair action. h_cin = the most recent checkout's Cin_no
    in that room (lookup `View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no=...
    order by cin_room_out desc`).
  - h_note distinguishes: plain note (start cleaning), `'เปลี่ยนสถานะเป็นซ่อม : <note>'`
    (sent to repair), or end-cleaning note.
  - **No UPDATE / DELETE.**

---

### Table: `HT_Rooms_Repair` (A)

- **Purpose**: Repair log.
- **PK**: `id int IDENTITY`.
- **Schema**: `id, room_no, Repair_date, Repair_by varchar(250), Repair_note text`.
- **Operations**: INSERT only via `Module1.INSERT_REPAIR`.

---

### Table: `HT_Rooms_Cancel` (A)

- **Purpose**: Cancellation log when a room is removed from a check-in.
- **PK**: `id int` (NOT IDENTITY) via `get_id`.
- **Schema**: `id, room_no, cin_no, cancel_date, cancel_by varchar(50), cancel_note text`.
- **Operations**: INSERT only via `Module1.SaveCancel`.

---

### Table: `HT_POWER_LOG` (A)

- **Purpose**: Per-room electricity-relay on/off log.
- **PK**: `id int IDENTITY`.
- **Schema** (8 cols): `id, ROOM_NO, ROOM_POWER_START, ROOM_POWER_END, ROOM_POWER_START_BY,
  ROOM_POWER_END_BY, ROOM_POWER_NOTE varchar(250), ROOM_POWER_NOTE2 varchar(250)`.
- **Operations** (Module1.save_power_log):
  - On power-ON: INSERT new row with ROOM_POWER_START=GETDATE(), ROOM_POWER_END_BY=`''`
    (sentinel: empty string means "still on").
  - On power-OFF:
    `update HT_POWER_LOG SET ROOM_POWER_END=GETDATE(),ROOM_POWER_END_BY='<emp>',ROOM_POWER_NOTE2='<note>' where room_no='<r>' and ROOM_POWER_END_BY=''`.
- **Invariant**: at most one row per room with `ROOM_POWER_END_BY=''` at any time = the
  currently-on session. The new app must preserve this convention.

---

### Table: `HT_Log` (A — sparse)

- **Purpose**: System audit log. **NOT** a general write log — only specific events:
  login success/failure, admin trapdoor entry, receipt deletion.
- **PK**: `id int IDENTITY`.
- **Schema**: `id, details varchar(250), Emp_name varchar(150), DODATE varchar(50)`.
- **`DODATE` is `varchar`**, not datetime. Format `dd/MM/yyyy HH:mm:ss`.
- **Operations**: INSERT only via `Module1.LOG`. Callers: login.cs, frmMain1.cs (admin
  trapdoor), FrmReportShiftCash.cs (receipt deletion). The new app does NOT need to write
  to this table for general events.

---

### Table: `HT_Log_Debt` (A)

- **Purpose**: Audit trail for `HT_Customers.Cust_Price_Over` changes (every running-balance
  change).
- **PK**: `id int IDENTITY`.
- **Schema**: `id, log_cus, log_ds varchar(250), log_date, log_price_From, log_price, log_price_To`.
- **Operations**: INSERT only by `Module1.UPDATE_MONEY` after changing
  `HT_Customers.Cust_Price_Over`. The new app MUST replicate this audit insert when
  changing any customer balance.

---

### Table: `HT_EMP_SMS` (A) — internal employee-to-employee notes

- **PK**: `SMS_ID int IDENTITY`.
- **Schema**: `SMS_ID, SMS_TO varchar(50), SMS_Details text, SMS_By varchar(250),
  SMS_Readed varchar(50)`.
- **Status**: `SMS_Readed`: `'no'` (default on insert) / `'yes'` (read).
- **Operations** (EMP_Note.cs): INSERT on send. UPDATE for read.

### Table: `HT_Room_SMS` (A) — sticky note on a room

- Same shape as HT_EMP_SMS but keyed on `SMS_Room` (room_no) instead of SMS_TO.

---

### Table: `HT_Round_Bill` (A) — shift / round

- **Purpose**: Cashier shift open/close ledger.
- **PK**: `id int NOT NULL` (NOT IDENTITY) via `get_id("HT_Round_Bill","id")`.
- **Schema** (6 cols): `id, round_no, round_price float, round_by varchar(150),
  round_start datetime, round_end datetime`.
- **Operations** (FrmDueBill.cs:1653, 1670):
  - **Open**: INSERT (id, round_start, round_price, round_by). round_end NULL.
  - **Close**: `update HT_Round_Bill set round_end='<now>',round_by='<emp>' where round_end is null`.
- **Invariant**: at most one row with `round_end IS NULL` at any time. This is the
  **transactional gate** for the entire system — `check_round_bill()` reads it on every
  POS/check-in operation.

---

### Table: `HT_Products` (A)

- **PK**: `id int` (NOT IDENTITY) via `get_id`. `Pro_no varchar(50)` is the business key
  (format `<ProType.id_full>-<3digit>`, e.g. `B-001`). Sentinel: `'P001'` = "room rent line".
- **Schema** (11 cols): `id, Pro_no, Pro_Type, Pro_Name, Pro_PriceA, Pro_PriceB, Pro_PriceC,
  Pro_Amt (stock), Pro_Unit, Pro_cap, Pro_Barcode`.
- **Stock invariant**: every INSERT/UPDATE/DELETE of HT_CheckIn_Product (or other product
  consumption) must update `Pro_Amt` accordingly (see HT_CheckIn_Product section).

### Table: `HT_Products_Price` (A)

- **PK**: `id int IDENTITY`.
- **Purpose**: per-customer-type price overrides for products. Composite key (P_ID, P_CustType).
- **Schema**: `id, P_ID (=Pro_no), P_CustType, P_Price`.
- **Operations**: delete-then-reinsert per product on edit (FrmManageProduct).

### Table: `HT_Rooms_Price` (A)

- **PK**: `id int IDENTITY`. Composite key (Room_Type, Room_CustType).
- **Schema**: `id, Room_Type, Room_CustType, Room_Price, Room_Price_H (hourly), Room_Price_M (monthly)`.
- **Operations**: delete-then-reinsert on FrmSETRoomType/FrmSETCsuType edits.

### Table: `HT_ContinueTime` (A)

- **PK**: `id int IDENTITY`.
- **Purpose**: extension/continue-stay pricing rules.
- **Schema**: `id, Con_Name, Con_Minute int, Con_Price float, Con_Type`.
- **`Con_Type`** values seen: free-text categorization.

### Table: `HT_SET_RoomType` (A) — master room types

- **PK**: `id int IDENTITY`.
- **Schema**: `id, id_full varchar(50), name varchar(50), Room_PriceA/B/C float`.

### Table: `HT_SET_CusType` (A) — customer-type tier

- **PK**: `id int IDENTITY`.
- **Schema**: `id, id_full, name, deposit float NOT NULL DEFAULT 0`.

### Table: `HT_SET_CusType_Main` (A)

- Parent of HT_SET_CusType. `id int IDENTITY, id_full, name`.

### Table: `HT_SET_ProductType` (A) — same shape as CusType.

### Table: `HT_SET_Sale` (A) — sales rep master

- `id int IDENTITY, id_full, name varchar(150), tel, address, other`.

### Tables: `HT_Order_Up`, `HT_Order_Down` (A)

- **Purpose**: Per-customer-type, per-month price-override factors.
- **PK**: `id int` (NOT IDENTITY). The `id` is a positional row-index `1..n` within a single
  Cust_Type — DO NOT assume globally unique; the writer does
  `delete from HT_Order_Up where Cast_Type='<X>'` then INSERTs sequential ids 1..n.
  See FormManageOrderCust.cs:760.
- **Schema**: `id, Cust_Type varchar(50), Cust_Month int, Cast_Type varchar(250)`.
  Note `Cast_Type` (typo) is the *parent* customer type group.

### Table: `Tb_Save_Image` (A)

- **PK**: `id int IDENTITY`.
- **Purpose**: BLOB store for ID-card photos and customer photos.
- **Schema**: `id, cin_no, ttype varchar(250), pic varbinary(MAX), cust_no, tmp_no, pic_date`.
- **`ttype`** values:
  - `'บัตรประชาชน'` — Thai national ID card image (read from Smart Card or scanned).
  - `'รูปลูกค้า'` — customer face photo (probable; verify in code).
- **Operations**:
  - INSERT during check-in or customer save. `pic` is a hex literal `0x{HEXBYTES}` in the
    SQL string (NOT base64, NOT binary param).
  - During check-in, photos are inserted with `tmp_no=<random>` and `cust_no=''`. After
    check-in commit:
    `update Tb_Save_Image set cin_no='<cin>', cust_no='<custno>', tmp_no='' where tmp_no='<tmp>'`.
  - **Startup prune** (frmMain1.cs:7071):
    `DELETE FROM Tb_Save_Image WHERE cust_no='' and tmp_no<>'' and pic_date<<2_days_ago>`
    — orphaned uploads are auto-cleaned after 2 days.
- **The new app must use the same hex-literal varbinary writing convention** (or use a
  parameterized `varbinary` parameter — both work; the issue is reads must be by-row).

### Table: `TB_FOLIO` (A)

- **Purpose**: User-editable folio body (free-text rows for printing a folio document).
- **PK**: `id int` (NOT IDENTITY) via `get_id`.
- **Schema** (13 cols, ALL `varchar` except `id`): `id, NO varchar(20), CIN_NAME1/2/3,
  F_ROOM, F_NAME, F_IN, F_OUT, F_NIGHT, F_PRICE, F_PRICE_TOTAL, F_STATUS varchar(20)`.
- **Operations** (FormFolio.cs:1224):
  - **POSITIONAL VALUES** insert (no column list): `INSERT INTO TB_FOLIO VALUES (<id>, '<cin>', '<n1>', '<n2>', '<n3>', '<room>', '<name>', '<in>', '<out>', '<night>', '<price>', '<total>', NULL_or_'')`.
  - **Delete-then-reinsert** per Cin_no on save: `delete from TB_FOLIO where NO='<cin>'`.
- **`F_STATUS`**: not actively set; always NULL or `''`.

### Table: `TB_Pay_History` (A)

- **Purpose**: General-ledger income/expense entries (rayrap-rayjai).
- **PK**: `id int` (NOT IDENTITY) via `get_id`.
- **Schema** (10 cols): `id, Pay_Date float (OADate!), Pay_Bill, Pay_Cust, Pay_Type,
  Pay_Total, Pay_Note, Pay_Program float (OADate!), Pay_Group, Pay_Account`.
- **`Pay_Date`** is **stored as OADate (float)**, not datetime. Convert via
  `DateTime.ToOADate()` / `DateTime.FromOADate()`.
- **Operations** (FrmAddPay.cs:638): positional INSERT (no column list).

### Table: `TB_SETTINGS` (R-only-mostly)

- **Purpose**: Single-row global config (company info, VAT, defaults). Updated only by
  `FrmSettings`, `frmMain1` startup migrations, and `frmReg`.
- **PK**: none. Single-row table; queries are unconditional `SELECT * FROM TB_SETTINGS`.
- **Schema** (33 cols): see schema; key fields: `Company_Name, Company_Address, Company_Tel,
  Company_Tax, CHK_IN_Before float (HHmm cutoff), CHK_Out, Maximum_Book, Vat_per, Vat_Head,
  reg_type varchar(10), AUTO_CUT_POWER varchar(10) ('True'/'False'), MANUAL_POWER, VAT_OUT
  ('เปิด'/'ปิด')`.
- **The new app should NOT modify** unless replicating the FrmSettings UI. Reads are fine.

### Table: `TB_MRP_EMPLOYEE` (R-only)

- **PK**: `ID int IDENTITY`.
- **Schema**: `ID, Emp_Username, Emp_Password (plaintext!), Emp_Name, Emp_Level, Emp_Vat`.
- **Use**: login authentication. New app should treat as read-only, OR if managing users,
  match the plaintext-password convention (yes, plaintext — security debt).

### Table: `TB_MRP_Permission` (R-only)

- **Schema**: `Level_Name varchar(50), Level_Command varchar(50)`.
- **Operations**: delete-then-reinsert per Level_Name on FrmPermission save.

### Tables: `TB_SET_Branch`, `TB_SET_MyType2`, `TB_SET_MyType2_2`, `TB_SET_MyType3` (R-mostly)

- Master-data taxonomy tables. Each has `id IDENTITY, id_full, name`.

### Table: `Tb_Version`

- **Purpose**: Single-cell schema version. `V_NO varchar(50)`.
- Used by `frmMain1.Update_Version` to know which migration steps to apply on startup.
  The new app should NOT touch this; let the old app drive migrations.

---

## 3. Multi-Table Cascade Catalog

> Each cascade is the **complete set of writes** the old app performs as part of a logical
> operation. The new app MUST perform every step (in order, where sequence matters) to
> avoid leaving orphan state.

### 3.1 Walk-in Check-in (no prior booking)

Trigger: user clicks `FrmCheckIn → ButtonOK` after entering rooms/dates/customer.
Source: `FrmCheckIn.cs:9300-9700`.

```
Step 0: TdocNum.Text = GET_DOC()  -- generate Cin_no = "CH" + yy + "-" + 6digit
Step 1: If new customer:
    text = "C" + (MAX(HT_Customers.id)+1, 4-digit)
    INSERT HT_Customers (id, Cust_no, Cust_name, ..., Cust_Last_Change=Today)
Step 2: For each room row in Grid1:
    INSERT HT_CheckIn_Ds (id=get_id(), Cin_No, Cin_Room_No, ..., Cin_Room_Status='เข้าพัก' or 'ยังไม่เข้าพัก',
                          Cin_Dep_Status='ยังไม่คืนค่ามัดจำ' or 'ไม่เก็บค่ามัดจำ', Cin_cupon=N)
    -- num6++
    if room_pay_partial > 0:
        Insert_Pay(...) -- writes HT_CheckIn_Pay with new Pay_no
    UPDATE HT_Rooms SET room_use='yes' WHERE room_no=<r>
    For each calendar-day in stay:
        if HT_Room_Status row exists: UPDATE room_status='เข้าพัก', room_Details=<custname>, room_CheckIn_No=<cin>
        else: INSERT HT_Room_Status (id=get_id(), room_no, room_date, room_status='เข้าพัก', room_Details, room_CheckIn_No, room_date_oa=ToOADate)
        GEN_Cupon(room, cin, date, count, true) -- writes HT_Cupon (one row per coupon)
Step 3: For each product row in Grid2:
    INSERT HT_CheckIn_Product (Cin_No, Cin_Room_no, Cin_Pro_id, ...)
    if pay_amount > 0: Insert_Pay(...)
    UPDATE HT_Products SET Pro_Amt=Pro_Amt-<num> WHERE Pro_no=<p>  -- decrement stock
Step 4: For each "other people" entry:
    INSERT HT_CheckIn_Other_People (Cin_no, Cin_name, Cin_contry)
Step 5: If converting from booking (TbookNo not empty):
    UPDATE HT_Book_H SET Book_Status='เข้าพัก' WHERE Book_ID=<book>
    UPDATE HT_Rooms SET room_book_ds='', room_book='', room_book_name='', room_book_time=''
        WHERE room_no in (SELECT room_no FROM View_HT_ROOM WHERE book_no=<book>)
Step 6: INSERT HT_CheckIn_H (Cin_no, Cin_Date, Cin_Book_no, Cin_cust_no, Cin_cust_price,
                              Cin_status='ปกติ', Total_Price_Room/Product/Net/Pay/Balance,
                              Cin_Car_type, Cin_Car_id, Cin_Room_ALL=<list>, Cin_by=loginName,
                              Cin_Date_in, Cin_Date_out, Cin_Type=<0|1|2>, Cin_foreign='True'/'False')
Step 7: Module1.UPDATE_MONEY(custno, amount, "DEL", "ตัดจากใบลงทะเบียน <cin>")
    -> UPDATE HT_Customers SET Cust_Price_Over -= amount
    -> INSERT HT_Log_Debt (log_cus, log_ds, log_date, log_price=-amount, log_price_From, log_price_To)
Step 8: For each room: Module1.Power_set(room, "ON", "", "เปิดไฟ อัตโนมัติ จากเช็คอิน No.<cin>")
    -> writes HT_POWER_LOG (insert new ON-row if no open one)
    -> UPDATE HT_Rooms SET Room_Power_STATUS='on' WHERE room_no=<r>
Step 9: UPDATE Tb_Save_Image SET cin_no=<cin>, cust_no=<cust>, tmp_no='' WHERE tmp_no=<tmp_no>
Step 10: Print_Report → ReportReg_*
```

### 3.2 Check-out & Settle

Source: `FrmCheckOut.cs:6190-6400`.

```
Pre: FormConfirmPay collected payment info (PFREE, TRANN, WEB, Branch via ComboBox1).
Step 1: If editing existing checkin (re-entry):
    For each existing HT_CheckIn_Product row:
        UPDATE HT_Products SET Pro_Amt=Pro_Amt+<num>  -- restore stock
    DELETE FROM HT_CheckIn_Product WHERE Cin_no=<cin>
Step 2: For each room in Grid1:
    if (mark_checkout=true and current_status<>'Check-Out'):
        Module1.Power_set(room, "OFF", "", "ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.<cin>")
        UPDATE HT_CheckIn_Ds SET Cin_Room_Out=<now>, Cin_Room_Status='Check-Out',
                                 Cin_Room_Pay_Total=<total>, Cin_Room_night=<nights>,
                                 Cin_Room_PriceTotal=<total>, Cin_note=<note>
            WHERE id=<dsid>
        UPDATE HT_Rooms SET room_use='no', Room_Clean='yes', Room_Use_Count=Room_Use_Count+<nights>
            WHERE room_no=<r>
        UPDATE HT_Room_Status SET room_status='Check Out'  -- (without hyphen!)
            WHERE room_no=<r> AND room_CheckIn_No=<cin>
    elif (uncheckout, mark_checkout=false and current='Check-Out'):  -- reverse
        Module1.Power_set(room, "ON", ..., "...แล้วปรับเป็นเช็คอิน...")
        UPDATE HT_CheckIn_Ds SET Cin_Room_Out=<now>, Cin_Room_Status='เข้าพัก', ... WHERE id=<dsid>
        UPDATE HT_Rooms SET room_use='yes', Room_Clean='no', Room_Use_Count -= nights
        UPDATE HT_Room_Status SET room_status='เข้าพัก' WHERE room_no=<r> AND room_CheckIn_No=<cin>
    else: only update HT_CheckIn_Ds totals.
    if pay > 0: Module1.Insert_Pay(cin, room, now, cash, debt, "ค่าห้อง", pay, "รายการ", payno, custno, "P001", nights, total, price, note, FREE, TRAN, WEB)
Step 3: For each product row in Grid2:
    INSERT HT_CheckIn_Product (Cin_No, ..., Cin_Pro_priceTotal, Cin_Pro_pay, Cin_Pro_note)
    if pay > 0: Insert_Pay(...)
    UPDATE HT_Products SET Pro_Amt -= num
Step 4: UPDATE HT_CheckIn_H SET Total_Price_Room/Product/Net/Pay/Balance, Cin_note WHERE Cin_no=<cin>
Step 5: Module1.UPDATE_MONEY(custno, amt, "DEL", "ตัดจากใบลงทะเบียน <cin>")
    -> UPDATE HT_Customers, INSERT HT_Log_Debt
Step 6: If overcharge VAT scenario (rare): Insert_Pay extra row "ค่าออกภาษีส่วนเกิน".
Step 7: Print_Report → sale_vat / sale_vat0 / Folio.
Step 8: Optional FormSMS_DEBT (if outstanding debt).
```

### 3.3 Create Booking (no specific rooms, "ระบุประเภทห้อง")

Source: `FrmAddBook.cs:3030-3270`.

```
Step 0: TdocNum.Text = GET_DOC()  -- "R" + 6digit
Step 1: If new customer: INSERT HT_Customers (same shape as J1 step 1).
Step 2: INSERT HT_Book_H (Book_ID, Book_Date=now, Book_Cust_ID=<custno>, Book_Cust_Name,
                          Book_Cust_Name2, Book_Cust_Tel, Book_Price_Total, Book_Price_Pay,
                          Book_Status='จอง', Book_Date_in, Book_Date_out, Book_by=loginName,
                          Book_room_all=<list>, Book_room_note, book_room_type=1,
                          Book_Notify_Day=<n>, Book_sale)
Step 3: For each room-type line in Grid1:
    INSERT HT_Book_Ds (Book_No, Book_Room_Type, Book_Room_Start, Book_Room_End,
                       Book_Room_Price, Book_Room_Night, Book_Room_Num, Book_Room_PriceToTal,
                       Book_Room_Note)
    For each calendar day in this line:
        INSERT HT_Book_Date (id=get_id(), Book_no, Book_type=<RoomType>, Book_date_ds=<date>,
                             Book_Num=<count>, Book_USE=0)
Step 4: If deposit paid (Tpay>0):
    Module1.Insert_Pay(Book_ID, "การจองแบบไม่ระบุห้อง", now, cash, credit, "เงินจองห้อง",
                      total, "รายการ", payno, custno, "P001", 1, total, total, note, FREE, TRAN, WEB)
Step 5: Module1.UPDATE_MONEY(custno, amount, "ADD", "เงินจองห้อง <book_id>")
    -> HT_Customers.Cust_Price_Over += amount
    -> INSERT HT_Log_Debt
Step 6: Print_Report → ReportBooking
```

### 3.4 Create Booking (with specific rooms, FrmAddBook2)

Same as 3.3 but adds:
- `Book_room_type=2`.
- Step 3: ALSO sets `HT_Rooms.Room_Book=<HT_Book_Date.id>, Room_Book_Name=<custname>,
  Room_Book_Time=<datetime>, Room_Book_ds=<descr>` for each (room, date) in range.
- Step 3.5: For each pre-booked product line in Grid2:
  - `INSERT HT_Book_Pro (B_NO=Book_ID, B_ROOM, B_NAME, B_UNIT, B_NUM, B_PRICE, B_PRICE_TOTAL, B_PRO_ID)`.

### 3.5 Cancel Booking (from FrmShowBookNotify cancel button)

```
Step 1: UPDATE HT_Book_H SET Book_Status='ยกเลิก' WHERE Book_ID=<id>
Step 2: UPDATE HT_Book_ds SET Book_status=3 WHERE Book_No=<id>
Step 3: Module1.SET_STATUS_BOOKING(BOOK_NO, true)
    -> If deposit was paid: refund via Insert_Pay (negative cash/credit), UPDATE_MONEY(..., "DEL").
    -- The HT_Book_Date and HT_Book_Ds rows are NOT deleted; they're just marked.
```

### 3.6 Cancel Booking (from ClickBook cancel-on-room button)

This path is more aggressive — it actually deletes:
```
For each affected room:
    UPDATE HT_Rooms SET room_book_ds='', Room_Book='', Room_Book_Name='', Room_Book_Time='' WHERE id=<roomid>
    UPDATE HT_Book_Date SET Book_ok=1 WHERE id=<bookdateid>  -- mark-cancel that day
    DELETE FROM HT_Book_Date WHERE book_type=<type> AND book_no=<bookid>
    DELETE FROM HT_Book_Ds WHERE book_room_type=<type> AND book_no=<bookid>
    Module1.SET_STATUS_BOOKING(<bookid>)  -- refund deposit + cleanup HT_Book_H accordingly
```

### 3.7 Edit Booking (FrmAddBook2.SAVE_EDIT) — full rewrite pattern

```
Step 1: update_cust()  -- update HT_Customers fields
Step 2: read existing HT_Book_H (Book_Price_Pay, Book_Cust_ID)
Step 3: If pay-amount changed:
    if new_pay>0: ShowDialog FormConfirmPay
    if old_pay<>0: refund existing payment via Insert_Pay (negated values), UPDATE_MONEY(..., "DEL")
Step 4: Reverse rooms:
    UPDATE HT_Rooms SET room_book_ds='', Room_Book='', Room_Book_Name='', Room_Book_Time=''
        WHERE Room_Book IN (SELECT id FROM HT_Book_Date WHERE Book_no=<edit_id>)
Step 5: DELETE FROM HT_Book_Date WHERE Book_no=<id>
        DELETE FROM HT_Book_H WHERE Book_ID=<id>
        DELETE FROM HT_Book_Ds WHERE Book_no=<id>
        DELETE FROM HT_Book_Pro WHERE [B_NO]=<id>
Step 6: Re-INSERT all (same as 3.4 create flow).
```
This **delete-then-reinsert** pattern preserves Book_ID but **changes
HT_Book_Date.id and HT_Book_Ds.id** (because they're regenerated). Any other table
that holds those ids (HT_Rooms.Room_Book) is also updated/cleared.

### 3.8 Sell Product to Room (in-stay POS)

Source: `FrmAddSale.cs:4015-4120`, ClickUSE → "ขายสินค้าเข้าห้อง".

```
Step 1: UPDATE HT_CheckIn_H SET Total_Price_vat += vat_amount WHERE Cin_no=<cin>
Step 2: id = get_id("HT_Receipt_H","id")
        INSERT HT_Receipt_H (id, Receipt_no=GetSIR(), Receipt_Date, Receipt_Name, ...,
                             Receipt_VatIn='True'/'False', Receipt_VatPer, status_name='ปกติ',
                             Receipt_ref=<cin>, Receipt_c_no=<custno>, Receipt_cin_vat_before=<n>,
                             Receipt_note, Receipt_Tax, Receipt_noteUP)
Step 3: For each line item:
        INSERT HT_Receipt_Ds (S_Sale_id=<receipt_h.id>, S_Product_no, S_Product_name,
                              S_Unit, S_UnitName, S_Price, S_Total, S_PriceDiscount_per,
                              S_PriceDiscount)
Step 4: If pay>0:
        Insert_Pay(Receipt_no, ref, ...)
Step 5: If overcharge VAT > 0:
        Insert_Pay extra row "ค่าออกภาษีส่วนเกิน"
Step 6: Print_Report.Print_SaleVat / Print_Sale
```
> **No INSERT into `HT_CheckIn_Product`** in this path. The check-in folio reads sales
> from `HT_Receipt_H/Ds` joined by `Receipt_ref=Cin_no`. If the new app issues a
> sale through this path, it must NOT also write HT_CheckIn_Product (would double-count).

### 3.9 Add Payment / Add Product to Folio (FrmPayAdd / FrmPayAddPro)

```
Step 1: For each updated line in Grid1 (rooms):
    UPDATE HT_CheckIn_Ds SET Cin_Room_Pay_Total=<n>, Cin_note=<note> WHERE id=<dsid>
    if pay>0: Insert_Pay(...)
Step 2: For each NEW product in Grid2 (FrmPayAddPro only):
    INSERT HT_CheckIn_Product
    UPDATE HT_Products SET Pro_Amt -= num
    if pay>0: Insert_Pay(...)
Step 3: UPDATE HT_CheckIn_H SET Total_Price_Pay+=, Total_Price_Balance-= WHERE Cin_no=<cin>
Step 4: UPDATE_MONEY → HT_Customers, HT_Log_Debt
```

### 3.10 Take Deposit (standalone, FrmAddDep)

```
Step 1: id = get_id("HT_Deposit","id")
        INSERT HT_Deposit (id, Dep_no=GetSIR(), Dep_Date, Dep_Room, Dep_Name, Dep_Price,
                          Dep_Status='รับเงินมัดจำแล้ว', Dep_ref=<cin or book>)
Step 2: Print_Report.Print_Dep
```
NOTE: Insert into `TB_Pay_History` may also be done depending on flow — verify in code path.

### 3.11 Issue VAT Invoice (separate from sale receipt)

Source: `FrmAddInvoiceSale.cs:3672-3760`.

```
Step 1: UPDATE HT_CheckIn_H SET Total_Price_vat += vat_amount WHERE Cin_no=<ref>
Step 2: id = get_id("HT_Invoice_H","id")
        INSERT HT_Invoice_H (id, Receipt_no=GetSIR(), ..., status_name='ปกติ',
                             Receipt_ref=<cin>, Receipt_c_no=<custno>)
Step 3: For each line: INSERT HT_Invoice_Ds (S_Sale_id=invoice_h.id, ...)
Step 4: Print_Report.Print_INVVat
```

### 3.12 Issue Booking Invoice (FormBookingInvoice — rarely used)

```
Step 1: DELETE FROM HT_INVOICE WHERE INV_booking_no=<book_id>  -- clear prior
Step 2: INV_NO = get_id("HT_INVOICE","INV_NO")
        INSERT HT_INVOICE (INV_NO, INV_booking_no, INV_DATE=now, INV_BY=loginName,
                          INV_TITLE, INV_NAME, INV_COMPANY, INV_ADDRESS, INV_TEL,
                          INV_NIGHT, INV_PAX, INV_PAX_CHILD, INV_PAYMENT, INV_DUEDATE,
                          INV_NOTE, INV_STAY)
Step 3: Print_Report.print_inv_booking
```

### 3.13 Mark Room Dirty (housewife start cleaning)

Source: `ClickClean.cs:493-540`.

```
For each affected room:
    Read latest checkout: SELECT TOP 1 cin_no, cin_cust_name FROM View_CheckIn_Ds
                          WHERE Cin_room_status='Check-Out' AND cin_room_no=<r>
                          ORDER BY cin_room_out DESC
    UPDATE HT_Rooms SET Room_Clean='no', Room_Clean_Time='' WHERE id=<roomid>
    INSERT HT_Housewife (h_name=<emp>, h_room=<r>, h_date=now, h_note=<note>,
                         h_cin=<latest_cin>, h_cin_name=<latest_custname>)
    Module1.Power_set(<r>, "OFF", "", "ปิดไฟจากปุ่มทำความสะอาดเรียบร้อย")
```

### 3.14 Mark Room Clean Done (ClickCleanOK)

```
For each affected room:
    UPDATE HT_Rooms SET Room_Clean='no', Room_Clean_Time='' WHERE id=<roomid>  -- (idempotent)
    UPDATE HT_Rooms SET Room_Clean_Time=<now.ToOADate()> WHERE Room_no=<r>
    INSERT HT_Housewife (...)
```
(Note: the WHERE id=<roomid> is the structural mark; Room_Clean is set to 'no' but
Room_Clean_Time is set to NOW. The naming is confusing but `Room_Clean='no'` here means
"awaiting verification" (visual: red→green pending).)

### 3.15 Send Room to Maintenance (ClickClean.ButtonX6)

```
For each affected room:
    UPDATE HT_Rooms SET Room_Clean='no', Room_Manternace='yes' WHERE id=<roomid>
    INSERT HT_Housewife (h_note='เปลี่ยนสถานะเป็นซ่อม : <note>', ...)
    Module1.INSERT_REPAIR(room_no, by, note)  -- INSERT HT_Rooms_Repair
```

### 3.16 Return Room from Maintenance (ClickManternance)

```
UPDATE HT_Rooms SET Room_Clean='no', Room_Manternace='no' WHERE id=<roomid>
```

### 3.17 Change Room Mid-Stay (ClickUSE → Change Room)

Source: `Module1.Change_Room`.

```
Read current HT_CheckIn_Ds row for from_room.
INSERT HT_Changed_Room (cin_no, room_before, room_after, change_date=now,
                         room_before_price, Note, ToPrice)
[Caller updates HT_CheckIn_Ds.Cin_Room_No, HT_Rooms flags for both rooms]
```

### 3.18 Cancel Check-in / Reverse a Stay (ClickUSE.cs:1500+)

```
For each room in checkin:
    UPDATE HT_Rooms SET Room_Clean='yes', Room_Use='no' WHERE room_no=<r>
UPDATE HT_CheckIn_H SET Total_Price_*= ..., cin_status='ยกเลิก' WHERE Cin_no=<cin>
[Cascade: HT_CheckIn_Pay rows already get cin_status='ยกเลิก' via WHERE clause]
```

### 3.19 Cancel Receipt (FrmReceiptMain.cs:2679)

```
UPDATE HT_CheckIn_Pay SET cin_status='ยกเลิก' WHERE cin_no=<receipt_no>
UPDATE HT_CheckIn_H SET Total_Price_vat -= <receipt_cin_vat_before> WHERE Cin_no=<receipt_ref>
UPDATE HT_Receipt_H SET status_name='ยกเลิก' WHERE id=<id>
```

### 3.20 Open Round-Bill (FrmDueBill open)

```
INSERT HT_Round_Bill (id=get_id, round_start=now, round_price=<float>, round_by=loginName)
   [round_end stays NULL]
```

### 3.21 Close Round-Bill

```
UPDATE HT_Round_Bill SET round_end='<now>', round_by=<emp> WHERE round_end IS NULL
```

### 3.22 Add Sticky Note to Room / Employee

```
INSERT HT_Room_SMS (SMS_Room=<r>, SMS_Details=<text>, SMS_By=loginName, SMS_Readed='no')
   [or HT_EMP_SMS with SMS_TO=<emp_username>]
```

### 3.23 Power On/Off a Room (Module1.Power_set + save_power_log)

```
[hardware command sent via SerialPort or HTTP queue]
UPDATE HT_Rooms SET Room_Power_STATUS='on' or 'off' WHERE room_no=<r>
if going ON and no open log row:
    INSERT HT_POWER_LOG (ROOM_NO, ROOM_POWER_START=GETDATE(), ROOM_POWER_START_BY=loginName,
                         ROOM_POWER_END_BY='', ROOM_POWER_NOTE=<note>, ROOM_POWER_NOTE2='')
if going OFF:
    UPDATE HT_POWER_LOG SET ROOM_POWER_END=GETDATE(), ROOM_POWER_END_BY=loginName,
                            ROOM_POWER_NOTE2=<note>
        WHERE room_no=<r> AND ROOM_POWER_END_BY=''
```

### 3.24 Delete Customer (FrmManageCustomersNew)

```
DELETE FROM Tb_Save_Image WHERE cust_no IN (SELECT cust_no FROM HT_Customers WHERE id=<id>)
DELETE FROM HT_Customers WHERE id=<id>
UPDATE HT_CheckIn_H SET Cin_cust_no='C0000' WHERE Cin_cust_no=<delcust_no>
UPDATE HT_CheckIn_Pay SET Cin_Cust_no='C0000' WHERE Cin_Cust_no=<delcust_no>
UPDATE HT_Book_H SET Book_Cust_ID='C0000' WHERE Book_Cust_ID=<delcust_no>
UPDATE HT_Bill_Debt_H SET Bill_Cust_ID='C0000' WHERE Bill_Cust_ID=<delcust_no>
UPDATE HT_Invoice_H SET Receipt_c_no='C0000' WHERE Receipt_c_no=<delcust_no>
UPDATE HT_Receipt_H SET Receipt_c_no='C0000' WHERE Receipt_c_no=<delcust_no>
```

### 3.25 Soft-delete vs Hard-delete

- **Soft-delete**: HT_CheckIn_H (Cin_status='ยกเลิก'), HT_Book_H (Book_Status='ยกเลิก'),
  HT_Receipt_H (status_name='ยกเลิก'), HT_Invoice_H (status_name='ยกเลิก'),
  HT_CheckIn_Pay (cin_status='ยกเลิก'), HT_Book_Ds (Book_status=3), HT_Cupon (no cancel
  semantic, just `cupon_print=1`).
- **Hard-delete** (DELETE FROM):
  - On edit: HT_Book_H/Ds/Date/Pro (delete-then-reinsert pattern — Book_ID preserved).
  - On edit checkin: HT_CheckIn_Other_People, HT_CheckIn_Product, HT_Room_Status (then re-insert).
  - On startup: HT_Room_Status WHERE room_status='Check-Out', Tb_Save_Image stale orphans,
    HT_Book_Date older than 60 days.
  - On factory reset (admin button): all HT_* booking/checkin/receipt tables wiped.
  - On customer delete: HT_Customers, Tb_Save_Image (per the customer).
  - On master-data edit: HT_Rooms, HT_Products, HT_Rooms_Price, HT_Products_Price,
    HT_SET_*, TB_SET_*, HT_Order_Up/Down (per Cust_Type), HT_ContinueTime.
  - HT_Bill_Debt_Ds on edit `delete from HT_Bill_Debt_Ds where Bill_No=<id>`.
  - HT_INVOICE on edit `delete from HT_INVOICE where INV_booking_no=<id>`.
  - HT_Invoice_Ds on edit `delete from HT_Invoice_Ds where S_Sale_id=<id>`.
  - HT_Receipt_Ds on edit `delete from HT_Receipt_Ds where S_Sale_id=<id>`.
  - TB_FOLIO on save `delete from TB_FOLIO where NO=<cin>`.
  - TB_MRP_Permission on save `delete from TB_MRP_Permission where Level_Name=<level>`.
  - TB_SMS_FAVORITES_2 (out of schema list) on FormSMSSendManual.

---

## 4. ID Generation Patterns (race-condition register)

| Table.Column | Format | Generator | Race-prone? | Recommendation |
|---|---|---|---|---|
| `HT_Customers.id` | int (sequential) | `get_id` (MAX+1) | Yes (no UNIQUE) | Wrap in transaction with UPDLOCK on table, or migrate to IDENTITY. |
| `HT_Customers.Cust_no` | `C{4digit}` | `MAX(id)+1` | Yes | Same as above — derived from id. |
| `HT_Book_H.Book_ID` | `R{6digit}` | `MAX(replace('R',''))+1` | Yes | Wrap in transaction. |
| `HT_CheckIn_H.Cin_no` | `CH{yy}-{6digit}` | per-year MAX+1 | Yes | Wrap; or reserve via counter table. |
| `HT_CheckIn_Ds.id` | int | `get_id` (MAX+1) **at start of loop, +1 per iter** | Very (multi-row insert in unprotected loop) | Critical fix needed: use IDENTITY (alter schema) or reserve range. |
| `HT_Room_Status.id` | int | `get_id` MAX+1 | Yes | Same. |
| `HT_Book_Date.id` | int | `get_id` MAX+1 | Yes | Same. |
| `HT_Cupon.cupon_no` | int | `get_id` MAX+1 (in loop) | Yes | Same. |
| `HT_Receipt_H.id` | int | `get_id` (NOT IDENTITY in this DB) | Yes | Wrap. |
| `HT_Receipt_H.Receipt_no` | `B/SB/CB{yyMM}-{4digit}` | per-month MAX | Yes | Wrap. |
| `HT_Invoice_H.id` / `Receipt_no` | same | per-month MAX | Yes | Wrap. |
| `HT_INVOICE.INV_NO` | int | `get_id` MAX+1 | Yes (but only 2 rows in production) | Low risk in practice. |
| `HT_CheckIn_Pay.Pay_no` | `R{yyMM}-{4digit}` | per-month MAX from Pay_no parsing | Yes | Wrap. |
| `HT_Bill_Debt_H.Bill_No` | `B{yyMM}-{4digit}` | per-month MAX | Yes (collides with Receipt_no namespace!) | **Note**: `B…` prefix is shared with HT_Receipt_H.Receipt_no — they live in different tables but a human would confuse them. The new app must keep them in their own namespaces. |
| `HT_Deposit.Dep_no` | `DEP{yyMM}-{4digit}` | per-month MAX | Yes | Wrap. |
| `HT_Round_Bill.id` | int | `get_id` | Low (only one open at a time anyway) | OK. |
| `HT_Rooms.id` | int | `get_id` (room admin only — low concurrency) | Low | OK. |
| `HT_Rooms_Cancel.id` | int | `get_id` | Yes | Wrap. |
| `TB_FOLIO.id` | int | `get_id`, then +1 per row | Yes | Wrap whole save. |
| `TB_Pay_History.id` | int | `get_id` | Yes | Wrap. |
| `HT_Order_Up.id` / `HT_Order_Down.id` | int 1..n per Cust_Type | row-index | No (delete-then-reinsert pattern) | Safe. |

**Compatible coexistence approach**: For each Cin_no/Pay_no/Receipt_no/Bill_No/Dep_no
counter, the new app should:
1. `BEGIN TRAN; SELECT max with UPDLOCK,HOLDLOCK; INSERT; COMMIT;` — both apps see a
   serialized counter view. **Collation note**: SQL Server's UPDLOCK on a non-existent row
   doesn't help; must lock the *table* with TABLOCKX or use a counter row.
2. **Better**: Add a counter table `HT_Sequence (name varchar PK, value int)` and use
   `UPDATE HT_Sequence SET value=value+1 OUTPUT INSERTED.value WHERE name='Cin_no_2025'`.
   But the OLD APP doesn't read this table, so it would still race. Therefore option (1)
   with `WITH (TABLOCKX, HOLDLOCK)` on the parent table is the only fully-compatible
   approach.
3. **For `HT_CheckIn_Ds.id` specifically**: the old app's `num++` in a loop is dangerous.
   The new app should call `get_id` at the start with `WITH (TABLOCKX, HOLDLOCK)` and
   commit only after the entire batch is inserted. This blocks the old app briefly but
   no data corruption. Or migrate the column to IDENTITY (breaking change).

---

## 5. Denormalization Map

> The schema heavily denormalizes for query performance. The new app MUST keep these in sync.

| Source-of-truth | Denormalized copy(ies) | Sync trigger |
|---|---|---|
| `HT_Customers.Cust_no` | `HT_CheckIn_H.Cin_cust_no`, `HT_CheckIn_Pay.Cin_Cust_no`, `HT_Book_H.Book_Cust_ID`, `HT_Bill_Debt_H.Bill_Cust_ID`, `HT_Invoice_H.Receipt_c_no`, `HT_Receipt_H.Receipt_c_no`, `HT_Deposit` (no FK col, written by name only) | On customer delete → all set to `'C0000'`. On rename → name fields updated separately (see below). |
| `HT_Customers.Cust_name` | `HT_CheckIn_H` does NOT copy name; reads via JOIN. BUT `HT_Book_H.Book_Cust_Name`, `HT_Receipt_H.Receipt_Name`, `HT_Invoice_H.Receipt_Name`, `HT_Bill_Debt_H.Bill_Cust_Name`, `HT_Deposit.Dep_Name`, `HT_Housewife.h_cin_name`, `HT_Room_Status.room_Details` (display), `HT_Rooms.Room_Book_Name` | NOT auto-synced. Old app does NOT update these on customer rename — they're copies frozen at write time (this is intentional for receipts to retain printed data). |
| `HT_Customers.Cust_Add_*` | `HT_Receipt_H.Receipt_Address`, `HT_Invoice_H.Receipt_Address`, `HT_Bill_Debt_H.Bill_Cust_Address` | Frozen copy on doc creation. |
| `HT_CheckIn_H.Cin_no` | `HT_CheckIn_Ds.Cin_No`, `HT_CheckIn_Pay.Cin_No` (sometimes Receipt_no!), `HT_CheckIn_Product.Cin_No`, `HT_CheckIn_Other_People.Cin_no`, `HT_Room_Status.room_CheckIn_No`, `Tb_Save_Image.cin_no`, `HT_Cupon.cupon_cin_no`, `HT_Changed_Room.cin_no`, `HT_Housewife.h_cin`, `HT_Invoice_Note.Cin_no`, `TB_FOLIO.NO`, `HT_Rooms_Cancel.cin_no`, `HT_Rooms_Repair` (no FK, but indirectly via room) | All written explicitly. On Cin_no being an immutable string, no rename problem — but on DELETE of HT_CheckIn_H all these can be left orphan unless cascading manually. |
| `HT_Book_H.Book_ID` | `HT_Book_Ds.Book_No`, `HT_Book_Date.Book_no`, `HT_Book_Pro.B_NO`, `HT_Room_Status.room_Book_No`, `HT_Rooms.Room_Book` (varchar of int id, pointing to `HT_Book_Date.id`), `HT_CheckIn_H.Cin_Book_no` | Manual cascade on delete-edit. |
| `HT_Rooms.Room_no` | Used as foreign key in 17 columns by name across HT_*. Never renamed in practice; if needed, the new app must update all of them. |
| `HT_Receipt_H.id` | `HT_Receipt_Ds.S_Sale_id` (parent.id copy) |
| `HT_Invoice_H.id` | `HT_Invoice_Ds.S_Sale_id` |
| `HT_Bill_Debt_H.Bill_No` | `HT_Bill_Debt_Ds.Bill_No` |
| `HT_Book_Date.id` | `HT_Rooms.Room_Book` (varchar of the int) | Cleared to `''` when booking cancelled or converted to checkin. |
| `HT_CheckIn_Ds.Cin_Room_Pay_Total` | aggregated up to `HT_CheckIn_H.Total_Price_*` | Sum maintained on every payment / sale change. |
| `HT_Products.Pro_Amt` | aggregated stock-in/stock-out from HT_CheckIn_Product, HT_Bill_Debt_Ds | Decrement on insert, increment on delete/refund. |
| `HT_Customers.Cust_Price_Over` | running balance — affected by Insert_Pay (sale debt), UPDATE_MONEY | Atomic: every UPDATE_MONEY is paired with INSERT into HT_Log_Debt. |
| `HT_Room_Status.room_date_oa` | `room_date.Date.ToOADate()` as float | Maintained on every Room_Status insert. |

**The new app MUST**:
- On any write involving customer name on documents (receipts, bookings), use the
  *current* `HT_Customers.Cust_name` at write-time (frozen copy; OK to differ from current).
- On creating any HT_*_H row, copy the customer's address/phone/etc. (frozen copy).
- On every payment, recompute and update parent `HT_CheckIn_H.Total_Price_*`.
- On every product-line write, update `HT_Products.Pro_Amt`.
- On any change to `HT_Customers.Cust_Price_Over`, INSERT a row into `HT_Log_Debt` with
  log_price_From, log_price (delta), log_price_To.

---

## 6. Trigger-Equivalent Behaviors

The DB has **zero triggers**. The new app must replicate the following "in-app triggers":

### 6.1 On every payment write to HT_CheckIn_Pay
- Update parent `HT_CheckIn_H.Total_Price_Pay` and `Total_Price_Balance` accordingly.

### 6.2 On every sale to a check-in (HT_Receipt_H with Receipt_ref=Cin_no)
- Update `HT_CheckIn_H.Total_Price_vat += vat_amount`. On cancel, subtract.

### 6.3 On every product-line insert/delete to HT_CheckIn_Product or HT_Bill_Debt_Ds
- Update `HT_Products.Pro_Amt -= num` (insert) / `+= num` (delete/refund).

### 6.4 On every state change of HT_Rooms (clean/dirty/repair/check-in/out)
- `HT_Housewife` log row inserted (for clean/dirty/repair).
- `HT_POWER_LOG` row started/closed if power state changes.
- `HT_Rooms_Repair` row inserted (for repair).
- `HT_Rooms_Cancel` row inserted on room-cancel (Module1.SaveCancel).
- `HT_Changed_Room` row on room change.

### 6.5 On every UPDATE_MONEY (customer balance change)
- INSERT `HT_Log_Debt` with from/to/delta values.

### 6.6 On every customer delete
- Cascade-update 6 referencing tables to set Cust_no='C0000'.

### 6.7 On startup (NEVER skip these — they fix orphan state from interrupted writes)
- `DELETE FROM HT_Room_Status WHERE room_status='Check-Out'`.
- `DELETE FROM Tb_Save_Image WHERE cust_no='' AND tmp_no<>'' AND pic_date < (now-2 days)`.
- `UPDATE HT_Room_Status SET room_status='Check-Out' WHERE room_no IN (SELECT Room_no FROM HT_Rooms WHERE Room_Use='no') AND room_status='เข้าพัก'`.
- `DELETE FROM HT_Book_Date WHERE Book_date_ds < (now-60 days)`.

> If the new app runs alongside the old, it MUST tolerate (and ideally avoid duplicating)
> these startup cleanups. The old app's startup will silently delete data the new app may
> have just written if the new app doesn't follow conventions (e.g. writing
> `room_status='Check-Out'` to HT_Room_Status as a permanent value will get deleted on
> next old-app launch).

---

## 7. Things the New App Should NOT Touch

### 7.1 Tables to ignore (dead or read-only legacy)

- `HT_Book_H2`, `HT_Book_Ds2`, `HT_Book_Date2`, `HT_Book_Status` — legacy duplicates.
- `HT_Bank_Accounts`, `HT_Bank_Transfer` — schema only, no usage.
- `HT_Register` — only `FrmAddReg` reads, never writes.
- `TB_MRP_Permission_name` — empty/unused.
- `Cached*` — typed-DataSet plumbing, NOT real tables in DB. (Verified: not in
  `INFORMATION_SCHEMA.TABLES`.)
- `View_*` — these are SQL VIEWs, not base tables. Read-only by definition. The new app
  may read but must NOT issue DDL to recreate them (frmMain1.cs does this on version-bump
  startup; let the old app handle).

### 7.2 Columns that are vestigial / unused

- `HT_Book_H.Book_room_note` — written but rarely read.
- `HT_Book_H.Book_room_all` — string-concat list; redundant with HT_Book_Ds.
- `TB_SETTINGS.login_url` — always blank; experimental.
- `HT_Rooms.Room_Polity` — set to int but never read (rumored "priority" feature).
- `HT_CheckIn_H.Cin_Work_number` — written `0`, never read.
- `HT_Invoice_Note.NOTE_STATUS` — never written.
- `TB_FOLIO.F_STATUS` — never written.
- `Tb_Version.V_NO` — managed by old app's startup migrations; do NOT touch.

### 7.3 Online endpoints / license to avoid

- `http://www.kpsystem.co.th/version_hotel.php` — version check.
- `http://www.kpsystem.co.th/chk_hotel.php` — **kill switch** (closes app remotely).
- `http://www.kpsystem.co.th/sms/sms.php` — SMS gateway.
- The `string_0` MAC whitelist in Module1.
- `HKLM\Software\microsoft\MSXKPHTEL` registry key.
- `reg.txt` file in install dir.
- TripleDES key `ruj5de4` / `554683`.
- MS Access password `foreverbu`.

The new app should NOT replicate these unless explicitly required.

---

## 8. Open Questions and Gotchas

### 8.1 Schema-vs-code discrepancies

1. **`HT_Receipt_H.id` is_identity**: Schema (`_SCHEMA.sql`) shows it as plain int, and
   `FrmAddSale.cs:4021` does `get_id("HT_Receipt_H","id")` then INSERTs explicit [id].
   Live DB: `INFORMATION_SCHEMA` says `is_identity=0` for this column → matches code path.
   **The new app should INSERT explicit [id] from get_id** for HT_Receipt_H, HT_Invoice_H.
2. **`HT_Book_Ds.id` IS IDENTITY** in live DB but `FrmBookRooms.cs:2461` INSERTs with explicit [id].
   This INSERT would FAIL in MSSQL without IDENTITY_INSERT ON, OR it silently succeeds because
   FrmBookRooms is not actually used in the live app (FrmAddBook2 is the active path and it
   does NOT include [id]). **Recommendation**: DO NOT include [id] in HT_Book_Ds INSERTs.
3. **`HT_CheckIn_Ds.id` per `INFORMATION_SCHEMA` is `is_identity=0`** but inserts include [id].
   Confirmed live: column is `int NOT NULL` plain. Get_id pattern works.
4. **`HT_CheckIn_Pay.id` is IDENTITY** confirmed. Code omits [id]. OK.
5. **`HT_CheckIn_Product.id` is IDENTITY**. Code omits [id]. OK.

> **Action item**: run `sp_help` on each non-IDENTITY-but-might-be-IDENTITY table to verify
> exact behavior in the production DB *before* writing through the new app. If the new app
> assumes a column is IDENTITY but it isn't, inserts will fail. If it assumes it isn't but
> it is, IDENTITY_INSERT errors will occur.

### 8.2 Status values present in data but not all in code

- `HT_Bill_Debt_H.Bill_Status='ชำระแล้ว'` (paid) — referenced in some report code but not
  found in any explicit `update Bill_Status='ชำระแล้ว'` statement. Likely set by a payment
  flow not yet traced.
- The live DB has 0 rows in HT_Bill_Debt_H so this can't be verified. **Open question**.
- `room_status='เช่า'` referenced in dead form `FormSelectRoom`; likely never written by
  current code.

### 8.3 Date-format ambiguity

- Old app writes `'M/D/YYYY 00:00:00'` for date-range filters. This works only if SQL Server
  session has `LANGUAGE = us_english` (or equivalent MDY DATEFORMAT). Live DB connection's
  default LANGUAGE is unknown — *verify* via `SELECT @@LANGUAGE`. The new app should
  default to ISO format for safety.

### 8.4 The `HT_INVOICE.INV_NO` underuse

- Live DB has 2 rows. `FormBookingInvoice` is wired but rarely used. The form does
  `get_id("HT_INVOICE","INV_NO")` which returns 1 if empty, MAX+1 otherwise. Race-prone but
  low risk.

### 8.5 Concurrent-cleanup race

- `frmMain1` startup deletes `HT_Room_Status WHERE room_status='Check-Out'`. If the new
  app writes a `'Check-Out'` row and the old app launches, the row is gone. This is
  intentional — `'Check-Out'` is ephemeral by old-app design. **The new app must not store
  durable state in this row.**

### 8.6 Encoding pitfalls

- `varchar` columns with Thai_CI_AS use Windows-874 codepage. Sending UTF-8 / Unicode
  literals (`N'…'`) will result in fallback character `?` for every Thai character.
- `varbinary` columns (Tb_Save_Image.pic, TB_SETTINGS.Company_Image, HT_Bank_Transfer.Transfer_Slip)
  are written as `0x{HEXBYTES}` literals. The new app should use parameterized binary
  insertion (safer) — they read back identical.
- Thai text in `text` columns (HT_Book_H.Book_room_all, etc.) — `text` is deprecated in
  SQL Server but old app freely uses it. The new app may use `varchar(max)` for new tables
  but must keep `text` on existing.

### 8.7 The `Cin_Pay_Status='1'` literal vs default `((1))`

Schema says `Cin_Status varchar(50) NOT NULL DEFAULT ((1))`. The default `((1))` is an int
literal in MSSQL, but the column is varchar — MSSQL implicitly stringifies → `'1'`. Live
data confirms: `Cin_Pay_Status='1'` (string). The new app should write the literal `'1'`,
NOT `1`, to be safe in case the default ever gets re-evaluated.

### 8.8 Single-quote escaping

Old app does NOT escape `'` except in INV_Note/Room_Note/EMP_Note (where `'`→`"`).
**Outstanding**: any Thai customer name with apostrophes will silently fail to write under
the old app. The new app should escape `''` and tolerate reading either.

### 8.9 The `Branch` column on HT_CheckIn_Pay

`Branch` is filled from `FormConfirmPay.ComboBox1.Text` which is a free-text combo bound
to `TB_SET_Branch.name`. Empty string when no branch configured. Reports use it for
filtering. New app must populate consistently (e.g. always the user's home branch name) to
keep reports correct.

### 8.10 `HT_Book_Ds.Book_Room_No` vs `HT_Book_Ds.Book_Room_Type`

Looking at FrmAddBook (no-room) the column-list inserts use `[Book_Room_Type]` to hold the
*type*. FrmBookRooms (with-room) UPDATE writes `[Book_Room_No]` as a column — but
`Book_Room_No` is **not in the schema** (`HT_Book_Ds` has only `Book_Room_Type`). Either
this is a code error or the FrmBookRooms path was migrated to using the column name
`Book_Room_Type` throughout. Check FrmBookRooms behavior in live DB.

### 8.11 The two `Check-Out` spellings

Code paths writing `'Check-Out'` (with hyphen): FrmCheckIn deletion guard, ClickUSE,
ClickClean, frmMain1 startup, view filters.
Code paths writing `'Check Out'` (space): FrmCheckOut.cs:6246.
Both reads-on-equality-comparison. **The space-version write would NOT be selected by
hyphen-version reads**. This is a **bug** that probably manifests as: a fully checked-out
room shows status 'Check Out' in HT_Room_Status until the next app startup, when frmMain1's
DELETE wipes hyphen-version rows but leaves space-version rows untouched.
**The new app should**:
- Write `'Check-Out'` (with hyphen) consistently to match all read paths.
- Tolerate reading both forms.

### 8.12 IDENTITY / explicit-id mixed bag

The combination of (some HT_* tables having IDENTITY, others not) plus (some inserts
including [id], others not) means there is no single rule. The new app's data layer should
have a per-table policy table:
| Table | Identity? | Code includes [id] in INSERT? |
|---|---|---|
| HT_Customers | NO | YES (get_id) |
| HT_Rooms | NO | YES |
| HT_Room_Status | NO | YES |
| HT_Book_H | (no id col, Book_ID is varchar PK) | YES |
| HT_Book_Ds | YES | NO (correct path) / YES (broken path) |
| HT_Book_Date | NO | YES |
| HT_Book_Pro | YES | NO |
| HT_CheckIn_H | (no id, Cin_no is varchar PK) | NO (has Cin_no in column list, not id) |
| HT_CheckIn_Ds | NO | YES |
| HT_CheckIn_Pay | YES | NO |
| HT_CheckIn_Product | YES | NO |
| HT_CheckIn_Other_People | YES | NO |
| HT_Receipt_H | NO | YES |
| HT_Receipt_Ds | YES | NO |
| HT_Invoice_H | NO | YES |
| HT_Invoice_Ds | YES | NO |
| HT_INVOICE | NO | YES |
| HT_Cupon | NO | YES |
| HT_Deposit | NO | YES |
| HT_Bill_Debt_H | (no id, Bill_No is varchar PK) | N/A |
| HT_Bill_Debt_Ds | YES | NO |
| HT_Changed_Room | YES | NO |
| HT_Housewife | YES | NO |
| HT_POWER_LOG | YES | NO |
| HT_Rooms_Repair | YES | NO |
| HT_Rooms_Cancel | NO | YES (get_id) |
| HT_Round_Bill | NO | YES |
| HT_Order_Up / Down | NO | YES (positional) |
| HT_Products | NO | YES |
| HT_Products_Price | YES | NO |
| HT_Rooms_Price | YES | NO |
| HT_ContinueTime | YES | NO |
| HT_SET_RoomType / CusType / CusType_Main / ProductType / Sale | YES | NO |
| HT_Log | YES | NO |
| HT_Log_Debt | YES | NO |
| HT_Room_SMS / EMP_SMS | YES (SMS_ID) | NO |
| TB_FOLIO | NO | YES |
| TB_Pay_History | NO | YES |
| TB_MRP_EMPLOYEE | YES | NO |
| Tb_Save_Image | YES | NO |
| TB_SET_Branch / MyType2 / MyType2_2 / MyType3 | YES | NO |

Verified live via `INFORMATION_SCHEMA.COLUMNS.is_identity` — but each rewrite-team test
should reverify.

### 8.13 Read-vs-Write case mismatches

The old app freely mixes case in lowercase column references in WHERE clauses (e.g.
`where cin_no='X'` vs the actual column `Cin_no`). SQL Server resolves these via
collation. The new app should follow the same convention for SQL it generates (just don't
assume the schema is case-sensitive).

### 8.14 Float precision

All money columns are `float` (not `decimal`). This is risky for accounting rounding. The
old app routinely does decimal arithmetic in C# then assigns to `float` columns — small
precision loss can accumulate. The new app should NOT migrate columns to `decimal`
(breaking change) but can use `decimal` in C# code and accept tiny float-rounding on
storage. Watch for `=` comparisons on float — use `ABS(a-b)<0.005`.

### 8.15 Multi-row `INSERT ... VALUES` not used

The old app inserts row-by-row via Module1.connect (one statement, one row). The new app
can use multi-row inserts for performance but must split if mixing with old-app concurrent
reads (otherwise no diff).

### 8.16 No transactions span multi-statement cascades

If the new app uses transactions (recommended), the old app on the other side may
temporarily see partially-committed state. SQL Server's default `READ COMMITTED` isolation
plus the app's no-tx-pattern means this is already a normal state. Snapshot isolation
might break the old app if it relies on reading uncommitted writes — keep `READ COMMITTED`.

---

## 9. Quick-Reference: Status Literal Matrix

This is the **single most important coexistence section**. Every literal here must be
written exactly as shown.

| Column | Literal | Meaning |
|---|---|---|
| `HT_Rooms.Room_Clean` | `'yes'` / `'no'` | clean / dirty (default `'no'`) |
| `HT_Rooms.Room_Use` | `'yes'` / `'no'` | occupied / free |
| `HT_Rooms.Room_Manternace` | `'yes'` / `'no'` | repair / not |
| `HT_Rooms.Room_Power_STATUS` | `'on'` / `'off'` | electricity (lowercase) |
| `HT_CheckIn_H.Cin_status` | `'ปกติ'` / `'ยกเลิก'` | normal / cancelled |
| `HT_CheckIn_H.Cin_foreign` | `'True'` / `'False'` | (capitalized) |
| `HT_CheckIn_H.Cin_type` | `0` / `1` / `2` | day / hour / month |
| `HT_CheckIn_Ds.Cin_Room_Status` | `'ยังไม่เข้าพัก'` / `'เข้าพัก'` / `'Check-Out'` | tri-state |
| `HT_CheckIn_Ds.Cin_Dep_Status` | `'ยังไม่คืนค่ามัดจำ'` / `'คืนเงินแล้ว'` / `'ไม่เก็บค่ามัดจำ'` | deposit lifecycle |
| `HT_CheckIn_Pay.Cin_Status` | `'1'` (active) / `'ยกเลิก'` | (note: literal string `'1'`) |
| `HT_Book_H.Book_Status` | `'จอง'` / `'เข้าพัก'` / `'ยกเลิก'` | booked / in-stay / cancelled |
| `HT_Book_H.Book_Notify_Note` | `NULL` / `'ไม่แจ้งเตือน'` | notify on / off |
| `HT_Book_H.Book_room_type` | `1` / `2` | no-room / with-rooms |
| `HT_Book_Ds.Book_status` | `1` / `3` | active / cancelled (no `2`) |
| `HT_Book_Date.Book_USE` | `0` / `1` | unused / used (rarely toggled) |
| `HT_Book_Date.Book_ok` | `0` / `1` | OK / cancelled-day |
| `HT_Receipt_H.status_name` | `'ปกติ'` / `'ยกเลิก'` |
| `HT_Receipt_H.Receipt_VatIn` | `'True'` / `'False'` | (capitalized) |
| `HT_Invoice_H.status_name` | `'ปกติ'` / `'ยกเลิก'` |
| `HT_Bill_Debt_H.Bill_Status` | `'ปกติ'` / `'ยกเลิก'` / `'ชำระแล้ว'` (?) |
| `HT_Bill_Debt_H.Bill_Type` | `'เงินสด'` / `'เครดิต'` |
| `HT_Cupon.cupon_print` | `0` / `1` |
| `HT_Deposit.Dep_Status` | `'รับเงินมัดจำแล้ว'` / `'คืนเงินแล้ว'` |
| `HT_Room_Status.room_status` | `'จอง'` / `'เข้าพัก'` / `'Check-Out'` (with hyphen — write this!) | NB: `'Check Out'` (space) is a code bug, tolerate but do not write |
| `HT_Room_SMS.SMS_Readed` / `HT_EMP_SMS.SMS_Readed` | `'no'` / `'yes'` |
| `HT_Round_Bill.round_end` | `NULL` (open) / `<datetime>` (closed) |
| `HT_POWER_LOG.ROOM_POWER_END_BY` | `''` (still on) / `<emp_name>` (off) |
| `HT_Customers.Cust_no` | `'C0001'`+ / sentinel `'C0000'` (deleted/orphan) |

---

## 10. Key Sentinel Values

| Sentinel | Meaning | Tables |
|---|---|---|
| `'C0000'` | Customer was deleted; this row references a non-existent customer | HT_CheckIn_H/Pay, HT_Book_H, HT_Bill_Debt_H, HT_Invoice_H, HT_Receipt_H |
| `'P001'` | Special "product code" for room-rent payment lines (not a real product) | HT_CheckIn_Pay.Cin_Pay_Ds_ID |
| `''` (empty) | "Absent / not set" for varchar status columns | most HT_* varchar |
| `'1'` (varchar) | active payment | HT_CheckIn_Pay.Cin_Status default |
| Empty `Room_Book` | room is not booked | HT_Rooms |
| `room_status='Check-Out'` row | ephemeral, deleted on next app launch | HT_Room_Status |
| `tmp_no<>'' AND cust_no=''` | uncommitted upload, deleted after 2 days | Tb_Save_Image |

---

## 11. Acceptance Tests (recommended for the new app)

1. **Round-trip a check-in**: write a check-in via the new app; close+reopen the old app;
   verify the check-in appears with all rooms, products, payments, and that
   `HT_CheckIn_H.Total_Price_*` matches the sum of HT_CheckIn_Pay rows.
2. **Status-flag end-to-end**: walk-in check-in → verify HT_Rooms.Room_Use='yes',
   HT_Room_Status row exists with room_status='เข้าพัก'; check-out → verify Room_Use='no',
   Room_Clean='yes', and HT_Room_Status row gets room_status='Check-Out' (which the
   old-app startup will then delete — that is correct).
3. **Concurrent ID generation**: from two new-app sessions, simultaneously create
   check-ins; verify no two `HT_CheckIn_H.Cin_no` collide. (Fix: serialization required.)
4. **Customer delete cascade**: delete a customer in the new app; verify all 6 tables get
   `'C0000'` updates (and Tb_Save_Image rows deleted).
5. **Round-bill gate**: with no open HT_Round_Bill, ensure new app refuses sale (or at
   minimum warns).
6. **Old-app reads new-app data**: write a customer with apostrophe-free name in new app,
   then open in old app's FrmManageCustomersNew — name displays correctly. Repeat with a
   Thai name — verifies codepage.
7. **Power-log invariant**: ON-then-OFF a room twice from new app; verify exactly two
   completed HT_POWER_LOG rows and no row with ROOM_POWER_END_BY=''.
8. **The `'Check Out'` (space) bug**: simulate the old app's bad write; verify the new app
   tolerates it (reads it as checked-out).

---

*End of cheatsheet. Last updated 2026-04-26 by source review of decompiled-clean tree
and live DB on `localhost\db`.*
