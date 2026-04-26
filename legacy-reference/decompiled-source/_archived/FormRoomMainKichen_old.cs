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
public class FormRoomMainKichen_old : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("GroupPanel1")]
	private GroupPanel _GroupPanel1;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("SuperTooltip1")]
	private SuperTooltip _SuperTooltip1;

	[AccessedThroughProperty("TimerPority")]
	private Timer _TimerPority;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("FlowLayoutPanel6")]
	private FlowLayoutPanel _FlowLayoutPanel6;

	[AccessedThroughProperty("FlowLayoutPanel5")]
	private FlowLayoutPanel _FlowLayoutPanel5;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("Timer3")]
	private Timer _Timer3;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

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
			_Timer1 = value;
		}
	}

	internal virtual GroupPanel GroupPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupPanel1 = value;
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

	internal virtual SuperTooltip SuperTooltip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SuperTooltip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SuperTooltip1 = value;
		}
	}

	internal virtual Timer TimerPority
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerPority;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TimerPority = value;
		}
	}

	internal virtual Timer Timer2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Timer2 = value;
		}
	}

	internal virtual FlowLayoutPanel FlowLayoutPanel6
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_FlowLayoutPanel6 = value;
		}
	}

	internal virtual FlowLayoutPanel FlowLayoutPanel5
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_FlowLayoutPanel5 = value;
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
			_ButtonX5 = value;
		}
	}

	internal virtual Timer Timer3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Timer3 = value;
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

	[DebuggerNonUserCode]
	static FormRoomMainKichen_old()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormRoomMainKichen_old()
	{
		Class2.LH6iGfYz9j3MJ();
		base._002Ector();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormRoomMainKichen_old));
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.GroupPanel1 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.FlowLayoutPanel6 = new System.Windows.Forms.FlowLayoutPanel();
		this.FlowLayoutPanel5 = new System.Windows.Forms.FlowLayoutPanel();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.SuperTooltip1 = new DevComponents.DotNetBar.SuperTooltip();
		this.TimerPority = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.Timer3 = new System.Windows.Forms.Timer(this.components);
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.GroupPanel1.SuspendLayout();
		this.FlowLayoutPanel5.SuspendLayout();
		this.SuspendLayout();
		this.Timer1.Interval = 30000;
		this.GroupPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel1.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel1.Controls.Add(this.ButtonX5);
		this.GroupPanel1.Controls.Add(this.FlowLayoutPanel6);
		this.GroupPanel1.Controls.Add(this.FlowLayoutPanel5);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel = this.GroupPanel1;
		System.Drawing.Point location = new System.Drawing.Point(1136, 73);
		groupPanel.Location = location;
		this.GroupPanel1.Name = "GroupPanel1";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel2 = this.GroupPanel1;
		System.Drawing.Size size = new System.Drawing.Size(272, 174);
		groupPanel2.Size = size;
		this.GroupPanel1.Style.BackColor2SchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.GroupPanel1.Style.BackColorGradientAngle = 90;
		this.GroupPanel1.Style.BackColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.GroupPanel1.Style.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel1.Style.BorderBottomWidth = 1;
		this.GroupPanel1.Style.BorderColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.GroupPanel1.Style.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel1.Style.BorderLeftWidth = 1;
		this.GroupPanel1.Style.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel1.Style.BorderRightWidth = 1;
		this.GroupPanel1.Style.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel1.Style.BorderTopWidth = 1;
		this.GroupPanel1.Style.Class = "";
		this.GroupPanel1.Style.CornerDiameter = 4;
		this.GroupPanel1.Style.CornerType = DevComponents.DotNetBar.eCornerType.Rounded;
		this.GroupPanel1.Style.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.GroupPanel1.Style.TextColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.GroupPanel1.Style.TextLineAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Near;
		this.GroupPanel1.StyleMouseDown.Class = "";
		this.GroupPanel1.StyleMouseOver.Class = "";
		this.GroupPanel1.TabIndex = 7;
		this.GroupPanel1.Text = "NOTE (ข\u0e49อความ)";
		this.GroupPanel1.Visible = false;
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.FocusCuesEnabled = false;
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX5;
		location = new System.Drawing.Point(161, 146);
		buttonX.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX5;
		size = new System.Drawing.Size(106, 23);
		buttonX2.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 4;
		this.ButtonX5.Text = "เข\u0e35ยนข\u0e49อความ";
		this.FlowLayoutPanel6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.FlowLayoutPanel6.AutoScroll = true;
		this.FlowLayoutPanel6.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel = this.FlowLayoutPanel6;
		location = new System.Drawing.Point(4, 29);
		flowLayoutPanel.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel2 = this.FlowLayoutPanel6;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel2.Margin = margin;
		this.FlowLayoutPanel6.Name = "FlowLayoutPanel6";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel3 = this.FlowLayoutPanel6;
		size = new System.Drawing.Size(295, 114);
		flowLayoutPanel3.Size = size;
		this.FlowLayoutPanel6.TabIndex = 3;
		this.FlowLayoutPanel5.AutoScroll = true;
		this.FlowLayoutPanel5.BackColor = System.Drawing.Color.Transparent;
		this.FlowLayoutPanel5.Controls.Add(this.Label1);
		this.FlowLayoutPanel5.Controls.Add(this.Label2);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel4 = this.FlowLayoutPanel5;
		location = new System.Drawing.Point(4, 7);
		flowLayoutPanel4.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel5 = this.FlowLayoutPanel5;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel5.Margin = margin;
		this.FlowLayoutPanel5.Name = "FlowLayoutPanel5";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel6 = this.FlowLayoutPanel5;
		size = new System.Drawing.Size(258, 23);
		flowLayoutPanel6.Size = size;
		this.FlowLayoutPanel5.TabIndex = 2;
		this.Label1.BackColor = System.Drawing.Color.WhiteSmoke;
		this.Label1.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(0, 0);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		margin = new System.Windows.Forms.Padding(0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		size = new System.Drawing.Size(190, 22);
		label3.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "ข\u0e49อความถ\u0e36ง";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label2.BackColor = System.Drawing.Color.PaleGreen;
		this.Label2.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label4 = this.Label2;
		location = new System.Drawing.Point(190, 0);
		label4.Location = location;
		System.Windows.Forms.Label label5 = this.Label2;
		margin = new System.Windows.Forms.Padding(0);
		label5.Margin = margin;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(68, 22);
		label6.Size = size;
		this.Label2.TabIndex = 2;
		this.Label2.Text = "จำนวน";
		this.Label2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.SuperTooltip1.AntiAlias = false;
		this.SuperTooltip1.DefaultFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.SuperTooltip superTooltip = this.SuperTooltip1;
		size = new System.Drawing.Size(200, 24);
		superTooltip.MinimumTooltipSize = size;
		this.SuperTooltip1.ShowTooltipImmediately = true;
		this.SuperTooltip1.TooltipDuration = 60;
		this.TimerPority.Enabled = true;
		this.TimerPority.Interval = 18000000;
		this.Timer2.Interval = 15000;
		this.Timer3.Interval = 1000;
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.FocusCuesEnabled = false;
		this.ButtonX7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX7;
		location = new System.Drawing.Point(397, 116);
		buttonX3.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX7;
		size = new System.Drawing.Size(229, 71);
		buttonX4.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 9;
		this.ButtonX7.Text = "รายงาน\r\nค\u0e39ปองอาหาร";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX4;
		location = new System.Drawing.Point(397, 39);
		buttonX5.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX4;
		size = new System.Drawing.Size(229, 71);
		buttonX6.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 4;
		this.ButtonX4.Text = "รายงานการจองห\u0e49องพ\u0e31ก";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1137, 654);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX7);
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.GroupPanel1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormRoomMainKichen";
		this.Text = "ระบบห\u0e49องอาหาร";
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.GroupPanel1.ResumeLayout(false);
		this.FlowLayoutPanel5.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSearchBook.ShowDialog();
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportCoupon.ShowDialog();
	}
}
