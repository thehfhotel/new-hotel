# Checkout receipt comparison — iHOTEL (old) vs new server folio

**For reception verification before flipping `CHECKOUT_SERVER_TOTAL_ENABLED`.**
Re-verified 2026-06-30 (post-#34).

---

## ✅ RE-VERIFICATION (2026-06-30): folio now matches iHOTEL — flippable, one caveat

The 2026-06-27 ⛔ result (folio room/net = 0, would corrupt `Total_Price_*`) is
**superseded**. Task #34 added the fallback `room_total = rate>0 ? rate×nights :
cin_total_amount`. Re-ran the comparison against live canonical PG **and** the HF
Hotel legacy MSSQL (`192.168.100.222,1433`, db `db`):

- `cin_rate_per_night` is still 0 for ~100% of check-ins → the fallback fires →
  `room_total = cin_total_amount`. `ht_pos_sales` is still empty (0 rows both
  sites) → `product_total = 0` → `net_total = cin_total_amount`.
- **Folio `net_total` == iHOTEL `Total_Price_Net` to the cent** on 5 spot-checked
  real bills, and the 12-row replica reconciles (`SUM(ht_payments) == cin_total_amount`
  → `balance = 0` every row):

| cin_no | folio net (PG) | iHOTEL Net | iHOTEL Room | iHOTEL Product | balance |
|--------|----------------|------------|-------------|----------------|---------|
| CH26-005886 | 3,560.00  | 3,560.00  | 3,560.00  | 0 | 0 |
| CH26-005899 | 1,780.00  | 1,780.00  | 1,780.00  | 0 | 0 |
| CH26-005902 | 1,547.78  | 1,547.78  | 1,547.78  | 0 | 0 |
| CH26-005908 | 2,797.20  | 2,797.20  | 2,797.20  | 0 | 0 |
| CH26-005931 |   890.00  |   890.00  |   890.00  | 0 | 0 |

Note iHOTEL itself stores `Total_Price_Room == Total_Price_Net` (whole bill in the
room bucket, Product=0) — exactly what the `cin_total_amount` fallback reproduces.

**Difference A (nights basis) is now MOOT.** Because `room_total` comes from
`cin_total_amount`, not `nights × rate`, the late-checkout "+1 night?" policy
question no longer affects the total. (`nights` only drives the cosmetic
displayed per-night rate `= total ÷ nights`.)

**Live impact of flipping today ≈ none.** `shadow.checkout_total` = 0 events and
no new-app checkouts occur (checkouts ride iHOTEL), so the flag's writeback path
is essentially unexercised; flipping writes back the same value already present.

### ⚠ One landmine to resolve BEFORE new-app POS is used at checkout

`net_total = cin_total_amount + product_total`. While the rate-fallback fires
(rate=0) AND `ht_pos_sales` is empty, this is exactly `cin_total_amount` — safe.
But once new-app POS lands rows in `ht_pos_sales` for a stay whose
`cin_total_amount` (synced from iHOTEL's net) **already includes** those items,
`net_total` would **double-count** them. This is dormant today (POS unused) but
must be addressed (e.g. base room on `cin_total_amount − pos`, or only add
`product_total` when the rate path is used) before the new-app folio is the
source of truth for stays with POS. Until then, flipping is safe **only because
POS is empty**.

**Verdict:** the #30 blocker is cleared and parity is exact. Flipping is safe now
(near-no-op) — recommended to flip as part of go-live alongside the POS
double-count fix, rather than in isolation, since its value materializes only when
new-app checkouts actually happen. The flip itself is a prod money-path config
change (set `CHECKOUT_SERVER_TOTAL_ENABLED=true` for the backend via the pipeline)
and should be made with reception aware.

Everything below is the original as-designed comparison plan + worksheet; it
stands (the §3/§4 worked examples assume a non-zero rate, which the live data does
not have — the fallback path above is what actually runs).

---

## Why this exists / ทำไมต้องตรวจสอบ

Today, when a guest checks out in the **new app**, the **total is whatever the
receptionist sees on screen** — computed in the browser as simply
`จำนวนคืน × ราคา/คืน` (nights × rate). It does **not** include shop/POS items, and
it counts nights by **elapsed time** (`ceil(now − check-in)`).

We have built a **server-computed folio** (Phase 2) that matches the way **iHOTEL**
builds a checkout bill: room + products, VAT (inclusive), deposit shown
separately, balance = net − paid. It is **shipped dark** (`CHECKOUT_SERVER_TOTAL_ENABLED=off`)
so nothing has changed for guests yet.

Before we turn it on, we need reception to confirm: **for real recent checkouts,
does the new server folio match the iHOTEL receipt?** This document is the
field mapping, the two places they can differ, worked examples, and a blank
worksheet to fill with real bills.

> The new folio can be read **safely, read-only, without turning anything on** —
> the `checkout-quote` endpoint is not gated. See "How to pull the numbers" below.

---

## 1. Field mapping / การจับคู่ช่อง

iHOTEL stores the bill in `HT_CheckIn_H.Total_Price_*`. The new folio
(`/api/checkins/{id}/checkout-quote`) returns the same shape. They are designed
to line up one-to-one:

| iHOTEL (`HT_CheckIn_H`)      | New folio field | Meaning / Thai            | Formula (both sides)                         | Should match? |
|------------------------------|-----------------|---------------------------|----------------------------------------------|---------------|
| `Total_Price_Room`           | `roomTotal`     | ค่าห้องพัก                  | `rate × nights`                              | ⚠ nights basis differs — **verify** |
| `Total_Price_Product`        | `productTotal`  | ค่าสินค้า/บริการ            | Σ POS sales, non-voided                      | ⚠ old browser total **omits this** |
| `Total_Price_Net`            | `netTotal`      | ยอดรวม                     | `Room + Product`                             | ✅ if the two above match |
| `Total_Price_Pay`            | `payTotal`      | ชำระแล้ว                   | Σ payments, non-voided                       | ✅ |
| `Total_Price_Balance`        | `balance`       | คงเหลือ                    | `Net − Pay`                                  | ✅ |
| `Total_Price_vat`            | `vat`           | ภาษี (ในราคา / inclusive)  | `net × v ÷ (100 + v)`                        | ✅ (HF Hotel `v = 0` → 0; same formula both sides) |
| `Cin_Room_Dep`               | `deposit`       | เงินมัดจำ                   | Σ deposits (shown, **not** subtracted from balance) | ✅ |
| `Cin_Room_night`             | `nights`        | จำนวนคืน                   | date-diff (see §2)                           | ⚠ **the key check** |

The **structure is identical** to iHOTEL. There are only **two places** the
*numbers* can diverge, both isolated to the room charge. Everything downstream
(net, pay, balance, VAT, deposit) is pure arithmetic on those.

---

## 2. The two places they can differ

### Difference A — how nights are counted (the decision we need from reception)

| | Counts nights as | Example: in Mon 15:00, out Wed 18:00 |
|---|---|---|
| **Old browser total** (charged today) | `ceil(elapsed time)` — any part of a day rounds **up** | `ceil(2 days 3h) = ` **3 nights** |
| **New server folio** (= iHOTEL design) | calendar dates: `checkout-date − checkin-date` | Mon→Wed = **2 nights** |

So on a **late checkout** (guest leaves after the noon checkout time), the old
browser charges an extra night; the new folio charges by calendar date.

**This is the policy question for reception:** when a guest checks out late, does
iHOTEL (what you do today) charge the extra night or not? The new folio must be
set to match your real practice. Please confirm against a few **late-checkout**
bills specifically.

> Implementation note (for us): the new folio nights =
> `(COALESCE(actual_checkout, expected_checkout)::date − checkin::date)`, min 1.
> If reception's rule is "late checkout = +1 night," we adjust the server rule
> before flipping — that's exactly what this verification decides.

### Difference B — shop / POS items (an old-app bug the new folio fixes)

The **old browser total ignores POS items entirely** — it only charges
`nights × rate`. iHOTEL includes them in `Total_Price_Product`, and so does the
new folio. So for any bill that had minibar / shop / service items, the old app
**under-charged**, and the new folio brings the new app **back in line with
iHOTEL**. Worth flagging to reception as a *fix*, not a regression.

---

## 3. Worked examples / ตัวอย่าง

All amounts in ฿. "Old app" = what the new app charges today (flag off).

**Example 1 — normal checkout, no shop items → everything matches**
Room 305, rate ฿700. In: 20 Jun 15:00 · Out: 22 Jun 11:00 (on time).

| | Nights | Room | Product | Net | Pay | Balance |
|---|---|---|---|---|---|---|
| iHOTEL receipt | 2 | 1,400 | 0 | 1,400 | 1,400 | 0 |
| New folio      | 2 | 1,400 | 0 | 1,400 | 1,400 | 0 |
| Old app        | 2 | 1,400 | — | 1,400 | — | — |
→ **✅ match.**

**Example 2 — LATE checkout → nights basis diverges (the decision)**
Room 402, rate ฿700. In: 20 Jun 15:00 · Out: 22 Jun **18:00** (late).

| | Nights | Room | Net |
|---|---|---|---|
| New folio (calendar) | 2 | 1,400 | 1,400 |
| Old app (`ceil` elapsed) | 3 | 2,100 | 2,100 |
| iHOTEL | **?** | **?** | **?** ← reception confirms which one matches |

→ If iHOTEL = 2 nights, the **new folio is right** and the old app was
over-charging late checkouts. If iHOTEL = 3, we change the server rule to "+1
night on late checkout" before flipping.

**Example 3 — with shop items → new folio fixes an under-charge**
Room 210, rate ฿700, 2 nights, minibar ฿250.

| | Room | Product | Net |
|---|---|---|---|
| iHOTEL receipt | 1,400 | 250 | 1,650 |
| New folio      | 1,400 | 250 | 1,650 |
| Old app        | 1,400 | **0** | **1,400** ← missed the ฿250 |
→ **✅ new folio matches iHOTEL; old app under-charged ฿250.**

---

## 4. Verification worksheet / ตารางตรวจสอบ

Pick **8–10 recent real checkouts** — include at least 2 **late checkouts** and
2 with **shop items**. Fill iHOTEL from the printed receipt / `HT_CheckIn_H`;
fill "New folio" from the `checkout-quote` endpoint (§5). Mark ✓ if they match.

| # | CIN_NO | Date | iH Nights | iH Room | iH Prod | iH Net | iH Pay | iH Bal | Folio Nights | Folio Room | Folio Prod | Folio Net | Folio Pay | Folio Bal | Match? | Note |
|---|--------|------|----------|---------|---------|--------|--------|--------|--------------|------------|------------|-----------|-----------|--------|------|
| 1 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 2 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 3 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 4 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 5 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 6 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 7 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |
| 8 |        |      |          |         |         |        |        |        |              |            |            |           |           |        |      |

**Pass criteria:** every row matches **except** the nights column on late
checkouts, where the answer tells us the policy to set. Any *other* mismatch =
investigate before flipping.

---

## 5. How to pull the numbers (for us, not reception)

**New folio (read-only, safe, flag-independent):**
```
GET https://hotel.thehfhotel.org/api/checkins/{cin_id}/checkout-quote
# HF Ville: add the branch the UI uses (branch header/param), since the folio is per-pool
```
Returns `{ nights, ratePerNight, roomTotal, productTotal, vatPercent, vat,
deposit, netTotal, payTotal, balance }`. No writes, no side effects — calling it
does **not** change anything for the guest.

**iHOTEL side (legacy MSSQL, read-only):**
```sql
SELECT Cin_no, Total_Price_Room, Total_Price_Product, Total_Price_Net,
       Total_Price_Pay, Total_Price_Balance, Total_Price_vat
FROM HT_CheckIn_H
WHERE Cin_no = '<CIN_NO>';
-- nights per room: SELECT Cin_Room_night FROM HT_CheckIn_Ds WHERE Cin_no='<CIN_NO>'
```

**Shadow log (already running in prod, sizes the gap automatically):**
```
docker logs new-hotel-production-backend-1 | grep 'shadow.checkout_total'
```
Each line records `client_total` vs `server_room_total` and the `delta` for any
checkout where they differ — a free, accumulating sample of exactly the
Difference-A / Difference-B cases above.

---

## 6. The decision this produces

1. **Nights policy** (Difference A) — confirm with reception whether late
   checkouts add a night in iHOTEL, and set the server rule to match.
2. Once the worksheet passes (with the nights rule agreed) and the shadow log
   shows no surprising deltas, flip `CHECKOUT_SERVER_TOTAL_ENABLED=true` in
   `docker-compose.yml` via the pipeline.

Related: `docs/spikes/2026-06-27-frontend-backend-encapsulation.md`,
`docs/VERIFICATION-CHECKLIST.md` (Section B, Phase 2).
