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
public class FormRoomMainKichen : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("สถานะห\u0e49องพ\u0e31ก")]
	private PanelEx panelEx_0;

	[AccessedThroughProperty("FlowLayoutPanel1")]
	private FlowLayoutPanel _FlowLayoutPanel1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

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

	[AccessedThroughProperty("GroupPanel1")]
	private GroupPanel _GroupPanel1;

	[AccessedThroughProperty("GroupPanel2")]
	private GroupPanel _GroupPanel2;

	[AccessedThroughProperty("FlowLayoutPanel2")]
	private FlowLayoutPanel _FlowLayoutPanel2;

	[AccessedThroughProperty("S1")]
	private Label _S1;

	[AccessedThroughProperty("S2")]
	private Label _S2;

	[AccessedThroughProperty("S3")]
	private Label _S3;

	[AccessedThroughProperty("S4")]
	private Label _S4;

	[AccessedThroughProperty("S5")]
	private Label _S5;

	[AccessedThroughProperty("FlowLayoutPanel3")]
	private FlowLayoutPanel _FlowLayoutPanel3;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("PanelNum")]
	private PanelEx _PanelNum;

	[AccessedThroughProperty("SuperTooltip1")]
	private SuperTooltip _SuperTooltip1;

	[AccessedThroughProperty("BOOK_MAIN")]
	private Panel _BOOK_MAIN;

	[AccessedThroughProperty("BOOK_PANEL")]
	private PanelEx _BOOK_PANEL;

	[AccessedThroughProperty("BOOK_HEAD")]
	private PanelEx _BOOK_HEAD;

	[AccessedThroughProperty("PanelEx3")]
	private PanelEx _PanelEx3;

	[AccessedThroughProperty("FlowLayoutPanel4")]
	private FlowLayoutPanel _FlowLayoutPanel4;

	[AccessedThroughProperty("TimerPority")]
	private Timer _TimerPority;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("Panel_Nofi")]
	private FlowLayoutPanel _Panel_Nofi;

	[AccessedThroughProperty("Panel2")]
	private Panel _Panel2;

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

	[AccessedThroughProperty("CheckBoxX1")]
	private CheckBoxX _CheckBoxX1;

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("Timer3")]
	private Timer _Timer3;

	[AccessedThroughProperty("S6")]
	private Label _S6;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("ComboBoxEx1")]
	private ComboBoxEx _ComboBoxEx1;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	private ResizeableControl rc;

	private string SELECT_ROOM_NOW;

	private ArrayList ArrTip;

	private int TipNow;

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

	internal virtual FlowLayoutPanel FlowLayoutPanel1
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

	internal virtual GroupPanel GroupPanel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupPanel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupPanel2 = value;
		}
	}

	internal virtual FlowLayoutPanel FlowLayoutPanel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_FlowLayoutPanel2 = value;
		}
	}

	internal virtual Label S1
	{
		[DebuggerNonUserCode]
		get
		{
			return _S1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_S1 = value;
		}
	}

	internal virtual Label S2
	{
		[DebuggerNonUserCode]
		get
		{
			return _S2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_S2 = value;
		}
	}

	internal virtual Label S3
	{
		[DebuggerNonUserCode]
		get
		{
			return _S3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_S3 = value;
		}
	}

	internal virtual Label S4
	{
		[DebuggerNonUserCode]
		get
		{
			return _S4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_S4 = value;
		}
	}

	internal virtual Label S5
	{
		[DebuggerNonUserCode]
		get
		{
			return _S5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_S5 = value;
		}
	}

	internal virtual FlowLayoutPanel FlowLayoutPanel3
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_FlowLayoutPanel3 = value;
		}
	}

	internal virtual Button Button1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button1_Click;
			if (_Button1 != null)
			{
				_Button1.Click -= value2;
			}
			_Button1 = value;
			if (_Button1 != null)
			{
				_Button1.Click += value2;
			}
		}
	}

	internal virtual Button Button2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button2_Click;
			if (_Button2 != null)
			{
				_Button2.Click -= value2;
			}
			_Button2 = value;
			if (_Button2 != null)
			{
				_Button2.Click += value2;
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

	internal virtual Panel BOOK_MAIN
	{
		[DebuggerNonUserCode]
		get
		{
			return _BOOK_MAIN;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_BOOK_MAIN = value;
		}
	}

	internal virtual PanelEx BOOK_PANEL
	{
		[DebuggerNonUserCode]
		get
		{
			return _BOOK_PANEL;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			MouseEventHandler value2 = PanelEx3_MouseDown;
			if (_BOOK_PANEL != null)
			{
				_BOOK_PANEL.MouseDown -= value2;
			}
			_BOOK_PANEL = value;
			if (_BOOK_PANEL != null)
			{
				_BOOK_PANEL.MouseDown += value2;
			}
		}
	}

	internal virtual PanelEx BOOK_HEAD
	{
		[DebuggerNonUserCode]
		get
		{
			return _BOOK_HEAD;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_BOOK_HEAD = value;
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

	internal virtual FlowLayoutPanel FlowLayoutPanel4
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_FlowLayoutPanel4 = value;
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

	internal virtual Label S6
	{
		[DebuggerNonUserCode]
		get
		{
			return _S6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_S6 = value;
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
			_PanelEx1 = value;
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
	static FormRoomMainKichen()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormRoomMainKichen()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FormRoomMainClean_FormClosing;
		base.Load += FormRoomMain_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		SELECT_ROOM_NOW = "";
		ArrTip = new ArrayList();
		TipNow = 0;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormRoomMainKichen));
		this.PanelEx_0 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.ComboBoxEx1 = new DevComponents.DotNetBar.Controls.ComboBoxEx();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.FlowLayoutPanel1 = new System.Windows.Forms.FlowLayoutPanel();
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
		this.GroupPanel1 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.FlowLayoutPanel6 = new System.Windows.Forms.FlowLayoutPanel();
		this.FlowLayoutPanel5 = new System.Windows.Forms.FlowLayoutPanel();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.GroupPanel2 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.FlowLayoutPanel3 = new System.Windows.Forms.FlowLayoutPanel();
		this.FlowLayoutPanel2 = new System.Windows.Forms.FlowLayoutPanel();
		this.S1 = new System.Windows.Forms.Label();
		this.S6 = new System.Windows.Forms.Label();
		this.S2 = new System.Windows.Forms.Label();
		this.S3 = new System.Windows.Forms.Label();
		this.S4 = new System.Windows.Forms.Label();
		this.S5 = new System.Windows.Forms.Label();
		this.SuperTooltip1 = new DevComponents.DotNetBar.SuperTooltip();
		this.BOOK_MAIN = new System.Windows.Forms.Panel();
		this.BOOK_PANEL = new DevComponents.DotNetBar.PanelEx();
		this.BOOK_HEAD = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx3 = new DevComponents.DotNetBar.PanelEx();
		this.FlowLayoutPanel4 = new System.Windows.Forms.FlowLayoutPanel();
		this.TimerPority = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.Timer3 = new System.Windows.Forms.Timer(this.components);
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx_0.SuspendLayout();
		this.FlowLayoutPanel1.SuspendLayout();
		this.Panel1.SuspendLayout();
		this.PanelCenter.SuspendLayout();
		this.Panel_Nofi.SuspendLayout();
		this.PanelTOP.SuspendLayout();
		this.GroupPanel1.SuspendLayout();
		this.FlowLayoutPanel5.SuspendLayout();
		this.GroupPanel2.SuspendLayout();
		this.FlowLayoutPanel2.SuspendLayout();
		this.BOOK_MAIN.SuspendLayout();
		this.FlowLayoutPanel4.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx_0.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_0.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx_0.Controls.Add(this.PanelEx1);
		this.PanelEx_0.Controls.Add(this.LabelX1);
		this.PanelEx_0.Controls.Add(this.ComboBoxEx1);
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
		this.PanelEx1.AllowDrop = true;
		this.PanelEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Cursor = System.Windows.Forms.Cursors.Hand;
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx1;
		location = new System.Drawing.Point(265, 41);
		panelEx4.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx1;
		size = new System.Drawing.Size(25, 29);
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
		location = new System.Drawing.Point(898, 9);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		size = new System.Drawing.Size(94, 23);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 11;
		this.LabelX1.Text = "กล\u0e38\u0e48มห\u0e49องพ\u0e31ก";
		this.ComboBoxEx1.AllowDrop = true;
		this.ComboBoxEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBoxEx1.DisplayMember = "Text";
		this.ComboBoxEx1.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawVariable;
		this.ComboBoxEx1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBoxEx1.FormattingEnabled = true;
		this.ComboBoxEx1.ItemHeight = 21;
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx = this.ComboBoxEx1;
		location = new System.Drawing.Point(994, 6);
		comboBoxEx.Location = location;
		this.ComboBoxEx1.Name = "ComboBoxEx1";
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx2 = this.ComboBoxEx1;
		size = new System.Drawing.Size(134, 27);
		comboBoxEx2.Size = size;
		this.ComboBoxEx1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ComboBoxEx1.TabIndex = 10;
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX6.Checked = true;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX6.Image = (System.Drawing.Image)resources.GetObject("ButtonX6.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX6;
		location = new System.Drawing.Point(470, 4);
		buttonX.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX6;
		size = new System.Drawing.Size(152, 30);
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
		location = new System.Drawing.Point(691, 7);
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
		this.ButtonX2.Visible = false;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX1;
		location = new System.Drawing.Point(458, 7);
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
		this.ButtonX1.Visible = false;
		this.DateTimePicker1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(490, 7);
		dateTimePicker.Location = location;
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		dateTimePicker2.Margin = margin;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		size = new System.Drawing.Size(198, 27);
		dateTimePicker3.Size = size;
		this.DateTimePicker1.TabIndex = 0;
		this.DateTimePicker1.Visible = false;
		this.FlowLayoutPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.FlowLayoutPanel1.AutoScroll = true;
		this.FlowLayoutPanel1.BackColor = System.Drawing.SystemColors.GradientActiveCaption;
		this.FlowLayoutPanel1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.FlowLayoutPanel1.Controls.Add(this.Panel1);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel = this.FlowLayoutPanel1;
		location = new System.Drawing.Point(0, 38);
		flowLayoutPanel.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel2 = this.FlowLayoutPanel1;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel2.Margin = margin;
		this.FlowLayoutPanel1.Name = "FlowLayoutPanel1";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel3 = this.FlowLayoutPanel1;
		size = new System.Drawing.Size(988, 614);
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
		this.PanelCenter.Style.BackColor1.Color = System.Drawing.Color.PaleGreen;
		this.PanelCenter.Style.BackColor2.Color = System.Drawing.Color.LightGreen;
		this.PanelCenter.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelCenter.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelCenter.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelCenter.Style.GradientAngle = 90;
		this.PanelCenter.Style.TextTrimming = System.Drawing.StringTrimming.None;
		this.PanelCenter.TabIndex = 2;
		this.Panel_Nofi.Controls.Add(this.Panel2);
		this.Panel_Nofi.Dock = System.Windows.Forms.DockStyle.Bottom;
		System.Windows.Forms.FlowLayoutPanel panel_Nofi = this.Panel_Nofi;
		location = new System.Drawing.Point(0, 71);
		panel_Nofi.Location = location;
		this.Panel_Nofi.Name = "Panel_Nofi";
		System.Windows.Forms.FlowLayoutPanel panel_Nofi2 = this.Panel_Nofi;
		size = new System.Drawing.Size(125, 20);
		panel_Nofi2.Size = size;
		this.Panel_Nofi.TabIndex = 2;
		this.Panel2.BackgroundImage = iHOTEL2025.My.Resources.Resources._1353686083_cash_stack;
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
		location = new System.Drawing.Point(102, 71);
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
		this.GroupPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel1.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel1.Controls.Add(this.ButtonX5);
		this.GroupPanel1.Controls.Add(this.FlowLayoutPanel6);
		this.GroupPanel1.Controls.Add(this.FlowLayoutPanel5);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel = this.GroupPanel1;
		location = new System.Drawing.Point(1136, 73);
		groupPanel.Location = location;
		this.GroupPanel1.Name = "GroupPanel1";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel2 = this.GroupPanel1;
		size = new System.Drawing.Size(272, 174);
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
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX5;
		location = new System.Drawing.Point(161, 146);
		buttonX9.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX5;
		size = new System.Drawing.Size(106, 23);
		buttonX10.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 4;
		this.ButtonX5.Text = "เข\u0e35ยนข\u0e49อความ";
		this.FlowLayoutPanel6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.FlowLayoutPanel6.AutoScroll = true;
		this.FlowLayoutPanel6.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel4 = this.FlowLayoutPanel6;
		location = new System.Drawing.Point(4, 29);
		flowLayoutPanel4.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel5 = this.FlowLayoutPanel6;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel5.Margin = margin;
		this.FlowLayoutPanel6.Name = "FlowLayoutPanel6";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel6 = this.FlowLayoutPanel6;
		size = new System.Drawing.Size(295, 114);
		flowLayoutPanel6.Size = size;
		this.FlowLayoutPanel6.TabIndex = 3;
		this.FlowLayoutPanel5.AutoScroll = true;
		this.FlowLayoutPanel5.BackColor = System.Drawing.Color.Transparent;
		this.FlowLayoutPanel5.Controls.Add(this.Label1);
		this.FlowLayoutPanel5.Controls.Add(this.Label2);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel7 = this.FlowLayoutPanel5;
		location = new System.Drawing.Point(4, 7);
		flowLayoutPanel7.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel8 = this.FlowLayoutPanel5;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel8.Margin = margin;
		this.FlowLayoutPanel5.Name = "FlowLayoutPanel5";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel9 = this.FlowLayoutPanel5;
		size = new System.Drawing.Size(258, 23);
		flowLayoutPanel9.Size = size;
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
		this.GroupPanel2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel2.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel2.Controls.Add(this.Button2);
		this.GroupPanel2.Controls.Add(this.Button1);
		this.GroupPanel2.Controls.Add(this.FlowLayoutPanel3);
		this.GroupPanel2.Controls.Add(this.FlowLayoutPanel2);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel3 = this.GroupPanel2;
		location = new System.Drawing.Point(1084, 485);
		groupPanel3.Location = location;
		this.GroupPanel2.Name = "GroupPanel2";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel4 = this.GroupPanel2;
		size = new System.Drawing.Size(271, 198);
		groupPanel4.Size = size;
		this.GroupPanel2.Style.BackColor2SchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.GroupPanel2.Style.BackColorGradientAngle = 90;
		this.GroupPanel2.Style.BackColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.GroupPanel2.Style.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel2.Style.BorderBottomWidth = 1;
		this.GroupPanel2.Style.BorderColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.GroupPanel2.Style.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel2.Style.BorderLeftWidth = 1;
		this.GroupPanel2.Style.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel2.Style.BorderRightWidth = 1;
		this.GroupPanel2.Style.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.GroupPanel2.Style.BorderTopWidth = 1;
		this.GroupPanel2.Style.Class = "";
		this.GroupPanel2.Style.CornerDiameter = 4;
		this.GroupPanel2.Style.CornerType = DevComponents.DotNetBar.eCornerType.Rounded;
		this.GroupPanel2.Style.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.GroupPanel2.Style.TextColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.GroupPanel2.Style.TextLineAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Near;
		this.GroupPanel2.StyleMouseDown.Class = "";
		this.GroupPanel2.StyleMouseOver.Class = "";
		this.GroupPanel2.TabIndex = 7;
		this.GroupPanel2.Text = "สถานะห\u0e49องท\u0e31\u0e49งหมด";
		this.GroupPanel2.Visible = false;
		this.Button2.Image = (System.Drawing.Image)resources.GetObject("Button2.Image");
		System.Windows.Forms.Button button = this.Button2;
		location = new System.Drawing.Point(242, 151);
		button.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button2 = this.Button2;
		size = new System.Drawing.Size(23, 21);
		button2.Size = size;
		this.Button2.TabIndex = 2;
		this.Button2.UseVisualStyleBackColor = true;
		this.Button1.Image = (System.Drawing.Image)resources.GetObject("Button1.Image");
		System.Windows.Forms.Button button3 = this.Button1;
		location = new System.Drawing.Point(220, 151);
		button3.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button4 = this.Button1;
		size = new System.Drawing.Size(23, 21);
		button4.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.UseVisualStyleBackColor = true;
		this.FlowLayoutPanel3.AutoScroll = true;
		this.FlowLayoutPanel3.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel10 = this.FlowLayoutPanel3;
		location = new System.Drawing.Point(1, 24);
		flowLayoutPanel10.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel11 = this.FlowLayoutPanel3;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel11.Margin = margin;
		this.FlowLayoutPanel3.Name = "FlowLayoutPanel3";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel12 = this.FlowLayoutPanel3;
		size = new System.Drawing.Size(295, 124);
		flowLayoutPanel12.Size = size;
		this.FlowLayoutPanel3.TabIndex = 1;
		this.FlowLayoutPanel2.BackColor = System.Drawing.Color.Transparent;
		this.FlowLayoutPanel2.Controls.Add(this.S1);
		this.FlowLayoutPanel2.Controls.Add(this.S6);
		this.FlowLayoutPanel2.Controls.Add(this.S2);
		this.FlowLayoutPanel2.Controls.Add(this.S3);
		this.FlowLayoutPanel2.Controls.Add(this.S4);
		this.FlowLayoutPanel2.Controls.Add(this.S5);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel13 = this.FlowLayoutPanel2;
		location = new System.Drawing.Point(1, 2);
		flowLayoutPanel13.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel14 = this.FlowLayoutPanel2;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel14.Margin = margin;
		this.FlowLayoutPanel2.Name = "FlowLayoutPanel2";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel15 = this.FlowLayoutPanel2;
		size = new System.Drawing.Size(263, 23);
		flowLayoutPanel15.Size = size;
		this.FlowLayoutPanel2.TabIndex = 0;
		this.S1.BackColor = System.Drawing.Color.WhiteSmoke;
		this.S1.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label s = this.S1;
		location = new System.Drawing.Point(0, 0);
		s.Location = location;
		System.Windows.Forms.Label s2 = this.S1;
		margin = new System.Windows.Forms.Padding(0);
		s2.Margin = margin;
		this.S1.Name = "S1";
		System.Windows.Forms.Label s3 = this.S1;
		size = new System.Drawing.Size(80, 22);
		s3.Size = size;
		this.S1.TabIndex = 0;
		this.S1.Text = "ประเภทห\u0e49อง";
		this.S1.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.S6.BackColor = System.Drawing.Color.Aqua;
		this.S6.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label s4 = this.S6;
		location = new System.Drawing.Point(80, 0);
		s4.Location = location;
		System.Windows.Forms.Label s5 = this.S6;
		margin = new System.Windows.Forms.Padding(0);
		s5.Margin = margin;
		this.S6.Name = "S6";
		System.Windows.Forms.Label s6 = this.S6;
		size = new System.Drawing.Size(34, 22);
		s6.Size = size;
		this.S6.TabIndex = 5;
		this.S6.Text = "ค\u0e37น";
		this.S6.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.S2.BackColor = System.Drawing.Color.PaleGreen;
		this.S2.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label s7 = this.S2;
		location = new System.Drawing.Point(114, 0);
		s7.Location = location;
		System.Windows.Forms.Label s8 = this.S2;
		margin = new System.Windows.Forms.Padding(0);
		s8.Margin = margin;
		this.S2.Name = "S2";
		System.Windows.Forms.Label s9 = this.S2;
		size = new System.Drawing.Size(34, 22);
		s9.Size = size;
		this.S2.TabIndex = 1;
		this.S2.Text = "ว\u0e48าง";
		this.S2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.S3.BackColor = System.Drawing.Color.FromArgb(255, 128, 128);
		this.S3.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label s10 = this.S3;
		location = new System.Drawing.Point(148, 0);
		s10.Location = location;
		System.Windows.Forms.Label s11 = this.S3;
		margin = new System.Windows.Forms.Padding(0);
		s11.Margin = margin;
		this.S3.Name = "S3";
		System.Windows.Forms.Label s12 = this.S3;
		size = new System.Drawing.Size(41, 22);
		s12.Size = size;
		this.S3.TabIndex = 2;
		this.S3.Text = "ไม\u0e48ว\u0e48าง";
		this.S3.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.S4.BackColor = System.Drawing.Color.Yellow;
		this.S4.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label s13 = this.S4;
		location = new System.Drawing.Point(189, 0);
		s13.Location = location;
		System.Windows.Forms.Label s14 = this.S4;
		margin = new System.Windows.Forms.Padding(0);
		s14.Margin = margin;
		this.S4.Name = "S4";
		System.Windows.Forms.Label s15 = this.S4;
		size = new System.Drawing.Size(35, 22);
		s15.Size = size;
		this.S4.TabIndex = 3;
		this.S4.Text = "จอง";
		this.S4.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.S5.BackColor = System.Drawing.Color.FromArgb(255, 192, 128);
		this.S5.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label s16 = this.S5;
		location = new System.Drawing.Point(224, 0);
		s16.Location = location;
		System.Windows.Forms.Label s17 = this.S5;
		margin = new System.Windows.Forms.Padding(0);
		s17.Margin = margin;
		this.S5.Name = "S5";
		System.Windows.Forms.Label s18 = this.S5;
		size = new System.Drawing.Size(35, 22);
		s18.Size = size;
		this.S5.TabIndex = 4;
		this.S5.Text = "รอ";
		this.S5.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.SuperTooltip1.AntiAlias = false;
		this.SuperTooltip1.DefaultFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.SuperTooltip superTooltip = this.SuperTooltip1;
		size = new System.Drawing.Size(200, 24);
		superTooltip.MinimumTooltipSize = size;
		this.SuperTooltip1.ShowTooltipImmediately = true;
		this.SuperTooltip1.TooltipDuration = 60;
		this.BOOK_MAIN.Controls.Add(this.BOOK_PANEL);
		this.BOOK_MAIN.Controls.Add(this.BOOK_HEAD);
		System.Windows.Forms.Panel bOOK_MAIN = this.BOOK_MAIN;
		location = new System.Drawing.Point(3, 4);
		bOOK_MAIN.Location = location;
		System.Windows.Forms.Panel bOOK_MAIN2 = this.BOOK_MAIN;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bOOK_MAIN2.Margin = margin;
		this.BOOK_MAIN.Name = "BOOK_MAIN";
		System.Windows.Forms.Panel bOOK_MAIN3 = this.BOOK_MAIN;
		size = new System.Drawing.Size(128, 61);
		bOOK_MAIN3.Size = size;
		this.BOOK_MAIN.TabIndex = 0;
		this.BOOK_PANEL.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.BOOK_PANEL.CanvasColor = System.Drawing.SystemColors.Control;
		this.BOOK_PANEL.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		DevComponents.DotNetBar.PanelEx bOOK_PANEL = this.BOOK_PANEL;
		location = new System.Drawing.Point(0, 21);
		bOOK_PANEL.Location = location;
		DevComponents.DotNetBar.PanelEx bOOK_PANEL2 = this.BOOK_PANEL;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bOOK_PANEL2.Margin = margin;
		this.BOOK_PANEL.Name = "BOOK_PANEL";
		DevComponents.DotNetBar.PanelEx bOOK_PANEL3 = this.BOOK_PANEL;
		size = new System.Drawing.Size(125, 40);
		bOOK_PANEL3.Size = size;
		this.BOOK_PANEL.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.BOOK_PANEL.Style.BackColor1.Color = System.Drawing.Color.Yellow;
		this.BOOK_PANEL.Style.BackColor2.Color = System.Drawing.Color.Yellow;
		this.BOOK_PANEL.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.BOOK_PANEL.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.BOOK_PANEL.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.BOOK_PANEL.Style.GradientAngle = 90;
		this.BOOK_PANEL.Style.TextTrimming = System.Drawing.StringTrimming.None;
		this.BOOK_PANEL.TabIndex = 2;
		this.BOOK_PANEL.Text = "1234";
		this.BOOK_HEAD.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.BOOK_HEAD.CanvasColor = System.Drawing.SystemColors.Control;
		this.BOOK_HEAD.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		DevComponents.DotNetBar.PanelEx bOOK_HEAD = this.BOOK_HEAD;
		location = new System.Drawing.Point(0, 0);
		bOOK_HEAD.Location = location;
		DevComponents.DotNetBar.PanelEx bOOK_HEAD2 = this.BOOK_HEAD;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bOOK_HEAD2.Margin = margin;
		this.BOOK_HEAD.Name = "BOOK_HEAD";
		DevComponents.DotNetBar.PanelEx bOOK_HEAD3 = this.BOOK_HEAD;
		size = new System.Drawing.Size(125, 22);
		bOOK_HEAD3.Size = size;
		this.BOOK_HEAD.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.BOOK_HEAD.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.BOOK_HEAD.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.BOOK_HEAD.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.BOOK_HEAD.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.BOOK_HEAD.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.BOOK_HEAD.Style.GradientAngle = 90;
		this.BOOK_HEAD.TabIndex = 0;
		this.BOOK_HEAD.Text = "ห\u0e49องเด\u0e35\u0e48ยว";
		this.PanelEx3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx3.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx3.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		DevComponents.DotNetBar.PanelEx panelEx9 = this.PanelEx3;
		location = new System.Drawing.Point(1084, 252);
		panelEx9.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx10 = this.PanelEx3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx10.Margin = margin;
		this.PanelEx3.Name = "PanelEx3";
		DevComponents.DotNetBar.PanelEx panelEx11 = this.PanelEx3;
		size = new System.Drawing.Size(272, 28);
		panelEx11.Size = size;
		this.PanelEx3.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx3.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx3.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx3.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx3.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx3.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx3.Style.GradientAngle = 90;
		this.PanelEx3.TabIndex = 0;
		this.PanelEx3.Text = "รายการจอง\r\n\r\n(ท\u0e35\u0e48ย\u0e31งไม\u0e48ได\u0e49เล\u0e37อกห\u0e49อง)";
		this.PanelEx3.Visible = false;
		this.FlowLayoutPanel4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.FlowLayoutPanel4.AutoScroll = true;
		this.FlowLayoutPanel4.BackColor = System.Drawing.Color.White;
		this.FlowLayoutPanel4.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.FlowLayoutPanel4.Controls.Add(this.BOOK_MAIN);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel16 = this.FlowLayoutPanel4;
		location = new System.Drawing.Point(1084, 279);
		flowLayoutPanel16.Location = location;
		this.FlowLayoutPanel4.Name = "FlowLayoutPanel4";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel17 = this.FlowLayoutPanel4;
		size = new System.Drawing.Size(272, 202);
		flowLayoutPanel17.Size = size;
		this.FlowLayoutPanel4.TabIndex = 8;
		this.FlowLayoutPanel4.Visible = false;
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
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX7;
		location = new System.Drawing.Point(995, 156);
		buttonX11.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX7;
		size = new System.Drawing.Size(134, 71);
		buttonX12.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 9;
		this.ButtonX7.Text = "รายงาน\r\nค\u0e39ปองอาหาร";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX4;
		location = new System.Drawing.Point(995, 79);
		buttonX13.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX4;
		size = new System.Drawing.Size(134, 71);
		buttonX14.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 4;
		this.ButtonX4.Text = "รายงานการจองห\u0e49องพ\u0e31ก";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX3;
		location = new System.Drawing.Point(995, 44);
		buttonX15.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX16.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX3;
		size = new System.Drawing.Size(134, 28);
		buttonX17.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 3;
		this.ButtonX3.Text = "Refresh";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1137, 654);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX7);
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.FlowLayoutPanel4);
		this.Controls.Add(this.PanelEx3);
		this.Controls.Add(this.GroupPanel2);
		this.Controls.Add(this.GroupPanel1);
		this.Controls.Add(this.FlowLayoutPanel1);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.PanelEx_0);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormRoomMainKichen";
		this.Text = "ระบบห\u0e49องคร\u0e31ว";
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.PanelEx_0.ResumeLayout(false);
		this.FlowLayoutPanel1.ResumeLayout(false);
		this.Panel1.ResumeLayout(false);
		this.PanelCenter.ResumeLayout(false);
		this.Panel_Nofi.ResumeLayout(false);
		this.PanelTOP.ResumeLayout(false);
		this.GroupPanel1.ResumeLayout(false);
		this.FlowLayoutPanel5.ResumeLayout(false);
		this.GroupPanel2.ResumeLayout(false);
		this.FlowLayoutPanel2.ResumeLayout(false);
		this.BOOK_MAIN.ResumeLayout(false);
		this.FlowLayoutPanel4.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	protected override bool ProcessCmdKey(ref Message msg, Keys keyData)
	{
		checked
		{
			if (ButtonX4.Checked)
			{
				FlowLayoutPanel1.Focus();
				Point location;
				switch (keyData)
				{
				case Keys.Left:
				{
					Control control2 = FlowLayoutPanel1.Controls[icontrol];
					Point location2 = new Point(FlowLayoutPanel1.Controls[icontrol].Location.X - 1, FlowLayoutPanel1.Controls[icontrol].Location.Y);
					control2.Location = location2;
					Refresh();
					break;
				}
				case Keys.Right:
					location = (FlowLayoutPanel1.Controls[icontrol].Location = new Point(FlowLayoutPanel1.Controls[icontrol].Location.X + 1, FlowLayoutPanel1.Controls[icontrol].Location.Y));
					Refresh();
					break;
				case Keys.Up:
					location = (FlowLayoutPanel1.Controls[icontrol].Location = new Point(FlowLayoutPanel1.Controls[icontrol].Location.X, FlowLayoutPanel1.Controls[icontrol].Location.Y - 1));
					Refresh();
					break;
				case Keys.Down:
				{
					Control control = FlowLayoutPanel1.Controls[icontrol];
					location = new Point(FlowLayoutPanel1.Controls[icontrol].Location.X, FlowLayoutPanel1.Controls[icontrol].Location.Y + 1);
					control.Location = location;
					Refresh();
					break;
				}
				}
			}
			bool result = default(bool);
			return result;
		}
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

	private void FormRoomMainClean_FormClosing(object sender, FormClosingEventArgs e)
	{
		Timer1.Enabled = false;
		Timer2.Enabled = false;
		Timer3.Enabled = false;
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
	}

	public void method_0(string R_NO, string R_TYPE, string R_STATUS, string R_DS, int x, int y, string R_polity, DataSet C_status, DataSet C_SMS, string NAMESTATUStext)
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
		CheckBoxX checkBoxX = new CheckBoxX();
		checkBoxX.BackgroundStyle.Class = "";
		Point location = new Point(5, 1);
		checkBoxX.Location = location;
		checkBoxX.Name = R_NO;
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
		checked
		{
			int num = C_status.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (!Operators.ConditionalCompareObjectEqual(C_status.Tables[0].Rows[num2]["cin_room_no"], R_NO, TextCompare: false))
					{
						num2++;
						continue;
					}
					SuperTooltip1.SetSuperTooltip(panel2, new SuperTooltipInfo("ยอดค\u0e49างชำระ", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num2]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num2]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดค\u0e49างชำระท\u0e31\u0e49งส\u0e34\u0e49น "), Strings.Format(RuntimeHelpers.GetObjectValue(C_status.Tables[0].Rows[num2]["Total_Price_Balance"]), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Yellow));
					size = new Size(20, 20);
					panel2.Size = size;
					break;
				}
				break;
			}
			panel3.BackgroundImage = Resources.email;
			panel3.BackgroundImageLayout = ImageLayout.None;
			location = new Point(0, 0);
			panel3.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			panel3.Margin = margin;
			panel3.Name = "0";
			size = new Size(0, 20);
			panel3.Size = size;
			panel3.TabIndex = 0;
			panel3.Cursor = Cursors.Hand;
			panel3.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
			{
				ROOM_NOTE_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
			};
			flowLayoutPanel.Controls.Add(panel3);
			int num5 = C_SMS.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					if (!Operators.ConditionalCompareObjectEqual(C_SMS.Tables[0].Rows[num6]["SMS_Room"], R_NO, TextCompare: false))
					{
						num6++;
						continue;
					}
					panel3.Name = Conversions.ToString(C_SMS.Tables[0].Rows[num6]["SMS_ID"]);
					size = new Size(20, 20);
					panel3.Size = size;
					break;
				}
				break;
			}
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
			else if (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0)
			{
				panelEx3.Style.BackColor1.Color = Color.WhiteSmoke;
				panelEx3.Style.BackColor2.Color = Color.DarkGray;
			}
			else if (R_STATUS.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx3.Style.BackColor1.Color = Color.MistyRose;
				panelEx3.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx3.Style.BackColor1.Color = Color.Snow;
				panelEx3.Style.BackColor2.Color = Color.LightYellow;
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
			else if (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0)
			{
				panelEx3.Style.BackColor1.Color = Color.FloralWhite;
				panelEx3.Style.BackColor2.Color = Color.White;
			}
			else if (R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0)
			{
				panelEx3.Style.BackColor1.Color = Color.Honeydew;
				panelEx3.Style.BackColor2.Color = Color.LightGreen;
			}
			panelEx4.Anchor = AnchorStyles.Bottom | AnchorStyles.Right;
			panelEx4.CanvasColor = SystemColors.Control;
			panelEx4.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
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
			panelEx2.Style.Alignment = StringAlignment.Center;
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
			if (ButtonX4.Checked)
			{
				labelX.Cursor = Cursors.NoMove2D;
			}
			else
			{
				labelX.Cursor = Cursors.Hand;
			}
			if ((R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0) | (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0) | (R_STATUS.ToString().IndexOf("จอง") == 0) | ButtonX4.Checked)
			{
				labelX.MouseDown += nodebtn_MouseDown;
			}
			else
			{
				labelX.MouseDown += nodebtn_MouseDown_Drag;
			}
			labelX.MouseClick += nodebtn_MouseClick;
			labelX.MouseMove += nodebtn_MouseMove;
			labelX.MouseUp += nodebtn_MouseUp;
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
			C_status.Dispose();
			C_SMS.Dispose();
			FlowLayoutPanel1.Controls.Add(panel);
		}
	}

	public void SETButton_Notclear(string R_NO, string R_TYPE, string R_STATUS, string R_DS, int x, int y, string R_polity, DataSet C_status, DataSet C_SMS, string NAMESTATUStext)
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
					if (Operators.CompareString(R_NO, FlowLayoutPanel1.Controls[num2].Name, TextCompare: false) == 0)
					{
						break;
					}
					num2++;
					continue;
				}
				return;
			}
			PanelEx panelEx = (PanelEx)FlowLayoutPanel1.Controls[num2].Controls[2];
			LabelX labelX = (LabelX)panelEx.Controls[2];
			FlowLayoutPanel flowLayoutPanel = (FlowLayoutPanel)panelEx.Controls[0];
			PanelEx panelEx2 = (PanelEx)panelEx.Controls[1];
			Panel panel = (Panel)flowLayoutPanel.Controls[0];
			Panel panel2 = (Panel)flowLayoutPanel.Controls[1];
			labelX.Text = R_STATUS;
			panelEx2.Text = R_polity;
			SuperTooltip1.SetSuperTooltip(labelX, new SuperTooltipInfo("ห\u0e49องพ\u0e31ก", "iHOTEL", NAMESTATUStext, Resources.boy_emoticon_009, null, eTooltipColor.Lemon));
			labelX.MouseDown -= nodebtn_MouseDown;
			labelX.MouseDown -= nodebtn_MouseDown_Drag;
			labelX.MouseClick -= nodebtn_MouseClick;
			labelX.MouseMove -= nodebtn_MouseMove;
			labelX.MouseUp -= nodebtn_MouseUp;
			if (ButtonX4.Checked)
			{
				labelX.Cursor = Cursors.NoMove2D;
			}
			else
			{
				labelX.Cursor = Cursors.Hand;
			}
			if ((R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0) | (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0) | (R_STATUS.ToString().IndexOf("จอง") == 0) | ButtonX4.Checked)
			{
				labelX.MouseDown += nodebtn_MouseDown;
			}
			else
			{
				labelX.MouseDown += nodebtn_MouseDown_Drag;
			}
			labelX.MouseClick += nodebtn_MouseClick;
			labelX.MouseMove += nodebtn_MouseMove;
			labelX.MouseUp += nodebtn_MouseUp;
			if (R_STATUS.ToString().IndexOf("จอง") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.LightYellow;
				panelEx.Style.BackColor2.Color = Color.Yellow;
			}
			else if (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.WhiteSmoke;
				panelEx.Style.BackColor2.Color = Color.DarkGray;
			}
			else if (R_STATUS.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.MistyRose;
				panelEx.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.Snow;
				panelEx.Style.BackColor2.Color = Color.LightYellow;
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
			else if (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.FloralWhite;
				panelEx.Style.BackColor2.Color = Color.White;
			}
			else if (R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.Honeydew;
				panelEx.Style.BackColor2.Color = Color.LightGreen;
			}
			bool flag = false;
			bool flag2 = false;
			int num5 = C_status.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					if (!Operators.ConditionalCompareObjectEqual(C_status.Tables[0].Rows[num6]["cin_room_no"], R_NO, TextCompare: false))
					{
						num6++;
						continue;
					}
					SuperTooltip1.SetSuperTooltip(panel, new SuperTooltipInfo("ยอดค\u0e49างชำระ", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num6]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num6]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดค\u0e49างชำระท\u0e31\u0e49งส\u0e34\u0e49น "), Strings.Format(RuntimeHelpers.GetObjectValue(C_status.Tables[0].Rows[num6]["Total_Price_Balance"]), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Yellow));
					Size size = new Size(20, 20);
					panel.Size = size;
					flag = true;
					break;
				}
				break;
			}
			if (!flag)
			{
				Size size = new Size(0, 20);
				panel.Size = size;
			}
			int num8 = C_SMS.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 <= num4)
				{
					if (!Operators.ConditionalCompareObjectEqual(C_SMS.Tables[0].Rows[num9]["SMS_Room"], R_NO, TextCompare: false))
					{
						num9++;
						continue;
					}
					panel2.Name = Conversions.ToString(C_SMS.Tables[0].Rows[num9]["SMS_ID"]);
					Size size = new Size(20, 20);
					panel2.Size = size;
					flag2 = true;
					break;
				}
				break;
			}
			if (!flag2)
			{
				Size size = new Size(0, 20);
				panel2.Size = size;
			}
		}
	}

	private void nodebtn_MouseDown(object sender, MouseEventArgs e)
	{
		if (!ButtonX4.Checked || e.Button != MouseButtons.Left)
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

	private void nodebtn_MouseDown_Drag(object sender, MouseEventArgs e)
	{
		MOVEEEEE = false;
		PanelEx1.Text = "ย\u0e49ายข\u0e49ามกล\u0e38\u0e48ม/หน\u0e49าจอไม\u0e48พอ ให\u0e49ลากมาวางท\u0e35\u0e48น\u0e35\u0e48ก\u0e48อน";
		PanelEx1.Name = "PanelEx1";
		PanelEx1.Visible = true;
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
		if (!ButtonX4.Checked || e.Button != MouseButtons.Left)
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

	private void nodebtn_MouseMove(object sender, MouseEventArgs e)
	{
		MOVEEEEE = true;
		if (ButtonX4.Checked && drag)
		{
			Control control = FlowLayoutPanel1.Controls[icontrol];
			Point location = checked(new Point(FlowLayoutPanel1.Controls[icontrol].Location.X + e.X - ptX, FlowLayoutPanel1.Controls[icontrol].Location.Y + e.Y - ptY));
			control.Location = location;
			Refresh();
		}
	}

	private void nodebtn_MouseUp(object sender, MouseEventArgs e)
	{
		if (!ButtonX4.Checked)
		{
			return;
		}
		drag = false;
		int num = 0;
		int num2 = 0;
		Control control = FlowLayoutPanel1.Controls[icontrol];
		checked
		{
			Point location = new Point(FlowLayoutPanel1.Controls[icontrol].Location.X + e.X - ptX, FlowLayoutPanel1.Controls[icontrol].Location.Y + e.Y - ptY);
			control.Location = location;
			int num3 = FlowLayoutPanel1.Controls.Count - 1;
			int num4 = 0;
			Point point;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 > num6)
				{
					break;
				}
				if ((FlowLayoutPanel1.Controls[icontrol].Location.X >= FlowLayoutPanel1.Controls[num4].Location.X + 65) & (FlowLayoutPanel1.Controls[icontrol].Location.X <= FlowLayoutPanel1.Controls[num4].Location.X + 114 + 50))
				{
					bool num7 = FlowLayoutPanel1.Controls[icontrol].Location.Y >= FlowLayoutPanel1.Controls[num4].Location.Y - 50;
					int num8 = FlowLayoutPanel1.Controls[icontrol].Location.Y;
					point = FlowLayoutPanel1.Controls[num4].Location;
					if (num7 & (num8 <= point.Y + 50))
					{
						num = FlowLayoutPanel1.Controls[num4].Location.X + 114;
						num2 = FlowLayoutPanel1.Controls[num4].Location.Y;
						break;
					}
				}
				num4++;
			}
			if (unchecked(num != 0 && num2 != 0))
			{
				Control control2 = FlowLayoutPanel1.Controls[icontrol];
				Point location2 = new Point(num, num2);
				control2.Location = location2;
			}
			else if (num2 != 0)
			{
				point = (FlowLayoutPanel1.Controls[icontrol].Location = new Point(FlowLayoutPanel1.Controls[icontrol].Location.X + e.X - ptX, num2));
			}
			else if (num != 0)
			{
				point = (FlowLayoutPanel1.Controls[icontrol].Location = new Point(num, FlowLayoutPanel1.Controls[icontrol].Location.Y + e.Y - ptY));
			}
		}
	}

	private void nodebtn_MouseClick(object sender, MouseEventArgs e)
	{
		if (ButtonX4.Checked)
		{
			return;
		}
		if (!Module1.check_round_bill())
		{
			MessageBox.Show("กร\u0e38ณาเป\u0e34ดรอบบ\u0e34ลก\u0e48อนทำรายการ");
		}
		else
		{
			if (e.Button != MouseButtons.Left || NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("จอง") == 0)
			{
				return;
			}
			if ((NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0) | (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0))
			{
				MyProject.Forms.ClickUSE3_0.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
				MyProject.Forms.ClickUSE3_0.RoomArr = CHK_Array;
				MyProject.Forms.ClickUSE3_0.ShowDialog();
				if (MyProject.Forms.ClickUSE3_0.ISOK)
				{
					LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
				}
				ClearCheck();
			}
			else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
			{
				MyProject.Forms.ClickUSE3_0.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
				MyProject.Forms.ClickUSE3_0.RoomArr = CHK_Array;
				MyProject.Forms.ClickUSE3_0.ShowDialog();
				if (MyProject.Forms.ClickUSE3_0.ISOK)
				{
					LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
				}
				ClearCheck();
			}
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
				CHK_Array.Add(RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)));
				DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT * FROM HT_CheckIn_Ds WHERE Cin_Room_Status<>'Check-Out' and (Cin_No IN (SELECT Cin_No FROM HT_CheckIn_Ds AS HT_CheckIn_Ds_1  WHERE (Cin_Room_No = '", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), "') AND (Cin_Room_Status <> 'Check-Out')))")));
				if (dataSet.Tables[0].Rows.Count != 0)
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
						int num8 = FlowLayoutPanel1.Controls.Count - 1;
						int num9 = 0;
						while (true)
						{
							int num10 = num9;
							num4 = num8;
							if (num10 <= num4)
							{
								if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num9].Controls[1].Controls[0].Name, dataSet.Tables[0].Rows[num6]["Cin_Room_No"], TextCompare: false))
								{
									num9++;
									continue;
								}
								FlowLayoutPanel1.Controls[num9].Controls[1].Controls[0].Visible = true;
								FlowLayoutPanel1.Controls[num9].Visible = true;
								break;
							}
							break;
						}
						num6++;
					}
					return;
				}
				string right = "";
				int num11 = FlowLayoutPanel1.Controls.Count - 1;
				int num12 = 0;
				while (true)
				{
					int num13 = num12;
					int num4 = num11;
					if (num13 <= num4)
					{
						if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num12].Controls[1].Controls[0].Name, NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), TextCompare: false))
						{
							num12++;
							continue;
						}
						right = FlowLayoutPanel1.Controls[num12].Controls[2].Controls[2].Text;
						break;
					}
					break;
				}
				int num14 = FlowLayoutPanel1.Controls.Count - 1;
				int num15 = 0;
				while (true)
				{
					int num16 = num15;
					int num4 = num14;
					if (num16 <= num4)
					{
						if (Operators.CompareString(FlowLayoutPanel1.Controls[num15].Controls[2].Controls[2].Text, right, TextCompare: false) == 0)
						{
							FlowLayoutPanel1.Controls[num15].Controls[1].Controls[0].Visible = true;
							FlowLayoutPanel1.Controls[num15].Visible = true;
						}
						num15++;
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
				int num17 = FlowLayoutPanel1.Controls.Count - 1;
				int num18 = 0;
				while (true)
				{
					int num19 = num18;
					int num4 = num17;
					if (num19 <= num4)
					{
						FlowLayoutPanel1.Controls[num18].Controls[1].Controls[0].Visible = true;
						FlowLayoutPanel1.Controls[num18].Visible = true;
						num18++;
						continue;
					}
					break;
				}
				return;
			}
			ButtonX6.Visible = true;
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT * FROM HT_CheckIn_Ds WHERE  Cin_Room_Status<>'Check-Out' and  (Cin_No IN (SELECT Cin_No FROM HT_CheckIn_Ds AS HT_CheckIn_Ds_1  WHERE (Cin_Room_No = '", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), "') AND (Cin_Room_Status <> 'Check-Out')))")));
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				int num20 = dataSet2.Tables[0].Rows.Count - 1;
				int num21 = 0;
				while (true)
				{
					int num22 = num21;
					int num4 = num20;
					if (num22 > num4)
					{
						break;
					}
					int num23 = FlowLayoutPanel1.Controls.Count - 1;
					int num24 = 0;
					while (true)
					{
						int num25 = num24;
						num4 = num23;
						if (num25 <= num4)
						{
							if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num24].Controls[1].Controls[0].Name, dataSet2.Tables[0].Rows[num21]["Cin_Room_No"], TextCompare: false))
							{
								num24++;
								continue;
							}
							FlowLayoutPanel1.Controls[num24].Controls[1].Controls[0].Visible = true;
							FlowLayoutPanel1.Controls[num24].Visible = true;
							break;
						}
						break;
					}
					num21++;
				}
				return;
			}
			string right2 = "";
			int num26 = FlowLayoutPanel1.Controls.Count - 1;
			int num27 = 0;
			while (true)
			{
				int num28 = num27;
				int num4 = num26;
				if (num28 <= num4)
				{
					if (!Operators.ConditionalCompareObjectEqual(FlowLayoutPanel1.Controls[num27].Controls[1].Controls[0].Name, NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), TextCompare: false))
					{
						num27++;
						continue;
					}
					right2 = FlowLayoutPanel1.Controls[num27].Controls[2].Controls[2].Text;
					break;
				}
				break;
			}
			int num29 = FlowLayoutPanel1.Controls.Count - 1;
			int num30 = 0;
			while (true)
			{
				int num31 = num30;
				int num4 = num29;
				if (num31 <= num4)
				{
					if (Operators.CompareString(FlowLayoutPanel1.Controls[num30].Controls[2].Controls[2].Text, right2, TextCompare: false) == 0)
					{
						FlowLayoutPanel1.Controls[num30].Controls[1].Controls[0].Visible = true;
						FlowLayoutPanel1.Controls[num30].Visible = true;
					}
					num30++;
					continue;
				}
				break;
			}
		}
	}

	private void nodebtn_Book_Down(object sender, MouseEventArgs e)
	{
		Timer1.Enabled = false;
		if (!((e.Button == MouseButtons.Left) & (Cursor == Cursors.Default)))
		{
			return;
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select Book_type from View_Book_Date where id=", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null))));
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
		IS_LIST_ROOM = false;
		Cursor = Cursors.WaitCursor;
		SELECT_ROOM_NOW = "";
		DateTime dateTime = DateTimePicker1.Value;
		if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), new decimal(Module1.CHK_IN_Before)) < 0))
		{
			dateTime = dateTime.AddDays(-1.0);
		}
		PanelEx_0.Text = "  สถานะห\u0e49องพ\u0e31กของค\u0e37นว\u0e31นท\u0e35\u0e48 " + Strings.Format(dateTime.Date, "dd/MM/yyyy");
		if (!IS_LIST_ROOM)
		{
			FlowLayoutPanel1.Controls.Clear();
		}
		DataSet dataSet = Module1.connect("select * From HT_Rooms  where room_use='yes'  order by room_NO");
		DataSet dataSet2 = Module1.connect("select * from View_Room_All where room_date='" + Conversions.ToString(dateTime.Date) + "'");
		DataSet dataSet3 = Module1.connect("select * from View_Book_Ds2 where book_status='1จอง'");
		DataSet dataSet4 = Module1.connect("select cin_no,cin_room_all,Cin_room_no,Total_Price_Balance from View_CheckIn_Ds where Total_Price_Balance>0 and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		DataSet dataSet5 = Module1.connect("select SMS_Room,SMS_ID from HT_Room_SMS where SMS_Readed='no'");
		DataSet dataSet6 = Module1.connect("select * from View_Room_All where cin_room_status='เข\u0e49าพ\u0e31ก' order by cin_date_out desc");
		string text = "";
		ArrayList arrayList = new ArrayList();
		ArrayList arrayList2 = new ArrayList();
		DataSet dataSet7 = Module1.connect("select * from HT_SET_RoomType");
		checked
		{
			int num = dataSet7.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				string[] value = new string[6]
				{
					Conversions.ToString(dataSet7.Tables[0].Rows[num2]["name"]),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0)
				};
				arrayList2.Add(value);
				num2++;
			}
			int num5 = dataSet3.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				string[] value2 = new string[2]
				{
					Conversions.ToString(dataSet3.Tables[0].Rows[num6]["book_room_type"]),
					Conversions.ToString(dataSet3.Tables[0].Rows[num6]["num"])
				};
				arrayList.Add(value2);
				num6++;
			}
			if (DateTime.Compare(DateTimePicker1.Value.Date, DateTime.Now.Date) >= 0)
			{
				int num8 = dataSet.Tables[0].Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					int num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					text = "";
					object obj = "\r\n";
					object obj2 = "ว\u0e48าง";
					string r_NO = Conversions.ToString(dataSet.Tables[0].Rows[num9]["room_NO"]);
					string text2 = Conversions.ToString(dataSet.Tables[0].Rows[num9]["Room_Type"]);
					object objectValue = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["Room_Polity"]);
					int num11 = Conversions.ToInteger(dataSet.Tables[0].Rows[num9]["Room_x"]);
					int num12 = Conversions.ToInteger(dataSet.Tables[0].Rows[num9]["Room_y"]);
					string right = "";
					int num13 = dataSet2.Tables[0].Rows.Count - 1;
					int num14 = 0;
					while (true)
					{
						int num15 = num14;
						num4 = num13;
						if (num15 > num4)
						{
							break;
						}
						if (!Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num14]["room_no"], dataSet.Tables[0].Rows[num9]["room_NO"], TextCompare: false))
						{
							num14++;
							continue;
						}
						obj2 = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num14]["room_status"]);
						obj = Operators.ConcatenateObject("\r\n", dataSet2.Tables[0].Rows[num14]["room_details"]);
						right = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num14]["Cin_room_out"]), "dd-MM-yy HH:mm");
						Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num14]["Cin_room_out"]), "HH:mm");
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num14]["Cin_Room_Status"], "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก", TextCompare: false))
						{
							obj2 = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
						}
						text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num14]["cust_name"], "\r\n"), dataSet2.Tables[0].Rows[num14]["cust_work_name"]), "\r\n"), "เลขท\u0e35\u0e48 "), dataSet2.Tables[0].Rows[num14]["room_checkin_no"]), "\r\n"), "ห\u0e49อง "), dataSet2.Tables[0].Rows[num14]["cin_room_all"]));
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(obj2, "Check Out", TextCompare: false))
					{
						obj2 = "ว\u0e48าง";
						text = "ว\u0e48าง";
					}
					if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(obj2, "ว\u0e48าง", TextCompare: false), Operators.CompareObjectEqual(obj2, "จอง", TextCompare: false))))
					{
						text = dataSet.Tables[0].Rows[num9]["room_book_ds"].ToString();
						if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num9]["room_use"], "yes", TextCompare: false))
						{
							string text3 = "";
							int num16 = dataSet6.Tables[0].Rows.Count - 1;
							int num17 = 0;
							while (true)
							{
								int num18 = num17;
								num4 = num16;
								if (num18 <= num4)
								{
									if (!Operators.ConditionalCompareObjectEqual(dataSet6.Tables[0].Rows[num17]["room_no"], dataSet.Tables[0].Rows[num9]["room_no"], TextCompare: false))
									{
										num17++;
										continue;
									}
									text3 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet6.Tables[0].Rows[num17]["cin_date_out"]), "dd-MM-yy HH:mm");
									text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet6.Tables[0].Rows[num17]["cust_name"], "\r\n"), dataSet6.Tables[0].Rows[num17]["cust_work_name"]), "\r\n"), "เลขท\u0e35\u0e48 "), dataSet6.Tables[0].Rows[num17]["room_checkin_no"]), "\r\n"), "ห\u0e49อง "), dataSet6.Tables[0].Rows[num17]["cin_room_all"]));
									break;
								}
								break;
							}
							obj2 = "ย\u0e31งไม\u0e48ได\u0e49 Check-Out\r\nออกว\u0e31นท\u0e35\u0e48\r\n" + text3;
						}
						if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num9]["room_clean"], "yes", TextCompare: false))
						{
							text = "รอ ทำความสะอาด";
							if (Operators.CompareString(dataSet.Tables[0].Rows[num9]["Room_Clean_Time"].ToString(), "", TextCompare: false) != 0)
							{
								int num19 = (int)DateAndTime.DateDiff(DateInterval.Minute, DateTime.Now, DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num9]["Room_Clean_Time"])).AddMinutes(Convert.ToDouble(Module1.decimal_1)));
								text = "กำล\u0e31ง ทำความสะอาด\r\nเหล\u0e37อ " + Conversions.ToString(num19) + " นาท\u0e35";
							}
						}
					}
					if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(obj2, "เข\u0e49าพ\u0e31ก", TextCompare: false), Operators.CompareObjectEqual(obj2, "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก", TextCompare: false))))
					{
						obj2 = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(obj2, "\r\n"), "ออกว\u0e31นท\u0e35\u0e48"), "\r\n"), right);
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num9]["room_clean"], "yes", TextCompare: false))
					{
						obj2 = "รอ ทำความสะอาด";
						text = "รอ ทำความสะอาด";
						if (Operators.CompareString(dataSet.Tables[0].Rows[num9]["Room_Clean_Time"].ToString(), "", TextCompare: false) != 0)
						{
							int num20 = (int)DateAndTime.DateDiff(DateInterval.Minute, DateTime.Now, DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num9]["Room_Clean_Time"])).AddMinutes(Convert.ToDouble(Module1.decimal_1)));
							obj2 = "กำล\u0e31ง ทำความสะอาด\r\nเหล\u0e37อ " + Conversions.ToString(num20) + " นาท\u0e35";
							text = "กำล\u0e31ง ทำความสะอาด\r\nเหล\u0e37อ " + Conversions.ToString(num20) + " นาท\u0e35";
						}
					}
					if (Operators.CompareString(dataSet.Tables[0].Rows[num9]["room_book"].ToString(), "", TextCompare: false) != 0)
					{
						obj2 = "จอง\r\n" + dataSet.Tables[0].Rows[num9]["room_book_name"].ToString() + "\r\nเวลา : " + dataSet.Tables[0].Rows[num9]["room_book_time"].ToString();
					}
					if (Operators.CompareString(dataSet.Tables[0].Rows[num9]["Room_Manternace"].ToString(), "yes", TextCompare: false) == 0)
					{
						obj2 = "ซ\u0e48อม";
						text = "ซ\u0e48อมบำร\u0e38ง";
					}
					int num21 = arrayList.Count - 1;
					int num22 = 0;
					while (true)
					{
						int num23 = num22;
						num4 = num21;
						if (num23 > num4)
						{
							break;
						}
						if (!Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList[num22], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num9]["room_type"], TextCompare: false))
						{
							num22++;
							continue;
						}
						if (decimal.Compare(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num22], new object[1] { 1 }, null)), 0m) > 0)
						{
							NewLateBinding.LateIndexSetComplex(arrayList[num22], new object[2]
							{
								1,
								decimal.Subtract(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList[num22], new object[1] { 1 }, null)), 1m)
							}, null, OptimisticSet: false, RValueBase: true);
						}
						break;
					}
					int num24 = arrayList2.Count - 1;
					int num25 = 0;
					while (true)
					{
						int num26 = num25;
						num4 = num24;
						if (num26 > num4)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 0 }, null), text2, TextCompare: false))
						{
							if (obj2.ToString().IndexOf("จอง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num25], new object[2]
								{
									3,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 3 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("ป\u0e34ดปร\u0e31บปร\u0e38ง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num25], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num25], new object[2]
								{
									2,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 2 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num25], new object[2]
								{
									5,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 5 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("รอ ทำความสะอาด") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num25], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj2.ToString().IndexOf("ว\u0e48าง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList2[num25], new object[2]
								{
									1,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList2[num25], new object[1] { 1 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
						}
						num25++;
					}
					if (!IS_LIST_ROOM)
					{
						if (Operators.CompareString(ComboBoxEx1.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) == 0)
						{
							method_0(r_NO, text2, Conversions.ToString(obj2), Conversions.ToString(obj), num11, num12, Conversions.ToString(objectValue), dataSet4, dataSet5, text);
						}
						else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num9]["Room_Group"], ComboBoxEx1.Text, TextCompare: false))
						{
							method_0(r_NO, text2, Conversions.ToString(obj2), Conversions.ToString(obj), num11, num12, Conversions.ToString(objectValue), dataSet4, dataSet5, text);
						}
					}
					else if (Operators.CompareString(ComboBoxEx1.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) == 0)
					{
						SETButton_Notclear(r_NO, text2, Conversions.ToString(obj2), Conversions.ToString(obj), num11, num12, Conversions.ToString(objectValue), dataSet4, dataSet5, text);
					}
					else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num9]["Room_Group"], ComboBoxEx1.Text, TextCompare: false))
					{
						SETButton_Notclear(r_NO, text2, Conversions.ToString(obj2), Conversions.ToString(obj), num11, num12, Conversions.ToString(objectValue), dataSet4, dataSet5, text);
					}
					num9++;
				}
				arrayList2 = Load_Booking(arrayList2);
				SET_STATUS(arrayList2);
				SET_SMS();
			}
			IS_LIST_ROOM = true;
			dataSet.Dispose();
			dataSet2.Dispose();
			dataSet3.Dispose();
			dataSet4.Dispose();
			dataSet5.Dispose();
			dataSet7.Dispose();
			arrayList.Clear();
			arrayList2.Clear();
			Timer1.Enabled = false;
			Timer1.Enabled = true;
			Cursor = Cursors.Default;
		}
	}

	public ArrayList Load_Booking(ArrayList aa)
	{
		FlowLayoutPanel4.Controls.Clear();
		DateTime dateTime = DateTimePicker1.Value;
		if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
		{
			dateTime = dateTime.AddDays(-1.0);
		}
		int num = 0;
		DataSet dataSet = Module1.connect("select * from View_Book_Date where Book_Status='จอง' and Book_date_ds='" + Conversions.ToString(dateTime.Date) + "' order by id");
		checked
		{
			int num2 = dataSet.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				int num6 = Conversions.ToInteger(Operators.SubtractObject(dataSet.Tables[0].Rows[num3]["book_num"], dataSet.Tables[0].Rows[num3]["book_ok"]));
				int num7 = 1;
				while (true)
				{
					int num8 = num7;
					num5 = num6;
					if (num8 > num5)
					{
						break;
					}
					num++;
					Panel panel = new Panel();
					PanelEx panelEx = new PanelEx();
					PanelEx panelEx2 = new PanelEx();
					panel.Controls.Add(panelEx2);
					panel.Controls.Add(panelEx);
					Point location = new Point(3, 4);
					panel.Location = location;
					System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
					panel.Margin = margin;
					panel.Name = Conversions.ToString(dataSet.Tables[0].Rows[num3]["book_type"]);
					Size size = new Size(120, 80);
					panel.Size = size;
					panel.TabIndex = 0;
					panelEx.Dock = DockStyle.Fill;
					panelEx.CanvasColor = SystemColors.Control;
					panelEx.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
					location = new Point(0, 21);
					panelEx.Location = location;
					margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
					panelEx.Margin = margin;
					panelEx.Name = Conversions.ToString(dataSet.Tables[0].Rows[num3]["id"]);
					size = new Size(125, 80);
					panelEx.Size = size;
					panelEx.Style.Alignment = StringAlignment.Center;
					panelEx.Style.BackColor1.Color = Color.Yellow;
					panelEx.Style.BackColor2.Color = Color.Yellow;
					panelEx.Style.Border = eBorderType.SingleLine;
					panelEx.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
					panelEx.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
					panelEx.Style.GradientAngle = 90;
					panelEx.Style.TextTrimming = StringTrimming.None;
					panelEx.TabIndex = 2;
					panelEx.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("\r\n", dataSet.Tables[0].Rows[num3]["Book_Cust_Name"]), dataSet.Tables[0].Rows[num3]["Book_Cust_Name2"]), "\r\n"), dataSet.Tables[0].Rows[num3]["Book_Cust_Tel"]), "\r\n"), "เวลา : "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Book_Date_in"]), "HH:mm")), "\r\n"), dataSet.Tables[0].Rows[num3]["Book_room_note"]));
					panelEx.Cursor = Cursors.SizeAll;
					panelEx.Font = new Font("Tahoma", 8.5f, FontStyle.Regular, GraphicsUnit.Point, 222);
					panelEx.MouseDown += nodebtn_Book_Down;
					string bodyText = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num3]["Book_Cust_Name"], dataSet.Tables[0].Rows[num3]["Book_Cust_Name2"]), "\r\n"), dataSet.Tables[0].Rows[num3]["Book_Cust_Tel"]), "\r\n"), "เวลาเข\u0e49าพ\u0e31ก : "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Book_Date_in"]), "HH:mm")), "\r\n"), "\r\n"), dataSet.Tables[0].Rows[num3]["Book_room_all"]), "\r\n"), "หมายเหต\u0e38 : "), dataSet.Tables[0].Rows[num3]["Book_room_note"]));
					SuperTooltip1.SetSuperTooltip(panelEx, new SuperTooltipInfo("รายการจองห\u0e49องพ\u0e31ก", "iHOTEL", bodyText, Resources.boy_emoticon_009, null, eTooltipColor.Teal));
					panelEx2.Dock = DockStyle.Top;
					panelEx2.CanvasColor = SystemColors.Control;
					panelEx2.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
					location = new Point(0, 0);
					panelEx2.Location = location;
					margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
					panelEx2.Margin = margin;
					panelEx2.Name = Conversions.ToString(dataSet.Tables[0].Rows[num3]["book_type"]);
					size = new Size(125, 18);
					panelEx2.Size = size;
					panelEx2.Style.Alignment = StringAlignment.Center;
					panelEx2.Style.BackColor1.ColorSchemePart = eColorSchemePart.PanelBackground;
					panelEx2.Style.BackColor2.ColorSchemePart = eColorSchemePart.PanelBackground2;
					panelEx2.Style.Border = eBorderType.SingleLine;
					panelEx2.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
					panelEx2.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
					panelEx2.Style.GradientAngle = 90;
					panelEx2.TabIndex = 0;
					panelEx2.Text = Conversions.ToString(dataSet.Tables[0].Rows[num3]["book_type"]);
					FlowLayoutPanel4.Controls.Add(panel);
					int num9 = aa.Count - 1;
					int num10 = 0;
					while (true)
					{
						int num11 = num10;
						num5 = num9;
						if (num11 > num5)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(aa[num10], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num3]["book_type"], TextCompare: false))
						{
							NewLateBinding.LateIndexSetComplex(aa[num10], new object[2]
							{
								3,
								Operators.AddObject(NewLateBinding.LateIndexGet(aa[num10], new object[1] { 3 }, null), 1)
							}, null, OptimisticSet: false, RValueBase: true);
							NewLateBinding.LateIndexSetComplex(aa[num10], new object[2]
							{
								1,
								Operators.SubtractObject(NewLateBinding.LateIndexGet(aa[num10], new object[1] { 1 }, null), 1)
							}, null, OptimisticSet: false, RValueBase: true);
						}
						num10++;
					}
					num7++;
				}
				num3++;
			}
			PanelEx3.Text = "รายการจองห\u0e49อง ท\u0e35\u0e48ย\u0e31งไม\u0e48ได\u0e49เล\u0e37อกห\u0e49อง (" + Conversions.ToString(num) + " ห\u0e49อง)";
			return aa;
		}
	}

	public void SET_STATUS(ArrayList aa)
	{
		FlowLayoutPanel3.Controls.Clear();
		int num = 0;
		int num2 = 0;
		int num3 = 0;
		int num4 = 0;
		int num5 = 0;
		checked
		{
			int num6 = aa.Count - 1;
			int num7 = 0;
			Point location;
			System.Windows.Forms.Padding margin;
			Size size;
			while (true)
			{
				int num8 = num7;
				int num9 = num6;
				if (num8 > num9)
				{
					break;
				}
				Label label = new Label();
				Label label2 = new Label();
				Label label3 = new Label();
				Label label4 = new Label();
				Label label5 = new Label();
				Label label6 = new Label();
				label.BackColor = Color.White;
				location = new Point(0, 0);
				label.Location = location;
				margin = new System.Windows.Forms.Padding(0);
				label.Margin = margin;
				label.Name = "S1";
				size = new Size(80, 22);
				label.Size = size;
				label.TabIndex = 0;
				label.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num7], new object[1] { 0 }, null));
				label.TextAlign = ContentAlignment.MiddleLeft;
				label2.BackColor = Color.PaleGreen;
				location = new Point(118, 0);
				label2.Location = location;
				margin = new System.Windows.Forms.Padding(0);
				label2.Margin = margin;
				label2.Name = "S2";
				size = new Size(34, 22);
				label2.Size = size;
				label2.TabIndex = 1;
				label2.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num7], new object[1] { 1 }, null));
				label2.TextAlign = ContentAlignment.MiddleCenter;
				label3.BackColor = Color.FromArgb(255, 128, 128);
				location = new Point(152, 0);
				label3.Location = location;
				margin = new System.Windows.Forms.Padding(0);
				label3.Margin = margin;
				label3.Name = "S3";
				size = new Size(41, 22);
				label3.Size = size;
				label3.TabIndex = 2;
				label3.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num7], new object[1] { 2 }, null));
				label3.TextAlign = ContentAlignment.MiddleCenter;
				label4.BackColor = Color.Yellow;
				location = new Point(193, 0);
				label4.Location = location;
				margin = new System.Windows.Forms.Padding(0);
				label4.Margin = margin;
				label4.Name = "S4";
				size = new Size(35, 22);
				label4.Size = size;
				label4.TabIndex = 3;
				label4.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num7], new object[1] { 3 }, null));
				label4.TextAlign = ContentAlignment.MiddleCenter;
				label5.BackColor = Color.FromArgb(255, 192, 128);
				location = new Point(228, 0);
				label5.Location = location;
				margin = new System.Windows.Forms.Padding(0);
				label5.Margin = margin;
				label5.Name = "S5";
				size = new Size(35, 22);
				label5.Size = size;
				label5.TabIndex = 4;
				label5.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num7], new object[1] { 4 }, null));
				label5.TextAlign = ContentAlignment.MiddleCenter;
				label6.BackColor = Color.Aqua;
				location = new Point(118, 0);
				label6.Location = location;
				margin = new System.Windows.Forms.Padding(0);
				label6.Margin = margin;
				label6.Name = "S6";
				size = new Size(34, 22);
				label6.Size = size;
				label6.TabIndex = 1;
				label6.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num7], new object[1] { 5 }, null));
				label6.TextAlign = ContentAlignment.MiddleCenter;
				FlowLayoutPanel3.Controls.Add(label);
				FlowLayoutPanel3.Controls.Add(label6);
				FlowLayoutPanel3.Controls.Add(label2);
				FlowLayoutPanel3.Controls.Add(label3);
				FlowLayoutPanel3.Controls.Add(label4);
				FlowLayoutPanel3.Controls.Add(label5);
				num = Conversions.ToInteger(Operators.AddObject(num, NewLateBinding.LateIndexGet(aa[num7], new object[1] { 1 }, null)));
				num2 = Conversions.ToInteger(Operators.AddObject(num2, NewLateBinding.LateIndexGet(aa[num7], new object[1] { 2 }, null)));
				num3 = Conversions.ToInteger(Operators.AddObject(num3, NewLateBinding.LateIndexGet(aa[num7], new object[1] { 3 }, null)));
				num4 = Conversions.ToInteger(Operators.AddObject(num4, NewLateBinding.LateIndexGet(aa[num7], new object[1] { 4 }, null)));
				num5 = Conversions.ToInteger(Operators.AddObject(num5, NewLateBinding.LateIndexGet(aa[num7], new object[1] { 5 }, null)));
				num7++;
			}
			Label label7 = new Label();
			Label label8 = new Label();
			Label label9 = new Label();
			Label label10 = new Label();
			Label label11 = new Label();
			Label label12 = new Label();
			label7.BackColor = Color.WhiteSmoke;
			label7.ForeColor = Color.FromArgb(0, 0, 192);
			location = new Point(0, 0);
			label7.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			label7.Margin = margin;
			label7.Name = "S1";
			size = new Size(80, 22);
			label7.Size = size;
			label7.TabIndex = 0;
			label7.Text = "รวม (" + Conversions.ToString(num + num2 + num3 + num4 + num5) + ")";
			label7.TextAlign = ContentAlignment.MiddleLeft;
			label8.BackColor = Color.PaleGreen;
			label8.ForeColor = Color.FromArgb(0, 0, 192);
			location = new Point(118, 0);
			label8.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			label8.Margin = margin;
			label8.Name = "S2";
			size = new Size(34, 22);
			label8.Size = size;
			label8.TabIndex = 1;
			label8.Text = Conversions.ToString(num);
			label8.TextAlign = ContentAlignment.MiddleCenter;
			label9.BackColor = Color.FromArgb(255, 128, 128);
			label9.ForeColor = Color.FromArgb(0, 0, 192);
			location = new Point(152, 0);
			label9.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			label9.Margin = margin;
			label9.Name = "S3";
			size = new Size(41, 22);
			label9.Size = size;
			label9.TabIndex = 2;
			label9.Text = Conversions.ToString(num2);
			label9.TextAlign = ContentAlignment.MiddleCenter;
			label10.BackColor = Color.Yellow;
			label10.ForeColor = Color.FromArgb(0, 0, 192);
			location = new Point(193, 0);
			label10.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			label10.Margin = margin;
			label10.Name = "S4";
			size = new Size(35, 22);
			label10.Size = size;
			label10.TabIndex = 3;
			label10.Text = Conversions.ToString(num3);
			label10.TextAlign = ContentAlignment.MiddleCenter;
			label11.BackColor = Color.FromArgb(255, 192, 128);
			label11.ForeColor = Color.FromArgb(0, 0, 192);
			location = new Point(228, 0);
			label11.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			label11.Margin = margin;
			label11.Name = "S5";
			size = new Size(35, 22);
			label11.Size = size;
			label11.TabIndex = 4;
			label11.Text = Conversions.ToString(num4);
			label11.TextAlign = ContentAlignment.MiddleCenter;
			label12.BackColor = Color.Aqua;
			label12.ForeColor = Color.FromArgb(0, 0, 192);
			location = new Point(118, 0);
			label12.Location = location;
			margin = new System.Windows.Forms.Padding(0);
			label12.Margin = margin;
			label12.Name = "S6";
			size = new Size(34, 22);
			label12.Size = size;
			label12.TabIndex = 1;
			label12.Text = Conversions.ToString(num5);
			label12.TextAlign = ContentAlignment.MiddleCenter;
			FlowLayoutPanel3.Controls.Add(label7);
			FlowLayoutPanel3.Controls.Add(label12);
			FlowLayoutPanel3.Controls.Add(label8);
			FlowLayoutPanel3.Controls.Add(label9);
			FlowLayoutPanel3.Controls.Add(label10);
			FlowLayoutPanel3.Controls.Add(label11);
		}
	}

	public void SET_SMS()
	{
		FlowLayoutPanel6.Controls.Clear();
		DataSet dataSet = Module1.connect("select SMS_TO,count(SMS_ID) as num from HT_EMP_SMS where SMS_Readed='no' group by SMS_TO");
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					Label label = new Label();
					Label label2 = new Label();
					label.BackColor = Color.White;
					Point location = new Point(0, 0);
					label.Location = location;
					System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(0);
					label.Margin = margin;
					label.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["SMS_TO"]);
					Size size = new Size(190, 22);
					label.Size = size;
					label.TabIndex = 0;
					label.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["SMS_TO"]);
					label.TextAlign = ContentAlignment.MiddleLeft;
					label.Cursor = Cursors.Hand;
					label2.BackColor = Color.PaleGreen;
					location = new Point(118, 0);
					label2.Location = location;
					margin = new System.Windows.Forms.Padding(0);
					label2.Margin = margin;
					label2.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["SMS_TO"]);
					size = new Size(68, 22);
					label2.Size = size;
					label2.TabIndex = 1;
					label2.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["num"]);
					label2.TextAlign = ContentAlignment.MiddleCenter;
					label.Cursor = Cursors.Hand;
					label.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
					{
						EMP_NOTE_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
					};
					label2.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
					{
						EMP_NOTE_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
					};
					FlowLayoutPanel6.Controls.Add(label);
					FlowLayoutPanel6.Controls.Add(label2);
					num2++;
					continue;
				}
				break;
			}
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

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		DateTimePicker1.Value = DateTime.Now;
		ClearCheck();
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		if (!ButtonX4.Checked)
		{
			DateTimePicker1.Value = DateTime.Now;
			LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		checked
		{
			if (FlowLayoutPanel3.VerticalScroll.Maximum > FlowLayoutPanel3.VerticalScroll.Value + 30)
			{
				FlowLayoutPanel3.VerticalScroll.Value += 10;
				FlowLayoutPanel3.VerticalScroll.Value += 10;
				FlowLayoutPanel3.VerticalScroll.Value += 10;
			}
			else
			{
				FlowLayoutPanel3.VerticalScroll.Value = FlowLayoutPanel3.VerticalScroll.Maximum;
			}
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		checked
		{
			if (FlowLayoutPanel3.VerticalScroll.Minimum < FlowLayoutPanel3.VerticalScroll.Value - 30)
			{
				FlowLayoutPanel3.VerticalScroll.Value -= 10;
				FlowLayoutPanel3.VerticalScroll.Value -= 10;
				FlowLayoutPanel3.VerticalScroll.Value -= 10;
			}
			else
			{
				FlowLayoutPanel3.VerticalScroll.Value = FlowLayoutPanel3.VerticalScroll.Minimum;
			}
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSearchBook.ShowDialog();
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
		Module1.Set_room_pority();
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

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportCoupon.ShowDialog();
	}
}
