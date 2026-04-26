using System.Diagnostics;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class TwImageInfo
{
	public int int_0;

	public int int_1;

	public int ImageWidth;

	public int ImageLength;

	public short SamplesPerPixel;

	[MarshalAs(UnmanagedType.ByValArray, SizeConst = 8)]
	public short[] BitsPerSample;

	public short BitsPerPixel;

	public short Planar;

	public short PixelType;

	public short Compression;

	[DebuggerNonUserCode]
	public TwImageInfo()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}
}
