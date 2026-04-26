using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Net;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmUpdate : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("BackgroundWorker1")]
	private BackgroundWorker _BackgroundWorker1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Timer2")]
	private Timer _Timer2;

	[AccessedThroughProperty("Timer3")]
	private Timer _Timer3;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBar _ProgressBarX1;

	[AccessedThroughProperty("PanelEx3")]
	private PanelEx _PanelEx3;

	[AccessedThroughProperty("Button1")]
	private ButtonX _Button1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("WebBrowser1")]
	private WebBrowser _WebBrowser1;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("PanelEx4")]
	private PanelEx _PanelEx4;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Timerclose")]
	private Timer _Timerclose;

	[AccessedThroughProperty("WebBrowser2")]
	private WebBrowser _WebBrowser2;

	[AccessedThroughProperty("Timer4")]
	private Timer _Timer4;

	private string UpdateUrl;

	private string Updatefilename_FULL;

	private string Updatefilename_SHORT;

	private string VERSION_HTTP;

	private string GETUpdateHTTP;

	private string FtpUsername;

	private string FtpPassowrd;

	private int NFilesUpdate;

	private bool bool_0;

	private long long_0;

	private string F_NAME;

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

	internal virtual BackgroundWorker BackgroundWorker1
	{
		[DebuggerNonUserCode]
		get
		{
			return _BackgroundWorker1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			DoWorkEventHandler value2 = BackgroundWorker1_DoWork;
			RunWorkerCompletedEventHandler value3 = BackgroundWorker1_RunWorkerCompleted;
			if (_BackgroundWorker1 != null)
			{
				_BackgroundWorker1.DoWork -= value2;
				_BackgroundWorker1.RunWorkerCompleted -= value3;
			}
			_BackgroundWorker1 = value;
			if (_BackgroundWorker1 != null)
			{
				_BackgroundWorker1.DoWork += value2;
				_BackgroundWorker1.RunWorkerCompleted += value3;
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
			EventHandler value2 = Timer3_Tick;
			if (_Timer3 != null)
			{
				_Timer3.Tick -= value2;
			}
			_Timer3 = value;
			if (_Timer3 != null)
			{
				_Timer3.Tick += value2;
			}
		}
	}

	internal virtual ProgressBar ProgressBarX1
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

	internal virtual PanelEx PanelEx3
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx3 = value;
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
			CancelEventHandler value2 = WebBrowser1_NewWindow;
			if (_WebBrowser1 != null)
			{
				_WebBrowser1.NewWindow -= value2;
			}
			_WebBrowser1 = value;
			if (_WebBrowser1 != null)
			{
				_WebBrowser1.NewWindow += value2;
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

	internal virtual PanelEx PanelEx4
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx4 = value;
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

	internal virtual Timer Timerclose
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timerclose;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timerclose_Tick;
			if (_Timerclose != null)
			{
				_Timerclose.Tick -= value2;
			}
			_Timerclose = value;
			if (_Timerclose != null)
			{
				_Timerclose.Tick += value2;
			}
		}
	}

	internal virtual WebBrowser WebBrowser2
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowser2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser2_DocumentCompleted;
			if (_WebBrowser2 != null)
			{
				_WebBrowser2.DocumentCompleted -= value2;
			}
			_WebBrowser2 = value;
			if (_WebBrowser2 != null)
			{
				_WebBrowser2.DocumentCompleted += value2;
			}
		}
	}

	internal virtual Timer Timer4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timer4_Tick;
			if (_Timer4 != null)
			{
				_Timer4.Tick -= value2;
			}
			_Timer4 = value;
			if (_Timer4 != null)
			{
				_Timer4.Tick += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmUpdate()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmUpdate()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmUpdate_FormClosing;
		base.Load += FrmUpdate_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		UpdateUrl = "ftp://downloads.kpddns.com/updates/";
		Updatefilename_FULL = "";
		Updatefilename_SHORT = "";
		VERSION_HTTP = "http://www.kpsystem.co.th/chk_ver_log.php?pro=hotel";
		GETUpdateHTTP = "http://www.kpsystem.co.th/version_get_update.php?PNAME=hotel";
		FtpUsername = "attosoft";
		FtpPassowrd = "1q2w3e4r";
		NFilesUpdate = 0;
		bool_0 = false;
		long_0 = 0L;
		F_NAME = "";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmUpdate));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.WebBrowser2 = new System.Windows.Forms.WebBrowser();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx3 = new DevComponents.DotNetBar.PanelEx();
		this.Button1 = new DevComponents.DotNetBar.ButtonX();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx4 = new DevComponents.DotNetBar.PanelEx();
		this.WebBrowser1 = new System.Windows.Forms.WebBrowser();
		this.ProgressBarX1 = new System.Windows.Forms.ProgressBar();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.BackgroundWorker1 = new System.ComponentModel.BackgroundWorker();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Timer2 = new System.Windows.Forms.Timer(this.components);
		this.Timer3 = new System.Windows.Forms.Timer(this.components);
		this.Timerclose = new System.Windows.Forms.Timer(this.components);
		this.Timer4 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2.SuspendLayout();
		this.PanelEx3.SuspendLayout();
		this.PanelEx4.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(639, 35);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Far;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.PanelEx1.Text = "อ\u0e31บเดทโปรแกรม |";
		this.WebBrowser2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.WebBrowser webBrowser = this.WebBrowser2;
		location = new System.Drawing.Point(19, 170);
		webBrowser.Location = location;
		System.Windows.Forms.WebBrowser webBrowser2 = this.WebBrowser2;
		size = new System.Drawing.Size(20, 20);
		webBrowser2.MinimumSize = size;
		this.WebBrowser2.Name = "WebBrowser2";
		System.Windows.Forms.WebBrowser webBrowser3 = this.WebBrowser2;
		size = new System.Drawing.Size(410, 230);
		webBrowser3.Size = size;
		this.WebBrowser2.TabIndex = 12;
		this.WebBrowser2.Url = new System.Uri("", System.UriKind.Relative);
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx2.Controls.Add(this.PanelEx3);
		this.PanelEx2.Controls.Add(this.ButtonX1);
		this.PanelEx2.Controls.Add(this.PanelEx4);
		this.PanelEx2.Controls.Add(this.ProgressBarX1);
		this.PanelEx2.Controls.Add(this.Label6);
		this.PanelEx2.Controls.Add(this.Label2);
		this.PanelEx2.Controls.Add(this.Label1);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		location = new System.Drawing.Point(0, 35);
		panelEx3.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		size = new System.Drawing.Size(639, 576);
		panelEx4.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Far;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 1;
		this.PanelEx3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx3.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx3.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.Office2007;
		this.PanelEx3.Controls.Add(this.Button1);
		this.PanelEx3.Controls.Add(this.Label8);
		this.PanelEx3.Controls.Add(this.Label7);
		this.PanelEx3.Controls.Add(this.Label3);
		this.PanelEx3.Controls.Add(this.Label4);
		this.PanelEx3.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx5 = this.PanelEx3;
		location = new System.Drawing.Point(0, 466);
		panelEx5.Location = location;
		this.PanelEx3.Name = "PanelEx3";
		DevComponents.DotNetBar.PanelEx panelEx6 = this.PanelEx3;
		size = new System.Drawing.Size(639, 110);
		panelEx6.Size = size;
		this.PanelEx3.Style.Alignment = System.Drawing.StringAlignment.Far;
		this.PanelEx3.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx3.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx3.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx3.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx3.Style.GradientAngle = 90;
		this.PanelEx3.TabIndex = 2;
		this.Button1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Button1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Button1.Cursor = System.Windows.Forms.Cursors.Hand;
		this.Button1.Enabled = false;
		this.Button1.FocusCuesEnabled = false;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Button1.Image = (System.Drawing.Image)resources.GetObject("Button1.Image");
		DevComponents.DotNetBar.ButtonX button = this.Button1;
		location = new System.Drawing.Point(239, 22);
		button.Location = location;
		this.Button1.Name = "Button1";
		DevComponents.DotNetBar.ButtonX button2 = this.Button1;
		size = new System.Drawing.Size(388, 36);
		button2.Size = size;
		this.Button1.TabIndex = 5;
		this.Button1.Text = "กดอ\u0e31บเดทโปรแกรม ท\u0e35\u0e48น\u0e35\u0e48";
		this.Label8.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Label8.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label8.ForeColor = System.Drawing.Color.DarkGreen;
		System.Windows.Forms.Label label = this.Label8;
		location = new System.Drawing.Point(1, 72);
		label.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label2 = this.Label8;
		size = new System.Drawing.Size(636, 33);
		label2.Size = size;
		this.Label8.TabIndex = 8;
		this.Label8.Text = "** ถ\u0e49าอ\u0e31บเดทไม\u0e48ผ\u0e48านลองป\u0e34ด Antivirus ก\u0e48อนอ\u0e31บเดท **\r\n** เพราะ Antivirus บางต\u0e31วจะมองโปรแกรมอ\u0e31บเดทเป\u0e47นไวร\u0e31ส ซ\u0e36\u0e48งโปรแกรมต\u0e31วน\u0e35\u0e49ไม\u0e48ม\u0e35ไวร\u0e31ส **";
		this.Label8.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label7.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		this.Label7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label7.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label3 = this.Label7;
		location = new System.Drawing.Point(8, 29);
		label3.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label4 = this.Label7;
		size = new System.Drawing.Size(225, 22);
		label4.Size = size;
		this.Label7.TabIndex = 10;
		this.Label7.Text = "เวอร\u0e4cช\u0e31\u0e48นของค\u0e38ณค\u0e37อ 1.7.8";
		this.Label7.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(28, 255);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(391, 34);
		label6.Size = size;
		this.Label3.TabIndex = 3;
		this.Label3.Text = "   ";
		this.Label3.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(396, 180);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(19, 14);
		label8.Size = size;
		this.Label4.TabIndex = 2;
		this.Label4.Text = "   ";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(542, 540);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		size = new System.Drawing.Size(91, 27);
		buttonX2.Size = size;
		this.ButtonX1.TabIndex = 0;
		this.ButtonX1.Text = "ป\u0e34ดโปรแกรม";
		this.PanelEx4.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.PanelEx4.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx4.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx4.Controls.Add(this.WebBrowser1);
		this.PanelEx4.Controls.Add(this.WebBrowser2);
		DevComponents.DotNetBar.PanelEx panelEx7 = this.PanelEx4;
		location = new System.Drawing.Point(12, 35);
		panelEx7.Location = location;
		this.PanelEx4.Name = "PanelEx4";
		DevComponents.DotNetBar.PanelEx panelEx8 = this.PanelEx4;
		size = new System.Drawing.Size(615, 422);
		panelEx8.Size = size;
		this.PanelEx4.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx4.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx4.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx4.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx4.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx4.Style.GradientAngle = 90;
		this.PanelEx4.TabIndex = 11;
		this.PanelEx4.Text = "PanelEx4";
		this.WebBrowser1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.WebBrowser webBrowser4 = this.WebBrowser1;
		location = new System.Drawing.Point(3, 2);
		webBrowser4.Location = location;
		System.Windows.Forms.WebBrowser webBrowser5 = this.WebBrowser1;
		size = new System.Drawing.Size(20, 20);
		webBrowser5.MinimumSize = size;
		this.WebBrowser1.Name = "WebBrowser1";
		System.Windows.Forms.WebBrowser webBrowser6 = this.WebBrowser1;
		size = new System.Drawing.Size(609, 417);
		webBrowser6.Size = size;
		this.WebBrowser1.TabIndex = 8;
		this.WebBrowser1.Url = new System.Uri("", System.UriKind.Relative);
		this.ProgressBarX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.ProgressBar progressBarX = this.ProgressBarX1;
		location = new System.Drawing.Point(15, 473);
		progressBarX.Location = location;
		this.ProgressBarX1.MarqueeAnimationSpeed = 50;
		this.ProgressBarX1.Name = "ProgressBarX1";
		System.Windows.Forms.ProgressBar progressBarX2 = this.ProgressBarX1;
		size = new System.Drawing.Size(612, 23);
		progressBarX2.Size = size;
		this.ProgressBarX1.Style = System.Windows.Forms.ProgressBarStyle.Continuous;
		this.ProgressBarX1.TabIndex = 4;
		this.Label6.AutoSize = true;
		this.Label6.Font = new System.Drawing.Font("Tahoma", 11.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label9 = this.Label6;
		location = new System.Drawing.Point(12, 10);
		label9.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label10 = this.Label6;
		size = new System.Drawing.Size(121, 18);
		label10.Size = size;
		this.Label6.TabIndex = 9;
		this.Label6.Text = "การปร\u0e31บปร\u0e38งเวอร\u0e4cช\u0e31\u0e48น";
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label11 = this.Label2;
		location = new System.Drawing.Point(269, 496);
		label11.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label12 = this.Label2;
		size = new System.Drawing.Size(355, 34);
		label12.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label13 = this.Label1;
		location = new System.Drawing.Point(18, 497);
		label13.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label14 = this.Label1;
		size = new System.Drawing.Size(245, 30);
		label14.Size = size;
		this.Label1.TabIndex = 2;
		this.Label1.Text = "   ";
		this.Timer1.Interval = 10;
		this.Timer2.Interval = 500;
		this.Timer3.Interval = 2000;
		this.Timerclose.Interval = 2000;
		this.Timer4.Enabled = true;
		this.Timer4.Interval = 2000;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(639, 611);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FrmUpdate";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "อ\u0e31บเดทโปรแกรม";
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx2.PerformLayout();
		this.PanelEx3.ResumeLayout(false);
		this.PanelEx3.PerformLayout();
		this.PanelEx4.ResumeLayout(false);
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

	private void FrmUpdate_FormClosing(object sender, FormClosingEventArgs e)
	{
		Timer4.Enabled = false;
	}

	private void FrmUpdate_Load(object sender, EventArgs e)
	{
		Updatefilename_FULL = "";
		Updatefilename_SHORT = "";
		Button1.Enabled = false;
		Button1.Text = "กร\u0e38ณาตรวจสอบอ\u0e34นเตอร\u0e4cเน\u0e47ต";
		Label7.Text = "เวอร\u0e4cช\u0e31\u0e48นของค\u0e38ณค\u0e37อ " + Module1.ProgramVersion;
		PanelEx3.Visible = true;
		WebBrowser2.Navigate(GETUpdateHTTP);
		WebBrowser1.Navigate(VERSION_HTTP);
		Timer4.Enabled = true;
	}

	private void BackgroundWorker1_DoWork(object sender, DoWorkEventArgs e)
	{
		bool_0 = true;
		Uri requestUri = new Uri(UpdateUrl + Updatefilename_SHORT);
		MemoryStream memoryStream = new MemoryStream();
		int num = 0;
		FtpWebRequest ftpWebRequest = (FtpWebRequest)WebRequest.Create(requestUri);
		ftpWebRequest.Credentials = new NetworkCredential(Conversions.ToString(FtpUsername), Conversions.ToString(FtpPassowrd));
		ftpWebRequest.Timeout = 60000;
		ftpWebRequest.KeepAlive = false;
		ftpWebRequest.UseBinary = true;
		ftpWebRequest.Method = "RETR";
		FtpWebResponse ftpWebResponse;
		try
		{
			ftpWebResponse = (FtpWebResponse)ftpWebRequest.GetResponse();
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show("ผ\u0e34ดพลาด \r\n" + ex2.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			ProjectData.ClearProjectError();
			return;
		}
		Stream responseStream = ftpWebResponse.GetResponseStream();
		byte[] array = new byte[257];
		int num2 = responseStream.Read(array, 0, array.Length);
		num = num2;
		checked
		{
			while (num2 > 0)
			{
				memoryStream.Write(array, 0, num2);
				num2 = responseStream.Read(array, 0, array.Length);
				num += num2;
				long_0 += num2;
			}
			memoryStream.Flush();
			memoryStream.Seek(0L, SeekOrigin.Begin);
			responseStream.Close();
			ftpWebResponse.Close();
			try
			{
				FileStream fileStream = File.OpenWrite(Module1.PathF + "\\" + Updatefilename_SHORT);
				memoryStream.WriteTo(fileStream);
				memoryStream.Close();
				fileStream.Flush();
				fileStream.Close();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			if (File.Exists(Module1.PathF + "\\" + Updatefilename_SHORT))
			{
				NFilesUpdate++;
			}
		}
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Label1.Text = "กำล\u0e31ง Download ไฟล\u0e4cอ\u0e31ปเดท...";
		try
		{
			ProgressBarX1.Maximum = 100;
			if ((double)long_0 / 1000000.0 > 32.0)
			{
				ProgressBarX1.Value = 100;
			}
			else
			{
				ProgressBarX1.Value = checked((int)Math.Round((double)long_0 / 1000000.0 * 3.0));
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProgressBarX1.Maximum = 100;
			ProgressBarX1.Value = 100;
			ProjectData.ClearProjectError();
		}
		Label2.Text = "[FTP] กำล\u0e31งโหลด " + Updatefilename_SHORT + Environment.NewLine + "Loading.. " + Module1.FormatFileSize(long_0);
	}

	private void BackgroundWorker1_RunWorkerCompleted(object sender, RunWorkerCompletedEventArgs e)
	{
		Timer1.Enabled = false;
		if (NFilesUpdate != 1)
		{
			MessageBox.Show("อ\u0e31บเดทไฟล\u0e4cไม\u0e48ได\u0e49 โปรแกรมจะลองโหลดใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			method_1();
			return;
		}
		Timer3.Enabled = true;
		Label1.Text = "Download ไฟล\u0e4cอ\u0e31ปเดทเสร\u0e47จเร\u0e35ยบร\u0e49อย...";
		ProgressBarX1.Maximum = 100;
		ProgressBarX1.Value = 100;
		Label2.Text = "จำนวนไฟล\u0e4c " + Conversions.ToString(1) + "/" + Conversions.ToString(1);
		ButtonX1.Enabled = false;
		ButtonX1.Text = "Waiting..";
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		ProjectData.EndApp();
	}

	private void Timer2_Tick(object sender, EventArgs e)
	{
		Timer2.Enabled = false;
		Timer1.Enabled = true;
		BackgroundWorker1.RunWorkerAsync();
	}

	private void Timer3_Tick(object sender, EventArgs e)
	{
		Timer3.Enabled = false;
		MessageBox.Show("ดาวน\u0e4cโหลดไฟล\u0e4cอ\u0e31บเดทเสร\u0e47จเร\u0e35ยบร\u0e49อย หล\u0e31งจากเป\u0e34ดโปรแกรมอ\u0e31บเดท ให\u0e49กดป\u0e38\u0e48ม Extract จากน\u0e31\u0e49นกด Yes to All เพ\u0e37\u0e48ออ\u0e31บเดทโปรแกรม", "อ\u0e31บเดทโปรแกรม", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
		new Process();
		Process.Start(Module1.PathF + Updatefilename_SHORT, "");
		ProjectData.EndApp();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Updatefilename_FULL, "", TextCompare: false) == 0)
		{
			WebBrowser2.Navigate(GETUpdateHTTP);
			MessageBox.Show("ไม\u0e48สามารถต\u0e34ดต\u0e48อเซ\u0e34ฟเวอร\u0e4cได\u0e49 กร\u0e38ณาตรวจสอบ อ\u0e34นเตอร\u0e4cเน\u0e47ต แล\u0e49วลองใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else if (!bool_0)
		{
			method_0();
		}
		else
		{
			method_1();
		}
	}

	public void method_0()
	{
		PanelEx3.Visible = false;
		Application.DoEvents();
		Module1.PathF = Assembly.GetExecutingAssembly().Location;
		Module1.PathF = Module1.PathF.Substring(0, checked(Module1.PathF.LastIndexOf("\\") + 1));
		Label1.Text = "กำล\u0e31งต\u0e34ดต\u0e48อ Server...";
		Timer2.Enabled = true;
	}

	public void method_1()
	{
		string updatefilename_FULL = Updatefilename_FULL;
		if (updatefilename_FULL.IndexOf("http") != -1)
		{
			DL(updatefilename_FULL);
		}
		else if (updatefilename_FULL.IndexOf(".exe") != -1)
		{
			DL("http://www.kpsystem.co.th/downloads/" + updatefilename_FULL);
		}
		else if (updatefilename_FULL.IndexOf(".zip") != -1)
		{
			DL("http://www.kpsystem.co.th/downloads/" + updatefilename_FULL);
		}
	}

	public void DL(string URL)
	{
		F_NAME = checked(URL.Substring(URL.LastIndexOf("/") + 1, URL.Length - URL.LastIndexOf("/") - 1));
		if (File.Exists(Module1.PathF + F_NAME))
		{
			try
			{
				File.Delete(Module1.PathF + F_NAME);
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				MessageBox.Show("ไม\u0e48สามารถลบไฟล\u0e4cอ\u0e31บเดทเก\u0e48าได\u0e49 กร\u0e38ณาป\u0e34ดไฟล\u0e4cเก\u0e48าก\u0e48อน ถ\u0e49าไม\u0e48ได\u0e49ลอง ป\u0e34ดเป\u0e34ด เคร\u0e37\u0e48องใหม\u0e48แล\u0e49วลองใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				ProjectData.ClearProjectError();
				return;
			}
		}
		PanelEx3.Visible = false;
		Label2.Text = "";
		ProgressBarX1.Value = 0;
		ProgressBarX1.Maximum = 100;
		WebClient webClient = new WebClient();
		webClient.DownloadProgressChanged += client_ProgressChanged;
		webClient.DownloadFileCompleted += client_DownloadCompleted;
		webClient.DownloadFileAsync(new Uri(URL), F_NAME);
		Label1.Text = "กำล\u0e31งโหลดไฟล\u0e4c " + F_NAME;
	}

	private void client_ProgressChanged(object sender, DownloadProgressChangedEventArgs e)
	{
		double num = double.Parse(e.BytesReceived.ToString());
		double num2 = double.Parse(e.TotalBytesToReceive.ToString());
		double num3 = num / num2 * 100.0;
		ProgressBarX1.Value = int.Parse(Math.Truncate(num3).ToString());
		Label2.Text = checked("[HTTP] " + Module1.FormatFileSize((long)Math.Round(num)) + " / " + Module1.FormatFileSize((long)Math.Round(num2))) + " (" + Strings.Format(num3, "0.00") + "%)";
	}

	private void client_DownloadCompleted(object sender, AsyncCompletedEventArgs e)
	{
		if (File.Exists(Module1.PathF + F_NAME) && FileSystem.FileLen(Module1.PathF + F_NAME) == 0L)
		{
			File.Delete(Module1.PathF + F_NAME);
		}
		if (!File.Exists(Module1.PathF + F_NAME))
		{
			MessageBox.Show("ไม\u0e48สามารถโหลดไฟล\u0e4cได\u0e49 กร\u0e38ณาลองใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			PanelEx3.Visible = true;
			return;
		}
		Label2.Text = "Download Complete";
		MessageBox.Show("ดาวน\u0e4cโหลดไฟล\u0e4cอ\u0e31บเดทเสร\u0e47จเร\u0e35ยบร\u0e49อย หล\u0e31งจากเป\u0e34ดโปรแกรมอ\u0e31บเดท ให\u0e49กดป\u0e38\u0e48ม Extract จากน\u0e31\u0e49นกด Yes to All เพ\u0e37\u0e48ออ\u0e31บเดทโปรแกรม", "อ\u0e31บเดทโปรแกรม", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
		new Process();
		Process.Start(Module1.PathF + F_NAME, "");
		Close();
		ProjectData.EndApp();
	}

	private void WebBrowser1_NewWindow(object sender, CancelEventArgs e)
	{
		Timerclose.Enabled = true;
	}

	private void Timerclose_Tick(object sender, EventArgs e)
	{
		Timerclose.Enabled = false;
		MyProject.Forms.frmMain1.Close();
	}

	private void WebBrowser2_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		Button1.Enabled = false;
		string text = WebBrowser2.DocumentText;
		try
		{
			text = Strings.Trim(text);
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show(ex2.Message);
			ProjectData.ClearProjectError();
		}
		checked
		{
			if (text.IndexOf("http://") != -1)
			{
				Updatefilename_FULL = text;
				Updatefilename_SHORT = text.Substring(text.LastIndexOf("/") + 1, text.Length - text.LastIndexOf("/") - 1);
			}
			else if (text.IndexOf("https://") != -1)
			{
				Updatefilename_FULL = text;
				Updatefilename_SHORT = text.Substring(text.LastIndexOf("/") + 1, text.Length - text.LastIndexOf("/") - 1);
			}
			else if (text.IndexOf(".exe") != -1)
			{
				Updatefilename_FULL = text;
				Updatefilename_SHORT = text;
			}
			else if (text.IndexOf(".zip") != -1)
			{
				Updatefilename_FULL = text;
				Updatefilename_SHORT = text;
			}
			if (Operators.CompareString(Updatefilename_FULL, "", TextCompare: false) != 0)
			{
				Button1.Enabled = true;
				Button1.Text = "กดอ\u0e31บเดทโปรแกรม ท\u0e35\u0e48น\u0e35\u0e48";
				if (File.Exists(Module1.PathF + Updatefilename_SHORT))
				{
					try
					{
						File.Delete(Module1.PathF + Updatefilename_SHORT);
						bool_0 = false;
					}
					catch (Exception projectError)
					{
						ProjectData.SetProjectError(projectError);
						bool_0 = true;
						ProjectData.ClearProjectError();
					}
				}
			}
			if (Operators.CompareString(Updatefilename_FULL, "", TextCompare: false) == 0)
			{
				Button1.Text = "กร\u0e38ณาตรวจสอบอ\u0e34นเตอร\u0e4cเน\u0e47ต";
			}
		}
	}

	private void Timer4_Tick(object sender, EventArgs e)
	{
		if (Operators.CompareString(Updatefilename_FULL, "", TextCompare: false) == 0)
		{
			Button1.Enabled = false;
			Button1.Text = "กำล\u0e31งค\u0e49นหาเซ\u0e34ฟเวอร\u0e4c...";
			WebBrowser1.Navigate(VERSION_HTTP);
			WebBrowser2.Navigate(GETUpdateHTTP);
		}
	}
}
