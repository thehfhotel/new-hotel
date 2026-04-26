using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using CrystalDecisions.CrystalReports.Engine;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using PrintableListView;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmReportRR4 : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("ColumnHeader5")]
	private ColumnHeader _ColumnHeader5;

	[AccessedThroughProperty("ColumnHeader6")]
	private ColumnHeader _ColumnHeader6;

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("ColumnHeader8")]
	private ColumnHeader _ColumnHeader8;

	[AccessedThroughProperty("ColumnHeader9")]
	private ColumnHeader _ColumnHeader9;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("RadioButton2")]
	private RadioButton _RadioButton2;

	[AccessedThroughProperty("DateTimePicker3")]
	private DateTimePicker _DateTimePicker3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("ColumnHeader12")]
	private ColumnHeader _ColumnHeader12;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

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
			EventHandler value2 = ListView1_SelectedIndexChanged;
			ItemCheckedEventHandler value3 = ListView1_ItemChecked;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
				_ListView1.ItemChecked -= value3;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
				_ListView1.ItemChecked += value3;
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

	internal virtual RadioButton RadioButton2
	{
		[DebuggerNonUserCode]
		get
		{
			return _RadioButton2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_RadioButton2 = value;
		}
	}

	internal virtual DateTimePicker DateTimePicker3
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = DateTimePicker3_ValueChanged;
			if (_DateTimePicker3 != null)
			{
				_DateTimePicker3.ValueChanged -= value2;
			}
			_DateTimePicker3 = value;
			if (_DateTimePicker3 != null)
			{
				_DateTimePicker3.ValueChanged += value2;
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
			EventHandler value2 = Button3_Click_1;
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

	[DebuggerNonUserCode]
	static FrmReportRR4()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmReportRR4()
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
		this.RadioButton2 = new System.Windows.Forms.RadioButton();
		this.DateTimePicker3 = new System.Windows.Forms.DateTimePicker();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.ListView1 = new global::PrintableListView.PrintableListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader8 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader9 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader12 = new System.Windows.Forms.ColumnHeader();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.Button3 = new System.Windows.Forms.Button();
		this.GroupBox1.SuspendLayout();
		this.SuspendLayout();
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.Controls.Add(this.RadioButton2);
		this.GroupBox1.Controls.Add(this.DateTimePicker3);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.Label3);
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
		this.GroupBox1.Text = "รายงานทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก";
		this.RadioButton2.AutoSize = true;
		this.RadioButton2.Checked = true;
		System.Windows.Forms.RadioButton radioButton = this.RadioButton2;
		location = new System.Drawing.Point(6, 30);
		radioButton.Location = location;
		this.RadioButton2.Name = "RadioButton2";
		System.Windows.Forms.RadioButton radioButton2 = this.RadioButton2;
		size = new System.Drawing.Size(14, 13);
		radioButton2.Size = size;
		this.RadioButton2.TabIndex = 10;
		this.RadioButton2.TabStop = true;
		this.RadioButton2.UseVisualStyleBackColor = true;
		this.DateTimePicker3.CustomFormat = "ddMMMMyyyy";
		this.DateTimePicker3.Format = System.Windows.Forms.DateTimePickerFormat.Custom;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker3;
		location = new System.Drawing.Point(86, 25);
		dateTimePicker.Location = location;
		this.DateTimePicker3.Name = "DateTimePicker3";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker3;
		size = new System.Drawing.Size(176, 24);
		dateTimePicker2.Size = size;
		this.DateTimePicker3.TabIndex = 8;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label = this.Label4;
		location = new System.Drawing.Point(26, 28);
		label.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label2 = this.Label4;
		size = new System.Drawing.Size(61, 17);
		label2.Size = size;
		this.Label4.TabIndex = 7;
		this.Label4.Text = "ค\u0e37นว\u0e31นท\u0e35\u0e48 :";
		this.Label3.AutoSize = true;
		this.Label3.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label3 = this.Label3;
		location = new System.Drawing.Point(87, 58);
		label3.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label4 = this.Label3;
		size = new System.Drawing.Size(34, 17);
		label4.Size = size;
		this.Label3.TabIndex = 6;
		this.Label3.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Atto_กระดาษแนวนอน = false;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[12]
		{
			this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader7, this.ColumnHeader4, this.ColumnHeader5, this.ColumnHeader10, this.ColumnHeader11, this.ColumnHeader6, this.ColumnHeader8,
			this.ColumnHeader9, this.ColumnHeader12
		});
		this.ListView1.FitToPage = true;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		global::PrintableListView.PrintableListView listView = this.ListView1;
		location = new System.Drawing.Point(6, 81);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		global::PrintableListView.PrintableListView listView2 = this.ListView1;
		size = new System.Drawing.Size(1033, 396);
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
		this.ColumnHeader2.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader2.Width = 110;
		this.ColumnHeader3.Text = "เลขห\u0e49องพ\u0e31ก";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader7.Text = "ช\u0e37\u0e48อ";
		this.ColumnHeader7.Width = 250;
		this.ColumnHeader4.Text = "ส\u0e31ญชาต\u0e34";
		this.ColumnHeader4.Width = 80;
		this.ColumnHeader5.Text = "เลขประจำต\u0e31ว";
		this.ColumnHeader5.Width = 100;
		this.ColumnHeader10.Text = "ท\u0e35\u0e48อย\u0e39\u0e48";
		this.ColumnHeader10.Width = 100;
		this.ColumnHeader11.Text = "อาช\u0e35พ";
		this.ColumnHeader11.Width = 100;
		this.ColumnHeader6.Text = "มาจาก";
		this.ColumnHeader6.Width = 110;
		this.ColumnHeader8.Text = "จะไปท\u0e35\u0e48";
		this.ColumnHeader8.Width = 110;
		this.ColumnHeader9.Text = "ว\u0e31นท\u0e35\u0e48ออก";
		this.ColumnHeader9.Width = 110;
		this.ColumnHeader12.Text = "หมายเหต\u0e38";
		this.ColumnHeader12.Width = 100;
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
		this.Button3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button5 = this.Button3;
		location = new System.Drawing.Point(13, 502);
		button5.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button6 = this.Button3;
		size = new System.Drawing.Size(134, 23);
		button6.Size = size;
		this.Button3.TabIndex = 4;
		this.Button3.Text = "ลบรายการท\u0e35\u0e48เล\u0e37อก";
		this.Button3.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1073, 539);
		this.ClientSize = size;
		this.Controls.Add(this.Button3);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.GroupBox1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmReportRR4";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก";
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
		Cursor = Cursors.WaitCursor;
		object left = "SELECT * from View_Report_RR4";
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		Conversions.ToDate(Operators.ConcatenateObject(Operators.ConcatenateObject(Conversions.ToString(DateTimePicker3.Value.Date) + " ", right), ":01"));
		Conversions.ToDate(Operators.ConcatenateObject(Operators.ConcatenateObject(Conversions.ToString(DateTimePicker3.Value.AddDays(1.0).Date) + " ", right2), ":00"));
		left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(" where (cin_room_in between '" + Conversions.ToString(DateTimePicker3.Value.Date), " "), right), ":01' and '"), DateTimePicker3.Value.AddDays(1.0).Date), " "), right2), ":00') and Cin_Status<>'ยกเล\u0e34ก'"));
		left = Operators.ConcatenateObject(left, " order by cin_room_in");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(left));
		ListView1.Items.Clear();
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
				string left2 = dataSet2.Tables[0].Rows[num2]["Cust_Contry"].ToString();
				if (Operators.CompareString(left2, "", TextCompare: false) == 0)
				{
					left2 = "ไทย";
				}
				global::PrintableListView.PrintableListView listView = ListView1;
				listView.Items.Add(Conversions.ToString(num2 + 1));
				listView.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["cin_room_in"]), "dd/MM/yyyy HH:mm น."));
				listView.Items[num2].SubItems.Add(dataSet2.Tables[0].Rows[num2]["cin_room_no"].ToString());
				listView.Items[num2].SubItems.Add(dataSet2.Tables[0].Rows[num2]["Cust_name"].ToString());
				listView.Items[num2].SubItems.Add(left2);
				listView.Items[num2].SubItems.Add(dataSet2.Tables[0].Rows[num2]["Cust_IDcard"].ToString());
				listView.Items[num2].SubItems.Add(Module1.Address_Rcplace(dataSet2.Tables[0].Rows[num2]["C_Address"].ToString()));
				listView.Items[num2].SubItems.Add("");
				listView.Items[num2].SubItems.Add("");
				listView.Items[num2].SubItems.Add("");
				listView.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num2]["cin_room_out"]), "dd/MM/yyyy HH:mm น."));
				listView.Items[num2].SubItems.Add("");
				listView = null;
				num2++;
			}
			Cursor = Cursors.Default;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void FrmReportImcome_Load(object sender, EventArgs e)
	{
		ListView1.Items.Clear();
		sumdate();
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
	}

	private void DateTimePicker3_ValueChanged(object sender, EventArgs e)
	{
		sumdate();
		search();
	}

	public void sumdate()
	{
		DataSet dataSet = Module1.connect("select * from TB_SETTINGS");
		object right = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		object right2 = Strings.Format(Conversions.ToInteger(dataSet.Tables[0].Rows[0]["CHK_IN_Before"]), "00:00");
		Label3.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("จากว\u0e31นท\u0e35\u0e48  " + Strings.Format(DateTimePicker3.Value, "dd/MM/yy"), " "), right), ":01 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 "), Strings.Format(DateTimePicker3.Value.AddDays(1.0), "dd/MM/yy")), " "), right2), ":00"));
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select CompanyName from TB_SETTINGS");
		Module1.localdata.GClass0_0.Rows.Clear();
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				Module1.localdata.GClass0_0.method_1(Conversions.ToString(dataSet.Tables[0].Rows[0]["CompanyName"]), Strings.Format(DateTimePicker3.Value, "dd/MM/yyyy"), ListView1.Items[num2].SubItems[0].Text, ListView1.Items[num2].SubItems[1].Text, ListView1.Items[num2].SubItems[2].Text, ListView1.Items[num2].SubItems[3].Text, ListView1.Items[num2].SubItems[4].Text, ListView1.Items[num2].SubItems[5].Text, ListView1.Items[num2].SubItems[6].Text, ListView1.Items[num2].SubItems[7].Text, ListView1.Items[num2].SubItems[8].Text, ListView1.Items[num2].SubItems[9].Text, ListView1.Items[num2].SubItems[10].Text, ListView1.Items[num2].SubItems[11].Text);
				num2++;
			}
			MyProject.Forms.FrmPrint.Close();
			ReportDocument reportDocument = new ReportDocument();
			if (File.Exists(Module1.Path_Program + "reports/Report_RR4.rpt"))
			{
				reportDocument.Load(Module1.Path_Program + "reports/Report_RR4.rpt");
			}
			else
			{
				reportDocument.Load(Module1.Path_Program + "/Report_RR4.rpt");
			}
			reportDocument.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.Show();
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = reportDocument;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void Button3_Click_1(object sender, EventArgs e)
	{
		if (ListView1.CheckedItems.Count == 0)
		{
			return;
		}
		int num = 0;
		checked
		{
			int num2 = ListView1.CheckedItems.Count - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				ListView1.CheckedItems[num3 - num].Remove();
				num++;
				num3++;
			}
			int num6 = ListView1.Items.Count - 1;
			int num7 = 0;
			while (true)
			{
				int num8 = num7;
				int num5 = num6;
				if (num8 <= num5)
				{
					ListView1.Items[num7].SubItems[0].Text = Conversions.ToString(num7 + 1);
					num7++;
					continue;
				}
				break;
			}
		}
	}

	private void ListView1_ItemChecked(object sender, ItemCheckedEventArgs e)
	{
		checked
		{
			int num = ListView1.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (ListView1.Items[num2].Checked)
					{
						ListView1.Items[num2].BackColor = Color.LightPink;
					}
					else
					{
						ListView1.Items[num2].BackColor = Color.White;
					}
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}
}
