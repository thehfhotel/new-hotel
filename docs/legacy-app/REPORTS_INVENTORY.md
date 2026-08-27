# Crystal Reports Inventory — Hotel-2018 V.1.45

> Inventory of all Crystal Reports (`.rpt`) files used by the legacy iHOTEL2025 VB.NET app, prepared as input to a modern replacement (QuestPDF / RDLC / HTML→PDF). Cross-referenced with the typed-DataSet wrappers, the `Cached*` plumbing, the central `Print_Report` orchestrator, the `Datalocal` in-memory cache, and the launcher Forms (`Frm*`, `Form*`, etc.).
>
> Sources:
> - 30+ standalone `.rpt` files at the project root (also duplicated under `reports/`, `reports00/`, `report/` and inside `_decompiled\iHOTEL2025.*.rpt` — embedded resources extracted from the assembly).
> - Decompiled wrappers / cached wrappers / typed datasets at `_decompiled_clean\iHOTEL2025\`.
> - Central printing module: `_decompiled_clean\iHOTEL2025\Print_Report.cs` (~3,170 lines).
> - In-memory typed-DataSet container: `_decompiled_clean\iHOTEL2025\Datalocal.cs` (`Module1.localdata`).
> - Print routing globals in `Module1.cs:184-216` (`Receipt_Report`, `POS_Report`, `Cupon_Report`, `Tax_preview`, `inv_print`, `Receipt_print/preview`, `POS_print/preview`, etc.).

---

## 1. Summary table — every report

Every distinct `.rpt` file shipped with the application. Most reports come in **families** (`*_58.rpt` / `*_80.rpt` are 58 mm / 80 mm thermal-printer variants of the same content; `*_copy.rpt` is a duplicate slip ("merchant copy" or "second voucher"); `*0` denotes the no-VAT variant; `SB`/`CB` are short/copy-bill layouts; `_RR4` is the legal A4 RR.4 form). A "—" in **Wrapper class** means the report is loaded purely by file path through `ReportDocument.Load(...)` and has no compiled typed wrapper. A "—" in **Dataset class** means it does not bind a typed dataset (binds raw `Module1.connect()` rows or shares a global table inside `Module1.localdata`).

| Report file | Wrapper class (`iHOTEL2025.*`) | Dataset class / table in `Module1.localdata` | Purpose | Triggered from | Output medium | Paper-size hint | Module |
|---|---|---|---|---|---|---|---|
| `sale.rpt` | `sale` | `Datalocal.ReportBillCash` | Continuous-paper / dot-matrix sales receipt (default `กระดาษต่อเนื่อง`) | `Print_Report.Print_Sale` (called by FrmCheckOut, FrmReceiptMain, FrmAddSale, FrmCheckIn, ClickUSE) | Printer + preview | A4 / continuous form | Sales / Receipt |
| `sale2_58.rpt` | `sale2_58` | `Datalocal.ReportBillCash` | Sales receipt for 58 mm thermal printer | `Print_Report.Print_Sale` (when `Receipt_Report = "เครื่องพิมพ์ใบเสร็จ (58mm)"`) | Thermal printer | 58 mm | Sales / Receipt |
| `sale2_80.rpt` | `sale2_80` | `Datalocal.ReportBillCash` | Sales receipt for 80 mm thermal printer | `Print_Report.Print_Sale` (80 mm path) | Thermal printer | 80 mm | Sales / Receipt |
| `sale3_folio.rpt` | `sale3_folio` | `Datalocal.ReportBillCash`, `Datalocal.Bill_H` | Folio-style receipt ("HHOTEL"/"Guest Folio" mode) printed at check-out | `Print_Report.smethod_0` (folio variant of Print_Sale) | Printer + preview | A4 | Folio / Receipt |
| `sale_credit.rpt` | `sale_credit` | `Datalocal.ReportBillCredit` | Credit-sale receipt (deferred payment, `HT_Bill_Debt_*`) | `Print_Report.Print_Sale_Credit` (FrmAddSale2_Credit, FrmPayDebt2, FrmSaleMain2) | Printer + preview | A4 / continuous | Credit Sales |
| `sale_cin_credit.rpt` | `sale_cin_credit` | (typed `sale_cin_credit` dataset) | Credit-sale receipt linked to a check-in (room charge → credit) | `Print_Report` (credit-from-checkin path) | Printer | A4 | Credit Sales |
| `sale_vat.rpt` | `sale_vat` | `Datalocal.ReportBillVat` (+ `Bill_H`) | VAT receipt with line-level tax ("เครื่อง 80mm" or "กระดาษต่อเนื่อง" w/ VAT) | `Print_Report.Print_Sale_Vat` (Print_Report.cs, ~lines 1001, 1163) | Printer + preview | A4 / 80 mm | Tax Receipt |
| `sale_vat_copy.rpt` | `sale_vat_copy` | `Datalocal.ReportBillVat` | Customer/merchant **copy** of `sale_vat` (second slip — different watermark) | `Print_Report.Print_Sale_Vat` (Print_Report.cs, ~lines 1014, 1185) | Printer | Same as `sale_vat` | Tax Receipt |
| `sale_vat0.rpt` | `sale_vat0` | `Datalocal.ReportBillVat` | VAT-zero receipt (Vat-inclusive style: VAT broken out from total, not added) | `Print_Report` (Print_Report.cs, ~lines 931, 1072) | Printer + preview | A4 / 80 mm | Tax Receipt |
| `sale_vat0_copy.rpt` | `sale_vat0_copy` | `Datalocal.ReportBillVat` | Copy of `sale_vat0` | `Print_Report` (Print_Report.cs, ~lines 944, 1091) | Printer | Same as above | Tax Receipt |
| `sale_vat0SB.rpt` | `sale_vat0SB` | `Datalocal.ReportBillVat` | VAT-0 — **Short Bill** layout (compressed thermal) | `Print_Report` (Print_Report.cs, ~lines 959, 1029, 1110, 1208) | Thermal printer | 58/80 mm | Tax Receipt |
| `sale_vat0CB.rpt` | `sale_vat0CB` | `Datalocal.ReportBillVat` | VAT-0 — **Copy Bill** (printed merchant copy of SB) | `Print_Report` (Print_Report.cs, ~lines 974, 1044, 1137, 1235) | Thermal printer | 58/80 mm | Tax Receipt |
| `sale_vat - Copy.rpt` | — | — | Backup duplicate of `sale_vat.rpt` (file-system copy left by developer) | not loaded directly | — | — | DEAD / backup |
| `inv_sale_vat.rpt` | `inv_sale_vat` | (typed `inv_sale_vat` dataset) | A4 VAT-invoice variant (separate doc from receipt) | `FrmReceiptInvoice` (via `Print_Report` `inv_print` branch) | Printer + preview | A4 | Invoice / VAT |
| `inv_sale_vat0.rpt` | `inv_sale_vat0` | (typed `inv_sale_vat0` dataset) | A4 VAT-zero invoice | Same as above | Printer | A4 | Invoice / VAT |
| `inv_sale_vat0_debt.rpt` | `inv_sale_vat0_debt` | (typed dataset) | Debt-receipt-style invoice (accounts receivable) | `FrmReportRecPay` and `FrmPayDebt*` | Printer + preview | A4 | Debt / AR |
| `inv_sale_vat0_debt_hhotel.rpt` | `inv_sale_vat0_debt_hhotel` | (typed dataset) | "HHOTEL" hotel-style A4 layout for the same | Same | Printer | A4 | Debt / AR |
| `inv_sale_vat0_debt_hhotel_no.rpt` | `inv_sale_vat0_debt_hhotel_no` | (typed dataset) | HHOTEL layout, **without** VAT line (zero-rated guest) | Same | Printer | A4 | Debt / AR |
| `inv_sale_vat0_debt_hhotel_other.rpt` | `inv_sale_vat0_debt_hhotel_other` | (typed dataset) | HHOTEL layout for **other-charges** debt only | Same | Printer | A4 | Debt / AR |
| `inv_sale_other_vat.rpt` | — | — (loaded by file path) | Standalone "other-charges" VAT invoice (non-room income) | `FrmReceiptInvoice` (when Type ≠ room) | Printer | A4 | Invoice / VAT |
| `inv_sale_other_novat.rpt` | — | — | Standalone "other-charges" non-VAT invoice | Same | Printer | A4 | Invoice / VAT |
| `invoice_room.rpt` | — | — | Room invoice template (legacy / standalone) | Loaded by `ReportDocument.Load("invoice_room.rpt")` (FrmReceiptInvoice fallback) | Printer | A4 | Invoice |
| `ReportSaleVat.rpt` | `ReportSaleVat` | `Datalocal.ReportSaleVat` | Sales-VAT report (period summary of VAT-eligible sales) | `ReportTax` form (ReportTax.cs:401-405) | Preview | A4 (landscape) | Reports / Tax |
| `ReportBooking.rpt` | `ReportBooking` | `Datalocal.ReportBooking` (+ `Bill_H`) | Booking confirmation slip (rooms + add-on products) | `Print_Report.print_booking` (FrmAddBook, FrmReportBook) | Printer + preview | A4 | Booking |
| (`ReportBookingINV.rpt`, embedded) | `ReportBookingINV` | `Datalocal.ReportBookingINV` | Booking **invoice** (booking + HT_INVOICE row) | `Print_Report.print_inv_booking` (FormBookingInvoice, FrmReportBook2) | Printer + preview | A4 | Booking / Invoice |
| `ReportReg_1.rpt` | `ReportReg_1` | `Datalocal.ReportReg_1` (+ `Bill_H`) | Guest registration form — 1-up layout | `Print_Report.PrintReg`/`PrintReg2` (FrmCheckIn, FrmRegMain) | Printer + preview | A4 portrait | Check-in / Registration |
| `ReportReg_2.rpt` | `ReportReg_2` | `Datalocal.ReportReg_1` | Guest registration form — 2-up (two slips per page) | Same (`Module1.Reg_Print = "2"` selector) | Printer + preview | A4 (split) | Check-in / Registration |
| `ReportReg_3.rpt` | `ReportReg_3` | `Datalocal.ReportReg_1` | Guest registration form — 3-up | Same (`"3"` selector) | Printer | A4 (split 3) | Check-in / Registration |
| (`ReportReg2.rpt`, embedded) | `ReportReg2` | `Datalocal.ReportReg2` | Newer registration sheet (different fields — incl. national-ID barcode) | `Print_Report.PrintReg2_NEW` (Print_Report.cs, ~lines 1647-1671) | Printer + preview | A4 portrait | Check-in / Registration |
| `ReportFolio1.rpt` | `ReportFolio1` | `Datalocal.ReportFolio1` (+ `Bill_H`) | Guest folio (statement) — page 1 layout | `Print_Report.PrintFolio1` (Print_Report.cs, ~line 2247) (ClickUSE → FormFolio) | Printer + preview | A4 portrait | Folio |
| `ReportFolio1_2.rpt` | `ReportFolio1_2` | `Datalocal.ReportFolio1_2` | Folio — continuation page 2 | Same routine, paginates rows 1-19 then overflows to `_2` | Printer | A4 portrait | Folio |
| `ReportFolio2.rpt` | `ReportFolio2` | `Datalocal.ReportFolio2` (+ `Bill_H`) | Alternative folio layout (multi-room / multi-guest grid) | `Print_Report.PrintFolio2` (Print_Report.cs, ~line 2268) | Printer + preview | A4 landscape | Folio |
| `ReportFolio2 - Copy.rpt` | — | — | Backup duplicate | not loaded | — | — | DEAD / backup |
| `ReportDep.rpt` | `ReportDep` | `Datalocal.ReportDep` (+ `Bill_H`) | Deposit receipt (รับเงินมัดจำ / refund) — A4 / continuous | `Print_Report.PrintDep` (Print_Report.cs, ~line 3112) (FrmAddDep, FrmReportPaybooking, FrmReportMudjumRec/Back) | Printer | A4 / continuous | Deposit |
| `ReportDep2_58.rpt` | `ReportDep2_58` | `Datalocal.ReportDep` | Deposit receipt 58 mm thermal | `Print_Report.PrintDep` | Thermal printer | 58 mm | Deposit |
| `ReportDep2_80.rpt` | `ReportDep2_80` | `Datalocal.ReportDep` | Deposit receipt 80 mm thermal | `Print_Report.PrintDep` | Thermal printer | 80 mm | Deposit |
| `ReportCupon58.rpt` | `ReportCupon58` | `Datalocal.ReportCupon` (+ `Bill_H`) | Food/breakfast coupon — 58 mm thermal | `Print_Report.PrintCupon` (Print_Report.cs, ~lines 2570/2693) (FrmCuponMain, FrmCheckIn) | Thermal printer | 58 mm | Coupon |
| `ReportCupon80.rpt` | `ReportCupon80` | `Datalocal.ReportCupon` | Food/breakfast coupon — 80 mm thermal | Same (Print_Report.cs, ~lines 2563/2686) | Thermal printer | 80 mm | Coupon |
| `Report_RR4.rpt` | `Report_RR4` | `Datalocal.ReportRR4` | Thai government **RR.4** guest-registry form (sent to อำเภอ) | `FrmReportRR4` (FrmReportRR4.cs, ~line 781) | Printer + preview | A4 portrait | Government / Compliance |
| `ReportPictures.rpt` (embedded) | `ReportPictures` | `Datalocal.ReportPic` | Single-page guest photo print (`Tb_Save_Image.pic` blob) | `Print_Report.Print_Picture1` (Print_Report.cs, ~line 3157) (FrmManageCustomersNew, FrmCheckIn) | Printer + preview | A4 portrait | Customer / Image |
| `ReportIncome.rpt` (embedded) | `ReportIncome` | `Datalocal.ReportSale` | Income / receipts–payments report (date range) | `FrmReportRecPay.cs:711`, `FrmReportImcome` (`R17`) | Preview | A4 landscape | Reports / Accounting |
| `ReportIncome2.rpt` (embedded) | `ReportIncome2` | `Datalocal.ReportSale` | Income report by **round bill** (`HT_Round_Bill`) | `FrmReportRecPay.cs:704`, `FrmReportImcome2` (`R18`) | Preview | A4 landscape | Reports / Accounting |
| `ReportShipCash.rpt` (embedded) | `ReportShipCash` | (typed `ReportShipCash` dataset) | Shift-close cash count + sales summary (current layout) | `FrmReportShiftCash.cs:1819` | Preview / Printer | A4 portrait | Reports / Shift |
| `ReportShipCashOLD.rpt` (embedded) | `ReportShipCashOLD` | (typed dataset) | Legacy shift-close report (kept for backward compat) | (referenced, fallback) | Preview / Printer | A4 portrait | Reports / Shift |
| `CrystalReportDays.rpt` (embedded) | `CrystalReportDays` | (built-in section bands, no dataset binding) | Daily summary report (rooms used, sales, deposits) | `ReportDays` form (`R1` ribbon button) | Preview | A4 portrait | Reports / Daily |
| `CrystalReportDaysContenue.rpt` (embedded) | `CrystalReportDaysContenue` | — | "Continuing rooms" daily report — guests still in-house | `ReportContnueRoom` form | Preview | A4 portrait | Reports / Daily |
| `CrystalReportDaysContenue2.rpt` (embedded) | `CrystalReportDaysContenue2` | — | Variant of above (different filter / 2nd layout) | `ReportContnueRoom2` form | Preview | A4 portrait | Reports / Daily |
| `CrystalReportCustIn.rpt` (embedded) | `CrystalReportCustIn` | — | Guests currently checked in | `ReportCustIn` form (`R4`) | Preview | A4 portrait | Reports / Guests |
| `CrystalReportCustOut.rpt` (embedded) | `CrystalReportCustOut` | — | Guests checked out (history) | `ReportCustOut` form (`R3`) | Preview | A4 portrait | Reports / Guests |
| `CrystalReportCustOutToday.rpt` (embedded) | `CrystalReportCustOutToday` | — | Guests checking out today | `ReportCustOutToday` form (`ButtonItem51`) | Preview | A4 portrait | Reports / Guests |
| `CrystalReportCustOutTodayHousewife.rpt` (embedded) | `CrystalReportCustOutTodayHousewife` | — | Today's check-outs **for housekeeping** (room-cleaning prep list) | `ReportCustOutToday2` form (`ButtonItem63`) | Preview | A4 portrait | Reports / Housekeeping |
| `CrystalReportCustChange.rpt` (embedded) | `CrystalReportCustChange` | — | Room-change events log (who moved from A→B and when) | `ReportCustChange` form (`ButtonItem53_Click` / `52_Click_1`) | Preview | A4 portrait | Reports / Rooms |
| `CrystalReportCustStay.rpt` (embedded) | `CrystalReportCustStay` | — | Total nights stayed per guest (loyalty) | (form: `ReportCustDays` etc.) | Preview | A4 portrait | Reports / Guests |
| `CrystalReportCustStay2.rpt` (embedded) | `CrystalReportCustStay2` | — | Same with extra grouping by room type | (variant) | Preview | A4 portrait | Reports / Guests |
| `CrystalReportCleanRoom.rpt` (embedded) | `CrystalReportCleanRoom` | — | Housekeeper / cleaning report — who cleaned what room | `ReportCleanRoom` form, `FrmReportHousewife` | Preview | A4 portrait | Reports / Housekeeping |
| `CrystalReport_Room_ALL.rpt` (embedded) | `CrystalReport_Room_ALL` | — | Composite "all rooms" status report (sales+occupancy combined) | `FormReportAll.cs:1260` | Preview | A4 landscape | Reports / All |
| `CrystalReport_Room_ALL2.rpt` (embedded) | `CrystalReport_Room_ALL2` | — | Variant of the above (extra columns from `HT_CheckIn_Ds`) | `FormReportAll2.cs:2108` | Preview | A4 landscape | Reports / All |

> **Counts:** ~46 distinct logical reports (about **30 stand-alone `.rpt` files at root** + a further set embedded as resources in the assembly under `_decompiled\iHOTEL2025.*.rpt`). Several `.rpt` files are file-system copies of each other (`sale_vat - Copy.rpt`, `ReportFolio2 - Copy.rpt`, `report\ReportCupon80.rpt`, `reports\ReportCupon80.rpt`) — those are **dead duplicates**, not used at runtime.
>
> **Loading pattern (see `Print_Report.cs`):** Every print call first tries `ReportDocument.Load(Path_Program + "reports/<name>.rpt")`, then `<name>.rpt` next to the EXE, and only if both fail does it `new <wrapper-class>()` — i.e. the **`.rpt` files at root are user-overridable templates**, and the compiled `Cached*`/wrapper classes are the safe fallback.

---

## 2. By module (grouped view)

### Booking (3 reports)
- **`ReportBooking.rpt`** — guest-facing booking confirmation; printed when an advance reservation is created in `FrmAddBook`/`FrmAddBook2`. Includes booked rooms (date in/out, nights, room-type rate) and pre-ordered products from `HT_Book_Pro`. Calls `print_booking(book_id)`.
- **`ReportBookingINV.rpt`** — same booking, but as an **invoice** (with `HT_INVOICE` reference and tax block). Issued from `FormBookingInvoice` / `FrmReportBook2`.
- *(Booking searches `FrmReportBook` / `FrmBookMain` use the above two; no dedicated "all bookings" CR — it's an in-grid DataGridView.)*

### Check-in / Registration (4 reports)
- **`ReportReg_1.rpt`** / **`ReportReg_2.rpt`** / **`ReportReg_3.rpt`** — guest **registration** sheet, 1-up / 2-up / 3-up on A4 (chosen by `Module1.Reg_Print` setting). Printed at check-in by `FrmCheckIn`/`FrmRegMain`. Contains guest name, ID, address, room, dates, signature line.
- **`ReportReg2.rpt`** — newer "Reg2" sheet with extra columns (used by hotels that need a different format; selectable from settings).

### Check-out / Folio (3 reports)
- **`ReportFolio1.rpt`** + **`ReportFolio1_2.rpt`** — itemised guest **folio statement** (room charges, products, deposits, taxes). Page 1 (`_1`) holds the first 19 rows, anything more spills onto continuation page 2 (`_2`).
- **`ReportFolio2.rpt`** — alternative landscape folio layout for multi-room/family bookings (groups by room).

### Sales / POS — receipts (4 form factors × 2-3 variants)
- **`sale.rpt`** — continuous-form / dot-matrix receipt (default).
- **`sale2_58.rpt`** — 58 mm thermal printer slip.
- **`sale2_80.rpt`** — 80 mm thermal printer slip.
- **`sale3_folio.rpt`** — A4 folio-style receipt (printed when `Receipt_Report = "HHOTEL"` or `"FOLIO"` or `"Guest Folio"`).
- **`sale_credit.rpt`** / **`sale_cin_credit.rpt`** — receipts for credit (deferred-payment) sales; the `_cin_` variant is for credit added against an active check-in.

### Sales / POS — VAT receipts (6 layouts)
- **`sale_vat.rpt`** + **`sale_vat_copy.rpt`** — VAT receipt + carbonless customer/merchant copy.
- **`sale_vat0.rpt`** + **`sale_vat0_copy.rpt`** — VAT-zero variant + copy. Used when the receipt is "VAT inclusive" (price already contains VAT — break-out only).
- **`sale_vat0SB.rpt`** — Short-Bill (compact thermal-style on A4).
- **`sale_vat0CB.rpt`** — Copy-Bill (merchant copy of the SB).

### Invoices / VAT — separate document (7 layouts)
- **`inv_sale_vat.rpt`**, **`inv_sale_vat0.rpt`** — A4 VAT invoice and VAT-zero invoice from `FrmReceiptInvoice`.
- **`inv_sale_other_vat.rpt`**, **`inv_sale_other_novat.rpt`** — invoice for "other charges" income (not tied to a room).
- **`invoice_room.rpt`** — legacy room-only invoice template, kept as fallback.
- **`inv_sale_vat0_debt.rpt`**, **`inv_sale_vat0_debt_hhotel.rpt`**, **`inv_sale_vat0_debt_hhotel_no.rpt`**, **`inv_sale_vat0_debt_hhotel_other.rpt`** — debt-receipt / accounts-receivable invoice family. Different layouts for plain vs. hotel-branded vs. hotel-branded-without-VAT vs. hotel-branded-other-charges.

### Deposit (3 reports — same content, three paper sizes)
- **`ReportDep.rpt`** (A4 / continuous), **`ReportDep2_58.rpt`** (58 mm thermal), **`ReportDep2_80.rpt`** (80 mm thermal) — receipt for deposit received (รับเงินมัดจำ) or refunded (คืนเงินมัดจำ). Selected by `Module1.Deposit_Report` setting.

### Coupon (2 reports)
- **`ReportCupon58.rpt`** / **`ReportCupon80.rpt`** — food / breakfast coupons issued at check-in or via `FrmCuponMain`. Two thermal-paper variants only (no A4 version).

### Daily reports (occupancy / housekeeping)
- **`CrystalReportDays.rpt`** — daily occupancy + sales summary.
- **`CrystalReportDaysContenue.rpt`**, **`CrystalReportDaysContenue2.rpt`** — guests still in-house ("continued rooms").
- **`CrystalReportCustIn.rpt`** — current in-house list.
- **`CrystalReportCustOut.rpt`** — checked-out history.
- **`CrystalReportCustOutToday.rpt`** — guests scheduled to leave today.
- **`CrystalReportCustOutTodayHousewife.rpt`** — same list, formatted for the housekeeping team (focus: room number + dirty/clean state).
- **`CrystalReportCustChange.rpt`** — room-change audit log.
- **`CrystalReportCustStay.rpt`**, **`CrystalReportCustStay2.rpt`** — total-nights-per-guest report (loyalty / VIP detection).
- **`CrystalReportCleanRoom.rpt`** — housekeeping log (who cleaned what / when).

### Period reports (income, sales, shift)
- **`ReportIncome.rpt`** — income by date range (cash in/out, by account category).
- **`ReportIncome2.rpt`** — income by **round bill** (cashier shift).
- **`ReportShipCash.rpt`** + **`ReportShipCashOLD.rpt`** — shift-close cash count + sales summary (current + legacy layouts).
- **`ReportSaleVat.rpt`** — VAT-eligible sales summary for tax filing.

### Composite "all-in-one" reports
- **`CrystalReport_Room_ALL.rpt`** / **`CrystalReport_Room_ALL2.rpt`** — landscape dashboard with rooms × sales × occupancy across one period. Used by `FormReportAll` / `FormReportAll2` (the "รายงานทั้งหมด" big-picture sheets).

### Government / Compliance
- **`Report_RR4.rpt`** — Thai government **RR.4** form (foreign-guest registry, sent to district office). Strict legal layout.

### Customer / Image
- **`ReportPictures.rpt`** — single-page photo print (selects `Tb_Save_Image.pic` blob and renders it).

---

## 3. Per-report deep-dive

> Reports are clustered by family. For each cluster I list the dataset table fields the report binds (read from `Datalocal.cs` and from `Print_Report.cs` `Add*Row(...)` argument lists), the SQL queries that supply the data, and the rewrite difficulty.

### 3.1 Sales-receipt family — `sale*` (`Datalocal.ReportBillCash`)
**Variants:** `sale.rpt`, `sale2_58.rpt`, `sale2_80.rpt`, `sale3_folio.rpt`. Same dataset, different paper sizes / layouts.

**Dataset columns** (deduced from `Print_Report.AddReportBillCashRow(...)` calls, Print_Report.cs:93-118): page-no, date `dd/MM/yyyy`, pay-no, pay-note, customer name, customer address (with prefix-strip Replace cleanup of `หมู่`/`ซอย`/`ถนน`/`เขต/อำเภอ`/`แขวง/ตำบล`/`จังหวัด`), line-no, item description (price, qty), room-or-product label, line subtotal `#,##0.00`, line total, payment paid (cash + credit + transfer), change, grand total, "ผู้ออกบิล" (cashier name), "ผู้ส่ง" (sender), Thai-text amount via `DecimalToText_TH.ThaiBahtText(...)`, plus blanks for sub-report alignment.

