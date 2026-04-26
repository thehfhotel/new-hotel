using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class INV_Note : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("RichTextBox1")]
	private RichTextBox _RichTextBox1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	public string R_NO;

	public bool isok;

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
			_RichTextBox1 = value;
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

	[DebuggerNonUserCode]
	static INV_Note()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public INV_Note()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += Room_Note_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		R_NO = "";
		isok = false;
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
		this.RichTextBox1 = new System.Windows.Forms.RichTextBox();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(15, 9);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(157, 16);
		label3.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "กรอก หมายเหต\u0e38ใต\u0e49ใบแจ\u0e49งหน\u0e35\u0e49";
		System.Windows.Forms.RichTextBox richTextBox = this.RichTextBox1;
		location = new System.Drawing.Point(12, 32);
		richTextBox.Location = location;
		this.RichTextBox1.Name = "RichTextBox1";
		System.Windows.Forms.RichTextBox richTextBox2 = this.RichTextBox1;
		size = new System.Drawing.Size(433, 262);
		richTextBox2.Size = size;
		this.RichTextBox1.TabIndex = 1;
		this.RichTextBox1.Text = "";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(370, 304);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		size = new System.Drawing.Size(75, 23);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(289, 304);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(75, 23);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "ตกลง";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(457, 339);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.RichTextBox1);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "INV_Note";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "หมายเหต\u0e38";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Room_Note_Load(object sender, EventArgs e)
	{
		isok = false;
		DataSet dataSet = Module1.connect("select * from HT_Invoice_Note where Cin_no='" + R_NO + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			RichTextBox1.Text = "";
		}
		else
		{
			RichTextBox1.Text = dataSet.Tables[0].Rows[0]["note"].ToString();
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		isok = false;
		Close();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from HT_Invoice_Note where Cin_no='" + R_NO + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			Module1.connect("INSERT INTO HT_Invoice_Note VALUES ('" + R_NO + "','" + RichTextBox1.Text.Replace("'", "\"") + "')");
		}
		else
		{
			Module1.connect("update HT_Invoice_Note set note='" + RichTextBox1.Text.Replace("'", "\"") + "' where Cin_no='" + R_NO + "'");
		}
		isok = true;
		Close();
	}
}
