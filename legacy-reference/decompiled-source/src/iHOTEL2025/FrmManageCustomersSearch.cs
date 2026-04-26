using System;
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
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmManageCustomersSearch : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

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

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("AddMenu")]
	private ToolStripMenuItem _AddMenu;

	[AccessedThroughProperty("EditMenu")]
	private ToolStripMenuItem _EditMenu;

	[AccessedThroughProperty("DelMenu")]
	private ToolStripMenuItem _DelMenu;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ColumnHeader17")]
	private ColumnHeader _ColumnHeader17;

	[AccessedThroughProperty("ColumnHeader18")]
	private ColumnHeader _ColumnHeader18;

	[AccessedThroughProperty("ColumnHeader19")]
	private ColumnHeader _ColumnHeader19;

	[AccessedThroughProperty("ColumnHeader20")]
	private ColumnHeader _ColumnHeader20;

	[AccessedThroughProperty("ColumnHeader21")]
	private ColumnHeader _ColumnHeader21;

	[AccessedThroughProperty("ColumnHeader22")]
	private ColumnHeader _ColumnHeader22;

	[AccessedThroughProperty("ColumnHeader23")]
	private ColumnHeader _ColumnHeader23;

	[AccessedThroughProperty("ColumnHeader24")]
	private ColumnHeader _ColumnHeader24;

	[AccessedThroughProperty("ColumnHeader25")]
	private ColumnHeader _ColumnHeader25;

	[AccessedThroughProperty("ColumnHeader26")]
	private ColumnHeader _ColumnHeader26;

	[AccessedThroughProperty("ColumnHeader27")]
	private ColumnHeader _ColumnHeader27;

	[AccessedThroughProperty("ColumnHeader28")]
	private ColumnHeader _ColumnHeader28;

	[AccessedThroughProperty("ColumnHeader29")]
	private ColumnHeader _ColumnHeader29;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("TextBoxX1")]
	private TextBoxX _TextBoxX1;

	[AccessedThroughProperty("TextBoxX2")]
	private TextBoxX _TextBoxX2;

	[AccessedThroughProperty("Label29")]
	private Label _Label29;

	[AccessedThroughProperty("Label30")]
	private Label _Label30;

	[AccessedThroughProperty("ColumnHeader30")]
	private ColumnHeader _ColumnHeader30;

	[AccessedThroughProperty("ColumnHeader31")]
	private ColumnHeader _ColumnHeader31;

	[AccessedThroughProperty("ColumnHeader32")]
	private ColumnHeader _ColumnHeader32;

	[AccessedThroughProperty("ColumnHeader33")]
	private ColumnHeader _ColumnHeader33;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TextBoxX3")]
	private TextBoxX _TextBoxX3;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

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
			EventHandler value2 = ListViewEx1_DoubleClick;
			if (_ListViewEx1 != null)
			{
				_ListViewEx1.DoubleClick -= value2;
			}
			_ListViewEx1 = value;
			if (_ListViewEx1 != null)
			{
				_ListViewEx1.DoubleClick += value2;
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

	internal virtual ColumnHeader ColumnHeader12
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader12 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader13
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader13 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader14
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader14 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader15
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader15 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader16
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader16 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader17
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader17 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader18
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader18 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader19
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader19 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader20
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader20 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader21
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader21 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader22
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader22 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader23
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader23 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader24
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader24 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader25
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader25;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader25 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader26
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader26;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader26 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader27
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader27;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader27 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader28
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader28;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader28 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader29
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader29;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader29 = value;
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

	internal virtual TextBoxX TextBoxX_0
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
			EventHandler value2 = TextBoxX1_TextChanged;
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

	internal virtual TextBoxX TextBoxX_1
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
			EventHandler value2 = TextBoxX2_TextChanged;
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

	internal virtual Label Label29
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label29;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label29 = value;
		}
	}

	internal virtual Label Label30
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label30;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label30 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader30
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader30;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader30 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader31
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader31;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader31 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader32
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader32;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader32 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader33
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader33;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader33 = value;
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

	internal virtual ComboBox ComboBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox1_SelectedIndexChanged;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged -= value2;
			}
			_ComboBox1 = value;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmManageCustomersSearch()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmManageCustomersSearch()
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
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.ListViewEx1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader30 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader31 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader32 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader17 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader18 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader19 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader20 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader21 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader22 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader23 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader24 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader25 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader26 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader27 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader28 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader29 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader33 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.AddMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.EditMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.DelMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.TextBoxX_1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label29 = new System.Windows.Forms.Label();
		this.Label30 = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.Label1 = new System.Windows.Forms.Label();
		this.TextBoxX_2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label2 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.ListViewEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListViewEx1.Atto_กระดาษแนวนอน = true;
		this.ListViewEx1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[33]
		{
			this.ColumnHeader7, this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader30, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader31, this.ColumnHeader32,
			this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader11, this.ColumnHeader12, this.ColumnHeader13, this.ColumnHeader14, this.ColumnHeader15, this.ColumnHeader16, this.ColumnHeader17, this.ColumnHeader18,
			this.ColumnHeader19, this.ColumnHeader20, this.ColumnHeader21, this.ColumnHeader22, this.ColumnHeader23, this.ColumnHeader24, this.ColumnHeader25, this.ColumnHeader26, this.ColumnHeader27, this.ColumnHeader28,
			this.ColumnHeader29, this.ColumnHeader8, this.ColumnHeader33
		});
		this.ListViewEx1.FitToPage = true;
		this.ListViewEx1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListViewEx1.FullRowSelect = true;
		this.ListViewEx1.GridLines = true;
		global::PrintableListView.PrintableListView listViewEx = this.ListViewEx1;
		System.Drawing.Point location = new System.Drawing.Point(14, 38);
		listViewEx.Location = location;
		global::PrintableListView.PrintableListView listViewEx2 = this.ListViewEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listViewEx2.Margin = margin;
		this.ListViewEx1.MultiSelect = false;
		this.ListViewEx1.Name = "ListViewEx1";
		global::PrintableListView.PrintableListView listViewEx3 = this.ListViewEx1;
		System.Drawing.Size size = new System.Drawing.Size(1150, 395);
		listViewEx3.Size = size;
		this.ListViewEx1.TabIndex = 0;
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
		this.ColumnHeader6.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.ColumnHeader6.Width = 70;
		this.ColumnHeader30.Text = "คำนำหน\u0e49า";
		this.ColumnHeader30.Width = 80;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader3.Text = "นามสก\u0e38ล";
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader4.Text = "ราคาท\u0e35\u0e48ใช\u0e49";
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader5.Text = "อ\u0e35เมลล\u0e4c";
		this.ColumnHeader5.Width = 100;
		this.ColumnHeader31.Text = "เพศ";
		this.ColumnHeader32.Text = "เลขประจำต\u0e31ว";
		this.ColumnHeader32.Width = 100;
		this.ColumnHeader9.Text = "ท\u0e35\u0e48อย\u0e39\u0e48 เลขท\u0e35\u0e48";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader9.Width = 100;
		this.ColumnHeader10.Text = "หม\u0e39\u0e48";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader10.Width = 40;
		this.ColumnHeader11.Text = "ซอย";
		this.ColumnHeader11.Width = 100;
		this.ColumnHeader12.Text = "ถนน";
		this.ColumnHeader12.Width = 100;
		this.ColumnHeader13.Text = "ตำบล";
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader14.Text = "อำเภอ";
		this.ColumnHeader14.Width = 100;
		this.ColumnHeader15.Text = "จ\u0e31งหว\u0e31ด";
		this.ColumnHeader15.Width = 100;
		this.ColumnHeader16.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.ColumnHeader16.Width = 100;
		this.ColumnHeader17.Text = "โทร";
		this.ColumnHeader17.Width = 100;
		this.ColumnHeader18.Text = "Fax";
		this.ColumnHeader18.Width = 100;
		this.ColumnHeader19.Text = "ท\u0e35\u0e48ทำงาน ช\u0e37\u0e48อ";
		this.ColumnHeader19.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader19.Width = 150;
		this.ColumnHeader20.Text = "เลขท\u0e35\u0e48";
		this.ColumnHeader20.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader21.Text = "หม\u0e39\u0e48";
		this.ColumnHeader21.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader21.Width = 40;
		this.ColumnHeader22.Text = "ซอย";
		this.ColumnHeader22.Width = 100;
		this.ColumnHeader23.Text = "ถนน";
		this.ColumnHeader23.Width = 100;
		this.ColumnHeader24.Text = "ตำบล";
		this.ColumnHeader24.Width = 100;
		this.ColumnHeader25.Text = "อำเภอ";
		this.ColumnHeader25.Width = 100;
		this.ColumnHeader26.Text = "จ\u0e31งหว\u0e31ด";
		this.ColumnHeader26.Width = 100;
		this.ColumnHeader27.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.ColumnHeader27.Width = 100;
		this.ColumnHeader28.Text = "โทร";
		this.ColumnHeader28.Width = 100;
		this.ColumnHeader29.Text = "Fax";
		this.ColumnHeader29.Width = 100;
		this.ColumnHeader8.Text = "ประเภทล\u0e39กค\u0e49า";
		this.ColumnHeader33.Text = "ยอดเง\u0e34นเก\u0e34น";
		this.ColumnHeader33.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader33.Width = 100;
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
		this.TextBoxX_0.BackColor = System.Drawing.Color.White;
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_0;
		location = new System.Drawing.Point(77, 7);
		textBoxX_.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBoxX_2.Margin = margin;
		this.TextBoxX_0.MaxLength = 255;
		this.TextBoxX_0.Name = "TextBoxX1";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_3 = this.TextBoxX_0;
		size = new System.Drawing.Size(103, 23);
		textBoxX_3.Size = size;
		this.TextBoxX_0.TabIndex = 0;
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.TextBoxX_1.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_4 = this.TextBoxX_1;
		location = new System.Drawing.Point(253, 7);
		textBoxX_4.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_5 = this.TextBoxX_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBoxX_5.Margin = margin;
		this.TextBoxX_1.MaxLength = 255;
		this.TextBoxX_1.Name = "TextBoxX2";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_6 = this.TextBoxX_1;
		size = new System.Drawing.Size(198, 23);
		textBoxX_6.Size = size;
		this.TextBoxX_1.TabIndex = 1;
		this.Label29.AutoSize = true;
		System.Windows.Forms.Label label = this.Label29;
		location = new System.Drawing.Point(14, 12);
		label.Location = location;
		this.Label29.Name = "Label29";
		System.Windows.Forms.Label label2 = this.Label29;
		size = new System.Drawing.Size(60, 16);
		label2.Size = size;
		this.Label29.TabIndex = 11;
		this.Label29.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label30.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label30;
		location = new System.Drawing.Point(196, 10);
		label3.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label4 = this.Label30;
		size = new System.Drawing.Size(54, 16);
		label4.Size = size;
		this.Label30.TabIndex = 11;
		this.Label30.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.Timer2.Enabled = true;
		this.OpenFileDialog1.RestoreDirectory = true;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(452, 11);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(68, 16);
		label6.Size = size;
		this.Label1.TabIndex = 13;
		this.Label1.Text = "ช\u0e37\u0e48อท\u0e35\u0e48ทำงาน";
		this.TextBoxX_2.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_7 = this.TextBoxX_2;
		location = new System.Drawing.Point(522, 8);
		textBoxX_7.Location = location;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_8 = this.TextBoxX_2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBoxX_8.Margin = margin;
		this.TextBoxX_2.MaxLength = 255;
		this.TextBoxX_2.Name = "TextBoxX3";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_9 = this.TextBoxX_2;
		size = new System.Drawing.Size(209, 23);
		textBoxX_9.Size = size;
		this.TextBoxX_2.TabIndex = 12;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label2;
		location = new System.Drawing.Point(737, 11);
		label7.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label8 = this.Label2;
		size = new System.Drawing.Size(54, 16);
		label8.Size = size;
		this.Label2.TabIndex = 14;
		this.Label2.Text = "เร\u0e35ยงตาม";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "ช\u0e37\u0e48อ", "นามสก\u0e38ล", "รห\u0e31ส" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(797, 8);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(165, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 15;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1177, 448);
		this.ClientSize = size;
		this.Controls.Add(this.ComboBox1);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.TextBoxX_2);
		this.Controls.Add(this.ListViewEx1);
		this.Controls.Add(this.TextBoxX_0);
		this.Controls.Add(this.Label29);
		this.Controls.Add(this.Label30);
		this.Controls.Add(this.TextBoxX_1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmManageCustomersSearch";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "ค\u0e49นหาล\u0e39กค\u0e49า";
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void FrmManageRoom_Load(object sender, EventArgs e)
	{
		ComboBox1.SelectedIndex = 0;
		Search();
	}

	public void Search()
	{
		object obj = "select top 100 * from HT_Customers where id<>0";
		if (Operators.CompareString(TextBoxX_0.Text, "", TextCompare: false) != 0)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat(" and Cust_no like '%" + TextBoxX_0.Text, "%' "));
		}
		if (Operators.CompareString(TextBoxX_1.Text, "", TextCompare: false) != 0)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat(string.Concat(string.Concat(" and (Cust_name like '%" + TextBoxX_1.Text, "%' or Cust_name2 like '%"), TextBoxX_1.Text), "%')"));
		}
		if (Operators.CompareString(TextBoxX_2.Text, "", TextCompare: false) != 0)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat(" and Cust_Work_Name like '%" + TextBoxX_2.Text, "%' "));
		}
		if (ComboBox1.SelectedIndex == 0)
		{
			obj = Operators.ConcatenateObject(obj, " order by  Cust_name");
		}
		else if (ComboBox1.SelectedIndex == 1)
		{
			obj = Operators.ConcatenateObject(obj, " order by  Cust_name2");
		}
		else if (ComboBox1.SelectedIndex == 2)
		{
			obj = Operators.ConcatenateObject(obj, " order by  Cust_no");
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(obj));
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
					columnName = "Cust_no";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_perfix"].ToString());
					ListViewItem.ListViewSubItemCollection subItems2 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "Cust_name";
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
					columnName = "Cust_name2";
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
					columnName = "Cust_Type";
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
					columnName = "Cust_Email";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array = array3;
					object[] arguments6 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems5, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_sex"].ToString());
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_IDcard"].ToString());
					ListViewItem.ListViewSubItemCollection subItems6 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array10 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow8 = dataRow;
					columnName = "Cust_Add_no";
					array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
					array = array3;
					object[] arguments7 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems6, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems7 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array11 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow9 = dataRow;
					columnName = "Cust_Add_moo";
					array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
					array = array3;
					object[] arguments8 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems7, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems8 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array12 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow10 = dataRow;
					columnName = "Cust_Add_soi";
					array12[0] = RuntimeHelpers.GetObjectValue(dataRow10[columnName]);
					array = array3;
					object[] arguments9 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems8, null, "Add", arguments9, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems9 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array13 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow11 = dataRow;
					columnName = "Cust_Add_road";
					array13[0] = RuntimeHelpers.GetObjectValue(dataRow11[columnName]);
					array = array3;
					object[] arguments10 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems9, null, "Add", arguments10, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems10 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array14 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow12 = dataRow;
					columnName = "Cust_Add_tambon";
					array14[0] = RuntimeHelpers.GetObjectValue(dataRow12[columnName]);
					array = array3;
					object[] arguments11 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems10, null, "Add", arguments11, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems11 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array15 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow13 = dataRow;
					columnName = "Cust_Add_ampore";
					array15[0] = RuntimeHelpers.GetObjectValue(dataRow13[columnName]);
					array = array3;
					object[] arguments12 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems11, null, "Add", arguments12, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems12 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array16 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow14 = dataRow;
					columnName = "Cust_Add_province";
					array16[0] = RuntimeHelpers.GetObjectValue(dataRow14[columnName]);
					array = array3;
					object[] arguments13 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems12, null, "Add", arguments13, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems13 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array17 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow15 = dataRow;
					columnName = "Cust_Add_code";
					array17[0] = RuntimeHelpers.GetObjectValue(dataRow15[columnName]);
					array = array3;
					object[] arguments14 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems13, null, "Add", arguments14, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems14 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array18 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow16 = dataRow;
					columnName = "Cust_Add_tel";
					array18[0] = RuntimeHelpers.GetObjectValue(dataRow16[columnName]);
					array = array3;
					object[] arguments15 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems14, null, "Add", arguments15, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems15 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array19 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow17 = dataRow;
					columnName = "Cust_Add_fax";
					array19[0] = RuntimeHelpers.GetObjectValue(dataRow17[columnName]);
					array = array3;
					object[] arguments16 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems15, null, "Add", arguments16, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems16 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array20 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow18 = dataRow;
					columnName = "Cust_Work_Name";
					array20[0] = RuntimeHelpers.GetObjectValue(dataRow18[columnName]);
					array = array3;
					object[] arguments17 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems16, null, "Add", arguments17, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems17 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array21 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow19 = dataRow;
					columnName = "Cust_Work_no";
					array21[0] = RuntimeHelpers.GetObjectValue(dataRow19[columnName]);
					array = array3;
					object[] arguments18 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems17, null, "Add", arguments18, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems18 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array22 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow20 = dataRow;
					columnName = "Cust_Work_moo";
					array22[0] = RuntimeHelpers.GetObjectValue(dataRow20[columnName]);
					array = array3;
					object[] arguments19 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems18, null, "Add", arguments19, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems19 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array23 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow21 = dataRow;
					columnName = "Cust_Work_soi";
					array23[0] = RuntimeHelpers.GetObjectValue(dataRow21[columnName]);
					array = array3;
					object[] arguments20 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems19, null, "Add", arguments20, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems20 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array24 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow22 = dataRow;
					columnName = "Cust_Work_road";
					array24[0] = RuntimeHelpers.GetObjectValue(dataRow22[columnName]);
					array = array3;
					object[] arguments21 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems20, null, "Add", arguments21, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems21 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array25 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow23 = dataRow;
					columnName = "Cust_Work_tambon";
					array25[0] = RuntimeHelpers.GetObjectValue(dataRow23[columnName]);
					array = array3;
					object[] arguments22 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems21, null, "Add", arguments22, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems22 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array26 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow24 = dataRow;
					columnName = "Cust_Work_ampore";
					array26[0] = RuntimeHelpers.GetObjectValue(dataRow24[columnName]);
					array = array3;
					object[] arguments23 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems22, null, "Add", arguments23, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems23 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array27 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow25 = dataRow;
					columnName = "Cust_Work_province";
					array27[0] = RuntimeHelpers.GetObjectValue(dataRow25[columnName]);
					array = array3;
					object[] arguments24 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems23, null, "Add", arguments24, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems24 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array28 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow26 = dataRow;
					columnName = "Cust_Work_code";
					array28[0] = RuntimeHelpers.GetObjectValue(dataRow26[columnName]);
					array = array3;
					object[] arguments25 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems24, null, "Add", arguments25, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems25 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array29 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow27 = dataRow;
					columnName = "Cust_Work_tel";
					array29[0] = RuntimeHelpers.GetObjectValue(dataRow27[columnName]);
					array = array3;
					object[] arguments26 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems25, null, "Add", arguments26, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems26 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array30 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow28 = dataRow;
					columnName = "Cust_Work_fax";
					array30[0] = RuntimeHelpers.GetObjectValue(dataRow28[columnName]);
					array = array3;
					object[] arguments27 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems26, null, "Add", arguments27, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems27 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array31 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow29 = dataRow;
					columnName = "Cust_Type_Main";
					array31[0] = RuntimeHelpers.GetObjectValue(dataRow29[columnName]);
					array = array3;
					object[] arguments28 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems27, null, "Add", arguments28, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems28 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array32 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow30 = dataRow;
					columnName = "Cust_Price_Over";
					array32[0] = RuntimeHelpers.GetObjectValue(dataRow30[columnName]);
					array = array3;
					object[] arguments29 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems28, null, "Add", arguments29, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listViewEx = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void TextBoxX1_TextChanged(object sender, EventArgs e)
	{
		if (TextBoxX_0.TextLength != 0)
		{
			TextBoxX_1.Text = "";
			TextBoxX_2.Text = "";
		}
		if (TextBoxX_0.TextLength >= 2)
		{
			Search();
		}
	}

	private void TextBoxX2_TextChanged(object sender, EventArgs e)
	{
		if (TextBoxX_1.TextLength != 0)
		{
			TextBoxX_0.Text = "";
			TextBoxX_2.Text = "";
		}
		if (TextBoxX_1.TextLength >= 0)
		{
			Search();
		}
	}

	private void ListViewEx1_DoubleClick(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count != 0)
		{
			EditID = ListViewEx1.SelectedItems[0].SubItems[2].Text;
			Close();
		}
	}

	private void TextBoxX3_TextChanged(object sender, EventArgs e)
	{
		if (TextBoxX_2.TextLength != 0)
		{
			TextBoxX_0.Text = "";
			TextBoxX_1.Text = "";
		}
		if (TextBoxX_2.TextLength >= 2)
		{
			Search();
		}
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		Search();
	}
}