**Source data:** `select * from view_pay_ds where pay_no='<id>'` (a database **view** that joins `HT_CheckIn_Pay`, `HT_CheckIn_Pay_Ds`, `HT_Customers`, `HT_CheckIn_H`, etc.) plus `select * from TB_SETTINGS` for the company header (logo blob from `MyProject.Forms.login.ReflectionImage1.Image`).

**Field count / complexity:** ~22 columns. **Small slip** (1 page, ≤9 lines for thermal / ≤100 lines for continuous). No subreports.

**Localization:** Thai labels hard-coded in the `.rpt` design (e.g. `ผู้ออกบิล`); date format `dd/MM/yyyy`; numeric `#,##0.00`. Logo embedded as PNG blob in `Bill_H.Copany_Logo`.

**Replacement difficulty:** **Trivial**. ~80 % of reports in the system follow this pattern.

---

### 3.2 Credit-sales family — `sale_credit.rpt`, `sale_cin_credit.rpt`
**Dataset:** `Datalocal.ReportBillCredit` (similar to ReportBillCash but with extra "due date" / "remaining" columns).
**Source:** `HT_Bill_Debt_H` + `HT_Bill_Debt_Ds` (credit master + lines), or for the `_cin_` variant `HT_CheckIn_*` joined with the credit ledger.
**Replacement difficulty:** **Trivial** (same as Sales family; one extra `due_date`/`balance` column).

