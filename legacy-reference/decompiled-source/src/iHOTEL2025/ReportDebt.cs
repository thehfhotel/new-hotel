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

namespace iHOTEL2025;

[DesignerGenerated]
public class ReportDebt : Office2007Form
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

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	private int EditID;

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

	[DebuggerNonUserCode]
	static ReportDebt()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ReportDebt()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmPayDebt_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EditID = 0;
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
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Button2 = new System.Windows.Forms.Button();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Button2);
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
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button2;
		location = new System.Drawing.Point(935, 486);
		button.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button2 = this.Button2;
		size = new System.Drawing.Size(75, 23);
		button2.Size = size;
		this.Button2.TabIndex = 4;
		this.Button2.Text = "พ\u0e34มพ\u0e4c";
		this.Button2.UseVisualStyleBackColor = true;
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.CheckBox1);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Controls.Add(this.ComboBox2);
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
		size = new System.Drawing.Size(998, 471);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายงานล\u0e39กหน\u0e35\u0e49";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label = this.Label4;
		location = new System.Drawing.Point(408, 52);
		label.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label2 = this.Label4;
		size = new System.Drawing.Size(45, 16);
		label2.Size = size;
		this.Label4.TabIndex = 8;
		this.Label4.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48";
		this.DateTimePicker2.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker2.Enabled = false;
		this.DateTimePicker2.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(455, 48);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(186, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 6;
		this.DateTimePicker1.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker1.Enabled = false;
		this.DateTimePicker1.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(214, 48);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(176, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 7;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(141, 50);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(71, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 5;
		this.CheckBox1.Text = "จากว\u0e31นท\u0e35\u0e48";
		this.CheckBox1.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button3 = this.Button1;
		location = new System.Drawing.Point(650, 17);
		button3.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button4 = this.Button1;
		size = new System.Drawing.Size(75, 54);
		button4.Size = size;
		this.Button1.TabIndex = 4;
		this.Button1.Text = "ค\u0e49นหา";
		this.Button1.UseVisualStyleBackColor = true;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[3] { "ชำระครบ", "ชำระย\u0e31งไม\u0e48ครบ", "ท\u0e31\u0e49งหมด" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox2;
		location = new System.Drawing.Point(455, 19);
		comboBox.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox2;
		size = new System.Drawing.Size(186, 24);
		comboBox2.Size = size;
		this.ComboBox2.TabIndex = 3;
		this.ComboBox2.Text = "ชำระย\u0e31งไม\u0e48ครบ";
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox1;
		location = new System.Drawing.Point(137, 18);
		comboBox3.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox1;
		size = new System.Drawing.Size(253, 24);
		comboBox4.Size = size;
		this.ComboBox1.TabIndex = 2;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label3;
		location = new System.Drawing.Point(405, 22);
		label3.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label4 = this.Label3;
		size = new System.Drawing.Size(44, 16);
		label4.Size = size;
		this.Label3.TabIndex = 1;
		this.Label3.Text = "สถานะ";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label2;
		location = new System.Drawing.Point(81, 22);
		label5.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(54, 16);
		label6.Size = size;
		this.Label2.TabIndex = 1;
		this.Label2.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label7 = this.Label1;
		location = new System.Drawing.Point(11, 22);
		label7.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label8 = this.Label1;
		size = new System.Drawing.Size(67, 16);
		label8.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "ค\u0e49นหาตาม";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[12]
		{
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader11, this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader7, this.ColumnHeader8, this.ColumnHeader9,
			this.ColumnHeader10, this.ColumnHeader12
		});
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(7, 77);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(985, 388);
		listView2.Size = size;
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
		this.ColumnHeader11.Text = "เลขห\u0e49อง";
		this.ColumnHeader11.Width = 150;
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
		this.ColumnHeader12.Text = "หมายเหต\u0e38";
		this.ColumnHeader12.Width = 200;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1022, 516);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportDebt";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานล\u0e39กหน\u0e35\u0e49";
		this.PanelEx1.ResumeLayout(false);
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmPayDebt_Load(object sender, EventArgs e)
	{
		ListCust();
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 00:00:00");
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 23:59:59");
	}

	public void ListCust()
	{
		DataSet dataSet = Module1.connect("select Cust_name from View_Customers order by Cust_name");
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
		object left = "select * from View_CheckIn_H where Cin_status='ปกต\u0e34'";
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
		if (CheckBox1.Checked)
		{
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" and (Cin_date between '" + Conversions.ToString(DateTimePicker1.Value), "' and '"), Conversions.ToString(DateTimePicker2.Value)), "')"));
		}
		left = Operators.ConcatenateObject(left, " order by Cin_date desc");
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		ListView1.Items.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		checked
		{
			int num4 = dataSet.Tables[0].Rows.Count - 1;
			int num5 = 0;
			while (true)
			{
				int num6 = num5;
				int num7 = num4;
				if (num6 > num7)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView = ListView1;
				ListView.ListViewItemCollection items = listView.Items;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num5];
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
				listView.Items[num5].SubItems.Add(Conversions.ToString(dataSet.Tables[0].Rows.Count - num5));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[num5].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num5];
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
				listView.Items[num5].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_date"]), "dd/MM/yyyy"));
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[num5].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet.Tables[0].Rows[num5];
				DataRow dataRow4 = dataRow;
				columnName = "cin_room_all";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[num5].SubItems;
				array3 = new object[1];
				object[] array7 = array3;
				dataRow = dataSet.Tables[0].Rows[num5];
				DataRow dataRow5 = dataRow;
				columnName = "cust_name";
				array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
				array = array3;
				object[] arguments4 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[num5].SubItems;
				array3 = new object[1];
				object[] array8 = array3;
				dataRow = dataSet.Tables[0].Rows[num5];
				DataRow dataRow6 = dataRow;
				columnName = "C_address";
				array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
				array = array3;
				object[] arguments5 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems4, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[num5].SubItems;
				array3 = new object[1];
				object[] array9 = array3;
				dataRow = dataSet.Tables[0].Rows[num5];
				DataRow dataRow7 = dataRow;
				columnName = "cust_add_tel";
				array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
				array = array3;
				object[] arguments6 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems5, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[num5].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["total_price_net"]), "#,##0.00"));
				listView.Items[num5].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["total_price_pay"]), "#,##0.00"));
				listView.Items[num5].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["total_price_balance"]), "#,##0.00"));
				if (Operators.ConditionalCompareObjectLessEqual(dataSet.Tables[0].Rows[num5]["total_price_balance"], 0, TextCompare: false))
				{
					listView.Items[num5].BackColor = Color.LightGreen;
				}
				listView.Items[num5].SubItems.Add(dataSet.Tables[0].Rows[num5]["cin_note"].ToString());
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet.Tables[0].Rows[num5]["total_price_net"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet.Tables[0].Rows[num5]["total_price_pay"]));
				num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet.Tables[0].Rows[num5]["total_price_balance"]));
				listView = null;
				num5++;
			}
			global::PrintableListView.PrintableListView listView2 = ListView1;
			int count = listView2.Items.Count;
			listView2.Items.Add("");
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].SubItems.Add("รวม");
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].SubItems.Add(Strings.Format(num, "#,##0.00"));
			listView2.Items[count].SubItems.Add(Strings.Format(num2, "#,##0.00"));
			listView2.Items[count].SubItems.Add(Strings.Format(num3, "#,##0.00"));
			listView2.Items[count].SubItems.Add("");
			listView2.Items[count].BackColor = Color.LightPink;
			listView2 = null;
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

	private void Button1_Click(object sender, EventArgs e)
	{
		SearchDebt();
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		if (CheckBox1.Checked)
		{
			ListView1.Title = "รายงานล\u0e39กหน\u0e35\u0e49 สถานะ " + ComboBox2.Text + " ระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy") + " ถ\u0e36งว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker2.Value, "dd/MM/yy");
		}
		else
		{
			ListView1.Title = "รายงานล\u0e39กหน\u0e35\u0e49 สถานะ " + ComboBox2.Text;
		}
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
		DateTimePicker1.Enabled = CheckBox1.Checked;
		DateTimePicker2.Enabled = CheckBox1.Checked;
	}
}
