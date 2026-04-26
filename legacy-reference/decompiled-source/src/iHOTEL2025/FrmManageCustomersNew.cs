using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmManageCustomersNew : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

	[AccessedThroughProperty("GroupBox2")]
	private GroupBox _GroupBox2;

	[AccessedThroughProperty("ยกเล\u0e34ก")]
	private ButtonX buttonX_0;

	[AccessedThroughProperty("บ\u0e31นท\u0e36ก")]
	private ButtonX buttonX_1;

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

	[AccessedThroughProperty("GroupBox3")]
	private GroupBox _GroupBox3;

	[AccessedThroughProperty("ItemPanel1")]
	private ItemPanel _ItemPanel1;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Rsex")]
	private ComboBox _Rsex;

	[AccessedThroughProperty("Rperfix")]
	private ComboBox _Rperfix;

	[AccessedThroughProperty("Label29")]
	private Label _Label29;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Ridcard")]
	private TextBoxX _Ridcard;

	[AccessedThroughProperty("Label30")]
	private Label _Label30;

	[AccessedThroughProperty("Label32")]
	private Label _Label32;

	[AccessedThroughProperty("Tover")]
	private TextBoxX _Tover;

	[AccessedThroughProperty("Label31")]
	private Label _Label31;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Tcontry")]
	private TextBoxX _Tcontry;

	[AccessedThroughProperty("Label33")]
	private Label _Label33;

	[AccessedThroughProperty("Label34")]
	private Label _Label34;

	[AccessedThroughProperty("G2_TAX")]
	private TextBoxX _G2_TAX;

	[AccessedThroughProperty("Label35")]
	private Label _Label35;

	[AccessedThroughProperty("ButtonItem2")]
	private ButtonItem _ButtonItem2;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	public string EditID;

	public string tmp_no;

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

	internal virtual ComboBox Rsex
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rsex;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Rtype_Main_SelectedIndexChanged;
			if (_Rsex != null)
			{
				_Rsex.SelectedIndexChanged -= value2;
			}
			_Rsex = value;
			if (_Rsex != null)
			{
				_Rsex.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual ComboBox Rperfix
	{
		[DebuggerNonUserCode]
		get
		{
			return _Rperfix;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Rtype_Main_SelectedIndexChanged;
			if (_Rperfix != null)
			{
				_Rperfix.SelectedIndexChanged -= value2;
			}
			_Rperfix = value;
			if (_Rperfix != null)
			{
				_Rperfix.SelectedIndexChanged += value2;
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

	internal virtual TextBoxX Ridcard
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ridcard;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ridcard = value;
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

	internal virtual Label Label32
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label32;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label32 = value;
		}
	}

	internal virtual TextBoxX Tover
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tover;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tover = value;
		}
	}

	internal virtual Label Label31
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label31;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label31 = value;
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

	internal virtual TextBoxX Tcontry
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcontry;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcontry = value;
		}
	}

	internal virtual Label Label33
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label33;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label33 = value;
		}
	}

	internal virtual Label Label34
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label34;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label34 = value;
		}
	}

	internal virtual TextBoxX G2_TAX
	{
		[DebuggerNonUserCode]
		get
		{
			return _G2_TAX;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_G2_TAX = value;
		}
	}

	internal virtual Label Label35
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label35;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label35 = value;
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
			_ButtonItem2 = value;
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

	[DebuggerNonUserCode]
	static FrmManageCustomersNew()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmManageCustomersNew()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmManageRoom_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		EditID = "";
		tmp_no = "";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmManageCustomersNew));
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.AddMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.EditMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.DelMenu = new System.Windows.Forms.ToolStripMenuItem();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.Label35 = new System.Windows.Forms.Label();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.Label32 = new System.Windows.Forms.Label();
		this.G2 = new System.Windows.Forms.GroupBox();
		this.Label34 = new System.Windows.Forms.Label();
		this.G2_TAX = new DevComponents.DotNetBar.Controls.TextBoxX();
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
		this.GroupBox3 = new System.Windows.Forms.GroupBox();
		this.ItemPanel1 = new DevComponents.DotNetBar.ItemPanel();
		this.Tover = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G1 = new System.Windows.Forms.GroupBox();
		this.G1_No = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Tcontry = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.G1_Fax = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label9 = new System.Windows.Forms.Label();
		this.G1_Tel = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label33 = new System.Windows.Forms.Label();
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
		this.Rsex = new System.Windows.Forms.ComboBox();
		this.Rperfix = new System.Windows.Forms.ComboBox();
		this.Rtype_Main = new System.Windows.Forms.ComboBox();
		this.Label31 = new System.Windows.Forms.Label();
		this.Rtype = new System.Windows.Forms.ComboBox();
		this.Rno = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Rname2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Rname = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Ridcard = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Remail = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label29 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label30 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label28 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Button1 = new System.Windows.Forms.Button();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem2 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.ContextMenuStrip1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.G2.SuspendLayout();
		this.GroupBox3.SuspendLayout();
		this.G1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[3] { this.AddMenu, this.EditMenu, this.DelMenu });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		System.Drawing.Size size = new System.Drawing.Size(102, 70);
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
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox2.Controls.Add(this.ButtonX7);
		this.GroupBox2.Controls.Add(this.Label35);
		this.GroupBox2.Controls.Add(this.ButtonX2);
		this.GroupBox2.Controls.Add(this.GroupBox1);
		this.GroupBox2.Controls.Add(this.ButtonX1);
		this.GroupBox2.Controls.Add(this.Label32);
		this.GroupBox2.Controls.Add(this.Button1);
		this.GroupBox2.Controls.Add(this.G2);
		this.GroupBox2.Controls.Add(this.GroupBox3);
		this.GroupBox2.Controls.Add(this.Tover);
		this.GroupBox2.Controls.Add(this.G1);
		this.GroupBox2.Controls.Add(this.Rsex);
		this.GroupBox2.Controls.Add(this.Rperfix);
		this.GroupBox2.Controls.Add(this.Rtype_Main);
		this.GroupBox2.Controls.Add(this.Label31);
		this.GroupBox2.Controls.Add(this.Rtype);
		this.GroupBox2.Controls.Add(this.Rno);
		this.GroupBox2.Controls.Add(this.Rname2);
		this.GroupBox2.Controls.Add(this.Rname);
		this.GroupBox2.Controls.Add(this.Ridcard);
		this.GroupBox2.Controls.Add(this.Remail);
		this.GroupBox2.Controls.Add(this.Label6);
		this.GroupBox2.Controls.Add(this.Label29);
		this.GroupBox2.Controls.Add(this.Label1);
		this.GroupBox2.Controls.Add(this.Label30);
		this.GroupBox2.Controls.Add(this.Label7);
		this.GroupBox2.Controls.Add(this.Label4);
		this.GroupBox2.Controls.Add(this.Label28);
		this.GroupBox2.Controls.Add(this.Label8);
		this.GroupBox2.Controls.Add(this.Label5);
		this.GroupBox2.Controls.Add(this.ButtonX_0);
		this.GroupBox2.Controls.Add(this.ButtonX_1);
		this.GroupBox2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox2;
		System.Drawing.Point location = new System.Drawing.Point(4, 38);
		groupBox.Location = location;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox2;
		size = new System.Drawing.Size(950, 573);
		groupBox2.Size = size;
		this.GroupBox2.TabIndex = 0;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไข ล\u0e39กค\u0e49า";
		this.Label35.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label = this.Label35;
		location = new System.Drawing.Point(380, 535);
		label.Location = location;
		this.Label35.Name = "Label35";
		System.Windows.Forms.Label label2 = this.Label35;
		size = new System.Drawing.Size(309, 37);
		label2.Size = size;
		this.Label35.TabIndex = 19;
		this.Label35.Text = "*ถ\u0e49าไม\u0e48กรอกท\u0e35\u0e48อย\u0e39\u0e48ตามช\u0e48อง ให\u0e49กรอกลงไปในช\u0e48อง เลขท\u0e35\u0e48ช\u0e48องเด\u0e35ยวได\u0e49เลย";
		this.GroupBox1.Controls.Add(this.ListView1);
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox1;
		location = new System.Drawing.Point(329, 22);
		groupBox3.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox1;
		size = new System.Drawing.Size(239, 258);
		groupBox4.Size = size;
		this.GroupBox1.TabIndex = 17;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายการราคา";
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[2] { this.ColumnHeader1, this.ColumnHeader2 });
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(10, 19);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(218, 230);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ประเภทห\u0e49อง";
		this.ColumnHeader1.Width = 140;
		this.ColumnHeader2.Text = "ราคา";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Label32.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label32;
		location = new System.Drawing.Point(213, 539);
		label3.Location = location;
		this.Label32.Name = "Label32";
		System.Windows.Forms.Label label4 = this.Label32;
		size = new System.Drawing.Size(31, 16);
		label4.Size = size;
		this.Label32.TabIndex = 15;
		this.Label32.Text = "บาท";
		this.G2.Controls.Add(this.Label34);
		this.G2.Controls.Add(this.G2_TAX);
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
		location = new System.Drawing.Point(482, 286);
		g.Location = location;
		this.G2.Name = "G2";
		System.Windows.Forms.GroupBox g2 = this.G2;
		size = new System.Drawing.Size(452, 238);
		g2.Size = size;
		this.G2.TabIndex = 12;
		this.G2.TabStop = false;
		this.G2.Text = "สถานท\u0e35\u0e48ทำงาน";
		this.Label34.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label34;
		location = new System.Drawing.Point(17, 210);
		label5.Location = location;
		this.Label34.Name = "Label34";
		System.Windows.Forms.Label label6 = this.Label34;
		size = new System.Drawing.Size(75, 16);
		label6.Size = size;
		this.Label34.TabIndex = 15;
		this.Label34.Text = "เลขประจำต\u0e31ว";
		this.G2_TAX.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_TAX = this.G2_TAX;
		location = new System.Drawing.Point(94, 206);
		g2_TAX.Location = location;
		this.G2_TAX.MaxLength = 255;
		this.G2_TAX.Name = "G2_TAX";
		DevComponents.DotNetBar.Controls.TextBoxX g2_TAX2 = this.G2_TAX;
		size = new System.Drawing.Size(105, 23);
		g2_TAX2.Size = size;
		this.G2_TAX.TabIndex = 14;
		this.G2_Work.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Work = this.G2_Work;
		location = new System.Drawing.Point(94, 16);
		g2_Work.Location = location;
		this.G2_Work.MaxLength = 255;
		this.G2_Work.Name = "G2_Work";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Work2 = this.G2_Work;
		size = new System.Drawing.Size(297, 23);
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
		DevComponents.DotNetBar.Controls.TextBoxX g2_No = this.G2_No;
		location = new System.Drawing.Point(94, 44);
		g2_No.Location = location;
		this.G2_No.MaxLength = 249;
		this.G2_No.Multiline = true;
		this.G2_No.Name = "G2_No";
		DevComponents.DotNetBar.Controls.TextBoxX g2_No2 = this.G2_No;
		size = new System.Drawing.Size(297, 49);
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
		location = new System.Drawing.Point(68, 101);
		label11.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label12 = this.Label18;
		size = new System.Drawing.Size(24, 16);
		label12.Size = size;
		this.Label18.TabIndex = 11;
		this.Label18.Text = "หม\u0e39\u0e48";
		this.G2_Fax.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Fax = this.G2_Fax;
		location = new System.Drawing.Point(281, 205);
		g2_Fax.Location = location;
		this.G2_Fax.MaxLength = 255;
		this.G2_Fax.Name = "G2_Fax";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Fax2 = this.G2_Fax;
		size = new System.Drawing.Size(110, 23);
		g2_Fax2.Size = size;
		this.G2_Fax.TabIndex = 9;
		this.Label19.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label19;
		location = new System.Drawing.Point(250, 181);
		label13.Location = location;
		this.Label19.Name = "Label19";
		System.Windows.Forms.Label label14 = this.Label19;
		size = new System.Drawing.Size(29, 16);
		label14.Size = size;
		this.Label19.TabIndex = 11;
		this.Label19.Text = "โทร";
		this.G2_Tel.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tel = this.G2_Tel;
		location = new System.Drawing.Point(281, 178);
		g2_Tel.Location = location;
		this.G2_Tel.MaxLength = 255;
		this.G2_Tel.Name = "G2_Tel";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tel2 = this.G2_Tel;
		size = new System.Drawing.Size(110, 23);
		g2_Tel2.Size = size;
		this.G2_Tel.TabIndex = 8;
		this.Label20.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label20;
		location = new System.Drawing.Point(246, 101);
		label15.Location = location;
		this.Label20.Name = "Label20";
		System.Windows.Forms.Label label16 = this.Label20;
		size = new System.Drawing.Size(33, 16);
		label16.Size = size;
		this.Label20.TabIndex = 11;
		this.Label20.Text = "ซอย";
		this.Label21.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label21;
		location = new System.Drawing.Point(55, 128);
		label17.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label18 = this.Label21;
		size = new System.Drawing.Size(32, 16);
		label18.Size = size;
		this.Label21.TabIndex = 11;
		this.Label21.Text = "ถนน";
		this.Label22.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label22;
		location = new System.Drawing.Point(251, 208);
		label19.Location = location;
		this.Label22.Name = "Label22";
		System.Windows.Forms.Label label20 = this.Label22;
		size = new System.Drawing.Size(28, 16);
		label20.Size = size;
		this.Label22.TabIndex = 11;
		this.Label22.Text = "Fax";
		this.Label23.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label23;
		location = new System.Drawing.Point(208, 128);
		label21.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label22 = this.Label23;
		size = new System.Drawing.Size(71, 16);
		label22.Size = size;
		this.Label23.TabIndex = 11;
		this.Label23.Text = "แขวง/ตำบล";
		this.G2_Code.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Code = this.G2_Code;
		location = new System.Drawing.Point(94, 179);
		g2_Code.Location = location;
		this.G2_Code.MaxLength = 255;
		this.G2_Code.Name = "G2_Code";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Code2 = this.G2_Code;
		size = new System.Drawing.Size(105, 23);
		g2_Code2.Size = size;
		this.G2_Code.TabIndex = 7;
		this.G2_Moo.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Moo = this.G2_Moo;
		location = new System.Drawing.Point(94, 98);
		g2_Moo.Location = location;
		this.G2_Moo.MaxLength = 255;
		this.G2_Moo.Name = "G2_Moo";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Moo2 = this.G2_Moo;
		size = new System.Drawing.Size(105, 23);
		g2_Moo2.Size = size;
		this.G2_Moo.TabIndex = 2;
		this.G2_ampore.AutoCompleteCustomSource.AddRange(new string[926]
		{
			"เม\u0e37องกระบ\u0e35\u0e48", "เกาะล\u0e31นตา", "เขาพนม", "คลองท\u0e48อม", "ปลายพระยา", "ลำท\u0e31บ", "เหน\u0e37อคลอง", "อ\u0e48าวล\u0e36ก", "คลองสาน", "คลองเตย",
			"จอมทอง", "จต\u0e38จ\u0e31กร", "ด\u0e38ส\u0e34ต", "ดอนเม\u0e37อง", "ตล\u0e34\u0e48งช\u0e31น", "ธนบ\u0e38ร\u0e35", "บางกอกน\u0e49อย", "บางกอกใหญ\u0e48", "บางกะป\u0e34", "บางข\u0e38นเท\u0e35ยน",
			"บางเขน", "บางคอแหลม", "บางซ\u0e37\u0e48อ", "บางพล\u0e31ด", "บางร\u0e31ก", "บ\u0e36งก\u0e38\u0e48ม", "ประเวศ", "ปท\u0e38มว\u0e31น", "ป\u0e49อมปราบศ\u0e31ตร\u0e39พ\u0e48าย", "พญาไท",
			"พระโขนง", "พระนคร", "ภาษ\u0e35เจร\u0e34ญ", "ม\u0e35นบ\u0e38ร\u0e35", "ยานนาวา", "ราชเทว\u0e35", "ราษฎร\u0e4cบ\u0e39รณะ", "ลาดกระบ\u0e31ง", "ลาดพร\u0e49าว", "สาทร",
			"ส\u0e31มพ\u0e31นธวงศ\u0e4c", "หนองแขม", "หนองจอก", "ห\u0e49วยขวาง", "สวนหลวง", "ด\u0e34นแดง", "หล\u0e31กส\u0e35\u0e48", "สายไหม", "ค\u0e31นนายาว", "สะพานส\u0e39ง",
			"ว\u0e31งทองหลาง", "คลองสามวา", "ว\u0e31ฒนา", "บางนา", "ทว\u0e35ว\u0e31ฒนา", "บางแค", "ท\u0e38\u0e48งคร\u0e38", "บางบอน", "เม\u0e37องกาญจนบ\u0e38ร\u0e35", "ด\u0e48านมะขามเต\u0e35\u0e49ย",
			"ทองผาภ\u0e39ม\u0e34", "ท\u0e48าม\u0e48วง", "ท\u0e48ามะกา", "ไทรโยค", "บ\u0e48อพลอย", "พนมทวน", "เลาขว\u0e31ญ", "ศร\u0e35สว\u0e31สด\u0e34\u0e4c", "ส\u0e31งขละบ\u0e38ร\u0e35", "หนองปร\u0e37อ",
			"ห\u0e49วยกระเจา", "เม\u0e37องกาฬส\u0e34นธ\u0e38\u0e4c", "กมลาไสย", "ก\u0e38ฉ\u0e34นารายณ\u0e4c", "เขาวง", "คำม\u0e48วง", "ท\u0e48าค\u0e31นโท", "นามน", "ยางตลาด", "ร\u0e48องคำ",
			"สมเด\u0e47จ", "สห\u0e31สข\u0e31นธ\u0e4c", "หนองก\u0e38งศร\u0e35", "ห\u0e49วยผ\u0e36\u0e49ง", "ห\u0e49วยเม\u0e47ก", "นาค\u0e39", "สามช\u0e31ย", "ดอนจาน", "ฆ\u0e49องช\u0e31ย", "เม\u0e37องกำแพงเพชร",
			"ขาณ\u0e38วรล\u0e31กษบ\u0e38ร\u0e35", "คลองขล\u0e38ง", "คลองลาน", "ทรายทองว\u0e31ฒนา", "ไทรงาม", "ปางศ\u0e34ลาทอง", "พรานกระต\u0e48าย", "ลานกระบ\u0e37อ", "บ\u0e36งสาม\u0e31คค\u0e35", "โกส\u0e31มพ\u0e35นคร",
			"เม\u0e37องขอนแก\u0e48น", "กระนวน", "เขาสวนกวาง", "โคกโพธ\u0e34\u0e4cไชย", "ชำส\u0e39ง", "ชนบท", "ช\u0e38มแพ", "น\u0e49ำพอง", "บ\u0e49านไผ\u0e48", "บ\u0e49านฝาง",
			"เป\u0e37อยน\u0e49อย", "พล", "พระย\u0e37น", "ภ\u0e39เว\u0e35ยง", "ภ\u0e39ผาม\u0e48าน", "ม\u0e31ญจาค\u0e35ร\u0e35", "แวงน\u0e49อย", "แวงใหญ\u0e48", "ส\u0e35ชมพ\u0e39", "หนองสองห\u0e49อง",
			"หนองเร\u0e37อ", "หนองนาคำ", "อ\u0e38บลร\u0e31ตน\u0e4c", "โนนศ\u0e34ลา", "บ\u0e49านแฮด", "เม\u0e37องจ\u0e31นทบ\u0e38ร\u0e35", "แก\u0e48งหางแมว", "ขล\u0e38ง", "ท\u0e48าใหม\u0e48", "นายายอาม",
			"โป\u0e48งน\u0e49ำร\u0e49อน", "มะขาม", "สอยดาว", "แหลมส\u0e34งห\u0e4c", "เขาค\u0e34ชฌก\u0e39ฏ", "เม\u0e37องฉะเช\u0e34งเทรา", "บางคล\u0e49า", "บางน\u0e49ำเปร\u0e35\u0e49ยว", "บางปะกง", "บ\u0e49านโพธ\u0e34\u0e4c",
			"แปลงยาว", "พนมสารคาม", "ราชสาส\u0e4cน", "สนามช\u0e31ยเขต", "ท\u0e48าตะเก\u0e35ยบ", "คลองเข\u0e37\u0e48อน", "เม\u0e37องชลบ\u0e38ร\u0e35", "เกาะส\u0e35ช\u0e31ง", "บ\u0e48อทอง", "บางละม\u0e38ง",
			"บ\u0e49านบ\u0e36ง", "พานทอง", "พน\u0e31สน\u0e34คม", "ศร\u0e35ราชา", "ส\u0e31ตห\u0e35บ", "หนองใหญ\u0e48", "เกาะจ\u0e31นทร\u0e4c", "เม\u0e37องช\u0e31ยนาท", "มโนรมย\u0e4c", "ว\u0e31ดส\u0e34งห\u0e4c",
			"สรรคบ\u0e38ร\u0e35", "สรรพยา", "ห\u0e31นคา", "หนองมะโมง", "เน\u0e34นขาม", "เม\u0e37องช\u0e31ยภ\u0e39ม\u0e34", "เกษตรสมบ\u0e39รณ\u0e4c", "แก\u0e49งคร\u0e49อ", "คอนสวรรค\u0e4c", "คอนสาร",
			"จ\u0e31ต\u0e38ร\u0e31ส", "เทพสถ\u0e34ต", "เน\u0e34นสง\u0e48า", "บ\u0e49านเขว\u0e49า", "บ\u0e49านแท\u0e48น", "บำเหน\u0e47จณรงค\u0e4c", "ภ\u0e39เข\u0e35ยว", "ภ\u0e31กด\u0e35ช\u0e38มพล", "หนองบ\u0e31วแดง", "หนองบ\u0e31วระเหว",
			"ซ\u0e31บใหญ\u0e48", "เม\u0e37องช\u0e38มพร", "ท\u0e48าแซะ", "ท\u0e38\u0e48งตะโก", "ปะท\u0e34ว", "พะโต\u0e4aะ", "ละแม", "สว\u0e35", "หล\u0e31งสวน", "เม\u0e37องเช\u0e35ยงราย",
			"ข\u0e38นตาล", "เช\u0e35ยงของ", "เช\u0e35ยงแสน", "เท\u0e34ง", "ป\u0e48าแดด", "พาน", "แม\u0e48จ\u0e31น", "แม\u0e48ฟ\u0e49าหลวง", "แม\u0e48สรวย", "แม\u0e48สาย",
			"เว\u0e35ยงแก\u0e48น", "เว\u0e35ยงช\u0e31ย", "เว\u0e35ยงป\u0e48าเป\u0e49า", "พญาเม\u0e47งราย", "แม\u0e48ลาว", "ดอยหลวง", "เว\u0e35ยงเช\u0e35ยงร\u0e38\u0e49ง", "เม\u0e37องเช\u0e35ยงใหม\u0e48", "จอมทอง", "เช\u0e35ยงดาว",
			"ไชยปราการ", "ดอยเต\u0e48า", "ดอยหล\u0e48อ", "ดอยสะเก\u0e47ด", "ฝาง", "พร\u0e49าว", "แม\u0e48แจ\u0e48ม", "แม\u0e48แตง", "แม\u0e48ร\u0e34ม", "แม\u0e48วาง",
			"แม\u0e48อาย", "แม\u0e48ออน", "เว\u0e35ยงแหง", "สะเม\u0e34ง", "ส\u0e31นกำแพง", "ส\u0e31นทราย", "ส\u0e31นป\u0e48าตอง", "สารภ\u0e35", "หางดง", "อมก\u0e4bอย",
			"ฮอด", "เม\u0e37องตร\u0e31ง", "ก\u0e31นต\u0e31ง", "ปะเหล\u0e35ยน", "ย\u0e48านตาขาว", "ร\u0e31ษฎา", "ส\u0e34เกา", "ห\u0e49วยยอด", "ว\u0e31งว\u0e34เศษ", "หาดสำราญ",
			"นาโยง", "เม\u0e37องตราด", "เกาะช\u0e49าง", "เขาสม\u0e34ง", "คลองใหญ\u0e48", "บ\u0e48อไร\u0e48", "แหลมงอบ", "เกาะก\u0e39ด", "เม\u0e37องตาก", "ท\u0e48าสองยาง",
			"บ\u0e49านตาก", "พบพระ", "แม\u0e48ระมาด", "แม\u0e48สอด", "สามเงา", "อ\u0e38\u0e49มผาง", "ว\u0e31งเจ\u0e49า", "เม\u0e37องนครนายก", "บ\u0e49านนา", "ปากพล\u0e35",
			"องคร\u0e31กษ\u0e4c", "เม\u0e37องนครปฐม", "กำแพงแสน", "ดอนต\u0e39ม", "นครช\u0e31ยศร\u0e35", "บางเลน", "พ\u0e38ทธมณฑล", "สามพราน", "เม\u0e37องนครพนม", "ท\u0e48าอ\u0e38เทน",
			"ธาต\u0e38พนม", "นาแก", "นาหว\u0e49า", "บ\u0e49านแพง", "ปลาปาก", "โพนสวรรค\u0e4c", "เรณ\u0e39นคร", "ศร\u0e35สงคราม", "ว\u0e31งยาง", "นาทม",
			"เม\u0e37องนครราชส\u0e35มา", "แก\u0e49งสนามนาง", "ขามทะเลสอ", "ขามสะแกแสง", "คง", "ครบ\u0e38ร\u0e35", "จ\u0e31กราช", "ช\u0e38มพวง", "โชคช\u0e31ย", "ด\u0e48านข\u0e38นทด",
			"โนนแดง", "โนนไทย", "โนนส\u0e39ง", "บ\u0e31วใหญ\u0e48", "บ\u0e49านเหล\u0e37\u0e48อม", "ประทาย", "ป\u0e31กธงช\u0e31ย", "ปากช\u0e48อง", "พ\u0e34มาย", "ว\u0e31งน\u0e49ำเข\u0e35ยว",
			"ส\u0e35ค\u0e34\u0e49ว", "ส\u0e39งเน\u0e34น", "เส\u0e34งสาง", "ห\u0e49วยแถลง", "หนองบ\u0e38นนาก", "เทพาร\u0e31กษ\u0e4c", "เม\u0e37องยาง", "พระทองคำ", "ลำทะเมนช\u0e31ย", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34",
			"ส\u0e35ดา", "บ\u0e31วลาย", "เม\u0e37องนครศร\u0e35ธรรมราช", "ขนอม", "ฉวาง", "ชะอวด", "เช\u0e35ยรใหญ\u0e48", "ท\u0e48าศาลา", "ท\u0e38\u0e48งใหญ\u0e48", "ท\u0e38\u0e48งสง",
			"พระพรหม", "นาบอน", "บางข\u0e31น", "ปากพน\u0e31ง", "พรหมค\u0e35ร\u0e35", "พ\u0e34ป\u0e39น", "ร\u0e48อนพ\u0e34บ\u0e39ลย\u0e4c", "ลานสะกา", "ส\u0e34ชล", "ห\u0e31วไทร",
			"จ\u0e38ฬาภรณ\u0e4c", "นบพ\u0e34ตำ", "ช\u0e49างกลาง", "ถ\u0e49ำพรรณรา", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "เม\u0e37องนครสวรรค\u0e4c", "เก\u0e49าเล\u0e35\u0e49ยว", "โกรกพระ", "ช\u0e38มแสง", "ตากฟ\u0e49า",
			"ตาคล\u0e35", "ท\u0e48าตะโก", "บรรพตพ\u0e34ส\u0e31ย", "พย\u0e38หค\u0e35ร\u0e35", "ไพศาล\u0e35", "แม\u0e48วงก\u0e4c", "ลาดยาว", "หนองบ\u0e31ว", "แม\u0e48เป\u0e34น", "ช\u0e38มตาบง",
			"เม\u0e37องนนทบ\u0e38ร\u0e35", "ไทรน\u0e49อย", "บางกรวย", "บางบ\u0e31วทอง", "บางใหญ\u0e48", "ปากเกร\u0e47ด", "เม\u0e37องนราธ\u0e34วาส", "จะแนะ", "ตากใบ", "บาเจาะ",
			"ย\u0e35\u0e48งอ", "ระแงะ", "ร\u0e37อเสาะ", "แว\u0e49ง", "ศร\u0e35สาคร", "ส\u0e38ค\u0e34ร\u0e34น", "ส\u0e38ไหงโกลก", "ส\u0e38ไหงปาด\u0e35", "เจาะไอร\u0e49อง", "เม\u0e37องน\u0e48าน",
			"เช\u0e35ยงกลาง", "ท\u0e48าว\u0e31งผา", "ท\u0e38\u0e48งช\u0e49าง", "นาน\u0e49อย", "นาหม\u0e37\u0e48น", "บ\u0e49านหลวง", "ป\u0e31ว", "แม\u0e48จร\u0e34ม", "เว\u0e35ยงสา", "ส\u0e31นต\u0e34ส\u0e38ข",
			"บ\u0e48อเกล\u0e37อ", "สองแคว", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "ภ\u0e39เพ\u0e35ยง", "เม\u0e37องบ\u0e38ร\u0e35ร\u0e31มย\u0e4c", "กระส\u0e31ง", "ค\u0e39เม\u0e37อง", "ชำน\u0e34", "นาโพธ\u0e34\u0e4c", "นางรอง",
			"โนนด\u0e34นแดง", "โนนส\u0e38วรรณ", "บ\u0e49านกรวด", "พล\u0e31บพลาช\u0e31ย", "บ\u0e49านใหม\u0e48ไชยพจน\u0e4c", "ประโคนช\u0e31ย", "ปะคำ", "พ\u0e38ทไธสง", "ละหานทราย", "ลำปลายมาศ",
			"สต\u0e36ก", "หนองก\u0e35\u0e48", "หนองหงส\u0e4c", "ห\u0e49วยราช", "บ\u0e49านด\u0e48าน", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "แคนดง", "เม\u0e37องปท\u0e38มธาน\u0e35", "คลองหลวง", "ธ\u0e31ญบ\u0e38ร\u0e35",
			"ลาดหล\u0e38มแก\u0e49ว", "ลำล\u0e39กกา", "สามโคก", "หนองเส\u0e37อ", "เม\u0e37องประจวบค\u0e35ร\u0e35ข\u0e31นธ\u0e4c", "ก\u0e38ยบ\u0e38ร\u0e35", "ท\u0e31บสะแก", "บางสะพาน", "บางสะพานน\u0e49อย", "ปราณบ\u0e38ร\u0e35",
			"ห\u0e31วห\u0e34น", "สามร\u0e49อยยอด", "เม\u0e37องปราจ\u0e35นบ\u0e38ร\u0e35", "กบ\u0e34นทร\u0e4cบ\u0e38ร\u0e35", "ศร\u0e35มโหสถ", "นาด\u0e35", "บ\u0e49านสร\u0e49าง", "ประจ\u0e31นตคาม", "ศร\u0e35มหาโพธ\u0e34", "เม\u0e37องป\u0e31ตตาน\u0e35",
			"กะพ\u0e49อ", "โคกโพธ\u0e34\u0e4c", "ท\u0e38\u0e48งยางแดง", "ปะนาเระ", "มายอ", "ไม\u0e49แก\u0e48น", "ยะร\u0e31ง", "ยะหร\u0e34\u0e48ง", "สายบ\u0e38ร\u0e35", "หนองจ\u0e34ก",
			"แม\u0e48ลาน", "พระนครศร\u0e35อย\u0e38ธยา", "ท\u0e48าเร\u0e37อ", "นครหลวง", "บางซ\u0e49าย", "บางไทร", "บางบาล", "บางปะห\u0e31น", "บางปะอ\u0e34น", "บ\u0e49านแพรก",
			"ผ\u0e31กไห\u0e48", "ภาช\u0e35", "มหาราช", "ลาดบ\u0e31วหลวง", "ว\u0e31งน\u0e49อย", "เสนา", "อ\u0e38ท\u0e31ย", "เม\u0e37องพะเยา", "จ\u0e38น", "เช\u0e35ยงคำ",
			"เช\u0e35ยงม\u0e48วน", "ดอกคำใต\u0e49", "ปง", "แม\u0e48ใจ", "ภ\u0e39ซาง", "ภ\u0e39กามยาว", "เม\u0e37องพ\u0e31งงา", "กะปง", "เกาะยาว", "ค\u0e38ระบ\u0e38ร\u0e35",
			"ตะก\u0e31\u0e48วท\u0e38\u0e48ง", "ตะก\u0e31\u0e48วป\u0e48า", "ท\u0e31บป\u0e38ด", "ท\u0e49ายเหม\u0e37อง", "เม\u0e37องพ\u0e31ทล\u0e38ง", "กงหรา", "เขาช\u0e31ยสน", "ควนขน\u0e38น", "ตะโหมด", "ปากพะย\u0e39น",
			"ป\u0e48าบอน", "ป\u0e48าพะยอม", "ศร\u0e35บรรพต", "บางแก\u0e49ว", "ศร\u0e35นคร\u0e34นทร\u0e4c", "เม\u0e37องพ\u0e34จ\u0e34ตร", "ตะพานห\u0e34น", "ท\u0e31บคล\u0e49อ", "บางม\u0e39ลนาก", "โพทะเล",
			"โพธ\u0e34\u0e4cประท\u0e31บช\u0e49าง", "สามง\u0e48าม", "ว\u0e31งทรายพ\u0e39น", "สากเหล\u0e47ก", "บ\u0e36งนาราง", "ดงเจร\u0e34ญ", "วช\u0e34รบารม\u0e35", "เม\u0e37องพ\u0e34ษณ\u0e38โลก", "นครไทย", "ชาต\u0e34ตระการ",
			"เน\u0e34นมะปราง", "บางกระท\u0e38\u0e48ม", "บางระกำ", "พรหมพ\u0e34ราม", "ว\u0e31งทอง", "ว\u0e31ดโบสถ\u0e4c", "เม\u0e37องเพชรบ\u0e38ร\u0e35", "แก\u0e48งกระจาน", "เขาย\u0e49อย", "ชะอำ",
			"ท\u0e48ายาง", "บ\u0e49านลาด", "บ\u0e49านแหลม", "หนองหญ\u0e49าปล\u0e49อง", "เม\u0e37องเพชรบ\u0e39รณ\u0e4c", "เขาค\u0e49อ", "ชนแดน", "น\u0e49ำหนาว", "บ\u0e36งสามพ\u0e31น", "ว\u0e34เช\u0e35ยรบ\u0e38ร\u0e35",
			"ศร\u0e35เทพ", "หนองไผ\u0e48", "หล\u0e48มเก\u0e48า", "หล\u0e48มส\u0e31ก", "ว\u0e31งโป\u0e48ง", "เม\u0e37องแพร\u0e48", "เด\u0e48นช\u0e31ย", "ร\u0e49องกวาง", "ลอง", "ว\u0e31งช\u0e34\u0e49น",
			"สอง", "หนองม\u0e48วงไข\u0e48", "ส\u0e39งเม\u0e48น", "เม\u0e37องภ\u0e39เก\u0e47ต", "กะท\u0e39\u0e49", "ถลาง", "เม\u0e37องมหาสารคาม", "ก\u0e31นทรว\u0e34ช\u0e31ย", "แกดำ", "โกส\u0e38มพ\u0e34ส\u0e31ย",
			"เช\u0e35ยงย\u0e37น", "นาเช\u0e37อก", "นาด\u0e39น", "บรบ\u0e37อ", "พย\u0e31คฆภ\u0e39ม\u0e34พ\u0e34ส\u0e31ย", "วาป\u0e35ปท\u0e38ม", "ก\u0e38ดร\u0e31ง", "ยางส\u0e35ส\u0e38ราช", "ช\u0e37\u0e48นชม", "เม\u0e37องม\u0e38กดาหาร",
			"คำชะอ\u0e35", "ดงหลวง", "ดอนตาล", "น\u0e34คมคำสร\u0e49อย", "หนองส\u0e39ง", "หว\u0e49านใหญ\u0e48", "เม\u0e37องแม\u0e48ฮ\u0e48องสอน", "ข\u0e38นยวม", "ปางมะผ\u0e49า", "ปาย",
			"แม\u0e48ลาน\u0e49อย", "แม\u0e48สะเร\u0e35ยง", "สบเมย", "เม\u0e37องยโสธร", "ก\u0e38ดช\u0e38ม", "ค\u0e49อว\u0e31ง", "คำเข\u0e37\u0e48อนแก\u0e49ว", "ไทยเจร\u0e34ญ", "ทรายม\u0e39ล", "ป\u0e48าต\u0e34\u0e49ว",
			"มหาชนะช\u0e31ย", "เล\u0e34งนกทา", "เม\u0e37องยะลา", "กาบ\u0e31ง", "กรงป\u0e34น\u0e31ง", "ธารโต", "บ\u0e31นน\u0e31งสตา", "เบตง", "ยะหา", "ราม\u0e31น",
			"เม\u0e37องร\u0e49อยเอ\u0e47ด", "เกษตรว\u0e34ส\u0e31ย", "จต\u0e38รพ\u0e31กตร\u0e4cพ\u0e34มาน", "จ\u0e31งหาร", "ธว\u0e31ชบ\u0e38ร\u0e35", "ปท\u0e38มร\u0e31ตน\u0e4c", "พนมไพร", "โพธ\u0e34\u0e4cช\u0e31ย", "โพนทราย", "โพนทอง",
			"เมยวด\u0e35", "เม\u0e37องสรวง", "ศร\u0e35สมเด\u0e47จ", "เสลภ\u0e39ม\u0e34", "ส\u0e38วรรณภ\u0e39ม\u0e34", "หนองพอก", "อาจสามารถ", "เช\u0e35ยงขว\u0e31ญ", "หนองฮ\u0e35", "ท\u0e38\u0e48งเขาหลวง",
			"เม\u0e37องระนอง", "กระบ\u0e38ร\u0e35", "กะเปอร\u0e4c", "ละอ\u0e38\u0e48น", "ส\u0e38ขสำราญ", "เม\u0e37องระยอง", "แกลง", "บ\u0e49านค\u0e48าย", "บ\u0e49านฉาง", "ปลวกแดง",
			"ว\u0e31งจ\u0e31นทร\u0e4c", "เขาชะเมา", "น\u0e34คมพ\u0e31ฒนา", "เม\u0e37องราชบ\u0e38ร\u0e35", "จอมบ\u0e36ง", "ดำเน\u0e34นสะดวก", "บางแพ", "บ\u0e49านโป\u0e48ง", "ปากท\u0e48อ", "โพธาราม",
			"ว\u0e31ดเพลง", "สวนผ\u0e36\u0e49ง", "บ\u0e49านคา", "เม\u0e37องลพบ\u0e38ร\u0e35", "โคกเจร\u0e34ญ", "โคกสำโรง", "ช\u0e31ยบาดาล", "ท\u0e48าว\u0e38\u0e49ง", "ท\u0e48าหลวง", "บ\u0e49านหม\u0e35\u0e48",
			"พ\u0e31ฒนาน\u0e34คม", "ลำสนธ\u0e34", "สระโบถส\u0e4c", "หนองม\u0e48วง", "เม\u0e37องเลย", "เช\u0e35ยงคาน", "ด\u0e48านซ\u0e49าย", "ท\u0e48าล\u0e35\u0e48", "นาด\u0e49วง", "นาแห\u0e49ว",
			"ปากชม", "ผาขาว", "ภ\u0e39กระด\u0e36ง", "ภ\u0e39เร\u0e37อ", "ภ\u0e39หลวง", "ว\u0e31งสะพ\u0e38ง", "เอราว\u0e31ณ", "หนองห\u0e34น", "เม\u0e37องลำปาง", "เกาะคา",
			"งาว", "แจ\u0e49ห\u0e48ม", "เถ\u0e34น", "แม\u0e48ทะ", "แม\u0e48พร\u0e34ก", "เม\u0e37องปาน", "แม\u0e48เมาะ", "ว\u0e31งเหน\u0e37อ", "สบปราบ", "เสร\u0e34มงาม",
			"ห\u0e49างฉ\u0e31ตร", "เม\u0e37องลำพ\u0e39น", "ท\u0e38\u0e48งห\u0e31วช\u0e49าง", "บ\u0e49านโฮ\u0e48ง", "ป\u0e48าซาง", "แม\u0e48ทา", "ล\u0e35\u0e49", "บ\u0e49านธ\u0e34", "เว\u0e35ยงหนองล\u0e48อง", "เม\u0e37องศ\u0e35รสะเกษ",
			"ก\u0e31นทรล\u0e31กษ\u0e4c", "ก\u0e31นทรารมย\u0e4c", "ข\u0e38ข\u0e31นธ\u0e4c", "ข\u0e38นหาญ", "น\u0e49ำเกล\u0e35\u0e49ยง", "โนนค\u0e39ณ", "บ\u0e36งบ\u0e39รพ\u0e4c", "เบญจล\u0e31กษณ\u0e4c", "ปรางค\u0e4cก\u0e39\u0e48", "พย\u0e38ห\u0e4c",
			"ไพรบ\u0e36ง", "โพธ\u0e34\u0e4cศร\u0e35ส\u0e38วรรณ", "ภ\u0e39ส\u0e34งห\u0e4c", "เม\u0e37องจ\u0e31นทร\u0e4c", "ยางช\u0e38มน\u0e49อย", "ราษ\u0e35ไศล", "ว\u0e31งห\u0e34น", "ศร\u0e35ร\u0e31ตนะ", "ห\u0e49วยท\u0e31บท\u0e31น", "อ\u0e38ท\u0e38มพรพ\u0e34ส\u0e31ย",
			"ศ\u0e34ลาลาด", "เม\u0e37องสกลนคร", "ก\u0e38ดบาก", "ก\u0e38ส\u0e38มาลย\u0e4c", "คำตากล\u0e49า", "เจร\u0e34ญศ\u0e34ลป\u0e4c", "เต\u0e48างอย", "น\u0e34คมน\u0e49ำอ\u0e39น", "บ\u0e49านม\u0e48วง", "พรรณาน\u0e34คม",
			"พ\u0e31งโคน", "วานรน\u0e34วาส", "วาร\u0e34ชภ\u0e39ม\u0e34", "โคกศร\u0e35ส\u0e38พรรณ", "สว\u0e48างแดนด\u0e34น", "ส\u0e48องดาว", "อากาศอำนวย", "ภ\u0e39พาน", "โพนนาแก\u0e49ว", "เม\u0e37องสงขลา",
			"กระแสส\u0e34นธ\u0e38\u0e4c", "ควนเน\u0e35ยง", "จะนะ", "เทพา", "นาทว\u0e35", "นาหม\u0e48อม", "บางกล\u0e48ำ", "ระโนด", "ร\u0e31ตภ\u0e39ม\u0e34", "สท\u0e34งพระ",
			"สะเดา", "สะบ\u0e49าย\u0e49อย", "ส\u0e34งหนคร", "หาดใหญ\u0e48", "คลองหอยโข\u0e48ง", "เม\u0e37องสต\u0e39ล", "ควนกาหลง", "ควนโดน", "ท\u0e48าแพ", "ท\u0e38\u0e48งหว\u0e49า",
			"ละง\u0e39", "มะน\u0e31ง", "เม\u0e37องสม\u0e38ทรปราการ", "บางบ\u0e48อ", "บางพล\u0e35", "พระประแดง", "พระสม\u0e38ทรเจด\u0e35ย\u0e4c", "บางเสาธง", "เม\u0e37องสม\u0e38ทรสงคราม", "บางคณท\u0e35",
			"อ\u0e31มพวา", "เม\u0e37องสม\u0e38ทรสาคร", "กระท\u0e38\u0e48มแบน", "บ\u0e49านแพ\u0e49ว", "เม\u0e37องสระแก\u0e49ว", "เขาฉกรรจ\u0e4c", "คลองหาด", "ตาพระยา", "ว\u0e31งน\u0e49ำเย\u0e47น", "ว\u0e31ฒนานคร",
			"อร\u0e31ญประเทศ", "โคกส\u0e39ง", "ว\u0e31งสมบ\u0e39รณ\u0e4c", "เม\u0e37องสระบ\u0e38ร\u0e35", "แก\u0e48งคอย", "ดอนพ\u0e38ด", "บ\u0e49านหมอ", "พระพ\u0e38ทธบาท", "มวกเหล\u0e47ก", "ว\u0e34หารแดง",
			"เสาไห\u0e49", "หนองแค", "หนองแซง", "หนองโดน", "ว\u0e31งม\u0e48วง", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "เม\u0e37องส\u0e34งห\u0e4cบ\u0e38ร\u0e35", "ค\u0e48ายบางระจ\u0e31น", "ท\u0e48าช\u0e49าง", "บางระจ\u0e31น",
			"พรหมบ\u0e38ร\u0e35", "อ\u0e34นทร\u0e4cบ\u0e38ร\u0e35", "เม\u0e37องส\u0e38โขท\u0e31ย", "กงไกรลาศ", "ค\u0e35ร\u0e35มาศ", "ท\u0e38\u0e48งเสล\u0e35\u0e48ยม", "บ\u0e49านด\u0e48านลานหอย", "ศร\u0e35นคร", "ศร\u0e35ส\u0e31ชนาล\u0e31ย", "ศร\u0e35สำโรง",
			"สวรรคโลก", "เม\u0e37องส\u0e38พรรณบ\u0e38ร\u0e35", "ดอนเจด\u0e35ย\u0e4c", "ด\u0e48านช\u0e49าง", "เด\u0e34มบางนางบวช", "บางปลาม\u0e49า", "ศร\u0e35ประจ\u0e31นต\u0e4c", "สองพ\u0e35\u0e48น\u0e49อง", "สามช\u0e38ก", "อ\u0e39\u0e48ทอง",
			"หนองหญ\u0e49าไซ", "เม\u0e37องส\u0e38ราษฎร\u0e4cธาน\u0e35", "กาญจนด\u0e34ษฐ\u0e4c", "เกาะพะง\u0e31น", "เกาะสม\u0e38ย", "ค\u0e35ร\u0e35ร\u0e31ฐน\u0e34คม", "เค\u0e35ยนซา", "ช\u0e31ยบ\u0e38ร\u0e35", "ไชยา", "ดอนส\u0e31ก",
			"ท\u0e48าฉาง", "ท\u0e48าชนะ", "บ\u0e49านตาข\u0e38น", "บ\u0e49านนาเด\u0e34ม", "บ\u0e49านนาสาร", "พนม", "พระแสง", "พ\u0e38นพ\u0e34น", "ว\u0e34ภาวด\u0e35", "เว\u0e35ยงสระ",
			"เม\u0e37องส\u0e38ร\u0e34นทร\u0e4c", "กาบเช\u0e34ง", "จอมพระ", "ช\u0e38มพลบ\u0e38ร\u0e35", "ท\u0e48าต\u0e39ม", "บ\u0e31วเชด", "ปราสาท", "ร\u0e31ตนบ\u0e38ร\u0e35", "ลำดวน", "ศ\u0e35ขรภ\u0e39ม\u0e34",
			"สนม", "ส\u0e31งขะ", "สำโรงทาบ", "ศร\u0e35ณรงค\u0e4c", "พนมดงร\u0e31ก", "เขวาส\u0e34นร\u0e34นทร\u0e4c", "โนนนารายณ\u0e4c", "เม\u0e37องหนองคาย", "เซกา", "โซ\u0e48พ\u0e34ส\u0e31ย",
			"ท\u0e48าบ\u0e48อ", "บ\u0e36งกาฬ", "บ\u0e36งโขลงหลง", "ปากคาด", "พรเจร\u0e34ญ", "โพนพ\u0e34ส\u0e31ย", "ศร\u0e35เช\u0e35ยงใหม\u0e48", "ศร\u0e35ว\u0e34ไล", "ส\u0e31งคม", "สระใคร\u0e48",
			"บ\u0e38\u0e48งคล\u0e49า", "ร\u0e31ตนวาป\u0e35", "เฝ\u0e49าไร\u0e48", "โพธ\u0e34\u0e4cตาก", "เม\u0e37องหนองบ\u0e31วลำภ\u0e39", "นากลาง", "โนนส\u0e31ง", "ศร\u0e35บ\u0e38ญเร\u0e37อง", "ส\u0e38วรรณค\u0e39หา", "นาว\u0e31ง",
			"เม\u0e37องอ\u0e48างทอง", "ไชโย", "ป\u0e48าโมก", "โพธ\u0e34\u0e4cทอง", "ว\u0e34เศษช\u0e31ยชาญ", "สามโก\u0e49", "แสวงหา", "เม\u0e37องอำนาจเจร\u0e34ญ", "ชาน\u0e38มาน", "ปท\u0e38มราชวงศา",
			"พนา", "เสนางคน\u0e34คม", "ห\u0e31วตะพาน", "ล\u0e37ออำนาจ", "เม\u0e37องอ\u0e38ดรธาน\u0e35", "ก\u0e39\u0e48แก\u0e49ว", "ก\u0e38ดจ\u0e31บ", "ก\u0e38มภวาป\u0e35", "ไชยวาน", "ท\u0e38\u0e48งฝน",
			"นาย\u0e39ง", "น\u0e49ำโสม", "โนนสะอาด", "บ\u0e49านด\u0e38ง", "บ\u0e49านผ\u0e37อ", "พ\u0e34บ\u0e39ลย\u0e4cร\u0e31กษ\u0e4c", "เพ\u0e47ญ", "ว\u0e31งสามหมอ", "ศร\u0e35ธาต\u0e38", "สร\u0e49างคอม",
			"หนองว\u0e31วซอ", "หนองหาน", "หนองแสง", "ประจ\u0e31กษ\u0e4cศ\u0e34ลปาคาม", "เม\u0e37องอ\u0e38ตรด\u0e34ตถ\u0e4c", "ตรอน", "ทองแสนข\u0e31น", "ท\u0e48าปลา", "น\u0e49ำปาด", "บ\u0e49านโคก",
			"พ\u0e34ช\u0e31ย", "ฟากท\u0e48า", "ล\u0e31บแล", "เม\u0e37องอ\u0e38ท\u0e31ยธาน\u0e35", "ท\u0e31พท\u0e31น", "บ\u0e49านไร\u0e48", "ลานส\u0e31ก", "สว\u0e48างอารมณ\u0e4c", "หนองขาหย\u0e48าง", "หนองฉาง",
			"ห\u0e49วยคต", "เม\u0e37องอ\u0e38บลราชธาน\u0e35", "ก\u0e38ดข\u0e49าวป\u0e38\u0e49น", "เขมราฐ", "เข\u0e37\u0e48องใน", "โขงเจ\u0e35ยม", "เดชอ\u0e38ดม", "ตระการพ\u0e37ชผล", "ตาลส\u0e38ม", "ท\u0e38\u0e48งศร\u0e35อ\u0e38ดม",
			"นาจะหลวย", "น\u0e49ำย\u0e37น", "บ\u0e38ณฑร\u0e34ก", "พ\u0e34บ\u0e39ลม\u0e31งสาหาร", "โพธ\u0e34\u0e4cไทร", "ม\u0e48วงสามส\u0e34บ", "เหล\u0e48าเส\u0e37อโก\u0e49ก", "วาร\u0e34นชำราบ", "ศร\u0e35เม\u0e37องใหม\u0e48", "สำโรง",
			"ส\u0e34ร\u0e34นธร", "นาเย\u0e35ย", "นาตาล", "สว\u0e48างว\u0e35ระวงศ\u0e4c", "น\u0e49ำข\u0e38\u0e48น", "ดอนมดแดง"
		});
		this.G2_ampore.AutoCompleteMode = System.Windows.Forms.AutoCompleteMode.SuggestAppend;
		this.G2_ampore.AutoCompleteSource = System.Windows.Forms.AutoCompleteSource.CustomSource;
		this.G2_ampore.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_ampore = this.G2_ampore;
		location = new System.Drawing.Point(94, 152);
		g2_ampore.Location = location;
		this.G2_ampore.MaxLength = 255;
		this.G2_ampore.Name = "G2_ampore";
		DevComponents.DotNetBar.Controls.TextBoxX g2_ampore2 = this.G2_ampore;
		size = new System.Drawing.Size(105, 23);
		g2_ampore2.Size = size;
		this.G2_ampore.TabIndex = 6;
		this.Label24.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label24;
		location = new System.Drawing.Point(236, 154);
		label23.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label24 = this.Label24;
		size = new System.Drawing.Size(43, 16);
		label24.Size = size;
		this.Label24.TabIndex = 11;
		this.Label24.Text = "จ\u0e31งหว\u0e31ด";
		this.G2_Province.AutoCompleteCustomSource.AddRange(new string[77]
		{
			"กร\u0e38งเทพมหานคร", "กระบ\u0e35\u0e48", "กาญจนบ\u0e38ร\u0e35", "กาฬส\u0e34นธ\u0e38\u0e4c", "กำแพงเพชร", "ขอนแก\u0e48น", "จ\u0e31นทบ\u0e38ร\u0e35", "ฉะเช\u0e34งเทรา", "ชลบ\u0e38ร\u0e35", "ช\u0e31ยนาท",
			"ช\u0e31ยภ\u0e39ม\u0e34", "ช\u0e38มพร", "เช\u0e35ยงราย", "เช\u0e35ยงใหม\u0e48", "ตร\u0e31ง", "ตราด", "ตาก", "นครนายก", "นครปฐม", "นครพนม",
			"นครราชส\u0e35มา", "นครศร\u0e35ธรรมราช", "นครสวรรค\u0e4c", "นนทบ\u0e38ร\u0e35", "นราธ\u0e34วาส", "น\u0e48าน", "บ\u0e36งกาฬ", "บ\u0e38ร\u0e35ร\u0e31มย\u0e4c", "ปท\u0e38มธาน\u0e35", "ประจวบค\u0e35ร\u0e35ข\u0e31นธ\u0e4c",
			"ปราจ\u0e35นบ\u0e38ร\u0e35", "ป\u0e31ตตาน\u0e35", "พระนครศร\u0e35อย\u0e38ธยา", "พะเยา", "พ\u0e31งงา", "พ\u0e31ทล\u0e38ง", "พ\u0e34จ\u0e34ตร", "พ\u0e34ษณ\u0e38โลก", "เพชรบ\u0e38ร\u0e35", "เพชรบ\u0e39รณ\u0e4c",
			"แพร\u0e48", "ภ\u0e39เก\u0e47ต", "มหาสารคาม", "ม\u0e38กดาหาร", "แม\u0e48ฮ\u0e48องสอน", "ยโสธร", "ยะลา", "ร\u0e49อยเอ\u0e47ด", "ระนอง", "ระยอง",
			"ราชบ\u0e38ร\u0e35", "ลพบ\u0e38ร\u0e35", "ลำปาง", "ลำพ\u0e39น", "เลย", "ศร\u0e35สะเกษ", "สกลนคร", "สงขลา", "สต\u0e39ล", "สม\u0e38ทรปราการ",
			"สม\u0e38ทรสงคราม", "สม\u0e38ทรสาคร", "สระแก\u0e49ว", "สระบ\u0e38ร\u0e35", "ส\u0e34งห\u0e4cบ\u0e38ร\u0e35", "ส\u0e38โขท\u0e31ย", "ส\u0e38พรรณบ\u0e38ร\u0e35", "ส\u0e38ราษฎร\u0e4cธาน\u0e35", "ส\u0e38ร\u0e34นทร\u0e4c", "หนองคาย",
			"หนองบ\u0e31วลำภ\u0e39", "อ\u0e48างทอง", "อำนาจเจร\u0e34ญ", "อ\u0e38ดรธาน\u0e35", "อ\u0e38ตรด\u0e34ตถ\u0e4c", "อ\u0e38ท\u0e31ยธาน\u0e35", "อ\u0e38บลราชธาน\u0e35"
		});
		this.G2_Province.AutoCompleteMode = System.Windows.Forms.AutoCompleteMode.SuggestAppend;
		this.G2_Province.AutoCompleteSource = System.Windows.Forms.AutoCompleteSource.CustomSource;
		this.G2_Province.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Province = this.G2_Province;
		location = new System.Drawing.Point(281, 151);
		g2_Province.Location = location;
		this.G2_Province.MaxLength = 255;
		this.G2_Province.Name = "G2_Province";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Province2 = this.G2_Province;
		size = new System.Drawing.Size(110, 23);
		g2_Province2.Size = size;
		this.G2_Province.TabIndex = 7;
		this.Label25.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label25;
		location = new System.Drawing.Point(20, 155);
		label25.Location = location;
		this.Label25.Name = "Label25";
		System.Windows.Forms.Label label26 = this.Label25;
		size = new System.Drawing.Size(67, 16);
		label26.Size = size;
		this.Label25.TabIndex = 11;
		this.Label25.Text = "เขต/อำเภอ";
		this.G2_Tambon.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tambon = this.G2_Tambon;
		location = new System.Drawing.Point(281, 125);
		g2_Tambon.Location = location;
		this.G2_Tambon.MaxLength = 255;
		this.G2_Tambon.Name = "G2_Tambon";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Tambon2 = this.G2_Tambon;
		size = new System.Drawing.Size(110, 23);
		g2_Tambon2.Size = size;
		this.G2_Tambon.TabIndex = 5;
		this.G2_Soi.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Soi = this.G2_Soi;
		location = new System.Drawing.Point(281, 98);
		g2_Soi.Location = location;
		this.G2_Soi.MaxLength = 255;
		this.G2_Soi.Name = "G2_Soi";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Soi2 = this.G2_Soi;
		size = new System.Drawing.Size(110, 23);
		g2_Soi2.Size = size;
		this.G2_Soi.TabIndex = 3;
		this.G2_Road.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Road = this.G2_Road;
		location = new System.Drawing.Point(94, 125);
		g2_Road.Location = location;
		this.G2_Road.MaxLength = 255;
		this.G2_Road.Name = "G2_Road";
		DevComponents.DotNetBar.Controls.TextBoxX g2_Road2 = this.G2_Road;
		size = new System.Drawing.Size(105, 23);
		g2_Road2.Size = size;
		this.G2_Road.TabIndex = 4;
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label26;
		location = new System.Drawing.Point(7, 182);
		label27.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label28 = this.Label26;
		size = new System.Drawing.Size(80, 16);
		label28.Size = size;
		this.Label26.TabIndex = 11;
		this.Label26.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.GroupBox3.Controls.Add(this.ItemPanel1);
		System.Windows.Forms.GroupBox groupBox5 = this.GroupBox3;
		location = new System.Drawing.Point(577, 22);
		groupBox5.Location = location;
		this.GroupBox3.Name = "GroupBox3";
		System.Windows.Forms.GroupBox groupBox6 = this.GroupBox3;
		size = new System.Drawing.Size(357, 258);
		groupBox6.Size = size;
		this.GroupBox3.TabIndex = 10;
		this.GroupBox3.TabStop = false;
		this.GroupBox3.Text = "รายการสแกนเอกสาร";
		this.ItemPanel1.AllowDrop = true;
		this.ItemPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ItemPanel1.AutoScroll = true;
		this.ItemPanel1.BackgroundStyle.Class = "ItemPanel";
		this.ItemPanel1.ContainerControlProcessDialogKey = true;
		this.ItemPanel1.FitButtonsToContainerWidth = true;
		this.ItemPanel1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem1, this.ButtonItem2 });
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel1;
		location = new System.Drawing.Point(6, 15);
		itemPanel.Location = location;
		this.ItemPanel1.MultiLine = true;
		this.ItemPanel1.Name = "ItemPanel1";
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel1;
		size = new System.Drawing.Size(345, 234);
		itemPanel2.Size = size;
		this.ItemPanel1.TabIndex = 0;
		this.ItemPanel1.Text = "ItemPanel1";
		this.Tover.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tover.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tover = this.Tover;
		location = new System.Drawing.Point(105, 536);
		tover.Location = location;
		this.Tover.MaxLength = 255;
		this.Tover.Name = "Tover";
		this.Tover.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX tover2 = this.Tover;
		size = new System.Drawing.Size(105, 23);
		tover2.Size = size;
		this.Tover.TabIndex = 9;
		this.Tover.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.G1.Controls.Add(this.G1_No);
		this.G1.Controls.Add(this.Label2);
		this.G1.Controls.Add(this.Label3);
		this.G1.Controls.Add(this.Tcontry);
		this.G1.Controls.Add(this.G1_Fax);
		this.G1.Controls.Add(this.Label9);
		this.G1.Controls.Add(this.G1_Tel);
		this.G1.Controls.Add(this.Label11);
		this.G1.Controls.Add(this.Label33);
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
		location = new System.Drawing.Point(11, 286);
		g3.Location = location;
		this.G1.Name = "G1";
		System.Windows.Forms.GroupBox g4 = this.G1;
		size = new System.Drawing.Size(461, 238);
		g4.Size = size;
		this.G1.TabIndex = 11;
		this.G1.TabStop = false;
		this.G1.Text = "ท\u0e35\u0e48อย\u0e39\u0e48ป\u0e31จจ\u0e38บ\u0e31น";
		this.G1_No.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_No = this.G1_No;
		location = new System.Drawing.Point(94, 16);
		g1_No.Location = location;
		this.G1_No.MaxLength = 249;
		this.G1_No.Multiline = true;
		this.G1_No.Name = "G1_No";
		DevComponents.DotNetBar.Controls.TextBoxX g1_No2 = this.G1_No;
		size = new System.Drawing.Size(219, 48);
		g1_No2.Size = size;
		this.G1_No.TabIndex = 0;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label2;
		location = new System.Drawing.Point(55, 19);
		label29.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label30 = this.Label2;
		size = new System.Drawing.Size(37, 16);
		label30.Size = size;
		this.Label2.TabIndex = 11;
		this.Label2.Text = "เลขท\u0e35\u0e48";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label3;
		location = new System.Drawing.Point(319, 30);
		label31.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label32 = this.Label3;
		size = new System.Drawing.Size(24, 16);
		label32.Size = size;
		this.Label3.TabIndex = 11;
		this.Label3.Text = "หม\u0e39\u0e48";
		this.Tcontry.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tcontry = this.Tcontry;
		location = new System.Drawing.Point(94, 205);
		tcontry.Location = location;
		this.Tcontry.MaxLength = 255;
		this.Tcontry.Name = "Tcontry";
		DevComponents.DotNetBar.Controls.TextBoxX tcontry2 = this.Tcontry;
		size = new System.Drawing.Size(308, 23);
		tcontry2.Size = size;
		this.Tcontry.TabIndex = 9;
		this.G1_Fax.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Fax = this.G1_Fax;
		location = new System.Drawing.Point(94, 178);
		g1_Fax.Location = location;
		this.G1_Fax.MaxLength = 255;
		this.G1_Fax.Name = "G1_Fax";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Fax2 = this.G1_Fax;
		size = new System.Drawing.Size(308, 23);
		g1_Fax2.Size = size;
		this.G1_Fax.TabIndex = 9;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label9;
		location = new System.Drawing.Point(63, 154);
		label33.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label34 = this.Label9;
		size = new System.Drawing.Size(29, 16);
		label34.Size = size;
		this.Label9.TabIndex = 11;
		this.Label9.Text = "โทร";
		this.G1_Tel.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tel = this.G1_Tel;
		location = new System.Drawing.Point(94, 151);
		g1_Tel.Location = location;
		this.G1_Tel.MaxLength = 255;
		this.G1_Tel.Name = "G1_Tel";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tel2 = this.G1_Tel;
		size = new System.Drawing.Size(308, 23);
		g1_Tel2.Size = size;
		this.G1_Tel.TabIndex = 8;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label11;
		location = new System.Drawing.Point(59, 73);
		label35.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label36 = this.Label11;
		size = new System.Drawing.Size(33, 16);
		label36.Size = size;
		this.Label11.TabIndex = 11;
		this.Label11.Text = "ซอย";
		this.Label33.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label33;
		location = new System.Drawing.Point(44, 208);
		label37.Location = location;
		this.Label33.Name = "Label33";
		System.Windows.Forms.Label label38 = this.Label33;
		size = new System.Drawing.Size(49, 16);
		label38.Size = size;
		this.Label33.TabIndex = 11;
		this.Label33.Text = "ประเทศ";
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label12;
		location = new System.Drawing.Point(258, 73);
		label39.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label40 = this.Label12;
		size = new System.Drawing.Size(32, 16);
		label40.Size = size;
		this.Label12.TabIndex = 11;
		this.Label12.Text = "ถนน";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label10;
		location = new System.Drawing.Point(63, 181);
		label41.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label42 = this.Label10;
		size = new System.Drawing.Size(28, 16);
		label42.Size = size;
		this.Label10.TabIndex = 11;
		this.Label10.Text = "Fax";
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label43 = this.Label13;
		location = new System.Drawing.Point(22, 100);
		label43.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label44 = this.Label13;
		size = new System.Drawing.Size(71, 16);
		label44.Size = size;
		this.Label13.TabIndex = 11;
		this.Label13.Text = "แขวง/ตำบล";
		this.G1_Code.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Code = this.G1_Code;
		location = new System.Drawing.Point(292, 124);
		g1_Code.Location = location;
		this.G1_Code.MaxLength = 255;
		this.G1_Code.Name = "G1_Code";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Code2 = this.G1_Code;
		size = new System.Drawing.Size(110, 23);
		g1_Code2.Size = size;
		this.G1_Code.TabIndex = 7;
		this.G1_Moo.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Moo = this.G1_Moo;
		location = new System.Drawing.Point(346, 27);
		g1_Moo.Location = location;
		this.G1_Moo.MaxLength = 255;
		this.G1_Moo.Name = "G1_Moo";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Moo2 = this.G1_Moo;
		size = new System.Drawing.Size(56, 23);
		g1_Moo2.Size = size;
		this.G1_Moo.TabIndex = 1;
		this.G1_ampore.AutoCompleteCustomSource.AddRange(new string[926]
		{
			"เม\u0e37องกระบ\u0e35\u0e48", "เกาะล\u0e31นตา", "เขาพนม", "คลองท\u0e48อม", "ปลายพระยา", "ลำท\u0e31บ", "เหน\u0e37อคลอง", "อ\u0e48าวล\u0e36ก", "คลองสาน", "คลองเตย",
			"จอมทอง", "จต\u0e38จ\u0e31กร", "ด\u0e38ส\u0e34ต", "ดอนเม\u0e37อง", "ตล\u0e34\u0e48งช\u0e31น", "ธนบ\u0e38ร\u0e35", "บางกอกน\u0e49อย", "บางกอกใหญ\u0e48", "บางกะป\u0e34", "บางข\u0e38นเท\u0e35ยน",
			"บางเขน", "บางคอแหลม", "บางซ\u0e37\u0e48อ", "บางพล\u0e31ด", "บางร\u0e31ก", "บ\u0e36งก\u0e38\u0e48ม", "ประเวศ", "ปท\u0e38มว\u0e31น", "ป\u0e49อมปราบศ\u0e31ตร\u0e39พ\u0e48าย", "พญาไท",
			"พระโขนง", "พระนคร", "ภาษ\u0e35เจร\u0e34ญ", "ม\u0e35นบ\u0e38ร\u0e35", "ยานนาวา", "ราชเทว\u0e35", "ราษฎร\u0e4cบ\u0e39รณะ", "ลาดกระบ\u0e31ง", "ลาดพร\u0e49าว", "สาทร",
			"ส\u0e31มพ\u0e31นธวงศ\u0e4c", "หนองแขม", "หนองจอก", "ห\u0e49วยขวาง", "สวนหลวง", "ด\u0e34นแดง", "หล\u0e31กส\u0e35\u0e48", "สายไหม", "ค\u0e31นนายาว", "สะพานส\u0e39ง",
			"ว\u0e31งทองหลาง", "คลองสามวา", "ว\u0e31ฒนา", "บางนา", "ทว\u0e35ว\u0e31ฒนา", "บางแค", "ท\u0e38\u0e48งคร\u0e38", "บางบอน", "เม\u0e37องกาญจนบ\u0e38ร\u0e35", "ด\u0e48านมะขามเต\u0e35\u0e49ย",
			"ทองผาภ\u0e39ม\u0e34", "ท\u0e48าม\u0e48วง", "ท\u0e48ามะกา", "ไทรโยค", "บ\u0e48อพลอย", "พนมทวน", "เลาขว\u0e31ญ", "ศร\u0e35สว\u0e31สด\u0e34\u0e4c", "ส\u0e31งขละบ\u0e38ร\u0e35", "หนองปร\u0e37อ",
			"ห\u0e49วยกระเจา", "เม\u0e37องกาฬส\u0e34นธ\u0e38\u0e4c", "กมลาไสย", "ก\u0e38ฉ\u0e34นารายณ\u0e4c", "เขาวง", "คำม\u0e48วง", "ท\u0e48าค\u0e31นโท", "นามน", "ยางตลาด", "ร\u0e48องคำ",
			"สมเด\u0e47จ", "สห\u0e31สข\u0e31นธ\u0e4c", "หนองก\u0e38งศร\u0e35", "ห\u0e49วยผ\u0e36\u0e49ง", "ห\u0e49วยเม\u0e47ก", "นาค\u0e39", "สามช\u0e31ย", "ดอนจาน", "ฆ\u0e49องช\u0e31ย", "เม\u0e37องกำแพงเพชร",
			"ขาณ\u0e38วรล\u0e31กษบ\u0e38ร\u0e35", "คลองขล\u0e38ง", "คลองลาน", "ทรายทองว\u0e31ฒนา", "ไทรงาม", "ปางศ\u0e34ลาทอง", "พรานกระต\u0e48าย", "ลานกระบ\u0e37อ", "บ\u0e36งสาม\u0e31คค\u0e35", "โกส\u0e31มพ\u0e35นคร",
			"เม\u0e37องขอนแก\u0e48น", "กระนวน", "เขาสวนกวาง", "โคกโพธ\u0e34\u0e4cไชย", "ชำส\u0e39ง", "ชนบท", "ช\u0e38มแพ", "น\u0e49ำพอง", "บ\u0e49านไผ\u0e48", "บ\u0e49านฝาง",
			"เป\u0e37อยน\u0e49อย", "พล", "พระย\u0e37น", "ภ\u0e39เว\u0e35ยง", "ภ\u0e39ผาม\u0e48าน", "ม\u0e31ญจาค\u0e35ร\u0e35", "แวงน\u0e49อย", "แวงใหญ\u0e48", "ส\u0e35ชมพ\u0e39", "หนองสองห\u0e49อง",
			"หนองเร\u0e37อ", "หนองนาคำ", "อ\u0e38บลร\u0e31ตน\u0e4c", "โนนศ\u0e34ลา", "บ\u0e49านแฮด", "เม\u0e37องจ\u0e31นทบ\u0e38ร\u0e35", "แก\u0e48งหางแมว", "ขล\u0e38ง", "ท\u0e48าใหม\u0e48", "นายายอาม",
			"โป\u0e48งน\u0e49ำร\u0e49อน", "มะขาม", "สอยดาว", "แหลมส\u0e34งห\u0e4c", "เขาค\u0e34ชฌก\u0e39ฏ", "เม\u0e37องฉะเช\u0e34งเทรา", "บางคล\u0e49า", "บางน\u0e49ำเปร\u0e35\u0e49ยว", "บางปะกง", "บ\u0e49านโพธ\u0e34\u0e4c",
			"แปลงยาว", "พนมสารคาม", "ราชสาส\u0e4cน", "สนามช\u0e31ยเขต", "ท\u0e48าตะเก\u0e35ยบ", "คลองเข\u0e37\u0e48อน", "เม\u0e37องชลบ\u0e38ร\u0e35", "เกาะส\u0e35ช\u0e31ง", "บ\u0e48อทอง", "บางละม\u0e38ง",
			"บ\u0e49านบ\u0e36ง", "พานทอง", "พน\u0e31สน\u0e34คม", "ศร\u0e35ราชา", "ส\u0e31ตห\u0e35บ", "หนองใหญ\u0e48", "เกาะจ\u0e31นทร\u0e4c", "เม\u0e37องช\u0e31ยนาท", "มโนรมย\u0e4c", "ว\u0e31ดส\u0e34งห\u0e4c",
			"สรรคบ\u0e38ร\u0e35", "สรรพยา", "ห\u0e31นคา", "หนองมะโมง", "เน\u0e34นขาม", "เม\u0e37องช\u0e31ยภ\u0e39ม\u0e34", "เกษตรสมบ\u0e39รณ\u0e4c", "แก\u0e49งคร\u0e49อ", "คอนสวรรค\u0e4c", "คอนสาร",
			"จ\u0e31ต\u0e38ร\u0e31ส", "เทพสถ\u0e34ต", "เน\u0e34นสง\u0e48า", "บ\u0e49านเขว\u0e49า", "บ\u0e49านแท\u0e48น", "บำเหน\u0e47จณรงค\u0e4c", "ภ\u0e39เข\u0e35ยว", "ภ\u0e31กด\u0e35ช\u0e38มพล", "หนองบ\u0e31วแดง", "หนองบ\u0e31วระเหว",
			"ซ\u0e31บใหญ\u0e48", "เม\u0e37องช\u0e38มพร", "ท\u0e48าแซะ", "ท\u0e38\u0e48งตะโก", "ปะท\u0e34ว", "พะโต\u0e4aะ", "ละแม", "สว\u0e35", "หล\u0e31งสวน", "เม\u0e37องเช\u0e35ยงราย",
			"ข\u0e38นตาล", "เช\u0e35ยงของ", "เช\u0e35ยงแสน", "เท\u0e34ง", "ป\u0e48าแดด", "พาน", "แม\u0e48จ\u0e31น", "แม\u0e48ฟ\u0e49าหลวง", "แม\u0e48สรวย", "แม\u0e48สาย",
			"เว\u0e35ยงแก\u0e48น", "เว\u0e35ยงช\u0e31ย", "เว\u0e35ยงป\u0e48าเป\u0e49า", "พญาเม\u0e47งราย", "แม\u0e48ลาว", "ดอยหลวง", "เว\u0e35ยงเช\u0e35ยงร\u0e38\u0e49ง", "เม\u0e37องเช\u0e35ยงใหม\u0e48", "จอมทอง", "เช\u0e35ยงดาว",
			"ไชยปราการ", "ดอยเต\u0e48า", "ดอยหล\u0e48อ", "ดอยสะเก\u0e47ด", "ฝาง", "พร\u0e49าว", "แม\u0e48แจ\u0e48ม", "แม\u0e48แตง", "แม\u0e48ร\u0e34ม", "แม\u0e48วาง",
			"แม\u0e48อาย", "แม\u0e48ออน", "เว\u0e35ยงแหง", "สะเม\u0e34ง", "ส\u0e31นกำแพง", "ส\u0e31นทราย", "ส\u0e31นป\u0e48าตอง", "สารภ\u0e35", "หางดง", "อมก\u0e4bอย",
			"ฮอด", "เม\u0e37องตร\u0e31ง", "ก\u0e31นต\u0e31ง", "ปะเหล\u0e35ยน", "ย\u0e48านตาขาว", "ร\u0e31ษฎา", "ส\u0e34เกา", "ห\u0e49วยยอด", "ว\u0e31งว\u0e34เศษ", "หาดสำราญ",
			"นาโยง", "เม\u0e37องตราด", "เกาะช\u0e49าง", "เขาสม\u0e34ง", "คลองใหญ\u0e48", "บ\u0e48อไร\u0e48", "แหลมงอบ", "เกาะก\u0e39ด", "เม\u0e37องตาก", "ท\u0e48าสองยาง",
			"บ\u0e49านตาก", "พบพระ", "แม\u0e48ระมาด", "แม\u0e48สอด", "สามเงา", "อ\u0e38\u0e49มผาง", "ว\u0e31งเจ\u0e49า", "เม\u0e37องนครนายก", "บ\u0e49านนา", "ปากพล\u0e35",
			"องคร\u0e31กษ\u0e4c", "เม\u0e37องนครปฐม", "กำแพงแสน", "ดอนต\u0e39ม", "นครช\u0e31ยศร\u0e35", "บางเลน", "พ\u0e38ทธมณฑล", "สามพราน", "เม\u0e37องนครพนม", "ท\u0e48าอ\u0e38เทน",
			"ธาต\u0e38พนม", "นาแก", "นาหว\u0e49า", "บ\u0e49านแพง", "ปลาปาก", "โพนสวรรค\u0e4c", "เรณ\u0e39นคร", "ศร\u0e35สงคราม", "ว\u0e31งยาง", "นาทม",
			"เม\u0e37องนครราชส\u0e35มา", "แก\u0e49งสนามนาง", "ขามทะเลสอ", "ขามสะแกแสง", "คง", "ครบ\u0e38ร\u0e35", "จ\u0e31กราช", "ช\u0e38มพวง", "โชคช\u0e31ย", "ด\u0e48านข\u0e38นทด",
			"โนนแดง", "โนนไทย", "โนนส\u0e39ง", "บ\u0e31วใหญ\u0e48", "บ\u0e49านเหล\u0e37\u0e48อม", "ประทาย", "ป\u0e31กธงช\u0e31ย", "ปากช\u0e48อง", "พ\u0e34มาย", "ว\u0e31งน\u0e49ำเข\u0e35ยว",
			"ส\u0e35ค\u0e34\u0e49ว", "ส\u0e39งเน\u0e34น", "เส\u0e34งสาง", "ห\u0e49วยแถลง", "หนองบ\u0e38นนาก", "เทพาร\u0e31กษ\u0e4c", "เม\u0e37องยาง", "พระทองคำ", "ลำทะเมนช\u0e31ย", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34",
			"ส\u0e35ดา", "บ\u0e31วลาย", "เม\u0e37องนครศร\u0e35ธรรมราช", "ขนอม", "ฉวาง", "ชะอวด", "เช\u0e35ยรใหญ\u0e48", "ท\u0e48าศาลา", "ท\u0e38\u0e48งใหญ\u0e48", "ท\u0e38\u0e48งสง",
			"พระพรหม", "นาบอน", "บางข\u0e31น", "ปากพน\u0e31ง", "พรหมค\u0e35ร\u0e35", "พ\u0e34ป\u0e39น", "ร\u0e48อนพ\u0e34บ\u0e39ลย\u0e4c", "ลานสะกา", "ส\u0e34ชล", "ห\u0e31วไทร",
			"จ\u0e38ฬาภรณ\u0e4c", "นบพ\u0e34ตำ", "ช\u0e49างกลาง", "ถ\u0e49ำพรรณรา", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "เม\u0e37องนครสวรรค\u0e4c", "เก\u0e49าเล\u0e35\u0e49ยว", "โกรกพระ", "ช\u0e38มแสง", "ตากฟ\u0e49า",
			"ตาคล\u0e35", "ท\u0e48าตะโก", "บรรพตพ\u0e34ส\u0e31ย", "พย\u0e38หค\u0e35ร\u0e35", "ไพศาล\u0e35", "แม\u0e48วงก\u0e4c", "ลาดยาว", "หนองบ\u0e31ว", "แม\u0e48เป\u0e34น", "ช\u0e38มตาบง",
			"เม\u0e37องนนทบ\u0e38ร\u0e35", "ไทรน\u0e49อย", "บางกรวย", "บางบ\u0e31วทอง", "บางใหญ\u0e48", "ปากเกร\u0e47ด", "เม\u0e37องนราธ\u0e34วาส", "จะแนะ", "ตากใบ", "บาเจาะ",
			"ย\u0e35\u0e48งอ", "ระแงะ", "ร\u0e37อเสาะ", "แว\u0e49ง", "ศร\u0e35สาคร", "ส\u0e38ค\u0e34ร\u0e34น", "ส\u0e38ไหงโกลก", "ส\u0e38ไหงปาด\u0e35", "เจาะไอร\u0e49อง", "เม\u0e37องน\u0e48าน",
			"เช\u0e35ยงกลาง", "ท\u0e48าว\u0e31งผา", "ท\u0e38\u0e48งช\u0e49าง", "นาน\u0e49อย", "นาหม\u0e37\u0e48น", "บ\u0e49านหลวง", "ป\u0e31ว", "แม\u0e48จร\u0e34ม", "เว\u0e35ยงสา", "ส\u0e31นต\u0e34ส\u0e38ข",
			"บ\u0e48อเกล\u0e37อ", "สองแคว", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "ภ\u0e39เพ\u0e35ยง", "เม\u0e37องบ\u0e38ร\u0e35ร\u0e31มย\u0e4c", "กระส\u0e31ง", "ค\u0e39เม\u0e37อง", "ชำน\u0e34", "นาโพธ\u0e34\u0e4c", "นางรอง",
			"โนนด\u0e34นแดง", "โนนส\u0e38วรรณ", "บ\u0e49านกรวด", "พล\u0e31บพลาช\u0e31ย", "บ\u0e49านใหม\u0e48ไชยพจน\u0e4c", "ประโคนช\u0e31ย", "ปะคำ", "พ\u0e38ทไธสง", "ละหานทราย", "ลำปลายมาศ",
			"สต\u0e36ก", "หนองก\u0e35\u0e48", "หนองหงส\u0e4c", "ห\u0e49วยราช", "บ\u0e49านด\u0e48าน", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "แคนดง", "เม\u0e37องปท\u0e38มธาน\u0e35", "คลองหลวง", "ธ\u0e31ญบ\u0e38ร\u0e35",
			"ลาดหล\u0e38มแก\u0e49ว", "ลำล\u0e39กกา", "สามโคก", "หนองเส\u0e37อ", "เม\u0e37องประจวบค\u0e35ร\u0e35ข\u0e31นธ\u0e4c", "ก\u0e38ยบ\u0e38ร\u0e35", "ท\u0e31บสะแก", "บางสะพาน", "บางสะพานน\u0e49อย", "ปราณบ\u0e38ร\u0e35",
			"ห\u0e31วห\u0e34น", "สามร\u0e49อยยอด", "เม\u0e37องปราจ\u0e35นบ\u0e38ร\u0e35", "กบ\u0e34นทร\u0e4cบ\u0e38ร\u0e35", "ศร\u0e35มโหสถ", "นาด\u0e35", "บ\u0e49านสร\u0e49าง", "ประจ\u0e31นตคาม", "ศร\u0e35มหาโพธ\u0e34", "เม\u0e37องป\u0e31ตตาน\u0e35",
			"กะพ\u0e49อ", "โคกโพธ\u0e34\u0e4c", "ท\u0e38\u0e48งยางแดง", "ปะนาเระ", "มายอ", "ไม\u0e49แก\u0e48น", "ยะร\u0e31ง", "ยะหร\u0e34\u0e48ง", "สายบ\u0e38ร\u0e35", "หนองจ\u0e34ก",
			"แม\u0e48ลาน", "พระนครศร\u0e35อย\u0e38ธยา", "ท\u0e48าเร\u0e37อ", "นครหลวง", "บางซ\u0e49าย", "บางไทร", "บางบาล", "บางปะห\u0e31น", "บางปะอ\u0e34น", "บ\u0e49านแพรก",
			"ผ\u0e31กไห\u0e48", "ภาช\u0e35", "มหาราช", "ลาดบ\u0e31วหลวง", "ว\u0e31งน\u0e49อย", "เสนา", "อ\u0e38ท\u0e31ย", "เม\u0e37องพะเยา", "จ\u0e38น", "เช\u0e35ยงคำ",
			"เช\u0e35ยงม\u0e48วน", "ดอกคำใต\u0e49", "ปง", "แม\u0e48ใจ", "ภ\u0e39ซาง", "ภ\u0e39กามยาว", "เม\u0e37องพ\u0e31งงา", "กะปง", "เกาะยาว", "ค\u0e38ระบ\u0e38ร\u0e35",
			"ตะก\u0e31\u0e48วท\u0e38\u0e48ง", "ตะก\u0e31\u0e48วป\u0e48า", "ท\u0e31บป\u0e38ด", "ท\u0e49ายเหม\u0e37อง", "เม\u0e37องพ\u0e31ทล\u0e38ง", "กงหรา", "เขาช\u0e31ยสน", "ควนขน\u0e38น", "ตะโหมด", "ปากพะย\u0e39น",
			"ป\u0e48าบอน", "ป\u0e48าพะยอม", "ศร\u0e35บรรพต", "บางแก\u0e49ว", "ศร\u0e35นคร\u0e34นทร\u0e4c", "เม\u0e37องพ\u0e34จ\u0e34ตร", "ตะพานห\u0e34น", "ท\u0e31บคล\u0e49อ", "บางม\u0e39ลนาก", "โพทะเล",
			"โพธ\u0e34\u0e4cประท\u0e31บช\u0e49าง", "สามง\u0e48าม", "ว\u0e31งทรายพ\u0e39น", "สากเหล\u0e47ก", "บ\u0e36งนาราง", "ดงเจร\u0e34ญ", "วช\u0e34รบารม\u0e35", "เม\u0e37องพ\u0e34ษณ\u0e38โลก", "นครไทย", "ชาต\u0e34ตระการ",
			"เน\u0e34นมะปราง", "บางกระท\u0e38\u0e48ม", "บางระกำ", "พรหมพ\u0e34ราม", "ว\u0e31งทอง", "ว\u0e31ดโบสถ\u0e4c", "เม\u0e37องเพชรบ\u0e38ร\u0e35", "แก\u0e48งกระจาน", "เขาย\u0e49อย", "ชะอำ",
			"ท\u0e48ายาง", "บ\u0e49านลาด", "บ\u0e49านแหลม", "หนองหญ\u0e49าปล\u0e49อง", "เม\u0e37องเพชรบ\u0e39รณ\u0e4c", "เขาค\u0e49อ", "ชนแดน", "น\u0e49ำหนาว", "บ\u0e36งสามพ\u0e31น", "ว\u0e34เช\u0e35ยรบ\u0e38ร\u0e35",
			"ศร\u0e35เทพ", "หนองไผ\u0e48", "หล\u0e48มเก\u0e48า", "หล\u0e48มส\u0e31ก", "ว\u0e31งโป\u0e48ง", "เม\u0e37องแพร\u0e48", "เด\u0e48นช\u0e31ย", "ร\u0e49องกวาง", "ลอง", "ว\u0e31งช\u0e34\u0e49น",
			"สอง", "หนองม\u0e48วงไข\u0e48", "ส\u0e39งเม\u0e48น", "เม\u0e37องภ\u0e39เก\u0e47ต", "กะท\u0e39\u0e49", "ถลาง", "เม\u0e37องมหาสารคาม", "ก\u0e31นทรว\u0e34ช\u0e31ย", "แกดำ", "โกส\u0e38มพ\u0e34ส\u0e31ย",
			"เช\u0e35ยงย\u0e37น", "นาเช\u0e37อก", "นาด\u0e39น", "บรบ\u0e37อ", "พย\u0e31คฆภ\u0e39ม\u0e34พ\u0e34ส\u0e31ย", "วาป\u0e35ปท\u0e38ม", "ก\u0e38ดร\u0e31ง", "ยางส\u0e35ส\u0e38ราช", "ช\u0e37\u0e48นชม", "เม\u0e37องม\u0e38กดาหาร",
			"คำชะอ\u0e35", "ดงหลวง", "ดอนตาล", "น\u0e34คมคำสร\u0e49อย", "หนองส\u0e39ง", "หว\u0e49านใหญ\u0e48", "เม\u0e37องแม\u0e48ฮ\u0e48องสอน", "ข\u0e38นยวม", "ปางมะผ\u0e49า", "ปาย",
			"แม\u0e48ลาน\u0e49อย", "แม\u0e48สะเร\u0e35ยง", "สบเมย", "เม\u0e37องยโสธร", "ก\u0e38ดช\u0e38ม", "ค\u0e49อว\u0e31ง", "คำเข\u0e37\u0e48อนแก\u0e49ว", "ไทยเจร\u0e34ญ", "ทรายม\u0e39ล", "ป\u0e48าต\u0e34\u0e49ว",
			"มหาชนะช\u0e31ย", "เล\u0e34งนกทา", "เม\u0e37องยะลา", "กาบ\u0e31ง", "กรงป\u0e34น\u0e31ง", "ธารโต", "บ\u0e31นน\u0e31งสตา", "เบตง", "ยะหา", "ราม\u0e31น",
			"เม\u0e37องร\u0e49อยเอ\u0e47ด", "เกษตรว\u0e34ส\u0e31ย", "จต\u0e38รพ\u0e31กตร\u0e4cพ\u0e34มาน", "จ\u0e31งหาร", "ธว\u0e31ชบ\u0e38ร\u0e35", "ปท\u0e38มร\u0e31ตน\u0e4c", "พนมไพร", "โพธ\u0e34\u0e4cช\u0e31ย", "โพนทราย", "โพนทอง",
			"เมยวด\u0e35", "เม\u0e37องสรวง", "ศร\u0e35สมเด\u0e47จ", "เสลภ\u0e39ม\u0e34", "ส\u0e38วรรณภ\u0e39ม\u0e34", "หนองพอก", "อาจสามารถ", "เช\u0e35ยงขว\u0e31ญ", "หนองฮ\u0e35", "ท\u0e38\u0e48งเขาหลวง",
			"เม\u0e37องระนอง", "กระบ\u0e38ร\u0e35", "กะเปอร\u0e4c", "ละอ\u0e38\u0e48น", "ส\u0e38ขสำราญ", "เม\u0e37องระยอง", "แกลง", "บ\u0e49านค\u0e48าย", "บ\u0e49านฉาง", "ปลวกแดง",
			"ว\u0e31งจ\u0e31นทร\u0e4c", "เขาชะเมา", "น\u0e34คมพ\u0e31ฒนา", "เม\u0e37องราชบ\u0e38ร\u0e35", "จอมบ\u0e36ง", "ดำเน\u0e34นสะดวก", "บางแพ", "บ\u0e49านโป\u0e48ง", "ปากท\u0e48อ", "โพธาราม",
			"ว\u0e31ดเพลง", "สวนผ\u0e36\u0e49ง", "บ\u0e49านคา", "เม\u0e37องลพบ\u0e38ร\u0e35", "โคกเจร\u0e34ญ", "โคกสำโรง", "ช\u0e31ยบาดาล", "ท\u0e48าว\u0e38\u0e49ง", "ท\u0e48าหลวง", "บ\u0e49านหม\u0e35\u0e48",
			"พ\u0e31ฒนาน\u0e34คม", "ลำสนธ\u0e34", "สระโบถส\u0e4c", "หนองม\u0e48วง", "เม\u0e37องเลย", "เช\u0e35ยงคาน", "ด\u0e48านซ\u0e49าย", "ท\u0e48าล\u0e35\u0e48", "นาด\u0e49วง", "นาแห\u0e49ว",
			"ปากชม", "ผาขาว", "ภ\u0e39กระด\u0e36ง", "ภ\u0e39เร\u0e37อ", "ภ\u0e39หลวง", "ว\u0e31งสะพ\u0e38ง", "เอราว\u0e31ณ", "หนองห\u0e34น", "เม\u0e37องลำปาง", "เกาะคา",
			"งาว", "แจ\u0e49ห\u0e48ม", "เถ\u0e34น", "แม\u0e48ทะ", "แม\u0e48พร\u0e34ก", "เม\u0e37องปาน", "แม\u0e48เมาะ", "ว\u0e31งเหน\u0e37อ", "สบปราบ", "เสร\u0e34มงาม",
			"ห\u0e49างฉ\u0e31ตร", "เม\u0e37องลำพ\u0e39น", "ท\u0e38\u0e48งห\u0e31วช\u0e49าง", "บ\u0e49านโฮ\u0e48ง", "ป\u0e48าซาง", "แม\u0e48ทา", "ล\u0e35\u0e49", "บ\u0e49านธ\u0e34", "เว\u0e35ยงหนองล\u0e48อง", "เม\u0e37องศ\u0e35รสะเกษ",
			"ก\u0e31นทรล\u0e31กษ\u0e4c", "ก\u0e31นทรารมย\u0e4c", "ข\u0e38ข\u0e31นธ\u0e4c", "ข\u0e38นหาญ", "น\u0e49ำเกล\u0e35\u0e49ยง", "โนนค\u0e39ณ", "บ\u0e36งบ\u0e39รพ\u0e4c", "เบญจล\u0e31กษณ\u0e4c", "ปรางค\u0e4cก\u0e39\u0e48", "พย\u0e38ห\u0e4c",
			"ไพรบ\u0e36ง", "โพธ\u0e34\u0e4cศร\u0e35ส\u0e38วรรณ", "ภ\u0e39ส\u0e34งห\u0e4c", "เม\u0e37องจ\u0e31นทร\u0e4c", "ยางช\u0e38มน\u0e49อย", "ราษ\u0e35ไศล", "ว\u0e31งห\u0e34น", "ศร\u0e35ร\u0e31ตนะ", "ห\u0e49วยท\u0e31บท\u0e31น", "อ\u0e38ท\u0e38มพรพ\u0e34ส\u0e31ย",
			"ศ\u0e34ลาลาด", "เม\u0e37องสกลนคร", "ก\u0e38ดบาก", "ก\u0e38ส\u0e38มาลย\u0e4c", "คำตากล\u0e49า", "เจร\u0e34ญศ\u0e34ลป\u0e4c", "เต\u0e48างอย", "น\u0e34คมน\u0e49ำอ\u0e39น", "บ\u0e49านม\u0e48วง", "พรรณาน\u0e34คม",
			"พ\u0e31งโคน", "วานรน\u0e34วาส", "วาร\u0e34ชภ\u0e39ม\u0e34", "โคกศร\u0e35ส\u0e38พรรณ", "สว\u0e48างแดนด\u0e34น", "ส\u0e48องดาว", "อากาศอำนวย", "ภ\u0e39พาน", "โพนนาแก\u0e49ว", "เม\u0e37องสงขลา",
			"กระแสส\u0e34นธ\u0e38\u0e4c", "ควนเน\u0e35ยง", "จะนะ", "เทพา", "นาทว\u0e35", "นาหม\u0e48อม", "บางกล\u0e48ำ", "ระโนด", "ร\u0e31ตภ\u0e39ม\u0e34", "สท\u0e34งพระ",
			"สะเดา", "สะบ\u0e49าย\u0e49อย", "ส\u0e34งหนคร", "หาดใหญ\u0e48", "คลองหอยโข\u0e48ง", "เม\u0e37องสต\u0e39ล", "ควนกาหลง", "ควนโดน", "ท\u0e48าแพ", "ท\u0e38\u0e48งหว\u0e49า",
			"ละง\u0e39", "มะน\u0e31ง", "เม\u0e37องสม\u0e38ทรปราการ", "บางบ\u0e48อ", "บางพล\u0e35", "พระประแดง", "พระสม\u0e38ทรเจด\u0e35ย\u0e4c", "บางเสาธง", "เม\u0e37องสม\u0e38ทรสงคราม", "บางคณท\u0e35",
			"อ\u0e31มพวา", "เม\u0e37องสม\u0e38ทรสาคร", "กระท\u0e38\u0e48มแบน", "บ\u0e49านแพ\u0e49ว", "เม\u0e37องสระแก\u0e49ว", "เขาฉกรรจ\u0e4c", "คลองหาด", "ตาพระยา", "ว\u0e31งน\u0e49ำเย\u0e47น", "ว\u0e31ฒนานคร",
			"อร\u0e31ญประเทศ", "โคกส\u0e39ง", "ว\u0e31งสมบ\u0e39รณ\u0e4c", "เม\u0e37องสระบ\u0e38ร\u0e35", "แก\u0e48งคอย", "ดอนพ\u0e38ด", "บ\u0e49านหมอ", "พระพ\u0e38ทธบาท", "มวกเหล\u0e47ก", "ว\u0e34หารแดง",
			"เสาไห\u0e49", "หนองแค", "หนองแซง", "หนองโดน", "ว\u0e31งม\u0e48วง", "เฉล\u0e34มพระเก\u0e35ยรต\u0e34", "เม\u0e37องส\u0e34งห\u0e4cบ\u0e38ร\u0e35", "ค\u0e48ายบางระจ\u0e31น", "ท\u0e48าช\u0e49าง", "บางระจ\u0e31น",
			"พรหมบ\u0e38ร\u0e35", "อ\u0e34นทร\u0e4cบ\u0e38ร\u0e35", "เม\u0e37องส\u0e38โขท\u0e31ย", "กงไกรลาศ", "ค\u0e35ร\u0e35มาศ", "ท\u0e38\u0e48งเสล\u0e35\u0e48ยม", "บ\u0e49านด\u0e48านลานหอย", "ศร\u0e35นคร", "ศร\u0e35ส\u0e31ชนาล\u0e31ย", "ศร\u0e35สำโรง",
			"สวรรคโลก", "เม\u0e37องส\u0e38พรรณบ\u0e38ร\u0e35", "ดอนเจด\u0e35ย\u0e4c", "ด\u0e48านช\u0e49าง", "เด\u0e34มบางนางบวช", "บางปลาม\u0e49า", "ศร\u0e35ประจ\u0e31นต\u0e4c", "สองพ\u0e35\u0e48น\u0e49อง", "สามช\u0e38ก", "อ\u0e39\u0e48ทอง",
			"หนองหญ\u0e49าไซ", "เม\u0e37องส\u0e38ราษฎร\u0e4cธาน\u0e35", "กาญจนด\u0e34ษฐ\u0e4c", "เกาะพะง\u0e31น", "เกาะสม\u0e38ย", "ค\u0e35ร\u0e35ร\u0e31ฐน\u0e34คม", "เค\u0e35ยนซา", "ช\u0e31ยบ\u0e38ร\u0e35", "ไชยา", "ดอนส\u0e31ก",
			"ท\u0e48าฉาง", "ท\u0e48าชนะ", "บ\u0e49านตาข\u0e38น", "บ\u0e49านนาเด\u0e34ม", "บ\u0e49านนาสาร", "พนม", "พระแสง", "พ\u0e38นพ\u0e34น", "ว\u0e34ภาวด\u0e35", "เว\u0e35ยงสระ",
			"เม\u0e37องส\u0e38ร\u0e34นทร\u0e4c", "กาบเช\u0e34ง", "จอมพระ", "ช\u0e38มพลบ\u0e38ร\u0e35", "ท\u0e48าต\u0e39ม", "บ\u0e31วเชด", "ปราสาท", "ร\u0e31ตนบ\u0e38ร\u0e35", "ลำดวน", "ศ\u0e35ขรภ\u0e39ม\u0e34",
			"สนม", "ส\u0e31งขะ", "สำโรงทาบ", "ศร\u0e35ณรงค\u0e4c", "พนมดงร\u0e31ก", "เขวาส\u0e34นร\u0e34นทร\u0e4c", "โนนนารายณ\u0e4c", "เม\u0e37องหนองคาย", "เซกา", "โซ\u0e48พ\u0e34ส\u0e31ย",
			"ท\u0e48าบ\u0e48อ", "บ\u0e36งกาฬ", "บ\u0e36งโขลงหลง", "ปากคาด", "พรเจร\u0e34ญ", "โพนพ\u0e34ส\u0e31ย", "ศร\u0e35เช\u0e35ยงใหม\u0e48", "ศร\u0e35ว\u0e34ไล", "ส\u0e31งคม", "สระใคร\u0e48",
			"บ\u0e38\u0e48งคล\u0e49า", "ร\u0e31ตนวาป\u0e35", "เฝ\u0e49าไร\u0e48", "โพธ\u0e34\u0e4cตาก", "เม\u0e37องหนองบ\u0e31วลำภ\u0e39", "นากลาง", "โนนส\u0e31ง", "ศร\u0e35บ\u0e38ญเร\u0e37อง", "ส\u0e38วรรณค\u0e39หา", "นาว\u0e31ง",
			"เม\u0e37องอ\u0e48างทอง", "ไชโย", "ป\u0e48าโมก", "โพธ\u0e34\u0e4cทอง", "ว\u0e34เศษช\u0e31ยชาญ", "สามโก\u0e49", "แสวงหา", "เม\u0e37องอำนาจเจร\u0e34ญ", "ชาน\u0e38มาน", "ปท\u0e38มราชวงศา",
			"พนา", "เสนางคน\u0e34คม", "ห\u0e31วตะพาน", "ล\u0e37ออำนาจ", "เม\u0e37องอ\u0e38ดรธาน\u0e35", "ก\u0e39\u0e48แก\u0e49ว", "ก\u0e38ดจ\u0e31บ", "ก\u0e38มภวาป\u0e35", "ไชยวาน", "ท\u0e38\u0e48งฝน",
			"นาย\u0e39ง", "น\u0e49ำโสม", "โนนสะอาด", "บ\u0e49านด\u0e38ง", "บ\u0e49านผ\u0e37อ", "พ\u0e34บ\u0e39ลย\u0e4cร\u0e31กษ\u0e4c", "เพ\u0e47ญ", "ว\u0e31งสามหมอ", "ศร\u0e35ธาต\u0e38", "สร\u0e49างคอม",
			"หนองว\u0e31วซอ", "หนองหาน", "หนองแสง", "ประจ\u0e31กษ\u0e4cศ\u0e34ลปาคาม", "เม\u0e37องอ\u0e38ตรด\u0e34ตถ\u0e4c", "ตรอน", "ทองแสนข\u0e31น", "ท\u0e48าปลา", "น\u0e49ำปาด", "บ\u0e49านโคก",
			"พ\u0e34ช\u0e31ย", "ฟากท\u0e48า", "ล\u0e31บแล", "เม\u0e37องอ\u0e38ท\u0e31ยธาน\u0e35", "ท\u0e31พท\u0e31น", "บ\u0e49านไร\u0e48", "ลานส\u0e31ก", "สว\u0e48างอารมณ\u0e4c", "หนองขาหย\u0e48าง", "หนองฉาง",
			"ห\u0e49วยคต", "เม\u0e37องอ\u0e38บลราชธาน\u0e35", "ก\u0e38ดข\u0e49าวป\u0e38\u0e49น", "เขมราฐ", "เข\u0e37\u0e48องใน", "โขงเจ\u0e35ยม", "เดชอ\u0e38ดม", "ตระการพ\u0e37ชผล", "ตาลส\u0e38ม", "ท\u0e38\u0e48งศร\u0e35อ\u0e38ดม",
			"นาจะหลวย", "น\u0e49ำย\u0e37น", "บ\u0e38ณฑร\u0e34ก", "พ\u0e34บ\u0e39ลม\u0e31งสาหาร", "โพธ\u0e34\u0e4cไทร", "ม\u0e48วงสามส\u0e34บ", "เหล\u0e48าเส\u0e37อโก\u0e49ก", "วาร\u0e34นชำราบ", "ศร\u0e35เม\u0e37องใหม\u0e48", "สำโรง",
			"ส\u0e34ร\u0e34นธร", "นาเย\u0e35ย", "นาตาล", "สว\u0e48างว\u0e35ระวงศ\u0e4c", "น\u0e49ำข\u0e38\u0e48น", "ดอนมดแดง"
		});
		this.G1_ampore.AutoCompleteMode = System.Windows.Forms.AutoCompleteMode.SuggestAppend;
		this.G1_ampore.AutoCompleteSource = System.Windows.Forms.AutoCompleteSource.CustomSource;
		this.G1_ampore.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_ampore = this.G1_ampore;
		location = new System.Drawing.Point(292, 97);
		g1_ampore.Location = location;
		this.G1_ampore.MaxLength = 255;
		this.G1_ampore.Name = "G1_ampore";
		DevComponents.DotNetBar.Controls.TextBoxX g1_ampore2 = this.G1_ampore;
		size = new System.Drawing.Size(110, 23);
		g1_ampore2.Size = size;
		this.G1_ampore.TabIndex = 5;
		this.Label15.AutoSize = true;
		System.Windows.Forms.Label label45 = this.Label15;
		location = new System.Drawing.Point(49, 127);
		label45.Location = location;
		this.Label15.Name = "Label15";
		System.Windows.Forms.Label label46 = this.Label15;
		size = new System.Drawing.Size(43, 16);
		label46.Size = size;
		this.Label15.TabIndex = 11;
		this.Label15.Text = "จ\u0e31งหว\u0e31ด";
		this.G1_Province.AutoCompleteCustomSource.AddRange(new string[77]
		{
			"กร\u0e38งเทพมหานคร", "กระบ\u0e35\u0e48", "กาญจนบ\u0e38ร\u0e35", "กาฬส\u0e34นธ\u0e38\u0e4c", "กำแพงเพชร", "ขอนแก\u0e48น", "จ\u0e31นทบ\u0e38ร\u0e35", "ฉะเช\u0e34งเทรา", "ชลบ\u0e38ร\u0e35", "ช\u0e31ยนาท",
			"ช\u0e31ยภ\u0e39ม\u0e34", "ช\u0e38มพร", "เช\u0e35ยงราย", "เช\u0e35ยงใหม\u0e48", "ตร\u0e31ง", "ตราด", "ตาก", "นครนายก", "นครปฐม", "นครพนม",
			"นครราชส\u0e35มา", "นครศร\u0e35ธรรมราช", "นครสวรรค\u0e4c", "นนทบ\u0e38ร\u0e35", "นราธ\u0e34วาส", "น\u0e48าน", "บ\u0e36งกาฬ", "บ\u0e38ร\u0e35ร\u0e31มย\u0e4c", "ปท\u0e38มธาน\u0e35", "ประจวบค\u0e35ร\u0e35ข\u0e31นธ\u0e4c",
			"ปราจ\u0e35นบ\u0e38ร\u0e35", "ป\u0e31ตตาน\u0e35", "พระนครศร\u0e35อย\u0e38ธยา", "พะเยา", "พ\u0e31งงา", "พ\u0e31ทล\u0e38ง", "พ\u0e34จ\u0e34ตร", "พ\u0e34ษณ\u0e38โลก", "เพชรบ\u0e38ร\u0e35", "เพชรบ\u0e39รณ\u0e4c",
			"แพร\u0e48", "ภ\u0e39เก\u0e47ต", "มหาสารคาม", "ม\u0e38กดาหาร", "แม\u0e48ฮ\u0e48องสอน", "ยโสธร", "ยะลา", "ร\u0e49อยเอ\u0e47ด", "ระนอง", "ระยอง",
			"ราชบ\u0e38ร\u0e35", "ลพบ\u0e38ร\u0e35", "ลำปาง", "ลำพ\u0e39น", "เลย", "ศร\u0e35สะเกษ", "สกลนคร", "สงขลา", "สต\u0e39ล", "สม\u0e38ทรปราการ",
			"สม\u0e38ทรสงคราม", "สม\u0e38ทรสาคร", "สระแก\u0e49ว", "สระบ\u0e38ร\u0e35", "ส\u0e34งห\u0e4cบ\u0e38ร\u0e35", "ส\u0e38โขท\u0e31ย", "ส\u0e38พรรณบ\u0e38ร\u0e35", "ส\u0e38ราษฎร\u0e4cธาน\u0e35", "ส\u0e38ร\u0e34นทร\u0e4c", "หนองคาย",
			"หนองบ\u0e31วลำภ\u0e39", "อ\u0e48างทอง", "อำนาจเจร\u0e34ญ", "อ\u0e38ดรธาน\u0e35", "อ\u0e38ตรด\u0e34ตถ\u0e4c", "อ\u0e38ท\u0e31ยธาน\u0e35", "อ\u0e38บลราชธาน\u0e35"
		});
		this.G1_Province.AutoCompleteMode = System.Windows.Forms.AutoCompleteMode.SuggestAppend;
		this.G1_Province.AutoCompleteSource = System.Windows.Forms.AutoCompleteSource.CustomSource;
		this.G1_Province.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Province = this.G1_Province;
		location = new System.Drawing.Point(94, 124);
		g1_Province.Location = location;
		this.G1_Province.MaxLength = 255;
		this.G1_Province.Name = "G1_Province";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Province2 = this.G1_Province;
		size = new System.Drawing.Size(105, 23);
		g1_Province2.Size = size;
		this.G1_Province.TabIndex = 6;
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label47 = this.Label14;
		location = new System.Drawing.Point(223, 100);
		label47.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label48 = this.Label14;
		size = new System.Drawing.Size(67, 16);
		label48.Size = size;
		this.Label14.TabIndex = 11;
		this.Label14.Text = "เขต/อำเภอ";
		this.G1_Tambon.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tambon = this.G1_Tambon;
		location = new System.Drawing.Point(94, 97);
		g1_Tambon.Location = location;
		this.G1_Tambon.MaxLength = 255;
		this.G1_Tambon.Name = "G1_Tambon";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Tambon2 = this.G1_Tambon;
		size = new System.Drawing.Size(105, 23);
		g1_Tambon2.Size = size;
		this.G1_Tambon.TabIndex = 4;
		this.G1_Soi.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Soi = this.G1_Soi;
		location = new System.Drawing.Point(94, 70);
		g1_Soi.Location = location;
		this.G1_Soi.MaxLength = 255;
		this.G1_Soi.Name = "G1_Soi";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Soi2 = this.G1_Soi;
		size = new System.Drawing.Size(105, 23);
		g1_Soi2.Size = size;
		this.G1_Soi.TabIndex = 2;
		this.G1_Road.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Road = this.G1_Road;
		location = new System.Drawing.Point(292, 70);
		g1_Road.Location = location;
		this.G1_Road.MaxLength = 255;
		this.G1_Road.Name = "G1_Road";
		DevComponents.DotNetBar.Controls.TextBoxX g1_Road2 = this.G1_Road;
		size = new System.Drawing.Size(110, 23);
		g1_Road2.Size = size;
		this.G1_Road.TabIndex = 3;
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label49 = this.Label16;
		location = new System.Drawing.Point(210, 127);
		label49.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label50 = this.Label16;
		size = new System.Drawing.Size(80, 16);
		label50.Size = size;
		this.Label16.TabIndex = 11;
		this.Label16.Text = "รห\u0e31สไปรษณ\u0e35ย\u0e4c";
		this.Rsex.FormattingEnabled = true;
		this.Rsex.Items.AddRange(new object[2] { "ชาย", "หญ\u0e34ง" });
		System.Windows.Forms.ComboBox rsex = this.Rsex;
		location = new System.Drawing.Point(90, 136);
		rsex.Location = location;
		this.Rsex.Name = "Rsex";
		System.Windows.Forms.ComboBox rsex2 = this.Rsex;
		size = new System.Drawing.Size(91, 24);
		rsex2.Size = size;
		this.Rsex.TabIndex = 5;
		this.Rsex.Text = "ชาย";
		this.Rperfix.FormattingEnabled = true;
		this.Rperfix.Items.AddRange(new object[7] { "นาย", "นาง", "นางสาว", "Mr.", "Mrs.", "Miss.", "ค\u0e38ณ" });
		System.Windows.Forms.ComboBox rperfix = this.Rperfix;
		location = new System.Drawing.Point(90, 54);
		rperfix.Location = location;
		this.Rperfix.Name = "Rperfix";
		System.Windows.Forms.ComboBox rperfix2 = this.Rperfix;
		size = new System.Drawing.Size(88, 24);
		rperfix2.Size = size;
		this.Rperfix.TabIndex = 2;
		this.Rtype_Main.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Rtype_Main.FormattingEnabled = true;
		System.Windows.Forms.ComboBox rtype_Main = this.Rtype_Main;
		location = new System.Drawing.Point(90, 221);
		rtype_Main.Location = location;
		this.Rtype_Main.Name = "Rtype_Main";
		System.Windows.Forms.ComboBox rtype_Main2 = this.Rtype_Main;
		size = new System.Drawing.Size(227, 24);
		rtype_Main2.Size = size;
		this.Rtype_Main.TabIndex = 8;
		this.Label31.AutoSize = true;
		System.Windows.Forms.Label label51 = this.Label31;
		location = new System.Drawing.Point(11, 539);
		label51.Location = location;
		this.Label31.Name = "Label31";
		System.Windows.Forms.Label label52 = this.Label31;
		size = new System.Drawing.Size(92, 16);
		label52.Size = size;
		this.Label31.TabIndex = 11;
		this.Label31.Text = "ยอดเง\u0e34นส\u0e48วนเก\u0e34น";
		this.Rtype.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Rtype.FormattingEnabled = true;
		System.Windows.Forms.ComboBox rtype = this.Rtype;
		location = new System.Drawing.Point(90, 250);
		rtype.Location = location;
		this.Rtype.Name = "Rtype";
		System.Windows.Forms.ComboBox rtype2 = this.Rtype;
		size = new System.Drawing.Size(227, 24);
		rtype2.Size = size;
		this.Rtype.TabIndex = 9;
		this.Rno.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Rno.Border.Class = "TextBoxBorder";
		this.Rno.Enabled = false;
		DevComponents.DotNetBar.Controls.TextBoxX rno = this.Rno;
		location = new System.Drawing.Point(90, 28);
		rno.Location = location;
		this.Rno.MaxLength = 255;
		this.Rno.Name = "Rno";
		this.Rno.ReadOnly = true;
		DevComponents.DotNetBar.Controls.TextBoxX rno2 = this.Rno;
		size = new System.Drawing.Size(88, 23);
		rno2.Size = size;
		this.Rno.TabIndex = 0;
		this.Rno.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Rname2.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX rname = this.Rname2;
		location = new System.Drawing.Point(90, 108);
		rname.Location = location;
		this.Rname2.MaxLength = 255;
		this.Rname2.Name = "Rname2";
		DevComponents.DotNetBar.Controls.TextBoxX rname2 = this.Rname2;
		size = new System.Drawing.Size(227, 23);
		rname2.Size = size;
		this.Rname2.TabIndex = 4;
		this.Rname.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX rname3 = this.Rname;
		location = new System.Drawing.Point(90, 81);
		rname3.Location = location;
		this.Rname.MaxLength = 255;
		this.Rname.Name = "Rname";
		DevComponents.DotNetBar.Controls.TextBoxX rname4 = this.Rname;
		size = new System.Drawing.Size(227, 23);
		rname4.Size = size;
		this.Rname.TabIndex = 3;
		this.Ridcard.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX ridcard = this.Ridcard;
		location = new System.Drawing.Point(90, 165);
		ridcard.Location = location;
		this.Ridcard.MaxLength = 255;
		this.Ridcard.Name = "Ridcard";
		DevComponents.DotNetBar.Controls.TextBoxX ridcard2 = this.Ridcard;
		size = new System.Drawing.Size(227, 23);
		ridcard2.Size = size;
		this.Ridcard.TabIndex = 6;
		this.Remail.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX remail = this.Remail;
		location = new System.Drawing.Point(90, 193);
		remail.Location = location;
		this.Remail.MaxLength = 255;
		this.Remail.Name = "Remail";
		DevComponents.DotNetBar.Controls.TextBoxX remail2 = this.Remail;
		size = new System.Drawing.Size(227, 23);
		remail2.Size = size;
		this.Remail.TabIndex = 7;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label53 = this.Label6;
		location = new System.Drawing.Point(34, 111);
		label53.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label54 = this.Label6;
		size = new System.Drawing.Size(54, 16);
		label54.Size = size;
		this.Label6.TabIndex = 11;
		this.Label6.Text = "นามสก\u0e38ล";
		this.Label29.AutoSize = true;
		System.Windows.Forms.Label label55 = this.Label29;
		location = new System.Drawing.Point(60, 140);
		label55.Location = location;
		this.Label29.Name = "Label29";
		System.Windows.Forms.Label label56 = this.Label29;
		size = new System.Drawing.Size(29, 16);
		label56.Size = size;
		this.Label29.TabIndex = 11;
		this.Label29.Text = "เพศ";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label57 = this.Label1;
		location = new System.Drawing.Point(34, 84);
		label57.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label58 = this.Label1;
		size = new System.Drawing.Size(54, 16);
		label58.Size = size;
		this.Label1.TabIndex = 11;
		this.Label1.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.Label30.AutoSize = true;
		System.Windows.Forms.Label label59 = this.Label30;
		location = new System.Drawing.Point(13, 168);
		label59.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label60 = this.Label30;
		size = new System.Drawing.Size(75, 16);
		label60.Size = size;
		this.Label30.TabIndex = 11;
		this.Label30.Text = "เลขประจำต\u0e31ว";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label61 = this.Label7;
		location = new System.Drawing.Point(30, 58);
		label61.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label62 = this.Label7;
		size = new System.Drawing.Size(58, 16);
		label62.Size = size;
		this.Label7.TabIndex = 11;
		this.Label7.Text = "คำนำหน\u0e49า";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label63 = this.Label4;
		location = new System.Drawing.Point(45, 196);
		label63.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label64 = this.Label4;
		size = new System.Drawing.Size(43, 16);
		label64.Size = size;
		this.Label4.TabIndex = 11;
		this.Label4.Text = "E-Mail";
		this.Label28.AutoSize = true;
		System.Windows.Forms.Label label65 = this.Label28;
		location = new System.Drawing.Point(8, 226);
		label65.Location = location;
		this.Label28.Name = "Label28";
		System.Windows.Forms.Label label66 = this.Label28;
		size = new System.Drawing.Size(79, 16);
		label66.Size = size;
		this.Label28.TabIndex = 11;
		this.Label28.Text = "ประเภทล\u0e39กค\u0e49า";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label67 = this.Label8;
		location = new System.Drawing.Point(28, 32);
		label67.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label68 = this.Label8;
		size = new System.Drawing.Size(60, 16);
		label68.Size = size;
		this.Label8.TabIndex = 11;
		this.Label8.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label69 = this.Label5;
		location = new System.Drawing.Point(31, 255);
		label69.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label70 = this.Label5;
		size = new System.Drawing.Size(57, 16);
		label70.Size = size;
		this.Label5.TabIndex = 11;
		this.Label5.Text = "ราคาท\u0e35\u0e48ใช\u0e49";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		size = new System.Drawing.Size(966, 32);
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
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.FocusCuesEnabled = false;
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX7;
		location = new System.Drawing.Point(182, 55);
		buttonX.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX7;
		size = new System.Drawing.Size(135, 23);
		buttonX2.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 80;
		this.ButtonX7.Text = "อ\u0e48านจาก SmartCard";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(251, 533);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(122, 28);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 18;
		this.ButtonX2.Text = "ด\u0e39ประว\u0e31ต\u0e34ยอดเง\u0e34น";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX1;
		location = new System.Drawing.Point(857, 533);
		buttonX5.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX1;
		size = new System.Drawing.Size(75, 28);
		buttonX6.Size = size;
		this.ButtonX1.TabIndex = 16;
		this.ButtonX1.Text = "ลบ";
		this.Button1.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(182, 28);
		button.Location = location;
		System.Windows.Forms.Button button2 = this.Button1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button2.Margin = margin;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button3 = this.Button1;
		size = new System.Drawing.Size(37, 24);
		button3.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.UseVisualStyleBackColor = true;
		this.ButtonItem1.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem1.Checked = true;
		this.ButtonItem1.ColorTable = DevComponents.DotNetBar.eButtonColor.Blue;
		this.ButtonItem1.Image = iHOTEL2025.My.Resources.Resources.thai_car_id;
		this.ButtonItem1.ImagePaddingHorizontal = 6;
		this.ButtonItem1.ImagePaddingVertical = 7;
		this.ButtonItem1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.Text = "11/12/55\r\n46464646";
		this.ButtonItem2.ButtonStyle = DevComponents.DotNetBar.eButtonStyle.ImageAndText;
		this.ButtonItem2.Image = iHOTEL2025.My.Resources.Resources.passport_icon_29;
		this.ButtonItem2.ImagePaddingHorizontal = 6;
		this.ButtonItem2.ImagePaddingVertical = 7;
		this.ButtonItem2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		this.ButtonItem2.Name = "ButtonItem2";
		this.ButtonItem2.Text = "11/12/55\r\n46464646";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ยกเล\u0e34ก.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX_0;
		location = new System.Drawing.Point(775, 533);
		buttonX7.Location = location;
		this.ButtonX_0.Name = "ยกเล\u0e34ก";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX_0;
		size = new System.Drawing.Size(75, 28);
		buttonX8.Size = size;
		this.ButtonX_0.TabIndex = 14;
		this.ButtonX_0.Text = "เคล\u0e35ยร\u0e4c";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX_1.Image = (System.Drawing.Image)resources.GetObject("บ\u0e31นท\u0e36ก.Image");
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX_1;
		location = new System.Drawing.Point(694, 533);
		buttonX9.Location = location;
		this.ButtonX_1.Name = "บ\u0e31นท\u0e36ก";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX_1;
		size = new System.Drawing.Size(75, 28);
		buttonX10.Size = size;
		this.ButtonX_1.TabIndex = 13;
		this.ButtonX_1.Text = "บ\u0e31นท\u0e36ก";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(966, 624);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.GroupBox2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmManageCustomersNew";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "จ\u0e31ดการล\u0e39กค\u0e49า";
		this.ContextMenuStrip1.ResumeLayout(false);
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.GroupBox1.ResumeLayout(false);
		this.G2.ResumeLayout(false);
		this.G2.PerformLayout();
		this.GroupBox3.ResumeLayout(false);
		this.G1.ResumeLayout(false);
		this.G1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmManageRoom_Load(object sender, EventArgs e)
	{
		Listtype();
		ListtypeM();
		cancel();
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

	public void ShowScan(object sender, EventArgs e)
	{
		GForm0 gForm = new GForm0();
		gForm.showID = Conversions.ToInteger(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		gForm.ShowDialog();
		RefreshScan(Rno.Text);
	}

	public void DOScan(object sender, EventArgs e)
	{
		if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
		{
			FrmAddSaveImage frmAddSaveImage = new FrmAddSaveImage();
			frmAddSaveImage.Temp_no = tmp_no;
			frmAddSaveImage.Tname.Text = "";
			frmAddSaveImage.cust_no = "";
			frmAddSaveImage.ShowDialog();
			RefreshScan(Rno.Text);
		}
		else
		{
			FrmAddSaveImage frmAddSaveImage2 = new FrmAddSaveImage();
			frmAddSaveImage2.Temp_no = "";
			frmAddSaveImage2.Tname.Text = "";
			frmAddSaveImage2.cust_no = Rno.Text;
			frmAddSaveImage2.ShowDialog();
			RefreshScan(Rno.Text);
		}
	}

	public void DOScan2(object sender, EventArgs e)
	{
		FrmAddSaveImage frmAddSaveImage = new FrmAddSaveImage();
		frmAddSaveImage.Temp_no = "";
		frmAddSaveImage.Tname.Text = "";
		frmAddSaveImage.cust_no = Rno.Text;
		frmAddSaveImage.ShowDialog();
		RefreshScan(Rno.Text);
	}

	public void RefreshScan(string c_no)
	{
		ItemPanel1.BeginUpdate();
		ItemPanel1.Items.Clear();
		ButtonItem buttonItem = new ButtonItem();
		buttonItem.ButtonStyle = eButtonStyle.ImageAndText;
		buttonItem.Image = Resources._1362591067_scanner;
		buttonItem.ImagePosition = eImagePosition.Top;
		buttonItem.Name = "DoS";
		buttonItem.Text = "เพ\u0e34\u0e48มเอกสาร\r\nสแกน/ถ\u0e48ายร\u0e39ป";
		buttonItem.Click += DOScan;
		ItemPanel1.Items.Add(buttonItem);
		checked
		{
			if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
			{
				DataSet dataSet = Module1.connect("SELECT id, cin_no, ttype, cust_no, tmp_no,pic_date FROM Tb_Save_Image where tmp_no='" + tmp_no + "'  order by id desc");
				int num = dataSet.Tables[0].Rows.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						ButtonItem buttonItem2 = new ButtonItem();
						buttonItem2.ButtonStyle = eButtonStyle.ImageAndText;
						if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num2]["ttype"], "บ\u0e31ตรประชาชน", TextCompare: false))
						{
							buttonItem2.Image = Resources.thai_id;
						}
						else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num2]["ttype"], "ใบข\u0e31บข\u0e35\u0e48", TextCompare: false))
						{
							buttonItem2.Image = Resources.thai_car_id;
						}
						else if (Operators.CompareString(dataSet.Tables[0].Rows[num2]["ttype"].ToString().ToLower(), "passport", TextCompare: false) == 0)
						{
							buttonItem2.Image = Resources.passport_icon_29;
						}
						else
						{
							buttonItem2.Image = Resources.vcard;
						}
						buttonItem2.ImagePosition = eImagePosition.Top;
						buttonItem2.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["id"]);
						buttonItem2.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["ttype"], "\r\n"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["pic_date"]), "dd/MM/yy")), '\r'), '\n'));
						buttonItem2.Click += ShowScan;
						ItemPanel1.Items.Add(buttonItem2);
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				DataSet dataSet2 = Module1.connect("SELECT id, cin_no, ttype, cust_no, tmp_no,pic_date FROM Tb_Save_Image where cust_no='" + c_no + "'  order by id desc");
				int num5 = dataSet2.Tables[0].Rows.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					ButtonItem buttonItem3 = new ButtonItem();
					buttonItem3.ButtonStyle = eButtonStyle.ImageAndText;
					if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num6]["ttype"], "บ\u0e31ตรประชาชน", TextCompare: false))
					{
						buttonItem3.Image = Resources.thai_id;
					}
					else if (Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num6]["ttype"], "ใบข\u0e31บข\u0e35\u0e48", TextCompare: false))
					{
						buttonItem3.Image = Resources.thai_car_id;
					}
					else if (Operators.CompareString(dataSet2.Tables[0].Rows[num6]["ttype"].ToString().ToLower(), "passport", TextCompare: false) == 0)
					{
						buttonItem3.Image = Resources.passport_icon_29;
					}
					else
					{
						buttonItem3.Image = Resources.vcard;
					}
					buttonItem3.ImagePosition = eImagePosition.Top;
					buttonItem3.Name = Conversions.ToString(dataSet2.Tables[0].Rows[num6]["id"]);
					buttonItem3.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num6]["ttype"], "\r\n"), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num6]["pic_date"]), "dd/MM/yy")), '\r'), '\n'));
					buttonItem3.Click += ShowScan;
					ItemPanel1.Items.Add(buttonItem3);
					num6++;
				}
			}
			ItemPanel1.EndUpdate();
		}
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

	private void ButtonX_0_Click(object sender, EventArgs e)
	{
		cancel();
	}

	public void cancel()
	{
		Rperfix.Focus();
		Rno.Text = "";
		Rname.Text = "";
		Rname2.Text = "";
		Rtype.SelectedIndex = 0;
		Rtype_Main.SelectedIndex = 0;
		Remail.Text = "";
		Rsex.SelectedIndex = 0;
		Ridcard.Text = "";
		Rperfix.Text = "";
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
		Tover.Text = Conversions.ToString(0);
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
		G2_TAX.Text = "";
		Tcontry.Text = "";
		Random random = new Random();
		tmp_no = Conversions.ToString(random.Next(1, 999999));
		RunID();
		EditID = "";
		RefreshScan("");
	}

	private void ButtonX_1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Rno.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขล\u0e39กค\u0e49า");
		}
		else if (Operators.CompareString(Rname.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อล\u0e39กค\u0e49า");
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
				left = Operators.ConcatenateObject(left, ",[Cust_perfix]");
				left = Operators.ConcatenateObject(left, ",[Cust_sex]");
				left = Operators.ConcatenateObject(left, ",[Cust_IDcard]");
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
				left = Operators.ConcatenateObject(left, ",[Cust_Work_fax],[Cust_Last_Change],[Cust_Type_Main],[Cust_Work_Tax]");
				left = Operators.ConcatenateObject(left, ")");
				left = Operators.ConcatenateObject(left, " VALUES ");
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, right);
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rno.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rname.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rname2.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rperfix.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Rsex.Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Ridcard.Text, "'"));
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
				left = Operators.ConcatenateObject(left, string.Concat(",'" + G2_TAX.Text, "'"));
				left = Operators.ConcatenateObject(left, ")");
				Module1.connect(Conversions.ToString(left));
				Module1.connect("update Tb_Save_Image set cust_no='" + Rno.Text + "' ,tmp_no='' where tmp_no='" + tmp_no + "'");
			}
			else
			{
				object left2 = "UPDATE [HT_Customers] SET ";
				left2 = Operators.ConcatenateObject(left2, string.Concat(" [Cust_name]='" + Rname.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_name2]='" + Rname2.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_perfix]='" + Rperfix.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_sex]='" + Rsex.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_IDcard]='" + Ridcard.Text, "'"));
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
				left2 = Operators.ConcatenateObject(left2, string.Concat(",[Cust_Work_Tax]='" + G2_TAX.Text, "'"));
				left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(" where id=", EditID));
				Module1.connect(Conversions.ToString(left2));
			}
			cancel();
		}
	}

	private void Rtype_Main_SelectedIndexChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Order_Up where cast_type='" + Rtype_Main.Text + "' order by id");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			Rtype.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["cust_type"]);
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmManageCustomersSearch.ShowDialog();
		if (Operators.ConditionalCompareObjectNotEqual(MyProject.Forms.FrmManageCustomersSearch.EditID, "", TextCompare: false))
		{
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Customers where Cust_no='", MyProject.Forms.FrmManageCustomersSearch.EditID), "' and id<>0")));
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				EditID = dataSet.Tables[0].Rows[0]["id"].ToString();
				Rno.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_no"]);
				Rperfix.Text = dataSet.Tables[0].Rows[0]["Cust_perfix"].ToString();
				Rname.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				Rname2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				Rsex.Text = dataSet.Tables[0].Rows[0]["Cust_sex"].ToString();
				Ridcard.Text = dataSet.Tables[0].Rows[0]["Cust_IDcard"].ToString();
				Remail.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Email"]);
				Rtype_Main.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type_Main"]);
				Rtype.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				G1_No.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_no"]);
				G1_Moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_moo"]);
				G1_Soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_soi"]);
				G1_Road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_road"]);
				G1_Tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tambon"]);
				G1_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_ampore"]);
				G1_Province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_province"]);
				G1_Code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_code"]);
				G1_Tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				G1_Fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				G2_Work.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"]);
				G2_No.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_no"]);
				G2_Moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_moo"]);
				G2_Soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_soi"]);
				G2_Road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_road"]);
				G2_Tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tambon"]);
				G2_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_ampore"]);
				G2_Province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_province"]);
				G2_Code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_code"]);
				G2_Tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"]);
				G2_Fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_fax"]);
				G2_TAX.Text = dataSet.Tables[0].Rows[0]["Cust_Work_Tax"].ToString();
				Tcontry.Text = dataSet.Tables[0].Rows[0]["Cust_Contry"].ToString();
				Tover.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Price_Over"]);
				RefreshScan(Rno.Text);
			}
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการล\u0e39กค\u0e49า");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Exclamation) == DialogResult.Yes)
		{
			Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from Tb_Save_Image where cust_no in (select cust_no  from HT_Customers where id=", EditID), ")")));
			Module1.connect(Conversions.ToString(Operators.ConcatenateObject("delete from HT_Customers where id=", EditID)));
			Module1.connect("update HT_CheckIn_H set Cin_cust_no='C0000' where Cin_cust_no='" + Rno.Text + "'");
			Module1.connect("update HT_CheckIn_Pay set Cin_Cust_no='C0000' where Cin_Cust_no='" + Rno.Text + "'");
			Module1.connect("update HT_Book_H set Book_Cust_ID='C0000' where Book_Cust_ID='" + Rno.Text + "'");
			Module1.connect("update HT_Bill_Debt_H set Bill_Cust_ID='C0000' where Bill_Cust_ID='" + Rno.Text + "'");
			Module1.connect("update HT_Invoice_H set Receipt_c_no='C0000' where Receipt_c_no='" + Rno.Text + "'");
			Module1.connect("update HT_Receipt_H set Receipt_c_no='C0000' where Receipt_c_no='" + Rno.Text + "'");
			cancel();
		}
	}

	private void Rtype_SelectedIndexChanged(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Rooms_Price where Room_custtype='" + Rtype.Text + "'");
		ListView1.Items.Clear();
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
					ListView.ListViewItemCollection items = ListView1.Items;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow2 = dataRow;
					string columnName = "Room_type";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[num2].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow3 = dataRow;
					columnName = "Room_price";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmPriceHistory.CUST_NO = Rno.Text;
		MyProject.Forms.FrmPriceHistory.ShowDialog();
	}

	private void ButtonItem1_Click(object sender, EventArgs e)
	{
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		Module1.GenSmartCard();
		Process process = new Process();
		process.EnableRaisingEvents = false;
		process.StartInfo.FileName = Module1.Path_Program + "KPThaiNationalIDCard.exe";
		process.Start();
		process.WaitForExit();
		if (!File.Exists("thaiid.txt"))
		{
			return;
		}
		StreamReader streamReader = new StreamReader(Module1.PathF + "\\thaiid.txt");
		string expression = streamReader.ReadToEnd();
		streamReader.Close();
		string[] array = Strings.Split(expression, "\r\n");
		int num = 1;
		string str = "";
		string str2 = "";
		string str3 = "";
		string str4 = "";
		string str5 = "";
		string str6 = "";
		string str7 = "";
		string str8 = "";
		string str9 = "";
		string str10 = "";
		string str11 = "";
		string str12 = "";
		string[] array2 = array;
		checked
		{
			foreach (string text in array2)
			{
				if (Operators.CompareString(text, "", TextCompare: false) != 0)
				{
					switch (num)
					{
					case 1:
						str = text;
						break;
					case 2:
						str2 = text;
						break;
					case 3:
						str3 = text;
						break;
					case 4:
						str4 = text;
						break;
					case 5:
						str5 = text;
						break;
					case 10:
						str6 = text;
						break;
					case 11:
						str7 = text;
						break;
					case 12:
						str8 = text;
						break;
					case 13:
						str9 = text;
						break;
					case 14:
						str10 = text;
						break;
					case 15:
						str11 = text;
						break;
					case 16:
						str12 = text;
						break;
					}
				}
				num++;
			}
			DataSet dataSet = Module1.connect("select * from HT_Customers where (Cust_name='" + Strings.Trim(str3) + "' and Cust_name2='" + Strings.Trim(str4) + "') or Cust_IDcard='" + Strings.Trim(str) + "'");
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				MessageBox.Show("ม\u0e35ช\u0e37\u0e48อ " + Strings.Trim(str3) + " " + Strings.Trim(str4) + " อย\u0e39\u0e48ในระบบแล\u0e49ว", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
				EditID = dataSet.Tables[0].Rows[0]["id"].ToString();
				Rno.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_no"]);
				Rperfix.Text = dataSet.Tables[0].Rows[0]["Cust_perfix"].ToString();
				Rname.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				Rname2.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name2"]);
				Rsex.Text = dataSet.Tables[0].Rows[0]["Cust_sex"].ToString();
				Ridcard.Text = dataSet.Tables[0].Rows[0]["Cust_IDcard"].ToString();
				Remail.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Email"]);
				Rtype_Main.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type_Main"]);
				Rtype.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				G1_No.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_no"]);
				G1_Moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_moo"]);
				G1_Soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_soi"]);
				G1_Road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_road"]);
				G1_Tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tambon"]);
				G1_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_ampore"]);
				G1_Province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_province"]);
				G1_Code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_code"]);
				G1_Tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				G1_Fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				G2_Work.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_Name"]);
				G2_No.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_no"]);
				G2_Moo.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_moo"]);
				G2_Soi.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_soi"]);
				G2_Road.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_road"]);
				G2_Tambon.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tambon"]);
				G2_ampore.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_ampore"]);
				G2_Province.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_province"]);
				G2_Code.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_code"]);
				G2_Tel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_tel"]);
				G2_Fax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Work_fax"]);
				G2_TAX.Text = dataSet.Tables[0].Rows[0]["Cust_Work_Tax"].ToString();
				Tcontry.Text = dataSet.Tables[0].Rows[0]["Cust_Contry"].ToString();
				Tover.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Price_Over"]);
				RefreshScan(Rno.Text);
				return;
			}
			Rname.Text = Strings.Trim(str3);
			Rname2.Text = Strings.Trim(str4);
			Rperfix.Text = Strings.Trim(str2);
			Rsex.Text = Strings.Trim(str5);
			Ridcard.Text = Strings.Trim(str);
			G1_No.Text = Strings.Trim(str6);
			G1_Moo.Text = Strings.Trim(str7);
			G1_Soi.Text = Strings.Trim(str8);
			G1_Road.Text = Strings.Trim(str9);
			G1_Tambon.Text = Strings.Trim(str10);
			G1_ampore.Text = Strings.Trim(str11);
			G1_Province.Text = Strings.Trim(str12);
			if (!File.Exists(Module1.PathF + "/thaiid.png"))
			{
				return;
			}
			DataSet dataSet2 = Module1.connect("SELECT id FROM Tb_Save_Image where (cust_no='" + Rno.Text + "') and ttype='บ\u0e31ตรประชาชน' order by id desc");
			Application.DoEvents();
			if (dataSet2.Tables[0].Rows.Count == 0)
			{
				MyProject.Forms.FrmShowPreviewSmartCard.loadpic();
				MyProject.Forms.FrmShowPreviewSmartCard.Show();
				Application.DoEvents();
				FileStream fileStream = new FileStream(Module1.PathF + "/thaiid.png", FileMode.Open, FileAccess.Read);
				BinaryReader binaryReader = new BinaryReader(fileStream);
				byte[] array3 = binaryReader.ReadBytes((int)fileStream.Length);
				binaryReader.Close();
				fileStream.Close();
				int num2 = 1;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num2;
					num2++;
				}
				while (num2 <= 30);
				Application.DoEvents();
				StringBuilder stringBuilder = new StringBuilder();
				byte[] array4 = array3;
				foreach (byte b in array4)
				{
					stringBuilder.Append(b.ToString("X2"));
				}
				int num3 = 31;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num3;
					num3++;
				}
				while (num3 <= 50);
				Application.DoEvents();
				if (Operators.ConditionalCompareObjectEqual(EditID, "", TextCompare: false))
				{
					object left = "INSERT INTO [Tb_Save_Image]";
					left = Operators.ConcatenateObject(left, "([cin_no]");
					left = Operators.ConcatenateObject(left, ",[ttype]");
					left = Operators.ConcatenateObject(left, ",[pic],[cust_no],[tmp_no],[pic_date])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(''");
					left = Operators.ConcatenateObject(left, ",'บ\u0e31ตรประชาชน'");
					left = Operators.ConcatenateObject(left, ",0x" + stringBuilder.ToString());
					left = Operators.ConcatenateObject(left, ",''");
					left = Operators.ConcatenateObject(left, string.Concat(",'" + tmp_no, "'"));
					left = Operators.ConcatenateObject(left, ",getdate()");
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
				}
				else
				{
					object left2 = "INSERT INTO [Tb_Save_Image]";
					left2 = Operators.ConcatenateObject(left2, "([cin_no]");
					left2 = Operators.ConcatenateObject(left2, ",[ttype]");
					left2 = Operators.ConcatenateObject(left2, ",[pic],[cust_no],[tmp_no],[pic_date])");
					left2 = Operators.ConcatenateObject(left2, "VALUES");
					left2 = Operators.ConcatenateObject(left2, "(''");
					left2 = Operators.ConcatenateObject(left2, ",'บ\u0e31ตรประชาชน'");
					left2 = Operators.ConcatenateObject(left2, ",0x" + stringBuilder.ToString());
					left2 = Operators.ConcatenateObject(left2, string.Concat(",'" + Rno.Text, "'"));
					left2 = Operators.ConcatenateObject(left2, ",''");
					left2 = Operators.ConcatenateObject(left2, ",getdate()");
					left2 = Operators.ConcatenateObject(left2, ")");
					Module1.connect(Conversions.ToString(left2));
				}
				Application.DoEvents();
				int num4 = 51;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num4;
					num4++;
				}
				while (num4 <= 80);
				RefreshScan(Rno.Text);
				int num5 = 81;
				do
				{
					MyProject.Forms.FrmShowPreviewSmartCard.ProgressBarX1.Value = num5;
					num5++;
				}
				while (num5 <= 100);
			}
		}
	}
}
