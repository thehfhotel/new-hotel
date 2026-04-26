using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using C1.Win.C1FlexGrid;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormBookingInvoice : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("TextBoxX5")]
	private TextBoxX _TextBoxX5;

	[AccessedThroughProperty("LabelX6")]
	private LabelX _LabelX6;

	[AccessedThroughProperty("Tbook_no")]
	private TextBox _Tbook_no;

	[AccessedThroughProperty("LabelX4")]
	private LabelX _LabelX4;

	[AccessedThroughProperty("Taddress")]
	private TextBox _Taddress;

	[AccessedThroughProperty("Ttel")]
	private TextBox _Ttel;

	[AccessedThroughProperty("Tcompany")]
	private TextBox _Tcompany;

	[AccessedThroughProperty("LabelX8")]
	private LabelX _LabelX8;

	[AccessedThroughProperty("LabelX9")]
	private LabelX _LabelX9;

	[AccessedThroughProperty("Tgroup")]
	private TextBox _Tgroup;

	[AccessedThroughProperty("LabelX7")]
	private LabelX _LabelX7;

	[AccessedThroughProperty("Ttitle")]
	private TextBox _Ttitle;

	[AccessedThroughProperty("LabelX5")]
	private LabelX _LabelX5;

	[AccessedThroughProperty("Tinv_no")]
	private TextBox _Tinv_no;

	[AccessedThroughProperty("Tdue_date")]
	private DateTimePicker _Tdue_date;

	[AccessedThroughProperty("Tpax_c")]
	private TextBox _Tpax_c;

	[AccessedThroughProperty("Tpax")]
	private TextBox _Tpax;

	[AccessedThroughProperty("Tnight")]
	private TextBox _Tnight;

	[AccessedThroughProperty("Tpayment")]
	private TextBox _Tpayment;

	[AccessedThroughProperty("Tdatein")]
	private TextBox _Tdatein;

	[AccessedThroughProperty("LabelX15")]
	private LabelX _LabelX15;

	[AccessedThroughProperty("LabelX14")]
	private LabelX _LabelX14;

	[AccessedThroughProperty("LabelX13")]
	private LabelX _LabelX13;

	[AccessedThroughProperty("LabelX12")]
	private LabelX _LabelX12;

	[AccessedThroughProperty("LabelX11")]
	private LabelX _LabelX11;

	[AccessedThroughProperty("LabelX17")]
	private LabelX _LabelX17;

	[AccessedThroughProperty("LabelX10")]
	private LabelX _LabelX10;

	[AccessedThroughProperty("LabelX16")]
	private LabelX _LabelX16;

	[AccessedThroughProperty("LabelX18")]
	private LabelX _LabelX18;

	[AccessedThroughProperty("Tnote")]
	private TextBox _Tnote;

	[AccessedThroughProperty("LabelX19")]
	private LabelX _LabelX19;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	public string BOOKING_NO;

	private bool ISEDIT;

	private bool EDIT_NOW;

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

	internal virtual TextBoxX TextBoxX_0
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

	internal virtual TextBox Tbook_no
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tbook_no;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tbook_no = value;
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

	internal virtual TextBox Taddress
	{
		[DebuggerNonUserCode]
		get
		{
			return _Taddress;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Taddress = value;
		}
	}

	internal virtual TextBox Ttel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ttel = value;
		}
	}

	internal virtual TextBox Tcompany
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcompany;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcompany = value;
		}
	}

	internal virtual LabelX LabelX8
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX8 = value;
		}
	}

	internal virtual LabelX LabelX9
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX9 = value;
		}
	}

	internal virtual TextBox Tgroup
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tgroup;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tgroup = value;
		}
	}

	internal virtual LabelX LabelX7
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX7 = value;
		}
	}

	internal virtual TextBox Ttitle
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttitle;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ttitle = value;
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

	internal virtual TextBox Tinv_no
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tinv_no;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tinv_no = value;
		}
	}

	internal virtual DateTimePicker Tdue_date
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tdue_date;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tdue_date = value;
		}
	}

	internal virtual TextBox Tpax_c
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpax_c;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpax_c = value;
		}
	}

	internal virtual TextBox Tpax
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpax = value;
		}
	}

	internal virtual TextBox Tnight
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnight;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tnight = value;
		}
	}

	internal virtual TextBox Tpayment
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpayment;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpayment = value;
		}
	}

	internal virtual TextBox Tdatein
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tdatein;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tdatein = value;
		}
	}

	internal virtual LabelX LabelX15
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX15 = value;
		}
	}

	internal virtual LabelX LabelX14
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX14 = value;
		}
	}

	internal virtual LabelX LabelX13
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX13 = value;
		}
	}

	internal virtual LabelX LabelX12
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX12 = value;
		}
	}

	internal virtual LabelX LabelX11
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX11 = value;
		}
	}

	internal virtual LabelX LabelX17
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX17 = value;
		}
	}

	internal virtual LabelX LabelX10
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX10 = value;
		}
	}

	internal virtual LabelX LabelX16
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX16 = value;
		}
	}

	internal virtual LabelX LabelX18
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX18 = value;
		}
	}

	internal virtual TextBox Tnote
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnote;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tnote = value;
		}
	}

	internal virtual LabelX LabelX19
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX19 = value;
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
			_ListView1 = value;
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

	[DebuggerNonUserCode]
	static FormBookingInvoice()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormBookingInvoice()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormFolio_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISEDIT = false;
		EDIT_NOW = false;
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
		this.Tdue_date = new System.Windows.Forms.DateTimePicker();
		this.Taddress = new System.Windows.Forms.TextBox();
		this.Tpax_c = new System.Windows.Forms.TextBox();
		this.Tpax = new System.Windows.Forms.TextBox();
		this.Tnight = new System.Windows.Forms.TextBox();
		this.Tpayment = new System.Windows.Forms.TextBox();
		this.Tdatein = new System.Windows.Forms.TextBox();
		this.Ttel = new System.Windows.Forms.TextBox();
		this.LabelX15 = new DevComponents.DotNetBar.LabelX();
		this.Tcompany = new System.Windows.Forms.TextBox();
		this.LabelX14 = new DevComponents.DotNetBar.LabelX();
		this.LabelX13 = new DevComponents.DotNetBar.LabelX();
		this.LabelX12 = new DevComponents.DotNetBar.LabelX();
		this.LabelX11 = new DevComponents.DotNetBar.LabelX();
		this.LabelX17 = new DevComponents.DotNetBar.LabelX();
		this.LabelX10 = new DevComponents.DotNetBar.LabelX();
		this.LabelX16 = new DevComponents.DotNetBar.LabelX();
		this.LabelX18 = new DevComponents.DotNetBar.LabelX();
		this.LabelX8 = new DevComponents.DotNetBar.LabelX();
		this.LabelX9 = new DevComponents.DotNetBar.LabelX();
		this.Tgroup = new System.Windows.Forms.TextBox();
		this.LabelX7 = new DevComponents.DotNetBar.LabelX();
		this.Ttitle = new System.Windows.Forms.TextBox();
		this.LabelX5 = new DevComponents.DotNetBar.LabelX();
		this.Tinv_no = new System.Windows.Forms.TextBox();
		this.LabelX4 = new DevComponents.DotNetBar.LabelX();
		this.Tbook_no = new System.Windows.Forms.TextBox();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX6 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.LabelX19 = new DevComponents.DotNetBar.LabelX();
		this.Tnote = new System.Windows.Forms.TextBox();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.ListView1);
		this.PanelEx1.Controls.Add(this.Tdue_date);
		this.PanelEx1.Controls.Add(this.Tnote);
		this.PanelEx1.Controls.Add(this.Taddress);
		this.PanelEx1.Controls.Add(this.Tpax_c);
		this.PanelEx1.Controls.Add(this.Tpax);
		this.PanelEx1.Controls.Add(this.Tnight);
		this.PanelEx1.Controls.Add(this.Tpayment);
		this.PanelEx1.Controls.Add(this.Tdatein);
		this.PanelEx1.Controls.Add(this.Ttel);
		this.PanelEx1.Controls.Add(this.LabelX15);
		this.PanelEx1.Controls.Add(this.Tcompany);
		this.PanelEx1.Controls.Add(this.LabelX14);
		this.PanelEx1.Controls.Add(this.LabelX13);
		this.PanelEx1.Controls.Add(this.LabelX12);
		this.PanelEx1.Controls.Add(this.LabelX11);
		this.PanelEx1.Controls.Add(this.LabelX17);
		this.PanelEx1.Controls.Add(this.LabelX10);
		this.PanelEx1.Controls.Add(this.LabelX16);
		this.PanelEx1.Controls.Add(this.LabelX19);
		this.PanelEx1.Controls.Add(this.LabelX18);
		this.PanelEx1.Controls.Add(this.LabelX8);
		this.PanelEx1.Controls.Add(this.LabelX9);
		this.PanelEx1.Controls.Add(this.Tgroup);
		this.PanelEx1.Controls.Add(this.LabelX7);
		this.PanelEx1.Controls.Add(this.Ttitle);
		this.PanelEx1.Controls.Add(this.LabelX5);
		this.PanelEx1.Controls.Add(this.Tinv_no);
		this.PanelEx1.Controls.Add(this.LabelX4);
		this.PanelEx1.Controls.Add(this.Tbook_no);
		this.PanelEx1.Controls.Add(this.TextBoxX_0);
		this.PanelEx1.Controls.Add(this.LabelX6);
		this.PanelEx1.Controls.Add(this.ButtonX3);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Controls.Add(this.LabelX2);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(757, 741);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.Tdue_date.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.DateTimePicker tdue_date = this.Tdue_date;
		location = new System.Drawing.Point(152, 366);
		tdue_date.Location = location;
		this.Tdue_date.Name = "Tdue_date";
		System.Windows.Forms.DateTimePicker tdue_date2 = this.Tdue_date;
		size = new System.Drawing.Size(284, 27);
		tdue_date2.Size = size;
		this.Tdue_date.TabIndex = 12;
		this.Taddress.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox taddress = this.Taddress;
		location = new System.Drawing.Point(152, 159);
		taddress.Location = location;
		this.Taddress.Multiline = true;
		this.Taddress.Name = "Taddress";
		System.Windows.Forms.TextBox taddress2 = this.Taddress;
		size = new System.Drawing.Size(475, 70);
		taddress2.Size = size;
		this.Taddress.TabIndex = 5;
		this.Tpax_c.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Tpax_c.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tpax_c = this.Tpax_c;
		location = new System.Drawing.Point(364, 306);
		tpax_c.Location = location;
		this.Tpax_c.Name = "Tpax_c";
		System.Windows.Forms.TextBox tpax_c2 = this.Tpax_c;
		size = new System.Drawing.Size(72, 27);
		tpax_c2.Size = size;
		this.Tpax_c.TabIndex = 10;
		this.Tpax_c.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Tpax.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Tpax.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tpax = this.Tpax;
		location = new System.Drawing.Point(152, 306);
		tpax.Location = location;
		this.Tpax.Name = "Tpax";
		System.Windows.Forms.TextBox tpax2 = this.Tpax;
		size = new System.Drawing.Size(72, 27);
		tpax2.Size = size;
		this.Tpax.TabIndex = 9;
		this.Tpax.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Tnight.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Tnight.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tnight = this.Tnight;
		location = new System.Drawing.Point(152, 280);
		tnight.Location = location;
		this.Tnight.Name = "Tnight";
		System.Windows.Forms.TextBox tnight2 = this.Tnight;
		size = new System.Drawing.Size(72, 27);
		tnight2.Size = size;
		this.Tnight.TabIndex = 8;
		this.Tnight.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Tpayment.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tpayment = this.Tpayment;
		location = new System.Drawing.Point(152, 340);
		tpayment.Location = location;
		this.Tpayment.Name = "Tpayment";
		System.Windows.Forms.TextBox tpayment2 = this.Tpayment;
		size = new System.Drawing.Size(284, 27);
		tpayment2.Size = size;
		this.Tpayment.TabIndex = 11;
		this.Tdatein.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tdatein = this.Tdatein;
		location = new System.Drawing.Point(152, 254);
		tdatein.Location = location;
		this.Tdatein.Name = "Tdatein";
		System.Windows.Forms.TextBox tdatein2 = this.Tdatein;
		size = new System.Drawing.Size(475, 27);
		tdatein2.Size = size;
		this.Tdatein.TabIndex = 7;
		this.Ttel.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox ttel = this.Ttel;
		location = new System.Drawing.Point(152, 228);
		ttel.Location = location;
		this.Ttel.Name = "Ttel";
		System.Windows.Forms.TextBox ttel2 = this.Ttel;
		size = new System.Drawing.Size(475, 27);
		ttel2.Size = size;
		this.Ttel.TabIndex = 6;
		this.LabelX15.BackgroundStyle.Class = "";
		this.LabelX15.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX = this.LabelX15;
		location = new System.Drawing.Point(442, 306);
		labelX.Location = location;
		this.LabelX15.Name = "LabelX15";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX15;
		size = new System.Drawing.Size(144, 28);
		labelX2.Size = size;
		this.LabelX15.TabIndex = 100;
		this.LabelX15.Text = "คน";
		this.Tcompany.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tcompany = this.Tcompany;
		location = new System.Drawing.Point(152, 133);
		tcompany.Location = location;
		this.Tcompany.Name = "Tcompany";
		System.Windows.Forms.TextBox tcompany2 = this.Tcompany;
		size = new System.Drawing.Size(475, 27);
		tcompany2.Size = size;
		this.Tcompany.TabIndex = 4;
		this.LabelX14.BackgroundStyle.Class = "";
		this.LabelX14.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX14;
		location = new System.Drawing.Point(270, 306);
		labelX3.Location = location;
		this.LabelX14.Name = "LabelX14";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX14;
		size = new System.Drawing.Size(104, 28);
		labelX4.Size = size;
		this.LabelX14.TabIndex = 100;
		this.LabelX14.Text = "จำนวน เด\u0e47ก";
		this.LabelX13.BackgroundStyle.Class = "";
		this.LabelX13.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX13;
		location = new System.Drawing.Point(230, 306);
		labelX5.Location = location;
		this.LabelX13.Name = "LabelX13";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX13;
		size = new System.Drawing.Size(144, 28);
		labelX6.Size = size;
		this.LabelX13.TabIndex = 100;
		this.LabelX13.Text = "คน";
		this.LabelX12.BackgroundStyle.Class = "";
		this.LabelX12.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX7 = this.LabelX12;
		location = new System.Drawing.Point(12, 306);
		labelX7.Location = location;
		this.LabelX12.Name = "LabelX12";
		DevComponents.DotNetBar.LabelX labelX8 = this.LabelX12;
		size = new System.Drawing.Size(144, 28);
		labelX8.Size = size;
		this.LabelX12.TabIndex = 100;
		this.LabelX12.Text = "จำนวน ผ\u0e39\u0e49ใหญ\u0e4b";
		this.LabelX11.BackgroundStyle.Class = "";
		this.LabelX11.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX9 = this.LabelX11;
		location = new System.Drawing.Point(230, 280);
		labelX9.Location = location;
		this.LabelX11.Name = "LabelX11";
		DevComponents.DotNetBar.LabelX labelX10 = this.LabelX11;
		size = new System.Drawing.Size(144, 28);
		labelX10.Size = size;
		this.LabelX11.TabIndex = 100;
		this.LabelX11.Text = "ค\u0e37น";
		this.LabelX17.BackgroundStyle.Class = "";
		this.LabelX17.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX11 = this.LabelX17;
		location = new System.Drawing.Point(12, 366);
		labelX11.Location = location;
		this.LabelX17.Name = "LabelX17";
		DevComponents.DotNetBar.LabelX labelX12 = this.LabelX17;
		size = new System.Drawing.Size(144, 28);
		labelX12.Size = size;
		this.LabelX17.TabIndex = 100;
		this.LabelX17.Text = "ครบกำหนด";
		this.LabelX10.BackgroundStyle.Class = "";
		this.LabelX10.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX13 = this.LabelX10;
		location = new System.Drawing.Point(12, 280);
		labelX13.Location = location;
		this.LabelX10.Name = "LabelX10";
		DevComponents.DotNetBar.LabelX labelX14 = this.LabelX10;
		size = new System.Drawing.Size(144, 28);
		labelX14.Size = size;
		this.LabelX10.TabIndex = 100;
		this.LabelX10.Text = "จำนวน ค\u0e37น";
		this.LabelX16.BackgroundStyle.Class = "";
		this.LabelX16.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX15 = this.LabelX16;
		location = new System.Drawing.Point(12, 340);
		labelX15.Location = location;
		this.LabelX16.Name = "LabelX16";
		DevComponents.DotNetBar.LabelX labelX16 = this.LabelX16;
		size = new System.Drawing.Size(144, 28);
		labelX16.Size = size;
		this.LabelX16.TabIndex = 100;
		this.LabelX16.Text = "ประเภทการชำระ";
		this.LabelX18.BackgroundStyle.Class = "";
		this.LabelX18.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX17 = this.LabelX18;
		location = new System.Drawing.Point(12, 254);
		labelX17.Location = location;
		this.LabelX18.Name = "LabelX18";
		DevComponents.DotNetBar.LabelX labelX18 = this.LabelX18;
		size = new System.Drawing.Size(144, 28);
		labelX18.Size = size;
		this.LabelX18.TabIndex = 100;
		this.LabelX18.Text = "เข\u0e49าพ\u0e31กว\u0e31นท\u0e35\u0e48";
		this.LabelX8.BackgroundStyle.Class = "";
		this.LabelX8.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX19 = this.LabelX8;
		location = new System.Drawing.Point(12, 159);
		labelX19.Location = location;
		this.LabelX8.Name = "LabelX8";
		DevComponents.DotNetBar.LabelX labelX20 = this.LabelX8;
		size = new System.Drawing.Size(144, 28);
		labelX20.Size = size;
		this.LabelX8.TabIndex = 100;
		this.LabelX8.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.LabelX9.BackgroundStyle.Class = "";
		this.LabelX9.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX21 = this.LabelX9;
		location = new System.Drawing.Point(12, 228);
		labelX21.Location = location;
		this.LabelX9.Name = "LabelX9";
		DevComponents.DotNetBar.LabelX labelX22 = this.LabelX9;
		size = new System.Drawing.Size(144, 28);
		labelX22.Size = size;
		this.LabelX9.TabIndex = 100;
		this.LabelX9.Text = "เบอร\u0e4cโทร";
		this.Tgroup.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tgroup = this.Tgroup;
		location = new System.Drawing.Point(152, 107);
		tgroup.Location = location;
		this.Tgroup.Name = "Tgroup";
		System.Windows.Forms.TextBox tgroup2 = this.Tgroup;
		size = new System.Drawing.Size(475, 27);
		tgroup2.Size = size;
		this.Tgroup.TabIndex = 3;
		this.LabelX7.BackgroundStyle.Class = "";
		this.LabelX7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX23 = this.LabelX7;
		location = new System.Drawing.Point(12, 133);
		labelX23.Location = location;
		this.LabelX7.Name = "LabelX7";
		DevComponents.DotNetBar.LabelX labelX24 = this.LabelX7;
		size = new System.Drawing.Size(144, 28);
		labelX24.Size = size;
		this.LabelX7.TabIndex = 100;
		this.LabelX7.Text = "ช\u0e37\u0e48อหน\u0e48วยงาน";
		this.Ttitle.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox ttitle = this.Ttitle;
		location = new System.Drawing.Point(152, 81);
		ttitle.Location = location;
		this.Ttitle.Name = "Ttitle";
		System.Windows.Forms.TextBox ttitle2 = this.Ttitle;
		size = new System.Drawing.Size(475, 27);
		ttitle2.Size = size;
		this.Ttitle.TabIndex = 2;
		this.LabelX5.BackgroundStyle.Class = "";
		this.LabelX5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX25 = this.LabelX5;
		location = new System.Drawing.Point(12, 107);
		labelX25.Location = location;
		this.LabelX5.Name = "LabelX5";
		DevComponents.DotNetBar.LabelX labelX26 = this.LabelX5;
		size = new System.Drawing.Size(144, 28);
		labelX26.Size = size;
		this.LabelX5.TabIndex = 100;
		this.LabelX5.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.Tinv_no.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tinv_no.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tinv_no = this.Tinv_no;
		location = new System.Drawing.Point(457, 49);
		tinv_no.Location = location;
		this.Tinv_no.Name = "Tinv_no";
		this.Tinv_no.ReadOnly = true;
		System.Windows.Forms.TextBox tinv_no2 = this.Tinv_no;
		size = new System.Drawing.Size(170, 27);
		tinv_no2.Size = size;
		this.Tinv_no.TabIndex = 1;
		this.LabelX4.BackgroundStyle.Class = "";
		this.LabelX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX27 = this.LabelX4;
		location = new System.Drawing.Point(12, 81);
		labelX27.Location = location;
		this.LabelX4.Name = "LabelX4";
		DevComponents.DotNetBar.LabelX labelX28 = this.LabelX4;
		size = new System.Drawing.Size(144, 28);
		labelX28.Size = size;
		this.LabelX4.TabIndex = 100;
		this.LabelX4.Text = "โครงการ";
		this.Tbook_no.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tbook_no.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tbook_no = this.Tbook_no;
		location = new System.Drawing.Point(152, 49);
		tbook_no.Location = location;
		this.Tbook_no.Name = "Tbook_no";
		this.Tbook_no.ReadOnly = true;
		System.Windows.Forms.TextBox tbook_no2 = this.Tbook_no;
		size = new System.Drawing.Size(170, 27);
		tbook_no2.Size = size;
		this.Tbook_no.TabIndex = 0;
		this.TextBoxX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		this.TextBoxX_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBoxX_0.ForeColor = System.Drawing.Color.Red;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_0;
		location = new System.Drawing.Point(93, 702);
		textBoxX_.Location = location;
		this.TextBoxX_0.MaxLength = 255;
		this.TextBoxX_0.Name = "TextBoxX5";
		this.TextBoxX_0.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_0;
		size = new System.Drawing.Size(143, 27);
		textBoxX_2.Size = size;
		this.TextBoxX_0.TabIndex = 98;
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.LabelX6.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LabelX6.BackgroundStyle.Class = "";
		this.LabelX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX29 = this.LabelX6;
		location = new System.Drawing.Point(16, 698);
		labelX29.Location = location;
		this.LabelX6.Name = "LabelX6";
		DevComponents.DotNetBar.LabelX labelX30 = this.LabelX6;
		size = new System.Drawing.Size(71, 31);
		labelX30.Size = size;
		this.LabelX6.TabIndex = 97;
		this.LabelX6.Text = "ราคารวม";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX3;
		location = new System.Drawing.Point(649, 702);
		buttonX.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX3;
		size = new System.Drawing.Size(96, 27);
		buttonX2.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 14;
		this.ButtonX3.Text = "บ\u0e31นท\u0e36ก/พ\u0e34มพ\u0e4c";
		this.LabelX3.BackgroundStyle.Class = "";
		this.LabelX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX31 = this.LabelX3;
		location = new System.Drawing.Point(338, 48);
		labelX31.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX32 = this.LabelX3;
		size = new System.Drawing.Size(144, 28);
		labelX32.Size = size;
		this.LabelX3.TabIndex = 2;
		this.LabelX3.Text = "INVOICE NO.";
		this.LabelX2.BackgroundStyle.Class = "";
		this.LabelX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX33 = this.LabelX2;
		location = new System.Drawing.Point(12, 49);
		labelX33.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX34 = this.LabelX2;
		size = new System.Drawing.Size(144, 28);
		labelX34.Size = size;
		this.LabelX2.TabIndex = 1;
		this.LabelX2.Text = "หมายเลขการจอง";
		this.LabelX1.BackColor = System.Drawing.Color.Transparent;
		this.LabelX1.BackgroundStyle.Class = "";
		this.LabelX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX35 = this.LabelX1;
		location = new System.Drawing.Point(12, 8);
		labelX35.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX36 = this.LabelX1;
		size = new System.Drawing.Size(692, 26);
		labelX36.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ใบ INVOICE";
		this.LabelX1.TextAlignment = System.Drawing.StringAlignment.Center;
		this.LabelX19.BackgroundStyle.Class = "";
		this.LabelX19.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.LabelX labelX37 = this.LabelX19;
		location = new System.Drawing.Point(12, 400);
		labelX37.Location = location;
		this.LabelX19.Name = "LabelX19";
		DevComponents.DotNetBar.LabelX labelX38 = this.LabelX19;
		size = new System.Drawing.Size(144, 28);
		labelX38.Size = size;
		this.LabelX19.TabIndex = 100;
		this.LabelX19.Text = "หมายเหต\u0e38";
		this.Tnote.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tnote = this.Tnote;
		location = new System.Drawing.Point(152, 400);
		tnote.Location = location;
		this.Tnote.Multiline = true;
		this.Tnote.Name = "Tnote";
		System.Windows.Forms.TextBox tnote2 = this.Tnote;
		size = new System.Drawing.Size(475, 70);
		tnote2.Size = size;
		this.Tnote.TabIndex = 5;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[7] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6, this.ColumnHeader7 });
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(12, 476);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(733, 216);
		listView2.Size = size;
		this.ListView1.TabIndex = 101;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "Room TYPE";
		this.ColumnHeader1.Width = 150;
		this.ColumnHeader2.Text = "Check-IN";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader2.Width = 80;
		this.ColumnHeader3.Text = "Check-OUT";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader3.Width = 80;
		this.ColumnHeader4.Text = "Rate";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 80;
		this.ColumnHeader5.Text = "QTY";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader6.Text = "Night(s)";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader7.Text = "Total";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 100;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(757, 741);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormBookingInvoice";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ใบ INVOICE";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FormFolio_Load(object sender, EventArgs e)
	{
		EDIT_NOW = false;
		ISEDIT = false;
		Tnight.Text = Conversions.ToString(0);
		DataSet dataSet = Module1.connect("select * from HT_Book_H where book_id='" + BOOKING_NO + "'");
		checked
		{
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				Tbook_no.Text = BOOKING_NO;
				Tinv_no.Text = Strings.Format(Module1.get_id("HT_INVOICE", "INV_NO"), "0000");
				DataSet dataSet2 = Module1.connect("select * from HT_Book_Ds where book_no='" + BOOKING_NO + "' order by book_room_start asc");
				int num = dataSet2.Tables[0].Rows.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					TextBox tnight = Tnight;
					Type typeFromHandle = typeof(Math);
					object[] array = new object[2]
					{
						Conversions.ToInteger(Tnight.Text),
						null
					};
					DataRow dataRow = dataSet2.Tables[0].Rows[num2];
					string columnName = "book_room_night";
					array[1] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[2] { false, true };
					object obj = NewLateBinding.LateGet(null, typeFromHandle, "Max", array2, null, null, array3);
					if (array3[1])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[1]);
					}
					tnight.Text = Conversions.ToString(obj);
					num2++;
				}
				DataSet dataSet3 = Module1.connect("select book_room_end from HT_Book_Ds where book_no='" + BOOKING_NO + "' order by book_room_end desc");
				Tdatein.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["book_room_start"]), "dd/MM/yyyy") + " - " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[0]["book_room_end"]), "dd/MM/yyyy");
				DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_Customers where Cust_no='", dataSet.Tables[0].Rows[0]["Book_Cust_ID"]), "'")));
				if (dataSet4.Tables[0].Rows.Count != 0)
				{
					if (Operators.CompareString(dataSet4.Tables[0].Rows[0]["Cust_Work_Name"].ToString(), "", TextCompare: false) != 0)
					{
						Tcompany.Text = dataSet4.Tables[0].Rows[0]["Cust_Work_Name"].ToString();
						Taddress.Text = dataSet4.Tables[0].Rows[0]["W_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
							.Replace("ถนน ", "")
							.Replace("เขต/อำเภอ ", "")
							.Replace("แขวง/ตำบล ", "")
							.Replace("จ\u0e31งหว\u0e31ด ", "");
						Ttel.Text = dataSet4.Tables[0].Rows[0]["Cust_Add_tel"].ToString();
					}
					else
					{
						Tcompany.Text = dataSet4.Tables[0].Rows[0]["Cust_name"].ToString();
						Taddress.Text = dataSet4.Tables[0].Rows[0]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
							.Replace("ถนน ", "")
							.Replace("เขต/อำเภอ ", "")
							.Replace("แขวง/ตำบล ", "")
							.Replace("จ\u0e31งหว\u0e31ด ", "");
						Ttel.Text = dataSet4.Tables[0].Rows[0]["Cust_Add_tel"].ToString();
					}
				}
				Load_INV();
				chkEdit();
			}
			else
			{
				MessageBox.Show("ไม\u0e48พบหมายเลขการจอง " + BOOKING_NO, "error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				Close();
			}
		}
	}

	public void Load_INV()
	{
		DataSet dataSet = Module1.connect("select * from HT_INVOICE where INV_booking_no='" + Tbook_no.Text + "'");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			EDIT_NOW = true;
			Tinv_no.Text = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["INV_NO"]), "0000");
			Ttitle.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_TITLE"]);
			Tgroup.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_NAME"]);
			Tcompany.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_COMPANY"]);
			Taddress.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_ADDRESS"]);
			Ttel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_TEL"]);
			Tnight.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_NIGHT"]);
			Tpax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_PAX"]);
			Tpax_c.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_PAX_CHILD"]);
			Tpayment.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_PAYMENT"]);
			Tdue_date.Value = Conversions.ToDate(dataSet.Tables[0].Rows[0]["INV_DUEDATE"]);
			Tnote.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_NOTE"]);
			Tdatein.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["INV_STAY"]);
		}
	}

	public void chkEdit()
	{
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect("select * from HT_Book_H where book_id='" + BOOKING_NO + "'");
		DataSet dataSet2 = Module1.connect("SELECT     dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, SUM(dbo.HT_Book_Ds.Book_Room_Num) AS Book_Room_Num, dbo.HT_Book_Ds.Book_Room_Note, dbo.HT_Book_Ds.Book_status, dbo.HT_Rooms.Room_Type FROM         dbo.HT_Book_Ds INNER JOIN dbo.HT_Rooms ON dbo.HT_Book_Ds.Book_Room_Type = dbo.HT_Rooms.Room_no where book_no='" + BOOKING_NO + "' GROUP BY dbo.HT_Book_Ds.Book_No, dbo.HT_Book_Ds.Book_Room_Start, dbo.HT_Book_Ds.Book_Room_End, dbo.HT_Book_Ds.Book_Room_Price, dbo.HT_Book_Ds.Book_Room_Night, dbo.HT_Book_Ds.Book_Room_Num, dbo.HT_Book_Ds.Book_Room_Note, dbo.HT_Book_Ds.Book_status, dbo.HT_Rooms.Room_Type ");
		DataSet dataSet3 = Module1.connect("select B_NAME,sum(B_NUM) as B_NUM,B_PRICE from HT_Book_Pro where B_NO='" + BOOKING_NO + "' group by B_NAME,B_PRICE");
		decimal num = default(decimal);
		checked
		{
			int num2 = dataSet2.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(Operators.MultiplyObject(dataSet2.Tables[0].Rows[num3]["book_room_price"], dataSet2.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet2.Tables[0].Rows[num3]["book_room_night"])));
				ListView listView = ListView1;
				int count = listView.Items.Count;
				ListView.ListViewItemCollection items = listView.Items;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet2.Tables[0].Rows[num3];
				DataRow dataRow2 = dataRow;
				string columnName = "room_type";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num3]["Book_Room_Start"]), "dd/MM/yyyy"));
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num3]["Book_Room_End"]), "dd/MM/yyyy"));
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num3]["book_room_price"]), "#,##0.00"));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet2.Tables[0].Rows[num3];
				DataRow dataRow3 = dataRow;
				columnName = "Book_Room_Num";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet2.Tables[0].Rows[num3];
				DataRow dataRow4 = dataRow;
				columnName = "Book_Room_Night";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(Operators.MultiplyObject(Operators.MultiplyObject(dataSet2.Tables[0].Rows[num3]["book_room_price"], dataSet2.Tables[0].Rows[num3]["Book_Room_Num"]), dataSet2.Tables[0].Rows[num3]["book_room_night"]), "#,##0.00"));
				listView = null;
				num3++;
			}
			int num6 = dataSet3.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 > num5)
				{
					break;
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, Operators.MultiplyObject(dataSet3.Tables[0].Rows[num7]["B_NUM"], dataSet3.Tables[0].Rows[num7]["B_PRICE"])));
				Module1.localdata.ReportBooking.AddReportBookingRow(BOOKING_NO, Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["book_cust_name"], " "), dataSet.Tables[0].Rows[0]["book_cust_name2"])), "-", "-", "", Conversions.ToString(dataSet3.Tables[0].Rows[num7]["B_NAME"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"), Conversions.ToString(dataSet3.Tables[0].Rows[num7]["B_NUM"]), "", Strings.Format(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num7]["B_NUM"], dataSet3.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[0]["book_room_note"]), Conversions.ToString(dataSet.Tables[0].Rows[0]["book_by"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["book_date"]), "dd/MM/yyyy"), "-", "-", Strings.Format(num, "#,##0.00"));
				ListView listView2 = ListView1;
				int count2 = listView2.Items.Count;
				ListView.ListViewItemCollection items2 = listView2.Items;
				object[] array3 = new object[1];
				object[] array7 = array3;
				DataRow dataRow = dataSet3.Tables[0].Rows[num7];
				DataRow dataRow5 = dataRow;
				string columnName = "B_NAME";
				array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
				object[] array = array3;
				object[] arguments4 = array;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(items2, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView2.Items[count2].SubItems.Add("");
				listView2.Items[count2].SubItems.Add("");
				listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"));
				ListViewItem.ListViewSubItemCollection subItems3 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array8 = array3;
				dataRow = dataSet3.Tables[0].Rows[num7];
				DataRow dataRow6 = dataRow;
				columnName = "B_NUM";
				array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
				array = array3;
				object[] arguments5 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView2.Items[count2].SubItems.Add("");
				listView2.Items[count2].SubItems.Add(Strings.Format(Operators.MultiplyObject(dataSet3.Tables[0].Rows[num7]["B_NUM"], dataSet3.Tables[0].Rows[num7]["B_PRICE"]), "#,##0.00"));
				listView2 = null;
				num7++;
			}
			TextBoxX_0.Text = Strings.Format(num, "#,##0.00");
		}
	}

	private void Grid1_AfterEdit(object sender, RowColEventArgs e)
	{
	}

	private void Grid1_Click(object sender, EventArgs e)
	{
	}

	private void Grid1_AfterDeleteRow(object sender, RowColEventArgs e)
	{
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		save();
		Print_Report.print_inv_booking(Tbook_no.Text);
	}

	public void save()
	{
		Module1.connect("delete from HT_INVOICE where INV_booking_no='" + Tbook_no.Text + "'");
		string right = Strings.Format(Module1.get_id("HT_INVOICE", "INV_NO"), "0000");
		if (EDIT_NOW)
		{
			right = Tinv_no.Text;
		}
		object left = "INSERT INTO [HT_INVOICE]";
		left = Operators.ConcatenateObject(left, "(");
		left = Operators.ConcatenateObject(left, " [INV_NO]");
		left = Operators.ConcatenateObject(left, ",[INV_booking_no]");
		left = Operators.ConcatenateObject(left, ",[INV_DATE]");
		left = Operators.ConcatenateObject(left, ",[INV_BY]");
		left = Operators.ConcatenateObject(left, ",[INV_TITLE]");
		left = Operators.ConcatenateObject(left, ",[INV_NAME]");
		left = Operators.ConcatenateObject(left, ",[INV_COMPANY]");
		left = Operators.ConcatenateObject(left, ",[INV_ADDRESS]");
		left = Operators.ConcatenateObject(left, ",[INV_TEL]");
		left = Operators.ConcatenateObject(left, ",[INV_NIGHT]");
		left = Operators.ConcatenateObject(left, ",[INV_PAX]");
		left = Operators.ConcatenateObject(left, ",[INV_PAX_CHILD]");
		left = Operators.ConcatenateObject(left, ",[INV_PAYMENT]");
		left = Operators.ConcatenateObject(left, ",[INV_DUEDATE]");
		left = Operators.ConcatenateObject(left, ",[INV_NOTE]");
		left = Operators.ConcatenateObject(left, ",[INV_STAY])");
		left = Operators.ConcatenateObject(left, "VALUES(");
		left = Operators.ConcatenateObject(left, right);
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tbook_no.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now), "'"));
		left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", Module1.loginName), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Ttitle.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tgroup.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tcompany.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Taddress.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Ttel.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tnight.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tpax.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tpax_c.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tpayment.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(Tdue_date.Value), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tnote.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",'" + Tdatein.Text, "'"));
		left = Operators.ConcatenateObject(left, ")");
		Module1.connect(Conversions.ToString(left));
		EDIT_NOW = true;
	}
}
