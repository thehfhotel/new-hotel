using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class TwUserInterface
{
	public short ShowUI;

	public short ModalUI;

	public IntPtr ParentHand;

	[DebuggerNonUserCode]
	public TwUserInterface()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}
}
