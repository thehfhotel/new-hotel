using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSMSLog : Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("WebBrowser1")]
	private WebBrowser _WebBrowser1;

	private string SUser;

	private string Spass;

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
			_WebBrowser1 = value;
		}
	}

	[DebuggerNonUserCode]
	static FormSMSLog()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSMSLog()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormSMSLog_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		SUser = "";
		Spass = "";
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
		this.WebBrowser1 = new System.Windows.Forms.WebBrowser();
		this.SuspendLayout();
		this.WebBrowser1.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.WebBrowser webBrowser = this.WebBrowser1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		webBrowser.Location = location;
		System.Windows.Forms.WebBrowser webBrowser2 = this.WebBrowser1;
		System.Drawing.Size minimumSize = new System.Drawing.Size(20, 20);
		webBrowser2.MinimumSize = minimumSize;
		this.WebBrowser1.Name = "WebBrowser1";
		System.Windows.Forms.WebBrowser webBrowser3 = this.WebBrowser1;
		minimumSize = new System.Drawing.Size(769, 699);
		webBrowser3.Size = minimumSize;
		this.WebBrowser1.TabIndex = 0;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		minimumSize = new System.Drawing.Size(769, 699);
		this.ClientSize = minimumSize;
		this.Controls.Add(this.WebBrowser1);
		this.Name = "FormSMSLog";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ประว\u0e31ต\u0e34การใช\u0e49 SMS";
		this.TopMost = true;
		this.ResumeLayout(false);
	}

	private void FormSMSLog_Load(object sender, EventArgs e)
	{
		WebBrowser1.DocumentText = "";
		SUser = MyProject.Forms.FrmSettingsSMS.Tuser.Text;
		Spass = MyProject.Forms.FrmSettingsSMS.Tpass.Text;
		WebBrowser1.Url = new Uri("http://www.kpsystem.co.th/sms/smslog.php?&u=" + SUser + "&p=" + Spass, UriKind.Absolute);
	}
}
