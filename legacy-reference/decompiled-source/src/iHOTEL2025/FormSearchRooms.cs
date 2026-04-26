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
public class FormSearchRooms : Office2007Form
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

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

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
			if (_TextBox1 != null)
			{
				_TextBox1.TextChanged -= value2;
			}
			_TextBox1 = value;
			if (_TextBox1 != null)
			{
				_TextBox1.TextChanged += value2;
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
			EventHandler value2 = ListView1_DoubleClick;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick -= value2;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick += value2;
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

	[DebuggerNonUserCode]
	static FormSearchRooms()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSearchRooms()
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
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Label8 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.Button4 = new System.Windows.Forms.Button();
		this.SuspendLayout();
		this.TextBox1.BackColor = System.Drawing.Color.White;
		this.TextBox1.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		System.Drawing.Point location = new System.Drawing.Point(101, 12);
		textBox.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		System.Drawing.Size size = new System.Drawing.Size(166, 23);
		textBox2.Size = size;
		this.TextBox1.TabIndex = 2;
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label = this.Label8;
		location = new System.Drawing.Point(19, 15);
		label.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label2 = this.Label8;
		size = new System.Drawing.Size(76, 16);
		label2.Size = size;
		this.Label8.TabIndex = 0;
		this.Label8.Text = "เลขท\u0e35\u0e48ห\u0e49องพ\u0e31ก";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[7] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader6, this.ColumnHeader3, this.ColumnHeader5, this.ColumnHeader4, this.ColumnHeader7 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(17, 41);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(742, 402);
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
		this.Button4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button4;
		location = new System.Drawing.Point(691, 451);
		button.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button2 = this.Button4;
		size = new System.Drawing.Size(68, 23);
		button2.Size = size;
		this.Button4.TabIndex = 3;
		this.Button4.Text = "ป\u0e34ด";
		this.Button4.UseVisualStyleBackColor = true;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(771, 486);
		this.ClientSize = size;
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.Label8);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.SizableToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormSearchRooms";
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
			right = " where Room_Type like '%" + TextBox1.Text + "%'";
		}
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Rooms ", right), " order by Room_no")));
		ListView1.Items.Clear();
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
					ListView listView = ListView1;
					listView.Items.Add(Conversions.ToString(num2 + 1));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[num2].SubItems;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
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
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[num2].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
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
					ListViewItem.ListViewSubItemCollection subItems3 = listView.Items[num2].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
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
					ListViewItem.ListViewSubItemCollection subItems4 = listView.Items[num2].SubItems;
					array3 = new object[1];
					object[] array7 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
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
					ListViewItem.ListViewSubItemCollection subItems5 = listView.Items[num2].SubItems;
					array3 = new object[1];
					object[] array8 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
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
					ListViewItem.ListViewSubItemCollection subItems6 = listView.Items[num2].SubItems;
					array3 = new object[1];
					object[] array9 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
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
					listView = null;
					num2++;
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

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			SelectNO = ListView1.SelectedItems[0].SubItems[1].Text;
			Close();
		}
	}
}
