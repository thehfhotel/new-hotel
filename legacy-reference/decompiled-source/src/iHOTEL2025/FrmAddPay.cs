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
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmAddPay : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("TextBox2")]
	private TextBox _TextBox2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ComboBox2")]
	private ComboBox _ComboBox2;

	[AccessedThroughProperty("TextBox3")]
	private TextBox _TextBox3;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("ComboBox3")]
	private ComboBox _ComboBox3;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

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
			KeyEventHandler value2 = TextBox1_KeyDown;
			EventHandler value3 = TextBox1_TextChanged;
			if (_TextBox1 != null)
			{
				_TextBox1.KeyDown -= value2;
				_TextBox1.TextChanged -= value3;
			}
			_TextBox1 = value;
			if (_TextBox1 != null)
			{
				_TextBox1.KeyDown += value2;
				_TextBox1.TextChanged += value3;
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
			KeyEventHandler value2 = TextBox2_KeyDown;
			if (_TextBox2 != null)
			{
				_TextBox2.KeyDown -= value2;
			}
			_TextBox2 = value;
			if (_TextBox2 != null)
			{
				_TextBox2.KeyDown += value2;
			}
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
			EventHandler value2 = Timer1_Tick;
			if (_Timer1 != null)
			{
				_Timer1.Tick -= value2;
			}
			_Timer1 = value;
			if (_Timer1 != null)
			{
				_Timer1.Tick += value2;
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

	internal virtual ComboBox ComboBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = ComboBox2_KeyDown;
			EventHandler value3 = ComboBox2_SelectedIndexChanged;
			if (_ComboBox2 != null)
			{
				_ComboBox2.KeyDown -= value2;
				_ComboBox2.SelectedIndexChanged -= value3;
			}
			_ComboBox2 = value;
			if (_ComboBox2 != null)
			{
				_ComboBox2.KeyDown += value2;
				_ComboBox2.SelectedIndexChanged += value3;
			}
		}
	}

	internal virtual TextBox TextBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = TextBox3_KeyDown;
			if (_TextBox3 != null)
			{
				_TextBox3.KeyDown -= value2;
			}
			_TextBox3 = value;
			if (_TextBox3 != null)
			{
				_TextBox3.KeyDown += value2;
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

	internal virtual ComboBox ComboBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox3_SelectedIndexChanged;
			if (_ComboBox3 != null)
			{
				_ComboBox3.SelectedIndexChanged -= value2;
			}
			_ComboBox3 = value;
			if (_ComboBox3 != null)
			{
				_ComboBox3.SelectedIndexChanged += value2;
			}
		}
	}

	internal virtual Label Label7
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label7 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmAddPay()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmAddPay()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmAddPay_Load;
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
		this.components = new System.ComponentModel.Container();
		this.Label1 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.TextBox2 = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Label5 = new System.Windows.Forms.Label();
		this.ComboBox2 = new System.Windows.Forms.ComboBox();
		this.TextBox3 = new System.Windows.Forms.TextBox();
		this.Label6 = new System.Windows.Forms.Label();
		this.ComboBox3 = new System.Windows.Forms.ComboBox();
		this.Label7 = new System.Windows.Forms.Label();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(27, 13);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(49, 16);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ประเภท";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[2] { "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(78, 9);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(121, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 0;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(15, 48);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(61, 16);
		label4.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "จำนวนเง\u0e34น";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(18, 149);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(58, 16);
		label6.Size = size;
		this.Label3.TabIndex = 0;
		this.Label3.Text = "หมายเหต\u0e38";
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		location = new System.Drawing.Point(78, 44);
		textBox.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		size = new System.Drawing.Size(121, 23);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 2;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(221, 14);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(31, 16);
		label8.Size = size;
		this.Label4.TabIndex = 0;
		this.Label4.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(258, 10);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(164, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 1;
		System.Windows.Forms.TextBox textBox3 = this.TextBox2;
		location = new System.Drawing.Point(78, 146);
		textBox3.Location = location;
		this.TextBox2.Name = "TextBox2";
		System.Windows.Forms.TextBox textBox4 = this.TextBox2;
		size = new System.Drawing.Size(344, 23);
		textBox4.Size = size;
		this.TextBox2.TabIndex = 3;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(347, 175);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(75, 34);
		button2.Size = size;
		this.Button1.TabIndex = 4;
		this.Button1.Text = "บ\u0e31นท\u0e35ก";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label5.AutoSize = true;
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(38, 81);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(38, 16);
		label10.Size = size;
		this.Label5.TabIndex = 0;
		this.Label5.Text = "หมวด";
		this.ComboBox2.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox2.DropDownWidth = 250;
		this.ComboBox2.FormattingEnabled = true;
		this.ComboBox2.Items.AddRange(new object[2] { "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox comboBox3 = this.ComboBox2;
		location = new System.Drawing.Point(78, 77);
		comboBox3.Location = location;
		this.ComboBox2.Name = "ComboBox2";
		System.Windows.Forms.ComboBox comboBox4 = this.ComboBox2;
		size = new System.Drawing.Size(121, 24);
		comboBox4.Size = size;
		this.ComboBox2.TabIndex = 0;
		System.Windows.Forms.TextBox textBox5 = this.TextBox3;
		location = new System.Drawing.Point(258, 110);
		textBox5.Location = location;
		this.TextBox3.Name = "TextBox3";
		System.Windows.Forms.TextBox textBox6 = this.TextBox3;
		size = new System.Drawing.Size(164, 23);
		textBox6.Size = size;
		this.TextBox3.TabIndex = 6;
		this.Label6.AutoSize = true;
		System.Windows.Forms.Label label11 = this.Label6;
		location = new System.Drawing.Point(199, 114);
		label11.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label12 = this.Label6;
		size = new System.Drawing.Size(57, 16);
		label12.Size = size;
		this.Label6.TabIndex = 5;
		this.Label6.Text = "โค\u0e4aดบ\u0e31ญช\u0e35";
		this.ComboBox3.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox3.DropDownWidth = 250;
		this.ComboBox3.FormattingEnabled = true;
		this.ComboBox3.Items.AddRange(new object[2] { "รายร\u0e31บ", "รายจ\u0e48าย" });
		System.Windows.Forms.ComboBox comboBox5 = this.ComboBox3;
		location = new System.Drawing.Point(78, 109);
		comboBox5.Location = location;
		this.ComboBox3.Name = "ComboBox3";
		System.Windows.Forms.ComboBox comboBox6 = this.ComboBox3;
		size = new System.Drawing.Size(121, 24);
		comboBox6.Size = size;
		this.ComboBox3.TabIndex = 8;
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label13 = this.Label7;
		location = new System.Drawing.Point(19, 113);
		label13.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label14 = this.Label7;
		size = new System.Drawing.Size(57, 16);
		label14.Size = size;
		this.Label7.TabIndex = 7;
		this.Label7.Text = "รห\u0e31สบ\u0e31ญช\u0e35";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(434, 221);
		this.ClientSize = size;
		this.Controls.Add(this.ComboBox3);
		this.Controls.Add(this.Label7);
		this.Controls.Add(this.TextBox3);
		this.Controls.Add(this.Label6);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.TextBox2);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.ComboBox2);
		this.Controls.Add(this.ComboBox1);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Label4);
		this.Controls.Add(this.Label5);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		size = new System.Drawing.Size(450, 184);
		this.MinimumSize = size;
		this.Name = "FrmAddPay";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เพ\u0e34\u0e48มรายการร\u0e31บ-จ\u0e48าย";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48จำนวนเง\u0e34น");
		}
		else if (!Versioned.IsNumeric(TextBox1.Text))
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48จำนวนเง\u0e34นเป\u0e47นต\u0e31วเลข");
		}
		else if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37กประเภท");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการบ\u0e31นท\u0e36กหร\u0e37อไม\u0e48", "บ\u0e31นท\u0e36ก", MessageBoxButtons.YesNo) == DialogResult.Yes)
		{
			string text = "";
			if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) != 0)
			{
				text = ComboBox2.Text.Substring(checked(ComboBox2.Text.IndexOf("|") + 2));
			}
			string command = string.Format("INSERT INTO TB_Pay_History VALUES({0},{1},'{2}','{3}','{4}',{5},'{6}',{7},'{8}','{9}')", Module1.get_id("TB_Pay_History", "id"), DateTimePicker1.Value.ToOADate(), TextBox2.Text, "", ComboBox1.Text, Conversions.ToDecimal(TextBox1.Text), "", 0, text, TextBox3.Text);
			Module1.connect(command);
			MessageBox.Show("บ\u0e31นท\u0e36กเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			TextBox1.Focus();
			TextBox1.Text = "";
			TextBox2.Text = "";
			TextBox3.Text = "";
			ComboBox2.SelectedIndex = 0;
		}
	}

	private void FrmAddPay_Load(object sender, EventArgs e)
	{
		MyProject.Application.ChangeCulture("en-US");
		loadGroup();
		loadAccount();
		DateTimePicker1.Value = DateTime.Now;
		TextBox1.Text = "";
		TextBox2.Text = "";
		TextBox3.Text = "";
		ComboBox2.SelectedIndex = 0;
		TextBox1.Focus();
		Timer1.Enabled = true;
	}

	public void loadGroup()
	{
		ComboBox2.Items.Clear();
		ComboBox2.Items.Add("");
		DataSet dataSet = Module1.connect("select * from TB_SET_MyType2 order by id_full");
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
					ComboBox2.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void loadAccount()
	{
		ComboBox3.Items.Clear();
		ComboBox3.Items.Add("");
		object obj = "";
		checked
		{
			if (Operators.CompareString(ComboBox2.Text, "", TextCompare: false) != 0)
			{
				obj = " where id_full like '" + ComboBox2.Text.Substring(0, ComboBox2.Text.IndexOf("|")) + "%' ";
				DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from TB_SET_MyType3 ", obj), " order by id_full")));
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
					ComboBox3.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "| "), dataSet.Tables[0].Rows[num2]["name"]));
					num2++;
				}
			}
			TextBox3.Text = "";
		}
	}

	private void TextBox1_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			ComboBox2.Focus();
		}
	}

	private void TextBox2_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			Button1.Focus();
		}
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		TextBox1.Focus();
	}

	private void TextBox1_TextChanged(object sender, EventArgs e)
	{
	}

	private void ComboBox2_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			TextBox3.Focus();
		}
	}

	private void TextBox3_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			TextBox2.Focus();
		}
	}

	private void ComboBox2_SelectedIndexChanged(object sender, EventArgs e)
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

	private void ComboBox3_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			if (Operators.CompareString(ComboBox3.Text, "", TextCompare: false) != 0)
			{
				TextBox3.Text = Module1.GEN_ACCOUNT(ComboBox3.Text.Substring(0, ComboBox3.Text.IndexOf("|")));
			}
			else
			{
				TextBox3.Text = "";
			}
			TextBox2.Focus();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}
}
