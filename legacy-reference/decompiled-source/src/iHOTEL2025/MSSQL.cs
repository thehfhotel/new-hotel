using System;
using System.Data;
using System.Data.SqlClient;
using System.IO;
using System.Windows.Forms;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[StandardModule]
internal sealed class MSSQL
{
	public static bool SHOW_CONNECT_SQL;

	public static string MysqlDatabase;

	public static string MysqlServer;

	public static string MysqlUsername;

	public static string MysqlPassword;

	public static string DataError;

	public static bool IS_ERROR;

	public static SqlConnection conn;

	public static SqlDataAdapter da;

	public static string connstr;

	public static bool CodeErr;

	static MSSQL()
	{
		Class2.LH6iGfYz9j3MJ();
		SHOW_CONNECT_SQL = false;
		MysqlDatabase = "ANLOTTO";
		MysqlServer = "Attosoft-PC";
		MysqlUsername = "sa";
		MysqlPassword = "1234";
		DataError = "";
		IS_ERROR = false;
		CodeErr = false;
	}

	public static void connectmssql()
	{
		if (conn != null)
		{
			conn.Close();
		}
		try
		{
			connstr = $"data source={MysqlServer};initial catalog='{MysqlDatabase}';UID='{MysqlUsername}';PWD='{MysqlPassword}' ;Connect Timeout=15 ";
			conn = new SqlConnection(connstr);
			conn.Open();
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			Module1.MYSQL_CONNECT_ERROR = ex2.Message;
			try
			{
				conn.Close();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			ProjectData.ClearProjectError();
		}
	}

	public static void Create_MssqlDatabase(string pathcreate)
	{
		conn.Close();
		pathcreate = ((Operators.CompareString(pathcreate, "", TextCompare: false) != 0) ? (pathcreate + "\\Database") : (Module1.Path_Program + "Database"));
		if (!Directory.Exists(pathcreate))
		{
			Directory.CreateDirectory(pathcreate);
		}
		string text = "CREATE DATABASE [" + MysqlDatabase + "] ON  PRIMARY ";
		text = text + "( NAME = N'" + MysqlDatabase + "', FILENAME = N'" + pathcreate + "\\" + MysqlDatabase + ".mdf' , SIZE = 5120KB , MAXSIZE = UNLIMITED, FILEGROWTH = 2048KB )";
		text += "LOG ON ";
		text = text + "( NAME = N'" + MysqlDatabase + "_log', FILENAME = N'" + pathcreate + "\\" + MysqlDatabase + "_log.LDF' , SIZE = 5120KB , MAXSIZE = 2048GB , FILEGROWTH = 10%)";
		text += " COLLATE Thai_CI_AS";
		SqlConnection sqlConnection = new SqlConnection("Server=" + MysqlServer + ";uid=" + MysqlUsername + ";pwd=" + MysqlPassword + ";database=master");
		SqlCommand sqlCommand = new SqlCommand(text, sqlConnection);
		bool flag = false;
		try
		{
			sqlConnection.Open();
			sqlCommand.ExecuteNonQuery();
			sqlConnection.Close();
			flag = true;
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show(ex2.ToString());
			ProjectData.ClearProjectError();
		}
		finally
		{
			if (sqlConnection.State == ConnectionState.Open)
			{
				sqlConnection.Close();
			}
		}
		if (flag)
		{
			createtableMSSQL();
		}
	}

	public static void caeateTable(string str)
	{
		SqlConnection sqlConnection = new SqlConnection("Server=" + MysqlServer + ";uid=" + MysqlUsername + ";pwd=" + MysqlPassword + ";database=" + MysqlDatabase);
		SqlCommand sqlCommand = new SqlCommand(str, sqlConnection);
		sqlConnection.Open();
		sqlCommand.ExecuteNonQuery();
		sqlConnection.Close();
	}

	public static void createtableMSSQL()
	{
		string text = "";
		text = "CREATE TABLE [BookSettingsHEAD]([id] [int] NOT NULL DEFAULT ((0)), [LT_CODE] [varchar](250) NULL, [LT_NAME] [varchar](250) NULL, [LT_ADDRESS] [varchar](250) NULL, [LT_TEL] [varchar](250) NULL) ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_Cust](";
		text += "\t[Cust_id] [varchar](250) NOT NULL,";
		text += "\t[Cust_Name] [varchar](250) NULL,";
		text += "\t[Cust_Tel] [varchar](250) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_KEEPS](";
		text += "\t[K_id] [float] NULL,";
		text += "\t[K_Mode] [varchar](20) NULL,";
		text += "\t[K_NUM] [varchar](20) NULL,";
		text += "\t[K_PRICE] [varchar](20) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_SEND](";
		text += "\t[id] [int] IDENTITY(1,1) NOT NULL,";
		text += "\t[N1] [varchar](10) NULL,";
		text += "\t[Price1] [varchar](15) NULL,";
		text += "\t[Mode] [varchar](2) NULL,";
		text += "\t[Price2] [varchar](15) NULL,";
		text += "\t[Chanel] [varchar](5) NULL,";
		text += "\t[computer_name] [varchar](50) NULL,";
		text += "\t[time] [varchar](50) NULL,";
		text += "\t[cust] [varchar](20) NULL,";
		text += "\t[page] [varchar](5) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [User_level](";
		text += "\t[id] [int] NOT NULL  DEFAULT ((0)),";
		text += "\t[name] [varchar](50) NULL,";
		text += "\t[m_main] [varchar](50) NULL,";
		text += "\t[m_cus] [varchar](50) NULL,";
		text += "\t[m_set] [varchar](50) NULL,";
		text += "\t[m_update] [varchar](50) NULL,";
		text += "\t[m_user] [varchar](50) NULL,";
		text += "\t[m_dbcp] [varchar](50) NULL,";
		text += "\t[m_dbbk] [varchar](50) NULL,";
		text += "\t[m_dbre] [varchar](50) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_Ds](";
		text += "\t[id] [int] IDENTITY(1,1) NOT NULL,";
		text += "\t[id_h] [int] NULL DEFAULT ((0)),";
		text += "\t[ตรง] [varchar](50) NULL,";
		text += "\t[ว\u0e34\u0e48ง] [varchar](50) NULL,";
		text += "\t[สองต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[สามต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[สองต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[สามต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[โต\u0e4aด] [varchar](50) NULL,";
		text += "\t[ว\u0e34\u0e48งบน] [varchar](50) NULL,";
		text += "\t[ว\u0e34\u0e48งล\u0e48าง] [varchar](50) NULL,";
		text += "\t[สองโต\u0e4aด] [varchar](50) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_H](";
		text += "\t[id] [int] NOT NULL DEFAULT ((0)),";
		text += "\t[cust_branch] [int] NULL DEFAULT ((0)),";
		text += "\t[cust_id] [int] NULL DEFAULT ((0)),";
		text += "\t[cust_name] [varchar](50) NULL,";
		text += "\t[รวมตรง] [varchar](50) NULL,";
		text += "\t[รวมว\u0e34\u0e48ง] [varchar](50) NULL,";
		text += "\t[รวม2ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[รวม3ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[รวม2ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[รวม3ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[รวมโต\u0e4aด] [varchar](50) NULL,";
		text += "\t[รวมว\u0e34\u0e48งบน] [varchar](50) NULL,";
		text += "\t[รวมว\u0e34\u0e48งล\u0e48าง] [varchar](50) NULL,";
		text += "\t[Pตรง] [varchar](50) NULL,";
		text += "\t[Pว\u0e34\u0e48ง] [varchar](50) NULL,";
		text += "\t[P2ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[P3ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[P2ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[P3ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[Pโต\u0e4aด] [varchar](50) NULL,";
		text += "\t[Pว\u0e34\u0e48งบน] [varchar](50) NULL,";
		text += "\t[Pว\u0e34\u0e48งล\u0e48าง] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อตรง] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อว\u0e34\u0e48ง] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อ2ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อ3ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อ2ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อ3ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อโต\u0e4aด] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อว\u0e34\u0e48งบน] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อว\u0e34\u0e48งล\u0e48าง] [varchar](50) NULL,";
		text += "\t[ยอดซ\u0e37\u0e49อ] [varchar](50) NULL,";
		text += "\t[ยอดถ\u0e39ก] [varchar](50) NULL,";
		text += "\t[เก\u0e47บจ\u0e48ายช\u0e37\u0e48อ] [varchar](50) NULL,";
		text += "\t[เก\u0e47บจ\u0e48ายราคา] [varchar](50) NULL,";
		text += "\t[จ\u0e48ายก\u0e48อน] [varchar](50) NULL,";
		text += "\t[จ\u0e48ายพ\u0e34เศษ] [varchar](50) NULL,";
		text += "\t[ยอดตก] [varchar](50) NULL,";
		text += "\t[ค\u0e49างเก\u0e48า] [varchar](50) NULL,";
		text += "\t[รวมยอด] [varchar](50) NULL,";
		text += "\t[ชำระ] [varchar](50) NULL,";
		text += "\t[ผ\u0e39\u0e49ร\u0e31บเง\u0e34น] [varchar](50) NULL,";
		text += "\t[ส\u0e39ญเส\u0e35ย] [varchar](50) NULL,";
		text += "\t[คงเหล\u0e37อ] [varchar](50) NULL,";
		text += "\t[จำนวนแผ\u0e48น] [int] NULL DEFAULT ((1)),";
		text += "\t[รวม2โต\u0e4aด] [varchar](50) NULL,";
		text += "\t[P2โต\u0e4aด] [varchar](50) NULL,";
		text += "\t[เหล\u0e37อ2โต\u0e4aด] [varchar](50) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [BookSettings](";
		text += "\t[id] [int] NOT NULL DEFAULT ((0)),";
		text += "\t[store_name] [varchar](250) NULL,";
		text += "\t[store_name2] [varchar](250) NULL,";
		text += "\t[store_address] [varchar](250) NULL,";
		text += "\t[store_tel] [varchar](250) NULL,";
		text += "\t[store_tel2] [varchar](250) NULL,";
		text += "\t[store_fax] [varchar](250) NULL,";
		text += "\t[store_setting_len] [varchar](250) NULL,";
		text += "\t[store_setting_print] [varchar](250) NULL,";
		text += "\t[store_setting_decimal] [varchar](250) NULL,";
		text += "\t[store_camera] [varchar](250) NULL,";
		text += "\t[store_footer] [varchar](250) NULL,";
		text += "\t[store_all_per] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[store_ok_per] [decimal](18, 2) NULL  DEFAULT ((0)),";
		text += "\t[store_2up] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[store_3up] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[store_2down] [decimal](18, 2) NULL  DEFAULT ((0)),";
		text += "\t[store_3down] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[store_tod] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[store_runup] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[store_rundown] [decimal](18, 2) NULL  DEFAULT ((0)),";
		text += "\t[S_22] [float] NULL   DEFAULT ((0)),";
		text += "\t[store_tod2] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[p_ver] [float] NULL  DEFAULT ((0)),";
		text += "\t[REPORT_TYPE] [varchar](50) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_SETTING2](";
		text += "\t[S_id] [float] NULL,";
		text += "\t[S_Mode] [varchar](20) NULL,";
		text += "\t[S_NUM] [varchar](20) NULL,";
		text += "\t[S_PRICE] [varchar](20) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_Round](";
		text += "\t[ROUND_STATUS] [varchar](20) NULL,";
		text += "\t[ROUND_START] [varchar](30) NULL,";
		text += "\t[ROUND_STOP] [varchar](30) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_CUST2](";
		text += "\t[id] [varchar](20) NULL,";
		text += "\t[NAME] [varchar](255) NULL,";
		text += "\t[TEL] [varchar](30) NULL,";
		text += "\t[USER] [varchar](30) NULL,";
		text += "\t[PASS] [varchar](30) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_Branch](";
		text += "\t[id] [int] NOT NULL   DEFAULT ((0)),";
		text += "\t[Name] [varchar](250) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [BookAdmin](";
		text += "\t[id] [int] NOT NULL   DEFAULT ((0)),";
		text += "\t[Username] [varchar](250) NULL,";
		text += "\t[Password] [varchar](250) NULL,";
		text += "\t[Name] [varchar](250) NULL,";
		text += "\t[level] [varchar](250) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [BookSettings2](";
		text += "\t[id] [varchar](250) NOT NULL,";
		text += "\t[2upPay] [varchar](250) NULL,";
		text += "\t[2upPer] [varchar](250) NULL,";
		text += "\t[2upKeep] [varchar](250) NULL,";
		text += "\t[3upPay] [varchar](250) NULL,";
		text += "\t[3upPer] [varchar](250) NULL,";
		text += "\t[3upKeep] [varchar](250) NULL,";
		text += "\t[2downPay] [varchar](250) NULL,";
		text += "\t[2downPer] [varchar](250) NULL,";
		text += "\t[2downKeep] [varchar](250) NULL,";
		text += "\t[3downPay] [varchar](250) NULL,";
		text += "\t[3downPer] [varchar](250) NULL,";
		text += "\t[3downKeep] [varchar](250) NULL,";
		text += "\t[2todPay] [varchar](250) NULL,";
		text += "\t[2todPer] [varchar](250) NULL,";
		text += "\t[2todKeep] [varchar](250) NULL,";
		text += "\t[3todPay] [varchar](250) NULL,";
		text += "\t[3todPer] [varchar](250) NULL,";
		text += "\t[3todKeep] [varchar](250) NULL,";
		text += "\t[runUpPay] [varchar](250) NULL,";
		text += "\t[runUpPer] [varchar](250) NULL,";
		text += "\t[runUpKeep] [varchar](250) NULL,";
		text += "\t[runDownPay] [varchar](250) NULL,";
		text += "\t[runDownPer] [varchar](250) NULL,";
		text += "\t[runDownKeep] [varchar](250) NULL,";
		text += "\t[Block2UP] [varchar](250) NULL,";
		text += "\t[Block3UP] [varchar](250) NULL,";
		text += "\t[Block2Down] [varchar](250) NULL,";
		text += "\t[Block3Down] [varchar](250) NULL,";
		text += "\t[Block2Tod] [varchar](250) NULL,";
		text += "\t[Block3Tod] [varchar](250) NULL,";
		text += "\t[BlockRunUp] [varchar](250) NULL,";
		text += "\t[BlockRunDown] [varchar](250) NULL,";
		text += "\t[All_TOTAL_TOD] [float] NULL   DEFAULT ((0)),";
		text += "\t[NOT_GROUP] [varchar](50) NULL,";
		text += "\t[USE_SOUND] [varchar](50) NULL,";
		text += "\t[CUT_PRINT] [varchar](50) NULL,";
		text += "\t[2upPay2] [varchar](50) NULL,";
		text += "\t[2upPer2] [varchar](50) NULL,";
		text += "\t[3upPay2] [varchar](50) NULL,";
		text += "\t[3upPer2] [varchar](50) NULL,";
		text += "\t[2downPay2] [varchar](50) NULL,";
		text += "\t[2downPer2] [varchar](50) NULL,";
		text += "\t[3downPay2] [varchar](50) NULL,";
		text += "\t[3downPer2] [varchar](50) NULL,";
		text += "\t[2todPay2] [varchar](50) NULL,";
		text += "\t[2todPer2] [varchar](50) NULL,";
		text += "\t[3todPay2] [varchar](50) NULL,";
		text += "\t[3todPer2] [varchar](50) NULL,";
		text += "\t[runUpPay2] [varchar](50) NULL,";
		text += "\t[runUpPer2] [varchar](50) NULL,";
		text += "\t[runDownPay2] [varchar](50) NULL,";
		text += "\t[runDownPer2] [varchar](50) NULL,";
		text += "\t[Art_Group] [varchar](20) NULL,";
		text += "\t[SET_COLOR] [varchar](20) NULL,";
		text += "\t[SET_REFRESH_TIME] [varchar](20) NULL,";
		text += "\t[SET_SORT] [varchar](20) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_Account](";
		text += "\t[id] [int] NOT NULL   DEFAULT ((0)),";
		text += "\t[Branch] [int] NULL   DEFAULT ((0)),";
		text += "\t[ช\u0e37\u0e48อ] [varchar](250) NULL,";
		text += "\t[ธนาคาร] [varchar](250) NULL,";
		text += "\t[สาขา] [varchar](250) NULL,";
		text += "\t[จ\u0e31งหว\u0e31ด] [varchar](250) NULL,";
		text += "\t[ประเภท] [varchar](250) NULL,";
		text += "\t[เลขท\u0e35\u0e48] [varchar](250) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_CUSTOMERS](";
		text += "\t[id] [int] NOT NULL   DEFAULT ((0)),";
		text += "\t[c_id] [int] NULL   DEFAULT ((0)),";
		text += "\t[c_branch] [int] NULL   DEFAULT ((0)),";
		text += "\t[c_name] [varchar](250) NULL,";
		text += "\t[c_house] [varchar](250) NULL,";
		text += "\t[c_tambon] [varchar](250) NULL,";
		text += "\t[c_amper] [varchar](250) NULL,";
		text += "\t[c_province] [varchar](250) NULL,";
		text += "\t[c_tel] [varchar](250) NULL,";
		text += "\t[c_bank] [varchar](250) NULL,";
		text += "\t[c_bank_branch] [varchar](250) NULL,";
		text += "\t[c_bank_province] [varchar](250) NULL,";
		text += "\t[c_bank_name] [varchar](250) NULL,";
		text += "\t[c_bank_type] [varchar](250) NULL,";
		text += "\t[c_bank_no] [varchar](250) NULL,";
		text += "\t[c_pay_sp] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_pay_debt] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_all_per] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_ok_per] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_2up] [decimal](18, 2) NULL    DEFAULT ((0)),";
		text += "\t[c_3up] [decimal](18, 2) NULL    DEFAULT ((0)),";
		text += "\t[c_2down] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_3down] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_tod] [decimal](18, 2) NULL   DEFAULT ((0)),";
		text += "\t[c_runUP] [decimal](18, 2) NULL    DEFAULT ((0)),";
		text += "\t[c_runDown] [decimal](18, 2) NULL    DEFAULT ((0)),";
		text += "\t[c_tod2] [decimal](18, 2) NULL  DEFAULT ((0)),";
		text += "\t[LINK_ID] [varchar](30) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_Win](";
		text += "\t[id] [int] NOT NULL   DEFAULT ((0)),";
		text += "\t[ว\u0e31นท\u0e35\u0e48] [datetime] NULL,";
		text += "\t[3ต\u0e31วบน] [varchar](50) NULL,";
		text += "\t[2ต\u0e31วล\u0e48าง] [varchar](50) NULL,";
		text += "\t[3ต\u0e31วล\u0e48าง1] [varchar](50) NULL,";
		text += "\t[3ต\u0e31วล\u0e48าง2] [varchar](50) NULL,";
		text += "\t[3ต\u0e31วล\u0e48าง3] [varchar](50) NULL,";
		text += "\t[3ต\u0e31วล\u0e48าง4] [varchar](50) NULL,";
		text += "\t[Branch] [int] NULL   DEFAULT ((0)),";
		text += "\t[USE_34] [varchar](20) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE TABLE [LT_F6](";
		text += "\t[id] [int] NOT NULL   DEFAULT ((0)),";
		text += "\t[t_branch] [int] NULL   DEFAULT ((0)),";
		text += "\t[t_ลำด\u0e31บ] [varchar](50) NULL,";
		text += "\t[t_ช\u0e37\u0e48อ] [varchar](50) NULL,";
		text += "\t[t_บ\u0e49าน] [varchar](50) NULL,";
		text += "\t[t_จำนวน] [varchar](50) NULL,";
		text += "\t[t_ยอดซ\u0e37\u0e49อ] [varchar](50) NULL,";
		text += "\t[t_ยอดถ\u0e39ก] [varchar](50) NULL,";
		text += "\t[t_เก\u0e47บ] [varchar](50) NULL,";
		text += "\t[t_จ\u0e48าย] [varchar](50) NULL,";
		text += "\t[t_เหล\u0e37อ_text] [varchar](50) NULL,";
		text += "\t[t_เหล\u0e37อ] [varchar](50) NULL";
		text += ") ON [PRIMARY]";
		caeateTable(text);
		text = "CREATE VIEW [View_LT_H]";
		text += "AS ";
		text += "   SELECT ";
		text += "      [LT_H].*, ";
		text += "      [LT_CUSTOMERS].[c_house], ";
		text += "      [LT_CUSTOMERS].[c_tambon], ";
		text += "      [LT_CUSTOMERS].[c_amper], ";
		text += "      [LT_CUSTOMERS].[c_province], ";
		text += "      [LT_CUSTOMERS].[c_tel]";
		text += "   FROM ";
		text += "      [LT_H] ";
		text += "         INNER JOIN [LT_CUSTOMERS] ";
		text += "         ON ([LT_H].[cust_branch] = [LT_CUSTOMERS].[c_branch]) AND ([LT_H].[cust_id] = [LT_CUSTOMERS].[c_id])";
		caeateTable(text);
		text = "CREATE VIEW [view_users]";
		text += "AS ";
		text += "   /*Generated by SQL Server Migration Assistant for Access version 5.2.1257.*/";
		text += "   SELECT ";
		text += "      [BookAdmin].[id], ";
		text += "      [BookAdmin].[Username], ";
		text += "      [BookAdmin].[Password], ";
		text += "      [BookAdmin].[Name], ";
		text += "      [BookAdmin].[level], ";
		text += "      [User_level].[m_main], ";
		text += "      [User_level].[m_cus], ";
		text += "      [User_level].[m_set], ";
		text += "      [User_level].[m_update], ";
		text += "      [User_level].[m_user], ";
		text += "      [User_level].[m_dbcp], ";
		text += "      [User_level].[m_dbbk], ";
		text += "      [User_level].[m_dbre]";
		text += "   FROM ";
		text += "      [BookAdmin] ";
		text += "         INNER JOIN [User_level] ";
		text += "         ON [BookAdmin].[level] = [User_level].[name]";
		caeateTable(text);
		caeateTable("INSERT INTO BOOKADMIN (id, Username, Password, Name, level) VALUES(1, 'Admin', 'Admin', 'Admin', 'Admin')");
		caeateTable("INSERT INTO BOOKADMIN (id, Username, Password, Name, level) VALUES(2, '1', '1', '1', 'พน\u0e31กงาน')");
		caeateTable("INSERT INTO BookSettings (id, store_name, store_name2, store_address, store_tel, store_tel2, store_fax, store_setting_len, store_setting_print, store_setting_decimal, store_camera, store_footer, store_all_per, store_ok_per, store_2up, store_3up, store_2down, store_3down, store_tod, store_runup, store_rundown, S_22, store_tod2, p_ver, REPORT_TYPE) VALUES (0, 'Apple Book', 'Apple Book', 'ท\u0e35\u0e48อย\u0e39\u0e48 ', '0803044131', '0858626345', '0802638528', 'before', 'no', 'up', '358 x 590', 'Thank You!!', 25.0000, 25.0000, 65.0000, 550.0000, 65.0000, 10.0000, 100.0000, 3.0000, 4.0000, NULL, 10.0000, 17, '0')");
		caeateTable("INSERT INTO BOOKSETTINGS2 (id, [2upPay], [2upPer], [2upKeep], [3upPay], [3upPer], [3upKeep], [2downPay], [2downPer], [2downKeep], [3downPay], [3downPer], [3downKeep], [2todPay], [2todPer], [2todKeep], [3todPay], [3todPer], [3todKeep], [runUpPay], [runUpPer], [runUpKeep], runDownPay, runDownPer, runDownKeep, Block2UP, Block3UP, Block2Down, Block3Down, Block2Tod, Block3Tod, BlockRunUp, BlockRunDown, All_TOTAL_TOD, NOT_GROUP, USE_SOUND, CUT_PRINT, [2upPay2], [2upPer2], [3upPay2], [3upPer2], [2downPay2], [2downPer2], [3downPay2], [3downPer2], [2todPay2], [2todPer2], [3todPay2], [3todPer2], [runUpPay2], [runUpPer2], [runDownPay2], [runDownPer2], [Art_Group], [SET_COLOR], [SET_REFRESH_TIME], [SET_SORT]) VALUES('1', '70', '30', '800', '550', '35', '900', '70', '25', '1000', '100', '25', '1000', '10', '25', '1000', '100', '25', '1000', '3', '12', '2000', '4', '12', '2000', '99999', '99999', '99999', '99999', '99999', '99999', '99999', '99999', 1, 'False', 'True', 'False', '70', '25', '550', '30', '70', '20', '100', '20', '10', '20', '100', '20', '3', '12', '4', '12', '2', 'False', '30', 'False') ");
		caeateTable("INSERT INTO BOOKSETTINGSHEAD (id, LT_CODE, LT_NAME, LT_ADDRESS, LT_TEL) VALUES(0, '', 'เปล\u0e35\u0e48ยนช\u0e37\u0e48อได\u0e49ท\u0e35\u0e48หน\u0e49า ต\u0e31\u0e49งค\u0e48าโปรแกรม', '', '') ");
		caeateTable("INSERT INTO LT_BRANCH (id, Name) VALUES(1, 'กล\u0e38\u0e48มท\u0e35\u0e48 1') ");
		caeateTable("INSERT INTO LT_BRANCH (id, Name) VALUES(2, 'กล\u0e38\u0e48มท\u0e35\u0e48 2') ");
		caeateTable("INSERT INTO LT_CUST (Cust_id, Cust_Name, Cust_Tel) VALUES('001', 'ทดสอบ1', '') ");
		caeateTable("INSERT INTO LT_CUST (Cust_id, Cust_Name, Cust_Tel) VALUES('002', 'ทดสอบ2', '') ");
		caeateTable("INSERT INTO LT_ROUND (ROUND_STATUS, ROUND_START, ROUND_STOP) VALUES('ป\u0e34ด', '42050', '42050')");
		caeateTable("INSERT INTO LT_WIN (id, [ว\u0e31นท\u0e35\u0e48], [3ต\u0e31วบน], [2ต\u0e31วล\u0e48าง], [3ต\u0e31วล\u0e48าง1], [3ต\u0e31วล\u0e48าง2], [3ต\u0e31วล\u0e48าง3], [3ต\u0e31วล\u0e48าง4], Branch, USE_34) VALUES(1, '1/1/2019 00:00:00', '123', '45', '789', '999', '888', '777', 1, 'True')");
		caeateTable("INSERT INTO USER_LEVEL (id, name, m_main, m_cus, m_set, m_update, m_user, m_dbcp, m_dbbk, m_dbre) VALUES(1, 'Admin', 'True', 'True', 'True', 'True', 'True', 'True', 'True', 'True')");
		caeateTable("INSERT INTO USER_LEVEL (id, name, m_main, m_cus, m_set, m_update, m_user, m_dbcp, m_dbbk, m_dbre) VALUES(2, 'พน\u0e31กงาน', 'True', 'False', 'True', 'True', 'True', 'True', 'True', 'False')");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(1, 1, '', 'กส\u0e34กรไทย', 'มน', 'พ\u0e34ษณ\u0e38โลก', 'ออมทร\u0e31พย\u0e4c', '12837492') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(2, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(3, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(4, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(5, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(6, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(7, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(8, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(9, 1, '', '', '', '', '', '') ");
		caeateTable("INSERT INTO LT_ACCOUNT (id, Branch, ช\u0e37\u0e48อ, ธนาคาร, สาขา, จ\u0e31งหว\u0e31ด, ประเภท, เลขท\u0e35\u0e48) VALUES(10, 1, '', '', '', '', '', '') ");
		MessageBox.Show("สร\u0e49างฐานข\u0e49อม\u0e39ล " + MysqlDatabase + " เร\u0e35ยบร\u0e49อย กด OK เพ\u0e37\u0e48อดำเน\u0e34นการต\u0e48อ", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
	}

	public static DataSet smethod_0(string command)
	{
		DataSet dataSet = new DataSet();
		dataSet.Tables.Add("0");
		dataSet = ExCommand(command);
		while (Operators.CompareString(DataError, "", TextCompare: false) != 0 && !Module1.CloseProgram)
		{
			if (conn.State == ConnectionState.Closed)
			{
				if (SHOW_CONNECT_SQL)
				{
					MyProject.Forms.frmMain1.Timer1.Enabled = false;
					MyProject.Forms.connect_mssql.ShowDialog();
					MyProject.Forms.frmMain1.Timer1.Enabled = true;
				}
				if (Module1.CloseProgram)
				{
					break;
				}
				if (SHOW_CONNECT_SQL)
				{
					if (MessageBox.Show("พบข\u0e49อผ\u0e34ดพลาดของฐานข\u0e49อม\u0e39ล : " + DataError + Environment.NewLine + Environment.NewLine + "กด Retry เพ\u0e37\u0e48อทำข\u0e49อม\u0e39ลซ\u0e49ำอ\u0e35กคร\u0e31\u0e49ง (แนะนำ)" + Environment.NewLine + "กด Cancel เพ\u0e37\u0e48อยกเล\u0e34กข\u0e49อม\u0e39ลน\u0e35\u0e49 (อาจจะม\u0e35ป\u0e31ญหาเก\u0e35\u0e48ยวก\u0e31บข\u0e49อม\u0e39ลช\u0e38ดน\u0e35\u0e49 กร\u0e38ณาป\u0e34ดเป\u0e34ดเข\u0e49ามาด\u0e39ใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง)", "ข\u0e49อผ\u0e34ดพลาด", MessageBoxButtons.RetryCancel, MessageBoxIcon.Hand, MessageBoxDefaultButton.Button1) == DialogResult.Retry)
					{
						IS_ERROR = true;
						dataSet = ExCommand(command);
						continue;
					}
					IS_ERROR = true;
					dataSet.Tables.Clear();
					dataSet.Tables.Add("0");
					DataError = "";
				}
				else
				{
					IS_ERROR = true;
					dataSet.Tables.Clear();
					dataSet.Tables.Add("0");
					DataError = "";
				}
				continue;
			}
			CodeErr = true;
			dataSet.Tables.Clear();
			dataSet.Tables.Add("0");
			DataError = "";
			break;
		}
		return dataSet;
	}

	public static DataSet ExCommand(string Command)
	{
		CodeErr = false;
		DataSet dataSet = new DataSet();
		dataSet.Tables.Add("0");
		DataSet result;
		try
		{
			da = new SqlDataAdapter(Command, conn);
			da.SelectCommand.CommandTimeout = 120;
			dataSet.Tables.Clear();
			da.Fill(dataSet);
			DataError = "";
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show(ex2.Message);
			DataError = ex2.Message;
			dataSet.Tables.Clear();
			dataSet.Tables.Add("0");
			result = dataSet;
			ProjectData.ClearProjectError();
			goto IL_00a6;
		}
		result = dataSet;
		goto IL_00a6;
		IL_00a6:
		return result;
	}

	public static DataSet connectaa(string command)
	{
		DataSet dataSet = new DataSet();
		dataSet.Tables.Add("0");
		DataSet result;
		try
		{
			da = new SqlDataAdapter(command, conn);
			dataSet.Tables.Clear();
			da.Fill(dataSet);
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show("ข\u0e49อผ\u0e34ดพลาดของฐานข\u0e49อม\u0e39ล = " + ex2.Message);
			dataSet.Tables.Clear();
			dataSet.Tables.Add("0");
			result = dataSet;
			ProjectData.ClearProjectError();
			goto IL_0084;
		}
		result = dataSet;
		goto IL_0084;
		IL_0084:
		return result;
	}
}
