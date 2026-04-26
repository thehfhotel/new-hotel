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
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class ClickManternance : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX15")]
	private ButtonX _ButtonX15;

	[AccessedThroughProperty("ButtonX14")]
	private ButtonX _ButtonX14;

	public string RoomNo;

	public bool ISOK;

	public ArrayList RoomArr;

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

	[DebuggerNonUserCode]
	static ClickManternance()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickManternance()
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
		ISOK = false;
		RoomArr = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickManternance));
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		System.Drawing.Point location = new System.Drawing.Point(182, 73);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		System.Drawing.Size size = new System.Drawing.Size(162, 52);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX3;
		location = new System.Drawing.Point(12, 73);
		buttonX3.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX3;
		size = new System.Drawing.Size(162, 52);
		buttonX4.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "ซ\u0e48อมบำร\u0e38ง\r\nเสร\u0e47จเร\u0e35ยบร\u0e49อย";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ButtonX15.Image");
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_0;
		location = new System.Drawing.Point(182, 12);
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
		location = new System.Drawing.Point(12, 12);
		buttonX_3.Location = location;
		this.ButtonX_1.Name = "ButtonX14";
		DevComponents.DotNetBar.ButtonX buttonX_4 = this.ButtonX_1;
		size = new System.Drawing.Size(162, 52);
		buttonX_4.Size = size;
		this.ButtonX_1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_1.TabIndex = 14;
		this.ButtonX_1.Text = "เป\u0e34ดไฟ";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(354, 134);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX_0);
		this.Controls.Add(this.ButtonX_1);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickManternance";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ClickBook";
		this.ResumeLayout(false);
	}

	private void ClickBook_FormClosing(object sender, FormClosingEventArgs e)
	{
		MSSQL.CodeErr = false;
		RoomNo = "";
	}

	private void ClickBook_Load(object sender, EventArgs e)
	{
		MSSQL.CodeErr = true;
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
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Manternace='no' where id=", dataSet.Tables[0].Rows[0]["id"])));
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
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", RoomArr[num2]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Manternace='no' where id=", dataSet2.Tables[0].Rows[0]["id"])));
					num2++;
				}
			}
			if (RoomArr.Count != 0)
			{
				int num5 = RoomArr.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num6]), "OFF", "", "ป\u0e34ดไฟ ซ\u0e48องบำร\u0e38งเสร\u0e47จเร\u0e35ยบร\u0e49อย");
						num6++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟ ซ\u0e48องบำร\u0e38งเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			}
			check_power();
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
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าซ\u0e48อมบำร\u0e38ง");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าซ\u0e48อมบำร\u0e38ง");
			}
			check_power();
			ISOK = true;
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
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าซ\u0e48อมบำร\u0e38ง");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าซ\u0e48อมบำร\u0e38ง");
			}
			check_power();
			ISOK = true;
		}
	}
}
