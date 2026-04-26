using System;
using System.Collections;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using Microsoft.VisualBasic;

namespace iHOTEL2025;

public class Twain
{
	[StructLayout(LayoutKind.Sequential, Pack = 4)]
	internal struct WINMSG_S
	{
		public IntPtr hwnd;

		public int message;

		public IntPtr wParam;

		public IntPtr lParam;

		public int time;

		public int x;

		public int y;
	}

	private IntPtr hwnd;

	private TwIdentity appid;

	private TwIdentity srcds;

	private TwEvent evtmsg;

	private WINMSG_S winmsg_m;

	public static int ScreenBitDepth
	{
		get
		{
			IntPtr intPtr = CreateDC("DISPLAY", null, null, IntPtr.Zero);
			int deviceCaps = GetDeviceCaps(intPtr, 12);
			deviceCaps = checked(deviceCaps * GetDeviceCaps(intPtr, 14));
			DeleteDC(intPtr);
			return deviceCaps;
		}
	}

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1([In][Out] TwIdentity origin, IntPtr zeroptr, TwDG dg, TwDAT dat, TwMSG msg, ref IntPtr refptr);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_1([In][Out] TwIdentity origin, IntPtr zeroptr, TwDG dg, TwDAT dat, TwMSG msg, [In][Out] TwIdentity idds);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_2([In][Out] TwIdentity origin, IntPtr zeroptr, TwDG dg, TwDAT dat, TwMSG msg, [In][Out] TwStatus dsmstat);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_3([In][Out] TwIdentity origin, [In][Out] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, TwUserInterface guif);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_4([In][Out] TwIdentity origin, [In][Out] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, ref TwEvent evt);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_5([In][Out] TwIdentity origin, [In] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, [In][Out] TwStatus dsmstat);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_6([In][Out] TwIdentity origin, [In] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, [In][Out] TwCapability capa);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_7([In][Out] TwIdentity origin, [In] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, [In][Out] TwImageInfo imginf);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_8([In][Out] TwIdentity origin, [In] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, ref IntPtr hbitmap);

	[DllImport("twain_32.dll", EntryPoint = "#1")]
	private static extern TwRC twain_32_1_9([In][Out] TwIdentity origin, [In] TwIdentity dest, TwDG dg, TwDAT dat, TwMSG msg, [In][Out] TwPendingXfers pxfr);

	[DllImport("kernel32.dll", ExactSpelling = true)]
	internal static extern IntPtr GlobalAlloc(int flags, int size);

	[DllImport("kernel32.dll", ExactSpelling = true)]
	internal static extern IntPtr GlobalLock(IntPtr handle);

	[DllImport("kernel32.dll", ExactSpelling = true)]
	internal static extern bool GlobalUnlock(IntPtr handle);

	[DllImport("kernel32.dll", ExactSpelling = true)]
	internal static extern IntPtr GlobalFree(IntPtr handle);

	[DllImport("User32.dll", ExactSpelling = true)]
	private static extern int GetMessagePos();

	[DllImport("User32.dll", ExactSpelling = true)]
	private static extern int GetMessageTime();

	[DllImport("GDI32.DLL", ExactSpelling = true)]
	private static extern int GetDeviceCaps(IntPtr hDC, int nIndex);

	[DllImport("GDI32.DLL", CharSet = CharSet.Auto)]
	private static extern IntPtr CreateDC(string szdriver, string szdevice, string szoutput, IntPtr devmode);

	[DllImport("GDI32.DLL", ExactSpelling = true)]
	private static extern bool DeleteDC(IntPtr hdc);

