using System;
using System.Collections;
using System.Data;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using CrystalDecisions.CrystalReports.Engine;
using CrystalDecisions.Shared;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[StandardModule]
internal sealed class Print_Report
{
	public static PrintDialog PrintSet;

	static Print_Report()
	{
		Class2.LH6iGfYz9j3MJ();
		PrintSet = new PrintDialog();
	}

	public static bool ShowPrinter()
	{
		if (Directory.Exists("C:\\Program Files (x86)") | Directory.Exists("D:\\Program Files (x86)"))
		{
			PrintSet.UseEXDialog = true;
		}
		else
		{
			PrintSet.UseEXDialog = false;
		}
		if (PrintSet.ShowDialog() == DialogResult.OK)
		{
			return true;
		}
		return false;
	}

	public static void Print_Sale(string id, bool preview)
	{
		preview = true;
		int num = 1;
		int num2 = 9;
		object value = 0;
		if (Operators.CompareString(Module1.Receipt_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) != 0)
		{
			num2 = 100;
		}
		if ((Operators.CompareString(Module1.Receipt_Report, "HHOTEL", TextCompare: false) == 0) | (Operators.CompareString(Module1.Receipt_Report, "FOLIO", TextCompare: false) == 0))
		{
			num2 = 9;
		}
		DataSet dataSet = Module1.connect("select * from view_pay_ds where pay_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		if (((Operators.CompareString(Module1.Receipt_Report, "HHOTEL", TextCompare: false) == 0) | (Operators.CompareString(Module1.Receipt_Report, "FOLIO", TextCompare: false) == 0) | (Operators.CompareString(Module1.Receipt_Report, "Guest Folio", TextCompare: false) == 0)) & (dataSet.Tables[0].Rows[0]["Cin_no"].ToString().IndexOf("CH") != -1))
		{
			smethod_0(id, preview);
		}
		else if (Operators.CompareString(Module1.Receipt_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0 || Module1.Receipt_Report.IndexOf("เคร\u0e37\u0e48อง") != -1)
		{
			DataSet dataSet2 = Module1.connect("select * from TB_SETTINGS");
			MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
			FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
			object instance = new BinaryReader(fileStream);
			byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
			fileStream.Close();
			Module1.localdata.ReportBillCash.Rows.Clear();
			Module1.localdata.Bill_H.Rows.Clear();
			Module1.localdata.Bill_H.AddBill_HRow(dataSet2.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
			decimal num3 = default(decimal);
			checked
			{
				int num4 = dataSet.Tables[0].Rows.Count - 1;
				int num5 = 0;
				while (true)
				{
					int num6 = num5;
					int num7 = num4;
					if (num6 > num7)
					{
						break;
					}
					string text = "";
					text = ((!Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Name"], "ค\u0e48าห\u0e49อง", TextCompare: false)) ? Conversions.ToString(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Name"]) : Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Name"], " "), dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds"])));
					Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[num5]["Pay_no"]), Conversions.ToString(dataSet.Tables[0].Rows[num5]["Cin_Pay_note"]), dataSet.Tables[0].Rows[num5]["Cust_name"].ToString(), dataSet.Tables[0].Rows[num5]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
						.Replace("ถนน ", "")
						.Replace("เขต/อำเภอ ", "")
						.Replace("แขวง/ตำบล ", "")
						.Replace("จ\u0e31งหว\u0e31ด ", ""), Conversions.ToString(num5 + 1), text, "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Price"]), "#,##0.00"), Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[num5]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[num5]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[num5]["Cin_Pay_tran"]), "#,##0.00"), "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Price"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", "ผ\u0e39\u0e49ส\u0e48ง", DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[num5]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[num5]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[num5]["Cin_Pay_tran"]), "0.00"))), null, "", "", "", "");
					if (num5 + 1 == num2 * num)
					{
						num++;
					}
					value = num5 + 1;
					num5++;
				}
				if ((Operators.CompareString(Module1.Receipt_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0) | (Operators.CompareString(Module1.Receipt_Report, "HHOTEL", TextCompare: false) == 0) | (Operators.CompareString(Module1.Receipt_Report, "FOLIO", TextCompare: false) == 0))
				{
					int num8 = Conversions.ToInteger(value);
					int num9 = num2 * num - 1;
					int num10 = num8;
					while (true)
					{
						int num11 = num10;
						int num7 = num9;
						if (num11 > num7)
						{
							break;
						}
						Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Pay_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Pay_no"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Pay_note"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["C_Address"]), "", "", "", "", "", "", Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[0]["Cin_Pay_tran"]), "#,##0.00"), "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Pay_Ds_Price"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", "ผ\u0e39\u0e49ส\u0e48ง", DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[0]["Cin_Pay_tran"]), "0.00"))), null, "", "", "", "");
						value = num10 + 1;
						num10++;
					}
				}
				if (Operators.CompareString(Module1.Receipt_preview, "เป\u0e34ด", TextCompare: false) == 0)
				{
					MyProject.Forms.FrmPrint.Close();
					MyProject.Forms.FrmPrint.Show();
					if (Operators.CompareString(Module1.Receipt_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
					{
						ReportDocument reportDocument = new ReportDocument();
						if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
						{
							reportDocument.Load(Module1.Path_Program + "reports/sale.rpt");
						}
						else
						{
							reportDocument.Load(Module1.Path_Program + "/sale.rpt");
							if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
							{
								reportDocument = new sale();
							}
						}
						reportDocument.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
						return;
					}
					if (Operators.CompareString(Module1.Receipt_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
					{
						ReportDocument reportDocument2 = new ReportDocument();
						if (File.Exists(Module1.Path_Program + "reports/sale2_58.rpt"))
						{
							reportDocument2.Load(Module1.Path_Program + "reports/sale2_58.rpt");
						}
						else
						{
							reportDocument2.Load(Module1.Path_Program + "/sale2_58.rpt");
							if (!File.Exists(Module1.Path_Program + "/sale2_58.rpt"))
							{
								reportDocument2 = new sale2_58();
							}
						}
						reportDocument2.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument2;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
						return;
					}
					if (Operators.CompareString(Module1.Receipt_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
					{
						ReportDocument reportDocument3 = new ReportDocument();
						if (File.Exists(Module1.Path_Program + "reports/sale2_80.rpt"))
						{
							reportDocument3.Load(Module1.Path_Program + "reports/sale2_80.rpt");
						}
						else
						{
							reportDocument3.Load(Module1.Path_Program + "/sale2_80.rpt");
							if (!File.Exists(Module1.Path_Program + "/sale2_80.rpt"))
							{
								reportDocument3 = new sale2_80();
							}
						}
						reportDocument3.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument3;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
						return;
					}
					ReportDocument reportDocument4 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
					{
						reportDocument4.Load(Module1.Path_Program + "reports/sale.rpt");
					}
					else
					{
						reportDocument4.Load(Module1.Path_Program + "/sale.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
						{
							reportDocument4 = new sale();
						}
					}
					reportDocument4.SetDataSource(Module1.localdata);
					MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument4;
					MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
					return;
				}
			}
			if (Operators.CompareString(MyProject.Forms.FrmSettings.Print1.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
			{
				if (!ShowPrinter())
				{
					return;
				}
				if (Operators.CompareString(Module1.Receipt_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
				{
					ReportDocument reportDocument5 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
					{
						reportDocument5.Load(Module1.Path_Program + "reports/sale.rpt");
					}
					else
					{
						reportDocument5.Load(Module1.Path_Program + "/sale.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
						{
							reportDocument5 = new sale();
						}
					}
					reportDocument5.SetDataSource(Module1.localdata);
					reportDocument5.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument5.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
					reportDocument5.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument5.Dispose();
					return;
				}
				if (Operators.CompareString(Module1.Receipt_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument6 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale2_58.rpt"))
					{
						reportDocument6.Load(Module1.Path_Program + "reports/sale2_58.rpt");
					}
					else
					{
						reportDocument6.Load(Module1.Path_Program + "/sale2_58.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale2_58.rpt"))
						{
							reportDocument6 = new sale2_58();
						}
					}
					reportDocument6.SetDataSource(Module1.localdata);
					reportDocument6.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument6.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument6.Dispose();
					return;
				}
				if (Operators.CompareString(Module1.Receipt_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument7 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale2_80.rpt"))
					{
						reportDocument7.Load(Module1.Path_Program + "reports/sale2_80.rpt");
					}
					else
					{
						reportDocument7.Load(Module1.Path_Program + "/sale2_80.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale2_80.rpt"))
						{
							reportDocument7 = new sale2_80();
						}
					}
					reportDocument7.SetDataSource(Module1.localdata);
					reportDocument7.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument7.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument7.Dispose();
					return;
				}
				ReportDocument reportDocument8 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
				{
					reportDocument8.Load(Module1.Path_Program + "reports/sale.rpt");
				}
				else
				{
					reportDocument8.Load(Module1.Path_Program + "/sale.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
					{
						reportDocument8 = new sale();
					}
				}
				reportDocument8.SetDataSource(Module1.localdata);
				reportDocument8.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument8.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument8.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument8.Dispose();
				return;
			}
			if (Operators.CompareString(Module1.Receipt_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
			{
				ReportDocument reportDocument9 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
				{
					reportDocument9.Load(Module1.Path_Program + "reports/sale.rpt");
				}
				else
				{
					reportDocument9.Load(Module1.Path_Program + "/sale.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
					{
						reportDocument9 = new sale();
					}
				}
				reportDocument9.SetDataSource(Module1.localdata);
				reportDocument9.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print1.Text;
				reportDocument9.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument9.PrintToPrinter(Module1.int_1, collated: true, 0, 0);
				reportDocument9.Dispose();
				return;
			}
			if (Operators.CompareString(Module1.Receipt_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument10 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale2_58.rpt"))
				{
					reportDocument10.Load(Module1.Path_Program + "reports/sale2_58.rpt");
				}
				else
				{
					reportDocument10.Load(Module1.Path_Program + "/sale2_58.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale2_58.rpt"))
					{
						reportDocument10 = new sale2_58();
					}
				}
				reportDocument10.SetDataSource(Module1.localdata);
				reportDocument10.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print1.Text;
				reportDocument10.PrintToPrinter(Module1.int_1, collated: true, 0, 0);
				reportDocument10.Dispose();
				return;
			}
			if (Operators.CompareString(Module1.Receipt_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument11 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale2_80.rpt"))
				{
					reportDocument11.Load(Module1.Path_Program + "reports/sale2_80.rpt");
				}
				else
				{
					reportDocument11.Load(Module1.Path_Program + "/sale2_80.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale2_80.rpt"))
					{
						reportDocument11 = new sale2_80();
					}
				}
				reportDocument11.SetDataSource(Module1.localdata);
				reportDocument11.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print1.Text;
				reportDocument11.PrintToPrinter(Module1.int_1, collated: true, 0, 0);
				reportDocument11.Dispose();
				return;
			}
			ReportDocument reportDocument12 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
			{
				reportDocument12.Load(Module1.Path_Program + "reports/sale.rpt");
			}
			else
			{
				reportDocument12.Load(Module1.Path_Program + "/sale.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
				{
					reportDocument12 = new sale();
				}
			}
			reportDocument12.SetDataSource(Module1.localdata);
			reportDocument12.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print1.Text;
			reportDocument12.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument12.PrintToPrinter(Module1.int_1, collated: true, 0, 0);
			reportDocument12.Dispose();
		}
		else
		{
			smethod_1(id, preview);
		}
	}

	public static void Print_SalePOS(string id, bool preview)
	{
		preview = true;
		int num = 1;
		int num2 = 9;
		object value = 0;
		if (Operators.CompareString(Module1.POS_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) != 0)
		{
			num2 = 100;
		}
		if ((Operators.CompareString(Module1.POS_Report, "HHOTEL", TextCompare: false) == 0) | (Operators.CompareString(Module1.POS_Report, "FOLIO", TextCompare: false) == 0))
		{
			num2 = 9;
		}
		DataSet dataSet = Module1.connect("select * from view_pay_ds where pay_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		if (((Operators.CompareString(Module1.POS_Report, "HHOTEL", TextCompare: false) == 0) | (Operators.CompareString(Module1.POS_Report, "FOLIO", TextCompare: false) == 0) | (Operators.CompareString(Module1.POS_Report, "Guest Folio", TextCompare: false) == 0)) & (dataSet.Tables[0].Rows[0]["Cin_no"].ToString().IndexOf("CH") != -1))
		{
			smethod_0(id, preview);
		}
		else if (Operators.CompareString(Module1.POS_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0 || Module1.POS_Report.IndexOf("เคร\u0e37\u0e48อง") != -1)
		{
			DataSet dataSet2 = Module1.connect("select * from TB_SETTINGS");
			MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
			FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
			object instance = new BinaryReader(fileStream);
			byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
			fileStream.Close();
			Module1.localdata.ReportBillCash.Rows.Clear();
			Module1.localdata.Bill_H.Rows.Clear();
			Module1.localdata.Bill_H.AddBill_HRow(dataSet2.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
			decimal num3 = default(decimal);
			checked
			{
				int num4 = dataSet.Tables[0].Rows.Count - 1;
				int num5 = 0;
				while (true)
				{
					int num6 = num5;
					int num7 = num4;
					if (num6 > num7)
					{
						break;
					}
					string text = "";
					text = ((!Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Name"], "ค\u0e48าห\u0e49อง", TextCompare: false)) ? Conversions.ToString(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Name"]) : Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Name"], " "), dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds"])));
					Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[num5]["Pay_no"]), Conversions.ToString(dataSet.Tables[0].Rows[num5]["Cin_Pay_note"]), dataSet.Tables[0].Rows[num5]["Cust_name"].ToString(), dataSet.Tables[0].Rows[num5]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
						.Replace("ถนน ", "")
						.Replace("เขต/อำเภอ ", "")
						.Replace("แขวง/ตำบล ", "")
						.Replace("จ\u0e31งหว\u0e31ด ", ""), Conversions.ToString(num5 + 1), text, "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Price"]), "#,##0.00"), Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[num5]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[num5]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[num5]["Cin_Pay_tran"]), "#,##0.00"), "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num5]["Cin_Pay_Ds_Price"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", "ผ\u0e39\u0e49ส\u0e48ง", DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[num5]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[num5]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[num5]["Cin_Pay_tran"]), "0.00"))), null, "", "", "", "");
					if (num5 + 1 == num2 * num)
					{
						num++;
					}
					value = num5 + 1;
					num5++;
				}
				if ((Operators.CompareString(Module1.POS_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0) | (Operators.CompareString(Module1.POS_Report, "HHOTEL", TextCompare: false) == 0) | (Operators.CompareString(Module1.POS_Report, "FOLIO", TextCompare: false) == 0))
				{
					int num8 = Conversions.ToInteger(value);
					int num9 = num2 * num - 1;
					int num10 = num8;
					while (true)
					{
						int num11 = num10;
						int num7 = num9;
						if (num11 > num7)
						{
							break;
						}
						Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Pay_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Pay_no"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Pay_note"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["C_Address"]), "", "", "", "", "", "", Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[0]["Cin_Pay_tran"]), "#,##0.00"), "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Pay_Ds_Price"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", "ผ\u0e39\u0e49ส\u0e48ง", DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[0]["Cin_Pay_tran"]), "0.00"))), null, "", "", "", "");
						value = num10 + 1;
						num10++;
					}
				}
				if (Operators.CompareString(Module1.POS_preview, "เป\u0e34ด", TextCompare: false) == 0)
				{
					MyProject.Forms.FrmPrint.Close();
					MyProject.Forms.FrmPrint.Show();
					if (Operators.CompareString(Module1.POS_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
					{
						sale sale2 = new sale();
						sale2.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = sale2;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
					}
					else if (Operators.CompareString(Module1.POS_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
					{
						sale2_58 sale2_81 = new sale2_58();
						sale2_81.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = sale2_81;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
					}
					else if (Operators.CompareString(Module1.POS_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
					{
						sale2_80 sale2_82 = new sale2_80();
						sale2_82.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = sale2_82;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
					}
					else
					{
						sale sale3 = new sale();
						sale3.SetDataSource(Module1.localdata);
						MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = sale3;
						MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
					}
					return;
				}
			}
			if (Operators.CompareString(MyProject.Forms.FrmSettings.print7.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
			{
				if (!ShowPrinter())
				{
					return;
				}
				if (Operators.CompareString(Module1.POS_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
				{
					ReportDocument reportDocument = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
						{
							reportDocument = new sale();
						}
					}
					reportDocument.SetDataSource(Module1.localdata);
					reportDocument.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
					reportDocument.PrintToPrinter(Module1.copy_POS, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument.Dispose();
					return;
				}
				if (Operators.CompareString(Module1.POS_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument2 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale2_58.rpt"))
					{
						reportDocument2.Load(Module1.Path_Program + "reports/sale2_58.rpt");
					}
					else
					{
						reportDocument2.Load(Module1.Path_Program + "/sale2_58.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale2_58.rpt"))
						{
							reportDocument2 = new sale2_58();
						}
					}
					reportDocument2.SetDataSource(Module1.localdata);
					reportDocument2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument2.PrintToPrinter(Module1.copy_POS, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument2.Dispose();
					return;
				}
				if (Operators.CompareString(Module1.POS_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument3 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/sale2_80.rpt"))
					{
						reportDocument3.Load(Module1.Path_Program + "reports/sale2_80.rpt");
					}
					else
					{
						reportDocument3.Load(Module1.Path_Program + "/sale2_80.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale2_80.rpt"))
						{
							reportDocument3 = new sale2_80();
						}
					}
					reportDocument3.SetDataSource(Module1.localdata);
					reportDocument3.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument3.PrintToPrinter(Module1.copy_POS, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument3.Dispose();
					return;
				}
				ReportDocument reportDocument4 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
				{
					reportDocument4.Load(Module1.Path_Program + "reports/sale.rpt");
				}
				else
				{
					reportDocument4.Load(Module1.Path_Program + "/sale.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
					{
						reportDocument4 = new sale();
					}
				}
				reportDocument4.SetDataSource(Module1.localdata);
				reportDocument4.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument4.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument4.PrintToPrinter(Module1.copy_POS, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument4.Dispose();
				return;
			}
			if (Operators.CompareString(Module1.POS_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
			{
				ReportDocument reportDocument5 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
				{
					reportDocument5.Load(Module1.Path_Program + "reports/sale.rpt");
				}
				else
				{
					reportDocument5.Load(Module1.Path_Program + "/sale.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
					{
						reportDocument5 = new sale();
					}
				}
				reportDocument5.SetDataSource(Module1.localdata);
				reportDocument5.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print7.Text;
				reportDocument5.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument5.PrintToPrinter(Module1.copy_POS, collated: true, 0, 0);
				reportDocument5.Dispose();
				return;
			}
			if (Operators.CompareString(Module1.POS_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument6 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale2_58.rpt"))
				{
					reportDocument6.Load(Module1.Path_Program + "reports/sale2_58.rpt");
				}
				else
				{
					reportDocument6.Load(Module1.Path_Program + "/sale2_58.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale2_58.rpt"))
					{
						reportDocument6 = new sale2_58();
					}
				}
				reportDocument6.SetDataSource(Module1.localdata);
				reportDocument6.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print7.Text;
				reportDocument6.PrintToPrinter(Module1.copy_POS, collated: true, 0, 0);
				reportDocument6.Dispose();
				return;
			}
			if (Operators.CompareString(Module1.POS_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument7 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale2_80.rpt"))
				{
					reportDocument7.Load(Module1.Path_Program + "reports/sale2_80.rpt");
				}
				else
				{
					reportDocument7.Load(Module1.Path_Program + "/sale2_80.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale2_80.rpt"))
					{
						reportDocument7 = new sale2_80();
					}
				}
				reportDocument7.SetDataSource(Module1.localdata);
				reportDocument7.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print7.Text;
				reportDocument7.PrintToPrinter(Module1.copy_POS, collated: true, 0, 0);
				reportDocument7.Dispose();
				return;
			}
			ReportDocument reportDocument8 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/sale.rpt"))
			{
				reportDocument8.Load(Module1.Path_Program + "reports/sale.rpt");
			}
			else
			{
				reportDocument8.Load(Module1.Path_Program + "/sale.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale.rpt"))
				{
					reportDocument8 = new sale();
				}
			}
			reportDocument8.SetDataSource(Module1.localdata);
			reportDocument8.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print7.Text;
			reportDocument8.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument8.PrintToPrinter(Module1.copy_POS, collated: true, 0, 0);
			reportDocument8.Dispose();
		}
		else
		{
			smethod_1(id, preview);
		}
	}

	public static void Print_SaleVat(int id, bool preview)
	{
		preview = true;
		int num = 1;
		int num2 = 7;
		object obj = 0;
		bool flag = false;
		DataSet dataSet = Module1.connect("select * from HT_Receipt_H where id=" + Conversions.ToString(id));
		DataSet dataSet2 = Module1.connect("select * from HT_Receipt_Ds where s_sale_id=" + Conversions.ToString(id) + " order by id");
		DataSet dataSet3 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportBillCash.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet3.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		decimal num3 = default(decimal);
		num2 = Conversions.ToInteger(dataSet3.Tables[0].Rows[0]["Vat_Rows"]);
		if (Operators.CompareString(dataSet3.Tables[0].Rows[0]["Vat_Head2"].ToString(), "", TextCompare: false) != 0)
		{
			flag = true;
		}
		if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") != 0)
		{
		}
		if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") != 0)
		{
		}
		ReportDocument reportDocument;
		string text;
		int nCopies;
		checked
		{
			if (num2 != 0)
			{
				int num4 = dataSet2.Tables[0].Rows.Count - 1;
				int num5 = 0;
				while (true)
				{
					int num6 = num5;
					int num7 = num4;
					if (num6 > num7)
					{
						break;
					}
					int num8 = Module1.smethod_2(Conversions.ToString(dataSet2.Tables[0].Rows[num5]["s_product_name"]));
					if (num8 > 120)
					{
						num2 -= 3;
					}
					else if (num8 > 80)
					{
						num2 -= 2;
					}
					else if (num8 > 40)
					{
						num2--;
					}
					if (num2 <= 0)
					{
						num2 = 0;
					}
					num5++;
				}
			}
			int num9 = dataSet2.Tables[0].Rows.Count - 1;
			int num10 = 0;
			while (true)
			{
				int num11 = num10;
				int num7 = num9;
				if (num11 > num7)
				{
					break;
				}
				Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]), "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]), Conversions.ToString(num10 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["s_product_name"]), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["s_unit"]), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["s_unitname"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["s_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["s_total"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_BeforeVat"])), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Vat"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", dataSet.Tables[0].Rows[0]["Receipt_Tax"].ToString(), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "0.00"))), null, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["S_PriceDiscount"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Discount"]), "#,##0.00"), dataSet3.Tables[0].Rows[0]["Vat_Head"].ToString(), dataSet.Tables[0].Rows[0]["Receipt_note"].ToString());
				if (flag)
				{
					Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]), "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]), Conversions.ToString(num10 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["s_product_name"]), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["s_unit"]), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["s_unitname"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["s_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["s_total"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_BeforeVat"])), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Vat"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", dataSet.Tables[0].Rows[0]["Receipt_Tax"].ToString(), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "0.00"))), null, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["S_PriceDiscount"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Discount"]), "#,##0.00"), dataSet3.Tables[0].Rows[0]["Vat_Head2"].ToString(), dataSet.Tables[0].Rows[0]["Receipt_note"].ToString());
				}
				if (num10 + 1 == num2 * num)
				{
					num++;
				}
				obj = num10 + 1;
				num10++;
			}
			int num12 = Conversions.ToInteger(obj);
			int num13 = num2 * num - 1;
			int num14 = num12;
			while (true)
			{
				int num15 = num14;
				int num7 = num13;
				if (num15 > num7)
				{
					break;
				}
				string string_ = "";
				if (Operators.ConditionalCompareObjectEqual(obj, num2 * num - 1, TextCompare: false) && Operators.CompareString(dataSet.Tables[0].Rows[0]["Receipt_noteUP"].ToString(), "", TextCompare: false) != 0)
				{
					string_ = dataSet.Tables[0].Rows[0]["Receipt_noteUP"].ToString();
				}
				Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]), "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]), "", string_, "", "", "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_BeforeVat"]), "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Vat"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", dataSet.Tables[0].Rows[0]["Receipt_Tax"].ToString(), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "0.00"))), null, "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Discount"]), "#,##0.00"), dataSet3.Tables[0].Rows[0]["Vat_Head"].ToString(), dataSet.Tables[0].Rows[0]["Receipt_note"].ToString());
				if (flag)
				{
					Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]), "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]), "", string_, "", "", "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_BeforeVat"]), "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Vat"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", dataSet.Tables[0].Rows[0]["Receipt_Tax"].ToString(), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "0.00"))), null, "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Discount"]), "#,##0.00"), dataSet3.Tables[0].Rows[0]["Vat_Head2"].ToString(), dataSet.Tables[0].Rows[0]["Receipt_note"].ToString());
				}
				obj = num14 + 1;
				num14++;
			}
			reportDocument = new ReportDocument();
			if (Operators.CompareString(Module1.Tax_preview, "เป\u0e34ด", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmPrint.Close();
				MyProject.Forms.FrmPrint.Show();
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["Receipt_VatPer"], 0, TextCompare: false))
				{
					if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
					{
						if (!flag)
						{
							if (File.Exists(Module1.Path_Program + "reports/sale_vat0.rpt"))
							{
								reportDocument.Load(Module1.Path_Program + "reports/sale_vat0.rpt");
							}
							else
							{
								reportDocument.Load(Module1.Path_Program + "/sale_vat0.rpt");
							}
						}
						else if (File.Exists(Module1.Path_Program + "reports/sale_vat0_copy.rpt"))
						{
							reportDocument.Load(Module1.Path_Program + "reports/sale_vat0_copy.rpt");
						}
						else
						{
							reportDocument.Load(Module1.Path_Program + "/sale_vat0_copy.rpt");
						}
					}
					else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
					{
						if (File.Exists(Module1.Path_Program + "reports/sale_vat0SB.rpt"))
						{
							reportDocument.Load(Module1.Path_Program + "reports/sale_vat0SB.rpt");
						}
						else
						{
							reportDocument.Load(Module1.Path_Program + "/sale_vat0SB.rpt");
						}
					}
					else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") == 0)
					{
						if (File.Exists(Module1.Path_Program + "reports/sale_vat0CB.rpt"))
						{
							reportDocument.Load(Module1.Path_Program + "reports/sale_vat0CB.rpt");
						}
						else
						{
							reportDocument.Load(Module1.Path_Program + "/sale_vat0CB.rpt");
						}
					}
				}
				else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
				{
					if (!flag)
					{
						if (File.Exists(Module1.Path_Program + "reports/sale_vat.rpt"))
						{
							reportDocument.Load(Module1.Path_Program + "reports/sale_vat.rpt");
						}
						else
						{
							reportDocument.Load(Module1.Path_Program + "/sale_vat.rpt");
						}
					}
					else if (File.Exists(Module1.Path_Program + "reports/sale_vat_copy.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat_copy.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat_copy.rpt");
					}
				}
				else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
				{
					if (File.Exists(Module1.Path_Program + "reports/sale_vat0SB.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat0SB.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat0SB.rpt");
					}
				}
				else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") == 0)
				{
					if (File.Exists(Module1.Path_Program + "reports/sale_vat0CB.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat0CB.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat0CB.rpt");
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				return;
			}
			text = "";
			nCopies = 1;
			if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
			{
				text = MyProject.Forms.FrmSettings.Print3.Text;
				nCopies = Module1.int_4;
			}
			else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
			{
				text = MyProject.Forms.FrmSettings.print8.Text;
				nCopies = Module1.int_5;
			}
			else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") == 0)
			{
				text = MyProject.Forms.FrmSettings.print9.Text;
				nCopies = Module1.int_6;
			}
		}
		if (Operators.CompareString(text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (!ShowPrinter())
			{
				return;
			}
			if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["Receipt_VatPer"], 0, TextCompare: false))
			{
				if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
				{
					if (!flag)
					{
						if (File.Exists(Module1.Path_Program + "reports/sale_vat0.rpt"))
						{
							reportDocument.Load(Module1.Path_Program + "reports/sale_vat0.rpt");
						}
						else
						{
							reportDocument.Load(Module1.Path_Program + "/sale_vat0.rpt");
							if (!File.Exists(Module1.Path_Program + "/sale_vat0.rpt"))
							{
								reportDocument = new sale_vat0();
							}
						}
					}
					else if (File.Exists(Module1.Path_Program + "reports/sale_vat0_copy.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat0_copy.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat0_copy.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale_vat0_copy.rpt"))
						{
							reportDocument = new sale_vat0_copy();
						}
					}
				}
				else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
				{
					if (File.Exists(Module1.Path_Program + "reports/sale_vat0SB.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat0SB.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat0SB.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale_vat0SB.rpt"))
						{
							reportDocument = new sale_vat0SB();
						}
					}
				}
				else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") == 0)
				{
					if (File.Exists(Module1.Path_Program + "reports/sale_vat0CB.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat0CB.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat0CB.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale_vat0CB.rpt"))
						{
							reportDocument = new sale_vat0CB();
						}
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				reportDocument.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				if (num2 <= 7)
				{
					reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				}
				reportDocument.PrintToPrinter(PrintSet.PrinterSettings.Copies, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument.Dispose();
				return;
			}
			if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
			{
				if (!flag)
				{
					if (File.Exists(Module1.Path_Program + "reports/sale_vat.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale_vat.rpt"))
						{
							reportDocument = new sale_vat();
						}
					}
				}
				else if (File.Exists(Module1.Path_Program + "reports/sale_vat_copy.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat_copy.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat_copy.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat_copy.rpt"))
					{
						reportDocument = new sale_vat_copy();
					}
				}
			}
			else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
			{
				if (File.Exists(Module1.Path_Program + "reports/sale_vat0SB.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat0SB.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat0SB.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat0SB.rpt"))
					{
						reportDocument = new sale_vat0SB();
					}
				}
			}
			else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") == 0)
			{
				if (File.Exists(Module1.Path_Program + "reports/sale_vat0CB.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat0CB.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat0CB.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat0CB.rpt"))
					{
						reportDocument = new sale_vat0CB();
					}
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			reportDocument.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
			if (num2 <= 7)
			{
				reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			}
			reportDocument.PrintToPrinter(PrintSet.PrinterSettings.Copies, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			reportDocument.Dispose();
		}
		else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["Receipt_VatPer"], 0, TextCompare: false))
		{
			if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
			{
				if (!flag)
				{
					if (File.Exists(Module1.Path_Program + "reports/sale_vat0.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/sale_vat0.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/sale_vat0.rpt");
						if (!File.Exists(Module1.Path_Program + "/sale_vat0.rpt"))
						{
							reportDocument = new sale_vat0();
						}
					}
					reportDocument.SetDataSource(Module1.localdata);
					reportDocument.PrintOptions.PrinterName = text;
					reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
					reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
					reportDocument.Dispose();
					return;
				}
				if (File.Exists(Module1.Path_Program + "reports/sale_vat0_copy.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat0_copy.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat0_copy.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat0_copy.rpt"))
					{
						reportDocument = new sale_vat0_copy();
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				reportDocument.PrintOptions.PrinterName = text;
				reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
				reportDocument.Dispose();
			}
			else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
			{
				if (File.Exists(Module1.Path_Program + "reports/sale_vat0SB.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat0SB.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat0SB.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat0SB.rpt"))
					{
						reportDocument = new sale_vat0SB();
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				reportDocument.PrintOptions.PrinterName = text;
				if (num2 <= 7)
				{
					reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				}
				reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
				reportDocument.Dispose();
			}
			else
			{
				if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") != 0)
				{
					return;
				}
				if (File.Exists(Module1.Path_Program + "reports/sale_vat0CB.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat0CB.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat0CB.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat0CB.rpt"))
					{
						reportDocument = new sale_vat0CB();
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				reportDocument.PrintOptions.PrinterName = text;
				if (num2 <= 7)
				{
					reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				}
				reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
				reportDocument.Dispose();
			}
		}
		else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("B") == 0)
		{
			if (!flag)
			{
				if (File.Exists(Module1.Path_Program + "reports/sale_vat.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale_vat.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale_vat.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale_vat.rpt"))
					{
						reportDocument = new sale_vat();
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				reportDocument.PrintOptions.PrinterName = text;
				if (num2 <= 7)
				{
					reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				}
				reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
				reportDocument.Dispose();
				return;
			}
			if (File.Exists(Module1.Path_Program + "reports/sale_vat_copy.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/sale_vat_copy.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/sale_vat_copy.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale_vat_copy.rpt"))
				{
					reportDocument = new sale_vat_copy();
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			reportDocument.PrintOptions.PrinterName = text;
			if (num2 <= 7)
			{
				reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			}
			reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
			reportDocument.Dispose();
		}
		else if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("SB") == 0)
		{
			if (File.Exists(Module1.Path_Program + "reports/sale_vat0SB.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/sale_vat0SB.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/sale_vat0SB.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale_vat0SB.rpt"))
				{
					reportDocument = new sale_vat0SB();
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			reportDocument.PrintOptions.PrinterName = text;
			if (num2 <= 7)
			{
				reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			}
			reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
			reportDocument.Dispose();
		}
		else
		{
			if (dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().IndexOf("CB") != 0)
			{
				return;
			}
			if (File.Exists(Module1.Path_Program + "reports/sale_vat0CB.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/sale_vat0CB.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/sale_vat0CB.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale_vat0CB.rpt"))
				{
					reportDocument = new sale_vat0CB();
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			reportDocument.PrintOptions.PrinterName = text;
			if (num2 <= 7)
			{
				reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			}
			reportDocument.PrintToPrinter(nCopies, collated: true, 0, 0);
			reportDocument.Dispose();
		}
	}

	public static void Print_INVVat(int id, bool preview)
	{
		preview = true;
		int num = 1;
		int num2 = 7;
		object obj = 0;
		DataSet dataSet = Module1.connect("select * from HT_invoice_H where id=" + Conversions.ToString(id));
		DataSet dataSet2 = Module1.connect("select * from HT_invoice_Ds where s_sale_id=" + Conversions.ToString(id) + " order by id");
		DataSet dataSet3 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportBillCash.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet3.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		decimal num3 = default(decimal);
		ReportDocument reportDocument;
		checked
		{
			int num4 = dataSet2.Tables[0].Rows.Count - 1;
			int num5 = 0;
			while (true)
			{
				int num6 = num5;
				int num7 = num4;
				if (num6 > num7)
				{
					break;
				}
				Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]), "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]), Conversions.ToString(num5 + 1), Conversions.ToString(dataSet2.Tables[0].Rows[num5]["s_product_name"]), Conversions.ToString(dataSet2.Tables[0].Rows[num5]["s_unit"]), Conversions.ToString(dataSet2.Tables[0].Rows[num5]["s_unitname"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num5]["s_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num5]["s_total"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_BeforeVat"])), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Vat"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", dataSet.Tables[0].Rows[0]["Receipt_Tax"].ToString(), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "0.00"))), null, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num5]["S_PriceDiscount"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Discount"]), "#,##0.00"), dataSet3.Tables[0].Rows[0]["Vat_Head"].ToString(), dataSet.Tables[0].Rows[0]["Receipt_note"].ToString());
				if (num5 + 1 == num2 * num)
				{
					num++;
				}
				obj = num5 + 1;
				num5++;
			}
			int num8 = Conversions.ToInteger(obj);
			int num9 = num2 * num - 1;
			int num10 = num8;
			while (true)
			{
				int num11 = num10;
				int num7 = num9;
				if (num11 > num7)
				{
					break;
				}
				string string_ = "";
				if (Operators.ConditionalCompareObjectEqual(obj, num2 * num - 1, TextCompare: false) && Operators.CompareString(dataSet.Tables[0].Rows[0]["Receipt_noteUP"].ToString(), "", TextCompare: false) != 0)
				{
					string_ = dataSet.Tables[0].Rows[0]["Receipt_noteUP"].ToString();
				}
				Module1.localdata.ReportBillCash.AddReportBillCashRow(Conversions.ToString(num), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]), "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]), "", string_, "", "", "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_BeforeVat"]), "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Vat"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "#,##0.00"), "ผ\u0e39\u0e49ออกบ\u0e34ล", dataSet.Tables[0].Rows[0]["Receipt_Tax"].ToString(), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Total"]), "0.00"))), null, "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Receipt_Discount"]), "#,##0.00"), dataSet3.Tables[0].Rows[0]["Vat_Head"].ToString(), dataSet.Tables[0].Rows[0]["Receipt_note"].ToString());
				obj = num10 + 1;
				num10++;
			}
			reportDocument = new ReportDocument();
			if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["Receipt_VatPer"], 0, TextCompare: false))
			{
				if (File.Exists(Module1.Path_Program + "reports/inv_sale_other_novat.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/inv_sale_other_novat.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/inv_sale_other_vat.rpt");
				}
			}
			else if (File.Exists(Module1.Path_Program + "reports/inv_sale_other_vat.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/inv_sale_other_vat.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/inv_sale_other_vat.rpt");
			}
			reportDocument.SetDataSource(Module1.localdata);
		}
		if (Operators.CompareString(Module1.Tax_preview, "เป\u0e34ด", TextCompare: false) == 0)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
		else if (Operators.CompareString(MyProject.Forms.FrmSettings.Print3.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (ShowPrinter())
			{
				reportDocument.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument.PrintToPrinter(Module1.int_8, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument.Dispose();
			}
		}
		else
		{
			reportDocument.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print3.Text;
			reportDocument.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument.PrintToPrinter(Module1.int_8, collated: true, 0, 0);
			reportDocument.Dispose();
		}
	}

	public static void Print_Reg(string id, bool preview)
	{
		preview = true;
		DataSet dataSet = Module1.connect("select * from View_CheckIn_H where Cin_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		string string_ = "";
		string string_2 = "";
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 * from HT_CheckIn_Ds where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' order by id")));
		if (dataSet2.Tables[0].Rows.Count != 0)
		{
			string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yyyy HH:mm");
			string_2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_Price"]), "#,##0.00");
		}
		DataSet dataSet3 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportReg.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet3.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 * from Tb_Save_Image where cust_no='", dataSet.Tables[0].Rows[0]["Cin_cust_no"]), "' order by id desc")));
		string string_3 = "";
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Cin_book_no"].ToString(), "", TextCompare: false) != 0)
		{
			DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Book_H where book_id='", dataSet.Tables[0].Rows[0]["Cin_book_no"]), "' ")));
			if (dataSet5.Tables[0].Rows.Count != 0)
			{
				string_3 = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("จ\u0e48ายล\u0e48วงหน\u0e49า จากการจองเลขท\u0e35\u0e48 ", dataSet5.Tables[0].Rows[0]["book_id"]), " จำนวนเง\u0e34น "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[0]["book_price_pay"]), "#,##0.00")), " บาท"));
			}
		}
		string string_4 = "";
		string string_5 = "ไทย";
		DataSet dataSet6 = Module1.connect("select cust_idcard,cust_contry from HT_customers where Cust_no='" + dataSet.Tables[0].Rows[0]["Cin_cust_no"].ToString() + "'");
		if (dataSet6.Tables[0].Rows.Count != 0)
		{
			string_4 = dataSet6.Tables[0].Rows[0]["cust_idcard"].ToString();
			if (Operators.CompareString(dataSet6.Tables[0].Rows[0]["cust_contry"].ToString(), "", TextCompare: false) != 0)
			{
				string_5 = dataSet6.Tables[0].Rows[0]["cust_contry"].ToString();
			}
		}
		if ((Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 2.0) & (dataSet4.Tables[0].Rows.Count != 0))
		{
			byte[] buffer = (byte[])dataSet4.Tables[0].Rows[0]["pic"];
			Bitmap bmpimage = new Bitmap(new MemoryStream(buffer));
			bmpimage = Module1.RotateImg(bmpimage, Convert.ToSingle(270f));
			ImageConverter imageConverter = new ImageConverter();
			Module1.localdata.ReportReg.AddReportRegRow(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Room_ALL"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]), dataSet.Tables[0].Rows[0]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
				.Replace("ถนน ", "")
				.Replace("เขต/อำเภอ ", "")
				.Replace("แขวง/ตำบล ", "")
				.Replace("จ\u0e31งหว\u0e31ด ", ""), "", string_5, "", "", string_4, dataSet.Tables[0].Rows[0]["Cust_add_tel"].ToString(), string_, string_2, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_room"]), "#,##0.00"), dataSet.Tables[0].Rows[0]["cin_car_id"].ToString(), (byte[])imageConverter.ConvertTo(bmpimage, typeof(byte[])), dataSet.Tables[0].Rows[0]["Cin_cust_no"].ToString(), string_3, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_product"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_net"]), "#,##0.00"));
		}
		else
		{
			Module1.localdata.ReportReg.AddReportRegRow(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Room_ALL"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]), dataSet.Tables[0].Rows[0]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
				.Replace("ถนน ", "")
				.Replace("เขต/อำเภอ ", "")
				.Replace("แขวง/ตำบล ", "")
				.Replace("จ\u0e31งหว\u0e31ด ", ""), "", string_5, "", "", string_4, dataSet.Tables[0].Rows[0]["Cust_add_tel"].ToString(), string_, string_2, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_room"]), "#,##0.00"), dataSet.Tables[0].Rows[0]["cin_car_id"].ToString(), null, dataSet.Tables[0].Rows[0]["Cin_cust_no"].ToString(), string_3, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_product"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_net"]), "#,##0.00"));
		}
		if (Operators.CompareString(Module1.Cin_preview, "เป\u0e34ด", TextCompare: false) == 0)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			ReportDocument reportDocument = new ReportDocument();
			if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 0.0)
			{
				if (File.Exists(Module1.Path_Program + "reports/ReportReg_1.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/ReportReg_1.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/ReportReg_1.rpt");
				}
			}
			else if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 1.0)
			{
				if (File.Exists(Module1.Path_Program + "reports/ReportReg_2.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/ReportReg_2.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/ReportReg_2.rpt");
				}
			}
			else if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 2.0)
			{
				if (File.Exists(Module1.Path_Program + "reports/ReportReg_3.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/ReportReg_3.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/ReportReg_3.rpt");
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
		else if (Operators.CompareString(MyProject.Forms.FrmSettings.Print4.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (!ShowPrinter())
			{
				return;
			}
			if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 0.0)
			{
				ReportDocument reportDocument2 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportReg_1.rpt"))
				{
					reportDocument2.Load(Module1.Path_Program + "reports/ReportReg_1.rpt");
				}
				else
				{
					reportDocument2.Load(Module1.Path_Program + "/ReportReg_1.rpt");
					if (!File.Exists(Module1.Path_Program + "/ReportReg_1.rpt"))
					{
						reportDocument2 = new ReportReg_1();
					}
				}
				reportDocument2.SetDataSource(Module1.localdata);
				reportDocument2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument2.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument2.PrintToPrinter(Module1.int_7, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument2.Dispose();
			}
			else if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 1.0)
			{
				ReportDocument reportDocument3 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportReg_2.rpt"))
				{
					reportDocument3.Load(Module1.Path_Program + "reports/ReportReg_2.rpt");
				}
				else
				{
					reportDocument3.Load(Module1.Path_Program + "/ReportReg_2.rpt");
					if (!File.Exists(Module1.Path_Program + "/ReportReg_2.rpt"))
					{
						reportDocument3 = new ReportReg_2();
					}
				}
				reportDocument3.SetDataSource(Module1.localdata);
				reportDocument3.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument3.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument3.PrintToPrinter(Module1.int_7, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument3.Dispose();
			}
			else
			{
				if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) != 2.0)
				{
					return;
				}
				ReportDocument reportDocument4 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportReg_3.rpt"))
				{
					reportDocument4.Load(Module1.Path_Program + "reports/ReportReg_3.rpt");
				}
				else
				{
					reportDocument4.Load(Module1.Path_Program + "/ReportReg_3.rpt");
					if (!File.Exists(Module1.Path_Program + "/ReportReg_3.rpt"))
					{
						reportDocument4 = new ReportReg_3();
					}
				}
				reportDocument4.SetDataSource(Module1.localdata);
				reportDocument4.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument4.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("A4", PrintSet.PrinterSettings.PrinterName);
				reportDocument4.PrintToPrinter(Module1.int_7, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument4.Dispose();
			}
		}
		else if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 0.0)
		{
			ReportDocument reportDocument5 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/ReportReg_1.rpt"))
			{
				reportDocument5.Load(Module1.Path_Program + "reports/ReportReg_1.rpt");
			}
			else
			{
				reportDocument5.Load(Module1.Path_Program + "/ReportReg_1.rpt");
				if (!File.Exists(Module1.Path_Program + "/ReportReg_1.rpt"))
				{
					reportDocument5 = new ReportReg_1();
				}
			}
			reportDocument5.SetDataSource(Module1.localdata);
			reportDocument5.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print4.Text;
			reportDocument5.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument5.PrintToPrinter(Module1.int_7, collated: true, 0, 0);
			reportDocument5.Dispose();
		}
		else if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) == 1.0)
		{
			ReportDocument reportDocument6 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/ReportReg_2.rpt"))
			{
				reportDocument6.Load(Module1.Path_Program + "reports/ReportReg_2.rpt");
			}
			else
			{
				reportDocument6.Load(Module1.Path_Program + "/ReportReg_2.rpt");
				if (!File.Exists(Module1.Path_Program + "/ReportReg_2.rpt"))
				{
					reportDocument6 = new ReportReg_2();
				}
			}
			reportDocument6.SetDataSource(Module1.localdata);
			reportDocument6.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print4.Text;
			reportDocument6.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument6.PrintToPrinter(Module1.int_7, collated: true, 0, 0);
			reportDocument6.Dispose();
		}
		else
		{
			if (Conversions.ToDouble(dataSet3.Tables[0].Rows[0]["reg_type"].ToString()) != 2.0)
			{
				return;
			}
			ReportDocument reportDocument7 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/ReportReg_3.rpt"))
			{
				reportDocument7.Load(Module1.Path_Program + "reports/ReportReg_3.rpt");
			}
			else
			{
				reportDocument7.Load(Module1.Path_Program + "/ReportReg_3.rpt");
				if (!File.Exists(Module1.Path_Program + "/ReportReg_3.rpt"))
				{
					reportDocument7 = new ReportReg_3();
				}
			}
			reportDocument7.SetDataSource(Module1.localdata);
			reportDocument7.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print4.Text;
			reportDocument7.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("A4", PrintSet.PrinterSettings.PrinterName);
			reportDocument7.PrintToPrinter(Module1.int_7, collated: true, 0, 0);
			reportDocument7.Dispose();
		}
	}

	public static void Print_Reg2(string id, bool preview)
	{
		preview = true;
		DataSet dataSet = Module1.connect("select * from View_CheckIn_H where Cin_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		string string_ = "";
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 * from HT_CheckIn_Ds where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' order by id")));
		if (dataSet2.Tables[0].Rows.Count != 0)
		{
			string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yyyy HH:mm");
			Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_Price"]), "#,##0.00");
		}
		DataSet dataSet3 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportReg.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet3.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		string string_2 = "";
		string string_3 = "ไทย";
		DataSet dataSet4 = Module1.connect("select cust_idcard,cust_contry from HT_customers where Cust_no='" + dataSet.Tables[0].Rows[0]["Cin_cust_no"].ToString() + "'");
		if (dataSet4.Tables[0].Rows.Count != 0)
		{
			string_2 = dataSet4.Tables[0].Rows[0]["cust_idcard"].ToString();
			if (Operators.CompareString(dataSet4.Tables[0].Rows[0]["cust_contry"].ToString(), "", TextCompare: false) != 0)
			{
				string_3 = dataSet4.Tables[0].Rows[0]["cust_contry"].ToString();
			}
		}
		Module1.localdata.ReportReg.AddReportRegRow(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_Room_ALL"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["C_Address"]), "", string_3, "", "", string_2, dataSet.Tables[0].Rows[0]["Cust_add_tel"].ToString(), string_, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_balance"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["total_price_balance"]), "#,##0.00"), "", null, dataSet.Tables[0].Rows[0]["Cin_cust_no"].ToString(), "", "", "");
		if (Operators.CompareString(Module1.inv_preview, "เป\u0e34ด", TextCompare: false) == 0)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			ReportReg2 reportReg = new ReportReg2();
			reportReg.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportReg;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
		else if (Operators.CompareString(MyProject.Forms.FrmSettings.print6.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (ShowPrinter())
			{
				ReportReg2 reportReg2 = new ReportReg2();
				reportReg2.SetDataSource(Module1.localdata);
				reportReg2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportReg2.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportReg2.PrintToPrinter(Module1.int_8, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportReg2.Dispose();
			}
		}
		else
		{
			ReportReg2 reportReg3 = new ReportReg2();
			reportReg3.SetDataSource(Module1.localdata);
			reportReg3.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print6.Text;
			reportReg3.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportReg3.PrintToPrinter(Module1.int_8, collated: true, 0, 0);
			reportReg3.Dispose();
		}
	}

	public static void Print_Reg3(string id, bool preview)
	{
		preview = true;
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		DataSet dataSet = Module1.connect("select * from View_CheckIn_H where Cin_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		string right = "";
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select  * from HT_CheckIn_Ds where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' order by Cin_room_No")));
		DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select  * from HT_CheckIn_Product where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' order by Cin_pro_name")));
		string string_ = "";
		DataSet dataSet4 = Module1.connect("select * from HT_Invoice_Note where Cin_no='" + id + "'");
		if (dataSet4.Tables[0].Rows.Count != 0)
		{
			string_ = dataSet4.Tables[0].Rows[0]["note"].ToString();
		}
		DataSet dataSet5 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Report_Debt_INV.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet5.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		decimal num3 = Conversions.ToDecimal(dataSet5.Tables[0].Rows[0]["VAT_PER"]);
		int num4 = 0;
		string string_2 = "";
		string string_3 = "";
		object obj = "";
		object obj2 = "";
		object obj3 = "";
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
				string_3 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yyyy HH:mm");
				string_2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yyyy HH:mm");
				num4++;
				string string_4 = "ค\u0e37น";
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["cin_type"], 1, TextCompare: false))
				{
					string_4 = "ช\u0e31\u0e48วโมง";
				}
				else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["cin_type"], 2, TextCompare: false))
				{
					string_4 = "เด\u0e37อน";
					DateTime dateTime = Conversions.ToDate(dataSet2.Tables[0].Rows[0]["Cin_room_out"]);
					right = "\r\n ระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yyyy") + " ถ\u0e36ง " + Strings.Format(DateTime.DaysInMonth(dateTime.Year, dateTime.Month), "00") + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_out"]), "/MM/yyyy");
					string_3 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yyyy");
					string_2 = Strings.Format(DateTime.DaysInMonth(dateTime.Year, dateTime.Month), "00") + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_out"]), "/MM/yyyy");
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num6]["Cin_room_pricetotal"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num6]["Cin_room_pay_total"]));
				obj = dataSet.Tables[0].Rows[0]["Cust_name"].ToString();
				obj2 = Module1.Address_Rcplace(dataSet.Tables[0].Rows[0]["C_Address"].ToString());
				obj3 = dataSet.Tables[0].Rows[0]["Cust_add_tel"].ToString();
				if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"].ToString(), "", TextCompare: false) != 0)
				{
					obj = dataSet.Tables[0].Rows[0]["Cust_Work_Name"].ToString();
					obj2 = Module1.Address_Rcplace(dataSet.Tables[0].Rows[0]["W_Address"].ToString());
				}
				if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"].ToString(), "", TextCompare: false) != 0)
				{
					obj3 = dataSet.Tables[0].Rows[0]["Cust_Work_tel"].ToString();
				}
				Module1.localdata.Report_Debt_INV.AddReport_Debt_INVRow(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), string_3, dataSet.Tables[0].Rows[0]["Cin_Room_ALL"].ToString(), Conversions.ToString(obj), Conversions.ToString(obj2), Conversions.ToString(obj3), Conversions.ToString(num4), Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ค\u0e48าห\u0e49องพ\u0e31ก [", dataSet2.Tables[0].Rows[num6]["Cin_room_No"]), "]"), right)), Conversions.ToString(dataSet2.Tables[0].Rows[num6]["Cin_room_Night"]), string_4, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["Cin_room_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["Cin_room_pricetotal"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["Cin_room_pay_total"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet2.Tables[0].Rows[num6]["Cin_room_pricetotal"], dataSet2.Tables[0].Rows[num6]["Cin_room_pay_total"]), "#,##0.00"), Strings.Format(decimal.Subtract(num, num2), "#,##0.00"), Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), string_, DecimalToText_TH.ThaiBahtText(Convert.ToDouble(decimal.Subtract(num, num2))), string_2, Conversions.ToString(num3), Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3))), "0.00"), Strings.Format(decimal.Subtract(num, decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3)))), "#,##0.00"), "", "");
				num6++;
			}
			int num9 = dataSet3.Tables[0].Rows.Count - 1;
			int num10 = 0;
			while (true)
			{
				int num11 = num10;
				int num8 = num9;
				if (num11 > num8)
				{
					break;
				}
				num4++;
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet3.Tables[0].Rows[num10]["Cin_pro_pricetotal"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet3.Tables[0].Rows[num10]["Cin_pro_pay"]));
				Module1.localdata.Report_Debt_INV.AddReport_Debt_INVRow(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), string_3, dataSet.Tables[0].Rows[0]["Cin_Room_ALL"].ToString(), Conversions.ToString(obj), Conversions.ToString(obj2), Conversions.ToString(obj3), Conversions.ToString(num4), Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num10]["Cin_pro_name"], " ["), dataSet3.Tables[0].Rows[num10]["Cin_room_No"]), "]")), Conversions.ToString(dataSet3.Tables[0].Rows[num10]["Cin_pro_num"]), dataSet3.Tables[0].Rows[num10]["Cin_pro_unit"].ToString().Replace("0", "รายการ"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_pro_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_pro_pricetotal"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num10]["Cin_pro_pay"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet3.Tables[0].Rows[num10]["Cin_pro_pricetotal"], dataSet3.Tables[0].Rows[num10]["Cin_pro_pay"]), "#,##0.00"), Strings.Format(decimal.Subtract(num, num2), "#,##0.00"), Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), string_, DecimalToText_TH.ThaiBahtText(Convert.ToDouble(decimal.Subtract(num, num2))), string_2, Conversions.ToString(num3), Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3))), "0.00"), Strings.Format(decimal.Subtract(num, decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3)))), "#,##0.00"), "", "");
				num10++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			ReportDocument reportDocument = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/invoice_room.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/invoice_room.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/invoice_room.rpt");
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	public static void smethod_0(string id, bool preview)
	{
		preview = true;
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_Pay where pay_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_H where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
		if (dataSet2.Tables[0].Rows.Count == 0)
		{
			return;
		}
		string right = "";
		DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select  * from HT_CheckIn_Ds where Cin_no='", dataSet2.Tables[0].Rows[0]["Cin_no"]), "' order by Cin_room_No")));
		DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select  * from HT_CheckIn_Product where Cin_no='", dataSet2.Tables[0].Rows[0]["Cin_no"]), "' order by Cin_pro_name")));
		if ((dataSet3.Tables[0].Rows.Count == 0) & (dataSet4.Tables[0].Rows.Count == 0))
		{
			smethod_1(id, preview);
			return;
		}
		DataSet dataSet5 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Report_Debt_INV.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet5.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet5.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		decimal num3 = Conversions.ToDecimal(dataSet5.Tables[0].Rows[0]["VAT_PER"]);
		int num4 = 0;
		string string_ = "";
		string string_2 = "";
		decimal num5 = Conversions.ToDecimal(Operators.AddObject(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[0]["Cin_Pay_Tran"]), dataSet.Tables[0].Rows[0]["Cin_Pay_Free"]));
		string text = "";
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "เง\u0e34นสด";
		}
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "บ\u0e31ตรเครด\u0e34ต";
		}
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Tran"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "โอนเง\u0e34น";
		}
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Free"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "ฟร\u0e35/จองผ\u0e48านเน\u0e47ต/จ\u0e48ายล\u0e48วงหน\u0e49า";
		}
		checked
		{
			int num6 = dataSet3.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num9 = num6;
				if (num8 > num9)
				{
					break;
				}
				string_2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yyyy HH:mm");
				string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yyyy HH:mm");
				num4++;
				string string_3 = "ค\u0e37น";
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[0]["cin_type"], 1, TextCompare: false))
				{
					string_3 = "ช\u0e31\u0e48วโมง";
				}
				else if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[0]["cin_type"], 2, TextCompare: false))
				{
					string_3 = "เด\u0e37อน";
					DateTime dateTime = Conversions.ToDate(dataSet3.Tables[0].Rows[0]["Cin_room_out"]);
					right = "\r\n ระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yyyy") + " ถ\u0e36ง " + Strings.Format(DateTime.DaysInMonth(dateTime.Year, dateTime.Month), "00") + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["Cin_room_out"]), "/MM/yyyy");
					string_2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yyyy");
					string_ = Strings.Format(DateTime.DaysInMonth(dateTime.Year, dateTime.Month), "00") + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["Cin_room_out"]), "/MM/yyyy");
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet3.Tables[0].Rows[num7]["Cin_room_pricetotal"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet3.Tables[0].Rows[num7]["Cin_room_pay_total"]));
				Module1.localdata.Report_Debt_INV.AddReport_Debt_INVRow(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["Cin_no"], "/ "), id)), string_2, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Pay_date"]), "dd/MM/yyyy HH:mm"), dataSet2.Tables[0].Rows[0]["Cust_name"].ToString(), dataSet2.Tables[0].Rows[0]["C_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Cust_add_tel"].ToString(), Conversions.ToString(num4), Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ค\u0e48าห\u0e49องพ\u0e31ก [", dataSet3.Tables[0].Rows[num7]["Cin_room_No"]), "]"), right)), Conversions.ToString(dataSet3.Tables[0].Rows[num7]["Cin_room_Night"]), string_3, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num7]["Cin_room_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num7]["Cin_room_pricetotal"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num7]["Cin_room_pay_total"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet3.Tables[0].Rows[num7]["Cin_room_pricetotal"], dataSet3.Tables[0].Rows[num7]["Cin_room_pay_total"]), "#,##0.00"), Strings.Format(decimal.Subtract(num, num2), "#,##0.00"), Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), "", DecimalToText_TH.ThaiBahtText(Convert.ToDouble(decimal.Subtract(num, num2))), string_, Conversions.ToString(num3), Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3))), "0.00"), Strings.Format(decimal.Subtract(num, decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3)))), "#,##0.00"), text, Strings.Format(num5, "#,##0.00"));
				num7++;
			}
			int num10 = dataSet4.Tables[0].Rows.Count - 1;
			int num11 = 0;
			while (true)
			{
				int num12 = num11;
				int num9 = num10;
				if (num12 > num9)
				{
					break;
				}
				num4++;
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet4.Tables[0].Rows[num11]["Cin_pro_pricetotal"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet4.Tables[0].Rows[num11]["Cin_pro_pay"]));
				Module1.localdata.Report_Debt_INV.AddReport_Debt_INVRow(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["Cin_no"], "/ "), id)), string_2, Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_Pay_date"]), "dd/MM/yyyy HH:mm"), dataSet2.Tables[0].Rows[0]["Cust_name"].ToString(), dataSet2.Tables[0].Rows[0]["C_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Cust_add_tel"].ToString(), Conversions.ToString(num4), Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet4.Tables[0].Rows[num11]["Cin_pro_name"], " ["), dataSet4.Tables[0].Rows[num11]["Cin_room_No"]), "]")), Conversions.ToString(dataSet4.Tables[0].Rows[num11]["Cin_pro_num"]), dataSet4.Tables[0].Rows[num11]["Cin_pro_unit"].ToString().Replace("0", "รายการ"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num11]["Cin_pro_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num11]["Cin_pro_pricetotal"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num11]["Cin_pro_pay"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(dataSet4.Tables[0].Rows[num11]["Cin_pro_pricetotal"], dataSet4.Tables[0].Rows[num11]["Cin_pro_pay"]), "#,##0.00"), Strings.Format(decimal.Subtract(num, num2), "#,##0.00"), Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), "", DecimalToText_TH.ThaiBahtText(Convert.ToDouble(decimal.Subtract(num, num2))), string_, Conversions.ToString(num3), Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3))), "0.00"), Strings.Format(decimal.Subtract(num, decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3)))), "#,##0.00"), text, Strings.Format(num5, "#,##0.00"));
				num11++;
			}
			if (Operators.CompareString(Module1.Receipt_preview, "เป\u0e34ด", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmPrint.Close();
				MyProject.Forms.FrmPrint.Show();
				ReportDocument reportDocument = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale3_folio.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale3_folio.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale3_folio.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale3_folio.rpt"))
					{
						reportDocument = new sale3_folio();
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				return;
			}
			ReportDocument reportDocument2 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/sale3_folio.rpt"))
			{
				reportDocument2.Load(Module1.Path_Program + "reports/sale3_folio.rpt");
			}
			else
			{
				reportDocument2.Load(Module1.Path_Program + "/sale3_folio.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale3_folio.rpt"))
				{
					reportDocument2 = new sale3_folio();
				}
			}
			reportDocument2.SetDataSource(Module1.localdata);
			if (Operators.CompareString(MyProject.Forms.FrmSettings.Print1.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
			{
				if (!ShowPrinter())
				{
					return;
				}
				reportDocument2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument2.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			}
			else
			{
				reportDocument2.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print1.Text;
				reportDocument2.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			}
			reportDocument2.Dispose();
		}
	}

	public static void smethod_1(string id, bool preview)
	{
		preview = true;
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_Pay where pay_no='" + id + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
		DataSet dataSet2 = Module1.connect("select * from view_pay_ds where pay_no='" + id + "'");
		if (dataSet2.Tables[0].Rows.Count == 0)
		{
			return;
		}
		DataSet dataSet3 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Report_Debt_INV.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet3.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet3.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		decimal num3 = Conversions.ToDecimal(dataSet3.Tables[0].Rows[0]["VAT_PER"]);
		int num4 = 0;
		decimal num5 = Conversions.ToDecimal(Operators.AddObject(Operators.AddObject(Operators.AddObject(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"]), dataSet.Tables[0].Rows[0]["Cin_Pay_Tran"]), dataSet.Tables[0].Rows[0]["Cin_Pay_Free"]));
		string text = "";
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Cash"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "เง\u0e34นสด";
		}
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Credit"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "บ\u0e31ตรเครด\u0e34ต";
		}
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Tran"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "โอนเง\u0e34น";
		}
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["Cin_Pay_Free"], 0, TextCompare: false))
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text += ", ";
			}
			text += "ฟร\u0e35/จองผ\u0e48านเน\u0e47ต/จ\u0e48ายล\u0e48วงหน\u0e49า";
		}
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
				num4++;
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_priceTotal"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, Operators.AddObject(Operators.AddObject(Operators.AddObject(dataSet2.Tables[0].Rows[num7]["Cin_pay_cash"], dataSet2.Tables[0].Rows[num7]["Cin_pay_credit"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_Free"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_tran"])));
				if (dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_name"].ToString().IndexOf("ยกเล\u0e34กห\u0e49อง") != -1)
				{
					Module1.localdata.Report_Debt_INV.AddReport_Debt_INVRow(id, "-", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_Pay_date"]), "dd/MM/yyyy HH:mm"), dataSet2.Tables[0].Rows[0]["Cust_name"].ToString(), dataSet2.Tables[0].Rows[0]["C_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Cust_add_tel"].ToString(), Conversions.ToString(num4), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_name"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_num"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_unit"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_priceOne"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_priceTotal"]), "#,##0.00"), "", "", "", Strings.Format(num, "#,##0.00"), "", "", "", "-", Conversions.ToString(num3), Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3))), "0.00"), Strings.Format(decimal.Subtract(num, decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3)))), "#,##0.00"), text, Strings.Format(num5, "#,##0.00"));
				}
				else
				{
					Module1.localdata.Report_Debt_INV.AddReport_Debt_INVRow(id, "-", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_Pay_date"]), "dd/MM/yyyy HH:mm"), dataSet2.Tables[0].Rows[0]["Cust_name"].ToString(), dataSet2.Tables[0].Rows[0]["C_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Cust_add_tel"].ToString(), Conversions.ToString(num4), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_name"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_num"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_unit"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_priceOne"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_priceTotal"]), "#,##0.00"), Strings.Format(Operators.AddObject(Operators.AddObject(Operators.AddObject(dataSet2.Tables[0].Rows[num7]["Cin_pay_cash"], dataSet2.Tables[0].Rows[num7]["Cin_pay_credit"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_Free"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_tran"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(Operators.SubtractObject(Operators.SubtractObject(Operators.SubtractObject(dataSet2.Tables[0].Rows[num7]["Cin_pay_ds_priceTotal"], dataSet2.Tables[0].Rows[num7]["Cin_pay_cash"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_credit"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_Free"]), dataSet2.Tables[0].Rows[num7]["Cin_pay_tran"]), "#,##0.00"), Strings.Format(decimal.Subtract(num, num2), "#,##0.00"), Strings.Format(num, "#,##0.00"), Strings.Format(num2, "#,##0.00"), "", DecimalToText_TH.ThaiBahtText(Convert.ToDouble(decimal.Subtract(num, num2))), "-", Conversions.ToString(num3), Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3))), "0.00"), Strings.Format(decimal.Subtract(num, decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num3)))), "#,##0.00"), text, Strings.Format(num5, "#,##0.00"));
				}
				num7++;
			}
			if (Operators.CompareString(Module1.Receipt_preview, "เป\u0e34ด", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmPrint.Close();
				MyProject.Forms.FrmPrint.Show();
				ReportDocument reportDocument = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/sale3_folio.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/sale3_folio.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/sale3_folio.rpt");
					if (!File.Exists(Module1.Path_Program + "/sale3_folio.rpt"))
					{
						reportDocument = new sale3_folio();
					}
				}
				reportDocument.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				return;
			}
			ReportDocument reportDocument2 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/sale3_folio.rpt"))
			{
				reportDocument2.Load(Module1.Path_Program + "reports/sale3_folio.rpt");
			}
			else
			{
				reportDocument2.Load(Module1.Path_Program + "/sale3_folio.rpt");
				if (!File.Exists(Module1.Path_Program + "/sale3_folio.rpt"))
				{
					reportDocument2 = new sale3_folio();
				}
			}
			reportDocument2.SetDataSource(Module1.localdata);
			if (Operators.CompareString(MyProject.Forms.FrmSettings.Print1.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
			{
				if (!ShowPrinter())
				{
					return;
				}
				reportDocument2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument2.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			}
			else
			{
				reportDocument2.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print1.Text;
				reportDocument2.PrintToPrinter(Module1.int_1, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			}
			reportDocument2.Dispose();
		}
	}

	public static void Print_SaleCredit(string id, bool preview)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		Module1.localdata.ReportBillCredit.Rows.Clear();
		DataSet dataSet2 = Module1.connect("select * from HT_Bill_Debt_H where bill_no='" + id + "'");
		DataSet dataSet3 = Module1.connect("select * from HT_Bill_Debt_Ds where bill_no='" + id + "' order by DS_ID");
		decimal num = default(decimal);
		int num2 = 0;
		checked
		{
			int num3 = dataSet3.Tables[0].Rows.Count - 1;
			int num4 = 0;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 > num6)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet3.Tables[0].Rows[num4]["DS_NUM"]));
				Module1.localdata.ReportBillCredit.AddReportBillCreditRow(Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_No"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Bill_date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_ID"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_NAME"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_address"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_tel"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_by"]), Conversions.ToString(num4 + 1), Conversions.ToString(dataSet3.Tables[0].Rows[num4]["DS_NO"]), Conversions.ToString(dataSet3.Tables[0].Rows[num4]["DS_NAME"]), Conversions.ToString(dataSet3.Tables[0].Rows[num4]["DS_UNIT"]), Conversions.ToString(dataSet3.Tables[0].Rows[num4]["DS_NUM"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num4]["DS_PRICE"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num4]["DS_PRICE_TOTAL"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Bill_Total"]), "#,##0.00"), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(dataSet2.Tables[0].Rows[0]["Bill_Total"])), Conversions.ToString(num));
				num2++;
				num4++;
			}
			int num7 = num2;
			while (true)
			{
				int num8 = num7;
				int num6 = 5;
				if (num8 > 5)
				{
					break;
				}
				Module1.localdata.ReportBillCredit.AddReportBillCreditRow(Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_No"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Bill_date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_ID"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_NAME"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_address"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_Cust_tel"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Bill_by"]), "", "", "", "", "", "", "", Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Bill_Total"]), "#,##0.00"), DecimalToText_TH.ThaiBahtText(Conversions.ToDouble(dataSet2.Tables[0].Rows[0]["Bill_Total"])), Conversions.ToString(num));
				num2++;
				num7++;
			}
		}
		if (Operators.CompareString(Module1.inv_preview, "เป\u0e34ด", TextCompare: false) == 0)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			sale_credit sale_credit2 = new sale_credit();
			sale_credit2.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = sale_credit2;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
		else if (Operators.CompareString(MyProject.Forms.FrmSettings.print6.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (ShowPrinter())
			{
				sale_credit sale_credit3 = new sale_credit();
				sale_credit3.SetDataSource(Module1.localdata);
				sale_credit3.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				sale_credit3.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				sale_credit3.PrintToPrinter(Module1.int_8, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				sale_credit3.Dispose();
			}
		}
		else
		{
			sale_credit sale_credit4 = new sale_credit();
			sale_credit4.SetDataSource(Module1.localdata);
			sale_credit4.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print6.Text;
			sale_credit4.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			sale_credit4.PrintToPrinter(Module1.int_8, collated: true, 0, 0);
			sale_credit4.Dispose();
		}
	}

	public static void PrintFolio1(string CIN_NO)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportFolio1.Rows.Clear();
		Module1.localdata.ReportFolio1_2.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		decimal num = default(decimal);
		DataSet dataSet2 = Module1.connect("select * from View_CheckIn_Ds where cin_no='" + CIN_NO + "'");
		DataSet dataSet3 = Module1.connect("select * from HT_CheckIn_Product where cin_no='" + CIN_NO + "'");
		checked
		{
			int num2 = dataSet3.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet3.Tables[0].Rows[num3]["cin_pro_priceTotal"]));
				Module1.localdata.ReportFolio1_2.AddReportFolio1_2Row(Conversions.ToString(dataSet3.Tables[0].Rows[num3]["cin_pro_name"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["cin_pro_num"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["cin_pro_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["cin_pro_priceTotal"]), "#,##0.00"));
				num3++;
			}
			if (Module1.localdata.ReportFolio1_2.Rows.Count == 0)
			{
				Module1.localdata.ReportFolio1_2.AddReportFolio1_2Row("อ\u0e37\u0e48นๆ", "", "", "");
			}
			decimal num6 = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["vat_per"]);
			decimal num7 = default(decimal);
			decimal num8 = default(decimal);
			int num9 = dataSet2.Tables[0].Rows.Count - 1;
			int num10 = 0;
			while (true)
			{
				int num11 = num10;
				int num5 = num9;
				if (num11 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num10]["cin_room_priceTotal"]));
				num7 = Conversions.ToDecimal(Strings.Format(decimal.Subtract(num, decimal.Divide(decimal.Multiply(num, 100m), decimal.Add(100m, num6))), "0.00"));
				num8 = decimal.Subtract(num, num7);
				Module1.localdata.ReportFolio1.AddReportFolio1Row(Conversions.ToString(dataSet2.Tables[0].Rows[num10]["cin_cust_name"]), "", "", "", Conversions.ToString(dataSet2.Tables[0].Rows[num10]["cin_room_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["cin_room_in"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["cin_room_out"]), "dd/MM/yyyy"), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["cin_room_night"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["cin_room_price"]), "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["cin_room_priceTotal"]), "#,##0.00"), Strings.Format(num, "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[num10]["cin_no"]), Conversions.ToString(num6), Strings.Format(num7, "#,##0.00"), Strings.Format(num8, "#,##0.00"), DecimalToText_TH.ThaiBahtText(Convert.ToDouble(num)));
				num10++;
			}
			if (Module1.localdata.ReportFolio1.Rows.Count + Module1.localdata.ReportFolio1_2.Rows.Count <= 20)
			{
				int num12 = Module1.localdata.ReportFolio1.Rows.Count + Module1.localdata.ReportFolio1_2.Rows.Count;
				while (true)
				{
					int num13 = num12;
					int num5 = 19;
					if (num13 > 19)
					{
						break;
					}
					Module1.localdata.ReportFolio1.AddReportFolio1Row(Conversions.ToString(dataSet2.Tables[0].Rows[0]["cin_cust_name"]), "", "", "", "", "", "", "", "", "", Strings.Format(num, "#,##0.00"), "", Conversions.ToString(num6), Strings.Format(num7, "#,##0.00"), Strings.Format(num8, "#,##0.00"), DecimalToText_TH.ThaiBahtText(Convert.ToDouble(num)));
					num12++;
				}
			}
			ReportDocument reportDocument = new ReportDocument();
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (File.Exists(Module1.Path_Program + "reports/ReportFolio1.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/ReportFolio1.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/ReportFolio1.rpt");
				if (!File.Exists(Module1.Path_Program + "/ReportFolio1.rpt"))
				{
					reportDocument = new ReportFolio1();
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	public static void PrintFolio2(string CIN_NO)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		Module1.localdata.ReportFolio2.Rows.Clear();
		DataSet dataSet2 = Module1.connect("select * from TB_FOLIO where no='" + CIN_NO + "' order by id ");
		int num = 0;
		string text = "";
		string text2 = "";
		string text3 = "";
		string text4 = "";
		string text5 = "";
		string text6 = "";
		string text7 = "";
		decimal num2 = default(decimal);
		decimal num3 = Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["vat_per"]);
		decimal num4 = default(decimal);
		decimal num5 = default(decimal);
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
				if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num7]["F_IN"], "", TextCompare: false))
				{
					text2 = "";
					text = "";
					text3 = "";
					text4 = "";
					text5 = "";
					text6 = "";
					text7 = "";
					Module1.localdata.ReportFolio2.Rows[Module1.localdata.ReportFolio2.Rows.Count - 1][5] = Operators.ConcatenateObject(Operators.ConcatenateObject(Module1.localdata.ReportFolio2.Rows[Module1.localdata.ReportFolio2.Rows.Count - 1][5], "\r\n"), dataSet2.Tables[0].Rows[num7]["F_NAME"]);
				}
				else
				{
					num++;
					text2 = Conversions.ToString(num);
					text = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_ROOM"]);
					text3 = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_IN"]);
					text4 = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_OUT"]);
					text5 = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_NIGHT"]);
					text6 = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_PRICE"]);
					text7 = Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_PRICE_TOTAL"]);
					num2 = decimal.Add(num2, Conversions.ToDecimal(dataSet2.Tables[0].Rows[num7]["F_PRICE_TOTAL"]));
					num4 = Conversions.ToDecimal(Strings.Format(decimal.Subtract(num2, decimal.Divide(decimal.Multiply(num2, 100m), decimal.Add(100m, num3))), "0.00"));
					num5 = decimal.Subtract(num2, num4);
					Module1.localdata.ReportFolio2.AddReportFolio2Row(Conversions.ToString(dataSet2.Tables[0].Rows[num7]["CIN_NAME1"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["CIN_NAME2"]), Conversions.ToString(dataSet2.Tables[0].Rows[num7]["CIN_NAME3"]), text2, text, Conversions.ToString(dataSet2.Tables[0].Rows[num7]["F_NAME"]), text3, text4, text5, text6, text7, Strings.Format(num2, "#,##0.00"), CIN_NO, Conversions.ToString(num3), Strings.Format(num4, "#,##0.00"), Strings.Format(num5, "#,##0.00"), DecimalToText_TH.ThaiBahtText(Convert.ToDouble(num2)));
				}
				num7++;
			}
			ReportDocument reportDocument = new ReportDocument();
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (File.Exists(Module1.Path_Program + "reports/ReportFolio2.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/ReportFolio2.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/ReportFolio2.rpt");
				if (!File.Exists(Module1.Path_Program + "/ReportFolio2.rpt"))
				{
					reportDocument = new ReportFolio2();
				}
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	public static void print_booking(string booking_NO)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportBooking.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		DataSet dataSet2 = Module1.connect("select * from HT_Book_H where book_id='" + booking_NO + "'");
		DataSet dataSet3 = Module1.connect("SELECT     dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, SUM(dbo.HT_Book_Ds.Book_Room_Num) AS Book_Room_Num, dbo.HT_Book_Ds.Book_Room_Note, dbo.HT_Book_Ds.Book_status, dbo.HT_Rooms.Room_Type FROM         dbo.HT_Book_Ds INNER JOIN dbo.HT_Rooms ON dbo.HT_Book_Ds.Book_Room_Type = dbo.HT_Rooms.Room_no where book_no='" + booking_NO + "' GROUP BY dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, dbo.HT_Book_Ds.Book_Room_Num, dbo.HT_Book_Ds.Book_Room_Note, dbo.HT_Book_Ds.Book_status, dbo.HT_Rooms.Room_Type ");
		DataSet dataSet4 = Module1.connect("select B_NAME,sum(B_NUM) as B_NUM,B_PRICE from HT_Book_Pro where B_NO='" + booking_NO + "' group by B_NAME,B_PRICE");
		decimal num = default(decimal);
		checked
		{
			int num2 = dataSet3.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num3]["book_room_price"], dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet3.Tables[0].Rows[num3]["book_room_night"])));
				Module1.localdata.ReportBooking.AddReportBookingRow(booking_NO, Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["book_cust_name"], " "), dataSet2.Tables[0].Rows[0]["book_cust_name2"])), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["book_room_night"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["room_type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["book_room_price"]), "#,##0.00"), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Night"]), Strings.Format(Operators.MultiplyObject(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num3]["book_room_price"], dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet3.Tables[0].Rows[num3]["book_room_night"]), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_room_note"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_by"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_date"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"), Strings.Format(num, "#,##0.00"));
				num3++;
			}
			int num6 = dataSet4.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(dataSet4.Tables[0].Rows[num7]["B_NUM"], dataSet4.Tables[0].Rows[num7]["B_PRICE"])));
				Module1.localdata.ReportBooking.AddReportBookingRow(booking_NO, Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["book_cust_name"], " "), dataSet2.Tables[0].Rows[0]["book_cust_name2"])), "-", "-", "", Conversions.ToString(dataSet4.Tables[0].Rows[num7]["B_NAME"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"), Conversions.ToString(dataSet4.Tables[0].Rows[num7]["B_NUM"]), "", Strings.Format(Operators.MultiplyObject(dataSet4.Tables[0].Rows[num7]["B_NUM"], dataSet4.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_room_note"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_by"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_date"]), "dd/MM/yyyy"), "-", "-", Strings.Format(num, "#,##0.00"));
				num7++;
			}
			ReportDocument reportDocument = new ReportDocument();
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (File.Exists(Module1.Path_Program + "reports/ReportBooking.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/ReportBooking.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/ReportBooking.rpt");
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	public static void print_inv_booking(string booking_NO)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportBookingINV.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		DataSet dataSet2 = Module1.connect("select * from HT_Book_H where book_id='" + booking_NO + "'");
		DataSet dataSet3 = Module1.connect("SELECT     dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, SUM(dbo.HT_Book_Ds.Book_Room_Num) AS Book_Room_Num, dbo.HT_Book_Ds.Book_Room_Note, dbo.HT_Book_Ds.Book_status, dbo.HT_Rooms.Room_Type FROM         dbo.HT_Book_Ds INNER JOIN dbo.HT_Rooms ON dbo.HT_Book_Ds.Book_Room_Type = dbo.HT_Rooms.Room_no where book_no='" + booking_NO + "' GROUP BY dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, dbo.HT_Book_Ds.Book_Room_Num, dbo.HT_Book_Ds.Book_Room_Note, dbo.HT_Book_Ds.Book_status, dbo.HT_Rooms.Room_Type ");
		DataSet dataSet4 = Module1.connect("select B_NAME,sum(B_NUM) as B_NUM,B_PRICE from HT_Book_Pro where B_NO='" + booking_NO + "' group by B_NAME,B_PRICE");
		DataSet dataSet5 = Module1.connect("select * from HT_INVOICE where INV_booking_no='" + booking_NO + "'");
		decimal num = default(decimal);
		checked
		{
			int num2 = dataSet3.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num3]["book_room_price"], dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet3.Tables[0].Rows[num3]["book_room_night"])));
				Module1.localdata.ReportBookingINV.AddReportBookingINVRow(booking_NO, Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["book_cust_name"], " "), dataSet2.Tables[0].Rows[0]["book_cust_name2"])), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["book_room_night"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["room_type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["book_room_price"]), "#,##0.00"), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Night"]), Strings.Format(Operators.MultiplyObject(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num3]["book_room_price"], dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet3.Tables[0].Rows[num3]["book_room_night"]), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_room_note"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_by"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_date"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"), Strings.Format(num, "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[0]["INV_DATE"]), "dd/MM/yyyy"), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_BY"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_TITLE"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_NAME"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_COMPANY"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_ADDRESS"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_TEL"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_NIGHT"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_PAX"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_PAX_CHILD"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_PAYMENT"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[0]["INV_DUEDATE"]), "dd/MM/yyyy"), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_NOTE"]), Strings.Format(Conversions.ToInteger(dataSet5.Tables[0].Rows[0]["INV_NO"]), "0000"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_price_pay"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(num, dataSet2.Tables[0].Rows[0]["book_price_pay"]), "#,##0.00"), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_STAY"]));
				num3++;
			}
			int num6 = dataSet4.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(dataSet4.Tables[0].Rows[num7]["B_NUM"], dataSet4.Tables[0].Rows[num7]["B_PRICE"])));
				Module1.localdata.ReportBookingINV.AddReportBookingINVRow(booking_NO, Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["book_cust_name"], " "), dataSet2.Tables[0].Rows[0]["book_cust_name2"])), "-", "-", "", Conversions.ToString(dataSet4.Tables[0].Rows[num7]["B_NAME"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet4.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"), Conversions.ToString(dataSet4.Tables[0].Rows[num7]["B_NUM"]), "", Strings.Format(Operators.MultiplyObject(dataSet4.Tables[0].Rows[num7]["B_NUM"], dataSet4.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_room_note"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_by"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_date"]), "dd/MM/yyyy"), "-", "-", Strings.Format(num, "#,##0.00"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[0]["INV_DATE"]), "dd/MM/yyyy"), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_BY"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_TITLE"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_NAME"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_COMPANY"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_ADDRESS"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_TEL"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_NIGHT"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_PAX"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_PAX_CHILD"]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_PAYMENT"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet5.Tables[0].Rows[0]["INV_DUEDATE"]), "dd/MM/yyyy"), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_NOTE"]), Strings.Format(Conversions.ToInteger(dataSet5.Tables[0].Rows[0]["INV_NO"]), "0000"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_price_pay"]), "#,##0.00"), Strings.Format(Operators.SubtractObject(num, dataSet2.Tables[0].Rows[0]["book_price_pay"]), "#,##0.00"), Conversions.ToString(dataSet5.Tables[0].Rows[0]["INV_STAY"]));
				num7++;
			}
			ReportDocument reportDocument = new ReportDocument();
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (File.Exists(Module1.Path_Program + "reports/ReportBookingINV.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/ReportBookingINV.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/ReportBookingINV.rpt");
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	public static void print_booking_type(string booking_NO)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportBooking.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		DataSet dataSet2 = Module1.connect("select * from HT_Book_H where book_id='" + booking_NO + "'");
		DataSet dataSet3 = Module1.connect("SELECT     Book_No, Book_Room_Price, Book_Room_Start, Book_Room_End, Book_Room_Night, SUM(Book_Room_Num) AS Book_Room_Num, Book_Room_Note, Book_status, Book_Room_Type FROM dbo.HT_Book_Ds where book_no='" + booking_NO + "' GROUP BY Book_No, Book_Room_Start, Book_Room_End, Book_Room_Price, Book_Room_Night, Book_Room_Num, Book_Room_Note, Book_status, Book_Room_Type");
		decimal num = default(decimal);
		checked
		{
			int num2 = dataSet3.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num3]["book_room_price"], dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet3.Tables[0].Rows[num3]["book_room_night"])));
				Module1.localdata.ReportBooking.AddReportBookingRow(booking_NO, Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[0]["book_cust_name"], " "), dataSet2.Tables[0].Rows[0]["book_cust_name2"])), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["book_room_night"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Type"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["book_room_price"]), "#,##0.00"), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), Conversions.ToString(dataSet3.Tables[0].Rows[num3]["Book_Room_Night"]), Strings.Format(Operators.MultiplyObject(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num3]["book_room_price"], dataSet3.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet3.Tables[0].Rows[num3]["book_room_night"]), "#,##0.00"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_room_note"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_by"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_date"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"), Strings.Format(num, "#,##0.00"));
				num3++;
			}
			ReportDocument reportDocument = new ReportDocument();
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (File.Exists(Module1.Path_Program + "reports/ReportBooking.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/ReportBooking.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/ReportBooking.rpt");
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	public static void Print_coupon_from_no(string cin_no)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		Module1.localdata.ReportCupon.Rows.Clear();
		DataSet dataSet2 = Module1.connect("select * from View_cupon where cupon_cin_no='" + cin_no + "' and cupon_print=0");
		checked
		{
			int num = dataSet2.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				Module1.localdata.ReportCupon.AddReportCuponRow(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["cupon_no"]), "00000"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["cupon_no"]), "00000"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["cupon_gen_date"]), "dd/MM/yyyy HH:mm"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["cupon_date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet2.Tables[0].Rows[num2]["cupon_cin_room"]), Conversions.ToString(dataSet2.Tables[0].Rows[num2]["Cust_name"]));
				num2++;
			}
			Module1.connect("update HT_Cupon set cupon_print=1 where cupon_cin_no='" + cin_no + "'");
			if (dataSet2.Tables[0].Rows.Count == 0)
			{
				return;
			}
			if (Operators.CompareString(Module1.Cupon_preview, "เป\u0e34ด", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmPrint.Close();
				MyProject.Forms.FrmPrint.Show();
				if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
				{
					ReportCupon80 reportCupon = new ReportCupon80();
					reportCupon.SetDataSource(Module1.localdata);
					MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportCupon;
					MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				}
				else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
				{
					ReportCupon58 reportCupon2 = new ReportCupon58();
					reportCupon2.SetDataSource(Module1.localdata);
					MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportCupon2;
					MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				}
			}
			else if (Operators.CompareString(MyProject.Forms.FrmSettings.print5.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
			{
				if (!ShowPrinter())
				{
					return;
				}
				if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/ReportCupon80.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/ReportCupon80.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/ReportCupon80.rpt");
					}
					reportDocument.SetDataSource(Module1.localdata);
					reportDocument.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument.Dispose();
				}
				else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument2 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/ReportCupon58.rpt"))
					{
						reportDocument2.Load(Module1.Path_Program + "reports/ReportCupon58.rpt");
					}
					else
					{
						reportDocument2.Load(Module1.Path_Program + "/ReportCupon58.rpt");
					}
					reportDocument2.SetDataSource(Module1.localdata);
					reportDocument2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument2.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument2.Dispose();
				}
			}
			else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument3 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportCupon80.rpt"))
				{
					reportDocument3.Load(Module1.Path_Program + "reports/ReportCupon80.rpt");
				}
				else
				{
					reportDocument3.Load(Module1.Path_Program + "/ReportCupon80.rpt");
				}
				reportDocument3.SetDataSource(Module1.localdata);
				reportDocument3.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print5.Text;
				reportDocument3.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument3.Dispose();
			}
			else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument4 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportCupon58.rpt"))
				{
					reportDocument4.Load(Module1.Path_Program + "reports/ReportCupon58.rpt");
				}
				else
				{
					reportDocument4.Load(Module1.Path_Program + "/ReportCupon58.rpt");
				}
				reportDocument4.SetDataSource(Module1.localdata);
				reportDocument4.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print5.Text;
				reportDocument4.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument4.Dispose();
			}
		}
	}

	public static void Print_coupon(ArrayList id)
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		Module1.localdata.ReportCupon.Rows.Clear();
		string text = "";
		checked
		{
			int num = id.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_cupon where cupon_no='", id[num2]), "'")));
				Module1.localdata.ReportCupon.AddReportCuponRow(Strings.Format(Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["cupon_no"]), "00000"), Strings.Format(Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["cupon_no"]), "00000"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["cupon_gen_date"]), "dd/MM/yyyy HH:mm"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["cupon_date"]), "dd/MM/yyyy"), Conversions.ToString(dataSet2.Tables[0].Rows[0]["cupon_cin_room"]), Conversions.ToString(dataSet2.Tables[0].Rows[0]["Cust_name"]));
				text = ((num2 != 0) ? Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(",", dataSet2.Tables[0].Rows[0]["cupon_no"]))) : Conversions.ToString(dataSet2.Tables[0].Rows[0]["cupon_no"]));
				num2++;
			}
			Module1.connect("update HT_Cupon set cupon_print=1 where cupon_no in (" + text + ")");
			if (Operators.CompareString(Module1.Cupon_preview, "เป\u0e34ด", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmPrint.Close();
				MyProject.Forms.FrmPrint.Show();
				if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
				{
					ReportCupon80 reportCupon = new ReportCupon80();
					reportCupon.SetDataSource(Module1.localdata);
					MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportCupon;
					MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				}
				else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
				{
					ReportCupon58 reportCupon2 = new ReportCupon58();
					reportCupon2.SetDataSource(Module1.localdata);
					MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportCupon2;
					MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
				}
			}
			else if (Operators.CompareString(MyProject.Forms.FrmSettings.print5.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
			{
				if (!ShowPrinter())
				{
					return;
				}
				if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/ReportCupon80.rpt"))
					{
						reportDocument.Load(Module1.Path_Program + "reports/ReportCupon80.rpt");
					}
					else
					{
						reportDocument.Load(Module1.Path_Program + "/ReportCupon80.rpt");
					}
					reportDocument.SetDataSource(Module1.localdata);
					reportDocument.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument.Dispose();
				}
				else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
				{
					ReportDocument reportDocument2 = new ReportDocument();
					if (File.Exists(Module1.Path_Program + "reports/ReportCupon58.rpt"))
					{
						reportDocument2.Load(Module1.Path_Program + "reports/ReportCupon58.rpt");
					}
					else
					{
						reportDocument2.Load(Module1.Path_Program + "/ReportCupon58.rpt");
					}
					reportDocument2.SetDataSource(Module1.localdata);
					reportDocument2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
					reportDocument2.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
					reportDocument2.Dispose();
				}
			}
			else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument3 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportCupon80.rpt"))
				{
					reportDocument3.Load(Module1.Path_Program + "reports/ReportCupon80.rpt");
				}
				else
				{
					reportDocument3.Load(Module1.Path_Program + "/ReportCupon80.rpt");
				}
				reportDocument3.SetDataSource(Module1.localdata);
				reportDocument3.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print5.Text;
				reportDocument3.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument3.Dispose();
			}
			else if (Operators.CompareString(Module1.string_2, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument4 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportCupon58.rpt"))
				{
					reportDocument4.Load(Module1.Path_Program + "reports/ReportCupon58.rpt");
				}
				else
				{
					reportDocument4.Load(Module1.Path_Program + "/ReportCupon58.rpt");
				}
				reportDocument4.SetDataSource(Module1.localdata);
				reportDocument4.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.print5.Text;
				reportDocument4.PrintToPrinter(Module1.int_3, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument4.Dispose();
			}
		}
	}

	public static void Print_Dep(string id, bool preview, string NOTIN = "")
	{
		preview = true;
		if (Operators.CompareString(NOTIN, "", TextCompare: false) != 0)
		{
			NOTIN = " and id not in (" + NOTIN + ") ";
		}
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_No='" + id + "' and cin_room_dep<>0 " + NOTIN);
		DataSet dataSet2 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportDep.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet2.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
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
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num2]["Cin_Room_Dep"], 0, TextCompare: false))
				{
					Module1.localdata.ReportDep.AddReportDepRow(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["Cin_No"], "_"), dataSet.Tables[0].Rows[num2]["Cin_Room_No"])), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cin_Date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["Cin_No"], "_"), dataSet.Tables[0].Rows[num2]["Cin_Room_No"])), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Cin_Room_No"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Cin_cust_name"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Cin_Room_Dep"]), DecimalToText_TH.ThaiBahtText(Convert.ToDouble(Conversions.ToDecimal(dataSet.Tables[0].Rows[num2]["Cin_Room_Dep"]))));
				}
				num2++;
			}
		}
		if (Operators.CompareString(Module1.Deposit_preview, "เป\u0e34ด", TextCompare: false) == 0)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (Operators.CompareString(Module1.Deposit_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
			{
				ReportDocument reportDocument = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/ReportDep.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/ReportDep.rpt");
				}
				reportDocument.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument2 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_80.rpt"))
				{
					reportDocument2.Load(Module1.Path_Program + "reports/ReportDep2_80.rpt");
				}
				else
				{
					reportDocument2.Load(Module1.Path_Program + "/ReportDep2_80.rpt");
				}
				reportDocument2.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument2;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument3 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_58.rpt"))
				{
					reportDocument3.Load(Module1.Path_Program + "reports/ReportDep2_58.rpt");
				}
				else
				{
					reportDocument3.Load(Module1.Path_Program + "/ReportDep2_58.rpt");
				}
				reportDocument3.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument3;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
		}
		else if (Operators.CompareString(MyProject.Forms.FrmSettings.Print2.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (!ShowPrinter())
			{
				return;
			}
			if (Operators.CompareString(Module1.Deposit_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
			{
				ReportDocument reportDocument4 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep.rpt"))
				{
					reportDocument4.Load(Module1.Path_Program + "reports/ReportDep.rpt");
				}
				else
				{
					reportDocument4.Load(Module1.Path_Program + "/ReportDep.rpt");
				}
				reportDocument4.SetDataSource(Module1.localdata);
				reportDocument4.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument4.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument4.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument4.Dispose();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument5 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_58.rpt"))
				{
					reportDocument5.Load(Module1.Path_Program + "reports/ReportDep2_58.rpt");
				}
				else
				{
					reportDocument5.Load(Module1.Path_Program + "/ReportDep2_58.rpt");
				}
				reportDocument5.SetDataSource(Module1.localdata);
				reportDocument5.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument5.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument5.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument5.Dispose();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument6 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_80.rpt"))
				{
					reportDocument6.Load(Module1.Path_Program + "reports/ReportDep2_80.rpt");
				}
				else
				{
					reportDocument6.Load(Module1.Path_Program + "/ReportDep2_80.rpt");
				}
				reportDocument6.SetDataSource(Module1.localdata);
				reportDocument6.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument6.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument6.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument6.Dispose();
			}
		}
		else if (Operators.CompareString(Module1.Deposit_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
		{
			ReportDocument reportDocument7 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/ReportDep.rpt"))
			{
				reportDocument7.Load(Module1.Path_Program + "reports/ReportDep.rpt");
			}
			else
			{
				reportDocument7.Load(Module1.Path_Program + "/ReportDep.rpt");
			}
			reportDocument7.SetDataSource(Module1.localdata);
			reportDocument7.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print2.Text;
			reportDocument7.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument7.PrintToPrinter(Module1.int_2, collated: true, 0, 0);
			reportDocument7.Dispose();
		}
		else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
		{
			ReportDocument reportDocument8 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/ReportDep2_58.rpt"))
			{
				reportDocument8.Load(Module1.Path_Program + "reports/ReportDep2_58.rpt");
			}
			else
			{
				reportDocument8.Load(Module1.Path_Program + "/ReportDep2_58.rpt");
			}
			reportDocument8.SetDataSource(Module1.localdata);
			reportDocument8.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print2.Text;
			reportDocument8.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument8.PrintToPrinter(Module1.int_2, collated: true, 0, 0);
			reportDocument8.Dispose();
		}
		else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
		{
			ReportDocument reportDocument9 = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/ReportDep2_80.rpt"))
			{
				reportDocument9.Load(Module1.Path_Program + "reports/ReportDep2_80.rpt");
			}
			else
			{
				reportDocument9.Load(Module1.Path_Program + "/ReportDep2_80.rpt");
			}
			reportDocument9.SetDataSource(Module1.localdata);
			reportDocument9.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print2.Text;
			reportDocument9.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDocument9.PrintToPrinter(Module1.int_2, collated: true, 0, 0);
			reportDocument9.Dispose();
		}
	}

	public static void Print_Dep_per(string id, bool preview)
	{
		preview = true;
		DataSet dataSet = Module1.connect("select * from View_Deposit_H where dep_no='" + id + "'");
		DataSet dataSet2 = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.ReportDep.Rows.Clear();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet2.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet2.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
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
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num2]["Cin_Room_Dep"], 0, TextCompare: false))
				{
					Module1.localdata.ReportDep.AddReportDepRow(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["Cin_No"], "_"), dataSet.Tables[0].Rows[num2]["Cin_Room_No"])), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cin_Date"]), "dd/MM/yyyy HH:mm"), Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["Cin_No"], "_"), dataSet.Tables[0].Rows[num2]["Cin_Room_No"])), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Cin_Room_No"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["cust_name"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Cin_Room_Dep"]), DecimalToText_TH.ThaiBahtText(Convert.ToDouble(Conversions.ToDecimal(dataSet.Tables[0].Rows[num2]["Cin_Room_Dep"]))));
				}
				num2++;
			}
		}
		if (Operators.CompareString(Module1.Deposit_preview, "เป\u0e34ด", TextCompare: false) == 0)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (Operators.CompareString(Module1.Deposit_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
			{
				ReportDocument reportDocument = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep.rpt"))
				{
					reportDocument.Load(Module1.Path_Program + "reports/ReportDep.rpt");
				}
				else
				{
					reportDocument.Load(Module1.Path_Program + "/ReportDep.rpt");
				}
				reportDocument.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument2 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_58.rpt"))
				{
					reportDocument2.Load(Module1.Path_Program + "reports/ReportDep2_58.rpt");
				}
				else
				{
					reportDocument2.Load(Module1.Path_Program + "/ReportDep2_58.rpt");
				}
				reportDocument2.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument2;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument3 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_80.rpt"))
				{
					reportDocument3.Load(Module1.Path_Program + "reports/ReportDep2_80.rpt");
				}
				else
				{
					reportDocument3.Load(Module1.Path_Program + "/ReportDep2_80.rpt");
				}
				reportDocument3.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument3;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
		}
		else if (Operators.CompareString(MyProject.Forms.FrmSettings.Print2.Text, "เล\u0e37อกตอนพ\u0e34มพ\u0e4c", TextCompare: false) == 0)
		{
			if (!ShowPrinter())
			{
				return;
			}
			if (Operators.CompareString(Module1.Deposit_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
			{
				ReportDocument reportDocument4 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep.rpt"))
				{
					reportDocument4.Load(Module1.Path_Program + "reports/ReportDep.rpt");
				}
				else
				{
					reportDocument4.Load(Module1.Path_Program + "/ReportDep.rpt");
				}
				reportDocument4.SetDataSource(Module1.localdata);
				reportDocument4.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument4.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument4.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument4.Dispose();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument5 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_58.rpt"))
				{
					reportDocument5.Load(Module1.Path_Program + "reports/ReportDep2_58.rpt");
				}
				else
				{
					reportDocument5.Load(Module1.Path_Program + "/ReportDep2_58.rpt");
				}
				reportDocument5.SetDataSource(Module1.localdata);
				reportDocument5.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument5.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument5.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument5.Dispose();
			}
			else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
			{
				ReportDocument reportDocument6 = new ReportDocument();
				if (File.Exists(Module1.Path_Program + "reports/ReportDep2_80.rpt"))
				{
					reportDocument6.Load(Module1.Path_Program + "reports/ReportDep2_80.rpt");
				}
				else
				{
					reportDocument6.Load(Module1.Path_Program + "/ReportDep2_80.rpt");
				}
				reportDocument6.SetDataSource(Module1.localdata);
				reportDocument6.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
				reportDocument6.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
				reportDocument6.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
				reportDocument6.Dispose();
			}
		}
		else if (Operators.CompareString(Module1.Deposit_Report, "กระดาษต\u0e48อเน\u0e37\u0e48อง", TextCompare: false) == 0)
		{
			ReportDep reportDep = new ReportDep();
			reportDep.SetDataSource(Module1.localdata);
			reportDep.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print2.Text;
			reportDep.PrintOptions.PaperSize = (PaperSize)Module1.getPaperSize("sir_Form", PrintSet.PrinterSettings.PrinterName);
			reportDep.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			reportDep.Dispose();
		}
		else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", TextCompare: false) == 0)
		{
			ReportDep2_58 reportDep2_ = new ReportDep2_58();
			reportDep2_.SetDataSource(Module1.localdata);
			reportDep2_.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print2.Text;
			reportDep2_.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			reportDep2_.Dispose();
		}
		else if (Operators.CompareString(Module1.Deposit_Report, "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", TextCompare: false) == 0)
		{
			ReportDep2_80 reportDep2_2 = new ReportDep2_80();
			reportDep2_2.SetDataSource(Module1.localdata);
			reportDep2_2.PrintOptions.PrinterName = MyProject.Forms.FrmSettings.Print2.Text;
			reportDep2_2.PrintToPrinter(Module1.int_2, collated: true, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			reportDep2_2.Dispose();
		}
	}

	public static void Print_Picture1(string id, bool preview)
	{
		DataSet dataSet = Module1.connect("select * from Tb_Save_Image where id=" + id);
		Module1.localdata.ReportPic.Rows.Clear();
		Datalocal.ReportPicDataTable reportPic = Module1.localdata.ReportPic;
		object[] array = new object[1];
		DataRow dataRow = dataSet.Tables[0].Rows[0];
		string columnName = "pic";
		array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
		object[] array2 = array;
		bool[] array3 = new bool[1] { true };
		NewLateBinding.LateCall(reportPic, null, "AddReportPicRow", array2, null, null, array3, IgnoreReturn: true);
		if (array3[0])
		{
			dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
		}
		if (preview)
		{
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			ReportPictures reportPictures = new ReportPictures();
			reportPictures.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportPictures;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
		else
		{
			ReportPictures reportPictures2 = new ReportPictures();
			reportPictures2.SetDataSource(Module1.localdata);
			reportPictures2.PrintOptions.PrinterName = PrintSet.PrinterSettings.PrinterName;
			reportPictures2.PrintToPrinter(PrintSet.PrinterSettings.Copies, PrintSet.PrinterSettings.Collate, PrintSet.PrinterSettings.FromPage, PrintSet.PrinterSettings.ToPage);
			reportPictures2.Dispose();
		}
		Module1.localdata.ReportPic.Rows.Clear();
	}
}
