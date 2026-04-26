using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormShowVAT : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label_NO")]
	private Label _Label_NO;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("LabelItem1")]
	private LabelItem _LabelItem1;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("LabelItem2")]
	private LabelItem _LabelItem2;

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

	internal virtual Label Label_NO
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label_NO;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label_NO = value;
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
			EventHandler value2 = ButtonX2_MouseHover;
			EventHandler value3 = ButtonX2_Click;
			if (_ButtonX2 != null)
			{
				_ButtonX2.MouseHover -= value2;
				_ButtonX2.Click -= value3;
			}
			_ButtonX2 = value;
			if (_ButtonX2 != null)
			{
				_ButtonX2.MouseHover += value2;
				_ButtonX2.Click += value3;
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
			EventHandler value2 = ButtonX1_MouseHover;
			EventHandler value3 = ButtonX1_Click;
			if (_ButtonX1 != null)
			{
				_ButtonX1.MouseHover -= value2;
				_ButtonX1.Click -= value3;
			}
			_ButtonX1 = value;
			if (_ButtonX1 != null)
			{
				_ButtonX1.MouseHover += value2;
				_ButtonX1.Click += value3;
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

	internal virtual LabelItem LabelItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelItem1 = value;
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
			EventHandler value2 = ButtonX4_MouseHover;
			if (_ButtonX4 != null)
			{
				_ButtonX4.MouseHover -= value2;
			}
			_ButtonX4 = value;
			if (_ButtonX4 != null)
			{
				_ButtonX4.MouseHover += value2;
			}
		}
	}

	internal virtual LabelItem LabelItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = LabelItem2_Click;
			if (_LabelItem2 != null)
			{
				_LabelItem2.Click -= value2;
			}
			_LabelItem2 = value;
			if (_LabelItem2 != null)
			{
				_LabelItem2.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FormShowVAT()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormShowVAT()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormShowVAT_Load;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormShowVAT));
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.LabelItem2 = new DevComponents.DotNetBar.LabelItem();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.LabelItem1 = new DevComponents.DotNetBar.LabelItem();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label_NO = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.ButtonX4);
		this.PanelEx1.Controls.Add(this.ButtonX3);
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.Label3);
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.Label_NO);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(636, 402);
		panelEx3.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX4;
		location = new System.Drawing.Point(321, 214);
		buttonX.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX4;
		size = new System.Drawing.Size(285, 96);
		buttonX2.Size = size;
		this.ButtonX4.SplitButton = true;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.LabelItem2 });
		this.ButtonX4.TabIndex = 7;
		this.ButtonX4.Text = "แก\u0e49ไข ใบกำก\u0e31บภาษ\u0e35\r\nท\u0e35\u0e48เคยออกไปแล\u0e49ว";
		this.LabelItem2.GlobalItem = false;
		this.LabelItem2.Name = "LabelItem2";
		this.LabelItem2.Text = "กร\u0e38ณาเล\u0e37อกเลขใบกำก\u0e31บภาษ\u0e35";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX3;
		location = new System.Drawing.Point(508, 324);
		buttonX3.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX3;
		size = new System.Drawing.Size(98, 49);
		buttonX4.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 6;
		this.ButtonX3.Text = "ป\u0e34ด";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX2;
		location = new System.Drawing.Point(22, 214);
		buttonX5.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX2;
		size = new System.Drawing.Size(293, 96);
		buttonX6.Size = size;
		this.ButtonX2.SplitButton = true;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.LabelItem1 });
		this.ButtonX2.TabIndex = 5;
		this.ButtonX2.Text = "พ\u0e34มพ\u0e4c ใบกำก\u0e31บภาษ\u0e35\r\nท\u0e35\u0e48เคยออกไปแล\u0e49ว";
		this.LabelItem1.GlobalItem = false;
		this.LabelItem1.Name = "LabelItem1";
		this.LabelItem1.Text = "กร\u0e38ณาเล\u0e37อกเลขใบกำก\u0e31บภาษ\u0e35";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX1;
		location = new System.Drawing.Point(22, 103);
		buttonX7.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX1;
		size = new System.Drawing.Size(584, 96);
		buttonX8.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 4;
		this.ButtonX1.Text = "ออกใบกำก\u0e31บภาษ\u0e35เพ\u0e34\u0e48ม\r\nยอดคงเหล\u0e37อ (200 บาท)";
		this.Label3.AutoSize = true;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label3.ForeColor = System.Drawing.Color.FromArgb(64, 64, 64);
		System.Windows.Forms.Label label = this.Label3;
		location = new System.Drawing.Point(18, 67);
		label.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label2 = this.Label3;
		size = new System.Drawing.Size(206, 19);
		label2.Size = size;
		this.Label3.TabIndex = 3;
		this.Label3.Text = "กร\u0e38ณาเล\u0e37อกตามเมน\u0e39ด\u0e49านล\u0e48าง";
		this.Label2.AutoSize = true;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(349, 16);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(257, 23);
		label4.Size = size;
		this.Label2.TabIndex = 2;
		this.Label2.Text = "ม\u0e35การออกใบกำก\u0e31บภาษ\u0e35ไปแล\u0e49ว";
		this.Label_NO.AutoSize = true;
		this.Label_NO.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label_NO.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label_NO = this.Label_NO;
		location = new System.Drawing.Point(208, 16);
		label_NO.Location = location;
		this.Label_NO.Name = "Label_NO";
		System.Windows.Forms.Label label_NO2 = this.Label_NO;
		size = new System.Drawing.Size(130, 23);
		label_NO2.Size = size;
		this.Label_NO.TabIndex = 1;
		this.Label_NO.Text = "CH-0000001";
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.SystemColors.HotTrack;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(18, 16);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(189, 23);
		label6.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "บ\u0e31ตรลงทะเบ\u0e35ยนเลขท\u0e35\u0e48 ";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(636, 402);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FormShowVAT";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ใบกำก\u0e31บภาษ\u0e35";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		ButtonX2.Expanded = true;
	}

	public void loadVAT()
	{
		checked
		{
			int num = ButtonX2.SubItems.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ButtonX2.SubItems.RemoveAt(1);
				num2++;
			}
			int num5 = ButtonX4.SubItems.Count - 1;
			int num6 = 1;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				ButtonX4.SubItems.RemoveAt(1);
				num6++;
			}
			DataSet dataSet = Module1.connect("select * from View_CheckIn_H where Cin_no='" + Label_NO.Text + "'");
			ButtonX1.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ออกใบกำก\u0e31บภาษ\u0e35เพ\u0e34\u0e48ม\r\nยอดคงเหล\u0e37อ (", dataSet.Tables[0].Rows[0]["EN_VAT"]), " บาท)"));
			DataSet dataSet2 = Module1.connect("select * from HT_Receipt_H where status_name<>'ยกเล\u0e34ก' and  Receipt_Ref='" + Label_NO.Text + "' order by id desc");
			int num8 = dataSet2.Tables[0].Rows.Count - 1;
			int num9 = 0;
			while (true)
			{
				int num10 = num9;
				int num4 = num8;
				if (num10 <= num4)
				{
					ButtonItem buttonItem = new ButtonItem();
					buttonItem.ForeColor = Color.Red;
					buttonItem.GlobalItem = false;
					buttonItem.Name = Conversions.ToString(dataSet2.Tables[0].Rows[num9]["id"]);
					buttonItem.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num9]["Receipt_no"], " ["), dataSet2.Tables[0].Rows[num9]["Receipt_Name"]), "]"));
					buttonItem.Click += ButtonSub_PRINT;
					ButtonX2.SubItems.Add(buttonItem);
					ButtonItem buttonItem2 = new ButtonItem();
					buttonItem2.ForeColor = Color.Red;
					buttonItem2.GlobalItem = false;
					buttonItem2.Name = Conversions.ToString(dataSet2.Tables[0].Rows[num9]["id"]);
					buttonItem2.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num9]["Receipt_no"], " ["), dataSet2.Tables[0].Rows[num9]["Receipt_Name"]), "]"));
					buttonItem2.Click += ButtonSub_EDIT;
					ButtonX4.SubItems.Add(buttonItem2);
					num9++;
					continue;
				}
				break;
			}
		}
	}

	public void ButtonSub_EDIT(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select id,status_name from HT_Receipt_H where id=", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null))));
		if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["status_name"], "ยกเล\u0e34ก", TextCompare: false))
		{
			MessageBox.Show("รายการบ\u0e34ลน\u0e35\u0e49ได\u0e49ยกเล\u0e34กไปแล\u0e49ว");
			return;
		}
		MyProject.Forms.FrmAddSale.clear();
		MyProject.Forms.FrmAddSale.IEdit = (string)RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["id"]);
		MyProject.Forms.FrmAddSale.clear();
		MyProject.Forms.FrmAddSale.ShowDialog();
		loadVAT();
	}

	public void ButtonSub_PRINT(object sender, EventArgs e)
	{
		Print_Report.Print_SaleVat(Conversions.ToInteger(NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), preview: false);
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_no='" + Label_NO.Text + "' order by cin_room_night");
		MyProject.Forms.FrmAddSale.IEdit = (string)(object)0;
		MyProject.Forms.FrmAddSale.clear();
		MyProject.Forms.FrmAddSale.Tref.Text = Label_NO.Text;
		MyProject.Forms.FrmAddSale.B2_Click(Label_NO.Text);
		MyProject.Forms.FrmAddSale.Tnote.Text = "เข\u0e49าพ\u0e31ก ว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yy") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yy");
		MyProject.Forms.FrmAddSale.ShowDialog();
		loadVAT();
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void FormShowVAT_Load(object sender, EventArgs e)
	{
		loadVAT();
	}

	private void ButtonX2_MouseHover(object sender, EventArgs e)
	{
		ButtonX2.Expanded = true;
		ButtonX4.Expanded = false;
	}

	private void ButtonX4_MouseHover(object sender, EventArgs e)
	{
		ButtonX4.Expanded = true;
		ButtonX2.Expanded = false;
	}

	private void ButtonX1_MouseHover(object sender, EventArgs e)
	{
		ButtonX4.Expanded = false;
		ButtonX2.Expanded = false;
	}

	private void LabelItem2_Click(object sender, EventArgs e)
	{
	}
}
