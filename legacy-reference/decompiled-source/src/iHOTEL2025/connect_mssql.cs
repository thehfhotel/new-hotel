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
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class connect_mssql : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBarX _ProgressBarX1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	private int num_dot;

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

	[DebuggerNonUserCode]
	static connect_mssql()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public connect_mssql()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += connect_mssql_FormClosing;
		base.Load += connect_mssql_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		num_dot = 1;
		InitializeComponent();
	}

	[DebuggerNonUserCode]
	protected override void Dispose(bool disposing)
	{
		if (disposing && components != null)
		{
			components.Dispose();
		}
		base.Dispose(disposing);
	}

	[System.Diagnostics.DebuggerStepThrough]
	private void InitializeComponent()
	{
		this.components = new System.ComponentModel.Container();
		this.Label1 = new System.Windows.Forms.Label();
		this.ProgressBarX1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(6, -1);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(280, 23);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "กำล\u0e31งต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ล กร\u0e38ณารอ...";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.ProgressBarX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX = this.ProgressBarX1;
		location = new System.Drawing.Point(6, 22);
		progressBarX.Location = location;
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX2 = this.ProgressBarX1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		progressBarX2.Margin = margin;
		this.ProgressBarX1.Maximum = 500;
		this.ProgressBarX1.Name = "ProgressBarX1";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX3 = this.ProgressBarX1;
		size = new System.Drawing.Size(313, 18);
		progressBarX3.Size = size;
		this.ProgressBarX1.TabIndex = 1;
		this.ProgressBarX1.Text = "ProgressBarX1";
		this.Timer1.Interval = 10;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = iHOTEL2025.My.Resources.Resources.reload;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(6, 47);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		size = new System.Drawing.Size(198, 27);
		buttonX3.Size = size;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (0)";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = iHOTEL2025.My.Resources.Resources.delete1;
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		location = new System.Drawing.Point(208, 47);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX5.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX1;
		size = new System.Drawing.Size(111, 27);
		buttonX6.Size = size;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ดโปรแกรม";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(325, 76);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.ProgressBarX1);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedDialog;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		size = new System.Drawing.Size(341, 115);
		this.MaximumSize = size;
		size = new System.Drawing.Size(341, 115);
		this.MinimumSize = size;
		this.Name = "connect_mssql";
		this.Opacity = 0.9;
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ไม\u0e48สามารถต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลได\u0e49";
		this.ResumeLayout(false);
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		checked
		{
			if (num_dot == 500)
			{
				num_dot = 1;
				MSSQL.connectmssql();
			}
			else
			{
				num_dot++;
			}
			if (num_dot == 10)
			{
				ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (" + Conversions.ToString(5) + ")";
			}
			else if (num_dot == 90)
			{
				ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (" + Conversions.ToString(4) + ")";
			}
			else if (num_dot == 180)
			{
				ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (" + Conversions.ToString(3) + ")";
			}
			else if (num_dot == 270)
			{
				ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (" + Conversions.ToString(2) + ")";
			}
			else if (num_dot == 360)
			{
				ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (" + Conversions.ToString(1) + ")";
			}
			else if (num_dot == 450)
			{
				ButtonX2.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลเด\u0e35\u0e4bยวน\u0e35\u0e49 (" + Conversions.ToString(0) + ")";
			}
			ProgressBarX1.Value = num_dot;
			show_text();
		}
	}

	public void show_text()
	{
		if (MSSQL.conn.State == ConnectionState.Open)
		{
			Label1.Text = "ต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ลได\u0e49แล\u0e49ว";
			Close();
		}
		else
		{
			Label1.Text = "กำล\u0e31งต\u0e34ดต\u0e48อฐานข\u0e49อม\u0e39ล";
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Module1.CloseProgram = true;
		MSSQL.DataError = "";
		MyProject.Forms.frmMain1.Close();
		Application.Exit();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		MSSQL.connectmssql();
		show_text();
		num_dot = 1;
	}

	private void connect_mssql_FormClosing(object sender, FormClosingEventArgs e)
	{
		Timer1.Enabled = false;
	}

	private void connect_mssql_Load(object sender, EventArgs e)
	{
		Timer1.Enabled = true;
		Module1.CloseProgram = false;
	}
}
