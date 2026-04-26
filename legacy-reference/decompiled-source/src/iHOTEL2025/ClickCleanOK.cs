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
public class ClickCleanOK : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("RichTextBox1")]
	private RichTextBox _RichTextBox1;

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("ButtonX15")]
	private ButtonX _ButtonX15;

	[AccessedThroughProperty("ButtonX14")]
	private ButtonX _ButtonX14;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	public string RoomNo;

	public ArrayList RoomArr;

	public bool ISOK;

	internal virtual ButtonX ButtonX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX1_Click;
			if (_ButtonX1 != null)
			{
				_ButtonX1.Click -= value2;
			}
			_ButtonX1 = value;
			if (_ButtonX1 != null)
			{
				_ButtonX1.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX3_Click;
			if (_ButtonX3 != null)
			{
				_ButtonX3.Click -= value2;
			}
			_ButtonX3 = value;
			if (_ButtonX3 != null)
			{
				_ButtonX3.Click += value2;
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

	internal virtual RichTextBox RichTextBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _RichTextBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RichTextBox1 = value;
		}
	}

	internal virtual ButtonX ButtonX6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX6_Click;
			if (_ButtonX6 != null)
			{
				_ButtonX6.Click -= value2;
			}
			_ButtonX6 = value;
			if (_ButtonX6 != null)
			{
				_ButtonX6.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX15_Click;
			if (_ButtonX15 != null)
			{
				_ButtonX15.Click -= value2;
			}
			_ButtonX15 = value;
			if (_ButtonX15 != null)
			{
				_ButtonX15.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX14_Click;
			if (_ButtonX14 != null)
			{
				_ButtonX14.Click -= value2;
			}
			_ButtonX14 = value;
			if (_ButtonX14 != null)
			{
				_ButtonX14.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX2_Click_1;
			if (_ButtonX2 != null)
			{
				_ButtonX2.Click -= value2;
			}
			_ButtonX2 = value;
			if (_ButtonX2 != null)
			{
				_ButtonX2.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static ClickCleanOK()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickCleanOK()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += ClickBook_FormClosing;
		base.Load += ClickBook_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		RoomNo = "";
		RoomArr = new ArrayList();
		ISOK = false;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickCleanOK));
		this.Label1 = new System.Windows.Forms.Label();
		this.RichTextBox1 = new System.Windows.Forms.RichTextBox();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(12, 10);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(105, 23);
		label2.Size = size;
		this.Label1.TabIndex = 3;
		this.Label1.Text = "หมายเหต\u0e38 :";
		this.RichTextBox1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.RichTextBox1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.RichTextBox richTextBox = this.RichTextBox1;
		location = new System.Drawing.Point(12, 36);
		richTextBox.Location = location;
		this.RichTextBox1.Name = "RichTextBox1";
		System.Windows.Forms.RichTextBox richTextBox2 = this.RichTextBox1;
		size = new System.Drawing.Size(329, 167);
		richTextBox2.Size = size;
		this.RichTextBox1.TabIndex = 4;
		this.RichTextBox1.Text = "";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(11, 325);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(161, 52);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 16;
		this.ButtonX2.Text = "  เร\u0e34\u0e48ม\r\n  ทำความสะอาด";
		this.ButtonX2.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX2.Visible = false;
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ButtonX15.Image");
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_0;
		location = new System.Drawing.Point(178, 267);
		buttonX_.Location = location;
		this.ButtonX_0.Name = "ButtonX15";
		DevComponents.DotNetBar.ButtonX buttonX_2 = this.ButtonX_0;
		size = new System.Drawing.Size(162, 52);
		buttonX_2.Size = size;
		this.ButtonX_0.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_0.TabIndex = 15;
		this.ButtonX_0.Text = "ป\u0e34ดไฟ";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_1.FocusCuesEnabled = false;
		this.ButtonX_1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_1.Image = (System.Drawing.Image)resources.GetObject("ButtonX14.Image");
		DevComponents.DotNetBar.ButtonX buttonX_3 = this.ButtonX_1;
		location = new System.Drawing.Point(11, 267);
		buttonX_3.Location = location;
		this.ButtonX_1.Name = "ButtonX14";
		DevComponents.DotNetBar.ButtonX buttonX_4 = this.ButtonX_1;
		size = new System.Drawing.Size(161, 52);
		buttonX_4.Size = size;
		this.ButtonX_1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_1.TabIndex = 14;
		this.ButtonX_1.Text = "เป\u0e34ดไฟ";
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX6.FocusCuesEnabled = false;
		this.ButtonX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX6.Image = (System.Drawing.Image)resources.GetObject("ButtonX6.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX6;
		location = new System.Drawing.Point(178, 209);
		buttonX3.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX6;
		size = new System.Drawing.Size(162, 52);
		buttonX4.Size = size;
		this.ButtonX6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX6.TabIndex = 5;
		this.ButtonX6.Text = "ซ\u0e48อมบำร\u0e38ง";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(11, 209);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(161, 52);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "ทำความสะอาดเร\u0e35ยบร\u0e49อย";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX1;
		location = new System.Drawing.Point(95, 325);
		buttonX7.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX1;
		size = new System.Drawing.Size(162, 52);
		buttonX8.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(354, 389);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX_0);
		this.Controls.Add(this.ButtonX_1);
		this.Controls.Add(this.ButtonX6);
		this.Controls.Add(this.RichTextBox1);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickCleanOK";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ClickBook";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void ClickBook_FormClosing(object sender, FormClosingEventArgs e)
	{
		MSSQL.CodeErr = false;
		RoomNo = "";
	}

	private void ClickBook_Load(object sender, EventArgs e)
	{
		MSSQL.CodeErr = true;
		RichTextBox1.Text = "";
		Text = "รายการห\u0e49อง " + RoomNo;
		checked
		{
			int num = RoomArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (num2 == 0)
				{
					Text = Conversions.ToString(Operators.ConcatenateObject("รายการห\u0e49อง ", RoomArr[num2]));
				}
				else
				{
					Text = Conversions.ToString(Operators.ConcatenateObject(Text, Operators.ConcatenateObject(", ", RoomArr[num2])));
				}
				num2++;
			}
			check_power();
			ISOK = false;
			if (!Module1.MANUAL_POWER)
			{
				ButtonX_1.Enabled = false;
				ButtonX_0.Enabled = false;
			}
		}
	}

	public void check_power()
	{
		if (!Module1.POWER_USED)
		{
			ButtonX_1.Enabled = false;
			ButtonX_0.Enabled = false;
		}
		else if (RoomArr.Count >= 1)
		{
			ButtonX_1.Text = "เป\u0e34ดไฟ ห\u0e49องท\u0e35\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
			ButtonX_1.Enabled = true;
			ButtonX_0.Text = "ป\u0e34ดไฟ ห\u0e49องท\u0e35\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
			ButtonX_0.Enabled = true;
		}
		else if (Operators.CompareString(Module1.Power_Status(RoomNo), "OFF", TextCompare: false) == 0)
		{
			ButtonX_1.Text = "เป\u0e34ดไฟ " + RoomNo;
			ButtonX_1.Enabled = true;
			ButtonX_0.Text = "ป\u0e34ดไฟ " + RoomNo;
			ButtonX_0.Enabled = false;
		}
		else
		{
			ButtonX_1.Text = "เป\u0e34ดไฟ " + RoomNo;
			ButtonX_1.Enabled = false;
			ButtonX_0.Text = "ป\u0e34ดไฟ " + RoomNo;
			ButtonX_0.Enabled = true;
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		checked
		{
			if (!Module1.HouseWifeMode)
			{
				if (RoomArr.Count == 0)
				{
					DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
					string right = "";
					string right2 = "";
					DataSet dataSet2 = Module1.connect("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='" + RoomNo + "' order by cin_room_out desc");
					if (dataSet2.Tables[0].Rows.Count != 0)
					{
						right = Conversions.ToString(dataSet2.Tables[0].Rows[0]["cin_no"]);
						right2 = Conversions.ToString(dataSet2.Tables[0].Rows[0]["cin_cust_name"]);
					}
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=", dataSet.Tables[0].Rows[0]["id"])));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", Module1.loginName), "', '"), RoomNo), "', '"), DateTime.Now), "', '"), RichTextBox1.Text), "','"), right), "','"), right2), "')")));
					Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟทำความสะอาดเร\u0e35ยบร\u0e49อย");
				}
				else
				{
					int num = RoomArr.Count - 1;
					int num2 = 0;
					while (true)
					{
						int num3 = num2;
						int num4 = num;
						if (num3 > num4)
						{
							break;
						}
						DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", RoomArr[num2]), "'")));
						string right3 = "";
						string right4 = "";
						DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='", RoomArr[num2]), "' order by cin_room_out desc")));
						if (dataSet4.Tables[0].Rows.Count != 0)
						{
							right3 = Conversions.ToString(dataSet4.Tables[0].Rows[0]["cin_no"]);
							right4 = Conversions.ToString(dataSet4.Tables[0].Rows[0]["cin_cust_name"]);
						}
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=", dataSet3.Tables[0].Rows[0]["id"])));
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", Module1.loginName), "', '"), RoomArr[num2]), "', '"), DateTime.Now), "', '"), RichTextBox1.Text), "','"), right3), "','"), right4), "')")));
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "OFF", "", "ป\u0e34ดไฟทำความสะอาดเร\u0e35ยบร\u0e49อย");
						num2++;
					}
				}
				ISOK = true;
				Close();
				return;
			}
			object obj = Interaction.InputBox("กร\u0e38ณากรอกรห\u0e31สผ\u0e48าน", "รห\u0e31สผ\u0e48าน");
			if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
			{
				return;
			}
			DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from TB_MRP_EMPLOYEE where emp_password='", obj), "'")));
			if (dataSet5.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show("ไม\u0e48พบรห\u0e31ส");
				return;
			}
			if (RoomArr.Count == 0)
			{
				DataSet dataSet6 = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
				string right5 = "";
				string right6 = "";
				DataSet dataSet7 = Module1.connect("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='" + RoomNo + "' order by cin_room_out desc");
				if (dataSet7.Tables[0].Rows.Count != 0)
				{
					right5 = Conversions.ToString(dataSet7.Tables[0].Rows[0]["cin_no"]);
					right6 = Conversions.ToString(dataSet7.Tables[0].Rows[0]["cin_cust_name"]);
				}
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=", dataSet6.Tables[0].Rows[0]["id"])));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", dataSet5.Tables[0].Rows[0]["Emp_Name"]), "', '"), RoomNo), "', '"), DateTime.Now), "', '"), RichTextBox1.Text), "','"), right5), "','"), right6), "')")));
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟทำความสะอาดเร\u0e35ยบร\u0e49อย");
			}
			else
			{
				int num5 = RoomArr.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					DataSet dataSet8 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", RoomArr[num6]), "'")));
					string right7 = "";
					string right8 = "";
					DataSet dataSet9 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='", RoomArr[num6]), "' order by cin_room_out desc")));
					if (dataSet9.Tables[0].Rows.Count != 0)
					{
						right7 = Conversions.ToString(dataSet9.Tables[0].Rows[0]["cin_no"]);
						right8 = Conversions.ToString(dataSet9.Tables[0].Rows[0]["cin_cust_name"]);
					}
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=", dataSet8.Tables[0].Rows[0]["id"])));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", dataSet5.Tables[0].Rows[0]["Emp_Name"]), "', '"), RoomArr[num6]), "', '"), DateTime.Now), "', '"), RichTextBox1.Text), "','"), right7), "','"), right8), "')")));
					Module1.Power_set(Conversions.ToString(RoomArr[num6]), "OFF", "", "ป\u0e34ดไฟทำความสะอาดเร\u0e35ยบร\u0e49อย");
					num6++;
				}
			}
			ISOK = true;
			Close();
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from HT_Book_Date where id=", dataSet.Tables[0].Rows[0]["room_book"])));
		MyProject.Forms.frmMain1.ButtonItem10_Click(null, null, Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]));
		Close();
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		checked
		{
			if (!Module1.HouseWifeMode)
			{
				if (RoomArr.Count == 0)
				{
					DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
					string right = "";
					string right2 = "";
					DataSet dataSet2 = Module1.connect("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='" + RoomNo + "' order by cin_room_out desc");
					if (dataSet2.Tables[0].Rows.Count != 0)
					{
						right = Conversions.ToString(dataSet2.Tables[0].Rows[0]["cin_no"]);
						right2 = Conversions.ToString(dataSet2.Tables[0].Rows[0]["cin_cust_name"]);
					}
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Manternace='yes' where id=", dataSet.Tables[0].Rows[0]["id"])));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", Module1.loginName), "', '"), RoomNo), "', '"), DateTime.Now), "', 'เปล\u0e35\u0e48ยนสถานะเป\u0e47นซ\u0e48อม : "), RichTextBox1.Text), "','"), right), "','"), right2), "')")));
					Module1.INSERT_REPAIR(RoomNo, Conversions.ToString(Module1.loginName), RichTextBox1.Text);
				}
				else
				{
					int num = RoomArr.Count - 1;
					int num2 = 0;
					while (true)
					{
						int num3 = num2;
						int num4 = num;
						if (num3 > num4)
						{
							break;
						}
						DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", RoomArr[num2]), "'")));
						string right3 = "";
						string right4 = "";
						DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='", RoomArr[num2]), "' order by cin_room_out desc")));
						if (dataSet4.Tables[0].Rows.Count != 0)
						{
							right3 = Conversions.ToString(dataSet4.Tables[0].Rows[0]["cin_no"]);
							right4 = Conversions.ToString(dataSet4.Tables[0].Rows[0]["cin_cust_name"]);
						}
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Manternace='yes' where id=", dataSet3.Tables[0].Rows[0]["id"])));
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", Module1.loginName), "', '"), RoomArr[num2]), "', '"), DateTime.Now), "', 'เปล\u0e35\u0e48ยนสถานะเป\u0e47นซ\u0e48อม : "), RichTextBox1.Text), "','"), right3), "','"), right4), "')")));
						Module1.INSERT_REPAIR(Conversions.ToString(RoomArr[num2]), Conversions.ToString(Module1.loginName), RichTextBox1.Text);
						num2++;
					}
				}
				ISOK = true;
				Close();
				return;
			}
			object obj = Interaction.InputBox("กร\u0e38ณากรอกรห\u0e31สผ\u0e48าน", "รห\u0e31สผ\u0e48าน");
			if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
			{
				return;
			}
			DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from TB_MRP_EMPLOYEE where emp_password='", obj), "'")));
			if (dataSet5.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show("ไม\u0e48พบรห\u0e31ส");
				return;
			}
			if (RoomArr.Count == 0)
			{
				DataSet dataSet6 = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
				string right5 = "";
				string right6 = "";
				DataSet dataSet7 = Module1.connect("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='" + RoomNo + "' order by cin_room_out desc");
				if (dataSet7.Tables[0].Rows.Count != 0)
				{
					right5 = Conversions.ToString(dataSet7.Tables[0].Rows[0]["cin_no"]);
					right6 = Conversions.ToString(dataSet7.Tables[0].Rows[0]["cin_cust_name"]);
				}
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Manternace='yes' where id=", dataSet6.Tables[0].Rows[0]["id"])));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", dataSet5.Tables[0].Rows[0]["Emp_Name"]), "', '"), RoomNo), "', '"), DateTime.Now), "', 'เปล\u0e35\u0e48ยนสถานะเป\u0e47นซ\u0e48อม : "), RichTextBox1.Text), "','"), right5), "','"), right6), "')")));
				Module1.INSERT_REPAIR(RoomNo, Conversions.ToString(dataSet5.Tables[0].Rows[0]["Emp_Name"]), RichTextBox1.Text);
			}
			else
			{
				int num5 = RoomArr.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					DataSet dataSet8 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", RoomArr[num6]), "'")));
					string right7 = "";
					string right8 = "";
					DataSet dataSet9 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='", RoomArr[num6]), "' order by cin_room_out desc")));
					if (dataSet9.Tables[0].Rows.Count != 0)
					{
						right7 = Conversions.ToString(dataSet9.Tables[0].Rows[0]["cin_no"]);
						right8 = Conversions.ToString(dataSet9.Tables[0].Rows[0]["cin_cust_name"]);
					}
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Manternace='yes' where id=", dataSet8.Tables[0].Rows[0]["id"])));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", dataSet5.Tables[0].Rows[0]["Emp_Name"]), "', '"), RoomArr[num6]), "', '"), DateTime.Now), "', 'เปล\u0e35\u0e48ยนสถานะเป\u0e47นซ\u0e48อม : "), RichTextBox1.Text), "','"), right7), "','"), right8), "')")));
					Module1.INSERT_REPAIR(Conversions.ToString(RoomArr[num6]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["Emp_Name"]), RichTextBox1.Text);
					num6++;
				}
			}
			if (RoomArr.Count != 0)
			{
				int num8 = RoomArr.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					int num4 = num8;
					if (num10 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num9]), "ON", "", "เป\u0e34ดไฟจากป\u0e38\u0e48ม ซ\u0e48อม");
						num9++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "ON", "", "เป\u0e34ดไฟจากป\u0e38\u0e48ม ซ\u0e48อม");
			}
			check_power();
			ISOK = true;
			Close();
		}
	}

	private void ButtonX14_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count != 0)
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องทำความสะอาด");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องทำความสะอาด");
			}
			check_power();
		}
	}

	private void ButtonX15_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count != 0)
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องทำความสะอาด");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องทำความสะอาด");
			}
			check_power();
		}
	}

	private void ButtonX2_Click_1(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count != 0)
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "ON", "", "เป\u0e34ดไฟทำความสะอาด");
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("update HT_Rooms set Room_Clean_Time='" + Conversions.ToString(DateTime.Now.ToOADate()), "' where Room_no='"), RoomArr[num2]), "'")));
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "ON", "", "เป\u0e34ดไฟทำความสะอาด");
				Module1.connect("update HT_Rooms set Room_Clean_Time='" + Conversions.ToString(DateTime.Now.ToOADate()) + "' where Room_no='" + RoomNo + "'");
			}
			ISOK = true;
			Close();
		}
	}
}
