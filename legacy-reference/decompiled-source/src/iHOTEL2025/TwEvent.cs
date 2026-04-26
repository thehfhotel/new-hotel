using System;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal struct TwEvent
{
	public IntPtr EventPtr;

	public short Message;
}
