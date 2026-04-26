using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Validator;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmAddSaveImage : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("LabelItem1")]
	private LabelItem _LabelItem1;

	[AccessedThroughProperty("LabelItem2")]
	private LabelItem _LabelItem2;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("RequiredFieldValidator1")]
	private RequiredFieldValidator _RequiredFieldValidator1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("PanelPic")]
	private PanelEx _PanelPic;

	[AccessedThroughProperty("PictureBox1")]
	private PictureBox _PictureBox1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Tname")]
	private TextBox _Tname;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Panel1")]
	private Panel _Panel1;

	[AccessedThroughProperty("ttype")]
	private ComboBox _ttype;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	public static Bitmap myBitmap;

	private string invtype;

	public string Temp_no;

	public string cust_no;

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

	internal virtual RequiredFieldValidator RequiredFieldValidator1
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator1 = value;
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

	internal virtual PanelEx PanelPic
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelPic;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelPic = value;
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

	internal virtual TextBox Tname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tname = value;
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

	internal virtual Panel Panel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel1 = value;
		}
	}

	internal virtual ComboBox ttype
	{
		[DebuggerNonUserCode]
		get
		{
			return _ttype;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ttype_SelectedIndexChanged;
			if (_ttype != null)
			{
				_ttype.SelectedIndexChanged -= value2;
			}
			_ttype = value;
			if (_ttype != null)
			{
				_ttype.SelectedIndexChanged += value2;
			}
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

	internal virtual OpenFileDialog OpenFileDialog1
	{
		[DebuggerNonUserCode]
		get
		{
			return _OpenFileDialog1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_OpenFileDialog1 = value;
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

	[DebuggerNonUserCode]
	static FrmAddSaveImage()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmAddSaveImage()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmAddSaveImage_FormClosing;
		base.Load += FrmAddSaveImage_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		invtype = "";
		Temp_no = "";
		cust_no = "";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmAddSaveImage));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.LabelItem1 = new DevComponents.DotNetBar.LabelItem();
		this.LabelItem2 = new DevComponents.DotNetBar.LabelItem();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ttype = new System.Windows.Forms.ComboBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.PanelPic = new DevComponents.DotNetBar.PanelEx();
		this.PictureBox1 = new System.Windows.Forms.PictureBox();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.Tname = new System.Windows.Forms.TextBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.RequiredFieldValidator1 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("");
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.PanelEx2.SuspendLayout();
		this.Panel1.SuspendLayout();
		this.PanelPic.SuspendLayout();
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
		System.Drawing.Size size = new System.Drawing.Size(836, 35);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Far;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.Color = System.Drawing.Color.Transparent;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 4;
		this.PanelEx1.Text = "เพ\u0e34\u0e48มรายการสแกน |";
		this.LabelItem1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.LabelItem1.Name = "LabelItem1";
		this.LabelItem1.Text = "กร\u0e38ณารอส\u0e31กคร\u0e39\u0e48...  ";
		this.LabelItem2.Name = "LabelItem2";
		this.LabelItem2.Text = "    ";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.ButtonX5);
		this.PanelEx2.Controls.Add(this.ButtonX4);
		this.PanelEx2.Controls.Add(this.ButtonX3);
		this.PanelEx2.Controls.Add(this.ttype);
		this.PanelEx2.Controls.Add(this.Label4);
		this.PanelEx2.Controls.Add(this.Panel1);
		this.PanelEx2.Controls.Add(this.ButtonX2);
		this.PanelEx2.Controls.Add(this.Tname);
		this.PanelEx2.Controls.Add(this.Label3);
		this.PanelEx2.Controls.Add(this.ButtonX1);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		location = new System.Drawing.Point(0, 35);
		panelEx3.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		size = new System.Drawing.Size(836, 627);
		panelEx4.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 102;
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX5;
		location = new System.Drawing.Point(173, 6);
		buttonX.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX5;
		size = new System.Drawing.Size(189, 35);
		buttonX2.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 94;
		this.ButtonX5.Text = "เร\u0e34\u0e48มการ Scan 64bit";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX4;
		location = new System.Drawing.Point(466, 6);
		buttonX3.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX4;
		size = new System.Drawing.Size(106, 35);
		buttonX4.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 92;
		this.ButtonX4.Text = "เป\u0e34ดไฟล\u0e4c";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(368, 6);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(92, 35);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 91;
		this.ButtonX3.Text = "ถ\u0e48ายร\u0e39ป";
		this.ttype.FormattingEnabled = true;
		this.ttype.Items.AddRange(new object[8] { "บ\u0e31ตรประชาชน", "บ\u0e31ตรช\u0e49าราชการ", "ใบข\u0e31บข\u0e35\u0e48", "ใบเสร\u0e47จ", "รายการจดทะเบ\u0e35ยน1", "รายการจดทะเบ\u0e35ยน2", "ทะเบ\u0e35ยนบ\u0e49าน", "Passport" });
		System.Windows.Forms.ComboBox comboBox = this.ttype;
		location = new System.Drawing.Point(278, 593);
		comboBox.Location = location;
		this.ttype.Name = "ttype";
		System.Windows.Forms.ComboBox comboBox2 = this.ttype;
		size = new System.Drawing.Size(121, 22);
		comboBox2.Size = size;
		this.ttype.TabIndex = 2;
		this.Label4.AutoSize = true;
		this.Label4.BackColor = System.Drawing.Color.Transparent;
		this.Label4.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label = this.Label4;
		location = new System.Drawing.Point(222, 597);
		label.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label2 = this.Label4;
		size = new System.Drawing.Size(53, 14);
		label2.Size = size;
		this.Label4.TabIndex = 90;
		this.Label4.Text = "ประเภท :";
		this.Panel1.AutoScroll = true;
		this.Panel1.Controls.Add(this.PanelPic);
		System.Windows.Forms.Panel panel = this.Panel1;
		location = new System.Drawing.Point(14, 48);
		panel.Location = location;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel2 = this.Panel1;
		size = new System.Drawing.Size(810, 530);
		panel2.Size = size;
		this.Panel1.TabIndex = 41;
		this.PanelPic.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelPic.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelPic.Controls.Add(this.PictureBox1);
		DevComponents.DotNetBar.PanelEx panelPic = this.PanelPic;
		location = new System.Drawing.Point(2, 2);
		panelPic.Location = location;
		this.PanelPic.Name = "PanelPic";
		DevComponents.DotNetBar.PanelEx panelPic2 = this.PanelPic;
		size = new System.Drawing.Size(788, 1000);
		panelPic2.Size = size;
		this.PanelPic.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelPic.Style.BackColor1.Color = System.Drawing.Color.White;
		this.PanelPic.Style.BackColor2.Color = System.Drawing.Color.White;
		this.PanelPic.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelPic.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelPic.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelPic.Style.GradientAngle = 90;
		this.PanelPic.TabIndex = 39;
		this.PictureBox1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch;
		System.Windows.Forms.PictureBox pictureBox = this.PictureBox1;
		location = new System.Drawing.Point(2, 2);
		pictureBox.Location = location;
		this.PictureBox1.Name = "PictureBox1";
		System.Windows.Forms.PictureBox pictureBox2 = this.PictureBox1;
		size = new System.Drawing.Size(110, 79);
		pictureBox2.Size = size;
		this.PictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
		this.PictureBox1.TabIndex = 0;
		this.PictureBox1.TabStop = false;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX2;
		location = new System.Drawing.Point(405, 593);
		buttonX7.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX2;
		size = new System.Drawing.Size(97, 23);
		buttonX8.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 4;
		this.ButtonX2.Text = "บ\u0e31นท\u0e36ก";
		this.Tname.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tname.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		System.Windows.Forms.TextBox tname = this.Tname;
		location = new System.Drawing.Point(69, 593);
		tname.Location = location;
		this.Tname.Name = "Tname";
		this.Tname.ReadOnly = true;
		System.Windows.Forms.TextBox tname2 = this.Tname;
		size = new System.Drawing.Size(153, 22);
		tname2.Size = size;
		this.Tname.TabIndex = 1;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label3;
		location = new System.Drawing.Point(14, 597);
		label3.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label4 = this.Label3;
		size = new System.Drawing.Size(50, 14);
		label4.Size = size;
		this.Label3.TabIndex = 40;
		this.Label3.Text = "Cin No :";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX1;
		location = new System.Drawing.Point(14, 6);
		buttonX9.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX1;
		size = new System.Drawing.Size(153, 35);
		buttonX10.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 1;
		this.ButtonX1.Text = "เร\u0e34\u0e48มการ Scan";
		this.OpenFileDialog1.FileName = "OpenFileDialog1";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BottomLeftCornerSize = 0;
		this.BottomRightCornerSize = 0;
		size = new System.Drawing.Size(836, 662);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmAddSaveImage";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เพ\u0e34\u0e48มรายการสแกน";
		this.TopLeftCornerSize = 0;
		this.TopRightCornerSize = 0;
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx2.PerformLayout();
		this.Panel1.ResumeLayout(false);
		this.PanelPic.ResumeLayout(false);
		((System.ComponentModel.ISupportInitialize)this.PictureBox1).EndInit();
		this.ResumeLayout(false);
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		ButtonX1.Enabled = false;
		string text = MyProject.Forms.TwainHandler.ScanIt(Module1.PathF).ToString();
		if (Operators.CompareString(text, "", TextCompare: false) == 0)
		{
			Interaction.MsgBox("error in accessing twain");
		}
		else if (File.Exists(Module1.PathF + "/" + text))
		{
			FileStream fileStream = new FileStream(Module1.PathF + "/" + text, FileMode.Open);
			byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
			fileStream.Read(array, 0, array.Length);
			fileStream.Close();
			MemoryStream memoryStream = new MemoryStream(array);
			myBitmap = new Bitmap(memoryStream);
			ResizePic();
			PictureBox1.Image = myBitmap;
			memoryStream.Close();
			fileStream.Close();
			try
			{
				File.Delete(Module1.PathF + "/" + text);
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			ButtonX1.Enabled = true;
			Tname.Focus();
		}
		else
		{
			PictureBox1.Image = null;
			ButtonX1.Enabled = true;
		}
	}

	public void ResizePic()
	{
		decimal num = default(decimal);
		int num2 = 4;
		checked
		{
			if (myBitmap.Width > PanelPic.Width - 4)
			{
				PictureBox1.Width = PanelPic.Width - num2;
				num = new decimal((double)((myBitmap.Width - PanelPic.Width) * 100) / (double)myBitmap.Width);
				PictureBox1.Height = Convert.ToInt32(decimal.Divide(decimal.Multiply(new decimal(myBitmap.Height), decimal.Subtract(100m, num)), 100m));
			}
			else
			{
				PictureBox1.Width = myBitmap.Width;
				PictureBox1.Height = myBitmap.Height;
			}
			if (PictureBox1.Height > PanelPic.Height - num2)
			{
				num = new decimal((double)((PictureBox1.Height - PanelPic.Height) * 100) / (double)PictureBox1.Height);
				PictureBox1.Width = Convert.ToInt32(decimal.Divide(decimal.Multiply(new decimal(PictureBox1.Width), decimal.Subtract(100m, num)), 100m));
				PictureBox1.Height = PanelPic.Height - num2;
			}
		}
	}

	public void DeleteImage()
	{
		FileStream fileStream = new FileStream(Module1.PathF + "/NoImage.png", FileMode.Open);
		byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
		fileStream.Read(array, 0, array.Length);
		fileStream.Close();
		MemoryStream memoryStream = new MemoryStream(array);
		myBitmap = new Bitmap(memoryStream);
		ResizePic();
		PictureBox1.Image = myBitmap;
		memoryStream.Close();
	}

	private void FrmAddSaveImage_FormClosing(object sender, FormClosingEventArgs e)
	{
		cust_no = "";
	}

	private void FrmAddSaveImage_Load(object sender, EventArgs e)
	{
		ttype.SelectedIndex = 0;
		DeleteImage();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(ttype.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกประเภทเอกสาร");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Cursor = Cursors.WaitCursor;
			Bitmap bitmap = new Bitmap(myBitmap, PictureBox1.Width, PictureBox1.Height);
			bitmap.Save(Module1.PathF + "/tmp.png", ImageFormat.Png);
			bitmap.Dispose();
			FileStream fileStream = new FileStream(Module1.PathF + "/tmp.png", FileMode.Open, FileAccess.Read);
			BinaryReader binaryReader = new BinaryReader(fileStream);
			byte[] array = binaryReader.ReadBytes(checked((int)fileStream.Length));
			binaryReader.Close();
			fileStream.Close();
			StringBuilder stringBuilder = new StringBuilder();
			byte[] array2 = array;
			foreach (byte b in array2)
			{
				stringBuilder.Append(b.ToString("X2"));
			}
			object left = "INSERT INTO [Tb_Save_Image]";
			left = Operators.ConcatenateObject(left, "([cin_no]");
			left = Operators.ConcatenateObject(left, ",[ttype]");
			left = Operators.ConcatenateObject(left, ",[pic],[cust_no],[tmp_no],[pic_date])");
			left = Operators.ConcatenateObject(left, "VALUES");
			left = Operators.ConcatenateObject(left, "(''");
			left = Operators.ConcatenateObject(left, string.Concat(",'" + ttype.Text, "'"));
			left = Operators.ConcatenateObject(left, ",0x" + stringBuilder.ToString());
			left = Operators.ConcatenateObject(left, string.Concat(",'" + cust_no, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",'" + Temp_no, "'"));
			left = Operators.ConcatenateObject(left, ",getdate()");
			left = Operators.ConcatenateObject(left, ")");
			Module1.connect(Conversions.ToString(left));
			Cursor = Cursors.Default;
			Tname.Text = "";
			ttype.SelectedIndex = 0;
			DeleteImage();
			ButtonX1.Focus();
			Close();
		}
	}

	private void ttype_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		Form form = MyProject.Forms.frmCapture;
		form.ShowDialog();
		if (File.Exists(Module1.PathF + "/capture.bmp"))
		{
			FileStream fileStream = new FileStream(Module1.PathF + "/capture.bmp", FileMode.Open);
			byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
			fileStream.Read(array, 0, array.Length);
			fileStream.Close();
			MemoryStream memoryStream = new MemoryStream(array);
			myBitmap = new Bitmap(memoryStream);
			ResizePic();
			PictureBox1.Image = myBitmap;
			memoryStream.Close();
			File.Delete(Module1.PathF + "/capture.bmp");
		}
		else
		{
			PictureBox1.Image = null;
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		openfiles();
	}

	public void openfiles()
	{
		OpenFileDialog openFileDialog = OpenFileDialog1;
		openFileDialog.Filter = "JPG files (*.jpg)|*.jpg|JPEG files (*.jpeg)|*.jpeg |GIF files (*.Gif)|*.gif |BMP files (*.bmp)|*.bmp";
		openFileDialog.FilterIndex = 1;
		openFileDialog.InitialDirectory = "C:\\";
		openFileDialog.Title = "เป\u0e34ดไฟล\u0e4c....";
		openFileDialog = null;
		if (OpenFileDialog1.ShowDialog() == DialogResult.OK)
		{
			try
			{
				FileStream fileStream = new FileStream(OpenFileDialog1.FileName, FileMode.Open);
				byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
				fileStream.Read(array, 0, array.Length);
				fileStream.Close();
				MemoryStream memoryStream = new MemoryStream(array);
				myBitmap = new Bitmap(memoryStream);
				ResizePic();
				PictureBox1.Image = myBitmap;
				memoryStream.Close();
			}
			catch (Exception ex)
			{
				ProjectData.SetProjectError(ex);
				Exception ex2 = ex;
				MessageBox.Show(ex2.Message, "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		Process process = new Process();
		process.StartInfo.WorkingDirectory = Module1.PathF;
		process.StartInfo.FileName = "TwainGui.exe";
		process.Start();
		Hide();
		process.WaitForExit();
		Show();
		process.Close();
		if (File.Exists(Module1.PathF + "/filescan.bmp"))
		{
			FileStream fileStream = new FileStream(Module1.PathF + "/filescan.bmp", FileMode.Open);
			byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
			fileStream.Read(array, 0, array.Length);
			fileStream.Close();
			MemoryStream memoryStream = new MemoryStream(array);
			myBitmap = new Bitmap(memoryStream);
			ResizePic();
			PictureBox1.Image = myBitmap;
			memoryStream.Close();
			File.Delete(Module1.PathF + "/filescan.bmp");
		}
		else
		{
			PictureBox1.Image = null;
		}
	}
}
