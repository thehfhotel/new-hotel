# iHOTEL room-grid refresh mechanics — decompile extraction (2026-08-03/04)

> Extracted from `evergreen:/home/nut/new-hotel/legacy/Hotel-2018- V.1.45/_decompiled_clean/iHOTEL2025/`
> (see [`EVERGREEN_ARTIFACTS.md`](EVERGREEN_ARTIFACTS.md) for how to reach it). All line
> references are `FormRoomMain.cs` unless another file is named. Written for the
> legacy-stale-notification design (`docs/adr/0006-legacy-stale-notification.md`) — these
> findings are recorded NOWHERE else in-repo and are expensive to re-derive from the
> off-repo decompile, which is the whole reason this file exists. Every citation below was
> independently re-verified line-by-line against the live decompile on 2026-08-04 while
> writing this doc (not just transcribed from the originating investigation).

## Bottom line

While a receptionist is working in our app (or anything else) instead of iHOTEL, **iHOTEL's
own room grid does not auto-refresh at all** — not slowly, not eventually. The only thing
that can wake it up from outside is the receptionist herself clicking iHOTEL's "Refresh"
button, and that button has a side effect (destroys the in-progress multi-room selection).
There is no IPC surface anywhere in the decompiled binary that an external process could
use to trigger a refresh. See §2 and §6.

## 1. The refresh timer

`FormRoomMain.Timer1.Interval = 60560` ms (`:2133`) — not a round number, presumably an
artifact of the form designer. The tick handler is `Timer1_Tick` (`:5749`):

```csharp
private void Timer1_Tick(object sender, EventArgs e)
{
    if (!MSSQL.CodeErr && !ButtonX4.Checked)
    {
        DateTimePicker1.Value = DateTime.Now;
    }
}
```

Setting `DateTimePicker1.Value` fires `DateTimePicker1_ValueChanged` (`:5738`), which is a
one-line pass-through to `LoadRooms` (`:4680`) — the full grid rebuild (§4):

```csharp
private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
{
    LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
}
```

`LoadRooms` self-reschedules the timer partway through its own body
(`Timer1.Enabled = false;` / `Timer1.Enabled = true;`, `:5140-5141`), so the effective period
is worst-case ≈ 60.6 s **plus** however long the load itself takes — the interval doesn't
start counting again until the previous load has finished.

**Second gate not in the original brief, confirmed by re-reading the tick body above:**
`!ButtonX4.Checked` is a second precondition alongside `!MSSQL.CodeErr`. `ButtonX4` is the
"แก้ไขแผนผัง" (edit-layout) toggle button (text set at `:1839`) — the same mode CONTEXT.md's
**Layout-edit drag** glossary entry describes. So the timer is *also* suppressed for the
duration reception has the layout-edit mode open, independent of window focus.

## 2. The focus gate — the headline finding

```csharp
private void FormRoomMain_Activated(object sender, EventArgs e)   // :2796
{
    MSSQL.CodeErr = false;                                          // :2798
}

private void FormRoomMain_Deactivate(object sender, EventArgs e)  // :2801
{
    MSSQL.CodeErr = true;                                           // :2803
}
```

`MSSQL.CodeErr` is a static field, so this isn't scoped to `FormRoomMain` alone — anything
elsewhere in the process that also gates on it (§8, `FormRoomMain_ViewBook`) is affected
too. `MSSQL.cs:536` sets the same field `true` on any SQL execution error, so a connectivity
blip has the identical suppressing effect as losing focus, until the form is reactivated.

**Consequence, stated plainly: while iHOTEL is not the active window, its room grid never
auto-refreshes, for any reason, no matter how long it sits unfocused.** Since reception
works in our app when a writeback commits into legacy MSSQL, this focus gate — not the
60.6 s interval — is the dominant cause of the staleness this doc exists to address. A
receptionist who tabs back into iHOTEL after ten minutes away sees a ten-minute-stale grid
until the next tick fires, or until she clicks Refresh (§5).

## 3. Correction: the "10 s poll" is a myth

`frmMain1.cs:6273` sets `Timer1.Interval = 10000` (10 s) on the MDI parent shell — a
**different** `Timer1` from `FormRoomMain`'s. Its tick (`:7257`) calls `CHK_CON()`
(`:7269`):

```csharp
public void CHK_CON()
{
    if (MSSQL.conn.State == ConnectionState.Closed)
    {
        Timer1.Enabled = false;
        MyProject.Forms.connect_mssql.ShowDialog();
        Timer1.Enabled = true;
    }
}
```