---

### 3.3 VAT-receipt family — `sale_vat*`, `inv_sale_vat*`
**Variants:** `sale_vat.rpt`, `sale_vat_copy.rpt`, `sale_vat0.rpt`, `sale_vat0_copy.rpt`, `sale_vat0SB.rpt`, `sale_vat0CB.rpt`, `inv_sale_vat.rpt`, `inv_sale_vat0.rpt`.

The differences:
- **`*_copy`** = duplicate slip with "สำเนา" (= "copy") marker — printed for merchant carbon copy.
- **`*0`** = VAT-inclusive layout: VAT is **broken out from** the displayed price, not added to it.
- **`*SB`** = "Short Bill" — compact thermal layout (single-column, no address).
- **`*CB`** = "Copy Bill" of the SB.
- **`inv_*`** = standalone tax invoice (separate document number, header reads "ใบกำกับภาษี" instead of "ใบเสร็จ").

**Dataset columns:** Same as Sales family **plus** `vat_per`, `vat_amount`, `before_vat`, `after_vat`, `tax_id`, `inv_no`. Tax % from `TB_SETTINGS.vat_per`.

**Source data:** Same `view_pay_ds` plus the tax %; the `inv_*` variants additionally read `HT_INVOICE` and `HT_Invoice_Ds`.

