using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormEditPay : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("TextBox_ฟร\u0e35")]
	private TextBoxX textBoxX_0;

	[AccessedThroughProperty("LabelX7")]
	private LabelX _LabelX7;

	[AccessedThroughProperty("TextBox_โอน")]
	private TextBoxX textBoxX_1;

	[AccessedThroughProperty("LabelX6")]
	private LabelX _LabelX6;

	[AccessedThroughProperty("TextBox_บ\u0e31ตร")]
	private TextBoxX textBoxX_2;

	[AccessedThroughProperty("LabelX5")]
	private LabelX _LabelX5;

	[AccessedThroughProperty("TextBox_สด")]
	private TextBoxX textBoxX_3;

	[AccessedThroughProperty("LabelX4")]
	private LabelX _LabelX4;

	[AccessedThroughProperty("Label_price")]
	private LabelX _Label_price;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("Label_num")]
	private LabelX _Label_num;

	[AccessedThroughProperty("LabelX8")]
	private LabelX _LabelX8;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("TextBox_เว\u0e47บ")]
	private TextBoxX textBoxX_4;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	public bool isok;

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

	internal virtual TextBoxX TextBoxX_0
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
			EventHandler value2 = method_0;
			EventHandler value3 = method_1;
			if (textBoxX_0 != null)
			{
				textBoxX_0.LostFocus -= value2;
				textBoxX_0.TextChanged -= value3;
			}
			textBoxX_0 = value;
			if (textBoxX_0 != null)
			{
				textBoxX_0.LostFocus += value2;
				textBoxX_0.TextChanged += value3;
			}
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

	internal virtual TextBoxX TextBoxX_1
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
			EventHandler value2 = TextBoxX_1_LostFocus;
			if (textBoxX_1 != null)
			{
				textBoxX_1.LostFocus -= value2;
			}
			textBoxX_1 = value;
			if (textBoxX_1 != null)
			{
				textBoxX_1.LostFocus += value2;
			}
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

	internal virtual TextBoxX TextBoxX_2
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
			EventHandler value2 = TextBoxX_2_LostFocus;
			if (textBoxX_2 != null)
			{
				textBoxX_2.LostFocus -= value2;
			}
			textBoxX_2 = value;
			if (textBoxX_2 != null)
			{
				textBoxX_2.LostFocus += value2;
			}
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

	internal virtual TextBoxX TextBoxX_3
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
			EventHandler value2 = TextBoxX_3_LostFocus;
			if (textBoxX_3 != null)
			{
				textBoxX_3.LostFocus -= value2;
			}
			textBoxX_3 = value;
			if (textBoxX_3 != null)
			{
				textBoxX_3.LostFocus += value2;
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

	internal virtual LabelX Label_price
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_price;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_price = value;
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

	internal virtual LabelX Label_num
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_num;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_num = value;
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

	internal virtual TextBoxX TextBoxX_4
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
			EventHandler value2 = method_3;
			EventHandler value3 = method_2;
			if (textBoxX_4 != null)
			{
				textBoxX_4.TextChanged -= value2;
				textBoxX_4.LostFocus -= value3;
			}
			textBoxX_4 = value;
			if (textBoxX_4 != null)
			{
				textBoxX_4.TextChanged += value2;
				textBoxX_4.LostFocus += value3;
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

	[DebuggerNonUserCode]
	static FormEditPay()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormEditPay()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormEditPay_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		isok = false;
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
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label_num = new DevComponents.DotNetBar.LabelX();
		this.LabelX8 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX7 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX6 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX5 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_3 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX4 = new DevComponents.DotNetBar.LabelX();
		this.Label_price = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_4 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.TextBoxX_4);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.Label_num);
		this.PanelEx1.Controls.Add(this.LabelX8);
		this.PanelEx1.Controls.Add(this.TextBoxX_0);
		this.PanelEx1.Controls.Add(this.LabelX7);
		this.PanelEx1.Controls.Add(this.TextBoxX_1);
		this.PanelEx1.Controls.Add(this.LabelX6);
		this.PanelEx1.Controls.Add(this.TextBoxX_2);
		this.PanelEx1.Controls.Add(this.LabelX5);
		this.PanelEx1.Controls.Add(this.TextBoxX_3);
		this.PanelEx1.Controls.Add(this.LabelX4);
		this.PanelEx1.Controls.Add(this.Label_price);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(312, 412);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(160, 329);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(129, 56);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 8;
		this.ButtonX2.Text = "ยกเล\u0e34ก";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		location = new System.Drawing.Point(16, 329);
		buttonX3.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		size = new System.Drawing.Size(138, 56);
		buttonX4.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 7;
		this.ButtonX1.Text = "แก\u0e49ไข";
		this.Label_num.BackgroundStyle.Class = "";
		this.Label_num.ForeColor = System.Drawing.Color.Blue;
		DevComponents.DotNetBar.LabelX label_num = this.Label_num;
		location = new System.Drawing.Point(150, 50);
		label_num.Location = location;
		this.Label_num.Name = "Label_num";
		DevComponents.DotNetBar.LabelX label_num2 = this.Label_num;
		size = new System.Drawing.Size(139, 33);
		label_num2.Size = size;
		this.Label_num.TabIndex = 6;
		this.Label_num.Text = "เลขท\u0e35\u0e48";
		this.Label_num.TextAlignment = System.Drawing.StringAlignment.Far;
		this.LabelX8.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX8;
		location = new System.Drawing.Point(16, 50);
		labelX.Location = location;
		this.LabelX8.Name = "LabelX8";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX8;
		size = new System.Drawing.Size(139, 33);
		labelX2.Size = size;
		this.LabelX8.TabIndex = 5;
		this.LabelX8.Text = "เลขท\u0e35\u0e48";
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX = this.TextBoxX_0;
		location = new System.Drawing.Point(150, 248);
		textBoxX.Location = location;
		this.TextBoxX_0.Name = "TextBox_ฟร\u0e35";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX2 = this.TextBoxX_0;
		size = new System.Drawing.Size(139, 30);
		textBoxX2.Size = size;
		this.TextBoxX_0.TabIndex = 4;
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX7.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX7;
		location = new System.Drawing.Point(16, 248);
		labelX3.Location = location;
		this.LabelX7.Name = "LabelX7";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX7;
		size = new System.Drawing.Size(128, 33);
		labelX4.Size = size;
		this.LabelX7.TabIndex = 3;
		this.LabelX7.Text = "ฟร\u0e35";
		this.TextBoxX_1.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX3 = this.TextBoxX_1;
		location = new System.Drawing.Point(150, 212);
		textBoxX3.Location = location;
		this.TextBoxX_1.Name = "TextBox_โอน";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX4 = this.TextBoxX_1;
		size = new System.Drawing.Size(139, 30);
		textBoxX4.Size = size;
		this.TextBoxX_1.TabIndex = 4;
		this.TextBoxX_1.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX6.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX6;
		location = new System.Drawing.Point(16, 212);
		labelX5.Location = location;
		this.LabelX6.Name = "LabelX6";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX6;
		size = new System.Drawing.Size(128, 33);
		labelX6.Size = size;
		this.LabelX6.TabIndex = 3;
		this.LabelX6.Text = "โอนเง\u0e34น";
		this.TextBoxX_2.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX5 = this.TextBoxX_2;
		location = new System.Drawing.Point(150, 176);
		textBoxX5.Location = location;
		this.TextBoxX_2.Name = "TextBox_บ\u0e31ตร";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX6 = this.TextBoxX_2;
		size = new System.Drawing.Size(139, 30);
		textBoxX6.Size = size;
		this.TextBoxX_2.TabIndex = 4;
		this.TextBoxX_2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX5.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX7 = this.LabelX5;
		location = new System.Drawing.Point(16, 176);
		labelX7.Location = location;
		this.LabelX5.Name = "LabelX5";
		DevComponents.DotNetBar.LabelX labelX8 = this.LabelX5;
		size = new System.Drawing.Size(128, 33);
		labelX8.Size = size;
		this.LabelX5.TabIndex = 3;
		this.LabelX5.Text = "บ\u0e31ตรเครด\u0e34ต";
		this.TextBoxX_3.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX7 = this.TextBoxX_3;
		location = new System.Drawing.Point(150, 140);
		textBoxX7.Location = location;
		this.TextBoxX_3.Name = "TextBox_สด";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX8 = this.TextBoxX_3;
		size = new System.Drawing.Size(139, 30);
		textBoxX8.Size = size;
		this.TextBoxX_3.TabIndex = 4;
		this.TextBoxX_3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX4.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX9 = this.LabelX4;
		location = new System.Drawing.Point(16, 140);
		labelX9.Location = location;
		this.LabelX4.Name = "LabelX4";
		DevComponents.DotNetBar.LabelX labelX10 = this.LabelX4;
		size = new System.Drawing.Size(128, 33);
		labelX10.Size = size;
		this.LabelX4.TabIndex = 3;
		this.LabelX4.Text = "เง\u0e34นสด";
		this.Label_price.BackgroundStyle.Class = "";
		this.Label_price.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Underline, System.Drawing.GraphicsUnit.Point, 222);
		this.Label_price.ForeColor = System.Drawing.Color.Red;
		DevComponents.DotNetBar.LabelX label_price = this.Label_price;
		location = new System.Drawing.Point(150, 89);
		label_price.Location = location;
		this.Label_price.Name = "Label_price";
		DevComponents.DotNetBar.LabelX label_price2 = this.Label_price;
		size = new System.Drawing.Size(139, 33);
		label_price2.Size = size;
		this.Label_price.TabIndex = 2;
		this.Label_price.Text = "1,000,000.00";
		this.Label_price.TextAlignment = System.Drawing.StringAlignment.Far;
		this.LabelX2.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX11 = this.LabelX2;
		location = new System.Drawing.Point(16, 89);
		labelX11.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX12 = this.LabelX2;
		size = new System.Drawing.Size(139, 33);
		labelX12.Size = size;
		this.LabelX2.TabIndex = 1;
		this.LabelX2.Text = "จำนวนเง\u0e34นท\u0e35\u0e48จ\u0e48าย";
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX13 = this.LabelX1;
		location = new System.Drawing.Point(16, 9);
		labelX13.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX14 = this.LabelX1;
		size = new System.Drawing.Size(247, 33);
		labelX14.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "แก\u0e49ไขรายการใบสำค\u0e31ญร\u0e31บเง\u0e34น";
		this.TextBoxX_4.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX9 = this.TextBoxX_4;
		location = new System.Drawing.Point(150, 284);
		textBoxX9.Location = location;
		this.TextBoxX_4.Name = "TextBox_เว\u0e47บ";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX10 = this.TextBoxX_4;
		size = new System.Drawing.Size(139, 30);
		textBoxX10.Size = size;
		this.TextBoxX_4.TabIndex = 10;
		this.TextBoxX_4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.LabelX3.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX15 = this.LabelX3;
		location = new System.Drawing.Point(16, 284);
		labelX15.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX16 = this.LabelX3;
		size = new System.Drawing.Size(128, 33);
		labelX16.Size = size;
		this.LabelX3.TabIndex = 9;
		this.LabelX3.Text = "เว\u0e47บไซต\u0e4c";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(10f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(312, 412);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5);
		this.Margin = margin;
		this.Name = "FormEditPay";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "แก\u0e49ไขใบสำค\u0e31ญร\u0e31บเง\u0e34น";
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void TextBoxX_3_LostFocus(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_3.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_3.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBoxX_3.Text))
		{
			TextBoxX_3.Text = Conversions.ToString(0);
		}
	}

	private void TextBoxX_2_LostFocus(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_2.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_2.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBoxX_2.Text))
		{
			TextBoxX_2.Text = Conversions.ToString(0);
		}
	}

	private void TextBoxX_1_LostFocus(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_1.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_1.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBoxX_1.Text))
		{
			TextBoxX_1.Text = Conversions.ToString(0);
		}
	}

	private void method_0(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_0.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_0.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBoxX_0.Text))
		{
			TextBoxX_0.Text = Conversions.ToString(0);
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		if (decimal.Compare(Conversions.ToDecimal(Label_price.Text), decimal.Add(decimal.Add(decimal.Add(decimal.Add(Conversions.ToDecimal(TextBoxX_3.Text), Conversions.ToDecimal(TextBoxX_2.Text)), Conversions.ToDecimal(TextBoxX_0.Text)), Conversions.ToDecimal(TextBoxX_1.Text)), Conversions.ToDecimal(TextBoxX_4.Text))) != 0)
		{
			MessageBox.Show("จำนวนเง\u0e34นจ\u0e48ายไม\u0e48เท\u0e48าก\u0e31น ไม\u0e48สามารถแก\u0e49ไขได\u0e49", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการแก\u0e49ไขใบสำค\u0e31ญจ\u0e48ายหร\u0e37อไม\u0e48", "แก\u0e49ไข", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("update HT_CheckIn_Pay SET Cin_Pay_Cash=" + Conversions.ToString(Conversions.ToDecimal(TextBoxX_3.Text)) + ",Cin_Pay_Credit=" + Conversions.ToString(Conversions.ToDecimal(TextBoxX_2.Text)) + ",Cin_Pay_Free=" + Conversions.ToString(Conversions.ToDecimal(TextBoxX_0.Text)) + ",Cin_Pay_Tran=" + Conversions.ToString(Conversions.ToDecimal(TextBoxX_1.Text)) + ",Cin_Pay_web=" + Conversions.ToString(Conversions.ToDecimal(TextBoxX_4.Text)) + " where Pay_no='" + Label_num.Text + "'");
			isok = true;
			MessageBox.Show("แก\u0e49ไขเร\u0e35ยบร\u0e49อย");
			Close();
		}
	}

	private void FormEditPay_Load(object sender, EventArgs e)
	{
		isok = false;
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		isok = false;
		Close();
	}

	private void method_1(object sender, EventArgs e)
	{
	}

	private void method_2(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_4.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_4.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBoxX_4.Text))
		{
			TextBoxX_4.Text = Conversions.ToString(0);
		}
	}

	private void method_3(object sender, EventArgs e)
	{
	}
}
