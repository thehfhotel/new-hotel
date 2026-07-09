# iHOTEL functions not yet replicated — verified gap matrix (2026-07-09)

> Produced by an 8-agent cross-reference of `docs/legacy-app/FEATURE_MAP.md` (~132 forms) +
> `REPORTS_INVENTORY.md` against the actual app surface (`app/`, `components/`,
> `hotel-backend/src/routes|service|writeback/recipes`), then adversarially verified in both
> directions (claimed-missing-but-exists / claimed-covered-but-missing). Flag states verified
> against **production `gh` repo variables**, not compose defaults.
> Framing: coexistence is permanent (ADR 0002) — "replicated" means *reception could do the
> function in our app*, with legacy mirrored where the function writes data. Complements
> `docs/adoption/feature-gap-audit.md` (earlier audit; some of its flag rows are now stale).

## 0. The flag frontier — formerly-dark features that are LIVE in prod now

These were widely believed dark (and appear dark in `docker-compose.yml` local defaults) but the
production repo vars are ON. Any gap analysis citing compose defaults overstates the gaps:

| Feature | Flag | Live since |
|---|---|---|
| Auth enforcement (login required) | `AUTH_ENABLED` + `AUTH_REQUIRED` | 2026-05-10 |
| HF Ville writes (co-equal second site) | `HFVILLE_WRITES_ENABLED` | 2026-06-29 |
| Server-authoritative checkout total | `CHECKOUT_SERVER_TOTAL_ENABLED` | 2026-06-29 |
| Booking-form server validation surfacing | `BOOKING_VALIDATION_ENABLED` | 2026-06-29 |
| Cashier round open/close → `HT_Round_Bill` writeback (+ RoundControl UI) | `ROUND_WRITEBACK_ENABLED` | 2026-06-30 |
| Guest ID/passport image mirror → `Tb_Save_Image` | `GUEST_DOCUMENT_STORAGE_ENABLED` | 2026-07-01 ¹ |

Consequences: payments are **no longer parasitic on an iHOTEL-opened round** (we open/close our
own, mirrored); Ville mutations no longer 403; the check-in registration feature (Thai ID chip,
passport MRZ, doc photos) is merged to master and live-iterated against prod iHOTEL (2026-07-02).

¹ Flag is on and the mirror verified live (CH26-005957), but Task #8 is nominally still mid-test:
the generated Thai-ID card's TEXT quality (font/placement vs iHOTEL's original) is a deferred
must-fix before the permanent-enable decision.

Still dark by explicit choice: `TM30_COMPANION_WRITEBACK_ENABLED=false` (set OFF 2026-07-02 after
the companion echo-loop incident; convergent-delta fix shipped dark in e23a5ad, retest needed),
`NOTES_WRITEBACK_ENABLED` (no repo var), plus the unwired cash-entry recipe (§2).

## 1. Genuine gaps — daily front-desk workflows (highest blocking power)

