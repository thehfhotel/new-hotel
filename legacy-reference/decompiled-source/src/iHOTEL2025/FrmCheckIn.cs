using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using C1.Win.C1FlexGrid;
using C1.Win.C1FlexGrid.Util.BaseControls;
using DevComponents.DotNetBar;
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmCheckIn : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TdocNum")]
	private TextBox _TdocNum;

	[AccessedThroughProperty("TCusSearch")]
	private TextBox _TCusSearch;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Tc_no")]
	private TextBox _Tc_no;

	[AccessedThroughProperty("TcusName")]
	private TextBox _TcusName;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Tc_fax")]
	private TextBox _Tc_fax;

	[AccessedThroughProperty("Button7")]
	private Button _Button7;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label16")]
	private Label _Label16;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("LabelTroom")]
	private Label _LabelTroom;

	[AccessedThroughProperty("Label24")]
	private Label _Label24;

	[AccessedThroughProperty("TCarType")]
	private ComboBox _TCarType;

	[AccessedThroughProperty("Label31")]
	private Label _Label31;

	[AccessedThroughProperty("TCusType")]
	private ComboBox _TCusType;

	[AccessedThroughProperty("Label21")]
	private Label _Label21;

	[AccessedThroughProperty("Label29")]
	private Label _Label29;

	[AccessedThroughProperty("Label27")]
	private Label _Label27;

	[AccessedThroughProperty("Label26")]
	private Label _Label26;

	[AccessedThroughProperty("TCarID")]
	private TextBox _TCarID;

	[AccessedThroughProperty("TCusEmail")]
	private TextBox _TCusEmail;

	[AccessedThroughProperty("TCusName2")]
	private TextBox _TCusName2;

	[AccessedThroughProperty("expandablePanel5")]
	private ExpandablePanel _expandablePanel5;

	[AccessedThroughProperty("expandablePanel4")]
	private ExpandablePanel _expandablePanel4;

	[AccessedThroughProperty("ButtonItem15")]
	private ButtonItem _ButtonItem15;

	[AccessedThroughProperty("ButtonItem43")]
	private ButtonItem _ButtonItem43;

	[AccessedThroughProperty("ButtonItem40")]
	private ButtonItem _ButtonItem40;

	[AccessedThroughProperty("ButtonItem44")]
	private ButtonItem _ButtonItem44;

	[AccessedThroughProperty("ButtonItem45")]
	private ButtonItem _ButtonItem45;

	[AccessedThroughProperty("ButtonItem46")]
	private ButtonItem _ButtonItem46;

	[AccessedThroughProperty("ButtonItem16")]
	private ButtonItem _ButtonItem16;

	[AccessedThroughProperty("ButtonItem17")]
	private ButtonItem _ButtonItem17;

	[AccessedThroughProperty("ButtonItem20")]
	private ButtonItem _ButtonItem20;

	[AccessedThroughProperty("ButtonItem32")]
	private ButtonItem _ButtonItem32;

	[AccessedThroughProperty("ButtonItem33")]
	private ButtonItem _ButtonItem33;

	[AccessedThroughProperty("ButtonItem11")]
	private ButtonItem _ButtonItem11;

	[AccessedThroughProperty("ButtonItem12")]
	private ButtonItem _ButtonItem12;

	[AccessedThroughProperty("ButtonItem13")]
	private ButtonItem _ButtonItem13;

	[AccessedThroughProperty("ButtonItem14")]
	private ButtonItem _ButtonItem14;

	[AccessedThroughProperty("ButtonItem7")]
	private ButtonItem _ButtonItem7;

	[AccessedThroughProperty("ButtonItem28")]
	private ButtonItem _ButtonItem28;

	[AccessedThroughProperty("ButtonItem29")]
	private ButtonItem _ButtonItem29;

	[AccessedThroughProperty("ButtonItem30")]
	private ButtonItem _ButtonItem30;

	[AccessedThroughProperty("ButtonItem31")]
	private ButtonItem _ButtonItem31;

	[AccessedThroughProperty("ButtonItem2")]
	private ButtonItem _ButtonItem2;

	[AccessedThroughProperty("ButtonItem41")]
	private ButtonItem _ButtonItem41;

	[AccessedThroughProperty("Panel1")]
	private Panel _Panel1;

	[AccessedThroughProperty("Tc_ampore")]
	private TextBox _Tc_ampore;

	[AccessedThroughProperty("Label39")]
	private Label _Label39;

	[AccessedThroughProperty("Tc_tambon")]
	private TextBox _Tc_tambon;

	[AccessedThroughProperty("Label38")]
	private Label _Label38;

	[AccessedThroughProperty("Tc_road")]
	private TextBox _Tc_road;

	[AccessedThroughProperty("Label37")]
	private Label _Label37;

	[AccessedThroughProperty("Tc_soi")]
	private TextBox _Tc_soi;

	[AccessedThroughProperty("Label36")]
	private Label _Label36;

	[AccessedThroughProperty("Tc_moo")]
	private TextBox _Tc_moo;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Tc_tel")]
	private ComboBox _Tc_tel;

	[AccessedThroughProperty("Tc_code")]
	private TextBox _Tc_code;

	[AccessedThroughProperty("Label41")]
	private Label _Label41;

	[AccessedThroughProperty("Tc_province")]
	private TextBox _Tc_province;

	[AccessedThroughProperty("Label40")]
	private Label _Label40;

	[AccessedThroughProperty("Panel2")]
	private Panel _Panel2;

	[AccessedThroughProperty("Tw_ampore")]
	private TextBox _Tw_ampore;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Tw_tambon")]
	private TextBox _Tw_tambon;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Tw_tel")]
	private ComboBox _Tw_tel;

	[AccessedThroughProperty("Tw_code")]
	private TextBox _Tw_code;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Tw_road")]
	private TextBox _Tw_road;

	[AccessedThroughProperty("Tw_privince")]
	private TextBox _Tw_privince;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label42")]
	private Label _Label42;

	[AccessedThroughProperty("Label43")]
	private Label _Label43;

	[AccessedThroughProperty("Label44")]
	private Label _Label44;

	[AccessedThroughProperty("Tw")]
	private TextBox _Tw;

	[AccessedThroughProperty("Label48")]
	private Label _Label48;

	[AccessedThroughProperty("Tw_soi")]
	private TextBox _Tw_soi;

	[AccessedThroughProperty("Label45")]
	private Label _Label45;

	[AccessedThroughProperty("Tw_moo")]
	private TextBox _Tw_moo;

	[AccessedThroughProperty("Label46")]
	private Label _Label46;

	[AccessedThroughProperty("Tw_no")]
	private TextBox _Tw_no;

	[AccessedThroughProperty("Label47")]
	private Label _Label47;

	[AccessedThroughProperty("Tw_fax")]
	private TextBox _Tw_fax;

	[AccessedThroughProperty("Panel3")]
	private Panel _Panel3;

	[AccessedThroughProperty("TimerDrop")]
	private Timer _TimerDrop;

	[AccessedThroughProperty("TimerDrop2")]
	private Timer _TimerDrop2;

	[AccessedThroughProperty("Button11")]
	private Button _Button11;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("Button9")]
	private Button _Button9;

	[AccessedThroughProperty("Button5")]
	private Button _Button5;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("LabelDebt")]
	private Label _LabelDebt;

	[AccessedThroughProperty("Label18")]
	private Label _Label18;

	[AccessedThroughProperty("LabelPayed")]
	private Label _LabelPayed;

	[AccessedThroughProperty("Label33")]
	private Label _Label33;

	[AccessedThroughProperty("Labelroompro")]
	private Label _Labelroompro;

	[AccessedThroughProperty("Label20")]
	private Label _Label20;

	[AccessedThroughProperty("LabelTpro")]
	private Label _LabelTpro;

	[AccessedThroughProperty("Label17")]
	private Label _Label17;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("Button_DEP")]
	private Button _Button_DEP;

	[AccessedThroughProperty("Button_REG")]
	private Button _Button_REG;

	[AccessedThroughProperty("Label30")]
	private Label _Label30;

	[AccessedThroughProperty("Label23")]
	private Label _Label23;

	[AccessedThroughProperty("Label35")]
	private Label _Label35;

	[AccessedThroughProperty("PanelCust")]
	private Panel _PanelCust;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Label50")]
	private Label _Label50;

	[AccessedThroughProperty("TCusID")]
	private TextBox _TCusID;

	[AccessedThroughProperty("Tend")]
	private DateTimePicker _Tend;

	[AccessedThroughProperty("Tstart")]
	private DateTimePicker _Tstart;

	[AccessedThroughProperty("Label51")]
	private Label _Label51;

	[AccessedThroughProperty("Label52")]
	private Label _Label52;

	[AccessedThroughProperty("Label53")]
	private Label _Label53;

	[AccessedThroughProperty("Label54")]
	private Label _Label54;

	[AccessedThroughProperty("Tnum")]
	private TextBox _Tnum;

	[AccessedThroughProperty("TselectRoom")]
	private ComboBox _TselectRoom;

	[AccessedThroughProperty("Label55")]
	private Label _Label55;

	[AccessedThroughProperty("Tnote")]
	private TextBox _Tnote;

	[AccessedThroughProperty("Tdebt")]
	private TextBox _Tdebt;

	[AccessedThroughProperty("Tcash")]
	private TextBox _Tcash;

	[AccessedThroughProperty("Tpay")]
	private TextBox _Tpay;

	[AccessedThroughProperty("LabelButton7")]
	private Label _LabelButton7;

	[AccessedThroughProperty("Label15")]
	private Label _Label15;

	[AccessedThroughProperty("TbookNo")]
	private TextBox _TbookNo;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("TCusTypeMain")]
	private ComboBox _TCusTypeMain;

	[AccessedThroughProperty("Label19")]
	private Label _Label19;

	[AccessedThroughProperty("ExpandablePanel1")]
	private ExpandablePanel _ExpandablePanel1;

	[AccessedThroughProperty("Panel4")]
	private Panel _Panel4;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ItemPanel1")]
	private ItemPanel _ItemPanel1;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("SplitContainer1")]
	private SplitContainer _SplitContainer1;

	[AccessedThroughProperty("POver")]
	private TextBox _POver;

	[AccessedThroughProperty("LOver")]
	private Label _LOver;

	[AccessedThroughProperty("Tover")]
	private TextBox _Tover;

	[AccessedThroughProperty("Label22")]
	private Label _Label22;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Tcusperfix")]
	private ComboBox _Tcusperfix;

	[AccessedThroughProperty("Label25")]
	private Label _Label25;

	[AccessedThroughProperty("TcusSex")]
	private ComboBox _TcusSex;

	[AccessedThroughProperty("Label28")]
	private Label _Label28;

	[AccessedThroughProperty("Label32")]
	private Label _Label32;

	[AccessedThroughProperty("TcusCardID")]
	private TextBox _TcusCardID;

	[AccessedThroughProperty("Label34")]
	private Label _Label34;

	[AccessedThroughProperty("Tcontry")]
	private TextBox _Tcontry;

	[AccessedThroughProperty("Panel5")]
	private Panel _Panel5;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("Button10")]
	private Button _Button10;

	[AccessedThroughProperty("Button8")]
	private Button _Button8;

	[AccessedThroughProperty("Tcontry2")]
	private TextBox _Tcontry2;

	[AccessedThroughProperty("Label56")]
	private Label _Label56;

	[AccessedThroughProperty("Tname2")]
	private TextBox _Tname2;

	[AccessedThroughProperty("Label49")]
	private Label _Label49;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("Button12")]
	private Button _Button12;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label57")]
	private Label _Label57;

	[AccessedThroughProperty("Button14")]
	private Button _Button14;

	[AccessedThroughProperty("Button13")]
	private Button _Button13;

	[AccessedThroughProperty("Button15")]
	private Button _Button15;

	[AccessedThroughProperty("Button16")]
	private Button _Button16;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("Label58")]
	private Label _Label58;

	[AccessedThroughProperty("TwTax")]
	private TextBox _TwTax;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("ButtonT1")]
	private ButtonX _ButtonT1;

	[AccessedThroughProperty("ButtonT3")]
	private ButtonX _ButtonT3;

	[AccessedThroughProperty("ButtonT2")]
	private ButtonX _ButtonT2;

	[AccessedThroughProperty("Label59")]
	private Label _Label59;

	[AccessedThroughProperty("Label60")]
	private Label _Label60;

	[AccessedThroughProperty("Label61")]
	private Label _Label61;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("Label62")]
	private Label _Label62;

	[AccessedThroughProperty("Grid2")]
	private C1FlexGrid _Grid2;

	[AccessedThroughProperty("Labelจอง")]
	private Label label_0;

	[AccessedThroughProperty("Troomม\u0e31ดจำ")]
	private Label label_1;

	[AccessedThroughProperty("Label65")]
	private Label _Label65;

	[AccessedThroughProperty("Tม\u0e31ดจำ")]
	private Label label_2;

	[AccessedThroughProperty("Label64")]
	private Label _Label64;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("Label63")]
	private Label _Label63;

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	[AccessedThroughProperty("Grid1")]
	private C1FlexGrid _Grid1;

	[AccessedThroughProperty("Button6")]
	private Button _Button6;

	public string EDIT_ID;

	public string tmp_no;

	private decimal Dep_price;

	public string tmp_room;

	public ArrayList tmp_roomarr;

	public DateTime Fstart;

	public DateTime Fend;

	private string Booking_cust;

	private bool isbook;

	private int WORK_ID;

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

	internal virtual TabItem TabItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _TabItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TabItem4 = value;
		}
	}

	internal virtual GroupBox GroupBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox1 = value;
		}
	}

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
			EventHandler value2 = Button1_Click;
			if (_Button1 != null)
			{
				_Button1.Click -= value2;
			}
			_Button1 = value;
			if (_Button1 != null)
			{
				_Button1.Click += value2;
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

	internal virtual TextBox TdocNum
	{
		[DebuggerNonUserCode]
		get
		{
			return _TdocNum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TdocNum = value;
		}
	}

	internal virtual TextBox TextBox_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusSearch;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TCusNo_TextChanged;
			EventHandler value3 = TCusNo_GotFocus;
			EventHandler value4 = TCusSearch_LostFocus;
			KeyEventHandler value5 = TCusSearch_KeyDown;
			if (_TCusSearch != null)
			{
				_TCusSearch.TextChanged -= value2;
				_TCusSearch.GotFocus -= value3;
				_TCusSearch.LostFocus -= value4;
				_TCusSearch.KeyDown -= value5;
			}
			_TCusSearch = value;
			if (_TCusSearch != null)
			{
				_TCusSearch.TextChanged += value2;
				_TCusSearch.GotFocus += value3;
				_TCusSearch.LostFocus += value4;
				_TCusSearch.KeyDown += value5;
			}
		}
	}

	internal virtual Label Label8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label8 = value;
		}
	}

	internal virtual Label Label9
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label9 = value;
		}
	}

	internal virtual Label Label7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label7 = value;
		}
	}

	internal virtual TextBox Tc_no
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_no;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_no = value;
		}
	}

	internal virtual TextBox TcusName
	{
		[DebuggerNonUserCode]
		get
		{
			return _TcusName;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = TcusName_KeyDown;
			if (_TcusName != null)
			{
				_TcusName.KeyDown -= value2;
			}
			_TcusName = value;
			if (_TcusName != null)
			{
				_TcusName.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label10
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label10 = value;
		}
	}

	internal virtual TextBox Tc_fax
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_fax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_fax = value;
		}
	}

	internal virtual Button Button7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button7_Click;
			if (_Button7 != null)
			{
				_Button7.Click -= value2;
			}
			_Button7 = value;
			if (_Button7 != null)
			{
				_Button7.Click += value2;
			}
		}
	}

	internal virtual DateTimePicker DateTimePicker1
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DateTimePicker1 = value;
		}
	}

	internal virtual Label Label16
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label16 = value;
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

	internal virtual Label LabelTroom
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelTroom;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelTroom = value;
		}
	}

	internal virtual Label Label24
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label24 = value;
		}
	}

	internal virtual ComboBox TCarType
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCarType;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCarType = value;
		}
	}

	internal virtual Label Label31
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label31;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label31 = value;
		}
	}

	internal virtual ComboBox TCusType
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusType;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TCusType_SelectedIndexChanged;
			if (_TCusType != null)
			{
				_TCusType.SelectedIndexChanged -= value2;
			}
			_TCusType = value;
			if (_TCusType != null)
			{
				_TCusType.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label21
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label21 = value;
		}
	}

	internal virtual Label Label29
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label29;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label29 = value;
		}
	}

	internal virtual Label Label27
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label27;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label27 = value;
		}
	}

	internal virtual Label Label26
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label26;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label26 = value;
		}
	}

	internal virtual TextBox TCarID
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCarID;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = TCarID_KeyDown;
			if (_TCarID != null)
			{
				_TCarID.KeyDown -= value2;
			}
			_TCarID = value;
			if (_TCarID != null)
			{
				_TCarID.KeyDown += value2;
			}
		}
	}

	internal virtual TextBox TextBox_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusEmail;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCusEmail = value;
		}
	}

	internal virtual TextBox TextBox_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusName2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = TCusName2_KeyDown;
			if (_TCusName2 != null)
			{
				_TCusName2.KeyDown -= value2;
			}
			_TCusName2 = value;
			if (_TCusName2 != null)
			{
				_TCusName2.KeyDown += value2;
			}
		}
	}

	internal virtual ExpandablePanel expandablePanel5
	{
		[DebuggerNonUserCode]
		get
		{
			return _expandablePanel5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			ExpandChangeEventHandler value2 = expandablePanel5_ExpandedChanged;
			if (_expandablePanel5 != null)
			{
				_expandablePanel5.ExpandedChanged -= value2;
			}
			_expandablePanel5 = value;
			if (_expandablePanel5 != null)
			{
				_expandablePanel5.ExpandedChanged += value2;
			}
		}
	}

	internal virtual ExpandablePanel expandablePanel4
	{
		[DebuggerNonUserCode]
		get
		{
			return _expandablePanel4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = expandablePanel4_Click;
			ExpandChangeEventHandler value3 = expandablePanel4_ExpandedChanged;
			if (_expandablePanel4 != null)
			{
				_expandablePanel4.Click -= value2;
				_expandablePanel4.ExpandedChanged -= value3;
			}
			_expandablePanel4 = value;
			if (_expandablePanel4 != null)
			{
				_expandablePanel4.Click += value2;
				_expandablePanel4.ExpandedChanged += value3;
			}
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
			_ButtonItem43 = value;
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
			_ButtonItem40 = value;
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
			_ButtonItem44 = value;
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
			_ButtonItem45 = value;
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
			_ButtonItem46 = value;
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
			_ButtonItem20 = value;
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
			_ButtonItem32 = value;
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
			_ButtonItem33 = value;
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
			_ButtonItem11 = value;
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
			_ButtonItem12 = value;
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
			_ButtonItem13 = value;
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
			_ButtonItem14 = value;
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
			_ButtonItem28 = value;
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
			_ButtonItem29 = value;
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
			_ButtonItem30 = value;
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
			_ButtonItem31 = value;
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
			_ButtonItem41 = value;
		}
	}

	internal virtual Panel Panel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel1 = value;
		}
	}

	internal virtual TextBox Tc_ampore
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_ampore;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_ampore = value;
		}
	}

	internal virtual Label Label39
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label39;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label39 = value;
		}
	}

	internal virtual TextBox Tc_tambon
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_tambon;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_tambon = value;
		}
	}

	internal virtual Label Label38
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label38;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label38 = value;
		}
	}

	internal virtual TextBox Tc_road
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_road;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_road = value;
		}
	}

	internal virtual Label Label37
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label37;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label37 = value;
		}
	}

	internal virtual TextBox Tc_soi
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_soi;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_soi = value;
		}
	}

	internal virtual Label Label36
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label36;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label36 = value;
		}
	}

	internal virtual TextBox Tc_moo
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_moo;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_moo = value;
		}
	}

	internal virtual Label Label11
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label11 = value;
		}
	}

	internal virtual ComboBox Tc_tel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_tel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox3_LostFocus;
			EventHandler value3 = ComboBox3_GotFocus;
			KeyEventHandler value4 = ComboBox3_KeyDown;
			if (_Tc_tel != null)
			{
				_Tc_tel.LostFocus -= value2;
				_Tc_tel.GotFocus -= value3;
				_Tc_tel.KeyDown -= value4;
			}
			_Tc_tel = value;
			if (_Tc_tel != null)
			{
				_Tc_tel.LostFocus += value2;
				_Tc_tel.GotFocus += value3;
				_Tc_tel.KeyDown += value4;
			}
		}
	}

	internal virtual TextBox Tc_code
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_code;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_code = value;
		}
	}

	internal virtual Label Label41
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label41;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label41 = value;
		}
	}

	internal virtual TextBox Tc_province
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tc_province;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tc_province = value;
		}
	}

	internal virtual Label Label40
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label40;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label40 = value;
		}
	}

	internal virtual Panel Panel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel2 = value;
		}
	}

	internal virtual TextBox Tw_ampore
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_ampore;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tw_ampore_TextChanged;
			KeyEventHandler value3 = NEXKEY;
			if (_Tw_ampore != null)
			{
				_Tw_ampore.TextChanged -= value2;
				_Tw_ampore.KeyDown -= value3;
			}
			_Tw_ampore = value;
			if (_Tw_ampore != null)
			{
				_Tw_ampore.TextChanged += value2;
				_Tw_ampore.KeyDown += value3;
			}
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

	internal virtual TextBox Tw_tambon
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_tambon;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_tambon != null)
			{
				_Tw_tambon.KeyDown -= value2;
			}
			_Tw_tambon = value;
			if (_Tw_tambon != null)
			{
				_Tw_tambon.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label3 = value;
		}
	}

	internal virtual ComboBox Tw_tel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_tel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox4_LostFocus;
			EventHandler value3 = ComboBox4_GotFocus;
			KeyEventHandler value4 = NEXKEY;
			if (_Tw_tel != null)
			{
				_Tw_tel.LostFocus -= value2;
				_Tw_tel.GotFocus -= value3;
				_Tw_tel.KeyDown -= value4;
			}
			_Tw_tel = value;
			if (_Tw_tel != null)
			{
				_Tw_tel.LostFocus += value2;
				_Tw_tel.GotFocus += value3;
				_Tw_tel.KeyDown += value4;
			}
		}
	}

	internal virtual TextBox Tw_code
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_code;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_code != null)
			{
				_Tw_code.KeyDown -= value2;
			}
			_Tw_code = value;
			if (_Tw_code != null)
			{
				_Tw_code.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label4 = value;
		}
	}

	internal virtual TextBox Tw_road
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_road;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_road != null)
			{
				_Tw_road.KeyDown -= value2;
			}
			_Tw_road = value;
			if (_Tw_road != null)
			{
				_Tw_road.KeyDown += value2;
			}
		}
	}

	internal virtual TextBox Tw_privince
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_privince;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_privince != null)
			{
				_Tw_privince.KeyDown -= value2;
			}
			_Tw_privince = value;
			if (_Tw_privince != null)
			{
				_Tw_privince.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label5 = value;
		}
	}

	internal virtual Label Label42
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label42;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label42 = value;
		}
	}

	internal virtual Label Label43
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label43;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label43 = value;
		}
	}

	internal virtual Label Label44
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label44;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label44 = value;
		}
	}

	internal virtual TextBox Tw
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw != null)
			{
				_Tw.KeyDown -= value2;
			}
			_Tw = value;
			if (_Tw != null)
			{
				_Tw.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label48
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label48;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label48 = value;
		}
	}

	internal virtual TextBox Tw_soi
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_soi;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_soi != null)
			{
				_Tw_soi.KeyDown -= value2;
			}
			_Tw_soi = value;
			if (_Tw_soi != null)
			{
				_Tw_soi.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label45
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label45;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label45 = value;
		}
	}

	internal virtual TextBox Tw_moo
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_moo;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_moo != null)
			{
				_Tw_moo.KeyDown -= value2;
			}
			_Tw_moo = value;
			if (_Tw_moo != null)
			{
				_Tw_moo.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label46
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label46;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label46 = value;
		}
	}

	internal virtual TextBox Tw_no
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_no;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_no != null)
			{
				_Tw_no.KeyDown -= value2;
			}
			_Tw_no = value;
			if (_Tw_no != null)
			{
				_Tw_no.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label47
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label47;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label47 = value;
		}
	}

	internal virtual TextBox Tw_fax
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tw_fax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_Tw_fax != null)
			{
				_Tw_fax.KeyDown -= value2;
			}
			_Tw_fax = value;
			if (_Tw_fax != null)
			{
				_Tw_fax.KeyDown += value2;
			}
		}
	}

	internal virtual Panel Panel3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel3 = value;
		}
	}

	internal virtual Timer TimerDrop
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerDrop;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerDrop_Tick;
			if (_TimerDrop != null)
			{
				_TimerDrop.Tick -= value2;
			}
			_TimerDrop = value;
			if (_TimerDrop != null)
			{
				_TimerDrop.Tick += value2;
			}
		}
	}

	internal virtual Timer TimerDrop2
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerDrop2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerDrop2_Tick;
			if (_TimerDrop2 != null)
			{
				_TimerDrop2.Tick -= value2;
			}
			_TimerDrop2 = value;
			if (_TimerDrop2 != null)
			{
				_TimerDrop2.Tick += value2;
			}
		}
	}

	internal virtual Button Button11
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button11_Click;
			if (_Button11 != null)
			{
				_Button11.Click -= value2;
			}
			_Button11 = value;
			if (_Button11 != null)
			{
				_Button11.Click += value2;
			}
		}
	}

	internal virtual Button Button3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
			{
				Button3_Click(RuntimeHelpers.GetObjectValue(sender), e);
			};
			if (_Button3 != null)
			{
				_Button3.Click -= value2;
			}
			_Button3 = value;
			if (_Button3 != null)
			{
				_Button3.Click += value2;
			}
		}
	}

	internal virtual Label Label14
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label14 = value;
		}
	}

	internal virtual Button Button9
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button9_Click;
			if (_Button9 != null)
			{
				_Button9.Click -= value2;
			}
			_Button9 = value;
			if (_Button9 != null)
			{
				_Button9.Click += value2;
			}
		}
	}

	internal virtual Button Button5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button5_Click;
			if (_Button5 != null)
			{
				_Button5.Click -= value2;
			}
			_Button5 = value;
			if (_Button5 != null)
			{
				_Button5.Click += value2;
			}
		}
	}

	internal virtual Label Label12
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label12 = value;
		}
	}

	internal virtual Label LabelDebt
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelDebt;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelDebt = value;
		}
	}

	internal virtual Label Label18
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label18 = value;
		}
	}

	internal virtual Label LabelPayed
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelPayed;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelPayed = value;
		}
	}

	internal virtual Label Label33
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label33;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label33 = value;
		}
	}

	internal virtual Label Labelroompro
	{
		[DebuggerNonUserCode]
		get
		{
			return _Labelroompro;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Labelroompro = value;
		}
	}

	internal virtual Label Label20
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label20 = value;
		}
	}

	internal virtual Label LabelTpro
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelTpro;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelTpro = value;
		}
	}

	internal virtual Label Label17
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label17 = value;
		}
	}

	internal virtual Label Label13
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label13 = value;
		}
	}

	internal virtual Button Button_DEP
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button_DEP;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button10_Click;
			if (_Button_DEP != null)
			{
				_Button_DEP.Click -= value2;
			}
			_Button_DEP = value;
			if (_Button_DEP != null)
			{
				_Button_DEP.Click += value2;
			}
		}
	}

	internal virtual Button Button_REG
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button_REG;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button8_Click;
			if (_Button_REG != null)
			{
				_Button_REG.Click -= value2;
			}
			_Button_REG = value;
			if (_Button_REG != null)
			{
				_Button_REG.Click += value2;
			}
		}
	}

	internal virtual Label Label30
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label30;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label30 = value;
		}
	}

	internal virtual Label Label23
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label23 = value;
		}
	}

	internal virtual Label Label35
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label35;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label35 = value;
		}
	}

	internal virtual Panel PanelCust
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelCust;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelCust = value;
		}
	}

	internal virtual ListView ListView1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = ListView1_KeyDown;
			EventHandler value3 = ListView1_DoubleClick;
			EventHandler value4 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.KeyDown -= value2;
				_ListView1.DoubleClick -= value3;
				_ListView1.SelectedIndexChanged -= value4;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.KeyDown += value2;
				_ListView1.DoubleClick += value3;
				_ListView1.SelectedIndexChanged += value4;
			}
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

	internal virtual ColumnHeader ColumnHeader3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader3 = value;
		}
	}

	internal virtual Label Label50
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label50;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label50 = value;
		}
	}

	internal virtual TextBox TCusID
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusID;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCusID = value;
		}
	}

	internal virtual DateTimePicker Tend
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tend;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tstart_ValueChanged;
			if (_Tend != null)
			{
				_Tend.ValueChanged -= value2;
			}
			_Tend = value;
			if (_Tend != null)
			{
				_Tend.ValueChanged += value2;
			}
		}
	}

	internal virtual DateTimePicker Tstart
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tstart;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tstart_ValueChanged;
			if (_Tstart != null)
			{
				_Tstart.ValueChanged -= value2;
			}
			_Tstart = value;
			if (_Tstart != null)
			{
				_Tstart.ValueChanged += value2;
			}
		}
	}

	internal virtual Label Label51
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label51;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label51 = value;
		}
	}

	internal virtual Label Label52
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label52;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label52 = value;
		}
	}

	internal virtual Label Label53
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label53;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label53 = value;
		}
	}

	internal virtual Label Label54
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label54;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label54 = value;
		}
	}

	internal virtual TextBox Tnum
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tnum = value;
		}
	}

	internal virtual ComboBox TselectRoom
	{
		[DebuggerNonUserCode]
		get
		{
			return _TselectRoom;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TselectRoom = value;
		}
	}

	internal virtual Label Label55
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label55;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label55 = value;
		}
	}

	internal virtual TextBox Tnote
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnote;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tnote = value;
		}
	}

	internal virtual TextBox Tdebt
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tdebt;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tdebt_LostFocus;
			KeyEventHandler value3 = Tdebt_KeyDown;
			EventHandler value4 = Tdebt_TextChanged;
			if (_Tdebt != null)
			{
				_Tdebt.LostFocus -= value2;
				_Tdebt.KeyDown -= value3;
				_Tdebt.TextChanged -= value4;
			}
			_Tdebt = value;
			if (_Tdebt != null)
			{
				_Tdebt.LostFocus += value2;
				_Tdebt.KeyDown += value3;
				_Tdebt.TextChanged += value4;
			}
		}
	}

	internal virtual TextBox Tcash
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcash;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcash = value;
		}
	}

	internal virtual TextBox Tpay
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpay;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpay = value;
		}
	}

	internal virtual Label LabelButton7
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelButton7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = LabelButton7_Click;
			if (_LabelButton7 != null)
			{
				_LabelButton7.Click -= value2;
			}
			_LabelButton7 = value;
			if (_LabelButton7 != null)
			{
				_LabelButton7.Click += value2;
			}
		}
	}

	internal virtual Label Label15
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label15 = value;
		}
	}

	internal virtual TextBox TbookNo
	{
		[DebuggerNonUserCode]
		get
		{
			return _TbookNo;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TbookNo_TextChanged;
			if (_TbookNo != null)
			{
				_TbookNo.TextChanged -= value2;
			}
			_TbookNo = value;
			if (_TbookNo != null)
			{
				_TbookNo.TextChanged += value2;
			}
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

	internal virtual ComboBox TCusTypeMain
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusTypeMain;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TCusTypeMain_SelectedIndexChanged;
			if (_TCusTypeMain != null)
			{
				_TCusTypeMain.SelectedIndexChanged -= value2;
			}
			_TCusTypeMain = value;
			if (_TCusTypeMain != null)
			{
				_TCusTypeMain.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label19
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label19 = value;
		}
	}

	internal virtual ExpandablePanel ExpandablePanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ExpandablePanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			ExpandChangeEventHandler value2 = ExpandablePanel1_ExpandedChanged;
			if (_ExpandablePanel1 != null)
			{
				_ExpandablePanel1.ExpandedChanged -= value2;
			}
			_ExpandablePanel1 = value;
			if (_ExpandablePanel1 != null)
			{
				_ExpandablePanel1.ExpandedChanged += value2;
			}
		}
	}

	internal virtual Panel Panel4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel4 = value;
		}
	}

	internal virtual ButtonX ButtonX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX1_Click;
			if (_ButtonX1 != null)
			{
				_ButtonX1.Click -= value2;
			}
			_ButtonX1 = value;
			if (_ButtonX1 != null)
			{
				_ButtonX1.Click += value2;
			}
		}
	}

	internal virtual ItemPanel ItemPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemPanel1 = value;
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
			EventHandler value2 = ButtonItem1_Click;
			if (_ButtonItem1 != null)
			{
				_ButtonItem1.Click -= value2;
			}
			_ButtonItem1 = value;
			if (_ButtonItem1 != null)
			{
				_ButtonItem1.Click += value2;
			}
		}
	}

	internal virtual SplitContainer SplitContainer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SplitContainer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SplitContainer1 = value;
		}
	}

	internal virtual TextBox POver
	{
		[DebuggerNonUserCode]
		get
		{
			return _POver;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_POver = value;
		}
	}

	internal virtual Label LOver
	{
		[DebuggerNonUserCode]
		get
		{
			return _LOver;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LOver = value;
		}
	}

	internal virtual TextBox Tover
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tover;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tover = value;
		}
	}

	internal virtual Label Label22
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label22 = value;
		}
	}

	internal virtual Button Button2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button2_Click;
			if (_Button2 != null)
			{
				_Button2.Click -= value2;
			}
			_Button2 = value;
			if (_Button2 != null)
			{
				_Button2.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX2_Click;
			if (_ButtonX2 != null)
			{
				_ButtonX2.Click -= value2;
			}
			_ButtonX2 = value;
			if (_ButtonX2 != null)
			{
				_ButtonX2.Click += value2;
			}
		}
	}

	internal virtual ComboBox Tcusperfix
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcusperfix;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tcusperfix_SelectedIndexChanged;
			KeyEventHandler value3 = Tcusperfix_KeyDown;
			if (_Tcusperfix != null)
			{
				_Tcusperfix.SelectedIndexChanged -= value2;
				_Tcusperfix.KeyDown -= value3;
			}
			_Tcusperfix = value;
			if (_Tcusperfix != null)
			{
				_Tcusperfix.SelectedIndexChanged += value2;
				_Tcusperfix.KeyDown += value3;
			}
		}
	}

	internal virtual Label Label25
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label25;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label25 = value;
		}
	}

	internal virtual ComboBox TcusSex
	{
		[DebuggerNonUserCode]
		get
		{
			return _TcusSex;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = TcusSex_KeyDown;
			if (_TcusSex != null)
			{
				_TcusSex.KeyDown -= value2;
			}
			_TcusSex = value;
			if (_TcusSex != null)
			{
				_TcusSex.KeyDown += value2;
			}
		}
	}

	internal virtual Label Label28
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label28;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label28 = value;
		}
	}

	internal virtual Label Label32
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label32;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label32 = value;
		}
	}

	internal virtual TextBox TcusCardID
	{
		[DebuggerNonUserCode]
		get
		{
			return _TcusCardID;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TcusCardID = value;
		}
	}

	internal virtual Label Label34
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label34;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label34 = value;
		}
	}

	internal virtual TextBox Tcontry
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcontry;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcontry = value;
		}
	}

	internal virtual Panel Panel5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel5 = value;
		}
	}

	internal virtual Button Button4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button4_Click;
			if (_Button4 != null)
			{
				_Button4.Click -= value2;
			}
			_Button4 = value;
			if (_Button4 != null)
			{
				_Button4.Click += value2;
			}
		}
	}

	internal virtual ListView ListView2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListView1_SelectedIndexChanged;
			if (_ListView2 != null)
			{
				_ListView2.SelectedIndexChanged -= value2;
			}
			_ListView2 = value;
			if (_ListView2 != null)
			{
				_ListView2.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader4 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader5 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader6 = value;
		}
	}

	internal virtual Button Button10
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button10_Click_1;
			if (_Button10 != null)
			{
				_Button10.Click -= value2;
			}
			_Button10 = value;
			if (_Button10 != null)
			{
				_Button10.Click += value2;
			}
		}
	}

	internal virtual Button Button8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button8_Click_1;
			if (_Button8 != null)
			{
				_Button8.Click -= value2;
			}
			_Button8 = value;
			if (_Button8 != null)
			{
				_Button8.Click += value2;
			}
		}
	}

	internal virtual TextBox Tcontry2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcontry2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcontry2 = value;
		}
	}

	internal virtual Label Label56
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label56;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label56 = value;
		}
	}

	internal virtual TextBox Tname2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tname2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tname2 = value;
		}
	}

	internal virtual Label Label49
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label49;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label49 = value;
		}
	}

	internal virtual ButtonX ButtonX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX3_Click;
			if (_ButtonX3 != null)
			{
				_ButtonX3.Click -= value2;
			}
			_ButtonX3 = value;
			if (_ButtonX3 != null)
			{
				_ButtonX3.Click += value2;
			}
		}
	}

	internal virtual Button Button12
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button12_Click;
			if (_Button12 != null)
			{
				_Button12.Click -= value2;
			}
			_Button12 = value;
			if (_Button12 != null)
			{
				_Button12.Click += value2;
			}
		}
	}

	internal virtual ComboBox ComboBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox1_SelectedIndexChanged;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged -= value2;
			}
			_ComboBox1 = value;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label57
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label57;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label57 = value;
		}
	}

	internal virtual Button Button14
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button14_Click;
			if (_Button14 != null)
			{
				_Button14.Click -= value2;
			}
			_Button14 = value;
			if (_Button14 != null)
			{
				_Button14.Click += value2;
			}
		}
	}

	internal virtual Button Button13
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button13_Click;
			if (_Button13 != null)
			{
				_Button13.Click -= value2;
			}
			_Button13 = value;
			if (_Button13 != null)
			{
				_Button13.Click += value2;
			}
		}
	}

	internal virtual Button Button15
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button15_Click;
			if (_Button15 != null)
			{
				_Button15.Click -= value2;
			}
			_Button15 = value;
			if (_Button15 != null)
			{
				_Button15.Click += value2;
			}
		}
	}

	internal virtual Button Button16
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button16_Click;
			if (_Button16 != null)
			{
				_Button16.Click -= value2;
			}
			_Button16 = value;
			if (_Button16 != null)
			{
				_Button16.Click += value2;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader7 = value;
		}
	}

	internal virtual CheckBox CheckBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CheckBox1 = value;
		}
	}

	internal virtual Label Label58
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label58;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label58 = value;
		}
	}

	internal virtual TextBox TwTax
	{
		[DebuggerNonUserCode]
		get
		{
			return _TwTax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = NEXKEY;
			if (_TwTax != null)
			{
				_TwTax.KeyDown -= value2;
			}
			_TwTax = value;
			if (_TwTax != null)
			{
				_TwTax.KeyDown += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX3_Click;
			if (_ButtonX4 != null)
			{
				_ButtonX4.Click -= value2;
			}
			_ButtonX4 = value;
			if (_ButtonX4 != null)
			{
				_ButtonX4.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonT1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonT1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonT1_Click;
			if (_ButtonT1 != null)
			{
				_ButtonT1.Click -= value2;
			}
			_ButtonT1 = value;
			if (_ButtonT1 != null)
			{
				_ButtonT1.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonT3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonT3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonT3_Click;
			if (_ButtonT3 != null)
			{
				_ButtonT3.Click -= value2;
			}
			_ButtonT3 = value;
			if (_ButtonT3 != null)
			{
				_ButtonT3.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonT2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonT2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonT2_Click;
			if (_ButtonT2 != null)
			{
				_ButtonT2.Click -= value2;
			}
			_ButtonT2 = value;
			if (_ButtonT2 != null)
			{
				_ButtonT2.Click += value2;
			}
		}
	}

	internal virtual Label Label59
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label59;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label59 = value;
		}
	}

	internal virtual Label Label60
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label60;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label60 = value;
		}
	}

	internal virtual Label Label61
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label61;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label61 = value;
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

	internal virtual Label Label62
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label62;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label62 = value;
		}
	}

	internal virtual C1FlexGrid Grid2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Grid2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			RowColEventHandler value2 = Grid2_AfterEdit;
			RowColEventHandler value3 = Grid2_StartEdit;
			if (_Grid2 != null)
			{
				_Grid2.AfterEdit -= value2;
				_Grid2.StartEdit -= value3;
			}
			_Grid2 = value;
			if (_Grid2 != null)
			{
				_Grid2.AfterEdit += value2;
				_Grid2.StartEdit += value3;
			}
		}
	}

	internal virtual Label Label_0
	{
		[DebuggerNonUserCode]
		get
		{
			return label_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			label_0 = value;
		}
	}

	internal virtual Label Label_1
	{
		[DebuggerNonUserCode]
		get
		{
			return label_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			label_1 = value;
		}
	}

	internal virtual Label Label65
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label65;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label65 = value;
		}
	}

	internal virtual Label Label_2
	{
		[DebuggerNonUserCode]
		get
		{
			return label_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			label_2 = value;
		}
	}

	internal virtual Label Label64
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label64;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label64 = value;
		}
	}

	internal virtual ButtonX ButtonX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX5_Click;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click -= value2;
			}
			_ButtonX5 = value;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click += value2;
			}
		}
	}

	internal virtual Label Label63
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label63;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label63 = value;
		}
	}

	internal virtual ButtonX ButtonX6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX6_Click;
			if (_ButtonX6 != null)
			{
				_ButtonX6.Click -= value2;
			}
			_ButtonX6 = value;
			if (_ButtonX6 != null)
			{
				_ButtonX6.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX7_Click;
			if (_ButtonX7 != null)
			{
				_ButtonX7.Click -= value2;
			}
			_ButtonX7 = value;
			if (_ButtonX7 != null)
			{
				_ButtonX7.Click += value2;
			}
		}
	}

	internal virtual C1FlexGrid Grid1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Grid1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			RowColEventHandler value2 = Grid1_StartEdit;
			EventHandler value3 = Grid1_Click;
			RowColEventHandler value4 = Grid1_AfterEdit;
			RowColEventHandler value5 = Grid1_BeforeEdit;
			if (_Grid1 != null)
			{
				_Grid1.StartEdit -= value2;
				_Grid1.Click -= value3;
				_Grid1.AfterEdit -= value4;
				_Grid1.BeforeEdit -= value5;
			}
			_Grid1 = value;
			if (_Grid1 != null)
			{
				_Grid1.StartEdit += value2;
				_Grid1.Click += value3;
				_Grid1.AfterEdit += value4;
				_Grid1.BeforeEdit += value5;
			}
		}
	}

	internal virtual Button Button6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button6_Click;
			if (_Button6 != null)
			{
				_Button6.Click -= value2;
			}
			_Button6 = value;
			if (_Button6 != null)
			{
				_Button6.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmCheckIn()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmCheckIn()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmCheckIn_FormClosing;
		base.Load += FrmCheckIn_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EDIT_ID = "";
		tmp_no = "";
		Dep_price = default(decimal);
		tmp_room = "";
		tmp_roomarr = new ArrayList();
		Booking_cust = "";
		isbook = false;
		WORK_ID = 0;
		InitializeComponent();
	}

	[DebuggerNonUserCode]
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmCheckIn));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.Tover = new System.Windows.Forms.TextBox();
		this.Label22 = new System.Windows.Forms.Label();
		this.Label_0 = new System.Windows.Forms.Label();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.TextBox_0 = new System.Windows.Forms.TextBox();
		this.TCusTypeMain = new System.Windows.Forms.ComboBox();
		this.Label63 = new System.Windows.Forms.Label();
		this.TcusCardID = new System.Windows.Forms.TextBox();
		this.Label32 = new System.Windows.Forms.Label();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.TCusType = new System.Windows.Forms.ComboBox();
		this.ButtonT3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonT2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonT1 = new DevComponents.DotNetBar.ButtonX();
		this.TcusName = new System.Windows.Forms.TextBox();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Label34 = new System.Windows.Forms.Label();
		this.Tcontry = new System.Windows.Forms.TextBox();
		this.TextBox_1 = new System.Windows.Forms.TextBox();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.TextBox_2 = new System.Windows.Forms.TextBox();
		this.TcusSex = new System.Windows.Forms.ComboBox();
		this.Label28 = new System.Windows.Forms.Label();
		this.Tcusperfix = new System.Windows.Forms.ComboBox();
		this.Tc_tel = new System.Windows.Forms.ComboBox();
		this.Label25 = new System.Windows.Forms.Label();
		this.Label60 = new System.Windows.Forms.Label();
		this.Label19 = new System.Windows.Forms.Label();
		this.Panel3 = new System.Windows.Forms.Panel();
		this.ExpandablePanel1 = new DevComponents.DotNetBar.ExpandablePanel();
		this.Panel4 = new System.Windows.Forms.Panel();
		this.ItemPanel1 = new DevComponents.DotNetBar.ItemPanel();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.expandablePanel5 = new DevComponents.DotNetBar.ExpandablePanel();
		this.Panel2 = new System.Windows.Forms.Panel();
		this.Label62 = new System.Windows.Forms.Label();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.Label58 = new System.Windows.Forms.Label();
		this.TwTax = new System.Windows.Forms.TextBox();
		this.Tw_ampore = new System.Windows.Forms.TextBox();
		this.Label2 = new System.Windows.Forms.Label();
		this.Tw_tambon = new System.Windows.Forms.TextBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.Tw_tel = new System.Windows.Forms.ComboBox();
		this.Tw_code = new System.Windows.Forms.TextBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.Tw_road = new System.Windows.Forms.TextBox();
		this.Tw_privince = new System.Windows.Forms.TextBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label42 = new System.Windows.Forms.Label();
		this.Label43 = new System.Windows.Forms.Label();
		this.Label44 = new System.Windows.Forms.Label();
		this.Tw = new System.Windows.Forms.TextBox();
		this.Label48 = new System.Windows.Forms.Label();
		this.Tw_soi = new System.Windows.Forms.TextBox();
		this.Label45 = new System.Windows.Forms.Label();
		this.Tw_moo = new System.Windows.Forms.TextBox();
		this.Label46 = new System.Windows.Forms.Label();
		this.Tw_no = new System.Windows.Forms.TextBox();
		this.Label47 = new System.Windows.Forms.Label();
		this.Tw_fax = new System.Windows.Forms.TextBox();
		this.expandablePanel4 = new DevComponents.DotNetBar.ExpandablePanel();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.Label10 = new System.Windows.Forms.Label();
		this.Tc_fax = new System.Windows.Forms.TextBox();
		this.Tc_ampore = new System.Windows.Forms.TextBox();
		this.Label39 = new System.Windows.Forms.Label();
		this.Tc_tambon = new System.Windows.Forms.TextBox();
		this.Label38 = new System.Windows.Forms.Label();
		this.Tc_code = new System.Windows.Forms.TextBox();
		this.Label41 = new System.Windows.Forms.Label();
		this.Tc_road = new System.Windows.Forms.TextBox();
		this.Tc_province = new System.Windows.Forms.TextBox();
		this.Label37 = new System.Windows.Forms.Label();
		this.Label40 = new System.Windows.Forms.Label();
		this.Tc_soi = new System.Windows.Forms.TextBox();
		this.Label36 = new System.Windows.Forms.Label();
		this.Tc_moo = new System.Windows.Forms.TextBox();
		this.Label11 = new System.Windows.Forms.Label();
		this.Tc_no = new System.Windows.Forms.TextBox();
		this.Label8 = new System.Windows.Forms.Label();
		this.TCarType = new System.Windows.Forms.ComboBox();
		this.Label9 = new System.Windows.Forms.Label();
		this.Label31 = new System.Windows.Forms.Label();
		this.Label21 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label16 = new System.Windows.Forms.Label();
		this.Label29 = new System.Windows.Forms.Label();
		this.Label57 = new System.Windows.Forms.Label();
		this.Label27 = new System.Windows.Forms.Label();
		this.Label15 = new System.Windows.Forms.Label();
		this.Label26 = new System.Windows.Forms.Label();
		this.Label50 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.TCarID = new System.Windows.Forms.TextBox();
		this.TbookNo = new System.Windows.Forms.TextBox();
		this.TCusID = new System.Windows.Forms.TextBox();
		this.TdocNum = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.PanelCust = new System.Windows.Forms.Panel();
		this.Label61 = new System.Windows.Forms.Label();
		this.Label59 = new System.Windows.Forms.Label();
		this.Button2 = new System.Windows.Forms.Button();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Label_1 = new System.Windows.Forms.Label();
		this.Label65 = new System.Windows.Forms.Label();
		this.Label_2 = new System.Windows.Forms.Label();
		this.Label64 = new System.Windows.Forms.Label();
		this.Button15 = new System.Windows.Forms.Button();
		this.Button16 = new System.Windows.Forms.Button();
		this.Panel5 = new System.Windows.Forms.Panel();
		this.Button12 = new System.Windows.Forms.Button();
		this.Button10 = new System.Windows.Forms.Button();
		this.Button8 = new System.Windows.Forms.Button();
		this.Tcontry2 = new System.Windows.Forms.TextBox();
		this.Label56 = new System.Windows.Forms.Label();
		this.Tname2 = new System.Windows.Forms.TextBox();
		this.Label49 = new System.Windows.Forms.Label();
		this.Button4 = new System.Windows.Forms.Button();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.SplitContainer1 = new System.Windows.Forms.SplitContainer();
		this.Button6 = new System.Windows.Forms.Button();
		this.Button14 = new System.Windows.Forms.Button();
		this.Button13 = new System.Windows.Forms.Button();
		this.Label14 = new System.Windows.Forms.Label();
		this.Button3 = new System.Windows.Forms.Button();
		this.Button11 = new System.Windows.Forms.Button();
		this.Grid1 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.Label52 = new System.Windows.Forms.Label();
		this.Label51 = new System.Windows.Forms.Label();
		this.Tstart = new System.Windows.Forms.DateTimePicker();
		this.Tend = new System.Windows.Forms.DateTimePicker();
		this.Label53 = new System.Windows.Forms.Label();
		this.Tnum = new System.Windows.Forms.TextBox();
		this.Label54 = new System.Windows.Forms.Label();
		this.LabelButton7 = new System.Windows.Forms.Label();
		this.Tnote = new System.Windows.Forms.TextBox();
		this.Tdebt = new System.Windows.Forms.TextBox();
		this.POver = new System.Windows.Forms.TextBox();
		this.Tcash = new System.Windows.Forms.TextBox();
		this.Tpay = new System.Windows.Forms.TextBox();
		this.TselectRoom = new System.Windows.Forms.ComboBox();
		this.Label55 = new System.Windows.Forms.Label();
		this.Grid2 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.Button9 = new System.Windows.Forms.Button();
		this.Label35 = new System.Windows.Forms.Label();
		this.LOver = new System.Windows.Forms.Label();
		this.Label30 = new System.Windows.Forms.Label();
		this.Label23 = new System.Windows.Forms.Label();
		this.LabelDebt = new System.Windows.Forms.Label();
		this.Label18 = new System.Windows.Forms.Label();
		this.LabelPayed = new System.Windows.Forms.Label();
		this.Label33 = new System.Windows.Forms.Label();
		this.Labelroompro = new System.Windows.Forms.Label();
		this.Label20 = new System.Windows.Forms.Label();
		this.LabelTpro = new System.Windows.Forms.Label();
		this.Label17 = new System.Windows.Forms.Label();
		this.LabelTroom = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.Label24 = new System.Windows.Forms.Label();
		this.Button5 = new System.Windows.Forms.Button();
		this.Label12 = new System.Windows.Forms.Label();
		this.Button_DEP = new System.Windows.Forms.Button();
		this.Button_REG = new System.Windows.Forms.Button();
		this.Button7 = new System.Windows.Forms.Button();
		this.ButtonItem15 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem43 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem40 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem44 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem45 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem46 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem16 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem17 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem20 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem32 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem33 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem11 = new DevComponents.DotNetBar.ButtonItem();
		this.TimerDrop = new System.Windows.Forms.Timer(this.components);
		this.TimerDrop2 = new System.Windows.Forms.Timer(this.components);
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.ButtonItem12 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem13 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem14 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem7 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem28 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem29 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem30 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem31 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem2 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem41 = new DevComponents.DotNetBar.ButtonItem();
		this.GroupBox1.SuspendLayout();
		this.Panel3.SuspendLayout();
		this.ExpandablePanel1.SuspendLayout();
		this.Panel4.SuspendLayout();
		this.expandablePanel5.SuspendLayout();
		this.Panel2.SuspendLayout();
		this.expandablePanel4.SuspendLayout();
		this.Panel1.SuspendLayout();
		this.PanelCust.SuspendLayout();
		this.PanelEx1.SuspendLayout();
		this.Panel5.SuspendLayout();
		this.SplitContainer1.Panel1.SuspendLayout();
		this.SplitContainer1.Panel2.SuspendLayout();
		this.SplitContainer1.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.Grid1).BeginInit();
		((System.ComponentModel.ISupportInitialize)this.Grid2).BeginInit();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.ButtonX7);
		this.GroupBox1.Controls.Add(this.ButtonX2);
		this.GroupBox1.Controls.Add(this.Tover);
		this.GroupBox1.Controls.Add(this.Label22);
		this.GroupBox1.Controls.Add(this.Label_0);
		this.GroupBox1.Controls.Add(this.ButtonX6);
		this.GroupBox1.Controls.Add(this.TextBox_0);
		this.GroupBox1.Controls.Add(this.TCusTypeMain);
		this.GroupBox1.Controls.Add(this.Label63);
		this.GroupBox1.Controls.Add(this.TcusCardID);
		this.GroupBox1.Controls.Add(this.Label32);
		this.GroupBox1.Controls.Add(this.ButtonX5);
		this.GroupBox1.Controls.Add(this.TCusType);
		this.GroupBox1.Controls.Add(this.ButtonT3);
		this.GroupBox1.Controls.Add(this.ButtonT2);
		this.GroupBox1.Controls.Add(this.ButtonT1);
		this.GroupBox1.Controls.Add(this.TcusName);
		this.GroupBox1.Controls.Add(this.CheckBox1);
		this.GroupBox1.Controls.Add(this.Label34);
		this.GroupBox1.Controls.Add(this.Tcontry);
		this.GroupBox1.Controls.Add(this.TextBox_1);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.TextBox_2);
		this.GroupBox1.Controls.Add(this.TcusSex);
		this.GroupBox1.Controls.Add(this.Label28);
		this.GroupBox1.Controls.Add(this.Tcusperfix);
		this.GroupBox1.Controls.Add(this.Tc_tel);
		this.GroupBox1.Controls.Add(this.Label25);
		this.GroupBox1.Controls.Add(this.Label60);
		this.GroupBox1.Controls.Add(this.Label19);
		this.GroupBox1.Controls.Add(this.Panel3);
		this.GroupBox1.Controls.Add(this.TCarType);
		this.GroupBox1.Controls.Add(this.Label9);
		this.GroupBox1.Controls.Add(this.Label31);
		this.GroupBox1.Controls.Add(this.Label21);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label16);
		this.GroupBox1.Controls.Add(this.Label29);
		this.GroupBox1.Controls.Add(this.Label57);
		this.GroupBox1.Controls.Add(this.Label27);
		this.GroupBox1.Controls.Add(this.Label15);
		this.GroupBox1.Controls.Add(this.Label26);
		this.GroupBox1.Controls.Add(this.Label50);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.TCarID);
		this.GroupBox1.Controls.Add(this.TbookNo);
		this.GroupBox1.Controls.Add(this.TCusID);
		this.GroupBox1.Controls.Add(this.TdocNum);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(3, 3);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1257, 331);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายละเอ\u0e35ยดการ Check-In";
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.FocusCuesEnabled = false;
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX7;
		location = new System.Drawing.Point(520, 49);
		buttonX.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX7;
		size = new System.Drawing.Size(138, 23);
		buttonX2.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 79;
		this.ButtonX7.Text = "อ\u0e48านจาก SmartCard";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(842, 43);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		this.ButtonX2.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(82, 36);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 18;
		this.ButtonX2.Text = "ใช\u0e49ยอดน\u0e35\u0e49จ\u0e48าย";
		this.Tover.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tover.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tover.Font = new System.Drawing.Font("Tahoma", 18f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Tover.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tover = this.Tover;
		location = new System.Drawing.Point(752, 43);
		tover.Location = location;
		System.Windows.Forms.TextBox tover2 = this.Tover;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tover2.Margin = margin;
		this.Tover.Name = "Tover";
		this.Tover.ReadOnly = true;
		System.Windows.Forms.TextBox tover3 = this.Tover;
		size = new System.Drawing.Size(91, 36);
		tover3.Size = size;
		this.Tover.TabIndex = 17;
		this.Tover.Text = "1000";
		this.Tover.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label22.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label22.ForeColor = System.Drawing.Color.MediumBlue;
		System.Windows.Forms.Label label = this.Label22;
		location = new System.Drawing.Point(778, 11);
		label.Location = location;
		this.Label22.Name = "Label22";
		System.Windows.Forms.Label label2 = this.Label22;
		size = new System.Drawing.Size(122, 25);
		label2.Size = size;
		this.Label22.TabIndex = 47;
		this.Label22.Text = "เง\u0e34นจ\u0e48ายล\u0e48วงหน\u0e49า";
		this.Label22.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label_0.AutoSize = true;
		this.Label_0.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label_0.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label3 = this.Label_0;
		location = new System.Drawing.Point(198, 37);
		label3.Location = location;
		this.Label_0.Name = "Labelจอง";
		System.Windows.Forms.Label label4 = this.Label_0;
		size = new System.Drawing.Size(213, 13);
		label4.Size = size;
		this.Label_0.TabIndex = 75;
		this.Label_0.Text = "*ไม\u0e48สามารถเปล\u0e35\u0e48ยนได\u0e49เน\u0e37\u0e48องจากม\u0e35ยอดเง\u0e34นจอง*";
		this.Label_0.Visible = false;
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX6.FocusCuesEnabled = false;
		this.ButtonX6.Image = (System.Drawing.Image)resources.GetObject("ButtonX6.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX6;
		location = new System.Drawing.Point(310, 21);
		buttonX5.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX6;
		size = new System.Drawing.Size(92, 23);
		buttonX6.Size = size;
		this.ButtonX6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX6.TabIndex = 78;
		this.ButtonX6.Text = "เร\u0e34\u0e48มใหม\u0e48";
		System.Windows.Forms.TextBox textBox_ = this.TextBox_0;
		location = new System.Drawing.Point(119, 46);
		textBox_.Location = location;
		System.Windows.Forms.TextBox textBox_2 = this.TextBox_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_2.Margin = margin;
		this.TextBox_0.Name = "TCusSearch";
		System.Windows.Forms.TextBox textBox_3 = this.TextBox_0;
		size = new System.Drawing.Size(144, 23);
		textBox_3.Size = size;
		this.TextBox_0.TabIndex = 5;
		this.TCusTypeMain.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TCusTypeMain.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tCusTypeMain = this.TCusTypeMain;
		location = new System.Drawing.Point(119, 80);
		tCusTypeMain.Location = location;
		System.Windows.Forms.ComboBox tCusTypeMain2 = this.TCusTypeMain;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusTypeMain2.Margin = margin;
		this.TCusTypeMain.Name = "TCusTypeMain";
		System.Windows.Forms.ComboBox tCusTypeMain3 = this.TCusTypeMain;
		size = new System.Drawing.Size(144, 24);
		tCusTypeMain3.Size = size;
		this.TCusTypeMain.TabIndex = 12;
		this.Label63.AutoSize = true;
		this.Label63.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label63.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label5 = this.Label63;
		location = new System.Drawing.Point(115, 68);
		label5.Location = location;
		this.Label63.Name = "Label63";
		System.Windows.Forms.Label label6 = this.Label63;
		size = new System.Drawing.Size(147, 13);
		label6.Size = size;
		this.Label63.TabIndex = 77;
		this.Label63.Text = "ค\u0e49นหาล\u0e39กค\u0e49าเก\u0e48าในช\u0e48องเบอร\u0e4cโทร";
		this.TcusCardID.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tcusCardID = this.TcusCardID;
		location = new System.Drawing.Point(752, 105);
		tcusCardID.Location = location;
		System.Windows.Forms.TextBox tcusCardID2 = this.TcusCardID;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcusCardID2.Margin = margin;
		this.TcusCardID.Name = "TcusCardID";
		System.Windows.Forms.TextBox tcusCardID3 = this.TcusCardID;
		size = new System.Drawing.Size(172, 23);
		tcusCardID3.Size = size;
		this.TcusCardID.TabIndex = 11;
		this.Label32.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label32;
		location = new System.Drawing.Point(666, 109);
		label7.Location = location;
		this.Label32.Name = "Label32";
		System.Windows.Forms.Label label8 = this.Label32;
		size = new System.Drawing.Size(80, 16);
		label8.Size = size;
		this.Label32.TabIndex = 47;
		this.Label32.Text = "หมายเลขบ\u0e31ตร";
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.FocusCuesEnabled = false;
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX5;
		location = new System.Drawing.Point(419, 49);
		buttonX7.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX5;
		size = new System.Drawing.Size(100, 23);
		buttonX8.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 76;
		this.ButtonX5.Text = "ล\u0e39กค\u0e49าใหม\u0e48";
		this.TCusType.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TCusType.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tCusType = this.TCusType;
		location = new System.Drawing.Point(119, 108);
		tCusType.Location = location;
		System.Windows.Forms.ComboBox tCusType2 = this.TCusType;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusType2.Margin = margin;
		this.TCusType.Name = "TCusType";
		System.Windows.Forms.ComboBox tCusType3 = this.TCusType;
		size = new System.Drawing.Size(144, 24);
		tCusType3.Size = size;
		this.TCusType.TabIndex = 13;
		this.ButtonT3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonT3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonT3.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonT = this.ButtonT3;
		location = new System.Drawing.Point(240, 136);
		buttonT.Location = location;
		this.ButtonT3.Name = "ButtonT3";
		DevComponents.DotNetBar.ButtonX buttonT2 = this.ButtonT3;
		size = new System.Drawing.Size(58, 23);
		buttonT2.Size = size;
		this.ButtonT3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonT3.TabIndex = 74;
		this.ButtonT3.Text = "รายเด\u0e37อน";
		this.ButtonT2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonT2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonT2.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonT3 = this.ButtonT2;
		location = new System.Drawing.Point(170, 136);
		buttonT3.Location = location;
		this.ButtonT2.Name = "ButtonT2";
		DevComponents.DotNetBar.ButtonX buttonT4 = this.ButtonT2;
		size = new System.Drawing.Size(64, 23);
		buttonT4.Size = size;
		this.ButtonT2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonT2.TabIndex = 73;
		this.ButtonT2.Text = "รายช\u0e31\u0e48วโมง";
		this.ButtonT1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonT1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonT1.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonT5 = this.ButtonT1;
		location = new System.Drawing.Point(119, 136);
		buttonT5.Location = location;
		this.ButtonT1.Name = "ButtonT1";
		DevComponents.DotNetBar.ButtonX buttonT6 = this.ButtonT1;
		size = new System.Drawing.Size(47, 23);
		buttonT6.Size = size;
		this.ButtonT1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonT1.TabIndex = 72;
		this.ButtonT1.Text = "รายว\u0e31น";
		this.TcusName.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TcusName.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox tcusName = this.TcusName;
		location = new System.Drawing.Point(419, 78);
		tcusName.Location = location;
		System.Windows.Forms.TextBox tcusName2 = this.TcusName;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcusName2.Margin = margin;
		this.TcusName.Name = "TcusName";
		System.Windows.Forms.TextBox tcusName3 = this.TcusName;
		size = new System.Drawing.Size(100, 23);
		tcusName3.Size = size;
		this.TcusName.TabIndex = 8;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(336, 137);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(162, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 71;
		this.CheckBox1.Text = "ผ\u0e39\u0e49เข\u0e49าพ\u0e31กเป\u0e47นล\u0e39กค\u0e49าต\u0e48างชาต\u0e34";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.Label34.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label34;
		location = new System.Drawing.Point(494, 138);
		label9.Location = location;
		this.Label34.Name = "Label34";
		System.Windows.Forms.Label label10 = this.Label34;
		size = new System.Drawing.Size(49, 16);
		label10.Size = size;
		this.Label34.TabIndex = 55;
		this.Label34.Text = "ประเทศ";
		this.Tcontry.BackColor = System.Drawing.Color.White;
		this.Tcontry.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tcontry = this.Tcontry;
		location = new System.Drawing.Point(545, 134);
		tcontry.Location = location;
		System.Windows.Forms.TextBox tcontry2 = this.Tcontry;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcontry2.Margin = margin;
		this.Tcontry.Name = "Tcontry";
		System.Windows.Forms.TextBox tcontry3 = this.Tcontry;
		size = new System.Drawing.Size(113, 23);
		tcontry3.Size = size;
		this.Tcontry.TabIndex = 54;
		this.TextBox_1.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox textBox_4 = this.TextBox_1;
		location = new System.Drawing.Point(943, 117);
		textBox_4.Location = location;
		System.Windows.Forms.TextBox textBox_5 = this.TextBox_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_5.Margin = margin;
		this.TextBox_1.Name = "TCusEmail";
		System.Windows.Forms.TextBox textBox_6 = this.TextBox_1;
		size = new System.Drawing.Size(101, 23);
		textBox_6.Size = size;
		this.TextBox_1.TabIndex = 16;
		this.TextBox_1.Visible = false;
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "รายงว\u0e31น", "รายช\u0e31\u0e48วโมง", "รายเด\u0e37อน" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(752, 134);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(172, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 70;
		this.ComboBox1.Visible = false;
		this.TextBox_2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox_2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox textBox_7 = this.TextBox_2;
		location = new System.Drawing.Point(557, 78);
		textBox_7.Location = location;
		System.Windows.Forms.TextBox textBox_8 = this.TextBox_2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_8.Margin = margin;
		this.TextBox_2.Name = "TCusName2";
		System.Windows.Forms.TextBox textBox_9 = this.TextBox_2;
		size = new System.Drawing.Size(101, 23);
		textBox_9.Size = size;
		this.TextBox_2.TabIndex = 9;
		this.TcusSex.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.TcusSex.FormattingEnabled = true;
		this.TcusSex.Items.AddRange(new object[2] { "ชาย", "หญ\u0e34ง" });
		System.Windows.Forms.ComboBox tcusSex = this.TcusSex;
		location = new System.Drawing.Point(1149, 23);
		tcusSex.Location = location;
		this.TcusSex.Name = "TcusSex";
		System.Windows.Forms.ComboBox tcusSex2 = this.TcusSex;
		size = new System.Drawing.Size(48, 24);
		tcusSex2.Size = size;
		this.TcusSex.TabIndex = 10;
		this.TcusSex.Text = "ชาย";
		this.TcusSex.Visible = false;
		this.Label28.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label28;
		location = new System.Drawing.Point(1117, 27);
		label11.Location = location;
		this.Label28.Name = "Label28";
		System.Windows.Forms.Label label12 = this.Label28;
		size = new System.Drawing.Size(29, 16);
		label12.Size = size;
		this.Label28.TabIndex = 69;
		this.Label28.Text = "เพศ";
		this.Label28.Visible = false;
		this.Tcusperfix.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tcusperfix.ForeColor = System.Drawing.Color.Blue;
		this.Tcusperfix.FormattingEnabled = true;
		this.Tcusperfix.Items.AddRange(new object[8] { "", "นาย", "นาง", "นางสาว", "Mr.", "Mrs.", "Miss.", "ค\u0e38ณ" });
		System.Windows.Forms.ComboBox tcusperfix = this.Tcusperfix;
		location = new System.Drawing.Point(336, 78);
		tcusperfix.Location = location;
		this.Tcusperfix.Name = "Tcusperfix";
		System.Windows.Forms.ComboBox tcusperfix2 = this.Tcusperfix;
		size = new System.Drawing.Size(56, 24);
		tcusperfix2.Size = size;
		this.Tcusperfix.TabIndex = 7;
		this.Tcusperfix.Text = "นาย";
		this.Tc_tel.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tc_tel = this.Tc_tel;
		location = new System.Drawing.Point(910, 108);
		tc_tel.Location = location;
		System.Windows.Forms.ComboBox tc_tel2 = this.Tc_tel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_tel2.Margin = margin;
		this.Tc_tel.Name = "Tc_tel";
		System.Windows.Forms.ComboBox tc_tel3 = this.Tc_tel;
		size = new System.Drawing.Size(39, 24);
		tc_tel3.Size = size;
		this.Tc_tel.TabIndex = 8;
		this.Tc_tel.Visible = false;
		this.Label25.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label25;
		location = new System.Drawing.Point(276, 82);
		label13.Location = location;
		this.Label25.Name = "Label25";
		System.Windows.Forms.Label label14 = this.Label25;
		size = new System.Drawing.Size(58, 16);
		label14.Size = size;
		this.Label25.TabIndex = 67;
		this.Label25.Text = "คำนำหน\u0e49า";
		this.Label60.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label60;
		location = new System.Drawing.Point(62, 49);
		label15.Location = location;
		this.Label60.Name = "Label60";
		System.Windows.Forms.Label label16 = this.Label60;
		size = new System.Drawing.Size(55, 16);
		label16.Size = size;
		this.Label60.TabIndex = 63;
		this.Label60.Text = "เบอร\u0e4cโทร";
		this.Label19.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label19;
		location = new System.Drawing.Point(55, 84);
		label17.Location = location;
		this.Label19.Name = "Label19";
		System.Windows.Forms.Label label18 = this.Label19;
		size = new System.Drawing.Size(62, 16);
		label18.Size = size;
		this.Label19.TabIndex = 63;
		this.Label19.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.Panel3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Panel3.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Panel3.Controls.Add(this.ExpandablePanel1);
		this.Panel3.Controls.Add(this.expandablePanel5);
		this.Panel3.Controls.Add(this.expandablePanel4);
		System.Windows.Forms.Panel panel = this.Panel3;
		location = new System.Drawing.Point(4, 164);
		panel.Location = location;
		this.Panel3.Name = "Panel3";
		System.Windows.Forms.Panel panel2 = this.Panel3;
		size = new System.Drawing.Size(1245, 160);
		panel2.Size = size;
		this.Panel3.TabIndex = 61;
		this.ExpandablePanel1.CanvasColor = System.Drawing.SystemColors.Control;
		this.ExpandablePanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.ExpandablePanel1.Controls.Add(this.Panel4);
		this.ExpandablePanel1.Dock = System.Windows.Forms.DockStyle.Top;
		this.ExpandablePanel1.Expanded = false;
		DevComponents.DotNetBar.ExpandablePanel expandablePanel = this.ExpandablePanel1;
		System.Drawing.Rectangle expandedBounds = new System.Drawing.Rectangle(0, 52, 1243, 105);
		expandablePanel.ExpandedBounds = expandedBounds;
		this.ExpandablePanel1.ExpandOnTitleClick = true;
		this.ExpandablePanel1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ExpandablePanel expandablePanel2 = this.ExpandablePanel1;
		location = new System.Drawing.Point(0, 130);
		expandablePanel2.Location = location;
		this.ExpandablePanel1.Name = "ExpandablePanel1";
		DevComponents.DotNetBar.ExpandablePanel expandablePanel3 = this.ExpandablePanel1;
		size = new System.Drawing.Size(1243, 26);
		expandablePanel3.Size = size;
		this.ExpandablePanel1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.ExpandablePanel1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarBackground;
		this.ExpandablePanel1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarBackground2;
		this.ExpandablePanel1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.ExpandablePanel1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarDockedBorder;
		this.ExpandablePanel1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.ItemText;
		this.ExpandablePanel1.Style.GradientAngle = 90;
		this.ExpandablePanel1.TabIndex = 4;
		this.ExpandablePanel1.TitleStyle.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.ExpandablePanel1.TitleStyle.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.ExpandablePanel1.TitleStyle.Border = DevComponents.DotNetBar.eBorderType.RaisedInner;
		this.ExpandablePanel1.TitleStyle.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.ExpandablePanel1.TitleStyle.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.ExpandablePanel1.TitleStyle.GradientAngle = 90;
		this.ExpandablePanel1.TitleStyle.MarginLeft = 12;
		this.ExpandablePanel1.TitleText = "สแกนเอกสาร (บ\u0e31ตรประชาชน, Passport, อ\u0e37\u0e48นๆ)";
		this.Panel4.BackColor = System.Drawing.Color.LightCyan;
		this.Panel4.Controls.Add(this.ItemPanel1);
		this.Panel4.Controls.Add(this.ButtonX1);
		this.Panel4.Dock = System.Windows.Forms.DockStyle.Fill;
		this.Panel4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		System.Windows.Forms.Panel panel3 = this.Panel4;
		location = new System.Drawing.Point(0, 26);
		panel3.Location = location;
		this.Panel4.Name = "Panel4";
		System.Windows.Forms.Panel panel4 = this.Panel4;
		size = new System.Drawing.Size(1243, 0);
		panel4.Size = size;
		this.Panel4.TabIndex = 2;
		this.ItemPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ItemPanel1.BackgroundStyle.Class = "ItemPanel";
		this.ItemPanel1.ContainerControlProcessDialogKey = true;
		this.ItemPanel1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem1 });
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel1;
		location = new System.Drawing.Point(98, 2);
		itemPanel.Location = location;
		this.ItemPanel1.Name = "ItemPanel1";
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel1;
		size = new System.Drawing.Size(1143, 74);
		itemPanel2.Size = size;
		this.ItemPanel1.TabIndex = 1;
		this.ItemPanel1.Text = "ItemPanel1";
		this.ButtonItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem1.Image = iHOTEL2025.My.Resources.Resources.star;
		this.ButtonItem1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.Text = "11/12/55\r\n46464646";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		this.ButtonX1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX1;
		location = new System.Drawing.Point(13, 10);
		buttonX9.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX1;
		size = new System.Drawing.Size(73, 59);
		buttonX10.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 0;
		this.ButtonX1.Text = "เพ\u0e34\u0e48มเอกสาร";
		this.expandablePanel5.CanvasColor = System.Drawing.SystemColors.Control;
		this.expandablePanel5.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.expandablePanel5.Controls.Add(this.Panel2);
		this.expandablePanel5.Dock = System.Windows.Forms.DockStyle.Top;
		this.expandablePanel5.ExpandOnTitleClick = true;
		this.expandablePanel5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ExpandablePanel expandablePanel4 = this.expandablePanel5;
		location = new System.Drawing.Point(0, 26);
		expandablePanel4.Location = location;
		this.expandablePanel5.Name = "expandablePanel5";
		DevComponents.DotNetBar.ExpandablePanel expandablePanel5 = this.expandablePanel5;
		size = new System.Drawing.Size(1243, 104);
		expandablePanel5.Size = size;
		this.expandablePanel5.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.expandablePanel5.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarBackground;
		this.expandablePanel5.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarBackground2;
		this.expandablePanel5.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.expandablePanel5.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarDockedBorder;
		this.expandablePanel5.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.ItemText;
		this.expandablePanel5.Style.GradientAngle = 90;
		this.expandablePanel5.TabIndex = 3;
		this.expandablePanel5.TitleStyle.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.expandablePanel5.TitleStyle.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.expandablePanel5.TitleStyle.Border = DevComponents.DotNetBar.eBorderType.RaisedInner;
		this.expandablePanel5.TitleStyle.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.expandablePanel5.TitleStyle.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.expandablePanel5.TitleStyle.GradientAngle = 90;
		this.expandablePanel5.TitleStyle.MarginLeft = 12;
		this.expandablePanel5.TitleText = "สถานท\u0e35\u0e48ทำงาน (สำหร\u0e31บออกใบกำก\u0e31บภาษ\u0e35)";
		this.Panel2.BackColor = System.Drawing.Color.LavenderBlush;
		this.Panel2.Controls.Add(this.Label62);
		this.Panel2.Controls.Add(this.ButtonX4);
		this.Panel2.Controls.Add(this.Label58);
		this.Panel2.Controls.Add(this.TwTax);
		this.Panel2.Controls.Add(this.Tw_ampore);
		this.Panel2.Controls.Add(this.Label2);
		this.Panel2.Controls.Add(this.Tw_tambon);
		this.Panel2.Controls.Add(this.Label3);
		this.Panel2.Controls.Add(this.Tw_tel);
		this.Panel2.Controls.Add(this.Tw_code);
		this.Panel2.Controls.Add(this.Label4);
		this.Panel2.Controls.Add(this.Tw_road);
		this.Panel2.Controls.Add(this.Tw_privince);
		this.Panel2.Controls.Add(this.Label5);
		this.Panel2.Controls.Add(this.Label42);
		this.Panel2.Controls.Add(this.Label43);
		this.Panel2.Controls.Add(this.Label44);
		this.Panel2.Controls.Add(this.Tw);
		this.Panel2.Controls.Add(this.Label48);
		this.Panel2.Controls.Add(this.Tw_soi);
		this.Panel2.Controls.Add(this.Label45);
		this.Panel2.Controls.Add(this.Tw_moo);
		this.Panel2.Controls.Add(this.Label46);
		this.Panel2.Controls.Add(this.Tw_no);
		this.Panel2.Controls.Add(this.Label47);
		this.Panel2.Controls.Add(this.Tw_fax);
		this.Panel2.Dock = System.Windows.Forms.DockStyle.Fill;
		this.Panel2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		System.Windows.Forms.Panel panel5 = this.Panel2;
		location = new System.Drawing.Point(0, 26);
		panel5.Location = location;
		this.Panel2.Name = "Panel2";
		System.Windows.Forms.Panel panel6 = this.Panel2;
		size = new System.Drawing.Size(1243, 78);
		panel6.Size = size;
		this.Panel2.TabIndex = 2;
		this.Label62.AutoSize = true;
		this.Label62.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label19 = this.Label62;
		location = new System.Drawing.Point(557, 6);
		label19.Location = location;
		this.Label62.Name = "Label62";
		System.Windows.Forms.Label label20 = this.Label62;
		size = new System.Drawing.Size(391, 16);
		label20.Size = size;
		this.Label62.TabIndex = 58;
		this.Label62.Text = "* กรณ\u0e35จะพ\u0e34มพ\u0e4cท\u0e35\u0e48อย\u0e39\u0e48เองไม\u0e48ใส\u0e48ตามช\u0e48องให\u0e49พ\u0e34มพ\u0e4cท\u0e35\u0e48ช\u0e48อง เลขท\u0e35\u0e48 ช\u0e48องเด\u0e35ยวเท\u0e48าน\u0e31\u0e49น";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Checked = true;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ButtonX4.ForeColor = System.Drawing.Color.DarkBlue;
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX4;
		location = new System.Drawing.Point(1015, 12);
		buttonX11.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX4;
		size = new System.Drawing.Size(208, 54);
		buttonX12.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 57;
		this.ButtonX4.Text = "รายช\u0e37\u0e48อผ\u0e39\u0e49เข\u0e49าพ\u0e31ก";
		this.Label58.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label58;
		location = new System.Drawing.Point(765, 56);
		label21.Location = location;
		this.Label58.Name = "Label58";
		System.Windows.Forms.Label label22 = this.Label58;
		size = new System.Drawing.Size(75, 16);
		label22.Size = size;
		this.Label58.TabIndex = 49;
		this.Label58.Text = "เลขประจำต\u0e31ว";
		System.Windows.Forms.TextBox twTax = this.TwTax;
		location = new System.Drawing.Point(842, 53);
		twTax.Location = location;
		System.Windows.Forms.TextBox twTax2 = this.TwTax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		twTax2.Margin = margin;
		this.TwTax.Name = "TwTax";
		System.Windows.Forms.TextBox twTax3 = this.TwTax;
		size = new System.Drawing.Size(130, 23);
		twTax3.Size = size;
		this.TwTax.TabIndex = 48;
		this.Tw_ampore.AutoCompleteCustomSource.AddRange(new string[926]
		{
			"เม\u0e37องกระบ\u0e35\u0e48", "เกาะล\u0e31นตา", "เขาพนม", "คลองท\u0e48อม", "ปลายพระยา", "ลำท\u0e31บ", "เหน\u0e37อคลอง", "อ\u0e48าวล\u0e36ก", "คลองสาน", "คลองเตย",
			"จอมทอง", "จต\u0e38จ\u0e31กร", "ด\u0e38ส\u0e34ต", "ดอนเม\u0e37อง", "ตล\u0e34\u0e48งช\u0e31น", "ธนบ\u0e38ร\u0e35", "บางกอกน\u0e49อย", "บางกอกใหญ\u0e48", "บางกะป\u0e34", "บางข\u0e38นเท\u0e35ยน",
			"บางเขน", "บางคอแหลม", "บางซ\u0e37\u0e48อ", "บางพล\u0e31ด", "บางร\u0e31ก", "บ\u0e36งก\u0e38\u0e48ม", "ประเวศ", "ปท\u0e38มว\u0e31น", "ป\u0e49อมปราบศ\u0e31ตร\u0e39พ\u0e48าย", "พญาไท",
			"พระโขนง", "พระนคร", "ภาษ\u0e35เจร\u0e34ญ", "ม\u0e35นบ\u0e38ร\u0e35", "ยานนาวา", "ราชเทว\u0e35", "ราษฎร\u0e4cบ\u0e39รณะ", "ลาดกระบ\u0e31ง", "ลาดพร\u0e49าว", "สาทร",
			"ส\u0e31มพ\u0e31นธวงศ\u0e4c", "หนองแขม", "หนองจอก", "ห\u0e49วยขวาง", "สวนหลวง", "ด\u0e34นแดง", "หล\u0e31กส\u0e35\u0e48", "สายไหม", "ค\u0e31นนายาว", "สะพานส\u0e39ง",
			"ว\u0e31งทองหลาง", "คลองสามวา", "ว\u0e31ฒนา", "บางนา", "ทว\u0e35ว\u0e31ฒนา", "บางแค", "ท\u0e38\u0e48งคร\u0e38", "บางบอน", "เม\u0e37องกาญจนบ\u0e38ร\u0e35", "ด\u0e48านมะขามเต\u0e35\u0e49ย",
			"ทองผาภ\u0e39ม\u0e34", "ท\u0e48าม\u0e48วง", "ท\u0e48ามะกา", "ไทรโยค", "บ\u0e48อพลอย", "พนมทวน", "เลาขว\u0e31ญ", "ศร\u0e35สว\u0e31สด\u0e34\u0e4c", "ส\u0e31งขละบ\u0e38ร\u0e35", "หนองปร\u0e37อ",
			"ห\u0e49วยกระเจา", "เม\u0e37องกาฬส\u0e34นธ\u0e38\u0e4c", "กมลาไสย", "ก\u0e38ฉ\u0e34นารายณ\u0e4c", "เขาวง", "คำม\u0e48วง", "ท\u0e48าค\u0e31นโท", "นามน", "ยางตลาด", "ร\u0e48องคำ",
			"สมเด\u0e47จ", "สห\u0e31สข\u0e31นธ\u0e4c", "หนองก\u0e38งศร\u0e35", "ห\u0e49วยผ\u0e36\u0e49ง", "ห\u0e49วยเม\u0e47ก", "นาค\u0e39", "สามช\u0e31ย", "ดอนจาน", "ฆ\u0e49องช\u0e31ย", "เม\u0e37องกำแพงเพชร",
			"ขาณ\u0e38วรล\u0e31กษบ\u0e38ร\u0e35", "คลองขล\u0e38ง", "คลองลาน", "ทรายทองว\u0e31ฒนา", "ไทรงาม", "ปางศ\u0e34ลาทอง", "พรานกระต\u0e48าย", "ลานกระบ\u0e37อ", "บ\u0e36งสาม\u0e31คค\u0e35", "โกส\u0e31มพ\u0e35นคร",
			"เม\u0e37องขอนแก\u0e48น", "กระนวน", "เขาสวนกวาง", "โคกโพธ\u0e34\u0e4cไชย", "ชำส\u0e39ง", "ชนบท", "ช\u0e38มแพ", "น\u0e49ำพอง", "บ\u0e49านไผ\u0e48", "บ\u0e49านฝาง",
			"เป\u0e37อยน\u0e49อย", "พล", "พระย\u0e37น", "ภ\u0e39เว\u0e35ยง", "ภ\u0e39ผาม\u0e48าน", "ม\u0e31ญจาค\u0e35ร\u0e35", "แวงน\u0e49อย", "แวงใหญ\u0e48", "ส\u0e35ชมพ\u0e39", "หนองสองห\u0e49อง",
			"หนองเร\u0e37อ", "หนองนาคำ", "อ\u0e38บลร\u0e31ตน\u0e4c", "โนนศ\u0e34ลา", "บ\u0e49านแฮด", "เม\u0e37องจ\u0e31นทบ\u0e38ร\u0e35", "แก\u0e48งหางแมว", "ขล\u0e38ง", "ท\u0e48าใหม\u0e48", "นายายอาม",
			"โป\u0e48งน\u0e49ำร\u0e49อน", "มะขาม", "สอยดาว", "แหลมส\u0e34งห\u0e4c", "เขาค\u0e34ชฌก\u0e39ฏ", "เม\u0e37องฉะเช\u0e34งเทรา", "บางคล\u0e49า", "บางน\u0e49ำเปร\u0e35\u0e49ยว", "บางปะกง", "บ\u0e49านโพธ\u0e34\u0e4c",
			"แปลงยาว", "พนมสารคาม", "ราชสาส\u0e4cน", "สนามช\u0e31ยเขต", "ท\u0e48าตะเก\u0e35ยบ", "คลองเข\u0e37\u0e48อน", "เม\u0e37องชลบ\u0e38ร\u0e35", "เกาะส\u0e35ช\u0e31ง", "บ\u0e48อทอง", "บางละม\u0e38ง",
			"บ\u0e49านบ\u0e36ง", "พานทอง", "พน\u0e31สน\u0e34คม", "ศร\u0e35ราชา", "ส\u0e31ตห\u0e35บ", "หนองใหญ\u0e48", "เกาะจ\u0e31นทร\u0e4c", "เม\u0e37องช\u0e31ยนาท", "มโนรมย\u0e4c", "ว\u0e31ดส\u0e34งห\u0e4c",
			"สรรคบ\u0e38ร\u0e35", "สรรพยา", "ห\u0e31นคา", "หนองมะโมง", "เน\u0e34นขาม", "เม\u0e37องช\u0e31ยภ\u0e39ม\u0e34", "เกษตรสมบ\u0e39รณ\u0e4c", "แก\u0e49งคร\u0e49อ", "คอนสวรรค\u0e4c", "คอนสาร",
			"จ\u0e31ต\u0e38ร\u0e31ส", "เทพสถ\u0e34ต", "เน\u0e34นสง\u0e48า", "บ\u0e49านเขว\u0e49า", "บ\u0e49านแท\u0e48น", "บำเหน\u0e47จณรงค\u0e4c", "ภ\u0e39เข\u0e35ยว", "ภ\u0e31กด\u0e35ช\u0e38มพล", "หนองบ\u0e31วแดง", "หนองบ\u0e31วระเหว",
			"ซ\u0e31บใหญ\u0e48", "เม\u0e37องช\u0e38มพร", "ท\u0e48าแซะ", "ท\u0e38\u0e48งตะโก", "ปะท\u0e34ว", "พะโต\u0e4aะ", "ละแม", "สว\u0e35", "หล\u0e31งสวน", "เม\u0e37องเช\u0e35ยงราย",
			"ข\u0e38นตาล", "เช\u0e35ยงของ", "เช\u0e35ยงแสน", "เท\u0e34ง", "ป\u0e48าแดด", "พาน", "แม\u0e48จ\u0e31น", "แม\u0e48ฟ\u0e49าหลวง", "แม\u0e48สรวย", "แม\u0e48สาย",
			"เว\u0e35ยงแก\u0e48น", "เว\u0e35ยงช\u0e31ย", "เว\u0e35ยงป\u0e48าเป\u0e49า", "พญาเม\u0e47งราย", "แม\u0e48ลาว", "ดอยหลวง", "เว\u0e35ยงเช\u0e35ยงร\u0e38\u0e49ง", "เม\u0e37องเช\u0e35ยงใหม\u0e48", "จอมทอง", "เช\u0e35ยงดาว",
			"ไชยปราการ", "ดอยเต\u0e48า", "ดอยหล\u0e48อ", "ดอยสะเก\u0e47ด", "ฝาง", "พร\u0e49าว", "แม\u0e48แจ\u0e48ม", "แม\u0e48แตง", "แม\u0e48ร\u0e34ม", "แม\u0e48วาง",
			"แม\u0e48อาย", "แม\u0e48ออน", "เว\u0e35ยงแหง", "สะเม\u0e34ง", "ส\u0e31นกำแพง", "ส\u0e31นทราย", "ส\u0e31นป\u0e48าตอง", "สารภ\u0e35", "หางดง", "อมก\u0e4bอย",
			"ฮอด", "เม\u0e37องตร\u0e31ง", "ก\u0e31นต\u0e31ง", "ปะเหล\u0e35ยน", "ย\u0e48านตาขาว", "ร\u0e31ษฎา", "ส\u0e34เกา", "ห\u0e49วยยอด", "ว\u0e31งว\u0e34เศษ", "หาดสำราญ",
			"นาโยง", "เม\u0e37องตราด", "เกาะช\u0e49าง", "เขาสม\u0e34ง", "คลองใหญ\u0e48", "บ\u0e48อไร\u0e48", "แหลมงอบ", "เกาะก\u0e39ด", "เม\u0e37องตาก", "ท\u0e48าสองยาง",
			"บ\u0e49านตาก", "พบพระ", "แม\u0e48ระมาด", "แม\u0e48สอด", "สามเงา", "อ\u0e38\u0e49มผาง", "ว\u0e31งเจ\u0e49า", "เม\u0e37องนครนายก", "บ\u0e49านนา", "ปากพล\u0e35",
			"องคร\u0e31กษ\u0e4c", "เม\u0e37องนครปฐม", "กำแพงแสน", "ดอนต\u0e39ม", "นครช\u0e31ยศร\u0e35", "บางเลน", "พ\u0e38ทธมณฑล", "สามพราน", "เม\u0e37องนครพนม", "ท\u0e48าอ\u0e38เทน",
			"ธาต\u0e38พนม", "นาแก", "นาหว\u0e49า", "บ\u0e49านแพง", "ปลาปาก", "โพนสวรรค\u0e4c", "เรณ\u0e39นคร", "ศร\u0e35สงคราม", "ว\u0e31งยาง", "นาทม",
			"เม\u0e37องนครราชส\u0e35มา", "แก\u0e49งสนามนาง", "ขามทะเลสอ", "ขามสะแกแสง", "คง", "ครบ\u0e38ร\u0e35", "จ\u0e31กราช", "ช\u0e38มพวง", "โชคช\u0e31ย", "ด\u0e48านข\u0e38นทด",
			"โนนแดง", "โนนไทย", "โนนส\u0e39ง", "บ\u0e31วใหญ\u0e48", "บ\u0e49านเหล\u0e37\u0e48อม", "ประทาย", "ป\u0e31กธงช\u0e31ย", "ปากช\u0e48อง", "พ\u0e34มาย", "ว\u0e31งน\u0e49ำเข\u0e35ยว",
			"ส\u0e35ค\u0e34\u0e49ว", "ส\u0e39งเน\u0e34น", "เส\u0e34งสาง", "ห\u0e49วยแถลง", "หนองบ\u0e38นนาก", "เทพาร\u0e31กษ\u0e4c", "เม\u0e37องยาง", "พระทองคำ", "ลำทะเมนช\u0e31ย", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34",
			"ส\u0e35ดา", "บ\u0e31วลาย", "เม\u0e37องนครศร\u0e35ธรรมราช", "ขนอม", "ฉวาง", "ชะอวด", "เช\u0e35ยรใหญ\u0e48", "ท\u0e48าศาลา", "ท\u0e38\u0e48งใหญ\u0e48", "ท\u0e38\u0e48งสง",
			"พระพรหม", "นาบอน", "บางข\u0e31น", "ปากพน\u0e31ง", "พรหมค\u0e35ร\u0e35", "พ\u0e34ป\u0e39น", "ร\u0e48อนพ\u0e34บ\u0e39ลย\u0e4c", "ลานสะกา", "ส\u0e34ชล", "ห\u0e31วไทร",
			"จ\u0e38ฬาภรณ\u0e4c", "นบพ\u0e34ตำ", "ช\u0e49างกลาง", "ถ\u0e49ำพรรณรา", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "เม\u0e37องนครสวรรค\u0e4c", "เก\u0e49าเล\u0e35\u0e49ยว", "โกรกพระ", "ช\u0e38มแสง", "ตากฟ\u0e49า",
			"ตาคล\u0e35", "ท\u0e48าตะโก", "บรรพตพ\u0e34ส\u0e31ย", "พย\u0e38หค\u0e35ร\u0e35", "ไพศาล\u0e35", "แม\u0e48วงก\u0e4c", "ลาดยาว", "หนองบ\u0e31ว", "แม\u0e48เป\u0e34น", "ช\u0e38มตาบง",
			"เม\u0e37องนนทบ\u0e38ร\u0e35", "ไทรน\u0e49อย", "บางกรวย", "บางบ\u0e31วทอง", "บางใหญ\u0e48", "ปากเกร\u0e47ด", "เม\u0e37องนราธ\u0e34วาส", "จะแนะ", "ตากใบ", "บาเจาะ",
			"ย\u0e35\u0e48งอ", "ระแงะ", "ร\u0e37อเสาะ", "แว\u0e49ง", "ศร\u0e35สาคร", "ส\u0e38ค\u0e34ร\u0e34น", "ส\u0e38ไหงโกลก", "ส\u0e38ไหงปาด\u0e35", "เจาะไอร\u0e49อง", "เม\u0e37องน\u0e48าน",
			"เช\u0e35ยงกลาง", "ท\u0e48าว\u0e31งผา", "ท\u0e38\u0e48งช\u0e49าง", "นาน\u0e49อย", "นาหม\u0e37\u0e48น", "บ\u0e49านหลวง", "ป\u0e31ว", "แม\u0e48จร\u0e34ม", "เว\u0e35ยงสา", "ส\u0e31นต\u0e34ส\u0e38ข",
			"บ\u0e48อเกล\u0e37อ", "สองแคว", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "ภ\u0e39เพ\u0e35ยง", "เม\u0e37องบ\u0e38ร\u0e35ร\u0e31มย\u0e4c", "กระส\u0e31ง", "ค\u0e39เม\u0e37อง", "ชำน\u0e34", "นาโพธ\u0e34\u0e4c", "นางรอง",
			"โนนด\u0e34นแดง", "โนนส\u0e38วรรณ", "บ\u0e49านกรวด", "พล\u0e31บพลาช\u0e31ย", "บ\u0e49านใหม\u0e48ไชยพจน\u0e4c", "ประโคนช\u0e31ย", "ปะคำ", "พ\u0e38ทไธสง", "ละหานทราย", "ลำปลายมาศ",
			"สต\u0e36ก", "หนองก\u0e35\u0e48", "หนองหงส\u0e4c", "ห\u0e49วยราช", "บ\u0e49านด\u0e48าน", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "แคนดง", "เม\u0e37องปท\u0e38มธาน\u0e35", "คลองหลวง", "ธ\u0e31ญบ\u0e38ร\u0e35",
			"ลาดหล\u0e38มแก\u0e49ว", "ลำล\u0e39กกา", "สามโคก", "หนองเส\u0e37อ", "เม\u0e37องประจวบค\u0e35ร\u0e35ข\u0e31นธ\u0e4c", "ก\u0e38ยบ\u0e38ร\u0e35", "ท\u0e31บสะแก", "บางสะพาน", "บางสะพานน\u0e49อย", "ปราณบ\u0e38ร\u0e35",
			"ห\u0e31วห\u0e34น", "สามร\u0e49อยยอด", "เม\u0e37องปราจ\u0e35นบ\u0e38ร\u0e35", "กบ\u0e34นทร\u0e4cบ\u0e38ร\u0e35", "ศร\u0e35มโหสถ", "นาด\u0e35", "บ\u0e49านสร\u0e49าง", "ประจ\u0e31นตคาม", "ศร\u0e35มหาโพธ\u0e34", "เม\u0e37องป\u0e31ตตาน\u0e35",
			"กะพ\u0e49อ", "โคกโพธ\u0e34\u0e4c", "ท\u0e38\u0e48งยางแดง", "ปะนาเระ", "มายอ", "ไม\u0e49แก\u0e48น", "ยะร\u0e31ง", "ยะหร\u0e34\u0e48ง", "สายบ\u0e38ร\u0e35", "หนองจ\u0e34ก",
			"แม\u0e48ลาน", "พระนครศร\u0e35อย\u0e38ธยา", "ท\u0e48าเร\u0e37อ", "นครหลวง", "บางซ\u0e49าย", "บางไทร", "บางบาล", "บางปะห\u0e31น", "บางปะอ\u0e34น", "บ\u0e49านแพรก",
			"ผ\u0e31กไห\u0e48", "ภาช\u0e35", "มหาราช", "ลาดบ\u0e31วหลวง", "ว\u0e31งน\u0e49อย", "เสนา", "อ\u0e38ท\u0e31ย", "เม\u0e37องพะเยา", "จ\u0e38น", "เช\u0e35ยงคำ",
			"เช\u0e35ยงม\u0e48วน", "ดอกคำใต\u0e49", "ปง", "แม\u0e48ใจ", "ภ\u0e39ซาง", "ภ\u0e39กามยาว", "เม\u0e37องพ\u0e31งงา", "กะปง", "เกาะยาว", "ค\u0e38ระบ\u0e38ร\u0e35",
			"ตะก\u0e31\u0e48วท\u0e38\u0e48ง", "ตะก\u0e31\u0e48วป\u0e48า", "ท\u0e31บป\u0e38ด", "ท\u0e49ายเหม\u0e37อง", "เม\u0e37องพ\u0e31ทล\u0e38ง", "กงหรา", "เขาช\u0e31ยสน", "ควนขน\u0e38น", "ตะโหมด", "ปากพะย\u0e39น",
			"ป\u0e48าบอน", "ป\u0e48าพะยอม", "ศร\u0e35บรรพต", "บางแก\u0e49ว", "ศร\u0e35นคร\u0e34นทร\u0e4c", "เม\u0e37องพ\u0e34จ\u0e34ตร", "ตะพานห\u0e34น", "ท\u0e31บคล\u0e49อ", "บางม\u0e39ลนาก", "โพทะเล",
			"โพธ\u0e34\u0e4cประท\u0e31บช\u0e49าง", "สามง\u0e48าม", "ว\u0e31งทรายพ\u0e39น", "สากเหล\u0e47ก", "บ\u0e36งนาราง", "ดงเจร\u0e34ญ", "วช\u0e34รบารม\u0e35", "เม\u0e37องพ\u0e34ษณ\u0e38โลก", "นครไทย", "ชาต\u0e34ตระการ",
			"เน\u0e34นมะปราง", "บางกระท\u0e38\u0e48ม", "บางระกำ", "พรหมพ\u0e34ราม", "ว\u0e31งทอง", "ว\u0e31ดโบสถ\u0e4c", "เม\u0e37องเพชรบ\u0e38ร\u0e35", "แก\u0e48งกระจาน", "เขาย\u0e49อย", "ชะอำ",
			"ท\u0e48ายาง", "บ\u0e49านลาด", "บ\u0e49านแหลม", "หนองหญ\u0e49าปล\u0e49อง", "เม\u0e37องเพชรบ\u0e39รณ\u0e4c", "เขาค\u0e49อ", "ชนแดน", "น\u0e49ำหนาว", "บ\u0e36งสามพ\u0e31น", "ว\u0e34เช\u0e35ยรบ\u0e38ร\u0e35",
			"ศร\u0e35เทพ", "หนองไผ\u0e48", "หล\u0e48มเก\u0e48า", "หล\u0e48มส\u0e31ก", "ว\u0e31งโป\u0e48ง", "เม\u0e37องแพร\u0e48", "เด\u0e48นช\u0e31ย", "ร\u0e49องกวาง", "ลอง", "ว\u0e31งช\u0e34\u0e49น",
			"สอง", "หนองม\u0e48วงไข\u0e48", "ส\u0e39งเม\u0e48น", "เม\u0e37องภ\u0e39เก\u0e47ต", "กะท\u0e39\u0e49", "ถลาง", "เม\u0e37องมหาสารคาม", "ก\u0e31นทรว\u0e34ช\u0e31ย", "แกดำ", "โกส\u0e38มพ\u0e34ส\u0e31ย",
			"เช\u0e35ยงย\u0e37น", "นาเช\u0e37อก", "นาด\u0e39น", "บรบ\u0e37อ", "พย\u0e31คฆภ\u0e39ม\u0e34พ\u0e34ส\u0e31ย", "วาป\u0e35ปท\u0e38ม", "ก\u0e38ดร\u0e31ง", "ยางส\u0e35ส\u0e38ราช", "ช\u0e37\u0e48นชม", "เม\u0e37องม\u0e38กดาหาร",
			"คำชะอ\u0e35", "ดงหลวง", "ดอนตาล", "น\u0e34คมคำสร\u0e49อย", "หนองส\u0e39ง", "หว\u0e49านใหญ\u0e48", "เม\u0e37องแม\u0e48ฮ\u0e48องสอน", "ข\u0e38นยวม", "ปางมะผ\u0e49า", "ปาย",
			"แม\u0e48ลาน\u0e49อย", "แม\u0e48สะเร\u0e35ยง", "สบเมย", "เม\u0e37องยโสธร", "ก\u0e38ดช\u0e38ม", "ค\u0e49อว\u0e31ง", "คำเข\u0e37\u0e48อนแก\u0e49ว", "ไทยเจร\u0e34ญ", "ทรายม\u0e39ล", "ป\u0e48าต\u0e34\u0e49ว",
			"มหาชนะช\u0e31ย", "เล\u0e34งนกทา", "เม\u0e37องยะลา", "กาบ\u0e31ง", "กรงป\u0e34น\u0e31ง", "ธารโต", "บ\u0e31นน\u0e31งสตา", "เบตง", "ยะหา", "ราม\u0e31น",
			"เม\u0e37องร\u0e49อยเอ\u0e47ด", "เกษตรว\u0e34ส\u0e31ย", "จต\u0e38รพ\u0e31กตร\u0e4cพ\u0e34มาน", "จ\u0e31งหาร", "ธว\u0e31ชบ\u0e38ร\u0e35", "ปท\u0e38มร\u0e31ตน\u0e4c", "พนมไพร", "โพธ\u0e34\u0e4cช\u0e31ย", "โพนทราย", "โพนทอง",
			"เมยวด\u0e35", "เม\u0e37องสรวง", "ศร\u0e35สมเด\u0e47จ", "เสลภ\u0e39ม\u0e34", "ส\u0e38วรรณภ\u0e39ม\u0e34", "หนองพอก", "อาจสามารถ", "เช\u0e35ยงขว\u0e31ญ", "หนองฮ\u0e35", "ท\u0e38\u0e48งเขาหลวง",
			"เม\u0e37องระนอง", "กระบ\u0e38ร\u0e35", "กะเปอร\u0e4c", "ละอ\u0e38\u0e48น", "ส\u0e38ขสำราญ", "เม\u0e37องระยอง", "แกลง", "บ\u0e49านค\u0e48าย", "บ\u0e49านฉาง", "ปลวกแดง",
			"ว\u0e31งจ\u0e31นทร\u0e4c", "เขาชะเมา", "น\u0e34คมพ\u0e31ฒนา", "เม\u0e37องราชบ\u0e38ร\u0e35", "จอมบ\u0e36ง", "ดำเน\u0e34นสะดวก", "บางแพ", "บ\u0e49านโป\u0e48ง", "ปากท\u0e48อ", "โพธาราม",
			"ว\u0e31ดเพลง", "สวนผ\u0e36\u0e49ง", "บ\u0e49านคา", "เม\u0e37องลพบ\u0e38ร\u0e35", "โคกเจร\u0e34ญ", "โคกสำโรง", "ช\u0e31ยบาดาล", "ท\u0e48าว\u0e38\u0e49ง", "ท\u0e48าหลวง", "บ\u0e49านหม\u0e35\u0e48",
			"พ\u0e31ฒนาน\u0e34คม", "ลำสนธ\u0e34", "สระโบถส\u0e4c", "หนองม\u0e48วง", "เม\u0e37องเลย", "เช\u0e35ยงคาน", "ด\u0e48านซ\u0e49าย", "ท\u0e48าล\u0e35\u0e48", "นาด\u0e49วง", "นาแห\u0e49ว",
			"ปากชม", "ผาขาว", "ภ\u0e39กระด\u0e36ง", "ภ\u0e39เร\u0e37อ", "ภ\u0e39หลวง", "ว\u0e31งสะพ\u0e38ง", "เอราว\u0e31ณ", "หนองห\u0e34น", "เม\u0e37องลำปาง", "เกาะคา",
			"งาว", "แจ\u0e49ห\u0e48ม", "เถ\u0e34น", "แม\u0e48ทะ", "แม\u0e48พร\u0e34ก", "เม\u0e37องปาน", "แม\u0e48เมาะ", "ว\u0e31งเหน\u0e37อ", "สบปราบ", "เสร\u0e34มงาม",
			"ห\u0e49างฉ\u0e31ตร", "เม\u0e37องลำพ\u0e39น", "ท\u0e38\u0e48งห\u0e31วช\u0e49าง", "บ\u0e49านโฮ\u0e48ง", "ป\u0e48าซาง", "แม\u0e48ทา", "ล\u0e35\u0e49", "บ\u0e49านธ\u0e34", "เว\u0e35ยงหนองล\u0e48อง", "เม\u0e37องศ\u0e35รสะเกษ",
			"ก\u0e31นทรล\u0e31กษ\u0e4c", "ก\u0e31นทรารมย\u0e4c", "ข\u0e38ข\u0e31นธ\u0e4c", "ข\u0e38นหาญ", "น\u0e49ำเกล\u0e35\u0e49ยง", "โนนค\u0e39ณ", "บ\u0e36งบ\u0e39รพ\u0e4c", "เบญจล\u0e31กษณ\u0e4c", "ปรางค\u0e4cก\u0e39\u0e48", "พย\u0e38ห\u0e4c",
			"ไพรบ\u0e36ง", "โพธ\u0e34\u0e4cศร\u0e35ส\u0e38วรรณ", "ภ\u0e39ส\u0e34งห\u0e4c", "เม\u0e37องจ\u0e31นทร\u0e4c", "ยางช\u0e38มน\u0e49อย", "ราษ\u0e35ไศล", "ว\u0e31งห\u0e34น", "ศร\u0e35ร\u0e31ตนะ", "ห\u0e49วยท\u0e31บท\u0e31น", "อ\u0e38ท\u0e38มพรพ\u0e34ส\u0e31ย",
			"ศ\u0e34ลาลาด", "เม\u0e37องสกลนคร", "ก\u0e38ดบาก", "ก\u0e38ส\u0e38มาลย\u0e4c", "คำตากล\u0e49า", "เจร\u0e34ญศ\u0e34ลป\u0e4c", "เต\u0e48างอย", "น\u0e34คมน\u0e49ำอ\u0e39น", "บ\u0e49านม\u0e48วง", "พรรณาน\u0e34คม",
			"พ\u0e31งโคน", "วานรน\u0e34วาส", "วาร\u0e34ชภ\u0e39ม\u0e34", "โคกศร\u0e35ส\u0e38พรรณ", "สว\u0e48างแดนด\u0e34น", "ส\u0e48องดาว", "อากาศอำนวย", "ภ\u0e39พาน", "โพนนาแก\u0e49ว", "เม\u0e37องสงขลา",
			"กระแสส\u0e34นธ\u0e38\u0e4c", "ควนเน\u0e35ยง", "จะนะ", "เทพา", "นาทว\u0e35", "นาหม\u0e48อม", "บางกล\u0e48ำ", "ระโนด", "ร\u0e31ตภ\u0e39ม\u0e34", "สท\u0e34งพระ",
			"สะเดา", "สะบ\u0e49าย\u0e49อย", "ส\u0e34งหนคร", "หาดใหญ\u0e48", "คลองหอยโข\u0e48ง", "เม\u0e37องสต\u0e39ล", "ควนกาหลง", "ควนโดน", "ท\u0e48าแพ", "ท\u0e38\u0e48งหว\u0e49า",
			"ละง\u0e39", "มะน\u0e31ง", "เม\u0e37องสม\u0e38ทรปราการ", "บางบ\u0e48อ", "บางพล\u0e35", "พระประแดง", "พระสม\u0e38ทรเจด\u0e35ย\u0e4c", "บางเสาธง", "เม\u0e37องสม\u0e38ทรสงคราม", "บางคณท\u0e35",
			"อ\u0e31มพวา", "เม\u0e37องสม\u0e38ทรสาคร", "กระท\u0e38\u0e48มแบน", "บ\u0e49านแพ\u0e49ว", "เม\u0e37องสระแก\u0e49ว", "เขาฉกรรจ\u0e4c", "คลองหาด", "ตาพระยา", "ว\u0e31งน\u0e49ำเย\u0e47น", "ว\u0e31ฒนานคร",
			"อร\u0e31ญประเทศ", "โคกส\u0e39ง", "ว\u0e31งสมบ\u0e39รณ\u0e4c", "เม\u0e37องสระบ\u0e38ร\u0e35", "แก\u0e48งคอย", "ดอนพ\u0e38ด", "บ\u0e49านหมอ", "พระพ\u0e38ทธบาท", "มวกเหล\u0e47ก", "ว\u0e34หารแดง",
			"เสาไห\u0e49", "หนองแค", "หนองแซง", "หนองโดน", "ว\u0e31งม\u0e48วง", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "เม\u0e37องส\u0e34งห\u0e4cบ\u0e38ร\u0e35", "ค\u0e48ายบางระจ\u0e31น", "ท\u0e48าช\u0e49าง", "บางระจ\u0e31น",
			"พรหมบ\u0e38ร\u0e35", "อ\u0e34นทร\u0e4cบ\u0e38ร\u0e35", "เม\u0e37องส\u0e38โขท\u0e31ย", "กงไกรลาศ", "ค\u0e35ร\u0e35มาศ", "ท\u0e38\u0e48งเสล\u0e35\u0e48ยม", "บ\u0e49านด\u0e48านลานหอย", "ศร\u0e35นคร", "ศร\u0e35ส\u0e31ชนาล\u0e31ย", "ศร\u0e35สำโรง",
			"สวรรคโลก", "เม\u0e37องส\u0e38พรรณบ\u0e38ร\u0e35", "ดอนเจด\u0e35ย\u0e4c", "ด\u0e48านช\u0e49าง", "เด\u0e34มบางนางบวช", "บางปลาม\u0e49า", "ศร\u0e35ประจ\u0e31นต\u0e4c", "สองพ\u0e35\u0e48น\u0e49อง", "สามช\u0e38ก", "อ\u0e39\u0e48ทอง",
			"หนองหญ\u0e49าไซ", "เม\u0e37องส\u0e38ราษฎร\u0e4cธาน\u0e35", "กาญจนด\u0e34ษฐ\u0e4c", "เกาะพะง\u0e31น", "เกาะสม\u0e38ย", "ค\u0e35ร\u0e35ร\u0e31ฐน\u0e34คม", "เค\u0e35ยนซา", "ช\u0e31ยบ\u0e38ร\u0e35", "ไชยา", "ดอนส\u0e31ก",
			"ท\u0e48าฉาง", "ท\u0e48าชนะ", "บ\u0e49านตาข\u0e38น", "บ\u0e49านนาเด\u0e34ม", "บ\u0e49านนาสาร", "พนม", "พระแสง", "พ\u0e38นพ\u0e34น", "ว\u0e34ภาวด\u0e35", "เว\u0e35ยงสระ",
			"เม\u0e37องส\u0e38ร\u0e34นทร\u0e4c", "กาบเช\u0e34ง", "จอมพระ", "ช\u0e38มพลบ\u0e38ร\u0e35", "ท\u0e48าต\u0e39ม", "บ\u0e31วเชด", "ปราสาท", "ร\u0e31ตนบ\u0e38ร\u0e35", "ลำดวน", "ศ\u0e35ขรภ\u0e39ม\u0e34",
			"สนม", "ส\u0e31งขะ", "สำโรงทาบ", "ศร\u0e35ณรงค\u0e4c", "พนมดงร\u0e31ก", "เขวาส\u0e34นร\u0e34นทร\u0e4c", "โนนนารายณ\u0e4c", "เม\u0e37องหนองคาย", "เซกา", "โซ\u0e48พ\u0e34ส\u0e31ย",
			"ท\u0e48าบ\u0e48อ", "บ\u0e36งกาฬ", "บ\u0e36งโขลงหลง", "ปากคาด", "พรเจร\u0e34ญ", "โพนพ\u0e34ส\u0e31ย", "ศร\u0e35เช\u0e35ยงใหม\u0e48", "ศร\u0e35ว\u0e34ไล", "ส\u0e31งคม", "สระใคร\u0e48",
			"บ\u0e38\u0e48งคล\u0e49า", "ร\u0e31ตนวาป\u0e35", "เฝ\u0e49าไร\u0e48", "โพธ\u0e34\u0e4cตาก", "เม\u0e37องหนองบ\u0e31วลำภ\u0e39", "นากลาง", "โนนส\u0e31ง", "ศร\u0e35บ\u0e38ญเร\u0e37อง", "ส\u0e38วรรณค\u0e39หา", "นาว\u0e31ง",
			"เม\u0e37องอ\u0e48างทอง", "ไชโย", "ป\u0e48าโมก", "โพธ\u0e34\u0e4cทอง", "ว\u0e34เศษช\u0e31ยชาญ", "สามโก\u0e49", "แสวงหา", "เม\u0e37องอำนาจเจร\u0e34ญ", "ชาน\u0e38มาน", "ปท\u0e38มราชวงศา",
			"พนา", "เสนางคน\u0e34คม", "ห\u0e31วตะพาน", "ล\u0e37ออำนาจ", "เม\u0e37องอ\u0e38ดรธาน\u0e35", "ก\u0e39\u0e48แก\u0e49ว", "ก\u0e38ดจ\u0e31บ", "ก\u0e38มภวาป\u0e35", "ไชยวาน", "ท\u0e38\u0e48งฝน",
			"นาย\u0e39ง", "น\u0e49ำโสม", "โนนสะอาด", "บ\u0e49านด\u0e38ง", "บ\u0e49านผ\u0e37อ", "พ\u0e34บ\u0e39ลย\u0e4cร\u0e31กษ\u0e4c", "เพ\u0e47ญ", "ว\u0e31งสามหมอ", "ศร\u0e35ธาต\u0e38", "สร\u0e49างคอม",
			"หนองว\u0e31วซอ", "หนองหาน", "หนองแสง", "ประจ\u0e31กษ\u0e4cศ\u0e34ลปาคาม", "เม\u0e37องอ\u0e38ตรด\u0e34ตถ\u0e4c", "ตรอน", "ทองแสนข\u0e31น", "ท\u0e48าปลา", "น\u0e49ำปาด", "บ\u0e49านโคก",
			"พ\u0e34ช\u0e31ย", "ฟากท\u0e48า", "ล\u0e31บแล", "เม\u0e37องอ\u0e38ท\u0e31ยธาน\u0e35", "ท\u0e31พท\u0e31น", "บ\u0e49านไร\u0e48", "ลานส\u0e31ก", "สว\u0e48างอารมณ\u0e4c", "หนองขาหย\u0e48าง", "หนองฉาง",
			"ห\u0e49วยคต", "เม\u0e37องอ\u0e38บลราชธาน\u0e35", "ก\u0e38ดข\u0e49าวป\u0e38\u0e49น", "เขมราฐ", "เข\u0e37\u0e48องใน", "โขงเจ\u0e35ยม", "เดชอ\u0e38ดม", "ตระการพ\u0e37ชผล", "ตาลส\u0e38ม", "ท\u0e38\u0e48งศร\u0e35อ\u0e38ดม",
			"นาจะหลวย", "น\u0e49ำย\u0e37น", "บ\u0e38ณฑร\u0e34ก", "พ\u0e34บ\u0e39ลม\u0e31งสาหาร", "โพธ\u0e34\u0e4cไทร", "ม\u0e48วงสามส\u0e34บ", "เหล\u0e48าเส\u0e37อโก\u0e49ก", "วาร\u0e34นชำราบ", "ศร\u0e35เม\u0e37องใหม\u0e48", "สำโรง",
			"ส\u0e34ร\u0e34นธร", "นาเย\u0e35ย", "นาตาล", "สว\u0e48างว\u0e35ระวงศ\u0e4c", "น\u0e49ำข\u0e38\u0e48น", "ดอนมดแดง"
		});
		this.Tw_ampore.AutoCompleteMode = System.Windows.Forms.AutoCompleteMode.SuggestAppend;
		this.Tw_ampore.AutoCompleteSource = System.Windows.Forms.AutoCompleteSource.CustomSource;
		System.Windows.Forms.TextBox tw_ampore = this.Tw_ampore;
		location = new System.Drawing.Point(869, 28);
		tw_ampore.Location = location;
		System.Windows.Forms.TextBox tw_ampore2 = this.Tw_ampore;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_ampore2.Margin = margin;
		this.Tw_ampore.Name = "Tw_ampore";
		System.Windows.Forms.TextBox tw_ampore3 = this.Tw_ampore;
		size = new System.Drawing.Size(103, 23);
		tw_ampore3.Size = size;
		this.Tw_ampore.TabIndex = 6;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label2;
		location = new System.Drawing.Point(804, 32);
		label23.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label24 = this.Label2;
		size = new System.Drawing.Size(67, 16);
		label24.Size = size;
		this.Label2.TabIndex = 47;
		this.Label2.Text = "เขต/อำเภอ";
		System.Windows.Forms.TextBox tw_tambon = this.Tw_tambon;
		location = new System.Drawing.Point(704, 28);
		tw_tambon.Location = location;
		System.Windows.Forms.TextBox tw_tambon2 = this.Tw_tambon;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_tambon2.Margin = margin;
		this.Tw_tambon.Name = "Tw_tambon";
		System.Windows.Forms.TextBox tw_tambon3 = this.Tw_tambon;
		size = new System.Drawing.Size(100, 23);
		tw_tambon3.Size = size;
		this.Tw_tambon.TabIndex = 5;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label3;
		location = new System.Drawing.Point(633, 32);
		label25.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label26 = this.Label3;
		size = new System.Drawing.Size(71, 16);
		label26.Size = size;
		this.Label3.TabIndex = 47;
		this.Label3.Text = "แขวง/ตำบล";
		this.Tw_tel.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tw_tel = this.Tw_tel;
		location = new System.Drawing.Point(422, 53);
		tw_tel.Location = location;
		System.Windows.Forms.ComboBox tw_tel2 = this.Tw_tel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_tel2.Margin = margin;
		this.Tw_tel.Name = "Tw_tel";
		System.Windows.Forms.ComboBox tw_tel3 = this.Tw_tel;
		size = new System.Drawing.Size(170, 24);
		tw_tel3.Size = size;
		this.Tw_tel.TabIndex = 9;
		System.Windows.Forms.TextBox tw_code = this.Tw_code;
		location = new System.Drawing.Point(290, 54);
		tw_code.Location = location;
		System.Windows.Forms.TextBox tw_code2 = this.Tw_code;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_code2.Margin = margin;
		this.Tw_code.Name = "Tw_code";
		System.Windows.Forms.TextBox tw_code3 = this.Tw_code;
		size = new System.Drawing.Size(94, 23);
		tw_code3.Size = size;
		this.Tw_code.TabIndex = 8;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label4;
		location = new System.Drawing.Point(211, 58);
		label27.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label28 = this.Label4;
		size = new System.Drawing.Size(80, 16);
		label28.Size = size;
		this.Label4.TabIndex = 47;
		this.Label4.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		System.Windows.Forms.TextBox tw_road = this.Tw_road;
		location = new System.Drawing.Point(548, 28);
		tw_road.Location = location;
		System.Windows.Forms.TextBox tw_road2 = this.Tw_road;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_road2.Margin = margin;
		this.Tw_road.Name = "Tw_road";
		System.Windows.Forms.TextBox tw_road3 = this.Tw_road;
		size = new System.Drawing.Size(83, 23);
		tw_road3.Size = size;
		this.Tw_road.TabIndex = 4;
		this.Tw_privince.AutoCompleteCustomSource.AddRange(new string[77]
		{
			"กร\u0e38งเทพมหานคร", "กระบ\u0e35\u0e48", "กาญจนบ\u0e38ร\u0e35", "กาฬส\u0e34นธ\u0e38\u0e4c", "กำแพงเพชร", "ขอนแก\u0e48น", "จ\u0e31นทบ\u0e38ร\u0e35", "ฉะเช\u0e34งเทรา", "ชลบ\u0e38ร\u0e35", "ช\u0e31ยนาท",
			"ช\u0e31ยภ\u0e39ม\u0e34", "ช\u0e38มพร", "เช\u0e35ยงราย", "เช\u0e35ยงใหม\u0e48", "ตร\u0e31ง", "ตราด", "ตาก", "นครนายก", "นครปฐม", "นครพนม",
			"นครราชส\u0e35มา", "นครศร\u0e35ธรรมราช", "นครสวรรค\u0e4c", "นนทบ\u0e38ร\u0e35", "นราธ\u0e34วาส", "น\u0e48าน", "บ\u0e36งกาฬ", "บ\u0e38ร\u0e35ร\u0e31มย\u0e4c", "ปท\u0e38มธาน\u0e35", "ประจวบค\u0e35ร\u0e35ข\u0e31นธ\u0e4c",
			"ปราจ\u0e35นบ\u0e38ร\u0e35", "ป\u0e31ตตาน\u0e35", "พระนครศร\u0e35อย\u0e38ธยา", "พะเยา", "พ\u0e31งงา", "พ\u0e31ทล\u0e38ง", "พ\u0e34จ\u0e34ตร", "พ\u0e34ษณ\u0e38โลก", "เพชรบ\u0e38ร\u0e35", "เพชรบ\u0e39รณ\u0e4c",
			"แพร\u0e48", "ภ\u0e39เก\u0e47ต", "มหาสารคาม", "ม\u0e38กดาหาร", "แม\u0e48ฮ\u0e48องสอน", "ยโสธร", "ยะลา", "ร\u0e49อยเอ\u0e47ด", "ระนอง", "ระยอง",
			"ราชบ\u0e38ร\u0e35", "ลพบ\u0e38ร\u0e35", "ลำปาง", "ลำพ\u0e39น", "เลย", "ศร\u0e35สะเกษ", "สกลนคร", "สงขลา", "สต\u0e39ล", "สม\u0e38ทรปราการ",
			"สม\u0e38ทรสงคราม", "สม\u0e38ทรสาคร", "สระแก\u0e49ว", "สระบ\u0e38ร\u0e35", "ส\u0e34งห\u0e4cบ\u0e38ร\u0e35", "ส\u0e38โขท\u0e31ย", "ส\u0e38พรรณบ\u0e38ร\u0e35", "ส\u0e38ราษฎร\u0e4cธาน\u0e35", "ส\u0e38ร\u0e34นทร\u0e4c", "หนองคาย",
			"หนองบ\u0e31วลำภ\u0e39", "อ\u0e48างทอง", "อำนาจเจร\u0e34ญ", "อ\u0e38ดรธาน\u0e35", "อ\u0e38ตรด\u0e34ตถ\u0e4c", "อ\u0e38ท\u0e31ยธาน\u0e35", "อ\u0e38บลราชธาน\u0e35"
		});
		this.Tw_privince.AutoCompleteMode = System.Windows.Forms.AutoCompleteMode.SuggestAppend;
		this.Tw_privince.AutoCompleteSource = System.Windows.Forms.AutoCompleteSource.CustomSource;
		System.Windows.Forms.TextBox tw_privince = this.Tw_privince;
		location = new System.Drawing.Point(80, 53);
		tw_privince.Location = location;
		System.Windows.Forms.TextBox tw_privince2 = this.Tw_privince;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_privince2.Margin = margin;
		this.Tw_privince.Name = "Tw_privince";
		System.Windows.Forms.TextBox tw_privince3 = this.Tw_privince;
		size = new System.Drawing.Size(125, 23);
		tw_privince3.Size = size;
		this.Tw_privince.TabIndex = 7;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label5;
		location = new System.Drawing.Point(518, 32);
		label29.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label30 = this.Label5;
		size = new System.Drawing.Size(32, 16);
		label30.Size = size;
		this.Label5.TabIndex = 47;
		this.Label5.Text = "ถนน";
		this.Label42.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label42;
		location = new System.Drawing.Point(598, 56);
		label31.Location = location;
		this.Label42.Name = "Label42";
		System.Windows.Forms.Label label32 = this.Label42;
		size = new System.Drawing.Size(28, 16);
		label32.Size = size;
		this.Label42.TabIndex = 47;
		this.Label42.Text = "Fax";
		this.Label43.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label43;
		location = new System.Drawing.Point(35, 59);
		label33.Location = location;
		this.Label43.Name = "Label43";
		System.Windows.Forms.Label label34 = this.Label43;
		size = new System.Drawing.Size(43, 16);
		label34.Size = size;
		this.Label43.TabIndex = 47;
		this.Label43.Text = "จ\u0e31งหว\u0e31ด";
		this.Label44.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label44;
		location = new System.Drawing.Point(391, 57);
		label35.Location = location;
		this.Label44.Name = "Label44";
		System.Windows.Forms.Label label36 = this.Label44;
		size = new System.Drawing.Size(29, 16);
		label36.Size = size;
		this.Label44.TabIndex = 47;
		this.Label44.Text = "โทร";
		this.Tw.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tw.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox tw = this.Tw;
		location = new System.Drawing.Point(80, 3);
		tw.Location = location;
		System.Windows.Forms.TextBox tw2 = this.Tw;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw2.Margin = margin;
		this.Tw.Name = "Tw";
		System.Windows.Forms.TextBox tw3 = this.Tw;
		size = new System.Drawing.Size(471, 23);
		tw3.Size = size;
		this.Tw.TabIndex = 0;
		this.Label48.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label48;
		location = new System.Drawing.Point(10, 6);
		label37.Location = location;
		this.Label48.Name = "Label48";
		System.Windows.Forms.Label label38 = this.Label48;
		size = new System.Drawing.Size(68, 16);
		label38.Size = size;
		this.Label48.TabIndex = 47;
		this.Label48.Text = "ช\u0e37\u0e48อท\u0e35\u0e48ทำงาน";
		System.Windows.Forms.TextBox tw_soi = this.Tw_soi;
		location = new System.Drawing.Point(422, 28);
		tw_soi.Location = location;
		System.Windows.Forms.TextBox tw_soi2 = this.Tw_soi;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_soi2.Margin = margin;
		this.Tw_soi.Name = "Tw_soi";
		System.Windows.Forms.TextBox tw_soi3 = this.Tw_soi;
		size = new System.Drawing.Size(96, 23);
		tw_soi3.Size = size;
		this.Tw_soi.TabIndex = 3;
		this.Label45.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label45;
		location = new System.Drawing.Point(392, 31);
		label39.Location = location;
		this.Label45.Name = "Label45";
		System.Windows.Forms.Label label40 = this.Label45;
		size = new System.Drawing.Size(33, 16);
		label40.Size = size;
		this.Label45.TabIndex = 2;
		this.Label45.Text = "ซอย";
		System.Windows.Forms.TextBox tw_moo = this.Tw_moo;
		location = new System.Drawing.Point(338, 28);
		tw_moo.Location = location;
		System.Windows.Forms.TextBox tw_moo2 = this.Tw_moo;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_moo2.Margin = margin;
		this.Tw_moo.Name = "Tw_moo";
		System.Windows.Forms.TextBox tw_moo3 = this.Tw_moo;
		size = new System.Drawing.Size(46, 23);
		tw_moo3.Size = size;
		this.Tw_moo.TabIndex = 2;
		this.Label46.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label46;
		location = new System.Drawing.Point(303, 31);
		label41.Location = location;
		this.Label46.Name = "Label46";
		System.Windows.Forms.Label label42 = this.Label46;
		size = new System.Drawing.Size(33, 16);
		label42.Size = size;
		this.Label46.TabIndex = 1;
		this.Label46.Text = "หม\u0e39\u0e48ท\u0e35\u0e48";
		System.Windows.Forms.TextBox tw_no = this.Tw_no;
		location = new System.Drawing.Point(80, 28);
		tw_no.Location = location;
		System.Windows.Forms.TextBox tw_no2 = this.Tw_no;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_no2.Margin = margin;
		this.Tw_no.Name = "Tw_no";
		System.Windows.Forms.TextBox tw_no3 = this.Tw_no;
		size = new System.Drawing.Size(217, 23);
		tw_no3.Size = size;
		this.Tw_no.TabIndex = 1;
		this.Label47.AutoSize = true;
		System.Windows.Forms.Label label43 = this.Label47;
		location = new System.Drawing.Point(42, 32);
		label43.Location = location;
		this.Label47.Name = "Label47";
		System.Windows.Forms.Label label44 = this.Label47;
		size = new System.Drawing.Size(37, 16);
		label44.Size = size;
		this.Label47.TabIndex = 47;
		this.Label47.Text = "เลขท\u0e35\u0e48";
		System.Windows.Forms.TextBox tw_fax = this.Tw_fax;
		location = new System.Drawing.Point(628, 53);
		tw_fax.Location = location;
		System.Windows.Forms.TextBox tw_fax2 = this.Tw_fax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_fax2.Margin = margin;
		this.Tw_fax.Name = "Tw_fax";
		System.Windows.Forms.TextBox tw_fax3 = this.Tw_fax;
		size = new System.Drawing.Size(130, 23);
		tw_fax3.Size = size;
		this.Tw_fax.TabIndex = 10;
		this.expandablePanel4.CanvasColor = System.Drawing.SystemColors.Control;
		this.expandablePanel4.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.expandablePanel4.Controls.Add(this.Panel1);
		this.expandablePanel4.Dock = System.Windows.Forms.DockStyle.Top;
		this.expandablePanel4.Expanded = false;
		DevComponents.DotNetBar.ExpandablePanel expandablePanel6 = this.expandablePanel4;
		expandedBounds = new System.Drawing.Rectangle(0, 0, 1247, 105);
		expandablePanel6.ExpandedBounds = expandedBounds;
		this.expandablePanel4.ExpandOnTitleClick = true;
		this.expandablePanel4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ExpandablePanel expandablePanel7 = this.expandablePanel4;
		location = new System.Drawing.Point(0, 0);
		expandablePanel7.Location = location;
		this.expandablePanel4.Name = "expandablePanel4";
		DevComponents.DotNetBar.ExpandablePanel expandablePanel8 = this.expandablePanel4;
		size = new System.Drawing.Size(1243, 26);
		expandablePanel8.Size = size;
		this.expandablePanel4.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.expandablePanel4.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarBackground;
		this.expandablePanel4.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarBackground2;
		this.expandablePanel4.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.expandablePanel4.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarDockedBorder;
		this.expandablePanel4.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.ItemText;
		this.expandablePanel4.Style.GradientAngle = 90;
		this.expandablePanel4.TabIndex = 3;
		this.expandablePanel4.TitleStyle.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.expandablePanel4.TitleStyle.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.expandablePanel4.TitleStyle.Border = DevComponents.DotNetBar.eBorderType.RaisedInner;
		this.expandablePanel4.TitleStyle.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.expandablePanel4.TitleStyle.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.expandablePanel4.TitleStyle.GradientAngle = 90;
		this.expandablePanel4.TitleStyle.MarginLeft = 12;
		this.expandablePanel4.TitleText = "ท\u0e35\u0e48อย\u0e39\u0e48ตามบ\u0e31ตรประจำต\u0e31ว";
		this.Panel1.BackColor = System.Drawing.Color.FromArgb(227, 239, 255);
		this.Panel1.Controls.Add(this.ButtonX3);
		this.Panel1.Controls.Add(this.Label10);
		this.Panel1.Controls.Add(this.Tc_fax);
		this.Panel1.Controls.Add(this.Tc_ampore);
		this.Panel1.Controls.Add(this.Label39);
		this.Panel1.Controls.Add(this.Tc_tambon);
		this.Panel1.Controls.Add(this.Label38);
		this.Panel1.Controls.Add(this.Tc_code);
		this.Panel1.Controls.Add(this.Label41);
		this.Panel1.Controls.Add(this.Tc_road);
		this.Panel1.Controls.Add(this.Tc_province);
		this.Panel1.Controls.Add(this.Label37);
		this.Panel1.Controls.Add(this.Label40);
		this.Panel1.Controls.Add(this.Tc_soi);
		this.Panel1.Controls.Add(this.Label36);
		this.Panel1.Controls.Add(this.Tc_moo);
		this.Panel1.Controls.Add(this.Label11);
		this.Panel1.Controls.Add(this.Tc_no);
		this.Panel1.Controls.Add(this.Label8);
		this.Panel1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.Panel1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		System.Windows.Forms.Panel panel7 = this.Panel1;
		location = new System.Drawing.Point(0, 26);
		panel7.Location = location;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel8 = this.Panel1;
		size = new System.Drawing.Size(1247, 0);
		panel8.Size = size;
		this.Panel1.TabIndex = 0;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Checked = true;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ButtonX3.ForeColor = System.Drawing.Color.DarkBlue;
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX3;
		location = new System.Drawing.Point(987, 13);
		buttonX13.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX3;
		size = new System.Drawing.Size(199, 54);
		buttonX14.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 56;
		this.ButtonX3.Text = "รายช\u0e37\u0e48อผ\u0e39\u0e49เข\u0e49าพ\u0e31ก";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label45 = this.Label10;
		location = new System.Drawing.Point(806, 49);
		label45.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label46 = this.Label10;
		size = new System.Drawing.Size(28, 16);
		label46.Size = size;
		this.Label10.TabIndex = 47;
		this.Label10.Text = "Fax";
		System.Windows.Forms.TextBox tc_fax = this.Tc_fax;
		location = new System.Drawing.Point(836, 46);
		tc_fax.Location = location;
		System.Windows.Forms.TextBox tc_fax2 = this.Tc_fax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_fax2.Margin = margin;
		this.Tc_fax.Name = "Tc_fax";
		System.Windows.Forms.TextBox tc_fax3 = this.Tc_fax;
		size = new System.Drawing.Size(130, 23);
		tc_fax3.Size = size;
		this.Tc_fax.TabIndex = 9;
		System.Windows.Forms.TextBox tc_ampore = this.Tc_ampore;
		location = new System.Drawing.Point(836, 14);
		tc_ampore.Location = location;
		System.Windows.Forms.TextBox tc_ampore2 = this.Tc_ampore;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_ampore2.Margin = margin;
		this.Tc_ampore.Name = "Tc_ampore";
		System.Windows.Forms.TextBox tc_ampore3 = this.Tc_ampore;
		size = new System.Drawing.Size(130, 23);
		tc_ampore3.Size = size;
		this.Tc_ampore.TabIndex = 5;
		this.Label39.AutoSize = true;
		System.Windows.Forms.Label label47 = this.Label39;
		location = new System.Drawing.Point(767, 17);
		label47.Location = location;
		this.Label39.Name = "Label39";
		System.Windows.Forms.Label label48 = this.Label39;
		size = new System.Drawing.Size(67, 16);
		label48.Size = size;
		this.Label39.TabIndex = 47;
		this.Label39.Text = "เขต/อำเภอ";
		System.Windows.Forms.TextBox tc_tambon = this.Tc_tambon;
		location = new System.Drawing.Point(622, 14);
		tc_tambon.Location = location;
		System.Windows.Forms.TextBox tc_tambon2 = this.Tc_tambon;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_tambon2.Margin = margin;
		this.Tc_tambon.Name = "Tc_tambon";
		System.Windows.Forms.TextBox tc_tambon3 = this.Tc_tambon;
		size = new System.Drawing.Size(130, 23);
		tc_tambon3.Size = size;
		this.Tc_tambon.TabIndex = 4;
		this.Label38.AutoSize = true;
		System.Windows.Forms.Label label49 = this.Label38;
		location = new System.Drawing.Point(549, 16);
		label49.Location = location;
		this.Label38.Name = "Label38";
		System.Windows.Forms.Label label50 = this.Label38;
		size = new System.Drawing.Size(71, 16);
		label50.Size = size;
		this.Label38.TabIndex = 47;
		this.Label38.Text = "แขวง/ตำบล";
		System.Windows.Forms.TextBox tc_code = this.Tc_code;
		location = new System.Drawing.Point(266, 42);
		tc_code.Location = location;
		System.Windows.Forms.TextBox tc_code2 = this.Tc_code;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_code2.Margin = margin;
		this.Tc_code.Name = "Tc_code";
		System.Windows.Forms.TextBox tc_code3 = this.Tc_code;
		size = new System.Drawing.Size(94, 23);
		tc_code3.Size = size;
		this.Tc_code.TabIndex = 7;
		this.Label41.AutoSize = true;
		System.Windows.Forms.Label label51 = this.Label41;
		location = new System.Drawing.Point(187, 46);
		label51.Location = location;
		this.Label41.Name = "Label41";
		System.Windows.Forms.Label label52 = this.Label41;
		size = new System.Drawing.Size(80, 16);
		label52.Size = size;
		this.Label41.TabIndex = 47;
		this.Label41.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		System.Windows.Forms.TextBox tc_road = this.Tc_road;
		location = new System.Drawing.Point(416, 14);
		tc_road.Location = location;
		System.Windows.Forms.TextBox tc_road2 = this.Tc_road;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_road2.Margin = margin;
		this.Tc_road.Name = "Tc_road";
		System.Windows.Forms.TextBox tc_road3 = this.Tc_road;
		size = new System.Drawing.Size(129, 23);
		tc_road3.Size = size;
		this.Tc_road.TabIndex = 3;
		System.Windows.Forms.TextBox tc_province = this.Tc_province;
		location = new System.Drawing.Point(56, 42);
		tc_province.Location = location;
		System.Windows.Forms.TextBox tc_province2 = this.Tc_province;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_province2.Margin = margin;
		this.Tc_province.Name = "Tc_province";
		System.Windows.Forms.TextBox tc_province3 = this.Tc_province;
		size = new System.Drawing.Size(125, 23);
		tc_province3.Size = size;
		this.Tc_province.TabIndex = 6;
		this.Label37.AutoSize = true;
		System.Windows.Forms.Label label53 = this.Label37;
		location = new System.Drawing.Point(381, 17);
		label53.Location = location;
		this.Label37.Name = "Label37";
		System.Windows.Forms.Label label54 = this.Label37;
		size = new System.Drawing.Size(32, 16);
		label54.Size = size;
		this.Label37.TabIndex = 47;
		this.Label37.Text = "ถนน";
		this.Label40.AutoSize = true;
		System.Windows.Forms.Label label55 = this.Label40;
		location = new System.Drawing.Point(11, 46);
		label55.Location = location;
		this.Label40.Name = "Label40";
		System.Windows.Forms.Label label56 = this.Label40;
		size = new System.Drawing.Size(43, 16);
		label56.Size = size;
		this.Label40.TabIndex = 47;
		this.Label40.Text = "จ\u0e31งหว\u0e31ด";
		System.Windows.Forms.TextBox tc_soi = this.Tc_soi;
		location = new System.Drawing.Point(231, 14);
		tc_soi.Location = location;
		System.Windows.Forms.TextBox tc_soi2 = this.Tc_soi;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_soi2.Margin = margin;
		this.Tc_soi.Name = "Tc_soi";
		System.Windows.Forms.TextBox tc_soi3 = this.Tc_soi;
		size = new System.Drawing.Size(129, 23);
		tc_soi3.Size = size;
		this.Tc_soi.TabIndex = 2;
		this.Label36.AutoSize = true;
		System.Windows.Forms.Label label57 = this.Label36;
		location = new System.Drawing.Point(196, 17);
		label57.Location = location;
		this.Label36.Name = "Label36";
		System.Windows.Forms.Label label58 = this.Label36;
		size = new System.Drawing.Size(33, 16);
		label58.Size = size;
		this.Label36.TabIndex = 47;
		this.Label36.Text = "ซอย";
		System.Windows.Forms.TextBox tc_moo = this.Tc_moo;
		location = new System.Drawing.Point(146, 14);
		tc_moo.Location = location;
		System.Windows.Forms.TextBox tc_moo2 = this.Tc_moo;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_moo2.Margin = margin;
		this.Tc_moo.Name = "Tc_moo";
		System.Windows.Forms.TextBox tc_moo3 = this.Tc_moo;
		size = new System.Drawing.Size(46, 23);
		tc_moo3.Size = size;
		this.Tc_moo.TabIndex = 1;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label59 = this.Label11;
		location = new System.Drawing.Point(111, 17);
		label59.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label60 = this.Label11;
		size = new System.Drawing.Size(33, 16);
		label60.Size = size;
		this.Label11.TabIndex = 1;
		this.Label11.Text = "หม\u0e39\u0e48ท\u0e35\u0e48";
		System.Windows.Forms.TextBox tc_no = this.Tc_no;
		location = new System.Drawing.Point(56, 14);
		tc_no.Location = location;
		System.Windows.Forms.TextBox tc_no2 = this.Tc_no;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_no2.Margin = margin;
		this.Tc_no.Name = "Tc_no";
		System.Windows.Forms.TextBox tc_no3 = this.Tc_no;
		size = new System.Drawing.Size(46, 23);
		tc_no3.Size = size;
		this.Tc_no.TabIndex = 0;
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label61 = this.Label8;
		location = new System.Drawing.Point(17, 18);
		label61.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label62 = this.Label8;
		size = new System.Drawing.Size(37, 16);
		label62.Size = size;
		this.Label8.TabIndex = 47;
		this.Label8.Text = "เลขท\u0e35\u0e48";
		this.TCarType.ForeColor = System.Drawing.Color.Blue;
		this.TCarType.FormattingEnabled = true;
		this.TCarType.Items.AddRange(new object[4] { "รถเก\u0e4bง", "รถกระบะ", "รถต\u0e39\u0e49", "รถบ\u0e31ส" });
		System.Windows.Forms.ComboBox tCarType = this.TCarType;
		location = new System.Drawing.Point(930, 112);
		tCarType.Location = location;
		System.Windows.Forms.ComboBox tCarType2 = this.TCarType;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCarType2.Margin = margin;
		this.TCarType.Name = "TCarType";
		System.Windows.Forms.ComboBox tCarType3 = this.TCarType;
		size = new System.Drawing.Size(170, 24);
		tCarType3.Size = size;
		this.TCarType.TabIndex = 15;
		this.TCarType.Visible = false;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label63 = this.Label9;
		location = new System.Drawing.Point(879, 113);
		label63.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label64 = this.Label9;
		size = new System.Drawing.Size(29, 16);
		label64.Size = size;
		this.Label9.TabIndex = 47;
		this.Label9.Text = "โทร";
		this.Label9.Visible = false;
		this.Label31.AutoSize = true;
		System.Windows.Forms.Label label65 = this.Label31;
		location = new System.Drawing.Point(689, 83);
		label65.Location = location;
		this.Label31.Name = "Label31";
		System.Windows.Forms.Label label66 = this.Label31;
		size = new System.Drawing.Size(63, 16);
		label66.Size = size;
		this.Label31.TabIndex = 50;
		this.Label31.Text = "ประเภทรถ";
		this.Label31.Visible = false;
		this.Label21.AutoSize = true;
		System.Windows.Forms.Label label67 = this.Label21;
		location = new System.Drawing.Point(60, 112);
		label67.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label68 = this.Label21;
		size = new System.Drawing.Size(57, 16);
		label68.Size = size;
		this.Label21.TabIndex = 50;
		this.Label21.Text = "ราคาท\u0e35\u0e48ใช\u0e49";
		this.DateTimePicker1.CustomFormat = "ddMMMMyyyy เวลา HH:mm";
		this.DateTimePicker1.Enabled = false;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(754, 21);
		dateTimePicker.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker2.Margin = margin;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		size = new System.Drawing.Size(170, 23);
		dateTimePicker3.Size = size;
		this.DateTimePicker1.TabIndex = 4;
		this.DateTimePicker1.Visible = false;
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label69 = this.Label16;
		location = new System.Drawing.Point(721, 24);
		label69.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label70 = this.Label16;
		size = new System.Drawing.Size(31, 16);
		label70.Size = size;
		this.Label16.TabIndex = 47;
		this.Label16.Text = "ว\u0e31นท\u0e35\u0e48";
		this.Label16.Visible = false;
		this.Label29.AutoSize = true;
		System.Windows.Forms.Label label71 = this.Label29;
		location = new System.Drawing.Point(268, 109);
		label71.Location = location;
		this.Label29.Name = "Label29";
		System.Windows.Forms.Label label72 = this.Label29;
		size = new System.Drawing.Size(65, 16);
		label72.Size = size;
		this.Label29.TabIndex = 47;
		this.Label29.Text = "ทะเบ\u0e35ยนรถ";
		this.Label57.AutoSize = true;
		System.Windows.Forms.Label label73 = this.Label57;
		location = new System.Drawing.Point(12, 140);
		label73.Location = location;
		this.Label57.Name = "Label57";
		System.Windows.Forms.Label label74 = this.Label57;
		size = new System.Drawing.Size(104, 16);
		label74.Size = size;
		this.Label57.TabIndex = 47;
		this.Label57.Text = "ประเภทการเข\u0e49าพ\u0e31ก";
		this.Label27.AutoSize = true;
		System.Windows.Forms.Label label75 = this.Label27;
		location = new System.Drawing.Point(902, 120);
		label75.Location = location;
		this.Label27.Name = "Label27";
		System.Windows.Forms.Label label76 = this.Label27;
		size = new System.Drawing.Size(39, 16);
		label76.Size = size;
		this.Label27.TabIndex = 47;
		this.Label27.Text = "Email";
		this.Label27.Visible = false;
		this.Label15.AutoSize = true;
		System.Windows.Forms.Label label77 = this.Label15;
		location = new System.Drawing.Point(456, 24);
		label77.Location = location;
		this.Label15.Name = "Label15";
		System.Windows.Forms.Label label78 = this.Label15;
		size = new System.Drawing.Size(99, 16);
		label78.Size = size;
		this.Label15.TabIndex = 47;
		this.Label15.Text = "หมายเลขการจอง";
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label79 = this.Label26;
		location = new System.Drawing.Point(522, 82);
		label79.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label80 = this.Label26;
		size = new System.Drawing.Size(32, 16);
		label80.Size = size;
		this.Label26.TabIndex = 47;
		this.Label26.Text = "สก\u0e38ล";
		this.Label50.AutoSize = true;
		System.Windows.Forms.Label label81 = this.Label50;
		location = new System.Drawing.Point(274, 52);
		label81.Location = location;
		this.Label50.Name = "Label50";
		System.Windows.Forms.Label label82 = this.Label50;
		size = new System.Drawing.Size(60, 16);
		label82.Size = size;
		this.Label50.TabIndex = 47;
		this.Label50.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label83 = this.Label7;
		location = new System.Drawing.Point(394, 82);
		label83.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label84 = this.Label7;
		size = new System.Drawing.Size(24, 16);
		label84.Size = size;
		this.Label7.TabIndex = 47;
		this.Label7.Text = "ช\u0e37\u0e48อ";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label85 = this.Label1;
		location = new System.Drawing.Point(1, 24);
		label85.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label86 = this.Label1;
		size = new System.Drawing.Size(116, 16);
		label86.Size = size;
		this.Label1.TabIndex = 47;
		this.Label1.Text = "บ\u0e31ตรลงทะเบ\u0e35ยนเลขท\u0e35\u0e48";
		this.TCarID.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tCarID = this.TCarID;
		location = new System.Drawing.Point(336, 106);
		tCarID.Location = location;
		System.Windows.Forms.TextBox tCarID2 = this.TCarID;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCarID2.Margin = margin;
		this.TCarID.Name = "TCarID";
		System.Windows.Forms.TextBox tCarID3 = this.TCarID;
		size = new System.Drawing.Size(322, 23);
		tCarID3.Size = size;
		this.TCarID.TabIndex = 14;
		this.TbookNo.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TbookNo.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tbookNo = this.TbookNo;
		location = new System.Drawing.Point(557, 21);
		tbookNo.Location = location;
		System.Windows.Forms.TextBox tbookNo2 = this.TbookNo;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tbookNo2.Margin = margin;
		this.TbookNo.Name = "TbookNo";
		this.TbookNo.ReadOnly = true;
		System.Windows.Forms.TextBox tbookNo3 = this.TbookNo;
		size = new System.Drawing.Size(100, 23);
		tbookNo3.Size = size;
		this.TbookNo.TabIndex = 3;
		this.TCusID.BackColor = System.Drawing.Color.FromArgb(255, 224, 192);
		this.TCusID.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tCusID = this.TCusID;
		location = new System.Drawing.Point(336, 49);
		tCusID.Location = location;
		System.Windows.Forms.TextBox tCusID2 = this.TCusID;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusID2.Margin = margin;
		this.TCusID.Name = "TCusID";
		this.TCusID.ReadOnly = true;
		System.Windows.Forms.TextBox tCusID3 = this.TCusID;
		size = new System.Drawing.Size(82, 23);
		tCusID3.Size = size;
		this.TCusID.TabIndex = 6;
		this.TdocNum.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tdocNum = this.TdocNum;
		location = new System.Drawing.Point(119, 21);
		tdocNum.Location = location;
		System.Windows.Forms.TextBox tdocNum2 = this.TdocNum;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tdocNum2.Margin = margin;
		this.TdocNum.Name = "TdocNum";
		this.TdocNum.ReadOnly = true;
		System.Windows.Forms.TextBox tdocNum3 = this.TdocNum;
		size = new System.Drawing.Size(144, 23);
		tdocNum3.Size = size;
		this.TdocNum.TabIndex = 0;
		this.Button1.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(267, 20);
		button.Location = location;
		System.Windows.Forms.Button button2 = this.Button1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button2.Margin = margin;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button3 = this.Button1;
		size = new System.Drawing.Size(37, 24);
		button3.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.UseVisualStyleBackColor = true;
		this.PanelCust.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.PanelCust.BackColor = System.Drawing.Color.DarkGray;
		this.PanelCust.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		this.PanelCust.Controls.Add(this.Label61);
		this.PanelCust.Controls.Add(this.Label59);
		this.PanelCust.Controls.Add(this.Button2);
		this.PanelCust.Controls.Add(this.ListView1);
		System.Windows.Forms.Panel panelCust = this.PanelCust;
		location = new System.Drawing.Point(126, 78);
		panelCust.Location = location;
		this.PanelCust.Name = "PanelCust";
		System.Windows.Forms.Panel panelCust2 = this.PanelCust;
		size = new System.Drawing.Size(521, 346);
		panelCust2.Size = size;
		this.PanelCust.TabIndex = 62;
		this.PanelCust.Visible = false;
		this.Label61.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Label61.BackColor = System.Drawing.Color.White;
		this.Label61.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label87 = this.Label61;
		location = new System.Drawing.Point(10, 292);
		label87.Location = location;
		this.Label61.Name = "Label61";
		System.Windows.Forms.Label label88 = this.Label61;
		size = new System.Drawing.Size(502, 23);
		label88.Size = size;
		this.Label61.TabIndex = 78;
		this.Label61.Text = "*ระหว\u0e48างการค\u0e49นหาจากช\u0e48องเบอร\u0e4cโทรถ\u0e49าไม\u0e48เจอเบอร\u0e4cโทรให\u0e49กด Enter หน\u0e49าน\u0e35\u0e49จะป\u0e34ดให\u0e49อ\u0e31ตโนม\u0e31ต\u0e34*";
		this.Label61.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label59.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label59.AutoSize = true;
		this.Label59.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label89 = this.Label59;
		location = new System.Drawing.Point(12, 322);
		label89.Location = location;
		this.Label59.Name = "Label59";
		System.Windows.Forms.Label label90 = this.Label59;
		size = new System.Drawing.Size(280, 16);
		label90.Size = size;
		this.Label59.TabIndex = 77;
		this.Label59.Text = "ด\u0e31บเบ\u0e34\u0e49ลคล\u0e34\u0e4aกท\u0e35\u0e48ช\u0e37\u0e48อล\u0e39กค\u0e49าเพ\u0e37\u0e48อเล\u0e37อกล\u0e39กค\u0e49าลงในใบเช\u0e47คอ\u0e34น";
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button2.Image = (System.Drawing.Image)resources.GetObject("Button2.Image");
		this.Button2.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button4 = this.Button2;
		location = new System.Drawing.Point(441, 318);
		button4.Location = location;
		System.Windows.Forms.Button button5 = this.Button2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button5.Margin = margin;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button6 = this.Button2;
		size = new System.Drawing.Size(72, 24);
		button6.Size = size;
		this.Button2.TabIndex = 76;
		this.Button2.Text = "    ป\u0e34ด";
		this.Button2.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button2.UseVisualStyleBackColor = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.BorderStyle = System.Windows.Forms.BorderStyle.None;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[4] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader7 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(10, 9);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(502, 283);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "รห\u0e31ส";
		this.ColumnHeader1.Width = 70;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 130;
		this.ColumnHeader3.Text = "นามสก\u0e38ล";
		this.ColumnHeader3.Width = 130;
		this.ColumnHeader7.Text = "เบอร\u0e4cโทร";
		this.ColumnHeader7.Width = 100;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Label_1);
		this.PanelEx1.Controls.Add(this.Label65);
		this.PanelEx1.Controls.Add(this.Label_2);
		this.PanelEx1.Controls.Add(this.Label64);
		this.PanelEx1.Controls.Add(this.PanelCust);
		this.PanelEx1.Controls.Add(this.Button15);
		this.PanelEx1.Controls.Add(this.Button16);
		this.PanelEx1.Controls.Add(this.Panel5);
		this.PanelEx1.Controls.Add(this.SplitContainer1);
		this.PanelEx1.Controls.Add(this.LabelButton7);
		this.PanelEx1.Controls.Add(this.Tnote);
		this.PanelEx1.Controls.Add(this.Tdebt);
		this.PanelEx1.Controls.Add(this.POver);
		this.PanelEx1.Controls.Add(this.Tcash);
		this.PanelEx1.Controls.Add(this.Tpay);
		this.PanelEx1.Controls.Add(this.TselectRoom);
		this.PanelEx1.Controls.Add(this.Label55);
		this.PanelEx1.Controls.Add(this.Grid2);
		this.PanelEx1.Controls.Add(this.Button9);
		this.PanelEx1.Controls.Add(this.Label35);
		this.PanelEx1.Controls.Add(this.LOver);
		this.PanelEx1.Controls.Add(this.Label30);
		this.PanelEx1.Controls.Add(this.Label23);
		this.PanelEx1.Controls.Add(this.LabelDebt);
		this.PanelEx1.Controls.Add(this.Label18);
		this.PanelEx1.Controls.Add(this.LabelPayed);
		this.PanelEx1.Controls.Add(this.Label33);
		this.PanelEx1.Controls.Add(this.Labelroompro);
		this.PanelEx1.Controls.Add(this.Label20);
		this.PanelEx1.Controls.Add(this.LabelTpro);
		this.PanelEx1.Controls.Add(this.Label17);
		this.PanelEx1.Controls.Add(this.LabelTroom);
		this.PanelEx1.Controls.Add(this.Label13);
		this.PanelEx1.Controls.Add(this.Label24);
		this.PanelEx1.Controls.Add(this.Button5);
		this.PanelEx1.Controls.Add(this.Label12);
		this.PanelEx1.Controls.Add(this.Button_DEP);
		this.PanelEx1.Controls.Add(this.Button_REG);
		this.PanelEx1.Controls.Add(this.Button7);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(1276, 789);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 50;
		this.Label_1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label_1.BackColor = System.Drawing.Color.CornflowerBlue;
		this.Label_1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Label_1.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Label_1.ForeColor = System.Drawing.Color.White;
		System.Windows.Forms.Label label91 = this.Label_1;
		location = new System.Drawing.Point(1140, 639);
		label91.Location = location;
		this.Label_1.Name = "Troomม\u0e31ดจำ";
		System.Windows.Forms.Label label92 = this.Label_1;
		size = new System.Drawing.Size(129, 25);
		label92.Size = size;
		this.Label_1.TabIndex = 92;
		this.Label_1.Text = "0.00";
		this.Label_1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label65.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label65.AutoSize = true;
		this.Label65.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label93 = this.Label65;
		location = new System.Drawing.Point(1045, 644);
		label93.Location = location;
		this.Label65.Name = "Label65";
		System.Windows.Forms.Label label94 = this.Label65;
		size = new System.Drawing.Size(93, 16);
		label94.Size = size;
		this.Label65.TabIndex = 91;
		this.Label65.Text = "ราคาห\u0e49อง+ม\u0e31ดจำ";
		this.Label_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label_2.BackColor = System.Drawing.Color.CornflowerBlue;
		this.Label_2.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Label_2.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Label_2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label95 = this.Label_2;
		location = new System.Drawing.Point(1140, 613);
		label95.Location = location;
		this.Label_2.Name = "Tม\u0e31ดจำ";
		System.Windows.Forms.Label label96 = this.Label_2;
		size = new System.Drawing.Size(129, 25);
		label96.Size = size;
		this.Label_2.TabIndex = 92;
		this.Label_2.Text = "0.00";
		this.Label_2.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label64.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label64.AutoSize = true;
		this.Label64.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label97 = this.Label64;
		location = new System.Drawing.Point(1068, 618);
		label97.Location = location;
		this.Label64.Name = "Label64";
		System.Windows.Forms.Label label98 = this.Label64;
		size = new System.Drawing.Size(71, 16);
		label98.Size = size;
		this.Label64.TabIndex = 91;
		this.Label64.Text = "รวมค\u0e48าม\u0e31ดจำ";
		this.Button15.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button7 = this.Button15;
		location = new System.Drawing.Point(910, 545);
		button7.Location = location;
		System.Windows.Forms.Button button8 = this.Button15;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button8.Margin = margin;
		this.Button15.Name = "Button15";
		System.Windows.Forms.Button button9 = this.Button15;
		size = new System.Drawing.Size(109, 24);
		button9.Size = size;
		this.Button15.TabIndex = 90;
		this.Button15.Text = "ไม\u0e48จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button15.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button15.UseVisualStyleBackColor = true;
		this.Button16.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button10 = this.Button16;
		location = new System.Drawing.Point(823, 545);
		button10.Location = location;
		System.Windows.Forms.Button button11 = this.Button16;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button11.Margin = margin;
		this.Button16.Name = "Button16";
		System.Windows.Forms.Button button12 = this.Button16;
		size = new System.Drawing.Size(87, 24);
		button12.Size = size;
		this.Button16.TabIndex = 89;
		this.Button16.Text = "จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button16.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button16.UseVisualStyleBackColor = true;
		this.Panel5.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Panel5.BackColor = System.Drawing.Color.DarkGray;
		this.Panel5.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		this.Panel5.Controls.Add(this.Button12);
		this.Panel5.Controls.Add(this.Button10);
		this.Panel5.Controls.Add(this.Button8);
		this.Panel5.Controls.Add(this.Tcontry2);
		this.Panel5.Controls.Add(this.Label56);
		this.Panel5.Controls.Add(this.Tname2);
		this.Panel5.Controls.Add(this.Label49);
		this.Panel5.Controls.Add(this.Button4);
		this.Panel5.Controls.Add(this.ListView2);
		System.Windows.Forms.Panel panel9 = this.Panel5;
		location = new System.Drawing.Point(648, 267);
		panel9.Location = location;
		this.Panel5.Name = "Panel5";
		System.Windows.Forms.Panel panel10 = this.Panel5;
		size = new System.Drawing.Size(589, 157);
		panel10.Size = size;
		this.Panel5.TabIndex = 62;
		this.Panel5.Visible = false;
		this.Button12.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button12.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button13 = this.Button12;
		location = new System.Drawing.Point(385, 129);
		button13.Location = location;
		System.Windows.Forms.Button button14 = this.Button12;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button14.Margin = margin;
		this.Button12.Name = "Button12";
		System.Windows.Forms.Button button15 = this.Button12;
		size = new System.Drawing.Size(145, 24);
		button15.Size = size;
		this.Button12.TabIndex = 81;
		this.Button12.Text = "เพ\u0e34\u0e48มจากบ\u0e31ตรลงทะเบ\u0e35ยน";
		this.Button12.UseVisualStyleBackColor = true;
		this.Button10.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button10.Image = (System.Drawing.Image)resources.GetObject("Button10.Image");
		this.Button10.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button16 = this.Button10;
		location = new System.Drawing.Point(279, 129);
		button16.Location = location;
		System.Windows.Forms.Button button17 = this.Button10;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button17.Margin = margin;
		this.Button10.Name = "Button10";
		System.Windows.Forms.Button button18 = this.Button10;
		size = new System.Drawing.Size(59, 24);
		button18.Size = size;
		this.Button10.TabIndex = 80;
		this.Button10.Text = "    เพ\u0e34\u0e48ม";
		this.Button10.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button10.UseVisualStyleBackColor = true;
		this.Button8.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button8.Image = (System.Drawing.Image)resources.GetObject("Button8.Image");
		this.Button8.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button19 = this.Button8;
		location = new System.Drawing.Point(336, 129);
		button19.Location = location;
		System.Windows.Forms.Button button20 = this.Button8;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button20.Margin = margin;
		this.Button8.Name = "Button8";
		System.Windows.Forms.Button button21 = this.Button8;
		size = new System.Drawing.Size(52, 24);
		button21.Size = size;
		this.Button8.TabIndex = 79;
		this.Button8.Text = "    ลบ";
		this.Button8.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button8.UseVisualStyleBackColor = true;
		this.Tcontry2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox tcontry4 = this.Tcontry2;
		location = new System.Drawing.Point(194, 129);
		tcontry4.Location = location;
		this.Tcontry2.Name = "Tcontry2";
		System.Windows.Forms.TextBox tcontry5 = this.Tcontry2;
		size = new System.Drawing.Size(79, 23);
		tcontry5.Size = size;
		this.Tcontry2.TabIndex = 78;
		this.Label56.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label56.AutoSize = true;
		System.Windows.Forms.Label label99 = this.Label56;
		location = new System.Drawing.Point(144, 132);
		label99.Location = location;
		this.Label56.Name = "Label56";
		System.Windows.Forms.Label label100 = this.Label56;
		size = new System.Drawing.Size(49, 16);
		label100.Size = size;
		this.Label56.TabIndex = 77;
		this.Label56.Text = "ประเทศ";
		this.Tname2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox tname = this.Tname2;
		location = new System.Drawing.Point(27, 129);
		tname.Location = location;
		this.Tname2.Name = "Tname2";
		System.Windows.Forms.TextBox tname2 = this.Tname2;
		size = new System.Drawing.Size(115, 23);
		tname2.Size = size;
		this.Tname2.TabIndex = 78;
		this.Label49.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label49.AutoSize = true;
		System.Windows.Forms.Label label101 = this.Label49;
		location = new System.Drawing.Point(3, 132);
		label101.Location = location;
		this.Label49.Name = "Label49";
		System.Windows.Forms.Label label102 = this.Label49;
		size = new System.Drawing.Size(24, 16);
		label102.Size = size;
		this.Label49.TabIndex = 77;
		this.Label49.Text = "ช\u0e37\u0e48อ";
		this.Button4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button4.Image = (System.Drawing.Image)resources.GetObject("Button4.Image");
		this.Button4.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button22 = this.Button4;
		location = new System.Drawing.Point(528, 129);
		button22.Location = location;
		System.Windows.Forms.Button button23 = this.Button4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button23.Margin = margin;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button24 = this.Button4;
		size = new System.Drawing.Size(52, 24);
		button24.Size = size;
		this.Button4.TabIndex = 76;
		this.Button4.Text = "    ป\u0e34ด";
		this.Button4.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button4.UseVisualStyleBackColor = true;
		this.ListView2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView2.BorderStyle = System.Windows.Forms.BorderStyle.None;
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[3] { this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6 });
		this.ListView2.FullRowSelect = true;
		this.ListView2.GridLines = true;
		System.Windows.Forms.ListView listView3 = this.ListView2;
		location = new System.Drawing.Point(10, 9);
		listView3.Location = location;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView4 = this.ListView2;
		size = new System.Drawing.Size(570, 116);
		listView4.Size = size;
		this.ListView2.TabIndex = 0;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader4.Text = "ลำด\u0e31บ";
		this.ColumnHeader5.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader5.Width = 250;
		this.ColumnHeader6.Text = "ประเทศ";
		this.ColumnHeader6.Width = 100;
		this.SplitContainer1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.SplitContainer1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.SplitContainer1.FixedPanel = System.Windows.Forms.FixedPanel.Panel1;
		System.Windows.Forms.SplitContainer splitContainer = this.SplitContainer1;
		location = new System.Drawing.Point(3, 3);
		splitContainer.Location = location;
		this.SplitContainer1.Name = "SplitContainer1";
		this.SplitContainer1.Orientation = System.Windows.Forms.Orientation.Horizontal;
		this.SplitContainer1.Panel1.Controls.Add(this.GroupBox1);
		this.SplitContainer1.Panel2.Controls.Add(this.Button6);
		this.SplitContainer1.Panel2.Controls.Add(this.Button14);
		this.SplitContainer1.Panel2.Controls.Add(this.Button13);
		this.SplitContainer1.Panel2.Controls.Add(this.Label14);
		this.SplitContainer1.Panel2.Controls.Add(this.Button3);
		this.SplitContainer1.Panel2.Controls.Add(this.Button11);
		this.SplitContainer1.Panel2.Controls.Add(this.Grid1);
		this.SplitContainer1.Panel2.Controls.Add(this.Label52);
		this.SplitContainer1.Panel2.Controls.Add(this.Label51);
		this.SplitContainer1.Panel2.Controls.Add(this.Tstart);
		this.SplitContainer1.Panel2.Controls.Add(this.Tend);
		this.SplitContainer1.Panel2.Controls.Add(this.Label53);
		this.SplitContainer1.Panel2.Controls.Add(this.Tnum);
		this.SplitContainer1.Panel2.Controls.Add(this.Label54);
		System.Windows.Forms.SplitContainer splitContainer2 = this.SplitContainer1;
		size = new System.Drawing.Size(1270, 524);
		splitContainer2.Size = size;
		this.SplitContainer1.SplitterDistance = 332;
		this.SplitContainer1.TabIndex = 88;
		this.Button6.Image = iHOTEL2025.My.Resources.Resources.reload;
		System.Windows.Forms.Button button25 = this.Button6;
		location = new System.Drawing.Point(807, 7);
		button25.Location = location;
		System.Windows.Forms.Button button26 = this.Button6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button26.Margin = margin;
		this.Button6.Name = "Button6";
		System.Windows.Forms.Button button27 = this.Button6;
		size = new System.Drawing.Size(184, 24);
		button27.Size = size;
		this.Button6.TabIndex = 87;
		this.Button6.Text = "เปล\u0e35\u0e48ยนว\u0e31น ออก ของรายเด\u0e37อน";
		this.Button6.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button6.UseVisualStyleBackColor = true;
		this.Button6.Visible = false;
		this.Button14.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button28 = this.Button14;
		location = new System.Drawing.Point(1152, 7);
		button28.Location = location;
		System.Windows.Forms.Button button29 = this.Button14;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button29.Margin = margin;
		this.Button14.Name = "Button14";
		System.Windows.Forms.Button button30 = this.Button14;
		size = new System.Drawing.Size(109, 24);
		button30.Size = size;
		this.Button14.TabIndex = 86;
		this.Button14.Text = "ไม\u0e48จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button14.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button14.UseVisualStyleBackColor = true;
		this.Button13.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button31 = this.Button13;
		location = new System.Drawing.Point(1065, 7);
		button31.Location = location;
		System.Windows.Forms.Button button32 = this.Button13;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button32.Margin = margin;
		this.Button13.Name = "Button13";
		System.Windows.Forms.Button button33 = this.Button13;
		size = new System.Drawing.Size(87, 24);
		button33.Size = size;
		this.Button13.TabIndex = 86;
		this.Button13.Text = "จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button13.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button13.UseVisualStyleBackColor = true;
		this.Label14.AutoSize = true;
		this.Label14.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label103 = this.Label14;
		location = new System.Drawing.Point(4, 11);
		label103.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label104 = this.Label14;
		size = new System.Drawing.Size(193, 16);
		label104.Size = size;
		this.Label14.TabIndex = 47;
		this.Label14.Text = "รายการห\u0e49องท\u0e35\u0e48ต\u0e49องการ Check In";
		this.Button3.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button34 = this.Button3;
		location = new System.Drawing.Point(807, 7);
		button34.Location = location;
		System.Windows.Forms.Button button35 = this.Button3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button35.Margin = margin;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button36 = this.Button3;
		size = new System.Drawing.Size(85, 24);
		button36.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "เล\u0e37อกห\u0e49อง";
		this.Button3.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button3.UseVisualStyleBackColor = true;
		this.Button11.Image = (System.Drawing.Image)resources.GetObject("Button11.Image");
		this.Button11.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button37 = this.Button11;
		location = new System.Drawing.Point(894, 7);
		button37.Location = location;
		System.Windows.Forms.Button button38 = this.Button11;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button38.Margin = margin;
		this.Button11.Name = "Button11";
		System.Windows.Forms.Button button39 = this.Button11;
		size = new System.Drawing.Size(50, 24);
		button39.Size = size;
		this.Button11.TabIndex = 4;
		this.Button11.Text = "    ลบ";
		this.Button11.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button11.UseVisualStyleBackColor = true;
		this.Grid1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Grid1.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid1.ColumnInfo = resources.GetString("Grid1.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid = this.Grid1;
		location = new System.Drawing.Point(7, 33);
		grid.Location = location;
		this.Grid1.Name = "Grid1";
		this.Grid1.Rows.Count = 500;
		this.Grid1.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid2 = this.Grid1;
		size = new System.Drawing.Size(1253, 147);
		grid2.Size = size;
		this.Grid1.StyleInfo = resources.GetString("Grid1.StyleInfo");
		this.Grid1.TabIndex = 76;
		this.Grid1.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.Label52.AutoSize = true;
		System.Windows.Forms.Label label105 = this.Label52;
		location = new System.Drawing.Point(196, 11);
		label105.Location = location;
		this.Label52.Name = "Label52";
		System.Windows.Forms.Label label106 = this.Label52;
		size = new System.Drawing.Size(66, 16);
		label106.Size = size;
		this.Label52.TabIndex = 80;
		this.Label52.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก";
		this.Label51.AutoSize = true;
		System.Windows.Forms.Label label107 = this.Label51;
		location = new System.Drawing.Point(432, 10);
		label107.Location = location;
		this.Label51.Name = "Label51";
		System.Windows.Forms.Label label108 = this.Label51;
		size = new System.Drawing.Size(55, 16);
		label108.Size = size;
		this.Label51.TabIndex = 79;
		this.Label51.Text = "ว\u0e31นท\u0e35\u0e48ออก";
		this.Tstart.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.Tstart.Enabled = false;
		this.Tstart.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tstart = this.Tstart;
		location = new System.Drawing.Point(263, 8);
		tstart.Location = location;
		System.Windows.Forms.DateTimePicker tstart2 = this.Tstart;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tstart2.Margin = margin;
		this.Tstart.Name = "Tstart";
		System.Windows.Forms.DateTimePicker tstart3 = this.Tstart;
		size = new System.Drawing.Size(164, 23);
		tstart3.Size = size;
		this.Tstart.TabIndex = 0;
		this.Tend.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.Tend.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tend = this.Tend;
		location = new System.Drawing.Point(489, 7);
		tend.Location = location;
		System.Windows.Forms.DateTimePicker tend2 = this.Tend;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tend2.Margin = margin;
		this.Tend.Name = "Tend";
		System.Windows.Forms.DateTimePicker tend3 = this.Tend;
		size = new System.Drawing.Size(171, 23);
		tend3.Size = size;
		this.Tend.TabIndex = 1;
		this.Label53.AutoSize = true;
		System.Windows.Forms.Label label109 = this.Label53;
		location = new System.Drawing.Point(665, 11);
		label109.Location = location;
		this.Label53.Name = "Label53";
		System.Windows.Forms.Label label110 = this.Label53;
		size = new System.Drawing.Size(59, 16);
		label110.Size = size;
		this.Label53.TabIndex = 84;
		this.Label53.Text = "จำนวนค\u0e37น";
		this.Tnum.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tnum = this.Tnum;
		location = new System.Drawing.Point(727, 7);
		tnum.Location = location;
		System.Windows.Forms.TextBox tnum2 = this.Tnum;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tnum2.Margin = margin;
		this.Tnum.Name = "Tnum";
		this.Tnum.ReadOnly = true;
		System.Windows.Forms.TextBox tnum3 = this.Tnum;
		size = new System.Drawing.Size(47, 23);
		tnum3.Size = size;
		this.Tnum.TabIndex = 2;
		this.Tnum.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label54.AutoSize = true;
		System.Windows.Forms.Label label111 = this.Label54;
		location = new System.Drawing.Point(777, 11);
		label111.Location = location;
		this.Label54.Name = "Label54";
		System.Windows.Forms.Label label112 = this.Label54;
		size = new System.Drawing.Size(24, 16);
		label112.Size = size;
		this.Label54.TabIndex = 85;
		this.Label54.Text = "ค\u0e37น";
		this.LabelButton7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelButton7.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.LabelButton7.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label labelButton = this.LabelButton7;
		location = new System.Drawing.Point(788, 741);
		labelButton.Location = location;
		this.LabelButton7.Name = "LabelButton7";
		System.Windows.Forms.Label labelButton2 = this.LabelButton7;
		size = new System.Drawing.Size(92, 38);
		labelButton2.Size = size;
		this.LabelButton7.TabIndex = 85;
		this.LabelButton7.Text = "ม\u0e35การ Check-Out ไปแล\u0e49ว";
		this.LabelButton7.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Tnote.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tnote.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tnote.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tnote.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox tnote = this.Tnote;
		location = new System.Drawing.Point(1140, 755);
		tnote.Location = location;
		System.Windows.Forms.TextBox tnote2 = this.Tnote;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tnote2.Margin = margin;
		this.Tnote.Name = "Tnote";
		System.Windows.Forms.TextBox tnote3 = this.Tnote;
		size = new System.Drawing.Size(129, 26);
		tnote3.Size = size;
		this.Tnote.TabIndex = 87;
		this.Tdebt.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tdebt.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tdebt.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tdebt.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox tdebt = this.Tdebt;
		location = new System.Drawing.Point(548, 759);
		tdebt.Location = location;
		System.Windows.Forms.TextBox tdebt2 = this.Tdebt;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tdebt2.Margin = margin;
		this.Tdebt.Name = "Tdebt";
		System.Windows.Forms.TextBox tdebt3 = this.Tdebt;
		size = new System.Drawing.Size(129, 26);
		tdebt3.Size = size;
		this.Tdebt.TabIndex = 87;
		this.Tdebt.Text = "0.00";
		this.Tdebt.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tdebt.Visible = false;
		this.POver.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.POver.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.POver.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.POver.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.POver.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox pOver = this.POver;
		location = new System.Drawing.Point(152, 756);
		pOver.Location = location;
		System.Windows.Forms.TextBox pOver2 = this.POver;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		pOver2.Margin = margin;
		this.POver.Name = "POver";
		this.POver.ReadOnly = true;
		System.Windows.Forms.TextBox pOver3 = this.POver;
		size = new System.Drawing.Size(129, 26);
		pOver3.Size = size;
		this.POver.TabIndex = 87;
		this.POver.Text = "0.00";
		this.POver.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tcash.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tcash.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tcash.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tcash.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tcash.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox tcash = this.Tcash;
		location = new System.Drawing.Point(548, 731);
		tcash.Location = location;
		System.Windows.Forms.TextBox tcash2 = this.Tcash;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcash2.Margin = margin;
		this.Tcash.Name = "Tcash";
		this.Tcash.ReadOnly = true;
		System.Windows.Forms.TextBox tcash3 = this.Tcash;
		size = new System.Drawing.Size(129, 26);
		tcash3.Size = size;
		this.Tcash.TabIndex = 87;
		this.Tcash.Text = "0.00";
		this.Tcash.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tcash.Visible = false;
		this.Tpay.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tpay.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tpay.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tpay.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tpay.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tpay = this.Tpay;
		location = new System.Drawing.Point(1140, 726);
		tpay.Location = location;
		System.Windows.Forms.TextBox tpay2 = this.Tpay;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tpay2.Margin = margin;
		this.Tpay.Name = "Tpay";
		this.Tpay.ReadOnly = true;
		System.Windows.Forms.TextBox tpay3 = this.Tpay;
		size = new System.Drawing.Size(129, 26);
		tpay3.Size = size;
		this.Tpay.TabIndex = 87;
		this.Tpay.Text = "0.00";
		this.Tpay.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.TselectRoom.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.TselectRoom.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TselectRoom.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tselectRoom = this.TselectRoom;
		location = new System.Drawing.Point(204, 544);
		tselectRoom.Location = location;
		System.Windows.Forms.ComboBox tselectRoom2 = this.TselectRoom;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tselectRoom2.Margin = margin;
		this.TselectRoom.Name = "TselectRoom";
		System.Windows.Forms.ComboBox tselectRoom3 = this.TselectRoom;
		size = new System.Drawing.Size(121, 24);
		tselectRoom3.Size = size;
		this.TselectRoom.TabIndex = 86;
		this.Label55.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label55.AutoSize = true;
		System.Windows.Forms.Label label113 = this.Label55;
		location = new System.Drawing.Point(152, 549);
		label113.Location = location;
		this.Label55.Name = "Label55";
		System.Windows.Forms.Label label114 = this.Label55;
		size = new System.Drawing.Size(50, 16);
		label114.Size = size;
		this.Label55.TabIndex = 80;
		this.Label55.Text = "เลขห\u0e49อง";
		this.Grid2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Grid2.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid2.ColumnInfo = resources.GetString("Grid2.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid3 = this.Grid2;
		location = new System.Drawing.Point(11, 571);
		grid3.Location = location;
		this.Grid2.Name = "Grid2";
		this.Grid2.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid4 = this.Grid2;
		size = new System.Drawing.Size(1008, 158);
		grid4.Size = size;
		this.Grid2.StyleInfo = resources.GetString("Grid2.StyleInfo");
		this.Grid2.TabIndex = 78;
		this.Grid2.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.Button9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button9.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button40 = this.Button9;
		location = new System.Drawing.Point(381, 544);
		button40.Location = location;
		System.Windows.Forms.Button button41 = this.Button9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button41.Margin = margin;
		this.Button9.Name = "Button9";
		System.Windows.Forms.Button button42 = this.Button9;
		size = new System.Drawing.Size(52, 24);
		button42.Size = size;
		this.Button9.TabIndex = 75;
		this.Button9.Text = "    ลบ";
		this.Button9.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button9.UseVisualStyleBackColor = true;
		this.Label35.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label35.AutoSize = true;
		this.Label35.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label115 = this.Label35;
		location = new System.Drawing.Point(1039, 759);
		label115.Location = location;
		this.Label35.Name = "Label35";
		System.Windows.Forms.Label label116 = this.Label35;
		size = new System.Drawing.Size(99, 16);
		label116.Size = size;
		this.Label35.TabIndex = 53;
		this.Label35.Text = "หมายเหต\u0e38การจ\u0e48าย";
		this.LOver.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LOver.AutoSize = true;
		this.LOver.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label lOver = this.LOver;
		location = new System.Drawing.Point(19, 762);
		lOver.Location = location;
		this.LOver.Name = "LOver";
		System.Windows.Forms.Label lOver2 = this.LOver;
		size = new System.Drawing.Size(131, 16);
		lOver2.Size = size;
		this.LOver.TabIndex = 53;
		this.LOver.Text = "เง\u0e34นจ\u0e48ายล\u0e48วงหน\u0e49าคงเหล\u0e37อ";
		this.Label30.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label30.AutoSize = true;
		this.Label30.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label117 = this.Label30;
		location = new System.Drawing.Point(914, 700);
		label117.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label118 = this.Label30;
		size = new System.Drawing.Size(70, 16);
		label118.Size = size;
		this.Label30.TabIndex = 53;
		this.Label30.Text = "เครด\u0e34ตการ\u0e4cด";
		this.Label30.Visible = false;
		this.Label23.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label23.AutoSize = true;
		this.Label23.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label119 = this.Label23;
		location = new System.Drawing.Point(941, 673);
		label119.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label120 = this.Label23;
		size = new System.Drawing.Size(42, 16);
		label120.Size = size;
		this.Label23.TabIndex = 53;
		this.Label23.Text = "เง\u0e34นสด";
		this.Label23.Visible = false;
		this.LabelDebt.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelDebt.BackColor = System.Drawing.Color.Black;
		this.LabelDebt.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelDebt.ForeColor = System.Drawing.Color.FromArgb(255, 128, 128);
		System.Windows.Forms.Label labelDebt = this.LabelDebt;
		location = new System.Drawing.Point(1140, 697);
		labelDebt.Location = location;
		this.LabelDebt.Name = "LabelDebt";
		System.Windows.Forms.Label labelDebt2 = this.LabelDebt;
		size = new System.Drawing.Size(129, 25);
		labelDebt2.Size = size;
		this.LabelDebt.TabIndex = 63;
		this.LabelDebt.Text = "0.00";
		this.LabelDebt.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label18.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label18.AutoSize = true;
		this.Label18.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label121 = this.Label18;
		location = new System.Drawing.Point(1037, 731);
		label121.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label122 = this.Label18;
		size = new System.Drawing.Size(101, 16);
		label122.Size = size;
		this.Label18.TabIndex = 53;
		this.Label18.Text = "รวมยอดจ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49";
		this.LabelPayed.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelPayed.BackColor = System.Drawing.Color.Black;
		this.LabelPayed.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelPayed.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label labelPayed = this.LabelPayed;
		location = new System.Drawing.Point(1140, 671);
		labelPayed.Location = location;
		this.LabelPayed.Name = "LabelPayed";
		System.Windows.Forms.Label labelPayed2 = this.LabelPayed;
		size = new System.Drawing.Size(129, 25);
		labelPayed2.Size = size;
		this.LabelPayed.TabIndex = 63;
		this.LabelPayed.Text = "0.00";
		this.LabelPayed.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label33.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label33.AutoSize = true;
		this.Label33.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label123 = this.Label33;
		location = new System.Drawing.Point(1090, 702);
		label123.Location = location;
		this.Label33.Name = "Label33";
		System.Windows.Forms.Label label124 = this.Label33;
		size = new System.Drawing.Size(49, 16);
		label124.Size = size;
		this.Label33.TabIndex = 53;
		this.Label33.Text = "ค\u0e49างจ\u0e48าย";
		this.Labelroompro.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Labelroompro.BackColor = System.Drawing.Color.Navy;
		this.Labelroompro.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Labelroompro.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Labelroompro.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label labelroompro = this.Labelroompro;
		location = new System.Drawing.Point(1140, 582);
		labelroompro.Location = location;
		this.Labelroompro.Name = "Labelroompro";
		System.Windows.Forms.Label labelroompro2 = this.Labelroompro;
		size = new System.Drawing.Size(129, 25);
		labelroompro2.Size = size;
		this.Labelroompro.TabIndex = 63;
		this.Labelroompro.Text = "0.00";
		this.Labelroompro.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label20.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label20.AutoSize = true;
		this.Label20.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label125 = this.Label20;
		location = new System.Drawing.Point(1062, 675);
		label125.Location = location;
		this.Label20.Name = "Label20";
		System.Windows.Forms.Label label126 = this.Label20;
		size = new System.Drawing.Size(76, 16);
		label126.Size = size;
		this.Label20.TabIndex = 53;
		this.Label20.Text = "รวมชำระแล\u0e49ว";
		this.LabelTpro.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelTpro.BackColor = System.Drawing.Color.Navy;
		this.LabelTpro.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabelTpro.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelTpro.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label labelTpro = this.LabelTpro;
		location = new System.Drawing.Point(1140, 556);
		labelTpro.Location = location;
		this.LabelTpro.Name = "LabelTpro";
		System.Windows.Forms.Label labelTpro2 = this.LabelTpro;
		size = new System.Drawing.Size(129, 25);
		labelTpro2.Size = size;
		this.LabelTpro.TabIndex = 63;
		this.LabelTpro.Text = "0.00";
		this.LabelTpro.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label17.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label17.AutoSize = true;
		this.Label17.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label127 = this.Label17;
		location = new System.Drawing.Point(1024, 585);
		label127.Location = location;
		this.Label17.Name = "Label17";
		System.Windows.Forms.Label label128 = this.Label17;
		size = new System.Drawing.Size(113, 16);
		label128.Size = size;
		this.Label17.TabIndex = 53;
		this.Label17.Text = "ราคาห\u0e49อง+ค\u0e48าใช\u0e49จ\u0e48าย";
		this.LabelTroom.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelTroom.BackColor = System.Drawing.Color.Navy;
		this.LabelTroom.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabelTroom.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelTroom.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label labelTroom = this.LabelTroom;
		location = new System.Drawing.Point(1140, 530);
		labelTroom.Location = location;
		this.LabelTroom.Name = "LabelTroom";
		System.Windows.Forms.Label labelTroom2 = this.LabelTroom;
		size = new System.Drawing.Size(129, 25);
		labelTroom2.Size = size;
		this.LabelTroom.TabIndex = 63;
		this.LabelTroom.Text = "0.00";
		this.LabelTroom.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label13.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label13.AutoSize = true;
		this.Label13.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label129 = this.Label13;
		location = new System.Drawing.Point(1059, 560);
		label129.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label130 = this.Label13;
		size = new System.Drawing.Size(79, 16);
		label130.Size = size;
		this.Label13.TabIndex = 53;
		this.Label13.Text = "ค\u0e48าใช\u0e49จ\u0e48ายอ\u0e37\u0e48นๆ";
		this.Label24.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label24.AutoSize = true;
		this.Label24.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label131 = this.Label24;
		location = new System.Drawing.Point(1045, 534);
		label131.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label132 = this.Label24;
		size = new System.Drawing.Size(93, 16);
		label132.Size = size;
		this.Label24.TabIndex = 53;
		this.Label24.Text = "รวมราคาห\u0e49องพ\u0e31ก";
		this.Button5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button5.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button43 = this.Button5;
		location = new System.Drawing.Point(328, 544);
		button43.Location = location;
		System.Windows.Forms.Button button44 = this.Button5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button44.Margin = margin;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button45 = this.Button5;
		size = new System.Drawing.Size(51, 24);
		button45.Size = size;
		this.Button5.TabIndex = 3;
		this.Button5.UseVisualStyleBackColor = true;
		this.Label12.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label12.AutoSize = true;
		this.Label12.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label133 = this.Label12;
		location = new System.Drawing.Point(17, 548);
		label133.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label134 = this.Label12;
		size = new System.Drawing.Size(133, 16);
		label134.Size = size;
		this.Label12.TabIndex = 47;
		this.Label12.Text = "ค\u0e48าใช\u0e49จ\u0e48ายเพ\u0e34\u0e48มเต\u0e34มอ\u0e37\u0e48นๆ";
		this.Button_DEP.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button_DEP.Enabled = false;
		this.Button_DEP.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Button_DEP.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button_DEP = this.Button_DEP;
		location = new System.Drawing.Point(960, 738);
		button_DEP.Location = location;
		System.Windows.Forms.Button button_DEP2 = this.Button_DEP;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button_DEP2.Margin = margin;
		this.Button_DEP.Name = "Button_DEP";
		System.Windows.Forms.Button button_DEP3 = this.Button_DEP;
		size = new System.Drawing.Size(59, 44);
		button_DEP3.Size = size;
		this.Button_DEP.TabIndex = 3;
		this.Button_DEP.Text = "ใบค\u0e48าม\u0e31ดจำ";
		this.Button_DEP.UseVisualStyleBackColor = true;
		this.Button_REG.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button_REG.Enabled = false;
		this.Button_REG.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Button_REG.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button_REG = this.Button_REG;
		location = new System.Drawing.Point(884, 738);
		button_REG.Location = location;
		System.Windows.Forms.Button button_REG2 = this.Button_REG;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button_REG2.Margin = margin;
		this.Button_REG.Name = "Button_REG";
		System.Windows.Forms.Button button_REG3 = this.Button_REG;
		size = new System.Drawing.Size(75, 44);
		button_REG3.Size = size;
		this.Button_REG.TabIndex = 3;
		this.Button_REG.Text = "บ\u0e31ตรลงทะเบ\u0e35ยน";
		this.Button_REG.UseVisualStyleBackColor = true;
		this.Button7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Button7.Image = (System.Drawing.Image)resources.GetObject("Button7.Image");
		this.Button7.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button46 = this.Button7;
		location = new System.Drawing.Point(784, 738);
		button46.Location = location;
		System.Windows.Forms.Button button47 = this.Button7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button47.Margin = margin;
		this.Button7.Name = "Button7";
		System.Windows.Forms.Button button48 = this.Button7;
		size = new System.Drawing.Size(99, 44);
		button48.Size = size;
		this.Button7.TabIndex = 3;
		this.Button7.Text = "      บ\u0e31นท\u0e36ก";
		this.Button7.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button7.UseVisualStyleBackColor = true;
		this.ButtonItem15.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem15.Name = "ButtonItem15";
		this.ButtonItem15.Text = "นำเข\u0e49าข\u0e49อม\u0e39ลหน\u0e31งส\u0e37อจาก Excel";
		this.ButtonItem43.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem43.Name = "ButtonItem43";
		this.ButtonItem43.Text = "นำเข\u0e49าข\u0e49อม\u0e39ลสมาช\u0e34กจาก Excel";
		this.ButtonItem40.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem40.Name = "ButtonItem40";
		this.ButtonItem40.Text = "เคล\u0e35ยร\u0e4c History  (ย\u0e37ม,ค\u0e37น ฯลฯ)";
		this.ButtonItem44.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem44.Name = "ButtonItem44";
		this.ButtonItem44.Text = "ลบรายช\u0e37\u0e48อสมาช\u0e34กท\u0e31\u0e49งหมด";
		this.ButtonItem45.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem45.Name = "ButtonItem45";
		this.ButtonItem45.Text = "ลบหน\u0e31งส\u0e37อท\u0e31\u0e49งหมด";
		this.ButtonItem46.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem46.Name = "ButtonItem46";
		this.ButtonItem46.Text = "เร\u0e35ยกค\u0e37นฐานข\u0e49อม\u0e39ลเก\u0e48า";
		this.ButtonItem16.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem16.Name = "ButtonItem16";
		this.ButtonItem16.Text = "เก\u0e35\u0e48ยวก\u0e31บโปรแกรม";
		this.ButtonItem17.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem17.Name = "ButtonItem17";
		this.ButtonItem17.Text = "ข\u0e49อม\u0e39ลระบบ";
		this.ButtonItem17.Visible = false;
		this.ButtonItem20.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem20.Name = "ButtonItem20";
		this.ButtonItem20.Text = "ผ\u0e39\u0e49ใช\u0e49งานระบบ";
		this.ButtonItem32.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem32.Name = "ButtonItem32";
		this.ButtonItem32.Text = "ลงทะเบ\u0e35ยน";
		this.ButtonItem33.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem33.Name = "ButtonItem33";
		this.ButtonItem33.Text = "ว\u0e34ธ\u0e35การใช\u0e49งาน";
		this.ButtonItem33.Visible = false;
		this.ButtonItem11.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem11.Name = "ButtonItem11";
		this.ButtonItem11.Text = "รายงานข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.TimerDrop.Interval = 300;
		this.TimerDrop2.Interval = 300;
		this.Timer1.Interval = 500;
		this.Timer2.Interval = 300;
		this.ButtonItem12.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem12.Image = (System.Drawing.Image)resources.GetObject("ButtonItem12.Image");
		this.ButtonItem12.Name = "ButtonItem12";
		this.ButtonItem12.Text = "รายงานข\u0e49อม\u0e39ลหน\u0e31งส\u0e37อ";
		this.ButtonItem13.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem13.Image = (System.Drawing.Image)resources.GetObject("ButtonItem13.Image");
		this.ButtonItem13.Name = "ButtonItem13";
		this.ButtonItem13.Text = "รายงานประว\u0e31ต\u0e34การเช\u0e48า";
		this.ButtonItem14.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem14.Image = (System.Drawing.Image)resources.GetObject("ButtonItem14.Image");
		this.ButtonItem14.Name = "ButtonItem14";
		this.ButtonItem14.Text = "รายงานสร\u0e38ปรายการย\u0e37ม";
		this.ButtonItem7.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem7.Image = (System.Drawing.Image)resources.GetObject("ButtonItem7.Image");
		this.ButtonItem7.Name = "ButtonItem7";
		this.ButtonItem7.Text = "รายงานสร\u0e38ปรายการค\u0e37น";
		this.ButtonItem28.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem28.Image = (System.Drawing.Image)resources.GetObject("ButtonItem28.Image");
		this.ButtonItem28.Name = "ButtonItem28";
		this.ButtonItem28.Text = "รายงานสร\u0e38ปรายการหน\u0e31งส\u0e37อค\u0e49างส\u0e48ง";
		this.ButtonItem29.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem29.Image = (System.Drawing.Image)resources.GetObject("ButtonItem29.Image");
		this.ButtonItem29.Name = "ButtonItem29";
		this.ButtonItem29.Text = "รายงานสร\u0e38ปยอดเง\u0e34นประจำว\u0e31น";
		this.ButtonItem30.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem30.Image = (System.Drawing.Image)resources.GetObject("ButtonItem30.Image");
		this.ButtonItem30.Name = "ButtonItem30";
		this.ButtonItem30.Text = "รายงานสร\u0e38ปค\u0e48าม\u0e31ดจำ";
		this.ButtonItem30.Visible = false;
		this.ButtonItem31.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem31.Image = (System.Drawing.Image)resources.GetObject("ButtonItem31.Image");
		this.ButtonItem31.Name = "ButtonItem31";
		this.ButtonItem31.Text = "รายงานสร\u0e38ปหน\u0e31งส\u0e37อยอดน\u0e34ยม";
		this.ButtonItem2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem2.Image = (System.Drawing.Image)resources.GetObject("ButtonItem2.Image");
		this.ButtonItem2.Name = "ButtonItem2";
		this.ButtonItem2.Text = "รายงานส\u0e48วนลด";
		this.ButtonItem41.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem41.Image = (System.Drawing.Image)resources.GetObject("ButtonItem41.Image");
		this.ButtonItem41.Name = "ButtonItem41";
		this.ButtonItem41.Text = "รายงานค\u0e49างจ\u0e48าย";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1276, 789);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmCheckIn";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "Check-In";
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.Panel3.ResumeLayout(false);
		this.ExpandablePanel1.ResumeLayout(false);
		this.Panel4.ResumeLayout(false);
		this.expandablePanel5.ResumeLayout(false);
		this.Panel2.ResumeLayout(false);
		this.Panel2.PerformLayout();
		this.expandablePanel4.ResumeLayout(false);
		this.Panel1.ResumeLayout(false);
		this.Panel1.PerformLayout();
		this.PanelCust.ResumeLayout(false);
		this.PanelCust.PerformLayout();
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.Panel5.ResumeLayout(false);
		this.Panel5.PerformLayout();
		this.SplitContainer1.Panel1.ResumeLayout(false);
		this.SplitContainer1.Panel2.ResumeLayout(false);
		this.SplitContainer1.Panel2.PerformLayout();
		this.SplitContainer1.ResumeLayout(false);
		((System.ComponentModel.ISupportInitialize)this.Grid1).EndInit();
		((System.ComponentModel.ISupportInitialize)this.Grid2).EndInit();
		this.ResumeLayout(false);
	}

	private void ComboBox3_GotFocus(object sender, EventArgs e)
	{
		TimerDrop.Enabled = true;
	}

	private void ComboBox3_KeyDown(object sender, KeyEventArgs e)
	{
		checked
		{
			if (e.KeyData == Keys.Return)
			{
				Tc_tel.DroppedDown = false;
				bool flag = false;
				int num = Tc_tel.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(Tc_tel.Items[num2], Tc_tel.Text, TextCompare: false))
					{
						flag = true;
					}
					num2++;
				}
				if (Operators.CompareString(Tc_tel.Text, "", TextCompare: false) == 0)
				{
					flag = true;
				}
				if (!flag)
				{
					Tc_tel.Items.Add(Tc_tel.Text);
				}
				Tc_fax.Focus();
			}
			if (Tc_tel.Items.Count != 0 && e.KeyData == Keys.Delete)
			{
				try
				{
					Tc_tel.Items.RemoveAt(Tc_tel.SelectedIndex);
					TimerDrop.Enabled = true;
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
			}
		}
	}

	private void TimerDrop_Tick(object sender, EventArgs e)
	{
		TimerDrop.Enabled = false;
		Tc_tel.DroppedDown = true;
	}

	private void ComboBox3_LostFocus(object sender, EventArgs e)
	{
		if (Tc_tel.Items.Count != 0)
		{
			Tc_tel.SelectedIndex = 0;
		}
	}

	private void ComboBox4_GotFocus(object sender, EventArgs e)
	{
		TimerDrop2.Enabled = true;
	}

	private void ComboBox4_KeyDown(object sender, KeyEventArgs e)
	{
		checked
		{
			if (e.KeyData == Keys.Return)
			{
				Tw_tel.DroppedDown = false;
				bool flag = false;
				int num = Tw_tel.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(Tw_tel.Items[num2], Tw_tel.Text, TextCompare: false))
					{
						flag = true;
					}
					num2++;
				}
				if (Operators.CompareString(Tw_tel.Text, "", TextCompare: false) == 0)
				{
					flag = true;
				}
				if (!flag)
				{
					Tw_tel.Items.Add(Tw_tel.Text);
				}
				Tc_fax.Focus();
			}
			if (Tw_tel.Items.Count != 0 && e.KeyData == Keys.Delete)
			{
				try
				{
					Tw_tel.Items.RemoveAt(Tw_tel.SelectedIndex);
					TimerDrop2.Enabled = true;
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
			}
		}
	}

	private void TimerDrop2_Tick(object sender, EventArgs e)
	{
		TimerDrop2.Enabled = false;
		Tw_tel.DroppedDown = true;
	}

	private void ComboBox4_LostFocus(object sender, EventArgs e)
	{
		if (Tw_tel.Items.Count != 0)
		{
			Tw_tel.SelectedIndex = 0;
		}
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			DataSet dataSet = Module1.connect("select * from HT_Customers where Cust_no ='" + ListView1.SelectedItems[0].SubItems[0].Text + "'");
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				TextBox_0.Text = "";
				TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_no"]);
				TcusName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				TextBox_2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				TCusTypeMain.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type_Main"]);
				TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				TextBox_1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Email"]);
				Tcusperfix.Text = dataSet.Tables[0].Rows[0]["Cust_perfix"].ToString();
				TcusSex.Text = dataSet.Tables[0].Rows[0]["Cust_sex"].ToString();
				TcusCardID.Text = dataSet.Tables[0].Rows[0]["Cust_IDcard"].ToString();
				Tc_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_no"]);
				Tc_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_moo"]);
				Tc_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_soi"]);
				Tc_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_road"]);
				Tc_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tambon"]);
				Tc_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_ampore"]);
				Tc_province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_province"]);
				Tc_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_code"]);
				Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				TextBox_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				Tc_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				Tw.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"]);
				Tw_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_no"]);
				Tw_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_moo"]);
				Tw_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_soi"]);
				Tw_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_road"]);
				Tw_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tambon"]);
				Tw_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_ampore"]);
				Tw_privince.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_province"]);
				Tw_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_code"]);
				Tw_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"]);
				Tw_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_fax"]);
				TwTax.Text = dataSet.Tables[0].Rows[0]["Cust_Work_tax"].ToString();
				Tover.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Price_Over"]);
				Tcontry.Text = dataSet.Tables[0].Rows[0]["Cust_contry"].ToString();
				method_0();
				RefreshScan();
				PanelCust.Visible = false;
				Button3.Focus();
				Refresh_Dep_auto(0m);
			}
		}
	}

	public void Refresh_Dep_auto(decimal pay_price = -9989989m)
	{
		object obj = "";
		if (decimal.Compare(pay_price, -9989989m) == 0)
		{
			obj = Interaction.InputBox("กร\u0e38ณาระบ\u0e38จำนวนเง\u0e34น", "จำนวนเง\u0e34น", Conversions.ToString(Conversions.ToDecimal(Tover.Text)));
			if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
			{
				return;
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
			{
				MessageBox.Show("กร\u0e38ณาใส\u0e48จำนวนเง\u0e34นเป\u0e47นต\u0e31วเลข", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
		}
		else
		{
			obj = pay_price;
		}
		decimal num = Conversions.ToDecimal(obj);
		checked
		{
			int num2 = Grid1.Rows.Count - 1;
			int num3 = 1;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num3, 1]), "", TextCompare: false) != 0)
				{
					Grid1[num3, 11] = 0;
				}
				num3++;
			}
			int num6 = Grid1.Rows.Count - 1;
			int num7 = 1;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 > num5)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num7, 1]), "", TextCompare: false) != 0)
				{
					decimal num9 = default(decimal);
					if (decimal.Compare(num, 0m) > 0)
					{
						decimal num10 = decimal.Subtract(Conversions.ToDecimal(Grid1[num7, 10]), Conversions.ToDecimal(Grid1[num7, 12]));
						if (decimal.Compare(num10, 0m) >= 0)
						{
							if (decimal.Compare(num10, num) == 0)
							{
								num9 = num;
								num = default(decimal);
							}
							else if (decimal.Compare(num10, num) > 0)
							{
								num9 = num;
								num = default(decimal);
							}
							else if (decimal.Compare(num10, num) < 0)
							{
								num9 = num10;
								num = decimal.Subtract(num, num10);
							}
							Grid1[num7, 11] = num9;
							Grid1[num7, 13] = decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid1[num7, 10]), num9), Conversions.ToDecimal(Grid1[num7, 12]));
						}
					}
				}
				num7++;
			}
			int num11 = Grid2.Rows.Count - 1;
			int num12 = 1;
			while (true)
			{
				int num13 = num12;
				int num5 = num11;
				if (num13 > num5)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num12, 1]), "", TextCompare: false) != 0)
				{
					Grid2[num12, 14] = 0;
				}
				num12++;
			}
			int num14 = Grid2.Rows.Count - 1;
			int num15 = 1;
			while (true)
			{
				int num16 = num15;
				int num5 = num14;
				if (num16 > num5)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num15, 1]), "", TextCompare: false) != 0)
				{
					decimal num17 = default(decimal);
					if (decimal.Compare(num, 0m) > 0)
					{
						decimal num18 = decimal.Subtract(Conversions.ToDecimal(Grid2[num15, 8]), Conversions.ToDecimal(Grid2[num15, 9]));
						if (decimal.Compare(num18, 0m) >= 0)
						{
							if (decimal.Compare(num18, num) == 0)
							{
								num17 = num;
								num = default(decimal);
							}
							else if (decimal.Compare(num18, num) > 0)
							{
								num17 = num;
								num = default(decimal);
							}
							else if (decimal.Compare(num18, num) < 0)
							{
								num17 = num18;
								num = decimal.Subtract(num, num18);
							}
							Grid2[num15, 14] = num17;
							Grid2[num15, 10] = decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid2[num15, 8]), num17), Conversions.ToDecimal(Grid2[num15, 9]));
						}
					}
				}
				num15++;
			}
			if (decimal.Compare(Conversions.ToDecimal(Tover.Text), 0m) > 0)
			{
				LOver.Visible = true;
				POver.Visible = true;
				POver.Text = Conversions.ToString(num);
			}
			else
			{
				LOver.Visible = false;
				POver.Visible = false;
				POver.Text = Conversions.ToString(0);
			}
			sum();
		}
	}

	public void Refresh_Dep_auto_total()
	{
		decimal d = Conversions.ToDecimal(Tover.Text);
		decimal num = default(decimal);
		checked
		{
			int num2 = Grid1.Rows.Count - 1;
			int num3 = 1;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num3, 1]), "", TextCompare: false) != 0)
				{
					num = decimal.Add(num, Conversions.ToDecimal(Grid1[num3, 11]));
				}
				num3++;
			}
			int num6 = Grid2.Rows.Count - 1;
			int num7 = 1;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 > num5)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num7, 1]), "", TextCompare: false) != 0)
				{
					num = decimal.Add(num, Conversions.ToDecimal(Grid2[num7, 14]));
				}
				num7++;
			}
			if (decimal.Compare(Conversions.ToDecimal(Tover.Text), 0m) > 0)
			{
				LOver.Visible = true;
				POver.Visible = true;
				POver.Text = Conversions.ToString(decimal.Subtract(d, num));
			}
			else
			{
				LOver.Visible = false;
				POver.Visible = false;
				POver.Text = Conversions.ToString(0);
			}
		}
	}

	public void method_0()
	{
		if (!Module1.AutoCalCust || Operators.CompareString(TCusID.Text, "", TextCompare: false) == 0)
		{
			return;
		}
		DataSet dataSet = Module1.connect("select top 1 * from HT_CheckIn_H where cin_cust_no='" + TCusID.Text + "' order by cin_date desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		checked
		{
			int num = (int)DateAndTime.DateDiff(DateInterval.Day, Conversions.ToDate(NewLateBinding.LateGet(dataSet.Tables[0].Rows[0]["Cin_Date_out"], null, "date", new object[0], null, null, null)), DateTime.Now.Date);
			int num2 = -1;
			string text = "";
			DataSet dataSet2 = Module1.connect("select * from HT_Order_Up where Cast_Type='" + TCusTypeMain.Text + "' order by id");
			int num3 = dataSet2.Tables[0].Rows.Count - 1;
			int num4 = 0;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 <= num6)
				{
					if (!Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num4]["Cust_Type"], TCusType.Text, TextCompare: false) || dataSet2.Tables[0].Rows.Count - 1 == num4)
					{
						num4++;
						continue;
					}
					num2 = Conversions.ToInteger(dataSet2.Tables[0].Rows[num4 + 1]["Cust_month"]);
					text = Conversions.ToString(dataSet2.Tables[0].Rows[num4 + 1]["Cust_type"]);
					break;
				}
				break;
			}
			if (num2 != -1)
			{
				if (num <= num2)
				{
					TCusType.Text = text;
				}
				return;
			}
			int num7 = -1;
			DataSet dataSet3 = Module1.connect("select * from HT_Order_Down where Cast_Type='" + TCusTypeMain.Text + "' order by id");
			int num8 = dataSet3.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num6 = num8;
				if (num10 <= num6)
				{
					if (!Operators.ConditionalCompareObjectEqual(dataSet3.Tables[0].Rows[num9]["Cust_Type"], TCusType.Text, TextCompare: false) || dataSet3.Tables[0].Rows.Count - 1 == num9)
					{
						num9++;
						continue;
					}
					num7 = Conversions.ToInteger(dataSet3.Tables[0].Rows[num9 + 1]["Cust_month"]);
					text = Conversions.ToString(dataSet3.Tables[0].Rows[num9 + 1]["Cust_type"]);
					break;
				}
				break;
			}
			if (num7 != -1 && num >= num7)
			{
				TCusType.Text = text;
			}
		}
	}

	private void TCusNo_GotFocus(object sender, EventArgs e)
	{
		PanelCust.Visible = true;
		listcust();
	}

	private void TCusSearch_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			if (ListView1.Items.Count == 0)
			{
				Button2_Click(null, null);
				Tcusperfix.Focus();
			}
			else
			{
				ListView1.Focus();
				SendKeys.Send("{down}");
			}
		}
	}

	private void TCusSearch_LostFocus(object sender, EventArgs e)
	{
	}

	private void TCusNo_TextChanged(object sender, EventArgs e)
	{
		listcust();
	}

	public void listcust()
	{
		ListView1.Items.Clear();
		checked
		{
			if (Operators.CompareString(TextBox_0.Text, "", TextCompare: false) != 0)
			{
				DataSet dataSet = Module1.connect("select  top 100 cust_no,cust_name,cust_name2,cust_add_tel from HT_Customers where cust_no like '%" + TextBox_0.Text + "%' or  cust_name like '%" + TextBox_0.Text + "%' or  cust_name2 like '%" + TextBox_0.Text + "%' or cust_add_tel like '%" + TextBox_0.Text + "%' order by cust_name");
				int num = dataSet.Tables[0].Rows.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						ListView.ListViewItemCollection items = ListView1.Items;
						object[] array = new object[1];
						object[] array2 = array;
						DataRow dataRow = dataSet.Tables[0].Rows[num2];
						DataRow dataRow2 = dataRow;
						string columnName = "cust_no";
						array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
						object[] array3 = array;
						object[] arguments = array3;
						bool[] array4 = new bool[1] { true };
						NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[num2].SubItems;
						array3 = new object[1];
						object[] array5 = array3;
						dataRow = dataSet.Tables[0].Rows[num2];
						DataRow dataRow3 = dataRow;
						columnName = "cust_name";
						array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
						array = array3;
						object[] arguments2 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems2 = ListView1.Items[num2].SubItems;
						array3 = new object[1];
						object[] array6 = array3;
						dataRow = dataSet.Tables[0].Rows[num2];
						DataRow dataRow4 = dataRow;
						columnName = "cust_name2";
						array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
						array = array3;
						object[] arguments3 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems3 = ListView1.Items[num2].SubItems;
						array3 = new object[1];
						object[] array7 = array3;
						dataRow = dataSet.Tables[0].Rows[num2];
						DataRow dataRow5 = dataRow;
						columnName = "cust_add_tel";
						array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
						array = array3;
						object[] arguments4 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems3, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						num2++;
						continue;
					}
					break;
				}
				return;
			}
			DataSet dataSet2 = Module1.connect("select top 100 cust_no,cust_name,cust_name2,cust_add_tel from HT_Customers order by cust_name");
			int num5 = dataSet2.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					ListView.ListViewItemCollection items2 = ListView1.Items;
					object[] array3 = new object[1];
					object[] array8 = array3;
					DataRow dataRow = dataSet2.Tables[0].Rows[num6];
					DataRow dataRow6 = dataRow;
					string columnName = "cust_no";
					array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
					object[] array = array3;
					object[] arguments5 = array;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(items2, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems4 = ListView1.Items[num6].SubItems;
					array3 = new object[1];
					object[] array9 = array3;
					dataRow = dataSet2.Tables[0].Rows[num6];
					DataRow dataRow7 = dataRow;
					columnName = "cust_name";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array = array3;
					object[] arguments6 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems4, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems5 = ListView1.Items[num6].SubItems;
					array3 = new object[1];
					object[] array10 = array3;
					dataRow = dataSet2.Tables[0].Rows[num6];
					DataRow dataRow8 = dataRow;
					columnName = "cust_name2";
					array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
					array = array3;
					object[] arguments7 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems5, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems6 = ListView1.Items[num6].SubItems;
					array3 = new object[1];
					object[] array11 = array3;
					dataRow = dataSet2.Tables[0].Rows[num6];
					DataRow dataRow9 = dataRow;
					columnName = "cust_add_tel";
					array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
					array = array3;
					object[] arguments8 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems6, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					num6++;
					continue;
				}
				break;
			}
		}
	}

	public void Clear(bool isshowsearch = false)
	{
		CheckBox1.Checked = false;
		ComboBox1.Enabled = true;
		ComboBox1.SelectedIndex = 0;
		Panel5.Visible = false;
		Tcontry.Text = "";
		Tcontry2.Text = "";
		Tname2.Text = "";
		ListView2.Items.Clear();
		Tstart.Enabled = false;
		Random random = new Random();
		tmp_no = Conversions.ToString(random.Next(1, 999999));
		Tstart.Value = DateTime.Now;
		Tend.Value = DateTime.Now;
		TdocNum.Text = GET_DOC();
		Tnote.Text = "";
		Tdebt.Text = "0.00";
		TbookNo.Text = "";
		clear_Add_cus();
		EDIT_ID = "";
		Grid1.Rows.RemoveRange(1, 499);
		Grid2.Rows.RemoveRange(1, 49);
		Grid1.Rows.Add(499);
		Grid2.Rows.Add(49);
		AddItemInCombobox();
		sum();
		LabelButton7.Visible = false;
		Button7.Enabled = true;
		Button_REG.Enabled = false;
		Button_DEP.Enabled = false;
		Button11.Visible = true;
		Button3.Enabled = true;
		RefreshScan();
		LOver.Visible = false;
		POver.Visible = false;
		if (isshowsearch)
		{
			Timer2.Enabled = true;
		}
	}

	public void clear_Add_cus()
	{
		TextBox_0.Text = "";
		TCusID.Text = "";
		TcusName.Text = "";
		TextBox_2.Text = "";
		TcusCardID.Text = "";
		TCusTypeMain.SelectedIndex = 0;
		TCarID.Text = "";
		TCarType.Text = "";
		TextBox_1.Text = "";
		Tc_no.Text = "";
		Tc_moo.Text = "";
		Tc_soi.Text = "";
		Tc_road.Text = "";
		Tc_tambon.Text = "";
		Tc_ampore.Text = "";
		Tc_province.Text = "";
		Tc_code.Text = "";
		Tc_tel.Text = "";
		TextBox_0.Text = "";
		Tc_fax.Text = "";
		Tw.Text = "";
		Tw_no.Text = "";
		Tw_moo.Text = "";
		Tw_soi.Text = "";
		Tw_road.Text = "";
		Tw_tambon.Text = "";
		Tw_ampore.Text = "";
		Tw_privince.Text = "";
		Tw_code.Text = "";
		Tw_tel.Text = "";
		Tw_fax.Text = "";
		TwTax.Text = "";
		Tover.Text = Conversions.ToString(0);
	}

	public string GET_DOC()
	{
		DataSet dataSet = Module1.connect("select top 1 * from HT_CheckIn_H where cin_date between '1/1/" + Conversions.ToString(DateTime.Now.Year) + " 00:00:00' and '12/31/" + Conversions.ToString(DateTime.Now.Year) + " 23:59:59' order by Cin_no desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return "CH" + Strings.Format(DateTime.Now, "yy-") + Strings.Format(1, "000000");
		}
		string text = dataSet.Tables[0].Rows[0]["Cin_no"].ToString().Replace("CH", "");
		checked
		{
			if (text.IndexOf("-") != -1)
			{
				text = text.Substring(text.IndexOf("-") + 1);
			}
			return "CH" + Strings.Format(DateTime.Now, "yy-") + Strings.Format(Conversions.ToInteger(text) + 1, "000000");
		}
	}

	private void FrmCheckIn_FormClosing(object sender, FormClosingEventArgs e)
	{
		Module1.checkin_mode = "";
		MSSQL.CodeErr = false;
	}

	private void FrmCheckIn_Load(object sender, EventArgs e)
	{
		WORK_ID = 0;
		MSSQL.CodeErr = true;
		Button7.Enabled = true;
		Dep_price = default(decimal);
		ButtonT1.Enabled = true;
		ButtonT2.Enabled = true;
		ButtonT3.Enabled = true;
		Label_0.Visible = false;
		TextBox_0.Enabled = true;
		LoadTypeM();
		LoadType();
		checked
		{
			int num = TCusTypeMain.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (NewLateBinding.LateIndexGet(TCusTypeMain.Items[num2], new object[1] { 2 }, null).ToString().IndexOf("ธรรมดา") != -1)
				{
					TCusTypeMain.SelectedIndex = num2;
				}
				num2++;
			}
			int num5 = TCusType.Items.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				if (NewLateBinding.LateIndexGet(TCusType.Items[num6], new object[1] { 2 }, null).ToString().IndexOf("ปกต\u0e34") != -1)
				{
					TCusType.SelectedIndex = num6;
				}
				num6++;
			}
			if (Operators.CompareString(EDIT_ID, "", TextCompare: false) != 0)
			{
				LoadBill();
			}
			else if (Operators.CompareString(TbookNo.Text, "", TextCompare: false) != 0)
			{
				CheckIN_From_Booking();
			}
			else
			{
				Clear();
			}
			if ((tmp_roomarr.Count == 0) & (Operators.CompareString(tmp_room, "", TextCompare: false) != 0) & (Operators.CompareString(TbookNo.Text, "", TextCompare: false) == 0))
			{
				Tstart.Value = Fstart;
				Tend.Value = Fend;
				Button3_Click(null, null, tmp_room);
			}
			else if ((tmp_roomarr.Count != 0) & (Operators.CompareString(tmp_room, "", TextCompare: false) == 0) & (Operators.CompareString(TbookNo.Text, "", TextCompare: false) == 0))
			{
				Tstart.Value = Fstart;
				Tend.Value = Fend;
				int num8 = tmp_roomarr.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					int num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					Button3_Click(null, null, Conversions.ToString(tmp_roomarr[num9]));
					num9++;
				}
			}
			isbook = false;
			if (Operators.CompareString(EDIT_ID, "", TextCompare: false) == 0 && Operators.CompareString(TbookNo.Text, "", TextCompare: false) == 0)
			{
				if (Operators.ConditionalCompareObjectEqual(Module1.checkin_mode, "ช\u0e31\u0e48วคราว", TextCompare: false))
				{
					ButtonT2_Click(null, null);
				}
				else if (Operators.ConditionalCompareObjectEqual(Module1.checkin_mode, "รายเด\u0e37อน", TextCompare: false))
				{
					ButtonT3_Click(null, null);
				}
			}
			if (Module1.IS_TRIAL && Conversions.ToInteger(TdocNum.Text.Substring(TdocNum.Text.IndexOf("-") + 1)) > 100)
			{
				MessageBox.Show("ค\u0e38ณได\u0e49ทดลองใช\u0e49งาน ครบ 100 รายการแล\u0e49ว");
				Close();
			}
		}
	}

	public void LoadBill()
	{
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_H where Cin_no='" + EDIT_ID + "'");
		DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Ds where Cin_no='" + EDIT_ID + "' order by Cin_Room_No");
		DataSet dataSet3 = Module1.connect("select * from HT_CheckIn_Product where Cin_no='" + EDIT_ID + "'");
		DataSet dataSet4 = Module1.connect("select * from HT_CheckIn_Other_People where Cin_no='" + EDIT_ID + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("ไม\u0e48พบเลขบ\u0e34ล " + EDIT_ID);
			return;
		}
		Clear();
		EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		Button11.Visible = false;
		ComboBox1.Enabled = false;
		ComboBox1.SelectedIndex = Conversions.ToInteger(dataSet.Tables[0].Rows[0]["Cin_type"]);
		TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_cust_no"]);
		TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		TbookNo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Book_no"]);
		CheckBox1.Checked = Conversions.ToBoolean(dataSet.Tables[0].Rows[0]["Cin_foreign"]);
		WORK_ID = Module1.GET_WORK_NUMBER(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		DataSet dataSet5 = Module1.connect("select * from HT_Customers where Cust_no ='" + TCusID.Text + "'");
		if (dataSet5.Tables[0].Rows.Count != 0)
		{
			TextBox_0.Text = "";
			TCusID.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_no"]);
			TcusName.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_name"]);
			TextBox_2.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_name2"]);
			TCusTypeMain.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Type_Main"]);
			TCusType.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Type"]);
			TextBox_1.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Email"]);
			Tcusperfix.Text = dataSet5.Tables[0].Rows[0]["Cust_perfix"].ToString();
			TcusSex.Text = dataSet5.Tables[0].Rows[0]["Cust_sex"].ToString();
			TcusCardID.Text = dataSet5.Tables[0].Rows[0]["Cust_IDcard"].ToString();
			Tc_no.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_no"]);
			Tc_moo.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_moo"]);
			Tc_soi.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_soi"]);
			Tc_road.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_road"]);
			Tc_tambon.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_tambon"]);
			Tc_ampore.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_ampore"]);
			Tc_province.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_province"]);
			Tc_code.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_code"]);
			Tc_tel.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_tel"]);
			TextBox_0.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_tel"]);
			Tc_fax.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_fax"]);
			Tw.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_Name"]);
			Tw_no.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_no"]);
			Tw_moo.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_moo"]);
			Tw_soi.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_soi"]);
			Tw_road.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_road"]);
			Tw_tambon.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_tambon"]);
			Tw_ampore.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_ampore"]);
			Tw_privince.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_province"]);
			Tw_code.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_code"]);
			Tw_tel.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_tel"]);
			Tw_fax.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_fax"]);
			TwTax.Text = dataSet5.Tables[0].Rows[0]["Cust_work_tax"].ToString();
			Tover.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Price_Over"]);
			RefreshScan();
			Button3.Focus();
		}
		TCarID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Car_id"]);
		TCarType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Car_type"]);
		bool flag = false;
		checked
		{
			int num = dataSet2.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				Tstart.Value = Conversions.ToDate(dataSet2.Tables[0].Rows[num2]["Cin_Room_In"]);
				Tend.Value = Conversions.ToDate(dataSet2.Tables[0].Rows[num2]["Cin_Room_Out"]);
				Grid1[num2 + 1, 1] = num2 + 1;
				Grid1[num2 + 1, 2] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_No"]);
				Grid1[num2 + 1, 3] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Type"]);
				Grid1[num2 + 1, 4] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_In"]);
				Grid1[num2 + 1, 5] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Out"]);
				Grid1[num2 + 1, 6] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"]);
				Grid1[num2 + 1, 7] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Dep"]);
				Grid1[num2 + 1, 8] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Price"]);
				Grid1[num2 + 1, 9] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Night"]);
				Grid1[num2 + 1, 10] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_PriceToTal"]);
				Grid1[num2 + 1, 11] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Pay_Before"]);
				Grid1[num2 + 1, 12] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_Room_Pay_Total"]);
				Grid1[num2 + 1, 13] = Operators.SubtractObject(dataSet2.Tables[0].Rows[num2]["Cin_Room_PriceToTal"], dataSet2.Tables[0].Rows[num2]["Cin_Room_Pay_Total"]);
				Grid1[num2 + 1, 14] = "0";
				if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "เข\u0e49าพ\u0e31ก", TextCompare: false), Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "Check-Out", TextCompare: false))))
				{
					Grid1[num2 + 1, 15] = true;
				}
				Grid1[num2 + 1, 16] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_note"]);
				Grid1[num2 + 1, 17] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["id"]);
				Grid1[num2 + 1, 18] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_cupon"]);
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "เข\u0e49าพ\u0e31ก", TextCompare: false))
				{
					flag = true;
				}
				num2++;
			}
			if (!flag)
			{
				Button7.Enabled = false;
				Button3.Enabled = false;
				LabelButton7.Visible = true;
			}
			int num5 = dataSet3.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				Grid2[num6 + 1, 1] = num6 + 1;
				Grid2[num6 + 1, 2] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Room_no"]);
				Grid2[num6 + 1, 3] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Ds_date"]);
				Grid2[num6 + 1, 4] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_name"]);
				Grid2[num6 + 1, 5] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_Unit"]);
				Grid2[num6 + 1, 6] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_num"]);
				Grid2[num6 + 1, 7] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_price"]);
				Grid2[num6 + 1, 8] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_priceTotal"]);
				Grid2[num6 + 1, 9] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_pay"]);
				Grid2[num6 + 1, 10] = Operators.SubtractObject(dataSet3.Tables[0].Rows[num6]["Cin_Pro_priceTotal"], dataSet3.Tables[0].Rows[num6]["Cin_Pro_pay"]);
				Grid2[num6 + 1, 11] = "0";
				Grid2[num6 + 1, 12] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_note"]);
				Grid2[num6 + 1, 13] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num6]["Cin_Pro_id"]);
				Grid2[num6 + 1, 14] = "0";
				num6++;
			}
			int num8 = dataSet4.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 > num4)
				{
					break;
				}
				ListView listView = ListView2;
				int count = listView.Items.Count;
				listView.Items.Add(Conversions.ToString(count + 1));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet4.Tables[0].Rows[num9];
				DataRow dataRow2 = dataRow;
				string columnName = "Cin_name";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet4.Tables[0].Rows[num9];
				DataRow dataRow3 = dataRow;
				columnName = "Cin_contry";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Tcontry2.Text);
				listView = null;
				num9++;
			}
			AddItemInCombobox();
			sum();
			Button_REG.Enabled = true;
			Button11.Visible = false;
			Check_Deposit();
			if (ComboBox1.SelectedIndex == 0)
			{
				ButtonT1.Enabled = true;
				ButtonT2.Enabled = false;
				ButtonT3.Enabled = false;
			}
			if (ComboBox1.SelectedIndex == 1)
			{
				ButtonT1.Enabled = false;
				ButtonT2.Enabled = true;
				ButtonT3.Enabled = false;
			}
			if (ComboBox1.SelectedIndex == 2)
			{
				ButtonT1.Enabled = false;
				ButtonT2.Enabled = false;
				ButtonT3.Enabled = true;
			}
		}
	}

	public void LoadType()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
		TCusType.DataSource = dataSet.Tables[0];
		TCusType.DisplayMember = "name";
		TCusType.ValueMember = "id";
	}

	public void LoadTypeM()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType_Main order by name desc");
		TCusTypeMain.DataSource = dataSet.Tables[0];
		TCusTypeMain.DisplayMember = "name";
		TCusTypeMain.ValueMember = "id";
	}

	private void Tstart_ValueChanged(object sender, EventArgs e)
	{
		if (DateTime.Compare(Tstart.Value, DateTime.Now) < 0)
		{
			Tstart.Value = DateTime.Now;
		}
		if (DateTime.Compare(Tend.Value, Tstart.Value) == 0)
		{
			if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
			else
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.AddDays(1.0).Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
		}
		if (DateTime.Compare(Tend.Value, Tstart.Value) < 0)
		{
			if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
			else
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.AddDays(1.0).Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
		}
		Tnum.Text = Conversions.ToString(DateAndTime.DateDiff(DateInterval.Day, Tstart.Value.Date, Tend.Value.Date));
		if (DateTime.Compare(Tstart.Value.Date, Tend.Value.Date) == 0)
		{
			Tnum.Text = Conversions.ToString(1);
		}
		else if (DateTime.Compare(Tstart.Value.Date, Tend.Value.Date) != 0 && ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0)))
		{
			Tnum.Text = Conversions.ToString(decimal.Add(Conversions.ToDecimal(Tnum.Text), 1m));
		}
	}

	public void Button3_Click(object sender, EventArgs e, string string_0 = "")
	{
		if (Operators.CompareString(TdocNum.Text, "", TextCompare: false) == 0)
		{
			Clear();
			TdocNum.Text = GET_DOC();
		}
		checked
		{
			if (Operators.CompareString(string_0, "", TextCompare: false) == 0)
			{
				MyProject.Forms.FormSearchRoomsCin2.DateTimePicker1.Value = Tstart.Value;
				MyProject.Forms.FormSearchRoomsCin2.DateTimePicker2.Value = Tend.Value;
				MyProject.Forms.FormSearchRoomsCin2.Days = Conversions.ToInteger(Tnum.Text);
				if (!((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 499m) <= 0)))
				{
				}
				MyProject.Forms.FormSearchRoomsCin2.noRoom = "";
				int num = Grid1.Rows.Count - 1;
				int num2 = 1;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4 || Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) == 0)
					{
						break;
					}
					if (Operators.CompareString(MyProject.Forms.FormSearchRoomsCin2.noRoom, "", TextCompare: false) == 0)
					{
						MyProject.Forms.FormSearchRoomsCin2.noRoom = "'" + Conversions.ToString(Grid1[num2, 2]) + "'";
					}
					else
					{
						FormSearchRoomsCin2 formSearchRoomsCin = MyProject.Forms.FormSearchRoomsCin2;
						formSearchRoomsCin.noRoom = formSearchRoomsCin.noRoom + ",'" + Conversions.ToString(Grid1[num2, 2]) + "'";
					}
					num2++;
				}
				MyProject.Forms.FormSearchRoomsCin2.filter = "";
				MyProject.Forms.FormSearchRoomsCin2.ShowDialog();
			}
			else
			{
				MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR.Clear();
				MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR.Add(string_0);
			}
			if (MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR.Count == 0)
			{
				return;
			}
			int num5 = MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					return;
				}
				int num8 = Grid1.Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					if (!Operators.ConditionalCompareObjectEqual(MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR[num6], Conversions.ToString(Grid1[num9, 2]), TextCompare: false))
					{
						num9++;
						continue;
					}
					MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ม\u0e35เลขห\u0e49อง ", MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR[num6]), " อย\u0e39\u0e48ในรายการอย\u0e39\u0e48แล\u0e49ว")));
					return;
				}
				Conversions.ToDecimal(Tover.Text);
				DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR[num6]), "'")));
				if (dataSet.Tables[0].Rows.Count != 0)
				{
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms_Price where Room_Type='", dataSet.Tables[0].Rows[0]["Room_Type"]), "' and Room_CustType='"), TCusType.Text), "'")));
					if (dataSet2.Tables[0].Rows.Count == 0)
					{
						break;
					}
					if (dataSet2.Tables[0].Rows.Count != 0)
					{
						int num11 = 1;
						int num12 = Grid1.Rows.Count - 1;
						int num13 = 0;
						while (true)
						{
							int num14 = num13;
							num4 = num12;
							if (num14 <= num4)
							{
								if (Operators.CompareString(Conversions.ToString(Grid1[num13, 1]), "", TextCompare: false) != 0)
								{
									num13++;
									continue;
								}
								num11 = num13;
								break;
							}
							break;
						}
						int num15 = 0;
						DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_SET_RoomType where name='", dataSet.Tables[0].Rows[0]["Room_Type"]), "'")));
						if (dataSet3.Tables[0].Rows.Count != 0 && Operators.CompareString(dataSet3.Tables[0].Rows[0]["Room_priceA"].ToString(), "", TextCompare: false) != 0)
						{
							if (Operators.ConditionalCompareObjectEqual(dataSet3.Tables[0].Rows[0]["Room_priceA"], 0, TextCompare: false))
							{
								num15 = 0;
							}
							else if (Operators.ConditionalCompareObjectGreater(dataSet3.Tables[0].Rows[0]["Room_priceA"], 0, TextCompare: false))
							{
								num15 = 2;
							}
						}
						Grid1[num11, 1] = num11;
						Grid1[num11, 2] = RuntimeHelpers.GetObjectValue(MyProject.Forms.FormSearchRoomsCin2.SelectNO_ARR[num6]);
						Grid1[num11, 3] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Room_Type"]);
						Grid1[num11, 4] = Tstart.Value;
						Grid1[num11, 5] = Tend.Value;
						Grid1[num11, 6] = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
						Grid1[num11, 7] = Dep_price;
						Grid1[num11, 8] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Room_Price"]), "#,##0.00");
						Grid1[num11, 9] = Tnum.Text;
						Grid1[num11, 10] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Room_Price"]), Conversions.ToDecimal(Tnum.Text)), "#,##0.00");
						Grid1[num11, 11] = Strings.Format(0, "#,##0.00");
						Grid1[num11, 12] = "0.00";
						Grid1[num11, 13] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Room_Price"]), Conversions.ToDecimal(Tnum.Text)), "#,##0.00");
						Grid1[num11, 14] = "0.00";
						Grid1[num11, 15] = true;
						Grid1[num11, 18] = num15;
						AddItemInCombobox();
						sum();
					}
				}
				num6++;
			}
			MessageBox.Show("ราคาส\u0e34นค\u0e49าย\u0e31งไม\u0e48ได\u0e49ต\u0e31\u0e49งราคา กร\u0e38ณาไปต\u0e31\u0e49งราคาก\u0e48อน");
		}
	}

	public void Check_Deposit()
	{
		Button_DEP.Enabled = false;
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
				{
					if (Operators.CompareString(Conversions.ToString(Grid1[num2, 7]), "", TextCompare: false) == 0)
					{
						Grid1[num2, 7] = 0;
					}
					if (decimal.Compare(Conversions.ToDecimal(Grid1[num2, 7]), 0m) != 0)
					{
						Button_DEP.Enabled = true;
					}
				}
				num2++;
			}
		}
	}

	private void Grid1_AfterEdit(object sender, RowColEventArgs e)
	{
		if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 2]), "", TextCompare: false) == 0)
		{
			Grid1[e.Row, e.Col] = "";
			return;
		}
		if (e.Col == 8)
		{
			if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 8]), "", TextCompare: false) == 0)
			{
				Grid1[e.Row, 8] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid1[e.Row, 8])))
			{
				Grid1[e.Row, 8] = "0";
			}
			Grid1[e.Row, 10] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(Grid1[e.Row, 8]), Conversions.ToDecimal(Grid1[e.Row, 9])), "#,##0.00");
			Grid1[e.Row, 13] = Strings.Format(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid1[e.Row, 10]), Conversions.ToDecimal(Grid1[e.Row, 11])), Conversions.ToDecimal(Grid1[e.Row, 12])), "#,##0.00");
			Grid1[e.Row, 14] = "0";
			sum();
		}
		if (e.Col == 7)
		{
			if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 7]), "", TextCompare: false) == 0)
			{
				Grid1[e.Row, 7] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid1[e.Row, 7])))
			{
				Grid1[e.Row, 7] = "0";
			}
			sum();
		}
		if (e.Col == 14)
		{
			if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 14]), "", TextCompare: false) == 0)
			{
				Grid1[e.Row, 14] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid1[e.Row, 14])))
			{
				Grid1[e.Row, 14] = "0";
			}
			sum();
		}
		if (e.Col == 18)
		{
			if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 18]), "", TextCompare: false) == 0)
			{
				Grid1[e.Row, 18] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid1[e.Row, 18])))
			{
				Grid1[e.Row, 18] = "0";
			}
		}
	}

	private void Grid1_StartEdit(object sender, RowColEventArgs e)
	{
		if (e.Col == 14 && decimal.Compare(Conversions.ToDecimal(Grid1[e.Row, 14]), 0m) == 0 && decimal.Compare(Conversions.ToDecimal(Grid1[e.Row, 13]), 0m) > 0)
		{
			Grid1[e.Row, 14] = RuntimeHelpers.GetObjectValue(Grid1[e.Row, 13]);
			sum();
		}
	}

	private void Grid2_StartEdit(object sender, RowColEventArgs e)
	{
		if (e.Col == 11 && decimal.Compare(Conversions.ToDecimal(Grid2[e.Row, 11]), 0m) == 0)
		{
			Grid2[e.Row, 11] = RuntimeHelpers.GetObjectValue(Grid2[e.Row, 10]);
			sum();
		}
	}

	public void AddItemInCombobox()
	{
		TselectRoom.Items.Clear();
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4 && Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
				{
					TselectRoom.Items.Add(RuntimeHelpers.GetObjectValue(Grid1[num2, 2]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void Button5_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TselectRoom.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกห\u0e49องพ\u0e31ก");
			return;
		}
		MyProject.Forms.FormSearchPro.ShowDialog();
		if (!Operators.ConditionalCompareObjectNotEqual(MyProject.Forms.FormSearchPro.SelectNO, "", TextCompare: false))
		{
			return;
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products where Pro_no='", MyProject.Forms.FormSearchPro.SelectNO), "'")));
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบรห\u0e31สส\u0e34นค\u0e49า ", MyProject.Forms.FormSearchPro.SelectNO)));
			return;
		}
		decimal num = default(decimal);
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products_Price where P_ID='", dataSet.Tables[0].Rows[0]["Pro_no"]), "' and P_CustType='"), TCusType.Text), "'")));
		if (dataSet2.Tables[0].Rows.Count != 0)
		{
			num = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["P_Price"]);
		}
		string right = "";
		decimal num2 = 1m;
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Pro_PriceB"], 0, TextCompare: false))
		{
			MyProject.Forms.FormSETelec_0.ShowDialog();
			if (!MyProject.Forms.FormSETelec_0.isOK)
			{
				return;
			}
			num2 = decimal.Subtract(Conversions.ToDecimal(MyProject.Forms.FormSETelec_0.TextBox2.Text), Conversions.ToDecimal(MyProject.Forms.FormSETelec_0.TextBox1.Text));
			right = " (จากหน\u0e48วยท\u0e35\u0e48 " + MyProject.Forms.FormSETelec_0.TextBox1.Text + " ถ\u0e36งหน\u0e48วยท\u0e35\u0e48 " + MyProject.Forms.FormSETelec_0.TextBox2.Text + ")";
		}
		int num3 = 1;
		checked
		{
			int num4 = Grid2.Rows.Count - 1;
			int num5 = 0;
			while (true)
			{
				int num6 = num5;
				int num7 = num4;
				if (num6 <= num7)
				{
					if (Operators.CompareString(Conversions.ToString(Grid2[num5, 1]), "", TextCompare: false) != 0)
					{
						num5++;
						continue;
					}
					num3 = num5;
					break;
				}
				break;
			}
			Grid2[num3, 1] = num3;
			Grid2[num3, 2] = TselectRoom.Text;
			Grid2[num3, 3] = DateTime.Now;
			Grid2[num3, 4] = Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["Pro_name"], right);
			Grid2[num3, 5] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Pro_Unit"]);
			Grid2[num3, 6] = num2;
			Grid2[num3, 7] = Strings.Format(num, "#,##0.00");
			Grid2[num3, 8] = Strings.Format(decimal.Multiply(num, num2), "#,##0.00");
			Grid2[num3, 9] = "0.00";
			Grid2[num3, 10] = Strings.Format(decimal.Multiply(num, num2), "#,##0.00");
			Grid2[num3, 11] = "0.00";
			Grid2[num3, 12] = "";
			Grid2[num3, 13] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Pro_no"]);
			Grid2[num3, 14] = "0.00";
			sum();
		}
	}

	private void Grid2_AfterEdit(object sender, RowColEventArgs e)
	{
		if (Operators.CompareString(Conversions.ToString(Grid2[e.Row, 2]), "", TextCompare: false) == 0)
		{
			Grid2[e.Row, e.Col] = "";
			return;
		}
		if ((e.Col == 6) | (e.Col == 7))
		{
			if (Operators.CompareString(Conversions.ToString(Grid2[e.Row, 6]), "", TextCompare: false) == 0)
			{
				Grid2[e.Row, 6] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid2[e.Row, 6])))
			{
				Grid2[e.Row, 6] = "0";
			}
			if (Operators.CompareString(Conversions.ToString(Grid2[e.Row, 7]), "", TextCompare: false) == 0)
			{
				Grid2[e.Row, 7] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid2[e.Row, 7])))
			{
				Grid2[e.Row, 7] = "0";
			}
			Grid2[e.Row, 8] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(Grid2[e.Row, 6]), Conversions.ToDecimal(Grid2[e.Row, 7])), "#,##0.00");
			Grid2[e.Row, 10] = Strings.Format(Conversions.ToDecimal(Operators.SubtractObject(Operators.SubtractObject(Grid2[e.Row, 8], Conversions.ToDecimal(Grid2[e.Row, 9])), Conversions.ToDecimal(Grid2[e.Row, 14]))), "#,##0.00");
			Grid2[e.Row, 11] = "0";
			sum();
		}
		if (e.Col == 11)
		{
			if (Operators.CompareString(Conversions.ToString(Grid2[e.Row, 11]), "", TextCompare: false) == 0)
			{
				Grid2[e.Row, 11] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid2[e.Row, 11])))
			{
				Grid2[e.Row, 11] = "0";
			}
			sum();
		}
	}

	public void sum()
	{
		if (Operators.CompareString(Tdebt.Text, "", TextCompare: false) == 0)
		{
			Tdebt.Text = "0.00";
		}
		if (!Versioned.IsNumeric(Tdebt.Text))
		{
			Tdebt.Text = "0.00";
		}
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		decimal num5 = default(decimal);
		decimal num6 = default(decimal);
		checked
		{
			int num7 = Grid1.Rows.Count - 1;
			int num8 = 1;
			while (true)
			{
				int num9 = num8;
				int num10 = num7;
				if (num9 > num10)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num8, 1]), "", TextCompare: false) != 0)
				{
					num = decimal.Add(num, Conversions.ToDecimal(Grid1[num8, 10]));
					num3 = decimal.Add(num3, Conversions.ToDecimal(Grid1[num8, 12]));
					num5 = decimal.Add(num5, Conversions.ToDecimal(Grid1[num8, 13]));
					num4 = decimal.Add(num4, Conversions.ToDecimal(Grid1[num8, 14]));
					num6 = decimal.Add(num6, Conversions.ToDecimal(Grid1[num8, 7]));
				}
				num8++;
			}
			int num11 = Grid2.Rows.Count - 1;
			int num12 = 1;
			while (true)
			{
				int num13 = num12;
				int num10 = num11;
				if (num13 > num10)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num12, 1]), "", TextCompare: false) != 0)
				{
					num2 = decimal.Add(num2, Conversions.ToDecimal(Grid2[num12, 8]));
					num3 = decimal.Add(num3, Conversions.ToDecimal(Grid2[num12, 9]));
					num5 = decimal.Add(num5, Conversions.ToDecimal(Grid2[num12, 10]));
					num4 = decimal.Add(num4, Conversions.ToDecimal(Grid2[num12, 11]));
				}
				num12++;
			}
			LabelTroom.Text = Strings.Format(num, "#,##0.00");
			LabelTpro.Text = Strings.Format(num2, "#,##0.00");
			Labelroompro.Text = Strings.Format(decimal.Add(num, num2), "#,##0.00");
			LabelPayed.Text = Strings.Format(num3, "#,##0.00");
			LabelDebt.Text = Strings.Format(num5, "#,##0.00");
			Label_2.Text = Strings.Format(num6, "#,##0.00");
			Label_1.Text = Strings.Format(decimal.Add(decimal.Add(num6, num), num2), "#,##0.00");
			Tpay.Text = Strings.Format(num4, "#,##0.00");
			Tcash.Text = Strings.Format(decimal.Subtract(num4, Conversions.ToDecimal(Tdebt.Text)), "#,##0.00");
			Refresh_Dep_auto_total();
		}
	}

	private void Button9_Click(object sender, EventArgs e)
	{
		checked
		{
			try
			{
				Grid2.RemoveItem(Grid2.RowSel);
				Grid2.AddItem(Conversions.ToString(1));
				int num = Grid2.Rows.Count - 1;
				int num2 = 1;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4 || Operators.CompareString(Conversions.ToString(Grid2[num2, 1]), "", TextCompare: false) == 0)
					{
						break;
					}
					Grid2[num2, 1] = num2;
					num2++;
				}
				sum();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void Button11_Click(object sender, EventArgs e)
	{
		checked
		{
			try
			{
				Grid1.RemoveItem(Grid1.RowSel);
				Grid1.AddItem(Conversions.ToString(1));
				int num = 0;
				int num2 = Grid1.Rows.Count - 1;
				int num3 = 1;
				while (true)
				{
					int num4 = num3;
					int num5 = num2;
					if (num4 > num5 || Operators.CompareString(Conversions.ToString(Grid1[num3, 1]), "", TextCompare: false) == 0)
					{
						break;
					}
					Grid1[num3, 1] = num3;
					num = num3;
					num3++;
				}
				if (num == 0)
				{
					Tstart.Enabled = true;
					Tend.Enabled = true;
				}
				sum();
				Refresh_Dep_auto_total();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void Tdebt_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			Tnote.Focus();
		}
	}

	private void Tdebt_LostFocus(object sender, EventArgs e)
	{
		sum();
	}

	public string SAVE_CUST()
	{
		string text = "";
		DataSet dataSet = Module1.connect("select top 1 * from HT_Customers order by id desc");
		text = ((dataSet.Tables[0].Rows.Count != 0) ? ("C" + Strings.Format(Operators.AddObject(dataSet.Tables[0].Rows[0]["id"], 1), "0000")) : ("C" + Strings.Format(1, "0000")));
		object right = Module1.get_id("HT_Customers", "id");
		object left = "INSERT INTO [HT_Customers]";
		left = Operators.ConcatenateObject(left, "([id]");
		left = Operators.ConcatenateObject(left, ",[Cust_no]");
		left = Operators.ConcatenateObject(left, ",[Cust_name]");
		left = Operators.ConcatenateObject(left, ",[Cust_name2]");
		left = Operators.ConcatenateObject(left, ",[Cust_Type]");
		left = Operators.ConcatenateObject(left, ",[Cust_Email]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_no]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_moo]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_soi]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_road]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_tambon]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_ampore]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_province]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_code]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_tel]");
		left = Operators.ConcatenateObject(left, ",[Cust_Add_fax]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_Name]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_no]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_moo]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_soi]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_road]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_tambon]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_ampore]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_province]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_code]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_tel]");
		left = Operators.ConcatenateObject(left, ",[Cust_Work_fax],[Cust_Last_Change],[Cust_Type_Main]");
		left = Operators.ConcatenateObject(left, ",[Cust_perfix]");
		left = Operators.ConcatenateObject(left, ",[Cust_sex]");
		left = Operators.ConcatenateObject(left, ",[Cust_IDcard],[Cust_Contry],[Cust_Work_tax]");
		left = Operators.ConcatenateObject(left, ")");
		left = Operators.ConcatenateObject(left, " VALUES ");
		left = Operators.ConcatenateObject(left, "(");
		left = Operators.ConcatenateObject(left, right);
		left = Operators.ConcatenateObject(left, string.Concat(",'" + text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusName.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_2.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TCusType.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_1.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_no.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_moo.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_soi.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_road.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_tambon.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_ampore.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_province.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_code.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_tel.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_fax.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_no.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_moo.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_soi.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_road.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_tambon.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_ampore.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_privince.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_code.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_tel.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tw_fax.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now.Date), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TCusTypeMain.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tcusperfix.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusSex.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusCardID.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tcontry.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TwTax.Text, "'"));
		left = Operators.ConcatenateObject(left, ")");
		Module1.connect(Conversions.ToString(left));
		return text;
	}

	public void EDIT_CUST()
	{
		object left = "UPDATE [HT_Customers] SET ";
		left = Operators.ConcatenateObject(left, string.Concat(" [Cust_name]='" + TcusName.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_name2]='" + TextBox_2.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Type]='" + TCusType.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Type_main]='" + TCusTypeMain.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Email]='" + TextBox_1.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_no]='" + Tc_no.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_moo]='" + Tc_moo.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_soi]='" + Tc_soi.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_road]='" + Tc_road.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_tambon]='" + Tc_tambon.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_ampore]='" + Tc_ampore.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_province]='" + Tc_province.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_code]='" + Tc_code.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_tel]='" + Tc_tel.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Add_fax]='" + Tc_fax.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_Name]='" + Tw.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_no]='" + Tw_no.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_moo]='" + Tw_moo.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_soi]='" + Tw_soi.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_road]='" + Tw_road.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_tambon]='" + Tw_tambon.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_ampore]='" + Tw_ampore.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_province]='" + Tw_privince.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_code]='" + Tw_code.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_tel]='" + Tw_tel.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_fax]='" + Tw_fax.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Work_tax]='" + TwTax.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_perfix]='" + Tcusperfix.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_sex]='" + TcusSex.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_IDcard]='" + TcusCardID.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Contry]='" + Tcontry.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(" where Cust_no='" + TCusID.Text, "'"));
		Module1.connect(Conversions.ToString(left));
	}

	private void Button7_Click(object sender, EventArgs e)
	{
		Button7.Enabled = false;
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			Button7.Enabled = true;
			return;
		}
		if (Operators.CompareString(TcusName.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อผ\u0e39\u0e49เข\u0e49าพ\u0e31ก");
			Button7.Enabled = true;
			return;
		}
		bool flag = false;
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
				{
					flag = true;
				}
				num2++;
			}
			if (!flag)
			{
				MessageBox.Show("กร\u0e38ณาเพ\u0e34\u0e48มรายการห\u0e49องพ\u0e31ก");
				Button7.Enabled = true;
				return;
			}
			if (Operators.CompareString(EDIT_ID, "", TextCompare: false) == 0)
			{
				string text = "";
				int num5 = TselectRoom.Items.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					text = ((Operators.CompareString(text, "", TextCompare: false) != 0) ? Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", TselectRoom.Items[num6]), "'"))) : Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("'", TselectRoom.Items[num6]), "'")));
					num6++;
				}
				if (Operators.CompareString(text, "", TextCompare: false) != 0)
				{
					DataSet dataSet = Module1.connect("select room_no from HT_Rooms where room_no in (" + text + ") and Room_use='yes'");
					if (dataSet.Tables[0].Rows.Count != 0)
					{
						object obj = "";
						int num8 = dataSet.Tables[0].Rows.Count - 1;
						int num9 = 0;
						while (true)
						{
							int num10 = num9;
							int num4 = num8;
							if (num10 > num4)
							{
								break;
							}
							obj = ((!Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false)) ? Operators.ConcatenateObject(obj, Operators.ConcatenateObject(", ", dataSet.Tables[0].Rows[num9]["room_no"])) : RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["room_no"]));
							num9++;
						}
						MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49องพ\u0e31ก ", obj), " ได\u0e49เช\u0e47คอ\u0e34นไปแล\u0e49ว")));
						Button7.Enabled = true;
						return;
					}
				}
			}
			Tc_tel.Text = TextBox_0.Text.Replace(" ", "").Replace("-", "");
			if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), Conversions.ToDecimal(LabelDebt.Text)) > 0)
			{
				MyProject.Forms.FormConfirmOverBill.LabelName.Text = Strings.Format(decimal.Subtract(Conversions.ToDecimal(Tpay.Text), Conversions.ToDecimal(LabelDebt.Text)), "#,##0.00");
				MyProject.Forms.FormConfirmOverBill.ShowDialog();
				if (!MyProject.Forms.FormConfirmOverBill.ISOK)
				{
					Button7.Enabled = true;
					return;
				}
			}
			Cursor = Cursors.WaitCursor;
			if (Operators.CompareString(EDIT_ID, "", TextCompare: false) == 0)
			{
				SAVE_ADD();
			}
			else
			{
				SAVE_EDIT();
			}
			Cursor = Cursors.Default;
		}
	}

	public void SAVE_ADD()
	{
		checked
		{
			if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
			{
				if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) > 0)
				{
					MyProject.Forms.FormConfirmPay.PTOTAl = Conversions.ToDecimal(Tpay.Text);
					MyProject.Forms.FormConfirmPay.ShowDialog();
					Tcash.Text = Conversions.ToString(MyProject.Forms.FormConfirmPay.PCASH);
					Tdebt.Text = Conversions.ToString(MyProject.Forms.FormConfirmPay.PCREDIT);
					if (!MyProject.Forms.FormConfirmPay.ISOK)
					{
						Button7.Enabled = true;
						return;
					}
				}
				string text = "";
				if (Operators.CompareString(TCusID.Text, "", TextCompare: false) != 0)
				{
					EDIT_CUST();
					text = TCusID.Text;
				}
				else
				{
					text = SAVE_CUST();
				}
				if (ListView2.Items.Count == 0)
				{
					Button12_Click(null, null);
				}
				TdocNum.Text = GET_DOC();
				string text2 = "";
				int num = TselectRoom.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(TselectRoom.Items[num2], " ")));
					num2++;
				}
				DateTime now = DateTime.Now;
				string sIR_PAY = Module1.GetSIR_PAY();
				decimal num5 = default(decimal);
				new ArrayList();
				Module1.connect("update Tb_Save_Image set cin_no='" + TdocNum.Text + "',cust_no='" + text + "',tmp_no='' where tmp_no='" + tmp_no + "'");
				object obj = "";
				int num6 = Module1.get_id("HT_CheckIn_Ds", "id");
				int num7 = Grid1.Rows.Count - 1;
				int num8 = 1;
				while (true)
				{
					int num9 = num8;
					int num4 = num7;
					if (num9 > num4 || Operators.CompareString(Conversions.ToString(Grid1[num8, 1]), "", TextCompare: false) == 0)
					{
						break;
					}
					string text3 = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
					if (Operators.ConditionalCompareObjectEqual(Grid1[num8, 15], true, TextCompare: false))
					{
						text3 = "เข\u0e49าพ\u0e31ก";
					}
					string text4 = "ย\u0e31งไม\u0e48ค\u0e37นค\u0e48าม\u0e31ดจำ";
					if (decimal.Compare(Conversions.ToDecimal(Grid1[num8, 7]), 0m) <= 0)
					{
						text4 = "ไม\u0e48เก\u0e47บค\u0e48าม\u0e31ดจำ";
					}
					Module1.Power_set(Conversions.ToString(Grid1[num8, 2]), "ON", "", "เป\u0e34ดไฟ อ\u0e31ตโนม\u0e31ต\u0e34 จากเช\u0e47คอ\u0e34น No." + TdocNum.Text);
					num5 = decimal.Add(num5, Conversions.ToDecimal(Grid1[num8, 11]));
					obj = "INSERT INTO [HT_CheckIn_Ds]";
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, "[id],[Cin_No]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_No]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Type]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_In]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Out]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Status]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Dep]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Price]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Night]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_PriceToTal]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Pay_Before]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Pay_Total],[Cin_note],[Cin_dep_status],[Cin_cupon])");
					obj = Operators.ConcatenateObject(obj, "VALUES");
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, " " + Conversions.ToString(num6));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TdocNum.Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid1[num8, 2]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid1[num8, 3]), "'"));
					obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num8, 4]), "'"));
					obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num8, 5]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text3, "'"));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num8, 7])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num8, 8])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num8, 9])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num8, 10])));
					obj = Operators.ConcatenateObject(obj, ",0");
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid1[num8, 12]), Conversions.ToDecimal(Grid1[num8, 14])), Conversions.ToDecimal(Grid1[num8, 11]))));
					obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num8, 16]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text4, "'"));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num8, 18])));
					obj = Operators.ConcatenateObject(obj, ")");
					Module1.connect(Conversions.ToString(obj));
					num6++;
					if (decimal.Compare(Conversions.ToDecimal(Grid1[num8, 14]), 0m) > 0)
					{
						Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num8, 2]), now, Conversions.ToDecimal(Tcash.Text), Conversions.ToDecimal(Tdebt.Text), "ค\u0e48าห\u0e49อง", Conversions.ToDecimal(Grid1[num8, 14]), "รายการ", sIR_PAY, text, "P001", Conversions.ToDecimal(Grid1[num8, 9]), Conversions.ToDecimal(Grid1[num8, 10]), Conversions.ToDecimal(Grid1[num8, 8]), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
					}
					Module1.connect("update HT_Rooms set room_use='yes' where room_no='" + Conversions.ToString(Grid1[num8, 2]) + "'");
					int num10 = Convert.ToInt32(decimal.Subtract(Conversions.ToDecimal(Grid1[num8, 9]), 1m));
					if (ComboBox1.SelectedIndex == 2 && Operators.CompareString(Conversions.ToString(Grid1[num8, 1]), "", TextCompare: false) != 0)
					{
						num10 = (int)DateAndTime.DateDiff(DateInterval.Day, Conversions.ToDate(Grid1[num8, 4]).Date, Conversions.ToDate(Grid1[num8, 5]).Date);
						if (DateTime.Compare(Conversions.ToDate(Grid1[num8, 4]).Date, Conversions.ToDate(Grid1[num8, 5]).Date) == 0)
						{
							num10 = 1;
						}
						else if (DateTime.Compare(Conversions.ToDate(Grid1[num8, 4]).Date, Conversions.ToDate(Grid1[num8, 5]).Date) != 0 && ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num8, 4]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num8, 4]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0)))
						{
							num10++;
						}
					}
					if (ComboBox1.SelectedIndex == 2)
					{
						DateTime date = Conversions.ToDate(Grid1[num8, 4]);
						DateTime date2 = Conversions.ToDate(Grid1[num8, 5]);
						num10 = (int)DateAndTime.DateDiff(DateInterval.Day, date, date2);
					}
					if (ComboBox1.SelectedIndex == 1)
					{
						num10 = 0;
					}
					int num11 = num10;
					int num12 = 0;
					while (true)
					{
						int num13 = num12;
						num4 = num11;
						if (num13 > num4)
						{
							break;
						}
						DateTime dateTime = Conversions.ToDate(Grid1[num8, 4]);
						if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num8, 4]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num8, 4]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
						{
							dateTime = dateTime.AddDays(-1.0);
						}
						DataSet dataSet = Module1.connect("select * from HT_Room_Status where room_date='" + Conversions.ToString(dateTime.AddDays(num12).Date) + "' and room_no='" + Conversions.ToString(Grid1[num8, 2]) + "'");
						if (dataSet.Tables[0].Rows.Count != 0)
						{
							object left = "update [HT_Room_Status] SET ";
							left = Operators.ConcatenateObject(left, " [room_status]='เข\u0e49าพ\u0e31ก'");
							left = Operators.ConcatenateObject(left, string.Concat(",[room_Details]='" + TcusName.Text, "'"));
							left = Operators.ConcatenateObject(left, string.Concat(",[room_CheckIn_No]='" + TdocNum.Text, "'"));
							left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" where room_date='" + Conversions.ToString(dateTime.AddDays(num12).Date), "' and room_no='"), Conversions.ToString(Grid1[num8, 2])), "'"));
							Module1.connect(Conversions.ToString(left));
						}
						else
						{
							object right = Module1.get_id("HT_Room_Status", "id");
							object left2 = "INSERT INTO [HT_Room_Status]";
							left2 = Operators.ConcatenateObject(left2, "([id]");
							left2 = Operators.ConcatenateObject(left2, ",[room_no]");
							left2 = Operators.ConcatenateObject(left2, ",[room_date]");
							left2 = Operators.ConcatenateObject(left2, ",[room_status]");
							left2 = Operators.ConcatenateObject(left2, ",[room_Details],[room_CheckIn_No],[room_date_oa])");
							left2 = Operators.ConcatenateObject(left2, "VALUES");
							left2 = Operators.ConcatenateObject(left2, "(");
							left2 = Operators.ConcatenateObject(left2, right);
							left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(Grid1[num8, 2]), "'"));
							left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(dateTime.AddDays(num12).Date), "'"));
							left2 = Operators.ConcatenateObject(left2, ",'เข\u0e49าพ\u0e31ก'");
							left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + TcusName.Text, "'"));
							left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + TdocNum.Text, "'"));
							left2 = Operators.ConcatenateObject(left2, "," + Conversions.ToString(dateTime.AddDays(num12).Date.ToOADate()));
							left2 = Operators.ConcatenateObject(left2, ")");
							Module1.connect(Conversions.ToString(left2));
						}
						Module1.GEN_Cupon(Conversions.ToString(Grid1[num8, 2]), TdocNum.Text, dateTime.AddDays(num12 + 1).Date, Convert.ToInt32(Conversions.ToDecimal(Grid1[num8, 18])), AlwayAdd: true);
						num12++;
					}
					num8++;
				}
				int num14 = Grid2.Rows.Count - 1;
				int num15 = 1;
				while (true)
				{
					int num16 = num15;
					int num4 = num14;
					if (num16 > num4 || Operators.CompareString(Conversions.ToString(Grid2[num15, 1]), "", TextCompare: false) == 0)
					{
						break;
					}
					num5 = decimal.Add(num5, Conversions.ToDecimal(Grid2[num15, 14]));
					obj = "INSERT INTO [HT_CheckIn_Product]";
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, " [Cin_No]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_no]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Ds_date]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_id]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_name]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_Unit]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_num]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_price]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_priceTotal]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_pay],[Cin_Pro_note])");
					obj = Operators.ConcatenateObject(obj, "VALUES");
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, string.Concat(" '" + TdocNum.Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num15, 2]), "'"));
					obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid2[num15, 3]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num15, 13]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num15, 4]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num15, 5]), "'"));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num15, 6])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num15, 7])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num15, 8])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid2[num15, 9]), Conversions.ToDecimal(Grid2[num15, 11])), Conversions.ToDecimal(Grid2[num15, 14]))));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num15, 12]), "'"));
					obj = Operators.ConcatenateObject(obj, ")");
					Module1.connect(Conversions.ToString(obj));
					if (decimal.Compare(Conversions.ToDecimal(Grid2[num15, 11]), 0m) > 0)
					{
						Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid2[num15, 2]), now, Conversions.ToDecimal(Tcash.Text), Conversions.ToDecimal(Tdebt.Text), Conversions.ToString(Grid2[num15, 4]), Conversions.ToDecimal(Grid2[num15, 11]), Conversions.ToString(Grid2[num15, 5]), sIR_PAY, text, Conversions.ToString(Grid2[num15, 13]), Conversions.ToDecimal(Grid2[num15, 6]), Conversions.ToDecimal(Grid2[num15, 8]), Conversions.ToDecimal(Grid2[num15, 7]), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
					}
					Module1.connect("update HT_Products set Pro_Amt=Pro_Amt-" + Conversions.ToString(Conversions.ToDecimal(Grid2[num15, 6])) + " where Pro_no='" + Conversions.ToString(Grid2[num15, 13]) + "'");
					num15++;
				}
				int num17 = ListView2.Items.Count - 1;
				int num18 = 0;
				while (true)
				{
					int num19 = num18;
					int num4 = num17;
					if (num19 > num4)
					{
						break;
					}
					obj = "INSERT INTO [HT_CheckIn_Other_People]";
					obj = Operators.ConcatenateObject(obj, "([Cin_no]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_name]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_contry])");
					obj = Operators.ConcatenateObject(obj, "VALUES");
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, string.Concat("'" + TdocNum.Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + ListView2.Items[num18].SubItems[1].Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + ListView2.Items[num18].SubItems[2].Text, "'"));
					obj = Operators.ConcatenateObject(obj, ")");
					Module1.connect(Conversions.ToString(obj));
					num18++;
				}
				if (Operators.CompareString(TbookNo.Text, "", TextCompare: false) != 0)
				{
					Module1.connect("update HT_Book_H set Book_Status='เข\u0e49าพ\u0e31ก' where Book_ID='" + TbookNo.Text + "'");
					Module1.connect("update HT_Rooms set room_book_ds='',room_book='',room_book_name='',room_book_time='' where room_no in (select room_no from View_HT_ROOM where book_no='" + TbookNo.Text + "')");
				}
				obj = "INSERT INTO [HT_CheckIn_H]";
				obj = Operators.ConcatenateObject(obj, "(");
				obj = Operators.ConcatenateObject(obj, "[Cin_no]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Date]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Book_no]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_cust_no]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_cust_price]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_status]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Room]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Product]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Net]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Pay]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Balance]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Car_type]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Car_id]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Room_ALL]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_by]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Date_in]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Date_Out],[Cin_Type],[Cin_foreign]");
				obj = Operators.ConcatenateObject(obj, ")");
				obj = Operators.ConcatenateObject(obj, "VALUES");
				obj = Operators.ConcatenateObject(obj, "(");
				obj = Operators.ConcatenateObject(obj, string.Concat("'" + TdocNum.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(DateTimePicker1.Value), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TbookNo.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TCusType.Text, "'"));
				obj = Operators.ConcatenateObject(obj, ",'ปกต\u0e34'");
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(LabelTroom.Text)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(LabelTpro.Text)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Labelroompro.Text)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(LabelPayed.Text), Conversions.ToDecimal(Tpay.Text)), num5)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Subtract(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Labelroompro.Text), Conversions.ToDecimal(LabelPayed.Text)), Conversions.ToDecimal(Tpay.Text)), num5)));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TCarType.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TCarID.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text2, "'"));
				obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Module1.loginName), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Tstart.Value), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Tend.Value), "'"));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(ComboBox1.SelectedIndex));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(CheckBox1.Checked), "'"));
				obj = Operators.ConcatenateObject(obj, ")");
				Module1.connect(Conversions.ToString(obj));
				Module1.UPDATE_MONEY(text, num5, "DEL", "ต\u0e31ดจากใบลงทะเบ\u0e35ยน " + TdocNum.Text);
				object obj2 = "";
				if (decimal.Compare(num5, 0m) > 0)
				{
					obj2 = Module1.GetSIR_PAY();
					decimal num20 = default(decimal);
					int num21 = Grid1.Rows.Count - 1;
					int num22 = 1;
					while (true)
					{
						int num23 = num22;
						int num4 = num21;
						if (num23 > num4)
						{
							break;
						}
						num20 = decimal.Add(num20, Conversions.ToDecimal(Grid1[num22, 11]));
						num22++;
					}
					int num24 = Grid2.Rows.Count - 1;
					int num25 = 1;
					while (true)
					{
						int num26 = num25;
						int num4 = num24;
						if (num26 > num4)
						{
							break;
						}
						num20 = decimal.Add(num20, Conversions.ToDecimal(Grid2[num25, 14]));
						num25++;
					}
					string text5 = "";
					if (Operators.CompareString(TbookNo.Text, "", TextCompare: false) != 0)
					{
						text5 = "Booking No:" + TbookNo.Text + " ";
					}
					int num27 = Grid1.Rows.Count - 1;
					int num28 = 1;
					while (true)
					{
						int num29 = num28;
						int num4 = num27;
						if (num29 > num4)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid1[num28, 1]), "", TextCompare: false) != 0 && decimal.Compare(Conversions.ToDecimal(Grid1[num28, 11]), 0m) > 0)
						{
							Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num28, 2]), now, 0m, 0m, "ต\u0e31ดยอดล\u0e48วงหน\u0e49า " + text5, Conversions.ToDecimal(Grid1[num28, 11]), "รายการ", Conversions.ToString(obj2), text, "P001", Conversions.ToDecimal(Grid1[num28, 9]), Conversions.ToDecimal(Grid1[num28, 10]), Conversions.ToDecimal(Grid1[num28, 8]), "จ\u0e48ายล\u0e48วงหน\u0e49า", num20, 0m, 0m);
						}
						num28++;
					}
					int num30 = Grid2.Rows.Count - 1;
					int num31 = 1;
					while (true)
					{
						int num32 = num31;
						int num4 = num30;
						if (num32 > num4)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid2[num31, 1]), "", TextCompare: false) != 0)
						{
							num5 = decimal.Add(num5, Conversions.ToDecimal(Grid2[num31, 14]));
							if (decimal.Compare(Conversions.ToDecimal(Grid2[num31, 14]), 0m) > 0)
							{
								Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid2[num31, 2]), now, 0m, 0m, "ต\u0e31ดยอดล\u0e48วงหน\u0e49า " + Conversions.ToString(Grid2[num31, 4]), Conversions.ToDecimal(Grid2[num31, 14]), Conversions.ToString(Grid2[num31, 5]), Conversions.ToString(obj2), text, Conversions.ToString(Grid2[num31, 13]), Conversions.ToDecimal(Grid2[num31, 6]), Conversions.ToDecimal(Grid2[num31, 8]), Conversions.ToDecimal(Grid2[num31, 7]), "จ\u0e48ายล\u0e48วงหน\u0e49า", num20, 0m, 0m);
							}
						}
						num31++;
					}
				}
				Button_REG.Enabled = true;
				Check_Deposit();
				decimal num33 = default(decimal);
				string text6 = "";
				MyProject.Forms.FormShowDEP_0.ISPRINT = false;
				if (Button_DEP.Enabled)
				{
					int num34 = Grid1.Rows.Count - 1;
					int num35 = 1;
					while (true)
					{
						int num36 = num35;
						int num4 = num34;
						if (num36 > num4)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid1[num35, 1]), "", TextCompare: false) != 0)
						{
							if ((decimal.Compare(Conversions.ToDecimal(Grid1[num35, 7]), 0m) != 0) & (Operators.CompareString(Conversions.ToString(Grid1[num35, 17]), "", TextCompare: false) == 0))
							{
								num33 = decimal.Add(num33, Conversions.ToDecimal(Grid1[num35, 7]));
							}
							if (Operators.CompareString(Conversions.ToString(Grid1[num35, 17]), "", TextCompare: false) != 0)
							{
								text6 = ((Operators.CompareString(text6, "", TextCompare: false) != 0) ? (text6 + "," + Conversions.ToString(Grid1[num35, 17])) : Conversions.ToString(Grid1[num35, 17]));
							}
						}
						num35++;
					}
					if (decimal.Compare(num33, 0m) > 0)
					{
						MyProject.Forms.FormShowDEP_0.DEPPRICE.Text = Strings.Format(num33, "#,##0.00");
						MyProject.Forms.FormShowDEP_0.ShowDialog();
					}
				}
				if (Operators.CompareString(Module1.Cin_Print, "เป\u0e34ด", TextCompare: false) == 0)
				{
					Print_Report.Print_Reg(TdocNum.Text, preview: false);
				}
				if (MyProject.Forms.FormShowDEP_0.ISPRINT)
				{
					Print_Report.Print_Dep(TdocNum.Text, preview: false, text6);
				}
				if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
				{
					Print_Report.Print_Sale(sIR_PAY, preview: false);
				}
				if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0 && Operators.ConditionalCompareObjectNotEqual(obj2, "", TextCompare: false) && Operators.ConditionalCompareObjectNotEqual(sIR_PAY, obj2, TextCompare: false))
				{
					Print_Report.Print_Sale(Conversions.ToString(obj2), preview: false);
				}
				if (Operators.CompareString(Module1.Cupon_Report, "เป\u0e34ด", TextCompare: false) == 0)
				{
					Print_Report.Print_coupon_from_no(TdocNum.Text);
				}
				EDIT_ID = TdocNum.Text;
				MessageBox.Show("Check-In เสร\u0e47จเร\u0e35ยบร\u0e49อย", "สถานะ", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
				Module1.IsListroom = true;
				Close();
			}
			else
			{
				Button7.Enabled = true;
			}
		}
	}

	public void SAVE_EDIT()
	{
		DataSet dataSet = Module1.connect("select Cin_Work_number from HT_CheckIn_H where Cin_no='" + EDIT_ID + "'");
		checked
		{
			if (Operators.ConditionalCompareObjectNotEqual(WORK_ID, dataSet.Tables[0].Rows[0]["Cin_Work_number"], TextCompare: false))
			{
				MessageBox.Show("ม\u0e35การแก\u0e49ไข รายการใบลงทะเบ\u0e35ยน  " + EDIT_ID + " จากเคร\u0e37\u0e48องอ\u0e37\u0e48น กร\u0e38ณาป\u0e34ดแล\u0e49วเข\u0e49ามาทำรายการใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				Close();
			}
			else if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) != 0)
			{
				MessageBox.Show("ไม\u0e48สามารถจ\u0e48ายเง\u0e34นหน\u0e49าแก\u0e49ไขได\u0e49 ต\u0e49องไปจ\u0e48ายหน\u0e49าชำระเง\u0e34น", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				Close();
			}
			else if (MessageBox.Show("ค\u0e38ณต\u0e49องการแก\u0e49ไขหร\u0e37อไม\u0e48", "แก\u0e49ไข", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
			{
				if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) > 0)
				{
					MyProject.Forms.FormConfirmPay.PTOTAl = Conversions.ToDecimal(Tpay.Text);
					MyProject.Forms.FormConfirmPay.ShowDialog();
					Tcash.Text = Conversions.ToString(MyProject.Forms.FormConfirmPay.PCASH);
					Tdebt.Text = Conversions.ToString(MyProject.Forms.FormConfirmPay.PCREDIT);
					if (!MyProject.Forms.FormConfirmPay.ISOK)
					{
						Button7.Enabled = true;
						return;
					}
				}
				string text = "";
				if (Operators.CompareString(TCusID.Text, "", TextCompare: false) != 0)
				{
					EDIT_CUST();
					text = TCusID.Text;
				}
				else
				{
					text = SAVE_CUST();
				}
				DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Product where cin_no='" + EDIT_ID + "'");
				int num = dataSet2.Tables[0].Rows.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Products set Pro_Amt=Pro_Amt+", dataSet2.Tables[0].Rows[num2]["cin_pro_num"]), " where Pro_no='"), dataSet2.Tables[0].Rows[num2]["cin_pro_id"]), "'")));
					num2++;
				}
				DateTime now = DateTime.Now;
				int num5 = Module1.get_id("HT_CheckIn_Ds", "id");
				Module1.connect("update HT_Rooms set room_use='no' where room_no in (select Cin_Room_No from HT_CheckIn_Ds where Cin_no='" + TdocNum.Text + "' and Cin_Room_Status <> 'Check-Out')");
				Module1.connect("delete from HT_Room_Status where room_CheckIn_No='" + TdocNum.Text + "'");
				Module1.connect("delete from HT_CheckIn_H where Cin_no='" + TdocNum.Text + "'");
				Module1.connect("delete from HT_CheckIn_Ds where Cin_no='" + TdocNum.Text + "' and Cin_Room_Status <> 'Check-Out'");
				Module1.connect("delete from HT_CheckIn_Product where Cin_no='" + TdocNum.Text + "'");
				Module1.connect("update Tb_Save_Image set cin_no='" + TdocNum.Text + "',cust_no='" + text + "',tmp_no='' where tmp_no='" + tmp_no + "'");
				decimal num6 = default(decimal);
				string text2 = "";
				int num7 = TselectRoom.Items.Count - 1;
				int num8 = 0;
				while (true)
				{
					int num9 = num8;
					int num4 = num7;
					if (num9 > num4)
					{
						break;
					}
					text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(TselectRoom.Items[num8], " ")));
					num8++;
				}
				string sIR_PAY = Module1.GetSIR_PAY();
				object obj = "";
				int num10 = Grid1.Rows.Count - 1;
				int num11 = 1;
				while (true)
				{
					int num12 = num11;
					int num4 = num10;
					if (num12 > num4)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid1[num11, 6]), "Check-Out", TextCompare: false) == 0)
					{
						string text3 = "UPDATE HT_CheckIn_Ds SET";
						text3 = text3 + " Cin_room_pay_total=" + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid1[num11, 12]), Conversions.ToDecimal(Grid1[num11, 14])), Conversions.ToDecimal(Grid1[num11, 11])));
						text3 = Conversions.ToString(Operators.ConcatenateObject(text3, Operators.ConcatenateObject(" where id=", Grid1[num11, 17])));
						Module1.connect(text3);
					}
					else
					{
						if (Operators.CompareString(Conversions.ToString(Grid1[num11, 1]), "", TextCompare: false) == 0)
						{
							break;
						}
						string text4 = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
						if (Operators.ConditionalCompareObjectEqual(Grid1[num11, 15], true, TextCompare: false))
						{
							text4 = "เข\u0e49าพ\u0e31ก";
						}
						string text5 = "ย\u0e31งไม\u0e48ค\u0e37นค\u0e48าม\u0e31ดจำ";
						if (decimal.Compare(Conversions.ToDecimal(Grid1[num11, 7]), 0m) <= 0)
						{
							text5 = "ไม\u0e48เก\u0e47บค\u0e48าม\u0e31ดจำ";
						}
						int num13 = num5;
						if (Operators.CompareString(Conversions.ToString(Grid1[num11, 17]), "", TextCompare: false) != 0)
						{
							num13 = Conversions.ToInteger(Grid1[num11, 17]);
						}
						else
						{
							num5++;
						}
						Module1.Power_set(Conversions.ToString(Grid1[num11, 2]), "ON", "", "เป\u0e34ดไฟ อ\u0e31ตโนม\u0e31ต\u0e34 จากแก\u0e49ไขเช\u0e47คอ\u0e34น No." + TdocNum.Text);
						num6 = decimal.Add(num6, Conversions.ToDecimal(Grid1[num11, 11]));
						obj = "INSERT INTO [HT_CheckIn_Ds]";
						obj = Operators.ConcatenateObject(obj, "([id]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_No]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_No]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Type]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_In]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Out]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Status]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Dep]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Price]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Night]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_PriceToTal]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Pay_Before]");
						obj = Operators.ConcatenateObject(obj, ",[Cin_Room_Pay_Total],[Cin_note],[Cin_dep_status],[Cin_cupon])");
						obj = Operators.ConcatenateObject(obj, "VALUES");
						obj = Operators.ConcatenateObject(obj, "(");
						obj = Operators.ConcatenateObject(obj, " " + Conversions.ToString(num13));
						obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TdocNum.Text, "'"));
						obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid1[num11, 2]), "'"));
						obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid1[num11, 3]), "'"));
						obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num11, 4]), "'"));
						obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num11, 5]), "'"));
						obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text4, "'"));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num11, 7])));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num11, 8])));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num11, 9])));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num11, 10])));
						obj = Operators.ConcatenateObject(obj, ",0");
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid1[num11, 12]), Conversions.ToDecimal(Grid1[num11, 14])), Conversions.ToDecimal(Grid1[num11, 11]))));
						obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num11, 16]), "'"));
						obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text5, "'"));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num11, 18])));
						obj = Operators.ConcatenateObject(obj, ")");
						Module1.connect(Conversions.ToString(obj));
						if (decimal.Compare(Conversions.ToDecimal(Grid1[num11, 14]), 0m) > 0)
						{
							Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num11, 2]), now, Conversions.ToDecimal(Tcash.Text), Conversions.ToDecimal(Tdebt.Text), "ค\u0e48าห\u0e49อง", Conversions.ToDecimal(Conversions.ToString(Grid1[num11, 14])), "รายการ", sIR_PAY, text, "P001", Conversions.ToDecimal(Grid1[num11, 9]), Conversions.ToDecimal(Grid1[num11, 10]), Conversions.ToDecimal(Grid1[num11, 8]), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
						}
						Module1.connect("update HT_Rooms set room_use='yes' where room_no='" + Conversions.ToString(Grid1[num11, 2]) + "'");
						int num14 = Convert.ToInt32(decimal.Subtract(Conversions.ToDecimal(Grid1[num11, 9]), 1m));
						if (ComboBox1.SelectedIndex == 2 && Operators.CompareString(Conversions.ToString(Grid1[num11, 1]), "", TextCompare: false) != 0)
						{
							num14 = (int)DateAndTime.DateDiff(DateInterval.Day, Conversions.ToDate(Grid1[num11, 4]).Date, Conversions.ToDate(Grid1[num11, 5]).Date);
							if (DateTime.Compare(Conversions.ToDate(Grid1[num11, 4]).Date, Conversions.ToDate(Grid1[num11, 5]).Date) == 0)
							{
								num14 = 1;
							}
							else if (DateTime.Compare(Conversions.ToDate(Grid1[num11, 4]).Date, Conversions.ToDate(Grid1[num11, 5]).Date) != 0 && ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num11, 4]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num11, 4]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0)))
							{
								num14++;
							}
						}
						if (ComboBox1.SelectedIndex == 2)
						{
							DateTime date = Conversions.ToDate(Grid1[num11, 4]);
							DateTime date2 = Conversions.ToDate(Grid1[num11, 5]);
							num14 = (int)DateAndTime.DateDiff(DateInterval.Day, date, date2);
							Module1.connect("delete from HT_Room_Status where room_no='" + Conversions.ToString(Grid1[num11, 2]) + "'");
						}
						if (ComboBox1.SelectedIndex == 1)
						{
							num14 = 0;
						}
						int num15 = num14;
						int num16 = 0;
						while (true)
						{
							int num17 = num16;
							num4 = num15;
							if (num17 > num4)
							{
								break;
							}
							DateTime dateTime = Conversions.ToDate(Grid1[num11, 4]);
							if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num11, 4]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num11, 4]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
							{
								dateTime = dateTime.AddDays(-1.0);
							}
							DataSet dataSet3 = Module1.connect("select * from HT_Room_Status where room_date='" + Conversions.ToString(dateTime.AddDays(num16).Date) + "' and room_no='" + Conversions.ToString(Grid1[num11, 2]) + "'");
							if (dataSet3.Tables[0].Rows.Count != 0)
							{
								object left = "update [HT_Room_Status] SET ";
								left = Operators.ConcatenateObject(left, " [room_status]='เข\u0e49าพ\u0e31ก'");
								left = Operators.ConcatenateObject(left, string.Concat(",[room_Details]='" + TcusName.Text, "'"));
								left = Operators.ConcatenateObject(left, string.Concat(",[room_CheckIn_No]='" + TdocNum.Text, "'"));
								left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" where room_date='" + Conversions.ToString(dateTime.AddDays(num16).Date), "' and room_no='"), Conversions.ToString(Grid1[num11, 2])), "'"));
								Module1.connect(Conversions.ToString(left));
							}
							else
							{
								object right = Module1.get_id("HT_Room_Status", "id");
								object left2 = "INSERT INTO [HT_Room_Status]";
								left2 = Operators.ConcatenateObject(left2, "([id]");
								left2 = Operators.ConcatenateObject(left2, ",[room_no]");
								left2 = Operators.ConcatenateObject(left2, ",[room_date]");
								left2 = Operators.ConcatenateObject(left2, ",[room_status]");
								left2 = Operators.ConcatenateObject(left2, ",[room_Details],[room_CheckIn_No],[room_date_oa])");
								left2 = Operators.ConcatenateObject(left2, "VALUES");
								left2 = Operators.ConcatenateObject(left2, "(");
								left2 = Operators.ConcatenateObject(left2, right);
								left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(Grid1[num11, 2]), "'"));
								left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(dateTime.AddDays(num16).Date), "'"));
								left2 = Operators.ConcatenateObject(left2, ",'เข\u0e49าพ\u0e31ก'");
								left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + TcusName.Text, "'"));
								left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + TdocNum.Text, "'"));
								left2 = Operators.ConcatenateObject(left2, "," + Conversions.ToString(dateTime.AddDays(num16).Date.ToOADate()));
								left2 = Operators.ConcatenateObject(left2, ")");
								Module1.connect(Conversions.ToString(left2));
							}
							Module1.GEN_Cupon(Conversions.ToString(Grid1[num11, 2]), TdocNum.Text, dateTime.AddDays(num16 + 1).Date, Convert.ToInt32(Conversions.ToDecimal(Grid1[num11, 18])), AlwayAdd: false);
							num16++;
						}
					}
					num11++;
				}
				int num18 = Grid2.Rows.Count - 1;
				int num19 = 1;
				while (true)
				{
					int num20 = num19;
					int num4 = num18;
					if (num20 > num4 || Operators.CompareString(Conversions.ToString(Grid2[num19, 1]), "", TextCompare: false) == 0)
					{
						break;
					}
					num6 = decimal.Add(num6, Conversions.ToDecimal(Grid2[num19, 14]));
					obj = "INSERT INTO [HT_CheckIn_Product]";
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, " [Cin_No]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Room_no]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Ds_date]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_id]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_name]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_Unit]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_num]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_price]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_priceTotal]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_Pro_pay],[Cin_Pro_note])");
					obj = Operators.ConcatenateObject(obj, "VALUES");
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, string.Concat(" '" + TdocNum.Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num19, 2]), "'"));
					obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid2[num19, 3]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num19, 13]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num19, 4]), "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num19, 5]), "'"));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num19, 6])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num19, 7])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num19, 8])));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid2[num19, 9]), Conversions.ToDecimal(Grid2[num19, 11])), Conversions.ToDecimal(Grid2[num19, 14]))));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num19, 12]), "'"));
					obj = Operators.ConcatenateObject(obj, ")");
					Module1.connect(Conversions.ToString(obj));
					if (decimal.Compare(Conversions.ToDecimal(Grid2[num19, 11]), 0m) > 0)
					{
						Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid2[num19, 2]), now, Conversions.ToDecimal(Tcash.Text), Conversions.ToDecimal(Tdebt.Text), Conversions.ToString(Grid2[num19, 4]), Conversions.ToDecimal(Conversions.ToString(Grid2[num19, 11])), Conversions.ToString(Grid2[num19, 5]), sIR_PAY, text, Conversions.ToString(Grid2[num19, 13]), Conversions.ToDecimal(Grid2[num19, 6]), Conversions.ToDecimal(Grid2[num19, 8]), Conversions.ToDecimal(Grid2[num19, 7]), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
					}
					Module1.connect("update HT_Products set Pro_Amt=Pro_Amt-" + Conversions.ToString(Conversions.ToDecimal(Grid2[num19, 6])) + " where Pro_no='" + Conversions.ToString(Grid2[num19, 13]) + "'");
					num19++;
				}
				Module1.connect("delete from HT_CheckIn_Other_People where Cin_no='" + TdocNum.Text + "'");
				int num21 = ListView2.Items.Count - 1;
				int num22 = 0;
				while (true)
				{
					int num23 = num22;
					int num4 = num21;
					if (num23 > num4)
					{
						break;
					}
					obj = "INSERT INTO [HT_CheckIn_Other_People]";
					obj = Operators.ConcatenateObject(obj, "([Cin_no]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_name]");
					obj = Operators.ConcatenateObject(obj, ",[Cin_contry])");
					obj = Operators.ConcatenateObject(obj, "VALUES");
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, string.Concat("'" + TdocNum.Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + ListView2.Items[num22].SubItems[1].Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + ListView2.Items[num22].SubItems[2].Text, "'"));
					obj = Operators.ConcatenateObject(obj, ")");
					Module1.connect(Conversions.ToString(obj));
					num22++;
				}
				if (Operators.CompareString(TbookNo.Text, "", TextCompare: false) != 0)
				{
					Module1.connect("update HT_Book_H set Book_Status='เข\u0e49าพ\u0e31ก' where Book_ID='" + TbookNo.Text + "'");
					Module1.connect("update HT_Rooms set room_book_ds='',room_book='',room_book_name='',room_book_time='' where room_no in (select room_no from View_HT_ROOM where book_no='" + TbookNo.Text + "')");
				}
				obj = "INSERT INTO [HT_CheckIn_H]";
				obj = Operators.ConcatenateObject(obj, "(");
				obj = Operators.ConcatenateObject(obj, "[Cin_no]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Date]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Book_no]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_cust_no]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_cust_price]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_status]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Room]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Product]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Net]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Pay]");
				obj = Operators.ConcatenateObject(obj, ",[Total_Price_Balance]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Car_type]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Car_id]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Room_ALL]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Date_in]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Date_Out]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_Type]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_by]");
				obj = Operators.ConcatenateObject(obj, ",[Cin_foreign]");
				obj = Operators.ConcatenateObject(obj, " )");
				obj = Operators.ConcatenateObject(obj, "VALUES");
				obj = Operators.ConcatenateObject(obj, "(");
				obj = Operators.ConcatenateObject(obj, string.Concat("'" + TdocNum.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(DateTimePicker1.Value), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TbookNo.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TCusType.Text, "'"));
				obj = Operators.ConcatenateObject(obj, ",'ปกต\u0e34'");
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(LabelTroom.Text)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(LabelTpro.Text)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Labelroompro.Text)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(LabelPayed.Text), Conversions.ToDecimal(Tpay.Text)), num6)));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Subtract(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Labelroompro.Text), Conversions.ToDecimal(LabelPayed.Text)), Conversions.ToDecimal(Tpay.Text)), num6)));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TCarType.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TCarID.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + text2, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Tstart.Value), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Tend.Value), "'"));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(ComboBox1.SelectedIndex));
				obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Module1.loginName), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(CheckBox1.Checked), "'"));
				obj = Operators.ConcatenateObject(obj, ")");
				Module1.connect(Conversions.ToString(obj));
				Module1.UPDATE_MONEY(text, num6, "DEL", "ต\u0e31ดจากใบลงทะเบ\u0e35ยน " + TdocNum.Text);
				object obj2 = "";
				if (decimal.Compare(num6, 0m) > 0)
				{
					obj2 = Module1.GetSIR_PAY();
					decimal num24 = default(decimal);
					int num25 = Grid1.Rows.Count - 1;
					int num26 = 1;
					while (true)
					{
						int num27 = num26;
						int num4 = num25;
						if (num27 > num4)
						{
							break;
						}
						num24 = decimal.Add(num24, Conversions.ToDecimal(Grid1[num26, 11]));
						num26++;
					}
					int num28 = Grid2.Rows.Count - 1;
					int num29 = 1;
					while (true)
					{
						int num30 = num29;
						int num4 = num28;
						if (num30 > num4)
						{
							break;
						}
						num24 = decimal.Add(num24, Conversions.ToDecimal(Grid2[num29, 14]));
						num29++;
					}
					string text6 = "";
					if (Operators.CompareString(TbookNo.Text, "", TextCompare: false) != 0)
					{
						text6 = "Booking No:" + TbookNo.Text + " ";
					}
					int num31 = Grid1.Rows.Count - 1;
					int num32 = 1;
					while (true)
					{
						int num33 = num32;
						int num4 = num31;
						if (num33 > num4)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid1[num32, 1]), "", TextCompare: false) != 0 && decimal.Compare(Conversions.ToDecimal(Grid1[num32, 11]), 0m) > 0)
						{
							Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num32, 2]), now, 0m, 0m, "ต\u0e31ดยอดล\u0e48วงหน\u0e49า " + text6, Conversions.ToDecimal(Grid1[num32, 11]), "รายการ", Conversions.ToString(obj2), text, "P001", Conversions.ToDecimal(Grid1[num32, 9]), Conversions.ToDecimal(Grid1[num32, 10]), Conversions.ToDecimal(Grid1[num32, 8]), "จ\u0e48ายล\u0e48วงหน\u0e49า", num24, 0m, 0m);
						}
						num32++;
					}
					int num34 = Grid2.Rows.Count - 1;
					int num35 = 1;
					while (true)
					{
						int num36 = num35;
						int num4 = num34;
						if (num36 > num4)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid2[num35, 1]), "", TextCompare: false) != 0)
						{
							num6 = decimal.Add(num6, Conversions.ToDecimal(Grid2[num35, 14]));
							if (decimal.Compare(Conversions.ToDecimal(Grid2[num35, 14]), 0m) > 0)
							{
								Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid2[num35, 2]), now, 0m, 0m, "ต\u0e31ดยอดล\u0e48วงหน\u0e49า " + Conversions.ToString(Grid2[num35, 4]), Conversions.ToDecimal(Grid2[num35, 14]), Conversions.ToString(Grid2[num35, 5]), Conversions.ToString(obj2), text, Conversions.ToString(Grid2[num35, 13]), Conversions.ToDecimal(Grid2[num35, 6]), Conversions.ToDecimal(Grid2[num35, 8]), Conversions.ToDecimal(Grid2[num35, 7]), "จ\u0e48ายล\u0e48วงหน\u0e49า", num24, 0m, 0m);
							}
						}
						num35++;
					}
				}
				MessageBox.Show("บ\u0e31นท\u0e36กแก\u0e49ไขเสร\u0e47จเร\u0e35ยบร\u0e49อย");
				if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
				{
					Print_Report.Print_Sale(sIR_PAY, preview: false);
				}
				if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0 && Operators.ConditionalCompareObjectNotEqual(obj2, "", TextCompare: false) && Operators.ConditionalCompareObjectNotEqual(sIR_PAY, obj2, TextCompare: false))
				{
					Print_Report.Print_Sale(Conversions.ToString(obj2), preview: false);
				}
				Button_REG.Enabled = true;
				Check_Deposit();
				if (Button_DEP.Enabled)
				{
					decimal num37 = default(decimal);
					string text7 = "";
					int num38 = Grid1.Rows.Count - 1;
					int num39 = 1;
					while (true)
					{
						int num40 = num39;
						int num4 = num38;
						if (num40 > num4)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid1[num39, 1]), "", TextCompare: false) != 0)
						{
							if ((decimal.Compare(Conversions.ToDecimal(Grid1[num39, 7]), 0m) != 0) & (Operators.CompareString(Conversions.ToString(Grid1[num39, 17]), "", TextCompare: false) == 0))
							{
								num37 = decimal.Add(num37, Conversions.ToDecimal(Grid1[num39, 7]));
							}
							if (Operators.CompareString(Conversions.ToString(Grid1[num39, 17]), "", TextCompare: false) != 0)
							{
								text7 = ((Operators.CompareString(text7, "", TextCompare: false) != 0) ? (text7 + "," + Conversions.ToString(Grid1[num39, 17])) : Conversions.ToString(Grid1[num39, 17]));
							}
						}
						num39++;
					}
					if (decimal.Compare(num37, 0m) > 0)
					{
						MyProject.Forms.FormShowDEP_0.DEPPRICE.Text = Strings.Format(num37, "#,##0.00");
						MyProject.Forms.FormShowDEP_0.ShowDialog();
						if (MyProject.Forms.FormShowDEP_0.ISPRINT)
						{
							Print_Report.Print_Dep(TdocNum.Text, preview: false, text7);
						}
					}
				}
				EDIT_ID = TdocNum.Text;
				LoadBill();
				Button7.Enabled = true;
				Module1.IsListroom = true;
				Close();
			}
			else
			{
				Button7.Enabled = true;
			}
		}
	}

	private void ListView1_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return && ListView1.SelectedItems.Count != 0)
		{
			DataSet dataSet = Module1.connect("select * from HT_Customers where Cust_no ='" + ListView1.SelectedItems[0].SubItems[0].Text + "'");
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				TextBox_0.Text = "";
				TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_no"]);
				TcusName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				TextBox_2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				TCusTypeMain.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type_Main"]);
				TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				TextBox_1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Email"]);
				Tcusperfix.Text = dataSet.Tables[0].Rows[0]["Cust_perfix"].ToString();
				TcusSex.Text = dataSet.Tables[0].Rows[0]["Cust_sex"].ToString();
				TcusCardID.Text = dataSet.Tables[0].Rows[0]["Cust_IDcard"].ToString();
				Tc_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_no"]);
				Tc_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_moo"]);
				Tc_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_soi"]);
				Tc_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_road"]);
				Tc_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tambon"]);
				Tc_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_ampore"]);
				Tc_province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_province"]);
				Tc_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_code"]);
				Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				TextBox_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				Tc_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				Tw.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"]);
				Tw_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_no"]);
				Tw_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_moo"]);
				Tw_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_soi"]);
				Tw_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_road"]);
				Tw_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tambon"]);
				Tw_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_ampore"]);
				Tw_privince.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_province"]);
				Tw_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_code"]);
				Tw_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"]);
				Tw_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_fax"]);
				TwTax.Text = dataSet.Tables[0].Rows[0]["Cust_Work_tax"].ToString();
				Tover.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Price_Over"]);
				Tcontry.Text = dataSet.Tables[0].Rows[0]["Cust_contry"].ToString();
				method_0();
				RefreshScan();
				PanelCust.Visible = false;
				Button3.Focus();
				Refresh_Dep_auto(0m);
			}
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormSearchChechIn.ShowDialog();
		if (Operators.ConditionalCompareObjectNotEqual(MyProject.Forms.FormSearchChechIn.SelectNO, "", TextCompare: false))
		{
			EDIT_ID = Conversions.ToString(MyProject.Forms.FormSearchChechIn.SelectNO);
			LoadBill();
		}
	}

	private void Button8_Click(object sender, EventArgs e)
	{
		Print_Report.Print_Reg(EDIT_ID, preview: false);
	}

	private void Button10_Click(object sender, EventArgs e)
	{
		Print_Report.Print_Dep(EDIT_ID, preview: false);
	}

	private void LabelButton7_Click(object sender, EventArgs e)
	{
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		CheckIN_From_Booking();
	}

	public void CheckIN_From_Booking()
	{
		if (Operators.CompareString(EDIT_ID, "", TextCompare: false) != 0 || Operators.CompareString(TbookNo.Text, "", TextCompare: false) == 0)
		{
			return;
		}
		isbook = true;
		DataSet dataSet = Module1.connect("select * from HT_Book_H where Book_ID='" + TbookNo.Text + "'");
		DataSet dataSet2 = Module1.connect("select * from View_HT_ROOM where Book_no='" + TbookNo.Text + "' order by room_no");
		DataSet dataSet3 = Module1.connect("select * from HT_Book_Pro where B_no='" + TbookNo.Text + "' order by id");
		DataSet dataSet4 = Module1.connect("select * from HT_Book_Ds where book_no='" + TbookNo.Text + "' ");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("ไม\u0e48พบหมายเลขการจองเลขท\u0e35\u0e48 " + TbookNo.Text);
			Clear();
			return;
		}
		if (dataSet2.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("หมายเลขการจองเลขท\u0e35\u0e48 " + TbookNo.Text + " ย\u0e31งไม\u0e48ได\u0e49จ\u0e31ดห\u0e49องพ\u0e31ก กร\u0e38ณาจ\u0e31ดการลงห\u0e49องพ\u0e31กก\u0e48อน ในหน\u0e49า สถานะห\u0e49องพ\u0e31ก");
			Clear();
			return;
		}
		Clear();
		TbookNo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_ID"]);
		TcusName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_Name"]);
		TextBox_2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_Name2"]);
		Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_tel"]);
		TextBox_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_tel"]);
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["book_cust_id"], "", TextCompare: false))
		{
			DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Customers where Cust_no ='", dataSet.Tables[0].Rows[0]["book_cust_id"]), "'")));
			if (dataSet5.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ไม\u0e48พบรห\u0e31สล\u0e39กค\u0e49า ", dataSet.Tables[0].Rows[0]["book_cust_id"]), " กร\u0e38ณากล\u0e31บไปแก\u0e49ไขใบจอง แล\u0e49วเล\u0e37อกล\u0e39กค\u0e49ามาลงใบจองใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand, MessageBoxDefaultButton.Button1);
				Close();
				return;
			}
			Booking_cust = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_no"]);
			TextBox_0.Text = "";
			TCusID.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_no"]);
			TcusName.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_name"]);
			TextBox_2.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_name2"]);
			TCusTypeMain.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Type_main"]);
			TCusType.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Type"]);
			TextBox_1.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Email"]);
			Tc_no.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_no"]);
			Tc_moo.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_moo"]);
			Tc_soi.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_soi"]);
			Tc_road.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_road"]);
			Tc_tambon.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_tambon"]);
			Tc_ampore.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_ampore"]);
			Tc_province.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_province"]);
			Tc_code.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_code"]);
			Tc_tel.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_tel"]);
			TextBox_0.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_tel"]);
			Tc_fax.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Add_fax"]);
			Tw.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_Name"]);
			Tw_no.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_no"]);
			Tw_moo.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_moo"]);
			Tw_soi.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_soi"]);
			Tw_road.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_road"]);
			Tw_tambon.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_tambon"]);
			Tw_ampore.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_ampore"]);
			Tw_privince.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_province"]);
			Tw_code.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_code"]);
			Tw_tel.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_tel"]);
			Tw_fax.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Work_fax"]);
			TwTax.Text = dataSet5.Tables[0].Rows[0]["Cust_Work_tax"].ToString();
			Tover.Text = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cust_Price_Over"]);
			if (Operators.ConditionalCompareObjectNotEqual(dataSet5.Tables[0].Rows[0]["Cust_Price_Over"], 0, TextCompare: false))
			{
				Label_0.Visible = true;
				TextBox_0.Enabled = false;
			}
			else
			{
				Label_0.Visible = false;
				TextBox_0.Enabled = true;
			}
		}
		if (Operators.CompareString(Tc_tel.Text, "", TextCompare: false) == 0)
		{
			Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_tel"]);
			TextBox_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_tel"]);
		}
		Tend.Value = Conversions.ToDate(dataSet.Tables[0].Rows[0]["Book_Date_out"]);
		Conversions.ToDecimal(Tover.Text);
		checked
		{
			int num = dataSet2.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					DataSet dataSet6 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", dataSet2.Tables[0].Rows[num2]["room_no"]), "'")));
					if (dataSet6.Tables[0].Rows.Count != 0)
					{
						DataSet dataSet7 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_Book_Date where id=", dataSet6.Tables[0].Rows[0]["Room_book"])));
						DataSet dataSet8 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Book_Ds where book_no='", dataSet7.Tables[0].Rows[0]["book_no"]), "' and book_room_type='"), dataSet7.Tables[0].Rows[0]["book_type"]), "'")));
						if (dataSet8.Tables[0].Rows.Count == 0)
						{
							MessageBox.Show("ราคาห\u0e49องย\u0e31งไม\u0e48ได\u0e49ต\u0e31\u0e49งราคา กร\u0e38ณาไปต\u0e31\u0e49งราคาก\u0e48อน");
							break;
						}
						if (dataSet8.Tables[0].Rows.Count != 0)
						{
							int num5 = 1;
							int num6 = Grid1.Rows.Count - 1;
							int num7 = 0;
							while (true)
							{
								int num8 = num7;
								num4 = num6;
								if (num8 <= num4)
								{
									if (Operators.CompareString(Conversions.ToString(Grid1[num7, 1]), "", TextCompare: false) != 0)
									{
										num7++;
										continue;
									}
									num5 = num7;
									break;
								}
								break;
							}
							Tend.Value = Conversions.ToDate(dataSet8.Tables[0].Rows[0]["Book_room_end"]);
							decimal num9 = decimal.Multiply(Conversions.ToDecimal(dataSet8.Tables[0].Rows[0]["book_room_price"]), Conversions.ToDecimal(Tnum.Text));
							int num10 = 0;
							DataSet dataSet9 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_SET_RoomType where name='", dataSet6.Tables[0].Rows[0]["Room_Type"]), "'")));
							if (dataSet9.Tables[0].Rows.Count != 0 && Operators.CompareString(dataSet9.Tables[0].Rows[0]["Room_priceA"].ToString(), "", TextCompare: false) != 0)
							{
								if (Operators.ConditionalCompareObjectEqual(dataSet9.Tables[0].Rows[0]["Room_priceA"], 0, TextCompare: false))
								{
									num10 = 0;
								}
								else if (Operators.ConditionalCompareObjectGreater(dataSet9.Tables[0].Rows[0]["Room_priceA"], 0, TextCompare: false))
								{
									num10 = 2;
								}
							}
							DateTime value = Tstart.Value;
							DateTime value2 = Tend.Value;
							int num11 = dataSet4.Tables[0].Rows.Count - 1;
							int num12 = 0;
							while (true)
							{
								int num13 = num12;
								num4 = num11;
								if (num13 > num4)
								{
									break;
								}
								if (Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num12]["book_room_type"], dataSet2.Tables[0].Rows[num2]["room_no"], TextCompare: false))
								{
									Tend.Value = Conversions.ToDate(dataSet4.Tables[0].Rows[num12]["book_room_end"]);
								}
								num12++;
							}
							Grid1[num5, 1] = num5;
							Grid1[num5, 2] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["room_no"]);
							Grid1[num5, 3] = RuntimeHelpers.GetObjectValue(dataSet6.Tables[0].Rows[0]["Room_Type"]);
							Grid1[num5, 4] = Tstart.Value;
							Grid1[num5, 5] = Tend.Value;
							Grid1[num5, 6] = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
							Grid1[num5, 7] = Dep_price;
							Grid1[num5, 8] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet8.Tables[0].Rows[0]["book_room_price"]), "#,##0.00");
							Grid1[num5, 9] = Tnum.Text;
							Grid1[num5, 10] = Strings.Format(num9, "#,##0.00");
							Grid1[num5, 11] = Strings.Format(0, "#,##0.00");
							Grid1[num5, 12] = "0.00";
							Grid1[num5, 13] = Strings.Format(num9, "#,##0.00");
							Grid1[num5, 14] = "0.00";
							Grid1[num5, 15] = true;
							Grid1[num5, 18] = num10;
							AddItemInCombobox();
							Tstart.Value = value;
							Tend.Value = value2;
						}
					}
					num2++;
					continue;
				}
				DataSet dataSet10 = Module1.connect("select id,pro_no from HT_Products");
				int num14 = dataSet3.Tables[0].Rows.Count - 1;
				int num15 = 0;
				while (true)
				{
					int num16 = num15;
					num4 = num14;
					if (num16 > num4)
					{
						break;
					}
					string value3 = "-1";
					int num17 = dataSet10.Tables[0].Rows.Count - 1;
					int num18 = 0;
					while (true)
					{
						int num19 = num18;
						num4 = num17;
						if (num19 <= num4)
						{
							if (!Operators.ConditionalCompareObjectEqual(dataSet10.Tables[0].Rows[num18]["id"], dataSet3.Tables[0].Rows[num15]["B_PRO_ID"], TextCompare: false))
							{
								num18++;
								continue;
							}
							value3 = Conversions.ToString(dataSet10.Tables[0].Rows[num18]["pro_no"]);
							break;
						}
						break;
					}
					Grid2[num15 + 1, 1] = num15 + 1;
					Grid2[num15 + 1, 2] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_ROOM"]);
					Grid2[num15 + 1, 3] = DateTime.Now;
					Grid2[num15 + 1, 4] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_NAME"]);
					Grid2[num15 + 1, 5] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_UNIT"]);
					Grid2[num15 + 1, 6] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_NUM"]);
					Grid2[num15 + 1, 7] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_PRICE"]), "#,##0.00");
					Grid2[num15 + 1, 8] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_PRICE_TOTAL"]), "#,##0.00");
					Grid2[num15 + 1, 9] = "0.00";
					Grid2[num15 + 1, 10] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num15]["B_PRICE_TOTAL"]), "#,##0.00");
					Grid2[num15 + 1, 11] = "0.00";
					Grid2[num15 + 1, 12] = "";
					Grid2[num15 + 1, 13] = value3;
					Grid2[num15 + 1, 14] = "0.00";
					num15++;
				}
				sum();
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["book_price_pay"], 0, TextCompare: false))
				{
					if (Operators.ConditionalCompareObjectGreaterEqual(Conversions.ToDecimal(Tover.Text), dataSet.Tables[0].Rows[0]["book_price_pay"], TextCompare: false))
					{
						Refresh_Dep_auto(Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["book_price_pay"]));
					}
					else if (decimal.Compare(Conversions.ToDecimal(Tover.Text), 0m) > 0)
					{
						Refresh_Dep_auto(Conversions.ToDecimal(Tover.Text));
					}
				}
				break;
			}
		}
	}

	private void TCusTypeMain_SelectedIndexChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Order_Up where cast_type='" + TCusTypeMain.Text + "' order by id");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["cust_type"]);
		}
	}

	private void expandablePanel4_ExpandedChanged(object sender, ExpandedChangeEventArgs e)
	{
		if (e.NewExpandedValue)
		{
			ExpandablePanel1.Expanded = false;
			expandablePanel5.Expanded = false;
		}
		if (!ExpandablePanel1.Expanded & !expandablePanel4.Expanded & !expandablePanel5.Expanded)
		{
			ExpandablePanel1.Expanded = true;
		}
	}

	private void ExpandablePanel1_ExpandedChanged(object sender, ExpandedChangeEventArgs e)
	{
		if (e.NewExpandedValue)
		{
			expandablePanel4.Expanded = false;
			expandablePanel5.Expanded = false;
		}
		if (!ExpandablePanel1.Expanded & !expandablePanel4.Expanded & !expandablePanel5.Expanded)
		{
			ExpandablePanel1.Expanded = true;
		}
	}

	private void expandablePanel5_ExpandedChanged(object sender, ExpandedChangeEventArgs e)
	{
		if (e.NewExpandedValue)
		{
			ExpandablePanel1.Expanded = false;
			expandablePanel4.Expanded = false;
		}
		if (!ExpandablePanel1.Expanded & !expandablePanel4.Expanded & !expandablePanel5.Expanded)
		{
			ExpandablePanel1.Expanded = true;
		}
	}

	private void expandablePanel4_Click(object sender, EventArgs e)
	{
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		FrmAddSaveImage frmAddSaveImage = new FrmAddSaveImage();
		frmAddSaveImage.TopMost = true;
		frmAddSaveImage.Temp_no = tmp_no;
		frmAddSaveImage.Tname.Text = TdocNum.Text;
		frmAddSaveImage.cust_no = "";
		frmAddSaveImage.ShowDialog();
		RefreshScan();
	}

	public void ShowScan(object sender, EventArgs e)
	{
		GForm0 gForm = new GForm0();
		gForm.showID = Conversions.ToInteger(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		gForm.ShowDialog();
		RefreshScan();
	}

	public void RefreshScan()
	{
		ItemPanel1.BeginUpdate();
		ItemPanel1.Items.Clear();
		string text = "AS999999D";
		if (Operators.CompareString(TCusID.Text, "", TextCompare: false) != 0)
		{
			text = TCusID.Text;
		}
		DataSet dataSet = Module1.connect("SELECT id, cin_no, ttype, cust_no, tmp_no,pic_date FROM Tb_Save_Image where (cust_no='" + text + "' or tmp_no='" + tmp_no + "') order by id desc");
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ButtonItem buttonItem = new ButtonItem();
				buttonItem.ButtonStyle = eButtonStyle.ImageAndText;
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num2]["ttype"], "บ\u0e31ตรประชาชน", TextCompare: false))
				{
					buttonItem.Image = Resources.thai_id;
				}
				else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num2]["ttype"], "ใบข\u0e31บข\u0e35\u0e48", TextCompare: false))
				{
					buttonItem.Image = Resources.thai_car_id;
				}
				else if (Operators.CompareString(dataSet.Tables[0].Rows[num2]["ttype"].ToString().ToLower(), "passport", TextCompare: false) == 0)
				{
					buttonItem.Image = Resources.passport_icon_29;
				}
				else
				{
					buttonItem.Image = Resources.vcard;
				}
				buttonItem.ImagePosition = eImagePosition.Top;
				buttonItem.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["id"]);
				buttonItem.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["ttype"], "\r\n"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["pic_date"]), "dd/MM/yy")), '\r'), '\n'));
				buttonItem.Click += ShowScan;
				ItemPanel1.Items.Add(buttonItem);
				num2++;
			}
			ItemPanel1.EndUpdate();
		}
	}

	private void ButtonItem1_Click(object sender, EventArgs e)
	{
	}

	public void CHANGE_PRICE()
	{
		if (isbook)
		{
			return;
		}
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType where name='" + TCusType.Text + "'");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			Dep_price = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["deposit"]);
		}
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
				{
					DataSet dataSet2 = Module1.connect("select * from HT_Rooms_Price where room_type='" + Conversions.ToString(Grid1[num2, 3]) + "' and room_custType='" + TCusType.Text + "'");
					decimal num5 = default(decimal);
					try
					{
						num5 = ((ComboBox1.SelectedIndex == 0) ? Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Room_Price"]) : ((ComboBox1.SelectedIndex != 1) ? Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Room_Price_M"]) : Conversions.ToDecimal(Strings.FormatNumber(Operators.DivideObject(dataSet2.Tables[0].Rows[0]["Room_Price_H"], Module1.decimal_0), 4))));
					}
					catch (Exception projectError)
					{
						ProjectData.SetProjectError(projectError);
						ProjectData.ClearProjectError();
					}
					if (dataSet2.Tables[0].Rows.Count != 0)
					{
						Grid1[num2, 7] = Dep_price;
						if (ComboBox1.SelectedIndex == 1)
						{
							Grid1[num2, 8] = num5;
						}
						else
						{
							Grid1[num2, 8] = Strings.Format(num5, "#,##0.00");
						}
						Grid1[num2, 10] = Strings.Format(decimal.Multiply(num5, Conversions.ToDecimal(Grid1[num2, 9])), "#,##0.00");
						Grid1[num2, 13] = Strings.Format(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid1[num2, 10]), Conversions.ToDecimal(Grid1[num2, 11])), Conversions.ToDecimal(Grid1[num2, 12])), "#,##0.00");
					}
					sum();
				}
				num2++;
			}
		}
	}

	private void TCusType_SelectedIndexChanged(object sender, EventArgs e)
	{
		CHANGE_PRICE();
	}

	private void TbookNo_TextChanged(object sender, EventArgs e)
	{
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		PanelCust.Visible = false;
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Refresh_Dep_auto();
	}

	private void Button10_Click_1(object sender, EventArgs e)
	{
		if (Operators.CompareString(Tname2.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อผ\u0e39\u0e49เข\u0e49าพ\u0e31ก");
			return;
		}
		ListView listView = ListView2;
		int count = listView.Items.Count;
		listView.Items.Add(Conversions.ToString(checked(count + 1)));
		listView.Items[count].SubItems.Add(Tname2.Text);
		listView.Items[count].SubItems.Add(Tcontry2.Text);
		listView = null;
		Tname2.Text = "";
		Tcontry2.Text = "";
		Tname2.Focus();
	}

	private void Button8_Click_1(object sender, EventArgs e)
	{
		if (ListView2.SelectedItems.Count == 0)
		{
			return;
		}
		ListView2.SelectedItems[0].Remove();
		checked
		{
			int num = ListView2.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					ListView2.Items[0].Text = Conversions.ToString(num2 + 1);
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		Panel5.Visible = false;
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		if (Panel5.Visible)
		{
			Panel5.Visible = false;
		}
		else
		{
			Panel5.Visible = true;
		}
	}

	private void Button12_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TcusName.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อในบ\u0e31ตรลงทะเบ\u0e35ยน");
			return;
		}
		ListView listView = ListView2;
		int count = listView.Items.Count;
		listView.Items.Add(Conversions.ToString(checked(count + 1)));
		listView.Items[count].SubItems.Add(Tcusperfix.Text + " " + TcusName.Text + " " + TextBox_2.Text);
		listView.Items[count].SubItems.Add(Tcontry.Text);
		listView = null;
		Tname2.Text = "";
		Tcontry2.Text = "";
		Tname2.Focus();
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		Button6.Visible = false;
		if (ComboBox1.SelectedIndex == 0)
		{
			Button3.Visible = true;
			Grid1.Cols[9].Caption = "จำนวนค\u0e37น";
			Grid1.Cols[5].Width = 100;
			ButtonT1.Checked = true;
			ButtonT2.Checked = false;
			ButtonT3.Checked = false;
			resum();
			return;
		}
		if (ComboBox1.SelectedIndex == 1)
		{
			Button3.Visible = true;
			Grid1.Cols[9].Caption = "ช\u0e31\u0e48วโมง";
			Grid1.Cols[5].Width = 100;
			ButtonT1.Checked = false;
			ButtonT2.Checked = true;
			ButtonT3.Checked = false;
			resum();
			return;
		}
		if (Operators.CompareString(EDIT_ID, "", TextCompare: false) != 0)
		{
			Button3.Visible = false;
			Button6.Visible = true;
		}
		Grid1.Cols[9].Caption = "เด\u0e37อน";
		Grid1.Cols[5].Width = 100;
		ButtonT1.Checked = false;
		ButtonT2.Checked = false;
		ButtonT3.Checked = true;
		resum();
	}

	public void resum()
	{
		int num = 0;
		checked
		{
			if (ComboBox1.SelectedIndex == 0)
			{
				int num2 = Grid1.Rows.Count - 1;
				int num3 = 1;
				while (true)
				{
					int num4 = num3;
					int num5 = num2;
					if (num4 > num5)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid1[num3, 1]), "", TextCompare: false) != 0)
					{
						Grid1[num3, 5] = Tend.Value;
						num = (int)DateAndTime.DateDiff(DateInterval.Day, Conversions.ToDate(Grid1[num3, 4]).Date, Conversions.ToDate(Grid1[num3, 5]).Date);
						if (DateTime.Compare(Conversions.ToDate(Grid1[num3, 4]).Date, Conversions.ToDate(Grid1[num3, 5]).Date) == 0)
						{
							num = 1;
						}
						else if (DateTime.Compare(Conversions.ToDate(Grid1[num3, 4]).Date, Conversions.ToDate(Grid1[num3, 5]).Date) != 0 && ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num3, 4]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num3, 4]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0)))
						{
							num++;
						}
						Grid1[num3, 9] = num;
						Grid1[num3, 10] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(Grid1[num3, 8]), Conversions.ToDecimal(Grid1[num3, 9])), "#,##0.00");
						Grid1[num3, 13] = Strings.Format(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid1[num3, 10]), Conversions.ToDecimal(Grid1[num3, 11])), Conversions.ToDecimal(Grid1[num3, 12])), "#,##0.00");
						Grid1[num3, 14] = "0";
					}
					num3++;
				}
			}
			else if (ComboBox1.SelectedIndex == 1)
			{
				int num6 = Grid1.Rows.Count - 1;
				int num7 = 1;
				while (true)
				{
					int num8 = num7;
					int num5 = num6;
					if (num8 <= num5)
					{
						if (Operators.CompareString(Conversions.ToString(Grid1[num7, 1]), "", TextCompare: false) != 0)
						{
							num = Convert.ToInt32(Module1.decimal_0);
							DateTime dateTime = Conversions.ToDate(Grid1[num7, 4]);
							Grid1[num7, 5] = dateTime.AddHours(num);
							Grid1[num7, 9] = num;
							Grid1[num7, 10] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(Grid1[num7, 8]), Conversions.ToDecimal(Grid1[num7, 9])), "#,##0.00");
							Grid1[num7, 13] = Strings.Format(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid1[num7, 10]), Conversions.ToDecimal(Grid1[num7, 11])), Conversions.ToDecimal(Grid1[num7, 12])), "#,##0.00");
							Grid1[num7, 14] = "0";
						}
						num7++;
						continue;
					}
					break;
				}
			}
			else if (ComboBox1.SelectedIndex == 2)
			{
				int num9 = Grid1.Rows.Count - 1;
				int num10 = 1;
				while (true)
				{
					int num11 = num10;
					int num5 = num9;
					if (num11 > num5)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid1[num10, 1]), "", TextCompare: false) != 0)
					{
						num = 1;
						Grid1[num10, 5] = Tend.Value;
						Grid1[num10, 9] = 1;
						Grid1[num10, 10] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(Grid1[num10, 8]), Conversions.ToDecimal(Grid1[num10, 9])), "#,##0.00");
						Grid1[num10, 13] = Strings.Format(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Grid1[num10, 10]), Conversions.ToDecimal(Grid1[num10, 11])), Conversions.ToDecimal(Grid1[num10, 12])), "#,##0.00");
						Grid1[num10, 14] = "0";
					}
					num10++;
				}
			}
			CHANGE_PRICE();
			sum();
		}
	}

	private void Button13_Click(object sender, EventArgs e)
	{
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
				{
					Grid1[num2, 14] = RuntimeHelpers.GetObjectValue(Grid1[num2, 13]);
				}
				num2++;
			}
			sum();
		}
	}

	private void Button14_Click(object sender, EventArgs e)
	{
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
				{
					Grid1[num2, 14] = 0;
				}
				num2++;
			}
			sum();
		}
	}

	private void Tw_ampore_TextChanged(object sender, EventArgs e)
	{
	}

	private void Button16_Click(object sender, EventArgs e)
	{
		checked
		{
			int num = Grid2.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num2, 1]), "", TextCompare: false) != 0)
				{
					Grid2[num2, 11] = RuntimeHelpers.GetObjectValue(Grid2[num2, 10]);
				}
				num2++;
			}
			sum();
		}
	}

	private void Button15_Click(object sender, EventArgs e)
	{
		checked
		{
			int num = Grid2.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num2, 1]), "", TextCompare: false) != 0)
				{
					Grid2[num2, 11] = 0;
				}
				num2++;
			}
			sum();
		}
	}

	private void Label6_Click(object sender, EventArgs e)
	{
	}

	private void ButtonT1_Click(object sender, EventArgs e)
	{
		ComboBox1.SelectedIndex = 0;
	}

	private void ButtonT2_Click(object sender, EventArgs e)
	{
		ComboBox1.SelectedIndex = 1;
	}

	private void ButtonT3_Click(object sender, EventArgs e)
	{
		ComboBox1.SelectedIndex = 2;
	}

	private void Grid1_Click(object sender, EventArgs e)
	{
	}

	private void Grid1_BeforeEdit(object sender, RowColEventArgs e)
	{
		if (Operators.ConditionalCompareObjectEqual(Grid1[e.Row, 6], "Check-Out", TextCompare: false) && e.Col != 14)
		{
			e.Cancel = true;
		}
	}

	private void Tcusperfix_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			TcusName.Focus();
		}
	}

	private void Tcusperfix_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void TcusName_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			TextBox_2.Focus();
		}
	}

	private void TCusName2_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			TcusSex.Focus();
		}
	}

	private void TcusSex_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			TCarID.Focus();
		}
	}

	private void TCarID_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			Tc_tel.Focus();
		}
	}

	private void NEXKEY(object sender, KeyEventArgs e)
	{
		if (e.KeyCode == Keys.Return)
		{
			SendKeys.Send("{tab}");
		}
	}

	private void Timer2_Tick(object sender, EventArgs e)
	{
		Timer2.Enabled = false;
		TextBox_0.Focus();
	}

	private void Tdebt_TextChanged(object sender, EventArgs e)
	{
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		if (Label_0.Visible)
		{
			MessageBox.Show("ไม\u0e48สามารถเปล\u0e35\u0e48ยนล\u0e39กค\u0e49าได\u0e49เน\u0e37\u0e48องจากม\u0e35เง\u0e34นจองค\u0e49างอย\u0e39\u0e48", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			return;
		}
		object obj = Interaction.InputBox("กร\u0e38ณาใส\u0e48เบอร\u0e4cโทรศ\u0e31พท\u0e4c", "กร\u0e38ณาใส\u0e48เบอร\u0e4cโทรศ\u0e31พท\u0e4c");
		if (Operators.ConditionalCompareObjectNotEqual(obj, "", TextCompare: false))
		{
			obj = Strings.Trim(Conversions.ToString(obj)).Replace(" ", "").Replace("-", "")
				.Replace(".", "");
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Customers where Cust_Add_tel='", obj), "'")));
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("เบอร\u0e4cโทร ", obj), " ม\u0e35อย\u0e39\u0e48ในระบบแล\u0e49ว ช\u0e37\u0e48อ "), dataSet.Tables[0].Rows[0]["Cust_name"]), " "), dataSet.Tables[0].Rows[0]["Cust_name2"])), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK);
				TextBox_0.Text = "";
				TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_no"]);
				TcusName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				TextBox_2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				TCusTypeMain.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type_Main"]);
				TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				TextBox_1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Email"]);
				Tcusperfix.Text = dataSet.Tables[0].Rows[0]["Cust_perfix"].ToString();
				TcusSex.Text = dataSet.Tables[0].Rows[0]["Cust_sex"].ToString();
				TcusCardID.Text = dataSet.Tables[0].Rows[0]["Cust_IDcard"].ToString();
				Tc_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_no"]);
				Tc_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_moo"]);
				Tc_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_soi"]);
				Tc_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_road"]);
				Tc_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tambon"]);
				Tc_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_ampore"]);
				Tc_province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_province"]);
				Tc_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_code"]);
				Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				TextBox_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				Tc_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				Tw.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"]);
				Tw_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_no"]);
				Tw_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_moo"]);
				Tw_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_soi"]);
				Tw_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_road"]);
				Tw_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tambon"]);
				Tw_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_ampore"]);
				Tw_privince.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_province"]);
				Tw_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_code"]);
				Tw_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"]);
				Tw_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_fax"]);
				TwTax.Text = dataSet.Tables[0].Rows[0]["Cust_Work_tax"].ToString();
				Tover.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Price_Over"]);
				Tcontry.Text = dataSet.Tables[0].Rows[0]["Cust_contry"].ToString();
				method_0();
				RefreshScan();
				PanelCust.Visible = false;
				Button3.Focus();
				Refresh_Dep_auto(0m);
			}
			else
			{
				clear_Add_cus();
				TextBox_0.Text = Conversions.ToString(obj);
				TcusName.Focus();
			}
		}
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		Clear();
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		Module1.GenSmartCard();
		Process process = new Process();
		process.EnableRaisingEvents = false;
		process.StartInfo.FileName = Module1.Path_Program + "KPThaiNationalIDCard.exe";
		process.Start();
		process.WaitForExit();
		if (!File.Exists("thaiid.txt"))
		{
			return;
		}
		StreamReader streamReader = new StreamReader(Module1.PathF + "\\thaiid.txt");
		string expression = streamReader.ReadToEnd();
		streamReader.Close();
		string[] array = Strings.Split(expression, "\r\n");
		int num = 1;
		string str = "";
		string str2 = "";
		string str3 = "";
		string str4 = "";
		string str5 = "";
		string str6 = "";
		string str7 = "";
		string str8 = "";
		string str9 = "";
		string str10 = "";
		string str11 = "";
		string str12 = "";
		string[] array2 = array;
		checked
		{
			foreach (string text in array2)
			{
				if (Operators.CompareString(text, "", TextCompare: false) != 0)
				{
					switch (num)
					{
					case 1:
						str = text;
						break;
					case 2:
						str2 = text;
						break;
					case 3:
						str3 = text;
						break;
					case 4:
						str4 = text;
						break;
					case 5:
						str5 = text;
						break;
					case 10:
						str6 = text;
						break;
					case 11:
						str7 = text;
						break;
					case 12:
						str8 = text;
						break;
					case 13:
						str9 = text;
						break;
					case 14:
						str10 = text;
						break;
					case 15:
						str11 = text;
						break;
					case 16:
						str12 = text;
						break;
					}
				}
				num++;
			}
			DataSet dataSet = Module1.connect("select * from HT_Customers where Cust_name='" + Strings.Trim(str3) + "' and Cust_name2='" + Strings.Trim(str4) + "'");
			if ((dataSet.Tables[0].Rows.Count != 0) & !Label_0.Visible)
			{
				TextBox_0.Text = "";
				TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_no"]);
				TcusName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				TextBox_2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				TCusTypeMain.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type_Main"]);
				TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				TextBox_1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Email"]);
				Tcusperfix.Text = dataSet.Tables[0].Rows[0]["Cust_perfix"].ToString();
				TcusSex.Text = dataSet.Tables[0].Rows[0]["Cust_sex"].ToString();
				TcusCardID.Text = dataSet.Tables[0].Rows[0]["Cust_IDcard"].ToString();
				Tc_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_no"]);
				Tc_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_moo"]);
				Tc_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_soi"]);
				Tc_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_road"]);
				Tc_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tambon"]);
				Tc_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_ampore"]);
				Tc_province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_province"]);
				Tc_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_code"]);
				Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				TextBox_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				Tc_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				Tw.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"]);
				Tw_no.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_no"]);
				Tw_moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_moo"]);
				Tw_soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_soi"]);
				Tw_road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_road"]);
				Tw_tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tambon"]);
				Tw_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_ampore"]);
				Tw_privince.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_province"]);
				Tw_code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_code"]);
				Tw_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"]);
				Tw_fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_fax"]);
				TwTax.Text = dataSet.Tables[0].Rows[0]["Cust_Work_tax"].ToString();
				Tover.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Price_Over"]);
				Tcontry.Text = dataSet.Tables[0].Rows[0]["Cust_contry"].ToString();
				if (Operators.CompareString(TcusCardID.Text, "", TextCompare: false) == 0)
				{
					TcusCardID.Text = Strings.Trim(str);
				}
				method_0();
				RefreshScan();
				PanelCust.Visible = false;
				Button3.Focus();
				Refresh_Dep_auto(0m);
				if (!File.Exists(Module1.PathF + "/thaiid.png"))
				{
					return;
				}
				DataSet dataSet2 = ((Operators.CompareString(TCusID.Text, "", TextCompare: false) == 0) ? Module1.connect("SELECT id FROM Tb_Save_Image where ( tmp_no='" + tmp_no + "') and ttype='บ\u0e31ตรประชาชน' order by id desc") : Module1.connect("SELECT id FROM Tb_Save_Image where (cust_no='" + TCusID.Text + "') and ttype='บ\u0e31ตรประชาชน' order by id desc"));
				if (dataSet2.Tables[0].Rows.Count == 0)
				{
					MyProject.Forms.FrmShowPreviewSmartCard.loadpic();
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = 0;
					MyProject.Forms.FrmShowPreviewSmartCard.Show();
					Application.DoEvents();
					FileStream fileStream = new FileStream(Module1.PathF + "/thaiid.png", FileMode.Open, FileAccess.Read);
					BinaryReader binaryReader = new BinaryReader(fileStream);
					byte[] array3 = binaryReader.ReadBytes((int)fileStream.Length);
					binaryReader.Close();
					fileStream.Close();
					int num2 = 1;
					do
					{
						MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num2;
						num2++;
					}
					while (num2 <= 30);
					Application.DoEvents();
					StringBuilder stringBuilder = new StringBuilder();
					byte[] array4 = array3;
					foreach (byte b in array4)
					{
						stringBuilder.Append(b.ToString("X2"));
					}
					int num3 = 31;
					do
					{
						MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num3;
						num3++;
					}
					while (num3 <= 50);
					Application.DoEvents();
					if (Operators.CompareString(TCusID.Text, "", TextCompare: false) != 0)
					{
						object left = "INSERT INTO [Tb_Save_Image]";
						left = Operators.ConcatenateObject(left, "([cin_no]");
						left = Operators.ConcatenateObject(left, ",[ttype]");
						left = Operators.ConcatenateObject(left, ",[pic],[cust_no],[tmp_no],[pic_date])");
						left = Operators.ConcatenateObject(left, "VALUES");
						left = Operators.ConcatenateObject(left, "(''");
						left = Operators.ConcatenateObject(left, ",'บ\u0e31ตรประชาชน'");
						left = Operators.ConcatenateObject(left, ",0x" + stringBuilder.ToString());
						left = Operators.ConcatenateObject(left, string.Concat(",'" + TCusID.Text, "'"));
						left = Operators.ConcatenateObject(left, ",''");
						left = Operators.ConcatenateObject(left, ",getdate()");
						left = Operators.ConcatenateObject(left, ")");
						Module1.connect(Conversions.ToString(left));
					}
					else
					{
						object left2 = "INSERT INTO [Tb_Save_Image]";
						left2 = Operators.ConcatenateObject(left2, "([cin_no]");
						left2 = Operators.ConcatenateObject(left2, ",[ttype]");
						left2 = Operators.ConcatenateObject(left2, ",[pic],[cust_no],[tmp_no],[pic_date])");
						left2 = Operators.ConcatenateObject(left2, "VALUES");
						left2 = Operators.ConcatenateObject(left2, "(''");
						left2 = Operators.ConcatenateObject(left2, ",'บ\u0e31ตรประชาชน'");
						left2 = Operators.ConcatenateObject(left2, ",0x" + stringBuilder.ToString());
						left2 = Operators.ConcatenateObject(left2, ",''");
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + tmp_no, "'"));
						left2 = Operators.ConcatenateObject(left2, ",getdate()");
						left2 = Operators.ConcatenateObject(left2, ")");
						Module1.connect(Conversions.ToString(left2));
					}
					int num4 = 51;
					do
					{
						MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num4;
						num4++;
					}
					while (num4 <= 80);
					Application.DoEvents();
					RefreshScan();
					int num5 = 81;
					do
					{
						MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num5;
						num5++;
					}
					while (num5 <= 100);
				}
				else
				{
					RefreshScan();
				}
				return;
			}
			if (Operators.CompareString(TbookNo.Text, "", TextCompare: false) == 0)
			{
				clear_Add_cus();
			}
			else if (!Label_0.Visible && Operators.CompareString(Booking_cust, TCusID.Text, TextCompare: false) != 0)
			{
				clear_Add_cus();
			}
			TcusName.Text = Strings.Trim(str3);
			TextBox_2.Text = Strings.Trim(str4);
			Tcusperfix.Text = Strings.Trim(str2);
			TcusSex.Text = Strings.Trim(str5);
			TcusCardID.Text = Strings.Trim(str);
			Tc_no.Text = Strings.Trim(str6);
			Tc_moo.Text = Strings.Trim(str7);
			Tc_soi.Text = Strings.Trim(str8);
			Tc_road.Text = Strings.Trim(str9);
			Tc_tambon.Text = Strings.Trim(str10);
			Tc_ampore.Text = Strings.Trim(str11);
			Tc_province.Text = Strings.Trim(str12);
			if (!File.Exists(Module1.PathF + "/thaiid.png"))
			{
				return;
			}
			DataSet dataSet3 = ((Operators.CompareString(TCusID.Text, "", TextCompare: false) == 0) ? Module1.connect("SELECT id FROM Tb_Save_Image where ( tmp_no='" + tmp_no + "') and ttype='บ\u0e31ตรประชาชน' order by id desc") : Module1.connect("SELECT id FROM Tb_Save_Image where (cust_no='" + TCusID.Text + "') and ttype='บ\u0e31ตรประชาชน' order by id desc"));
			Application.DoEvents();
			if (dataSet3.Tables[0].Rows.Count == 0)
			{
				MyProject.Forms.FrmShowPreviewSmartCard.loadpic();
				MyProject.Forms.FrmShowPreviewSmartCard.Show();
				Application.DoEvents();
				FileStream fileStream2 = new FileStream(Module1.PathF + "/thaiid.png", FileMode.Open, FileAccess.Read);
				BinaryReader binaryReader2 = new BinaryReader(fileStream2);
				byte[] array5 = binaryReader2.ReadBytes((int)fileStream2.Length);
				binaryReader2.Close();
				fileStream2.Close();
				int num6 = 1;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num6;
					num6++;
				}
				while (num6 <= 30);
				Application.DoEvents();
				StringBuilder stringBuilder2 = new StringBuilder();
				byte[] array6 = array5;
				foreach (byte b2 in array6)
				{
					stringBuilder2.Append(b2.ToString("X2"));
				}
				int num7 = 31;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num7;
					num7++;
				}
				while (num7 <= 50);
				Application.DoEvents();
				if (Operators.CompareString(TCusID.Text, "", TextCompare: false) != 0)
				{
					object left3 = "INSERT INTO [Tb_Save_Image]";
					left3 = Operators.ConcatenateObject(left3, "([cin_no]");
					left3 = Operators.ConcatenateObject(left3, ",[ttype]");
					left3 = Operators.ConcatenateObject(left3, ",[pic],[cust_no],[tmp_no],[pic_date])");
					left3 = Operators.ConcatenateObject(left3, "VALUES");
					left3 = Operators.ConcatenateObject(left3, "(''");
					left3 = Operators.ConcatenateObject(left3, ",'บ\u0e31ตรประชาชน'");
					left3 = Operators.ConcatenateObject(left3, ",0x" + stringBuilder2.ToString());
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + TCusID.Text, "'"));
					left3 = Operators.ConcatenateObject(left3, ",''");
					left3 = Operators.ConcatenateObject(left3, ",getdate()");
					left3 = Operators.ConcatenateObject(left3, ")");
					Module1.connect(Conversions.ToString(left3));
				}
				else
				{
					object left4 = "INSERT INTO [Tb_Save_Image]";
					left4 = Operators.ConcatenateObject(left4, "([cin_no]");
					left4 = Operators.ConcatenateObject(left4, ",[ttype]");
					left4 = Operators.ConcatenateObject(left4, ",[pic],[cust_no],[tmp_no],[pic_date])");
					left4 = Operators.ConcatenateObject(left4, "VALUES");
					left4 = Operators.ConcatenateObject(left4, "(''");
					left4 = Operators.ConcatenateObject(left4, ",'บ\u0e31ตรประชาชน'");
					left4 = Operators.ConcatenateObject(left4, ",0x" + stringBuilder2.ToString());
					left4 = Operators.ConcatenateObject(left4, ",''");
					left4 = Operators.ConcatenateObject(left4, string.Concat(",'" + tmp_no, "'"));
					left4 = Operators.ConcatenateObject(left4, ",getdate()");
					left4 = Operators.ConcatenateObject(left4, ")");
					Module1.connect(Conversions.ToString(left4));
				}
				Application.DoEvents();
				int num8 = 51;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num8;
					num8++;
				}
				while (num8 <= 80);
				RefreshScan();
				int num9 = 81;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num9;
					num9++;
				}
				while (num9 <= 100);
			}
			else
			{
				RefreshScan();
			}
		}
	}

	private void Button6_Click(object sender, EventArgs e)
	{
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
					{
						Grid1[num2, 5] = Tend.Value;
					}
					num2++;
					continue;
				}
				break;
			}
		}
	}
}
