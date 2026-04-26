using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmDueBill : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Button1")]
	private ButtonX _Button1;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("GroupBox2")]
	private GroupBox _GroupBox2;

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("Labeltotal")]
	private Label _Labeltotal;

	[AccessedThroughProperty("Labelcredit")]
	private Label _Labelcredit;

	[AccessedThroughProperty("Label15")]
	private Label _Label15;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label16")]
	private Label _Label16;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("Button5")]
	private ButtonX _Button5;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("Label101")]
	private Label _Label101;

	[AccessedThroughProperty("Label20")]
	private Label _Label20;

	[AccessedThroughProperty("Label100")]
	private Label _Label100;

	[AccessedThroughProperty("Label18")]
	private Label _Label18;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("Labeltran")]
	private Label _Labeltran;

	[AccessedThroughProperty("Label19")]
	private Label _Label19;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

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
			EventHandler value2 = GroupBox1_Enter;
			if (_GroupBox1 != null)
			{
				_GroupBox1.Enter -= value2;
			}
			_GroupBox1 = value;
			if (_GroupBox1 != null)
			{
				_GroupBox1.Enter += value2;
			}
		}
	}

	internal virtual ButtonX Button1
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
			_ListView1 = value;
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

	internal virtual Label Labeltotal
	{
		[DebuggerNonUserCode]
		get
		{
			return _Labeltotal;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Labeltotal = value;
		}
	}

	internal virtual Label Labelcredit
	{
		[DebuggerNonUserCode]
		get
		{
			return _Labelcredit;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Labelcredit = value;
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

	internal virtual DateTimePicker DateTimePicker2
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DateTimePicker2 = value;
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

	internal virtual ButtonX Button5
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
			EventHandler value2 = Button1_Click;
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

	internal virtual Label Label101
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label101;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label101 = value;
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

	internal virtual Label Labeltran
	{
		[DebuggerNonUserCode]
		get
		{
			return _Labeltran;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Labeltran = value;
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

	[DebuggerNonUserCode]
	static FrmDueBill()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmDueBill()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmDueBill_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		InitializeComponent();
	}

	[DebuggerNonUserCode]
	protected override void Dispose(bool disposing)
	{
		try
		{
			if (disposing && components != null)
			{
				components.Dispose();
			}
		}
		finally
		{
			base.Dispose(disposing);
		}
	}

	[System.Diagnostics.DebuggerStepThrough]
	private void InitializeComponent()
	{
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmDueBill));
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Labeltran = new System.Windows.Forms.Label();
		this.Label19 = new System.Windows.Forms.Label();
		this.Label101 = new System.Windows.Forms.Label();
		this.Label20 = new System.Windows.Forms.Label();
		this.Label100 = new System.Windows.Forms.Label();
		this.Label18 = new System.Windows.Forms.Label();
		this.Labeltotal = new System.Windows.Forms.Label();
		this.Labelcredit = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label15 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label9 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.Button4 = new System.Windows.Forms.Button();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label14 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label16 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button3 = new System.Windows.Forms.Button();
		this.Button5 = new DevComponents.DotNetBar.ButtonX();
		this.Button1 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.GroupBox1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.Labeltran);
		this.GroupBox1.Controls.Add(this.Label19);
		this.GroupBox1.Controls.Add(this.Label101);
		this.GroupBox1.Controls.Add(this.Label20);
		this.GroupBox1.Controls.Add(this.Label100);
		this.GroupBox1.Controls.Add(this.Label18);
		this.GroupBox1.Controls.Add(this.Button5);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Controls.Add(this.Labeltotal);
		this.GroupBox1.Controls.Add(this.Labelcredit);
		this.GroupBox1.Controls.Add(this.Label12);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.Label15);
		this.GroupBox1.Controls.Add(this.Label10);
		this.GroupBox1.Controls.Add(this.Label13);
		this.GroupBox1.Controls.Add(this.Label8);
		this.GroupBox1.Controls.Add(this.Label11);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label6);
		this.GroupBox1.Controls.Add(this.Label9);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.Label1);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(12, 9);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1100, 170);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รอบบ\u0e34ลป\u0e31จจ\u0e38บ\u0e31น";
		this.Labeltran.BackColor = System.Drawing.Color.Black;
		this.Labeltran.ForeColor = System.Drawing.Color.White;
		System.Windows.Forms.Label labeltran = this.Labeltran;
		location = new System.Drawing.Point(619, 36);
		labeltran.Location = location;
		this.Labeltran.Name = "Labeltran";
		System.Windows.Forms.Label labeltran2 = this.Labeltran;
		size = new System.Drawing.Size(107, 22);
		labeltran2.Size = size;
		this.Labeltran.TabIndex = 7;
		this.Labeltran.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Labeltran.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label19.AutoSize = true;
		System.Windows.Forms.Label label = this.Label19;
		location = new System.Drawing.Point(525, 39);
		label.Location = location;
		this.Label19.Name = "Label19";
		System.Windows.Forms.Label label2 = this.Label19;
		size = new System.Drawing.Size(92, 16);
		label2.Size = size;
		this.Label19.TabIndex = 6;
		this.Label19.Text = "จำนวนเง\u0e34นโอน :";
		this.Label101.BackColor = System.Drawing.Color.Black;
		this.Label101.ForeColor = System.Drawing.Color.FromArgb(255, 192, 192);
		System.Windows.Forms.Label label3 = this.Label101;
		location = new System.Drawing.Point(363, 111);
		label3.Location = location;
		this.Label101.Name = "Label101";
		System.Windows.Forms.Label label4 = this.Label101;
		size = new System.Drawing.Size(107, 22);
		label4.Size = size;
		this.Label101.TabIndex = 5;
		this.Label101.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label101.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label20.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label20;
		location = new System.Drawing.Point(276, 114);
		label5.Location = location;
		this.Label20.Name = "Label20";
		System.Windows.Forms.Label label6 = this.Label20;
		size = new System.Drawing.Size(85, 16);
		label6.Size = size;
		this.Label20.TabIndex = 4;
		this.Label20.Text = "จ\u0e48ายเง\u0e34นม\u0e31ดจำ :";
		this.Label100.BackColor = System.Drawing.Color.Black;
		this.Label100.ForeColor = System.Drawing.Color.FromArgb(255, 192, 192);
		System.Windows.Forms.Label label7 = this.Label100;
		location = new System.Drawing.Point(363, 87);
		label7.Location = location;
		this.Label100.Name = "Label100";
		System.Windows.Forms.Label label8 = this.Label100;
		size = new System.Drawing.Size(107, 22);
		label8.Size = size;
		this.Label100.TabIndex = 3;
		this.Label100.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label100.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label18.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label18;
		location = new System.Drawing.Point(283, 90);
		label9.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label10 = this.Label18;
		size = new System.Drawing.Size(78, 16);
		label10.Size = size;
		this.Label18.TabIndex = 2;
		this.Label18.Text = "ร\u0e31บเง\u0e34นม\u0e31ดจำ :";
		this.Labeltotal.BackColor = System.Drawing.Color.Black;
		this.Labeltotal.ForeColor = System.Drawing.Color.Cyan;
		System.Windows.Forms.Label labeltotal = this.Labeltotal;
		location = new System.Drawing.Point(619, 60);
		labeltotal.Location = location;
		this.Labeltotal.Name = "Labeltotal";
		System.Windows.Forms.Label labeltotal2 = this.Labeltotal;
		size = new System.Drawing.Size(107, 22);
		labeltotal2.Size = size;
		this.Labeltotal.TabIndex = 0;
		this.Labeltotal.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Labeltotal.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Labelcredit.BackColor = System.Drawing.Color.Black;
		this.Labelcredit.ForeColor = System.Drawing.Color.White;
		System.Windows.Forms.Label labelcredit = this.Labelcredit;
		location = new System.Drawing.Point(619, 12);
		labelcredit.Location = location;
		this.Labelcredit.Name = "Labelcredit";
		System.Windows.Forms.Label labelcredit2 = this.Labelcredit;
		size = new System.Drawing.Size(107, 22);
		labelcredit2.Size = size;
		this.Labelcredit.TabIndex = 0;
		this.Labelcredit.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Labelcredit.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label12.BackColor = System.Drawing.Color.Black;
		this.Label12.ForeColor = System.Drawing.Color.Fuchsia;
		System.Windows.Forms.Label label11 = this.Label12;
		location = new System.Drawing.Point(363, 138);
		label11.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label12 = this.Label12;
		size = new System.Drawing.Size(107, 22);
		label12.Size = size;
		this.Label12.TabIndex = 0;
		this.Label12.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label12.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label4.BackColor = System.Drawing.Color.Black;
		this.Label4.ForeColor = System.Drawing.Color.FromArgb(255, 192, 192);
		System.Windows.Forms.Label label13 = this.Label4;
		location = new System.Drawing.Point(363, 63);
		label13.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label14 = this.Label4;
		size = new System.Drawing.Size(107, 22);
		label14.Size = size;
		this.Label4.TabIndex = 0;
		this.Label4.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label4.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label15.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label15;
		location = new System.Drawing.Point(488, 63);
		label15.Location = location;
		this.Label15.Name = "Label15";
		System.Windows.Forms.Label label16 = this.Label15;
		size = new System.Drawing.Size(129, 16);
		label16.Size = size;
		this.Label15.TabIndex = 0;
		this.Label15.Text = "จำนวนเง\u0e34นรวมท\u0e31\u0e49งหมด :";
		this.Label10.BackColor = System.Drawing.Color.Black;
		this.Label10.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label label17 = this.Label10;
		location = new System.Drawing.Point(363, 39);
		label17.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label18 = this.Label10;
		size = new System.Drawing.Size(107, 22);
		label18.Size = size;
		this.Label10.TabIndex = 0;
		this.Label10.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label10.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label13;
		location = new System.Drawing.Point(491, 15);
		label19.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label20 = this.Label13;
		size = new System.Drawing.Size(126, 16);
		label20.Size = size;
		this.Label13.TabIndex = 0;
		this.Label13.Text = "จำนวนเง\u0e34นบ\u0e31ตรเครด\u0e34ต :";
		this.Label8.BackColor = System.Drawing.Color.Black;
		this.Label8.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label label21 = this.Label8;
		location = new System.Drawing.Point(363, 15);
		label21.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label22 = this.Label8;
		size = new System.Drawing.Size(107, 22);
		label22.Size = size;
		this.Label8.TabIndex = 0;
		this.Label8.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label8.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label11;
		location = new System.Drawing.Point(255, 141);
		label23.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label24 = this.Label11;
		size = new System.Drawing.Size(106, 16);
		label24.Size = size;
		this.Label11.TabIndex = 0;
		this.Label11.Text = "จำนวนเง\u0e34นสดรวม :";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label3;
		location = new System.Drawing.Point(261, 66);
		label25.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label26 = this.Label3;
		size = new System.Drawing.Size(100, 16);
		label26.Size = size;
		this.Label3.TabIndex = 0;
		this.Label3.Text = "จำนวนเง\u0e34นท\u0e35\u0e48จ\u0e48าย :";
		this.Label6.AutoSize = true;
		this.Label6.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label27 = this.Label6;
		location = new System.Drawing.Point(83, 27);
		label27.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label28 = this.Label6;
		size = new System.Drawing.Size(60, 16);
		label28.Size = size;
		this.Label6.TabIndex = 0;
		this.Label6.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label9;
		location = new System.Drawing.Point(275, 42);
		label29.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label30 = this.Label9;
		size = new System.Drawing.Size(86, 16);
		label30.Size = size;
		this.Label9.TabIndex = 0;
		this.Label9.Text = "จำนวนเง\u0e34นสด :";
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label31 = this.Label2;
		location = new System.Drawing.Point(83, 50);
		label31.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label32 = this.Label2;
		size = new System.Drawing.Size(60, 16);
		label32.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label7;
		location = new System.Drawing.Point(245, 18);
		label33.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label34 = this.Label7;
		size = new System.Drawing.Size(116, 16);
		label34.Size = size;
		this.Label7.TabIndex = 0;
		this.Label7.Text = "จำนวนเง\u0e34นในล\u0e34\u0e49นช\u0e31ก :";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label5;
		location = new System.Drawing.Point(10, 27);
		label35.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label36 = this.Label5;
		size = new System.Drawing.Size(67, 16);
		label36.Size = size;
		this.Label5.TabIndex = 0;
		this.Label5.Text = "หมายเลข :";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label1;
		location = new System.Drawing.Point(17, 50);
		label37.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label38 = this.Label1;
		size = new System.Drawing.Size(60, 16);
		label38.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.Controls.Add(this.Button4);
		this.GroupBox2.Controls.Add(this.DateTimePicker2);
		this.GroupBox2.Controls.Add(this.Label14);
		this.GroupBox2.Controls.Add(this.DateTimePicker1);
		this.GroupBox2.Controls.Add(this.Label16);
		this.GroupBox2.Controls.Add(this.ListView1);
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox2;
		location = new System.Drawing.Point(12, 185);
		groupBox3.Location = location;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox2;
		size = new System.Drawing.Size(1100, 433);
		groupBox4.Size = size;
		this.GroupBox2.TabIndex = 1;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "รอบบ\u0e34ล";
		System.Windows.Forms.Button button = this.Button4;
		location = new System.Drawing.Point(530, 18);
		button.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button2 = this.Button4;
		size = new System.Drawing.Size(84, 25);
		button2.Size = size;
		this.Button4.TabIndex = 7;
		this.Button4.Text = "ค\u0e49นหา";
		this.Button4.UseVisualStyleBackColor = true;
		this.DateTimePicker2.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker2.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(338, 19);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(186, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 9;
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label14;
		location = new System.Drawing.Point(279, 22);
		label39.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label40 = this.Label14;
		size = new System.Drawing.Size(54, 16);
		label40.Size = size;
		this.Label14.TabIndex = 8;
		this.Label14.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		this.DateTimePicker1.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker1.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(97, 19);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(176, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 10;
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label16;
		location = new System.Drawing.Point(13, 22);
		label41.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label42 = this.Label16;
		size = new System.Drawing.Size(81, 16);
		label42.Size = size;
		this.Label16.TabIndex = 6;
		this.Label16.Text = "จากว\u0e31นท\u0e35\u0e48เป\u0e34ด :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[13]
		{
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader8, this.ColumnHeader11, this.ColumnHeader12, this.ColumnHeader7, this.ColumnHeader4,
			this.ColumnHeader13, this.ColumnHeader9, this.ColumnHeader10
		});
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(10, 47);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(1081, 377);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "หมายเลข";
		this.ColumnHeader1.Width = 70;
		this.ColumnHeader2.Text = "ว\u0e31นท\u0e35\u0e48เป\u0e34ด";
		this.ColumnHeader2.Width = 140;
		this.ColumnHeader3.Text = "ว\u0e31นท\u0e35\u0e48ป\u0e34ด";
		this.ColumnHeader3.Width = 140;
		this.ColumnHeader5.Text = "เง\u0e34นในล\u0e34\u0e49นช\u0e31ก";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 100;
		this.ColumnHeader6.Text = "เง\u0e34นสด";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader6.Width = 100;
		this.ColumnHeader8.Text = "ยอดจ\u0e48าย";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Width = 100;
		this.ColumnHeader11.Text = "ร\u0e31บเง\u0e34นม\u0e31ดจำ";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 100;
		this.ColumnHeader12.Text = "จ\u0e48ายเง\u0e34นม\u0e31ดจำ";
		this.ColumnHeader12.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader12.Width = 100;
		this.ColumnHeader7.Text = "รวมเง\u0e34นสดคงเหล\u0e37อ";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 115;
		this.ColumnHeader4.Text = "บ\u0e31ตรเครด\u0e34ต";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader13.Text = "โอนเง\u0e34น";
		this.ColumnHeader13.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader9.Text = "รวมท\u0e31\u0e49งหมด";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 100;
		this.ColumnHeader10.Text = "ผ\u0e39\u0e49ป\u0e34ดรอบ";
		this.ColumnHeader10.Width = 100;
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(1029, 628);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(75, 23);
		button4.Size = size;
		this.Button2.TabIndex = 2;
		this.Button2.Text = "ป\u0e34ด";
		this.Button2.UseVisualStyleBackColor = true;
		this.Button3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.Button button5 = this.Button3;
		location = new System.Drawing.Point(12, 624);
		button5.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button6 = this.Button3;
		size = new System.Drawing.Size(75, 23);
		button6.Size = size;
		this.Button3.TabIndex = 2;
		this.Button3.Text = "พ\u0e34มพ\u0e4c";
		this.Button3.UseVisualStyleBackColor = true;
		this.Button5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Button5.Checked = true;
		this.Button5.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.Button5.FocusCuesEnabled = false;
		this.Button5.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Button5.ForeColor = System.Drawing.Color.Red;
		this.Button5.Image = (System.Drawing.Image)resources.GetObject("Button5.Image");
		DevComponents.DotNetBar.ButtonX button7 = this.Button5;
		location = new System.Drawing.Point(939, 52);
		button7.Location = location;
		this.Button5.Name = "Button5";
		DevComponents.DotNetBar.ButtonX button8 = this.Button5;
		size = new System.Drawing.Size(144, 57);
		button8.Size = size;
		this.Button5.TabIndex = 1;
		this.Button5.Text = "ป\u0e34ดรอบบ\u0e34ล";
		this.Button1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Button1.Checked = true;
		this.Button1.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.Button1.FocusCuesEnabled = false;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Button1.ForeColor = System.Drawing.Color.Blue;
		this.Button1.Image = (System.Drawing.Image)resources.GetObject("Button1.Image");
		DevComponents.DotNetBar.ButtonX button9 = this.Button1;
		location = new System.Drawing.Point(774, 52);
		button9.Location = location;
		this.Button1.Name = "Button1";
		DevComponents.DotNetBar.ButtonX button10 = this.Button1;
		size = new System.Drawing.Size(144, 57);
		button10.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.Text = "เป\u0e34ดรอบบ\u0e34ล";
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Controls.Add(this.Button3);
		this.PanelEx1.Controls.Add(this.GroupBox2);
		this.PanelEx1.Controls.Add(this.Button2);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(1124, 659);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 3;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1124, 659);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FrmDueBill";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "จ\u0e31ดการรอบบ\u0e34ล";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void FrmDueBill_Load(object sender, EventArgs e)
	{
		Button1.Enabled = false;
		Button5.Enabled = false;
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 00:00:00");
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 23:59:59");
		LoadBill();
		LoadBillNow();
	}

	public void LoadBill()
	{
		DataSet dataSet = Module1.connect("select top 100 * from View_RBill_H where round_end is not null and (round_start between '" + Conversions.ToString(DateTimePicker1.Value) + "' and '" + Conversions.ToString(DateTimePicker2.Value) + "') order by id desc");
		ListView1.Items.Clear();
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
					global::PrintableListView.PrintableListView listView = ListView1;
					int count = listView.Items.Count;
					listView.Items.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["id"]), "0000000"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_start"]), "dd/MM/yyyy HH:mm:ss"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_end"]), "dd/MM/yyyy HH:mm:ss"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_price"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_price_rec"]), "#,##0.00"));
					listView.Items[count].SubItems.Add("-" + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_price_pay"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["dep_rec"]), "#,##0.00"));
					listView.Items[count].SubItems.Add("-" + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["dep_pay"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(Operators.SubtractObject(Operators.AddObject(Operators.SubtractObject(dataSet.Tables[0].Rows[num2]["round_price_rec"], dataSet.Tables[0].Rows[num2]["round_price_pay"]), dataSet.Tables[0].Rows[num2]["dep_rec"]), dataSet.Tables[0].Rows[num2]["dep_pay"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_price_credit"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["round_price_tran"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(Operators.SubtractObject(Operators.AddObject(Operators.SubtractObject(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[num2]["round_price_tran"], dataSet.Tables[0].Rows[num2]["round_price_credit"]), dataSet.Tables[0].Rows[num2]["round_price_rec"]), dataSet.Tables[0].Rows[num2]["round_price_pay"]), dataSet.Tables[0].Rows[num2]["dep_rec"]), dataSet.Tables[0].Rows[num2]["dep_pay"]), "#,##0.00"));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					object[] array = new object[1];
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					string columnName = "round_by";
					array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", array2, null, null, array3, IgnoreReturn: true);
					if (array3[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
					}
					listView = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void LoadBillNow()
	{
		DataSet dataSet = Module1.connect("select top 1 * from View_RBill_H where round_end is null order by id desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			Label6.Text = "-";
			Label2.Text = "-";
			Label4.Text = "0.00";
			Label8.Text = "0.00";
			Label10.Text = "0.00";
			Label12.Text = "0.00";
			Label100.Text = "0.00";
			Label101.Text = "0.00";
			Labelcredit.Text = "0.00";
			Labeltran.Text = "0.00";
			Labeltotal.Text = "0.00";
			Button1.Enabled = true;
			Button5.Enabled = false;
		}
		else
		{
			Label6.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["id"]), "0000000");
			Label2.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd/MM/yyyy HH:mm:ss");
			Label8.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_price"]), "#,##0.00");
			Label10.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_price_rec"]), "#,##0.00");
			Label100.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["dep_rec"]), "#,##0.00");
			Label101.Text = "-" + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["dep_pay"]), "#,##0.00");
			Label4.Text = "-" + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_price_pay"]), "#,##0.00");
			Label12.Text = Strings.Format(Operators.SubtractObject(Operators.AddObject(Operators.SubtractObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["round_price"], dataSet.Tables[0].Rows[0]["round_price_rec"]), dataSet.Tables[0].Rows[0]["round_price_pay"]), dataSet.Tables[0].Rows[0]["dep_rec"]), dataSet.Tables[0].Rows[0]["round_price_pay"]), "#,##0.00");
			Labelcredit.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_price_credit"]), "#,##0.00");
			Labeltran.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_price_tran"]), "#,##0.00");
			Labeltotal.Text = Strings.Format(Operators.SubtractObject(Operators.AddObject(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["round_price_tran"], dataSet.Tables[0].Rows[0]["round_price_credit"]), dataSet.Tables[0].Rows[0]["round_price"]), dataSet.Tables[0].Rows[0]["round_price_rec"]), dataSet.Tables[0].Rows[0]["round_price_pay"]), "#,##0.00");
			Button1.Enabled = false;
			Button5.Enabled = true;
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Button1.Enabled)
		{
			MyProject.Forms.FormConfirmRoundBill.LabelName.Text = Conversions.ToString(Module1.loginName);
			MyProject.Forms.FormConfirmRoundBill.TextBoxX_0.Text = Conversions.ToString(0);
			MyProject.Forms.FormConfirmRoundBill.ShowDialog();
			if (!MyProject.Forms.FormConfirmRoundBill.ISOK)
			{
				return;
			}
			object obj = MyProject.Forms.FormConfirmRoundBill.TextBoxX_0.Text;
			if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
			{
				MessageBox.Show("กร\u0e38ณาใส\u0e48จำนวนเง\u0e34น");
				return;
			}
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
			{
				MessageBox.Show("กร\u0e38ณาใส\u0e48จำนวนเง\u0e34นเป\u0e47นต\u0e31วเลข");
				return;
			}
			object left = "INSERT INTO [HT_Round_Bill]";
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, "[id],[round_start]");
			left = Operators.ConcatenateObject(left, ",[round_price]");
			left = Operators.ConcatenateObject(left, ",[round_by]");
			left = Operators.ConcatenateObject(left, ")");
			left = Operators.ConcatenateObject(left, "VALUES");
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, "" + Conversions.ToString(Module1.get_id("HT_Round_Bill", "id")));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now), "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(obj)));
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Module1.loginName), "'"));
			left = Operators.ConcatenateObject(left, ")");
			Module1.connect(Conversions.ToString(left));
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการป\u0e34ดรอบบ\u0e34ลหร\u0e37อไม\u0e48", "ป\u0e34ดรอบบ\u0e34ล", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("update HT_Round_Bill set round_end='" + Conversions.ToString(DateTime.Now), "',round_by='"), Module1.loginName), "' where round_end is null")));
		}
		LoadBill();
		LoadBillNow();
	}

	public int listdue()
	{
		DataSet dataSet = Module1.connect("select * from TB_Due where due_start between '" + Conversions.ToString(DateTime.Now.Date) + " 00:00:00' and '" + Conversions.ToString(DateTime.Now.Date) + " 23:59:59' order by due_num");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return 1;
		}
		return Conversions.ToInteger(Operators.AddObject(dataSet.Tables[0].Rows[0]["due_num"], 1));
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		ListView1.Title = "รายงานรอบบ\u0e34ล";
		ListView1.PrintPreview();
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		LoadBill();
	}

	private void GroupBox1_Enter(object sender, EventArgs e)
	{
	}
}