| # | Function (iHOTEL analog) | Status | Detail |
|---|---|---|---|
| 1 | **Multi-room check-in** (FrmCheckIn J1/J2) | missing | `service/checkin.rs` still hard-rejects >1 room. The walk-in writeback recipe already fans out N rooms (Track B4) — remaining work is lifting the service guard + UI, not recipe design. Groups/families force a switch to iHOTEL. |
| 2 | **Edit an open check-in** (FrmCheckIn_EditOnly via ClickUSE) | missing | No general edit of guest name/ID/price on an open stay; only extend / change-dates(backend-only) / change-room. Daily correction workflow. |
| 3 | **Check-in cancel** (void a wrong check-in) | partial | Service + writeback recipe (`checkin_cancel.rs`) exist; **no HTTP route is mounted** — unreachable from any UI. Small wiring job. |
| 4 | **Edit payment row** (FormEditPay) | missing | Only refund + re-take; mis-keyed tender/amount can't be corrected in place. |
| 5 | ~~Standalone deposit slips~~ (FrmAddDep → `HT_Deposit`) | **verified unused** | `HT_Deposit` = 0 rows EVER on both sites (checked live 2026-07-09) — won't build (ADR 0003). Booking-deposit take IS live; in-checkin `Cin_Room_Dep` deposits ARE used and covered. Deposit reports likewise dropped. |
| 6 | Deposit refund on legacy-origin folios | partial | Silently WARN-no-ops until `cr_legacy_ds_id` backfill; most current stays are legacy-origin. |
| 7 | **Click-to-reserve** from room board / calendar (ClickBook) | missing | Our boards/calendar are read-only; booking always detours through the form. |
| 8 | Post-check-in companion add/delete mirror (`HT_CheckIn_Other_People`) | dark-flag | Check-in-time companions ARE mirrored (walk-in recipe). The post-check-in delta mirror is built but `TM30_COMPANION_WRITEBACK_ENABLED=false` — iHOTEL-printed RR.4 under-reports occupants added later in our app. |
| 9 | Sticky-note mirror (`HT_Room_SMS`/`HT_EMP_SMS`) | dark-flag | `/v2/notes` full-stack, but notes invisible to the iHOTEL shift until `NOTES_WRITEBACK_ENABLED` flips — defeats shift handover while both apps run. |
| 10 | Hourly continue-time pricing (`HT_ContinueTime` + FrmSETTimeContnue rules editor) | missing | Late-checkout/hourly extension charging rules never replicated; extensions can misprice vs house rules. |
| 11 | Settle-time confirms (overcharge, VAT-over threshold, bill rounding) + electricity charge | missing | iHOTEL's checkout guardrail dialogs have no analog. |
| 12 | Shorten-stay / change-dates UI | partial | `PUT /change-dates` exists backend-only; no frontend surface. |
| 12b | **Drag-and-drop guest move on the room grid** (added 2026-07-09, user ask) | missing | Beyond-iHOTEL-parity UX: drag an occupied room tile onto a vacant one to run the room change. The whole backend already exists (change-room service + `HT_Changed_Room` writeback + room-change slip, live via RoomActionSheet) — this is a pure frontend interaction layer on `/v2/rooms`: drag source = occupied tile, drop target = vacant(+clean?) tile, confirm dialog reusing the existing change-room flow + slip print. Sibling of #7 click-to-reserve: together they make the grid directly manipulable instead of menu-driven. |

## 2. Money & back-office gaps

| # | Function | Status | Detail |
|---|---|---|---|
| 13 | ~~Credit sales + debtor management~~ (`HT_Bill_Debt_*`) | **verified unused** | `HT_Bill_Debt_H` = 0 rows EVER on both sites (checked live 2026-07-09) — won't build (ADR 0003); debt reports + credit receipt layouts dropped with it. Revisit only if the business starts extending credit in iHOTEL. |
| 14 | Price/debt change history viewer (`HT_Log_Debt`, FrmPriceHistory) | missing | Audit trail for disputed charges. |
| 15 | Petty-cash outbound writeback (`TB_Pay_History`) | partial | `/v2/cash` UI is canonical-only; recipe `cash_entry.rs` exists but deliberately unwired (byte-shape unverified) → iHOTEL income/shift reports under-report our entries. |
| 16 | Cash category tree CRUD (`TB_SET_MyType2/2_2/3`) | missing | Read-only in our app. |
| 17 | POS sales/refunds bypass `HT_Receipt_*` (VAT attribution) | partial | Open finance decision from the 2026-06 audit; blocks trusting either app's tax report alone. |
| 18 | Walk-up POS receipt void/refund | missing — **elevated to shift-loop** | Void only covers folio sales. Walk-up receipts are a LIVE daily flow: `HT_Receipt_H` 4,318 rows/12m at HF Hotel (latest same-day), 105 at Ville (checked 2026-07-09) — also raises the stakes on #17 VAT-attribution scope. |
| 19 | Legacy gapless `HT_INVOICE.INV_NO` allocation | partial | Our tax invoices carry PG-only numbers; Thai tax-document practice expects the gapless legacy series. |
| 20 | Booking invoice (FormBookingInvoice / ReportBookingINV) | missing | Corporate/agent prepay invoice from a booking; our `/v2/invoice` is per-check-in only. |
| 21 | Invoice notes (`HT_Invoice_Note`) | missing | Minor; part of ClickUSE dispatcher parity. |
| 22 | Roomless (waitlist) bookings → iHOTEL | partial | Writeback skips no-room bookings; waitlist made in our app can be double-booked from iHOTEL. |
| 23 | Booking pre-order products (`HT_Book_Pro`) outbound | partial | Inbound mirror only. (Our booking→check-in product transfer IS live, default-on.) |
| 24 | Unpaid-booking reminder parity (notify-day + per-item actions) | partial | v2 bell exists; no per-booking notify window or act-on-item (view/mute/convert). |

