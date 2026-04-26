using System.Diagnostics;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class TwStatus
{
	public short ConditionCode;

	public short Reserved;

	[DebuggerNonUserCode]
	public TwStatus()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}
}