	public Twain()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		appid = new TwIdentity();
		appid.Id = IntPtr.Zero;
		appid.Version.MajorNum = 1;
		appid.Version.MinorNum = 1;
		appid.Version.Language = 13;
		appid.Version.Country = 1;
		appid.Version.Info = "Twain 1.0";
		appid.ProtocolMajor = 1;
		appid.ProtocolMinor = 9;
		appid.SupportedGroups = 3;
		appid.Manufacturer = "MKA-SOFT";
		appid.ProductFamily = "Freeware";
		appid.ProductName = "EasyTwain";
		srcds = new TwIdentity();
		srcds.Id = IntPtr.Zero;
		evtmsg.EventPtr = Marshal.AllocHGlobal(Marshal.SizeOf(winmsg_m));
	}

	public void Dispose()
	{
		Marshal.FreeHGlobal(evtmsg.EventPtr);
	}

	protected virtual void Finalize()
	{
		Marshal.FreeHGlobal(evtmsg.EventPtr);
	}

	public void Init(IntPtr hwndp)
	{
		Finish();
		if (twain_32_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Parent, TwMSG.OpenDSM, ref hwndp) == TwRC.Success)
		{
			if (twain_32_1_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Identity, TwMSG.GetDefault, srcds) == TwRC.Success)
			{
				hwnd = hwndp;
			}
			else
			{
				TwRC twRC = twain_32_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Parent, TwMSG.CloseDSM, ref hwndp);
			}
		}
	}

	public void Select()
	{
		if (object.Equals(appid.Id, IntPtr.Zero))
		{
			Init(hwnd);
			if (object.Equals(appid.Id, IntPtr.Zero))
			{
				return;
			}
		}
		twain_32_1_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Identity, TwMSG.UserSelect, srcds);
	}

	public void Acquire()
	{
		if (object.Equals(appid.Id, IntPtr.Zero))
		{
			Init(hwnd);
			if (object.Equals(appid.Id, IntPtr.Zero))
			{
				return;
			}
		}
		if (twain_32_1_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Identity, TwMSG.OpenDS, srcds) != 0)
		{
			return;
		}
		TwCapability capa = new TwCapability(TwCap.XferCount, 1);
		if (twain_32_1_6(appid, srcds, TwDG.Control, TwDAT.Capability, TwMSG.Set, capa) != 0)
		{
			CloseSrc();
			return;
		}
		TwUserInterface twUserInterface = new TwUserInterface();
		twUserInterface.ShowUI = 1;
		twUserInterface.ModalUI = 1;
		twUserInterface.ParentHand = hwnd;
		if (twain_32_1_3(appid, srcds, TwDG.Control, TwDAT.UserInterface, TwMSG.EnableDS, twUserInterface) != 0)
		{
			CloseSrc();
		}
	}

	public ArrayList TransferPictures()
	{
		ArrayList arrayList = new ArrayList();
		if (object.Equals(srcds.Id, IntPtr.Zero))
		{
			return arrayList;
		}
		IntPtr zero = IntPtr.Zero;
		TwPendingXfers twPendingXfers = new TwPendingXfers();
		TwRC twRC;
		do
		{
			twPendingXfers.Count = 0;
			zero = IntPtr.Zero;
			TwImageInfo imginf = new TwImageInfo();
			if (twain_32_1_7(appid, srcds, TwDG.Image, TwDAT.ImageInfo, TwMSG.Get, imginf) == TwRC.Success)
			{
				twRC = twain_32_1_8(appid, srcds, TwDG.Image, TwDAT.ImageNativeXfer, TwMSG.Get, ref zero);
				if (twRC == TwRC.XferDone)
				{
					if (twain_32_1_9(appid, srcds, TwDG.Control, TwDAT.PendingXfers, TwMSG.EndXfer, twPendingXfers) == TwRC.Success)
					{
						arrayList.Add(zero);
						continue;
					}
					CloseSrc();
					return arrayList;
				}
				CloseSrc();
				return arrayList;
			}
			CloseSrc();
			return arrayList;
		}
		while (twPendingXfers.Count != 0);
		twRC = twain_32_1_9(appid, srcds, TwDG.Control, TwDAT.PendingXfers, TwMSG.Reset, twPendingXfers);
		return arrayList;
	}

	public TwainCommand PassMessage(Message m)
	{
		if (object.Equals(srcds.Id, IntPtr.Zero))
		{
			return TwainCommand.Not;
		}
		int messagePos = GetMessagePos();
		winmsg_m.hwnd = m.HWnd;
		winmsg_m.message = m.Msg;
		winmsg_m.wParam = m.WParam;
		winmsg_m.lParam = m.LParam;
		winmsg_m.time = GetMessageTime();
		winmsg_m.x = messagePos;
		winmsg_m.y = checked((int)Math.Round(Conversion.Int((double)messagePos / 65536.0)));
		Marshal.StructureToPtr(winmsg_m, evtmsg.EventPtr, fDeleteOld: false);
		evtmsg.Message = 0;
		TwRC twRC = twain_32_1_4(appid, srcds, TwDG.Control, TwDAT.Event, TwMSG.ProcessEvent, ref evtmsg);
		if (twRC == TwRC.const_5)
		{
			return TwainCommand.Not;
		}
		if (evtmsg.Message == 257)
		{
			return TwainCommand.TransferReady;
		}
		if (evtmsg.Message == 258)
		{
			return TwainCommand.CloseRequest;
		}
		if (evtmsg.Message == 259)
		{
			return TwainCommand.CloseOk;
		}
		if (evtmsg.Message == 260)
		{
			return TwainCommand.DeviceEvent;
		}
		return TwainCommand.Null;
	}

	public void CloseSrc()
	{
		if (!object.Equals(srcds.Id, IntPtr.Zero))
		{
			TwUserInterface guif = new TwUserInterface();
			twain_32_1_3(appid, srcds, TwDG.Control, TwDAT.UserInterface, TwMSG.DisableDS, guif);
			twain_32_1_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Identity, TwMSG.CloseDS, srcds);
		}
	}

	public void Finish()
	{
		CloseSrc();
		if (!object.Equals(appid.Id, IntPtr.Zero))
		{
			twain_32_1(appid, IntPtr.Zero, TwDG.Control, TwDAT.Parent, TwMSG.CloseDSM, ref hwnd);
		}
		appid.Id = IntPtr.Zero;
	}
}
