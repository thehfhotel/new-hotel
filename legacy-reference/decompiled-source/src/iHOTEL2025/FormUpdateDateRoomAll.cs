using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormUpdateDateRoomAll : Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBarX _ProgressBarX1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	internal virtual ProgressBarX ProgressBarX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ProgressBarX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ProgressBarX1 = value;
		}
	}

	internal virtual Timer Timer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timer1_Tick;
			if (_Timer1 != null)
			{
				_Timer1.Tick -= value2;
			}
			_Timer1 = value;
			if (_Timer1 != null)
			{
				_Timer1.Tick += value2;
			}
		}
	}

	internal virtual LabelX LabelX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX1 = value;
		}
	}

	[DebuggerNonUserCode]
	static FormUpdateDateRoomAll()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormUpdateDateRoomAll()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormUpdateDateRoomAll_Load;
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
		this.components = new System.ComponentModel.Container();
		this.ProgressBarX1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.SuspendLayout();
		this.ProgressBarX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX = this.ProgressBarX1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		progressBarX.Location = location;
		this.ProgressBarX1.Name = "ProgressBarX1";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX2 = this.ProgressBarX1;
		System.Drawing.Size size = new System.Drawing.Size(572, 23);
		progressBarX2.Size = size;
		this.ProgressBarX1.TabIndex = 0;
		this.ProgressBarX1.Text = "ProgressBarX1";
		this.Timer1.Interval = 10;
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX1;
		location = new System.Drawing.Point(14, 37);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		size = new System.Drawing.Size(571, 23);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 2;
		this.LabelX1.Text = "LabelX1";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(597, 70);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.LabelX1);
		this.Controls.Add(this.ProgressBarX1);
		this.Name = "FormUpdateDateRoomAll";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "อ\u0e31บเดทข\u0e49อม\u0e39ลว\u0e31นท\u0e35\u0e48";
		this.TopMost = true;
		this.ResumeLayout(false);
	}

	private void FormUpdateDateRoomAll_Load(object sender, EventArgs e)
	{
		Timer1.Enabled = true;
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		LabelX1.Text = "กำล\u0e31งอ\u0e31บเดทข\u0e49อม\u0e39ล";
		ProgressBarX1.Value = 0;
		DataSet dataSet = Module1.connect("select * from HT_Room_Status where room_date_oa=0");
		ProgressBarX1.Maximum = dataSet.Tables[0].Rows.Count;
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
				LabelX1.Text = "กำล\u0e31งอ\u0e31บเดทข\u0e49อม\u0e39ล " + Conversions.ToString(num2 + 1) + " / " + Conversions.ToString(dataSet.Tables[0].Rows.Count);
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_Room_Status set room_date_oa=" + Conversions.ToString(Conversions.ToDate(dataSet.Tables[0].Rows[num2]["room_date"]).ToOADate()), " where id="), dataSet.Tables[0].Rows[num2]["id"])));
				ProgressBarX1.Value = num2 + 1;
				Application.DoEvents();
				num2++;
			}
			Close();
		}
	}
}
