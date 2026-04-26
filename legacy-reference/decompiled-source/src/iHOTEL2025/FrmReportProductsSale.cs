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
public class FrmReportProductsSale : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

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

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("Search_type")]
	private ComboBox _Search_type;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("คนหาสนคาชอToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

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
			_DateTimePicker2 = value;
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
			_DateTimePicker1 = value;
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

	internal virtual TextBox TextBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox1 = value;
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

	internal virtual ComboBox Search_type
	{
		[DebuggerNonUserCode]
		get
		{
			return _Search_type;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Search_type = value;
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

	[DebuggerNonUserCode]
	static FrmReportProductsSale()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportProductsSale()
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
		this.components = new System.ComponentModel.Container();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Search_type = new System.Windows.Forms.ComboBox();
		this.Label12 = new System.Windows.Forms.Label();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Button3 = new System.Windows.Forms.Button();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.Search_type);
		this.GroupBox1.Controls.Add(this.Label12);
		this.GroupBox1.Controls.Add(this.TextBox1);
		this.GroupBox1.Controls.Add(this.Button3);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label1);
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
		this.GroupBox1.Text = "รายงานการขายส\u0e34นค\u0e49า";
		this.Search_type.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Search_type.FormattingEnabled = true;
		System.Windows.Forms.ComboBox search_type = this.Search_type;
		location = new System.Drawing.Point(365, 52);
		search_type.Location = location;
		this.Search_type.Name = "Search_type";
		System.Windows.Forms.ComboBox search_type2 = this.Search_type;
		size = new System.Drawing.Size(272, 24);
		search_type2.Size = size;
		this.Search_type.TabIndex = 12;
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label = this.Label12;
		location = new System.Drawing.Point(268, 57);
		label.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label2 = this.Label12;
		size = new System.Drawing.Size(96, 17);
		label2.Size = size;
		this.Label12.TabIndex = 13;
		this.Label12.Text = "ประเภทส\u0e34นค\u0e49า :";
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		location = new System.Drawing.Point(86, 53);
		textBox.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		size = new System.Drawing.Size(176, 24);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 6;
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(553, 24);
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
		location = new System.Drawing.Point(365, 25);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(176, 24);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 4;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(306, 28);
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
		location = new System.Drawing.Point(86, 25);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(176, 24);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 5;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(35, 57);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(51, 17);
		label6.Size = size;
		this.Label3.TabIndex = 2;
		this.Label3.Text = "ส\u0e34นค\u0e49า :";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label1;
		location = new System.Drawing.Point(21, 28);
		label7.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label8 = this.Label1;
		size = new System.Drawing.Size(66, 17);
		label8.Size = size;
		this.Label1.TabIndex = 2;
		this.Label1.Text = "จากว\u0e31นท\u0e35\u0e48 :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[9] { this.ColumnHeader1, this.ColumnHeader3, this.ColumnHeader8, this.ColumnHeader2, this.ColumnHeader7, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader9 });
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(6, 82);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(1033, 395);
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
		this.ColumnHeader3.Text = "ห\u0e49อง";
		this.ColumnHeader3.Width = 80;
		this.ColumnHeader8.Text = "ว\u0e31นท\u0e35\u0e48ทำรายการ";
		this.ColumnHeader8.Width = 120;
		this.ColumnHeader2.Text = "รห\u0e31ส";
		this.ColumnHeader2.Width = 130;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.ColumnHeader7.Width = 280;
		this.ColumnHeader4.Text = "ราคา";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 80;
		this.ColumnHeader5.Text = "จำนวน";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 90;
		this.ColumnHeader6.Text = "ราคารวม";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader6.Width = 90;
		this.ColumnHeader9.Text = "จำนวนคงเหล\u0e37อ";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 100;
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
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[1] { this.ToolStripMenuItem_0 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(157, 26);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "คนหาสนคาชอToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(156, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ค\u0e49นหา ส\u0e34นค\u0e49าช\u0e37\u0e48อ";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1073, 539);
		this.ClientSize = size;
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.GroupBox1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportProductsSale";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานการขายส\u0e34นค\u0e49า";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		search();
	}

	public void search()
	{
		ToolStripMenuItem_0.Visible = false;
		object left = "SELECT * from HT_CheckIn_Product";
		left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" where (cin_ds_date between '" + Conversions.ToString(DateTimePicker1.Value), "' and '"), Conversions.ToString(DateTimePicker2.Value)), "')"));
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and cin_pro_Name like '%" + TextBox1.Text, "%' "));
		}
		if (Operators.CompareString(Search_type.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and cin_pro_id in ( select Pro_no from HT_Products where Pro_Type = '" + Search_type.Text, "' )"));
		}
		left = Operators.ConcatenateObject(left, " order by cin_ds_date");
		object left2 = "SELECT * from View_Pay_Ds";
		left2 = Operators.ConcatenateObject(left2, string.Concat(string.Concat(string.Concat(" where (cin_pay_date between '" + Conversions.ToString(DateTimePicker1.Value), "' and '"), Conversions.ToString(DateTimePicker2.Value)), "')"));
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0)
		{
			left2 = Operators.ConcatenateObject(left2, string.Concat(" and cin_pay_ds_name like '%" + TextBox1.Text, "%' "));
		}
		if (Operators.CompareString(Search_type.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			left2 = Operators.ConcatenateObject(left2, string.Concat(" and cin_pay_ds_id in ( select Pro_no from HT_Products where Pro_Type = '" + Search_type.Text, "' )"));
		}
		left2 = Operators.ConcatenateObject(left2, " and Cin_status<>'ยกเล\u0e34ก' and cin_pay_note='รายการไม\u0e48อ\u0e49างอ\u0e34งใบลงทะเบ\u0e35ยน'");
		left2 = Operators.ConcatenateObject(left2, " order by cin_pay_date");
		object left3 = "SELECT * from View_Bill_Debt_Ds";
		left3 = Operators.ConcatenateObject(left3, string.Concat(string.Concat(string.Concat(" where (bill_date between '" + Conversions.ToString(DateTimePicker1.Value), "' and '"), Conversions.ToString(DateTimePicker2.Value)), "')"));
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0)
		{
			left3 = Operators.ConcatenateObject(left3, string.Concat(" and ds_name like '%" + TextBox1.Text, "%' "));
		}
		if (Operators.CompareString(Search_type.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			left3 = Operators.ConcatenateObject(left3, string.Concat(" and DS_NO in ( select Pro_no from HT_Products where Pro_Type = '" + Search_type.Text, "' )"));
		}
		left3 = Operators.ConcatenateObject(left3, " and bill_status<>'ยกเล\u0e34ก'");
		left3 = Operators.ConcatenateObject(left3, " order by bill_date");
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		DataSet dataSet2 = Module1.connect(Conversions.ToString(left2));
		DataSet dataSet3 = Module1.connect(Conversions.ToString(left3));
		ListView1.Items.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		DataSet dataSet4 = Module1.connect("select * from HT_Products ");
		checked
		{
			int num3 = dataSet.Tables[0].Rows.Count - 1;
			int num4 = 0;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 > num6)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView = ListView1;
				int count = listView.Items.Count;
				listView.Items.Add(Conversions.ToString(count + 1));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num4];
				DataRow dataRow2 = dataRow;
				string columnName = "cin_room_no";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["cin_ds_date"]), "dd/MM/yy HH:mm"));
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num4];
				DataRow dataRow3 = dataRow;
				columnName = "cin_pro_id";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet.Tables[0].Rows[num4];
				DataRow dataRow4 = dataRow;
				columnName = "cin_pro_Name";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array7 = array3;
				dataRow = dataSet.Tables[0].Rows[num4];
				DataRow dataRow5 = dataRow;
				columnName = "cin_pro_price";
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
				dataRow = dataSet.Tables[0].Rows[num4];
				DataRow dataRow6 = dataRow;
				columnName = "cin_pro_num";
				array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
				array = array3;
				object[] arguments5 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems5, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems6 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array9 = array3;
				dataRow = dataSet.Tables[0].Rows[num4];
				DataRow dataRow7 = dataRow;
				columnName = "cin_pro_priceTotal";
				array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
				array = array3;
				object[] arguments6 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems6, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				decimal num7 = default(decimal);
				int num8 = dataSet4.Tables[0].Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					num6 = num8;
					if (num10 <= num6)
					{
						if (!Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num9]["pro_no"], dataSet.Tables[0].Rows[num4]["cin_pro_id"], TextCompare: false))
						{
							num9++;
							continue;
						}
						num7 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num9]["pro_amt"]);
						break;
					}
					break;
				}
				listView.Items[count].SubItems.Add(Conversions.ToString(num7));
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet.Tables[0].Rows[num4]["cin_pro_num"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet.Tables[0].Rows[num4]["cin_pro_priceTotal"]));
				listView = null;
				num4++;
			}
			int num11 = dataSet2.Tables[0].Rows.Count - 1;
			int num12 = 0;
			while (true)
			{
				int num13 = num12;
				int num6 = num11;
				if (num13 > num6)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView2 = ListView1;
				int count2 = listView2.Items.Count;
				listView2.Items.Add(Conversions.ToString(count2 + 1));
				listView2.Items[count2].SubItems.Add("หน\u0e49าฟร\u0e49อน");
				listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num12]["cin_pay_date"]), "dd/MM/yy HH:mm"));
				ListViewItem.ListViewSubItemCollection subItems7 = listView2.Items[count2].SubItems;
				object[] array3 = new object[1];
				object[] array10 = array3;
				DataRow dataRow = dataSet2.Tables[0].Rows[num12];
				DataRow dataRow8 = dataRow;
				string columnName = "cin_pay_ds_id";
				array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
				object[] array = array3;
				object[] arguments7 = array;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems7, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems8 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array11 = array3;
				dataRow = dataSet2.Tables[0].Rows[num12];
				DataRow dataRow9 = dataRow;
				columnName = "cin_pay_ds_name";
				array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
				array = array3;
				object[] arguments8 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems8, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems9 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array12 = array3;
				dataRow = dataSet2.Tables[0].Rows[num12];
				DataRow dataRow10 = dataRow;
				columnName = "cin_pay_ds_priceOne";
				array12[0] = RuntimeHelpers.GetObjectValue(dataRow10[columnName]);
				array = array3;
				object[] arguments9 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems9, null, "Add", arguments9, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems10 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array13 = array3;
				dataRow = dataSet2.Tables[0].Rows[num12];
				DataRow dataRow11 = dataRow;
				columnName = "cin_pay_ds_num";
				array13[0] = RuntimeHelpers.GetObjectValue(dataRow11[columnName]);
				array = array3;
				object[] arguments10 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems10, null, "Add", arguments10, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems11 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array14 = array3;
				dataRow = dataSet2.Tables[0].Rows[num12];
				DataRow dataRow12 = dataRow;
				columnName = "cin_pay_ds_priceTotal";
				array14[0] = RuntimeHelpers.GetObjectValue(dataRow12[columnName]);
				array = array3;
				object[] arguments11 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems11, null, "Add", arguments11, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView2.Items[count2].BackColor = Color.LightCyan;
				decimal num14 = default(decimal);
				int num15 = dataSet4.Tables[0].Rows.Count - 1;
				int num16 = 0;
				while (true)
				{
					int num17 = num16;
					num6 = num15;
					if (num17 <= num6)
					{
						if (!Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num16]["pro_no"], dataSet2.Tables[0].Rows[num12]["cin_pay_ds_id"], TextCompare: false))
						{
							num16++;
							continue;
						}
						num14 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num16]["pro_amt"]);
						break;
					}
					break;
				}
				listView2.Items[count2].SubItems.Add(Conversions.ToString(num14));
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num12]["cin_pay_ds_num"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num12]["cin_pay_ds_priceTotal"]));
				listView2 = null;
				num12++;
			}
			int num18 = dataSet3.Tables[0].Rows.Count - 1;
			int num19 = 0;
			while (true)
			{
				int num20 = num19;
				int num6 = num18;
				if (num20 > num6)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView3 = ListView1;
				int count3 = listView3.Items.Count;
				listView3.Items.Add(Conversions.ToString(count3 + 1));
				ListViewItem.ListViewSubItemCollection subItems12 = listView3.Items[count3].SubItems;
				object[] array3 = new object[1];
				object[] array15 = array3;
				DataRow dataRow = dataSet3.Tables[0].Rows[num19];
				DataRow dataRow13 = dataRow;
				string columnName = "bill_no";
				array15[0] = RuntimeHelpers.GetObjectValue(dataRow13[columnName]);
				object[] array = array3;
				object[] arguments12 = array;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems12, null, "Add", arguments12, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView3.Items[count3].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num19]["bill_date"]), "dd/MM/yy HH:mm"));
				ListViewItem.ListViewSubItemCollection subItems13 = listView3.Items[count3].SubItems;
				array3 = new object[1];
				object[] array16 = array3;
				dataRow = dataSet3.Tables[0].Rows[num19];
				DataRow dataRow14 = dataRow;
				columnName = "DS_NO";
				array16[0] = RuntimeHelpers.GetObjectValue(dataRow14[columnName]);
				array = array3;
				object[] arguments13 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems13, null, "Add", arguments13, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems14 = listView3.Items[count3].SubItems;
				array3 = new object[1];
				object[] array17 = array3;
				dataRow = dataSet3.Tables[0].Rows[num19];
				DataRow dataRow15 = dataRow;
				columnName = "DS_NAME";
				array17[0] = RuntimeHelpers.GetObjectValue(dataRow15[columnName]);
				array = array3;
				object[] arguments14 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems14, null, "Add", arguments14, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems15 = listView3.Items[count3].SubItems;
				array3 = new object[1];
				object[] array18 = array3;
				dataRow = dataSet3.Tables[0].Rows[num19];
				DataRow dataRow16 = dataRow;
				columnName = "DS_PRICE";
				array18[0] = RuntimeHelpers.GetObjectValue(dataRow16[columnName]);
				array = array3;
				object[] arguments15 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems15, null, "Add", arguments15, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems16 = listView3.Items[count3].SubItems;
				array3 = new object[1];
				object[] array19 = array3;
				dataRow = dataSet3.Tables[0].Rows[num19];
				DataRow dataRow17 = dataRow;
				columnName = "DS_NUM";
				array19[0] = RuntimeHelpers.GetObjectValue(dataRow17[columnName]);
				array = array3;
				object[] arguments16 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems16, null, "Add", arguments16, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems17 = listView3.Items[count3].SubItems;
				array3 = new object[1];
				object[] array20 = array3;
				dataRow = dataSet3.Tables[0].Rows[num19];
				DataRow dataRow18 = dataRow;
				columnName = "DS_PRICE_TOTAL";
				array20[0] = RuntimeHelpers.GetObjectValue(dataRow18[columnName]);
				array = array3;
				object[] arguments17 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems17, null, "Add", arguments17, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				decimal num21 = default(decimal);
				int num22 = dataSet4.Tables[0].Rows.Count - 1;
				int num23 = 0;
				while (true)
				{
					int num24 = num23;
					num6 = num22;
					if (num24 <= num6)
					{
						if (!Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num23]["pro_no"], dataSet3.Tables[0].Rows[num19]["DS_NO"], TextCompare: false))
						{
							num23++;
							continue;
						}
						num21 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num23]["pro_amt"]);
						break;
					}
					break;
				}
				listView3.Items[count3].SubItems.Add(Conversions.ToString(num21));
				listView3.Items[count3].BackColor = Color.LightCyan;
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet3.Tables[0].Rows[num19]["DS_NUM"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet3.Tables[0].Rows[num19]["DS_PRICE_TOTAL"]));
				listView3 = null;
				num19++;
			}
			global::PrintableListView.PrintableListView listView4 = ListView1;
			int count4 = listView4.Items.Count;
			listView4.Items.Add("");
			listView4.Items[count4].SubItems.Add("รวม");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num));
			listView4.Items[count4].SubItems.Add(Conversions.ToString(num2));
			listView4.Items[count4].SubItems.Add("");
			listView4.Items[count4].BackColor = Color.LightPink;
			listView4 = null;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Search_type.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			ListView1.Title = "รายงานการขายส\u0e34นค\u0e49า (" + Search_type.Text + ") \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		}
		else
		{
			ListView1.Title = "รายงานการขายส\u0e34นค\u0e49า \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		}
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 00:00:00");
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 23:59:59");
		Listtype();
		search();
	}

	public void Listtype()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_ProductType order by name");
		Search_type.Items.Clear();
		Search_type.Items.Add("ท\u0e31\u0e49งหมด");
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
				Search_type.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			Search_type.SelectedIndex = 0;
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			ToolStripMenuItem_0.Visible = true;
			ToolStripMenuItem_0.Text = "ค\u0e49นหาส\u0e34นค\u0e49า ช\u0e37\u0e48อ " + ListView1.SelectedItems[0].SubItems[4].Text;
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		TextBox1.Text = ToolStripMenuItem_0.Text.Replace("ค\u0e49นหาส\u0e34นค\u0e49า ช\u0e37\u0e48อ ", "");
		Button3_Click(null, null);
	}
}