**Replacement difficulty:** **Trivial-medium**. The math is `before = total × 100 / (100 + vat_per)`; `vat = total - before` (already done in C# code, the report just renders the values).

---

### 3.4 Folio family — `ReportFolio1.rpt` + `_2`, `ReportFolio2.rpt`
**Datasets:** `ReportFolio1` / `ReportFolio1_2` / `ReportFolio2` (defined in `Datalocal.cs`).

**`ReportFolio1`/_2 columns** (from `print_report.cs:2243`, `2218-2230`): customer-name, blank slot×9, page-total `#,##0.00`, blank, page-no, sub-total, grand-total, Thai-text amount (16 fields).

**`ReportFolio2` columns** (from `print_report.cs:2328`): `CIN_NAME1/2/3`, line-no, room, item-name, in-date, out-date, nights, price, line-total, running-total, `CIN_NO`, `vat_per`, before-VAT, VAT, Thai-text (17 fields).

**Source data:**
- `select * from TB_SETTINGS` (company header)
- `select * from TB_FOLIO where no='<CIN_NO>' order by id` — the folio is a denormalised "ledger" table with one row per chargeable line (room-night, product, deposit credit, etc.). Special row when `F_IN=''` triggers a description merge into the previous line (i.e. a continuation note).

**Field count / complexity:** **Medium** (1-3 pages, hard-coded 19-row first page → spill to `_2`).

**Subreports:** none, but `_1` + `_2` is effectively a 2-band manual paginator.

**Localization:** Thai labels in design; Thai-Baht-text via `DecimalToText_TH.ThaiBahtText`.

**Replacement difficulty:** **Medium**. The 19-row page break is hardcoded in C#; in QuestPDF this becomes a single auto-paginated table.

---

### 3.5 Booking family — `ReportBooking.rpt`, `ReportBookingINV.rpt`
**Datasets:** `Datalocal.ReportBooking` / `ReportBookingINV`.

**`ReportBooking` columns** (from `print_report.cs:2381`, 16 fields): `book_no`, customer-name (concat), Book_Room_Start/End `dd/MM/yyyy`, nights, room-type, price `#,##0.00`, qty, nights, line-total, room-note, booked-by, booking-date, start, end, grand-total. Then a second loop adds add-on products (`HT_Book_Pro` rows) re-using the same row schema.

**Source data:** 4 SQL queries:
1. `TB_SETTINGS` (header).
2. `HT_Book_H where book_id='<id>'` (booking master).
3. `HT_Book_Ds` JOIN `HT_Rooms` GROUP BY (booked rooms aggregated).
4. `HT_Book_Pro where B_NO=… GROUP BY B_NAME,B_PRICE` (booking products).

**`ReportBookingINV`** = the same plus `select * from HT_INVOICE where INV_booking_no='<id>'` — adds the invoice number/date/tax-id.

**Field count / complexity:** **Medium** (1-2 pages, two row-types interleaved).

**Replacement difficulty:** **Medium** (table + secondary table + Thai-text totals).

---

### 3.6 Registration family — `ReportReg_1.rpt`, `_2.rpt`, `_3.rpt`, `ReportReg2.rpt`
**Datasets:** `Datalocal.ReportReg_1` (shared by 1/2/3-up variants — same data, different layout). `ReportReg2` has its own typed dataset.

**Reg_1 columns** (from `print_report.cs:1462`-1530, ~25 fields): guest title, name, last-name, ID-card-no, nationality, birth-date, address, occupation, came-from, going-to, arrival-date, departure-date, purpose-of-stay, room-no, signature-image (`Tb_Save_Image` blob), photo (`Tb_Save_Image` blob), accompanying-persons-list, vehicle-plate, contact-tel, registered-by, register-date, etc.

**Source data:** `HT_CheckIn_H` + `HT_CheckIn_Other_People` + `HT_Customers` + `Tb_Save_Image` + `TB_SETTINGS`.

**Field count / complexity:** **Medium** (1 page, lots of fields with image blobs). 1-up vs. 2-up vs. 3-up = different visual layouts of the same data, intended to fit 1, 2, or 3 guests on a single A4 page.

**Replacement difficulty:** **Medium**. Image rendering + signature placement matters.

---

### 3.7 Deposit family — `ReportDep.rpt`, `ReportDep2_58.rpt`, `ReportDep2_80.rpt`
**Dataset:** `Datalocal.ReportDep` shared by all three.
**Source:** `HT_Deposit` + `HT_Book_H` + `TB_SETTINGS`.
**Field count:** Small (~10 fields: deposit-no, customer, booking-no, amount, date, received-by, Thai-text amount, paid/refunded marker).
**Replacement difficulty:** **Trivial**.

---

### 3.8 Coupon family — `ReportCupon58.rpt`, `ReportCupon80.rpt`
**Dataset:** `Datalocal.ReportCupon`.
**Source:** `HT_Cupon` + `TB_SETTINGS`.
**Fields:** ~6 (coupon-no, room, guest-name, meal type, valid-date, qty).
**Localization:** **Barcode!** Coupon serial is rendered as a Code 39 barcode using **`3OF9.ttf`** (present in the project root, file size ~36 KB). The .rpt likely uses this font on a text field.
**Replacement difficulty:** **Trivial** (use ZXing.NET or QuestPDF.Barcode for the serial; ship the font).

---

### 3.9 Government — `Report_RR4.rpt`
**Dataset:** `Datalocal.ReportRR4`.
**Source:** Built by `FrmReportRR4.cs:781` from `HT_CheckIn_H` + `HT_CheckIn_Other_People` + `HT_Customers` + `TB_SETTINGS`.
**Fields:** Strict legal-form column set: name, nationality, passport-no, arrival-date, room, etc.
**Replacement difficulty:** **Medium-High**. The layout must match the official Thai government RR.4 paper template **exactly** (column widths, header art, official seal placement). This is the one report where pixel-fidelity matters for compliance.

---

### 3.10 Daily reports — `CrystalReportDays`, `CrystalReportDaysContenue(2)`, `CrystalReportCustIn/Out/OutToday/Change/Stay/Stay2/CleanRoom/CustOutTodayHousewife`
These do **not** bind a typed dataset. The launcher form (`ReportDays.cs`, `ReportCustIn.cs`, etc.) opens a CrystalReportViewer that uses the report's own embedded SQL connection (Crystal's old "Database Expert" — connection-string baked into the .rpt).
**Source:** Direct queries on `HT_CheckIn_H`, `HT_CheckIn_Ds`, `HT_Room_Status`, `HT_Rooms`, `HT_Housewife`, `HT_Customers` filtered by the form's date pickers.
**Fields:** 5-12 columns each (single-table reports).
**Replacement difficulty:** **Trivial-medium**. Replace the embedded SQL with a parametric C# query and feed the rows.

