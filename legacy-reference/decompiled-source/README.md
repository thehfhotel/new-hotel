# Hotel-2018 V.1.45 — Reference Codebase

> **What this is**: a cleaned, navigable copy of the decompiled `HOTEL-cleaned.exe`,
> meant as **read-only reference material** while you write a modern rewrite.
> It is **not** a maintained fork; it is **not** intended to run.
>
> **What this is NOT**: a working build of the app. The original depends on
> Crystal Reports for VS 2008 (32-bit, no clean .NET 8 story), DotNetBar2,
> ComponentOne FlexGrid, and a sister `KPThaiNationalIDCard.exe`. We made no
> attempt to make those dependencies work on a modern stack.

## Folder layout

```
_reference/
├── README.md                            ← you are here
├── _OBFUSCATOR_STUBS_REMOVED.md         ← what we deleted vs archived & why
├── _FEATURE_MAP.md                      ← copy of the per-module feature map
├── _COMPAT_CHEATSHEET.md                ← decompiler-artifact cheat sheet
├── HOTEL-cleaned.csproj                 ← project file (loads, mostly compiles)
├── _archived/                           ← duplicate / older copies (12 files)
└── src/
    ├── _ReferenceStubs.cs               ← scaffolding to make compile work
    ├── app.ico, app.manifest
    ├── HOTEL.exe.licenses
    ├── iHOTEL2025.<FormName>.resx       ← all per-form resources (top-level)
    ├── *.rpt                            ← all 47 Crystal Reports
    ├── Properties/AssemblyInfo.cs
    ├── iHOTEL2025.My/                   ← VB My namespace plumbing
    ├── iHOTEL2025.My.Resources/
    └── iHOTEL2025/                      ← 298 .cs files (the application)
```

## How to open it

**Best**: **JetBrains Rider** — it's the most forgiving with half-broken
projects. Open `HOTEL-cleaned.csproj` directly. F12 / Ctrl-Click navigation
works on every file even where the project itself fails to build.

**Also fine**: **Visual Studio 2022** (any edition). Same caveat — works
even with build errors.

**Workable**: **VS Code with the C# DevKit extension**. Open the
`_reference/` folder. The Roslyn workspace will index everything; navigation
works file-by-file.

You can also just `grep`/`rg` the `src/iHOTEL2025/` folder — files are flat,
filenames match class names exactly.

## The 3 hub forms — start here

These are the three biggest entry points; reading them gives you the
bulk of the application's flow.

1. **`src/iHOTEL2025/frmMain1.cs`** — 8500-line MDI parent. The ribbon,
   notifications, license check, idle logout, online version/kill-switch
   poll, serial-port relay logic, all live here. Search for
   `frmMain1_Load` (line ~6938) for the boot sequence — it runs the entire
   startup pipeline (config, license, DB picker, login, permissions, cleanup
   queries, online checks).

2. **`src/iHOTEL2025/FormRoomMain.cs`** — the room-status grid. The central
   hub that every room operation routes through: clicking a cell dispatches
   to `ClickAvliable.cs`, `ClickBook.cs`, `ClickClean.cs`, `ClickUSE.cs`,
   `ClickManternance.cs`, etc. depending on the room's current state.

3. **`src/iHOTEL2025/FrmCheckIn.cs`** — ~11,000 lines, 19 tables touched.
   The biggest individual transactional form: customer search, smart-card
   read, photo capture, room-price computation, products attached, deposit,
   signature, registration print. This is the canonical example of how a
   complete write-path works in this codebase.

## Data layer entry points

Everything reads/writes via `Module1.connect(string sql) -> DataSet`. There
is **no** repository, no parameterization, no abstraction.

- **`src/iHOTEL2025/Module1.cs`** — god module. Look at:
  - `connect(string sql)` (the only DB function the app uses)
  - `ReadDB_2018()` (opens the Access OleDb or SQL Server connection)
  - `Database_Mode` field ("ACCESS" vs "SQL"; flips `datechar` between
    `#` and `'` so dates fit either dialect)
  - `loginID`, `loginName`, `loginMode` globals (set by `login.cs`)
  - `string_0` — embedded MAC/serial whitelist (~250 entries) for licensing
  - `check_round_bill()` — gate function called by every transactional form
- **`src/iHOTEL2025/MSSQL.cs`** — SQL Server-specific connection helper +
  schema bootstrap (`Create_MssqlDatabase`).
- **`src/iHOTEL2025/GENDB.cs`** — older generic DB helper, partly superseded
  by Module1.

