using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Net;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class frmReg : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	[AccessedThroughProperty("Lreg")]
	private Label _Lreg;

	[AccessedThroughProperty("Ldetails")]
	private Label _Ldetails;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("TextBox2")]
	private MaskedTextBox _TextBox2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Ldate")]
	private Label _Ldate;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ButtonX11")]
	private ButtonX _ButtonX11;

	[AccessedThroughProperty("WebBLOCK")]
	private WebBrowser _WebBLOCK;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("Panel1")]
	private Panel _Panel1;

	[AccessedThroughProperty("LabelRef")]
	private Label _LabelRef;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	private bool ISOK;

	private string comds;

	private string UpdateUr2;

	private string FtpUsername;

	private string FtpPassowrd;

	private int num;

	private long Maxnum;

	private int NumF;

	private string fnow;

	private string fromserver;

	private string ACT_NOW;

	private int NFilesUpdate;

	private string ss;

	private int clickasss;

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
			EventHandler value2 = Label5_Click;
			if (_Label5 != null)
			{
				_Label5.Click -= value2;
			}
			_Label5 = value;
			if (_Label5 != null)
			{
				_Label5.Click += value2;
			}
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

	internal virtual Label Lreg
	{
		[DebuggerNonUserCode]
		get
		{
			return _Lreg;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Lreg = value;
		}
	}

	internal virtual Label Ldetails
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ldetails;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ldetails = value;
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
			EventHandler value2 = ButtonX3_Click_1;
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

	internal virtual MaskedTextBox TextBox2
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
			EventHandler value2 = ButtonX1_Click_1;
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

	internal virtual Label Ldate
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ldate;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ldate = value;
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

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX11_Click;
			if (_ButtonX11 != null)
			{
				_ButtonX11.Click -= value2;
			}
			_ButtonX11 = value;
			if (_ButtonX11 != null)
			{
				_ButtonX11.Click += value2;
			}
		}
	}

	internal virtual WebBrowser WebBLOCK
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBLOCK;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBLOCK_DocumentCompleted;
			if (_WebBLOCK != null)
			{
				_WebBLOCK.DocumentCompleted -= value2;
			}
			_WebBLOCK = value;
			if (_WebBLOCK != null)
			{
				_WebBLOCK.DocumentCompleted += value2;
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
			EventHandler value2 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
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

	internal virtual Panel Panel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			PaintEventHandler value2 = Panel1_Paint;
			MouseEventHandler value3 = Panel1_MouseClick;
			if (_Panel1 != null)
			{
				_Panel1.Paint -= value2;
				_Panel1.MouseClick -= value3;
			}
			_Panel1 = value;
			if (_Panel1 != null)
			{
				_Panel1.Paint += value2;
				_Panel1.MouseClick += value3;
			}
		}
	}

	internal virtual Label LabelRef
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelRef;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelRef = value;
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
			EventHandler value2 = Timer2_Tick;
			if (_Timer2 != null)
			{
				_Timer2.Tick -= value2;
			}
			_Timer2 = value;
			if (_Timer2 != null)
			{
				_Timer2.Tick += value2;
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

	[DebuggerNonUserCode]
	static frmReg()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public frmReg()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += frmReg_FormClosing;
		base.Load += frmReg_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISOK = false;
		comds = "";
		UpdateUr2 = "";
		FtpUsername = "attosoft";
		FtpPassowrd = "1q2w3e4r";
		num = 0;
		Maxnum = 0L;
		NumF = 0;
		fnow = "";
		fromserver = "";
		ACT_NOW = "";
		NFilesUpdate = 0;
		ss = "";
		clickasss = 0;
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
		System.Windows.Forms.ListViewItem listViewItem = new System.Windows.Forms.ListViewItem("ABDHSADBHDGEWY");
		System.Windows.Forms.ListViewItem listViewItem2 = new System.Windows.Forms.ListViewItem("AKLJDSSIDJW");
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.frmReg));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.WebBLOCK = new System.Windows.Forms.WebBrowser();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.Ldate = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.TextBox2 = new System.Windows.Forms.MaskedTextBox();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.LabelRef = new System.Windows.Forms.Label();
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.Lreg = new System.Windows.Forms.Label();
		this.Ldetails = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.Label7 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.PanelEx1.SuspendLayout();
		this.PanelEx2.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx1.Controls.Add(this.ListView1);
		this.PanelEx1.Controls.Add(this.Panel1);
		this.PanelEx1.Controls.Add(this.WebBLOCK);
		this.PanelEx1.Controls.Add(this.ButtonX_0);
		this.PanelEx1.Controls.Add(this.Ldate);
		this.PanelEx1.Controls.Add(this.Label3);
		this.PanelEx1.Controls.Add(this.Label4);
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.TextBox2);
		this.PanelEx1.Controls.Add(this.ButtonX3);
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.ButtonX4);
		this.PanelEx1.Controls.Add(this.TextBox1);
		this.PanelEx1.Controls.Add(this.Label6);
		this.PanelEx1.Controls.Add(this.Label5);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Controls.Add(this.LabelRef);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(716, 300);
		panelEx3.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.Style.LineAlignment = System.Drawing.StringAlignment.Near;
		this.PanelEx1.TabIndex = 0;
		this.ListView1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[2] { this.ColumnHeader1, this.ColumnHeader2 });
		this.ListView1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListView1.FullRowSelect = true;
		this.ListView1.Items.AddRange(new System.Windows.Forms.ListViewItem[2] { listViewItem, listViewItem2 });
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(118, 126);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(581, 159);
		listView2.Size = size;
		this.ListView1.TabIndex = 26;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ListView1.Visible = false;
		this.ColumnHeader1.Text = "รห\u0e31สเคร\u0e37\u0e48อง";
		this.ColumnHeader1.Width = 250;
		this.ColumnHeader2.Text = "ล\u0e4aอครห\u0e31สก\u0e31บอ\u0e38ปกรณ\u0e4c";
		this.ColumnHeader2.Width = 300;
		this.Panel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Panel1.BackColor = System.Drawing.SystemColors.Control;
		this.Panel1.BackgroundImage = (System.Drawing.Image)resources.GetObject("Panel1.BackgroundImage");
		this.Panel1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Center;
		this.Panel1.Cursor = System.Windows.Forms.Cursors.Hand;
		System.Windows.Forms.Panel panel = this.Panel1;
		location = new System.Drawing.Point(669, 102);
		panel.Location = location;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel2 = this.Panel1;
		size = new System.Drawing.Size(28, 23);
		panel2.Size = size;
		this.Panel1.TabIndex = 25;
		this.Panel1.Tag = "เล\u0e37อกรห\u0e31สเคร\u0e37\u0e48อง";
		System.Windows.Forms.WebBrowser webBLOCK = this.WebBLOCK;
		location = new System.Drawing.Point(481, 236);
		webBLOCK.Location = location;
		System.Windows.Forms.WebBrowser webBLOCK2 = this.WebBLOCK;
		size = new System.Drawing.Size(20, 20);
		webBLOCK2.MinimumSize = size;
		this.WebBLOCK.Name = "WebBLOCK";
		System.Windows.Forms.WebBrowser webBLOCK3 = this.WebBLOCK;
		size = new System.Drawing.Size(57, 71);
		webBLOCK3.Size = size;
		this.WebBLOCK.TabIndex = 22;
		this.WebBLOCK.Visible = false;
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Checked = true;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.Office2007WithBackground;
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ButtonX11.Image");
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_0;
		location = new System.Drawing.Point(174, 196);
		buttonX_.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX_2 = this.ButtonX_0;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		buttonX_2.Margin = margin;
		this.ButtonX_0.Name = "ButtonX11";
		DevComponents.DotNetBar.ButtonX buttonX_3 = this.ButtonX_0;
		size = new System.Drawing.Size(283, 34);
		buttonX_3.Size = size;
		this.ButtonX_0.TabIndex = 16;
		this.ButtonX_0.Text = "ปลดบล\u0e4aอค";
		this.ButtonX_0.Visible = false;
		this.Ldate.AutoSize = true;
		System.Windows.Forms.Label ldate = this.Ldate;
		location = new System.Drawing.Point(460, 45);
		ldate.Location = location;
		System.Windows.Forms.Label ldate2 = this.Ldate;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		ldate2.Margin = margin;
		this.Ldate.Name = "Ldate";
		System.Windows.Forms.Label ldate3 = this.Ldate;
		size = new System.Drawing.Size(27, 19);
		ldate3.Size = size;
		this.Ldate.TabIndex = 14;
		this.Ldate.Text = "ว\u0e31น";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label = this.Label3;
		location = new System.Drawing.Point(512, 45);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label3;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label2.Margin = margin;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label3 = this.Label3;
		size = new System.Drawing.Size(27, 19);
		label3.Size = size;
		this.Label3.TabIndex = 15;
		this.Label3.Text = "ว\u0e31น";
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label4 = this.Label4;
		location = new System.Drawing.Point(175, 45);
		label4.Location = location;
		System.Windows.Forms.Label label5 = this.Label4;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label5.Margin = margin;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label6 = this.Label4;
		size = new System.Drawing.Size(135, 19);
		label6.Size = size;
		this.Label4.TabIndex = 12;
		this.Label4.Text = "โปรแกรมทดลองใช\u0e49";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label2;
		location = new System.Drawing.Point(319, 45);
		label7.Location = location;
		System.Windows.Forms.Label label8 = this.Label2;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label8.Margin = margin;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label9 = this.Label2;
		size = new System.Drawing.Size(118, 19);
		label9.Size = size;
		this.Label2.TabIndex = 13;
		this.Label2.Text = "เหล\u0e37อเวลาใช\u0e49งาน";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(175, 197);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		buttonX2.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		size = new System.Drawing.Size(282, 33);
		buttonX3.Size = size;
		this.ButtonX1.TabIndex = 11;
		this.ButtonX1.Text = "ทดลองใช\u0e49";
		System.Windows.Forms.MaskedTextBox textBox = this.TextBox2;
		location = new System.Drawing.Point(118, 149);
		textBox.Location = location;
		this.TextBox2.Mask = "AAAAA-AAAAA-AAAAA-AAAAA-AAAAA-AAAAA-AA";
		this.TextBox2.Name = "TextBox2";
		System.Windows.Forms.MaskedTextBox textBox2 = this.TextBox2;
		size = new System.Drawing.Size(491, 27);
		textBox2.Size = size;
		this.TextBox2.TabIndex = 10;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX3;
		location = new System.Drawing.Point(257, 238);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		buttonX5.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(72, 33);
		buttonX6.Size = size;
		this.ButtonX3.TabIndex = 6;
		this.ButtonX3.Text = "ลงทะเบ\u0e35ยน On Lan";
		this.ButtonX3.Visible = false;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX2;
		location = new System.Drawing.Point(616, 148);
		buttonX7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX2;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		buttonX8.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX2;
		size = new System.Drawing.Size(81, 27);
		buttonX9.Size = size;
		this.ButtonX2.TabIndex = 3;
		this.ButtonX2.Text = "ลงทะเบ\u0e35ยน";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX4;
		location = new System.Drawing.Point(465, 197);
		buttonX10.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX4;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		buttonX11.Margin = margin;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX4;
		size = new System.Drawing.Size(139, 33);
		buttonX12.Size = size;
		this.ButtonX4.TabIndex = 3;
		this.ButtonX4.Text = "ป\u0e34ดโปรแกรม";
		this.TextBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.TextBox1.BackColor = System.Drawing.Color.Moccasin;
		System.Windows.Forms.TextBox textBox3 = this.TextBox1;
		location = new System.Drawing.Point(118, 100);
		textBox3.Location = location;
		System.Windows.Forms.TextBox textBox4 = this.TextBox1;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		textBox4.Margin = margin;
		this.TextBox1.Name = "TextBox1";
		this.TextBox1.ReadOnly = true;
		System.Windows.Forms.TextBox textBox5 = this.TextBox1;
		size = new System.Drawing.Size(581, 27);
		textBox5.Size = size;
		this.TextBox1.TabIndex = 1;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label10 = this.Label6;
		location = new System.Drawing.Point(10, 153);
		label10.Location = location;
		System.Windows.Forms.Label label11 = this.Label6;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label11.Margin = margin;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label12 = this.Label6;
		size = new System.Drawing.Size(106, 19);
		label12.Size = size;
		this.Label6.TabIndex = 0;
		this.Label6.Text = "รห\u0e31สลงทะเบ\u0e35ยน";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label5;
		location = new System.Drawing.Point(40, 104);
		label13.Location = location;
		System.Windows.Forms.Label label14 = this.Label5;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label14.Margin = margin;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label15 = this.Label5;
		size = new System.Drawing.Size(76, 19);
		label15.Size = size;
		this.Label5.TabIndex = 0;
		this.Label5.Text = "รห\u0e31สเคร\u0e37\u0e48อง";
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label16 = this.Label1;
		location = new System.Drawing.Point(315, 11);
		label16.Location = location;
		System.Windows.Forms.Label label17 = this.Label1;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label17.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label18 = this.Label1;
		size = new System.Drawing.Size(122, 23);
		label18.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "iHotel 2022";
		this.LabelRef.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.LabelRef.BackColor = System.Drawing.Color.Transparent;
		this.LabelRef.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.LabelRef.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label labelRef = this.LabelRef;
		location = new System.Drawing.Point(120, 71);
		labelRef.Location = location;
		System.Windows.Forms.Label labelRef2 = this.LabelRef;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		labelRef2.Margin = margin;
		this.LabelRef.Name = "LabelRef";
		System.Windows.Forms.Label labelRef3 = this.LabelRef;
		size = new System.Drawing.Size(215, 36);
		labelRef3.Size = size;
		this.LabelRef.TabIndex = 28;
		this.LabelRef.Text = "รห\u0e31สอ\u0e49างอ\u0e34ง 12345";
		this.LabelRef.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.LabelRef.Visible = false;
		this.OpenFileDialog1.FileName = "Bookshop";
		this.OpenFileDialog1.Filter = "ไฟล\u0e4cลงทะเบ\u0e35ยน (*.reg)|*.reg";
		this.PanelEx2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx2.Controls.Add(this.Label7);
		this.PanelEx2.Controls.Add(this.Label8);
		this.PanelEx2.Controls.Add(this.ButtonX7);
		this.PanelEx2.Controls.Add(this.Lreg);
		this.PanelEx2.Controls.Add(this.Ldetails);
		this.PanelEx2.Controls.Add(this.Label10);
		this.PanelEx2.Controls.Add(this.Label11);
		this.PanelEx2.Controls.Add(this.Label13);
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		panelEx5.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx2;
		size = new System.Drawing.Size(716, 300);
		panelEx6.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.LineAlignment = System.Drawing.StringAlignment.Near;
		this.PanelEx2.TabIndex = 1;
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX7;
		location = new System.Drawing.Point(641, 253);
		buttonX13.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX7;
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		buttonX14.Margin = margin;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX7;
		size = new System.Drawing.Size(62, 33);
		buttonX15.Size = size;
		this.ButtonX7.TabIndex = 3;
		this.ButtonX7.Text = "ป\u0e34ด";
		this.Lreg.AutoSize = true;
		this.Lreg.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label lreg = this.Lreg;
		location = new System.Drawing.Point(176, 58);
		lreg.Location = location;
		System.Windows.Forms.Label lreg2 = this.Lreg;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		lreg2.Margin = margin;
		this.Lreg.Name = "Lreg";
		System.Windows.Forms.Label lreg3 = this.Lreg;
		size = new System.Drawing.Size(27, 19);
		lreg3.Size = size;
		this.Lreg.TabIndex = 0;
		this.Lreg.Text = "ว\u0e31น";
		this.Ldetails.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label ldetails = this.Ldetails;
		location = new System.Drawing.Point(176, 126);
		ldetails.Location = location;
		System.Windows.Forms.Label ldetails2 = this.Ldetails;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		ldetails2.Margin = margin;
		this.Ldetails.Name = "Ldetails";
		System.Windows.Forms.Label ldetails3 = this.Ldetails;
		size = new System.Drawing.Size(317, 118);
		ldetails3.Size = size;
		this.Ldetails.TabIndex = 0;
		this.Ldetails.Text = "ร";
		this.Label10.AutoSize = true;
		System.Windows.Forms.Label label19 = this.Label10;
		location = new System.Drawing.Point(130, 126);
		label19.Location = location;
		System.Windows.Forms.Label label20 = this.Label10;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label20.Margin = margin;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label21 = this.Label10;
		size = new System.Drawing.Size(34, 19);
		label21.Size = size;
		this.Label10.TabIndex = 0;
		this.Label10.Text = "Key";
		this.Label11.AutoSize = true;
		System.Windows.Forms.Label label22 = this.Label11;
		location = new System.Drawing.Point(58, 58);
		label22.Location = location;
		System.Windows.Forms.Label label23 = this.Label11;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label23.Margin = margin;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label24 = this.Label11;
		size = new System.Drawing.Size(106, 19);
		label24.Size = size;
		this.Label11.TabIndex = 0;
		this.Label11.Text = "ลงทะเบ\u0e35ยนโดย";
		this.Label13.AutoSize = true;
		this.Label13.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label13.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label25 = this.Label13;
		location = new System.Drawing.Point(295, 10);
		label25.Location = location;
		System.Windows.Forms.Label label26 = this.Label13;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label26.Margin = margin;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label27 = this.Label13;
		size = new System.Drawing.Size(122, 23);
		label27.Size = size;
		this.Label13.TabIndex = 0;
		this.Label13.Text = "iHotel 2022";
		this.Timer2.Interval = 5000;
		this.Label7.AutoSize = true;
		this.Label7.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label28 = this.Label7;
		location = new System.Drawing.Point(176, 91);
		label28.Location = location;
		System.Windows.Forms.Label label29 = this.Label7;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label29.Margin = margin;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label30 = this.Label7;
		size = new System.Drawing.Size(27, 19);
		label30.Size = size;
		this.Label7.TabIndex = 5;
		this.Label7.Text = "ว\u0e31น";
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label31 = this.Label8;
		location = new System.Drawing.Point(16, 91);
		label31.Location = location;
		System.Windows.Forms.Label label32 = this.Label8;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label32.Margin = margin;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label33 = this.Label8;
		size = new System.Drawing.Size(148, 19);
		label33.Size = size;
		this.Label8.TabIndex = 4;
		this.Label8.Text = "ลงทะเบ\u0e35ยนก\u0e31บอ\u0e38ปกรณ\u0e4c";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(9f, 19f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(716, 300);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "frmReg";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "การลงทะเบ\u0e35ยน";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx2.PerformLayout();
		this.ResumeLayout(false);
	}

	private void frmReg_FormClosing(object sender, FormClosingEventArgs e)
	{
		if (!ISOK)
		{
			Module1.close_program = 1;
		}
	}

	private void frmReg_Load(object sender, EventArgs e)
	{
		fromserver = "no";
		ACT_NOW = "no";
		bool flag = false;
		ArrayList cpuArr = Module1.cpuArr;
		string text = "";
		checked
		{
			int num = cpuArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				object objectValue = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 0 }, null));
				string string_ = Module1.string_0;
				object[] array = new object[1] { RuntimeHelpers.GetObjectValue(objectValue) };
				object[] arguments = array;
				bool[] array2 = new bool[1] { true };
				object left = NewLateBinding.LateGet(string_, null, "IndexOf", arguments, null, null, array2);
				if (array2[0])
				{
					objectValue = RuntimeHelpers.GetObjectValue(array[0]);
				}
				if (Operators.ConditionalCompareObjectNotEqual(left, -1, TextCompare: false))
				{
					if (Operators.CompareString(Module1.CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(objectValue, "HOTEL"))), Module1.RegCode, TextCompare: false) == 0)
					{
						text = Conversions.ToString(objectValue);
						comds = Conversions.ToString(NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 1 }, null));
						flag = true;
					}
					if (Operators.CompareString(Module1.CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(objectValue, "HOTEL2"))), Module1.RegCode, TextCompare: false) == 0)
					{
						text = Conversions.ToString(objectValue);
						comds = Conversions.ToString(NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 1 }, null));
						flag = true;
					}
				}
				num2++;
			}
			if (!flag)
			{
				int num5 = cpuArr.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					object objectValue2 = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(cpuArr[num6], new object[1] { 0 }, null));
					if (Operators.CompareString(Module1.CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(objectValue2, Module1.string_1))), Module1.RegCode, TextCompare: false) == 0)
					{
						text = Conversions.ToString(objectValue2);
						comds = Conversions.ToString(NewLateBinding.LateIndexGet(cpuArr[num6], new object[1] { 1 }, null));
						flag = true;
					}
					num6++;
				}
			}
			if (flag)
			{
				PanelEx1.Visible = false;
				PanelEx2.Visible = true;
				Lreg.Text = text + Check_Ref_Return(text);
				Label7.Text = comds;
				Ldetails.Text = Module1.RegCode;
			}
			else
			{
				PanelEx2.Visible = false;
				PanelEx1.Visible = true;
				TextBox1.Text = Conversions.ToString(NewLateBinding.LateIndexGet(cpuArr[0], new object[1] { 0 }, null));
				Ldate.Text = Conversions.ToString(DateAndTime.DateDiff("d", DateTime.Now, Module1.date_end) + 1L);
				if ((Conversions.ToDouble(Ldate.Text) <= 0.0) | (Module1.always == 1))
				{
					ButtonX1.Enabled = false;
				}
				Module1.COM_ID = TextBox1.Text;
			}
			gencodetolistview();
			if (!flag)
			{
				string hostName = Dns.GetHostName();
				if (File.Exists(Module1.Path_Program + hostName + "_adapter.txt"))
				{
					StreamReader streamReader = new StreamReader(Module1.Path_Program + hostName + "_adapter.txt", Encoding.Default);
					string encryptedString = streamReader.ReadLine();
					streamReader.Close();
					streamReader = null;
					object[] array3 = FormEN_DE.Decrypt1(encryptedString, "regadapter").Split('|');
					if (array3.Length == 2)
					{
						MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("เคร\u0e37\u0e48องน\u0e35\u0e49เคยลงทะเบ\u0e35ยนไปแล\u0e49ว\r\nรห\u0e31สเคร\u0e37\u0e48อง = ", array3[1]), Check_Ref_Return(Conversions.ToString(array3[1]))), ""), "\r\n"), "ลงทะเบ\u0e35ยนก\u0e31บอ\u0e38ปกรณ\u0e4c = "), array3[0]), ""), "\r\n"), "โปรดตรวจสอบได\u0e49กดป\u0e34ดไปหร\u0e37อเปล\u0e48า หล\u0e31งจากเป\u0e34ดแล\u0e49วให\u0e49เข\u0e49าโปรแกรมใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง")), "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
					}
				}
			}
			Check_Ref();
		}
	}

	public void gencodetolistview()
	{
		ArrayList cpuArr = Module1.cpuArr;
		ListView1.Items.Clear();
		checked
		{
			int num = cpuArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				bool flag = false;
				int num5 = ListView1.Items.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(ListView1.Items[num6].SubItems[0].Text, NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 0 }, null), TextCompare: false))
					{
						flag = true;
					}
					num6++;
				}
				if (!flag)
				{
					object objectValue = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 1 }, null));
					if (!Operators.ConditionalCompareObjectEqual(objectValue, "ช\u0e37\u0e48อบร\u0e34ษ\u0e31ท", TextCompare: false) && objectValue.ToString().ToLower().IndexOf("microsoft") == -1)
					{
						ListView.ListViewItemCollection items = ListView1.Items;
						object[] array = new object[1];
						object instance = cpuArr[num2];
						object[] array2 = new object[1];
						object[] array3 = array2;
						int num8 = 0;
						array3[0] = 0;
						array[0] = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(instance, array2, null));
						object[] array4 = array;
						object[] arguments = array4;
						bool[] array5 = new bool[1] { true };
						NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array5, IgnoreReturn: true);
						if (array5[0])
						{
							NewLateBinding.LateIndexSetComplex(instance, new object[2]
							{
								num8,
								RuntimeHelpers.GetObjectValue(array4[0])
							}, null, OptimisticSet: true, RValueBase: true);
						}
						ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[ListView1.Items.Count - 1].SubItems;
						array4 = new object[1] { RuntimeHelpers.GetObjectValue(objectValue) };
						object[] arguments2 = array4;
						array5 = new bool[1] { true };
						NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array5, IgnoreReturn: true);
						if (array5[0])
						{
							objectValue = RuntimeHelpers.GetObjectValue(array4[0]);
						}
					}
				}
				num2++;
			}
		}
	}

	public void gencodetolistviewAll()
	{
		ArrayList cpuArr = Module1.cpuArr;
		ListView1.Items.Clear();
		checked
		{
			int num = cpuArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				bool flag = false;
				int num5 = ListView1.Items.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(ListView1.Items[num6].SubItems[0].Text, NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 0 }, null), TextCompare: false))
					{
						flag = true;
					}
					num6++;
				}
				if (!flag)
				{
					object objectValue = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 1 }, null));
					ListView.ListViewItemCollection items = ListView1.Items;
					object[] array = new object[1];
					object instance = cpuArr[num2];
					object[] array2 = new object[1];
					object[] array3 = array2;
					int num8 = 0;
					array3[0] = 0;
					array[0] = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(instance, array2, null));
					object[] array4 = array;
					object[] arguments = array4;
					bool[] array5 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array5, IgnoreReturn: true);
					if (array5[0])
					{
						NewLateBinding.LateIndexSetComplex(instance, new object[2]
						{
							num8,
							RuntimeHelpers.GetObjectValue(array4[0])
						}, null, OptimisticSet: true, RValueBase: true);
					}
					ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[ListView1.Items.Count - 1].SubItems;
					array4 = new object[1] { RuntimeHelpers.GetObjectValue(objectValue) };
					object[] arguments2 = array4;
					array5 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array5, IgnoreReturn: true);
					if (array5[0])
					{
						objectValue = RuntimeHelpers.GetObjectValue(array4[0]);
					}
				}
				num2++;
			}
			MessageBox.Show("Code All Ok");
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Module1.RegCode, "855694122154121566451", TextCompare: false) == 0)
		{
			MessageBox.Show("เคร\u0e37\u0e48องน\u0e35\u0e49ได\u0e49ยกเล\u0e34กไปแล\u0e49ว กร\u0e38ณาต\u0e34ดต\u0e48อผ\u0e39\u0e49ด\u0e39แลโปรแกรม", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			ButtonX_0.Visible = true;
			return;
		}
		bool flag = false;
		ArrayList cpuArr = Module1.cpuArr;
		checked
		{
			int num = cpuArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (Operators.CompareString(Module1.CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(cpuArr[num2], new object[1] { 0 }, null), Module1.RegPro))), TextBox2.Text.Replace("-", ""), TextCompare: false) == 0)
				{
					Module1.RegOld = true;
					flag = true;
				}
				num2++;
			}
			if (!flag)
			{
				int num5 = cpuArr.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					if (Operators.CompareString(Module1.CalculateMD5(Conversions.ToString(Operators.ConcatenateObject(NewLateBinding.LateIndexGet(cpuArr[num6], new object[1] { 0 }, null), Module1.string_1))), TextBox2.Text.Replace("-", ""), TextCompare: false) == 0)
					{
						Module1.RegOld = false;
						flag = true;
					}
					num6++;
				}
			}
			if (flag)
			{
				StreamWriter streamWriter = File.CreateText(Module1.Path_Program + "\\reg.txt");
				streamWriter.WriteLine(TextBox2.Text.Replace("-", ""));
				streamWriter.Close();
				MessageBox.Show("การลงทะเบ\u0e35ยนเสร\u0e47จสมบ\u0e39รณ\u0e4c\r\nรห\u0e31สน\u0e35\u0e49จะอย\u0e39\u0e48ก\u0e31บเคร\u0e37\u0e48องน\u0e35\u0e49ตลอด จดรห\u0e31สน\u0e35\u0e49ไว\u0e49เวลาลง Windows ใหม\u0e48นะคร\u0e31บ " + TextBox2.Text + "");
				Module1.RegCode = TextBox2.Text.Replace("-", "");
				Module1.ReadSettingsConfig();
				ISOK = true;
				Module1.IS_TRIAL = false;
				Close();
			}
			else
			{
				MessageBox.Show("ไม\u0e48พบค\u0e35ย\u0e4cลงทะเบ\u0e35ยน");
			}
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		OpenFileDialog1.ShowDialog();
		TextBox2.Text = OpenFileDialog1.FileName;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		ISOK = true;
		Close();
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		ISOK = false;
		Close();
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		ISOK = true;
		Close();
	}

	private long GetFileSize(string url)
	{
		long result = 0L;
		try
		{
			FtpWebRequest ftpWebRequest = (FtpWebRequest)WebRequest.Create(new Uri(url));
			ftpWebRequest.Method = "SIZE";
			ftpWebRequest.UseBinary = true;
			ftpWebRequest.Credentials = new NetworkCredential(Conversions.ToString(FtpUsername), Conversions.ToString(FtpPassowrd));
			FtpWebResponse ftpWebResponse = (FtpWebResponse)ftpWebRequest.GetResponse();
			Stream responseStream = ftpWebResponse.GetResponseStream();
			result = ftpWebResponse.ContentLength;
			responseStream.Close();
			ftpWebResponse.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		return result;
	}

	public void downloadM_Before()
	{
		Uri requestUri = new Uri("ftp://stcord.no-ip.org/activate_hotel/computer/" + TextBox1.Text + ".txt");
		MemoryStream memoryStream = new MemoryStream();
		int num = 0;
		FtpWebRequest ftpWebRequest = (FtpWebRequest)WebRequest.Create(requestUri);
		ftpWebRequest.Credentials = new NetworkCredential(Conversions.ToString(FtpUsername), Conversions.ToString(FtpPassowrd));
		ftpWebRequest.Timeout = 60000;
		ftpWebRequest.KeepAlive = false;
		ftpWebRequest.UseBinary = true;
		ftpWebRequest.Method = "RETR";
		try
		{
			FtpWebResponse ftpWebResponse = (FtpWebResponse)ftpWebRequest.GetResponse();
			GetFileSize("ftp://stcord.no-ip.org/activate_hotel/computer/" + TextBox1.Text + ".txt");
			Stream responseStream = ftpWebResponse.GetResponseStream();
			byte[] array = new byte[1025];
			int num2 = responseStream.Read(array, 0, array.Length);
			num = num2;
			while (num2 > 0)
			{
				memoryStream.Write(array, 0, num2);
				num2 = responseStream.Read(array, 0, array.Length);
				num = checked(num + num2);
			}
			memoryStream.Flush();
			memoryStream.Seek(0L, SeekOrigin.Begin);
			responseStream.Close();
			ftpWebResponse.Close();
			StreamReader streamReader = new StreamReader(memoryStream, Encoding.UTF8);
			ACT_NOW = streamReader.ReadToEnd();
			streamReader.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public void downloadM()
	{
		Uri requestUri = new Uri(UpdateUr2);
		MemoryStream memoryStream = new MemoryStream();
		int num = 0;
		FtpWebRequest ftpWebRequest = (FtpWebRequest)WebRequest.Create(requestUri);
		ftpWebRequest.Credentials = new NetworkCredential(Conversions.ToString(FtpUsername), Conversions.ToString(FtpPassowrd));
		ftpWebRequest.Timeout = 60000;
		ftpWebRequest.KeepAlive = false;
		ftpWebRequest.UseBinary = true;
		ftpWebRequest.Method = "RETR";
		try
		{
			FtpWebResponse ftpWebResponse = (FtpWebResponse)ftpWebRequest.GetResponse();
			GetFileSize(UpdateUr2);
			Stream responseStream = ftpWebResponse.GetResponseStream();
			byte[] array = new byte[1025];
			int num2 = responseStream.Read(array, 0, array.Length);
			num = num2;
			while (num2 > 0)
			{
				memoryStream.Write(array, 0, num2);
				num2 = responseStream.Read(array, 0, array.Length);
				num = checked(num + num2);
			}
			memoryStream.Flush();
			memoryStream.Seek(0L, SeekOrigin.Begin);
			responseStream.Close();
			ftpWebResponse.Close();
			StreamReader streamReader = new StreamReader(memoryStream, Encoding.UTF8);
			fromserver = streamReader.ReadToEnd();
			streamReader.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			MessageBox.Show("ไม\u0e48พบค\u0e35ย\u0e4cลงทะเบ\u0e35ยน");
			ProjectData.ClearProjectError();
		}
	}

	private void ButtonX3_Click_1(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select CompanyName,Company_Tax from TB_SETTINGS");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("Error 101", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			return;
		}
		object obj = Interaction.InputBox("กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อหน\u0e48วยงาน", "กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อหน\u0e48วยงาน", Conversions.ToString(dataSet.Tables[0].Rows[0]["CompanyName"]));
		if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			return;
		}
		object obj2 = Interaction.InputBox(Conversions.ToString(Operators.ConcatenateObject("กร\u0e38ณาใส\u0e48เลขประจำต\u0e31วผ\u0e39\u0e49เส\u0e35ยภาษ\u0e35 ของ ", obj)), "กร\u0e38ณาใส\u0e48เลขประจำต\u0e31วผ\u0e39\u0e49เส\u0e35ยภาษ\u0e35", Conversions.ToString(dataSet.Tables[0].Rows[0]["Company_Tax"]));
		if (!Operators.ConditionalCompareObjectEqual(obj2, "", TextCompare: false))
		{
			object obj3 = Interaction.InputBox(Conversions.ToString(Operators.ConcatenateObject("กร\u0e38ณาใส\u0e48รห\u0e31สลงทะเบ\u0e35ยน ของ ", obj)), "กร\u0e38ณาใส\u0e48รห\u0e31สลงทะเบ\u0e35ยน", TextBox2.Text);
			if (!Operators.ConditionalCompareObjectEqual(obj3, "", TextCompare: false))
			{
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update TB_SETTINGS set CompanyName='", obj), "',Company_Tax='"), obj2), "'")));
				TextBox1.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(obj2, "/"), obj));
				TextBox2.Text = Conversions.ToString(obj3);
				ButtonX2_Click(null, null);
			}
		}
	}

	public void Writekey1()
	{
		FtpWebRequest ftpWebRequest = (FtpWebRequest)WebRequest.Create(UpdateUr2);
		ftpWebRequest.Credentials = new NetworkCredential(Conversions.ToString(Operators.ConcatenateObject(FtpUsername, "2")), Conversions.ToString(FtpPassowrd));
		ftpWebRequest.Method = "STOR";
		byte[] bytes = Encoding.ASCII.GetBytes(TextBox1.Text);
		Stream requestStream = ftpWebRequest.GetRequestStream();
		requestStream.Write(bytes, 0, bytes.Length);
		requestStream.Close();
		requestStream.Dispose();
	}

	public void Writekey2()
	{
		FtpWebRequest ftpWebRequest = (FtpWebRequest)WebRequest.Create("ftp://stcord.no-ip.org/activate_hotel/computer/" + TextBox1.Text + ".txt");
		ftpWebRequest.Credentials = new NetworkCredential(Conversions.ToString(Operators.ConcatenateObject(FtpUsername, "2")), Conversions.ToString(FtpPassowrd));
		ftpWebRequest.Method = "STOR";
		byte[] bytes = Encoding.ASCII.GetBytes(TextBox1.Text);
		Stream requestStream = ftpWebRequest.GetRequestStream();
		requestStream.Write(bytes, 0, bytes.Length);
		requestStream.Close();
		requestStream.Dispose();
	}

	private void ButtonX1_Click_1(object sender, EventArgs e)
	{
		ISOK = true;
		Close();
	}

	private void ButtonX11_Click(object sender, EventArgs e)
	{
		try
		{
			WebBLOCK.Url = new Uri("http://www.kpsystem.co.th/chk_hotel.php");
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void WebBLOCK_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		string documentText = WebBLOCK.DocumentText;
		if (documentText.IndexOf(TextBox1.Text) != -1)
		{
			MessageBox.Show("ไม\u0e48สามารถปลอบล\u0e4aอคได\u0e49 เน\u0e37\u0e48องจากเคร\u0e37\u0e48อง " + TextBox1.Text + " ได\u0e49เล\u0e34กใช\u0e49งานไปแล\u0e49ว กร\u0e38ณาต\u0e34ดต\u0e48อผ\u0e39\u0e49ด\u0e39แลโปรแกรม", "ERROR!!", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else if ((documentText.IndexOf(TextBox1.Text) == -1) & (documentText.IndexOf("123456789123") != -1))
		{
			StreamWriter streamWriter = new StreamWriter(Module1.PathF + "reg.txt");
			streamWriter.Write("1234");
			streamWriter.Close();
			MessageBox.Show("ปลดบล\u0e4aอคเสร\u0e47จสมบ\u0e39รณ\u0e4c");
			ButtonX4_Click(null, null);
		}
		else
		{
			MessageBox.Show("ไม\u0e48สามารถต\u0e34ดต\u0e48อก\u0e31บเซ\u0e34ฟเวอร\u0e4cได\u0e49");
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			TextBox1.Text = ListView1.SelectedItems[0].SubItems[0].Text;
			ListView1.Visible = false;
			Check_Ref();
		}
	}

	public void Check_Ref(string idd = "")
	{
		LabelRef.Text = "";
		LabelRef.Visible = false;
		if (Operators.CompareString(idd, "", TextCompare: false) == 0)
		{
			idd = TextBox1.Text;
		}
		string text = Module1.WEB_LOAD_GET("http://www.kpsystem.co.th/chk_lotto_ref.php?comid=" + idd);
		if (Operators.CompareString(text, "", TextCompare: false) != 0)
		{
			if (Versioned.IsNumeric(Strings.Trim(text)))
			{
				LabelRef.Text = "(รห\u0e31สอ\u0e49างอ\u0e34ง " + Strings.Trim(text) + ")";
				LabelRef.Visible = true;
			}
		}
		else
		{
			Timer2.Enabled = true;
		}
	}

	public string Check_Ref_Return(string idd = "")
	{
		string result = "";
		string text = Module1.WEB_LOAD_GET("http://www.kpsystem.co.th/chk_lotto_ref.php?comid=" + idd);
		if (Operators.CompareString(text, "", TextCompare: false) != 0 && Versioned.IsNumeric(Strings.Trim(text)))
		{
			result = " (รห\u0e31สอ\u0e49างอ\u0e34ง " + Strings.Trim(text) + ")";
		}
		return result;
	}

	private void Timer2_Tick(object sender, EventArgs e)
	{
		Timer2.Enabled = false;
		Check_Ref();
	}

	private void Panel1_MouseClick(object sender, MouseEventArgs e)
	{
		if (ListView1.Visible)
		{
			ListView1.Visible = false;
		}
		else
		{
			ListView1.Visible = true;
		}
	}

	private void Panel1_Paint(object sender, PaintEventArgs e)
	{
	}

	private void Label5_Click(object sender, EventArgs e)
	{
		checked
		{
			clickasss++;
			if (clickasss >= 5)
			{
				gencodetolistviewAll();
			}
		}
	}
}
