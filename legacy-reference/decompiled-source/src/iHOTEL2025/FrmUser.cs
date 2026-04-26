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
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmUser : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX9")]
	private ButtonX _ButtonX9;

	[AccessedThroughProperty("Bsave")]
	private ButtonX _Bsave;

	[AccessedThroughProperty("Cb1")]
	private ComboBox _Cb1;

	[AccessedThroughProperty("Tbname")]
	private TextBox _Tbname;

	[AccessedThroughProperty("Tbpass")]
	private TextBox _Tbpass;

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("Tbuser")]
	private TextBox _Tbuser;

	[AccessedThroughProperty("Bdel")]
	private ButtonX _Bdel;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Badd")]
	private ButtonX _Badd;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("panelEx1")]
	private PanelEx _panelEx1;

	[AccessedThroughProperty("Highlighter1")]
	private Highlighter _Highlighter1;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	private string bstatus;

	private string tid;

	private DataSet data;

	private int i;

	internal virtual ButtonX ButtonX9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX9_Click;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click -= value2;
			}
			_ButtonX9 = value;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click += value2;
			}
		}
	}

	internal virtual ButtonX Bsave
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bsave;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bsave_Click;
			if (_Bsave != null)
			{
				_Bsave.Click -= value2;
			}
			_Bsave = value;
			if (_Bsave != null)
			{
				_Bsave.Click += value2;
			}
		}
	}

	internal virtual ComboBox Cb1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Cb1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Cb1 = value;
		}
	}

	internal virtual TextBox Tbname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tbname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tbname = value;
		}
	}

	internal virtual TextBox Tbpass
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tbpass;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tbpass = value;
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

	internal virtual TextBox Tbuser
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tbuser;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tbuser = value;
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

	internal virtual ButtonX Badd
	{
		[DebuggerNonUserCode]
		get
		{
			return _Badd;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Badd_Click;
			if (_Badd != null)
			{
				_Badd.Click -= value2;
			}
			_Badd = value;
			if (_Badd != null)
			{
				_Badd.Click += value2;
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

	internal virtual PanelEx panelEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _panelEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_panelEx1 = value;
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

	[DebuggerNonUserCode]
	static FrmUser()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmUser()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += EditUser_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		bstatus = "";
		tid = "";
		data = new DataSet();
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
		this.ButtonX9 = new DevComponents.DotNetBar.ButtonX();
		this.Cb1 = new System.Windows.Forms.ComboBox();
		this.Tbname = new System.Windows.Forms.TextBox();
		this.Tbpass = new System.Windows.Forms.TextBox();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.Tbuser = new System.Windows.Forms.TextBox();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.panelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Bsave = new DevComponents.DotNetBar.ButtonX();
		this.Bdel = new DevComponents.DotNetBar.ButtonX();
		this.Badd = new DevComponents.DotNetBar.ButtonX();
		this.Highlighter1 = new DevComponents.DotNetBar.Validator.Highlighter();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.SuspendLayout();
		this.ButtonX9.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX9.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX9;
		System.Drawing.Point location = new System.Drawing.Point(770, 160);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX9;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX9.Name = "ButtonX9";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX9;
		System.Drawing.Size size = new System.Drawing.Size(35, 28);
		buttonX3.Size = size;
		this.ButtonX9.TabIndex = 6;
		this.ButtonX9.Text = "+-";
		this.Cb1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Cb1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Cb1.FormattingEnabled = true;
		this.Highlighter1.SetHighlightOnFocus(this.Cb1, true);
		System.Windows.Forms.ComboBox cb = this.Cb1;
		location = new System.Drawing.Point(549, 161);
		cb.Location = location;
		System.Windows.Forms.ComboBox cb2 = this.Cb1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		cb2.Margin = margin;
		this.Cb1.Name = "Cb1";
		System.Windows.Forms.ComboBox cb3 = this.Cb1;
		size = new System.Drawing.Size(214, 24);
		cb3.Size = size;
		this.Cb1.TabIndex = 5;
		this.Tbname.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tbname.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tbname, true);
		System.Windows.Forms.TextBox tbname = this.Tbname;
		location = new System.Drawing.Point(549, 65);
		tbname.Location = location;
		System.Windows.Forms.TextBox tbname2 = this.Tbname;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tbname2.Margin = margin;
		this.Tbname.Name = "Tbname";
		System.Windows.Forms.TextBox tbname3 = this.Tbname;
		size = new System.Drawing.Size(214, 23);
		tbname3.Size = size;
		this.Tbname.TabIndex = 0;
		this.Tbpass.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tbpass.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tbpass, true);
		System.Windows.Forms.TextBox tbpass = this.Tbpass;
		location = new System.Drawing.Point(549, 128);
		tbpass.Location = location;
		System.Windows.Forms.TextBox tbpass2 = this.Tbpass;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tbpass2.Margin = margin;
		this.Tbpass.Name = "Tbpass";
		this.Tbpass.PasswordChar = '•';
		System.Windows.Forms.TextBox tbpass3 = this.Tbpass;
		size = new System.Drawing.Size(116, 23);
		tbpass3.Size = size;
		this.Tbpass.TabIndex = 2;
		this.Tbpass.UseSystemPasswordChar = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[5] { this.ColumnHeader1, this.ColumnHeader5, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4 });
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(7, 49);
		listView.Location = location;
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView3 = this.ListView1;
		size = new System.Drawing.Size(432, 260);
		listView3.Size = size;
		this.ListView1.TabIndex = 10;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader1.Width = 30;
		this.ColumnHeader5.Width = 0;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 123;
		this.ColumnHeader3.Text = "UserName";
		this.ColumnHeader3.Width = 123;
		this.ColumnHeader4.Text = "Level";
		this.ColumnHeader4.Width = 122;
		this.Tbuser.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Tbuser.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Highlighter1.SetHighlightOnFocus(this.Tbuser, true);
		System.Windows.Forms.TextBox tbuser = this.Tbuser;
		location = new System.Drawing.Point(549, 97);
		tbuser.Location = location;
		System.Windows.Forms.TextBox tbuser2 = this.Tbuser;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tbuser2.Margin = margin;
		this.Tbuser.Name = "Tbuser";
		System.Windows.Forms.TextBox tbuser3 = this.Tbuser;
		size = new System.Drawing.Size(116, 23);
		tbuser3.Size = size;
		this.Tbuser.TabIndex = 1;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label1.AutoSize = true;
		this.Label1.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(514, 70);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(33, 16);
		label2.Size = size;
		this.Label1.TabIndex = 18;
		this.Label1.Text = "ช\u0e37\u0e48อ :";
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label2.AutoSize = true;
		this.Label2.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(475, 101);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(75, 16);
		label4.Size = size;
		this.Label2.TabIndex = 20;
		this.Label2.Text = "Username :";
		this.Label3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label3.AutoSize = true;
		this.Label3.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(477, 133);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(72, 16);
		label6.Size = size;
		this.Label3.TabIndex = 21;
		this.Label3.Text = "Password :";
		this.Label5.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label5.AutoSize = true;
		this.Label5.BackColor = System.Drawing.Color.Transparent;
		System.Windows.Forms.Label label7 = this.Label5;
		location = new System.Drawing.Point(451, 166);
		label7.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label8 = this.Label5;
		size = new System.Drawing.Size(97, 16);
		label8.Size = size;
		this.Label5.TabIndex = 19;
		this.Label5.Text = "ระด\u0e31บการเข\u0e49าถ\u0e36ง :";
		this.panelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.panelEx1.Dock = System.Windows.Forms.DockStyle.Top;
		this.panelEx1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.panelEx1;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.panelEx1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.panelEx1.Name = "panelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.panelEx1;
		size = new System.Drawing.Size(812, 39);
		panelEx3.Size = size;
		this.panelEx1.Style.BackColor1.Color = System.Drawing.Color.FromArgb(95, 136, 215);
		this.panelEx1.Style.BackColor2.Color = System.Drawing.Color.FromArgb(67, 108, 191);
		this.panelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.panelEx1.Style.GradientAngle = 90;
		this.panelEx1.Style.MarginLeft = 8;
		this.panelEx1.TabIndex = 30;
		this.panelEx1.Text = "จ\u0e31ดการข\u0e49อม\u0e39ลผ\u0e39\u0e49ใช\u0e49";
		this.Bsave.AccessibleRole = System.Windows.Forms.AccessibleRole.CheckButton;
		this.Bsave.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		DevComponents.DotNetBar.ButtonX bsave = this.Bsave;
		location = new System.Drawing.Point(619, 279);
		bsave.Location = location;
		DevComponents.DotNetBar.ButtonX bsave2 = this.Bsave;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bsave2.Margin = margin;
		this.Bsave.Name = "Bsave";
		DevComponents.DotNetBar.ButtonX bsave3 = this.Bsave;
		size = new System.Drawing.Size(87, 28);
		bsave3.Size = size;
		this.Bsave.TabIndex = 8;
		this.Bsave.Text = "บ\u0e31นท\u0e36ก";
		this.Bdel.AccessibleRole = System.Windows.Forms.AccessibleRole.CheckButton;
		this.Bdel.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		DevComponents.DotNetBar.ButtonX bdel = this.Bdel;
		location = new System.Drawing.Point(714, 279);
		bdel.Location = location;
		DevComponents.DotNetBar.ButtonX bdel2 = this.Bdel;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bdel2.Margin = margin;
		this.Bdel.Name = "Bdel";
		DevComponents.DotNetBar.ButtonX bdel3 = this.Bdel;
		size = new System.Drawing.Size(87, 28);
		bdel3.Size = size;
		this.Bdel.TabIndex = 9;
		this.Bdel.Text = "ลบ";
		this.Badd.AccessibleRole = System.Windows.Forms.AccessibleRole.CheckButton;
		this.Badd.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		DevComponents.DotNetBar.ButtonX badd = this.Badd;
		location = new System.Drawing.Point(525, 279);
		badd.Location = location;
		DevComponents.DotNetBar.ButtonX badd2 = this.Badd;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		badd2.Margin = margin;
		this.Badd.Name = "Badd";
		DevComponents.DotNetBar.ButtonX badd3 = this.Badd;
		size = new System.Drawing.Size(87, 28);
		badd3.Size = size;
		this.Badd.TabIndex = 7;
		this.Badd.Text = "เพ\u0e34\u0e48ม";
		this.Highlighter1.ContainerControl = this;
		this.Highlighter1.FocusHighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Orange;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(674, 130);
		checkBox.Location = location;
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		checkBox2.Margin = margin;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox3 = this.CheckBox1;
		size = new System.Drawing.Size(64, 20);
		checkBox3.Size = size;
		this.CheckBox1.TabIndex = 31;
		this.CheckBox1.Text = "V Only";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.CheckBox1.Visible = false;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(812, 320);
		this.ClientSize = size;
		this.Controls.Add(this.CheckBox1);
		this.Controls.Add(this.panelEx1);
		this.Controls.Add(this.ButtonX9);
		this.Controls.Add(this.Bsave);
		this.Controls.Add(this.Cb1);
		this.Controls.Add(this.Tbname);
		this.Controls.Add(this.Tbpass);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Tbuser);
		this.Controls.Add(this.Bdel);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.Badd);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Label5);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		size = new System.Drawing.Size(700, 283);
		this.MinimumSize = size;
		this.Name = "FrmUser";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "จ\u0e31ดการข\u0e49อม\u0e39ลผ\u0e39\u0e49ใช\u0e49";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		Form form = new FrmPermission();
		form.ShowDialog();
		listLevel();
	}

	private void Badd_Click(object sender, EventArgs e)
	{
		cleartextbox();
		enableTextBox(status: true);
		tid = Conversions.ToString(Module1.get_id("TB_MRP_EMPLOYEE", "ID"));
		Badd.Enabled = false;
		Bdel.Enabled = true;
		Bsave.Enabled = true;
		Tbname.Focus();
	}

	public void enableTextBox(bool status)
	{
		Tbname.Enabled = status;
		Tbuser.Enabled = status;
		Tbpass.Enabled = status;
		Cb1.Enabled = status;
	}

	public void cleartextbox()
	{
		Tbname.Clear();
		Tbuser.Clear();
		Tbpass.Clear();
		Cb1.Text = "เล\u0e37อก";
		CheckBox1.Checked = false;
	}

	public void listuser()
	{
		data = Module1.connect("SELECT * From View_MRP_EMPLOYEE");
		if (data.Tables[0].Rows.Count == 0)
		{
			return;
		}
		ListView1.Items.Clear();
		checked
		{
			int num = data.Tables[0].Rows.Count - 1;
			i = 0;
			while (true)
			{
				int num2 = i;
				int num3 = num;
				if (num2 <= num3)
				{
					ListViewItem listViewItem = new ListViewItem();
					listViewItem = new ListViewItem((ListView1.Items.Count + 1).ToString(), 0);
					ListViewItem.ListViewSubItemCollection subItems = listViewItem.SubItems;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = data.Tables[0].Rows[i];
					DataRow dataRow2 = dataRow;
					string columnName = "id";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listViewItem.SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = data.Tables[0].Rows[i];
					DataRow dataRow3 = dataRow;
					columnName = "EMP_Name";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems3 = listViewItem.SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = data.Tables[0].Rows[i];
					DataRow dataRow4 = dataRow;
					columnName = "EMP_Username";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems4 = listViewItem.SubItems;
					array3 = new object[1];
					object[] array7 = array3;
					dataRow = data.Tables[0].Rows[i];
					DataRow dataRow5 = dataRow;
					columnName = "EMP_level";
					array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
					array = array3;
					object[] arguments4 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListView1.Items.AddRange(new ListViewItem[1] { listViewItem });
					i++;
					continue;
				}
				break;
			}
		}
	}

	private void Bdel_Click(object sender, EventArgs e)
	{
		if (!Badd.Enabled)
		{
			cleartextbox();
			Badd.Enabled = true;
			Bdel.Enabled = false;
			Bsave.Enabled = false;
		}
		else if (ListView1.SelectedItems.Count != 0 && MessageBox.Show("ค\u0e38ณต\u0e49องการลบ " + ListView1.SelectedItems[0].SubItems[2].Text + " หร\u0e37อไม\u0e48", "ลบผ\u0e39\u0e49ใช\u0e49งาน", MessageBoxButtons.YesNo, MessageBoxIcon.Exclamation) == DialogResult.Yes)
		{
			string command = "DELETE From TB_MRP_EMPLOYEE where id=" + ListView1.SelectedItems[0].SubItems[1].Text;
			Module1.connect(command).Clear();
			enableTextBox(status: false);
			MessageBox.Show("ลบรายการเสร\u0e47จเร\u0e35ยบร\u0e49อย!!");
			listuser();
			cleartextbox();
			Badd.Enabled = true;
			Bdel.Enabled = false;
			Bsave.Enabled = false;
		}
	}

	private void EditUser_Load(object sender, EventArgs e)
	{
		listuser();
		listLevel();
		enableTextBox(status: false);
		Bsave.Enabled = false;
		Bdel.Enabled = false;
		Tbname.Focus();
	}

	public void listLevel()
	{
		Cb1.Items.Clear();
		DataSet dataSet = Module1.connect("SELECT Level_Name From TB_MRP_PERMISSION group by Level_Name");
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			i = 0;
			while (true)
			{
				int num2 = i;
				int num3 = num;
				if (num2 <= num3)
				{
					Cb1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[i]["Level_Name"]));
					i++;
					continue;
				}
				break;
			}
		}
	}

	public bool checkError()
	{
		if (Operators.CompareString(Tbname.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48 ช\u0e37\u0e48อ", "ผ\u0e34ดพลาด", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Tbname.Focus();
			return false;
		}
		if (Operators.CompareString(Tbuser.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48 Username", "ผ\u0e34ดพลาด", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Tbuser.Focus();
			return false;
		}
		if (Operators.CompareString(Tbpass.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48 Password", "ผ\u0e34ดพลาด", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Tbpass.Focus();
			return false;
		}
		if (Operators.CompareString(Cb1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48 เล\u0e37อกระด\u0e31บการเข\u0e49าถ\u0e36ง", "ผ\u0e34ดพลาด", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Cb1.Focus();
			return false;
		}
		return true;
	}

	private void Bsave_Click(object sender, EventArgs e)
	{
		if (!checkError())
		{
			return;
		}
		checked
		{
			if (!Badd.Enabled)
			{
				int num = ListView1.Items.Count - 1;
				i = 0;
				while (true)
				{
					int num2 = i;
					int num3 = num;
					if (num2 > num3)
					{
						break;
					}
					if (Operators.CompareString(ListView1.Items[i].SubItems[2].Text.ToLower(), Tbuser.Text.ToLower(), TextCompare: false) != 0)
					{
						i++;
						continue;
					}
					MessageBox.Show("ม\u0e35ช\u0e37\u0e48อ " + Tbuser.Text + " อย\u0e39\u0e48แล\u0e49ว");
					Tbuser.Focus();
					return;
				}
				string text = "INSERT INTO [TB_MRP_EMPLOYEE]";
				text += "([Emp_Username]";
				text += ",[Emp_Password]";
				text += ",[Emp_Name]";
				text += ",[Emp_Level],[EMP_VAT])";
				text += "VALUES(";
				text = text + "'" + Tbuser.Text + "'";
				text = text + ",'" + Tbpass.Text + "'";
				text = text + ",'" + Tbname.Text + "'";
				text = text + ",'" + Cb1.Text + "'";
				text = text + ",'" + Conversions.ToString(CheckBox1.Checked) + "'";
				text += ")";
				Module1.connect(text).Clear();
				listuser();
				enableTextBox(status: false);
				MessageBox.Show("เพ\u0e34\u0e48มข\u0e49อม\u0e39ลเสร\u0e47จเร\u0e35ยบร\u0e49อย!!");
			}
			else
			{
				string text2 = "UPDATE TB_MRP_EMPLOYEE SET ";
				text2 = text2 + "[Emp_Username]='" + Tbuser.Text + "'";
				text2 = text2 + ", [Emp_Password]='" + Tbpass.Text + "'";
				text2 = text2 + ", [Emp_Name]='" + Tbname.Text + "'";
				text2 = text2 + ", [Emp_level]='" + Cb1.Text + "'";
				text2 = text2 + " where id=" + tid;
				Module1.connect(text2).Clear();
				listuser();
				enableTextBox(status: false);
				MessageBox.Show("อ\u0e31บเดทข\u0e49อม\u0e39ลเสร\u0e47จเร\u0e35ยบร\u0e49อย!!");
			}
			cleartextbox();
			Badd.Enabled = true;
			Bdel.Enabled = false;
			Bsave.Enabled = false;
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			data = Module1.connect("SELECT * From TB_MRP_EMPLOYEE where id =" + ListView1.SelectedItems[0].SubItems[1].Text);
			tid = Conversions.ToString(data.Tables[0].Rows[0]["id"]);
			Tbname.Text = Conversions.ToString(data.Tables[0].Rows[0]["EMP_Name"]);
			Tbuser.Text = Conversions.ToString(data.Tables[0].Rows[0]["EMP_Username"]);
			Tbpass.Text = Conversions.ToString(data.Tables[0].Rows[0]["EMP_Password"]);
			Cb1.Text = Conversions.ToString(data.Tables[0].Rows[0]["EMP_level"]);
			if (Operators.CompareString(data.Tables[0].Rows[0]["EMP_vat"].ToString().ToUpper(), "TRUE", TextCompare: false) == 0)
			{
				CheckBox1.Checked = true;
			}
			else
			{
				CheckBox1.Checked = false;
			}
			Badd.Enabled = true;
			Bdel.Enabled = true;
			Bsave.Enabled = true;
		}
		enableTextBox(status: true);
	}
}