## Navigation index by module

Detailed per-form table touches and full per-form list lives in
`_FEATURE_MAP.md` (Section 5). The summary index below points at the
canonical entry forms for each functional module.

### Auth & session
- Login: `src/iHOTEL2025/login.cs`
- Permissions editor: `src/iHOTEL2025/FrmPermission.cs`
- User CRUD: `src/iHOTEL2025/FrmUser.cs`
- Admin trapdoor: `src/iHOTEL2025/FormPass.cs` → `src/iHOTEL2025/FormLog.cs`
- Tables: `TB_MRP_EMPLOYEE`, `TB_MRP_PERMISSION`, `HT_Log`

### Room grid (the hub)
- Main grid: `src/iHOTEL2025/FormRoomMain.cs`
- Booking calendar: `src/iHOTEL2025/FormRoomMain_ViewBook.cs`
- Housewife shell: `src/iHOTEL2025/FormRoomMainClean.cs`
- Kitchen shell: `src/iHOTEL2025/FormRoomMainKichen.cs`
- Time table: `src/iHOTEL2025/frmTimeTable.cs`

### Room cell click handlers (state-dependent dialogs)
- Mark available: `src/iHOTEL2025/ClickAvliable.cs` (and `_book` variant)
- Book: `src/iHOTEL2025/ClickBook.cs` (and `_book` variant)
- Clean (start / done): `src/iHOTEL2025/ClickClean.cs`, `ClickCleanOK.cs`
- Maintenance: `src/iHOTEL2025/ClickManternance.cs`
- In use (the big dispatcher, ~1900 lines): `src/iHOTEL2025/ClickUSE.cs`
  (also `ClickUSE2.cs`, `ClickUSE3.cs` for restricted variants)
- Add guest: `src/iHOTEL2025/ClickAddmore.cs`
- Sticky notes: `src/iHOTEL2025/Room_Note.cs`, `Room_Note_Read.cs`,
  `EMP_Note.cs`, `EMP_Note_Read.cs`

### Booking
- Entry (no specific room): `src/iHOTEL2025/FrmBookMain.cs`
- Entry (with rooms): `src/iHOTEL2025/FrmBookMain2.cs`
- Add booking: `src/iHOTEL2025/FrmAddBook.cs`, `FrmAddBook2.cs`
- Confirm + select rooms: `src/iHOTEL2025/FormBookRooms.cs`,
  `src/iHOTEL2025/FrmBookRooms.cs`
- Search: `src/iHOTEL2025/FrmSearchBook.cs`,
  `FormSearchBooking.cs`, `FormSearchBooking2.cs`
- Notification popup: `src/iHOTEL2025/FrmShowBookNotify.cs`
- Booking → invoice: `src/iHOTEL2025/FormBookingInvoice.cs`
- Tables: `HT_Book_H`, `HT_Book_Ds`, `HT_Book_Date`, `HT_Book_Pro`

### Check-in / Check-out
- Check-in (~11k lines): `src/iHOTEL2025/FrmCheckIn.cs`
- Edit-only check-in: `src/iHOTEL2025/FrmCheckIn_EditOnly.cs`
- Check-out: `src/iHOTEL2025/FrmCheckOut.cs`
- Confirm-pay modal (everywhere): `src/iHOTEL2025/FormConfirmPay.cs`
- Round-bill confirm: `src/iHOTEL2025/FormConfirmRoundBill.cs`
- Over-charge confirm: `src/iHOTEL2025/FormConfirmOverBill.cs`
- VAT screens: `src/iHOTEL2025/FormShowVAT.cs`, `FormVatOver.cs`
- Deposit apply / refund: `src/iHOTEL2025/FormShowDEP.cs`, `FormShowDEPBack.cs`
- Save-out room: `src/iHOTEL2025/FormShowSAVEout2.cs`
- Tables: `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_CheckIn_Pay`,
  `HT_CheckIn_Product`, `HT_CheckIn_Other_People`, `HT_Room_Status`,
  `HT_Customers`, `Tb_Save_Image`

### Customers
- Current master: `src/iHOTEL2025/FrmManageCustomersNew.cs`
- Legacy: `src/iHOTEL2025/FrmManageCustomers.cs`
- Pickers: `src/iHOTEL2025/FrmManageCustomersSearch.cs`,
  `FrmSearchCustomers.cs`, `FormSearchCust.cs`
- Smart-card preview: `src/iHOTEL2025/FrmShowPreviewSmartCard.cs`
- Tables: `HT_Customers`, `Tb_Save_Image`

