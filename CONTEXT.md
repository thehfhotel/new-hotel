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
Dragging a room tile to rearrange the board itself (iHOTEL FormRoomMain semantics; persists
`room_x`/`room_y`). Distinct gesture/mode from **guest-move drag**.

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

### Sync (established elsewhere, recorded for vocabulary)

**Sync lag / unconverged**:
A reconcile-log row observing two hashes not yet converged — normally self-heals within a tick.
_Avoid_: "drift", "divergence" (imply durable states needing operator action; see CLAUDE.md
vocabulary note)
