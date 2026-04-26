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
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormConfirmRoundBill : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("LabelName")]
	private LabelX _LabelName;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("TextBoxX1")]
	private TextBoxX _TextBoxX1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	public bool ISOK;

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

	internal virtual LabelX LabelName
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelName;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelName = value;
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

	internal virtual TextBoxX TextBoxX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBoxX1 = value;
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
	static FormConfirmRoundBill()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormConfirmRoundBill()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FormConfirmRoundBill_FormClosing;
		base.Load += FormConfirmRoundBill_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
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
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelName = new DevComponents.DotNetBar.LabelX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX1;
		System.Drawing.Point location = new System.Drawing.Point(17, 13);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		System.Drawing.Size size = new System.Drawing.Size(308, 34);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ค\u0e38ณต\u0e49องการเป\u0e34ดรอบบ\u0e34ลหร\u0e37อไม\u0e48";
		this.LabelX2.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX2;
		location = new System.Drawing.Point(17, 65);
		labelX3.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX2;
		size = new System.Drawing.Size(102, 34);
		labelX4.Size = size;
		this.LabelX2.TabIndex = 1;
		this.LabelX2.Text = "ผ\u0e39\u0e49เป\u0e34ดรอบบ\u0e34ล";
		this.LabelName.BackgroundStyle.Class = "";
		this.LabelName.Font = new System.Drawing.Font("Tahoma", 26.25f, System.Drawing.FontStyle.Underline, System.Drawing.GraphicsUnit.Point, 222);
		this.LabelName.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		DevComponents.DotNetBar.LabelX labelName = this.LabelName;
		location = new System.Drawing.Point(126, 54);
		labelName.Location = location;
		this.LabelName.Name = "LabelName";
		DevComponents.DotNetBar.LabelX labelName2 = this.LabelName;
		size = new System.Drawing.Size(187, 46);
		labelName2.Size = size;
		this.LabelName.TabIndex = 2;
		this.LabelName.Text = "OPEN";
		this.LabelName.TextAlignment = System.Drawing.StringAlignment.Center;
		this.LabelX3.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX3;
		location = new System.Drawing.Point(17, 124);
		labelX5.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX3;
		size = new System.Drawing.Size(158, 34);
		labelX6.Size = size;
		this.LabelX3.TabIndex = 3;
		this.LabelX3.Text = "จำนวเง\u0e34นในล\u0e34\u0e49นช\u0e31ก";
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		this.TextBoxX_0.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_0;
		location = new System.Drawing.Point(165, 118);
		textBoxX_.Location = location;
		this.TextBoxX_0.Name = "TextBoxX1";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_0;
		size = new System.Drawing.Size(209, 40);
		textBoxX_2.Size = size;
		this.TextBoxX_0.TabIndex = 4;
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(165, 171);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		size = new System.Drawing.Size(102, 44);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 5;
		this.ButtonX1.Text = "ตกลง";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(272, 171);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(102, 44);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 6;
		this.ButtonX2.Text = "ยกเล\u0e34ก";
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.ButtonX3);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.LabelName);
		this.PanelEx1.Controls.Add(this.TextBoxX_0);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(386, 231);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 7;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(319, 59);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(55, 37);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 7;
		this.ButtonX3.Text = "เปล\u0e35\u0e48ยน";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(10f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(386, 231);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5);
		this.Margin = margin;
		this.Name = "FormConfirmRoundBill";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ย\u0e37นย\u0e31นการเป\u0e34ดรอบ";
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void FormConfirmRoundBill_FormClosing(object sender, FormClosingEventArgs e)
	{
		TextBoxX_0.Focus();
	}

	private void FormConfirmRoundBill_Load(object sender, EventArgs e)
	{
		ISOK = false;
		TextBoxX_0.Focus();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		ISOK = false;
		Close();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		ISOK = true;
		Close();
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		MyProject.Forms.login.User.Text = "";
		MyProject.Forms.login.Pass.Text = "";
		MyProject.Forms.login.User.Focus();
		MyProject.Forms.login.ShowDialog();
		LabelName.Text = Conversions.ToString(Module1.loginName);
	}
}
