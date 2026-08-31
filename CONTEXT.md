# HF Hotel PMS

Hotel property-management system for two sites (HF Hotel + HF Ville), coexisting permanently
with the legacy iHOTEL app (ADR 0002) — both apps write both sites, kept consistent by
bidirectional sync. This glossary pins the domain language; deeper references are
`docs/architecture.md` and `docs/legacy-app/COMPAT_CHEATSHEET.md`.

## Language

### UX

**iHOTEL-anchored UX**:
The reception UI contract: iHOTEL's workflows, vocabulary, and visual familiarity are the
baseline; deviations are sanctioned only when they reduce steps or automate work, and the
iHOTEL-familiar path must always remain available.
_Avoid_: "UX parity" (ambiguous — implies cloning), "modern redesign" (implies free deviation)

**Spatial room grid**:
The room board rendered at each room's receptionist-arranged `room_x`/`room_y` position
(mirrored from iHOTEL `HT_Rooms.Room_X/Room_y`) — the default home-base view. The
floor-grouped list is a secondary toggle view.
_Avoid_: "room list", "floor view" (that's the toggle, not the anchor)

**Layout-edit drag**:
Dragging a room tile to rearrange the board itself, inside the explicit จัดผัง mode (ungated,
like iHOTEL). The board is SHARED: each drop immediately writes `HT_Rooms.Room_X/Room_y`
(neighbor-derived pixels that round-trip into the same grid cell; drop-on-occupied swaps the
two tiles), so iHOTEL's board rearranges too. Placing rooms from the unplaced row is part of
the mode. Distinct gesture/mode from **guest-move drag**; while active, tap-select and
guest-move are disabled.
_Avoid_: "canonical-only layout" (forks the two boards), batch "save layout" (a step iHOTEL
doesn't have)

**Guest-move drag**:
Dragging an occupied tile onto a vacant one to run the existing room-change flow (issue #225).
The default drag on the spatial grid; layout-edit requires an explicit mode.
_Avoid_: conflating with layout-edit drag

**Reception shift-loop**:
The surfaces bound by iHOTEL-anchored UX — boundary rule: anything reception does with a guest
or cash at the counter. Room grid, check-in (incl. edit/cancel), checkout + settle,
payments/receipts (incl. walk-up receipts) + edit-payment, deposits (booking + in-checkin),
booking create/convert, round open/close, POS-to-room. Back-office surfaces (reports, masters,
admin, sync monitor, verification hub) stay v2-native.
_Avoid_: "the whole app" (back-office is deliberately unbound)

**Verified unused**:
A legacy feature with zero (or long-dead) rows in BOTH sites' live legacy DBs, reclassified out
of the build scope with the evidence dated in the gap matrix. Current members (2026-07-09):
credit sales/debtors, standalone deposit slips, coupons (dormant since 2025-07).
_Avoid_: "missing feature" (implies it should be built)

**Improvement invariant**:
What every shortened/automated flow must preserve to stay iHOTEL-anchored: the iHOTEL entry
point, the Thai vocabulary, every previously-typed value visible + editable on one confirm
screen before commit, byte-identical printed artifacts and legacy writeback rows, and no
capability reachable in iHOTEL becoming unreachable here.
_Avoid_: "silent automation" (auto-commit without the confirm moment)

**Status colors**:
Room-state hues are legacy vocabulary, not design assets: the hue that means each state
(vacant / occupied / dirty / reserved / maintenance) is iHOTEL's, extracted from the decompile,
rendered in v2-quality shades — one color language across every surface that shows room state.
_Avoid_: per-surface palettes, v2-native tones for room state

### Housekeeping

**Room signal**:
A canned, room-scoped notice between reception and maids on the housekeeping surface —
strictly one of the fixed types below, never free text (anything unanticipated stays a phone
call). Always about ONE room; broadcast to the other role at that room's branch (no personal
addressing — the first responder acts). Lifecycle: open → acked (by name) → done (by name);
the sender may cancel while open; a signal stays visible until done, whatever the day. A
maid's เสร็จแล้ว report auto-completes that room's open cleaning-urgency signals
(ทำห้องนี้ก่อน, แขกเช็คเอาท์แล้ว), recorded as completed by her report. Desk→maid types:
ทำห้องนี้ก่อน, แขกขอผ้าเพิ่ม, งดทำห้องนี้, แขกเช็คเอาท์แล้ว, ขอเช็คห้อง. Maid→desk types:
ลูกค้ายังอยู่ในห้อง, พบของลืมในห้อง, มีของหาย, มีของเสียหาย.
_Avoid_: "chat", "message" (imply conversation and threading — ruled out), "notification"
(that's a delivery mechanism, not the domain object)

**Room-check (ขอเช็คห้อง)**:
The checkout coordination signal — fired manually by the desk when a guest asks to check
out (never auto-fired by the PMS checkout itself), the most urgent signal in the system
because the guest is waiting at the counter. Unlike other signals its completion is an
ANSWER, not a bare tap: the maid inspects and answers **เคลียร์** (all good — settle now) or
one or both of มีของหาย / มีของเสียหาย, which then also stand as guest-accountability
signals for the desk to act on before settling. The answer completes the check.
_Avoid_: "inspection checklist" (no per-item checklist exists — the answer is the maid's
judgment), auto-firing on checkout open (many checkouts settle without an inspection)

**On-duty maid**:
A maid who clocked in today at a given branch's fingerprint device and has not clocked out —
the attendance system's answer to "who is physically working here right now." The day's punch
device decides her branch, beating her stored home branch (a maid covering the other property
is on-duty THERE). The only audience an escalation may ever reach; nobody on-duty ⇒ no
escalation at all (the desk phones).
_Avoid_: "branch roster" (home-branch membership — includes day-off and gone-home maids),
"active maid" (is_active is an employment flag, not a shift state)

**Guest-accountability signal**:
The maid→desk room signals that mean "this room's guest may owe for something — know it
BEFORE they settle": มีของหาย (an expected room item is missing, e.g. รีโมททีวี, towels
gone with the guest) and มีของเสียหาย (an item damaged). Distinct from **ขาดผ้า** (the maid
is short of linen and needs a restock brought to her) and from แจ้งซ่อม (a maintenance work
order in the housekeeping ops app — if damage also needs a technician, แจ้งซ่อม is still
filed there separately).
_Avoid_: folding these into ขาดผ้า (restock ≠ missing item), "damage report" routed to the
repair queue (the desk, not the technician, is the audience)

### Coexistence

**Legacy-stale signal**:
The `pg_notify('legacy_stale_signal', ...)` fired when a writeback commits into legacy
MSSQL at a moment iHOTEL's room grid cannot pick up on its own (ADR 0006) — an ephemeral
wake-up hint, not a durable domain event. Distinct from `RESYNC_EVENT`/`"refresh"`
(`routes/events.rs`), which means "refetch our own UI's data," not "iHOTEL is stale."
_Avoid_: "refresh event" (collides with the existing `RESYNC_EVENT` name/meaning)

**Grid staleness latch**:
The one-toast-per-stale-episode state machine gating reception's notification (ADR 0006
§5): opens on the first writeback while iHOTEL can't auto-refresh, absorbs further
writebacks in the same episode as a silent counter (no repeat toast), closes on a
reception-invoked refresh or ~65s of iHOTEL continuously holding foreground. Exists to
prevent alarm fatigue, not to surface every individual write.
_Avoid_: "alert on every writeback" (defeats the latch; retrains reception to ignore toasts)

**Reception-invoked refresh**:
The receptionist clicking "Refresh iHOTEL now" on the toast (ADR 0006) — the only refresh
path this system ever triggers. iHOTEL's own Refresh button (`ButtonX3`) is what actually
runs; our app never calls it, drives it, or automates it on her behalf.
_Avoid_: "auto-refresh iHOTEL", "push to iHOTEL" (both imply we drive the vendor app
unasked — exactly what ADR 0006 rules out)

**Folio lock token**:
`HT_CheckIn_H.Cin_Work_number` — iHOTEL's per-folio optimistic-lock token, written on folio
LOAD by `Module1.GET_WORK_NUMBER` in five reception forms (FrmEditDate, FrmPayAdd,
FrmPayAddPro, FrmCheckIn, FrmCheckOut) and re-checked in each form's `SAVE_EDIT()` before it
writes; on mismatch the form shows `มีการแก้ไข … จากเครื่องอื่น`, closes, and discards the
receptionist's typed work. (In FrmEditDate a save-confirm dialog runs first, so answering
"No" returns before the token is ever read.) `extend_stay` bumps it deliberately so a stale iHOTEL form
fails closed rather than overwriting our totals with its pre-extend literals. Advisory only —
the check and the save are not in one transaction. Detail: `COMPAT_CHEATSHEET.md` §7.4.
_Avoid_: "TM.30 batch number" (the 2026-04 spike's disproven inference), "work number",
"async batch job"

### Sync (established elsewhere, recorded for vocabulary)

**Sync lag / unconverged**:
A reconcile-log row observing two hashes not yet converged — normally self-heals within a tick.
_Avoid_: "drift", "divergence" (imply durable states needing operator action; see CLAUDE.md
vocabulary note)

**Dropped legacy change**:
A change made in iHOTEL that never reached canonical and never will — the event was consumed
without being applied, and the legacy change history needed to redeliver it has since been
discarded. Distinct from **sync lag**: no tick will fix it, and the affected record is simply
absent from our app (a booking in this state is invisible to occupancy, arrivals and forecast
even though reception can see it in iHOTEL). Repairable only by re-reading the legacy row and
re-applying it.
_Avoid_: "sync lag" (implies it self-heals — the 2026-07-27 alert used that word for 16 days
and it is why nobody acted), "drift" (implies both sides exist and disagree)
