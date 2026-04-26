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
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormReportAll : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ListView3")]
	private ListView _ListView3;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

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

	internal virtual ListView ListView2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ListView2 = value;
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

	[DebuggerNonUserCode]
	static FormReportAll()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormReportAll()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
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
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label1 = new System.Windows.Forms.Label();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label3 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.Label4 = new System.Windows.Forms.Label();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.Label5 = new System.Windows.Forms.Label();
		this.ListView3 = new System.Windows.Forms.ListView();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Button2 = new System.Windows.Forms.Button();
		this.SuspendLayout();
		this.DateTimePicker2.CustomFormat = "ddMMMMyy HH:mm";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		System.Drawing.Point location = new System.Drawing.Point(90, 45);
		dateTimePicker.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker2.Margin = margin;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker2;
		System.Drawing.Size size = new System.Drawing.Size(205, 23);
		dateTimePicker3.Size = size;
		this.DateTimePicker2.TabIndex = 8;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label = this.Label2;
		location = new System.Drawing.Point(33, 49);
		label.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label2 = this.Label2;
		size = new System.Drawing.Size(54, 16);
		label2.Size = size;
		this.Label2.TabIndex = 7;
		this.Label2.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		this.DateTimePicker1.CustomFormat = "ddMMMMyy HH:mm";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		location = new System.Drawing.Point(90, 13);
		dateTimePicker4.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker5 = this.DateTimePicker1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker5.Margin = margin;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker6 = this.DateTimePicker1;
		size = new System.Drawing.Size(205, 23);
		dateTimePicker6.Size = size;
		this.DateTimePicker1.TabIndex = 9;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label1;
		location = new System.Drawing.Point(27, 17);
		label3.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label4 = this.Label1;
		size = new System.Drawing.Size(61, 16);
		label4.Size = size;
		this.Label1.TabIndex = 6;
		this.Label1.Text = "จากว\u0e31นท\u0e35\u0e48 :";
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(328, 27);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(89, 29);
		button2.Size = size;
		this.Button1.TabIndex = 10;
		this.Button1.Text = "ค\u0e49นหา";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(12, 94);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(79, 16);
		label6.Size = size;
		this.Label3.TabIndex = 7;
		this.Label3.Text = "รายการห\u0e49อง :";
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[4] { this.ColumnHeader1, this.ColumnHeader4, this.ColumnHeader2, this.ColumnHeader3 });
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(15, 115);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(523, 150);
		listView2.Size = size;
		this.ListView1.TabIndex = 11;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader1.Width = 200;
		this.ColumnHeader4.Text = "รายว\u0e31น";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader2.Text = "รายเด\u0e37อน";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader3.Text = "รายป\u0e35";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader3.Width = 100;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(12, 268);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(110, 16);
		label8.Size = size;
		this.Label4.TabIndex = 7;
		this.Label4.Text = "ส\u0e34นค\u0e49าตามประเภท :";
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[4] { this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader7, this.ColumnHeader8 });
		System.Windows.Forms.ListView listView3 = this.ListView2;
		location = new System.Drawing.Point(15, 289);
		listView3.Location = location;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView4 = this.ListView2;
		size = new System.Drawing.Size(523, 97);
		listView4.Size = size;
		this.ListView2.TabIndex = 11;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader5.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader5.Width = 200;
		this.ColumnHeader6.Text = "รายว\u0e31น";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader6.Width = 100;
		this.ColumnHeader7.Text = "รายเด\u0e37อน";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 100;
		this.ColumnHeader8.Text = "รายป\u0e35";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Width = 100;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(12, 399);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(49, 16);
		label10.Size = size;
		this.Label5.TabIndex = 7;
		this.Label5.Text = "ล\u0e39กหน\u0e35\u0e49 :";
		this.ListView3.Columns.AddRange(new System.Windows.Forms.ColumnHeader[4] { this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader11, this.ColumnHeader12 });
		System.Windows.Forms.ListView listView5 = this.ListView3;
		location = new System.Drawing.Point(15, 420);
		listView5.Location = location;
		this.ListView3.Name = "ListView3";
		System.Windows.Forms.ListView listView6 = this.ListView3;
		size = new System.Drawing.Size(523, 97);
		listView6.Size = size;
		this.ListView3.TabIndex = 11;
		this.ListView3.UseCompatibleStateImageBehavior = false;
		this.ListView3.View = System.Windows.Forms.View.Details;
		this.ColumnHeader9.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader9.Width = 200;
		this.ColumnHeader10.Text = "รายว\u0e31น";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 100;
		this.ColumnHeader11.Text = "รายเด\u0e37อน";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 100;
		this.ColumnHeader12.Text = "รายป\u0e35";
		this.ColumnHeader12.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader12.Width = 100;
		this.PanelEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(12, 198);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(533, 108);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 12;
		this.PanelEx1.Text = "กำล\u0e31งคำนวนอาจใช\u0e49เวลาส\u0e31กคร\u0e38\u0e48....";
		this.PanelEx1.Visible = false;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(449, 537);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(89, 29);
		button4.Size = size;
		this.Button2.TabIndex = 13;
		this.Button2.Text = "พ\u0e34มพ\u0e4cรายงาน";
		this.Button2.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(557, 579);
		this.ClientSize = size;
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.PanelEx1);
		this.Controls.Add(this.ListView3);
		this.Controls.Add(this.ListView2);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Label5);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Label4);
		this.Controls.Add(this.DateTimePicker2);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormReportAll";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานสร\u0e38ปภาพรวม";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		PanelEx1.Visible = true;
		Application.DoEvents();
		ListView1.Items.Clear();
		ListView2.Items.Clear();
		ListView3.Items.Clear();
		Search_Room();
		Search_Products();
		Search_Debt();
		PanelEx1.Visible = false;
		Application.DoEvents();
	}

	public void Search_Room()
	{
		DataSet dataSet = null;
		DataSet dataSet2 = null;
		DataSet dataSet3 = null;
		dataSet = Module1.connect("select count(id) as num from view_CheckIn_Ds where (cin_date between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
		dataSet2 = Module1.connect("select count(id) as num from view_CheckIn_Ds where (cin_date between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
		dataSet3 = Module1.connect("select count(id) as num from view_CheckIn_Ds where (cin_date between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
		ListView listView = ListView1;
		listView.Items.Add("การเช\u0e47คอ\u0e34น");
		checked
		{
			ListViewItem.ListViewSubItemCollection subItems = listView.Items[listView.Items.Count - 1].SubItems;
			object[] array = new object[1];
			object[] array2 = array;
			DataRow dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow2 = dataRow;
			string columnName = "num";
			array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
			object[] array3 = array;
			object[] arguments = array3;
			bool[] array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[listView.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array5 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow3 = dataRow;
			columnName = "num";
			array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
			array = array3;
			object[] arguments2 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[listView.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array6 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow4 = dataRow;
			columnName = "num";
			array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
			array = array3;
			object[] arguments3 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView = null;
			dataSet = Module1.connect("select count(id) as num from HT_Rooms_Cancel where (Cancel_date between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
			dataSet2 = Module1.connect("select count(id) as num from HT_Rooms_Cancel where (Cancel_date between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
			dataSet3 = Module1.connect("select count(id) as num from HT_Rooms_Cancel where (Cancel_date between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
			ListView listView2 = ListView1;
			listView2.Items.Add("ยกเล\u0e34กห\u0e49อง");
			ListViewItem.ListViewSubItemCollection subItems4 = listView2.Items[listView2.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array7 = array3;
			dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow5 = dataRow;
			columnName = "num";
			array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
			array = array3;
			object[] arguments4 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems5 = listView2.Items[listView2.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array8 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow6 = dataRow;
			columnName = "num";
			array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
			array = array3;
			object[] arguments5 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems5, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems6 = listView2.Items[listView2.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array9 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow7 = dataRow;
			columnName = "num";
			array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
			array = array3;
			object[] arguments6 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems6, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView2 = null;
			dataSet = Module1.connect("select count(id) as num from view_CheckIn_Ds where cin_room_dep<>0 and (cin_date between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
			dataSet2 = Module1.connect("select count(id) as num from view_CheckIn_Ds where cin_room_dep<>0 and (cin_date between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
			dataSet3 = Module1.connect("select count(id) as num from view_CheckIn_Ds where cin_room_dep<>0 and (cin_date between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
			ListView listView3 = ListView1;
			listView3.Items.Add("ม\u0e31ดจำค\u0e48าห\u0e49อง");
			ListViewItem.ListViewSubItemCollection subItems7 = listView3.Items[listView3.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array10 = array3;
			dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow8 = dataRow;
			columnName = "num";
			array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
			array = array3;
			object[] arguments7 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems7, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems8 = listView3.Items[listView3.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array11 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow9 = dataRow;
			columnName = "num";
			array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
			array = array3;
			object[] arguments8 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems8, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems9 = listView3.Items[listView3.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array12 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow10 = dataRow;
			columnName = "num";
			array12[0] = RuntimeHelpers.GetObjectValue(dataRow10[columnName]);
			array = array3;
			object[] arguments9 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems9, null, "Add", arguments9, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView3 = null;
			dataSet = Module1.connect("select count(id) as num from view_CheckIn_Ds where cin_room_dep<>0 and (cin_dep_return_date between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
			dataSet2 = Module1.connect("select count(id) as num from view_CheckIn_Ds where cin_room_dep<>0 and (cin_dep_return_date between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
			dataSet3 = Module1.connect("select count(id) as num from view_CheckIn_Ds where cin_room_dep<>0 and (cin_dep_return_date between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
			ListView listView4 = ListView1;
			listView4.Items.Add("ค\u0e37นม\u0e31ดจำค\u0e48าห\u0e49อง");
			ListViewItem.ListViewSubItemCollection subItems10 = listView4.Items[listView4.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array13 = array3;
			dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow11 = dataRow;
			columnName = "num";
			array13[0] = RuntimeHelpers.GetObjectValue(dataRow11[columnName]);
			array = array3;
			object[] arguments10 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems10, null, "Add", arguments10, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems11 = listView4.Items[listView4.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array14 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow12 = dataRow;
			columnName = "num";
			array14[0] = RuntimeHelpers.GetObjectValue(dataRow12[columnName]);
			array = array3;
			object[] arguments11 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems11, null, "Add", arguments11, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems12 = listView4.Items[listView4.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array15 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow13 = dataRow;
			columnName = "num";
			array15[0] = RuntimeHelpers.GetObjectValue(dataRow13[columnName]);
			array = array3;
			object[] arguments12 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems12, null, "Add", arguments12, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView4 = null;
			dataSet = Module1.connect("select count(id) as num from HT_Book_Ds where   (book_room_start between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
			dataSet2 = Module1.connect("select count(id) as num from HT_Book_Ds where   (book_room_start between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
			dataSet3 = Module1.connect("select count(id) as num from HT_Book_Ds where   (book_room_start between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
			ListView listView5 = ListView1;
			listView5.Items.Add("จองห\u0e49องพ\u0e31ก");
			ListViewItem.ListViewSubItemCollection subItems13 = listView5.Items[listView5.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array16 = array3;
			dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow14 = dataRow;
			columnName = "num";
			array16[0] = RuntimeHelpers.GetObjectValue(dataRow14[columnName]);
			array = array3;
			object[] arguments13 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems13, null, "Add", arguments13, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems14 = listView5.Items[listView5.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array17 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow15 = dataRow;
			columnName = "num";
			array17[0] = RuntimeHelpers.GetObjectValue(dataRow15[columnName]);
			array = array3;
			object[] arguments14 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems14, null, "Add", arguments14, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems15 = listView5.Items[listView5.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array18 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow16 = dataRow;
			columnName = "num";
			array18[0] = RuntimeHelpers.GetObjectValue(dataRow16[columnName]);
			array = array3;
			object[] arguments15 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems15, null, "Add", arguments15, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView5 = null;
		}
	}

	public void Search_Products()
	{
		DataSet dataSet = Module1.connect("select pro_no,pro_name from HT_Products order by pro_name");
		DataSet dataSet2 = null;
		DataSet dataSet3 = null;
		DataSet dataSet4 = null;
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
				decimal num5 = default(decimal);
				decimal num6 = default(decimal);
				decimal num7 = default(decimal);
				dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select COALESCE (sum(cin_pro_num), 0) as num from HT_CheckIn_Product where cin_pro_id='", dataSet.Tables[0].Rows[num2]["pro_no"]), "' and (cin_ds_date between '"), DateTimePicker1.Value.Date), " 00:00:00' and '"), DateTimePicker2.Value.Date), " 23:59:59')")));
				dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select COALESCE (sum(cin_pro_num), 0) as num from HT_CheckIn_Product where cin_pro_id='", dataSet.Tables[0].Rows[num2]["pro_no"]), "' and (cin_ds_date between '"), Strings.Format(DateTimePicker1.Value, "MM/01/yyyy")), " 00:00:00' and '"), Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy")), " 23:59:59')")));
				dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select COALESCE (sum(cin_pro_num), 0) as num from HT_CheckIn_Product where cin_pro_id='", dataSet.Tables[0].Rows[num2]["pro_no"]), "' and (cin_ds_date between '"), Strings.Format(DateTimePicker1.Value, "01/01/yyyy")), " 00:00:00' and '"), Strings.Format(DateTimePicker1.Value, "12/31/yyyy")), " 23:59:59')")));
				num5 = Conversions.ToDecimal(Operators.AddObject(num5, dataSet2.Tables[0].Rows[0]["num"]));
				num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet3.Tables[0].Rows[0]["num"]));
				num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet4.Tables[0].Rows[0]["num"]));
				dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT     COALESCE (sum(dbo.HT_Bill_Debt_Ds.DS_NUM), 0) as num FROM dbo.HT_Bill_Debt_H INNER JOIN  dbo.HT_Bill_Debt_Ds ON dbo.HT_Bill_Debt_H.Bill_No = dbo.HT_Bill_Debt_Ds.Bill_No where DS_NO='", dataSet.Tables[0].Rows[num2]["pro_no"]), "' and (bill_date between '"), DateTimePicker1.Value.Date), " 00:00:00' and '"), DateTimePicker2.Value.Date), " 23:59:59')")));
				dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT     COALESCE (sum(dbo.HT_Bill_Debt_Ds.DS_NUM), 0) as num FROM dbo.HT_Bill_Debt_H INNER JOIN  dbo.HT_Bill_Debt_Ds ON dbo.HT_Bill_Debt_H.Bill_No = dbo.HT_Bill_Debt_Ds.Bill_No where DS_NO='", dataSet.Tables[0].Rows[num2]["pro_no"]), "' and (bill_date between '"), Strings.Format(DateTimePicker1.Value, "MM/01/yyyy")), " 00:00:00' and '"), Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy")), " 23:59:59')")));
				dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT     COALESCE (sum(dbo.HT_Bill_Debt_Ds.DS_NUM), 0) as num FROM dbo.HT_Bill_Debt_H INNER JOIN  dbo.HT_Bill_Debt_Ds ON dbo.HT_Bill_Debt_H.Bill_No = dbo.HT_Bill_Debt_Ds.Bill_No where DS_NO='", dataSet.Tables[0].Rows[num2]["pro_no"]), "' and (bill_date between '"), Strings.Format(DateTimePicker1.Value, "01/01/yyyy")), " 00:00:00' and '"), Strings.Format(DateTimePicker1.Value, "12/31/yyyy")), " 23:59:59')")));
				num5 = Conversions.ToDecimal(Operators.AddObject(num5, dataSet2.Tables[0].Rows[0]["num"]));
				num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet3.Tables[0].Rows[0]["num"]));
				num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet4.Tables[0].Rows[0]["num"]));
				if ((decimal.Compare(num5, 0m) != 0) | (decimal.Compare(num6, 0m) != 0) | (decimal.Compare(num7, 0m) != 0))
				{
					ListView listView = ListView2;
					ListView.ListViewItemCollection items = listView.Items;
					object[] array = new object[1];
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					string columnName = "pro_name";
					array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", array2, null, null, array3, IgnoreReturn: true);
					if (array3[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
					}
					listView.Items[listView.Items.Count - 1].SubItems.Add(Conversions.ToString(num5));
					listView.Items[listView.Items.Count - 1].SubItems.Add(Conversions.ToString(num6));
					listView.Items[listView.Items.Count - 1].SubItems.Add(Conversions.ToString(num7));
					listView = null;
				}
				num2++;
			}
		}
	}

	public void Search_Debt()
	{
		DataSet dataSet = null;
		DataSet dataSet2 = null;
		DataSet dataSet3 = null;
		dataSet = Module1.connect("select COALESCE (sum(total_price_balance),0) as num from HT_CheckIn_H where total_price_balance >0 and (cin_date between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
		dataSet2 = Module1.connect("select COALESCE (sum(total_price_balance),0) as num from HT_CheckIn_H where total_price_balance >0 and (cin_date between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
		dataSet3 = Module1.connect("select COALESCE (sum(total_price_balance),0) as num from HT_CheckIn_H where total_price_balance >0 and (cin_date between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
		ListView listView = ListView3;
		listView.Items.Add("ล\u0e39กหน\u0e36\u0e49 ลงทะเบ\u0e35ยน");
		checked
		{
			ListViewItem.ListViewSubItemCollection subItems = listView.Items[listView.Items.Count - 1].SubItems;
			object[] array = new object[1];
			object[] array2 = array;
			DataRow dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow2 = dataRow;
			string columnName = "num";
			array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
			object[] array3 = array;
			object[] arguments = array3;
			bool[] array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[listView.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array5 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow3 = dataRow;
			columnName = "num";
			array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
			array = array3;
			object[] arguments2 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[listView.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array6 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow4 = dataRow;
			columnName = "num";
			array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
			array = array3;
			object[] arguments3 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView = null;
			dataSet = Module1.connect("select COALESCE (sum(bill_debt),0) as num from HT_Bill_Debt_H where bill_debt >0 and (bill_date between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59')");
			dataSet2 = Module1.connect("select COALESCE (sum(bill_debt),0) as num from HT_Bill_Debt_H where bill_debt >0 and (bill_date between '" + Strings.Format(DateTimePicker1.Value, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker1.Value.Year, DateTimePicker1.Value.Month)) + "/yyyy") + " 23:59:59')");
			dataSet3 = Module1.connect("select COALESCE (sum(bill_debt),0) as num from HT_Bill_Debt_H where bill_debt >0 and (bill_date between '" + Strings.Format(DateTimePicker1.Value, "01/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker1.Value, "12/31/yyyy") + " 23:59:59')");
			ListView listView2 = ListView3;
			listView2.Items.Add("ล\u0e39กหน\u0e36\u0e49 ขายส\u0e34นค\u0e49า");
			ListViewItem.ListViewSubItemCollection subItems4 = listView2.Items[listView2.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array7 = array3;
			dataRow = dataSet.Tables[0].Rows[0];
			DataRow dataRow5 = dataRow;
			columnName = "num";
			array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
			array = array3;
			object[] arguments4 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems5 = listView2.Items[listView2.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array8 = array3;
			dataRow = dataSet2.Tables[0].Rows[0];
			DataRow dataRow6 = dataRow;
			columnName = "num";
			array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
			array = array3;
			object[] arguments5 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems5, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			ListViewItem.ListViewSubItemCollection subItems6 = listView2.Items[listView2.Items.Count - 1].SubItems;
			array3 = new object[1];
			object[] array9 = array3;
			dataRow = dataSet3.Tables[0].Rows[0];
			DataRow dataRow7 = dataRow;
			columnName = "num";
			array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
			array = array3;
			object[] arguments6 = array;
			array4 = new bool[1] { true };
			NewLateBinding.LateCall(subItems6, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
			if (array4[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView2 = null;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		string text = "";
		text = "รายงานสร\u0e38ปภาพรวมรายได\u0e49 ระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy");
		Module1.localdata.Report_Room_all.Rows.Clear();
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				Module1.localdata.Report_Room_all.AddReport_Room_allRow(text, "ภาพรวมห\u0e49องพ\u0e31ก", ListView1.Items[num2].SubItems[0].Text, ListView1.Items[num2].SubItems[1].Text, ListView1.Items[num2].SubItems[2].Text, ListView1.Items[num2].SubItems[3].Text, "");
				num2++;
			}
			int num5 = ListView2.Items.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				Module1.localdata.Report_Room_all.AddReport_Room_allRow(text, "ภาพรวมส\u0e34นค\u0e49า", ListView2.Items[num6].SubItems[0].Text, ListView2.Items[num6].SubItems[1].Text, ListView2.Items[num6].SubItems[2].Text, ListView2.Items[num6].SubItems[3].Text, "");
				num6++;
			}
			int num8 = ListView3.Items.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 > num4)
				{
					break;
				}
				Module1.localdata.Report_Room_all.AddReport_Room_allRow(text, "ภาพรวมล\u0e39กหน\u0e35\u0e49", ListView3.Items[num9].SubItems[0].Text, ListView3.Items[num9].SubItems[1].Text, ListView3.Items[num9].SubItems[2].Text, ListView3.Items[num9].SubItems[3].Text, "");
				num9++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReport_Room_ALL crystalReport_Room_ALL = new CrystalReport_Room_ALL();
			crystalReport_Room_ALL.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReport_Room_ALL;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}
}
