using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
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
public class FrmPayAdd : Office2007Form
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

	[AccessedThroughProperty("Button6")]
	private Button _Button6;

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

	[AccessedThroughProperty("Label30")]
	private Label _Label30;

	[AccessedThroughProperty("Label23")]
	private Label _Label23;

	[AccessedThroughProperty("Label35")]
	private Label _Label35;

	[AccessedThroughProperty("Label50")]
	private Label _Label50;

	[AccessedThroughProperty("TCusID")]
	private TextBox _TCusID;

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

	[AccessedThroughProperty("SuperTooltip1")]
	private SuperTooltip _SuperTooltip1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Tover")]
	private TextBox _Tover;

	[AccessedThroughProperty("Label25")]
	private Label _Label25;

	[AccessedThroughProperty("Grid2")]
	private C1FlexGrid _Grid2;

	[AccessedThroughProperty("POver")]
	private TextBox _POver;

	[AccessedThroughProperty("LOver")]
	private Label _LOver;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Tcontry")]
	private TextBox _Tcontry;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label57")]
	private Label _Label57;

	[AccessedThroughProperty("Grid1")]
	private C1FlexGrid _Grid1;

	[AccessedThroughProperty("Button15")]
	private Button _Button15;

	[AccessedThroughProperty("Button16")]
	private Button _Button16;

	[AccessedThroughProperty("Button14")]
	private Button _Button14;

	[AccessedThroughProperty("Button13")]
	private Button _Button13;

	public string EDIT_ID;

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
			_TcusName = value;
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
			_TCusType = value;
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
			_TCarID = value;
		}
	}

	internal virtual TextBox TextBox_0
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

	internal virtual TextBox TextBox_1
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
			_TCusName2 = value;
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
			_expandablePanel5 = value;
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
			if (_expandablePanel4 != null)
			{
				_expandablePanel4.Click -= value2;
			}
			_expandablePanel4 = value;
			if (_expandablePanel4 != null)
			{
				_expandablePanel4.Click += value2;
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
			PaintEventHandler value2 = Panel1_Paint;
			if (_Panel1 != null)
			{
				_Panel1.Paint -= value2;
			}
			_Panel1 = value;
			if (_Panel1 != null)
			{
				_Panel1.Paint += value2;
			}
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
			EventHandler value2 = ComboBox3_GotFocus;
			KeyEventHandler value3 = ComboBox3_KeyDown;
			EventHandler value4 = ComboBox3_LostFocus;
			if (_Tc_tel != null)
			{
				_Tc_tel.GotFocus -= value2;
				_Tc_tel.KeyDown -= value3;
				_Tc_tel.LostFocus -= value4;
			}
			_Tc_tel = value;
			if (_Tc_tel != null)
			{
				_Tc_tel.GotFocus += value2;
				_Tc_tel.KeyDown += value3;
				_Tc_tel.LostFocus += value4;
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
			_Tw_ampore = value;
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
			_Tw_tambon = value;
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
			KeyEventHandler value3 = ComboBox4_KeyDown;
			EventHandler value4 = ComboBox4_GotFocus;
			if (_Tw_tel != null)
			{
				_Tw_tel.LostFocus -= value2;
				_Tw_tel.KeyDown -= value3;
				_Tw_tel.GotFocus -= value4;
			}
			_Tw_tel = value;
			if (_Tw_tel != null)
			{
				_Tw_tel.LostFocus += value2;
				_Tw_tel.KeyDown += value3;
				_Tw_tel.GotFocus += value4;
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
			_Tw_code = value;
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
			_Tw_road = value;
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
			_Tw_privince = value;
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
			_Tw = value;
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
			_Tw_soi = value;
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
			_Tw_moo = value;
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
			_Tw_no = value;
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
			_Tw_fax = value;
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
			if (_Tdebt != null)
			{
				_Tdebt.LostFocus -= value2;
				_Tdebt.KeyDown -= value3;
			}
			_Tdebt = value;
			if (_Tdebt != null)
			{
				_Tdebt.LostFocus += value2;
				_Tdebt.KeyDown += value3;
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

	internal virtual SuperTooltip SuperTooltip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SuperTooltip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SuperTooltip1 = value;
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

	internal virtual Label Label6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label6 = value;
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
			RowColEventHandler value3 = Grid1_AfterEdit;
			if (_Grid1 != null)
			{
				_Grid1.StartEdit -= value2;
				_Grid1.AfterEdit -= value3;
			}
			_Grid1 = value;
			if (_Grid1 != null)
			{
				_Grid1.StartEdit += value2;
				_Grid1.AfterEdit += value3;
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

	[DebuggerNonUserCode]
	static FrmPayAdd()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmPayAdd()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmCheckIn_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EDIT_ID = "";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmPayAdd));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label57 = new System.Windows.Forms.Label();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.Tover = new System.Windows.Forms.TextBox();
		this.Label25 = new System.Windows.Forms.Label();
		this.Panel3 = new System.Windows.Forms.Panel();
		this.expandablePanel5 = new DevComponents.DotNetBar.ExpandablePanel();
		this.Panel2 = new System.Windows.Forms.Panel();
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
		this.Label6 = new System.Windows.Forms.Label();
		this.Tcontry = new System.Windows.Forms.TextBox();
		this.Tc_ampore = new System.Windows.Forms.TextBox();
		this.Label39 = new System.Windows.Forms.Label();
		this.Tc_tambon = new System.Windows.Forms.TextBox();
		this.Label38 = new System.Windows.Forms.Label();
		this.Tc_tel = new System.Windows.Forms.ComboBox();
		this.Tc_code = new System.Windows.Forms.TextBox();
		this.Label41 = new System.Windows.Forms.Label();
		this.Tc_road = new System.Windows.Forms.TextBox();
		this.Tc_province = new System.Windows.Forms.TextBox();
		this.Label37 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label40 = new System.Windows.Forms.Label();
		this.Label9 = new System.Windows.Forms.Label();
		this.Tc_soi = new System.Windows.Forms.TextBox();
		this.Label36 = new System.Windows.Forms.Label();
		this.Tc_moo = new System.Windows.Forms.TextBox();
		this.Label11 = new System.Windows.Forms.Label();
		this.Tc_no = new System.Windows.Forms.TextBox();
		this.Label8 = new System.Windows.Forms.Label();
		this.Tc_fax = new System.Windows.Forms.TextBox();
		this.TCarType = new System.Windows.Forms.ComboBox();
		this.Label31 = new System.Windows.Forms.Label();
		this.TCusType = new System.Windows.Forms.ComboBox();
		this.Label21 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label16 = new System.Windows.Forms.Label();
		this.Label29 = new System.Windows.Forms.Label();
		this.Label27 = new System.Windows.Forms.Label();
		this.Label26 = new System.Windows.Forms.Label();
		this.Label50 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.TCarID = new System.Windows.Forms.TextBox();
		this.TextBox_0 = new System.Windows.Forms.TextBox();
		this.TextBox_1 = new System.Windows.Forms.TextBox();
		this.TCusID = new System.Windows.Forms.TextBox();
		this.TcusName = new System.Windows.Forms.TextBox();
		this.TdocNum = new System.Windows.Forms.TextBox();
		this.Button6 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Button14 = new System.Windows.Forms.Button();
		this.Button13 = new System.Windows.Forms.Button();
		this.Button15 = new System.Windows.Forms.Button();
		this.Button16 = new System.Windows.Forms.Button();
		this.POver = new System.Windows.Forms.TextBox();
		this.LOver = new System.Windows.Forms.Label();
		this.Tnote = new System.Windows.Forms.TextBox();
		this.Tdebt = new System.Windows.Forms.TextBox();
		this.Tcash = new System.Windows.Forms.TextBox();
		this.Tpay = new System.Windows.Forms.TextBox();
		this.TselectRoom = new System.Windows.Forms.ComboBox();
		this.Label55 = new System.Windows.Forms.Label();
		this.Grid2 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.Grid1 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.Button9 = new System.Windows.Forms.Button();
		this.Label35 = new System.Windows.Forms.Label();
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
		this.Button7 = new System.Windows.Forms.Button();
		this.Label14 = new System.Windows.Forms.Label();
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
		this.SuperTooltip1 = new DevComponents.DotNetBar.SuperTooltip();
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
		this.expandablePanel5.SuspendLayout();
		this.Panel2.SuspendLayout();
		this.expandablePanel4.SuspendLayout();
		this.Panel1.SuspendLayout();
		this.PanelEx1.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.Grid2).BeginInit();
		((System.ComponentModel.ISupportInitialize)this.Grid1).BeginInit();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label57);
		this.GroupBox1.Controls.Add(this.ButtonX2);
		this.GroupBox1.Controls.Add(this.Tover);
		this.GroupBox1.Controls.Add(this.Label25);
		this.GroupBox1.Controls.Add(this.Panel3);
		this.GroupBox1.Controls.Add(this.TCarType);
		this.GroupBox1.Controls.Add(this.Label31);
		this.GroupBox1.Controls.Add(this.TCusType);
		this.GroupBox1.Controls.Add(this.Label21);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label16);
		this.GroupBox1.Controls.Add(this.Label29);
		this.GroupBox1.Controls.Add(this.Label27);
		this.GroupBox1.Controls.Add(this.Label26);
		this.GroupBox1.Controls.Add(this.Label50);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.TCarID);
		this.GroupBox1.Controls.Add(this.TextBox_0);
		this.GroupBox1.Controls.Add(this.TextBox_1);
		this.GroupBox1.Controls.Add(this.TCusID);
		this.GroupBox1.Controls.Add(this.TcusName);
		this.GroupBox1.Controls.Add(this.TdocNum);
		this.GroupBox1.Controls.Add(this.Button6);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(10, 4);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1201, 272);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 12;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายละเอ\u0e35ยด";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "รายงว\u0e31น", "รายช\u0e31\u0e48วโมง", "รายเด\u0e37อน" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(750, 20);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(170, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 74;
		this.Label57.AutoSize = true;
		System.Windows.Forms.Label label = this.Label57;
		location = new System.Drawing.Point(699, 25);
		label.Location = location;
		this.Label57.Name = "Label57";
		System.Windows.Forms.Label label2 = this.Label57;
		size = new System.Drawing.Size(49, 16);
		label2.Size = size;
		this.Label57.TabIndex = 73;
		this.Label57.Text = "ประเภท";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(1086, 74);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		this.ButtonX2.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(109, 31);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 66;
		this.ButtonX2.Text = "ใช\u0e49ยอดน\u0e35\u0e49จ\u0e48าย";
		this.Tover.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Tover.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tover.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tover.Font = new System.Drawing.Font("Tahoma", 18f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Tover.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tover = this.Tover;
		location = new System.Drawing.Point(1086, 36);
		tover.Location = location;
		System.Windows.Forms.TextBox tover2 = this.Tover;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tover2.Margin = margin;
		this.Tover.Name = "Tover";
		this.Tover.ReadOnly = true;
		System.Windows.Forms.TextBox tover3 = this.Tover;
		size = new System.Drawing.Size(109, 36);
		tover3.Size = size;
		this.Tover.TabIndex = 65;
		this.Tover.Text = "1000";
		this.Tover.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Label25.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label25.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label25.ForeColor = System.Drawing.Color.MediumBlue;
		System.Windows.Forms.Label label3 = this.Label25;
		location = new System.Drawing.Point(1031, 12);
		label3.Location = location;
		this.Label25.Name = "Label25";
		System.Windows.Forms.Label label4 = this.Label25;
		size = new System.Drawing.Size(166, 25);
		label4.Size = size;
		this.Label25.TabIndex = 67;
		this.Label25.Text = "เง\u0e34นจ\u0e48ายล\u0e48วงหน\u0e49า";
		this.Label25.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Panel3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Panel3.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Panel3.Controls.Add(this.expandablePanel5);
		this.Panel3.Controls.Add(this.expandablePanel4);
		System.Windows.Forms.Panel panel = this.Panel3;
		location = new System.Drawing.Point(6, 109);
		panel.Location = location;
		this.Panel3.Name = "Panel3";
		System.Windows.Forms.Panel panel2 = this.Panel3;
		size = new System.Drawing.Size(1189, 158);
		panel2.Size = size;
		this.Panel3.TabIndex = 61;
		this.expandablePanel5.CanvasColor = System.Drawing.SystemColors.Control;
		this.expandablePanel5.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.expandablePanel5.Controls.Add(this.Panel2);
		this.expandablePanel5.Dock = System.Windows.Forms.DockStyle.Top;
		this.expandablePanel5.ExpandButtonVisible = false;
		this.expandablePanel5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ExpandablePanel expandablePanel = this.expandablePanel5;
		location = new System.Drawing.Point(0, 130);
		expandablePanel.Location = location;
		this.expandablePanel5.Name = "expandablePanel5";
		DevComponents.DotNetBar.ExpandablePanel expandablePanel2 = this.expandablePanel5;
		size = new System.Drawing.Size(1187, 130);
		expandablePanel2.Size = size;
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
		this.expandablePanel5.TitleText = "สถานท\u0e35\u0e48ทำงาน";
		this.Panel2.BackColor = System.Drawing.Color.LavenderBlush;
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
		System.Windows.Forms.Panel panel3 = this.Panel2;
		location = new System.Drawing.Point(0, 26);
		panel3.Location = location;
		this.Panel2.Name = "Panel2";
		System.Windows.Forms.Panel panel4 = this.Panel2;
		size = new System.Drawing.Size(1187, 104);
		panel4.Size = size;
		this.Panel2.TabIndex = 2;
		this.Tw_ampore.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_ampore = this.Tw_ampore;
		location = new System.Drawing.Point(842, 43);
		tw_ampore.Location = location;
		System.Windows.Forms.TextBox tw_ampore2 = this.Tw_ampore;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_ampore2.Margin = margin;
		this.Tw_ampore.Name = "Tw_ampore";
		this.Tw_ampore.ReadOnly = true;
		System.Windows.Forms.TextBox tw_ampore3 = this.Tw_ampore;
		size = new System.Drawing.Size(130, 23);
		tw_ampore3.Size = size;
		this.Tw_ampore.TabIndex = 1;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label2;
		location = new System.Drawing.Point(773, 46);
		label5.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(67, 16);
		label6.Size = size;
		this.Label2.TabIndex = 47;
		this.Label2.Text = "เขต/อำเภอ";
		this.Tw_tambon.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_tambon = this.Tw_tambon;
		location = new System.Drawing.Point(628, 43);
		tw_tambon.Location = location;
		System.Windows.Forms.TextBox tw_tambon2 = this.Tw_tambon;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_tambon2.Margin = margin;
		this.Tw_tambon.Name = "Tw_tambon";
		this.Tw_tambon.ReadOnly = true;
		System.Windows.Forms.TextBox tw_tambon3 = this.Tw_tambon;
		size = new System.Drawing.Size(130, 23);
		tw_tambon3.Size = size;
		this.Tw_tambon.TabIndex = 1;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label3;
		location = new System.Drawing.Point(555, 45);
		label7.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label8 = this.Label3;
		size = new System.Drawing.Size(71, 16);
		label8.Size = size;
		this.Label3.TabIndex = 47;
		this.Label3.Text = "แขวง/ตำบล";
		this.Tw_tel.Enabled = false;
		this.Tw_tel.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tw_tel = this.Tw_tel;
		location = new System.Drawing.Point(422, 70);
		tw_tel.Location = location;
		System.Windows.Forms.ComboBox tw_tel2 = this.Tw_tel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_tel2.Margin = margin;
		this.Tw_tel.Name = "Tw_tel";
		System.Windows.Forms.ComboBox tw_tel3 = this.Tw_tel;
		size = new System.Drawing.Size(170, 24);
		tw_tel3.Size = size;
		this.Tw_tel.TabIndex = 51;
		this.Tw_code.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_code = this.Tw_code;
		location = new System.Drawing.Point(290, 71);
		tw_code.Location = location;
		System.Windows.Forms.TextBox tw_code2 = this.Tw_code;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_code2.Margin = margin;
		this.Tw_code.Name = "Tw_code";
		this.Tw_code.ReadOnly = true;
		System.Windows.Forms.TextBox tw_code3 = this.Tw_code;
		size = new System.Drawing.Size(94, 23);
		tw_code3.Size = size;
		this.Tw_code.TabIndex = 1;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label4;
		location = new System.Drawing.Point(211, 75);
		label9.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label10 = this.Label4;
		size = new System.Drawing.Size(80, 16);
		label10.Size = size;
		this.Label4.TabIndex = 47;
		this.Label4.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.Tw_road.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_road = this.Tw_road;
		location = new System.Drawing.Point(422, 43);
		tw_road.Location = location;
		System.Windows.Forms.TextBox tw_road2 = this.Tw_road;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_road2.Margin = margin;
		this.Tw_road.Name = "Tw_road";
		this.Tw_road.ReadOnly = true;
		System.Windows.Forms.TextBox tw_road3 = this.Tw_road;
		size = new System.Drawing.Size(129, 23);
		tw_road3.Size = size;
		this.Tw_road.TabIndex = 1;
		this.Tw_privince.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_privince = this.Tw_privince;
		location = new System.Drawing.Point(80, 71);
		tw_privince.Location = location;
		System.Windows.Forms.TextBox tw_privince2 = this.Tw_privince;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_privince2.Margin = margin;
		this.Tw_privince.Name = "Tw_privince";
		this.Tw_privince.ReadOnly = true;
		System.Windows.Forms.TextBox tw_privince3 = this.Tw_privince;
		size = new System.Drawing.Size(125, 23);
		tw_privince3.Size = size;
		this.Tw_privince.TabIndex = 1;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label5;
		location = new System.Drawing.Point(387, 46);
		label11.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label12 = this.Label5;
		size = new System.Drawing.Size(32, 16);
		label12.Size = size;
		this.Label5.TabIndex = 47;
		this.Label5.Text = "ถนน";
		this.Label42.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label42;
		location = new System.Drawing.Point(598, 74);
		label13.Location = location;
		this.Label42.Name = "Label42";
		System.Windows.Forms.Label label14 = this.Label42;
		size = new System.Drawing.Size(28, 16);
		label14.Size = size;
		this.Label42.TabIndex = 47;
		this.Label42.Text = "Fax";
		this.Label43.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label43;
		location = new System.Drawing.Point(35, 75);
		label15.Location = location;
		this.Label43.Name = "Label43";
		System.Windows.Forms.Label label16 = this.Label43;
		size = new System.Drawing.Size(43, 16);
		label16.Size = size;
		this.Label43.TabIndex = 47;
		this.Label43.Text = "จ\u0e31งหว\u0e31ด";
		this.Label44.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label44;
		location = new System.Drawing.Point(391, 75);
		label17.Location = location;
		this.Label44.Name = "Label44";
		System.Windows.Forms.Label label18 = this.Label44;
		size = new System.Drawing.Size(29, 16);
		label18.Size = size;
		this.Label44.TabIndex = 47;
		this.Label44.Text = "โทร";
		this.Tw.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw = this.Tw;
		location = new System.Drawing.Point(80, 12);
		tw.Location = location;
		System.Windows.Forms.TextBox tw2 = this.Tw;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw2.Margin = margin;
		this.Tw.Name = "Tw";
		this.Tw.ReadOnly = true;
		System.Windows.Forms.TextBox tw3 = this.Tw;
		size = new System.Drawing.Size(892, 23);
		tw3.Size = size;
		this.Tw.TabIndex = 1;
		this.Label48.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label48;
		location = new System.Drawing.Point(10, 15);
		label19.Location = location;
		this.Label48.Name = "Label48";
		System.Windows.Forms.Label label20 = this.Label48;
		size = new System.Drawing.Size(68, 16);
		label20.Size = size;
		this.Label48.TabIndex = 47;
		this.Label48.Text = "ช\u0e37\u0e48อท\u0e35\u0e48ทำงาน";
		this.Tw_soi.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_soi = this.Tw_soi;
		location = new System.Drawing.Point(255, 43);
		tw_soi.Location = location;
		System.Windows.Forms.TextBox tw_soi2 = this.Tw_soi;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_soi2.Margin = margin;
		this.Tw_soi.Name = "Tw_soi";
		this.Tw_soi.ReadOnly = true;
		System.Windows.Forms.TextBox tw_soi3 = this.Tw_soi;
		size = new System.Drawing.Size(129, 23);
		tw_soi3.Size = size;
		this.Tw_soi.TabIndex = 1;
		this.Label45.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label45;
		location = new System.Drawing.Point(220, 46);
		label21.Location = location;
		this.Label45.Name = "Label45";
		System.Windows.Forms.Label label22 = this.Label45;
		size = new System.Drawing.Size(33, 16);
		label22.Size = size;
		this.Label45.TabIndex = 47;
		this.Label45.Text = "ซอย";
		this.Tw_moo.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_moo = this.Tw_moo;
		location = new System.Drawing.Point(170, 43);
		tw_moo.Location = location;
		System.Windows.Forms.TextBox tw_moo2 = this.Tw_moo;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_moo2.Margin = margin;
		this.Tw_moo.Name = "Tw_moo";
		this.Tw_moo.ReadOnly = true;
		System.Windows.Forms.TextBox tw_moo3 = this.Tw_moo;
		size = new System.Drawing.Size(46, 23);
		tw_moo3.Size = size;
		this.Tw_moo.TabIndex = 1;
		this.Label46.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label46;
		location = new System.Drawing.Point(135, 46);
		label23.Location = location;
		this.Label46.Name = "Label46";
		System.Windows.Forms.Label label24 = this.Label46;
		size = new System.Drawing.Size(33, 16);
		label24.Size = size;
		this.Label46.TabIndex = 47;
		this.Label46.Text = "หม\u0e39\u0e48ท\u0e35\u0e48";
		this.Tw_no.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_no = this.Tw_no;
		location = new System.Drawing.Point(80, 43);
		tw_no.Location = location;
		System.Windows.Forms.TextBox tw_no2 = this.Tw_no;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_no2.Margin = margin;
		this.Tw_no.Name = "Tw_no";
		this.Tw_no.ReadOnly = true;
		System.Windows.Forms.TextBox tw_no3 = this.Tw_no;
		size = new System.Drawing.Size(46, 23);
		tw_no3.Size = size;
		this.Tw_no.TabIndex = 1;
		this.Label47.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label47;
		location = new System.Drawing.Point(42, 47);
		label25.Location = location;
		this.Label47.Name = "Label47";
		System.Windows.Forms.Label label26 = this.Label47;
		size = new System.Drawing.Size(37, 16);
		label26.Size = size;
		this.Label47.TabIndex = 47;
		this.Label47.Text = "เลขท\u0e35\u0e48";
		this.Tw_fax.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tw_fax = this.Tw_fax;
		location = new System.Drawing.Point(628, 71);
		tw_fax.Location = location;
		System.Windows.Forms.TextBox tw_fax2 = this.Tw_fax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tw_fax2.Margin = margin;
		this.Tw_fax.Name = "Tw_fax";
		this.Tw_fax.ReadOnly = true;
		System.Windows.Forms.TextBox tw_fax3 = this.Tw_fax;
		size = new System.Drawing.Size(130, 23);
		tw_fax3.Size = size;
		this.Tw_fax.TabIndex = 1;
		this.expandablePanel4.CanvasColor = System.Drawing.SystemColors.Control;
		this.expandablePanel4.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.expandablePanel4.Controls.Add(this.Panel1);
		this.expandablePanel4.Dock = System.Windows.Forms.DockStyle.Top;
		this.expandablePanel4.ExpandOnTitleClick = true;
		this.expandablePanel4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ExpandablePanel expandablePanel3 = this.expandablePanel4;
		location = new System.Drawing.Point(0, 0);
		expandablePanel3.Location = location;
		this.expandablePanel4.Name = "expandablePanel4";
		DevComponents.DotNetBar.ExpandablePanel expandablePanel4 = this.expandablePanel4;
		size = new System.Drawing.Size(1187, 130);
		expandablePanel4.Size = size;
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
		this.Panel1.Controls.Add(this.Label6);
		this.Panel1.Controls.Add(this.Tcontry);
		this.Panel1.Controls.Add(this.Tc_ampore);
		this.Panel1.Controls.Add(this.Label39);
		this.Panel1.Controls.Add(this.Tc_tambon);
		this.Panel1.Controls.Add(this.Label38);
		this.Panel1.Controls.Add(this.Tc_tel);
		this.Panel1.Controls.Add(this.Tc_code);
		this.Panel1.Controls.Add(this.Label41);
		this.Panel1.Controls.Add(this.Tc_road);
		this.Panel1.Controls.Add(this.Tc_province);
		this.Panel1.Controls.Add(this.Label37);
		this.Panel1.Controls.Add(this.Label10);
		this.Panel1.Controls.Add(this.Label40);
		this.Panel1.Controls.Add(this.Label9);
		this.Panel1.Controls.Add(this.Tc_soi);
		this.Panel1.Controls.Add(this.Label36);
		this.Panel1.Controls.Add(this.Tc_moo);
		this.Panel1.Controls.Add(this.Label11);
		this.Panel1.Controls.Add(this.Tc_no);
		this.Panel1.Controls.Add(this.Label8);
		this.Panel1.Controls.Add(this.Tc_fax);
		this.Panel1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.Panel1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		System.Windows.Forms.Panel panel5 = this.Panel1;
		location = new System.Drawing.Point(0, 26);
		panel5.Location = location;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel6 = this.Panel1;
		size = new System.Drawing.Size(1187, 104);
		panel6.Size = size;
		this.Panel1.TabIndex = 1;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label6;
		location = new System.Drawing.Point(362, 60);
		label27.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label28 = this.Label6;
		size = new System.Drawing.Size(49, 16);
		label28.Size = size;
		this.Label6.TabIndex = 53;
		this.Label6.Text = "ประเทศ";
		this.Tcontry.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tcontry = this.Tcontry;
		location = new System.Drawing.Point(412, 56);
		tcontry.Location = location;
		System.Windows.Forms.TextBox tcontry2 = this.Tcontry;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcontry2.Margin = margin;
		this.Tcontry.Name = "Tcontry";
		this.Tcontry.ReadOnly = true;
		System.Windows.Forms.TextBox tcontry3 = this.Tcontry;
		size = new System.Drawing.Size(129, 23);
		tcontry3.Size = size;
		this.Tcontry.TabIndex = 52;
		this.Tc_ampore.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_ampore = this.Tc_ampore;
		location = new System.Drawing.Point(832, 28);
		tc_ampore.Location = location;
		System.Windows.Forms.TextBox tc_ampore2 = this.Tc_ampore;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_ampore2.Margin = margin;
		this.Tc_ampore.Name = "Tc_ampore";
		this.Tc_ampore.ReadOnly = true;
		System.Windows.Forms.TextBox tc_ampore3 = this.Tc_ampore;
		size = new System.Drawing.Size(130, 23);
		tc_ampore3.Size = size;
		this.Tc_ampore.TabIndex = 1;
		this.Label39.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label39;
		location = new System.Drawing.Point(763, 31);
		label29.Location = location;
		this.Label39.Name = "Label39";
		System.Windows.Forms.Label label30 = this.Label39;
		size = new System.Drawing.Size(67, 16);
		label30.Size = size;
		this.Label39.TabIndex = 47;
		this.Label39.Text = "เขต/อำเภอ";
		this.Tc_tambon.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_tambon = this.Tc_tambon;
		location = new System.Drawing.Point(618, 28);
		tc_tambon.Location = location;
		System.Windows.Forms.TextBox tc_tambon2 = this.Tc_tambon;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_tambon2.Margin = margin;
		this.Tc_tambon.Name = "Tc_tambon";
		this.Tc_tambon.ReadOnly = true;
		System.Windows.Forms.TextBox tc_tambon3 = this.Tc_tambon;
		size = new System.Drawing.Size(130, 23);
		tc_tambon3.Size = size;
		this.Tc_tambon.TabIndex = 1;
		this.Label38.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label38;
		location = new System.Drawing.Point(545, 30);
		label31.Location = location;
		this.Label38.Name = "Label38";
		System.Windows.Forms.Label label32 = this.Label38;
		size = new System.Drawing.Size(71, 16);
		label32.Size = size;
		this.Label38.TabIndex = 47;
		this.Label38.Text = "แขวง/ตำบล";
		this.Tc_tel.Enabled = false;
		this.Tc_tel.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tc_tel = this.Tc_tel;
		location = new System.Drawing.Point(618, 57);
		tc_tel.Location = location;
		System.Windows.Forms.ComboBox tc_tel2 = this.Tc_tel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_tel2.Margin = margin;
		this.Tc_tel.Name = "Tc_tel";
		System.Windows.Forms.ComboBox tc_tel3 = this.Tc_tel;
		size = new System.Drawing.Size(170, 24);
		tc_tel3.Size = size;
		this.Tc_tel.TabIndex = 51;
		this.Tc_code.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_code = this.Tc_code;
		location = new System.Drawing.Point(266, 56);
		tc_code.Location = location;
		System.Windows.Forms.TextBox tc_code2 = this.Tc_code;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_code2.Margin = margin;
		this.Tc_code.Name = "Tc_code";
		this.Tc_code.ReadOnly = true;
		System.Windows.Forms.TextBox tc_code3 = this.Tc_code;
		size = new System.Drawing.Size(94, 23);
		tc_code3.Size = size;
		this.Tc_code.TabIndex = 1;
		this.Label41.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label41;
		location = new System.Drawing.Point(187, 60);
		label33.Location = location;
		this.Label41.Name = "Label41";
		System.Windows.Forms.Label label34 = this.Label41;
		size = new System.Drawing.Size(80, 16);
		label34.Size = size;
		this.Label41.TabIndex = 47;
		this.Label41.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.Tc_road.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_road = this.Tc_road;
		location = new System.Drawing.Point(412, 28);
		tc_road.Location = location;
		System.Windows.Forms.TextBox tc_road2 = this.Tc_road;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_road2.Margin = margin;
		this.Tc_road.Name = "Tc_road";
		this.Tc_road.ReadOnly = true;
		System.Windows.Forms.TextBox tc_road3 = this.Tc_road;
		size = new System.Drawing.Size(129, 23);
		tc_road3.Size = size;
		this.Tc_road.TabIndex = 1;
		this.Tc_province.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_province = this.Tc_province;
		location = new System.Drawing.Point(56, 56);
		tc_province.Location = location;
		System.Windows.Forms.TextBox tc_province2 = this.Tc_province;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_province2.Margin = margin;
		this.Tc_province.Name = "Tc_province";
		this.Tc_province.ReadOnly = true;
		System.Windows.Forms.TextBox tc_province3 = this.Tc_province;
		size = new System.Drawing.Size(125, 23);
		tc_province3.Size = size;
		this.Tc_province.TabIndex = 1;
		this.Label37.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label37;
		location = new System.Drawing.Point(377, 31);
		label35.Location = location;
		this.Label37.Name = "Label37";
		System.Windows.Forms.Label label36 = this.Label37;
		size = new System.Drawing.Size(32, 16);
		label36.Size = size;
		this.Label37.TabIndex = 47;
		this.Label37.Text = "ถนน";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label10;
		location = new System.Drawing.Point(802, 61);
		label37.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label38 = this.Label10;
		size = new System.Drawing.Size(28, 16);
		label38.Size = size;
		this.Label10.TabIndex = 47;
		this.Label10.Text = "Fax";
		this.Label40.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label40;
		location = new System.Drawing.Point(11, 60);
		label39.Location = location;
		this.Label40.Name = "Label40";
		System.Windows.Forms.Label label40 = this.Label40;
		size = new System.Drawing.Size(43, 16);
		label40.Size = size;
		this.Label40.TabIndex = 47;
		this.Label40.Text = "จ\u0e31งหว\u0e31ด";
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label9;
		location = new System.Drawing.Point(587, 62);
		label41.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label42 = this.Label9;
		size = new System.Drawing.Size(29, 16);
		label42.Size = size;
		this.Label9.TabIndex = 47;
		this.Label9.Text = "โทร";
		this.Tc_soi.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_soi = this.Tc_soi;
		location = new System.Drawing.Point(231, 28);
		tc_soi.Location = location;
		System.Windows.Forms.TextBox tc_soi2 = this.Tc_soi;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_soi2.Margin = margin;
		this.Tc_soi.Name = "Tc_soi";
		this.Tc_soi.ReadOnly = true;
		System.Windows.Forms.TextBox tc_soi3 = this.Tc_soi;
		size = new System.Drawing.Size(129, 23);
		tc_soi3.Size = size;
		this.Tc_soi.TabIndex = 1;
		this.Label36.AutoSize = true;
		System.Windows.Forms.Label label43 = this.Label36;
		location = new System.Drawing.Point(196, 31);
		label43.Location = location;
		this.Label36.Name = "Label36";
		System.Windows.Forms.Label label44 = this.Label36;
		size = new System.Drawing.Size(33, 16);
		label44.Size = size;
		this.Label36.TabIndex = 47;
		this.Label36.Text = "ซอย";
		this.Tc_moo.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_moo = this.Tc_moo;
		location = new System.Drawing.Point(146, 28);
		tc_moo.Location = location;
		System.Windows.Forms.TextBox tc_moo2 = this.Tc_moo;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_moo2.Margin = margin;
		this.Tc_moo.Name = "Tc_moo";
		this.Tc_moo.ReadOnly = true;
		System.Windows.Forms.TextBox tc_moo3 = this.Tc_moo;
		size = new System.Drawing.Size(46, 23);
		tc_moo3.Size = size;
		this.Tc_moo.TabIndex = 1;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label45 = this.Label11;
		location = new System.Drawing.Point(111, 31);
		label45.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label46 = this.Label11;
		size = new System.Drawing.Size(33, 16);
		label46.Size = size;
		this.Label11.TabIndex = 47;
		this.Label11.Text = "หม\u0e39\u0e48ท\u0e35\u0e48";
		this.Tc_no.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_no = this.Tc_no;
		location = new System.Drawing.Point(56, 28);
		tc_no.Location = location;
		System.Windows.Forms.TextBox tc_no2 = this.Tc_no;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_no2.Margin = margin;
		this.Tc_no.Name = "Tc_no";
		this.Tc_no.ReadOnly = true;
		System.Windows.Forms.TextBox tc_no3 = this.Tc_no;
		size = new System.Drawing.Size(46, 23);
		tc_no3.Size = size;
		this.Tc_no.TabIndex = 1;
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label47 = this.Label8;
		location = new System.Drawing.Point(17, 32);
		label47.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label48 = this.Label8;
		size = new System.Drawing.Size(37, 16);
		label48.Size = size;
		this.Label8.TabIndex = 47;
		this.Label8.Text = "เลขท\u0e35\u0e48";
		this.Tc_fax.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tc_fax = this.Tc_fax;
		location = new System.Drawing.Point(832, 58);
		tc_fax.Location = location;
		System.Windows.Forms.TextBox tc_fax2 = this.Tc_fax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_fax2.Margin = margin;
		this.Tc_fax.Name = "Tc_fax";
		this.Tc_fax.ReadOnly = true;
		System.Windows.Forms.TextBox tc_fax3 = this.Tc_fax;
		size = new System.Drawing.Size(130, 23);
		tc_fax3.Size = size;
		this.Tc_fax.TabIndex = 1;
		this.TCarType.Enabled = false;
		this.TCarType.FormattingEnabled = true;
		this.TCarType.Items.AddRange(new object[4] { "รถเก\u0e4bง", "รถกระบะ", "รถต\u0e39\u0e49", "รถบ\u0e31ส" });
		System.Windows.Forms.ComboBox tCarType = this.TCarType;
		location = new System.Drawing.Point(519, 80);
		tCarType.Location = location;
		System.Windows.Forms.ComboBox tCarType2 = this.TCarType;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCarType2.Margin = margin;
		this.TCarType.Name = "TCarType";
		System.Windows.Forms.ComboBox tCarType3 = this.TCarType;
		size = new System.Drawing.Size(170, 24);
		tCarType3.Size = size;
		this.TCarType.TabIndex = 51;
		this.Label31.AutoSize = true;
		System.Windows.Forms.Label label49 = this.Label31;
		location = new System.Drawing.Point(455, 85);
		label49.Location = location;
		this.Label31.Name = "Label31";
		System.Windows.Forms.Label label50 = this.Label31;
		size = new System.Drawing.Size(63, 16);
		label50.Size = size;
		this.Label31.TabIndex = 50;
		this.Label31.Text = "ประเภทรถ";
		this.TCusType.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TCusType.Enabled = false;
		this.TCusType.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tCusType = this.TCusType;
		location = new System.Drawing.Point(119, 79);
		tCusType.Location = location;
		System.Windows.Forms.ComboBox tCusType2 = this.TCusType;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusType2.Margin = margin;
		this.TCusType.Name = "TCusType";
		System.Windows.Forms.ComboBox tCusType3 = this.TCusType;
		size = new System.Drawing.Size(144, 24);
		tCusType3.Size = size;
		this.TCusType.TabIndex = 51;
		this.Label21.AutoSize = true;
		System.Windows.Forms.Label label51 = this.Label21;
		location = new System.Drawing.Point(55, 83);
		label51.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label52 = this.Label21;
		size = new System.Drawing.Size(62, 16);
		label52.Size = size;
		this.Label21.TabIndex = 50;
		this.Label21.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.DateTimePicker1.CustomFormat = "ddMMMMyyyy เวลา HH:mm";
		this.DateTimePicker1.Enabled = false;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(519, 20);
		dateTimePicker.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker2.Margin = margin;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		size = new System.Drawing.Size(170, 23);
		dateTimePicker3.Size = size;
		this.DateTimePicker1.TabIndex = 48;
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label53 = this.Label16;
		location = new System.Drawing.Point(485, 23);
		label53.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label54 = this.Label16;
		size = new System.Drawing.Size(31, 16);
		label54.Size = size;
		this.Label16.TabIndex = 47;
		this.Label16.Text = "ว\u0e31นท\u0e35\u0e48";
		this.Label29.AutoSize = true;
		System.Windows.Forms.Label label55 = this.Label29;
		location = new System.Drawing.Point(269, 85);
		label55.Location = location;
		this.Label29.Name = "Label29";
		System.Windows.Forms.Label label56 = this.Label29;
		size = new System.Drawing.Size(65, 16);
		label56.Size = size;
		this.Label29.TabIndex = 47;
		this.Label29.Text = "ทะเบ\u0e35ยนรถ";
		this.Label27.AutoSize = true;
		System.Windows.Forms.Label label57 = this.Label27;
		location = new System.Drawing.Point(710, 84);
		label57.Location = location;
		this.Label27.Name = "Label27";
		System.Windows.Forms.Label label58 = this.Label27;
		size = new System.Drawing.Size(39, 16);
		label58.Size = size;
		this.Label27.TabIndex = 47;
		this.Label27.Text = "Email";
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label59 = this.Label26;
		location = new System.Drawing.Point(464, 53);
		label59.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label60 = this.Label26;
		size = new System.Drawing.Size(54, 16);
		label60.Size = size;
		this.Label26.TabIndex = 47;
		this.Label26.Text = "นามสก\u0e38ล";
		this.Label50.AutoSize = true;
		System.Windows.Forms.Label label61 = this.Label50;
		location = new System.Drawing.Point(57, 53);
		label61.Location = location;
		this.Label50.Name = "Label50";
		System.Windows.Forms.Label label62 = this.Label50;
		size = new System.Drawing.Size(60, 16);
		label62.Size = size;
		this.Label50.TabIndex = 47;
		this.Label50.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label63 = this.Label7;
		location = new System.Drawing.Point(276, 53);
		label63.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label64 = this.Label7;
		size = new System.Drawing.Size(24, 16);
		label64.Size = size;
		this.Label7.TabIndex = 47;
		this.Label7.Text = "ช\u0e37\u0e48อ";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label65 = this.Label1;
		location = new System.Drawing.Point(1, 25);
		label65.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label66 = this.Label1;
		size = new System.Drawing.Size(116, 16);
		label66.Size = size;
		this.Label1.TabIndex = 47;
		this.Label1.Text = "บ\u0e31ตรลงทะเบ\u0e35ยนเลขท\u0e35\u0e48";
		this.TCarID.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tCarID = this.TCarID;
		location = new System.Drawing.Point(336, 80);
		tCarID.Location = location;
		System.Windows.Forms.TextBox tCarID2 = this.TCarID;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCarID2.Margin = margin;
		this.TCarID.Name = "TCarID";
		this.TCarID.ReadOnly = true;
		System.Windows.Forms.TextBox tCarID3 = this.TCarID;
		size = new System.Drawing.Size(116, 23);
		tCarID3.Size = size;
		this.TCarID.TabIndex = 1;
		this.TextBox_0.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox textBox_ = this.TextBox_0;
		location = new System.Drawing.Point(750, 80);
		textBox_.Location = location;
		System.Windows.Forms.TextBox textBox_2 = this.TextBox_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_2.Margin = margin;
		this.TextBox_0.Name = "TCusEmail";
		this.TextBox_0.ReadOnly = true;
		System.Windows.Forms.TextBox textBox_3 = this.TextBox_0;
		size = new System.Drawing.Size(170, 23);
		textBox_3.Size = size;
		this.TextBox_0.TabIndex = 1;
		this.TextBox_1.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TextBox_1.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox textBox_4 = this.TextBox_1;
		location = new System.Drawing.Point(519, 49);
		textBox_4.Location = location;
		System.Windows.Forms.TextBox textBox_5 = this.TextBox_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_5.Margin = margin;
		this.TextBox_1.Name = "TCusName2";
		this.TextBox_1.ReadOnly = true;
		System.Windows.Forms.TextBox textBox_6 = this.TextBox_1;
		size = new System.Drawing.Size(170, 23);
		textBox_6.Size = size;
		this.TextBox_1.TabIndex = 1;
		this.TCusID.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TCusID.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tCusID = this.TCusID;
		location = new System.Drawing.Point(119, 49);
		tCusID.Location = location;
		System.Windows.Forms.TextBox tCusID2 = this.TCusID;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusID2.Margin = margin;
		this.TCusID.Name = "TCusID";
		this.TCusID.ReadOnly = true;
		System.Windows.Forms.TextBox tCusID3 = this.TCusID;
		size = new System.Drawing.Size(144, 23);
		tCusID3.Size = size;
		this.TCusID.TabIndex = 1;
		this.TcusName.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TcusName.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tcusName = this.TcusName;
		location = new System.Drawing.Point(302, 49);
		tcusName.Location = location;
		System.Windows.Forms.TextBox tcusName2 = this.TcusName;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcusName2.Margin = margin;
		this.TcusName.Name = "TcusName";
		this.TcusName.ReadOnly = true;
		System.Windows.Forms.TextBox tcusName3 = this.TcusName;
		size = new System.Drawing.Size(150, 23);
		tcusName3.Size = size;
		this.TcusName.TabIndex = 1;
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
		this.TdocNum.TabIndex = 1;
		this.Button6.Image = (System.Drawing.Image)resources.GetObject("Button6.Image");
		this.Button6.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button = this.Button6;
		location = new System.Drawing.Point(373, 17);
		button.Location = location;
		System.Windows.Forms.Button button2 = this.Button6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button2.Margin = margin;
		this.Button6.Name = "Button6";
		System.Windows.Forms.Button button3 = this.Button6;
		size = new System.Drawing.Size(81, 24);
		button3.Size = size;
		this.Button6.TabIndex = 3;
		this.Button6.Text = "     ยกเล\u0e34ก";
		this.Button6.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button6.UseVisualStyleBackColor = true;
		this.Button6.Visible = false;
		this.Button1.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button4 = this.Button1;
		location = new System.Drawing.Point(267, 20);
		button4.Location = location;
		System.Windows.Forms.Button button5 = this.Button1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button5.Margin = margin;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button6 = this.Button1;
		size = new System.Drawing.Size(37, 24);
		button6.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.UseVisualStyleBackColor = true;
		this.Button1.Visible = false;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Button14);
		this.PanelEx1.Controls.Add(this.Button13);
		this.PanelEx1.Controls.Add(this.Button15);
		this.PanelEx1.Controls.Add(this.Button16);
		this.PanelEx1.Controls.Add(this.POver);
		this.PanelEx1.Controls.Add(this.LOver);
		this.PanelEx1.Controls.Add(this.Tnote);
		this.PanelEx1.Controls.Add(this.Tdebt);
		this.PanelEx1.Controls.Add(this.Tcash);
		this.PanelEx1.Controls.Add(this.Tpay);
		this.PanelEx1.Controls.Add(this.TselectRoom);
		this.PanelEx1.Controls.Add(this.Label55);
		this.PanelEx1.Controls.Add(this.Grid2);
		this.PanelEx1.Controls.Add(this.Grid1);
		this.PanelEx1.Controls.Add(this.Button9);
		this.PanelEx1.Controls.Add(this.Label35);
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
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Controls.Add(this.Button5);
		this.PanelEx1.Controls.Add(this.Label12);
		this.PanelEx1.Controls.Add(this.Button7);
		this.PanelEx1.Controls.Add(this.Label14);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(1223, 761);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 50;
		this.Button14.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button7 = this.Button14;
		location = new System.Drawing.Point(1096, 283);
		button7.Location = location;
		System.Windows.Forms.Button button8 = this.Button14;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button8.Margin = margin;
		this.Button14.Name = "Button14";
		System.Windows.Forms.Button button9 = this.Button14;
		size = new System.Drawing.Size(109, 24);
		button9.Size = size;
		this.Button14.TabIndex = 99;
		this.Button14.Text = "ไม\u0e48จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button14.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button14.UseVisualStyleBackColor = true;
		this.Button13.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button10 = this.Button13;
		location = new System.Drawing.Point(1009, 283);
		button10.Location = location;
		System.Windows.Forms.Button button11 = this.Button13;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button11.Margin = margin;
		this.Button13.Name = "Button13";
		System.Windows.Forms.Button button12 = this.Button13;
		size = new System.Drawing.Size(87, 24);
		button12.Size = size;
		this.Button13.TabIndex = 98;
		this.Button13.Text = "จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button13.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button13.UseVisualStyleBackColor = true;
		this.Button15.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button13 = this.Button15;
		location = new System.Drawing.Point(858, 463);
		button13.Location = location;
		System.Windows.Forms.Button button14 = this.Button15;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button14.Margin = margin;
		this.Button15.Name = "Button15";
		System.Windows.Forms.Button button15 = this.Button15;
		size = new System.Drawing.Size(109, 24);
		button15.Size = size;
		this.Button15.TabIndex = 97;
		this.Button15.Text = "ไม\u0e48จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button15.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button15.UseVisualStyleBackColor = true;
		this.Button16.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button16 = this.Button16;
		location = new System.Drawing.Point(771, 463);
		button16.Location = location;
		System.Windows.Forms.Button button17 = this.Button16;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button17.Margin = margin;
		this.Button16.Name = "Button16";
		System.Windows.Forms.Button button18 = this.Button16;
		size = new System.Drawing.Size(87, 24);
		button18.Size = size;
		this.Button16.TabIndex = 96;
		this.Button16.Text = "จ\u0e48ายท\u0e31\u0e49งหมด";
		this.Button16.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button16.UseVisualStyleBackColor = true;
		this.POver.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.POver.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.POver.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.POver.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.POver.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox pOver = this.POver;
		location = new System.Drawing.Point(155, 729);
		pOver.Location = location;
		System.Windows.Forms.TextBox pOver2 = this.POver;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		pOver2.Margin = margin;
		this.POver.Name = "POver";
		this.POver.ReadOnly = true;
		System.Windows.Forms.TextBox pOver3 = this.POver;
		size = new System.Drawing.Size(129, 26);
		pOver3.Size = size;
		this.POver.TabIndex = 89;
		this.POver.Text = "0.00";
		this.POver.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LOver.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LOver.AutoSize = true;
		this.LOver.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label lOver = this.LOver;
		location = new System.Drawing.Point(22, 735);
		lOver.Location = location;
		this.LOver.Name = "LOver";
		System.Windows.Forms.Label lOver2 = this.LOver;
		size = new System.Drawing.Size(131, 16);
		lOver2.Size = size;
		this.LOver.TabIndex = 88;
		this.LOver.Text = "เง\u0e34นจ\u0e48ายล\u0e48วงหน\u0e49าคงเหล\u0e37อ";
		this.Tnote.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tnote.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tnote.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tnote.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox tnote = this.Tnote;
		location = new System.Drawing.Point(1075, 681);
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
		location = new System.Drawing.Point(1075, 653);
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
		this.Tcash.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tcash.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tcash.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tcash.Font = new System.Drawing.Font("Times New Roman", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tcash.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox tcash = this.Tcash;
		location = new System.Drawing.Point(1075, 625);
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
		location = new System.Drawing.Point(1075, 597);
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
		location = new System.Drawing.Point(204, 461);
		tselectRoom.Location = location;
		System.Windows.Forms.ComboBox tselectRoom2 = this.TselectRoom;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tselectRoom2.Margin = margin;
		this.TselectRoom.Name = "TselectRoom";
		System.Windows.Forms.ComboBox tselectRoom3 = this.TselectRoom;
		size = new System.Drawing.Size(121, 24);
		tselectRoom3.Size = size;
		this.TselectRoom.TabIndex = 86;
		this.TselectRoom.Visible = false;
		this.Label55.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label55.AutoSize = true;
		System.Windows.Forms.Label label67 = this.Label55;
		location = new System.Drawing.Point(152, 466);
		label67.Location = location;
		this.Label55.Name = "Label55";
		System.Windows.Forms.Label label68 = this.Label55;
		size = new System.Drawing.Size(50, 16);
		label68.Size = size;
		this.Label55.TabIndex = 80;
		this.Label55.Text = "เลขห\u0e49อง";
		this.Label55.Visible = false;
		this.Grid2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Grid2.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid2.ColumnInfo = resources.GetString("Grid2.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid = this.Grid2;
		location = new System.Drawing.Point(17, 488);
		grid.Location = location;
		this.Grid2.Name = "Grid2";
		this.Grid2.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid2 = this.Grid2;
		size = new System.Drawing.Size(949, 237);
		grid2.Size = size;
		this.Grid2.StyleInfo = resources.GetString("Grid2.StyleInfo");
		this.Grid2.TabIndex = 78;
		this.Grid2.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.Grid1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Grid1.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid1.ColumnInfo = resources.GetString("Grid1.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid3 = this.Grid1;
		location = new System.Drawing.Point(16, 308);
		grid3.Location = location;
		this.Grid1.Name = "Grid1";
		this.Grid1.Rows.Count = 500;
		this.Grid1.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid4 = this.Grid1;
		size = new System.Drawing.Size(1188, 151);
		grid4.Size = size;
		this.Grid1.StyleInfo = resources.GetString("Grid1.StyleInfo");
		this.Grid1.TabIndex = 76;
		this.Grid1.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.Button9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button9.Image = (System.Drawing.Image)resources.GetObject("Button9.Image");
		this.Button9.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button19 = this.Button9;
		location = new System.Drawing.Point(381, 461);
		button19.Location = location;
		System.Windows.Forms.Button button20 = this.Button9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button20.Margin = margin;
		this.Button9.Name = "Button9";
		System.Windows.Forms.Button button21 = this.Button9;
		size = new System.Drawing.Size(52, 24);
		button21.Size = size;
		this.Button9.TabIndex = 75;
		this.Button9.Text = "    ลบ";
		this.Button9.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button9.UseVisualStyleBackColor = true;
		this.Button9.Visible = false;
		this.Label35.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label35.AutoSize = true;
		this.Label35.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label69 = this.Label35;
		location = new System.Drawing.Point(974, 685);
		label69.Location = location;
		this.Label35.Name = "Label35";
		System.Windows.Forms.Label label70 = this.Label35;
		size = new System.Drawing.Size(99, 16);
		label70.Size = size;
		this.Label35.TabIndex = 53;
		this.Label35.Text = "หมายเหต\u0e38การจ\u0e48าย";
		this.Label30.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label30.AutoSize = true;
		this.Label30.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label71 = this.Label30;
		location = new System.Drawing.Point(1003, 658);
		label71.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label72 = this.Label30;
		size = new System.Drawing.Size(70, 16);
		label72.Size = size;
		this.Label30.TabIndex = 53;
		this.Label30.Text = "เครด\u0e34ตการ\u0e4cด";
		this.Label30.Visible = false;
		this.Label23.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label23.AutoSize = true;
		this.Label23.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label73 = this.Label23;
		location = new System.Drawing.Point(1030, 631);
		label73.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label74 = this.Label23;
		size = new System.Drawing.Size(42, 16);
		label74.Size = size;
		this.Label23.TabIndex = 53;
		this.Label23.Text = "เง\u0e34นสด";
		this.Label23.Visible = false;
		this.LabelDebt.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelDebt.BackColor = System.Drawing.Color.Black;
		this.LabelDebt.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelDebt.ForeColor = System.Drawing.Color.FromArgb(255, 128, 128);
		System.Windows.Forms.Label labelDebt = this.LabelDebt;
		location = new System.Drawing.Point(1075, 568);
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
		System.Windows.Forms.Label label75 = this.Label18;
		location = new System.Drawing.Point(972, 602);
		label75.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label76 = this.Label18;
		size = new System.Drawing.Size(101, 16);
		label76.Size = size;
		this.Label18.TabIndex = 53;
		this.Label18.Text = "รวมยอดจ\u0e48ายคร\u0e31\u0e49งน\u0e35\u0e49";
		this.LabelPayed.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelPayed.BackColor = System.Drawing.Color.Black;
		this.LabelPayed.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelPayed.ForeColor = System.Drawing.Color.Yellow;
		this.LabelPayed.Image = (System.Drawing.Image)resources.GetObject("LabelPayed.Image");
		this.LabelPayed.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Label labelPayed = this.LabelPayed;
		location = new System.Drawing.Point(1075, 542);
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
		System.Windows.Forms.Label label77 = this.Label33;
		location = new System.Drawing.Point(1030, 573);
		label77.Location = location;
		this.Label33.Name = "Label33";
		System.Windows.Forms.Label label78 = this.Label33;
		size = new System.Drawing.Size(42, 16);
		label78.Size = size;
		this.Label33.TabIndex = 53;
		this.Label33.Text = "คงค\u0e49าง";
		this.Labelroompro.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Labelroompro.BackColor = System.Drawing.Color.Navy;
		this.Labelroompro.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Labelroompro.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Labelroompro.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label labelroompro = this.Labelroompro;
		location = new System.Drawing.Point(1075, 514);
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
		System.Windows.Forms.Label label79 = this.Label20;
		location = new System.Drawing.Point(997, 546);
		label79.Location = location;
		this.Label20.Name = "Label20";
		System.Windows.Forms.Label label80 = this.Label20;
		size = new System.Drawing.Size(76, 16);
		label80.Size = size;
		this.Label20.TabIndex = 53;
		this.Label20.Text = "รวมชำระแล\u0e49ว";
		this.LabelTpro.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelTpro.BackColor = System.Drawing.Color.Navy;
		this.LabelTpro.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabelTpro.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelTpro.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label labelTpro = this.LabelTpro;
		location = new System.Drawing.Point(1075, 488);
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
		System.Windows.Forms.Label label81 = this.Label17;
		location = new System.Drawing.Point(1006, 518);
		label81.Location = location;
		this.Label17.Name = "Label17";
		System.Windows.Forms.Label label82 = this.Label17;
		size = new System.Drawing.Size(67, 16);
		label82.Size = size;
		this.Label17.TabIndex = 53;
		this.Label17.Text = "รวมท\u0e31\u0e49งหมด";
		this.LabelTroom.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelTroom.BackColor = System.Drawing.Color.Navy;
		this.LabelTroom.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabelTroom.Font = new System.Drawing.Font("Times New Roman", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelTroom.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label labelTroom = this.LabelTroom;
		location = new System.Drawing.Point(1075, 462);
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
		System.Windows.Forms.Label label83 = this.Label13;
		location = new System.Drawing.Point(970, 492);
		label83.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label84 = this.Label13;
		size = new System.Drawing.Size(103, 16);
		label84.Size = size;
		this.Label13.TabIndex = 53;
		this.Label13.Text = "รวมราคาค\u0e48าใช\u0e49จ\u0e48าย";
		this.Label24.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label24.AutoSize = true;
		this.Label24.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label85 = this.Label24;
		location = new System.Drawing.Point(980, 467);
		label85.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label86 = this.Label24;
		size = new System.Drawing.Size(93, 16);
		label86.Size = size;
		this.Label24.TabIndex = 53;
		this.Label24.Text = "รวมราคาห\u0e49องพ\u0e31ก";
		this.Button5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button5.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button22 = this.Button5;
		location = new System.Drawing.Point(328, 461);
		button22.Location = location;
		System.Windows.Forms.Button button23 = this.Button5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button23.Margin = margin;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button24 = this.Button5;
		size = new System.Drawing.Size(51, 24);
		button24.Size = size;
		this.Button5.TabIndex = 3;
		this.Button5.UseVisualStyleBackColor = true;
		this.Button5.Visible = false;
		this.Label12.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label12.AutoSize = true;
		this.Label12.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label87 = this.Label12;
		location = new System.Drawing.Point(17, 465);
		label87.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label88 = this.Label12;
		size = new System.Drawing.Size(133, 16);
		label88.Size = size;
		this.Label12.TabIndex = 47;
		this.Label12.Text = "ค\u0e48าใช\u0e49จ\u0e48ายเพ\u0e34\u0e48มเต\u0e34มอ\u0e37\u0e48นๆ";
		this.Button7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Button7.Image = (System.Drawing.Image)resources.GetObject("Button7.Image");
		this.Button7.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button25 = this.Button7;
		location = new System.Drawing.Point(970, 709);
		button25.Location = location;
		System.Windows.Forms.Button button26 = this.Button7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button26.Margin = margin;
		this.Button7.Name = "Button7";
		System.Windows.Forms.Button button27 = this.Button7;
		size = new System.Drawing.Size(235, 44);
		button27.Size = size;
		this.Button7.TabIndex = 3;
		this.Button7.Text = "          บ\u0e31นท\u0e36ก การชำระเง\u0e34น";
		this.Button7.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button7.UseVisualStyleBackColor = true;
		this.Label14.AutoSize = true;
		this.Label14.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label89 = this.Label14;
		location = new System.Drawing.Point(13, 283);
		label89.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label90 = this.Label14;
		size = new System.Drawing.Size(78, 16);
		label90.Size = size;
		this.Label14.TabIndex = 47;
		this.Label14.Text = "รายการห\u0e49อง";
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
		this.SuperTooltip1.AntiAlias = false;
		this.SuperTooltip1.DefaultFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.SuperTooltip1.MaximumWidth = 500;
		DevComponents.DotNetBar.SuperTooltip superTooltip = this.SuperTooltip1;
		size = new System.Drawing.Size(400, 24);
		superTooltip.MinimumTooltipSize = size;
		this.SuperTooltip1.ShowTooltipImmediately = true;
		this.SuperTooltip1.TooltipDuration = 60;
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
		size = new System.Drawing.Size(1223, 761);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmPayAdd";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "จ\u0e48ายเง\u0e34น";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.Panel3.ResumeLayout(false);
		this.expandablePanel5.ResumeLayout(false);
		this.Panel2.ResumeLayout(false);
		this.Panel2.PerformLayout();
		this.expandablePanel4.ResumeLayout(false);
		this.Panel1.ResumeLayout(false);
		this.Panel1.PerformLayout();
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.Grid2).EndInit();
		((System.ComponentModel.ISupportInitialize)this.Grid1).EndInit();
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

	private void Button6_Click(object sender, EventArgs e)
	{
		TdocNum.Text = "";
		Clear();
	}

	public void Clear()
	{
		LOver.Visible = false;
		POver.Visible = false;
		Tdebt.Text = "0.00";
		Tnote.Text = "";
		clear_Add_cus();
		EDIT_ID = "";
		Grid1.Rows.RemoveRange(1, 499);
		Grid2.Rows.RemoveRange(1, 49);
		Grid1.Rows.Add(499);
		Grid2.Rows.Add(49);
		AddItemInCombobox();
		sum();
	}

	public void clear_Add_cus()
	{
		TCusID.Text = "";
		TcusName.Text = "";
		TextBox_1.Text = "";
		TCusType.SelectedIndex = 0;
		TCarID.Text = "";
		TCarType.Text = "";
		TextBox_0.Text = "";
		Tc_no.Text = "";
		Tc_moo.Text = "";
		Tc_soi.Text = "";
		Tc_road.Text = "";
		Tc_tambon.Text = "";
		Tc_ampore.Text = "";
		Tc_province.Text = "";
		Tc_code.Text = "";
		Tc_tel.Text = "";
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
	}

	public string GET_DOC()
	{
		DataSet dataSet = Module1.connect("select top 1 * from HT_CheckIn_H order by Cin_no desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return "CH" + Strings.Format(1, "000000");
		}
		return "CH" + Strings.Format(checked(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["Cin_no"].ToString().Replace("CH", "")) + 1), "000000");
	}

	private void FrmCheckIn_Load(object sender, EventArgs e)
	{
		Button7.Enabled = true;
		LoadType();
		LoadBill();
	}

	public void LoadBill()
	{
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_H where Cin_no='" + EDIT_ID + "'");
		DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Ds where Cin_no='" + EDIT_ID + "' order by Cin_Room_No");
		DataSet dataSet3 = Module1.connect("select * from HT_CheckIn_Product where Cin_no='" + EDIT_ID + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("ไม\u0e48พบเลขบ\u0e34ล " + EDIT_ID);
			return;
		}
		Clear();
		WORK_ID = Module1.GET_WORK_NUMBER(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		ComboBox1.Enabled = false;
		ComboBox1.SelectedIndex = Conversions.ToInteger(dataSet.Tables[0].Rows[0]["Cin_type"]);
		EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_cust_no"]);
		TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		DataSet dataSet4 = Module1.connect("select * from HT_Customers where Cust_no ='" + TCusID.Text + "'");
		if (dataSet4.Tables[0].Rows.Count != 0)
		{
			TCusID.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_no"]);
			TcusName.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_name"]);
			TextBox_1.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_name2"]);
			TCusType.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Type"]);
			TextBox_0.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Email"]);
			Tc_no.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_no"]);
			Tc_moo.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_moo"]);
			Tc_soi.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_soi"]);
			Tc_road.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_road"]);
			Tc_tambon.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_tambon"]);
			Tc_ampore.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_ampore"]);
			Tc_province.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_province"]);
			Tc_code.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_code"]);
			Tc_tel.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_tel"]);
			Tc_fax.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_fax"]);
			Tw.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_Name"]);
			Tw_no.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_no"]);
			Tw_moo.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_moo"]);
			Tw_soi.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_soi"]);
			Tw_road.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_road"]);
			Tw_tambon.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_tambon"]);
			Tw_ampore.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_ampore"]);
			Tw_privince.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_province"]);
			Tw_code.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_code"]);
			Tw_tel.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_tel"]);
			Tw_fax.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Work_fax"]);
			Tover.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Price_Over"]);
			Tcontry.Text = dataSet4.Tables[0].Rows[0]["Cust_contry"].ToString();
		}
		TCarID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Car_id"]);
		TCarType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Car_type"]);
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
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "Check-Out", TextCompare: false))
				{
					Grid1[num2 + 1, 15] = true;
				}
				Grid1[num2 + 1, 16] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_note"]);
				Grid1[num2 + 1, 17] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["id"]);
				num2++;
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
				num6++;
			}
			AddItemInCombobox();
			DataSet dataSet5 = Module1.connect("select cin_no,cin_pay_cash,cin_pay_credit,cin_pay_date from View_Pay_Ds where cin_no='" + TdocNum.Text + "' group by cin_no,cin_pay_cash,cin_pay_credit,cin_pay_date order by cin_pay_date");
			DataSet dataSet6 = Module1.connect("select * from HT_Log_Debt where log_ds = 'ต\u0e31ดจากใบลงทะเบ\u0e35ยน " + TdocNum.Text + "' order by log_date");
			string text = "ท\u0e35\u0e48    ว\u0e31นท\u0e35\u0e48                         เง\u0e34นสด              เครด\u0e34ต             จ\u0e48ายล\u0e48วงหน\u0e49า";
			int num8 = dataSet5.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 > num4)
				{
					break;
				}
				text = text + "\r\n" + Conversions.ToString(num9 + 1) + ".   " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[num9]["cin_pay_date"]), "dd/MM/yy HH:mm") + "          " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[num9]["cin_pay_cash"]), "#,##0.00") + "           " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[num9]["cin_pay_credit"]), "#,##0.00");
				num9++;
			}
			int num11 = dataSet6.Tables[0].Rows.Count - 1;
			int num12 = 0;
			while (true)
			{
				int num13 = num12;
				int num4 = num11;
				if (num13 > num4)
				{
					break;
				}
				string[] array = new string[7]
				{
					text,
					"\r\n",
					Conversions.ToString(dataSet5.Tables[0].Rows.Count + num12 + 1),
					".   ",
					Strings.Format(RuntimeHelpers.GetObjectValue(dataSet6.Tables[0].Rows[num12]["log_date"]), "dd/MM/yy HH:mm"),
					"                                                      ",
					null
				};
				string[] array2 = array;
				Type typeFromHandle = typeof(Math);
				object[] array3 = new object[1];
				DataRow dataRow = dataSet6.Tables[0].Rows[num12];
				string columnName = "log_price";
				array3[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
				object[] array4 = array3;
				bool[] array5 = new bool[1] { true };
				object obj = NewLateBinding.LateGet(null, typeFromHandle, "Abs", array4, null, null, array5);
				if (array5[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array4[0]);
				}
				array2[6] = Strings.Format(RuntimeHelpers.GetObjectValue(obj), "#,##0.00");
				text = string.Concat(array);
				num12++;
			}
			SuperTooltip1.SetSuperTooltip(LabelPayed, new SuperTooltipInfo("รายการชำระเง\u0e34น", Conversions.ToString(Module1.Company_Name), text, Resources.boy_emoticon_009, null, eTooltipColor.Orange));
			sum();
		}
	}

	public void LoadType()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by id");
		TCusType.DataSource = dataSet.Tables[0];
		TCusType.DisplayMember = "name";
		TCusType.ValueMember = "id";
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
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products_Price where P_ID='", dataSet.Tables[0].Rows[0]["Pro_no"]), "' and P_CustType='"), TCusType.Text), "'")));
		if (dataSet2.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("ราคาส\u0e34นค\u0e49าย\u0e31งไม\u0e48ได\u0e49ต\u0e31\u0e49งราคา กร\u0e38ณาไปต\u0e31\u0e49งราคาก\u0e48อน");
			return;
		}
		int num = 1;
		checked
		{
			int num2 = Grid2.Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 <= num5)
				{
					if (Operators.CompareString(Conversions.ToString(Grid2[num3, 1]), "", TextCompare: false) != 0)
					{
						num3++;
						continue;
					}
					num = num3;
					break;
				}
				break;
			}
			Grid2[num, 1] = num;
			Grid2[num, 2] = TselectRoom.Text;
			Grid2[num, 3] = DateTime.Now;
			Grid2[num, 4] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Pro_name"]);
			Grid2[num, 5] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Pro_Unit"]);
			Grid2[num, 6] = "1";
			Grid2[num, 7] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["P_Price"]), "#,##0.00");
			Grid2[num, 8] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["P_Price"]), "#,##0.00");
			Grid2[num, 9] = "0.00";
			Grid2[num, 10] = Strings.Format(decimal.Multiply(Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["P_Price"]), 1m), "#,##0.00");
			Grid2[num, 11] = "0.00";
			Grid2[num, 12] = "";
			Grid2[num, 13] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Pro_no"]);
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
			Grid2[e.Row, 10] = Strings.Format(Conversions.ToDecimal(Operators.SubtractObject(Grid2[e.Row, 8], Conversions.ToDecimal(Grid2[e.Row, 9]))), "#,##0.00");
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
		checked
		{
			int num6 = Grid1.Rows.Count - 1;
			int num7 = 1;
			while (true)
			{
				int num8 = num7;
				int num9 = num6;
				if (num8 > num9)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num7, 1]), "", TextCompare: false) != 0)
				{
					num = decimal.Add(num, Conversions.ToDecimal(Grid1[num7, 10]));
					num3 = decimal.Add(num3, Conversions.ToDecimal(Grid1[num7, 12]));
					num5 = decimal.Add(num5, Conversions.ToDecimal(Grid1[num7, 13]));
					num4 = decimal.Add(num4, Conversions.ToDecimal(Grid1[num7, 14]));
				}
				num7++;
			}
			int num10 = Grid2.Rows.Count - 1;
			int num11 = 1;
			while (true)
			{
				int num12 = num11;
				int num9 = num10;
				if (num12 > num9)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num11, 1]), "", TextCompare: false) != 0)
				{
					num2 = decimal.Add(num2, Conversions.ToDecimal(Grid2[num11, 8]));
					num3 = decimal.Add(num3, Conversions.ToDecimal(Grid2[num11, 9]));
					num5 = decimal.Add(num5, Conversions.ToDecimal(Grid2[num11, 10]));
					num4 = decimal.Add(num4, Conversions.ToDecimal(Grid2[num11, 11]));
				}
				num11++;
			}
			LabelTroom.Text = Strings.Format(num, "#,##0.00");
			LabelTpro.Text = Strings.Format(num2, "#,##0.00");
			Labelroompro.Text = Strings.Format(decimal.Add(num, num2), "#,##0.00");
			LabelPayed.Text = Strings.Format(num3, "#,##0.00");
			LabelDebt.Text = Strings.Format(num5, "#,##0.00");
			Tpay.Text = Strings.Format(num4, "#,##0.00");
			Tcash.Text = Strings.Format(decimal.Subtract(num4, Conversions.ToDecimal(Tdebt.Text)), "#,##0.00");
			Refresh_Dep_auto_total();
		}
	}

	private void Button9_Click(object sender, EventArgs e)
	{
		sum();
	}

	private void Button11_Click(object sender, EventArgs e)
	{
		sum();
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
		left = Operators.ConcatenateObject(left, ",[Cust_Work_fax]");
		left = Operators.ConcatenateObject(left, ")");
		left = Operators.ConcatenateObject(left, " VALUES ");
		left = Operators.ConcatenateObject(left, "(");
		left = Operators.ConcatenateObject(left, right);
		left = Operators.ConcatenateObject(left, string.Concat(",'" + text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusName.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_1.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TCusType.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_0.Text, "'"));
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
		left = Operators.ConcatenateObject(left, ")");
		Module1.connect(Conversions.ToString(left));
		return text;
	}

	public void EDIT_CUST()
	{
		object left = "UPDATE [HT_Customers] SET ";
		left = Operators.ConcatenateObject(left, string.Concat(" [Cust_name]='" + TcusName.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_name2]='" + TextBox_1.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Type]='" + TCusType.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Cust_Email]='" + TextBox_0.Text, "'"));
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
		bool flag2 = false;
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
				if (Operators.CompareString(Conversions.ToString(Grid1[num2, 11]), "", TextCompare: false) != 0)
				{
					flag2 = true;
				}
				num2++;
			}
		}
		if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) <= 0 || !flag2)
		{
			if (!((decimal.Compare(Conversions.ToDecimal(LabelDebt.Text), 0m) < 0) & (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) != 0)))
			{
				MessageBox.Show("ไม\u0e48ม\u0e35รายการชำระเง\u0e34น");
				Button7.Enabled = true;
				return;
			}
			if (Operators.CompareString(Tnote.Text, "", TextCompare: false) == 0)
			{
				MessageBox.Show("กร\u0e38ณาใส\u0e48หมายเหต\u0e38ในการจ\u0e48ายเง\u0e34นต\u0e34ดลบ");
				Tnote.Focus();
				Button7.Enabled = true;
				return;
			}
		}
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
		if (flag)
		{
		}
		if (Operators.CompareString(EDIT_ID, "", TextCompare: false) != 0)
		{
			SAVE_EDIT();
		}
	}

	public void SAVE_EDIT()
	{
		DataSet dataSet = Module1.connect("select Cin_Work_number from HT_CheckIn_H where Cin_no='" + EDIT_ID + "'");
		if (Operators.ConditionalCompareObjectNotEqual(WORK_ID, dataSet.Tables[0].Rows[0]["Cin_Work_number"], TextCompare: false))
		{
			MessageBox.Show("ม\u0e35การแก\u0e49ไข เลขลงทะเบ\u0e35ยน  " + EDIT_ID + " จากเคร\u0e37\u0e48องอ\u0e37\u0e48น กร\u0e38ณาป\u0e34ดแล\u0e49วเข\u0e49ามาทำรายการใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Close();
			return;
		}
		if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) > 0)
		{
			MyProject.Forms.FormConfirmPay.PTOTAl = Conversions.ToDecimal(Tpay.Text);
			MyProject.Forms.FormConfirmPay.ShowDialog();
			if (!MyProject.Forms.FormConfirmPay.ISOK)
			{
				Button7.Enabled = true;
				return;
			}
			Tcash.Text = Conversions.ToString(MyProject.Forms.FormConfirmPay.PCASH);
			Tdebt.Text = Conversions.ToString(MyProject.Forms.FormConfirmPay.PCREDIT);
		}
		string text = TCusID.Text;
		DateTime now = DateTime.Now;
		Module1.connect("delete from HT_CheckIn_Product where Cin_no='" + TdocNum.Text + "'");
		string left = "";
		checked
		{
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
				left = Conversions.ToString(Operators.ConcatenateObject(left, Operators.ConcatenateObject(TselectRoom.Items[num2], " ")));
				num2++;
			}
			string sIR_PAY = Module1.GetSIR_PAY();
			object obj = "";
			decimal num5 = default(decimal);
			int num6 = Grid1.Rows.Count - 1;
			int num7 = 1;
			while (true)
			{
				int num8 = num7;
				int num4 = num6;
				if (num8 > num4 || Operators.CompareString(Conversions.ToString(Grid1[num7, 1]), "", TextCompare: false) == 0)
				{
					break;
				}
				num5 = decimal.Add(num5, Conversions.ToDecimal(Grid1[num7, 11]));
				obj = "update [HT_CheckIn_Ds] SET ";
				obj = Operators.ConcatenateObject(obj, " [Cin_Room_Pay_Total]=" + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid1[num7, 12]), Conversions.ToDecimal(Grid1[num7, 14])), Conversions.ToDecimal(Grid1[num7, 11]))));
				obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",[Cin_note]='", Grid1[num7, 16]), "'"));
				obj = Operators.ConcatenateObject(obj, " where id=" + Conversions.ToString(Grid1[num7, 17]));
				Module1.connect(Conversions.ToString(obj));
				if (decimal.Compare(Conversions.ToDecimal(Grid1[num7, 14]), 0m) > 0)
				{
					Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num7, 2]), now, Conversions.ToDecimal(Tcash.Text), Conversions.ToDecimal(Tdebt.Text), "ค\u0e48าห\u0e49อง", Conversions.ToDecimal(Conversions.ToString(Grid1[num7, 14])), "รายการ", sIR_PAY, text, "P001", Conversions.ToDecimal(Grid1[num7, 9]), Conversions.ToDecimal(Grid1[num7, 10]), Conversions.ToDecimal(Grid1[num7, 8]), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
				}
				else if (decimal.Compare(Conversions.ToDecimal(Grid1[num7, 14]), 0m) < 0)
				{
					Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num7, 2]), now, Conversions.ToDecimal(Conversions.ToString(Grid1[num7, 14])), 0m, "ค\u0e48าห\u0e49อง", Conversions.ToDecimal(Conversions.ToString(Grid1[num7, 14])), "รายการ", sIR_PAY, text, "P001", Conversions.ToDecimal(Grid1[num7, 9]), Conversions.ToDecimal(Grid1[num7, 10]), Conversions.ToDecimal(Grid1[num7, 8]), Tnote.Text, 0m, 0m, 0m);
				}
				num7++;
			}
			int num9 = Grid2.Rows.Count - 1;
			int num10 = 1;
			while (true)
			{
				int num11 = num10;
				int num4 = num9;
				if (num11 > num4 || Operators.CompareString(Conversions.ToString(Grid2[num10, 1]), "", TextCompare: false) == 0)
				{
					break;
				}
				num5 = decimal.Add(num5, Conversions.ToDecimal(Grid2[num10, 14]));
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
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num10, 2]), "'"));
				obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid2[num10, 3]), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num10, 13]), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num10, 4]), "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num10, 5]), "'"));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num10, 6])));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num10, 7])));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num10, 8])));
				obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(Grid2[num10, 9]), Conversions.ToDecimal(Grid2[num10, 11])), Conversions.ToDecimal(Grid2[num10, 14]))));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(Grid2[num10, 12]), "'"));
				obj = Operators.ConcatenateObject(obj, ")");
				Module1.connect(Conversions.ToString(obj));
				if (decimal.Compare(Conversions.ToDecimal(Grid2[num10, 11]), 0m) > 0)
				{
					Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid2[num10, 2]), now, Conversions.ToDecimal(Tcash.Text), Conversions.ToDecimal(Tdebt.Text), Conversions.ToString(Grid2[num10, 4]), Conversions.ToDecimal(Conversions.ToString(Grid2[num10, 11])), Conversions.ToString(Grid2[num10, 5]), sIR_PAY, text, Conversions.ToString(Grid2[num10, 13]), Conversions.ToDecimal(Grid2[num10, 6]), Conversions.ToDecimal(Grid2[num10, 8]), Conversions.ToDecimal(Grid2[num10, 7]), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
				}
				num10++;
			}
			obj = "UPDATE [HT_CheckIn_H] SET ";
			obj = Operators.ConcatenateObject(obj, " [Total_Price_Room]=" + Conversions.ToString(Conversions.ToDecimal(LabelTroom.Text)));
			obj = Operators.ConcatenateObject(obj, ",[Total_Price_Product]=" + Conversions.ToString(Conversions.ToDecimal(LabelTpro.Text)));
			obj = Operators.ConcatenateObject(obj, ",[Total_Price_Net]=" + Conversions.ToString(Conversions.ToDecimal(Labelroompro.Text)));
			obj = Operators.ConcatenateObject(obj, ",[Total_Price_Pay]=" + Conversions.ToString(decimal.Add(decimal.Add(Conversions.ToDecimal(LabelPayed.Text), Conversions.ToDecimal(Tpay.Text)), num5)));
			obj = Operators.ConcatenateObject(obj, ",[Total_Price_Balance]=" + Conversions.ToString(decimal.Subtract(decimal.Subtract(decimal.Subtract(Conversions.ToDecimal(Labelroompro.Text), Conversions.ToDecimal(LabelPayed.Text)), Conversions.ToDecimal(Tpay.Text)), num5)));
			obj = Operators.ConcatenateObject(obj, string.Concat(" where [Cin_no]='" + TdocNum.Text, "'"));
			Module1.connect(Conversions.ToString(obj));
			Module1.UPDATE_MONEY(text, num5, "DEL", "ต\u0e31ดจากใบลงทะเบ\u0e35ยน " + TdocNum.Text);
			object obj2 = "";
			if (decimal.Compare(num5, 0m) > 0)
			{
				obj2 = Module1.GetSIR_PAY();
				decimal num12 = default(decimal);
				int num13 = Grid1.Rows.Count - 1;
				int num14 = 1;
				while (true)
				{
					int num15 = num14;
					int num4 = num13;
					if (num15 > num4)
					{
						break;
					}
					num12 = decimal.Add(num12, Conversions.ToDecimal(Grid1[num14, 11]));
					num14++;
				}
				int num16 = Grid2.Rows.Count - 1;
				int num17 = 1;
				while (true)
				{
					int num18 = num17;
					int num4 = num16;
					if (num18 > num4)
					{
						break;
					}
					num12 = decimal.Add(num12, Conversions.ToDecimal(Grid2[num17, 14]));
					num17++;
				}
				int num19 = Grid1.Rows.Count - 1;
				int num20 = 1;
				while (true)
				{
					int num21 = num20;
					int num4 = num19;
					if (num21 > num4)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid1[num20, 1]), "", TextCompare: false) != 0 && decimal.Compare(Conversions.ToDecimal(Grid1[num20, 11]), 0m) > 0)
					{
						Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid1[num20, 2]), now, 0m, 0m, "ต\u0e31ดยอดล\u0e48วงหน\u0e49า ค\u0e48าห\u0e49อง", Conversions.ToDecimal(Grid1[num20, 11]), "รายการ", Conversions.ToString(obj2), text, "P001", Conversions.ToDecimal(Grid1[num20, 9]), Conversions.ToDecimal(Grid1[num20, 10]), Conversions.ToDecimal(Grid1[num20, 8]), "จ\u0e48ายล\u0e48วงหน\u0e49า", num12, 0m, 0m);
					}
					num20++;
				}
				int num22 = Grid2.Rows.Count - 1;
				int num23 = 1;
				while (true)
				{
					int num24 = num23;
					int num4 = num22;
					if (num24 > num4)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid2[num23, 1]), "", TextCompare: false) != 0)
					{
						num5 = decimal.Add(num5, Conversions.ToDecimal(Grid2[num23, 14]));
						if (decimal.Compare(Conversions.ToDecimal(Grid2[num23, 14]), 0m) > 0)
						{
							Module1.Insert_Pay(TdocNum.Text, Conversions.ToString(Grid2[num23, 2]), now, 0m, 0m, "ต\u0e31ดยอดล\u0e48วงหน\u0e49า " + Conversions.ToString(Grid2[num23, 4]), Conversions.ToDecimal(Grid2[num23, 14]), Conversions.ToString(Grid2[num23, 5]), Conversions.ToString(obj2), text, Conversions.ToString(Grid2[num23, 13]), Conversions.ToDecimal(Grid2[num23, 6]), Conversions.ToDecimal(Grid2[num23, 8]), Conversions.ToDecimal(Grid2[num23, 7]), "จ\u0e48ายล\u0e48วงหน\u0e49า", num12, 0m, 0m);
						}
					}
					num23++;
				}
			}
			MessageBox.Show("บ\u0e31นท\u0e36กเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
			{
				Print_Report.Print_Sale(sIR_PAY, preview: false);
			}
			if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0 && Operators.ConditionalCompareObjectNotEqual(obj2, "", TextCompare: false) && Operators.ConditionalCompareObjectNotEqual(sIR_PAY, obj2, TextCompare: false))
			{
				Print_Report.Print_Sale(Conversions.ToString(obj2), preview: false);
			}
			TdocNum.Text = "";
			Clear();
			Close();
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormSearchChechInnotOut.ShowDialog();
		if (Operators.ConditionalCompareObjectNotEqual(MyProject.Forms.FormSearchChechInnotOut.SelectNO, "", TextCompare: false))
		{
			EDIT_ID = Conversions.ToString(MyProject.Forms.FormSearchChechInnotOut.SelectNO);
			LoadBill();
		}
	}

	private void expandablePanel4_Click(object sender, EventArgs e)
	{
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Refresh_Dep_auto();
	}

	public void Refresh_Dep_auto(decimal pay_price = 0m)
	{
		object obj = "";
		if (decimal.Compare(pay_price, 0m) == 0)
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

	private void Panel1_Paint(object sender, PaintEventArgs e)
	{
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ComboBox1.SelectedIndex == 0)
		{
			Grid1.Cols[9].Caption = "จำนวนค\u0e37น";
		}
		else if (ComboBox1.SelectedIndex == 1)
		{
			Grid1.Cols[9].Caption = "ช\u0e31\u0e48วโมง";
		}
		else
		{
			Grid1.Cols[9].Caption = "เด\u0e37อน";
		}
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
}
