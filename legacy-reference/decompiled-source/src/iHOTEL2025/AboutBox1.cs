using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public sealed class AboutBox1 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	[AccessedThroughProperty("TableLayoutPanel")]
	private TableLayoutPanel _TableLayoutPanel;

	[AccessedThroughProperty("LogoPictureBox")]
	private PictureBox _LogoPictureBox;

	[AccessedThroughProperty("LabelProductName")]
	private Label _LabelProductName;

	[AccessedThroughProperty("LabelVersion")]
	private Label _LabelVersion;

	[AccessedThroughProperty("LabelCompanyName")]
	private Label _LabelCompanyName;

	[AccessedThroughProperty("TextBoxDescription")]
	private TextBox _TextBoxDescription;

	[AccessedThroughProperty("LabelCopyright")]
	private Label _LabelCopyright;

	private IContainer components;

	[AccessedThroughProperty("OKButton")]
	private Button _OKButton;

	internal virtual TableLayoutPanel TableLayoutPanel
	{
		[DebuggerNonUserCode]
		get
		{
			return _TableLayoutPanel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TableLayoutPanel = value;
		}
	}

	internal virtual PictureBox LogoPictureBox
	{
		[DebuggerNonUserCode]
		get
		{
			return _LogoPictureBox;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LogoPictureBox = value;
		}
	}

	internal virtual Label LabelProductName
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelProductName;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelProductName = value;
		}
	}

	internal virtual Label LabelVersion
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelVersion;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelVersion = value;
		}
	}

	internal virtual Label LabelCompanyName
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelCompanyName;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelCompanyName = value;
		}
	}

	internal virtual TextBox TextBoxDescription
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBoxDescription;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBoxDescription = value;
		}
	}

	internal virtual Label LabelCopyright
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelCopyright;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelCopyright = value;
		}
	}

	internal virtual Button OKButton
	{
		[DebuggerNonUserCode]
		get
		{
			return _OKButton;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = OKButton_Click;
			if (_OKButton != null)
			{
				_OKButton.Click -= value2;
			}
			_OKButton = value;
			if (_OKButton != null)
			{
				_OKButton.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static AboutBox1()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public AboutBox1()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += AboutBox1_Load;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.AboutBox1));
		this.TableLayoutPanel = new System.Windows.Forms.TableLayoutPanel();
		this.LogoPictureBox = new System.Windows.Forms.PictureBox();
		this.LabelProductName = new System.Windows.Forms.Label();
		this.LabelVersion = new System.Windows.Forms.Label();
		this.LabelCopyright = new System.Windows.Forms.Label();
		this.LabelCompanyName = new System.Windows.Forms.Label();
		this.TextBoxDescription = new System.Windows.Forms.TextBox();
		this.OKButton = new System.Windows.Forms.Button();
		this.TableLayoutPanel.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.LogoPictureBox).BeginInit();
		this.SuspendLayout();
		this.TableLayoutPanel.ColumnCount = 2;
		this.TableLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 33f));
		this.TableLayoutPanel.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 67f));
		this.TableLayoutPanel.Controls.Add(this.LogoPictureBox, 0, 0);
		this.TableLayoutPanel.Controls.Add(this.LabelProductName, 1, 0);
		this.TableLayoutPanel.Controls.Add(this.LabelVersion, 1, 1);
		this.TableLayoutPanel.Controls.Add(this.LabelCopyright, 1, 2);
		this.TableLayoutPanel.Controls.Add(this.LabelCompanyName, 1, 3);
		this.TableLayoutPanel.Controls.Add(this.TextBoxDescription, 1, 4);
		this.TableLayoutPanel.Controls.Add(this.OKButton, 1, 5);
		this.TableLayoutPanel.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.TableLayoutPanel tableLayoutPanel = this.TableLayoutPanel;
		System.Drawing.Point location = new System.Drawing.Point(10, 11);
		tableLayoutPanel.Location = location;
		System.Windows.Forms.TableLayoutPanel tableLayoutPanel2 = this.TableLayoutPanel;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		tableLayoutPanel2.Margin = margin;
		this.TableLayoutPanel.Name = "TableLayoutPanel";
		this.TableLayoutPanel.RowCount = 6;
		this.TableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10f));
		this.TableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10f));
		this.TableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10f));
		this.TableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10f));
		this.TableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 50f));
		this.TableLayoutPanel.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 10f));
		System.Windows.Forms.TableLayoutPanel tableLayoutPanel3 = this.TableLayoutPanel;
		System.Drawing.Size size = new System.Drawing.Size(463, 281);
		tableLayoutPanel3.Size = size;
		this.TableLayoutPanel.TabIndex = 0;
		this.LogoPictureBox.Dock = System.Windows.Forms.DockStyle.Fill;
		this.LogoPictureBox.Image = (System.Drawing.Image)resources.GetObject("LogoPictureBox.Image");
		System.Windows.Forms.PictureBox logoPictureBox = this.LogoPictureBox;
		location = new System.Drawing.Point(3, 4);
		logoPictureBox.Location = location;
		System.Windows.Forms.PictureBox logoPictureBox2 = this.LogoPictureBox;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		logoPictureBox2.Margin = margin;
		this.LogoPictureBox.Name = "LogoPictureBox";
		this.TableLayoutPanel.SetRowSpan(this.LogoPictureBox, 6);
		System.Windows.Forms.PictureBox logoPictureBox3 = this.LogoPictureBox;
		size = new System.Drawing.Size(146, 273);
		logoPictureBox3.Size = size;
		this.LogoPictureBox.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
		this.LogoPictureBox.TabIndex = 0;
		this.LogoPictureBox.TabStop = false;
		this.LabelProductName.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.Label labelProductName = this.LabelProductName;
		location = new System.Drawing.Point(159, 0);
		labelProductName.Location = location;
		System.Windows.Forms.Label labelProductName2 = this.LabelProductName;
		margin = new System.Windows.Forms.Padding(7, 0, 3, 0);
		labelProductName2.Margin = margin;
		System.Windows.Forms.Label labelProductName3 = this.LabelProductName;
		size = new System.Drawing.Size(0, 21);
		labelProductName3.MaximumSize = size;
		this.LabelProductName.Name = "LabelProductName";
		System.Windows.Forms.Label labelProductName4 = this.LabelProductName;
		size = new System.Drawing.Size(301, 21);
		labelProductName4.Size = size;
		this.LabelProductName.TabIndex = 0;
		this.LabelProductName.Text = "KP HOTEL";
		this.LabelProductName.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.LabelVersion.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.Label labelVersion = this.LabelVersion;
		location = new System.Drawing.Point(159, 28);
		labelVersion.Location = location;
		System.Windows.Forms.Label labelVersion2 = this.LabelVersion;
		margin = new System.Windows.Forms.Padding(7, 0, 3, 0);
		labelVersion2.Margin = margin;
		System.Windows.Forms.Label labelVersion3 = this.LabelVersion;
		size = new System.Drawing.Size(0, 21);
		labelVersion3.MaximumSize = size;
		this.LabelVersion.Name = "LabelVersion";
		System.Windows.Forms.Label labelVersion4 = this.LabelVersion;
		size = new System.Drawing.Size(301, 21);
		labelVersion4.Size = size;
		this.LabelVersion.TabIndex = 0;
		this.LabelVersion.Text = "Version  1 build 3";
		this.LabelVersion.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.LabelCopyright.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.Label labelCopyright = this.LabelCopyright;
		location = new System.Drawing.Point(159, 56);
		labelCopyright.Location = location;
		System.Windows.Forms.Label labelCopyright2 = this.LabelCopyright;
		margin = new System.Windows.Forms.Padding(7, 0, 3, 0);
		labelCopyright2.Margin = margin;
		System.Windows.Forms.Label labelCopyright3 = this.LabelCopyright;
		size = new System.Drawing.Size(0, 21);
		labelCopyright3.MaximumSize = size;
		this.LabelCopyright.Name = "LabelCopyright";
		System.Windows.Forms.Label labelCopyright4 = this.LabelCopyright;
		size = new System.Drawing.Size(301, 21);
		labelCopyright4.Size = size;
		this.LabelCopyright.TabIndex = 0;
		this.LabelCopyright.Text = "Copyright© 2015";
		this.LabelCopyright.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.LabelCompanyName.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.Label labelCompanyName = this.LabelCompanyName;
		location = new System.Drawing.Point(159, 84);
		labelCompanyName.Location = location;
		System.Windows.Forms.Label labelCompanyName2 = this.LabelCompanyName;
		margin = new System.Windows.Forms.Padding(7, 0, 3, 0);
		labelCompanyName2.Margin = margin;
		System.Windows.Forms.Label labelCompanyName3 = this.LabelCompanyName;
		size = new System.Drawing.Size(0, 21);
		labelCompanyName3.MaximumSize = size;
		this.LabelCompanyName.Name = "LabelCompanyName";
		System.Windows.Forms.Label labelCompanyName4 = this.LabelCompanyName;
		size = new System.Drawing.Size(301, 21);
		labelCompanyName4.Size = size;
		this.LabelCompanyName.TabIndex = 0;
		this.LabelCompanyName.Text = "PiPE Software";
		this.LabelCompanyName.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.TextBoxDescription.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.TextBox textBoxDescription = this.TextBoxDescription;
		location = new System.Drawing.Point(159, 116);
		textBoxDescription.Location = location;
		System.Windows.Forms.TextBox textBoxDescription2 = this.TextBoxDescription;
		margin = new System.Windows.Forms.Padding(7, 4, 3, 4);
		textBoxDescription2.Margin = margin;
		this.TextBoxDescription.Multiline = true;
		this.TextBoxDescription.Name = "TextBoxDescription";
		this.TextBoxDescription.ReadOnly = true;
		this.TextBoxDescription.ScrollBars = System.Windows.Forms.ScrollBars.Both;
		System.Windows.Forms.TextBox textBoxDescription3 = this.TextBoxDescription;
		size = new System.Drawing.Size(301, 132);
		textBoxDescription3.Size = size;
		this.TextBoxDescription.TabIndex = 0;
		this.TextBoxDescription.TabStop = false;
		this.TextBoxDescription.Text = "Description:\r\n\r\nต\u0e34ดต\u0e48อ : บร\u0e34ษ\u0e31ท เคพ\u0e35ซ\u0e34สเต\u0e47มแอนด\u0e4cเน\u0e47ตเว\u0e34ร\u0e4cค จำก\u0e31ด (ปร\u0e31ชญา เข\u0e47มม\u0e38ข)\r\nEmail : spicekung@hotmail.com\r\nTel : 086-7388335\r\nhttp://www.kpsystem.co.th\r\n";
		this.OKButton.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.OKButton.BackColor = System.Drawing.SystemColors.Control;
		this.OKButton.DialogResult = System.Windows.Forms.DialogResult.Cancel;
		System.Windows.Forms.Button oKButton = this.OKButton;
		location = new System.Drawing.Point(373, 256);
		oKButton.Location = location;
		System.Windows.Forms.Button oKButton2 = this.OKButton;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		oKButton2.Margin = margin;
		this.OKButton.Name = "OKButton";
		System.Windows.Forms.Button oKButton3 = this.OKButton;
		size = new System.Drawing.Size(87, 21);
		oKButton3.Size = size;
		this.OKButton.TabIndex = 0;
		this.OKButton.Text = "&OK";
		this.OKButton.UseVisualStyleBackColor = false;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.CancelButton = this.OKButton;
		size = new System.Drawing.Size(483, 303);
		this.ClientSize = size;
		this.Controls.Add(this.TableLayoutPanel);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedDialog;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "AboutBox1";
		margin = new System.Windows.Forms.Padding(10, 11, 10, 11);
		this.Padding = margin;
		this.ShowInTaskbar = false;
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "เก\u0e35\u0e48ยวก\u0e31บโปรแกรม";
		this.TableLayoutPanel.ResumeLayout(false);
		this.TableLayoutPanel.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.LogoPictureBox).EndInit();
		this.ResumeLayout(false);
	}

	private void AboutBox1_Load(object sender, EventArgs e)
	{
		string arg = ((Operators.CompareString(MyProject.Application.Info.Title, "", TextCompare: false) == 0) ? Path.GetFileNameWithoutExtension(MyProject.Application.Info.AssemblyName) : MyProject.Application.Info.Title);
		Text = $"About {arg}";
		LabelVersion.Text = Module1.ProgramVersion;
		LabelCopyright.Text = MyProject.Application.Info.Copyright;
		LabelCompanyName.Text = MyProject.Application.Info.CompanyName;
	}

	private void OKButton_Click(object sender, EventArgs e)
	{
		Close();
	}
}
