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
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmManageRoom : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("GroupBox2")]
	private GroupBox _GroupBox2;

	[AccessedThroughProperty("ยกเล\u0e34ก")]
	private ButtonX buttonX_0;

	[AccessedThroughProperty("บ\u0e31นท\u0e36ก")]
	private ButtonX buttonX_1;

	[AccessedThroughProperty("ListViewEx1")]
	private global::PrintableListView.PrintableListView _ListViewEx1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("ลบ")]
	private ButtonX buttonX_2;

	[AccessedThroughProperty("แก\u0e49ไข")]
	private ButtonX buttonX_3;

	[AccessedThroughProperty("เพ\u0e34\u0e48ม")]
	private ButtonX buttonX_4;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("AddMenu")]
	private ToolStripMenuItem _AddMenu;

	[AccessedThroughProperty("EditMenu")]
	private ToolStripMenuItem _EditMenu;

	[AccessedThroughProperty("DelMenu")]
	private ToolStripMenuItem _DelMenu;

	[AccessedThroughProperty("RpriceA")]
	private TextBoxX _RpriceA;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("Rno")]
	private TextBoxX _Rno;

	[AccessedThroughProperty("Rdtails")]
	private TextBoxX _Rdtails;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("RpriceC")]
	private TextBoxX _RpriceC;

	[AccessedThroughProperty("RpriceB")]
	private TextBoxX _RpriceB;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Rtype")]
	private ComboBox _Rtype;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("SuperTooltip1")]
	private SuperTooltip _SuperTooltip1;

	[AccessedThroughProperty("ComboGroup")]
	private ComboBox _ComboGroup;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("POWER_OFF")]
	private TextBoxX _POWER_OFF;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("POWER_ON")]
	private TextBoxX _POWER_ON;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	public string EditID;

	internal virtual ComboItem ComboItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem1 = value;
		}
	}

	internal virtual ComboItem ComboItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem2 = value;
		}
	}

	internal virtual TabItem TabItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _TabItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TabItem4 = value;
		}
	}

	internal virtual GroupBox GroupBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox1 = value;
		}
	}

	internal virtual GroupBox GroupBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox2 = value;
		}
	}

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX_0_Click;
			if (buttonX_0 != null)
			{
				buttonX_0.Click -= value2;
			}
			buttonX_0 = value;
			if (buttonX_0 != null)
			{
				buttonX_0.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_1
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX_1_Click;
			if (buttonX_1 != null)
			{
				buttonX_1.Click -= value2;
			}
			buttonX_1 = value;
			if (buttonX_1 != null)
			{
				buttonX_1.Click += value2;
			}
		}
	}

	internal virtual global::PrintableListView.PrintableListView ListViewEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListViewEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListViewEx1_SelectedIndexChanged;
			if (_ListViewEx1 != null)
			{
				_ListViewEx1.SelectedIndexChanged -= value2;
			}
			_ListViewEx1 = value;
			if (_ListViewEx1 != null)
			{
				_ListViewEx1.SelectedIndexChanged += value2;
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

	internal virtual ColumnHeader ColumnHeader8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader8 = value;
		}
	}

	internal virtual Label Label7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label7 = value;
		}
	}

	internal virtual ButtonX ButtonX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX_2_Click;
			if (buttonX_2 != null)
			{
				buttonX_2.Click -= value2;
			}
			buttonX_2 = value;
			if (buttonX_2 != null)
			{
				buttonX_2.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_3
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX_3_Click;
			if (buttonX_3 != null)
			{
				buttonX_3.Click -= value2;
			}
			buttonX_3 = value;
			if (buttonX_3 != null)
			{
				buttonX_3.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_4
	{
		[DebuggerNonUserCode]
		get
		{
			return buttonX_4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX_4_Click;
			if (buttonX_4 != null)
			{
				buttonX_4.Click -= value2;
			}
			buttonX_4 = value;
			if (buttonX_4 != null)
			{
				buttonX_4.Click += value2;
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
			_Timer1 = value;
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

	internal virtual Label Label8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label8 = value;
		}
	}

	internal virtual Label Label5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label5 = value;
		}
	}

	internal virtual ContextMenuStrip ContextMenuStrip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ContextMenuStrip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ContextMenuStrip1 = value;
		}
	}

	internal virtual ToolStripMenuItem AddMenu
	{
		[DebuggerNonUserCode]
		get
		{
			return _AddMenu;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_AddMenu = value;
		}
	}

	internal virtual ToolStripMenuItem EditMenu
	{
		[DebuggerNonUserCode]
		get
		{
			return _EditMenu;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_EditMenu = value;
		}
	}

	internal virtual ToolStripMenuItem DelMenu
	{
		[DebuggerNonUserCode]
		get
		{
			return _DelMenu;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DelMenu = value;
		}
	}

	internal virtual TextBoxX RpriceA
	{
		[DebuggerNonUserCode]
		get
		{
			return _RpriceA;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RpriceA = value;
		}
	}

	internal virtual Label Label14
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label14 = value;
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

	internal virtual TextBoxX Rno
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rno;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Rno = value;
		}
	}

	internal virtual TextBoxX Rdtails
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rdtails;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Rdtails = value;
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

	internal virtual TextBoxX RpriceC
	{
		[DebuggerNonUserCode]
		get
		{
			return _RpriceC;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RpriceC = value;
		}
	}

	internal virtual TextBoxX RpriceB
	{
		[DebuggerNonUserCode]
		get
		{
			return _RpriceB;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RpriceB = value;
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

	internal virtual ComboBox Rtype
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rtype;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Rtype_SelectedIndexChanged;
			if (_Rtype != null)
			{
				_Rtype.SelectedIndexChanged -= value2;
			}
			_Rtype = value;
			if (_Rtype != null)
			{
				_Rtype.SelectedIndexChanged += value2;
			}
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

	internal virtual ComboBox ComboGroup
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboGroup;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Rtype_SelectedIndexChanged;
			if (_ComboGroup != null)
			{
				_ComboGroup.SelectedIndexChanged -= value2;
			}
			_ComboGroup = value;
			if (_ComboGroup != null)
			{
				_ComboGroup.SelectedIndexChanged += value2;
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

	internal virtual ColumnHeader ColumnHeader9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader9 = value;
		}
	}

	internal virtual TextBoxX POWER_OFF
	{
		[DebuggerNonUserCode]
		get
		{
			return _POWER_OFF;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_POWER_OFF = value;
		}
	}

	internal virtual Label Label9
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label9 = value;
		}
	}

	internal virtual TextBoxX POWER_ON
	{
		[DebuggerNonUserCode]
		get
		{
			return _POWER_ON;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_POWER_ON = value;
		}
	}

	internal virtual Label Label6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label6 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader10
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader10 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader11
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader11 = value;
		}
	}

	internal virtual Label Label10
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label10 = value;
		}
	}

	internal virtual Label Label11
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label11 = value;
		}
	}

	internal virtual Label Label12
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label12 = value;
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

	internal virtual Label Label13
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label13 = value;
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

	[DebuggerNonUserCode]
	static FrmManageRoom()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmManageRoom()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmManageRoom_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EditID = "";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmManageRoom));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ButtonX_2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_4 = new DevComponents.DotNetBar.ButtonX();
		this.ListViewEx1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.AddMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.EditMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.DelMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.Label7 = new System.Windows.Forms.Label();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.Label13 = new System.Windows.Forms.Label();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.POWER_OFF = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label9 = new System.Windows.Forms.Label();
		this.POWER_ON = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label6 = new System.Windows.Forms.Label();
		this.ComboGroup = new System.Windows.Forms.ComboBox();
		this.Rtype = new System.Windows.Forms.ComboBox();
		this.RpriceC = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.RpriceB = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.RpriceA = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Rno = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label3 = new System.Windows.Forms.Label();
		this.Rdtails = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label14 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.SuperTooltip1 = new DevComponents.DotNetBar.SuperTooltip();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.ButtonX_2);
		this.GroupBox1.Controls.Add(this.ButtonX_3);
		this.GroupBox1.Controls.Add(this.ButtonX_4);
		this.GroupBox1.Controls.Add(this.ListViewEx1);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(8, 43);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(469, 425);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_2.Image = (System.Drawing.Image)resources.GetObject("ลบ.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX_2;
		location = new System.Drawing.Point(198, 384);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX_2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX_2.Name = "ลบ";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX_2;
		size = new System.Drawing.Size(87, 28);
		buttonX3.Size = size;
		this.ButtonX_2.TabIndex = 5;
		this.ButtonX_2.Text = "ลบ";
		this.ButtonX_3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_3.Image = (System.Drawing.Image)resources.GetObject("แก\u0e49ไข.Image");
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX_3;
		location = new System.Drawing.Point(105, 384);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX_3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX5.Margin = margin;
		this.ButtonX_3.Name = "แก\u0e49ไข";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX_3;
		size = new System.Drawing.Size(87, 28);
		buttonX6.Size = size;
		this.ButtonX_3.TabIndex = 4;
		this.ButtonX_3.Text = "แก\u0e49ไข";
		this.ButtonX_4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_4.Image = (System.Drawing.Image)resources.GetObject("เพ\u0e34\u0e48ม.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX_4;
		location = new System.Drawing.Point(13, 384);
		buttonX7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX_4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX8.Margin = margin;
		this.ButtonX_4.Name = "เพ\u0e34\u0e48ม";
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX_4;
		size = new System.Drawing.Size(87, 28);
		buttonX9.Size = size;
		this.ButtonX_4.TabIndex = 3;
		this.ButtonX_4.Text = "เพ\u0e34\u0e48ม";
		this.ListViewEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListViewEx1.Atto_กระดาษแนวนอน = true;
		this.ListViewEx1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[11]
		{
			this.ColumnHeader7, this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader8, this.ColumnHeader9, this.ColumnHeader10,
			this.ColumnHeader11
		});
		this.ListViewEx1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListViewEx1.FitToPage = true;
		this.ListViewEx1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListViewEx1.FullRowSelect = true;
		this.ListViewEx1.GridLines = true;
		global::PrintableListView.PrintableListView listViewEx = this.ListViewEx1;
		location = new System.Drawing.Point(13, 23);
		listViewEx.Location = location;
		global::PrintableListView.PrintableListView listViewEx2 = this.ListViewEx1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listViewEx2.Margin = margin;
		this.ListViewEx1.MultiSelect = false;
		this.ListViewEx1.Name = "ListViewEx1";
		global::PrintableListView.PrintableListView listViewEx3 = this.ListViewEx1;
		size = new System.Drawing.Size(438, 352);
		listViewEx3.Size = size;
		this.ListViewEx1.TabIndex = 2;
		this.ListViewEx1.Title = "";
		this.ListViewEx1.Title2 = "";
		this.ListViewEx1.Title2Tab = "";
		this.ListViewEx1.Title3 = "";
		this.ListViewEx1.Title3Tab = "";
		this.ListViewEx1.UseCompatibleStateImageBehavior = false;
		this.ListViewEx1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader7.Width = 0;
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader1.Width = 40;
		this.ColumnHeader6.Text = "เลขห\u0e49อง";
		this.ColumnHeader6.Width = 70;
		this.ColumnHeader2.Text = "ประเภทห\u0e49อง";
		this.ColumnHeader2.Width = 150;
		this.ColumnHeader3.Text = "รายละเอ\u0e35ยดห\u0e49อง";
		this.ColumnHeader3.Width = 150;
		this.ColumnHeader4.Text = "ราคา A";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 0;
		this.ColumnHeader5.Text = "ราคา B";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 0;
		this.ColumnHeader8.Text = "ราคา C";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Width = 0;
		this.ColumnHeader9.Width = 0;
		this.ColumnHeader10.Text = "POWER ON";
		this.ColumnHeader10.Width = 100;
		this.ColumnHeader11.Text = "POWER OFF";
		this.ColumnHeader11.Width = 100;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[3] { this.AddMenu, this.EditMenu, this.DelMenu });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(102, 70);
		contextMenuStrip.Size = size;
		this.AddMenu.Name = "AddMenu";
		System.Windows.Forms.ToolStripMenuItem addMenu = this.AddMenu;
		size = new System.Drawing.Size(101, 22);
		addMenu.Size = size;
		this.AddMenu.Text = "เพ\u0e34\u0e48ม";
		this.EditMenu.Name = "EditMenu";
		System.Windows.Forms.ToolStripMenuItem editMenu = this.EditMenu;
		size = new System.Drawing.Size(101, 22);
		editMenu.Size = size;
		this.EditMenu.Text = "แก\u0e49ไข";
		this.DelMenu.Name = "DelMenu";
		System.Windows.Forms.ToolStripMenuItem delMenu = this.DelMenu;
		size = new System.Drawing.Size(101, 22);
		delMenu.Size = size;
		this.DelMenu.Text = "ลบ";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label = this.Label7;
		location = new System.Drawing.Point(33, 27);
		label.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label2 = this.Label7;
		size = new System.Drawing.Size(0, 16);
		label2.Size = size;
		this.Label7.TabIndex = 11;
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox2.Controls.Add(this.ButtonX4);
		this.GroupBox2.Controls.Add(this.ButtonX3);
		this.GroupBox2.Controls.Add(this.ButtonX2);
		this.GroupBox2.Controls.Add(this.Label13);
		this.GroupBox2.Controls.Add(this.Label11);
		this.GroupBox2.Controls.Add(this.Label12);
		this.GroupBox2.Controls.Add(this.Label10);
		this.GroupBox2.Controls.Add(this.POWER_OFF);
		this.GroupBox2.Controls.Add(this.Label9);
		this.GroupBox2.Controls.Add(this.POWER_ON);
		this.GroupBox2.Controls.Add(this.Label6);
		this.GroupBox2.Controls.Add(this.ComboGroup);
		this.GroupBox2.Controls.Add(this.Rtype);
		this.GroupBox2.Controls.Add(this.RpriceC);
		this.GroupBox2.Controls.Add(this.RpriceB);
		this.GroupBox2.Controls.Add(this.RpriceA);
		this.GroupBox2.Controls.Add(this.Rno);
		this.GroupBox2.Controls.Add(this.Label3);
		this.GroupBox2.Controls.Add(this.Rdtails);
		this.GroupBox2.Controls.Add(this.Label2);
		this.GroupBox2.Controls.Add(this.Label14);
		this.GroupBox2.Controls.Add(this.Label4);
		this.GroupBox2.Controls.Add(this.Label1);
		this.GroupBox2.Controls.Add(this.Label8);
		this.GroupBox2.Controls.Add(this.Label5);
		this.GroupBox2.Controls.Add(this.ButtonX_0);
		this.GroupBox2.Controls.Add(this.ButtonX1);
		this.GroupBox2.Controls.Add(this.ButtonX_1);
		this.GroupBox2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox5 = this.GroupBox2;
		location = new System.Drawing.Point(483, 43);
		groupBox5.Location = location;
		System.Windows.Forms.GroupBox groupBox6 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox6.Margin = margin;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox7 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox7.Padding = margin;
		System.Windows.Forms.GroupBox groupBox8 = this.GroupBox2;
		size = new System.Drawing.Size(404, 425);
		groupBox8.Size = size;
		this.GroupBox2.TabIndex = 1;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไข";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Enabled = false;
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX2;
		location = new System.Drawing.Point(21, 348);
		buttonX10.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX11.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX2;
		size = new System.Drawing.Size(179, 28);
		buttonX12.Size = size;
		this.ButtonX2.TabIndex = 20;
		this.ButtonX2.Text = "นำขนาดไปใช\u0e49ก\u0e31บท\u0e38กห\u0e49อง";
		this.Label13.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label3 = this.Label13;
		location = new System.Drawing.Point(203, 154);
		label3.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label4 = this.Label13;
		size = new System.Drawing.Size(197, 51);
		label4.Size = size;
		this.Label13.TabIndex = 19;
		this.Label13.Text = "* ขนาน ม\u0e35ผลต\u0e48อหน\u0e49าแสดงรวมห\u0e49องพ\u0e31ก ถ\u0e49าอยากให\u0e49 ร\u0e39ปกว\u0e49างข\u0e36\u0e49น/ยาวข\u0e36\u0e49น ให\u0e49เพ\u0e34\u0e48มต\u0e31วเลขไป ต\u0e31\u0e49งแต\u0e48 1-10";
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label11;
		location = new System.Drawing.Point(174, 185);
		label5.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label6 = this.Label11;
		size = new System.Drawing.Size(27, 16);
		label6.Size = size;
		this.Label11.TabIndex = 18;
		this.Label11.Text = "เท\u0e48า";
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label12;
		location = new System.Drawing.Point(20, 185);
		label7.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label8 = this.Label12;
		size = new System.Drawing.Size(100, 16);
		label8.Size = size;
		this.Label12.TabIndex = 17;
		this.Label12.Text = "ห\u0e49องกว\u0e49างกว\u0e48าปกต\u0e34";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label10;
		location = new System.Drawing.Point(174, 157);
		label9.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label10 = this.Label10;
		size = new System.Drawing.Size(27, 16);
		label10.Size = size;
		this.Label10.TabIndex = 16;
		this.Label10.Text = "เท\u0e48า";
		this.POWER_OFF.Border.Class = "TextBoxBorder";
		this.POWER_OFF.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX pOWER_OFF = this.POWER_OFF;
		location = new System.Drawing.Point(122, 243);
		pOWER_OFF.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX pOWER_OFF2 = this.POWER_OFF;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		pOWER_OFF2.Margin = margin;
		this.POWER_OFF.MaxLength = 255;
		this.POWER_OFF.Name = "POWER_OFF";
		DevComponents.DotNetBar.Controls.TextBoxX pOWER_OFF3 = this.POWER_OFF;
		size = new System.Drawing.Size(265, 23);
		pOWER_OFF3.Size = size;
		this.POWER_OFF.TabIndex = 14;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label9;
		location = new System.Drawing.Point(24, 247);
		label11.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label12 = this.Label9;
		size = new System.Drawing.Size(96, 16);
		label12.Size = size;
		this.Label9.TabIndex = 15;
		this.Label9.Text = "CODE การป\u0e34ดไฟ";
		this.POWER_ON.Border.Class = "TextBoxBorder";
		this.POWER_ON.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX pOWER_ON = this.POWER_ON;
		location = new System.Drawing.Point(122, 212);
		pOWER_ON.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX pOWER_ON2 = this.POWER_ON;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		pOWER_ON2.Margin = margin;
		this.POWER_ON.MaxLength = 255;
		this.POWER_ON.Name = "POWER_ON";
		DevComponents.DotNetBar.Controls.TextBoxX pOWER_ON3 = this.POWER_ON;
		size = new System.Drawing.Size(265, 23);
		pOWER_ON3.Size = size;
		this.POWER_ON.TabIndex = 12;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label6;
		location = new System.Drawing.Point(20, 216);
		label13.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label14 = this.Label6;
		size = new System.Drawing.Size(100, 16);
		label14.Size = size;
		this.Label6.TabIndex = 13;
		this.Label6.Text = "CODE การเป\u0e34ดไฟ";
		this.ComboGroup.Enabled = false;
		this.ComboGroup.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboGroup = this.ComboGroup;
		location = new System.Drawing.Point(123, 121);
		comboGroup.Location = location;
		System.Windows.Forms.ComboBox comboGroup2 = this.ComboGroup;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		comboGroup2.Margin = margin;
		this.ComboGroup.Name = "ComboGroup";
		System.Windows.Forms.ComboBox comboGroup3 = this.ComboGroup;
		size = new System.Drawing.Size(264, 24);
		comboGroup3.Size = size;
		this.ComboGroup.TabIndex = 1;
		this.Rtype.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Rtype.Enabled = false;
		this.Rtype.FormattingEnabled = true;
		System.Windows.Forms.ComboBox rtype = this.Rtype;
		location = new System.Drawing.Point(123, 55);
		rtype.Location = location;
		System.Windows.Forms.ComboBox rtype2 = this.Rtype;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		rtype2.Margin = margin;
		this.Rtype.Name = "Rtype";
		System.Windows.Forms.ComboBox rtype3 = this.Rtype;
		size = new System.Drawing.Size(264, 24);
		rtype3.Size = size;
		this.Rtype.TabIndex = 1;
		this.RpriceC.BackColor = System.Drawing.Color.White;
		this.RpriceC.Border.Class = "TextBoxBorder";
		this.RpriceC.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceC = this.RpriceC;
		location = new System.Drawing.Point(122, 153);
		rpriceC.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceC2 = this.RpriceC;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		rpriceC2.Margin = margin;
		this.RpriceC.Name = "RpriceC";
		DevComponents.DotNetBar.Controls.TextBoxX rpriceC3 = this.RpriceC;
		size = new System.Drawing.Size(49, 23);
		rpriceC3.Size = size;
		this.RpriceC.TabIndex = 5;
		this.RpriceB.BackColor = System.Drawing.Color.White;
		this.RpriceB.Border.Class = "TextBoxBorder";
		this.RpriceB.Enabled = false;
		this.RpriceB.FocusHighlightColor = System.Drawing.Color.White;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceB = this.RpriceB;
		location = new System.Drawing.Point(122, 182);
		rpriceB.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceB2 = this.RpriceB;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		rpriceB2.Margin = margin;
		this.RpriceB.Name = "RpriceB";
		DevComponents.DotNetBar.Controls.TextBoxX rpriceB3 = this.RpriceB;
		size = new System.Drawing.Size(49, 23);
		rpriceB3.Size = size;
		this.RpriceB.TabIndex = 4;
		this.RpriceA.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.RpriceA.Border.Class = "TextBoxBorder";
		this.RpriceA.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceA = this.RpriceA;
		location = new System.Drawing.Point(123, 121);
		rpriceA.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceA2 = this.RpriceA;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		rpriceA2.Margin = margin;
		this.RpriceA.Name = "RpriceA";
		this.RpriceA.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceA3 = this.RpriceA;
		size = new System.Drawing.Size(90, 23);
		rpriceA3.Size = size;
		this.RpriceA.TabIndex = 3;
		this.RpriceA.Visible = false;
		this.Rno.BackColor = System.Drawing.Color.White;
		this.Rno.Border.Class = "TextBoxBorder";
		this.Rno.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rno = this.Rno;
		location = new System.Drawing.Point(123, 23);
		rno.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX rno2 = this.Rno;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		rno2.Margin = margin;
		this.Rno.MaxLength = 255;
		this.Rno.Name = "Rno";
		DevComponents.DotNetBar.Controls.TextBoxX rno3 = this.Rno;
		size = new System.Drawing.Size(103, 23);
		rno3.Size = size;
		this.Rno.TabIndex = 0;
		this.Rno.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label3;
		location = new System.Drawing.Point(26, 156);
		label15.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label16 = this.Label3;
		size = new System.Drawing.Size(94, 16);
		label16.Size = size;
		this.Label3.TabIndex = 11;
		this.Label3.Text = "ห\u0e49องยาวกว\u0e48าปกต\u0e34";
		this.Rdtails.Border.Class = "TextBoxBorder";
		this.Rdtails.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rdtails = this.Rdtails;
		location = new System.Drawing.Point(123, 89);
		rdtails.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX rdtails2 = this.Rdtails;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		rdtails2.Margin = margin;
		this.Rdtails.MaxLength = 255;
		this.Rdtails.Name = "Rdtails";
		DevComponents.DotNetBar.Controls.TextBoxX rdtails3 = this.Rdtails;
		size = new System.Drawing.Size(265, 23);
		rdtails3.Size = size;
		this.Rdtails.TabIndex = 2;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label2;
		location = new System.Drawing.Point(67, 289);
		label17.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label18 = this.Label2;
		size = new System.Drawing.Size(45, 16);
		label18.Size = size;
		this.Label2.TabIndex = 11;
		this.Label2.Text = "ราคา B";
		this.Label2.Visible = false;
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label14;
		location = new System.Drawing.Point(67, 289);
		label19.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label20 = this.Label14;
		size = new System.Drawing.Size(46, 16);
		label20.Size = size;
		this.Label14.TabIndex = 11;
		this.Label14.Text = "ราคา A";
		this.Label14.Visible = false;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label4;
		location = new System.Drawing.Point(29, 93);
		label21.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label22 = this.Label4;
		size = new System.Drawing.Size(92, 16);
		label22.Size = size;
		this.Label4.TabIndex = 11;
		this.Label4.Text = "รายละเอ\u0e35ยดห\u0e49อง";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label1;
		location = new System.Drawing.Point(50, 125);
		label23.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label24 = this.Label1;
		size = new System.Drawing.Size(71, 16);
		label24.Size = size;
		this.Label1.TabIndex = 11;
		this.Label1.Text = "กล\u0e38\u0e48มห\u0e49องพ\u0e31ก";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label8;
		location = new System.Drawing.Point(71, 27);
		label25.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label26 = this.Label8;
		size = new System.Drawing.Size(50, 16);
		label26.Size = size;
		this.Label8.TabIndex = 11;
		this.Label8.Text = "เลขห\u0e49อง";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label5;
		location = new System.Drawing.Point(50, 59);
		label27.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label28 = this.Label5;
		size = new System.Drawing.Size(71, 16);
		label28.Size = size;
		this.Label5.TabIndex = 11;
		this.Label5.Text = "ประเภทห\u0e49อง";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.Enabled = false;
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ยกเล\u0e34ก.Image");
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX_0;
		location = new System.Drawing.Point(302, 384);
		buttonX13.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX14.Margin = margin;
		this.ButtonX_0.Name = "ยกเล\u0e34ก";
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX_0;
		size = new System.Drawing.Size(87, 28);
		buttonX15.Size = size;
		this.ButtonX_0.TabIndex = 7;
		this.ButtonX_0.Text = "ยกเล\u0e34ก";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Enabled = false;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX1;
		location = new System.Drawing.Point(21, 384);
		buttonX16.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX17.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX18 = this.ButtonX1;
		size = new System.Drawing.Size(179, 28);
		buttonX18.Size = size;
		this.SuperTooltip1.SetSuperTooltip(this.ButtonX1, new DevComponents.DotNetBar.SuperTooltipInfo("การ Copy จาก Excel", "iHOTEL", "ให\u0e49ทำการสร\u0e49างช\u0e37\u0e48อห\u0e49องจากเซล\u0e4cใดเซล\u0e4cหน\u0e36\u0e48งโดยการเร\u0e35ยงแถวเลขห\u0e49องลงมาเร\u0e37\u0e48อยๆ จากน\u0e31\u0e49น ให\u0e49เล\u0e37อกรายการเซล\u0e4cท\u0e35\u0e48ต\u0e49องการท\u0e31\u0e49งหมด แล\u0e49วคล\u0e34\u0e4aกขวาเล\u0e37อกค\u0e31ดลอกจากน\u0e31\u0e49นให\u0e49มากดท\u0e35\u0e48ป\u0e38\u0e48มน\u0e35\u0e49", null, null, DevComponents.DotNetBar.eTooltipColor.Apple));
		this.ButtonX1.TabIndex = 6;
		this.ButtonX1.Text = "ด\u0e36งจาก Clipboard";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_1.Enabled = false;
		this.ButtonX_1.Image = (System.Drawing.Image)resources.GetObject("บ\u0e31นท\u0e36ก.Image");
		DevComponents.DotNetBar.ButtonX buttonX19 = this.ButtonX_1;
		location = new System.Drawing.Point(207, 384);
		buttonX19.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX20 = this.ButtonX_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX20.Margin = margin;
		this.ButtonX_1.Name = "บ\u0e31นท\u0e36ก";
		DevComponents.DotNetBar.ButtonX buttonX21 = this.ButtonX_1;
		size = new System.Drawing.Size(87, 28);
		buttonX21.Size = size;
		this.ButtonX_1.TabIndex = 6;
		this.ButtonX_1.Text = "บ\u0e31นท\u0e36ก";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		size = new System.Drawing.Size(899, 39);
		panelEx3.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Red;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Maroon;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "จ\u0e31ดการห\u0e49องพ\u0e31ก";
		this.Timer2.Enabled = true;
		this.OpenFileDialog1.RestoreDirectory = true;
		this.SuperTooltip1.DefaultFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.SuperTooltip1.TooltipDuration = 30;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX22 = this.ButtonX3;
		location = new System.Drawing.Point(122, 273);
		buttonX22.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX23 = this.ButtonX3;
		size = new System.Drawing.Size(125, 23);
		buttonX23.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 21;
		this.ButtonX3.Text = "ทดสอบเป\u0e34ดไฟ";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX24 = this.ButtonX4;
		location = new System.Drawing.Point(261, 273);
		buttonX24.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX25 = this.ButtonX4;
		size = new System.Drawing.Size(125, 23);
		buttonX25.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 22;
		this.ButtonX4.Text = "ทดสอบป\u0e34ดไฟ";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(899, 480);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox1);
		this.Controls.Add(this.GroupBox2);
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmManageRoom";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "จ\u0e31ดการห\u0e49องพ\u0e31ก";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmManageRoom_Load(object sender, EventArgs e)
	{
		Listtype();
		ListGroup();
		Search();
	}

	public void Search()
	{
		cancel();
		DataSet dataSet = Module1.connect("select * from HT_Rooms order by Room_no");
		ListViewEx1.Items.Clear();
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
					global::PrintableListView.PrintableListView listViewEx = ListViewEx1;
					ListView.ListViewItemCollection items = listViewEx.Items;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow2 = dataRow;
					string columnName = "id";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					listViewEx.Items[num2].SubItems.Add(Conversions.ToString(num2 + 1));
					ListViewItem.ListViewSubItemCollection subItems = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow3 = dataRow;
					columnName = "Room_no";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "Room_Type";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems3 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array7 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow5 = dataRow;
					columnName = "Room_Details";
					array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
					array = array3;
					object[] arguments4 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems4 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array8 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow6 = dataRow;
					columnName = "Room_PriceA";
					array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
					array = array3;
					object[] arguments5 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems4, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems5 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array9 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow7 = dataRow;
					columnName = "Room_PriceB";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array = array3;
					object[] arguments6 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems5, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems6 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array10 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow8 = dataRow;
					columnName = "Room_PriceC";
					array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
					array = array3;
					object[] arguments7 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems6, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Room_Group"].ToString());
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Room_Power_OPEN"].ToString());
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Room_Power_CLOSE"].ToString());
					listViewEx = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void ListGroup()
	{
		DataSet dataSet = Module1.connect("select Room_Group from HT_Rooms where Room_Group is not null and Room_Group<>'' group by Room_Group");
		ComboGroup.DataSource = dataSet.Tables[0];
		ComboGroup.DisplayMember = "Room_Group";
	}

	public void Listtype()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_RoomType order by name");
		Rtype.DataSource = dataSet.Tables[0];
		Rtype.DisplayMember = "name";
	}

	private void ListViewEx1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count != 0)
		{
			cancel();
			ButtonX_4.Enabled = true;
			ButtonX_3.Enabled = true;
			ButtonX_2.Enabled = true;
			Rno.Text = ListViewEx1.SelectedItems[0].SubItems[2].Text;
			Rtype.Text = ListViewEx1.SelectedItems[0].SubItems[3].Text;
			Rdtails.Text = ListViewEx1.SelectedItems[0].SubItems[4].Text;
			RpriceA.Text = ListViewEx1.SelectedItems[0].SubItems[5].Text;
			RpriceB.Text = ListViewEx1.SelectedItems[0].SubItems[6].Text;
			RpriceC.Text = ListViewEx1.SelectedItems[0].SubItems[7].Text;
			ComboGroup.Text = ListViewEx1.SelectedItems[0].SubItems[8].Text;
			POWER_ON.Text = ListViewEx1.SelectedItems[0].SubItems[9].Text;
			POWER_OFF.Text = ListViewEx1.SelectedItems[0].SubItems[10].Text;
		}
	}

	private void ButtonX_4_Click(object sender, EventArgs e)
	{
		cancel();
		Rno.Enabled = true;
		Rtype.Enabled = true;
		Rdtails.Enabled = true;
		RpriceA.Enabled = true;
		RpriceB.Enabled = true;
		RpriceC.Enabled = true;
		ComboGroup.Enabled = true;
		POWER_OFF.Enabled = true;
		POWER_ON.Enabled = true;
		ButtonX_1.Enabled = true;
		ButtonX_0.Enabled = true;
		ButtonX1.Enabled = true;
		ButtonX_4.Enabled = false;
		ButtonX_3.Enabled = false;
		ButtonX_2.Enabled = false;
		Rno.Focus();
	}

	private void ButtonX_3_Click(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการห\u0e49อง");
			return;
		}
		cancel();
		ComboGroup.Enabled = true;
		ButtonX2.Enabled = true;
		Rno.Enabled = true;
		Rtype.Enabled = true;
		Rdtails.Enabled = true;
		RpriceA.Enabled = true;
		RpriceB.Enabled = true;
		RpriceC.Enabled = true;
		ButtonX_1.Enabled = true;
		ButtonX_0.Enabled = true;
		ButtonX_4.Enabled = false;
		ButtonX_3.Enabled = false;
		ButtonX_2.Enabled = false;
		POWER_OFF.Enabled = true;
		POWER_ON.Enabled = true;
		EditID = ListViewEx1.SelectedItems[0].SubItems[0].Text;
		Rno.Text = ListViewEx1.SelectedItems[0].SubItems[2].Text;
		Rtype.Text = ListViewEx1.SelectedItems[0].SubItems[3].Text;
		Rdtails.Text = ListViewEx1.SelectedItems[0].SubItems[4].Text;
		RpriceA.Text = ListViewEx1.SelectedItems[0].SubItems[5].Text;
		RpriceB.Text = ListViewEx1.SelectedItems[0].SubItems[6].Text;
		RpriceC.Text = ListViewEx1.SelectedItems[0].SubItems[7].Text;
		POWER_ON.Text = ListViewEx1.SelectedItems[0].SubItems[9].Text;
		POWER_OFF.Text = ListViewEx1.SelectedItems[0].SubItems[10].Text;
		Rno.Focus();
	}

	private void ButtonX_0_Click(object sender, EventArgs e)
	{
		cancel();
	}

	public void cancel()
	{
		Rno.Text = "";
		Rtype.SelectedIndex = 0;
		Rdtails.Text = "";
		RpriceA.Text = "";
		RpriceB.Text = "1";
		RpriceC.Text = "1";
		EditID = "";
		ButtonX2.Enabled = false;
		ComboGroup.Enabled = false;
		Rno.Enabled = false;
		Rtype.Enabled = false;
		Rdtails.Enabled = false;
		RpriceA.Enabled = false;
		RpriceB.Enabled = false;
		RpriceC.Enabled = false;
		ButtonX_1.Enabled = false;
		ButtonX_0.Enabled = false;
		ButtonX1.Enabled = false;
		POWER_OFF.Enabled = false;
		POWER_ON.Enabled = false;
		ButtonX_4.Enabled = true;
		ButtonX_3.Enabled = true;
		ButtonX_2.Enabled = true;
	}

	private void ButtonX_1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Rno.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขห\u0e49อง");
		}
		else if (Rno.Text.IndexOf(" ") != -1)
		{
			MessageBox.Show("เลขห\u0e49องไม\u0e48สามารถใช\u0e49 เว\u0e49นวรรคได\u0e49");
		}
		else if (Rno.Text.IndexOf("'") != -1)
		{
			MessageBox.Show("เลขห\u0e49องไม\u0e48สามารถใช\u0e49 เคร\u0e37\u0e48องหมาย ' ได\u0e49");
		}
		else if (Operators.CompareString(ComboGroup.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48กล\u0e38\u0e48มห\u0e49องพ\u0e31ก");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			if (Operators.CompareString(RpriceA.Text, "", TextCompare: false) == 0)
			{
				RpriceA.Text = Conversions.ToString(0);
			}
			if (!Versioned.IsNumeric(RpriceA.Text))
			{
				RpriceA.Text = Conversions.ToString(0);
			}
			if (Operators.CompareString(RpriceB.Text, "", TextCompare: false) == 0)
			{
				RpriceB.Text = Conversions.ToString(1);
			}
			if (!Versioned.IsNumeric(RpriceB.Text))
			{
				RpriceB.Text = Conversions.ToString(1);
			}
			if (Operators.CompareString(RpriceC.Text, "", TextCompare: false) == 0)
			{
				RpriceC.Text = Conversions.ToString(1);
			}
			if (!Versioned.IsNumeric(RpriceC.Text))
			{
				RpriceC.Text = Conversions.ToString(1);
			}
			if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
			{
				object right = Module1.get_id("HT_Rooms", "id");
				object left = "INSERT INTO [HT_Rooms]";
				left = Operators.ConcatenateObject(left, "([id]");
				left = Operators.ConcatenateObject(left, ",[Room_no]");
				left = Operators.ConcatenateObject(left, ",[Room_Type]");
				left = Operators.ConcatenateObject(left, ",[Room_Details]");
				left = Operators.ConcatenateObject(left, ",[Room_PriceA]");
				left = Operators.ConcatenateObject(left, ",[Room_PriceB]");
				left = Operators.ConcatenateObject(left, ",[Room_PriceC],[Room_Book],[Room_Group],[Room_Power_OPEN],[Room_Power_CLOSE],[Room_Power_STATUS])");
				left = Operators.ConcatenateObject(left, "VALUES");
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, right);
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rno.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rtype.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rdtails.Text, "'"));
				left = Operators.ConcatenateObject(left, "," + RpriceA.Text);
				left = Operators.ConcatenateObject(left, "," + RpriceB.Text);
				left = Operators.ConcatenateObject(left, "," + RpriceC.Text);
				left = Operators.ConcatenateObject(left, ",''");
				left = Operators.ConcatenateObject(left, string.Concat(",'" + ComboGroup.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + POWER_ON.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + POWER_OFF.Text, "'"));
				left = Operators.ConcatenateObject(left, ",'off'");
				left = Operators.ConcatenateObject(left, ")");
				Module1.connect(Conversions.ToString(left));
				ListGroup();
				Search();
			}
			else
			{
				object left2 = "UPDATE [HT_Rooms] SET ";
				left2 = Operators.ConcatenateObject(left2, string.Concat("[Room_no]='" + Rno.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Room_Type]='" + Rtype.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Room_Details]='" + Rdtails.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, ",[Room_PriceA]=" + RpriceA.Text);
				left2 = Operators.ConcatenateObject(left2, ",[Room_PriceB]=" + RpriceB.Text);
				left2 = Operators.ConcatenateObject(left2, ",[Room_PriceC]=" + RpriceC.Text);
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Room_Group]='" + ComboGroup.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Room_Power_OPEN]='" + POWER_ON.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Room_Power_CLOSE]='" + POWER_OFF.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(" where id=", EditID));
				Module1.connect(Conversions.ToString(left2));
				ListGroup();
				Search();
			}
		}
	}

	private void ButtonX_2_Click(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการห\u0e49อง");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("delete from HT_Rooms where id=" + ListViewEx1.SelectedItems[0].SubItems[0].Text);
			Search();
		}
	}

	private void Rtype_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (Rtype.SelectedIndex != 0)
		{
			Module1.connect("select * from HT_SET_RoomType where name='" + Rtype.Text + "'");
			RpriceA.Text = Conversions.ToString(0);
			RpriceB.Text = Conversions.ToString(0);
			RpriceC.Text = Conversions.ToString(0);
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		IDataObject dataObject = Clipboard.GetDataObject();
		string text = Conversions.ToString(dataObject.GetData(DataFormats.Text));
		ArrayList arrayList = new ArrayList();
		string[] array = Strings.Split(text, "\r\n");
		string[] array2 = array;
		foreach (string text2 in array2)
		{
			if (Operators.CompareString(text2, "", TextCompare: false) != 0)
			{
				arrayList.Add(text2);
			}
		}
		checked
		{
			int num = arrayList.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				text = ((num2 != 0) ? Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(",", arrayList[num2]))) : Conversions.ToString(arrayList[num2]));
				num2++;
			}
			if (MessageBox.Show("รายการห\u0e49องท\u0e35\u0e48ด\u0e36งมาท\u0e31\u0e49งหมด(" + Rtype.Text + ")\r\n\r\n" + text + "\r\n\r\nถ\u0e39กต\u0e49องหร\u0e37อไม\u0e48", "รายการท\u0e35\u0e48ด\u0e36งมา", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk, MessageBoxDefaultButton.Button1) != DialogResult.Yes)
			{
				return;
			}
			int num5 = arrayList.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				object right = Module1.get_id("HT_Rooms", "id");
				object left = "INSERT INTO [HT_Rooms]";
				left = Operators.ConcatenateObject(left, "([id]");
				left = Operators.ConcatenateObject(left, ",[Room_no]");
				left = Operators.ConcatenateObject(left, ",[Room_Type]");
				left = Operators.ConcatenateObject(left, ",[Room_Details]");
				left = Operators.ConcatenateObject(left, ",[Room_PriceA]");
				left = Operators.ConcatenateObject(left, ",[Room_PriceB]");
				left = Operators.ConcatenateObject(left, ",[Room_PriceC],[Room_Book])");
				left = Operators.ConcatenateObject(left, "VALUES");
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, right);
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", arrayList[num6]), "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rtype.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rdtails.Text, "'"));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(0));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(0));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(0));
				left = Operators.ConcatenateObject(left, ",''");
				left = Operators.ConcatenateObject(left, ")");
				Module1.connect(Conversions.ToString(left));
				num6++;
			}
			Search();
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการนำขนานห\u0e49องไปต\u0e31\u0e49งค\u0e48าก\u0e31บท\u0e38กห\u0e49องหร\u0e37อไม\u0e48", "ต\u0e31\u0e49งค\u0e48าขนาดห\u0e49อง", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk, MessageBoxDefaultButton.Button1) == DialogResult.Yes)
		{
			object left = "UPDATE [HT_Rooms] SET ";
			left = Operators.ConcatenateObject(left, "[Room_PriceB]=" + RpriceB.Text);
			left = Operators.ConcatenateObject(left, ",[Room_PriceC]=" + RpriceC.Text);
			Module1.connect(Conversions.ToString(left));
			MessageBox.Show("ต\u0e31\u0e49งค\u0e48าเสร\u0e47จเร\u0e35ยบร\u0e49อย");
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		Module1.Power_set(Rno.Text, "ON", POWER_ON.Text, "ทดสอบเป\u0e34ดไฟจากหน\u0e49าจ\u0e31ดการห\u0e49องพ\u0e31ก");
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		Module1.Power_set(Rno.Text, "OFF", POWER_OFF.Text, "ทดสอบป\u0e34ดไฟจากหน\u0e49าจ\u0e31ดการห\u0e49องพ\u0e31ก");
	}
}
