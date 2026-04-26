using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

public class TwainHandler : Office2007Form, IMessageFilter
{
	private static List<WeakReference> __ENCList;

	private bool msgfilter;

	private Twain tw;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	private int picnumber;

	public string FilePath;

	public string FileName;

	private IContainer components;

	internal virtual Button Button1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Button1 = value;
		}
	}

	[DebuggerNonUserCode]
	static TwainHandler()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DllImport("GDI32.DLL", ExactSpelling = true)]
	internal static extern int SetDIBitsToDevice(IntPtr hdc, int xdst, int ydst, int width, int height, int xsrc, int ysrc, int start, int lines, IntPtr bitsptr, IntPtr bmiptr, int color);

	[DllImport("kernel32.dll", ExactSpelling = true)]
	internal static extern IntPtr GlobalLock(IntPtr handle);

	[DllImport("kernel32.dll", ExactSpelling = true)]
	internal static extern IntPtr GlobalFree(IntPtr handle);

	[DllImport("kernel32.dll", CharSet = CharSet.Auto)]
	public static extern void OutputDebugString(string outstr);

	public TwainHandler()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += TwainHandler_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		picnumber = 0;
		InitializeComponent();
		tw = new Twain();
		tw.Init(Handle);
	}

	protected override void Dispose(bool disposing)
	{
		if (disposing && components != null)
		{
			components.Dispose();
		}
		base.Dispose(disposing);
	}

	public bool PreFilterMessage(ref Message m)
	{
		checked
		{
			switch (tw.PassMessage(m))
			{
			case TwainCommand.Not:
				return false;
			case TwainCommand.Null:
				EndingScan();
				tw.CloseSrc();
				break;
			case TwainCommand.TransferReady:
			{
				ArrayList arrayList = tw.TransferPictures();
				EndingScan();
				tw.CloseSrc();
				picnumber++;
				int num = arrayList.Count - 1;
				int num2 = 0;
				IntPtr intPtr = default(IntPtr);
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						object obj = arrayList[num2];
						IntPtr intPtr2 = ((obj != null) ? ((IntPtr)obj) : intPtr);
						IntPtr handle = intPtr2;
						IntPtr intPtr3 = GlobalLock(handle);
						IntPtr pixelInfo = GetPixelInfo(intPtr3);
						if (!Gdip.smethod_0(FilePath + "\\" + FileName, intPtr3, pixelInfo))
						{
							FileName = "";
						}
						GlobalFree(handle);
						handle = IntPtr.Zero;
						num2++;
						continue;
					}
					break;
				}
				break;
			}
			case TwainCommand.CloseRequest:
				EndingScan();
				tw.CloseSrc();
				break;
			case TwainCommand.CloseOk:
				EndingScan();
				tw.CloseSrc();
				break;
			}
			return true;
		}
	}

	protected IntPtr GetPixelInfo(IntPtr bmpptr)
	{
		BITMAPINFOHEADER bITMAPINFOHEADER = new BITMAPINFOHEADER();
		Marshal.PtrToStructure(bmpptr, bITMAPINFOHEADER);
		checked
		{
			if (bITMAPINFOHEADER.biSizeImage == 0)
			{
				bITMAPINFOHEADER.biSizeImage = (int)Math.Round(Conversion.Int(Conversions.ToDouble(Conversions.ToString(bITMAPINFOHEADER.biWidth * bITMAPINFOHEADER.biBitCount + 31) + Conversion.Hex(-32)) / 8.0) * (double)bITMAPINFOHEADER.biHeight);
			}
			int num = bITMAPINFOHEADER.biClrUsed;
			if ((num == 0) & (bITMAPINFOHEADER.biBitCount <= 8))
			{
				num = (int)Math.Round(Conversion.Int(1.0 * Math.Pow(2.0, bITMAPINFOHEADER.biBitCount)));
			}
			num = num * 4 + bITMAPINFOHEADER.biSize + bmpptr.ToInt32();
			return new IntPtr(num);
		}
	}

	private void EndingScan()
	{
		if (msgfilter)
		{
			Application.RemoveMessageFilter(this);
			msgfilter = false;
			Enabled = true;
			Activate();
		}
	}

	public string ScanIt(string TheFilePath)
	{
		tw.Select();
		if (!msgfilter)
		{
			Enabled = false;
			msgfilter = true;
			Application.AddMessageFilter(this);
		}
		FilePath = TheFilePath;
		FileName = Strings.Format(DateAndTime.Now, "yyyyMMddhhmmss") + ".jpg";
		tw.Acquire();
		while (!Enabled)
		{
			Application.DoEvents();
		}
		return FileName;
	}

	private void InitializeComponent()
	{
		this.SuspendLayout();
		this.TopMost = true;
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		System.Drawing.Size clientSize = new System.Drawing.Size(197, 106);
		this.ClientSize = clientSize;
		this.Name = "TwainHandler";
		this.ResumeLayout(false);
	}

	private void TwainHandler_Load(object sender, EventArgs e)
	{
	}
}
