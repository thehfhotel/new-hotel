using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Net;
using System.Runtime.CompilerServices;
using System.ServiceProcess;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSelectDB : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("PanelEx5")]
	private PanelEx _PanelEx5;

	[AccessedThroughProperty("ButtonX8")]
	private ButtonX _ButtonX8;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	[AccessedThroughProperty("ButtonX9")]
	private ButtonX _ButtonX9;

	[AccessedThroughProperty("ButtonX10")]
	private ButtonX _ButtonX10;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("ButtonX11")]
	private ButtonX _ButtonX11;

	[AccessedThroughProperty("ButtonMSSQL")]
	private ButtonX _ButtonMSSQL;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("Label_com")]
	private Label _Label_com;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ButtonX13")]
	private ButtonX _ButtonX13;

	[AccessedThroughProperty("TimerwaitServer")]
	private Timer _TimerwaitServer;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("ButtonItem2")]
	private ButtonItem _ButtonItem2;

	[AccessedThroughProperty("TimerwaitServer2")]
	private Timer _TimerwaitServer2;

	public bool ISOK;

	public string Return_Select_DB;

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
			EventHandler value2 = PanelEx1_Click;
			if (_PanelEx1 != null)
			{
				_PanelEx1.Click -= value2;
			}
			_PanelEx1 = value;
			if (_PanelEx1 != null)
			{
				_PanelEx1.Click += value2;
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
			EventHandler value2 = ListView1_SelectedIndexChanged;
			KeyEventHandler value3 = ListView1_KeyDown;
			EventHandler value4 = ListView1_DoubleClick;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
				_ListView1.KeyDown -= value3;
				_ListView1.DoubleClick -= value4;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
				_ListView1.KeyDown += value3;
				_ListView1.DoubleClick += value4;
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
			EventHandler value2 = Label2_Click;
			if (_Label2 != null)
			{
				_Label2.Click -= value2;
			}
			_Label2 = value;
			if (_Label2 != null)
			{
				_Label2.Click += value2;
			}
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

	internal virtual PanelEx PanelEx5
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = PanelEx5_Click;
			if (_PanelEx5 != null)
			{
				_PanelEx5.Click -= value2;
			}
			_PanelEx5 = value;
			if (_PanelEx5 != null)
			{
				_PanelEx5.Click += value2;
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

	internal virtual CheckBox CheckBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = CheckBox1_CheckedChanged;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged -= value2;
			}
			_CheckBox1 = value;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged += value2;
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
			EventHandler value2 = ButtonX11_Click_1;
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

	internal virtual ButtonX ButtonX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonMSSQL;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonMSSQL_Click;
			if (_ButtonMSSQL != null)
			{
				_ButtonMSSQL.Click -= value2;
			}
			_ButtonMSSQL = value;
			if (_ButtonMSSQL != null)
			{
				_ButtonMSSQL.Click += value2;
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
			EventHandler value2 = LabelX2_Click;
			if (_LabelX2 != null)
			{
				_LabelX2.Click -= value2;
			}
			_LabelX2 = value;
			if (_LabelX2 != null)
			{
				_LabelX2.Click += value2;
			}
		}
	}

	internal virtual Label Label_com
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_com;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_com = value;
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

	internal virtual ColumnHeader ColumnHeader7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader7 = value;
		}
	}

	internal virtual ButtonX ButtonX_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX13_Click;
			if (_ButtonX13 != null)
			{
				_ButtonX13.Click -= value2;
			}
			_ButtonX13 = value;
			if (_ButtonX13 != null)
			{
				_ButtonX13.Click += value2;
			}
		}
	}

	internal virtual Timer TimerwaitServer
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerwaitServer;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TimerwaitServer = value;
		}
	}

	internal virtual ButtonItem ButtonItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem1_Click;
			if (_ButtonItem1 != null)
			{
				_ButtonItem1.Click -= value2;
			}
			_ButtonItem1 = value;
			if (_ButtonItem1 != null)
			{
				_ButtonItem1.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem2_Click;
			if (_ButtonItem2 != null)
			{
				_ButtonItem2.Click -= value2;
			}
			_ButtonItem2 = value;
			if (_ButtonItem2 != null)
			{
				_ButtonItem2.Click += value2;
			}
		}
	}

	internal virtual Timer TimerwaitServer2
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimerwaitServer2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimerwaitServer2_Tick;
			if (_TimerwaitServer2 != null)
			{
				_TimerwaitServer2.Tick -= value2;
			}
			_TimerwaitServer2 = value;
			if (_TimerwaitServer2 != null)
			{
				_TimerwaitServer2.Tick += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FormSelectDB()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSelectDB()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += login_selectDB_Load;
		base.Resize += FormSelectDB_Resize;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISOK = false;
		Return_Select_DB = "";
		InitializeComponent();
	}

	[DebuggerNonUserCode]
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormSelectDB));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx5 = new DevComponents.DotNetBar.PanelEx();
		this.Label1 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label_com = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX_2 = new DevComponents.DotNetBar.ButtonX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.TimerwaitServer = new System.Windows.Forms.Timer(this.components);
		this.TimerwaitServer2 = new System.Windows.Forms.Timer(this.components);
		this.ButtonX_3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX8 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem2 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX9 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1.SuspendLayout();
		this.PanelEx5.SuspendLayout();
		this.PanelEx2.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.PanelEx5);
		this.PanelEx1.Controls.Add(this.Label_com);
		this.PanelEx1.Controls.Add(this.Label3);
		this.PanelEx1.Controls.Add(this.PanelEx2);
		this.PanelEx1.Controls.Add(this.ButtonX9);
		this.PanelEx1.Controls.Add(this.CheckBox1);
		this.PanelEx1.Controls.Add(this.ButtonX_1);
		this.PanelEx1.Controls.Add(this.ButtonX5);
		this.PanelEx1.Controls.Add(this.ButtonX3);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(764, 361);
		panelEx3.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.Color = System.Drawing.Color.FromArgb(227, 239, 255);
		this.PanelEx1.Style.BackColor2.Color = System.Drawing.Color.FromArgb(175, 210, 255);
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.PanelEx5.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx5.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx5.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx5.Controls.Add(this.ButtonX_3);
		this.PanelEx5.Controls.Add(this.ButtonX_0);
		this.PanelEx5.Controls.Add(this.Label1);
		this.PanelEx5.Controls.Add(this.ButtonX8);
		this.PanelEx5.Controls.Add(this.ButtonX7);
		this.PanelEx5.Controls.Add(this.ListView1);
		this.PanelEx5.Controls.Add(this.ButtonX6);
		this.PanelEx5.Controls.Add(this.ButtonX2);
		this.PanelEx5.Controls.Add(this.ButtonX4);
		this.PanelEx5.Controls.Add(this.Label2);
		this.PanelEx5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx5;
		location = new System.Drawing.Point(11, 42);
		panelEx4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx5.Margin = margin;
		this.PanelEx5.Name = "PanelEx5";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx5;
		size = new System.Drawing.Size(741, 224);
		panelEx6.Size = size;
		this.PanelEx5.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx5.Style.BackColor1.Color = System.Drawing.Color.FromArgb(227, 239, 255);
		this.PanelEx5.Style.BackColor2.Color = System.Drawing.Color.FromArgb(175, 210, 255);
		this.PanelEx5.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx5.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx5.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx5.Style.GradientAngle = 90;
		this.PanelEx5.TabIndex = 0;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.RoyalBlue;
		this.Label1.ImageAlign = System.Drawing.ContentAlignment.TopRight;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(352, -2);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(379, 28);
		label2.Size = size;
		this.Label1.TabIndex = 18;
		this.Label1.Text = "&User name";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[7] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader7 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(4, 26);
		listView.Location = location;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(732, 159);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader1.Width = 30;
		this.ColumnHeader2.Text = "ประเภท";
		this.ColumnHeader2.Width = 90;
		this.ColumnHeader3.Text = "ท\u0e35\u0e48อย\u0e39\u0e48ไฟล\u0e4c หร\u0e37อ IP Address ฐานข\u0e49อม\u0e39ล";
		this.ColumnHeader3.Width = 260;
		this.ColumnHeader4.Text = "ช\u0e37\u0e48อฐานข\u0e49อม\u0e39ล";
		this.ColumnHeader4.Width = 150;
		this.ColumnHeader5.Text = "User";
		this.ColumnHeader5.Width = 0;
		this.ColumnHeader6.Text = "Pass";
		this.ColumnHeader6.Width = 0;
		this.ColumnHeader7.Text = "Comment";
		this.ColumnHeader7.Width = 170;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.Label2.ImageAlign = System.Drawing.ContentAlignment.TopRight;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(3, 5);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(705, 28);
		label4.Size = size;
		this.Label2.TabIndex = 19;
		this.Label2.Text = "กร\u0e38ณา เล\u0e37อก เซ\u0e34ฟเวอร\u0e4c เพ\u0e37\u0e48อเข\u0e49าใช\u0e49งานโปรแกรม";
		this.Label_com.Font = new System.Drawing.Font("Tahoma", 20f, System.Drawing.FontStyle.Bold | System.Drawing.FontStyle.Underline);
		this.Label_com.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		this.Label_com.ImageAlign = System.Drawing.ContentAlignment.TopRight;
		System.Windows.Forms.Label label_com = this.Label_com;
		location = new System.Drawing.Point(136, -3);
		label_com.Location = location;
		this.Label_com.Name = "Label_com";
		System.Windows.Forms.Label label_com2 = this.Label_com;
		size = new System.Drawing.Size(557, 48);
		label_com2.Size = size;
		this.Label_com.TabIndex = 25;
		this.Label_com.Text = "COM0";
		this.Label_com.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 20f);
		this.Label3.ForeColor = System.Drawing.Color.Green;
		this.Label3.ImageAlign = System.Drawing.ContentAlignment.TopRight;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(7, -2);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(328, 48);
		label6.Size = size;
		this.Label3.TabIndex = 24;
		this.Label3.Text = "ช\u0e37\u0e48อเคร\u0e37\u0e48อง :";
		this.Label3.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.PanelEx2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.LabelX2);
		this.PanelEx2.Controls.Add(this.ButtonX_2);
		this.PanelEx2.Controls.Add(this.LabelX1);
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx7 = this.PanelEx2;
		location = new System.Drawing.Point(11, 265);
		panelEx7.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx8 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx8.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx9 = this.PanelEx2;
		size = new System.Drawing.Size(741, 34);
		panelEx9.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.FromArgb(175, 210, 255);
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.FromArgb(175, 210, 255);
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 23;
		this.LabelX2.BackgroundStyle.Class = "";
		this.LabelX2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX labelX = this.LabelX2;
		location = new System.Drawing.Point(276, -1);
		labelX.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX2;
		size = new System.Drawing.Size(243, 35);
		labelX2.Size = size;
		this.LabelX2.TabIndex = 23;
		this.LabelX2.Text = "(เอาไว\u0e49ตรวจสอบเฉพาะเคร\u0e37\u0e48องท\u0e35\u0e48เป\u0e47นแม\u0e48)";
		this.LabelX2.TextAlignment = System.Drawing.StringAlignment.Center;
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX_2.FocusCuesEnabled = false;
		this.ButtonX_2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_2;
		location = new System.Drawing.Point(154, 4);
		buttonX_.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX_2 = this.ButtonX_2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX_2.Margin = margin;
		this.ButtonX_2.Name = "ButtonMSSQL";
		DevComponents.DotNetBar.ButtonX buttonX_3 = this.ButtonX_2;
		size = new System.Drawing.Size(155, 26);
		buttonX_3.Size = size;
		this.ButtonX_2.TabIndex = 20;
		this.ButtonX_2.Text = "MSSQL : ออนไลน\u0e4c";
		this.ButtonX_2.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX_2.Tooltip = "คล\u0e34\u0e4aกเพ\u0e37\u0e48อ Start Service";
		this.LabelX1.BackgroundStyle.Class = "";
		this.LabelX1.Font = new System.Drawing.Font("Tahoma", 11f, System.Drawing.FontStyle.Bold);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX1;
		location = new System.Drawing.Point(5, 5);
		labelX3.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX1;
		size = new System.Drawing.Size(152, 24);
		labelX4.Size = size;
		this.LabelX1.TabIndex = 22;
		this.LabelX1.Text = "สถานะเคร\u0e37\u0e48อง Server";
		this.CheckBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.CheckBox1.AutoSize = true;
		this.CheckBox1.Checked = true;
		this.CheckBox1.CheckState = System.Windows.Forms.CheckState.Checked;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(17, 336);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(15, 14);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 6;
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.Timer1.Enabled = true;
		this.Timer1.Interval = 200;
		this.TimerwaitServer.Enabled = true;
		this.TimerwaitServer.Interval = 300;
		this.TimerwaitServer2.Enabled = true;
		this.TimerwaitServer2.Interval = 300;
		this.ButtonX_3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_3.FocusCuesEnabled = false;
		this.ButtonX_3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_3.Image = (System.Drawing.Image)resources.GetObject("ButtonX13.Image");
		DevComponents.DotNetBar.ButtonX buttonX_4 = this.ButtonX_3;
		location = new System.Drawing.Point(263, 190);
		buttonX_4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX_5 = this.ButtonX_3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX_5.Margin = margin;
		this.ButtonX_3.Name = "ButtonX13";
		DevComponents.DotNetBar.ButtonX buttonX_6 = this.ButtonX_3;
		size = new System.Drawing.Size(129, 28);
		buttonX_6.Size = size;
		this.ButtonX_3.TabIndex = 20;
		this.ButtonX_3.Text = "ค\u0e49นหาเซ\u0e34ฟเวอร\u0e4c";
		this.ButtonX_3.Visible = false;
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_0.Image = iHOTEL2025.My.Resources.Resources._46__5_;
		DevComponents.DotNetBar.ButtonX buttonX_7 = this.ButtonX_0;
		location = new System.Drawing.Point(411, 190);
		buttonX_7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX_8 = this.ButtonX_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX_8.Margin = margin;
		this.ButtonX_0.Name = "ButtonX10";
		DevComponents.DotNetBar.ButtonX buttonX_9 = this.ButtonX_0;
		size = new System.Drawing.Size(237, 28);
		buttonX_9.Size = size;
		this.ButtonX_0.TabIndex = 6;
		this.ButtonX_0.Text = "ดาวน\u0e4cโหลดไฟล\u0e4cต\u0e34ดต\u0e31\u0e49งระบบเซ\u0e34ฟเวอร\u0e4c";
		this.ButtonX_0.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX8.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX8.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX8.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX8.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX8.Image = (System.Drawing.Image)resources.GetObject("ButtonX8.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX8;
		location = new System.Drawing.Point(659, 184);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX8;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX8.Name = "ButtonX8";
		this.ButtonX8.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX8;
		size = new System.Drawing.Size(39, 34);
		buttonX3.Size = size;
		this.ButtonX8.TabIndex = 4;
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX7;
		location = new System.Drawing.Point(697, 184);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX5.Margin = margin;
		this.ButtonX7.Name = "ButtonX7";
		this.ButtonX7.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX7;
		size = new System.Drawing.Size(39, 34);
		buttonX6.Size = size;
		this.ButtonX7.TabIndex = 5;
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX6.FocusCuesEnabled = false;
		this.ButtonX6.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX6.Image = iHOTEL2025.My.Resources.Resources.edit;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX6;
		location = new System.Drawing.Point(126, 190);
		buttonX7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX8.Margin = margin;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX6;
		size = new System.Drawing.Size(68, 28);
		buttonX9.Size = size;
		this.ButtonX6.TabIndex = 2;
		this.ButtonX6.Text = "แก\u0e49ไข";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = iHOTEL2025.My.Resources.Resources._11__5_;
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX2;
		location = new System.Drawing.Point(5, 190);
		buttonX10.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX11.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX2;
		size = new System.Drawing.Size(115, 28);
		buttonX12.Size = size;
		this.ButtonX2.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem1, this.ButtonItem2 });
		this.ButtonX2.TabIndex = 1;
		this.ButtonX2.Text = "เพ\u0e34\u0e48มเซ\u0e34ฟเวอร\u0e4c";
		this.ButtonItem1.GlobalItem = false;
		this.ButtonItem1.Image = (System.Drawing.Image)resources.GetObject("ButtonItem1.Image");
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.Text = "เพ\u0e34\u0e48มด\u0e49วยตนเอง";
		this.ButtonItem2.GlobalItem = false;
		this.ButtonItem2.Image = (System.Drawing.Image)resources.GetObject("ButtonItem2.Image");
		this.ButtonItem2.Name = "ButtonItem2";
		this.ButtonItem2.Text = "เพ\u0e34\u0e48มอ\u0e31ตโนม\u0e31ต\u0e34โดยการค\u0e49นหาเซ\u0e34ฟเวอร\u0e4cจากวง LAN";
		this.ButtonItem2.Visible = false;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = iHOTEL2025.My.Resources.Resources.delete1;
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX4;
		location = new System.Drawing.Point(201, 190);
		buttonX13.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX14.Margin = margin;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX4;
		size = new System.Drawing.Size(56, 28);
		buttonX15.Size = size;
		this.ButtonX4.TabIndex = 3;
		this.ButtonX4.Text = "ลบ";
		this.ButtonX9.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX9.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX9.FocusCuesEnabled = false;
		this.ButtonX9.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX9.Image = (System.Drawing.Image)resources.GetObject("ButtonX9.Image");
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX9;
		location = new System.Drawing.Point(151, 302);
		buttonX16.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX17.Margin = margin;
		this.ButtonX9.Name = "ButtonX9";
		DevComponents.DotNetBar.ButtonX buttonX18 = this.ButtonX9;
		size = new System.Drawing.Size(118, 25);
		buttonX18.Size = size;
		this.ButtonX9.TabIndex = 2;
		this.ButtonX9.Text = "Teamviewer";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX_1.FocusCuesEnabled = false;
		this.ButtonX_1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_1.Image = (System.Drawing.Image)resources.GetObject("ButtonX11.Image");
		DevComponents.DotNetBar.ButtonX buttonX_10 = this.ButtonX_1;
		location = new System.Drawing.Point(11, 330);
		buttonX_10.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX_11 = this.ButtonX_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX_11.Margin = margin;
		this.ButtonX_1.Name = "ButtonX11";
		DevComponents.DotNetBar.ButtonX buttonX_12 = this.ButtonX_1;
		size = new System.Drawing.Size(361, 25);
		buttonX_12.Size = size;
		this.ButtonX_1.TabIndex = 7;
		this.ButtonX_1.Text = "ป\u0e34ด Firewall เม\u0e37\u0e48อเข\u0e49าโปรแกรม (แนะนำ เคร\u0e37\u0e48องแม\u0e48)";
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.FocusCuesEnabled = false;
		this.ButtonX5.Font = new System.Drawing.Font("Tahoma", 14.05f, System.Drawing.FontStyle.Bold);
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX19 = this.ButtonX5;
		location = new System.Drawing.Point(592, 306);
		buttonX19.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX20 = this.ButtonX5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX20.Margin = margin;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX21 = this.ButtonX5;
		size = new System.Drawing.Size(160, 48);
		buttonX21.Size = size;
		this.ButtonX5.TabIndex = 4;
		this.ButtonX5.Text = "ป\u0e34ดโปรแกรม";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = iHOTEL2025.My.Resources.Resources.trash_empty;
		DevComponents.DotNetBar.ButtonX buttonX22 = this.ButtonX3;
		location = new System.Drawing.Point(11, 302);
		buttonX22.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX23 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX23.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX24 = this.ButtonX3;
		size = new System.Drawing.Size(134, 25);
		buttonX24.Size = size;
		this.ButtonX3.TabIndex = 1;
		this.ButtonX3.Text = "อ\u0e31บเดทโปรแกรม";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 14f, System.Drawing.FontStyle.Bold);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX25 = this.ButtonX1;
		location = new System.Drawing.Point(419, 306);
		buttonX25.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX26 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX26.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX27 = this.ButtonX1;
		size = new System.Drawing.Size(167, 48);
		buttonX27.Size = size;
		this.ButtonX1.TabIndex = 3;
		this.ButtonX1.Text = "เข\u0e49าโปรแกรม";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.FromArgb(194, 217, 247);
		this.BottomLeftCornerSize = 0;
		this.BottomRightCornerSize = 0;
		size = new System.Drawing.Size(764, 361);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.Name = "FormSelectDB";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "กร\u0e38ณา เล\u0e37อก เซ\u0e34ฟเวอร\u0e4c เพ\u0e37\u0e48อเข\u0e49าใช\u0e49งานโปรแกรม";
		this.TopLeftCornerSize = 0;
		this.TopRightCornerSize = 0;
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.PanelEx5.ResumeLayout(false);
		this.PanelEx2.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	public void load_chkFirewall()
	{
		if (!File.Exists(Module1.Path_Program + "autofirewall.txt"))
		{
			save_chkFirewall();
		}
		try
		{
			StreamReader streamReader = new StreamReader(Module1.Path_Program + "\\autofirewall.txt", Encoding.Default);
			string text = default(string);
			while (streamReader.Peek() != -1)
			{
				text = streamReader.ReadLine();
			}
			streamReader.Close();
			streamReader = null;
			if (Operators.CompareString(text.ToString(), "True", TextCompare: false) == 0)
			{
				CheckBox1.Checked = true;
			}
			else
			{
				CheckBox1.Checked = false;
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public void save_chkFirewall()
	{
		try
		{
			StreamWriter streamWriter = new StreamWriter(Module1.Path_Program + "autofirewall.txt");
			streamWriter.Write(CheckBox1.Checked);
			streamWriter.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void login_selectDB_Load(object sender, EventArgs e)
	{
		ISOK = false;
		Label_com.Text = Dns.GetHostName();
		Label1.Text = "Version " + Module1.ProgramVersion;
		Module1.loadDatabaseIndex();
		ReadDB1();
		Timer1.Enabled = true;
		Return_Select_DB = "";
		load_chkFirewall();
		checkService();
		if (Module1.AccessError.IndexOf("Unrecognized database format") != -1)
		{
			Module1.AccessError = "";
			MessageBox.Show("ฐานข\u0e49อม\u0e39ล Access ม\u0e35ป\u0e31ญหา ต\u0e49องทำการซ\u0e48อมแซมฐานข\u0e49อม\u0e39ล กร\u0e38ณาป\u0e34ดโปรแกรมท\u0e38กเคร\u0e37\u0e48องถ\u0e49าเช\u0e37\u0e48อมก\u0e31นอย\u0e39\u0e48 จากน\u0e31\u0e49นกด OK", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Exclamation);
		}
		else if (Module1.AccessError.IndexOf("ไม\u0e48ร\u0e39\u0e49จ\u0e31ก") != -1)
		{
			Module1.AccessError = "";
			MessageBox.Show("ฐานข\u0e49อม\u0e39ล Access ม\u0e35ป\u0e31ญหา ต\u0e49องทำการซ\u0e48อมแซมฐานข\u0e49อม\u0e39ล กร\u0e38ณาป\u0e34ดโปรแกรมท\u0e38กเคร\u0e37\u0e48องถ\u0e49าเช\u0e37\u0e48อมก\u0e31นอย\u0e39\u0e48 จากน\u0e31\u0e49นกด OK", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Exclamation);
		}
	}

	public void ReadDB1()
	{
		ListView1.Items.Clear();
		if (!File.Exists(Module1.Path_Program + "server.txt"))
		{
			StreamWriter streamWriter = File.CreateText(Module1.Path_Program + "server.txt");
			streamWriter.WriteLine();
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(Module1.Path_Program + "\\server.txt", Encoding.UTF8);
		checked
		{
			while (streamReader.Peek() != -1)
			{
				string text = streamReader.ReadLine();
				string[] array = text.Split('|');
				if (text.IndexOf("|") == -1)
				{
					ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("ACCESS");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(text);
					if (text.IndexOf("\\\\") != -1)
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("ANLOTTO");
					}
					else
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = "";
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว");
				}
				else if (text.IndexOf("|ACCESS") != -1)
				{
					ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("ACCESS");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[0]);
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[3]);
					}
					catch (Exception projectError)
					{
						ProjectData.SetProjectError(projectError);
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ProjectData.ClearProjectError();
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = "";
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = array[5];
					}
					catch (Exception projectError2)
					{
						ProjectData.SetProjectError(projectError2);
						ProjectData.ClearProjectError();
					}
					if (Operators.CompareString(ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text, "", TextCompare: false) == 0 && Operators.CompareString(ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text, "ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว", TextCompare: false) == 0)
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = "ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว";
						ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text = "";
					}
				}
				else if (text.IndexOf("|MYSQL") != -1)
				{
					ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("MYSQL");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[0]);
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[3]);
					}
					catch (Exception projectError3)
					{
						ProjectData.SetProjectError(projectError3);
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ProjectData.ClearProjectError();
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[1]);
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[2]);
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = array[2];
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Text = "".PadRight(ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag.ToString().Length, '*');
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = array[5];
					}
					catch (Exception projectError4)
					{
						ProjectData.SetProjectError(projectError4);
						ProjectData.ClearProjectError();
					}
				}
				else if (text.IndexOf("|CloudServer") != -1)
				{
					ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("CloudServer");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[0]);
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[3]);
					}
					catch (Exception projectError5)
					{
						ProjectData.SetProjectError(projectError5);
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ProjectData.ClearProjectError();
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[1]);
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[2]);
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = array[2];
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Text = "".PadRight(ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag.ToString().Length, '*');
					if (Operators.CompareString(ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text, "", TextCompare: false) == 0)
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text = "ANLOTTO";
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = array[5];
					}
					catch (Exception projectError6)
					{
						ProjectData.SetProjectError(projectError6);
						ProjectData.ClearProjectError();
					}
				}
				else
				{
					ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("MSSQL");
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[0]);
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[3]);
					}
					catch (Exception projectError7)
					{
						ProjectData.SetProjectError(projectError7);
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
						ProjectData.ClearProjectError();
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[1]);
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(array[2]);
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = array[2];
					ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Text = "".PadRight(ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag.ToString().Length, '*');
					if (Operators.CompareString(ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text, "", TextCompare: false) == 0)
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text = "ANLOTTO";
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
					try
					{
						ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = array[5];
					}
					catch (Exception projectError8)
					{
						ProjectData.SetProjectError(projectError8);
						ProjectData.ClearProjectError();
					}
				}
			}
			streamReader.Close();
			streamReader = null;
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUpdate.ShowDialog();
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		if (ListView1.Items.Count == 1)
		{
			ListView1.Items[0].Selected = true;
			ButtonX1.Focus();
		}
		else if (ListView1.Items.Count > 1)
		{
			try
			{
				ListView1.Items[Module1.data_index].Selected = true;
				ButtonX1.Focus();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกเซ\u0e34ฟเวอร\u0e4cฐานข\u0e49อม\u0e39ลในตาราง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			return;
		}
		ISOK = true;
		Module1.saveDatabaseIndex(Conversions.ToString(ListView1.SelectedItems[0].Index));
		Return_Select_DB = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(string.Concat(ListView1.SelectedItems[0].SubItems[2].Text + "|", ListView1.SelectedItems[0].SubItems[4].Text), "|"), ListView1.SelectedItems[0].SubItems[5].Tag), "|"), ListView1.SelectedItems[0].SubItems[3].Text), "|"), ListView1.SelectedItems[0].SubItems[1].Text));
		string text = Conversions.ToString(ListView1.SelectedItems[0].SubItems[5].Tag);
		if (text.IndexOf(":EG:") == -1)
		{
			object right = FormEN_DE.Encrypt1(text, "ruj5de4");
			text = Conversions.ToString(Operators.ConcatenateObject(":EG:", right));
		}
		if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[1].Text.ToUpper(), "ACCESS", TextCompare: false) == 0)
		{
			Module1.Settingstring = ListView1.SelectedItems[0].SubItems[2].Text + "|" + ListView1.SelectedItems[0].SubItems[4].Text + "|" + text + "|" + ListView1.SelectedItems[0].SubItems[3].Text + "|" + ListView1.SelectedItems[0].SubItems[1].Text;
		}
		else
		{
			Module1.Settingstring = Label_com.Text + "|" + ListView1.SelectedItems[0].SubItems[4].Text + "|" + text + "|" + ListView1.SelectedItems[0].SubItems[3].Text + "|" + ListView1.SelectedItems[0].SubItems[1].Text;
		}
		if (CheckBox1.Checked)
		{
			MyProject.Forms.frmMain1.OFF_FIREWALL_NEW();
		}
		save_chkFirewall();
		Close();
	}

	public void startService()
	{
		Cursor = Cursors.WaitCursor;
		try
		{
			ServiceController serviceController = new ServiceController("MSSQLSERVER");
			serviceController.Start();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		try
		{
			ServiceController serviceController2 = new ServiceController("mysql");
			serviceController2.Start();
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			ProjectData.ClearProjectError();
		}
		try
		{
			ServiceController serviceController3 = new ServiceController("mysql57");
			serviceController3.Start();
		}
		catch (Exception projectError3)
		{
			ProjectData.SetProjectError(projectError3);
			ProjectData.ClearProjectError();
		}
		Cursor = Cursors.Default;
	}

	public void checkService()
	{
		try
		{
			ServiceController serviceController = new ServiceController("MSSQLSERVER");
			if (serviceController.Status == ServiceControllerStatus.Running)
			{
				ButtonX_2.Text = "MSSQL : ทำงาน";
				ButtonX_2.Image = Resources._02__4_;
			}
			else
			{
				ButtonX_2.Text = "MSSQL : หย\u0e38ดทำงาน";
				ButtonX_2.Image = Resources._01__4_;
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ButtonX_2.Text = "MSSQL : ไม\u0e48ได\u0e49ต\u0e34ดต\u0e31\u0e49ง";
			ButtonX_2.Image = Resources.warning_triangle;
			ProjectData.ClearProjectError();
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		ButtonX2.Expanded = true;
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		ButtonX1_Click(null, null);
	}

	private void ListView1_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			ButtonX1_Click(null, null);
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		checked
		{
			try
			{
				if (ListView1.SelectedItems.Count == 0)
				{
					return;
				}
				int num = ListView1.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					ListView1.Items[num2].BackColor = Color.White;
					num2++;
				}
				ListView1.SelectedItems[0].BackColor = Color.LightGreen;
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกเซ\u0e34ฟเวอร\u0e4cฐานข\u0e49อม\u0e39ลในตาราง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else if (ListView1.SelectedItems.Count != 0 && MessageBox.Show("ค\u0e38ณต\u0e49องการลบหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			ListView1.SelectedItems[0].Remove();
			save_db();
		}
	}

	public void save_db()
	{
		if (File.Exists(Module1.Path_Program + "server.txt"))
		{
			File.Delete(Module1.Path_Program + "server.txt");
		}
		StreamWriter streamWriter = new StreamWriter(Module1.Path_Program + "server.txt", append: true, Encoding.UTF8);
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ListView1.Items[num2].SubItems[0].Text = Conversions.ToString(num2 + 1);
				streamWriter.WriteLine(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(string.Concat(ListView1.Items[num2].SubItems[2].Text + "|", ListView1.Items[num2].SubItems[4].Text), "|"), ListView1.Items[num2].SubItems[5].Tag), "|"), ListView1.Items[num2].SubItems[3].Text), "|"), ListView1.Items[num2].SubItems[1].Text), "|"), ListView1.Items[num2].SubItems[6].Text));
				num2++;
			}
			streamWriter.Close();
		}
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		ISOK = false;
		Close();
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกเซ\u0e34ฟเวอร\u0e4cฐานข\u0e49อม\u0e39ลในตาราง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else
		{
			if (ListView1.SelectedItems.Count == 0)
			{
				return;
			}
			if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[1].Text, "MSSQL", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmAddEditServer.ComboBox1.SelectedIndex = 0;
			}
			else
			{
				MyProject.Forms.FrmAddEditServer.ComboBox1.SelectedIndex = 2;
			}
			MyProject.Forms.FrmAddEditServer.Tpath.Text = ListView1.SelectedItems[0].SubItems[2].Text;
			if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[3].Text, "ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว", TextCompare: false) == 0)
			{
				MyProject.Forms.FrmAddEditServer.Tdbname.Text = "";
			}
			else
			{
				MyProject.Forms.FrmAddEditServer.Tdbname.Text = ListView1.SelectedItems[0].SubItems[3].Text;
			}
			MyProject.Forms.FrmAddEditServer.TextBox_user.Text = ListView1.SelectedItems[0].SubItems[4].Text;
			MyProject.Forms.FrmAddEditServer.TextBox_password.Text = Conversions.ToString(ListView1.SelectedItems[0].SubItems[5].Tag);
			MyProject.Forms.FrmAddEditServer.Tnote.Text = ListView1.SelectedItems[0].SubItems[6].Text;
			MyProject.Forms.FrmAddEditServer.ShowDialog();
			if (!MyProject.Forms.FrmAddEditServer.ISOK)
			{
				return;
			}
			if ((Operators.CompareString(MyProject.Forms.FrmAddEditServer.ComboBox1.Text, "ACCESS", TextCompare: false) == 0) | (Operators.CompareString(MyProject.Forms.FrmAddEditServer.Tpath.Text, "kphotel.accdb", TextCompare: false) == 0))
			{
				ListView1.SelectedItems[0].SubItems[1].Text = MyProject.Forms.FrmAddEditServer.ComboBox1.Text;
				ListView1.SelectedItems[0].SubItems[2].Text = MyProject.Forms.FrmAddEditServer.Tpath.Text;
				if (MyProject.Forms.FrmAddEditServer.Tpath.Text.IndexOf("\\\\") != -1)
				{
					ListView1.SelectedItems[0].SubItems[3].Text = "ANLOTTO";
				}
				else
				{
					ListView1.SelectedItems[0].SubItems[3].Text = "ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว";
				}
				ListView1.SelectedItems[0].SubItems[4].Text = "";
				ListView1.SelectedItems[0].SubItems[5].Text = "";
				ListView1.SelectedItems[0].SubItems[5].Tag = "";
				ListView1.SelectedItems[0].SubItems[5].Text = "";
				ListView1.SelectedItems[0].SubItems[6].Text = MyProject.Forms.FrmAddEditServer.Tnote.Text;
			}
			else
			{
				ListView1.SelectedItems[0].SubItems[1].Text = MyProject.Forms.FrmAddEditServer.ComboBox1.Text;
				ListView1.SelectedItems[0].SubItems[2].Text = MyProject.Forms.FrmAddEditServer.Tpath.Text;
				ListView1.SelectedItems[0].SubItems[3].Text = MyProject.Forms.FrmAddEditServer.Tdbname.Text;
				ListView1.SelectedItems[0].SubItems[4].Text = MyProject.Forms.FrmAddEditServer.TextBox_user.Text;
				ListView1.SelectedItems[0].SubItems[5].Text = MyProject.Forms.FrmAddEditServer.TextBox_password.Text;
				ListView1.SelectedItems[0].SubItems[5].Tag = MyProject.Forms.FrmAddEditServer.TextBox_password.Text;
				ListView1.SelectedItems[0].SubItems[5].Text = "".PadRight(ListView1.SelectedItems[0].SubItems[5].Tag.ToString().Length, '*');
				ListView1.SelectedItems[0].SubItems[6].Text = MyProject.Forms.FrmAddEditServer.Tnote.Text;
			}
			save_db();
		}
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		checked
		{
			try
			{
				string text = ListView1.SelectedItems[0].SubItems[1].Text;
				string text2 = ListView1.SelectedItems[0].SubItems[2].Text;
				string text3 = ListView1.SelectedItems[0].SubItems[3].Text;
				string text4 = ListView1.SelectedItems[0].SubItems[4].Text;
				string tag = Conversions.ToString(ListView1.SelectedItems[0].SubItems[5].Tag);
				string text5 = ListView1.SelectedItems[0].SubItems[6].Text;
				int index = ListView1.SelectedItems[0].Index;
				if (index != ListView1.Items.Count - 1)
				{
					string text6 = ListView1.Items[index + 1].SubItems[1].Text;
					string text7 = ListView1.Items[index + 1].SubItems[2].Text;
					string text8 = ListView1.Items[index + 1].SubItems[3].Text;
					string text9 = ListView1.Items[index + 1].SubItems[4].Text;
					string tag2 = Conversions.ToString(ListView1.Items[index + 1].SubItems[5].Tag);
					string text10 = ListView1.Items[index + 1].SubItems[6].Text;
					ListView1.SelectedItems[0].SubItems[1].Text = text6;
					ListView1.SelectedItems[0].SubItems[2].Text = text7;
					ListView1.SelectedItems[0].SubItems[3].Text = text8;
					ListView1.SelectedItems[0].SubItems[4].Text = text9;
					ListView1.SelectedItems[0].SubItems[5].Tag = tag2;
					ListView1.SelectedItems[0].SubItems[5].Text = "".PadRight(ListView1.SelectedItems[0].SubItems[5].Tag.ToString().Length, '*');
					ListView1.SelectedItems[0].SubItems[6].Text = text10;
					ListView1.Items[index + 1].SubItems[1].Text = text;
					ListView1.Items[index + 1].SubItems[2].Text = text2;
					ListView1.Items[index + 1].SubItems[3].Text = text3;
					ListView1.Items[index + 1].SubItems[4].Text = text4;
					ListView1.Items[index + 1].SubItems[5].Tag = tag;
					ListView1.Items[index + 1].SubItems[5].Text = "".PadRight(ListView1.Items[index + 1].SubItems[5].Tag.ToString().Length, '*');
					ListView1.Items[index + 1].SubItems[6].Text = text5;
					ListView1.Items[index + 1].Selected = true;
					ListView1.Focus();
					save_db();
				}
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void ButtonX8_Click(object sender, EventArgs e)
	{
		checked
		{
			try
			{
				string text = ListView1.SelectedItems[0].SubItems[1].Text;
				string text2 = ListView1.SelectedItems[0].SubItems[2].Text;
				string text3 = ListView1.SelectedItems[0].SubItems[3].Text;
				string text4 = ListView1.SelectedItems[0].SubItems[4].Text;
				string tag = Conversions.ToString(ListView1.SelectedItems[0].SubItems[5].Tag);
				string text5 = ListView1.SelectedItems[0].SubItems[6].Text;
				int index = ListView1.SelectedItems[0].Index;
				if (index != 0)
				{
					string text6 = ListView1.Items[index - 1].SubItems[1].Text;
					string text7 = ListView1.Items[index - 1].SubItems[2].Text;
					string text8 = ListView1.Items[index - 1].SubItems[3].Text;
					string text9 = ListView1.Items[index - 1].SubItems[4].Text;
					string tag2 = Conversions.ToString(ListView1.Items[index - 1].SubItems[5].Tag);
					string text10 = ListView1.Items[index - 1].SubItems[6].Text;
					ListView1.SelectedItems[0].SubItems[1].Text = text6;
					ListView1.SelectedItems[0].SubItems[2].Text = text7;
					ListView1.SelectedItems[0].SubItems[3].Text = text8;
					ListView1.SelectedItems[0].SubItems[4].Text = text9;
					ListView1.SelectedItems[0].SubItems[5].Tag = tag2;
					ListView1.SelectedItems[0].SubItems[5].Text = "".PadRight(ListView1.SelectedItems[0].SubItems[5].Tag.ToString().Length, '*');
					ListView1.SelectedItems[0].SubItems[6].Text = text10;
					ListView1.Items[index - 1].SubItems[1].Text = text;
					ListView1.Items[index - 1].SubItems[2].Text = text2;
					ListView1.Items[index - 1].SubItems[3].Text = text3;
					ListView1.Items[index - 1].SubItems[4].Text = text4;
					ListView1.Items[index - 1].SubItems[5].Tag = tag;
					ListView1.Items[index - 1].SubItems[5].Text = "".PadRight(ListView1.Items[index - 1].SubItems[5].Tag.ToString().Length, '*');
					ListView1.Items[index - 1].SubItems[6].Text = text5;
					ListView1.Items[index - 1].Selected = true;
					ListView1.Focus();
					save_db();
				}
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		Process process = null;
		ProcessStartInfo processStartInfo = new ProcessStartInfo();
		processStartInfo.FileName = Module1.Path_Program + "TeamViewerQS2.exe";
		if (Environment.OSVersion.Version.Major >= 6)
		{
			processStartInfo.Verb = "runas";
		}
		processStartInfo.Arguments = "";
		processStartInfo.WindowStyle = ProcessWindowStyle.Normal;
		processStartInfo.UseShellExecute = true;
		Process.Start(processStartInfo)?.Dispose();
	}

	private void ButtonItem4_Click(object sender, EventArgs e)
	{
	}

	private void ButtonX10_Click(object sender, EventArgs e)
	{
		checkService();
	}

	private void ButtonX11_Click(object sender, EventArgs e)
	{
		Process process = new Process();
		ProcessStartInfo processStartInfo = new ProcessStartInfo();
		processStartInfo.FileName = "control firewall.cpl";
		process.StartInfo = processStartInfo;
		process.Start();
	}

	private void ButtonX11_Click_1(object sender, EventArgs e)
	{
		if (CheckBox1.Checked)
		{
			CheckBox1.Checked = false;
		}
		else
		{
			CheckBox1.Checked = true;
		}
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
		ButtonX_1.Checked = CheckBox1.Checked;
	}

	private void ButtonMSSQL_Click(object sender, EventArgs e)
	{
		startService();
		checkService();
	}

	private void ButtonMYSQL_Click(object sender, EventArgs e)
	{
		startService();
		checkService();
	}

	private void Label2_Click(object sender, EventArgs e)
	{
	}

	private void PanelEx1_Click(object sender, EventArgs e)
	{
	}

	private void PanelEx5_Click(object sender, EventArgs e)
	{
	}

	private void LabelX2_Click(object sender, EventArgs e)
	{
	}

	private void ButtonX13_Click(object sender, EventArgs e)
	{
		TimerwaitServer.Enabled = true;
		TimerwaitServer2.Enabled = false;
	}

	private void ButtonItem1_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmAddEditServer.clear();
		MyProject.Forms.FrmAddEditServer.ShowDialog();
		if (!MyProject.Forms.FrmAddEditServer.ISOK)
		{
			return;
		}
		checked
		{
			if ((Operators.CompareString(MyProject.Forms.FrmAddEditServer.ComboBox1.Text, "ACCESS", TextCompare: false) == 0) | (Operators.CompareString(MyProject.Forms.FrmAddEditServer.Tpath.Text, "kphotel.accdb", TextCompare: false) == 0))
			{
				if ((Operators.CompareString(MyProject.Forms.FrmAddEditServer.ComboBox1.Text, "ACCESS", TextCompare: false) == 0) & (Operators.CompareString(MyProject.Forms.FrmAddEditServer.Tpath.Text, "kphotel.accdb", TextCompare: false) == 0))
				{
					int num = ListView1.Items.Count - 1;
					int num2 = 0;
					while (true)
					{
						int num3 = num2;
						int num4 = num;
						if (num3 <= num4)
						{
							if (!((Operators.CompareString(ListView1.Items[num2].SubItems[1].Text, "ACCESS", TextCompare: false) == 0) & (Operators.CompareString(ListView1.Items[num2].SubItems[2].Text, "kphotel.accdb", TextCompare: false) == 0)))
							{
								num2++;
								continue;
							}
							MessageBox.Show("ม\u0e35ฐานข\u0e49อม\u0e39ลในเคร\u0e37\u0e48องอย\u0e39\u0e48แล\u0e49ว อย\u0e39\u0e48ท\u0e35\u0e48 ข\u0e49อ " + Conversions.ToString(num2 + 1), "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK);
							return;
						}
						break;
					}
				}
				ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.ComboBox1.Text);
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.Tpath.Text);
				if (MyProject.Forms.FrmAddEditServer.Tpath.Text.IndexOf("\\\\") != -1)
				{
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("ANLOTTO");
				}
				else
				{
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว");
				}
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
				ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = "";
				ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Text = "";
				ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = MyProject.Forms.FrmAddEditServer.Tnote.Text;
				if ((Operators.CompareString(ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text, "", TextCompare: false) == 0) & (Operators.CompareString(ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text, "ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว", TextCompare: false) == 0))
				{
					ListView1.Items[ListView1.Items.Count - 1].SubItems[3].Text = "";
					ListView1.Items[ListView1.Items.Count - 1].SubItems[6].Text = "ใช\u0e49เคร\u0e37\u0e48องเด\u0e35ยว";
				}
			}
			else
			{
				ListView1.Items.Add(Conversions.ToString(ListView1.Items.Count + 1));
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.ComboBox1.Text);
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.Tpath.Text);
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.Tdbname.Text);
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.TextBox_user.Text);
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.TextBox_password.Text);
				ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag = MyProject.Forms.FrmAddEditServer.TextBox_password.Text;
				ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Text = "".PadRight(ListView1.Items[ListView1.Items.Count - 1].SubItems[5].Tag.ToString().Length, '*');
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(MyProject.Forms.FrmAddEditServer.Tnote.Text);
			}
			MyProject.Forms.FrmAddEditServer.clear();
			save_db();
			Timer1.Enabled = true;
		}
	}

	private void ButtonItem2_Click(object sender, EventArgs e)
	{
		TimerwaitServer.Enabled = false;
		TimerwaitServer2.Enabled = true;
		Timer1.Enabled = true;
	}

	private void TimerwaitServer2_Tick(object sender, EventArgs e)
	{
	}

	private void FormSelectDB_Resize(object sender, EventArgs e)
	{
	}

	private void ButtonX12_Click(object sender, EventArgs e)
	{
	}
}
