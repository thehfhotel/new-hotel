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
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormRoomMain_ViewBook : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("สถานะห\u0e49องพ\u0e31ก")]
	private PanelEx panelEx_0;

	[AccessedThroughProperty("FlowLayoutPanel1")]
	private Panel _FlowLayoutPanel1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Panel1")]
	private Panel _Panel1;

	[AccessedThroughProperty("PanelTOP")]
	private PanelEx _PanelTOP;

	[AccessedThroughProperty("PanelCenter")]
	private PanelEx _PanelCenter;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("PanelNum")]
	private PanelEx _PanelNum;

	[AccessedThroughProperty("SuperTooltip1")]
	private SuperTooltip _SuperTooltip1;

	[AccessedThroughProperty("TimerPority")]
	private Timer _TimerPority;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("Panel_Nofi")]
	private FlowLayoutPanel _Panel_Nofi;

	[AccessedThroughProperty("Panel2")]
	private Panel _Panel2;

	[AccessedThroughProperty("CheckBoxX1")]
	private CheckBoxX _CheckBoxX1;

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("Timer3")]
	private Timer _Timer3;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("ComboBoxEx1")]
	private ComboBoxEx _ComboBoxEx1;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Timer4")]
	private Timer _Timer4;

	private ResizeableControl rc;

	private string SELECT_ROOM_NOW;

	private ArrayList ArrTip;

	private int TipNow;

	public bool showFull;

	private bool IS_LIST_ROOM;

	private int ptX;

	private int ptY;

	private bool drag;

	private int icontrol;

	public bool MOVEEEEE;

	private int CHK_NUM;

	private bool CHK_Busy;

	private ArrayList CHK_Array;

	internal virtual PanelEx PanelEx_0
	{
		[DebuggerNonUserCode]
		get
		{
			return panelEx_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			panelEx_0 = value;
		}
	}

	internal virtual Panel FlowLayoutPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			PaintEventHandler value2 = FlowLayoutPanel1_Paint;
			if (_FlowLayoutPanel1 != null)
			{
				_FlowLayoutPanel1.Paint -= value2;
			}
			_FlowLayoutPanel1 = value;
			if (_FlowLayoutPanel1 != null)
			{
				_FlowLayoutPanel1.Paint += value2;
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

	internal virtual DateTimePicker DateTimePicker1
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = DateTimePicker1_ValueChanged;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged -= value2;
			}
			_DateTimePicker1 = value;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged += value2;
			}
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

	internal virtual PanelEx PanelTOP
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelTOP;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelTOP = value;
		}
	}

	internal virtual PanelEx PanelCenter
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelCenter;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelCenter = value;
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

	internal virtual PanelEx PanelNum
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelNum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelNum = value;
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
			EventHandler value2 = TimerPority_Tick;
			if (_TimerPority != null)
			{
				_TimerPority.Tick -= value2;
			}
			_TimerPority = value;
			if (_TimerPority != null)
			{
				_TimerPority.Tick += value2;
			}
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

	internal virtual FlowLayoutPanel Panel_Nofi
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel_Nofi;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel_Nofi = value;
		}
	}

	internal virtual Panel Panel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel2 = value;
		}
	}

	internal virtual CheckBoxX CheckBoxX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBoxX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CheckBoxX1 = value;
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
			EventHandler value2 = Timer3_Tick;
			if (_Timer3 != null)
			{
				_Timer3.Tick -= value2;
			}
			_Timer3 = value;
			if (_Timer3 != null)
			{
				_Timer3.Tick += value2;
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
			_LabelX1 = value;
		}
	}

	internal virtual ComboBoxEx ComboBoxEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBoxEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBoxEx1_SelectedIndexChanged;
			EventHandler value3 = ComboBoxEx1_MouseHover;
			DragEventHandler value4 = ComboBoxEx1_DragOver;
			if (_ComboBoxEx1 != null)
			{
				_ComboBoxEx1.SelectedIndexChanged -= value2;
				_ComboBoxEx1.MouseHover -= value3;
				_ComboBoxEx1.DragOver -= value4;
			}
			_ComboBoxEx1 = value;
			if (_ComboBoxEx1 != null)
			{
				_ComboBoxEx1.SelectedIndexChanged += value2;
				_ComboBoxEx1.MouseHover += value3;
				_ComboBoxEx1.DragOver += value4;
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
			MouseEventHandler value2 = PanelEx1_MouseDown;
			DragEventHandler value3 = PanelEx1_DragEnter;
			DragEventHandler value4 = PanelEx1_DragDrop;
			EventHandler value5 = PanelEx1_Click;
			if (_PanelEx1 != null)
			{
				_PanelEx1.MouseDown -= value2;
				_PanelEx1.DragEnter -= value3;
				_PanelEx1.DragDrop -= value4;
				_PanelEx1.Click -= value5;
			}
			_PanelEx1 = value;
			if (_PanelEx1 != null)
			{
				_PanelEx1.MouseDown += value2;
				_PanelEx1.DragEnter += value3;
				_PanelEx1.DragDrop += value4;
				_PanelEx1.Click += value5;
			}
		}
	}

	internal virtual Timer Timer4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timer4_Tick;
			if (_Timer4 != null)
			{
				_Timer4.Tick -= value2;
			}
			_Timer4 = value;
			if (_Timer4 != null)
			{
				_Timer4.Tick += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FormRoomMain_ViewBook()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormRoomMain_ViewBook()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Activated += FormRoomMain_Activated;
		base.Deactivate += FormRoomMain_Deactivate;
		base.FormClosing += FormRoomMain_FormClosing;
		base.Load += FormRoomMain_Load;
		base.Validated += FormRoomMain_Validated;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		SELECT_ROOM_NOW = "";
		ArrTip = new ArrayList();
		TipNow = 0;
		showFull = false;
		IS_LIST_ROOM = false;
		icontrol = 0;
		MOVEEEEE = false;
		CHK_NUM = 0;
		CHK_Busy = false;
		CHK_Array = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormRoomMain_ViewBook));
		this.PanelEx_0 = new DevComponents.DotNetBar.PanelEx();
		this.ComboBoxEx1 = new DevComponents.DotNetBar.Controls.ComboBoxEx();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.FlowLayoutPanel1 = new System.Windows.Forms.Panel();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.PanelCenter = new DevComponents.DotNetBar.PanelEx();
		this.Panel_Nofi = new System.Windows.Forms.FlowLayoutPanel();
		this.Panel2 = new System.Windows.Forms.Panel();
		this.PanelNum = new DevComponents.DotNetBar.PanelEx();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.PanelTOP = new DevComponents.DotNetBar.PanelEx();
		this.CheckBoxX_0 = new DevComponents.DotNetBar.Controls.CheckBoxX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.SuperTooltip1 = new DevComponents.DotNetBar.SuperTooltip();
		this.TimerPority = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.Timer3 = new System.Windows.Forms.Timer(this.components);
		this.Timer4 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx_0.SuspendLayout();
		this.FlowLayoutPanel1.SuspendLayout();
		this.Panel1.SuspendLayout();
		this.PanelCenter.SuspendLayout();
		this.Panel_Nofi.SuspendLayout();
		this.PanelTOP.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx_0.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_0.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx_0.Controls.Add(this.ComboBoxEx1);
		this.PanelEx_0.Controls.Add(this.PanelEx1);
		this.PanelEx_0.Controls.Add(this.LabelX1);
		this.PanelEx_0.Controls.Add(this.ButtonX6);
		this.PanelEx_0.Controls.Add(this.ButtonX2);
		this.PanelEx_0.Controls.Add(this.ButtonX1);
		this.PanelEx_0.Controls.Add(this.DateTimePicker1);
		this.PanelEx_0.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx_0;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx_0;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx_0.Name = "สถานะห\u0e49องพ\u0e31ก";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx_0;
		System.Drawing.Size size = new System.Drawing.Size(1137, 38);
		panelEx3.Size = size;
		this.PanelEx_0.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_0.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_0.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_0.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_0.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_0.Style.GradientAngle = 90;
		this.PanelEx_0.TabIndex = 5;
		this.PanelEx_0.Text = "สถานะห\u0e49องพ\u0e31ก";
		this.ComboBoxEx1.AllowDrop = true;
		this.ComboBoxEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBoxEx1.DisplayMember = "Text";
		this.ComboBoxEx1.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawVariable;
		this.ComboBoxEx1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBoxEx1.FormattingEnabled = true;
		this.ComboBoxEx1.ItemHeight = 21;
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx = this.ComboBoxEx1;
		location = new System.Drawing.Point(710, 5);
		comboBoxEx.Location = location;
		this.ComboBoxEx1.Name = "ComboBoxEx1";
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx2 = this.ComboBoxEx1;
		size = new System.Drawing.Size(152, 27);
		comboBoxEx2.Size = size;
		this.ComboBoxEx1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ComboBoxEx1.TabIndex = 10;
		this.PanelEx1.AllowDrop = true;
		this.PanelEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Cursor = System.Windows.Forms.Cursors.Hand;
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx1;
		location = new System.Drawing.Point(265, 5);
		panelEx4.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx1;
		size = new System.Drawing.Size(77, 29);
		panelEx5.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx1.Style.BackColor2.Color = System.Drawing.Color.FromArgb(255, 192, 128);
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 12;
		this.PanelEx1.Text = "ย\u0e49ายข\u0e49ามกล\u0e38\u0e48ม/หน\u0e49าจอไม\u0e48พอ ให\u0e49ลากมาวางท\u0e35\u0e48น\u0e35\u0e48ก\u0e48อน";
		this.PanelEx1.Visible = false;
		this.LabelX1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX1;
		location = new System.Drawing.Point(615, 7);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		size = new System.Drawing.Size(94, 23);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 11;
		this.LabelX1.Text = "กล\u0e38\u0e48มห\u0e49องพ\u0e31ก";
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX6.Checked = true;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX6.Image = (System.Drawing.Image)resources.GetObject("ButtonX6.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX6;
		location = new System.Drawing.Point(314, 4);
		buttonX.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX6;
		size = new System.Drawing.Size(178, 30);
		buttonX2.Size = size;
		this.ButtonX6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX6.TabIndex = 9;
		this.ButtonX6.Text = "ยกเล\u0e34กการเล\u0e37อก";
		this.ButtonX6.Visible = false;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(1101, 6);
		buttonX3.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX4.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX2;
		size = new System.Drawing.Size(29, 27);
		buttonX5.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX1;
		location = new System.Drawing.Point(868, 6);
		buttonX6.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX7.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX1;
		size = new System.Drawing.Size(29, 27);
		buttonX8.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 1;
		this.DateTimePicker1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(900, 6);
		dateTimePicker.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker2.Margin = margin;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		size = new System.Drawing.Size(198, 27);
		dateTimePicker3.Size = size;
		this.DateTimePicker1.TabIndex = 0;
		this.FlowLayoutPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.FlowLayoutPanel1.AutoScroll = true;
		this.FlowLayoutPanel1.BackColor = System.Drawing.SystemColors.GradientActiveCaption;
		this.FlowLayoutPanel1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.FlowLayoutPanel1.Controls.Add(this.Panel1);
		System.Windows.Forms.Panel flowLayoutPanel = this.FlowLayoutPanel1;
		location = new System.Drawing.Point(0, 38);
		flowLayoutPanel.Location = location;
		System.Windows.Forms.Panel flowLayoutPanel2 = this.FlowLayoutPanel1;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel2.Margin = margin;
		this.FlowLayoutPanel1.Name = "FlowLayoutPanel1";
		System.Windows.Forms.Panel flowLayoutPanel3 = this.FlowLayoutPanel1;
		size = new System.Drawing.Size(1137, 614);
		flowLayoutPanel3.Size = size;
		this.FlowLayoutPanel1.TabIndex = 4;
		this.Panel1.Controls.Add(this.PanelEx2);
		this.Panel1.Controls.Add(this.PanelCenter);
		this.Panel1.Controls.Add(this.PanelTOP);
		System.Windows.Forms.Panel panel = this.Panel1;
		location = new System.Drawing.Point(3, 4);
		panel.Location = location;
		System.Windows.Forms.Panel panel2 = this.Panel1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panel2.Margin = margin;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel3 = this.Panel1;
		size = new System.Drawing.Size(128, 142);
		panel3.Size = size;
		this.Panel1.TabIndex = 0;
		this.PanelEx2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx2;
		location = new System.Drawing.Point(0, 120);
		panelEx6.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx7 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx7.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx8 = this.PanelEx2;
		size = new System.Drawing.Size(125, 22);
		panelEx8.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 3;
		this.PanelEx2.Text = "ห\u0e49องเด\u0e35\u0e48ยว";
		this.PanelCenter.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelCenter.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelCenter.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelCenter.Controls.Add(this.Panel_Nofi);
		this.PanelCenter.Controls.Add(this.PanelNum);
		this.PanelCenter.Controls.Add(this.LabelX2);
		DevComponents.DotNetBar.PanelEx panelCenter = this.PanelCenter;
		location = new System.Drawing.Point(0, 30);
		panelCenter.Location = location;
		DevComponents.DotNetBar.PanelEx panelCenter2 = this.PanelCenter;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelCenter2.Margin = margin;
		this.PanelCenter.Name = "PanelCenter";
		DevComponents.DotNetBar.PanelEx panelCenter3 = this.PanelCenter;
		size = new System.Drawing.Size(125, 91);
		panelCenter3.Size = size;
		this.PanelCenter.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelCenter.Style.BackColor1.Color = System.Drawing.Color.LavenderBlush;
		this.PanelCenter.Style.BackColor2.Color = System.Drawing.Color.Violet;
		this.PanelCenter.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelCenter.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelCenter.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelCenter.Style.GradientAngle = 90;
		this.PanelCenter.Style.TextTrimming = System.Drawing.StringTrimming.None;
		this.PanelCenter.TabIndex = 2;
		this.Panel_Nofi.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Panel_Nofi.Controls.Add(this.Panel2);
		System.Windows.Forms.FlowLayoutPanel panel_Nofi = this.Panel_Nofi;
		location = new System.Drawing.Point(0, 71);
		panel_Nofi.Location = location;
		this.Panel_Nofi.Name = "Panel_Nofi";
		System.Windows.Forms.FlowLayoutPanel panel_Nofi2 = this.Panel_Nofi;
		size = new System.Drawing.Size(125, 20);
		panel_Nofi2.Size = size;
		this.Panel_Nofi.TabIndex = 2;
		this.Panel2.BackgroundImage = iHOTEL2025.My.Resources.Resources.lightbulb;
		this.Panel2.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.Panel panel4 = this.Panel2;
		location = new System.Drawing.Point(5, 0);
		panel4.Location = location;
		System.Windows.Forms.Panel panel5 = this.Panel2;
		margin = new System.Windows.Forms.Padding(5, 0, 0, 0);
		panel5.Margin = margin;
		this.Panel2.Name = "Panel2";
		System.Windows.Forms.Panel panel6 = this.Panel2;
		size = new System.Drawing.Size(20, 20);
		panel6.Size = size;
		this.Panel2.TabIndex = 0;
		this.PanelNum.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.PanelNum.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelNum.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		DevComponents.DotNetBar.PanelEx panelNum = this.PanelNum;
		location = new System.Drawing.Point(108, 54);
		panelNum.Location = location;
		this.PanelNum.Name = "PanelNum";
		DevComponents.DotNetBar.PanelEx panelNum2 = this.PanelNum;
		size = new System.Drawing.Size(23, 20);
		panelNum2.Size = size;
		this.PanelNum.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelNum.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelNum.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelNum.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelNum.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelNum.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelNum.Style.GradientAngle = 90;
		this.SuperTooltip1.SetSuperTooltip(this.PanelNum, new DevComponents.DotNetBar.SuperTooltipInfo("ความสำค\u0e31ญของห\u0e49องพ\u0e31ก", "iHOTEL", "จะเร\u0e35ยมตามการใช\u0e49งานห\u0e49องพ\u0e31ก ถ\u0e49าห\u0e49องพ\u0e31กห\u0e49องไหรใช\u0e49งานน\u0e49อยก\u0e47จะม\u0e35เลขความสำค\u0e31ญไปอย\u0e39\u0e48อ\u0e31นด\u0e31บต\u0e49นๆ ถ\u0e49าห\u0e49องไหนม\u0e35การใช\u0e49งานมากอ\u0e31นด\u0e31บก\u0e47จะไปอย\u0e39\u0e48ห\u0e49องท\u0e49ายๆ", iHOTEL2025.My.Resources.Resources.panda_dog_emoticon_008, null, DevComponents.DotNetBar.eTooltipColor.Orange));
		this.PanelNum.TabIndex = 1;
		this.PanelNum.Text = "10";
		this.LabelX2.BackgroundStyle.Class = "";
		this.LabelX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX2;
		location = new System.Drawing.Point(6, 0);
		labelX3.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX2;
		size = new System.Drawing.Size(113, 70);
		labelX4.Size = size;
		this.SuperTooltip1.SetSuperTooltip(this.LabelX2, new DevComponents.DotNetBar.SuperTooltipInfo("", "", "", null, null, DevComponents.DotNetBar.eTooltipColor.Yellow));
		this.LabelX2.TabIndex = 0;
		this.LabelX2.Text = "เด\u0e35\u0e48ยว เด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยวเด\u0e35\u0e48ยว";
		this.LabelX2.TextAlignment = System.Drawing.StringAlignment.Center;
		this.LabelX2.WordWrap = true;
		this.PanelTOP.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelTOP.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelTOP.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelTOP.Controls.Add(this.CheckBoxX_0);
		DevComponents.DotNetBar.PanelEx panelTOP = this.PanelTOP;
		location = new System.Drawing.Point(0, 0);
		panelTOP.Location = location;
		DevComponents.DotNetBar.PanelEx panelTOP2 = this.PanelTOP;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelTOP2.Margin = margin;
		this.PanelTOP.Name = "PanelTOP";
		DevComponents.DotNetBar.PanelEx panelTOP3 = this.PanelTOP;
		size = new System.Drawing.Size(125, 31);
		panelTOP3.Size = size;
		this.PanelTOP.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelTOP.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelTOP.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelTOP.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelTOP.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelTOP.Style.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.PanelTOP.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelTOP.Style.GradientAngle = 90;
		this.PanelTOP.TabIndex = 0;
		this.PanelTOP.Text = "101";
		this.CheckBoxX_0.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.CheckBoxX checkBoxX_ = this.CheckBoxX_0;
		location = new System.Drawing.Point(5, 5);
		checkBoxX_.Location = location;
		this.CheckBoxX_0.Name = "CheckBoxX1";
		DevComponents.DotNetBar.Controls.CheckBoxX checkBoxX_2 = this.CheckBoxX_0;
		size = new System.Drawing.Size(22, 23);
		checkBoxX_2.Size = size;
		this.CheckBoxX_0.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.CheckBoxX_0.TabIndex = 0;
		this.Timer1.Interval = 30000;
		this.SuperTooltip1.AntiAlias = false;
		this.SuperTooltip1.DefaultFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.SuperTooltip1.MaximumWidth = 500;
		DevComponents.DotNetBar.SuperTooltip superTooltip = this.SuperTooltip1;
		size = new System.Drawing.Size(0, 0);
		superTooltip.MinimumTooltipSize = size;
		this.SuperTooltip1.ShowTooltipImmediately = true;
		this.SuperTooltip1.TooltipDuration = 60;
		this.TimerPority.Enabled = true;
		this.TimerPority.Interval = 18000000;
		this.Timer2.Interval = 15000;
		this.Timer3.Interval = 1000;
		this.Timer4.Interval = 5000;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1137, 654);
		this.ClientSize = size;
		this.Controls.Add(this.FlowLayoutPanel1);
		this.Controls.Add(this.PanelEx_0);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormRoomMain_ViewBook";
		this.Text = "สถานะห\u0e49องพ\u0e31ก";
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.PanelEx_0.ResumeLayout(false);
		this.FlowLayoutPanel1.ResumeLayout(false);
		this.Panel1.ResumeLayout(false);
		this.PanelCenter.ResumeLayout(false);
		this.Panel_Nofi.ResumeLayout(false);
		this.PanelTOP.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	public void ListGroup()
	{
		DataSet dataSet = Module1.connect("select Room_Group from HT_Rooms where Room_Group is not null and Room_Group<>'' group by Room_Group");
		ComboBoxEx1.Items.Clear();
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
				ComboBoxEx1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Room_Group"]));
				num2++;
			}
			ComboBoxEx1.SelectedIndex = 0;
		}
	}

	private void FormRoomMain_Activated(object sender, EventArgs e)
	{
		MSSQL.CodeErr = false;
	}

	private void FormRoomMain_Deactivate(object sender, EventArgs e)
	{
		MSSQL.CodeErr = true;
	}

	private void FormRoomMain_FormClosing(object sender, FormClosingEventArgs e)
	{
		if (Module1.ISfullscreen)
		{
			FormRoomMain formRoomMain = new FormRoomMain();
			formRoomMain.Show();
		}
	}

	private void FormRoomMain_Load(object sender, EventArgs e)
	{
		ArrTip.Clear();
		ArrTip.Add("tip1 : 'การย\u0e49ายห\u0e49อง' ให\u0e49คล\u0e34\u0e4aกค\u0e49างห\u0e49องท\u0e35\u0e48จะย\u0e49ายแล\u0e49วลากเมาส\u0e4cไปปล\u0e48อยในกรอบห\u0e49องว\u0e48างได\u0e49เลย");
		ArrTip.Add("tip2 : 'การจองห\u0e49อง' ให\u0e49คล\u0e34\u0e4aกค\u0e49างช\u0e48องส\u0e35เหล\u0e37องด\u0e49านล\u0e48างแล\u0e49วลากเมาส\u0e4cไปปล\u0e48อยในกรอบห\u0e49องว\u0e48างได\u0e49เลย");
		ArrTip.Add("tip3 : 'การทำแผนผ\u0e31ง' ให\u0e49กดป\u0e38\u0e48ม 'แก\u0e49ไขแผนผ\u0e31ง' จากน\u0e31\u0e49นคล\u0e34\u0e4aกท\u0e35\u0e48กรอบส\u0e35เหล\u0e35\u0e48ยมลากไปวางได\u0e49เลย และสามารถใช\u0e49 ป\u0e38\u0e48ม บน ล\u0e48าง ซ\u0e49าย ขวา เพ\u0e37\u0e48อเคล\u0e35\u0e48อนย\u0e49ายกรอบ");
		ListGroup();
		Timer1.Enabled = true;
		Timer2.Enabled = true;
		Timer3.Enabled = true;
		Timer4.Enabled = true;
	}

	public void method_0(string R_NO, string R_TYPE, string R_STATUS, string R_DS, int x, int y, string R_polity, string NAMESTATUStext, double big, double big2)
	{
		Panel panel = new Panel();
		PanelEx panelEx = new PanelEx();
		PanelEx panelEx2 = new PanelEx();
		PanelEx panelEx3 = new PanelEx();
		LabelX labelX = new LabelX();
		PanelEx panelEx4 = new PanelEx();
		FlowLayoutPanel flowLayoutPanel = new FlowLayoutPanel();
		Panel panel2 = new Panel();
		Panel panel3 = new Panel();
		Panel panel4 = new Panel();
		CheckBoxX checkBoxX = new CheckBoxX();
		checkBoxX.BackgroundStyle.Class = "";
		Point location = new Point(5, 1);
		checkBoxX.Location = location;
		checkBoxX.Name = R_NO + "|" + R_STATUS;
		Size size = new Size(23, 23);
		checkBoxX.Size = size;
		checkBoxX.Style = eDotNetBarStyle.StyleManagerControlled;
		checkBoxX.FocusCuesEnabled = false;
		checkBoxX.TabIndex = 0;
		checkBoxX.Text = "";
		checkBoxX.CheckedChanged += BOC_CHECK;
		panel2.BackgroundImage = Resources.coins;
		panel2.BackgroundImageLayout = ImageLayout.None;
		location = new Point(0, 0);
		panel2.Location = location;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(0);
		panel2.Margin = margin;
		panel2.Name = R_NO;
		size = new Size(0, 20);
		panel2.Size = size;
		panel2.TabIndex = 0;
		margin = new System.Windows.Forms.Padding(2, 0, 0, 0);
		panel2.Margin = margin;
		panel2.Cursor = Cursors.Hand;
		panel2.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
		{
			ROOM_DEBT_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
		};
		flowLayoutPanel.Controls.Add(panel2);
		panel4.BackgroundImage = Resources.email;
		panel4.BackgroundImageLayout = ImageLayout.None;
		location = new Point(0, 0);
		panel4.Location = location;
		margin = new System.Windows.Forms.Padding(0);
		panel4.Margin = margin;
		panel4.Name = "0";
		size = new Size(0, 20);
		panel4.Size = size;
		panel4.TabIndex = 0;
		panel4.Cursor = Cursors.Hand;
		panel4.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
		{
			ROOM_NOTE_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
		};
		flowLayoutPanel.Controls.Add(panel4);
		panel3.BackgroundImage = Resources.lightbulb;
		panel3.BackgroundImageLayout = ImageLayout.None;
		location = new Point(0, 0);
		panel3.Location = location;
		margin = new System.Windows.Forms.Padding(0);
		panel3.Margin = margin;
		panel3.Name = R_NO;
		size = new Size(0, 20);
		panel3.Size = size;
		panel3.TabIndex = 0;
		margin = new System.Windows.Forms.Padding(0, 0, 0, 0);
		panel3.Margin = margin;
		SuperTooltip1.SetSuperTooltip(panel3, new SuperTooltipInfo("สถานะการเป\u0e34ดไฟ", "iHOTEL", "ไฟเป\u0e34ดอย\u0e39\u0e48", Resources.lightbulb, null, eTooltipColor.Cyan));
		flowLayoutPanel.Controls.Add(panel3);
		panelEx.Dock = DockStyle.Top;
		panelEx.CanvasColor = SystemColors.Control;
		panelEx.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
		location = new Point(0, 0);
		panelEx.Location = location;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx.Margin = margin;
		panelEx.Name = "PanelTOP";
		size = new Size(125, 25);
		panelEx.Size = size;
		panelEx.Style.Alignment = StringAlignment.Center;
		panelEx.Style.BackColor1.ColorSchemePart = eColorSchemePart.PanelBackground;
		panelEx.Style.BackColor2.ColorSchemePart = eColorSchemePart.PanelBackground2;
		panelEx.Style.Border = eBorderType.SingleLine;
		panelEx.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
		panelEx.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
		panelEx.Style.GradientAngle = 90;
		panelEx.Style.Font = new Font("Tahoma", 12f, FontStyle.Bold, GraphicsUnit.Point, 222);
		panelEx.TabIndex = 0;
		panelEx.Text = R_NO;
		panelEx.Controls.Add(checkBoxX);
		panelEx3.Dock = DockStyle.Fill;
		panelEx3.CanvasColor = SystemColors.Control;
		panelEx3.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
		location = new Point(0, 30);
		panelEx3.Location = location;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx3.Margin = margin;
		panelEx3.Name = "PanelCenter";
		size = new Size(125, 91);
		panelEx3.Size = size;
		panelEx3.Style.Alignment = StringAlignment.Center;
		panelEx3.Style.BackColor1.ColorSchemePart = eColorSchemePart.PanelBackground;
		panelEx3.Style.BackColor2.ColorSchemePart = eColorSchemePart.PanelBackground2;
		panelEx3.Style.Border = eBorderType.SingleLine;
		panelEx3.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
		panelEx3.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
		panelEx3.Style.GradientAngle = 90;
		panelEx3.Controls.Add(flowLayoutPanel);
		panelEx3.Controls.Add(panelEx4);
		panelEx3.Controls.Add(labelX);
		panelEx3.TabIndex = 2;
		panelEx3.Text = "";
		if (R_STATUS.ToString().IndexOf("จอง") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.LightYellow;
			panelEx3.Style.BackColor2.Color = Color.Yellow;
		}
		else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.Snow;
			panelEx3.Style.BackColor2.Color = Color.LightYellow;
		}
		else if (R_STATUS.ToString().IndexOf("  ") != -1)
		{
			panelEx3.Style.BackColor1.Color = Color.White;
			panelEx3.Style.BackColor2.Color = Color.Violet;
		}
		else if (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.WhiteSmoke;
			panelEx3.Style.BackColor2.Color = Color.DarkGray;
		}
		else if (R_STATUS.ToString().IndexOf("รายเด\u0e37อน") != -1)
		{
			panelEx3.Style.BackColor1.Color = Color.Linen;
			panelEx3.Style.BackColor2.Color = Color.DarkOrange;
		}
		else if (R_STATUS.ToString().IndexOf("รายช\u0e31\u0e48วโมง") != -1)
		{
			panelEx3.Style.BackColor1.Color = Color.White;
			panelEx3.Style.BackColor2.Color = Color.Gold;
		}
		else if (R_STATUS.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.MistyRose;
			panelEx3.Style.BackColor2.Color = Color.OrangeRed;
		}
		else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.AliceBlue;
			panelEx3.Style.BackColor2.Color = Color.DeepSkyBlue;
		}
		else if (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.FloralWhite;
			panelEx3.Style.BackColor2.Color = Color.Moccasin;
		}
		else if (R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0)
		{
			panelEx3.Style.BackColor1.Color = Color.Honeydew;
			panelEx3.Style.BackColor2.Color = Color.LightGreen;
		}
		panelEx4.Anchor = AnchorStyles.Bottom | AnchorStyles.Right;
		panelEx4.CanvasColor = SystemColors.Control;
		panelEx4.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
		checked
		{
			location = (panelEx4.Location = new Point(panelEx3.Size.Width - 15, panelEx3.Size.Height - 40));
			panelEx4.Name = "PanelNum";
			Size size2 = new Size(15, 20);
			panelEx4.Size = size2;
			panelEx4.Style.Alignment = StringAlignment.Center;
			panelEx4.Style.BackColor1.ColorSchemePart = eColorSchemePart.PanelBackground;
			panelEx4.Style.BackColor2.ColorSchemePart = eColorSchemePart.PanelBackground2;
			panelEx4.Style.Border = eBorderType.SingleLine;
			panelEx4.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
			panelEx4.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
			panelEx4.Style.GradientAngle = 90;
			panelEx4.TabIndex = 1;
			panelEx4.Text = R_polity;
			SuperTooltip1.SetSuperTooltip(panelEx4, new SuperTooltipInfo("ลำด\u0e31บการใช\u0e49งานของห\u0e49องพ\u0e31ก", "iHOTEL", "จะเร\u0e35ยงตามการใช\u0e49งานห\u0e49องพ\u0e31ก ถ\u0e49าห\u0e49องพ\u0e31กห\u0e49องไหนใช\u0e49งานน\u0e49อยก\u0e47จะม\u0e35เลขลำด\u0e31บการใช\u0e49งานไปอย\u0e39\u0e48อ\u0e31นด\u0e31บต\u0e49นๆ ถ\u0e49าห\u0e49องไหนม\u0e35การใช\u0e49งานมากก\u0e47จะม\u0e35เลขไปอย\u0e39\u0e48อ\u0e31นด\u0e31บท\u0e49ายๆ", Resources.boy_emoticon_009, null, eTooltipColor.Orange));
			panelEx2.Dock = DockStyle.Bottom;
			panelEx2.CanvasColor = SystemColors.Control;
			panelEx2.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
			location = new Point(0, 120);
			panelEx2.Location = location;
			margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			panelEx2.Margin = margin;
			panelEx2.Name = "PanelEx2";
			size2 = new Size(125, 22);
			panelEx2.Size = size2;
			panelEx2.Style.Alignment = StringAlignment.Near;
			panelEx2.Style.BackColor1.ColorSchemePart = eColorSchemePart.PanelBackground;
			panelEx2.Style.BackColor2.ColorSchemePart = eColorSchemePart.PanelBackground2;
			panelEx2.Style.Border = eBorderType.SingleLine;
			panelEx2.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
			panelEx2.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
			panelEx2.Style.GradientAngle = 90;
			panelEx2.TabIndex = 3;
			panelEx2.Text = R_TYPE;
			labelX.BackgroundStyle.Class = "";
			location = new Point(0, 0);
			labelX.Location = location;
			labelX.Name = R_NO;
			labelX.Dock = DockStyle.Fill;
			labelX.TabIndex = 0;
			labelX.Text = R_STATUS;
			labelX.WordWrap = true;
			labelX.TextLineAlignment = StringAlignment.Center;
			labelX.TextAlignment = StringAlignment.Center;
			labelX.AllowDrop = true;
			SuperTooltip1.SetSuperTooltip(labelX, new SuperTooltipInfo("ห\u0e49องพ\u0e31ก", "iHOTEL", NAMESTATUStext, Resources.boy_emoticon_009, null, eTooltipColor.Lemon));
			labelX.Cursor = Cursors.Hand;
			if ((R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0) | (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0) | (R_STATUS.ToString().IndexOf("จอง") == 0))
			{
				labelX.MouseDown += nodebtn_MouseDown;
			}
			labelX.MouseClick += nodebtn_MouseClick;
			labelX.MouseMove += nodebtn_MouseMove;
			labelX.DragEnter += drag_en;
			panel.Controls.Add(panelEx2);
			panel.Controls.Add(panelEx);
			panel.Controls.Add(panelEx3);
			panel.Font = new Font("Tahoma", 9f, FontStyle.Regular, GraphicsUnit.Point, 222);
			location = new Point(x, y);
			panel.Location = location;
			margin = new System.Windows.Forms.Padding(0, 0, 0, 0);
			panel.Margin = margin;
			panel.Name = R_NO;
			size2 = new Size(115, 115);
			panel.Size = size2;
			location = new Point(0, panel.Height - 38);
			flowLayoutPanel.Location = location;
			flowLayoutPanel.Name = "Panel_Nofi";
			size2 = new Size(panel.Width - 20, 20);
			flowLayoutPanel.Size = size2;
			flowLayoutPanel.TabIndex = 2;
			flowLayoutPanel.Anchor = AnchorStyles.Bottom | AnchorStyles.Left;
			if (unchecked(big >= 0.1 && big <= 10.0 && big != 1.0))
			{
				size2 = new Size((int)Math.Round((double)panel.Width * big - 1.0), panel.Height);
				panel.Size = size2;
			}
			if (unchecked(big2 >= 0.1 && big2 <= 10.0 && big2 != 1.0))
			{
				size2 = new Size(panel.Width, (int)Math.Round((double)panel.Height * big2 - 1.0));
				panel.Size = size2;
			}
			FlowLayoutPanel1.Controls.Add(panel);
		}
	}

	public void SETButton_Notclear(string R_NO, string R_TYPE, string R_STATUS, string R_DS, int x, int y, string R_polity, string NAMESTATUStext)
	{
		ClearCheck();
		checked
		{
			int num = FlowLayoutPanel1.Controls.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (Operators.CompareString(R_NO, FlowLayoutPanel1.Controls[num2].Name, TextCompare: false) == 0)
					{
						break;
					}
					num2++;
					continue;
				}
				return;
			}
			CheckBoxX checkBoxX = (CheckBoxX)FlowLayoutPanel1.Controls[num2].Controls[1].Controls[0];
			PanelEx panelEx = (PanelEx)FlowLayoutPanel1.Controls[num2].Controls[2];
			LabelX labelX = (LabelX)panelEx.Controls[2];
			FlowLayoutPanel flowLayoutPanel = (FlowLayoutPanel)panelEx.Controls[0];
			PanelEx panelEx2 = (PanelEx)panelEx.Controls[1];
			_ = (Panel)flowLayoutPanel.Controls[0];
			_ = (Panel)flowLayoutPanel.Controls[1];
			Panel panel = (Panel)flowLayoutPanel.Controls[2];
			labelX.Text = R_STATUS;
			panelEx2.Text = R_polity;
			checkBoxX.Name = R_NO + "|" + R_STATUS;
			SuperTooltip1.SetSuperTooltip(labelX, new SuperTooltipInfo("ห\u0e49องพ\u0e31ก", "iHOTEL", NAMESTATUStext, Resources.boy_emoticon_009, null, eTooltipColor.Lemon));
			labelX.MouseDown -= nodebtn_MouseDown;
			labelX.MouseClick -= nodebtn_MouseClick;
			labelX.MouseMove -= nodebtn_MouseMove;
			labelX.Cursor = Cursors.Hand;
			if ((R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0) | (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0) | (R_STATUS.ToString().IndexOf("จอง") == 0))
			{
				labelX.MouseDown += nodebtn_MouseDown;
			}
			labelX.MouseClick += nodebtn_MouseClick;
			labelX.MouseMove += nodebtn_MouseMove;
			if (R_STATUS.ToString().IndexOf("จอง") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.LightYellow;
				panelEx.Style.BackColor2.Color = Color.Yellow;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.Snow;
				panelEx.Style.BackColor2.Color = Color.LightYellow;
			}
			else if (R_STATUS.ToString().IndexOf("  ") != -1)
			{
				panelEx.Style.BackColor1.Color = Color.White;
				panelEx.Style.BackColor2.Color = Color.Violet;
			}
			else if (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.WhiteSmoke;
				panelEx.Style.BackColor2.Color = Color.DarkGray;
			}
			else if (R_STATUS.ToString().IndexOf("รายเด\u0e37อน") != -1)
			{
				panelEx.Style.BackColor1.Color = Color.Linen;
				panelEx.Style.BackColor2.Color = Color.DarkOrange;
			}
			else if (R_STATUS.ToString().IndexOf("รายช\u0e31\u0e48วโมง") != -1)
			{
				panelEx.Style.BackColor1.Color = Color.White;
				panelEx.Style.BackColor2.Color = Color.Gold;
			}
			else if (R_STATUS.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.MistyRose;
				panelEx.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.AliceBlue;
				panelEx.Style.BackColor2.Color = Color.DeepSkyBlue;
			}
			else if (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.FloralWhite;
				panelEx.Style.BackColor2.Color = Color.Moccasin;
			}
			else if (R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.Honeydew;
				panelEx.Style.BackColor2.Color = Color.LightGreen;
			}
			Size size = new Size(0, 20);
			panel.Size = size;
		}
	}

	private void nodebtn_MouseDown(object sender, MouseEventArgs e)
	{
	}

	private void nodebtn_MouseMove(object sender, MouseEventArgs e)
	{
		MOVEEEEE = true;
	}

	private void nodebtn_MouseClick(object sender, MouseEventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		PanelEx1.Text = "ย\u0e49ายข\u0e49ามกล\u0e38\u0e48ม/หน\u0e49าจอไม\u0e48พอ ให\u0e49ลากมาวางท\u0e35\u0e48น\u0e35\u0e48ก\u0e48อน";
		PanelEx1.Name = "PanelEx1";
		PanelEx1.Visible = false;
		if (e.Button != MouseButtons.Left)
		{
			return;
		}
		if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("จอง") == 0)
		{
			MyProject.Forms.ClickBook_book.B_DATE = DateTimePicker1.Value;
			MyProject.Forms.ClickBook_book.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickBook_book.RoomArr = CHK_Array;
			MyProject.Forms.ClickBook_book.ShowDialog();
			if (MyProject.Forms.ClickBook_book.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ว\u0e48าง") == 0)
		{
			MyProject.Forms.ClickAvliable_book.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickAvliable_book.SET_DATE = DateTimePicker1.Value;
			MyProject.Forms.ClickAvliable_book.RoomArr = CHK_Array;
			MyProject.Forms.ClickAvliable_book.ShowDialog();
			if (MyProject.Forms.ClickAvliable_book.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
	}

	public void ClearCheck()
	{
		CHK_Busy = true;
		checked
		{
			int num = FlowLayoutPanel1.Controls.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				CheckBoxX checkBoxX = (CheckBoxX)FlowLayoutPanel1.Controls[num2].Controls[1].Controls[0];
				checkBoxX.Checked = false;
				checkBoxX.Visible = true;
				FlowLayoutPanel1.Controls[num2].Visible = true;
				num2++;
			}
			CHK_Busy = false;
			CHK_NUM = 0;
			CHK_Array.Clear();
			ButtonX6.Visible = false;
		}
	}

	private void BOC_CHECK(object sender, EventArgs e)
	{
		if (CHK_Busy)
		{
			return;
		}
		checked
		{
			if (CHK_NUM == 0)
			{
				int num = FlowLayoutPanel1.Controls.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					FlowLayoutPanel1.Controls[num2].Controls[1].Controls[0].Visible = false;
					FlowLayoutPanel1.Controls[num2].Visible = false;
					num2++;
				}
				ButtonX6.Visible = false;
			}
			if (Conversions.ToBoolean(NewLateBinding.LateGet(sender, null, "Checked", new object[0], null, null, null)))
			{
				CHK_NUM++;
				ButtonX6.Visible = true;
				CHK_Array.Add(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null).ToString().Substring(0, NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null).ToString().IndexOf("|")));
				string right = "";
				int num5 = FlowLayoutPanel1.Controls.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 <= num4)
					{
						if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num6].Controls[1].Controls[0].Name, NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), TextCompare: false))
						{
							num6++;
							continue;
						}
						right = FlowLayoutPanel1.Controls[num6].Controls[2].Controls[2].Text;
						break;
					}
					break;
				}
				int num8 = FlowLayoutPanel1.Controls.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					int num4 = num8;
					if (num10 <= num4)
					{
						if (Operators.CompareString(FlowLayoutPanel1.Controls[num9].Controls[2].Controls[2].Text, right, TextCompare: false) == 0)
						{
							FlowLayoutPanel1.Controls[num9].Controls[1].Controls[0].Visible = true;
							FlowLayoutPanel1.Controls[num9].Visible = true;
						}
						num9++;
						continue;
					}
					break;
				}
				return;
			}
			CHK_NUM--;
			CHK_Array.Remove(RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)));
			if (CHK_NUM == 0)
			{
				ButtonX6.Visible = false;
				int num11 = FlowLayoutPanel1.Controls.Count - 1;
				int num12 = 0;
				while (true)
				{
					int num13 = num12;
					int num4 = num11;
					if (num13 <= num4)
					{
						FlowLayoutPanel1.Controls[num12].Controls[1].Controls[0].Visible = true;
						FlowLayoutPanel1.Controls[num12].Visible = true;
						num12++;
						continue;
					}
					break;
				}
				return;
			}
			ButtonX6.Visible = true;
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT * FROM HT_CheckIn_Ds WHERE  Cin_Room_Status<>'Check-Out' and  (Cin_No IN (SELECT Cin_No FROM HT_CheckIn_Ds AS HT_CheckIn_Ds_1  WHERE (Cin_Room_No = '", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), "') AND (Cin_Room_Status <> 'Check-Out')))")));
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				int num14 = dataSet.Tables[0].Rows.Count - 1;
				int num15 = 0;
				while (true)
				{
					int num16 = num15;
					int num4 = num14;
					if (num16 > num4)
					{
						break;
					}
					int num17 = FlowLayoutPanel1.Controls.Count - 1;
					int num18 = 0;
					while (true)
					{
						int num19 = num18;
						num4 = num17;
						if (num19 <= num4)
						{
							if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num18].Controls[1].Controls[0].Name, dataSet.Tables[0].Rows[num15]["Cin_Room_No"], TextCompare: false))
							{
								num18++;
								continue;
							}
							FlowLayoutPanel1.Controls[num18].Controls[1].Controls[0].Visible = true;
							FlowLayoutPanel1.Controls[num18].Visible = true;
							break;
						}
						break;
					}
					num15++;
				}
				return;
			}
			string right2 = "";
			int num20 = FlowLayoutPanel1.Controls.Count - 1;
			int num21 = 0;
			while (true)
			{
				int num22 = num21;
				int num4 = num20;
				if (num22 <= num4)
				{
					if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num21].Controls[1].Controls[0].Name, NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), TextCompare: false))
					{
						num21++;
						continue;
					}
					right2 = FlowLayoutPanel1.Controls[num21].Controls[2].Controls[2].Text;
					break;
				}
				break;
			}
			int num23 = FlowLayoutPanel1.Controls.Count - 1;
			int num24 = 0;
			while (true)
			{
				int num25 = num24;
				int num4 = num23;
				if (num25 <= num4)
				{
					if (Operators.CompareString(FlowLayoutPanel1.Controls[num24].Controls[2].Controls[2].Text, right2, TextCompare: false) == 0)
					{
						FlowLayoutPanel1.Controls[num24].Controls[1].Controls[0].Visible = true;
						FlowLayoutPanel1.Controls[num24].Visible = true;
					}
					num24++;
					continue;
				}
				break;
			}
		}
	}

	private void nodebtn_Book_Down(object sender, MouseEventArgs e)
	{
		Timer1.Enabled = false;
		checked
		{
			if ((e.Button == MouseButtons.Left) & (Cursor == Cursors.Default))
			{
				DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select Book_type from View_Book_Date where id=", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null))));
				int num = FlowLayoutPanel1.Controls.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num2].Controls[0].Text, dataSet.Tables[0].Rows[0]["book_type"], TextCompare: false) && FlowLayoutPanel1.Controls[num2].Controls[2].Controls[2].Text.IndexOf("ว\u0e48าง") == 0)
					{
						PanelEx panelEx = (PanelEx)FlowLayoutPanel1.Controls[num2].Controls[2];
						panelEx.Style.BackColor1.Color = Color.LightPink;
						panelEx.Style.BackColor2.Color = Color.White;
					}
					num2++;
				}
				NewLateBinding.LateCall(sender, null, "DoDragDrop", new object[2]
				{
					Operators.ConcatenateObject("#จอง#", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)),
					DragDropEffects.Move
				}, null, null, null, IgnoreReturn: true);
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			else
			{
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select Book_type,book_no from View_Book_Date where id=", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null))));
				DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  HT_Book_H where book_id='", dataSet2.Tables[0].Rows[0]["book_no"]), "'")));
				if (Operators.ConditionalCompareObjectEqual(dataSet3.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
				{
					FrmAddBook frmAddBook = new FrmAddBook();
					frmAddBook.EDIT_ID = Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]);
					frmAddBook.ShowDialog();
				}
				else
				{
					FrmAddBook2 frmAddBook2 = new FrmAddBook2();
					frmAddBook2.EDIT_ID = Conversions.ToString(dataSet2.Tables[0].Rows[0]["book_no"]);
					frmAddBook2.ShowDialog();
				}
			}
		}
	}

	private void nodebtn_Book_click(object sender, MouseEventArgs e)
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select Book_type,book_no from View_Book_Date where id=", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null))));
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from  HT_Book_H where book_id='", dataSet.Tables[0].Rows[0]["book_no"]), "'")));
		if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
		{
			FrmAddBook frmAddBook = new FrmAddBook();
			frmAddBook.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_no"]);
			frmAddBook.ShowDialog();
		}
		else
		{
			FrmAddBook2 frmAddBook2 = new FrmAddBook2();
			frmAddBook2.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["book_no"]);
			frmAddBook2.ShowDialog();
		}
	}

	private void drag_en(object sender, DragEventArgs e)
	{
		if (e.Data.GetDataPresent(DataFormats.Text))
		{
			e.Effect = DragDropEffects.All;
		}
	}

	private void ROOM_NOTE_MouseClick(object sender, MouseEventArgs e)
	{
		Room_Note_Read room_Note_Read = new Room_Note_Read();
		room_Note_Read.R_NO = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		room_Note_Read.ShowDialog();
		if (room_Note_Read.ISOK)
		{
			LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
		}
	}

	private void ROOM_DEBT_MouseClick(object sender, MouseEventArgs e)
	{
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
			return;
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_Room_no='", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')")));
		FrmPayAdd frmPayAdd = new FrmPayAdd();
		frmPayAdd.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		frmPayAdd.ShowDialog();
		LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
	}

	private void EMP_NOTE_MouseClick(object sender, MouseEventArgs e)
	{
		EMP_Note_Read eMP_Note_Read = new EMP_Note_Read();
		eMP_Note_Read.R_NO = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		eMP_Note_Read.ShowDialog();
		if (eMP_Note_Read.ISOK)
		{
			LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
		}
	}

	public void clearFlow()
	{
		checked
		{
			int num = FlowLayoutPanel1.Controls.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					FlowLayoutPanel1.Controls[0].Dispose();
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void LoadRooms(int Px, int Py)
	{
		Cursor = Cursors.WaitCursor;
		SELECT_ROOM_NOW = "";
		DateTime value = DateTimePicker1.Value;
		PanelEx_0.Text = "  สถานะการจองห\u0e49องพ\u0e31กของค\u0e37นว\u0e31นท\u0e35\u0e48 " + Strings.Format(value.Date, "dd/MM/yyyy");
		if (!IS_LIST_ROOM)
		{
			FlowLayoutPanel1.Controls.Clear();
		}
		DataSet dataSet = Module1.connect("select * From HT_Rooms   order by room_NO");
		DataSet dataSet2 = Module1.connect("select * from View_Room_All where room_date_oa=" + Conversions.ToString(value.Date.ToOADate()));
		DataSet dataSet3 = Module1.connect("select * from View_Book_Date where book_status='จอง' and Book_Date_ds ='" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' ");
		Module1.connect("select * from View_Room_All where cin_room_status='เข\u0e49าพ\u0e31ก' order by cin_date_out desc");
		string text = "";
		ArrayList arrayList = new ArrayList();
		ArrayList arrayList2 = new ArrayList();
		DataSet dataSet4 = Module1.connect("select * from HT_SET_RoomType");
		checked
		{
			int num = dataSet4.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				string[] value2 = new string[6]
				{
					Conversions.ToString(dataSet4.Tables[0].Rows[num2]["name"]),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0)
				};
				arrayList2.Add(value2);
				num2++;
			}
			if (DateTime.Compare(DateTimePicker1.Value.Date, DateTime.Now.Date) >= 0)
			{
				int num5 = dataSet.Tables[0].Rows.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					text = "";
					object obj = "\r\n";
					object obj2 = "ว\u0e48าง";
					string text2 = Conversions.ToString(dataSet.Tables[0].Rows[num6]["room_NO"]);
					string r_TYPE = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" ", dataSet.Tables[0].Rows[num6]["Room_Type"]), " "), dataSet.Tables[0].Rows[num6]["Room_details"].ToString()));
					object objectValue = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Room_Polity"]);
					int num8 = Conversions.ToInteger(dataSet.Tables[0].Rows[num6]["Room_x"]);
					int num9 = Conversions.ToInteger(dataSet.Tables[0].Rows[num6]["Room_y"]);
					string value3 = dataSet.Tables[0].Rows[num6]["Room_PriceC"].ToString();
					string value4 = dataSet.Tables[0].Rows[num6]["Room_PriceB"].ToString();
					string text3 = "";
					string text4 = "";
					int num10 = dataSet2.Tables[0].Rows.Count - 1;
					int num11 = 0;
					while (true)
					{
						int num12 = num11;
						num4 = num10;
						if (num12 > num4)
						{
							break;
						}
						text4 = "";
						if (!Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num11]["room_no"], dataSet.Tables[0].Rows[num6]["room_NO"], TextCompare: false))
						{
							num11++;
							continue;
						}
						obj2 = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num11]["room_status"]);
						obj = Operators.ConcatenateObject("\r\n", dataSet2.Tables[0].Rows[num11]["room_details"]);
						text3 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num11]["Cin_room_out"]), "dd-MM-yy HH:mm");
						Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num11]["Cin_room_out"]), "HH:mm");
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num11]["Cin_type"], 1, TextCompare: false))
						{
							DateTime d = Conversions.ToDate(dataSet2.Tables[0].Rows[num11]["Cin_room_in"]);
							text4 = "รายช\u0e31\u0e48วโมง\r\nใช\u0e49ไป " + Convert_Time(d, DateTime.Now);
						}
						else if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num11]["Cin_type"], 2, TextCompare: false))
						{
							text4 = "## รายเด\u0e37อน ##";
						}
						if (Module1.CountCharacter(dataSet2.Tables[0].Rows[num11]["cin_room_all"].ToString(), ' ') >= 2)
						{
							text3 += "  ";
						}
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num11]["Cin_Room_Status"], "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก", TextCompare: false))
						{
							obj2 = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
						}
						text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num11]["cust_name"], "\r\n"), dataSet2.Tables[0].Rows[num11]["cust_work_name"]), "\r\n"), "เลขท\u0e35\u0e48 "), dataSet2.Tables[0].Rows[num11]["room_checkin_no"]), "\r\n"), "ห\u0e49อง "), dataSet2.Tables[0].Rows[num11]["cin_room_all"]));
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(obj2, "Check Out", TextCompare: false))
					{
						obj2 = "ว\u0e48าง";
						text = "ว\u0e48าง";
					}
					if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(obj2, "ว\u0e48าง", TextCompare: false), Operators.CompareObjectEqual(obj2, "จอง", TextCompare: false))))
					{
						text = "ว\u0e48าง";
					}
					if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(obj2, "เข\u0e49าพ\u0e31ก", TextCompare: false), Operators.CompareObjectEqual(obj2, "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก", TextCompare: false))))
					{
						obj2 = ((Operators.CompareString(text4, "", TextCompare: false) == 0) ? Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(obj2, "\r\n"), "ออกว\u0e31นท\u0e35\u0e48"), "\r\n"), text3) : ((text4.IndexOf("รายเด\u0e37อน") == -1) ? Operators.ConcatenateObject(Operators.ConcatenateObject(obj2, "\r\n"), text4) : Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(obj2, "\r\n"), text4), "\r\n"), text3)));
					}
					Conversions.ToString(dataSet.Tables[0].Rows[num6]["Room_Power_STATUS"]);
					int num13 = dataSet3.Tables[0].Rows.Count - 1;
					int num14 = 0;
					while (true)
					{
						int num15 = num14;
						num4 = num13;
						if (num15 <= num4)
						{
							if (!Operators.ConditionalCompareObjectEqual(dataSet3.Tables[0].Rows[num14]["book_type"], text2, TextCompare: false))
							{
								num14++;
								continue;
							}
							obj2 = "จอง\r\n" + dataSet3.Tables[0].Rows[num14]["book_cust_name"].ToString() + " " + dataSet3.Tables[0].Rows[num14]["book_cust_name2"].ToString() + "\r\nโทร." + dataSet3.Tables[0].Rows[num14]["book_cust_tel"].ToString();
							text = dataSet3.Tables[0].Rows[num14]["book_cust_name"].ToString() + " " + dataSet3.Tables[0].Rows[num14]["book_cust_name2"].ToString() + "\r\nโทร." + dataSet3.Tables[0].Rows[num14]["book_cust_tel"].ToString() + "\r\nเลขท\u0e35\u0e48 " + dataSet3.Tables[0].Rows[num14]["book_no"].ToString() + "\r\n" + dataSet3.Tables[0].Rows[num14]["book_room_all"].ToString();
							break;
						}
						break;
					}
					if (Operators.CompareString(dataSet.Tables[0].Rows[num6]["Room_Manternace"].ToString(), "yes", TextCompare: false) == 0)
					{
						obj2 = "ซ\u0e48อม";
						text = "ซ\u0e48อมบำร\u0e38ง";
					}
					int num16 = arrayList.Count - 1;
					int num17 = 0;
					while (true)
					{
						int num18 = num17;
						num4 = num16;
						if (num18 > num4)
						{
							break;
						}
						if (!Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num17], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num6]["room_type"], TextCompare: false))
						{
							num17++;
							continue;
						}
						if (decimal.Compare(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num17], new object[1] { 1 }, null)), 0m) > 0)
						{
							NewLateBinding.LateIndexSetComplex(arrayList[num17], new object[2]
							{
								1,
								decimal.Subtract(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num17], new object[1] { 1 }, null)), 1m)
							}, null, OptimisticSet: false, RValueBase: true);
						}
						break;
					}
					int num19 = arrayList2.Count - 1;
					int num20 = 0;
					while (true)
					{
						int num21 = num20;
						num4 = num19;
						if (num21 > num4)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num6]["Room_Type"], TextCompare: false))
						{
							if (obj2.ToString().IndexOf("จอง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num20], new object[2]
								{
									3,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 3 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("ป\u0e34ดปร\u0e31บปร\u0e38ง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num20], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num20], new object[2]
								{
									2,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 2 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num20], new object[2]
								{
									5,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 5 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("รอ ทำความสะอาด") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num20], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("ว\u0e48าง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num20], new object[2]
								{
									1,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num20], new object[1] { 1 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
						}
						num20++;
					}
					if (!IS_LIST_ROOM)
					{
						if (Operators.CompareString(ComboBoxEx1.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) == 0)
						{
							method_0(text2, r_TYPE, Conversions.ToString(obj2), Conversions.ToString(obj), num8, num9, Conversions.ToString(objectValue), text, Conversions.ToDouble(value3), Conversions.ToDouble(value4));
						}
						else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num6]["Room_Group"], ComboBoxEx1.Text, TextCompare: false))
						{
							method_0(text2, r_TYPE, Conversions.ToString(obj2), Conversions.ToString(obj), num8, num9, Conversions.ToString(objectValue), text, Conversions.ToDouble(value3), Conversions.ToDouble(value4));
						}
					}
					else if (Operators.CompareString(ComboBoxEx1.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) == 0)
					{
						SETButton_Notclear(text2, r_TYPE, Conversions.ToString(obj2), Conversions.ToString(obj), num8, num9, Conversions.ToString(objectValue), text);
					}
					else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num6]["Room_Group"], ComboBoxEx1.Text, TextCompare: false))
					{
						SETButton_Notclear(text2, r_TYPE, Conversions.ToString(obj2), Conversions.ToString(obj), num8, num9, Conversions.ToString(objectValue), text);
					}
					num6++;
				}
			}
			IS_LIST_ROOM = true;
			dataSet.Dispose();
			dataSet2.Dispose();
			dataSet3.Dispose();
			dataSet4.Dispose();
			arrayList.Clear();
			arrayList2.Clear();
			Timer1.Enabled = false;
			Timer1.Enabled = true;
			Cursor = Cursors.Default;
		}
	}

	public string Convert_Time(DateTime D1, DateTime D2)
	{
		string result = "";
		int num = 0;
		checked
		{
			num = (int)DateAndTime.DateDiff(DateInterval.Minute, D1, D2);
			if (num <= 59)
			{
				result = Conversions.ToString(num) + " น.";
			}
			else
			{
				int num2 = (int)Math.Round(Math.Floor((double)num / 60.0));
				if (num - num2 * 60 > 0)
				{
					result = Conversions.ToString(num2) + ":" + Strings.Format(num - num2 * 60, "00") + " ชม.";
				}
				else if (num - num2 * 60 == 0)
				{
					if (num2 >= 1)
					{
						Module1.PlaySound();
					}
				}
				else
				{
					result = Conversions.ToString(num2) + " ชม.";
				}
			}
			return result;
		}
	}

	public void ButtonSub(object sender, EventArgs e)
	{
		ButtonTable buttonTable = (ButtonTable)sender;
		MyProject.Forms.FormSelectRoom.Text = buttonTable.GroupName;
		MyProject.Forms.FormSelectRoom.Lno.Text = buttonTable.GroupName;
		MyProject.Forms.FormSelectRoom.DateTimePicker1.Value = DateTimePicker1.Value;
		MyProject.Forms.FormSelectRoom.ShowDialog();
		LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		DateTimePicker1.Value = DateTimePicker1.Value.AddDays(1.0);
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		DateTimePicker1.Value = DateTimePicker1.Value.AddDays(-1.0);
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		if (MSSQL.CodeErr)
		{
		}
	}

	private void PanelEx3_MouseDown(object sender, MouseEventArgs e)
	{
		if ((e.Button == MouseButtons.Left) & (Cursor == Cursors.Default))
		{
			NewLateBinding.LateCall(sender, null, "DoDragDrop", new object[2]
			{
				"#จอง#",
				DragDropEffects.Move
			}, null, null, null, IgnoreReturn: true);
		}
	}

	private void TimerPority_Tick(object sender, EventArgs e)
	{
		if (!MSSQL.CodeErr)
		{
			Module1.Set_room_pority();
		}
	}

	private void Timer2_Tick(object sender, EventArgs e)
	{
		MyProject.Forms.frmMain1.LabelStatus.Text = Conversions.ToString(ArrTip[TipNow]);
		checked
		{
			if (TipNow == ArrTip.Count - 1)
			{
				TipNow = 0;
			}
			else
			{
				TipNow++;
			}
		}
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		EMP_Note eMP_Note = new EMP_Note();
		eMP_Note.ShowDialog();
		LoadRooms(0, 0);
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		ClearCheck();
	}

	private void Timer3_Tick(object sender, EventArgs e)
	{
		if (Module1.IsListroom)
		{
			Module1.IsListroom = false;
			DateTimePicker1.Value = DateTime.Now;
			ClearCheck();
		}
	}

	private void FlowLayoutPanel1_Paint(object sender, PaintEventArgs e)
	{
	}

	private void ComboBoxEx1_DragOver(object sender, DragEventArgs e)
	{
		ComboBoxEx1.DroppedDown = true;
	}

	private void ComboBoxEx1_MouseHover(object sender, EventArgs e)
	{
	}

	private void ComboBoxEx1_SelectedIndexChanged(object sender, EventArgs e)
	{
		IS_LIST_ROOM = false;
		LoadRooms(0, 0);
	}

	private void PanelEx1_Click(object sender, EventArgs e)
	{
	}

	private void PanelEx1_DragDrop(object sender, DragEventArgs e)
	{
		if (e.Data.GetData(DataFormats.Text).ToString().IndexOf("#ย\u0e49ายห\u0e49อง#") != -1)
		{
			NewLateBinding.LateSet(sender, null, "name", new object[1] { e.Data.GetData(DataFormats.Text).ToString().Replace("#ย\u0e49ายห\u0e49อง#", "") }, null, null);
			NewLateBinding.LateSet(sender, null, "text", new object[1] { e.Data.GetData(DataFormats.Text).ToString().Replace("#ย\u0e49ายห\u0e49อง#", "ห\u0e49องรอย\u0e49าย ") }, null, null);
			MOVEEEEE = true;
		}
	}

	private void PanelEx1_DragEnter(object sender, DragEventArgs e)
	{
		if (e.Data.GetDataPresent(DataFormats.Text))
		{
			e.Effect = DragDropEffects.All;
		}
	}

	private void PanelEx1_MouseDown(object sender, MouseEventArgs e)
	{
		if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), "PanelEx1", TextCompare: false))
		{
			return;
		}
		MOVEEEEE = false;
		if ((e.Button == MouseButtons.Left) & (Cursor == Cursors.Default))
		{
			NewLateBinding.LateCall(sender, null, "DoDragDrop", new object[2]
			{
				Operators.ConcatenateObject("#ย\u0e49ายห\u0e49อง#", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)),
				DragDropEffects.Move
			}, null, null, null, IgnoreReturn: true);
			if (!MOVEEEEE)
			{
				nodebtn_MouseClick(RuntimeHelpers.GetObjectValue(sender), e);
			}
		}
		if (e.Button != MouseButtons.Left)
		{
			return;
		}
		drag = true;
		ptX = e.X;
		ptY = e.Y;
		checked
		{
			int num = FlowLayoutPanel1.Controls.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num2].Name, NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), TextCompare: false))
					{
						break;
					}
					num2++;
					continue;
				}
				return;
			}
			icontrol = num2;
		}
	}

	private void BOOK_PANEL_Click(object sender, EventArgs e)
	{
	}

	private void FormRoomMain_Validated(object sender, EventArgs e)
	{
	}

	private void Timer4_Tick(object sender, EventArgs e)
	{
		Timer4.Enabled = false;
		MSSQL.CodeErr = false;
	}
}
