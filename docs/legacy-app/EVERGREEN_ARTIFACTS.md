# Evergreen Off-Repo Artifacts

Pointer to the full iHOTEL2025 working folder + reverse-engineering artifacts
that live on `evergreen` and are deliberately NOT in this repo. The analytical
output (the `.md` files in this directory) is what's in git; the raw vendor
binaries and the full decompile that produced them stay off-repo for IP
safety.

## Location

```
evergreen:/home/nut/new-hotel/legacy/
├── README.md                    ← inventory + how to resume
└── Hotel-2018- V.1.45/          ← exact mirror of the dev-machine working folder
    ├── HOTEL.exe                Original obfuscated .NET Reactor binary
    ├── HOTEL-cleaned.exe        de4dot output (de-obfuscated)
    ├── HOTEL.pdb                Debug symbols (the reason decompile is so clean)
    ├── *.dll, *.rpt             Vendor DLLs (DotNetBar, C1FlexGrid, etc.) + Crystal Reports
    ├── _decompiled/             Initial ilspycmd output (obfuscated)
    ├── _decompiled_clean/       Cleaned ilspycmd output + analysis docs (source for this directory)
    ├── _reference/              Buildable .csproj reference codebase
    ├── _de4dot_src/             Patched de4dot source (already built)
    └── ProgramManage/           Microsoft installers (freely redistributable — SQL Server 2005, CRRedist)
```

Total size: ~165 MB after MS-installer cleanup (was 302 MB pre-cleanup;
`ProgramManage/` + `CRRedist2008_x86.msi` were removed on 2026-05-11 since
they're freely downloadable from Microsoft).

## Why off-repo

Three layers of IP risk if these are committed to a repo that goes public:

1. **HOTEL.exe / HOTEL.pdb / HOTEL-cleaned.exe** — proprietary build of the
   commercial iHOTEL2025 product. Distributing decompiled-or-not violates the
   vendor's EULA.
2. **Vendor DLLs** — `DevComponents.DotNetBar2.dll` (commercial UI library),
   `C1.Win.C1FlexGrid` (commercial grid), `Microsoft.Office.Interop.Excel`,
   `KPThaiNationalIDCard.dll`. Each has its own redistribution rules; we
   don't have the right to ship them.
3. **Decompiled source** (`_decompiled*/`, `_reference/`) — same EULA
   concern as the binary; even cleaned-up `de4dot` output is a derivative
   work.

The `.md` analysis docs in this directory's parent are FACTS DERIVED from
those artifacts (table layouts, write sequences, business rules) and don't
themselves contain vendor code.

## Git history status (public-flip context)

`legacy-reference/` was originally committed to this repo with the binaries
listed above. The history has since been **rewritten** to drop them:

- **`9cfc8ac` (2026-05-05)** — working-tree deletion (commit message
  acknowledged the history-rewrite todo)
- **Phase 6.5 history rewrite (2026-05-10)** — `git filter-repo` rewrite of
  master from `c44b7c7` → `c6a82e1`. Dropped `legacy-reference/` (466 files /
  ~26 MB) from all of history; also scrubbed rotated secrets. Force-pushed.
  Local repo size 31 MB → 14 MB after re-clone.

**Remaining blocker for public flip**: the safety tag
`pre-history-rewrite-2026-05-10` on origin still points at `c44b7c7` (the
pre-rewrite head). That tag keeps the old `legacy-reference/binaries/*`
blobs reachable in pack objects. Verified 2026-05-11 — large blobs reachable
only via this tag:

| Path | Size |
|---|---|
| `legacy-reference/binaries/HOTEL.exe` | 7.0 MB |
| `legacy-reference/binaries/HOTEL-cleaned.exe` | 6.2 MB |
| `legacy-reference/binaries/HOTEL.pdb` | 2.1 MB |
| `legacy-reference/vendor/DevComponents.DotNetBar2.dll` | 3.5 MB |
| `legacy-reference/vendor/Microsoft.Office.Interop.Excel.dll` | 1.1 MB |

When the rewrite is ratified (no rollback needed within the agreed window),
delete the tag locally + on origin and run `git gc --aggressive --prune=now`
to drop the unreachable blobs. After that, a fresh `git clone` will not see
any vendor binaries in pack history. Mirror copy at
`~/backup/new-hotel-pre-rewrite-2026-05-10.git` (31 MB) keeps offline access
for true emergencies.

## Reproducibility

If you need to re-derive any of the analysis docs:

```bash
ssh evergreen
cd '/home/nut/new-hotel/legacy/Hotel-2018- V.1.45'
# Reference codebase is buildable:
cd _reference && dotnet build
# Decompile from scratch:
ilspycmd HOTEL-cleaned.exe -o /tmp/new-decompile
```

The full Claude Code conversation that produced the original analysis
(including all the reasoning) is also preserved on evergreen at:

```
evergreen:/home/nut/.claude/projects/-home-nut-new-hotel-legacy/
├── e9d17934-01ce-4288-a34b-91f4411acef4.jsonl  ← main session (1.8 MB)
├── 962526a1-561e-45a7-8d5e-719ea4ce2b62.jsonl  (148 KB)
├── fae3ac60-4a41-4519-8f0c-b74768655e92.jsonl  (1.8 KB)
└── memory/                                      ← MEMORY.md + project notes
```

To resume the session on evergreen: `cd /home/nut/new-hotel/legacy && claude --resume`.

## Risk: single-host loss

The artifacts above exist on exactly one machine (`evergreen`). A disk failure
or filesystem corruption today loses the entire 165 MB of analysis work —
including the JSONL transcripts that document the reasoning. Off-host
encrypted backup to cold storage (S3, NAS, etc.) is recommended but **not
yet implemented** as of 2026-05-11. Mitigations to revisit:

- Encrypted tarball pushed to an S3 bucket on a different account
- Synced copy on a NAS in a different physical location
- Periodic `restic` snapshots to B2 / Backblaze
