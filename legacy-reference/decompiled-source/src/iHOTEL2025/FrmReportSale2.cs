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
using PrintableListView;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmReportSale2 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ลบToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

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

	internal virtual global::PrintableListView.PrintableListView ListView1
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
			_ListView1 = value;
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

	internal virtual ContextMenuStrip ContextMenuStrip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ContextMenuStrip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ContextMenuStrip1 = value;
		}
	}

	internal virtual ToolStripMenuItem ToolStripMenuItem_0
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_0_Click;
			if (toolStripMenuItem_0 != null)
			{
				toolStripMenuItem_0.Click -= value2;
			}
			toolStripMenuItem_0 = value;
			if (toolStripMenuItem_0 != null)
			{
				toolStripMenuItem_0.Click += value2;
			}
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
			EventHandler value2 = DateTimePicker2_ValueChanged;
			if (_DateTimePicker2 != null)
			{
				_DateTimePicker2.ValueChanged -= value2;
			}
			_DateTimePicker2 = value;
			if (_DateTimePicker2 != null)
			{
				_DateTimePicker2.ValueChanged += value2;
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
			EventHandler value2 = DateTimePicker1_ValueChanged;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged -= value2;
			}
			_DateTimePicker1 = value;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmReportSale2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportSale2()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmReportImcome_Load;
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
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label1 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label5 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.ListView1);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1048, 483);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายงาน เซลล\u0e4c แยกตามว\u0e31นท\u0e35\u0e48/ล\u0e39กค\u0e49า";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(340, 27);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(200, 24);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 95;
		this.Label1.AutoSize = true;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(280, 31);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(54, 16);
		label2.Size = size;
		this.Label1.TabIndex = 94;
		this.Label1.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(76, 27);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(200, 24);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 95;
		this.Label5.AutoSize = true;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label5.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label3 = this.Label5;
		location = new System.Drawing.Point(11, 31);
		label3.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label4 = this.Label5;
		size = new System.Drawing.Size(61, 16);
		label4.Size = size;
		this.Label5.TabIndex = 94;
		this.Label5.Text = "จากว\u0e31นท\u0e35\u0e48 :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(6, 59);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(1033, 418);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[1] { this.ToolStripMenuItem_0 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(93, 26);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "ลบToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(92, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ลบ";
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button2;
		location = new System.Drawing.Point(977, 503);
		button.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button2 = this.Button2;
		size = new System.Drawing.Size(84, 23);
		button2.Size = size;
		this.Button2.TabIndex = 3;
		this.Button2.Text = "ป\u0e34ด";
		this.Button2.UseVisualStyleBackColor = true;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button3 = this.Button1;
		location = new System.Drawing.Point(887, 503);
		button3.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button4 = this.Button1;
		size = new System.Drawing.Size(84, 23);
		button4.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "พ\u0e34มพ\u0e4c";
		this.Button1.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1073, 539);
		this.ClientSize = size;
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.GroupBox1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportSale2";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงาน เซลล\u0e4c แยกตามว\u0e31นท\u0e35\u0e48/ล\u0e39กค\u0e49า";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	public void search()
	{
		int num = 0;
		ListView1.Items.Clear();
		ListView1.Columns.Clear();
		DataSet dataSet = Module1.connect("select room_type from HT_Rooms group by room_type order by room_type");
		num = dataSet.Tables[0].Rows.Count;
		ListView1.Columns.Add("NO.", 40);
		ListView1.Columns.Add("IN - OUT", 240);
		ListView1.Columns.Add("NAME", 150);
		ListView1.Columns.Add("SALE", 80);
		checked
		{
			int num2 = num - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				ListView1.Columns.Add(Conversions.ToString(dataSet.Tables[0].Rows[num3]["room_type"]), 80, HorizontalAlignment.Center);
				num3++;
			}
			ListView1.Columns.Add("อ\u0e37\u0e48นๆ", 80, HorizontalAlignment.Center);
			DataSet dataSet2 = Module1.connect("select room_no,room_type from HT_Rooms");
			DataSet dataSet3 = Module1.connect("select * from View_Booking_Ds where book_room_start between '" + Conversions.ToString(DateTimePicker1.Value.Date) + " 00:00:00' and '" + Conversions.ToString(DateTimePicker2.Value.Date) + " 23:59:59' ORDER BY BOOK_NO,BOOK_ROOM_TYPE");
			string left = "";
			int num6 = 0;
			int num7 = dataSet3.Tables[0].Rows.Count - 1;
			int num8 = 0;
			while (true)
			{
				int num9 = num8;
				int num5 = num7;
				if (num9 > num5)
				{
					break;
				}
				if (Operators.ConditionalCompareObjectNotEqual(left, dataSet3.Tables[0].Rows[num8]["BOOK_NO"], TextCompare: false))
				{
					num6++;
					ListView1.Items.Add(Conversions.ToString(num6));
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num8]["book_room_start"]), "dd/MM/yyyy HH:mm") + " - " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet3.Tables[0].Rows[num8]["book_room_end"]), "dd/MM/yyyy HH:mm"));
					NewLateBinding.LateCall(ListView1.Items[ListView1.Items.Count - 1].SubItems, null, "Add", new object[1] { Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num8]["book_cust_name"], " "), dataSet3.Tables[0].Rows[num8]["book_cust_name2"]) }, null, null, null, IgnoreReturn: true);
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(dataSet3.Tables[0].Rows[num8]["book_sale"].ToString());
					int num10 = num - 1;
					int num11 = 0;
					while (true)
					{
						int num12 = num11;
						num5 = num10;
						if (num12 > num5)
						{
							break;
						}
						ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(Conversions.ToString(0));
						num11++;
					}
					ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(Conversions.ToString(0));
					left = Conversions.ToString(dataSet3.Tables[0].Rows[num8]["BOOK_NO"]);
				}
				int num13 = -1;
				int num14 = ListView1.Columns.Count - 1;
				int num15 = 4;
				while (true)
				{
					int num16 = num15;
					num5 = num14;
					if (num16 <= num5)
					{
						if (!Operators.ConditionalCompareObjectEqual(ListView1.Columns[num15].Text, dataSet3.Tables[0].Rows[num8]["Book_room_type"], TextCompare: false))
						{
							num15++;
							continue;
						}
						num13 = num15;
						break;
					}
					break;
				}
				string right = "";
				int num17 = dataSet2.Tables[0].Rows.Count - 1;
				int num18 = 0;
				while (true)
				{
					int num19 = num18;
					num5 = num17;
					if (num19 <= num5)
					{
						right = Conversions.ToString(dataSet3.Tables[0].Rows[num8]["Book_room_type"]);
						if (!Operators.ConditionalCompareObjectEqual(dataSet2.Tables[0].Rows[num18]["room_no"], right, TextCompare: false))
						{
							num18++;
							continue;
						}
						right = Conversions.ToString(dataSet2.Tables[0].Rows[num18]["room_type"]);
						break;
					}
					break;
				}
				int num20 = ListView1.Columns.Count - 1;
				int num21 = 1;
				while (true)
				{
					int num22 = num21;
					num5 = num20;
					if (num22 <= num5)
					{
						if (Operators.CompareString(ListView1.Columns[num21].Text, right, TextCompare: false) != 0)
						{
							num21++;
							continue;
						}
						num13 = num21;
						break;
					}
					break;
				}
				if (num13 == -1)
				{
					num13 = ListView1.Columns.Count - 1;
				}
				ListView1.Items[ListView1.Items.Count - 1].SubItems[num13].Text = Conversions.ToString(Operators.AddObject(Conversions.ToDecimal(ListView1.Items[ListView1.Items.Count - 1].SubItems[num13].Text), Operators.MultiplyObject(dataSet3.Tables[0].Rows[num8]["Book_Room_Night"], dataSet3.Tables[0].Rows[num8]["Book_Room_Num"])));
				num8++;
			}
			ListView1.Items.Add("");
			ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("GRAND TOTAL");
			ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
			ListView1.Items[ListView1.Items.Count - 1].SubItems.Add("");
			int num23 = num - 1;
			int num24 = 0;
			while (true)
			{
				int num25 = num24;
				int num5 = num23;
				if (num25 > num5)
				{
					break;
				}
				ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(Conversions.ToString(0));
				num24++;
			}
			ListView1.Items[ListView1.Items.Count - 1].SubItems.Add(Conversions.ToString(0));
			ListView1.Items[ListView1.Items.Count - 1].BackColor = Color.LightGreen;
			int num26 = ListView1.Columns.Count - 1;
			int num27 = 4;
			while (true)
			{
				int num28 = num27;
				int num5 = num26;
				if (num28 > num5)
				{
					break;
				}
				decimal num29 = default(decimal);
				int num30 = ListView1.Items.Count - 2;
				int num31 = 0;
				while (true)
				{
					int num32 = num31;
					num5 = num30;
					if (num32 > num5)
					{
						break;
					}
					num29 = decimal.Add(num29, Conversions.ToDecimal(ListView1.Items[num31].SubItems[num27].Text));
					num31++;
				}
				ListView1.Items[ListView1.Items.Count - 1].SubItems[num27].Text = Conversions.ToString(num29);
				num27++;
			}
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		ListView1.Title = "รายงาน เซลล\u0e4c แยกตามประเภทห\u0e49องพ\u0e31ก \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy");
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = false;
		ListView1.PrintPreview();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		search();
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		MyProject.Forms.FormPass.ShowDialog();
		if (Operators.CompareString(MyProject.Forms.FormPass.TextBox1.Text, "", TextCompare: false) != 0)
		{
			DataSet dataSet = Module1.connect("select * from TB_MRP_EMPLOYEE where emp_level='admin' and emp_password='" + MyProject.Forms.FormPass.TextBox1.Text + "'");
			if (dataSet.Tables[0].Rows.Count == 0)
			{
				MessageBox.Show("รห\u0e31สผ\u0e48านไม\u0e48ถ\u0e39กต\u0e49อง");
			}
			else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบใบเสร\u0e47จ " + ListView1.SelectedItems[0].SubItems[1].Text + " หร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
			{
				Module1.connect("delete from HT_CheckIn_Pay where pay_no='" + ListView1.SelectedItems[0].SubItems[1].Text + "'");
				search();
			}
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		search();
	}

	private void DateTimePicker2_ValueChanged(object sender, EventArgs e)
	{
		search();
	}
}