That is the entire body. It checks whether the SQL connection object is closed and, if so,
pops the reconnect dialog. **It does not re-query anything, and it has nothing to do with
the room grid.** Any assumption that iHOTEL "polls every 10 seconds" for room-state changes
is wrong and should be retired — flag it explicitly when it comes up, don't just quietly
work around it. Note `docs/legacy-spike/findings.md` §2, §5 "Allocation strategy" and §6
"Test 2" also mention "10s", but in a completely unrelated context — the observed UI hitch
while our writeback holds a `TABLOCKX` lock during ID allocation. Don't conflate the two
"10s" facts; they share a number and nothing else.

## 4. What a refresh actually costs

`LoadRooms` (`:4680`) is a full teardown/rebuild — no diffing against what's already
rendered. Per call it issues **7 queries**, all via `Module1.connect` (string-concatenated
SQL, no parameters — house convention, see `COMPAT_CHEATSHEET.md` §1.2):

| # | Line | Query | Purpose |
|---|---|---|---|
| 1 | `:4698` | `SELECT * FROM HT_Rooms ORDER BY room_NO` | Every room row |
| 2 | `:4700` | `SELECT * FROM View_Room_All WHERE room_date=<selected date>` | Checked-in / pending-checkin rows for the picker date |
| 3 | `:4701` | `SELECT book_room_type, SUM(book_room_num) FROM View_Book_Ds2 WHERE book_status='จอง' AND (...) GROUP BY book_room_type` | Booking counts for the summary strip |
| 4 | `:4702` | `SELECT cin_no, cin_room_all, Cin_room_no, Total_Price_Balance, total_price_vat FROM View_CheckIn_Ds WHERE (...)` | Active/pending check-ins, balances |
| 5 | `:4703` | `SELECT SMS_Room, SMS_ID FROM HT_Room_SMS WHERE SMS_Readed='no'` | Unread room-note icon |
| 6 | `:4704` | `SELECT * FROM View_Room_All WHERE cin_room_status='เข้าพัก' ORDER BY cin_date_out DESC` | Occupied rooms, ordered by due-out |
| 7 | `:4733` | `SELECT * FROM HT_SET_RoomType` | Room-type summary counts |

And critically, **a refresh is not read-only**:

- **Power scheduling** (`UpdateSecond = 600` at `:4706`, gated on
  `DateAndTime.DateDiff(..., lasttime_update_pwer, DateTime.Now) >= UpdateSecond`): at most
  once per 600 s per `LoadRooms` call, it loops every row of query #1 and calls
  `Module1.Power_set2(Room_no, Room_Power_STATUS, Room_Power_OPEN, Room_Power_CLOSE)`
  (`:4723`) — a serial-port relay write, not a DB write, but a physical side effect (room
  power) triggered by what looks like a passive grid refresh.
- **`AutoAddBookingRooms(dataSet)`** (called `:5144`, defined `:5149`): for every room tile
  currently showing empty/clean/no-maintenance that also matches a pending booking pointer,
  it `UPDATE`s **`HT_Rooms`** (`room_book_ds`, `Room_Book`, `Room_Book_Name`,
  `Room_Book_Time` — `:5201`) and **`HT_Book_Date`** (`Book_ok = Book_ok + 1` — `:5202`). If
  it makes any such match, it recursively re-triggers itself: sets
  `Module1.IsListroom = false`, `DateTimePicker1.Value = DateTime.Now` (another `LoadRooms`
  pass), and calls `ClearCheck()` (`:5211-5213`).

**This is load-bearing for anyone considering automating a refresh from outside iHOTEL**
(see the ADR): triggering a refresh isn't a free, side-effect-free read. It can write
`HT_Rooms`/`HT_Book_Date` and toggle physical room power, and those writes round-trip back
through Change Tracking into canonical PostgreSQL like any other legacy write.

## 5. The only external lever, and its hazards

Manual refresh is the "Refresh" button, `ButtonX3` (`Text = "Refresh"`, `:1857`; property
wired `:378-397`) → `ButtonX3_Click` (`:5743`):

```csharp
private void ButtonX3_Click(object sender, EventArgs e)
{
    DateTimePicker1.Value = DateTime.Now;
    ClearCheck();
}
```

Same `DateTimePicker1` → `DateTimePicker1_ValueChanged` → `LoadRooms` path as the timer.
**Critically, this handler does not check `MSSQL.CodeErr` at all** — it works even while
`FormRoomMain` is not the active window (e.g., another MDI child or another app has focus),
which the timer (§1, §2) structurally cannot do.

But `ButtonX3_Click` also unconditionally calls `ClearCheck()` (`:4123`):

```csharp
public void ClearCheck()
{
    CHK_NUM = 0;
    CHK_Array.Clear();
    ButtonX6.Visible = false;
    // ...
}
```

