using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using C1.Win.C1FlexGrid;
using C1.Win.C1FlexGrid.Util.BaseControls;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormFolio : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("LabelX4")]
	private LabelX _LabelX4;

	[AccessedThroughProperty("TextBoxX2")]
	private TextBoxX _TextBoxX2;

	[AccessedThroughProperty("TextBoxX1")]
	private TextBoxX _TextBoxX1;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("TselectRoom")]
	private ComboBox _TselectRoom;

	[AccessedThroughProperty("Label55")]
	private Label _Label55;

	[AccessedThroughProperty("TextBoxX3")]
	private TextBoxX _TextBoxX3;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TextBoxX4")]
	private TextBoxX _TextBoxX4;

	[AccessedThroughProperty("LabelX5")]
	private LabelX _LabelX5;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("TextBoxX5")]
	private TextBoxX _TextBoxX5;

	[AccessedThroughProperty("LabelX6")]
	private LabelX _LabelX6;

	[AccessedThroughProperty("Grid1")]
	private C1FlexGrid _Grid1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	public string CIN_ID;

	private DataSet A;

	private bool ISEDIT;

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

	internal virtual LabelX LabelX4
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX4 = value;
		}
	}

	internal virtual TextBoxX TextBoxX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = EEE;
			if (_TextBoxX2 != null)
			{
				_TextBoxX2.TextChanged -= value2;
			}
			_TextBoxX2 = value;
			if (_TextBoxX2 != null)
			{
				_TextBoxX2.TextChanged += value2;
			}
		}
	}

	internal virtual TextBoxX TextBoxX_1
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
			EventHandler value2 = EEE;
			if (_TextBoxX1 != null)
			{
				_TextBoxX1.TextChanged -= value2;
			}
			_TextBoxX1 = value;
			if (_TextBoxX1 != null)
			{
				_TextBoxX1.TextChanged += value2;
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

	internal virtual ComboBox TselectRoom
	{
		[DebuggerNonUserCode]
		get
		{
			return _TselectRoom;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TselectRoom = value;
		}
	}

	internal virtual Label Label55
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label55;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label55 = value;
		}
	}

	internal virtual TextBoxX TextBoxX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TextBoxX3_TextChanged;
			if (_TextBoxX3 != null)
			{
				_TextBoxX3.TextChanged -= value2;
			}
			_TextBoxX3 = value;
			if (_TextBoxX3 != null)
			{
				_TextBoxX3.TextChanged += value2;
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

	internal virtual TextBoxX TextBoxX_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxX4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = EEE;
			if (_TextBoxX4 != null)
			{
				_TextBoxX4.TextChanged -= value2;
			}
			_TextBoxX4 = value;
			if (_TextBoxX4 != null)
			{
				_TextBoxX4.TextChanged += value2;
			}
		}
	}

	internal virtual LabelX LabelX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX5 = value;
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

	internal virtual TextBoxX TextBoxX_4
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBoxX5 = value;
		}
	}

	internal virtual LabelX LabelX6
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX6 = value;
		}
	}

	internal virtual C1FlexGrid Grid1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Grid1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			RowColEventHandler value2 = Grid1_AfterDeleteRow;
			RowColEventHandler value3 = Grid1_AfterAddRow;
			EventHandler value4 = Grid1_Click;
			KeyEventHandler value5 = Grid1_KeyDown;
			RowColEventHandler value6 = Grid1_AfterEdit;
			if (_Grid1 != null)
			{
				_Grid1.AfterDeleteRow -= value2;
				_Grid1.AfterAddRow -= value3;
				_Grid1.Click -= value4;
				_Grid1.KeyDown -= value5;
				_Grid1.AfterEdit -= value6;
			}
			_Grid1 = value;
			if (_Grid1 != null)
			{
				_Grid1.AfterDeleteRow += value2;
				_Grid1.AfterAddRow += value3;
				_Grid1.Click += value4;
				_Grid1.KeyDown += value5;
				_Grid1.AfterEdit += value6;
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
			_Label2 = value;
		}
	}

	[DebuggerNonUserCode]
	static FormFolio()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormFolio()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormFolio_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		A = null;
		ISEDIT = false;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormFolio));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.TextBoxX_4 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX6 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.TextBoxX_3 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX5 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label1 = new System.Windows.Forms.Label();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.TselectRoom = new System.Windows.Forms.ComboBox();
		this.Label55 = new System.Windows.Forms.Label();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Grid1 = new C1.Win.C1FlexGrid.C1FlexGrid();
		this.LabelX4 = new DevComponents.DotNetBar.LabelX();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.TextBoxX_1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.Label2 = new System.Windows.Forms.Label();
		this.PanelEx1.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.Grid1).BeginInit();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.TextBoxX_4);
		this.PanelEx1.Controls.Add(this.LabelX6);
		this.PanelEx1.Controls.Add(this.ButtonX4);
		this.PanelEx1.Controls.Add(this.ButtonX3);
		this.PanelEx1.Controls.Add(this.TextBoxX_3);
		this.PanelEx1.Controls.Add(this.LabelX5);
		this.PanelEx1.Controls.Add(this.TextBoxX_2);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.TselectRoom);
		this.PanelEx1.Controls.Add(this.Label55);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.Grid1);
		this.PanelEx1.Controls.Add(this.LabelX4);
		this.PanelEx1.Controls.Add(this.TextBoxX_0);
		this.PanelEx1.Controls.Add(this.TextBoxX_1);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(757, 619);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.TextBoxX_4.Border.Class = "TextBoxBorder";
		this.TextBoxX_4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_4;
		location = new System.Drawing.Point(90, 581);
		textBoxX_.Location = location;
		this.TextBoxX_4.MaxLength = 255;
		this.TextBoxX_4.Name = "TextBoxX5";
		this.TextBoxX_4.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_4;
		size = new System.Drawing.Size(143, 27);
		textBoxX_2.Size = size;
		this.TextBoxX_4.TabIndex = 98;
		this.LabelX6.BackgroundStyle.Class = "";
		this.LabelX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX = this.LabelX6;
		location = new System.Drawing.Point(13, 577);
		labelX.Location = location;
		this.LabelX6.Name = "LabelX6";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX6;
		size = new System.Drawing.Size(71, 31);
		labelX2.Size = size;
		this.LabelX6.TabIndex = 97;
		this.LabelX6.Text = "ราคารวม";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX4;
		location = new System.Drawing.Point(649, 581);
		buttonX.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX4;
		size = new System.Drawing.Size(96, 27);
		buttonX2.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 96;
		this.ButtonX4.Text = "พ\u0e34มพ\u0e4c";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX3;
		location = new System.Drawing.Point(547, 581);
		buttonX3.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX3;
		size = new System.Drawing.Size(96, 27);
		buttonX4.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 95;
		this.ButtonX3.Text = "บ\u0e31นท\u0e36ก";
		this.TextBoxX_3.Border.Class = "TextBoxBorder";
		this.TextBoxX_3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_3 = this.TextBoxX_3;
		location = new System.Drawing.Point(109, 121);
		textBoxX_3.Location = location;
		this.TextBoxX_3.MaxLength = 255;
		this.TextBoxX_3.Name = "TextBoxX4";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_4 = this.TextBoxX_3;
		size = new System.Drawing.Size(595, 27);
		textBoxX_4.Size = size;
		this.TextBoxX_3.TabIndex = 94;
		this.LabelX5.BackgroundStyle.Class = "";
		this.LabelX5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX5;
		location = new System.Drawing.Point(12, 122);
		labelX3.Location = location;
		this.LabelX5.Name = "LabelX5";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX5;
		size = new System.Drawing.Size(106, 31);
		labelX4.Size = size;
		this.LabelX5.TabIndex = 93;
		this.LabelX5.Text = "เข\u0e49าพ\u0e31ก";
		this.TextBoxX_2.Border.Class = "TextBoxBorder";
		this.TextBoxX_2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_5 = this.TextBoxX_2;
		location = new System.Drawing.Point(280, 192);
		textBoxX_5.Location = location;
		this.TextBoxX_2.MaxLength = 255;
		this.TextBoxX_2.Name = "TextBoxX3";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_6 = this.TextBoxX_2;
		size = new System.Drawing.Size(38, 27);
		textBoxX_6.Size = size;
		this.TextBoxX_2.TabIndex = 92;
		this.TextBoxX_2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(204, 196);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(74, 19);
		label2.Size = size;
		this.Label1.TabIndex = 91;
		this.Label1.Text = "จำนวนคน";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX2;
		location = new System.Drawing.Point(323, 192);
		buttonX5.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX2;
		size = new System.Drawing.Size(161, 27);
		buttonX6.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 89;
		this.ButtonX2.Text = "เพ\u0e34\u0e48มลงตารางท\u0e35\u0e48ละห\u0e49อง";
		this.TselectRoom.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.TselectRoom.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.TselectRoom.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tselectRoom = this.TselectRoom;
		location = new System.Drawing.Point(77, 192);
		tselectRoom.Location = location;
		System.Windows.Forms.ComboBox tselectRoom2 = this.TselectRoom;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tselectRoom2.Margin = margin;
		this.TselectRoom.Name = "TselectRoom";
		System.Windows.Forms.ComboBox tselectRoom3 = this.TselectRoom;
		size = new System.Drawing.Size(121, 27);
		tselectRoom3.Size = size;
		this.TselectRoom.TabIndex = 88;
		this.Label55.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label55.AutoSize = true;
		this.Label55.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label3 = this.Label55;
		location = new System.Drawing.Point(9, 195);
		label3.Location = location;
		this.Label55.Name = "Label55";
		System.Windows.Forms.Label label4 = this.Label55;
		size = new System.Drawing.Size(62, 19);
		label4.Size = size;
		this.Label55.TabIndex = 87;
		this.Label55.Text = "เลขห\u0e49อง";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX1;
		location = new System.Drawing.Point(489, 192);
		buttonX7.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX1;
		size = new System.Drawing.Size(216, 27);
		buttonX8.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 78;
		this.ButtonX1.Text = "เพ\u0e34\u0e48มรายช\u0e37\u0e48อห\u0e49องละ 2 คนท\u0e38กห\u0e49อง";
		this.Grid1.AllowDelete = true;
		this.Grid1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Grid1.BorderStyle = C1.Win.C1FlexGrid.Util.BaseControls.BorderStyleEnum.FixedSingle;
		this.Grid1.ColumnInfo = resources.GetString("Grid1.ColumnInfo");
		C1.Win.C1FlexGrid.C1FlexGrid grid = this.Grid1;
		location = new System.Drawing.Point(12, 225);
		grid.Location = location;
		this.Grid1.Name = "Grid1";
		this.Grid1.Rows.Count = 1;
		this.Grid1.Rows.DefaultSize = 22;
		C1.Win.C1FlexGrid.C1FlexGrid grid2 = this.Grid1;
		size = new System.Drawing.Size(733, 350);
		grid2.Size = size;
		this.Grid1.StyleInfo = resources.GetString("Grid1.StyleInfo");
		this.Grid1.TabIndex = 77;
		this.Grid1.VisualStyle = C1.Win.C1FlexGrid.VisualStyle.Office2007Blue;
		this.LabelX4.BackgroundStyle.Class = "";
		this.LabelX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX4;
		location = new System.Drawing.Point(12, 158);
		labelX5.Location = location;
		this.LabelX4.Name = "LabelX4";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX4;
		size = new System.Drawing.Size(63, 27);
		labelX6.Size = size;
		this.LabelX4.TabIndex = 5;
		this.LabelX4.Text = "รายช\u0e37\u0e48อ";
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		this.TextBoxX_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_7 = this.TextBoxX_0;
		location = new System.Drawing.Point(109, 86);
		textBoxX_7.Location = location;
		this.TextBoxX_0.MaxLength = 255;
		this.TextBoxX_0.Name = "TextBoxX2";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_8 = this.TextBoxX_0;
		size = new System.Drawing.Size(595, 27);
		textBoxX_8.Size = size;
		this.TextBoxX_0.TabIndex = 4;
		this.TextBoxX_1.Border.Class = "TextBoxBorder";
		this.TextBoxX_1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_9 = this.TextBoxX_1;
		location = new System.Drawing.Point(109, 53);
		textBoxX_9.Location = location;
		this.TextBoxX_1.MaxLength = 255;
		this.TextBoxX_1.Name = "TextBoxX1";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_10 = this.TextBoxX_1;
		size = new System.Drawing.Size(595, 27);
		textBoxX_10.Size = size;
		this.TextBoxX_1.TabIndex = 3;
		this.LabelX3.BackgroundStyle.Class = "";
		this.LabelX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX7 = this.LabelX3;
		location = new System.Drawing.Point(12, 87);
		labelX7.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX8 = this.LabelX3;
		size = new System.Drawing.Size(106, 31);
		labelX8.Size = size;
		this.LabelX3.TabIndex = 2;
		this.LabelX3.Text = "ช\u0e37\u0e48อหน\u0e48วยงาน";
		this.LabelX2.BackgroundStyle.Class = "";
		this.LabelX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX9 = this.LabelX2;
		location = new System.Drawing.Point(12, 53);
		labelX9.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX10 = this.LabelX2;
		size = new System.Drawing.Size(106, 31);
		labelX10.Size = size;
		this.LabelX2.TabIndex = 1;
		this.LabelX2.Text = "ช\u0e37\u0e48อโครงการ";
		this.LabelX1.BackgroundStyle.Class = "";
		this.LabelX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX11 = this.LabelX1;
		location = new System.Drawing.Point(12, 12);
		labelX11.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX12 = this.LabelX1;
		size = new System.Drawing.Size(692, 31);
		labelX12.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ใบ FOLIO";
		this.LabelX1.TextAlignment = System.Drawing.StringAlignment.Center;
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label5 = this.Label2;
		location = new System.Drawing.Point(78, 172);
		label5.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(240, 16);
		label6.Size = size;
		this.Label2.TabIndex = 99;
		this.Label2.Text = "*ถ\u0e49าเป\u0e47นเลขห\u0e49องอ\u0e37\u0e48นๆพ\u0e34มพ\u0e4cลงไปในช\u0e48องได\u0e49เลย";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(757, 619);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormFolio";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ใบ Folio";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.Grid1).EndInit();
		this.ResumeLayout(false);
	}

	private void FormFolio_Load(object sender, EventArgs e)
	{
		ISEDIT = false;
		LabelX1.Text = "ใบ FOLIO เลขท\u0e35\u0e48 " + CIN_ID;
		TextBoxX_2.Text = Conversions.ToString(2);
		TextBoxX_4.Text = Conversions.ToString(0);
		TselectRoom.Items.Clear();
		A = Module1.connect("select * from View_CheckIn_Ds where cin_no='" + CIN_ID + "' order by Cin_room_no");
		checked
		{
			int num = A.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				TextBoxX_3.Text = "เข\u0e49าพ\u0e31ก ว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_in"]), "dd/MM/yyyy") + " ถ\u0e36งว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_out"]), "dd/MM/yyyy");
				bool flag = false;
				int num5 = TselectRoom.Items.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(A.Tables[0].Rows[num2]["Cin_room_no"], TselectRoom.Items[num6].ToString(), TextCompare: false))
					{
						flag = true;
					}
					num6++;
				}
				if (!flag)
				{
					TselectRoom.Items.Add(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_no"]));
				}
				num2++;
			}
			loaddata();
		}
	}

	public void loaddata()
	{
		DataSet dataSet = Module1.connect("select * from TB_FOLIO where no='" + CIN_ID + "' order by id");
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
				TextBoxX_1.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["CIN_NAME1"]);
				TextBoxX_0.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["CIN_NAME2"]);
				TextBoxX_3.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["CIN_NAME3"]);
				Grid1.Rows.Add(1);
				Grid1[Grid1.Rows.Count - 1, 2] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_ROOM"]);
				Grid1[Grid1.Rows.Count - 1, 3] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_NAME"]);
				Grid1[Grid1.Rows.Count - 1, 4] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_IN"]);
				Grid1[Grid1.Rows.Count - 1, 5] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_out"]);
				Grid1[Grid1.Rows.Count - 1, 6] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_NIGHT"]);
				Grid1[Grid1.Rows.Count - 1, 7] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_PRICE"]);
				Grid1[Grid1.Rows.Count - 1, 8] = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["F_PRICE_TOTAL"]);
				num2++;
			}
			sum();
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_2.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_2.Focus();
			MessageBox.Show("กร\u0e38ณากรอกจำนวนคนให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		if (!Versioned.IsNumeric(TextBoxX_2.Text))
		{
			TextBoxX_2.Focus();
			MessageBox.Show("กร\u0e38ณากรอกจำนวนคนให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		if (Operators.CompareString(TselectRoom.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกเลขห\u0e49อง");
			return;
		}
		int num = Conversions.ToInteger(TextBoxX_2.Text);
		int num2 = 1;
		while (true)
		{
			int num3 = num2;
			int num4 = num;
			if (num3 > num4)
			{
				break;
			}
			addd();
			num2 = checked(num2 + 1);
		}
		sum();
	}

	public void addd()
	{
		ISEDIT = true;
		bool flag = false;
		checked
		{
			int num = A.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				int num5 = 0;
				bool flag2 = false;
				int num6 = Grid1.Rows.Count - 1;
				int num7 = 1;
				while (true)
				{
					int num8 = num7;
					num4 = num6;
					if (num8 > num4)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid1[num7, 2]), TselectRoom.Text, TextCompare: false) == 0)
					{
						flag2 = true;
						num5 = num7;
					}
					num7++;
				}
				if (Operators.ConditionalCompareObjectEqual(A.Tables[0].Rows[num2]["Cin_room_no"], TselectRoom.Text, TextCompare: false))
				{
					flag = true;
					if (flag2)
					{
						Grid1.Rows.Insert(num5 + 1);
						Grid1[num5 + 1, 2] = TselectRoom.Text;
						Grid1[num5 + 1, 4] = "";
						Grid1[num5 + 1, 5] = "";
						Grid1[num5 + 1, 6] = "";
						Grid1[num5 + 1, 7] = "";
						Grid1[num5 + 1, 8] = "";
					}
					else
					{
						Grid1.Rows.Add(1);
						Grid1[Grid1.Rows.Count - 1, 2] = TselectRoom.Text;
						Grid1[Grid1.Rows.Count - 1, 4] = Strings.Format(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_in"]), "dd/MM/yyyy");
						Grid1[Grid1.Rows.Count - 1, 5] = Strings.Format(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_out"]), "dd/MM/yyyy");
						Grid1[Grid1.Rows.Count - 1, 6] = RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_night"]);
						Grid1[Grid1.Rows.Count - 1, 7] = RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_price"]);
						Grid1[Grid1.Rows.Count - 1, 8] = RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_priceTotal"]);
					}
				}
				num2++;
			}
			if (!flag)
			{
				Grid1.Rows.Add(1);
				Grid1[Grid1.Rows.Count - 1, 2] = TselectRoom.Text;
				Grid1[Grid1.Rows.Count - 1, 4] = "";
				Grid1[Grid1.Rows.Count - 1, 5] = "";
				Grid1[Grid1.Rows.Count - 1, 6] = "";
				Grid1[Grid1.Rows.Count - 1, 7] = "";
				Grid1[Grid1.Rows.Count - 1, 8] = "";
			}
		}
	}

	public void addd2(string r_no)
	{
		ISEDIT = true;
		checked
		{
			int num = A.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				int num5 = 0;
				bool flag = false;
				int num6 = Grid1.Rows.Count - 1;
				int num7 = 1;
				while (true)
				{
					int num8 = num7;
					num4 = num6;
					if (num8 > num4)
					{
						break;
					}
					if (Operators.CompareString(Conversions.ToString(Grid1[num7, 2]), r_no, TextCompare: false) == 0)
					{
						flag = true;
						num5 = num7;
					}
					num7++;
				}
				if (Operators.ConditionalCompareObjectEqual(A.Tables[0].Rows[num2]["Cin_room_no"], r_no, TextCompare: false))
				{
					if (flag)
					{
						Grid1.Rows.Insert(num5 + 1);
						Grid1[num5 + 1, 2] = r_no;
						Grid1[num5 + 1, 4] = "";
						Grid1[num5 + 1, 5] = "";
						Grid1[num5 + 1, 6] = "";
						Grid1[num5 + 1, 7] = "";
						Grid1[num5 + 1, 8] = "";
					}
					else
					{
						Grid1.Rows.Add(1);
						Grid1[Grid1.Rows.Count - 1, 2] = r_no;
						Grid1[Grid1.Rows.Count - 1, 4] = Strings.Format(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_in"]), "dd/MM/yyyy");
						Grid1[Grid1.Rows.Count - 1, 5] = Strings.Format(RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_out"]), "dd/MM/yyyy");
						Grid1[Grid1.Rows.Count - 1, 6] = RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_night"]);
						Grid1[Grid1.Rows.Count - 1, 7] = RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_price"]);
						Grid1[Grid1.Rows.Count - 1, 8] = RuntimeHelpers.GetObjectValue(A.Tables[0].Rows[num2]["Cin_room_priceTotal"]);
					}
				}
				num2++;
			}
		}
	}

	private void Grid1_AfterAddRow(object sender, RowColEventArgs e)
	{
		ISEDIT = true;
	}

	private void Grid1_AfterDeleteRow(object sender, RowColEventArgs e)
	{
		ISEDIT = true;
		sum();
	}

	public void sum()
	{
		TextBoxX_4.Text = Conversions.ToString(0);
		checked
		{
			int num = Grid1.Rows.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					Grid1[num2, 1] = num2;
					if (Operators.CompareString(Conversions.ToString(Grid1[num2, 8]), "", TextCompare: false) != 0)
					{
						TextBoxX_4.Text = Strings.Format(decimal.Add(Conversions.ToDecimal(TextBoxX_4.Text), new decimal(Conversions.ToInteger(Grid1[num2, 8]))), "#,##0.00");
					}
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void TextBoxX3_TextChanged(object sender, EventArgs e)
	{
		ButtonX1.Text = "เพ\u0e34\u0e48มรายช\u0e37\u0e48อห\u0e49องละ " + TextBoxX_2.Text + " คนท\u0e38กห\u0e49อง";
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBoxX_2.Text, "", TextCompare: false) == 0)
		{
			TextBoxX_2.Focus();
			MessageBox.Show("กร\u0e38ณากรอกจำนวนคนให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		if (!Versioned.IsNumeric(TextBoxX_2.Text))
		{
			TextBoxX_2.Focus();
			MessageBox.Show("กร\u0e38ณากรอกจำนวนคนให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		checked
		{
			int num = TselectRoom.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				int num5 = Conversions.ToInteger(TextBoxX_2.Text);
				int num6 = 1;
				while (true)
				{
					int num7 = num6;
					num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					addd2(TselectRoom.Items[num2].ToString());
					num6++;
				}
				num2++;
			}
			sum();
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		save(atr: true);
	}

	public void save(bool atr)
	{
		Module1.connect("delete from TB_FOLIO where NO='" + CIN_ID + "'");
		int num = Module1.get_id("TB_FOLIO", "id");
		checked
		{
			int num2 = Grid1.Rows.Count - 1;
			int num3 = 1;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				Module1.connect("INSERT INTO TB_FOLIO VALUES (" + Conversions.ToString(num) + ",'" + CIN_ID + "','" + TextBoxX_1.Text + "','" + TextBoxX_0.Text + "','" + TextBoxX_3.Text + "','" + Conversions.ToString(Grid1[num3, 2]) + "','" + Conversions.ToString(Grid1[num3, 3]) + "','" + Conversions.ToString(Grid1[num3, 4]) + "','" + Conversions.ToString(Grid1[num3, 5]) + "','" + Conversions.ToString(Grid1[num3, 6]) + "','" + Conversions.ToString(Grid1[num3, 7]) + "','" + Conversions.ToString(Grid1[num3, 8]) + "')");
				num++;
				num3++;
			}
			if (atr)
			{
				MessageBox.Show("บ\u0e31นท\u0e36กเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			}
			ISEDIT = false;
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		save(atr: false);
		Print_Report.PrintFolio2(CIN_ID);
	}

	private void EEE(object sender, EventArgs e)
	{
		ISEDIT = true;
	}

	private void Grid1_AfterEdit(object sender, RowColEventArgs e)
	{
		ISEDIT = true;
		if (!((e.Col == 7) | (e.Col == 6)))
		{
			return;
		}
		if (Operators.CompareString(Conversions.ToString(Grid1[e.Row, 6]), "", TextCompare: false) == 0)
		{
			Grid1[e.Row, 7] = "";
			return;
		}
		if (!Versioned.IsNumeric(Conversions.ToString(Grid1[e.Row, 7])))
		{
			Grid1[e.Row, 7] = 0;
		}
		if (!Versioned.IsNumeric(Conversions.ToString(Grid1[e.Row, 6])))
		{
			Grid1[e.Row, 6] = 0;
		}
		Grid1[e.Row, 8] = decimal.Multiply(Conversions.ToDecimal(Conversions.ToString(Grid1[e.Row, 7])), Conversions.ToDecimal(Conversions.ToString(Grid1[e.Row, 6])));
		sum();
	}

	private void Grid1_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Delete)
		{
			try
			{
				Grid1.RemoveItem(Grid1.RowSel);
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			sum();
		}
	}

	private void Grid1_Click(object sender, EventArgs e)
	{
	}
}