## 3. Reports & printed documents

| # | Function | Status |
|---|---|---|
| 25 | Receipt/invoice print layout family (~20 .rpt variants: 58/80mm thermal, merchant copies, VAT short-bill, other-charges VAT/non-VAT, AR) | partial — we have a handful of A4/thermal templates |
| 26 | Printable single daily summary (ReportDays) | partial — rosters/financial/analytics cover the data, no one-page daily |
| 27 | Deposit reports (received / refund / booking-deposit — ReportDep family) | missing |
| 28 | Coupon report (ReportCupon58/80) | **dormant** — see #34, dropped (ADR 0003) |
| 29 | Housekeeping reports (cleaning counts by employee, cleaning log) | missing |
| 30 | Cancellation reports (`HT_Rooms_Cancel`) | missing |
| 31 | Loyalty/visit-count reports (CustStay, UseCount) + room-change period report | missing |
| 32 | Repair history mirror + report (`HT_Rooms_Repair`) | partial — kanban is PG-only, no mirror/auto-offline/report |
| 33 | Registration card 2-up/3-up + barcode variants | partial — 1-up bilingual live (deliberate deviations) |
| 34 | Coupon redeem UI + per-night auto-generation | **dormant** — `HT_Cupon` dead since 2025-07 (8 rows/12m HF Hotel, 0 ever Ville; checked live 2026-07-09). Keep built endpoints, build nothing further (ADR 0003) |

## 4. Masters, admin, roles

| # | Function | Status | Detail |
|---|---|---|---|
| 35 | **Settings editor** (FrmSettings / `TB_SETTINGS` ~100 knobs) | missing | No analog beyond `app/admin/{sync,users}`; fragments live in other items (print layouts, confirms, notify-day) but the management function itself is unreplicated. Many knobs are per-station hardware config that may be won't-port. |
| 36 | Per-button permission grid + editor (TB_MRP_PERMISSION, FrmPermission) | partial | Auth IS enforced (since 2026-05-10) with 4 seeded roles (admin/receptionist/housekeeper/cashier), but: no admin UI to edit grants (SQL + restart), coarser than iHOTEL's per-button grid, and **no role-restricted UI shell** — a housekeeper login sees full reception UI (mutations blocked server-side only). Kitchen role absent entirely. |
| 37 | Idle auto-logout (TimerMouse / `Module1.AutoLogout`) | missing | Shared-terminal session security; known (branch `feat/auth-residual-idle-stamp` unshipped). |
| 38 | Customer-type master + Order_Up/Down price-override rules | missing | Rate ladder editable; type CRUD + ripple/override semantics stay in iHOTEL. |
| 39 | Customer create legacy mirror + cascade rename | partial | Update writeback live; standalone create canonical-only; iHOTEL's 6-table cascade rename not replicated. |
| 40 | Product create legacy mirror + product-type master | partial | Created products don't appear in iHOTEL's POS picker (documented TODO). |
| 41 | Room-type master writeback + sales-rep master/reports (`HT_SET_Sale`) | partial / missing | Rate matrix writes back; room-type + sales-rep don't. |
| 42 | `HT_Log` audit-trail parity + log viewer | missing | New-app actions never appear in iHOTEL's FormLog; minor (we have structured logs + audit tables) but a dual-app visibility gap. |
| 43 | frmTimeTable (room × time occupancy grid) | missing | Minor read-only viewer. |

## 5. Hardware & messaging (decide: port or won't-port)

| # | Function | Status |
|---|---|---|
| 44 | Room power relay control (COM-port) | missing — needs a hardware-bridge decision |
| 45 | Cash drawer kick | missing — no ESC/POS kick anywhere |
| 46 | SMS sending (manual, debt reminders, log) | missing — vendor gateway (kpsystem) likely won't-port; capability decision open |

## 6. Deliberately not replicated (won't-port)

