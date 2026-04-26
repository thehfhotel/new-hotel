using System;
using System.Collections;
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
public class ReportCustIn : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("TextBox_CUST")]
	private TextBox _TextBox_CUST;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("TextBox_EMP")]
	private TextBox _TextBox_EMP;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("ComboBox3")]
	private ComboBox _ComboBox3;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

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

	internal virtual TextBox TextBox_CUST
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox_CUST;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox_CUST = value;
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

	internal virtual TextBox TextBox_EMP
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox_EMP;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox_EMP = value;
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

	internal virtual ComboBox ComboBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox3 = value;
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
			EventHandler value2 = DateTimePicker1_ValueChanged;
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

	[DebuggerNonUserCode]
	static ReportCustIn()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportCustIn()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += ReportDays_Load;
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
		this.Label1 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.TextBox_CUST = new System.Windows.Forms.TextBox();
		this.Label6 = new System.Windows.Forms.Label();
		this.TextBox_EMP = new System.Windows.Forms.TextBox();
		this.Label7 = new System.Windows.Forms.Label();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.Label8 = new System.Windows.Forms.Label();
		this.ComboBox3 = new System.Windows.Forms.ComboBox();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(40, 33);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(31, 16);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(74, 29);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 1;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(347, 125);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(151, 52);
		button2.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.Text = "ออกรายงาน";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(77, 6);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(31, 16);
		label4.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker2;
		location = new System.Drawing.Point(74, 58);
		dateTimePicker3.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker2;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker2.TabIndex = 5;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(26, 62);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(45, 16);
		label6.Size = size;
		this.Label3.TabIndex = 4;
		this.Label3.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(283, 33);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(62, 16);
		label8.Size = size;
		this.Label4.TabIndex = 6;
		this.Label4.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(347, 29);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(150, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 7;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(17, 129);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(54, 16);
		label10.Size = size;
		this.Label5.TabIndex = 8;
		this.Label5.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		System.Windows.Forms.TextBox textBox_CUST = this.TextBox_CUST;
		location = new System.Drawing.Point(74, 125);
		textBox_CUST.Location = location;
		this.TextBox_CUST.Name = "TextBox_CUST";
		System.Windows.Forms.TextBox textBox_CUST2 = this.TextBox_CUST;
		size = new System.Drawing.Size(200, 23);
		textBox_CUST2.Size = size;
		this.TextBox_CUST.TabIndex = 9;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label6;
		location = new System.Drawing.Point(2, 157);
		label11.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label12 = this.Label6;
		size = new System.Drawing.Size(69, 16);
		label12.Size = size;
		this.Label6.TabIndex = 8;
		this.Label6.Text = "ช\u0e37\u0e48อพน\u0e31กงาน";
		System.Windows.Forms.TextBox textBox_EMP = this.TextBox_EMP;
		location = new System.Drawing.Point(74, 154);
		textBox_EMP.Location = location;
		this.TextBox_EMP.Name = "TextBox_EMP";
		System.Windows.Forms.TextBox textBox_EMP2 = this.TextBox_EMP;
		size = new System.Drawing.Size(200, 23);
		textBox_EMP2.Size = size;
		this.TextBox_EMP.TabIndex = 9;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label7;
		location = new System.Drawing.Point(296, 62);
		label13.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label14 = this.Label7;
		size = new System.Drawing.Size(49, 16);
		label14.Size = size;
		this.Label7.TabIndex = 6;
		this.Label7.Text = "ประเภท";
		this.ComboBox2.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox2;
		location = new System.Drawing.Point(347, 57);
		comboBox3.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox2;
		size = new System.Drawing.Size(150, 24);
		comboBox4.Size = size;
		this.ComboBox2.TabIndex = 7;
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label8;
		location = new System.Drawing.Point(274, 90);
		label15.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label16 = this.Label8;
		size = new System.Drawing.Size(71, 16);
		label16.Size = size;
		this.Label8.TabIndex = 6;
		this.Label8.Text = "ประเภทห\u0e49อง";
		this.ComboBox3.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox5 = this.ComboBox3;
		location = new System.Drawing.Point(347, 86);
		comboBox5.Location = location;
		this.ComboBox3.Name = "ComboBox3";
		System.Windows.Forms.ComboBox comboBox6 = this.ComboBox3;
		size = new System.Drawing.Size(150, 24);
		comboBox6.Size = size;
		this.ComboBox3.TabIndex = 7;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(74, 88);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(148, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 10;
		this.CheckBox1.Text = "ใช\u0e49เวลา 00:00 - 23:59";
		this.CheckBox1.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(510, 203);
		this.ClientSize = size;
		this.Controls.Add(this.CheckBox1);
		this.Controls.Add(this.TextBox_EMP);
		this.Controls.Add(this.Label6);
		this.Controls.Add(this.TextBox_CUST);
		this.Controls.Add(this.Label5);
		this.Controls.Add(this.ComboBox3);
		this.Controls.Add(this.Label8);
		this.Controls.Add(this.ComboBox2);
		this.Controls.Add(this.Label7);
		this.Controls.Add(this.ComboBox1);
		this.Controls.Add(this.Label4);
		this.Controls.Add(this.DateTimePicker2);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportCustIn";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานแขกเข\u0e49าพ\u0e31ก";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object obj = "";
		string text = "";
		if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) != 0)
		{
			text = text + " and cust_type_main='" + ComboBox1.Text + "'";
			obj = Operators.ConcatenateObject(obj, string.Concat(" [กล\u0e38\u0e48ม = " + ComboBox1.Text, "]"));
		}
		if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) != 0)
		{
			text = text + " and cin_cust_price='" + ComboBox2.Text + "'";
			obj = Operators.ConcatenateObject(obj, string.Concat(" [ประเภท = " + ComboBox2.Text, "]"));
		}
		if (Operators.CompareString(ComboBox3.Text, "", TextCompare: false) != 0)
		{
			text = text + " and cin_room_type='" + ComboBox3.Text + "'";
			obj = Operators.ConcatenateObject(obj, string.Concat(" [ประเภทห\u0e49อง = " + ComboBox3.Text, "]"));
		}
		if (Operators.CompareString(TextBox_CUST.Text, "", TextCompare: false) != 0 && Operators.CompareString(text, "", TextCompare: false) == 0)
		{
			text = text + " and cin_cust_name like '%" + TextBox_CUST.Text + "%'";
			obj = Operators.ConcatenateObject(obj, string.Concat(" ช\u0e37\u0e48อล\u0e39กค\u0e49า = *" + TextBox_CUST.Text, "*"));
		}
		if (Operators.CompareString(TextBox_EMP.Text, "", TextCompare: false) != 0 && Operators.CompareString(text, "", TextCompare: false) == 0)
		{
			text = text + " and cin_by like '%" + TextBox_EMP.Text + "%'";
			obj = Operators.ConcatenateObject(obj, string.Concat(" ช\u0e37\u0e48อพน\u0e31กงาน = *" + TextBox_EMP.Text, "*"));
		}
		object obj2 = "";
		object obj3 = "";
		if (CheckBox1.Checked)
		{
			obj2 = "select * from View_CheckIn_Ds where Cin_room_in between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59' " + text + " order by Cin_room_in";
			obj3 = "select * from HT_CheckIn_Product where cin_no in (select cin_no from View_CheckIn_Ds where Cin_room_in between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59' " + text + ")";
		}
		else
		{
			obj2 = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_CheckIn_Ds where Cin_room_in between '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":01' and '"), DateTimePicker2.Value.AddDays(1.0).Date), " "), right2), ":00' "), text), " order by Cin_room_in");
			obj3 = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from HT_CheckIn_Product where cin_no in (select cin_no from View_CheckIn_Ds where Cin_room_in between '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":01' and '"), DateTimePicker2.Value.AddDays(1.0).Date), " "), right2), ":00' "), text), ")");
		}
		DataSet dataSet2 = Module1.connect(Conversions.ToString(obj2));
		DataSet dataSet3 = Module1.connect(Conversions.ToString(obj3));
		Module1.localdata.ReportCustIN.Rows.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		decimal num5 = default(decimal);
		ArrayList arrayList = new ArrayList();
		checked
		{
			int num6 = dataSet2.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num9 = num6;
				if (num8 > num9)
				{
					break;
				}
				string string_ = "ย\u0e31งไม\u0e48ค\u0e37นห\u0e49อง";
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num7]["cin_room_status"], "Check-Out", TextCompare: false))
				{
					string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["cin_room_out"]), "dd/MM/yyyy HH:mm");
				}
				string text2 = "";
				decimal num10 = default(decimal);
				int num11 = dataSet3.Tables[0].Rows.Count - 1;
				int num12 = 0;
				while (true)
				{
					int num13 = num12;
					num9 = num11;
					if (num13 > num9)
					{
						break;
					}
					if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(dataSet3.Tables[0].Rows[num12]["cin_room_no"], dataSet2.Tables[0].Rows[num7]["cin_room_no"], TextCompare: false), Operators.CompareObjectEqual(dataSet3.Tables[0].Rows[num12]["cin_no"], dataSet2.Tables[0].Rows[num7]["cin_no"], TextCompare: false))))
					{
						text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num12]["cin_pro_name"], " ")));
						num10 = Conversions.ToDecimal(Operators.AddObject(num10, dataSet3.Tables[0].Rows[num12]["cin_pro_pricetotal"]));
					}
					num12++;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num7]["cin_room_priceTotal"]));
				num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num7]["cin_room_price"]));
				num2 = decimal.Add(num2, num10);
				int num14 = -1;
				int num15 = arrayList.Count - 1;
				int num16 = 0;
				while (true)
				{
					int num17 = num16;
					num9 = num15;
					if (num17 > num9)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num16], new object[1] { 0 }, null), dataSet2.Tables[0].Rows[num7]["cin_room_type"], TextCompare: false))
					{
						num14 = num16;
					}
					num16++;
				}
				if (num14 == -1)
				{
					string[] value = new string[3]
					{
						Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_room_type"]),
						"1",
						Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_room_price"])
					};
					arrayList.Add(value);
				}
				else
				{
					NewLateBinding.LateIndexSetComplex(arrayList[num14], new object[2]
					{
						1,
						Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList[num14], new object[1] { 1 }, null)) + 1
					}, null, OptimisticSet: false, RValueBase: true);
					NewLateBinding.LateIndexSetComplex(arrayList[num14], new object[2]
					{
						2,
						Operators.AddObject(Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList[num14], new object[1] { 2 }, null)), dataSet2.Tables[0].Rows[num7]["cin_room_price"])
					}, null, OptimisticSet: false, RValueBase: true);
				}
				object obj4 = "";
				if (num7 == dataSet2.Tables[0].Rows.Count - 1)
				{
					int num18 = 0;
					int num19 = arrayList.Count - 1;
					int num20 = 0;
					while (true)
					{
						int num21 = num20;
						num9 = num19;
						if (num21 > num9)
						{
							break;
						}
						num18 += Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList[num20], new object[1] { 1 }, null));
						obj4 = Operators.ConcatenateObject(obj4, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(arrayList[num20], new object[1] { 0 }, null), " = "), NewLateBinding.LateIndexGet(arrayList[num20], new object[1] { 1 }, null)), "\t"), "     รวมเง\u0e34น "), Strings.Format(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num20], new object[1] { 2 }, null)), "#,##0.00")), "\r\n"));
						if (num20 == arrayList.Count - 1)
						{
							obj4 = Operators.ConcatenateObject(obj4, string.Concat(string.Concat(string.Concat("รวม = " + Conversions.ToString(num18), "\t"), "     รวมเง\u0e34น "), Strings.Format(num3, "#,##0.00")));
						}
						num20++;
					}
				}
				num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num7]["cin_room_pay_total"]));
				num5 = Conversions.ToDecimal(Operators.AddObject(num5, Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["cin_room_priceTotal"], dataSet2.Tables[0].Rows[num7]["cin_room_pay_total"])));
				Module1.localdata.ReportCustIN.AddReportCustINRow(Label2.Text, Conversions.ToString(num7 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_no"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_room_no"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_room_priceTotal"]), Conversions.ToString(num10), Conversions.ToString(Operators.AddObject(dataSet2.Tables[0].Rows[num7]["cin_room_priceTotal"], num10)), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["cin_room_in"]), "dd/MM/yyyy HH:mm"), string_, text2, Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), Strings.Format(decimal.Add(num, num2), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_cust_name"]), "", dataSet2.Tables[0].Rows[num7]["cin_by"].ToString(), Conversions.ToString(Operators.SubtractObject(Operators.AddObject(dataSet2.Tables[0].Rows[num7]["cin_room_priceTotal"], num10), dataSet2.Tables[0].Rows[num7]["cin_room_pay_total"])), Conversions.ToString(obj4), Conversions.ToString(obj), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["cin_room_price"]), Strings.Format(num3, "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["cin_room_priceTotal"], dataSet2.Tables[0].Rows[num7]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(num4, "#,##0.00"), Strings.Format(num5, "#,##0.00"));
				num7++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCustIn crystalReportCustIn = new CrystalReportCustIn();
			crystalReportCustIn.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCustIn;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		if (CheckBox1.Checked)
		{
			Label2.Text = "จากว\u0e31นท\u0e35\u0e48  " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy") + " 00:00:00 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker2.Value, "dd/MM/yy") + " 23:59:59";
		}
		else
		{
			Label2.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("จากว\u0e31นท\u0e35\u0e48  " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy"), " "), right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker2.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00"));
		}
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1_ValueChanged(null, null);
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType_Main order by name");
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
				if (num3 > num4)
				{
					break;
				}
				ComboBox1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			dataSet = Module1.connect("select * from HT_SET_CusType order by name");
			ComboBox2.Items.Clear();
			ComboBox2.Items.Add("");
			int num5 = dataSet.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				ComboBox2.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["name"]));
				num6++;
			}
			dataSet = Module1.connect("select * from HT_SET_RoomType order by name");
			ComboBox3.Items.Clear();
			ComboBox3.Items.Add("");
			int num8 = dataSet.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 <= num4)
				{
					ComboBox3.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["name"]));
					num9++;
					continue;
				}
				break;
			}
		}
	}
}
