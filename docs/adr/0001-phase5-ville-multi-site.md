# ADR 0001 — Phase 5 (Ville) multi-site topology decisions

**Status**: Accepted
**Date**: 2026-04-29
**Decides**: how the same backend stack will run for HF Hotel and HF Ville simultaneously, given Ville's MSSQL upgrade today (SS2005 → SS2025 Express RTM) made the Phase 5 CT architecture feasible at Ville for the first time.

## Context

Until 2026-04-29, HF Ville's MSSQL was SQL Server 2005 — no Change Tracking, no `tiberius` support, no `MERGE`. The only Ville sync path was `bin/ville_sync.rs` (FreeTDS hash-polling, 4 tables, 90 s poll cadence) running on the Ville jumpbox and pushing to a `ville` schema in HF Hotel's central PG.

Today the Ville MSSQL was upgraded to SQL Server 2025 Express RTM (`17.0.1000.7`, host `<ville-db-host>`, db `HOTEL`). Schema is identical to HF Hotel's legacy DB. The Phase 5 CT-based real-time sync stack is now feasible at Ville with the same backend binary.

This ADR locks the six topology decisions that drive Phase 5 (Ville) implementation. Decisions came out of two parallel sub-agent analyses (Plan + codebase-auditor) on 2026-04-29 and were ratified in conversation with the project owner.

## Decision summary

| # | Question | Decision | Rationale |
|---|---|---|---|
| 1 | PG topology | **Per-DB**: one PG cluster, two databases (`hotelnew` + new `hotelville`) | `migrations/pg/013_legacy_ct_state.sql:14` enforces `id BIGINT PRIMARY KEY DEFAULT 1 CHECK (id = 1)` — single-row by design. Per-DB inherits this naturally; per-schema or site-column adds weeks of schema rewrites. Each site gets its own row/copy for free. Backups per site without filtering. Receptionist data never co-mingled. |
| 2 | Where Ville stack runs | **Central on evergreen, MSSQL via WG** | One deploy host, one CI runner, one Slack channel. ~50 ms RTT is invisible at 1 s CT poll cadence. Co-locating Ville compute on `desktop-0be5led` (same host as MSSQL) increases blast radius of a single host failure. |
| 3 | Tailscale subnet routing on `desktop-0be5led` | **Skip** — use WG path `<wg-self> → <wg-router> (DNAT) → <ville-mssql-host>:1436` | Mooted by the WG path actually deployed today. Smaller surface area. `desktop-0be5led`'s Tailscale daemon is fragile (Windows userspace, sleeps with RDP logout). |
| 4 | Frontend | **Extend existing `BranchContext`** | `contexts/BranchContext.tsx:5-32` already models `'hfhotel' \| 'hfville' \| 'all'`. Backend `AppState` (`hotel-backend/src/routes/mode.rs:85-298`) already exposes `ville_pool: Option<PgPool>`. Replace the old "ville_pool reads from `ville` schema in central newdb" with "ville_pool points at the new Ville DB." Same UX. |
| 5 | `ville_sync` retirement | **Strangler**: shadow mode → divert pool → drop after 1 wk parity | (i) Deploy new stack alongside `ville_sync` in shadow; (ii) divert backend `ville_pool` to new Ville DB; (iii) verify parity for 1 week using `scheduler/sync.rs` `DiffOnly` mode; (iv) stop `ville_sync`, remove `deploy/hfville/`, drop the `ville` schema. Zero downtime. |
| 6 | Backups / DR | **Parameterise `scripts/backup-db.sh` per site, daily `pg_dump`** | Reuse existing infra. Two cron entries on evergreen, retention 30 d, restore drill quarterly. Express-source MSSQL is upstream, NOT a backup target. |

## Detailed reasoning per decision

### Q1 — Per-DB PG topology (the load-bearing choice)

Three options were on the table:

- **Per-DB (chosen):** one cluster, two logical databases. Zero schema changes. Two backend deploys, one per site, each with its own `DATABASE_URL`. Migrations re-applied verbatim against both. CT watcher / writeback / API are naturally partitioned by which DB their `DATABASE_URL` points at. The `legacy_ct_state.id=1` CHECK constraint stays valid because each DB has its own row.
- **Per-schema:** sounds cheap. But every existing migration assumes `public` and every `sqlx::query!()` is unqualified. Retrofitting `search_path` works, but `legacy_ct_state` becomes "two rows, one per schema, single-row CHECK is now wrong" — touches a lot. Saves one PG container, costs weeks.
- **Site column on every table:** worst of both. Every query gains `WHERE site = $1`, every UNIQUE becomes `(site, ...)`, every event payload gets a `site` field. Fanout risks: a bug leaks one site's bookings into the other's UI.

If a cross-site query is ever needed (corporate dashboard), it can come from a third place — not by collapsing the topology.

### Q2 — Central on evergreen vs on-Ville host

The codebase auditor pushed back on "central" with: tiberius/bb8 today has no `connection_timeout` and no circuit-breaker, so a 60-second WG tunnel drop turns into a 60-deep bb8 acquire queue with no alert.

