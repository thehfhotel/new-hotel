using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmReportRecPay : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Button6")]
	private Button _Button6;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("M1")]
	private ComboBox _M1;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("M3")]
	private ComboBox _M3;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("M2")]
	private ComboBox _M2;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	internal virtual GroupBox GroupBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _GroupBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_GroupBox1 = value;
		}
	}

	internal virtual DateTimePicker DateTimePicker1
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DateTimePicker1 = value;
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

	internal virtual DateTimePicker DateTimePicker2
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DateTimePicker2 = value;
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

	internal virtual Button Button6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button6_Click;
			if (_Button6 != null)
			{
				_Button6.Click -= value2;
			}
			_Button6 = value;
			if (_Button6 != null)
			{
				_Button6.Click += value2;
			}
		}
	}

	internal virtual ComboBox ComboBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboBox1 = value;
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

	internal virtual ComboBox M1
	{
		[DebuggerNonUserCode]
		get
		{
			return _M1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox2_SelectedIndexChanged;
			if (_M1 != null)
			{
				_M1.SelectedIndexChanged -= value2;
			}
			_M1 = value;
			if (_M1 != null)
			{
				_M1.SelectedIndexChanged += value2;
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

	internal virtual ComboBox M3
	{
		[DebuggerNonUserCode]
		get
		{
			return _M3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = M3_SelectedIndexChanged;
			if (_M3 != null)
			{
				_M3.SelectedIndexChanged -= value2;
			}
			_M3 = value;
			if (_M3 != null)
			{
				_M3.SelectedIndexChanged += value2;
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

	internal virtual ComboBox M2
	{
		[DebuggerNonUserCode]
		get
		{
			return _M2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox4_SelectedIndexChanged;
			if (_M2 != null)
			{
				_M2.SelectedIndexChanged -= value2;
			}
			_M2 = value;
			if (_M2 != null)
			{
				_M2.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label6 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmReportRecPay()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportRecPay()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmReportsale_Load;
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
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.M2 = new System.Windows.Forms.ComboBox();
		this.Label6 = new System.Windows.Forms.Label();
		this.M3 = new System.Windows.Forms.ComboBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.M1 = new System.Windows.Forms.ComboBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label1 = new System.Windows.Forms.Label();
		this.Button6 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.M2);
		this.GroupBox1.Controls.Add(this.Label6);
		this.GroupBox1.Controls.Add(this.M3);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.M1);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.Button6);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(372, 240);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายงานระหว\u0e48างว\u0e31นท\u0e35\u0e48";
		this.M2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.M2.FormattingEnabled = true;
		this.M2.Items.AddRange(new object[3] { "รายร\u0e31บรายจ\u0e48าย", "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox m = this.M2;
		location = new System.Drawing.Point(87, 151);
		m.Location = location;
		this.M2.Name = "M2";
		System.Windows.Forms.ComboBox m2 = this.M2;
		size = new System.Drawing.Size(279, 24);
		m2.Size = size;
		this.M2.TabIndex = 10;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label = this.Label6;
		location = new System.Drawing.Point(0, 155);
		label.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label2 = this.Label6;
		size = new System.Drawing.Size(85, 16);
		label2.Size = size;
		this.Label6.TabIndex = 9;
		this.Label6.Text = "ประเภทบ\u0e31ญช\u0e35 :";
		this.M3.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.M3.FormattingEnabled = true;
		this.M3.Items.AddRange(new object[3] { "รายร\u0e31บรายจ\u0e48าย", "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox m3 = this.M3;
		location = new System.Drawing.Point(87, 182);
		m3.Location = location;
		this.M3.Name = "M3";
		System.Windows.Forms.ComboBox m4 = this.M3;
		size = new System.Drawing.Size(279, 24);
		m4.Size = size;
		this.M3.TabIndex = 8;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label5;
		location = new System.Drawing.Point(19, 186);
		label3.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label4 = this.Label5;
		size = new System.Drawing.Size(66, 16);
		label4.Size = size;
		this.Label5.TabIndex = 7;
		this.Label5.Text = "รห\u0e31สบ\u0e31ญช\u0e35 :";
		this.M1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.M1.FormattingEnabled = true;
		this.M1.Items.AddRange(new object[3] { "รายร\u0e31บรายจ\u0e48าย", "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox m5 = this.M1;
		location = new System.Drawing.Point(87, 121);
		m5.Location = location;
		this.M1.Name = "M1";
		System.Windows.Forms.ComboBox m6 = this.M1;
		size = new System.Drawing.Size(279, 24);
		m6.Size = size;
		this.M1.TabIndex = 6;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label4;
		location = new System.Drawing.Point(38, 125);
		label5.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label6 = this.Label4;
		size = new System.Drawing.Size(47, 16);
		label6.Size = size;
		this.Label4.TabIndex = 5;
		this.Label4.Text = "หมวด :";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "รายร\u0e31บรายจ\u0e48าย", "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(87, 91);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(161, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 4;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(87, 61);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(161, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 1;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label3;
		location = new System.Drawing.Point(28, 94);
		label7.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label8 = this.Label3;
		size = new System.Drawing.Size(58, 16);
		label8.Size = size;
		this.Label3.TabIndex = 0;
		this.Label3.Text = "ประเภท :";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label2;
		location = new System.Drawing.Point(28, 64);
		label9.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label10 = this.Label2;
		size = new System.Drawing.Size(54, 16);
		label10.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(87, 32);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(161, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 1;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label1;
		location = new System.Drawing.Point(22, 35);
		label11.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label12 = this.Label1;
		size = new System.Drawing.Size(61, 16);
		label12.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "จากว\u0e31นท\u0e35\u0e48 :";
		System.Windows.Forms.Button button = this.Button6;
		location = new System.Drawing.Point(269, 33);
		button.Location = location;
		this.Button6.Name = "Button6";
		System.Windows.Forms.Button button2 = this.Button6;
		size = new System.Drawing.Size(97, 50);
		button2.Size = size;
		this.Button6.TabIndex = 3;
		this.Button6.Text = "ออกรายงาน";
		this.Button6.UseVisualStyleBackColor = true;
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(298, 259);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(84, 23);
		button4.Size = size;
		this.Button2.TabIndex = 3;
		this.Button2.Text = "ป\u0e34ด";
		this.Button2.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(397, 295);
		this.ClientSize = size;
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.GroupBox1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportRecPay";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานรายร\u0e31บรายจ\u0e48าย";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void FrmReportsale_Load(object sender, EventArgs e)
	{
		MyProject.Application.ChangeCulture("en-US");
		ComboBox1.SelectedIndex = 0;
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		MyProject.Forms.login.ReflectionImage1.Image.Save(Module1.PathF + "logo.bmp");
		FileStream fileStream = new FileStream(Module1.PathF + "logo.bmp", FileMode.Open);
		object instance = new BinaryReader(fileStream);
		byte[] copany_Logo = (byte[])NewLateBinding.LateGet(instance, null, "ReadBytes", new object[1] { Conversions.ToInteger(NewLateBinding.LateGet(NewLateBinding.LateGet(instance, null, "BaseStream", new object[0], null, null, null), null, "Length", new object[0], null, null, null)) }, null, null, null);
		fileStream.Close();
		Module1.localdata.Bill_H.Rows.Clear();
		Module1.localdata.Bill_H.AddBill_HRow(dataSet.Tables[0].Rows[0]["CompanyName"].ToString(), dataSet.Tables[0].Rows[0]["Company_Address"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tax"].ToString(), dataSet.Tables[0].Rows[0]["Company_Tel"].ToString(), dataSet.Tables[0].Rows[0]["Company_Fax"].ToString(), copany_Logo);
		loadGroup();
		loadAccount();
	}

	public void loadGroup()
	{
		M1.Items.Clear();
		M1.Items.Add("");
		DataSet dataSet = Module1.connect("select * from TB_SET_MyType2 order by id_full");
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				M1.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			M1.SelectedIndex = 0;
		}
	}

	private void Button6_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกประเภท");
			return;
		}
		Cursor = Cursors.WaitCursor;
		string text = "";
		checked
		{
			if (Operators.CompareString(M1.Text, "", TextCompare: false) != 0)
			{
				text = M1.Text.Substring(M1.Text.IndexOf("|") + 2);
			}
			DateTime dateTime = Conversions.ToDate(Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00");
			DateTime dateTime2 = Conversions.ToDate(Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59");
			string text2 = "";
			if (Operators.CompareString(text, "", TextCompare: false) != 0)
			{
				text2 = "and Pay_Group='" + text + "'";
			}
			if (Operators.CompareString(M3.Text, "", TextCompare: false) != 0)
			{
				text2 = text2 + "and Pay_Account like '" + M3.Text.Substring(0, M3.Text.IndexOf("|")) + "%'";
			}
			else if (Operators.CompareString(M2.Text, "", TextCompare: false) != 0)
			{
				text2 = text2 + "and Pay_Account like '" + M2.Text.Substring(0, M2.Text.IndexOf("|")) + "%'";
			}
			else if (Operators.CompareString(M1.Text, "", TextCompare: false) != 0)
			{
				text2 = text2 + "and Pay_Account like '" + M1.Text.Substring(0, M1.Text.IndexOf("|")) + "%'";
			}
			string text3 = "";
			if (Operators.CompareString(ComboBox1.Text, "รายร\u0e31บรายจ\u0e48าย", TextCompare: false) != 0)
			{
				text3 = " pay_type='" + ComboBox1.Text + "' and ";
			}
			DataSet dataSet = Module1.connect("SELECT  * from tb_pay_history where " + text3 + "  (pay_date between " + Conversions.ToString(dateTime.ToOADate()) + " and " + Conversions.ToString(dateTime2.ToOADate()) + ") " + text2 + " order by id");
			Module1.localdata.ReportSale.Rows.Clear();
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			decimal num5 = default(decimal);
			decimal num6 = default(decimal);
			decimal num7 = default(decimal);
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				string text4 = "";
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num2]["pay_type"], "รายจ\u0e48าย", TextCompare: false))
				{
					text4 = "-";
					num5 = Conversions.ToDecimal(Operators.SubtractObject(num5, dataSet.Tables[0].Rows[num2]["pay_Total"]));
				}
				else
				{
					num5 = Conversions.ToDecimal(Operators.AddObject(num5, dataSet.Tables[0].Rows[num2]["pay_Total"]));
				}
				string string_ = "";
				string string_2 = text4 + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["pay_Total"]), "#,##0.00");
				num6 = Conversions.ToDecimal(Operators.AddObject(num6, dataSet.Tables[0].Rows[num2]["pay_Total"]));
				if (ComboBox1.SelectedIndex == 0 && Operators.CompareString(text4, "-", TextCompare: false) == 0)
				{
					string_ = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["pay_Total"]), "#,##0.00");
					string_2 = "";
					num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet.Tables[0].Rows[num2]["pay_Total"]));
					num6 = Conversions.ToDecimal(Operators.SubtractObject(num6, dataSet.Tables[0].Rows[num2]["pay_Total"]));
				}
				Module1.localdata.ReportSale.AddReportSaleRow(Conversions.ToString(num2 + 1), Strings.Format(DateTime.FromOADate(Conversions.ToDouble(dataSet.Tables[0].Rows[num2]["pay_date"])), "dd/MM/yyyy"), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Pay_bill"]), "", string_, string_2, "", Strings.Format(num5, "#,##0.00"), "รายงาน" + ComboBox1.Text + " ระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value.Date, "dd/MM/yy") + " ถ\u0e36งว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker2.Value.Date, "dd/MM/yy"), "", "", "", Strings.Format(num7, "#,##0.00"), Strings.Format(num6, "#,##0.00"), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Pay_Group"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["Pay_Account"]));
				num2++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			if (ComboBox1.SelectedIndex == 0)
			{
				ReportIncome2 reportIncome = new ReportIncome2();
				reportIncome.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportIncome;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			else
			{
				ReportIncome reportIncome2 = new ReportIncome();
				reportIncome2.SetDataSource(Module1.localdata);
				MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportIncome2;
				MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
			}
			Cursor = Cursors.Default;
		}
	}

	private void ComboBox2_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			loadAccounttype();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	public void loadAccounttype()
	{
		M2.Items.Clear();
		M2.Items.Add("");
		M3.Items.Clear();
		M3.Items.Add("");
		string text = "";
		if (Operators.CompareString(M1.Text, "", TextCompare: false) == 0)
		{
			return;
		}
		text = " where id_full like '" + M1.Text.Substring(0, M1.Text.IndexOf("|")) + "%' ";
		DataSet dataSet = Module1.connect("select * from TB_SET_MyType2_2 " + text + " order by id_full");
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					M2.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void loadAccount()
	{
		M3.Items.Clear();
		M3.Items.Add("");
		string text = "";
		if (Operators.CompareString(M2.Text, "", TextCompare: false) == 0)
		{
			return;
		}
		text = " where id_full like '" + M2.Text.Substring(0, M2.Text.IndexOf("|")) + "%' ";
		DataSet dataSet = Module1.connect("select * from TB_SET_MyType3 " + text + " order by id_full");
		checked
		{
			int num = dataSet.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					M3.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void ComboBox4_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			loadAccount();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void M3_SelectedIndexChanged(object sender, EventArgs e)
	{
	}
}
