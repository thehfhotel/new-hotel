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
public class ClickUSE3 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

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

	[DebuggerNonUserCode]
	static ClickUSE3()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickUSE3()
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickUSE3));
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX3;
		System.Drawing.Point location = new System.Drawing.Point(12, 12);
		buttonX.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX3;
		System.Drawing.Size size = new System.Drawing.Size(162, 52);
		buttonX2.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "ค\u0e48าใช\u0e49จ\u0e48ายอ\u0e37\u0e48นๆ";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		location = new System.Drawing.Point(12, 78);
		buttonX3.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		size = new System.Drawing.Size(162, 52);
		buttonX4.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(185, 142);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickUSE3";
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
		ISOK = false;
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		FrmPayAddPro frmPayAddPro = new FrmPayAddPro();
		frmPayAddPro.TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		frmPayAddPro.ShowDialog();
		ISOK = true;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		FrmPayAdd frmPayAdd = new FrmPayAdd();
		frmPayAdd.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		frmPayAdd.ShowDialog();
		ISOK = true;
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		new Room_Note();
		MyProject.Forms.Room_Note.R_NO = RoomNo;
		MyProject.Forms.Room_Note.ShowDialog();
		ISOK = true;
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Room_Status='เข\u0e49าพ\u0e31ก' where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and Cin_room_no='"), RoomNo), "'")));
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
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_Room_no='", RoomArr[num2]), "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Room_Status='เข\u0e49าพ\u0e31ก' where Cin_no='", dataSet2.Tables[0].Rows[0]["Cin_no"]), "' and Cin_room_no='"), RoomArr[num2]), "'")));
					num2++;
				}
			}
			ISOK = true;
			Close();
		}
	}

	private void ButtonX10_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		MyProject.Forms.frmMain1.ButtonItem10_Click(null, null, "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
	}
}
