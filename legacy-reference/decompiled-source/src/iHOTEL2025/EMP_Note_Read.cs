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
public class EMP_Note_Read : Office2007Form
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

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("LabelX2")]
	private LabelX _LabelX2;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	public string R_NO;

	public bool ISOK;

	private int Nrows;

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

	internal virtual LabelX LabelX2
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX2 = value;
		}
	}

	internal virtual LabelX LabelX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX3 = value;
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

	[DebuggerNonUserCode]
	static EMP_Note_Read()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public EMP_Note_Read()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += Room_Note_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		R_NO = "";
		ISOK = false;
		Nrows = 1;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.EMP_Note_Read));
		this.Label1 = new System.Windows.Forms.Label();
		this.RichTextBox1 = new System.Windows.Forms.RichTextBox();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.LabelX2 = new DevComponents.DotNetBar.LabelX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.Label2 = new System.Windows.Forms.Label();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(8, 9);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(88, 16);
		label3.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ฝากข\u0e49อความถ\u0e36ง";
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
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
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
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX2;
		location = new System.Drawing.Point(289, 304);
		buttonX3.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX2;
		size = new System.Drawing.Size(75, 23);
		buttonX4.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "อ\u0e48านแล\u0e49ว";
		this.LabelX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LabelX1.BackColor = System.Drawing.Color.WhiteSmoke;
		this.LabelX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX = this.LabelX1;
		location = new System.Drawing.Point(41, 304);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		size = new System.Drawing.Size(20, 23);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 3;
		this.LabelX1.Text = "0";
		this.LabelX1.TextAlignment = System.Drawing.StringAlignment.Center;
		this.LabelX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LabelX2.BackColor = System.Drawing.Color.WhiteSmoke;
		this.LabelX2.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX2;
		location = new System.Drawing.Point(59, 304);
		labelX3.Location = location;
		this.LabelX2.Name = "LabelX2";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX2;
		size = new System.Drawing.Size(31, 23);
		labelX4.Size = size;
		this.LabelX2.TabIndex = 4;
		this.LabelX2.Text = "จาก";
		this.LabelX2.TextAlignment = System.Drawing.StringAlignment.Center;
		this.LabelX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LabelX3.BackColor = System.Drawing.Color.WhiteSmoke;
		this.LabelX3.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.LabelX labelX5 = this.LabelX3;
		location = new System.Drawing.Point(87, 304);
		labelX5.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX6 = this.LabelX3;
		size = new System.Drawing.Size(20, 23);
		labelX6.Size = size;
		this.LabelX3.TabIndex = 5;
		this.LabelX3.Text = "0";
		this.LabelX3.TextAlignment = System.Drawing.StringAlignment.Center;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(14, 304);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(26, 23);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 6;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX4;
		location = new System.Drawing.Point(107, 304);
		buttonX7.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX4;
		size = new System.Drawing.Size(26, 23);
		buttonX8.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 6;
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label4 = this.Label2;
		location = new System.Drawing.Point(104, 9);
		label4.Location = location;
		System.Windows.Forms.Label label5 = this.Label2;
		margin = new System.Windows.Forms.Padding(2, 0, 2, 0);
		label5.Margin = margin;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(88, 16);
		label6.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "ฝากข\u0e49อความถ\u0e36ง";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.White;
		size = new System.Drawing.Size(457, 339);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.LabelX3);
		this.Controls.Add(this.LabelX1);
		this.Controls.Add(this.LabelX2);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.RichTextBox1);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "EMP_Note_Read";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ฝากข\u0e49อความ";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Room_Note_Load(object sender, EventArgs e)
	{
		Nrows = 1;
		ISOK = false;
		DataSet dataSet = Module1.connect("select * from HT_EMP_SMS where SMS_TO='" + R_NO + "' and SMS_Readed='no' order by SMS_ID");
		RichTextBox1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["SMS_Details"]);
		LabelX1.Text = Conversions.ToString(Nrows);
		Label2.Text = R_NO;
		LabelX3.Text = Conversions.ToString(dataSet.Tables[0].Rows.Count);
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		ISOK = true;
		Module1.connect("update [HT_EMP_SMS] SET [SMS_Readed]='yes' where SMS_TO='" + R_NO + "' and SMS_Readed='no' ");
		Close();
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		checked
		{
			Nrows++;
			DataSet dataSet = Module1.connect("select * from HT_EMP_SMS where SMS_TO='" + R_NO + "' and SMS_Readed='no' order by SMS_ID");
			if (Nrows >= dataSet.Tables[0].Rows.Count)
			{
				RichTextBox1.Text = Conversions.ToString(dataSet.Tables[0].Rows[dataSet.Tables[0].Rows.Count - 1]["SMS_Details"]);
				Nrows = dataSet.Tables[0].Rows.Count;
			}
			else
			{
				RichTextBox1.Text = Conversions.ToString(dataSet.Tables[0].Rows[Nrows - 1]["SMS_Details"]);
			}
			LabelX1.Text = Conversions.ToString(Nrows);
			LabelX3.Text = Conversions.ToString(dataSet.Tables[0].Rows.Count);
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		checked
		{
			Nrows--;
			DataSet dataSet = Module1.connect("select * from HT_EMP_SMS where SMS_TO='" + R_NO + "' and SMS_Readed='no' order by SMS_ID");
			if (0 >= Nrows)
			{
				RichTextBox1.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["SMS_Details"]);
				Nrows = 1;
			}
			else
			{
				RichTextBox1.Text = Conversions.ToString(dataSet.Tables[0].Rows[Nrows - 1]["SMS_Details"]);
			}
			LabelX1.Text = Conversions.ToString(Nrows);
			LabelX3.Text = Conversions.ToString(dataSet.Tables[0].Rows.Count);
		}
	}
}
