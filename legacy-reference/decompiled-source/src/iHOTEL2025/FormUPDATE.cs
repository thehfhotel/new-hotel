using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormUPDATE : Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Label_HOST")]
	private Label _Label_HOST;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("IP_USER")]
	private TextBox _IP_USER;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("IP_PASS")]
	private TextBox _IP_PASS;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label_IP")]
	private Label _Label_IP;

	[AccessedThroughProperty("Label_status")]
	private Label _Label_status;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("WebBrowser3_UPDATEI_IP")]
	private WebBrowser _WebBrowser3_UPDATEI_IP;

	[AccessedThroughProperty("WebBrowser1_checkip")]
	private WebBrowser _WebBrowser1_checkip;

	private bool WEBOK;

	private int isloadweb;

	private string lastip;

	private string string_0;

	private string string_1;

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

	internal virtual Label Label_HOST
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_HOST;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_HOST = value;
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

	internal virtual TextBox IP_USER
	{
		[DebuggerNonUserCode]
		get
		{
			return _IP_USER;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = IP_USER_KeyDown;
			if (_IP_USER != null)
			{
				_IP_USER.KeyDown -= value2;
			}
			_IP_USER = value;
			if (_IP_USER != null)
			{
				_IP_USER.KeyDown += value2;
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

	internal virtual TextBox IP_PASS
	{
		[DebuggerNonUserCode]
		get
		{
			return _IP_PASS;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = IP_USER_KeyDown;
			if (_IP_PASS != null)
			{
				_IP_PASS.KeyDown -= value2;
			}
			_IP_PASS = value;
			if (_IP_PASS != null)
			{
				_IP_PASS.KeyDown += value2;
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

	internal virtual Label Label_IP
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_IP;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_IP = value;
		}
	}

	internal virtual Label Label_status
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_status;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_status = value;
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

	internal virtual WebBrowser WebBrowser3_UPDATEI_IP
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowser3_UPDATEI_IP;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser3_UPDATEI_IP_DocumentCompleted;
			if (_WebBrowser3_UPDATEI_IP != null)
			{
				_WebBrowser3_UPDATEI_IP.DocumentCompleted -= value2;
			}
			_WebBrowser3_UPDATEI_IP = value;
			if (_WebBrowser3_UPDATEI_IP != null)
			{
				_WebBrowser3_UPDATEI_IP.DocumentCompleted += value2;
			}
		}
	}

	internal virtual WebBrowser WebBrowser1_checkip
	{
		[DebuggerNonUserCode]
		get
		{
			return _WebBrowser1_checkip;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser1_checkip_DocumentCompleted;
			if (_WebBrowser1_checkip != null)
			{
				_WebBrowser1_checkip.DocumentCompleted -= value2;
			}
			_WebBrowser1_checkip = value;
			if (_WebBrowser1_checkip != null)
			{
				_WebBrowser1_checkip.DocumentCompleted += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FormUPDATE()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormUPDATE()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FormUPDATE_FormClosing;
		base.Load += FormUPDATE_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		WEBOK = true;
		isloadweb = 0;
		lastip = "";
		string_0 = "https://kpsystem.co.th";
		string_1 = "https://kpline2.com/ANLOTTO";
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormUPDATE));
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label_HOST = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.IP_USER = new System.Windows.Forms.TextBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.IP_PASS = new System.Windows.Forms.TextBox();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label_IP = new System.Windows.Forms.Label();
		this.Label_status = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.WebBrowser3_UPDATEI_IP = new System.Windows.Forms.WebBrowser();
		this.WebBrowser1_checkip = new System.Windows.Forms.WebBrowser();
		this.SuspendLayout();
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		System.Drawing.Point location = new System.Drawing.Point(38, 163);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		System.Drawing.Size size = new System.Drawing.Size(273, 23);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 0;
		this.CheckBox1.Text = "ใช\u0e49การอ\u0e31บเดท IP ADDRESS อ\u0e31ตโนม\u0e31ต\u0e34";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(56, 16);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(62, 19);
		label2.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "HOST :";
		this.Label_HOST.AutoSize = true;
		this.Label_HOST.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label label_HOST = this.Label_HOST;
		location = new System.Drawing.Point(122, 15);
		label_HOST.Location = location;
		this.Label_HOST.Name = "Label_HOST";
		System.Windows.Forms.Label label_HOST2 = this.Label_HOST;
		size = new System.Drawing.Size(15, 19);
		label_HOST2.Size = size;
		this.Label_HOST.TabIndex = 2;
		this.Label_HOST.Text = "-";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label3;
		location = new System.Drawing.Point(27, 85);
		label3.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label4 = this.Label3;
		size = new System.Drawing.Size(91, 19);
		label4.Size = size;
		this.Label3.TabIndex = 3;
		this.Label3.Text = "Username :";
		System.Windows.Forms.TextBox iP_USER = this.IP_USER;
		location = new System.Drawing.Point(120, 81);
		iP_USER.Location = location;
		this.IP_USER.Name = "IP_USER";
		System.Windows.Forms.TextBox iP_USER2 = this.IP_USER;
		size = new System.Drawing.Size(203, 27);
		iP_USER2.Size = size;
		this.IP_USER.TabIndex = 0;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label4;
		location = new System.Drawing.Point(31, 123);
		label5.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label6 = this.Label4;
		size = new System.Drawing.Size(87, 19);
		label6.Size = size;
		this.Label4.TabIndex = 3;
		this.Label4.Text = "Password :";
		System.Windows.Forms.TextBox iP_PASS = this.IP_PASS;
		location = new System.Drawing.Point(120, 119);
		iP_PASS.Location = location;
		this.IP_PASS.Name = "IP_PASS";
		this.IP_PASS.PasswordChar = '•';
		System.Windows.Forms.TextBox iP_PASS2 = this.IP_PASS;
		size = new System.Drawing.Size(203, 27);
		iP_PASS2.Size = size;
		this.IP_PASS.TabIndex = 1;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		this.ButtonX1.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(346, 81);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		size = new System.Drawing.Size(119, 65);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "อ\u0e31บเดทเด\u0e35\u0e4bยวน\u0e35\u0e49";
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label5;
		location = new System.Drawing.Point(8, 45);
		label7.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label8 = this.Label5;
		size = new System.Drawing.Size(110, 19);
		label8.Size = size;
		this.Label5.TabIndex = 6;
		this.Label5.Text = "IP ADDRESS :";
		this.Label_IP.AutoSize = true;
		this.Label_IP.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label_IP = this.Label_IP;
		location = new System.Drawing.Point(122, 45);
		label_IP.Location = location;
		this.Label_IP.Name = "Label_IP";
		System.Windows.Forms.Label label_IP2 = this.Label_IP;
		size = new System.Drawing.Size(15, 19);
		label_IP2.Size = size;
		this.Label_IP.TabIndex = 7;
		this.Label_IP.Text = "-";
		this.Label_status.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label_status = this.Label_status;
		location = new System.Drawing.Point(23, 188);
		label_status.Location = location;
		this.Label_status.Name = "Label_status";
		System.Windows.Forms.Label label_status2 = this.Label_status;
		size = new System.Drawing.Size(429, 31);
		label_status2.Size = size;
		this.Label_status.TabIndex = 26;
		this.Label_status.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Timer1.Interval = 10000;
		System.Windows.Forms.WebBrowser webBrowser3_UPDATEI_IP = this.WebBrowser3_UPDATEI_IP;
		location = new System.Drawing.Point(23, 345);
		webBrowser3_UPDATEI_IP.Location = location;
		System.Windows.Forms.WebBrowser webBrowser3_UPDATEI_IP2 = this.WebBrowser3_UPDATEI_IP;
		size = new System.Drawing.Size(20, 20);
		webBrowser3_UPDATEI_IP2.MinimumSize = size;
		this.WebBrowser3_UPDATEI_IP.Name = "WebBrowser3_UPDATEI_IP";
		this.WebBrowser3_UPDATEI_IP.ScriptErrorsSuppressed = true;
		System.Windows.Forms.WebBrowser webBrowser3_UPDATEI_IP3 = this.WebBrowser3_UPDATEI_IP;
		size = new System.Drawing.Size(429, 87);
		webBrowser3_UPDATEI_IP3.Size = size;
		this.WebBrowser3_UPDATEI_IP.TabIndex = 27;
		System.Windows.Forms.WebBrowser webBrowser1_checkip = this.WebBrowser1_checkip;
		location = new System.Drawing.Point(24, 240);
		webBrowser1_checkip.Location = location;
		System.Windows.Forms.WebBrowser webBrowser1_checkip2 = this.WebBrowser1_checkip;
		size = new System.Drawing.Size(20, 20);
		webBrowser1_checkip2.MinimumSize = size;
		this.WebBrowser1_checkip.Name = "WebBrowser1_checkip";
		this.WebBrowser1_checkip.ScriptErrorsSuppressed = true;
		System.Windows.Forms.WebBrowser webBrowser1_checkip3 = this.WebBrowser1_checkip;
		size = new System.Drawing.Size(429, 99);
		webBrowser1_checkip3.Size = size;
		this.WebBrowser1_checkip.TabIndex = 28;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(9f, 19f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(477, 228);
		this.ClientSize = size;
		this.Controls.Add(this.WebBrowser1_checkip);
		this.Controls.Add(this.WebBrowser3_UPDATEI_IP);
		this.Controls.Add(this.Label_status);
		this.Controls.Add(this.Label_IP);
		this.Controls.Add(this.Label5);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.IP_PASS);
		this.Controls.Add(this.Label4);
		this.Controls.Add(this.IP_USER);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Label_HOST);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.CheckBox1);
		this.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FormUPDATE";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "UPDATE IP ADDRESS";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void FormUPDATE_FormClosing(object sender, FormClosingEventArgs e)
	{
		save_update();
	}

	private void FormUPDATE_Load(object sender, EventArgs e)
	{
		load_update();
	}

	public void load_update()
	{
		CheckBox1.Checked = false;
		if (!File.Exists(Module1.PathF + "ddns.txt"))
		{
			save_update();
		}
		StreamReader streamReader = new StreamReader(Module1.PathF + "\\ddns.txt", Encoding.Default);
		string[] array = default(string[]);
		while (streamReader.Peek() != -1)
		{
			string encryptedString = streamReader.ReadLine();
			array = FormEN_DE.Decrypt1(encryptedString, "156Unh").Split('|');
		}
		streamReader.Close();
		streamReader = null;
		try
		{
			CheckBox1.Checked = Conversions.ToBoolean(Strings.Trim(array[0]));
			IP_USER.Text = Strings.Trim(array[1]);
			IP_PASS.Text = Strings.Trim(array[2]);
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		if (CheckBox1.Checked)
		{
			ButtonX1_Click(null, null);
			return;
		}
		MyProject.Forms.frmMain1.LabelStatus.Text = "IP UPDATE : ไม\u0e48ได\u0e49ใช\u0e49งาน";
		MyProject.Forms.frmMain1.ribbonControl1.Refresh();
	}

	public void save_update()
	{
		string stringToEncrypt = " " + Conversions.ToString(CheckBox1.Checked) + "| " + IP_USER.Text + "| " + IP_PASS.Text;
		try
		{
			StreamWriter streamWriter = new StreamWriter(Module1.PathF + "ddns.txt");
			streamWriter.Write(FormEN_DE.Encrypt1(stringToEncrypt, "156Unh"));
			streamWriter.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public void ButtonX1_Click(object sender, EventArgs e)
	{
		ButtonX1.Enabled = false;
		ButtonX1.Text = "กำล\u0e31งอ\u0e31บเดท IP..";
		MyProject.Forms.frmMain1.LabelStatus.Text = "DDNS : กำล\u0e31งอ\u0e31บเดท IP...";
		if (!CheckBox1.Checked)
		{
			MyProject.Forms.frmMain1.LabelStatus.Text = "DDNS : ป\u0e34ด";
		}
		WEBOK = false;
		WebBrowser1_checkip.Navigate(new Uri("http://checkip.dyndns.org/"));
	}

	private void WebBrowser3_UPDATEI_IP_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		WEBOK = true;
		string text = Strings.Trim(WebBrowser3_UPDATEI_IP.DocumentText);
		if (text.IndexOf("|OK|") != -1)
		{
			Label_status.Text = "อ\u0e31บเดทสำเร\u0e47จ!! เวลา " + Strings.Format(DateTime.Now, "dd/MM/yy HH:mm:ss");
			Label_IP.Text = text.Replace("|OK|", "").Substring(0, text.Replace("|OK|", "").IndexOf("|"));
			Label_HOST.Text = text.Replace("|OK|", "").Substring(checked(text.Replace("|OK|", "").IndexOf("|") + 1));
			MyProject.Forms.frmMain1.LabelStatus.Text = "IP UPDATE : " + Label_HOST.Text + "@" + Label_IP.Text + "  [ " + Strings.Format(DateTime.Now, "HH:mm:ss") + " ]";
			lastip = Label_IP.Text;
			CheckBox1.Checked = true;
		}
		else if (text.IndexOf("|NOUSER|") != -1)
		{
			Label_status.Text = "ไม\u0e48พบ User/Password";
			Label_HOST.Text = "-";
			MyProject.Forms.frmMain1.LabelStatus.Text = "IP UPDATE : Error ไม\u0e48พบ User/Password";
			CheckBox1.Checked = false;
		}
		else
		{
			Label_status.Text = "กร\u0e38ณาตรวจสอบอ\u0e34นเตอร\u0e4cเน\u0e47ต";
			Label_HOST.Text = "-";
			Label_IP.Text = "-";
			MyProject.Forms.frmMain1.LabelStatus.Text = "IP UPDATE : ไม\u0e48ได\u0e49เช\u0e37\u0e48อมต\u0e48ออ\u0e34นเตอร\u0e4cเน\u0e47ต";
			if (isloadweb == 0)
			{
				isloadweb = 1;
				ButtonX1_Click(null, null);
			}
			else
			{
				isloadweb = 0;
			}
		}
		ButtonX1.Enabled = true;
		ButtonX1.Text = "อ\u0e31บเดทเด\u0e35\u0e4bยวน\u0e35\u0e49";
		Timer1.Enabled = false;
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		if (!WEBOK)
		{
			MyProject.Forms.frmMain1.LabelStatus.Text = "IP UPDATE : กร\u0e38ณาตรวบสอบเคร\u0e37อข\u0e48าย";
			ButtonX1.Enabled = true;
			ButtonX1.Text = "อ\u0e31บเดทเด\u0e35\u0e4bยวน\u0e35\u0e49";
		}
	}

	private void WebBrowser3_UPDATEI_IP_Navigating(object sender, WebBrowserNavigatingEventArgs e)
	{
		Timer1.Enabled = true;
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
	}

	private void WebBrowser1_checkip_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		string text = Strings.Trim(WebBrowser1_checkip.DocumentText);
		string text2 = "";
		if (text.ToUpper().IndexOf("IP") != -1)
		{
			text2 = text.Substring(checked(text.ToUpper().IndexOf(":") + 2));
			text2 = text2.Substring(0, text2.ToUpper().IndexOf("<"));
		}
		if (Versioned.IsNumeric(text2.Replace(".", "")))
		{
			Label_IP.Text = text2;
		}
		else
		{
			text2 = "";
			Label_IP.Text = "-";
		}
		if ((Operators.CompareString(IP_USER.Text, "test", TextCompare: false) != 0) & (Operators.CompareString(text2, "", TextCompare: false) != 0))
		{
			if (Operators.CompareString(lastip, text2, TextCompare: false) != 0)
			{
				if (isloadweb == 0)
				{
					WebBrowser3_UPDATEI_IP.Navigate(new Uri(string_1 + "/update_ip.php?U=" + IP_USER.Text + "&P=" + IP_PASS.Text + "&IP=" + text2));
				}
				else
				{
					WebBrowser3_UPDATEI_IP.Navigate(new Uri(string_0 + "/update_ip.php?U=" + IP_USER.Text + "&P=" + IP_PASS.Text + "&IP=" + text2));
				}
			}
			else
			{
				Label_status.Text = "อ\u0e31บเดทเป\u0e47น IP ป\u0e31จจ\u0e38บ\u0e31นแล\u0e49ว เวลา " + Strings.Format(DateTime.Now, "dd/MM/yy HH:mm:ss");
				MyProject.Forms.frmMain1.LabelStatus.Text = "IP UPDATE : " + Label_HOST.Text + "@" + Label_IP.Text + "  [ " + Strings.Format(DateTime.Now, "HH:mm:ss") + " ].";
				ButtonX1.Text = "อ\u0e31บเดทเด\u0e35\u0e4bยวน\u0e35\u0e49";
				ButtonX1.Enabled = true;
			}
		}
		else if ((Operators.CompareString(IP_USER.Text, "test", TextCompare: false) != 0) & (Operators.CompareString(text2, "", TextCompare: false) == 0))
		{
			if (isloadweb == 0)
			{
				WebBrowser3_UPDATEI_IP.Navigate(new Uri(string_1 + "/update_ip.php?U=" + IP_USER.Text + "&P=" + IP_PASS.Text + "&IP=" + text2));
			}
			else
			{
				WebBrowser3_UPDATEI_IP.Navigate(new Uri(string_0 + "/update_ip.php?U=" + IP_USER.Text + "&P=" + IP_PASS.Text + "&IP=" + text2));
			}
		}
		else
		{
			MyProject.Forms.frmMain1.LabelStatus.Text = "DDNS : ไม\u0e48ใช\u0e49งาน";
			ButtonX1.Text = "อ\u0e31บเดทเด\u0e35\u0e4bยวน\u0e35\u0e49";
			ButtonX1.Enabled = true;
		}
	}

	private void IP_USER_KeyDown(object sender, KeyEventArgs e)
	{
		lastip = "";
	}
}