This zeroes the multi-room tap-select state (`CHK_NUM`, `CHK_Array`) and hides the
"confirm selection" button (`ButtonX6`) — i.e. clicking Refresh **destroys whatever
in-progress multi-room selection reception had made**. The timer path does not call
`ClearCheck()` (§1's `Timer1_Tick` body has no such call), so the automatic tick is
selection-safe but focus-blind, and the manual button is focus-safe but selection-destructive.
There is no path that is both. There is also no `Keys.F5` handler anywhere in the
decompiled tree — zero hits for `Keys.F5` across all 308 files.

### 5b. Posted-click caveat: WinForms may swallow `Click` when the window is covered (2026-08-10)

Found while building the Stage-4 middleware action (verified against Microsoft's
published reference source for .NET Framework `Control.cs`, and confirmed unchanged
in current `dotnet/winforms`): WinForms' standard click path — `Control.WmMouseUp` —
raises the `Click` event only when
`StandardClick && STATE_MOUSEPRESSED && !IsDisposed && WindowFromPoint(PointToScreen(lParam)) == Handle`.
The last clause asks what is **actually visible at that screen point**. So a
`PostMessage(WM_LBUTTONDOWN/WM_LBUTTONUP)` pair delivered while iHOTEL is covered by
another window (our app, say) fires `MouseDown`/`MouseUp` but may NOT fire `Click` —
which is precisely the situation the whole refresh feature targets. It may still
work: `ButtonX` is hand-painted DevComponents, not `ButtonBase`-derived, and that
family commonly raises `Click` from its own unconditional `OnMouseUp` override — but
whether iHOTEL's build does is **unproven** until measured. `BM_CLICK` is not a
fallback (`ButtonBase.WndProc` is what handles it; `ButtonX` isn't one). Test with
`scripts/dev/ihotel-uia-harness.ps1` row F (obscured-window case), then once on a
real terminal. If it fails there, the lever changes to UIA `Invoke` /
`LegacyIAccessible.DoDefaultAction` — a design decision, not a patch.

## 6. The negative finding — no external trigger surface exists

Exhaustive `grep` across all 308 `.cs` files in `_decompiled_clean/iHOTEL2025/`, run
2026-08-03 and independently re-run 2026-08-04 while writing this doc (both zero hits):

| Search | Hits |
|---|---|
| `NamedPipeServerStream`, `PipeStream` | 0 |
| `System.Net.Sockets`, `TcpListener`, `HttpListener`, `new Socket(` | 0 |
| `FileSystemWatcher` | 0 |
| `SqlDependency`, `SqlNotificationRequest`, Service Broker, `OnChange` | 0 |
| `Mutex`, `EventWaitHandle`, `Semaphore`, `MemoryMappedFile`, Remoting, WCF, `ServiceHost` | 0 |
| `WndProc`, `DefWndProc`, `RegisterWindowMessage`, `WM_COPYDATA` | 0 |
| `RegisterHotKey`, `SetWindowsHookEx`, `NativeWindow` | 0 |

**Near-misses, ruled out on inspection** (each looks promising on a keyword match, isn't):

- `iCam.cs:35,84,89,92` — `SendMessageA` calls. Outbound Win32 messaging **to** an avicap32
  webcam child window (camera capture control), not an inbound IPC listener.
- `TwainHandler.cs:16,189` — the only `IMessageFilter` implementation in the tree. It's the
  TWAIN document-scanner driver's message pump (`Application.AddMessageFilter(this)` at
  `:189`); scoped to scanner events, not a general external-trigger surface.
- `frmReg.cs:1638-1778` — several `FtpWebRequest` instantiations. All outbound FTP client
  calls (update/activation checks against `stcord.no-ip.org`), not a listening service.
- `PIPE-PC-SQLEXPRESS.ini` (at the working-folder root, not inside the decompile) — a red
  herring on the filename alone. Contents: `<ServerIP>PIPE-PC\SQLEXPRESS</ServerIP>` — the
  literal machine name of a SQL Server Express install this config once pointed at.
  "PIPE-PC" is a hostname, not a named pipe.

**Conclusion: iHOTEL cannot be told to refresh by anything outside its own process.** There
is no socket, no named pipe, no filesystem watch, no SQL notification subscription, no
cross-process synchronization object, no window-message hook — nothing an external service
(our backend, a tray helper, anything) could signal into a running `HOTEL.exe` to make it
re-query. The only door in is a user click inside the process itself (§5).

## 7. The app's own refresh flag is unreachable

`Module1.IsListroom` (`public static bool`, declared `Module1.cs:176`, initialized `false`
at `:514`) is the closest thing iHOTEL has to an internal "something changed, please
refresh" flag — but it is a **static field in process memory only**: never persisted to the
DB, never written to a file, never exposed. It is:

- **Set `true`** at exactly three in-process sites: `FrmCheckIn.cs:9683`, `FrmCheckIn.cs:10174`,
  `FrmCheckOut.cs:6459` — i.e. only by this same process completing a check-in or check-out
  through its own UI.
- **Polled** by `FormRoomMain.cs`'s `Timer3` (`Timer3.Interval = 1000` — 1 s —
  `Timer3_Tick`, defined `:5870`, body `:5872-5879`), and equivalently by
  `FormRoomMainClean.cs:4029`, `FormRoomMainKichen.cs:3819`, and
  `FormRoomMain_ViewBook.cs:2212`.

`Timer3_Tick`'s body, for the record — note it does **not** check `MSSQL.CodeErr`, and
**it also calls `ClearCheck()`**, same selection-destroying side effect as the manual button
(§5):

```csharp
private void Timer3_Tick(object sender, EventArgs e)
{
    if (Module1.IsListroom)
    {
        Module1.IsListroom = false;
        DateTimePicker1.Value = DateTime.Now;
        ClearCheck();
    }
}
```

Being process-local and focus-independent, this is why a check-in completed on terminal A
refreshes A's own grid within about a second (§9) — but it is entirely unreachable from
outside the process, so it offers no hook for an external notifier either.

## 8. Sibling boards differ

`FormRoomMainClean.cs:1598` (housekeeping-restricted shell) and `FormRoomMainKichen.cs:1496`
(kitchen-restricted shell) both use `Timer1.Interval = 30000` (30 s) — faster than the main
board's 60.6 s, presumably because these are single-purpose views with less to load.

`FormRoomMain_ViewBook.cs:1001` (booking calendar/grid overlay) also sets
`Timer1.Interval = 30000`, but its tick is **dead code** (`:2155`):

```csharp
private void Timer1_Tick(object sender, EventArgs e)
{
    if (MSSQL.CodeErr)
    {
    }
}
```

An empty block. **The booking-view overlay never auto-refreshes, ever, focused or not** —
worth knowing if a future staleness fix is scoped to "the room grid" and someone assumes
this sibling view inherits it for free.

## 9. Cross-terminal: none

Each terminal (each running copy of `HOTEL.exe`) polls independently against the shared
MSSQL DB; there's no shared mutex, broadcast, or DB-mediated push notification between
terminals (consistent with §6's negative finding — the mechanism to build one doesn't
exist in the codebase). A check-in completed on terminal A:

- Refreshes **A** within ~1 s, via `IsListroom` (§7) — same-process only.
- Is invisible to terminal **B** for up to ~60.6 s (§1) — or **indefinitely**, if B is not
  the focused window (§2).

## 10. `FormRoomMain` is an MDI child, not top-level

`FormRoomMain` is opened as an MDI child of `frmMain1`
(`FEATURE_MAP.md:63` — "`FormRoomMain` (Show, MDI child)"; confirmed again in the boot-order
diagram at `FEATURE_MAP.md:430` — "`(normal) → FormRoomMain (MDI child, default)`"). This
matters for anyone doing Win32/UIA automation against the live process: `FormRoomMain` is
never the top-level window, so a WinForms modal dialog (e.g. the check-in/check-out forms)
disables `frmMain1`, the MDI **parent**, not the `FormRoomMain` child directly — window
enumeration and focus-state checks need to account for the MDI hierarchy, not just look at
the foreground HWND.

## 11. Why not fix this DB-side

Not considered further because it's foreclosed by two things already established elsewhere
in this repo, not because it wasn't thought of:

- **Legacy DDL is prohibited** (CLAUDE.md's legacy-database section) beyond the narrow,
  already-exhausted Change Tracking prerequisite carve-out in `migrations/legacy-mssql/`.
  A trigger or a notification mechanism added to the shared MSSQL is not that carve-out.
- **The legacy DB has zero triggers, stored procedures, or functions today**
  (`docs/legacy-spike/findings.md` §1 "Server & app fingerprint" — verified via
  `SELECT name FROM sys.triggers` returning 0 rows). iHOTEL's entire app assumes this —
  "no hidden side-effects; every state change is in the captured INSERT/UPDATE
  statements." Introducing one trigger to page reception would be the first hidden
  side-effect the shared DB has ever had, breaking an assumption every other part of this
  coexistence effort (byte-parity writeback, the spike's captured-statement methodology)
  depends on.

## See also

- [`docs/adr/0006-legacy-stale-notification.md`](../adr/0006-legacy-stale-notification.md)
  — the decision this file exists to support: notify reception rather than auto-refresh.
- [`FEATURE_MAP.md`](FEATURE_MAP.md) §3.2 — `FormRoomMain` in the wider form inventory.
- [`ROOM_STATUS_PALETTE.md`](ROOM_STATUS_PALETTE.md) — a comparable decompile extraction
  from the same file, for room-state colors rather than refresh timing.
- [`COMPAT_CHEATSHEET.md`](COMPAT_CHEATSHEET.md) §1.2 — the no-triggers, no-FKs,
  string-concatenated SQL conventions this file's findings are consistent with.
