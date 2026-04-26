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
public class FrmReportCoupon : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ListView1")]
	private global::PrintableListView.PrintableListView _ListView1;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader13")]
	private ColumnHeader _ColumnHeader13;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

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

	internal virtual ColumnHeader ColumnHeader12
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader12 = value;
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

	[DebuggerNonUserCode]
	static FrmReportCoupon()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportCoupon()
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
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader13 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.Button3 = new System.Windows.Forms.Button();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label1 = new System.Windows.Forms.Label();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.ListView1);
		this.GroupBox1.Controls.Add(this.Button3);
		this.GroupBox1.Controls.Add(this.DateTimePicker2);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.DateTimePicker1);
		this.GroupBox1.Controls.Add(this.Label1);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(13, 13);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(1048, 483);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 0;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "รายงานค\u0e39ปอง";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = true;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[9] { this.ColumnHeader4, this.ColumnHeader2, this.ColumnHeader13, this.ColumnHeader12, this.ColumnHeader6, this.ColumnHeader9, this.ColumnHeader5, this.ColumnHeader1, this.ColumnHeader3 });
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(24, 61);
		listView.Location = location;
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView3 = this.ListView1;
		size = new System.Drawing.Size(1018, 415);
		listView3.Size = size;
		this.ListView1.TabIndex = 16;
		this.ListView1.Title = "";
		this.ListView1.Title2 = "";
		this.ListView1.Title2Tab = "";
		this.ListView1.Title3 = "";
		this.ListView1.Title3Tab = "";
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader4.Text = "ท\u0e35\u0e48";
		this.ColumnHeader4.Width = 40;
		this.ColumnHeader2.Text = "เลขท\u0e35\u0e48";
		this.ColumnHeader2.Width = 100;
		this.ColumnHeader13.Text = "ห\u0e49อง";
		this.ColumnHeader13.Width = 100;
		this.ColumnHeader12.Text = "ว\u0e31นท\u0e35\u0e48จ\u0e31ดทำ";
		this.ColumnHeader12.Width = 140;
		this.ColumnHeader6.Text = "ว\u0e31นท\u0e35\u0e48ใช\u0e49ค\u0e39ปอง";
		this.ColumnHeader6.Width = 140;
		this.ColumnHeader9.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader9.Width = 300;
		this.ColumnHeader5.Text = "ผ\u0e39\u0e49จ\u0e31ดทำ";
		this.ColumnHeader5.Width = 100;
		this.ColumnHeader1.Text = "สถานะ";
		this.ColumnHeader1.Width = 100;
		System.Windows.Forms.Button button = this.Button3;
		location = new System.Drawing.Point(519, 29);
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
		this.ColumnHeader3.Text = "";
		this.ColumnHeader3.Width = 80;
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
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportCoupon";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานค\u0e39ปอง";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		Search();
	}

	public void Search()
	{
		Cursor = Cursors.WaitCursor;
		object left = "select * from View_cupon where cupon_no<>''";
		left = Operators.ConcatenateObject(left, string.Concat(string.Concat(string.Concat(" and (cupon_date between '" + Conversions.ToString(DateTimePicker1.Value.Date), " 00:00:00' and '"), Conversions.ToString(DateTimePicker2.Value.Date)), " 23:59:59')"));
		left = Operators.ConcatenateObject(left, " order by cupon_cin_room,cupon_no");
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect(Conversions.ToString(left));
		decimal num = default(decimal);
		checked
		{
			int num2 = dataSet.Tables[0].Rows.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				int count = ListView1.Items.Count;
				ListView1.Items.Add(Conversions.ToString(num3 + 1));
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["cupon_no"]), "00000"));
				ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[count].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow2 = dataRow;
				string columnName = "cupon_cin_room";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["cupon_gen_date"]), "dd/MM/yyyy HH:mm"));
				ListView1.Items[count].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num3]["cupon_date"]), "dd/MM/yyyy"));
				ListView1.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num3]["cust_name"].ToString());
				ListViewItem.ListViewSubItemCollection subItems2 = ListView1.Items[count].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num3];
				DataRow dataRow3 = dataRow;
				columnName = "cupon_by";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num3]["cupon_print"], 0, TextCompare: false))
				{
					ListView1.Items[count].SubItems.Add("ย\u0e31งไม\u0e48พ\u0e34มพ\u0e4c");
				}
				else
				{
					ListView1.Items[count].SubItems.Add("พ\u0e34มพ\u0e4cแล\u0e49ว");
					ListView1.Items[count].BackColor = Color.LightGreen;
				}
				ListView1.Items[count].SubItems.Add("");
				num3++;
			}
			Cursor = Cursors.Default;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		ListView1.Title = "รายงานค\u0e39ปอง \r\nระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd-MM-yy เวลา HH:mm น.") + " ถ\u0e36ง " + Strings.Format(DateTimePicker2.Value, "dd-MM-yy เวลา HH:mm น.");
		ListView1.Title3 = "_";
		ListView1.Atto_กระดาษแนวนอน = true;
		ListView1.PrintPreview();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		DateTimePicker1.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 00:00:00");
		DateTimePicker2.Value = Conversions.ToDate(Conversions.ToString(DateTime.Now.Date) + " 23:59:59");
		Search();
	}
}
