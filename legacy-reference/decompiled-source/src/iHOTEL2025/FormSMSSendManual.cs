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
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSMSSendManual : Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("RichTextBox1")]
	private RichTextBox _RichTextBox1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("WebBrowser1")]
	private WebBrowser _WebBrowser1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBarX _ProgressBarX1;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("RichTextBox2")]
	private RichTextBox _RichTextBox2;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Button5")]
	private Button _Button5;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("Button6")]
	private Button _Button6;

	[AccessedThroughProperty("Button7")]
	private Button _Button7;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	public bool ISOK;

	private int Send_NUm;

	private int Send_OK;

	private bool bool_0;

	private string SUser;

	private string Spass;

	private string Lastid;

	private string SMSNORMAL;

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

	internal virtual RichTextBox RichTextBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _RichTextBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = RichTextBox1_TextChanged;
			if (_RichTextBox1 != null)
			{
				_RichTextBox1.TextChanged -= value2;
			}
			_RichTextBox1 = value;
			if (_RichTextBox1 != null)
			{
				_RichTextBox1.TextChanged += value2;
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

	internal virtual WebBrowser WebBrowser1
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowser1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser1_DocumentCompleted;
			if (_WebBrowser1 != null)
			{
				_WebBrowser1.DocumentCompleted -= value2;
			}
			_WebBrowser1 = value;
			if (_WebBrowser1 != null)
			{
				_WebBrowser1.DocumentCompleted += value2;
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
			EventHandler value2 = PanelEx1_Click;
			if (_PanelEx1 != null)
			{
				_PanelEx1.Click -= value2;
			}
			_PanelEx1 = value;
			if (_PanelEx1 != null)
			{
				_PanelEx1.Click += value2;
			}
		}
	}

	internal virtual ProgressBarX ProgressBarX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ProgressBarX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ProgressBarX1 = value;
		}
	}

	internal virtual LabelX LabelX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX1 = value;
		}
	}

	internal virtual RichTextBox RichTextBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RichTextBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RichTextBox2 = value;
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

	[DebuggerNonUserCode]
	static FormSMSSendManual()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSMSSendManual()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormSMStext_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISOK = false;
		Send_NUm = 0;
		Send_OK = 0;
		bool_0 = false;
		SUser = "";
		Spass = "";
		Lastid = "";
		SMSNORMAL = "";
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
		this.Label1 = new System.Windows.Forms.Label();
		this.RichTextBox1 = new System.Windows.Forms.RichTextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.Label6 = new System.Windows.Forms.Label();
		this.WebBrowser1 = new System.Windows.Forms.WebBrowser();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.ProgressBarX1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.RichTextBox2 = new System.Windows.Forms.RichTextBox();
		this.Button3 = new System.Windows.Forms.Button();
		this.Label7 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Button4 = new System.Windows.Forms.Button();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.Button5 = new System.Windows.Forms.Button();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Button6 = new System.Windows.Forms.Button();
		this.Button7 = new System.Windows.Forms.Button();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label10 = new System.Windows.Forms.Label();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(22, 204);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(173, 19);
		label3.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "กร\u0e38ณากรอกข\u0e49อความ SMS";
		this.RichTextBox1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		System.Windows.Forms.RichTextBox richTextBox = this.RichTextBox1;
		location = new System.Drawing.Point(23, 270);
		richTextBox.Location = location;
		this.RichTextBox1.MaxLength = 250;
		this.RichTextBox1.Name = "RichTextBox1";
		System.Windows.Forms.RichTextBox richTextBox2 = this.RichTextBox1;
		size = new System.Drawing.Size(543, 193);
		richTextBox2.Size = size;
		this.RichTextBox1.TabIndex = 2;
		this.RichTextBox1.Text = "";
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(389, 469);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(96, 31);
		button2.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "ส\u0e48งข\u0e49อความ";
		this.Button1.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(491, 469);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(75, 31);
		button4.Size = size;
		this.Button2.TabIndex = 4;
		this.Button2.Text = "ยกเล\u0e34ก";
		this.Button2.UseVisualStyleBackColor = true;
		this.Label6.AutoSize = true;
		this.Label6.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label6.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label4 = this.Label6;
		location = new System.Drawing.Point(23, 524);
		label4.Location = location;
		System.Windows.Forms.Label label5 = this.Label6;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label5.Margin = margin;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label6 = this.Label6;
		size = new System.Drawing.Size(194, 16);
		label6.Size = size;
		this.Label6.TabIndex = 6;
		this.Label6.Text = "* 63 ต\u0e31วอ\u0e31กษร = 1 ข\u0e49อความ/1 ผ\u0e39\u0e49ร\u0e31บ";
		System.Windows.Forms.WebBrowser webBrowser = this.WebBrowser1;
		location = new System.Drawing.Point(463, 65);
		webBrowser.Location = location;
		System.Windows.Forms.WebBrowser webBrowser2 = this.WebBrowser1;
		size = new System.Drawing.Size(20, 20);
		webBrowser2.MinimumSize = size;
		this.WebBrowser1.Name = "WebBrowser1";
		System.Windows.Forms.WebBrowser webBrowser3 = this.WebBrowser1;
		size = new System.Drawing.Size(41, 33);
		webBrowser3.Size = size;
		this.WebBrowser1.TabIndex = 7;
		this.WebBrowser1.Visible = false;
		this.Timer1.Interval = 500;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Controls.Add(this.ProgressBarX1);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(6, 4);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(573, 559);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 8;
		this.PanelEx1.Visible = false;
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX1;
		location = new System.Drawing.Point(33, 265);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		size = new System.Drawing.Size(432, 23);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 1;
		this.LabelX1.Text = "LabelX1";
		this.ProgressBarX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX = this.ProgressBarX1;
		location = new System.Drawing.Point(33, 236);
		progressBarX.Location = location;
		this.ProgressBarX1.Name = "ProgressBarX1";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX2 = this.ProgressBarX1;
		size = new System.Drawing.Size(500, 23);
		progressBarX2.Size = size;
		this.ProgressBarX1.TabIndex = 0;
		this.ProgressBarX1.Text = "ProgressBarX1";
		this.RichTextBox2.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		System.Windows.Forms.RichTextBox richTextBox3 = this.RichTextBox2;
		location = new System.Drawing.Point(456, 538);
		richTextBox3.Location = location;
		this.RichTextBox2.Name = "RichTextBox2";
		System.Windows.Forms.RichTextBox richTextBox4 = this.RichTextBox2;
		size = new System.Drawing.Size(62, 25);
		richTextBox4.Size = size;
		this.RichTextBox2.TabIndex = 9;
		this.RichTextBox2.Text = "";
		this.RichTextBox2.Visible = false;
		System.Windows.Forms.Button button5 = this.Button3;
		location = new System.Drawing.Point(22, 469);
		button5.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button6 = this.Button3;
		size = new System.Drawing.Size(139, 31);
		button6.Size = size;
		this.Button3.TabIndex = 10;
		this.Button3.Text = "เพ\u0e34\u0e48มเข\u0e49า Favorites";
		this.Button3.UseVisualStyleBackColor = true;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label7;
		location = new System.Drawing.Point(22, 239);
		label7.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label8 = this.Label7;
		size = new System.Drawing.Size(125, 19);
		label8.Size = size;
		this.Label7.TabIndex = 11;
		this.Label7.Text = "รายการ Favorites";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.DropDownWidth = 800;
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(150, 235);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(354, 27);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 12;
		System.Windows.Forms.Button button7 = this.Button4;
		location = new System.Drawing.Point(505, 233);
		button7.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button8 = this.Button4;
		size = new System.Drawing.Size(61, 31);
		button8.Size = size;
		this.Button4.TabIndex = 13;
		this.Button4.Text = "ลบ";
		this.Button4.UseVisualStyleBackColor = true;
		this.Label8.AutoSize = true;
		this.Label8.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label8.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label9 = this.Label8;
		location = new System.Drawing.Point(23, 543);
		label9.Location = location;
		System.Windows.Forms.Label label10 = this.Label8;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label10.Margin = margin;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label11 = this.Label8;
		size = new System.Drawing.Size(233, 16);
		label11.Size = size;
		this.Label8.TabIndex = 14;
		this.Label8.Text = "* ตรวจสอบ Credit คงเหล\u0e37อได\u0e49ท\u0e35\u0e48หน\u0e49า ต\u0e31\u0e49งค\u0e48า";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label12 = this.Label2;
		location = new System.Drawing.Point(22, 16);
		label12.Location = location;
		System.Windows.Forms.Label label13 = this.Label2;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label13.Margin = margin;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label14 = this.Label2;
		size = new System.Drawing.Size(111, 19);
		label14.Size = size;
		this.Label2.TabIndex = 15;
		this.Label2.Text = "ผ\u0e39\u0e49ร\u0e31บ (เบอร\u0e4cโทร)";
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[3] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3 });
		this.ListView1.FullRowSelect = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(22, 45);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(544, 152);
		listView2.Size = size;
		this.ListView1.TabIndex = 16;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader2.Width = 300;
		this.ColumnHeader3.Text = "เบอร\u0e4cโทร";
		this.ColumnHeader3.Width = 120;
		System.Windows.Forms.Button button9 = this.Button5;
		location = new System.Drawing.Point(240, 11);
		button9.Location = location;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button10 = this.Button5;
		size = new System.Drawing.Size(46, 31);
		button10.Size = size;
		this.Button5.TabIndex = 17;
		this.Button5.Text = "เพ\u0e34\u0e48ม";
		this.Button5.UseVisualStyleBackColor = true;
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		location = new System.Drawing.Point(136, 13);
		textBox.Location = location;
		this.TextBox1.MaxLength = 10;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		size = new System.Drawing.Size(100, 27);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 18;
		System.Windows.Forms.Button button11 = this.Button6;
		location = new System.Drawing.Point(288, 11);
		button11.Location = location;
		this.Button6.Name = "Button6";
		System.Windows.Forms.Button button12 = this.Button6;
		size = new System.Drawing.Size(46, 31);
		button12.Size = size;
		this.Button6.TabIndex = 19;
		this.Button6.Text = "ลบ";
		this.Button6.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button13 = this.Button7;
		location = new System.Drawing.Point(386, 11);
		button13.Location = location;
		this.Button7.Name = "Button7";
		System.Windows.Forms.Button button14 = this.Button7;
		size = new System.Drawing.Size(180, 31);
		button14.Size = size;
		this.Button7.TabIndex = 19;
		this.Button7.Text = "เล\u0e37อกจากทะเบ\u0e35ยนล\u0e39กค\u0e49า";
		this.Button7.UseVisualStyleBackColor = true;
		this.Label3.AutoSize = true;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label3.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label15 = this.Label3;
		location = new System.Drawing.Point(23, 504);
		label15.Location = location;
		System.Windows.Forms.Label label16 = this.Label3;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label16.Margin = margin;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label17 = this.Label3;
		size = new System.Drawing.Size(225, 16);
		label17.Size = size;
		this.Label3.TabIndex = 20;
		this.Label3.Text = "* พ\u0e34มพ\u0e4cโค\u0e4aด *CUST* เพ\u0e34\u0e48อแสดงช\u0e37\u0e48อล\u0e39กค\u0e49า ";
		this.Label4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label4.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label18 = this.Label4;
		location = new System.Drawing.Point(336, 504);
		label18.Location = location;
		System.Windows.Forms.Label label19 = this.Label4;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label19.Margin = margin;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label20 = this.Label4;
		size = new System.Drawing.Size(233, 55);
		label20.Size = size;
		this.Label4.TabIndex = 21;
		this.Label4.Text = "** กร\u0e38ณาตรวจสอบเบอร\u0e4cให\u0e49ถ\u0e39กต\u0e49องถ\u0e49าส\u0e48งไปแล\u0e49วจะไม\u0e48สามารรถค\u0e37น credit ได\u0e49 **";
		this.Label4.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label10.AutoSize = true;
		this.Label10.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label21 = this.Label10;
		location = new System.Drawing.Point(221, 474);
		label21.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label22 = this.Label10;
		size = new System.Drawing.Size(129, 19);
		label22.Size = size;
		this.Label10.TabIndex = 24;
		this.Label10.Text = "จำนวน 0 ต\u0e31วอ\u0e31กษร";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(9f, 19f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(586, 568);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.Controls.Add(this.Label10);
		this.Controls.Add(this.Label4);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Button7);
		this.Controls.Add(this.Button6);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.Button5);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label8);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.ComboBox1);
		this.Controls.Add(this.Label7);
		this.Controls.Add(this.Button3);
		this.Controls.Add(this.RichTextBox2);
		this.Controls.Add(this.WebBrowser1);
		this.Controls.Add(this.Label6);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.RichTextBox1);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(4);
		this.Margin = margin;
		this.Name = "FormSMSSendManual";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ส\u0e48ง SMS ท\u0e31\u0e48วไป";
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void FormSMStext_Load(object sender, EventArgs e)
	{
		method_0();
		bool_0 = true;
		SUser = MyProject.Forms.FrmSettingsSMS.Tuser.Text;
		Spass = MyProject.Forms.FrmSettingsSMS.Tpass.Text;
		ISOK = false;
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการส\u0e48งข\u0e49อความหร\u0e37อไม\u0e48", "ส\u0e48งข\u0e49อความ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			LabelX1.Text = "กำล\u0e31งดำเน\u0e34นการ....";
			RichTextBox2.Text = "";
			Send_NUm = 0;
			PanelEx1.Visible = true;
			ProgressBarX1.Value = 0;
			ProgressBarX1.Maximum = ListView1.Items.Count;
			Send_OK = 0;
			Timer1.Enabled = true;
		}
	}

	private void RichTextBox1_TextChanged(object sender, EventArgs e)
	{
		Label10.Text = "จำนวน " + Conversions.ToString(RichTextBox1.Text.Length) + " ต\u0e31วอ\u0e31กษร";
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Cursor = Cursors.WaitCursor;
		checked
		{
			if ((Send_NUm <= ListView1.Items.Count - 1) & bool_0)
			{
				LabelX1.Text = "กำล\u0e31งส\u0e48งข\u0e49อความจาก " + Conversions.ToString(Send_NUm + 1) + " ถ\u0e36ง " + Conversions.ToString(ListView1.Items.Count);
				WebBrowser1.DocumentText = "";
				string text = (SMSNORMAL = RichTextBox1.Text.Replace("*CUST*", ListView1.Items[Send_NUm].SubItems[1].Text));
				bool_0 = false;
				WebBrowser1.Url = new Uri("http://www.kpsystem.co.th/sms/sms.php?mode=send&u=" + SUser + "&p=" + Spass + "&ds=" + SMSNORMAL + "&n=" + ListView1.Items[Send_NUm].SubItems[2].Text.ToString().Replace("-", "").ToString()
					.Replace(" ", ""), UriKind.Absolute);
				RichTextBox richTextBox = RichTextBox2;
				richTextBox.Text = richTextBox.Text + text + "\r\n";
				Send_NUm++;
				ProgressBarX1.Value++;
				Application.DoEvents();
			}
			else if ((Send_NUm == ListView1.Items.Count) & bool_0)
			{
				Timer1.Enabled = false;
				PanelEx1.Visible = false;
				WebBrowser1.Url = new Uri("http://www.kpsystem.co.th/sms/smsok.php", UriKind.Absolute);
				Cursor = Cursors.Default;
				MessageBox.Show("ส\u0e48งเสร\u0e47จเร\u0e35ยบร\u0e49อย สำเร\u0e47จ " + Conversions.ToString(Send_OK) + " จาก " + Conversions.ToString(ListView1.Items.Count) + " รายการ");
				ISOK = true;
			}
		}
	}

	private void WebBrowser1_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		checked
		{
			if (Operators.CompareString(WebBrowser1.DocumentText, "SMSOK", TextCompare: false) == 0)
			{
				Send_OK++;
			}
			Cursor = Cursors.Default;
			bool_0 = true;
		}
	}

	private void PanelEx1_Click(object sender, EventArgs e)
	{
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(RichTextBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาพ\u0e34มพ\u0e4cข\u0e49อความก\u0e48อน");
			return;
		}
		string text = Conversions.ToString(Module1.get_id("TB_SMS_FAVORITES_2", "id"));
		Module1.connect("INSERT INTO TB_SMS_FAVORITES_2 VALUES(" + text + ",'" + RichTextBox1.Text + "')");
		method_0();
	}

	public void method_0()
	{
		DataSet dataSet = Module1.connect("select * from TB_SMS_FAVORITES_2 order by fav_name");
		ComboBox1.Items.Clear();
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
					ComboBox1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["fav_name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		RichTextBox1.Text = ComboBox1.Text;
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		Module1.connect("delete from TB_SMS_FAVORITES_2 where fav_name='" + ComboBox1.Text + "'");
		method_0();
	}

	private void Button5_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกเบอร\u0e4cโทรศ\u0e31พท\u0e4c");
			return;
		}
		if (TextBox1.Text.Length != 10)
		{
			MessageBox.Show("กร\u0e38ณากรอกเบอร\u0e4cโทรศ\u0e31พท\u0e4c 10 ต\u0e31ว");
			return;
		}
		if (!Versioned.IsNumeric(TextBox1.Text))
		{
			MessageBox.Show("กร\u0e38ณากรอกเบอร\u0e4cโทรศ\u0e31พท\u0e4cเป\u0e47นต\u0e31วเลข");
			return;
		}
		ListView listView = ListView1;
		int count = listView.Items.Count;
		listView.Items.Add(Conversions.ToString(checked(count + 1)));
		listView.Items[count].SubItems.Add("");
		listView.Items[count].SubItems.Add(TextBox1.Text);
		listView = null;
		TextBox1.Text = "";
	}

	private void Button6_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกรายการท\u0e35\u0e48จะลบ");
			return;
		}
		checked
		{
			int num = ListView1.SelectedItems.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ListView1.SelectedItems[0].Remove();
				num2++;
			}
			int num5 = ListView1.Items.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					ListView1.Items[num6].SubItems[0].Text = Conversions.ToString(num6 + 1);
					num6++;
					continue;
				}
				break;
			}
		}
	}

	private void Button7_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmSearchCustomers.ShowDialog();
		if (MyProject.Forms.FrmSearchCustomers.Return_ARR.Count == 0)
		{
			return;
		}
		checked
		{
			int num = MyProject.Forms.FrmSearchCustomers.Return_ARR.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					ListView listView = ListView1;
					int count = listView.Items.Count;
					listView.Items.Add(Conversions.ToString(count + 1));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					object[] array = new object[1];
					object[] array2 = array;
					object obj = MyProject.Forms.FrmSearchCustomers.Return_ARR[num2];
					object instance = obj;
					object[] array3 = new object[1];
					object[] array4 = array3;
					int num5 = 1;
					array4[0] = 1;
					array2[0] = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(instance, array3, null));
					object[] array5 = array;
					object[] arguments = array5;
					bool[] array6 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array6, IgnoreReturn: true);
					if (array6[0])
					{
						NewLateBinding.LateIndexSetComplex(obj, new object[2]
						{
							num5,
							RuntimeHelpers.GetObjectValue(array5[0])
						}, null, OptimisticSet: true, RValueBase: true);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
					array5 = new object[1];
					object[] array7 = array5;
					obj = MyProject.Forms.FrmSearchCustomers.Return_ARR[num2];
					object instance2 = obj;
					object[] array8 = new object[1];
					object[] array9 = array8;
					num5 = 0;
					array9[0] = 0;
					array7[0] = RuntimeHelpers.GetObjectValue(NewLateBinding.LateIndexGet(instance2, array8, null));
					array = array5;
					object[] arguments2 = array;
					array6 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array6, IgnoreReturn: true);
					if (array6[0])
					{
						NewLateBinding.LateIndexSetComplex(obj, new object[2]
						{
							num5,
							RuntimeHelpers.GetObjectValue(array[0])
						}, null, OptimisticSet: true, RValueBase: true);
					}
					listView = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}
}
