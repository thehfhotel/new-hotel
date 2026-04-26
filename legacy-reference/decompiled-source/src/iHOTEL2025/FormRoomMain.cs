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
public class FormRoomMain : Office2007Form
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

	[AccessedThroughProperty("ButtonX8")]
	private ButtonX _ButtonX8;

	[AccessedThroughProperty("Timer4")]
	private Timer _Timer4;

	[AccessedThroughProperty("ButtonX9")]
	private ButtonX _ButtonX9;

	[AccessedThroughProperty("ButtonX10")]
	private ButtonX _ButtonX10;

	[AccessedThroughProperty("ButtonX11")]
	private ButtonX _ButtonX11;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("PanelEx4")]
	private PanelEx _PanelEx4;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ButtonX12")]
	private ButtonX _ButtonX12;

	[AccessedThroughProperty("TimerSearch")]
	private Timer _TimerSearch;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("Panel3")]
	private Panel _Panel3;

	[AccessedThroughProperty("Panel4")]
	private Panel _Panel4;

	[AccessedThroughProperty("Panel5")]
	private Panel _Panel5;

	[AccessedThroughProperty("Panel6")]
	private Panel _Panel6;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	private ResizeableControl rc;

	private string SELECT_ROOM_NOW;

	private ArrayList ArrTip;

	private int TipNow;

	public bool showFull;

	private bool IS_LIST_ROOM;

	private DateTime lasttime_update_pwer;

	private int UpdateSecond;

	private ArrayList arrroomboc;

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
			EventHandler value2 = BOOK_PANEL_Click;
			MouseEventHandler value3 = PanelEx3_MouseDown;
			if (_BOOK_PANEL != null)
			{
				_BOOK_PANEL.Click -= value2;
				_BOOK_PANEL.MouseDown -= value3;
			}
			_BOOK_PANEL = value;
			if (_BOOK_PANEL != null)
			{
				_BOOK_PANEL.Click += value2;
				_BOOK_PANEL.MouseDown += value3;
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

	internal virtual ButtonX ButtonX8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX8_Click;
			if (_ButtonX8 != null)
			{
				_ButtonX8.Click -= value2;
			}
			_ButtonX8 = value;
			if (_ButtonX8 != null)
			{
				_ButtonX8.Click += value2;
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

	internal virtual ButtonX ButtonX9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX9_Click;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click -= value2;
			}
			_ButtonX9 = value;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX10_Click;
			if (_ButtonX10 != null)
			{
				_ButtonX10.Click -= value2;
			}
			_ButtonX10 = value;
			if (_ButtonX10 != null)
			{
				_ButtonX10.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX11_Click;
			if (_ButtonX11 != null)
			{
				_ButtonX11.Click -= value2;
			}
			_ButtonX11 = value;
			if (_ButtonX11 != null)
			{
				_ButtonX11.Click += value2;
			}
		}
	}

	internal virtual TextBox TextBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TextBox1_TextChanged;
			EventHandler value3 = TextBox1_GotFocus;
			if (_TextBox1 != null)
			{
				_TextBox1.TextChanged -= value2;
				_TextBox1.GotFocus -= value3;
			}
			_TextBox1 = value;
			if (_TextBox1 != null)
			{
				_TextBox1.TextChanged += value2;
				_TextBox1.GotFocus += value3;
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

	internal virtual PanelEx PanelEx4
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx4 = value;
		}
	}

	internal virtual ListView ListView1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListView1_DoubleClick;
			EventHandler value3 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick -= value2;
				_ListView1.SelectedIndexChanged -= value3;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick += value2;
				_ListView1.SelectedIndexChanged += value3;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader1 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader2 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader3 = value;
		}
	}

	internal virtual ButtonX ButtonX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX12_Click_1;
			if (_ButtonX12 != null)
			{
				_ButtonX12.Click -= value2;
			}
			_ButtonX12 = value;
			if (_ButtonX12 != null)
			{
				_ButtonX12.Click += value2;
			}
		}
	}

	internal virtual Timer TimerSearch
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerSearch;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerSearch_Tick;
			if (_TimerSearch != null)
			{
				_TimerSearch.Tick -= value2;
			}
			_TimerSearch = value;
			if (_TimerSearch != null)
			{
				_TimerSearch.Tick += value2;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader4 = value;
		}
	}

	internal virtual Panel Panel3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel3 = value;
		}
	}

	internal virtual Panel Panel4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel4 = value;
		}
	}

	internal virtual Panel Panel5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel5 = value;
		}
	}

	internal virtual Panel Panel6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel6 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader5 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader6 = value;
		}
	}

	[DebuggerNonUserCode]
	static FormRoomMain()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormRoomMain()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormRoomMain_Load;
		base.Activated += FormRoomMain_Activated;
		base.Deactivate += FormRoomMain_Deactivate;
		base.FormClosing += FormRoomMain_FormClosing;
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
		UpdateSecond = 30;
		arrroomboc = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormRoomMain));
		this.PanelEx_0 = new DevComponents.DotNetBar.PanelEx();
		this.ComboBoxEx1 = new DevComponents.DotNetBar.Controls.ComboBoxEx();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.FlowLayoutPanel1 = new System.Windows.Forms.Panel();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.PanelCenter = new DevComponents.DotNetBar.PanelEx();
		this.Panel_Nofi = new System.Windows.Forms.FlowLayoutPanel();
		this.Panel2 = new System.Windows.Forms.Panel();
		this.Panel3 = new System.Windows.Forms.Panel();
		this.Panel4 = new System.Windows.Forms.Panel();
		this.Panel5 = new System.Windows.Forms.Panel();
		this.Panel6 = new System.Windows.Forms.Panel();
		this.PanelNum = new DevComponents.DotNetBar.PanelEx();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.PanelTOP = new DevComponents.DotNetBar.PanelEx();
		this.CheckBoxX_0 = new DevComponents.DotNetBar.Controls.CheckBoxX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.GroupPanel1 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.FlowLayoutPanel6 = new System.Windows.Forms.FlowLayoutPanel();
		this.FlowLayoutPanel5 = new System.Windows.Forms.FlowLayoutPanel();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
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
		this.ButtonX8 = new DevComponents.DotNetBar.ButtonX();
		this.Timer4 = new System.Windows.Forms.Timer(this.components);
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.PanelEx4 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX_2 = new DevComponents.DotNetBar.ButtonX();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.TimerSearch = new System.Windows.Forms.Timer(this.components);
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX9 = new DevComponents.DotNetBar.ButtonX();
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
		this.PanelEx4.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx_0.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_0.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx_0.Controls.Add(this.ComboBoxEx1);
		this.PanelEx_0.Controls.Add(this.ButtonX7);
		this.PanelEx_0.Controls.Add(this.PanelEx1);
		this.PanelEx_0.Controls.Add(this.LabelX1);
		this.PanelEx_0.Controls.Add(this.ButtonX6);
		this.PanelEx_0.Controls.Add(this.ButtonX4);
		this.PanelEx_0.Controls.Add(this.ButtonX3);
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
		location = new System.Drawing.Point(640, 6);
		comboBoxEx.Location = location;
		this.ComboBoxEx1.Name = "ComboBoxEx1";
		DevComponents.DotNetBar.Controls.ComboBoxEx comboBoxEx2 = this.ComboBoxEx1;
		size = new System.Drawing.Size(152, 27);
		comboBoxEx2.Size = size;
		this.ComboBoxEx1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ComboBoxEx1.TabIndex = 10;
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.FocusCuesEnabled = false;
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX7;
		location = new System.Drawing.Point(797, 5);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX7;
		size = new System.Drawing.Size(81, 28);
		buttonX3.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 13;
		this.ButtonX7.Text = "เต\u0e47มจอ";
		this.PanelEx1.AllowDrop = true;
		this.PanelEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Cursor = System.Windows.Forms.Cursors.Hand;
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx1;
		location = new System.Drawing.Point(4, 5);
		panelEx4.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx1;
		size = new System.Drawing.Size(333, 29);
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
		location = new System.Drawing.Point(541, 8);
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
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX6;
		location = new System.Drawing.Point(343, 4);
		buttonX4.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX6;
		size = new System.Drawing.Size(178, 30);
		buttonX5.Size = size;
		this.ButtonX6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX6.TabIndex = 9;
		this.ButtonX6.Text = "ยกเล\u0e34กการเล\u0e37อก";
		this.ButtonX6.Visible = false;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX4;
		location = new System.Drawing.Point(882, 5);
		buttonX6.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX4;
		size = new System.Drawing.Size(136, 28);
		buttonX7.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 4;
		this.ButtonX4.Text = "แก\u0e49ไขแผนผ\u0e31ง";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX3;
		location = new System.Drawing.Point(1022, 5);
		buttonX8.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX9.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX3;
		size = new System.Drawing.Size(109, 28);
		buttonX10.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 3;
		this.ButtonX3.Text = "Refresh";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX2;
		location = new System.Drawing.Point(691, 7);
		buttonX11.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX12.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX2;
		size = new System.Drawing.Size(29, 27);
		buttonX13.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Visible = false;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX1;
		location = new System.Drawing.Point(458, 7);
		buttonX14.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX15.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX1;
		size = new System.Drawing.Size(29, 27);
		buttonX16.Size = size;
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
		System.Windows.Forms.Panel flowLayoutPanel = this.FlowLayoutPanel1;
		location = new System.Drawing.Point(0, 38);
		flowLayoutPanel.Location = location;
		System.Windows.Forms.Panel flowLayoutPanel2 = this.FlowLayoutPanel1;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel2.Margin = margin;
		this.FlowLayoutPanel1.Name = "FlowLayoutPanel1";
		System.Windows.Forms.Panel flowLayoutPanel3 = this.FlowLayoutPanel1;
		size = new System.Drawing.Size(860, 614);
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
		this.Panel1.Visible = false;
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
		this.PanelCenter.Style.BackColor1.Color = System.Drawing.Color.MediumAquamarine;
		this.PanelCenter.Style.BackColor2.Color = System.Drawing.Color.White;
		this.PanelCenter.Style.BackgroundImage = iHOTEL2025.My.Resources.Resources.an4_40;
		this.PanelCenter.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.TopLeft;
		this.PanelCenter.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelCenter.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelCenter.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelCenter.Style.GradientAngle = 200;
		this.PanelCenter.Style.TextTrimming = System.Drawing.StringTrimming.None;
		this.PanelCenter.TabIndex = 2;
		this.Panel_Nofi.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Panel_Nofi.Controls.Add(this.Panel2);
		this.Panel_Nofi.Controls.Add(this.Panel3);
		this.Panel_Nofi.Controls.Add(this.Panel4);
		this.Panel_Nofi.Controls.Add(this.Panel5);
		this.Panel_Nofi.Controls.Add(this.Panel6);
		System.Windows.Forms.FlowLayoutPanel panel_Nofi = this.Panel_Nofi;
		location = new System.Drawing.Point(0, 71);
		panel_Nofi.Location = location;
		this.Panel_Nofi.Name = "Panel_Nofi";
		System.Windows.Forms.FlowLayoutPanel panel_Nofi2 = this.Panel_Nofi;
		size = new System.Drawing.Size(125, 20);
		panel_Nofi2.Size = size;
		this.Panel_Nofi.TabIndex = 2;
		this.Panel2.BackgroundImage = iHOTEL2025.My.Resources.Resources.vat7;
		this.Panel2.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.Panel panel4 = this.Panel2;
		location = new System.Drawing.Point(5, 0);
		panel4.Location = location;
		System.Windows.Forms.Panel panel5 = this.Panel2;
		margin = new System.Windows.Forms.Padding(5, 0, 0, 0);
		panel5.Margin = margin;
		this.Panel2.Name = "Panel2";
		System.Windows.Forms.Panel panel6 = this.Panel2;
		size = new System.Drawing.Size(32, 20);
		panel6.Size = size;
		this.Panel2.TabIndex = 0;
		this.Panel3.BackgroundImage = iHOTEL2025.My.Resources.Resources.vat7;
		this.Panel3.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.Panel panel7 = this.Panel3;
		location = new System.Drawing.Point(37, 0);
		panel7.Location = location;
		System.Windows.Forms.Panel panel8 = this.Panel3;
		margin = new System.Windows.Forms.Padding(0);
		panel8.Margin = margin;
		this.Panel3.Name = "Panel3";
		System.Windows.Forms.Panel panel9 = this.Panel3;
		size = new System.Drawing.Size(18, 20);
		panel9.Size = size;
		this.Panel3.TabIndex = 1;
		this.Panel4.BackgroundImage = iHOTEL2025.My.Resources.Resources.vat7;
		this.Panel4.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.Panel panel10 = this.Panel4;
		location = new System.Drawing.Point(60, 0);
		panel10.Location = location;
		System.Windows.Forms.Panel panel11 = this.Panel4;
		margin = new System.Windows.Forms.Padding(5, 0, 0, 0);
		panel11.Margin = margin;
		this.Panel4.Name = "Panel4";
		System.Windows.Forms.Panel panel12 = this.Panel4;
		size = new System.Drawing.Size(18, 20);
		panel12.Size = size;
		this.Panel4.TabIndex = 2;
		this.Panel5.BackgroundImage = iHOTEL2025.My.Resources.Resources.vat7;
		this.Panel5.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.Panel panel13 = this.Panel5;
		location = new System.Drawing.Point(83, 0);
		panel13.Location = location;
		System.Windows.Forms.Panel panel14 = this.Panel5;
		margin = new System.Windows.Forms.Padding(5, 0, 0, 0);
		panel14.Margin = margin;
		this.Panel5.Name = "Panel5";
		System.Windows.Forms.Panel panel15 = this.Panel5;
		size = new System.Drawing.Size(18, 20);
		panel15.Size = size;
		this.Panel5.TabIndex = 3;
		this.Panel6.BackgroundImage = iHOTEL2025.My.Resources.Resources.vat7;
		this.Panel6.BackgroundImageLayout = System.Windows.Forms.ImageLayout.None;
		System.Windows.Forms.Panel panel16 = this.Panel6;
		location = new System.Drawing.Point(106, 0);
		panel16.Location = location;
		System.Windows.Forms.Panel panel17 = this.Panel6;
		margin = new System.Windows.Forms.Padding(5, 0, 0, 0);
		panel17.Margin = margin;
		this.Panel6.Name = "Panel6";
		System.Windows.Forms.Panel panel18 = this.Panel6;
		size = new System.Drawing.Size(18, 20);
		panel18.Size = size;
		this.Panel6.TabIndex = 4;
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
		location = new System.Drawing.Point(0, 6);
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
		this.PanelTOP.Style.CornerType = DevComponents.DotNetBar.eCornerType.Rounded;
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
		this.Timer1.Interval = 60560;
		this.GroupPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel1.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel1.Controls.Add(this.FlowLayoutPanel6);
		this.GroupPanel1.Controls.Add(this.FlowLayoutPanel5);
		this.GroupPanel1.Controls.Add(this.ButtonX5);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel = this.GroupPanel1;
		location = new System.Drawing.Point(863, 89);
		groupPanel.Location = location;
		this.GroupPanel1.Name = "GroupPanel1";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel2 = this.GroupPanel1;
		size = new System.Drawing.Size(272, 89);
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
		this.FlowLayoutPanel6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.FlowLayoutPanel6.AutoScroll = true;
		this.FlowLayoutPanel6.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel4 = this.FlowLayoutPanel6;
		location = new System.Drawing.Point(4, 28);
		flowLayoutPanel4.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel5 = this.FlowLayoutPanel6;
		margin = new System.Windows.Forms.Padding(0);
		flowLayoutPanel5.Margin = margin;
		this.FlowLayoutPanel6.Name = "FlowLayoutPanel6";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel6 = this.FlowLayoutPanel6;
		size = new System.Drawing.Size(295, 26);
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
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.FocusCuesEnabled = false;
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX5;
		location = new System.Drawing.Point(3, 62);
		buttonX17.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX18 = this.ButtonX5;
		size = new System.Drawing.Size(266, 20);
		buttonX18.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.Windows7;
		this.ButtonX5.TabIndex = 4;
		this.ButtonX5.Text = "เข\u0e35ยนข\u0e49อความ";
		this.GroupPanel2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel2.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel2.Controls.Add(this.Button2);
		this.GroupPanel2.Controls.Add(this.Button1);
		this.GroupPanel2.Controls.Add(this.FlowLayoutPanel3);
		this.GroupPanel2.Controls.Add(this.FlowLayoutPanel2);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel3 = this.GroupPanel2;
		location = new System.Drawing.Point(863, 407);
		groupPanel3.Location = location;
		this.GroupPanel2.Name = "GroupPanel2";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel4 = this.GroupPanel2;
		size = new System.Drawing.Size(272, 209);
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
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button2.Image = (System.Drawing.Image)resources.GetObject("Button2.Image");
		System.Windows.Forms.Button button = this.Button2;
		location = new System.Drawing.Point(242, 176);
		button.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button2 = this.Button2;
		size = new System.Drawing.Size(23, 21);
		button2.Size = size;
		this.Button2.TabIndex = 2;
		this.Button2.UseVisualStyleBackColor = true;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button1.Image = (System.Drawing.Image)resources.GetObject("Button1.Image");
		System.Windows.Forms.Button button3 = this.Button1;
		location = new System.Drawing.Point(220, 176);
		button3.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button4 = this.Button1;
		size = new System.Drawing.Size(23, 21);
		button4.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.UseVisualStyleBackColor = true;
		this.FlowLayoutPanel3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
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
		size = new System.Drawing.Size(295, 149);
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
		this.S1.BackColor = System.Drawing.Color.FromArgb(215, 215, 215);
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
		this.S6.BackColor = System.Drawing.Color.FromArgb(0, 225, 225);
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
		this.S2.BackColor = System.Drawing.Color.FromArgb(122, 221, 122);
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
		this.S3.BackColor = System.Drawing.Color.FromArgb(225, 98, 98);
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
		this.S4.BackColor = System.Drawing.Color.FromArgb(225, 225, 0);
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
		this.S5.BackColor = System.Drawing.Color.FromArgb(225, 162, 98);
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
		this.SuperTooltip1.MaximumWidth = 500;
		DevComponents.DotNetBar.SuperTooltip superTooltip = this.SuperTooltip1;
		size = new System.Drawing.Size(0, 0);
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
		location = new System.Drawing.Point(863, 182);
		panelEx9.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx10 = this.PanelEx3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx10.Margin = margin;
		this.PanelEx3.Name = "PanelEx3";
		DevComponents.DotNetBar.PanelEx panelEx11 = this.PanelEx3;
		size = new System.Drawing.Size(272, 20);
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
		this.FlowLayoutPanel4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.FlowLayoutPanel4.AutoScroll = true;
		this.FlowLayoutPanel4.BackColor = System.Drawing.Color.White;
		this.FlowLayoutPanel4.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.FlowLayoutPanel4.Controls.Add(this.BOOK_MAIN);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel16 = this.FlowLayoutPanel4;
		location = new System.Drawing.Point(863, 202);
		flowLayoutPanel16.Location = location;
		this.FlowLayoutPanel4.Name = "FlowLayoutPanel4";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel17 = this.FlowLayoutPanel4;
		size = new System.Drawing.Size(272, 202);
		flowLayoutPanel17.Size = size;
		this.FlowLayoutPanel4.TabIndex = 8;
		this.TimerPority.Enabled = true;
		this.TimerPority.Interval = 18000000;
		this.Timer3.Interval = 1000;
		this.ButtonX8.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX8.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX8.BackColor = System.Drawing.Color.FromArgb(255, 255, 128);
		this.ButtonX8.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX8.FocusCuesEnabled = false;
		this.ButtonX8.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX19 = this.ButtonX8;
		location = new System.Drawing.Point(859, 622);
		buttonX19.Location = location;
		this.ButtonX8.Name = "ButtonX8";
		this.ButtonX8.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX20 = this.ButtonX8;
		size = new System.Drawing.Size(35, 30);
		buttonX20.Size = size;
		this.ButtonX8.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX8.TabIndex = 3;
		this.ButtonX8.Text = "ขยาย\r\n>>";
		this.ButtonX8.Tooltip = "ขยายให\u0e49เต\u0e47ม";
		this.Timer4.Interval = 5000;
		this.TextBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.TextBox1.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		location = new System.Drawing.Point(863, 62);
		textBox.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		size = new System.Drawing.Size(272, 23);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 11;
		this.LabelX3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.LabelX3.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX3;
		location = new System.Drawing.Point(865, 38);
		labelX5.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX3;
		size = new System.Drawing.Size(195, 23);
		labelX6.Size = size;
		this.LabelX3.TabIndex = 12;
		this.LabelX3.Text = "ค\u0e49นหาช\u0e37\u0e48อ/ทะเบ\u0e35ยนรถ/เบอร\u0e4cโทร :";
		this.PanelEx4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx4.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx4.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx4.Controls.Add(this.ButtonX_2);
		this.PanelEx4.Controls.Add(this.ListView1);
		DevComponents.DotNetBar.PanelEx panelEx12 = this.PanelEx4;
		location = new System.Drawing.Point(295, 85);
		panelEx12.Location = location;
		this.PanelEx4.Name = "PanelEx4";
		DevComponents.DotNetBar.PanelEx panelEx13 = this.PanelEx4;
		size = new System.Drawing.Size(840, 315);
		panelEx13.Size = size;
		this.PanelEx4.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx4.Style.BackColor1.Color = System.Drawing.Color.FromArgb(227, 239, 255);
		this.PanelEx4.Style.BackColor2.Color = System.Drawing.Color.FromArgb(175, 210, 255);
		this.PanelEx4.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx4.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx4.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx4.Style.GradientAngle = 90;
		this.PanelEx4.TabIndex = 14;
		this.PanelEx4.Visible = false;
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_2.Image = iHOTEL2025.My.Resources.Resources.delete1;
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_2;
		location = new System.Drawing.Point(9, 287);
		buttonX_.Location = location;
		this.ButtonX_2.Name = "ButtonX12";
		DevComponents.DotNetBar.ButtonX buttonX_2 = this.ButtonX_2;
		size = new System.Drawing.Size(75, 23);
		buttonX_2.Size = size;
		this.ButtonX_2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_2.TabIndex = 1;
		this.ButtonX_2.Text = "ป\u0e34ด";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[6] { this.ColumnHeader1, this.ColumnHeader3, this.ColumnHeader2, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6 });
		this.ListView1.FullRowSelect = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(9, 10);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(821, 273);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ห\u0e49อง";
		this.ColumnHeader1.Width = 90;
		this.ColumnHeader3.Text = "สถานะ";
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ/บร\u0e34ษ\u0e31ท";
		this.ColumnHeader2.Width = 400;
		this.ColumnHeader4.Width = 0;
		this.ColumnHeader5.Text = "เบอร\u0e4cโทร";
		this.ColumnHeader5.Width = 100;
		this.ColumnHeader6.Text = "ทะเบ\u0e35ยนรถ";
		this.ColumnHeader6.Width = 80;
		this.TimerSearch.Interval = 300;
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_1.BackColor = System.Drawing.Color.FromArgb(255, 255, 128);
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_1.FocusCuesEnabled = false;
		this.ButtonX_1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_1.Image = (System.Drawing.Image)resources.GetObject("ButtonX11.Image");
		DevComponents.DotNetBar.ButtonX buttonX_3 = this.ButtonX_1;
		location = new System.Drawing.Point(961, 622);
		buttonX_3.Location = location;
		this.ButtonX_1.Name = "ButtonX11";
		this.ButtonX_1.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX_4 = this.ButtonX_1;
		size = new System.Drawing.Size(35, 30);
		buttonX_4.Size = size;
		this.ButtonX_1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_1.TabIndex = 10;
		this.ButtonX_1.Tooltip = "ด\u0e39ผ\u0e31งการจอง";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_0.BackColor = System.Drawing.Color.FromArgb(255, 255, 128);
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ButtonX10.Image");
		DevComponents.DotNetBar.ButtonX buttonX_5 = this.ButtonX_0;
		location = new System.Drawing.Point(927, 622);
		buttonX_5.Location = location;
		this.ButtonX_0.Name = "ButtonX10";
		this.ButtonX_0.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX_6 = this.ButtonX_0;
		size = new System.Drawing.Size(35, 30);
		buttonX_6.Size = size;
		this.ButtonX_0.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_0.TabIndex = 9;
		this.ButtonX_0.Tooltip = "จองแบบไม\u0e48ระบ\u0e38ห\u0e49อง";
		this.ButtonX9.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX9.BackColor = System.Drawing.Color.FromArgb(255, 255, 128);
		this.ButtonX9.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX9.FocusCuesEnabled = false;
		this.ButtonX9.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX9.Image = (System.Drawing.Image)resources.GetObject("ButtonX9.Image");
		DevComponents.DotNetBar.ButtonX buttonX21 = this.ButtonX9;
		location = new System.Drawing.Point(893, 622);
		buttonX21.Location = location;
		this.ButtonX9.Name = "ButtonX9";
		this.ButtonX9.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX22 = this.ButtonX9;
		size = new System.Drawing.Size(35, 30);
		buttonX22.Size = size;
		this.ButtonX9.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX9.TabIndex = 4;
		this.ButtonX9.Tooltip = "จองแบบระบ\u0e38ห\u0e49อง";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1137, 654);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx4);
		this.Controls.Add(this.ButtonX8);
		this.Controls.Add(this.ButtonX_1);
		this.Controls.Add(this.ButtonX_0);
		this.Controls.Add(this.ButtonX9);
		this.Controls.Add(this.FlowLayoutPanel1);
		this.Controls.Add(this.GroupPanel2);
		this.Controls.Add(this.FlowLayoutPanel4);
		this.Controls.Add(this.PanelEx3);
		this.Controls.Add(this.PanelEx_0);
		this.Controls.Add(this.GroupPanel1);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.LabelX3);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormRoomMain";
		this.Text = "สถานะห\u0e49องพ\u0e31ก";
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
		this.PanelEx4.ResumeLayout(false);
		this.ResumeLayout(false);
		this.PerformLayout();
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
			try
			{
				ComboBoxEx1.SelectedIndex = 0;
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				MessageBox.Show("กร\u0e38ณาเพ\u0e34\u0e48มกล\u0e38\u0e48มห\u0e49องพ\u0e31กก\u0e48อน ในหน\u0e49า จ\u0e31ดการห\u0e49องพ\u0e31ก", "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				ProjectData.ClearProjectError();
			}
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
		Timer1.Enabled = false;
		Timer2.Enabled = false;
		Timer3.Enabled = false;
		Timer4.Enabled = false;
		if (Module1.ISfullscreen)
		{
			FormRoomMain formRoomMain = new FormRoomMain();
			formRoomMain.Show();
		}
	}

	private void FormRoomMain_Load(object sender, EventArgs e)
	{
		lasttime_update_pwer = DateTime.Now.AddHours(-1.0);
		Module1.ISfullscreen = false;
		if (showFull)
		{
			ButtonX7.Visible = true;
		}
		else
		{
			ButtonX7.Visible = false;
		}
		ArrTip.Clear();
		Timer1.Enabled = true;
		Timer2.Enabled = true;
		Timer3.Enabled = true;
		Timer4.Enabled = true;
	}

	public void method_0(string R_NO, string R_TYPE, string R_STATUS, string R_DS, int x, int y, string R_polity, DataSet C_status, DataSet C_SMS, string NAMESTATUStext, string power_on, double big, double big2, int icon_no, string cin_no)
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
		Panel panel5 = new Panel();
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
		panel3.BackgroundImage = Resources.coins;
		panel3.BackgroundImageLayout = ImageLayout.None;
		location = new Point(0, 0);
		panel3.Location = location;
		panel3.Name = R_NO;
		size = new Size(0, 20);
		panel3.Size = size;
		panel3.TabIndex = 0;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(2, 0, 0, 0);
		panel3.Margin = margin;
		panel3.Cursor = Cursors.Hand;
		panel3.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
		{
			ROOM_DEBT_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
		};
		flowLayoutPanel.Controls.Add(panel3);
		decimal d = default(decimal);
		string name = "";
		bool flag = false;
		checked
		{
			int num = C_status.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (!Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(C_status.Tables[0].Rows[num2]["cin_room_no"], R_NO, TextCompare: false), Operators.CompareObjectEqual(C_status.Tables[0].Rows[num2]["Cin_no"], cin_no, TextCompare: false))))
				{
					num2++;
					continue;
				}
				d = Conversions.ToDecimal(C_status.Tables[0].Rows[num2]["total_price_vat"]);
				name = Conversions.ToString(C_status.Tables[0].Rows[num2]["cin_no"]);
				if (Operators.ConditionalCompareObjectGreater(C_status.Tables[0].Rows[num2]["Total_Price_Balance"], 0, TextCompare: false))
				{
					SuperTooltip1.SetSuperTooltip(panel3, new SuperTooltipInfo("ยอดค\u0e49างชำระ", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num2]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num2]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดค\u0e49างชำระท\u0e31\u0e49งส\u0e34\u0e49น "), Strings.Format(RuntimeHelpers.GetObjectValue(C_status.Tables[0].Rows[num2]["Total_Price_Balance"]), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Yellow));
					size = new Size(18, 20);
					panel3.Size = size;
				}
				else if (Operators.ConditionalCompareObjectLess(C_status.Tables[0].Rows[num2]["Total_Price_Balance"], 0, TextCompare: false))
				{
					panel3.BackgroundImage = Resources.coins_delete;
					SuperTooltip superTooltip = SuperTooltip1;
					object left = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num2]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num2]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดเง\u0e34นเก\u0e34นท\u0e31\u0e49งส\u0e34\u0e49น ");
					Type typeFromHandle = typeof(Math);
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = C_status.Tables[0].Rows[num2];
					DataRow dataRow2 = dataRow;
					string columnName = "Total_Price_Balance";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					object obj = NewLateBinding.LateGet(null, typeFromHandle, "Abs", arguments, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					superTooltip.SetSuperTooltip(panel3, new SuperTooltipInfo("ยอดเง\u0e34นเก\u0e34น", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(left, Strings.Format(RuntimeHelpers.GetObjectValue(obj), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Purple));
					size = new Size(18, 20);
					panel3.Size = size;
				}
				flag = true;
				break;
			}
			if (!flag)
			{
				int num5 = C_status.Tables[0].Rows.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					if (!Operators.ConditionalCompareObjectEqual(C_status.Tables[0].Rows[num6]["cin_room_no"], R_NO, TextCompare: false))
					{
						num6++;
						continue;
					}
					d = Conversions.ToDecimal(C_status.Tables[0].Rows[num6]["total_price_vat"]);
					name = Conversions.ToString(C_status.Tables[0].Rows[num6]["cin_no"]);
					if (Operators.ConditionalCompareObjectGreater(C_status.Tables[0].Rows[num6]["Total_Price_Balance"], 0, TextCompare: false))
					{
						SuperTooltip1.SetSuperTooltip(panel3, new SuperTooltipInfo("ยอดค\u0e49างชำระ", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num6]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num6]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดค\u0e49างชำระท\u0e31\u0e49งส\u0e34\u0e49น "), Strings.Format(RuntimeHelpers.GetObjectValue(C_status.Tables[0].Rows[num6]["Total_Price_Balance"]), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Yellow));
						size = new Size(18, 20);
						panel3.Size = size;
					}
					else if (Operators.ConditionalCompareObjectLess(C_status.Tables[0].Rows[num6]["Total_Price_Balance"], 0, TextCompare: false))
					{
						panel3.BackgroundImage = Resources.coins_delete;
						SuperTooltip superTooltip2 = SuperTooltip1;
						object left2 = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num6]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num6]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดเง\u0e34นเก\u0e34นท\u0e31\u0e49งส\u0e34\u0e49น ");
						Type typeFromHandle2 = typeof(Math);
						object[] array3 = new object[1];
						object[] array5 = array3;
						DataRow dataRow = C_status.Tables[0].Rows[num6];
						DataRow dataRow3 = dataRow;
						string columnName = "Total_Price_Balance";
						array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
						object[] array = array3;
						object[] arguments2 = array;
						bool[] array4 = new bool[1] { true };
						object obj2 = NewLateBinding.LateGet(null, typeFromHandle2, "Abs", arguments2, null, null, array4);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						superTooltip2.SetSuperTooltip(panel3, new SuperTooltipInfo("ยอดเง\u0e34นเก\u0e34น", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(left2, Strings.Format(RuntimeHelpers.GetObjectValue(obj2), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Purple));
						size = new Size(18, 20);
						panel3.Size = size;
					}
					flag = true;
					break;
				}
			}
			panel5.BackgroundImage = Resources.email;
			panel5.BackgroundImageLayout = ImageLayout.None;
			location = new Point(0, 0);
			panel5.Location = location;
			margin = new System.Windows.Forms.Padding(0, 1, 0, 0);
			panel5.Margin = margin;
			panel5.Name = "0";
			size = new Size(0, 20);
			panel5.Size = size;
			panel5.TabIndex = 0;
			panel5.Cursor = Cursors.Hand;
			panel5.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
			{
				ROOM_NOTE_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
			};
			flowLayoutPanel.Controls.Add(panel5);
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
					panel5.Name = Conversions.ToString(C_SMS.Tables[0].Rows[num9]["SMS_ID"]);
					size = new Size(16, 20);
					panel5.Size = size;
					break;
				}
				break;
			}
			panel4.BackgroundImageLayout = ImageLayout.None;
			location = new Point(0, 0);
			panel4.Location = location;
			panel4.Name = R_NO;
			if (Operators.CompareString(power_on.ToUpper(), "ON", TextCompare: false) == 0)
			{
				size = new Size(16, 20);
				panel4.Size = size;
				panel4.BackgroundImage = Resources.lightbulb;
			}
			else
			{
				size = new Size(0, 20);
				panel4.Size = size;
				panel4.BackgroundImage = null;
			}
			panel4.TabIndex = 0;
			margin = new System.Windows.Forms.Padding(0, 0, 0, 0);
			panel4.Margin = margin;
			SuperTooltip1.SetSuperTooltip(panel4, new SuperTooltipInfo("สถานะการเป\u0e34ดไฟ", "iHOTEL", "ไฟเป\u0e34ดอย\u0e39\u0e48", Resources.lightbulb, null, eTooltipColor.Cyan));
			flowLayoutPanel.Controls.Add(panel4);
			panel2.BackgroundImageLayout = ImageLayout.None;
			location = new Point(0, 0);
			panel2.Location = location;
			margin = new System.Windows.Forms.Padding(0, 0, 0, 0);
			panel2.Margin = margin;
			panel2.Name = name;
			panel2.Cursor = Cursors.Hand;
			if (decimal.Compare(d, 0m) > 0)
			{
				size = new Size(32, 20);
				panel2.Size = size;
				panel2.BackgroundImage = Resources.vat7;
			}
			else
			{
				size = new Size(0, 20);
				panel2.Size = size;
				panel2.BackgroundImage = null;
			}
			panel2.TabIndex = 0;
			margin = new System.Windows.Forms.Padding(0, 0, 0, 0);
			panel2.Margin = margin;
			panel2.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
			{
				ROOM_BILL_MouseClick(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
			};
			SuperTooltip1.SetSuperTooltip(panel2, new SuperTooltipInfo("สถานะการออกใบกำก\u0e31บภาษ\u0e35", "iHOTEL", "ออกใบกำก\u0e31บภาษ\u0e35แล\u0e49ว", Resources.page_copy, null, eTooltipColor.Purple));
			flowLayoutPanel.Controls.Add(panel2);
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
			location = new Point(0, 29);
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
			panelEx3.Style.BackgroundImage = null;
			if (R_STATUS.ToString().IndexOf("จอง") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.LightYellow;
				panelEx3.Style.BackColor1.Color = Color.Yellow;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.Snow;
				panelEx3.Style.BackColor1.Color = Color.LightYellow;
			}
			else if (R_STATUS.ToString().IndexOf("  ") != -1)
			{
				panelEx3.Style.BackColor1.Color = Color.MistyRose;
				panelEx3.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.WhiteSmoke;
				panelEx3.Style.BackColor1.Color = Color.DarkGray;
			}
			else if (R_STATUS.ToString().IndexOf("รายเด\u0e37อน") != -1)
			{
				panelEx3.Style.BackColor2.Color = Color.Linen;
				panelEx3.Style.BackColor1.Color = Color.DarkOrange;
			}
			else if (R_STATUS.ToString().IndexOf("รายช\u0e31\u0e48วโมง") != -1)
			{
				panelEx3.Style.BackColor2.Color = Color.White;
				panelEx3.Style.BackColor1.Color = Color.SteelBlue;
			}
			else if (R_STATUS.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx3.Style.BackColor1.Color = Color.MistyRose;
				panelEx3.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.AliceBlue;
				panelEx3.Style.BackColor1.Color = Color.DeepSkyBlue;
			}
			else if (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.FloralWhite;
				panelEx3.Style.BackColor1.Color = Color.Moccasin;
			}
			else if (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.FloralWhite;
				panelEx3.Style.BackColor1.Color = Color.White;
			}
			else if (R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0)
			{
				panelEx3.Style.BackColor2.Color = Color.Honeydew;
				panelEx3.Style.BackColor1.Color = Color.LightGreen;
			}
			if (icon_no > -1)
			{
				panelEx3.Style.BackgroundImagePosition = eBackgroundImagePosition.TopRight;
				switch (icon_no)
				{
				case 0:
					panelEx3.Style.BackgroundImage = Resources.an4_36;
					break;
				case 1:
					panelEx3.Style.BackgroundImage = Resources.an4_2;
					break;
				case 2:
					panelEx3.Style.BackgroundImage = Resources.an4_3;
					break;
				case 3:
					panelEx3.Style.BackgroundImage = Resources.an4_39;
					break;
				case 4:
					panelEx3.Style.BackgroundImage = Resources.an4_5;
					break;
				case 5:
					panelEx3.Style.BackgroundImage = Resources.an4_6;
					break;
				case 6:
					panelEx3.Style.BackgroundImage = Resources.an4_7;
					break;
				case 7:
					panelEx3.Style.BackgroundImage = Resources.an4_8;
					break;
				case 8:
					panelEx3.Style.BackgroundImage = Resources.an4_34;
					break;
				case 9:
					panelEx3.Style.BackgroundImage = Resources.an4_10;
					break;
				case 10:
					panelEx3.Style.BackgroundImage = Resources.an4_11;
					break;
				case 11:
					panelEx3.Style.BackgroundImage = Resources.an4_12;
					break;
				case 12:
					panelEx3.Style.BackgroundImage = Resources.an4_33;
					break;
				case 13:
					panelEx3.Style.BackgroundImage = Resources.an4_14;
					break;
				case 14:
					panelEx3.Style.BackgroundImage = Resources.an4_15;
					break;
				case 15:
					panelEx3.Style.BackgroundImage = Resources.an4_16;
					break;
				case 16:
					panelEx3.Style.BackgroundImage = Resources.an4_17;
					break;
				case 17:
					panelEx3.Style.BackgroundImage = Resources.an4_18;
					break;
				case 18:
					panelEx3.Style.BackgroundImage = Resources.an4_19;
					break;
				case 19:
					panelEx3.Style.BackgroundImage = Resources.an4_20;
					break;
				case 20:
					panelEx3.Style.BackgroundImage = Resources.an4_21;
					break;
				case 21:
					panelEx3.Style.BackgroundImage = Resources.an4_22;
					break;
				case 22:
					panelEx3.Style.BackgroundImage = Resources.an4_23;
					break;
				case 23:
					panelEx3.Style.BackgroundImage = Resources.an4_24;
					break;
				case 24:
					panelEx3.Style.BackgroundImage = Resources.an4_25;
					break;
				case 25:
					panelEx3.Style.BackgroundImage = Resources.an4_26;
					break;
				case 26:
					panelEx3.Style.BackgroundImage = Resources.an4_27;
					break;
				case 27:
					panelEx3.Style.BackgroundImage = Resources.an4_28;
					break;
				case 28:
					panelEx3.Style.BackgroundImage = Resources.an4_29;
					break;
				case 29:
					panelEx3.Style.BackgroundImage = Resources.an4_30;
					break;
				case 30:
					panelEx3.Style.BackgroundImage = Resources.an4_31;
					break;
				case 31:
					panelEx3.Style.BackgroundImage = Resources.an4_32;
					break;
				case 32:
					panelEx3.Style.BackgroundImage = Resources.an4_13;
					break;
				case 33:
					panelEx3.Style.BackgroundImage = Resources.an4_9;
					break;
				case 34:
					panelEx3.Style.BackgroundImage = Resources.an4_35;
					break;
				case 35:
					panelEx3.Style.BackgroundImage = Resources.an4_1;
					break;
				case 36:
					panelEx3.Style.BackgroundImage = Resources.an4_37;
					break;
				case 37:
					panelEx3.Style.BackgroundImage = Resources.an4_38;
					break;
				case 38:
					panelEx3.Style.BackgroundImage = Resources.an4_4;
					break;
				case 39:
					panelEx3.Style.BackgroundImage = Resources.an4_40;
					break;
				}
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
			if (!ButtonX4.Checked)
			{
				labelX.DragEnter += drag_en;
				labelX.DragDrop += drag_drop;
			}
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
			size2 = new Size(panel.Width - 15, 20);
			flowLayoutPanel.Size = size2;
			flowLayoutPanel.TabIndex = 2;
			flowLayoutPanel.Anchor = AnchorStyles.Bottom | AnchorStyles.Left;
			C_status.Dispose();
			C_SMS.Dispose();
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

	public void SETButton_Notclear(string R_NO, string R_TYPE, string R_STATUS, string R_DS, int x, int y, string R_polity, DataSet C_status, DataSet C_SMS, string NAMESTATUStext, string power_on, int icon_no, string cin_no)
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
			Panel panel3 = (Panel)flowLayoutPanel.Controls[2];
			Panel panel4 = (Panel)flowLayoutPanel.Controls[3];
			labelX.Text = R_STATUS;
			panelEx2.Text = R_polity;
			SuperTooltip1.SetSuperTooltip(labelX, new SuperTooltipInfo("ห\u0e49องพ\u0e31ก", "iHOTEL", NAMESTATUStext, Resources.boy_emoticon_009, null, eTooltipColor.Lemon));
			labelX.MouseDown -= nodebtn_MouseDown;
			labelX.MouseDown -= nodebtn_MouseDown_Drag;
			labelX.MouseClick -= nodebtn_MouseClick;
			labelX.MouseMove -= nodebtn_MouseMove;
			labelX.MouseUp -= nodebtn_MouseUp;
			labelX.DragEnter -= drag_en;
			labelX.DragDrop -= drag_drop;
			if (ButtonX4.Checked)
			{
				labelX.Cursor = Cursors.NoMove2D;
			}
			else
			{
				labelX.Cursor = Cursors.Hand;
			}
			if ((R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0) | (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0) | (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0) | (R_STATUS.ToString().IndexOf("จอง") == 0) | ButtonX4.Checked)
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
			if (!ButtonX4.Checked)
			{
				labelX.DragEnter += drag_en;
				labelX.DragDrop += drag_drop;
			}
			panelEx.Style.BackgroundImage = null;
			panelEx.Style.GradientAngle = 90;
			if (R_STATUS.ToString().IndexOf("จอง") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.LightYellow;
				panelEx.Style.BackColor1.Color = Color.Yellow;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.Snow;
				panelEx.Style.BackColor1.Color = Color.LightYellow;
			}
			else if (R_STATUS.ToString().IndexOf("  ") != -1)
			{
				panelEx.Style.BackColor1.Color = Color.MistyRose;
				panelEx.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ซ\u0e48อม") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.WhiteSmoke;
				panelEx.Style.BackColor1.Color = Color.DarkGray;
			}
			else if (R_STATUS.ToString().IndexOf("รายเด\u0e37อน") != -1)
			{
				panelEx.Style.BackColor2.Color = Color.Linen;
				panelEx.Style.BackColor1.Color = Color.DarkOrange;
			}
			else if (R_STATUS.ToString().IndexOf("รายช\u0e31\u0e48วโมง") != -1)
			{
				panelEx.Style.BackColor2.Color = Color.White;
				panelEx.Style.BackColor1.Color = Color.SteelBlue;
			}
			else if (R_STATUS.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				panelEx.Style.BackColor1.Color = Color.MistyRose;
				panelEx.Style.BackColor2.Color = Color.OrangeRed;
			}
			else if (R_STATUS.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.AliceBlue;
				panelEx.Style.BackColor1.Color = Color.DeepSkyBlue;
			}
			else if (R_STATUS.ToString().IndexOf("รอ ทำความสะอาด") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.FloralWhite;
				panelEx.Style.BackColor1.Color = Color.Moccasin;
			}
			else if (R_STATUS.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.FloralWhite;
				panelEx.Style.BackColor1.Color = Color.White;
			}
			else if (R_STATUS.ToString().IndexOf("ว\u0e48าง") == 0)
			{
				panelEx.Style.BackColor2.Color = Color.Honeydew;
				panelEx.Style.BackColor1.Color = Color.LightGreen;
			}
			if (icon_no > -1)
			{
				panelEx.Style.BackgroundImagePosition = eBackgroundImagePosition.TopRight;
				switch (icon_no)
				{
				case 0:
					panelEx.Style.BackgroundImage = Resources.an4_36;
					break;
				case 1:
					panelEx.Style.BackgroundImage = Resources.an4_2;
					break;
				case 2:
					panelEx.Style.BackgroundImage = Resources.an4_3;
					break;
				case 3:
					panelEx.Style.BackgroundImage = Resources.an4_39;
					break;
				case 4:
					panelEx.Style.BackgroundImage = Resources.an4_5;
					break;
				case 5:
					panelEx.Style.BackgroundImage = Resources.an4_6;
					break;
				case 6:
					panelEx.Style.BackgroundImage = Resources.an4_7;
					break;
				case 7:
					panelEx.Style.BackgroundImage = Resources.an4_8;
					break;
				case 8:
					panelEx.Style.BackgroundImage = Resources.an4_34;
					break;
				case 9:
					panelEx.Style.BackgroundImage = Resources.an4_10;
					break;
				case 10:
					panelEx.Style.BackgroundImage = Resources.an4_11;
					break;
				case 11:
					panelEx.Style.BackgroundImage = Resources.an4_12;
					break;
				case 12:
					panelEx.Style.BackgroundImage = Resources.an4_33;
					break;
				case 13:
					panelEx.Style.BackgroundImage = Resources.an4_14;
					break;
				case 14:
					panelEx.Style.BackgroundImage = Resources.an4_15;
					break;
				case 15:
					panelEx.Style.BackgroundImage = Resources.an4_16;
					break;
				case 16:
					panelEx.Style.BackgroundImage = Resources.an4_17;
					break;
				case 17:
					panelEx.Style.BackgroundImage = Resources.an4_18;
					break;
				case 18:
					panelEx.Style.BackgroundImage = Resources.an4_19;
					break;
				case 19:
					panelEx.Style.BackgroundImage = Resources.an4_20;
					break;
				case 20:
					panelEx.Style.BackgroundImage = Resources.an4_21;
					break;
				case 21:
					panelEx.Style.BackgroundImage = Resources.an4_22;
					break;
				case 22:
					panelEx.Style.BackgroundImage = Resources.an4_23;
					break;
				case 23:
					panelEx.Style.BackgroundImage = Resources.an4_24;
					break;
				case 24:
					panelEx.Style.BackgroundImage = Resources.an4_25;
					break;
				case 25:
					panelEx.Style.BackgroundImage = Resources.an4_26;
					break;
				case 26:
					panelEx.Style.BackgroundImage = Resources.an4_27;
					break;
				case 27:
					panelEx.Style.BackgroundImage = Resources.an4_28;
					break;
				case 28:
					panelEx.Style.BackgroundImage = Resources.an4_29;
					break;
				case 29:
					panelEx.Style.BackgroundImage = Resources.an4_30;
					break;
				case 30:
					panelEx.Style.BackgroundImage = Resources.an4_31;
					break;
				case 31:
					panelEx.Style.BackgroundImage = Resources.an4_32;
					break;
				case 32:
					panelEx.Style.BackgroundImage = Resources.an4_13;
					break;
				case 33:
					panelEx.Style.BackgroundImage = Resources.an4_9;
					break;
				case 34:
					panelEx.Style.BackgroundImage = Resources.an4_35;
					break;
				case 35:
					panelEx.Style.BackgroundImage = Resources.an4_1;
					break;
				case 36:
					panelEx.Style.BackgroundImage = Resources.an4_37;
					break;
				case 37:
					panelEx.Style.BackgroundImage = Resources.an4_38;
					break;
				case 38:
					panelEx.Style.BackgroundImage = Resources.an4_4;
					break;
				case 39:
					panelEx.Style.BackgroundImage = Resources.an4_40;
					break;
				}
			}
			decimal d = default(decimal);
			string name = "";
			bool flag = false;
			bool flag2 = false;
			int num5 = C_status.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				if (!Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(C_status.Tables[0].Rows[num6]["cin_room_no"], R_NO, TextCompare: false), Operators.CompareObjectEqual(C_status.Tables[0].Rows[num6]["Cin_no"], cin_no, TextCompare: false))))
				{
					num6++;
					continue;
				}
				d = Conversions.ToDecimal(C_status.Tables[0].Rows[num6]["total_price_vat"]);
				name = Conversions.ToString(C_status.Tables[0].Rows[num6]["cin_no"]);
				if (Operators.ConditionalCompareObjectGreater(C_status.Tables[0].Rows[num6]["Total_Price_Balance"], 0, TextCompare: false))
				{
					if (panel.Width == 0)
					{
						Size size = new Size(18, 20);
						panel.Size = size;
					}
					panel.BackgroundImage = Resources.coins;
					SuperTooltip1.SetSuperTooltip(panel, new SuperTooltipInfo("ยอดค\u0e49างชำระ", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num6]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num6]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดค\u0e49างชำระท\u0e31\u0e49งส\u0e34\u0e49น "), Strings.Format(RuntimeHelpers.GetObjectValue(C_status.Tables[0].Rows[num6]["Total_Price_Balance"]), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Yellow));
					flag = true;
				}
				else if (Operators.ConditionalCompareObjectLess(C_status.Tables[0].Rows[num6]["Total_Price_Balance"], 0, TextCompare: false))
				{
					if (panel.Width == 0)
					{
						Size size = new Size(18, 20);
						panel.Size = size;
					}
					panel.BackgroundImage = Resources.coins_delete;
					SuperTooltip superTooltip = SuperTooltip1;
					object left = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num6]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num6]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดเง\u0e34นเก\u0e34นท\u0e31\u0e49งส\u0e34\u0e49น ");
					Type typeFromHandle = typeof(Math);
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = C_status.Tables[0].Rows[num6];
					DataRow dataRow2 = dataRow;
					string columnName = "Total_Price_Balance";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					object obj = NewLateBinding.LateGet(null, typeFromHandle, "Abs", arguments, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					superTooltip.SetSuperTooltip(panel, new SuperTooltipInfo("ยอดเง\u0e34นเก\u0e34น", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(left, Strings.Format(RuntimeHelpers.GetObjectValue(obj), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Purple));
					flag = true;
				}
				break;
			}
			if (!flag)
			{
				int num8 = C_status.Tables[0].Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					int num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					if (!Operators.ConditionalCompareObjectEqual(C_status.Tables[0].Rows[num9]["cin_room_no"], R_NO, TextCompare: false))
					{
						num9++;
						continue;
					}
					d = Conversions.ToDecimal(C_status.Tables[0].Rows[num9]["total_price_vat"]);
					name = Conversions.ToString(C_status.Tables[0].Rows[num9]["cin_no"]);
					if (Operators.ConditionalCompareObjectGreater(C_status.Tables[0].Rows[num9]["Total_Price_Balance"], 0, TextCompare: false))
					{
						if (panel.Width == 0)
						{
							Size size = new Size(18, 20);
							panel.Size = size;
						}
						panel.BackgroundImage = Resources.coins;
						SuperTooltip1.SetSuperTooltip(panel, new SuperTooltipInfo("ยอดค\u0e49างชำระ", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num9]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num9]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดค\u0e49างชำระท\u0e31\u0e49งส\u0e34\u0e49น "), Strings.Format(RuntimeHelpers.GetObjectValue(C_status.Tables[0].Rows[num9]["Total_Price_Balance"]), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Yellow));
						flag = true;
					}
					else if (Operators.ConditionalCompareObjectLess(C_status.Tables[0].Rows[num9]["Total_Price_Balance"], 0, TextCompare: false))
					{
						if (panel.Width == 0)
						{
							Size size = new Size(18, 20);
							panel.Size = size;
						}
						panel.BackgroundImage = Resources.coins_delete;
						SuperTooltip superTooltip2 = SuperTooltip1;
						object left2 = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", C_status.Tables[0].Rows[num9]["cin_room_all"]), " ("), C_status.Tables[0].Rows[num9]["cin_no"]), ")"), "\r\n"), "ม\u0e35ยอดเง\u0e34นเก\u0e34นท\u0e31\u0e49งส\u0e34\u0e49น ");
						Type typeFromHandle2 = typeof(Math);
						object[] array3 = new object[1];
						object[] array5 = array3;
						DataRow dataRow = C_status.Tables[0].Rows[num9];
						DataRow dataRow3 = dataRow;
						string columnName = "Total_Price_Balance";
						array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
						object[] array = array3;
						object[] arguments2 = array;
						bool[] array4 = new bool[1] { true };
						object obj2 = NewLateBinding.LateGet(null, typeFromHandle2, "Abs", arguments2, null, null, array4);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						superTooltip2.SetSuperTooltip(panel, new SuperTooltipInfo("ยอดเง\u0e34นเก\u0e34น", "iHOTEL", Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(left2, Strings.Format(RuntimeHelpers.GetObjectValue(obj2), "#,##0.00")), " บาท")), Resources.money, null, eTooltipColor.Purple));
						flag = true;
					}
					break;
				}
			}
			if (!flag)
			{
				Size size = new Size(0, 20);
				panel.Size = size;
			}
			int num11 = C_SMS.Tables[0].Rows.Count - 1;
			int num12 = 0;
			while (true)
			{
				int num13 = num12;
				int num4 = num11;
				if (num13 > num4)
				{
					break;
				}
				if (!Operators.ConditionalCompareObjectEqual(C_SMS.Tables[0].Rows[num12]["SMS_Room"], R_NO, TextCompare: false))
				{
					num12++;
					continue;
				}
				panel2.Name = Conversions.ToString(C_SMS.Tables[0].Rows[num12]["SMS_ID"]);
				if (panel2.Width == 0)
				{
					Size size = new Size(16, 20);
					panel2.Size = size;
				}
				flag2 = true;
				break;
			}
			if (!flag2)
			{
				Size size = new Size(0, 20);
				panel2.Size = size;
			}
			if (Operators.CompareString(power_on.ToUpper(), "ON", TextCompare: false) == 0)
			{
				if (panel3.Width == 0)
				{
					Size size = new Size(16, 20);
					panel3.Size = size;
					panel3.BackgroundImage = Resources.lightbulb;
				}
			}
			else
			{
				Size size = new Size(0, 20);
				panel3.Size = size;
				panel3.BackgroundImage = null;
			}
			if (decimal.Compare(d, 0m) > 0)
			{
				panel4.Name = name;
				if (panel4.Width == 0)
				{
					Size size = new Size(32, 20);
					panel4.Size = size;
					panel4.BackgroundImage = Resources.vat7;
				}
			}
			else
			{
				Size size = new Size(0, 20);
				panel4.Size = size;
				panel4.BackgroundImage = null;
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
			MyProject.Forms.ClickBook.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickBook.RoomArr = CHK_Array;
			MyProject.Forms.ClickBook.ShowDialog();
			if (MyProject.Forms.ClickBook.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("รอ ทำความสะอาด") == 0)
		{
			MyProject.Forms.ClickClean.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickClean.RoomArr = CHK_Array;
			MyProject.Forms.ClickClean.ShowDialog();
			if (MyProject.Forms.ClickClean.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0)
		{
			MyProject.Forms.ClickCleanOK.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickCleanOK.RoomArr = CHK_Array;
			MyProject.Forms.ClickCleanOK.ShowDialog();
			if (MyProject.Forms.ClickCleanOK.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ซ\u0e48อม") == 0)
		{
			MyProject.Forms.ClickManternance.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickManternance.RoomArr = CHK_Array;
			MyProject.Forms.ClickManternance.ShowDialog();
			if (MyProject.Forms.ClickManternance.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ว\u0e48าง") == 0)
		{
			MyProject.Forms.ClickAvliable.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickAvliable.RoomArr = CHK_Array;
			MyProject.Forms.ClickAvliable.ShowDialog();
			if (MyProject.Forms.ClickAvliable.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if ((NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0) | (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0))
		{
			if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
			{
				MyProject.Forms.ClickUSE.ButtonX_6.Visible = true;
			}
			if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				MyProject.Forms.ClickUSE.ButtonX7.Enabled = false;
				MyProject.Forms.ClickUSE.ButtonX_6.Visible = true;
			}
			else
			{
				MyProject.Forms.ClickUSE.ButtonX7.Enabled = true;
			}
			if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("รายช\u0e31\u0e48วโมง") != -1)
			{
				MyProject.Forms.ClickUSE.ButtonX_1.Enabled = false;
				MyProject.Forms.ClickUSE.ButtonX7.Enabled = false;
				MyProject.Forms.ClickUSE.ButtonX_6.Visible = true;
			}
			else
			{
				MyProject.Forms.ClickUSE.ButtonX_1.Enabled = true;
			}
			if (MyProject.Forms.ClickUSE.ButtonX_1.Enabled)
			{
				if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("รายเด\u0e37อน") != -1)
				{
					MyProject.Forms.ClickUSE.ButtonX_1.Enabled = false;
					MyProject.Forms.ClickUSE.ButtonX7.Enabled = false;
					MyProject.Forms.ClickUSE.ButtonX_6.Visible = false;
				}
				else
				{
					MyProject.Forms.ClickUSE.ButtonX_1.Enabled = true;
				}
			}
			MyProject.Forms.ClickUSE.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickUSE.RoomArr = CHK_Array;
			MyProject.Forms.ClickUSE.ShowDialog();
			if (MyProject.Forms.ClickUSE.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else if (NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null).ToString().IndexOf("ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก") == 0)
		{
			MyProject.Forms.ClickUSE2_0.RoomNo = Conversions.ToString(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			MyProject.Forms.ClickUSE2_0.RoomArr = CHK_Array;
			MyProject.Forms.ClickUSE2_0.ShowDialog();
			if (MyProject.Forms.ClickUSE2_0.ISOK)
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

	private void drag_drop(object sender, DragEventArgs e)
	{
		if (e.Data.GetData(DataFormats.Text).ToString().IndexOf("#ย\u0e49ายห\u0e49อง#") != -1)
		{
			object obj = e.Data.GetData(DataFormats.Text).ToString().Replace("#ย\u0e49ายห\u0e49อง#", "");
			object objectValue = RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			if (Operators.ConditionalCompareObjectEqual(obj, objectValue, TextCompare: false))
			{
				return;
			}
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where room_no='", objectValue), "'")));
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบเลขห\u0e49อง ", objectValue)), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			}
			else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["room_clean"], "yes", TextCompare: false))
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue), " ทำความสะอาดอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			}
			else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["Room_Manternace"], "yes", TextCompare: false))
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue), " ซ\u0e48อมบำร\u0e38งอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			}
			else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["room_use"], "yes", TextCompare: false))
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue), " ถ\u0e39กใช\u0e49งานอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			}
			else if (Operators.CompareString(dataSet.Tables[0].Rows[0]["room_book"].ToString(), "", TextCompare: false) != 0)
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue), " ถ\u0e39กจองอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			}
			else if (MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ค\u0e38ณต\u0e49องการย\u0e49ายห\u0e49องจาก ", obj), " เป\u0e47น "), objectValue), " หร\u0e37อไม\u0e48")), "ย\u0e49ายห\u0e49อง", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
			{
				object obj2 = Interaction.InputBox("กร\u0e38ณาใส\u0e48หมายเหต\u0e38", "กร\u0e38ณาใส\u0e48หมายเหต\u0e38");
				if (Operators.ConditionalCompareObjectEqual(obj2, "", TextCompare: false))
				{
					MessageBox.Show("ย\u0e31งไม\u0e48ได\u0e49ย\u0e49ายห\u0e49อง", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
					return;
				}
				bool flag = false;
				if (MessageBox.Show("ค\u0e38ณต\u0e49องการเปล\u0e35\u0e48ยนราคาห\u0e49องท\u0e35\u0e48ย\u0e49ายด\u0e49วยหร\u0e37อไม\u0e48\r\nถ\u0e49าเปล\u0e35\u0e48ยนราคากด Yes ถ\u0e49าไม\u0e48เปล\u0e35\u0e48ยนราคากด No", "ค\u0e38ณต\u0e49องการเปล\u0e35\u0e48ยนราคาห\u0e49องท\u0e35\u0e48ย\u0e49ายด\u0e49วยหร\u0e37อไม\u0e48", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
				{
					flag = true;
				}
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_H  where cin_no in (select cin_no from HT_CheckIn_Ds where (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก') and cin_room_no='", obj), "')")));
				if (dataSet2.Tables[0].Rows.Count == 0)
				{
					MOVEEEEE = true;
					MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบเลขห\u0e49อง ", obj)), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
					return;
				}
				Module1.Change_Room(Conversions.ToString(obj), Conversions.ToString(objectValue), Conversions.ToString(obj2), flag, Conversions.ToString(dataSet2.Tables[0].Rows[0]["cin_cust_price"]));
				if (!flag)
				{
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_H set Cin_room_all='", NewLateBinding.LateGet(dataSet2.Tables[0].Rows[0]["cin_room_all"].ToString(), null, "Replace", new object[2]
					{
						Operators.ConcatenateObject(obj, " "),
						Operators.ConcatenateObject(objectValue, " ")
					}, null, null, null)), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_room_no='", objectValue), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and cin_room_no='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Pay set cin_pay_ds='", objectValue), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and cin_pay_ds='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Product set Cin_room_no='", objectValue), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and Cin_room_no='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Room_Status set room_no='", objectValue), "' where room_CheckIn_No='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and room_no='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Rooms set Room_Use='yes' where room_no='", objectValue), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Rooms set Room_Use='no',Room_Clean='yes' where room_no='", obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Room_SMS set SMS_Room='", objectValue), "' where SMS_Room='"), obj), "' and SMS_Readed='no'")));
				}
				else
				{
					DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where room_no='", objectValue), "'")));
					DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_Ds  where cin_no='", dataSet2.Tables[0].Rows[0]["cin_no"]), "' and cin_room_no='"), obj), "'")));
					DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms_Price  where room_type='", dataSet3.Tables[0].Rows[0]["room_type"]), "' and room_custtype='"), dataSet2.Tables[0].Rows[0]["cin_cust_price"]), "' ")));
					decimal d = Conversions.ToDecimal(Operators.MultiplyObject(dataSet4.Tables[0].Rows[0]["cin_room_price"], dataSet4.Tables[0].Rows[0]["cin_room_night"]));
					decimal num = Conversions.ToDecimal(Operators.MultiplyObject(dataSet5.Tables[0].Rows[0]["room_price"], dataSet4.Tables[0].Rows[0]["cin_room_night"]));
					decimal num2 = default(decimal);
					num2 = decimal.Subtract(num, d);
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(string.Concat(string.Concat("update HT_CheckIn_H set total_price_room=total_price_room+" + Conversions.ToString(num2), ",total_price_net=total_price_net+"), Conversions.ToString(num2)), ",Cin_room_all='"), NewLateBinding.LateGet(dataSet2.Tables[0].Rows[0]["cin_room_all"].ToString(), null, "Replace", new object[2]
					{
						Operators.ConcatenateObject(obj, " "),
						Operators.ConcatenateObject(objectValue, " ")
					}, null, null, null)), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Room_Price=", dataSet5.Tables[0].Rows[0]["room_price"]), ",Cin_Room_PriceToTal="), num), ",Cin_room_no='"), objectValue), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and cin_room_no='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Pay set cin_pay_ds='", objectValue), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and cin_pay_ds='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Product set Cin_room_no='", objectValue), "' where cin_no='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and Cin_room_no='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Room_Status set room_no='", objectValue), "' where room_CheckIn_No='"), dataSet2.Tables[0].Rows[0]["cin_no"]), "' and room_no='"), obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Rooms set Room_Use='yes' where room_no='", objectValue), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Rooms set Room_Use='no',Room_Clean='yes' where room_no='", obj), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Room_SMS set SMS_Room='", objectValue), "' where SMS_Room='"), obj), "' and SMS_Readed='no'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_H set Total_Price_Balance=Total_Price_Net-Total_Price_Pay where cin_no='", dataSet2.Tables[0].Rows[0]["cin_no"]), "'")));
				}
				MessageBox.Show("ได\u0e49ทำการย\u0e49ายเสร\u0e47จเร\u0e35ยบร\u0e49อย");
				Module1.Power_set(Conversions.ToString(objectValue), "ON", "", Conversions.ToString(Operators.ConcatenateObject("เป\u0e34ดไฟ เน\u0e37\u0e48องจากย\u0e49ายห\u0e49องมาจาก ", obj)));
				Module1.Power_set(Conversions.ToString(obj), "OFF", "", Conversions.ToString(Operators.ConcatenateObject("ป\u0e34ดไฟ เน\u0e37\u0e48องจากย\u0e49ายไปห\u0e49อง ", objectValue)));
				PanelEx1.Text = "ย\u0e49ายข\u0e49ามกล\u0e38\u0e48ม/หน\u0e49าจอไม\u0e48พอ ให\u0e49ลากมาวางท\u0e35\u0e48น\u0e35\u0e48ก\u0e48อน";
				PanelEx1.Name = "PanelEx1";
				PanelEx1.Visible = false;
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
				MOVEEEEE = true;
			}
			else
			{
				MOVEEEEE = true;
			}
		}
		else
		{
			if (e.Data.GetData(DataFormats.Text).ToString().IndexOf("#จอง#") == -1)
			{
				return;
			}
			object objectValue2 = RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
			DataSet dataSet6 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where room_no='", objectValue2), "'")));
			if (dataSet6.Tables[0].Rows.Count == 0)
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบเลขห\u0e49อง ", objectValue2)), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (Operators.ConditionalCompareObjectEqual(dataSet6.Tables[0].Rows[0]["room_clean"], "yes", TextCompare: false))
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue2), " ทำความสะอาดอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (Operators.ConditionalCompareObjectEqual(dataSet6.Tables[0].Rows[0]["Room_Manternace"], "yes", TextCompare: false))
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue2), " ซ\u0e48อมบำร\u0e38งอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (Operators.ConditionalCompareObjectEqual(dataSet6.Tables[0].Rows[0]["room_use"], "yes", TextCompare: false))
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue2), " ถ\u0e39กใช\u0e49งานอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (Operators.CompareString(dataSet6.Tables[0].Rows[0]["room_book"].ToString(), "", TextCompare: false) != 0)
			{
				MOVEEEEE = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ห\u0e49อง ", objectValue2), " ถ\u0e39กจองอย\u0e39\u0e48")), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			DataSet dataSet7 = Module1.connect("select * from View_Book_Date where id=" + e.Data.GetData(DataFormats.Text).ToString().Replace("#จอง#", ""));
			if (Operators.ConditionalCompareObjectNotEqual(dataSet6.Tables[0].Rows[0]["Room_Type"], dataSet7.Tables[0].Rows[0]["Book_type"], TextCompare: false) && MessageBox.Show("ประเภทห\u0e49องไม\u0e48ตรงก\u0e31บท\u0e35\u0e48เล\u0e37อก ค\u0e38ณต\u0e49องการดำเน\u0e34นการต\u0e48อหร\u0e37อไม\u0e48", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.No)
			{
				MOVEEEEE = true;
				return;
			}
			string text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet7.Tables[0].Rows[0]["Book_Cust_Name"], dataSet7.Tables[0].Rows[0]["Book_Cust_Name2"]), "\r\n"), dataSet7.Tables[0].Rows[0]["Book_Cust_Tel"]), "\r\n"), "เวลาเข\u0e49าพ\u0e31ก : "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet7.Tables[0].Rows[0]["Book_Date_in"]), "HH:mm")), "\r\n"), "\r\n"), dataSet7.Tables[0].Rows[0]["Book_room_all"]), "\r\n"), "หมายเหต\u0e38 : "), dataSet7.Tables[0].Rows[0]["Book_room_note"]));
			Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("update HT_Rooms set room_book_ds='" + text, "', Room_Book='"), dataSet7.Tables[0].Rows[0]["id"]), "',Room_Book_Name='"), dataSet7.Tables[0].Rows[0]["Book_Cust_Name"]), " "), dataSet7.Tables[0].Rows[0]["Book_Cust_Name2"]), "',Room_Book_Time='"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet7.Tables[0].Rows[0]["Book_Date_in"]), "HH:mm")), "' where Room_no='"), objectValue2), "'")));
			Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Book_Date set Book_ok=Book_ok+1 where id=", dataSet7.Tables[0].Rows[0]["id"])));
		}
	}

	private void ROOM_BILL_MouseClick(object sender, MouseEventArgs e)
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_no='", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), "' order by cin_room_night")));
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Receipt_H where status_name<>'ยกเล\u0e34ก' and  Receipt_Ref='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' order by id desc")));
		if (dataSet2.Tables[0].Rows.Count == 0)
		{
			MyProject.Forms.FrmAddSale.IEdit = (string)(object)0;
			MyProject.Forms.FrmAddSale.clear();
			MyProject.Forms.FrmAddSale.Tref.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
			MyProject.Forms.FrmAddSale.B2_Click(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
			MyProject.Forms.FrmAddSale.Tnote.Text = "เข\u0e49าพ\u0e31ก ว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yy") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yy");
			MyProject.Forms.FrmAddSale.ShowDialog();
		}
		else
		{
			FormShowVAT formShowVAT = new FormShowVAT();
			formShowVAT.Label_NO.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
			formShowVAT.ShowDialog();
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

	public void check_timeout(DateTime dateout, string ROOM_NO)
	{
		if (Module1.bool_3 && DateTime.Compare(DateTime.Now, dateout) > 0)
		{
			Module1.Power_set(ROOM_NO, "OFF", "", "ป\u0e34ดไฟ อ\u0e31ตโนม\u0e31ต\u0e34 เน\u0e37\u0e48องจาก หมดเวลา [" + Strings.Format(dateout, "dd/MM/yy HH:mm") + "]");
		}
	}

	public void LoadRooms(int Px, int Py)
	{
		ArrayList arrayList = new ArrayList();
		Cursor = Cursors.WaitCursor;
		SELECT_ROOM_NOW = "";
		DateTime dateTime = DateTimePicker1.Value;
		if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), new decimal(Module1.CHK_IN_Before)) < 0))
		{
			dateTime = dateTime.AddDays(-1.0);
		}
		PanelEx_0.Text = "  สถานะห\u0e49องพ\u0e31กของค\u0e37นว\u0e31นท\u0e35\u0e48 " + Strings.Format(dateTime.Date, "dd/MM/yyyy");
		FlowLayoutPanel1.SuspendLayout();
		if (!IS_LIST_ROOM)
		{
			FlowLayoutPanel1.Hide();
			FlowLayoutPanel1.SuspendLayout();
			FlowLayoutPanel1.Controls.Clear();
		}
		DataSet dataSet = Module1.connect("select * From HT_Rooms  order by room_NO");
		object obj = "select * from View_Room_All where room_date=" + Module1.datechar + Conversions.ToString(dateTime.Date) + Module1.datechar + " and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก') order by id desc";
		DataSet dataSet2 = Module1.connect(Conversions.ToString(obj));
		DataSet dataSet3 = Module1.connect("select book_room_type,sum(book_room_num) as num from View_Book_Ds2 where book_status='จอง' and (Book_Date_in >= " + Module1.datechar + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00" + Module1.datechar + " or Book_date_out>=" + Module1.datechar + Conversions.ToString(DateTimePicker1.Value.Date) + " 23:59:59" + Module1.datechar + ")  group by book_room_type");
		DataSet dataSet4 = Module1.connect("select cin_no,cin_room_all,Cin_room_no,Total_Price_Balance,total_price_vat from View_CheckIn_Ds where (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		DataSet dataSet5 = Module1.connect("select SMS_Room,SMS_ID from HT_Room_SMS where SMS_Readed='no'");
		DataSet dataSet6 = Module1.connect("select * from View_Room_All where cin_room_status='เข\u0e49าพ\u0e31ก' order by cin_date_out desc");
		string text = "";
		UpdateSecond = 600;
		checked
		{
			if (DateAndTime.DateDiff(DateInterval.Second, lasttime_update_pwer, DateTime.Now) >= UpdateSecond)
			{
				MyProject.Forms.frmMain1.TimerOnoff.Enabled = false;
				lasttime_update_pwer = DateTime.Now;
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
					Module1.Power_set2(Conversions.ToString(dataSet.Tables[0].Rows[num2]["Room_no"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Room_Power_STATUS"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Room_Power_OPEN"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Room_Power_CLOSE"]));
					num2++;
				}
				if (MyProject.Forms.frmMain1.URL_ON_OFF.Items.Count != 0)
				{
					MyProject.Forms.frmMain1.TimerOnoff.Enabled = true;
				}
			}
			ArrayList arrayList2 = new ArrayList();
			ArrayList arrayList3 = new ArrayList();
			DataSet dataSet7 = Module1.connect("select * from HT_SET_RoomType");
			int num5 = dataSet7.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				string[] value = new string[6]
				{
					Conversions.ToString(dataSet7.Tables[0].Rows[num6]["name"]),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0),
					Conversions.ToString(0)
				};
				arrayList3.Add(value);
				num6++;
			}
			int num8 = dataSet3.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 > num4)
				{
					break;
				}
				string[] value2 = new string[2]
				{
					Conversions.ToString(dataSet3.Tables[0].Rows[num9]["book_room_type"]),
					Conversions.ToString(dataSet3.Tables[0].Rows[num9]["num"])
				};
				arrayList2.Add(value2);
				num9++;
			}
			if (DateTime.Compare(DateTimePicker1.Value.Date, DateTime.Now.Date) >= 0)
			{
				int num11 = dataSet.Tables[0].Rows.Count - 1;
				int num12 = 0;
				DateTime dateout = default(DateTime);
				while (true)
				{
					int num13 = num12;
					int num4 = num11;
					if (num13 > num4)
					{
						break;
					}
					text = "";
					int icon_no = -1;
					object obj2 = "\r\n";
					object obj3 = "ว\u0e48าง";
					string r_NO = Conversions.ToString(dataSet.Tables[0].Rows[num12]["room_NO"]);
					string cin_no = "";
					string r_TYPE = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" ", dataSet.Tables[0].Rows[num12]["Room_Type"]), " "), dataSet.Tables[0].Rows[num12]["Room_details"].ToString()));
					object objectValue = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num12]["Room_Polity"]);
					int num14 = Conversions.ToInteger(dataSet.Tables[0].Rows[num12]["Room_x"]);
					int num15 = Conversions.ToInteger(dataSet.Tables[0].Rows[num12]["Room_y"]);
					string value3 = dataSet.Tables[0].Rows[num12]["Room_PriceC"].ToString();
					string value4 = dataSet.Tables[0].Rows[num12]["Room_PriceB"].ToString();
					string text2 = "";
					string text3 = "";
					int num16 = dataSet2.Tables[0].Rows.Count - 1;
					int num17 = 0;
					while (true)
					{
						int num18 = num17;
						num4 = num16;
						if (num18 > num4)
						{
							break;
						}
						text3 = "";
						if (!Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num17]["room_no"], dataSet.Tables[0].Rows[num12]["room_NO"], TextCompare: false))
						{
							num17++;
							continue;
						}
						cin_no = Conversions.ToString(dataSet2.Tables[0].Rows[num17]["room_CheckIn_No"]);
						obj3 = RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num17]["room_status"]);
						obj2 = Operators.ConcatenateObject("\r\n", dataSet2.Tables[0].Rows[num17]["room_details"]);
						text2 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num17]["Cin_room_out"]), "dd-MM-yy HH:mm");
						dateout = Conversions.ToDate(dataSet2.Tables[0].Rows[num17]["Cin_room_out"]);
						Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num17]["Cin_room_out"]), "HH:mm");
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num17]["Cin_type"], 1, TextCompare: false))
						{
							Conversions.ToDate(dataSet2.Tables[0].Rows[num17]["Cin_room_in"]);
							DateTime d = Conversions.ToDate(dataSet2.Tables[0].Rows[num17]["Cin_room_out"]);
							text3 = "รายช\u0e31\u0e48วโมง\r\nเหล\u0e37อ " + Convert_Time(DateTime.Now, d, Conversions.ToString(dataSet2.Tables[0].Rows[num17]["room_no"]));
						}
						else if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num17]["Cin_type"], 2, TextCompare: false))
						{
							text3 = "## รายเด\u0e37อน ##";
						}
						if (Module1.CountCharacter(dataSet2.Tables[0].Rows[num17]["cin_room_all"].ToString(), ' ') >= 2)
						{
							text2 += "  ";
							if (Module1.SHOW_ICON)
							{
								bool flag = false;
								int num19 = arrayList.Count - 1;
								int num20 = 0;
								while (true)
								{
									int num21 = num20;
									num4 = num19;
									if (num21 <= num4)
									{
										if (!Operators.ConditionalCompareObjectEqual(arrayList[num20], dataSet2.Tables[0].Rows[num17]["cin_room_all"].ToString(), TextCompare: false))
										{
											num20++;
											continue;
										}
										flag = true;
										icon_no = num20;
										break;
									}
									break;
								}
								if (!flag)
								{
									arrayList.Add(dataSet2.Tables[0].Rows[num17]["cin_room_all"].ToString());
									icon_no = arrayList.Count - 1;
								}
							}
						}
						if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num17]["Cin_Room_Status"], "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก", TextCompare: false))
						{
							obj3 = "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก";
						}
						text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num17]["cust_name"], "\r\n"), dataSet2.Tables[0].Rows[num17]["cust_work_name"]), "\r\n"), "เลขท\u0e35\u0e48 "), dataSet2.Tables[0].Rows[num17]["room_checkin_no"]), "\r\n"), "ห\u0e49อง "), dataSet2.Tables[0].Rows[num17]["cin_room_all"]));
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(obj3, "Check Out", TextCompare: false))
					{
						obj3 = "ว\u0e48าง";
						text = "ว\u0e48าง";
					}
					if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(obj3, "ว\u0e48าง", TextCompare: false), Operators.CompareObjectEqual(obj3, "จอง", TextCompare: false))))
					{
						text = dataSet.Tables[0].Rows[num12]["room_book_ds"].ToString();
						if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num12]["room_use"], "yes", TextCompare: false))
						{
							string text4 = "";
							int num22 = dataSet6.Tables[0].Rows.Count - 1;
							int num23 = 0;
							while (true)
							{
								int num24 = num23;
								num4 = num22;
								if (num24 > num4)
								{
									break;
								}
								if (!Operators.ConditionalCompareObjectEqual(dataSet6.Tables[0].Rows[num23]["room_no"], dataSet.Tables[0].Rows[num12]["room_no"], TextCompare: false))
								{
									num23++;
									continue;
								}
								text4 = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet6.Tables[0].Rows[num23]["cin_room_out"]), "dd-MM-yy HH:mm");
								text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet6.Tables[0].Rows[num23]["cust_name"], "\r\n"), dataSet6.Tables[0].Rows[num23]["cust_work_name"]), "\r\n"), "เลขท\u0e35\u0e48 "), dataSet6.Tables[0].Rows[num23]["room_checkin_no"]), "\r\n"), "ห\u0e49อง "), dataSet6.Tables[0].Rows[num23]["cin_room_all"]));
								dateout = Conversions.ToDate(dataSet6.Tables[0].Rows[num23]["cin_room_out"]);
								if (!Module1.SHOW_ICON || Module1.CountCharacter(dataSet6.Tables[0].Rows[num23]["cin_room_all"].ToString(), ' ') < 2)
								{
									break;
								}
								bool flag2 = false;
								int num25 = arrayList.Count - 1;
								int num26 = 0;
								while (true)
								{
									int num27 = num26;
									num4 = num25;
									if (num27 <= num4)
									{
										if (!Operators.ConditionalCompareObjectEqual(arrayList[num26], dataSet6.Tables[0].Rows[num23]["cin_room_all"].ToString(), TextCompare: false))
										{
											num26++;
											continue;
										}
										flag2 = true;
										icon_no = num26;
										break;
									}
									break;
								}
								if (!flag2)
								{
									arrayList.Add(dataSet6.Tables[0].Rows[num23]["cin_room_all"].ToString());
									icon_no = arrayList.Count - 1;
								}
								break;
							}
							obj3 = "ย\u0e31งไม\u0e48ได\u0e49 Check-Out\r\nออกว\u0e31นท\u0e35\u0e48\r\n" + text4;
							check_timeout(dateout, Conversions.ToString(dataSet.Tables[0].Rows[num12]["room_NO"]));
						}
						else
						{
							cin_no = "";
						}
						if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num12]["room_clean"], "yes", TextCompare: false))
						{
							text = "รอ ทำความสะอาด";
							cin_no = "";
							if (Operators.CompareString(dataSet.Tables[0].Rows[num12]["Room_Clean_Time"].ToString(), "", TextCompare: false) != 0)
							{
								int num28 = (int)DateAndTime.DateDiff(DateInterval.Minute, DateTime.Now, DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num12]["Room_Clean_Time"])).AddMinutes(Convert.ToDouble(Module1.decimal_1)));
								text = "กำล\u0e31ง ทำความสะอาด\r\nเหล\u0e37อ " + Conversions.ToString(num28) + " นาท\u0e35";
							}
						}
					}
					if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(obj3, "เข\u0e49าพ\u0e31ก", TextCompare: false), Operators.CompareObjectEqual(obj3, "ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก", TextCompare: false))))
					{
						if (Operators.CompareString(text3, "", TextCompare: false) != 0)
						{
							obj3 = ((text3.IndexOf("รายเด\u0e37อน") == -1) ? Operators.ConcatenateObject(Operators.ConcatenateObject(obj3, "\r\n"), text3) : Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(obj3, "\r\n"), text3), "\r\n"), text2));
						}
						else
						{
							obj3 = Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(obj3, "\r\n"), "ออกว\u0e31นท\u0e35\u0e48"), "\r\n"), text2);
							check_timeout(dateout, Conversions.ToString(dataSet.Tables[0].Rows[num12]["room_NO"]));
						}
					}
					string power_on = Conversions.ToString(dataSet.Tables[0].Rows[num12]["Room_Power_STATUS"]);
					if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num12]["room_clean"], "yes", TextCompare: false))
					{
						obj3 = "รอ ทำความสะอาด";
						text = "รอ ทำความสะอาด";
						cin_no = "";
						if (Operators.CompareString(dataSet.Tables[0].Rows[num12]["Room_Clean_Time"].ToString(), "", TextCompare: false) != 0)
						{
							int num29 = (int)DateAndTime.DateDiff(DateInterval.Minute, DateTime.Now, DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num12]["Room_Clean_Time"])).AddMinutes(Convert.ToDouble(Module1.decimal_1)));
							obj3 = "กำล\u0e31ง ทำความสะอาด\r\nเหล\u0e37อ " + Conversions.ToString(num29) + " นาท\u0e35";
							text = "กำล\u0e31ง ทำความสะอาด\r\nเหล\u0e37อ " + Conversions.ToString(num29) + " นาท\u0e35";
							if (num29 <= 0)
							{
								string right = "";
								string right2 = "";
								DataSet dataSet8 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 cin_no,cin_cust_name from View_CheckIn_Ds where Cin_room_status='Check-Out' and cin_room_no='", dataSet.Tables[0].Rows[num12]["room_no"]), "' order by cin_room_out desc")));
								if (dataSet8.Tables[0].Rows.Count != 0)
								{
									right = Conversions.ToString(dataSet8.Tables[0].Rows[0]["cin_no"]);
									right2 = Conversions.ToString(dataSet8.Tables[0].Rows[0]["cin_cust_name"]);
								}
								Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='no',Room_Clean_Time='' where id=", dataSet.Tables[0].Rows[num12]["id"])));
								Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("INSERT INTO HT_Housewife ([h_name],[h_room],[h_date],[h_note],[h_cin],[h_cin_name]) VALUES ('", Module1.loginName), "', '"), dataSet.Tables[0].Rows[num12]["room_no"]), "', '"), DateTime.Now), "', 'ป\u0e34ดโดยโปรแกรม','"), right), "','"), right2), "')")));
								Module1.Power_set(Conversions.ToString(dataSet.Tables[0].Rows[num12]["room_no"]), "OFF", "", "ป\u0e34ดไฟ อ\u0e31ตโนม\u0e31ต\u0e34 เน\u0e37\u0e48องจากหมดเวลาทำความสะอาด");
							}
						}
					}
					if (Operators.CompareString(dataSet.Tables[0].Rows[num12]["room_book"].ToString(), "", TextCompare: false) != 0)
					{
						obj3 = "จอง\r\n" + dataSet.Tables[0].Rows[num12]["room_book_name"].ToString() + "\r\nเวลา : " + dataSet.Tables[0].Rows[num12]["room_book_time"].ToString();
					}
					if (Operators.CompareString(dataSet.Tables[0].Rows[num12]["Room_Manternace"].ToString(), "yes", TextCompare: false) == 0)
					{
						obj3 = "ซ\u0e48อม";
						text = "ซ\u0e48อมบำร\u0e38ง";
					}
					int num30 = arrayList2.Count - 1;
					int num31 = 0;
					while (true)
					{
						int num32 = num31;
						num4 = num30;
						if (num32 > num4)
						{
							break;
						}
						if (!Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList2[num31], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num12]["room_type"], TextCompare: false))
						{
							num31++;
							continue;
						}
						if (decimal.Compare(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList2[num31], new object[1] { 1 }, null)), 0m) > 0)
						{
							NewLateBinding.LateIndexSetComplex(arrayList2[num31], new object[2]
							{
								1,
								decimal.Subtract(Conversions.ToDecimal(NewLateBinding.LateIndexGet(arrayList2[num31], new object[1] { 1 }, null)), 1m)
							}, null, OptimisticSet: false, RValueBase: true);
						}
						break;
					}
					int num33 = arrayList3.Count - 1;
					int num34 = 0;
					while (true)
					{
						int num35 = num34;
						num4 = num33;
						if (num35 > num4)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num12]["Room_Type"], TextCompare: false))
						{
							if (obj3.ToString().IndexOf("จอง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									3,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 3 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj3.ToString().IndexOf("ป\u0e34ดปร\u0e31บปร\u0e38ง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj3.ToString().IndexOf("เข\u0e49าพ\u0e31ก") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									2,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 2 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj3.ToString().IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									5,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 5 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj3.ToString().IndexOf("รอ ทำความสะอาด") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj3.ToString().IndexOf("กำล\u0e31ง ทำความสะอาด") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									4,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 4 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
							else if (obj3.ToString().IndexOf("ว\u0e48าง") == 0)
							{
								NewLateBinding.LateIndexSetComplex(arrayList3[num34], new object[2]
								{
									1,
									Operators.AddObject(NewLateBinding.LateIndexGet(arrayList3[num34], new object[1] { 1 }, null), 1)
								}, null, OptimisticSet: false, RValueBase: true);
							}
						}
						num34++;
					}
					if (!IS_LIST_ROOM | (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0))
					{
						if (Operators.CompareString(ComboBoxEx1.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) == 0)
						{
							method_0(r_NO, r_TYPE, Conversions.ToString(obj3), Conversions.ToString(obj2), num14, num15, Conversions.ToString(objectValue), dataSet4, dataSet5, text, power_on, Conversions.ToDouble(value3), Conversions.ToDouble(value4), icon_no, cin_no);
						}
						else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num12]["Room_Group"], ComboBoxEx1.Text, TextCompare: false))
						{
							method_0(r_NO, r_TYPE, Conversions.ToString(obj3), Conversions.ToString(obj2), num14, num15, Conversions.ToString(objectValue), dataSet4, dataSet5, text, power_on, Conversions.ToDouble(value3), Conversions.ToDouble(value4), icon_no, cin_no);
						}
					}
					else if (Operators.CompareString(ComboBoxEx1.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) == 0)
					{
						SETButton_Notclear(r_NO, r_TYPE, Conversions.ToString(obj3), Conversions.ToString(obj2), num14, num15, Conversions.ToString(objectValue), dataSet4, dataSet5, text, power_on, icon_no, cin_no);
					}
					else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num12]["Room_Group"], ComboBoxEx1.Text, TextCompare: false))
					{
						SETButton_Notclear(r_NO, r_TYPE, Conversions.ToString(obj3), Conversions.ToString(obj2), num14, num15, Conversions.ToString(objectValue), dataSet4, dataSet5, text, power_on, icon_no, cin_no);
					}
					Application.DoEvents();
					num12++;
				}
				arrayList3 = Load_Booking(arrayList3);
				if (!IS_LIST_ROOM | (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0))
				{
					SET_STATUS(arrayList3);
				}
				else
				{
					SET_STATUS_NOTCLEAR(arrayList3);
				}
				SET_SMS();
			}
			if (!IS_LIST_ROOM)
			{
				FlowLayoutPanel1.Show();
				FlowLayoutPanel1.ResumeLayout();
			}
			IS_LIST_ROOM = true;
			dataSet.Dispose();
			dataSet2.Dispose();
			dataSet3.Dispose();
			dataSet4.Dispose();
			dataSet5.Dispose();
			dataSet7.Dispose();
			arrayList2.Clear();
			arrayList3.Clear();
			Timer1.Enabled = false;
			Timer1.Enabled = true;
			FlowLayoutPanel1.ResumeLayout();
			AutoAddBookingRooms(dataSet);
			Cursor = Cursors.Default;
		}
	}

	public void AutoAddBookingRooms(DataSet r_all)
	{
		int num = 0;
		checked
		{
			int num2 = FlowLayoutPanel4.Controls.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				int num6 = FlowLayoutPanel1.Controls.Count - 1;
				int num7 = 0;
				while (true)
				{
					int num8 = num7;
					num5 = num6;
					if (num8 > num5)
					{
						break;
					}
					if (Operators.CompareString(FlowLayoutPanel4.Controls[num3].Controls[0].Text, FlowLayoutPanel1.Controls[num7].Controls[1].Text, TextCompare: false) == 0)
					{
						bool flag = false;
						DataRow[] array = r_all.Tables[0].Select("Room_no = '" + FlowLayoutPanel1.Controls[num7].Controls[1].Text + "'");
						if (array.Length != 0 && Conversions.ToBoolean(Operators.AndObject(Operators.AndObject(Operators.CompareObjectEqual(array[0]["Room_Use"], "no", TextCompare: false), Operators.CompareObjectEqual(array[0]["Room_Clean"], "no", TextCompare: false)), Operators.CompareObjectEqual(array[0]["Room_Manternace"], "no", TextCompare: false))))
						{
							flag = true;
						}
						if (flag)
						{
							DataSet dataSet = Module1.connect("select * from View_Book_Date where id=" + FlowLayoutPanel4.Controls[num3].Controls[1].Name);
							DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Book_Ds where Book_No='", dataSet.Tables[0].Rows[0]["book_no"]), "'")));
							string text = "";
							int num9 = dataSet2.Tables[0].Rows.Count - 1;
							int num10 = 0;
							while (true)
							{
								int num11 = num10;
								num5 = num9;
								if (num11 > num5)
								{
									break;
								}
								text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Conversions.ToString(num10 + 1) + ". ", dataSet2.Tables[0].Rows[num10]["book_room_type"]), "  "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["book_room_start"]), "(dd/MM HH:mm)")), " ถ\u0e36ง "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num10]["book_room_end"]), "(dd/MM HH:mm)")), "\r\n")));
								num10++;
							}
							string text2 = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["Book_Cust_Name"], dataSet.Tables[0].Rows[0]["Book_Cust_Name2"]), "\r\n"), dataSet.Tables[0].Rows[0]["Book_Cust_Tel"]), "\r\n"), "เวลาเข\u0e49าพ\u0e31ก : "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Book_Date_in"]), "HH:mm")), "\r\n"), "\r\n"), text), "\r\n"), "หมายเหต\u0e38 : "), dataSet.Tables[0].Rows[0]["Book_room_note"]));
							Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("update HT_Rooms set room_book_ds='" + text2, "', Room_Book='"), dataSet.Tables[0].Rows[0]["id"]), "',Room_Book_Name='"), dataSet.Tables[0].Rows[0]["Book_Cust_Name"]), " "), dataSet.Tables[0].Rows[0]["Book_Cust_Name2"]), "',Room_Book_Time='"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Book_Date_in"]), "HH:mm")), "' where Room_no='"), FlowLayoutPanel4.Controls[num3].Controls[0].Text), "'")));
							Module1.connect(Conversions.ToString(Operators.ConcatenateObject("update HT_Book_Date set Book_ok=Book_ok+1 where id=", dataSet.Tables[0].Rows[0]["id"])));
							num++;
							break;
						}
					}
					num7++;
				}
				num3++;
			}
			if (num > 0)
			{
				Module1.IsListroom = false;
				DateTimePicker1.Value = DateTime.Now;
				ClearCheck();
			}
		}
	}

	public string Convert_Time(DateTime D1, DateTime D2, string ROOM_NO)
	{
		string text = "";
		int num = 0;
		checked
		{
			num = (int)DateAndTime.DateDiff(DateInterval.Minute, D1, D2);
			if (num <= 0)
			{
				text = Conversions.ToString(num) + " น.";
				if (num >= -2)
				{
					Module1.PlaySound();
					if (Module1.bool_3)
					{
						Module1.Power_set(ROOM_NO, "OFF", "", "ป\u0e34ดไฟ อ\u0e31ตโนม\u0e31ต\u0e34 เน\u0e37\u0e48องจาก หมดเวลา");
					}
				}
			}
			else if (num <= 59)
			{
				text = Conversions.ToString(num) + " น.";
			}
			else
			{
				int num2 = (int)Math.Round(Math.Floor((double)num / 60.0));
				text = ((num - num2 * 60 <= 0) ? (Conversions.ToString(num2) + " ชม.") : (Conversions.ToString(num2) + ":" + Strings.Format(num - num2 * 60, "00") + " ชม."));
			}
			return text;
		}
	}

	public ArrayList Load_Booking(ArrayList aa)
	{
		checked
		{
			ArrayList result;
			try
			{
				FlowLayoutPanel4.Controls.Clear();
				DateTime dateTime = DateTimePicker1.Value;
				if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(dateTime, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
				{
					dateTime = dateTime.AddDays(-1.0);
				}
				int num = 0;
				DataSet dataSet = Module1.connect("select * from View_Book_Date where Book_Status='จอง' and Book_date_ds=" + Module1.datechar + Conversions.ToString(dateTime.Date) + Module1.datechar + " order by id");
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
						size = new Size(115, 80);
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
						panelEx.MouseUp += nodebtn_Book_click;
						panelEx.MouseDown += nodebtn_Book_Down;
						string bodyText = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num3]["Book_Cust_Name"], dataSet.Tables[0].Rows[num3]["Book_Cust_Name2"]), "\r\n"), dataSet.Tables[0].Rows[num3]["Book_Cust_Tel"]), "\r\n"), "เวลาเข\u0e49าพ\u0e31ก : "), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Book_Date_in"]), "HH:mm")), "\r\n"), "\r\n"), dataSet.Tables[0].Rows[num3]["Book_room_all"]), "\r\n"), "หมายเหต\u0e38 : "), dataSet.Tables[0].Rows[num3]["Book_room_note"]), "\r\n"), "\r\n"), "*ถ\u0e49าจะแก\u0e49ไขให\u0e49คล\u0e34\u0e4aกขวาในรายการจองห\u0e49องท\u0e35\u0e48ย\u0e31งไม\u0e48ได\u0e49ด\u0e36ง"));
						SuperTooltip1.SetSuperTooltip(panelEx, new SuperTooltipInfo("รายการจองห\u0e49องพ\u0e31ก", "iHOTEL", bodyText, Resources.boy_emoticon_009, null, eTooltipColor.Teal));
						panelEx2.Dock = DockStyle.Top;
						panelEx2.CanvasColor = SystemColors.Control;
						panelEx2.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
						location = new Point(0, 0);
						panelEx2.Location = location;
						margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
						panelEx2.Margin = margin;
						panelEx2.Name = Conversions.ToString(dataSet.Tables[0].Rows[num3]["book_type"]);
						size = new Size(115, 18);
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
						try
						{
							FlowLayoutPanel4.Controls.Add(panel);
						}
						catch (Exception projectError)
						{
							ProjectData.SetProjectError(projectError);
							ProjectData.ClearProjectError();
						}
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
				result = aa;
			}
			catch (Exception projectError2)
			{
				ProjectData.SetProjectError(projectError2);
				result = aa;
				ProjectData.ClearProjectError();
			}
			return result;
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
			label7.BackColor = Color.FromArgb(215, 215, 215);
			label7.ForeColor = Color.FromArgb(0, 0, 142);
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
			label8.BackColor = Color.FromArgb(122, 221, 122);
			label8.ForeColor = Color.FromArgb(0, 0, 162);
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
			label9.BackColor = Color.FromArgb(225, 98, 98);
			label9.ForeColor = Color.FromArgb(0, 0, 162);
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
			label10.BackColor = Color.FromArgb(225, 225, 0);
			label10.ForeColor = Color.FromArgb(0, 0, 162);
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
			label11.BackColor = Color.FromArgb(225, 162, 98);
			label11.ForeColor = Color.FromArgb(0, 0, 162);
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
			label12.BackColor = Color.FromArgb(0, 225, 225);
			label12.ForeColor = Color.FromArgb(0, 0, 162);
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

	public void SET_STATUS_NOTCLEAR(ArrayList aa)
	{
		int num = 0;
		int num2 = 0;
		int num3 = 0;
		int num4 = 0;
		int num5 = 0;
		int num6 = 0;
		checked
		{
			int num7 = aa.Count - 1;
			int num8 = 0;
			while (true)
			{
				int num9 = num8;
				int num10 = num7;
				if (num9 > num10)
				{
					break;
				}
				try
				{
					Label label = (Label)FlowLayoutPanel3.Controls[num6];
					num6++;
					Label label2 = (Label)FlowLayoutPanel3.Controls[num6];
					num6++;
					Label label3 = (Label)FlowLayoutPanel3.Controls[num6];
					num6++;
					Label label4 = (Label)FlowLayoutPanel3.Controls[num6];
					num6++;
					Label label5 = (Label)FlowLayoutPanel3.Controls[num6];
					num6++;
					Label label6 = (Label)FlowLayoutPanel3.Controls[num6];
					num6++;
					label.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num8], new object[1] { 0 }, null));
					label3.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num8], new object[1] { 1 }, null));
					label4.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num8], new object[1] { 2 }, null));
					label5.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num8], new object[1] { 3 }, null));
					label6.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num8], new object[1] { 4 }, null));
					label2.Text = Conversions.ToString(NewLateBinding.LateIndexGet(aa[num8], new object[1] { 5 }, null));
					num = Conversions.ToInteger(Operators.AddObject(num, NewLateBinding.LateIndexGet(aa[num8], new object[1] { 1 }, null)));
					num2 = Conversions.ToInteger(Operators.AddObject(num2, NewLateBinding.LateIndexGet(aa[num8], new object[1] { 2 }, null)));
					num3 = Conversions.ToInteger(Operators.AddObject(num3, NewLateBinding.LateIndexGet(aa[num8], new object[1] { 3 }, null)));
					num4 = Conversions.ToInteger(Operators.AddObject(num4, NewLateBinding.LateIndexGet(aa[num8], new object[1] { 4 }, null)));
					num5 = Conversions.ToInteger(Operators.AddObject(num5, NewLateBinding.LateIndexGet(aa[num8], new object[1] { 5 }, null)));
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
				num8++;
			}
			Label label7 = (Label)FlowLayoutPanel3.Controls[FlowLayoutPanel3.Controls.Count - 6];
			Label label8 = (Label)FlowLayoutPanel3.Controls[FlowLayoutPanel3.Controls.Count - 5];
			Label label9 = (Label)FlowLayoutPanel3.Controls[FlowLayoutPanel3.Controls.Count - 4];
			Label label10 = (Label)FlowLayoutPanel3.Controls[FlowLayoutPanel3.Controls.Count - 3];
			Label label11 = (Label)FlowLayoutPanel3.Controls[FlowLayoutPanel3.Controls.Count - 2];
			Label label12 = (Label)FlowLayoutPanel3.Controls[FlowLayoutPanel3.Controls.Count - 1];
			label7.Text = "รวม (" + Conversions.ToString(num + num2 + num3 + num4 + num5) + ")";
			label9.Text = Conversions.ToString(num);
			label10.Text = Conversions.ToString(num2);
			label11.Text = Conversions.ToString(num3);
			label12.Text = Conversions.ToString(num4);
			label8.Text = Conversions.ToString(num5);
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
		if (!MSSQL.CodeErr && !ButtonX4.Checked)
		{
			DateTimePicker1.Value = DateTime.Now;
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
		checked
		{
			if (ButtonX4.Checked)
			{
				Cursor = Cursors.WaitCursor;
				ButtonX4.Checked = false;
				FlowLayoutPanel1.BackColor = SystemColors.GradientActiveCaption;
				Timer1.Enabled = true;
				FlowLayoutPanel1.VerticalScroll.Value = 0;
				FlowLayoutPanel1.HorizontalScroll.Value = 0;
				Refresh();
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
					Module1.connect("update HT_Rooms set Room_X=" + Conversions.ToString(FlowLayoutPanel1.Controls[num2].Location.X) + ",Room_y=" + Conversions.ToString(FlowLayoutPanel1.Controls[num2].Location.Y) + " where Room_no='" + FlowLayoutPanel1.Controls[num2].Controls[1].Text + "'");
					num2++;
				}
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
				Cursor = Cursors.Default;
			}
			else
			{
				Cursor = Cursors.WaitCursor;
				ButtonX4.Checked = true;
				FlowLayoutPanel1.BackColor = Color.NavajoWhite;
				Timer1.Enabled = false;
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
				Cursor = Cursors.Default;
			}
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
		Timer2.Enabled = false;
		ListGroup();
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

	private void BOOK_PANEL_Click(object sender, EventArgs e)
	{
	}

	private void FormRoomMain_Validated(object sender, EventArgs e)
	{
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		Module1.ISfullscreen = true;
		Close();
	}

	private void ButtonX8_Click(object sender, EventArgs e)
	{
		checked
		{
			if (ButtonX8.Text.IndexOf("ขยาย") != -1)
			{
				Panel flowLayoutPanel = FlowLayoutPanel1;
				Size size = new Size(FlowLayoutPanel1.Width + 245 + 35, FlowLayoutPanel1.Height);
				flowLayoutPanel.Size = size;
				ButtonX buttonX = ButtonX8;
				Point point = ButtonX8.Location;
				Point location = new Point(point.X + 245, ButtonX8.Location.Y);
				buttonX.Location = location;
				ButtonX8.Text = "ย\u0e48อ\r\n<<";
				ButtonX8.Tooltip = "ย\u0e48อหน\u0e49าจอ";
				point = (ButtonX_0.Location = new Point(ButtonX8.Location.X, ButtonX8.Location.Y - ButtonX_0.Height + 1));
				point = (ButtonX9.Location = new Point(ButtonX8.Location.X - ButtonX_0.Width + 1, ButtonX8.Location.Y));
				point = (ButtonX_1.Location = new Point(ButtonX8.Location.X - ButtonX_0.Width + 1, ButtonX8.Location.Y - ButtonX_0.Height + 1));
			}
			else
			{
				Panel flowLayoutPanel2 = FlowLayoutPanel1;
				Size size = new Size(FlowLayoutPanel1.Width - 245 - 35, FlowLayoutPanel1.Height);
				flowLayoutPanel2.Size = size;
				ButtonX buttonX2 = ButtonX8;
				Point point = new Point(ButtonX8.Location.X - 245, ButtonX8.Location.Y);
				buttonX2.Location = point;
				ButtonX8.Text = "ขยาย\r\n>>";
				ButtonX8.Tooltip = "ขยายหน\u0e49าจอ";
				point = (ButtonX9.Location = new Point(ButtonX8.Location.X + ButtonX8.Width - 1, ButtonX8.Location.Y));
				point = (ButtonX_0.Location = new Point(ButtonX8.Location.X + ButtonX8.Width + ButtonX9.Width - 2, ButtonX8.Location.Y));
				point = (ButtonX_1.Location = new Point(ButtonX8.Location.X + ButtonX8.Width + ButtonX9.Width + ButtonX_0.Width - 3, ButtonX8.Location.Y));
			}
		}
	}

	private void Timer4_Tick(object sender, EventArgs e)
	{
		Timer4.Enabled = false;
		MSSQL.CodeErr = false;
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		FrmAddBook2 frmAddBook = new FrmAddBook2();
		frmAddBook.ShowDialog();
	}

	private void ButtonX10_Click(object sender, EventArgs e)
	{
		FrmAddBook frmAddBook = new FrmAddBook();
		frmAddBook.ShowDialog();
	}

	private void ButtonX11_Click(object sender, EventArgs e)
	{
		Module1.smethod_0();
		MyProject.Forms.FormRoomMain_ViewBook.Close();
		MyProject.Forms.FormRoomMain_ViewBook.Show();
	}

	private void ButtonX12_Click(object sender, EventArgs e)
	{
		method_1();
	}

	public void method_1()
	{
		ListView1.Items.Clear();
		PanelEx4.Visible = true;
		DataSet dataSet = Module1.connect("select * from HT_Rooms order by room_no");
		DataSet dataSet2 = Module1.connect("select room_no,Cust_name,Cust_Work_Name,cin_room_status,Cin_Date_out,cin_type,Cust_Add_tel,Cin_Car_id from View_Room_All where  (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก') order by Cin_Date_out");
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
				if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) == 0)
				{
					string text = "";
					if (Operators.CompareString(dataSet.Tables[0].Rows[num2]["room_book_name"].ToString(), "", TextCompare: false) != 0)
					{
						text = dataSet.Tables[0].Rows[num2]["room_book_name"].ToString();
						ListView1.Items.Add(dataSet.Tables[0].Rows[num2]["room_no"].ToString());
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("จอง");
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(text);
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ListView1.Items[ListView1.Items.Count - 1].BackColor = Color.Yellow;
					}
					else
					{
						int num5 = dataSet2.Tables[0].Rows.Count - 1;
						int num6 = 0;
						while (true)
						{
							int num7 = num6;
							num4 = num5;
							if (num7 > num4)
							{
								break;
							}
							if (Operators.CompareString(dataSet2.Tables[0].Rows[num6]["room_no"].ToString(), dataSet.Tables[0].Rows[num2]["room_no"].ToString(), TextCompare: false) != 0)
							{
								num6++;
								continue;
							}
							string text2 = dataSet2.Tables[0].Rows[num6]["Cust_name"].ToString() + " / " + dataSet2.Tables[0].Rows[num6]["Cust_Work_Name"].ToString();
							ListView1.Items.Add(dataSet2.Tables[0].Rows[num6]["room_no"].ToString());
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num6]["cin_room_status"].ToString());
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(text2 + " [ออกว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["Cin_Date_out"]), "dd/MM/yy HH:mm") + "]");
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num6]["cin_type"].ToString());
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num6]["Cust_Add_tel"].ToString());
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num6]["Cin_Car_id"].ToString());
							if (Operators.CompareString(dataSet2.Tables[0].Rows[num6]["cin_room_status"].ToString(), "เข\u0e49าพ\u0e31ก", TextCompare: false) == 0)
							{
								ListView1.Items[ListView1.Items.Count - 1].BackColor = Color.LightPink;
							}
							break;
						}
					}
				}
				else
				{
					string text3 = "";
					if (Operators.CompareString(dataSet.Tables[0].Rows[num2]["room_book_name"].ToString(), "", TextCompare: false) != 0)
					{
						text3 = dataSet.Tables[0].Rows[num2]["room_book_name"].ToString();
						if (text3.IndexOf(TextBox1.Text) != -1)
						{
							ListView1.Items.Add(dataSet.Tables[0].Rows[num2]["room_no"].ToString());
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("จอง");
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(text3);
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
							ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
							ListView1.Items[ListView1.Items.Count - 1].BackColor = Color.Yellow;
						}
					}
					else
					{
						int num8 = dataSet2.Tables[0].Rows.Count - 1;
						int num9 = 0;
						while (true)
						{
							int num10 = num9;
							num4 = num8;
							if (num10 > num4)
							{
								break;
							}
							if (Operators.CompareString(dataSet2.Tables[0].Rows[num9]["room_no"].ToString(), dataSet.Tables[0].Rows[num2]["room_no"].ToString(), TextCompare: false) != 0)
							{
								num9++;
								continue;
							}
							string text4 = dataSet2.Tables[0].Rows[num9]["Cust_name"].ToString() + " / " + dataSet2.Tables[0].Rows[num9]["Cust_Work_Name"].ToString();
							string text5 = dataSet2.Tables[0].Rows[num9]["Cust_Add_tel"].ToString() + " / " + dataSet2.Tables[0].Rows[num9]["Cin_Car_id"].ToString();
							if ((text4.IndexOf(TextBox1.Text) != -1) | (text5.IndexOf(TextBox1.Text) != -1))
							{
								ListView1.Items.Add(dataSet2.Tables[0].Rows[num9]["room_no"].ToString());
								ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num9]["cin_room_status"].ToString());
								ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(text4 + " [ออกว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num9]["Cin_Date_out"]), "dd/MM/yy HH:mm") + "]");
								ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num9]["cin_type"].ToString());
								ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num9]["Cust_Add_tel"].ToString());
								ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet2.Tables[0].Rows[num9]["Cin_Car_id"].ToString());
								if (Operators.CompareString(dataSet2.Tables[0].Rows[num9]["cin_room_status"].ToString(), "เข\u0e49าพ\u0e31ก", TextCompare: false) == 0)
								{
									ListView1.Items[ListView1.Items.Count - 1].BackColor = Color.LightPink;
								}
							}
							break;
						}
					}
				}
				num2++;
			}
		}
	}

	private void TextBox1_GotFocus(object sender, EventArgs e)
	{
		method_1();
	}

	private void TextBox1_TextChanged(object sender, EventArgs e)
	{
		if (PanelEx4.Visible)
		{
			TimerSearch.Enabled = false;
			TimerSearch.Enabled = true;
		}
	}

	private void ButtonX12_Click_1(object sender, EventArgs e)
	{
		ListView1.Items.Clear();
		FlowLayoutPanel1.Focus();
		PanelEx4.Visible = false;
		TextBox1.Text = "";
	}

	private void TimerSearch_Tick(object sender, EventArgs e)
	{
		TimerSearch.Enabled = false;
		method_1();
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[1].Text, "จอง", TextCompare: false) == 0)
		{
			CHK_Array.Clear();
			MyProject.Forms.ClickBook.RoomNo = ListView1.SelectedItems[0].SubItems[0].Text;
			MyProject.Forms.ClickBook.RoomArr = CHK_Array;
			MyProject.Forms.ClickBook.ShowDialog();
			if (MyProject.Forms.ClickBook.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
		else
		{
			if (!((Operators.CompareString(ListView1.SelectedItems[0].SubItems[1].Text, "เข\u0e49าพ\u0e31ก", TextCompare: false) == 0) | (Operators.CompareString(ListView1.SelectedItems[0].SubItems[1].Text, "ย\u0e31งไม\u0e48ได\u0e49 Check-Out", TextCompare: false) == 0)))
			{
				return;
			}
			if (ListView1.SelectedItems[0].SubItems[1].Text.IndexOf("ย\u0e31งไม\u0e48ได\u0e49 Check-Out") == 0)
			{
				MyProject.Forms.ClickUSE.ButtonX_6.Visible = true;
			}
			if (ListView1.SelectedItems[0].SubItems[1].Text.IndexOf("เข\u0e49าพ\u0e31ก") == 0)
			{
				MyProject.Forms.ClickUSE.ButtonX7.Enabled = false;
				MyProject.Forms.ClickUSE.ButtonX_6.Visible = true;
			}
			else
			{
				MyProject.Forms.ClickUSE.ButtonX7.Enabled = true;
			}
			if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[3].Text, "1", TextCompare: false) == 0)
			{
				MyProject.Forms.ClickUSE.ButtonX_1.Enabled = false;
				MyProject.Forms.ClickUSE.ButtonX7.Enabled = false;
				MyProject.Forms.ClickUSE.ButtonX_6.Visible = true;
			}
			else
			{
				MyProject.Forms.ClickUSE.ButtonX_1.Enabled = true;
			}
			if (MyProject.Forms.ClickUSE.ButtonX_1.Enabled)
			{
				if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[3].Text, "2", TextCompare: false) == 0)
				{
					MyProject.Forms.ClickUSE.ButtonX_1.Enabled = false;
					MyProject.Forms.ClickUSE.ButtonX7.Enabled = false;
					MyProject.Forms.ClickUSE.ButtonX_6.Visible = false;
				}
				else
				{
					MyProject.Forms.ClickUSE.ButtonX_1.Enabled = true;
				}
			}
			CHK_Array.Clear();
			MyProject.Forms.ClickUSE.RoomNo = ListView1.SelectedItems[0].SubItems[0].Text;
			MyProject.Forms.ClickUSE.RoomArr = CHK_Array;
			MyProject.Forms.ClickUSE.ShowDialog();
			if (MyProject.Forms.ClickUSE.ISOK)
			{
				LoadRooms(FlowLayoutPanel1.AutoScrollPosition.X, FlowLayoutPanel1.AutoScrollPosition.Y);
			}
			ClearCheck();
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void ButtonX13_Click(object sender, EventArgs e)
	{
		TextBox1.Text = "แสดงท\u0e31\u0e49งหมด";
	}
}
