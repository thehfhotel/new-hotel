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
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmSETsale : Office2007Form
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

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("Tname")]
	private TextBox _Tname;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ลบToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

	[AccessedThroughProperty("แกไขToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Tid")]
	private TextBox _Tid;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("Taddress")]
	private TextBox _Taddress;

	[AccessedThroughProperty("Tother")]
	private TextBox _Tother;

	[AccessedThroughProperty("Ttel")]
	private TextBox _Ttel;

	private string Badd;

	private string Bupdate;

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

	internal virtual PanelEx PanelEx2
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx2 = value;
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

	internal virtual TextBox Tname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tname = value;
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
			EventHandler value2 = Button2_Click;
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

	internal virtual ToolStripMenuItem ToolStripMenuItem_1
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_1_Click;
			if (toolStripMenuItem_1 != null)
			{
				toolStripMenuItem_1.Click -= value2;
			}
			toolStripMenuItem_1 = value;
			if (toolStripMenuItem_1 != null)
			{
				toolStripMenuItem_1.Click += value2;
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

	internal virtual TextBox Tid
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

	internal virtual TextBox Taddress
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

	internal virtual TextBox Tother
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tother;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tother = value;
		}
	}

	internal virtual TextBox Ttel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ttel = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmSETsale()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmSETsale()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += add_seeds_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		Badd = "เพ\u0e34\u0e48ม";
		Bupdate = "บ\u0e31นท\u0e36ก";
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
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripMenuItem_1 = new System.Windows.Forms.ToolStripMenuItem();
		this.Tid = new System.Windows.Forms.TextBox();
		this.Tname = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.Taddress = new System.Windows.Forms.TextBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.Ttel = new System.Windows.Forms.TextBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.Tother = new System.Windows.Forms.TextBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.ListView1);
		this.GroupBox1.Controls.Add(this.Tid);
		this.GroupBox1.Controls.Add(this.Taddress);
		this.GroupBox1.Controls.Add(this.Tother);
		this.GroupBox1.Controls.Add(this.Ttel);
		this.GroupBox1.Controls.Add(this.Tname);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(9, 36);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(870, 478);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 12;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(19, 366);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(68, 16);
		label2.Size = size;
		this.Label1.TabIndex = 47;
		this.Label1.Text = "รห\u0e31สเซลล\u0e4c :";
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(184, 366);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(33, 16);
		label4.Size = size;
		this.Label2.TabIndex = 47;
		this.Label2.Text = "ช\u0e37\u0e48อ :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[6] { this.ColumnHeader3, this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6 });
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(14, 22);
		listView.Location = location;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(842, 326);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "id";
		this.ColumnHeader3.Width = 0;
		this.ColumnHeader1.Text = "รห\u0e31สเซลล\u0e4c";
		this.ColumnHeader1.Width = 90;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 250;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[2] { this.ToolStripMenuItem_0, this.ToolStripMenuItem_1 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(102, 48);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "ลบToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(152, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ลบ";
		this.ToolStripMenuItem_1.Name = "แกไขToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem2 = this.ToolStripMenuItem_1;
		size = new System.Drawing.Size(152, 22);
		toolStripMenuItem2.Size = size;
		this.ToolStripMenuItem_1.Text = "แก\u0e49ไข";
		this.Tid.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox tid = this.Tid;
		location = new System.Drawing.Point(93, 363);
		tid.Location = location;
		this.Tid.Name = "Tid";
		System.Windows.Forms.TextBox tid2 = this.Tid;
		size = new System.Drawing.Size(84, 23);
		tid2.Size = size;
		this.Tid.TabIndex = 1;
		this.Tname.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox tname = this.Tname;
		location = new System.Drawing.Point(223, 363);
		tname.Location = location;
		this.Tname.Name = "Tname";
		System.Windows.Forms.TextBox tname2 = this.Tname;
		size = new System.Drawing.Size(428, 23);
		tname2.Size = size;
		this.Tname.TabIndex = 2;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(673, 441);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(64, 23);
		button2.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "เพ\u0e34\u0e48ม";
		this.Button1.UseVisualStyleBackColor = true;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		size = new System.Drawing.Size(889, 32);
		panelEx2.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Blue;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Navy;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "จ\u0e31ดการเซลล\u0e4c";
		this.ColumnHeader4.Text = "เบอร\u0e4cโทร";
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader5.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.ColumnHeader5.Width = 200;
		this.ColumnHeader6.Text = "อ\u0e37\u0e48นๆ";
		this.ColumnHeader6.Width = 150;
		this.Taddress.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox taddress = this.Taddress;
		location = new System.Drawing.Point(223, 392);
		taddress.Location = location;
		this.Taddress.Multiline = true;
		this.Taddress.Name = "Taddress";
		System.Windows.Forms.TextBox taddress2 = this.Taddress;
		size = new System.Drawing.Size(428, 42);
		taddress2.Size = size;
		this.Taddress.TabIndex = 2;
		this.Label3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(175, 396);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(42, 16);
		label6.Size = size;
		this.Label3.TabIndex = 47;
		this.Label3.Text = "ท\u0e35\u0e48อย\u0e39\u0e48 :";
		this.Ttel.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox ttel = this.Ttel;
		location = new System.Drawing.Point(223, 440);
		ttel.Location = location;
		this.Ttel.Name = "Ttel";
		System.Windows.Forms.TextBox ttel2 = this.Ttel;
		size = new System.Drawing.Size(183, 23);
		ttel2.Size = size;
		this.Ttel.TabIndex = 2;
		this.Label4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(153, 444);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(64, 16);
		label8.Size = size;
		this.Label4.TabIndex = 47;
		this.Label4.Text = "เบอร\u0e4cโทร :";
		this.Tother.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox tother = this.Tother;
		location = new System.Drawing.Point(468, 440);
		tother.Location = location;
		this.Tother.Name = "Tother";
		System.Windows.Forms.TextBox tother2 = this.Tother;
		size = new System.Drawing.Size(183, 23);
		tother2.Size = size;
		this.Tother.TabIndex = 2;
		this.Label5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(424, 444);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(39, 16);
		label10.Size = size;
		this.Label5.TabIndex = 47;
		this.Label5.Text = "อ\u0e37\u0e48นๆ :";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(889, 525);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox1);
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmSETsale";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "จ\u0e31ดการเซลล\u0e4c";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void add_seeds_Load(object sender, EventArgs e)
	{
		load_type();
	}

	public void load_type()
	{
		Button1.Text = Conversions.ToString(Badd);
		Tname.Text = "";
		Tid.Text = "";
		Ttel.Text = "";
		Taddress.Text = "";
		Tother.Text = "";
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect("Select * From HT_SET_Sale order by id_full");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
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
					ListView listView = ListView1;
					int count = listView.Items.Count;
					ListView.ListViewItemCollection items = listView.Items;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow2 = dataRow;
					string columnName = "id";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow3 = dataRow;
					columnName = "id_full";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "name";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array7 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow5 = dataRow;
					columnName = "tel";
					array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
					array = array3;
					object[] arguments4 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array8 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow6 = dataRow;
					columnName = "address";
					array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
					array = array3;
					object[] arguments5 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems4, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array9 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow7 = dataRow;
					columnName = "other";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array = array3;
					object[] arguments6 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems5, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listView = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Tname.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อเซลล\u0e4c");
			return;
		}
		if (Operators.CompareString(Tid.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกรห\u0e31สเซลล\u0e4c");
			return;
		}
		if (Operators.ConditionalCompareObjectEqual(Button1.Text, Badd, TextCompare: false))
		{
			if (Module1.connect("SELECT * From HT_SET_Sale where name like '" + Tname.Text + "'").Tables[0].Rows.Count != 0)
			{
				MessageBox.Show("ม\u0e35ช\u0e37\u0e48อเซลล\u0e4cน\u0e35\u0e49อย\u0e39\u0e48แล\u0e49ว");
			}
			else
			{
				object obj = "";
				obj = "INSERT INTO [HT_SET_Sale]";
				obj = Operators.ConcatenateObject(obj, "([Name],[id_full],[tel],[address],[other])");
				obj = Operators.ConcatenateObject(obj, "VALUES");
				obj = Operators.ConcatenateObject(obj, "(");
				obj = Operators.ConcatenateObject(obj, string.Concat("'" + Tname.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Tid.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Ttel.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Taddress.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Tother.Text, "'"));
				obj = Operators.ConcatenateObject(obj, ")");
				Module1.connect(Conversions.ToString(obj));
				MessageBox.Show("เพ\u0e34\u0e48มช\u0e37\u0e48อเซลล\u0e4cเร\u0e35ยบร\u0e49อย");
			}
		}
		else
		{
			Module1.connect($"Update HT_SET_Sale SET name = '{Tname.Text}',id_full = '{Tid.Text}',[tel] = '{Ttel.Text}',[address] = '{Taddress.Text}',[other] = '{Tother.Text}' WHERE id={ListView1.SelectedItems[0].SubItems[0].Text}");
			MessageBox.Show("อ\u0e31บเดทเสร\u0e47จเร\u0e35ยบร\u0e49อย");
		}
		load_type();
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		DEL_item();
	}

	public void DEL_item()
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[2].Text, "รายช\u0e31\u0e48วโมง", TextCompare: false) == 0)
			{
				MessageBox.Show("ไม\u0e48สามารถลบรายการน\u0e35\u0e49ได\u0e49");
				return;
			}
			DataSet dataSet = Module1.connect("select id from HT_Customers where Cust_Type='" + ListView1.SelectedItems[0].SubItems[2].Text + "'");
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				MessageBox.Show("ม\u0e35รายการน\u0e35\u0e49ใช\u0e49อย\u0e39\u0e48ในทะเบ\u0e35ยนล\u0e39กค\u0e49าไม\u0e48สามารถลบได\u0e49");
			}
			else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบ " + ListView1.SelectedItems[0].SubItems[2].Text + " หร\u0e37อไม\u0e48", "ลบรายการ", MessageBoxButtons.YesNo, MessageBoxIcon.Exclamation) == DialogResult.Yes)
			{
				Module1.connect("delete from HT_SET_Sale where id=" + ListView1.SelectedItems[0].SubItems[0].Text);
				MessageBox.Show("ลบรายการเสร\u0e47จเร\u0e35ยบร\u0e49อย", "ลบรายการ", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
				load_type();
			}
		}
		else
		{
			MessageBox.Show("ไม\u0e48ม\u0e35รายการท\u0e35\u0e48เล\u0e37อก", "ลบรายการ", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
	}

	private void ToolStripMenuItem_1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[2].Text, "รายช\u0e31\u0e48วโมง", TextCompare: false) == 0)
		{
			MessageBox.Show("ไม\u0e48สามารถแก\u0e49ไขรายการน\u0e35\u0e49ได\u0e49");
		}
		else if (ListView1.SelectedItems.Count != 0)
		{
			Button1.Text = Conversions.ToString(Bupdate);
			Tid.Text = ListView1.SelectedItems[0].SubItems[1].Text;
			Tname.Text = ListView1.SelectedItems[0].SubItems[2].Text;
			Ttel.Text = ListView1.SelectedItems[0].SubItems[3].Text;
			Taddress.Text = ListView1.SelectedItems[0].SubItems[4].Text;
			Tother.Text = ListView1.SelectedItems[0].SubItems[5].Text;
		}
		else
		{
			MessageBox.Show("ไม\u0e48ม\u0e35รายการท\u0e35\u0e48เล\u0e37อก", "แก\u0e49ไขรายการ", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
	}
}
