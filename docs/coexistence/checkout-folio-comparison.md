# Checkout receipt comparison — iHOTEL (old) vs new server folio

**For reception verification before flipping `CHECKOUT_SERVER_TOTAL_ENABLED`.**
Updated 2026-06-27.

---

## ⛔ VERIFICATION RESULT (2026-06-27): DO NOT FLIP — folio is broken

Ran the comparison against live canonical PG. The server folio computes the room
charge as `cin_rate_per_night × nights`, but **`cin_rate_per_night = 0` for 100%
of check-ins** (hotelnew: 19,827 zero + 2 null, **0 positive**; hotelville:
1,873 zero, **0 positive**). There is **no fallback** to `cin_total_amount`.
Also, **`ht_pos_sales` is empty (0 rows)** — this hotel has no POS/product data,
so the entire "products" half of Phase 2 parity is moot.

Net effect of flipping the flag: every checkout would record/writeback
**room = 0, net = 0, balance = −(amount paid)** — i.e. it would **zero out the
room revenue and corrupt the legacy `Total_Price_*` columns**. Sample of 12 real
recent checkouts (folio replica vs reality):

| cin_no | nights | ACTUAL total | folio room | folio NET | folio balance |
|--------|--------|--------------|-----------|-----------|---------------|
| CH26-005900 | 1 | 490.00  | **0.00** | **0.00** | **−490.00** |
| CH26-005901 | 1 | 828.00  | **0.00** | **0.00** | **−828.00** |
| CH26-005907 | 1 | 890.00  | **0.00** | **0.00** | **−890.00** |
| CH26-005895 | 2 | 1,656.00| **0.00** | **0.00** | **−1,656.00** |
| CH26-005892 | 2 | 3,070.00| **0.00** | **0.00** | **−3,070.00** |
| …(all 12 identical pattern) | | | | | |

The real charged amount lives in `cin_total_amount` (= `cin_paid_amount`, synced
from iHOTEL). `ht_payments` is populated (706 rows), so the pay side is fine.

**Required fix before flip:** base room/net on `cin_total_amount` (e.g.
`room_total = rate>0 ? rate×nights : cin_total_amount`; derive display rate =
total ÷ nights). Then re-run this comparison. Until then `#30` is **BLOCKED**.

Everything below was the as-designed comparison plan; it stands, but the §3/§4
worked numbers assume a non-zero rate that does not exist in the data yet.

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
