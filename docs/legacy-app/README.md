# Legacy iHOTEL App — Coexistence Reference

Authoritative reference for resolving discrepancies between this app and the
running legacy Windows app (`HOTEL.exe`, "iHOTEL 2025", VB.NET WinForms,
Hotel-2018 V.1.45). Use this when canonical state in PostgreSQL doesn't match
what receptionists see in iHOTEL — these docs explain exactly which fields the
legacy app reads and writes, in what order, with what conventions.

## Contents

| File | What it is |
|---|---|
| [`COMPAT_CHEATSHEET.md`](COMPAT_CHEATSHEET.md) | **Coexistence contract** — every load-bearing literal, field, and convention. §1 global conventions (Thai_CI_AS varchar, date handling, ID generation). §2 per-table contract (every `HT_*` table). §3 multi-table cascade catalog (walk-in, check-out, booking, cancel, change-room mid-stay, mark dirty/clean, extension, etc.). §8 quirks (Branch column on payments, round-bill gate, etc.). **Read this first.** |
| [`FEATURE_MAP.md`](FEATURE_MAP.md) | Feature inventory grouped by UI screen / `Form*.cs` file, mapped to the DB tables each one touches. Use when you need to know "what does the receptionist clicking X actually do?" |
| [`REPORTS_INVENTORY.md`](REPORTS_INVENTORY.md) | Crystal Report inventory — each `.rpt` file, its purpose, the source query, and which legacy menu/button invokes it. |
| [`SCHEMA.sql`](SCHEMA.sql) | Live schema dump captured from the legacy `db` database. Less complete than the canonical baseline at [`docs/legacy-spike/schema/01-baseline-schema.txt`](../legacy-spike/schema/01-baseline-schema.txt) — that file is authoritative; this one is included for self-contained reference. |
| [`OBFUSCATOR_STUBS_REMOVED.md`](OBFUSCATOR_STUBS_REMOVED.md) | Notes on which `.NET Reactor` obfuscator stubs were removed during the `de4dot` cleanup to produce the reference codebase. Useful only if you need to re-derive the analysis from the binary. |
| [`EVERGREEN_ARTIFACTS.md`](EVERGREEN_ARTIFACTS.md) | Pointer to the proprietary binaries + full decompile that stay off-repo (on evergreen at `/home/nut/new-hotel/legacy/`). |

## Provenance

All docs in this directory were derived from a legal `de4dot` + `ilspycmd`
decompile of `HOTEL.exe` (the production binary running at HF Hotel + HF
Ville). The decompile itself, plus the vendor binaries (`HOTEL.exe`,
`HOTEL.pdb`, Crystal Reports, vendor DLLs), are NOT in this repo — they live
off-repo on evergreen per [`EVERGREEN_ARTIFACTS.md`](EVERGREEN_ARTIFACTS.md)
for IP-safety reasons. Only the analytical artifacts we produced (the `.md`
files here) live in git.

## Authority order

When this folder, the live database, and `docs/legacy-spike/findings.md`
disagree, **trust in this order**:

1. **Live database queries** against production MSSQL (`192.168.100.222:1433`
   for HF Hotel, `192.168.11.51:1436` for HF Ville). Reality wins.
2. **`docs/legacy-spike/findings.md`** — SQL recipes validated against live
   Extended Events captures of receptionists driving the legacy app. This is
   the *observed* behaviour as of 2026-04-24.
3. **This folder** — derived from the decompile. Reflects *intended* behaviour
   per the source code, which may differ from what the production binary
   actually does if the deployed binary is a different build.

## See also

- [`docs/legacy-spike/`](../legacy-spike/) — Extended Events captures + analysis from the original reverse-engineering spike (2026-04-24)
- [`docs/architecture.md`](../architecture.md) — Target architecture, decommission boundary
- [`migrations/legacy-mssql/`](../../migrations/legacy-mssql/) — PK additions + CT enablement applied to the legacy MSSQL
