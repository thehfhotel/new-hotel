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
public class FrmReportShiftCash : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

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

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ลบToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("Labelstatus")]
	private Label _Labelstatus;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("ColumnHeader17")]
	private ColumnHeader _ColumnHeader17;

	[AccessedThroughProperty("ColumnHeader18")]
	private ColumnHeader _ColumnHeader18;

	private decimal Pay_DEP;

	private decimal Rec_DEP;

	private decimal CASH_DRAWER;

	private string debt_name;

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

	internal virtual ContextMenuStrip ContextMenuStrip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ContextMenuStrip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ContextMenuStrip1 = value;
		}
	}

	internal virtual ToolStripMenuItem ToolStripMenuItem_0
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_0_Click;
			if (toolStripMenuItem_0 != null)
			{
				toolStripMenuItem_0.Click -= value2;
			}
			toolStripMenuItem_0 = value;
			if (toolStripMenuItem_0 != null)
			{
				toolStripMenuItem_0.Click += value2;
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

	internal virtual Label Labelstatus
	{
		[DebuggerNonUserCode]
		get
		{
			return _Labelstatus;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Labelstatus = value;
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
			EventHandler value2 = CheckBox1_CheckedChanged;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged -= value2;
			}
			_CheckBox1 = value;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged += value2;
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

	[DebuggerNonUserCode]
	static FrmReportShiftCash()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmReportShiftCash()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmReportImcome_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		Pay_DEP = default(decimal);
		Rec_DEP = default(decimal);
		CASH_DRAWER = default(decimal);
		debt_name = "จ\u0e48ายหน\u0e35\u0e49";
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
		this.components = new System.ComponentModel.Container();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.Button3 = new System.Windows.Forms.Button();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader17 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.Labelstatus = new System.Windows.Forms.Label();
		this.ColumnHeader18 = new System.Windows.Forms.ColumnHeader();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.CheckBox1);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.Button3);
		this.GroupBox1.Controls.Add(this.ListView1);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1048, 483);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายงานป\u0e34ดรอบ/เง\u0e34นสดคงเหล\u0e37อตามรอบบ\u0e34ล";
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(549, 32);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(119, 21);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 96;
		this.CheckBox1.Text = "ไม\u0e48แสดงรายจ\u0e48าย";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.DropDownWidth = 500;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "ท\u0e31\u0e49งหมด", "บ\u0e34ลธรรมดา", "บ\u0e34ลภาษ\u0e35" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(78, 29);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(375, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 95;
		this.Label5.AutoSize = true;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label5.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label = this.Label5;
		location = new System.Drawing.Point(11, 33);
		label.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label2 = this.Label5;
		size = new System.Drawing.Size(64, 16);
		label2.Size = size;
		this.Label5.TabIndex = 94;
		this.Label5.Text = "รอบบ\u0e34ลท\u0e35\u0e48 :";
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(459, 29);
		button.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button2 = this.Button3;
		size = new System.Drawing.Size(84, 25);
		button2.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "ค\u0e49นหา";
		this.Button3.UseVisualStyleBackColor = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[18]
		{
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader5, this.ColumnHeader16, this.ColumnHeader3, this.ColumnHeader7, this.ColumnHeader4, this.ColumnHeader6, this.ColumnHeader14, this.ColumnHeader15,
			this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader8, this.ColumnHeader11, this.ColumnHeader12, this.ColumnHeader18, this.ColumnHeader13, this.ColumnHeader17
		});
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(6, 59);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(1033, 418);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader1.Width = 40;
		this.ColumnHeader2.Text = "เลขท\u0e35\u0e48ใบเสร\u0e47จ";
		this.ColumnHeader2.Width = 90;
		this.ColumnHeader5.Text = "ว\u0e31นท\u0e35\u0e48ร\u0e31บ/จ\u0e48าย";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader5.Width = 120;
		this.ColumnHeader16.Text = "เลขลงทะเบ\u0e35ยน";
		this.ColumnHeader16.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader16.Width = 100;
		this.ColumnHeader3.Text = "เบอร\u0e4cห\u0e49อง";
		this.ColumnHeader3.Width = 170;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า/รายละเอ\u0e35ยด";
		this.ColumnHeader7.Width = 400;
		this.ColumnHeader4.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49า";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader4.Width = 110;
		this.ColumnHeader6.Text = "เรทห\u0e49อง";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader6.Width = 80;
		this.ColumnHeader14.Text = "รวมค\u0e48าห\u0e49องท\u0e31\u0e49งหมด";
		this.ColumnHeader14.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader14.Width = 120;
		this.ColumnHeader15.Text = "รวมค\u0e48าส\u0e34นค\u0e49า";
		this.ColumnHeader15.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader15.Width = 80;
		this.ColumnHeader9.Text = "เง\u0e34นสด";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 80;
		this.ColumnHeader10.Text = "บ\u0e31ตรเครด\u0e34ต";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 80;
		this.ColumnHeader8.Text = "ร\u0e31บหน\u0e35\u0e49";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Width = 80;
		this.ColumnHeader11.Text = "ฟร\u0e35 (คอมม\u0e34ชช\u0e31\u0e48น)";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 80;
		this.ColumnHeader12.Text = "โอน";
		this.ColumnHeader12.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader12.Width = 80;
		this.ColumnHeader13.Text = "พน\u0e31กงาน";
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader17.Text = "สาขา";
		this.ColumnHeader17.Width = 100;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[1] { this.ToolStripMenuItem_0 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(93, 26);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "ลบToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(92, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ลบ";
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(977, 503);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(84, 23);
		button4.Size = size;
		this.Button2.TabIndex = 3;
		this.Button2.Text = "ป\u0e34ด";
		this.Button2.UseVisualStyleBackColor = true;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button5 = this.Button1;
		location = new System.Drawing.Point(887, 503);
		button5.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button6 = this.Button1;
		size = new System.Drawing.Size(84, 23);
		button6.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "พ\u0e34มพ\u0e4c";
		this.Button1.UseVisualStyleBackColor = true;
		this.Labelstatus.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Labelstatus.AutoSize = true;
		this.Labelstatus.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label labelstatus = this.Labelstatus;
		location = new System.Drawing.Point(676, 509);
		labelstatus.Location = location;
		this.Labelstatus.Name = "Labelstatus";
		System.Windows.Forms.Label labelstatus2 = this.Labelstatus;
		size = new System.Drawing.Size(205, 17);
		labelstatus2.Size = size;
		this.Labelstatus.TabIndex = 5;
		this.Labelstatus.Text = "**ต\u0e49องป\u0e34ดรอบบ\u0e34ลก\u0e48อนถ\u0e36งจะพ\u0e34มพ\u0e4cได\u0e49";
		this.Labelstatus.Visible = false;
		this.ColumnHeader18.Text = "เว\u0e47บ";
		this.ColumnHeader18.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader18.Width = 80;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1073, 539);
		this.ClientSize = size;
		this.Controls.Add(this.Labelstatus);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.GroupBox1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportShiftCash";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานป\u0e34ดรอบ/เง\u0e34นสดคงเหล\u0e37อตามรอบบ\u0e34ล";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		search();
	}

	public void search()
	{
		Cursor = Cursors.WaitCursor;
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
		decimal num = default(decimal);
		object left = "select cust_name,Pay_by,pay_no,cin_no,cin_pay_cash,Cin_Pay_Free,cin_pay_credit,cin_pay_date,Cin_Pay_note,Cin_Pay_Tran,Cin_Pay_web,Branch,Cin_Status from View_Pay_Ds";
		object left2 = "select pay_no,cin_pay_ds from HT_CheckIn_Pay";
		object left3 = "select * from View_CheckIn_H where cin_no in (select cin_no from View_Pay_Ds";
		object left4 = "select * from HT_CheckIn_Ds where cin_no in (select cin_no from View_Pay_Ds";
		object left5 = "select COALESCE(sum(cin_room_dep),0) as dep from HT_CheckIn_Ds where ";
		object left6 = "select COALESCE(sum(cin_room_dep),0) as dep from HT_CheckIn_Ds where ";
		object left7 = "select * from TB_Pay_History where ";
		object left8 = "select * from HT_Rooms_Cancel where";
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left3 = Operators.ConcatenateObject(left3, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left4 = Operators.ConcatenateObject(left4, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left5 = Operators.ConcatenateObject(left5, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" (cin_dep_return_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left6 = Operators.ConcatenateObject(left6, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" (cin_room_in between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left7 = Operators.ConcatenateObject(left7, string.Concat(string.Concat(string.Concat(" (pay_date between " + Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[0]["round_start"]).ToOADate()), " and "), Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[0]["round_end"]).ToOADate())), ")"));
			left8 = Operators.ConcatenateObject(left8, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" (cancel_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
		}
		else
		{
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left3 = Operators.ConcatenateObject(left3, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left4 = Operators.ConcatenateObject(left4, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left5 = Operators.ConcatenateObject(left5, Operators.ConcatenateObject(Operators.ConcatenateObject("  (cin_dep_return_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left6 = Operators.ConcatenateObject(left6, Operators.ConcatenateObject(Operators.ConcatenateObject("  (cin_room_in >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left7 = Operators.ConcatenateObject(left7, string.Concat("  pay_date >= " + Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[0]["round_start"]).ToOADate()), ""));
			left8 = Operators.ConcatenateObject(left8, Operators.ConcatenateObject(Operators.ConcatenateObject(" (cancel_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
		}
		left = Operators.ConcatenateObject(left, "  and Cin_Status<>'ยกเล\u0e34ก' group by  cust_name,Pay_by,pay_no,cin_no,cin_pay_cash,Cin_Pay_Free,cin_pay_credit,cin_pay_date,Cin_Pay_note,Cin_Pay_Tran,Cin_Pay_web,Branch,Cin_Status order by pay_no");
		left2 = Operators.ConcatenateObject(left2, "  and Cin_Status<>'ยกเล\u0e34ก' order by pay_no");
		left3 = Operators.ConcatenateObject(left3, ")");
		left4 = Operators.ConcatenateObject(left4, ")");
		left5 = Operators.ConcatenateObject(left5, "");
		left6 = Operators.ConcatenateObject(left6, "");
		left7 = Operators.ConcatenateObject(left7, " order by pay_date");
		left8 = Operators.ConcatenateObject(left8, " order by cancel_date");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(left));
		DataSet dataSet3 = Module1.connect(Conversions.ToString(left2));
		DataSet dataSet4 = Module1.connect(Conversions.ToString(left3));
		DataSet dataSet5 = Module1.connect(Conversions.ToString(left4));
		DataSet dataSet6 = Module1.connect(Conversions.ToString(left5));
		DataSet dataSet7 = Module1.connect(Conversions.ToString(left6));
		DataSet dataSet8 = Module1.connect(Conversions.ToString(left7));
		DataSet dataSet9 = Module1.connect(Conversions.ToString(left8));
		ListView1.Items.Clear();
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		decimal num5 = default(decimal);
		decimal num6 = default(decimal);
		decimal num7 = default(decimal);
		checked
		{
			int num8 = dataSet2.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num11 = num8;
				if (num10 > num11)
				{
					break;
				}
				string left9 = "";
				int num12 = dataSet3.Tables[0].Rows.Count - 1;
				int num13 = 0;
				while (true)
				{
					int num14 = num13;
					num11 = num12;
					if (num14 > num11)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num9]["pay_no"].ToString(), dataSet3.Tables[0].Rows[num13]["pay_no"], TextCompare: false))
					{
						left9 = Conversions.ToString(Operators.ConcatenateObject(left9, Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num13]["cin_pay_ds"], " ")));
					}
					num13++;
				}
				bool flag = false;
				int num15 = dataSet4.Tables[0].Rows.Count - 1;
				int num16 = 0;
				while (true)
				{
					int num17 = num16;
					num11 = num15;
					if (num17 > num11)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num16]["cin_no"], dataSet2.Tables[0].Rows[num9]["cin_no"], TextCompare: false))
					{
						int count = ListView1.Items.Count;
						global::PrintableListView.PrintableListView listView = ListView1;
						listView.Items.Add(Conversions.ToString(count + 1));
						ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
						object[] array = new object[1];
						object[] array2 = array;
						DataRow dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow2 = dataRow;
						string columnName = "pay_no";
						array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
						object[] array3 = array;
						object[] arguments = array3;
						bool[] array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
						}
						listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num9]["cin_pay_date"]), "dd/MM/yy HH:mm"));
						ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
						array3 = new object[1];
						object[] array5 = array3;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow3 = dataRow;
						columnName = "cin_no";
						array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
						array = array3;
						object[] arguments2 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						listView.Items[count].SubItems.Add(left9);
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num9]["Cin_Status"], 3, TextCompare: false))
						{
							NewLateBinding.LateCall(listView.Items[count].SubItems, null, "Add", new object[1] { Operators.ConcatenateObject(string.Concat("[" + dataSet2.Tables[0].Rows[num9]["Cin_Pay_note"].ToString(), "] "), dataSet4.Tables[0].Rows[num16]["cust_name"]) }, null, null, null, IgnoreReturn: true);
							listView.Items[count].BackColor = Color.Red;
							listView.Items[count].ForeColor = Color.White;
						}
						else
						{
							ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
							array3 = new object[1];
							object[] array6 = array3;
							dataRow = dataSet4.Tables[0].Rows[num16];
							DataRow dataRow4 = dataRow;
							columnName = "cust_name";
							array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
							array = array3;
							object[] arguments3 = array;
							array4 = new bool[1] { true };
							NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
							if (array4[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
							}
						}
						listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num16]["cin_date"]), "dd/MM/yy HH:mm"));
						string text = "";
						int num18 = dataSet5.Tables[0].Rows.Count - 1;
						int num19 = 0;
						while (true)
						{
							int num20 = num19;
							num11 = num18;
							if (num20 <= num11)
							{
								if (!Operators.ConditionalCompareObjectEqual(dataSet5.Tables[0].Rows[num19]["cin_no"], dataSet2.Tables[0].Rows[num9]["cin_no"], TextCompare: false))
								{
									num19++;
									continue;
								}
								text = Conversions.ToString(dataSet5.Tables[0].Rows[num19]["cin_room_price"]);
								break;
							}
							break;
						}
						listView.Items[count].SubItems.Add(text);
						ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[count].SubItems;
						array3 = new object[1];
						object[] array7 = array3;
						dataRow = dataSet4.Tables[0].Rows[num16];
						DataRow dataRow5 = dataRow;
						columnName = "total_price_room";
						array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
						array = array3;
						object[] arguments4 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[count].SubItems;
						array3 = new object[1];
						object[] array8 = array3;
						dataRow = dataSet4.Tables[0].Rows[num16];
						DataRow dataRow6 = dataRow;
						columnName = "total_price_product";
						array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
						array = array3;
						object[] arguments5 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems5, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						string instance = dataSet2.Tables[0].Rows[num9]["Cin_Pay_note"].ToString();
						array = new object[1] { RuntimeHelpers.GetObjectValue(debt_name) };
						object[] arguments6 = array;
						array4 = new bool[1] { true };
						object left10 = NewLateBinding.LateGet(instance, null, "IndexOf", arguments6, null, null, array4);
						if (array4[0])
						{
							debt_name = (string)RuntimeHelpers.GetObjectValue(array[0]);
						}
						if (Operators.ConditionalCompareObjectNotEqual(left10, -1, TextCompare: false))
						{
							ListViewItem.ListViewSubItemCollection subItems6 = listView.Items[count].SubItems;
							object[] array9 = new object[1];
							object[] array10 = array9;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow7 = dataRow;
							columnName = "Cin_Pay_Cash";
							array10[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
							object[] array11 = array9;
							object[] arguments7 = array11;
							bool[] array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems6, null, "Add", arguments7, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array11[0]);
							}
							ListViewItem.ListViewSubItemCollection subItems7 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array13 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow8 = dataRow;
							columnName = "Cin_Pay_Credit";
							array13[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
							array9 = array11;
							object[] arguments8 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems7, null, "Add", arguments8, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							NewLateBinding.LateCall(listView.Items[count].SubItems, null, "Add", new object[1] { Operators.AddObject(Operators.AddObject(dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"], dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"]), dataSet2.Tables[0].Rows[num9]["Cin_Pay_Free"]) }, null, null, null, IgnoreReturn: true);
							ListViewItem.ListViewSubItemCollection subItems8 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array14 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow9 = dataRow;
							columnName = "Cin_Pay_Free";
							array14[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
							array9 = array11;
							object[] arguments9 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems8, null, "Add", arguments9, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							ListViewItem.ListViewSubItemCollection subItems9 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array15 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow10 = dataRow;
							columnName = "Cin_Pay_Tran";
							array15[0] = RuntimeHelpers.GetObjectValue(dataRow10[columnName]);
							array9 = array11;
							object[] arguments10 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems9, null, "Add", arguments10, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							ListViewItem.ListViewSubItemCollection subItems10 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array16 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow11 = dataRow;
							columnName = "Cin_Pay_web";
							array16[0] = RuntimeHelpers.GetObjectValue(dataRow11[columnName]);
							array9 = array11;
							object[] arguments11 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems10, null, "Add", arguments11, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							num5 = Conversions.ToDecimal(Operators.AddObject(num5, Operators.AddObject(dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"], dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"])));
							num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"]));
							num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"]));
							num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Free"]));
							num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Tran"]));
							num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet2.Tables[0].Rows[num9]["Cin_Pay_web"]));
						}
						else
						{
							ListViewItem.ListViewSubItemCollection subItems11 = listView.Items[count].SubItems;
							object[] array11 = new object[1];
							object[] array17 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow12 = dataRow;
							columnName = "Cin_Pay_Cash";
							array17[0] = RuntimeHelpers.GetObjectValue(dataRow12[columnName]);
							object[] array9 = array11;
							object[] arguments12 = array9;
							bool[] array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems11, null, "Add", arguments12, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							ListViewItem.ListViewSubItemCollection subItems12 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array18 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow13 = dataRow;
							columnName = "Cin_Pay_Credit";
							array18[0] = RuntimeHelpers.GetObjectValue(dataRow13[columnName]);
							array9 = array11;
							object[] arguments13 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems12, null, "Add", arguments13, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							listView.Items[count].SubItems.Add(Conversions.ToString(0));
							ListViewItem.ListViewSubItemCollection subItems13 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array19 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow14 = dataRow;
							columnName = "Cin_Pay_Free";
							array19[0] = RuntimeHelpers.GetObjectValue(dataRow14[columnName]);
							array9 = array11;
							object[] arguments14 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems13, null, "Add", arguments14, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							ListViewItem.ListViewSubItemCollection subItems14 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array20 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow15 = dataRow;
							columnName = "Cin_Pay_Tran";
							array20[0] = RuntimeHelpers.GetObjectValue(dataRow15[columnName]);
							array9 = array11;
							object[] arguments15 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems14, null, "Add", arguments15, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							ListViewItem.ListViewSubItemCollection subItems15 = listView.Items[count].SubItems;
							array11 = new object[1];
							object[] array21 = array11;
							dataRow = dataSet2.Tables[0].Rows[num9];
							DataRow dataRow16 = dataRow;
							columnName = "Cin_Pay_web";
							array21[0] = RuntimeHelpers.GetObjectValue(dataRow16[columnName]);
							array9 = array11;
							object[] arguments16 = array9;
							array12 = new bool[1] { true };
							NewLateBinding.LateCall(subItems15, null, "Add", arguments16, null, null, array12, IgnoreReturn: true);
							if (array12[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
							}
							num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"]));
							num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"]));
							num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Free"]));
							num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Tran"]));
							num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet2.Tables[0].Rows[num9]["Cin_Pay_web"]));
						}
						listView.Items[count].SubItems.Add(dataSet2.Tables[0].Rows[num9]["Pay_by"].ToString());
						listView.Items[count].SubItems.Add(" " + dataSet2.Tables[0].Rows[num9]["Branch"].ToString());
						listView = null;
						flag = true;
					}
					num16++;
				}
				if (!flag)
				{
					string text2 = "  - ";
					int num21 = dataSet3.Tables[0].Rows.Count - 1;
					int num22 = 0;
					while (true)
					{
						int num23 = num22;
						num11 = num21;
						if (num23 > num11)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num9]["pay_no"].ToString(), dataSet3.Tables[0].Rows[num22]["pay_no"], TextCompare: false))
						{
							text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num22]["cin_pay_ds"], " ")));
						}
						num22++;
					}
					int count2 = ListView1.Items.Count;
					global::PrintableListView.PrintableListView listView2 = ListView1;
					listView2.Items.Add(Conversions.ToString(count2 + 1));
					listView2.Items[count2].SubItems.Add(dataSet2.Tables[0].Rows[num9]["pay_no"].ToString());
					listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num9]["cin_pay_date"]), "dd/MM/yy HH:mm"));
					listView2.Items[count2].SubItems.Add(dataSet2.Tables[0].Rows[num9]["cin_no"].ToString());
					listView2.Items[count2].SubItems.Add(Strings.Trim(text2.Replace(" - การจองแบบ", "จอง")));
					if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num9]["Cin_Status"], 3, TextCompare: false))
					{
						listView2.Items[count2].SubItems.Add("[" + dataSet2.Tables[0].Rows[num9]["Cin_Pay_note"].ToString() + "] " + dataSet2.Tables[0].Rows[num9]["cust_name"].ToString());
						listView2.Items[count2].BackColor = Color.Red;
						listView2.Items[count2].ForeColor = Color.White;
					}
					else
					{
						listView2.Items[count2].SubItems.Add(dataSet2.Tables[0].Rows[num9]["cust_name"].ToString());
					}
					listView2.Items[count2].SubItems.Add("-");
					listView2.Items[count2].SubItems.Add("-");
					listView2.Items[count2].SubItems.Add("0");
					ListViewItem.ListViewSubItemCollection subItems16 = listView2.Items[count2].SubItems;
					object[] array11 = new object[1];
					object[] array22 = array11;
					DataRow dataRow = dataSet2.Tables[0].Rows[num9];
					DataRow dataRow17 = dataRow;
					string columnName = "Cin_Pay_Cash";
					array22[0] = RuntimeHelpers.GetObjectValue(dataRow17[columnName]);
					object[] array9 = array11;
					object[] arguments17 = array9;
					bool[] array12 = new bool[1] { true };
					NewLateBinding.LateCall(subItems16, null, "Add", arguments17, null, null, array12, IgnoreReturn: true);
					if (array12[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
					}
					string instance2 = dataSet2.Tables[0].Rows[num9]["Cin_Pay_note"].ToString();
					array9 = new object[1] { RuntimeHelpers.GetObjectValue(debt_name) };
					object[] arguments18 = array9;
					array12 = new bool[1] { true };
					object left11 = NewLateBinding.LateGet(instance2, null, "IndexOf", arguments18, null, null, array12);
					if (array12[0])
					{
						debt_name = (string)RuntimeHelpers.GetObjectValue(array9[0]);
					}
					if (Operators.ConditionalCompareObjectNotEqual(left11, -1, TextCompare: false))
					{
						ListViewItem.ListViewSubItemCollection subItems17 = listView2.Items[count2].SubItems;
						object[] array3 = new object[1];
						object[] array23 = array3;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow18 = dataRow;
						columnName = "Cin_Pay_Cash";
						array23[0] = RuntimeHelpers.GetObjectValue(dataRow18[columnName]);
						object[] array = array3;
						object[] arguments19 = array;
						bool[] array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems17, null, "Add", arguments19, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems18 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array24 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow19 = dataRow;
						columnName = "Cin_Pay_Credit";
						array24[0] = RuntimeHelpers.GetObjectValue(dataRow19[columnName]);
						array9 = array11;
						object[] arguments20 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems18, null, "Add", arguments20, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						NewLateBinding.LateCall(listView2.Items[count2].SubItems, null, "Add", new object[1] { Operators.AddObject(dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"], dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"]) }, null, null, null, IgnoreReturn: true);
						ListViewItem.ListViewSubItemCollection subItems19 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array25 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow20 = dataRow;
						columnName = "Cin_Pay_Free";
						array25[0] = RuntimeHelpers.GetObjectValue(dataRow20[columnName]);
						array9 = array11;
						object[] arguments21 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems19, null, "Add", arguments21, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems20 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array26 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow21 = dataRow;
						columnName = "Cin_Pay_Tran";
						array26[0] = RuntimeHelpers.GetObjectValue(dataRow21[columnName]);
						array9 = array11;
						object[] arguments22 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems20, null, "Add", arguments22, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems21 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array27 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow22 = dataRow;
						columnName = "Cin_Pay_web";
						array27[0] = RuntimeHelpers.GetObjectValue(dataRow22[columnName]);
						array9 = array11;
						object[] arguments23 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems21, null, "Add", arguments23, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						num5 = Conversions.ToDecimal(Operators.AddObject(num5, Operators.AddObject(Operators.AddObject(dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"], dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"]), dataSet2.Tables[0].Rows[num9]["Cin_Pay_Free"])));
						num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"]));
						num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"]));
						num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Free"]));
						num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Tran"]));
						num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet2.Tables[0].Rows[num9]["Cin_Pay_web"]));
					}
					else
					{
						ListViewItem.ListViewSubItemCollection subItems22 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array28 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow23 = dataRow;
						columnName = "Cin_Pay_Cash";
						array28[0] = RuntimeHelpers.GetObjectValue(dataRow23[columnName]);
						array9 = array11;
						object[] arguments24 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems22, null, "Add", arguments24, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems23 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array29 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow24 = dataRow;
						columnName = "Cin_Pay_Credit";
						array29[0] = RuntimeHelpers.GetObjectValue(dataRow24[columnName]);
						array9 = array11;
						object[] arguments25 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems23, null, "Add", arguments25, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						listView2.Items[count2].SubItems.Add(Conversions.ToString(0));
						ListViewItem.ListViewSubItemCollection subItems24 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array30 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow25 = dataRow;
						columnName = "Cin_Pay_Free";
						array30[0] = RuntimeHelpers.GetObjectValue(dataRow25[columnName]);
						array9 = array11;
						object[] arguments26 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems24, null, "Add", arguments26, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems25 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array31 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow26 = dataRow;
						columnName = "Cin_Pay_Tran";
						array31[0] = RuntimeHelpers.GetObjectValue(dataRow26[columnName]);
						array9 = array11;
						object[] arguments27 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems25, null, "Add", arguments27, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						ListViewItem.ListViewSubItemCollection subItems26 = listView2.Items[count2].SubItems;
						array11 = new object[1];
						object[] array32 = array11;
						dataRow = dataSet2.Tables[0].Rows[num9];
						DataRow dataRow27 = dataRow;
						columnName = "Cin_Pay_web";
						array32[0] = RuntimeHelpers.GetObjectValue(dataRow27[columnName]);
						array9 = array11;
						object[] arguments28 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems26, null, "Add", arguments28, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Cash"]));
						num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Credit"]));
						num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Free"]));
						num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet2.Tables[0].Rows[num9]["Cin_Pay_Tran"]));
						num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet2.Tables[0].Rows[num9]["Cin_Pay_web"]));
					}
					listView2.Items[count2].SubItems.Add(dataSet2.Tables[0].Rows[num9]["Pay_by"].ToString());
					listView2.Items[count2].SubItems.Add(" " + dataSet2.Tables[0].Rows[num9]["Branch"].ToString());
					listView2 = null;
				}
				num9++;
			}
			if (!CheckBox1.Checked)
			{
				int num24 = dataSet8.Tables[0].Rows.Count - 1;
				int num25 = 0;
				while (true)
				{
					int num26 = num25;
					int num11 = num24;
					if (num26 > num11)
					{
						break;
					}
					int count3 = ListView1.Items.Count;
					global::PrintableListView.PrintableListView listView3 = ListView1;
					listView3.Items.Add(Conversions.ToString(count3 + 1));
					ListViewItem.ListViewSubItemCollection subItems27 = listView3.Items[count3].SubItems;
					object[] array11 = new object[1];
					object[] array33 = array11;
					DataRow dataRow = dataSet8.Tables[0].Rows[num25];
					DataRow dataRow28 = dataRow;
					string columnName = "Pay_Account";
					array33[0] = RuntimeHelpers.GetObjectValue(dataRow28[columnName]);
					object[] array9 = array11;
					object[] arguments29 = array9;
					bool[] array12 = new bool[1] { true };
					NewLateBinding.LateCall(subItems27, null, "Add", arguments29, null, null, array12, IgnoreReturn: true);
					if (array12[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
					}
					listView3.Items[count3].SubItems.Add(Strings.Format(DateTime.FromOADate(Conversions.ToDouble(dataSet8.Tables[0].Rows[num25]["Pay_date"])), "dd/MM/yy HH:mm"));
					listView3.Items[count3].SubItems.Add("-");
					ListViewItem.ListViewSubItemCollection subItems28 = listView3.Items[count3].SubItems;
					array11 = new object[1];
					object[] array34 = array11;
					dataRow = dataSet8.Tables[0].Rows[num25];
					DataRow dataRow29 = dataRow;
					columnName = "Pay_Type";
					array34[0] = RuntimeHelpers.GetObjectValue(dataRow29[columnName]);
					array9 = array11;
					object[] arguments30 = array9;
					array12 = new bool[1] { true };
					NewLateBinding.LateCall(subItems28, null, "Add", arguments30, null, null, array12, IgnoreReturn: true);
					if (array12[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems29 = listView3.Items[count3].SubItems;
					array11 = new object[1];
					object[] array35 = array11;
					dataRow = dataSet8.Tables[0].Rows[num25];
					DataRow dataRow30 = dataRow;
					columnName = "Pay_bill";
					array35[0] = RuntimeHelpers.GetObjectValue(dataRow30[columnName]);
					array9 = array11;
					object[] arguments31 = array9;
					array12 = new bool[1] { true };
					NewLateBinding.LateCall(subItems29, null, "Add", arguments31, null, null, array12, IgnoreReturn: true);
					if (array12[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
					}
					listView3.Items[count3].SubItems.Add("-");
					listView3.Items[count3].SubItems.Add("-");
					listView3.Items[count3].SubItems.Add("-");
					listView3.Items[count3].SubItems.Add("-");
					if (Operators.ConditionalCompareObjectEqual(dataSet8.Tables[0].Rows[num25]["Pay_Type"], "รายจ\u0e48าย", TextCompare: false))
					{
						NewLateBinding.LateCall(listView3.Items[count3].SubItems, null, "Add", new object[1] { Operators.ConcatenateObject("-", dataSet8.Tables[0].Rows[num25]["Pay_Total"]) }, null, null, null, IgnoreReturn: true);
						num2 = Conversions.ToDecimal(Operators.SubtractObject(num2, dataSet8.Tables[0].Rows[num25]["Pay_Total"]));
					}
					else
					{
						ListViewItem.ListViewSubItemCollection subItems30 = listView3.Items[count3].SubItems;
						array11 = new object[1];
						object[] array36 = array11;
						dataRow = dataSet8.Tables[0].Rows[num25];
						DataRow dataRow31 = dataRow;
						columnName = "Pay_Total";
						array36[0] = RuntimeHelpers.GetObjectValue(dataRow31[columnName]);
						array9 = array11;
						object[] arguments32 = array9;
						array12 = new bool[1] { true };
						NewLateBinding.LateCall(subItems30, null, "Add", arguments32, null, null, array12, IgnoreReturn: true);
						if (array12[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
						}
						num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet8.Tables[0].Rows[num25]["Pay_Total"]));
					}
					listView3.Items[count3].SubItems.Add(Conversions.ToString(0));
					listView3.Items[count3].SubItems.Add(Conversions.ToString(0));
					listView3.Items[count3].SubItems.Add(Conversions.ToString(0));
					listView3.Items[count3].SubItems.Add(Conversions.ToString(0));
					listView3.Items[count3].SubItems.Add(Conversions.ToString(0));
					listView3.Items[count3].SubItems.Add("");
					listView3.Items[count3].SubItems.Add("(รายจ\u0e48าย)");
					listView3 = null;
					num25++;
				}
			}
			int count4 = ListView1.Items.Count;
			global::PrintableListView.PrintableListView listView4 = ListView1;
			listView4.Items.Add("");
			listView4.Items[count4].SubItems.Add("รวม");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num2));
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num3));
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num5));
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num4));
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num6));
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num7));
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("รวม");
			listView4.Items[count4].BackColor = Color.LightGreen;
			listView4 = null;
			int num27 = 1;
			int num28 = dataSet9.Tables[0].Rows.Count - 1;
			int num29 = 0;
			while (true)
			{
				int num30 = num29;
				int num11 = num28;
				if (num30 > num11)
				{
					break;
				}
				int count5 = ListView1.Items.Count;
				global::PrintableListView.PrintableListView listView5 = ListView1;
				listView5.Items.Add(Conversions.ToString(num27));
				listView5.Items[count5].SubItems.Add("รายการยกเล\u0e34ก");
				listView5.Items[count5].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet9.Tables[0].Rows[num29]["cancel_date"]), "dd/MM/yy HH:mm"));
				ListViewItem.ListViewSubItemCollection subItems31 = listView5.Items[count5].SubItems;
				object[] array11 = new object[1];
				object[] array37 = array11;
				DataRow dataRow = dataSet9.Tables[0].Rows[num29];
				DataRow dataRow32 = dataRow;
				string columnName = "cin_no";
				array37[0] = RuntimeHelpers.GetObjectValue(dataRow32[columnName]);
				object[] array9 = array11;
				object[] arguments33 = array9;
				bool[] array12 = new bool[1] { true };
				NewLateBinding.LateCall(subItems31, null, "Add", arguments33, null, null, array12, IgnoreReturn: true);
				if (array12[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems32 = listView5.Items[count5].SubItems;
				array11 = new object[1];
				object[] array38 = array11;
				dataRow = dataSet9.Tables[0].Rows[num29];
				DataRow dataRow33 = dataRow;
				columnName = "room_no";
				array38[0] = RuntimeHelpers.GetObjectValue(dataRow33[columnName]);
				array9 = array11;
				object[] arguments34 = array9;
				array12 = new bool[1] { true };
				NewLateBinding.LateCall(subItems32, null, "Add", arguments34, null, null, array12, IgnoreReturn: true);
				if (array12[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems33 = listView5.Items[count5].SubItems;
				array11 = new object[1];
				object[] array39 = array11;
				dataRow = dataSet9.Tables[0].Rows[num29];
				DataRow dataRow34 = dataRow;
				columnName = "cancel_note";
				array39[0] = RuntimeHelpers.GetObjectValue(dataRow34[columnName]);
				array9 = array11;
				object[] arguments35 = array9;
				array12 = new bool[1] { true };
				NewLateBinding.LateCall(subItems33, null, "Add", arguments35, null, null, array12, IgnoreReturn: true);
				if (array12[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
				}
				listView5.Items[count5].SubItems.Add("-");
				listView5.Items[count5].SubItems.Add("-");
				listView5.Items[count5].SubItems.Add("-");
				listView5.Items[count5].SubItems.Add("-");
				listView5.Items[count5].SubItems.Add(Conversions.ToString(0));
				listView5.Items[count5].SubItems.Add(Conversions.ToString(0));
				listView5.Items[count5].SubItems.Add(Conversions.ToString(0));
				listView5.Items[count5].SubItems.Add(Conversions.ToString(0));
				listView5.Items[count5].SubItems.Add(Conversions.ToString(0));
				listView5.Items[count5].SubItems.Add(Conversions.ToString(0));
				ListViewItem.ListViewSubItemCollection subItems34 = listView5.Items[count5].SubItems;
				array11 = new object[1];
				object[] array40 = array11;
				dataRow = dataSet9.Tables[0].Rows[num29];
				DataRow dataRow35 = dataRow;
				columnName = "cancel_by";
				array40[0] = RuntimeHelpers.GetObjectValue(dataRow35[columnName]);
				array9 = array11;
				object[] arguments36 = array9;
				array12 = new bool[1] { true };
				NewLateBinding.LateCall(subItems34, null, "Add", arguments36, null, null, array12, IgnoreReturn: true);
				if (array12[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array9[0]);
				}
				listView5.Items[count5].SubItems.Add("รายการยกเล\u0e34ก");
				listView5.Items[count5].BackColor = Color.Orange;
				listView5 = null;
				num27++;
				num29++;
			}
			int count6 = ListView1.Items.Count;
			global::PrintableListView.PrintableListView listView6 = ListView1;
			listView6.Items.Add("");
			listView6.Items[count6].SubItems.Add("รวมเง\u0e34นสด");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			NewLateBinding.LateCall(listView6.Items[count6].SubItems, null, "Add", new object[1] { Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("เง\u0e34นในล\u0e34\u0e49นช\u0e31ก ", dataSet.Tables[0].Rows[0]["round_price"]), " + "), dataSet7.Tables[0].Rows[0]["dep"]), " (ม\u0e31ดจำ) - "), dataSet6.Tables[0].Rows[0]["dep"]), " (ค\u0e37นม\u0e31ดจำ) + "), num2), " = "), Operators.AddObject(Operators.SubtractObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["round_price"], num2), dataSet6.Tables[0].Rows[0]["dep"]), dataSet7.Tables[0].Rows[0]["dep"])), " (ไม\u0e48รวมเง\u0e34นจอง "), num), " บาท)") }, null, null, null, IgnoreReturn: true);
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].SubItems.Add("");
			listView6.Items[count6].BackColor = Color.LightPink;
			listView6 = null;
			Pay_DEP = Conversions.ToDecimal(dataSet6.Tables[0].Rows[0]["dep"]);
			Rec_DEP = Conversions.ToDecimal(dataSet7.Tables[0].Rows[0]["dep"]);
			CASH_DRAWER = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["round_price"]);
			Cursor = Cursors.Default;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		int index = 0;
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (Operators.CompareString(ListView1.Items[num2].SubItems[1].Text, "รวม", TextCompare: false) != 0)
					{
						num2++;
						continue;
					}
					index = num2;
					break;
				}
				break;
			}
			string text = "";
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
			text = ((Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) == 0) ? ("รายงานป\u0e34ดรอบ/เง\u0e34นสดตามรอบบ\u0e34ลท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(ComboBox1.SelectedValue), "00000") + "\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTime.Now, "dd-MM-yy เวลา HH:mm น.")) : ("รายงานป\u0e34ดรอบ/เง\u0e34นสดตามรอบบ\u0e34ลท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(ComboBox1.SelectedValue), "00000") + "\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_end"]), "dd-MM-yy เวลา HH:mm น.")));
			Module1.localdata.TableShiftCash.Rows.Clear();
			string text2 = "";
			string text3 = "";
			text2 = ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Text;
			text3 = ListView1.Items[index].SubItems[12].Text;
			int num5 = ListView1.Items.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4 || num6 == ListView1.Items.Count - 1)
				{
					break;
				}
				Module1.localdata.TableShiftCash.AddTableShiftCashRow(text, ListView1.Items[num6].SubItems[0].Text, ListView1.Items[num6].SubItems[1].Text, ListView1.Items[num6].SubItems[2].Text, ListView1.Items[num6].SubItems[3].Text, ListView1.Items[num6].SubItems[4].Text, ListView1.Items[num6].SubItems[5].Text, ListView1.Items[num6].SubItems[6].Text, ListView1.Items[num6].SubItems[7].Text, ListView1.Items[num6].SubItems[8].Text, ListView1.Items[num6].SubItems[9].Text, ListView1.Items[num6].SubItems[10].Text, ListView1.Items[num6].SubItems[11].Text, ListView1.Items[num6].SubItems[16].Text, text2, _ListView1.Items[num6].SubItems[12].Text, text3, Strings.Format(Conversions.ToDecimal(ListView1.Items[index].SubItems[10].Text), "#,##0.00"), Strings.Format(Conversions.ToDecimal(ListView1.Items[index].SubItems[11].Text), "#,##0.00"), ListView1.Items[num6].SubItems[13].Text, Strings.Format(Conversions.ToDecimal(ListView1.Items[index].SubItems[13].Text), "#,##0.00"), Strings.Format(Rec_DEP, "#,##0.00"), Strings.Format(Pay_DEP, "#,##0.00"), Strings.Format(decimal.Add(decimal.Subtract(decimal.Add(Conversions.ToDecimal(ListView1.Items[index].SubItems[10].Text), Rec_DEP), Pay_DEP), CASH_DRAWER), "#,##0.00"), Strings.Format(CASH_DRAWER, "#,##0.00"), ListView1.Items[num6].SubItems[14].Text, Strings.Format(Conversions.ToDecimal(ListView1.Items[index].SubItems[14].Text), "#,##0.00"), Strings.Format(decimal.Add(decimal.Add(decimal.Add(Conversions.ToDecimal(ListView1.Items[index].SubItems[10].Text), Conversions.ToDecimal(ListView1.Items[index].SubItems[14].Text)), Conversions.ToDecimal(ListView1.Items[index].SubItems[11].Text)), Conversions.ToDecimal(ListView1.Items[index].SubItems[15].Text)), "#,##0.00"), ListView1.Items[num6].SubItems[17].Text, ListView1.Items[num6].SubItems[15].Text, Strings.Format(Conversions.ToDecimal(ListView1.Items[index].SubItems[15].Text), "#,##0.00"));
				num6++;
			}
			MyProject.Forms.FrmPrint.Close();
			ReportShipCash reportShipCash = new ReportShipCash();
			reportShipCash.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.Show();
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportShipCash;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		ListDueBill();
		search();
	}

	public void ListDueBill()
	{
		DataSet dataSet = Module1.connect("select top 1000 id,FullDate from View_RBill_H_Round_Only order by id desc");
		ComboBox1.DataSource = dataSet.Tables[0];
		ComboBox1.DisplayMember = "FullDate";
		ComboBox1.ValueMember = "id";
		ComboBox1.Text = "";
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			Labelstatus.Visible = false;
			Button1.Enabled = true;
			if (ComboBox1.Text.IndexOf("ป\u0e31จจ\u0e38บ\u0e31น") != -1)
			{
				Labelstatus.Visible = true;
				Button1.Enabled = false;
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		try
		{
			search();
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			ProjectData.ClearProjectError();
		}
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		MyProject.Forms.FormPass.ShowDialog();
		if (Operators.CompareString(MyProject.Forms.FormPass.TextBox1.Text, "", TextCompare: false) != 0)
		{
			DataSet dataSet = Module1.connect("select * from TB_MRP_EMPLOYEE where emp_level='admin' and emp_password='" + MyProject.Forms.FormPass.TextBox1.Text + "'");
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show("รห\u0e31สผ\u0e48านไม\u0e48ถ\u0e39กต\u0e49อง");
				Module1.LOG(Conversions.ToString(Operators.ConcatenateObject("ลบใบร\u0e31บเง\u0e34น ไม\u0e48สำเร\u0e47จ (รห\u0e31สผ\u0e48านผ\u0e34ด) : ", Module1.loginName)));
			}
			else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบใบเสร\u0e47จ " + ListView1.SelectedItems[0].SubItems[1].Text + " หร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
			{
				Module1.connect("delete from HT_CheckIn_Pay where pay_no='" + ListView1.SelectedItems[0].SubItems[1].Text + "'");
				Module1.LOG(Conversions.ToString(Operators.ConcatenateObject(string.Concat("ลบใบร\u0e31บเง\u0e34น " + ListView1.SelectedItems[0].SubItems[1].Text, " : "), Module1.loginName)));
				search();
			}
		}
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
		try
		{
			search();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}
}
