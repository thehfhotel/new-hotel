using System;
using System.Diagnostics;
using System.Drawing.Imaging;
using System.IO;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using Microsoft.VisualBasic;

namespace iHOTEL2025;

public class Gdip
{
	private static ImageCodecInfo[] codecs;

	static Gdip()
	{
		Class2.LH6iGfYz9j3MJ();
		codecs = ImageCodecInfo.GetImageEncoders();
	}

	[DebuggerNonUserCode]
	public Gdip()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
	}

	[DllImport("gdiplus.dll", ExactSpelling = true)]
	internal static extern int GdipCreateBitmapFromGdiDib(IntPtr bminfo, IntPtr pixdat, ref IntPtr image);

	[DllImport("gdiplus.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
	internal static extern int GdipSaveImageToFile(IntPtr image, string filename, [In] ref Guid clsid, IntPtr encparams);

	[DllImport("gdiplus.dll", ExactSpelling = true)]
	internal static extern int GdipDisposeImage(IntPtr image);

	private static bool GetCodecClsid(string filename, ref Guid clsid)
	{
		clsid = Guid.Empty;
		string extension = Path.GetExtension(filename);
		if (Information.IsNothing(extension))
		{
			return false;
		}
		extension = "*" + extension.ToUpper();
		ImageCodecInfo[] array = codecs;
		int num = 0;
		ImageCodecInfo imageCodecInfo;
		while (true)
		{
			if (num < array.Length)
			{
				imageCodecInfo = array[num];
				if (imageCodecInfo.FilenameExtension.IndexOf(extension) >= 0)
				{
					break;
				}
				num = checked(num + 1);
				continue;
			}
			return false;
		}
		clsid = imageCodecInfo.Clsid;
		return true;
	}

	public static bool smethod_0(string picname, IntPtr bminfo, IntPtr pixdat)
	{
		SaveFileDialog saveFileDialog = new SaveFileDialog();
		saveFileDialog.FileName = picname;
		Guid clsid = default(Guid);
		if (!GetCodecClsid(saveFileDialog.FileName, ref clsid))
		{
			MessageBox.Show("Unknown picture format for extension " + Path.GetExtension(saveFileDialog.FileName), "Image Codec", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
			return false;
		}
		IntPtr image = IntPtr.Zero;
		int num = GdipCreateBitmapFromGdiDib(bminfo, pixdat, ref image);
		if ((num != 0) | object.Equals(image, IntPtr.Zero))
		{
			return false;
		}
		num = GdipSaveImageToFile(image, saveFileDialog.FileName, ref clsid, IntPtr.Zero);
		GdipDisposeImage(image);
		return num == 0;
	}
}