---

### 3.11 Income / shift / VAT-summary period reports
- **`ReportIncome.rpt`** / **`ReportIncome2.rpt`** — bind `Datalocal.ReportSale` (~16 fields: line-no, date, bill-no, in-amount, out-amount, balance, header-title, totals, account-group, account). Source: `tb_pay_history` (cash in/out ledger) joined with `TB_SET_MyType2/2_2/3` (account categories). Field count: 16 columns × N rows; running-totals computed in C# before `AddReportSaleRow`.
- **`ReportShipCash.rpt`** — typed `ReportShipCash` dataset; reads `HT_CheckIn_Ds`, `HT_CheckIn_Pay`, `HT_Rooms_Cancel`, `tb_pay_history` for a shift window. Includes cash drawer count by denomination.
- **`ReportSaleVat.rpt`** — `Datalocal.ReportSaleVat`; reads VAT receipts in date range. Columns: receipt-no, date, customer, before-VAT, VAT, after-VAT.
**Replacement difficulty:** **Medium** (multi-grouping, totals, possibly landscape).

---

### 3.12 Composite — `CrystalReport_Room_ALL`, `_ALL2`
The biggest reports. They cross-join `HT_Bill_Debt_H`, `HT_Book_Ds`, `HT_CheckIn_H`, `HT_CheckIn_Product`, `HT_Products`, `HT_Rooms_Cancel`, plus `HT_CheckIn_Ds` for `_ALL2`. Multi-section bands (per-room-type → per-room → per-day grouping). Used as the all-in-one management dashboard.
**Replacement difficulty:** **High**. Multi-level grouping + many columns + landscape.

