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
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmManageCustomers : Office2007Form
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

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("Rno")]
	private TextBoxX _Rno;

	[AccessedThroughProperty("Remail")]
	private TextBoxX _Remail;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Rtype")]
	private ComboBox _Rtype;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("Rname")]
	private TextBoxX _Rname;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("G1_Fax")]
	private TextBoxX _G1_Fax;

	[AccessedThroughProperty("G1_Tel")]
	private TextBoxX _G1_Tel;

	[AccessedThroughProperty("Rname2")]
	private TextBoxX _Rname2;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

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

	[AccessedThroughProperty("G1_ampore")]
	private TextBoxX _G1_ampore;

	[AccessedThroughProperty("G1_Tambon")]
	private TextBoxX _G1_Tambon;

	[AccessedThroughProperty("G1_Road")]
	private TextBoxX _G1_Road;

	[AccessedThroughProperty("G1_Soi")]
	private TextBoxX _G1_Soi;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("G1_Moo")]
	private TextBoxX _G1_Moo;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("G1_No")]
	private TextBoxX _G1_No;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("G1_Code")]
	private TextBoxX _G1_Code;

	[AccessedThroughProperty("G1_Province")]
	private TextBoxX _G1_Province;

	[AccessedThroughProperty("Label16")]
	private Label _Label16;

	[AccessedThroughProperty("Label15")]
	private Label _Label15;

	[AccessedThroughProperty("G2")]
	private GroupBox _G2;

	[AccessedThroughProperty("G2_No")]
	private TextBoxX _G2_No;

	[AccessedThroughProperty("Label17")]
	private Label _Label17;

	[AccessedThroughProperty("Label18")]
	private Label _Label18;

	[AccessedThroughProperty("G2_Fax")]
	private TextBoxX _G2_Fax;

	[AccessedThroughProperty("Label19")]
	private Label _Label19;

	[AccessedThroughProperty("G2_Tel")]
	private TextBoxX _G2_Tel;

	[AccessedThroughProperty("Label20")]
	private Label _Label20;

	[AccessedThroughProperty("Label21")]
	private Label _Label21;

	[AccessedThroughProperty("Label22")]
	private Label _Label22;

	[AccessedThroughProperty("Label23")]
	private Label _Label23;

	[AccessedThroughProperty("G2_Code")]
	private TextBoxX _G2_Code;

	[AccessedThroughProperty("G2_Moo")]
	private TextBoxX _G2_Moo;

	[AccessedThroughProperty("G2_ampore")]
	private TextBoxX _G2_ampore;

	[AccessedThroughProperty("Label24")]
	private Label _Label24;

	[AccessedThroughProperty("G2_Province")]
	private TextBoxX _G2_Province;

	[AccessedThroughProperty("Label25")]
	private Label _Label25;

	[AccessedThroughProperty("G2_Tambon")]
	private TextBoxX _G2_Tambon;

	[AccessedThroughProperty("G2_Soi")]
	private TextBoxX _G2_Soi;

	[AccessedThroughProperty("G2_Road")]
	private TextBoxX _G2_Road;

	[AccessedThroughProperty("Label26")]
	private Label _Label26;

	[AccessedThroughProperty("G1")]
	private GroupBox _G1;

	[AccessedThroughProperty("G2_Work")]
	private TextBoxX _G2_Work;

	[AccessedThroughProperty("Label27")]
	private Label _Label27;

	[AccessedThroughProperty("Rtype_Main")]
	private ComboBox _Rtype_Main;

	[AccessedThroughProperty("Label28")]
	private Label _Label28;

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

	[AccessedThroughProperty("GroupBox3")]
	private GroupBox _GroupBox3;

	[AccessedThroughProperty("ItemPanel1")]
	private ItemPanel _ItemPanel1;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

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
			EventHandler value2 = GroupBox2_Enter;
			if (_GroupBox2 != null)
			{
				_GroupBox2.Enter -= value2;
			}
			_GroupBox2 = value;
			if (_GroupBox2 != null)
			{
				_GroupBox2.Enter += value2;
			}
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

	internal virtual TextBoxX Remail
	{
		[DebuggerNonUserCode]
		get
		{
			return _Remail;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Remail = value;
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
			_Rtype = value;
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

	internal virtual TextBoxX Rname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Rname = value;
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

	internal virtual TextBoxX G1_Fax
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Fax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Fax = value;
		}
	}

	internal virtual TextBoxX G1_Tel
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Tel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Tel = value;
		}
	}

	internal virtual TextBoxX Rname2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rname2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Rname2 = value;
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

	internal virtual TextBoxX G1_ampore
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_ampore;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_ampore = value;
		}
	}

	internal virtual TextBoxX G1_Tambon
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Tambon;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Tambon = value;
		}
	}

	internal virtual TextBoxX G1_Road
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Road;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Road = value;
		}
	}

	internal virtual TextBoxX G1_Soi
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Soi;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Soi = value;
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

	internal virtual TextBoxX G1_Moo
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Moo;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Moo = value;
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

	internal virtual TextBoxX G1_No
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_No;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_No = value;
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

	internal virtual TextBoxX G1_Code
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Code;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Code = value;
		}
	}

	internal virtual TextBoxX G1_Province
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1_Province;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1_Province = value;
		}
	}

	internal virtual Label Label16
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label16 = value;
		}
	}

	internal virtual Label Label15
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label15 = value;
		}
	}

	internal virtual GroupBox G2
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2 = value;
		}
	}

	internal virtual TextBoxX G2_No
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_No;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_No = value;
		}
	}

	internal virtual Label Label17
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label17 = value;
		}
	}

	internal virtual Label Label18
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label18 = value;
		}
	}

	internal virtual TextBoxX G2_Fax
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Fax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Fax = value;
		}
	}

	internal virtual Label Label19
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label19 = value;
		}
	}

	internal virtual TextBoxX G2_Tel
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Tel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Tel = value;
		}
	}

	internal virtual Label Label20
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label20 = value;
		}
	}

	internal virtual Label Label21
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label21 = value;
		}
	}

	internal virtual Label Label22
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label22 = value;
		}
	}

	internal virtual Label Label23
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label23;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label23 = value;
		}
	}

	internal virtual TextBoxX G2_Code
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Code;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Code = value;
		}
	}

	internal virtual TextBoxX G2_Moo
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Moo;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Moo = value;
		}
	}

	internal virtual TextBoxX G2_ampore
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_ampore;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_ampore = value;
		}
	}

	internal virtual Label Label24
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label24;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label24 = value;
		}
	}

	internal virtual TextBoxX G2_Province
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Province;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Province = value;
		}
	}

	internal virtual Label Label25
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label25;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label25 = value;
		}
	}

	internal virtual TextBoxX G2_Tambon
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Tambon;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Tambon = value;
		}
	}

	internal virtual TextBoxX G2_Soi
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Soi;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Soi = value;
		}
	}

	internal virtual TextBoxX G2_Road
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Road;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Road = value;
		}
	}

	internal virtual Label Label26
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label26;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label26 = value;
		}
	}

	internal virtual GroupBox G1
	{
		[DebuggerNonUserCode]
		get
		{
			return _G1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G1 = value;
		}
	}

	internal virtual TextBoxX G2_Work
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_Work;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_Work = value;
		}
	}

	internal virtual Label Label27
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label27;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label27 = value;
		}
	}

	internal virtual ComboBox Rtype_Main
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rtype_Main;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Rtype_Main_SelectedIndexChanged;
			if (_Rtype_Main != null)
			{
				_Rtype_Main.SelectedIndexChanged -= value2;
			}
			_Rtype_Main = value;
			if (_Rtype_Main != null)
			{
				_Rtype_Main.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label28
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label28;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label28 = value;
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

	internal virtual GroupBox GroupBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox3 = value;
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

	[DebuggerNonUserCode]
	static FrmManageCustomers()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmManageCustomers()
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
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.AddMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.EditMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.DelMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label7 = new System.Windows.Forms.Label();
		this.TextBoxX_1 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label29 = new System.Windows.Forms.Label();
		this.Label30 = new System.Windows.Forms.Label();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.G2 = new System.Windows.Forms.GroupBox();
		this.G2_Work = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label27 = new System.Windows.Forms.Label();
		this.G2_No = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label17 = new System.Windows.Forms.Label();
		this.Label18 = new System.Windows.Forms.Label();
		this.G2_Fax = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label19 = new System.Windows.Forms.Label();
		this.G2_Tel = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label20 = new System.Windows.Forms.Label();
		this.Label21 = new System.Windows.Forms.Label();
		this.Label22 = new System.Windows.Forms.Label();
		this.Label23 = new System.Windows.Forms.Label();
		this.G2_Code = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G2_Moo = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G2_ampore = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label24 = new System.Windows.Forms.Label();
		this.G2_Province = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label25 = new System.Windows.Forms.Label();
		this.G2_Tambon = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G2_Soi = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G2_Road = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label26 = new System.Windows.Forms.Label();
		this.G1 = new System.Windows.Forms.GroupBox();
		this.G1_No = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.G1_Fax = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label9 = new System.Windows.Forms.Label();
		this.G1_Tel = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.G1_Code = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G1_Moo = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G1_ampore = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label15 = new System.Windows.Forms.Label();
		this.G1_Province = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label14 = new System.Windows.Forms.Label();
		this.G1_Tambon = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G1_Soi = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G1_Road = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label16 = new System.Windows.Forms.Label();
		this.Rtype_Main = new System.Windows.Forms.ComboBox();
		this.Rtype = new System.Windows.Forms.ComboBox();
		this.Rno = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Rname2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Rname = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Remail = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label28 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.ItemPanel1 = new DevComponents.DotNetBar.ItemPanel();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.GroupBox3 = new System.Windows.Forms.GroupBox();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.G2.SuspendLayout();
		this.G1.SuspendLayout();
		this.GroupBox3.SuspendLayout();
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
		this.GroupBox1.Controls.Add(this.TextBoxX_0);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.TextBoxX_1);
		this.GroupBox1.Controls.Add(this.Label29);
		this.GroupBox1.Controls.Add(this.Label30);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(7, 35);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1096, 395);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX_2;
		location = new System.Drawing.Point(170, 362);
		buttonX.Location = location;
		this.ButtonX_2.Name = "ลบ";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX_2;
		size = new System.Drawing.Size(75, 23);
		buttonX2.Size = size;
		this.ButtonX_2.TabIndex = 3;
		this.ButtonX_2.Text = "ลบ";
		this.ButtonX_3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX_3;
		location = new System.Drawing.Point(90, 362);
		buttonX3.Location = location;
		this.ButtonX_3.Name = "แก\u0e49ไข";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX_3;
		size = new System.Drawing.Size(75, 23);
		buttonX4.Size = size;
		this.ButtonX_3.TabIndex = 2;
		this.ButtonX_3.Text = "แก\u0e49ไข";
		this.ButtonX_4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX_4;
		location = new System.Drawing.Point(11, 362);
		buttonX5.Location = location;
		this.ButtonX_4.Name = "เพ\u0e34\u0e48ม";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX_4;
		size = new System.Drawing.Size(75, 23);
		buttonX6.Size = size;
		this.ButtonX_4.TabIndex = 1;
		this.ButtonX_4.Text = "เพ\u0e34\u0e48ม";
		this.ListViewEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListViewEx1.Atto_กระดาษแนวนอน = true;
		this.ListViewEx1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[29]
		{
			this.ColumnHeader7, this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader9, this.ColumnHeader10, this.ColumnHeader11,
			this.ColumnHeader12, this.ColumnHeader13, this.ColumnHeader14, this.ColumnHeader15, this.ColumnHeader16, this.ColumnHeader17, this.ColumnHeader18, this.ColumnHeader19, this.ColumnHeader20, this.ColumnHeader21,
			this.ColumnHeader22, this.ColumnHeader23, this.ColumnHeader24, this.ColumnHeader25, this.ColumnHeader26, this.ColumnHeader27, this.ColumnHeader28, this.ColumnHeader29, this.ColumnHeader8
		});
		this.ListViewEx1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListViewEx1.FitToPage = true;
		this.ListViewEx1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListViewEx1.FullRowSelect = true;
		this.ListViewEx1.GridLines = true;
		global::PrintableListView.PrintableListView listViewEx = this.ListViewEx1;
		location = new System.Drawing.Point(11, 53);
		listViewEx.Location = location;
		this.ListViewEx1.MultiSelect = false;
		this.ListViewEx1.Name = "ListViewEx1";
		global::PrintableListView.PrintableListView listViewEx2 = this.ListViewEx1;
		size = new System.Drawing.Size(1070, 304);
		listViewEx2.Size = size;
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
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader3.Text = "นามสก\u0e38ล";
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader4.Text = "ราคาท\u0e35\u0e48ใช\u0e49";
		this.ColumnHeader4.Width = 100;
		this.ColumnHeader5.Text = "อ\u0e35เมลล\u0e4c";
		this.ColumnHeader5.Width = 100;
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
		this.TextBoxX_0.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_0;
		location = new System.Drawing.Point(77, 22);
		textBoxX_.Location = location;
		this.TextBoxX_0.MaxLength = 255;
		this.TextBoxX_0.Name = "TextBoxX1";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_0;
		size = new System.Drawing.Size(88, 23);
		textBoxX_2.Size = size;
		this.TextBoxX_0.TabIndex = 0;
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label = this.Label7;
		location = new System.Drawing.Point(28, 22);
		label.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label2 = this.Label7;
		size = new System.Drawing.Size(0, 16);
		label2.Size = size;
		this.Label7.TabIndex = 11;
		this.TextBoxX_1.Border.Class = "TextBoxBorder";
		this.TextBoxX_1.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_3 = this.TextBoxX_1;
		location = new System.Drawing.Point(227, 22);
		textBoxX_3.Location = location;
		this.TextBoxX_1.MaxLength = 255;
		this.TextBoxX_1.Name = "TextBoxX2";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_4 = this.TextBoxX_1;
		size = new System.Drawing.Size(227, 23);
		textBoxX_4.Size = size;
		this.TextBoxX_1.TabIndex = 1;
		this.Label29.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label29;
		location = new System.Drawing.Point(15, 26);
		label3.Location = location;
		this.Label29.Name = "Label29";
		System.Windows.Forms.Label label4 = this.Label29;
		size = new System.Drawing.Size(60, 16);
		label4.Size = size;
		this.Label29.TabIndex = 11;
		this.Label29.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label30.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label30;
		location = new System.Drawing.Point(171, 25);
		label5.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label6 = this.Label30;
		size = new System.Drawing.Size(54, 16);
		label6.Size = size;
		this.Label30.TabIndex = 11;
		this.Label30.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox2.Controls.Add(this.G2);
		this.GroupBox2.Controls.Add(this.GroupBox3);
		this.GroupBox2.Controls.Add(this.G1);
		this.GroupBox2.Controls.Add(this.Rtype_Main);
		this.GroupBox2.Controls.Add(this.Rtype);
		this.GroupBox2.Controls.Add(this.Rno);
		this.GroupBox2.Controls.Add(this.Rname2);
		this.GroupBox2.Controls.Add(this.Rname);
		this.GroupBox2.Controls.Add(this.Remail);
		this.GroupBox2.Controls.Add(this.Label6);
		this.GroupBox2.Controls.Add(this.Label1);
		this.GroupBox2.Controls.Add(this.Label4);
		this.GroupBox2.Controls.Add(this.Label28);
		this.GroupBox2.Controls.Add(this.Label8);
		this.GroupBox2.Controls.Add(this.Label5);
		this.GroupBox2.Controls.Add(this.ButtonX_0);
		this.GroupBox2.Controls.Add(this.ButtonX_1);
		this.GroupBox2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox2;
		location = new System.Drawing.Point(7, 436);
		groupBox3.Location = location;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox2;
		size = new System.Drawing.Size(1096, 319);
		groupBox4.Size = size;
		this.GroupBox2.TabIndex = 1;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไข";
		this.G2.Controls.Add(this.G2_Work);
		this.G2.Controls.Add(this.Label27);
		this.G2.Controls.Add(this.G2_No);
		this.G2.Controls.Add(this.Label17);
		this.G2.Controls.Add(this.Label18);
		this.G2.Controls.Add(this.G2_Fax);
		this.G2.Controls.Add(this.Label19);
		this.G2.Controls.Add(this.G2_Tel);
		this.G2.Controls.Add(this.Label20);
		this.G2.Controls.Add(this.Label21);
		this.G2.Controls.Add(this.Label22);
		this.G2.Controls.Add(this.Label23);
		this.G2.Controls.Add(this.G2_Code);
		this.G2.Controls.Add(this.G2_Moo);
		this.G2.Controls.Add(this.G2_ampore);
		this.G2.Controls.Add(this.Label24);
		this.G2.Controls.Add(this.G2_Province);
		this.G2.Controls.Add(this.Label25);
		this.G2.Controls.Add(this.G2_Tambon);
		this.G2.Controls.Add(this.G2_Soi);
		this.G2.Controls.Add(this.G2_Road);
		this.G2.Controls.Add(this.Label26);
		System.Windows.Forms.GroupBox g = this.G2;
		location = new System.Drawing.Point(705, 13);
		g.Location = location;
		this.G2.Name = "G2";
		System.Windows.Forms.GroupBox g2 = this.G2;
		size = new System.Drawing.Size(370, 238);
		g2.Size = size;
		this.G2.TabIndex = 6;
		this.G2.TabStop = false;
		this.G2.Text = "สถานท\u0e35\u0e48ทำงาน";
		this.G2_Work.Border.Class = "TextBoxBorder";
		this.G2_Work.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Work = this.G2_Work;
		location = new System.Drawing.Point(94, 16);
		g2_Work.Location = location;
		this.G2_Work.MaxLength = 255;
		this.G2_Work.Name = "G2_Work";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Work2 = this.G2_Work;
		size = new System.Drawing.Size(262, 23);
		g2_Work2.Size = size;
		this.G2_Work.TabIndex = 0;
		this.Label27.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label27;
		location = new System.Drawing.Point(35, 20);
		label7.Location = location;
		this.Label27.Name = "Label27";
		System.Windows.Forms.Label label8 = this.Label27;
		size = new System.Drawing.Size(57, 16);
		label8.Size = size;
		this.Label27.TabIndex = 13;
		this.Label27.Text = "ช\u0e37\u0e48อบร\u0e34ษ\u0e31ท";
		this.G2_No.Border.Class = "TextBoxBorder";
		this.G2_No.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_No = this.G2_No;
		location = new System.Drawing.Point(94, 44);
		g2_No.Location = location;
		this.G2_No.MaxLength = 255;
		this.G2_No.Name = "G2_No";
		DevComponents.DotNetBar.Controls.TextBoxX g2_No2 = this.G2_No;
		size = new System.Drawing.Size(49, 23);
		g2_No2.Size = size;
		this.G2_No.TabIndex = 1;
		this.Label17.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label17;
		location = new System.Drawing.Point(55, 47);
		label9.Location = location;
		this.Label17.Name = "Label17";
		System.Windows.Forms.Label label10 = this.Label17;
		size = new System.Drawing.Size(37, 16);
		label10.Size = size;
		this.Label17.TabIndex = 11;
		this.Label17.Text = "เลขท\u0e35\u0e48";
		this.Label18.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label18;
		location = new System.Drawing.Point(146, 47);
		label11.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label12 = this.Label18;
		size = new System.Drawing.Size(24, 16);
		label12.Size = size;
		this.Label18.TabIndex = 11;
		this.Label18.Text = "หม\u0e39\u0e48";
		this.G2_Fax.Border.Class = "TextBoxBorder";
		this.G2_Fax.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Fax = this.G2_Fax;
		location = new System.Drawing.Point(94, 206);
		g2_Fax.Location = location;
		this.G2_Fax.MaxLength = 255;
		this.G2_Fax.Name = "G2_Fax";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Fax2 = this.G2_Fax;
		size = new System.Drawing.Size(262, 23);
		g2_Fax2.Size = size;
		this.G2_Fax.TabIndex = 9;
		this.Label19.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label19;
		location = new System.Drawing.Point(63, 182);
		label13.Location = location;
		this.Label19.Name = "Label19";
		System.Windows.Forms.Label label14 = this.Label19;
		size = new System.Drawing.Size(29, 16);
		label14.Size = size;
		this.Label19.TabIndex = 11;
		this.Label19.Text = "โทร";
		this.G2_Tel.Border.Class = "TextBoxBorder";
		this.G2_Tel.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tel = this.G2_Tel;
		location = new System.Drawing.Point(94, 179);
		g2_Tel.Location = location;
		this.G2_Tel.MaxLength = 255;
		this.G2_Tel.Name = "G2_Tel";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tel2 = this.G2_Tel;
		size = new System.Drawing.Size(262, 23);
		g2_Tel2.Size = size;
		this.G2_Tel.TabIndex = 8;
		this.Label20.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label20;
		location = new System.Drawing.Point(59, 74);
		label15.Location = location;
		this.Label20.Name = "Label20";
		System.Windows.Forms.Label label16 = this.Label20;
		size = new System.Drawing.Size(33, 16);
		label16.Size = size;
		this.Label20.TabIndex = 11;
		this.Label20.Text = "ซอย";
		this.Label21.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label21;
		location = new System.Drawing.Point(211, 74);
		label17.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label18 = this.Label21;
		size = new System.Drawing.Size(32, 16);
		label18.Size = size;
		this.Label21.TabIndex = 11;
		this.Label21.Text = "ถนน";
		this.Label22.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label22;
		location = new System.Drawing.Point(63, 209);
		label19.Location = location;
		this.Label22.Name = "Label22";
		System.Windows.Forms.Label label20 = this.Label22;
		size = new System.Drawing.Size(28, 16);
		label20.Size = size;
		this.Label22.TabIndex = 11;
		this.Label22.Text = "Fax";
		this.Label23.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label23;
		location = new System.Drawing.Point(54, 101);
		label21.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label22 = this.Label23;
		size = new System.Drawing.Size(38, 16);
		label22.Size = size;
		this.Label23.TabIndex = 11;
		this.Label23.Text = "ตำบล";
		this.G2_Code.Border.Class = "TextBoxBorder";
		this.G2_Code.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Code = this.G2_Code;
		location = new System.Drawing.Point(94, 152);
		g2_Code.Location = location;
		this.G2_Code.MaxLength = 255;
		this.G2_Code.Name = "G2_Code";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Code2 = this.G2_Code;
		size = new System.Drawing.Size(105, 23);
		g2_Code2.Size = size;
		this.G2_Code.TabIndex = 7;
		this.G2_Moo.Border.Class = "TextBoxBorder";
		this.G2_Moo.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Moo = this.G2_Moo;
		location = new System.Drawing.Point(172, 44);
		g2_Moo.Location = location;
		this.G2_Moo.MaxLength = 255;
		this.G2_Moo.Name = "G2_Moo";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Moo2 = this.G2_Moo;
		size = new System.Drawing.Size(49, 23);
		g2_Moo2.Size = size;
		this.G2_Moo.TabIndex = 2;
		this.G2_ampore.Border.Class = "TextBoxBorder";
		this.G2_ampore.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_ampore = this.G2_ampore;
		location = new System.Drawing.Point(246, 98);
		g2_ampore.Location = location;
		this.G2_ampore.MaxLength = 255;
		this.G2_ampore.Name = "G2_ampore";
		DevComponents.DotNetBar.Controls.TextBoxX g2_ampore2 = this.G2_ampore;
		size = new System.Drawing.Size(110, 23);
		g2_ampore2.Size = size;
		this.G2_ampore.TabIndex = 5;
		this.Label24.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label24;
		location = new System.Drawing.Point(49, 128);
		label23.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label24 = this.Label24;
		size = new System.Drawing.Size(43, 16);
		label24.Size = size;
		this.Label24.TabIndex = 11;
		this.Label24.Text = "จ\u0e31งหว\u0e31ด";
		this.G2_Province.Border.Class = "TextBoxBorder";
		this.G2_Province.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Province = this.G2_Province;
		location = new System.Drawing.Point(94, 125);
		g2_Province.Location = location;
		this.G2_Province.MaxLength = 255;
		this.G2_Province.Name = "G2_Province";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Province2 = this.G2_Province;
		size = new System.Drawing.Size(262, 23);
		g2_Province2.Size = size;
		this.G2_Province.TabIndex = 6;
		this.Label25.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label25;
		location = new System.Drawing.Point(202, 101);
		label25.Location = location;
		this.Label25.Name = "Label25";
		System.Windows.Forms.Label label26 = this.Label25;
		size = new System.Drawing.Size(42, 16);
		label26.Size = size;
		this.Label25.TabIndex = 11;
		this.Label25.Text = "อำเภอ";
		this.G2_Tambon.Border.Class = "TextBoxBorder";
		this.G2_Tambon.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tambon = this.G2_Tambon;
		location = new System.Drawing.Point(94, 98);
		g2_Tambon.Location = location;
		this.G2_Tambon.MaxLength = 255;
		this.G2_Tambon.Name = "G2_Tambon";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tambon2 = this.G2_Tambon;
		size = new System.Drawing.Size(105, 23);
		g2_Tambon2.Size = size;
		this.G2_Tambon.TabIndex = 4;
		this.G2_Soi.Border.Class = "TextBoxBorder";
		this.G2_Soi.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Soi = this.G2_Soi;
		location = new System.Drawing.Point(94, 71);
		g2_Soi.Location = location;
		this.G2_Soi.MaxLength = 255;
		this.G2_Soi.Name = "G2_Soi";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Soi2 = this.G2_Soi;
		size = new System.Drawing.Size(105, 23);
		g2_Soi2.Size = size;
		this.G2_Soi.TabIndex = 2;
		this.G2_Road.Border.Class = "TextBoxBorder";
		this.G2_Road.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g2_Road = this.G2_Road;
		location = new System.Drawing.Point(246, 71);
		g2_Road.Location = location;
		this.G2_Road.MaxLength = 255;
		this.G2_Road.Name = "G2_Road";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Road2 = this.G2_Road;
		size = new System.Drawing.Size(110, 23);
		g2_Road2.Size = size;
		this.G2_Road.TabIndex = 3;
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label26;
		location = new System.Drawing.Point(12, 155);
		label27.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label28 = this.Label26;
		size = new System.Drawing.Size(80, 16);
		label28.Size = size;
		this.Label26.TabIndex = 11;
		this.Label26.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.G1.Controls.Add(this.G1_No);
		this.G1.Controls.Add(this.Label2);
		this.G1.Controls.Add(this.Label3);
		this.G1.Controls.Add(this.G1_Fax);
		this.G1.Controls.Add(this.Label9);
		this.G1.Controls.Add(this.G1_Tel);
		this.G1.Controls.Add(this.Label11);
		this.G1.Controls.Add(this.Label12);
		this.G1.Controls.Add(this.Label10);
		this.G1.Controls.Add(this.Label13);
		this.G1.Controls.Add(this.G1_Code);
		this.G1.Controls.Add(this.G1_Moo);
		this.G1.Controls.Add(this.G1_ampore);
		this.G1.Controls.Add(this.Label15);
		this.G1.Controls.Add(this.G1_Province);
		this.G1.Controls.Add(this.Label14);
		this.G1.Controls.Add(this.G1_Tambon);
		this.G1.Controls.Add(this.G1_Soi);
		this.G1.Controls.Add(this.G1_Road);
		this.G1.Controls.Add(this.Label16);
		System.Windows.Forms.GroupBox g3 = this.G1;
		location = new System.Drawing.Point(329, 13);
		g3.Location = location;
		this.G1.Name = "G1";
		System.Windows.Forms.GroupBox g4 = this.G1;
		size = new System.Drawing.Size(370, 238);
		g4.Size = size;
		this.G1.TabIndex = 5;
		this.G1.TabStop = false;
		this.G1.Text = "ท\u0e35\u0e48อย\u0e39\u0e48ป\u0e31จจ\u0e38บ\u0e31น";
		this.G1_No.Border.Class = "TextBoxBorder";
		this.G1_No.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_No = this.G1_No;
		location = new System.Drawing.Point(94, 42);
		g1_No.Location = location;
		this.G1_No.MaxLength = 255;
		this.G1_No.Name = "G1_No";
		DevComponents.DotNetBar.Controls.TextBoxX g1_No2 = this.G1_No;
		size = new System.Drawing.Size(49, 23);
		g1_No2.Size = size;
		this.G1_No.TabIndex = 0;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label2;
		location = new System.Drawing.Point(55, 45);
		label29.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label30 = this.Label2;
		size = new System.Drawing.Size(37, 16);
		label30.Size = size;
		this.Label2.TabIndex = 11;
		this.Label2.Text = "เลขท\u0e35\u0e48";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label3;
		location = new System.Drawing.Point(146, 45);
		label31.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label32 = this.Label3;
		size = new System.Drawing.Size(24, 16);
		label32.Size = size;
		this.Label3.TabIndex = 11;
		this.Label3.Text = "หม\u0e39\u0e48";
		this.G1_Fax.Border.Class = "TextBoxBorder";
		this.G1_Fax.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Fax = this.G1_Fax;
		location = new System.Drawing.Point(94, 204);
		g1_Fax.Location = location;
		this.G1_Fax.MaxLength = 255;
		this.G1_Fax.Name = "G1_Fax";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Fax2 = this.G1_Fax;
		size = new System.Drawing.Size(262, 23);
		g1_Fax2.Size = size;
		this.G1_Fax.TabIndex = 9;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label9;
		location = new System.Drawing.Point(63, 180);
		label33.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label34 = this.Label9;
		size = new System.Drawing.Size(29, 16);
		label34.Size = size;
		this.Label9.TabIndex = 11;
		this.Label9.Text = "โทร";
		this.G1_Tel.Border.Class = "TextBoxBorder";
		this.G1_Tel.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tel = this.G1_Tel;
		location = new System.Drawing.Point(94, 177);
		g1_Tel.Location = location;
		this.G1_Tel.MaxLength = 255;
		this.G1_Tel.Name = "G1_Tel";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tel2 = this.G1_Tel;
		size = new System.Drawing.Size(262, 23);
		g1_Tel2.Size = size;
		this.G1_Tel.TabIndex = 8;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label11;
		location = new System.Drawing.Point(59, 72);
		label35.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label36 = this.Label11;
		size = new System.Drawing.Size(33, 16);
		label36.Size = size;
		this.Label11.TabIndex = 11;
		this.Label11.Text = "ซอย";
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label12;
		location = new System.Drawing.Point(211, 72);
		label37.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label38 = this.Label12;
		size = new System.Drawing.Size(32, 16);
		label38.Size = size;
		this.Label12.TabIndex = 11;
		this.Label12.Text = "ถนน";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label10;
		location = new System.Drawing.Point(63, 207);
		label39.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label40 = this.Label10;
		size = new System.Drawing.Size(28, 16);
		label40.Size = size;
		this.Label10.TabIndex = 11;
		this.Label10.Text = "Fax";
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label13;
		location = new System.Drawing.Point(54, 99);
		label41.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label42 = this.Label13;
		size = new System.Drawing.Size(38, 16);
		label42.Size = size;
		this.Label13.TabIndex = 11;
		this.Label13.Text = "ตำบล";
		this.G1_Code.Border.Class = "TextBoxBorder";
		this.G1_Code.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Code = this.G1_Code;
		location = new System.Drawing.Point(94, 150);
		g1_Code.Location = location;
		this.G1_Code.MaxLength = 255;
		this.G1_Code.Name = "G1_Code";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Code2 = this.G1_Code;
		size = new System.Drawing.Size(105, 23);
		g1_Code2.Size = size;
		this.G1_Code.TabIndex = 7;
		this.G1_Moo.Border.Class = "TextBoxBorder";
		this.G1_Moo.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Moo = this.G1_Moo;
		location = new System.Drawing.Point(172, 42);
		g1_Moo.Location = location;
		this.G1_Moo.MaxLength = 255;
		this.G1_Moo.Name = "G1_Moo";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Moo2 = this.G1_Moo;
		size = new System.Drawing.Size(49, 23);
		g1_Moo2.Size = size;
		this.G1_Moo.TabIndex = 1;
		this.G1_ampore.Border.Class = "TextBoxBorder";
		this.G1_ampore.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_ampore = this.G1_ampore;
		location = new System.Drawing.Point(246, 96);
		g1_ampore.Location = location;
		this.G1_ampore.MaxLength = 255;
		this.G1_ampore.Name = "G1_ampore";
		DevComponents.DotNetBar.Controls.TextBoxX g1_ampore2 = this.G1_ampore;
		size = new System.Drawing.Size(110, 23);
		g1_ampore2.Size = size;
		this.G1_ampore.TabIndex = 5;
		this.Label15.AutoSize = true;
		System.Windows.Forms.Label label43 = this.Label15;
		location = new System.Drawing.Point(49, 126);
		label43.Location = location;
		this.Label15.Name = "Label15";
		System.Windows.Forms.Label label44 = this.Label15;
		size = new System.Drawing.Size(43, 16);
		label44.Size = size;
		this.Label15.TabIndex = 11;
		this.Label15.Text = "จ\u0e31งหว\u0e31ด";
		this.G1_Province.Border.Class = "TextBoxBorder";
		this.G1_Province.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Province = this.G1_Province;
		location = new System.Drawing.Point(94, 123);
		g1_Province.Location = location;
		this.G1_Province.MaxLength = 255;
		this.G1_Province.Name = "G1_Province";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Province2 = this.G1_Province;
		size = new System.Drawing.Size(262, 23);
		g1_Province2.Size = size;
		this.G1_Province.TabIndex = 6;
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label45 = this.Label14;
		location = new System.Drawing.Point(202, 99);
		label45.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label46 = this.Label14;
		size = new System.Drawing.Size(42, 16);
		label46.Size = size;
		this.Label14.TabIndex = 11;
		this.Label14.Text = "อำเภอ";
		this.G1_Tambon.Border.Class = "TextBoxBorder";
		this.G1_Tambon.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tambon = this.G1_Tambon;
		location = new System.Drawing.Point(94, 96);
		g1_Tambon.Location = location;
		this.G1_Tambon.MaxLength = 255;
		this.G1_Tambon.Name = "G1_Tambon";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tambon2 = this.G1_Tambon;
		size = new System.Drawing.Size(105, 23);
		g1_Tambon2.Size = size;
		this.G1_Tambon.TabIndex = 4;
		this.G1_Soi.Border.Class = "TextBoxBorder";
		this.G1_Soi.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Soi = this.G1_Soi;
		location = new System.Drawing.Point(94, 69);
		g1_Soi.Location = location;
		this.G1_Soi.MaxLength = 255;
		this.G1_Soi.Name = "G1_Soi";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Soi2 = this.G1_Soi;
		size = new System.Drawing.Size(105, 23);
		g1_Soi2.Size = size;
		this.G1_Soi.TabIndex = 2;
		this.G1_Road.Border.Class = "TextBoxBorder";
		this.G1_Road.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX g1_Road = this.G1_Road;
		location = new System.Drawing.Point(246, 69);
		g1_Road.Location = location;
		this.G1_Road.MaxLength = 255;
		this.G1_Road.Name = "G1_Road";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Road2 = this.G1_Road;
		size = new System.Drawing.Size(110, 23);
		g1_Road2.Size = size;
		this.G1_Road.TabIndex = 3;
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label47 = this.Label16;
		location = new System.Drawing.Point(12, 153);
		label47.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label48 = this.Label16;
		size = new System.Drawing.Size(80, 16);
		label48.Size = size;
		this.Label16.TabIndex = 11;
		this.Label16.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.Rtype_Main.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Rtype_Main.Enabled = false;
		this.Rtype_Main.FormattingEnabled = true;
		System.Windows.Forms.ComboBox rtype_Main = this.Rtype_Main;
		location = new System.Drawing.Point(90, 138);
		rtype_Main.Location = location;
		this.Rtype_Main.Name = "Rtype_Main";
		System.Windows.Forms.ComboBox rtype_Main2 = this.Rtype_Main;
		size = new System.Drawing.Size(227, 24);
		rtype_Main2.Size = size;
		this.Rtype_Main.TabIndex = 3;
		this.Rtype.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Rtype.Enabled = false;
		this.Rtype.FormattingEnabled = true;
		System.Windows.Forms.ComboBox rtype = this.Rtype;
		location = new System.Drawing.Point(90, 167);
		rtype.Location = location;
		this.Rtype.Name = "Rtype";
		System.Windows.Forms.ComboBox rtype2 = this.Rtype;
		size = new System.Drawing.Size(227, 24);
		rtype2.Size = size;
		this.Rtype.TabIndex = 3;
		this.Rno.BackColor = System.Drawing.Color.White;
		this.Rno.Border.Class = "TextBoxBorder";
		this.Rno.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rno = this.Rno;
		location = new System.Drawing.Point(90, 28);
		rno.Location = location;
		this.Rno.MaxLength = 255;
		this.Rno.Name = "Rno";
		DevComponents.DotNetBar.Controls.TextBoxX rno2 = this.Rno;
		size = new System.Drawing.Size(88, 23);
		rno2.Size = size;
		this.Rno.TabIndex = 0;
		this.Rno.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Rname2.Border.Class = "TextBoxBorder";
		this.Rname2.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rname = this.Rname2;
		location = new System.Drawing.Point(90, 82);
		rname.Location = location;
		this.Rname2.MaxLength = 255;
		this.Rname2.Name = "Rname2";
		DevComponents.DotNetBar.Controls.TextBoxX rname2 = this.Rname2;
		size = new System.Drawing.Size(227, 23);
		rname2.Size = size;
		this.Rname2.TabIndex = 2;
		this.Rname.Border.Class = "TextBoxBorder";
		this.Rname.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rname3 = this.Rname;
		location = new System.Drawing.Point(90, 55);
		rname3.Location = location;
		this.Rname.MaxLength = 255;
		this.Rname.Name = "Rname";
		DevComponents.DotNetBar.Controls.TextBoxX rname4 = this.Rname;
		size = new System.Drawing.Size(227, 23);
		rname4.Size = size;
		this.Rname.TabIndex = 1;
		this.Remail.Border.Class = "TextBoxBorder";
		this.Remail.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX remail = this.Remail;
		location = new System.Drawing.Point(90, 110);
		remail.Location = location;
		this.Remail.MaxLength = 255;
		this.Remail.Name = "Remail";
		DevComponents.DotNetBar.Controls.TextBoxX remail2 = this.Remail;
		size = new System.Drawing.Size(227, 23);
		remail2.Size = size;
		this.Remail.TabIndex = 4;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label49 = this.Label6;
		location = new System.Drawing.Point(34, 85);
		label49.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label50 = this.Label6;
		size = new System.Drawing.Size(54, 16);
		label50.Size = size;
		this.Label6.TabIndex = 11;
		this.Label6.Text = "นามสก\u0e38ล";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label51 = this.Label1;
		location = new System.Drawing.Point(34, 58);
		label51.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label52 = this.Label1;
		size = new System.Drawing.Size(54, 16);
		label52.Size = size;
		this.Label1.TabIndex = 11;
		this.Label1.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label53 = this.Label4;
		location = new System.Drawing.Point(45, 113);
		label53.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label54 = this.Label4;
		size = new System.Drawing.Size(43, 16);
		label54.Size = size;
		this.Label4.TabIndex = 11;
		this.Label4.Text = "E-Mail";
		this.Label28.AutoSize = true;
		System.Windows.Forms.Label label55 = this.Label28;
		location = new System.Drawing.Point(8, 143);
		label55.Location = location;
		this.Label28.Name = "Label28";
		System.Windows.Forms.Label label56 = this.Label28;
		size = new System.Drawing.Size(79, 16);
		label56.Size = size;
		this.Label28.TabIndex = 11;
		this.Label28.Text = "ประเภทล\u0e39กค\u0e49า";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label57 = this.Label8;
		location = new System.Drawing.Point(28, 32);
		label57.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label58 = this.Label8;
		size = new System.Drawing.Size(60, 16);
		label58.Size = size;
		this.Label8.TabIndex = 11;
		this.Label8.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label59 = this.Label5;
		location = new System.Drawing.Point(31, 172);
		label59.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label60 = this.Label5;
		size = new System.Drawing.Size(57, 16);
		label60.Size = size;
		this.Label5.TabIndex = 11;
		this.Label5.Text = "ราคาท\u0e35\u0e48ใช\u0e49";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.Enabled = false;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX_0;
		location = new System.Drawing.Point(171, 283);
		buttonX7.Location = location;
		this.ButtonX_0.Name = "ยกเล\u0e34ก";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX_0;
		size = new System.Drawing.Size(75, 23);
		buttonX8.Size = size;
		this.ButtonX_0.TabIndex = 8;
		this.ButtonX_0.Text = "ยกเล\u0e34ก";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_1.Enabled = false;
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX_1;
		location = new System.Drawing.Point(90, 283);
		buttonX9.Location = location;
		this.ButtonX_1.Name = "บ\u0e31นท\u0e36ก";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX_1;
		size = new System.Drawing.Size(75, 23);
		buttonX10.Size = size;
		this.ButtonX_1.TabIndex = 7;
		this.ButtonX_1.Text = "บ\u0e31นท\u0e36ก";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		size = new System.Drawing.Size(1112, 32);
		panelEx2.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Red;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Maroon;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "จ\u0e31ดการล\u0e39กค\u0e49า";
		this.Timer2.Enabled = true;
		this.OpenFileDialog1.RestoreDirectory = true;
		this.ItemPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ItemPanel1.BackgroundStyle.Class = "ItemPanel";
		this.ItemPanel1.ContainerControlProcessDialogKey = true;
		this.ItemPanel1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem1 });
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel1;
		location = new System.Drawing.Point(6, 15);
		itemPanel.Location = location;
		this.ItemPanel1.Name = "ItemPanel1";
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel1;
		size = new System.Drawing.Size(734, 42);
		itemPanel2.Size = size;
		this.ItemPanel1.TabIndex = 12;
		this.ItemPanel1.Text = "ItemPanel1";
		this.ButtonItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem1.Image = iHOTEL2025.My.Resources.Resources.star;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.Text = "11/12/55\r\n46464646";
		this.GroupBox3.Controls.Add(this.ItemPanel1);
		System.Windows.Forms.GroupBox groupBox5 = this.GroupBox3;
		location = new System.Drawing.Point(329, 250);
		groupBox5.Location = location;
		this.GroupBox3.Name = "GroupBox3";
		System.Windows.Forms.GroupBox groupBox6 = this.GroupBox3;
		size = new System.Drawing.Size(746, 62);
		groupBox6.Size = size;
		this.GroupBox3.TabIndex = 5;
		this.GroupBox3.TabStop = false;
		this.GroupBox3.Text = "รายการสแกนเอกสาร";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1112, 762);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox1);
		this.Controls.Add(this.GroupBox2);
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmManageCustomers";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "จ\u0e31ดการล\u0e39กค\u0e49า";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.G2.ResumeLayout(false);
		this.G2.PerformLayout();
		this.G1.ResumeLayout(false);
		this.G1.PerformLayout();
		this.GroupBox3.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void FrmManageRoom_Load(object sender, EventArgs e)
	{
		Listtype();
		ListtypeM();
		Search();
	}

	public void Search()
	{
		cancel();
		object left = "select * from HT_Customers where Cust_no<>''";
		if (Operators.CompareString(TextBoxX_0.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" and Cust_no like '%" + TextBoxX_0.Text, "%' "));
		}
		if (Operators.CompareString(TextBoxX_1.Text, "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" and (Cust_name like '%" + TextBoxX_1.Text, "%' or Cust_name2 like '%"), TextBoxX_1.Text), "%')"));
		}
		DataSet dataSet = Module1.connect("select * from HT_Customers order by Cust_no");
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
					listViewEx = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void Listtype()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
		Rtype.DataSource = dataSet.Tables[0];
		Rtype.DisplayMember = "name";
	}

	public void ListtypeM()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType_Main order by name");
		Rtype_Main.DataSource = dataSet.Tables[0];
		Rtype_Main.DisplayMember = "name";
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
			Rname.Text = ListViewEx1.SelectedItems[0].SubItems[3].Text;
			Rname2.Text = ListViewEx1.SelectedItems[0].SubItems[4].Text;
			Rtype_Main.Text = ListViewEx1.SelectedItems[0].SubItems[28].Text;
			Rtype.Text = ListViewEx1.SelectedItems[0].SubItems[5].Text;
			Remail.Text = ListViewEx1.SelectedItems[0].SubItems[6].Text;
			G1_No.Text = ListViewEx1.SelectedItems[0].SubItems[7].Text;
			G1_Moo.Text = ListViewEx1.SelectedItems[0].SubItems[8].Text;
			G1_Soi.Text = ListViewEx1.SelectedItems[0].SubItems[9].Text;
			G1_Road.Text = ListViewEx1.SelectedItems[0].SubItems[10].Text;
			G1_Tambon.Text = ListViewEx1.SelectedItems[0].SubItems[11].Text;
			G1_ampore.Text = ListViewEx1.SelectedItems[0].SubItems[12].Text;
			G1_Province.Text = ListViewEx1.SelectedItems[0].SubItems[13].Text;
			G1_Code.Text = ListViewEx1.SelectedItems[0].SubItems[14].Text;
			G1_Tel.Text = ListViewEx1.SelectedItems[0].SubItems[15].Text;
			G1_Fax.Text = ListViewEx1.SelectedItems[0].SubItems[16].Text;
			G2_Work.Text = ListViewEx1.SelectedItems[0].SubItems[17].Text;
			G2_No.Text = ListViewEx1.SelectedItems[0].SubItems[18].Text;
			G2_Moo.Text = ListViewEx1.SelectedItems[0].SubItems[19].Text;
			G2_Soi.Text = ListViewEx1.SelectedItems[0].SubItems[20].Text;
			G2_Road.Text = ListViewEx1.SelectedItems[0].SubItems[21].Text;
			G2_Tambon.Text = ListViewEx1.SelectedItems[0].SubItems[22].Text;
			G2_ampore.Text = ListViewEx1.SelectedItems[0].SubItems[23].Text;
			G2_Province.Text = ListViewEx1.SelectedItems[0].SubItems[24].Text;
			G2_Code.Text = ListViewEx1.SelectedItems[0].SubItems[25].Text;
			G2_Tel.Text = ListViewEx1.SelectedItems[0].SubItems[26].Text;
			G2_Fax.Text = ListViewEx1.SelectedItems[0].SubItems[27].Text;
			RefreshScan(ListViewEx1.SelectedItems[0].SubItems[2].Text);
		}
	}

	public void ShowScan(object sender, EventArgs e)
	{
		GForm0 gForm = new GForm0();
		gForm.showID = Conversions.ToInteger(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		gForm.ShowDialog();
		if (ListViewEx1.SelectedItems.Count != 0)
		{
			RefreshScan(ListViewEx1.SelectedItems[0].SubItems[2].Text);
		}
	}

	public void RefreshScan(string c_no)
	{
		ItemPanel1.BeginUpdate();
		ItemPanel1.Items.Clear();
		DataSet dataSet = Module1.connect("SELECT id, cin_no, ttype, cust_no, tmp_no,pic_date FROM Tb_Save_Image where cust_no='" + c_no + "'  order by id desc");
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
				ButtonItem buttonItem = new ButtonItem();
				buttonItem.ButtonStyle = eButtonStyle.ImageAndText;
				buttonItem.Image = Resources.star;
				buttonItem.ImagePosition = eImagePosition.Left;
				buttonItem.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["id"]);
				buttonItem.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["ttype"], "\r\n"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["pic_date"]), "dd/MM/yy")), '\r'), '\n'));
				buttonItem.Click += ShowScan;
				ItemPanel1.Items.Add(buttonItem);
				num2++;
			}
			ItemPanel1.EndUpdate();
		}
	}

	private void ButtonX_4_Click(object sender, EventArgs e)
	{
		cancel();
		RunID();
		Rno.Enabled = true;
		Rname.Enabled = true;
		Rname2.Enabled = true;
		Rtype.Enabled = true;
		Rtype_Main.Enabled = true;
		Remail.Enabled = true;
		G1_No.Enabled = true;
		G1_Moo.Enabled = true;
		G1_Soi.Enabled = true;
		G1_Road.Enabled = true;
		G1_Tambon.Enabled = true;
		G1_ampore.Enabled = true;
		G1_Province.Enabled = true;
		G1_Code.Enabled = true;
		G1_Tel.Enabled = true;
		G1_Fax.Enabled = true;
		G2_Work.Enabled = true;
		G2_No.Enabled = true;
		G2_Moo.Enabled = true;
		G2_Soi.Enabled = true;
		G2_Road.Enabled = true;
		G2_Tambon.Enabled = true;
		G2_ampore.Enabled = true;
		G2_Province.Enabled = true;
		G2_Code.Enabled = true;
		G2_Tel.Enabled = true;
		G2_Fax.Enabled = true;
		ButtonX_1.Enabled = true;
		ButtonX_0.Enabled = true;
		ButtonX_4.Enabled = false;
		ButtonX_3.Enabled = false;
		ButtonX_2.Enabled = false;
		Rno.Focus();
	}

	public void RunID()
	{
		DataSet dataSet = Module1.connect("select top 1 * from HT_Customers order by id desc");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			Rno.Text = "C" + Strings.Format(1, "0000");
		}
		else
		{
			Rno.Text = "C" + Strings.Format(Operators.AddObject(dataSet.Tables[0].Rows[0]["id"], 1), "0000");
		}
	}

	private void ButtonX_3_Click(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการล\u0e39กค\u0e49า");
			return;
		}
		cancel();
		Rno.Text = ListViewEx1.SelectedItems[0].SubItems[2].Text;
		Rname.Text = ListViewEx1.SelectedItems[0].SubItems[3].Text;
		Rname2.Text = ListViewEx1.SelectedItems[0].SubItems[4].Text;
		Rtype_Main.Text = ListViewEx1.SelectedItems[0].SubItems[28].Text;
		Rtype.Text = ListViewEx1.SelectedItems[0].SubItems[5].Text;
		Remail.Text = ListViewEx1.SelectedItems[0].SubItems[6].Text;
		G1_No.Text = ListViewEx1.SelectedItems[0].SubItems[7].Text;
		G1_Moo.Text = ListViewEx1.SelectedItems[0].SubItems[8].Text;
		G1_Soi.Text = ListViewEx1.SelectedItems[0].SubItems[9].Text;
		G1_Road.Text = ListViewEx1.SelectedItems[0].SubItems[10].Text;
		G1_Tambon.Text = ListViewEx1.SelectedItems[0].SubItems[11].Text;
		G1_ampore.Text = ListViewEx1.SelectedItems[0].SubItems[12].Text;
		G1_Province.Text = ListViewEx1.SelectedItems[0].SubItems[13].Text;
		G1_Code.Text = ListViewEx1.SelectedItems[0].SubItems[14].Text;
		G1_Tel.Text = ListViewEx1.SelectedItems[0].SubItems[15].Text;
		G1_Fax.Text = ListViewEx1.SelectedItems[0].SubItems[16].Text;
		G2_Work.Text = ListViewEx1.SelectedItems[0].SubItems[17].Text;
		G2_No.Text = ListViewEx1.SelectedItems[0].SubItems[18].Text;
		G2_Moo.Text = ListViewEx1.SelectedItems[0].SubItems[19].Text;
		G2_Soi.Text = ListViewEx1.SelectedItems[0].SubItems[20].Text;
		G2_Road.Text = ListViewEx1.SelectedItems[0].SubItems[21].Text;
		G2_Tambon.Text = ListViewEx1.SelectedItems[0].SubItems[22].Text;
		G2_ampore.Text = ListViewEx1.SelectedItems[0].SubItems[23].Text;
		G2_Province.Text = ListViewEx1.SelectedItems[0].SubItems[24].Text;
		G2_Code.Text = ListViewEx1.SelectedItems[0].SubItems[25].Text;
		G2_Tel.Text = ListViewEx1.SelectedItems[0].SubItems[26].Text;
		G2_Fax.Text = ListViewEx1.SelectedItems[0].SubItems[27].Text;
		Rno.Enabled = true;
		Rname.Enabled = true;
		Rname2.Enabled = true;
		Rtype.Enabled = true;
		Rtype_Main.Enabled = true;
		Remail.Enabled = true;
		G1_No.Enabled = true;
		G1_Moo.Enabled = true;
		G1_Soi.Enabled = true;
		G1_Road.Enabled = true;
		G1_Tambon.Enabled = true;
		G1_ampore.Enabled = true;
		G1_Province.Enabled = true;
		G1_Code.Enabled = true;
		G1_Tel.Enabled = true;
		G1_Fax.Enabled = true;
		G2_Work.Enabled = true;
		G2_No.Enabled = true;
		G2_Moo.Enabled = true;
		G2_Soi.Enabled = true;
		G2_Road.Enabled = true;
		G2_Tambon.Enabled = true;
		G2_ampore.Enabled = true;
		G2_Province.Enabled = true;
		G2_Code.Enabled = true;
		G2_Tel.Enabled = true;
		G2_Fax.Enabled = true;
		ButtonX_1.Enabled = true;
		ButtonX_0.Enabled = true;
		ButtonX_4.Enabled = false;
		ButtonX_3.Enabled = false;
		ButtonX_2.Enabled = false;
		EditID = ListViewEx1.SelectedItems[0].SubItems[0].Text;
		Rno.Focus();
	}

	private void ButtonX_0_Click(object sender, EventArgs e)
	{
		cancel();
	}

	public void cancel()
	{
		ItemPanel1.BeginUpdate();
		ItemPanel1.Items.Clear();
		ItemPanel1.EndUpdate();
		Rno.Text = "";
		Rname.Text = "";
		Rname2.Text = "";
		Rtype.SelectedIndex = 0;
		Rtype_Main.SelectedIndex = 0;
		Remail.Text = "";
		G1_No.Text = "";
		G1_Moo.Text = "";
		G1_Soi.Text = "";
		G1_Road.Text = "";
		G1_Tambon.Text = "";
		G1_ampore.Text = "";
		G1_Province.Text = "";
		G1_Code.Text = "";
		G1_Tel.Text = "";
		G1_Fax.Text = "";
		G2_Work.Text = "";
		G2_No.Text = "";
		G2_Moo.Text = "";
		G2_Soi.Text = "";
		G2_Road.Text = "";
		G2_Tambon.Text = "";
		G2_ampore.Text = "";
		G2_Province.Text = "";
		G2_Code.Text = "";
		G2_Tel.Text = "";
		G2_Fax.Text = "";
		EditID = "";
		Rno.Enabled = false;
		Rname.Enabled = false;
		Rname2.Enabled = false;
		Rtype.Enabled = false;
		Rtype_Main.Enabled = false;
		Remail.Enabled = false;
		G1_No.Enabled = false;
		G1_Moo.Enabled = false;
		G1_Soi.Enabled = false;
		G1_Road.Enabled = false;
		G1_Tambon.Enabled = false;
		G1_ampore.Enabled = false;
		G1_Province.Enabled = false;
		G1_Code.Enabled = false;
		G1_Tel.Enabled = false;
		G1_Fax.Enabled = false;
		G2_Work.Enabled = false;
		G2_No.Enabled = false;
		G2_Moo.Enabled = false;
		G2_Soi.Enabled = false;
		G2_Road.Enabled = false;
		G2_Tambon.Enabled = false;
		G2_ampore.Enabled = false;
		G2_Province.Enabled = false;
		G2_Code.Enabled = false;
		G2_Tel.Enabled = false;
		G2_Fax.Enabled = false;
		ButtonX_1.Enabled = false;
		ButtonX_0.Enabled = false;
		ButtonX_4.Enabled = true;
		ButtonX_3.Enabled = true;
		ButtonX_2.Enabled = true;
	}

	private void ButtonX_1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Rno.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขล\u0e39กค\u0e49า");
		}
		else if (Operators.CompareString(Rtype.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกประเภทล\u0e39กค\u0e49า");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
			{
				RunID();
				object right = Module1.get_id("HT_Customers", "id");
				object left = "INSERT INTO [HT_Customers]";
				left = Operators.ConcatenateObject(left, "([id]");
				left = Operators.ConcatenateObject(left, ",[Cust_no]");
				left = Operators.ConcatenateObject(left, ",[Cust_name]");
				left = Operators.ConcatenateObject(left, ",[Cust_name2]");
				left = Operators.ConcatenateObject(left, ",[Cust_Type]");
				left = Operators.ConcatenateObject(left, ",[Cust_Email]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_no]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_moo]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_soi]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_road]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_tambon]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_ampore]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_province]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_code]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_tel]");
				left = Operators.ConcatenateObject(left, ",[Cust_Add_fax]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_Name]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_no]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_moo]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_soi]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_road]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_tambon]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_ampore]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_province]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_code]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_tel]");
				left = Operators.ConcatenateObject(left, ",[Cust_Work_fax],[Cust_Last_Change],[Cust_Type_Main]");
				left = Operators.ConcatenateObject(left, ")");
				left = Operators.ConcatenateObject(left, " VALUES ");
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, right);
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rno.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rname.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rname2.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rtype.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Remail.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_No.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Moo.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Soi.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Road.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Tambon.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_ampore.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Province.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Code.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Tel.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G1_Fax.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Work.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_No.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Moo.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Soi.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Road.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Tambon.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_ampore.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Province.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Code.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Tel.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_Fax.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now.Date), "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rtype_Main.Text, "'"));
				left = Operators.ConcatenateObject(left, ")");
				Module1.connect(Conversions.ToString(left));
				Search();
			}
			else
			{
				object left2 = "UPDATE [HT_Customers] SET ";
				left2 = Operators.ConcatenateObject(left2, string.Concat(" [Cust_name]='" + Rname.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_name2]='" + Rname2.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Type]='" + Rtype.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Email]='" + Remail.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_no]='" + G1_No.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_moo]='" + G1_Moo.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_soi]='" + G1_Soi.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_road]='" + G1_Road.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_tambon]='" + G1_Tambon.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_ampore]='" + G1_ampore.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_province]='" + G1_Province.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_code]='" + G1_Code.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_tel]='" + G1_Tel.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Add_fax]='" + G1_Fax.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_Name]='" + G2_Work.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_no]='" + G2_No.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_moo]='" + G2_Moo.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_soi]='" + G2_Soi.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_road]='" + G2_Road.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_tambon]='" + G2_Tambon.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_ampore]='" + G2_ampore.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_province]='" + G2_Province.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_code]='" + G2_Code.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_tel]='" + G2_Tel.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_fax]='" + G2_Fax.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Type_Main]='" + Rtype_Main.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(" where id=", EditID));
				Module1.connect(Conversions.ToString(left2));
				Search();
			}
		}
	}

	private void ButtonX_2_Click(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการล\u0e39กค\u0e49า");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("delete from HT_Customers where id=" + ListViewEx1.SelectedItems[0].SubItems[0].Text);
			Search();
		}
	}

	private void GroupBox2_Enter(object sender, EventArgs e)
	{
	}

	private void Rtype_Main_SelectedIndexChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Order_Up where cast_type='" + Rtype_Main.Text + "' order by id");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			Rtype.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["cust_type"]);
		}
	}

	private void TextBoxX1_TextChanged(object sender, EventArgs e)
	{
		if (TextBoxX_0.TextLength != 0)
		{
			TextBoxX_1.Text = "";
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
		}
		if (TextBoxX_1.TextLength >= 2)
		{
			Search();
		}
	}
}
