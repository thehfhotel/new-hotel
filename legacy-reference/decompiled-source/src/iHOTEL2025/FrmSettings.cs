using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmSettings : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ลบToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

	[AccessedThroughProperty("แกไขToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_1;

	[AccessedThroughProperty("Button1")]
	private ButtonX _Button1;

	[AccessedThroughProperty("GroupBox2")]
	private GroupBox _GroupBox2;

	[AccessedThroughProperty("BPIC2")]
	private ButtonX _BPIC2;

	[AccessedThroughProperty("BPIC1")]
	private ButtonX _BPIC1;

	[AccessedThroughProperty("PanelPic")]
	private PanelEx _PanelPic;

	[AccessedThroughProperty("PictureBox1")]
	private PictureBox _PictureBox1;

	[AccessedThroughProperty("Tcompany_address")]
	private TextBox _Tcompany_address;

	[AccessedThroughProperty("Tfax")]
	private TextBox _Tfax;

	[AccessedThroughProperty("Ttel")]
	private TextBox _Ttel;

	[AccessedThroughProperty("Tcompany_name")]
	private TextBox _Tcompany_name;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("Ttax")]
	private TextBox _Ttax;

	[AccessedThroughProperty("Label18")]
	private Label _Label18;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("TCHK_IN_Before")]
	private TextBox _TCHK_IN_Before;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TCHK_Out")]
	private TextBox _TCHK_Out;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("TCHK_Out_Alert")]
	private TextBox _TCHK_Out_Alert;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("TCHK_Out_Before")]
	private TextBox _TCHK_Out_Before;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("TCHK_Out_H_price")]
	private TextBox _TCHK_Out_H_price;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("TMaximum_Book")]
	private TextBox _TMaximum_Book;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("TVat_Over")]
	private TextBox _TVat_Over;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("TPority")]
	private TextBox _TPority;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("AutoCust")]
	private ComboBox _AutoCust;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("Label17")]
	private Label _Label17;

	[AccessedThroughProperty("Label22")]
	private Label _Label22;

	[AccessedThroughProperty("Label21")]
	private Label _Label21;

	[AccessedThroughProperty("Label20")]
	private Label _Label20;

	[AccessedThroughProperty("Label19")]
	private Label _Label19;

	[AccessedThroughProperty("Label16")]
	private Label _Label16;

	[AccessedThroughProperty("Label15")]
	private Label _Label15;

	[AccessedThroughProperty("GroupBox3")]
	private GroupBox _GroupBox3;

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("Label24")]
	private Label _Label24;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label23")]
	private Label _Label23;

	[AccessedThroughProperty("ComboBox3")]
	private ComboBox _ComboBox3;

	[AccessedThroughProperty("Label25")]
	private Label _Label25;

	[AccessedThroughProperty("GroupBox4")]
	private GroupBox _GroupBox4;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Print1")]
	private TextBox _Print1;

	[AccessedThroughProperty("Label26")]
	private Label _Label26;

	[AccessedThroughProperty("Button9")]
	private Button _Button9;

	[AccessedThroughProperty("Button6")]
	private Button _Button6;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Print3")]
	private TextBox _Print3;

	[AccessedThroughProperty("Label28")]
	private Label _Label28;

	[AccessedThroughProperty("Print2")]
	private TextBox _Print2;

	[AccessedThroughProperty("Button5")]
	private Button _Button5;

	[AccessedThroughProperty("Label27")]
	private Label _Label27;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("GroupBox5")]
	private GroupBox _GroupBox5;

	[AccessedThroughProperty("ComboBox4")]
	private ComboBox _ComboBox4;

	[AccessedThroughProperty("Label29")]
	private Label _Label29;

	[AccessedThroughProperty("ComboBox6")]
	private ComboBox _ComboBox6;

	[AccessedThroughProperty("Label31")]
	private Label _Label31;

	[AccessedThroughProperty("Button8")]
	private Button _Button8;

	[AccessedThroughProperty("Print4")]
	private TextBox _Print4;

	[AccessedThroughProperty("Label33")]
	private Label _Label33;

	[AccessedThroughProperty("Button7")]
	private Button _Button7;

	[AccessedThroughProperty("GroupBox6")]
	private GroupBox _GroupBox6;

	[AccessedThroughProperty("ComboBox5")]
	private ComboBox _ComboBox5;

	[AccessedThroughProperty("Label30")]
	private Label _Label30;

	[AccessedThroughProperty("GroupBox7")]
	private GroupBox _GroupBox7;

	[AccessedThroughProperty("ComboBox7")]
	private ComboBox _ComboBox7;

	[AccessedThroughProperty("Label32")]
	private Label _Label32;

	[AccessedThroughProperty("ComboBox8")]
	private ComboBox _ComboBox8;

	[AccessedThroughProperty("Label34")]
	private Label _Label34;

	[AccessedThroughProperty("GroupBox8")]
	private GroupBox _GroupBox8;

	[AccessedThroughProperty("ComboBox9")]
	private ComboBox _ComboBox9;

	[AccessedThroughProperty("Label35")]
	private Label _Label35;

	[AccessedThroughProperty("ComboBox10")]
	private ComboBox _ComboBox10;

	[AccessedThroughProperty("Label36")]
	private Label _Label36;

	[AccessedThroughProperty("Button10")]
	private Button _Button10;

	[AccessedThroughProperty("print5")]
	private TextBox _print5;

	[AccessedThroughProperty("Label37")]
	private Label _Label37;

	[AccessedThroughProperty("Button11")]
	private Button _Button11;

	[AccessedThroughProperty("ComboBox11")]
	private ComboBox _ComboBox11;

	[AccessedThroughProperty("Label38")]
	private Label _Label38;

	[AccessedThroughProperty("GroupBox9")]
	private GroupBox _GroupBox9;

	[AccessedThroughProperty("ComboBox21")]
	private ComboBox _ComboBox21;

	[AccessedThroughProperty("Label39")]
	private Label _Label39;

	[AccessedThroughProperty("ComboBox20")]
	private ComboBox _ComboBox20;

	[AccessedThroughProperty("Label40")]
	private Label _Label40;

	[AccessedThroughProperty("Button12")]
	private Button _Button12;

	[AccessedThroughProperty("print6")]
	private TextBox _print6;

	[AccessedThroughProperty("Label41")]
	private Label _Label41;

	[AccessedThroughProperty("Button13")]
	private Button _Button13;

	[AccessedThroughProperty("Tvat_head")]
	private TextBox _Tvat_head;

	[AccessedThroughProperty("Label43")]
	private Label _Label43;

	[AccessedThroughProperty("Label42")]
	private Label _Label42;

	[AccessedThroughProperty("Tvat_per")]
	private TextBox _Tvat_per;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("ComboBox12")]
	private ComboBox _ComboBox12;

	[AccessedThroughProperty("Label44")]
	private Label _Label44;

	[AccessedThroughProperty("GroupBox10")]
	private GroupBox _GroupBox10;

	[AccessedThroughProperty("Label45")]
	private Label _Label45;

	[AccessedThroughProperty("ComboBox13")]
	private ComboBox _ComboBox13;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("TextURL")]
	private TextBox _TextURL;

	[AccessedThroughProperty("Label46")]
	private Label _Label46;

	[AccessedThroughProperty("CheckBox2")]
	private CheckBox _CheckBox2;

	[AccessedThroughProperty("GroupBox11")]
	private GroupBox _GroupBox11;

	[AccessedThroughProperty("Label48")]
	private Label _Label48;

	[AccessedThroughProperty("Label47")]
	private Label _Label47;

	[AccessedThroughProperty("MinHours")]
	private TextBox _MinHours;

	[AccessedThroughProperty("CheckBox3")]
	private CheckBox _CheckBox3;

	[AccessedThroughProperty("Label50")]
	private Label _Label50;

	[AccessedThroughProperty("comdelay")]
	private TextBox _comdelay;

	[AccessedThroughProperty("Label49")]
	private Label _Label49;

	[AccessedThroughProperty("ComboBox14")]
	private ComboBox _ComboBox14;

	[AccessedThroughProperty("Label51")]
	private Label _Label51;

	[AccessedThroughProperty("Tvat_head2")]
	private TextBox _Tvat_head2;

	[AccessedThroughProperty("Label52")]
	private Label _Label52;

	[AccessedThroughProperty("Tvat_Rows")]
	private TextBox _Tvat_Rows;

	[AccessedThroughProperty("Label54")]
	private Label _Label54;

	[AccessedThroughProperty("GroupBox12")]
	private GroupBox _GroupBox12;

	[AccessedThroughProperty("Label53")]
	private Label _Label53;

	[AccessedThroughProperty("Tclean")]
	private TextBox _Tclean;

	[AccessedThroughProperty("Label55")]
	private Label _Label55;

	[AccessedThroughProperty("CheckBoxIcon")]
	private CheckBox _CheckBoxIcon;

	[AccessedThroughProperty("Label56")]
	private Label _Label56;

	[AccessedThroughProperty("Tlogout")]
	private TextBox _Tlogout;

	[AccessedThroughProperty("Label57")]
	private Label _Label57;

	[AccessedThroughProperty("Copy6")]
	private TextBox _Copy6;

	[AccessedThroughProperty("Copy5")]
	private TextBox _Copy5;

	[AccessedThroughProperty("Copy4")]
	private TextBox _Copy4;

	[AccessedThroughProperty("Copy3")]
	private TextBox _Copy3;

	[AccessedThroughProperty("Copy2")]
	private TextBox _Copy2;

	[AccessedThroughProperty("Copy1")]
	private TextBox _Copy1;

	[AccessedThroughProperty("Label58")]
	private Label _Label58;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ComboBox15")]
	private ComboBox _ComboBox15;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("GroupBox13")]
	private GroupBox _GroupBox13;

	[AccessedThroughProperty("ComboBox17")]
	private ComboBox _ComboBox17;

	[AccessedThroughProperty("Label60")]
	private Label _Label60;

	[AccessedThroughProperty("Copy7")]
	private TextBox _Copy7;

	[AccessedThroughProperty("Button14")]
	private Button _Button14;

	[AccessedThroughProperty("print7")]
	private TextBox _print7;

	[AccessedThroughProperty("Label59")]
	private Label _Label59;

	[AccessedThroughProperty("Button15")]
	private Button _Button15;

	[AccessedThroughProperty("ComboBox16")]
	private ComboBox _ComboBox16;

	[AccessedThroughProperty("Label61")]
	private Label _Label61;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Copy8")]
	private TextBox _Copy8;

	[AccessedThroughProperty("Button17")]
	private Button _Button17;

	[AccessedThroughProperty("print8")]
	private TextBox _print8;

	[AccessedThroughProperty("Label62")]
	private Label _Label62;

	[AccessedThroughProperty("Button16")]
	private Button _Button16;

	[AccessedThroughProperty("Copy9")]
	private TextBox _Copy9;

	[AccessedThroughProperty("Button19")]
	private Button _Button19;

	[AccessedThroughProperty("print9")]
	private TextBox _print9;

	[AccessedThroughProperty("Label63")]
	private Label _Label63;

	[AccessedThroughProperty("Button18")]
	private Button _Button18;

	[AccessedThroughProperty("CheckBoxBookingNotification")]
	private CheckBox _CheckBoxBookingNotification;

	[AccessedThroughProperty("Label64")]
	private Label _Label64;

	public static Bitmap myBitmap;

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
			toolStripMenuItem_0 = value;
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
			toolStripMenuItem_1 = value;
		}
	}

	internal virtual ButtonX Button1
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

	internal virtual ButtonX BPIC2
	{
		[DebuggerNonUserCode]
		get
		{
			return _BPIC2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = BPIC2_Click;
			if (_BPIC2 != null)
			{
				_BPIC2.Click -= value2;
			}
			_BPIC2 = value;
			if (_BPIC2 != null)
			{
				_BPIC2.Click += value2;
			}
		}
	}

	internal virtual ButtonX BPIC1
	{
		[DebuggerNonUserCode]
		get
		{
			return _BPIC1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = BPIC1_Click;
			if (_BPIC1 != null)
			{
				_BPIC1.Click -= value2;
			}
			_BPIC1 = value;
			if (_BPIC1 != null)
			{
				_BPIC1.Click += value2;
			}
		}
	}

	internal virtual PanelEx PanelPic
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelPic;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelPic = value;
		}
	}

	internal virtual PictureBox PictureBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _PictureBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PictureBox1 = value;
		}
	}

	internal virtual TextBox Tcompany_address
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcompany_address;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcompany_address = value;
		}
	}

	internal virtual TextBox Tfax
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tfax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tfax = value;
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

	internal virtual TextBox Tcompany_name
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tcompany_name;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tcompany_name = value;
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

	internal virtual TextBox Ttax
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ttax = value;
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

	internal virtual TextBox TCHK_IN_Before
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCHK_IN_Before;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCHK_IN_Before = value;
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

	internal virtual TextBox TCHK_Out
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCHK_Out;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCHK_Out = value;
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

	internal virtual TextBox TCHK_Out_Alert
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCHK_Out_Alert;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCHK_Out_Alert = value;
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

	internal virtual TextBox TCHK_Out_Before
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCHK_Out_Before;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCHK_Out_Before = value;
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

	internal virtual TextBox TCHK_Out_H_price
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCHK_Out_H_price;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCHK_Out_H_price = value;
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

	internal virtual TextBox TMaximum_Book
	{
		[DebuggerNonUserCode]
		get
		{
			return _TMaximum_Book;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TMaximum_Book = value;
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

	internal virtual TextBox TVat_Over
	{
		[DebuggerNonUserCode]
		get
		{
			return _TVat_Over;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TVat_Over = value;
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

	internal virtual TextBox TPority
	{
		[DebuggerNonUserCode]
		get
		{
			return _TPority;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TPority = value;
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

	internal virtual ComboBox AutoCust
	{
		[DebuggerNonUserCode]
		get
		{
			return _AutoCust;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_AutoCust = value;
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
			_ComboBox2 = value;
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
			_ComboBox1 = value;
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

	internal virtual GroupBox GroupBox4
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox4 = value;
		}
	}

	internal virtual Button Button3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button3_Click;
			if (_Button3 != null)
			{
				_Button3.Click -= value2;
			}
			_Button3 = value;
			if (_Button3 != null)
			{
				_Button3.Click += value2;
			}
		}
	}

	internal virtual TextBox Print1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Print1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Print1 = value;
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

	internal virtual Button Button9
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button9_Click;
			if (_Button9 != null)
			{
				_Button9.Click -= value2;
			}
			_Button9 = value;
			if (_Button9 != null)
			{
				_Button9.Click += value2;
			}
		}
	}

	internal virtual Button Button6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button6_Click;
			if (_Button6 != null)
			{
				_Button6.Click -= value2;
			}
			_Button6 = value;
			if (_Button6 != null)
			{
				_Button6.Click += value2;
			}
		}
	}

	internal virtual Button Button2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button2_Click;
			if (_Button2 != null)
			{
				_Button2.Click -= value2;
			}
			_Button2 = value;
			if (_Button2 != null)
			{
				_Button2.Click += value2;
			}
		}
	}

	internal virtual TextBox Print3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Print3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Print3 = value;
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

	internal virtual TextBox Print2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Print2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Print2 = value;
		}
	}

	internal virtual Button Button5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button5_Click;
			if (_Button5 != null)
			{
				_Button5.Click -= value2;
			}
			_Button5 = value;
			if (_Button5 != null)
			{
				_Button5.Click += value2;
			}
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

	internal virtual Button Button4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button4_Click;
			if (_Button4 != null)
			{
				_Button4.Click -= value2;
			}
			_Button4 = value;
			if (_Button4 != null)
			{
				_Button4.Click += value2;
			}
		}
	}

	internal virtual GroupBox GroupBox5
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox5 = value;
		}
	}

	internal virtual ComboBox ComboBox4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox4 = value;
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

	internal virtual ComboBox ComboBox6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox6 = value;
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

	internal virtual Button Button8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button8_Click;
			if (_Button8 != null)
			{
				_Button8.Click -= value2;
			}
			_Button8 = value;
			if (_Button8 != null)
			{
				_Button8.Click += value2;
			}
		}
	}

	internal virtual TextBox Print4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Print4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Print4 = value;
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

	internal virtual Button Button7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button7_Click;
			if (_Button7 != null)
			{
				_Button7.Click -= value2;
			}
			_Button7 = value;
			if (_Button7 != null)
			{
				_Button7.Click += value2;
			}
		}
	}

	internal virtual GroupBox GroupBox6
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox6 = value;
		}
	}

	internal virtual ComboBox ComboBox5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox5 = value;
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

	internal virtual GroupBox GroupBox7
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox7 = value;
		}
	}

	internal virtual ComboBox ComboBox7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox7 = value;
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

	internal virtual ComboBox ComboBox8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox8 = value;
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

	internal virtual GroupBox GroupBox8
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox8 = value;
		}
	}

	internal virtual ComboBox ComboBox9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox9 = value;
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

	internal virtual ComboBox ComboBox_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox10 = value;
		}
	}

	internal virtual Label Label36
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label36;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label36 = value;
		}
	}

	internal virtual Button Button10
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button10_Click;
			if (_Button10 != null)
			{
				_Button10.Click -= value2;
			}
			_Button10 = value;
			if (_Button10 != null)
			{
				_Button10.Click += value2;
			}
		}
	}

	internal virtual TextBox print5
	{
		[DebuggerNonUserCode]
		get
		{
			return _print5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_print5 = value;
		}
	}

	internal virtual Label Label37
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label37;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label37 = value;
		}
	}

	internal virtual Button Button11
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button11_Click;
			if (_Button11 != null)
			{
				_Button11.Click -= value2;
			}
			_Button11 = value;
			if (_Button11 != null)
			{
				_Button11.Click += value2;
			}
		}
	}

	internal virtual ComboBox ComboBox_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox11 = value;
		}
	}

	internal virtual Label Label38
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label38;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label38 = value;
		}
	}

	internal virtual GroupBox GroupBox9
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox9 = value;
		}
	}

	internal virtual ComboBox ComboBox_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox21 = value;
		}
	}

	internal virtual Label Label39
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label39;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label39 = value;
		}
	}

	internal virtual ComboBox ComboBox_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox20 = value;
		}
	}

	internal virtual Label Label40
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label40;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label40 = value;
		}
	}

	internal virtual Button Button12
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button12_Click;
			if (_Button12 != null)
			{
				_Button12.Click -= value2;
			}
			_Button12 = value;
			if (_Button12 != null)
			{
				_Button12.Click += value2;
			}
		}
	}

	internal virtual TextBox print6
	{
		[DebuggerNonUserCode]
		get
		{
			return _print6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_print6 = value;
		}
	}

	internal virtual Label Label41
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label41;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label41 = value;
		}
	}

	internal virtual Button Button13
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button13_Click;
			if (_Button13 != null)
			{
				_Button13.Click -= value2;
			}
			_Button13 = value;
			if (_Button13 != null)
			{
				_Button13.Click += value2;
			}
		}
	}

	internal virtual TextBox Tvat_head
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tvat_head;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tvat_head = value;
		}
	}

	internal virtual Label Label43
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label43;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Label43_Click;
			if (_Label43 != null)
			{
				_Label43.Click -= value2;
			}
			_Label43 = value;
			if (_Label43 != null)
			{
				_Label43.Click += value2;
			}
		}
	}

	internal virtual Label Label42
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label42;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label42 = value;
		}
	}

	internal virtual TextBox Tvat_per
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tvat_per;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tvat_per = value;
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

	internal virtual ComboBox ComboBox_4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox12 = value;
		}
	}

	internal virtual Label Label44
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label44;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label44 = value;
		}
	}

	internal virtual GroupBox GroupBox_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox10 = value;
		}
	}

	internal virtual Label Label45
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label45;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label45 = value;
		}
	}

	internal virtual ComboBox ComboBox_5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox13_SelectedIndexChanged;
			if (_ComboBox13 != null)
			{
				_ComboBox13.SelectedIndexChanged -= value2;
			}
			_ComboBox13 = value;
			if (_ComboBox13 != null)
			{
				_ComboBox13.SelectedIndexChanged += value2;
			}
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
			EventHandler value2 = CheckBox1_CheckedChanged;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged -= value2;
			}
			_CheckBox1 = value;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged += value2;
			}
		}
	}

	internal virtual TextBox TextURL
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextURL;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextURL = value;
		}
	}

	internal virtual Label Label46
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label46;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label46 = value;
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

	internal virtual GroupBox GroupBox_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox11 = value;
		}
	}

	internal virtual Label Label48
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label48;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label48 = value;
		}
	}

	internal virtual Label Label47
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label47;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label47 = value;
		}
	}

	internal virtual TextBox MinHours
	{
		[DebuggerNonUserCode]
		get
		{
			return _MinHours;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_MinHours = value;
		}
	}

	internal virtual CheckBox CheckBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CheckBox3 = value;
		}
	}

	internal virtual Label Label50
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label50;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label50 = value;
		}
	}

	internal virtual TextBox comdelay
	{
		[DebuggerNonUserCode]
		get
		{
			return _comdelay;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_comdelay = value;
		}
	}

	internal virtual Label Label49
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label49;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label49 = value;
		}
	}

	internal virtual ComboBox ComboBox_6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox14 = value;
		}
	}

	internal virtual Label Label51
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label51;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label51 = value;
		}
	}

	internal virtual TextBox Tvat_head2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tvat_head2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tvat_head2 = value;
		}
	}

	internal virtual Label Label52
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label52;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label52 = value;
		}
	}

	internal virtual TextBox Tvat_Rows
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tvat_Rows;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tvat_Rows = value;
		}
	}

	internal virtual Label Label54
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label54;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label54 = value;
		}
	}

	internal virtual GroupBox GroupBox_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox12 = value;
		}
	}

	internal virtual Label Label53
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label53;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label53 = value;
		}
	}

	internal virtual TextBox Tclean
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tclean;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tclean = value;
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

	internal virtual CheckBox CheckBoxIcon
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBoxIcon;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CheckBoxIcon = value;
		}
	}

	internal virtual Label Label56
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label56;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label56 = value;
		}
	}

	internal virtual TextBox Tlogout
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tlogout;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tlogout = value;
		}
	}

	internal virtual Label Label57
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label57;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label57 = value;
		}
	}

	internal virtual TextBox Copy6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy6 != null)
			{
				_Copy6.LostFocus -= value2;
			}
			_Copy6 = value;
			if (_Copy6 != null)
			{
				_Copy6.LostFocus += value2;
			}
		}
	}

	internal virtual TextBox Copy5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy5 != null)
			{
				_Copy5.LostFocus -= value2;
			}
			_Copy5 = value;
			if (_Copy5 != null)
			{
				_Copy5.LostFocus += value2;
			}
		}
	}

	internal virtual TextBox Copy4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy4 != null)
			{
				_Copy4.LostFocus -= value2;
			}
			_Copy4 = value;
			if (_Copy4 != null)
			{
				_Copy4.LostFocus += value2;
			}
		}
	}

	internal virtual TextBox Copy3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy3 != null)
			{
				_Copy3.LostFocus -= value2;
			}
			_Copy3 = value;
			if (_Copy3 != null)
			{
				_Copy3.LostFocus += value2;
			}
		}
	}

	internal virtual TextBox Copy2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy2 != null)
			{
				_Copy2.LostFocus -= value2;
			}
			_Copy2 = value;
			if (_Copy2 != null)
			{
				_Copy2.LostFocus += value2;
			}
		}
	}

	internal virtual TextBox Copy1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy1 != null)
			{
				_Copy1.LostFocus -= value2;
			}
			_Copy1 = value;
			if (_Copy1 != null)
			{
				_Copy1.LostFocus += value2;
			}
		}
	}

	internal virtual Label Label58
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label58;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label58 = value;
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

	internal virtual ComboBox ComboBox_7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox15 = value;
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

	internal virtual GroupBox GroupBox_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox13 = value;
		}
	}

	internal virtual ComboBox ComboBox_8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox17_SelectedIndexChanged;
			if (_ComboBox17 != null)
			{
				_ComboBox17.SelectedIndexChanged -= value2;
			}
			_ComboBox17 = value;
			if (_ComboBox17 != null)
			{
				_ComboBox17.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label60
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label60;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label60 = value;
		}
	}

	internal virtual TextBox Copy7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Copy7 = value;
		}
	}

	internal virtual Button Button14
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button14_Click;
			if (_Button14 != null)
			{
				_Button14.Click -= value2;
			}
			_Button14 = value;
			if (_Button14 != null)
			{
				_Button14.Click += value2;
			}
		}
	}

	internal virtual TextBox print7
	{
		[DebuggerNonUserCode]
		get
		{
			return _print7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_print7 = value;
		}
	}

	internal virtual Label Label59
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label59;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label59 = value;
		}
	}

	internal virtual Button Button15
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button15_Click;
			if (_Button15 != null)
			{
				_Button15.Click -= value2;
			}
			_Button15 = value;
			if (_Button15 != null)
			{
				_Button15.Click += value2;
			}
		}
	}

	internal virtual ComboBox ComboBox_9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox16 = value;
		}
	}

	internal virtual Label Label61
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label61;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label61 = value;
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
			EventHandler value2 = Timer1_Tick;
			if (_Timer1 != null)
			{
				_Timer1.Tick -= value2;
			}
			_Timer1 = value;
			if (_Timer1 != null)
			{
				_Timer1.Tick += value2;
			}
		}
	}

	internal virtual TextBox Copy8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy8 != null)
			{
				_Copy8.LostFocus -= value2;
			}
			_Copy8 = value;
			if (_Copy8 != null)
			{
				_Copy8.LostFocus += value2;
			}
		}
	}

	internal virtual Button Button17
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button17_Click;
			if (_Button17 != null)
			{
				_Button17.Click -= value2;
			}
			_Button17 = value;
			if (_Button17 != null)
			{
				_Button17.Click += value2;
			}
		}
	}

	internal virtual TextBox print8
	{
		[DebuggerNonUserCode]
		get
		{
			return _print8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_print8 = value;
		}
	}

	internal virtual Label Label62
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label62;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label62 = value;
		}
	}

	internal virtual Button Button16
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button16_Click;
			if (_Button16 != null)
			{
				_Button16.Click -= value2;
			}
			_Button16 = value;
			if (_Button16 != null)
			{
				_Button16.Click += value2;
			}
		}
	}

	internal virtual TextBox Copy9
	{
		[DebuggerNonUserCode]
		get
		{
			return _Copy9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Copy1_LostFocus;
			if (_Copy9 != null)
			{
				_Copy9.LostFocus -= value2;
			}
			_Copy9 = value;
			if (_Copy9 != null)
			{
				_Copy9.LostFocus += value2;
			}
		}
	}

	internal virtual Button Button19
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button19_Click;
			if (_Button19 != null)
			{
				_Button19.Click -= value2;
			}
			_Button19 = value;
			if (_Button19 != null)
			{
				_Button19.Click += value2;
			}
		}
	}

	internal virtual TextBox print9
	{
		[DebuggerNonUserCode]
		get
		{
			return _print9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_print9 = value;
		}
	}

	internal virtual Label Label63
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label63;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label63 = value;
		}
	}

	internal virtual Button Button18
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button18_Click;
			if (_Button18 != null)
			{
				_Button18.Click -= value2;
			}
			_Button18 = value;
			if (_Button18 != null)
			{
				_Button18.Click += value2;
			}
		}
	}

	internal virtual CheckBox CheckBoxBookingNotification
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBoxBookingNotification;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_CheckBoxBookingNotification = value;
		}
	}

	internal virtual Label Label64
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label64;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label64 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmSettings()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmSettings()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmSettings_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
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
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripMenuItem_1 = new System.Windows.Forms.ToolStripMenuItem();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Button1 = new DevComponents.DotNetBar.ButtonX();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.Tcompany_address = new System.Windows.Forms.TextBox();
		this.Tfax = new System.Windows.Forms.TextBox();
		this.Ttel = new System.Windows.Forms.TextBox();
		this.Ttax = new System.Windows.Forms.TextBox();
		this.Tcompany_name = new System.Windows.Forms.TextBox();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label18 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.BPIC2 = new DevComponents.DotNetBar.ButtonX();
		this.BPIC1 = new DevComponents.DotNetBar.ButtonX();
		this.PanelPic = new DevComponents.DotNetBar.PanelEx();
		this.PictureBox1 = new System.Windows.Forms.PictureBox();
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ComboBox_7 = new System.Windows.Forms.ComboBox();
		this.Label58 = new System.Windows.Forms.Label();
		this.Label56 = new System.Windows.Forms.Label();
		this.Tlogout = new System.Windows.Forms.TextBox();
		this.Label57 = new System.Windows.Forms.Label();
		this.Label53 = new System.Windows.Forms.Label();
		this.Tclean = new System.Windows.Forms.TextBox();
		this.Label55 = new System.Windows.Forms.Label();
		this.Label17 = new System.Windows.Forms.Label();
		this.Label22 = new System.Windows.Forms.Label();
		this.Label21 = new System.Windows.Forms.Label();
		this.Label20 = new System.Windows.Forms.Label();
		this.Label19 = new System.Windows.Forms.Label();
		this.Label16 = new System.Windows.Forms.Label();
		this.Label15 = new System.Windows.Forms.Label();
		this.AutoCust = new System.Windows.Forms.ComboBox();
		this.Label14 = new System.Windows.Forms.Label();
		this.TVat_Over = new System.Windows.Forms.TextBox();
		this.Label7 = new System.Windows.Forms.Label();
		this.TMaximum_Book = new System.Windows.Forms.TextBox();
		this.Label6 = new System.Windows.Forms.Label();
		this.TCHK_Out_H_price = new System.Windows.Forms.TextBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.TCHK_Out_Before = new System.Windows.Forms.TextBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.TCHK_Out_Alert = new System.Windows.Forms.TextBox();
		this.Label3 = new System.Windows.Forms.Label();
		this.TCHK_Out = new System.Windows.Forms.TextBox();
		this.Label2 = new System.Windows.Forms.Label();
		this.TCHK_IN_Before = new System.Windows.Forms.TextBox();
		this.Label1 = new System.Windows.Forms.Label();
		this.GroupBox_2 = new System.Windows.Forms.GroupBox();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label8 = new System.Windows.Forms.Label();
		this.TPority = new System.Windows.Forms.TextBox();
		this.GroupBox_1 = new System.Windows.Forms.GroupBox();
		this.Label48 = new System.Windows.Forms.Label();
		this.Label47 = new System.Windows.Forms.Label();
		this.MinHours = new System.Windows.Forms.TextBox();
		this.GroupBox3 = new System.Windows.Forms.GroupBox();
		this.ComboBox3 = new System.Windows.Forms.ComboBox();
		this.Label25 = new System.Windows.Forms.Label();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.Label24 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label23 = new System.Windows.Forms.Label();
		this.GroupBox4 = new System.Windows.Forms.GroupBox();
		this.Copy7 = new System.Windows.Forms.TextBox();
		this.Button14 = new System.Windows.Forms.Button();
		this.print7 = new System.Windows.Forms.TextBox();
		this.Label59 = new System.Windows.Forms.Label();
		this.Button15 = new System.Windows.Forms.Button();
		this.Copy6 = new System.Windows.Forms.TextBox();
		this.Copy5 = new System.Windows.Forms.TextBox();
		this.Copy9 = new System.Windows.Forms.TextBox();
		this.Copy8 = new System.Windows.Forms.TextBox();
		this.Copy4 = new System.Windows.Forms.TextBox();
		this.Copy3 = new System.Windows.Forms.TextBox();
		this.Copy2 = new System.Windows.Forms.TextBox();
		this.Copy1 = new System.Windows.Forms.TextBox();
		this.Button12 = new System.Windows.Forms.Button();
		this.print6 = new System.Windows.Forms.TextBox();
		this.Label41 = new System.Windows.Forms.Label();
		this.Button13 = new System.Windows.Forms.Button();
		this.Button10 = new System.Windows.Forms.Button();
		this.print5 = new System.Windows.Forms.TextBox();
		this.Label37 = new System.Windows.Forms.Label();
		this.Button11 = new System.Windows.Forms.Button();
		this.Button8 = new System.Windows.Forms.Button();
		this.Button19 = new System.Windows.Forms.Button();
		this.Button17 = new System.Windows.Forms.Button();
		this.Button6 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.Print4 = new System.Windows.Forms.TextBox();
		this.Label33 = new System.Windows.Forms.Label();
		this.print9 = new System.Windows.Forms.TextBox();
		this.print8 = new System.Windows.Forms.TextBox();
		this.Print3 = new System.Windows.Forms.TextBox();
		this.Label63 = new System.Windows.Forms.Label();
		this.Label62 = new System.Windows.Forms.Label();
		this.Label28 = new System.Windows.Forms.Label();
		this.Button18 = new System.Windows.Forms.Button();
		this.Button7 = new System.Windows.Forms.Button();
		this.Button16 = new System.Windows.Forms.Button();
		this.Print2 = new System.Windows.Forms.TextBox();
		this.Button5 = new System.Windows.Forms.Button();
		this.Label27 = new System.Windows.Forms.Label();
		this.Button4 = new System.Windows.Forms.Button();
		this.Button3 = new System.Windows.Forms.Button();
		this.Print1 = new System.Windows.Forms.TextBox();
		this.Label26 = new System.Windows.Forms.Label();
		this.Button9 = new System.Windows.Forms.Button();
		this.GroupBox5 = new System.Windows.Forms.GroupBox();
		this.ComboBox4 = new System.Windows.Forms.ComboBox();
		this.Label29 = new System.Windows.Forms.Label();
		this.ComboBox6 = new System.Windows.Forms.ComboBox();
		this.Label31 = new System.Windows.Forms.Label();
		this.GroupBox6 = new System.Windows.Forms.GroupBox();
		this.Tvat_Rows = new System.Windows.Forms.TextBox();
		this.Label54 = new System.Windows.Forms.Label();
		this.Tvat_head2 = new System.Windows.Forms.TextBox();
		this.Label52 = new System.Windows.Forms.Label();
		this.ComboBox_6 = new System.Windows.Forms.ComboBox();
		this.Label51 = new System.Windows.Forms.Label();
		this.Tvat_head = new System.Windows.Forms.TextBox();
		this.Label43 = new System.Windows.Forms.Label();
		this.Label42 = new System.Windows.Forms.Label();
		this.Tvat_per = new System.Windows.Forms.TextBox();
		this.Label9 = new System.Windows.Forms.Label();
		this.ComboBox5 = new System.Windows.Forms.ComboBox();
		this.Label30 = new System.Windows.Forms.Label();
		this.GroupBox7 = new System.Windows.Forms.GroupBox();
		this.ComboBox_4 = new System.Windows.Forms.ComboBox();
		this.Label44 = new System.Windows.Forms.Label();
		this.ComboBox8 = new System.Windows.Forms.ComboBox();
		this.Label34 = new System.Windows.Forms.Label();
		this.ComboBox7 = new System.Windows.Forms.ComboBox();
		this.Label32 = new System.Windows.Forms.Label();
		this.GroupBox8 = new System.Windows.Forms.GroupBox();
		this.ComboBox_1 = new System.Windows.Forms.ComboBox();
		this.Label38 = new System.Windows.Forms.Label();
		this.ComboBox9 = new System.Windows.Forms.ComboBox();
		this.Label35 = new System.Windows.Forms.Label();
		this.ComboBox_0 = new System.Windows.Forms.ComboBox();
		this.Label36 = new System.Windows.Forms.Label();
		this.GroupBox9 = new System.Windows.Forms.GroupBox();
		this.ComboBox_2 = new System.Windows.Forms.ComboBox();
		this.Label39 = new System.Windows.Forms.Label();
		this.ComboBox_3 = new System.Windows.Forms.ComboBox();
		this.Label40 = new System.Windows.Forms.Label();
		this.GroupBox_0 = new System.Windows.Forms.GroupBox();
		this.Label50 = new System.Windows.Forms.Label();
		this.comdelay = new System.Windows.Forms.TextBox();
		this.Label49 = new System.Windows.Forms.Label();
		this.CheckBox3 = new System.Windows.Forms.CheckBox();
		this.CheckBox2 = new System.Windows.Forms.CheckBox();
		this.TextURL = new System.Windows.Forms.TextBox();
		this.Label46 = new System.Windows.Forms.Label();
		this.ComboBox_5 = new System.Windows.Forms.ComboBox();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Label45 = new System.Windows.Forms.Label();
		this.CheckBoxIcon = new System.Windows.Forms.CheckBox();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.CheckBoxBookingNotification = new System.Windows.Forms.CheckBox();
		this.GroupBox_3 = new System.Windows.Forms.GroupBox();
		this.ComboBox_9 = new System.Windows.Forms.ComboBox();
		this.Label61 = new System.Windows.Forms.Label();
		this.ComboBox_8 = new System.Windows.Forms.ComboBox();
		this.Label60 = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Label64 = new System.Windows.Forms.Label();
		this.ContextMenuStrip1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.PanelPic.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.PictureBox1).BeginInit();
		this.GroupBox1.SuspendLayout();
		this.GroupBox_2.SuspendLayout();
		this.GroupBox_1.SuspendLayout();
		this.GroupBox3.SuspendLayout();
		this.GroupBox4.SuspendLayout();
		this.GroupBox5.SuspendLayout();
		this.GroupBox6.SuspendLayout();
		this.GroupBox7.SuspendLayout();
		this.GroupBox8.SuspendLayout();
		this.GroupBox9.SuspendLayout();
		this.GroupBox_0.SuspendLayout();
		this.PanelEx1.SuspendLayout();
		this.GroupBox_3.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[2] { this.ToolStripMenuItem_0, this.ToolStripMenuItem_1 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		System.Drawing.Size size = new System.Drawing.Size(100, 48);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "ลบToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(99, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ลบ";
		this.ToolStripMenuItem_1.Name = "แกไขToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem2 = this.ToolStripMenuItem_1;
		size = new System.Drawing.Size(99, 22);
		toolStripMenuItem2.Size = size;
		this.ToolStripMenuItem_1.Text = "แก\u0e49ไข";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		size = new System.Drawing.Size(968, 39);
		panelEx3.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Lime;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Green;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "ต\u0e31\u0e49งค\u0e48า";
		this.Button1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 24f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX button = this.Button1;
		location = new System.Drawing.Point(740, 738);
		button.Location = location;
		DevComponents.DotNetBar.ButtonX button2 = this.Button1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button2.Margin = margin;
		this.Button1.Name = "Button1";
		DevComponents.DotNetBar.ButtonX button3 = this.Button1;
		size = new System.Drawing.Size(195, 69);
		button3.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "บ\u0e31นท\u0e36ก";
		this.GroupBox2.Controls.Add(this.Tcompany_address);
		this.GroupBox2.Controls.Add(this.Tfax);
		this.GroupBox2.Controls.Add(this.Ttel);
		this.GroupBox2.Controls.Add(this.Ttax);
		this.GroupBox2.Controls.Add(this.Tcompany_name);
		this.GroupBox2.Controls.Add(this.Label10);
		this.GroupBox2.Controls.Add(this.Label11);
		this.GroupBox2.Controls.Add(this.Label18);
		this.GroupBox2.Controls.Add(this.Label12);
		this.GroupBox2.Controls.Add(this.Label13);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox2;
		location = new System.Drawing.Point(7, 4);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox2;
		size = new System.Drawing.Size(408, 155);
		groupBox4.Size = size;
		this.GroupBox2.TabIndex = 0;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "ต\u0e31\u0e49งค\u0e48าบร\u0e34ษ\u0e31ท";
		this.Tcompany_address.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tcompany_address.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tcompany_address.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.TextBox tcompany_address = this.Tcompany_address;
		location = new System.Drawing.Point(87, 44);
		tcompany_address.Location = location;
		System.Windows.Forms.TextBox tcompany_address2 = this.Tcompany_address;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcompany_address2.Margin = margin;
		this.Tcompany_address.MaxLength = 500;
		this.Tcompany_address.Multiline = true;
		this.Tcompany_address.Name = "Tcompany_address";
		System.Windows.Forms.TextBox tcompany_address3 = this.Tcompany_address;
		size = new System.Drawing.Size(310, 57);
		tcompany_address3.Size = size;
		this.Tcompany_address.TabIndex = 1;
		this.Tfax.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tfax.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tfax.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.TextBox tfax = this.Tfax;
		location = new System.Drawing.Point(282, 103);
		tfax.Location = location;
		System.Windows.Forms.TextBox tfax2 = this.Tfax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tfax2.Margin = margin;
		this.Tfax.MaxLength = 50;
		this.Tfax.Name = "Tfax";
		System.Windows.Forms.TextBox tfax3 = this.Tfax;
		size = new System.Drawing.Size(115, 23);
		tfax3.Size = size;
		this.Tfax.TabIndex = 3;
		this.Ttel.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Ttel.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Ttel.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.TextBox ttel = this.Ttel;
		location = new System.Drawing.Point(87, 103);
		ttel.Location = location;
		System.Windows.Forms.TextBox ttel2 = this.Ttel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		ttel2.Margin = margin;
		this.Ttel.MaxLength = 50;
		this.Ttel.Name = "Ttel";
		System.Windows.Forms.TextBox ttel3 = this.Ttel;
		size = new System.Drawing.Size(115, 23);
		ttel3.Size = size;
		this.Ttel.TabIndex = 2;
		this.Ttax.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Ttax.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Ttax.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.TextBox ttax = this.Ttax;
		location = new System.Drawing.Point(162, 128);
		ttax.Location = location;
		System.Windows.Forms.TextBox ttax2 = this.Ttax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		ttax2.Margin = margin;
		this.Ttax.MaxLength = 500;
		this.Ttax.Name = "Ttax";
		System.Windows.Forms.TextBox ttax3 = this.Ttax;
		size = new System.Drawing.Size(235, 23);
		ttax3.Size = size;
		this.Ttax.TabIndex = 4;
		this.Tcompany_name.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tcompany_name.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tcompany_name.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.TextBox tcompany_name = this.Tcompany_name;
		location = new System.Drawing.Point(87, 19);
		tcompany_name.Location = location;
		System.Windows.Forms.TextBox tcompany_name2 = this.Tcompany_name;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tcompany_name2.Margin = margin;
		this.Tcompany_name.MaxLength = 500;
		this.Tcompany_name.Name = "Tcompany_name";
		System.Windows.Forms.TextBox tcompany_name3 = this.Tcompany_name;
		size = new System.Drawing.Size(310, 23);
		tcompany_name3.Size = size;
		this.Tcompany_name.TabIndex = 0;
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label = this.Label10;
		location = new System.Drawing.Point(44, 47);
		label.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label2 = this.Label10;
		size = new System.Drawing.Size(42, 16);
		label2.Size = size;
		this.Label10.TabIndex = 28;
		this.Label10.Text = "ท\u0e35\u0e48อย\u0e39\u0e48 :";
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label11;
		location = new System.Drawing.Point(212, 106);
		label3.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label4 = this.Label11;
		size = new System.Drawing.Size(67, 16);
		label4.Size = size;
		this.Label11.TabIndex = 32;
		this.Label11.Text = "เบอร\u0e4c Fax :";
		this.Label18.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label18;
		location = new System.Drawing.Point(20, 133);
		label5.Location = location;
		this.Label18.Name = "Label18";
		System.Windows.Forms.Label label6 = this.Label18;
		size = new System.Drawing.Size(136, 16);
		label6.Size = size;
		this.Label18.TabIndex = 30;
		this.Label18.Text = "เลขประจำต\u0e31วผ\u0e39\u0e49เส\u0e35ยภาษ\u0e35 :";
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label12;
		location = new System.Drawing.Point(21, 106);
		label7.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label8 = this.Label12;
		size = new System.Drawing.Size(64, 16);
		label8.Size = size;
		this.Label12.TabIndex = 31;
		this.Label12.Text = "เบอร\u0e4cโทร :";
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label13;
		location = new System.Drawing.Point(20, 23);
		label9.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label10 = this.Label13;
		size = new System.Drawing.Size(66, 16);
		label10.Size = size;
		this.Label13.TabIndex = 30;
		this.Label13.Text = "ช\u0e37\u0e48อบร\u0e34ษ\u0e31ท :";
		this.BPIC2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.BPIC2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX bPIC = this.BPIC2;
		location = new System.Drawing.Point(510, 156);
		bPIC.Location = location;
		DevComponents.DotNetBar.ButtonX bPIC2 = this.BPIC2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bPIC2.Margin = margin;
		this.BPIC2.Name = "BPIC2";
		DevComponents.DotNetBar.ButtonX bPIC3 = this.BPIC2;
		size = new System.Drawing.Size(50, 21);
		bPIC3.Size = size;
		this.BPIC2.TabIndex = 7;
		this.BPIC2.Text = "ลบร\u0e39ป";
		this.BPIC1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.BPIC1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX bPIC4 = this.BPIC1;
		location = new System.Drawing.Point(426, 156);
		bPIC4.Location = location;
		DevComponents.DotNetBar.ButtonX bPIC5 = this.BPIC1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bPIC5.Margin = margin;
		this.BPIC1.Name = "BPIC1";
		DevComponents.DotNetBar.ButtonX bPIC6 = this.BPIC1;
		size = new System.Drawing.Size(77, 21);
		bPIC6.Size = size;
		this.BPIC1.TabIndex = 6;
		this.BPIC1.Text = "เล\u0e37อกร\u0e39ป..";
		this.PanelPic.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelPic.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelPic.Controls.Add(this.PictureBox1);
		DevComponents.DotNetBar.PanelEx panelPic = this.PanelPic;
		location = new System.Drawing.Point(426, 13);
		panelPic.Location = location;
		DevComponents.DotNetBar.PanelEx panelPic2 = this.PanelPic;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelPic2.Margin = margin;
		this.PanelPic.Name = "PanelPic";
		DevComponents.DotNetBar.PanelEx panelPic3 = this.PanelPic;
		size = new System.Drawing.Size(134, 137);
		panelPic3.Size = size;
		this.PanelPic.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelPic.Style.BackColor1.Color = System.Drawing.Color.White;
		this.PanelPic.Style.BackColor2.Color = System.Drawing.Color.White;
		this.PanelPic.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelPic.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelPic.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelPic.Style.GradientAngle = 90;
		this.PanelPic.TabIndex = 5;
		this.PictureBox1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch;
		System.Windows.Forms.PictureBox pictureBox = this.PictureBox1;
		location = new System.Drawing.Point(2, 2);
		pictureBox.Location = location;
		System.Windows.Forms.PictureBox pictureBox2 = this.PictureBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		pictureBox2.Margin = margin;
		this.PictureBox1.Name = "PictureBox1";
		System.Windows.Forms.PictureBox pictureBox3 = this.PictureBox1;
		size = new System.Drawing.Size(128, 97);
		pictureBox3.Size = size;
		this.PictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
		this.PictureBox1.TabIndex = 0;
		this.PictureBox1.TabStop = false;
		this.OpenFileDialog1.FileName = "OpenFileDialog1";
		this.GroupBox1.Controls.Add(this.Label64);
		this.GroupBox1.Controls.Add(this.ButtonX2);
		this.GroupBox1.Controls.Add(this.ComboBox_7);
		this.GroupBox1.Controls.Add(this.Label58);
		this.GroupBox1.Controls.Add(this.Label56);
		this.GroupBox1.Controls.Add(this.Tlogout);
		this.GroupBox1.Controls.Add(this.Label57);
		this.GroupBox1.Controls.Add(this.Label53);
		this.GroupBox1.Controls.Add(this.Tclean);
		this.GroupBox1.Controls.Add(this.Label55);
		this.GroupBox1.Controls.Add(this.Label17);
		this.GroupBox1.Controls.Add(this.Label22);
		this.GroupBox1.Controls.Add(this.Label21);
		this.GroupBox1.Controls.Add(this.Label20);
		this.GroupBox1.Controls.Add(this.Label19);
		this.GroupBox1.Controls.Add(this.Label16);
		this.GroupBox1.Controls.Add(this.Label15);
		this.GroupBox1.Controls.Add(this.AutoCust);
		this.GroupBox1.Controls.Add(this.Label14);
		this.GroupBox1.Controls.Add(this.TVat_Over);
		this.GroupBox1.Controls.Add(this.Label7);
		this.GroupBox1.Controls.Add(this.TMaximum_Book);
		this.GroupBox1.Controls.Add(this.Label6);
		this.GroupBox1.Controls.Add(this.TCHK_Out_H_price);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.TCHK_Out_Before);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.TCHK_Out_Alert);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.TCHK_Out);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.TCHK_IN_Before);
		this.GroupBox1.Controls.Add(this.Label1);
		System.Windows.Forms.GroupBox groupBox5 = this.GroupBox1;
		location = new System.Drawing.Point(7, 163);
		groupBox5.Location = location;
		System.Windows.Forms.GroupBox groupBox6 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox6.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox7 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox7.Padding = margin;
		System.Windows.Forms.GroupBox groupBox8 = this.GroupBox1;
		size = new System.Drawing.Size(408, 306);
		groupBox8.Size = size;
		this.GroupBox1.TabIndex = 32;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ต\u0e31\u0e49งค\u0e48าอ\u0e37\u0e48นๆ";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(293, 272);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(62, 23);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 46;
		this.ButtonX2.Text = "ทดสอบ";
		this.ComboBox_7.FormattingEnabled = true;
		this.ComboBox_7.Items.AddRange(new object[1] { "COM1" });
		System.Windows.Forms.ComboBox comboBox_ = this.ComboBox_7;
		location = new System.Drawing.Point(175, 271);
		comboBox_.Location = location;
		this.ComboBox_7.Name = "ComboBox15";
		System.Windows.Forms.ComboBox comboBox_2 = this.ComboBox_7;
		size = new System.Drawing.Size(113, 24);
		comboBox_2.Size = size;
		this.ComboBox_7.TabIndex = 45;
		this.Label58.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label58;
		location = new System.Drawing.Point(19, 275);
		label11.Location = location;
		this.Label58.Name = "Label58";
		System.Windows.Forms.Label label12 = this.Label58;
		size = new System.Drawing.Size(154, 16);
		label12.Size = size;
		this.Label58.TabIndex = 44;
		this.Label58.Text = "ล\u0e34\u0e49นช\u0e31กเก\u0e47บเง\u0e34น : COM PORT";
		this.Label56.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label56;
		location = new System.Drawing.Point(362, 219);
		label13.Location = location;
		this.Label56.Name = "Label56";
		System.Windows.Forms.Label label14 = this.Label56;
		size = new System.Drawing.Size(31, 16);
		label14.Size = size;
		this.Label56.TabIndex = 43;
		this.Label56.Text = "นาท\u0e35";
		this.Tlogout.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tlogout.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tlogout.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tlogout = this.Tlogout;
		location = new System.Drawing.Point(293, 215);
		tlogout.Location = location;
		System.Windows.Forms.TextBox tlogout2 = this.Tlogout;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tlogout2.Margin = margin;
		this.Tlogout.MaxLength = 500;
		this.Tlogout.Name = "Tlogout";
		System.Windows.Forms.TextBox tlogout3 = this.Tlogout;
		size = new System.Drawing.Size(62, 23);
		tlogout3.Size = size;
		this.Tlogout.TabIndex = 41;
		this.Tlogout.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label57.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label57;
		location = new System.Drawing.Point(20, 218);
		label15.Location = location;
		this.Label57.Name = "Label57";
		System.Windows.Forms.Label label16 = this.Label57;
		size = new System.Drawing.Size(236, 16);
		label16.Size = size;
		this.Label57.TabIndex = 42;
		this.Label57.Text = "ออกจากระบบอ\u0e31ตโนม\u0e31ต\u0e34เม\u0e37\u0e48อไม\u0e48ได\u0e49ขย\u0e31บเมาส\u0e4c :";
		this.Label53.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label53;
		location = new System.Drawing.Point(362, 194);
		label17.Location = location;
		this.Label53.Name = "Label53";
		System.Windows.Forms.Label label18 = this.Label53;
		size = new System.Drawing.Size(31, 16);
		label18.Size = size;
		this.Label53.TabIndex = 40;
		this.Label53.Text = "นาท\u0e35";
		this.Tclean.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tclean.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tclean.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tclean = this.Tclean;
		location = new System.Drawing.Point(293, 190);
		tclean.Location = location;
		System.Windows.Forms.TextBox tclean2 = this.Tclean;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tclean2.Margin = margin;
		this.Tclean.MaxLength = 500;
		this.Tclean.Name = "Tclean";
		System.Windows.Forms.TextBox tclean3 = this.Tclean;
		size = new System.Drawing.Size(62, 23);
		tclean3.Size = size;
		this.Tclean.TabIndex = 38;
		this.Tclean.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label55.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label55;
		location = new System.Drawing.Point(20, 193);
		label19.Location = location;
		this.Label55.Name = "Label55";
		System.Windows.Forms.Label label20 = this.Label55;
		size = new System.Drawing.Size(159, 16);
		label20.Size = size;
		this.Label55.TabIndex = 39;
		this.Label55.Text = "เวลาทำความสะอาดห\u0e49องพ\u0e31ก :";
		this.Label17.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label17;
		location = new System.Drawing.Point(362, 69);
		label21.Location = location;
		this.Label17.Name = "Label17";
		System.Windows.Forms.Label label22 = this.Label17;
		size = new System.Drawing.Size(42, 16);
		label22.Size = size;
		this.Label17.TabIndex = 33;
		this.Label17.Text = "ช\u0e31\u0e48วโมง";
		this.Label22.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label22;
		location = new System.Drawing.Point(362, 169);
		label23.Location = location;
		this.Label22.Name = "Label22";
		System.Windows.Forms.Label label24 = this.Label22;
		size = new System.Drawing.Size(20, 16);
		label24.Size = size;
		this.Label22.TabIndex = 33;
		this.Label22.Text = "%";
		this.Label21.AutoSize = true;
		System.Windows.Forms.Label label25 = this.Label21;
		location = new System.Drawing.Point(362, 144);
		label25.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label26 = this.Label21;
		size = new System.Drawing.Size(30, 16);
		label26.Size = size;
		this.Label21.TabIndex = 33;
		this.Label21.Text = "ห\u0e49อง";
		this.Label20.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label20;
		location = new System.Drawing.Point(362, 119);
		label27.Location = location;
		this.Label20.Name = "Label20";
		System.Windows.Forms.Label label28 = this.Label20;
		size = new System.Drawing.Size(31, 16);
		label28.Size = size;
		this.Label20.TabIndex = 33;
		this.Label20.Text = "บาท";
		this.Label19.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label19;
		location = new System.Drawing.Point(362, 93);
		label29.Location = location;
		this.Label19.Name = "Label19";
		System.Windows.Forms.Label label30 = this.Label19;
		size = new System.Drawing.Size(45, 16);
		label30.Size = size;
		this.Label19.TabIndex = 33;
		this.Label19.Text = "นาฬ\u0e34กา";
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label16;
		location = new System.Drawing.Point(362, 44);
		label31.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label32 = this.Label16;
		size = new System.Drawing.Size(45, 16);
		label32.Size = size;
		this.Label16.TabIndex = 33;
		this.Label16.Text = "นาฬ\u0e34กา";
		this.Label15.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label15;
		location = new System.Drawing.Point(362, 19);
		label33.Location = location;
		this.Label15.Name = "Label15";
		System.Windows.Forms.Label label34 = this.Label15;
		size = new System.Drawing.Size(45, 16);
		label34.Size = size;
		this.Label15.TabIndex = 33;
		this.Label15.Text = "นาฬ\u0e34กา";
		this.AutoCust.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.AutoCust.FormattingEnabled = true;
		this.AutoCust.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox autoCust = this.AutoCust;
		location = new System.Drawing.Point(293, 242);
		autoCust.Location = location;
		this.AutoCust.Name = "AutoCust";
		System.Windows.Forms.ComboBox autoCust2 = this.AutoCust;
		size = new System.Drawing.Size(62, 24);
		autoCust2.Size = size;
		this.AutoCust.TabIndex = 32;
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label14;
		location = new System.Drawing.Point(20, 247);
		label35.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label36 = this.Label14;
		size = new System.Drawing.Size(197, 16);
		label36.Size = size;
		this.Label14.TabIndex = 30;
		this.Label14.Text = "ใช\u0e49งานการปร\u0e31บระด\u0e31บล\u0e39กค\u0e49าอ\u0e31ตโนม\u0e31ต\u0e34 :";
		this.TVat_Over.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TVat_Over.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TVat_Over.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tVat_Over = this.TVat_Over;
		location = new System.Drawing.Point(293, 165);
		tVat_Over.Location = location;
		System.Windows.Forms.TextBox tVat_Over2 = this.TVat_Over;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tVat_Over2.Margin = margin;
		this.TVat_Over.MaxLength = 500;
		this.TVat_Over.Name = "TVat_Over";
		System.Windows.Forms.TextBox tVat_Over3 = this.TVat_Over;
		size = new System.Drawing.Size(62, 23);
		tVat_Over3.Size = size;
		this.TVat_Over.TabIndex = 0;
		this.TVat_Over.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label7;
		location = new System.Drawing.Point(20, 169);
		label37.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label38 = this.Label7;
		size = new System.Drawing.Size(149, 16);
		label38.Size = size;
		this.Label7.TabIndex = 30;
		this.Label7.Text = "ค\u0e34ด % เม\u0e37\u0e48อออก VAT เก\u0e34น :";
		this.TMaximum_Book.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TMaximum_Book.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TMaximum_Book.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tMaximum_Book = this.TMaximum_Book;
		location = new System.Drawing.Point(293, 140);
		tMaximum_Book.Location = location;
		System.Windows.Forms.TextBox tMaximum_Book2 = this.TMaximum_Book;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tMaximum_Book2.Margin = margin;
		this.TMaximum_Book.MaxLength = 500;
		this.TMaximum_Book.Name = "TMaximum_Book";
		System.Windows.Forms.TextBox tMaximum_Book3 = this.TMaximum_Book;
		size = new System.Drawing.Size(62, 23);
		tMaximum_Book3.Size = size;
		this.TMaximum_Book.TabIndex = 0;
		this.TMaximum_Book.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label6;
		location = new System.Drawing.Point(20, 144);
		label39.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label40 = this.Label6;
		size = new System.Drawing.Size(168, 16);
		label40.Size = size;
		this.Label6.TabIndex = 30;
		this.Label6.Text = "จำนวนส\u0e39งส\u0e38ดท\u0e35\u0e48จองห\u0e49องได\u0e49/ว\u0e31น :";
		this.TCHK_Out_H_price.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TCHK_Out_H_price.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TCHK_Out_H_price.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tCHK_Out_H_price = this.TCHK_Out_H_price;
		location = new System.Drawing.Point(293, 115);
		tCHK_Out_H_price.Location = location;
		System.Windows.Forms.TextBox tCHK_Out_H_price2 = this.TCHK_Out_H_price;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCHK_Out_H_price2.Margin = margin;
		this.TCHK_Out_H_price.MaxLength = 500;
		this.TCHK_Out_H_price.Name = "TCHK_Out_H_price";
		System.Windows.Forms.TextBox tCHK_Out_H_price3 = this.TCHK_Out_H_price;
		size = new System.Drawing.Size(62, 23);
		tCHK_Out_H_price3.Size = size;
		this.TCHK_Out_H_price.TabIndex = 0;
		this.TCHK_Out_H_price.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label5;
		location = new System.Drawing.Point(20, 118);
		label41.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label42 = this.Label5;
		size = new System.Drawing.Size(199, 16);
		label42.Size = size;
		this.Label5.TabIndex = 30;
		this.Label5.Text = "Check-Out ค\u0e48าปร\u0e31บเก\u0e34นเวลา/ช\u0e31\u0e48วโมง :";
		this.TCHK_Out_Before.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TCHK_Out_Before.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TCHK_Out_Before.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tCHK_Out_Before = this.TCHK_Out_Before;
		location = new System.Drawing.Point(293, 90);
		tCHK_Out_Before.Location = location;
		System.Windows.Forms.TextBox tCHK_Out_Before2 = this.TCHK_Out_Before;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCHK_Out_Before2.Margin = margin;
		this.TCHK_Out_Before.MaxLength = 500;
		this.TCHK_Out_Before.Name = "TCHK_Out_Before";
		System.Windows.Forms.TextBox tCHK_Out_Before3 = this.TCHK_Out_Before;
		size = new System.Drawing.Size(62, 23);
		tCHK_Out_Before3.Size = size;
		this.TCHK_Out_Before.TabIndex = 0;
		this.TCHK_Out_Before.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label43 = this.Label4;
		location = new System.Drawing.Point(20, 93);
		label43.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label44 = this.Label4;
		size = new System.Drawing.Size(265, 16);
		label44.Size = size;
		this.Label4.TabIndex = 30;
		this.Label4.Text = "Check-Out ได\u0e49ถ\u0e36งเวลา ถ\u0e49าเก\u0e34นจะค\u0e34ดค\u0e48าปร\u0e31บ 1 ว\u0e31น :";
		this.TCHK_Out_Alert.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TCHK_Out_Alert.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TCHK_Out_Alert.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tCHK_Out_Alert = this.TCHK_Out_Alert;
		location = new System.Drawing.Point(293, 65);
		tCHK_Out_Alert.Location = location;
		System.Windows.Forms.TextBox tCHK_Out_Alert2 = this.TCHK_Out_Alert;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCHK_Out_Alert2.Margin = margin;
		this.TCHK_Out_Alert.MaxLength = 500;
		this.TCHK_Out_Alert.Name = "TCHK_Out_Alert";
		System.Windows.Forms.TextBox tCHK_Out_Alert3 = this.TCHK_Out_Alert;
		size = new System.Drawing.Size(62, 23);
		tCHK_Out_Alert3.Size = size;
		this.TCHK_Out_Alert.TabIndex = 0;
		this.TCHK_Out_Alert.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label45 = this.Label3;
		location = new System.Drawing.Point(20, 69);
		label45.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label46 = this.Label3;
		size = new System.Drawing.Size(201, 16);
		label46.Size = size;
		this.Label3.TabIndex = 30;
		this.Label3.Text = "แจ\u0e49งเต\u0e37อนก\u0e48อน Check-Out (ช\u0e31\u0e48วโมง) :";
		this.TCHK_Out.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TCHK_Out.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TCHK_Out.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tCHK_Out = this.TCHK_Out;
		location = new System.Drawing.Point(293, 40);
		tCHK_Out.Location = location;
		System.Windows.Forms.TextBox tCHK_Out2 = this.TCHK_Out;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCHK_Out2.Margin = margin;
		this.TCHK_Out.MaxLength = 500;
		this.TCHK_Out.Name = "TCHK_Out";
		System.Windows.Forms.TextBox tCHK_Out3 = this.TCHK_Out;
		size = new System.Drawing.Size(62, 23);
		tCHK_Out3.Size = size;
		this.TCHK_Out.TabIndex = 0;
		this.TCHK_Out.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label47 = this.Label2;
		location = new System.Drawing.Point(20, 44);
		label47.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label48 = this.Label2;
		size = new System.Drawing.Size(104, 16);
		label48.Size = size;
		this.Label2.TabIndex = 30;
		this.Label2.Text = "Check-Out เวลา :";
		this.TCHK_IN_Before.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TCHK_IN_Before.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TCHK_IN_Before.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tCHK_IN_Before = this.TCHK_IN_Before;
		location = new System.Drawing.Point(293, 15);
		tCHK_IN_Before.Location = location;
		System.Windows.Forms.TextBox tCHK_IN_Before2 = this.TCHK_IN_Before;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCHK_IN_Before2.Margin = margin;
		this.TCHK_IN_Before.MaxLength = 500;
		this.TCHK_IN_Before.Name = "TCHK_IN_Before";
		System.Windows.Forms.TextBox tCHK_IN_Before3 = this.TCHK_IN_Before;
		size = new System.Drawing.Size(62, 23);
		tCHK_IN_Before3.Size = size;
		this.TCHK_IN_Before.TabIndex = 0;
		this.TCHK_IN_Before.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label49 = this.Label1;
		location = new System.Drawing.Point(20, 19);
		label49.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label50 = this.Label1;
		size = new System.Drawing.Size(268, 16);
		label50.Size = size;
		this.Label1.TabIndex = 30;
		this.Label1.Text = "Check-In ก\u0e48อนเวลา ค\u0e34ดเป\u0e47น 1 ว\u0e31น (ภายในว\u0e31นน\u0e31\u0e49น) :";
		this.GroupBox_2.Controls.Add(this.ButtonX1);
		this.GroupBox_2.Controls.Add(this.Label8);
		this.GroupBox_2.Controls.Add(this.TPority);
		System.Windows.Forms.GroupBox groupBox_ = this.GroupBox_2;
		location = new System.Drawing.Point(426, 309);
		groupBox_.Location = location;
		this.GroupBox_2.Name = "GroupBox12";
		System.Windows.Forms.GroupBox groupBox_2 = this.GroupBox_2;
		size = new System.Drawing.Size(130, 131);
		groupBox_2.Size = size;
		this.GroupBox_2.TabIndex = 37;
		this.GroupBox_2.TabStop = false;
		this.GroupBox_2.Text = "จ\u0e31ดลำด\u0e31บห\u0e49องพ\u0e31ก";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		location = new System.Drawing.Point(11, 81);
		buttonX3.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		size = new System.Drawing.Size(100, 44);
		buttonX4.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 31;
		this.ButtonX1.Text = "จ\u0e31ดอ\u0e31นด\u0e31บเด\u0e35\u0e4bยวน\u0e35\u0e49";
		System.Windows.Forms.Label label51 = this.Label8;
		location = new System.Drawing.Point(14, 20);
		label51.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label52 = this.Label8;
		size = new System.Drawing.Size(100, 33);
		label52.Size = size;
		this.Label8.TabIndex = 30;
		this.Label8.Text = "ลำด\u0e31บการใช\u0e49งานของห\u0e49องพ\u0e31ก";
		this.Label8.TextAlign = System.Drawing.ContentAlignment.TopCenter;
		this.TPority.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TPority.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TPority.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tPority = this.TPority;
		location = new System.Drawing.Point(15, 54);
		tPority.Location = location;
		System.Windows.Forms.TextBox tPority2 = this.TPority;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tPority2.Margin = margin;
		this.TPority.MaxLength = 500;
		this.TPority.Name = "TPority";
		System.Windows.Forms.TextBox tPority3 = this.TPority;
		size = new System.Drawing.Size(93, 23);
		tPority3.Size = size;
		this.TPority.TabIndex = 0;
		this.TPority.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.GroupBox_1.Controls.Add(this.Label48);
		this.GroupBox_1.Controls.Add(this.Label47);
		this.GroupBox_1.Controls.Add(this.MinHours);
		System.Windows.Forms.GroupBox groupBox_3 = this.GroupBox_1;
		location = new System.Drawing.Point(426, 184);
		groupBox_3.Location = location;
		this.GroupBox_1.Name = "GroupBox11";
		System.Windows.Forms.GroupBox groupBox_4 = this.GroupBox_1;
		size = new System.Drawing.Size(130, 115);
		groupBox_4.Size = size;
		this.GroupBox_1.TabIndex = 36;
		this.GroupBox_1.TabStop = false;
		this.Label48.AutoSize = true;
		System.Windows.Forms.Label label53 = this.Label48;
		location = new System.Drawing.Point(39, 89);
		label53.Location = location;
		this.Label48.Name = "Label48";
		System.Windows.Forms.Label label54 = this.Label48;
		size = new System.Drawing.Size(42, 16);
		label54.Size = size;
		this.Label48.TabIndex = 36;
		this.Label48.Text = "ช\u0e31\u0e48วโมง";
		System.Windows.Forms.Label label55 = this.Label47;
		location = new System.Drawing.Point(8, 16);
		label55.Location = location;
		this.Label47.Name = "Label47";
		System.Windows.Forms.Label label56 = this.Label47;
		size = new System.Drawing.Size(108, 40);
		label56.Size = size;
		this.Label47.TabIndex = 35;
		this.Label47.Text = "จำนวนช\u0e31\u0e48วโมงข\u0e31\u0e49นต\u0e48ำ (สำหร\u0e31บช\u0e31\u0e48วคราว)";
		this.Label47.TextAlign = System.Drawing.ContentAlignment.TopCenter;
		this.MinHours.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.MinHours.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.MinHours.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox minHours = this.MinHours;
		location = new System.Drawing.Point(14, 58);
		minHours.Location = location;
		System.Windows.Forms.TextBox minHours2 = this.MinHours;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		minHours2.Margin = margin;
		this.MinHours.MaxLength = 500;
		this.MinHours.Name = "MinHours";
		System.Windows.Forms.TextBox minHours3 = this.MinHours;
		size = new System.Drawing.Size(93, 23);
		minHours3.Size = size;
		this.MinHours.TabIndex = 34;
		this.MinHours.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.GroupBox3.Controls.Add(this.ComboBox3);
		this.GroupBox3.Controls.Add(this.Label25);
		this.GroupBox3.Controls.Add(this.ComboBox2);
		this.GroupBox3.Controls.Add(this.Label24);
		this.GroupBox3.Controls.Add(this.ComboBox1);
		this.GroupBox3.Controls.Add(this.Label23);
		System.Windows.Forms.GroupBox groupBox9 = this.GroupBox3;
		location = new System.Drawing.Point(581, 4);
		groupBox9.Location = location;
		this.GroupBox3.Name = "GroupBox3";
		System.Windows.Forms.GroupBox groupBox10 = this.GroupBox3;
		size = new System.Drawing.Size(354, 90);
		groupBox10.Size = size;
		this.GroupBox3.TabIndex = 33;
		this.GroupBox3.TabStop = false;
		this.GroupBox3.Text = "ต\u0e31\u0e49งค\u0e48าใบสำค\u0e31ญร\u0e31บ";
		this.ComboBox3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox3.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox3.FormattingEnabled = true;
		this.ComboBox3.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox3;
		location = new System.Drawing.Point(194, 62);
		comboBox.Location = location;
		this.ComboBox3.Name = "ComboBox3";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox3;
		size = new System.Drawing.Size(149, 24);
		comboBox2.Size = size;
		this.ComboBox3.TabIndex = 32;
		this.Label25.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label25.AutoSize = true;
		System.Windows.Forms.Label label57 = this.Label25;
		location = new System.Drawing.Point(95, 65);
		label57.Location = location;
		this.Label25.Name = "Label25";
		System.Windows.Forms.Label label58 = this.Label25;
		size = new System.Drawing.Size(97, 16);
		label58.Size = size;
		this.Label25.TabIndex = 31;
		this.Label25.Text = "แสดงก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.ComboBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox2;
		location = new System.Drawing.Point(194, 37);
		comboBox3.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox2;
		size = new System.Drawing.Size(149, 24);
		comboBox4.Size = size;
		this.ComboBox2.TabIndex = 32;
		this.Label24.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label24.AutoSize = true;
		System.Windows.Forms.Label label59 = this.Label24;
		location = new System.Drawing.Point(97, 40);
		label59.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label60 = this.Label24;
		size = new System.Drawing.Size(95, 16);
		label60.Size = size;
		this.Label24.TabIndex = 31;
		this.Label24.Text = "พ\u0e34มพ\u0e4cเม\u0e37\u0e48อร\u0e31บเง\u0e34น :";
		this.ComboBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[4] { "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", "กระดาษต\u0e48อเน\u0e37\u0e48อง", "FOLIO" });
		System.Windows.Forms.ComboBox comboBox5 = this.ComboBox1;
		location = new System.Drawing.Point(194, 12);
		comboBox5.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox6 = this.ComboBox1;
		size = new System.Drawing.Size(149, 24);
		comboBox6.Size = size;
		this.ComboBox1.TabIndex = 32;
		this.Label23.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label23.AutoSize = true;
		System.Windows.Forms.Label label61 = this.Label23;
		location = new System.Drawing.Point(58, 16);
		label61.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label62 = this.Label23;
		size = new System.Drawing.Size(134, 16);
		label62.Size = size;
		this.Label23.TabIndex = 31;
		this.Label23.Text = "ร\u0e39ปแบบใบสำค\u0e31ญร\u0e31บเง\u0e34น :";
		this.GroupBox4.Controls.Add(this.Copy7);
		this.GroupBox4.Controls.Add(this.Button14);
		this.GroupBox4.Controls.Add(this.print7);
		this.GroupBox4.Controls.Add(this.Label59);
		this.GroupBox4.Controls.Add(this.Button15);
		this.GroupBox4.Controls.Add(this.Copy6);
		this.GroupBox4.Controls.Add(this.Copy5);
		this.GroupBox4.Controls.Add(this.Copy9);
		this.GroupBox4.Controls.Add(this.Copy8);
		this.GroupBox4.Controls.Add(this.Copy4);
		this.GroupBox4.Controls.Add(this.Copy3);
		this.GroupBox4.Controls.Add(this.Copy2);
		this.GroupBox4.Controls.Add(this.Copy1);
		this.GroupBox4.Controls.Add(this.Button12);
		this.GroupBox4.Controls.Add(this.print6);
		this.GroupBox4.Controls.Add(this.Label41);
		this.GroupBox4.Controls.Add(this.Button13);
		this.GroupBox4.Controls.Add(this.Button10);
		this.GroupBox4.Controls.Add(this.print5);
		this.GroupBox4.Controls.Add(this.Label37);
		this.GroupBox4.Controls.Add(this.Button11);
		this.GroupBox4.Controls.Add(this.Button8);
		this.GroupBox4.Controls.Add(this.Button19);
		this.GroupBox4.Controls.Add(this.Button17);
		this.GroupBox4.Controls.Add(this.Button6);
		this.GroupBox4.Controls.Add(this.Button2);
		this.GroupBox4.Controls.Add(this.Print4);
		this.GroupBox4.Controls.Add(this.Label33);
		this.GroupBox4.Controls.Add(this.print9);
		this.GroupBox4.Controls.Add(this.print8);
		this.GroupBox4.Controls.Add(this.Print3);
		this.GroupBox4.Controls.Add(this.Label63);
		this.GroupBox4.Controls.Add(this.Label62);
		this.GroupBox4.Controls.Add(this.Label28);
		this.GroupBox4.Controls.Add(this.Button18);
		this.GroupBox4.Controls.Add(this.Button7);
		this.GroupBox4.Controls.Add(this.Button16);
		this.GroupBox4.Controls.Add(this.Print2);
		this.GroupBox4.Controls.Add(this.Button5);
		this.GroupBox4.Controls.Add(this.Label27);
		this.GroupBox4.Controls.Add(this.Button4);
		this.GroupBox4.Controls.Add(this.Button3);
		this.GroupBox4.Controls.Add(this.Print1);
		this.GroupBox4.Controls.Add(this.Label26);
		this.GroupBox4.Controls.Add(this.Button9);
		System.Windows.Forms.GroupBox groupBox11 = this.GroupBox4;
		location = new System.Drawing.Point(7, 581);
		groupBox11.Location = location;
		this.GroupBox4.Name = "GroupBox4";
		System.Windows.Forms.GroupBox groupBox12 = this.GroupBox4;
		size = new System.Drawing.Size(568, 248);
		groupBox12.Size = size;
		this.GroupBox4.TabIndex = 34;
		this.GroupBox4.TabStop = false;
		this.GroupBox4.Text = "ต\u0e31\u0e49งค\u0e48าเคร\u0e37\u0e48องพ\u0e34มพ\u0e4c";
		this.Copy7.BackColor = System.Drawing.Color.Yellow;
		this.Copy7.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy7.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy7.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy = this.Copy7;
		location = new System.Drawing.Point(238, 135);
		copy.Location = location;
		System.Windows.Forms.TextBox copy2 = this.Copy7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy2.Margin = margin;
		this.Copy7.MaxLength = 500;
		this.Copy7.Name = "Copy7";
		System.Windows.Forms.TextBox copy3 = this.Copy7;
		size = new System.Drawing.Size(21, 23);
		copy3.Size = size;
		this.Copy7.TabIndex = 63;
		this.Copy7.Text = "1";
		this.Copy7.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		System.Windows.Forms.Button button4 = this.Button14;
		location = new System.Drawing.Point(260, 135);
		button4.Location = location;
		System.Windows.Forms.Button button5 = this.Button14;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button5.Margin = margin;
		this.Button14.Name = "Button14";
		System.Windows.Forms.Button button6 = this.Button14;
		size = new System.Drawing.Size(50, 23);
		button6.Size = size;
		this.Button14.TabIndex = 62;
		this.Button14.Text = "เล\u0e37อก";
		this.Button14.UseVisualStyleBackColor = true;
		this.print7.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.print7.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.print7.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.print7.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox textBox = this.print7;
		location = new System.Drawing.Point(139, 135);
		textBox.Location = location;
		System.Windows.Forms.TextBox textBox2 = this.print7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox2.Margin = margin;
		this.print7.MaxLength = 500;
		this.print7.Name = "print7";
		this.print7.ReadOnly = true;
		System.Windows.Forms.TextBox textBox3 = this.print7;
		size = new System.Drawing.Size(100, 23);
		textBox3.Size = size;
		this.print7.TabIndex = 60;
		this.Label59.AutoSize = true;
		System.Windows.Forms.Label label63 = this.Label59;
		location = new System.Drawing.Point(57, 139);
		label63.Location = location;
		this.Label59.Name = "Label59";
		System.Windows.Forms.Label label64 = this.Label59;
		size = new System.Drawing.Size(84, 16);
		label64.Size = size;
		this.Label59.TabIndex = 61;
		this.Label59.Text = "ใบเสร\u0e47จ POS :";
		System.Windows.Forms.Button button7 = this.Button15;
		location = new System.Drawing.Point(309, 135);
		button7.Location = location;
		System.Windows.Forms.Button button8 = this.Button15;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button8.Margin = margin;
		this.Button15.Name = "Button15";
		System.Windows.Forms.Button button9 = this.Button15;
		size = new System.Drawing.Size(97, 23);
		button9.Size = size;
		this.Button15.TabIndex = 59;
		this.Button15.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button15.UseVisualStyleBackColor = true;
		this.Copy6.BackColor = System.Drawing.Color.Yellow;
		this.Copy6.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy6.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy6.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy4 = this.Copy6;
		location = new System.Drawing.Point(238, 113);
		copy4.Location = location;
		System.Windows.Forms.TextBox copy5 = this.Copy6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy5.Margin = margin;
		this.Copy6.MaxLength = 500;
		this.Copy6.Name = "Copy6";
		System.Windows.Forms.TextBox copy6 = this.Copy6;
		size = new System.Drawing.Size(21, 23);
		copy6.Size = size;
		this.Copy6.TabIndex = 58;
		this.Copy6.Text = "1";
		this.Copy6.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy5.BackColor = System.Drawing.Color.Yellow;
		this.Copy5.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy5.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy7 = this.Copy5;
		location = new System.Drawing.Point(238, 92);
		copy7.Location = location;
		System.Windows.Forms.TextBox copy8 = this.Copy5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy8.Margin = margin;
		this.Copy5.MaxLength = 500;
		this.Copy5.Name = "Copy5";
		System.Windows.Forms.TextBox copy9 = this.Copy5;
		size = new System.Drawing.Size(21, 23);
		copy9.Size = size;
		this.Copy5.TabIndex = 57;
		this.Copy5.Text = "1";
		this.Copy5.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy9.BackColor = System.Drawing.Color.Yellow;
		this.Copy9.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy9.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy9.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy10 = this.Copy9;
		location = new System.Drawing.Point(238, 201);
		copy10.Location = location;
		System.Windows.Forms.TextBox copy11 = this.Copy9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy11.Margin = margin;
		this.Copy9.MaxLength = 500;
		this.Copy9.Name = "Copy9";
		System.Windows.Forms.TextBox copy12 = this.Copy9;
		size = new System.Drawing.Size(21, 23);
		copy12.Size = size;
		this.Copy9.TabIndex = 56;
		this.Copy9.Text = "1";
		this.Copy9.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy8.BackColor = System.Drawing.Color.Yellow;
		this.Copy8.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy8.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy8.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy13 = this.Copy8;
		location = new System.Drawing.Point(238, 179);
		copy13.Location = location;
		System.Windows.Forms.TextBox copy14 = this.Copy8;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy14.Margin = margin;
		this.Copy8.MaxLength = 500;
		this.Copy8.Name = "Copy8";
		System.Windows.Forms.TextBox copy15 = this.Copy8;
		size = new System.Drawing.Size(21, 23);
		copy15.Size = size;
		this.Copy8.TabIndex = 56;
		this.Copy8.Text = "1";
		this.Copy8.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy4.BackColor = System.Drawing.Color.Yellow;
		this.Copy4.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy4.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy16 = this.Copy4;
		location = new System.Drawing.Point(238, 157);
		copy16.Location = location;
		System.Windows.Forms.TextBox copy17 = this.Copy4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy17.Margin = margin;
		this.Copy4.MaxLength = 500;
		this.Copy4.Name = "Copy4";
		System.Windows.Forms.TextBox copy18 = this.Copy4;
		size = new System.Drawing.Size(21, 23);
		copy18.Size = size;
		this.Copy4.TabIndex = 56;
		this.Copy4.Text = "1";
		this.Copy4.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy3.BackColor = System.Drawing.Color.Yellow;
		this.Copy3.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy3.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy19 = this.Copy3;
		location = new System.Drawing.Point(238, 70);
		copy19.Location = location;
		System.Windows.Forms.TextBox copy20 = this.Copy3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy20.Margin = margin;
		this.Copy3.MaxLength = 500;
		this.Copy3.Name = "Copy3";
		System.Windows.Forms.TextBox copy21 = this.Copy3;
		size = new System.Drawing.Size(21, 23);
		copy21.Size = size;
		this.Copy3.TabIndex = 55;
		this.Copy3.Text = "1";
		this.Copy3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy2.BackColor = System.Drawing.Color.Yellow;
		this.Copy2.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy2.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy22 = this.Copy2;
		location = new System.Drawing.Point(238, 48);
		copy22.Location = location;
		System.Windows.Forms.TextBox copy23 = this.Copy2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy23.Margin = margin;
		this.Copy2.MaxLength = 500;
		this.Copy2.Name = "Copy2";
		System.Windows.Forms.TextBox copy24 = this.Copy2;
		size = new System.Drawing.Size(21, 23);
		copy24.Size = size;
		this.Copy2.TabIndex = 54;
		this.Copy2.Text = "1";
		this.Copy2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Copy1.BackColor = System.Drawing.Color.Yellow;
		this.Copy1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Copy1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Copy1.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox copy25 = this.Copy1;
		location = new System.Drawing.Point(238, 26);
		copy25.Location = location;
		System.Windows.Forms.TextBox copy26 = this.Copy1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		copy26.Margin = margin;
		this.Copy1.MaxLength = 500;
		this.Copy1.Name = "Copy1";
		System.Windows.Forms.TextBox copy27 = this.Copy1;
		size = new System.Drawing.Size(21, 23);
		copy27.Size = size;
		this.Copy1.TabIndex = 53;
		this.Copy1.Text = "1";
		this.Copy1.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		System.Windows.Forms.Button button10 = this.Button12;
		location = new System.Drawing.Point(260, 113);
		button10.Location = location;
		System.Windows.Forms.Button button11 = this.Button12;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button11.Margin = margin;
		this.Button12.Name = "Button12";
		System.Windows.Forms.Button button12 = this.Button12;
		size = new System.Drawing.Size(50, 23);
		button12.Size = size;
		this.Button12.TabIndex = 52;
		this.Button12.Text = "เล\u0e37อก";
		this.Button12.UseVisualStyleBackColor = true;
		this.print6.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.print6.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.print6.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.print6.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox textBox4 = this.print6;
		location = new System.Drawing.Point(139, 113);
		textBox4.Location = location;
		System.Windows.Forms.TextBox textBox5 = this.print6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox5.Margin = margin;
		this.print6.MaxLength = 500;
		this.print6.Name = "print6";
		this.print6.ReadOnly = true;
		System.Windows.Forms.TextBox textBox6 = this.print6;
		size = new System.Drawing.Size(100, 23);
		textBox6.Size = size;
		this.print6.TabIndex = 50;
		this.Label41.AutoSize = true;
		System.Windows.Forms.Label label65 = this.Label41;
		location = new System.Drawing.Point(68, 117);
		label65.Location = location;
		this.Label41.Name = "Label41";
		System.Windows.Forms.Label label66 = this.Label41;
		size = new System.Drawing.Size(73, 16);
		label66.Size = size;
		this.Label41.TabIndex = 51;
		this.Label41.Text = "ใบฝากเก\u0e47บ :";
		System.Windows.Forms.Button button13 = this.Button13;
		location = new System.Drawing.Point(309, 113);
		button13.Location = location;
		System.Windows.Forms.Button button14 = this.Button13;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button14.Margin = margin;
		this.Button13.Name = "Button13";
		System.Windows.Forms.Button button15 = this.Button13;
		size = new System.Drawing.Size(97, 23);
		button15.Size = size;
		this.Button13.TabIndex = 49;
		this.Button13.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button13.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button16 = this.Button10;
		location = new System.Drawing.Point(260, 70);
		button16.Location = location;
		System.Windows.Forms.Button button17 = this.Button10;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button17.Margin = margin;
		this.Button10.Name = "Button10";
		System.Windows.Forms.Button button18 = this.Button10;
		size = new System.Drawing.Size(50, 23);
		button18.Size = size;
		this.Button10.TabIndex = 48;
		this.Button10.Text = "เล\u0e37อก";
		this.Button10.UseVisualStyleBackColor = true;
		this.print5.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.print5.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.print5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.print5.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox textBox7 = this.print5;
		location = new System.Drawing.Point(139, 70);
		textBox7.Location = location;
		System.Windows.Forms.TextBox textBox8 = this.print5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox8.Margin = margin;
		this.print5.MaxLength = 500;
		this.print5.Name = "print5";
		this.print5.ReadOnly = true;
		System.Windows.Forms.TextBox textBox9 = this.print5;
		size = new System.Drawing.Size(100, 23);
		textBox9.Size = size;
		this.print5.TabIndex = 46;
		this.Label37.AutoSize = true;
		System.Windows.Forms.Label label67 = this.Label37;
		location = new System.Drawing.Point(60, 74);
		label67.Location = location;
		this.Label37.Name = "Label37";
		System.Windows.Forms.Label label68 = this.Label37;
		size = new System.Drawing.Size(81, 16);
		label68.Size = size;
		this.Label37.TabIndex = 47;
		this.Label37.Text = "ค\u0e39ปองอาหาร :";
		System.Windows.Forms.Button button19 = this.Button11;
		location = new System.Drawing.Point(309, 70);
		button19.Location = location;
		System.Windows.Forms.Button button20 = this.Button11;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button20.Margin = margin;
		this.Button11.Name = "Button11";
		System.Windows.Forms.Button button21 = this.Button11;
		size = new System.Drawing.Size(97, 23);
		button21.Size = size;
		this.Button11.TabIndex = 45;
		this.Button11.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button11.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button22 = this.Button8;
		location = new System.Drawing.Point(260, 92);
		button22.Location = location;
		System.Windows.Forms.Button button23 = this.Button8;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button23.Margin = margin;
		this.Button8.Name = "Button8";
		System.Windows.Forms.Button button24 = this.Button8;
		size = new System.Drawing.Size(50, 23);
		button24.Size = size;
		this.Button8.TabIndex = 44;
		this.Button8.Text = "เล\u0e37อก";
		this.Button8.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button25 = this.Button19;
		location = new System.Drawing.Point(260, 201);
		button25.Location = location;
		System.Windows.Forms.Button button26 = this.Button19;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button26.Margin = margin;
		this.Button19.Name = "Button19";
		System.Windows.Forms.Button button27 = this.Button19;
		size = new System.Drawing.Size(50, 23);
		button27.Size = size;
		this.Button19.TabIndex = 44;
		this.Button19.Text = "เล\u0e37อก";
		this.Button19.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button28 = this.Button17;
		location = new System.Drawing.Point(260, 179);
		button28.Location = location;
		System.Windows.Forms.Button button29 = this.Button17;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button29.Margin = margin;
		this.Button17.Name = "Button17";
		System.Windows.Forms.Button button30 = this.Button17;
		size = new System.Drawing.Size(50, 23);
		button30.Size = size;
		this.Button17.TabIndex = 44;
		this.Button17.Text = "เล\u0e37อก";
		this.Button17.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button31 = this.Button6;
		location = new System.Drawing.Point(260, 157);
		button31.Location = location;
		System.Windows.Forms.Button button32 = this.Button6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button32.Margin = margin;
		this.Button6.Name = "Button6";
		System.Windows.Forms.Button button33 = this.Button6;
		size = new System.Drawing.Size(50, 23);
		button33.Size = size;
		this.Button6.TabIndex = 44;
		this.Button6.Text = "เล\u0e37อก";
		this.Button6.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button34 = this.Button2;
		location = new System.Drawing.Point(260, 48);
		button34.Location = location;
		System.Windows.Forms.Button button35 = this.Button2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button35.Margin = margin;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button36 = this.Button2;
		size = new System.Drawing.Size(50, 23);
		button36.Size = size;
		this.Button2.TabIndex = 44;
		this.Button2.Text = "เล\u0e37อก";
		this.Button2.UseVisualStyleBackColor = true;
		this.Print4.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Print4.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Print4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Print4.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox print = this.Print4;
		location = new System.Drawing.Point(139, 92);
		print.Location = location;
		System.Windows.Forms.TextBox print2 = this.Print4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		print2.Margin = margin;
		this.Print4.MaxLength = 500;
		this.Print4.Name = "Print4";
		this.Print4.ReadOnly = true;
		System.Windows.Forms.TextBox print3 = this.Print4;
		size = new System.Drawing.Size(100, 23);
		print3.Size = size;
		this.Print4.TabIndex = 42;
		this.Label33.AutoSize = true;
		System.Windows.Forms.Label label69 = this.Label33;
		location = new System.Drawing.Point(53, 96);
		label69.Location = location;
		this.Label33.Name = "Label33";
		System.Windows.Forms.Label label70 = this.Label33;
		size = new System.Drawing.Size(88, 16);
		label70.Size = size;
		this.Label33.TabIndex = 43;
		this.Label33.Text = "ใบลงทะเบ\u0e35ยน :";
		this.print9.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.print9.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.print9.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.print9.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox textBox10 = this.print9;
		location = new System.Drawing.Point(139, 201);
		textBox10.Location = location;
		System.Windows.Forms.TextBox textBox11 = this.print9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox11.Margin = margin;
		this.print9.MaxLength = 500;
		this.print9.Name = "print9";
		this.print9.ReadOnly = true;
		System.Windows.Forms.TextBox textBox12 = this.print9;
		size = new System.Drawing.Size(100, 23);
		textBox12.Size = size;
		this.print9.TabIndex = 42;
		this.print8.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.print8.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.print8.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.print8.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox textBox13 = this.print8;
		location = new System.Drawing.Point(139, 179);
		textBox13.Location = location;
		System.Windows.Forms.TextBox textBox14 = this.print8;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox14.Margin = margin;
		this.print8.MaxLength = 500;
		this.print8.Name = "print8";
		this.print8.ReadOnly = true;
		System.Windows.Forms.TextBox textBox15 = this.print8;
		size = new System.Drawing.Size(100, 23);
		textBox15.Size = size;
		this.print8.TabIndex = 42;
		this.Print3.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Print3.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Print3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Print3.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox print4 = this.Print3;
		location = new System.Drawing.Point(139, 157);
		print4.Location = location;
		System.Windows.Forms.TextBox print5 = this.Print3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		print5.Margin = margin;
		this.Print3.MaxLength = 500;
		this.Print3.Name = "Print3";
		this.Print3.ReadOnly = true;
		System.Windows.Forms.TextBox print6 = this.Print3;
		size = new System.Drawing.Size(100, 23);
		print6.Size = size;
		this.Print3.TabIndex = 42;
		this.Label63.AutoSize = true;
		System.Windows.Forms.Label label71 = this.Label63;
		location = new System.Drawing.Point(74, 204);
		label71.Location = location;
		this.Label63.Name = "Label63";
		System.Windows.Forms.Label label72 = this.Label63;
		size = new System.Drawing.Size(67, 16);
		label72.Size = size;
		this.Label63.TabIndex = 43;
		this.Label63.Text = "บ\u0e34ลเง\u0e34นสด :";
		this.Label62.AutoSize = true;
		System.Windows.Forms.Label label73 = this.Label62;
		location = new System.Drawing.Point(12, 182);
		label73.Location = location;
		this.Label62.Name = "Label62";
		System.Windows.Forms.Label label74 = this.Label62;
		size = new System.Drawing.Size(129, 16);
		label74.Size = size;
		this.Label62.TabIndex = 43;
		this.Label62.Text = "ใบกำก\u0e31บภาษ\u0e35อย\u0e48างย\u0e48อ :";
		this.Label28.AutoSize = true;
		System.Windows.Forms.Label label75 = this.Label28;
		location = new System.Drawing.Point(56, 162);
		label75.Location = location;
		this.Label28.Name = "Label28";
		System.Windows.Forms.Label label76 = this.Label28;
		size = new System.Drawing.Size(85, 16);
		label76.Size = size;
		this.Label28.TabIndex = 43;
		this.Label28.Text = "ใบกำก\u0e31บภาษ\u0e35 :";
		System.Windows.Forms.Button button37 = this.Button18;
		location = new System.Drawing.Point(309, 201);
		button37.Location = location;
		System.Windows.Forms.Button button38 = this.Button18;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button38.Margin = margin;
		this.Button18.Name = "Button18";
		System.Windows.Forms.Button button39 = this.Button18;
		size = new System.Drawing.Size(97, 23);
		button39.Size = size;
		this.Button18.TabIndex = 41;
		this.Button18.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button18.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button40 = this.Button7;
		location = new System.Drawing.Point(309, 92);
		button40.Location = location;
		System.Windows.Forms.Button button41 = this.Button7;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button41.Margin = margin;
		this.Button7.Name = "Button7";
		System.Windows.Forms.Button button42 = this.Button7;
		size = new System.Drawing.Size(97, 23);
		button42.Size = size;
		this.Button7.TabIndex = 41;
		this.Button7.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button7.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button43 = this.Button16;
		location = new System.Drawing.Point(309, 179);
		button43.Location = location;
		System.Windows.Forms.Button button44 = this.Button16;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button44.Margin = margin;
		this.Button16.Name = "Button16";
		System.Windows.Forms.Button button45 = this.Button16;
		size = new System.Drawing.Size(97, 23);
		button45.Size = size;
		this.Button16.TabIndex = 41;
		this.Button16.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button16.UseVisualStyleBackColor = true;
		this.Print2.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Print2.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Print2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Print2.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox print7 = this.Print2;
		location = new System.Drawing.Point(139, 48);
		print7.Location = location;
		System.Windows.Forms.TextBox print8 = this.Print2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		print8.Margin = margin;
		this.Print2.MaxLength = 500;
		this.Print2.Name = "Print2";
		this.Print2.ReadOnly = true;
		System.Windows.Forms.TextBox print9 = this.Print2;
		size = new System.Drawing.Size(100, 23);
		print9.Size = size;
		this.Print2.TabIndex = 42;
		System.Windows.Forms.Button button46 = this.Button5;
		location = new System.Drawing.Point(309, 157);
		button46.Location = location;
		System.Windows.Forms.Button button47 = this.Button5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button47.Margin = margin;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button48 = this.Button5;
		size = new System.Drawing.Size(97, 23);
		button48.Size = size;
		this.Button5.TabIndex = 41;
		this.Button5.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button5.UseVisualStyleBackColor = true;
		this.Label27.AutoSize = true;
		System.Windows.Forms.Label label77 = this.Label27;
		location = new System.Drawing.Point(81, 53);
		label77.Location = location;
		this.Label27.Name = "Label27";
		System.Windows.Forms.Label label78 = this.Label27;
		size = new System.Drawing.Size(60, 16);
		label78.Size = size;
		this.Label27.TabIndex = 43;
		this.Label27.Text = "ใบม\u0e31ดจำ :";
		System.Windows.Forms.Button button49 = this.Button4;
		location = new System.Drawing.Point(309, 48);
		button49.Location = location;
		System.Windows.Forms.Button button50 = this.Button4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button50.Margin = margin;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button51 = this.Button4;
		size = new System.Drawing.Size(97, 23);
		button51.Size = size;
		this.Button4.TabIndex = 41;
		this.Button4.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button4.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button52 = this.Button3;
		location = new System.Drawing.Point(260, 26);
		button52.Location = location;
		System.Windows.Forms.Button button53 = this.Button3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button53.Margin = margin;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button54 = this.Button3;
		size = new System.Drawing.Size(50, 23);
		button54.Size = size;
		this.Button3.TabIndex = 40;
		this.Button3.Text = "เล\u0e37อก";
		this.Button3.UseVisualStyleBackColor = true;
		this.Print1.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Print1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Print1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Print1.ForeColor = System.Drawing.Color.MidnightBlue;
		System.Windows.Forms.TextBox print10 = this.Print1;
		location = new System.Drawing.Point(139, 26);
		print10.Location = location;
		System.Windows.Forms.TextBox print11 = this.Print1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		print11.Margin = margin;
		this.Print1.MaxLength = 500;
		this.Print1.Name = "Print1";
		this.Print1.ReadOnly = true;
		System.Windows.Forms.TextBox print12 = this.Print1;
		size = new System.Drawing.Size(100, 23);
		print12.Size = size;
		this.Print1.TabIndex = 38;
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label79 = this.Label26;
		location = new System.Drawing.Point(63, 30);
		label79.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label80 = this.Label26;
		size = new System.Drawing.Size(78, 16);
		label80.Size = size;
		this.Label26.TabIndex = 39;
		this.Label26.Text = "ใบสำค\u0e31ญร\u0e31บ :";
		System.Windows.Forms.Button button55 = this.Button9;
		location = new System.Drawing.Point(309, 26);
		button55.Location = location;
		System.Windows.Forms.Button button56 = this.Button9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		button56.Margin = margin;
		this.Button9.Name = "Button9";
		System.Windows.Forms.Button button57 = this.Button9;
		size = new System.Drawing.Size(97, 23);
		button57.Size = size;
		this.Button9.TabIndex = 37;
		this.Button9.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		this.Button9.UseVisualStyleBackColor = true;
		this.GroupBox5.Controls.Add(this.ComboBox4);
		this.GroupBox5.Controls.Add(this.Label29);
		this.GroupBox5.Controls.Add(this.ComboBox6);
		this.GroupBox5.Controls.Add(this.Label31);
		System.Windows.Forms.GroupBox groupBox13 = this.GroupBox5;
		location = new System.Drawing.Point(581, 94);
		groupBox13.Location = location;
		this.GroupBox5.Name = "GroupBox5";
		System.Windows.Forms.GroupBox groupBox14 = this.GroupBox5;
		size = new System.Drawing.Size(354, 64);
		groupBox14.Size = size;
		this.GroupBox5.TabIndex = 35;
		this.GroupBox5.TabStop = false;
		this.GroupBox5.Text = "ต\u0e31\u0e49งค\u0e48าใบม\u0e31ดจำ";
		this.ComboBox4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox4.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox4.FormattingEnabled = true;
		this.ComboBox4.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox7 = this.ComboBox4;
		location = new System.Drawing.Point(194, 36);
		comboBox7.Location = location;
		this.ComboBox4.Name = "ComboBox4";
		System.Windows.Forms.ComboBox comboBox8 = this.ComboBox4;
		size = new System.Drawing.Size(149, 24);
		comboBox8.Size = size;
		this.ComboBox4.TabIndex = 32;
		this.Label29.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label29.AutoSize = true;
		System.Windows.Forms.Label label81 = this.Label29;
		location = new System.Drawing.Point(52, 40);
		label81.Location = location;
		this.Label29.Name = "Label29";
		System.Windows.Forms.Label label82 = this.Label29;
		size = new System.Drawing.Size(140, 16);
		label82.Size = size;
		this.Label29.TabIndex = 31;
		this.Label29.Text = "แสดงใบม\u0e31ดจำก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.ComboBox6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox6.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox6.FormattingEnabled = true;
		this.ComboBox6.Items.AddRange(new object[3] { "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", "กระดาษต\u0e48อเน\u0e37\u0e48อง" });
		System.Windows.Forms.ComboBox comboBox9 = this.ComboBox6;
		location = new System.Drawing.Point(194, 11);
		comboBox9.Location = location;
		this.ComboBox6.Name = "ComboBox6";
		System.Windows.Forms.ComboBox comboBox10 = this.ComboBox6;
		size = new System.Drawing.Size(149, 24);
		comboBox10.Size = size;
		this.ComboBox6.TabIndex = 32;
		this.Label31.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label31.AutoSize = true;
		System.Windows.Forms.Label label83 = this.Label31;
		location = new System.Drawing.Point(94, 15);
		label83.Location = location;
		this.Label31.Name = "Label31";
		System.Windows.Forms.Label label84 = this.Label31;
		size = new System.Drawing.Size(98, 16);
		label84.Size = size;
		this.Label31.TabIndex = 31;
		this.Label31.Text = "ร\u0e39ปแบบใบม\u0e31ดจำ :";
		this.GroupBox6.Controls.Add(this.Tvat_Rows);
		this.GroupBox6.Controls.Add(this.Label54);
		this.GroupBox6.Controls.Add(this.Tvat_head2);
		this.GroupBox6.Controls.Add(this.Label52);
		this.GroupBox6.Controls.Add(this.ComboBox_6);
		this.GroupBox6.Controls.Add(this.Label51);
		this.GroupBox6.Controls.Add(this.Tvat_head);
		this.GroupBox6.Controls.Add(this.Label43);
		this.GroupBox6.Controls.Add(this.Label42);
		this.GroupBox6.Controls.Add(this.Tvat_per);
		this.GroupBox6.Controls.Add(this.Label9);
		this.GroupBox6.Controls.Add(this.ComboBox5);
		this.GroupBox6.Controls.Add(this.Label30);
		System.Windows.Forms.GroupBox groupBox15 = this.GroupBox6;
		location = new System.Drawing.Point(581, 160);
		groupBox15.Location = location;
		this.GroupBox6.Name = "GroupBox6";
		System.Windows.Forms.GroupBox groupBox16 = this.GroupBox6;
		size = new System.Drawing.Size(354, 129);
		groupBox16.Size = size;
		this.GroupBox6.TabIndex = 36;
		this.GroupBox6.TabStop = false;
		this.GroupBox6.Text = "ต\u0e31\u0e49งค\u0e48าใบกำก\u0e31บภาษ\u0e35";
		this.Tvat_Rows.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tvat_Rows.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tvat_Rows.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tvat_Rows = this.Tvat_Rows;
		location = new System.Drawing.Point(50, 41);
		tvat_Rows.Location = location;
		System.Windows.Forms.TextBox tvat_Rows2 = this.Tvat_Rows;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tvat_Rows2.Margin = margin;
		this.Tvat_Rows.MaxLength = 500;
		this.Tvat_Rows.Name = "Tvat_Rows";
		System.Windows.Forms.TextBox tvat_Rows3 = this.Tvat_Rows;
		size = new System.Drawing.Size(26, 23);
		tvat_Rows3.Size = size;
		this.Tvat_Rows.TabIndex = 43;
		this.Tvat_Rows.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label54.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label54.AutoSize = true;
		System.Windows.Forms.Label label85 = this.Label54;
		location = new System.Drawing.Point(1, 45);
		label85.Location = location;
		this.Label54.Name = "Label54";
		System.Windows.Forms.Label label86 = this.Label54;
		size = new System.Drawing.Size(54, 16);
		label86.Size = size;
		this.Label54.TabIndex = 42;
		this.Label54.Text = "บรรท\u0e31ด :";
		this.Tvat_head2.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tvat_head2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tvat_head2.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tvat_head = this.Tvat_head2;
		location = new System.Drawing.Point(194, 40);
		tvat_head.Location = location;
		System.Windows.Forms.TextBox tvat_head2 = this.Tvat_head2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tvat_head2.Margin = margin;
		this.Tvat_head2.MaxLength = 50;
		this.Tvat_head2.Name = "Tvat_head2";
		System.Windows.Forms.TextBox tvat_head3 = this.Tvat_head2;
		size = new System.Drawing.Size(149, 23);
		tvat_head3.Size = size;
		this.Tvat_head2.TabIndex = 41;
		this.Tvat_head2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label52.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label52.AutoSize = true;
		System.Windows.Forms.Label label87 = this.Label52;
		location = new System.Drawing.Point(75, 43);
		label87.Location = location;
		this.Label52.Name = "Label52";
		System.Windows.Forms.Label label88 = this.Label52;
		size = new System.Drawing.Size(117, 16);
		label88.Size = size;
		this.Label52.TabIndex = 40;
		this.Label52.Text = "ห\u0e31วใบกำก\u0e31บ(สำเนา) :";
		this.ComboBox_6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_6.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_6.FormattingEnabled = true;
		this.ComboBox_6.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox_3 = this.ComboBox_6;
		location = new System.Drawing.Point(194, 93);
		comboBox_3.Location = location;
		this.ComboBox_6.Name = "ComboBox14";
		System.Windows.Forms.ComboBox comboBox_4 = this.ComboBox_6;
		size = new System.Drawing.Size(149, 24);
		comboBox_4.Size = size;
		this.ComboBox_6.TabIndex = 39;
		this.Label51.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label51.AutoSize = true;
		System.Windows.Forms.Label label89 = this.Label51;
		location = new System.Drawing.Point(36, 97);
		label89.Location = location;
		this.Label51.Name = "Label51";
		System.Windows.Forms.Label label90 = this.Label51;
		size = new System.Drawing.Size(156, 16);
		label90.Size = size;
		this.Label51.TabIndex = 38;
		this.Label51.Text = "สร\u0e49างใบกำก\u0e31บหล\u0e31งเช\u0e47คเอ\u0e49าท\u0e4c :";
		this.Tvat_head.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tvat_head.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tvat_head.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tvat_head4 = this.Tvat_head;
		location = new System.Drawing.Point(194, 14);
		tvat_head4.Location = location;
		System.Windows.Forms.TextBox tvat_head5 = this.Tvat_head;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tvat_head5.Margin = margin;
		this.Tvat_head.MaxLength = 50;
		this.Tvat_head.Name = "Tvat_head";
		System.Windows.Forms.TextBox tvat_head6 = this.Tvat_head;
		size = new System.Drawing.Size(149, 23);
		tvat_head6.Size = size;
		this.Tvat_head.TabIndex = 37;
		this.Tvat_head.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label43.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label43.AutoSize = true;
		System.Windows.Forms.Label label91 = this.Label43;
		location = new System.Drawing.Point(117, 18);
		label91.Location = location;
		this.Label43.Name = "Label43";
		System.Windows.Forms.Label label92 = this.Label43;
		size = new System.Drawing.Size(75, 16);
		label92.Size = size;
		this.Label43.TabIndex = 36;
		this.Label43.Text = "ห\u0e31วใบกำก\u0e31บ :";
		this.Label42.AutoSize = true;
		System.Windows.Forms.Label label93 = this.Label42;
		location = new System.Drawing.Point(80, 20);
		label93.Location = location;
		this.Label42.Name = "Label42";
		System.Windows.Forms.Label label94 = this.Label42;
		size = new System.Drawing.Size(20, 16);
		label94.Size = size;
		this.Label42.TabIndex = 35;
		this.Label42.Text = "%";
		this.Tvat_per.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tvat_per.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tvat_per.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox tvat_per = this.Tvat_per;
		location = new System.Drawing.Point(50, 16);
		tvat_per.Location = location;
		System.Windows.Forms.TextBox tvat_per2 = this.Tvat_per;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tvat_per2.Margin = margin;
		this.Tvat_per.MaxLength = 500;
		this.Tvat_per.Name = "Tvat_per";
		System.Windows.Forms.TextBox tvat_per3 = this.Tvat_per;
		size = new System.Drawing.Size(26, 23);
		tvat_per3.Size = size;
		this.Tvat_per.TabIndex = 34;
		this.Tvat_per.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label9.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label95 = this.Label9;
		location = new System.Drawing.Point(14, 20);
		label95.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label96 = this.Label9;
		size = new System.Drawing.Size(41, 16);
		label96.Size = size;
		this.Label9.TabIndex = 33;
		this.Label9.Text = "ภาษ\u0e35 :";
		this.ComboBox5.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox5.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox5.FormattingEnabled = true;
		this.ComboBox5.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox11 = this.ComboBox5;
		location = new System.Drawing.Point(194, 66);
		comboBox11.Location = location;
		this.ComboBox5.Name = "ComboBox5";
		System.Windows.Forms.ComboBox comboBox12 = this.ComboBox5;
		size = new System.Drawing.Size(149, 24);
		comboBox12.Size = size;
		this.ComboBox5.TabIndex = 32;
		this.Label30.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label30.AutoSize = true;
		System.Windows.Forms.Label label97 = this.Label30;
		location = new System.Drawing.Point(95, 70);
		label97.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label98 = this.Label30;
		size = new System.Drawing.Size(97, 16);
		label98.Size = size;
		this.Label30.TabIndex = 31;
		this.Label30.Text = "แสดงก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.GroupBox7.Controls.Add(this.ComboBox_4);
		this.GroupBox7.Controls.Add(this.Label44);
		this.GroupBox7.Controls.Add(this.ComboBox8);
		this.GroupBox7.Controls.Add(this.Label34);
		this.GroupBox7.Controls.Add(this.ComboBox7);
		this.GroupBox7.Controls.Add(this.Label32);
		System.Windows.Forms.GroupBox groupBox17 = this.GroupBox7;
		location = new System.Drawing.Point(581, 295);
		groupBox17.Location = location;
		this.GroupBox7.Name = "GroupBox7";
		System.Windows.Forms.GroupBox groupBox18 = this.GroupBox7;
		size = new System.Drawing.Size(354, 96);
		groupBox18.Size = size;
		this.GroupBox7.TabIndex = 36;
		this.GroupBox7.TabStop = false;
		this.GroupBox7.Text = "ต\u0e31\u0e49งค\u0e48าใบลงทะเบ\u0e35ยน";
		this.ComboBox_4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_4.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_4.FormattingEnabled = true;
		this.ComboBox_4.Items.AddRange(new object[3] { "แบบท\u0e35\u0e48 1", "แบบท\u0e35\u0e48 2", "แบบท\u0e35\u0e48 3 + ร\u0e39ปบ\u0e31ตร" });
		System.Windows.Forms.ComboBox comboBox_5 = this.ComboBox_4;
		location = new System.Drawing.Point(194, 13);
		comboBox_5.Location = location;
		this.ComboBox_4.Name = "ComboBox12";
		System.Windows.Forms.ComboBox comboBox_6 = this.ComboBox_4;
		size = new System.Drawing.Size(149, 24);
		comboBox_6.Size = size;
		this.ComboBox_4.TabIndex = 38;
		this.Label44.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label44.AutoSize = true;
		System.Windows.Forms.Label label99 = this.Label44;
		location = new System.Drawing.Point(66, 17);
		label99.Location = location;
		this.Label44.Name = "Label44";
		System.Windows.Forms.Label label100 = this.Label44;
		size = new System.Drawing.Size(126, 16);
		label100.Size = size;
		this.Label44.TabIndex = 37;
		this.Label44.Text = "ร\u0e39ปแบบใบลงทะเบ\u0e35ยน :";
		this.ComboBox8.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox8.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox8.FormattingEnabled = true;
		this.ComboBox8.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox13 = this.ComboBox8;
		location = new System.Drawing.Point(194, 63);
		comboBox13.Location = location;
		this.ComboBox8.Name = "ComboBox8";
		System.Windows.Forms.ComboBox comboBox14 = this.ComboBox8;
		size = new System.Drawing.Size(149, 24);
		comboBox14.Size = size;
		this.ComboBox8.TabIndex = 36;
		this.Label34.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label34.AutoSize = true;
		System.Windows.Forms.Label label101 = this.Label34;
		location = new System.Drawing.Point(2, 67);
		label101.Location = location;
		this.Label34.Name = "Label34";
		System.Windows.Forms.Label label102 = this.Label34;
		size = new System.Drawing.Size(190, 16);
		label102.Size = size;
		this.Label34.TabIndex = 35;
		this.Label34.Text = "พ\u0e34มพ\u0e4cใบลงทะเบ\u0e35ยนหล\u0e31ง Check-In :";
		this.ComboBox7.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox7.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox7.FormattingEnabled = true;
		this.ComboBox7.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox15 = this.ComboBox7;
		location = new System.Drawing.Point(194, 38);
		comboBox15.Location = location;
		this.ComboBox7.Name = "ComboBox7";
		System.Windows.Forms.ComboBox comboBox16 = this.ComboBox7;
		size = new System.Drawing.Size(149, 24);
		comboBox16.Size = size;
		this.ComboBox7.TabIndex = 32;
		this.Label32.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label32.AutoSize = true;
		System.Windows.Forms.Label label103 = this.Label32;
		location = new System.Drawing.Point(24, 42);
		label103.Location = location;
		this.Label32.Name = "Label32";
		System.Windows.Forms.Label label104 = this.Label32;
		size = new System.Drawing.Size(168, 16);
		label104.Size = size;
		this.Label32.TabIndex = 31;
		this.Label32.Text = "แสดงใบลงทะเบ\u0e35ยนก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.GroupBox8.Controls.Add(this.ComboBox_1);
		this.GroupBox8.Controls.Add(this.Label38);
		this.GroupBox8.Controls.Add(this.ComboBox9);
		this.GroupBox8.Controls.Add(this.Label35);
		this.GroupBox8.Controls.Add(this.ComboBox_0);
		this.GroupBox8.Controls.Add(this.Label36);
		System.Windows.Forms.GroupBox groupBox19 = this.GroupBox8;
		location = new System.Drawing.Point(581, 397);
		groupBox19.Location = location;
		this.GroupBox8.Name = "GroupBox8";
		System.Windows.Forms.GroupBox groupBox20 = this.GroupBox8;
		size = new System.Drawing.Size(354, 93);
		groupBox20.Size = size;
		this.GroupBox8.TabIndex = 37;
		this.GroupBox8.TabStop = false;
		this.GroupBox8.Text = "ต\u0e31\u0e49งค\u0e48าค\u0e39ปองอาหาร";
		this.ComboBox_1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_1.FormattingEnabled = true;
		this.ComboBox_1.Items.AddRange(new object[2] { "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)" });
		System.Windows.Forms.ComboBox comboBox_7 = this.ComboBox_1;
		location = new System.Drawing.Point(194, 13);
		comboBox_7.Location = location;
		this.ComboBox_1.Name = "ComboBox11";
		System.Windows.Forms.ComboBox comboBox_8 = this.ComboBox_1;
		size = new System.Drawing.Size(149, 24);
		comboBox_8.Size = size;
		this.ComboBox_1.TabIndex = 38;
		this.Label38.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label38.AutoSize = true;
		System.Windows.Forms.Label label105 = this.Label38;
		location = new System.Drawing.Point(107, 17);
		label105.Location = location;
		this.Label38.Name = "Label38";
		System.Windows.Forms.Label label106 = this.Label38;
		size = new System.Drawing.Size(85, 16);
		label106.Size = size;
		this.Label38.TabIndex = 37;
		this.Label38.Text = "ร\u0e39ปแบบค\u0e39ปอง :";
		this.ComboBox9.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox9.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox9.FormattingEnabled = true;
		this.ComboBox9.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox17 = this.ComboBox9;
		location = new System.Drawing.Point(194, 63);
		comboBox17.Location = location;
		this.ComboBox9.Name = "ComboBox9";
		System.Windows.Forms.ComboBox comboBox18 = this.ComboBox9;
		size = new System.Drawing.Size(149, 24);
		comboBox18.Size = size;
		this.ComboBox9.TabIndex = 36;
		this.Label35.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label35.AutoSize = true;
		System.Windows.Forms.Label label107 = this.Label35;
		location = new System.Drawing.Point(43, 67);
		label107.Location = location;
		this.Label35.Name = "Label35";
		System.Windows.Forms.Label label108 = this.Label35;
		size = new System.Drawing.Size(149, 16);
		label108.Size = size;
		this.Label35.TabIndex = 35;
		this.Label35.Text = "พ\u0e34มพ\u0e4cค\u0e39ปองหล\u0e31ง Check-In :";
		this.ComboBox_0.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_0.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_0.FormattingEnabled = true;
		this.ComboBox_0.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox_9 = this.ComboBox_0;
		location = new System.Drawing.Point(194, 38);
		comboBox_9.Location = location;
		this.ComboBox_0.Name = "ComboBox10";
		System.Windows.Forms.ComboBox comboBox_10 = this.ComboBox_0;
		size = new System.Drawing.Size(149, 24);
		comboBox_10.Size = size;
		this.ComboBox_0.TabIndex = 32;
		this.Label36.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label36.AutoSize = true;
		System.Windows.Forms.Label label109 = this.Label36;
		location = new System.Drawing.Point(65, 42);
		label109.Location = location;
		this.Label36.Name = "Label36";
		System.Windows.Forms.Label label110 = this.Label36;
		size = new System.Drawing.Size(127, 16);
		label110.Size = size;
		this.Label36.TabIndex = 31;
		this.Label36.Text = "แสดงค\u0e39ปองก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.GroupBox9.Controls.Add(this.ComboBox_2);
		this.GroupBox9.Controls.Add(this.Label39);
		this.GroupBox9.Controls.Add(this.ComboBox_3);
		this.GroupBox9.Controls.Add(this.Label40);
		System.Windows.Forms.GroupBox groupBox21 = this.GroupBox9;
		location = new System.Drawing.Point(581, 495);
		groupBox21.Location = location;
		this.GroupBox9.Name = "GroupBox9";
		System.Windows.Forms.GroupBox groupBox22 = this.GroupBox9;
		size = new System.Drawing.Size(354, 73);
		groupBox22.Size = size;
		this.GroupBox9.TabIndex = 38;
		this.GroupBox9.TabStop = false;
		this.GroupBox9.Text = "ต\u0e31\u0e49งค\u0e48าใบฝากเก\u0e47บ/ใบแจ\u0e49งหน\u0e35\u0e49";
		this.ComboBox_2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_2.FormattingEnabled = true;
		this.ComboBox_2.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox_11 = this.ComboBox_2;
		location = new System.Drawing.Point(194, 41);
		comboBox_11.Location = location;
		this.ComboBox_2.Name = "ComboBox21";
		System.Windows.Forms.ComboBox comboBox_12 = this.ComboBox_2;
		size = new System.Drawing.Size(149, 24);
		comboBox_12.Size = size;
		this.ComboBox_2.TabIndex = 36;
		this.Label39.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label39.AutoSize = true;
		System.Windows.Forms.Label label111 = this.Label39;
		location = new System.Drawing.Point(10, 45);
		label111.Location = location;
		this.Label39.Name = "Label39";
		System.Windows.Forms.Label label112 = this.Label39;
		size = new System.Drawing.Size(182, 16);
		label112.Size = size;
		this.Label39.TabIndex = 35;
		this.Label39.Text = "พ\u0e34มพ\u0e4cใบฝากเก\u0e47บหล\u0e31ง Check-out :";
		this.ComboBox_3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_3.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_3.FormattingEnabled = true;
		this.ComboBox_3.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox_13 = this.ComboBox_3;
		location = new System.Drawing.Point(194, 16);
		comboBox_13.Location = location;
		this.ComboBox_3.Name = "ComboBox20";
		System.Windows.Forms.ComboBox comboBox_14 = this.ComboBox_3;
		size = new System.Drawing.Size(149, 24);
		comboBox_14.Size = size;
		this.ComboBox_3.TabIndex = 32;
		this.Label40.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label40.AutoSize = true;
		System.Windows.Forms.Label label113 = this.Label40;
		location = new System.Drawing.Point(39, 20);
		label113.Location = location;
		this.Label40.Name = "Label40";
		System.Windows.Forms.Label label114 = this.Label40;
		size = new System.Drawing.Size(153, 16);
		label114.Size = size;
		this.Label40.TabIndex = 31;
		this.Label40.Text = "แสดงใบฝากเก\u0e47บก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.GroupBox_0.Controls.Add(this.Label50);
		this.GroupBox_0.Controls.Add(this.comdelay);
		this.GroupBox_0.Controls.Add(this.Label49);
		this.GroupBox_0.Controls.Add(this.CheckBox3);
		this.GroupBox_0.Controls.Add(this.CheckBox2);
		this.GroupBox_0.Controls.Add(this.TextURL);
		this.GroupBox_0.Controls.Add(this.Label46);
		this.GroupBox_0.Controls.Add(this.ComboBox_5);
		this.GroupBox_0.Controls.Add(this.CheckBox1);
		this.GroupBox_0.Controls.Add(this.Label45);
		System.Windows.Forms.GroupBox groupBox_5 = this.GroupBox_0;
		location = new System.Drawing.Point(7, 474);
		groupBox_5.Location = location;
		this.GroupBox_0.Name = "GroupBox10";
		System.Windows.Forms.GroupBox groupBox_6 = this.GroupBox_0;
		size = new System.Drawing.Size(568, 105);
		groupBox_6.Size = size;
		this.GroupBox_0.TabIndex = 39;
		this.GroupBox_0.TabStop = false;
		this.GroupBox_0.Text = "ต\u0e31\u0e49งค\u0e48าการเป\u0e34ดป\u0e34ดไฟ";
		this.Label50.AutoSize = true;
		System.Windows.Forms.Label label115 = this.Label50;
		location = new System.Drawing.Point(418, 53);
		label115.Location = location;
		this.Label50.Name = "Label50";
		System.Windows.Forms.Label label116 = this.Label50;
		size = new System.Drawing.Size(144, 16);
		label116.Size = size;
		this.Label50.TabIndex = 41;
		this.Label50.Text = "ม\u0e34ล\u0e34ว\u0e34นาท\u0e35 (1000=1 ว\u0e34นาท\u0e35)";
		this.comdelay.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.comdelay.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.comdelay.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox textBox16 = this.comdelay;
		location = new System.Drawing.Point(372, 50);
		textBox16.Location = location;
		System.Windows.Forms.TextBox textBox17 = this.comdelay;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox17.Margin = margin;
		this.comdelay.MaxLength = 500;
		this.comdelay.Name = "comdelay";
		System.Windows.Forms.TextBox textBox18 = this.comdelay;
		size = new System.Drawing.Size(43, 23);
		textBox18.Size = size;
		this.comdelay.TabIndex = 40;
		this.comdelay.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label49.AutoSize = true;
		System.Windows.Forms.Label label117 = this.Label49;
		location = new System.Drawing.Point(233, 53);
		label117.Location = location;
		this.Label49.Name = "Label49";
		System.Windows.Forms.Label label118 = this.Label49;
		size = new System.Drawing.Size(136, 16);
		label118.Size = size;
		this.Label49.TabIndex = 39;
		this.Label49.Text = "COM PORT หน\u0e48วงเวลา :";
		this.CheckBox3.AutoSize = true;
		this.CheckBox3.Enabled = false;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox3;
		location = new System.Drawing.Point(337, 20);
		checkBox.Location = location;
		this.CheckBox3.Name = "CheckBox3";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox3;
		size = new System.Drawing.Size(225, 20);
		checkBox2.Size = size;
		this.CheckBox3.TabIndex = 38;
		this.CheckBox3.Text = "อน\u0e38ญาต\u0e34ให\u0e49พน\u0e31กงาน เป\u0e34ด/ป\u0e34ด ไฟเองได\u0e49";
		this.CheckBox3.UseVisualStyleBackColor = true;
		this.CheckBox2.AutoSize = true;
		this.CheckBox2.Enabled = false;
		System.Windows.Forms.CheckBox checkBox3 = this.CheckBox2;
		location = new System.Drawing.Point(162, 22);
		checkBox3.Location = location;
		this.CheckBox2.Name = "CheckBox2";
		System.Windows.Forms.CheckBox checkBox4 = this.CheckBox2;
		size = new System.Drawing.Size(169, 20);
		checkBox4.Size = size;
		this.CheckBox2.TabIndex = 37;
		this.CheckBox2.Text = "ป\u0e34ดไฟอ\u0e31ตโนม\u0e31ต\u0e34เม\u0e37\u0e48อเก\u0e34นเวลา";
		this.CheckBox2.UseVisualStyleBackColor = true;
		this.TextURL.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TextURL.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TextURL.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.TextBox textURL = this.TextURL;
		location = new System.Drawing.Point(106, 76);
		textURL.Location = location;
		System.Windows.Forms.TextBox textURL2 = this.TextURL;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textURL2.Margin = margin;
		this.TextURL.MaxLength = 250;
		this.TextURL.Name = "TextURL";
		System.Windows.Forms.TextBox textURL3 = this.TextURL;
		size = new System.Drawing.Size(456, 23);
		textURL3.Size = size;
		this.TextURL.TabIndex = 35;
		this.Label46.AutoSize = true;
		System.Windows.Forms.Label label119 = this.Label46;
		location = new System.Drawing.Point(6, 80);
		label119.Location = location;
		this.Label46.Name = "Label46";
		System.Windows.Forms.Label label120 = this.Label46;
		size = new System.Drawing.Size(98, 16);
		label120.Size = size;
		this.Label46.TabIndex = 36;
		this.Label46.Text = "HTTP login url :";
		this.ComboBox_5.FormattingEnabled = true;
		this.ComboBox_5.Items.AddRange(new object[1] { "COM1" });
		System.Windows.Forms.ComboBox comboBox_15 = this.ComboBox_5;
		location = new System.Drawing.Point(106, 49);
		comboBox_15.Location = location;
		this.ComboBox_5.Name = "ComboBox13";
		System.Windows.Forms.ComboBox comboBox_16 = this.ComboBox_5;
		size = new System.Drawing.Size(121, 24);
		comboBox_16.Size = size;
		this.ComboBox_5.TabIndex = 34;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox5 = this.CheckBox1;
		location = new System.Drawing.Point(24, 22);
		checkBox5.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox6 = this.CheckBox1;
		size = new System.Drawing.Size(126, 20);
		checkBox6.Size = size;
		this.CheckBox1.TabIndex = 33;
		this.CheckBox1.Text = "ใช\u0e49งาน เป\u0e34ด-ป\u0e34ด ไฟ";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.Label45.AutoSize = true;
		System.Windows.Forms.Label label121 = this.Label45;
		location = new System.Drawing.Point(24, 52);
		label121.Location = location;
		this.Label45.Name = "Label45";
		System.Windows.Forms.Label label122 = this.Label45;
		size = new System.Drawing.Size(80, 16);
		label122.Size = size;
		this.Label45.TabIndex = 32;
		this.Label45.Text = "COM PORT :";
		this.CheckBoxIcon.AutoSize = true;
		System.Windows.Forms.CheckBox checkBoxIcon = this.CheckBoxIcon;
		location = new System.Drawing.Point(739, 668);
		checkBoxIcon.Location = location;
		this.CheckBoxIcon.Name = "CheckBoxIcon";
		System.Windows.Forms.CheckBox checkBoxIcon2 = this.CheckBoxIcon;
		size = new System.Drawing.Size(185, 20);
		checkBoxIcon2.Size = size;
		this.CheckBoxIcon.TabIndex = 53;
		this.CheckBoxIcon.Text = "แสดงไอคอนกล\u0e38\u0e48มล\u0e39กค\u0e49าเข\u0e49าพ\u0e31ก";
		this.CheckBoxIcon.UseVisualStyleBackColor = true;
		this.PanelEx1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx1.AutoScroll = true;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.CheckBoxBookingNotification);
		this.PanelEx1.Controls.Add(this.GroupBox_3);
		this.PanelEx1.Controls.Add(this.GroupBox2);
		this.PanelEx1.Controls.Add(this.BPIC2);
		this.PanelEx1.Controls.Add(this.Button1);
		this.PanelEx1.Controls.Add(this.CheckBoxIcon);
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Controls.Add(this.GroupBox_2);
		this.PanelEx1.Controls.Add(this.GroupBox3);
		this.PanelEx1.Controls.Add(this.BPIC1);
		this.PanelEx1.Controls.Add(this.GroupBox5);
		this.PanelEx1.Controls.Add(this.GroupBox_1);
		this.PanelEx1.Controls.Add(this.GroupBox6);
		this.PanelEx1.Controls.Add(this.GroupBox_0);
		this.PanelEx1.Controls.Add(this.GroupBox7);
		this.PanelEx1.Controls.Add(this.PanelPic);
		this.PanelEx1.Controls.Add(this.GroupBox4);
		this.PanelEx1.Controls.Add(this.GroupBox9);
		this.PanelEx1.Controls.Add(this.GroupBox8);
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx1;
		location = new System.Drawing.Point(0, 39);
		panelEx4.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx1;
		size = new System.Drawing.Size(968, 651);
		panelEx5.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 54;
		this.CheckBoxBookingNotification.AutoSize = true;
		System.Windows.Forms.CheckBox checkBoxBookingNotification = this.CheckBoxBookingNotification;
		location = new System.Drawing.Point(739, 697);
		checkBoxBookingNotification.Location = location;
		this.CheckBoxBookingNotification.Name = "CheckBoxBookingNotification";
		System.Windows.Forms.CheckBox checkBoxBookingNotification2 = this.CheckBoxBookingNotification;
		size = new System.Drawing.Size(190, 20);
		checkBoxBookingNotification2.Size = size;
		this.CheckBoxBookingNotification.TabIndex = 64;
		this.CheckBoxBookingNotification.Text = "ใช\u0e49งานแจ\u0e49งเต\u0e37อนการจองห\u0e49องพ\u0e31ก";
		this.CheckBoxBookingNotification.UseVisualStyleBackColor = true;
		this.GroupBox_3.Controls.Add(this.ComboBox_9);
		this.GroupBox_3.Controls.Add(this.Label61);
		this.GroupBox_3.Controls.Add(this.ComboBox_8);
		this.GroupBox_3.Controls.Add(this.Label60);
		System.Windows.Forms.GroupBox groupBox_7 = this.GroupBox_3;
		location = new System.Drawing.Point(581, 572);
		groupBox_7.Location = location;
		this.GroupBox_3.Name = "GroupBox13";
		System.Windows.Forms.GroupBox groupBox_8 = this.GroupBox_3;
		size = new System.Drawing.Size(354, 78);
		groupBox_8.Size = size;
		this.GroupBox_3.TabIndex = 54;
		this.GroupBox_3.TabStop = false;
		this.GroupBox_3.Text = "ต\u0e31\u0e49งค\u0e48าใบเสร\u0e47จ ขายส\u0e34นค\u0e49า POS";
		this.ComboBox_9.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_9.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_9.FormattingEnabled = true;
		this.ComboBox_9.Items.AddRange(new object[2] { "เป\u0e34ด", "ป\u0e34ด" });
		System.Windows.Forms.ComboBox comboBox_17 = this.ComboBox_9;
		location = new System.Drawing.Point(194, 43);
		comboBox_17.Location = location;
		this.ComboBox_9.Name = "ComboBox16";
		System.Windows.Forms.ComboBox comboBox_18 = this.ComboBox_9;
		size = new System.Drawing.Size(149, 24);
		comboBox_18.Size = size;
		this.ComboBox_9.TabIndex = 34;
		this.Label61.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label61.AutoSize = true;
		System.Windows.Forms.Label label123 = this.Label61;
		location = new System.Drawing.Point(95, 46);
		label123.Location = location;
		this.Label61.Name = "Label61";
		System.Windows.Forms.Label label124 = this.Label61;
		size = new System.Drawing.Size(97, 16);
		label124.Size = size;
		this.Label61.TabIndex = 33;
		this.Label61.Text = "แสดงก\u0e48อนพ\u0e34มพ\u0e4c :";
		this.ComboBox_8.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.ComboBox_8.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox_8.FormattingEnabled = true;
		this.ComboBox_8.Items.AddRange(new object[3] { "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)", "เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (58mm)", "กระดาษต\u0e48อเน\u0e37\u0e48อง" });
		System.Windows.Forms.ComboBox comboBox_19 = this.ComboBox_8;
		location = new System.Drawing.Point(194, 16);
		comboBox_19.Location = location;
		this.ComboBox_8.Name = "ComboBox17";
		System.Windows.Forms.ComboBox comboBox_20 = this.ComboBox_8;
		size = new System.Drawing.Size(149, 24);
		comboBox_20.Size = size;
		this.ComboBox_8.TabIndex = 32;
		this.Label60.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label60.AutoSize = true;
		System.Windows.Forms.Label label125 = this.Label60;
		location = new System.Drawing.Point(102, 21);
		label125.Location = location;
		this.Label60.Name = "Label60";
		System.Windows.Forms.Label label126 = this.Label60;
		size = new System.Drawing.Size(90, 16);
		label126.Size = size;
		this.Label60.TabIndex = 31;
		this.Label60.Text = "ร\u0e39ปแบบใบเสร\u0e47จ:";
		this.Label64.AutoSize = true;
		this.Label64.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label127 = this.Label64;
		location = new System.Drawing.Point(124, 44);
		label127.Location = location;
		this.Label64.Name = "Label64";
		System.Windows.Forms.Label label128 = this.Label64;
		size = new System.Drawing.Size(161, 16);
		label128.Size = size;
		this.Label64.TabIndex = 47;
		this.Label64.Text = "(ใส\u0e48 ช.ม.และนาท\u0e35 เช\u0e48น 1200)";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.FromArgb(194, 217, 247);
		size = new System.Drawing.Size(968, 691);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmSettings";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ต\u0e31\u0e49งค\u0e48า";
		this.ContextMenuStrip1.ResumeLayout(false);
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.PanelPic.ResumeLayout(false);
		((System.ComponentModel.ISupportInitialize)this.PictureBox1).EndInit();
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.GroupBox_2.ResumeLayout(false);
		this.GroupBox_2.PerformLayout();
		this.GroupBox_1.ResumeLayout(false);
		this.GroupBox_1.PerformLayout();
		this.GroupBox3.ResumeLayout(false);
		this.GroupBox3.PerformLayout();
		this.GroupBox4.ResumeLayout(false);
		this.GroupBox4.PerformLayout();
		this.GroupBox5.ResumeLayout(false);
		this.GroupBox5.PerformLayout();
		this.GroupBox6.ResumeLayout(false);
		this.GroupBox6.PerformLayout();
		this.GroupBox7.ResumeLayout(false);
		this.GroupBox7.PerformLayout();
		this.GroupBox8.ResumeLayout(false);
		this.GroupBox8.PerformLayout();
		this.GroupBox9.ResumeLayout(false);
		this.GroupBox9.PerformLayout();
		this.GroupBox_0.ResumeLayout(false);
		this.GroupBox_0.PerformLayout();
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.GroupBox_3.ResumeLayout(false);
		this.GroupBox_3.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmSettings_Load(object sender, EventArgs e)
	{
		if (!Module1.IS_TRIAL && Module1.COM_ID.ToString().Length != 12)
		{
			Tcompany_name.Enabled = false;
			Ttax.Enabled = false;
		}
		ComboBox_5.Items.Clear();
		ComboBox_5.Items.Add("HTTP");
		foreach (string serialPortName in MyProject.Computer.Ports.SerialPortNames)
		{
			ComboBox_5.Items.Add(serialPortName);
		}
		ComboBox_7.Items.Clear();
		ComboBox_7.Items.Add("ไม\u0e48ใช\u0e49งาน");
		foreach (string serialPortName2 in MyProject.Computer.Ports.SerialPortNames)
		{
			ComboBox_7.Items.Add(serialPortName2);
		}
		ReadPrint();
		load_copy();
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		Tcompany_name.Text = dataSet.Tables[0].Rows[0]["CompanyName"].ToString();
		Tcompany_address.Text = dataSet.Tables[0].Rows[0]["company_address"].ToString();
		Ttel.Text = dataSet.Tables[0].Rows[0]["company_tel"].ToString();
		Tfax.Text = dataSet.Tables[0].Rows[0]["company_fax"].ToString();
		Ttax.Text = dataSet.Tables[0].Rows[0]["company_tax"].ToString();
		MinHours.Text = dataSet.Tables[0].Rows[0]["Min_HOURS"].ToString();
		CheckBox2.Checked = Conversions.ToBoolean(dataSet.Tables[0].Rows[0]["AUTO_CUT_POWER"]);
		CheckBox3.Checked = Conversions.ToBoolean(dataSet.Tables[0].Rows[0]["MANUAL_POWER"]);
		CheckBoxIcon.Checked = Conversions.ToBoolean(dataSet.Tables[0].Rows[0]["SHOW_ICON"]);
		Tclean.Text = dataSet.Tables[0].Rows[0]["Room_Clean_Time"].ToString();
		TCHK_IN_Before.Text = dataSet.Tables[0].Rows[0]["CHK_IN_Before"].ToString();
		TCHK_Out.Text = dataSet.Tables[0].Rows[0]["CHK_Out"].ToString();
		TCHK_Out_Alert.Text = dataSet.Tables[0].Rows[0]["CHK_Out_Alert"].ToString();
		TCHK_Out_Before.Text = dataSet.Tables[0].Rows[0]["CHK_Out_Before"].ToString();
		TCHK_Out_H_price.Text = dataSet.Tables[0].Rows[0]["CHK_Out_H_price"].ToString();
		TMaximum_Book.Text = dataSet.Tables[0].Rows[0]["Maximum_Book"].ToString();
		TPority.Text = dataSet.Tables[0].Rows[0]["Pority"].ToString();
		TVat_Over.Text = dataSet.Tables[0].Rows[0]["Vat_Over"].ToString();
		Tlogout.Text = dataSet.Tables[0].Rows[0]["Time_Logout"].ToString();
		Tvat_per.Text = dataSet.Tables[0].Rows[0]["Vat_per"].ToString();
		Tvat_head.Text = dataSet.Tables[0].Rows[0]["Vat_Head"].ToString();
		Tvat_head2.Text = dataSet.Tables[0].Rows[0]["Vat_Head2"].ToString();
		TextURL.Text = dataSet.Tables[0].Rows[0]["login_url"].ToString();
		ComboBox_6.Text = dataSet.Tables[0].Rows[0]["VAT_OUT"].ToString();
		Tvat_Rows.Text = dataSet.Tables[0].Rows[0]["Vat_Rows"].ToString();
		try
		{
			ComboBox_4.SelectedIndex = Conversions.ToInteger(dataSet.Tables[0].Rows[0]["reg_type"]);
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		comdelay.Text = dataSet.Tables[0].Rows[0]["POWER_Delay"].ToString();
		AutoCust.Text = dataSet.Tables[0].Rows[0]["Cal_Pority_Cust"].ToString();
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Company_Image"].ToString(), "", TextCompare: false) != 0)
		{
			byte[] buffer = (byte[])dataSet.Tables[0].Rows[0]["Company_Image"];
			myBitmap = new Bitmap(new MemoryStream(buffer));
			ResizePic();
			PictureBox1.Image = myBitmap;
		}
		else
		{
			DeleteImage();
		}
		Timer1.Enabled = true;
	}

	public void SAVE_COPY()
	{
		StreamWriter streamWriter = File.CreateText(Module1.PathF + "\\SetPageCopy.txt");
		streamWriter.WriteLine(Copy1.Text);
		streamWriter.WriteLine(Copy2.Text);
		streamWriter.WriteLine(Copy3.Text);
		streamWriter.WriteLine(Copy4.Text);
		streamWriter.WriteLine(Copy5.Text);
		streamWriter.WriteLine(Copy6.Text);
		streamWriter.WriteLine(Copy7.Text);
		streamWriter.WriteLine(Copy8.Text);
		streamWriter.WriteLine(Copy9.Text);
		streamWriter.Close();
		Module1.int_1 = Conversions.ToInteger(Copy1.Text);
		Module1.int_2 = Conversions.ToInteger(Copy2.Text);
		Module1.int_3 = Conversions.ToInteger(Copy3.Text);
		Module1.int_4 = Conversions.ToInteger(Copy4.Text);
		Module1.int_7 = Conversions.ToInteger(Copy5.Text);
		Module1.int_8 = Conversions.ToInteger(Copy6.Text);
		Module1.copy_POS = Conversions.ToInteger(Copy7.Text);
		Module1.int_5 = Conversions.ToInteger(Copy8.Text);
		Module1.int_6 = Conversions.ToInteger(Copy9.Text);
	}

	public void load_copy()
	{
		if (!File.Exists(Module1.PathF + "\\SetPageCopy.txt"))
		{
			StreamWriter streamWriter = File.CreateText(Module1.PathF + "\\SetPageCopy.txt");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.WriteLine("1");
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(Module1.PathF + "\\SetPageCopy.txt");
		string expression = streamReader.ReadToEnd();
		streamReader.Close();
		string[] array = Strings.Split(expression, "\r\n");
		int num = 1;
		string[] array2 = array;
		foreach (string left in array2)
		{
			if (Operators.CompareString(left, "", TextCompare: false) != 0)
			{
				switch (num)
				{
				case 1:
					Copy1.Text = left;
					break;
				case 2:
					Copy2.Text = left;
					break;
				case 3:
					Copy3.Text = left;
					break;
				case 4:
					Copy4.Text = left;
					break;
				case 5:
					Copy5.Text = left;
					break;
				case 6:
					Copy6.Text = left;
					break;
				case 7:
					Copy7.Text = left;
					break;
				case 8:
					Copy8.Text = left;
					break;
				case 9:
					Copy9.Text = left;
					break;
				}
			}
			num = checked(num + 1);
		}
		if (Operators.CompareString(Copy8.Text, "", TextCompare: false) == 0)
		{
			Copy8.Text = "1";
		}
		if (Operators.CompareString(Copy9.Text, "", TextCompare: false) == 0)
		{
			Copy9.Text = "1";
		}
		Module1.int_1 = Conversions.ToInteger(Copy1.Text);
		Module1.int_2 = Conversions.ToInteger(Copy2.Text);
		Module1.int_3 = Conversions.ToInteger(Copy3.Text);
		Module1.int_4 = Conversions.ToInteger(Copy4.Text);
		Module1.int_7 = Conversions.ToInteger(Copy5.Text);
		Module1.int_8 = Conversions.ToInteger(Copy6.Text);
		Module1.copy_POS = Conversions.ToInteger(Copy7.Text);
		Module1.int_5 = Conversions.ToInteger(Copy8.Text);
		Module1.int_6 = Conversions.ToInteger(Copy9.Text);
	}

	public void ReadPrint()
	{
		if (!File.Exists(Module1.PathF + "\\SetPrinter.txt"))
		{
			StreamWriter streamWriter = File.CreateText(Module1.PathF + "\\SetPrinter.txt");
			streamWriter.WriteLine("กระดาษต\u0e48อเน\u0e37\u0e48อง");
			streamWriter.WriteLine("ป\u0e34ด");
			streamWriter.WriteLine("เป\u0e34ด");
			streamWriter.WriteLine("กระดาษต\u0e48อเน\u0e37\u0e48อง");
			streamWriter.WriteLine("เป\u0e34ด");
			streamWriter.WriteLine("เป\u0e34ด");
			streamWriter.WriteLine("เป\u0e34ด");
			streamWriter.WriteLine("ป\u0e34ด");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เป\u0e34ด");
			streamWriter.WriteLine("ป\u0e34ด");
			streamWriter.WriteLine("เคร\u0e37\u0e48องพ\u0e34มพ\u0e4cใบเสร\u0e47จ (80mm)");
			streamWriter.WriteLine("ป\u0e34ด");
			streamWriter.WriteLine("ป\u0e34ด");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("False");
			streamWriter.WriteLine("COM1");
			streamWriter.WriteLine("ไม\u0e48ใช\u0e49งาน");
			streamWriter.WriteLine("กระดาษต\u0e48อเน\u0e37\u0e48อง");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เป\u0e34ด");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("เล\u0e37อกตอนพ\u0e34มพ\u0e4c");
			streamWriter.WriteLine("True");
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(Module1.PathF + "\\SetPrinter.txt");
		string expression = streamReader.ReadToEnd();
		streamReader.Close();
		string[] array = Strings.Split(expression, "\r\n");
		int num = 1;
		string[] array2 = array;
		foreach (string text in array2)
		{
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				switch (num)
				{
				case 1:
					ComboBox1.Text = text.Replace("HHOTEL", "FOLIO");
					break;
				case 2:
					ComboBox2.Text = text;
					break;
				case 3:
					ComboBox3.Text = text;
					break;
				case 4:
					ComboBox6.Text = text;
					break;
				case 5:
					ComboBox4.Text = text;
					break;
				case 6:
					ComboBox5.Text = text;
					break;
				case 7:
					ComboBox7.Text = text;
					break;
				case 8:
					ComboBox8.Text = text;
					break;
				case 9:
					Print1.Text = text;
					break;
				case 10:
					Print2.Text = text;
					break;
				case 11:
					Print3.Text = text;
					break;
				case 12:
					Print4.Text = text;
					break;
				case 13:
					print5.Text = text;
					break;
				case 14:
					ComboBox_0.Text = text;
					break;
				case 15:
					ComboBox9.Text = text;
					break;
				case 16:
					ComboBox_1.Text = text;
					break;
				case 17:
					ComboBox_3.Text = text;
					break;
				case 18:
					ComboBox_2.Text = text;
					break;
				case 19:
					print6.Text = text;
					break;
				case 20:
					CheckBox1.Checked = Conversions.ToBoolean(text);
					break;
				case 21:
					ComboBox_5.Text = text;
					break;
				case 22:
					ComboBox_7.Text = text;
					break;
				case 23:
					ComboBox_8.Text = text;
					break;
				case 24:
					print7.Text = text;
					break;
				case 25:
					ComboBox_9.Text = text;
					break;
				case 26:
					print8.Text = text;
					break;
				case 27:
					print9.Text = text;
					break;
				case 28:
					CheckBoxBookingNotification.Checked = Conversions.ToBoolean(text);
					break;
				}
			}
			num = checked(num + 1);
		}
		if (Operators.CompareString(print8.Text, "", TextCompare: false) == 0)
		{
			print8.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		}
		if (Operators.CompareString(print9.Text, "", TextCompare: false) == 0)
		{
			print9.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		}
		Module1.Receipt_Report = ComboBox1.Text;
		Module1.Receipt_preview = ComboBox3.Text;
		Module1.Receipt_print = ComboBox2.Text;
		Module1.Deposit_Report = ComboBox6.Text;
		Module1.Deposit_preview = ComboBox4.Text;
		Module1.string_2 = ComboBox_1.Text;
		Module1.Cupon_Report = ComboBox9.Text;
		Module1.Cupon_preview = ComboBox_0.Text;
		Module1.inv_preview = ComboBox_3.Text;
		Module1.inv_print = ComboBox_2.Text;
		Module1.Tax_preview = ComboBox5.Text;
		Module1.Cin_preview = ComboBox7.Text;
		Module1.Cin_Print = ComboBox8.Text;
		Module1.POWER_USED = CheckBox1.Checked;
		Module1.POWER_PORT = ComboBox_5.Text;
		Module1.bool_1 = CheckBoxBookingNotification.Checked;
		if (Operators.CompareString(ComboBox_7.Text, "", TextCompare: false) != 0)
		{
			Module1.CASH_PORT = ComboBox_7.Text;
		}
		try
		{
			if (Module1.POWER_USED)
			{
				if (!MyProject.Forms.frmMain1.SerialPort1.IsOpen)
				{
					MyProject.Forms.frmMain1.SerialPort1.PortName = Module1.POWER_PORT;
					MyProject.Forms.frmMain1.SerialPort1.Open();
				}
				else
				{
					MyProject.Forms.frmMain1.SerialPort1.Close();
					MyProject.Forms.frmMain1.SerialPort1.PortName = Module1.POWER_PORT;
					MyProject.Forms.frmMain1.SerialPort1.Open();
				}
			}
			else if (MyProject.Forms.frmMain1.SerialPort1.IsOpen)
			{
				MyProject.Forms.frmMain1.SerialPort1.Close();
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		if (Operators.CompareString(print7.Text, "", TextCompare: false) == 0)
		{
			print7.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
		}
		if (Operators.CompareString(ComboBox_8.Text, "", TextCompare: false) == 0)
		{
			ComboBox_8.SelectedIndex = 0;
		}
		if (Operators.CompareString(ComboBox_9.Text, "", TextCompare: false) == 0)
		{
			ComboBox_9.SelectedIndex = 0;
		}
		Module1.POS_Report = ComboBox_8.Text;
		Module1.POS_print = print7.Text;
		Module1.POS_preview = ComboBox_9.Text;
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (CheckBox1.Checked && Operators.CompareString(ComboBox_5.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48 COM PORT");
			ComboBox_5.Focus();
			return;
		}
		if (Operators.CompareString(ComboBox_7.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48 COM PORT ล\u0e34\u0e49นช\u0e31ก");
			ComboBox_7.Focus();
			return;
		}
		if (Operators.CompareString(comdelay.Text, "", TextCompare: false) == 0)
		{
			comdelay.Text = Conversions.ToString(500);
		}
		if (!Versioned.IsNumeric(comdelay.Text))
		{
			comdelay.Text = Conversions.ToString(500);
		}
		if (Operators.CompareString(MinHours.Text, "", TextCompare: false) == 0)
		{
			MinHours.Text = Conversions.ToString(1);
		}
		if (!Versioned.IsNumeric(MinHours.Text))
		{
			MinHours.Text = Conversions.ToString(1);
		}
		if (decimal.Compare(Conversions.ToDecimal(MinHours.Text), 1m) <= 0)
		{
			MinHours.Text = Conversions.ToString(1);
		}
		Cursor = Cursors.WaitCursor;
		if (Operators.CompareString(Tclean.Text, "", TextCompare: false) == 0)
		{
			Tclean.Text = "30";
		}
		if (!Versioned.IsNumeric(Tclean.Text))
		{
			Tclean.Text = "30";
		}
		if (Operators.CompareString(TCHK_IN_Before.Text, "", TextCompare: false) == 0)
		{
			TCHK_IN_Before.Text = "559";
		}
		if (!Versioned.IsNumeric(TCHK_IN_Before.Text))
		{
			TCHK_IN_Before.Text = "559";
		}
		if (Operators.CompareString(TCHK_Out.Text, "", TextCompare: false) == 0)
		{
			TCHK_Out.Text = "1200";
		}
		if (!Versioned.IsNumeric(TCHK_Out.Text))
		{
			TCHK_Out.Text = "1200";
		}
		if (Operators.CompareString(TCHK_Out_Alert.Text, "", TextCompare: false) == 0)
		{
			TCHK_Out_Alert.Text = "1";
		}
		if (!Versioned.IsNumeric(TCHK_Out_Alert.Text))
		{
			TCHK_Out_Alert.Text = "1";
		}
		if (Operators.CompareString(TCHK_Out_Before.Text, "", TextCompare: false) == 0)
		{
			TCHK_Out_Before.Text = "1600";
		}
		if (!Versioned.IsNumeric(TCHK_Out_Before.Text))
		{
			TCHK_Out_Before.Text = "1600";
		}
		if (Operators.CompareString(TCHK_Out_H_price.Text, "", TextCompare: false) == 0)
		{
			TCHK_Out_H_price.Text = "50";
		}
		if (!Versioned.IsNumeric(TCHK_Out_H_price.Text))
		{
			TCHK_Out_H_price.Text = "50";
		}
		if (Operators.CompareString(TMaximum_Book.Text, "", TextCompare: false) == 0)
		{
			TMaximum_Book.Text = "20";
		}
		if (!Versioned.IsNumeric(TMaximum_Book.Text))
		{
			TMaximum_Book.Text = "20";
		}
		if (Operators.CompareString(TPority.Text, "", TextCompare: false) == 0)
		{
			TPority.Text = "5";
		}
		if (!Versioned.IsNumeric(TPority.Text))
		{
			TPority.Text = "5";
		}
		if (Operators.CompareString(TVat_Over.Text, "", TextCompare: false) == 0)
		{
			TVat_Over.Text = "0";
		}
		if (!Versioned.IsNumeric(TVat_Over.Text))
		{
			TVat_Over.Text = "0";
		}
		if (Operators.CompareString(Tlogout.Text, "", TextCompare: false) == 0)
		{
			Tlogout.Text = "60";
		}
		if (!Versioned.IsNumeric(Tlogout.Text))
		{
			Tlogout.Text = "60";
		}
		if (Operators.CompareString(Tvat_Rows.Text, "", TextCompare: false) == 0)
		{
			Tvat_Rows.Text = Conversions.ToString(7);
		}
		if (!Versioned.IsNumeric(Tvat_Rows.Text))
		{
			Tvat_Rows.Text = "7";
		}
		Bitmap bitmap = new Bitmap(myBitmap, PictureBox1.Width, PictureBox1.Height);
		bitmap.Save(Module1.Path_Program + "/tmp.png", ImageFormat.Png);
		bitmap.Dispose();
		FileStream fileStream = new FileStream(Module1.Path_Program + "/tmp.png", FileMode.Open, FileAccess.Read);
		BinaryReader binaryReader = new BinaryReader(fileStream);
		checked
		{
			byte[] array = binaryReader.ReadBytes((int)fileStream.Length);
			binaryReader.Close();
			fileStream.Close();
			StringBuilder stringBuilder = new StringBuilder();
			byte[] array2 = array;
			foreach (byte b in array2)
			{
				stringBuilder.Append(b.ToString("X2"));
			}
			object left = "Update TB_SETTINGS SET ";
			left = Operators.ConcatenateObject(left, string.Concat(" companyname='" + Tcompany_name.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",company_address='" + Tcompany_address.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",company_tel='" + Ttel.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",company_fax='" + Tfax.Text, "'"));
			left = Operators.ConcatenateObject(left, ",company_image=0x" + stringBuilder.ToString());
			left = Operators.ConcatenateObject(left, string.Concat(",company_tax='" + Ttax.Text, "'"));
			left = Operators.ConcatenateObject(left, ",CHK_IN_Before=" + TCHK_IN_Before.Text);
			left = Operators.ConcatenateObject(left, ",CHK_Out=" + TCHK_Out.Text);
			left = Operators.ConcatenateObject(left, ",CHK_Out_Alert=" + TCHK_Out_Alert.Text);
			left = Operators.ConcatenateObject(left, ",CHK_Out_Before=" + TCHK_Out_Before.Text);
			left = Operators.ConcatenateObject(left, ",CHK_Out_H_price=" + TCHK_Out_H_price.Text);
			left = Operators.ConcatenateObject(left, ",Maximum_Book=" + TMaximum_Book.Text);
			left = Operators.ConcatenateObject(left, ",Pority=" + TPority.Text);
			left = Operators.ConcatenateObject(left, ",Vat_Over=" + TVat_Over.Text);
			left = Operators.ConcatenateObject(left, string.Concat(",Cal_Pority_Cust='" + AutoCust.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",receipt_type='" + ComboBox1.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",receipt_print='" + ComboBox2.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",receipt_preview='" + ComboBox3.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",reg_type='" + Conversions.ToString(ComboBox_4.SelectedIndex), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",login_url='" + TextURL.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",Min_HOURS='" + Conversions.ToString(Conversions.ToDecimal(MinHours.Text)), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",AUTO_CUT_POWER='" + Conversions.ToString(CheckBox2.Checked), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",MANUAL_POWER='" + Conversions.ToString(CheckBox3.Checked), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",POWER_Delay='" + comdelay.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",VAT_OUT='" + ComboBox_6.Text, "'"));
			left = Operators.ConcatenateObject(left, ",Vat_Rows=" + Tvat_Rows.Text);
			left = Operators.ConcatenateObject(left, string.Concat(",Room_Clean_Time='" + Conversions.ToString(Conversions.ToInteger(Tclean.Text)), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",SHOW_ICON='" + Conversions.ToString(CheckBoxIcon.Checked), "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",Time_Logout='" + Conversions.ToString(Conversions.ToInteger(Tlogout.Text)), "'"));
			try
			{
				left = Operators.ConcatenateObject(left, ",Vat_per=" + Conversions.ToString(Conversions.ToDecimal(Tvat_per.Text)));
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				left = Operators.ConcatenateObject(left, ",Vat_per=0");
				ProjectData.ClearProjectError();
			}
			left = Operators.ConcatenateObject(left, string.Concat(",Vat_Head='" + Tvat_head.Text, "'"));
			left = Operators.ConcatenateObject(left, string.Concat(",Vat_Head2='" + Tvat_head2.Text, "'"));
			Module1.connect(Conversions.ToString(left));
			Module1.Company_Head = Tcompany_name.Text;
			if (Operators.CompareString(Tcompany_address.Text, "", TextCompare: false) != 0)
			{
				Module1.Company_Head = (string)Operators.ConcatenateObject(Module1.Company_Head, string.Concat("\r\n" + Tcompany_address.Text, "\r\n"));
			}
			if (Operators.CompareString(Ttel.Text, "", TextCompare: false) != 0)
			{
				Module1.Company_Head = (string)Operators.ConcatenateObject(Module1.Company_Head, "Tel. " + Ttel.Text);
			}
			if (Operators.CompareString(Tfax.Text, "", TextCompare: false) != 0)
			{
				Module1.Company_Head = (string)Operators.ConcatenateObject(Module1.Company_Head, " Fax. " + Tfax.Text);
			}
			Module1.loginURLsplit(TextURL.Text);
			Module1.CHK_IN_Before = Conversions.ToInteger(TCHK_IN_Before.Text);
			Module1.CHK_Out = Conversions.ToInteger(TCHK_Out.Text);
			Module1.CHK_Out_Alert = Conversions.ToInteger(TCHK_Out_Alert.Text);
			Module1.CHK_Out_Before = Conversions.ToInteger(TCHK_Out_Before.Text);
			Module1.CHK_Out_H_price = Conversions.ToInteger(TCHK_Out_H_price.Text);
			Module1.Maximum_Book = Conversions.ToInteger(TMaximum_Book.Text);
			Module1.Pority = Conversions.ToInteger(TPority.Text);
			Module1.Vat_Over = Conversions.ToDecimal(TVat_Over.Text);
			Module1.AutoLogout = Conversions.ToInteger(Tlogout.Text) * 60;
			Module1.Receipt_Report = ComboBox1.Text;
			Module1.Receipt_preview = ComboBox3.Text;
			Module1.Receipt_print = ComboBox2.Text;
			Module1.Deposit_Report = ComboBox6.Text;
			Module1.Deposit_preview = ComboBox4.Text;
			Module1.string_2 = ComboBox_1.Text;
			Module1.Cupon_preview = ComboBox_0.Text;
			Module1.Cupon_Report = ComboBox9.Text;
			Module1.inv_preview = ComboBox_3.Text;
			Module1.inv_print = ComboBox_2.Text;
			Module1.Tax_preview = ComboBox5.Text;
			Module1.Cin_preview = ComboBox7.Text;
			Module1.Cin_Print = ComboBox8.Text;
			Module1.POS_print = print7.Text;
			Module1.POS_Report = ComboBox_8.Text;
			Module1.POS_preview = ComboBox_9.Text;
			Module1.VAT_OUT = ComboBox_6.Text;
			if (Operators.CompareString(AutoCust.Text, "เป\u0e34ด", TextCompare: false) == 0)
			{
				Module1.AutoCalCust = true;
			}
			else
			{
				Module1.AutoCalCust = false;
			}
			Module1.MANUAL_POWER = CheckBox3.Checked;
			Module1.POWER_Delay = Conversions.ToInteger(comdelay.Text);
			Module1.decimal_0 = Conversions.ToDecimal(MinHours.Text);
			Module1.bool_3 = CheckBox2.Checked;
			Module1.decimal_1 = new decimal(Conversions.ToInteger(Tclean.Text));
			Module1.bool_1 = CheckBoxBookingNotification.Checked;
			Module1.SHOW_ICON = CheckBoxIcon.Checked;
			StreamWriter streamWriter = File.CreateText(Module1.PathF + "\\SetPrinter.txt");
			streamWriter.WriteLine(ComboBox1.Text);
			streamWriter.WriteLine(ComboBox2.Text);
			streamWriter.WriteLine(ComboBox3.Text);
			streamWriter.WriteLine(ComboBox6.Text);
			streamWriter.WriteLine(ComboBox4.Text);
			streamWriter.WriteLine(ComboBox5.Text);
			streamWriter.WriteLine(ComboBox7.Text);
			streamWriter.WriteLine(ComboBox8.Text);
			streamWriter.WriteLine(Print1.Text);
			streamWriter.WriteLine(Print2.Text);
			streamWriter.WriteLine(Print3.Text);
			streamWriter.WriteLine(Print4.Text);
			streamWriter.WriteLine(print5.Text);
			streamWriter.WriteLine(ComboBox_0.Text);
			streamWriter.WriteLine(ComboBox9.Text);
			streamWriter.WriteLine(ComboBox_1.Text);
			streamWriter.WriteLine(ComboBox_3.Text);
			streamWriter.WriteLine(ComboBox_2.Text);
			streamWriter.WriteLine(print6.Text);
			streamWriter.WriteLine(CheckBox1.Checked);
			streamWriter.WriteLine(ComboBox_5.Text);
			streamWriter.WriteLine(ComboBox_7.Text);
			streamWriter.WriteLine(ComboBox_8.Text);
			streamWriter.WriteLine(print7.Text);
			streamWriter.WriteLine(ComboBox_9.Text);
			streamWriter.WriteLine(print8.Text);
			streamWriter.WriteLine(print9.Text);
			streamWriter.WriteLine(CheckBoxBookingNotification.Checked);
			streamWriter.Close();
			SAVE_COPY();
			Module1.POWER_USED = CheckBox1.Checked;
			Module1.POWER_PORT = ComboBox_5.Text;
			Module1.CASH_PORT = ComboBox_7.Text;
			try
			{
				if (Module1.POWER_USED)
				{
					if (!MyProject.Forms.frmMain1.SerialPort1.IsOpen)
					{
						MyProject.Forms.frmMain1.SerialPort1.PortName = Module1.POWER_PORT;
						MyProject.Forms.frmMain1.SerialPort1.Encoding = Encoding.GetEncoding(1252);
						MyProject.Forms.frmMain1.SerialPort1.Open();
					}
					else
					{
						MyProject.Forms.frmMain1.SerialPort1.Close();
						MyProject.Forms.frmMain1.SerialPort1.PortName = Module1.POWER_PORT;
						MyProject.Forms.frmMain1.SerialPort1.Encoding = Encoding.GetEncoding(1252);
						MyProject.Forms.frmMain1.SerialPort1.Open();
					}
				}
				else if (MyProject.Forms.frmMain1.SerialPort1.IsOpen)
				{
					MyProject.Forms.frmMain1.SerialPort1.Close();
				}
			}
			catch (Exception projectError2)
			{
				ProjectData.SetProjectError(projectError2);
				ProjectData.ClearProjectError();
			}
			Cursor = Cursors.Default;
			Close();
		}
	}

	public void openfiles()
	{
		OpenFileDialog openFileDialog = OpenFileDialog1;
		openFileDialog.Filter = "JPG files (*.jpg)|*.jpg|JPEG files (*.jpeg)|*.jpeg |GIF files (*.Gif)|*.gif |BMP files (*.bmp)|*.bmp |PNG files (*.png)|*.png";
		openFileDialog.FilterIndex = 1;
		openFileDialog.InitialDirectory = "C:\\";
		openFileDialog.Title = "เป\u0e34ดไฟล\u0e4c....";
		openFileDialog = null;
		if (OpenFileDialog1.ShowDialog() == DialogResult.OK)
		{
			try
			{
				FileStream fileStream = new FileStream(OpenFileDialog1.FileName, FileMode.Open);
				byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
				fileStream.Read(array, 0, array.Length);
				fileStream.Close();
				MemoryStream memoryStream = new MemoryStream(array);
				myBitmap = new Bitmap(memoryStream);
				ResizePic();
				PictureBox1.Image = myBitmap;
				memoryStream.Close();
			}
			catch (Exception ex)
			{
				ProjectData.SetProjectError(ex);
				Exception ex2 = ex;
				MessageBox.Show(ex2.Message, "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				ProjectData.ClearProjectError();
			}
		}
	}

	public void ResizePic()
	{
		decimal num = default(decimal);
		int num2 = 4;
		checked
		{
			if (myBitmap.Width > PanelPic.Width - 4)
			{
				PictureBox1.Width = PanelPic.Width - num2;
				num = new decimal((double)PanelPic.Width / (double)myBitmap.Width * 100.0);
				PictureBox1.Height = Convert.ToInt32(decimal.Divide(decimal.Multiply(new decimal(myBitmap.Height), num), 100m));
			}
			else
			{
				PictureBox1.Width = myBitmap.Width;
				PictureBox1.Height = myBitmap.Height;
			}
			if (PictureBox1.Height > PanelPic.Height - num2)
			{
				PictureBox1.Height = PanelPic.Height - num2;
				num = new decimal((double)PictureBox1.Height / (double)PanelPic.Height * 100.0);
				PictureBox1.Width = Convert.ToInt32(decimal.Divide(decimal.Multiply(new decimal(PictureBox1.Width), num), 100m));
			}
		}
	}

	public void DeleteImage()
	{
		FileStream fileStream = new FileStream(Module1.Path_Program + "/NoImage.png", FileMode.Open);
		byte[] array = new byte[checked((int)(fileStream.Length - 1L) + 1)];
		fileStream.Read(array, 0, array.Length);
		fileStream.Close();
		MemoryStream memoryStream = new MemoryStream(array);
		myBitmap = new Bitmap(memoryStream);
		ResizePic();
		PictureBox1.Image = myBitmap;
		memoryStream.Close();
	}

	private void BPIC2_Click(object sender, EventArgs e)
	{
		DeleteImage();
	}

	private void BPIC1_Click(object sender, EventArgs e)
	{
		openfiles();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Module1.Pority = Conversions.ToInteger(TPority.Text);
		Module1.Set_room_pority();
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			Print1.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	public bool SelectShowPrinter()
	{
		Print_Report.PrintSet.UseEXDialog = true;
		if (Print_Report.PrintSet.ShowDialog() == DialogResult.OK)
		{
			return true;
		}
		return false;
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			Print2.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button6_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			Print3.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button9_Click(object sender, EventArgs e)
	{
		Print1.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		Print2.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Button5_Click(object sender, EventArgs e)
	{
		Print3.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Button7_Click(object sender, EventArgs e)
	{
		Print4.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Button8_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			Print4.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button10_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			print5.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button11_Click(object sender, EventArgs e)
	{
		print5.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Button12_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			print6.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button13_Click(object sender, EventArgs e)
	{
		print6.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void ComboBox13_SelectedIndexChanged(object sender, EventArgs e)
	{
		TextURL.Enabled = false;
		try
		{
			if (ComboBox_5.SelectedIndex == 0)
			{
				TextURL.Enabled = true;
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
		CheckBox2.Enabled = CheckBox1.Checked;
		CheckBox3.Enabled = CheckBox1.Checked;
	}

	private void Label43_Click(object sender, EventArgs e)
	{
	}

	private void Copy1_LostFocus(object sender, EventArgs e)
	{
		if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null), "", TextCompare: false))
		{
			NewLateBinding.LateSet(sender, null, "text", new object[1] { 1 }, null, null);
		}
		if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null))))
		{
			NewLateBinding.LateSet(sender, null, "text", new object[1] { 1 }, null, null);
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		try
		{
			MyProject.Forms.frmMain1.SerialPort2.PortName = ComboBox_7.Text;
			MyProject.Forms.frmMain1.SerialPort2.Open();
			MyProject.Forms.frmMain1.SerialPort2.WriteLine(Module1.CASH_PORT);
			MyProject.Forms.frmMain1.SerialPort2.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			MessageBox.Show("ไม\u0e48พบล\u0e34\u0e49นช\u0e31ก");
			ProjectData.ClearProjectError();
		}
	}

	private void ComboBox17_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Button14_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			print7.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button15_Click(object sender, EventArgs e)
	{
		print7.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		WindowState = FormWindowState.Maximized;
	}

	private void Button17_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			print8.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button19_Click(object sender, EventArgs e)
	{
		if (SelectShowPrinter())
		{
			print9.Text = Print_Report.PrintSet.PrinterSettings.PrinterName;
		}
	}

	private void Button16_Click(object sender, EventArgs e)
	{
		print8.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}

	private void Button18_Click(object sender, EventArgs e)
	{
		print9.Text = "เล\u0e37อกตอนพ\u0e34มพ\u0e4c";
	}
}
