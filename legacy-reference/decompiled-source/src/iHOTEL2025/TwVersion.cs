using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal struct TwVersion
{
	public short MajorNum;

	public short MinorNum;

	public short Language;

	public short Country;

	[MarshalAs(UnmanagedType.ByValTStr, SizeConst = 34)]
	public string Info;
}