Licensing/registration/kill-switch/MAC whitelist; self-updater (→ CI/CD); DDNS updater (→
cloudflared/Tailscale); "delete all data" factory reset; DB-server picker + MS Access backend;
bulk room-status date fixer (→ reconcile sweep); UI theme switching; FormLog trapdoor (→
structured logs/audit tables); KP SMS gateway settings; branch master (site scoping is
connection-level); most per-station `TB_SETTINGS` hardware knobs; dead/duplicate .rpt layouts.

## 6b. UX anchoring (ADR 0003 — added 2026-07-09 after grill-with-docs)

Reception adoption requires **iHOTEL-anchored UX** on the shift-loop (boundary: anything
reception does with a guest or cash at the counter). Baseline = iHOTEL workflows + Thai
vocabulary + spatial board + status-color hues; deviations only toward fewer steps / more
automation, under the improvement invariant (see `CONTEXT.md`). Gate = the zero-training test
in reception-coordinated live tests. New build items this adds:

| # | Item | Notes |
|---|---|---|
| U1 | **Spatial room grid** as default home view | Render tiles at `room_x`/`room_y` (already synced into `ht_rooms_new`); floor-grouped becomes a toggle. Guest-move drag (#225) is the default gesture; layout-edit drag behind an explicit mode. |
| U2 | **Status-color extraction + adoption** | Pull iHOTEL's room-state palette from the decompile; adopt the hues (v2-quality shades) on every surface showing room state. |
| U3 | **Anchoring audit of existing shift-loop flows** | Dialog-by-dialog: check-in modal vs FrmCheckIn, checkout vs FrmCheckOut, payment, booking — field visibility per the improvement invariant + click-count vs iHOTEL recorded per flow. |

## 7. What IS solidly replicated (for orientation)

Room board + stay lifecycle (walk-in single-room check-in with 7-table legacy writeback, check-in
from booking incl. Book_Pro transfer, extend, room change + slip, whole-stay and per-room
checkout, folio); payments/receipts/refunds incl. deposit refund (app-origin); booking CRUD with
live writeback + search + calendar + confirmation print; POS sale-to-room with stock decrement +
walk-up sale (both sites); registration/compliance (Thai-ID chip, MRZ, doc capture + live
Tb_Save_Image mirror, registration card print, RR.4/TM.30 export with hash-logged
non-repudiation); housekeeping clean/dirty with live recipes; maintenance status flip;
customer master (update writeback); rooms + rate matrix writeback; coupon issue/print + redeem
endpoint; cashier rounds (sync + open/close writeback + reports); rosters/financial/analytics
reports; notes; auth (password/CF/NFC). Plus non-iHOTEL additions: SSE live refresh, sync
monitor, verification forms, amenity inventory, maintenance kanban.

## 8. Ranked short-list — what actually blocks "reception could go a full shift without iHOTEL"

1. **UX anchoring foundation** (U1 spatial grid + U2 colors) — the board reception sees first.
2. **Multi-room check-in** (#1) — recipe ready, guard + UI remain.
3. **Open-stay corrections**: edit check-in (#2), mount check-in cancel (#3), edit payment (#4).
4. **Deposit refund on legacy-origin folios** (#6) — slips dropped as verified unused.
5. **Walk-up receipt void** (#18) + VAT-attribution decision (#17) — live 4.3k/yr flow.
6. **Print/document family** (#25) — thermal receipt variants reception hands to guests daily.
7. **Flip the two social mirrors** (companions #8, notes #9) so the iHOTEL shift sees our data.
8. **Anchoring audit** (U3), then rules parity that misprices: continue-time (#10), settle confirms (#11).
9. Role-shell UX (#36) before handing devices to housekeeping/kitchen.

Dropped from the list per live-DB evidence (2026-07-09): credit sales/debtors, standalone
deposit slips, coupon reports/auto-gen — see "verified unused" rows above + ADR 0003.

Everything in §3 reports and §4 masters can trail adoption; §5 hardware needs a one-time decision.

UX bundle worth doing together when touching the room grid: #7 click-to-reserve + #12b
drag-and-drop guest move — same surface (`/v2/rooms` board), same "grid as the primary
control" idea, and both reuse existing backend flows.
