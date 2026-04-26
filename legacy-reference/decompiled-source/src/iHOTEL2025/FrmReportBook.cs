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

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmReportBook : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

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
			EventHandler value2 = DateTimePicker1_ValueChanged;
			if (_DateTimePicker2 != null)
			{
				_DateTimePicker2.ValueChanged -= value2;
			}
			_DateTimePicker2 = value;
			if (_DateTimePicker2 != null)
			{
				_DateTimePicker2.ValueChanged += value2;
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
			_ComboBox1 = value;
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
			_ComboBox2 = value;
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

	[DebuggerNonUserCode]
	static FrmReportBook()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportBook()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmReportImcome_Load;
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
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.Button3 = new System.Windows.Forms.Button();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label1 = new System.Windows.Forms.Label();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label4 = new System.Windows.Forms.Label();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.ComboBox2);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.ListView1);
		this.GroupBox1.Controls.Add(this.Button3);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.Label1);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(994, 483);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายงานการจองห\u0e49องพ\u0e31ก";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "ท\u0e31\u0e49งหมด", "ปกต\u0e34", "ยกเล\u0e34ก" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(639, 26);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(121, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 18;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label = this.Label3;
		location = new System.Drawing.Point(581, 28);
		label.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label2 = this.Label3;
		size = new System.Drawing.Size(56, 17);
		label2.Size = size;
		this.Label3.TabIndex = 17;
		this.Label3.Text = "สถานะ :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[15]
		{
			this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader9, this.ColumnHeader14, this.ColumnHeader12, this.ColumnHeader13, this.ColumnHeader5, this.ColumnHeader7, this.ColumnHeader10, this.ColumnHeader11,
			this.ColumnHeader2, this.ColumnHeader15, this.ColumnHeader1, this.ColumnHeader8, this.ColumnHeader6
		});
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(15, 80);
		listView.Location = location;
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView3 = this.ListView1;
		size = new System.Drawing.Size(961, 380);
		listView3.Size = size;
		this.ListView1.TabIndex = 16;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "ท\u0e35\u0e48";
		this.ColumnHeader3.Width = 40;
		this.ColumnHeader4.Text = "รห\u0e31ส";
		this.ColumnHeader4.Width = 80;
		this.ColumnHeader9.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader9.Width = 150;
		this.ColumnHeader14.Text = "โทร";
		this.ColumnHeader14.Width = 100;
		this.ColumnHeader12.Text = "ว\u0e31นท\u0e35\u0e48ทำรายการ";
		this.ColumnHeader12.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader12.Width = 110;
		this.ColumnHeader13.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก";
		this.ColumnHeader13.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader13.Width = 250;
		this.ColumnHeader5.Text = "ประเภทห\u0e49อง";
		this.ColumnHeader5.Width = 120;
		this.ColumnHeader7.Text = "จำนวนห\u0e49อง";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 70;
		this.ColumnHeader10.Text = "จำนวนค\u0e37น";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 70;
		this.ColumnHeader11.Text = "เรทห\u0e49อง";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 80;
		this.ColumnHeader2.Text = "ราคา";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader2.Width = 80;
		this.ColumnHeader15.Text = "จ\u0e48ายล\u0e48วงหน\u0e49า";
		this.ColumnHeader15.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader15.Width = 80;
		this.ColumnHeader1.Text = "สถานะ";
		this.ColumnHeader1.Width = 80;
		this.ColumnHeader8.Text = "ผ\u0e39\u0e49จ\u0e31ดทำ";
		this.ColumnHeader8.Width = 80;
		this.ColumnHeader6.Text = "หมายเหต\u0e38";
		this.ColumnHeader6.Width = 150;
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(768, 25);
		button.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button2 = this.Button3;
		size = new System.Drawing.Size(84, 25);
		button2.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "ค\u0e49นหา";
		this.Button3.UseVisualStyleBackColor = true;
		this.DateTimePicker2.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker2.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(399, 25);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(176, 24);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 4;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(340, 28);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(58, 17);
		label4.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		this.DateTimePicker1.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker1.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(150, 25);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(176, 24);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 5;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(14, 28);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(135, 17);
		label6.Size = size;
		this.Label1.TabIndex = 2;
		this.Label1.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49าพ\u0e31ก จากว\u0e31นท\u0e35\u0e48 :";
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(923, 503);
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
		location = new System.Drawing.Point(833, 503);
		button5.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button6 = this.Button1;
		size = new System.Drawing.Size(84, 23);
		button6.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "พ\u0e34มพ\u0e4c";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(67, 56);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(82, 17);
		label8.Size = size;
		this.Label4.TabIndex = 2;
		this.Label4.Text = "ช\u0e37\u0e48อผ\u0e39\u0e49เข\u0e49าพ\u0e31ก :";
		this.ComboBox2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[3] { "ท\u0e31\u0e49งหมด", "ปกต\u0e34", "ยกเล\u0e34ก" });
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox2;
		location = new System.Drawing.Point(150, 52);
		comboBox3.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox2;
		size = new System.Drawing.Size(610, 24);
		comboBox4.Size = size;
		this.ComboBox2.TabIndex = 19;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1019, 539);
		this.ClientSize = size;
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.GroupBox1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportBook";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานการจองห\u0e49องพ\u0e31ก";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		search();
	}

	public void search()
	{
		object left = "SELECT     dbo.HT_Book_H.Book_ID, dbo.HT_Book_H.Book_Date, dbo.HT_Book_H.Book_Date_in, dbo.HT_Book_H.Book_Date_out, dbo.HT_Book_H.Book_Cust_ID, dbo.HT_Book_H.Book_Cust_Name, dbo.HT_Book_H.Book_Cust_Name2, dbo.HT_Book_H.Book_Cust_Tel, dbo.HT_Book_H.Book_Price_Total, dbo.HT_Book_H.Book_Price_Pay, dbo.HT_Book_H.Book_Status, dbo.HT_Book_H.Book_by,dbo.HT_Book_H.Book_room_all, dbo.HT_Book_H.Book_room_note, dbo.HT_Book_Ds.id, dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Type, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, dbo.HT_Book_Ds.Book_Room_Num, dbo.HT_Book_Ds.Book_Room_PriceToTal,dbo.HT_Book_Ds.Book_Room_Note AS Expr1 FROM dbo.HT_Book_H INNER JOIN dbo.HT_Book_Ds ON dbo.HT_Book_H.Book_ID = dbo.HT_Book_Ds.Book_No";
		left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" where (dbo.HT_Book_Ds.Book_room_start between '" + Conversions.ToString(DateTimePicker1.Value), "' and '"), Conversions.ToString(DateTimePicker2.Value)), "')"));
		if (ComboBox1.SelectedIndex == 1)
		{
			left = Operators.ConcatenateObject(left, " and HT_Book_H.Book_Status<>'ยกเล\u0e34ก'");
		}
		else if (ComboBox1.SelectedIndex == 2)
		{
			left = Operators.ConcatenateObject(left, " and HT_Book_H.Book_Status='ยกเล\u0e34ก'");
		}
		left = Operators.ConcatenateObject(left, " order by Book_Date_in,Book_room_type");
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		int num = 0;
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		int num4 = 0;
		decimal num5 = default(decimal);
		checked
		{
			int num6 = dataSet.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num9 = num6;
				if (num8 > num9)
				{
					break;
				}
				if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) == 0 || Operators.CompareString(ComboBox2.Text, dataSet.Tables[0].Rows[num7]["Book_Cust_Name"].ToString() + " " + dataSet.Tables[0].Rows[num7]["Book_Cust_Name2"].ToString(), TextCompare: false) == 0)
				{
					num = Conversions.ToInteger(Operators.AddObject(num, dataSet.Tables[0].Rows[num7]["Book_room_num"]));
					num2 = Conversions.ToDecimal(Operators.AddObject(num2, Operators.MultiplyObject(Operators.MultiplyObject(dataSet.Tables[0].Rows[num7]["Book_room_num"], dataSet.Tables[0].Rows[num7]["Book_room_night"]), dataSet.Tables[0].Rows[num7]["Book_room_price"])));
					num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet.Tables[0].Rows[num7]["Book_Price_pay"]));
					num4 = ListView1.Items.Count;
					ListView1.Items.Add(Conversions.ToString(dataSet.Tables[0].Rows.Count - num7));
					ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[num4].SubItems;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num7];
					DataRow dataRow2 = dataRow;
					string columnName = "Book_ID";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListView1.Items[num4].SubItems.Add(dataSet.Tables[0].Rows[num7]["Book_Cust_Name"].ToString() + " " + dataSet.Tables[0].Rows[num7]["Book_Cust_Name2"].ToString());
					ListViewItem.ListViewSubItemCollection subItems2 = ListView1.Items[num4].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num7];
					DataRow dataRow3 = dataRow;
					columnName = "Book_Cust_Tel";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListView1.Items[num4].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num7]["Book_Date"]), "dd/MM/yy HH:mm"));
					ListView1.Items[num4].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num7]["Book_room_start"]), "[dd-MM-yy HH:mm]") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num7]["Book_room_end"]), "[dd-MM-yy HH:mm]"));
					ListView1.Items[num4].SubItems.Add(dataSet.Tables[0].Rows[num7]["Book_room_type"].ToString());
					ListView1.Items[num4].SubItems.Add(dataSet.Tables[0].Rows[num7]["Book_room_num"].ToString());
					ListView1.Items[num4].SubItems.Add(dataSet.Tables[0].Rows[num7]["Book_room_night"].ToString());
					ListView1.Items[num4].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num7]["Book_room_price"]), "#,##0.00"));
					ListView1.Items[num4].SubItems.Add(Strings.Format(Operators.MultiplyObject(Operators.MultiplyObject(dataSet.Tables[0].Rows[num7]["Book_room_num"], dataSet.Tables[0].Rows[num7]["Book_room_night"]), dataSet.Tables[0].Rows[num7]["Book_room_price"]), "#,##0.00"));
					ListView1.Items[num4].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num7]["Book_Price_pay"]), "#,##0.00"));
					ListViewItem.ListViewSubItemCollection subItems3 = ListView1.Items[num4].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num7];
					DataRow dataRow4 = dataRow;
					columnName = "Book_Status";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListView1.Items[num4].SubItems.Add(dataSet.Tables[0].Rows[num7]["Book_by"].ToString());
					ListView1.Items[num4].SubItems.Add(dataSet.Tables[0].Rows[num7]["Book_room_note"].ToString());
				}
				num7++;
			}
			num4 = ListView1.Items.Count;
			ListView1.Items.Add("");
			ListView1.Items[num4].SubItems.Add("รวม");
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add(Conversions.ToString(num));
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add(Strings.Format(num2, "#,##0.00"));
			ListView1.Items[num4].SubItems.Add(Strings.Format(num3, "#,##0.00"));
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].BackColor = Color.LightGreen;
			ListView1.Items[num4].SubItems.Add("");
			ListView1.Items[num4].SubItems.Add("");
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		ListView1.Title = "รายงานการจองห\u0e49องพ\u0e31ก \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) != 0)
		{
			ListView1.Title = "รายงานการจองห\u0e49องพ\u0e31ก  (" + ComboBox2.Text + ")\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		}
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		ComboBox1.SelectedIndex = 1;
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 00:00:00");
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 23:59:59");
		listname();
		search();
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		listname();
	}

	public void listname()
	{
		object left = "SELECT     dbo.HT_Book_H.Book_Cust_Name, dbo.HT_Book_H.Book_Cust_Name2 FROM dbo.HT_Book_H INNER JOIN dbo.HT_Book_Ds ON dbo.HT_Book_H.Book_ID = dbo.HT_Book_Ds.Book_No";
		left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" where (dbo.HT_Book_Ds.Book_room_start between '" + Conversions.ToString(DateTimePicker1.Value), "' and '"), Conversions.ToString(DateTimePicker2.Value)), "')"));
		if (ComboBox1.SelectedIndex == 1)
		{
			left = Operators.ConcatenateObject(left, " and HT_Book_H.Book_Status<>'ยกเล\u0e34ก'");
		}
		else if (ComboBox1.SelectedIndex == 2)
		{
			left = Operators.ConcatenateObject(left, " and HT_Book_H.Book_Status='ยกเล\u0e34ก'");
		}
		left = Operators.ConcatenateObject(left, " group by Book_Cust_Name,Book_Cust_Name2 order by Book_Cust_Name");
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		ComboBox2.Items.Clear();
		ComboBox2.Items.Add("");
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
					ComboBox2.Items.Add(dataSet.Tables[0].Rows[num2]["Book_Cust_Name"].ToString() + " " + dataSet.Tables[0].Rows[num2]["Book_Cust_Name2"].ToString());
					num2++;
					continue;
				}
				break;
			}
		}
	}
}
