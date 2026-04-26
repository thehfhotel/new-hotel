using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Management;
using System.Net;
using System.Runtime.CompilerServices;
using System.Security.AccessControl;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using DevComponents.DotNetBar.Validator;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmAddEditServer : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("PanelEx6")]
	private PanelEx _PanelEx6;

	[AccessedThroughProperty("Tpath")]
	private TextBoxX _Tpath;

	[AccessedThroughProperty("LabelX28")]
	private LabelX _LabelX28;

	[AccessedThroughProperty("PanelEx5")]
	private PanelEx _PanelEx5;

	[AccessedThroughProperty("Highlighter1")]
	private Highlighter _Highlighter1;

	[AccessedThroughProperty("ItemPanel1")]
	private ItemPanel _ItemPanel1;

	[AccessedThroughProperty("ControlContainerItem1")]
	private ControlContainerItem _ControlContainerItem1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("OpenFileDialog1")]
	private OpenFileDialog _OpenFileDialog1;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("TextBox_password")]
	private TextBoxX _TextBox_password;

	[AccessedThroughProperty("Lpass")]
	private LabelX _Lpass;

	[AccessedThroughProperty("TextBox_user")]
	private TextBoxX _TextBox_user;

	[AccessedThroughProperty("Luser")]
	private LabelX _Luser;

	[AccessedThroughProperty("Ldb")]
	private LabelX _Ldb;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("Tdbname")]
	private TextBoxX _Tdbname;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("Bpass")]
	private ButtonX _Bpass;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("ButtonX10")]
	private ButtonX _ButtonX10;

	[AccessedThroughProperty("Tnote")]
	private TextBoxX _Tnote;

	[AccessedThroughProperty("LabelX5")]
	private LabelX _LabelX5;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	public bool ISOK;

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

	internal virtual PanelEx PanelEx6
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx6 = value;
		}
	}

	internal virtual TextBoxX Tpath
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpath;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpath = value;
		}
	}

	internal virtual LabelX LabelX28
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX28;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX28 = value;
		}
	}

	internal virtual PanelEx PanelEx5
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = PanelEx5_Click;
			if (_PanelEx5 != null)
			{
				_PanelEx5.Click -= value2;
			}
			_PanelEx5 = value;
			if (_PanelEx5 != null)
			{
				_PanelEx5.Click += value2;
			}
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

	internal virtual ControlContainerItem ControlContainerItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ControlContainerItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ControlContainerItem1 = value;
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
			EventHandler value2 = ComboBox1_SelectedIndexChanged;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged -= value2;
			}
			_ComboBox1 = value;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual LabelX LabelX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX3 = value;
		}
	}

	internal virtual TextBoxX TextBox_password
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox_password;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TextBox_password_TextChanged;
			if (_TextBox_password != null)
			{
				_TextBox_password.TextChanged -= value2;
			}
			_TextBox_password = value;
			if (_TextBox_password != null)
			{
				_TextBox_password.TextChanged += value2;
			}
		}
	}

	internal virtual LabelX Lpass
	{
		[DebuggerNonUserCode]
		get
		{
			return _Lpass;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Lpass = value;
		}
	}

	internal virtual TextBoxX TextBox_user
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox_user;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = TextBox_user_LostFocus;
			EventHandler value3 = TextBox_user_TextChanged;
			if (_TextBox_user != null)
			{
				_TextBox_user.LostFocus -= value2;
				_TextBox_user.TextChanged -= value3;
			}
			_TextBox_user = value;
			if (_TextBox_user != null)
			{
				_TextBox_user.LostFocus += value2;
				_TextBox_user.TextChanged += value3;
			}
		}
	}

	internal virtual LabelX Luser
	{
		[DebuggerNonUserCode]
		get
		{
			return _Luser;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Luser = value;
		}
	}

	internal virtual LabelX Ldb
	{
		[DebuggerNonUserCode]
		get
		{
			return _Ldb;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Ldb = value;
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

	internal virtual TextBoxX Tdbname
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tdbname;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tdbname = value;
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

	internal virtual ButtonX Bpass
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bpass;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX4_Click;
			if (_Bpass != null)
			{
				_Bpass.Click -= value2;
			}
			_Bpass = value;
			if (_Bpass != null)
			{
				_Bpass.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX5_Click;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click -= value2;
			}
			_ButtonX5 = value;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX10_Click;
			if (_ButtonX10 != null)
			{
				_ButtonX10.Click -= value2;
			}
			_ButtonX10 = value;
			if (_ButtonX10 != null)
			{
				_ButtonX10.Click += value2;
			}
		}
	}

	internal virtual TextBoxX Tnote
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tnote;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tnote = value;
		}
	}

	internal virtual LabelX LabelX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX5 = value;
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
			EventHandler value2 = ButtonX4_Click_1;
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

	[DebuggerNonUserCode]
	static FrmAddEditServer()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmAddEditServer()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmAddEditServer_FormClosing;
		base.Load += FrmSettingsServer_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISOK = false;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmAddEditServer));
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx6 = new DevComponents.DotNetBar.PanelEx();
		this.TextBox_password = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Tnote = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX5 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.Bpass = new DevComponents.DotNetBar.ButtonX();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Tdbname = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.Ldb = new DevComponents.DotNetBar.LabelX();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.Lpass = new DevComponents.DotNetBar.LabelX();
		this.TextBox_user = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Luser = new DevComponents.DotNetBar.LabelX();
		this.Tpath = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.LabelX28 = new DevComponents.DotNetBar.LabelX();
		this.PanelEx5 = new DevComponents.DotNetBar.PanelEx();
		this.Highlighter1 = new DevComponents.DotNetBar.Validator.Highlighter();
		this.ItemPanel1 = new DevComponents.DotNetBar.ItemPanel();
		this.ControlContainerItem1 = new DevComponents.DotNetBar.ControlContainerItem();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.OpenFileDialog1 = new System.Windows.Forms.OpenFileDialog();
		this.PanelEx2.SuspendLayout();
		this.PanelEx6.SuspendLayout();
		this.ItemPanel1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.ButtonX4);
		this.PanelEx2.Controls.Add(this.ButtonX_0);
		this.PanelEx2.Controls.Add(this.ButtonX2);
		this.PanelEx2.Controls.Add(this.PanelEx6);
		this.PanelEx2.Controls.Add(this.PanelEx5);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		System.Drawing.Point location = new System.Drawing.Point(0, 3);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		System.Drawing.Size size = new System.Drawing.Size(820, 260);
		panelEx3.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 0;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Highlighter1.SetHighlightOnFocus(this.ButtonX4, true);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX4;
		location = new System.Drawing.Point(680, 220);
		buttonX.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX4;
		size = new System.Drawing.Size(48, 32);
		buttonX2.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 8;
		this.ButtonX4.Tooltip = "ค\u0e31ดลอก การต\u0e31\u0e49งค\u0e48าส\u0e48งให\u0e49เคร\u0e37\u0e48องล\u0e39ก";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.Flat;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_0;
		location = new System.Drawing.Point(8, 224);
		buttonX_.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX_2 = this.ButtonX_0;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX_2.Margin = margin;
		this.ButtonX_0.Name = "ButtonX10";
		DevComponents.DotNetBar.ButtonX buttonX_3 = this.ButtonX_0;
		size = new System.Drawing.Size(540, 27);
		buttonX_3.Size = size;
		this.ButtonX_0.TabIndex = 7;
		this.ButtonX_0.Text = "ดาวน\u0e4cโหลดไฟล\u0e4cต\u0e34ดต\u0e31\u0e49งระบบเซ\u0e34ฟเวอร\u0e4c (MSSQL หร\u0e37อ MySQL)";
		this.ButtonX_0.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Highlighter1.SetHighlightOnFocus(this.ButtonX2, true);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(734, 220);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(74, 32);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 1;
		this.ButtonX2.Text = "ป\u0e34ด";
		this.PanelEx6.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx6.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx6.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx6.Controls.Add(this.TextBox_password);
		this.PanelEx6.Controls.Add(this.Tnote);
		this.PanelEx6.Controls.Add(this.LabelX5);
		this.PanelEx6.Controls.Add(this.ButtonX5);
		this.PanelEx6.Controls.Add(this.Bpass);
		this.PanelEx6.Controls.Add(this.CheckBox1);
		this.PanelEx6.Controls.Add(this.Tdbname);
		this.PanelEx6.Controls.Add(this.ButtonX3);
		this.PanelEx6.Controls.Add(this.Ldb);
		this.PanelEx6.Controls.Add(this.ComboBox1);
		this.PanelEx6.Controls.Add(this.LabelX3);
		this.PanelEx6.Controls.Add(this.Lpass);
		this.PanelEx6.Controls.Add(this.TextBox_user);
		this.PanelEx6.Controls.Add(this.Luser);
		this.PanelEx6.Controls.Add(this.Tpath);
		this.PanelEx6.Controls.Add(this.LabelX28);
		this.PanelEx6.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx6;
		location = new System.Drawing.Point(8, 39);
		panelEx4.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx6;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx5.Margin = margin;
		this.PanelEx6.Name = "PanelEx6";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx6;
		size = new System.Drawing.Size(800, 176);
		panelEx6.Size = size;
		this.PanelEx6.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx6.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx6.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx6.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx6.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx6.Style.GradientAngle = 90;
		this.PanelEx6.TabIndex = 0;
		this.TextBox_password.BackColor = System.Drawing.Color.LightYellow;
		this.TextBox_password.Border.Class = "TextBoxBorder";
		this.TextBox_password.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.TextBox_password.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.Highlighter1.SetHighlightOnFocus(this.TextBox_password, true);
		DevComponents.DotNetBar.Controls.TextBoxX textBox_password = this.TextBox_password;
		location = new System.Drawing.Point(103, 110);
		textBox_password.Location = location;
		this.TextBox_password.Name = "TextBox_password";
		this.TextBox_password.PasswordChar = '•';
		DevComponents.DotNetBar.Controls.TextBoxX textBox_password2 = this.TextBox_password;
		size = new System.Drawing.Size(327, 27);
		textBox_password2.Size = size;
		this.TextBox_password.TabIndex = 4;
		this.Tnote.BackColor = System.Drawing.Color.LightYellow;
		this.Tnote.Border.Class = "TextBoxBorder";
		this.Tnote.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.Tnote.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.Highlighter1.SetHighlightOnFocus(this.Tnote, true);
		DevComponents.DotNetBar.Controls.TextBoxX tnote = this.Tnote;
		location = new System.Drawing.Point(103, 141);
		tnote.Location = location;
		this.Tnote.Name = "Tnote";
		DevComponents.DotNetBar.Controls.TextBoxX tnote2 = this.Tnote;
		size = new System.Drawing.Size(570, 27);
		tnote2.Size = size;
		this.Tnote.TabIndex = 5;
		this.LabelX5.BackgroundStyle.Class = "";
		this.LabelX5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX labelX = this.LabelX5;
		location = new System.Drawing.Point(10, 139);
		labelX.Location = location;
		this.LabelX5.Name = "LabelX5";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX5;
		size = new System.Drawing.Size(104, 33);
		labelX2.Size = size;
		this.LabelX5.TabIndex = 15;
		this.LabelX5.Text = "หมายเหต\u0e38";
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.FocusCuesEnabled = false;
		this.ButtonX5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Highlighter1.SetHighlightOnFocus(this.ButtonX5, true);
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX5;
		location = new System.Drawing.Point(436, 47);
		buttonX5.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX5;
		size = new System.Drawing.Size(237, 57);
		buttonX6.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 8;
		this.ButtonX5.Text = "ใช\u0e49เคร\u0e37\u0e48องน\u0e35\u0e49เป\u0e47นแม\u0e48\r\n(ต\u0e49องต\u0e34ดต\u0e31\u0e49ง MSSQ ก\u0e48อน)";
		this.ButtonX5.Tooltip = "ใช\u0e49เคร\u0e37\u0e48องน\u0e35\u0e49เป\u0e47นแม\u0e48 (ต\u0e49องต\u0e34ดต\u0e31\u0e49ง MSSQL หร\u0e37อ MYSQL ก\u0e48อน)";
		this.Bpass.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bpass.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bpass.FocusCuesEnabled = false;
		this.Bpass.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Highlighter1.SetHighlightOnFocus(this.Bpass, true);
		this.Bpass.Image = (System.Drawing.Image)resources.GetObject("Bpass.Image");
		DevComponents.DotNetBar.ButtonX bpass = this.Bpass;
		location = new System.Drawing.Point(436, 110);
		bpass.Location = location;
		this.Bpass.Name = "Bpass";
		DevComponents.DotNetBar.ButtonX bpass2 = this.Bpass;
		size = new System.Drawing.Size(237, 27);
		bpass2.Size = size;
		this.Bpass.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.Bpass.TabIndex = 7;
		this.Bpass.Text = "เข\u0e49ารห\u0e31ส Password";
		this.Bpass.Tooltip = "ด\u0e39รห\u0e31สผ\u0e48านท\u0e35\u0e48เข\u0e49ารห\u0e31สแล\u0e49ว\r\n(สามารถนำไปใส\u0e48เคร\u0e37\u0e48องล\u0e39กค\u0e49าได\u0e49เลย)";
		this.Bpass.Visible = false;
		this.CheckBox1.AutoSize = true;
		this.CheckBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(621, 147);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(116, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 11;
		this.CheckBox1.Text = "แสดง Password";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.CheckBox1.Visible = false;
		this.Tdbname.BackColor = System.Drawing.Color.LightYellow;
		this.Tdbname.Border.Class = "TextBoxBorder";
		this.Tdbname.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.Tdbname.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.Highlighter1.SetHighlightOnFocus(this.Tdbname, true);
		DevComponents.DotNetBar.Controls.TextBoxX tdbname = this.Tdbname;
		location = new System.Drawing.Point(330, 16);
		tdbname.Location = location;
		this.Tdbname.Name = "Tdbname";
		DevComponents.DotNetBar.Controls.TextBoxX tdbname2 = this.Tdbname;
		size = new System.Drawing.Size(343, 27);
		tdbname2.Size = size;
		this.Tdbname.TabIndex = 1;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Highlighter1.SetHighlightOnFocus(this.ButtonX3, true);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		this.ButtonX3.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX3;
		location = new System.Drawing.Point(682, 16);
		buttonX7.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX3;
		size = new System.Drawing.Size(104, 121);
		buttonX8.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 9;
		this.ButtonX3.Text = "บ\u0e31นท\u0e36ก";
		this.Ldb.BackgroundStyle.Class = "";
		this.Ldb.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX ldb = this.Ldb;
		location = new System.Drawing.Point(239, 13);
		ldb.Location = location;
		this.Ldb.Name = "Ldb";
		DevComponents.DotNetBar.LabelX ldb2 = this.Ldb;
		size = new System.Drawing.Size(259, 31);
		ldb2.Size = size;
		this.Ldb.TabIndex = 10;
		this.Ldb.Text = "ช\u0e37\u0e48อฐานข\u0e49อม\u0e39ล";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ComboBox1.FormattingEnabled = true;
		this.Highlighter1.SetHighlightOnFocus(this.ComboBox1, true);
		this.ComboBox1.Items.AddRange(new object[1] { "MSSQL" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(103, 17);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(124, 27);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 0;
		this.LabelX3.BackgroundStyle.Class = "";
		this.LabelX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX3;
		location = new System.Drawing.Point(10, 13);
		labelX3.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX3;
		size = new System.Drawing.Size(104, 31);
		labelX4.Size = size;
		this.LabelX3.TabIndex = 7;
		this.LabelX3.Text = "ประเภท";
		this.Lpass.BackgroundStyle.Class = "";
		this.Lpass.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX lpass = this.Lpass;
		location = new System.Drawing.Point(10, 109);
		lpass.Location = location;
		this.Lpass.Name = "Lpass";
		DevComponents.DotNetBar.LabelX lpass2 = this.Lpass;
		size = new System.Drawing.Size(104, 33);
		lpass2.Size = size;
		this.Lpass.TabIndex = 5;
		this.Lpass.Text = "Password";
		this.TextBox_user.BackColor = System.Drawing.Color.LightYellow;
		this.TextBox_user.Border.Class = "TextBoxBorder";
		this.TextBox_user.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.TextBox_user.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.Highlighter1.SetHighlightOnFocus(this.TextBox_user, true);
		DevComponents.DotNetBar.Controls.TextBoxX textBox_user = this.TextBox_user;
		location = new System.Drawing.Point(103, 78);
		textBox_user.Location = location;
		this.TextBox_user.Name = "TextBox_user";
		DevComponents.DotNetBar.Controls.TextBoxX textBox_user2 = this.TextBox_user;
		size = new System.Drawing.Size(327, 27);
		textBox_user2.Size = size;
		this.TextBox_user.TabIndex = 3;
		this.Luser.BackgroundStyle.Class = "";
		this.Luser.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX luser = this.Luser;
		location = new System.Drawing.Point(10, 76);
		luser.Location = location;
		this.Luser.Name = "Luser";
		DevComponents.DotNetBar.LabelX luser2 = this.Luser;
		size = new System.Drawing.Size(104, 33);
		luser2.Size = size;
		this.Luser.TabIndex = 3;
		this.Luser.Text = "Username";
		this.Tpath.BackColor = System.Drawing.Color.LightYellow;
		this.Tpath.Border.Class = "TextBoxBorder";
		this.Tpath.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.Tpath.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.Highlighter1.SetHighlightOnFocus(this.Tpath, true);
		DevComponents.DotNetBar.Controls.TextBoxX tpath = this.Tpath;
		location = new System.Drawing.Point(103, 47);
		tpath.Location = location;
		this.Tpath.Name = "Tpath";
		DevComponents.DotNetBar.Controls.TextBoxX tpath2 = this.Tpath;
		size = new System.Drawing.Size(327, 27);
		tpath2.Size = size;
		this.Tpath.TabIndex = 2;
		this.LabelX28.BackgroundStyle.Class = "";
		this.LabelX28.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX28;
		location = new System.Drawing.Point(10, 45);
		labelX5.Location = location;
		this.LabelX28.Name = "LabelX28";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX28;
		size = new System.Drawing.Size(104, 33);
		labelX6.Size = size;
		this.LabelX28.TabIndex = 0;
		this.LabelX28.Text = "ท\u0e35\u0e48อย\u0e39\u0e48 Server";
		this.PanelEx5.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx5.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx5.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx5.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx7 = this.PanelEx5;
		location = new System.Drawing.Point(8, 8);
		panelEx7.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx8 = this.PanelEx5;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx8.Margin = margin;
		this.PanelEx5.Name = "PanelEx5";
		DevComponents.DotNetBar.PanelEx panelEx9 = this.PanelEx5;
		size = new System.Drawing.Size(800, 32);
		panelEx9.Size = size;
		this.PanelEx5.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx5.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx5.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx5.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx5.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx5.Style.GradientAngle = 90;
		this.PanelEx5.TabIndex = 0;
		this.PanelEx5.Text = "ต\u0e31\u0e49งค\u0e48าเซ\u0e34ฟเวอร\u0e4c";
		this.Highlighter1.ContainerControl = this;
		this.Highlighter1.FocusHighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Orange;
		this.ItemPanel1.BackgroundStyle.BackColor2SchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.ItemPanel1.BackgroundStyle.BackColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.ItemPanel1.BackgroundStyle.Class = "ItemPanel";
		this.ItemPanel1.ContainerControlProcessDialogKey = true;
		this.ItemPanel1.Controls.Add(this.PanelEx2);
		this.ItemPanel1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.ItemPanel1.HorizontalItemAlignment = DevComponents.DotNetBar.eHorizontalItemsAlignment.Center;
		this.ItemPanel1.Items.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ControlContainerItem1 });
		DevComponents.DotNetBar.ItemPanel itemPanel = this.ItemPanel1;
		location = new System.Drawing.Point(0, 0);
		itemPanel.Location = location;
		this.ItemPanel1.Name = "ItemPanel1";
		this.ItemPanel1.ResizeItemsToFit = false;
		DevComponents.DotNetBar.ItemPanel itemPanel2 = this.ItemPanel1;
		size = new System.Drawing.Size(820, 263);
		itemPanel2.Size = size;
		this.ItemPanel1.Style = DevComponents.DotNetBar.eDotNetBarStyle.Windows7;
		this.ItemPanel1.TabIndex = 2;
		this.ItemPanel1.Text = "ItemPanel1";
		this.ControlContainerItem1.AllowItemResize = false;
		this.ControlContainerItem1.Control = this.PanelEx2;
		this.ControlContainerItem1.MenuVisibility = DevComponents.DotNetBar.eMenuVisibility.VisibleAlways;
		this.ControlContainerItem1.Name = "ControlContainerItem1";
		this.Timer1.Interval = 300;
		this.OpenFileDialog1.FileName = "kphotel.accdb";
		this.OpenFileDialog1.Filter = "Database files (*.accdb)|*.accdb";
		this.OpenFileDialog1.RestoreDirectory = true;
		this.OpenFileDialog1.Title = "กร\u0e38ณาเล\u0e37อกไฟล\u0e4cในเคร\u0e37\u0e48อง Server";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(820, 263);
		this.ClientSize = size;
		this.Controls.Add(this.ItemPanel1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FrmAddEditServer";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ต\u0e31\u0e49งค\u0e48า";
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx6.ResumeLayout(false);
		this.PanelEx6.PerformLayout();
		this.ItemPanel1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	protected override bool ProcessCmdKey(ref Message msg, Keys keyData)
	{
		if (keyData == Keys.Escape)
		{
			Close();
		}
		bool result = default(bool);
		return result;
	}

	public void clear()
	{
		ComboBox1.SelectedIndex = 0;
		Tdbname.Text = "";
		Tpath.Text = "";
		TextBox_user.Text = "";
		TextBox_password.Text = "";
		Tnote.Text = "";
	}

	private void FrmAddEditServer_FormClosing(object sender, FormClosingEventArgs e)
	{
	}

	private void FrmSettingsServer_Load(object sender, EventArgs e)
	{
		ISOK = false;
		try
		{
			if (File.Exists("C:\\PiPE.txt"))
			{
				Bpass.Visible = true;
			}
			else
			{
				Bpass.Visible = false;
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public static void RunCommandCom(string command, string arguments, bool permanent)
	{
		Process process = new Process();
		ProcessStartInfo processStartInfo = new ProcessStartInfo();
		processStartInfo.Arguments = " " + (permanent ? "/K" : "/C") + " " + command + " " + arguments;
		processStartInfo.FileName = "cmd.exe";
		process.StartInfo = processStartInfo;
		process.Start();
	}

	public string CreateShare(string Desc, string sharename, string path, string server)
	{
		string result;
		try
		{
			string scope = "\\\\" + server + "\\root\\cimv2";
			ManagementClass managementClass = new ManagementClass(scope, "Win32_Share", null);
			ManagementBaseObject methodParameters = managementClass.GetMethodParameters("Create");
			methodParameters["Access"] = null;
			methodParameters["Description"] = Desc;
			methodParameters["Name"] = sharename;
			methodParameters["Path"] = path;
			methodParameters["Type"] = 0;
			ManagementBaseObject managementBaseObject = managementClass.InvokeMethod("Create", methodParameters, null);
			result = managementBaseObject.Properties["ReturnValue"].Value.ToString();
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show(ex2.ToString());
			result = Conversions.ToString(1);
			ProjectData.ClearProjectError();
		}
		return result;
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void PanelEx5_Click(object sender, EventArgs e)
	{
	}

	private void Button5_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonItem1_Click(object sender, EventArgs e)
	{
		ComboBox1.SelectedIndex = 0;
		Tpath.Text = "kphotel.accdb";
		Interaction.Shell("net share LOTTO /DELETE");
		ButtonX3_Click(null, null);
	}

	private void ButtonItem2_Click(object sender, EventArgs e)
	{
		if (OpenFileDialog1.ShowDialog() == DialogResult.OK)
		{
			ComboBox1.SelectedIndex = 0;
			Tpath.Text = OpenFileDialog1.FileName;
			Interaction.Shell("net share LOTTO /DELETE");
			ButtonX3_Click(null, null);
			MessageBox.Show("ระบบจะป\u0e34ดโปรแกรมแล\u0e49วเป\u0e34ดใหม\u0e48");
			MyProject.Forms.frmMain1.Close();
		}
	}

	private void ButtonItem3_Click(object sender, EventArgs e)
	{
		Module1.Path_Program.Substring(0, checked(Module1.Path_Program.Length - 1));
		ComboBox1.SelectedIndex = 0;
		string text = "C:\\";
		if (Directory.Exists("D:\\"))
		{
			text = "D:\\";
		}
		if (Operators.CompareString(text, "C:\\", TextCompare: false) == 0 && Directory.Exists("E:\\"))
		{
			text = "E:\\";
		}
		if (Operators.CompareString(text, "C:\\", TextCompare: false) == 0 && Directory.Exists("F:\\"))
		{
			text = "F:\\";
		}
		string text2 = Module1.Path_Program + "kphotel.accdb";
		string text3 = text + "data\\kphotel.accdb";
		if (!Directory.Exists(text + "data"))
		{
			DirectorySecurity directorySecurity = new DirectorySecurity();
			directorySecurity.AddAccessRule(new FileSystemAccessRule("everyone", FileSystemRights.Modify, InheritanceFlags.ContainerInherit | InheritanceFlags.ObjectInherit, PropagationFlags.None, AccessControlType.Allow));
			Directory.CreateDirectory(text + "data", directorySecurity);
		}
		if (File.Exists(text2) && !File.Exists(text3))
		{
			File.Copy(text2, text3);
		}
		string hostName = Dns.GetHostName();
		Interaction.Shell("Net Share data=\"" + text + "data\" /GRANT:everyone,FULL");
		Tpath.Text = "\\\\" + hostName + "\\data\\kphotel.accdb";
		MessageBox.Show("ให\u0e49ต\u0e31\u0e49งค\u0e48าฐานข\u0e49อม\u0e39ลเคร\u0e37\u0e48องล\u0e39ก เป\u0e47น \\\\" + hostName + "\\data\\kphotel.accdb", "", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
		ButtonX3_Click(null, null);
	}

	private void ButtonItem5_Click(object sender, EventArgs e)
	{
		TopMost = false;
		string text = Interaction.InputBox("กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อฐานข\u0e49อม\u0e39ล เช\u0e48น THAI , LAOS , HANOI", "กร\u0e38ณาใส\u0e48ช\u0e37\u0e48อฐานข\u0e49อม\u0e39ล เช\u0e48น THAI , LAOS , HANOI โปรแกรม จะ Copy ฐานข\u0e49อม\u0e39ลเด\u0e34มจากไฟล\u0e4c kphotel.accdb มาให\u0e49", "THAI");
		if (Operators.CompareString(text, "", TextCompare: false) != 0)
		{
			try
			{
				string sourceFileName = Module1.Path_Program + "kphotel.accdb";
				string text2 = Module1.Path_Program + text + ".accdb";
				if (!File.Exists(text2))
				{
					File.Copy(sourceFileName, text2);
				}
				ComboBox1.SelectedIndex = 0;
				Tpath.Text = text + ".accdb";
			}
			catch (Exception ex)
			{
				ProjectData.SetProjectError(ex);
				Exception ex2 = ex;
				MessageBox.Show(ex2.Message, "", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				ProjectData.ClearProjectError();
			}
		}
		TopMost = true;
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(ComboBox1.Text, "ACCESS", TextCompare: false) == 0 && Tpath.Text.ToLower().IndexOf(".accdb") == -1)
		{
			MessageBox.Show("กร\u0e38ณากรอก ช\u0e37\u0e48อไฟล\u0e4cฐานข\u0e49อม\u0e39ลเช\u0e48น kphotel.accdb", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			Tpath.Focus();
			return;
		}
		if (TextBox_password.Text.IndexOf(":EG:") == -1)
		{
			object right = FormEN_DE.Encrypt1(TextBox_password.Text, "ruj5de4");
			TextBox_password.Text = Conversions.ToString(Operators.ConcatenateObject(":EG:", right));
		}
		if (Operators.CompareString(ComboBox1.Text, "MSSQL", TextCompare: false) == 0)
		{
			if (Operators.CompareString(Tdbname.Text, "", TextCompare: false) == 0)
			{
				MessageBox.Show("กร\u0e38ณากรอก ช\u0e37\u0e48อฐานข\u0e49อม\u0e39ลเช\u0e48น HOTEL", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (ComboBox1.Text.IndexOf("CloudServer") == -1 && Operators.CompareString(TextBox_user.Text, "", TextCompare: false) == 0)
			{
				MessageBox.Show("กร\u0e38ณากรอก Username ของฐานข\u0e49อม\u0e39ล", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
			if (Operators.CompareString(TextBox_password.Text, "", TextCompare: false) == 0)
			{
				MessageBox.Show("กร\u0e38ณากรอก Password ของฐานข\u0e49อม\u0e39ล", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				return;
			}
		}
		else
		{
			Tdbname.Text = "";
			TextBox_user.Text = "";
			TextBox_password.Text = "";
		}
		ISOK = true;
		Close();
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
		if (CheckBox1.Checked)
		{
			TextBox_password.PasswordChar = '\0';
		}
		else
		{
			TextBox_password.PasswordChar = '•';
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		TopMost = false;
		if (TextBox_password.Text.IndexOf(":EG:") != -1)
		{
			MessageBox.Show("เป\u0e47น Password ท\u0e35\u0e48เข\u0e49ารห\u0e31สไปแล\u0e49ว", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
		}
		else
		{
			object right = FormEN_DE.Encrypt1(TextBox_password.Text, "ruj5de4");
			TextBox_password.Text = Conversions.ToString(Operators.ConcatenateObject(":EG:", right));
		}
		TopMost = true;
	}

	private void TextBox_password_TextChanged(object sender, EventArgs e)
	{
		try
		{
			if (Operators.CompareString(TextBox_password.Text, "1234", TextCompare: false) == 0)
			{
				TextBox_password.PasswordChar = '\0';
			}
			else if (Operators.CompareString(TextBox_password.Text, "12345678", TextCompare: false) == 0)
			{
				TextBox_password.PasswordChar = '\0';
			}
			else if (Operators.CompareString(TextBox_password.Text, "12340000", TextCompare: false) == 0)
			{
				TextBox_password.PasswordChar = '\0';
			}
			else if (TextBox_password.Text.IndexOf(":") == 0)
			{
				TextBox_password.PasswordChar = '\0';
			}
			else
			{
				TextBox_password.PasswordChar = '•';
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(ComboBox1.Text, "MSSQL", TextCompare: false) == 0)
		{
			Tpath.Text = Dns.GetHostName();
			if (Operators.CompareString(ComboBox1.Text, "MSSQL", TextCompare: false) == 0)
			{
				if (Operators.CompareString(TextBox_user.Text, "", TextCompare: false) == 0)
				{
					TextBox_user.Text = "sa";
				}
				if (Operators.CompareString(TextBox_password.Text, "", TextCompare: false) == 0)
				{
					TextBox_password.Text = "12345678";
				}
			}
			return;
		}
		Tpath.Text = Dns.GetHostName();
		if (Operators.CompareString(ComboBox1.Text, "MYSQL", TextCompare: false) == 0)
		{
			if (Operators.CompareString(TextBox_user.Text, "", TextCompare: false) == 0)
			{
				TextBox_user.Text = "root";
			}
			if (Operators.CompareString(TextBox_password.Text, "", TextCompare: false) == 0)
			{
				TextBox_password.Text = "12345678";
			}
		}
	}

	private void ButtonX10_Click(object sender, EventArgs e)
	{
		TopMost = false;
		TopMost = true;
	}

	private void TextBox_user_LostFocus(object sender, EventArgs e)
	{
		if (Operators.CompareString(ComboBox1.Text, "CloudServer", TextCompare: false) == 0 && Operators.CompareString(Tdbname.Text, "", TextCompare: false) == 0)
		{
			Tdbname.Text = TextBox_user.Text;
		}
	}

	private void TextBox_user_TextChanged(object sender, EventArgs e)
	{
	}

	private void ButtonX4_Click_1(object sender, EventArgs e)
	{
		object obj = "การต\u0e31\u0e49งค\u0e48าเซ\u0e34ฟเวอร\u0e4c\r\nประเภท = " + ComboBox1.Text + "\r\n";
		if (Tdbname.Visible)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat("ช\u0e37\u0e48อฐานข\u0e49อม\u0e39ล = " + Tdbname.Text, "\r\n"));
		}
		if (Tpath.Visible)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat("ท\u0e35\u0e48อย\u0e39\u0e48 Server = " + Tpath.Text, "\r\n"));
		}
		if (TextBox_user.Visible)
		{
			obj = Operators.ConcatenateObject(obj, string.Concat("Username = " + TextBox_user.Text, "\r\n"));
		}
		if (TextBox_password.Visible)
		{
			if (TextBox_password.Text.IndexOf(":EG:") != -1)
			{
				obj = Operators.ConcatenateObject(obj, string.Concat("Password = " + TextBox_password.Text, "\r\n"));
			}
			else if ((Operators.CompareString(TextBox_password.Text, "1234", TextCompare: false) == 0) | (Operators.CompareString(TextBox_password.Text, "12340000", TextCompare: false) == 0) | (Operators.CompareString(TextBox_password.Text, "12345678", TextCompare: false) == 0))
			{
				obj = Operators.ConcatenateObject(obj, string.Concat("Password = " + TextBox_password.Text, "\r\n"));
			}
			else
			{
				object right = FormEN_DE.Encrypt1(TextBox_password.Text, "ruj5de4");
				obj = Operators.ConcatenateObject(obj, Operators.ConcatenateObject(Operators.ConcatenateObject("Password = :EG:", right), "\r\n"));
			}
		}
		Clipboard.SetText(Conversions.ToString(obj));
		MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(obj, "\r\n"), "\r\n"), "ค\u0e31ดลอกเร\u0e35ยบร\u0e49อย")));
	}
}
