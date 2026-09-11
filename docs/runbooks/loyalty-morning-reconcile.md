# Runbook — การตรวจสอบยอดจองแอปตอนเช้า / Loyalty morning reconciliation

> **ใครทำ / Who:** พนักงานต้อนรับที่เปิดกะเช้า — The receptionist opening the morning shift.
> **ทำเมื่อไหร่ / When:** ทุกเช้า ก่อนเคาน์เตอร์เริ่มยุ่ง (ประมาณ 07:00) — Every morning, before the desk gets busy.
> **ใช้เวลา / Duration:** 5 นาที — Five minutes. ส่วนใหญ่จะไม่มีอะไรเลย / most mornings the list is empty.
> **หน้าจอ / Where:** `GET /api/reports/loyalty-reconcile`
> **ทำไม / Why:** iHOTEL กับแอปอาจไม่ตรงกัน และไม่มีใครเห็นจนกว่าแขกจะมาถึงเคาน์เตอร์ — iHOTEL and our app can disagree, and nobody finds out until the guest is standing at the counter.

รายงานนี้อ่านจาก PostgreSQL อย่างเดียว ไม่แตะ iHOTEL เลย จึงยังตอบได้แม้ตอนที่ฝั่ง iHOTEL ล่ม
— This report reads **only** PostgreSQL and never touches MSSQL, so it keeps answering when the
legacy leg is the thing that is broken.

**นี่คือการตรวจก่อนเปิดใช้งานจริง (PRE-FLIP)** — B8 checklist **L6**. ตอนนี้ช่องทางแอปยังปิดอยู่
(dark) รายการจะว่างเปล่าทุกเช้า นั่นถูกต้องแล้ว — This is a **pre-flip** control: while the loyalty
channel is still dark the list is empty every morning, and that is the correct result, not a
broken report.

---

## 1. ขั้นตอน 5 นาที / The five-minute routine

1. **เปิดรายงาน / Open the report.**
   `GET /api/reports/loyalty-reconcile` (ใส่ `?branch=hfville` สำหรับ HF Ville / add
   `?branch=hfville` for HF Ville). ไม่ต้องใส่วันที่ — ระบบใช้วันนี้ตามเวลาไทยอยู่แล้ว / omit
   `date` and it uses today in Bangkok time.

2. **ดูบรรทัด `clear` ก่อน / Read `summary.clear` first.**
   ถ้า `clear: true` → **จบ ไม่ต้องทำอะไรต่อ** ปิดหน้าจอได้เลย
   — `clear: true` means **nothing to do**. Close the report. This is the normal morning.

   > `clear` คิดจาก `defects` ไม่ใช่ `total` — แถว `deposit_divergence` ไม่ใช่ความผิดพลาด
   > แต่เป็นข้อมูลที่ต้องบอกกะ / `clear` is computed from `defects`, not `total`: a
   > `deposit_divergence` row is not a fault, it is a briefing item.

3. **ถ้ามีรายการ ให้ทำจากบนลงล่าง / If there are rows, work top-down.**
   รายการเรียงตามความรุนแรงอยู่แล้ว — บนสุดคือเรื่องที่ทำให้ห้องถูกขายซ้ำได้
   — The list is already sorted by blast radius: the top row is the one that can double-sell a
   room. Do not re-sort it.

