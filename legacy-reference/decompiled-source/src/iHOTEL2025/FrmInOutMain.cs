using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Validator;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmInOutMain : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("TimeMaxzimize")]
	private Timer _TimeMaxzimize;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("RequiredFieldValidator7")]
	private RequiredFieldValidator _RequiredFieldValidator7;

	[AccessedThroughProperty("RequiredFieldValidator6")]
	private RequiredFieldValidator _RequiredFieldValidator6;

	[AccessedThroughProperty("RequiredFieldValidator4")]
	private RequiredFieldValidator _RequiredFieldValidator4;

	[AccessedThroughProperty("RequiredFieldValidator5")]
	private RequiredFieldValidator _RequiredFieldValidator5;

	[AccessedThroughProperty("RequiredFieldValidator3")]
	private RequiredFieldValidator _RequiredFieldValidator3;

	[AccessedThroughProperty("RequiredFieldValidator1")]
	private RequiredFieldValidator _RequiredFieldValidator1;

	[AccessedThroughProperty("CustomValidator4")]
	private CustomValidator _CustomValidator4;

	[AccessedThroughProperty("CustomValidator3")]
	private CustomValidator _CustomValidator3;

	[AccessedThroughProperty("CustomValidator2")]
	private CustomValidator _CustomValidator2;

	[AccessedThroughProperty("RequiredFieldValidator8")]
	private RequiredFieldValidator _RequiredFieldValidator8;

	[AccessedThroughProperty("CustomValidator5")]
	private CustomValidator _CustomValidator5;

	[AccessedThroughProperty("RequiredFieldValidator10")]
	private RequiredFieldValidator _RequiredFieldValidator10;

	[AccessedThroughProperty("RequiredFieldValidator9")]
	private RequiredFieldValidator _RequiredFieldValidator9;

	[AccessedThroughProperty("CustomValidator9")]
	private CustomValidator _CustomValidator9;

	[AccessedThroughProperty("RequiredFieldValidator13")]
	private RequiredFieldValidator _RequiredFieldValidator13;

	[AccessedThroughProperty("RequiredFieldValidator11")]
	private RequiredFieldValidator _RequiredFieldValidator11;

	[AccessedThroughProperty("CustomValidator6")]
	private CustomValidator _CustomValidator6;

	[AccessedThroughProperty("RequiredFieldValidator12")]
	private RequiredFieldValidator _RequiredFieldValidator12;

	[AccessedThroughProperty("CustomValidator7")]
	private CustomValidator _CustomValidator7;

	[AccessedThroughProperty("RequiredFieldValidator14")]
	private RequiredFieldValidator _RequiredFieldValidator14;

	[AccessedThroughProperty("CustomValidator8")]
	private CustomValidator _CustomValidator8;

	[AccessedThroughProperty("RequiredFieldValidator15")]
	private RequiredFieldValidator _RequiredFieldValidator15;

	[AccessedThroughProperty("RequiredFieldValidator2")]
	private RequiredFieldValidator _RequiredFieldValidator2;

	[AccessedThroughProperty("RequiredFieldValidator19")]
	private RequiredFieldValidator _RequiredFieldValidator19;

	[AccessedThroughProperty("RequiredFieldValidator18")]
	private RequiredFieldValidator _RequiredFieldValidator18;

	[AccessedThroughProperty("CustomValidator12")]
	private CustomValidator _CustomValidator12;

	[AccessedThroughProperty("RequiredFieldValidator16")]
	private RequiredFieldValidator _RequiredFieldValidator16;

	[AccessedThroughProperty("CustomValidator10")]
	private CustomValidator _CustomValidator10;

	[AccessedThroughProperty("RequiredFieldValidator17")]
	private RequiredFieldValidator _RequiredFieldValidator17;

	[AccessedThroughProperty("CustomValidator11")]
	private CustomValidator _CustomValidator11;

	[AccessedThroughProperty("CustomValidator13")]
	private CustomValidator _CustomValidator13;

	[AccessedThroughProperty("RequiredFieldValidator20")]
	private RequiredFieldValidator _RequiredFieldValidator20;

	[AccessedThroughProperty("RequiredFieldValidator21")]
	private RequiredFieldValidator _RequiredFieldValidator21;

	[AccessedThroughProperty("CustomValidator15")]
	private CustomValidator _CustomValidator15;

	[AccessedThroughProperty("RequiredFieldValidator26")]
	private RequiredFieldValidator _RequiredFieldValidator26;

	[AccessedThroughProperty("CustomValidator18")]
	private CustomValidator _CustomValidator18;

	[AccessedThroughProperty("RequiredFieldValidator22")]
	private RequiredFieldValidator _RequiredFieldValidator22;

	[AccessedThroughProperty("CustomValidator1")]
	private CustomValidator _CustomValidator1;

	[AccessedThroughProperty("RequiredFieldValidator24")]
	private RequiredFieldValidator _RequiredFieldValidator24;

	[AccessedThroughProperty("CustomValidator17")]
	private CustomValidator _CustomValidator17;

	[AccessedThroughProperty("RequiredFieldValidator23")]
	private RequiredFieldValidator _RequiredFieldValidator23;

	[AccessedThroughProperty("CustomValidator14")]
	private CustomValidator _CustomValidator14;

	[AccessedThroughProperty("RequiredFieldValidator25")]
	private RequiredFieldValidator _RequiredFieldValidator25;

	[AccessedThroughProperty("CustomValidator16")]
	private CustomValidator _CustomValidator16;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Tเลขท\u0e35\u0e48")]
	private TextBox textBox_0;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("Tcustname")]
	private TextBox _Tcustname;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("checkbox1")]
	private CheckBox _checkbox1;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader17")]
	private ColumnHeader _ColumnHeader17;

	[AccessedThroughProperty("ColumnHeader18")]
	private ColumnHeader _ColumnHeader18;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ListView3")]
	private ListView _ListView3;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ColumnHeader19")]
	private ColumnHeader _ColumnHeader19;

	[AccessedThroughProperty("ColumnHeader20")]
	private ColumnHeader _ColumnHeader20;

	[AccessedThroughProperty("ColumnHeader21")]
	private ColumnHeader _ColumnHeader21;

	[AccessedThroughProperty("ItemPanel1")]
	private ItemPanel _ItemPanel1;

	[AccessedThroughProperty("LabelItem10")]
	private LabelItem _LabelItem10;

	[AccessedThroughProperty("Lsell")]
	private LabelItem _Lsell;

	[AccessedThroughProperty("ColumnHeader22")]
	private ColumnHeader _ColumnHeader22;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader23")]
	private ColumnHeader _ColumnHeader23;

	[AccessedThroughProperty("ColumnHeader24")]
	private ColumnHeader _ColumnHeader24;

	[AccessedThroughProperty("ColumnHeader25")]
	private ColumnHeader _ColumnHeader25;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ListView4")]
	private ListView _ListView4;

	[AccessedThroughProperty("ColumnHeader26")]
	private ColumnHeader _ColumnHeader26;

	[AccessedThroughProperty("ColumnHeader27")]
	private ColumnHeader _ColumnHeader27;

	[AccessedThroughProperty("ColumnHeader28")]
	private ColumnHeader _ColumnHeader28;

	[AccessedThroughProperty("ColumnHeader29")]
	private ColumnHeader _ColumnHeader29;

	[AccessedThroughProperty("ColumnHeader30")]
	private ColumnHeader _ColumnHeader30;

	[AccessedThroughProperty("ColumnHeader31")]
	private ColumnHeader _ColumnHeader31;

	[AccessedThroughProperty("ColumnHeader32")]
	private ColumnHeader _ColumnHeader32;

	[AccessedThroughProperty("ColumnHeader33")]
	private ColumnHeader _ColumnHeader33;

	[AccessedThroughProperty("ColumnHeader34")]
	private ColumnHeader _ColumnHeader34;

	[AccessedThroughProperty("LinkLabel1")]
	private LinkLabel _LinkLabel1;

	[AccessedThroughProperty("ColumnHeader35")]
	private ColumnHeader _ColumnHeader35;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ออกใบFolioแบบธรรมดาToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

	[AccessedThroughProperty("ออกใบFolioแบบราชการToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_1;

	[AccessedThroughProperty("ใบกำกบภาษToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_2;

	[AccessedThroughProperty("ToolStripSeparator1")]
	private ToolStripSeparator _ToolStripSeparator1;

	[AccessedThroughProperty("พมพใบลงทะเบยนToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_3;

	[AccessedThroughProperty("ToolStripSeparator2")]
	private ToolStripSeparator _ToolStripSeparator2;

	private bool Busy;

	internal virtual Timer TimeMaxzimize
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimeMaxzimize;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TimeMaxzimize_Tick;
			if (_TimeMaxzimize != null)
			{
				_TimeMaxzimize.Tick -= value2;
			}
			_TimeMaxzimize = value;
			if (_TimeMaxzimize != null)
			{
				_TimeMaxzimize.Tick += value2;
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

	internal virtual RequiredFieldValidator RequiredFieldValidator7
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator7 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator6
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator6 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator4
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator4 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator5
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator5 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator3
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator3 = value;
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

	internal virtual CustomValidator CustomValidator4
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator4 = value;
		}
	}

	internal virtual CustomValidator CustomValidator3
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator3 = value;
		}
	}

	internal virtual CustomValidator CustomValidator2
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator2 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator8
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator8 = value;
		}
	}

	internal virtual CustomValidator CustomValidator5
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator5 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator10
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator10 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator9
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator9 = value;
		}
	}

	internal virtual CustomValidator CustomValidator9
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator9 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator13
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator13 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator11
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator11 = value;
		}
	}

	internal virtual CustomValidator CustomValidator6
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator6 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator12
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator12 = value;
		}
	}

	internal virtual CustomValidator CustomValidator7
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator7 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator14
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator14 = value;
		}
	}

	internal virtual CustomValidator CustomValidator8
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator8 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator15
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator15 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator2 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator19
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator19 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator18
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator18 = value;
		}
	}

	internal virtual CustomValidator CustomValidator12
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator12 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator16
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator16 = value;
		}
	}

	internal virtual CustomValidator CustomValidator10
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator10 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator17
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator17 = value;
		}
	}

	internal virtual CustomValidator CustomValidator11
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator11 = value;
		}
	}

	internal virtual CustomValidator CustomValidator13
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator13 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator20
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator20 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator21
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator21 = value;
		}
	}

	internal virtual CustomValidator CustomValidator15
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator15 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator26
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator26;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator26 = value;
		}
	}

	internal virtual CustomValidator CustomValidator18
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator18 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator22
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator22 = value;
		}
	}

	internal virtual CustomValidator CustomValidator1
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator1 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator24
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator24 = value;
		}
	}

	internal virtual CustomValidator CustomValidator17
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator17 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator23
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator23 = value;
		}
	}

	internal virtual CustomValidator CustomValidator14
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator14 = value;
		}
	}

	internal virtual RequiredFieldValidator RequiredFieldValidator25
	{
		[DebuggerNonUserCode]
		get
		{
			return _RequiredFieldValidator25;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RequiredFieldValidator25 = value;
		}
	}

	internal virtual CustomValidator CustomValidator16
	{
		[DebuggerNonUserCode]
		get
		{
			return _CustomValidator16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CustomValidator16 = value;
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

	internal virtual TextBox TextBox_0
	{
		[DebuggerNonUserCode]
		get
		{
			return textBox_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBox_0 = value;
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

	internal virtual global::PrintableListView.PrintableListView ListView1
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
			MouseEventHandler value3 = ListView1_MouseClick;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
				_ListView1.MouseClick -= value3;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
				_ListView1.MouseClick += value3;
			}
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

	internal virtual TextBox Tcustname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcustname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcustname = value;
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

	internal virtual DateTimePicker DateTimePicker2
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DateTimePicker2 = value;
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
			_DateTimePicker1 = value;
		}
	}

	internal virtual CheckBox checkbox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _checkbox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = checkbox1_CheckedChanged;
			if (_checkbox1 != null)
			{
				_checkbox1.CheckedChanged -= value2;
			}
			_checkbox1 = value;
			if (_checkbox1 != null)
			{
				_checkbox1.CheckedChanged += value2;
			}
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

	internal virtual ListView ListView2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ListView2 = value;
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
			EventHandler value2 = Label4_Click;
			if (_Label4 != null)
			{
				_Label4.Click -= value2;
			}
			_Label4 = value;
			if (_Label4 != null)
			{
				_Label4.Click += value2;
			}
		}
	}

	internal virtual ListView ListView3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListView3_SelectedIndexChanged;
			if (_ListView3 != null)
			{
				_ListView3.SelectedIndexChanged -= value2;
			}
			_ListView3 = value;
			if (_ListView3 != null)
			{
				_ListView3.SelectedIndexChanged += value2;
			}
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

	internal virtual ItemPanel ItemPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemPanel1 = value;
		}
	}

	internal virtual LabelItem LabelItem_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem10 = value;
		}
	}

	internal virtual LabelItem Lsell
	{
		[DebuggerNonUserCode]
		get
		{
			return _Lsell;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Lsell = value;
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

	internal virtual ListView ListView4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListView4_SelectedIndexChanged;
			if (_ListView4 != null)
			{
				_ListView4.SelectedIndexChanged -= value2;
			}
			_ListView4 = value;
			if (_ListView4 != null)
			{
				_ListView4.SelectedIndexChanged += value2;
			}
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

	internal virtual ColumnHeader ColumnHeader34
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader34;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader34 = value;
		}
	}

	internal virtual LinkLabel LinkLabel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LinkLabel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			LinkLabelLinkClickedEventHandler value2 = LinkLabel1_LinkClicked;
			if (_LinkLabel1 != null)
			{
				_LinkLabel1.LinkClicked -= value2;
			}
			_LinkLabel1 = value;
			if (_LinkLabel1 != null)
			{
				_LinkLabel1.LinkClicked += value2;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader35
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader35;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader35 = value;
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

	internal virtual ToolStripMenuItem ToolStripMenuItem_0
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_0_Click;
			if (toolStripMenuItem_0 != null)
			{
				toolStripMenuItem_0.Click -= value2;
			}
			toolStripMenuItem_0 = value;
			if (toolStripMenuItem_0 != null)
			{
				toolStripMenuItem_0.Click += value2;
			}
		}
	}

	internal virtual ToolStripMenuItem ToolStripMenuItem_1
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_1_Click;
			if (toolStripMenuItem_1 != null)
			{
				toolStripMenuItem_1.Click -= value2;
			}
			toolStripMenuItem_1 = value;
			if (toolStripMenuItem_1 != null)
			{
				toolStripMenuItem_1.Click += value2;
			}
		}
	}

	internal virtual ToolStripMenuItem ToolStripMenuItem_2
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_2_Click;
			if (toolStripMenuItem_2 != null)
			{
				toolStripMenuItem_2.Click -= value2;
			}
			toolStripMenuItem_2 = value;
			if (toolStripMenuItem_2 != null)
			{
				toolStripMenuItem_2.Click += value2;
			}
		}
	}

	internal virtual ToolStripSeparator ToolStripSeparator1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ToolStripSeparator1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ToolStripSeparator1 = value;
		}
	}

	internal virtual ToolStripMenuItem ToolStripMenuItem_3
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_3_Click;
			if (toolStripMenuItem_3 != null)
			{
				toolStripMenuItem_3.Click -= value2;
			}
			toolStripMenuItem_3 = value;
			if (toolStripMenuItem_3 != null)
			{
				toolStripMenuItem_3.Click += value2;
			}
		}
	}

	internal virtual ToolStripSeparator ToolStripSeparator2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ToolStripSeparator2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ToolStripSeparator2 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmInOutMain()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmInOutMain()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmSaleMain_Load;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmInOutMain));
		this.TimeMaxzimize = new System.Windows.Forms.Timer(this.components);
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.RequiredFieldValidator8 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกจำนวน");
		this.CustomValidator5 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator7 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator4 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator6 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator3 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator4 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator5 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator2 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator3 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator10 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator9 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator12 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator7 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator11 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator6 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator13 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator9 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator1 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.CustomValidator8 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator14 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator21 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator15 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator26 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator18 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator22 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator1 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator24 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator17 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator23 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator14 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator25 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator16 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator15 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator17 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator11 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator16 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator10 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator2 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.RequiredFieldValidator18 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator12 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator19 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณากรอกข\u0e49อม\u0e39ล");
		this.CustomValidator13 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.RequiredFieldValidator20 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader23 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader22 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader35 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripMenuItem_1 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripMenuItem_2 = new System.Windows.Forms.ToolStripMenuItem();
		this.ListView4 = new System.Windows.Forms.ListView();
		this.ColumnHeader26 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader27 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader28 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader29 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader30 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader31 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader32 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader33 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader34 = new System.Windows.Forms.ColumnHeader();
		this.Button1 = new System.Windows.Forms.Button();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader17 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader18 = new System.Windows.Forms.ColumnHeader();
		this.LinkLabel1 = new System.Windows.Forms.LinkLabel();
		this.Label5 = new System.Windows.Forms.Label();
		this.ItemPanel1 = new DevComponents.DotNetBar.ItemPanel();
		this.LabelItem_0 = new DevComponents.DotNetBar.LabelItem();
		this.Lsell = new DevComponents.DotNetBar.LabelItem();
		this.Label4 = new System.Windows.Forms.Label();
		this.ListView3 = new System.Windows.Forms.ListView();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader19 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader20 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader21 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader24 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader25 = new System.Windows.Forms.ColumnHeader();
		this.Label1 = new System.Windows.Forms.Label();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.checkbox1 = new System.Windows.Forms.CheckBox();
		this.Label11 = new System.Windows.Forms.Label();
		this.Tcustname = new System.Windows.Forms.TextBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.TextBox_0 = new System.Windows.Forms.TextBox();
		this.Label2 = new System.Windows.Forms.Label();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ToolStripSeparator1 = new System.Windows.Forms.ToolStripSeparator();
		this.ToolStripMenuItem_3 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripSeparator2 = new System.Windows.Forms.ToolStripSeparator();
		this.PanelEx2.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(964, 24);
		panelEx3.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Far;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 15;
		this.PanelEx1.Text = "รายการใบลงทะเบ\u0e35ยน |";
		this.RequiredFieldValidator8.ErrorMessage = "กร\u0e38ณากรอกจำนวน";
		this.RequiredFieldValidator8.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator5.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator5.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator7.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator7.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator4.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator4.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator6.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator6.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator3.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator3.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator4.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator4.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator5.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator5.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator2.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator2.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator3.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator3.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator10.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator10.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator9.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator9.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator12.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator12.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator7.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator7.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator11.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator11.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator6.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator6.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator13.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator13.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator9.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator9.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator1.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator1.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator8.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator8.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator14.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator14.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator21.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator21.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator15.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator15.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator26.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator26.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator18.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator18.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator22.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator22.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator1.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator1.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator24.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator24.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator17.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator17.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator23.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator23.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator14.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator14.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator25.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator25.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator16.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator16.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator15.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator15.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator17.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator17.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator11.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator11.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator16.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator16.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator10.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator10.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator2.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator2.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator18.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator18.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator12.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator12.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator19.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ล";
		this.RequiredFieldValidator19.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator13.ErrorMessage = "กร\u0e38ณากรอกข\u0e49อม\u0e39ลเป\u0e47นต\u0e31วเลข";
		this.CustomValidator13.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator20.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator20.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.ListView1);
		this.PanelEx2.Controls.Add(this.ListView4);
		this.PanelEx2.Controls.Add(this.Button1);
		this.PanelEx2.Controls.Add(this.ListView2);
		this.PanelEx2.Controls.Add(this.LinkLabel1);
		this.PanelEx2.Controls.Add(this.Label5);
		this.PanelEx2.Controls.Add(this.ItemPanel1);
		this.PanelEx2.Controls.Add(this.Label4);
		this.PanelEx2.Controls.Add(this.ListView3);
		this.PanelEx2.Controls.Add(this.Label1);
		this.PanelEx2.Controls.Add(this.GroupBox1);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		location = new System.Drawing.Point(0, 24);
		panelEx4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx5.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx2;
		size = new System.Drawing.Size(964, 420);
		panelEx6.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 110;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[12]
		{
			this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader9, this.ColumnHeader14, this.ColumnHeader12, this.ColumnHeader2, this.ColumnHeader1, this.ColumnHeader15, this.ColumnHeader23, this.ColumnHeader22,
			this.ColumnHeader13, this.ColumnHeader35
		});
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(7, 101);
		listView.Location = location;
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView3 = this.ListView1;
		size = new System.Drawing.Size(953, 131);
		listView3.Size = size;
		this.ListView1.TabIndex = 15;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "";
		this.ColumnHeader3.Width = 0;
		this.ColumnHeader4.Text = "เลขท\u0e35\u0e48";
		this.ColumnHeader4.Width = 120;
		this.ColumnHeader9.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader9.Width = 140;
		this.ColumnHeader14.Text = "โทร";
		this.ColumnHeader14.Width = 130;
		this.ColumnHeader12.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader12.Width = 140;
		this.ColumnHeader2.Text = "ราคาห\u0e49องพ\u0e31ก";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader1.Text = "ราคาส\u0e34นค\u0e49า";
		this.ColumnHeader1.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader1.Width = 100;
		this.ColumnHeader15.Text = "ราคารวม";
		this.ColumnHeader15.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader15.Width = 100;
		this.ColumnHeader23.Text = "ค\u0e49างจ\u0e48าย";
		this.ColumnHeader23.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader23.Width = 80;
		this.ColumnHeader22.Text = "สถานะ";
		this.ColumnHeader22.Width = 180;
		this.ColumnHeader13.Text = "ผ\u0e39\u0e49จ\u0e31ดทำ";
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader35.Text = "ประเภท";
		this.ColumnHeader35.Width = 80;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[6] { this.ToolStripMenuItem_3, this.ToolStripSeparator1, this.ToolStripMenuItem_0, this.ToolStripMenuItem_1, this.ToolStripSeparator2, this.ToolStripMenuItem_2 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(215, 104);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Image = (System.Drawing.Image)resources.GetObject("ออกใบFolioแบบธรรมดาToolStripMenuItem.Image");
		this.ToolStripMenuItem_0.Name = "ออกใบFolioแบบธรรมดาToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(214, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ออกใบ Folio แบบธรรมดา";
		this.ToolStripMenuItem_1.Image = (System.Drawing.Image)resources.GetObject("ออกใบFolioแบบราชการToolStripMenuItem.Image");
		this.ToolStripMenuItem_1.Name = "ออกใบFolioแบบราชการToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem2 = this.ToolStripMenuItem_1;
		size = new System.Drawing.Size(214, 22);
		toolStripMenuItem2.Size = size;
		this.ToolStripMenuItem_1.Text = "ออกใบ Folio แบบราชการ";
		this.ToolStripMenuItem_2.Image = (System.Drawing.Image)resources.GetObject("ใบกำกบภาษToolStripMenuItem.Image");
		this.ToolStripMenuItem_2.Name = "ใบกำกบภาษToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem3 = this.ToolStripMenuItem_2;
		size = new System.Drawing.Size(214, 22);
		toolStripMenuItem3.Size = size;
		this.ToolStripMenuItem_2.Text = "ออกใบกำก\u0e31บภาษ\u0e35";
		this.ListView4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ListView4.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.ListView4.Columns.AddRange(new System.Windows.Forms.ColumnHeader[9] { this.ColumnHeader26, this.ColumnHeader27, this.ColumnHeader28, this.ColumnHeader29, this.ColumnHeader30, this.ColumnHeader31, this.ColumnHeader32, this.ColumnHeader33, this.ColumnHeader34 });
		this.ListView4.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListView4.FullRowSelect = true;
		this.ListView4.GridLines = true;
		System.Windows.Forms.ListView listView4 = this.ListView4;
		location = new System.Drawing.Point(637, 256);
		listView4.Location = location;
		System.Windows.Forms.ListView listView5 = this.ListView4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView5.Margin = margin;
		this.ListView4.Name = "ListView4";
		System.Windows.Forms.ListView listView6 = this.ListView4;
		size = new System.Drawing.Size(323, 160);
		listView6.Size = size;
		this.ListView4.TabIndex = 22;
		this.ListView4.UseCompatibleStateImageBehavior = false;
		this.ListView4.View = System.Windows.Forms.View.Details;
		this.ColumnHeader26.Text = "ท\u0e35\u0e48";
		this.ColumnHeader26.Width = 25;
		this.ColumnHeader27.Text = "เลขท\u0e35\u0e48ใบเสร\u0e47จร\u0e31บเง\u0e34น";
		this.ColumnHeader27.Width = 110;
		this.ColumnHeader28.Text = "รายละเอ\u0e35ยด";
		this.ColumnHeader28.Width = 200;
		this.ColumnHeader29.Text = "รวมจ\u0e48าย(รายการ)";
		this.ColumnHeader29.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader29.Width = 100;
		this.ColumnHeader30.Text = "เง\u0e34นสด";
		this.ColumnHeader30.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader30.Width = 100;
		this.ColumnHeader31.Text = "เครด\u0e34ต";
		this.ColumnHeader31.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader31.Width = 100;
		this.ColumnHeader32.Text = "รวมเง\u0e34น";
		this.ColumnHeader32.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader32.Width = 100;
		this.ColumnHeader33.Text = "ว\u0e31นท\u0e35\u0e48ชำระ";
		this.ColumnHeader33.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader33.Width = 120;
		this.ColumnHeader34.Text = "หมายเหต\u0e38";
		this.ColumnHeader34.Width = 200;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(915, 231);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(46, 26);
		button2.Size = size;
		this.Button1.TabIndex = 25;
		this.Button1.Text = "พ\u0e34มพ\u0e4c";
		this.Button1.UseVisualStyleBackColor = true;
		this.ListView2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ListView2.CheckBoxes = true;
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[6] { this.ColumnHeader5, this.ColumnHeader11, this.ColumnHeader8, this.ColumnHeader10, this.ColumnHeader17, this.ColumnHeader18 });
		this.ListView2.FullRowSelect = true;
		this.ListView2.GridLines = true;
		System.Windows.Forms.ListView listView7 = this.ListView2;
		location = new System.Drawing.Point(235, 256);
		listView7.Location = location;
		System.Windows.Forms.ListView listView8 = this.ListView2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView8.Margin = margin;
		this.ListView2.MultiSelect = false;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView9 = this.ListView2;
		size = new System.Drawing.Size(398, 160);
		listView9.Size = size;
		this.ListView2.TabIndex = 17;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader5.Text = "";
		this.ColumnHeader5.Width = 0;
		this.ColumnHeader11.Text = "ห\u0e49อง";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader8.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.ColumnHeader8.Width = 120;
		this.ColumnHeader10.Text = "จำนวน";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader10.Width = 50;
		this.ColumnHeader17.Text = "ราคา";
		this.ColumnHeader17.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader17.Width = 70;
		this.ColumnHeader18.Text = "ราคารวม";
		this.ColumnHeader18.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader18.Width = 80;
		this.LinkLabel1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.LinkLabel1.AutoSize = true;
		System.Windows.Forms.LinkLabel linkLabel = this.LinkLabel1;
		location = new System.Drawing.Point(380, 238);
		linkLabel.Location = location;
		this.LinkLabel1.Name = "LinkLabel1";
		System.Windows.Forms.LinkLabel linkLabel2 = this.LinkLabel1;
		size = new System.Drawing.Size(43, 16);
		linkLabel2.Size = size;
		this.LinkLabel1.TabIndex = 24;
		this.LinkLabel1.TabStop = true;
		this.LinkLabel1.Text = "(ซ\u0e48อน)";
		this.Label5.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label = this.Label5;
		location = new System.Drawing.Point(639, 238);
		label.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label2 = this.Label5;
		size = new System.Drawing.Size(52, 16);
		label2.Size = size;
		this.Label5.TabIndex = 23;
		this.Label5.Text = "ชำระเง\u0e34น";
		this.ItemPanel1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ItemPanel1.BackgroundStyle.BackColor = System.Drawing.Color.FromArgb(255, 255, 132);
		this.ItemPanel1.BackgroundStyle.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel1.BackgroundStyle.BorderBottomWidth = 1;
		this.ItemPanel1.BackgroundStyle.BorderColor = System.Drawing.Color.FromArgb(127, 157, 185);
		this.ItemPanel1.BackgroundStyle.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel1.BackgroundStyle.BorderLeftWidth = 1;
		this.ItemPanel1.BackgroundStyle.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel1.BackgroundStyle.BorderRightWidth = 1;
		this.ItemPanel1.BackgroundStyle.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel1.BackgroundStyle.BorderTopWidth = 1;
		this.ItemPanel1.BackgroundStyle.Class = "";
		this.ItemPanel1.BackgroundStyle.PaddingBottom = 1;
		this.ItemPanel1.BackgroundStyle.PaddingLeft = 1;
		this.ItemPanel1.BackgroundStyle.PaddingRight = 1;
		this.ItemPanel1.BackgroundStyle.PaddingTop = 1;
		this.ItemPanel1.ContainerControlProcessDialogKey = true;
		this.ItemPanel1.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ItemPanel1.ForeColor = System.Drawing.Color.MidnightBlue;
		this.ItemPanel1.HorizontalItemAlignment = DevComponents.DotNetBar.eHorizontalItemsAlignment.Center;
		this.ItemPanel1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.LabelItem_0, this.Lsell });
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel1;
		location = new System.Drawing.Point(716, 231);
		itemPanel.Location = location;
		this.ItemPanel1.MultiLine = true;
		this.ItemPanel1.Name = "ItemPanel1";
		this.ItemPanel1.ResizeItemsToFit = false;
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel1;
		size = new System.Drawing.Size(201, 26);
		itemPanel2.Size = size;
		this.ItemPanel1.Style = DevComponents.DotNetBar.eDotNetBarStyle.Windows7;
		this.ItemPanel1.TabIndex = 21;
		this.ItemPanel1.Text = "ItemPanel1";
		this.LabelItem_0.Name = "LabelItem10";
		this.LabelItem_0.Text = "รวมราคา :";
		this.LabelItem_0.TextAlignment = System.Drawing.StringAlignment.Far;
		this.LabelItem_0.Width = 70;
		this.Lsell.BackColor = System.Drawing.Color.White;
		this.Lsell.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.Lsell.ForeColor = System.Drawing.Color.Green;
		this.Lsell.Name = "Lsell";
		this.Lsell.Text = "0.00";
		this.Lsell.TextAlignment = System.Drawing.StringAlignment.Far;
		this.Lsell.Width = 115;
		this.Label4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label4;
		location = new System.Drawing.Point(7, 238);
		label3.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label4 = this.Label4;
		size = new System.Drawing.Size(109, 16);
		label4.Size = size;
		this.Label4.TabIndex = 20;
		this.Label4.Text = "รายละเอ\u0e35ยดห\u0e49องพ\u0e31ก";
		this.ListView3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView3.CheckBoxes = true;
		this.ListView3.Columns.AddRange(new System.Windows.Forms.ColumnHeader[8] { this.ColumnHeader6, this.ColumnHeader7, this.ColumnHeader16, this.ColumnHeader19, this.ColumnHeader20, this.ColumnHeader21, this.ColumnHeader24, this.ColumnHeader25 });
		this.ListView3.FullRowSelect = true;
		this.ListView3.GridLines = true;
		System.Windows.Forms.ListView listView10 = this.ListView3;
		location = new System.Drawing.Point(7, 256);
		listView10.Location = location;
		System.Windows.Forms.ListView listView11 = this.ListView3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView11.Margin = margin;
		this.ListView3.MultiSelect = false;
		this.ListView3.Name = "ListView3";
		System.Windows.Forms.ListView listView12 = this.ListView3;
		size = new System.Drawing.Size(223, 160);
		listView12.Size = size;
		this.ListView3.TabIndex = 19;
		this.ListView3.UseCompatibleStateImageBehavior = false;
		this.ListView3.View = System.Windows.Forms.View.Details;
		this.ColumnHeader6.Text = "";
		this.ColumnHeader6.Width = 0;
		this.ColumnHeader7.Text = "ห\u0e49อง";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader16.Text = "ประเภท";
		this.ColumnHeader16.Width = 150;
		this.ColumnHeader19.Text = "จำนวน";
		this.ColumnHeader19.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader19.Width = 50;
		this.ColumnHeader20.Text = "ราคา";
		this.ColumnHeader20.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader20.Width = 70;
		this.ColumnHeader21.Text = "ราคารวม";
		this.ColumnHeader21.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader21.Width = 100;
		this.ColumnHeader24.Text = "ว\u0e31นท\u0e35\u0e48 IN";
		this.ColumnHeader24.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader24.Width = 110;
		this.ColumnHeader25.Text = "ว\u0e31นท\u0e35\u0e48 OUT";
		this.ColumnHeader25.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader25.Width = 110;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(234, 238);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(147, 16);
		label6.Size = size;
		this.Label1.TabIndex = 18;
		this.Label1.Text = "รายละเอ\u0e35ยดส\u0e34นค\u0e49า / บร\u0e34การ";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.checkbox1);
		this.GroupBox1.Controls.Add(this.Label11);
		this.GroupBox1.Controls.Add(this.Tcustname);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.TextBox_0);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.ButtonX1);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.GroupBox1.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		location = new System.Drawing.Point(7, 7);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox1;
		size = new System.Drawing.Size(953, 85);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 14;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(347, 47);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(150, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 89;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(347, 21);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(150, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 90;
		this.checkbox1.AutoSize = true;
		this.checkbox1.Checked = true;
		this.checkbox1.CheckState = System.Windows.Forms.CheckState.Checked;
		System.Windows.Forms.CheckBox checkBox = this.checkbox1;
		location = new System.Drawing.Point(291, 22);
		checkBox.Location = location;
		this.checkbox1.Name = "checkbox1";
		System.Windows.Forms.CheckBox checkBox2 = this.checkbox1;
		size = new System.Drawing.Size(59, 20);
		checkBox2.Size = size;
		this.checkbox1.TabIndex = 88;
		this.checkbox1.Text = "ว\u0e31นท\u0e35\u0e48 :";
		this.checkbox1.UseVisualStyleBackColor = true;
		this.Label11.AutoSize = true;
		this.Label11.BackColor = System.Drawing.Color.Transparent;
		this.Label11.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label7 = this.Label11;
		location = new System.Drawing.Point(294, 50);
		label7.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label8 = this.Label11;
		size = new System.Drawing.Size(50, 14);
		label8.Size = size;
		this.Label11.TabIndex = 87;
		this.Label11.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		this.Tcustname.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tcustname.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tcustname = this.Tcustname;
		location = new System.Drawing.Point(139, 50);
		tcustname.Location = location;
		System.Windows.Forms.TextBox tcustname2 = this.Tcustname;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcustname2.Margin = margin;
		this.Tcustname.Name = "Tcustname";
		System.Windows.Forms.TextBox tcustname3 = this.Tcustname;
		size = new System.Drawing.Size(136, 21);
		tcustname3.Size = size;
		this.Tcustname.TabIndex = 2;
		this.Label3.AutoSize = true;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label3.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label9 = this.Label3;
		location = new System.Drawing.Point(74, 52);
		label9.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label10 = this.Label3;
		size = new System.Drawing.Size(63, 16);
		label10.Size = size;
		this.Label3.TabIndex = 1;
		this.Label3.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า :";
		this.TextBox_0.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TextBox_0.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox textBox = this.TextBox_0;
		location = new System.Drawing.Point(139, 21);
		textBox.Location = location;
		System.Windows.Forms.TextBox textBox2 = this.TextBox_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox2.Margin = margin;
		this.TextBox_0.Name = "Tเลขท\u0e35\u0e48";
		System.Windows.Forms.TextBox textBox3 = this.TextBox_0;
		size = new System.Drawing.Size(136, 21);
		textBox3.Size = size;
		this.TextBox_0.TabIndex = 2;
		this.Label2.AutoSize = true;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label11 = this.Label2;
		location = new System.Drawing.Point(91, 23);
		label11.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label12 = this.Label2;
		size = new System.Drawing.Size(46, 16);
		label12.Size = size;
		this.Label2.TabIndex = 1;
		this.Label2.Text = "เลขท\u0e35\u0e48 :";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(512, 21);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		size = new System.Drawing.Size(115, 49);
		buttonX3.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 0;
		this.ButtonX1.Text = "ค\u0e49นหา";
		this.ToolStripSeparator1.Name = "ToolStripSeparator1";
		System.Windows.Forms.ToolStripSeparator toolStripSeparator = this.ToolStripSeparator1;
		size = new System.Drawing.Size(211, 6);
		toolStripSeparator.Size = size;
		this.ToolStripMenuItem_3.Image = (System.Drawing.Image)resources.GetObject("พมพใบลงทะเบยนToolStripMenuItem.Image");
		this.ToolStripMenuItem_3.Name = "พมพใบลงทะเบยนToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem4 = this.ToolStripMenuItem_3;
		size = new System.Drawing.Size(214, 22);
		toolStripMenuItem4.Size = size;
		this.ToolStripMenuItem_3.Text = "พ\u0e34มพ\u0e4cใบลงทะเบ\u0e35ยน";
		this.ToolStripSeparator2.Name = "ToolStripSeparator2";
		System.Windows.Forms.ToolStripSeparator toolStripSeparator2 = this.ToolStripSeparator2;
		size = new System.Drawing.Size(211, 6);
		toolStripSeparator2.Size = size;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(964, 444);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmInOutMain";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "รายการใบลงทะเบ\u0e35ยน";
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx2.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void TimeMaxzimize_Tick(object sender, EventArgs e)
	{
		TimeMaxzimize.Enabled = false;
		WindowState = FormWindowState.Normal;
		WindowState = FormWindowState.Maximized;
	}

	private void FrmSaleMain_Load(object sender, EventArgs e)
	{
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Month) + "/01/" + Conversions.ToString(DateTime.Now.Year));
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Month) + "/" + Conversions.ToString(DateTime.DaysInMonth(DateTime.Now.Year, DateTime.Now.Month)) + "/" + Conversions.ToString(DateTime.Now.Year));
		TimeMaxzimize.Enabled = true;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Search();
	}

	public void Search()
	{
		Lsell.Text = Conversions.ToString(0);
		Cursor = Cursors.WaitCursor;
		object left = "select * from View_Check_Out where cin_no<>''";
		if (Operators.CompareString(TextBox_0.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and Cin_no like '%" + TextBox_0.Text, "%'"));
		}
		if (Operators.CompareString(Tcustname.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and cust_name like '%" + Tcustname.Text, "%'"));
		}
		if (checkbox1.Checked)
		{
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" and (Cin_Date between '" + Conversions.ToString(DateTimePicker1.Value.Date), " 00:00:00' and '"), Conversions.ToString(DateTimePicker2.Value.Date)), " 23:59:59')"));
		}
		left = Operators.ConcatenateObject(left, " order by cin_no desc");
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		decimal num = default(decimal);
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
				int count = ListView1.Items.Count;
				ListView1.Items.Add(dataSet.Tables[0].Rows[num3]["Cin_no"].ToString());
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Cin_no"].ToString());
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["cust_name"].ToString());
				ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[count].SubItems;
				object[] array = new object[1];
				DataRow dataRow = dataSet.Tables[0].Rows[num3];
				string columnName = "cust_add_tel";
				array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
				object[] array2 = array;
				bool[] array3 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", array2, null, null, array3, IgnoreReturn: true);
				if (array3[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
				}
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Date"]), "dd/MM/yyyy HH:mm:ss"));
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Total_Price_Room"]), "#,##0.00"));
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Total_Price_Product"]), "#,##0.00"));
				ListView1.Items[count].SubItems.Add(Strings.Format(Operators.AddObject(dataSet.Tables[0].Rows[num3]["Total_Price_Room"], dataSet.Tables[0].Rows[num3]["Total_Price_Product"]), "#,##0.00"));
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Total_Price_balance"]), "#,##0.00"));
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num3]["checkout"], 0, TextCompare: false))
				{
					ListView1.Items[num3].SubItems.Add("ย\u0e31งไม\u0e48ได\u0e49 Check-Out");
					ListView1.Items[num3].BackColor = Color.LightPink;
				}
				else if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num3]["checkout"], dataSet.Tables[0].Rows[num3]["RoomALL"], TextCompare: false))
				{
					ListView1.Items[num3].SubItems.Add("Check-Out ย\u0e31งไม\u0e48ครบ");
					ListView1.Items[num3].BackColor = Color.Yellow;
				}
				else
				{
					ListView1.Items[num3].SubItems.Add("Check-Out เร\u0e35ยบร\u0e49อย");
					ListView1.Items[num3].BackColor = Color.LightGreen;
				}
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num3]["cin_status"], "ยกเล\u0e34ก", TextCompare: false))
				{
					ListView1.Items[num3].BackColor = Color.Red;
					ListView1.Items[num3].ForeColor = Color.White;
					ListView1.Items[num3].SubItems[8].Text = "ยกเล\u0e34ก";
				}
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Cin_by"].ToString());
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num3]["cin_type"], 0, TextCompare: false))
				{
					ListView1.Items[count].SubItems.Add("รายว\u0e31น");
				}
				else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num3]["cin_type"], 1, TextCompare: false))
				{
					ListView1.Items[count].SubItems.Add("รายช\u0e31\u0e48วโมง");
				}
				else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num3]["cin_type"], 2, TextCompare: false))
				{
					ListView1.Items[count].SubItems.Add("รายเด\u0e37อน");
				}
				Lsell.Text = Strings.Format(Operators.AddObject(Operators.AddObject(Conversions.ToDecimal(Lsell.Text), dataSet.Tables[0].Rows[num3]["Total_Price_Room"]), dataSet.Tables[0].Rows[num3]["Total_Price_Product"]), "#,##0.00");
				num3++;
			}
			Cursor = Cursors.Default;
		}
	}

	private void checkbox1_CheckedChanged(object sender, EventArgs e)
	{
		DateTimePicker1.Enabled = checkbox1.Checked;
		DateTimePicker2.Enabled = checkbox1.Checked;
	}

	private void ListView1_MouseClick(object sender, MouseEventArgs e)
	{
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		ListView2.Items.Clear();
		ListView3.Items.Clear();
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_Product where Cin_No='" + ListView1.SelectedItems[0].SubItems[1].Text + "' order by id");
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
				ListView listView = ListView2;
				int count = listView.Items.Count;
				ListView.ListViewItemCollection items = listView.Items;
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
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow3 = dataRow;
				columnName = "Cin_Room_no";
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
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow4 = dataRow;
				columnName = "Cin_Pro_name";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array7 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow5 = dataRow;
				columnName = "Cin_Pro_num";
				array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
				array = array3;
				object[] arguments4 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cin_Pro_price"]), "#,##0.00"));
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cin_Pro_priceTotal"]), "#,##0.00"));
				listView = null;
				num2++;
			}
			dataSet = Module1.connect("select * from HT_CheckIn_Ds where Cin_No='" + ListView1.SelectedItems[0].SubItems[1].Text + "' order by id");
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
				ListView listView2 = ListView3;
				int count2 = listView2.Items.Count;
				ListView.ListViewItemCollection items2 = listView2.Items;
				object[] array3 = new object[1];
				object[] array8 = array3;
				DataRow dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow6 = dataRow;
				string columnName = "id";
				array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
				object[] array = array3;
				object[] arguments5 = array;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(items2, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems4 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array9 = array3;
				dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow7 = dataRow;
				columnName = "Cin_Room_No";
				array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
				array = array3;
				object[] arguments6 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems4, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems5 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array10 = array3;
				dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow8 = dataRow;
				columnName = "Cin_Room_Type";
				array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
				array = array3;
				object[] arguments7 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems5, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems6 = listView2.Items[count2].SubItems;
				array3 = new object[1];
				object[] array11 = array3;
				dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow9 = dataRow;
				columnName = "Cin_Room_Night";
				array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
				array = array3;
				object[] arguments8 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems6, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Cin_Room_Price"]), "#,##0.00"));
				listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Cin_Room_PriceToTal"]), "#,##0.00"));
				listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Cin_Room_in"]), "dd-MM-yy HH:mm"));
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num6]["Cin_Room_status"], "เข\u0e49าพ\u0e31ก", TextCompare: false))
				{
					listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num6]["Cin_Room_out"]), "dd-MM-yy HH:mm"));
				}
				else
				{
					listView2.Items[count2].SubItems.Add("");
				}
				listView2 = null;
				num6++;
			}
			SearchPay();
		}
	}

	public void SearchPay()
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		DataSet dataSet = Module1.connect("select * from View_Pay_Ds where Cin_No='" + ListView1.SelectedItems[0].SubItems[1].Text + "' order by id");
		DataSet dataSet2 = Module1.connect("select * from HT_Log_Debt where log_ds = 'ต\u0e31ดจากใบลงทะเบ\u0e35ยน " + ListView1.SelectedItems[0].SubItems[1].Text + "' order by log_date");
		ListView4.Items.Clear();
		string right = "";
		int num = 1;
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			right = "";
		}
		string left = "YELLOW";
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
				ListView listView = ListView4;
				int count = listView.Items.Count;
				object[] array3;
				DataRow dataRow;
				string columnName;
				object[] array;
				bool[] array4;
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num3]["pay_no"], right, TextCompare: false))
				{
					listView.Items.Add(Conversions.ToString(num));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					array = new object[1];
					object[] array2 = array;
					dataRow = dataSet.Tables[0].Rows[num3];
					DataRow dataRow2 = dataRow;
					columnName = "pay_no";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					array3 = array;
					object[] arguments = array3;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					num++;
					left = ((Operators.CompareString(left, "YELLOW", TextCompare: false) != 0) ? "YELLOW" : "");
				}
				else
				{
					listView.Items.Add("");
					listView.Items[count].SubItems.Add("");
				}
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow3 = dataRow;
				columnName = "Cin_Pay_Ds_name";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow4 = dataRow;
				columnName = "Cin_Pay_Ds_Price";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				if (Operators.ConditionalCompareObjectNotEqual(dataSet.Tables[0].Rows[num3]["pay_no"], right, TextCompare: false))
				{
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Pay_cash"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Pay_credit"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(Operators.AddObject(dataSet.Tables[0].Rows[num3]["Cin_Pay_cash"], dataSet.Tables[0].Rows[num3]["Cin_Pay_credit"]), "#,##0.00"));
					listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Pay_Date"]), "dd/MM/yy HH:mm"));
				}
				else
				{
					listView.Items[count].SubItems.Add("");
					listView.Items[count].SubItems.Add("");
					listView.Items[count].SubItems.Add("");
					listView.Items[count].SubItems.Add("");
				}
				if (Operators.CompareString(left, "", TextCompare: false) != 0)
				{
					listView.Items[count].BackColor = Color.Yellow;
				}
				listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Cin_Pay_Note"].ToString());
				listView = null;
				right = Conversions.ToString(dataSet.Tables[0].Rows[num3]["pay_no"]);
				num3++;
			}
			int num6 = dataSet2.Tables[0].Rows.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 <= num5)
				{
					ListView listView2 = ListView4;
					int count2 = listView2.Items.Count;
					listView2.Items.Add("");
					listView2.Items[count2].SubItems.Add("########");
					listView2.Items[count2].SubItems.Add("ยอดจ\u0e48ายล\u0e48วงหน\u0e49า");
					ListViewItem.ListViewSubItemCollection subItems4 = listView2.Items[count2].SubItems;
					object[] array7 = new object[1];
					object[] array8 = array7;
					Type typeFromHandle = typeof(Math);
					object[] array3 = new object[1];
					object[] array9 = array3;
					DataRow dataRow = dataSet2.Tables[0].Rows[num7];
					DataRow dataRow5 = dataRow;
					string columnName = "log_price";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
					object[] array = array3;
					object[] arguments4 = array;
					bool[] array4 = new bool[1] { true };
					object obj = NewLateBinding.LateGet(null, typeFromHandle, "Abs", arguments4, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					array8[0] = RuntimeHelpers.GetObjectValue(obj);
					NewLateBinding.LateCall(subItems4, null, "Add", array7, null, null, null, IgnoreReturn: true);
					ListViewItem.ListViewSubItemCollection subItems5 = listView2.Items[count2].SubItems;
					Type typeFromHandle2 = typeof(Math);
					array7 = new object[1];
					object[] array10 = array7;
					dataRow = dataSet2.Tables[0].Rows[num7];
					DataRow dataRow6 = dataRow;
					columnName = "log_price";
					array10[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
					array3 = array7;
					object[] arguments5 = array3;
					array4 = new bool[1] { true };
					object obj2 = NewLateBinding.LateGet(null, typeFromHandle2, "Abs", arguments5, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					subItems5.Add(Strings.Format(RuntimeHelpers.GetObjectValue(obj2), "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(0, "#,##0.00"));
					ListViewItem.ListViewSubItemCollection subItems6 = listView2.Items[count2].SubItems;
					Type typeFromHandle3 = typeof(Math);
					array7 = new object[1];
					object[] array11 = array7;
					dataRow = dataSet2.Tables[0].Rows[num7];
					DataRow dataRow7 = dataRow;
					columnName = "log_price";
					array11[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array3 = array7;
					object[] arguments6 = array3;
					array4 = new bool[1] { true };
					object obj3 = NewLateBinding.LateGet(null, typeFromHandle3, "Abs", arguments6, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					subItems6.Add(Strings.Format(RuntimeHelpers.GetObjectValue(obj3), "#,##0.00"));
					listView2.Items[count2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num7]["log_Date"]), "dd/MM/yy HH:mm"));
					if (Operators.CompareString(left, "", TextCompare: false) != 0)
					{
						listView2.Items[count2].BackColor = Color.Yellow;
					}
					listView2.Items[count2].SubItems.Add("");
					listView2 = null;
					num7++;
					continue;
				}
				break;
			}
		}
	}

	private void Label4_Click(object sender, EventArgs e)
	{
	}

	private void ListView3_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void LinkLabel1_LinkClicked(object sender, LinkLabelLinkClickedEventArgs e)
	{
		checked
		{
			if (ListView2.Visible)
			{
				ListView2.Visible = false;
				Label1.Visible = false;
				ListView listView = ListView4;
				Point point = ListView2.Location;
				Point location = new Point(point.X, ListView2.Location.Y);
				listView.Location = location;
				ListView4.Width += 402;
				point = (Label5.Location = new Point(Label1.Location.X, Label1.Location.Y));
				LinkLabel1.Text = "(แสดงส\u0e34นค\u0e49า/บร\u0e34การ)";
			}
			else
			{
				ListView2.Visible = true;
				Label1.Visible = true;
				ListView listView2 = ListView4;
				Point point = new Point(ListView2.Location.X + 402, ListView2.Location.Y);
				listView2.Location = point;
				ListView4.Width -= 402;
				point = (Label5.Location = new Point(Label1.Location.X + 402, Label1.Location.Y));
				LinkLabel1.Text = "(ซ\u0e48อน)";
			}
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.Title = "รายงานการ Check-In/Check-Out";
		ListView1.PrintPreview();
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการใบลงทะเบ\u0e35ยน");
		}
		else
		{
			Print_Report.PrintFolio1(ListView1.SelectedItems[0].SubItems[1].Text);
		}
	}

	private void ToolStripMenuItem_1_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการใบลงทะเบ\u0e35ยน");
			return;
		}
		FormFolio formFolio = new FormFolio();
		formFolio.CIN_ID = ListView1.SelectedItems[0].SubItems[1].Text;
		formFolio.ShowDialog();
	}

	private void ToolStripMenuItem_2_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Receipt_H where status_name<>'ยกเล\u0e34ก' and  Receipt_Ref='" + ListView1.SelectedItems[0].SubItems[1].Text + "' order by id desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			DataSet dataSet2 = Module1.connect("select * from View_CheckIn_Ds where Cin_no='" + ListView1.SelectedItems[0].SubItems[1].Text + "'");
			MyProject.Forms.FrmAddSale.IEdit = (string)(object)0;
			MyProject.Forms.FrmAddSale.clear();
			MyProject.Forms.FrmAddSale.Tref.Text = Conversions.ToString(dataSet2.Tables[0].Rows[0]["Cin_no"]);
			MyProject.Forms.FrmAddSale.B2_Click(Conversions.ToString(dataSet2.Tables[0].Rows[0]["Cin_no"]));
			MyProject.Forms.FrmAddSale.Tnote.Text = "เข\u0e49าพ\u0e31ก ว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yy") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yy");
			MyProject.Forms.FrmAddSale.ShowDialog();
		}
		else
		{
			DataSet dataSet3 = Module1.connect("select * from View_CheckIn_Ds where Cin_no='" + ListView1.SelectedItems[0].SubItems[1].Text + "'");
			FormShowVAT formShowVAT = new FormShowVAT();
			formShowVAT.Label_NO.Text = Conversions.ToString(dataSet3.Tables[0].Rows[0]["Cin_no"]);
			formShowVAT.ShowDialog();
		}
	}

	private void ListView4_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void ToolStripMenuItem_3_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการใบลงทะเบ\u0e35ยน");
		}
		else
		{
			Print_Report.Print_Reg(ListView1.SelectedItems[0].SubItems[1].Text, preview: false);
		}
	}
}
