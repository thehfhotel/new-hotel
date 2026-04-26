using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSETelec : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("TextBox2")]
	private TextBox _TextBox2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	public bool isOK;

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

	internal virtual TextBox TextBox2
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

	[DebuggerNonUserCode]
	static FormSETelec()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSETelec()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormSETelec_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		isOK = false;
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
		this.Label1 = new System.Windows.Forms.Label();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.TextBox2 = new System.Windows.Forms.TextBox();
		this.Label2 = new System.Windows.Forms.Label();
		this.Button1 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(20, 15);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(100, 23);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "จากหน\u0e48วยท\u0e35\u0e48";
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		location = new System.Drawing.Point(122, 14);
		textBox.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		size = new System.Drawing.Size(100, 30);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 1;
		this.TextBox1.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		System.Windows.Forms.TextBox textBox3 = this.TextBox2;
		location = new System.Drawing.Point(326, 15);
		textBox3.Location = location;
		this.TextBox2.Name = "TextBox2";
		System.Windows.Forms.TextBox textBox4 = this.TextBox2;
		size = new System.Drawing.Size(100, 30);
		textBox4.Size = size;
		this.TextBox2.TabIndex = 3;
		this.TextBox2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(228, 16);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(89, 23);
		label4.Size = size;
		this.Label2.TabIndex = 2;
		this.Label2.Text = "ถ\u0e36งหน\u0e48วยท\u0e35\u0e48";
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(129, 70);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(96, 47);
		button2.Size = size;
		this.Button1.TabIndex = 4;
		this.Button1.Text = "ตกลง";
		this.Button1.UseVisualStyleBackColor = true;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(256, 70);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(96, 47);
		button4.Size = size;
		this.Button2.TabIndex = 5;
		this.Button2.Text = "ยกเล\u0e34ก";
		this.Button2.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(10f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(473, 132);
		this.ClientSize = size;
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.TextBox2);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 5, 5, 5);
		this.Margin = margin;
		this.Name = "FormSETelec";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "กร\u0e38ณาใส\u0e48หน\u0e48วยไฟฟ\u0e49า";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void FormSETelec_Load(object sender, EventArgs e)
	{
		TextBox1.Text = Conversions.ToString(0);
		TextBox2.Text = Conversions.ToString(0);
		isOK = false;
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขท\u0e35\u0e48ไฟให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		if (Operators.CompareString(TextBox2.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขท\u0e35\u0e48ไฟให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		if (!Versioned.IsNumeric(TextBox1.Text))
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขท\u0e35\u0e48ไฟให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		if (!Versioned.IsNumeric(TextBox2.Text))
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48เลขท\u0e35\u0e48ไฟให\u0e49ถ\u0e39กต\u0e49อง");
			return;
		}
		isOK = true;
		Close();
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}
}
