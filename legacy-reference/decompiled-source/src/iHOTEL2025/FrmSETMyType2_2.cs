using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.Editors;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmSETMyType2_2 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ComboItem1")]
	private ComboItem _ComboItem1;

	[AccessedThroughProperty("ComboItem2")]
	private ComboItem _ComboItem2;

	[AccessedThroughProperty("TabItem4")]
	private TabItem _TabItem4;

	[AccessedThroughProperty("GroupBox1")]
	private GroupBox _GroupBox1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("ContextMenuStrip1")]
	private ContextMenuStrip _ContextMenuStrip1;

	[AccessedThroughProperty("ลบToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_0;

	[AccessedThroughProperty("แกไขToolStripMenuItem")]
	private ToolStripMenuItem toolStripMenuItem_1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("TextBox2")]
	private TextBox _TextBox2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("TextBox3")]
	private TextBox _TextBox3;

	private string Badd;

	private string Bupdate;

	private string OLD_ID;

	internal virtual ComboItem ComboItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem1 = value;
		}
	}

	internal virtual ComboItem ComboItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ComboItem2 = value;
		}
	}

	internal virtual TabItem TabItem4
	{
		[DebuggerNonUserCode]
		get
		{
			return _TabItem4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TabItem4 = value;
		}
	}

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

	internal virtual PanelEx PanelEx2
	{
		[DebuggerNonUserCode]
		get
		{
			return _PanelEx2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_PanelEx2 = value;
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
			_TextBox1 = value;
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
			EventHandler value2 = Button2_Click;
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

	internal virtual ToolStripMenuItem ToolStripMenuItem_1
	{
		[DebuggerNonUserCode]
		get
		{
			return toolStripMenuItem_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ToolStripMenuItem_1_Click;
			if (toolStripMenuItem_1 != null)
			{
				toolStripMenuItem_1.Click -= value2;
			}
			toolStripMenuItem_1 = value;
			if (toolStripMenuItem_1 != null)
			{
				toolStripMenuItem_1.Click += value2;
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

	internal virtual TextBox TextBox2
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox2 = value;
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

	internal virtual TextBox TextBox3
	{
		[DebuggerNonUserCode]
		get
		{
			return _TextBox3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_TextBox3 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmSETMyType2_2()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmSETMyType2_2()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += add_seeds_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		Badd = "เพ\u0e34\u0e48ม";
		Bupdate = "บ\u0e31นท\u0e36ก";
		OLD_ID = "";
		InitializeComponent();
	}

	[DebuggerNonUserCode]
	protected override void Dispose(bool disposing)
	{
		if (disposing && components != null)
		{
			components.Dispose();
		}
		base.Dispose(disposing);
	}

	[System.Diagnostics.DebuggerStepThrough]
	private void InitializeComponent()
	{
		this.components = new System.ComponentModel.Container();
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripMenuItem_1 = new System.Windows.Forms.ToolStripMenuItem();
		this.TextBox2 = new System.Windows.Forms.TextBox();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Label3 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.TextBox3 = new System.Windows.Forms.TextBox();
		this.GroupBox1.SuspendLayout();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.TextBox3);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.ListView1);
		this.GroupBox1.Controls.Add(this.TextBox2);
		this.GroupBox1.Controls.Add(this.TextBox1);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(9, 36);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(498, 369);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 12;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(13, 333);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(41, 17);
		label2.Size = size;
		this.Label1.TabIndex = 47;
		this.Label1.Text = "รห\u0e31ส :";
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(157, 333);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(83, 17);
		label4.Size = size;
		this.Label2.TabIndex = 47;
		this.Label2.Text = "ช\u0e37\u0e48อประเภท  :";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[3] { this.ColumnHeader3, this.ColumnHeader1, this.ColumnHeader2 });
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(14, 47);
		listView.Location = location;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(470, 274);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "id";
		this.ColumnHeader3.Width = 0;
		this.ColumnHeader1.Text = "รห\u0e31ส";
		this.ColumnHeader1.Width = 100;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อประเภท";
		this.ColumnHeader2.Width = 250;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[2] { this.ToolStripMenuItem_0, this.ToolStripMenuItem_1 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(109, 48);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "ลบToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(108, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ลบ";
		this.ToolStripMenuItem_1.Name = "แกไขToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem2 = this.ToolStripMenuItem_1;
		size = new System.Drawing.Size(108, 22);
		toolStripMenuItem2.Size = size;
		this.ToolStripMenuItem_1.Text = "แก\u0e49ไข";
		this.TextBox2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox textBox = this.TextBox2;
		location = new System.Drawing.Point(96, 330);
		textBox.Location = location;
		this.TextBox2.Name = "TextBox2";
		System.Windows.Forms.TextBox textBox2 = this.TextBox2;
		size = new System.Drawing.Size(51, 24);
		textBox2.Size = size;
		this.TextBox2.TabIndex = 1;
		this.TextBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox textBox3 = this.TextBox1;
		location = new System.Drawing.Point(246, 330);
		textBox3.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox4 = this.TextBox1;
		size = new System.Drawing.Size(161, 24);
		textBox4.Size = size;
		this.TextBox1.TabIndex = 2;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(411, 331);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(64, 23);
		button2.Size = size;
		this.Button1.TabIndex = 3;
		this.Button1.Text = "เพ\u0e34\u0e48ม";
		this.Button1.UseVisualStyleBackColor = true;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		size = new System.Drawing.Size(517, 32);
		panelEx2.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Blue;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Navy;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "ต\u0e31\u0e49งค\u0e48าประเภทบ\u0e31ญช\u0e35";
		this.Label3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(14, 23);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(84, 17);
		label6.Size = size;
		this.Label3.TabIndex = 47;
		this.Label3.Text = "เล\u0e37อกหมวด  :";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(99, 19);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(184, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 48;
		this.TextBox3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.TextBox3.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		System.Windows.Forms.TextBox textBox5 = this.TextBox3;
		location = new System.Drawing.Point(62, 330);
		textBox5.Location = location;
		this.TextBox3.Name = "TextBox3";
		this.TextBox3.ReadOnly = true;
		System.Windows.Forms.TextBox textBox6 = this.TextBox3;
		size = new System.Drawing.Size(35, 24);
		textBox6.Size = size;
		this.TextBox3.TabIndex = 49;
		this.TextBox3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(517, 416);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox1);
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmSETMyType2_2";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "ต\u0e31\u0e49งค\u0e48าประเภทบ\u0e31ญช\u0e35";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void add_seeds_Load(object sender, EventArgs e)
	{
		list_group();
	}

	public void list_group()
	{
		DataSet dataSet = Module1.connect("select * from TB_SET_MyType2 order by id_full");
		ComboBox1.Items.Clear();
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
				ComboBox1.Items.Add(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet.Tables[0].Rows[num2]["id_full"], "|"), dataSet.Tables[0].Rows[num2]["name"]));
				num2++;
			}
			if (ComboBox1.Items.Count != 0)
			{
				ComboBox1.SelectedIndex = 0;
			}
		}
	}

	public void load_type()
	{
		Button1.Text = Conversions.ToString(Badd);
		TextBox1.Text = "";
		TextBox2.Text = "";
		ListView1.Items.Clear();
		TextBox3.Text = ComboBox1.Text.Substring(0, ComboBox1.Text.IndexOf("|"));
		DataSet dataSet = Module1.connect("Select * From TB_SET_MyType2_2 where id_full like '" + TextBox3.Text + "%' order by id_full");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			return;
		}
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
					int count = listView.Items.Count;
					ListView.ListViewItemCollection items = listView.Items;
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow2 = dataRow;
					string columnName = "id";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow3 = dataRow;
					columnName = "id_full";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[count].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "name";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
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

	private void Button2_Click(object sender, EventArgs e)
	{
		if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อหมวด");
			return;
		}
		if (Operators.CompareString(TextBox2.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกรห\u0e31สหมวด");
			return;
		}
		if (Operators.ConditionalCompareObjectEqual(Button1.Text, Badd, TextCompare: false))
		{
			if (Module1.connect("SELECT * From TB_SET_MyType2_2 where name like '" + TextBox1.Text + "'").Tables[0].Rows.Count != 0)
			{
				MessageBox.Show("ม\u0e35ช\u0e37\u0e48อหมวดน\u0e35\u0e49อย\u0e39\u0e48แล\u0e49ว");
			}
			else
			{
				object obj = "";
				obj = "INSERT INTO [TB_SET_MyType2_2]";
				obj = Operators.ConcatenateObject(obj, "([Name],[id_full])");
				obj = Operators.ConcatenateObject(obj, "VALUES");
				obj = Operators.ConcatenateObject(obj, "(");
				obj = Operators.ConcatenateObject(obj, string.Concat("'" + TextBox1.Text, "'"));
				obj = Operators.ConcatenateObject(obj, string.Concat(string.Concat(",'" + TextBox3.Text, TextBox2.Text), "'"));
				obj = Operators.ConcatenateObject(obj, ")");
				Module1.connect(Conversions.ToString(obj));
				MessageBox.Show("เพ\u0e34\u0e48มช\u0e37\u0e48อหมวดเร\u0e35ยบร\u0e49อย");
			}
		}
		else
		{
			Module1.connect(string.Format("Update TB_SET_MyType2_2 SET name = '{0}',id_full='{2}' WHERE id={1}", TextBox1.Text, OLD_ID, TextBox3.Text + TextBox2.Text));
			MessageBox.Show("อ\u0e31บเดทเสร\u0e47จเร\u0e35ยบร\u0e49อย");
		}
		try
		{
			load_type();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
		TextBox2.Enabled = true;
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		DEL_item();
	}

	public void DEL_item()
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			DataSet dataSet = Module1.connect("select * from TB_SET_MyType3 where id_full like '" + ListView1.SelectedItems[0].SubItems[1].Text + "%'");
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				MessageBox.Show("รห\u0e31ส " + ListView1.SelectedItems[0].SubItems[1].Text + " ไม\u0e48สามารถลบได\u0e49เน\u0e37\u0e48องจากรห\u0e31สบ\u0e31ญช\u0e35ย\u0e31งไม\u0e48ได\u0e49ลบ");
			}
			else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบ " + ListView1.SelectedItems[0].SubItems[2].Text + " หร\u0e37อไม\u0e48", "ลบรายการ", MessageBoxButtons.YesNo, MessageBoxIcon.Exclamation) == DialogResult.Yes)
			{
				Module1.connect("delete from TB_SET_MyType2_2 where id=" + ListView1.SelectedItems[0].SubItems[0].Text);
				MessageBox.Show("ลบรายการเสร\u0e47จเร\u0e35ยบร\u0e49อย", "ลบรายการ", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
				try
				{
					load_type();
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
			}
		}
		else
		{
			MessageBox.Show("ไม\u0e48ม\u0e35รายการท\u0e35\u0e48เล\u0e37อก", "ลบรายการ", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
	}

	private void ToolStripMenuItem_1_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			DataSet dataSet = Module1.connect("select * from TB_SET_MyType3 where id_full like '" + ListView1.SelectedItems[0].SubItems[1].Text + "%'");
			if (dataSet.Tables[0].Rows.Count != 0)
			{
				TextBox2.Enabled = false;
			}
			else
			{
				TextBox2.Enabled = true;
			}
			OLD_ID = ListView1.SelectedItems[0].SubItems[0].Text;
			Button1.Text = Conversions.ToString(Bupdate);
			TextBox2.Text = ListView1.SelectedItems[0].SubItems[1].Text.Substring(TextBox3.Text.Length);
			TextBox1.Text = ListView1.SelectedItems[0].SubItems[2].Text;
		}
		else
		{
			MessageBox.Show("ไม\u0e48ม\u0e35รายการท\u0e35\u0e48เล\u0e37อก", "แก\u0e49ไขรายการ", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		ToolStripMenuItem_1_Click(null, null);
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		try
		{
			load_type();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}
}
