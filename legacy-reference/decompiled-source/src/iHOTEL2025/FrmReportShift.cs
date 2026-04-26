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
public class FrmReportShift : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ColumnHeader17")]
	private ColumnHeader _ColumnHeader17;

	[AccessedThroughProperty("ColumnHeader18")]
	private ColumnHeader _ColumnHeader18;

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
	static FrmReportShift()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportShift()
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
		this.Label5 = new System.Windows.Forms.Label();
		this.Button3 = new System.Windows.Forms.Button();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader17 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.ColumnHeader18 = new System.Windows.Forms.ColumnHeader();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
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
		this.GroupBox1.Text = "รายงานการขายห\u0e49องตามรอบบ\u0e34ล";
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
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader7, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader16, this.ColumnHeader6, this.ColumnHeader8, this.ColumnHeader14,
			this.ColumnHeader15, this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader11, this.ColumnHeader12, this.ColumnHeader17, this.ColumnHeader18, this.ColumnHeader13
		});
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
		this.ColumnHeader2.Text = "เลขท\u0e35\u0e48";
		this.ColumnHeader2.Width = 80;
		this.ColumnHeader3.Text = "เบอร\u0e4cห\u0e49อง";
		this.ColumnHeader3.Width = 170;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.ColumnHeader7.Width = 350;
		this.ColumnHeader4.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49า";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader4.Width = 110;
		this.ColumnHeader5.Text = "ว\u0e31นท\u0e35\u0e48ออก";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader5.Width = 110;
		this.ColumnHeader16.Text = "เรทห\u0e49อง";
		this.ColumnHeader16.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader16.Width = 80;
		this.ColumnHeader6.Text = "ล\u0e39กหน\u0e35\u0e49";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Text = "ม\u0e31ดจำ";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader14.Text = "รวมค\u0e48าห\u0e49อง";
		this.ColumnHeader14.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader14.Width = 80;
		this.ColumnHeader15.Text = "รวมค\u0e48าส\u0e34นค\u0e49า";
		this.ColumnHeader15.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader15.Width = 80;
		this.ColumnHeader9.Text = "เง\u0e34นสด";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 80;
		this.ColumnHeader10.Text = "บ\u0e31ตรเครด\u0e34ต";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 80;
		this.ColumnHeader11.Text = "จ\u0e48ายล\u0e48วงหน\u0e49า";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 80;
		this.ColumnHeader12.Text = "ค\u0e37นแขก";
		this.ColumnHeader12.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader12.Width = 80;
		this.ColumnHeader17.Text = "เว\u0e47บ";
		this.ColumnHeader17.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader17.Width = 80;
		this.ColumnHeader13.Text = "พน\u0e31กงาน";
		this.ColumnHeader13.Width = 100;
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
		this.ColumnHeader18.Text = "ฟร\u0e35";
		this.ColumnHeader18.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader18.Width = 80;
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
		this.Name = "FrmReportShift";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานการขายตามรอบบ\u0e34ล";
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
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
		object left = "select * from View_CheckIn_Ds";
		object left2 = "select * from HT_CheckIn_Product where cin_no in (select cin_no from View_CheckIn_Ds";
		object left3 = "select pay_no,cin_no,cin_pay_cash,cin_pay_free,cin_pay_web,cin_pay_credit,pay_by from HT_CheckIn_Pay where cin_no in (select cin_no from View_CheckIn_Ds";
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left3 = Operators.ConcatenateObject(left3, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
		}
		else
		{
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left3 = Operators.ConcatenateObject(left3, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
		}
		left = Operators.ConcatenateObject(left, " order by cin_no,cin_room_no");
		left2 = Operators.ConcatenateObject(left2, ")");
		left3 = Operators.ConcatenateObject(left3, ") group by pay_no,cin_no,cin_pay_cash,cin_pay_free,cin_pay_web,cin_pay_credit,pay_by");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(left));
		DataSet dataSet3 = Module1.connect(Conversions.ToString(left2));
		DataSet dataSet4 = Module1.connect(Conversions.ToString(left3));
		ListView1.Items.Clear();
		string text = "";
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		decimal num5 = default(decimal);
		decimal num6 = default(decimal);
		decimal num7 = default(decimal);
		decimal num8 = default(decimal);
		decimal num9 = default(decimal);
		decimal num10 = default(decimal);
		decimal num11 = default(decimal);
		decimal num12 = default(decimal);
		decimal num13 = default(decimal);
		decimal num14 = default(decimal);
		decimal num15 = default(decimal);
		decimal num16 = default(decimal);
		decimal num17 = default(decimal);
		int num18 = 1;
		checked
		{
			int num19 = dataSet2.Tables[0].Rows.Count - 1;
			int num20 = 0;
			while (true)
			{
				int num21 = num20;
				int num22 = num19;
				if (num21 > num22)
				{
					break;
				}
				int count = ListView1.Items.Count;
				global::PrintableListView.PrintableListView listView = ListView1;
				listView.Items.Add(Conversions.ToString(decimal.Subtract(new decimal(count + 1), num17)));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow2 = dataRow;
				string columnName = "cin_no";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow3 = dataRow;
				columnName = "cin_room_no";
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
				dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow4 = dataRow;
				columnName = "cin_cust_name";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num20]["cin_room_in"]), "dd/MM/yy HH:mm"));
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num20]["cin_room_out"]), "dd/MM/yy HH:mm"));
				ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array7 = array3;
				dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow5 = dataRow;
				columnName = "cin_room_price";
				array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
				array = array3;
				object[] arguments4 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add("");
				ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array8 = array3;
				dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow6 = dataRow;
				columnName = "cin_room_dep";
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
				dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow7 = dataRow;
				columnName = "cin_room_pricetotal";
				array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
				array = array3;
				object[] arguments6 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems6, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Conversions.ToString(0));
				listView.Items[count].SubItems.Add("");
				listView.Items[count].SubItems.Add("");
				ListViewItem.ListViewSubItemCollection subItems7 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array10 = array3;
				dataRow = dataSet2.Tables[0].Rows[num20];
				DataRow dataRow8 = dataRow;
				columnName = "cin_room_pay_before";
				array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
				array = array3;
				object[] arguments7 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems7, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add("");
				listView.Items[count].SubItems.Add("");
				listView.Items[count].SubItems.Add("");
				listView.Items[count].SubItems.Add(dataSet2.Tables[0].Rows[num20]["cin_by"].ToString());
				if (num18 == 2)
				{
					listView.Items[count].BackColor = Color.LightCyan;
				}
				listView = null;
				int num23 = dataSet3.Tables[0].Rows.Count - 1;
				int num24 = 0;
				while (true)
				{
					int num25 = num24;
					num22 = num23;
					if (num25 > num22)
					{
						break;
					}
					if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num20]["cin_no"], dataSet3.Tables[0].Rows[num24]["cin_no"], TextCompare: false), Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num20]["cin_room_no"], dataSet3.Tables[0].Rows[num24]["cin_room_no"], TextCompare: false))))
					{
						ListView1.Items[count].SubItems[10].Text = Conversions.ToString(Operators.AddObject(Conversions.ToDecimal(ListView1.Items[count].SubItems[10].Text), dataSet3.Tables[0].Rows[num24]["cin_pro_priceTotal"]));
						num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet3.Tables[0].Rows[num24]["cin_pro_priceTotal"]));
					}
					num24++;
				}
				num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num20]["cin_room_dep"]));
				num5 = Conversions.ToDecimal(Operators.AddObject(num5, dataSet2.Tables[0].Rows[num20]["cin_room_pricetotal"]));
				num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num20]["cin_room_pay_before"]));
				text = Conversions.ToString(dataSet2.Tables[0].Rows[num20]["cin_no"]);
				bool flag = false;
				if (num20 != dataSet2.Tables[0].Rows.Count - 1 && Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[num20 + 1]["cin_no"], text, TextCompare: false))
				{
					flag = true;
				}
				if (unchecked(num20 == checked(dataSet2.Tables[0].Rows.Count - 1) || flag))
				{
					decimal num26 = default(decimal);
					decimal num27 = default(decimal);
					decimal num28 = default(decimal);
					decimal num29 = default(decimal);
					decimal num30 = default(decimal);
					string text2 = "";
					int num31 = dataSet4.Tables[0].Rows.Count - 1;
					int num32 = 0;
					while (true)
					{
						int num33 = num32;
						num22 = num31;
						if (num33 > num22)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num20]["cin_no"], dataSet4.Tables[0].Rows[num32]["cin_no"], TextCompare: false))
						{
							if (Operators.ConditionalCompareObjectLess(dataSet4.Tables[0].Rows[num32]["cin_pay_cash"], 0, TextCompare: false))
							{
								object left4 = num28;
								Type typeFromHandle = typeof(Math);
								array3 = new object[1];
								object[] array11 = array3;
								dataRow = dataSet4.Tables[0].Rows[num32];
								DataRow dataRow9 = dataRow;
								columnName = "cin_pay_cash";
								array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
								array = array3;
								object[] arguments8 = array;
								array4 = new bool[1] { true };
								object right = NewLateBinding.LateGet(null, typeFromHandle, "Abs", arguments8, null, null, array4);
								if (array4[0])
								{
									dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
								}
								num28 = Conversions.ToDecimal(Operators.AddObject(left4, right));
							}
							else
							{
								num26 = Conversions.ToDecimal(Operators.AddObject(num26, dataSet4.Tables[0].Rows[num32]["cin_pay_cash"]));
								num27 = Conversions.ToDecimal(Operators.AddObject(num27, dataSet4.Tables[0].Rows[num32]["cin_pay_credit"]));
								num29 = Conversions.ToDecimal(Operators.AddObject(num29, dataSet4.Tables[0].Rows[num32]["cin_pay_web"]));
								num30 = Conversions.ToDecimal(Operators.AddObject(num30, dataSet4.Tables[0].Rows[num32]["cin_pay_free"]));
							}
							text2 = Conversions.ToString(dataSet4.Tables[0].Rows[num32]["pay_by"]);
						}
						num32++;
					}
					count = ListView1.Items.Count;
					global::PrintableListView.PrintableListView listView2 = ListView1;
					listView2.Items.Add("");
					listView2.Items[count].SubItems.Add("");
					listView2.Items[count].SubItems.Add("");
					listView2.Items[count].SubItems.Add("รวม");
					listView2.Items[count].SubItems.Add("");
					listView2.Items[count].SubItems.Add("");
					listView2.Items[count].SubItems.Add("");
					listView2.Items[count].SubItems.Add(Conversions.ToString(decimal.Add(decimal.Subtract(decimal.Add(num5, num6), decimal.Add(num26, num27)), num28)));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num4));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num5));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num6));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num26));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num27));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num3));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num28));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num29));
					listView2.Items[count].SubItems.Add(Conversions.ToString(num30));
					listView2.Items[count].SubItems.Add(text2);
					if (num18 == 2)
					{
						listView2.Items[count].BackColor = Color.LightCyan;
					}
					listView2 = null;
					num18 = ((num18 != 1) ? 1 : 2);
					num17 = decimal.Add(num17, 1m);
					num9 = decimal.Add(num9, num5);
					num10 = decimal.Add(num10, num6);
					num8 = decimal.Add(num8, num4);
					num7 = decimal.Add(num7, num3);
					num11 = decimal.Add(num11, num26);
					num12 = decimal.Add(num12, num27);
					num13 = decimal.Add(num13, num28);
					num14 = decimal.Add(num14, decimal.Add(decimal.Subtract(decimal.Add(num5, num6), decimal.Add(num26, num27)), num28));
					num15 = decimal.Add(num15, num29);
					num5 = default(decimal);
					num6 = default(decimal);
					num4 = default(decimal);
					num3 = default(decimal);
				}
				if (num20 == dataSet2.Tables[0].Rows.Count - 1)
				{
					count = ListView1.Items.Count;
					global::PrintableListView.PrintableListView listView3 = ListView1;
					listView3.Items.Add("");
					listView3.Items[count].SubItems.Add("");
					listView3.Items[count].SubItems.Add("");
					listView3.Items[count].SubItems.Add("รวมท\u0e31\u0e49งหมด");
					listView3.Items[count].SubItems.Add("");
					listView3.Items[count].SubItems.Add("");
					listView3.Items[count].SubItems.Add("");
					listView3.Items[count].SubItems.Add(Conversions.ToString(num14));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num8));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num9));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num10));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num11));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num12));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num7));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num13));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num15));
					listView3.Items[count].SubItems.Add(Conversions.ToString(num16));
					listView3.Items[count].SubItems.Add("");
					listView3.Items[count].BackColor = Color.LightPink;
					listView3 = null;
				}
				num20++;
			}
			int num34 = ListView1.Columns.Count - 1;
			int num35 = 6;
			while (true)
			{
				int num36 = num35;
				int num22 = num34;
				if (num36 > num22)
				{
					break;
				}
				int num37 = ListView1.Items.Count - 1;
				int num38 = 0;
				while (true)
				{
					int num39 = num38;
					num22 = num37;
					if (num39 > num22)
					{
						break;
					}
					if (Operators.CompareString(ListView1.Items[num38].SubItems[num35].Text, "0", TextCompare: false) == 0)
					{
						ListView1.Items[num38].SubItems[num35].Text = "";
					}
					num38++;
				}
				num35++;
			}
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
		{
			ListView1.Title = "รายงานการขายห\u0e49องตามรอบบ\u0e34ล\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_end"]), "dd-MM-yy เวลา HH:mm น.");
		}
		else
		{
			ListView1.Title = "รายงานการขายห\u0e49องตามรอบบ\u0e34ล\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTime.Now, "dd-MM-yy เวลา HH:mm น.");
		}
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
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
	}
}
