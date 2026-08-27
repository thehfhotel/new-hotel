# Writeback recipe audit — 2026-05-12

Multi-agent audit of every writeback recipe + shared foundation against
`docs/legacy-spike/findings.md` §3a–k, `docs/legacy-app/COMPAT_CHEATSHEET.md`,
`docs/legacy-app/SCHEMA.sql`, `docs/legacy-app/FEATURE_MAP.md`.

**Scope:** `hotel-backend/src/writeback/**` (11 recipes + dispatcher, allocate,
fingerprint, format, constants, helpers, error, mod) and `bin/writeback.rs`
worker entry.

**Out of scope:** TABLOCKX+HOLDLOCK locking primitive (validated by spike
2026-04-24); recipe-vs-PG-canonical reconciliation (separate audit pass).

**Totals:** 3 CRIT • 15 HIGH • ~22 MED • ~25 LOW.

Existing byte-parity tests passed because they bypass allocators with
hardcoded IDs — the worst bugs (Pay_No / Receipt_no) are invisible to the
current test surface.

---

## CRIT — block any live writeback until fixed

### C1 — `allocate_pay_no` prefix and width wrong
**File:** `hotel-backend/src/writeback/allocate.rs:188-200`

Emits `P{yyMM}-{6digit}` (e.g. `P2604-000001`). Legacy convention per
`docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns" "R{yyMM}-{4digit}"
(was: a line-number citation into section 2 of `findings.md` — wrong document; that
format is documented nowhere in `findings.md` at any revision), primary source
`Module1.GetSIR_PAY` (decompile `Module1.cs:1756`), and live capture
`docs/legacy-spike/raw/invoice-20260424-100827/07-events.txt:154` is
`R{yyMM}-{4digit}` (e.g. `R2604-0241`).

Two failures:
- `WHERE Pay_No LIKE 'P2604-%'` never matches iHOTEL rows → parallel
  namespaces, MAX+1 invariant broken.
- 6-digit suffix vs 4-digit → receipt reports + string-sort downstream
  mis-align.

**Fix:** `format!("R{:02}{:02}-", year%100, month)` and `{next:04}`.
Add a unit test asserting `allocate_pay_no` emits `^R\d{4}-\d{4}$`.

### C2 — `allocate_receipt_no` prefix and width wrong
**File:** `hotel-backend/src/writeback/allocate.rs:206-218`

Emits `RC{yyMM}-{6digit}`. Legacy is `B{yyMM}-{4digit}` (with `SB`/`CB`
variants for SmallBill/CreditBill — single `B` is the default).
References: `docs/legacy-app/COMPAT_CHEATSHEET.md` §"1.6 ID generation patterns" "B{yyMM}-{4digit}"
(was: one line number pointed at two documents at once — the same number read as
a `findings.md` section-2 line AND as a cheatsheet line; only the cheatsheet
reading resolved), primary source
`FrmAddSale.GetSIR` (decompile `FrmAddSale.cs:3818`), live capture
`docs/legacy-spike/raw/walkin-20260424-095304/07-events.txt:120`.

**Fix:** `format!("B{:02}{:02}-", year%100, month)` and `{next:04}`.
Derive `SUBSTRING` offset from `prefix.len()+1` rather than hardcoded `8`.

### C3 — Allocator year/month uses UTC, not Bangkok
**Files:** `allocate.rs:108-122` (`allocate_cin_no`), `allocate.rs:189-190`
(`allocate_pay_no`), `allocate.rs:207-208` (`allocate_receipt_no`).

`Utc::now().year() % 100` and `.month()` straddle the BKK calendar boundary
by 7h. From 00:00–07:00 BKK on Jan 1 (or any month rollover) we emit the
previous period's prefix while iHOTEL has rolled forward — sequences fork.

**Fix:** `Utc::now().with_timezone(&Bangkok).year() % 100` and `.month()`.
`format::bangkok_*` helpers already exist for this.

---

## HIGH — correctness bugs under known scenarios

### Financial integrity

**H1 — `checkout.rs:132-148` hardcodes nights=1 and zeros all revenue.**
`execute()` passes `nights=1.0`, `room_price_total=0`, `product_total=0`,
`net_total=0`, `pay_total=0`, `balance=0` to `build_statements` regardless
of actual stay. Every checkout overwrites real revenue in MSSQL with zeros.
`service/checkin.rs` already has these on `CheckOutCommand` — just not
threaded through the intent. Fix: extend `WritebackIntent::CheckOut`
payload with the 5 totals; propagate.

**H2 — `checkout.rs:106` Room_Use_Count += 1 regardless of nights.**
Should be `Room_Use_Count + {nights}` per `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Rooms`" "Room_Use_Count=Room_Use_Count+<nights>"
and `docs/legacy-app/COMPAT_CHEATSHEET.md` §"3.2 Check-out & Settle" "Room_Use_Count=Room_Use_Count+<nights>".
Spike captures were 1-night stays so the bug was hidden. Multi-night
stays under-count by `nights-1`. Fix: parameterize from `inputs.nights`
(already in the payload after H1 fix).

**H3 — `payment.rs:213-217` violates legacy sum invariant.**
Sets `Cin_Pay_Ds_Price = nightly_total_2dp` but legacy invariant
(`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay`" "Cin_Pay_Cash+Cin_Pay_Credit+Cin_Pay_Free+Cin_Pay_Tran+Cin_Pay_web") is
`Cin_Pay_Ds_Price = Cash + Credit + Free + Tran + Web`. Partial payments
break the invariant; the shift report reading that sum becomes inconsistent.
Fix: `Cin_Pay_Ds_Price = Cin_Pay_Ds_PriceTotal = amount_2dp` (the actual
tender). Keep `Cin_Pay_Ds_PriceOne` and `Cin_Pay_Ds_Num` verbatim. Add a
`debug_assert!` on the sum.

**H4 — `payment.rs:249-254` HT_Receipt_Ds line missing `.00` formatting.**
Recipe emits `S_Unit=1, S_Price=801, S_Total=801, S_PriceDiscount=0`.
Live capture (`invoice-20260424-100827/writes.txt:8`) is `1.00, 711.00,
711.00, 0.00`. Doc-comment quoted the wrong snippet. Fix: route through
`money_2dp`.

**H5 — `domain/payment.rs:46` Transfer routed to wrong column.**
`PaymentMethod::Transfer.legacy_column()` returns `"Cin_Pay_Credit"` —
should be `"Cin_Pay_Tran"` per `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_CheckIn_Pay`" "`Cin_Pay_Tran`: bank-transfer amount". The recipe
at `payment.rs:146` is correct; the helper contradicts it. No current
caller uses the helper, so latent — but `pub`.

**H6 — `format.rs:131-134` round_money rule mismatches .NET.**
Comment claims it matches .NET's `Math.Round(value, 2)`. .NET defaults to
banker's rounding (`MidpointRounding.ToEven`); `round_money` uses
round-half-away-from-zero. Today's whole-baht prices never hit the 0.005
boundary, but the comment is wrong and will mislead. Fix: switch to
`.round_ties_even()` and add a test at the midpoint.

### Visibility / data loss

**H7 — `checkin_to_booking.rs:175-195` night-0 HT_Room_Status lost.**
Night-0 does `UPDATE HT_Room_Status WHERE room_date=… AND room_no=…`. Our
`booking_create` does not pre-insert these rows (only 4 INSERTs:
HT_Customers, HT_Book_H, HT_Book_Ds, HT_Book_Date). UPDATE silently
matches 0 rows; iHOTEL's calendar shows night-0 as empty.
Fix: `booking_create.rs` should also insert `HT_Room_Status` per night
with `status='จอง'` (matches `docs/legacy-app/COMPAT_CHEATSHEET.md` §"Table: `HT_Room_Status`" "**Insert booking day**: id from get_id, status='จอง'").
This is the cleanest fix — keeps checkin_to_booking unchanged.

**H8 — `booking_modify.rs:202-234` caption rewrite skipped on date-only edit.**
Caption rewrite (`UPDATE HT_Rooms SET room_book_ds=…`) only runs when
customer_name + room_no + stay are all `Some`. A date-only edit clears the
caption at step 0b but never re-writes it; calendar grid loses the booking
caption for the new date range. Fix: when `new_stay.is_some()`, resolve
customer_name + room_no from existing rows (mirror `fetch_existing_room_no`)
and always emit the caption rewrite.

**H9 — `booking_modify.rs:99-102` vs `booking_create.rs:109-114` date format mismatch.**
Create writes `Book_Date_in='4/25/2026'` (date-only). Modify writes
`'4/25/2026 12:00:00 AM'` (midnight). Same column, two shapes per
writeback session. The most recent capture set is date-only; modify is
wrong. The byte-parity test at `booking_create.rs:482` pins date-only.
Fix: modify uses `format_legacy_date(bangkok_date(stay.start))`; drop
the `midnight_of` wrapper. Update `§3k` table in both file headers.

**H10 — `booking_modify.rs:144-148` price-only modify corrupts multi-night total.**
`nights = new_nights_calendar.len().max(1)` is `1` when `new_stay` is None
(empty calendar vec). Price-only modify of a 3-night booking writes
`Book_Room_PriceToTal = baht * 1`, blowing away the multi-night total.
Fix: when `new_stay` is None and `new_price` is Some, look up existing
`Book_Room_Night` first (mirror `fetch_existing_room_no`).

### Robustness / injection / scope

**H11 — `bin/writeback.rs` `run_in_transaction` pool can leak open-tran connections.**
If its best-effort `ROLLBACK TRAN` fails, the bb8 conn returns to pool with
open transaction; bb8-tiberius has no `is_valid` per-checkout. Next
checkout reuses; next TABLOCKX hangs or commits to wrong tran.
Fix: defensive `IF @@TRANCOUNT > 0 ROLLBACK` at acquisition, or
`on_release` hook that drops the connection.
(Fixed since: `bin/writeback.rs` `RESET_TRANCOUNT_SQL`, run on every checkout.
Symbol anchors replace the original `:617-643` / `:634` line citations, which
now land in an unrelated Slack-alert formatter.)

**H12 — `recipes/mod.rs:111-136` SCOPE_IDENTITY across batch boundary.**
`execute_capturing_identity_at` runs the INSERT in one `simple_query`,
then `SELECT SCOPE_IDENTITY()` in a separate call. SCOPE_IDENTITY is
batch-scoped on the wire; tiberius may return NULL or zero.
Fix: replace with `OUTPUT INSERTED.id` on the INSERT itself — survives
any scope quirk. Failing that, concatenate `INSERT…; SELECT
CAST(SCOPE_IDENTITY() AS INT)` in one query and parse multi-result.

**H13 — Missing `validate_finite` on walkin / checkin_to_booking / checkout.**
`format!("{}", f64::NAN)` emits literal `NaN` in SQL → INSERT fails
mid-transaction, partial state. `checkin_cancel`, `booking_create`,
`booking_modify`, `extend_stay` all have the guard. Fix: add
`helpers::validate_finite(...)` at the top of `execute()` (or, better,
inside `build_statements` returning `WritebackResult<Vec<String>>`).

**H14 — `walkin.rs:149-155`, `checkin_to_booking.rs:139-145` empty tmp_no spoof.**
`UPDATE Tb_Save_Image … WHERE tmp_no=''` fires when payload sends
`Some("")`; re-stamps every orphan-pending-cleanup row with this
check-in's identifiers. Fix: filter inner block with
`.filter(|s| !s.trim().is_empty())` or normalize at payload boundary.

**H15 — `allocate.rs:115-122` LIKE pattern injects prefix via format!.**
Safe today (year/month are integer-derived), but pattern is templated.
Fix: validate prefix against `^[A-Z]{1,4}\d{0,4}-?$` before interpolating,
or compose via parameterized query.

---

## MED (~22 items) — defensive / edge case

Grouped by theme:

**Service → intent payload thin** — checkout, checkin_to_booking, payment
all need data the service already has but doesn't propagate:
- Customer phone wiped on booking-linked check-in (`checkin_to_booking.rs:282`)
- Real revenue totals not threaded to checkout (covered by H1)
- `price_per_night_baht` not passed to payment recipe → derived as
  `amount/nights`
- `ht_payments.legacy_pay_no` / `legacy_receipt_no` never back-populated to PG

**Multi-room blind spots:**
- `mark_clean.rs:96-102` joins `cin_status` (whole check-in) instead of
  `Cin_Room_Status` (per-room) → wrong "prior occupant" in multi-room
- `extend_stay.rs:150-152` step-5 only re-asserts `room_use='yes'` for one
  room while step-1 cleared all rooms of the check-in → multi-room
  extends leave rooms stuck `'no'`
- `HT_POWER_LOG` cancel UPDATE can close multiple open rows for same room

**Idempotency / retry safety:**
- `payment.rs` retry after network-drop-on-COMMIT doubles
  `Total_Price_Pay` (no `WHERE NOT EXISTS` guard on `Pay_no`)
- `bin/writeback.rs:1180` back-populates ht_* on `Err` arm too (should
  return after the log)

**Coverage gaps:**
- `fingerprint.rs` covers 10 tables; recipes touch `HT_Cupon`,
  `HT_CheckIn_Pay`, `HT_Receipt_Ds`, `HT_POWER_LOG`, `HT_Changed_Room` —
  vendor rename → silent corruption
- QR / web payments bypass writeback entirely
  (`service/payment.rs:164` → `insert_qr_payment_directly`)
- VAT internally inconsistent: `RECEIPT_VAT_PERCENT=7` vs spike's 0%
  hotel; `Total_Price_vat` accumulates gross, `Receipt_Vat` is real VAT

**Purity violations:**
- `Utc::now()` inside "PURE" `build_statements` (walkin, checkin_to_booking)
- `mark_clean.rs:102` `ORDER BY Cin_Room_Out DESC` with NULL handling
  database-default-dependent

**Other:**
- `booking_modify.rs` doesn't write `HT_Book_Ds.[Book_Room_Note]` when
  notes change — only header column updated
- `MED-3` `set_context_info` persists on pool checkout-reuse if pool is
  shared
- `booking_modify.rs:166-175` `Book_date_ds NOT IN (…)` relies on midnight
  consistency; cast to DATE for defense

---

## LOW (~25 items) — style / tightening

Highlights (full list in agent transcripts):

- Duplicate `end_of_stay_at_almost_noon`, `enumerate_calendar_nights`,
  `guest_prefix_for_country` across recipes → move to `format.rs` /
  `helpers.rs`
- `HT_Book_Ds.Book_Room_Num` hardcoded `1` — single-room assumption
  undocumented
- `booking_cancel.rs:63` "delete from  HT_Book_Date" double-space
  preserved from capture; document as intentional so future formatter
  doesn't normalize it
- Money format inconsistency: `{:.2}` in some columns, raw `{baht}` in
  others — codify via `money_2dp` helper
- `nights.max(1)` clamping floors masking caller bugs — raise validation
  to service layer
- `enumerate_calendar_nights` 365-night cap silently truncates, empty
  range silently injects phantom — log + error instead
- `allocate.rs:42-55` `select_next_int_with_lock` decodes as `i32`; will
  wrap at 2B (HT_Receipt_H, HT_Book_Date, HT_CheckIn_Ds) — add a
  sanity check / approaching-max alert
- Collation safety on Ville cutover: assert non-`_CS_` collation at
  startup
- `HT_Customers` INSERT byte-parity locked against R014820 capture but
  SCHEMA.sql truncated — regenerate from `information_schema.columns`
- `error.rs:71-90` `is_retryable` retries `unique_violation` forever —
  pattern-match on db codes (40001, 57P01, 08006)

---

## Fix waves (proposed)

Each wave is a separate PR. Order is by risk and dependency.

### Wave 1 — CRIT allocators (block live cutover)

Scope: C1 + C2 + C3.

- `allocate_pay_no` → `R{yyMM}-{4digit}` with BKK tz
- `allocate_receipt_no` → `B{yyMM}-{4digit}` with BKK tz
- `allocate_cin_no` BKK tz fix
- Derive `SUBSTRING` offset from prefix length, not hardcode
- New tests that exercise allocators end-to-end (not byte-parity with
  hardcoded IDs)
- Format-regex assertions in unit tests

Acceptance: a freshly-installed MSSQL with one existing iHOTEL `R2604-0250`
row produces `R2604-0251` as next Pay_No from our allocator.

### Wave 2 — financial integrity (HIGH)

Scope: H1, H2, H3, H4, H5, H6 + Wave 5 payment-idempotency MED.

- Extend `WritebackIntent::CheckOut` payload with real totals
  (`room_price_total`, `product_total`, `net_total`, `pay_total`,
  `balance`, `nights`)
- `Room_Use_Count += nights`
- Payment recipe: `Cin_Pay_Ds_Price = amount` (sum invariant)
- HT_Receipt_Ds 2dp formatting
- `PaymentMethod::Transfer.legacy_column() = "Cin_Pay_Tran"`
- `round_money` → banker's rounding
- Payment recipe `WHERE NOT EXISTS` guard on `Pay_no` for retry idempotency

Acceptance: a 3-night checkout writes real totals; a partial payment
satisfies `Cin_Pay_Ds_Price = sum(Cash+Credit+Free+Tran+Web)`.

### Wave 3 — visibility coordination (HIGH)

Scope: H7, H8, H9, H10.

- `booking_create.rs`: insert `HT_Room_Status` per booked night with
  `status='จอง'`, `room_Book_No=Book_ID` (resolves H7 at the source)
- `booking_modify.rs`: caption rewrite always fires when stay changes
  (resolve customer/room from existing rows)
- `booking_modify.rs` date format: align to date-only with create
- `booking_modify.rs` price-only modify: preserve existing
  `Book_Room_Night` when computing `Book_Room_PriceToTal`

Acceptance: a date-only booking edit leaves the calendar caption visible;
a multi-night booking-linked check-in produces N `HT_Room_Status` rows.

### Wave 4 — robustness (HIGH)

Scope: H11, H12, H13, H14, H15.

- `bin/writeback.rs`: defensive `IF @@TRANCOUNT > 0 ROLLBACK` at
  acquisition; `on_release` to discard poisoned conns
- `recipes/mod.rs`: replace `execute_capturing_identity_at` with
  `OUTPUT INSERTED.id` approach
- Add `validate_finite` to walkin / checkin_to_booking / checkout
- Guard empty `tmp_no` in `Tb_Save_Image` UPDATE (walkin +
  checkin_to_booking)
- Prefix-format validation regex in `allocate.rs`

Acceptance: a poisoned bb8 conn doesn't propagate; SCOPE_IDENTITY can't
return NULL; NaN/Infinity inputs surface as `Recipe(...)` errors before
any wire write.

### Wave 5 — MED cluster

Bundle by sub-theme:

- Payload thickening (customer_phone preserve, price_per_night to payment,
  ht_payments back-population)
- Multi-room semantics (mark_clean `Cin_Room_Status` filter, extend_stay
  multi-room `room_use`, HT_POWER_LOG row-target precision)
- Fingerprint expansion (add 5 missing tables)
- QR routing decision (doc OR plumb through with `Cin_Pay_web`)
- VAT consistency (read `TB_SETTINGS.VAT_PERCENT`, retire hardcoded
  constant)
- `Utc::now()` out of `build_statements`
- back-pop on `Err` arm guard

### Wave 6 — LOW tidying

Pick up opportunistically while touching files in higher waves; or one
PR sweep at the end.

---

## Confidence notes

- **High confidence:** all 3 CRITs (clear spec-vs-impl mismatch); H1
  (visible in code); H7 (mismatch between booking_create's 4 INSERTs and
  checkin_to_booking's UPDATE assumption).
- **Medium confidence:** H11 (pool-poisoning depends on bb8-tiberius
  internals not re-read); H12 (SCOPE_IDENTITY behaviour depends on
  tiberius driver — needs a live test).
- **Could not verify against live MSSQL:** any finding that requires
  multi-night / multi-room behaviour observation — H2, MED multi-room
  items. A fresh capture session targeting these scenarios would harden
  the claims.
- **Schema drift risk:** `docs/legacy-app/SCHEMA.sql` is the canonical
  legacy schema reference but is truncated mid-row on `HT_Customers` (line
  20); regenerate from `information_schema.columns` to fully verify
  HIGH-class column-name correctness.
