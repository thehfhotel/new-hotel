#define DEBUG
using System;
using System.Collections;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using System.Security.Cryptography;
using System.Text;
using System.Threading;
using System.Windows.Media.Imaging;
using FGIyrnoeXE1biscEL7;
using FpHJ8h91YAL9ENl1Hh;
using JF2JbQYXkmXJkjq6I2;
using PCSC;
using ThaiNationalIDCard;
using aNwb5fLJR80OGTF62W;
using sDT32rX3GAFjM98cjK;
using wOoivnck48bv66Xx6l;

[assembly: Guid("2be7c282-8499-426b-80e9-1c13c4ca4c84")]
[assembly: AssemblyFileVersion("1.0.1.0")]
[assembly: AssemblyTrademark("")]
[assembly: ComVisible(false)]
[assembly: AssemblyKeyName("")]
[assembly: AssemblyDelaySign(false)]
[assembly: Debuggable(DebuggableAttribute.DebuggingModes.Default | DebuggableAttribute.DebuggingModes.DisableOptimizations | DebuggableAttribute.DebuggingModes.IgnoreSymbolStoreSequencePoints | DebuggableAttribute.DebuggingModes.EnableEditAndContinue)]
[assembly: SuppressIldasm]
[assembly: TargetFramework(".NETFramework,Version=v4.0", FrameworkDisplayName = ".NET Framework 4")]
[assembly: RuntimeCompatibility(WrapNonExceptionThrows = true)]
[assembly: AssemblyCopyright("Copyright ©  2013")]
[assembly: AssemblyConfiguration("")]
[assembly: AssemblyCompany("")]
[assembly: AssemblyDescription("")]
[assembly: AssemblyProduct("ThaiNationalIDCard")]
[assembly: CompilationRelaxations(8)]
[assembly: AssemblyTitle("ThaiNationalIDCard")]
[assembly: AssemblyVersion("1.0.1.33513")]
namespace pG8C47vnLOfqifVSkt
{
	internal static class SWv6I34a8yfGM795e0
	{
	}
}
namespace FpHJ8h91YAL9ENl1Hh
{
	internal class Ch2F4WjomBJtjhG5SS
	{
		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] eiQJfbM26e()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[1][] { new byte[13]
			{
				0, 164, 4, 0, 8, 160, 0, 0, 0, 84,
				72, 0, 1
			} };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public CMD_PAIR[] eyDJ0rMUPm()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			CMD_PAIR[] array = new CMD_PAIR[21];
			for (int i = 0; i <= 20; i++)
			{
				int num = i * 254 + 379;
				int num2 = ((i != 20) ? 254 : 38);
				int num3 = (num >> 8) & 0xFF;
				int num4 = num & 0xFF;
				int num5 = num2 & 0xFF;
				int num6 = num2 & 0xFF;
				ref CMD_PAIR reference = ref array[i];
				byte[] array2 = new byte[7] { 128, 176, 0, 0, 2, 0, 0 };
				array2[2] = (byte)num3;
				array2[3] = (byte)num4;
				array2[6] = (byte)num5;
				reference.CMD1 = array2;
				array[i].CMD2 = new byte[5]
				{
					0,
					192,
					0,
					0,
					(byte)num6
				};
			}
			return array;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Ch2F4WjomBJtjhG5SS()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			base..ctor();
		}
	}
}
namespace ThaiNationalIDCard
{
	public struct CMD_PAIR
	{
		public byte[] CMD1;

		public byte[] CMD2;
	}
}
namespace aNwb5fLJR80OGTF62W
{
	internal interface rEV8TSS0jSiTyvRiyS
	{
		byte[][] eiQJfbM26e();

		byte[][] NUWJrv6I3a();

		byte[][] kyfJzGM795();

		byte[][] A034MG8C47();

		byte[][] FLO4JfqifV();

		CMD_PAIR[] eyDJ0rMUPm();
	}
}
namespace JF2JbQYXkmXJkjq6I2
{
	internal class GY1ZaY5fnxhqDu3iI6 : Ch2F4WjomBJtjhG5SS, rEV8TSS0jSiTyvRiyS
	{
		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] NUWJrv6I3a()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 0, 4, 2, 0, 13 },
				new byte[5] { 0, 192, 0, 1, 13 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] kyfJzGM795()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 0, 17, 2, 0, 209 },
				new byte[5] { 0, 192, 0, 1, 209 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] A034MG8C47()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 21, 121, 2, 0, 100 },
				new byte[5] { 0, 192, 0, 1, 100 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] FLO4JfqifV()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 1, 103, 2, 0, 18 },
				new byte[5] { 0, 192, 0, 1, 18 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public GY1ZaY5fnxhqDu3iI6()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			base..ctor();
		}
	}
}
namespace FGIyrnoeXE1biscEL7
{
	internal class lFqymCW8Q1M6pukOq1 : Ch2F4WjomBJtjhG5SS, rEV8TSS0jSiTyvRiyS
	{
		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] NUWJrv6I3a()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 0, 4, 2, 0, 13 },
				new byte[5] { 0, 192, 0, 0, 13 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] kyfJzGM795()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 0, 17, 2, 0, 209 },
				new byte[5] { 0, 192, 0, 0, 209 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] A034MG8C47()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 21, 121, 2, 0, 100 },
				new byte[5] { 0, 192, 0, 0, 100 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public byte[][] FLO4JfqifV()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return new byte[2][]
			{
				new byte[7] { 128, 176, 1, 103, 2, 0, 18 },
				new byte[5] { 0, 192, 0, 0, 18 }
			};
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public lFqymCW8Q1M6pukOq1()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			base..ctor();
		}
	}
}
namespace ThaiNationalIDCard
{
	public class Personal
	{
		private string eiQJbM26e;

		private string eyD4rMUPm;

		private string NUWvv6I3a;

		private string[] kyfjGM795;

		private string[] A039G8C47;

		private byte[] FLOSfqifV;

		[CompilerGenerated]
		private string fktLph2F4;

