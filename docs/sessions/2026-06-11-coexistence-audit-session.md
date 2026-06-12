# Session record — 2026-06-11/12: alert triage → June-3 repair → full iHOTEL coexistence audit → remediation

Working session log (Claude Code + Winut). Companion to
[`docs/audits/2026-06-11-ihotel-coexistence-audit.md`](../audits/2026-06-11-ihotel-coexistence-audit.md),
which holds the structured findings; this file records the narrative,
the decisions and their rationale, and the handoff state.

## 1. Where it started: Slack alert triage

Question: the recurring `:warning:` Slack alerts — are they noise?

- **"CT watermark check incomplete" (probe-timeout class): yes, noise.**
  Every instance was the overnight pattern — watermark idle because
  nothing changes, iHOTEL too busy to answer the 5s probe — and
  self-recovered in 1–6 min. Demoted to `:information_source:` with a
  severity-aware cooldown (a confirmed-backlog `:rotating_light:`
  bypasses the cooldown an info page started) and reason-accurate
  RECOVERED wording. Commit `ad1efa8`, deployed same day.
- **"Sync lag unconverged >4h": NOT noise.** It was pointing at real
  durable divergence.

## 2. The June-3 incident (C22209 / R015290)

The unconverged rows were customer **C22209 (Marcel Hadorn)** + booking
**R015290** (room type 512, stay 2027-01-12→15), created in iHOTEL
2026-06-03 21:47 Thai and **lost by the CT watcher** — present in MSSQL,
absent from every PG table, zero events, CT events aged out of the
2-day retention.

- Repaired live with same-value "touch" UPDATEs on the legacy rows
  (zero triggers in the legacy DB → side-effect-free) → fresh CT events
  → mappers upserted both rows → reconcile sweep auto-resolved its two
  ledger rows at 03:18 UTC. Recipe recorded in auto-memory.
- Root cause (confirmed later by two independent audit agents):
  `booking.rs` deferred with `Ok(None)` when its customer wasn't in PG
  yet; the watcher counted that `skipped`, not `errored`, so the
  watermark advanced — and in default global-watermark mode the
  `HT_Book_H` poll advancing also strands the customer's not-yet-polled
  CT row. Both rows gone, no errors. The `resolve.rs` contract doc
  ("next tick re-fires it") was simply false.

## 3. The full audit (`/goal`: "check and verify every aspect")

Five parallel read-only audit agents covered: writeback recipes (17),
CT mappers (12), reconcile/backfill/migrations SQL, cross-cutting
invariants, and FEATURE_MAP × REPORTS_INVENTORY coverage. Verdict:
**conditional pass — coexistence worked partly by accident.** Highlights:

- 4 × P0: the June-3 FK-defer/watermark class (7 silent-drop paths);
  `payment.rs` blindly deleting `HT_CheckIn_Product` without the §6.3
  stock pairing; `extend_stay.rs` zeroing `Total_Price_Product` /
  clobbering `Total_Price_Pay`; `room_change.rs` skipping the §3.17
  caller duties (stale iHOTEL occupancy).
- 6 × P1 incl.: **echo suppression was inert** (`SET CONTEXT_INFO`
  never populates `SYS_CHANGE_CONTEXT`; loop-safety was accidental via
  mapper idempotency — and the naive fix would CREATE June-3-style loss
  because CT coalesces per-PK); customer hard-deletes silently ignored;
  `N'…'` on varchar; fingerprint guard missing 3 written tables.
- Reference repairs: `docs/legacy-app/SCHEMA.sql` was truncated
  (29/61 tables) → regenerated complete from live prod sys.columns;
  CLAUDE.md got the CT-prerequisite-DDL carve-out.

## 4. Remediation (same day) + adversarial re-verification

Two implementation agents (isolated worktrees) fixed every P0/P1;
merges `ac4fda4` (sync) + `90bef62` (writeback). Then an **independent
adversarial reviewer** tried to refute each fix against the RAW spike
captures: all confirmed, zero new violations (its strongest suspect —
`Cin_note=''` in extend-stay — turned out to be verbatim in the raw
capture that findings.md had abridged). Its one legitimate refutation
(transient orphan-recovery lookup error advanced the watermark) was
fixed within the hour (`2b3a9d6`). Verification along the way also
surfaced and fixed: the **coupon legacy-id-reuse poison pill** (Delete
orphans the canonical row; MAX+1 reuse then errors every retry — same
class as the v2.66.3 room-calendar rebind), the Bangkok `pay_date`
fallback, and Conflict-mapping for concurrent enqueues.

## 5. P2 implementation wave (2026-06-12, user-approved "keep going")

- **Writeback intents** (`6ed2018`): `UpdateCustomer` (31-field re-save
  hydrated from canonical; deletes deliberately NOT written back —
  iHOTEL delete = destructive C0000 cascade), `MarkRoomDirty`,
  `SetRoomMaintenance`, + round-bill gate warning (log-only).
