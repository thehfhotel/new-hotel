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
public class ReportCleanRoom : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

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

	[DebuggerNonUserCode]
	static ReportCleanRoom()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportCleanRoom()
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
		this.SuspendLayout();
		this.Button1.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Button button = this.Button1;
		System.Drawing.Point location = new System.Drawing.Point(12, 12);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		System.Drawing.Size size = new System.Drawing.Size(299, 62);
		button2.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.Text = "ออกรายงาน";
		this.Button1.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(323, 86);
		this.ClientSize = size;
		this.Controls.Add(this.Button1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportCleanRoom";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานห\u0e49องท\u0e35\u0e48รอทำความสะอาด";
		this.ResumeLayout(false);
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_clean='yes' order by room_no");
		Module1.localdata.ReportDays.Rows.Clear();
		decimal num = default(decimal);
		checked
		{
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
				num = decimal.Add(num, 1m);
				Module1.localdata.ReportDays.AddReportDaysRow(Strings.Format(DateTime.Now, "dd/MM/yyyy HH:mm"), Conversions.ToString(num3 + 1), Conversions.ToString(dataSet.Tables[0].Rows[num3]["room_no"]), Conversions.ToString(dataSet.Tables[0].Rows[num3]["room_type"]), "-", "-", "", "", "", Conversions.ToString(num), "", "", "", "", "", "", "", "", "", "");
				num3++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCleanRoom crystalReportCleanRoom = new CrystalReportCleanRoom();
			crystalReportCleanRoom.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCleanRoom;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			Close();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1_ValueChanged(null, null);
	}
}
