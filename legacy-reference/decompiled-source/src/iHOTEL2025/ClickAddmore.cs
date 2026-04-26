using System;
using System.Collections.Generic;
using System.ComponentModel;
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
public class ClickAddmore : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("Tend")]
	private DateTimePicker _Tend;

	[AccessedThroughProperty("Tstart")]
	private DateTimePicker _Tstart;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("Tnum")]
	private TextBox _Tnum;

	public string RoomNo;

	public bool ISOK;

	internal virtual PanelEx PanelEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx1 = value;
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

	internal virtual ButtonX ButtonX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX5_Click;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click -= value2;
			}
			_ButtonX5 = value;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click += value2;
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

	internal virtual DateTimePicker Tend
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tend;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tend_ValueChanged;
			if (_Tend != null)
			{
				_Tend.ValueChanged -= value2;
			}
			_Tend = value;
			if (_Tend != null)
			{
				_Tend.ValueChanged += value2;
			}
		}
	}

	internal virtual DateTimePicker Tstart
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tstart;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tend_ValueChanged;
			if (_Tstart != null)
			{
				_Tstart.ValueChanged -= value2;
			}
			_Tstart = value;
			if (_Tstart != null)
			{
				_Tstart.ValueChanged += value2;
			}
		}
	}

	internal virtual LabelX LabelX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX3 = value;
		}
	}

	internal virtual LabelX LabelX2
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX2 = value;
		}
	}

	internal virtual TextBox Tnum
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tnum = value;
		}
	}

	[DebuggerNonUserCode]
	static ClickAddmore()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickAddmore()
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
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Tnum = new System.Windows.Forms.TextBox();
		this.Tend = new System.Windows.Forms.DateTimePicker();
		this.Tstart = new System.Windows.Forms.DateTimePicker();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Tnum);
		this.PanelEx1.Controls.Add(this.Tend);
		this.PanelEx1.Controls.Add(this.Tstart);
		this.PanelEx1.Controls.Add(this.ButtonX5);
		this.PanelEx1.Controls.Add(this.ButtonX4);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(354, 139);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 3;
		this.Tnum.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox tnum = this.Tnum;
		location = new System.Drawing.Point(147, 76);
		tnum.Location = location;
		System.Windows.Forms.TextBox tnum2 = this.Tnum;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tnum2.Margin = margin;
		this.Tnum.Name = "Tnum";
		this.Tnum.ReadOnly = true;
		System.Windows.Forms.TextBox tnum3 = this.Tnum;
		size = new System.Drawing.Size(47, 26);
		tnum3.Size = size;
		this.Tnum.TabIndex = 85;
		this.Tnum.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Tend.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.Tend.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tend = this.Tend;
		location = new System.Drawing.Point(147, 42);
		tend.Location = location;
		System.Windows.Forms.DateTimePicker tend2 = this.Tend;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tend2.Margin = margin;
		this.Tend.Name = "Tend";
		System.Windows.Forms.DateTimePicker tend3 = this.Tend;
		size = new System.Drawing.Size(193, 26);
		tend3.Size = size;
		this.Tend.TabIndex = 83;
		this.Tstart.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.Tstart.Enabled = false;
		this.Tstart.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tstart = this.Tstart;
		location = new System.Drawing.Point(147, 8);
		tstart.Location = location;
		System.Windows.Forms.DateTimePicker tstart2 = this.Tstart;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tstart2.Margin = margin;
		this.Tstart.Name = "Tstart";
		System.Windows.Forms.DateTimePicker tstart3 = this.Tstart;
		size = new System.Drawing.Size(193, 26);
		tstart3.Size = size;
		this.Tstart.TabIndex = 84;
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX5;
		location = new System.Drawing.Point(197, 111);
		buttonX.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX5;
		size = new System.Drawing.Size(75, 23);
		buttonX2.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 2;
		this.ButtonX5.Text = "ตกลง";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX4;
		location = new System.Drawing.Point(274, 111);
		buttonX3.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX4;
		size = new System.Drawing.Size(75, 23);
		buttonX4.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 2;
		this.ButtonX4.Text = "ยกเล\u0e34ก";
		this.LabelX3.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX3;
		location = new System.Drawing.Point(13, 79);
		labelX.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX3;
		size = new System.Drawing.Size(107, 23);
		labelX2.Size = size;
		this.LabelX3.TabIndex = 0;
		this.LabelX3.Text = "รวมจำนวนค\u0e37น";
		this.LabelX2.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX2;
		location = new System.Drawing.Point(13, 44);
		labelX3.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX2;
		size = new System.Drawing.Size(107, 23);
		labelX4.Size = size;
		this.LabelX2.TabIndex = 0;
		this.LabelX2.Text = "ว\u0e31นท\u0e35\u0e48 Check-OUT";
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX1;
		location = new System.Drawing.Point(13, 11);
		labelX5.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX1;
		size = new System.Drawing.Size(107, 23);
		labelX6.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ว\u0e31นท\u0e35\u0e48 Check-IN";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(354, 139);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickAddmore";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ClickBook";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void ClickBook_FormClosing(object sender, FormClosingEventArgs e)
	{
		RoomNo = "";
	}

	private void ClickBook_Load(object sender, EventArgs e)
	{
		Text = "รายการห\u0e49อง " + RoomNo;
		Tstart.Value = DateTime.Now;
		Tend.Value = DateTime.Now;
		ISOK = false;
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Tend_ValueChanged(object sender, EventArgs e)
	{
		if (DateTime.Compare(Tstart.Value, DateTime.Now) < 0)
		{
			Tstart.Value = DateTime.Now;
		}
		if (DateTime.Compare(Tend.Value, Tstart.Value) == 0)
		{
			if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
			else
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.AddDays(1.0).Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
		}
		if (DateTime.Compare(Tend.Value, Tstart.Value) < 0)
		{
			if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
			else
			{
				Tend.Value = Conversions.ToDate(Conversions.ToString(Tstart.Value.AddDays(1.0).Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
			}
		}
		Tnum.Text = Conversions.ToString(DateAndTime.DateDiff(DateInterval.Day, Tstart.Value.Date, Tend.Value.Date));
		if (DateTime.Compare(Tstart.Value.Date, Tend.Value.Date) == 0)
		{
			Tnum.Text = Conversions.ToString(1);
		}
		else if (DateTime.Compare(Tstart.Value.Date, Tend.Value.Date) != 0 && ((decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(Tstart.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0)))
		{
			Tnum.Text = Conversions.ToString(decimal.Add(Conversions.ToDecimal(Tnum.Text), 1m));
		}
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmCheckIn.Tstart.Value = Tstart.Value;
		MyProject.Forms.FrmCheckIn.Tend.Value = Tend.Value;
		MyProject.Forms.FrmCheckIn.Button3_Click(null, null);
		ISOK = true;
		Close();
	}
}
