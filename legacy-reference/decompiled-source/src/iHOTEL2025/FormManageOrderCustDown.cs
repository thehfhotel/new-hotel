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

namespace iHOTEL2025;

[DesignerGenerated]
public class FormManageOrderCustDown : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ListView2")]
	private ListView _ListView2;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Panel1")]
	private Panel _Panel1;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("Button5")]
	private Button _Button5;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("ListViewMain")]
	private ListView _ListViewMain;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

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

	internal virtual Panel Panel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Panel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Panel1 = value;
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

	internal virtual Button Button5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button5_Click;
			if (_Button5 != null)
			{
				_Button5.Click -= value2;
			}
			_Button5 = value;
			if (_Button5 != null)
			{
				_Button5.Click += value2;
			}
		}
	}

	internal virtual PanelEx PanelEx1
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx1 = value;
		}
	}

	internal virtual ListView ListViewMain
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListViewMain;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListViewMain_SelectedIndexChanged;
			if (_ListViewMain != null)
			{
				_ListViewMain.SelectedIndexChanged -= value2;
			}
			_ListViewMain = value;
			if (_ListViewMain != null)
			{
				_ListViewMain.SelectedIndexChanged += value2;
			}
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

	[DebuggerNonUserCode]
	static FormManageOrderCustDown()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormManageOrderCustDown()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormManageOrderCust_Load;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormManageOrderCustDown));
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ListView2 = new System.Windows.Forms.ListView();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.Button5 = new System.Windows.Forms.Button();
		this.Button3 = new System.Windows.Forms.Button();
		this.Button4 = new System.Windows.Forms.Button();
		this.Panel1 = new System.Windows.Forms.Panel();
		this.Button2 = new System.Windows.Forms.Button();
		this.Button1 = new System.Windows.Forms.Button();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.ListViewMain = new System.Windows.Forms.ListView();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[1] { this.ColumnHeader1 });
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		System.Drawing.Point location = new System.Drawing.Point(12, 144);
		listView.Location = location;
		System.Windows.Forms.ListView listView2 = this.ListView1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView2.Margin = margin;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView3 = this.ListView1;
		System.Drawing.Size size = new System.Drawing.Size(208, 259);
		listView3.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ประเภทล\u0e39กค\u0e49า";
		this.ColumnHeader1.Width = 150;
		this.ListView2.Columns.AddRange(new System.Windows.Forms.ColumnHeader[2] { this.ColumnHeader2, this.ColumnHeader3 });
		this.ListView2.FullRowSelect = true;
		this.ListView2.GridLines = true;
		System.Windows.Forms.ListView listView4 = this.ListView2;
		location = new System.Drawing.Point(284, 144);
		listView4.Location = location;
		System.Windows.Forms.ListView listView5 = this.ListView2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listView5.Margin = margin;
		this.ListView2.MultiSelect = false;
		this.ListView2.Name = "ListView2";
		System.Windows.Forms.ListView listView6 = this.ListView2;
		size = new System.Drawing.Size(278, 237);
		listView6.Size = size;
		this.ListView2.TabIndex = 0;
		this.ListView2.UseCompatibleStateImageBehavior = false;
		this.ListView2.View = System.Windows.Forms.View.Details;
		this.ColumnHeader2.Text = "ประเภทล\u0e39กค\u0e49า";
		this.ColumnHeader2.Width = 130;
		this.ColumnHeader3.Text = "ไม\u0e48มาพ\u0e31กภายใน (ว\u0e31น)";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader3.Width = 140;
		System.Windows.Forms.Button button = this.Button5;
		location = new System.Drawing.Point(438, 386);
		button.Location = location;
		this.Button5.Name = "Button5";
		System.Windows.Forms.Button button2 = this.Button5;
		size = new System.Drawing.Size(75, 23);
		button2.Size = size;
		this.Button5.TabIndex = 6;
		this.Button5.Text = "บ\u0e31นท\u0e36ก";
		this.Button5.UseVisualStyleBackColor = true;
		this.Button3.Image = (System.Drawing.Image)resources.GetObject("Button3.Image");
		System.Windows.Forms.Button button3 = this.Button3;
		location = new System.Drawing.Point(540, 387);
		button3.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button4 = this.Button3;
		size = new System.Drawing.Size(23, 21);
		button4.Size = size;
		this.Button3.TabIndex = 5;
		this.Button3.UseVisualStyleBackColor = true;
		this.Button4.Image = (System.Drawing.Image)resources.GetObject("Button4.Image");
		System.Windows.Forms.Button button5 = this.Button4;
		location = new System.Drawing.Point(518, 387);
		button5.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button6 = this.Button4;
		size = new System.Drawing.Size(23, 21);
		button6.Size = size;
		this.Button4.TabIndex = 4;
		this.Button4.UseVisualStyleBackColor = true;
		this.Panel1.BackgroundImage = (System.Drawing.Image)resources.GetObject("Panel1.BackgroundImage");
		this.Panel1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Center;
		System.Windows.Forms.Panel panel = this.Panel1;
		location = new System.Drawing.Point(225, 150);
		panel.Location = location;
		this.Panel1.Name = "Panel1";
		System.Windows.Forms.Panel panel2 = this.Panel1;
		size = new System.Drawing.Size(53, 42);
		panel2.Size = size;
		this.Panel1.TabIndex = 2;
		this.Button2.Image = (System.Drawing.Image)resources.GetObject("Button2.Image");
		System.Windows.Forms.Button button7 = this.Button2;
		location = new System.Drawing.Point(225, 261);
		button7.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button8 = this.Button2;
		size = new System.Drawing.Size(52, 54);
		button8.Size = size;
		this.Button2.TabIndex = 1;
		this.Button2.UseVisualStyleBackColor = true;
		this.Button1.Image = (System.Drawing.Image)resources.GetObject("Button1.Image");
		System.Windows.Forms.Button button9 = this.Button1;
		location = new System.Drawing.Point(225, 201);
		button9.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button10 = this.Button1;
		size = new System.Drawing.Size(52, 54);
		button10.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.UseVisualStyleBackColor = true;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.ListViewMain);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(12, 12);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(551, 124);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.Color = System.Drawing.Color.LavenderBlush;
		this.PanelEx1.Style.BackColor2.Color = System.Drawing.Color.Pink;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 8;
		this.ListViewMain.Columns.AddRange(new System.Windows.Forms.ColumnHeader[1] { this.ColumnHeader4 });
		this.ListViewMain.FullRowSelect = true;
		this.ListViewMain.GridLines = true;
		System.Windows.Forms.ListView listViewMain = this.ListViewMain;
		location = new System.Drawing.Point(6, 6);
		listViewMain.Location = location;
		System.Windows.Forms.ListView listViewMain2 = this.ListViewMain;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listViewMain2.Margin = margin;
		this.ListViewMain.Name = "ListViewMain";
		System.Windows.Forms.ListView listViewMain3 = this.ListViewMain;
		size = new System.Drawing.Size(540, 113);
		listViewMain3.Size = size;
		this.ListViewMain.TabIndex = 1;
		this.ListViewMain.UseCompatibleStateImageBehavior = false;
		this.ListViewMain.View = System.Windows.Forms.View.Details;
		this.ColumnHeader4.Text = "ประเภทล\u0e39กค\u0e49า";
		this.ColumnHeader4.Width = 500;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(575, 413);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.Controls.Add(this.Button5);
		this.Controls.Add(this.Button3);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.Panel1);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.ListView2);
		this.Controls.Add(this.ListView1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormManageOrderCustDown";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เปล\u0e35\u0e48ยนระด\u0e31บล\u0e39กค\u0e49าอ\u0e31ตโนม\u0e31ต\u0e34 (ปร\u0e31บราคาข\u0e36\u0e49น)";
		this.PanelEx1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void FormManageOrderCust_Load(object sender, EventArgs e)
	{
		ListView1.Items.Clear();
		ListView2.Items.Clear();
		ListViewMain.Items.Clear();
		ListTypeMain();
	}

	public void ListTypeMain()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType_main order by name");
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
					ListView listViewMain = ListViewMain;
					ListView.ListViewItemCollection items = listViewMain.Items;
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
					listViewMain = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void load_type()
	{
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by name");
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
					ListView.ListViewItemCollection items = listView.Items;
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
					listView = null;
					num2++;
					continue;
				}
				break;
			}
		}
	}

	public void load_type2()
	{
		DataSet dataSet = Module1.connect("select * from HT_Order_DOwn where Cast_Type='" + ListViewMain.SelectedItems[0].SubItems[0].Text + "'  order by id");
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
				ListView listView = ListView2;
				ListView.ListViewItemCollection items = listView.Items;
				object[] array = new object[1];
				object[] array2 = array;
				DataRow dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow2 = dataRow;
				string columnName = "cust_type";
				array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
				object[] array3 = array;
				object[] arguments = array3;
				bool[] array4 = new bool[1] { true };
				NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
				}
				ListViewItem.ListViewSubItemCollection subItems = listView.Items[num2].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow3 = dataRow;
				columnName = "cust_month";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				listView = null;
				int num5 = ListView1.Items.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					num4 = num5;
					if (num7 <= num4)
					{
						if (!Operators.ConditionalCompareObjectEqual(ListView1.Items[num6].SubItems[0].Text, dataSet.Tables[0].Rows[num2]["cust_type"], TextCompare: false))
						{
							num6++;
							continue;
						}
						ListView1.Items[num6].Remove();
						break;
					}
					break;
				}
				num2++;
			}
		}
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		object obj = Interaction.InputBox("กร\u0e38ณากรอกจำนวนเด\u0e37อนท\u0e35\u0e48ต\u0e49องการเปล\u0e35\u0e48ยนลำด\u0e31บ", "กร\u0e38ณากรอกจำนวนเด\u0e37อนท\u0e35\u0e48ต\u0e49องการเปล\u0e35\u0e48ยนลำด\u0e31บ", Conversions.ToString(1));
		if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false) || !Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
		{
			return;
		}
		checked
		{
			if (Operators.ConditionalCompareObjectEqual(obj, 0, TextCompare: false))
			{
				int num = ListView2.Items.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						if (!Operators.ConditionalCompareObjectEqual(ListView2.Items[num2].SubItems[1].Text, obj, TextCompare: false))
						{
							num2++;
							continue;
						}
						MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ม\u0e35ลำด\u0e31บจำนวน ", obj), " เด\u0e37อนอย\u0e39\u0e48แล\u0e49ว")));
						return;
					}
					break;
				}
			}
			ListView listView = ListView2;
			int count = listView.Items.Count;
			listView.Items.Add(ListView1.SelectedItems[0].SubItems[0].Text);
			ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
			object[] array = new object[1] { RuntimeHelpers.GetObjectValue(obj) };
			bool[] array2 = new bool[1] { true };
			NewLateBinding.LateCall(subItems, null, "Add", array, null, null, array2, IgnoreReturn: true);
			if (array2[0])
			{
				obj = RuntimeHelpers.GetObjectValue(array[0]);
			}
			listView = null;
			ListView1.SelectedItems[0].Remove();
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		if (ListView2.SelectedItems.Count != 0)
		{
			ListView listView = ListView1;
			_ = listView.Items.Count;
			listView.Items.Add(ListView2.SelectedItems[0].SubItems[0].Text);
			listView = null;
			ListView2.SelectedItems[0].Remove();
		}
	}

	private void Button5_Click(object sender, EventArgs e)
	{
		if (ListViewMain.SelectedItems.Count == 0)
		{
			return;
		}
		Module1.connect("delete from HT_Order_Down where Cast_Type='" + ListViewMain.SelectedItems[0].SubItems[0].Text + "'");
		checked
		{
			int num = ListView2.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					object left = "INSERT INTO [HT_Order_Down]";
					left = Operators.ConcatenateObject(left, "([id]");
					left = Operators.ConcatenateObject(left, ",[Cust_Type]");
					left = Operators.ConcatenateObject(left, ",[Cust_Month],[Cast_Type])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, num2 + 1);
					left = Operators.ConcatenateObject(left, string.Concat(",'" + ListView2.Items[num2].SubItems[0].Text, "'"));
					left = Operators.ConcatenateObject(left, "," + ListView2.Items[num2].SubItems[1].Text);
					left = Operators.ConcatenateObject(left, string.Concat(",'" + ListViewMain.SelectedItems[0].SubItems[0].Text, "'"));
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		checked
		{
			if (ListView2.SelectedItems.Count != 0 && ListView2.SelectedItems[0].Index != 0)
			{
				string text = ListView2.Items[ListView2.SelectedItems[0].Index - 1].SubItems[0].Text;
				string text2 = ListView2.Items[ListView2.SelectedItems[0].Index - 1].SubItems[1].Text;
				ListView2.Items[ListView2.SelectedItems[0].Index - 1].SubItems[0].Text = ListView2.SelectedItems[0].SubItems[0].Text;
				ListView2.Items[ListView2.SelectedItems[0].Index - 1].SubItems[1].Text = ListView2.SelectedItems[0].SubItems[1].Text;
				ListView2.SelectedItems[0].SubItems[0].Text = text;
				ListView2.SelectedItems[0].SubItems[1].Text = text2;
				ListView2.EnsureVisible(ListView2.SelectedItems[0].Index - 1);
				ListView2.Items[ListView2.SelectedItems[0].Index - 1].Selected = true;
			}
		}
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		checked
		{
			if (ListView2.SelectedItems.Count != 0 && ListView2.SelectedItems[0].Index != ListView2.Items.Count - 1)
			{
				string text = ListView2.Items[ListView2.SelectedItems[0].Index + 1].SubItems[0].Text;
				string text2 = ListView2.Items[ListView2.SelectedItems[0].Index + 1].SubItems[1].Text;
				ListView2.Items[ListView2.SelectedItems[0].Index + 1].SubItems[0].Text = ListView2.SelectedItems[0].SubItems[0].Text;
				ListView2.Items[ListView2.SelectedItems[0].Index + 1].SubItems[1].Text = ListView2.SelectedItems[0].SubItems[1].Text;
				ListView2.SelectedItems[0].SubItems[0].Text = text;
				ListView2.SelectedItems[0].SubItems[1].Text = text2;
				ListView2.EnsureVisible(ListView2.SelectedItems[0].Index + 1);
				ListView2.Items[ListView2.SelectedItems[0].Index + 1].Selected = true;
			}
		}
	}

	private void ListViewMain_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListViewMain.SelectedItems.Count == 0)
		{
			return;
		}
		checked
		{
			int num = ListViewMain.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ListViewMain.Items[num2].BackColor = Color.White;
				num2++;
			}
			ListView1.Items.Clear();
			ListView2.Items.Clear();
			ListViewMain.SelectedItems[0].BackColor = Color.Pink;
			load_type();
			load_type2();
		}
	}
}
