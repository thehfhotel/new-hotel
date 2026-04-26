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
public class ReportCustDays : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

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

	[DebuggerNonUserCode]
	static ReportCustDays()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportCustDays()
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
		this.Button1 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button3 = new System.Windows.Forms.Button();
		this.SuspendLayout();
		System.Windows.Forms.Button button = this.Button1;
		System.Drawing.Point location = new System.Drawing.Point(12, 15);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		System.Drawing.Size size = new System.Drawing.Size(203, 54);
		button2.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.Text = "รายงานตามห\u0e49อง\r\n(แสดงเฉพาะช\u0e37\u0e48อห\u0e31วบ\u0e31ตรลงทะเบ\u0e35ยน)";
		this.Button1.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(12, 75);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(203, 54);
		button4.Size = size;
		this.Button2.TabIndex = 3;
		this.Button2.Text = "รายงานตามช\u0e37\u0e48อ\r\n(รายช\u0e37\u0e48อเพ\u0e34\u0e48มเต\u0e34ม)";
		this.Button2.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button5 = this.Button3;
		location = new System.Drawing.Point(12, 135);
		button5.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button6 = this.Button3;
		size = new System.Drawing.Size(203, 54);
		button6.Size = size;
		this.Button3.TabIndex = 4;
		this.Button3.Text = "รายงานเฉพาะชาวต\u0e48างชาต\u0e34";
		this.Button3.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(228, 201);
		this.ClientSize = size;
		this.Controls.Add(this.Button3);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.Button1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportCustDays";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานแขกท\u0e35\u0e48อย\u0e39\u0e48ในโรงแรม";
		this.ResumeLayout(false);
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		Module1.connect("select * from TB_SETTINGS");
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_room_status<>'Check-Out' order by cin_room_no");
		Module1.localdata.ReportDays.Rows.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
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
				string string_ = "ย\u0e31งไม\u0e48ค\u0e37นห\u0e49อง";
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num4]["cin_room_status"], "Check-Out", TextCompare: false))
				{
					string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["cin_room_out"]), "dd/MM/yyyy HH:mm");
				}
				num = decimal.Add(num, 1m);
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet.Tables[0].Rows[num4]["cin_room_priceTotal"]));
				Module1.localdata.ReportDays.AddReportDaysRow(Strings.Format(DateTime.Now, "dd/MM/yyyy HH:mm"), Conversions.ToString(num4 + 1), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_no"]), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["cin_room_in"]), "dd/MM/yyyy HH:mm"), string_, Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_priceTotal"]), "", Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_priceTotal"]), Conversions.ToString(num), Strings.Format(num2, "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_cust_name"]), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_night"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["cin_room_out"]), "dd/MM/yyyy HH:mm"), "", "", "", "", "", "");
				num4++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCustStay crystalReportCustStay = new CrystalReportCustStay();
			crystalReportCustStay.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCustStay;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1_ValueChanged(null, null);
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Module1.connect("select * from TB_SETTINGS");
		DataSet dataSet = Module1.connect("select * from View_people where cin_no in ( select cin_no from HT_CheckIn_Ds where Cin_room_status<>'Check-Out') order by cin_name");
		Module1.localdata.ReportDays.Rows.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
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
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet.Tables[0].Rows[num4]["total_price_net"]));
				Module1.localdata.ReportDays.AddReportDaysRow(Strings.Format(DateTime.Now, "dd/MM/yyyy HH:mm"), Conversions.ToString(num4 + 1), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_all"]), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_contry"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["cin_date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["total_price_net"]), "#,##0.00"), "", Conversions.ToString(dataSet.Tables[0].Rows[num4]["total_price_net"]), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_all"]), Strings.Format(num2, "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_name"]), "", "", "", "", "", "", "", "");
				num4++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCustStay2 crystalReportCustStay = new CrystalReportCustStay2();
			crystalReportCustStay.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCustStay;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		Module1.connect("select * from TB_SETTINGS");
		DataSet dataSet = Module1.connect("select * from View_people where Cin_foreign='True' and cin_no in ( select cin_no from HT_CheckIn_Ds where Cin_room_status<>'Check-Out') order by cin_name");
		Module1.localdata.ReportDays.Rows.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
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
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet.Tables[0].Rows[num4]["total_price_net"]));
				Module1.localdata.ReportDays.AddReportDaysRow(Strings.Format(DateTime.Now, "dd/MM/yyyy HH:mm"), Conversions.ToString(num4 + 1), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_all"]), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_contry"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["cin_date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num4]["total_price_net"]), "#,##0.00"), "", Conversions.ToString(dataSet.Tables[0].Rows[num4]["total_price_net"]), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_room_all"]), Strings.Format(num2, "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[num4]["cin_name"]), "", "", "", "", "", "", "", "");
				num4++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCustStay2 crystalReportCustStay = new CrystalReportCustStay2();
			crystalReportCustStay.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCustStay;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}
}
