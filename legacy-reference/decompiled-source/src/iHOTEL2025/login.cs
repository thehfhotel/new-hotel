using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using DevComponents.DotNetBar.Validator;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class login : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Pass")]
	private TextBox _Pass;

	[AccessedThroughProperty("User")]
	private TextBox _User;

	[AccessedThroughProperty("PasswordLabel")]
	private Label _PasswordLabel;

	[AccessedThroughProperty("UsernameLabel")]
	private Label _UsernameLabel;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ReflectionImage1")]
	private ReflectionImage _ReflectionImage1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("SuperValidator1")]
	private SuperValidator _SuperValidator1;

	[AccessedThroughProperty("RequiredFieldValidator3")]
	private RequiredFieldValidator _RequiredFieldValidator3;

	[AccessedThroughProperty("ErrorProvider1")]
	private ErrorProvider _ErrorProvider1;

	[AccessedThroughProperty("Highlighter1")]
	private Highlighter _Highlighter1;

	[AccessedThroughProperty("RequiredFieldValidator4")]
	private RequiredFieldValidator _RequiredFieldValidator4;

	[AccessedThroughProperty("RequiredFieldValidator1")]
	private RequiredFieldValidator _RequiredFieldValidator1;

	[AccessedThroughProperty("RequiredFieldValidator2")]
	private RequiredFieldValidator _RequiredFieldValidator2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	private ArrayList DIsPer;

	public bool ISOK;

	internal virtual TextBox Pass
	{
		[DebuggerNonUserCode]
		get
		{
			return _Pass;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Pass = value;
		}
	}

	internal virtual TextBox User
	{
		[DebuggerNonUserCode]
		get
		{
			return _User;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_User = value;
		}
	}

	internal virtual Label PasswordLabel
	{
		[DebuggerNonUserCode]
		get
		{
			return _PasswordLabel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PasswordLabel = value;
		}
	}

	internal virtual Label UsernameLabel
	{
		[DebuggerNonUserCode]
		get
		{
			return _UsernameLabel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_UsernameLabel = value;
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

	internal virtual ReflectionImage ReflectionImage1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ReflectionImage1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ReflectionImage1 = value;
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

	internal virtual SuperValidator SuperValidator1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SuperValidator1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SuperValidator1 = value;
		}
	}

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

	internal virtual ErrorProvider ErrorProvider1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ErrorProvider1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ErrorProvider1 = value;
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
	static login()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public login()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += login_FormClosing;
		base.Load += login_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		DIsPer = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.login));
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Pass = new System.Windows.Forms.TextBox();
		this.User = new System.Windows.Forms.TextBox();
		this.PasswordLabel = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.UsernameLabel = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ReflectionImage1 = new DevComponents.DotNetBar.Controls.ReflectionImage();
		this.SuperValidator1 = new DevComponents.DotNetBar.Validator.SuperValidator();
		this.RequiredFieldValidator4 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณาใส\u0e48 Password");
		this.RequiredFieldValidator3 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("กร\u0e38ณาใส\u0e48 UserName");
		this.ErrorProvider1 = new System.Windows.Forms.ErrorProvider(this.components);
		this.Highlighter1 = new DevComponents.DotNetBar.Validator.Highlighter();
		this.RequiredFieldValidator1 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.RequiredFieldValidator2 = new DevComponents.DotNetBar.Validator.RequiredFieldValidator("Your error message here.");
		this.PanelEx2.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.ErrorProvider1).BeginInit();
		this.SuspendLayout();
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		System.Drawing.Point location = new System.Drawing.Point(318, 151);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		System.Drawing.Size size = new System.Drawing.Size(110, 31);
		buttonX3.Size = size;
		this.ButtonX2.TabIndex = 3;
		this.ButtonX2.Text = "Cancel";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Cursor = System.Windows.Forms.Cursors.Hand;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		location = new System.Drawing.Point(191, 151);
		buttonX4.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX5.Margin = margin;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX1;
		size = new System.Drawing.Size(120, 31);
		buttonX6.Size = size;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "Login..";
		this.Highlighter1.SetHighlightOnFocus(this.Pass, true);
		System.Windows.Forms.TextBox pass = this.Pass;
		location = new System.Drawing.Point(190, 117);
		pass.Location = location;
		System.Windows.Forms.TextBox pass2 = this.Pass;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		pass2.Margin = margin;
		this.Pass.Name = "Pass";
		this.Pass.PasswordChar = '*';
		System.Windows.Forms.TextBox pass3 = this.Pass;
		size = new System.Drawing.Size(240, 23);
		pass3.Size = size;
		this.Pass.TabIndex = 1;
		this.SuperValidator1.SetValidator1(this.Pass, this.RequiredFieldValidator4);
		this.Highlighter1.SetHighlightOnFocus(this.User, true);
		System.Windows.Forms.TextBox user = this.User;
		location = new System.Drawing.Point(190, 58);
		user.Location = location;
		System.Windows.Forms.TextBox user2 = this.User;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		user2.Margin = margin;
		this.User.Name = "User";
		System.Windows.Forms.TextBox user3 = this.User;
		size = new System.Drawing.Size(240, 23);
		user3.Size = size;
		this.User.TabIndex = 0;
		this.SuperValidator1.SetValidator1(this.User, this.RequiredFieldValidator3);
		System.Windows.Forms.Label passwordLabel = this.PasswordLabel;
		location = new System.Drawing.Point(188, 92);
		passwordLabel.Location = location;
		this.PasswordLabel.Name = "PasswordLabel";
		System.Windows.Forms.Label passwordLabel2 = this.PasswordLabel;
		size = new System.Drawing.Size(257, 28);
		passwordLabel2.Size = size;
		this.PasswordLabel.TabIndex = 16;
		this.PasswordLabel.Text = "&Password";
		this.PasswordLabel.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold | System.Drawing.FontStyle.Underline, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.RoyalBlue;
		this.Label1.ImageAlign = System.Drawing.ContentAlignment.TopRight;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(155, 6);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(404, 28);
		label2.Size = size;
		this.Label1.TabIndex = 14;
		this.Label1.Text = "&User name";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.TopRight;
		System.Windows.Forms.Label usernameLabel = this.UsernameLabel;
		location = new System.Drawing.Point(188, 28);
		usernameLabel.Location = location;
		this.UsernameLabel.Name = "UsernameLabel";
		System.Windows.Forms.Label usernameLabel2 = this.UsernameLabel;
		size = new System.Drawing.Size(257, 28);
		usernameLabel2.Size = size;
		this.UsernameLabel.TabIndex = 14;
		this.UsernameLabel.Text = "&User name";
		this.UsernameLabel.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Timer1.Enabled = true;
		this.Timer1.Interval = 200;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.CheckBox1);
		this.PanelEx2.Controls.Add(this.ButtonX4);
		this.PanelEx2.Controls.Add(this.ButtonX3);
		this.PanelEx2.Controls.Add(this.ReflectionImage1);
		this.PanelEx2.Controls.Add(this.ButtonX2);
		this.PanelEx2.Controls.Add(this.UsernameLabel);
		this.PanelEx2.Controls.Add(this.ButtonX1);
		this.PanelEx2.Controls.Add(this.Label1);
		this.PanelEx2.Controls.Add(this.Pass);
		this.PanelEx2.Controls.Add(this.PasswordLabel);
		this.PanelEx2.Controls.Add(this.User);
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		size = new System.Drawing.Size(571, 240);
		panelEx3.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 1;
		this.CheckBox1.AutoSize = true;
		this.CheckBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(438, 118);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(104, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 20;
		this.CheckBox1.Text = "บ\u0e31นท\u0e36กรห\u0e31สผ\u0e48าน";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX4;
		location = new System.Drawing.Point(318, 191);
		buttonX7.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX4;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX8.Margin = margin;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX4;
		size = new System.Drawing.Size(110, 32);
		buttonX9.Size = size;
		this.ButtonX4.TabIndex = 19;
		this.ButtonX4.Text = "โหมด ห\u0e49องอาหาร";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX3;
		location = new System.Drawing.Point(191, 191);
		buttonX10.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX3;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX11.Margin = margin;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX3;
		size = new System.Drawing.Size(120, 32);
		buttonX12.Size = size;
		this.ButtonX3.TabIndex = 18;
		this.ButtonX3.Text = "โหมด แม\u0e49บ\u0e49าน";
		this.ReflectionImage1.BackgroundStyle.Class = "";
		this.ReflectionImage1.BackgroundStyle.TextAlignment = DevComponents.DotNetBar.eStyleTextAlignment.Center;
		this.ReflectionImage1.Image = (System.Drawing.Image)resources.GetObject("ReflectionImage1.Image");
		DevComponents.DotNetBar.Controls.ReflectionImage reflectionImage = this.ReflectionImage1;
		location = new System.Drawing.Point(15, 7);
		reflectionImage.Location = location;
		DevComponents.DotNetBar.Controls.ReflectionImage reflectionImage2 = this.ReflectionImage1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		reflectionImage2.Margin = margin;
		this.ReflectionImage1.Name = "ReflectionImage1";
		DevComponents.DotNetBar.Controls.ReflectionImage reflectionImage3 = this.ReflectionImage1;
		size = new System.Drawing.Size(149, 242);
		reflectionImage3.Size = size;
		this.ReflectionImage1.TabIndex = 17;
		this.SuperValidator1.ContainerControl = this;
		this.SuperValidator1.ErrorProvider = this.ErrorProvider1;
		this.SuperValidator1.Highlighter = this.Highlighter1;
		this.RequiredFieldValidator4.ErrorMessage = "กร\u0e38ณาใส\u0e48 Password";
		this.RequiredFieldValidator4.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator3.ErrorMessage = "กร\u0e38ณาใส\u0e48 UserName";
		this.RequiredFieldValidator3.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.ErrorProvider1.ContainerControl = this;
		this.ErrorProvider1.Icon = (System.Drawing.Icon)resources.GetObject("ErrorProvider1.Icon");
		this.Highlighter1.ContainerControl = this;
		this.Highlighter1.FocusHighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Orange;
		this.RequiredFieldValidator1.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator1.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		this.RequiredFieldValidator2.ErrorMessage = "Your error message here.";
		this.RequiredFieldValidator2.HighlightColor = DevComponents.DotNetBar.Validator.eHighlightColor.Red;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BottomLeftCornerSize = 0;
		this.BottomRightCornerSize = 0;
		size = new System.Drawing.Size(571, 240);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.Name = "login";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เข\u0e49าส\u0e39\u0e48ระบบ";
		this.TopLeftCornerSize = 0;
		this.TopMost = true;
		this.TopRightCornerSize = 0;
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx2.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.ErrorProvider1).EndInit();
		this.ResumeLayout(false);
	}

	public void load_password()
	{
		CheckBox1.Checked = false;
		if (File.Exists(Module1.PathF + "svlgn.txt"))
		{
			StreamReader streamReader = new StreamReader(Module1.PathF + "\\svlgn.txt", Encoding.Default);
			string[] array = default(string[]);
			while (streamReader.Peek() != -1)
			{
				string encryptedString = streamReader.ReadLine();
				array = FormEN_DE.Decrypt1(encryptedString, "login54586").Split('|');
			}
			streamReader.Close();
			streamReader = null;
			try
			{
				User.Text = Strings.Trim(array[0]);
				Pass.Text = Strings.Trim(array[1]);
				CheckBox1.Checked = true;
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void login_FormClosing(object sender, FormClosingEventArgs e)
	{
		MyProject.Forms.frmMain1.TimerMouse.Enabled = true;
	}

	private void login_Load(object sender, EventArgs e)
	{
		MyProject.Forms.frmMain1.TimerMouse.Enabled = false;
		DataSet dataSet = Module1.connect("select * from TB_MRP_EMPLOYEE where emp_username='admin' and emp_password='admin'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			User.Text = "";
			Pass.Text = "";
		}
		EN(EE: false);
		Module1.KichenMode = false;
		Module1.HouseWifeMode = false;
		DataSet dataSet2 = Module1.connect("select * from TB_SETTINGS");
		try
		{
			Text = Conversions.ToString(dataSet2.Tables[0].Rows[0]["CompanyName"]);
			MyProject.Forms.frmMain1.Text = Conversions.ToString(dataSet2.Tables[0].Rows[0]["CompanyName"]);
			Module1.Company_Name = (string)RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["CompanyName"]);
			Module1.Company_Head = (string)RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[0]["CompanyName"]);
			Module1.decimal_1 = new decimal(Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["Room_Clean_Time"]));
			Module1.VAT_OUT = dataSet2.Tables[0].Rows[0]["VAT_OUT"].ToString();
			if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["Company_address"], "", TextCompare: false))
			{
				Module1.Company_Head = (string)Operators.ConcatenateObject(Module1.Company_Head, Operators.ConcatenateObject(Operators.ConcatenateObject("\r\n", dataSet2.Tables[0].Rows[0]["Company_address"]), "\r\n"));
			}
			if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["Company_tel"], "", TextCompare: false))
			{
				Module1.Company_Head = (string)Operators.ConcatenateObject(Module1.Company_Head, Operators.ConcatenateObject("Tel. ", dataSet2.Tables[0].Rows[0]["Company_tel"]));
			}
			if (Operators.ConditionalCompareObjectNotEqual(dataSet2.Tables[0].Rows[0]["Company_Fax"], "", TextCompare: false))
			{
				Module1.Company_Head = (string)Operators.ConcatenateObject(Module1.Company_Head, Operators.ConcatenateObject(" Fax. ", dataSet2.Tables[0].Rows[0]["Company_Fax"]));
			}
			Module1.AutoLogout = Conversions.ToInteger(Operators.MultiplyObject(dataSet2.Tables[0].Rows[0]["Time_Logout"], 60));
			MyProject.Forms.frmMain1.Label2.Text = Conversions.ToString(Module1.Company_Head);
			byte[] buffer = (byte[])dataSet2.Tables[0].Rows[0]["Company_Image"];
			Module1.loginURLsplit(dataSet2.Tables[0].Rows[0]["login_url"].ToString());
			Module1.CHK_IN_Before = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["CHK_IN_Before"].ToString());
			Module1.CHK_Out = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["CHK_Out"].ToString());
			Module1.CHK_Out_Alert = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["CHK_Out_Alert"].ToString());
			Module1.CHK_Out_Before = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["CHK_Out_Before"].ToString());
			Module1.CHK_Out_H_price = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["CHK_Out_H_price"].ToString());
			Module1.Maximum_Book = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["Maximum_Book"].ToString());
			Module1.Pority = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["Pority"].ToString());
			Module1.Vat_Over = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Vat_Over"].ToString());
			Module1.SHOW_ICON = Conversions.ToBoolean(dataSet2.Tables[0].Rows[0]["SHOW_ICON"]);
			Module1.bool_3 = Conversions.ToBoolean(dataSet2.Tables[0].Rows[0]["AUTO_CUT_POWER"]);
			Module1.decimal_0 = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Min_HOURS"]);
			Module1.MANUAL_POWER = Conversions.ToBoolean(dataSet2.Tables[0].Rows[0]["MANUAL_POWER"]);
			Module1.POWER_Delay = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["POWER_Delay"]);
			if (Operators.CompareString(dataSet2.Tables[0].Rows[0]["Cal_Pority_Cust"].ToString(), "เป\u0e34ด", TextCompare: false) == 0)
			{
				Module1.AutoCalCust = true;
			}
			else
			{
				Module1.AutoCalCust = false;
			}
			Bitmap image = new Bitmap(new MemoryStream(buffer));
			ReflectionImage1.Image = image;
			Label1.Text = "Version : " + Module1.ProgramVersion;
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		load_password();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		ISOK = false;
		Close();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		if (!SuperValidator1.Validate())
		{
			return;
		}
		DataSet dataSet = Module1.connect("select * from TB_MRP_EMPLOYEE where emp_username='" + User.Text + "' and emp_password='" + Pass.Text + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("Username/Password ไม\u0e48ถ\u0e39กต\u0e49อง");
			Module1.LOG("Login ผ\u0e34ด  USER:" + User.Text);
			return;
		}
		Module1.loginID = (string)RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["id"]);
		Module1.loginName = (string)RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["emp_name"]);
		Module1.loginMode = dataSet.Tables[0].Rows[0]["emp_level"].ToString().ToUpper();
		if (CheckBox1.Checked)
		{
			save_login(User.Text, Pass.Text);
		}
		else if (File.Exists(Module1.PathF + "\\svlgn.txt"))
		{
			try
			{
				File.Delete(Module1.PathF + "\\svlgn.txt");
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
		checked
		{
			if (Operators.CompareString(Module1.loginMode.ToString().ToLower(), "admin", TextCompare: false) != 0)
			{
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select Level_Command from TB_MRP_Permission where  level_name='", Module1.loginMode), "'")));
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
					DIsPer.Add(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["Level_Command"]));
					num2++;
				}
				LoopSET();
			}
			else
			{
				EN(EE: true);
			}
			MyProject.Forms.frmMain1.Text = Conversions.ToString(Operators.ConcatenateObject(string.Concat(string.Concat(string.Concat(MyProject.Forms.frmMain1.Text + " (", "Version : "), Module1.ProgramVersion), ") เข\u0e49าส\u0e39\u0e48ระบบโดย "), Module1.loginName));
			Module1.LOG(Conversions.ToString(Operators.ConcatenateObject("Login สำเร\u0e47จ : ", Module1.loginName)));
			ISOK = true;
			Close();
		}
	}

	public void save_login(string user, string pwd)
	{
		try
		{
			StreamWriter streamWriter = new StreamWriter(Module1.PathF + "svlgn.txt");
			streamWriter.Write(FormEN_DE.Encrypt1(user + "|" + pwd, "login54586"));
			streamWriter.Close();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public void EN(bool EE)
	{
		MyProject.Forms.frmMain1.B1.Enabled = EE;
		MyProject.Forms.frmMain1.B2.Enabled = EE;
		MyProject.Forms.frmMain1.B3.Enabled = EE;
		MyProject.Forms.frmMain1.B4.Enabled = EE;
		MyProject.Forms.frmMain1.B5.Enabled = EE;
		MyProject.Forms.frmMain1.B6.Enabled = EE;
		MyProject.Forms.frmMain1.B7.Enabled = EE;
		MyProject.Forms.frmMain1.B8.Enabled = EE;
		MyProject.Forms.frmMain1.B9.Enabled = EE;
		MyProject.Forms.frmMain1.B10.Enabled = EE;
		MyProject.Forms.frmMain1.B11.Enabled = EE;
		MyProject.Forms.frmMain1.B11_2.Enabled = EE;
		MyProject.Forms.frmMain1.B12.Enabled = EE;
		MyProject.Forms.frmMain1.ButtonItem6.Enabled = EE;
		MyProject.Forms.frmMain1.B13.Enabled = EE;
		MyProject.Forms.frmMain1.B14.Enabled = EE;
		MyProject.Forms.frmMain1.B15.Enabled = EE;
		MyProject.Forms.frmMain1.B16.Enabled = EE;
		MyProject.Forms.frmMain1.B17.Enabled = EE;
		MyProject.Forms.frmMain1.B18.Enabled = EE;
		MyProject.Forms.frmMain1.B19.Enabled = EE;
		MyProject.Forms.frmMain1.B20.Enabled = EE;
		MyProject.Forms.frmMain1.B21.Enabled = EE;
		MyProject.Forms.frmMain1.B22.Enabled = EE;
		MyProject.Forms.frmMain1.ButtonItem23.Enabled = EE;
		MyProject.Forms.frmMain1.B23.Enabled = EE;
		MyProject.Forms.frmMain1.B24.Enabled = EE;
		MyProject.Forms.frmMain1.R1.Enabled = EE;
		MyProject.Forms.frmMain1.R2.Enabled = EE;
		MyProject.Forms.frmMain1.R3.Enabled = EE;
		MyProject.Forms.frmMain1.R4.Enabled = EE;
		MyProject.Forms.frmMain1.R5.Enabled = EE;
		MyProject.Forms.frmMain1.R6.Enabled = EE;
		MyProject.Forms.frmMain1.R7.Enabled = EE;
		MyProject.Forms.frmMain1.R8.Enabled = EE;
		MyProject.Forms.frmMain1.R9.Enabled = EE;
		MyProject.Forms.frmMain1.R10.Enabled = EE;
		MyProject.Forms.frmMain1.R11.Enabled = EE;
		MyProject.Forms.frmMain1.R12.Enabled = EE;
		MyProject.Forms.frmMain1.R13.Enabled = EE;
		MyProject.Forms.frmMain1.R14.Enabled = EE;
		MyProject.Forms.frmMain1.R15.Enabled = EE;
		MyProject.Forms.frmMain1.R16.Enabled = EE;
		MyProject.Forms.frmMain1.R17.Enabled = EE;
		MyProject.Forms.frmMain1.R18.Enabled = EE;
		MyProject.Forms.frmMain1.R19.Enabled = EE;
		MyProject.Forms.frmMain1.R20.Enabled = EE;
		Module1.bool_0 = true;
	}

	public void LoopSET()
	{
		Module1.bool_0 = false;
		if (Operators.CompareString(Module1.loginMode.ToString().ToLower(), "admin", TextCompare: false) == 0)
		{
			return;
		}
		checked
		{
			int num = DIsPer.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					SETPER(Conversions.ToString(DIsPer[num2]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void SETPER(string name)
	{
		switch (name)
		{
		case "สถานะห\u0e49องพ\u0e31ก":
			MyProject.Forms.frmMain1.B1.Enabled = true;
			break;
		case "Check-In":
			MyProject.Forms.frmMain1.B2.Enabled = true;
			break;
		case "Check-Out":
			MyProject.Forms.frmMain1.B3.Enabled = true;
			break;
		case "รายการจอง":
			MyProject.Forms.frmMain1.B4.Enabled = true;
			break;
		case "ขายส\u0e34นค\u0e49า":
			MyProject.Forms.frmMain1.B5.Enabled = true;
			break;
		case "รายการ Check-In/Check-Out":
			MyProject.Forms.frmMain1.B6.Enabled = true;
			break;
		case "ใบลงทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก":
			MyProject.Forms.frmMain1.B7.Enabled = true;
			break;
		case "ใบม\u0e31ดจำ":
			MyProject.Forms.frmMain1.B8.Enabled = true;
			break;
		case "ใบเสร\u0e47จร\u0e31บเง\u0e34น":
			MyProject.Forms.frmMain1.B9.Enabled = true;
			break;
		case "ใบกำก\u0e31บภาษ\u0e35":
			MyProject.Forms.frmMain1.B10.Enabled = true;
			break;
		case "ชำระเง\u0e34น/ล\u0e39กหน\u0e35\u0e49":
			MyProject.Forms.frmMain1.B11.Enabled = true;
			MyProject.Forms.frmMain1.B11_2.Enabled = true;
			break;
		case "จ\u0e31ดการรอบบ\u0e34ล":
			MyProject.Forms.frmMain1.B12.Enabled = true;
			break;
		case "ค\u0e39ปอง":
			MyProject.Forms.frmMain1.ButtonItem6.Enabled = true;
			break;
		case "จ\u0e31ดการผ\u0e39\u0e49ใช\u0e49งาน":
			MyProject.Forms.frmMain1.B13.Enabled = true;
			break;
		case "จ\u0e31ดการประเภทห\u0e49องพ\u0e31ก":
			MyProject.Forms.frmMain1.B14.Enabled = true;
			break;
		case "จ\u0e31ดการห\u0e49องพ\u0e31ก":
			MyProject.Forms.frmMain1.B15.Enabled = true;
			break;
		case "จ\u0e31ดการล\u0e39กค\u0e49า":
			MyProject.Forms.frmMain1.B16.Enabled = true;
			break;
		case "จ\u0e31ดการประเภทล\u0e39กค\u0e49า":
			MyProject.Forms.frmMain1.B17.Enabled = true;
			break;
		case "ต\u0e31\u0e49งค\u0e48ากล\u0e38\u0e48มราคา":
			MyProject.Forms.frmMain1.B18.Enabled = true;
			break;
		case "ต\u0e31\u0e49งค\u0e48าปร\u0e31บราคาลง":
			MyProject.Forms.frmMain1.B19.Enabled = true;
			break;
		case "ต\u0e31\u0e49งค\u0e48าปร\u0e31บราคาข\u0e36\u0e49น":
			MyProject.Forms.frmMain1.B20.Enabled = true;
			break;
		case "จ\u0e31ดการประเภทส\u0e34นค\u0e49า":
			MyProject.Forms.frmMain1.B21.Enabled = true;
			break;
		case "จ\u0e31ดการส\u0e34นค\u0e49า":
			MyProject.Forms.frmMain1.B22.Enabled = true;
			MyProject.Forms.frmMain1.ButtonItem23.Enabled = true;
			break;
		case "ต\u0e31\u0e49งค\u0e48าโปรแกรม":
			MyProject.Forms.frmMain1.B23.Enabled = true;
			break;
		case "อ\u0e31บเดทโปรแกรม":
			MyProject.Forms.frmMain1.B24.Enabled = true;
			break;
		case "รายงานสร\u0e38ปประจำว\u0e31น":
			MyProject.Forms.frmMain1.R1.Enabled = true;
			break;
		case "รายงานแขกเข\u0e49าพ\u0e31ก":
			MyProject.Forms.frmMain1.R2.Enabled = true;
			break;
		case "รายงานแขกออก":
			MyProject.Forms.frmMain1.R3.Enabled = true;
			break;
		case "รายงานแขกท\u0e35\u0e48อย\u0e39\u0e48ในโรงแรม":
			MyProject.Forms.frmMain1.R4.Enabled = true;
			break;
		case "รายงานการย\u0e49ายห\u0e49อง":
			MyProject.Forms.frmMain1.R5.Enabled = true;
			break;
		case "รายงานภาษ\u0e35ขาย":
			MyProject.Forms.frmMain1.R6.Enabled = true;
			break;
		case "รายงานล\u0e39กหน\u0e35\u0e49":
			MyProject.Forms.frmMain1.R7.Enabled = true;
			break;
		case "รายงานส\u0e34นค\u0e49า":
			MyProject.Forms.frmMain1.R8.Enabled = true;
			break;
		case "รายงานการขายส\u0e34นค\u0e49า":
			MyProject.Forms.frmMain1.R9.Enabled = true;
			break;
		case "รายงานการทำความสะอาด":
			MyProject.Forms.frmMain1.R10.Enabled = true;
			break;
		case "รายงานการจองห\u0e49องพ\u0e31ก":
			MyProject.Forms.frmMain1.R11.Enabled = true;
			break;
		case "รายงานการยกเล\u0e34กห\u0e49องพ\u0e31ก":
			MyProject.Forms.frmMain1.R12.Enabled = true;
			break;
		case "รายงานค\u0e39ปอง":
			MyProject.Forms.frmMain1.R13.Enabled = true;
			break;
		case "รายงานเง\u0e34นม\u0e31ดจำ":
			MyProject.Forms.frmMain1.R14.Enabled = true;
			break;
		case "รายงานสร\u0e38ปภาพรวม":
			MyProject.Forms.frmMain1.R15.Enabled = true;
			break;
		case "รายงานซ\u0e48อม":
			MyProject.Forms.frmMain1.R16.Enabled = true;
			break;
		case "รายงานระหว\u0e48างว\u0e31นท\u0e35\u0e48":
			MyProject.Forms.frmMain1.R17.Enabled = true;
			break;
		case "รายงานตามรอบบ\u0e34ล":
			MyProject.Forms.frmMain1.R18.Enabled = true;
			break;
		case "รายงานการขายห\u0e49อง":
			MyProject.Forms.frmMain1.R19.Enabled = true;
			break;
		case "รายงานเง\u0e34นสดคงเหล\u0e37อ":
			MyProject.Forms.frmMain1.R20.Enabled = true;
			break;
		case "ยกเล\u0e34กห\u0e49องพ\u0e31ก":
			Module1.bool_0 = true;
			break;
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		MyProject.Forms.frmMain1.Text = MyProject.Forms.frmMain1.Text + " (Version : " + Module1.ProgramVersion + ") โหมดแม\u0e48บ\u0e49าน ";
		Module1.HouseWifeMode = true;
		Module1.KichenMode = false;
		ISOK = true;
		Close();
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		MyProject.Forms.frmMain1.Text = MyProject.Forms.frmMain1.Text + " (Version : " + Module1.ProgramVersion + ") โหมดห\u0e49องอาหาร ";
		Module1.KichenMode = true;
		Module1.HouseWifeMode = false;
		ISOK = true;
		Close();
	}
}