		public string Citizenid
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			[CompilerGenerated]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return fktLph2F4;
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			[CompilerGenerated]
			set
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				fktLph2F4 = value;
			}
		}

		public byte[] PhotoRaw
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return FLOSfqifV;
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			set
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				FLOSfqifV = value;
			}
		}

		public Bitmap PhotoBitmap
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				if (FLOSfqifV == null)
				{
					return null;
				}
				JpegBitmapDecoder jpegBitmapDecoder = new JpegBitmapDecoder(new MemoryStream(FLOSfqifV), BitmapCreateOptions.PreservePixelFormat, BitmapCacheOption.Default);
				BitmapSource source = jpegBitmapDecoder.Frames[0];
				using MemoryStream stream = new MemoryStream();
				BitmapEncoder bitmapEncoder = new BmpBitmapEncoder();
				bitmapEncoder.Frames.Add(BitmapFrame.Create(source));
				bitmapEncoder.Save(stream);
				return new Bitmap(stream);
			}
		}

		public string Info
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			set
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				eiQJbM26e = value;
				kyfjGM795 = eiQJbM26e.Substring(0, 100).Split('#');
				A039G8C47 = eiQJbM26e.Substring(100, 100).Split('#');
			}
		}

		public string Address
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Replace('#', ' ');
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			set
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				eyD4rMUPm = value.Trim();
			}
		}

		public string addrHouseNo
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[0].Trim();
			}
		}

		public string addrVillageNo
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[1].Trim();
			}
		}

		public string addrLane
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[2].Trim();
			}
		}

		public string addrRoad
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[3].Trim();
			}
		}

		public string addrTambol
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[5].Trim();
			}
		}

		public string addrAmphur
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[6].Trim();
			}
		}

		public string addrProvince
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eyD4rMUPm.Split('#')[7].Trim();
			}
		}

		public string Issue_Expire
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			set
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				NUWvv6I3a = value;
			}
		}

		public DateTime Issue
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return new DateTime(Convert.ToInt32(NUWvv6I3a.Substring(0, 4)) - 543, Convert.ToInt32(NUWvv6I3a.Substring(4, 2)), Convert.ToInt32(NUWvv6I3a.Substring(6, 2)));
			}
		}

		public DateTime Expire
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				int year = Convert.ToInt32(NUWvv6I3a.Substring(8, 4)) - 543;
				int num = Convert.ToInt32(NUWvv6I3a.Substring(12, 2));
				int num2 = Convert.ToInt32(NUWvv6I3a.Substring(14, 2));
				return new DateTime(year, (num > 12) ? 12 : num, (num2 > 31) ? 31 : num2);
			}
		}

		public DateTime Birthday
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return new DateTime(Convert.ToInt32(eiQJbM26e.Substring(200, 4)) - 543, Convert.ToInt32(eiQJbM26e.Substring(204, 2)), Convert.ToInt32(eiQJbM26e.Substring(206, 2)));
			}
		}

		public string Sex
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return eiQJbM26e.Substring(208, 1);
			}
		}

		public string Th_Prefix
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return kyfjGM795[0].Trim();
			}
		}

		public string Th_Firstname
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return kyfjGM795[1].Trim();
			}
		}

		public string Th_Lastname
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return kyfjGM795[3].Trim();
			}
		}

		public string En_Prefix
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return A039G8C47[0].Trim();
			}
		}

		public string En_Firstname
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return A039G8C47[1].Trim();
			}
		}

		public string En_Lastname
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return A039G8C47[3].Trim();
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Personal()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			base..ctor();
		}
	}
	public delegate void handlePhotoProgress(int value, int maximum);
	public delegate void handleCardInserted(Personal personal);
	public delegate void handleCardRemoved();
	public class ThaiIDCard
	{
		private static readonly IContextFactory h1ZBaYfnx;

		private ISCardContext LqDZu3iI6;

		private SCardReader JF23JbQXk;

		private SCardError EXJ1kjq6I;

		private IntPtr DdFuqymC8;

		private rEV8TSS0jSiTyvRiyS A1MT6pukO;

		private SCardMonitor R1n6GIyrn;

		private string AXEy1bisc;

		private int yL7X35E33;

		private handlePhotoProgress UgjHf5s3G;

		private handleCardInserted BG9OKXTyi;

		private handleCardInserted oQyK7Yhwa;

		private handleCardRemoved cCWDncRR5;

		public event handlePhotoProgress eventPhotoProgress
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			add
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handlePhotoProgress handlePhotoProgress2 = UgjHf5s3G;
				handlePhotoProgress handlePhotoProgress3;
				do
				{
					handlePhotoProgress3 = handlePhotoProgress2;
					handlePhotoProgress value2 = (handlePhotoProgress)Delegate.Combine(handlePhotoProgress3, value);
					handlePhotoProgress2 = Interlocked.CompareExchange(ref UgjHf5s3G, value2, handlePhotoProgress3);
				}
				while ((object)handlePhotoProgress2 != handlePhotoProgress3);
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			remove
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handlePhotoProgress handlePhotoProgress2 = UgjHf5s3G;
				handlePhotoProgress handlePhotoProgress3;
				do
				{
					handlePhotoProgress3 = handlePhotoProgress2;
					handlePhotoProgress value2 = (handlePhotoProgress)Delegate.Remove(handlePhotoProgress3, value);
					handlePhotoProgress2 = Interlocked.CompareExchange(ref UgjHf5s3G, value2, handlePhotoProgress3);
				}
				while ((object)handlePhotoProgress2 != handlePhotoProgress3);
			}
		}

		public event handleCardInserted eventCardInserted
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			add
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handleCardInserted handleCardInserted2 = BG9OKXTyi;
				handleCardInserted handleCardInserted3;
				do
				{
					handleCardInserted3 = handleCardInserted2;
					handleCardInserted value2 = (handleCardInserted)Delegate.Combine(handleCardInserted3, value);
					handleCardInserted2 = Interlocked.CompareExchange(ref BG9OKXTyi, value2, handleCardInserted3);
				}
				while ((object)handleCardInserted2 != handleCardInserted3);
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			remove
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handleCardInserted handleCardInserted2 = BG9OKXTyi;
				handleCardInserted handleCardInserted3;
				do
				{
					handleCardInserted3 = handleCardInserted2;
					handleCardInserted value2 = (handleCardInserted)Delegate.Remove(handleCardInserted3, value);
					handleCardInserted2 = Interlocked.CompareExchange(ref BG9OKXTyi, value2, handleCardInserted3);
				}
				while ((object)handleCardInserted2 != handleCardInserted3);
			}
		}

		public event handleCardInserted eventCardInsertedWithPhoto
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			add
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handleCardInserted handleCardInserted2 = oQyK7Yhwa;
				handleCardInserted handleCardInserted3;
				do
				{
					handleCardInserted3 = handleCardInserted2;
					handleCardInserted value2 = (handleCardInserted)Delegate.Combine(handleCardInserted3, value);
					handleCardInserted2 = Interlocked.CompareExchange(ref oQyK7Yhwa, value2, handleCardInserted3);
				}
				while ((object)handleCardInserted2 != handleCardInserted3);
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			remove
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handleCardInserted handleCardInserted2 = oQyK7Yhwa;
				handleCardInserted handleCardInserted3;
				do
				{
					handleCardInserted3 = handleCardInserted2;
					handleCardInserted value2 = (handleCardInserted)Delegate.Remove(handleCardInserted3, value);
					handleCardInserted2 = Interlocked.CompareExchange(ref oQyK7Yhwa, value2, handleCardInserted3);
				}
				while ((object)handleCardInserted2 != handleCardInserted3);
			}
		}

		public event handleCardRemoved eventCardRemoved
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			add
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handleCardRemoved handleCardRemoved2 = cCWDncRR5;
				handleCardRemoved handleCardRemoved3;
				do
				{
					handleCardRemoved3 = handleCardRemoved2;
					handleCardRemoved value2 = (handleCardRemoved)Delegate.Combine(handleCardRemoved3, value);
					handleCardRemoved2 = Interlocked.CompareExchange(ref cCWDncRR5, value2, handleCardRemoved3);
				}
				while ((object)handleCardRemoved2 != handleCardRemoved3);
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			remove
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				handleCardRemoved handleCardRemoved2 = cCWDncRR5;
				handleCardRemoved handleCardRemoved3;
				do
				{
					handleCardRemoved3 = handleCardRemoved2;
					handleCardRemoved value2 = (handleCardRemoved)Delegate.Remove(handleCardRemoved3, value);
					handleCardRemoved2 = Interlocked.CompareExchange(ref cCWDncRR5, value2, handleCardRemoved3);
				}
				while ((object)handleCardRemoved2 != handleCardRemoved3);
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public int ErrorCode()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return yL7X35E33;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public string Error()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return AXEy1bisc;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public string Version()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return Assembly.GetExecutingAssembly().GetName().Version.ToString();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void jom5BJtjh(CardStatusEventArgs P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				if (BG9OKXTyi != null)
				{
					Personal personal = readAll(with_photo: false, P_0.ReaderName);
					BG9OKXTyi(personal);
				}
				if (oQyK7Yhwa != null)
				{
					Personal personal = readAll(with_photo: true, P_0.ReaderName);
					oQyK7Yhwa(personal);
				}
			}
			catch (PCSCException ex)
			{
				yL7X35E33 = 256;
				AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(0) + ex.Message + ER5MIqF7oL5pcFCvml.JKSlhDfrB(34) + ex.SCardError.ToString() + ER5MIqF7oL5pcFCvml.JKSlhDfrB(42);
				Debug.Print(AXEy1bisc);
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void Q5SYSnpHJ(string P_0, CardStatusEventArgs P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (cCWDncRR5 != null)
			{
				cCWDncRR5();
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public bool MonitorStart(string readerName)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				R1n6GIyrn = new SCardMonitor(h1ZBaYfnx, SCardScope.System);
				R1n6GIyrn.CardInserted += [MethodImpl(MethodImplOptions.NoInlining)] (object  , CardStatusEventArgs  ) =>
				{
					while (false)
					{
						_ = ((object[])null)[0];
					}
					jom5BJtjh(P_1);
				};
				R1n6GIyrn.CardRemoved += [MethodImpl(MethodImplOptions.NoInlining)] (object  , CardStatusEventArgs  ) =>
				{
					while (false)
					{
						_ = ((object[])null)[0];
					}
					Q5SYSnpHJ(ER5MIqF7oL5pcFCvml.JKSlhDfrB(472), P_1);
				};
				R1n6GIyrn.Start(readerName);
				return true;
			}
			catch (PCSCException ex)
			{
				yL7X35E33 = 256;
				AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(48) + ex.Message + ER5MIqF7oL5pcFCvml.JKSlhDfrB(34) + ex.SCardError.ToString() + ER5MIqF7oL5pcFCvml.JKSlhDfrB(42);
				Debug.Print(AXEy1bisc);
				return false;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public bool MonitorStop(string readerName)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				if (R1n6GIyrn != null)
				{
					R1n6GIyrn.Cancel();
				}
				return true;
			}
			catch (PCSCException ex)
			{
				yL7X35E33 = 256;
				AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(88) + ex.Message + ER5MIqF7oL5pcFCvml.JKSlhDfrB(34) + ex.SCardError.ToString() + ER5MIqF7oL5pcFCvml.JKSlhDfrB(42);
				Debug.Print(AXEy1bisc);
				return false;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void kh1WYAL9E(SCardError P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (P_0 != 0)
			{
				throw new PCSCException(P_0, SCardHelper.StringifyError(P_0));
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private string Hl1oHhTEV(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			byte[] bytes = Encoding.Convert(Encoding.GetEncoding(ER5MIqF7oL5pcFCvml.JKSlhDfrB(126)), Encoding.UTF8, P_0);
			string @string = Encoding.UTF8.GetString(bytes);
			return @string.Substring(0, @string.Length - 2);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] kTS80jSiT(byte[][] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			byte[] receiveBuffer = new byte[256];
			foreach (byte[] sendBuffer in P_0)
			{
				receiveBuffer = new byte[256];
				EXJ1kjq6I = JF23JbQXk.Transmit(DdFuqymC8, sendBuffer, ref receiveBuffer);
				kh1WYAL9E(EXJ1kjq6I);
			}
			return receiveBuffer;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] tvRdiySJN()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			MemoryStream memoryStream = new MemoryStream();
			CMD_PAIR[] array = A1MT6pukO.eyDJ0rMUPm();
			for (int i = 0; i < array.Length; i++)
			{
				byte[] receiveBuffer = new byte[256];
				EXJ1kjq6I = JF23JbQXk.Transmit(DdFuqymC8, array[i].CMD1, ref receiveBuffer);
				kh1WYAL9E(EXJ1kjq6I);
				if (receiveBuffer.Length > 0)
				{
					byte[] receiveBuffer2 = new byte[256];
					EXJ1kjq6I = JF23JbQXk.Transmit(DdFuqymC8, array[i].CMD2, ref receiveBuffer2);
					kh1WYAL9E(EXJ1kjq6I);
					memoryStream.Write(receiveBuffer2, 0, receiveBuffer2.Length - 2);
				}
				if (UgjHf5s3G != null)
				{
					UgjHf5s3G(i + 1, array.Length);
				}
			}
			memoryStream.Seek(0L, SeekOrigin.Begin);
			return memoryStream.ToArray();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public string[] GetReaders()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				LqDZu3iI6 = h1ZBaYfnx.Establish(SCardScope.System);
				string[] readers = LqDZu3iI6.GetReaders();
				LqDZu3iI6.Release();
				if (readers.Length <= 0)
				{
					throw new PCSCException(SCardError.NoReadersAvailable, ER5MIqF7oL5pcFCvml.JKSlhDfrB(144));
				}
				return readers;
			}
			catch (PCSCException ex)
			{
				yL7X35E33 = 256;
				AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(220) + ex.Message + ER5MIqF7oL5pcFCvml.JKSlhDfrB(34) + ex.SCardError.ToString() + ER5MIqF7oL5pcFCvml.JKSlhDfrB(42);
				Debug.Print(AXEy1bisc);
				throw ex;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public bool Open(string readerName = null)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				Thread.Sleep(1500);
				LqDZu3iI6 = h1ZBaYfnx.Establish(SCardScope.System);
				JF23JbQXk = new SCardReader(LqDZu3iI6);
				if (string.IsNullOrEmpty(readerName))
				{
					string[] readers = LqDZu3iI6.GetReaders();
					if (readers.Length <= 0)
					{
						throw new PCSCException(SCardError.NoReadersAvailable, ER5MIqF7oL5pcFCvml.JKSlhDfrB(256));
					}
					EXJ1kjq6I = JF23JbQXk.Connect(readers[0], SCardShareMode.Exclusive, SCardProtocol.Any);
					kh1WYAL9E(EXJ1kjq6I);
				}
				else
				{
					EXJ1kjq6I = JF23JbQXk.Connect(readerName, SCardShareMode.Shared, SCardProtocol.Any);
					kh1WYAL9E(EXJ1kjq6I);
				}
				DdFuqymC8 = default(IntPtr);
				switch (JF23JbQXk.ActiveProtocol)
				{
				case SCardProtocol.T0:
					DdFuqymC8 = SCardPCI.T0;
					break;
				case SCardProtocol.T1:
					DdFuqymC8 = SCardPCI.T1;
					break;
				default:
					throw new PCSCException(SCardError.ProtocolMismatch, ER5MIqF7oL5pcFCvml.JKSlhDfrB(334) + JF23JbQXk.ActiveProtocol);
				}
				string[] readerName2;
				SCardState state;
				SCardProtocol protocol;
				byte[] atr;
				SCardError sCardError = JF23JbQXk.Status(out readerName2, out state, out protocol, out atr);
				if (atr == null || atr.Length < 2)
				{
					return false;
				}
				if (atr[0] == 59 && atr[1] == 104)
				{
					A1MT6pukO = new lFqymCW8Q1M6pukOq1();
				}
				else if (atr[0] == 59 && atr[1] == 120)
				{
					A1MT6pukO = new lFqymCW8Q1M6pukOq1();
				}
				else if (atr[0] == 59 && atr[1] == 121)
				{
					A1MT6pukO = new lFqymCW8Q1M6pukOq1();
				}
				else
				{
					if (atr[0] != 59 || atr[1] != 103)
					{
						A1MT6pukO = new lFqymCW8Q1M6pukOq1();
						kTS80jSiT(A1MT6pukO.eiQJfbM26e());
						if (Checksum(Hl1oHhTEV(kTS80jSiT(A1MT6pukO.NUWJrv6I3a()))))
						{
							return true;
						}
						yL7X35E33 = 1;
						AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(386);
						Debug.Print(AXEy1bisc);
						return false;
					}
					A1MT6pukO = new GY1ZaY5fnxhqDu3iI6();
				}
				return true;
			}
			catch (PCSCException ex)
			{
				yL7X35E33 = 256;
				AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(422) + ex.Message + ER5MIqF7oL5pcFCvml.JKSlhDfrB(34) + ex.SCardError.ToString() + ER5MIqF7oL5pcFCvml.JKSlhDfrB(42);
				Debug.Print(AXEy1bisc);
				return false;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public bool Close()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				JF23JbQXk.Disconnect(SCardReaderDisposition.Leave);
				LqDZu3iI6.Release();
				return true;
			}
			catch (PCSCException ex)
			{
				yL7X35E33 = 256;
				AXEy1bisc = ER5MIqF7oL5pcFCvml.JKSlhDfrB(446) + ex.Message + ER5MIqF7oL5pcFCvml.JKSlhDfrB(34) + ex.SCardError.ToString() + ER5MIqF7oL5pcFCvml.JKSlhDfrB(42);
				Debug.Print(AXEy1bisc);
				return false;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Personal readCitizenid(string readerName = null)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Personal personal = new Personal();
			if (Open(readerName))
			{
				kTS80jSiT(A1MT6pukO.eiQJfbM26e());
				personal.Citizenid = Hl1oHhTEV(kTS80jSiT(A1MT6pukO.NUWJrv6I3a()));
				Close();
				return personal;
			}
			return null;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Personal readAll(bool with_photo = false, string readerName = null)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Personal personal = new Personal();
			if (Open(readerName))
			{
				kTS80jSiT(A1MT6pukO.eiQJfbM26e());
				personal.Citizenid = Hl1oHhTEV(kTS80jSiT(A1MT6pukO.NUWJrv6I3a()));
				personal.Info = Hl1oHhTEV(kTS80jSiT(A1MT6pukO.kyfJzGM795()));
				personal.Address = Hl1oHhTEV(kTS80jSiT(A1MT6pukO.A034MG8C47()));
				personal.Issue_Expire = Hl1oHhTEV(kTS80jSiT(A1MT6pukO.FLO4JfqifV()));
				if (with_photo)
				{
					personal.PhotoRaw = tvRdiySJN();
				}
				Close();
				return personal;
			}
			return null;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Personal readAllPhoto(string readerName = null)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return readAll(with_photo: true, readerName);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public bool Checksum(string Citizenid)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			int num = 0;
			if (Citizenid.Length != 13)
			{
				return false;
			}
			for (int i = 0; i < 12; i++)
			{
				num += int.Parse(Citizenid.Substring(i, 1)) * (13 - i);
			}
			if ((11 - num % 11) % 10 == int.Parse(Citizenid.Substring(12, 1)))
			{
				return true;
			}
			return false;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public ThaiIDCard()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			base..ctor();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[CompilerGenerated]
		private void Ob5FfJR80(object P_0, CardStatusEventArgs P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			jom5BJtjh(P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[CompilerGenerated]
		private void vGTcF62WP(object P_0, CardStatusEventArgs P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Q5SYSnpHJ(ER5MIqF7oL5pcFCvml.JKSlhDfrB(472), P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		static ThaiIDCard()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			h1ZBaYfnx = ContextFactory.Instance;
		}
	}
}
internal class <Module>{40C5E004-ED68-4E35-998C-2BF90AF50739}
{
}
namespace uTyiZQdy7Yhwa4CWnc
{
	internal class p5E33A8gjf5s3GeG9K
	{
		internal delegate void SFU4mbT3GMret7THonf(object o);

		internal static Module aIqE7oL5p;

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void mI3xbvUULNEis(int typemdt)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Type type = aIqE7oL5p.ResolveType(33554432 + typemdt);
			FieldInfo[] fields = type.GetFields();
			foreach (FieldInfo fieldInfo in fields)
			{
				MethodInfo method = (MethodInfo)aIqE7oL5p.ResolveMethod(fieldInfo.MetadataToken + 100663296);
				fieldInfo.SetValue(null, (MulticastDelegate)Delegate.CreateDelegate(type, method));
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public p5E33A8gjf5s3GeG9K()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			base..ctor();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		static p5E33A8gjf5s3GeG9K()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
			aIqE7oL5p = typeof(p5E33A8gjf5s3GeG9K).Assembly.ManifestModule;
		}
	}
}
namespace wOoivnck48bv66Xx6l
{
	internal class ER5MIqF7oL5pcFCvml
	{
		internal class Ynf1aqBxsNGfLU1bDU : Attribute
		{
			internal class zghU6aZrkE35pPUbkp<T>
			{
				[MethodImpl(MethodImplOptions.NoInlining)]
				public zghU6aZrkE35pPUbkp()
				{
					while (false)
					{
						_ = ((object[])null)[0];
					}
					ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
					base..ctor();
				}
			}

			[MethodImpl(MethodImplOptions.NoInlining)]
			[Ynf1aqBxsNGfLU1bDU(typeof(zghU6aZrkE35pPUbkp<object>[]))]
			public Ynf1aqBxsNGfLU1bDU(object P_0)
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
				base..ctor();
			}
		}

		internal class c0Y3hj32WE298MPmI9
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
			internal static void ce4DmfsmSrOT856tDgfrkMb()
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				if (PBIJOrI3AN(Convert.ToBase64String(aIvziVeFm.GetName().GetPublicKeyToken()), ER5MIqF7oL5pcFCvml.JKSlhDfrB(498)) != ER5MIqF7oL5pcFCvml.JKSlhDfrB(504))
				{
					while (true)
					{
						ce4DmfsmSrOT856tDgfrkMb();
					}
				}
			}

			[MethodImpl(MethodImplOptions.NoInlining)]
			[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
			internal static string PBIJOrI3AN(string P_0, string P_1)
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				byte[] bytes = Encoding.Unicode.GetBytes(P_0);
				byte[] array = bytes;
				byte[] key = new byte[32]
				{
					82, 102, 104, 110, 32, 77, 24, 34, 118, 181,
					51, 17, 18, 51, 12, 109, 10, 32, 77, 24,
					34, 158, 161, 41, 97, 28, 118, 181, 5, 25,
					1, 88
				};
				byte[] iV = W9dPLvyyR(Encoding.Unicode.GetBytes(P_1));
				MemoryStream memoryStream = new MemoryStream();
				SymmetricAlgorithm symmetricAlgorithm = C40nY3hj2();
				symmetricAlgorithm.Key = key;
				symmetricAlgorithm.IV = iV;
				CryptoStream cryptoStream = new CryptoStream(memoryStream, symmetricAlgorithm.CreateEncryptor(), CryptoStreamMode.Write);
				cryptoStream.Write(array, 0, array.Length);
				cryptoStream.Close();
				return Convert.ToBase64String(memoryStream.ToArray());
			}

			[MethodImpl(MethodImplOptions.NoInlining)]
			public c0Y3hj32WE298MPmI9()
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				ETI8tSy9NUwElCcMTO.P7CxbvUz56ZVI();
				base..ctor();
			}
		}

		[UnmanagedFunctionPointer(CallingConvention.StdCall)]
		internal delegate uint lLvyyR1dl6FcuAM7ZN(IntPtr classthis, IntPtr comp, IntPtr info, [MarshalAs(UnmanagedType.U4)] uint flags, IntPtr nativeEntry, ref uint nativeSizeOfCode);

		[UnmanagedFunctionPointer(CallingConvention.StdCall)]
		private delegate IntPtr CnRtJluKShDfrBxANE();

		internal struct UexFLMT43TjPjL5fmx
		{
			internal bool MKdJKIWVqy;

			internal byte[] n2WJDjdlSD;
		}

		[Flags]
		private enum AiYTCB6a8ZAZw0BNtY
		{

		}

		private static uint[] Rx6JMpYEng;

		private static bool GaoJJtqSXa;

		private static byte[] Gd0JjcK8eA;

		private static byte[] dtNJ9xs775;

		private static byte[] MvQJSwAayP;

		private static IntPtr zvwJLUrpwm;

		private static object NDWJYRvTKr;

		private static int Sq7JFvfn3X;

		private static long jeaJcuCKYV;

		internal static lLvyyR1dl6FcuAM7ZN Hb7JBK02Fs;

		internal static Hashtable mFOJHdOA1E;

		private static bool FKyJTBXupO;

		private static IntPtr Hd3J5nMtSH;

		private static Assembly aIvziVeFm;

		private static byte[] cB7Jv5w0LL;

		private static int mJcJoaS5KT;

		internal static lLvyyR1dl6FcuAM7ZN I4lJZEmUw5;

		private static IntPtr tawJyLIdBb;

		private static bool icSJ4rnbG9;

		private static int hACJ6Yim29;

		private static bool B7lJuHx5jc;

		private static long mK4J31Fbqj;

		private static bool YmkJ8pePE6;

		private static int[] QucJW3W2xc;

		private static bool eZ1rLUbqA;

		[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
		private static bool dLmJXd1BWO;

		private static SortedList Wi7JdeH7vN;

		private static int kolJ1rDlnc;

		[MethodImpl(MethodImplOptions.NoInlining)]
		static ER5MIqF7oL5pcFCvml()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			eZ1rLUbqA = false;
			aIvziVeFm = typeof(ER5MIqF7oL5pcFCvml).Assembly;
			Rx6JMpYEng = new uint[64]
			{
				3614090360u, 3905402710u, 606105819u, 3250441966u, 4118548399u, 1200080426u, 2821735955u, 4249261313u, 1770035416u, 2336552879u,
				4294925233u, 2304563134u, 1804603682u, 4254626195u, 2792965006u, 1236535329u, 4129170786u, 3225465664u, 643717713u, 3921069994u,
				3593408605u, 38016083u, 3634488961u, 3889429448u, 568446438u, 3275163606u, 4107603335u, 1163531501u, 2850285829u, 4243563512u,
				1735328473u, 2368359562u, 4294588738u, 2272392833u, 1839030562u, 4259657740u, 2763975236u, 1272893353u, 4139469664u, 3200236656u,
				681279174u, 3936430074u, 3572445317u, 76029189u, 3654602809u, 3873151461u, 530742520u, 3299628645u, 4096336452u, 1126891415u,
				2878612391u, 4237533241u, 1700485571u, 2399980690u, 4293915773u, 2240044497u, 1873313359u, 4264355552u, 2734768916u, 1309151649u,
				4149444226u, 3174756917u, 718787259u, 3951481745u
			};
			GaoJJtqSXa = false;
			icSJ4rnbG9 = false;
			cB7Jv5w0LL = new byte[0];
			Gd0JjcK8eA = new byte[0];
			dtNJ9xs775 = new byte[0];
			MvQJSwAayP = new byte[0];
			zvwJLUrpwm = IntPtr.Zero;
			Hd3J5nMtSH = IntPtr.Zero;
			NDWJYRvTKr = new string[0];
			QucJW3W2xc = new int[0];
			mJcJoaS5KT = 1;
			YmkJ8pePE6 = false;
			Wi7JdeH7vN = new SortedList();
			Sq7JFvfn3X = 0;
			jeaJcuCKYV = 0L;
			Hb7JBK02Fs = null;
			I4lJZEmUw5 = null;
			mK4J31Fbqj = 0L;
			kolJ1rDlnc = 0;
			B7lJuHx5jc = false;
			FKyJTBXupO = false;
			hACJ6Yim29 = 0;
			tawJyLIdBb = IntPtr.Zero;
			dLmJXd1BWO = false;
			mFOJHdOA1E = new Hashtable();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void gRfxbvUeFDVL3()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static byte[] nFCVvmlKO(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			uint[] array = new uint[16];
			int num = 448 - P_0.Length * 8 % 512;
			uint num2 = (uint)((num + 512) % 512);
			if (num2 == 0)
			{
				num2 = 512u;
			}
			uint num3 = (uint)(P_0.Length + num2 / 8 + 8);
			ulong num4 = (ulong)P_0.Length * 8uL;
			byte[] array2 = new byte[num3];
			for (int i = 0; i < P_0.Length; i++)
			{
				array2[i] = P_0[i];
			}
			array2[P_0.Length] |= 128;
			for (int num5 = 8; num5 > 0; num5--)
			{
				array2[num3 - num5] = (byte)((num4 >> (8 - num5) * 8) & 0xFF);
			}
			uint num6 = (uint)(array2.Length * 8) / 32u;
			uint num7 = 1732584193u;
			uint num8 = 4023233417u;
			uint num9 = 2562383102u;
			uint num10 = 271733878u;
			for (uint num11 = 0u; num11 < num6 / 16; num11++)
			{
				uint num12 = num11 << 6;
				for (uint num13 = 0u; num13 < 61; num13 += 4)
				{
					array[num13 >> 2] = (uint)((array2[num12 + (num13 + 3)] << 24) | (array2[num12 + (num13 + 2)] << 16) | (array2[num12 + (num13 + 1)] << 8) | array2[num12 + num13]);
				}
				uint num14 = num7;
				uint num15 = num8;
				uint num16 = num9;
				uint num17 = num10;
				jivQnk48b(ref num7, num8, num9, num10, 0u, 7, 1u, array);
				jivQnk48b(ref num10, num7, num8, num9, 1u, 12, 2u, array);
				jivQnk48b(ref num9, num10, num7, num8, 2u, 17, 3u, array);
				jivQnk48b(ref num8, num9, num10, num7, 3u, 22, 4u, array);
				jivQnk48b(ref num7, num8, num9, num10, 4u, 7, 5u, array);
				jivQnk48b(ref num10, num7, num8, num9, 5u, 12, 6u, array);
				jivQnk48b(ref num9, num10, num7, num8, 6u, 17, 7u, array);
				jivQnk48b(ref num8, num9, num10, num7, 7u, 22, 8u, array);
				jivQnk48b(ref num7, num8, num9, num10, 8u, 7, 9u, array);
				jivQnk48b(ref num10, num7, num8, num9, 9u, 12, 10u, array);
				jivQnk48b(ref num9, num10, num7, num8, 10u, 17, 11u, array);
				jivQnk48b(ref num8, num9, num10, num7, 11u, 22, 12u, array);
				jivQnk48b(ref num7, num8, num9, num10, 12u, 7, 13u, array);
				jivQnk48b(ref num10, num7, num8, num9, 13u, 12, 14u, array);
				jivQnk48b(ref num9, num10, num7, num8, 14u, 17, 15u, array);
				jivQnk48b(ref num8, num9, num10, num7, 15u, 22, 16u, array);
				d66eXx6lr(ref num7, num8, num9, num10, 1u, 5, 17u, array);
				d66eXx6lr(ref num10, num7, num8, num9, 6u, 9, 18u, array);
				d66eXx6lr(ref num9, num10, num7, num8, 11u, 14, 19u, array);
				d66eXx6lr(ref num8, num9, num10, num7, 0u, 20, 20u, array);
				d66eXx6lr(ref num7, num8, num9, num10, 5u, 5, 21u, array);
				d66eXx6lr(ref num10, num7, num8, num9, 10u, 9, 22u, array);
				d66eXx6lr(ref num9, num10, num7, num8, 15u, 14, 23u, array);
				d66eXx6lr(ref num8, num9, num10, num7, 4u, 20, 24u, array);
				d66eXx6lr(ref num7, num8, num9, num10, 9u, 5, 25u, array);
				d66eXx6lr(ref num10, num7, num8, num9, 14u, 9, 26u, array);
				d66eXx6lr(ref num9, num10, num7, num8, 3u, 14, 27u, array);
				d66eXx6lr(ref num8, num9, num10, num7, 8u, 20, 28u, array);
				d66eXx6lr(ref num7, num8, num9, num10, 13u, 5, 29u, array);
				d66eXx6lr(ref num10, num7, num8, num9, 2u, 9, 30u, array);
				d66eXx6lr(ref num9, num10, num7, num8, 7u, 14, 31u, array);
				d66eXx6lr(ref num8, num9, num10, num7, 12u, 20, 32u, array);
				Ff1paqxsN(ref num7, num8, num9, num10, 5u, 4, 33u, array);
				Ff1paqxsN(ref num10, num7, num8, num9, 8u, 11, 34u, array);
				Ff1paqxsN(ref num9, num10, num7, num8, 11u, 16, 35u, array);
				Ff1paqxsN(ref num8, num9, num10, num7, 14u, 23, 36u, array);
				Ff1paqxsN(ref num7, num8, num9, num10, 1u, 4, 37u, array);
				Ff1paqxsN(ref num10, num7, num8, num9, 4u, 11, 38u, array);
				Ff1paqxsN(ref num9, num10, num7, num8, 7u, 16, 39u, array);
				Ff1paqxsN(ref num8, num9, num10, num7, 10u, 23, 40u, array);
				Ff1paqxsN(ref num7, num8, num9, num10, 13u, 4, 41u, array);
				Ff1paqxsN(ref num10, num7, num8, num9, 0u, 11, 42u, array);
				Ff1paqxsN(ref num9, num10, num7, num8, 3u, 16, 43u, array);
				Ff1paqxsN(ref num8, num9, num10, num7, 6u, 23, 44u, array);
				Ff1paqxsN(ref num7, num8, num9, num10, 9u, 4, 45u, array);
				Ff1paqxsN(ref num10, num7, num8, num9, 12u, 11, 46u, array);
				Ff1paqxsN(ref num9, num10, num7, num8, 15u, 16, 47u, array);
				Ff1paqxsN(ref num8, num9, num10, num7, 2u, 23, 48u, array);
				QfL2U1bDU(ref num7, num8, num9, num10, 0u, 6, 49u, array);
				QfL2U1bDU(ref num10, num7, num8, num9, 7u, 10, 50u, array);
				QfL2U1bDU(ref num9, num10, num7, num8, 14u, 15, 51u, array);
				QfL2U1bDU(ref num8, num9, num10, num7, 5u, 21, 52u, array);
				QfL2U1bDU(ref num7, num8, num9, num10, 12u, 6, 53u, array);
				QfL2U1bDU(ref num10, num7, num8, num9, 3u, 10, 54u, array);
				QfL2U1bDU(ref num9, num10, num7, num8, 10u, 15, 55u, array);
				QfL2U1bDU(ref num8, num9, num10, num7, 1u, 21, 56u, array);
				QfL2U1bDU(ref num7, num8, num9, num10, 8u, 6, 57u, array);
				QfL2U1bDU(ref num10, num7, num8, num9, 15u, 10, 58u, array);
				QfL2U1bDU(ref num9, num10, num7, num8, 6u, 15, 59u, array);
				QfL2U1bDU(ref num8, num9, num10, num7, 13u, 21, 60u, array);
				QfL2U1bDU(ref num7, num8, num9, num10, 4u, 6, 61u, array);
				QfL2U1bDU(ref num10, num7, num8, num9, 11u, 10, 62u, array);
				QfL2U1bDU(ref num9, num10, num7, num8, 2u, 15, 63u, array);
				QfL2U1bDU(ref num8, num9, num10, num7, 9u, 21, 64u, array);
				num7 += num14;
				num8 += num15;
				num9 += num16;
				num10 += num17;
			}
			byte[] array3 = new byte[16];
			Array.Copy(BitConverter.GetBytes(num7), 0, array3, 0, 4);
			Array.Copy(BitConverter.GetBytes(num8), 0, array3, 4, 4);
			Array.Copy(BitConverter.GetBytes(num9), 0, array3, 8, 4);
			Array.Copy(BitConverter.GetBytes(num10), 0, array3, 12, 4);
			return array3;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void jivQnk48b(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + zghmU6ark(P_0 + ((P_1 & P_2) | (~P_1 & P_3)) + P_7[P_4] + Rx6JMpYEng[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void d66eXx6lr(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + zghmU6ark(P_0 + ((P_1 & P_3) | (P_2 & ~P_3)) + P_7[P_4] + Rx6JMpYEng[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void Ff1paqxsN(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + zghmU6ark(P_0 + (P_1 ^ P_2 ^ P_3) + P_7[P_4] + Rx6JMpYEng[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void QfL2U1bDU(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + zghmU6ark(P_0 + (P_2 ^ (P_1 | ~P_3)) + P_7[P_4] + Rx6JMpYEng[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static uint zghmU6ark(uint P_0, ushort P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return (P_0 >> 32 - P_1) | (P_0 << (int)P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static bool y35RpPUbk()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (!GaoJJtqSXa)
			{
				iE2798MPm();
				GaoJJtqSXa = true;
			}
			return icSJ4rnbG9;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static SymmetricAlgorithm C40nY3hj2()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			SymmetricAlgorithm symmetricAlgorithm = null;
			if (y35RpPUbk())
			{
				return new AesCryptoServiceProvider();
			}
			try
			{
				return new RijndaelManaged();
			}
			catch
			{
				return (SymmetricAlgorithm)Activator.CreateInstance("System.Core, Version=3.5.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089", "System.Security.Cryptography.AesCryptoServiceProvider").Unwrap();
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void iE2798MPm()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				icSJ4rnbG9 = CryptoConfig.AllowOnlyFipsAlgorithms;
			}
			catch
			{
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static byte[] W9dPLvyyR(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (!y35RpPUbk())
			{
				return new MD5CryptoServiceProvider().ComputeHash(P_0);
			}
			return nFCVvmlKO(P_0);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static uint FZNC2nRtJ(uint P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return (uint)"{11111-22222-10009-11112}".Length;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
		static string JKSlhDfrB(int P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			int num = 375;
			uint num19 = default(uint);
			int num5 = default(int);
			byte[] array = default(byte[]);
			int num7 = default(int);
			byte[] array3 = default(byte[]);
			int num32 = default(int);
			int num11 = default(int);
			byte[] array6 = default(byte[]);
			int num17 = default(int);
			uint num18 = default(uint);
			uint num9 = default(uint);
			byte[] array5 = default(byte[]);
			byte[] array4 = default(byte[]);
			uint num34 = default(uint);
			uint num6 = default(uint);
			BinaryReader binaryReader = default(BinaryReader);
			int num8 = default(int);
			byte[] array2 = default(byte[]);
			int num33 = default(int);
			uint num13 = default(uint);
			int num12 = default(int);
			uint num31 = default(uint);
			CryptoStream cryptoStream = default(CryptoStream);
			int num15 = default(int);
			byte[] array7 = default(byte[]);
			MemoryStream memoryStream = default(MemoryStream);
			int num10 = default(int);
			int num14 = default(int);
			ICryptoTransform transform = default(ICryptoTransform);
			SymmetricAlgorithm symmetricAlgorithm = default(SymmetricAlgorithm);
			int num16 = default(int);
			int num4 = default(int);
			while (true)
			{
				int num2 = num;
				while (true)
				{
					IL_0c2b:
					int num3 = num2;
					while (true)
					{
						switch (num3)
						{
						case 120:
							num19 <<= 8;
							num2 = 221;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 117;
						case 117:
							num5 = 50 + 56;
							num2 = 76;
							goto IL_0c2b;
						case 429:
							break;
						case 360:
							goto IL_006d;
						case 59:
						case 368:
							array[8] = (byte)num5;
							num2 = 164;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 319;
						case 319:
							num7 = 152 - 50;
							num2 = 71;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 370;
						case 167:
							array[23] = (byte)num5;
							num = 290;
							goto end_IL_0c2f;
						case 309:
							array[27] = 98;
							num2 = 267;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 46;
						case 163:
							array3[15] = (byte)num7;
							num = 319;
							goto end_IL_0c2f;
						case 265:
							num32 = 0;
							num3 = 22;
							continue;
						case 181:
							num7 = 162 - 54;
							num2 = 111;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 288;
						case 288:
							num5 = 96 + 77;
							num = 295;
							goto end_IL_0c2f;
						case 108:
							num7 = 89 + 8;
							num = 197;
							goto end_IL_0c2f;
						case 222:
							array3[0] = (byte)num7;
							num3 = 264;
							continue;
						case 361:
							num7 = 164 - 54;
							num2 = 337;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 389;
						case 389:
							array3[6] = (byte)num7;
							num = 435;
							goto end_IL_0c2f;
						case 366:
							num5 = 171 - 57;
							num2 = 202;
							goto IL_0c2b;
						case 419:
							array[9] = (byte)num5;
							num = 134;
							goto end_IL_0c2f;
						case 121:
							num11 = 0;
							num2 = 428;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 97;
						case 97:
							array6[num17 + 2] = (byte)((num18 & 0xFF0000) >> 16);
							num2 = 338;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 156;
						case 156:
							array3[11] = 38;
							num = 432;
							goto end_IL_0c2f;
						case 256:
							array[30] = 171;
							num3 = 382;
							continue;
						case 340:
							num9 = 0u;
							num2 = 229;
							goto IL_0c2b;
						case 252:
							array[18] = (byte)num5;
							num = 334;
							goto end_IL_0c2f;
						case 247:
							num5 = 181 - 60;
							num = 48;
							goto end_IL_0c2f;
						case 173:
							if (array5 != null)
							{
								num = 433;
								goto end_IL_0c2f;
							}
							goto case 98;
						case 179:
							array[21] = (byte)num5;
							num2 = 16;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 237;
						case 102:
							num5 = 238 - 79;
							num3 = 69;
							continue;
						case 53:
							num7 = 253 - 84;
							num2 = 307;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 294;
						case 294:
							num7 = 188 - 124;
							num2 = 343;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 82;
						case 245:
							num5 = 225 - 75;
							num2 = 384;
							goto IL_0c2b;
						case 154:
							array[5] = 100;
							num = 158;
							goto end_IL_0c2f;
						case 86:
							array[30] = 170;
							num2 = 27;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 367;
						case 302:
							array3[8] = 104;
							num3 = 186;
							continue;
						case 329:
							array4[11] = array5[5];
							num = 296;
							goto end_IL_0c2f;
						case 278:
							csO5cFqlHIaBF80IiK(array4);
							num2 = 318;
							goto IL_0c2b;
						case 85:
							num5 = 12 + 104;
							num2 = 157;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 386;
						case 386:
							array6[num17] = (byte)(num18 & 0xFFu);
							num3 = 94;
							continue;
						case 241:
							num5 = 45 + 26;
							num2 = 332;
							goto IL_0c2b;
						case 323:
							num5 = 107 + 49;
							num3 = 107;
							continue;
						case 387:
							num34 = num6 ^ num19;
							num = 344;
							goto end_IL_0c2f;
						case 207:
							array[9] = 163;
							num = 145;
							goto end_IL_0c2f;
						case 415:
							num5 = 191 - 63;
							num2 = 139;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 269;
						case 269:
							array[7] = 105;
							num3 = 438;
							continue;
						case 206:
							array3[9] = 143;
							num3 = 181;
							continue;
						case 373:
							num7 = 159 - 53;
							num = 363;
							goto end_IL_0c2f;
						case 243:
							array[5] = (byte)num5;
							num3 = 193;
							continue;
						case 248:
							num5 = 167 - 55;
							num = 251;
							goto end_IL_0c2f;
						case 254:
							array[29] = 166;
							num3 = 273;
							continue;
						case 286:
							array[3] = (byte)num5;
							num3 = 400;
							continue;
						case 369:
							array3[0] = 109;
							num3 = 79;
							continue;
						case 7:
							array3[4] = 161;
							num2 = 188;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 203;
						case 203:
							num5 = 156 - 52;
							num3 = 9;
							continue;
						case 15:
							array[16] = 168;
							num2 = 310;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 314;
						case 314:
							array3[13] = (byte)num7;
							num2 = 13;
							goto IL_0c2b;
						case 16:
							num5 = 48 - 11;
							num2 = 165;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 23;
						case 23:
							binaryReader = new BinaryReader((Stream)Ossp97Sry5Dj3aeBaK(aIvziVeFm, "x74ntrNpHcax3N5M2p.AZVw68ZTMMBdft3yg1"));
							num3 = 18;
							continue;
						case 66:
							num5 = 79 + 52;
							num2 = 195;
							goto IL_0c2b;
						case 287:
							num5 = 218 - 72;
							num3 = 385;
							continue;
						case 347:
							array3[13] = 132;
							num = 373;
							goto end_IL_0c2f;
						case 198:
							num8 = array2.Length % 4;
							num3 = 70;
							continue;
						case 178:
							array[27] = 207;
							num2 = 331;
							goto IL_0c2b;
						case 6:
							num33++;
							num = 105;
							goto end_IL_0c2f;
						case 423:
							array[11] = (byte)num5;
							num2 = 36;
							goto IL_0c2b;
						case 303:
							array[16] = 65;
							num2 = 15;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 380;
						case 380:
							array[4] = 103;
							num = 128;
							goto end_IL_0c2f;
						case 422:
							num7 = 171 - 57;
							num = 113;
							goto end_IL_0c2f;
						case 99:
							num5 = 1 + 105;
							num2 = 327;
							goto IL_0c2b;
						case 152:
							num7 = 199 - 66;
							num2 = 89;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 25;
						case 25:
							array3[7] = 126;
							num2 = 365;
							goto IL_0c2b;
						case 221:
							num19 |= array2[array2.Length - (1 + num11)];
							num2 = 407;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 440;
						case 440:
							array[5] = (byte)num5;
							num2 = 170;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 209;
						case 209:
							array[24] = (byte)num5;
							num = 85;
							goto end_IL_0c2f;
						case 199:
							num5 = 226 - 75;
							num = 279;
							goto end_IL_0c2f;
						case 367:
							num6 += num13;
							num2 = 149;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 264;
						case 264:
							array3[0] = 46;
							num = 112;
							goto end_IL_0c2f;
						case 413:
							num5 = 174 - 58;
							num3 = 11;
							continue;
						case 183:
							array[1] = 209;
							num2 = 90;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 230;
						case 226:
							array[31] = (byte)num5;
							num2 = 248;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 431;
						case 431:
							num5 = 29 + 69;
							num = 179;
							goto end_IL_0c2f;
						case 234:
							array[15] = (byte)num5;
							num = 299;
							goto end_IL_0c2f;
						case 272:
							array[16] = 45;
							num2 = 99;
							goto IL_0c2b;
						case 281:
							array[2] = (byte)num5;
							num2 = 201;
							goto IL_0c2b;
						case 233:
							num5 = 11 + 69;
							num2 = 143;
							goto IL_0c2b;
						case 311:
							num7 = 95 + 31;
							num = 130;
							goto end_IL_0c2f;
						case 201:
							num5 = 91 + 68;
							num3 = 298;
							continue;
						case 237:
							array3[15] = 128;
							num3 = 225;
							continue;
						case 73:
							array[25] = 166;
							num = 84;
							goto end_IL_0c2f;
						case 107:
							array[27] = (byte)num5;
							num2 = 62;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 126;
						case 134:
							num5 = 189 - 85;
							num = 74;
							goto end_IL_0c2f;
						case 32:
							num5 = 238 - 79;
							num3 = 232;
							continue;
						case 13:
							num7 = 78 + 99;
							num2 = 275;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 435;
						case 435:
							array3[6] = 164;
							num = 50;
							goto end_IL_0c2f;
						case 219:
							array[13] = 84;
							num2 = 391;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 289;
						case 289:
							array3[12] = (byte)num7;
							num = 326;
							goto end_IL_0c2f;
						case 217:
						case 268:
							if (num12 >= num8)
							{
								num2 = 110;
								if (0 == 0)
								{
									goto IL_0c2b;
								}
								goto case 129;
							}
							if (num12 > 0)
							{
								num = 231;
								goto end_IL_0c2f;
							}
							goto case 441;
						case 129:
							array[20] = (byte)num5;
							num = 282;
							goto end_IL_0c2f;
						case 197:
							array3[3] = (byte)num7;
							num = 78;
							goto end_IL_0c2f;
						case 384:
							array[19] = (byte)num5;
							num = 259;
							goto end_IL_0c2f;
						case 433:
							if (array5.Length > 0)
							{
								num2 = 58;
								if (0 == 0)
								{
									goto IL_0c2b;
								}
								goto case 345;
							}
							goto case 98;
						case 345:
							array3[9] = 58;
							num2 = 40;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 318;
						case 318:
							array5 = (byte[])Ti480rfGZgDY3sVJfU(e0I2KhZ0ZRvHjytujl(aIvziVeFm));
							num3 = 173;
							continue;
						case 257:
							array3[8] = 144;
							num3 = 302;
							continue;
						case 231:
							num31 <<= 8;
							num3 = 414;
							continue;
						case 17:
							num5 = 5 + 35;
							num3 = 360;
							continue;
						case 114:
							array[6] = 152;
							num2 = 212;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 50;
						case 50:
							array3[6] = 192;
							num3 = 351;
							continue;
						case 182:
							array[29] = (byte)num5;
							goto case 140;
						default:
							num2 = 140;
							goto IL_0c2b;
						case 36:
							num5 = 208 - 85;
							num = 151;
							goto end_IL_0c2f;
						case 358:
							num5 = 133 + 52;
							num2 = 209;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 274;
						case 274:
							array3 = new byte[16];
							num3 = 369;
							continue;
						case 336:
							BjoN2TKoy9gjBUTyo6(cryptoStream);
							num2 = 191;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 277;
						case 277:
							array3[11] = (byte)num7;
							num3 = 10;
							continue;
						case 46:
							num7 = 232 - 77;
							num3 = 127;
							continue;
						case 81:
							num5 = 57 + 39;
							num2 = 52;
							goto IL_0c2b;
						case 285:
							num6 += num13;
							num = 210;
							goto end_IL_0c2f;
						case 321:
							vc0Tua1UeFqbTu59Jc(binaryReader);
							num = 65;
							goto end_IL_0c2f;
						case 244:
							num5 = 18 + 120;
							num = 12;
							goto end_IL_0c2f;
						case 128:
							num5 = 139 - 46;
							num2 = 220;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 35;
						case 35:
							array[11] = 154;
							num2 = 341;
							goto IL_0c2b;
						case 273:
							array[29] = 182;
							num2 = 106;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 33;
						case 33:
							array3[15] = (byte)num7;
							num2 = 93;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 44;
						case 44:
							num5 = 136 - 45;
							num = 255;
							goto end_IL_0c2f;
						case 39:
							array[14] = 191;
							num3 = 399;
							continue;
						case 187:
							array[12] = (byte)num5;
							num = 415;
							goto end_IL_0c2f;
						case 116:
							array[17] = (byte)num5;
							num3 = 425;
							continue;
						case 267:
							array[27] = 97;
							num2 = 178;
							goto IL_0c2b;
						case 40:
							array3[10] = 157;
							num2 = 189;
							goto IL_0c2b;
						case 171:
							array[22] = (byte)num5;
							num2 = 30;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 308;
						case 308:
							num5 = 138 - 46;
							num3 = 8;
							continue;
						case 296:
							array4[13] = array5[6];
							num2 = 176;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 123;
						case 123:
							array[13] = (byte)num5;
							num3 = 236;
							continue;
						case 54:
							array[13] = (byte)num5;
							num3 = 219;
							continue;
						case 332:
							array[20] = (byte)num5;
							num2 = 77;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 177;
						case 177:
							array3[6] = 156;
							num2 = 270;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 49;
						case 111:
							array3[9] = (byte)num7;
							num3 = 416;
							continue;
						case 400:
							array[3] = 148;
							num2 = 21;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 300;
						case 300:
							num5 = 17 + 122;
							num2 = 126;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 142;
						case 142:
							num5 = 186 - 84;
							num2 = 96;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 363;
						case 363:
							array3[13] = (byte)num7;
							num = 350;
							goto end_IL_0c2f;
						case 138:
							num5 = 120 + 59;
							num2 = 87;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 146;
						case 146:
							num5 = 87 + 42;
							num3 = 24;
							continue;
						case 414:
							num32 += 8;
							num3 = 441;
							continue;
						case 158:
							array[5] = 144;
							num = 250;
							goto end_IL_0c2f;
						case 437:
							num31 = 255u;
							num3 = 265;
							continue;
						case 220:
							array[4] = (byte)num5;
							num3 = 142;
							continue;
						case 420:
							array[17] = 99;
							num = 288;
							goto end_IL_0c2f;
						case 195:
							array[3] = (byte)num5;
							num2 = 57;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 334;
						case 334:
							num5 = 80 + 116;
							num = 184;
							goto end_IL_0c2f;
						case 202:
							array[31] = (byte)num5;
							num = 413;
							goto end_IL_0c2f;
						case 322:
							array[22] = 128;
							num3 = 194;
							continue;
						case 410:
							array3[3] = 207;
							num3 = 108;
							continue;
						case 20:
							num5 = 173 - 57;
							num = 123;
							goto end_IL_0c2f;
						case 143:
							array[23] = (byte)num5;
							num3 = 51;
							continue;
						case 427:
							array3[4] = (byte)num7;
							num = 284;
							goto end_IL_0c2f;
						case 141:
							num7 = 131 - 43;
							num = 34;
							goto end_IL_0c2f;
						case 316:
							num15 = array7.Length / 4;
							num3 = 185;
							continue;
						case 239:
							if (num8 > 0)
							{
								num2 = 312;
								goto IL_0c2b;
							}
							goto case 340;
						case 353:
							array3[9] = (byte)num7;
							num = 345;
							goto end_IL_0c2f;
						case 392:
							array[30] = (byte)num5;
							num3 = 86;
							continue;
						case 223:
							num5 = 239 - 79;
							num2 = 280;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 136;
						case 396:
							array[29] = (byte)num5;
							num2 = 254;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 189;
						case 189:
							num7 = 109 + 32;
							num = 424;
							goto end_IL_0c2f;
						case 76:
							array[21] = (byte)num5;
							num2 = 102;
							goto IL_0c2b;
						case 405:
							array[31] = (byte)num5;
							num2 = 119;
							goto IL_0c2b;
						case 337:
							array3[8] = (byte)num7;
							num3 = 258;
							continue;
						case 232:
							array[10] = (byte)num5;
							num2 = 393;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 48;
						case 48:
							array[1] = (byte)num5;
							num2 = 183;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 24;
						case 24:
							array[22] = (byte)num5;
							num2 = 199;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 149;
						case 355:
							array4[5] = array5[2];
							num = 82;
							goto end_IL_0c2f;
						case 430:
							num5 = 34 + 37;
							num2 = 211;
							goto IL_0c2b;
						case 180:
							array[23] = 72;
							num2 = 218;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 325;
						case 325:
							array3[1] = 154;
							num3 = 330;
							continue;
						case 292:
							array[3] = (byte)num5;
							num = 66;
							goto end_IL_0c2f;
						case 11:
							array[31] = (byte)num5;
							num = 421;
							goto end_IL_0c2f;
						case 238:
							array[28] = (byte)num5;
							num2 = 31;
							goto IL_0c2b;
						case 359:
							num7 = 104 + 85;
							num = 104;
							goto end_IL_0c2f;
						case 14:
							array3[3] = (byte)num7;
							num3 = 381;
							continue;
						case 191:
							array2 = dtNJ9xs775;
							num3 = 198;
							continue;
						case 284:
							num7 = 175 - 58;
							num3 = 136;
							continue;
						case 119:
							array7 = array;
							num3 = 274;
							continue;
						case 34:
							array3[12] = (byte)num7;
							num2 = 152;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 247;
						case 379:
							num7 = 54 + 113;
							num2 = 41;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 225;
						case 225:
							num7 = 94 + 103;
							num = 33;
							goto end_IL_0c2f;
						case 344:
							num12 = 0;
							num2 = 268;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 436;
						case 436:
							array[15] = (byte)num5;
							num3 = 17;
							continue;
						case 124:
							array[30] = 171;
							num2 = 256;
							goto IL_0c2b;
						case 327:
							array[17] = (byte)num5;
							num2 = 429;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 424;
						case 424:
							array3[10] = (byte)num7;
							num3 = 359;
							continue;
						case 306:
							array[2] = (byte)num5;
							num2 = 228;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 217;
						case 313:
							num7 = 187 + 20;
							num2 = 277;
							goto IL_0c2b;
						case 381:
							num7 = 57 + 103;
							num3 = 320;
							continue;
						case 385:
							array[12] = (byte)num5;
							num2 = 1;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 310;
						case 310:
							num5 = 73 + 110;
							num3 = 215;
							continue;
						case 157:
							array[25] = (byte)num5;
							num = 0;
							goto end_IL_0c2f;
						case 166:
							array4[9] = array5[4];
							num = 329;
							goto end_IL_0c2f;
						case 55:
							BjoN2TKoy9gjBUTyo6(memoryStream);
							num3 = 336;
							continue;
						case 139:
							array[12] = (byte)num5;
							num = 5;
							goto end_IL_0c2f;
						case 70:
							num10 = array2.Length / 4;
							num2 = 88;
							goto IL_0c2b;
						case 408:
							array[8] = (byte)num5;
							num = 409;
							goto end_IL_0c2f;
						case 169:
							array[22] = (byte)num5;
							num3 = 161;
							continue;
						case 377:
							array3[4] = (byte)num7;
							num = 262;
							goto end_IL_0c2f;
						case 135:
							num19 = 0u;
							num = 121;
							goto end_IL_0c2f;
						case 43:
							num7 = 232 - 108;
							num2 = 371;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 160;
						case 160:
							array3[13] = 210;
							num2 = 347;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 229;
						case 229:
							num14 = 0;
							num2 = 28;
							goto IL_0c2b;
						case 91:
							array[19] = 113;
							num3 = 430;
							continue;
						case 349:
							array[27] = 116;
							num2 = 323;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 6;
						case 78:
							num7 = 96 + 12;
							num = 342;
							goto end_IL_0c2f;
						case 118:
							array[0] = (byte)num5;
							num3 = 247;
							continue;
						case 168:
							num5 = 200 - 66;
							num2 = 227;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 29;
						case 29:
							num5 = 221 + 16;
							num = 306;
							goto end_IL_0c2f;
						case 190:
							num5 = 161 - 53;
							num3 = 182;
							continue;
						case 213:
							num5 = 205 - 81;
							num2 = 171;
							goto IL_0c2b;
						case 249:
							num19 = (uint)((array2[num9 + 3] << 24) | (array2[num9 + 2] << 16) | (array2[num9 + 1] << 8) | array2[num9]);
							num3 = 367;
							continue;
						case 74:
							array[9] = (byte)num5;
							num = 32;
							goto end_IL_0c2f;
						case 393:
							array[10] = 98;
							num3 = 308;
							continue;
						case 215:
							array[16] = (byte)num5;
							num2 = 272;
							goto IL_0c2b;
						case 417:
							num5 = 233 - 77;
							num2 = 398;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 176;
						case 176:
							array4[15] = array5[7];
							num3 = 98;
							continue;
						case 364:
							num7 = 229 + 24;
							num2 = 324;
							goto IL_0c2b;
						case 402:
							transform = (ICryptoTransform)vlsp9W2SqQC0tSZxQy(symmetricAlgorithm, array7, array4);
							num2 = 83;
							goto IL_0c2b;
						case 409:
							num5 = 109 + 20;
							num = 19;
							goto end_IL_0c2f;
						case 282:
							array[20] = 17;
							num3 = 196;
							continue;
						case 57:
							num5 = 130 - 43;
							num2 = 286;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 47;
						case 47:
							if (num8 <= 0)
							{
								goto IL_19d0;
							}
							num2 = 135;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 136;
						case 136:
							array3[4] = (byte)num7;
							num2 = 7;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 45;
						case 45:
							array[1] = (byte)num5;
							num2 = 204;
							goto IL_0c2b;
						case 103:
							array[26] = 162;
							num3 = 339;
							continue;
						case 262:
							array3[5] = 215;
							num2 = 53;
							goto IL_0c2b;
						case 125:
							array3[7] = 42;
							num = 46;
							goto end_IL_0c2f;
						case 165:
							array[21] = (byte)num5;
							num2 = 322;
							goto IL_0c2b;
						case 331:
							array[27] = 207;
							num2 = 349;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 290;
						case 290:
							array[24] = 122;
							num3 = 131;
							continue;
						case 236:
							num5 = 104 + 28;
							num2 = 54;
							goto IL_0c2b;
						case 84:
							num5 = 105 - 25;
							num3 = 403;
							continue;
						case 30:
							array[23] = 142;
							num3 = 61;
							continue;
						case 370:
							num5 = 173 + 54;
							num = 118;
							goto end_IL_0c2f;
						case 82:
							array4[7] = array5[3];
							num3 = 166;
							continue;
						case 188:
							num7 = 155 - 43;
							num = 377;
							goto end_IL_0c2f;
						case 18:
							YC9Xk7JQsaniiZfHNQ(PhSRwGR2qQsDxGHMpX(binaryReader), 0L);
							num2 = 115;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 363;
						case 439:
							num19 = 0u;
							num3 = 239;
							continue;
						case 186:
							array3[8] = 134;
							num3 = 200;
							continue;
						case 250:
							array[5] = 120;
							num2 = 266;
							goto IL_0c2b;
						case 204:
							array[1] = 80;
							num2 = 376;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 425;
						case 425:
							num5 = 93 + 80;
							num3 = 101;
							continue;
						case 38:
							num5 = 156 + 17;
							num2 = 234;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 163;
						case 172:
							array[14] = 132;
							num3 = 244;
							continue;
						case 242:
							array[10] = 36;
							num = 315;
							goto end_IL_0c2f;
						case 67:
							dtNJ9xs775 = array6;
							num = 159;
							goto end_IL_0c2f;
						case 89:
							array3[12] = (byte)num7;
							num = 144;
							goto end_IL_0c2f;
						case 150:
							num5 = 102 + 24;
							num = 440;
							goto end_IL_0c2f;
						case 317:
							array[6] = 40;
							num2 = 390;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 356;
						case 356:
							num7 = 101 - 34;
							num = 60;
							goto end_IL_0c2f;
						case 95:
							array[4] = 180;
							num2 = 380;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 299;
						case 72:
							array[21] = (byte)num5;
							num2 = 117;
							goto IL_0c2b;
						case 230:
						case 428:
							if (num11 >= num8)
							{
								num3 = 285;
								continue;
							}
							if (num11 > 0)
							{
								num3 = 120;
								continue;
							}
							goto case 221;
						case 77:
							num5 = 163 - 54;
							num2 = 129;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 354;
						case 354:
							array3[14] = 133;
							num = 379;
							goto end_IL_0c2f;
						case 275:
							array3[14] = (byte)num7;
							num2 = 354;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 130;
						case 130:
							array3[3] = (byte)num7;
							num = 147;
							goto end_IL_0c2f;
						case 26:
							array3[11] = 150;
							num3 = 260;
							continue;
						case 426:
							array[6] = (byte)num5;
							num2 = 317;
							goto IL_0c2b;
						case 398:
							array[21] = (byte)num5;
							num = 431;
							goto end_IL_0c2f;
						case 75:
							num7 = 192 - 64;
							num3 = 174;
							continue;
						case 335:
							array[23] = (byte)num5;
							num2 = 180;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 147;
						case 147:
							num7 = 135 - 45;
							num = 427;
							goto end_IL_0c2f;
						case 132:
							num9 = (uint)(num16 * 4);
							num = 328;
							goto end_IL_0c2f;
						case 299:
							num5 = 233 - 77;
							num2 = 395;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 279;
						case 279:
							array[22] = (byte)num5;
							num = 213;
							goto end_IL_0c2f;
						case 240:
							array3[1] = 244;
							num2 = 325;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 441;
						case 441:
							array6[num17 + num12] = (byte)((num34 & num31) >> num32);
							num2 = 301;
							goto IL_0c2b;
						case 320:
							array3[3] = (byte)num7;
							num = 410;
							goto end_IL_0c2f;
						case 304:
							num13 = 0u;
							num = 439;
							goto end_IL_0c2f;
						case 383:
							num5 = 82 + 104;
							num2 = 423;
							goto IL_0c2b;
						case 87:
							array[0] = (byte)num5;
							num3 = 370;
							continue;
						case 388:
							NVeRggQn7sICdx2Yyd(cryptoStream);
							num = 434;
							goto end_IL_0c2f;
						case 280:
							array[28] = (byte)num5;
							num2 = 168;
							goto IL_0c2b;
						case 235:
							DHvRL4mJbQ2Du6wCg0(symmetricAlgorithm, CipherMode.CBC);
							num2 = 402;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 217;
						case 406:
							num5 = 80 + 54;
							num3 = 436;
							continue;
						case 338:
							array6[num17 + 3] = (byte)((num18 & 0xFF000000u) >> 24);
							num3 = 297;
							continue;
						case 270:
							num7 = 98 + 74;
							num2 = 389;
							goto IL_0c2b;
						case 246:
							array[14] = (byte)num5;
							num2 = 172;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 93;
						case 93:
							array4 = array3;
							num = 278;
							goto end_IL_0c2f;
						case 2:
							array[12] = 245;
							num2 = 20;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 352;
						case 352:
							array[15] = 87;
							num3 = 406;
							continue;
						case 60:
							array3[2] = (byte)num7;
							num2 = 348;
							goto IL_0c2b;
						case 438:
							array[7] = 15;
							num2 = 411;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 263;
						case 263:
							num7 = 191 - 63;
							num2 = 283;
							goto IL_0c2b;
						case 41:
							array3[14] = (byte)num7;
							num2 = 357;
							goto IL_0c2b;
						case 144:
							num7 = 179 - 59;
							num2 = 289;
							goto IL_0c2b;
						case 216:
							array[19] = 145;
							num2 = 241;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 358;
						case 137:
							array[18] = (byte)num5;
							num2 = 153;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 61;
						case 61:
							num5 = 26 + 16;
							num3 = 335;
							continue;
						case 411:
							array[8] = 77;
							num3 = 133;
							continue;
						case 122:
							num5 = 129 - 43;
							num2 = 187;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 175;
						case 175:
							num17 = num14 * 4;
							num2 = 132;
							goto IL_0c2b;
						case 113:
							array3[5] = (byte)num7;
							num2 = 109;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 205;
						case 205:
							array3[12] = (byte)num7;
							num = 294;
							goto end_IL_0c2f;
						case 98:
							num33 = 0;
							num = 49;
							goto end_IL_0c2f;
						case 339:
							array[26] = 67;
							num3 = 309;
							continue;
						case 151:
							array[11] = (byte)num5;
							num2 = 122;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 112;
						case 112:
							num7 = 3 + 102;
							num3 = 56;
							continue;
						case 365:
							array3[7] = 156;
							num2 = 125;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 68;
						case 68:
							num5 = 42 + 96;
							num2 = 137;
							goto IL_0c2b;
						case 101:
							array[18] = (byte)num5;
							num2 = 300;
							goto IL_0c2b;
						case 401:
							array3[2] = 101;
							num3 = 356;
							continue;
						case 397:
							num7 = 213 - 71;
							num = 346;
							goto end_IL_0c2f;
						case 372:
							array[17] = 106;
							num2 = 420;
							goto IL_0c2b;
						case 5:
							array[12] = 113;
							num2 = 287;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 140;
						case 140:
							num5 = 224 - 74;
							num3 = 396;
							continue;
						case 1:
							num5 = 46 + 100;
							goto case 92;
						case 94:
							array6[num17 + 1] = (byte)((num18 & 0xFF00) >> 8);
							num = 97;
							goto end_IL_0c2f;
						case 71:
							array3[15] = (byte)num7;
							num2 = 291;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 348;
						case 348:
							num7 = 53 + 74;
							num2 = 14;
							goto IL_0c2b;
						case 307:
							array3[5] = (byte)num7;
							num3 = 412;
							continue;
						case 21:
							array[4] = 156;
							num3 = 95;
							continue;
						case 258:
							num7 = 185 - 61;
							num3 = 333;
							continue;
						case 42:
							num5 = 44 + 13;
							num2 = 408;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 22;
						case 96:
							array[4] = (byte)num5;
							num2 = 150;
							goto IL_0c2b;
						case 251:
							array[31] = (byte)num5;
							num = 366;
							goto end_IL_0c2f;
						case 295:
							array[17] = (byte)num5;
							num3 = 271;
							continue;
						case 127:
							array3[7] = (byte)num7;
							num2 = 43;
							goto IL_0c2b;
						case 260:
							array3[11] = 183;
							num2 = 156;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 253;
						case 253:
							if (num14 == num10 - 1)
							{
								num3 = 418;
								continue;
							}
							goto IL_2f56;
						case 259:
							array[19] = 52;
							num2 = 91;
							goto IL_0c2b;
						case 164:
							array[8] = 149;
							num3 = 42;
							continue;
						case 212:
							num5 = 83 + 31;
							num = 426;
							goto end_IL_0c2f;
						case 227:
							array[28] = (byte)num5;
							num2 = 305;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 5;
						case 51:
							num5 = 36 - 36;
							num2 = 167;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 271;
						case 271:
							num5 = 97 + 54;
							num3 = 116;
							continue;
						case 28:
						case 276:
							if (num14 >= num10)
							{
								num2 = 67;
								if (!dyOIuylN7SwQcetsD1())
								{
									goto IL_0c2b;
								}
								goto case 153;
							}
							num16 = num14 % num15;
							num = 175;
							goto end_IL_0c2f;
						case 153:
							num5 = 170 - 92;
							num = 252;
							goto end_IL_0c2f;
						case 192:
							array3[2] = 175;
							num2 = 401;
							goto IL_0c2b;
						case 315:
							array[11] = 117;
							num3 = 100;
							continue;
						case 37:
							goto IL_2c55;
						case 110:
						case 297:
							num14++;
							num = 276;
							goto end_IL_0c2f;
						case 184:
							array[19] = (byte)num5;
							num3 = 245;
							continue;
						case 395:
							array[16] = (byte)num5;
							num = 303;
							goto end_IL_0c2f;
						case 328:
							num13 = (uint)((array7[num9 + 3] << 24) | (array7[num9 + 2] << 16) | (array7[num9 + 1] << 8) | array7[num9]);
							num = 437;
							goto end_IL_0c2f;
						case 194:
							num5 = 45 + 79;
							num = 169;
							goto end_IL_0c2f;
						case 12:
							array[14] = (byte)num5;
							num2 = 352;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 421;
						case 421:
							num5 = 135 - 62;
							num = 405;
							goto end_IL_0c2f;
						case 196:
							num5 = 123 + 40;
							num2 = 72;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 330;
						case 330:
							array3[1] = 141;
							num = 192;
							goto end_IL_0c2f;
						case 90:
							num5 = 1 + 122;
							num2 = 45;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 228;
						case 131:
							array[24] = 155;
							num = 203;
							goto end_IL_0c2f;
						case 374:
							array[0] = (byte)num5;
							num3 = 138;
							continue;
						case 80:
							num5 = 15 + 72;
							num3 = 374;
							continue;
						case 115:
							HSspjvk81ibQ7ZdxRL(true);
							num = 155;
							goto end_IL_0c2f;
						case 159:
							num4 = K3i1OlsCcoa77PMoap(dtNJ9xs775, P_0);
							num = 442;
							goto end_IL_0c2f;
						case 391:
							array[14] = 200;
							num2 = 81;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 106;
						case 106:
							array[30] = 173;
							num3 = 124;
							continue;
						case 126:
							array[18] = (byte)num5;
							num2 = 68;
							goto IL_0c2b;
						case 9:
							array[24] = (byte)num5;
							num = 214;
							goto end_IL_0c2f;
						case 224:
							if (P_0 == -1)
							{
								num3 = 362;
								continue;
							}
							goto case 198;
						case 56:
							array3[1] = (byte)num7;
							num2 = 240;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 174;
						case 174:
							array3[15] = (byte)num7;
							num2 = 237;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 79;
						case 79:
							num7 = 177 - 59;
							num2 = 222;
							goto IL_0c2b;
						case 0:
							array[25] = 131;
							num3 = 73;
							continue;
						case 333:
							array3[8] = (byte)num7;
							num2 = 397;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 110;
						case 266:
							num5 = 49 + 117;
							num = 243;
							goto end_IL_0c2f;
						case 382:
							num5 = 236 - 78;
							num2 = 392;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 326;
						case 326:
							num7 = 0 + 39;
							num2 = 205;
							goto IL_0c2b;
						case 214:
							array[24] = 110;
							num2 = 358;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 261;
						case 261:
							cryptoStream = new CryptoStream(memoryStream, transform, CryptoStreamMode.Write);
							num3 = 394;
							continue;
						case 49:
						case 105:
							if (num33 < array4.Length)
							{
								array7[num33] ^= array4[num33];
								num = 6;
								goto end_IL_0c2f;
							}
							num2 = 224;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 31;
						case 31:
							num5 = 195 - 65;
							num2 = 378;
							goto IL_0c2b;
						case 343:
							array3[12] = (byte)num7;
							num2 = 160;
							goto IL_0c2b;
						case 399:
							num5 = 189 - 63;
							num2 = 246;
							goto IL_0c2b;
						case 88:
							array6 = new byte[array2.Length];
							num3 = 316;
							continue;
						case 291:
							array3[15] = 57;
							num2 = 75;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 346;
						case 432:
							array3[11] = 201;
							num2 = 313;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 148;
						case 148:
							array3[7] = (byte)num7;
							num2 = 25;
							goto IL_0c2b;
						case 10:
							array3[12] = 108;
							num2 = 141;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 100;
						case 100:
							array[11] = 125;
							num3 = 35;
							continue;
						case 375:
							if (dtNJ9xs775.Length == 0)
							{
								num = 23;
								goto end_IL_0c2f;
							}
							goto case 159;
						case 412:
							array3[5] = 134;
							num3 = 422;
							continue;
						case 305:
							num5 = 52 + 87;
							num = 238;
							goto end_IL_0c2f;
						case 298:
							array[2] = (byte)num5;
							num2 = 29;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 27;
						case 27:
							num5 = 120 + 49;
							num = 226;
							goto end_IL_0c2f;
						case 342:
							array3[3] = (byte)num7;
							num = 311;
							goto end_IL_0c2f;
						case 211:
							array[19] = (byte)num5;
							num2 = 216;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 381;
						case 65:
							array = new byte[32];
							num3 = 80;
							continue;
						case 69:
							array[21] = (byte)num5;
							num2 = 417;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 404;
						case 404:
							num7 = 119 + 102;
							num2 = 64;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 22;
						case 22:
							if (num14 == num10 - 1)
							{
								num = 47;
								goto end_IL_0c2f;
							}
							goto IL_19d0;
						case 52:
							array[14] = (byte)num5;
							num2 = 39;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 200;
						case 200:
							array3[9] = 57;
							num = 206;
							goto end_IL_0c2f;
						case 19:
							array[8] = (byte)num5;
							num2 = 207;
							goto IL_0c2b;
						case 63:
							array4[3] = array5[1];
							num = 355;
							goto end_IL_0c2f;
						case 162:
							array[22] = (byte)num5;
							num2 = 146;
							goto IL_0c2b;
						case 133:
							array[8] = 124;
							num2 = 37;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 149;
						case 149:
						case 210:
						{
							uint num20 = num6;
							uint num21 = num6;
							uint num22 = 608433572u;
							uint num23 = 376803549u;
							uint num24 = 1120927345u;
							uint num25 = 1198486640u;
							uint num26 = 1413770303u;
							uint num27 = num25 & 0xFF00FFu;
							uint num28 = num25 & 0xFF00FF00u;
							num27 = ((num27 >> 8) | (num28 << 8)) + num22;
							num25 = (num25 << 5) | (num25 >> 27);
							if ((double)num24 == 0.0)
							{
								num24--;
							}
							uint num29 = (uint)(1753167274.0 / (double)num24 + (double)num24);
							num24 = (uint)((double)(282190 * num29) + 1842983490.0);
							ulong num30 = num25 * num25;
							if (num30 == 0)
							{
								num30--;
							}
							num22 = (uint)(num22 * num22 % num30);
							num23 -= num25;
							num26 += num25;
							num21 ^= num21 >> 5;
							num21 += num22;
							num21 ^= num21 >> 28;
							num21 += num23;
							num21 ^= num21 << 27;
							num21 += num26;
							num21 = (((num25 << 11) + num23) ^ num23) + num21;
							num6 = num20 + (uint)(double)num21;
							num2 = 253;
							goto IL_0c2b;
						}
						case 62:
							array[28] = 84;
							num = 223;
							goto end_IL_0c2f;
						case 416:
							num7 = 83 + 47;
							num3 = 353;
							continue;
						case 434:
							dtNJ9xs775 = (byte[])uKNUceUNwIYqnB4CF6(memoryStream);
							num2 = 55;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 341;
						case 341:
							array[11] = 82;
							num2 = 383;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 283;
						case 283:
							array3[6] = (byte)num7;
							num3 = 404;
							continue;
						case 255:
							array[26] = (byte)num5;
							num = 103;
							goto end_IL_0c2f;
						case 58:
							array4[1] = array5[0];
							num3 = 63;
							continue;
						case 228:
							num5 = 125 - 41;
							num3 = 292;
							continue;
						case 371:
							array3[7] = (byte)num7;
							num2 = 361;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 161;
						case 161:
							num5 = 160 - 53;
							num2 = 162;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 109;
						case 109:
							array3[5] = 148;
							num3 = 263;
							continue;
						case 145:
							num5 = 88 + 99;
							num2 = 419;
							goto IL_0c2b;
						case 378:
							array[29] = (byte)num5;
							num3 = 190;
							continue;
						case 301:
							num12++;
							num2 = 217;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 408;
						case 170:
							array[5] = 142;
							num2 = 154;
							goto IL_0c2b;
						case 407:
							num11++;
							num2 = 230;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 348;
						case 3:
							array3[11] = 69;
							num2 = 26;
							goto IL_0c2b;
						case 312:
							num10++;
							num3 = 340;
							continue;
						case 350:
							num7 = 71 - 8;
							num3 = 314;
							continue;
						case 394:
							YlmQhAEmYLlOV0mnYC(cryptoStream, array2, 0, array2.Length);
							num3 = 388;
							continue;
						case 4:
							array[6] = 70;
							num2 = 114;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 193;
						case 193:
							array[6] = 138;
							num3 = 4;
							continue;
						case 418:
							if (num8 <= 0)
							{
								goto IL_2f56;
							}
							num2 = 387;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 8;
						case 8:
							array[10] = (byte)num5;
							num3 = 242;
							continue;
						case 64:
							array3[6] = (byte)num7;
							num2 = 177;
							if (0 == 0)
							{
								goto IL_0c2b;
							}
							goto case 357;
						case 357:
							num7 = 66 + 100;
							num3 = 163;
							continue;
						case 104:
							array3[10] = (byte)num7;
							num2 = 364;
							if (!dyOIuylN7SwQcetsD1())
							{
								goto IL_0c2b;
							}
							goto case 241;
						case 155:
							array2 = (byte[])sco8syVIMcR0fgtvyX(binaryReader, (int)LV0pNW8RqxQR7xjRw6(PhSRwGR2qQsDxGHMpX(binaryReader)));
							num2 = 321;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 208;
						case 208:
							array[23] = (byte)num5;
							num3 = 233;
							continue;
						case 185:
							num6 = 0u;
							num2 = 304;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 309;
						case 92:
						case 293:
							array[12] = (byte)num5;
							num2 = 2;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 351;
						case 351:
							num7 = 207 - 69;
							num2 = 148;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 324;
						case 324:
							array3[10] = (byte)num7;
							num2 = 3;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 250;
						case 376:
							num5 = 211 - 70;
							num2 = 281;
							goto IL_0c2b;
						case 390:
							array[7] = 138;
							num = 269;
							goto end_IL_0c2f;
						case 218:
							num5 = 56 + 35;
							num = 208;
							goto end_IL_0c2f;
						case 403:
							array[25] = (byte)num5;
							num3 = 44;
							continue;
						case 346:
							array3[8] = (byte)num7;
							num3 = 257;
							continue;
						case 442:
							try
							{
								return (string)n73tMN0n2Y8V3k9u4f(zq0rw4cGFSrHqyQuTj(), dtNJ9xs775, P_0 + 4, num4);
							}
							catch
							{
							}
							return "";
						case 362:
							symmetricAlgorithm = (SymmetricAlgorithm)kaMiRMrQWD0DNXUTGO();
							num2 = 235;
							if (true)
							{
								goto IL_0c2b;
							}
							goto case 206;
						case 83:
							{
								memoryStream = new MemoryStream();
								num = 261;
								goto end_IL_0c2f;
							}
							IL_2f56:
							num18 = num6 ^ num19;
							num2 = 386;
							if (KtVADlaAiaGNLrPfe9())
							{
								goto IL_0c2b;
							}
							goto case 217;
							IL_19d0:
							num9 = (uint)num17;
							num2 = 249;
							goto IL_0c2b;
						}
						array[17] = 126;
						num = 372;
						break;
						IL_2c55:
						num5 = 247 - 82;
						_ = 1;
						if (dyOIuylN7SwQcetsD1())
						{
							num3 = 293;
							continue;
						}
						num2 = 368;
						goto IL_0c2b;
						IL_006d:
						array[15] = (byte)num5;
						num = 38;
						break;
						continue;
						end_IL_0c2f:
						break;
					}
					break;
				}
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
		internal static string NANgEAexF(string P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			"{11111-22222-50001-00000}".Trim();
			byte[] array = Convert.FromBase64String(P_0);
			return Encoding.Unicode.GetString(array, 0, array.Length);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static int gM4s3TjPj()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return 5;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void g5fhmxQiY()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				RSACryptoServiceProvider.UseMachineKeyStore = true;
			}
			catch
			{
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static Delegate rCBka8ZAZ(IntPtr P_0, Type P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return (Delegate)typeof(Marshal).GetMethod("GetDelegateForFunctionPointer", new Type[2]
			{
				typeof(IntPtr),
				typeof(Type)
			}).Invoke(null, new object[2] { P_0, P_1 });
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object O0BxNtYmT(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				if (File.Exists(((Assembly)P_0).Location))
				{
					return ((Assembly)P_0).Location;
				}
			}
			catch
			{
			}
			try
			{
				if (File.Exists(((Assembly)P_0).GetName().CodeBase.ToString().Replace("file:///", "")))
				{
					return ((Assembly)P_0).GetName().CodeBase.ToString().Replace("file:///", "");
				}
			}
			catch
			{
			}
			try
			{
				if (File.Exists(P_0.GetType().GetProperty("Location").GetValue(P_0, new object[0])
					.ToString()))
				{
					return P_0.GetType().GetProperty("Location").GetValue(P_0, new object[0])
						.ToString();
				}
			}
			catch
			{
			}
			return "";
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
		private static byte[] V8twS9NUw(string P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			using FileStream fileStream = new FileStream(P_0, FileMode.Open, FileAccess.Read, FileShare.Read);
			int num = 0;
			long length = fileStream.Length;
			int num2 = (int)length;
			byte[] array = new byte[num2];
			while (num2 > 0)
			{
				int num3 = fileStream.Read(array, num, num2);
				num += num3;
				num2 -= num3;
			}
			return array;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[Ynf1aqBxsNGfLU1bDU(typeof(Ynf1aqBxsNGfLU1bDU.zghU6aZrkE35pPUbkp<object>[]))]
		private static byte[] ylCbcMTOT(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			MemoryStream memoryStream = new MemoryStream();
			SymmetricAlgorithm symmetricAlgorithm = C40nY3hj2();
			symmetricAlgorithm.Key = new byte[32]
			{
				95, 34, 140, 196, 236, 139, 207, 182, 189, 101,
				237, 111, 197, 84, 225, 213, 13, 25, 25, 59,
				212, 30, 172, 52, 1, 72, 205, 7, 236, 137,
				96, 186
			};
			symmetricAlgorithm.IV = new byte[16]
			{
				118, 136, 220, 40, 182, 121, 254, 78, 216, 42,
				251, 97, 172, 74, 212, 95
			};
			CryptoStream cryptoStream = new CryptoStream(memoryStream, symmetricAlgorithm.CreateDecryptor(), CryptoStreamMode.Write);
			cryptoStream.Write(P_0, 0, P_0.Length);
			cryptoStream.Close();
			return memoryStream.ToArray();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] xT3G2r3GA()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return null;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] mjMq98cjK()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return null;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] fCeaO6lgy()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-20001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] vk4UIF96c()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-20001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] iXeil5VZN()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-30001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] EUAA9hgjm()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-30001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] WwNtt3REi()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-40001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] zOpIOqhH6()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-40001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] s6GfP4BaW()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-50001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] A9Z0XW2E9()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-50001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public ER5MIqF7oL5pcFCvml()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			base..ctor();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object Ossp97Sry5Dj3aeBaK(object P_0, object P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Assembly)P_0).GetManifestResourceStream((string)P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object PhSRwGR2qQsDxGHMpX(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((BinaryReader)P_0).BaseStream;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void YC9Xk7JQsaniiZfHNQ(object P_0, long P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((Stream)P_0).Position = P_1;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void HSspjvk81ibQ7ZdxRL(bool P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			RSACryptoServiceProvider.UseMachineKeyStore = P_0;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static long LV0pNW8RqxQR7xjRw6(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Stream)P_0).Length;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object sco8syVIMcR0fgtvyX(object P_0, int P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((BinaryReader)P_0).ReadBytes(P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void vc0Tua1UeFqbTu59Jc(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((BinaryReader)P_0).Close();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void csO5cFqlHIaBF80IiK(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Array.Reverse((Array)P_0);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object e0I2KhZ0ZRvHjytujl(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Assembly)P_0).GetName();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object Ti480rfGZgDY3sVJfU(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((AssemblyName)P_0).GetPublicKeyToken();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object kaMiRMrQWD0DNXUTGO()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return C40nY3hj2();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void DHvRL4mJbQ2Du6wCg0(object P_0, CipherMode P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((SymmetricAlgorithm)P_0).Mode = P_1;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object vlsp9W2SqQC0tSZxQy(object P_0, object P_1, object P_2)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((SymmetricAlgorithm)P_0).CreateDecryptor((byte[])P_1, (byte[])P_2);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void YlmQhAEmYLlOV0mnYC(object P_0, object P_1, int P_2, int P_3)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((Stream)P_0).Write((byte[])P_1, P_2, P_3);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void NVeRggQn7sICdx2Yyd(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((CryptoStream)P_0).FlushFinalBlock();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object uKNUceUNwIYqnB4CF6(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((MemoryStream)P_0).ToArray();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void BjoN2TKoy9gjBUTyo6(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((Stream)P_0).Close();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static int K3i1OlsCcoa77PMoap(object P_0, int P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return BitConverter.ToInt32((byte[])P_0, P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object zq0rw4cGFSrHqyQuTj()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return Encoding.Unicode;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object n73tMN0n2Y8V3k9u4f(object P_0, object P_1, int P_2, int P_3)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Encoding)P_0).GetString((byte[])P_1, P_2, P_3);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static bool KtVADlaAiaGNLrPfe9()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return true;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static bool dyOIuylN7SwQcetsD1()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return false;
		}
	}
}
namespace sDT32rX3GAFjM98cjK
{
	internal class ETI8tSy9NUwElCcMTO
	{
		private static bool N2tJEQwbVL;

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void P7CxbvUz56ZVI()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public ETI8tSy9NUwElCcMTO()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			base..ctor();
		}
	}
}
