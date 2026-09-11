# ADR 0002 — Indefinite iHOTEL coexistence (no planned decommission)

**Status:** Accepted — 2026-06-26. Ratified by the project owner.
**Supersedes:** the *decommission-as-goal* framing only — ADR 0001 "Phase 8 / 9
(transition / decommission)" note, and `docs/architecture.md` §"State C — DECOMMISSIONED (only our app)"
(State C as an end-state) and `docs/architecture.md` §"8. Migration roadmap" "**∞ (dormant)**"
(the "∞ Decommission" target). It does **not** change any
shipped code or the finished Phase 0–7 work.

## Context

The original plan (ADR 0001; `docs/architecture.md` §"State C — DECOMMISSIONED (only our app)"
and `docs/architecture.md` §"8. Migration roadmap") framed the legacy MSSQL +
iHOTEL .NET app as something we would eventually **decommission**: coexist
during a transition (State B), then turn the legacy stack off (State C / "Phase
8/9"). Every design choice was made "decommission-ready."

That machinery is now **built and live** (Phases 0–7 ✅): canonical PG as source
of truth, the `bin/writeback.rs` outbox→MSSQL adapter, the `bin/sync.rs` Change
Tracking watcher, multi-site HF Hotel + HF Ville, and the drift-reconcile safety
net. HF Hotel already runs as true two-way coexistence — both iHOTEL and our app
write it, and the bidirectional sync keeps them consistent.

The owner has decided iHOTEL will **not** be decommissioned. The two systems
coexist indefinitely.

## Decision

1. **Permanent co-existence is the target end-state.** iHOTEL and our app run
   side by side indefinitely (State B is the *steady state*, not a transition).
   The CT-watcher + writeback sync run **permanently**; the `legacy_mirror`
   schema persists; `legacy_*_id` reference columns remain load-bearing, not
   merely historical.

2. **Both apps are co-equal writers on BOTH sites** (HF Hotel + HF Ville).
   Consistency is maintained by the existing bidirectional loop:
   our app → canonical PG → writeback → site MSSQL, and iHOTEL → site MSSQL →
   Change Tracking → canonical PG. This is exactly how HF Hotel already operates.

3. **Decommission (State C) is retained as a dormant capability, not a goal.**
   The clean layering + decommission boundary + env off-switch
   (`WRITEBACK_ENABLED=false`, `LEGACY_SYNC_ENABLED=false`) stay documented and
   exercisable — they are good engineering and a cheap insurance policy — but
   there is **no roadmap item, date, or intent** to use them. Do not remove the
   capability; do treat it as off-the-table operationally.

## Remaining work (coexistence completion)

The one gap to full two-way coexistence is **HF Ville writes from our app.**
Today the canonical *read* endpoints are branch-aware (`hfville → ville_pool`),
but the *write* endpoints (`create_checkin`, `checkout`, `extend`,
`change_room`, `pos_sale`, `create_booking`, `update_room_status`, …)
unconditionally target `new_pool` (HF Hotel). `/v2` therefore runs **view-only
for HF Ville** as a stopgap (gated to avoid misrouting a Ville write into the
HF Hotel DB).

To complete coexistence:
- **(a)** Branch-route the write endpoints (`hfville → ville_pool`), mirroring
  the read handlers.
- **(b) Prerequisite:** verify the writeback worker propagates Ville canonical
  writes → **Ville** MSSQL (per-site env). Without this, new-app Ville writes
  would land in `hotelville` canonical but never reach iHOTEL → divergence.
- **(c)** Lift the `/v2` Ville view-only restriction once (a)+(b) hold.

Until (a)+(b) ship, the `/v2` view-only gate stays — it is now an *interim*
state on the path to co-equal Ville writes, not a permanent design.

## Reconciliation with the finished plan

| Finished (Phases 0–7, ✅) | Status under this ADR |
|---|---|
| Layering, outbox/writeback, CT sync, multi-site, drift-reconcile | **Unchanged** — this IS the coexistence machinery; it now runs indefinitely rather than "until decommission." |
| `docs/architecture.md` §"8. Migration roadmap" "**∞ (dormant)**" row | **Reframed** to a dormant capability (decision 3), not a target. |
| ADR 0001 "Phase 8/9 transition→decommission" | **Superseded** — Phase 8 becomes "complete HF Ville co-equal writes"; there is no Phase 9 decommission. |
| Decommission-readiness as a design driver | **Reframed** as a retained safety property, not a goal. |

## Consequences

- The roadmap's terminal state is "steady-state coexistence," not "legacy off."
- HF Hotel SS2022→SS2025 parity, `sa` rotation, and Ville network isolation
  (ADR 0001 deferred items) remain relevant — they're now *permanent-operations*
  hardening, not pre-decommission cleanup.
- Decommission stays a one-env-flip capability should the decision ever change.