### Sales / POS
- Receipt main: `src/iHOTEL2025/FrmReceiptMain.cs`,
  `FrmReceiptMain_invoice.cs`
- Add line: `src/iHOTEL2025/FrmAddSale.cs`,
  `FrmAddSale2.cs`, `FrmAddSale2_Credit.cs`
- Sale list: `src/iHOTEL2025/FrmSaleMain2.cs`
- Coupon: `src/iHOTEL2025/FrmCuponMain.cs`
- Tables: `HT_Receipt_H`, `HT_Receipt_Ds`, `HT_Bill_Debt_*`, `HT_Cupon`

### Payments / Folio / Debt
- Cash in/out main: `src/iHOTEL2025/FrmPayMain.cs`
- Add pay (no products): `src/iHOTEL2025/FrmPayAdd.cs`
- Add pay (with product): `src/iHOTEL2025/FrmPayAddPro.cs`
- Add pay against debt: `src/iHOTEL2025/FrmPayAddDebt.cs`
- Debt list (check-ins): `src/iHOTEL2025/FrmPayDebt.cs`
- Debt list (sales): `src/iHOTEL2025/FrmPayDebt2.cs`
- Add manual income/expense: `src/iHOTEL2025/FrmAddPay.cs`
- Add deposit: `src/iHOTEL2025/FrmAddDep.cs`
- Deposit list: `src/iHOTEL2025/FrmDepositMain.cs`
- Folio: `src/iHOTEL2025/FormFolio.cs`
- Round-bill: `src/iHOTEL2025/FrmDueBill.cs`
- Edit dates: `src/iHOTEL2025/FrmEditDate.cs`
- Day in/out: `src/iHOTEL2025/FrmInOutMain.cs`
- Edit a single pay row: `src/iHOTEL2025/FormEditPay.cs`
- Tables: `tb_pay_history`, `HT_Round_Bill`, `TB_FOLIO`, `HT_Log_Debt`

### Invoices / VAT
- Issue invoice: `src/iHOTEL2025/FrmReceiptInvoice.cs`
- Add line: `src/iHOTEL2025/FrmAddInvoiceSale.cs`
- Notes: `src/iHOTEL2025/INV_Note.cs`
- Tables: `HT_Invoice_H`, `HT_Invoice_Ds`, `HT_Invoice_Note`, `HT_INVOICE`

### Master data (rooms / products / types)
- Room CRUD: `src/iHOTEL2025/FrmManageRoom.cs`
- Room type / price: `src/iHOTEL2025/FrmSETRoomType.cs`
- Product CRUD: `src/iHOTEL2025/FrmManageProduct.cs`
- Product type: `src/iHOTEL2025/FrmSETProType.cs`
- Customer type: `src/iHOTEL2025/FrmSETCsuType.cs`,
  `FrmSETCsuTypeMain.cs`
- Cash type tree: `src/iHOTEL2025/FrmSETMyType2.cs`,
  `FrmSETMyType2_2.cs`, `FrmSETMyType3.cs`
- Sales rep: `src/iHOTEL2025/FrmSETsale.cs`
- Branch: `src/iHOTEL2025/FrmSETBranch.cs`
- Time-extension pricing: `src/iHOTEL2025/FrmSETTimeContnue.cs`
- Per-customer-type price overrides: `src/iHOTEL2025/FormManageOrderCust.cs`,
  `FormManageOrderCustDown.cs`
- Price history: `src/iHOTEL2025/FrmPriceHistory.cs`

### Reports (~30 launcher forms + 47 Crystal templates)
The `Frm*Report*.cs` and `Form*Report*.cs` files are simple
DateTimePicker-driven launchers; each invokes a `.rpt` template via
Print_Report.cs. See `_FEATURE_MAP.md` Section 3.12 for the full
launcher → report mapping.

The `Cached*.cs`, `Crystal*.cs`, `inv_*.cs`, `sale*.cs`, and ReportClass-typed
`Report*.cs` files are auto-generated typed-DataSet wrappers — they're on
disk for navigation but **excluded from build**.

### SMS
- SMS settings: `src/iHOTEL2025/FrmSettingsSMS.cs`
- Send manual: `src/iHOTEL2025/FormSMSSendManual.cs`
- Send log: `src/iHOTEL2025/FormSMSLog.cs`
- Debt-reminder send: `src/iHOTEL2025/FormSMS_DEBT.cs`

