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
public class FrmReportPaybooking : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader16")]
	private ColumnHeader _ColumnHeader16;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ลบToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

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

	internal virtual Button Button3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button3_Click;
			if (_Button3 != null)
			{
				_Button3.Click -= value2;
			}
			_Button3 = value;
			if (_Button3 != null)
			{
				_Button3.Click += value2;
			}
		}
	}

	internal virtual ColumnHeader ColumnHeader7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader7 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader9 = value;
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
			EventHandler value2 = ComboBox1_SelectedIndexChanged;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged -= value2;
			}
			_ComboBox1 = value;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged += value2;
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

	internal virtual ColumnHeader ColumnHeader13
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader13 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader16
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader16 = value;
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

	[DebuggerNonUserCode]
	static FrmReportPaybooking()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportPaybooking()
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
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label5 = new System.Windows.Forms.Label();
		this.Button3 = new System.Windows.Forms.Button();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader16 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label5);
		this.GroupBox1.Controls.Add(this.Button3);
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
		this.GroupBox1.Text = "รายงานเง\u0e34นจองตามรอบบ\u0e34ล";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.DropDownWidth = 500;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[3] { "ท\u0e31\u0e49งหมด", "บ\u0e34ลธรรมดา", "บ\u0e34ลภาษ\u0e35" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(78, 29);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(375, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 95;
		this.Label5.AutoSize = true;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label5.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label = this.Label5;
		location = new System.Drawing.Point(11, 33);
		label.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label2 = this.Label5;
		size = new System.Drawing.Size(64, 16);
		label2.Size = size;
		this.Label5.TabIndex = 94;
		this.Label5.Text = "รอบบ\u0e34ลท\u0e35\u0e48 :";
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(459, 29);
		button.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button2 = this.Button3;
		size = new System.Drawing.Size(84, 25);
		button2.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "ค\u0e49นหา";
		this.Button3.UseVisualStyleBackColor = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[8] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader5, this.ColumnHeader16, this.ColumnHeader3, this.ColumnHeader7, this.ColumnHeader9, this.ColumnHeader13 });
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
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader1.Width = 40;
		this.ColumnHeader2.Text = "เลขท\u0e35\u0e48ใบจอง";
		this.ColumnHeader2.Width = 90;
		this.ColumnHeader5.Text = "ว\u0e31นท\u0e35\u0e48จ\u0e48าย";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader5.Width = 120;
		this.ColumnHeader16.Text = "เลขลงทะเบ\u0e35ยน";
		this.ColumnHeader16.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader16.Width = 100;
		this.ColumnHeader3.Text = "เบอร\u0e4cห\u0e49อง";
		this.ColumnHeader3.Width = 170;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อล\u0e39กค\u0e49า";
		this.ColumnHeader7.Width = 350;
		this.ColumnHeader9.Text = "ร\u0e31บเง\u0e34นจอง";
		this.ColumnHeader9.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader9.Width = 80;
		this.ColumnHeader13.Text = "พน\u0e31กงาน";
		this.ColumnHeader13.Width = 100;
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
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(977, 503);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(84, 23);
		button4.Size = size;
		this.Button2.TabIndex = 3;
		this.Button2.Text = "ป\u0e34ด";
		this.Button2.UseVisualStyleBackColor = true;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button5 = this.Button1;
		location = new System.Drawing.Point(887, 503);
		button5.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button6 = this.Button1;
		size = new System.Drawing.Size(84, 23);
		button6.Size = size;
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
		this.Name = "FrmReportPaybooking";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานเง\u0e34นจอง";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		search();
	}

	public void search()
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
		object left = "select cust_name,Pay_by,pay_no,cin_no,cin_pay_cash,cin_pay_credit,cin_pay_date,Cin_Pay_note from View_Pay_Ds";
		object left2 = "select * from HT_Book_H where ";
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where (book_date between '", dataSet.Tables[0].Rows[0]["round_start"]), "' and '"), dataSet.Tables[0].Rows[0]["round_end"]), "')"));
		}
		else
		{
			left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (cin_pay_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
			left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(" where (book_date >= '", dataSet.Tables[0].Rows[0]["round_start"]), "')"));
		}
		left = Operators.ConcatenateObject(left, " and cin_no like 'R%' and Cin_Status<>'ยกเล\u0e34ก' group by  cust_name,Pay_by,pay_no,cin_no,cin_pay_cash,cin_pay_credit,cin_pay_date,Cin_Pay_note order by pay_no");
		left2 = Operators.ConcatenateObject(left2, ")");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(left));
		Module1.connect(Conversions.ToString(left2));
		ListView1.Items.Clear();
		decimal num = default(decimal);
		decimal num2 = default(decimal);
		decimal num3 = default(decimal);
		checked
		{
			int num4 = dataSet2.Tables[0].Rows.Count - 1;
			int num5 = 0;
			while (true)
			{
				int num6 = num5;
				int num7 = num4;
				if (num6 > num7)
				{
					break;
				}
				int count = ListView1.Items.Count;
				global::PrintableListView.PrintableListView listView = ListView1;
				listView.Items.Add(Conversions.ToString(count + 1));
				listView.Items[count].SubItems.Add(dataSet2.Tables[0].Rows[num5]["pay_no"].ToString());
				listView.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num5]["cin_pay_date"]), "dd/MM/yy HH:mm"));
				listView.Items[count].SubItems.Add(dataSet2.Tables[0].Rows[num5]["cin_no"].ToString());
				listView.Items[count].SubItems.Add("-");
				listView.Items[count].SubItems.Add(dataSet2.Tables[0].Rows[num5]["cust_name"].ToString());
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				object[] array = new object[1];
				DataRow dataRow = dataSet2.Tables[0].Rows[num5];
				string columnName = "Cin_Pay_Cash";
				array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
				object[] array2 = array;
				bool[] array3 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", array2, null, null, array3, IgnoreReturn: true);
				if (array3[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
				}
				num = Conversions.ToDecimal(Operators.AddObject(num, dataSet2.Tables[0].Rows[num5]["Cin_Pay_Cash"]));
				num2 = Conversions.ToDecimal(Operators.AddObject(num2, dataSet2.Tables[0].Rows[num5]["Cin_Pay_Credit"]));
				listView.Items[count].SubItems.Add(dataSet2.Tables[0].Rows[num5]["Pay_by"].ToString());
				listView = null;
				num5++;
			}
			int count2 = ListView1.Items.Count;
			global::PrintableListView.PrintableListView listView2 = ListView1;
			listView2.Items.Add("");
			listView2.Items[count2].SubItems.Add("รวม");
			listView2.Items[count2].SubItems.Add("");
			listView2.Items[count2].SubItems.Add("");
			listView2.Items[count2].SubItems.Add("");
			listView2.Items[count2].SubItems.Add("");
			listView2.Items[count2].SubItems.Add(Conversions.ToString(num));
			listView2.Items[count2].SubItems.Add("");
			listView2.Items[count2].BackColor = Color.LightGreen;
			listView2 = null;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject("select * from View_RBill_H_Round_Only where id=", ComboBox1.SelectedValue)));
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["round_end"].ToString(), "", TextCompare: false) != 0)
		{
			ListView1.Title = "รายงานการจองห\u0e49องตามรอบบ\u0e34ล\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_end"]), "dd-MM-yy เวลา HH:mm น.");
		}
		else
		{
			ListView1.Title = "รายงานการจองห\u0e49องตามรอบบ\u0e34ล\r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["round_start"]), "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTime.Now, "dd-MM-yy เวลา HH:mm น.");
		}
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		ListDueBill();
		search();
	}

	public void ListDueBill()
	{
		DataSet dataSet = Module1.connect("select top 1000 * from View_RBill_H_Round_Only order by id desc");
		ComboBox1.DataSource = dataSet.Tables[0];
		ComboBox1.DisplayMember = "FullDate";
		ComboBox1.ValueMember = "id";
		ComboBox1.Text = "";
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
}
