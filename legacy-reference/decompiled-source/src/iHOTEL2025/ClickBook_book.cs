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

namespace iHOTEL2025;

[DesignerGenerated]
public class ClickBook_book : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	public string RoomNo;

	public bool ISOK;

	public ArrayList RoomArr;

	public DateTime B_DATE;

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
	static ClickBook_book()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickBook_book()
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickBook_book));
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		System.Drawing.Point location = new System.Drawing.Point(96, 77);
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
		location = new System.Drawing.Point(182, 11);
		buttonX3.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX3;
		size = new System.Drawing.Size(162, 52);
		buttonX4.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "   ลบการจอง\r\n  (ห\u0e49องท\u0e35\u0e48เล\u0e37อก)";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX4;
		location = new System.Drawing.Point(12, 11);
		buttonX5.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX4;
		size = new System.Drawing.Size(162, 52);
		buttonX6.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 3;
		this.ButtonX4.Text = "แก\u0e49ไขรายการจอง";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(354, 135);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX1);
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickBook_book";
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
				DataSet dataSet = Module1.connect("select * from  View_Book_Date where book_type='" + RoomNo + "' and book_date_ds='" + Conversions.ToString(B_DATE.Date) + "'");
				if (dataSet.Tables[0].Rows.Count != 0)
				{
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Date where book_type='", dataSet.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet.Tables[0].Rows[0]["book_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Ds where book_room_type='", dataSet.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet.Tables[0].Rows[0]["book_no"]), "'")));
					Module1.SET_STATUS_BOOKING(Conversions.ToString(dataSet.Tables[0].Rows[0]["book_no"]));
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
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  View_Book_Date where book_type='", RoomArr[num2]), "' and book_date_ds='"), B_DATE.Date), "'")));
					if (dataSet2.Tables[0].Rows.Count != 0)
					{
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Date where book_type='", dataSet2.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet2.Tables[0].Rows[0]["book_no"]), "'")));
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Book_Ds where book_room_type='", dataSet2.Tables[0].Rows[0]["book_type"]), "' and book_no='"), dataSet2.Tables[0].Rows[0]["book_no"]), "'")));
						Module1.SET_STATUS_BOOKING(Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]));
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

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		if (RoomArr.Count == 0)
		{
			DataSet dataSet = Module1.connect("select * from  View_Book_Date where book_type='" + RoomNo + "' and book_date_ds='" + Conversions.ToString(B_DATE.Date) + "'");
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  HT_Book_H where book_id='", dataSet.Tables[0].Rows[0]["book_no"]), "'")));
			if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
			{
				FrmAddBook frmAddBook = new FrmAddBook();
				frmAddBook.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_no"]);
				frmAddBook.ShowDialog();
			}
			else
			{
				FrmAddBook2 frmAddBook2 = new FrmAddBook2();
				frmAddBook2.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_no"]);
				frmAddBook2.ShowDialog();
			}
		}
		else
		{
			DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  View_Book_Date where book_type='", RoomArr[0]), "' and book_date_ds='"), B_DATE), "'")));
			DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  HT_Book_H where book_id='", dataSet3.Tables[0].Rows[0]["book_no"]), "'")));
			if (Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
			{
				FrmAddBook frmAddBook3 = new FrmAddBook();
				frmAddBook3.EDIT_ID = Conversions.ToString(dataSet3.Tables[0].Rows[0]["book_no"]);
				frmAddBook3.ShowDialog();
			}
			else
			{
				FrmAddBook2 frmAddBook4 = new FrmAddBook2();
				frmAddBook4.EDIT_ID = Conversions.ToString(dataSet3.Tables[0].Rows[0]["book_no"]);
				frmAddBook4.ShowDialog();
			}
		}
		ISOK = true;
		Close();
	}
}
