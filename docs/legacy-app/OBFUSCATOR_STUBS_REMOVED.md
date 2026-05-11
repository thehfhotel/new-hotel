# Obfuscator Stubs Removed

The original `_decompiled_clean/` folder contained a handful of files that are
not part of the application — they are leftovers from the .NET Reactor
obfuscator (which was applied to the original `HOTEL.exe` before it was cleaned
with de4dot and decompiled with ilspycmd). Those files were removed when
building this reference codebase.

## Removed (top-level, deleted entirely — not archived)

| File | Why removed |
|---|---|
| `Class0.cs` | Empty `internal static class Class0 {}` stub |
| `Class1.cs` | .NET Reactor metadata-token resolver. References `Class2.LH6iGfYz9j3MJ()` and rewires delegate fields via reflection. Pure runtime support; not application code. |
| `Class2.cs` | Empty `LH6iGfYz9j3MJ()` method called from Reactor's static ctors. No-op after de4dot cleanup. |
| `-Module--DE8036EB-0D5A-41DB-A04B-F80838A0AC12-.cs` | Empty `<Module>` placeholder class with a GUID name (Reactor module init stub) |
| `aR3nbf8dQp2feLmk31.SplashForm.resx` | Resource for the obfuscator's renamed splash form. The matching `.cs` was already merged into the iHOTEL2025 namespace by de4dot, so this resx is orphaned. |
| `aR3nbf8dQp2feLmk31.lSfgApatkdxsVcGcrktoFd.resx` | Same — obfuscator-renamed resource with no surviving matching class. |

These files exist only because the original assembly had Reactor's runtime
initializer baked in. They contain no business logic. Removing them does NOT
break F12 navigation or compile (the only call into `Class2.LH6iGfYz9j3MJ()`
came from `Class1`, which is also removed). The feature map already documents
this in section 8 ("Cruft / Skip in Rewrite").

## Archived (moved to `_archived/`, not deleted)

These are duplicate / older copies kept by the original developer. They were
moved out of the build path so they don't shadow the active versions, but kept
in case the user wants to diff them against the current ones.

| Archived file | Replaced by |
|---|---|
| `FormRoomMainKichen_old.cs` | `FormRoomMainKichen.cs` |
| `FormSearchRooms2_old.cs` | `FormSearchRooms2.cs` |
| `FormSelectDB_old.cs` | `FormSelectDB.cs` |
| `FrmAddBook2copy.cs` | `FrmAddBook2.cs` |
| `ReportShipCashOLD.cs` | `ReportShipCash.cs` |
| `CachedReportShipCashOLD.cs` | `CachedReportShipCash.cs` |
| `Cachedsale_vat0_copy.cs` | `Cachedsale_vat0.cs` |
| `Cachedsale_vat_copy.cs` | `Cachedsale_vat.cs` |
| `sale_vat0_copy.cs` | `sale_vat0.cs` |
| `sale_vat_copy.cs` | `sale_vat.cs` |
| `iHOTEL2025.FormRoomMainKichen_old.resx` | matched archived form |
| `iHOTEL2025.FrmAddBook2copy.resx` | matched archived form |
