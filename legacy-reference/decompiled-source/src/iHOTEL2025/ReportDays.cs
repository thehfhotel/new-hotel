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
public class ReportDays : Office2007Form
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

	[DebuggerNonUserCode]
	static ReportDays()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportDays()
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
		location = new System.Drawing.Point(279, 29);
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
		location = new System.Drawing.Point(65, 58);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(31, 16);
		label4.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(384, 86);
		this.ClientSize = size;
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportDays";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานประจำว\u0e31น";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_CheckIn_Ds where Cin_room_in between '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":01' and '"), DateTimePicker1.Value.AddDays(1.0).Date), " "), right2), ":00' order by cin_room_no")));
		Module1.localdata.ReportDays.Rows.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		checked
		{
			int num3 = dataSet2.Tables[0].Rows.Count - 1;
			int num4 = 0;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 > num6)
				{
					break;
				}
				string string_ = "ย\u0e31งไม\u0e48ค\u0e37นห\u0e49อง";
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num4]["cin_room_status"], "Check-Out", TextCompare: false))
				{
					string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num4]["cin_room_out"]), "dd/MM/yyyy HH:mm");
				}
				num = decimal.Add(num, 1m);
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num4]["cin_room_priceTotal"]));
				Module1.localdata.ReportDays.AddReportDaysRow(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Strings.Format(DateTimePicker1.Value, "dd/MM/yyyy") + " จากเวลา ", right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker1.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00")), Conversions.ToString(num4 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num4]["cin_room_no"]), Conversions.ToString(dataSet2.Tables[0].Rows[num4]["cin_room_type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num4]["cin_room_in"]), "dd/MM/yyyy HH:mm"), string_, Conversions.ToString(dataSet2.Tables[0].Rows[num4]["cin_room_priceTotal"]), "", Conversions.ToString(dataSet2.Tables[0].Rows[num4]["cin_room_priceTotal"]), Conversions.ToString(num), Strings.Format(num2, "#,##0.00"), "", Conversions.ToString(dataSet2.Tables[0].Rows[num4]["cin_room_night"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num4]["cin_room_out"]), "dd/MM/yyyy HH:mm"), "", "", "", "", "", "");
				num4++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportDays crystalReportDays = new CrystalReportDays();
			crystalReportDays.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportDays;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		Label2.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("จากเวลา ", right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker1.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00"));
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1_ValueChanged(null, null);
	}
}
