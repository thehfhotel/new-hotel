# iHOTEL room-status palette — decompile extraction (2026-07-09)

> Extracted from `evergreen:/home/nut/new-hotel/legacy/Hotel-2018- V.1.45/_decompiled_clean/iHOTEL2025/FormRoomMain.cs`
> for ADR 0003 / issue #227. This doc is the IN-REPO provenance for the hues used in `lib/v2/status.ts`
> and `app/v2/v2.css` — the decompile itself lives off-repo on evergreen.

## States (BackColor1 = dominant hue)

| State | Color | Glyph/text | Source |
|---|---|---|---|
| vacant (ว่าง) — vacant-clean | #90EE90 (Color.LightGreen; gradient to Honeydew #F0FFF0) | tile text "ว่าง"; usage-rank number bottom-right | FormRoomMain.cs:3166-3169 |
| dirty (รอ ทำความสะอาด) — vacant-dirty / waiting for cleaning | #FFE4B5 (Color.Moccasin; gradient to FloralWhite #FFFAF0) | tile text "รอ ทำความสะอาด" | FormRoomMain.cs:3156-3159 |
| cleaning-in-progress (กำลัง ทำความสะอาด) | #FFFFFF (Color.White; gradient to FloralWhite #FFFAF0) | countdown text "เหลือ N นาที" from Room_Clean_Time; auto-reverts to vacant at 0 | FormRoomMain.cs:3161-3164 (+4966-4998) |
| occupied (เข้าพัก) | #FFE4E1 (Color.MistyRose; gradient to OrangeRed #FF4500) | guest name + due-out date text; coins icon if balance due; vat7 icon if tax invoice; an4_* emoticon marks multi-room siblings | FormRoomMain.cs:3146-3149 (multi-room shortcut 3126-3129) |
| reserved (จอง) — HT_Rooms.room_book pointer | #FFFF00 (Color.Yellow; gradient to LightYellow #FFFFE0) | tile text "จอง\n{room_book_name}\nเวลา : {room_book_time}" | FormRoomMain.cs:3116-3119 (+5000-5003) |
| maintenance (ซ่อม) — Room_Manternace='yes', overrides all | #A9A9A9 (Color.DarkGray; gradient to WhiteSmoke #F5F5F5) | tile text "ซ่อม" | FormRoomMain.cs:3131-3134 (+5005-5009) |
| checkout-pending (ยังไม่ได้ Check-Out) — room_use='yes' with no active checkin | #00BFFF (Color.DeepSkyBlue; gradient to AliceBlue #F0F8FF) | tile text "ยังไม่ได้ Check-Out\nออกวันที่\n{dd-MM-yy HH:mm}" | FormRoomMain.cs:3151-3154 (+4880-4936) |
| arrival-pending (ยังไม่เข้าพัก) — checkin exists, guest not yet in room | #FFFFE0 (Color.LightYellow; gradient to Snow #FFFAFA) | guest name + checkin no. text | FormRoomMain.cs:3121-3124 |
| occupied-monthly (รายเดือน, Cin_type=2) | #FF8C00 (Color.DarkOrange; gradient to Linen #FAF0E6) | tile text "## รายเดือน ##" | FormRoomMain.cs:3136-3139 (+4831-4833) |
| occupied-hourly (รายชั่วโมง, Cin_type=1) | #4682B4 (Color.SteelBlue; gradient to White #FFFFFF) | countdown text "รายชั่วโมง เหลือ …" | FormRoomMain.cs:3141-3144 (+4826-4830) |

## Full extraction notes (rendering model, gradients, flag→status priority, tile anatomy, legend)

