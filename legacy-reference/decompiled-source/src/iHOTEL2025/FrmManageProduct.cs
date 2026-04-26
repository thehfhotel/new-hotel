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

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmManageProduct : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("Tamt")]
	private TextBoxX _Tamt;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Tunit")]
	private TextBoxX _Tunit;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("DataGridView1")]
	private DataGridView _DataGridView1;

	[AccessedThroughProperty("Column1")]
	private DataGridViewTextBoxColumn _Column1;

	[AccessedThroughProperty("ราคา")]
	private DataGridViewTextBoxColumn dataGridViewTextBoxColumn_0;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Tcap")]
	private TextBoxX _Tcap;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("Search_type")]
	private ComboBox _Search_type;

	[AccessedThroughProperty("search_name")]
	private TextBoxX _search_name;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("CheckBox2")]
	private CheckBox _CheckBox2;

	[AccessedThroughProperty("TextBoxX1")]
	private TextBoxX _TextBoxX1;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	public string EditID;

	public string EditID2;

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

	internal virtual TextBoxX Tamt
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tamt;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tamt = value;
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

	internal virtual TextBoxX Tunit
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tunit;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tunit = value;
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

	internal virtual DataGridView DataGridView1
	{
		[DebuggerNonUserCode]
		get
		{
			return _DataGridView1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DataGridView1 = value;
		}
	}

	internal virtual DataGridViewTextBoxColumn Column1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Column1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Column1 = value;
		}
	}

	internal virtual DataGridViewTextBoxColumn DataGridViewTextBoxColumn_0
	{
		[DebuggerNonUserCode]
		get
		{
			return dataGridViewTextBoxColumn_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			dataGridViewTextBoxColumn_0 = value;
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

	internal virtual TextBoxX Tcap
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcap;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcap = value;
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

	internal virtual ComboBox Search_type
	{
		[DebuggerNonUserCode]
		get
		{
			return _Search_type;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Search_type_SelectedIndexChanged;
			if (_Search_type != null)
			{
				_Search_type.SelectedIndexChanged -= value2;
			}
			_Search_type = value;
			if (_Search_type != null)
			{
				_Search_type.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual TextBoxX search_name
	{
		[DebuggerNonUserCode]
		get
		{
			return _search_name;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = search_name_TextChanged;
			if (_search_name != null)
			{
				_search_name.TextChanged -= value2;
			}
			_search_name = value;
			if (_search_name != null)
			{
				_search_name.TextChanged += value2;
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
			_CheckBox1 = value;
		}
	}

	internal virtual CheckBox CheckBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CheckBox2 = value;
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
			_TextBoxX1 = value;
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

	[DebuggerNonUserCode]
	static FrmManageProduct()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmManageProduct()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmManageRoom_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EditID = "";
		EditID2 = "";
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
		System.Windows.Forms.DataGridViewCellStyle dataGridViewCellStyle = new System.Windows.Forms.DataGridViewCellStyle();
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ButtonX_2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_4 = new DevComponents.DotNetBar.ButtonX();
		this.Search_type = new System.Windows.Forms.ComboBox();
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
		this.search_name = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.CheckBox2 = new System.Windows.Forms.CheckBox();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label9 = new System.Windows.Forms.Label();
		this.DataGridView1 = new System.Windows.Forms.DataGridView();
		this.Column1 = new System.Windows.Forms.DataGridViewTextBoxColumn();
		this.DataGridViewTextBoxColumn_0 = new System.Windows.Forms.DataGridViewTextBoxColumn();
		this.Rtype = new System.Windows.Forms.ComboBox();
		this.Tcap = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tunit = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tamt = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.RpriceC = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label10 = new System.Windows.Forms.Label();
		this.RpriceB = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label6 = new System.Windows.Forms.Label();
		this.RpriceA = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label1 = new System.Windows.Forms.Label();
		this.Rno = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label3 = new System.Windows.Forms.Label();
		this.Rdtails = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label14 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.TextBoxX_0 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label13 = new System.Windows.Forms.Label();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.DataGridView1).BeginInit();
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
		this.GroupBox1.Controls.Add(this.Search_type);
		this.GroupBox1.Controls.Add(this.ListViewEx1);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.search_name);
		this.GroupBox1.Controls.Add(this.Label11);
		this.GroupBox1.Controls.Add(this.Label12);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(7, 35);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(654, 541);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX_2;
		location = new System.Drawing.Point(170, 508);
		buttonX.Location = location;
		this.ButtonX_2.Name = "ลบ";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX_2;
		size = new System.Drawing.Size(75, 23);
		buttonX2.Size = size;
		this.ButtonX_2.TabIndex = 5;
		this.ButtonX_2.Text = "ลบ";
		this.ButtonX_3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX_3;
		location = new System.Drawing.Point(90, 508);
		buttonX3.Location = location;
		this.ButtonX_3.Name = "แก\u0e49ไข";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX_3;
		size = new System.Drawing.Size(75, 23);
		buttonX4.Size = size;
		this.ButtonX_3.TabIndex = 4;
		this.ButtonX_3.Text = "แก\u0e49ไข";
		this.ButtonX_4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX_4;
		location = new System.Drawing.Point(11, 508);
		buttonX5.Location = location;
		this.ButtonX_4.Name = "เพ\u0e34\u0e48ม";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX_4;
		size = new System.Drawing.Size(75, 23);
		buttonX6.Size = size;
		this.ButtonX_4.TabIndex = 3;
		this.ButtonX_4.Text = "เพ\u0e34\u0e48ม";
		this.Search_type.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Search_type.FormattingEnabled = true;
		System.Windows.Forms.ComboBox search_type = this.Search_type;
		location = new System.Drawing.Point(381, 25);
		search_type.Location = location;
		this.Search_type.Name = "Search_type";
		System.Windows.Forms.ComboBox search_type2 = this.Search_type;
		size = new System.Drawing.Size(227, 24);
		search_type2.Size = size;
		this.Search_type.TabIndex = 0;
		this.ListViewEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListViewEx1.Atto_กระดาษแนวนอน = true;
		this.ListViewEx1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[12]
		{
			this.ColumnHeader7, this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader12, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader8, this.ColumnHeader9,
			this.ColumnHeader10, this.ColumnHeader11
		});
		this.ListViewEx1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListViewEx1.FitToPage = true;
		this.ListViewEx1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListViewEx1.FullRowSelect = true;
		this.ListViewEx1.GridLines = true;
		global::PrintableListView.PrintableListView listViewEx = this.ListViewEx1;
		location = new System.Drawing.Point(11, 55);
		listViewEx.Location = location;
		this.ListViewEx1.MultiSelect = false;
		this.ListViewEx1.Name = "ListViewEx1";
		global::PrintableListView.PrintableListView listViewEx2 = this.ListViewEx1;
		size = new System.Drawing.Size(628, 447);
		listViewEx2.Size = size;
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
		this.ColumnHeader6.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		this.ColumnHeader6.Width = 70;
		this.ColumnHeader2.Text = "ประเภทส\u0e34นค\u0e49า";
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader3.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
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
		this.ColumnHeader9.Text = "จำนวนคงเหล\u0e37อ";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 100;
		this.ColumnHeader10.Width = 0;
		this.ColumnHeader11.Text = "ราคาท\u0e38น";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
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
		location = new System.Drawing.Point(28, 22);
		label.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label2 = this.Label7;
		size = new System.Drawing.Size(0, 16);
		label2.Size = size;
		this.Label7.TabIndex = 11;
		this.search_name.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX = this.search_name;
		location = new System.Drawing.Point(69, 25);
		textBoxX.Location = location;
		this.search_name.MaxLength = 255;
		this.search_name.Name = "search_name";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX2 = this.search_name;
		size = new System.Drawing.Size(227, 23);
		textBoxX2.Size = size;
		this.search_name.TabIndex = 2;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label11;
		location = new System.Drawing.Point(13, 29);
		label3.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label4 = this.Label11;
		size = new System.Drawing.Size(54, 16);
		label4.Size = size;
		this.Label11.TabIndex = 11;
		this.Label11.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label12;
		location = new System.Drawing.Point(300, 30);
		label5.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label6 = this.Label12;
		size = new System.Drawing.Size(79, 16);
		label6.Size = size;
		this.Label12.TabIndex = 11;
		this.Label12.Text = "ประเภทส\u0e34นค\u0e49า";
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox2.Controls.Add(this.TextBoxX_0);
		this.GroupBox2.Controls.Add(this.Label13);
		this.GroupBox2.Controls.Add(this.CheckBox2);
		this.GroupBox2.Controls.Add(this.CheckBox1);
		this.GroupBox2.Controls.Add(this.Button1);
		this.GroupBox2.Controls.Add(this.Label9);
		this.GroupBox2.Controls.Add(this.DataGridView1);
		this.GroupBox2.Controls.Add(this.Rtype);
		this.GroupBox2.Controls.Add(this.Tcap);
		this.GroupBox2.Controls.Add(this.Tunit);
		this.GroupBox2.Controls.Add(this.Tamt);
		this.GroupBox2.Controls.Add(this.RpriceC);
		this.GroupBox2.Controls.Add(this.Label10);
		this.GroupBox2.Controls.Add(this.RpriceB);
		this.GroupBox2.Controls.Add(this.Label6);
		this.GroupBox2.Controls.Add(this.RpriceA);
		this.GroupBox2.Controls.Add(this.Label1);
		this.GroupBox2.Controls.Add(this.Rno);
		this.GroupBox2.Controls.Add(this.Label3);
		this.GroupBox2.Controls.Add(this.Rdtails);
		this.GroupBox2.Controls.Add(this.Label2);
		this.GroupBox2.Controls.Add(this.Label14);
		this.GroupBox2.Controls.Add(this.Label4);
		this.GroupBox2.Controls.Add(this.Label8);
		this.GroupBox2.Controls.Add(this.Label5);
		this.GroupBox2.Controls.Add(this.ButtonX_0);
		this.GroupBox2.Controls.Add(this.ButtonX_1);
		this.GroupBox2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox2;
		location = new System.Drawing.Point(667, 35);
		groupBox3.Location = location;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox2;
		size = new System.Drawing.Size(436, 541);
		groupBox4.Size = size;
		this.GroupBox2.TabIndex = 1;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไข";
		this.CheckBox2.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox2;
		location = new System.Drawing.Point(187, 165);
		checkBox.Location = location;
		this.CheckBox2.Name = "CheckBox2";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox2;
		size = new System.Drawing.Size(247, 20);
		checkBox2.Size = size;
		this.CheckBox2.TabIndex = 54;
		this.CheckBox2.Text = "แสดงหน\u0e49าคำนวนหน\u0e48วย (กรณ\u0e35ค\u0e48าน\u0e49ำ-ค\u0e48าไฟ)";
		this.CheckBox2.UseVisualStyleBackColor = true;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox3 = this.CheckBox1;
		location = new System.Drawing.Point(187, 137);
		checkBox3.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox4 = this.CheckBox1;
		size = new System.Drawing.Size(119, 20);
		checkBox4.Size = size;
		this.CheckBox1.TabIndex = 53;
		this.CheckBox1.Text = "ออกใบกำก\u0e31บภาษ\u0e35";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.Button1.Enabled = false;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(283, 223);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(140, 23);
		button2.Size = size;
		this.Button1.TabIndex = 0;
		this.Button1.Text = "ใช\u0e49ราคาเด\u0e35ยวก\u0e31นท\u0e31\u0e49งหมด";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label9;
		location = new System.Drawing.Point(14, 229);
		label7.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label8 = this.Label9;
		size = new System.Drawing.Size(56, 16);
		label8.Size = size;
		this.Label9.TabIndex = 51;
		this.Label9.Text = "ราคาขาย";
		this.DataGridView1.AllowUserToAddRows = false;
		this.DataGridView1.AllowUserToDeleteRows = false;
		this.DataGridView1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.DataGridView1.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize;
		this.DataGridView1.Columns.AddRange(this.Column1, this.DataGridViewTextBoxColumn_0);
		System.Windows.Forms.DataGridView dataGridView = this.DataGridView1;
		location = new System.Drawing.Point(17, 250);
		dataGridView.Location = location;
		this.DataGridView1.Name = "DataGridView1";
		System.Windows.Forms.DataGridView dataGridView2 = this.DataGridView1;
		size = new System.Drawing.Size(406, 252);
		dataGridView2.Size = size;
		this.DataGridView1.TabIndex = 50;
		this.Column1.Frozen = true;
		this.Column1.HeaderText = "ประเภทล\u0e39กค\u0e49า";
		this.Column1.Name = "Column1";
		this.Column1.ReadOnly = true;
		this.Column1.Width = 250;
		dataGridViewCellStyle.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleRight;
		this.DataGridViewTextBoxColumn_0.DefaultCellStyle = dataGridViewCellStyle;
		this.DataGridViewTextBoxColumn_0.HeaderText = "ราคา";
		this.DataGridViewTextBoxColumn_0.Name = "ราคา";
		this.Rtype.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Rtype.Enabled = false;
		this.Rtype.FormattingEnabled = true;
		System.Windows.Forms.ComboBox rtype = this.Rtype;
		location = new System.Drawing.Point(89, 24);
		rtype.Location = location;
		this.Rtype.Name = "Rtype";
		System.Windows.Forms.ComboBox rtype2 = this.Rtype;
		size = new System.Drawing.Size(227, 24);
		rtype2.Size = size;
		this.Rtype.TabIndex = 0;
		this.Tcap.BackColor = System.Drawing.Color.White;
		this.Tcap.Border.Class = "TextBoxBorder";
		this.Tcap.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX tcap = this.Tcap;
		location = new System.Drawing.Point(89, 190);
		tcap.Location = location;
		this.Tcap.Name = "Tcap";
		DevComponents.DotNetBar.Controls.TextBoxX tcap2 = this.Tcap;
		size = new System.Drawing.Size(77, 23);
		tcap2.Size = size;
		this.Tcap.TabIndex = 5;
		this.Tunit.BackColor = System.Drawing.Color.White;
		this.Tunit.Border.Class = "TextBoxBorder";
		this.Tunit.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX tunit = this.Tunit;
		location = new System.Drawing.Point(89, 163);
		tunit.Location = location;
		this.Tunit.Name = "Tunit";
		DevComponents.DotNetBar.Controls.TextBoxX tunit2 = this.Tunit;
		size = new System.Drawing.Size(77, 23);
		tunit2.Size = size;
		this.Tunit.TabIndex = 5;
		this.Tamt.BackColor = System.Drawing.Color.White;
		this.Tamt.Border.Class = "TextBoxBorder";
		this.Tamt.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX tamt = this.Tamt;
		location = new System.Drawing.Point(89, 136);
		tamt.Location = location;
		this.Tamt.Name = "Tamt";
		DevComponents.DotNetBar.Controls.TextBoxX tamt2 = this.Tamt;
		size = new System.Drawing.Size(77, 23);
		tamt2.Size = size;
		this.Tamt.TabIndex = 5;
		this.RpriceC.BackColor = System.Drawing.Color.White;
		this.RpriceC.Border.Class = "TextBoxBorder";
		this.RpriceC.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceC = this.RpriceC;
		location = new System.Drawing.Point(89, 135);
		rpriceC.Location = location;
		this.RpriceC.Name = "RpriceC";
		DevComponents.DotNetBar.Controls.TextBoxX rpriceC2 = this.RpriceC;
		size = new System.Drawing.Size(77, 23);
		rpriceC2.Size = size;
		this.RpriceC.TabIndex = 5;
		this.RpriceC.Visible = false;
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label10;
		location = new System.Drawing.Point(20, 193);
		label9.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label10 = this.Label10;
		size = new System.Drawing.Size(67, 16);
		label10.Size = size;
		this.Label10.TabIndex = 11;
		this.Label10.Text = "ราคาต\u0e49นท\u0e38น";
		this.RpriceB.BackColor = System.Drawing.Color.White;
		this.RpriceB.Border.Class = "TextBoxBorder";
		this.RpriceB.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceB = this.RpriceB;
		location = new System.Drawing.Point(89, 135);
		rpriceB.Location = location;
		this.RpriceB.Name = "RpriceB";
		DevComponents.DotNetBar.Controls.TextBoxX rpriceB2 = this.RpriceB;
		size = new System.Drawing.Size(77, 23);
		rpriceB2.Size = size;
		this.RpriceB.TabIndex = 4;
		this.RpriceB.Visible = false;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label6;
		location = new System.Drawing.Point(49, 166);
		label11.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label12 = this.Label6;
		size = new System.Drawing.Size(38, 16);
		label12.Size = size;
		this.Label6.TabIndex = 11;
		this.Label6.Text = "หน\u0e48วย";
		this.RpriceA.BackColor = System.Drawing.Color.White;
		this.RpriceA.Border.Class = "TextBoxBorder";
		this.RpriceA.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rpriceA = this.RpriceA;
		location = new System.Drawing.Point(89, 135);
		rpriceA.Location = location;
		this.RpriceA.Name = "RpriceA";
		DevComponents.DotNetBar.Controls.TextBoxX rpriceA2 = this.RpriceA;
		size = new System.Drawing.Size(77, 23);
		rpriceA2.Size = size;
		this.RpriceA.TabIndex = 3;
		this.RpriceA.Visible = false;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label1;
		location = new System.Drawing.Point(3, 140);
		label13.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label14 = this.Label1;
		size = new System.Drawing.Size(85, 16);
		label14.Size = size;
		this.Label1.TabIndex = 11;
		this.Label1.Text = "จำนวนคงเหล\u0e37อ";
		this.Rno.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Rno.Border.Class = "TextBoxBorder";
		this.Rno.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rno = this.Rno;
		location = new System.Drawing.Point(89, 53);
		rno.Location = location;
		this.Rno.MaxLength = 255;
		this.Rno.Name = "Rno";
		this.Rno.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX rno2 = this.Rno;
		size = new System.Drawing.Size(88, 23);
		rno2.Size = size;
		this.Rno.TabIndex = 1;
		this.Rno.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label3;
		location = new System.Drawing.Point(41, 139);
		label15.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label16 = this.Label3;
		size = new System.Drawing.Size(46, 16);
		label16.Size = size;
		this.Label3.TabIndex = 11;
		this.Label3.Text = "ราคา C";
		this.Label3.Visible = false;
		this.Rdtails.Border.Class = "TextBoxBorder";
		this.Rdtails.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rdtails = this.Rdtails;
		location = new System.Drawing.Point(89, 109);
		rdtails.Location = location;
		this.Rdtails.MaxLength = 255;
		this.Rdtails.Name = "Rdtails";
		DevComponents.DotNetBar.Controls.TextBoxX rdtails2 = this.Rdtails;
		size = new System.Drawing.Size(227, 23);
		rdtails2.Size = size;
		this.Rdtails.TabIndex = 2;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label2;
		location = new System.Drawing.Point(41, 139);
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
		location = new System.Drawing.Point(41, 139);
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
		location = new System.Drawing.Point(33, 113);
		label21.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label22 = this.Label4;
		size = new System.Drawing.Size(54, 16);
		label22.Size = size;
		this.Label4.TabIndex = 11;
		this.Label4.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label8;
		location = new System.Drawing.Point(28, 57);
		label23.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label24 = this.Label8;
		size = new System.Drawing.Size(60, 16);
		label24.Size = size;
		this.Label8.TabIndex = 11;
		this.Label8.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label5;
		location = new System.Drawing.Point(8, 29);
		label25.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label26 = this.Label5;
		size = new System.Drawing.Size(79, 16);
		label26.Size = size;
		this.Label5.TabIndex = 11;
		this.Label5.Text = "ประเภทส\u0e34นค\u0e49า";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.Enabled = false;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX_0;
		location = new System.Drawing.Point(348, 508);
		buttonX7.Location = location;
		this.ButtonX_0.Name = "ยกเล\u0e34ก";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX_0;
		size = new System.Drawing.Size(75, 23);
		buttonX8.Size = size;
		this.ButtonX_0.TabIndex = 7;
		this.ButtonX_0.Text = "ยกเล\u0e34ก";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_1.Enabled = false;
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX_1;
		location = new System.Drawing.Point(267, 508);
		buttonX9.Location = location;
		this.ButtonX_1.Name = "บ\u0e31นท\u0e36ก";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX_1;
		size = new System.Drawing.Size(75, 23);
		buttonX10.Size = size;
		this.ButtonX_1.TabIndex = 6;
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
		this.PanelEx2.Text = "จ\u0e31ดการส\u0e34นค\u0e49า";
		this.Timer2.Enabled = true;
		this.OpenFileDialog1.RestoreDirectory = true;
		this.TextBoxX_0.BackColor = System.Drawing.Color.FromArgb(192, 255, 255);
		this.TextBoxX_0.Border.Class = "TextBoxBorder";
		this.TextBoxX_0.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_ = this.TextBoxX_0;
		location = new System.Drawing.Point(89, 81);
		textBoxX_.Location = location;
		this.TextBoxX_0.MaxLength = 40;
		this.TextBoxX_0.Name = "TextBoxX1";
		DevComponents.DotNetBar.Controls.TextBoxX textBoxX_2 = this.TextBoxX_0;
		size = new System.Drawing.Size(227, 23);
		textBoxX_2.Size = size;
		this.TextBoxX_0.TabIndex = 55;
		this.TextBoxX_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label13;
		location = new System.Drawing.Point(37, 85);
		label27.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label28 = this.Label13;
		size = new System.Drawing.Size(50, 16);
		label28.Size = size;
		this.Label13.TabIndex = 56;
		this.Label13.Text = "บาร\u0e4cโค\u0e4aด";
		this.ColumnHeader12.Text = "บาร\u0e4cโค\u0e4aด";
		this.ColumnHeader12.Width = 120;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1112, 586);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox1);
		this.Controls.Add(this.GroupBox2);
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmManageProduct";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "จ\u0e31ดการส\u0e34นค\u0e49า";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.DataGridView1).EndInit();
		this.ResumeLayout(false);
	}

	private void FrmManageRoom_Load(object sender, EventArgs e)
	{
		Listtype();
	}

	public void Search()
	{
		cancel();
		object obj = "";
		if (Operators.CompareString(search_name.Text, "", TextCompare: false) != 0)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat(" and Pro_Name like '%" + search_name.Text, "%' "));
		}
		if (Operators.CompareString(Search_type.Text, "", TextCompare: false) != 0)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat(" and Pro_Type like '%" + Search_type.Text, "%' "));
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products where pro_no<>'P001' ", obj), " order by Pro_no")));
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
					columnName = "Pro_no";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listViewEx.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Pro_Barcode"].ToString());
					ListViewItem.ListViewSubItemCollection subItems2 = listViewEx.Items[num2].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "Pro_Type";
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
					columnName = "Pro_Name";
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
					columnName = "Pro_PriceA";
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
					columnName = "Pro_PriceB";
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
					columnName = "Pro_PriceC";
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
					columnName = "Pro_amt";
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
					columnName = "Pro_Unit";
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
					columnName = "Pro_cap";
					array13[0] = RuntimeHelpers.GetObjectValue(dataRow11[columnName]);
					array = array3;
					object[] arguments10 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems9, null, "Add", arguments10, null, null, array4, IgnoreReturn: true);
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
		DataSet dataSet = Module1.connect("select * from HT_SET_ProductType order by name");
		Rtype.DataSource = dataSet.Tables[0];
		Rtype.DisplayMember = "name";
		Search_type.Items.Clear();
		Search_type.Items.Add("");
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
				Search_type.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			Search_type.SelectedIndex = 0;
		}
	}

	private void ListViewEx1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count == 0)
		{
			return;
		}
		cancel();
		ButtonX_4.Enabled = true;
		ButtonX_3.Enabled = true;
		ButtonX_2.Enabled = true;
		Rtype.Text = ListViewEx1.SelectedItems[0].SubItems[4].Text;
		Rno.Text = ListViewEx1.SelectedItems[0].SubItems[2].Text;
		TextBoxX_0.Text = ListViewEx1.SelectedItems[0].SubItems[3].Text;
		Rdtails.Text = ListViewEx1.SelectedItems[0].SubItems[5].Text;
		RpriceA.Text = ListViewEx1.SelectedItems[0].SubItems[6].Text;
		RpriceB.Text = ListViewEx1.SelectedItems[0].SubItems[7].Text;
		RpriceC.Text = ListViewEx1.SelectedItems[0].SubItems[8].Text;
		Tamt.Text = ListViewEx1.SelectedItems[0].SubItems[9].Text;
		Tunit.Text = ListViewEx1.SelectedItems[0].SubItems[10].Text;
		Tcap.Text = ListViewEx1.SelectedItems[0].SubItems[11].Text;
		if (decimal.Compare(Conversions.ToDecimal(ListViewEx1.SelectedItems[0].SubItems[6].Text), 0m) != 0)
		{
			CheckBox1.Checked = true;
		}
		else
		{
			CheckBox1.Checked = false;
		}
		if (decimal.Compare(Conversions.ToDecimal(ListViewEx1.SelectedItems[0].SubItems[7].Text), 0m) != 0)
		{
			CheckBox2.Checked = true;
		}
		else
		{
			CheckBox2.Checked = false;
		}
		DataGridView1.Rows.Clear();
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
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
				DataGridView1.Rows.Add();
				DataGridView1.Rows[num2].Cells[0].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]);
				DataGridView1.Rows[num2].Cells[1].Value = 0;
				num2++;
			}
			dataSet = Module1.connect("select * from HT_Products_Price where P_ID='" + ListViewEx1.SelectedItems[0].SubItems[2].Text + "'");
			int num5 = DataGridView1.Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				int num8 = dataSet.Tables[0].Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(Conversions.ToString(DataGridView1.Rows[num6].Cells[0].Value), dataSet.Tables[0].Rows[num9]["P_CustType"], TextCompare: false))
					{
						DataGridView1.Rows[num6].Cells[1].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["P_Price"]);
					}
					num9++;
				}
				num6++;
			}
		}
	}

	private void ButtonX_4_Click(object sender, EventArgs e)
	{
		cancel();
		CheckBox1.Enabled = true;
		CheckBox2.Enabled = true;
		Button1.Enabled = true;
		Rtype.Enabled = true;
		Rdtails.Enabled = true;
		RpriceA.Enabled = true;
		RpriceB.Enabled = true;
		RpriceC.Enabled = true;
		TextBoxX_0.Enabled = true;
		Tcap.Enabled = true;
		Tunit.Enabled = true;
		Tamt.Enabled = true;
		ButtonX_1.Enabled = true;
		ButtonX_0.Enabled = true;
		DataGridView1.Enabled = true;
		ButtonX_4.Enabled = false;
		ButtonX_3.Enabled = false;
		ButtonX_2.Enabled = false;
		AddNo();
		DataGridView1.Rows.Clear();
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
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
				DataGridView1.Rows.Add();
				DataGridView1.Rows[num2].Cells[0].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]);
				DataGridView1.Rows[num2].Cells[1].Value = 0;
				num2++;
			}
			Rno.Focus();
		}
	}

	public void AddNo()
	{
		try
		{
			if (Operators.CompareString(Rtype.Text, "", TextCompare: false) != 0)
			{
				DataSet dataSet = Module1.connect("select * from HT_SET_ProductType where name='" + Rtype.Text + "'");
				if (dataSet.Tables[0].Rows.Count != 0)
				{
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 1 * from HT_Products where Pro_Type  ='", dataSet.Tables[0].Rows[0]["name"]), "' order by pro_no desc")));
					if (dataSet2.Tables[0].Rows.Count == 0)
					{
						Rno.Text = Conversions.ToString(Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["id_full"], "-001"));
					}
					else
					{
						Rno.Text = Conversions.ToString(Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["id_full"], Strings.Format(checked(Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["pro_no"].ToString().Replace(Conversions.ToString(Operators.ConcatenateObject(dataSet.Tables[0].Rows[0]["id_full"], "-")), "")) + 1), "-000")));
					}
				}
				else
				{
					Rno.Text = "";
				}
			}
			else
			{
				Rno.Text = "";
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void ButtonX_3_Click(object sender, EventArgs e)
	{
		if (ListViewEx1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการห\u0e49อง");
			return;
		}
		cancel();
		CheckBox1.Enabled = true;
		CheckBox2.Enabled = true;
		Button1.Enabled = true;
		Rno.Enabled = true;
		Rtype.Enabled = true;
		Rdtails.Enabled = true;
		RpriceA.Enabled = true;
		RpriceB.Enabled = true;
		RpriceC.Enabled = true;
		Tunit.Enabled = true;
		Tamt.Enabled = true;
		Tcap.Enabled = true;
		ButtonX_1.Enabled = true;
		ButtonX_0.Enabled = true;
		ButtonX_4.Enabled = false;
		ButtonX_3.Enabled = false;
		ButtonX_2.Enabled = false;
		DataGridView1.Enabled = true;
		TextBoxX_0.Enabled = true;
		Rtype.Text = ListViewEx1.SelectedItems[0].SubItems[4].Text;
		EditID = ListViewEx1.SelectedItems[0].SubItems[0].Text;
		EditID2 = ListViewEx1.SelectedItems[0].SubItems[2].Text;
		Rno.Text = ListViewEx1.SelectedItems[0].SubItems[2].Text;
		TextBoxX_0.Text = ListViewEx1.SelectedItems[0].SubItems[3].Text;
		Rdtails.Text = ListViewEx1.SelectedItems[0].SubItems[5].Text;
		RpriceA.Text = ListViewEx1.SelectedItems[0].SubItems[6].Text;
		RpriceB.Text = ListViewEx1.SelectedItems[0].SubItems[7].Text;
		RpriceC.Text = ListViewEx1.SelectedItems[0].SubItems[8].Text;
		Tamt.Text = ListViewEx1.SelectedItems[0].SubItems[9].Text;
		Tunit.Text = ListViewEx1.SelectedItems[0].SubItems[10].Text;
		Tcap.Text = Conversions.ToString(Conversions.ToDecimal(ListViewEx1.SelectedItems[0].SubItems[11].Text));
		if (decimal.Compare(Conversions.ToDecimal(ListViewEx1.SelectedItems[0].SubItems[6].Text), 0m) != 0)
		{
			CheckBox1.Checked = true;
		}
		else
		{
			CheckBox1.Checked = false;
		}
		if (decimal.Compare(Conversions.ToDecimal(ListViewEx1.SelectedItems[0].SubItems[7].Text), 0m) != 0)
		{
			CheckBox2.Checked = true;
		}
		else
		{
			CheckBox2.Checked = false;
		}
		DataGridView1.Rows.Clear();
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
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
				DataGridView1.Rows.Add();
				DataGridView1.Rows[num2].Cells[0].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]);
				DataGridView1.Rows[num2].Cells[1].Value = 0;
				num2++;
			}
			dataSet = Module1.connect("select * from HT_Products_Price where P_ID='" + ListViewEx1.SelectedItems[0].SubItems[2].Text + "'");
			int num5 = DataGridView1.Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				int num8 = dataSet.Tables[0].Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(Conversions.ToString(DataGridView1.Rows[num6].Cells[0].Value), dataSet.Tables[0].Rows[num9]["P_CustType"], TextCompare: false))
					{
						DataGridView1.Rows[num6].Cells[1].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["P_Price"]);
					}
					num9++;
				}
				num6++;
			}
			Rno.Focus();
		}
	}

	private void ButtonX_0_Click(object sender, EventArgs e)
	{
		cancel();
	}

	public void cancel()
	{
		DataGridView1.Rows.Clear();
		Rno.Text = "";
		TextBoxX_0.Text = "";
		Rdtails.Text = "";
		RpriceA.Text = "";
		RpriceB.Text = "";
		RpriceC.Text = "";
		Tamt.Text = "";
		Tunit.Text = "";
		Tcap.Text = "";
		CheckBox1.Checked = false;
		EditID = "";
		Rtype.SelectedIndex = 0;
		CheckBox1.Enabled = false;
		CheckBox2.Enabled = false;
		Tunit.Enabled = false;
		Rno.Enabled = false;
		Rtype.Enabled = false;
		Rdtails.Enabled = false;
		Tcap.Enabled = false;
		RpriceA.Enabled = false;
		RpriceB.Enabled = false;
		RpriceC.Enabled = false;
		Tunit.Enabled = false;
		Button1.Enabled = false;
		DataGridView1.Enabled = false;
		Tamt.Enabled = false;
		ButtonX_1.Enabled = false;
		ButtonX_0.Enabled = false;
		TextBoxX_0.Enabled = false;
		ButtonX_4.Enabled = true;
		ButtonX_3.Enabled = true;
		ButtonX_2.Enabled = true;
	}

	private void ButtonX_1_Click(object sender, EventArgs e)
	{
		checked
		{
			if (Operators.CompareString(Rno.Text, "", TextCompare: false) == 0)
			{
				MessageBox.Show("กร\u0e38ณาใส\u0e48รห\u0e31สส\u0e34นค\u0e49า");
			}
			else
			{
				if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.Yes)
				{
					return;
				}
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
					RpriceB.Text = Conversions.ToString(0);
				}
				if (!Versioned.IsNumeric(RpriceB.Text))
				{
					RpriceB.Text = Conversions.ToString(0);
				}
				if (Operators.CompareString(RpriceC.Text, "", TextCompare: false) == 0)
				{
					RpriceC.Text = Conversions.ToString(0);
				}
				if (!Versioned.IsNumeric(RpriceC.Text))
				{
					RpriceC.Text = Conversions.ToString(0);
				}
				if (Operators.CompareString(Tamt.Text, "", TextCompare: false) == 0)
				{
					Tamt.Text = Conversions.ToString(0);
				}
				if (!Versioned.IsNumeric(Tamt.Text))
				{
					Tamt.Text = Conversions.ToString(0);
				}
				if (Operators.CompareString(Tcap.Text, "", TextCompare: false) == 0)
				{
					Tcap.Text = Conversions.ToString(0);
				}
				if (!Versioned.IsNumeric(Tcap.Text))
				{
					Tcap.Text = Conversions.ToString(0);
				}
				if (CheckBox1.Checked)
				{
					RpriceA.Text = Conversions.ToString(1);
				}
				else
				{
					RpriceA.Text = Conversions.ToString(0);
				}
				if (CheckBox2.Checked)
				{
					RpriceB.Text = Conversions.ToString(1);
				}
				else
				{
					RpriceB.Text = Conversions.ToString(0);
				}
				if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
				{
					object right = Module1.get_id("HT_Products", "id");
					object left = "INSERT INTO [HT_Products]";
					left = Operators.ConcatenateObject(left, "([id]");
					left = Operators.ConcatenateObject(left, ",[Pro_no]");
					left = Operators.ConcatenateObject(left, ",[Pro_Type]");
					left = Operators.ConcatenateObject(left, ",[Pro_Name]");
					left = Operators.ConcatenateObject(left, ",[Pro_PriceA]");
					left = Operators.ConcatenateObject(left, ",[Pro_PriceB]");
					left = Operators.ConcatenateObject(left, ",[Pro_PriceC],[Pro_amt],[Pro_Unit],[pro_cap],[Pro_Barcode])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, right);
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Rno.Text, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Rtype.Text, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Rdtails.Text, "'"));
					left = Operators.ConcatenateObject(left, "," + RpriceA.Text);
					left = Operators.ConcatenateObject(left, "," + RpriceB.Text);
					left = Operators.ConcatenateObject(left, "," + RpriceC.Text);
					left = Operators.ConcatenateObject(left, "," + Tamt.Text);
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Tunit.Text, "'"));
					left = Operators.ConcatenateObject(left, "," + Tcap.Text);
					left = Operators.ConcatenateObject(left, string.Concat(",'" + TextBoxX_0.Text, "'"));
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
					int num = DataGridView1.Rows.Count - 1;
					int num2 = 0;
					while (true)
					{
						int num3 = num2;
						int num4 = num;
						if (num3 > num4)
						{
							break;
						}
						left = "INSERT INTO [HT_Products_Price]";
						left = Operators.ConcatenateObject(left, "([P_ID],[P_CustType],[P_Price])");
						left = Operators.ConcatenateObject(left, "VALUES");
						left = Operators.ConcatenateObject(left, "(");
						left = Operators.ConcatenateObject(left, string.Concat("'" + Rno.Text, "'"));
						left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DataGridView1.Rows[num2].Cells[0].Value), "'"));
						left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num2].Cells[1].Value)));
						left = Operators.ConcatenateObject(left, ")");
						Module1.connect(Conversions.ToString(left));
						num2++;
					}
					Search();
					return;
				}
				object left2 = "UPDATE [HT_Products] SET ";
				left2 = Operators.ConcatenateObject(left2, string.Concat("[Pro_no]='" + Rno.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Pro_Type]='" + Rtype.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Pro_Name]='" + Rdtails.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, ",[Pro_PriceA]=" + RpriceA.Text);
				left2 = Operators.ConcatenateObject(left2, ",[Pro_PriceB]=" + RpriceB.Text);
				left2 = Operators.ConcatenateObject(left2, ",[Pro_PriceC]=" + RpriceC.Text);
				left2 = Operators.ConcatenateObject(left2, ",[Pro_Amt]=" + Tamt.Text);
				left2 = Operators.ConcatenateObject(left2, ",[Pro_cap]=" + Tcap.Text);
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Pro_Unit]='" + Tunit.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Pro_Barcode]='" + TextBoxX_0.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(" where id=", EditID));
				Module1.connect(Conversions.ToString(left2));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Products_Price where P_ID='", EditID2), "'")));
				int num5 = DataGridView1.Rows.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					left2 = "INSERT INTO [HT_Products_Price]";
					left2 = Operators.ConcatenateObject(left2, "([P_ID],[P_CustType],[P_Price])");
					left2 = Operators.ConcatenateObject(left2, "VALUES");
					left2 = Operators.ConcatenateObject(left2, "(");
					left2 = Operators.ConcatenateObject(left2, string.Concat("'" + Rno.Text, "'"));
					left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Conversions.ToString(DataGridView1.Rows[num6].Cells[0].Value), "'"));
					left2 = Operators.ConcatenateObject(left2, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num6].Cells[1].Value)));
					left2 = Operators.ConcatenateObject(left2, ")");
					Module1.connect(Conversions.ToString(left2));
					num6++;
				}
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
		else if (Operators.CompareString(ListViewEx1.SelectedItems[0].SubItems[2].Text, "P001", TextCompare: false) == 0)
		{
			MessageBox.Show("ไม\u0e48สามารถลบรายการน\u0e35\u0e49ได\u0e49", "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("delete from HT_Products where id=" + ListViewEx1.SelectedItems[0].SubItems[0].Text);
			Module1.connect("delete from HT_Products_Price where P_ID='" + ListViewEx1.SelectedItems[0].SubItems[2].Text + "'");
			Search();
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		object obj = Interaction.InputBox("กร\u0e38ณาใส\u0e48ราคา", "กร\u0e38ณาใส\u0e48ราคา");
		if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			return;
		}
		if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
		{
			MessageBox.Show("กร\u0e38ณากรอกราคาเป\u0e47นต\u0e31วเลข");
			return;
		}
		checked
		{
			int num = DataGridView1.Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					DataGridView1.Rows[num2].Cells[1].Value = RuntimeHelpers.GetObjectValue(obj);
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void Rtype_SelectedIndexChanged(object sender, EventArgs e)
	{
		AddNo();
	}

	private void Search_type_SelectedIndexChanged(object sender, EventArgs e)
	{
		Search();
	}

	private void search_name_TextChanged(object sender, EventArgs e)
	{
		Search();
	}
}