### Settings & admin
- Master settings (~7000 lines): `src/iHOTEL2025/FrmSettings.cs`
- DB connection picker: `src/iHOTEL2025/FormSelectDB.cs`
- Add server entry: `src/iHOTEL2025/FrmAddEditServer.cs`
- DDNS poller: `src/iHOTEL2025/FormUPDATE.cs`
- Bulk room-status fix: `src/iHOTEL2025/FormUpdateDateRoomAll.cs`
- Self-updater: `src/iHOTEL2025/FrmUpdate.cs`
- License registration: `src/iHOTEL2025/frmReg.cs`
- Loading splash: `src/iHOTEL2025/frmWanting.cs`
- About: `src/iHOTEL2025/AboutBox1.cs`

### Image capture
- Photo upload host: `src/iHOTEL2025/FrmAddSaveImage.cs`
- Webcam capture: `src/iHOTEL2025/frmCapture.cs`,
  `src/iHOTEL2025/iCam.cs`
- TWAIN scanner: `src/iHOTEL2025/Twain*.cs` (16 helper files)

### Plumbing / utilities
- Print orchestrator: `src/iHOTEL2025/Print_Report.cs`
- Print device factory: `src/iHOTEL2025/PrintFactory.cs`
- Print queue/preview: `src/iHOTEL2025/FrmPrint.cs`
- Resize helper: `src/iHOTEL2025/ResizeableControl.cs`
- Custom data button: `src/iHOTEL2025/ButtonTable.cs`
- Number → Thai text: `src/iHOTEL2025/DecimalToText_TH.cs`
- Click-route helper: `src/iHOTEL2025/ModuleClick.cs`
- TripleDES helpers: `src/iHOTEL2025/Encrypt.cs`,
  `src/iHOTEL2025/FormEN_DE.cs` (used to obfuscate SQL passwords in
  `server.txt`)
- In-memory config cache: `src/iHOTEL2025/Datalocal.cs`

## Known limitations & remaining compile errors

This project loads in any modern IDE. **It does NOT build cleanly.**
Out of ~280 source files in the build, ~60 files emit ~280 errors. The
errors fall into these buckets:

