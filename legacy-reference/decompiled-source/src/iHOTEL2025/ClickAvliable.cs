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

namespace iHOTEL2025;

[DesignerGenerated]
public class ClickAvliable : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

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

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("ButtonX15")]
	private ButtonX _ButtonX15;

	[AccessedThroughProperty("ButtonX14")]
	private ButtonX _ButtonX14;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	[AccessedThroughProperty("ButtonX8")]
	private ButtonX _ButtonX8;

	[AccessedThroughProperty("ButtonX9")]
	private ButtonX _ButtonX9;

	[AccessedThroughProperty("LabelX4")]
	private LabelX _LabelX4;

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

	internal virtual ButtonX ButtonX7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX7_Click;
			if (_ButtonX7 != null)
			{
				_ButtonX7.Click -= value2;
			}
			_ButtonX7 = value;
			if (_ButtonX7 != null)
			{
				_ButtonX7.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX8_Click;
			if (_ButtonX8 != null)
			{
				_ButtonX8.Click -= value2;
			}
			_ButtonX8 = value;
			if (_ButtonX8 != null)
			{
				_ButtonX8.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX9_Click;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click -= value2;
			}
			_ButtonX9 = value;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click += value2;
			}
		}
	}

	internal virtual LabelX LabelX4
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX4 = value;
		}
	}

	[DebuggerNonUserCode]
	static ClickAvliable()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickAvliable()
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickAvliable));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.LabelX4 = new DevComponents.DotNetBar.LabelX();
		this.Tnum = new System.Windows.Forms.TextBox();
		this.Tend = new System.Windows.Forms.DateTimePicker();
		this.Tstart = new System.Windows.Forms.DateTimePicker();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX9 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX8 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.LabelX4);
		this.PanelEx1.Controls.Add(this.Tnum);
		this.PanelEx1.Controls.Add(this.Tend);
		this.PanelEx1.Controls.Add(this.Tstart);
		this.PanelEx1.Controls.Add(this.ButtonX5);
		this.PanelEx1.Controls.Add(this.ButtonX4);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 18.25f);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(1, 1);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(353, 373);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 3;
		this.PanelEx1.Visible = false;
		this.LabelX4.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX4;
		location = new System.Drawing.Point(221, 191);
		labelX.Location = location;
		this.LabelX4.Name = "LabelX4";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX4;
		size = new System.Drawing.Size(54, 51);
		labelX2.Size = size;
		this.LabelX4.TabIndex = 86;
		this.LabelX4.Text = "ค\u0e37น";
		this.Tnum.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tnum.Font = new System.Drawing.Font("Tahoma", 21.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		System.Windows.Forms.TextBox tnum = this.Tnum;
		location = new System.Drawing.Point(124, 195);
		tnum.Location = location;
		System.Windows.Forms.TextBox tnum2 = this.Tnum;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tnum2.Margin = margin;
		this.Tnum.Name = "Tnum";
		this.Tnum.ReadOnly = true;
		System.Windows.Forms.TextBox tnum3 = this.Tnum;
		size = new System.Drawing.Size(91, 43);
		tnum3.Size = size;
		this.Tnum.TabIndex = 85;
		this.Tnum.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Tend.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.Tend.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tend = this.Tend;
		location = new System.Drawing.Point(12, 138);
		tend.Location = location;
		System.Windows.Forms.DateTimePicker tend2 = this.Tend;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tend2.Margin = margin;
		this.Tend.Name = "Tend";
		System.Windows.Forms.DateTimePicker tend3 = this.Tend;
		size = new System.Drawing.Size(307, 37);
		tend3.Size = size;
		this.Tend.TabIndex = 83;
		this.Tstart.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.Tstart.Enabled = false;
		this.Tstart.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker tstart = this.Tstart;
		location = new System.Drawing.Point(12, 53);
		tstart.Location = location;
		System.Windows.Forms.DateTimePicker tstart2 = this.Tstart;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tstart2.Margin = margin;
		this.Tstart.Name = "Tstart";
		System.Windows.Forms.DateTimePicker tstart3 = this.Tstart;
		size = new System.Drawing.Size(307, 37);
		tstart3.Size = size;
		this.Tstart.TabIndex = 84;
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX5;
		location = new System.Drawing.Point(55, 263);
		buttonX.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX5;
		size = new System.Drawing.Size(115, 63);
		buttonX2.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 2;
		this.ButtonX5.Text = "ตกลง";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX4;
		location = new System.Drawing.Point(182, 263);
		buttonX3.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX4;
		size = new System.Drawing.Size(115, 63);
		buttonX4.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 2;
		this.ButtonX4.Text = "ยกเล\u0e34ก";
		this.LabelX3.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX3;
		location = new System.Drawing.Point(12, 191);
		labelX3.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX3;
		size = new System.Drawing.Size(139, 51);
		labelX4.Size = size;
		this.LabelX3.TabIndex = 0;
		this.LabelX3.Text = "รวมจำนวน";
		this.LabelX2.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX2;
		location = new System.Drawing.Point(12, 92);
		labelX5.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX2;
		size = new System.Drawing.Size(182, 55);
		labelX6.Size = size;
		this.LabelX2.TabIndex = 0;
		this.LabelX2.Text = "ว\u0e31นท\u0e35\u0e48 Check-OUT";
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX7 = this.LabelX1;
		location = new System.Drawing.Point(12, 17);
		labelX7.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX8 = this.LabelX1;
		size = new System.Drawing.Size(182, 34);
		labelX8.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ว\u0e31นท\u0e35\u0e48 Check-IN";
		this.ButtonX9.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX9.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX9.FocusCuesEnabled = false;
		this.ButtonX9.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX9.Image = (System.Drawing.Image)resources.GetObject("ButtonX9.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX9;
		location = new System.Drawing.Point(178, 70);
		buttonX5.Location = location;
		this.ButtonX9.Name = "ButtonX9";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX9;
		size = new System.Drawing.Size(162, 52);
		buttonX6.Size = size;
		this.ButtonX9.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX9.TabIndex = 18;
		this.ButtonX9.Text = "Check-IN\r\n(รายเด\u0e37อน)";
		this.ButtonX9.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX8.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX8.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX8.FocusCuesEnabled = false;
		this.ButtonX8.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX8.Image = (System.Drawing.Image)resources.GetObject("ButtonX8.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX8;
		location = new System.Drawing.Point(178, 12);
		buttonX7.Location = location;
		this.ButtonX8.Name = "ButtonX8";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX8;
		size = new System.Drawing.Size(162, 52);
		buttonX8.Size = size;
		this.ButtonX8.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX8.TabIndex = 17;
		this.ButtonX8.Text = "Check-IN\r\n (ช\u0e31\u0e48วคราว)";
		this.ButtonX8.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.FocusCuesEnabled = false;
		this.ButtonX7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX7;
		location = new System.Drawing.Point(10, 128);
		buttonX9.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX7;
		size = new System.Drawing.Size(162, 52);
		buttonX10.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 16;
		this.ButtonX7.Text = "  จองห\u0e49องพ\u0e31ก";
		this.ButtonX7.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ButtonX15.Image");
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_0;
		location = new System.Drawing.Point(177, 244);
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
		location = new System.Drawing.Point(9, 244);
		buttonX_3.Location = location;
		this.ButtonX_1.Name = "ButtonX14";
		DevComponents.DotNetBar.ButtonX buttonX_4 = this.ButtonX_1;
		size = new System.Drawing.Size(162, 52);
		buttonX_4.Size = size;
		this.ButtonX_1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_1.TabIndex = 14;
		this.ButtonX_1.Text = "เป\u0e34ดไฟ";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 11f, System.Drawing.FontStyle.Bold);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX3;
		location = new System.Drawing.Point(9, 186);
		buttonX11.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX3;
		size = new System.Drawing.Size(162, 52);
		buttonX12.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "รอ ทำความสะอาด";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX2;
		location = new System.Drawing.Point(10, 12);
		buttonX13.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX2;
		size = new System.Drawing.Size(162, 110);
		buttonX14.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "Check-IN\r\n  (รายว\u0e31น)";
		this.ButtonX2.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX6.FocusCuesEnabled = false;
		this.ButtonX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX6.Image = (System.Drawing.Image)resources.GetObject("ButtonX6.Image");
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX6;
		location = new System.Drawing.Point(177, 186);
		buttonX15.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX6;
		size = new System.Drawing.Size(162, 52);
		buttonX16.Size = size;
		this.ButtonX6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX6.TabIndex = 2;
		this.ButtonX6.Text = "ซ\u0e48อมบำร\u0e38ง";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX1;
		location = new System.Drawing.Point(89, 302);
		buttonX17.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX18 = this.ButtonX1;
		size = new System.Drawing.Size(162, 52);
		buttonX18.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.FromArgb(194, 217, 247);
		size = new System.Drawing.Size(354, 376);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.Controls.Add(this.ButtonX9);
		this.Controls.Add(this.ButtonX8);
		this.Controls.Add(this.ButtonX7);
		this.Controls.Add(this.ButtonX_0);
		this.Controls.Add(this.ButtonX_1);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX6);
		this.Controls.Add(this.ButtonX1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickAvliable";
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
		if (!Module1.POWER_USED)
		{
			ButtonX_1.Enabled = false;
			ButtonX_0.Enabled = false;
		}
		else
		{
			ButtonX_1.Enabled = true;
			ButtonX_0.Enabled = true;
		}
		PanelEx1.Visible = false;
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
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='yes' where id=", dataSet.Tables[0].Rows[0]["id"])));
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
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='yes' where id=", dataSet2.Tables[0].Rows[0]["id"])));
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
		PanelEx1.Visible = true;
		Tstart.Value = DateTime.Now;
		Tend.Value = DateTime.Now;
		Module1.checkin_mode = "";
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		PanelEx1.Visible = false;
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
		LabelX4.Text = "ค\u0e37น";
		if (Operators.ConditionalCompareObjectEqual(Module1.checkin_mode, "รายเด\u0e37อน", TextCompare: false))
		{
			Tnum.Text = "1";
			LabelX4.Text = "เด\u0e37อน";
		}
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		FrmCheckIn frmCheckIn = new FrmCheckIn();
		frmCheckIn.Fstart = Tstart.Value;
		frmCheckIn.Fend = Tend.Value;
		if (RoomArr.Count == 0)
		{
			frmCheckIn.tmp_room = RoomNo;
			frmCheckIn.tmp_roomarr.Clear();
		}
		else
		{
			frmCheckIn.tmp_room = "";
			frmCheckIn.tmp_roomarr = RoomArr;
		}
		frmCheckIn.ShowDialog();
		Close();
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		object obj = Interaction.InputBox("กร\u0e38ณากรอกหมายเหต\u0e38", "กร\u0e38ณากรอกหมายเหต\u0e38");
		if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			return;
		}
		checked
		{
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from HT_Rooms where Room_no='" + RoomNo + "'");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Manternace='yes' where id=", dataSet.Tables[0].Rows[0]["id"])));
				Module1.INSERT_REPAIR(RoomNo, Conversions.ToString(Module1.loginName), Conversions.ToString(obj));
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
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Manternace='yes' where id=", dataSet2.Tables[0].Rows[0]["id"])));
					Module1.INSERT_REPAIR(Conversions.ToString(RoomArr[num2]), Conversions.ToString(Module1.loginName), Conversions.ToString(obj));
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
						Module1.Power_set(Conversions.ToString(RoomArr[num6]), "ON", "", "เป\u0e34ดไฟจากป\u0e38\u0e48ม ซ\u0e48อม");
						num6++;
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
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องว\u0e48าง");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องว\u0e48าง");
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
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องว\u0e48าง");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องว\u0e48าง");
			}
			check_power();
			ISOK = true;
		}
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count == 0)
			{
				FrmAddBook2 frmAddBook = new FrmAddBook2();
				frmAddBook.R_ARR.Add(RoomNo);
				frmAddBook.ShowDialog();
			}
			else
			{
				FrmAddBook2 frmAddBook2 = new FrmAddBook2();
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
					frmAddBook2.R_ARR.Add(RuntimeHelpers.GetObjectValue(RoomArr[num2]));
					num2++;
				}
				frmAddBook2.ShowDialog();
			}
			ISOK = true;
			Close();
		}
	}

	private void ButtonX8_Click(object sender, EventArgs e)
	{
		Module1.checkin_mode = "ช\u0e31\u0e48วคราว";
		PanelEx1.Visible = true;
		Tstart.Value = DateTime.Now;
		Tend.Value = DateTime.Now;
		ButtonX5_Click(null, null);
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		PanelEx1.Visible = true;
		Module1.checkin_mode = "รายเด\u0e37อน";
		PanelEx1.Visible = true;
		Tstart.Value = DateTime.Now;
		Tend.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.AddMonths(1).Date) + " " + Strings.Format(Module1.CHK_Out, "00:00") + ":00");
	}
}
