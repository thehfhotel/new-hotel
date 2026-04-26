using System.Diagnostics;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class TwPendingXfers
{
	public short Count;

	public int EOJ;

	[DebuggerNonUserCode]
	public TwPendingXfers()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}
}
