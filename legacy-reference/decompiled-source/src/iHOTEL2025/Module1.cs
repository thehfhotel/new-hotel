using System;
using System.Collections;
using System.Data;
using System.Data.OleDb;
using System.Data.SqlClient;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Drawing.Imaging;
using System.Drawing.Printing;
using System.IO;
using System.Management;
using System.Media;
using System.Net;
using System.Net.NetworkInformation;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Security.Cryptography;
using System.Text;
using System.Text.RegularExpressions;
using System.Windows.Forms;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[StandardModule]
internal sealed class Module1
{
	public static TripleDESCryptoServiceProvider DES;

	public static MD5CryptoServiceProvider MD5;

	public static string string_0;

	public static string information;

	public static string ProgramVersion;

	public static int close_program;

	public static bool SHOW_ICON;

	public static string Database_Mode;

	public static string AccessError;

	public static int data_index;

	public static int Whilecount;

	public static string DBPATH;

	public static string AconStr;

	public static OleDbConnection Aconn;

	public static OleDbDataAdapter da_access;

	public static string MYSQL_CONNECT_ERROR;

	public static bool bool_0;

	public static string PrinterName;

	public static string PathF;

	public static string loginName;

	public static string loginID;

	public static string loginMode;

	public static Datalocal localdata;

	public static Form Wainting;

	public static int Booking_Room_amount;

	public static bool bool_1;

	public static DateTime date_end;

	public static DateTime date_last;

	public static int always;

	public static string RegCode;

	public static int int_0;

	public static bool booking_use_Manternace;

	public static string COM_ID;

	public static string COM_DS;

	public static string P_MODE;

	public static bool IS_TRIAL;

	public static int ROOM_LENGHT;

	public static int POWER_Delay;

	public static int AutoLogout;

	public static bool bool_2;

	public static string checkin_mode;

	public static string Company_Head;

	public static string Company_Name;

	public static string Path_Program;

	public static bool CloseProgram;

	public static string RegPro;

	public static string string_1;

	public static bool RegOld;

	public static string VAT_OUT;

	public static ArrayList LOGIN_URL;

	public static bool MANUAL_POWER;

	public static bool ISfullscreen;

	public static int CHK_IN_Before;

	public static int CHK_Out;

	public static int CHK_Out_Alert;

	public static int CHK_Out_Before;

	public static int CHK_Out_H_price;

	public static int Maximum_Book;

	public static int Pority;

	public static decimal Vat_Over;

	public static decimal decimal_0;

	public static decimal decimal_1;

	public static bool bool_3;

	public static int int_1;

	public static int int_2;

	public static int int_3;

	public static int int_4;

	public static int int_5;

	public static int int_6;

	public static int int_7;

	public static int int_8;

	public static int copy_POS;

	public static string TEST11;

	public static bool IsListroom;

	public static bool AutoCalCust;

	public static bool HouseWifeMode;

	public static bool KichenMode;

	public static string Receipt_Report;

	public static string Receipt_print;

	public static string Receipt_preview;

	public static string POS_Report;

	public static string POS_print;

	public static string POS_preview;

	public static string Deposit_Report;

	public static string Deposit_preview;

	public static string inv_preview;

	public static string inv_print;

	public static string string_2;

	public static bool POWER_USED;

	public static string POWER_PORT;

	public static string CASH_PORT;

	public static string Cupon_Report;

	public static string Cupon_preview;

	public static string Tax_preview;

	public static string Cin_preview;

	public static string Cin_Print;

	public static string Settingstring;

	public static string datechar;

	public static SqlConnection conn2;

	public static SqlDataAdapter da2;

	public static string connstr2;

	public static bool CodeErr2;

