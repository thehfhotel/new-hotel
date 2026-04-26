using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class GENDB : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("TextBox2")]
	private TextBox _TextBox2;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

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
	static GENDB()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public GENDB()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += GENDB_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
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
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.TextBox2 = new System.Windows.Forms.TextBox();
		this.Button2 = new System.Windows.Forms.Button();
		this.SuspendLayout();
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		textBox.Location = location;
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(4, 4, 4, 4);
		textBox2.Margin = margin;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox3 = this.TextBox1;
		System.Drawing.Size size = new System.Drawing.Size(148, 27);
		textBox3.Size = size;
		this.TextBox1.TabIndex = 0;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(169, 13);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(75, 27);
		button2.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.Text = "ok";
		this.Button1.UseVisualStyleBackColor = true;
		System.Windows.Forms.TextBox textBox4 = this.TextBox2;
		location = new System.Drawing.Point(13, 48);
		textBox4.Location = location;
		System.Windows.Forms.TextBox textBox5 = this.TextBox2;
		margin = new System.Windows.Forms.Padding(4);
		textBox5.Margin = margin;
		this.TextBox2.Name = "TextBox2";
		System.Windows.Forms.TextBox textBox6 = this.TextBox2;
		size = new System.Drawing.Size(302, 27);
		textBox6.Size = size;
		this.TextBox2.TabIndex = 2;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(250, 13);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(65, 27);
		button4.Size = size;
		this.Button2.TabIndex = 3;
		this.Button2.Text = "Button2";
		this.Button2.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(9f, 19f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(337, 94);
		this.ClientSize = size;
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.TextBox2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.TextBox1);
		this.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(4, 4, 4, 4);
		this.Margin = margin;
		this.Name = "GENDB";
		this.Text = "GENDB";
		this.TopMost = true;
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void GENDB_Load(object sender, EventArgs e)
	{
		object left = Interaction.InputBox("กร\u0e38ณาใส\u0e48 Password");
		if (Operators.ConditionalCompareObjectNotEqual(left, "รห\u0e31สโรงแรม007", TextCompare: false))
		{
			Close();
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		TextBox2.Text = ":EG:" + Module1.Encrypt1(TextBox1.Text, "554683");
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		TextBox2.Text = Module1.Decrypt1(TextBox1.Text, "554683");
	}
}
