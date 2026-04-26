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
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmRegMain : Office2007Form
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
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader20")]
	private ColumnHeader _ColumnHeader20;

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

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

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

	[DebuggerNonUserCode]
	static FrmRegMain()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmRegMain()
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
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
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
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader20 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ExpandablePanel2.SuspendLayout();
		this.PanelEx2.SuspendLayout();
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
		this.PanelEx1.Text = "ใบลงทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก  |";
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
		size = new System.Drawing.Size(91, 420);
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
		this.ItemPanel2.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.LabelItem1, this.ItemContainer1 });
		this.ItemPanel2.LayoutOrientation = DevComponents.DotNetBar.eOrientation.Vertical;
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel2;
		location = new System.Drawing.Point(3, 33);
		itemPanel.Location = location;
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		itemPanel2.Margin = margin;
		this.ItemPanel2.Name = "ItemPanel2";
		DevComponents.DotNetBar.ItemPanel itemPanel3 = this.ItemPanel2;
		size = new System.Drawing.Size(85, 383);
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
		this.ItemContainer1.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem1 });
		this.ButtonItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem1.Image = iHOTEL2025.My.Resources.Resources.print1;
		this.ButtonItem1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.Text = "พ\u0e34มพ\u0e4c\r\nใบลงทะเบ\u0e35ยน";
		this.ButtonItem1.Tooltip = "พ\u0e34มพ\u0e4c";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.GroupBox1);
		this.PanelEx2.Controls.Add(this.ListView1);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		location = new System.Drawing.Point(91, 24);
		panelEx4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx5.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx2;
		size = new System.Drawing.Size(873, 420);
		panelEx6.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 110;
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
		size = new System.Drawing.Size(862, 85);
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
		System.Windows.Forms.Label label = this.Label11;
		location = new System.Drawing.Point(294, 50);
		label.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label2 = this.Label11;
		size = new System.Drawing.Size(50, 14);
		label2.Size = size;
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
		System.Windows.Forms.Label label3 = this.Label3;
		location = new System.Drawing.Point(74, 52);
		label3.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label4 = this.Label3;
		size = new System.Drawing.Size(63, 16);
		label4.Size = size;
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
		System.Windows.Forms.Label label5 = this.Label2;
		location = new System.Drawing.Point(20, 23);
		label5.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(117, 16);
		label6.Size = size;
		this.Label2.TabIndex = 1;
		this.Label2.Text = "เลขท\u0e35\u0e48ใบลงทะเบ\u0e35ยน :";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(503, 21);
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
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[9] { this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader6, this.ColumnHeader9, this.ColumnHeader12, this.ColumnHeader13, this.ColumnHeader20, this.ColumnHeader1, this.ColumnHeader2 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(7, 100);
		listView.Location = location;
		System.Windows.Forms.ListView listView2 = this.ListView1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView3 = this.ListView1;
		size = new System.Drawing.Size(862, 316);
		listView3.Size = size;
		this.ListView1.TabIndex = 15;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "";
		this.ColumnHeader3.Width = 0;
		this.ColumnHeader4.Text = "ท\u0e35\u0e48";
		this.ColumnHeader4.Width = 50;
		this.ColumnHeader9.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader9.Width = 140;
		this.ColumnHeader12.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader12.Width = 140;
		this.ColumnHeader13.Text = "ห\u0e49อง";
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader20.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.ColumnHeader20.Width = 250;
		this.ColumnHeader6.Text = "เลขท\u0e35\u0e48อ\u0e49างอ\u0e34ง";
		this.ColumnHeader6.Width = 100;
		this.ColumnHeader1.Text = "เบอร\u0e4cโทร";
		this.ColumnHeader1.Width = 100;
		this.ColumnHeader2.Text = "เบอร\u0e4cแฟกซ\u0e4c";
		this.ColumnHeader2.Width = 100;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(964, 444);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.ExpandablePanel2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmRegMain";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "ใบลงทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก";
		this.ExpandablePanel2.ResumeLayout(false);
		this.PanelEx2.ResumeLayout(false);
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
		Cursor = Cursors.WaitCursor;
		object left = "select * from View_CheckIn_H where Cin_no<>''";
		if (Operators.CompareString(TextBox_0.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and Cin_no like '%" + TextBox_0.Text, "%'"));
		}
		if (Operators.CompareString(Tcustname.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and Cust_name like '%" + Tcustname.Text, "%'"));
		}
		if (checkbox1.Checked)
		{
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" and (Cin_Date between '" + Conversions.ToString(DateTimePicker1.Value.Date), " 00:00:00' and '"), Conversions.ToString(DateTimePicker2.Value.Date)), " 23:59:59')"));
		}
		left = Operators.ConcatenateObject(left, " order by Cin_no desc");
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
				ListView1.Items.Add("");
				ListView1.Items[count].SubItems.Add(Conversions.ToString(dataSet.Tables[0].Rows.Count - num3));
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Cin_no"].ToString());
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["Cust_name"].ToString());
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["Cin_Date"]), "dd/MM/yyyy HH:mm:ss"));
				ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[count].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow2 = dataRow;
				string columnName = "Cin_Room_ALL";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
					.Replace("ถนน ", "")
					.Replace("เขต/อำเภอ ", "")
					.Replace("แขวง/ตำบล ", "")
					.Replace("จ\u0e31งหว\u0e31ด ", ""));
				ListViewItem.ListViewSubItemCollection subItems2 = ListView1.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow3 = dataRow;
				columnName = "Cust_Add_tel";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems3 = ListView1.Items[count].SubItems;
				array3 = new object[1];
				object[] array6 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow4 = dataRow;
				columnName = "Cust_Add_fax";
				array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
				array = array3;
				object[] arguments3 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				num3++;
			}
			Cursor = Cursors.Default;
		}
	}

	private void ButtonItem1_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการใบลงทะเบ\u0e35ยน");
		}
		else
		{
			Print_Report.Print_Reg(ListView1.SelectedItems[0].SubItems[2].Text, preview: false);
		}
	}

	private void checkbox1_CheckedChanged(object sender, EventArgs e)
	{
		DateTimePicker1.Enabled = checkbox1.Checked;
		DateTimePicker2.Enabled = checkbox1.Checked;
	}
}
