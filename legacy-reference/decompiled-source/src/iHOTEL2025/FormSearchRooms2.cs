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
public class FormSearchRooms2 : Office2007Form
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

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader10")]
	private ColumnHeader _ColumnHeader10;

	[AccessedThroughProperty("ColumnHeader11")]
	private ColumnHeader _ColumnHeader11;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Lnum")]
	private Label _Lnum;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	public string SelectNO;

	public DateTime D1;

	public DateTime D2;

	public int D3;

	public int DN;

	public string NotIn;

	public int N_room;

	public int USEROOM;

	private int RoomCIN;

	public ArrayList Room;

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
			EventHandler value3 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick -= value2;
				_ListView1.SelectedIndexChanged -= value3;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.DoubleClick += value2;
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

	internal virtual Label Lnum
	{
		[DebuggerNonUserCode]
		get
		{
			return _Lnum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Lnum = value;
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
	static FormSearchRooms2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSearchRooms2()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormSearchLuandery_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		SelectNO = "";
		DN = 1;
		NotIn = "";
		N_room = Module1.Maximum_Book;
		USEROOM = 0;
		RoomCIN = 0;
		Room = new ArrayList();
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
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.Button4 = new System.Windows.Forms.Button();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader10 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader11 = new System.Windows.Forms.ColumnHeader();
		this.Label2 = new System.Windows.Forms.Label();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.Label4 = new System.Windows.Forms.Label();
		this.Lnum = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
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
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[7] { this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader5, this.ColumnHeader4, this.ColumnHeader7, this.ColumnHeader2, this.ColumnHeader3 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(17, 41);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(742, 273);
		listView2.Size = size;
		this.ListView1.TabIndex = 4;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ลำด\u0e31บ";
		this.ColumnHeader1.Width = 50;
		this.ColumnHeader6.Text = "ประเภทห\u0e49อง";
		this.ColumnHeader6.Width = 140;
		this.ColumnHeader5.Text = "ราคา A";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 0;
		this.ColumnHeader4.Text = "ราคา B";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 0;
		this.ColumnHeader7.Text = "ราคา C";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 0;
		this.ColumnHeader2.Text = "จำนวนรวม";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader2.Width = 80;
		this.ColumnHeader3.Text = "จำนวนว\u0e48าง";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader3.Width = 80;
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
		this.ListView2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[2] { this.ColumnHeader10, this.ColumnHeader11 });
		this.ListView2.FullRowSelect = true;
		this.ListView2.GridLines = true;
		System.Windows.Forms.ListView listView3 = this.ListView2;
		location = new System.Drawing.Point(17, 336);
		listView3.Location = location;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView4 = this.ListView2;
		size = new System.Drawing.Size(742, 109);
		listView4.Size = size;
		this.ListView2.TabIndex = 6;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader10.Text = "ประเภทล\u0e39กค\u0e49า";
		this.ColumnHeader10.Width = 150;
		this.ColumnHeader11.Text = "ราคา";
		this.ColumnHeader11.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader11.Width = 100;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(19, 317);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(34, 16);
		label4.Size = size;
		this.Label2.TabIndex = 7;
		this.Label2.Text = "ราคา";
		this.Label1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label1.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(273, 3);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(486, 35);
		label6.Size = size;
		this.Label1.TabIndex = 5;
		this.Label1.Text = "ห\u0e49องว\u0e48างระหว\u0e48างว\u0e31นท\u0e35\u0e48 ";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label3.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label7 = this.Label3;
		location = new System.Drawing.Point(19, 448);
		label7.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label8 = this.Label3;
		size = new System.Drawing.Size(182, 35);
		label8.Size = size;
		this.Label3.TabIndex = 8;
		this.Label3.Text = "จำนวนท\u0e35\u0e48จองได\u0e49ต\u0e48อว\u0e31น";
		this.Label3.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label4.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label9 = this.Label4;
		location = new System.Drawing.Point(220, 449);
		label9.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label10 = this.Label4;
		size = new System.Drawing.Size(47, 35);
		label10.Size = size;
		this.Label4.TabIndex = 9;
		this.Label4.Text = "ห\u0e49อง";
		this.Label4.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Lnum.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Lnum.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label lnum = this.Lnum;
		location = new System.Drawing.Point(302, 446);
		lnum.Location = location;
		this.Lnum.Name = "Lnum";
		System.Windows.Forms.Label lnum2 = this.Lnum;
		size = new System.Drawing.Size(62, 35);
		lnum2.Size = size;
		this.Lnum.TabIndex = 10;
		this.Lnum.Text = "ห\u0e49อง";
		this.Lnum.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label5.ForeColor = System.Drawing.Color.Green;
		System.Windows.Forms.Label label11 = this.Label5;
		location = new System.Drawing.Point(177, 449);
		label11.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label12 = this.Label5;
		size = new System.Drawing.Size(47, 35);
		label12.Size = size;
		this.Label5.TabIndex = 9;
		this.Label5.Text = "ห\u0e49อง";
		this.Label5.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label6.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label13 = this.Label6;
		location = new System.Drawing.Point(260, 449);
		label13.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label14 = this.Label6;
		size = new System.Drawing.Size(47, 35);
		label14.Size = size;
		this.Label6.TabIndex = 9;
		this.Label6.Text = "เหล\u0e37อ";
		this.Label6.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label7.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label15 = this.Label7;
		location = new System.Drawing.Point(365, 448);
		label15.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label16 = this.Label7;
		size = new System.Drawing.Size(47, 35);
		label16.Size = size;
		this.Label7.TabIndex = 9;
		this.Label7.Text = "ห\u0e49อง";
		this.Label7.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(771, 486);
		this.ClientSize = size;
		this.Controls.Add(this.Lnum);
		this.Controls.Add(this.Label5);
		this.Controls.Add(this.Label6);
		this.Controls.Add(this.Label7);
		this.Controls.Add(this.Label4);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.ListView2);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.Label8);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.SizableToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormSearchRooms2";
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
		Label1.Text = "ห\u0e49องว\u0e48างระหว\u0e48างว\u0e31นท\u0e35\u0e48 " + Strings.Format(D1, "[dd/MM/yyyy HH:mm]") + " ถ\u0e36งว\u0e31นท\u0e35\u0e48 " + Strings.Format(D2, "[dd/MM/yyyy HH:mm]") + "";
		Label5.Text = Conversions.ToString(Module1.Maximum_Book);
		SelectNO = "";
		RoomCIN = 0;
		Search();
	}

	public void Search()
	{
		object right = "";
		if (Operators.CompareString(NotIn, "", TextCompare: false) != 0)
		{
			right = " and Book_No<>'" + NotIn + "'";
		}
		DataSet dataSet = Module1.connect("select name,Room_PriceA,Room_PriceB,Room_PriceC, (select count(Room_type) from HT_Rooms where Room_type=HT_SET_RoomType.name and Room_Manternace='no') as Ctotal  from HT_SET_RoomType order by name");
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
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[num2].SubItems;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow2 = dataRow;
				string columnName = "name";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				listView.Items[num2].SubItems.Add(Conversions.ToString(0));
				listView.Items[num2].SubItems.Add(Conversions.ToString(0));
				listView.Items[num2].SubItems.Add(Conversions.ToString(0));
				ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[num2].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow3 = dataRow;
				columnName = "Ctotal";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems2, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				DateTime dateTime = D1;
				if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(D1, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(D1, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
				{
					dateTime = dateTime.AddDays(-1.0);
				}
				int num5 = 0;
				if ((D3 == 0) | (D3 == 1))
				{
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select COALESCE(SUM(Book_Num),0)  AS cc from View_Book_Date where book_status<>'เข\u0e49าพ\u0e31ก' and Book_type='", dataSet.Tables[0].Rows[num2]["name"]), "' "), right), " and book_date_ds='"), dateTime.Date), "' and book_use=0")));
					DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select count(room_no)  AS cc  from View_Room_All where Room_Type='", dataSet.Tables[0].Rows[num2]["name"]), "' and room_date='"), dateTime.Date), "' ")));
					RoomCIN = Conversions.ToInteger(dataSet2.Tables[0].Rows[0]["cc"]);
					num5 = Conversions.ToInteger(Operators.AddObject(dataSet2.Tables[0].Rows[0]["cc"], dataSet3.Tables[0].Rows[0]["cc"]));
				}
				else if (D3 > 1)
				{
					int d = D3;
					int num6 = 1;
					while (true)
					{
						int num7 = num6;
						num4 = d;
						if (num7 > num4)
						{
							break;
						}
						DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select COALESCE(SUM(Book_Num),0) AS cc from View_Book_Date where book_status<>'เข\u0e49าพ\u0e31ก' and Book_type='", dataSet.Tables[0].Rows[num2]["name"]), "' "), right), " and book_date_ds='"), dateTime.AddDays(num6 - 1).Date), "' and book_use=0")));
						DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select count(room_no)  AS cc  from View_Room_All where Room_Type='", dataSet.Tables[0].Rows[num2]["name"]), "' and room_date='"), dateTime.AddDays(num6 - 1).Date), "' ")));
						Type typeFromHandle = typeof(Math);
						array = new object[2]
						{
							num5,
							Operators.AddObject(dataSet4.Tables[0].Rows[0]["cc"], dataSet5.Tables[0].Rows[0]["cc"])
						};
						object[] arguments3 = array;
						array4 = new bool[2] { true, false };
						object value = NewLateBinding.LateGet(null, typeFromHandle, "Max", arguments3, null, null, array4);
						if (array4[0])
						{
							num5 = (int)Conversions.ChangeType(RuntimeHelpers.GetObjectValue(array[0]), typeof(int));
						}
						num5 = Conversions.ToInteger(value);
						Type typeFromHandle2 = typeof(Math);
						array3 = new object[2] { RoomCIN, null };
						object[] array6 = array3;
						dataRow = dataSet4.Tables[0].Rows[0];
						DataRow dataRow4 = dataRow;
						columnName = "cc";
						array6[1] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
						array = array3;
						object[] arguments4 = array;
						array4 = new bool[2] { true, true };
						object value2 = NewLateBinding.LateGet(null, typeFromHandle2, "Max", arguments4, null, null, array4);
						if (array4[0])
						{
							RoomCIN = (int)Conversions.ChangeType(RuntimeHelpers.GetObjectValue(array[0]), typeof(int));
						}
						if (array4[1])
						{
							dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[1]);
						}
						RoomCIN = Conversions.ToInteger(value2);
						num6++;
					}
				}
				int num8 = 0;
				int num9 = Room.Count - 1;
				int num10 = 0;
				while (true)
				{
					int num11 = num10;
					num4 = num9;
					if (num11 > num4)
					{
						break;
					}
					if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(Room[num10], new object[1] { 0 }, null), dataSet.Tables[0].Rows[num2]["name"], TextCompare: false))
					{
						num8 += Conversions.ToInteger(NewLateBinding.LateIndexGet(Room[num10], new object[1] { 1 }, null));
					}
					num10++;
				}
				NewLateBinding.LateCall(listView.Items[num2].SubItems, null, "Add", new object[1] { Operators.SubtractObject(Operators.SubtractObject(dataSet.Tables[0].Rows[num2]["Ctotal"], num5), num8) }, null, null, null, IgnoreReturn: true);
				USEROOM += RoomCIN + num8;
				if (Operators.ConditionalCompareObjectLessEqual(Operators.SubtractObject(Operators.SubtractObject(dataSet.Tables[0].Rows[num2]["Ctotal"], num5), num8), 0, TextCompare: false))
				{
					listView.Items[num2].ForeColor = Color.Red;
				}
				listView = null;
				num2++;
			}
			Lnum.Text = Conversions.ToString(N_room - USEROOM);
		}
	}

	private void TextBox1_TextChanged(object sender, EventArgs e)
	{
		Search();
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		if (ListView1.SelectedItems[0].ForeColor == Color.Red)
		{
			MessageBox.Show("ห\u0e49องพ\u0e31กประเภท " + ListView1.SelectedItems[0].SubItems[1].Text + " เต\u0e47ม");
			return;
		}
		Hide();
		object obj = Interaction.InputBox("กร\u0e38ณาใส\u0e48จำนวนห\u0e49อง", "กร\u0e38ณาใส\u0e48จำนวนห\u0e49อง", "1");
		if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			Show();
			return;
		}
		if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
		{
			MessageBox.Show("กร\u0e38ณาใส\u0e48จำนวนห\u0e49องเป\u0e47นต\u0e31วเลข");
			Show();
			return;
		}
		Show();
		if (decimal.Compare(new decimal(Conversions.ToInteger(obj)), Conversions.ToDecimal(ListView1.SelectedItems[0].SubItems[6].Text)) > 0)
		{
			MessageBox.Show("จำนวนห\u0e49องเหล\u0e37อเพ\u0e35ยง " + ListView1.SelectedItems[0].SubItems[6].Text + " ห\u0e49อง");
		}
		else if (decimal.Compare(new decimal(Conversions.ToInteger(obj)), Conversions.ToDecimal(Lnum.Text)) > 0)
		{
			MessageBox.Show("จำนวนห\u0e49องสำหร\u0e31บว\u0e31นท\u0e35\u0e48  " + Label1.Text + "\r\nเหล\u0e37อเพ\u0e35ยง " + Lnum.Text + " ห\u0e49อง");
		}
		else
		{
			DN = Conversions.ToInteger(obj);
			SelectNO = ListView1.SelectedItems[0].SubItems[1].Text;
			Close();
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
				dataSet = Module1.connect("select * from HT_Rooms_Price where Room_type='" + ListView1.SelectedItems[0].SubItems[1].Text + "'");
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
}
