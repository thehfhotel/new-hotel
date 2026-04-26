using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.Editors;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmSettingsSMS : Form
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

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("GroupBox5")]
	private GroupBox _GroupBox5;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Label25")]
	private Label _Label25;

	[AccessedThroughProperty("Label24")]
	private Label _Label24;

	[AccessedThroughProperty("Label27")]
	private Label _Label27;

	[AccessedThroughProperty("Tpass")]
	private TextBox _Tpass;

	[AccessedThroughProperty("Tuser")]
	private TextBox _Tuser;

	[AccessedThroughProperty("Label23")]
	private Label _Label23;

	[AccessedThroughProperty("Label30")]
	private Label _Label30;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Label28")]
	private Label _Label28;

	[AccessedThroughProperty("Label26")]
	private Label _Label26;

	[AccessedThroughProperty("CHK_Browser")]
	private WebBrowser _CHK_Browser;

	public static Bitmap myBitmap;

	private bool AutoAret;

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

	internal virtual TextBox Tpass
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tpass;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tpass = value;
		}
	}

	internal virtual TextBox Tuser
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tuser;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Tuser = value;
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

	internal virtual WebBrowser CHK_Browser
	{
		[DebuggerNonUserCode]
		get
		{
			return _CHK_Browser;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			WebBrowserDocumentCompletedEventHandler value2 = WebBrowser1_DocumentCompleted;
			if (_CHK_Browser != null)
			{
				_CHK_Browser.DocumentCompleted -= value2;
			}
			_CHK_Browser = value;
			if (_CHK_Browser != null)
			{
				_CHK_Browser.DocumentCompleted += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmSettingsSMS()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmSettingsSMS()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmSettings_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		AutoAret = false;
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
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Button1 = new System.Windows.Forms.Button();
		this.GroupBox5 = new System.Windows.Forms.GroupBox();
		this.Tpass = new System.Windows.Forms.TextBox();
		this.Label28 = new System.Windows.Forms.Label();
		this.Tuser = new System.Windows.Forms.TextBox();
		this.Button3 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.Label26 = new System.Windows.Forms.Label();
		this.Label25 = new System.Windows.Forms.Label();
		this.Label24 = new System.Windows.Forms.Label();
		this.Label27 = new System.Windows.Forms.Label();
		this.Label23 = new System.Windows.Forms.Label();
		this.Label30 = new System.Windows.Forms.Label();
		this.CHK_Browser = new System.Windows.Forms.WebBrowser();
		this.GroupBox5.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		System.Drawing.Size size = new System.Drawing.Size(522, 32);
		panelEx2.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Lime;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Green;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "ต\u0e31\u0e49งค\u0e48า SMS";
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(405, 117);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(110, 33);
		button2.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "บ\u0e31นท\u0e36ก";
		this.Button1.UseVisualStyleBackColor = true;
		this.GroupBox5.Controls.Add(this.Tpass);
		this.GroupBox5.Controls.Add(this.Label28);
		this.GroupBox5.Controls.Add(this.Tuser);
		this.GroupBox5.Controls.Add(this.Button3);
		this.GroupBox5.Controls.Add(this.Button2);
		this.GroupBox5.Controls.Add(this.Label26);
		this.GroupBox5.Controls.Add(this.Label25);
		this.GroupBox5.Controls.Add(this.Label24);
		this.GroupBox5.Controls.Add(this.Label27);
		this.GroupBox5.Controls.Add(this.Label23);
		this.GroupBox5.Controls.Add(this.Label30);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox5;
		location = new System.Drawing.Point(14, 38);
		groupBox.Location = location;
		this.GroupBox5.Name = "GroupBox5";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox5;
		size = new System.Drawing.Size(490, 67);
		groupBox2.Size = size;
		this.GroupBox5.TabIndex = 33;
		this.GroupBox5.TabStop = false;
		this.GroupBox5.Text = "ต\u0e31\u0e49งค\u0e48า SMS";
		this.Tpass.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tpass.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tpass.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tpass = this.Tpass;
		location = new System.Drawing.Point(308, 17);
		tpass.Location = location;
		this.Tpass.MaxLength = 500;
		this.Tpass.Name = "Tpass";
		System.Windows.Forms.TextBox tpass2 = this.Tpass;
		size = new System.Drawing.Size(101, 21);
		tpass2.Size = size;
		this.Tpass.TabIndex = 5;
		this.Tpass.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label28.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Underline, System.Drawing.GraphicsUnit.Point, 222);
		this.Label28.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label = this.Label28;
		location = new System.Drawing.Point(298, 34);
		label.Location = location;
		this.Label28.Name = "Label28";
		System.Windows.Forms.Label label2 = this.Label28;
		size = new System.Drawing.Size(103, 30);
		label2.Size = size;
		this.Label28.TabIndex = 35;
		this.Label28.Text = "00/00/0000";
		this.Label28.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Tuser.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.Tuser.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Tuser.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.TextBox tuser = this.Tuser;
		location = new System.Drawing.Point(100, 15);
		tuser.Location = location;
		this.Tuser.MaxLength = 500;
		this.Tuser.Name = "Tuser";
		System.Windows.Forms.TextBox tuser2 = this.Tuser;
		size = new System.Drawing.Size(101, 21);
		tuser2.Size = size;
		this.Tuser.TabIndex = 2;
		this.Tuser.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		System.Windows.Forms.Button button3 = this.Button3;
		location = new System.Drawing.Point(407, 39);
		button3.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button4 = this.Button3;
		size = new System.Drawing.Size(74, 23);
		button4.Size = size;
		this.Button3.TabIndex = 34;
		this.Button3.Text = "ซ\u0e37\u0e49อ Credit";
		this.Button3.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button5 = this.Button2;
		location = new System.Drawing.Point(406, 16);
		button5.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button6 = this.Button2;
		size = new System.Drawing.Size(75, 23);
		button6.Size = size;
		this.Button2.TabIndex = 33;
		this.Button2.Text = "ตรวจสอบ";
		this.Button2.UseVisualStyleBackColor = true;
		this.Label26.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label26;
		location = new System.Drawing.Point(232, 45);
		label3.Location = location;
		this.Label26.Name = "Label26";
		System.Windows.Forms.Label label4 = this.Label26;
		size = new System.Drawing.Size(65, 13);
		label4.Size = size;
		this.Label26.TabIndex = 32;
		this.Label26.Text = "ว\u0e31นหมดอาย\u0e38 :";
		this.Label25.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label25;
		location = new System.Drawing.Point(183, 45);
		label5.Location = location;
		this.Label25.Name = "Label25";
		System.Windows.Forms.Label label6 = this.Label25;
		size = new System.Drawing.Size(45, 13);
		label6.Size = size;
		this.Label25.TabIndex = 32;
		this.Label25.Text = "ข\u0e49อความ";
		this.Label24.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Underline, System.Drawing.GraphicsUnit.Point, 222);
		this.Label24.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label7 = this.Label24;
		location = new System.Drawing.Point(99, 34);
		label7.Location = location;
		this.Label24.Name = "Label24";
		System.Windows.Forms.Label label8 = this.Label24;
		size = new System.Drawing.Size(81, 30);
		label8.Size = size;
		this.Label24.TabIndex = 31;
		this.Label24.Text = "0";
		this.Label24.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label27.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label27;
		location = new System.Drawing.Point(236, 21);
		label9.Location = location;
		this.Label27.Name = "Label27";
		System.Windows.Forms.Label label10 = this.Label27;
		size = new System.Drawing.Size(60, 13);
		label10.Size = size;
		this.Label27.TabIndex = 30;
		this.Label27.Text = "Password :";
		this.Label23.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label23;
		location = new System.Drawing.Point(18, 45);
		label11.Location = location;
		this.Label23.Name = "Label23";
		System.Windows.Forms.Label label12 = this.Label23;
		size = new System.Drawing.Size(81, 13);
		label12.Size = size;
		this.Label23.TabIndex = 30;
		this.Label23.Text = "เครด\u0e34ตคงเหล\u0e37อ :";
		this.Label30.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label30;
		location = new System.Drawing.Point(36, 19);
		label13.Location = location;
		this.Label30.Name = "Label30";
		System.Windows.Forms.Label label14 = this.Label30;
		size = new System.Drawing.Size(62, 13);
		label14.Size = size;
		this.Label30.TabIndex = 30;
		this.Label30.Text = "Username :";
		System.Windows.Forms.WebBrowser cHK_Browser = this.CHK_Browser;
		location = new System.Drawing.Point(12, 120);
		cHK_Browser.Location = location;
		System.Windows.Forms.WebBrowser cHK_Browser2 = this.CHK_Browser;
		size = new System.Drawing.Size(20, 20);
		cHK_Browser2.MinimumSize = size;
		this.CHK_Browser.Name = "CHK_Browser";
		System.Windows.Forms.WebBrowser cHK_Browser3 = this.CHK_Browser;
		size = new System.Drawing.Size(20, 24);
		cHK_Browser3.Size = size;
		this.CHK_Browser.TabIndex = 35;
		this.CHK_Browser.Visible = false;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(522, 156);
		this.ClientSize = size;
		this.Controls.Add(this.CHK_Browser);
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.GroupBox5);
		this.Controls.Add(this.Button1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmSettingsSMS";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "ต\u0e31\u0e49งค\u0e48า SMS";
		this.GroupBox5.ResumeLayout(false);
		this.GroupBox5.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FrmSettings_Load(object sender, EventArgs e)
	{
		LoadSMS();
		CHK_Browser.Url = new Uri("http://www.kpsystem.co.th/sms/sms.php?mode=check&u=" + Tuser.Text + "&p=" + Tpass.Text, UriKind.Absolute);
	}

	public void LoadSMS()
	{
		if (!File.Exists(Module1.PathF + "\\SetSMS.txt"))
		{
			StreamWriter streamWriter = File.CreateText(Module1.PathF + "\\SetSMS.txt");
			streamWriter.WriteLine("");
			streamWriter.WriteLine("");
			streamWriter.Close();
		}
		StreamReader streamReader = new StreamReader(Module1.PathF + "\\SetSMS.txt");
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
					Tuser.Text = left;
					break;
				case 2:
					Tpass.Text = left;
					break;
				}
			}
			num = checked(num + 1);
		}
	}

	public void SaveSMS()
	{
		StreamWriter streamWriter = File.CreateText(Module1.PathF + "\\SetSMS.txt");
		streamWriter.WriteLine(Tuser.Text);
		streamWriter.WriteLine(Tpass.Text);
		streamWriter.Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		Cursor = Cursors.WaitCursor;
		SaveSMS();
		Cursor = Cursors.Default;
		Close();
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Button2.Enabled = false;
		Cursor = Cursors.WaitCursor;
		AutoAret = true;
		CHK_Browser.Stop();
		CHK_Browser.DocumentText = "";
		CHK_Browser.Url = new Uri("http://www.kpsystem.co.th/sms/sms.php?mode=check&u=" + Tuser.Text + "&p=" + Tpass.Text, UriKind.Absolute);
	}

	private void WebBrowser1_DocumentCompleted(object sender, WebBrowserDocumentCompletedEventArgs e)
	{
		try
		{
			if (Operators.CompareString(CHK_Browser.DocumentText.Substring(0, CHK_Browser.DocumentText.IndexOf("|")), "-99", TextCompare: false) == 0)
			{
				Label24.Text = Conversions.ToString(0);
				Label28.Text = "00/00/0000";
				if (AutoAret)
				{
					MessageBox.Show("กร\u0e38ณาตรวจสอบ Username/Password");
				}
			}
			else if (Versioned.IsNumeric(CHK_Browser.DocumentText.Substring(0, CHK_Browser.DocumentText.IndexOf("|"))))
			{
				Label24.Text = CHK_Browser.DocumentText.Substring(0, CHK_Browser.DocumentText.IndexOf("|"));
				string text = CHK_Browser.DocumentText.Substring(checked(CHK_Browser.DocumentText.IndexOf("|") + 1));
				Label28.Text = text.Substring(8, 2) + "/" + text.Substring(5, 2) + "/" + text.Substring(0, 4);
				if (AutoAret)
				{
					MessageBox.Show("จำนวนเครด\u0e34ตค\u0e38ณคงเหล\u0e37อ " + Label24.Text + " ข\u0e49อความ");
				}
			}
			else
			{
				Label24.Text = Conversions.ToString(0);
				Label28.Text = "00/00/0000";
				if (AutoAret)
				{
					MessageBox.Show("กร\u0e38ณาตรวจสอบ Internet");
				}
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			Label24.Text = Conversions.ToString(0);
			Label28.Text = "00/00/0000";
			if (AutoAret)
			{
				MessageBox.Show("กร\u0e38ณาตรวจสอบ Internet");
			}
			ProjectData.ClearProjectError();
		}
		Button2.Enabled = true;
		Cursor = Cursors.Default;
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		FormSMSLog formSMSLog = new FormSMSLog();
		formSMSLog.ShowDialog();
	}
}
