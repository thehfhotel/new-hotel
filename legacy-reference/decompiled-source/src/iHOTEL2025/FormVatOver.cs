using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormVatOver : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label_P3")]
	private Label _Label_P3;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label_P2")]
	private Label _Label_P2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label_P1")]
	private Label _Label_P1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

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

	internal virtual Label Label7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label7 = value;
		}
	}

	internal virtual Label Label3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label3 = value;
		}
	}

	internal virtual Label Label4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label4 = value;
		}
	}

	internal virtual Label Label_P3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_P3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_P3 = value;
		}
	}

	internal virtual Label Label5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label5 = value;
		}
	}

	internal virtual Label Label_P2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_P2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_P2 = value;
		}
	}

	internal virtual Label Label2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label2 = value;
		}
	}

	internal virtual Label Label_P1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_P1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_P1 = value;
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

	internal virtual Label Label8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label8 = value;
		}
	}

	[DebuggerNonUserCode]
	static FormVatOver()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormVatOver()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormVatOver_Load;
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
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label_P3 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label_P2 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label_P1 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.Label8);
		this.PanelEx1.Controls.Add(this.Label7);
		this.PanelEx1.Controls.Add(this.Label3);
		this.PanelEx1.Controls.Add(this.Label4);
		this.PanelEx1.Controls.Add(this.Label_P3);
		this.PanelEx1.Controls.Add(this.Label5);
		this.PanelEx1.Controls.Add(this.Label_P2);
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.Label_P1);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(451, 283);
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
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(255, 210);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(129, 54);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 14;
		this.ButtonX2.Text = "ยกเล\u0e34ก";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		location = new System.Drawing.Point(87, 210);
		buttonX3.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		size = new System.Drawing.Size(129, 54);
		buttonX4.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 13;
		this.ButtonX1.Text = "ตกลง";
		this.Label8.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label8.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.Label label = this.Label8;
		location = new System.Drawing.Point(15, 164);
		label.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label2 = this.Label8;
		size = new System.Drawing.Size(369, 23);
		label2.Size = size;
		this.Label8.TabIndex = 12;
		this.Label8.Text = "ค\u0e38ณต\u0e49องการทำรายการออกภาษ\u0e35หร\u0e37อไม\u0e48";
		this.Label8.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label7.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label7.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label3 = this.Label7;
		location = new System.Drawing.Point(12, 110);
		label3.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label4 = this.Label7;
		size = new System.Drawing.Size(224, 23);
		label4.Size = size;
		this.Label7.TabIndex = 11;
		this.Label7.Text = "เป\u0e47นเง\u0e34นท\u0e35\u0e48ต\u0e49องเก\u0e47บเพ\u0e34\u0e48ม";
		this.Label7.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label3.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(12, 67);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(224, 23);
		label6.Size = size;
		this.Label3.TabIndex = 10;
		this.Label3.Text = "ห\u0e31ก";
		this.Label3.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label4.AutoSize = true;
		this.Label4.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label4.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(387, 110);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(47, 23);
		label8.Size = size;
		this.Label4.TabIndex = 9;
		this.Label4.Text = "บาท";
		this.Label_P3.BackColor = System.Drawing.Color.Black;
		this.Label_P3.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label_P3.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label label_P = this.Label_P3;
		location = new System.Drawing.Point(241, 105);
		label_P.Location = location;
		this.Label_P3.Name = "Label_P3";
		System.Windows.Forms.Label label_P2 = this.Label_P3;
		size = new System.Drawing.Size(139, 34);
		label_P2.Size = size;
		this.Label_P3.TabIndex = 8;
		this.Label_P3.Text = "CH-0001";
		this.Label_P3.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label5.AutoSize = true;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label5.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(387, 67);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(33, 23);
		label10.Size = size;
		this.Label5.TabIndex = 6;
		this.Label5.Text = "%";
		this.Label_P2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label_P2.ForeColor = System.Drawing.Color.FromArgb(0, 192, 0);
		System.Windows.Forms.Label label_P3 = this.Label_P2;
		location = new System.Drawing.Point(250, 67);
		label_P3.Location = location;
		this.Label_P2.Name = "Label_P2";
		System.Windows.Forms.Label label_P4 = this.Label_P2;
		size = new System.Drawing.Size(130, 23);
		label_P4.Size = size;
		this.Label_P2.TabIndex = 5;
		this.Label_P2.Text = "CH-0000001";
		this.Label_P2.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label2.AutoSize = true;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label11 = this.Label2;
		location = new System.Drawing.Point(387, 22);
		label11.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label12 = this.Label2;
		size = new System.Drawing.Size(47, 23);
		label12.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "บาท";
		this.Label_P1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label_P1.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label_P5 = this.Label_P1;
		location = new System.Drawing.Point(249, 22);
		label_P5.Location = location;
		this.Label_P1.Name = "Label_P1";
		System.Windows.Forms.Label label_P6 = this.Label_P1;
		size = new System.Drawing.Size(130, 23);
		label_P6.Size = size;
		this.Label_P1.TabIndex = 2;
		this.Label_P1.Text = "CH-0000001";
		this.Label_P1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label13 = this.Label1;
		location = new System.Drawing.Point(12, 22);
		label13.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label14 = this.Label1;
		size = new System.Drawing.Size(224, 23);
		label14.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "จำนวนเง\u0e34นท\u0e35\u0e48ออกภาษ\u0e35เก\u0e34น";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(451, 283);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FormVatOver";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ออกภาษ\u0e35เก\u0e34น";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		ISOK = false;
		Close();
	}

	private void FormVatOver_Load(object sender, EventArgs e)
	{
		ISOK = false;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		ISOK = true;
		Close();
	}
}