	public static ArrayList cpuArr
	{
		get
		{
			ArrayList arrayList = new ArrayList();
			try
			{
				ManagementObjectSearcher managementObjectSearcher = new ManagementObjectSearcher("SELECT * FROM Win32_DiskDrive");
				foreach (ManagementObject item in managementObjectSearcher.Get())
				{
					try
					{
						string text = rot13(Strings.Trim(item["SerialNumber"].ToString())).ToUpper();
						string text2 = "(SN) " + Strings.Trim(item["Model"].ToString());
						string text3 = Strings.Trim(item["MediaType"].ToString()).ToLower();
						if (text3.IndexOf("fixed") != -1 && ((text.Length >= 8) & (text.Length <= 35)) && !((text[0] == text[1]) & (text[0] == text[2]) & (text[0] == text[3]) & (text[0] == text[4]) & (text[0] == text[5]) & (text[0] == text[6]) & (text[0] == text[7])))
						{
							string[] value = new string[2] { text, text2 };
							arrayList.Add(value);
						}
					}
					catch (Exception projectError)
					{
						ProjectData.SetProjectError(projectError);
						ProjectData.ClearProjectError();
					}
				}
				foreach (ManagementObject item2 in managementObjectSearcher.Get())
				{
					try
					{
						string text4 = rot13(Strings.Trim(item2["SerialNumber"].ToString())).ToUpper();
						string text5 = "(SN) " + Strings.Trim(item2["Model"].ToString());
						string text6 = Strings.Trim(item2["MediaType"].ToString()).ToLower();
						if (text6.IndexOf("fixed") == -1 && ((text4.Length >= 8) & (text4.Length <= 35)) && !((text4[0] == text4[1]) & (text4[0] == text4[2]) & (text4[0] == text4[3]) & (text4[0] == text4[4]) & (text4[0] == text4[5]) & (text4[0] == text4[6]) & (text4[0] == text4[7])))
						{
							string[] value2 = new string[2] { text4, text5 };
							arrayList.Add(value2);
						}
					}
					catch (Exception projectError2)
					{
						ProjectData.SetProjectError(projectError2);
						ProjectData.ClearProjectError();
					}
				}
			}
			catch (Exception projectError3)
			{
				ProjectData.SetProjectError(projectError3);
				ProjectData.ClearProjectError();
			}
			NetworkInterface[] allNetworkInterfaces = NetworkInterface.GetAllNetworkInterfaces();
			checked
			{
				try
				{
					int num = allNetworkInterfaces.Length - 1;
					int num2 = 0;
					while (true)
					{
						int num3 = num2;
						int num4 = num;
						if (num3 > num4)
						{
							break;
						}
						if (((Operators.CompareString(allNetworkInterfaces[num2].GetPhysicalAddress().ToString(), "", TextCompare: false) != 0) & (allNetworkInterfaces[num2].GetPhysicalAddress().ToString().Length == 12)) && allNetworkInterfaces[num2].Description.ToString().ToLower().IndexOf("vpn") == -1 && allNetworkInterfaces[num2].Description.ToString().ToLower().IndexOf("microsoft") == -1 && allNetworkInterfaces[num2].Description.ToString().ToLower().IndexOf("vmware") == -1 && allNetworkInterfaces[num2].GetPhysicalAddress().ToString().ToLower()
							.IndexOf("005056") != 0 && ((allNetworkInterfaces[num2].NetworkInterfaceType.ToString().ToLower().IndexOf("eth") != -1) | (allNetworkInterfaces[num2].NetworkInterfaceType.ToString().ToLower().IndexOf("lan") != -1) | (allNetworkInterfaces[num2].NetworkInterfaceType.ToString().ToLower().IndexOf("local") != -1)) && ((allNetworkInterfaces[num2].Name.ToLower().IndexOf("อ\u0e35") != -1) | (allNetworkInterfaces[num2].Name.ToLower().IndexOf("local") != -1) | (allNetworkInterfaces[num2].Name.ToLower().IndexOf("eth") != -1)))
						{
							string[] value3 = new string[2]
							{
								allNetworkInterfaces[num2].GetPhysicalAddress().ToString(),
								allNetworkInterfaces[num2].Description
							};
							arrayList.Add(value3);
						}
						num2++;
					}
					int num5 = allNetworkInterfaces.Length - 1;
					int num6 = 0;
					while (true)
					{
						int num7 = num6;
						int num4 = num5;
						if (num7 > num4)
						{
							break;
						}
						if (((Operators.CompareString(allNetworkInterfaces[num6].GetPhysicalAddress().ToString(), "", TextCompare: false) != 0) & (allNetworkInterfaces[num6].GetPhysicalAddress().ToString().Length == 12)) && allNetworkInterfaces[num6].Description.ToString().ToLower().IndexOf("vpn") == -1 && allNetworkInterfaces[num6].Description.ToString().ToLower().IndexOf("microsoft") == -1 && allNetworkInterfaces[num6].Description.ToString().ToLower().IndexOf("vmware") == -1 && allNetworkInterfaces[num6].GetPhysicalAddress().ToString().ToLower()
							.IndexOf("005056") != 0 && ((allNetworkInterfaces[num6].NetworkInterfaceType.ToString().ToLower().IndexOf("eth") != -1) | (allNetworkInterfaces[num6].NetworkInterfaceType.ToString().ToLower().IndexOf("lan") != -1) | (allNetworkInterfaces[num6].NetworkInterfaceType.ToString().ToLower().IndexOf("local") != -1)) && allNetworkInterfaces[num6].Name.ToLower().IndexOf("blue") != -1)
						{
							string[] value4 = new string[2]
							{
								allNetworkInterfaces[num6].GetPhysicalAddress().ToString(),
								allNetworkInterfaces[num6].Description
							};
							arrayList.Add(value4);
						}
						num6++;
					}
					int num8 = allNetworkInterfaces.Length - 1;
					int num9 = 0;
					while (true)
					{
						int num10 = num9;
						int num4 = num8;
						if (num10 > num4)
						{
							break;
						}
						if (((Operators.CompareString(allNetworkInterfaces[num9].GetPhysicalAddress().ToString(), "", TextCompare: false) != 0) & (allNetworkInterfaces[num9].GetPhysicalAddress().ToString().Length == 12)) && allNetworkInterfaces[num9].Description.ToString().ToLower().IndexOf("vpn") == -1 && allNetworkInterfaces[num9].Description.ToString().ToLower().IndexOf("microsoft") == -1 && allNetworkInterfaces[num9].Description.ToString().ToLower().IndexOf("vmware") == -1 && allNetworkInterfaces[num9].GetPhysicalAddress().ToString().ToLower()
							.IndexOf("005056") != 0 && ((allNetworkInterfaces[num9].NetworkInterfaceType.ToString().ToLower().IndexOf("wire") != -1) | (allNetworkInterfaces[num9].NetworkInterfaceType.ToString().ToLower().IndexOf("wifi") != -1) | (allNetworkInterfaces[num9].NetworkInterfaceType.ToString().ToLower().IndexOf("blue") != -1) | (allNetworkInterfaces[num9].NetworkInterfaceType.ToString().ToLower().IndexOf("wi-fi") != -1)))
						{
							string[] value5 = new string[2]
							{
								allNetworkInterfaces[num9].GetPhysicalAddress().ToString(),
								allNetworkInterfaces[num9].Description
							};
							arrayList.Add(value5);
						}
						num9++;
					}
					int num11 = allNetworkInterfaces.Length - 1;
					int num12 = 0;
					while (true)
					{
						int num13 = num12;
						int num4 = num11;
						if (num13 > num4)
						{
							break;
						}
						if (((Operators.CompareString(allNetworkInterfaces[num12].GetPhysicalAddress().ToString(), "", TextCompare: false) != 0) & (allNetworkInterfaces[num12].GetPhysicalAddress().ToString().Length == 12)) && allNetworkInterfaces[num12].Description.ToString().ToLower().IndexOf("vpn") == -1 && allNetworkInterfaces[num12].Description.ToString().ToLower().IndexOf("vmware") == -1 && allNetworkInterfaces[num12].GetPhysicalAddress().ToString().ToLower()
							.IndexOf("005056") != 0)
						{
							bool flag = false;
							int num14 = arrayList.Count - 1;
							int num15 = 0;
							while (true)
							{
								int num16 = num15;
								num4 = num14;
								if (num16 > num4)
								{
									break;
								}
								if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num15], new object[1] { 0 }, null), allNetworkInterfaces[num12].GetPhysicalAddress().ToString(), TextCompare: false))
								{
									flag = true;
								}
								num15++;
							}
							if (!flag)
							{
								string[] value6 = new string[2]
								{
									allNetworkInterfaces[num12].GetPhysicalAddress().ToString(),
									allNetworkInterfaces[num12].Description
								};
								arrayList.Add(value6);
							}
						}
						num12++;
					}
				}
				catch (Exception projectError4)
				{
					ProjectData.SetProjectError(projectError4);
					ProjectData.ClearProjectError();
				}
				DataSet dataSet = connect("select CompanyName,Company_Tax from TB_SETTINGS");
				string[] value7 = new string[2]
				{
					dataSet.Tables[0].Rows[0]["Company_Tax"].ToString() + "/" + dataSet.Tables[0].Rows[0]["CompanyName"].ToString(),
					"ช\u0e37\u0e48อบร\u0e34ษ\u0e31ท"
				};
				arrayList.Add(value7);
				return arrayList;
			}
		}
	}

	public static int id
	{
		get
		{
			int num = 0;
			DataSet dataSet = connect("select max(" + Column + ") as num from " + Tables);
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				num = 1;
			}
			else
			{
				try
				{
					num = Conversions.ToInteger(Operators.AddObject(dataSet.Tables[0].Rows[0]["num"], 1));
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					num = 1;
					ProjectData.ClearProjectError();
				}
			}
			dataSet.Dispose();
			return num;
		}
	}

	static Module1()
	{
		Class2.LH6iGfYz9j3MJ();
		DES = new TripleDESCryptoServiceProvider();
		MD5 = new MD5CryptoServiceProvider();
		string_0 = "0533558000396/ห\u0e49างห\u0e38\u0e49นส\u0e48วนจำก\u0e31ดมาลาด\u0e35,2522793655,2522793658,3321354455,141877993830A,0000CCDD0000,000D0A112118,000E68F48BD2,00112233AA00,0013727EA54C,0014C206F234,0017648EE728,0017648EE730,001A8701FF7D,001AA0D5247D,001CC048F1F0,001D093122B0,001D60E2AA27,001E4FF14E0F,001E90E37C43,001FC6ABAC11,001FD0ADA2E9,0021977B5BAA,0022FB3A1D7C,002354B98F2C,002381288EE7,00238129E15C,0023812B1746,0023812B1BCC,00241D543153,002421ABEA48,0024547B7D9C,0024547EA6C6,0025AB4E7974,0025AB6763AD,0025AB6DC283,002618191D6B,003067DE2C33,00883225560E,00AA33BB5566,00BB21354401,00E04C360354,00E04C78002B,00FF892ED6A8,02068897203D,10BEF5FB1E85,10FEED01A176,12A58977A227,14CC20044C16,14DAE9E9A540,14DDA9780CE1,162F689CF0E1,18CF5E01E690,18CF5E020BC0,18D6C70477B8,1C1B0D73A601,1C83411B6F99,1C83412299ED,1CB72C9C34DA,1CBDB97FF4D5,1EB57DA1C447,2047476DA2DF,20689DDCBAD7,20689DDCBADC,20CF302A58DD,20CF309E2FF3,20CF309E4A81,20CF30AF3011,20CF30AF3D01,20CF30CD570C,2C56DC3E08FB,2C6E85FDA657,3085A992E740,3C1E04472D54,3CF862FF4C27,3CF862FF4C28,40167E6E2868,408D5C64B277,40A8F04FD621,40F02F3A7332,441CA8231563,441CA8324826,4487FC496873,4487FC7CA15A,4487FC8ED2F5,4487FCFF3338,448A5B8EE19F,4874E2EA6920,4C72B92F22D6,4CCC6A27C6CB,4CCC6AA3AEA1,4CCC6AFA07F5,503AA0FD0E5B,50465D66BAE7,50465D68BA67,50465D6A177C,50E549829058,5404A65A4880,5404A6A4BBE8,54B80A630BD0,54E1ADE3DA8F,54E1ADE3DBA1,5C93A2A5C2F0,5EF93897E098,60029238965C,60E32718FACA,6CF04994CD09,6CF04994CD5F,6CF04994CD73,704D7B2EC8AF,7085C28DA2E4,70F1A1E91855,7427EA265796,7427EA555987,74D43594E560,74D435AD6ABC,78542E70B98B,7A79167371D9,7C67A24BD3C7,80CE62513F27,84C9B275C758,889FFA660511,8C89A5584521,8CEC4B518DDE,902B341164D9,902B347BDFE4,902B3490BFD8,909916250C5E,90E6BA0403E6,90E6BABB1D8C,94DE8047C4F2,94DE80B0A3D1,9829A68A4FF8,9C305B90ADB7,9CB654A9BF2E,A41F72988519,A81E846F6673,AA91458B9877,ACD1B805E982,B083FEAB652F,B091378B9859,B482FEC892CB,B4B6760080BC,B88198DE200A,B885849DF4AB,B8975A635215,B8975A63523B,BA8198DE2006,BCAEC5827C44,BCEE7B9C9098,BCEE7B9C92AD,C0335EE79E37,C4E9841AD406,C83A35CDBAC0,C83DD43F4361,CC2F71DB0119,CCB0DA9E858F,D050999AA406,D85DE245D13F,D85DE245D1BE,D897BA4EE930,D897BA4EE990,D89EF31B6149,D8CB8AA19344,D8CB8AA19367,DAC49793DB67,E06995FAB751,E0D55EC0E4D0,E0D55EF76B49,E0D55EF76BF4,E442A6386847,E4F3F5703801,E8DE27186737,EC086B1CDFE8,EC086B1CDFE9,EC0EC460BC30,EC8EB5D9D2CF,EEDE27186737,F079598BF2C8,F430B9A8AA79,F46D047B384A,F46D04ED8E76,F80F41663CC6,F80F41663D9E,F80F41663F96,F828199F66D7,F8BC127A6654,FC459626F21E,FC459626F263,FC4596B4D9E7,FC4596B4DADE,FCAA1452C81E,FCAA14A8F6FE,FCAA14CFB02A";
		ProgramVersion = "1.6.1";
		close_program = 0;
		SHOW_ICON = false;
		Database_Mode = "ACCESS";
		AccessError = "";
		data_index = 0;
		Whilecount = 0;
		DBPATH = Path_Program + "kphotel.accdb";
		AconStr = "";
		MYSQL_CONNECT_ERROR = "";
		bool_0 = true;
		PrinterName = "";
		loginName = "";
		loginID = "";
		loginMode = "";
		localdata = new Datalocal();
		Wainting = MyProject.Forms.frmWanting;
		Booking_Room_amount = 30;
		bool_1 = true;
		always = 0;
		RegCode = "1234";
		int_0 = 0;
		booking_use_Manternace = true;
		COM_ID = "";
		COM_DS = "";
		P_MODE = "DEMO";
		IS_TRIAL = true;
		ROOM_LENGHT = 3;
		POWER_Delay = 500;
		AutoLogout = 600;
		bool_2 = false;
		checkin_mode = "";
		Company_Head = "";
		Company_Name = "";
		CloseProgram = false;
		RegPro = "HOTEL";
		string_1 = "HOTELJSUD-NEW-SS";
		RegOld = false;
		VAT_OUT = "ป\u0e34ด";
		LOGIN_URL = new ArrayList();
		MANUAL_POWER = false;
		ISfullscreen = false;
		CHK_IN_Before = 559;
		CHK_Out = 1200;
		CHK_Out_Alert = 1;
		CHK_Out_Before = 1600;
		CHK_Out_H_price = 50;
		Maximum_Book = 20;
		Pority = 5;
		Vat_Over = 30m;
		decimal_0 = 3m;
		decimal_1 = 30m;
		bool_3 = false;
		int_1 = 1;
		int_2 = 1;
		int_3 = 1;
		int_4 = 1;
		int_5 = 1;
		int_6 = 1;
		int_7 = 1;
		int_8 = 1;
		copy_POS = 1;
		TEST11 = "jhklhkljhgilhoiugiugiu4rfga9rs47g98rg47sdf9rh7th97tgj97dfj9h7gj49dfgj74fh9kj7h9gk7jug9k7jfug9k79";
		IsListroom = false;
		AutoCalCust = false;
		HouseWifeMode = false;
		KichenMode = false;
		Receipt_Report = "";
		Receipt_print = "";
		Receipt_preview = "";
		POS_Report = "";
		POS_print = "";
		POS_preview = "";
		Deposit_Report = "";
		Deposit_preview = "";
		inv_preview = "";
		inv_print = "";
		string_2 = "";
		POWER_USED = false;
		POWER_PORT = "COM1";
		CASH_PORT = "ไม\u0e48ใช\u0e49งาน";
		Cupon_Report = "";
		Cupon_preview = "";
		Tax_preview = "";
		Cin_preview = "";
		Cin_Print = "";
		Settingstring = "";
		datechar = "'";
		CodeErr2 = false;
	}

	public static decimal Cdec0(string num)
	{
		num = num.Replace(",", "");
		if (Operators.CompareString(num, "", TextCompare: false) == 0)
		{
			return 0m;
		}
		if (!Versioned.IsNumeric(num))
		{
			return 0m;
		}
		return Conversions.ToDecimal(num);
	}

	public static void smethod_0()
	{
		DataSet dataSet = connect("select id from HT_Room_Status where room_date_oa=0");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			MyProject.Forms.FormUpdateDateRoomAll.ShowDialog();
		}
	}

	public static string FormatFileSize(long lngFileSize)
	{
		int num = 0;
		string text = "";
		float num2 = lngFileSize;
		while (!(Conversion.Int(num2) < 1000f))
		{
			num = checked(num + 1);
			num2 /= 1024f;
		}
		num2 = (float)Math.Round(num2, 2);
		return Strings.Format(num2, "#,##0.00") + " " + num switch
		{
			0 => "Bytes", 
			1 => "KB", 
			2 => "MB", 
			3 => "GB", 
			4 => "TB", 
			5 => "PB", 
			6 => "EB", 
			7 => "ZB", 
			8 => "YB", 
			_ => "Too big to compute :)", 
		};
	}

	public static string convertDate(DateTime dd)
	{
		if (Operators.CompareString(Database_Mode, "SQL", TextCompare: false) == 0)
		{
			return Conversions.ToString(dd);
		}
		return Strings.Format(dd.Day, "00") + "/" + Strings.Format(dd.Month, "00") + "/" + Conversions.ToString(checked(dd.Year + 543)) + " " + Strings.Format(dd.Hour, "00") + ":" + Strings.Format(dd.Minute, "00") + ":00";
	}

	public static DataSet connect(string command)
	{
		DataSet result;
		if (Operators.CompareString(Database_Mode, "SQL", TextCompare: false) == 0)
		{
			result = MSSQL.smethod_0(command);
		}
		else
		{
			DataSet dataSet = new DataSet();
			dataSet.Tables.Add(Conversions.ToString(0));
			try
			{
				da_access = new OleDbDataAdapter(command, Aconn);
				dataSet.Tables.Clear();
				da_access.Fill(dataSet);
			}
			catch (Exception ex)
			{
				ProjectData.SetProjectError(ex);
				Exception ex2 = ex;
				MessageBox.Show(ex2.Message);
				AccessError = ex2.Message;
				dataSet.Tables.Clear();
				dataSet.Tables.Add(Conversions.ToString(0));
				result = dataSet;
				ProjectData.ClearProjectError();
				goto IL_00a8;
			}
			result = dataSet;
		}
		goto IL_00a8;
		IL_00a8:
		return result;
	}

	public static void saveDatabaseIndex(string save_)
	{
		try
		{
			StreamWriter streamWriter = new StreamWriter(Path_Program + "dataindex.txt");
			streamWriter.Write(save_);
			streamWriter.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		try
		{
			data_index = Conversions.ToInteger(save_);
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			ProjectData.ClearProjectError();
		}
	}

	public static void loadDatabaseIndex()
	{
		string value = default(string);
		try
		{
			if (!File.Exists(Path_Program + "dataindex.txt"))
			{
				StreamWriter streamWriter = new StreamWriter(Path_Program + "dataindex.txt");
				streamWriter.Write("0");
				streamWriter.Close();
			}
			StreamReader streamReader = new StreamReader(Path_Program + "\\dataindex.txt", Encoding.Default);
			value = streamReader.ReadLine();
			streamReader.Close();
			streamReader = null;
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		try
		{
			data_index = Conversions.ToInteger(value);
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			ProjectData.ClearProjectError();
		}
	}

	public static void ReadDB_2018()
	{
		string return_Select_DB = MyProject.Forms.FormSelectDB.Return_Select_DB;
		string[] array = return_Select_DB.Split('|');
		if (return_Select_DB.IndexOf("|ACCESS") != -1)
		{
			Database_Mode = "ACCESS";
			MSSQL.MysqlServer = array[0];
			if (Operators.CompareString(MSSQL.MysqlServer, "kphotel.accdb", TextCompare: false) == 0)
			{
				DBPATH = Path_Program + "kphotel.accdb";
			}
			else if ((return_Select_DB.IndexOf("\\") == -1) & (return_Select_DB.IndexOf("/") == -1))
			{
				DBPATH = Path_Program + MSSQL.MysqlServer;
			}
			else
			{
				DBPATH = MSSQL.MysqlServer;
			}
			try
			{
				Aconn.Close();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			AconStr = "Provider=Microsoft.ACE.OLEDB.12.0;data source=" + DBPATH + ";Jet OLEDB:Database Password=foreverbu";
			Aconn = new OleDbConnection(AconStr);
			datechar = "#";
			return;
		}
		Database_Mode = "SQL";
		MSSQL.MysqlServer = array[0];
		MSSQL.MysqlUsername = array[1];
		MSSQL.MysqlPassword = array[2];
		object obj = "";
		if (MSSQL.MysqlServer.IndexOf(",") != -1)
		{
			obj = MSSQL.MysqlServer.Substring(MSSQL.MysqlServer.IndexOf(","));
			MSSQL.MysqlServer = MSSQL.MysqlServer.Substring(0, MSSQL.MysqlServer.IndexOf(","));
		}
		if (MSSQL.MysqlPassword.IndexOf(":EG:") != -1)
		{
			MSSQL.MysqlPassword = FormEN_DE.Decrypt1(MSSQL.MysqlPassword.Replace(":EG:", ""), "ruj5de4");
		}
		if (MSSQL.MysqlServer.IndexOf(":EG:") != -1)
		{
			MSSQL.MysqlServer = FormEN_DE.Decrypt1(MSSQL.MysqlServer.Replace(":EG:", ""), "ruj5de4");
		}
		if (MSSQL.MysqlUsername.IndexOf(":EG:") != -1)
		{
			MSSQL.MysqlUsername = FormEN_DE.Decrypt1(MSSQL.MysqlUsername.Replace(":EG:", ""), "ruj5de4");
		}
		try
		{
			MSSQL.MysqlDatabase = array[3];
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			MSSQL.MysqlDatabase = "KPHOTEL";
			ProjectData.ClearProjectError();
		}
		if (Operators.ConditionalCompareObjectNotEqual(obj, "", TextCompare: false))
		{
			MSSQL.MysqlServer = Conversions.ToString(Operators.ConcatenateObject(MSSQL.MysqlServer, obj));
		}
		datechar = "'";
		DBPATH = MSSQL.MysqlServer;
		MSSQL.connectmssql();
		if (!((MYSQL_CONNECT_ERROR.ToLower().IndexOf("cannot open database") != -1) & (return_Select_DB.IndexOf("|CloudServer") == -1)) || MessageBox.Show("ไม\u0e48พบฐานข\u0e49อม\u0e39ล ค\u0e38ณต\u0e49องการสร\u0e49างฐานข\u0e49อม\u0e39ลหร\u0e37อไม\u0e48", "สร\u0e49างฐานข\u0e49อม\u0e39ล", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.Yes)
		{
			return;
		}
		string text = "";
		FolderBrowserDialog folderBrowserDialog = new FolderBrowserDialog();
		while (true)
		{
			text = "";
			folderBrowserDialog.ShowNewFolderButton = true;
			folderBrowserDialog.Description = "กร\u0e38ณาเล\u0e37อก Folder เก\u0e47บไฟล\u0e4cฐานข\u0e49อม\u0e39ล";
			folderBrowserDialog.Tag = "กร\u0e38ณาเล\u0e37อก Folder เก\u0e47บไฟล\u0e4cฐานข\u0e49อม\u0e39ล";
			folderBrowserDialog.SelectedPath = Path_Program;
			folderBrowserDialog.RootFolder = Environment.SpecialFolder.MyComputer;
			if (folderBrowserDialog.ShowDialog() == DialogResult.OK)
			{
				text = folderBrowserDialog.SelectedPath;
			}
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				if (MessageBox.Show("โปรแกรมจะสร\u0e49างไฟล\u0e4cฐานข\u0e49อม\u0e39ลไว\u0e49ท\u0e35\u0e48 " + text + "\\Database", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
				{
					MSSQL.Create_MssqlDatabase(text);
					MessageBox.Show("กร\u0e38ณาเป\u0e34ดโปรแกรมใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "สร\u0e49างฐานข\u0e49อม\u0e39ล", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
					break;
				}
				continue;
			}
			MessageBox.Show("ยกเล\u0e34ก\r\nกร\u0e38ณาเป\u0e34ดโปรแกรมใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "สร\u0e49างฐานข\u0e49อม\u0e39ล", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			break;
		}
		close_program = 1;
	}

	public static void LOG(string details)
	{
		object left = "INSERT INTO [HT_Log]";
		left = Operators.ConcatenateObject(left, "([details]");
		left = Operators.ConcatenateObject(left, ",[Emp_name]");
		left = Operators.ConcatenateObject(left, ",[DODATE])");
		left = Operators.ConcatenateObject(left, "VALUES(");
		left = Operators.ConcatenateObject(left, string.Concat("'" + details, "'"));
		left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", loginName), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Strings.Format(DateTime.Now, "dd/MM/yyyy HH:mm:ss"), "'"));
		left = Operators.ConcatenateObject(left, ")");
		connect(Conversions.ToString(left));
	}

	public static void loginURLsplit(string CurrentLine)
	{
		LOGIN_URL.Clear();
		string[] array = CurrentLine.Split(',');
		checked
		{
			int num = array.Length - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (Operators.CompareString(Strings.Trim(array[num2]), "", TextCompare: false) != 0)
					{
						LOGIN_URL.Add(Strings.Trim(array[num2]));
					}
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public static string Convert_Minites_TO_Hour(int mmm)
	{
		return Conversions.ToString(mmm / 60) + ":" + (mmm % 60).ToString("00");
	}

	public static void smethod_1()
	{
		DataSet dataSet = connect("select * from HT_Rooms");
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
					Power_set(Conversions.ToString(dataSet.Tables[0].Rows[num2]["Room_no"]), dataSet.Tables[0].Rows[num2]["Room_Power_STATUS"].ToString().ToUpper(), "", "เป\u0e34ดป\u0e34ดไฟใหม\u0e48ท\u0e31\u0e49งหมด");
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public static int CountCharacter(string value, char ch)
	{
		return checked(Strings.Len(value) - Strings.Len(Strings.Replace(value, Conversions.ToString(ch), "")));
	}

	public static int smethod_2(string str)
	{
		str = str.Replace("\u0e31", "");
		str = str.Replace("\u0e34", "");
		str = str.Replace("\u0e35", "");
		str = str.Replace("\u0e36", "");
		str = str.Replace("\u0e37", "");
		str = str.Replace("\u0e38", "");
		str = str.Replace("\u0e39", "");
		str = str.Replace("\u0e3a", "");
		str = str.Replace("\u0e47", "");
		str = str.Replace("\u0e48", "");
		str = str.Replace("\u0e49", "");
		str = str.Replace("\u0e4a", "");
		str = str.Replace("\u0e4b", "");
		str = str.Replace("\u0e4c", "");
		str = str.Replace("\u0e4d", "");
		str = str.Replace("\u0e4e", "");
		return str.Length;
	}

	public static void save_deposit()
	{
		StreamWriter streamWriter = File.CreateText(PathF + "\\SetAutoDep.txt");
		streamWriter.WriteLine(bool_2);
		streamWriter.Flush();
		streamWriter.Close();
	}

	public static Bitmap RotateImg(Bitmap bmpimage, float angle)
	{
		int width = bmpimage.Width;
		int height = bmpimage.Height;
		PixelFormat pixelFormat = PixelFormat.Undefined;
		pixelFormat = bmpimage.PixelFormat;
		Bitmap bitmap = new Bitmap(width, height, pixelFormat);
		Graphics graphics = Graphics.FromImage(bitmap);
		graphics.DrawImageUnscaled(bmpimage, 1, 1);
		graphics.Dispose();
		GraphicsPath graphicsPath = new GraphicsPath();
		RectangleF rect = new RectangleF(0f, 0f, width, height);
		graphicsPath.AddRectangle(rect);
		Matrix matrix = new Matrix();
		matrix.Rotate(angle);
		RectangleF bounds = graphicsPath.GetBounds(matrix);
		Bitmap bitmap2 = new Bitmap(Convert.ToInt32(bounds.Width), Convert.ToInt32(bounds.Height), pixelFormat);
		graphics = Graphics.FromImage(bitmap2);
		graphics.TranslateTransform(0f - bounds.X, 0f - bounds.Y);
		graphics.RotateTransform(angle);
		graphics.InterpolationMode = InterpolationMode.HighQualityBilinear;
		graphics.DrawImageUnscaled(bitmap, 0, 0);
		graphics.Dispose();
		bitmap.Dispose();
		return bitmap2;
	}

	public static void smethod_3()
	{
		if ((Operators.CompareString(CASH_PORT, "ไม\u0e48ใช\u0e49งาน", TextCompare: false) != 0) & (Operators.CompareString(CASH_PORT, "", TextCompare: false) != 0))
		{
			try
			{
				MyProject.Forms.frmMain1.SerialPort2.PortName = CASH_PORT;
				MyProject.Forms.frmMain1.SerialPort2.Open();
				MyProject.Forms.frmMain1.SerialPort2.WriteLine(CASH_PORT);
				MyProject.Forms.frmMain1.SerialPort2.Close();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	public static void load_book_num()
	{
		if (!File.Exists(PathF + "\\SetBookingNum.txt"))
		{
			StreamWriter streamWriter = File.CreateText(PathF + "\\SetBookingNum.txt");
			streamWriter.WriteLine("20");
			streamWriter.Flush();
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(PathF + "\\SetBookingNum.txt");
		string str = streamReader.ReadLine();
		streamReader.Close();
		try
		{
			Booking_Room_amount = Conversions.ToInteger(Strings.Trim(str));
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public static void save_book_num(string nn)
	{
		StreamWriter streamWriter = File.CreateText(PathF + "\\SetBookingNum.txt");
		streamWriter.WriteLine(nn);
		streamWriter.Flush();
		streamWriter.Close();
		Booking_Room_amount = Conversions.ToInteger(Strings.Trim(nn));
	}

	public static void load_deposit()
	{
		if (!File.Exists(PathF + "\\SetAutoDep.txt"))
		{
			StreamWriter streamWriter = File.CreateText(PathF + "\\SetAutoDep.txt");
			streamWriter.WriteLine("False");
			streamWriter.Flush();
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(PathF + "\\SetAutoDep.txt");
		string value = streamReader.ReadLine();
		streamReader.Close();
		try
		{
			bool_2 = Conversions.ToBoolean(value);
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public static void SET_STATUS_BOOKING(string BOOK_NO, bool bool_4 = false)
	{
		DataSet dataSet = connect("select * from HT_Book_Ds where book_no='" + BOOK_NO + "'");
		if (!(dataSet.Tables[0].Rows.Count == 0 || bool_4))
		{
			return;
		}
		dataSet = connect("select * from HT_Book_H where book_id='" + BOOK_NO + "'");
		if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[0]["book_price_pay"], 0, TextCompare: false))
		{
			string text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ค\u0e37นเง\u0e34นล\u0e39กค\u0e49า ", dataSet.Tables[0].Rows[0]["Book_Price_Pay"]), " บาท "), "\r\n"));
			DataSet dataSet2 = connect("select * from HT_CheckIn_Pay where cin_no='" + BOOK_NO + "' order by id desc");
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				string sIR_PAY = GetSIR_PAY();
				Insert_Pay(BOOK_NO, "การจองแบบระบ\u0e38ห\u0e49อง", DateTime.Now, Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_cash"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_credit"])), "ค\u0e37นเง\u0e34นจองห\u0e49อง", Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_ds_priceTotal"])), "รายการ", sIR_PAY, Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_ID"]), "P001", 1m, Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_ds_priceTotal"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_ds_priceTotal"])), "", Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_free"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_tran"])), Conversions.ToDecimal(Operators.NegateObject(dataSet2.Tables[0].Rows[0]["cin_pay_web"])));
				if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["cin_pay_cash"], 0, TextCompare: false))
				{
					text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject("\r\nเง\u0e34นสด ", dataSet2.Tables[0].Rows[0]["cin_pay_cash"])));
				}
				if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["cin_pay_credit"], 0, TextCompare: false))
				{
					text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject("\r\nบ\u0e31ตร ", dataSet2.Tables[0].Rows[0]["cin_pay_credit"])));
				}
				if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["cin_pay_tran"], 0, TextCompare: false))
				{
					text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject("\r\nโอน ", dataSet2.Tables[0].Rows[0]["cin_pay_tran"])));
				}
				if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["cin_pay_free"], 0, TextCompare: false))
				{
					text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject("\r\nฟร\u0e35 ", dataSet2.Tables[0].Rows[0]["cin_pay_free"])));
				}
				if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["cin_pay_web"], 0, TextCompare: false))
				{
					text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject("\r\nเว\u0e47ป ", dataSet2.Tables[0].Rows[0]["cin_pay_web"])));
				}
			}
			MessageBox.Show(text, "ค\u0e37นเง\u0e34น", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
			UPDATE_MONEY(Conversions.ToString(dataSet.Tables[0].Rows[0]["Book_Cust_ID"]), Conversions.ToDecimal(dataSet.Tables[0].Rows[0]["Book_Price_Pay"]), "DEL", "ค\u0e37นเง\u0e34นจากการยกเล\u0e34กใบจอง " + BOOK_NO);
		}
		connect("update HT_Book_H set book_status='ยกเล\u0e34ก' where book_id='" + BOOK_NO + "'");
	}

	public static string GEN_ACCOUNT(string NO)
	{
		NO = Strings.Trim(NO);
		if (Operators.CompareString(NO, "", TextCompare: false) != 0)
		{
			int num = 1;
			try
			{
				DataSet dataSet = connect("select top 1  Pay_Account from TB_Pay_History where Pay_Account like '" + NO + "-%' order by Pay_Account desc");
				if (dataSet.Tables[0].Rows.Count != 0)
				{
					num = checked(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["Pay_Account"].ToString().Replace(NO + "-", "")) + 1);
				}
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			return NO + "-" + Strings.Format(num, "000000");
		}
		return "";
	}

	public static void INSERT_REPAIR(string R_NO, string R_BY, string R_NOTE)
	{
		object left = "INSERT INTO [HT_Rooms_Repair]";
		left = Operators.ConcatenateObject(left, "([room_no]");
		left = Operators.ConcatenateObject(left, ",[Repair_date]");
		left = Operators.ConcatenateObject(left, ",[Repair_by]");
		left = Operators.ConcatenateObject(left, ",[Repair_note])");
		left = Operators.ConcatenateObject(left, "VALUES");
		left = Operators.ConcatenateObject(left, "(");
		left = Operators.ConcatenateObject(left, string.Concat("'" + R_NO, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + R_BY, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + R_NOTE, "'"));
		left = Operators.ConcatenateObject(left, ")");
		connect(Conversions.ToString(left));
	}

	public static void ReadSettingsConfig()
	{
		IS_TRIAL = true;
		bool flag = false;
		ArrayList arrayList = cpuArr;
		checked
		{
			int num = arrayList.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				object objectValue = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(arrayList[num2], new object[1] { 0 }, null));
				string instance = string_0;
				object[] array = new object[1] { RuntimeHelpers.GetObjectValue(objectValue) };
				object[] arguments = array;
				bool[] array2 = new bool[1] { true };
				object left = NewLateBinding.LateGet(instance, null, "IndexOf", arguments, null, null, array2);
				if (array2[0])
				{
					objectValue = RuntimeHelpers.GetObjectValue(array[0]);
				}
				if (Operators.ConditionalCompareObjectNotEqual(left, -1, TextCompare: false))
				{
					if (Operators.CompareString(CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(objectValue, "HOTEL"))), RegCode, TextCompare: false) == 0)
					{
						RegOld = true;
						flag = true;
						COM_ID = (string)RuntimeHelpers.GetObjectValue(objectValue);
						COM_DS = (string)RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(arrayList[num2], new object[1] { 1 }, null));
					}
					if (Operators.CompareString(CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(objectValue, "HOTEL2"))), RegCode, TextCompare: false) == 0)
					{
						RegOld = true;
						flag = true;
						COM_ID = (string)RuntimeHelpers.GetObjectValue(objectValue);
						COM_DS = (string)RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(arrayList[num2], new object[1] { 1 }, null));
					}
				}
				num2++;
			}
			if (!flag)
			{
				int num5 = arrayList.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					object objectValue2 = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(arrayList[num6], new object[1] { 0 }, null));
					if (Operators.CompareString(CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(objectValue2, string_1))), RegCode, TextCompare: false) == 0)
					{
						flag = true;
						COM_ID = (string)RuntimeHelpers.GetObjectValue(objectValue2);
						COM_DS = (string)RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(arrayList[num6], new object[1] { 1 }, null));
					}
					num6++;
				}
			}
			if (!flag)
			{
				frmReg frmReg2 = new frmReg();
				frmReg2.ShowDialog();
			}
			else
			{
				IS_TRIAL = false;
			}
		}
	}

	public static string Encrypt1(string stringToEncrypt, string key)
	{
		DES.Key = MD5Hash(key);
		DES.Mode = CipherMode.ECB;
		byte[] bytes = Encoding.ASCII.GetBytes(stringToEncrypt);
		return Convert.ToBase64String(DES.CreateEncryptor().TransformFinalBlock(bytes, 0, bytes.Length));
	}

	public static byte[] MD5Hash(string value)
	{
		return MD5.ComputeHash(Encoding.ASCII.GetBytes(value));
	}

	public static string rot13(string input)
	{
		input = Regex.Replace(input, "[^A-Za-z0-9\\-/]", "");
		input = input.Replace("O", "0");
		return input;
	}

	public static string WEB_LOAD_GET(string url)
	{
		string result = "";
		try
		{
			HttpWebRequest httpWebRequest = (HttpWebRequest)WebRequest.Create(url);
			httpWebRequest.Accept = "*/*";
			httpWebRequest.AllowAutoRedirect = false;
			httpWebRequest.UserAgent = "http_requester/0.1";
			httpWebRequest.Timeout = 5000;
			httpWebRequest.Method = "GET";
			HttpWebResponse httpWebResponse = (HttpWebResponse)httpWebRequest.GetResponse();
			if (httpWebResponse.StatusCode == HttpStatusCode.OK)
			{
				StreamReader streamReader = new StreamReader(httpWebResponse.GetResponseStream());
				result = streamReader.ReadToEnd();
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		return result;
	}

	public static string CalculateMD5(string s)
	{
		return CalculateMD5(s, new UnicodeEncoding());
	}

	private static string CalculateMD5(string s, Encoding enc)
	{
		MD5CryptoServiceProvider mD5CryptoServiceProvider = new MD5CryptoServiceProvider();
		StringBuilder stringBuilder = new StringBuilder(32);
		byte[] bytes = enc.GetBytes(s);
		byte[] array = mD5CryptoServiceProvider.ComputeHash(bytes);
		byte[] array2 = array;
		foreach (byte b in array2)
		{
			stringBuilder.Append($"{b:X2}");
		}
		return stringBuilder.ToString();
	}

	public static void GEN_Cupon(string room_no, string cin_no, DateTime cupon_date, int cupon_num, bool AlwayAdd)
	{
		if (cupon_num == 0)
		{
			return;
		}
		int num = 1;
		checked
		{
			if (!AlwayAdd)
			{
				DataSet dataSet = connect("select * from HT_Cupon where cupon_cin_room='" + room_no + "' and cupon_date='" + Conversions.ToString(cupon_date) + "' and cupon_cin_no='" + cin_no + "'");
				if (cupon_num <= dataSet.Tables[0].Rows.Count)
				{
					return;
				}
				num += dataSet.Tables[0].Rows.Count;
			}
			int num2 = num;
			while (true)
			{
				int num3 = num2;
				int num4 = cupon_num;
				if (num3 <= num4)
				{
					object right = Module1.get_id("HT_Cupon", "cupon_no");
					object left = "INSERT INTO [HT_Cupon]";
					left = Operators.ConcatenateObject(left, "([cupon_no]");
					left = Operators.ConcatenateObject(left, ",[cupon_cin_no]");
					left = Operators.ConcatenateObject(left, ",[cupon_cin_room]");
					left = Operators.ConcatenateObject(left, ",[cupon_date],[cupon_gen_date],[cupon_by])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, right);
					left = Operators.ConcatenateObject(left, string.Concat(",'" + cin_no, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + room_no, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(cupon_date.Date), "'"));
					left = Operators.ConcatenateObject(left, ",getdate()");
					left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", loginName), "'"));
					left = Operators.ConcatenateObject(left, ")");
					connect(Conversions.ToString(left));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public static string Decrypt1(string encryptedString, string key)
	{
		string @string = default(string);
		try
		{
			DES.Key = MD5Hash(key);
			DES.Mode = CipherMode.ECB;
			byte[] array = Convert.FromBase64String(encryptedString);
			@string = Encoding.ASCII.GetString(DES.CreateDecryptor().TransformFinalBlock(array, 0, array.Length));
			return @string;
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			MessageBox.Show("Invalid Key", "Decryption Failed", MessageBoxButtons.OK, MessageBoxIcon.Exclamation);
			ProjectData.ClearProjectError();
		}
		return @string;
	}

	public static Color RandomColor(int nn)
	{
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 0m) == 0)
		{
			return Color.Violet;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 1m) == 0)
		{
			return Color.Pink;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 2m) == 0)
		{
			return Color.MediumOrchid;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 3m) == 0)
		{
			return Color.MediumSlateBlue;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 4m) == 0)
		{
			return Color.SlateBlue;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 5m) == 0)
		{
			return Color.RoyalBlue;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 6m) == 0)
		{
			return Color.HotPink;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 7m) == 0)
		{
			return Color.Fuchsia;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 8m) == 0)
		{
			return Color.LightPink;
		}
		if (decimal.Compare(Math.Floor(new decimal(nn % 10)), 9m) == 0)
		{
			return Color.DodgerBlue;
		}
		return Color.LemonChiffon;
	}

	public static void Change_Room(string from_room, string To_room, string ss, bool ch_p, string cin_cust_price)
	{
		DataSet dataSet = connect("select * from View_CheckIn_Ds where cin_room_status<>'Check-Out' and  cin_room_no='" + from_room + "'");
		if (ch_p)
		{
			DataSet dataSet2 = connect("select * from HT_Rooms where room_no='" + To_room + "'");
			DataSet dataSet3 = connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms_Price  where room_type='", dataSet2.Tables[0].Rows[0]["room_type"]), "' and room_custtype='"), cin_cust_price), "' ")));
			object left = "INSERT INTO [HT_Changed_Room]";
			left = Operators.ConcatenateObject(left, "([cin_no]");
			left = Operators.ConcatenateObject(left, ",[room_before]");
			left = Operators.ConcatenateObject(left, ",[room_after]");
			left = Operators.ConcatenateObject(left, ",[change_date],[room_before_price],[Note],[ToPrice])");
			left = Operators.ConcatenateObject(left, "VALUES");
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject("'", dataSet.Tables[0].Rows[0]["cin_no"]), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + from_room, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + To_room, "'"));
			left = Operators.ConcatenateObject(left, ",getdate()");
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(",", dataSet.Tables[0].Rows[0]["cin_room_price"]));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + ss, "'"));
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", dataSet3.Tables[0].Rows[0]["room_price"]), "'"));
			left = Operators.ConcatenateObject(left, ")");
			connect(Conversions.ToString(left));
		}
		else
		{
			object left2 = "INSERT INTO [HT_Changed_Room]";
			left2 = Operators.ConcatenateObject(left2, "([cin_no]");
			left2 = Operators.ConcatenateObject(left2, ",[room_before]");
			left2 = Operators.ConcatenateObject(left2, ",[room_after]");
			left2 = Operators.ConcatenateObject(left2, ",[change_date],[room_before_price],[Note],[ToPrice])");
			left2 = Operators.ConcatenateObject(left2, "VALUES");
			left2 = Operators.ConcatenateObject(left2, "(");
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject("'", dataSet.Tables[0].Rows[0]["cin_no"]), "'"));
			left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + from_room, "'"));
			left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + To_room, "'"));
			left2 = Operators.ConcatenateObject(left2, ",getdate()");
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(",", dataSet.Tables[0].Rows[0]["cin_room_price"]));
			left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + ss, "'"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", dataSet.Tables[0].Rows[0]["cin_room_price"]), "'"));
			left2 = Operators.ConcatenateObject(left2, ")");
			connect(Conversions.ToString(left2));
		}
	}

	public static void save_power_log(string room_no, string power_status, string Note)
	{
		DataSet dataSet = connect("select id from HT_POWER_LOG where room_no='" + room_no + "' and ROOM_POWER_END_BY=''");
		string left = "ป\u0e34ด";
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			left = "เป\u0e34ด";
		}
		if (Operators.CompareString(power_status.ToUpper(), "ON", TextCompare: false) == 0)
		{
			if (Operators.CompareString(left, "เป\u0e34ด", TextCompare: false) != 0)
			{
				object left2 = "";
				left2 = Operators.ConcatenateObject(left2, "INSERT INTO [HT_POWER_LOG]");
				left2 = Operators.ConcatenateObject(left2, "([ROOM_NO]");
				left2 = Operators.ConcatenateObject(left2, ",[ROOM_POWER_START]");
				left2 = Operators.ConcatenateObject(left2, ",[ROOM_POWER_START_BY]");
				left2 = Operators.ConcatenateObject(left2, ",[ROOM_POWER_END_BY]");
				left2 = Operators.ConcatenateObject(left2, ",[ROOM_POWER_NOTE]");
				left2 = Operators.ConcatenateObject(left2, ",[ROOM_POWER_NOTE2]");
				left2 = Operators.ConcatenateObject(left2, ")");
				left2 = Operators.ConcatenateObject(left2, "VALUES(");
				left2 = Operators.ConcatenateObject(left2, string.Concat("'" + room_no, "'"));
				left2 = Operators.ConcatenateObject(left2, ",GETDATE()");
				left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", loginName), "'"));
				left2 = Operators.ConcatenateObject(left2, ",''");
				left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Note, "'"));
				left2 = Operators.ConcatenateObject(left2, ",''");
				left2 = Operators.ConcatenateObject(left2, ")");
				connect(Conversions.ToString(left2));
			}
		}
		else if (Operators.CompareString(left, "เป\u0e34ด", TextCompare: false) == 0)
		{
			connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_POWER_LOG SET ROOM_POWER_END=GETDATE(),ROOM_POWER_END_BY='", loginName), "',ROOM_POWER_NOTE2='"), Note), "' where room_no='"), room_no), "' and ROOM_POWER_END_BY=''")));
		}
	}

	public static void GenSmartCard()
	{
		try
		{
			StreamWriter streamWriter = File.CreateText(PathF + "\\kpsmartcard.txt");
			streamWriter.WriteLine("ok");
			streamWriter.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public static void Power_set2(string room_no, string status, string code_open, string code_close)
	{
		if (!POWER_USED)
		{
			return;
		}
		if (Operators.CompareString(status.ToUpper(), "ON", TextCompare: false) == 0)
		{
			if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) == 0)
			{
				if (Operators.CompareString(code_open, "", TextCompare: false) != 0)
				{
					MyProject.Forms.frmMain1.URL_ON_OFF.Items.Add(code_open);
				}
			}
			else if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) != 0 && Operators.CompareString(code_open, "", TextCompare: false) != 0)
			{
				MyProject.Forms.frmMain1.URL_ON_OFF_SERIALS.Items.Add(code_open);
			}
		}
		else
		{
			if (Operators.CompareString(status.ToUpper(), "OFF", TextCompare: false) != 0)
			{
				return;
			}
			if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) == 0)
			{
				if (Operators.CompareString(code_close, "", TextCompare: false) != 0)
				{
					MyProject.Forms.frmMain1.URL_ON_OFF.Items.Add(code_close);
				}
			}
			else if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) != 0 && Operators.CompareString(code_close, "", TextCompare: false) != 0)
			{
				MyProject.Forms.frmMain1.URL_ON_OFF_SERIALS.Items.Add(code_close);
			}
		}
	}

	public static void Power_set(string room_no, string status, string manual_code, string PW_NOTE)
	{
		MyProject.Forms.frmMain1.TimerOnoff.Enabled = false;
		if (POWER_USED)
		{
			DataSet dataSet = connect("select * from HT_Rooms where room_no='" + room_no + "'");
			if (Operators.CompareString(status.ToUpper(), "ON", TextCompare: false) == 0)
			{
				if (Operators.CompareString(manual_code, "", TextCompare: false) == 0)
				{
					manual_code = dataSet.Tables[0].Rows[0]["Room_Power_OPEN"].ToString();
				}
				if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) == 0)
				{
					if (Operators.CompareString(manual_code, "", TextCompare: false) != 0)
					{
						MyProject.Forms.frmMain1.URL_ON_OFF.Items.Insert(0, manual_code);
						connect("update HT_Rooms set Room_Power_STATUS='on' where room_no='" + room_no + "'");
					}
				}
				else if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) != 0)
				{
					MyProject.Forms.frmMain1.URL_ON_OFF_SERIALS.Items.Insert(0, manual_code);
					connect("update HT_Rooms set Room_Power_STATUS='on' where room_no='" + room_no + "'");
				}
				save_power_log(room_no, status, PW_NOTE);
			}
			else if (Operators.CompareString(status.ToUpper(), "OFF", TextCompare: false) == 0)
			{
				if (Operators.CompareString(manual_code, "", TextCompare: false) == 0)
				{
					manual_code = dataSet.Tables[0].Rows[0]["Room_Power_CLOSE"].ToString();
				}
				if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) == 0)
				{
					if (Operators.CompareString(manual_code, "", TextCompare: false) != 0)
					{
						MyProject.Forms.frmMain1.URL_ON_OFF.Items.Insert(0, manual_code);
						connect("update HT_Rooms set Room_Power_STATUS='off' where room_no='" + room_no + "'");
					}
				}
				else if (Operators.CompareString(POWER_PORT.ToUpper(), "HTTP", TextCompare: false) != 0)
				{
					MyProject.Forms.frmMain1.URL_ON_OFF_SERIALS.Items.Insert(0, manual_code);
					connect("update HT_Rooms set Room_Power_STATUS='off' where room_no='" + room_no + "'");
				}
				save_power_log(room_no, status, PW_NOTE);
			}
		}
		if (MyProject.Forms.frmMain1.URL_ON_OFF.Items.Count != 0)
		{
			MyProject.Forms.frmMain1.TimerOnoff.Enabled = true;
		}
	}

	public static string Power_Status(string room_no)
	{
		DataSet dataSet = connect("select * from HT_Rooms where room_no='" + room_no + "'");
		return dataSet.Tables[0].Rows[0]["Room_Power_STATUS"].ToString().ToUpper();
	}

	public static int CheckLastStay(string cust_id)
	{
		int result = 100;
		DataSet dataSet = connect("select top 1 * from HT_CheckIn_H where cin_cust_no='" + cust_id + "' order by cin_date desc");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			result = checked((int)DateAndTime.DateDiff(DateInterval.Month, Conversions.ToDate(dataSet.Tables[0].Rows[0]["cin_date"]), DateTime.Now));
		}
		return result;
	}

	public static bool check_round_bill()
	{
		DataSet dataSet = connect("select id from HT_Round_Bill where round_end is null");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return false;
		}
		return true;
	}

	public static void KeyPattern(KeyPressEventArgs KeyPttn, Control Target)
	{
		if (KeyPttn.KeyChar == '\r')
		{
			KeyPttn.Handled = true;
			SendKeys.Send("{TAB}");
		}
		else if (KeyPttn.KeyChar == '\u001b')
		{
			KeyPttn.Handled = true;
			Del(Target);
		}
	}

	public static void Set_room_pority()
	{
		MyProject.Forms.frmMain1.Cursor = Cursors.WaitCursor;
		DataSet dataSet = connect("select * from HT_Rooms order by room_use_count,room_no");
		checked
		{
			if (dataSet.Tables[0].Rows.Count >= Pority)
			{
				object right = Math.Floor((double)dataSet.Tables[0].Rows.Count / (double)Pority);
				int num = 1;
				object left = 1;
				int num2 = dataSet.Tables[0].Rows.Count - 1;
				int num3 = 0;
				while (true)
				{
					int num4 = num3;
					int num5 = num2;
					if (num4 > num5)
					{
						break;
					}
					if (num == Pority)
					{
						connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_Rooms set Room_Polity=" + Conversions.ToString(num), " where id="), dataSet.Tables[0].Rows[num3]["id"])));
					}
					else if (Operators.ConditionalCompareObjectNotEqual(left, right, TextCompare: false))
					{
						connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_Rooms set Room_Polity=" + Conversions.ToString(num), " where id="), dataSet.Tables[0].Rows[num3]["id"])));
						left = Operators.AddObject(left, 1);
					}
					else
					{
						connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_Rooms set Room_Polity=" + Conversions.ToString(num), " where id="), dataSet.Tables[0].Rows[num3]["id"])));
						num++;
						left = 1;
					}
					num3++;
				}
			}
			dataSet.Dispose();
			MyProject.Forms.frmMain1.Cursor = Cursors.Default;
		}
	}

	public static void PlaySound()
	{
		try
		{
			SoundPlayer soundPlayer = new SoundPlayer();
			soundPlayer.SoundLocation = PathF + "alarma.wav";
			soundPlayer.Load();
			soundPlayer.Play();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public static void Del(Control Target)
	{
		if (Target is TextBox)
		{
			((TextBox)Target).Clear();
		}
		else if (Target is Label)
		{
			((Label)Target).Text = "";
		}
		else if (Target is ComboBox)
		{
			((ComboBox)Target).Text = "";
		}
	}

	public static void Crop(Control Target)
	{
		if (Target is TextBox)
		{
			((TextBox)Target).SelectAll();
		}
	}

	public static int GET_WORK_NUMBER(string cin_no)
	{
		int num = 0;
		Random random = new Random();
		num = random.Next(100000, 999999);
		connect("update HT_CheckIn_H set Cin_Work_number=" + Conversions.ToString(num) + " where Cin_No='" + cin_no + "'");
		return num;
	}

	public static string Address_Rcplace(string SS)
	{
		SS = SS.Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ถนน", "ถนน").Replace("ถนน แขวง/ตำบล", "แขวง/ตำบล")
			.Replace("แขวง/ตำบล เขต/อำเภอ", "เขต/อำเภอ")
			.Replace("เขต/อำเภอ จ\u0e31งหว\u0e31ด", "จ\u0e31งหว\u0e31ด")
			.Replace("จ\u0e31งหว\u0e31ด ", "");
		bool flag = false;
		if (SS.IndexOf("กทม") != -1)
		{
			SS = SS.Replace("แขวง/ตำบล", "แขวง");
			SS = SS.Replace("เขต/อำเภอ", "เขต");
			SS = SS.Replace("จ\u0e31งหว\u0e31ด", "");
			flag = true;
		}
		if (SS.IndexOf("กร\u0e38งเทพ") != -1)
		{
			SS = SS.Replace("แขวง/ตำบล", "แขวง");
			SS = SS.Replace("เขต/อำเภอ", "เขต");
			SS = SS.Replace("จ\u0e31งหว\u0e31ด", "");
			flag = true;
		}
		if (!flag)
		{
			SS = SS.Replace("แขวง/ตำบล", "ตำบล");
			SS = SS.Replace("เขต/อำเภอ", "อำเภอ");
		}
		return SS;
	}

	public static void CopyListViewToClipboard(ListView lv)
	{
		if (lv.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อก รายการในตารางก\u0e48อน");
			return;
		}
		StringBuilder stringBuilder = new StringBuilder();
		checked
		{
			int num = lv.Columns.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				stringBuilder.Append(lv.Columns[num2].Text);
				stringBuilder.Append("\t");
				num2++;
			}
			stringBuilder.Append("\r\n");
			int num5 = lv.SelectedItems.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				int num8 = lv.Columns.Count - 1;
				int num9 = 1;
				while (true)
				{
					int num10 = num9;
					num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					stringBuilder.Append(lv.SelectedItems[num6].SubItems[num9].Text);
					stringBuilder.Append("\t");
					num9++;
				}
				stringBuilder.Append("\r\n");
				num6++;
			}
			MyProject.Computer.Clipboard.SetText(stringBuilder.ToString());
		}
	}

	public static string GetSIR_PAY()
	{
		int num = 1;
		DataSet dataSet = connect("select top 1 pay_no from HT_CheckIn_Pay where  Cin_Pay_Date between '" + Conversions.ToString(DateTime.Now.Month) + "/01/" + Conversions.ToString(DateTime.Now.Year) + " 00:00:00' and '" + Conversions.ToString(DateTime.Now.Month) + "/" + Conversions.ToString(DateTime.DaysInMonth(DateTime.Now.Year, DateTime.Now.Month)) + "/" + Conversions.ToString(DateTime.Now.Year) + " 23:59:59' order by pay_No Desc");
		checked
		{
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				num = 1;
			}
			else
			{
				num = Conversions.ToInteger(dataSet.Tables[0].Rows[0]["pay_no"].ToString().Substring(dataSet.Tables[0].Rows[0]["pay_no"].ToString().LastIndexOf("-") + 1));
				num++;
			}
			return Strings.Format(DateTime.Now, "RyyMM") + "-" + Strings.Format(num, "0000");
		}
	}

	public static void Insert_Pay(string no, string ds, DateTime dd, decimal cash, decimal credit, string proname, decimal proname_total, string prounit, string payno, string cid, string ds_id, decimal ds_num, decimal ds_total, decimal ds_total1, string note, decimal ds_free, decimal ds_tran, decimal ds_web)
	{
		while (true)
		{
			object left = "INSERT INTO [HT_CheckIn_Pay]";
			left = Operators.ConcatenateObject(left, "( ");
			left = Operators.ConcatenateObject(left, " [Cin_No]");
			left = Operators.ConcatenateObject(left, ",[Cin_Pay_Ds]");
			left = Operators.ConcatenateObject(left, ",[Cin_Pay_Cash]");
			left = Operators.ConcatenateObject(left, ",[Cin_Pay_Credit]");
			left = Operators.ConcatenateObject(left, ",[Cin_Pay_Date]");
			left = Operators.ConcatenateObject(left, ",[Cin_Pay_Ds_Name]");
			left = Operators.ConcatenateObject(left, ",[Cin_Pay_Ds_Price],[Cin_Pay_Ds_unit],[Pay_No],[Cin_Cust_no],[Cin_Pay_Ds_ID],[Cin_Pay_Ds_Num],[Cin_Pay_Ds_PriceTotal],[Cin_Pay_Ds_PriceOne],[Cin_Pay_Note],[Pay_by],[Cin_Pay_Free],[Cin_Pay_Tran],[Branch],[Cin_Pay_web])");
			left = Operators.ConcatenateObject(left, "VALUES");
			left = Operators.ConcatenateObject(left, "(");
			left = Operators.ConcatenateObject(left, string.Concat(" '" + no, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + ds, "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(cash));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(credit));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(dd), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + proname, "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(proname_total));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + prounit, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + payno, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + cid, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + ds_id, "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(ds_num));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(ds_total));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(ds_total1));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + note, "'"));
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", loginName), "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(ds_free));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(ds_tran));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + MyProject.Forms.FormConfirmPay.ComboBox1.Text, "'"));
			left = Operators.ConcatenateObject(left, "," + Conversions.ToString(ds_web));
			left = Operators.ConcatenateObject(left, ")");
			connect(Conversions.ToString(left));
			DataSet dataSet = connect("select * from HT_CheckIn_Pay where Pay_No='" + payno + "' and Cin_Pay_Ds='" + ds + "'");
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show("ไม\u0e48สามารถบ\u0e31นท\u0e36กการจ\u0e48ายเลขท\u0e35\u0e48 " + payno + " ได\u0e49กด OK เพ\u0e34\u0e48มลองใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				continue;
			}
			break;
		}
	}

	public static void SaveCancel(string room_no, string cin_no, string note)
	{
		object left = "INSERT INTO [HT_Rooms_Cancel]";
		left = Operators.ConcatenateObject(left, "([id]");
		left = Operators.ConcatenateObject(left, ",[room_no]");
		left = Operators.ConcatenateObject(left, ",[cin_no]");
		left = Operators.ConcatenateObject(left, ",[cancel_date]");
		left = Operators.ConcatenateObject(left, ",[cancel_by],[cancel_note]");
		left = Operators.ConcatenateObject(left, ")");
		left = Operators.ConcatenateObject(left, "VALUES");
		left = Operators.ConcatenateObject(left, "(");
		left = Operators.ConcatenateObject(left, Module1.get_id("HT_Rooms_Cancel", "id"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + room_no, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + cin_no, "'"));
		left = Operators.ConcatenateObject(left, ",getdate()");
		left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", loginName), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + note, "'"));
		left = Operators.ConcatenateObject(left, ")");
		connect(Conversions.ToString(left));
	}

	public static void UPDATE_MONEY(string CUSTNO, decimal price, string mode, string ds)
	{
		if (decimal.Compare(price, 0m) != 0)
		{
			if (Operators.CompareString(mode, "ADD", TextCompare: false) == 0)
			{
				DataSet dataSet = connect("select cust_price_over from HT_Customers where cust_no='" + CUSTNO + "'");
				connect("update HT_Customers set Cust_Price_Over=Cust_Price_Over+" + Conversions.ToString(price) + " where Cust_no='" + CUSTNO + "'");
				DataSet dataSet2 = connect("select cust_price_over from HT_Customers where cust_no='" + CUSTNO + "'");
				connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(string.Concat(string.Concat(string.Concat(string.Concat("INSERT INTO HT_Log_Debt (log_cus,log_ds,log_date,log_price,log_price_From,log_price_To) VALUES ('" + CUSTNO, "','"), ds), "',getdate(),"), Conversions.ToString(price)), ","), dataSet.Tables[0].Rows[0]["cust_price_over"]), ","), dataSet2.Tables[0].Rows[0]["cust_price_over"]), ")")));
			}
			else
			{
				DataSet dataSet3 = connect("select cust_price_over from HT_Customers where cust_no='" + CUSTNO + "'");
				connect("update HT_Customers set Cust_Price_Over=Cust_Price_Over-" + Conversions.ToString(price) + " where Cust_no='" + CUSTNO + "'");
				DataSet dataSet4 = connect("select cust_price_over from HT_Customers where cust_no='" + CUSTNO + "'");
				connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(string.Concat(string.Concat(string.Concat(string.Concat("INSERT INTO HT_Log_Debt (log_cus,log_ds,log_date,log_price,log_price_From,log_price_To) VALUES ('" + CUSTNO, "','"), ds), "',getdate(),-"), Conversions.ToString(price)), ","), dataSet3.Tables[0].Rows[0]["cust_price_over"]), ","), dataSet4.Tables[0].Rows[0]["cust_price_over"]), ")")));
			}
		}
	}

	public static void connectmssql2()
	{
		if (conn2 != null)
		{
			conn2.Close();
		}
		try
		{
			connstr2 = string.Format("data source={0};initial catalog={1};UID={2};PWD={3} ;Connect Timeout=3", MSSQL.MysqlServer, "test", MSSQL.MysqlUsername, MSSQL.MysqlPassword);
			conn2 = new SqlConnection(connstr2);
			conn2.Open();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			try
			{
				conn2.Close();
			}
			catch (Exception projectError2)
			{
				ProjectData.SetProjectError(projectError2);
				ProjectData.ClearProjectError();
			}
			ProjectData.ClearProjectError();
		}
	}

	public static DataSet connect2(string command)
	{
		DataSet dataSet = new DataSet();
		dataSet.Tables.Add("0");
		return ExCommand2(command);
	}

	public static DataSet ExCommand2(string Command)
	{
		CodeErr2 = false;
		DataSet dataSet = new DataSet();
		dataSet.Tables.Add("0");
		DataSet result;
		try
		{
			da2 = new SqlDataAdapter(Command, conn2);
			dataSet.Tables.Clear();
			da2.Fill(dataSet);
			MSSQL.DataError = "";
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show(ex2.Message);
			MSSQL.DataError = ex2.Message;
			dataSet.Tables.Clear();
			dataSet.Tables.Add("0");
			result = dataSet;
			ProjectData.ClearProjectError();
			goto IL_0095;
		}
		result = dataSet;
		goto IL_0095;
		IL_0095:
		return result;
	}

	public static int getPaperSize(string sizeName, string PrintName)
	{
		int num = 0;
		PrintDocument printDocument = new PrintDocument();
		printDocument.PrinterSettings.PrinterName = PrintName;
		checked
		{
			int num2 = printDocument.PrinterSettings.PaperSizes.Count - 1;
			num = 0;
			while (true)
			{
				int num3 = num;
				int num4 = num2;
				if (num3 <= num4)
				{
					if (Operators.CompareString(printDocument.PrinterSettings.PaperSizes[num].PaperName, sizeName, TextCompare: false) != 0)
					{
						num++;
						continue;
					}
					return Conversions.ToInteger(printDocument.PrinterSettings.PaperSizes[num].GetType().GetField("kind", BindingFlags.Instance | BindingFlags.NonPublic).GetValue(printDocument.PrinterSettings.PaperSizes[num]));
				}
				break;
			}
			int result = default(int);
			return result;
		}
	}

	public static void saveConfig()
	{
		localdata.Config.WriteXml(PathF + "Config.ini");
	}
}
