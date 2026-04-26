using System;
using System.Drawing;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

public class iCam
{
	private string iDevice;

	private int hHwnd;

	private int lwndC;

	public bool iRunning;

	private int CamFrameRate;

	private int OutputHeight;

	private int OutputWidth;

	public iCam()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		CamFrameRate = 15;
		OutputHeight = 480;
		OutputWidth = 640;
	}

	[DllImport("user32", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	private static extern int SendMessageA(int hwnd, int wMsg, short wParam, [MarshalAs(UnmanagedType.VBByRefStr)] ref string lParam);

	[DllImport("avicap32.dll", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	private static extern int capCreateCaptureWindowA([MarshalAs(UnmanagedType.VBByRefStr)] ref string lpszWindowName, int dwStyle, int x, int y, int nWidth, short nHeight, int hWndParent, int nID);

	[DllImport("avicap32.dll", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	private static extern bool capGetDriverDescriptionA(short wDriver, [MarshalAs(UnmanagedType.VBByRefStr)] ref string lpszName, int cbName, [MarshalAs(UnmanagedType.VBByRefStr)] ref string lpszVer, int cbVer);

	[DllImport("GDI32.DLL", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	private static extern bool BitBlt(IntPtr hdcDest, int nXDest, int nYDest, int nWidth, int nHeight, IntPtr hdcSrc, int nXSrc, int nYSrc, int dwRop);

	public void resetCam()
	{
		if (iRunning)
		{
			closeCam();
			Application.DoEvents();
			if (!setCam())
			{
				MessageBox.Show("Errror Setting/Re-Setting Camera");
			}
		}
	}

	public void initCam(int parentH)
	{
		if (iRunning)
		{
			MessageBox.Show("กล\u0e49องได\u0e49ถ\u0e39กเป\u0e34ดโดยโปรแกรมอ\u0e37\u0e48นอย\u0e39\u0e48");
			return;
		}
		hHwnd = capCreateCaptureWindowA(ref iDevice, 1342177280, -100, 0, OutputWidth, checked((short)OutputHeight), parentH, 0);
		if (!setCam())
		{
			MessageBox.Show("ไม\u0e48พบกล\u0e49อง โปรดตรวจสอบกล\u0e49อง");
		}
	}

	public void setFrameRate(long iRate)
	{
		CamFrameRate = checked((int)Math.Round(1000.0 / (double)iRate));
		resetCam();
	}

	private bool setCam()
	{
		int hwnd = hHwnd;
		short wParam = Conversions.ToShort(iDevice);
		string lParam = Conversions.ToString(0);
		if (SendMessageA(hwnd, 1034, wParam, ref lParam) == 1)
		{
			int hwnd2 = hHwnd;
			short wParam2 = checked((short)CamFrameRate);
			string lParam2 = Conversions.ToString(0);
			SendMessageA(hwnd2, 1076, wParam2, ref lParam2);
			int hwnd3 = hHwnd;
			lParam2 = Conversions.ToString(0);
			SendMessageA(hwnd3, 1074, 1, ref lParam2);
			iRunning = true;
			return true;
		}
		iRunning = false;
		return false;
	}

	public bool closeCam()
	{
		bool result = default(bool);
		if (iRunning)
		{
			int hwnd = hHwnd;
			string lParam = Conversions.ToString(0);
			result = SendMessageA(hwnd, 1035, 0, ref lParam) != 0;
			iRunning = false;
		}
		return result;
	}

	public Bitmap copyFrame(PictureBox src, RectangleF rect)
	{
		checked
		{
			Bitmap result = default(Bitmap);
			if (iRunning)
			{
				Graphics graphics = src.CreateGraphics();
				Bitmap bitmap = new Bitmap(src.Width, src.Height, graphics);
				Graphics graphics2 = Graphics.FromImage(bitmap);
				IntPtr hdc = graphics.GetHdc();
				IntPtr hdc2 = graphics2.GetHdc();
				BitBlt(hdc2, 0, 0, (int)Math.Round(rect.Width), (int)Math.Round(rect.Height), hdc, (int)Math.Round(rect.X), (int)Math.Round(rect.Y), 13369376);
				result = (Bitmap)bitmap.Clone();
				graphics.ReleaseHdc(hdc);
				graphics2.ReleaseHdc(hdc2);
				graphics.Dispose();
				graphics2.Dispose();
			}
			else
			{
				MessageBox.Show("กล\u0e49องถ\u0e39กป\u0e34ดอย\u0e39\u0e48!");
			}
			return result;
		}
	}

	public int FPS()
	{
		return checked((int)Math.Round(1000.0 / (double)CamFrameRate));
	}
}