| Error pattern | Count (approx) | Why |
|---|---|---|
| `'X' does not contain a definition for '_002Ector'` | ~90 | The decompiler emitted the IL token `..ctor` as the C# identifier `_002Ector`. The most common variant — `base._002Ector();` inside constructors — was patched project-wide (see _ReferenceStubs.cs comment); other forms (`new DataTable._002Ector()`, etc.) remain in `Datalocal.cs` and a few generated typed-DataSet code paths. |
| `Module1.id.get: cannot explicitly call operator or accessor` | ~88 | `Module1.id` is a 0-arg property; the decompiler turned dozens of call sites into `Module1.get_id("HT_Room_Status", "id")`-style calls (which would only be valid for a 2-arg method). The IL is fine; it's a round-trip artifact. |
| `'Print_Report' does not exist in the current context` | ~162 | `Print_Report.cs` was excluded from build (it heavily uses Crystal Reports types we can't reference on .NET 8). Code that imports it can't resolve the type. |
| `'CrystalReport*' could not be found` | ~80 | Same — the typed `CrystalReport*.cs` wrappers are excluded; their consumers don't build. |
| `'FrmPrint' does not contain a definition for 'CrystalReportViewer1'` | ~76 | `FrmPrint` is replaced by an empty stub (in `_ReferenceStubs.cs`) because the real one references `CrystalDecisions.Windows.Forms.CrystalReportViewer`. Reports forms that wire into it hit this. |

These errors do **not** block IDE navigation — Roslyn parses and indexes
each file independently. F12, Find Usages, Find in Files, and rename all
work normally even on files that contain errors.

### Files excluded from build (kept on disk for navigation)

- ~95 Crystal Reports plumbing files: `Cached*.cs`, `Crystal*.cs`,
  `inv_sale_*.cs`, `sale*.cs` (the ReportClass ones), and 20
  ReportClass-typed `Report*.cs` files.
- 7 forms that depend directly on `CrystalDecisions.*` types:
  `Print_Report.cs`, `FrmPrint.cs`, `FrmReportRR4.cs`, `ReportTax.cs`,
  `GClass3.cs`, `GClass4.cs`, `GForm0.cs`.
- 1 file with sealed-type override artifacts: `AboutBox1.cs`.
- The entire `_archived/` folder.

See the `<Compile Remove>` blocks in `HOTEL-cleaned.csproj` for the exact
list. Around 28 ReportClass-typed `.cs` files plus 47 of the `Cached*.cs`
typed-DataSet files are excluded — but the 10 `Office2007Form`-based
report-launcher forms (`ReportDays.cs`, `ReportDebt.cs`, `ReportTax.cs`
etc.) are NOT excluded; they're the actual launcher UIs.

### Files patched in-place (small decompiler-artifact fixes)

- `src/iHOTEL2025/ResizeableControl.cs` — removed an invalid
  `private virtual` modifier
- `src/iHOTEL2025/PrintFactory.cs` — same
- `src/iHOTEL2025/TwCapability.cs` — converted a method-with-the-class-name
  back into a constructor
- `src/iHOTEL2025.My/MyProject.cs` — added `using iHOTEL2025;` so the
  unqualified type references in this auto-generated VB My helper resolve
- 283 files: `base._002Ector();` → `// base._002Ector();  // REF: stripped`
  (decompiler noise — the C# compiler implicitly emits the parameterless
  base ctor call anyway)

Each patch is marked with a `// REFERENCE-CODEBASE PATCH:` comment in the
file so you can identify them. Full list in `_OBFUSCATOR_STUBS_REMOVED.md`.

### Files added by us (scaffolding, not original source)

- `src/_ReferenceStubs.cs` — global-namespace `Class2` stub (the .NET
  Reactor obfuscator's static-init hook, originally an empty no-op) plus
  empty Form stubs for the few types removed-from-build that
  `MyProject.cs` references by name.

## Caveats

- **Original language was VB.NET, not C#.** The decompile is from
  `HOTEL-cleaned.exe` after running it through ilspycmd. Expect to see
  VB-isms throughout: `Microsoft.VisualBasic.Operators.CompareString`,
  `Conversions.ToString`, `[StandardModule]` (VB Module marker),
  `[AccessedThroughProperty]`, `Strings.InStr`, `Information.IsDate`, etc.
  The `iHOTEL2025/Module1.cs` pattern of static fields + static methods
  with no class is exactly a VB `Module`. See `_COMPAT_CHEATSHEET.md` for
  a guide on translating these idioms back to idiomatic C# in your rewrite.
- **De4dot did most of the cleanup but didn't restore everything.** Names
  like `string_0`, `method_0`, `bool_0`, and the GUID-named class
  `_003CModule_003E_007BDE...}` are de4dot-renamed leftovers from the
  obfuscator. They have no semantic meaning beyond "the de4dot tool didn't
  know what to call them".
- **SQL has zero parameterization.** Every query in the codebase is
  built with string concatenation, and `Module1.datechar` (`#` for Access,
  `'` for SQL Server) is concatenated around date literals. **Do not
  replicate this in the rewrite** — it's a textbook SQL-injection vector.
  Use parameterized queries from day one.
- **Hardcoded production endpoints.** `frmMain1.cs` polls
  `http://www.kpsystem.co.th/version_hotel.php` (version check) and
  `http://www.kpsystem.co.th/chk_hotel.php` (kill switch — can disable
  the install remotely). `FrmSettingsSMS.cs` posts to
  `http://www.kpsystem.co.th/sms/sms.php`. The MS Access password
  `foreverbu` is embedded in `Module1.cs`. Plan to neutralize all of these
  before shipping any rewrite.
- **License whitelist.** `Module1.string_0` contains a comma-separated
  list of ~250 MAC addresses / disk serials. The original app cross-checks
  the local machine against this list at startup. Skip in the rewrite.
- **Two database backends.** The original supports both MS Access
  (`Provider=Microsoft.ACE.OLEDB.12.0`) and SQL Server, switching via
  `Module1.Database_Mode = "ACCESS" | "SQL"`. The schema is largely the
  same; only the date-literal quoting differs. The rewrite should pick
  one (probably SQL Server / PostgreSQL).
- **Crystal Reports.** ~47 `.rpt` files are checked in (they live at
  `src/*.rpt`). They're embedded resources in the original build. There
  is no clean modern replacement — most teams swap them out for either
  RDLC, QuestPDF, or a server-side reporting service. None of them
  contain business logic, just layout + simple SQL.
- **The 132 Form classes do roughly the work of 85 distinct functional
  forms** — the rest are duplicates / variants. See `_FEATURE_MAP.md`
  Section 8 ("Cruft / Skip in Rewrite") for the consolidation
  recommendations: e.g. `FrmReceiptMain` vs `FrmReceiptMain_invoice` are
  90% the same; `FrmAddSale` / `FrmAddSale2` / `FrmAddSale2_Credit` should
  collapse into one form with a mode enum; etc.
