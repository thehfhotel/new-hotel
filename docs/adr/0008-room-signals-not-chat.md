# ADR 0008 — Room signals over chat; LINE as door and escalation valve, never the pipe

**Status:** Accepted — 2026-09-01 (grill session with the owner; vocabulary pinned in
`CONTEXT.md` §Housekeeping the same day).
**Scope:** how reception and maids communicate on the housekeeping surface — the domain
shape (canned room signals, not conversation) and the delivery architecture (own rails,
with LINE OA push confined to one bounded escalation). Does **not** design the build
(schema, endpoints, SSE wiring, the HF ID escalation endpoint) — that is separately-scoped
work this ADR licenses.

## Context

Maids work from the LINE-opened `/hk` surface (HF ID identity, `housekeeping` grant);
reception works the v2 desk surface on the PC and, since 2026-09, also holds a read-only
`/hk` viewer (`reception` grant). Coordination between them today is corridor shouting and
phone calls: "ทำห้องนี้ก่อน", "แขกขอผ้าเพิ่ม", and the checkout ritual — guest waits at the
counter while a maid inspects the room and reports เคลียร์ / มีของหาย / มีของเสียหาย.

The obvious builds are (a) a chat box between the roles, and (b) delivering messages as
LINE pushes — the maids are literally on LINE, and HF ID already holds every maid's LINE
user id and a working staff-OA push path (used by rich-menu provisioning).

The load-bearing constraint is that **LINE meters exactly one thing: messages the OA
sends, counted per recipient.** The Thai free tier is small (~200/month floor as of 2026 —
verify in OA Manager; the next tier is ฿1,200/month). Everything else this estate uses
LINE for — the rich menu, opening `/hk` in the LINE browser, silent HF ID login, menu
provisioning — consumes zero messages, forever. Chat-over-LINE at even modest volume
(~30 messages/day × ~3 recipients ≈ 2,700/month) is 13× the free tier from week one;
chat-in-app would still need per-message notification, threading, read state, and a
moderation surface, and its content is invisible to the room board.

## Decision

1. **The domain object is the room signal, not the message** (`CONTEXT.md` §Housekeeping):
   canned-only (no free text — anything unanticipated stays a phone call), about exactly
   one room, broadcast to the other role at that room's branch. Lifecycle
   open → acked-by-name → done-by-name; sender may cancel while open; visible until done;
   a maid's เสร็จแล้ว report auto-completes the room's open cleaning-urgency signals.
   ขอเช็คห้อง is the checkout coordinator: its completion is an *answer* — เคลียร์, or
   มีของหาย / มีของเสียหาย, which then stand as guest-accountability signals the desk
   resolves before settling.
2. **Routine delivery rides our own rails and consumes zero LINE messages**: PG rows,
   SSE to reception's board and (extended) to the open `/hk` page with a sound cue, the
   existing poll as fallback. A fully-handled signal — including a whole checkout check —
   touches LINE zero times.
3. **LINE OA push is an escalation valve with a closed definition**: only ขอเช็คห้อง,
   only when still unacked after 2 minutes, one push per signal (never repeated), sent
   only to **on-duty maids at that branch** (today's clock-in at that branch's device and
   not clocked out — resolved by HF ID, which owns attendance; nobody on-duty ⇒ no push at
   all, the desk phones). A monthly push counter with a hard stop (~150) makes silent
   quota burn impossible. No other signal type pushes; there are no per-message pushes.

## Consequences

- The signal table doubles as the audit record behind guest charges (ของหาย/เสียหาย at
  settle time) — one reason reversing into a thread model later means migrating rows that
  carry money-adjacent meaning, not just UI.
- Three surfaces render signals as room state (maid list/room page, desk board, checkout
  screen), not scrollback; the ack answers "who's on it" and existing room state answers
  "is it done".
- Clock-in discipline becomes load-bearing for escalation: a maid who forgot to clock in
  is invisible to the push. Accepted deliberately (option "on-duty, no fallback") — the
  failure mode is today's behavior (the desk phones), not a new one.
- Escalation volume is the health metric: sustained escalations mean maids aren't acking
  in-app. The remedies are operational (acking habits), commercial (฿1,200/month tier), or
  a later Web Push build (`/hk` as a subscribing PWA) — none change this architecture.
- Anyone tempted to add a free-text field or a per-message LINE push should read the
  Context section first: both were considered and rejected, not overlooked. The canned
  vocabulary is an app-side constant list (the ขาดผ้า precedent) — extending it is the
  sanctioned cheap change.

## Alternatives considered

- **Free-text chat (in-app)**: maximum flexibility; rejected for threading/read-state/
  moderation weight, per-message notification pressure, and content the board cannot
  render. The phone call remains the escape hatch and costs nothing to keep.
- **Chat or per-signal delivery over LINE push**: zero-build delivery; rejected on quota
  arithmetic (above) — it converts every routine tap into metered spend and makes the
  free tier the system's ceiling.
- **Canned + optional note** (recommended during the grill): rejected by the owner in
  favor of strictly canned — a free-text note is the thin end of the chat wedge.
- **Web Push in v1**: `/hk` is not a PWA and maids don't run the managers' portal; the
  subscription build is real work for a gap the 2-minute escalation already covers.
  Deferred, not rejected.