```
SOURCE: evergreen:"/home/nut/new-hotel/legacy/Hotel-2018- V.1.45/_decompiled_clean/iHOTEL2025/FormRoomMain.cs" (decompiled VB.NET WinForms, DevComponents DotNetBar). Line refs are into that file.

RENDERING MODEL — each tile is a PanelEx with a vertical two-stop gradient (Style.GradientAngle=90): BackColor1 = top stop, BackColor2 = bottom stop. In the states[] list, color = BackColor1 (the dominant hue); the gradient partner (BackColor2) is given in the glyph-free note per state below. The color switch matches on the Thai R_STATUS string prefix and exists twice, byte-identical: method_0 (FormRoomMain.cs:3116-3169) and SETButton_Notclear (FormRoomMain.cs:3469-3523).

GRADIENT PAIRS (BackColor1 → BackColor2):
- reserved "จอง": Yellow #FFFF00 → LightYellow #FFFFE0
- arrival-pending "ยังไม่เข้าพัก": LightYellow #FFFFE0 → Snow #FFFAFA
- occupied "เข้าพัก" (and multi-room shortcut via IndexOf("  ")!=-1 at :3126): MistyRose #FFE4E1 → OrangeRed #FF4500
- maintenance "ซ่อม": DarkGray #A9A9A9 → WhiteSmoke #F5F5F5
- occupied-monthly "รายเดือน": DarkOrange #FF8C00 → Linen #FAF0E6
- occupied-hourly "รายชั่วโมง": SteelBlue #4682B4 → White #FFFFFF
- checkout-pending "ยังไม่ได้ Check-Out": DeepSkyBlue #00BFFF → AliceBlue #F0F8FF
- dirty "รอ ทำความสะอาด": Moccasin #FFE4B5 → FloralWhite #FFFAF0
- cleaning-in-progress "กำลัง ทำความสะอาด": White #FFFFFF → FloralWhite #FFFAF0
- vacant "ว่าง": LightGreen #90EE90 → Honeydew #F0FFF0

FLAG → STATUS DERIVATION (refresh loop, lines ~4795–5000; later assignments override earlier — priority is bottom-up):
1. default obj3 = "ว่าง" (vacant) — :4795
2. active checkin row (View: room_status) → "เข้าพัก" / "ยังไม่เข้าพัก"; "Check Out" → back to "ว่าง" — :4865–4877; Cin_type=1 appends "รายชั่วโมง เหลือ …", Cin_type=2 appends "## รายเดือน ##" (:4826–4833) — these substrings win the color match over plain occupied
3. HT_Rooms.room_use='yes' while status is vacant/booked → "ยังไม่ได้ Check-Out\r\nออกวันที่…" (checkout-pending / overdue due-out) — :4880–4936
4. HT_Rooms.room_clean='yes' → "รอ ทำความสะอาด"; if Room_Clean_Time set → "กำลัง ทำความสะอาด เหลือ N นาที" countdown; on expiry the form itself UPDATEs Room_Clean='no', logs to HT_Housewife, powers off lights — :4966–4998
5. HT_Rooms.room_book <> '' (reserved pointer) → "จอง\r\n{room_book_name}\r\nเวลา : {room_book_time}" — :5000–5003
6. HT_Rooms.Room_Manternace='yes' → "ซ่อม" (maintenance) — LAST assignment, overrides everything — :5005–5009
Note: "ปิดปรับปรุง" (closed-for-renovation) is counted into the legend "waiting" bucket at :5043 but is never assigned anywhere in this form — dead branch, do not build it.

TILE ANATOMY / GLYPHS (method_0, :2838–3400): header PanelEx = room number (Tahoma 12 bold, theme colors, NOT state-colored); center PanelEx = state gradient + LabelX with the full Thai R_STATUS text (word-wrapped, centered — status text IS the glyph: countdown minutes for cleaning, "ออกวันที่ dd-MM-yy HH:mm" due-out for occupied, booker name+time for reserved); footer PanelEx = room type + details; small bottom-right PanelNum = usage-rank number ("ลำดับการใช้งาน"). Overlay icon strip (bottom-left FlowLayoutPanel "Panel_Nofi"): Resources.coins = outstanding balance >0 (tooltip ยอดค้างชำระ) :2903; Resources.coins_delete = overpayment <0 (ยอดเงินเกิน) :2916; Resources.email = unread room note (HT_Room_SMS) :3009; Resources.lightbulb = room power ON :3036; Resources.vat7 = tax invoice issued (total_price_vat>0) :3060. Multi-room stays (cin_room_all with ≥2 spaces) get a per-group emoticon background image top-right of the center panel (Resources.an4_2…an4_39, indexed per stay) so sibling rooms are visually linked — :3172–3260.

LEGEND / SUMMARY STRIP (per-room-type counts): designer labels S1–S6 at :2333–2422 — S1 "ประเภทห้อง" #D7D7D7, S2 "ว่าง" rgb(122,221,122) #7ADD7A, S3 "ไม่ว่าง" rgb(225,98,98) #E16262, S4 "จอง" rgb(225,225,0) #E1E100, S5 "รอ" rgb(225,162,98) #E1A262, S6 "คืน" (due-out) rgb(0,225,225) #00E1E1, all ForeColor #0000C0. Runtime rows (SET_STATUS :5397+) use White / PaleGreen #98FB98 / rgb(255,128,128) #FF8080 / Yellow #FFFF00 / rgb(255,192,128) #FFC080 / Aqua #00FFFF, totals row fore #0000A2. Bucketing at :5030–5100: booked→S4, occupied→S3, cleaning(both)+renovation→S5, checkout-pending→S6, vacant→S2. So the "flat" legend hues are the saturated versions of the tile gradients — good candidates for the v2-quality single-hue rendering the Status colors glossary entry calls for.

Companion forms FormRoomMainClean.cs / FormRoomMainKichen.cs reuse the same flag logic (housekeeping/kitchen boards) — not extracted here but same palette family. All states/colors above are from FormRoomMain.cs, the reception room board referenced by FEATURE_MAP §3.2.
```