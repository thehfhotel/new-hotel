using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmShowPreviewSmartCard : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("LabelItem1")]
	private LabelItem _LabelItem1;

	[AccessedThroughProperty("LabelItem2")]
	private LabelItem _LabelItem2;

	[AccessedThroughProperty("LabelItem4")]
	private LabelItem _LabelItem4;

	[AccessedThroughProperty("ItemContainer2")]
	private ItemContainer _ItemContainer2;

	[AccessedThroughProperty("ButtonItem6")]
	private ButtonItem _ButtonItem6;

	[AccessedThroughProperty("ButtonItem7")]
	private ButtonItem _ButtonItem7;

	[AccessedThroughProperty("ButtonItem8")]
	private ButtonItem _ButtonItem8;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("PictureBox1")]
	private PictureBox _PictureBox1;

	[AccessedThroughProperty("PanelEx3")]
	private PanelEx _PanelEx3;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBarX _ProgressBarX1;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

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

	internal virtual PanelEx PanelEx2
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx2 = value;
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

	internal virtual LabelItem LabelItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem4 = value;
		}
	}

	internal virtual ItemContainer ItemContainer2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer2 = value;
		}
	}

	internal virtual ButtonItem ButtonItem6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem6 = value;
		}
	}

	internal virtual ButtonItem ButtonItem7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem7 = value;
		}
	}

	internal virtual ButtonItem ButtonItem8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonItem8 = value;
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

	internal virtual PictureBox PictureBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _PictureBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PictureBox1 = value;
		}
	}

	internal virtual PanelEx PanelEx3
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx3 = value;
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
			EventHandler value2 = Timer2_Tick;
			if (_Timer2 != null)
			{
				_Timer2.Tick -= value2;
			}
			_Timer2 = value;
			if (_Timer2 != null)
			{
				_Timer2.Tick += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmShowPreviewSmartCard()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmShowPreviewSmartCard()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmShowPreviewSmartCard_FormClosing;
		base.Load += FrmShowPreviewSmartCard_Load;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmShowPreviewSmartCard));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx3 = new DevComponents.DotNetBar.PanelEx();
		this.Label1 = new System.Windows.Forms.Label();
		this.PictureBox1 = new System.Windows.Forms.PictureBox();
		this.LabelItem1 = new DevComponents.DotNetBar.LabelItem();
		this.LabelItem2 = new DevComponents.DotNetBar.LabelItem();
		this.LabelItem4 = new DevComponents.DotNetBar.LabelItem();
		this.ItemContainer2 = new DevComponents.DotNetBar.ItemContainer();
		this.ButtonItem6 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem7 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem8 = new DevComponents.DotNetBar.ButtonItem();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.ProgressBarX1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2.SuspendLayout();
		this.PanelEx3.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.PictureBox1).BeginInit();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(735, 35);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Far;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.Color = System.Drawing.Color.Transparent;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 4;
		this.PanelEx1.Text = "กำล\u0e31งบ\u0e31นท\u0e36กร\u0e39ปภาพ...  |";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx2.Controls.Add(this.PanelEx3);
		this.PanelEx2.Controls.Add(this.PictureBox1);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		location = new System.Drawing.Point(0, 35);
		panelEx3.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		size = new System.Drawing.Size(735, 656);
		panelEx4.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.BorderColor.Color = System.Drawing.Color.Transparent;
		this.PanelEx2.Style.BorderWidth = 0;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 100;
		this.PanelEx3.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx3.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx3.Controls.Add(this.ProgressBarX1);
		this.PanelEx3.Controls.Add(this.Label1);
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx3;
		location = new System.Drawing.Point(52, 532);
		panelEx5.Location = location;
		this.PanelEx3.Name = "PanelEx3";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx3;
		size = new System.Drawing.Size(628, 81);
		panelEx6.Size = size;
		this.PanelEx3.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx3.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx3.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx3.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx3.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx3.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx3.Style.GradientAngle = 90;
		this.PanelEx3.TabIndex = 3;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(237, 53);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(142, 14);
		label2.Size = size;
		this.Label1.TabIndex = 2;
		this.Label1.Text = "กำล\u0e31งบ\u0e31นท\u0e36ก กร\u0e38ณารอส\u0e31กคร\u0e39\u0e48..";
		this.PictureBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PictureBox1.BackColor = System.Drawing.Color.White;
		this.PictureBox1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.PictureBox pictureBox = this.PictureBox1;
		location = new System.Drawing.Point(15, 14);
		pictureBox.Location = location;
		this.PictureBox1.Name = "PictureBox1";
		System.Windows.Forms.PictureBox pictureBox2 = this.PictureBox1;
		size = new System.Drawing.Size(703, 631);
		pictureBox2.Size = size;
		this.PictureBox1.TabIndex = 0;
		this.PictureBox1.TabStop = false;
		this.LabelItem1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.LabelItem1.Name = "LabelItem1";
		this.LabelItem1.Text = "กร\u0e38ณารอส\u0e31กคร\u0e39\u0e48...  ";
		this.LabelItem2.Name = "LabelItem2";
		this.LabelItem2.Text = "    ";
		this.LabelItem4.BackColor = System.Drawing.Color.FromArgb(235, 235, 235);
		this.LabelItem4.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.LabelItem4.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.LabelItem4.Name = "LabelItem4";
		this.LabelItem4.PaddingBottom = 1;
		this.LabelItem4.PaddingLeft = 1;
		this.LabelItem4.PaddingRight = 1;
		this.LabelItem4.PaddingTop = 1;
		this.LabelItem4.SingleLineColor = System.Drawing.Color.FromArgb(197, 197, 197);
		this.LabelItem4.Text = "<b>Tools - หล\u0e31ก</b>";
		this.ItemContainer2.BackgroundStyle.Class = "";
		this.ItemContainer2.HorizontalItemAlignment = DevComponents.DotNetBar.eHorizontalItemsAlignment.Center;
		this.ItemContainer2.ItemSpacing = 0;
		this.ItemContainer2.MultiLine = true;
		this.ItemContainer2.Name = "ItemContainer2";
		this.ItemContainer2.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[3] { this.ButtonItem6, this.ButtonItem7, this.ButtonItem8 });
		this.ItemContainer2.VerticalItemAlignment = DevComponents.DotNetBar.eVerticalItemsAlignment.Middle;
		this.ButtonItem6.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem6.Image = (System.Drawing.Image)resources.GetObject("ButtonItem6.Image");
		this.ButtonItem6.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem6.Name = "ButtonItem6";
		this.ButtonItem6.Text = "เพ\u0e34\u0e48ม\r\nใบวางบ\u0e34ล";
		this.ButtonItem6.Tooltip = "เพ\u0e34\u0e48มใบวางบ\u0e34ล";
		this.ButtonItem7.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem7.Image = (System.Drawing.Image)resources.GetObject("ButtonItem7.Image");
		this.ButtonItem7.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem7.Name = "ButtonItem7";
		this.ButtonItem7.Text = "ยกเล\u0e34ก\r\nใบวางบ\u0e34ล";
		this.ButtonItem7.Tooltip = "ยกเล\u0e34กใบวางบ\u0e34ล";
		this.ButtonItem8.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem8.Image = (System.Drawing.Image)resources.GetObject("ButtonItem8.Image");
		this.ButtonItem8.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem8.Name = "ButtonItem8";
		this.ButtonItem8.Text = "พ\u0e34มพ\u0e4cใบวางบ\u0e34ล";
		this.ButtonItem8.Tooltip = "พ\u0e34มพ\u0e4cใบวางบ\u0e34ล";
		this.Timer1.Interval = 2;
		this.ProgressBarX1.BackgroundStyle.Class = "";
		this.ProgressBarX1.FocusCuesEnabled = false;
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX = this.ProgressBarX1;
		location = new System.Drawing.Point(14, 21);
		progressBarX.Location = location;
		this.ProgressBarX1.Name = "ProgressBarX1";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX2 = this.ProgressBarX1;
		size = new System.Drawing.Size(598, 23);
		progressBarX2.Size = size;
		this.ProgressBarX1.TabIndex = 3;
		this.ProgressBarX1.Text = "ProgressBarX1";
		this.Timer2.Interval = 300;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BottomLeftCornerSize = 0;
		this.BottomRightCornerSize = 0;
		size = new System.Drawing.Size(735, 691);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		this.Name = "FrmShowPreviewSmartCard";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "กำล\u0e31งบ\u0e31นท\u0e36กร\u0e39ปภาพ...";
		this.TopLeftCornerSize = 0;
		this.TopMost = true;
		this.TopRightCornerSize = 0;
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx3.ResumeLayout(false);
		this.PanelEx3.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.PictureBox1).EndInit();
		this.ResumeLayout(false);
	}

	private void FrmShowPreviewSmartCard_FormClosing(object sender, FormClosingEventArgs e)
	{
		PictureBox1.Image = null;
		Timer1.Enabled = false;
	}

	private void FrmShowPreviewSmartCard_Load(object sender, EventArgs e)
	{
		Timer1.Enabled = true;
	}

	public void loadpic()
	{
		FileStream fileStream = new FileStream(Module1.PathF + "/thaiid.png", FileMode.Open, FileAccess.Read);
		PictureBox1.Image = Image.FromStream(fileStream);
		fileStream.Close();
	}

	private void ProgressBar1_Click(object sender, EventArgs e)
	{
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		if (ProgressBarX1.Value > 99)
		{
			Timer1.Enabled = false;
			Timer2.Enabled = true;
		}
	}

	private void Timer2_Tick(object sender, EventArgs e)
	{
		Timer2.Enabled = false;
		Close();
	}
}
