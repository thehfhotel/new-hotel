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
public class FormSearchCust : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	public string SelectNO;

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
			EventHandler value2 = TextBox1_TextChanged;
			MouseEventHandler value3 = TextBox1_MouseDown;
			KeyEventHandler value4 = TextBox1_KeyDown;
			if (_TextBox1 != null)
			{
				_TextBox1.TextChanged -= value2;
				_TextBox1.MouseDown -= value3;
				_TextBox1.KeyDown -= value4;
			}
			_TextBox1 = value;
			if (_TextBox1 != null)
			{
				_TextBox1.TextChanged += value2;
				_TextBox1.MouseDown += value3;
				_TextBox1.KeyDown += value4;
			}
		}
	}

	internal virtual Label Label8
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label8 = value;
		}
	}

	internal virtual ListView ListView1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListView1_SelectedIndexChanged;
			EventHandler value3 = ListView1_DoubleClick;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
				_ListView1.DoubleClick -= value3;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
				_ListView1.DoubleClick += value3;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader1 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader2 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader3 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader5 = value;
		}
	}

	internal virtual Button Button4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button4_Click;
			if (_Button4 != null)
			{
				_Button4.Click -= value2;
			}
			_Button4 = value;
			if (_Button4 != null)
			{
				_Button4.Click += value2;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader6 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader4 = value;
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

	[DebuggerNonUserCode]
	static FormSearchCust()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSearchCust()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormSearchLuandery_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		SelectNO = "";
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
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Label8 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.Button4 = new System.Windows.Forms.Button();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.SuspendLayout();
		this.TextBox1.BackColor = System.Drawing.Color.White;
		this.TextBox1.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		System.Drawing.Point location = new System.Drawing.Point(177, 12);
		textBox.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		System.Drawing.Size size = new System.Drawing.Size(198, 23);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 2;
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label = this.Label8;
		location = new System.Drawing.Point(19, 15);
		label.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label2 = this.Label8;
		size = new System.Drawing.Size(155, 16);
		label2.Size = size;
		this.Label8.TabIndex = 0;
		this.Label8.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า/ท\u0e35\u0e48ทำงาน/เบอร\u0e4cโทร";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[6] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader6, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(17, 41);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(932, 402);
		listView2.Size = size;
		this.ListView1.TabIndex = 4;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ลำด\u0e31บ";
		this.ColumnHeader1.Width = 50;
		this.ColumnHeader2.Text = "รห\u0e31สล\u0e39กค\u0e49า";
		this.ColumnHeader2.Width = 90;
		this.ColumnHeader6.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader6.Width = 140;
		this.ColumnHeader3.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.ColumnHeader3.Width = 250;
		this.ColumnHeader4.Text = "ท\u0e35\u0e48อย\u0e39\u0e48ท\u0e35\u0e48ทำงาน";
		this.ColumnHeader4.Width = 450;
		this.ColumnHeader5.Text = "โทร";
		this.ColumnHeader5.Width = 110;
		this.Button4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button4;
		location = new System.Drawing.Point(881, 451);
		button.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button2 = this.Button4;
		size = new System.Drawing.Size(68, 23);
		button2.Size = size;
		this.Button4.TabIndex = 3;
		this.Button4.Text = "ป\u0e34ด";
		this.Button4.UseVisualStyleBackColor = true;
		this.Timer1.Interval = 300;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(961, 486);
		this.ClientSize = size;
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.Label8);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.SizableToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormSearchCust";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ค\u0e49นหาล\u0e39กค\u0e49า";
		this.TopMost = true;
		this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void FormSearchLuandery_Load(object sender, EventArgs e)
	{
		SelectNO = "";
		Search();
	}

	public void Search()
	{
		object right = "";
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0)
		{
			right = " and (Cust_name like '%" + TextBox1.Text + "%' or  Cust_add_tel like '%" + TextBox1.Text + "%' or  Cust_no like '%" + TextBox1.Text + "%' or  Cust_work_name like '%" + TextBox1.Text + "%')";
		}
		ListView1.BeginUpdate();
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select top 500 * from View_Customers  where Cust_name<>'' ", right), "  order by Cust_no")));
		ListView1.Items.Clear();
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
				ListView listView = ListView1;
				listView.Items.Add(Conversions.ToString(num2 + 1));
				listView.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_no"].ToString());
				listView.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_name"].ToString());
				listView.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["C_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
					.Replace("ถนน ", "")
					.Replace("เขต/อำเภอ ", "")
					.Replace("แขวง/ตำบล ", "")
					.Replace("จ\u0e31งหว\u0e31ด ", ""));
				listView.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_work_name"].ToString() + " " + dataSet.Tables[0].Rows[num2]["W_Address"].ToString().Replace("หม\u0e39\u0e48  ", "").Replace("ซอย ", "")
					.Replace("ถนน ", "")
					.Replace("เขต/อำเภอ ", "")
					.Replace("แขวง/ตำบล ", "")
					.Replace("จ\u0e31งหว\u0e31ด ", ""));
				listView.Items[num2].SubItems.Add(dataSet.Tables[0].Rows[num2]["Cust_Add_tel"].ToString());
				listView = null;
				num2++;
			}
			ListView1.EndUpdate();
		}
	}

	private void TextBox1_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return)
		{
			Timer1.Enabled = false;
			Search();
		}
		else
		{
			Timer1.Enabled = false;
			Timer1.Enabled = true;
		}
	}

	private void TextBox1_MouseDown(object sender, MouseEventArgs e)
	{
	}

	private void TextBox1_TextChanged(object sender, EventArgs e)
	{
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			SelectNO = ListView1.SelectedItems[0].SubItems[1].Text;
			Close();
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		Search();
	}
}
