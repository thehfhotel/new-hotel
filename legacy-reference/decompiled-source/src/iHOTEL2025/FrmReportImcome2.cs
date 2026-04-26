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
public class FrmReportImcome2 : Office2007Form
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

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("Labelstatus")]
	private Label _Labelstatus;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

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
			_CheckBox1 = value;
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
	static FrmReportImcome2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportImcome2()
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
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.Label1 = new System.Windows.Forms.Label();
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
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.Labelstatus = new System.Windows.Forms.Label();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.CheckBox1);
		this.GroupBox1.Controls.Add(this.ComboBox2);
		this.GroupBox1.Controls.Add(this.Label1);
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
		this.GroupBox1.Text = "รายงานรายร\u0e31บ รายจ\u0e48าย";
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(749, 46);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(119, 21);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 98;
		this.CheckBox1.Text = "ไม\u0e48แสดงรายจ\u0e48าย";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.ComboBox2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox2.DropDownWidth = 500;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[3] { "ท\u0e31\u0e49งหมด", "บ\u0e34ลธรรมดา", "บ\u0e34ลภาษ\u0e35" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox2;
		location = new System.Drawing.Point(102, 59);
		comboBox.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox2;
		size = new System.Drawing.Size(534, 24);
		comboBox2.Size = size;
		this.ComboBox2.TabIndex = 97;
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(19, 63);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(78, 16);
		label2.Size = size;
		this.Label1.TabIndex = 96;
		this.Label1.Text = "ถ\u0e36งรอบบ\u0e34ลท\u0e35\u0e48 :";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.DropDownWidth = 500;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "ท\u0e31\u0e49งหมด", "บ\u0e34ลธรรมดา", "บ\u0e34ลภาษ\u0e35" });
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox1;
		location = new System.Drawing.Point(102, 29);
		comboBox3.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox1;
		size = new System.Drawing.Size(534, 24);
		comboBox4.Size = size;
		this.ComboBox1.TabIndex = 95;
		this.Label5.AutoSize = true;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label5.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label3 = this.Label5;
		location = new System.Drawing.Point(11, 33);
		label3.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label4 = this.Label5;
		size = new System.Drawing.Size(85, 16);
		label4.Size = size;
		this.Label5.TabIndex = 94;
		this.Label5.Text = "จากรอบบ\u0e34ลท\u0e35\u0e48 :";
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(645, 28);
		button.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button2 = this.Button3;
		size = new System.Drawing.Size(84, 55);
		button2.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "ค\u0e49นหา";
		this.Button3.UseVisualStyleBackColor = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[12]
		{
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader7, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader10, this.ColumnHeader12, this.ColumnHeader6, this.ColumnHeader11,
			this.ColumnHeader8, this.ColumnHeader9
		});
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(6, 96);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(1033, 381);
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
		this.ColumnHeader2.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader2.Width = 110;
		this.ColumnHeader3.Text = "เลขลงทะเบ\u0e35ยน / เลขท\u0e35\u0e48ร\u0e31บเง\u0e34น";
		this.ColumnHeader3.Width = 190;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader7.Width = 250;
		this.ColumnHeader4.Text = "เง\u0e34นสด";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 80;
		this.ColumnHeader5.Text = "บ\u0e31ตรเครด\u0e34ต";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 80;
		this.ColumnHeader10.Text = "โอน";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 80;
		this.ColumnHeader6.Text = "รวมเง\u0e34นร\u0e31บ";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader6.Width = 110;
		this.ColumnHeader11.Text = "ฟร\u0e35";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 80;
		this.ColumnHeader12.Text = "เว\u0e47บ";
		this.ColumnHeader12.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader12.Width = 80;
		this.ColumnHeader8.Text = "รายละเอ\u0e35ยดการร\u0e31บ/จ\u0e48าย";
		this.ColumnHeader8.Width = 400;
		this.ColumnHeader9.Text = "หมายเหต\u0e38";
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
		this.Labelstatus.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Labelstatus.AutoSize = true;
		this.Labelstatus.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label labelstatus = this.Labelstatus;
		location = new System.Drawing.Point(665, 508);
		labelstatus.Location = location;
		this.Labelstatus.Name = "Labelstatus";
		System.Windows.Forms.Label labelstatus2 = this.Labelstatus;
		size = new System.Drawing.Size(205, 17);
		labelstatus2.Size = size;
		this.Labelstatus.TabIndex = 4;
		this.Labelstatus.Text = "**ต\u0e49องป\u0e34ดรอบบ\u0e34ลก\u0e48อนถ\u0e36งจะพ\u0e34มพ\u0e4cได\u0e49";
		this.Labelstatus.Visible = false;
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
		this.Name = "FrmReportImcome2";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานรายร\u0e31บของโรงแรม";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		search();
	}

	public void search()
	{
		object left = "SELECT * from View_Pay_Ds";
		object left2 = "SELECT * from TB_Pay_History";
		if (Operators.CompareString(ComboBox1.Text, ComboBox2.Text, TextCompare: false) == 0)
		{
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
			if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
			{
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(string.Concat(string.Concat(" where (pay_date between " + Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[0]["round_start"]).ToOADate()), " and "), Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[0]["round_end"]).ToOADate())), ")"));
			}
			else
			{
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(" where (pay_date >= " + Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[0]["round_start"]).ToOADate()), ")"));
			}
			left = Operators.ConcatenateObject(left, " and Cin_Status<>'ยกเล\u0e34ก' order by pay_no");
			left2 = Operators.ConcatenateObject(left2, " order by pay_date");
		}
		else
		{
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
			DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox2.SelectedValue)));
			if (Operators.CompareString(dataSet3.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
			{
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet2.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet3.Tables[0].Rows[0]["round_end"]), "')"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(string.Concat(string.Concat(" where (pay_date between " + Conversions.ToString(Conversions.ToDate(dataSet2.Tables[0].Rows[0]["round_start"]).ToOADate()), " and "), Conversions.ToString(Conversions.ToDate(dataSet3.Tables[0].Rows[0]["round_end"]).ToOADate())), ")"));
			}
			else
			{
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet2.Tables[0].Rows[0]["round_start"]), "')"));
				left = Operators.ConcatenateObject(left, string.Concat(" where (pay_date >= " + Conversions.ToString(Conversions.ToDate(dataSet2.Tables[0].Rows[0]["round_start"]).ToOADate()), ")"));
			}
			left = Operators.ConcatenateObject(left, " and Cin_Status<>'ยกเล\u0e34ก' order by pay_no");
			left2 = Operators.ConcatenateObject(left2, " order by pay_date");
		}
		DataSet dataSet4 = Module1.connect(Conversions.ToString(left));
		DataSet dataSet5 = Module1.connect(Conversions.ToString(left2));
		ListView1.Items.Clear();
		decimal num = default(decimal);
		decimal d = default(decimal);
		string left3 = "";
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		decimal num5 = default(decimal);
		decimal num6 = default(decimal);
		string text = "";
		string text2 = "";
		string text3 = "";
		string text4 = "";
		string text5 = "";
		string text6 = "";
		decimal num7 = default(decimal);
		decimal num8 = default(decimal);
		decimal num9 = default(decimal);
		decimal num10 = default(decimal);
		decimal num11 = default(decimal);
		checked
		{
			int num12 = dataSet4.Tables[0].Rows.Count - 1;
			int num13 = 0;
			while (true)
			{
				int num14 = num13;
				int num15 = num12;
				if (num14 > num15)
				{
					break;
				}
				if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectNotEqual(left3, dataSet4.Tables[0].Rows[num13]["pay_no"], TextCompare: false), num13 != 0)))
				{
					num = decimal.Add(num, decimal.Add(decimal.Add(decimal.Add(num3, num2), num4), num6));
					d = decimal.Add(d, 0m);
					num7 = decimal.Add(num7, num3);
					num8 = decimal.Add(num8, num2);
					num9 = decimal.Add(num9, num4);
					num10 = decimal.Add(num10, num5);
					num11 = decimal.Add(num11, num6);
					global::PrintableListView.PrintableListView listView = ListView1;
					int count = listView.Items.Count;
					listView.Items.Add(Conversions.ToString(count + 1));
					listView.Items[count].SubItems.Add(text4);
					listView.Items[count].SubItems.Add(text5);
					listView.Items[count].SubItems.Add(text6);
					listView.Items[count].SubItems.Add(Strings.Format(num3, "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(num2, "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(num4, "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(num6, "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(decimal.Add(decimal.Add(decimal.Add(num3, num2), num4), num6), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(num5, "#,##0.00"));
					listView.Items[count].SubItems.Add(text2 + " (" + text + ")");
					listView.Items[count].SubItems.Add(text3);
					listView = null;
					num2 = default(decimal);
					num3 = default(decimal);
					num4 = default(decimal);
					num5 = default(decimal);
					num6 = default(decimal);
					text = "";
					text2 = "";
					text3 = "";
					text4 = "";
					text5 = "";
					text6 = "";
				}
				text4 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num13]["cin_pay_date"]), "dd/MM/yy HH:mm");
				text5 = dataSet4.Tables[0].Rows[num13]["cin_no"].ToString() + " / " + dataSet4.Tables[0].Rows[num13]["pay_no"].ToString();
				text6 = dataSet4.Tables[0].Rows[num13]["cust_name"].ToString();
				num2 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num13]["cin_pay_credit"]);
				num3 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num13]["cin_pay_cash"]);
				num4 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num13]["cin_pay_tran"]);
				num5 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num13]["cin_pay_free"]);
				num6 = Conversions.ToDecimal(dataSet4.Tables[0].Rows[num13]["cin_pay_web"]);
				left3 = Conversions.ToString(dataSet4.Tables[0].Rows[num13]["pay_no"]);
				if ((text.IndexOf(dataSet4.Tables[0].Rows[num13]["cin_pay_ds"].ToString()) == -1) | (Operators.CompareString(text, "", TextCompare: false) == 0))
				{
					if (Operators.CompareString(text, "", TextCompare: false) != 0)
					{
						text += ",";
					}
					text += dataSet4.Tables[0].Rows[num13]["cin_pay_ds"].ToString();
				}
				if ((text2.IndexOf(dataSet4.Tables[0].Rows[num13]["cin_pay_ds_name"].ToString()) == -1) | (Operators.CompareString(text2, "", TextCompare: false) == 0))
				{
					if (Operators.CompareString(text2, "", TextCompare: false) != 0)
					{
						text2 += ",";
					}
					text2 += dataSet4.Tables[0].Rows[num13]["cin_pay_ds_name"].ToString();
				}
				if ((text3.IndexOf(dataSet4.Tables[0].Rows[num13]["cin_pay_note"].ToString()) == -1) | (Operators.CompareString(text3, "", TextCompare: false) == 0))
				{
					if (Operators.CompareString(text3, "", TextCompare: false) != 0)
					{
						text3 += ",";
					}
					text3 += dataSet4.Tables[0].Rows[num13]["cin_pay_note"].ToString();
				}
				if (num13 == dataSet4.Tables[0].Rows.Count - 1)
				{
					num = decimal.Add(num, decimal.Add(decimal.Add(decimal.Add(num3, num2), num4), num6));
					num7 = decimal.Add(num7, num3);
					num8 = decimal.Add(num8, num2);
					num9 = decimal.Add(num9, num4);
					num10 = decimal.Add(num10, num5);
					num11 = decimal.Add(num11, num6);
					global::PrintableListView.PrintableListView listView2 = ListView1;
					int count2 = listView2.Items.Count;
					listView2.Items.Add(Conversions.ToString(count2 + 1));
					listView2.Items[count2].SubItems.Add(text4);
					listView2.Items[count2].SubItems.Add(text5);
					listView2.Items[count2].SubItems.Add(text6);
					listView2.Items[count2].SubItems.Add(Strings.Format(num3, "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(num2, "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(num4, "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(num6, "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(decimal.Add(decimal.Add(decimal.Add(num3, num2), num4), num6), "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(num5, "#,##0.00"));
					listView2.Items[count2].SubItems.Add(text2 + " (" + text + ")");
					listView2.Items[count2].SubItems.Add(text3);
					listView2 = null;
				}
				num13++;
			}
			if (!CheckBox1.Checked)
			{
				int num16 = dataSet5.Tables[0].Rows.Count - 1;
				int num17 = 0;
				while (true)
				{
					int num18 = num17;
					int num15 = num16;
					if (num18 > num15)
					{
						break;
					}
					num2 = default(decimal);
					num3 = Conversions.ToDecimal(dataSet5.Tables[0].Rows[num17]["pay_total"]);
					num4 = default(decimal);
					num5 = default(decimal);
					num6 = default(decimal);
					if (Operators.ConditionalCompareObjectEqual(dataSet5.Tables[0].Rows[num17]["pay_type"], "รายจ\u0e48าย", TextCompare: false))
					{
						num3 = decimal.Multiply(num3, -1m);
						num7 = decimal.Add(num7, num3);
						num = decimal.Add(num, num3);
					}
					else
					{
						num7 = decimal.Add(num7, num3);
						num = decimal.Add(num, num3);
					}
					global::PrintableListView.PrintableListView listView3 = ListView1;
					int count3 = listView3.Items.Count;
					listView3.Items.Add(Conversions.ToString(count3 + 1));
					listView3.Items[count3].SubItems.Add(Strings.Format(DateTime.FromOADate(Conversions.ToDouble(dataSet5.Tables[0].Rows[num17]["pay_date"])), "dd/MM/yy HH:mm"));
					ListViewItem.ListViewSubItemCollection subItems = listView3.Items[count3].SubItems;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet5.Tables[0].Rows[num17];
					DataRow dataRow2 = dataRow;
					string columnName = "pay_account";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listView3.Items[count3].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet5.Tables[0].Rows[num17];
					DataRow dataRow3 = dataRow;
					columnName = "pay_type";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listView3.Items[count3].SubItems.Add(Strings.Format(num3, "#,##0.00"));
					listView3.Items[count3].SubItems.Add(Strings.Format(0, "#,##0.00"));
					listView3.Items[count3].SubItems.Add(Strings.Format(0, "#,##0.00"));
					listView3.Items[count3].SubItems.Add(Strings.Format(0, "#,##0.00"));
					listView3.Items[count3].SubItems.Add(Strings.Format(decimal.Add(decimal.Add(decimal.Add(num3, num2), num4), num6), "#,##0.00"));
					listView3.Items[count3].SubItems.Add(Strings.Format(0, "#,##0.00"));
					ListViewItem.ListViewSubItemCollection subItems3 = listView3.Items[count3].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet5.Tables[0].Rows[num17];
					DataRow dataRow4 = dataRow;
					columnName = "pay_bill";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listView3.Items[count3].SubItems.Add("");
					listView3 = null;
					num17++;
				}
			}
			global::PrintableListView.PrintableListView listView4 = ListView1;
			listView4.Items.Add("");
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add("รวม");
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add("");
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add("");
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(num7, "#,##0.00"));
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(num8, "#,##0.00"));
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(num9, "#,##0.00"));
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(num11, "#,##0.00"));
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(num, "#,##0.00"));
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(num10, "#,##0.00"));
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add("");
			listView4.Items[ListView1.Items.Count - 1].SubItems.Add("");
			listView4 = null;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(ComboBox1.Text, ComboBox2.Text, TextCompare: false) == 0)
		{
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
			if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
			{
				ListView1.Title = "รายงานรายร\u0e31บของโรงแรม\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_end"]), "dd-MM-yy เวลา HH:mm น.");
			}
			else
			{
				ListView1.Title = "รายงานรายร\u0e31บของโรงแรม\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTime.Now, "dd-MM-yy เวลา HH:mm น.");
			}
		}
		else
		{
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
			DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox2.SelectedValue)));
			if (Operators.CompareString(dataSet3.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
			{
				ListView1.Title = "รายงานรายร\u0e31บของโรงแรม\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["round_end"]), "dd-MM-yy เวลา HH:mm น.");
			}
			else
			{
				ListView1.Title = "รายงานรายร\u0e31บของโรงแรม\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTime.Now, "dd-MM-yy เวลา HH:mm น.");
			}
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
		DataSet dataSet2 = Module1.connect("select top 1000 id,FullDate from View_RBill_H_Round_Only order by id desc");
		ComboBox1.DataSource = dataSet.Tables[0];
		ComboBox1.DisplayMember = "FullDate";
		ComboBox1.ValueMember = "id";
		ComboBox1.Text = "";
		ComboBox2.DataSource = dataSet2.Tables[0];
		ComboBox2.DisplayMember = "FullDate";
		ComboBox2.ValueMember = "id";
		ComboBox2.Text = "";
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			ComboBox2.SelectedValue = RuntimeHelpers.GetObjectValue(ComboBox1.SelectedValue);
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
	}

	private void ComboBox2_SelectedIndexChanged(object sender, EventArgs e)
	{
	}
}
