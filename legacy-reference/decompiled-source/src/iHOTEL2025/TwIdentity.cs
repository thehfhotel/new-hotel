using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class TwIdentity
{
	public IntPtr Id;

	public TwVersion Version;

	public short ProtocolMajor;

	public short ProtocolMinor;

	public int SupportedGroups;

	[MarshalAs(UnmanagedType.ByValTStr, SizeConst = 34)]
	public string Manufacturer;

	[MarshalAs(UnmanagedType.ByValTStr, SizeConst = 34)]
	public string ProductFamily;

	[MarshalAs(UnmanagedType.ByValTStr, SizeConst = 34)]
	public string ProductName;

	[DebuggerNonUserCode]
	public TwIdentity()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}
}
