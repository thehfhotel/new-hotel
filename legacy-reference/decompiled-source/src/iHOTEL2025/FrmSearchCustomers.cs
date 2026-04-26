using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using DevComponents.Editors;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmSearchCustomers : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("GroupBox2")]
	private GroupBox _GroupBox2;

	[AccessedThroughProperty("mode")]
	private Label _mode;

	[AccessedThroughProperty("Tname2")]
	private TextBoxX _Tname2;

	[AccessedThroughProperty("Taddress")]
	private TextBoxX _Taddress;

	[AccessedThroughProperty("Tcard_id")]
	private TextBoxX _Tcard_id;

	[AccessedThroughProperty("Ttel2")]
	private TextBoxX _Ttel2;

	[AccessedThroughProperty("Ttel1")]
	private TextBoxX _Ttel1;

	[AccessedThroughProperty("Tname1")]
	private TextBoxX _Tname1;

	[AccessedThroughProperty("Tid")]
	private TextBoxX _Tid;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("Label17")]
	private Label _Label17;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("ยกเล\u0e34ก")]
	private ButtonX buttonX_0;

	[AccessedThroughProperty("บ\u0e31นท\u0e36ก")]
	private ButtonX buttonX_1;

	[AccessedThroughProperty("ListViewEx1")]
	private global::PrintableListView.PrintableListView _ListViewEx1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("Tsearch")]
	private TextBoxX _Tsearch;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("ลบ")]
	private ButtonX buttonX_2;

	[AccessedThroughProperty("แก\u0e49ไข")]
	private ButtonX buttonX_3;

	[AccessedThroughProperty("เพ\u0e34\u0e48ม")]
	private ButtonX buttonX_4;

	[AccessedThroughProperty("ttype")]
	private ComboBox _ttype;

	[AccessedThroughProperty("ComboBoxEx1")]
	private ComboBoxEx _ComboBoxEx1;

	[AccessedThroughProperty("TsName")]
	private TextBoxX _TsName;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("SaveFileDialog1")]
	private SaveFileDialog _SaveFileDialog1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("Temail")]
	private TextBoxX _Temail;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Tpackage")]
	private TextBoxX _Tpackage;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	private string modeAdd;

	private string modeedit;

	private int editID;

	public static Bitmap myBitmap;

	public ArrayList Return_ARR;

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

	internal virtual GroupBox GroupBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox2 = value;
		}
	}

	internal virtual Label mode
	{
		[DebuggerNonUserCode]
		get
		{
			return _mode;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_mode = value;
		}
	}

	internal virtual TextBoxX Tname2
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

	internal virtual TextBoxX Taddress
	{
		[DebuggerNonUserCode]
		get
		{
			return _Taddress;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Taddress = value;
		}
	}

	internal virtual TextBoxX Tcard_id
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcard_id;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcard_id = value;
		}
	}

	internal virtual TextBoxX Ttel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ttel2 = value;
		}
	}

	internal virtual TextBoxX Ttel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ttel1 = value;
		}
	}

	internal virtual TextBoxX Tname1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tname1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tname1 = value;
		}
	}

	internal virtual TextBoxX Tid
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tid;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tid = value;
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

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			buttonX_0 = value;
		}
	}

	internal virtual ButtonX ButtonX_1
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			buttonX_1 = value;
		}
	}

	internal virtual global::PrintableListView.PrintableListView ListViewEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListViewEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ListViewEx1 = value;
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

	internal virtual ColumnHeader ColumnHeader8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader8 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader9 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader10
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader10 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader11
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader11 = value;
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

	internal virtual ColumnHeader ColumnHeader12
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader12 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader15
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader15 = value;
		}
	}

	internal virtual TextBoxX Tsearch
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tsearch;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tsearch = value;
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
			_ButtonX1 = value;
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

	internal virtual ButtonX ButtonX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			buttonX_2 = value;
		}
	}

	internal virtual ButtonX ButtonX_3
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			buttonX_3 = value;
		}
	}

	internal virtual ButtonX ButtonX_4
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			buttonX_4 = value;
		}
	}

	internal virtual ComboBox ttype
	{
		[DebuggerNonUserCode]
		get
		{
			return _ttype;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ttype = value;
		}
	}

	internal virtual ComboBoxEx ComboBoxEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBoxEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBoxEx1 = value;
		}
	}

	internal virtual TextBoxX TsName
	{
		[DebuggerNonUserCode]
		get
		{
			return _TsName;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TsName = value;
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
			_Timer1 = value;
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
			_ButtonX2 = value;
		}
	}

	internal virtual SaveFileDialog SaveFileDialog1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SaveFileDialog1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SaveFileDialog1 = value;
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
			_ButtonX3 = value;
		}
	}

	internal virtual TextBoxX Temail
	{
		[DebuggerNonUserCode]
		get
		{
			return _Temail;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Temail = value;
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

	internal virtual TextBoxX Tpackage
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpackage;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpackage = value;
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

	internal virtual ColumnHeader ColumnHeader13
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader13 = value;
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
			_ButtonX4 = value;
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

	[DebuggerNonUserCode]
	static FrmSearchCustomers()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmSearchCustomers()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		modeAdd = "เพ\u0e34\u0e48มช\u0e37\u0e48อล\u0e39กค\u0e49า";
		modeedit = "แก\u0e49ไขช\u0e37\u0e48อล\u0e39กค\u0e49า";
		editID = 0;
		Return_ARR = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmSearchCustomers));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ListViewEx1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_4 = new DevComponents.DotNetBar.ButtonX();
		this.ComboBoxEx1 = new DevComponents.DotNetBar.Controls.ComboBoxEx();
		this.TsName = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tsearch = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.ttype = new System.Windows.Forms.ComboBox();
		this.mode = new System.Windows.Forms.Label();
		this.Tpackage = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Temail = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tname2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Taddress = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tcard_id = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Ttel2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Ttel1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tname1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tid = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.Label17 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label14 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label9 = new System.Windows.Forms.Label();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.SaveFileDialog1 = new System.Windows.Forms.SaveFileDialog();
		this.PanelEx1.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Controls.Add(this.GroupBox2);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx1.Font = new System.Drawing.Font("Microsoft Sans Serif", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(847, 458);
		panelEx3.Size = size;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.Style.LineAlignment = System.Drawing.StringAlignment.Near;
		this.PanelEx1.TabIndex = 0;
		this.PanelEx1.Text = " ล\u0e39กค\u0e49า";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.CheckBox1);
		this.GroupBox1.Controls.Add(this.ButtonX4);
		this.GroupBox1.Controls.Add(this.ListViewEx1);
		this.GroupBox1.Controls.Add(this.ButtonX3);
		this.GroupBox1.Controls.Add(this.ButtonX2);
		this.GroupBox1.Controls.Add(this.ButtonX_2);
		this.GroupBox1.Controls.Add(this.ButtonX_3);
		this.GroupBox1.Controls.Add(this.ButtonX_4);
		this.GroupBox1.Controls.Add(this.ComboBoxEx1);
		this.GroupBox1.Controls.Add(this.TsName);
		this.GroupBox1.Controls.Add(this.Tsearch);
		this.GroupBox1.Controls.Add(this.ButtonX1);
		this.GroupBox1.Controls.Add(this.Label8);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		location = new System.Drawing.Point(5, 33);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox1;
		size = new System.Drawing.Size(830, 414);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 12;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.CheckBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(13, 378);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(181, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 27;
		this.CheckBox1.Text = "เล\u0e37อกท\u0e31\u0e49งหมด/ไม\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX4;
		location = new System.Drawing.Point(712, 373);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX4;
		size = new System.Drawing.Size(106, 28);
		buttonX3.Size = size;
		this.ButtonX4.TabIndex = 26;
		this.ButtonX4.Text = "เล\u0e37อก";
		this.ListViewEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListViewEx1.Atto_กระดาษแนวนอน = false;
		this.ListViewEx1.CheckBoxes = true;
		this.ListViewEx1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[14]
		{
			this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader2, this.ColumnHeader8, this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader11, this.ColumnHeader7, this.ColumnHeader12, this.ColumnHeader15,
			this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader13
		});
		this.ListViewEx1.FitToPage = true;
		this.ListViewEx1.FullRowSelect = true;
		this.ListViewEx1.GridLines = true;
		global::PrintableListView.PrintableListView listViewEx = this.ListViewEx1;
		location = new System.Drawing.Point(13, 92);
		listViewEx.Location = location;
		global::PrintableListView.PrintableListView listViewEx2 = this.ListViewEx1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listViewEx2.Margin = margin;
		this.ListViewEx1.MultiSelect = false;
		this.ListViewEx1.Name = "ListViewEx1";
		global::PrintableListView.PrintableListView listViewEx3 = this.ListViewEx1;
		size = new System.Drawing.Size(806, 277);
		listViewEx3.Size = size;
		this.ListViewEx1.TabIndex = 16;
		this.ListViewEx1.Title = "";
		this.ListViewEx1.Title2 = "";
		this.ListViewEx1.Title2Tab = "";
		this.ListViewEx1.Title3 = "";
		this.ListViewEx1.Title3Tab = "";
		this.ListViewEx1.UseCompatibleStateImageBehavior = false;
		this.ListViewEx1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "";
		this.ColumnHeader1.Width = 20;
		this.ColumnHeader6.Text = "รห\u0e31ส";
		this.ColumnHeader6.Width = 70;
		this.ColumnHeader2.Text = "บาร\u0e4cโค\u0e49ด";
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader8.Text = "ช\u0e37\u0e48อ คนท\u0e35\u0e481";
		this.ColumnHeader8.Width = 150;
		this.ColumnHeader9.Text = "ช\u0e37\u0e48อ คนท\u0e35\u0e482";
		this.ColumnHeader9.Width = 150;
		this.ColumnHeader10.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.ColumnHeader10.Width = 200;
		this.ColumnHeader11.Text = "โทรศ\u0e31พท\u0e4c 1";
		this.ColumnHeader11.Width = 100;
		this.ColumnHeader7.Text = "โทรศ\u0e31พท\u0e4c 2";
		this.ColumnHeader7.Width = 100;
		this.ColumnHeader12.Text = "ประเภทสมาช\u0e34ก";
		this.ColumnHeader12.Width = 100;
		this.ColumnHeader15.Text = "หมายเหต\u0e38";
		this.ColumnHeader15.Width = 120;
		this.ColumnHeader3.Text = "Email";
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader4.Text = "Package";
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader5.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e47นสมาช\u0e34ก";
		this.ColumnHeader5.Width = 120;
		this.ColumnHeader13.Text = "สามาช\u0e34กOA";
		this.ColumnHeader13.Width = 0;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX3;
		location = new System.Drawing.Point(772, 377);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX5.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(106, 28);
		buttonX6.Size = size;
		this.ButtonX3.TabIndex = 25;
		this.ButtonX3.Text = "พ\u0e34มพ\u0e4c";
		this.ButtonX3.Visible = false;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX2;
		location = new System.Drawing.Point(646, 377);
		buttonX7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX8.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX2;
		size = new System.Drawing.Size(120, 28);
		buttonX9.Size = size;
		this.ButtonX2.TabIndex = 5;
		this.ButtonX2.Text = "ส\u0e48งออก Excel";
		this.ButtonX2.Visible = false;
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX_2;
		location = new System.Drawing.Point(551, 377);
		buttonX10.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX_2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX11.Margin = margin;
		this.ButtonX_2.Name = "ลบ";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX_2;
		size = new System.Drawing.Size(87, 28);
		buttonX12.Size = size;
		this.ButtonX_2.TabIndex = 5;
		this.ButtonX_2.Text = "ลบ";
		this.ButtonX_2.Visible = false;
		this.ButtonX_3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX_3;
		location = new System.Drawing.Point(458, 377);
		buttonX13.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX_3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX14.Margin = margin;
		this.ButtonX_3.Name = "แก\u0e49ไข";
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX_3;
		size = new System.Drawing.Size(87, 28);
		buttonX15.Size = size;
		this.ButtonX_3.TabIndex = 4;
		this.ButtonX_3.Text = "แก\u0e49ไข";
		this.ButtonX_3.Visible = false;
		this.ButtonX_4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX_4;
		location = new System.Drawing.Point(366, 377);
		buttonX16.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX_4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX17.Margin = margin;
		this.ButtonX_4.Name = "เพ\u0e34\u0e48ม";
		DevComponents.DotNetBar.ButtonX buttonX18 = this.ButtonX_4;
		size = new System.Drawing.Size(87, 28);
		buttonX18.Size = size;
		this.ButtonX_4.TabIndex = 3;
		this.ButtonX_4.Text = "เพ\u0e34\u0e48ม";
		this.ButtonX_4.Visible = false;
		this.ComboBoxEx1.DisplayMember = "Text";
		this.ComboBoxEx1.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawFixed;
		this.ComboBoxEx1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBoxEx1.FormattingEnabled = true;
		this.ComboBoxEx1.ItemHeight = 17;
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx = this.ComboBoxEx1;
		location = new System.Drawing.Point(265, 23);
		comboBoxEx.Location = location;
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx2 = this.ComboBoxEx1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		comboBoxEx2.Margin = margin;
		this.ComboBoxEx1.Name = "ComboBoxEx1";
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx3 = this.ComboBoxEx1;
		size = new System.Drawing.Size(147, 23);
		comboBoxEx3.Size = size;
		this.ComboBoxEx1.TabIndex = 1;
		this.TsName.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tsName = this.TsName;
		location = new System.Drawing.Point(86, 57);
		tsName.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tsName2 = this.TsName;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tsName2.Margin = margin;
		this.TsName.Name = "TsName";
		DevComponents.DotNetBar.Controls.TextBoxX tsName3 = this.TsName;
		size = new System.Drawing.Size(234, 23);
		tsName3.Size = size;
		this.TsName.TabIndex = 0;
		this.Tsearch.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tsearch = this.Tsearch;
		location = new System.Drawing.Point(86, 23);
		tsearch.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tsearch2 = this.Tsearch;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tsearch2.Margin = margin;
		this.Tsearch.Name = "Tsearch";
		DevComponents.DotNetBar.Controls.TextBoxX tsearch3 = this.Tsearch;
		size = new System.Drawing.Size(76, 23);
		tsearch3.Size = size;
		this.Tsearch.TabIndex = 0;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX19 = this.ButtonX1;
		location = new System.Drawing.Point(325, 57);
		buttonX19.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX20 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX20.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX21 = this.ButtonX1;
		size = new System.Drawing.Size(87, 23);
		buttonX21.Size = size;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ค\u0e49นหา";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label = this.Label8;
		location = new System.Drawing.Point(192, 27);
		label.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label2 = this.Label8;
		size = new System.Drawing.Size(70, 16);
		label2.Size = size;
		this.Label8.TabIndex = 11;
		this.Label8.Text = "ประเภล\u0e39กค\u0e49า";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label1;
		location = new System.Drawing.Point(30, 61);
		label3.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label4 = this.Label1;
		size = new System.Drawing.Size(54, 16);
		label4.Size = size;
		this.Label1.TabIndex = 11;
		this.Label1.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label7;
		location = new System.Drawing.Point(22, 27);
		label5.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label6 = this.Label7;
		size = new System.Drawing.Size(60, 16);
		label6.Size = size;
		this.Label7.TabIndex = 11;
		this.Label7.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox2.Controls.Add(this.DateTimePicker1);
		this.GroupBox2.Controls.Add(this.ttype);
		this.GroupBox2.Controls.Add(this.mode);
		this.GroupBox2.Controls.Add(this.Tpackage);
		this.GroupBox2.Controls.Add(this.Temail);
		this.GroupBox2.Controls.Add(this.Tname2);
		this.GroupBox2.Controls.Add(this.Taddress);
		this.GroupBox2.Controls.Add(this.Tcard_id);
		this.GroupBox2.Controls.Add(this.Ttel2);
		this.GroupBox2.Controls.Add(this.Ttel1);
		this.GroupBox2.Controls.Add(this.Tname1);
		this.GroupBox2.Controls.Add(this.Tid);
		this.GroupBox2.Controls.Add(this.Label11);
		this.GroupBox2.Controls.Add(this.Label13);
		this.GroupBox2.Controls.Add(this.Label17);
		this.GroupBox2.Controls.Add(this.Label4);
		this.GroupBox2.Controls.Add(this.Label3);
		this.GroupBox2.Controls.Add(this.Label14);
		this.GroupBox2.Controls.Add(this.Label2);
		this.GroupBox2.Controls.Add(this.Label12);
		this.GroupBox2.Controls.Add(this.Label10);
		this.GroupBox2.Controls.Add(this.Label6);
		this.GroupBox2.Controls.Add(this.Label9);
		this.GroupBox2.Controls.Add(this.ButtonX_0);
		this.GroupBox2.Controls.Add(this.ButtonX_1);
		this.GroupBox2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox5 = this.GroupBox2;
		location = new System.Drawing.Point(442, 33);
		groupBox5.Location = location;
		System.Windows.Forms.GroupBox groupBox6 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox6.Margin = margin;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox7 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox7.Padding = margin;
		System.Windows.Forms.GroupBox groupBox8 = this.GroupBox2;
		size = new System.Drawing.Size(388, 414);
		groupBox8.Size = size;
		this.GroupBox2.TabIndex = 12;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไข";
		this.GroupBox2.Visible = false;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(113, 268);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(258, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 23;
		this.ttype.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ttype.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox = this.ttype;
		location = new System.Drawing.Point(114, 186);
		comboBox.Location = location;
		System.Windows.Forms.ComboBox comboBox2 = this.ttype;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		comboBox2.Margin = margin;
		this.ttype.Name = "ttype";
		System.Windows.Forms.ComboBox comboBox3 = this.ttype;
		size = new System.Drawing.Size(258, 24);
		comboBox3.Size = size;
		this.ttype.TabIndex = 15;
		this.mode.ForeColor = System.Drawing.Color.Brown;
		System.Windows.Forms.Label label7 = this.mode;
		location = new System.Drawing.Point(22, 373);
		label7.Location = location;
		this.mode.Name = "mode";
		System.Windows.Forms.Label label8 = this.mode;
		size = new System.Drawing.Size(166, 28);
		label8.Size = size;
		this.mode.TabIndex = 22;
		this.mode.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Tpackage.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tpackage = this.Tpackage;
		location = new System.Drawing.Point(113, 241);
		tpackage.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tpackage2 = this.Tpackage;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tpackage2.Margin = margin;
		this.Tpackage.MaxLength = 255;
		this.Tpackage.Name = "Tpackage";
		DevComponents.DotNetBar.Controls.TextBoxX tpackage3 = this.Tpackage;
		size = new System.Drawing.Size(258, 23);
		tpackage3.Size = size;
		this.Tpackage.TabIndex = 8;
		this.Temail.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX temail = this.Temail;
		location = new System.Drawing.Point(113, 214);
		temail.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX temail2 = this.Temail;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		temail2.Margin = margin;
		this.Temail.MaxLength = 255;
		this.Temail.Name = "Temail";
		DevComponents.DotNetBar.Controls.TextBoxX temail3 = this.Temail;
		size = new System.Drawing.Size(258, 23);
		temail3.Size = size;
		this.Temail.TabIndex = 8;
		this.Tname2.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tname = this.Tname2;
		location = new System.Drawing.Point(114, 84);
		tname.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tname2 = this.Tname2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tname2.Margin = margin;
		this.Tname2.MaxLength = 255;
		this.Tname2.Name = "Tname2";
		DevComponents.DotNetBar.Controls.TextBoxX tname3 = this.Tname2;
		size = new System.Drawing.Size(257, 23);
		tname3.Size = size;
		this.Tname2.TabIndex = 8;
		this.Taddress.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX taddress = this.Taddress;
		location = new System.Drawing.Point(113, 111);
		taddress.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX taddress2 = this.Taddress;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		taddress2.Margin = margin;
		this.Taddress.MaxLength = 255;
		this.Taddress.Multiline = true;
		this.Taddress.Name = "Taddress";
		DevComponents.DotNetBar.Controls.TextBoxX taddress3 = this.Taddress;
		size = new System.Drawing.Size(258, 44);
		taddress3.Size = size;
		this.Taddress.TabIndex = 12;
		this.Tcard_id.Border.Class = "TextBoxBorder";
		this.Tcard_id.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX tcard_id = this.Tcard_id;
		location = new System.Drawing.Point(114, 295);
		tcard_id.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tcard_id2 = this.Tcard_id;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcard_id2.Margin = margin;
		this.Tcard_id.MaxLength = 20;
		this.Tcard_id.Multiline = true;
		this.Tcard_id.Name = "Tcard_id";
		DevComponents.DotNetBar.Controls.TextBoxX tcard_id3 = this.Tcard_id;
		size = new System.Drawing.Size(257, 69);
		tcard_id3.Size = size;
		this.Tcard_id.TabIndex = 18;
		this.Ttel2.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX ttel = this.Ttel2;
		location = new System.Drawing.Point(275, 158);
		ttel.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX ttel2 = this.Ttel2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		ttel2.Margin = margin;
		this.Ttel2.Name = "Ttel2";
		DevComponents.DotNetBar.Controls.TextBoxX ttel3 = this.Ttel2;
		size = new System.Drawing.Size(96, 23);
		ttel3.Size = size;
		this.Ttel2.TabIndex = 14;
		this.Ttel1.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX ttel4 = this.Ttel1;
		location = new System.Drawing.Point(113, 158);
		ttel4.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX ttel5 = this.Ttel1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		ttel5.Margin = margin;
		this.Ttel1.Name = "Ttel1";
		DevComponents.DotNetBar.Controls.TextBoxX ttel6 = this.Ttel1;
		size = new System.Drawing.Size(90, 23);
		ttel6.Size = size;
		this.Ttel1.TabIndex = 13;
		this.Tname1.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tname4 = this.Tname1;
		location = new System.Drawing.Point(114, 57);
		tname4.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tname5 = this.Tname1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tname5.Margin = margin;
		this.Tname1.MaxLength = 255;
		this.Tname1.Name = "Tname1";
		DevComponents.DotNetBar.Controls.TextBoxX tname6 = this.Tname1;
		size = new System.Drawing.Size(257, 23);
		tname6.Size = size;
		this.Tname1.TabIndex = 7;
		this.Tid.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tid = this.Tid;
		location = new System.Drawing.Point(114, 27);
		tid.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX tid2 = this.Tid;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tid2.Margin = margin;
		this.Tid.MaxLength = 255;
		this.Tid.Name = "Tid";
		DevComponents.DotNetBar.Controls.TextBoxX tid3 = this.Tid;
		size = new System.Drawing.Size(120, 23);
		tid3.Size = size;
		this.Tid.TabIndex = 6;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label11;
		location = new System.Drawing.Point(77, 115);
		label9.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label10 = this.Label11;
		size = new System.Drawing.Size(33, 16);
		label10.Size = size;
		this.Label11.TabIndex = 11;
		this.Label11.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label13;
		location = new System.Drawing.Point(225, 162);
		label11.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label12 = this.Label13;
		size = new System.Drawing.Size(49, 16);
		label12.Size = size;
		this.Label13.TabIndex = 11;
		this.Label13.Text = "โทรสาร";
		this.Label17.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label17;
		location = new System.Drawing.Point(45, 298);
		label13.Location = location;
		this.Label17.Name = "Label17";
		System.Windows.Forms.Label label14 = this.Label17;
		size = new System.Drawing.Size(67, 16);
		label14.Size = size;
		this.Label17.TabIndex = 11;
		this.Label17.Text = "หมายเหต\u0e38 :";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label4;
		location = new System.Drawing.Point(7, 270);
		label15.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label16 = this.Label4;
		size = new System.Drawing.Size(104, 16);
		label16.Size = size;
		this.Label4.TabIndex = 11;
		this.Label4.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e47นสามาช\u0e34ก :";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label3;
		location = new System.Drawing.Point(47, 244);
		label17.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label18 = this.Label3;
		size = new System.Drawing.Size(64, 16);
		label18.Size = size;
		this.Label3.TabIndex = 11;
		this.Label3.Text = "Package :";
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label14;
		location = new System.Drawing.Point(22, 191);
		label19.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label20 = this.Label14;
		size = new System.Drawing.Size(87, 16);
		label20.Size = size;
		this.Label14.TabIndex = 11;
		this.Label14.Text = "ประเภทสมาช\u0e34ก";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label2;
		location = new System.Drawing.Point(64, 217);
		label21.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label22 = this.Label2;
		size = new System.Drawing.Size(47, 16);
		label22.Size = size;
		this.Label2.TabIndex = 11;
		this.Label2.Text = "EMail :";
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label12;
		location = new System.Drawing.Point(56, 162);
		label23.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label24 = this.Label12;
		size = new System.Drawing.Size(55, 16);
		label24.Size = size;
		this.Label12.TabIndex = 11;
		this.Label12.Text = "โทรศ\u0e31พท\u0e4c";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label10;
		location = new System.Drawing.Point(13, 88);
		label25.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label26 = this.Label10;
		size = new System.Drawing.Size(101, 16);
		label26.Size = size;
		this.Label10.TabIndex = 11;
		this.Label10.Text = "ช\u0e37\u0e48อ - สก\u0e38ล คนท\u0e35\u0e48 2";
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label6;
		location = new System.Drawing.Point(13, 60);
		label27.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label28 = this.Label6;
		size = new System.Drawing.Size(101, 16);
		label28.Size = size;
		this.Label6.TabIndex = 11;
		this.Label6.Text = "ช\u0e37\u0e48อ - สก\u0e38ล คนท\u0e35\u0e48 1";
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label9;
		location = new System.Drawing.Point(35, 31);
		label29.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label30 = this.Label9;
		size = new System.Drawing.Size(72, 16);
		label30.Size = size;
		this.Label9.TabIndex = 11;
		this.Label9.Text = "รห\u0e31สบาร\u0e4cโค\u0e49ด";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX22 = this.ButtonX_0;
		location = new System.Drawing.Point(285, 373);
		buttonX22.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX23 = this.ButtonX_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX23.Margin = margin;
		this.ButtonX_0.Name = "ยกเล\u0e34ก";
		DevComponents.DotNetBar.ButtonX buttonX24 = this.ButtonX_0;
		size = new System.Drawing.Size(87, 28);
		buttonX24.Size = size;
		this.ButtonX_0.TabIndex = 22;
		this.ButtonX_0.Text = "ยกเล\u0e34ก";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX25 = this.ButtonX_1;
		location = new System.Drawing.Point(192, 373);
		buttonX25.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX26 = this.ButtonX_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX26.Margin = margin;
		this.ButtonX_1.Name = "บ\u0e31นท\u0e36ก";
		DevComponents.DotNetBar.ButtonX buttonX27 = this.ButtonX_1;
		size = new System.Drawing.Size(87, 28);
		buttonX27.Size = size;
		this.ButtonX_1.TabIndex = 21;
		this.ButtonX_1.Text = "บ\u0e31นท\u0e36ก";
		this.SaveFileDialog1.DefaultExt = "csv";
		this.SaveFileDialog1.FileName = "Members";
		this.SaveFileDialog1.Filter = "Excel Files|*.csv";
		this.SaveFileDialog1.Title = "Export";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(847, 458);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmSearchCustomers";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "ล\u0e39กค\u0e49า";
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.PanelEx1.ResumeLayout(false);
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.ResumeLayout(false);
	}
}
