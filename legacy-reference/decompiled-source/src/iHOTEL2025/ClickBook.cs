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
public class ClickBook : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

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
			EventHandler value2 = ButtonX2_Click;
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

	internal virtual ButtonX ButtonX4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX4_Click;
			if (_ButtonX4 != null)
			{
				_ButtonX4.Click -= value2;
			}
			_ButtonX4 = value;
			if (_ButtonX4 != null)
			{
				_ButtonX4.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static ClickBook()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickBook()
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickBook));
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		System.Drawing.Point location = new System.Drawing.Point(96, 131);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		System.Drawing.Size size = new System.Drawing.Size(162, 52);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(12, 12);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(332, 52);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "Check-IN";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(182, 71);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(162, 52);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "   ลบการจอง\r\n  (ห\u0e49องท\u0e35\u0e48เล\u0e37อก)";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX4;
		location = new System.Drawing.Point(12, 71);
		buttonX7.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX4;
		size = new System.Drawing.Size(162, 52);
		buttonX8.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 3;
		this.ButtonX4.Text = "แก\u0e49ไขรายการจอง";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(354, 190);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX1);
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickBook";
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
			ISOK = false;
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการยกเล\u0e34กการจองห\u0e49องพ\u0e31กท\u0e35\u0e48เล\u0e37อกหร\u0e37อไม\u0e48", "ยกเล\u0e34กการจอง", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.No)
		{
			return;
		}
		checked
		{
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' where id=", dataSet.Tables[0].Rows[0]["id"])));
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from  HT_Book_Date where id=", dataSet.Tables[0].Rows[0]["Room_Book"])));
				if (dataSet2.Tables[0].Rows.Count != 0)
				{
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Date where book_type='", dataSet2.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet2.Tables[0].Rows[0]["book_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Ds where book_room_type='", dataSet2.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet2.Tables[0].Rows[0]["book_no"]), "'")));
					Module1.SET_STATUS_BOOKING(Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]));
				}
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
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set room_book_ds='',Room_Book='',Room_Book_Name='',Room_Book_Time='' where id=", dataSet3.Tables[0].Rows[0]["id"])));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Book_Date set Book_ok=1 where id=", dataSet3.Tables[0].Rows[0]["Room_Book"])));
					DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from  HT_Book_Date where id=", dataSet3.Tables[0].Rows[0]["Room_Book"])));
					if (dataSet4.Tables[0].Rows.Count != 0)
					{
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Date where book_type='", dataSet4.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet4.Tables[0].Rows[0]["book_no"]), "'")));
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Ds where book_room_type='", dataSet4.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet4.Tables[0].Rows[0]["book_no"]), "'")));
						Module1.SET_STATUS_BOOKING(Conversions.ToString(dataSet4.Tables[0].Rows[0]["book_no"]));
					}
					num2++;
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
		Module1.checkin_mode = "";
		DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from HT_Book_Date where id=", dataSet.Tables[0].Rows[0]["room_book"])));
		MyProject.Forms.frmMain1.ButtonItem10_Click(null, null, Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]));
		Close();
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		if (RoomArr.Count == 0)
		{
			DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from  HT_Book_Date where id=", dataSet.Tables[0].Rows[0]["Room_Book"])));
			DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  HT_Book_H where book_id='", dataSet2.Tables[0].Rows[0]["book_no"]), "'")));
			if (Operators.ConditionalCompareObjectEqual(dataSet3.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
			{
				FrmAddBook frmAddBook = new FrmAddBook();
				frmAddBook.EDIT_ID = Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]);
				frmAddBook.ShowDialog();
			}
			else
			{
				FrmAddBook2 frmAddBook2 = new FrmAddBook2();
				frmAddBook2.EDIT_ID = Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]);
				frmAddBook2.ShowDialog();
			}
		}
		else
		{
			DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_no='", RoomArr[0]), "'")));
			DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from  HT_Book_Date where id=", dataSet4.Tables[0].Rows[0]["Room_Book"])));
			DataSet dataSet6 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  HT_Book_H where book_id='", dataSet5.Tables[0].Rows[0]["book_no"]), "'")));
			if (Operators.ConditionalCompareObjectEqual(dataSet6.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
			{
				FrmAddBook frmAddBook3 = new FrmAddBook();
				frmAddBook3.EDIT_ID = Conversions.ToString(dataSet5.Tables[0].Rows[0]["book_no"]);
				frmAddBook3.ShowDialog();
			}
			else
			{
				FrmAddBook2 frmAddBook4 = new FrmAddBook2();
				frmAddBook4.EDIT_ID = Conversions.ToString(dataSet5.Tables[0].Rows[0]["book_no"]);
				frmAddBook4.ShowDialog();
			}
		}
		ISOK = true;
		Close();
	}
}
