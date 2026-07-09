# iHOTEL-anchored UX for the reception shift-loop

status: accepted (2026-07-09, grill-with-docs session)

Reception adoption of the new app is gated on receptionists switching mid-shift without
retraining, so we deliberately anchor the reception shift-loop UI to iHOTEL rather than
designing it freely: iHOTEL's workflows, Thai vocabulary, spatial room-board layout
(`room_x`/`room_y`, receptionist-arranged, already mirrored into canonical PG), and status-color
hues (extracted from the decompile) are the baseline. Deviations are sanctioned in exactly one
direction — fewer steps and more automation — under the **improvement invariant**: same iHOTEL
entry point, same vocabulary, every value iHOTEL made reception type stays visible and editable
on one confirm screen before commit, byte-identical printed artifacts and legacy writeback rows,
and no iHOTEL-reachable capability becomes unreachable. Enhancements (e.g. drag-and-drop guest
move, #225) are additive alternates; the familiar path always remains.

**Scope**: the reception shift-loop only — room grid, check-in (incl. edit/cancel), checkout +
settle, payments/receipts (incl. walk-up receipts — verified live: 4,318 HT_Receipt_H rows in
12 months at HF Hotel) and edit-payment, deposits (booking + in-checkin), booking
create/convert, round open/close, POS-to-room. Back-office (reports, masters, admin, sync
monitor, verification hub) stays v2-native.

**Explicit no-s, from live legacy-DB evidence (2026-07-09, both sites)**: credit sales/debtor
management (`HT_Bill_Debt_H` = 0 rows ever) and standalone deposit slips (`HT_Deposit` = 0 rows
ever) are NOT built — reclassified "verified unused"; coupons (`HT_Cupon` dead since 2025-07)
are dormant — existing endpoints kept, no further UI/reports. Revisit only if the business
starts using them in iHOTEL.

**Acceptance gate**: the zero-training test — a receptionist who knows iHOTEL completes the
task in our app without instruction, verified during reception-coordinated live tests;
click/step counts vs iHOTEL are recorded at design time as a check, not the goal.

Considered and rejected: pixel/layout cloning of iHOTEL (fights the /v2 design investment for
no additional adoption gain beyond workflow+vocabulary+spatial+color anchoring), and
efficiency-only parity with free flow redesign (retrains reception per screen — the exact
adoption risk this decision exists to remove).

Vocabulary: `CONTEXT.md` — iHOTEL-anchored UX, spatial room grid, layout-edit drag vs
guest-move drag, status colors, improvement invariant, reception shift-loop.
