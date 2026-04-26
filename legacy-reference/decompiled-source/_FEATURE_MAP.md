# Hotel-2018 V.1.45 — Feature Map for Rewrite Planning

> Source: `_decompiled_clean/iHOTEL2025/` (deobfuscated VB.NET → C# decompile)
> ~308 files, ~132 Form classes, 61 DB tables (`HT_*`, `TB_*`).
> UI: DevComponents DotNetBar (Office-style ribbon), MDI parent = `frmMain1`.
> Data access: `Module1.connect(string sql) -> DataSet` — pure string concatenation, no parameters.
> Two DB backends supported: MS Access (`Provider=Microsoft.ACE.OLEDB.12.0`, password `foreverbu`) and SQL Server (`Database_Mode = "SQL"|"ACCESS"`).

---

## 1. Entry Point & App Startup

**Main entry**: `iHOTEL2025.My.MyApplication.Main` (auto-generated VB MyApplication).
- File: `_decompiled_clean/iHOTEL2025.My/MyApplication.cs`
- `OnCreateMainForm()` → `MainForm = MyProject.Forms.frmMain1` — single MDI parent for everything.
- `IsSingleInstance = true`, `ShutdownMode.AfterMainFormCloses`.
- There is a **SplashForm** referenced via the resource `aR3nbf8dQp2feLmk31.SplashForm.resx` (renamed by deobfuscator), but its source is in the obfuscated `Class*` files, not in iHOTEL2025/. It is shown by the VB My runtime before `frmMain1`.

**`frmMain1.cs:6938` — `frmMain1_Load` boot sequence** (this is the real bootstrap):
1. `ReadConfig()` — load `localdata.Config` (Theme, defaults).
2. `IsAdmin()` check — if not admin, relaunch `HOTEL.exe` with `runas`.
3. `method_0()` — read/write registry `HKLM\Software\microsoft\MSXKPHTEL` for trial dates, `RegCode`, `always`, `SD/SL/ED` (license key dates).
4. `ReadReg()` — read company/license info.
5. If `server.txt` missing → `ReadDB_old()` and exit.
6. `FrmSettings.ReadPrint()` — load printer config.
7. `Module1.load_book_num()` — load booking sequence number.
8. **`FormSelectDB.ShowDialog()`** — DB connection picker (Access path / MSSQL server,user,pass,db). Loops on connection failure.
9. `Module1.ReadDB_2018()` — open connection (Access OleDb or `MSSQL.connectmssql()`), creates DB on first run via `MSSQL.Create_MssqlDatabase`.
10. `Update_Version()` + `Module1.ReadSettingsConfig()` — pull `TB_SETTINGS`.
11. **`login.ShowDialog()`** — username/password form (TB_MRP_EMPLOYEE). Sets `Module1.loginID/loginName/loginMode`.
12. If `HouseWifeMode` → hide main, open `FormRoomMainClean` instead. If `KichenMode` → `FormRoomMainKichen` instead. Both are MDI-less role-restricted shells.
13. `loadLogin()` — apply permissions (TB_MRP_PERMISSION) by hiding ribbon buttons.
14. `Module1.load_deposit()`, load receipt copy counts.
15. Cleans up: `DELETE HT_Room_Status WHERE room_status='Check-Out'`, fixes orphaned status rows, prunes `HT_Book_Date < 60 days`.
16. Trial-mode banner if `IS_TRIAL`.
17. `CHK_NOTIFLY()` — query upcoming bookings unpaid → ribbon notification dropdown.
18. `FrmSettingsSMS.LoadSMS()` — preload SMS credentials.
19. **Online license check** — `WebBrowser2.Url = http://www.kpsystem.co.th/version_hotel.php?comid=...&PVER=...` (version check).
20. **Online block check** — `WebBrowserBlock.Url = http://www.kpsystem.co.th/chk_hotel.php` — kill switch (writes `reg.txt` and Close() if COM_ID matched).
21. `FormUPDATE_0.load_update()` — scheduled IP-update timer.
22. Idle/auto-logout: `TimerMouse_Tick` re-shows `login` after `Module1.AutoLogout` seconds of no mouse movement.

**Critical globals in `Module1`** (see `Module1.cs:30–540`):
- `Database_Mode` ("ACCESS"|"SQL"), `datechar` ("#" Access | "'" SQL).
- `loginID`, `loginName`, `loginMode` (admin / user / housewife etc.).
- `ProgramVersion = "1.6.1"` (string also says 1.8.5 in ribbon — inconsistency).
- `IS_TRIAL`, `P_MODE` (DEMO/FULL), `CHK_IN_Before`, `CHK_Out`, `CHK_Out_Before`, `Maximum_Book`, `Vat_Over`.
- `POWER_USED`, `POWER_PORT="COM1"`, `CASH_PORT` — relay/cashdrawer hardware flags.
- `HouseWifeMode`, `KichenMode` — alternate UI shells.
- `Receipt_Report/_print/_preview`, `POS_Report/_print/_preview`, `Cupon_Report`, `Tax_preview`, `inv_print` — Crystal Report names per print job.
- `string_0` — hardcoded comma-separated MAC/serial whitelist for licensing (~250 entries).

---

## 2. Main Menu (Ribbon) Structure

`frmMain1` is a `Office2007Form` with a `RibbonControl1` containing 6 ribbon tabs. Sub-tabs come from `RibbonBar*` panels which contain `ButtonItem` buttons. Below: every tab → bar → button → form opened. Method names are the click handlers in `frmMain1.cs:7355–8534`.

### Tab: หน้าหลัก (Main / `ribbonTabItem1`)

| Bar | Button (Thai) | Handler | Opens |
|---|---|---|---|
| ห้องพัก (Rooms) `RibbonBar1` | รายการห้องพัก | `ButtonItem9_Click` | `FormRoomMain` (Show, MDI child) |
| | Check-In | `ButtonItem10_Click(book_no, c_no)` | `new FrmCheckIn()` (after `check_round_bill`) |
| | Check-Out | `ButtonItem12_Click` / `ButtonItem59_Click` | `new FrmCheckOut()` |
| | จองห้องพัก ▼ | `ButtonItem43_Click` | expands sub-menu B4 |
| | ↳ จองแบบไม่ระบุห้อง | `ButtonItem101_Click` | `FrmBookMain` |
| | ↳ จองแบบระบุห้อง | `ButtonItem_0_Click` | `FrmBookMain2` |
| | ↳ ค้นหารายการจอง | `ButtonItem9_Click_3` | `FrmSearchBook` |
| | ↳ แผนผังตารางการจอง | `ButtonItem34_Click_1` | `FormRoomMain_ViewBook` |
| ขายสินค้า ▼ `B5` | เพิ่มรายการขาย | `ButtonItem13_Click` | `FrmReceiptMain` (MDI) |
| | ดูรายการขายสินค้า | `ButtonItem21_Click_1` | `FrmSaleMain2` |
| คูปองอาหาร | `ButtonItem6_Click_1` | `FrmCuponMain` |
| ใบลงทะเบียน `B6` | (ButtonItem11) | `ButtonItem11_Click` | `FrmRegMain` |
| จัดการรอบบิล | `B12_Click` | `FrmDueBill` |
| สต๊อค `RibbonBar2` | จัดการสินค้า | `ButtonItem23_Click` | `new FrmManageProduct()` |
| บัญชี `RibbonBar4` | ใบลงทะเบียนผู้เข้าพัก `B7` | (linked to RibbonTabItem2) | `FrmRegMain` |
| | ใบมัดจำ `B8` | | `FrmDepositMain` |
| | ใบสำคัญรับ `B9` | | `FrmReceiptMain` |
| | ใบแจ้งหนี้ (อื่นๆ) `ButtonItem32` | `ButtonItem32_Click_2` | `FrmReceiptInvoice` |
| | ใบกำกับภาษี `B10` | | (Crystal Report from Print_Report) |
| | ชำระเงิน/ลูกหนี้ ▼ `B11` | | expand |
| | ↳ ลงทะเบียน | `ButtonItem28_Click` | `FrmDepositMain` (MDI) |
| | ↳ ขายสินค้า | `ButtonItem29_Click_1` | `FrmPayDebt2` |
| (status bar) ออกจากระบบ | `ButtonItem48_Click` | `login.ShowDialog()` then re-init |

### Tab: บัญชี (Accounting / `RibbonTabItem2`)

| Bar | Button | Handler | Opens |
|---|---|---|---|
| `RibbonBar_7` | รายรับ-รายจ่าย | `ButtonItem39_Click_1` | `FrmPayMain` |
| | ชำระเงิน/ลูกหนี้ ลงทะเบียน | `ButtonItem41_Click_1` | `FrmPayDebt` |
| | ชำระเงิน/ลูกหนี้ ขายสินค้า | `ButtonItem42_Click_1` | `FrmPayDebt2` |
| | (สินค้าเครดิต) | `ButtonItem13_Click_1` | `new FrmAddSale2_Credit()` |
| | จัดการรอบบิล | `B12_Click` | `FrmDueBill` |
| | ใบแจ้งหนี้อื่นๆ | `ButtonItem32_Click_2` | `FrmReceiptInvoice` |
| | ใบสำคัญรับ | `ButtonItem42_Click` | `FrmReceiptMain_invoice` |
| | รายร-รายจ่าย2 | `ButtonItem40_Click` | `FrmInOutMain` |

### Tab: รายงาน (Reports / `RibbonTabItem4`)

The largest tab. Bars: `RibbonBar_2` (general), `RibbonBar_3` (income), `RibbonBar_4` (shift/round), R8/R9/R13/R14/R15/R6 sub-bars.

| Button | Opens |
|---|---|
| สถานะห้องพัก `ButtonItem12` | `ReportTax` (CR viewer) |
| รายงานสินค้า `R8` → `FrmReportProducts` | `ButtonItem47_Click_2` |
| รายงานการขายสินค้า `R9` | `FrmReportProductsSale` |
| รายงานบัญชี/ภาษี `ButtonItem5` ▼ | sub-menu |
| ↳ รายงานลูกหนี้ `R7` | `ReportDebt` |
| ↳ รายงานเงินมัดจำ `R14` ▼ | sub-menu |
| ↳↳ รับเงินมัดจำ `ButtonItem20` | `FrmReportMudjumRec` |
| ↳↳ คืนเงินมัดจำ `ButtonItem50` | `FrmReportMudjumBack` |
| ↳ รายงานสรุปภาพรวมรายได้ `ButtonItem11` | `FormReportAll2` |
| ↳ รายงานภาษีขาย `R6` | `ReportTax` |
| รายงานเกี่ยวกับห้อง `ButtonItem43` ▼ | sub-menu (R1/R4/R3/etc.) |
| ↳ รายงานสรุปประจำวัน `R1` | `ReportDays` |
| ↳ รายงานแขกที่อยู่ในโรงแรม `R4` | `ReportCustIn` |
| ↳ แขกที่กำลังจะออก `ButtonItem51` | `ReportCustOutToday` |
| ↳ แขกที่ออกไปแล้ว `R3` | `ReportCustOut` |
| ↳ รายงานการจองทั้งหมด `ButtonItem56` | `FrmReportBook` |
| ↳ รายงานการจองแบบระบุห้อง `ButtonItem57` | `FrmReportBook2` |
| ↳ รายงานเงินจอง `ButtonItem58` | `FrmReportPaybooking` |
| ↳ รายงานเซลล์ตามประเภทห้อง `ButtonItem55` | `FrmReportSale1` |
| ↳ รายงานเซลล์ตามวันที่/ลูกค้า `ButtonItem61` | `FrmReportSale2` |
| รายงาน เปิด-ปิดไฟ `ButtonItem54` | `FrmReportPower` |
| รายงานแม่บ้าน `ButtonItem59` | `FrmCheckOut` (also doubles as housewife report) |
| รายชื่อห้องที่จะออกวันนี้ `ButtonItem63` | `ReportCustOutToday2` |
| รายชื่อห้องที่พักต่อ `ButtonItem64` | `ReportContnueRoom2` |
| รายงานคูปอง `R13` ▼ | sub-menu |
| ↳ ตามคูปอง `ButtonItem65` | `FrmReportCoupon` |
| ↳ ตามใบลงทะเบียน `ButtonItem66` | (placeholder) |
| รายงานสรุปภาพรวม `R15` | `ReportContnueRoom2` |
| รายงานจำนวนเข้าพักลูกค้า `ButtonItem40` | `FrmUseCount` |
| รายงานส่งอำเภอ รร.4 `ButtonItem62` | `FrmReportRR4` |
| รายงานยกเลิก/ขายสินค้า `ButtonItem67` | `FrmReportCancelSale` |
| `RibbonBar_3` รายงานระหว่างวันที่ `R17` | `FrmReportImcome` |
| `RibbonBar_3` รายงานตามรอบบิล `R18` | `FrmReportImcome2` |
| `RibbonBar_4` รายงานการขายห้อง `R19` | `FrmReportShift` |
| `RibbonBar_4` รายงานปิดรอบ/เงินสดคงเหลือ `R20` | `FrmReportShiftCash` |
| รายงานยกเลิก `ButtonItem55_Click` | `FrmReportCancel` |
| รายงานซ่อมห้อง `ButtonItem50_Click_3` | `FrmReportrepair` |
| รายงานการต่อเวลา `ButtonItem30_Click_1` | `ReportContnueRoom` |
| ตั้งค่าการต่อเวลา `ButtonItem46_Click` | `new FrmSETTimeContnue()` |
| รายงานทำความสะอาด `ButtonItem47_Click_3` | `ReportCleanRoom` |
| รายงานแม่บ้าน `ButtonItem53_Click_1` | `FrmReportHousewife` |
| รายงานเปลี่ยนห้อง `ButtonItem53_Click` / `ButtonItem52_Click_1` | `ReportCustChange` |
| รายงานวันเข้าพัก `ButtonItem51_Click_1` | `ReportCustDays` |
| รายงานทั้งหมด `ButtonItem9_Click_2` | `FormReportAll` |
| ตารางเวลา `ButtonItem12_Click_2` | `frmTimeTable` |

### Tab: ตั้งค่า (Settings / `RibbonTabItem3`)

| Bar | Button | Opens |
|---|---|---|
| สินค้า `RibbonBar_0` | ประเภทสินค้า `B21` | `new FrmSETProType()` |
| | ทะเบียนสินค้า `B22` | `new FrmManageProduct()` |
| ห้องพัก `RibbonBar8` | ประเภทห้อง/ราคา `B14` | `new FrmSETRoomType()` |
| | จัดการห้องพัก `B15` | `new FrmManageRoom()` |
| | จัดการต่อเวลา `ButtonItem46` | `new FrmSETTimeContnue()` |
| ลูกค้า `RibbonBar9` | ลูกค้า `B16` | `new FrmManageCustomersNew()` |
| | ประเภทลูกค้า `B17` | `new FrmSETCsuType()` |
| | กลุ่มราคา/มัดจำ `B18` | `FrmSETCsuTypeMain` |
| | ตั้งค่าปรับราคาลง `B19` | `FormManageOrderCustDown` |
| | ตั้งค่าปรับราคาขึ้น `B20` | `FormManageOrderCust` |
| | เซลล์ `ButtonItem60` | `new FrmSETsale()` |
| | สาขา `ButtonItem68` | `new FrmSETBranch()` |
| ผู้ใช้งาน `RibbonBar3` | ผู้ใช้งาน `B13` | `FrmUser` (also opens `FrmPermission`) |
| ตั้งค่า `RibbonBar_1` | ตั้งค่าโปรแกรม `B23` | `FrmSettings` (master settings, ~7000 lines) |
| | ตั้งค่า SMS `ButtonItem52` | `FrmSettingsSMS` |
| | ลบข้อมูลทั้งหมด `ButtonItem38` | inline `delete from HT_*` (factory reset) |
| | DDNS `ButtonItem71` | `FormUPDATE` |

### Tab: ธีม (Theme / `RibbonTabItem5`)
Pure UI theme switching — `Office2007Blue/Silver/Black`, `VistaGlass`, `Office2010`, `Windows7Blue`. Persists to `localdata.Config`.

### Tab: เกี่ยวกับ (About / `RibbonTabItem_0`)
- เกี่ยวกับโปรแกรม `ButtonItem37` → `AboutBox1`
- การลงทะเบียน `ButtonItem70` → `frmReg`
- ผู้ผลิต — info only.
- อัปเดตโปรแกรม `B24` → `FrmUpdate`
- เปลี่ยน Server `ButtonItem49` → relaunch `HOTEL.exe`
- ส่ง SMS `ButtonItem53_Click_2` → `new FormSMSSendManual()`
- หลังบ้าน (admin trapdoor) `ButtonItem69` → `FormPass` → if pwd matches admin row → `FormLog`
- รุ่น `ButtonItem_Version_Click` → `FrmUpdate`

### Quick Action / Toolbar
- `ButtonNotification` (ribbon corner) — clickable list of unpaid bookings within `Book_Notify_Day`. Each item → `FrmShowBookNotify`.
- `ButtonItem36_Click_1` (toggle) — show/hide hidden WebBrowser1 (used for serial-port relay log).
- `ButtonItem3` "รับกล้อง/ใช้กล้อง" → camera capture (frmCapture).
- `ButtonItem1/22` — DDNS panel/camera management.

---

## 3. Functional Modules

### 3.1 Auth & Session (3 forms)
| Form | Purpose |
|---|---|
| `login.cs` | Username/password against `TB_MRP_EMPLOYEE`. Sets `loginID/Name/Mode`. Calls `loadLogin()` permissions. |
| `FormPass.cs` | Single-textbox password gate (admin trapdoor for FormLog). |
| `FrmPermission.cs` | Per-role permission editor — checks/unchecks command IDs in `TB_MRP_PERMISSION`. Opened from `FrmUser`. |
| `FrmUser.cs` | CRUD for `TB_MRP_EMPLOYEE` (employees). Opens FrmPermission for level edits. |
| `FormLog.cs` | Read-only log viewer (`HT_Log`). |

### 3.2 Main Shell & Room Grid (5 forms)
| Form | Purpose |
|---|---|
| `frmMain1.cs` (8548 lines) | MDI parent, ribbon, notifications timer, license check, idle logout, serial-port power relay, online version check. |
| `FormRoomMain.cs` | The main room-status grid (matrix of room buttons by floor/zone). Hub for all room actions; routes clicks to ClickAvliable/ClickBook/ClickClean/ClickUSE/ClickManternance handlers. Used by `frmMain1.ButtonItem9_Click`. |
| `FormRoomMain_ViewBook.cs` | Booking calendar/grid overlay (room × date). |
| `FormRoomMainClean.cs` | Housewife-mode-only shell — limited-permission view of dirty rooms. Used as alternate main form when `HouseWifeMode=true`. |
| `FormRoomMainKichen.cs` | Kitchen-mode-only shell (similar restrictive UI). |
| `FormRoomMainKichen_old.cs` | DEAD — old version. |
| `frmTimeTable.cs` | Reads `HT_Rooms` + `HT_Room_Status`, displays time grid. |

### 3.3 Room Cell Click Handlers (6 + 2 dupes)
These are pseudo-modal forms shown when the user clicks a room cell on `FormRoomMain`. Each is a thin dialog around a context-specific action.

| Form | Purpose |
|---|---|
| `ClickAvliable.cs` | "Mark room available" — flips Room_Clean='yes', clears flags. |
| `ClickAvliable_book.cs` | Same in booking-mode context. |
| `ClickBook.cs` | "Book this room" → opens `FrmAddBook`/`FrmAddBook2`/`FrmAddBook3`/`FrmAddBook4`. |
| `ClickBook_book.cs` | Same, booking-grid context. |
| `ClickClean.cs` | "Mark dirty" → write to `HT_Housewife`, update `HT_Rooms.Room_Clean`. |
| `ClickCleanOK.cs` | "Mark clean done" → write `HT_Housewife` with cleaner emp_id. |
| `ClickManternance.cs` | "Send to maintenance" → `HT_Rooms.Room_Manternace='yes'`. |
| `ClickUSE.cs` (1900 lines) | The big one — "currently in use" room dialog: edit checkin, sell to room (`FrmAddSale`), payment (`FrmPayAddPro`/`FrmPayAdd`), pay deposit, change date (`FrmEditDate`), folio (`FormFolio`), VAT (`FormShowVAT`), notes (`Room_Note`/`EMP_Note`), checkout (`FrmCheckOut`), invoice (`INV_Note`). |
| `ClickUSE2.cs` | In-use variant 2 (smaller scope, possibly housewife). |
| `ClickUSE3.cs` | In-use variant 3. |
| `ClickAddmore.cs` | "Add more guest" small dialog. |
| `Room_Note.cs` / `Room_Note_Read.cs` | Per-room sticky notes (`HT_Room_SMS`). |
| `EMP_Note.cs` / `EMP_Note_Read.cs` | Inter-employee sticky notes (`HT_EMP_SMS`). |

### 3.4 Booking (8 forms)
| Form | Purpose |
|---|---|
| `FrmBookMain.cs` | List/search bookings (no specific room) — opens `FrmAddBook`. |
| `FrmBookMain2.cs` | List/search bookings WITH specific room — opens `FrmAddBook` and `FormBookingInvoice`. |
| `FrmAddBook.cs` (~22 table touches) | Create/edit booking without room assignment. Writes `HT_Book_H`, `HT_Book_Ds`, `HT_Book_Date`, `HT_Customers` upsert. |
| `FrmAddBook2.cs` (~33) | Create/edit booking with rooms. Adds `HT_Book_Pro`, sets `HT_Rooms.Room_Book*`. |
| `FrmAddBook2copy.cs` | DEAD copy. |
| `FrmBookRooms.cs` | Pick rooms for an existing booking — writes `HT_Book_Ds`, `HT_Room_Status`. |
| `FormBookRooms.cs` | Read-only "rooms in this booking" listing. |
| `FrmShowBookNotify.cs` | Notification popup detail — opens for each `Book_Notify_*` reminder. |
| `FormBookingInvoice.cs` | Generates HT_INVOICE row from a booking (writes `HT_INVOICE`, reads `HT_Book_*`, `HT_Book_Pro`). |
| `FrmSearchBook.cs` | Search bookings by date/customer (used as MDI child from main menu). |
| `FormSearchBooking.cs` / `FormSearchBooking2.cs` | Pickers for legacy `HT_Book_H` / `HT_Book_H2`. |
| `FrmShowBookNotify.cs` | Reminder action sheet (mark "ไม่แจ้งเตือน", view, delete). |

### 3.5 Check-in / Check-out (5 forms)
| Form | Purpose |
|---|---|
| `FrmCheckIn.cs` (~11k lines, 19 tables) | The main check-in form: customer search, SmartCard read, photo capture, room price calc, products attached, deposit, signature, register print. Writes HT_CheckIn_H/Ds/Pay/Product, HT_Customers, HT_Rooms, HT_Room_Status, Tb_Save_Image. |
| `FrmCheckIn_EditOnly.cs` | Read-only/edit-existing variant of FrmCheckIn (no room change). |
| `FrmCheckOut.cs` | Final settle: products, debt, payment methods, prints receipt/folio. Updates `HT_Rooms.Room_Use='no'`, `HT_Room_Status='Check-Out'`. |
| `FormConfirmPay.cs` | Confirm-payment modal (ID, change due, payment method) used everywhere. |
| `FormConfirmRoundBill.cs` | Confirm round-bill action (truncate to nearest baht). |
| `FormConfirmOverBill.cs` | Confirm over-charge override prompt. |
| `FormVatOver.cs` | Over-VAT alert / override. |
| `FormShowVAT.cs` | Show VAT breakdown for receipt. |
| `FormShowDEP.cs` / `FormShowDEPBack.cs` | Apply / refund deposit dialogs. |
| `FormShowSAVEout2.cs` | Save-out-room dialog. |
| `FormSelectRoom.cs` | Generic room picker. |
| `FormSearchChechIn.cs` / `FormSearchChechInnotOut.cs` / `FormSearchChechInVAT.cs` (referenced) | Search check-in records. |

### 3.6 Customers (6 forms)
| Form | Purpose |
|---|---|
| `FrmManageCustomersNew.cs` (12 tables) | The current customer master — CRUD `HT_Customers`, photo (Tb_Save_Image), SmartCard read, history (Bill_Debt, Receipt, Invoice, CheckIn, Book). |
| `FrmManageCustomers.cs` | Older/legacy customer mgmt (kept for compatibility). |
| `FrmManageCustomersSearch.cs` | Lightweight customer picker. |
| `FrmCustomers.cs` | Possibly orphan — see Section 8. |
| `FrmSearchCustomers.cs` / `FormSearchCust.cs` | Customer pickers used by sale/booking forms. |
| `FrmShowPreviewSmartCard.cs` | Modal that displays Thai national-ID smart-card data + progress bar while `KPThaiNationalIDCard.exe` runs. |

### 3.7 Sales / POS (5 forms)
| Form | Purpose |
|---|---|
| `FrmReceiptMain.cs` | Sale receipt main (MDI) — opens `FrmAddSale` for new lines, `FormConfirmPay` for tender. |
| `FrmReceiptMain_invoice.cs` | Same flow but for VAT-invoice receipts. |
| `FrmReceiptInvoice.cs` | Issue separate `HT_Invoice_*` doc. |
| `FrmAddInvoiceSale.cs` | Add line to invoice from CheckIn product. |
| `FrmAddSale.cs` (16 tables) | Add sale line: customer pick, product pick, price, write `HT_Receipt_Ds`, update `HT_CheckIn_*` if linked to room. |
| `FrmAddSale2.cs` | Variant — adds line directly to receipt without check-in linkage. |
| `FrmAddSale2_Credit.cs` | Credit sale variant — writes `HT_Bill_Debt_Ds`. |
| `FrmSaleMain2.cs` | List/search sales (`HT_Bill_Debt_*`). |

### 3.8 Cupon (1 form)
| `FrmCuponMain.cs` | Food coupon dispenser (`HT_Cupon`). |

### 3.9 Payments / Folio / Debt (8 forms)
| Form | Purpose |
|---|---|
| `FrmPayMain.cs` | Cash in/out (rayrap-rayjai) main. Writes `tb_pay_history`. |
| `FrmPayAdd.cs` | Add payment to checkin (no products). |
| `FrmPayAddPro.cs` | Add payment + product line to checkin. |
| `FrmPayAddDebt.cs` | Add payment against debt. |
| `FrmPayDebt.cs` | List debts from check-ins. |
| `FrmPayDebt2.cs` | List debts from sales. |
| `FrmAddPay.cs` | Add manual income/expense entry. |
| `FrmAddDep.cs` | Add deposit (HT_Deposit). |
| `FrmDepositMain.cs` | List deposits (MDI). |
| `FormFolio.cs` | Per-checkin folio view — TB_FOLIO. |
| `FrmDueBill.cs` | Round-bill control: opens new `HT_Round_Bill` row, locks operations until `check_round_bill()` passes. |
| `FrmEditDate.cs` | Change check-in dates / extend stay. |
| `FrmInOutMain.cs` | List of all transactions (in/out) for the day/range. |
| `FormEditPay.cs` | Edit a single `HT_CheckIn_Pay` row. |

### 3.10 Invoices / VAT (4 forms)
| `FrmReceiptInvoice.cs` | Issue/print HT_Invoice_H. |
| `FrmAddInvoiceSale.cs` | Add CheckIn product → Invoice line. |
| `INV_Note.cs` | Notes on invoice (`HT_Invoice_Note`). |
| `FormShowVAT.cs`, `FormVatOver.cs` | VAT presentation/override. |

### 3.11 Rooms / Products / Types Master Data (10 forms)
| `FrmManageRoom.cs` | CRUD `HT_Rooms` (room number, type, default price). |
| `FrmSETRoomType.cs` | CRUD `HT_SET_RoomType` + price ladder `HT_Rooms_Price`. |
| `FrmManageProduct.cs` | CRUD `HT_Products` + `HT_Products_Price`. |
| `FrmSETProType.cs` | CRUD `HT_SET_ProductType`. |
| `FrmSETCsuType.cs` | Customer type — touches `HT_SET_CusType_Main`, `HT_Customers`, ripples price updates. |
| `FrmSETCsuTypeMain.cs` | Customer-group main (parent of CusType) — same scope. |
| `FrmSETMyType2.cs` / `FrmSETMyType2_2.cs` / `FrmSETMyType3.cs` | Cash-in/out category trees (`TB_SET_MyType2/2_2/3`). |
| `FrmSETsale.cs` | Sales-rep master (`HT_SET_Sale`, links `HT_Customers`). |
| `FrmSETBranch.cs` | Branch master (`TB_SET_Branch`). |
| `FrmSETTimeContnue.cs` | "Continue time" (extension) pricing rules (`HT_ContinueTime`). |
| `FormManageOrderCust.cs` / `FormManageOrderCustDown.cs` | Per-customer-type up/down price overrides (`HT_Order_Up` / `HT_Order_Down`). |
| `FrmPriceHistory.cs` | View `HT_Log_Debt` price-change history. |

### 3.12 Reports (~30 forms)
Each is a thin DateTimePicker-driven launcher for a Crystal Report.

| Launcher Form | Crystal Report | Purpose |
|---|---|---|
| `FrmReportImcome.cs` | ReportIncome | Income between dates |
| `FrmReportImcome2.cs` | ReportIncome2 | Income by round-bill |
| `FrmReportShift.cs` | (sale_*) | Shift sales |
| `FrmReportShiftCash.cs` | ReportShipCash(OLD) | Round close + cash count |
| `FrmReportSale1.cs` | sale_vat0CB | Sales by room type |
| `FrmReportSale2.cs` | sale_vat0SB | Sales by date/customer |
| `FrmReportRR4.cs` | Report_RR4 | RR.4 government form |
| `FrmReportBook.cs` | ReportBooking | All bookings |
| `FrmReportBook2.cs` | ReportBookingINV | Bookings with invoice |
| `FrmReportPaybooking.cs` | ReportDep / ReportDep2_58/80 | Deposit reports |
| `FrmReportCancel.cs` / `FrmReportCancelSale.cs` | (sale_*) | Cancellations |
| `FrmReportProducts.cs` | sale | Product sales |
| `FrmReportProductsSale.cs` | sale | Product sales detail |
| `FrmReportCoupon.cs` | ReportCupon58/80 | Cupon reports |
| `FrmReportHousewife.cs` | CleanRoom | Housekeeping report |
| `FrmReportrepair.cs` | (none) | Maintenance log |
| `FrmReportPower.cs` | (none) | Relay on/off log |
| `FrmReportMudjumRec.cs` / `FrmReportMudjumBack.cs` | ReportDep | Deposit in / refund |
| `FrmReportRecPay.cs` | inv_sale_vat0_debt_* | Receipt-pay history |
| `FrmUseCount.cs` | (inline grid) | Customer visit counts |
| `FormReportAll.cs` / `FormReportAll2.cs` | (composite) | Big "all reports" dashboard |
| `ReportDays.cs`, `ReportCustIn/Out/Days/Change`, `ReportContnueRoom(2)`, `ReportCleanRoom`, `ReportTax`, `ReportCustOutToday(2)` | matching CR | Each is a Form wrapping a `CrystalReportViewer` plus filter controls. |

The 30+ `Cached*.cs` (`CachedReportIncome.cs` etc.), `Crystal*.cs`, and `Report*.cs` (without "Frm") files are **auto-generated Crystal Reports plumbing** — DataSet/typed-DataTable classes for each `.rpt`. Treat them as one row in the rewrite plan ("port reports"); do not enumerate.

### 3.13 SMS (3 forms + tables)
| `FrmSettingsSMS.cs` | SMS account credentials → `http://www.kpsystem.co.th/sms/sms.php` (KP SMS gateway). Stores in `localdata.Config`. |
| `FormSMSSendManual.cs` | Manual SMS composer (uses `TB_SMS_FAVORITES_2`). |
| `FormSMSLog.cs` | SMS send-log viewer. |
| `FormSMS_DEBT.cs` (referenced from FrmCheckOut) | Send debt-reminder SMS. |

### 3.14 Settings & Admin (8 forms)
| `FrmSettings.cs` (~7000 lines) | Master settings — printers, COM ports, receipt format, deposit defaults, tax % , 100+ knobs in `TB_SETTINGS`. |
| `FormSelectDB.cs` | DB connection picker (saves to `server.txt`). |
| `FormSelectDB_old.cs` | DEAD legacy. |
| `FrmAddEditServer.cs` | Add/edit server entry (used by FormSelectDB). |
| `connect_mssql.cs` | Standalone MSSQL connect dialog (alternate route). |
| `FormUPDATE.cs` (DDNS) | Periodic POST current public IP somewhere (DDNS-style). |
| `FormUpdateDateRoomAll.cs` | Bulk-fix `HT_Room_Status.room_date_oa=0` rows. |
| `FrmUpdate.cs` | Self-updater — downloads new `HOTEL.exe` from kpsystem.co.th. |
| `frmReg.cs` | License registration screen (writes `TB_SETTINGS`, posts to KP). |
| `frmWanting.cs` | "Loading…" splash modal (Module1.Wainting). |
| `AboutBox1.cs` | About dialog. |

### 3.15 Image Capture (3 forms)
| `FrmAddSaveImage.cs` | Photo upload/capture host — invokes `TwainGui.exe` for scanner. |
| `frmCapture.cs` | Webcam capture (uses iCam.cs). |
| `iCam.cs` | DirectShow webcam wrapper. |
| `Twain*.cs` (15 files) | TWAIN protocol bindings (open-source TWAIN.NET) — group as one library. |

### 3.16 Misc Utility / Hidden
| `FormART.cs` | Art-on-screen toy (shown in `frmMain1` line 6471 unconditionally). |
| `Form1.cs` | Empty test form — DEAD. |
| `FrmPrint.cs` | Print queue/preview wrapper. |
| `FormLog.cs` | System log viewer (admin trapdoor). |
| `FormPass.cs` | Password gate. |
| `Datalocal.cs` | In-memory cache class for client-side config (DataSet "Config"). |
| `Encrypt.cs` / `FormEN_DE.cs` | TripleDES helpers used to obfuscate SQL passwords in `server.txt`. |
| `MSSQL.cs` | SQL Server connection wrapper, schema creator. |
| `Module1.cs` | God-module: globals, `connect()`, helpers, license/MAC check, settings load, common SQL ops. |
| `GENDB.cs` | Generic DB helper (some legacy methods). |
| `Print_Report.cs` | Centralized print orchestrator — opens correct CR per print type. Has many table-fetch SQLs. |
| `PrintFactory.cs` | Print device factory. |
| `ResizeableControl.cs` | UI helper. |
| `ButtonTable.cs` | Custom data button. |
| `DecimalToText_TH.cs` | Number → Thai text (for receipts). |
| `ModuleClick.cs` | Helper module for click routing. |
| `GClass3/4.cs`, `GForm0/1.cs`, `Class0/1/2.cs`, `BITMAPINFOHEADER.cs`, `Gdip.cs` | Decompiler artifacts / obfuscation stubs. **Treat as opaque support code; do not port verbatim.** |

---

## 4. Form-to-Form Navigation Graph (key flows only)

```
SplashForm (Class*.cs)
  └── frmMain1.Load
        ├── FormSelectDB                       (connection picker)
        │     └── FrmAddEditServer              (add MSSQL server)
        ├── login                               (auth)
        ├── (HouseWifeMode) → FormRoomMainClean     ─ side-shell
        ├── (KichenMode)    → FormRoomMainKichen    ─ side-shell
        └── (normal)        → FormRoomMain          (MDI child, default)

FormRoomMain  (room grid — central hub)
  ├── click empty cell    → ClickAvliable               → updates HT_Rooms only
  ├── click empty cell + book mode → ClickBook         → FrmAddBook / FrmAddBook2 / FrmAddBook3 / FrmAddBook4
  ├── click clean cell    → ClickCleanOK
  ├── click dirty cell    → ClickClean
  ├── click maint cell    → ClickManternance
  ├── click occupied cell → ClickUSE                     (huge dispatcher)
  │       ├── FrmAddSale / FormShowVAT / FormFolio
  │       ├── FrmPayAdd / FrmPayAddPro
  │       ├── FrmEditDate
  │       ├── INV_Note / Room_Note / EMP_Note
  │       └── FrmCheckOut → FormConfirmPay → (prints folio/receipt via Print_Report)
  ├── housewife click     → ClickUSE2 / ClickUSE3
  ├── room note button    → Room_Note_Read
  ├── emp note button     → EMP_Note_Read
  ├── pick room           → FormSelectRoom
  └── view bookings       → FormRoomMain_ViewBook

frmMain1 ribbon → FrmBookMain  (no-room booking list)
                        └── FrmAddBook
frmMain1 ribbon → FrmBookMain2 (with-room booking list)
                        ├── FrmAddBook (and AddBook variants)
                        └── FormBookingInvoice → writes HT_INVOICE

frmMain1 ribbon → FrmCheckIn  (direct check-in path, no room grid)
                        ├── FormSearchPro                (product picker)
                        ├── FormSearchChechIn            (existing checkin search)
                        ├── FrmAddSaveImage              (photo / TwainGui.exe)
                        ├── FormConfirmPay
                        └── FrmShowPreviewSmartCard      ← KPThaiNationalIDCard.exe

frmMain1 ribbon → FrmCheckOut (direct check-out path)
                        ├── FormSearchChechInnotOut       (still-checked-in search)
                        ├── FormSearchPro
                        ├── FormShowSAVEout2 / FormShowDEPBack
                        ├── FrmAddSale / FormSMS_DEBT
                        ├── FormConfirmOverBill / FormConfirmPay
                        └── FormSETelec_0                  (electricity/elec usage)

frmMain1 ribbon → FrmManageCustomersNew
                        ├── FrmShowPreviewSmartCard       ← KPThaiNationalIDCard.exe
                        ├── FrmAddSaveImage / GForm0      (photo capture)
                        ├── FrmManageCustomersSearch
                        └── FrmPriceHistory               (HT_Log_Debt)

frmMain1 ribbon → FrmReceiptMain        → FrmAddSale → FormConfirmPay
frmMain1 ribbon → FrmReceiptMain_invoice → FrmAddInvoiceSale
frmMain1 ribbon → FrmReceiptInvoice (standalone invoice)
frmMain1 ribbon → FrmDepositMain (no children — read-only list)
frmMain1 ribbon → FrmRegMain     (registration form host)
frmMain1 ribbon → FrmSaleMain2   (HT_Bill_Debt_* listing)
frmMain1 ribbon → FrmCuponMain
frmMain1 ribbon → FrmPayMain
frmMain1 ribbon → FrmPayDebt / FrmPayDebt2 → FrmPayAddDebt
frmMain1 ribbon → FrmDueBill (round-bill open/close)

frmMain1 ribbon → Settings forms (all simple ShowDialog, no children except FrmUser→FrmPermission)

frmMain1 ribbon → FrmUpdate                ← downloads from kpsystem.co.th
frmMain1 ribbon → FormUPDATE (DDNS auto)
frmMain1 ribbon → FrmSettingsSMS           ← embedded WebBrowser hits sms.php
frmMain1 ribbon → FormSMSSendManual
frmMain1 ribbon → FormPass → FormLog       (admin trapdoor)
frmMain1 ribbon → frmReg                   (license registration)
frmMain1 idle    → login                    (auto-logout via TimerMouse)
frmMain1 notify  → FrmShowBookNotify       (per-row click)
```

**Key invariant**: almost every transactional flow checks `Module1.check_round_bill()` first and refuses unless an open `HT_Round_Bill` row exists.

---

## 5. Per-Form Table Touches

R = read (SELECT/JOIN), W = write (INSERT/UPDATE/DELETE), RW = both. Auto-generated CR plumbing files omitted.

| Form | Tables |
|---|---|
| `ClickAvliable` | HT_Rooms(RW) |
| `ClickBook` | HT_Book_Date(RW), HT_Book_Ds(W), HT_Book_H(R), HT_Rooms(RW) |
| `ClickBook_book` | HT_Book_Date(W), HT_Book_Ds(W), HT_Book_H(R) |
| `ClickClean` | HT_Book_Date(R), HT_Housewife(W), HT_Rooms(RW), TB_MRP_EMPLOYEE(R) |
| `ClickCleanOK` | HT_Book_Date(R), HT_Housewife(W), HT_Rooms(RW), TB_MRP_EMPLOYEE(R) |
| `ClickManternance` | HT_Book_Date(R), HT_Rooms(RW) |
| `ClickUSE` | HT_CheckIn_Ds(RW), HT_CheckIn_H(RW), HT_CheckIn_Product(RW), HT_ContinueTime(R), HT_Cupon(R), HT_Receipt_H(R), HT_Room_Status(RW), HT_Rooms(W) |
| `ClickUSE2` / `ClickUSE3` | HT_CheckIn_Ds(W) |
| `EMP_Note_Read` | HT_EMP_SMS(R) |
| `FormBookRooms` | HT_Room_Status(R), HT_Rooms(R) |
| `FormBookingInvoice` | HT_Book_Ds(R), HT_Book_H(R), HT_Book_Pro(R), HT_INVOICE(RW) |
| `FormConfirmPay` | TB_SET_Branch(R) |
| `FormEditPay` | HT_CheckIn_Pay(W) |
| `FormFolio` | TB_FOLIO(RW) |
| `FormLog` | HT_Log(R) |
| `FormManageOrderCust` | HT_Order_Up(RW), HT_SET_CusType(R), HT_SET_CusType_main(R) |
| `FormManageOrderCustDown` | HT_Order_DOwn(R), HT_Order_Down(W), HT_SET_CusType(R), HT_SET_CusType_main(R) |
| `FormReportAll` | HT_Bill_Debt_H(R), HT_Book_Ds(R), HT_CheckIn_H(R), HT_CheckIn_Product(R), HT_Products(R), HT_Rooms_Cancel(R) |
| `FormReportAll2` | + HT_CheckIn_Ds(R) |
| `FormRoomMain` | HT_Book_Date(W), HT_Book_Ds(R), HT_Book_H(R), HT_CheckIn_Ds(RW), HT_CheckIn_H(RW), HT_CheckIn_Pay(W), HT_CheckIn_Product(W), HT_EMP_SMS(R), HT_Housewife(W), HT_Receipt_H(R), HT_Room_SMS(RW), HT_Room_Status(W), HT_Rooms(RW), HT_Rooms_Price(R), HT_SET_RoomType(R) |
| `FormRoomMainClean` | HT_CheckIn_Ds(R), HT_EMP_SMS(R), HT_Room_SMS(R), HT_Rooms(R), HT_SET_RoomType(R) |
| `FormRoomMainKichen` | (same as Clean) |
| `FormRoomMain_ViewBook` | HT_Book_H(R), HT_CheckIn_Ds(R), HT_Rooms(R), HT_SET_RoomType(R) |
| `FormSMSSendManual` | TB_SMS_FAVORITES_2(RW) |
| `FormSearchBooking` | HT_Book_H(R) |
| `FormSearchBooking2` | HT_Book_H2(R) **← deprecated table** |
| `FormSearchPro` | HT_Products(R), HT_Products_Price(R), HT_SET_CusType(R) |
| `FormSearchRooms` | HT_Rooms(R) |
| `FormSearchRooms2` | + HT_Rooms_Price(R), HT_SET_CusType(R), HT_SET_RoomType(R) |
| `FormSearchRoomsCin` / `Cin2` | HT_Room_Status(R), HT_Rooms(R), HT_Rooms_Price(R), HT_SET_CusType(R) |
| `FormSelectRoom` | HT_Room_Status(R), HT_Rooms(RW) |
| `FormShowDEPBack` | HT_CheckIn_Ds(RW) |
| `FormShowVAT` | HT_Receipt_H(R) |
| `FormUpdateDateRoomAll` | HT_Room_Status(RW) |
| `FrmAddBook` | HT_Book_Date(W), HT_Book_Ds(W), HT_Book_H(RW), HT_CheckIn_Ds/H/Pay/Product(R), HT_Customers(RW), HT_Rooms(W), HT_Rooms_Price(R), HT_SET_CusType(R), HT_SET_Sale(R) |
| `FrmAddBook2` | + HT_Book_Pro(RW), HT_Products(R), HT_Products_Price(R), HT_Rooms(RW) |
| `FrmAddDep` | HT_Deposit(R) |
| `FrmAddInvoiceSale` | HT_CheckIn_Ds(R), HT_CheckIn_Product(R), HT_Invoice_Ds(RW), HT_Invoice_H(R), HT_Products(R), TB_SETTINGS(R) |
| `FrmAddPay` | TB_Pay_History(W), TB_SET_MyType2(R), TB_SET_MyType3(R) |
| `FrmAddReg` | HT_Register(R) |
| `FrmAddSale` | HT_CheckIn_Ds(R), HT_CheckIn_H(W), HT_CheckIn_Pay(W), HT_CheckIn_Product(R), HT_Customers(R), HT_Products(R), HT_Receipt_Ds(RW), HT_Receipt_H(R), TB_SETTINGS(R) |
| `FrmAddSale2` | HT_Products(RW), HT_Products_Price(R), HT_Receipt_Ds(RW), HT_Receipt_H(R), HT_SET_CusType(R) |
| `FrmAddSale2_Credit` | + HT_Bill_Debt_Ds(W), HT_Bill_Debt_H(R) |
| `FrmBookMain` | HT_Book_Date(W), HT_Book_Ds(R), HT_Book_H(RW) |
| `FrmBookMain2` | + HT_Rooms(W) |
| `FrmBookRooms` | HT_Book_Ds(RW), HT_Book_H(R), HT_Customers(R), HT_Room_Status(RW), HT_Rooms(R) |
| `FrmCheckIn` | HT_Book_Ds(R), HT_Book_H(RW), HT_Book_Pro(R), HT_CheckIn_Ds(RW), HT_CheckIn_H(RW), HT_CheckIn_Other_People(RW), HT_CheckIn_Product(RW), HT_Customers(R), HT_Order_Down(R), HT_Order_Up(R), HT_Products(RW), HT_Products_Price(R), HT_Room_Status(RW), HT_Rooms(RW), HT_Rooms_Price(R), HT_SET_CusType(R), HT_SET_CusType_Main(R), HT_SET_RoomType(R), Tb_Save_Image(RW) |
| `FrmCheckIn_EditOnly` | (subset of FrmCheckIn, no Book_*, no Rooms write) |
| `FrmCheckOut` | HT_CheckIn_Ds(R), HT_CheckIn_H(R), HT_CheckIn_Product(RW), HT_Customers(R), HT_Log_Debt(R), HT_Products(RW), HT_Products_Price(R), HT_Room_Status(W), HT_Rooms(W), HT_SET_CusType(R) |
| `FrmDepositMain` | HT_CheckIn_Ds(W) |
| `FrmDueBill` | HT_Round_Bill(W), TB_Due(R) |
| `FrmEditDate` | HT_CheckIn_Ds/H/Product(R), HT_Customers(R), HT_Room_Status(RW), HT_Rooms(W), HT_SET_CusType(R) |
| `FrmInOutMain` | HT_CheckIn_Ds(R), HT_CheckIn_Product(R), HT_Log_Debt(R), HT_Receipt_H(R) |
| `FrmManageCustomers` | HT_Customers(RW), HT_Order_Up(R), HT_SET_CusType(R), HT_SET_CusType_Main(R), Tb_Save_Image(R) |
| `FrmManageCustomersNew` | + HT_Bill_Debt_H(W), HT_Book_H(W), HT_CheckIn_H(W), HT_CheckIn_Pay(W), HT_Invoice_H(W), HT_Receipt_H(W) (cascade-update on customer rename), HT_Rooms_Price(R) |
| `FrmManageCustomersSearch` | HT_Customers(R) |
| `FrmManageProduct` | HT_Products(RW), HT_Products_Price(RW), HT_SET_CusType(R), HT_SET_ProductType(R) |
| `FrmManageRoom` | HT_Rooms(RW), HT_SET_RoomType(R) |
| `FrmPayAdd` | HT_CheckIn_Ds/H(R), HT_CheckIn_Product(RW), HT_Customers(R), HT_Log_Debt(R), HT_Products(R), HT_Products_Price(R), HT_SET_CusType(R) |
| `FrmPayAddDebt` | (same as FrmPayAdd) |
| `FrmPayAddPro` | + HT_Products(RW) |
| `FrmPayDebt` | HT_CheckIn_Ds(R), HT_Invoice_Note(R), HT_Log_Debt(R), HT_SET_CusType_Main(R) |
| `FrmPayDebt2` | HT_Bill_Debt_Ds(R), HT_Bill_Debt_H(RW), HT_Log_Debt(R) |
| `FrmPayMain` | TB_SET_MyType2(R), TB_SET_MyType3(R), TB_Sale_H(W), tb_pay_history(RW) |
| `FrmPermission` | TB_MRP_PERMISSION(W), TB_MRP_Permission(R) |
| `FrmPriceHistory` | HT_Log_Debt(R) |
| `FrmReceiptInvoice` | HT_Invoice_Ds(R), HT_Invoice_H(RW) |
| `FrmReceiptMain` | HT_CheckIn_H(W), HT_CheckIn_Pay(W), HT_Receipt_Ds(R), HT_Receipt_H(RW) |
| `FrmReceiptMain_invoice` | HT_CheckIn_Pay(RW), HT_CheckIn_Product(R), HT_Products(W) |
| `FrmReportBook2` | HT_Rooms(R) |
| `FrmReportHousewife` | HT_Housewife(R), TB_MRP_EMPLOYEE(R) |
| `FrmReportImcome` | TB_Pay_History(R), TB_SETTINGS(R) |
| `FrmReportImcome2` | TB_Pay_History(R) |
| `FrmReportPaybooking` | HT_Book_H(R), HT_CheckIn_Pay(W), TB_MRP_EMPLOYEE(R) |
| `FrmReportPower` | HT_POWER_LOG(R), HT_Rooms(R) |
| `FrmReportProducts` | HT_CheckIn_Product(R), HT_Products(R), HT_Products_Price(R), HT_SET_CusType(R), HT_SET_ProductType(R) |
| `FrmReportProductsSale` | HT_CheckIn_Product(R), HT_Products(R), HT_SET_ProductType(R) |
| `FrmReportRR4` | TB_SETTINGS(R) |
| `FrmReportRecPay` | TB_SETTINGS(R), TB_SET_MyType2(R), TB_SET_MyType2_2(R), TB_SET_MyType3(R), tb_pay_history(R) |
| `FrmReportSale1` | HT_CheckIn_Pay(W), HT_Rooms(R), HT_SET_Sale(R), TB_MRP_EMPLOYEE(R) |
| `FrmReportSale2` | HT_CheckIn_Pay(W), HT_Rooms(R), TB_MRP_EMPLOYEE(R) |
| `FrmReportShift` | HT_CheckIn_Pay(R), HT_CheckIn_Product(R) |
| `FrmReportShiftCash` | HT_CheckIn_Ds(R), HT_CheckIn_Pay(RW), HT_Rooms_Cancel(R), TB_MRP_EMPLOYEE(R), TB_Pay_History(R) |
| `FrmReportrepair` | HT_Rooms_Repair(R) |
| `FrmSETBranch` | TB_SET_Branch(RW) |
| `FrmSETCsuType` | HT_Customers(RW), HT_Order_Down(W), HT_Order_Up(W), HT_Products_Price(W), HT_Rooms_Price(W), HT_SET_CusType_Main(RW) |
| `FrmSETCsuTypeMain` | (mirror of above, swapped CusType) |
| `FrmSETMyType2` | TB_SET_MyType2(RW), TB_SET_MyType2_2(R) |
| `FrmSETMyType2_2` | TB_SET_MyType2(R), TB_SET_MyType2_2(RW), TB_SET_MyType3(R) |
| `FrmSETMyType3` | TB_SET_MyType2_2(R), TB_SET_MyType3(RW) |
| `FrmSETProType` | HT_Products(RW), HT_Products_Price(RW), HT_SET_ProductType(RW) |
| `FrmSETRoomType` | HT_Rooms(RW), HT_Rooms_Price(RW), HT_SET_CusType(R), HT_SET_RoomType(RW) |
| `FrmSETTimeContnue` | HT_ContinueTime(RW) |
| `FrmSETsale` | HT_Customers(R), HT_SET_Sale(RW) |
| `FrmSaleMain2` | HT_Bill_Debt_Ds(R), HT_Bill_Debt_H(RW), HT_CheckIn_Pay(W), HT_Products(W) |
| `FrmSearchStock` | HT_Products(R) |
| `FrmSettings` | TB_SETTINGS(RW) |
| `FrmShowBookNotify` | HT_Book_Date(W), HT_Book_Ds(R), HT_Book_H(RW) |
| `FrmUseCount` | HT_CheckIn_Ds(R), HT_CheckIn_H(R) |
| `FrmUser` | TB_MRP_EMPLOYEE(RW), TB_MRP_PERMISSION(R) |
| `GForm0` | Tb_Save_Image(RW) |
| `INV_Note` | HT_Invoice_Note(RW) |
| `Module1` (god class) | HT_Book_Ds(R), HT_Book_H(RW), HT_CheckIn_H(RW), HT_CheckIn_Pay(R), HT_Cupon(R), HT_Customers(RW), HT_Log_Debt(W), HT_POWER_LOG(RW), HT_Room_Status(R), HT_Rooms(RW), HT_Rooms_Price(R), HT_Round_Bill(R), TB_Pay_History(R), TB_SETTINGS(R) |
| `Print_Report` (print orchestrator) | HT_Bill_Debt_*, HT_Book_H, HT_Book_Pro, HT_CheckIn_*, HT_Cupon(W), HT_INVOICE(R), HT_Invoice_Note(R), HT_Receipt_*, TB_FOLIO(R), TB_SETTINGS(R), Tb_Save_Image(R) |
| `ReportCleanRoom` | HT_Rooms(R) |
| `ReportContnueRoom(2)` | TB_SETTINGS(R) |
| `ReportCustChange` | TB_SETTINGS(R) |
| `ReportCustDays` | HT_CheckIn_Ds(R), TB_SETTINGS(R) |
| `ReportCustIn` | HT_CheckIn_Product(R), HT_SET_*(R), TB_SETTINGS(R) |
| `ReportCustOut(Today)(2)` | HT_CheckIn_Product(R), HT_SET_CusType_Main(R), TB_SETTINGS(R) |
| `ReportDays` | TB_SETTINGS(R) |
| `ReportTax` | HT_Receipt_H(R), TB_SETTINGS(R) |
| `Room_Note_Read` | HT_Room_SMS(R) |
| `frmMain1` (god form) | HT_Bill_Debt_H(R), HT_Book_Date/Ds/H/Status, HT_Changed_Room(R), HT_CheckIn_*, HT_Customers, HT_Receipt_*, HT_Room_SMS(W), HT_Room_Status(RW), HT_Rooms(RW), HT_Rooms_Cancel(R), HT_Rooms_Price(W), HT_Round_Bill(R), TB_MRP_EMPLOYEE(R), TB_SETTINGS(RW), TB_SET_Branch/MyType*(W), Tb_Save_Image(W), Tb_Version(RW) |
| `frmReg` | TB_SETTINGS(RW) |
| `frmTimeTable` | HT_Room_Status(R), HT_Rooms(R) |
| `login` | TB_MRP_EMPLOYEE(R), TB_MRP_Permission(R), TB_SETTINGS(R) |

**Tables NOT touched by any module form** (dead schema?): `HT_Book_H2`, `HT_Book_Date2`, `HT_book_date` (lowercase variant), `HT_Book_ds` (lowercase variant) — these appear only in legacy/copy forms. Likely candidates to drop in the rewrite.

---

## 6. End-to-End User Journeys

### J1. Walk-in Check-in (most common path)

```
frmMain1 (Tab: หน้าหลัก, RibbonBar1)
  └─ B1 (รายการห้องพัก) → FormRoomMain
       └─ click an empty room → ClickAvliable     [confirm room is "ว่าง"]
            └─ "Check-In" button on grid → FrmCheckIn  (Module1.check_round_bill required)
                 ├─ "อ่านจาก SmartCard" → spawns KPThaiNationalIDCard.exe
                 │     └─ FrmShowPreviewSmartCard (pic + progress)
                 ├─ "ถ่ายรูป" → FrmAddSaveImage / frmCapture (Twain or webcam)
                 │     └─ writes Tb_Save_Image
                 ├─ pick room price → reads HT_Rooms_Price + HT_SET_CusType
                 ├─ deposit → writes HT_CheckIn_Pay
                 └─ confirm → writes:
                      INSERT HT_CheckIn_H + HT_CheckIn_Ds (one row per room/night)
                      INSERT HT_CheckIn_Pay (deposit)
                      UPDATE HT_Rooms SET Room_Use='yes', Room_Book='', Room_Clean='no'
                      INSERT HT_Room_Status (room_status='เข้าพัก')
                      UPSERT HT_Customers
                      INSERT HT_CheckIn_Other_People
                      Print_Report → ReportReg_1/2/3 (registration form via Crystal)
```

### J2. Check-out + Settle

```
FormRoomMain → click occupied room → ClickUSE
  └─ "Check-Out" → new FrmCheckOut()
       ├─ FormSearchChechInnotOut (find checkin if multi-room)
       ├─ pull all HT_CheckIn_Product, HT_CheckIn_Pay
       ├─ optionally FrmAddSale (last-minute consumables)
       ├─ optionally FormShowDEPBack (refund deposit)
       ├─ FormConfirmOverBill (if overcharge)
       ├─ FormConfirmPay → take final payment
       └─ commit:
            UPDATE HT_CheckIn_H SET checkout_date, total
            INSERT HT_CheckIn_Pay (final)
            UPDATE HT_Rooms SET Room_Use='no', Room_Clean='no' (now dirty)
            UPDATE HT_Room_Status SET room_status='Check-Out'
            Print_Report → sale_vat / sale_vat0 / Folio
       └─ optionally FormSMS_DEBT (send debt SMS if outstanding)
```

### J3. Reservation (room-specific) → Check-in

```
frmMain1 → ButtonItem_0_Click → FrmBookMain2
  └─ "เพิ่มการจอง" → FrmAddBook (or FrmAddBook2 if rooms picked)
       ├─ pick customer (HT_Customers, optional UPSERT)
       ├─ pick rooms × dates → FrmBookRooms
       │     └─ INSERT HT_Book_Date (one row per room×date), HT_Book_Ds
       ├─ optional product attach → INSERT HT_Book_Pro
       ├─ pay deposit → INSERT HT_Book_H.Book_Price_Pay
       └─ INSERT HT_Book_H, UPDATE HT_Rooms.Room_Book*
            Print_Report → ReportBooking
       
On arrival day:
frmMain1 ButtonNotification dropdown → FrmShowBookNotify
  └─ "เปลี่ยนเป็น Check-In" → ButtonItem10_Click(book_no=...) → FrmCheckIn (pre-filled)
       └─ same as J1 from "confirm" step,
            additionally UPDATE HT_Book_H SET Book_Status='เข้าพัก'
```

### J4. Sale to Room (in-stay POS)

```
FormRoomMain → click occupied room → ClickUSE
  └─ "ขายสินค้าเข้าห้อง" → MyProject.Forms.FrmAddSale.ShowDialog()
       ├─ FormSearchPro (product picker, HT_Products + Price by CusType)
       ├─ adjust qty/price
       └─ Save:
            INSERT HT_Receipt_H + HT_Receipt_Ds
            INSERT HT_CheckIn_Product (link to HT_CheckIn_H)
            UPDATE HT_Products.Stock
            optionally Print_Report → sale (POS slip)
```

### J5. Round-Bill Open / Close (shift)

```
frmMain1 → จัดการรอบบิล → FrmDueBill
  ├─ "เปิดรอบบิล" → INSERT HT_Round_Bill (open_date, open_emp, open_cash)
  └─ at end: "ปิดรอบบิล" → UPDATE HT_Round_Bill (close_date, close_cash)
       └─ FrmReportShiftCash → Crystal Report sale_cin_credit (cash count + variance)
```

`Module1.check_round_bill()` is called by every transactional handler; without an open round, all sales/check-in are blocked.

### J6. Tax Invoice Issue

```
frmMain1 → ButtonItem32_Click_2 → FrmReceiptInvoice
  └─ pick a checkin / receipt → FrmAddInvoiceSale
       └─ INSERT HT_Invoice_H + HT_Invoice_Ds
            (Print_Report → inv_sale_vat / inv_sale_vat0_debt_hhotel)

(alternate) FormBookingInvoice from booking → INSERT HT_INVOICE for prepaid rooms
```

### J7. Housewife / Cleaning Workflow

```
login → if loginMode='housewife' or HouseWifeMode set → FormRoomMainClean (instead of frmMain1)
  └─ shows only dirty/checkout rooms
       ├─ click → ClickClean (mark started cleaning) → INSERT HT_Housewife
       └─ click again when done → ClickCleanOK → INSERT HT_Housewife (with finish_emp)
       reports: FrmReportHousewife (counts by employee) + ReportCleanRoom
```

---

## 7. Hardware & Integration Touchpoints

| Touchpoint | File(s) | Notes |
|---|---|---|
| **Serial port (power relay)** | `frmMain1.cs:367,538,4974,7934-7992` (`SerialPort1`, `SerialPort2`), `FrmSettings.cs:6143-6789` | Two `System.IO.Ports.SerialPort` instances. `SerialPort1.PortName = Module1.POWER_PORT` (default COM1). Used to power-cycle room electricity via relay. `URL_ON_OFF_SERIALS` queue — strings written to the port. `SerialPort2.PortName="COM3"` configured but receive handler not wired in. Cash drawer also via `Module1.CASH_PORT`. |
| **Thai national-ID smart card** | `FrmCheckIn.cs:11195-11513`, `FrmCheckIn_EditOnly.cs:8897-9208`, `FrmManageCustomersNew.cs:3725-3936`, `FrmShowPreviewSmartCard.cs` | App calls `Module1.GenSmartCard()` then spawns external `KPThaiNationalIDCard.exe` (separate exe in install dir) which writes the card data to disk; `FrmShowPreviewSmartCard.loadpic()` polls progress. **No** in-process pcsc-sharp / SCard P/Invoke — it's all driven by a sister .exe. |
| **Webcam capture** | `frmCapture.cs`, `iCam.cs`, `FrmAddSaveImage.cs` | DirectShow (likely DirectShowLib). Saves to `Tb_Save_Image` BLOB. |
| **TWAIN scanner** | `Twain.cs`, `TwainCommand.cs`, `TwainHandler.cs` + 14 helper files | Open-source TWAIN.NET wrapper. **Also** invokes external `TwainGui.exe` via `FrmAddSaveImage.cs:910` (`Process.Start("TwainGui.exe")`) — second path. |
| **SMS** | `FrmSettingsSMS.cs:649,708`, `FormSMSSendManual.cs`, `FormSMSLog.cs`, `FormSMS_DEBT.cs`, `Room_Note.cs`, `EMP_Note.cs` | KP SMS gateway via WebBrowser: `http://www.kpsystem.co.th/sms/sms.php?mode=check&u=...&p=...`. Send is similar URL with phone+message params. Local tables: `TB_SMS_FAVORITES_2`, `HT_Room_SMS`, `HT_EMP_SMS`. |
| **Printer / Crystal Reports** | `Print_Report.cs`, `PrintFactory.cs`, `FrmPrint.cs`, all `Crystal*.cs` + `Cached*.cs` | Crystal Reports for Visual Studio runtime. ~30 .rpt files. Default `PrinterName` from settings; `Receipt_Report`, `POS_Report`, `Cupon_Report`, `Tax_preview`, `Cin_Print`, `inv_print` choose the .rpt per task. Receipt formats include 58mm and 80mm thermal variants (`sale2_58.rpt` / `sale2_80.rpt`, `ReportDep2_58/80`). |
| **External `.exe` invocations** | `Process.Start(...)` calls | `HOTEL.exe` (relaunch on server-change), `KPThaiNationalIDCard.exe` (smart card), `TwainGui.exe` (alt scanner UI). |
| **Online services** | `frmMain1.cs:7094,7105,8365,8494`, `FrmSettingsSMS.cs:649`, `FrmUpdate.cs` | All to `kpsystem.co.th`: `version_hotel.php` (version check), `chk_hotel.php` (kill switch / blocked-machine list), `sms/sms.php` (SMS gateway). `FormUPDATE` posts public IP for DDNS. |
| **License / activation** | `Module1.cs:30,450` (`string_0` MAC whitelist), `frmMain1.method_0` (registry `HKLM\Software\microsoft\MSXKPHTEL`), `frmReg.cs`, `FrmUpdate.cs` | TripleDES-encrypted reg code stored in registry + file `reg.txt`. CPU/HDD serial enumeration via WMI (`Win32_DiskDrive`). |
| **MS Access fallback DB** | `Module1.cs:721` | OleDb provider with embedded password `foreverbu`. Path `kphotel.accdb`. |
| **Auto-update** | `FrmUpdate.cs`, `FormUPDATE.cs` | `FrmUpdate` downloads new `HOTEL.exe` from kpsystem; `FormUPDATE` is the periodic-DDNS form. |

---

## 8. Cruft / Skip in Rewrite

### Definitely-dead source files
- `Form1.cs` — empty design test form, unreferenced.
- `FormRoomMainKichen_old.cs` — superseded by `FormRoomMainKichen.cs`.
- `FormSelectDB_old.cs` — superseded by `FormSelectDB.cs`. Still referenced from `frmMain1.cs:7249` (a hidden ButtonItem) — verify before deletion.
- `FormSearchRooms2_old.cs` — superseded by `FormSearchRooms2.cs`. Touches dead table `HT_Book_Date2`.
- `FrmAddBook2copy.cs` — explicit "copy" of `FrmAddBook2.cs`.
- `Cachedsale_vat0_copy.cs`, `sale_vat0_copy.cs`, `Cachedsale_vat_copy.cs`, `sale_vat_copy.cs` — duplicate Crystal Report wrappers.
- `ReportShipCashOLD.cs` / `CachedReportShipCashOLD.cs` — superseded by ReportShipCash.
- `Class0.cs`, `Class1.cs`, `Class2.cs`, `-Module--DE8036EB-...-.cs` — obfuscator runtime stubs (`Class2.LH6iGfYz9j3MJ()` is a stub call). Required to compile decompile but adds nothing.
- `GClass3.cs`, `GClass4.cs`, `GForm0.cs`, `GForm1.cs` — generic-named decompiler artifacts. `GForm0` is real (image picker) but `GForm1` looks empty.

### Deprecated tables in schema
- `HT_Book_H2` — only read by `FormSearchBooking2`. Use `HT_Book_H` only.
- `HT_Book_Date2` — only `FormSearchRooms2_old`. Drop.
- Lowercase aliases: `HT_book_date`, `HT_Book_ds`, `Tb_Save_Image` vs `tb_save_image`, `HT_customers` vs `HT_Customers`, `HT_invoice_H/Ds` vs `HT_Invoice_*`, `tb_pay_history` vs `TB_Pay_History` — these are case variants used inconsistently. SQL Server is case-insensitive by default so they refer to the same tables, but normalize during rewrite.
- `Tb_Version` — used only by `frmMain1.Update_Version()`. Probably auto-managed schema version; consolidate.

### Forms that look orphan / never opened from menu
- `FrmCustomers.cs` — present, but no `ShowDialog`/`Show` reference found. `FrmManageCustomersNew` is what the menu opens. Likely earlier customer screen — DEAD candidate.
- `FrmCheckIn_EditOnly.cs` — opened from inside `ClickUSE` ("Edit checkin") rather than the menu; legitimate but very large duplicate of `FrmCheckIn`. Consider unifying with FrmCheckIn behind an `EditOnly` mode flag in the rewrite.
- `FormART.cs` — shown by `frmMain1.cs:6471` but appears to be a decorative splash; verify if used. Likely DEAD.
- `FrmCustomers`, `FrmAddPay` (different from `FormConfirmPay`) — verify if reachable.
- `FormSelectDB_old`, `connect_mssql.cs` — alternative DB-connect dialogs reachable only via dev-only ribbon paths.

### Duplicate functionality to consolidate
- `FrmReceiptMain` vs `FrmReceiptMain_invoice` — 90% same code, only doc type differs.
- `FrmAddSale` / `FrmAddSale2` / `FrmAddSale2_Credit` — 3 sale entry forms; converge into one with `mode` enum.
- `FrmPayDebt` (checkin-debt) vs `FrmPayDebt2` (sales-debt) — split by source table; in rewrite, treat as a single Debt module with filter.
- `FrmBookMain` vs `FrmBookMain2` — split is "with-room vs without-room"; better as one screen with toggle.
- `FrmAddBook` / `FrmAddBook2` / `FrmAddBook3` (referenced in ClickBook) / `FrmAddBook4` — booking entry variants. Only `FrmAddBook` and `FrmAddBook2` are present as files; AddBook3/4 are likely planned but not implemented (still-referenced typed names, may be `FrmBookMain` aliases). Inspect before deciding.
- `FormManageOrderCust` / `FormManageOrderCustDown` — just up/down sign — combine.
- `FrmSETMyType2` / `FrmSETMyType2_2` / `FrmSETMyType3` — 3-level category tree split into 3 forms. Replace with one tree control.
- `FormRoomMainClean` / `FormRoomMainKichen` — alternate role-restricted shells; both are heavy. In rewrite, gate by RBAC, not by separate forms.
- `ClickUSE` / `ClickUSE2` / `ClickUSE3` — different in-use scopes; converge.

### Hardcoded production endpoints to replace
- `http://www.kpsystem.co.th/version_hotel.php` (HTTP, not HTTPS).
- `http://www.kpsystem.co.th/chk_hotel.php` — **kill switch** that can disable the install remotely. **Critical to neutralize for in-house rewrite.**
- `http://www.kpsystem.co.th/sms/sms.php` — replace with parametric SMS provider.
- Embedded MS Access password `foreverbu`.
- TripleDES key `ruj5de4` in `Module1.ReadDB_2018()` and `FormEN_DE.Decrypt1`.
- The 250+ MAC/serial whitelist in `Module1.string_0` — relic of old per-machine licensing.

---

## Appendix A: File counts

- Total `.cs` files in `iHOTEL2025/`: 308
- Form classes (`Form*` / `Frm*` / `frm*` / `Click*`): 132
- Distinct functional forms (excluding 30 Crystal Report wrappers + 30 Cached* + 17 Twain* + ~10 obfuscator stubs): **~85**
- Distinct DB tables touched across all functional forms: **~50** (out of 61 in `_SCHEMA.sql`)
