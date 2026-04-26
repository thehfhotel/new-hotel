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
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmAddSale2 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("RequiredFieldValidator3")]
	private RequiredFieldValidator _RequiredFieldValidator3;

	[AccessedThroughProperty("RequiredFieldValidator4")]
	private RequiredFieldValidator _RequiredFieldValidator4;

	[AccessedThroughProperty("RequiredFieldValidator1")]
	private RequiredFieldValidator _RequiredFieldValidator1;

	[AccessedThroughProperty("RequiredFieldValidator2")]
	private RequiredFieldValidator _RequiredFieldValidator2;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("Highlighter1")]
	private Highlighter _Highlighter1;

	[AccessedThroughProperty("RequiredFieldValidator5")]
	private RequiredFieldValidator _RequiredFieldValidator5;

	[AccessedThroughProperty("CustomValidator1")]
	private CustomValidator _CustomValidator1;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Aaddress")]
	private TextBox _Aaddress;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("GroupBox2")]
	private GroupBox _GroupBox2;

	[AccessedThroughProperty("Adate")]
	private DateTimePicker _Adate;

	[AccessedThroughProperty("Ano")]
	private TextBox _Ano;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Tprice")]
	private TextBox _Tprice;

	[AccessedThroughProperty("Tnum")]
	private TextBox _Tnum;

	[AccessedThroughProperty("Tunit")]
	private TextBox _Tunit;

	[AccessedThroughProperty("Tname")]
	private TextBox _Tname;

	[AccessedThroughProperty("BsaveEdit")]
	private ButtonX _BsaveEdit;

	[AccessedThroughProperty("RequiredFieldValidator7")]
	private RequiredFieldValidator _RequiredFieldValidator7;

	[AccessedThroughProperty("RequiredFieldValidator6")]
	private RequiredFieldValidator _RequiredFieldValidator6;

	[AccessedThroughProperty("RequiredFieldValidator8")]
	private RequiredFieldValidator _RequiredFieldValidator8;

	[AccessedThroughProperty("AFax")]
	private TextBox _AFax;

	[AccessedThroughProperty("Atel")]
	private TextBox _Atel;

	[AccessedThroughProperty("Label17")]
	private Label _Label17;

	[AccessedThroughProperty("Label16")]
	private Label _Label16;

	[AccessedThroughProperty("Label22")]
	private Label _Label22;

	[AccessedThroughProperty("Label21")]
	private Label _Label21;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("ราคาท\u0e31\u0e49งหมด")]
	private TextBox textBox_0;

	[AccessedThroughProperty("ราคาvat")]
	private TextBox textBox_1;

	[AccessedThroughProperty("ราคารวม")]
	private TextBox textBox_2;

	[AccessedThroughProperty("Tvat")]
	private TextBox _Tvat;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Tno")]
	private TextBox _Tno;

	[AccessedThroughProperty("Bdel")]
	private ButtonX _Bdel;

	[AccessedThroughProperty("Label24")]
	private Label _Label24;

	[AccessedThroughProperty("Ttotal")]
	private TextBox _Ttotal;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Tno1")]
	private TextBox _Tno1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("AName")]
	private TextBox _AName;

	[AccessedThroughProperty("TimeScan")]
	private Timer _TimeScan;

	[AccessedThroughProperty("Cbnum")]
	private CheckBox _Cbnum;

	[AccessedThroughProperty("Label23")]
	private Label _Label23;

	[AccessedThroughProperty("Tdis")]
	private TextBox _Tdis;

	[AccessedThroughProperty("Label28")]
	private Label _Label28;

	[AccessedThroughProperty("ราคาลดพ\u0e34เศษ")]
	private TextBox textBox_3;

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

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("CheckVat")]
	private CheckBox _CheckVat;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Label31")]
	private Label _Label31;

	[AccessedThroughProperty("GroupPanel1")]
	private GroupPanel _GroupPanel1;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("Timer3")]
	private Timer _Timer3;

	[AccessedThroughProperty("Tref")]
	private TextBox _Tref;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("TCusType")]
	private ComboBox _TCusType;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	[AccessedThroughProperty("PanelEx27")]
	private PanelEx _PanelEx27;

	[AccessedThroughProperty("PanelEx21")]
	private PanelEx _PanelEx21;

	[AccessedThroughProperty("PanelEx22")]
	private PanelEx _PanelEx22;

	[AccessedThroughProperty("FlowLayoutPanel3")]
	private FlowLayoutPanel _FlowLayoutPanel3;

	[AccessedThroughProperty("PanelEx18")]
	private PanelEx _PanelEx18;

	[AccessedThroughProperty("PanelEx19")]
	private PanelEx _PanelEx19;

	[AccessedThroughProperty("PanelEx20")]
	private PanelEx _PanelEx20;

	[AccessedThroughProperty("PanelEx31")]
	private PanelEx _PanelEx31;

	[AccessedThroughProperty("PanelEx37")]
	private PanelEx _PanelEx37;

	[AccessedThroughProperty("PanelEx38")]
	private PanelEx _PanelEx38;

	[AccessedThroughProperty("PanelEx39")]
	private PanelEx _PanelEx39;

	[AccessedThroughProperty("FlowLayoutPanel1")]
	private FlowLayoutPanel _FlowLayoutPanel1;

	[AccessedThroughProperty("PanelEx34")]
	private PanelEx _PanelEx34;

	[AccessedThroughProperty("PanelEx35")]
	private PanelEx _PanelEx35;

	private object IdSelectNow;

	private object ItemEdit;

	public object IEdit;

	private string Gnow;

	private string CUST_ID;

	private int idsave;

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

	internal virtual Highlighter Highlighter1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Highlighter1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Highlighter1 = value;
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

	internal virtual TextBox Aaddress
	{
		[DebuggerNonUserCode]
		get
		{
			return _Aaddress;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Aaddress = value;
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

	internal virtual DateTimePicker Adate
	{
		[DebuggerNonUserCode]
		get
		{
			return _Adate;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Adate = value;
		}
	}

	internal virtual TextBox Ano
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ano;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ano = value;
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

	internal virtual TextBox Tprice
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tprice;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tnum_TextChanged;
			KeyPressEventHandler value3 = TabEnter;
			if (_Tprice != null)
			{
				_Tprice.TextChanged -= value2;
				_Tprice.KeyPress -= value3;
			}
			_Tprice = value;
			if (_Tprice != null)
			{
				_Tprice.TextChanged += value2;
				_Tprice.KeyPress += value3;
			}
		}
	}

	internal virtual TextBox Tnum
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tnum_TextChanged;
			KeyPressEventHandler value3 = TabEnter;
			if (_Tnum != null)
			{
				_Tnum.TextChanged -= value2;
				_Tnum.KeyPress -= value3;
			}
			_Tnum = value;
			if (_Tnum != null)
			{
				_Tnum.TextChanged += value2;
				_Tnum.KeyPress += value3;
			}
		}
	}

	internal virtual TextBox Tunit
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
			KeyPressEventHandler value2 = TabEnter;
			if (_Tunit != null)
			{
				_Tunit.KeyPress -= value2;
			}
			_Tunit = value;
			if (_Tunit != null)
			{
				_Tunit.KeyPress += value2;
			}
		}
	}

	internal virtual TextBox Tname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyPressEventHandler value2 = TabEnter;
			if (_Tname != null)
			{
				_Tname.KeyPress -= value2;
			}
			_Tname = value;
			if (_Tname != null)
			{
				_Tname.KeyPress += value2;
			}
		}
	}

	internal virtual ButtonX BsaveEdit
	{
		[DebuggerNonUserCode]
		get
		{
			return _BsaveEdit;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = BsaveEdit_Click;
			if (_BsaveEdit != null)
			{
				_BsaveEdit.Click -= value2;
			}
			_BsaveEdit = value;
			if (_BsaveEdit != null)
			{
				_BsaveEdit.Click += value2;
			}
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

	internal virtual TextBox AFax
	{
		[DebuggerNonUserCode]
		get
		{
			return _AFax;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_AFax = value;
		}
	}

	internal virtual TextBox Atel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Atel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Atel = value;
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

	internal virtual TextBox TextBox_1
	{
		[DebuggerNonUserCode]
		get
		{
			return textBox_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBox_1 = value;
		}
	}

	internal virtual TextBox TextBox_2
	{
		[DebuggerNonUserCode]
		get
		{
			return textBox_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBox_2 = value;
		}
	}

	internal virtual TextBox Tvat
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tvat;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tvat_TextChanged;
			if (_Tvat != null)
			{
				_Tvat.TextChanged -= value2;
			}
			_Tvat = value;
			if (_Tvat != null)
			{
				_Tvat.TextChanged += value2;
			}
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

	internal virtual TextBox Tno
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tno;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tno = value;
		}
	}

	internal virtual ButtonX Bdel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bdel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bdel_Click;
			if (_Bdel != null)
			{
				_Bdel.Click -= value2;
			}
			_Bdel = value;
			if (_Bdel != null)
			{
				_Bdel.Click += value2;
			}
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

	internal virtual TextBox Ttotal
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ttotal;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyPressEventHandler value2 = TabEnter;
			if (_Ttotal != null)
			{
				_Ttotal.KeyPress -= value2;
			}
			_Ttotal = value;
			if (_Ttotal != null)
			{
				_Ttotal.KeyPress += value2;
			}
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

	internal virtual TextBox Tno1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tno1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyPressEventHandler value2 = TabEnter;
			if (_Tno1 != null)
			{
				_Tno1.KeyPress -= value2;
			}
			_Tno1 = value;
			if (_Tno1 != null)
			{
				_Tno1.KeyPress += value2;
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

	internal virtual TextBox AName
	{
		[DebuggerNonUserCode]
		get
		{
			return _AName;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_AName = value;
		}
	}

	internal virtual Timer TimeScan
	{
		[DebuggerNonUserCode]
		get
		{
			return _TimeScan;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TimeScan = value;
		}
	}

	internal virtual CheckBox Cbnum
	{
		[DebuggerNonUserCode]
		get
		{
			return _Cbnum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Cbnum = value;
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

	internal virtual TextBox Tdis
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tdis;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tnum_TextChanged;
			KeyPressEventHandler value3 = TabEnter;
			if (_Tdis != null)
			{
				_Tdis.TextChanged -= value2;
				_Tdis.KeyPress -= value3;
			}
			_Tdis = value;
			if (_Tdis != null)
			{
				_Tdis.TextChanged += value2;
				_Tdis.KeyPress += value3;
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

	internal virtual TextBox TextBox_3
	{
		[DebuggerNonUserCode]
		get
		{
			return textBox_3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TextBox_3_TextChanged;
			if (textBox_3 != null)
			{
				textBox_3.TextChanged -= value2;
			}
			textBox_3 = value;
			if (textBox_3 != null)
			{
				textBox_3.TextChanged += value2;
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
			EventHandler value2 = ListView1_DoubleClick;
			KeyEventHandler value3 = ListView1_KeyDown;
			EventHandler value4 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick -= value2;
				_ListView1.KeyDown -= value3;
				_ListView1.SelectedIndexChanged -= value4;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick += value2;
				_ListView1.KeyDown += value3;
				_ListView1.SelectedIndexChanged += value4;
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

	internal virtual CheckBox CheckVat
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckVat;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = CheckVat_CheckedChanged;
			if (_CheckVat != null)
			{
				_CheckVat.CheckedChanged -= value2;
			}
			_CheckVat = value;
			if (_CheckVat != null)
			{
				_CheckVat.CheckedChanged += value2;
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

	internal virtual Timer Timer3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Timer3 = value;
		}
	}

	internal virtual TextBox Tref
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tref;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tref = value;
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

	internal virtual ComboBox TCusType
	{
		[DebuggerNonUserCode]
		get
		{
			return _TCusType;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TCusType = value;
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

	internal virtual PanelEx PanelEx_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx27;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx27 = value;
		}
	}

	internal virtual PanelEx PanelEx_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx21;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx21 = value;
		}
	}

	internal virtual PanelEx PanelEx_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx22;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx22 = value;
		}
	}

	internal virtual FlowLayoutPanel FlowLayoutPanel3
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			PaintEventHandler value2 = FlowLayoutPanel3_Paint;
			if (_FlowLayoutPanel3 != null)
			{
				_FlowLayoutPanel3.Paint -= value2;
			}
			_FlowLayoutPanel3 = value;
			if (_FlowLayoutPanel3 != null)
			{
				_FlowLayoutPanel3.Paint += value2;
			}
		}
	}

	internal virtual PanelEx PanelEx_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx18;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx18 = value;
		}
	}

	internal virtual PanelEx PanelEx_4
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx19;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx19 = value;
		}
	}

	internal virtual PanelEx PanelEx_5
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx20;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx20 = value;
		}
	}

	internal virtual PanelEx PanelEx_6
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx31;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx31 = value;
		}
	}

	internal virtual PanelEx PanelEx_7
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx37;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx37 = value;
		}
	}

	internal virtual PanelEx PanelEx_8
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx38;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx38 = value;
		}
	}

	internal virtual PanelEx PanelEx_9
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx39;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx39 = value;
		}
	}

	internal virtual FlowLayoutPanel FlowLayoutPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_FlowLayoutPanel1 = value;
		}
	}

	internal virtual PanelEx PanelEx_10
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx34;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx34 = value;
		}
	}

	internal virtual PanelEx PanelEx_11
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx35;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx35 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmAddSale2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmAddSale2()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmAddSale_FormClosing;
		base.Load += FrmAddSale_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		IdSelectNow = 0;
		ItemEdit = 0;
		IEdit = 0;
		Gnow = "";
		CUST_ID = "";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmAddSale2));
		this.RequiredFieldValidator4 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณาใส\u0e48 Password");
		this.RequiredFieldValidator3 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณาใส\u0e48 UserName");
		this.RequiredFieldValidator1 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.RequiredFieldValidator2 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.RequiredFieldValidator5 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อ Forecast");
		this.CustomValidator1 = new DevComponents.DotNetBar.Validator.CustomValidator();
		this.Highlighter1 = new DevComponents.DotNetBar.Validator.Highlighter();
		this.BsaveEdit = new DevComponents.DotNetBar.ButtonX();
		this.Tprice = new System.Windows.Forms.TextBox();
		this.Tnum = new System.Windows.Forms.TextBox();
		this.Tunit = new System.Windows.Forms.TextBox();
		this.Tname = new System.Windows.Forms.TextBox();
		this.Adate = new System.Windows.Forms.DateTimePicker();
		this.Ano = new System.Windows.Forms.TextBox();
		this.Aaddress = new System.Windows.Forms.TextBox();
		this.AFax = new System.Windows.Forms.TextBox();
		this.Atel = new System.Windows.Forms.TextBox();
		this.TextBox_2 = new System.Windows.Forms.TextBox();
		this.TextBox_1 = new System.Windows.Forms.TextBox();
		this.TextBox_0 = new System.Windows.Forms.TextBox();
		this.Tno = new System.Windows.Forms.TextBox();
		this.Ttotal = new System.Windows.Forms.TextBox();
		this.Tno1 = new System.Windows.Forms.TextBox();
		this.Tdis = new System.Windows.Forms.TextBox();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.AName = new System.Windows.Forms.TextBox();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.Tref = new System.Windows.Forms.TextBox();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.GroupPanel1 = new DevComponents.DotNetBar.Controls.GroupPanel();
		this.TextBox_3 = new System.Windows.Forms.TextBox();
		this.Label31 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label28 = new System.Windows.Forms.Label();
		this.CheckVat = new System.Windows.Forms.CheckBox();
		this.Label21 = new System.Windows.Forms.Label();
		this.Tvat = new System.Windows.Forms.TextBox();
		this.Label22 = new System.Windows.Forms.Label();
		this.GroupBox2 = new System.Windows.Forms.GroupBox();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label23 = new System.Windows.Forms.Label();
		this.Cbnum = new System.Windows.Forms.CheckBox();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.Bdel = new DevComponents.DotNetBar.ButtonX();
		this.Label24 = new System.Windows.Forms.Label();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label9 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.TCusType = new System.Windows.Forms.ComboBox();
		this.Label14 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label17 = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label16 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.RequiredFieldValidator6 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.RequiredFieldValidator7 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.RequiredFieldValidator8 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.TimeScan = new System.Windows.Forms.Timer(this.components);
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.Timer3 = new System.Windows.Forms.Timer(this.components);
		this.FlowLayoutPanel1 = new System.Windows.Forms.FlowLayoutPanel();
		this.PanelEx_10 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_11 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_8 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_7 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_9 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_0 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_1 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_2 = new DevComponents.DotNetBar.PanelEx();
		this.FlowLayoutPanel3 = new System.Windows.Forms.FlowLayoutPanel();
		this.PanelEx_3 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_4 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_5 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx_6 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx1.SuspendLayout();
		this.GroupPanel1.SuspendLayout();
		this.GroupBox2.SuspendLayout();
		this.GroupBox1.SuspendLayout();
		this.FlowLayoutPanel1.SuspendLayout();
		this.PanelEx_7.SuspendLayout();
		this.PanelEx_0.SuspendLayout();
		this.FlowLayoutPanel3.SuspendLayout();
		this.SuspendLayout();
		this.RequiredFieldValidator4.ErrorMessage = "กร\u0e38ณาใส\u0e48 Password";
		this.RequiredFieldValidator4.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator3.ErrorMessage = "กร\u0e38ณาใส\u0e48 UserName";
		this.RequiredFieldValidator3.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator1.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator1.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator2.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator2.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.OpenFileDialog1.FileName = "OpenFileDialog1";
		this.RequiredFieldValidator5.ErrorMessage = "กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อ Forecast";
		this.RequiredFieldValidator5.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.CustomValidator1.ErrorMessage = "Your error message here.";
		this.CustomValidator1.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.Highlighter1.ContainerControl = this;
		this.Highlighter1.FocusHighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Orange;
		this.BsaveEdit.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.BsaveEdit.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.BsaveEdit.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.BsaveEdit.Enabled = false;
		this.BsaveEdit.FocusCuesEnabled = false;
		this.Highlighter1.SetHighlightOnFocus(this.BsaveEdit, true);
		DevComponents.DotNetBar.ButtonX bsaveEdit = this.BsaveEdit;
		System.Drawing.Point location = new System.Drawing.Point(647, 38);
		bsaveEdit.Location = location;
		DevComponents.DotNetBar.ButtonX bsaveEdit2 = this.BsaveEdit;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bsaveEdit2.Margin = margin;
		this.BsaveEdit.Name = "BsaveEdit";
		this.BsaveEdit.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX bsaveEdit3 = this.BsaveEdit;
		System.Drawing.Size size = new System.Drawing.Size(45, 22);
		bsaveEdit3.Size = size;
		this.BsaveEdit.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.BsaveEdit.TabIndex = 10;
		this.BsaveEdit.Text = "แก\u0e49ไข";
		this.Tprice.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Tprice.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tprice, true);
		System.Windows.Forms.TextBox tprice = this.Tprice;
		location = new System.Drawing.Point(511, 38);
		tprice.Location = location;
		System.Windows.Forms.TextBox tprice2 = this.Tprice;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tprice2.Margin = margin;
		this.Tprice.Name = "Tprice";
		System.Windows.Forms.TextBox tprice3 = this.Tprice;
		size = new System.Drawing.Size(62, 22);
		tprice3.Size = size;
		this.Tprice.TabIndex = 7;
		this.Tprice.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tnum.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Tnum.BackColor = System.Drawing.Color.White;
		this.Tnum.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tnum, true);
		System.Windows.Forms.TextBox tnum = this.Tnum;
		location = new System.Drawing.Point(448, 38);
		tnum.Location = location;
		System.Windows.Forms.TextBox tnum2 = this.Tnum;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tnum2.Margin = margin;
		this.Tnum.Name = "Tnum";
		System.Windows.Forms.TextBox tnum3 = this.Tnum;
		size = new System.Drawing.Size(64, 22);
		tnum3.Size = size;
		this.Tnum.TabIndex = 6;
		this.Tnum.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tunit.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Tunit.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tunit.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tunit, true);
		System.Windows.Forms.TextBox tunit = this.Tunit;
		location = new System.Drawing.Point(383, 38);
		tunit.Location = location;
		System.Windows.Forms.TextBox tunit2 = this.Tunit;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tunit2.Margin = margin;
		this.Tunit.Name = "Tunit";
		this.Tunit.ReadOnly = true;
		System.Windows.Forms.TextBox tunit3 = this.Tunit;
		size = new System.Drawing.Size(66, 22);
		tunit3.Size = size;
		this.Tunit.TabIndex = 4;
		this.Tname.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Tname.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tname.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tname, true);
		System.Windows.Forms.TextBox tname = this.Tname;
		location = new System.Drawing.Point(266, 38);
		tname.Location = location;
		System.Windows.Forms.TextBox tname2 = this.Tname;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tname2.Margin = margin;
		this.Tname.Name = "Tname";
		this.Tname.ReadOnly = true;
		System.Windows.Forms.TextBox tname3 = this.Tname;
		size = new System.Drawing.Size(119, 22);
		tname3.Size = size;
		this.Tname.TabIndex = 3;
		this.Highlighter1.SetHighlightOnFocus(this.Adate, true);
		System.Windows.Forms.DateTimePicker adate = this.Adate;
		location = new System.Drawing.Point(472, 50);
		adate.Location = location;
		System.Windows.Forms.DateTimePicker adate2 = this.Adate;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		adate2.Margin = margin;
		this.Adate.Name = "Adate";
		System.Windows.Forms.DateTimePicker adate3 = this.Adate;
		size = new System.Drawing.Size(166, 22);
		adate3.Size = size;
		this.Adate.TabIndex = 6;
		this.Ano.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Ano.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Ano, true);
		System.Windows.Forms.TextBox ano = this.Ano;
		location = new System.Drawing.Point(473, 24);
		ano.Location = location;
		System.Windows.Forms.TextBox ano2 = this.Ano;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		ano2.Margin = margin;
		this.Ano.Name = "Ano";
		this.Ano.ReadOnly = true;
		System.Windows.Forms.TextBox ano3 = this.Ano;
		size = new System.Drawing.Size(165, 22);
		ano3.Size = size;
		this.Ano.TabIndex = 5;
		this.Aaddress.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Aaddress.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Aaddress, true);
		System.Windows.Forms.TextBox aaddress = this.Aaddress;
		location = new System.Drawing.Point(97, 47);
		aaddress.Location = location;
		System.Windows.Forms.TextBox aaddress2 = this.Aaddress;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		aaddress2.Margin = margin;
		this.Aaddress.Multiline = true;
		this.Aaddress.Name = "Aaddress";
		this.Aaddress.ReadOnly = true;
		this.Aaddress.ScrollBars = System.Windows.Forms.ScrollBars.Both;
		System.Windows.Forms.TextBox aaddress3 = this.Aaddress;
		size = new System.Drawing.Size(288, 25);
		aaddress3.Size = size;
		this.Aaddress.TabIndex = 2;
		this.AFax.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.AFax.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.AFax, true);
		System.Windows.Forms.TextBox aFax = this.AFax;
		location = new System.Drawing.Point(273, 75);
		aFax.Location = location;
		System.Windows.Forms.TextBox aFax2 = this.AFax;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		aFax2.Margin = margin;
		this.AFax.Name = "AFax";
		this.AFax.ReadOnly = true;
		System.Windows.Forms.TextBox aFax3 = this.AFax;
		size = new System.Drawing.Size(112, 22);
		aFax3.Size = size;
		this.AFax.TabIndex = 4;
		this.Atel.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Atel.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Atel, true);
		System.Windows.Forms.TextBox atel = this.Atel;
		location = new System.Drawing.Point(97, 75);
		atel.Location = location;
		System.Windows.Forms.TextBox atel2 = this.Atel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		atel2.Margin = margin;
		this.Atel.Name = "Atel";
		this.Atel.ReadOnly = true;
		System.Windows.Forms.TextBox atel3 = this.Atel;
		size = new System.Drawing.Size(111, 22);
		atel3.Size = size;
		this.Atel.TabIndex = 3;
		this.TextBox_2.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TextBox_2.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TextBox_2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox_2.ForeColor = System.Drawing.Color.Blue;
		this.Highlighter1.SetHighlightOnFocus(this.TextBox_2, true);
		System.Windows.Forms.TextBox textBox = this.TextBox_2;
		location = new System.Drawing.Point(19, 19);
		textBox.Location = location;
		System.Windows.Forms.TextBox textBox2 = this.TextBox_2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox2.Margin = margin;
		this.TextBox_2.Name = "ราคารวม";
		this.TextBox_2.ReadOnly = true;
		System.Windows.Forms.TextBox textBox3 = this.TextBox_2;
		size = new System.Drawing.Size(140, 30);
		textBox3.Size = size;
		this.TextBox_2.TabIndex = 14;
		this.TextBox_2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.TextBox_1.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TextBox_1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TextBox_1.Enabled = false;
		this.TextBox_1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox_1.ForeColor = System.Drawing.Color.Blue;
		this.Highlighter1.SetHighlightOnFocus(this.TextBox_1, true);
		System.Windows.Forms.TextBox textBox4 = this.TextBox_1;
		location = new System.Drawing.Point(23, 412);
		textBox4.Location = location;
		System.Windows.Forms.TextBox textBox5 = this.TextBox_1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox5.Margin = margin;
		this.TextBox_1.Name = "ราคาvat";
		this.TextBox_1.ReadOnly = true;
		System.Windows.Forms.TextBox textBox6 = this.TextBox_1;
		size = new System.Drawing.Size(140, 30);
		textBox6.Size = size;
		this.TextBox_1.TabIndex = 16;
		this.TextBox_1.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.TextBox_1.Visible = false;
		this.TextBox_0.BackColor = System.Drawing.Color.Black;
		this.TextBox_0.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TextBox_0.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox_0.ForeColor = System.Drawing.Color.Lime;
		this.Highlighter1.SetHighlightOnFocus(this.TextBox_0, true);
		System.Windows.Forms.TextBox textBox7 = this.TextBox_0;
		location = new System.Drawing.Point(8, 137);
		textBox7.Location = location;
		System.Windows.Forms.TextBox textBox8 = this.TextBox_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox8.Margin = margin;
		this.TextBox_0.Name = "ราคาท\u0e31\u0e49งหมด";
		this.TextBox_0.ReadOnly = true;
		System.Windows.Forms.TextBox textBox9 = this.TextBox_0;
		size = new System.Drawing.Size(156, 40);
		textBox9.Size = size;
		this.TextBox_0.TabIndex = 17;
		this.TextBox_0.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.TextBox_0.Visible = false;
		this.Tno.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Tno.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tno, true);
		System.Windows.Forms.TextBox tno = this.Tno;
		location = new System.Drawing.Point(11, 38);
		tno.Location = location;
		System.Windows.Forms.TextBox tno2 = this.Tno;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tno2.Margin = margin;
		this.Tno.Name = "Tno";
		System.Windows.Forms.TextBox tno3 = this.Tno;
		size = new System.Drawing.Size(132, 22);
		tno3.Size = size;
		this.Tno.TabIndex = 0;
		this.Ttotal.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Ttotal.BackColor = System.Drawing.Color.FromArgb(255, 192, 255);
		this.Ttotal.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Ttotal, true);
		System.Windows.Forms.TextBox ttotal = this.Ttotal;
		location = new System.Drawing.Point(572, 38);
		ttotal.Location = location;
		System.Windows.Forms.TextBox ttotal2 = this.Ttotal;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		ttotal2.Margin = margin;
		this.Ttotal.Name = "Ttotal";
		this.Ttotal.ReadOnly = true;
		System.Windows.Forms.TextBox ttotal3 = this.Ttotal;
		size = new System.Drawing.Size(75, 22);
		ttotal3.Size = size;
		this.Ttotal.TabIndex = 9;
		this.Ttotal.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tno1.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.Tno1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tno1, true);
		System.Windows.Forms.TextBox tno4 = this.Tno1;
		location = new System.Drawing.Point(206, 38);
		tno4.Location = location;
		System.Windows.Forms.TextBox tno5 = this.Tno1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tno5.Margin = margin;
		this.Tno1.Name = "Tno1";
		this.Tno1.ReadOnly = true;
		System.Windows.Forms.TextBox tno6 = this.Tno1;
		size = new System.Drawing.Size(61, 22);
		tno6.Size = size;
		this.Tno1.TabIndex = 2;
		this.Tdis.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Tdis.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tdis, true);
		System.Windows.Forms.TextBox tdis = this.Tdis;
		location = new System.Drawing.Point(133, 405);
		tdis.Location = location;
		System.Windows.Forms.TextBox tdis2 = this.Tdis;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tdis2.Margin = margin;
		this.Tdis.Name = "Tdis";
		System.Windows.Forms.TextBox tdis3 = this.Tdis;
		size = new System.Drawing.Size(62, 22);
		tdis3.Size = size;
		this.Tdis.TabIndex = 8;
		this.Tdis.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.Tdis.Visible = false;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[10] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader6, this.ColumnHeader5, this.ColumnHeader7, this.ColumnHeader10, this.ColumnHeader8, this.ColumnHeader9 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		this.Highlighter1.SetHighlightOnFocus(this.ListView1, true);
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(10, 63);
		listView.Location = location;
		System.Windows.Forms.ListView listView2 = this.ListView1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView3 = this.ListView1;
		size = new System.Drawing.Size(682, 326);
		listView3.Size = size;
		this.ListView1.TabIndex = 24;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "";
		this.ColumnHeader1.Width = 0;
		this.ColumnHeader2.Text = "ลำด\u0e31บ";
		this.ColumnHeader2.Width = 40;
		this.ColumnHeader3.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		this.ColumnHeader3.Width = 80;
		this.ColumnHeader4.Text = "รายละเอ\u0e35ยดส\u0e34นค\u0e49\u0e48า";
		this.ColumnHeader4.Width = 220;
		this.ColumnHeader6.Text = "หน\u0e48วย";
		this.ColumnHeader6.Width = 80;
		this.ColumnHeader5.Text = "จำนวน";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 80;
		this.ColumnHeader7.Text = "ราคาต\u0e48อหน\u0e48วย";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 80;
		this.ColumnHeader10.Text = "ส\u0e48วนลด %";
		this.ColumnHeader10.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader10.Width = 0;
		this.ColumnHeader8.Text = "ส\u0e48วนลด";
		this.ColumnHeader8.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader8.Width = 0;
		this.ColumnHeader9.Text = "จำนวนเง\u0e34น";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 80;
		this.AName.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.AName.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.AName, true);
		System.Windows.Forms.TextBox aName = this.AName;
		location = new System.Drawing.Point(97, 22);
		aName.Location = location;
		System.Windows.Forms.TextBox aName2 = this.AName;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		aName2.Margin = margin;
		this.AName.Name = "AName";
		this.AName.ReadOnly = true;
		System.Windows.Forms.TextBox aName3 = this.AName;
		size = new System.Drawing.Size(257, 22);
		aName3.Size = size;
		this.AName.TabIndex = 1;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Highlighter1.SetHighlightOnFocus(this.ButtonX3, true);
		this.ButtonX3.Image = iHOTEL2025.My.Resources.Resources.print1;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX3;
		location = new System.Drawing.Point(279, 393);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX3;
		size = new System.Drawing.Size(204, 36);
		buttonX3.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 22;
		this.ButtonX3.Text = "บ\u0e31นท\u0e36ก /พ\u0e34มพ\u0e4cใบเสร\u0e47จร\u0e31บเง\u0e34น";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.Highlighter1.SetHighlightOnFocus(this.ButtonX2, true);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		location = new System.Drawing.Point(487, 393);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX5.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX2;
		size = new System.Drawing.Size(100, 36);
		buttonX6.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "บ\u0e31นท\u0e36ก\r\n";
		this.Tref.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Tref.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tref, true);
		System.Windows.Forms.TextBox tref = this.Tref;
		location = new System.Drawing.Point(473, 77);
		tref.Location = location;
		System.Windows.Forms.TextBox tref2 = this.Tref;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tref2.Margin = margin;
		this.Tref.Name = "Tref";
		this.Tref.ReadOnly = true;
		System.Windows.Forms.TextBox tref3 = this.Tref;
		size = new System.Drawing.Size(165, 22);
		tref3.Size = size;
		this.Tref.TabIndex = 13;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.PanelEx_0);
		this.PanelEx1.Controls.Add(this.FlowLayoutPanel3);
		this.PanelEx1.Controls.Add(this.PanelEx_7);
		this.PanelEx1.Controls.Add(this.FlowLayoutPanel1);
		this.PanelEx1.Controls.Add(this.GroupPanel1);
		this.PanelEx1.Controls.Add(this.GroupBox2);
		this.PanelEx1.Controls.Add(this.GroupBox1);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		size = new System.Drawing.Size(1224, 588);
		panelEx3.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.GroupPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.GroupPanel1.CanvasColor = System.Drawing.SystemColors.Control;
		this.GroupPanel1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.GroupPanel1.Controls.Add(this.TextBox_3);
		this.GroupPanel1.Controls.Add(this.Label31);
		this.GroupPanel1.Controls.Add(this.TextBox_2);
		this.GroupPanel1.Controls.Add(this.Label12);
		this.GroupPanel1.Controls.Add(this.Label28);
		this.GroupPanel1.Controls.Add(this.CheckVat);
		this.GroupPanel1.Controls.Add(this.Label21);
		this.GroupPanel1.Controls.Add(this.Tvat);
		this.GroupPanel1.Controls.Add(this.TextBox_1);
		this.GroupPanel1.Controls.Add(this.TextBox_0);
		this.GroupPanel1.Controls.Add(this.Label22);
		this.GroupPanel1.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel = this.GroupPanel1;
		location = new System.Drawing.Point(1042, 17);
		groupPanel.Location = location;
		this.GroupPanel1.Name = "GroupPanel1";
		DevComponents.DotNetBar.Controls.GroupPanel groupPanel2 = this.GroupPanel1;
		size = new System.Drawing.Size(168, 116);
		groupPanel2.Size = size;
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
		this.GroupPanel1.TabIndex = 22;
		this.GroupPanel1.Text = "PRICE";
		this.TextBox_3.BackColor = System.Drawing.Color.White;
		this.TextBox_3.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.TextBox_3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.TextBox_3.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox textBox10 = this.TextBox_3;
		location = new System.Drawing.Point(19, 75);
		textBox10.Location = location;
		System.Windows.Forms.TextBox textBox11 = this.TextBox_3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox11.Margin = margin;
		this.TextBox_3.Name = "ราคาลดพ\u0e34เศษ";
		System.Windows.Forms.TextBox textBox12 = this.TextBox_3;
		size = new System.Drawing.Size(140, 30);
		textBox12.Size = size;
		this.TextBox_3.TabIndex = 23;
		this.TextBox_3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.TextBox_3.Visible = false;
		this.Label31.AutoSize = true;
		System.Windows.Forms.Label label = this.Label31;
		location = new System.Drawing.Point(80, 383);
		label.Location = location;
		this.Label31.Name = "Label31";
		System.Windows.Forms.Label label2 = this.Label31;
		size = new System.Drawing.Size(33, 25);
		label2.Size = size;
		this.Label31.TabIndex = 26;
		this.Label31.Text = "%";
		this.Label31.Visible = false;
		this.Label12.AutoSize = true;
		this.Label12.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label3 = this.Label12;
		location = new System.Drawing.Point(0, -5);
		label3.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label4 = this.Label12;
		size = new System.Drawing.Size(86, 25);
		label4.Size = size;
		this.Label12.TabIndex = 9;
		this.Label12.Text = "ราคารวม";
		this.Label28.AutoSize = true;
		this.Label28.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label5 = this.Label28;
		location = new System.Drawing.Point(-1, 51);
		label5.Location = location;
		this.Label28.Name = "Label28";
		System.Windows.Forms.Label label6 = this.Label28;
		size = new System.Drawing.Size(129, 25);
		label6.Size = size;
		this.Label28.TabIndex = 22;
		this.Label28.Text = "ส\u0e48วนลดการค\u0e49า";
		this.Label28.Visible = false;
		this.CheckVat.AutoSize = true;
		this.CheckVat.Checked = true;
		this.CheckVat.CheckState = System.Windows.Forms.CheckState.Checked;
		this.CheckVat.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.CheckBox checkVat = this.CheckVat;
		location = new System.Drawing.Point(9, 353);
		checkVat.Location = location;
		this.CheckVat.Name = "CheckVat";
		System.Windows.Forms.CheckBox checkVat2 = this.CheckVat;
		size = new System.Drawing.Size(166, 23);
		checkVat2.Size = size;
		this.CheckVat.TabIndex = 25;
		this.CheckVat.Text = "ค\u0e34ดภาษ\u0e35ในราคาส\u0e34นค\u0e49า";
		this.CheckVat.UseVisualStyleBackColor = true;
		this.CheckVat.Visible = false;
		this.Label21.AutoSize = true;
		this.Label21.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label7 = this.Label21;
		location = new System.Drawing.Point(3, 324);
		label7.Location = location;
		this.Label21.Name = "Label21";
		System.Windows.Forms.Label label8 = this.Label21;
		size = new System.Drawing.Size(135, 25);
		label8.Size = size;
		this.Label21.TabIndex = 9;
		this.Label21.Text = "ภาษ\u0e35ม\u0e39ลค\u0e48าเพ\u0e34\u0e48ม";
		this.Label21.Visible = false;
		this.Tvat.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tvat.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.TextBox tvat = this.Tvat;
		location = new System.Drawing.Point(23, 383);
		tvat.Location = location;
		System.Windows.Forms.TextBox tvat2 = this.Tvat;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tvat2.Margin = margin;
		this.Tvat.Name = "Tvat";
		System.Windows.Forms.TextBox tvat3 = this.Tvat;
		size = new System.Drawing.Size(55, 30);
		tvat3.Size = size;
		this.Tvat.TabIndex = 15;
		this.Tvat.Text = "7";
		this.Tvat.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Tvat.Visible = false;
		this.Label22.AutoSize = true;
		this.Label22.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label22.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label9 = this.Label22;
		location = new System.Drawing.Point(4, 109);
		label9.Location = location;
		this.Label22.Name = "Label22";
		System.Windows.Forms.Label label10 = this.Label22;
		size = new System.Drawing.Size(125, 25);
		label10.Size = size;
		this.Label22.TabIndex = 9;
		this.Label22.Text = "ราคารวมส\u0e38ทธ\u0e34";
		this.Label22.Visible = false;
		this.GroupBox2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox2.Controls.Add(this.Tno1);
		this.GroupBox2.Controls.Add(this.ButtonX3);
		this.GroupBox2.Controls.Add(this.ButtonX1);
		this.GroupBox2.Controls.Add(this.ButtonX2);
		this.GroupBox2.Controls.Add(this.ListView1);
		this.GroupBox2.Controls.Add(this.Label23);
		this.GroupBox2.Controls.Add(this.Tdis);
		this.GroupBox2.Controls.Add(this.Cbnum);
		this.GroupBox2.Controls.Add(this.ButtonX7);
		this.GroupBox2.Controls.Add(this.BsaveEdit);
		this.GroupBox2.Controls.Add(this.Bdel);
		this.GroupBox2.Controls.Add(this.Label24);
		this.GroupBox2.Controls.Add(this.Label11);
		this.GroupBox2.Controls.Add(this.Label10);
		this.GroupBox2.Controls.Add(this.Label9);
		this.GroupBox2.Controls.Add(this.Label4);
		this.GroupBox2.Controls.Add(this.Label8);
		this.GroupBox2.Controls.Add(this.Ttotal);
		this.GroupBox2.Controls.Add(this.Tprice);
		this.GroupBox2.Controls.Add(this.Tnum);
		this.GroupBox2.Controls.Add(this.Tunit);
		this.GroupBox2.Controls.Add(this.Tname);
		this.GroupBox2.Controls.Add(this.Label7);
		this.GroupBox2.Controls.Add(this.Tno);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox2;
		location = new System.Drawing.Point(17, 139);
		groupBox.Location = location;
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox2.Margin = margin;
		this.GroupBox2.Name = "GroupBox2";
		System.Windows.Forms.GroupBox groupBox3 = this.GroupBox2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox3.Padding = margin;
		System.Windows.Forms.GroupBox groupBox4 = this.GroupBox2;
		size = new System.Drawing.Size(703, 436);
		groupBox4.Size = size;
		this.GroupBox2.TabIndex = 1;
		this.GroupBox2.TabStop = false;
		this.GroupBox2.Text = "รายการ";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.ButtonX1.Image = iHOTEL2025.My.Resources.Resources.delete;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX1;
		location = new System.Drawing.Point(591, 393);
		buttonX7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX8.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX1;
		size = new System.Drawing.Size(100, 36);
		buttonX9.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 3;
		this.ButtonX1.Text = "ยกเล\u0e34ก\r\n";
		this.Label23.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label23.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label23;
		location = new System.Drawing.Point(133, 388);
		label11.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label12 = this.Label23;
		size = new System.Drawing.Size(49, 14);
		label12.Size = size;
		this.Label23.TabIndex = 19;
		this.Label23.Text = "ส\u0e48วนลด :";
		this.Label23.Visible = false;
		this.Cbnum.AutoSize = true;
		System.Windows.Forms.CheckBox cbnum = this.Cbnum;
		location = new System.Drawing.Point(222, 43);
		cbnum.Location = location;
		this.Cbnum.Name = "Cbnum";
		System.Windows.Forms.CheckBox cbnum2 = this.Cbnum;
		size = new System.Drawing.Size(15, 14);
		cbnum2.Size = size;
		this.Cbnum.TabIndex = 18;
		this.Cbnum.UseVisualStyleBackColor = true;
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.Image = iHOTEL2025.My.Resources.Resources.search;
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX7;
		location = new System.Drawing.Point(142, 38);
		buttonX10.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		this.ButtonX7.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor();
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX7;
		size = new System.Drawing.Size(53, 22);
		buttonX11.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 1;
		this.ButtonX7.Text = "ค\u0e49นหา";
		this.Bdel.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bdel.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Bdel.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bdel.FocusCuesEnabled = false;
		this.Bdel.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bdel = this.Bdel;
		location = new System.Drawing.Point(10, 391);
		bdel.Location = location;
		DevComponents.DotNetBar.ButtonX bdel2 = this.Bdel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bdel2.Margin = margin;
		this.Bdel.Name = "Bdel";
		this.Bdel.Shape = new DevComponents.DotNetBar.RoundRectangleShapeDescriptor(2);
		DevComponents.DotNetBar.ButtonX bdel3 = this.Bdel;
		size = new System.Drawing.Size(54, 24);
		bdel3.Size = size;
		this.Bdel.Style = DevComponents.DotNetBar.eDotNetBarStyle.Windows7;
		this.Bdel.TabIndex = 13;
		this.Bdel.Text = "ลบ";
		this.Label24.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label24.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label24;
		location = new System.Drawing.Point(589, 22);
		label13.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label14 = this.Label24;
		size = new System.Drawing.Size(25, 14);
		label14.Size = size;
		this.Label24.TabIndex = 0;
		this.Label24.Text = "รวม";
		this.Label11.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label15 = this.Label11;
		location = new System.Drawing.Point(514, 22);
		label15.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label16 = this.Label11;
		size = new System.Drawing.Size(31, 14);
		label16.Size = size;
		this.Label11.TabIndex = 0;
		this.Label11.Text = "ราคา";
		this.Label10.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label17 = this.Label10;
		location = new System.Drawing.Point(451, 22);
		label17.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label18 = this.Label10;
		size = new System.Drawing.Size(40, 14);
		label18.Size = size;
		this.Label10.TabIndex = 0;
		this.Label10.Text = "จำนวน";
		this.Label9.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label9.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label9;
		location = new System.Drawing.Point(386, 22);
		label19.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label20 = this.Label9;
		size = new System.Drawing.Size(35, 14);
		label20.Size = size;
		this.Label9.TabIndex = 0;
		this.Label9.Text = "หน\u0e48วย";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label21 = this.Label4;
		location = new System.Drawing.Point(271, 22);
		label21.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label22 = this.Label4;
		size = new System.Drawing.Size(49, 14);
		label22.Size = size;
		this.Label4.TabIndex = 0;
		this.Label4.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label23 = this.Label8;
		location = new System.Drawing.Point(209, 22);
		label23.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label24 = this.Label8;
		size = new System.Drawing.Size(54, 14);
		label24.Size = size;
		this.Label8.TabIndex = 0;
		this.Label8.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		this.Label7.AutoSize = true;
		this.Label7.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label25 = this.Label7;
		location = new System.Drawing.Point(10, 19);
		label25.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label26 = this.Label7;
		size = new System.Drawing.Size(54, 14);
		label26.Size = size;
		this.Label7.TabIndex = 0;
		this.Label7.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.TCusType);
		this.GroupBox1.Controls.Add(this.Label14);
		this.GroupBox1.Controls.Add(this.Tref);
		this.GroupBox1.Controls.Add(this.Label13);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Controls.Add(this.Adate);
		this.GroupBox1.Controls.Add(this.Ano);
		this.GroupBox1.Controls.Add(this.AFax);
		this.GroupBox1.Controls.Add(this.AName);
		this.GroupBox1.Controls.Add(this.Atel);
		this.GroupBox1.Controls.Add(this.Aaddress);
		this.GroupBox1.Controls.Add(this.Label17);
		this.GroupBox1.Controls.Add(this.Label6);
		this.GroupBox1.Controls.Add(this.Label16);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label2);
		System.Windows.Forms.GroupBox groupBox5 = this.GroupBox1;
		location = new System.Drawing.Point(17, 28);
		groupBox5.Location = location;
		System.Windows.Forms.GroupBox groupBox6 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox6.Margin = margin;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox7 = this.GroupBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		groupBox7.Padding = margin;
		System.Windows.Forms.GroupBox groupBox8 = this.GroupBox1;
		size = new System.Drawing.Size(1018, 105);
		groupBox8.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ข\u0e49อม\u0e39ลเอกสาร";
		this.TCusType.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.TCusType.FormattingEnabled = true;
		System.Windows.Forms.ComboBox tCusType = this.TCusType;
		location = new System.Drawing.Point(707, 24);
		tCusType.Location = location;
		System.Windows.Forms.ComboBox tCusType2 = this.TCusType;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tCusType2.Margin = margin;
		this.TCusType.Name = "TCusType";
		System.Windows.Forms.ComboBox tCusType3 = this.TCusType;
		size = new System.Drawing.Size(144, 22);
		tCusType3.Size = size;
		this.TCusType.TabIndex = 53;
		this.Label14.AutoSize = true;
		System.Windows.Forms.Label label27 = this.Label14;
		location = new System.Drawing.Point(643, 28);
		label27.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label28 = this.Label14;
		size = new System.Drawing.Size(55, 14);
		label28.Size = size;
		this.Label14.TabIndex = 52;
		this.Label14.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.Label13.AutoSize = true;
		System.Windows.Forms.Label label29 = this.Label13;
		location = new System.Drawing.Point(397, 81);
		label29.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label30 = this.Label13;
		size = new System.Drawing.Size(74, 14);
		label30.Size = size;
		this.Label13.TabIndex = 12;
		this.Label13.Text = "เลขท\u0e35\u0e48อ\u0e49างอ\u0e34ง :";
		this.Button1.Image = iHOTEL2025.My.Resources.Resources.search;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(355, 21);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(30, 23);
		button2.Size = size;
		this.Button1.TabIndex = 11;
		this.Button1.UseVisualStyleBackColor = true;
		this.Label17.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label17;
		location = new System.Drawing.Point(228, 80);
		label31.Location = location;
		this.Label17.Name = "Label17";
		System.Windows.Forms.Label label32 = this.Label17;
		size = new System.Drawing.Size(36, 14);
		label32.Size = size;
		this.Label17.TabIndex = 0;
		this.Label17.Text = "FAX :";
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label33 = this.Label6;
		location = new System.Drawing.Point(434, 54);
		label33.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label34 = this.Label6;
		size = new System.Drawing.Size(37, 14);
		label34.Size = size;
		this.Label6.TabIndex = 0;
		this.Label6.Text = "ว\u0e31นท\u0e35\u0e48 :";
		this.Label16.AutoSize = true;
		System.Windows.Forms.Label label35 = this.Label16;
		location = new System.Drawing.Point(62, 79);
		label35.Location = location;
		this.Label16.Name = "Label16";
		System.Windows.Forms.Label label36 = this.Label16;
		size = new System.Drawing.Size(31, 14);
		label36.Size = size;
		this.Label16.TabIndex = 0;
		this.Label16.Text = "Tel :";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label37 = this.Label5;
		location = new System.Drawing.Point(429, 28);
		label37.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label38 = this.Label5;
		size = new System.Drawing.Size(42, 14);
		label38.Size = size;
		this.Label5.TabIndex = 0;
		this.Label5.Text = "เลขท\u0e35\u0e48 :";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label39 = this.Label3;
		location = new System.Drawing.Point(57, 52);
		label39.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label40 = this.Label3;
		size = new System.Drawing.Size(38, 14);
		label40.Size = size;
		this.Label3.TabIndex = 0;
		this.Label3.Text = "ท\u0e35\u0e48อย\u0e39\u0e48 :";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label41 = this.Label2;
		location = new System.Drawing.Point(38, 25);
		label41.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label42 = this.Label2;
		size = new System.Drawing.Size(57, 14);
		label42.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า :";
		this.Label1.AutoSize = true;
		this.Label1.BackColor = System.Drawing.Color.Transparent;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label43 = this.Label1;
		location = new System.Drawing.Point(14, 6);
		label43.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label44 = this.Label1;
		size = new System.Drawing.Size(115, 18);
		label44.Size = size;
		this.Label1.TabIndex = 21;
		this.Label1.Text = "เพ\u0e34\u0e48มรายการขาย";
		this.RequiredFieldValidator6.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator6.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator7.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator7.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator8.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator8.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.TimeScan.Enabled = true;
		this.TimeScan.Interval = 2000;
		this.Timer2.Interval = 500;
		this.Timer3.Interval = 500;
		this.FlowLayoutPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.FlowLayoutPanel1.AutoScroll = true;
		this.FlowLayoutPanel1.BackColor = System.Drawing.Color.AliceBlue;
		this.FlowLayoutPanel1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.FlowLayoutPanel1.Controls.Add(this.PanelEx_10);
		this.FlowLayoutPanel1.Controls.Add(this.PanelEx_11);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel = this.FlowLayoutPanel1;
		location = new System.Drawing.Point(726, 139);
		flowLayoutPanel.Location = location;
		this.FlowLayoutPanel1.Name = "FlowLayoutPanel1";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel2 = this.FlowLayoutPanel1;
		size = new System.Drawing.Size(467, 179);
		flowLayoutPanel2.Size = size;
		this.FlowLayoutPanel1.TabIndex = 42;
		this.PanelEx_10.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_10.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_10.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_ = this.PanelEx_10;
		location = new System.Drawing.Point(2, 2);
		panelEx_.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx_2 = this.PanelEx_10;
		margin = new System.Windows.Forms.Padding(2);
		panelEx_2.Margin = margin;
		this.PanelEx_10.Name = "PanelEx34";
		DevComponents.DotNetBar.PanelEx panelEx_3 = this.PanelEx_10;
		size = new System.Drawing.Size(94, 58);
		panelEx_3.Size = size;
		this.PanelEx_10.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_10.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_10.Style.BackColor2.Color = System.Drawing.Color.PaleGreen;
		this.PanelEx_10.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_10.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_10.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_10.Style.GradientAngle = 90;
		this.PanelEx_10.TabIndex = 3;
		this.PanelEx_10.Text = "น\u0e49ำแข\u0e47ง";
		this.PanelEx_11.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_11.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_11.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_4 = this.PanelEx_11;
		location = new System.Drawing.Point(100, 2);
		panelEx_4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx_5 = this.PanelEx_11;
		margin = new System.Windows.Forms.Padding(2);
		panelEx_5.Margin = margin;
		this.PanelEx_11.Name = "PanelEx35";
		DevComponents.DotNetBar.PanelEx panelEx_6 = this.PanelEx_11;
		size = new System.Drawing.Size(94, 58);
		panelEx_6.Size = size;
		this.PanelEx_11.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_11.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_11.Style.BackColor2.Color = System.Drawing.Color.PaleGreen;
		this.PanelEx_11.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_11.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_11.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_11.Style.GradientAngle = 90;
		this.PanelEx_11.TabIndex = 4;
		this.PanelEx_11.Text = "น\u0e49ำแข\u0e47ง";
		this.PanelEx_8.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx_8.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_8.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_8.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_7 = this.PanelEx_8;
		location = new System.Drawing.Point(0, 0);
		panelEx_7.Location = location;
		this.PanelEx_8.Name = "PanelEx38";
		DevComponents.DotNetBar.PanelEx panelEx_8 = this.PanelEx_8;
		size = new System.Drawing.Size(40, 43);
		panelEx_8.Size = size;
		this.PanelEx_8.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_8.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_8.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_8.Style.BackgroundImage = (System.Drawing.Image)resources.GetObject("PanelEx38.Style.BackgroundImage");
		this.PanelEx_8.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.Center;
		this.PanelEx_8.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_8.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_8.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_8.Style.GradientAngle = 90;
		this.PanelEx_8.StyleMouseOver.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_8.StyleMouseOver.BackColor1.Color = System.Drawing.Color.Yellow;
		this.PanelEx_8.StyleMouseOver.BackColor2.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_8.TabIndex = 30;
		this.PanelEx_7.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx_7.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_7.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_7.Controls.Add(this.PanelEx_8);
		this.PanelEx_7.Controls.Add(this.PanelEx_9);
		this.PanelEx_7.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_9 = this.PanelEx_7;
		location = new System.Drawing.Point(1175, 139);
		panelEx_9.Location = location;
		this.PanelEx_7.Name = "PanelEx37";
		DevComponents.DotNetBar.PanelEx panelEx_10 = this.PanelEx_7;
		size = new System.Drawing.Size(40, 179);
		panelEx_10.Size = size;
		this.PanelEx_7.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_7.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_7.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_7.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.Center;
		this.PanelEx_7.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_7.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_7.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_7.Style.GradientAngle = 90;
		this.PanelEx_7.StyleMouseOver.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_7.TabIndex = 43;
		this.PanelEx_9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx_9.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_9.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_9.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_11 = this.PanelEx_9;
		location = new System.Drawing.Point(0, 140);
		panelEx_11.Location = location;
		this.PanelEx_9.Name = "PanelEx39";
		DevComponents.DotNetBar.PanelEx panelEx_12 = this.PanelEx_9;
		size = new System.Drawing.Size(40, 39);
		panelEx_12.Size = size;
		this.PanelEx_9.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_9.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_9.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_9.Style.BackgroundImage = (System.Drawing.Image)resources.GetObject("PanelEx39.Style.BackgroundImage");
		this.PanelEx_9.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.Center;
		this.PanelEx_9.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_9.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_9.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_9.Style.GradientAngle = 90;
		this.PanelEx_9.StyleMouseOver.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_9.StyleMouseOver.BackColor1.Color = System.Drawing.Color.Yellow;
		this.PanelEx_9.StyleMouseOver.BackColor2.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_9.TabIndex = 31;
		this.PanelEx_0.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx_0.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_0.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_0.Controls.Add(this.PanelEx_1);
		this.PanelEx_0.Controls.Add(this.PanelEx_2);
		this.PanelEx_0.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_13 = this.PanelEx_0;
		location = new System.Drawing.Point(1175, 324);
		panelEx_13.Location = location;
		this.PanelEx_0.Name = "PanelEx27";
		DevComponents.DotNetBar.PanelEx panelEx_14 = this.PanelEx_0;
		size = new System.Drawing.Size(40, 249);
		panelEx_14.Size = size;
		this.PanelEx_0.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_0.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_0.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_0.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.Center;
		this.PanelEx_0.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_0.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_0.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_0.Style.GradientAngle = 90;
		this.PanelEx_0.StyleMouseOver.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_0.TabIndex = 45;
		this.PanelEx_1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx_1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_1.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_15 = this.PanelEx_1;
		location = new System.Drawing.Point(0, 0);
		panelEx_15.Location = location;
		this.PanelEx_1.Name = "PanelEx21";
		DevComponents.DotNetBar.PanelEx panelEx_16 = this.PanelEx_1;
		size = new System.Drawing.Size(40, 58);
		panelEx_16.Size = size;
		this.PanelEx_1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_1.Style.BackgroundImage = (System.Drawing.Image)resources.GetObject("PanelEx21.Style.BackgroundImage");
		this.PanelEx_1.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.Center;
		this.PanelEx_1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_1.Style.GradientAngle = 90;
		this.PanelEx_1.StyleMouseOver.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_1.StyleMouseOver.BackColor1.Color = System.Drawing.Color.Yellow;
		this.PanelEx_1.StyleMouseOver.BackColor2.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_1.TabIndex = 30;
		this.PanelEx_2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx_2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_2.Font = new System.Drawing.Font("Tahoma", 27.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_17 = this.PanelEx_2;
		location = new System.Drawing.Point(0, 191);
		panelEx_17.Location = location;
		this.PanelEx_2.Name = "PanelEx22";
		DevComponents.DotNetBar.PanelEx panelEx_18 = this.PanelEx_2;
		size = new System.Drawing.Size(40, 58);
		panelEx_18.Size = size;
		this.PanelEx_2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx_2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx_2.Style.BackgroundImage = (System.Drawing.Image)resources.GetObject("PanelEx22.Style.BackgroundImage");
		this.PanelEx_2.Style.BackgroundImagePosition = DevComponents.DotNetBar.eBackgroundImagePosition.Center;
		this.PanelEx_2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_2.Style.GradientAngle = 90;
		this.PanelEx_2.StyleMouseOver.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_2.StyleMouseOver.BackColor1.Color = System.Drawing.Color.Yellow;
		this.PanelEx_2.StyleMouseOver.BackColor2.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_2.TabIndex = 31;
		this.FlowLayoutPanel3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.FlowLayoutPanel3.AutoScroll = true;
		this.FlowLayoutPanel3.BackColor = System.Drawing.Color.AliceBlue;
		this.FlowLayoutPanel3.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.FlowLayoutPanel3.Controls.Add(this.PanelEx_3);
		this.FlowLayoutPanel3.Controls.Add(this.PanelEx_4);
		this.FlowLayoutPanel3.Controls.Add(this.PanelEx_5);
		this.FlowLayoutPanel3.Controls.Add(this.PanelEx_6);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel3 = this.FlowLayoutPanel3;
		location = new System.Drawing.Point(726, 324);
		flowLayoutPanel3.Location = location;
		this.FlowLayoutPanel3.Name = "FlowLayoutPanel3";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel4 = this.FlowLayoutPanel3;
		size = new System.Drawing.Size(465, 249);
		flowLayoutPanel4.Size = size;
		this.FlowLayoutPanel3.TabIndex = 44;
		this.PanelEx_3.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_3.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_3.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_19 = this.PanelEx_3;
		location = new System.Drawing.Point(2, 2);
		panelEx_19.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx_20 = this.PanelEx_3;
		margin = new System.Windows.Forms.Padding(2);
		panelEx_20.Margin = margin;
		this.PanelEx_3.Name = "PanelEx18";
		DevComponents.DotNetBar.PanelEx panelEx_21 = this.PanelEx_3;
		size = new System.Drawing.Size(94, 58);
		panelEx_21.Size = size;
		this.PanelEx_3.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_3.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_3.Style.BackColor2.Color = System.Drawing.Color.PaleGreen;
		this.PanelEx_3.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_3.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_3.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_3.Style.GradientAngle = 90;
		this.PanelEx_3.TabIndex = 2;
		this.PanelEx_3.Text = "น\u0e49ำแข\u0e47งhhjjhjghghk jkjhk\r\n100";
		this.PanelEx_4.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_4.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_4.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_22 = this.PanelEx_4;
		location = new System.Drawing.Point(100, 2);
		panelEx_22.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx_23 = this.PanelEx_4;
		margin = new System.Windows.Forms.Padding(2);
		panelEx_23.Margin = margin;
		this.PanelEx_4.Name = "PanelEx19";
		DevComponents.DotNetBar.PanelEx panelEx_24 = this.PanelEx_4;
		size = new System.Drawing.Size(94, 58);
		panelEx_24.Size = size;
		this.PanelEx_4.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_4.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_4.Style.BackColor2.Color = System.Drawing.Color.PaleGreen;
		this.PanelEx_4.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_4.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_4.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_4.Style.GradientAngle = 90;
		this.PanelEx_4.TabIndex = 3;
		this.PanelEx_4.Text = "น\u0e49ำแข\u0e47ง";
		this.PanelEx_5.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_5.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_5.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_25 = this.PanelEx_5;
		location = new System.Drawing.Point(198, 2);
		panelEx_25.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx_26 = this.PanelEx_5;
		margin = new System.Windows.Forms.Padding(2);
		panelEx_26.Margin = margin;
		this.PanelEx_5.Name = "PanelEx20";
		DevComponents.DotNetBar.PanelEx panelEx_27 = this.PanelEx_5;
		size = new System.Drawing.Size(94, 58);
		panelEx_27.Size = size;
		this.PanelEx_5.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_5.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_5.Style.BackColor2.Color = System.Drawing.Color.PaleGreen;
		this.PanelEx_5.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_5.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_5.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_5.Style.GradientAngle = 90;
		this.PanelEx_5.TabIndex = 4;
		this.PanelEx_5.Text = "น\u0e49ำแข\u0e47ง";
		this.PanelEx_6.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx_6.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx_6.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx_28 = this.PanelEx_6;
		location = new System.Drawing.Point(296, 2);
		panelEx_28.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx_29 = this.PanelEx_6;
		margin = new System.Windows.Forms.Padding(2);
		panelEx_29.Margin = margin;
		this.PanelEx_6.Name = "PanelEx31";
		DevComponents.DotNetBar.PanelEx panelEx_30 = this.PanelEx_6;
		size = new System.Drawing.Size(94, 58);
		panelEx_30.Size = size;
		this.PanelEx_6.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx_6.Style.BackColor1.Color = System.Drawing.Color.FromArgb(255, 255, 192);
		this.PanelEx_6.Style.BackColor2.Color = System.Drawing.Color.PaleGreen;
		this.PanelEx_6.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx_6.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx_6.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx_6.Style.GradientAngle = 90;
		this.PanelEx_6.TabIndex = 5;
		this.PanelEx_6.Text = "น\u0e49ำแข\u0e47ง";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BottomLeftCornerSize = 0;
		this.BottomRightCornerSize = 0;
		size = new System.Drawing.Size(1224, 588);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmAddSale2";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เพ\u0e34\u0e48มรายการใบเสร\u0e47จ";
		this.TopLeftCornerSize = 0;
		this.TopRightCornerSize = 0;
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.GroupPanel1.ResumeLayout(false);
		this.GroupPanel1.PerformLayout();
		this.GroupBox2.ResumeLayout(false);
		this.GroupBox2.PerformLayout();
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.FlowLayoutPanel1.ResumeLayout(false);
		this.PanelEx_7.ResumeLayout(false);
		this.PanelEx_0.ResumeLayout(false);
		this.FlowLayoutPanel3.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormSearchPro.ShowDialog();
		if (!Operators.ConditionalCompareObjectNotEqual(MyProject.Forms.FormSearchPro.SelectNO, "", TextCompare: false))
		{
			return;
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products where Pro_no='", MyProject.Forms.FormSearchPro.SelectNO), "'")));
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบรห\u0e31สส\u0e34นค\u0e49า ", MyProject.Forms.FormSearchPro.SelectNO)));
		}
		else
		{
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products_Price where P_ID='", dataSet.Tables[0].Rows[0]["Pro_no"]), "' and P_CustType='"), TCusType.Text), "'")));
			decimal num = default(decimal);
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				num = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["P_Price"]);
			}
			Tno.Text = "";
			Tno1.Text = dataSet.Tables[0].Rows[0]["Pro_no"].ToString();
			Tname.Text = dataSet.Tables[0].Rows[0]["Pro_Name"].ToString();
			Tunit.Text = dataSet.Tables[0].Rows[0]["Pro_Unit"].ToString();
			IdSelectNow = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["id"]);
			Tnum.Text = Conversions.ToString(1);
			Tprice.Text = Conversions.ToString(num);
			Ttotal.Text = Conversions.ToString(num);
			Tnum.Focus();
			BsaveEdit.Text = "เพ\u0e34\u0e48ม";
			BsaveEdit.Enabled = true;
		}
		Tno.Text = "";
	}

	private void TabEnter(object sender, KeyPressEventArgs e)
	{
		Module1.KeyPattern(e, (Control)sender);
	}

	private void BsaveEdit_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Tprice.Text, "", TextCompare: false) == 0)
		{
			Tprice.Text = Conversions.ToString(0);
		}
		if (Operators.CompareString(Tnum.Text, "", TextCompare: false) == 0)
		{
			Tnum.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(Tprice.Text))
		{
			MessageBox.Show("กร\u0e38ณากรอกราคาเป\u0e47นต\u0e31วเลข");
			return;
		}
		if (!Versioned.IsNumeric(Tnum.Text))
		{
			MessageBox.Show("กร\u0e38ณากรอกจำนวนเป\u0e47นต\u0e31วเลข");
			return;
		}
		if (Operators.CompareString(BsaveEdit.Text, "เพ\u0e34\u0e48ม", TextCompare: false) == 0)
		{
			Adlist(Conversions.ToString(NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null)));
		}
		else
		{
			NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 5 }, null, null, null), null, "Text", new object[1] { Strings.Format(Conversions.ToDecimal(Tnum.Text), "#,##0") }, null, null, OptimisticSet: false, RValueBase: true);
			NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 4 }, null, null, null), null, "Text", new object[1] { Tunit.Text }, null, null, OptimisticSet: false, RValueBase: true);
			NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 6 }, null, null, null), null, "Text", new object[1] { Strings.Format(Conversions.ToDecimal(Tprice.Text), "#,##0.00") }, null, null, OptimisticSet: false, RValueBase: true);
			if (Operators.CompareString(Tdis.Text.Substring(checked(Tdis.Text.Length - 1), 1), "%", TextCompare: false) == 0)
			{
				NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 7 }, null, null, null), null, "Text", new object[1] { Tdis.Text }, null, null, OptimisticSet: false, RValueBase: true);
				NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 8 }, null, null, null), null, "Text", new object[1] { Strings.Format(decimal.Divide(decimal.Multiply(decimal.Multiply(Conversions.ToDecimal(Tprice.Text), Conversions.ToDecimal(Tnum.Text)), Conversions.ToDecimal(Tdis.Text.Replace("%", ""))), 100m), "#,##0.00") }, null, null, OptimisticSet: false, RValueBase: true);
			}
			else
			{
				NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 7 }, null, null, null), null, "Text", new object[1] { "" }, null, null, OptimisticSet: false, RValueBase: true);
				NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 8 }, null, null, null), null, "Text", new object[1] { Strings.Format(Conversions.ToDecimal(Tdis.Text), "#,##0.00") }, null, null, OptimisticSet: false, RValueBase: true);
			}
			NewLateBinding.LateSetComplex(NewLateBinding.LateGet(NewLateBinding.LateGet(ListView1.Items, null, "Item", new object[1] { RuntimeHelpers.GetObjectValue(ItemEdit) }, null, null, null), null, "SubItems", new object[1] { 9 }, null, null, null), null, "Text", new object[1] { Strings.Format(Conversions.ToDecimal(Ttotal.Text), "#,##0.00") }, null, null, OptimisticSet: false, RValueBase: true);
			BsaveEdit.Text = "เพ\u0e34\u0e48ม";
		}
		Tno1.Text = "";
		Tname.Text = "";
		Tunit.Text = "";
		Tnum.Text = "";
		Tprice.Text = "";
		Tdis.Text = "";
		Ttotal.Text = "";
		BsaveEdit.Enabled = true;
		Bdel.Enabled = true;
		ListView1.Enabled = true;
		BsaveEdit.Enabled = false;
		Tno.Focus();
		sumALL();
	}

	public void Adlist(string sender)
	{
		if (Operators.CompareString(Tno1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาค\u0e49นหารายการส\u0e34นค\u0e49า");
			return;
		}
		if (Operators.CompareString(Tdis.Text, "", TextCompare: false) == 0)
		{
			Tdis.Text = Conversions.ToString(0);
		}
		checked
		{
			if (Operators.CompareString(sender, "เพ\u0e34\u0e48ม", TextCompare: false) == 0)
			{
				if (SearchIteminlist(Tno1.Text, Conversions.ToString(0)))
				{
					MessageBox.Show("ได\u0e49พบรายการส\u0e34นค\u0e49า : " + Tno1.Text + " ในรายการขายแล\u0e49ว");
					ClearAdd();
					return;
				}
				int count = ListView1.Items.Count;
				ListView.ListViewItemCollection items = ListView1.Items;
				object[] array = new object[1] { RuntimeHelpers.GetObjectValue(IdSelectNow) };
				bool[] array2 = new bool[1] { true };
				NewLateBinding.LateCall(items, null, "Add", array, null, null, array2, IgnoreReturn: true);
				if (array2[0])
				{
					IdSelectNow = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListView1.Items[count].SubItems.Add(Conversions.ToString(count + 1));
				ListView1.Items[count].SubItems.Add(Tno1.Text);
				ListView1.Items[count].SubItems.Add(Tname.Text);
				ListView1.Items[count].SubItems.Add(Tunit.Text);
				ListView1.Items[count].SubItems.Add(Strings.Format(Conversions.ToDecimal(Tnum.Text), "#,##0.00"));
				ListView1.Items[count].SubItems.Add(Strings.Format(Conversions.ToDecimal(Tprice.Text), "#,##0.00"));
				if (Operators.CompareString(Tdis.Text.Substring(Tdis.Text.Length - 1, 1), "%", TextCompare: false) == 0)
				{
					ListView1.Items[count].SubItems.Add(Tdis.Text);
					ListView1.Items[count].SubItems.Add(Strings.Format(decimal.Divide(decimal.Multiply(decimal.Multiply(Conversions.ToDecimal(Tprice.Text), Conversions.ToDecimal(Tnum.Text)), Conversions.ToDecimal(Tdis.Text.Replace("%", ""))), 100m), "#,##0.00"));
				}
				else
				{
					ListView1.Items[count].SubItems.Add("");
					ListView1.Items[count].SubItems.Add(Strings.Format(Conversions.ToDecimal(Tdis.Text), "#,##0.00"));
				}
				ListView1.Items[count].SubItems.Add(Strings.Format(Conversions.ToDecimal(Ttotal.Text), "#,##0.00"));
			}
			ClearAdd();
		}
	}

	public bool SearchIteminlist(string Ino, string IPO)
	{
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if ((Operators.CompareString(Ino, ListView1.Items[num2].SubItems[2].Text, TextCompare: false) == 0) & (Operators.CompareString(IPO, ListView1.Items[num2].SubItems[4].Text, TextCompare: false) == 0))
					{
						break;
					}
					num2++;
					continue;
				}
				return false;
			}
			return true;
		}
	}

	public void ClearAdd()
	{
		IdSelectNow = 0;
		Tno1.Text = "";
		Tname.Text = "";
		Tnum.Text = "";
		Tunit.Text = "";
		Tprice.Text = "";
		Ttotal.Text = "";
		Tdis.Text = Conversions.ToString(0);
		BsaveEdit.Enabled = false;
		Tno.Focus();
		sumALL();
	}

	public void sumALL()
	{
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			decimal num5 = default(decimal);
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				num5 = decimal.Add(num5, Conversions.ToDecimal(ListView1.Items[num2].SubItems[9].Text));
				num2++;
			}
			if (!Versioned.IsNumeric(TextBox_1.Text))
			{
				TextBox_1.Text = Conversions.ToString(0);
			}
			if (Operators.CompareString(TextBox_3.Text, "", TextCompare: false) == 0)
			{
				TextBox_3.Text = Conversions.ToString(0);
			}
			if (!Versioned.IsNumeric(TextBox_3.Text))
			{
				TextBox_3.Text = Conversions.ToString(0);
			}
			if (Operators.CompareString(Tvat.Text, "", TextCompare: false) == 0)
			{
				Tvat.Text = Conversions.ToString(0);
			}
			if (!Versioned.IsNumeric(Tvat.Text))
			{
				Tvat.Text = Conversions.ToString(0);
			}
			if (CheckVat.Checked)
			{
				TextBox_2.Text = Strings.Format(num5, "#,##0.00");
				TextBox_1.Text = Strings.Format(decimal.Divide(decimal.Multiply(decimal.Subtract(num5, Conversions.ToDecimal(TextBox_3.Text)), Conversions.ToDecimal(Tvat.Text)), 107m), "#,##0.00");
				TextBox_0.Text = Strings.Format(decimal.Subtract(Conversions.ToDecimal(TextBox_2.Text), Conversions.ToDecimal(TextBox_3.Text)), "#,##0.00");
			}
			else
			{
				TextBox_2.Text = Strings.Format(num5, "#,##0.00");
				TextBox_1.Text = Strings.Format(decimal.Divide(decimal.Multiply(decimal.Subtract(num5, Conversions.ToDecimal(TextBox_3.Text)), Conversions.ToDecimal(Tvat.Text)), 100m), "#,##0.00");
				TextBox_0.Text = Strings.Format(decimal.Subtract(decimal.Add(Conversions.ToDecimal(TextBox_2.Text), Conversions.ToDecimal(TextBox_1.Text)), Conversions.ToDecimal(TextBox_3.Text)), "#,##0.00");
			}
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FormSearchCust.ShowDialog();
		if (Operators.ConditionalCompareObjectNotEqual(MyProject.Forms.FormSearchCust.SelectNO, "", TextCompare: false))
		{
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from view_Customers where Cust_no='", MyProject.Forms.FormSearchCust.SelectNO), "'")));
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบรห\u0e31สล\u0e39กค\u0e49า ", MyProject.Forms.FormSearchCust.SelectNO)));
				AName.Text = "";
				Aaddress.Text = "";
				AFax.Text = "";
				Atel.Text = "";
				CUST_ID = "";
			}
			else
			{
				AName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
				Atel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
				AFax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
				Aaddress.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["C_Address"]);
				TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
				CUST_ID = (string)RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cust_no"]);
				Tno.Focus();
			}
		}
	}

	private void FrmAddSale_FormClosing(object sender, FormClosingEventArgs e)
	{
		clear();
	}

	public void LoadType()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by id");
		TCusType.DataSource = dataSet.Tables[0];
		TCusType.DisplayMember = "name";
		TCusType.ValueMember = "id";
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			TCusType.SelectedIndex = 0;
		}
	}

	private void FrmAddSale_Load(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from view_Customers where Cust_no='C0000'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			AName.Text = "";
			Aaddress.Text = "";
			AFax.Text = "";
			Atel.Text = "";
			CUST_ID = "";
			return;
		}
		AName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_name"]);
		Atel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_tel"]);
		AFax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Add_fax"]);
		Aaddress.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["C_Address"]);
		TCusType.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cust_Type"]);
		CUST_ID = (string)RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cust_no"]);
		Tno.Focus();
		loadButton();
		loadButtonG();
		Tref.Text = "หน\u0e49าฟร\u0e49อน";
		LoadType();
		Ano.Text = Module1.GetSIR_PAY();
		Adate.Value = DateTime.Now;
		CheckEdit();
		sumALL();
	}

	private void GROUPMouseDown(object sender, MouseEventArgs e)
	{
		checked
		{
			int num = FlowLayoutPanel1.Controls.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				PanelEx panelEx = (PanelEx)FlowLayoutPanel1.Controls[num2];
				panelEx.Style.BackColor1.Color = Color.WhiteSmoke;
				panelEx.Style.BackColor2.Color = Color.WhiteSmoke;
				if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null), panelEx.Name, TextCompare: false))
				{
					panelEx.Style.BackColor1.Color = Color.Yellow;
					panelEx.Style.BackColor2.Color = Color.Yellow;
					Gnow = panelEx.Name;
				}
				num2++;
			}
			loadButton();
		}
	}

	public void ButtonClick(object sender, MouseEventArgs e)
	{
		Tno.Focus();
		MyProject.Forms.FormSearchPro.SelectNO = (string)RuntimeHelpers.GetObjectValue(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null));
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products where Pro_no='", MyProject.Forms.FormSearchPro.SelectNO), "'")));
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject("ไม\u0e48พบรห\u0e31สส\u0e34นค\u0e49า ", MyProject.Forms.FormSearchPro.SelectNO)));
		}
		else
		{
			DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Products_Price where P_ID='", dataSet.Tables[0].Rows[0]["Pro_no"]), "' and P_CustType='"), TCusType.Text), "'")));
			decimal num = default(decimal);
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				num = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["P_Price"]);
			}
			Tno.Text = "";
			Tno1.Text = dataSet.Tables[0].Rows[0]["Pro_no"].ToString();
			Tname.Text = dataSet.Tables[0].Rows[0]["Pro_Name"].ToString();
			Tunit.Text = dataSet.Tables[0].Rows[0]["Pro_Unit"].ToString();
			IdSelectNow = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["id"]);
			Tnum.Text = Conversions.ToString(1);
			Tprice.Text = Conversions.ToString(num);
			Ttotal.Text = Conversions.ToString(num);
			Tnum.Focus();
			BsaveEdit.Text = "เพ\u0e34\u0e48ม";
			BsaveEdit.Enabled = true;
		}
		Tno.Text = "";
		BsaveEdit_Click(BsaveEdit, null);
	}

	public void loadButton()
	{
		FlowLayoutPanel3.Controls.Clear();
		if (Operators.CompareString(Gnow, "Favorites", TextCompare: false) == 0)
		{
			return;
		}
		DataSet dataSet = Module1.connect("select * from HT_Products where pro_type='" + Gnow + "' order by pro_name");
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
					PanelEx panelEx = new PanelEx();
					panelEx.CanvasColor = SystemColors.Control;
					panelEx.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
					panelEx.Font = new Font("Tahoma", 9.25f, FontStyle.Bold, GraphicsUnit.Point, 222);
					Point location = new Point(2, 2);
					panelEx.Location = location;
					System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(2);
					panelEx.Margin = margin;
					panelEx.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["pro_no"]);
					Size size = new Size(94, 58);
					panelEx.Size = size;
					panelEx.Style.Alignment = StringAlignment.Center;
					panelEx.Style.BackColor1.Color = Color.White;
					panelEx.Style.BackColor2.Color = Module1.RandomColor(dataSet.Tables[0].Rows[num2]["pro_name"].ToString().Length);
					panelEx.Style.Border = eBorderType.SingleLine;
					panelEx.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
					panelEx.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
					panelEx.Style.GradientAngle = 90;
					panelEx.Style.WordWrap = true;
					panelEx.StyleMouseOver.BackColor1.Color = Color.Yellow;
					panelEx.StyleMouseOver.BackColor2.Color = Color.LightYellow;
					panelEx.StyleMouseOver.Alignment = StringAlignment.Center;
					panelEx.TabIndex = 2;
					panelEx.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["pro_name"]);
					panelEx.MouseDown += ButtonClick;
					FlowLayoutPanel3.Controls.Add(panelEx);
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void loadButtonG()
	{
		FlowLayoutPanel1.Controls.Clear();
		DataSet dataSet = Module1.connect("select pro_type from HT_Products group by pro_type  order by pro_type");
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
					PanelEx panelEx = new PanelEx();
					panelEx.CanvasColor = SystemColors.Control;
					panelEx.ColorSchemeStyle = eDotNetBarStyle.StyleManagerControlled;
					panelEx.Font = new Font("Tahoma", 10.25f, FontStyle.Bold, GraphicsUnit.Point, 222);
					Point location = new Point(2, 2);
					panelEx.Location = location;
					System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(2);
					panelEx.Margin = margin;
					panelEx.Name = Conversions.ToString(dataSet.Tables[0].Rows[num2]["pro_type"]);
					Size size = new Size(110, 50);
					panelEx.Size = size;
					panelEx.Style.Alignment = StringAlignment.Center;
					panelEx.Style.BackColor1.Color = Color.WhiteSmoke;
					panelEx.Style.BackColor2.Color = Color.WhiteSmoke;
					panelEx.Style.Border = eBorderType.SingleLine;
					panelEx.Style.BorderColor.ColorSchemePart = eColorSchemePart.PanelBorder;
					panelEx.Style.ForeColor.ColorSchemePart = eColorSchemePart.PanelText;
					panelEx.Style.GradientAngle = 90;
					panelEx.Style.WordWrap = true;
					panelEx.StyleMouseOver.BackColor1.Color = Color.Yellow;
					panelEx.StyleMouseOver.BackColor2.Color = Color.LightYellow;
					panelEx.StyleMouseOver.Alignment = StringAlignment.Center;
					panelEx.TabIndex = 2;
					panelEx.Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["pro_type"]);
					panelEx.MouseDown += GROUPMouseDown;
					FlowLayoutPanel1.Controls.Add(panelEx);
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void CheckEdit()
	{
		if (!Operators.ConditionalCompareObjectNotEqual(IEdit, 0, TextCompare: false))
		{
			return;
		}
		Text = "แก\u0e49ไขใบเสร\u0e47จร\u0e31บเง\u0e34น";
		Label1.Text = "แก\u0e49ไขใบเสร\u0e47จร\u0e31บเง\u0e34น";
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from HT_Receipt_H where id=", IEdit)));
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Receipt_Ds where S_sale_id=", IEdit), " order by id")));
		AName.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Name"]);
		Aaddress.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Address"]);
		Atel.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Tel"]);
		AFax.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Fax"]);
		Ano.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_no"]);
		Adate.Value = Conversions.ToDate(dataSet.Tables[0].Rows[0]["Receipt_Date"]);
		CheckVat.Checked = Conversions.ToBoolean(dataSet.Tables[0].Rows[0]["Receipt_VatIn"]);
		TextBox_3.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_Discount"]);
		Tref.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_ref"]);
		Tvat.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Receipt_VatPer"]);
		ListView1.Items.Clear();
		checked
		{
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
				try
				{
					ListView listView = ListView1;
					int count = listView.Items.Count;
					ListView.ListViewItemCollection items = listView.Items;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet2.Tables[0].Rows[num2];
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
					listView.Items[count].SubItems.Add(Conversions.ToString(count + 1));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow3 = dataRow;
					columnName = "s_product_no";
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
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "s_product_name";
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
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow5 = dataRow;
					columnName = "s_unitName";
					array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
					array = array3;
					object[] arguments4 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array8 = array3;
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow6 = dataRow;
					columnName = "s_unit";
					array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
					array = array3;
					object[] arguments5 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems4, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array9 = array3;
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow7 = dataRow;
					columnName = "s_price";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array = array3;
					object[] arguments6 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems5, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems6 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array10 = array3;
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow8 = dataRow;
					columnName = "S_PriceDiscount_per";
					array10[0] = RuntimeHelpers.GetObjectValue(dataRow8[columnName]);
					array = array3;
					object[] arguments7 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems6, null, "Add", arguments7, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems7 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array11 = array3;
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow9 = dataRow;
					columnName = "S_PriceDiscount";
					array11[0] = RuntimeHelpers.GetObjectValue(dataRow9[columnName]);
					array = array3;
					object[] arguments8 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems7, null, "Add", arguments8, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems8 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array12 = array3;
					dataRow = dataSet2.Tables[0].Rows[num2];
					DataRow dataRow10 = dataRow;
					columnName = "s_total";
					array12[0] = RuntimeHelpers.GetObjectValue(dataRow10[columnName]);
					array = array3;
					object[] arguments9 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems8, null, "Add", arguments9, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listView = null;
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
				num2++;
			}
			sumALL();
			ButtonX2.Text = "แก\u0e49ไข";
		}
	}

	public void clear()
	{
		Text = "เพ\u0e34\u0e48มรายการใบเสร\u0e47จร\u0e31บเง\u0e34น";
		Label1.Text = "เพ\u0e34\u0e48มรายการใบเสร\u0e47จร\u0e31บเง\u0e34น";
		ButtonX2.Text = "บ\u0e31นท\u0e36ก";
		CheckVat.Checked = true;
		Tvat.Text = Conversions.ToString(0);
		ClearAdd();
		CUST_ID = "";
		Ano.Text = Module1.GetSIR_PAY();
		AName.Text = "";
		Adate.Value = DateTime.Now;
		Tref.Text = "";
		ListView1.Items.Clear();
		Aaddress.Text = "";
		Atel.Text = "";
		AFax.Text = "";
		Timer1.Enabled = true;
		sumALL();
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		Tno.Focus();
	}

	public string GetSIR()
	{
		int num = 1;
		DataSet dataSet = Module1.connect("select top 1 Receipt_no from HT_Receipt_H where Receipt_Date between '" + Conversions.ToString(Adate.Value.Month) + "/01/" + Conversions.ToString(Adate.Value.Year) + " 00:00:00' and '" + Conversions.ToString(Adate.Value.Month) + "/" + Conversions.ToString(DateTime.DaysInMonth(Adate.Value.Year, Adate.Value.Month)) + "/" + Conversions.ToString(Adate.Value.Year) + " 23:59:59' order by Receipt_no Desc");
		checked
		{
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				num = 1;
			}
			else
			{
				num = Conversions.ToInteger(dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().Substring(dataSet.Tables[0].Rows[0]["Receipt_no"].ToString().LastIndexOf("-") + 1));
				num++;
			}
			return Strings.Format(DateTime.Now, "ByyMM") + "-" + Strings.Format(num, "0000");
		}
	}

	private void Bdel_Click(object sender, EventArgs e)
	{
		del();
	}

	public void del()
	{
		checked
		{
			if (ListView1.SelectedItems.Count == 0)
			{
				MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการท\u0e35\u0e48จะลบ");
				Timer1.Enabled = true;
			}
			else
			{
				if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบรายการท\u0e35\u0e48เล\u0e37อกหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.Yes)
				{
					return;
				}
				ListView1.Items.Remove(ListView1.SelectedItems[0]);
				int num = ListView1.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					ListView1.Items[num2].SubItems[1].Text = Conversions.ToString(num2 + 1);
					num2++;
				}
				sumALL();
				Timer1.Enabled = true;
			}
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		clear();
	}

	private void CheckVat_CheckedChanged(object sender, EventArgs e)
	{
		sumALL();
	}

	private void Tvat_TextChanged(object sender, EventArgs e)
	{
		sumALL();
	}

	private void TextBox_3_TextChanged(object sender, EventArgs e)
	{
		sumALL();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Save(pp: false);
		Timer1.Enabled = true;
	}

	public void Save(bool pp)
	{
		if (ListView1.Items.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเพ\u0e34\u0e48มรายการขาย");
			return;
		}
		if (Operators.CompareString(Ano.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขท\u0e35\u0e48ใบขายส\u0e34นค\u0e49า");
			return;
		}
		if (Operators.ConditionalCompareObjectEqual(CUST_ID, "", TextCompare: false))
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกล\u0e39กค\u0e49า");
			return;
		}
		if (Operators.ConditionalCompareObjectEqual(IEdit, 0, TextCompare: false))
		{
			SaveAdd(pp);
			return;
		}
		SaveEdit(pp);
		Close();
	}

	public void SaveAdd(bool pp)
	{
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กรายการขายหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36กรายการขาย", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.No)
		{
			return;
		}
		string sIR_PAY = Module1.GetSIR_PAY();
		DateTime now = DateTime.Now;
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				Module1.Insert_Pay(Tref.Text, ListView1.Items[num2].SubItems[3].Text, now, Conversions.ToDecimal(TextBox_2.Text), 0m, ListView1.Items[num2].SubItems[3].Text, Conversions.ToDecimal(ListView1.Items[num2].SubItems[9].Text), ListView1.Items[num2].SubItems[4].Text, sIR_PAY, Conversions.ToString(CUST_ID), ListView1.Items[num2].SubItems[2].Text, Conversions.ToDecimal(ListView1.Items[num2].SubItems[5].Text), Conversions.ToDecimal(ListView1.Items[num2].SubItems[9].Text), Conversions.ToDecimal(ListView1.Items[num2].SubItems[6].Text), "รายการไม\u0e48อ\u0e49างอ\u0e34งใบลงทะเบ\u0e35ยน", 0m, 0m, 0m);
				Module1.connect("update HT_Products set Pro_Amt=Pro_Amt-" + Conversions.ToString(Conversions.ToDecimal(ListView1.Items[num2].SubItems[5].Text)) + " where Pro_no='" + ListView1.Items[num2].SubItems[2].Text + "'");
				num2++;
			}
			if (pp)
			{
				Print_Report.Print_Sale(sIR_PAY, preview: false);
			}
			clear();
			BsaveEdit.Text = "เพ\u0e34\u0e48ม";
		}
	}

	public void SaveEdit(bool pp)
	{
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการแก\u0e49ไขรายการขายหร\u0e37อไม\u0e48", "แก\u0e49ไขรายการขาย", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.Yes)
		{
			return;
		}
		object left = "";
		left = Operators.ConcatenateObject(left, "UPDATE [HT_Receipt_H] SET ");
		left = Operators.ConcatenateObject(left, string.Concat(" [Receipt_Date]='" + Conversions.ToString(Adate.Value), "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Receipt_Name]='" + AName.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Receipt_Address]='" + Aaddress.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Receipt_Tel]='" + Atel.Text, "'"));
		left = Operators.ConcatenateObject(left, string.Concat(",[Receipt_Fax]='" + AFax.Text, "'"));
		left = Operators.ConcatenateObject(left, ",[Receipt_Total]=" + Conversions.ToString(Conversions.ToDecimal(TextBox_0.Text)));
		left = Operators.ConcatenateObject(left, ",[Receipt_Vat]=" + Conversions.ToString(Conversions.ToDecimal(TextBox_1.Text)));
		left = Operators.ConcatenateObject(left, ",[Receipt_BeforeVat]=" + Conversions.ToString(decimal.Subtract(Conversions.ToDecimal(TextBox_0.Text), Conversions.ToDecimal(TextBox_1.Text))));
		left = Operators.ConcatenateObject(left, string.Concat(",[Receipt_VatIn]='" + Conversions.ToString(CheckVat.Checked), "'"));
		left = Operators.ConcatenateObject(left, ",[Receipt_VatPer]=" + Conversions.ToString(Conversions.ToDecimal(Tvat.Text)));
		left = Operators.ConcatenateObject(left, ",[Receipt_Discount]=" + Conversions.ToString(Conversions.ToDecimal(TextBox_3.Text)));
		left = Operators.ConcatenateObject(left, string.Concat(",[Receipt_ref]='" + Tref.Text, "'"));
		left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(" where id=", IEdit));
		Module1.connect(Conversions.ToString(left));
		Module1.connect(Conversions.ToString(Operators.ConcatenateObject("delete from HT_Receipt_Ds where S_Sale_id=", IEdit)));
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				left = "";
				left = Operators.ConcatenateObject(left, "INSERT INTO [HT_Receipt_Ds]");
				left = Operators.ConcatenateObject(left, "([S_Sale_id]");
				left = Operators.ConcatenateObject(left, ",[S_Product_no]");
				left = Operators.ConcatenateObject(left, ",[S_Product_name]");
				left = Operators.ConcatenateObject(left, ",[S_Unit]");
				left = Operators.ConcatenateObject(left, ",[S_UnitName]");
				left = Operators.ConcatenateObject(left, ",[S_Price]");
				left = Operators.ConcatenateObject(left, ",[S_Total],S_PriceDiscount_per,S_PriceDiscount)");
				left = Operators.ConcatenateObject(left, "VALUES");
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, IEdit);
				left = Operators.ConcatenateObject(left, string.Concat(",'" + ListView1.Items[num2].SubItems[2].Text, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + ListView1.Items[num2].SubItems[3].Text, "'"));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(ListView1.Items[num2].SubItems[5].Text)));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + ListView1.Items[num2].SubItems[4].Text, "'"));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(ListView1.Items[num2].SubItems[6].Text)));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(ListView1.Items[num2].SubItems[9].Text)));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + ListView1.Items[num2].SubItems[7].Text, "'"));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(ListView1.Items[num2].SubItems[8].Text)));
				left = Operators.ConcatenateObject(left, ")");
				Module1.connect(Conversions.ToString(left));
				num2++;
			}
			MessageBox.Show("แก\u0e49ไขรายการขายเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			if (pp)
			{
				Print_Report.Print_SaleVat(Conversions.ToInteger(IEdit), preview: false);
			}
			BsaveEdit.Text = "เพ\u0e34\u0e48ม";
			Close();
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		Save(pp: true);
		Timer1.Enabled = true;
	}

	private void Tnum_TextChanged(object sender, EventArgs e)
	{
		if (Operators.CompareString(Tnum.Text, "", TextCompare: false) == 0)
		{
			Tnum.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(Tnum.Text))
		{
			Tnum.Text = Conversions.ToString(0);
		}
		if (Operators.CompareString(Tprice.Text, "", TextCompare: false) == 0)
		{
			Tprice.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(Tprice.Text))
		{
			Tprice.Text = Conversions.ToString(0);
		}
		if (Operators.CompareString(Tdis.Text, "", TextCompare: false) == 0)
		{
			Tdis.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(Tdis.Text.Replace("%", "")))
		{
			Tdis.Text = Conversions.ToString(0);
		}
		if (!Versioned.IsNumeric(Tdis.Text.Replace("%", "")))
		{
			Tdis.Text = Conversions.ToString(0);
		}
		if (Operators.CompareString(Tdis.Text.Substring(checked(Tdis.Text.Length - 1), 1), "%", TextCompare: false) == 0)
		{
			Ttotal.Text = Conversions.ToString(decimal.Subtract(decimal.Multiply(Conversions.ToDecimal(Tnum.Text), Conversions.ToDecimal(Tprice.Text)), decimal.Divide(decimal.Multiply(decimal.Multiply(Conversions.ToDecimal(Tnum.Text), Conversions.ToDecimal(Tprice.Text)), Conversions.ToDecimal(Tdis.Text.Replace("%", ""))), 100m)));
		}
		else
		{
			Ttotal.Text = Conversions.ToString(decimal.Subtract(decimal.Multiply(Conversions.ToDecimal(Tnum.Text), Conversions.ToDecimal(Tprice.Text)), Conversions.ToDecimal(Tdis.Text)));
		}
		Ttotal.Text = Strings.Format(Conversions.ToDecimal(Ttotal.Text), "#,##0.00");
	}

	private void FlowLayoutPanel3_Paint(object sender, PaintEventArgs e)
	{
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		Eitems();
	}

	private void ListView1_KeyDown(object sender, KeyEventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			Bdel_Click(Bdel, null);
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	public void Eitems()
	{
		if (ListView1.SelectedItems.Count != 1)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการ 1 รายการ");
			return;
		}
		ListView1.Enabled = false;
		Tunit.Text = "";
		Tno1.Text = ListView1.SelectedItems[0].SubItems[2].Text;
		Tname.Text = ListView1.SelectedItems[0].SubItems[3].Text;
		Tunit.Text = ListView1.SelectedItems[0].SubItems[4].Text;
		Tnum.Text = Conversions.ToString(Conversions.ToDecimal(ListView1.SelectedItems[0].SubItems[5].Text));
		Tprice.Text = Conversions.ToString(Conversions.ToDecimal(ListView1.SelectedItems[0].SubItems[6].Text));
		if (Operators.CompareString(ListView1.SelectedItems[0].SubItems[7].Text, "", TextCompare: false) != 0)
		{
			Tdis.Text = ListView1.SelectedItems[0].SubItems[7].Text;
		}
		else
		{
			Tdis.Text = Conversions.ToString(Conversions.ToDecimal(ListView1.SelectedItems[0].SubItems[8].Text));
		}
		Ttotal.Text = Conversions.ToString(Conversions.ToDecimal(ListView1.SelectedItems[0].SubItems[9].Text));
		ItemEdit = ListView1.SelectedItems[0].Index;
		BsaveEdit.Enabled = false;
		Bdel.Enabled = false;
		BsaveEdit.Enabled = true;
		BsaveEdit.Text = "แก\u0e49ไข";
		Tnum.Focus();
	}
}
