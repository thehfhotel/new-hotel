using System.Diagnostics;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class BITMAPINFOHEADER
{
	public int biSize;

	public int biWidth;

	public int biHeight;

	public short biPlanes;

	public short biBitCount;

	public int biCompression;

	public int biSizeImage;

	public int biXPelsPerMeter;

	public int biYPelsPerMeter;

	public int biClrUsed;

	public int biClrImportant;

	[DebuggerNonUserCode]
	public BITMAPINFOHEADER()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}
}
