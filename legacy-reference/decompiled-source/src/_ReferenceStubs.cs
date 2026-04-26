// REFERENCE-CODEBASE SCAFFOLDING — not part of the original application.
//
// The decompile output references two things that don't have sources we can
// build against. This file provides minimal stubs so the project compiles
// enough for IDE navigation. None of this code is meant to do anything.
//
// 1. `Class2.LH6iGfYz9j3MJ()` — the .NET Reactor obfuscator's runtime
//    initializer. Originally `Class2.cs` provided an empty no-op method;
//    we deleted that file (see _OBFUSCATOR_STUBS_REMOVED.md) so we provide
//    the same no-op stub here.
//
// 2. (handled separately) `base._002Ector()` calls inside constructors are
//    decompile noise — they represent `base..ctor()` (the parameterless base
//    constructor call) which the C# compiler emits implicitly. They have
//    been stripped in-place across the codebase. The original calls are
//    preserved as `// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs`
//    so you can still see where the IL had them.
//
// Global namespace — must match the original Class2 location (no namespace).

internal static class Class2
{
    // .NET Reactor's static-init hook. Originally a no-op after de4dot
    // cleanup. Kept as a no-op here so the hundreds of synthetic call sites
    // throughout the decompile still compile.
    internal static void LH6iGfYz9j3MJ() { }
}

// Forward-declaration stubs so MyProject.cs (the auto-generated VB My namespace
// helper) can compile. The real classes either:
//   (a) live in archived/ (older copies, see _OBFUSCATOR_STUBS_REMOVED.md), or
//   (b) are excluded from build because of decompiler artifacts (see csproj).
// Either way, MyProject.cs holds a `m_X` field plus a property for every Form
// in the assembly — those references compile against these empty Form stubs.
namespace iHOTEL2025
{
    internal class FormRoomMainKichen_old : System.Windows.Forms.Form { }
    internal class FormSearchRooms2_old   : System.Windows.Forms.Form { }
    internal class FormSelectDB_old       : System.Windows.Forms.Form { }
    internal class FrmAddBook2copy        : System.Windows.Forms.Form { }
    internal class AboutBox1              : System.Windows.Forms.Form { }
    internal class FrmPrint               : System.Windows.Forms.Form { }
    internal class FrmReportRR4           : System.Windows.Forms.Form { }
    internal class GForm0                 : System.Windows.Forms.Form { }
    internal class ReportTax              : System.Windows.Forms.Form { }
}