Resolution: **keep central**, but mitigate at two layers:
- **Network layer**: WG path `evergreen → the edge router DNAT → MSSQL` instead of Tailscale-direct-to-Windows-desktop. WG is purpose-built network gear (RouterOS kernel WG, no userspace daemon, no sleep, no Windows update interference). Today the WG link has 87 GiB throughput history at audit time and 11+ d jumpbox uptime. (See `docs/runbook-sync.md` and `ville_constraint.md` for the WG path.)
- **App layer**: task #68 adds `MSSQL_PORT` env, bb8 `connection_timeout` + `acquire_timeout`, tiberius socket timeout. Application-level circuit breaker that's independent of network.

Re-evaluate Q2 only if Ville's WAN proves unreliable (>1 outage/quarter that the WG path can't survive).

### Q3 — Tailscale subnet routing

`desktop-0be5led` advertises no subnet routes. Two options were considered: enable `<ville-lan>` advertising via Tailscale admin, OR rely on the WG path through the jumpbox. WG was chosen because `<ville-lan>` is the **an internal network segment** (per HF Ville network repo `vlan-analysis.md`) and exposing the whole subnet is contrary to the network's isolation intent. The WG path scopes routing to `<ville-mssql-host>/32` — only MSSQL, nothing else.

### Q4 — Frontend already wired

Discovery during planning: `BranchContext` and `Sidebar.tsx` already model the three-way branch selector. Backend already routes `params.branch=hfville` to a separate pool. Today that pool reads from the `ville` schema fed by `ville_sync`; tomorrow it reads from the new Ville DB fed by CT watcher. UI doesn't change.

### Q5 — Strangler pattern for `ville_sync` retirement

The DiffOnly mode of `scheduler/sync.rs` is purpose-built as a parity check between the cache layer and the canonical source. It's the natural verifier during the strangler window. Drop only after a clean week.

### Q6 — Backups via existing tooling

`scripts/backup-db.sh` already exists for the single-site case. Adding a `--site` flag (or `SITE_ID` env) and two cron entries is incremental. Restoring tested quarterly per `docs/runbook-dr.md` (to be created in task #79).

## Implications for the codebase

The auditor's findings map cleanly to follow-up tasks:

| Auditor finding | Task # |
|---|---|
| Hardcoded MSSQL port 1433 in `db/pool.rs:26` | #68 |
| Password fallback `"12345678"` in `config.rs:34` | #68 |
| No tiberius/bb8 timeouts | #68 |
| No SITE_ID in alerts/logs/healthcheck | #69 |
| Single global writeback fingerprint in `writeback/fingerprint.rs:257` | #70 |
| `legacy_ct_state` single-tenant by design | resolved by Q1 (per-DB) — no code change needed |
| `legacy_sync_status.table_name` PK has no site discriminator | resolved by Q1 |
| `ht_reconcile_log` no site column | resolved by Q1 |
| `writeback_channel` global LISTEN | resolved by Q1 (separate PG = separate channel namespace) |
| CI/CD hardcoded single deploy target | #72 |

All of the schema-shaped concerns dissolve under the per-DB topology choice. The remaining work is purely application code (env wiring, observability, fingerprint per site).

## What this ADR does NOT decide

- **Phase 5.5 (Ville mirror feature)** roll-out timing is intentionally deferred. Phase 5 (Ville) gives canonical-sync parity. Phase 5.5 (Ville) adds the 6 mirror tables and view-only history panels. Splitting them lets us validate the multi-site infrastructure against the more battle-tested Phase 5 scope first. Tracked as tasks #80-#82.
- **Phase 8 / 9 (transition / decommission)** of the legacy MSSQL — this ADR doesn't pre-judge them, but every Q1 / Q2 choice is deliberately compatible with both eventual states.
- **HF Hotel SS2022 → SS2025 upgrade** for version parity — defer 30+ days post-Phase 7 cutover. No functional gap today.
- **Move MSSQL off the network segment (an internal segment) at Ville** — out of scope here. Isolation is enforced at the edge router forward chain + WG `/32` AllowedIPs scope. Worth scheduling separately as ops hardening.
- **`sa / 12345678` password rotation** at both sites — out of scope. Audit flagged it; schedule when next maintenance window opens.

## References

- Plan agent output (in-conversation, 2026-04-29) — sub-phases 7a-7i
- Codebase auditor output (in-conversation, 2026-04-29) — 5 cutover blockers + Express-edition risk profile + multi-site assumption survey
- `docs/architecture.md` — target architecture, ops state machine
- `docs/runbook-sync.md` — operator playbook, shadow-mode 2-day trap, retention overflow
- Memory: `ville_constraint.md` — Ville post-upgrade state and WG path
- HF Ville network ops repo: `~/hfville-network/credentials-and-access.md`, `vlan-analysis.md`, `wireguard-plan.md`, `fix-log.md` (step 11 logs the WG MSSQL allow rules)
