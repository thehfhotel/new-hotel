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
public class ReportCustOutToday2 : Office2007Form
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

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

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
	static ReportCustOutToday2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportCustOutToday2()
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
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(29, 33);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(31, 16);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(63, 29);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 1;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(492, 28);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(90, 50);
		button2.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.Text = "ออกรายงาน";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(65, 94);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(31, 16);
		label4.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker2;
		location = new System.Drawing.Point(63, 58);
		dateTimePicker3.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker2;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker2.TabIndex = 7;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(15, 62);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(45, 16);
		label6.Size = size;
		this.Label3.TabIndex = 6;
		this.Label3.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48";
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(336, 28);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(150, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 9;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(270, 32);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(62, 16);
		label8.Size = size;
		this.Label4.TabIndex = 8;
		this.Label4.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(588, 132);
		this.ClientSize = size;
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
		this.Name = "ReportCustOutToday";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานแขกท\u0e35\u0e48กำล\u0e31งจะออกว\u0e31นน\u0e35\u0e49";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		string right3 = "";
		if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) != 0)
		{
			right3 = " and cust_type_main='" + ComboBox1.Text + "'";
		}
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_CheckIn_Ds where (Cin_room_out between '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":01' and '"), DateTimePicker2.Value.AddDays(1.0).Date), " "), right2), ":00') and cin_room_status<>'Check-Out' "), right3), " order by Cin_room_out")));
		DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from HT_CheckIn_Product where cin_no in (select cin_no from View_CheckIn_Ds where (Cin_room_out between '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":01' and '"), DateTimePicker2.Value.AddDays(1.0).Date), " "), right2), ":00') and cin_room_status<>'Check-Out' "), right3), ")")));
		Module1.localdata.ReportCustIN.Rows.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		checked
		{
			int num5 = dataSet2.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num8 = num5;
				if (num7 > num8)
				{
					break;
				}
				object obj = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_out"]), "dd/MM/yyyy HH:mm");
				string string_ = "";
				string text = "";
				decimal num9 = default(decimal);
				int num10 = dataSet3.Tables[0].Rows.Count - 1;
				int num11 = 0;
				while (true)
				{
					int num12 = num11;
					num8 = num10;
					if (num12 > num8)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet3.Tables[0].Rows[num11]["cin_room_no"], dataSet2.Tables[0].Rows[num6]["cin_room_no"], TextCompare: false))
					{
						text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num11]["cin_pro_name"], " ")));
						num9 = Conversions.ToDecimal(Operators.AddObject(num9, dataSet3.Tables[0].Rows[num11]["cin_pro_pricetotal"]));
					}
					num11++;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"]));
				num2 = decimal.Add(num2, num9);
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num6]["total_price_balance"], 0, TextCompare: false))
				{
					string_ = "จ\u0e48ายครบ";
				}
				else if (Operators.ConditionalCompareObjectGreater(dataSet2.Tables[0].Rows[num6]["total_price_balance"], 0, TextCompare: false))
				{
					string_ = Conversions.ToString(Operators.ConcatenateObject("ค\u0e49างจ\u0e48าย ", dataSet2.Tables[0].Rows[num6]["total_price_balance"]));
				}
				else if (Operators.ConditionalCompareObjectLess(dataSet2.Tables[0].Rows[num6]["total_price_balance"], 0, TextCompare: false))
				{
					Type typeFromHandle = typeof(Math);
					object[] array = new object[1];
					DataRow dataRow = dataSet2.Tables[0].Rows[num6];
					string columnName = "total_price_balance";
					array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[1] { true };
					object right4 = NewLateBinding.LateGet(null, typeFromHandle, "Abs", array2, null, null, array3);
					if (array3[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
					}
					string_ = Conversions.ToString(Operators.ConcatenateObject("จ\u0e48ายเก\u0e34น ", right4));
				}
				num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]));
				num4 = Conversions.ToDecimal(Operators.AddObject(num4, Operators.SubtractObject(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"], dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"])));
				Module1.localdata.ReportCustIN.AddReportCustINRow(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Strings.Format(DateTimePicker1.Value, "dd/MM/yyyy") + " จากเวลา ", right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker1.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00")), Conversions.ToString(num6 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_no"]), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_no"]), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"]), Conversions.ToString(num9), Conversions.ToString(Operators.AddObject(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"], num9)), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_in"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(obj), text, Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), Strings.Format(decimal.Add(num, num2), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_cust_name"]), string_, "", Strings.Format(Operators.SubtractObject(Operators.AddObject(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"], num9), dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]), "#,##0.00"), "", "", "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"], dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(num3, "#,##0.00"), Strings.Format(num4, "#,##0.00"));
				num6++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCustOutTodayHousewife crystalReportCustOutTodayHousewife = new CrystalReportCustOutTodayHousewife();
			crystalReportCustOutTodayHousewife.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCustOutTodayHousewife;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		Label2.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("จากว\u0e31นท\u0e35\u0e48  " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy"), " "), right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker2.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00"));
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1.Value = DateTime.Now;
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
				if (num3 <= num4)
				{
					ComboBox1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}
}