---

### 3.13 Picture print — `ReportPictures.rpt`
**Dataset:** `Datalocal.ReportPic` (1 column: `pic` blob).
**Source:** `select * from Tb_Save_Image where id=<id>`.
**Replacement difficulty:** **Trivial** (single page, single image; doesn't need a "report" engine at all — print-image API is enough).

---

## 4. Recommended modern replacement strategy

### Three-bucket triage

**Bucket A — Trivial to port (≈25 reports):** Single-page slips with simple repeating tables. Direct fit for **QuestPDF** (or HTML+CSS print).
- All `sale*.rpt` / `inv_sale_*.rpt` / `sale_vat*.rpt` / `sale_credit*.rpt` (sales/credit/VAT receipt families)
- `ReportDep*` (deposit slips)
- `ReportCupon*` (coupons; ship `3OF9.ttf` and use Code 39 string)
- `ReportPictures` (single image — this could just be `PrintDocument` directly)
- All `CrystalReportCust*` daily/guest list reports
- `CrystalReportCleanRoom`, `CrystalReportDays`, `CrystalReportDaysContenue*`

**Bucket B — Medium effort (≈10 reports):** Multi-section, computed totals, or 2-up/3-up layouts that need careful template work.
- `ReportFolio1` + `_2` (manual page break logic)
- `ReportFolio2` (landscape grouping by room)
- `ReportBooking` / `ReportBookingINV` (two interleaved row types + totals)
- `ReportReg_1` / `_2` / `_3` / `ReportReg2` (multi-up registration with image blobs)
- `ReportIncome` / `ReportIncome2` (running totals + grouping)
- `ReportShipCash` / `ReportShipCashOLD` (shift summary + cash count)
- `ReportSaleVat` (VAT period summary)

**Bucket C — Needs design decision (≈5 reports):**
- `Report_RR4` — must match the official Thai government RR.4 form layout. *Recommendation:* model the existing template in a one-time pixel-perfect QuestPDF Composer, get it signed off by the hotel's legal advisor before retiring the Crystal version.
- `CrystalReport_Room_ALL` / `_ALL2` — multi-level dashboard; consider whether the new app even needs this monolithic report, or whether it should become a **Power BI-style live UI page** instead of a printed report.
- `inv_sale_vat0_debt_hhotel*` family — four nearly-identical layouts; consolidate into **one parametric template** in the rewrite (saves four files of maintenance).

### Recommended technology

> **Primary recommendation: QuestPDF (MIT, .NET-native).**
>
> Rationale: every report in this app is a simple data-bound page; the codebase already builds row-by-row in C# via `Datalocal.AddXxxRow(...)`. That data shape is exactly what QuestPDF wants — feed in the same DataRow loop, output PDF, print via `System.Drawing.Printing.PrintDocument` or system print dialog. Removes the **CRRedist2008_x86.msi** Crystal Reports runtime dependency entirely (currently required on every workstation), removes the Crystal licence-attribution risk, and removes the .NET Framework lock (QuestPDF works on .NET 6/7/8). Add **ZXing.Net** for barcodes (replacing the `3OF9.ttf` workaround).

**Alternatives considered:**
- **RDLC (Microsoft Local Reports)** — would let you keep designer-style WYSIWYG editing, but locks you back into `Microsoft.ReportViewer.WinForms` (de-facto deprecated, WinForms-only). Don't recommend.
- **HTML+CSS → PDF (Puppeteer / wkhtmltopdf)** — fine for one-page slips but flaky for receipt-printer raw mode and Thai font kerning. OK as a fallback.
- **Crystal Reports for VS** — still updated by SAP; but keeps the runtime dependency and licensing question.

### Cross-cutting concerns to handle in the rewrite

1. **Thai numeric-to-text** — port `DecimalToText_TH.ThaiBahtText(double)` from `_decompiled_clean\iHOTEL2025\DecimalToText_TH.cs` verbatim; it's used by ~15 reports.
2. **Logo handling** — currently `MyProject.Forms.login.ReflectionImage1.Image` is round-tripped through a temp `logo.bmp` file. In the new system, store the company logo once in `TB_SETTINGS.Company_Logo` blob and load directly.
3. **Address-prefix stripping** — `Print_Report.Print_Sale` strips `หมู่ ` / `ซอย ` / `ถนน ` etc. from `C_Address` before printing because the receipt is too narrow to fit them. Do this in the projection query (or in a `CustomerAddressFormatter` helper) instead of in print code.
4. **Receipt-style routing** — the user's selected printer style is held in `Module1.Receipt_Report`, `POS_Report`, `Cupon_Report`, `Deposit_Report`, etc. (string compare against Thai labels — fragile). Replace with an enum `PrinterStyle { ContinuousPaper, Thermal58, Thermal80, A4Folio, HHotel, ... }`.
5. **3OF9 barcode font** — `3OF9.ttf` is in the project root and used by `ReportCupon*`. Replace with **ZXing.Net** Code 39 image rendering (no font install needed on workstations).
6. **Override-template loading** — currently `Print_Report` first tries `reports/<name>.rpt` next to the EXE so the customer can hand-edit the layout. Provide an equivalent **JSON/YAML template override** mechanism (or just allow the user to edit a Razor / Scriban template).
7. **`Module1.localdata` is global mutable state** — every report clears its table at the start (`Module1.localdata.ReportXxx.Rows.Clear()`) and adds rows, then prints. In the rewrite, build a fresh DTO per print job; don't keep a global cache.
8. **Eliminate `CRRedist2008_x86.msi`** from the installer once all reports are ported — saves ~120 MB and a per-machine install step.

---

## 5. Replacement priority order

### Phase 1 — Daily-operation blockers (must port first)

These are printed every single check-in / check-out / sale and block hotel operations if broken:

1. **`sale.rpt` + `sale2_58.rpt` + `sale2_80.rpt`** — sales receipt (every POS transaction).
2. **`ReportReg_1.rpt`** (and 2/3-up variants) — guest registration form (every check-in, legally required).
3. **`ReportFolio1.rpt` + `ReportFolio1_2.rpt`** — guest folio (every check-out).
4. **`sale_vat.rpt` + `sale_vat0.rpt`** — tax receipt (every VAT-eligible sale).
5. **`ReportDep.rpt` + `ReportDep2_58/80.rpt`** — deposit receipt (every booking with a deposit).

### Phase 2 — Important but not minute-by-minute

6. `ReportBooking.rpt` / `ReportBookingINV.rpt` — booking confirmation.
7. `inv_sale_vat.rpt` / `inv_sale_vat0.rpt` / `invoice_room.rpt` / `inv_sale_other_*.rpt` — standalone invoices (issued on demand).
8. `ReportCupon58.rpt` / `ReportCupon80.rpt` — meal coupons (issued at check-in).
9. `sale_credit.rpt` / `sale_cin_credit.rpt` — credit sales.
10. `sale3_folio.rpt` — folio-style A4 receipt.

### Phase 3 — Reports (back-office)

11. `CrystalReportDays.rpt`, `CrystalReportCustIn/Out/OutToday/Change/Stay.rpt` — daily occupancy reports.
12. `CrystalReportCleanRoom.rpt`, `CrystalReportCustOutTodayHousewife.rpt` — housekeeping.
13. `ReportIncome.rpt` / `ReportIncome2.rpt` — period income.
14. `ReportShipCash.rpt` — shift close.
15. `ReportSaleVat.rpt` — VAT-period summary.

### Phase 4 — Compliance + composite (do carefully)

16. **`Report_RR4.rpt`** — government registry form (must be pixel-perfect, requires sign-off).
17. `CrystalReport_Room_ALL.rpt` / `_ALL2.rpt` — composite dashboard (consider replacing with a live UI screen).

### Phase 5 — Optional / dead

18. `ReportPictures.rpt` — could be replaced with direct image-print, no report engine.
19. `ReportShipCashOLD.rpt`, `sale_vat - Copy.rpt`, `ReportFolio2 - Copy.rpt`, `report\ReportCupon80.rpt`, `reports\ReportCupon80.rpt` — **DEAD duplicates**, do not port.
20. `*_copy.rpt` (`sale_vat_copy.rpt`, `sale_vat0_copy.rpt`, `sale_vat0CB.rpt`) — collapse into the main report with a "Copy / สำเนา" parameter; one template can render both originals and copies.
