using System;
using System.Collections;
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
public class FrmAddBook2 : Office2007Form
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

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TdocNum")]
	private TextBox _TdocNum;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("TCusSearch")]
	private TextBox _TCusSearch;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("TcusName")]
	private TextBox _TcusName;

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

	[AccessedThroughProperty("Label26")]
	private Label _Label26;

	[AccessedThroughProperty("TCusName2")]
	private TextBox _TCusName2;

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

	[AccessedThroughProperty("Tc_tel")]
	private ComboBox _Tc_tel;

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

	[AccessedThroughProperty("Label18")]
	private Label _Label18;

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

	[AccessedThroughProperty("Tstart")]
	private DateTimePicker _Tstart;

	[AccessedThroughProperty("Label52")]
	private Label _Label52;

	[AccessedThroughProperty("Tnote")]
	private TextBox _Tnote;

	[AccessedThroughProperty("Tpay")]
	private TextBox _Tpay;

	[AccessedThroughProperty("TCusType")]
	private ComboBox _TCusType;

	[AccessedThroughProperty("Label21")]
	private Label _Label21;

	[AccessedThroughProperty("Grid1")]
	private C1FlexGrid _Grid1;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("TselectRoom")]
	private ComboBox _TselectRoom;

	[AccessedThroughProperty("Label55")]
	private Label _Label55;

	[AccessedThroughProperty("Button9")]
	private Button _Button9;

	[AccessedThroughProperty("Button5")]
	private Button _Button5;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("Grid2")]
	private C1FlexGrid _Grid2;

	[AccessedThroughProperty("LabeR")]
	private Label _LabeR;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("LabelP")]
	private Label _LabelP;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Tdays")]
	private TextBox _Tdays;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ComboSale")]
	private ComboBox _ComboSale;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Tnum")]
	private TextBox _Tnum;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	public string EDIT_ID;

	public ArrayList R_ARR;

	public string SETdtae;

	private decimal EDIT_PRICE;

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
			EventHandler value3 = TCusNo_LostFocus;
			EventHandler value4 = TCusNo_GotFocus;
			if (_TCusSearch != null)
			{
				_TCusSearch.TextChanged -= value2;
				_TCusSearch.LostFocus -= value3;
				_TCusSearch.GotFocus -= value4;
			}
			_TCusSearch = value;
			if (_TCusSearch != null)
			{
				_TCusSearch.TextChanged += value2;
				_TCusSearch.LostFocus += value3;
				_TCusSearch.GotFocus += value4;
			}
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
			EventHandler value2 = DateTimePicker1_ValueChanged;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged -= value2;
			}
			_DateTimePicker1 = value;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged += value2;
			}
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
			EventHandler value3 = Tc_tel_SelectedIndexChanged;
			EventHandler value4 = ComboBox3_LostFocus;
			if (_Tc_tel != null)
			{
				_Tc_tel.GotFocus -= value2;
				_Tc_tel.SelectedIndexChanged -= value3;
				_Tc_tel.LostFocus -= value4;
			}
			_Tc_tel = value;
			if (_Tc_tel != null)
			{
				_Tc_tel.GotFocus += value2;
				_Tc_tel.SelectedIndexChanged += value3;
				_Tc_tel.LostFocus += value4;
			}
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
			_TimerDrop2 = value;
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
			EventHandler value2 = Button3_Click;
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
			EventHandler value2 = ListView1_DoubleClick;
			EventHandler value3 = ListView1_SelectedIndexChanged;
			EventHandler value4 = TCusNo_LostFocus;
			EventHandler value5 = TCusNo_GotFocus;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick -= value2;
				_ListView1.SelectedIndexChanged -= value3;
				_ListView1.LostFocus -= value4;
				_ListView1.GotFocus -= value5;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick += value2;
				_ListView1.SelectedIndexChanged += value3;
				_ListView1.LostFocus += value4;
				_ListView1.GotFocus += value5;
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
			EventHandler value2 = Grid1_Click;
			RowColEventHandler value3 = Grid1_AfterEdit;
			RowColEventHandler value4 = Grid1_AfterDeleteRow;
			if (_Grid1 != null)
			{
				_Grid1.Click -= value2;
				_Grid1.AfterEdit -= value3;
				_Grid1.AfterDeleteRow -= value4;
			}
			_Grid1 = value;
			if (_Grid1 != null)
			{
				_Grid1.Click += value2;
				_Grid1.AfterEdit += value3;
				_Grid1.AfterDeleteRow += value4;
			}
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
			EventHandler value2 = Button9_Click_1;
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
			EventHandler value3 = Grid2_Click;
			if (_Grid2 != null)
			{
				_Grid2.AfterEdit -= value2;
				_Grid2.Click -= value3;
			}
			_Grid2 = value;
			if (_Grid2 != null)
			{
				_Grid2.AfterEdit += value2;
				_Grid2.Click += value3;
			}
		}
	}

	internal virtual Label LabeR
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabeR;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabeR = value;
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

	internal virtual Label LabelP
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelP;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelP = value;
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

	internal virtual TextBox Tdays
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tdays;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tdays = value;
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

	internal virtual ComboBox ComboSale
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboSale;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox3_GotFocus;
			EventHandler value3 = ComboSale_SelectedIndexChanged;
			if (_ComboSale != null)
			{
				_ComboSale.GotFocus -= value2;
				_ComboSale.SelectedIndexChanged -= value3;
			}
			_ComboSale = value;
			if (_ComboSale != null)
			{
				_ComboSale.GotFocus += value2;
				_ComboSale.SelectedIndexChanged += value3;
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
			EventHandler value2 = Tnum_LostFocus;
			EventHandler value3 = Tnum_TextChanged;
			if (_Tnum != null)
			{
				_Tnum.LostFocus -= value2;
				_Tnum.TextChanged -= value3;
			}
			_Tnum = value;
			if (_Tnum != null)
			{
				_Tnum.LostFocus += value2;
				_Tnum.TextChanged += value3;
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

	[DebuggerNonUserCode]
	static FrmAddBook2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmAddBook2()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmCheckIn_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EDIT_ID = "";
		R_ARR = new ArrayList();
		SETdtae = "";
		EDIT_PRICE = default(decimal);
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmAddBook2));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.TCusType = new System.Windows.Forms.ComboBox();
		this.Label21 = new System.Windows.Forms.Label();
		this.ComboSale = new System.Windows.Forms.ComboBox();
		this.Tc_tel = new System.Windows.Forms.ComboBox();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label16 = new System.Windows.Forms.Label();
		this.Label26 = new System.Windows.Forms.Label();
		this.Label50 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label9 = new System.Windows.Forms.Label();
		this.TextBox_1 = new System.Windows.Forms.TextBox();
		this.TCusID = new System.Windows.Forms.TextBox();
		this.TcusName = new System.Windows.Forms.TextBox();
		this.TextBox_0 = new System.Windows.Forms.TextBox();
		this.TdocNum = new System.Windows.Forms.TextBox();
		this.PanelCust = new System.Windows.Forms.Panel();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Tnum = new System.Windows.Forms.TextBox();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Grid2 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.Tdays = new System.Windows.Forms.TextBox();
		this.Label2 = new System.Windows.Forms.Label();
		this.LabeR = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.LabelP = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.TselectRoom = new System.Windows.Forms.ComboBox();
		this.Label55 = new System.Windows.Forms.Label();
		this.Button9 = new System.Windows.Forms.Button();
		this.Button5 = new System.Windows.Forms.Button();
		this.Label12 = new System.Windows.Forms.Label();
		this.Tnote = new System.Windows.Forms.TextBox();
		this.Tpay = new System.Windows.Forms.TextBox();
		this.Tstart = new System.Windows.Forms.DateTimePicker();
		this.Label52 = new System.Windows.Forms.Label();
		this.Grid1 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.Button11 = new System.Windows.Forms.Button();
		this.Label35 = new System.Windows.Forms.Label();
		this.Label18 = new System.Windows.Forms.Label();
		this.LabelTroom = new System.Windows.Forms.Label();
		this.Label24 = new System.Windows.Forms.Label();
		this.Button3 = new System.Windows.Forms.Button();
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
		this.PanelCust.SuspendLayout();
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
		this.GroupBox1.Controls.Add(this.TCusType);
		this.GroupBox1.Controls.Add(this.Label21);
		this.GroupBox1.Controls.Add(this.ComboSale);
		this.GroupBox1.Controls.Add(this.Tc_tel);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label16);
		this.GroupBox1.Controls.Add(this.Label26);
		this.GroupBox1.Controls.Add(this.Label50);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.Label6);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.Label9);
		this.GroupBox1.Controls.Add(this.TextBox_1);
		this.GroupBox1.Controls.Add(this.TCusID);
		this.GroupBox1.Controls.Add(this.TcusName);
		this.GroupBox1.Controls.Add(this.TextBox_0);
		this.GroupBox1.Controls.Add(this.TdocNum);
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
		System.Drawing.Size size = new System.Drawing.Size(995, 117);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 12;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายละเอ\u0e35ยดการ จอง";
		this.TCusType.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TCusType.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tCusType = this.TCusType;
		location = new System.Drawing.Point(119, 78);
		tCusType.Location = location;
		System.Windows.Forms.ComboBox tCusType2 = this.TCusType;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusType2.Margin = margin;
		this.TCusType.Name = "TCusType";
		System.Windows.Forms.ComboBox tCusType3 = this.TCusType;
		size = new System.Drawing.Size(144, 24);
		tCusType3.Size = size;
		this.TCusType.TabIndex = 64;
		this.Label21.AutoSize = true;
		System.Windows.Forms.Label label = this.Label21;
		location = new System.Drawing.Point(55, 82);
		label.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label2 = this.Label21;
		size = new System.Drawing.Size(62, 16);
		label2.Size = size;
		this.Label21.TabIndex = 63;
		this.Label21.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.ComboSale.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboSale = this.ComboSale;
		location = new System.Drawing.Point(759, 23);
		comboSale.Location = location;
		System.Windows.Forms.ComboBox comboSale2 = this.ComboSale;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		comboSale2.Margin = margin;
		this.ComboSale.Name = "ComboSale";
		System.Windows.Forms.ComboBox comboSale3 = this.ComboSale;
		size = new System.Drawing.Size(170, 24);
		comboSale3.Size = size;
		this.ComboSale.TabIndex = 51;
		this.Tc_tel.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tc_tel = this.Tc_tel;
		location = new System.Drawing.Point(759, 78);
		tc_tel.Location = location;
		System.Windows.Forms.ComboBox tc_tel2 = this.Tc_tel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tc_tel2.Margin = margin;
		this.Tc_tel.Name = "Tc_tel";
		System.Windows.Forms.ComboBox tc_tel3 = this.Tc_tel;
		size = new System.Drawing.Size(170, 24);
		tc_tel3.Size = size;
		this.Tc_tel.TabIndex = 51;
		this.DateTimePicker1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.DateTimePicker1.CustomFormat = "ddMMMMyyyy เวลา HH:mm";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(336, 21);
		dateTimePicker.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker2.Margin = margin;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		size = new System.Drawing.Size(170, 23);
		dateTimePicker3.Size = size;
		this.DateTimePicker1.TabIndex = 48;
		this.Label16.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label16;
		location = new System.Drawing.Point(282, 24);
		label3.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label4 = this.Label16;
		size = new System.Drawing.Size(52, 16);
		label4.Size = size;
		this.Label16.TabIndex = 47;
		this.Label16.Text = "ว\u0e31นท\u0e35\u0e48จอง";
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label26;
		location = new System.Drawing.Point(512, 83);
		label5.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label6 = this.Label26;
		size = new System.Drawing.Size(54, 16);
		label6.Size = size;
		this.Label26.TabIndex = 47;
		this.Label26.Text = "นามสก\u0e38ล";
		this.Label50.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label50;
		location = new System.Drawing.Point(274, 53);
		label7.Location = location;
		this.Label50.Name = "Label50";
		System.Windows.Forms.Label label8 = this.Label50;
		size = new System.Drawing.Size(60, 16);
		label8.Size = size;
		this.Label50.TabIndex = 47;
		this.Label50.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label7;
		location = new System.Drawing.Point(309, 82);
		label9.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label10 = this.Label7;
		size = new System.Drawing.Size(24, 16);
		label10.Size = size;
		this.Label7.TabIndex = 47;
		this.Label7.Text = "ช\u0e37\u0e48อ";
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label6;
		location = new System.Drawing.Point(2, 48);
		label11.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label12 = this.Label6;
		size = new System.Drawing.Size(115, 32);
		label12.Size = size;
		this.Label6.TabIndex = 47;
		this.Label6.Text = "ค\u0e49นหาล\u0e39กค\u0e49า รห\u0e31ส/ช\u0e37\u0e48อ\r\nเบอร\u0e4cโทร";
		this.Label6.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label5;
		location = new System.Drawing.Point(674, 27);
		label13.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label14 = this.Label5;
		size = new System.Drawing.Size(83, 16);
		label14.Size = size;
		this.Label5.TabIndex = 47;
		this.Label5.Text = "เซลล\u0e4cผ\u0e39\u0e49ร\u0e31บเร\u0e37\u0e48อง";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label1;
		location = new System.Drawing.Point(45, 25);
		label15.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label16 = this.Label1;
		size = new System.Drawing.Size(72, 16);
		label16.Size = size;
		this.Label1.TabIndex = 47;
		this.Label1.Text = "เลขท\u0e35\u0e48ใบจอง";
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label9;
		location = new System.Drawing.Point(728, 83);
		label17.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label18 = this.Label9;
		size = new System.Drawing.Size(29, 16);
		label18.Size = size;
		this.Label9.TabIndex = 47;
		this.Label9.Text = "โทร";
		this.TextBox_1.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox textBox_ = this.TextBox_1;
		location = new System.Drawing.Point(569, 79);
		textBox_.Location = location;
		System.Windows.Forms.TextBox textBox_2 = this.TextBox_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_2.Margin = margin;
		this.TextBox_1.Name = "TCusName2";
		System.Windows.Forms.TextBox textBox_3 = this.TextBox_1;
		size = new System.Drawing.Size(154, 23);
		textBox_3.Size = size;
		this.TextBox_1.TabIndex = 1;
		this.TCusID.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
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
		size = new System.Drawing.Size(170, 23);
		tCusID3.Size = size;
		this.TCusID.TabIndex = 1;
		this.TcusName.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tcusName = this.TcusName;
		location = new System.Drawing.Point(336, 78);
		tcusName.Location = location;
		System.Windows.Forms.TextBox tcusName2 = this.TcusName;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcusName2.Margin = margin;
		this.TcusName.Name = "TcusName";
		System.Windows.Forms.TextBox tcusName3 = this.TcusName;
		size = new System.Drawing.Size(170, 23);
		tcusName3.Size = size;
		this.TcusName.TabIndex = 1;
		System.Windows.Forms.TextBox textBox_4 = this.TextBox_0;
		location = new System.Drawing.Point(119, 49);
		textBox_4.Location = location;
		System.Windows.Forms.TextBox textBox_5 = this.TextBox_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox_5.Margin = margin;
		this.TextBox_0.Name = "TCusSearch";
		System.Windows.Forms.TextBox textBox_6 = this.TextBox_0;
		size = new System.Drawing.Size(144, 23);
		textBox_6.Size = size;
		this.TextBox_0.TabIndex = 1;
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
		this.PanelCust.BackColor = System.Drawing.Color.DimGray;
		this.PanelCust.Controls.Add(this.ListView1);
		System.Windows.Forms.Panel panelCust = this.PanelCust;
		location = new System.Drawing.Point(279, 47);
		panelCust.Location = location;
		this.PanelCust.Name = "PanelCust";
		System.Windows.Forms.Panel panelCust2 = this.PanelCust;
		size = new System.Drawing.Size(408, 304);
		panelCust2.Size = size;
		this.PanelCust.TabIndex = 62;
		this.PanelCust.Visible = false;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.BorderStyle = System.Windows.Forms.BorderStyle.None;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[4] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader5 });
		this.ListView1.FullRowSelect = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(3, 2);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(402, 299);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "รห\u0e31ส";
		this.ColumnHeader1.Width = 70;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader3.Text = "นามสก\u0e38ล";
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader5.Text = "เบอร\u0e4cโทร";
		this.ColumnHeader5.Width = 100;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.PanelCust);
		this.PanelEx1.Controls.Add(this.Tnum);
		this.PanelEx1.Controls.Add(this.Label10);
		this.PanelEx1.Controls.Add(this.Label8);
		this.PanelEx1.Controls.Add(this.Grid2);
		this.PanelEx1.Controls.Add(this.Tdays);
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.LabeR);
		this.PanelEx1.Controls.Add(this.Label4);
		this.PanelEx1.Controls.Add(this.LabelP);
		this.PanelEx1.Controls.Add(this.Label3);
		this.PanelEx1.Controls.Add(this.TselectRoom);
		this.PanelEx1.Controls.Add(this.Label55);
		this.PanelEx1.Controls.Add(this.Button9);
		this.PanelEx1.Controls.Add(this.Button5);
		this.PanelEx1.Controls.Add(this.Label12);
		this.PanelEx1.Controls.Add(this.Tnote);
		this.PanelEx1.Controls.Add(this.Tpay);
		this.PanelEx1.Controls.Add(this.Tstart);
		this.PanelEx1.Controls.Add(this.Label52);
		this.PanelEx1.Controls.Add(this.Grid1);
		this.PanelEx1.Controls.Add(this.Button11);
		this.PanelEx1.Controls.Add(this.Label35);
		this.PanelEx1.Controls.Add(this.Label18);
		this.PanelEx1.Controls.Add(this.LabelTroom);
		this.PanelEx1.Controls.Add(this.Label24);
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Controls.Add(this.Button3);
		this.PanelEx1.Controls.Add(this.Button7);
		this.PanelEx1.Controls.Add(this.Label14);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(1017, 683);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.Color = System.Drawing.Color.LavenderBlush;
		this.PanelEx1.Style.BackColor2.Color = System.Drawing.Color.Pink;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 50;
		System.Windows.Forms.TextBox tnum = this.Tnum;
		location = new System.Drawing.Point(606, 128);
		tnum.Location = location;
		this.Tnum.Name = "Tnum";
		System.Windows.Forms.TextBox tnum2 = this.Tnum;
		size = new System.Drawing.Size(42, 23);
		tnum2.Size = size;
		this.Tnum.TabIndex = 99;
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label10;
		location = new System.Drawing.Point(652, 132);
		label19.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label20 = this.Label10;
		size = new System.Drawing.Size(24, 16);
		label20.Size = size;
		this.Label10.TabIndex = 98;
		this.Label10.Text = "ค\u0e37น";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label8;
		location = new System.Drawing.Point(506, 132);
		label21.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label22 = this.Label8;
		size = new System.Drawing.Size(98, 16);
		label22.Size = size;
		this.Label8.TabIndex = 98;
		this.Label8.Text = "จำนวนค\u0e37นท\u0e35\u0e48แสดง";
		this.Grid2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Grid2.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid2.ColumnInfo = resources.GetString("Grid2.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid = this.Grid2;
		location = new System.Drawing.Point(15, 466);
		grid.Location = location;
		this.Grid2.Name = "Grid2";
		this.Grid2.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid2 = this.Grid2;
		size = new System.Drawing.Size(728, 205);
		grid2.Size = size;
		this.Grid2.StyleInfo = resources.GetString("Grid2.StyleInfo");
		this.Grid2.TabIndex = 91;
		this.Grid2.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.Tdays.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tdays.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tdays.Font = new System.Drawing.Font("Times New Roman", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tdays.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.TextBox tdays = this.Tdays;
		location = new System.Drawing.Point(876, 607);
		tdays.Location = location;
		System.Windows.Forms.TextBox tdays2 = this.Tdays;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tdays2.Margin = margin;
		this.Tdays.Name = "Tdays";
		System.Windows.Forms.TextBox tdays3 = this.Tdays;
		size = new System.Drawing.Size(129, 29);
		tdays3.Size = size;
		this.Tdays.TabIndex = 97;
		this.Tdays.Text = "3";
		this.Tdays.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label23 = this.Label2;
		location = new System.Drawing.Point(741, 599);
		label23.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label24 = this.Label2;
		size = new System.Drawing.Size(133, 48);
		label24.Size = size;
		this.Label2.TabIndex = 96;
		this.Label2.Text = "แจ\u0e49งเต\u0e37อนการชำระเง\u0e34น (ว\u0e31น)";
		this.Label2.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.LabeR.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabeR.BackColor = System.Drawing.Color.Navy;
		this.LabeR.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabeR.Font = new System.Drawing.Font("Times New Roman", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabeR.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label labeR = this.LabeR;
		location = new System.Drawing.Point(876, 434);
		labeR.Location = location;
		this.LabeR.Name = "LabeR";
		System.Windows.Forms.Label labeR2 = this.LabeR;
		size = new System.Drawing.Size(129, 25);
		labeR2.Size = size;
		this.LabeR.TabIndex = 95;
		this.LabeR.Text = "0.00";
		this.LabeR.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label4.AutoSize = true;
		this.Label4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label25 = this.Label4;
		location = new System.Drawing.Point(758, 438);
		label25.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label26 = this.Label4;
		size = new System.Drawing.Size(115, 19);
		label26.Size = size;
		this.Label4.TabIndex = 94;
		this.Label4.Text = "รวมราคาห\u0e49องพ\u0e31ก";
		this.LabelP.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelP.BackColor = System.Drawing.Color.Navy;
		this.LabelP.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabelP.Font = new System.Drawing.Font("Times New Roman", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelP.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label labelP = this.LabelP;
		location = new System.Drawing.Point(876, 461);
		labelP.Location = location;
		this.LabelP.Name = "LabelP";
		System.Windows.Forms.Label labelP2 = this.LabelP;
		size = new System.Drawing.Size(129, 25);
		labelP2.Size = size;
		this.LabelP.TabIndex = 95;
		this.LabelP.Text = "0.00";
		this.LabelP.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label3.AutoSize = true;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label27 = this.Label3;
		location = new System.Drawing.Point(768, 465);
		label27.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label28 = this.Label3;
		size = new System.Drawing.Size(105, 19);
		label28.Size = size;
		this.Label3.TabIndex = 94;
		this.Label3.Text = "รวมราคาส\u0e34นค\u0e49า";
		this.TselectRoom.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.TselectRoom.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TselectRoom.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tselectRoom = this.TselectRoom;
		location = new System.Drawing.Point(208, 435);
		tselectRoom.Location = location;
		System.Windows.Forms.ComboBox tselectRoom2 = this.TselectRoom;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tselectRoom2.Margin = margin;
		this.TselectRoom.Name = "TselectRoom";
		System.Windows.Forms.ComboBox tselectRoom3 = this.TselectRoom;
		size = new System.Drawing.Size(121, 24);
		tselectRoom3.Size = size;
		this.TselectRoom.TabIndex = 93;
		this.Label55.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label55.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label55;
		location = new System.Drawing.Point(156, 440);
		label29.Location = location;
		this.Label55.Name = "Label55";
		System.Windows.Forms.Label label30 = this.Label55;
		size = new System.Drawing.Size(50, 16);
		label30.Size = size;
		this.Label55.TabIndex = 92;
		this.Label55.Text = "เลขห\u0e49อง";
		this.Button9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button9.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button = this.Button9;
		location = new System.Drawing.Point(385, 435);
		button.Location = location;
		System.Windows.Forms.Button button2 = this.Button9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button2.Margin = margin;
		this.Button9.Name = "Button9";
		System.Windows.Forms.Button button3 = this.Button9;
		size = new System.Drawing.Size(52, 24);
		button3.Size = size;
		this.Button9.TabIndex = 90;
		this.Button9.Text = "    ลบ";
		this.Button9.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button9.UseVisualStyleBackColor = true;
		this.Button5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button5.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button4 = this.Button5;
		location = new System.Drawing.Point(332, 435);
		button4.Location = location;
		System.Windows.Forms.Button button5 = this.Button5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button5.Margin = margin;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button6 = this.Button5;
		size = new System.Drawing.Size(51, 24);
		button6.Size = size;
		this.Button5.TabIndex = 88;
		this.Button5.UseVisualStyleBackColor = true;
		this.Label12.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label12.AutoSize = true;
		this.Label12.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label31 = this.Label12;
		location = new System.Drawing.Point(21, 439);
		label31.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label32 = this.Label12;
		size = new System.Drawing.Size(133, 16);
		label32.Size = size;
		this.Label12.TabIndex = 89;
		this.Label12.Text = "ค\u0e48าใช\u0e49จ\u0e48ายเพ\u0e34\u0e48มเต\u0e34มอ\u0e37\u0e48นๆ";
		this.Tnote.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tnote.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tnote.Font = new System.Drawing.Font("Times New Roman", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tnote.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox tnote = this.Tnote;
		location = new System.Drawing.Point(876, 550);
		tnote.Location = location;
		System.Windows.Forms.TextBox tnote2 = this.Tnote;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tnote2.Margin = margin;
		this.Tnote.Multiline = true;
		this.Tnote.Name = "Tnote";
		System.Windows.Forms.TextBox tnote3 = this.Tnote;
		size = new System.Drawing.Size(129, 53);
		tnote3.Size = size;
		this.Tnote.TabIndex = 87;
		this.Tpay.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tpay.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tpay.Font = new System.Drawing.Font("Times New Roman", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Tpay.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tpay = this.Tpay;
		location = new System.Drawing.Point(876, 517);
		tpay.Location = location;
		System.Windows.Forms.TextBox tpay2 = this.Tpay;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tpay2.Margin = margin;
		this.Tpay.Name = "Tpay";
		System.Windows.Forms.TextBox tpay3 = this.Tpay;
		size = new System.Drawing.Size(129, 29);
		tpay3.Size = size;
		this.Tpay.TabIndex = 87;
		this.Tpay.Text = "0.00";
		this.Tpay.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tstart.CustomFormat = "dd/MM/yy เวลาข\u0e49าพ\u0e31ก HH:mm";
		this.Tstart.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tstart = this.Tstart;
		location = new System.Drawing.Point(163, 129);
		tstart.Location = location;
		System.Windows.Forms.DateTimePicker tstart2 = this.Tstart;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tstart2.Margin = margin;
		this.Tstart.Name = "Tstart";
		System.Windows.Forms.DateTimePicker tstart3 = this.Tstart;
		size = new System.Drawing.Size(210, 23);
		tstart3.Size = size;
		this.Tstart.TabIndex = 82;
		this.Label52.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label52;
		location = new System.Drawing.Point(96, 132);
		label33.Location = location;
		this.Label52.Name = "Label52";
		System.Windows.Forms.Label label34 = this.Label52;
		size = new System.Drawing.Size(66, 16);
		label34.Size = size;
		this.Label52.TabIndex = 80;
		this.Label52.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก";
		this.Grid1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Grid1.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid1.ColumnInfo = resources.GetString("Grid1.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid3 = this.Grid1;
		location = new System.Drawing.Point(16, 159);
		grid3.Location = location;
		this.Grid1.Name = "Grid1";
		this.Grid1.Rows.Count = 300;
		this.Grid1.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid4 = this.Grid1;
		size = new System.Drawing.Size(989, 266);
		grid4.Size = size;
		this.Grid1.StyleInfo = resources.GetString("Grid1.StyleInfo");
		this.Grid1.TabIndex = 76;
		this.Grid1.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.Button11.Image = (System.Drawing.Image)resources.GetObject("Button11.Image");
		this.Button11.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button7 = this.Button11;
		location = new System.Drawing.Point(889, 130);
		button7.Location = location;
		System.Windows.Forms.Button button8 = this.Button11;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button8.Margin = margin;
		this.Button11.Name = "Button11";
		System.Windows.Forms.Button button9 = this.Button11;
		size = new System.Drawing.Size(50, 24);
		button9.Size = size;
		this.Button11.TabIndex = 75;
		this.Button11.Text = "    ลบ";
		this.Button11.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button11.UseVisualStyleBackColor = true;
		this.Button11.Visible = false;
		this.Label35.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label35.AutoSize = true;
		this.Label35.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label35 = this.Label35;
		location = new System.Drawing.Point(753, 554);
		label35.Location = location;
		this.Label35.Name = "Label35";
		System.Windows.Forms.Label label36 = this.Label35;
		size = new System.Drawing.Size(121, 19);
		label36.Size = size;
		this.Label35.TabIndex = 53;
		this.Label35.Text = "หมายเหต\u0e38การจ\u0e48าย";
		this.Label18.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label18.AutoSize = true;
		this.Label18.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label37 = this.Label18;
		location = new System.Drawing.Point(785, 523);
		label37.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label38 = this.Label18;
		size = new System.Drawing.Size(89, 19);
		label38.Size = size;
		this.Label18.TabIndex = 53;
		this.Label18.Text = "จ\u0e48ายล\u0e48วงหน\u0e49า";
		this.LabelTroom.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LabelTroom.BackColor = System.Drawing.Color.Navy;
		this.LabelTroom.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.LabelTroom.Font = new System.Drawing.Font("Times New Roman", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelTroom.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label labelTroom = this.LabelTroom;
		location = new System.Drawing.Point(876, 488);
		labelTroom.Location = location;
		this.LabelTroom.Name = "LabelTroom";
		System.Windows.Forms.Label labelTroom2 = this.LabelTroom;
		size = new System.Drawing.Size(129, 25);
		labelTroom2.Size = size;
		this.LabelTroom.TabIndex = 63;
		this.LabelTroom.Text = "0.00";
		this.LabelTroom.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label24.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label24.AutoSize = true;
		this.Label24.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label39 = this.Label24;
		location = new System.Drawing.Point(757, 490);
		label39.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label40 = this.Label24;
		size = new System.Drawing.Size(116, 19);
		label40.Size = size;
		this.Label24.TabIndex = 53;
		this.Label24.Text = "รวมราคาท\u0e31\u0e49งหมด";
		this.Button3.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button10 = this.Button3;
		location = new System.Drawing.Point(379, 128);
		button10.Location = location;
		System.Windows.Forms.Button button11 = this.Button3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button11.Margin = margin;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button12 = this.Button3;
		size = new System.Drawing.Size(109, 24);
		button12.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "เล\u0e37อกเลขห\u0e49อง";
		this.Button3.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
		this.Button3.UseVisualStyleBackColor = true;
		this.Button7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Button7.Image = (System.Drawing.Image)resources.GetObject("Button7.Image");
		this.Button7.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button13 = this.Button7;
		location = new System.Drawing.Point(876, 638);
		button13.Location = location;
		System.Windows.Forms.Button button14 = this.Button7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button14.Margin = margin;
		this.Button7.Name = "Button7";
		System.Windows.Forms.Button button15 = this.Button7;
		size = new System.Drawing.Size(129, 41);
		button15.Size = size;
		this.Button7.TabIndex = 3;
		this.Button7.Text = "        บ\u0e31นท\u0e36ก";
		this.Button7.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button7.UseVisualStyleBackColor = true;
		this.Label14.AutoSize = true;
		this.Label14.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label41 = this.Label14;
		location = new System.Drawing.Point(13, 132);
		label41.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label42 = this.Label14;
		size = new System.Drawing.Size(78, 16);
		label42.Size = size;
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
		size = new System.Drawing.Size(1017, 683);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmAddBook2";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เพ\u0e34\u0e48มรายการจอง";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.PanelCust.ResumeLayout(false);
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.Grid2).EndInit();
		((System.ComponentModel.ISupportInitialize)this.Grid1).EndInit();
		this.ResumeLayout(false);
	}

	private void ComboBox3_GotFocus(object sender, EventArgs e)
	{
	}

	private void TimerDrop_Tick(object sender, EventArgs e)
	{
		TimerDrop.Enabled = false;
		Tc_tel.DroppedDown = true;
	}

	private void ComboBox3_LostFocus(object sender, EventArgs e)
	{
	}

	public void chk_from_roommain()
	{
		checked
		{
			if (R_ARR.Count != 0)
			{
				int num = 1;
				int num2 = R_ARR.Count - 1;
				int num3 = 0;
				while (true)
				{
					int num4 = num3;
					int num5 = num2;
					if (num4 > num5)
					{
						break;
					}
					DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select Room_type,Room_no from HT_Rooms where Room_no='", R_ARR[num3]), "'")));
					dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms_Price where room_type='", dataSet.Tables[0].Rows[0]["Room_type"]), "' and room_custType='"), TCusType.Text), "' ")));
					decimal num6 = default(decimal);
					if (dataSet.Tables[0].Rows.Count != 0)
					{
						num6 = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["room_price"]);
					}
					Grid1[num, 1] = num;
					Grid1[num, 2] = RuntimeHelpers.GetObjectValue(R_ARR[num3]);
					Grid1[num, 3] = Conversions.ToString(Tstart.Value.Date) + " 12:00:00";
					if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
					{
						Grid1[num, 4] = Conversions.ToString(Tstart.Value.Date) + " 11:59:59";
					}
					else
					{
						Grid1[num, 4] = Conversions.ToString(Tstart.Value.Date.AddDays(1.0)) + " 11:59:59";
					}
					Grid1[num, 5] = num6;
					Grid1[num, 6] = 1;
					Grid1[num, 7] = 1;
					Grid1[num, 8] = Strings.Format(decimal.Multiply(num6, 1m), "#,##0.00");
					Grid1[num, 9] = "";
					num++;
					num3++;
				}
				num++;
			}
			sum();
			AddItemInCombobox();
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
				TextBox_1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				Tc_tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				Button3.Focus();
			}
		}
	}

	private void TCusNo_GotFocus(object sender, EventArgs e)
	{
		PanelCust.Visible = true;
		listcust();
	}

	private void TCusNo_LostFocus(object sender, EventArgs e)
	{
		PanelCust.Visible = false;
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

	public void Clear()
	{
		if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
		{
			Tstart.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date.AddDays(-1.0)) + " 12:00:00");
		}
		else
		{
			Tstart.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 12:00:00");
		}
		TdocNum.Text = GET_DOC();
		Tnote.Text = "";
		Tdays.Text = Conversions.ToString(3);
		clear_Add_cus();
		ComboSale.Text = "";
		Grid1.Rows.RemoveRange(1, 299);
		Grid1.Rows.Add(299);
		Grid2.Rows.RemoveRange(1, 49);
		Grid2.Rows.Add(49);
		sum();
		Button7.Enabled = true;
	}

	public void clear_Add_cus()
	{
		TextBox_0.Text = "";
		TCusID.Text = "";
		TcusName.Text = "";
		TextBox_1.Text = "";
		Tc_tel.Text = "";
	}

	public string GET_DOC()
	{
		DataSet dataSet = Module1.connect("select top 1 * from HT_Book_H order by Book_ID desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return "R" + Strings.Format(1, "000000");
		}
		return "R" + Strings.Format(checked(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["Book_ID"].ToString().Replace("R", "")) + 1), "000000");
	}

	private void FrmCheckIn_Load(object sender, EventArgs e)
	{
		Tnum.Text = Conversions.ToString(Module1.Booking_Room_amount);
		LOAD_SALE();
		LoadType();
		Clear();
		chkEdit();
		if (Operators.CompareString(SETdtae, "", TextCompare: false) != 0)
		{
			Tstart.Value = Conversions.ToDate(SETdtae);
		}
		chk_from_roommain();
	}

	public void LOAD_SALE()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_Sale order by name");
		ComboSale.Items.Clear();
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					ComboSale.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void chkEdit()
	{
		if (Operators.CompareString(EDIT_ID, "", TextCompare: false) == 0)
		{
			return;
		}
		Text = "แก\u0e49ไขรายการจอง";
		DataSet dataSet = Module1.connect("select * from HT_Book_H where book_id='" + EDIT_ID + "'");
		DataSet dataSet2 = Module1.connect("select * from HT_Book_ds where book_no='" + EDIT_ID + "'");
		DataSet dataSet3 = Module1.connect("select * from HT_Book_Pro where b_no='" + EDIT_ID + "'");
		DateTimePicker1.Value = Conversions.ToDate(dataSet.Tables[0].Rows[0]["book_date"]);
		TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_id"]);
		TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_cust_id"]);
		Tpay.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_price_pay"]);
		Tnote.Text = dataSet.Tables[0].Rows[0]["book_room_note"].ToString();
		ComboSale.Text = dataSet.Tables[0].Rows[0]["book_sale"].ToString();
		try
		{
			Tdays.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Notify_Day"]);
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			Tdays.Text = Conversions.ToString(0);
			ProjectData.ClearProjectError();
		}
		EDIT_PRICE = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["book_price_pay"]);
		DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Customers where Cust_no ='", dataSet.Tables[0].Rows[0]["book_cust_id"]), "'")));
		if (dataSet4.Tables[0].Rows.Count != 0)
		{
			TextBox_0.Text = "";
			TCusID.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_no"]);
			TcusName.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_name"]);
			TextBox_1.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_name2"]);
			TCusType.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Type"]);
			Tc_tel.Text = Conversions.ToString(dataSet4.Tables[0].Rows[0]["Cust_Add_tel"]);
			Button3.Focus();
		}
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
				Grid1[num5, 1] = num5;
				Grid1[num5, 2] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_type"]);
				Grid1[num5, 3] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_start"]);
				Grid1[num5, 4] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_end"]);
				Grid1[num5, 5] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_price"]);
				Grid1[num5, 6] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_night"]);
				Grid1[num5, 7] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_num"]);
				Grid1[num5, 8] = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_priceTotal"]), "#,##0.00");
				Grid1[num5, 9] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["book_room_note"]);
				num5++;
				num2++;
			}
			int num9 = dataSet3.Tables[0].Rows.Count - 1;
			int num10 = 0;
			while (true)
			{
				int num11 = num10;
				int num4 = num9;
				if (num11 > num4)
				{
					break;
				}
				int num12 = 1;
				int num13 = Grid2.Rows.Count - 1;
				int num14 = 0;
				while (true)
				{
					int num15 = num14;
					num4 = num13;
					if (num15 <= num4)
					{
						if (Operators.CompareString(Conversions.ToString(Grid2[num14, 1]), "", TextCompare: false) != 0)
						{
							num14++;
							continue;
						}
						num12 = num14;
						break;
					}
					break;
				}
				Grid2[num12, 1] = num12;
				Grid2[num12, 2] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_ROOM"]);
				Grid2[num12, 3] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_NAME"]);
				Grid2[num12, 4] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_UNIT"]);
				Grid2[num12, 5] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_NUM"]);
				Grid2[num12, 6] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_PRICE"]);
				Grid2[num12, 7] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_PRICE_TOTAL"]);
				Grid2[num12, 8] = RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["B_PRO_ID"]);
				num12++;
				num10++;
			}
			sum();
		}
	}

	public void LoadType()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name desc");
		TCusType.DataSource = dataSet.Tables[0];
		TCusType.DisplayMember = "name";
		TCusType.ValueMember = "id";
		checked
		{
			int num = TCusType.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (NewLateBinding.LateIndexGet(TCusType.Items[num2], new object[1] { 2 }, null).ToString().IndexOf("ปกต\u0e34") != -1)
					{
						TCusType.SelectedIndex = num2;
					}
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void LoadBill()
	{
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_H where Cin_no='" + EDIT_ID + "'");
		DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Ds where Cin_no='" + EDIT_ID + "' order by Cin_Room_No");
		Module1.connect("select * from HT_CheckIn_Product where Cin_no='" + EDIT_ID + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("ไม\u0e48พบเลขบ\u0e34ล " + EDIT_ID);
			return;
		}
		Clear();
		EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		TCusID.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_cust_no"]);
		TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		DataSet dataSet3 = Module1.connect("select * from HT_Customers where Cust_no ='" + TCusID.Text + "'");
		if (dataSet3.Tables[0].Rows.Count != 0)
		{
			TextBox_0.Text = "";
			TCusID.Text = Conversions.ToString(dataSet3.Tables[0].Rows[0]["Cust_no"]);
			TcusName.Text = Conversions.ToString(dataSet3.Tables[0].Rows[0]["Cust_name"]);
			TextBox_1.Text = Conversions.ToString(dataSet3.Tables[0].Rows[0]["Cust_name2"]);
			Tc_tel.Text = Conversions.ToString(dataSet3.Tables[0].Rows[0]["Cust_Add_tel"]);
			Button3.Focus();
		}
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
				if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "เข\u0e49าพ\u0e31ก", TextCompare: false), Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "Check-Out", TextCompare: false))))
				{
					Grid1[num2 + 1, 15] = true;
				}
				Grid1[num2 + 1, 16] = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Cin_note"]);
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num2]["Cin_Room_Status"], "Check-Out", TextCompare: false))
				{
					Button7.Enabled = false;
				}
				num2++;
			}
			sum();
		}
	}

	private void Grid1_AfterDeleteRow(object sender, RowColEventArgs e)
	{
	}

	private void Grid1_AfterEdit(object sender, RowColEventArgs e)
	{
		if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 2]), "", TextCompare: false) == 0)
		{
			Grid1[e.Row, e.Col] = "";
		}
		else if (e.Col == 5)
		{
			if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 5]), "", TextCompare: false) == 0)
			{
				Grid1[e.Row, 5] = "0";
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid1[e.Row, 5])))
			{
				Grid1[e.Row, 5] = "0";
			}
			Grid1[e.Row, 8] = Strings.Format(decimal.Multiply(decimal.Multiply(Conversions.ToDecimal(Grid1[e.Row, 5]), Conversions.ToDecimal(Grid1[e.Row, 6])), Conversions.ToDecimal(Grid1[e.Row, 7])), "#,##0.00");
			sum();
		}
	}

	public void sum()
	{
		decimal num = default(decimal);
		int num2 = 0;
		checked
		{
			int num3 = Grid2.Rows.Count - 1;
			int num4 = 1;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 > num6)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num4 - num2, 1]), "", TextCompare: false) != 0)
				{
					bool flag = false;
					int num7 = Grid1.Rows.Count - 1;
					int num8 = 1;
					while (true)
					{
						int num9 = num8;
						num6 = num7;
						if (num9 > num6)
						{
							break;
						}
						if (Operators.CompareString(Conversions.ToString(Grid2[num4 - num2, 2]), Conversions.ToString(Grid1[num8, 2]), TextCompare: false) == 0)
						{
							flag = true;
							Conversions.ToInteger(Grid1[num8, 6]);
						}
						num8++;
					}
					if (!flag)
					{
						Grid2.Rows.Remove(num4 - num2);
						num2++;
					}
				}
				num4++;
			}
			LabeR.Text = Conversions.ToString(0);
			LabelP.Text = Conversions.ToString(0);
			int num10 = Grid1.Rows.Count - 1;
			int num11 = 1;
			while (true)
			{
				int num12 = num11;
				int num6 = num10;
				if (num12 > num6)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num11, 1]), "", TextCompare: false) != 0)
				{
					num = decimal.Add(num, Conversions.ToDecimal(Grid1[num11, 8]));
					LabeR.Text = Conversions.ToString(decimal.Add(Conversions.ToDecimal(LabeR.Text), Conversions.ToDecimal(Grid1[num11, 8])));
				}
				num11++;
			}
			int num13 = Grid2.Rows.Count - 1;
			int num14 = 1;
			while (true)
			{
				int num15 = num14;
				int num6 = num13;
				if (num15 > num6)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num14, 1]), "", TextCompare: false) != 0)
				{
					num = decimal.Add(num, Conversions.ToDecimal(Grid2[num14, 7]));
					LabelP.Text = Conversions.ToString(decimal.Add(Conversions.ToDecimal(LabelP.Text), Conversions.ToDecimal(Grid2[num14, 7])));
					Grid2[num14, 1] = num14;
				}
				num14++;
			}
			LabelTroom.Text = Strings.Format(num, "#,##0.00");
			LabeR.Text = Strings.Format(Conversions.ToDecimal(LabeR.Text), "#,##0.00");
			LabelP.Text = Strings.Format(Conversions.ToDecimal(LabelP.Text), "#,##0.00");
		}
	}

	private void Button9_Click(object sender, EventArgs e)
	{
		sum();
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
				}
				sum();
				AddItemInCombobox();
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

	private void Button7_Click(object sender, EventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		if (Operators.CompareString(Tpay.Text, "", TextCompare: false) == 0)
		{
			Tpay.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(Tpay.Text))
		{
			MessageBox.Show("กร\u0e38ณากรอกจ\u0e48ายล\u0e48วงหน\u0e49าเป\u0e47นต\u0e31วเลข");
			return;
		}
		if (Operators.CompareString(TcusName.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อผ\u0e39\u0e49เข\u0e49าพ\u0e31ก");
			return;
		}
		if (!Versioned.IsNumeric(Tdays.Text))
		{
			MessageBox.Show("กร\u0e38ณากรอกจำนวนว\u0e31นเป\u0e47นต\u0e31วเลข");
			return;
		}
		if (Operators.CompareString(Tdays.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกจำนวนว\u0e31นเป\u0e47นต\u0e31วเลข");
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
				MessageBox.Show("กร\u0e38ณาเพ\u0e34\u0e48มรายการจองห\u0e49องพ\u0e31ก");
			}
			else if (Operators.CompareString(EDIT_ID, "", TextCompare: false) == 0)
			{
				SAVE_ADD();
			}
			else
			{
				SAVE_EDIT();
			}
		}
	}

	public void SAVE_ADD()
	{
		sum();
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.Yes)
		{
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
		}
		string text = "";
		text = ((Operators.CompareString(TCusID.Text, "", TextCompare: false) == 0) ? SAVE_CUST() : TCusID.Text);
		ArrayList arrayList = new ArrayList();
		string text2 = "";
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
					int num5 = arrayList.Count - 1;
					int num6 = 0;
					while (true)
					{
						int num7 = num6;
						num4 = num5;
						if (num7 > num4)
						{
							break;
						}
						bool flag = false;
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num6], new object[1] { 0 }, null), Conversions.ToString(Grid1[num2, 2]), TextCompare: false))
						{
							flag = true;
							NewLateBinding.LateIndexSetComplex(arrayList[num6], new object[2]
							{
								1,
								decimal.Add(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num6], new object[1] { 0 }, null)), 1m)
							}, null, OptimisticSet: false, RValueBase: true);
						}
						else if (!flag)
						{
							string[] value = new string[2]
							{
								Conversions.ToString(Grid1[num2, 2]),
								Conversions.ToString(1)
							};
							arrayList.Add(value);
						}
						num6++;
					}
				}
				num2++;
			}
			int num8 = arrayList.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 > num4)
				{
					break;
				}
				text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(arrayList[num9], new object[1] { 0 }, null), " "), NewLateBinding.LateIndexGet(arrayList[num9], new object[1] { 1 }, null)), " ห\u0e49อง"), "\r\n")));
				num9++;
			}
			string text3 = "";
			int num11 = Grid1.Rows.Count - 1;
			int num12 = 1;
			while (true)
			{
				int num13 = num12;
				int num4 = num11;
				if (num13 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num12, 1]), "", TextCompare: false) != 0 && Operators.CompareString(Conversions.ToString(Grid1[num12, 9]), "", TextCompare: false) != 0)
				{
					text3 = text3 + " " + Conversions.ToString(Grid1[num12, 9]);
				}
				num12++;
			}
			DateTime dateTime = DateTime.Now;
			DateTime dateTime2 = DateTime.Now;
			int num14 = Grid1.Rows.Count - 1;
			int num15 = 1;
			while (true)
			{
				int num16 = num15;
				int num4 = num14;
				if (num16 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num15, 1]), "", TextCompare: false) != 0)
				{
					if (Operators.ConditionalCompareObjectLess(Grid1[num15, 3], dateTime, TextCompare: false))
					{
						dateTime = Conversions.ToDate(Grid1[num15, 3]);
					}
					if (Operators.ConditionalCompareObjectGreater(Grid1[num15, 4], dateTime2, TextCompare: false))
					{
						dateTime2 = Conversions.ToDate(Grid1[num15, 4]);
					}
					if (num15 == 1)
					{
						dateTime = Conversions.ToDate(Grid1[num15, 3]);
						dateTime2 = Conversions.ToDate(Grid1[num15, 4]);
					}
				}
				num15++;
			}
			TdocNum.Text = GET_DOC();
			object left = "INSERT INTO [HT_Book_H]";
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, " [Book_ID]");
			left = Operators.ConcatenateObject(left, ",[Book_Date]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_ID]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_Name]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_Name2]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_Tel]");
			left = Operators.ConcatenateObject(left, ",[Book_Price_Total]");
			left = Operators.ConcatenateObject(left, ",[Book_Price_Pay]");
			left = Operators.ConcatenateObject(left, ",[Book_Status],[Book_Date_in],[Book_Date_out],[Book_by],[Book_room_all],[Book_room_note],[book_room_type],Book_Notify_Day,Book_sale)");
			left = Operators.ConcatenateObject(left, "VALUES");
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, string.Concat(" '" + TdocNum.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTimePicker1.Value), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusName.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_1.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_tel.Text, "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(LabelTroom.Text)));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(Tpay.Text)));
			left = Operators.ConcatenateObject(left, ",'จอง'");
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(dateTime.Date), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(dateTime2.Date), "'"));
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Module1.loginName), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + text2, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(",'" + Tnote.Text, " "), text3), "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(2));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToInteger(Tdays.Text)));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + ComboSale.Text, "'"));
			left = Operators.ConcatenateObject(left, ")");
			Module1.connect(Conversions.ToString(left));
			int num17 = Grid1.Rows.Count - 1;
			int num18 = 1;
			while (true)
			{
				int num19 = num18;
				int num4 = num17;
				if (num19 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num18, 1]), "", TextCompare: false) != 0)
				{
					left = "INSERT INTO [HT_Book_Ds]";
					left = Operators.ConcatenateObject(left, "([Book_No]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Type]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Start]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_End]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Price]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Night]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Num]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_PriceToTal]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Note])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, string.Concat(" '" + TdocNum.Text, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Grid1[num18, 2]), "'"));
					left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num18, 3]), "'"));
					left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num18, 4]), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 5])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 6])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 7])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 8])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Grid1[num18, 9]), "'"));
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
					int num20 = Convert.ToInt32(decimal.Subtract(Conversions.ToDecimal(Grid1[num18, 6]), 1m));
					int num21 = 0;
					while (true)
					{
						int num22 = num21;
						num4 = num20;
						if (num22 > num4)
						{
							break;
						}
						DateTime dateTime3 = Conversions.ToDate(Grid1[num18, 3]);
						if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num18, 3]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num18, 3]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
						{
							dateTime3 = dateTime3.AddDays(-1.0);
						}
						object right = Module1.get_id("HT_Book_Date", "id");
						object left2 = "INSERT INTO [HT_Book_Date]";
						left2 = Operators.ConcatenateObject(left2, "([id]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_no]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_type]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_date_ds]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_Num],[Book_USE])");
						left2 = Operators.ConcatenateObject(left2, "VALUES");
						left2 = Operators.ConcatenateObject(left2, "(");
						left2 = Operators.ConcatenateObject(left2, right);
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + TdocNum.Text, "'"));
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(Grid1[num18, 2]), "'"));
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(dateTime3.AddDays(num21).Date), "'"));
						left2 = Operators.ConcatenateObject(left2, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 7])));
						left2 = Operators.ConcatenateObject(left2, ",0");
						left2 = Operators.ConcatenateObject(left2, ")");
						Module1.connect(Conversions.ToString(left2));
						num21++;
					}
				}
				num18++;
			}
			int num23 = Grid2.Rows.Count - 1;
			int num24 = 1;
			while (true)
			{
				int num25 = num24;
				int num4 = num23;
				if (num25 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num24, 1]), "", TextCompare: false) != 0)
				{
					object left3 = "INSERT INTO [HT_Book_Pro]";
					left3 = Operators.ConcatenateObject(left3, "(");
					left3 = Operators.ConcatenateObject(left3, "[B_NO]");
					left3 = Operators.ConcatenateObject(left3, ",[B_ROOM]");
					left3 = Operators.ConcatenateObject(left3, ",[B_NAME]");
					left3 = Operators.ConcatenateObject(left3, ",[B_UNIT]");
					left3 = Operators.ConcatenateObject(left3, ",[B_NUM]");
					left3 = Operators.ConcatenateObject(left3, ",[B_PRICE]");
					left3 = Operators.ConcatenateObject(left3, ",[B_PRICE_TOTAL],[B_PRO_ID])");
					left3 = Operators.ConcatenateObject(left3, "VALUES");
					left3 = Operators.ConcatenateObject(left3, "(");
					left3 = Operators.ConcatenateObject(left3, string.Concat("'" + TdocNum.Text, "'"));
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + Conversions.ToString(Grid2[num24, 2]), "'"));
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + Conversions.ToString(Grid2[num24, 3]), "'"));
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + Conversions.ToString(Grid2[num24, 4]), "'"));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num24, 5])));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num24, 6])));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num24, 7])));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToInteger(Grid2[num24, 8])));
					left3 = Operators.ConcatenateObject(left3, ")");
					Module1.connect(Conversions.ToString(left3));
				}
				num24++;
			}
			if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) > 0)
			{
				string sIR_PAY = Module1.GetSIR_PAY();
				Module1.Insert_Pay(TdocNum.Text, "การจองแบบระบ\u0e38ห\u0e49อง", DateTime.Now, MyProject.Forms.FormConfirmPay.PCASH, MyProject.Forms.FormConfirmPay.PCREDIT, "จ\u0e48ายล\u0e48วงหน\u0e49า", Conversions.ToDecimal(Tpay.Text), "รายการ", sIR_PAY, text, "P001", 1m, Conversions.ToDecimal(Tpay.Text), Conversions.ToDecimal(Tpay.Text), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
				Module1.UPDATE_MONEY(text, Conversions.ToDecimal(Tpay.Text), "ADD", "จ\u0e48ายล\u0e48วงหน\u0e49าจากใบจอง " + TdocNum.Text);
				if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
				{
					Print_Report.Print_Sale(sIR_PAY, preview: false);
				}
			}
			MessageBox.Show("บ\u0e31นท\u0e36กเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			Close();
		}
	}

	public void update_cust()
	{
		Module1.connect("update HT_Customers set Cust_name='" + TcusName.Text + "',Cust_name2='" + TextBox_1.Text + "',Cust_Type='" + TCusType.Text + "',Cust_Add_tel='" + Tc_tel.Text + "' where Cust_no='" + TCusID.Text + "' ");
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
		left = Operators.ConcatenateObject(left, ",[Cust_perfix]");
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
		left = Operators.ConcatenateObject(left, ")");
		left = Operators.ConcatenateObject(left, " VALUES ");
		left = Operators.ConcatenateObject(left, "(");
		left = Operators.ConcatenateObject(left, right);
		left = Operators.ConcatenateObject(left, string.Concat(",'" + text, "'"));
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusName.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_1.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TCusType.Text, "'"));
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_tel.Text, "'"));
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, ",''");
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now.Date), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + TCusType.Text, "'"));
		left = Operators.ConcatenateObject(left, ")");
		Module1.connect(Conversions.ToString(left));
		return text;
	}

	public void SAVE_EDIT()
	{
		sum();
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการแก\u0e49ไขหร\u0e37อไม\u0e48", "แก\u0e49ไข", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.Yes)
		{
			return;
		}
		update_cust();
		DataSet dataSet = Module1.connect("select Book_ID,Book_Status,Book_Price_Pay,Book_Cust_ID from HT_Book_H where Book_ID='" + EDIT_ID + "'");
		string sIR_PAY = Module1.GetSIR_PAY();
		if (decimal.Compare(EDIT_PRICE, Conversions.ToDecimal(Tpay.Text)) != 0)
		{
			if (decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) > 0)
			{
				MyProject.Forms.FormConfirmPay.PTOTAl = Conversions.ToDecimal(Tpay.Text);
				MyProject.Forms.FormConfirmPay.ShowDialog();
				if (!MyProject.Forms.FormConfirmPay.ISOK)
				{
					Button7.Enabled = true;
					return;
				}
			}
			if (decimal.Compare(EDIT_PRICE, 0m) != 0)
			{
				DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Pay where cin_no='" + TdocNum.Text + "' order by id desc");
				if (dataSet2.Tables[0].Rows.Count != 0)
				{
					Module1.Insert_Pay(EDIT_ID, "การจองแบบระบ\u0e38ห\u0e49อง", DateTime.Now, Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_cash"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_credit"])), "ค\u0e37นเง\u0e34นจองห\u0e49อง", Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_ds_priceTotal"])), "รายการ", sIR_PAY, Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_ID"]), "P001", 1m, Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_ds_priceTotal"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_ds_priceTotal"])), "", Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_free"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_tran"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_web"])));
				}
				Module1.UPDATE_MONEY(Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_ID"]), Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["Book_Price_Pay"]), "DEL", "ค\u0e37นเง\u0e34นจากการแก\u0e49ไขใบจอง " + EDIT_ID);
			}
		}
		Module1.connect("update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' where room_book in (select id from ht_book_date  where Book_no='" + EDIT_ID + "')");
		Module1.connect("delete from  HT_Book_Date where Book_no='" + EDIT_ID + "'");
		Module1.connect("delete from  HT_Book_H where Book_ID='" + EDIT_ID + "'");
		Module1.connect("delete from  HT_Book_Ds where Book_no='" + EDIT_ID + "'");
		Module1.connect("delete from  HT_Book_Pro where [B_NO]='" + EDIT_ID + "'");
		string text = "";
		text = ((Operators.CompareString(TCusID.Text, "", TextCompare: false) == 0) ? SAVE_CUST() : TCusID.Text);
		ArrayList arrayList = new ArrayList();
		string text2 = "";
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
					bool flag = false;
					int num5 = arrayList.Count - 1;
					int num6 = 0;
					while (true)
					{
						int num7 = num6;
						num4 = num5;
						if (num7 <= num4)
						{
							if (!Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num6], new object[1] { 0 }, null), Conversions.ToString(Grid1[num2, 2]), TextCompare: false))
							{
								num6++;
								continue;
							}
							flag = true;
							NewLateBinding.LateIndexSetComplex(arrayList[num6], new object[2]
							{
								1,
								decimal.Add(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num6], new object[1] { 1 }, null)), 1m)
							}, null, OptimisticSet: false, RValueBase: true);
							break;
						}
						break;
					}
					if (!flag)
					{
						string[] value = new string[2]
						{
							Conversions.ToString(Grid1[num2, 2]),
							Conversions.ToString(1)
						};
						arrayList.Add(value);
					}
				}
				num2++;
			}
			int num8 = arrayList.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 > num4)
				{
					break;
				}
				text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(arrayList[num9], new object[1] { 0 }, null), " "), NewLateBinding.LateIndexGet(arrayList[num9], new object[1] { 1 }, null)), " ห\u0e49อง "), "\r\n")));
				num9++;
			}
			string text3 = "";
			int num11 = Grid1.Rows.Count - 1;
			int num12 = 1;
			while (true)
			{
				int num13 = num12;
				int num4 = num11;
				if (num13 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num12, 1]), "", TextCompare: false) != 0 && Operators.CompareString(Conversions.ToString(Grid1[num12, 9]), "", TextCompare: false) != 0)
				{
					text3 = text3 + " " + Conversions.ToString(Grid1[num12, 9]);
				}
				num12++;
			}
			DateTime dateTime = DateTime.Now;
			DateTime dateTime2 = DateTime.Now;
			int num14 = Grid1.Rows.Count - 1;
			int num15 = 1;
			while (true)
			{
				int num16 = num15;
				int num4 = num14;
				if (num16 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num15, 1]), "", TextCompare: false) != 0)
				{
					if (Operators.ConditionalCompareObjectLess(Grid1[num15, 3], dateTime, TextCompare: false))
					{
						dateTime = Conversions.ToDate(Grid1[num15, 3]);
					}
					if (Operators.ConditionalCompareObjectGreater(Grid1[num15, 4], dateTime2, TextCompare: false))
					{
						dateTime2 = Conversions.ToDate(Grid1[num15, 4]);
					}
					if (num15 == 1)
					{
						dateTime = Conversions.ToDate(Grid1[num15, 3]);
						dateTime2 = Conversions.ToDate(Grid1[num15, 4]);
					}
				}
				num15++;
			}
			TdocNum.Text = EDIT_ID;
			object left = "INSERT INTO [HT_Book_H]";
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, " [Book_ID]");
			left = Operators.ConcatenateObject(left, ",[Book_Date]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_ID]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_Name]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_Name2]");
			left = Operators.ConcatenateObject(left, ",[Book_Cust_Tel]");
			left = Operators.ConcatenateObject(left, ",[Book_Price_Total]");
			left = Operators.ConcatenateObject(left, ",[Book_Price_Pay]");
			left = Operators.ConcatenateObject(left, ",[Book_Status],[Book_Date_in],[Book_Date_out],[Book_by],[Book_room_all],[Book_room_note],[book_room_type],Book_Notify_Day,Book_sale)");
			left = Operators.ConcatenateObject(left, "VALUES");
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, string.Concat(" '" + TdocNum.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTimePicker1.Value), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + TcusName.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBox_1.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Tc_tel.Text, "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(LabelTroom.Text)));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(Tpay.Text)));
			left = Operators.ConcatenateObject(left, ",'จอง'");
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(dateTime.Date), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(dateTime2.Date), "'"));
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Module1.loginName), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + text2, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(",'" + Tnote.Text, " "), text3), "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(2));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToInteger(Tdays.Text)));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + ComboSale.Text, "'"));
			left = Operators.ConcatenateObject(left, ")");
			Module1.connect(Conversions.ToString(left));
			int num17 = Grid1.Rows.Count - 1;
			int num18 = 1;
			while (true)
			{
				int num19 = num18;
				int num4 = num17;
				if (num19 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid1[num18, 1]), "", TextCompare: false) != 0)
				{
					left = "INSERT INTO [HT_Book_Ds]";
					left = Operators.ConcatenateObject(left, "([Book_No]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Type]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Start]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_End]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Price]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Night]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Num]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_PriceToTal]");
					left = Operators.ConcatenateObject(left, ",[Book_Room_Note])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, string.Concat(" '" + TdocNum.Text, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Grid1[num18, 2]), "'"));
					left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num18, 3]), "'"));
					left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Grid1[num18, 4]), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 5])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 6])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 7])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 8])), "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Grid1[num18, 9]), "'"));
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
					int num20 = Convert.ToInt32(decimal.Subtract(Conversions.ToDecimal(Grid1[num18, 6]), 1m));
					int num21 = 0;
					while (true)
					{
						int num22 = num21;
						num4 = num20;
						if (num22 > num4)
						{
							break;
						}
						DateTime dateTime3 = Conversions.ToDate(Grid1[num18, 3]);
						if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num18, 3]), "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(RuntimeHelpers.GetObjectValue(Grid1[num18, 3]), "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
						{
							dateTime3 = dateTime3.AddDays(-1.0);
						}
						object right = Module1.get_id("HT_Book_Date", "id");
						object left2 = "INSERT INTO [HT_Book_Date]";
						left2 = Operators.ConcatenateObject(left2, "([id]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_no]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_type]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_date_ds]");
						left2 = Operators.ConcatenateObject(left2, ",[Book_Num],[Book_USE])");
						left2 = Operators.ConcatenateObject(left2, "VALUES");
						left2 = Operators.ConcatenateObject(left2, "(");
						left2 = Operators.ConcatenateObject(left2, right);
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + TdocNum.Text, "'"));
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(Grid1[num18, 2]), "'"));
						left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(dateTime3.AddDays(num21).Date), "'"));
						left2 = Operators.ConcatenateObject(left2, "," + Conversions.ToString(Conversions.ToDecimal(Grid1[num18, 7])));
						left2 = Operators.ConcatenateObject(left2, ",0");
						left2 = Operators.ConcatenateObject(left2, ")");
						Module1.connect(Conversions.ToString(left2));
						num21++;
					}
				}
				num18++;
			}
			int num23 = Grid2.Rows.Count - 1;
			int num24 = 1;
			while (true)
			{
				int num25 = num24;
				int num4 = num23;
				if (num25 > num4)
				{
					break;
				}
				if (Operators.CompareString(Conversions.ToString(Grid2[num24, 1]), "", TextCompare: false) != 0)
				{
					object left3 = "INSERT INTO [HT_Book_Pro]";
					left3 = Operators.ConcatenateObject(left3, "(");
					left3 = Operators.ConcatenateObject(left3, "[B_NO]");
					left3 = Operators.ConcatenateObject(left3, ",[B_ROOM]");
					left3 = Operators.ConcatenateObject(left3, ",[B_NAME]");
					left3 = Operators.ConcatenateObject(left3, ",[B_UNIT]");
					left3 = Operators.ConcatenateObject(left3, ",[B_NUM]");
					left3 = Operators.ConcatenateObject(left3, ",[B_PRICE]");
					left3 = Operators.ConcatenateObject(left3, ",[B_PRICE_TOTAL],[B_PRO_ID])");
					left3 = Operators.ConcatenateObject(left3, "VALUES");
					left3 = Operators.ConcatenateObject(left3, "(");
					left3 = Operators.ConcatenateObject(left3, string.Concat("'" + TdocNum.Text, "'"));
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + Conversions.ToString(Grid2[num24, 2]), "'"));
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + Conversions.ToString(Grid2[num24, 3]), "'"));
					left3 = Operators.ConcatenateObject(left3, string.Concat(",'" + Conversions.ToString(Grid2[num24, 4]), "'"));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num24, 5])));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num24, 6])));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToDecimal(Grid2[num24, 7])));
					left3 = Operators.ConcatenateObject(left3, "," + Conversions.ToString(Conversions.ToInteger(Grid2[num24, 8])));
					left3 = Operators.ConcatenateObject(left3, ")");
					Module1.connect(Conversions.ToString(left3));
				}
				num24++;
			}
			if (decimal.Compare(EDIT_PRICE, Conversions.ToDecimal(Tpay.Text)) != 0 && decimal.Compare(Conversions.ToDecimal(Tpay.Text), 0m) > 0)
			{
				sIR_PAY = Module1.GetSIR_PAY();
				Module1.Insert_Pay(TdocNum.Text, "การจองแบบระบ\u0e38ห\u0e49อง", DateTime.Now, MyProject.Forms.FormConfirmPay.PCASH, MyProject.Forms.FormConfirmPay.PCREDIT, "จ\u0e48ายล\u0e48วงหน\u0e49า", Conversions.ToDecimal(Tpay.Text), "รายการ", sIR_PAY, text, "P001", 1m, Conversions.ToDecimal(Tpay.Text), Conversions.ToDecimal(Tpay.Text), Tnote.Text, MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
				Module1.UPDATE_MONEY(text, Conversions.ToDecimal(Tpay.Text), "ADD", "จ\u0e48ายล\u0e48วงหน\u0e49าจากใบจอง " + TdocNum.Text);
				if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
				{
					Print_Report.Print_Sale(sIR_PAY, preview: false);
				}
			}
			MessageBox.Show("บ\u0e31นท\u0e36กแก\u0e49ไขเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			Close();
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(EDIT_ID, "", TextCompare: false) != 0)
		{
			Module1.connect("update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' where room_book in (select id from ht_book_date  where Book_no='" + EDIT_ID + "')");
			Module1.connect("update HT_book_date set book_ok=0 where Book_no='" + EDIT_ID + "'");
		}
		FormBookRooms formBookRooms = new FormBookRooms();
		formBookRooms.Room_Old.Clear();
		checked
		{
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
				string[] value = new string[3]
				{
					Conversions.ToString(Grid1[num2, 2]),
					Conversions.ToString(Conversions.ToDate(Grid1[num2, 3]).Date),
					Conversions.ToString(Grid1[num2, 6])
				};
				formBookRooms.Room_Old.Add(value);
				num2++;
			}
			formBookRooms.start_date = Tstart.Value;
			formBookRooms.ShowDialog();
			if (formBookRooms.Room_Arr.Count != 0)
			{
				Grid1.Rows.RemoveRange(1, 299);
				Grid1.Rows.Add(299);
				int num5 = 1;
				int num6 = Grid1.Rows.Count - 1;
				int num7 = 0;
				while (true)
				{
					int num8 = num7;
					int num4 = num6;
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
				int num9 = formBookRooms.Room_Arr.Count - 1;
				int num10 = 0;
				while (true)
				{
					int num11 = num10;
					int num4 = num9;
					if (num11 > num4)
					{
						break;
					}
					DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select Room_type,Room_no from HT_Rooms where Room_no='", NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 0 }, null)), "'")));
					dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms_Price where room_type='", dataSet.Tables[0].Rows[0]["Room_type"]), "' and room_custType='"), TCusType.Text), "' ")));
					decimal num12 = default(decimal);
					if (dataSet.Tables[0].Rows.Count != 0)
					{
						num12 = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["room_price"]);
					}
					Grid1[num5, 1] = num5;
					Grid1[num5, 2] = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 0 }, null));
					Grid1[num5, 3] = Operators.ConcatenateObject(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 1 }, null), " 12:00:00");
					Grid1[num5, 4] = Operators.ConcatenateObject(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 2 }, null), " 11:59:59");
					Grid1[num5, 5] = num12;
					Grid1[num5, 6] = DateAndTime.DateDiff(DateInterval.Day, Conversions.ToDate(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 1 }, null)), Conversions.ToDate(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 2 }, null)));
					Grid1[num5, 7] = 1;
					Grid1[num5, 8] = Strings.Format(decimal.Multiply(decimal.Multiply(new decimal(DateAndTime.DateDiff(DateInterval.Day, Conversions.ToDate(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 1 }, null)), Conversions.ToDate(NewLateBinding.LateIndexGet(formBookRooms.Room_Arr[num10], new object[1] { 2 }, null)))), num12), 1m), "#,##0.00");
					Grid1[num5, 9] = "";
					num5++;
					num10++;
				}
				num5++;
			}
			sum();
			AddItemInCombobox();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
	}

	private void TCusType_SelectedIndexChanged(object sender, EventArgs e)
	{
		checked
		{
			try
			{
				int num = Grid1.Rows.Count - 1;
				int num2 = 1;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4 && Operators.CompareString(Conversions.ToString(Grid1[num2, 1]), "", TextCompare: false) != 0)
					{
						DataSet dataSet = Module1.connect("select Room_type,Room_no from HT_Rooms where Room_no='" + Conversions.ToString(Grid1[num2, 2]) + "'");
						dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms_Price where room_type='", dataSet.Tables[0].Rows[0]["Room_type"]), "' and room_custType='"), TCusType.Text), "' ")));
						decimal num5 = default(decimal);
						if (dataSet.Tables[0].Rows.Count != 0)
						{
							num5 = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["room_price"]);
						}
						Grid1[num2, 5] = num5;
						Grid1[num2, 8] = Strings.Format(Operators.MultiplyObject(Operators.MultiplyObject(Grid1[num2, 6], num5), 1), "#,##0.00");
						num2++;
						continue;
					}
					break;
				}
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void Tstart_ValueChanged(object sender, EventArgs e)
	{
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
			Grid2[num3, 3] = Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["Pro_name"], right);
			Grid2[num3, 4] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Pro_Unit"]);
			Grid2[num3, 5] = num2;
			Grid2[num3, 6] = Strings.Format(num, "#,##0.00");
			Grid2[num3, 7] = Strings.Format(decimal.Multiply(num, num2), "#,##0.00");
			Grid2[num3, 8] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["id"]);
			sum();
		}
	}

	private void Button9_Click_1(object sender, EventArgs e)
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

	private void Grid1_Click(object sender, EventArgs e)
	{
	}

	private void Grid2_AfterEdit(object sender, RowColEventArgs e)
	{
		if ((e.Col == 5) | (e.Col == 6))
		{
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid2[e.Row, 5])))
			{
				Grid2[e.Row, 5] = 1;
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(Grid2[e.Row, 6])))
			{
				Grid2[e.Row, 6] = 0;
			}
			Grid2[e.Row, 7] = Operators.MultiplyObject(Grid2[e.Row, 5], Grid2[e.Row, 6]);
			sum();
		}
	}

	private void Grid2_Click(object sender, EventArgs e)
	{
	}

	private void ComboSale_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Tc_tel_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Tnum_LostFocus(object sender, EventArgs e)
	{
		if (Operators.CompareString(Tnum.Text, "", TextCompare: false) == 0)
		{
			Tnum.Text = Conversions.ToString(20);
		}
		if (!Versioned.IsNumeric(Tnum.Text))
		{
			Tnum.Text = Conversions.ToString(20);
		}
		Module1.save_book_num(Tnum.Text);
	}

	private void Tnum_TextChanged(object sender, EventArgs e)
	{
	}
}
