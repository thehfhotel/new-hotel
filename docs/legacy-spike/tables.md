# Legacy DB Table Catalog (`db` on <legacy-host>)

60+ tables in `dbo` schema. Names prefix conventions:
- `HT_*` — hotel domain (the bulk)
- `TB_*` / `Tb_*` — system / setup
- `View_*` — read-only views, mostly simple JOINs over base tables

Columns marked **observed** were seen written to in our 2026-04-24 capture.

## Operational tables (the writeback surface)

| Table | Rows | Purpose | Observed in writes |
|---|---|---|---|
| `HT_Customers` | 21,609 | Customer master. PK `id` (identity int). App-key `Cust_no` (varchar). Long address fields (`Cust_Add_*`, `Cust_Work_*`). | walk-in, booking, modify, check-in-to-booking, check-out (UPDATE only) |
| `HT_Book_H` | 14,809 | Booking header. PK `Book_ID` (varchar = `R\d{6}`). | booking, modify, check-in-to-booking |
| `HT_Book_Ds` | 15,949 | Booking detail (one row per room in a booking). FK `Book_No` → `HT_Book_H.Book_ID`. ⚠️ `Book_Room_Type` actually stores room **number**. | booking, modify |
| `HT_Book_Date` | 1,011 | One row per booked night. FK `Book_no`. `Book_USE=0` for booking, `=1` after check-in. | booking, modify |
| `HT_CheckIn_H` | 19,335 | Check-in header. PK `Cin_no` (varchar = `CH\d{2}-\d{6}`). FK `Cin_Book_no` → `HT_Book_H.Book_ID` (NULL for walk-ins). Holds totals. | walk-in, check-in-to-booking, extend, check-out, payment |
| `HT_CheckIn_Ds` | 24,867 | Check-in detail (one row per room in a check-in). PK `id`. FK `Cin_No`. | walk-in, check-in-to-booking, extend, check-out |
| `HT_CheckIn_Other_People` | 19,341 | Accompanying guests for TM.30. FK `Cin_no`. | walk-in, check-in-to-booking |
| `HT_CheckIn_Pay` | 27,090 | Payments on check-ins. FK `Cin_No`. | invoice/payment |
| `HT_CheckIn_Product` | 0 | Per-check-in products (minibar, etc). Empty in our DB. **Unused?** | not yet observed |
| `HT_Receipt_H` | 20,653 | Receipt header. PK `id`. | invoice |
| `HT_Receipt_Ds` | 26,063 | Receipt line items. FK `S_Sale_id` → `HT_Receipt_H.id`. | invoice |
| `HT_Rooms` | 58 | Room master. PK `id` (identity int). App-key `room_no` (varchar). Status fields: `room_use`, `Room_Clean`, `Room_Use_Count`, `room_book_*`. | walk-in, check-in-to-booking, extend, check-out |
| `HT_Room_Status` | 4,449 | One row per (room, date). **Current state, not history.** Overwritten on new check-in for same room+date. PK `id` (app-allocated). | walk-in, check-in-to-booking, extend, check-out |
| `HT_POWER_LOG` | 29,755 | IoT relay log (lights on/off per occupancy). One INSERT on check-in (start), one UPDATE on check-out (end). Filter `WHERE ROOM_POWER_END_BY=''` for in-progress. | walk-in, check-in-to-booking, check-out |
| `HT_Housewife` | 30,210 | Housekeeping work items. App-INSERTed when room marked dirty. | check-out (after) |
| `HT_Cupon` | 17,894 | Pre-allocated coupons; `cupon_print` flag flipped on check-in. Purpose unclear. | walk-in, check-in-to-booking |
| `Tb_Save_Image` | 9,688 | ID card / customer photo storage. Pre-staged with `tmp_no`, claimed by `cin_no`/`cust_no` on check-in completion. | walk-in (no-op if no photo), check-in-to-booking |

## Tables observed in walk-in but with unclear purpose

