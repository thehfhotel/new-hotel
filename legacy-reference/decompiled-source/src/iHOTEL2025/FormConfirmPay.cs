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
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormConfirmPay : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("C5")]
	private ButtonX _C5;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("C1")]
	private ButtonX _C1;

	[AccessedThroughProperty("C2")]
	private ButtonX _C2;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("TextBoxX3")]
	private TextBoxX _TextBoxX3;

	[AccessedThroughProperty("Tบ\u0e31ตร")]
	private TextBoxX textBoxX_0;

	[AccessedThroughProperty("LabelX5")]
	private LabelX _LabelX5;

	[AccessedThroughProperty("Tสด")]
	private TextBoxX textBoxX_1;

	[AccessedThroughProperty("LabelX4")]
	private LabelX _LabelX4;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("C4")]
	private ButtonX _C4;

	[AccessedThroughProperty("Tฟร\u0e35")]
	private TextBoxX textBoxX_2;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("C3")]
	private ButtonX _C3;

	[AccessedThroughProperty("Tโอน")]
	private TextBoxX textBoxX_3;

	[AccessedThroughProperty("LabelX6")]
	private LabelX _LabelX6;

	[AccessedThroughProperty("C6")]
	private ButtonX _C6;

	[AccessedThroughProperty("C7")]
	private ButtonX _C7;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("LabelX7")]
	private LabelX _LabelX7;

	[AccessedThroughProperty("C8")]
	private ButtonX _C8;

	[AccessedThroughProperty("Tเว\u0e47ป")]
	private TextBoxX textBoxX_4;

	[AccessedThroughProperty("LabelX8")]
	private LabelX _LabelX8;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	public decimal PTOTAl;

	public decimal PCASH;

	public decimal PCREDIT;

	public decimal PFREE;

	public decimal TRANN;

	public decimal WEB;

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
			EventHandler value2 = PanelEx1_Click;
			if (_PanelEx1 != null)
			{
				_PanelEx1.Click -= value2;
			}
			_PanelEx1 = value;
			if (_PanelEx1 != null)
			{
				_PanelEx1.Click += value2;
			}
		}
	}

	internal virtual ButtonX C5
	{
		[DebuggerNonUserCode]
		get
		{
			return _C5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX1_Click;
			if (_C5 != null)
			{
				_C5.Click -= value2;
			}
			_C5 = value;
			if (_C5 != null)
			{
				_C5.Click += value2;
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
			EventHandler value2 = LabelX1_Click;
			if (_LabelX1 != null)
			{
				_LabelX1.Click -= value2;
			}
			_LabelX1 = value;
			if (_LabelX1 != null)
			{
				_LabelX1.Click += value2;
			}
		}
	}

	internal virtual ButtonX C1
	{
		[DebuggerNonUserCode]
		get
		{
			return _C1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX2_Click;
			if (_C1 != null)
			{
				_C1.Click -= value2;
			}
			_C1 = value;
			if (_C1 != null)
			{
				_C1.Click += value2;
			}
		}
	}

	internal virtual ButtonX C2
	{
		[DebuggerNonUserCode]
		get
		{
			return _C2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX3_Click;
			if (_C2 != null)
			{
				_C2.Click -= value2;
			}
			_C2 = value;
			if (_C2 != null)
			{
				_C2.Click += value2;
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

	internal virtual TextBoxX TextBoxX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBoxX3 = value;
		}
	}

	internal virtual TextBoxX TextBoxX_1
	{
		[DebuggerNonUserCode]
		get
		{
			return textBoxX_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBoxX_0 = value;
		}
	}

	internal virtual LabelX LabelX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX5 = value;
		}
	}

	internal virtual TextBoxX TextBoxX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return textBoxX_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBoxX_1 = value;
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

	internal virtual ButtonX C4
	{
		[DebuggerNonUserCode]
		get
		{
			return _C4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX6_Click;
			if (_C4 != null)
			{
				_C4.Click -= value2;
			}
			_C4 = value;
			if (_C4 != null)
			{
				_C4.Click += value2;
			}
		}
	}

	internal virtual TextBoxX TextBoxX_3
	{
		[DebuggerNonUserCode]
		get
		{
			return textBoxX_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBoxX_2 = value;
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

	internal virtual ButtonX C3
	{
		[DebuggerNonUserCode]
		get
		{
			return _C3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX7_Click;
			if (_C3 != null)
			{
				_C3.Click -= value2;
			}
			_C3 = value;
			if (_C3 != null)
			{
				_C3.Click += value2;
			}
		}
	}

	internal virtual TextBoxX TextBoxX_4
	{
		[DebuggerNonUserCode]
		get
		{
			return textBoxX_3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBoxX_3 = value;
		}
	}

	internal virtual LabelX LabelX6
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX6 = value;
		}
	}

	internal virtual ButtonX C6
	{
		[DebuggerNonUserCode]
		get
		{
			return _C6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX8_Click;
			if (_C6 != null)
			{
				_C6.Click -= value2;
			}
			_C6 = value;
			if (_C6 != null)
			{
				_C6.Click += value2;
			}
		}
	}

	internal virtual ButtonX C7
	{
		[DebuggerNonUserCode]
		get
		{
			return _C7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX9_Click;
			if (_C7 != null)
			{
				_C7.Click -= value2;
			}
			_C7 = value;
			if (_C7 != null)
			{
				_C7.Click += value2;
			}
		}
	}

	internal virtual ComboBox ComboBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox1 = value;
		}
	}

	internal virtual LabelX LabelX7
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX7 = value;
		}
	}

	internal virtual ButtonX C8
	{
		[DebuggerNonUserCode]
		get
		{
			return _C8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = C8_Click;
			if (_C8 != null)
			{
				_C8.Click -= value2;
			}
			_C8 = value;
			if (_C8 != null)
			{
				_C8.Click += value2;
			}
		}
	}

	internal virtual TextBoxX TextBoxX_5
	{
		[DebuggerNonUserCode]
		get
		{
			return textBoxX_4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBoxX_4 = value;
		}
	}

	internal virtual LabelX LabelX8
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX8 = value;
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

	[DebuggerNonUserCode]
	static FormConfirmPay()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormConfirmPay()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormConfirmPay_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		PTOTAl = default(decimal);
		PCASH = default(decimal);
		PCREDIT = default(decimal);
		PFREE = default(decimal);
		TRANN = default(decimal);
		WEB = default(decimal);
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormConfirmPay));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.TextBoxX_5 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX8 = new DevComponents.DotNetBar.LabelX();
		this.C8 = new DevComponents.DotNetBar.ButtonX();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.C7 = new DevComponents.DotNetBar.ButtonX();
		this.TextBoxX_4 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX6 = new DevComponents.DotNetBar.LabelX();
		this.C3 = new DevComponents.DotNetBar.ButtonX();
		this.TextBoxX_3 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.C4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.TextBoxX_1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX5 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX4 = new DevComponents.DotNetBar.LabelX();
		this.LabelX7 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.C2 = new DevComponents.DotNetBar.ButtonX();
		this.C1 = new DevComponents.DotNetBar.ButtonX();
		this.C6 = new DevComponents.DotNetBar.ButtonX();
		this.C5 = new DevComponents.DotNetBar.ButtonX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.TextBoxX_5);
		this.PanelEx1.Controls.Add(this.LabelX8);
		this.PanelEx1.Controls.Add(this.C8);
		this.PanelEx1.Controls.Add(this.ComboBox1);
		this.PanelEx1.Controls.Add(this.C7);
		this.PanelEx1.Controls.Add(this.TextBoxX_4);
		this.PanelEx1.Controls.Add(this.LabelX6);
		this.PanelEx1.Controls.Add(this.C3);
		this.PanelEx1.Controls.Add(this.TextBoxX_3);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Controls.Add(this.C4);
		this.PanelEx1.Controls.Add(this.ButtonX5);
		this.PanelEx1.Controls.Add(this.ButtonX4);
		this.PanelEx1.Controls.Add(this.TextBoxX_0);
		this.PanelEx1.Controls.Add(this.TextBoxX_1);
		this.PanelEx1.Controls.Add(this.LabelX5);
		this.PanelEx1.Controls.Add(this.TextBoxX_2);
		this.PanelEx1.Controls.Add(this.LabelX4);
		this.PanelEx1.Controls.Add(this.LabelX7);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.C2);
		this.PanelEx1.Controls.Add(this.C1);
		this.PanelEx1.Controls.Add(this.C6);
		this.PanelEx1.Controls.Add(this.C5);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(974, 747);
		panelEx3.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.Color = System.Drawing.Color.LightYellow;
		this.PanelEx1.Style.BackColor2.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.TextBoxX_5.BackColor = System.Drawing.Color.LavenderBlush;
		this.TextBoxX_5.Border.Class = "TextBoxBorder";
		this.TextBoxX_5.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_5;
		location = new System.Drawing.Point(239, 678);
		textBoxX_.Location = location;
		this.TextBoxX_5.Name = "Tเว\u0e47ป";
		this.TextBoxX_5.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_5;
		size = new System.Drawing.Size(231, 50);
		textBoxX_2.Size = size;
		this.TextBoxX_5.TabIndex = 23;
		this.TextBoxX_5.Text = "0.00";
		this.TextBoxX_5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX8.BackgroundStyle.Class = "";
		this.LabelX8.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX = this.LabelX8;
		location = new System.Drawing.Point(22, 677);
		labelX.Location = location;
		this.LabelX8.Name = "LabelX8";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX8;
		size = new System.Drawing.Size(211, 56);
		labelX2.Size = size;
		this.LabelX8.TabIndex = 22;
		this.LabelX8.Text = "เว\u0e47บไซต\u0e4c";
		this.C8.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C8.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C8.Font = new System.Drawing.Font("Tahoma", 18.75f);
		this.C8.Image = (System.Drawing.Image)resources.GetObject("C8.Image");
		this.C8.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.C8.ImageTextSpacing = -7;
		DevComponents.DotNetBar.ButtonX c = this.C8;
		location = new System.Drawing.Point(728, 219);
		c.Location = location;
		this.C8.Name = "C8";
		DevComponents.DotNetBar.ButtonX c2 = this.C8;
		size = new System.Drawing.Size(229, 150);
		c2.Size = size;
		this.C8.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C8.TabIndex = 21;
		this.C8.Text = "เว\u0e47บไซต\u0e4c";
		this.ComboBox1.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(668, 449);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(234, 50);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 20;
		this.C7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C7.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C7.Font = new System.Drawing.Font("Tahoma", 20.75f);
		this.C7.Image = (System.Drawing.Image)resources.GetObject("C7.Image");
		this.C7.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c3 = this.C7;
		location = new System.Drawing.Point(494, 219);
		c3.Location = location;
		this.C7.Name = "C7";
		DevComponents.DotNetBar.ButtonX c4 = this.C7;
		size = new System.Drawing.Size(229, 150);
		c4.Size = size;
		this.C7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C7.TabIndex = 19;
		this.C7.Text = "บ\u0e31ตรเครด\u0e34ต+โอนเง\u0e34น";
		this.TextBoxX_4.BackColor = System.Drawing.Color.LavenderBlush;
		this.TextBoxX_4.Border.Class = "TextBoxBorder";
		this.TextBoxX_4.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX = this.TextBoxX_4;
		location = new System.Drawing.Point(239, 506);
		textBoxX.Location = location;
		this.TextBoxX_4.Name = "Tโอน";
		this.TextBoxX_4.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX2 = this.TextBoxX_4;
		size = new System.Drawing.Size(231, 50);
		textBoxX2.Size = size;
		this.TextBoxX_4.TabIndex = 18;
		this.TextBoxX_4.Text = "0.00";
		this.TextBoxX_4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX6.BackgroundStyle.Class = "";
		this.LabelX6.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX6;
		location = new System.Drawing.Point(22, 505);
		labelX3.Location = location;
		this.LabelX6.Name = "LabelX6";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX6;
		size = new System.Drawing.Size(211, 56);
		labelX4.Size = size;
		this.LabelX6.TabIndex = 17;
		this.LabelX6.Text = "โอนเง\u0e34น";
		this.C3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C3.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C3.Font = new System.Drawing.Font("Tahoma", 21.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.C3.Image = (System.Drawing.Image)resources.GetObject("C3.Image");
		this.C3.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c5 = this.C3;
		location = new System.Drawing.Point(494, 63);
		c5.Location = location;
		this.C3.Name = "C3";
		DevComponents.DotNetBar.ButtonX c6 = this.C3;
		size = new System.Drawing.Size(229, 150);
		c6.Size = size;
		this.C3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C3.TabIndex = 16;
		this.C3.Text = "เง\u0e34นสด+โอนเง\u0e34น";
		this.TextBoxX_3.BackColor = System.Drawing.Color.LavenderBlush;
		this.TextBoxX_3.Border.Class = "TextBoxBorder";
		this.TextBoxX_3.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX3 = this.TextBoxX_3;
		location = new System.Drawing.Point(239, 449);
		textBoxX3.Location = location;
		this.TextBoxX_3.Name = "Tฟร\u0e35";
		this.TextBoxX_3.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX4 = this.TextBoxX_3;
		size = new System.Drawing.Size(231, 50);
		textBoxX4.Size = size;
		this.TextBoxX_3.TabIndex = 15;
		this.TextBoxX_3.Text = "0.00";
		this.TextBoxX_3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX3.BackgroundStyle.Class = "";
		this.LabelX3.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX3;
		location = new System.Drawing.Point(22, 448);
		labelX5.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX3;
		size = new System.Drawing.Size(211, 56);
		labelX6.Size = size;
		this.LabelX3.TabIndex = 14;
		this.LabelX3.Text = "คอมม\u0e34ชช\u0e31\u0e48น";
		this.C4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C4.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C4.Font = new System.Drawing.Font("Tahoma", 24f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.C4.Image = (System.Drawing.Image)resources.GetObject("C4.Image");
		this.C4.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c7 = this.C4;
		location = new System.Drawing.Point(728, 63);
		c7.Location = location;
		this.C4.Name = "C4";
		DevComponents.DotNetBar.ButtonX c8 = this.C4;
		size = new System.Drawing.Size(229, 150);
		c8.Size = size;
		this.C4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C4.TabIndex = 13;
		this.C4.Text = "ฟร\u0e35 คอมม\u0e34ชช\u0e31\u0e48น";
		this.C4.Tooltip = "คอมม\u0e34ชช\u0e31\u0e48น";
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		this.ButtonX5.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX5;
		location = new System.Drawing.Point(741, 548);
		buttonX.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX5;
		size = new System.Drawing.Size(161, 101);
		buttonX2.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 12;
		this.ButtonX5.Text = "ยกเล\u0e34ก";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		this.ButtonX4.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX4;
		location = new System.Drawing.Point(550, 548);
		buttonX3.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX4;
		size = new System.Drawing.Size(161, 101);
		buttonX4.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 11;
		this.ButtonX4.Text = "ตกลง";
		this.TextBoxX_0.BackColor = System.Drawing.Color.Black;
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		this.TextBoxX_0.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBoxX_0.ForeColor = System.Drawing.Color.Lime;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX5 = this.TextBoxX_0;
		location = new System.Drawing.Point(239, 391);
		textBoxX5.Location = location;
		this.TextBoxX_0.Name = "TextBoxX3";
		this.TextBoxX_0.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX6 = this.TextBoxX_0;
		size = new System.Drawing.Size(231, 50);
		textBoxX6.Size = size;
		this.TextBoxX_0.TabIndex = 10;
		this.TextBoxX_0.Text = "0.00";
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.TextBoxX_1.BackColor = System.Drawing.Color.LavenderBlush;
		this.TextBoxX_1.Border.Class = "TextBoxBorder";
		this.TextBoxX_1.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX7 = this.TextBoxX_1;
		location = new System.Drawing.Point(239, 621);
		textBoxX7.Location = location;
		this.TextBoxX_1.Name = "Tบ\u0e31ตร";
		this.TextBoxX_1.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX8 = this.TextBoxX_1;
		size = new System.Drawing.Size(231, 50);
		textBoxX8.Size = size;
		this.TextBoxX_1.TabIndex = 9;
		this.TextBoxX_1.Text = "0.00";
		this.TextBoxX_1.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX5.BackgroundStyle.Class = "";
		this.LabelX5.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX7 = this.LabelX5;
		location = new System.Drawing.Point(22, 620);
		labelX7.Location = location;
		this.LabelX5.Name = "LabelX5";
		DevComponents.DotNetBar.LabelX labelX8 = this.LabelX5;
		size = new System.Drawing.Size(211, 56);
		labelX8.Size = size;
		this.LabelX5.TabIndex = 8;
		this.LabelX5.Text = "จ\u0e48ายบ\u0e31ตร";
		this.TextBoxX_2.BackColor = System.Drawing.Color.LavenderBlush;
		this.TextBoxX_2.Border.Class = "TextBoxBorder";
		this.TextBoxX_2.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX9 = this.TextBoxX_2;
		location = new System.Drawing.Point(239, 564);
		textBoxX9.Location = location;
		this.TextBoxX_2.Name = "Tสด";
		this.TextBoxX_2.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX10 = this.TextBoxX_2;
		size = new System.Drawing.Size(231, 50);
		textBoxX10.Size = size;
		this.TextBoxX_2.TabIndex = 7;
		this.TextBoxX_2.Text = "0.00";
		this.TextBoxX_2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX4.BackgroundStyle.Class = "";
		this.LabelX4.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX9 = this.LabelX4;
		location = new System.Drawing.Point(22, 563);
		labelX9.Location = location;
		this.LabelX4.Name = "LabelX4";
		DevComponents.DotNetBar.LabelX labelX10 = this.LabelX4;
		size = new System.Drawing.Size(211, 56);
		labelX10.Size = size;
		this.LabelX4.TabIndex = 6;
		this.LabelX4.Text = "จ\u0e48ายเง\u0e34นสด";
		this.LabelX7.BackgroundStyle.Class = "";
		this.LabelX7.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX11 = this.LabelX7;
		location = new System.Drawing.Point(552, 445);
		labelX11.Location = location;
		this.LabelX7.Name = "LabelX7";
		DevComponents.DotNetBar.LabelX labelX12 = this.LabelX7;
		size = new System.Drawing.Size(211, 56);
		labelX12.Size = size;
		this.LabelX7.TabIndex = 4;
		this.LabelX7.Text = "สาขา";
		this.LabelX2.BackgroundStyle.Class = "";
		this.LabelX2.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX13 = this.LabelX2;
		location = new System.Drawing.Point(22, 385);
		labelX13.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX14 = this.LabelX2;
		size = new System.Drawing.Size(211, 56);
		labelX14.Size = size;
		this.LabelX2.TabIndex = 4;
		this.LabelX2.Text = "ยอดเง\u0e34นจ\u0e48าย";
		this.C2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C2.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C2.Font = new System.Drawing.Font("Tahoma", 21.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.C2.Image = (System.Drawing.Image)resources.GetObject("C2.Image");
		this.C2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c9 = this.C2;
		location = new System.Drawing.Point(259, 63);
		c9.Location = location;
		this.C2.Name = "C2";
		DevComponents.DotNetBar.ButtonX c10 = this.C2;
		size = new System.Drawing.Size(229, 150);
		c10.Size = size;
		this.C2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C2.TabIndex = 3;
		this.C2.Text = "เง\u0e34นสด+บ\u0e31ตรเครด\u0e34ต";
		this.C1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C1.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C1.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.C1.Image = (System.Drawing.Image)resources.GetObject("C1.Image");
		this.C1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c11 = this.C1;
		location = new System.Drawing.Point(24, 63);
		c11.Location = location;
		this.C1.Name = "C1";
		DevComponents.DotNetBar.ButtonX c12 = this.C1;
		size = new System.Drawing.Size(229, 150);
		c12.Size = size;
		this.C1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C1.TabIndex = 2;
		this.C1.Text = "เง\u0e34นสด";
		this.C6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C6.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C6.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.C6.Image = (System.Drawing.Image)resources.GetObject("C6.Image");
		this.C6.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c13 = this.C6;
		location = new System.Drawing.Point(259, 219);
		c13.Location = location;
		this.C6.Name = "C6";
		DevComponents.DotNetBar.ButtonX c14 = this.C6;
		size = new System.Drawing.Size(229, 150);
		c14.Size = size;
		this.C6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C6.TabIndex = 1;
		this.C6.Text = "โอนเง\u0e34น";
		this.C5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.C5.ColorTable = DevComponents.DotNetBar.eButtonColor.MagentaWithBackground;
		this.C5.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.C5.Image = (System.Drawing.Image)resources.GetObject("C5.Image");
		this.C5.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX c15 = this.C5;
		location = new System.Drawing.Point(24, 219);
		c15.Location = location;
		this.C5.Name = "C5";
		DevComponents.DotNetBar.ButtonX c16 = this.C5;
		size = new System.Drawing.Size(229, 150);
		c16.Size = size;
		this.C5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.C5.TabIndex = 1;
		this.C5.Text = "บ\u0e31ตรเครด\u0e34ต";
		this.LabelX1.BackgroundStyle.Class = "";
		this.LabelX1.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX15 = this.LabelX1;
		location = new System.Drawing.Point(24, 7);
		labelX15.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX16 = this.LabelX1;
		size = new System.Drawing.Size(364, 56);
		labelX16.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ประเภทการชำระเง\u0e34น";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(974, 747);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FormConfirmPay";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.TopMost = true;
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void FormConfirmPay_Load(object sender, EventArgs e)
	{
		uncheck();
		Size size = new Size(990, 405);
		Size = size;
		TextBoxX_0.Text = Strings.Format(PTOTAl, "#,##0.00");
		TextBoxX_1.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_2.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_4.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
		ComboBox1.Items.Clear();
		DataSet dataSet = Module1.connect("select name from TB_SET_Branch order by id");
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
				ComboBox1.Items.Add(dataSet.Tables[0].Rows[num2]["name"].ToString());
				num2++;
			}
			if (ComboBox1.Items.Count != 0)
			{
				ComboBox1.SelectedIndex = 0;
			}
			ISOK = false;
			Timer1.Enabled = true;
		}
	}

	public void uncheck()
	{
		C1.Checked = false;
		C2.Checked = false;
		C3.Checked = false;
		C4.Checked = false;
		C5.Checked = false;
		C6.Checked = false;
		C7.Checked = false;
		C8.Checked = false;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		uncheck();
		C5.Checked = true;
		TextBoxX_1.Text = Strings.Format(PTOTAl, "#,##0.00");
		TextBoxX_2.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_4.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
		Size size = new Size(990, 763);
		Size = size;
		Module1.smethod_3();
		ButtonX4.Focus();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		uncheck();
		C1.Checked = true;
		TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_1.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_2.Text = Strings.Format(PTOTAl, "#,##0.00");
		TextBoxX_4.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
		Size size = new Size(990, 763);
		Size = size;
		Module1.smethod_3();
		ButtonX4.Focus();
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		TopMost = false;
		object obj = Interaction.InputBox("กร\u0e38ณากรอกจำนวนเง\u0e34นบ\u0e31ตรเครด\u0e34ตท\u0e35\u0e48จะจ\u0e48าย", "กร\u0e38ณากรอกจำนวนเง\u0e34นบ\u0e31ตรเครด\u0e34ตท\u0e35\u0e48จะจ\u0e48าย", Conversions.ToString(PTOTAl));
		TopMost = true;
		if (!Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
			{
				MessageBox.Show("กร\u0e38ณากรอกให\u0e49เป\u0e47นต\u0e31วเลข", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (decimal.Compare(Conversions.ToDecimal(obj), Conversions.ToDecimal(TextBoxX_0.Text)) > 0)
			{
				MessageBox.Show("จำนวนเง\u0e34นบ\u0e31ตรเครด\u0e34ตเก\u0e34นกว\u0e48าท\u0e35\u0e48จะจ\u0e48าย", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			uncheck();
			C2.Checked = true;
			TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
			TextBoxX_1.Text = Strings.Format(Conversions.ToDecimal(obj), "#,##0.00");
			TextBoxX_4.Text = Strings.Format(0m, "#,##0.00");
			TextBoxX_2.Text = Strings.Format(decimal.Subtract(PTOTAl, Conversions.ToDecimal(obj)), "#,##0.00");
			TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
			Size size = new Size(990, 763);
			Size = size;
			Module1.smethod_3();
			ButtonX4.Focus();
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		PCASH = Conversions.ToDecimal(TextBoxX_2.Text);
		PCREDIT = Conversions.ToDecimal(TextBoxX_1.Text);
		PFREE = Conversions.ToDecimal(TextBoxX_3.Text);
		TRANN = Conversions.ToDecimal(TextBoxX_4.Text);
		WEB = Conversions.ToDecimal(TextBoxX_5.Text);
		if ((decimal.Compare(PCASH, 0m) == 0) & (decimal.Compare(PCREDIT, 0m) == 0) & (decimal.Compare(PFREE, 0m) == 0) & (decimal.Compare(TRANN, 0m) == 0) & (decimal.Compare(WEB, 0m) == 0))
		{
			MessageBox.Show("กร\u0e38ณาตรวจสอบยอดชำระเง\u0e34น", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			return;
		}
		ISOK = true;
		Close();
	}

	private void PanelEx1_Click(object sender, EventArgs e)
	{
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		ISOK = false;
		Close();
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		uncheck();
		C4.Checked = true;
		TextBoxX_3.Text = Strings.Format(PTOTAl, "#,##0.00");
		TextBoxX_1.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_2.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_4.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
		Size size = new Size(990, 763);
		Size = size;
		Module1.smethod_3();
		ButtonX4.Focus();
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		TopMost = false;
		object obj = Interaction.InputBox("กร\u0e38ณากรอกจำนวนเง\u0e34นโอน", "กร\u0e38ณากรอกจำนวนเง\u0e34นโอน", Conversions.ToString(PTOTAl));
		TopMost = true;
		if (!Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
			{
				MessageBox.Show("กร\u0e38ณากรอกให\u0e49เป\u0e47นต\u0e31วเลข", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (decimal.Compare(Conversions.ToDecimal(obj), Conversions.ToDecimal(TextBoxX_0.Text)) > 0)
			{
				MessageBox.Show("จำนวนเง\u0e34นโอนเก\u0e34นกว\u0e48าท\u0e35\u0e48จะจ\u0e48าย", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			uncheck();
			C3.Checked = true;
			TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
			TextBoxX_1.Text = Strings.Format(0m, "#,##0.00");
			TextBoxX_4.Text = Strings.Format(Conversions.ToDecimal(obj), "#,##0.00");
			TextBoxX_2.Text = Strings.Format(decimal.Subtract(PTOTAl, Conversions.ToDecimal(obj)), "#,##0.00");
			TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
			Size size = new Size(990, 763);
			Size = size;
			Module1.smethod_3();
			ButtonX4.Focus();
		}
	}

	private void ButtonX8_Click(object sender, EventArgs e)
	{
		uncheck();
		C6.Checked = true;
		TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_1.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_4.Text = Strings.Format(PTOTAl, "#,##0.00");
		TextBoxX_2.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
		Size size = new Size(990, 763);
		Size = size;
		Module1.smethod_3();
		ButtonX4.Focus();
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		TopMost = false;
		object obj = Interaction.InputBox("กร\u0e38ณากรอกจำนวนเง\u0e34นโอน", "กร\u0e38ณากรอกจำนวนเง\u0e34นโอน", Conversions.ToString(PTOTAl));
		TopMost = true;
		if (!Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
			{
				MessageBox.Show("กร\u0e38ณากรอกให\u0e49เป\u0e47นต\u0e31วเลข", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (decimal.Compare(Conversions.ToDecimal(obj), Conversions.ToDecimal(TextBoxX_0.Text)) > 0)
			{
				MessageBox.Show("จำนวนเง\u0e34นโอนเก\u0e34นกว\u0e48าท\u0e35\u0e48จะจ\u0e48าย", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			uncheck();
			C7.Checked = true;
			TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
			TextBoxX_1.Text = Strings.Format(decimal.Subtract(PTOTAl, Conversions.ToDecimal(obj)), "#,##0.00");
			TextBoxX_4.Text = Strings.Format(Conversions.ToDecimal(obj), "#,##0.00");
			TextBoxX_2.Text = Strings.Format(0m, "#,##0.00");
			TextBoxX_5.Text = Strings.Format(0m, "#,##0.00");
			Size size = new Size(990, 763);
			Size = size;
			ButtonX4.Focus();
		}
	}

	private void LabelX1_Click(object sender, EventArgs e)
	{
	}

	private void C8_Click(object sender, EventArgs e)
	{
		uncheck();
		C8.Checked = true;
		TextBoxX_3.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_1.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_4.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_2.Text = Strings.Format(0m, "#,##0.00");
		TextBoxX_5.Text = Strings.Format(PTOTAl, "#,##0.00");
		Size size = new Size(990, 763);
		Size = size;
		Module1.smethod_3();
		ButtonX4.Focus();
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		C1.Focus();
	}
}
