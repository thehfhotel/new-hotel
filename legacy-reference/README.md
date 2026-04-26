# Legacy Reference

Reverse-engineering artifacts derived from the original `HOTEL.exe` binary. Use as **complementary reference** to `docs/legacy-spike/` (which is the *authoritative* source — it's based on live SQL Server Extended Events captures of the running app, not just code analysis).

When the two disagree, **trust `docs/legacy-spike/` over this folder.** Live-capture beats code-inferred.

## What's in this folder

```
legacy-reference/
├── analysis/                      Code-derived analysis docs (markdown)
│   ├── _FEATURE_MAP.md            Every form, navigation graph, per-form table touches, end-to-end user journeys
│   ├── _COMPAT_CHEATSHEET.md      Coexistence contract: status literals, multi-table cascades, ID generation, denormalization map (1,901 lines)
│   ├── _REPORTS_INVENTORY.md      All 46 Crystal Reports cataloged with port priority + QuestPDF replacement plan
│   └── _SCHEMA.sql                CREATE TABLE statements for all 61 tables (less complete than docs/legacy-spike/schema/01-baseline-schema.txt — prefer that one)
├── decompiled-source/             Buildable C# reference codebase (~298 .cs files)
│   ├── HOTEL-cleaned.csproj       net8.0-windows project, opens in Rider/VS/VSCode
│   ├── README.md                  Module navigation index, top-5 files to open first
│   ├── src/iHOTEL2025/            The actual source (flat — don't rename, F12 navigation depends on it)
│   └── _archived/                 Old/copy variants kept for reference but excluded from build
├── binaries/                      Original Windows binaries
│   ├── HOTEL.exe                  Original obfuscated build (Dec 2024)
│   ├── HOTEL-cleaned.exe          de4dot output — string literals inlined, ConfuserEx-like protections stripped
│   └── HOTEL.pdb                  Debug symbols (variable names + line numbers — what made the decompile so clean)
└── vendor/                        Commercial 3rd-party DLLs the .csproj references
                                   ⚠️ DotNetBar, C1FlexGrid etc. are paid commercial products — internal use only.
                                   Hotel app references them; they're here so the .csproj loads in IDE.
```

## How this was produced

| Step | Tool | Output |
|---|---|---|
| 1. Decompile | ilspycmd 8.2 | C# source from .NET assembly |
| 2. Detect obfuscator | de4dot 3.1 | Identified as **.NET Reactor** |
| 3. Deobfuscate | de4dot (built from source for .NET 8) | `HOTEL-cleaned.exe` with string literals decrypted |
| 4. Re-decompile | ilspycmd | Clean C# with readable strings + SQL queries |
| 5. Analyze | Multiple Claude Code agent passes | The 4 markdown docs in `analysis/` |

The original app was VB.NET (clues: `Microsoft.VisualBasic.*` imports, `[AccessedThroughProperty]` attributes, `[StandardModule]` on what would be a VB Module). Decompile output is C# — easier to read, but watch for `Operators.CompareString`, `Conversions.ToString`, etc. which are VB-runtime artifacts.

## Useful for the rewrite

- **Looking up exactly what the legacy app does for some action**: open the relevant Form in `decompiled-source/src/iHOTEL2025/` (use `_FEATURE_MAP.md` to find the right form), then trace the `Module1.connect("...")` calls. SQL is plain string concat — fully readable.
- **Validating writeback shape**: cross-reference `_COMPAT_CHEATSHEET.md` cascades against `docs/legacy-spike/findings.md` and the captured `writes.txt` files.
- **Adding a new writeback flow**: find the Form that performs the action in the legacy app, list its SQL queries, then capture a fresh Extended Events session via `scripts/legacy-spike/run.sh` to validate.
- **Replacing reports**: see `_REPORTS_INVENTORY.md` — recommends QuestPDF + ZXing.Net.

## Critical landmines (also in `_COMPAT_CHEATSHEET.md`)

1. **Thai-text encoding**: text columns are `varchar Thai_CI_AS` (Windows-874/TIS-620). Sending `N'…'` Unicode literals corrupts Thai → `?`. Use plain `varchar` parameters.
2. **`'Check-Out'` (hyphen) vs `'Check Out'` (space) bug** at `FrmCheckOut.cs:6246`. New app: write hyphen, tolerate both on read. Hyphen-versions get purged on legacy-app startup.
3. **PK generation race**: `Module1.get_id` uses `SELECT MAX(col)+1`. Use `TABLOCKX+HOLDLOCK` (proven in `docs/legacy-spike/findings.md` §6).
4. **Customer delete sentinel**: deleted customers get `Cust_no='C0000'` in 6 dependent tables (not orphaned).
5. **Online kill switch** at `http://www.kpsystem.co.th/chk_hotel.php` — vendor can remotely disable installs. Mostly relevant if the original binary stays running long-term.

## Caveats

- C# in `decompiled-source/` reflects the obfuscation→cleanup→decompile pipeline. May differ subtly from original VB.NET source.
- `decompiled-source/HOTEL-cleaned.csproj` reports ~280 build errors in 4 known buckets (decompiler artifacts) — none block IDE navigation. See its README.
- ~95 Crystal Reports plumbing files are kept on disk but excluded from build (`<Compile Remove>`). Crystal has no clean .NET 8 path; you're moving to QuestPDF.
- `_COMPAT_CHEATSHEET.md` `'C0000'` sentinel info, ID-format conventions, and several other items overlap with `docs/legacy-spike/findings.md`. Trust live-capture wherever they conflict.