| Table | Rows | Notes |
|---|---|---|
| `HT_Changed_Room` | 3,866 | Room change history. |
| `HT_Round_Bill` | 4,575 | End-of-day billing rounds. |
| `HT_Rooms_Cancel` | 298 | **Cancel-check-in audit log.** Captured: INSERT on cancel-check-in. `id` is app-allocated MAX+1. Records `(room_no, cin_no, cancel_date, cancel_by, cancel_note)`. Distinct from booking cancel which only flips status. |
| `HT_Rooms_Repair` | 683 | Maintenance / out-of-service log. |
| `HT_Rooms_Price` | 32 | Pricing rules. |
| `HT_Log` | 6,224 | Generic activity log. |
| `HT_Log_Debt` | 686 | Debt activity log. |
| `HT_Room_SMS` | 2,379 | SMS history per room. |
| `HT_EMP_SMS` | 78 | SMS history per employee. |

## Configuration / setup tables

| Table | Rows | Purpose |
|---|---|---|
| `TB_SETTINGS` | 1 | Global settings (single-row config). |
| `TB_SET_Branch` | 1 | Branch identity / hotel info. |
| `TB_MRP_EMPLOYEE` | 4 | Employee accounts. |
| `TB_MRP_Permission` | 24 | Permission rules. |
| `TB_MRP_Permission_name` | 1 | Permission labels. |
| `HT_SET_CusType` | 4 | Customer rate categories. |
| `HT_SET_CusType_Main` | 4 | Customer main types (e.g. `'บุคคลธรรมดา'` = individual). |
| `HT_SET_ProductType` | 6 | Product categories. |
| `HT_SET_RoomType` | 8 | Room types. |
| `HT_SET_Sale` | 1 | Sales channel? |
| `HT_Products` | 5 | Product master (used in HT_Receipt_Ds via `S_Product_no`). |
| `HT_Products_Price` | 20 | Product pricing. |
| `Tb_Version` | 1 | Schema version marker. |
| `TB_FOLIO` | 6 | Folio? |
| `TB_Pay_History` | 3 | Payment history. |
| `TB_SET_MyType2` / `TB_SET_MyType2_2` / `TB_SET_MyType3` | 5 / 20 / 40 | Unknown — typed sub-records of some kind. |

## Empty tables (likely unused features)

| Table | Notes |
|---|---|
| `HT_Bank_Accounts` / `HT_Bank_Transfer` | Bank integration — not in use. |
| `HT_Bill_Debt_Ds` / `HT_Bill_Debt_H` | Debt billing — not in use (but views reference them). |
| `HT_Book_Ds2` / `HT_Book_H2` | Alternate booking format — not in use. |
| `HT_Book_Pro` | Promotion-related. Not in use but DELETE statements still reference it. |
| `HT_Book_Status` | Booking statuses lookup — empty. Statuses are stored as inline strings instead. |
| `HT_Booking_Notes` | Per-booking notes — not in use. |
| `HT_ContinueTime` | Hourly extension. |
| `HT_Deposit` | Deposit tracking. |
| `HT_Invoice_Note` | Per-invoice notes. |
| `HT_Order_Down` / `HT_Order_Up` | Unknown. |
| `HT_Register` | Unknown. |

## Views (definitions in `schema/01-baseline-schema.txt`)

Most views are simple SELECTs over the base `HT_*` tables. **Updatable
views are rare** — `View_Customers` has computed concatenation, `View_Pay_H`
has GROUP BY, several have CASE/SELECT subqueries. **Always write to the
underlying base tables**, never to views.

Notable views:
- `View_Booking_Ds` — booking detail joined with header info
- `View_CheckIn_Ds` / `View_CheckIn_H` / `View_Check_Out` — check-in views
- `View_Customers` — customer with concatenated name/address
- `View_Bill_Cancel`, `View_Bill_Debt_Ds` — billing views
- `View_Pay_Ds` / `View_Pay_H` — payment views (H is grouped)
- `View_RBill_H` / `View_RBill_H_Round_Only` — daily rounding views
- `View_Report_RR4` — government reporting view (likely TM.30 export)

No triggers, no stored procedures, no functions.
