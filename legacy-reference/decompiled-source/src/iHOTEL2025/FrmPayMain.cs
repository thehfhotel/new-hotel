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
using DevComponents.DotNetBar.Validator;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmPayMain : Office2007Form
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

	[AccessedThroughProperty("ExpandablePanel2")]
	private ExpandablePanel _ExpandablePanel2;

	[AccessedThroughProperty("ItemPanel2")]
	private ItemPanel _ItemPanel2;

	[AccessedThroughProperty("LabelItem1")]
	private LabelItem _LabelItem1;

	[AccessedThroughProperty("ItemContainer1")]
	private ItemContainer _ItemContainer1;

	[AccessedThroughProperty("ButtonAdd")]
	private ButtonItem _ButtonAdd;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader14")]
	private ColumnHeader _ColumnHeader14;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ButtonItem2")]
	private ButtonItem _ButtonItem2;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("checkbox1")]
	private CheckBox _checkbox1;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("ButtonCancel")]
	private ButtonItem _ButtonCancel;

	[AccessedThroughProperty("GroupPanel2")]
	private GroupPanel _GroupPanel2;

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("GroupPanel1")]
	private GroupPanel _GroupPanel1;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("TextBox2")]
	private TextBox _TextBox2;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("PanelEx3")]
	private PanelEx _PanelEx3;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TextBox3")]
	private TextBox _TextBox3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("LabelItem2")]
	private LabelItem _LabelItem2;

	[AccessedThroughProperty("ItemContainer2")]
	private ItemContainer _ItemContainer2;

	[AccessedThroughProperty("ButtonItem3")]
	private ButtonItem _ButtonItem3;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader15")]
	private ColumnHeader _ColumnHeader15;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("LabelItem3")]
	private LabelItem _LabelItem3;

	[AccessedThroughProperty("ItemContainer3")]
	private ItemContainer _ItemContainer3;

	[AccessedThroughProperty("ButtonItem4")]
	private ButtonItem _ButtonItem4;

	[AccessedThroughProperty("ButtonItem6")]
	private ButtonItem _ButtonItem6;

	[AccessedThroughProperty("ComboBox3")]
	private ComboBox _ComboBox3;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("ButtonItem7")]
	private ButtonItem _ButtonItem7;

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

	internal virtual ExpandablePanel ExpandablePanel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ExpandablePanel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ExpandablePanel2 = value;
		}
	}

	internal virtual ItemPanel ItemPanel2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemPanel2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemPanel2 = value;
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

	internal virtual ItemContainer ItemContainer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer1 = value;
		}
	}

	internal virtual ButtonItem ButtonAdd
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonAdd;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonAdd_Click;
			if (_ButtonAdd != null)
			{
				_ButtonAdd.Click -= value2;
			}
			_ButtonAdd = value;
			if (_ButtonAdd != null)
			{
				_ButtonAdd.Click += value2;
			}
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
			_ListView1 = value;
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
			_checkbox1 = value;
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
			_ButtonItem1 = value;
		}
	}

	internal virtual ButtonItem ButtonCancel
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonCancel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ButtonCancel = value;
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
			_TextBox1 = value;
		}
	}

	internal virtual TextBox TextBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox2 = value;
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

	internal virtual TextBox TextBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox3 = value;
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

	internal virtual ItemContainer ItemContainer2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer2 = value;
		}
	}

	internal virtual ButtonItem ButtonItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem3_Click;
			if (_ButtonItem3 != null)
			{
				_ButtonItem3.Click -= value2;
			}
			_ButtonItem3 = value;
			if (_ButtonItem3 != null)
			{
				_ButtonItem3.Click += value2;
			}
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

	internal virtual ComboBox ComboBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox2_SelectedIndexChanged;
			if (_ComboBox2 != null)
			{
				_ComboBox2.SelectedIndexChanged -= value2;
			}
			_ComboBox2 = value;
			if (_ComboBox2 != null)
			{
				_ComboBox2.SelectedIndexChanged += value2;
			}
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

	internal virtual LabelItem LabelItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem3 = value;
		}
	}

	internal virtual ItemContainer ItemContainer3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ItemContainer3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ItemContainer3 = value;
		}
	}

	internal virtual ButtonItem ButtonItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem4_Click;
			if (_ButtonItem4 != null)
			{
				_ButtonItem4.Click -= value2;
			}
			_ButtonItem4 = value;
			if (_ButtonItem4 != null)
			{
				_ButtonItem4.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem6_Click;
			if (_ButtonItem6 != null)
			{
				_ButtonItem6.Click -= value2;
			}
			_ButtonItem6 = value;
			if (_ButtonItem6 != null)
			{
				_ButtonItem6.Click += value2;
			}
		}
	}

	internal virtual ComboBox ComboBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox3 = value;
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

	internal virtual ButtonItem ButtonItem7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem7_Click;
			if (_ButtonItem7 != null)
			{
				_ButtonItem7.Click -= value2;
			}
			_ButtonItem7 = value;
			if (_ButtonItem7 != null)
			{
				_ButtonItem7.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmPayMain()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmPayMain()
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmPayMain));
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
		this.ExpandablePanel2 = new DevComponents.DotNetBar.ExpandablePanel();
		this.ItemPanel2 = new DevComponents.DotNetBar.ItemPanel();
		this.LabelItem1 = new DevComponents.DotNetBar.LabelItem();
		this.ItemContainer1 = new DevComponents.DotNetBar.ItemContainer();
		this.ButtonAdd = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem2 = new DevComponents.DotNetBar.ButtonItem();
		this.LabelItem2 = new DevComponents.DotNetBar.LabelItem();
		this.ItemContainer2 = new DevComponents.DotNetBar.ItemContainer();
		this.ButtonItem3 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem7 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem6 = new DevComponents.DotNetBar.ButtonItem();
		this.LabelItem3 = new DevComponents.DotNetBar.LabelItem();
		this.ItemContainer3 = new DevComponents.DotNetBar.ItemContainer();
		this.ButtonItem4 = new DevComponents.DotNetBar.ButtonItem();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx3 = new DevComponents.DotNetBar.PanelEx();
		this.Label1 = new System.Windows.Forms.Label();
		this.TextBox3 = new System.Windows.Forms.TextBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.TextBox2 = new System.Windows.Forms.TextBox();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.GroupPanel2 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader15 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.GroupPanel1 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader14 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ComboBox3 = new System.Windows.Forms.ComboBox();
		this.Label7 = new System.Windows.Forms.Label();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.checkbox1 = new System.Windows.Forms.CheckBox();
		this.Label11 = new System.Windows.Forms.Label();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonCancel = new DevComponents.DotNetBar.ButtonItem();
		this.ExpandablePanel2.SuspendLayout();
		this.PanelEx2.SuspendLayout();
		this.PanelEx3.SuspendLayout();
		this.GroupPanel2.SuspendLayout();
		this.GroupPanel1.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.TimeMaxzimize.Enabled = true;
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
		System.Drawing.Size size = new System.Drawing.Size(1018, 24);
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
		this.PanelEx1.Text = "รายร\u0e31บ-รายจ\u0e48าย  |";
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
		this.ExpandablePanel2.AnimationTime = 1;
		this.ExpandablePanel2.CanvasColor = System.Drawing.SystemColors.Control;
		this.ExpandablePanel2.CollapseDirection = DevComponents.DotNetBar.eCollapseDirection.RightToLeft;
		this.ExpandablePanel2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.ExpandablePanel2.Controls.Add(this.ItemPanel2);
		this.ExpandablePanel2.Dock = System.Windows.Forms.DockStyle.Left;
		this.ExpandablePanel2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ExpandablePanel expandablePanel = this.ExpandablePanel2;
		location = new System.Drawing.Point(0, 24);
		expandablePanel.Location = location;
		DevComponents.DotNetBar.ExpandablePanel expandablePanel2 = this.ExpandablePanel2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		expandablePanel2.Margin = margin;
		this.ExpandablePanel2.Name = "ExpandablePanel2";
		DevComponents.DotNetBar.ExpandablePanel expandablePanel3 = this.ExpandablePanel2;
		size = new System.Drawing.Size(91, 567);
		expandablePanel3.Size = size;
		this.ExpandablePanel2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.ExpandablePanel2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.ExpandablePanel2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.ExpandablePanel2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.ExpandablePanel2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.BarDockedBorder;
		this.ExpandablePanel2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.ItemText;
		this.ExpandablePanel2.Style.GradientAngle = 90;
		this.ExpandablePanel2.TabIndex = 109;
		this.ExpandablePanel2.TitleHeight = 32;
		this.ExpandablePanel2.TitleStyle.Alignment = System.Drawing.StringAlignment.Center;
		this.ExpandablePanel2.TitleStyle.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.ExpandablePanel2.TitleStyle.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.ExpandablePanel2.TitleStyle.Border = DevComponents.DotNetBar.eBorderType.RaisedInner;
		this.ExpandablePanel2.TitleStyle.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.ExpandablePanel2.TitleStyle.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.ExpandablePanel2.TitleStyle.GradientAngle = 90;
		this.ExpandablePanel2.TitleText = "Options";
		this.ItemPanel2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ItemPanel2.AutoScroll = true;
		this.ItemPanel2.BackgroundStyle.BackColor = System.Drawing.Color.White;
		this.ItemPanel2.BackgroundStyle.BorderBottom = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel2.BackgroundStyle.BorderBottomWidth = 1;
		this.ItemPanel2.BackgroundStyle.BorderColor = System.Drawing.Color.FromArgb(127, 157, 185);
		this.ItemPanel2.BackgroundStyle.BorderLeft = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel2.BackgroundStyle.BorderLeftWidth = 1;
		this.ItemPanel2.BackgroundStyle.BorderRight = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel2.BackgroundStyle.BorderRightWidth = 1;
		this.ItemPanel2.BackgroundStyle.BorderTop = DevComponents.DotNetBar.eStyleBorderType.Solid;
		this.ItemPanel2.BackgroundStyle.BorderTopWidth = 1;
		this.ItemPanel2.BackgroundStyle.Class = "";
		this.ItemPanel2.BackgroundStyle.PaddingBottom = 1;
		this.ItemPanel2.BackgroundStyle.PaddingLeft = 1;
		this.ItemPanel2.BackgroundStyle.PaddingRight = 1;
		this.ItemPanel2.BackgroundStyle.PaddingTop = 1;
		this.ItemPanel2.ContainerControlProcessDialogKey = true;
		this.ItemPanel2.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ItemPanel2.Items.AddRange(new DevComponents.DotNetBar.BaseItem[6] { this.LabelItem1, this.ItemContainer1, this.LabelItem2, this.ItemContainer2, this.LabelItem3, this.ItemContainer3 });
		this.ItemPanel2.LayoutOrientation = DevComponents.DotNetBar.eOrientation.Vertical;
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel2;
		location = new System.Drawing.Point(3, 33);
		itemPanel.Location = location;
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		itemPanel2.Margin = margin;
		this.ItemPanel2.Name = "ItemPanel2";
		DevComponents.DotNetBar.ItemPanel itemPanel3 = this.ItemPanel2;
		size = new System.Drawing.Size(85, 530);
		itemPanel3.Size = size;
		this.ItemPanel2.TabIndex = 40;
		this.ItemPanel2.Text = "ItemPanel2";
		this.LabelItem1.BackColor = System.Drawing.Color.FromArgb(235, 235, 235);
		this.LabelItem1.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.LabelItem1.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.LabelItem1.Name = "LabelItem1";
		this.LabelItem1.PaddingBottom = 1;
		this.LabelItem1.PaddingLeft = 1;
		this.LabelItem1.PaddingRight = 1;
		this.LabelItem1.PaddingTop = 1;
		this.LabelItem1.SingleLineColor = System.Drawing.Color.FromArgb(197, 197, 197);
		this.LabelItem1.Text = "<b>Tools - หล\u0e31ก</b>";
		this.ItemContainer1.BackgroundStyle.Class = "";
		this.ItemContainer1.HorizontalItemAlignment = DevComponents.DotNetBar.eHorizontalItemsAlignment.Center;
		this.ItemContainer1.ItemSpacing = 0;
		this.ItemContainer1.MultiLine = true;
		this.ItemContainer1.Name = "ItemContainer1";
		this.ItemContainer1.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonAdd, this.ButtonItem2 });
		this.ButtonAdd.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonAdd.Image = (System.Drawing.Image)resources.GetObject("ButtonAdd.Image");
		this.ButtonAdd.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonAdd.Name = "ButtonAdd";
		this.ButtonAdd.Text = "เพ\u0e34\u0e48มรายร\u0e31บ";
		this.ButtonItem2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem2.Image = (System.Drawing.Image)resources.GetObject("ButtonItem2.Image");
		this.ButtonItem2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem2.Name = "ButtonItem2";
		this.ButtonItem2.Text = "เพ\u0e34\u0e48มรายจ\u0e48าย";
		this.ButtonItem2.Tooltip = "แก\u0e49ไข";
		this.LabelItem2.BackColor = System.Drawing.Color.FromArgb(235, 235, 235);
		this.LabelItem2.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.LabelItem2.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.LabelItem2.Name = "LabelItem2";
		this.LabelItem2.PaddingBottom = 1;
		this.LabelItem2.PaddingLeft = 1;
		this.LabelItem2.PaddingRight = 1;
		this.LabelItem2.PaddingTop = 1;
		this.LabelItem2.SingleLineColor = System.Drawing.Color.FromArgb(197, 197, 197);
		this.LabelItem2.Text = "<b>ต\u0e31\u0e49งค\u0e48า</b>";
		this.ItemContainer2.BackgroundStyle.Class = "";
		this.ItemContainer2.HorizontalItemAlignment = DevComponents.DotNetBar.eHorizontalItemsAlignment.Center;
		this.ItemContainer2.ItemSpacing = 0;
		this.ItemContainer2.MultiLine = true;
		this.ItemContainer2.Name = "ItemContainer2";
		this.ItemContainer2.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[3] { this.ButtonItem3, this.ButtonItem7, this.ButtonItem6 });
		this.ButtonItem3.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem3.Image = (System.Drawing.Image)resources.GetObject("ButtonItem3.Image");
		this.ButtonItem3.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem3.Name = "ButtonItem3";
		this.ButtonItem3.Text = "ต\u0e31\u0e49งค\u0e48าหมวด";
		this.ButtonItem7.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem7.Image = (System.Drawing.Image)resources.GetObject("ButtonItem7.Image");
		this.ButtonItem7.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem7.Name = "ButtonItem7";
		this.ButtonItem7.Text = "ต\u0e31\u0e49งค\u0e48าประเภท";
		this.ButtonItem6.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem6.Image = (System.Drawing.Image)resources.GetObject("ButtonItem6.Image");
		this.ButtonItem6.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem6.Name = "ButtonItem6";
		this.ButtonItem6.Text = "ต\u0e31\u0e49งค\u0e48ารห\u0e31สบ\u0e31ญช\u0e35";
		this.LabelItem3.BackColor = System.Drawing.Color.FromArgb(235, 235, 235);
		this.LabelItem3.BorderSide = DevComponents.DotNetBar.eBorderSide.Bottom;
		this.LabelItem3.BorderType = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.LabelItem3.Name = "LabelItem3";
		this.LabelItem3.PaddingBottom = 1;
		this.LabelItem3.PaddingLeft = 1;
		this.LabelItem3.PaddingRight = 1;
		this.LabelItem3.PaddingTop = 1;
		this.LabelItem3.SingleLineColor = System.Drawing.Color.FromArgb(197, 197, 197);
		this.LabelItem3.Text = "<b>รายงาน</b>";
		this.ItemContainer3.BackgroundStyle.Class = "";
		this.ItemContainer3.HorizontalItemAlignment = DevComponents.DotNetBar.eHorizontalItemsAlignment.Center;
		this.ItemContainer3.ItemSpacing = 0;
		this.ItemContainer3.MultiLine = true;
		this.ItemContainer3.Name = "ItemContainer3";
		this.ItemContainer3.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem4 });
		this.ButtonItem4.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem4.Image = (System.Drawing.Image)resources.GetObject("ButtonItem4.Image");
		this.ButtonItem4.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem4.Name = "ButtonItem4";
		this.ButtonItem4.Text = "พ\u0e34มพ\u0e4cรายงาน";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.PanelEx3);
		this.PanelEx2.Controls.Add(this.GroupPanel2);
		this.PanelEx2.Controls.Add(this.GroupPanel1);
		this.PanelEx2.Controls.Add(this.GroupBox1);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		location = new System.Drawing.Point(91, 24);
		panelEx4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx5.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx2;
		size = new System.Drawing.Size(927, 567);
		panelEx6.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 110;
		this.PanelEx3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.PanelEx3.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx3.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx3.Controls.Add(this.Label1);
		this.PanelEx3.Controls.Add(this.TextBox3);
		this.PanelEx3.Controls.Add(this.Label4);
		this.PanelEx3.Controls.Add(this.TextBox2);
		this.PanelEx3.Controls.Add(this.TextBox1);
		this.PanelEx3.Controls.Add(this.Label3);
		DevComponents.DotNetBar.PanelEx panelEx7 = this.PanelEx3;
		location = new System.Drawing.Point(7, 520);
		panelEx7.Location = location;
		this.PanelEx3.Name = "PanelEx3";
		DevComponents.DotNetBar.PanelEx panelEx8 = this.PanelEx3;
		size = new System.Drawing.Size(994, 42);
		panelEx8.Size = size;
		this.PanelEx3.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx3.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx3.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx3.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx3.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx3.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx3.Style.GradientAngle = 90;
		this.PanelEx3.TabIndex = 18;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(513, 10);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(101, 23);
		label2.Size = size;
		this.Label1.TabIndex = 16;
		this.Label1.Text = "รวมส\u0e48วนต\u0e48าง";
		this.TextBox3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.TextBox3.BackColor = System.Drawing.Color.Black;
		this.TextBox3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox3.ForeColor = System.Drawing.Color.Cyan;
		System.Windows.Forms.TextBox textBox = this.TextBox3;
		location = new System.Drawing.Point(616, 7);
		textBox.Location = location;
		this.TextBox3.Name = "TextBox3";
		this.TextBox3.ReadOnly = true;
		System.Windows.Forms.TextBox textBox2 = this.TextBox3;
		size = new System.Drawing.Size(152, 30);
		textBox2.Size = size;
		this.TextBox3.TabIndex = 17;
		this.TextBox3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label4.AutoSize = true;
		this.Label4.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label3 = this.Label4;
		location = new System.Drawing.Point(251, 10);
		label3.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label4 = this.Label4;
		size = new System.Drawing.Size(97, 23);
		label4.Size = size;
		this.Label4.TabIndex = 16;
		this.Label4.Text = "รวมรายจ\u0e48าย";
		this.TextBox2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.TextBox2.BackColor = System.Drawing.Color.Black;
		this.TextBox2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox2.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.TextBox textBox3 = this.TextBox2;
		location = new System.Drawing.Point(354, 7);
		textBox3.Location = location;
		this.TextBox2.Name = "TextBox2";
		this.TextBox2.ReadOnly = true;
		System.Windows.Forms.TextBox textBox4 = this.TextBox2;
		size = new System.Drawing.Size(152, 30);
		textBox4.Size = size;
		this.TextBox2.TabIndex = 17;
		this.TextBox2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.TextBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.TextBox1.BackColor = System.Drawing.Color.Black;
		this.TextBox1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox1.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.TextBox textBox5 = this.TextBox1;
		location = new System.Drawing.Point(97, 7);
		textBox5.Location = location;
		this.TextBox1.Name = "TextBox1";
		this.TextBox1.ReadOnly = true;
		System.Windows.Forms.TextBox textBox6 = this.TextBox1;
		size = new System.Drawing.Size(148, 30);
		textBox6.Size = size;
		this.TextBox1.TabIndex = 17;
		this.TextBox1.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label3.AutoSize = true;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(11, 10);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(86, 23);
		label6.Size = size;
		this.Label3.TabIndex = 16;
		this.Label3.Text = "รวมรายร\u0e31บ";
		this.GroupPanel2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel2.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel2.Controls.Add(this.ButtonX3);
		this.GroupPanel2.Controls.Add(this.ListView2);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel = this.GroupPanel2;
		location = new System.Drawing.Point(6, 325);
		groupPanel.Location = location;
		this.GroupPanel2.Name = "GroupPanel2";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel2 = this.GroupPanel2;
		size = new System.Drawing.Size(916, 187);
		groupPanel2.Size = size;
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
		this.GroupPanel2.TabIndex = 17;
		this.GroupPanel2.Text = "รายจ\u0e48าย";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX3;
		location = new System.Drawing.Point(4, 158);
		buttonX.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX3;
		size = new System.Drawing.Size(75, 23);
		buttonX2.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 18;
		this.ButtonX3.Text = "ลบ";
		this.ListView2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView2.CheckBoxes = true;
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[8] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader5, this.ColumnHeader11, this.ColumnHeader15, this.ColumnHeader6, this.ColumnHeader7, this.ColumnHeader8 });
		this.ListView2.FullRowSelect = true;
		this.ListView2.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView2;
		location = new System.Drawing.Point(4, 4);
		listView.Location = location;
		System.Windows.Forms.ListView listView2 = this.ListView2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView2.MultiSelect = false;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView3 = this.ListView2;
		size = new System.Drawing.Size(903, 147);
		listView3.Size = size;
		this.ListView2.TabIndex = 15;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "";
		this.ColumnHeader1.Width = 0;
		this.ColumnHeader2.Text = "ท\u0e35\u0e48";
		this.ColumnHeader2.Width = 40;
		this.ColumnHeader5.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader5.Width = 120;
		this.ColumnHeader11.Text = "หมวด";
		this.ColumnHeader11.Width = 110;
		this.ColumnHeader15.Text = "โค\u0e4aดบ\u0e31ญช\u0e35";
		this.ColumnHeader15.Width = 120;
		this.ColumnHeader6.Text = "หมายเหต\u0e38";
		this.ColumnHeader6.Width = 200;
		this.ColumnHeader7.Text = "จำนวนเง\u0e34น";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 100;
		this.ColumnHeader8.Text = "จากโปรแกรม";
		this.ColumnHeader8.Width = 0;
		this.GroupPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel1.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel1.Controls.Add(this.ButtonX2);
		this.GroupPanel1.Controls.Add(this.ListView1);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel3 = this.GroupPanel1;
		location = new System.Drawing.Point(6, 98);
		groupPanel3.Location = location;
		this.GroupPanel1.Name = "GroupPanel1";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel4 = this.GroupPanel1;
		size = new System.Drawing.Size(916, 222);
		groupPanel4.Size = size;
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
		this.GroupPanel1.TabIndex = 16;
		this.GroupPanel1.Text = "รายร\u0e31บ";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(3, 193);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(75, 23);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 18;
		this.ButtonX2.Text = "ลบ";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[8] { this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader12, this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader13, this.ColumnHeader14, this.ColumnHeader16 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView4 = this.ListView1;
		location = new System.Drawing.Point(3, 4);
		listView4.Location = location;
		System.Windows.Forms.ListView listView5 = this.ListView1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView5.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView6 = this.ListView1;
		size = new System.Drawing.Size(904, 182);
		listView6.Size = size;
		this.ListView1.TabIndex = 15;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "";
		this.ColumnHeader3.Width = 0;
		this.ColumnHeader4.Text = "ท\u0e35\u0e48";
		this.ColumnHeader4.Width = 40;
		this.ColumnHeader12.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader12.Width = 120;
		this.ColumnHeader9.Text = "หมวด";
		this.ColumnHeader9.Width = 110;
		this.ColumnHeader10.Text = "โค\u0e4aดบ\u0e31ญช\u0e35";
		this.ColumnHeader10.Width = 120;
		this.ColumnHeader13.Text = "หมายเหต\u0e38";
		this.ColumnHeader13.Width = 200;
		this.ColumnHeader14.Text = "จำนวนเง\u0e34น";
		this.ColumnHeader14.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader14.Width = 100;
		this.ColumnHeader16.Text = "จากโปรแกรม";
		this.ColumnHeader16.Width = 0;
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.ComboBox3);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.ComboBox2);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.checkbox1);
		this.GroupBox1.Controls.Add(this.Label11);
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
		size = new System.Drawing.Size(916, 85);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 14;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.ComboBox3.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox3.DropDownWidth = 250;
		this.ComboBox3.FormattingEnabled = true;
		this.ComboBox3.Items.AddRange(new object[2] { "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox3;
		location = new System.Drawing.Point(294, 50);
		comboBox.Location = location;
		this.ComboBox3.Name = "ComboBox3";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox3;
		size = new System.Drawing.Size(141, 24);
		comboBox2.Size = size;
		this.ComboBox3.TabIndex = 94;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label7;
		location = new System.Drawing.Point(235, 54);
		label7.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label8 = this.Label7;
		size = new System.Drawing.Size(57, 16);
		label8.Size = size;
		this.Label7.TabIndex = 93;
		this.Label7.Text = "รห\u0e31สบ\u0e31ญช\u0e35";
		this.ComboBox2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[2] { "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox2;
		location = new System.Drawing.Point(294, 22);
		comboBox3.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox2;
		size = new System.Drawing.Size(141, 24);
		comboBox4.Size = size;
		this.ComboBox2.TabIndex = 92;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(254, 26);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(38, 16);
		label10.Size = size;
		this.Label5.TabIndex = 91;
		this.Label5.Text = "หมวด";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(72, 48);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(150, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 89;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(72, 22);
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
		location = new System.Drawing.Point(16, 23);
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
		System.Windows.Forms.Label label11 = this.Label11;
		location = new System.Drawing.Point(19, 51);
		label11.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label12 = this.Label11;
		size = new System.Drawing.Size(50, 14);
		label12.Size = size;
		this.Label11.TabIndex = 87;
		this.Label11.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX1;
		location = new System.Drawing.Point(454, 22);
		buttonX5.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX6.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX1;
		size = new System.Drawing.Size(115, 49);
		buttonX7.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 0;
		this.ButtonX1.Text = "ค\u0e49นหา";
		this.ButtonItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem1.Image = (System.Drawing.Image)resources.GetObject("ButtonItem1.Image");
		this.ButtonItem1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonCancel });
		this.ButtonItem1.Text = "พ\u0e34มพ\u0e4c\r\nใบเสนอราคา";
		this.ButtonItem1.Tooltip = "พ\u0e34มพ\u0e4c";
		this.ButtonCancel.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonCancel.Image = (System.Drawing.Image)resources.GetObject("ButtonCancel.Image");
		this.ButtonCancel.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonCancel.Name = "ButtonCancel";
		this.ButtonCancel.Text = "ยกเล\u0e34ก\r\nใบเสนอราคา";
		this.ButtonCancel.Tooltip = "ยกเล\u0e34ก";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1018, 591);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.ExpandablePanel2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmPayMain";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "รายร\u0e31บ-รายจ\u0e48าย";
		this.ExpandablePanel2.ResumeLayout(false);
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx3.ResumeLayout(false);
		this.PanelEx3.PerformLayout();
		this.GroupPanel2.ResumeLayout(false);
		this.GroupPanel1.ResumeLayout(false);
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmSaleMain_Load(object sender, EventArgs e)
	{
		MyProject.Application.ChangeCulture("en-US");
		loadGroup();
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Month) + "/01/" + Conversions.ToString(DateTime.Now.Year));
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Month) + "/" + Conversions.ToString(DateTime.DaysInMonth(DateTime.Now.Year, DateTime.Now.Month)) + "/" + Conversions.ToString(DateTime.Now.Year));
	}

	public void loadGroup()
	{
		ComboBox2.Items.Clear();
		ComboBox2.Items.Add("");
		DataSet dataSet = Module1.connect("select * from TB_SET_MyType2 order by id_full");
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
				ComboBox2.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			ComboBox3.Text = "";
		}
	}

	public void loadGroup2()
	{
	}

	private void TimeMaxzimize_Tick(object sender, EventArgs e)
	{
		TimeMaxzimize.Enabled = false;
		WindowState = FormWindowState.Normal;
		WindowState = FormWindowState.Maximized;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		SearchRec();
		SearchPay();
	}

	public void SearchPay()
	{
		DateTime dateTime = Conversions.ToDate(Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00");
		DateTime dateTime2 = Conversions.ToDate(Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59");
		string text = "";
		checked
		{
			if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) != 0)
			{
				text = ComboBox2.Text.Substring(ComboBox2.Text.IndexOf("|") + 2);
			}
			string text2 = "";
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text2 = "and Pay_Group='" + text + "'";
			}
			if (Operators.CompareString(ComboBox3.Text, "", TextCompare: false) != 0)
			{
				text2 = text2 + "and Pay_Account like '" + ComboBox3.Text.Substring(0, ComboBox3.Text.IndexOf("|")) + "-%'";
			}
			DataSet dataSet = Module1.connect("select * from tb_pay_history where pay_type='รายจ\u0e48าย' " + text2 + " and (pay_date between " + Conversions.ToString(dateTime.ToOADate()) + " and " + Conversions.ToString(dateTime2.ToOADate()) + ") order by id");
			ListView2.Items.Clear();
			decimal num = default(decimal);
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
				ListView listView = ListView2;
				int count = listView.Items.Count;
				ListView.ListViewItemCollection items = listView.Items;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num3];
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
				listView.Items[count].SubItems.Add(Conversions.ToString(num3 + 1));
				listView.Items[count].SubItems.Add(Strings.Format(DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num3]["Pay_date"])), "dd/MM/yy HH:mm"));
				listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Pay_Group"].ToString());
				listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Pay_Account"].ToString());
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow3 = dataRow;
				columnName = "Pay_Bill";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Pay_TOtal"]), "#,##0.00"));
				listView = null;
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet.Tables[0].Rows[num3]["Pay_TOtal"]));
				num3++;
			}
			TextBox2.Text = Strings.Format(num, "#,##0.00");
			SUMALL();
		}
	}

	public void SUMALL()
	{
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) == 0)
		{
			TextBox1.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBox1.Text))
		{
			TextBox1.Text = Conversions.ToString(0);
		}
		if (Operators.CompareString(TextBox2.Text, "", TextCompare: false) == 0)
		{
			TextBox2.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(TextBox2.Text))
		{
			TextBox2.Text = Conversions.ToString(0);
		}
		if (decimal.Compare(decimal.Subtract(Conversions.ToDecimal(TextBox1.Text), Conversions.ToDecimal(TextBox2.Text)), 0m) < 0)
		{
			TextBox3.ForeColor = Color.Red;
		}
		else
		{
			TextBox3.ForeColor = Color.Cyan;
		}
		TextBox3.Text = Strings.Format(decimal.Subtract(Conversions.ToDecimal(TextBox1.Text), Conversions.ToDecimal(TextBox2.Text)), "#,##0.00");
	}

	public void SearchRec()
	{
		DateTime dateTime = Conversions.ToDate(Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00");
		DateTime dateTime2 = Conversions.ToDate(Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59");
		string text = "";
		checked
		{
			if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) != 0)
			{
				text = ComboBox2.Text.Substring(ComboBox2.Text.IndexOf("|") + 2);
			}
			string text2 = "";
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text2 = "and Pay_Group='" + text + "'";
			}
			if (Operators.CompareString(ComboBox3.Text, "", TextCompare: false) != 0)
			{
				text2 = text2 + "and Pay_Account like '" + ComboBox3.Text.Substring(0, ComboBox3.Text.IndexOf("|")) + "-%'";
			}
			DataSet dataSet = Module1.connect("select * from tb_pay_history where pay_type='รายร\u0e31บ'  " + text2 + " and (pay_date between " + Conversions.ToString(dateTime.ToOADate()) + " and " + Conversions.ToString(dateTime2.ToOADate()) + ") order by id");
			ListView1.Items.Clear();
			decimal num = default(decimal);
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
				ListView listView = ListView1;
				int count = listView.Items.Count;
				ListView.ListViewItemCollection items = listView.Items;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num3];
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
				listView.Items[count].SubItems.Add(Conversions.ToString(num3 + 1));
				listView.Items[count].SubItems.Add(Strings.Format(DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num3]["Pay_date"])), "dd/MM/yy HH:mm"));
				listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Pay_Group"].ToString());
				listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Pay_Account"].ToString());
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow3 = dataRow;
				columnName = "Pay_Bill";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Pay_TOtal"]), "#,##0.00"));
				listView = null;
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet.Tables[0].Rows[num3]["Pay_TOtal"]));
				num3++;
			}
			TextBox1.Text = Strings.Format(num, "#,##0.00");
			SUMALL();
		}
	}

	private void ButtonAdd_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmAddPay.ComboBox1.SelectedIndex = 0;
		MyProject.Forms.FrmAddPay.ShowDialog();
		SearchRec();
		SearchPay();
	}

	private void ButtonItem2_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmAddPay.ComboBox1.SelectedIndex = 1;
		MyProject.Forms.FrmAddPay.ShowDialog();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		if (ListView1.Items.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการท\u0e35\u0e48จะลบ");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการจะลบรายการท\u0e35\u0e48เล\u0e37อกหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("delete from tb_pay_history where id=" + ListView1.SelectedItems[0].SubItems[0].Text);
			Module1.connect("update TB_Sale_H set Sale_Pay_total=Sale_Pay_total-" + Conversions.ToString(Conversions.ToDecimal(ListView1.SelectedItems[0].SubItems[6].Text)) + " , Sale_Pay_Status=2 where Sale_No='" + ListView1.SelectedItems[0].SubItems[5].Text + "'");
			MessageBox.Show("ลบรายการท\u0e35\u0e48เล\u0e37อกเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			SearchRec();
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		if (ListView2.Items.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการท\u0e35\u0e48จะลบ");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการจะลบรายการท\u0e35\u0e48เล\u0e37อกหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("delete from tb_pay_history where id=" + ListView2.SelectedItems[0].SubItems[0].Text);
			MessageBox.Show("ลบรายการท\u0e35\u0e48เล\u0e37อกเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			SearchPay();
		}
	}

	private void ButtonItem3_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSETMyType2.ShowDialog();
		loadGroup();
	}

	private void ButtonItem4_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmReportRecPay.ShowDialog();
	}

	private void ButtonItem6_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSETMyType3.ShowDialog();
	}

	private void ComboBox2_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			loadAccount();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public void loadAccount()
	{
		ComboBox3.Items.Clear();
		ComboBox3.Items.Add("");
		object obj = "";
		if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) == 0)
		{
			return;
		}
		obj = " where id_full like '" + ComboBox2.Text.Substring(0, ComboBox2.Text.IndexOf("|")) + "%' ";
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from TB_SET_MyType3 ", obj), " order by id_full")));
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
					ComboBox3.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void ButtonItem7_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSETMyType2_2.ShowDialog();
		loadGroup2();
	}
}
