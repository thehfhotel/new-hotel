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

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmReportProducts : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Search_type")]
	private ComboBox _Search_type;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

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

	internal virtual ComboBox Search_type
	{
		[DebuggerNonUserCode]
		get
		{
			return _Search_type;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Search_type = value;
		}
	}

	internal virtual Label Label12
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label12 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmReportProducts()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportProducts()
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
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Button3 = new System.Windows.Forms.Button();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label1 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.Search_type = new System.Windows.Forms.ComboBox();
		this.Label12 = new System.Windows.Forms.Label();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.Search_type);
		this.GroupBox1.Controls.Add(this.Label12);
		this.GroupBox1.Controls.Add(this.Button3);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label1);
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
		this.GroupBox1.Text = "รายงานส\u0e34นค\u0e49าภายในโรงแรม";
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(776, 29);
		button.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button2 = this.Button3;
		size = new System.Drawing.Size(84, 25);
		button2.Size = size;
		this.Button3.TabIndex = 3;
		this.Button3.Text = "ค\u0e49นหา";
		this.Button3.UseVisualStyleBackColor = true;
		this.DateTimePicker2.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker2.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker2;
		location = new System.Drawing.Point(327, 30);
		dateTimePicker.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker2;
		size = new System.Drawing.Size(186, 24);
		dateTimePicker2.Size = size;
		this.DateTimePicker2.TabIndex = 4;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label = this.Label2;
		location = new System.Drawing.Point(268, 33);
		label.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label2 = this.Label2;
		size = new System.Drawing.Size(58, 17);
		label2.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48 :";
		this.DateTimePicker1.CustomFormat = "ddMMMMyy HH:mm";
		this.DateTimePicker1.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker1;
		location = new System.Drawing.Point(86, 30);
		dateTimePicker3.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker1;
		size = new System.Drawing.Size(176, 24);
		dateTimePicker4.Size = size;
		this.DateTimePicker1.TabIndex = 5;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label1;
		location = new System.Drawing.Point(21, 33);
		label3.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label4 = this.Label1;
		size = new System.Drawing.Size(66, 17);
		label4.Size = size;
		this.Label1.TabIndex = 2;
		this.Label1.Text = "จากว\u0e31นท\u0e35\u0e48 :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[6] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader7, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader6 });
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
		this.ColumnHeader2.Text = "รห\u0e31ส";
		this.ColumnHeader2.Width = 130;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.ColumnHeader7.Width = 280;
		this.ColumnHeader4.Text = "ราคา";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 80;
		this.ColumnHeader5.Text = "คงเหล\u0e37อ";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 90;
		this.ColumnHeader6.Text = "ขายได\u0e49";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader6.Width = 90;
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
		this.Search_type.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.Search_type.FormattingEnabled = true;
		System.Windows.Forms.ComboBox search_type = this.Search_type;
		location = new System.Drawing.Point(616, 30);
		search_type.Location = location;
		this.Search_type.Name = "Search_type";
		System.Windows.Forms.ComboBox search_type2 = this.Search_type;
		size = new System.Drawing.Size(154, 24);
		search_type2.Size = size;
		this.Search_type.TabIndex = 14;
		this.Label12.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label12;
		location = new System.Drawing.Point(519, 35);
		label5.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label6 = this.Label12;
		size = new System.Drawing.Size(96, 17);
		label6.Size = size;
		this.Label12.TabIndex = 15;
		this.Label12.Text = "ประเภทส\u0e34นค\u0e49า :";
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
		this.Name = "FrmReportProducts";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานส\u0e34นค\u0e49า";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		search();
	}

	public void search()
	{
		object left = "SELECT * from HT_Products";
		object left2 = "SELECT cin_pro_id,sum(cin_pro_num) as p_total from HT_CheckIn_Product";
		object left3 = "SELECT cin_pay_ds_id as cin_pro_id,sum(cin_pay_ds_num) as p_total from View_Pay_Ds";
		left2 = Operators.ConcatenateObject(left2, string.Concat(string.Concat(string.Concat(" where (cin_ds_date between '" + Conversions.ToString(DateTimePicker1.Value.Date), " 00:00:00' and '"), Conversions.ToString(DateTimePicker2.Value.Date)), " 23:59:59')"));
		left3 = Operators.ConcatenateObject(left3, string.Concat(string.Concat(string.Concat(" where (cin_pay_date between '" + Conversions.ToString(DateTimePicker1.Value.Date), " 00:00:00' and '"), Conversions.ToString(DateTimePicker2.Value.Date)), " 23:59:59')"));
		left2 = Operators.ConcatenateObject(left2, " group by cin_pro_id");
		left3 = Operators.ConcatenateObject(left3, " and cin_status<>'ยกเล\u0e34ก' group by cin_pay_ds_id");
		if (Operators.CompareString(Search_type.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			left = Operators.ConcatenateObject(left, string.Concat(" where pro_no in ( select Pro_no from HT_Products where Pro_Type = '" + Search_type.Text, "' )"));
		}
		left = Operators.ConcatenateObject(left, " order by Pro_Name");
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		DataSet dataSet2 = Module1.connect("select * from HT_SET_CusType order by id");
		DataSet dataSet3 = Module1.connect("select * from HT_Products_Price");
		DataSet dataSet4 = Module1.connect(Conversions.ToString(left2));
		DataSet dataSet5 = Module1.connect(Conversions.ToString(left3));
		ListView1.Columns.Clear();
		ListView1.Items.Clear();
		ListView1.Columns.Add("ท\u0e35\u0e48", 40);
		ListView1.Columns.Add("รห\u0e31ส", 140);
		ListView1.Columns.Add("ช\u0e37\u0e48อส\u0e34นค\u0e49า", 280);
		checked
		{
			int num = dataSet2.Tables[0].Rows.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ListView1.Columns.Add(Conversions.ToString(dataSet2.Tables[0].Rows[num2]["name"]), 120);
				num2++;
			}
			ListView1.Columns.Add("คงเหล\u0e37อ", 90);
			ListView1.Columns.Add("ขายได\u0e49", 90);
			int num5 = dataSet.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				global::PrintableListView.PrintableListView listView = ListView1;
				int count = listView.Items.Count;
				listView.Items.Add(Conversions.ToString(count + 1));
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow2 = dataRow;
				string columnName = "Pro_no";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow3 = dataRow;
				columnName = "Pro_Name";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				int num8 = dataSet2.Tables[0].Rows.Count - 1;
				int num9 = 0;
				while (true)
				{
					int num10 = num9;
					num4 = num8;
					if (num10 > num4)
					{
						break;
					}
					bool flag = false;
					int num11 = dataSet3.Tables[0].Rows.Count - 1;
					int num12 = 0;
					while (true)
					{
						int num13 = num12;
						num4 = num11;
						if (num13 > num4)
						{
							break;
						}
						if (!Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num9]["name"], dataSet3.Tables[0].Rows[num12]["P_CustType"], TextCompare: false), Operators.CompareObjectEqual(dataSet.Tables[0].Rows[num6]["Pro_no"], dataSet3.Tables[0].Rows[num12]["P_ID"], TextCompare: false))))
						{
							num12++;
							continue;
						}
						ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[count].SubItems;
						array3 = new object[1];
						object[] array6 = array3;
						dataRow = dataSet3.Tables[0].Rows[num12];
						DataRow dataRow4 = dataRow;
						columnName = "P_Price";
						array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
						array = array3;
						object[] arguments3 = array;
						array4 = new bool[1] { true };
						NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
						if (array4[0])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
						}
						flag = true;
						break;
					}
					if (!flag)
					{
						listView.Items[count].SubItems.Add(Conversions.ToString(0));
					}
					num9++;
				}
				ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[count].SubItems;
				array3 = new object[1];
				object[] array7 = array3;
				dataRow = dataSet.Tables[0].Rows[num6];
				DataRow dataRow5 = dataRow;
				columnName = "Pro_Amt";
				array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
				array = array3;
				object[] arguments4 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				decimal num14 = default(decimal);
				int num15 = dataSet4.Tables[0].Rows.Count - 1;
				int num16 = 0;
				while (true)
				{
					int num17 = num16;
					num4 = num15;
					if (num17 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num16]["cin_pro_id"], dataSet.Tables[0].Rows[num6]["Pro_no"], TextCompare: false))
					{
						num14 = Conversions.ToDecimal(Operators.AddObject(num14, dataSet4.Tables[0].Rows[num16]["p_total"]));
					}
					num16++;
				}
				int num18 = dataSet5.Tables[0].Rows.Count - 1;
				int num19 = 0;
				while (true)
				{
					int num20 = num19;
					num4 = num18;
					if (num20 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet5.Tables[0].Rows[num19]["cin_pro_id"], dataSet.Tables[0].Rows[num6]["Pro_no"], TextCompare: false))
					{
						num14 = Conversions.ToDecimal(Operators.AddObject(num14, dataSet5.Tables[0].Rows[num19]["p_total"]));
					}
					num19++;
				}
				listView.Items[count].SubItems.Add(Conversions.ToString(num14));
				listView = null;
				num6++;
			}
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(Search_type.Text, "ท\u0e31\u0e49งหมด", TextCompare: false) != 0)
		{
			ListView1.Title = "รายงานส\u0e34นค\u0e49าภายในโรงแรม (" + Search_type.Text + ") \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		}
		else
		{
			ListView1.Title = "รายงานส\u0e34นค\u0e49าภายในโรงแรม \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		}
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 00:00:00");
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 23:59:59");
		Listtype();
		search();
	}

	public void Listtype()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_ProductType order by name");
		Search_type.Items.Clear();
		Search_type.Items.Add("ท\u0e31\u0e49งหมด");
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
				Search_type.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			Search_type.SelectedIndex = 0;
		}
	}
}