4. **ดูช่อง `action` ของแต่ละแถว / Read each row's `action`.** มีแค่สามอย่าง / only three verbs:

   | `action` | ภาษาไทย | English | หัวข้อ / Section |
   |---|---|---|---|
   | `call_guest` | **โทรหาแขก** | Phone the guest | [§2.1](#21-writeback_stalled--ihotel-ไม่เห็นการจองนี้) |
   | `resend_hold` | **แจ้งช่าง/ผู้ดูแลระบบให้ส่งซ้ำ** | Have the leg re-driven | [§2.2](#22-legacy_hold_orphan--ihotel-ยังจองห้องไว้ทั้งที่ยกเลิกแล้ว), [§2.3](#23-sweep_lag--holds-ที่หมดอายุแล้วแต่ยังไม่ถูกปล่อย) |
   | `brief_desk` | **บอกกะที่เข้าเวร** | Tell the shift | [§2.4](#24-unlinked_checkin--เช็คอินแล้วแต่ไม่ได้ผูกกับการจอง), [§2.5](#25-deposit_divergence--ihotel-จะแสดงมัดจำเป็น-0) |

5. **บันทึกถ้ามีอะไรถึงแขก / Log anything that reached a guest**
   ใน `docs/coexistence/sync-incident-log.md`.

**ห้ามทำเด็ดขาด / Never, at any hour:** เขียนแถวลง iHOTEL เอง, แก้ `writeback_jobs` ด้วยมือ,
หรือเปิด flag เพื่อ "ให้มันไหล" — hand-write legacy rows, edit `writeback_jobs` by hand, or flip a
writeback flag to make it go. การเขียนลงฐานข้อมูลเก่าทุกครั้งต้องผ่านการนัดหมายกับแผนกต้อนรับก่อน /
every new legacy write ships dark and needs coordinated verification first.

---

## 2. ทำอย่างไรกับแต่ละชนิดแถว / What to do for each row kind

### 2.1 `writeback_stalled` — iHOTEL ไม่เห็นการจองนี้

**เกิดอะไรขึ้น / What happened.** แขกจองผ่านแอปสำเร็จแล้ว เงินอาจจ่ายมัดจำแล้วด้วย แต่ยังเขียนลง
iHOTEL ไม่สำเร็จ **กระดานห้องใน iHOTEL จึงแสดงว่าห้องนี้ว่าง ทั้งที่ขายไปแล้ว**
— The guest booked successfully and may have paid a 50% deposit, but the legacy write has not
landed. **The iHOTEL room board shows that room as FREE while our app has it sold.** This is the
only kind that can produce a double-sold room-night.

**ทำอย่างไร / What to do:**

1. **ถือว่า iHOTEL ผิดเรื่องห้องนี้ — ให้ถือว่าห้องไม่ว่าง.** ถ้ามี walk-in ขอห้องนี้ ให้ห้องอื่น
   — Treat the room as OCCUPIED. Give a walk-in a different room.
2. **จองห้องในกระดาน iHOTEL ด้วยมือ** แบบเดียวกับจองทางโทรศัพท์ พร้อมจดเลข `bookNo`
   นี่คือการกันห้องด้วยมือ ไม่ใช่ writeback และจะไม่ชนกัน
   — Block the room on the iHOTEL board the way you would for a phone booking, noting the
   `bookNo`. This is a manual hold; it does not conflict with the writeback when it lands.
3. **โทรหาแขก** ยืนยันว่าการจอง "ยังอยู่" และห้องยังเป็นของเขา ยืนยันจากฝั่งแอป/PMS เท่านั้น
   **อย่ายืนยันจากกระดาน iHOTEL** ซึ่งเป็นหน้าจอที่ผิดอยู่ตอนนี้
   — Phone the guest and confirm from the app/PMS side, **never** from the iHOTEL room board,
   which is the surface that is currently wrong.
4. **ดูช่อง `bookStatus`:** ถ้าเป็น `confirmed` แปลว่าแขก**จ่ายเงินแล้ว** — เรื่องนี้ด่วนที่สุด
   ถ้าเป็น `pending` แขกยังไม่จ่าย และ hold อาจหมดอายุเองใน 2 ชั่วโมง
   — `confirmed` means the guest has **paid**; that is the most urgent case. `pending` means the
   hold may self-cancel at its 2-hour TTL.
5. **ถ้ามีหลายแถว (มากกว่า 3)** แปลว่า writeback worker น่าจะล่มทั้งตัว → ไปที่
   [`writeback-leg-degraded.md`](writeback-leg-degraded.md) §4 และแจ้งผู้ดูแลระบบ
   — More than a handful of these means the worker itself is down: escalate via the
   writeback-leg-degraded runbook.

> แถวนี้ใช้เงื่อนไขเดียวกันกับการแจ้งเตือน Track F5 เป๊ะ ๆ ถ้ามันขึ้นที่นี่ มันจะแจ้งเตือนด้วย
> — This kind shares its predicate with the Track F5 Slack alert: a row here is a row that would
> page, and vice versa. If you saw the alert overnight, this is the same booking.

### 2.2 `legacy_hold_orphan` — iHOTEL ยังจองห้องไว้ ทั้งที่ยกเลิกแล้ว

**เกิดอะไรขึ้น / What happened.** hold หมดอายุ (หรือถูกยกเลิก) ในระบบเราเรียบร้อยแล้ว แต่คำสั่ง
ยกเลิกยังส่งไป iHOTEL ไม่สำเร็จ **iHOTEL จึงยังแสดง `จอง` ค้างไว้สำหรับห้องที่จริง ๆ แล้วว่าง**
— The hold is dead in PostgreSQL, but the cancel never reached iHOTEL, so **iHOTEL still shows
`จอง` for a room that is actually free.** ตรงข้ามกับ §2.1: ไม่ใช่ขายซ้ำ แต่เป็นการเสียห้อง /
inverted harm — not a double-sell, but a room we will fail to sell.

**ทำอย่างไร / What to do:**

1. **ห้ามลบแถว `จอง` ใน iHOTEL ด้วยมือ** — ปล่อยไว้ก่อน ระบบจะส่งซ้ำเอง
   — **Do NOT delete the `จอง` row in iHOTEL by hand.** The retry will land.
2. **บอกกะว่าห้องนี้ว่างจริง** ถ้ามี walk-in ขอห้องนี้ **ให้ได้** และลงในระบบเราตามปกติ
   — Tell the shift the room is genuinely available. A walk-in may take it; enter it in our app
   as normal.
3. **ถ้าค้างเกิน 1 ชั่วโมง (`age` > `1h 00m`)** แจ้งผู้ดูแลระบบให้ส่ง writeback ซ้ำ —
   ดู [`writeback-leg-degraded.md`](writeback-leg-degraded.md) §4
   — If `age` is over an hour, ask for the leg to be re-driven.

### 2.3 `sweep_lag` — holds ที่หมดอายุแล้วแต่ยังไม่ถูกปล่อย

**เกิดอะไรขึ้น / What happened.** hold เลยเวลา `holdExpiresAt` แล้ว แต่สถานะยังเป็น `pending`
ปกติระบบกวาดทุก 5 นาที **ห้องนี้จึงถูกกันไว้ในแอปเราโดยไม่มีใครจ่ายเงิน**
— A hold is past its deadline but still `pending`; the sweep runs every 5 minutes. **The room is
being held in our own app against a guest who has not paid.**

**ทำอย่างไร / What to do:**

1. **ถ้า `age` น้อยกว่า 10 นาที — ไม่ต้องทำอะไร** รอบกวาดถัดไปจะจัดการเอง โหลดใหม่อีกครั้งตอนท้ายกะ
   — Under 10 minutes: **do nothing.** The next sweep tick handles it. Re-check later in the shift.
2. **ถ้า `age` เกิน 15 นาที** แปลว่า scheduler ไม่ทำงาน แจ้งผู้ดูแลระบบ
   — Over 15 minutes means the scheduler is not running: escalate.
3. **ระหว่างนี้ ถือว่าห้องยังไม่ว่าง** อย่าเพิ่งขายห้องนี้ให้ walk-in จนกว่าแถวจะหายไป — ถ้าแขกจ่าย
   เงินเข้ามาพอดี hold จะกลายเป็น `confirmed` และห้องเป็นของเขาจริง ๆ
   — Meanwhile treat the room as **still held**: a late payment can still flip it to `confirmed`,
   and that guest has genuinely bought the room.

> ⚠️ อย่าสับสนระหว่าง `holdExpiresAt` ที่ผ่านมาแล้ว กับ "แขกเสียห้อง" — ระบบ**ไม่ลบ**
> `holdExpiresAt` ตอนที่แขกจ่ายเงิน ดังนั้นการจองที่ `bookStatus: "confirmed"` ซึ่งมี
> `holdExpiresAt` เก่า ๆ คือแขกที่**จ่ายเงินแล้ว** ห้องเป็นของเขา
> — `confirm_booking_payment` deliberately leaves `book_hold_expires_at` in place, so a
> **`confirmed`** booking with an old deadline is a guest who has **paid**. Never resell that
> room. This report only ever puts `pending` bookings in this kind, precisely so that mistake
> cannot be made from the list.

### 2.4 `unlinked_checkin` — เช็คอินแล้ว แต่ไม่ได้ผูกกับการจอง

**เกิดอะไรขึ้น / What happened.** แขกที่จองผ่านแอปถูกเช็คอินเข้าไปโดยไม่มีการผูกกับใบจอง
(เช็คอินจาก iHOTEL หรือเช็คอินแบบ walk-in จากกระดานห้องของเรา) **ป้ายแจ้ง "แขกจ่ายมัดจำมาแล้ว"
จะไม่ขึ้นที่ไหนเลย** — ไม่ขึ้นในหน้าเช็คอิน ใบลงทะเบียน โฟลิโอ หน้าจอรับเงิน หรือหน้าเช็คเอาต์
— An app guest was checked in with no booking link, so **every app-deposit signpost is invisible**:
the check-in modal, the printed registration slip, the folio, the payment dialog and the checkout
modal all show nothing. This is the B7a gap.

**ความเสี่ยง / The risk:** **เก็บเงินแขกซ้ำตอนเช็คเอาต์** — charging the guest for money they
already transferred in the app.

**ทำอย่างไร / What to do:**

1. **ดูช่อง `depositAmount`** ถ้ามีตัวเลข → **นี่คือเงินที่แขกจ่ายมาแล้ว** เขียนใส่กระดาษโน้ตติดไว้
   ที่โฟลิโอ และบอกกะทุกกะจนกว่าแขกจะเช็คเอาต์
   — If `depositAmount` is set, that is money already in the bank. Note it on the folio and brief
   every shift until the guest checks out.
2. **บอกกะ:** "ห้อง `roomNo` แขกจ่ายมัดจำมาทางแอปแล้ว `depositAmount` บาท — หักตอนเช็คเอาต์"
   — Brief the shift with the room number and the amount.
3. **ครั้งต่อไป: ใช้แอปเราเช็คอินแขกที่จองผ่านแอป** จากหน้ารายการจอง หรือกระดานห้อง (`จองแล้ว` →
   **เช็คอิน**) ไม่ใช่เช็คอินแบบ walk-in และไม่ใช่จาก iHOTEL — การผูกจะเกิดขึ้นทันที
   — **Prevention:** check an app booking in from *our* reservations list or room board
   (`จองแล้ว` → **เช็คอิน**), not as a walk-in and not from iHOTEL. The link is written in the
   same transaction and the signposts render immediately.
4. **ห้ามใส่ยอดมัดจำของใบจองลงในช่อง `เงินมัดจำที่รับที่เคาน์เตอร์`** — คนละก้อนกัน:
   ยอดในใบจองคือเงินโอนเข้าบัญชี ส่วนช่องที่เคาน์เตอร์คือเงินสดในลิ้นชัก ถ้าใส่ผิด ระบบจะคืนเงินสด
   ให้แขกตอนเช็คเอาต์สำหรับเงินที่เขาโอนมา
   — **Never** copy the booking deposit into the desk's `เงินมัดจำที่รับที่เคาน์เตอร์` field.
   `book_deposit_amount` is money in the bank; `cr_dep_amount` is money in the drawer. Conflating
   them refunds the guest in cash for a transfer.

### 2.5 `deposit_divergence` — iHOTEL จะแสดงมัดจำเป็น 0

**นี่ไม่ใช่ข้อผิดพลาด / This is NOT a fault.** เป็นเรื่องที่ออกแบบไว้แบบนี้และจะเป็นแบบนี้ตลอดไป
— it is a documented, permanent divergence. iHOTEL ไม่มีช่องเก็บมัดจำที่ recipe ของเราเขียนได้
โดยไม่ผิด byte-parity (`booking_modify` ไม่มีขา `Book_Price_Pay`) ดังนั้น **iHOTEL จะแสดง 0
จนกว่าจะเช็คเอาต์** ความจริงของเงินจะไปปรากฏที่โฟลิโอตอนเช็คเอาต์
— The validated `booking_modify` recipe has no `Book_Price_Pay` leg and inventing one would break
byte-parity, so **iHOTEL shows deposit 0 until checkout**. Folio truth lands at checkout.

แถวเหล่านี้**ไม่ถูกนับเป็น `defects`** และไม่ทำให้เช้าวันนั้น "ไม่ clear"
— These rows do not count toward `defects` and never make a morning read red.

**ทำอย่างไร / What to do:**

1. **บอกกะ ก่อนแขกมาถึง:** "ห้อง `roomNo` (`bookNo`) แขกจ่ายมัดจำมาแล้ว `depositAmount` บาท —
   **iHOTEL จะขึ้น 0 อย่าไปเชื่อ**"
   — Brief the shift before the guest arrives: name the room, the booking number, the amount, and
   say plainly that iHOTEL will read 0 and must not be believed.
2. **ดู `summary.depositTotal`** คือยอดรวมที่**ห้ามเก็บซ้ำ**วันนี้/พรุ่งนี้
   — `summary.depositTotal` is the total the desk must not ask for twice today and tomorrow.
3. **เช็คอินแขกกลุ่มนี้จากแอปเรา** (ดู §2.4 ข้อ 3) ป้ายแจ้งมัดจำจะขึ้นเองทุกหน้าจอ
   — Check these guests in from **our** app and every signpost renders automatically. This is the
   single action that makes this whole kind harmless.

---

## 3. อ่านค่าในรายงาน / Reading the response

```jsonc
{
  "success": true,
  "date": "2026-09-11",          // วันที่ใช้ (วันนี้ตามเวลาไทย) / resolved business day, Bangkok
  "through": "2026-09-12",       // ครอบคลุมถึง (วันนี้ + พรุ่งนี้) / deposit look-ahead end
  "stallThresholdMinutes": 10,   // งานที่ใหม่กว่านี้ยังไม่ขึ้น / jobs younger than this are in-flight
  "summary": {
    "total": 3,                  // ทุกแถว / every row
    "defects": 1,                // ที่ต้องทำอะไรจริง ๆ / rows that are actually wrong
    "clear": false,              // defects == 0 → ไม่ต้องทำอะไร / nothing to do
    "writebackStalled": 1,
    "legacyHoldOrphan": 0,
    "sweepLag": 0,
    "unlinkedCheckin": 0,
    "depositDivergence": 2,
    "depositTotal": 2400.0,      // บาท / baht — ห้ามเก็บซ้ำ / do not collect twice
    "oldestDefectMinutes": 34
  },
  "rows": [ /* เรียงตามความรุนแรง / sorted by blast radius */ ]
}
```

**ช่อง `age` หมายถึงคนละอย่างในแต่ละชนิดแถว / `age` means different things per kind** — นี่ตั้งใจ
เพราะแต่ละชนิดมีนาฬิกาคนละเรือน:

| `kind` | `age` นับจาก / measured from |
|---|---|
| `writeback_stalled`, `legacy_hold_orphan` | ตั้งแต่ job เข้าคิว / since the writeback job was enqueued |
| `sweep_lag` | ตั้งแต่ hold หมดอายุ / since `holdExpiresAt` passed |
| `unlinked_checkin` | ตั้งแต่แขกเช็คอิน / since the guest checked in |
| `deposit_divergence` | ไม่มี (ว่าง) — ไม่มีอะไรสาย / none — nothing is late |

**เวลา / Timestamps.** `checkedInAt` เก็บเป็นเวลาไทยแบบไม่มี timezone (มิเรอร์มาจาก MSSQL)
ถ้าเอาไปแสดงผล **ต้องใช้ `timeZone: 'UTC'`** เพื่อให้แสดงค่าที่เก็บไว้ตรง ๆ **ห้ามใช้
`Asia/Bangkok`** เพราะจะบวกไป 7 ชั่วโมงซ้ำอีกรอบ
— `checkedInAt` is naive local Thai wall time mirrored from MSSQL. Format it with
`timeZone: 'UTC'`; **never** `Asia/Bangkok`, which would shift it a second time.
`holdExpiresAt` is a real `TIMESTAMPTZ` instant and formats normally.

---

## 4. ถ้ารายงานใช้ไม่ได้ — SQL อ่านอย่างเดียว / If the route is down: read-only SQL

รันบน PostgreSQL (`hotelnew`, หรือ `hotelville` สำหรับ HF Ville) ทุกคำสั่ง **อ่านอย่างเดียว**
ไม่มี `INSERT` / `UPDATE` / `DELETE` — Run against PostgreSQL. Every statement below is
**read-only**; there is no write in this section and none should be added.

### 4.1 `writeback_stalled` + `legacy_hold_orphan`

เงื่อนไขเดียวกับ Track F5 (`scheduler::sync::fetch_stalled_loyalty_writebacks`) เป๊ะ ๆ
— The exact Track F5 predicate.

```sql
SELECT CASE
         WHEN j.intent = 'cancel_booking' AND b.book_status = 'cancelled'
           THEN 'legacy_hold_orphan'      -- iHOTEL ยังค้าง จอง / phantom จอง
           ELSE 'writeback_stalled'       -- iHOTEL ไม่เห็นการจอง / no iHOTEL twin
       END                                                        AS kind,
       b.book_no,
       b.book_status,
       j.intent,
       j.status                                                   AS job_status,
       (EXTRACT(EPOCH FROM (now() - j.created_at)) / 60)::bigint   AS age_minutes,
       b.book_checkin,
       b.book_hold_expires_at,
       (SELECT rn.room_no
          FROM ht_booking_rooms br
          JOIN ht_rooms_new rn ON rn.room_id = br.br_room_id
         WHERE br.br_book_id = b.book_id
         ORDER BY rn.room_no LIMIT 1)                              AS room_no
  FROM writeback_jobs j
  JOIN ht_bookings b ON b.aggregate_id = j.aggregate_id
 WHERE j.status <> 'done'                                  -- 'done' = MSSQL commit landed
   AND j.created_at <= now() - make_interval(mins => 10)    -- LOYALTY_WRITEBACK_STALL_ALERT_MINUTES
   AND b.book_channel = 'loyalty'
 ORDER BY j.created_at
 LIMIT 200;
```

> `j.status <> 'done'` ไม่ใช่รายชื่อสถานะที่แย่ — ตั้งใจให้สถานะใหม่ที่เพิ่มมาในอนาคต**ขึ้นรายงาน**
> ไว้ก่อน ไม่ใช่เงียบหาย โดยเฉพาะ `in_progress` ซึ่งคือเคสที่ worker ตายกลางทาง
> — Written as "not `done`" rather than an allow-list so a future status defaults to *appearing*.
> `in_progress` matters most: a worker that dies mid-claim leaves the row claimed forever.

### 4.2 `sweep_lag`

เงื่อนไขเดียวกับตัวกวาด hold (`repository::channel::expired_hold_ids`) เป๊ะ ๆ ทั้งสี่บรรทัด
— All four clauses of the expiry sweep's own input query.

```sql
SELECT b.book_no,
       b.book_status,
       b.book_hold_expires_at,
       (EXTRACT(EPOCH FROM (now() - b.book_hold_expires_at)) / 60)::bigint AS age_minutes,
       b.book_checkin,
       COALESCE(b.book_deposit_amount, 0)::float8                          AS deposit_amount
  FROM ht_bookings b
 WHERE b.book_channel = 'loyalty'
   AND b.book_status = 'pending'              -- เฉพาะ hold ที่ยังไม่จ่าย / unpaid holds only
   AND b.book_hold_expires_at IS NOT NULL
   AND b.book_hold_expires_at < NOW()
 ORDER BY b.book_hold_expires_at
 LIMIT 200;
```

> `book_status = 'pending'` เป็นเงื่อนไขเดียวกับ `release_hold` — ทำให้แถวนี้**ไม่มีทาง**เป็นแขกที่
> จ่ายเงินแล้ว / the same guard `release_hold` uses, so this query can never list a guest who has
> paid. Do not relax it to "non-terminal".

### 4.3 `unlinked_checkin`

ใช้เงื่อนไขทับซ้อนเดียวกับ `FREE_ROOM_PREDICATE` ฝั่งเช็คอิน
— The check-in overlap leg of `repository::channel::FREE_ROOM_PREDICATE`, verbatim.

```sql
SELECT b.book_no,
       b.book_status,
       ci.cin_no,
       ci.cin_checkin_time,                          -- เวลาไทยแบบ naive / naive local Thai
       rn.room_no,
       COALESCE(b.book_deposit_amount, 0)::float8 AS deposit_amount
  FROM ht_bookings b
  JOIN ht_booking_rooms br ON br.br_book_id = b.book_id
  JOIN ht_rooms_new     rn ON rn.room_id     = br.br_room_id
  JOIN ht_checkins      ci
    ON ci.cin_status <> 'cancelled'
   AND (ci.cin_room_id = br.br_room_id OR EXISTS (
           SELECT 1 FROM ht_checkin_rooms cr
            WHERE cr.cr_cin_id  = ci.cin_id
              AND cr.cr_room_id = br.br_room_id))    -- ห้องของคณะพักหลายห้อง / multi-room stays
   AND ci.cin_checkin_time::date < b.book_checkout
   AND COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date > b.book_checkin
 WHERE b.book_channel = 'loyalty'
   AND b.book_status IS DISTINCT FROM 'cancelled'
   AND ci.cin_book_id IS NULL                        -- ← ช่องว่าง B7a / the B7a gap
   AND b.book_checkin  <= (now() AT TIME ZONE 'Asia/Bangkok')::date + 1
   AND b.book_checkout >  (now() AT TIME ZONE 'Asia/Bangkok')::date
 ORDER BY ci.cin_checkin_time, b.book_no
 LIMIT 200;
```

### 4.4 `deposit_divergence`

```sql
SELECT b.book_no,
       b.book_status,
       b.book_checkin,
       COALESCE(b.book_deposit_amount, 0)::float8 AS deposit_amount,
       (SELECT rn.room_no
          FROM ht_booking_rooms br
          JOIN ht_rooms_new rn ON rn.room_id = br.br_room_id
         WHERE br.br_book_id = b.book_id
         ORDER BY rn.room_no LIMIT 1)              AS room_no
  FROM ht_bookings b
 WHERE b.book_channel = 'loyalty'
   AND COALESCE(b.book_deposit_amount, 0) > 0
   AND b.book_checkin >= (now() AT TIME ZONE 'Asia/Bangkok')::date
   AND b.book_checkin <= (now() AT TIME ZONE 'Asia/Bangkok')::date + 1
   AND b.book_status IS DISTINCT FROM 'cancelled'
 ORDER BY b.book_checkin, b.book_no
 LIMIT 200;
```

> `IS DISTINCT FROM` ไม่ใช่ `<>` เพราะ `book_status` เป็น NULL ได้ — ถ้าใช้ `<>` การจองที่สถานะ
> เป็น NULL จะหายไปจากทั้งสองฝั่ง / `book_status` is nullable, and `<>` would silently drop a
> NULL-status booking from both sides of the test.

### 4.5 จุดบอดที่ SQL ข้างบนมองไม่เห็น / The blind spot none of the above covers

รายงานและ Track F5 ผูกอยู่กับ **แถวใน `writeback_jobs` ที่มีอยู่จริง** ทั้งคู่ ซึ่งแปลว่า
การจองแอปที่มีห้องแต่**ไม่มีแถว job เลย** จะไม่ขึ้นที่ไหนเลย ตามทฤษฎีเกิดไม่ได้ (การ enqueue อยู่ใน
transaction เดียวกับการ insert) แต่ถ้าอยากตรวจด้วยมือ:
— Both this report and the F5 alert anchor on a job row that **exists**, so a roomed loyalty
booking with **no `writeback_jobs` row at all** is invisible to both. It should be unreachable —
the enqueue is in the same transaction as the canonical insert — but to check by hand:

```sql
SELECT b.book_no, b.book_status, b.book_checkin, b.created_at
  FROM ht_bookings b
 WHERE b.book_channel = 'loyalty'
   AND EXISTS (SELECT 1 FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id)
   AND NOT EXISTS (SELECT 1 FROM writeback_jobs j WHERE j.aggregate_id = b.aggregate_id)
 ORDER BY b.created_at DESC
 LIMIT 50;
```

ควรได้ 0 แถวเสมอ ถ้าไม่ใช่ ให้แจ้งวิศวกร — นี่คือบั๊ก ไม่ใช่งานหน้าเคาน์เตอร์
— This should always return zero rows. If it does not, escalate to engineering: that is a bug,
not a desk task.

---

## 5. ทำไมต้องใช้เงื่อนไขซ้ำของเดิม / Why every predicate is borrowed

รายงานที่นิยามคำว่า "ยังไม่ถึง iHOTEL" หรือ "หมดอายุ" ต่างจากตัวที่**ทำงานจริง** จะแย่กว่าไม่มีรายงาน
เลย เพราะมันสอนให้เคาน์เตอร์ไม่เชื่อการแจ้งเตือน ("แจ้งเตือนมาแต่ในรายการเช้าไม่มี")
— A report that defines "not applied" or "expired" differently from the machinery that *acts* on
those rows is worse than no report: it teaches the desk to distrust the alert.

| ชนิดแถว / Kind | ยืมเงื่อนไขมาจาก / Predicate borrowed from |
|---|---|
| `writeback_stalled` | `scheduler::sync::fetch_stalled_loyalty_writebacks` (Track F5) |
| `legacy_hold_orphan` | เดียวกัน แยกด้วย `intent` + `book_status` / same query, split by intent |
| `sweep_lag` | `repository::channel::expired_hold_ids` (ตัวป้อนของการกวาดทุก 5 นาที) |
| `unlinked_checkin` | ขาเช็คอินของ `repository::channel::FREE_ROOM_PREDICATE` |
| `deposit_divergence` | ข้อตกลง B7 ใน `docs/loyalty-channel.md` |

ค่าคงที่ `WRITEBACK_APPLIED_STATUS` (`'done'`) และ `DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES` (`10`)
ถูก **import** มาจาก `scheduler::sync` ไม่ได้พิมพ์ซ้ำ — แก้ที่ตัวแจ้งเตือน รายงานจะขยับตาม
— `WRITEBACK_APPLIED_STATUS` and `DEFAULT_LOYALTY_WRITEBACK_STALL_MINUTES` are **imported** from
the tripwire, not re-spelled, so changing the alert moves the report with it. The two
`repository::channel` predicates live in private consts the report may not edit, so they are
restated with a unit test pinning the literals
(`the_sweep_predicate_literals_match_the_sweep`).

---

## 6. ปุ่มปรับ / Tuning

| Var | ความหมาย / Meaning | ค่าเริ่มต้น |
|---|---|---|
| `LOYALTY_WRITEBACK_STALL_ALERT_MINUTES` | อายุที่ถือว่า writeback ค้างจริง ใช้ร่วมกับการแจ้งเตือน F5 / age at which an unapplied writeback counts as stalled — **shared with the F5 alert** | `10` |

ตัวเดียวเท่านั้น และ**ตั้งใจให้ใช้ร่วมกัน** — ถ้าปรับ ทั้งรายงานเช้าและการแจ้งเตือนจะขยับพร้อมกัน
ซึ่งคือสิ่งที่ต้องการ / One knob, deliberately shared: tuning it moves the morning list and the
Slack alert together, which is the point. ต่ำกว่า ~6 นาทีจะเริ่มขึ้นงานที่ยัง retry ปกติอยู่ /
below ~6 minutes you will list healthy retries — see
[`writeback-leg-degraded.md`](writeback-leg-degraded.md) §7 for the sizing argument.

ช่วงมองล่วงหน้าของมัดจำ (วันนี้ + พรุ่งนี้) และเพดาน 200 แถวต่อชนิด เป็นค่าคงที่ในโค้ด
(`DEPOSIT_HORIZON_DAYS`, `MAX_ROWS_PER_KIND`) ไม่ใช่ env — ถ้าได้ผลลัพธ์เต็ม 200 แถว แปลว่ามี
ปัญหาระดับระบบ ต้องใช้ runbook ไม่ใช่รายการที่ยาวขึ้น / the deposit horizon and the per-kind cap
are code constants, not env: a four-figure result means something systemic is broken and the desk
needs the runbook, not a longer list.

---

## Related

- [`writeback-leg-degraded.md`](writeback-leg-degraded.md) — การแจ้งเตือน Track F5 ที่ยิงสด ๆ
  ตอนกลางคืน สำหรับแถวชนิดเดียวกับ §2.1 / the live Track F5 alert for the same rows as §2.1,
  including the escalation ladder (§4) and the 02:00 responsibilities (§6).
- `docs/loyalty-channel.md` — สัญญาของช่องทางแอป: hold TTL 2 ชม., ทำไม hold ถึงเขียนลง iHOTEL ทันที,
  มัดจำที่ไม่ถูกมิเรอร์, การเช็คอินสองทาง (B7a) / the channel contract.
- hf-tasks **B8** (`direct-booking-designs/b8-overbooking-analysis.md`) §4 — checklist **L6**,
  the line this runbook and its route implement, and the other controls (**L2** last-room guard,
  **L3** serialized pick→create, **L4** late-slip status filter, **L5** client timeout +
  idempotency key, **L7** the F5 tripwire) that this one does **not** replace.
- `docs/coexistence/sync-incident-log.md` — ที่บันทึกเหตุการณ์ที่กระทบแขก / where to log anything
  that reached a guest.
