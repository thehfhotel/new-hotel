using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using iHOTEL2025.My;

namespace iHOTEL2025;

public class AlertCustom : Balloon
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("label2")]
	private Label _label2;

	[AccessedThroughProperty("linkLabel1")]
	private LinkLabel _linkLabel1;

	[AccessedThroughProperty("labelX2")]
	private LabelX _labelX2;

	[AccessedThroughProperty("labelX1")]
	private LabelX _labelX1;

	[AccessedThroughProperty("bar1")]
	private Bar _bar1;

	[AccessedThroughProperty("Bar2")]
	private Bar _Bar2;

	[AccessedThroughProperty("ControlContainerItem1")]
	private ControlContainerItem _ControlContainerItem1;

	[AccessedThroughProperty("ReflectionImage1")]
	private ReflectionImage _ReflectionImage1;

	[AccessedThroughProperty("LabelItem1")]
	private LabelItem _LabelItem1;

	[AccessedThroughProperty("LabelItem2")]
	private LabelItem _LabelItem2;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("buttonItem3")]
	private ButtonItem _buttonItem3;

	internal virtual Label label2
	{
		[DebuggerNonUserCode]
		get
		{
			return _label2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_label2 = value;
		}
	}

	internal virtual LinkLabel linkLabel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _linkLabel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_linkLabel1 = value;
		}
	}

	internal virtual LabelX labelX2
	{
		[DebuggerNonUserCode]
		get
		{
			return _labelX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = labelX2_Click;
			if (_labelX2 != null)
			{
				_labelX2.Click -= value2;
			}
			_labelX2 = value;
			if (_labelX2 != null)
			{
				_labelX2.Click += value2;
			}
		}
	}

	internal virtual LabelX labelX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _labelX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_labelX1 = value;
		}
	}

	internal virtual Bar bar1
	{
		[DebuggerNonUserCode]
		get
		{
			return _bar1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_bar1 = value;
		}
	}

	internal virtual Bar Bar2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bar2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bar2_ItemClick;
			if (_Bar2 != null)
			{
				_Bar2.ItemClick -= value2;
			}
			_Bar2 = value;
			if (_Bar2 != null)
			{
				_Bar2.ItemClick += value2;
			}
		}
	}

	internal virtual ControlContainerItem ControlContainerItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ControlContainerItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ControlContainerItem1 = value;
		}
	}

	internal virtual ReflectionImage ReflectionImage1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ReflectionImage1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ReflectionImage1 = value;
		}
	}

	internal virtual LabelItem LabelItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem1 = value;
		}
	}

	internal virtual LabelItem LabelItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem2 = value;
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

	internal virtual ButtonItem buttonItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _buttonItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = buttonItem3_Click;
			if (_buttonItem3 != null)
			{
				_buttonItem3.Click -= value2;
			}
			_buttonItem3 = value;
			if (_buttonItem3 != null)
			{
				_buttonItem3.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static AlertCustom()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public AlertCustom()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		InitializeComponent();
	}

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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.AlertCustom));
		this.label2 = new System.Windows.Forms.Label();
		this.linkLabel1 = new System.Windows.Forms.LinkLabel();
		this.labelX2 = new DevComponents.DotNetBar.LabelX();
		this.labelX1 = new DevComponents.DotNetBar.LabelX();
		this.bar1 = new DevComponents.DotNetBar.Bar();
		this.buttonItem3 = new DevComponents.DotNetBar.ButtonItem();
		this.Bar2 = new DevComponents.DotNetBar.Bar();
		this.LabelItem1 = new DevComponents.DotNetBar.LabelItem();
		this.LabelItem2 = new DevComponents.DotNetBar.LabelItem();
		this.ControlContainerItem1 = new DevComponents.DotNetBar.ControlContainerItem();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.ReflectionImage1 = new DevComponents.DotNetBar.Controls.ReflectionImage();
		((System.ComponentModel.ISupportInitialize)this.bar1).BeginInit();
		((System.ComponentModel.ISupportInitialize)this.Bar2).BeginInit();
		this.SuspendLayout();
		this.label2.BackColor = System.Drawing.Color.Transparent;
		this.label2.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.label2.ForeColor = System.Drawing.Color.Firebrick;
		System.Windows.Forms.Label label = this.label2;
		System.Drawing.Point location = new System.Drawing.Point(16, 144);
		label.Location = location;
		this.label2.Name = "label2";
		System.Windows.Forms.Label label2 = this.label2;
		System.Drawing.Size size = new System.Drawing.Size(168, 16);
		label2.Size = size;
		this.label2.TabIndex = 7;
		this.label2.Text = "Click 'Enable Balloons' now!";
		this.label2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.linkLabel1.BackColor = System.Drawing.Color.Transparent;
		this.linkLabel1.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		System.Windows.Forms.LinkLabel linkLabel = this.linkLabel1;
		location = new System.Drawing.Point(48, 168);
		linkLabel.Location = location;
		this.linkLabel1.Name = "linkLabel1";
		System.Windows.Forms.LinkLabel linkLabel2 = this.linkLabel1;
		size = new System.Drawing.Size(152, 16);
		linkLabel2.Size = size;
		this.linkLabel1.TabIndex = 5;
		this.linkLabel1.TabStop = true;
		this.linkLabel1.Text = "Click to visit DevComponents";
		this.labelX2.BackColor = System.Drawing.Color.Transparent;
		this.labelX2.BackgroundStyle.Class = "";
		this.labelX2.Cursor = System.Windows.Forms.Cursors.Hand;
		this.labelX2.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX = this.labelX2;
		location = new System.Drawing.Point(68, 51);
		labelX.Location = location;
		this.labelX2.Name = "labelX2";
		DevComponents.DotNetBar.LabelX labelX2 = this.labelX2;
		size = new System.Drawing.Size(184, 60);
		labelX2.Size = size;
		this.labelX2.TabIndex = 11;
		this.labelX2.Text = "กร\u0e38ณาอ\u0e31บเดทโปรแกรมให\u0e49เป\u0e47นเวอร\u0e4cช\u0e31\u0e48นป\u0e31จจ\u0e38บ\u0e31นสามารถอ\u0e31บเดทได\u0e49โดยคล\u0e34\u0e4aกท\u0e35\u0e48 Update Now ได\u0e49ท\u0e35\u0e48 ม\u0e38มขวาล\u0e48างของกรอบข\u0e49อความน\u0e35\u0e49....";
		this.labelX2.TextLineAlignment = System.Drawing.StringAlignment.Near;
		this.labelX2.WordWrap = true;
		this.labelX1.BackColor = System.Drawing.Color.Transparent;
		this.labelX1.BackgroundStyle.Class = "";
		this.labelX1.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX3 = this.labelX1;
		location = new System.Drawing.Point(66, 32);
		labelX3.Location = location;
		this.labelX1.Name = "labelX1";
		DevComponents.DotNetBar.LabelX labelX4 = this.labelX1;
		size = new System.Drawing.Size(184, 16);
		labelX4.Size = size;
		this.labelX1.TabIndex = 10;
		this.labelX1.Text = "<b>โปรแกรมได\u0e49อ\u0e31บเดทเวอร\u0e4cช\u0e31\u0e48นใหม\u0e48แล\u0e49ว</b>";
		this.bar1.BackColor = System.Drawing.Color.Transparent;
		this.bar1.Dock = System.Windows.Forms.DockStyle.Bottom;
		this.bar1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.buttonItem3 });
		DevComponents.DotNetBar.Bar bar = this.bar1;
		location = new System.Drawing.Point(0, 111);
		bar.Location = location;
		this.bar1.Name = "bar1";
		DevComponents.DotNetBar.Bar bar2 = this.bar1;
		size = new System.Drawing.Size(255, 25);
		bar2.Size = size;
		this.bar1.Stretch = true;
		this.bar1.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.bar1.TabIndex = 9;
		this.bar1.TabStop = false;
		this.bar1.Text = "bar1";
		this.buttonItem3.ItemAlignment = DevComponents.DotNetBar.eItemAlignment.Far;
		this.buttonItem3.Name = "buttonItem3";
		this.buttonItem3.Text = "Update Now...";
		this.Bar2.BackColor = System.Drawing.Color.Transparent;
		this.Bar2.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.LabelItem1, this.LabelItem2 });
		DevComponents.DotNetBar.Bar bar3 = this.Bar2;
		location = new System.Drawing.Point(0, 1);
		bar3.Location = location;
		this.Bar2.Name = "Bar2";
		this.Bar2.SingleLineColor = System.Drawing.SystemColors.ActiveCaption;
		DevComponents.DotNetBar.Bar bar4 = this.Bar2;
		size = new System.Drawing.Size(227, 19);
		bar4.Size = size;
		this.Bar2.Stretch = true;
		this.Bar2.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.Bar2.TabIndex = 12;
		this.Bar2.TabStop = false;
		this.Bar2.Text = "Bar2";
		this.LabelItem1.Name = "LabelItem1";
		this.LabelItem2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.LabelItem2.Name = "LabelItem2";
		this.LabelItem2.Text = "<b>โปรแกรมแจ\u0e49งเต\u0e37อน </b>";
		this.ControlContainerItem1.AllowItemResize = false;
		this.ControlContainerItem1.MenuVisibility = DevComponents.DotNetBar.eMenuVisibility.VisibleAlways;
		this.ControlContainerItem1.Name = "ControlContainerItem1";
		this.Timer1.Enabled = true;
		this.Timer1.Interval = 300;
		this.ReflectionImage1.BackColor = System.Drawing.Color.Transparent;
		this.ReflectionImage1.BackgroundStyle.Class = "";
		this.ReflectionImage1.BackgroundStyle.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.ReflectionImage1.Image = (System.Drawing.Image)resources.GetObject("ReflectionImage1.Image");
		DevComponents.DotNetBar.Controls.ReflectionImage reflectionImage = this.ReflectionImage1;
		location = new System.Drawing.Point(9, 32);
		reflectionImage.Location = location;
		this.ReflectionImage1.Name = "ReflectionImage1";
		DevComponents.DotNetBar.Controls.ReflectionImage reflectionImage2 = this.ReflectionImage1;
		size = new System.Drawing.Size(50, 80);
		reflectionImage2.Size = size;
		this.ReflectionImage1.TabIndex = 13;
		size = new System.Drawing.Size(6, 16);
		this.AutoScaleBaseSize = size;
		this.BackColor = System.Drawing.Color.FromArgb(227, 239, 255);
		this.BackColor2 = System.Drawing.Color.FromArgb(175, 210, 255);
		this.BorderColor = System.Drawing.Color.FromArgb(101, 147, 207);
		this.CaptionFont = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		size = new System.Drawing.Size(255, 136);
		this.ClientSize = size;
		this.Controls.Add(this.ReflectionImage1);
		this.Controls.Add(this.Bar2);
		this.Controls.Add(this.labelX2);
		this.Controls.Add(this.labelX1);
		this.Controls.Add(this.bar1);
		this.Controls.Add(this.label2);
		this.Controls.Add(this.linkLabel1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ForeColor = System.Drawing.Color.FromArgb(8, 55, 114);
		location = new System.Drawing.Point(0, 0);
		this.Location = location;
		this.Name = "AlertCustom";
		this.Style = DevComponents.DotNetBar.eBallonStyle.Office2007Alert;
		((System.ComponentModel.ISupportInitialize)this.bar1).EndInit();
		((System.ComponentModel.ISupportInitialize)this.Bar2).EndInit();
		this.ResumeLayout(false);
	}

	private void buttonItem3_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUpdate.ShowDialog();
		MyProject.Forms.frmMain1.Close();
	}

	private void Bar2_ItemClick(object sender, EventArgs e)
	{
		Close();
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
	}

	private void labelX2_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUpdate.ShowDialog();
		MyProject.Forms.frmMain1.Close();
	}
}
