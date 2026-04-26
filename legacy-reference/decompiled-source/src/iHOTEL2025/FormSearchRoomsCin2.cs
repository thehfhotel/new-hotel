using System;
using System.Collections;
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

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSearchRoomsCin2 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

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

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("DateTimePicker2")]
	private DateTimePicker _DateTimePicker2;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	public ArrayList SelectNO_ARR;

	public string filter;

	public string room_no;

	public string noRoom;

	public int Days;

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
			ItemCheckedEventHandler value2 = ListView1_ItemChecked;
			EventHandler value3 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.ItemChecked -= value2;
				_ListView1.SelectedIndexChanged -= value3;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.ItemChecked += value2;
				_ListView1.SelectedIndexChanged += value3;
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

	internal virtual ColumnHeader ColumnHeader8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader8 = value;
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

	internal virtual ListView ListView2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListView2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ListView2 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader10
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader10 = value;
		}
	}

	internal virtual ColumnHeader ColumnHeader11
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader11 = value;
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

	internal virtual CheckBox CheckBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = CheckBox1_CheckedChanged;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged -= value2;
			}
			_CheckBox1 = value;
			if (_CheckBox1 != null)
			{
				_CheckBox1.CheckedChanged += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FormSearchRoomsCin2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSearchRoomsCin2()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FormSearchRoomsCin2_FormClosing;
		base.Load += FormSearchLuandery_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		SelectNO_ARR = new ArrayList();
		filter = "";
		room_no = "";
		noRoom = "";
		Days = 0;
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
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.Button4 = new System.Windows.Forms.Button();
		this.Label1 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker2 = new System.Windows.Forms.DateTimePicker();
		this.Label3 = new System.Windows.Forms.Label();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.Button1 = new System.Windows.Forms.Button();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.SuspendLayout();
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[8] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader6, this.ColumnHeader3, this.ColumnHeader5, this.ColumnHeader4, this.ColumnHeader7, this.ColumnHeader8 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		System.Drawing.Point location = new System.Drawing.Point(17, 41);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		System.Drawing.Size size = new System.Drawing.Size(938, 254);
		listView2.Size = size;
		this.ListView1.TabIndex = 4;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ลำด\u0e31บ";
		this.ColumnHeader1.Width = 50;
		this.ColumnHeader2.Text = "เลขห\u0e49อง";
		this.ColumnHeader2.Width = 90;
		this.ColumnHeader6.Text = "ประเภทห\u0e49อง";
		this.ColumnHeader6.Width = 140;
		this.ColumnHeader3.Text = "รายละเอ\u0e35ยดห\u0e49อง";
		this.ColumnHeader3.Width = 200;
		this.ColumnHeader5.Text = "ราคา A";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 0;
		this.ColumnHeader4.Text = "ราคา B";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 0;
		this.ColumnHeader7.Text = "ราคา C";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 0;
		this.ColumnHeader8.Text = "หมายเหต\u0e38";
		this.ColumnHeader8.Width = 400;
		this.Button4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button4;
		location = new System.Drawing.Point(887, 448);
		button.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button2 = this.Button4;
		size = new System.Drawing.Size(68, 36);
		button2.Size = size;
		this.Button4.TabIndex = 3;
		this.Button4.Text = "ป\u0e34ด";
		this.Button4.UseVisualStyleBackColor = true;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(14, 13);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(149, 16);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "รายการห\u0e49องว\u0e48างของค\u0e37นว\u0e31นท\u0e35\u0e48";
		this.DateTimePicker1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.DateTimePicker1.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.DateTimePicker1.Enabled = false;
		this.DateTimePicker1.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(168, 9);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 5;
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(377, 12);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(45, 16);
		label4.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "ถ\u0e36งว\u0e31นท\u0e35\u0e48";
		this.DateTimePicker2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.DateTimePicker2.CustomFormat = "dd/MM/yy เวลา HH:mm";
		this.DateTimePicker2.Enabled = false;
		this.DateTimePicker2.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker3 = this.DateTimePicker2;
		location = new System.Drawing.Point(425, 9);
		dateTimePicker3.Location = location;
		this.DateTimePicker2.Name = "DateTimePicker2";
		System.Windows.Forms.DateTimePicker dateTimePicker4 = this.DateTimePicker2;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker4.Size = size;
		this.DateTimePicker2.TabIndex = 5;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(19, 317);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(34, 16);
		label6.Size = size;
		this.Label3.TabIndex = 11;
		this.Label3.Text = "ราคา";
		this.ListView2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[2] { this.ColumnHeader10, this.ColumnHeader11 });
		this.ListView2.FullRowSelect = true;
		this.ListView2.GridLines = true;
		System.Windows.Forms.ListView listView3 = this.ListView2;
		location = new System.Drawing.Point(17, 336);
		listView3.Location = location;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView4 = this.ListView2;
		size = new System.Drawing.Size(938, 109);
		listView4.Size = size;
		this.ListView2.TabIndex = 10;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader10.Text = "ประเภทล\u0e39กค\u0e49า";
		this.ColumnHeader10.Width = 251;
		this.ColumnHeader11.Text = "ราคา";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 100;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.Button button3 = this.Button1;
		location = new System.Drawing.Point(813, 448);
		button3.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button4 = this.Button1;
		size = new System.Drawing.Size(68, 36);
		button4.Size = size;
		this.Button1.TabIndex = 12;
		this.Button1.Text = "ตกลง";
		this.Button1.UseVisualStyleBackColor = true;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(20, 297);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(181, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 13;
		this.CheckBox1.Text = "เล\u0e37อกท\u0e31\u0e49งหมด/ไม\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
		this.CheckBox1.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(967, 486);
		this.ClientSize = size;
		this.Controls.Add(this.CheckBox1);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.ListView2);
		this.Controls.Add(this.DateTimePicker2);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.SizableToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormSearchRoomsCin2";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ค\u0e49นหาห\u0e49องพ\u0e31ก";
		this.TopMost = true;
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void FormSearchRoomsCin2_FormClosing(object sender, FormClosingEventArgs e)
	{
		Days = 0;
		room_no = "";
	}

	private void FormSearchLuandery_Load(object sender, EventArgs e)
	{
		SelectNO_ARR.Clear();
		Search();
	}

	public void Search()
	{
		object right = "";
		checked
		{
			if (Days == 1)
			{
				right = ((!((decimal.Compare(Conversions.ToDecimal(Strings.Format(DateTimePicker1.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(DateTimePicker1.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))) ? (" room_date='" + Conversions.ToString(DateTimePicker1.Value.Date) + "'") : (" room_date='" + Conversions.ToString(DateTimePicker1.Value.AddDays(-1.0).Date) + "'"));
			}
			else if (Days > 1)
			{
				DateTime dateTime = DateTimePicker1.Value;
				right = " room_date in (";
				int days = Days;
				if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(DateTimePicker1.Value, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(DateTimePicker1.Value, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
				{
					dateTime = dateTime.AddDays(-1.0);
				}
				int num = days - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					if (num2 != 0)
					{
						right = Operators.ConcatenateObject(right, ",");
					}
					right = Operators.ConcatenateObject(right, string.Concat("'" + Conversions.ToString(dateTime.AddDays(num2).Date), "'"));
					num2++;
				}
				right = Operators.ConcatenateObject(right, ")");
			}
			if (Operators.ConditionalCompareObjectNotEqual(filter, "", TextCompare: false))
			{
				filter = (string)Operators.ConcatenateObject(Operators.ConcatenateObject(" and Room_Type='", filter), "'");
			}
			if (Operators.ConditionalCompareObjectNotEqual(room_no, "", TextCompare: false))
			{
				room_no = (string)Operators.ConcatenateObject(Operators.ConcatenateObject(" and Room_no='", room_no), "'");
			}
			if (Operators.CompareString(noRoom, "", TextCompare: false) != 0)
			{
				noRoom = " and Room_no not in (" + noRoom + ") ";
			}
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms where Room_Manternace='no' and room_no not in (select room_no from HT_Room_Status where ", right), ") "), filter), noRoom), room_no), " order by Room_no")));
			ListView1.Items.Clear();
			CheckBox1.Checked = false;
			int num5 = dataSet.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					ListView listView = ListView1;
					listView.Items.Add(Conversions.ToString(num6 + 1));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[num6].SubItems;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num6];
					DataRow dataRow2 = dataRow;
					string columnName = "Room_no";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[num6].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num6];
					DataRow dataRow3 = dataRow;
					columnName = "Room_Type";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[num6].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num6];
					DataRow dataRow4 = dataRow;
					columnName = "Room_Details";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems3, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[num6].SubItems;
					array3 = new object[1];
					object[] array7 = array3;
					dataRow = dataSet.Tables[0].Rows[num6];
					DataRow dataRow5 = dataRow;
					columnName = "Room_PriceA";
					array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
					array = array3;
					object[] arguments4 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems4, null, "Add", arguments4, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[num6].SubItems;
					array3 = new object[1];
					object[] array8 = array3;
					dataRow = dataSet.Tables[0].Rows[num6];
					DataRow dataRow6 = dataRow;
					columnName = "Room_PriceB";
					array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
					array = array3;
					object[] arguments5 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems5, null, "Add", arguments5, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems6 = listView.Items[num6].SubItems;
					array3 = new object[1];
					object[] array9 = array3;
					dataRow = dataSet.Tables[0].Rows[num6];
					DataRow dataRow7 = dataRow;
					columnName = "Room_PriceC";
					array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
					array = array3;
					object[] arguments6 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems6, null, "Add", arguments6, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num6]["Room_Clean"], "yes", TextCompare: false))
					{
						listView.Items[num6].BackColor = Color.AntiqueWhite;
						listView.Items[num6].SubItems.Add("รอทำความสะอาด");
					}
					else if (Operators.CompareString(dataSet.Tables[0].Rows[num6]["Room_book"].ToString(), "", TextCompare: false) != 0)
					{
						listView.Items[num6].SubItems.Add("จอง (" + dataSet.Tables[0].Rows[num6]["Room_Book_Name"].ToString() + " เวลา " + dataSet.Tables[0].Rows[num6]["Room_Book_Time"].ToString() + ")");
						listView.Items[num6].BackColor = Color.Yellow;
					}
					else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num6]["Room_Use"], "no", TextCompare: false))
					{
						listView.Items[num6].SubItems.Add("ว\u0e48าง");
						listView.Items[num6].BackColor = Color.White;
					}
					else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[num6]["Room_Use"], "yes", TextCompare: false))
					{
						listView.Items[num6].BackColor = Color.Aquamarine;
						listView.Items[num6].SubItems.Add("ย\u0e31งไม\u0e48ได\u0e49 Check-Out");
					}
					listView = null;
					num6++;
					continue;
				}
				break;
			}
		}
	}

	private void TextBox1_TextChanged(object sender, EventArgs e)
	{
		Search();
	}

	private void ListView1_ItemChecked(object sender, ItemCheckedEventArgs e)
	{
		if (e.Item.Checked)
		{
			if ((ListView1.Items[e.Item.Index].BackColor != Color.White) & (ListView1.Items[e.Item.Index].BackColor != Color.Yellow))
			{
				MessageBox.Show("ไม\u0e48สามารถใช\u0e49ห\u0e49องน\u0e35\u0e49ได\u0e49กร\u0e38ณาตรวจสอบ");
				e.Item.Checked = false;
			}
			else
			{
				ListView1.Items[e.Item.Index].ForeColor = Color.Blue;
			}
		}
		else
		{
			ListView1.Items[e.Item.Index].ForeColor = Color.Black;
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		checked
		{
			if (ListView1.SelectedItems.Count != 0)
			{
				ListView2.Items.Clear();
				DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
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
					ListView.ListViewItemCollection items = ListView2.Items;
					object[] array = new object[1];
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					string columnName = "name";
					array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", array2, null, null, array3, IgnoreReturn: true);
					if (array3[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
					}
					ListView2.Items[num2].SubItems.Add(Conversions.ToString(0));
					num2++;
				}
				dataSet = Module1.connect("select * from HT_Rooms_Price where Room_type='" + ListView1.SelectedItems[0].SubItems[2].Text + "'");
				int num5 = ListView2.Items.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					int num8 = dataSet.Tables[0].Rows.Count - 1;
					int num9 = 0;
					while (true)
					{
						int num10 = num9;
						num4 = num8;
						if (num10 > num4)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(ListView2.Items[num6].SubItems[0].Text, dataSet.Tables[0].Rows[num9]["Room_CustType"], TextCompare: false))
						{
							ListView2.Items[num6].SubItems[1].Text = Conversions.ToString(dataSet.Tables[0].Rows[num9]["Room_Price"]);
						}
						num9++;
					}
					num6++;
				}
			}
			else
			{
				ListView2.Items.Clear();
			}
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		SelectNO_ARR.Clear();
		checked
		{
			int num = ListView1.CheckedItems.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				SelectNO_ARR.Add(ListView1.CheckedItems[num2].SubItems[1].Text);
				num2++;
			}
			Close();
		}
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
		checked
		{
			if (CheckBox1.Checked)
			{
				int num = ListView1.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						if (ListView1.Items[num2].BackColor == Color.White)
						{
							ListView1.Items[num2].Checked = true;
						}
						num2++;
						continue;
					}
					break;
				}
				return;
			}
			int num5 = ListView1.Items.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					ListView1.Items[num6].Checked = false;
					num6++;
					continue;
				}
				break;
			}
		}
	}

	private void DateTimePicker2_ValueChanged(object sender, EventArgs e)
	{
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
	}
}
