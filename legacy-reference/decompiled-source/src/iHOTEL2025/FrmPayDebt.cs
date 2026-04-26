using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmPayDebt : Office2007Form
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

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("Panel1")]
	private Panel _Panel1;

	[AccessedThroughProperty("ListView4")]
	private global::PrintableListView.PrintableListView _ListView4;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ColumnHeader17")]
	private ColumnHeader _ColumnHeader17;

	[AccessedThroughProperty("Button13")]
	private Button _Button13;

	[AccessedThroughProperty("Label31")]
	private Label _Label31;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader18")]
	private ColumnHeader _ColumnHeader18;

	[AccessedThroughProperty("ColumnHeader19")]
	private ColumnHeader _ColumnHeader19;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("ColumnHeader20")]
	private ColumnHeader _ColumnHeader20;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("ComboBoxG")]
	private ComboBox _ComboBoxG;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ListView3")]
	private ListView _ListView3;

	[AccessedThroughProperty("ColumnHeader21")]
	private ColumnHeader _ColumnHeader21;

	[AccessedThroughProperty("ColumnHeader22")]
	private ColumnHeader _ColumnHeader22;

	[AccessedThroughProperty("ColumnHeader23")]
	private ColumnHeader _ColumnHeader23;

	[AccessedThroughProperty("ColumnHeader24")]
	private ColumnHeader _ColumnHeader24;

	[AccessedThroughProperty("ColumnHeader25")]
	private ColumnHeader _ColumnHeader25;

	[AccessedThroughProperty("ColumnHeader26")]
	private ColumnHeader _ColumnHeader26;

	[AccessedThroughProperty("ColumnHeader27")]
	private ColumnHeader _ColumnHeader27;

	[AccessedThroughProperty("ColumnHeader28")]
	private ColumnHeader _ColumnHeader28;

	[AccessedThroughProperty("Button5")]
	private Button _Button5;

	[AccessedThroughProperty("ColumnHeader29")]
	private ColumnHeader _ColumnHeader29;

	private int EditID;

	private bool IS_BUSY;

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

	internal virtual global::PrintableListView.PrintableListView ListView1
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
			EventHandler value2 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
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

	internal virtual global::PrintableListView.PrintableListView ListView4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ListView4 = value;
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

	internal virtual ColumnHeader ColumnHeader16
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader16 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader17
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader17 = value;
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

	internal virtual ComboBox ComboBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox2_SelectedIndexChanged;
			if (_ComboBox2 != null)
			{
				_ComboBox2.SelectedIndexChanged -= value2;
			}
			_ComboBox2 = value;
			if (_ComboBox2 != null)
			{
				_ComboBox2.SelectedIndexChanged += value2;
			}
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

	internal virtual ColumnHeader ColumnHeader14
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader14 = value;
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

	internal virtual ColumnHeader ColumnHeader18
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader18 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader19
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader19 = value;
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

	internal virtual ColumnHeader ColumnHeader20
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader20 = value;
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

	internal virtual ComboBox ComboBoxG
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBoxG;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox2_SelectedIndexChanged;
			if (_ComboBoxG != null)
			{
				_ComboBoxG.SelectedIndexChanged -= value2;
			}
			_ComboBoxG = value;
			if (_ComboBoxG != null)
			{
				_ComboBoxG.SelectedIndexChanged += value2;
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

	internal virtual ListView ListView3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ListView3 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader21
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader21 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader22
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader22 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader23
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader23 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader24
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader24 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader25
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader25;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader25 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader26
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader26;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader26 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader27
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader27;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader27 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader28
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader28;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader28 = value;
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

	internal virtual ColumnHeader ColumnHeader29
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader29;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader29 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmPayDebt()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmPayDebt()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmPayDebt_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EditID = 0;
		IS_BUSY = false;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmPayDebt));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.Button3 = new System.Windows.Forms.Button();
		this.ListView4 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader18 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader17 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader19 = new System.Windows.Forms.ColumnHeader();
		this.Button13 = new System.Windows.Forms.Button();
		this.Label31 = new System.Windows.Forms.Label();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.ListView3 = new System.Windows.Forms.ListView();
		this.ColumnHeader21 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader22 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader23 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader24 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader25 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader26 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader27 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader28 = new System.Windows.Forms.ColumnHeader();
		this.Button5 = new System.Windows.Forms.Button();
		this.Button4 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.ComboBoxG = new System.Windows.Forms.ComboBox();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader20 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader29 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1.SuspendLayout();
		this.Panel1.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Panel1);
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(1022, 516);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.Panel1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Panel1.BackgroundImage = (System.Drawing.Image)resources.GetObject("Panel1.BackgroundImage");
		this.Panel1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch;
		this.Panel1.Controls.Add(this.Button3);
		this.Panel1.Controls.Add(this.ListView4);
		this.Panel1.Controls.Add(this.Button13);
		this.Panel1.Controls.Add(this.Label31);
		System.Windows.Forms.Panel panel = this.Panel1;
		location = new System.Drawing.Point(-2, 364);
		panel.Location = location;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel2 = this.Panel1;
		size = new System.Drawing.Size(1022, 155);
		panel2.Size = size;
		this.Panel1.TabIndex = 76;
		this.Button3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button3.Image = (System.Drawing.Image)resources.GetObject("Button3.Image");
		this.Button3.ImageAlign = System.Drawing.ContentAlignment.TopCenter;
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(898, 96);
		button.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button2 = this.Button3;
		size = new System.Drawing.Size(108, 45);
		button2.Size = size;
		this.Button3.TabIndex = 58;
		this.Button3.Text = "พ\u0e34มพ\u0e4cรายการชำระ";
		this.Button3.TextAlign = System.Drawing.ContentAlignment.BottomCenter;
		this.Button3.UseVisualStyleBackColor = true;
		this.ListView4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView4.Atto_กระดาษแนวนอน = true;
		this.ListView4.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.ListView4.Columns.AddRange(new System.Windows.Forms.ColumnHeader[9] { this.ColumnHeader11, this.ColumnHeader12, this.ColumnHeader15, this.ColumnHeader18, this.ColumnHeader14, this.ColumnHeader13, this.ColumnHeader16, this.ColumnHeader17, this.ColumnHeader19 });
		this.ListView4.FitToPage = true;
		this.ListView4.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListView4.FullRowSelect = true;
		global::PrintableListView.PrintableListView listView = this.ListView4;
		location = new System.Drawing.Point(22, 40);
		listView.Location = location;
		global::PrintableListView.PrintableListView listView2 = this.ListView4;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView4.Name = "ListView4";
		global::PrintableListView.PrintableListView listView3 = this.ListView4;
		size = new System.Drawing.Size(870, 101);
		listView3.Size = size;
		this.ListView4.TabIndex = 6;
		this.ListView4.Title = "";
		this.ListView4.Title2 = "";
		this.ListView4.Title2Tab = "";
		this.ListView4.Title3 = "";
		this.ListView4.Title3Tab = "";
		this.ListView4.UseCompatibleStateImageBehavior = false;
		this.ListView4.View = System.Windows.Forms.View.Details;
		this.ColumnHeader11.Text = "ท\u0e35\u0e48";
		this.ColumnHeader11.Width = 25;
		this.ColumnHeader12.Text = "เลขท\u0e35\u0e48ใบเสร\u0e47จร\u0e31บเง\u0e34น";
		this.ColumnHeader12.Width = 110;
		this.ColumnHeader15.Text = "รายละเอ\u0e35ยด";
		this.ColumnHeader15.Width = 200;
		this.ColumnHeader18.Text = "รวมจ\u0e48าย(รายการ)";
		this.ColumnHeader18.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader18.Width = 100;
		this.ColumnHeader14.Text = "เง\u0e34นสด";
		this.ColumnHeader14.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader14.Width = 100;
		this.ColumnHeader13.Text = "เครด\u0e34ต";
		this.ColumnHeader13.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader16.Text = "รวมเง\u0e34น";
		this.ColumnHeader16.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader16.Width = 100;
		this.ColumnHeader17.Text = "ว\u0e31นท\u0e35\u0e48ชำระ";
		this.ColumnHeader17.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader17.Width = 120;
		this.ColumnHeader19.Text = "หมายเหต\u0e38";
		this.ColumnHeader19.Width = 200;
		this.Button13.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Button13.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Button13.Image = (System.Drawing.Image)resources.GetObject("Button13.Image");
		this.Button13.ImageAlign = System.Drawing.ContentAlignment.TopCenter;
		System.Windows.Forms.Button button3 = this.Button13;
		location = new System.Drawing.Point(898, 40);
		button3.Location = location;
		System.Windows.Forms.Button button4 = this.Button13;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button4.Margin = margin;
		this.Button13.Name = "Button13";
		System.Windows.Forms.Button button5 = this.Button13;
		size = new System.Drawing.Size(108, 51);
		button5.Size = size;
		this.Button13.TabIndex = 57;
		this.Button13.Text = "ชำระเง\u0e34น";
		this.Button13.TextAlign = System.Drawing.ContentAlignment.BottomCenter;
		this.Button13.UseVisualStyleBackColor = true;
		this.Label31.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Label31.BackColor = System.Drawing.Color.FromArgb(128, 255, 128);
		this.Label31.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label31.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label = this.Label31;
		location = new System.Drawing.Point(22, 14);
		label.Location = location;
		this.Label31.Name = "Label31";
		System.Windows.Forms.Label label2 = this.Label31;
		size = new System.Drawing.Size(984, 22);
		label2.Size = size;
		this.Label31.TabIndex = 0;
		this.Label31.Text = "รายการชำระเง\u0e34น";
		this.Label31.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.ListView3);
		this.GroupBox1.Controls.Add(this.Button5);
		this.GroupBox1.Controls.Add(this.Button4);
		this.GroupBox1.Controls.Add(this.Button2);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Controls.Add(this.ComboBoxG);
		this.GroupBox1.Controls.Add(this.ComboBox2);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.ListView1);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		location = new System.Drawing.Point(12, 12);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		size = new System.Drawing.Size(998, 346);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายการล\u0e39กหน\u0e35\u0e49 รายการลงทะเบ\u0e35ยน";
		this.Label5.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label5;
		location = new System.Drawing.Point(703, 52);
		label3.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label4 = this.Label5;
		size = new System.Drawing.Size(109, 16);
		label4.Size = size;
		this.Label5.TabIndex = 22;
		this.Label5.Text = "รายละเอ\u0e35ยดห\u0e49องพ\u0e31ก";
		this.ListView3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ListView3.CheckBoxes = true;
		this.ListView3.Columns.AddRange(new System.Windows.Forms.ColumnHeader[8] { this.ColumnHeader21, this.ColumnHeader22, this.ColumnHeader23, this.ColumnHeader24, this.ColumnHeader25, this.ColumnHeader26, this.ColumnHeader27, this.ColumnHeader28 });
		this.ListView3.FullRowSelect = true;
		this.ListView3.GridLines = true;
		System.Windows.Forms.ListView listView4 = this.ListView3;
		location = new System.Drawing.Point(703, 70);
		listView4.Location = location;
		System.Windows.Forms.ListView listView5 = this.ListView3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView5.Margin = margin;
		this.ListView3.MultiSelect = false;
		this.ListView3.Name = "ListView3";
		System.Windows.Forms.ListView listView6 = this.ListView3;
		size = new System.Drawing.Size(289, 237);
		listView6.Size = size;
		this.ListView3.TabIndex = 21;
		this.ListView3.UseCompatibleStateImageBehavior = false;
		this.ListView3.View = System.Windows.Forms.View.Details;
		this.ColumnHeader21.Text = "";
		this.ColumnHeader21.Width = 0;
		this.ColumnHeader22.Text = "ห\u0e49อง";
		this.ColumnHeader22.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader23.Text = "ประเภท";
		this.ColumnHeader23.Width = 0;
		this.ColumnHeader24.Text = "จำนวน";
		this.ColumnHeader24.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader24.Width = 50;
		this.ColumnHeader25.Text = "ราคา";
		this.ColumnHeader25.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader25.Width = 70;
		this.ColumnHeader26.Text = "ราคารวม";
		this.ColumnHeader26.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader26.Width = 100;
		this.ColumnHeader27.Text = "ว\u0e31นท\u0e35\u0e48 IN";
		this.ColumnHeader27.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader27.Width = 110;
		this.ColumnHeader28.Text = "ว\u0e31นท\u0e35\u0e48 OUT";
		this.ColumnHeader28.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader28.Width = 110;
		this.Button5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Button5.Image = iHOTEL2025.My.Resources.Resources.edit;
		this.Button5.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button6 = this.Button5;
		location = new System.Drawing.Point(7, 313);
		button6.Location = location;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button7 = this.Button5;
		size = new System.Drawing.Size(133, 26);
		button7.Size = size;
		this.Button5.TabIndex = 6;
		this.Button5.Text = "   แก\u0e49ไขใบลงทะเบ\u0e35ยน";
		this.Button5.UseVisualStyleBackColor = true;
		this.Button4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button4.Image = iHOTEL2025.My.Resources.Resources.print;
		this.Button4.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button8 = this.Button4;
		location = new System.Drawing.Point(703, 314);
		button8.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button9 = this.Button4;
		size = new System.Drawing.Size(151, 26);
		button9.Size = size;
		this.Button4.TabIndex = 6;
		this.Button4.Text = "  พ\u0e34มพ\u0e4cรายการท\u0e31\u0e49งหมด";
		this.Button4.UseVisualStyleBackColor = true;
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button2.Image = iHOTEL2025.My.Resources.Resources.print;
		this.Button2.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button10 = this.Button2;
		location = new System.Drawing.Point(860, 314);
		button10.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button11 = this.Button2;
		size = new System.Drawing.Size(133, 26);
		button11.Size = size;
		this.Button2.TabIndex = 5;
		this.Button2.Text = "  พ\u0e34มพ\u0e4cใบแจ\u0e49งหน\u0e35\u0e49";
		this.Button2.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button12 = this.Button1;
		location = new System.Drawing.Point(791, 22);
		button12.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button13 = this.Button1;
		size = new System.Drawing.Size(75, 23);
		button13.Size = size;
		this.Button1.TabIndex = 4;
		this.Button1.Text = "ค\u0e49นหา";
		this.Button1.UseVisualStyleBackColor = true;
		this.ComboBoxG.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBoxG.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBoxG = this.ComboBoxG;
		location = new System.Drawing.Point(477, 21);
		comboBoxG.Location = location;
		this.ComboBoxG.Name = "ComboBoxG";
		System.Windows.Forms.ComboBox comboBoxG2 = this.ComboBoxG;
		size = new System.Drawing.Size(127, 24);
		comboBoxG2.Size = size;
		this.ComboBoxG.TabIndex = 3;
		this.ComboBox2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[3] { "ชำระครบ", "ชำระย\u0e31งไม\u0e48ครบ", "ท\u0e31\u0e49งหมด" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox2;
		location = new System.Drawing.Point(658, 21);
		comboBox.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox2;
		size = new System.Drawing.Size(127, 24);
		comboBox2.Size = size;
		this.ComboBox2.TabIndex = 3;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label4;
		location = new System.Drawing.Point(442, 25);
		label5.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label6 = this.Label4;
		size = new System.Drawing.Size(32, 16);
		label6.Size = size;
		this.Label4.TabIndex = 1;
		this.Label4.Text = "กล\u0e38\u0e48ม";
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox1;
		location = new System.Drawing.Point(192, 21);
		comboBox3.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox1;
		size = new System.Drawing.Size(242, 24);
		comboBox4.Size = size;
		this.ComboBox1.TabIndex = 2;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label3;
		location = new System.Drawing.Point(613, 25);
		label7.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label8 = this.Label3;
		size = new System.Drawing.Size(44, 16);
		label8.Size = size;
		this.Label3.TabIndex = 1;
		this.Label3.Text = "สถานะ";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label2;
		location = new System.Drawing.Point(81, 25);
		label9.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label10 = this.Label2;
		size = new System.Drawing.Size(110, 16);
		label10.Size = size;
		this.Label2.TabIndex = 1;
		this.Label2.Text = "ช\u0e37\u0e48อ/เบอร\u0e4cโทร/เลขท\u0e35\u0e48";
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label11 = this.Label1;
		location = new System.Drawing.Point(11, 25);
		label11.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label12 = this.Label1;
		size = new System.Drawing.Size(67, 16);
		label12.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "ค\u0e49นหาตาม";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[12]
		{
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader7, this.ColumnHeader8, this.ColumnHeader9, this.ColumnHeader10,
			this.ColumnHeader20, this.ColumnHeader29
		});
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView7 = this.ListView1;
		location = new System.Drawing.Point(7, 51);
		listView7.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView8 = this.ListView1;
		size = new System.Drawing.Size(690, 256);
		listView8.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "";
		this.ColumnHeader1.Width = 0;
		this.ColumnHeader2.Text = "ท\u0e35\u0e48";
		this.ColumnHeader3.Text = "เลขท\u0e35\u0e48";
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader4.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader5.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.ColumnHeader5.Width = 140;
		this.ColumnHeader6.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.ColumnHeader6.Width = 200;
		this.ColumnHeader7.Text = "โทร";
		this.ColumnHeader7.Width = 100;
		this.ColumnHeader8.Text = "ราคารวม";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Width = 100;
		this.ColumnHeader9.Text = "ชำระแล\u0e49ว";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 100;
		this.ColumnHeader10.Text = "คงเหล\u0e37อ";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 100;
		this.ColumnHeader20.Text = "หมายเหต\u0e38";
		this.ColumnHeader20.Width = 200;
		this.ColumnHeader29.Text = "หมายเหต\u0e382";
		this.ColumnHeader29.Width = 200;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1022, 516);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmPayDebt";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ชำระเง\u0e34น/ล\u0e39กหน\u0e35\u0e49 รายการลงทะเบ\u0e35ยน";
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.PanelEx1.ResumeLayout(false);
		this.Panel1.ResumeLayout(false);
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmPayDebt_Load(object sender, EventArgs e)
	{
		IS_BUSY = true;
		ComboBox2.SelectedIndex = 1;
		ListCust();
		ListG();
		IS_BUSY = false;
		SearchDebt();
	}

	public void ListG()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType_Main order by name");
		ComboBoxG.Items.Clear();
		ComboBoxG.Items.Add("ท\u0e31\u0e49งหมด");
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
				ComboBoxG.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			ComboBoxG.SelectedIndex = 0;
		}
	}

	public void ListCust()
	{
		object left = "select cust_name from View_CheckIn_H where Cin_status='ปกต\u0e34'";
		if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and cust_name like '%" + ComboBox1.Text, "%'"));
		}
		if (Operators.CompareString(ComboBox2.Text, "ชำระย\u0e31งไม\u0e48ครบ", TextCompare: false) == 0)
		{
			left = Operators.ConcatenateObject(left, " and total_price_balance > 0");
		}
		else if (Operators.CompareString(ComboBox2.Text, "ชำระครบ", TextCompare: false) == 0)
		{
			left = Operators.ConcatenateObject(left, " and total_price_balance <= 0");
		}
		if ((Operators.CompareString(ComboBoxG.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0) & (Operators.CompareString(ComboBoxG.Text, "", TextCompare: false) != 0))
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and cust_type_main = '" + ComboBoxG.Text, "'"));
		}
		left = Operators.ConcatenateObject(left, " group by Cust_name order by Cust_name");
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		ComboBox1.Items.Clear();
		ComboBox1.Items.Add("");
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
					ComboBox1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cust_name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void SearchDebt()
	{
		if (IS_BUSY)
		{
			return;
		}
		object left = "select * from View_CheckIn_H where Cin_status='ปกต\u0e34'";
		if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(string.Concat(string.Concat(" and (cust_name like '%" + ComboBox1.Text, "%' or Cin_no like '%"), ComboBox1.Text), "%' or cust_add_tel like '%"), ComboBox1.Text), "%')"));
		}
		if (Operators.CompareString(ComboBox2.Text, "ชำระย\u0e31งไม\u0e48ครบ", TextCompare: false) == 0)
		{
			left = Operators.ConcatenateObject(left, " and total_price_balance > 0");
		}
		else if (Operators.CompareString(ComboBox2.Text, "ชำระครบ", TextCompare: false) == 0)
		{
			left = Operators.ConcatenateObject(left, " and total_price_balance <= 0");
		}
		if (Operators.CompareString(ComboBoxG.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and cust_type_main = '" + ComboBoxG.Text, "'"));
		}
		left = Operators.ConcatenateObject(left, " order by Cin_no");
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		ListView1.Items.Clear();
		ListView3.Items.Clear();
		ListView4.Items.Clear();
		DataSet dataSet2 = Module1.connect("select * from HT_Invoice_Note");
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
				global::PrintableListView.PrintableListView listView = ListView1;
				ListView.ListViewItemCollection items = listView.Items;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow2 = dataRow;
				string columnName = "Cin_no";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				listView.Items[num2].SubItems.Add(Conversions.ToString(num2 + 1));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[num2].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow3 = dataRow;
				columnName = "Cin_no";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cin_date"]), "dd/MM/yyyy"));
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[num2].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow4 = dataRow;
				columnName = "cust_name";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[num2].SubItems.Add(Module1.Address_Rcplace(Conversions.ToString(dataSet.Tables[0].Rows[num2]["C_address"])));
				ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[num2].SubItems;
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
				listView.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["total_price_net"]), "#,##0.00"));
				listView.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["total_price_pay"]), "#,##0.00"));
				listView.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["total_price_balance"]), "#,##0.00"));
				if (Operators.ConditionalCompareObjectLessEqual(dataSet.Tables[0].Rows[num2]["total_price_balance"], 0, TextCompare: false))
				{
					listView.Items[num2].BackColor = Color.LightGreen;
				}
				listView.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cin_note"].ToString());
				bool flag = false;
				int num5 = dataSet2.Tables[0].Rows.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					num4 = num5;
					if (num7 <= num4)
					{
						if (!Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num6]["Cin_no"], dataSet.Tables[0].Rows[num2]["Cin_no"], TextCompare: false))
						{
							num6++;
							continue;
						}
						listView.Items[num2].SubItems.Add(dataSet2.Tables[0].Rows[num6]["note"].ToString());
						flag = true;
						break;
					}
					break;
				}
				if (!flag)
				{
					listView.Items[num2].SubItems.Add("");
				}
				listView = null;
				num2++;
			}
		}
	}

	private void ComboBox2_SelectedIndexChanged(object sender, EventArgs e)
	{
		SearchDebt();
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		SearchDebt();
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		DataSet dataSet = Module1.connect("select * from View_Pay_Ds where Cin_No='" + ListView1.SelectedItems[0].SubItems[2].Text + "' order by id");
		DataSet dataSet2 = Module1.connect("select * from HT_Log_Debt where log_ds = 'ต\u0e31ดจากใบลงทะเบ\u0e35ยน " + ListView1.SelectedItems[0].SubItems[2].Text + "' order by log_date");
		ListView4.Items.Clear();
		ListView3.Items.Clear();
		string right = "";
		int num = 1;
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			right = "";
		}
		string left = "YELLOW";
		checked
		{
			int num2 = dataSet.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView = ListView4;
				int count = listView.Items.Count;
				object[] array3;
				DataRow dataRow;
				string columnName;
				object[] array;
				bool[] array4;
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num3]["pay_no"], right, TextCompare: false))
				{
					listView.Items.Add(Conversions.ToString(num));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					array = new object[1];
					object[] array2 = array;
					dataRow = dataSet.Tables[0].Rows[num3];
					DataRow dataRow2 = dataRow;
					columnName = "pay_no";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					array3 = array;
					object[] arguments = array3;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					num++;
					left = ((Operators.CompareString(left, "YELLOW", TextCompare: false) != 0) ? "YELLOW" : "");
				}
				else
				{
					listView.Items.Add("");
					listView.Items[count].SubItems.Add("");
				}
				if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(dataSet.Tables[0].Rows[num3]["Cin_Pay_Ds_name"], "ค\u0e48าห\u0e49อง", TextCompare: false), Operators.CompareObjectEqual(dataSet.Tables[0].Rows[num3]["Cin_Pay_Ds_name"], "เต\u0e35ยงเสร\u0e34ม", TextCompare: false))))
				{
					NewLateBinding.LateCall(listView.Items[count].SubItems, null, "Add", new object[1] { Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num3]["Cin_Pay_Ds_name"], " ["), dataSet.Tables[0].Rows[num3]["Cin_Pay_Ds"]), "]") }, null, null, null, IgnoreReturn: true);
				}
				else
				{
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num3];
					DataRow dataRow3 = dataRow;
					columnName = "Cin_Pay_Ds_name";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
				}
				ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow4 = dataRow;
				columnName = "Cin_Pay_Ds_Price";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num3]["pay_no"], right, TextCompare: false))
				{
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Pay_cash"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Pay_credit"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(Operators.AddObject(dataSet.Tables[0].Rows[num3]["Cin_Pay_cash"], dataSet.Tables[0].Rows[num3]["Cin_Pay_credit"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Pay_Date"]), "dd/MM/yy HH:mm"));
				}
				else
				{
					listView.Items[count].SubItems.Add("");
					listView.Items[count].SubItems.Add("");
					listView.Items[count].SubItems.Add("");
					listView.Items[count].SubItems.Add("");
				}
				if (Operators.CompareString(left, "", TextCompare: false) != 0)
				{
					listView.Items[count].BackColor = Color.Yellow;
				}
				listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Cin_Pay_Note"].ToString());
				listView = null;
				right = Conversions.ToString(dataSet.Tables[0].Rows[num3]["pay_no"]);
				num3++;
			}
			int num6 = dataSet2.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 > num5)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView2 = ListView4;
				int count2 = listView2.Items.Count;
				listView2.Items.Add("");
				listView2.Items[count2].SubItems.Add("########");
				listView2.Items[count2].SubItems.Add("ยอดจ\u0e48ายล\u0e48วงหน\u0e49า");
				ListViewItem.ListViewSubItemCollection subItems4 = listView2.Items[count2].SubItems;
				object[] array7 = new object[1];
				object[] array8 = array7;
				Type typeFromHandle = typeof(Math);
				object[] array3 = new object[1];
				object[] array9 = array3;
				DataRow dataRow = dataSet2.Tables[0].Rows[num7];
				DataRow dataRow5 = dataRow;
				string columnName = "log_price";
				array9[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
				object[] array = array3;
				object[] arguments4 = array;
				bool[] array4 = new bool[1] { true };
				object obj = NewLateBinding.LateGet(null, typeFromHandle, "Abs", arguments4, null, null, array4);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				array8[0] = RuntimeHelpers.GetObjectValue(obj);
				NewLateBinding.LateCall(subItems4, null, "Add", array7, null, null, null, IgnoreReturn: true);
				ListViewItem.ListViewSubItemCollection subItems5 = listView2.Items[count2].SubItems;
				Type typeFromHandle2 = typeof(Math);
				array7 = new object[1];
				object[] array10 = array7;
				dataRow = dataSet2.Tables[0].Rows[num7];
				DataRow dataRow6 = dataRow;
				columnName = "log_price";
				array10[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
				array3 = array7;
				object[] arguments5 = array3;
				array4 = new bool[1] { true };
				object obj2 = NewLateBinding.LateGet(null, typeFromHandle2, "Abs", arguments5, null, null, array4);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				subItems5.Add(Strings.Format(RuntimeHelpers.GetObjectValue(obj2), "#,##0.00"));
				listView2.Items[count2].SubItems.Add(Strings.Format(0, "#,##0.00"));
				ListViewItem.ListViewSubItemCollection subItems6 = listView2.Items[count2].SubItems;
				Type typeFromHandle3 = typeof(Math);
				array7 = new object[1];
				object[] array11 = array7;
				dataRow = dataSet2.Tables[0].Rows[num7];
				DataRow dataRow7 = dataRow;
				columnName = "log_price";
				array11[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
				array3 = array7;
				object[] arguments6 = array3;
				array4 = new bool[1] { true };
				object obj3 = NewLateBinding.LateGet(null, typeFromHandle3, "Abs", arguments6, null, null, array4);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				subItems6.Add(Strings.Format(RuntimeHelpers.GetObjectValue(obj3), "#,##0.00"));
				listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["log_Date"]), "dd/MM/yy HH:mm"));
				if (Operators.CompareString(left, "", TextCompare: false) != 0)
				{
					listView2.Items[count2].BackColor = Color.Yellow;
				}
				listView2.Items[count2].SubItems.Add("");
				listView2 = null;
				num7++;
			}
			DataSet dataSet3 = Module1.connect("select * from HT_CheckIn_Ds where Cin_No='" + ListView1.SelectedItems[0].SubItems[2].Text + "' order by id");
			int num9 = dataSet3.Tables[0].Rows.Count - 1;
			int num10 = 0;
			while (true)
			{
				int num11 = num10;
				int num5 = num9;
				if (num11 > num5)
				{
					break;
				}
				ListView listView3 = ListView3;
				int count3 = listView3.Items.Count;
				ListView.ListViewItemCollection items = listView3.Items;
				object[] array7 = new object[1];
				object[] array12 = array7;
				DataRow dataRow = dataSet3.Tables[0].Rows[num10];
				DataRow dataRow8 = dataRow;
				string columnName = "id";
				array12[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
				object[] array3 = array7;
				object[] arguments7 = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(items, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems7 = listView3.Items[count3].SubItems;
				array7 = new object[1];
				object[] array13 = array7;
				dataRow = dataSet3.Tables[0].Rows[num10];
				DataRow dataRow9 = dataRow;
				columnName = "Cin_Room_No";
				array13[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
				array3 = array7;
				object[] arguments8 = array3;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems7, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems8 = listView3.Items[count3].SubItems;
				array7 = new object[1];
				object[] array14 = array7;
				dataRow = dataSet3.Tables[0].Rows[num10];
				DataRow dataRow10 = dataRow;
				columnName = "Cin_Room_Type";
				array14[0] = RuntimeHelpers.GetObjectValue(dataRow10[columnName]);
				array3 = array7;
				object[] arguments9 = array3;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems8, null, "Add", arguments9, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems9 = listView3.Items[count3].SubItems;
				array7 = new object[1];
				object[] array15 = array7;
				dataRow = dataSet3.Tables[0].Rows[num10];
				DataRow dataRow11 = dataRow;
				columnName = "Cin_Room_Night";
				array15[0] = RuntimeHelpers.GetObjectValue(dataRow11[columnName]);
				array3 = array7;
				object[] arguments10 = array3;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems9, null, "Add", arguments10, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				listView3.Items[count3].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_Room_Price"]), "#,##0.00"));
				listView3.Items[count3].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_Room_PriceToTal"]), "#,##0.00"));
				listView3.Items[count3].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_Room_in"]), "dd-MM-yy HH:mm"));
				if (Operators.ConditionalCompareObjectNotEqual(dataSet3.Tables[0].Rows[num10]["Cin_Room_status"], "เข\u0e49าพ\u0e31ก", TextCompare: false))
				{
					listView3.Items[count3].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_Room_out"]), "dd-MM-yy HH:mm"));
				}
				else
				{
					listView3.Items[count3].SubItems.Add("");
				}
				listView3 = null;
				num10++;
			}
			EditID = ListView1.SelectedItems[0].Index;
			Button13.Enabled = true;
		}
	}

	private void Button13_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			if (ListView1.SelectedItems[0].BackColor == Color.LightGreen)
			{
				MessageBox.Show("รายการเลขท\u0e35\u0e48 " + ListView1.SelectedItems[0].SubItems[2].Text + " ได\u0e49ชำระครบไปแล\u0e49ว");
				return;
			}
			FrmPayAddDebt frmPayAddDebt = new FrmPayAddDebt();
			frmPayAddDebt.EDIT_ID = ListView1.SelectedItems[0].SubItems[2].Text;
			frmPayAddDebt.ShowDialog();
			SearchDebt();
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		SearchDebt();
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			MyProject.Forms.INV_Note.R_NO = ListView1.SelectedItems[0].SubItems[2].Text;
			MyProject.Forms.INV_Note.ShowDialog();
			if (MyProject.Forms.INV_Note.isok)
			{
				Print_Report.Print_Reg3(ListView1.SelectedItems[0].SubItems[2].Text, preview: true);
			}
		}
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		ListView1.Atto_กระดาษแนวนอน = false;
		ListView1.Title = "รายการค\u0e49างชำระ (สถานะ " + ComboBox2.Text + " )";
		ListView1.PrintPreview();
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการใบลงทะเบ\u0e35ยน");
			return;
		}
		if (ListView4.Items.Count == 0)
		{
			MessageBox.Show("ไม\u0e48ม\u0e35รายการชำระเง\u0e34น");
			return;
		}
		ListView4.Atto_กระดาษแนวนอน = false;
		ListView4.Title = "รายการชำระเง\u0e34น ของ " + ListView1.SelectedItems[0].SubItems[4].Text;
		ListView4.Title2 = "เลขท\u0e35\u0e48ใบลงทะเบ\u0e35ยน " + ListView1.SelectedItems[0].SubItems[2].Text;
		ListView4.PrintPreview();
	}

	private void Button5_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการใบลงทะเบ\u0e35ยน");
			return;
		}
		if (ListView1.SelectedItems[0].BackColor == Color.LightGreen)
		{
			MessageBox.Show("รายการเลขท\u0e35\u0e48 " + ListView1.SelectedItems[0].SubItems[2].Text + " ได\u0e49ชำระครบไปแล\u0e49ว");
			return;
		}
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_Ds where Cin_no='" + ListView1.SelectedItems[0].SubItems[2].Text + "' and Cin_Room_Status <>'Check-Out'");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			MessageBox.Show("รายการเลขท\u0e35\u0e48 " + ListView1.SelectedItems[0].SubItems[2].Text + " ย\u0e31งเช\u0e47คเอาท\u0e4cไม\u0e48หมดไม\u0e48สามารถแก\u0e49ไขได\u0e49", "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			return;
		}
		FrmCheckIn_EditOnly frmCheckIn_EditOnly = new FrmCheckIn_EditOnly();
		frmCheckIn_EditOnly.EDIT_ID = ListView1.SelectedItems[0].SubItems[2].Text;
		frmCheckIn_EditOnly.ShowDialog();
		Button1_Click(null, null);
	}
}
