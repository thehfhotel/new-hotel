using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using CrystalDecisions.CrystalReports.Engine;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class ReportTax : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox3")]
	private GroupBox _GroupBox3;

	[AccessedThroughProperty("Button7")]
	private Button _Button7;

	[AccessedThroughProperty("DateTimePicker6")]
	private DateTimePicker _DateTimePicker6;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	internal virtual GroupBox GroupBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox3 = value;
		}
	}

	internal virtual Button Button7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button7_Click;
			if (_Button7 != null)
			{
				_Button7.Click -= value2;
			}
			_Button7 = value;
			if (_Button7 != null)
			{
				_Button7.Click += value2;
			}
		}
	}

	internal virtual DateTimePicker DateTimePicker6
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DateTimePicker6 = value;
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

	[DebuggerNonUserCode]
	static ReportTax()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportTax()
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
		this.GroupBox3 = new System.Windows.Forms.GroupBox();
		this.Button7 = new System.Windows.Forms.Button();
		this.DateTimePicker6 = new System.Windows.Forms.DateTimePicker();
		this.Label7 = new System.Windows.Forms.Label();
		this.GroupBox3.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox3.Controls.Add(this.Button7);
		this.GroupBox3.Controls.Add(this.DateTimePicker6);
		this.GroupBox3.Controls.Add(this.Label7);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox3;
		System.Drawing.Point location = new System.Drawing.Point(12, 12);
		groupBox.Location = location;
		this.GroupBox3.Name = "GroupBox3";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox3;
		System.Drawing.Size size = new System.Drawing.Size(472, 71);
		groupBox2.Size = size;
		this.GroupBox3.TabIndex = 4;
		this.GroupBox3.TabStop = false;
		this.GroupBox3.Text = "รายงานภาษ\u0e35ขาย";
		System.Windows.Forms.Button button = this.Button7;
		location = new System.Drawing.Point(259, 24);
		button.Location = location;
		this.Button7.Name = "Button7";
		System.Windows.Forms.Button button2 = this.Button7;
		size = new System.Drawing.Size(121, 33);
		button2.Size = size;
		this.Button7.TabIndex = 2;
		this.Button7.Text = "ออกรายงาน";
		this.Button7.UseVisualStyleBackColor = true;
		this.DateTimePicker6.CustomFormat = "MMMM yyyy";
		this.DateTimePicker6.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker6;
		location = new System.Drawing.Point(80, 29);
		dateTimePicker.Location = location;
		this.DateTimePicker6.Name = "DateTimePicker6";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker6;
		size = new System.Drawing.Size(161, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker6.TabIndex = 1;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label = this.Label7;
		location = new System.Drawing.Point(15, 32);
		label.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label2 = this.Label7;
		size = new System.Drawing.Size(45, 16);
		label2.Size = size;
		this.Label7.TabIndex = 0;
		this.Label7.Text = "เด\u0e37อน :";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(496, 97);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox3);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportTax";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานภาษ\u0e35ขาย";
		this.GroupBox3.ResumeLayout(false);
		this.GroupBox3.PerformLayout();
		this.ResumeLayout(false);
	}

	private void Button7_Click(object sender, EventArgs e)
	{
		Cursor = Cursors.WaitCursor;
		int num = 17;
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object left = "SELECT * from HT_Receipt_H where (receipt_no like 'B%' or receipt_no like 'SB%') and (receipt_date between '" + Strings.Format(DateTimePicker6.Value.Date, "MM/01/yyyy") + " 00:00:00' and '" + Strings.Format(DateTimePicker6.Value.Date, "MM/" + Conversions.ToString(DateTime.DaysInMonth(DateTimePicker6.Value.Date.Year, DateTimePicker6.Value.Date.Month)) + "/yyyy") + " 23:59:59') ";
		left = Operators.ConcatenateObject(left, " order by receipt_no");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(left));
		Module1.localdata.ReportVat.Rows.Clear();
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		decimal num4 = default(decimal);
		int num5 = 0;
		MyProject.Application.ChangeCulture("th-TH");
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
				num5++;
				object obj = "";
				if (num5 == num + 1)
				{
					num5 = 0;
					obj = "1";
				}
				RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["receipt_name"]);
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num7]["status_name"], "ยกเล\u0e34ก", TextCompare: false))
				{
					Module1.localdata.ReportVat.AddReportVatRow(Conversions.ToString(num7 + 1), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["receipt_date"]), "dd-MM-yy"), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["receipt_no"]), "ยกเล\u0e34ก", "-", "-", "-", "-", Strings.Format(Math.Floor(num2), "#,##0"), Strings.Format(decimal.Subtract(num2, Math.Floor(num2)), "0.00").Replace("0.", "").Replace("00", "-"), Strings.Format(Math.Floor(num3), "#,##0"), Strings.Format(decimal.Subtract(num3, Math.Floor(num3)), "0.00").Replace("0.", "").Replace("00", "-"), Conversions.ToString(obj), Strings.Format(DateTimePicker6.Value.Date, "MMMM"), Conversions.ToString(DateTimePicker6.Value.Date.Year + 543), "-", "-", Strings.Format(Math.Floor(num4), "#,##0"), Strings.Format(decimal.Subtract(num4, Math.Floor(num4)), "0.00").Replace("0.", "").Replace("00", "-"), "-", "-", dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString());
				}
				else
				{
					string text = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["receipt_name"]);
					string text2 = "";
					if (text.IndexOf("(สำน\u0e31กงานใหญ\u0e48)") != -1)
					{
						text2 = "สำน\u0e31กงานใหญ\u0e48";
						text = text.Replace("(สำน\u0e31กงานใหญ\u0e48)", "");
					}
					if (text.IndexOf("( สำน\u0e31กงานใหญ\u0e48 )") != -1)
					{
						text2 = "สำน\u0e31กงานใหญ\u0e48";
						text = text.Replace("( สำน\u0e31กงานใหญ\u0e48 )", "");
					}
					if (text.IndexOf(" สำน\u0e31กงานใหญ\u0e48") != -1)
					{
						text2 = "สำน\u0e31กงานใหญ\u0e48";
						text = text.Replace(" สำน\u0e31กงานใหญ\u0e48", "");
					}
					if (Operators.CompareString(text2, "", TextCompare: false) == 0)
					{
						if (text.IndexOf("(สาขา") != -1)
						{
							text2 = text.Substring(text.IndexOf("(สาขา")).Replace("(", "").Replace(")", "");
							text = text.Substring(0, text.IndexOf("(สาขา"));
						}
						if (text.IndexOf("( สาขา") != -1)
						{
							text2 = text.Substring(text.IndexOf("( สาขา")).Replace("(", "").Replace(")", "");
							text = text.Substring(0, text.IndexOf("( สาขา"));
						}
						if (text.IndexOf(" สาขา") != -1)
						{
							text2 = text.Substring(text.IndexOf(" สาขา")).Replace("(", "").Replace(")", "");
							text = text.Substring(0, text.IndexOf(" สาขา"));
						}
					}
					num2 = Conversions.ToDecimal(Operators.AddObject(num2, Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["receipt_total"], dataSet2.Tables[0].Rows[num7]["receipt_vat"])));
					num3 = Conversions.ToDecimal(Operators.AddObject(num3, dataSet2.Tables[0].Rows[num7]["receipt_vat"]));
					num4 = Conversions.ToDecimal(Operators.AddObject(num4, dataSet2.Tables[0].Rows[num7]["receipt_total"]));
					Datalocal.ReportVatDataTable reportVat = Module1.localdata.ReportVat;
					string string_ = Conversions.ToString(num7 + 1);
					string string_2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["receipt_date"]), "dd-MM-yy");
					string string_3 = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["receipt_no"]);
					string string_4 = text;
					string string_5 = Strings.Format(RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(null, typeof(Math), "Floor", new object[1] { Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["receipt_total"], dataSet2.Tables[0].Rows[num7]["receipt_vat"]) }, null, null, null)), "#,##0");
					string string_6 = Strings.Format(Operators.SubtractObject(Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["receipt_total"], dataSet2.Tables[0].Rows[num7]["receipt_vat"]), NewLateBinding.LateGet(null, typeof(Math), "Floor", new object[1] { Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["receipt_total"], dataSet2.Tables[0].Rows[num7]["receipt_vat"]) }, null, null, null)), "0.00").Replace("0.", "").Replace("00", "-");
					Type typeFromHandle = typeof(Math);
					object[] array = new object[1];
					DataRow dataRow = dataSet2.Tables[0].Rows[num7];
					string columnName = "receipt_vat";
					array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[1] { true };
					object obj2 = NewLateBinding.LateGet(null, typeFromHandle, "Floor", array2, null, null, array3);
					if (array3[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
					}
					string string_7 = Strings.Format(RuntimeHelpers.GetObjectValue(obj2), "#,##0");
					object left2 = dataSet2.Tables[0].Rows[num7]["receipt_vat"];
					Type typeFromHandle2 = typeof(Math);
					object[] array4 = new object[1];
					DataRow dataRow2 = dataSet2.Tables[0].Rows[num7];
					string columnName2 = "receipt_vat";
					array4[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName2]);
					object[] array5 = array4;
					bool[] array6 = new bool[1] { true };
					object right = NewLateBinding.LateGet(null, typeFromHandle2, "Floor", array5, null, null, array6);
					if (array6[0])
					{
						dataRow2[columnName2] = RuntimeHelpers.GetObjectValue(array5[0]);
					}
					string string_8 = Strings.Format(Operators.SubtractObject(left2, right), "0.00").Replace("0.", "").Replace("00", "-");
					string string_9 = Strings.Format(Math.Floor(num2), "#,##0");
					string string_10 = Strings.Format(decimal.Subtract(num2, Math.Floor(num2)), "0.00").Replace("0.", "").Replace("00", "-");
					string string_11 = Strings.Format(Math.Floor(num3), "#,##0");
					string string_12 = Strings.Format(decimal.Subtract(num3, Math.Floor(num3)), "0.00").Replace("0.", "").Replace("00", "-");
					string newpage = Conversions.ToString(obj);
					string string_13 = Strings.Format(DateTimePicker6.Value.Date, "MMMM");
					string string_14 = Conversions.ToString(DateTimePicker6.Value.Date.Year + 543);
					Type typeFromHandle3 = typeof(Math);
					object[] array7 = new object[1];
					DataRow dataRow3 = dataSet2.Tables[0].Rows[num7];
					string columnName3 = "receipt_total";
					array7[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName3]);
					object[] array8 = array7;
					bool[] array9 = new bool[1] { true };
					object obj3 = NewLateBinding.LateGet(null, typeFromHandle3, "Floor", array8, null, null, array9);
					if (array9[0])
					{
						dataRow3[columnName3] = RuntimeHelpers.GetObjectValue(array8[0]);
					}
					string string_15 = Strings.Format(RuntimeHelpers.GetObjectValue(obj3), "#,##0");
					object left3 = dataSet2.Tables[0].Rows[num7]["receipt_total"];
					Type typeFromHandle4 = typeof(Math);
					object[] array10 = new object[1];
					DataRow dataRow4 = dataSet2.Tables[0].Rows[num7];
					string columnName4 = "receipt_total";
					array10[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName4]);
					object[] array11 = array10;
					bool[] array12 = new bool[1] { true };
					object right2 = NewLateBinding.LateGet(null, typeFromHandle4, "Floor", array11, null, null, array12);
					if (array12[0])
					{
						dataRow4[columnName4] = RuntimeHelpers.GetObjectValue(array11[0]);
					}
					reportVat.AddReportVatRow(string_, string_2, string_3, string_4, string_5, string_6, string_7, string_8, string_9, string_10, string_11, string_12, newpage, string_13, string_14, string_15, Strings.Format(Operators.SubtractObject(left3, right2), "0.00").Replace("0.", "").Replace("00", "-"), Strings.Format(Math.Floor(num4), "#,##0"), Strings.Format(decimal.Subtract(num4, Math.Floor(num4)), "0.00").Replace("0.", "").Replace("00", "-"), dataSet2.Tables[0].Rows[num7]["receipt_tax"].ToString(), text2, dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString());
				}
				num7++;
			}
			if (num5 == num + 1)
			{
				num5 = 0;
			}
			if (num5 > 0)
			{
				int num10 = num5 + 1;
				int num11 = num;
				int num12 = num10;
				while (true)
				{
					int num13 = num12;
					int num9 = num11;
					if (num13 > num9)
					{
						break;
					}
					Module1.localdata.ReportVat.AddReportVatRow("", "", "", "", "", "", "", "", Strings.Format(Math.Floor(num2), "#,##0"), Strings.Format(decimal.Subtract(num2, Math.Floor(num2)), "0.00").Replace("0.", "").Replace("00", "-"), Strings.Format(Math.Floor(num3), "#,##0"), Strings.Format(decimal.Subtract(num3, Math.Floor(num3)), "0.00").Replace("0.", "").Replace("00", "-"), "", "", "", "", "", Strings.Format(Math.Floor(num4), "#,##0"), Strings.Format(decimal.Subtract(num4, Math.Floor(num4)), "0.00").Replace("0.", "").Replace("00", "-"), "", "", dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString());
					num12++;
				}
			}
			MyProject.Application.ChangeCulture("en-US");
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (File.Exists(Module1.Path_Program + "reports/ReportSaleVat.rpt"))
			{
				ReportDocument reportDocument = new ReportDocument();
				reportDocument.Load(Module1.Path_Program + "reports/ReportSaleVat.rpt");
				reportDocument.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else if (File.Exists(Module1.Path_Program + "ReportSaleVat.rpt"))
			{
				ReportDocument reportDocument2 = new ReportDocument();
				reportDocument2.Load(Module1.Path_Program + "/ReportSaleVat.rpt");
				reportDocument2.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument2;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else
			{
				ReportSaleVat reportSaleVat = new ReportSaleVat();
				reportSaleVat.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportSaleVat;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			Cursor = Cursors.Default;
		}
	}
}
