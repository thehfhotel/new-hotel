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
public class ReportContnueRoom2 : Office2007Form
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

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("TextBox_EMP")]
	private TextBox _TextBox_EMP;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("TextBox_CUST")]
	private TextBox _TextBox_CUST;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

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

	[DebuggerNonUserCode]
	static ReportContnueRoom2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportContnueRoom2()
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
		this.Label3 = new System.Windows.Forms.Label();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.TextBox_EMP = new System.Windows.Forms.TextBox();
		this.Label6 = new System.Windows.Forms.Label();
		this.TextBox_CUST = new System.Windows.Forms.TextBox();
		this.Label5 = new System.Windows.Forms.Label();
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
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(184, 188);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(90, 23);
		button2.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.Text = "ออกรายงาน";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(71, 94);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(31, 16);
		label4.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(27, 62);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(45, 16);
		label6.Size = size;
		this.Label3.TabIndex = 0;
		this.Label3.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker2;
		location = new System.Drawing.Point(74, 58);
		dateTimePicker3.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker2;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker2.TabIndex = 1;
		System.Windows.Forms.TextBox textBox_EMP = this.TextBox_EMP;
		location = new System.Drawing.Point(74, 155);
		textBox_EMP.Location = location;
		this.TextBox_EMP.Name = "TextBox_EMP";
		System.Windows.Forms.TextBox textBox_EMP2 = this.TextBox_EMP;
		size = new System.Drawing.Size(200, 23);
		textBox_EMP2.Size = size;
		this.TextBox_EMP.TabIndex = 12;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label6;
		location = new System.Drawing.Point(2, 158);
		label7.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label8 = this.Label6;
		size = new System.Drawing.Size(69, 16);
		label8.Size = size;
		this.Label6.TabIndex = 11;
		this.Label6.Text = "ช\u0e37\u0e48อพน\u0e31กงาน";
		System.Windows.Forms.TextBox textBox_CUST = this.TextBox_CUST;
		location = new System.Drawing.Point(74, 126);
		textBox_CUST.Location = location;
		this.TextBox_CUST.Name = "TextBox_CUST";
		System.Windows.Forms.TextBox textBox_CUST2 = this.TextBox_CUST;
		size = new System.Drawing.Size(200, 23);
		textBox_CUST2.Size = size;
		this.TextBox_CUST.TabIndex = 13;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(17, 130);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(54, 16);
		label10.Size = size;
		this.Label5.TabIndex = 10;
		this.Label5.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(288, 229);
		this.ClientSize = size;
		this.Controls.Add(this.TextBox_EMP);
		this.Controls.Add(this.Label6);
		this.Controls.Add(this.TextBox_CUST);
		this.Controls.Add(this.Label5);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.DateTimePicker2);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportContnueRoom";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานห\u0e49องพ\u0e31กต\u0e48อ";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		checked
		{
			if (DateTime.Compare(DateTimePicker1.Value.Date, DateTimePicker2.Value.Date) == 0)
			{
				DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
				object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
				object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
				string text = "";
				if (Operators.CompareString(TextBox_CUST.Text, "", TextCompare: false) != 0 && Operators.CompareString(text, "", TextCompare: false) == 0)
				{
					text = text + " and cin_cust_name like '%" + TextBox_CUST.Text + "%'";
				}
				if (Operators.CompareString(TextBox_EMP.Text, "", TextCompare: false) != 0 && Operators.CompareString(text, "", TextCompare: false) == 0)
				{
					text = text + " and cin_by like '%" + TextBox_EMP.Text + "%'";
				}
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_CheckIn_Ds where Cin_room_in <= '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":01' and cin_room_out >= '"), DateTimePicker1.Value.AddDays(1.0).Date), " "), right2), ":00' "), text), " order by cin_room_no")));
				Module1.localdata.ReportDays.Rows.Clear();
				decimal num = default(decimal);
				decimal num2 = default(decimal);
				decimal num3 = default(decimal);
				decimal num4 = default(decimal);
				ArrayList arrayList = new ArrayList();
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
					int num9 = -1;
					int num10 = arrayList.Count - 1;
					int num11 = 0;
					while (true)
					{
						int num12 = num11;
						num8 = num10;
						if (num12 > num8)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num11], new object[1] { 0 }, null), dataSet2.Tables[0].Rows[num6]["cin_room_type"], TextCompare: false))
						{
							num9 = num11;
						}
						num11++;
					}
					if (num9 == -1)
					{
						string[] value = new string[2]
						{
							Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_type"]),
							"1"
						};
						arrayList.Add(value);
					}
					else
					{
						NewLateBinding.LateIndexSetComplex(arrayList[num9], new object[2]
						{
							1,
							Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList[num9], new object[1] { 1 }, null)) + 1
						}, null, OptimisticSet: false, RValueBase: true);
					}
					object obj = "";
					if (num6 == dataSet2.Tables[0].Rows.Count - 1)
					{
						int num13 = 0;
						int num14 = arrayList.Count - 1;
						int num15 = 0;
						while (true)
						{
							int num16 = num15;
							num8 = num14;
							if (num16 > num8)
							{
								break;
							}
							num13 += Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList[num15], new object[1] { 1 }, null));
							obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(arrayList[num15], new object[1] { 0 }, null), " = "), NewLateBinding.LateIndexGet(arrayList[num15], new object[1] { 1 }, null)), "\r\n"));
							if (num15 == arrayList.Count - 1)
							{
								obj = Operators.ConcatenateObject(obj, "รวม " + Conversions.ToString(num13));
							}
							num15++;
						}
					}
					string string_ = "ย\u0e31งไม\u0e48ค\u0e37นห\u0e49อง";
					if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num6]["cin_room_status"], "Check-Out", TextCompare: false))
					{
						string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_out"]), "dd/MM/yyyy HH:mm");
					}
					num = decimal.Add(num, 1m);
					num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"]));
					object obj2 = Strings.Format(DateTimePicker1.Value, "dd/MM/yyyy");
					if (num6 != 0)
					{
						obj2 = "\"";
					}
					num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]));
					num4 = Conversions.ToDecimal(Operators.AddObject(num4, Operators.SubtractObject(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"], dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"])));
					Module1.localdata.ReportDays.AddReportDaysRow(Strings.Format(DateTimePicker1.Value, "dd/MM/yyyy"), Conversions.ToString(num6 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_no"]), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_in"]), "dd/MM/yyyy HH:mm"), string_, Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"]), "", Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"]), Conversions.ToString(num), Strings.Format(num2, "#,##0.00"), "", Conversions.ToString(dataSet2.Tables[0].Rows[num6]["cin_room_night"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_out"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(obj), Conversions.ToString(obj2), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet2.Tables[0].Rows[num6]["cin_room_priceTotal"], dataSet2.Tables[0].Rows[num6]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(num3, "#,##0.00"), Strings.Format(num4, "#,##0.00"));
					num6++;
				}
				MyProject.Forms.FrmPrint.Close();
				MyProject.Forms.FrmPrint.Show();
				CrystalReportDaysContenue2 crystalReportDaysContenue = new CrystalReportDaysContenue2();
				crystalReportDaysContenue.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportDaysContenue;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				return;
			}
			object value2 = DateAndTime.DateDiff(DateInterval.Day, DateTimePicker1.Value.Date, DateTimePicker2.Value.Date);
			ArrayList arrayList2 = new ArrayList();
			DataSet dataSet3 = Module1.connect("select * from TB_SETTINGS");
			object right3 = Strings.Format(Conversions.ToInteger(dataSet3.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
			object right4 = Strings.Format(Conversions.ToInteger(dataSet3.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
			Module1.localdata.ReportDays.Rows.Clear();
			decimal num17 = default(decimal);
			decimal num18 = default(decimal);
			decimal num19 = default(decimal);
			decimal num20 = default(decimal);
			int num21 = 0;
			int num22 = Conversions.ToInteger(value2);
			int num23 = 0;
			while (true)
			{
				int num24 = num23;
				int num8 = num22;
				if (num24 > num8)
				{
					break;
				}
				DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_CheckIn_Ds where Cin_room_in <= '" + Conversions.ToString(DateTimePicker1.Value.AddDays(num23).Date), " "), right3), ":01' and cin_room_out >= '"), DateTimePicker1.Value.AddDays(num23 + 1).Date), " "), right4), ":00' order by cin_room_no")));
				int num25 = dataSet4.Tables[0].Rows.Count - 1;
				int num26 = 0;
				while (true)
				{
					int num27 = num26;
					num8 = num25;
					if (num27 > num8)
					{
						break;
					}
					num21++;
					int num28 = -1;
					int num29 = arrayList2.Count - 1;
					int num30 = 0;
					while (true)
					{
						int num31 = num30;
						num8 = num29;
						if (num31 > num8)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList2[num30], new object[1] { 0 }, null), dataSet4.Tables[0].Rows[num26]["cin_room_type"], TextCompare: false))
						{
							num28 = num30;
						}
						num30++;
					}
					if (num28 == -1)
					{
						string[] value3 = new string[2]
						{
							Conversions.ToString(dataSet4.Tables[0].Rows[num26]["cin_room_type"]),
							"1"
						};
						arrayList2.Add(value3);
					}
					else
					{
						NewLateBinding.LateIndexSetComplex(arrayList2[num28], new object[2]
						{
							1,
							Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList2[num28], new object[1] { 1 }, null)) + 1
						}, null, OptimisticSet: false, RValueBase: true);
					}
					object obj3 = "";
					if (num26 == dataSet4.Tables[0].Rows.Count - 1)
					{
						int num32 = 0;
						int num33 = arrayList2.Count - 1;
						int num34 = 0;
						while (true)
						{
							int num35 = num34;
							num8 = num33;
							if (num35 > num8)
							{
								break;
							}
							num32 += Conversions.ToInteger(NewLateBinding.LateIndexGet(arrayList2[num34], new object[1] { 1 }, null));
							obj3 = Operators.ConcatenateObject(obj3, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(arrayList2[num34], new object[1] { 0 }, null), " = "), NewLateBinding.LateIndexGet(arrayList2[num34], new object[1] { 1 }, null)), "\r\n"));
							if (num34 == arrayList2.Count - 1)
							{
								obj3 = Operators.ConcatenateObject(obj3, "รวม " + Conversions.ToString(num32));
							}
							num34++;
						}
					}
					string string_2 = "ย\u0e31งไม\u0e48ค\u0e37นห\u0e49อง";
					if (Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num26]["cin_room_status"], "Check-Out", TextCompare: false))
					{
						string_2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num26]["cin_room_out"]), "dd/MM/yyyy HH:mm");
					}
					num17 = decimal.Add(num17, 1m);
					num18 = Conversions.ToDecimal(Operators.AddObject(num18, dataSet4.Tables[0].Rows[num26]["cin_room_priceTotal"]));
					object obj4 = Strings.Format(DateTimePicker1.Value.AddDays(num23), "dd/MM/yyyy");
					if (num26 != 0)
					{
						obj4 = "\"";
					}
					num19 = Conversions.ToDecimal(Operators.AddObject(num19, dataSet4.Tables[0].Rows[num26]["cin_room_pay_total"]));
					num20 = Conversions.ToDecimal(Operators.AddObject(num20, Operators.SubtractObject(dataSet4.Tables[0].Rows[num26]["cin_room_priceTotal"], dataSet4.Tables[0].Rows[num26]["cin_room_pay_total"])));
					Module1.localdata.ReportDays.AddReportDaysRow(Strings.Format(DateTimePicker1.Value, "dd/MM/yyyy") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd/MM/yyyy"), Conversions.ToString(num21), Conversions.ToString(dataSet4.Tables[0].Rows[num26]["cin_room_no"]), Conversions.ToString(dataSet4.Tables[0].Rows[num26]["cin_room_type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num26]["cin_room_in"]), "dd/MM/yyyy HH:mm"), string_2, Conversions.ToString(dataSet4.Tables[0].Rows[num26]["cin_room_priceTotal"]), "", Conversions.ToString(dataSet4.Tables[0].Rows[num26]["cin_room_priceTotal"]), Conversions.ToString(num17), Strings.Format(num18, "#,##0.00"), "", Conversions.ToString(dataSet4.Tables[0].Rows[num26]["cin_room_night"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num26]["cin_room_out"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(obj3), Conversions.ToString(obj4), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num26]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet4.Tables[0].Rows[num26]["cin_room_priceTotal"], dataSet4.Tables[0].Rows[num26]["cin_room_pay_total"]), "#,##0.00"), Strings.Format(num19, "#,##0.00"), Strings.Format(num20, "#,##0.00"));
					num26++;
				}
				num23++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportDaysContenue2 crystalReportDaysContenue2 = new CrystalReportDaysContenue2();
			crystalReportDaysContenue2.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportDaysContenue2;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		Label2.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("จากว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy"), " "), right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker2.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00"));
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1_ValueChanged(null, null);
	}
}