- **`HT_Book_Pro` ingestion** (`7c8a5d8`): legacy migration 023 + PG 056
  + `BookProMirrorMapper` + watcher seeds per the 033/050 new-table
  pattern; conversion-transfer TODO documented (needs `B_PRO_ID`→`Pro_no`
  verification).
- **`backfill_customer_legacy_ids` bin** (`7c8a5d8`).
- **Room-FK family closed** (`7e76e9b`) — gated on a live data-quality
  check first (both sites: ZERO orphan room references; PG mirrors
  58/58 + 34/34 rooms): room master mapper now auto-creates unknown
  rooms; calendar/booking-line misses hold the watermark loudly.
- **Latent idempotency bug** (`2453776`): five repeatable intents
  (ModifyBooking/ExtendStay/RoomChange/MarkRoomClean/UpdateRoom) used
  deterministic keys against permanently-retained done-jobs — the
  SECOND occurrence per aggregate would 409. Switched to per-event
  discriminator keys. Rule going forward: **any repeatable-per-aggregate
  intent uses `generate_idempotency_key(&intent, Uuid::new_v4())`.**

## 6. Test-infra lesson (eight files fixed)

The DB-backed integration suite assumed a fresh DB per run; against the
persistent dev PG it flaked constantly. Two recurring shapes, now the
house pattern:
1. Fixture keys need **process-unique residues** (nanos + pid + atomic
   counter, ≥6-digit namespaces, disjoint per-file prefixes) — bare
   `nanos % N` and `rand::<u8>()` collide.
2. Tests sharing a fixture family (singleton rows, fixed PKs, marker
   `LIKE` cleanups) need a **static `tokio::sync::Mutex`** per family.

Final state: **1140 tests, 5/5 consecutive fully-green runs** against a
dirty persistent database; clippy clean.

## 7. Handoff — Winut's action list

1. **Review + `git push`** — 19 local commits (`ad1efa8..f627fa1`);
   push deploys via the pipeline.
2. **Apply `migrations/legacy-mssql/023`** at both sites (Sch-M window,
   receptionist-coordinated, same as 020-022).
3. **Run `backfill_customer_legacy_ids --dry-run` → live**, per site,
   post-deploy.
4. **One capture on a product-bearing folio** (payment + receipt-cancel
   + extend-stay in iHOTEL) — settles `Total_Price_vat` accumulator-vs-Σ
   and extend-stay `Total_Price_Net` semantics.
5. **POS/refund receipt scope with finance** — until decided, iHOTEL
   `ReportSaleVat`/`ReportTax` are wrong for new-app sales/refunds.
6. **`SYNC_PER_TABLE_WATERMARK=true`** at a monitored window (hfville
   canary first). Defense-in-depth only now.
7. Optional: `HT_Products_Price`/`HT_Receipt_Ds` ingestion (023/056 is
   the template); Book_Pro conversion transfer; periodic `cargo clean`
   (target/ hit 18 GB and filled the disk mid-session).

## Commit index (this session, oldest first)

| Commit | What |
|---|---|
| `ad1efa8` | fix(sync): probe-timeout page → informational + severity-aware cooldown (deployed 2026-06-11) |
| `a5b5fa6` | docs(audit): the audit report |
| `7fc8b59` | docs(legacy-app): SCHEMA.sql regenerated from live prod + CLAUDE.md CT-DDL carve-out |
| `a558d82` | refactor(backend): drop vestigial `AppState.legacy_pool` |
| `30d2d72`/`ac4fda4` | fix(sync): FK-defer silent-drop class + customer deletes (migration 055) + inert echo filter removed |
| `70914fd`/`90bef62` | fix(writeback): payment cart-clear, extend-stay totals, room-change §3.17, N-strips, fingerprint |
| `3a8d358` | fix(sync): coupon id-reuse poison pill + Bangkok pay_date + Conflict-on-enqueue |
| `d96a4ed` | test: suite determinism on persistent DB (wave 1) |
| `4311a84` | docs(audit): remediation status |
| `2b3a9d6` | fix(sync): hold watermark on transient orphan-recovery errors (adversarial-review finding) |
| `7e76e9b` | fix(sync): room-FK family — auto-create + loud misses |
| `7057705`/`7c8a5d8` | feat(sync): HT_Book_Pro ingestion + legacy_id backfill bin |
| `3fd3205`/`6ed2018` | feat(writeback): customer-edit/mark-dirty/maintenance intents + round-bill warning |
| `2453776` | fix(writeback): per-event idempotency keys for repeatable intents |
| `719b4d9` | test: remaining shared-fixture races (wave 2) |
| `f627fa1` | docs(audit): P2 wave completion |
