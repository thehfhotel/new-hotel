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
public class FrmSETRoomType : Office2007Form
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

	[AccessedThroughProperty("DataGridView1")]
	private DataGridView _DataGridView1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("ColumnHeader4")]
	private ColumnHeader _ColumnHeader4;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Column1")]
	private DataGridViewTextBoxColumn _Column1;

	[AccessedThroughProperty("ราคา")]
	private DataGridViewTextBoxColumn dataGridViewTextBoxColumn_0;

	[AccessedThroughProperty("ราคาช\u0e31\u0e48วคราว")]
	private DataGridViewTextBoxColumn dataGridViewTextBoxColumn_1;

	[AccessedThroughProperty("ราคารายเด\u0e37อน")]
	private DataGridViewTextBoxColumn dataGridViewTextBoxColumn_2;

	private string Badd;

	private string Bupdate;

	private string Editid;

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
			EventHandler value2 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged -= value2;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.SelectedIndexChanged += value2;
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

	internal virtual DataGridView DataGridView1
	{
		[DebuggerNonUserCode]
		get
		{
			return _DataGridView1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DataGridView1 = value;
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
			_ComboBox1 = value;
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
			EventHandler value2 = Button2_Click_1;
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

	internal virtual DataGridViewTextBoxColumn Column1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Column1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Column1 = value;
		}
	}

	internal virtual DataGridViewTextBoxColumn DataGridViewTextBoxColumn_0
	{
		[DebuggerNonUserCode]
		get
		{
			return dataGridViewTextBoxColumn_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			dataGridViewTextBoxColumn_0 = value;
		}
	}

	internal virtual DataGridViewTextBoxColumn DataGridViewTextBoxColumn_1
	{
		[DebuggerNonUserCode]
		get
		{
			return dataGridViewTextBoxColumn_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			dataGridViewTextBoxColumn_1 = value;
		}
	}

	internal virtual DataGridViewTextBoxColumn DataGridViewTextBoxColumn_2
	{
		[DebuggerNonUserCode]
		get
		{
			return dataGridViewTextBoxColumn_2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			dataGridViewTextBoxColumn_2 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmSETRoomType()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmSETRoomType()
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
		Editid = "";
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
		System.Windows.Forms.DataGridViewCellStyle dataGridViewCellStyle = new System.Windows.Forms.DataGridViewCellStyle();
		System.Windows.Forms.DataGridViewCellStyle dataGridViewCellStyle2 = new System.Windows.Forms.DataGridViewCellStyle();
		System.Windows.Forms.DataGridViewCellStyle dataGridViewCellStyle3 = new System.Windows.Forms.DataGridViewCellStyle();
		this.ComboItem1 = new DevComponents.Editors.ComboItem();
		this.ComboItem2 = new DevComponents.Editors.ComboItem();
		this.TabItem4 = new DevComponents.DotNetBar.TabItem(this.components);
		this.GroupBox1 = new System.Windows.Forms.GroupBox();
		this.Button2 = new System.Windows.Forms.Button();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label3 = new System.Windows.Forms.Label();
		this.DataGridView1 = new System.Windows.Forms.DataGridView();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ContextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
		this.ToolStripMenuItem_0 = new System.Windows.Forms.ToolStripMenuItem();
		this.ToolStripMenuItem_1 = new System.Windows.Forms.ToolStripMenuItem();
		this.TextBox2 = new System.Windows.Forms.TextBox();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Column1 = new System.Windows.Forms.DataGridViewTextBoxColumn();
		this.DataGridViewTextBoxColumn_0 = new System.Windows.Forms.DataGridViewTextBoxColumn();
		this.DataGridViewTextBoxColumn_1 = new System.Windows.Forms.DataGridViewTextBoxColumn();
		this.DataGridViewTextBoxColumn_2 = new System.Windows.Forms.DataGridViewTextBoxColumn();
		this.GroupBox1.SuspendLayout();
		((System.ComponentModel.ISupportInitialize)this.DataGridView1).BeginInit();
		this.ContextMenuStrip1.SuspendLayout();
		this.SuspendLayout();
		this.ComboItem1.Text = "ชาย";
		this.ComboItem2.Text = "หญ\u0e34ง";
		this.TabItem4.Name = "TabItem4";
		this.TabItem4.Text = "ข\u0e49อม\u0e39ลสมาช\u0e34ก";
		this.GroupBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.GroupBox1.BackColor = System.Drawing.Color.Transparent;
		this.GroupBox1.Controls.Add(this.Button2);
		this.GroupBox1.Controls.Add(this.ComboBox1);
		this.GroupBox1.Controls.Add(this.Label4);
		this.GroupBox1.Controls.Add(this.Label3);
		this.GroupBox1.Controls.Add(this.DataGridView1);
		this.GroupBox1.Controls.Add(this.Label1);
		this.GroupBox1.Controls.Add(this.Label2);
		this.GroupBox1.Controls.Add(this.ListView1);
		this.GroupBox1.Controls.Add(this.TextBox2);
		this.GroupBox1.Controls.Add(this.TextBox1);
		this.GroupBox1.Controls.Add(this.Button1);
		this.GroupBox1.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.GroupBox groupBox = this.GroupBox1;
		System.Drawing.Point location = new System.Drawing.Point(9, 36);
		groupBox.Location = location;
		this.GroupBox1.Name = "GroupBox1";
		System.Windows.Forms.GroupBox groupBox2 = this.GroupBox1;
		System.Drawing.Size size = new System.Drawing.Size(614, 561);
		groupBox2.Size = size;
		this.GroupBox1.TabIndex = 12;
		this.GroupBox1.TabStop = false;
		this.GroupBox1.Text = "ค\u0e49นหา";
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.Button button = this.Button2;
		location = new System.Drawing.Point(529, 334);
		button.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button2 = this.Button2;
		size = new System.Drawing.Size(71, 24);
		button2.Size = size;
		this.Button2.TabIndex = 53;
		this.Button2.Text = "เคล\u0e35ยร\u0e4c";
		this.Button2.UseVisualStyleBackColor = true;
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[2] { "ไม\u0e48ม\u0e35", "ม\u0e35" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(102, 335);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(84, 24);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 52;
		this.Label4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label4.AutoSize = true;
		System.Windows.Forms.Label label = this.Label4;
		location = new System.Drawing.Point(31, 340);
		label.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label2 = this.Label4;
		size = new System.Drawing.Size(69, 16);
		label2.Size = size;
		this.Label4.TabIndex = 51;
		this.Label4.Text = "อาหารเช\u0e49า :";
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label3;
		location = new System.Drawing.Point(15, 368);
		label3.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label4 = this.Label3;
		size = new System.Drawing.Size(34, 16);
		label4.Size = size;
		this.Label3.TabIndex = 49;
		this.Label3.Text = "ราคา";
		this.DataGridView1.AllowUserToAddRows = false;
		this.DataGridView1.AllowUserToDeleteRows = false;
		this.DataGridView1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.DataGridView1.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize;
		this.DataGridView1.Columns.AddRange(this.Column1, this.DataGridViewTextBoxColumn_0, this.DataGridViewTextBoxColumn_1, this.DataGridViewTextBoxColumn_2);
		System.Windows.Forms.DataGridView dataGridView = this.DataGridView1;
		location = new System.Drawing.Point(14, 387);
		dataGridView.Location = location;
		this.DataGridView1.Name = "DataGridView1";
		System.Windows.Forms.DataGridView dataGridView2 = this.DataGridView1;
		size = new System.Drawing.Size(586, 163);
		dataGridView2.Size = size;
		this.DataGridView1.TabIndex = 48;
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(20, 309);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(80, 16);
		label6.Size = size;
		this.Label1.TabIndex = 47;
		this.Label1.Text = "รห\u0e31สประเภท :";
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label2;
		location = new System.Drawing.Point(204, 309);
		label7.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label8 = this.Label2;
		size = new System.Drawing.Size(70, 16);
		label8.Size = size;
		this.Label2.TabIndex = 47;
		this.Label2.Text = "ช\u0e37\u0e48อประเภท:";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[4] { this.ColumnHeader3, this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader4 });
		this.ListView1.ContextMenuStrip = this.ContextMenuStrip1;
		this.ListView1.FullRowSelect = true;
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(14, 22);
		listView.Location = location;
		this.ListView1.MultiSelect = false;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(586, 257);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader3.Text = "id";
		this.ColumnHeader3.Width = 0;
		this.ColumnHeader1.Text = "รห\u0e31สประเภท";
		this.ColumnHeader1.Width = 90;
		this.ColumnHeader2.Text = "ช\u0e37\u0e48อประเภทห\u0e49องพ\u0e31ก";
		this.ColumnHeader2.Width = 250;
		this.ColumnHeader4.Text = "อาหารเช\u0e49า";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader4.Width = 100;
		this.ContextMenuStrip1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ContextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[2] { this.ToolStripMenuItem_0, this.ToolStripMenuItem_1 });
		this.ContextMenuStrip1.Name = "ContextMenuStrip1";
		System.Windows.Forms.ContextMenuStrip contextMenuStrip = this.ContextMenuStrip1;
		size = new System.Drawing.Size(102, 48);
		contextMenuStrip.Size = size;
		this.ToolStripMenuItem_0.Name = "ลบToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem = this.ToolStripMenuItem_0;
		size = new System.Drawing.Size(101, 22);
		toolStripMenuItem.Size = size;
		this.ToolStripMenuItem_0.Text = "ลบ";
		this.ToolStripMenuItem_1.Name = "แกไขToolStripMenuItem";
		System.Windows.Forms.ToolStripMenuItem toolStripMenuItem2 = this.ToolStripMenuItem_1;
		size = new System.Drawing.Size(101, 22);
		toolStripMenuItem2.Size = size;
		this.ToolStripMenuItem_1.Text = "แก\u0e49ไข";
		this.TextBox2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox textBox = this.TextBox2;
		location = new System.Drawing.Point(102, 306);
		textBox.Location = location;
		this.TextBox2.Name = "TextBox2";
		System.Windows.Forms.TextBox textBox2 = this.TextBox2;
		size = new System.Drawing.Size(84, 23);
		textBox2.Size = size;
		this.TextBox2.TabIndex = 1;
		this.TextBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.TextBox textBox3 = this.TextBox1;
		location = new System.Drawing.Point(276, 306);
		textBox3.Location = location;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox4 = this.TextBox1;
		size = new System.Drawing.Size(324, 23);
		textBox4.Size = size;
		this.TextBox1.TabIndex = 2;
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		System.Windows.Forms.Button button3 = this.Button1;
		location = new System.Drawing.Point(452, 334);
		button3.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button4 = this.Button1;
		size = new System.Drawing.Size(71, 24);
		button4.Size = size;
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
		size = new System.Drawing.Size(633, 32);
		panelEx2.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.Blue;
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.Navy;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 31;
		this.PanelEx2.Text = "จ\u0e31ดการประเภทห\u0e49องพ\u0e31ก";
		this.Column1.Frozen = true;
		this.Column1.HeaderText = "ประเภทล\u0e39กค\u0e49า";
		this.Column1.Name = "Column1";
		this.Column1.ReadOnly = true;
		this.Column1.Width = 220;
		dataGridViewCellStyle.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleRight;
		this.DataGridViewTextBoxColumn_0.DefaultCellStyle = dataGridViewCellStyle;
		this.DataGridViewTextBoxColumn_0.HeaderText = "ราคารายว\u0e31น";
		this.DataGridViewTextBoxColumn_0.Name = "ราคา";
		dataGridViewCellStyle2.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleRight;
		this.DataGridViewTextBoxColumn_1.DefaultCellStyle = dataGridViewCellStyle2;
		this.DataGridViewTextBoxColumn_1.HeaderText = "ราคาช\u0e31\u0e48วคราว";
		this.DataGridViewTextBoxColumn_1.Name = "ราคาช\u0e31\u0e48วคราว";
		dataGridViewCellStyle3.Alignment = System.Windows.Forms.DataGridViewContentAlignment.MiddleRight;
		this.DataGridViewTextBoxColumn_2.DefaultCellStyle = dataGridViewCellStyle3;
		this.DataGridViewTextBoxColumn_2.HeaderText = "ราคารายเด\u0e37อน";
		this.DataGridViewTextBoxColumn_2.Name = "ราคารายเด\u0e37อน";
		this.DataGridViewTextBoxColumn_2.Width = 120;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(633, 608);
		this.ClientSize = size;
		this.Controls.Add(this.GroupBox1);
		this.Controls.Add(this.PanelEx2);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Name = "FrmSETRoomType";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "จ\u0e31ดการประเภทห\u0e49องพ\u0e31ก";
		this.GroupBox1.ResumeLayout(false);
		this.GroupBox1.PerformLayout();
		((System.ComponentModel.ISupportInitialize)this.DataGridView1).EndInit();
		this.ContextMenuStrip1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void add_seeds_Load(object sender, EventArgs e)
	{
		load_type();
	}

	public void load_type()
	{
		Button1.Text = Conversions.ToString(Badd);
		TextBox1.Text = "";
		TextBox2.Text = "";
		ComboBox1.Text = "";
		TextBox2.Focus();
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect("Select * From HT_SET_RoomType order by id");
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
					listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num2]["Room_PriceA"].ToString().Replace("0", "ไม\u0e48ม\u0e35").Replace("1", "ม\u0e35"));
					listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num2]["Room_PriceB"].ToString());
					listView.Items[count].SubItems.Add(dataSet.Tables[0].Rows[num2]["Room_PriceC"].ToString());
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
			MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อประเภทห\u0e49องพ\u0e31ก");
			return;
		}
		if (Operators.CompareString(TextBox2.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณากรอกรห\u0e31สประเภทห\u0e49องพ\u0e31ก");
			return;
		}
		if (Operators.CompareString(ComboBox1.Text, "", TextCompare: false) == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกอาหารเช\u0e49า");
			return;
		}
		checked
		{
			if (Operators.ConditionalCompareObjectEqual(Button1.Text, Badd, TextCompare: false))
			{
				if (Module1.connect("SELECT * From HT_SET_RoomType where name like '" + TextBox1.Text + "'").Tables[0].Rows.Count != 0)
				{
					MessageBox.Show("ม\u0e35ช\u0e37\u0e48อประเภทห\u0e49องพ\u0e31กน\u0e35\u0e49อย\u0e39\u0e48แล\u0e49ว");
				}
				else
				{
					object obj = "";
					obj = "INSERT INTO [HT_SET_RoomType]";
					obj = Operators.ConcatenateObject(obj, "([Name],[id_full],[room_priceA])");
					obj = Operators.ConcatenateObject(obj, "VALUES");
					obj = Operators.ConcatenateObject(obj, "(");
					obj = Operators.ConcatenateObject(obj, string.Concat("'" + TextBox1.Text, "'"));
					obj = Operators.ConcatenateObject(obj, string.Concat(",'" + TextBox2.Text, "'"));
					obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(ComboBox1.SelectedIndex));
					obj = Operators.ConcatenateObject(obj, ")");
					Module1.connect(Conversions.ToString(obj));
					int num = DataGridView1.Rows.Count - 1;
					int num2 = 0;
					while (true)
					{
						int num3 = num2;
						int num4 = num;
						if (num3 > num4)
						{
							break;
						}
						obj = "INSERT INTO [HT_Rooms_Price]";
						obj = Operators.ConcatenateObject(obj, "([Room_Type],[Room_CustType],[Room_Price],[Room_Price_H],[Room_Price_M])");
						obj = Operators.ConcatenateObject(obj, "VALUES");
						obj = Operators.ConcatenateObject(obj, "(");
						obj = Operators.ConcatenateObject(obj, string.Concat("'" + TextBox1.Text, "'"));
						obj = Operators.ConcatenateObject(obj, string.Concat(",'" + Conversions.ToString(DataGridView1.Rows[num2].Cells[0].Value), "'"));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num2].Cells[1].Value)));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num2].Cells[2].Value)));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num2].Cells[3].Value)));
						obj = Operators.ConcatenateObject(obj, "," + Conversions.ToString(new decimal(ComboBox1.SelectedIndex)));
						obj = Operators.ConcatenateObject(obj, ")");
						DataGridView1.Rows[num2].Cells[1].Value = 0;
						DataGridView1.Rows[num2].Cells[2].Value = 0;
						DataGridView1.Rows[num2].Cells[3].Value = 0;
						Module1.connect(Conversions.ToString(obj));
						num2++;
					}
					MessageBox.Show("เพ\u0e34\u0e48มช\u0e37\u0e48อประเภทห\u0e49องพ\u0e31กเร\u0e35ยบร\u0e49อย");
				}
			}
			else
			{
				Module1.connect(string.Format("Update HT_SET_RoomType SET room_priceA={2}, name = '{0}' WHERE id={1}", TextBox1.Text, ListView1.SelectedItems[0].SubItems[0].Text, ComboBox1.SelectedIndex));
				Module1.connect($"Update HT_Rooms SET Room_type = '{TextBox1.Text}' WHERE Room_type='{RuntimeHelpers.GetObjectValue(Editid)}'");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Rooms_Price where Room_Type='", Editid), "'")));
				int num5 = DataGridView1.Rows.Count - 1;
				int num6 = 0;
				while (true)
				{
					int num7 = num6;
					int num4 = num5;
					if (num7 > num4)
					{
						break;
					}
					object left = "INSERT INTO [HT_Rooms_Price]";
					left = Operators.ConcatenateObject(left, "([Room_Type],[Room_CustType],[Room_Price],[Room_Price_H],[Room_Price_M])");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, string.Concat("'" + TextBox1.Text, "'"));
					left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DataGridView1.Rows[num6].Cells[0].Value), "'"));
					left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num6].Cells[1].Value)));
					left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num6].Cells[2].Value)));
					left = Operators.ConcatenateObject(left, "," + Conversions.ToString(Conversions.ToDecimal(DataGridView1.Rows[num6].Cells[3].Value)));
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
					DataGridView1.Rows[num6].Cells[1].Value = 0;
					DataGridView1.Rows[num6].Cells[2].Value = 0;
					DataGridView1.Rows[num6].Cells[3].Value = 0;
					num6++;
				}
				MessageBox.Show("อ\u0e31บเดทเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			}
			load_type();
		}
	}

	private void ToolStripMenuItem_0_Click(object sender, EventArgs e)
	{
		DEL_item();
	}

	public void DEL_item()
	{
		if (ListView1.SelectedItems.Count != 0)
		{
			if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบ " + ListView1.SelectedItems[0].SubItems[2].Text + " หร\u0e37อไม\u0e48", "ลบรายการ", MessageBoxButtons.YesNo, MessageBoxIcon.Exclamation) == DialogResult.Yes)
			{
				DataSet dataSet = Module1.connect("select top 1 * from HT_Rooms where Room_type='" + ListView1.SelectedItems[0].SubItems[2].Text + "'");
				if (dataSet.Tables[0].Rows.Count != 0)
				{
					MessageBox.Show("รายกาน\u0e35\u0e49ได\u0e49ถ\u0e39กใช\u0e49งานอย\u0e39\u0e48ในรายการห\u0e49องพ\u0e31ก ไม\u0e48สามารถลบได\u0e49", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
					return;
				}
				Module1.connect("delete from HT_SET_RoomType where id=" + ListView1.SelectedItems[0].SubItems[0].Text);
				Module1.connect("delete from HT_Rooms_Price where Room_Type='" + ListView1.SelectedItems[0].SubItems[2].Text + "'");
				MessageBox.Show("ลบรายการเสร\u0e47จเร\u0e35ยบร\u0e49อย", "ลบรายการ", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
				load_type();
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
			Button1.Text = Conversions.ToString(Bupdate);
			TextBox2.Text = ListView1.SelectedItems[0].SubItems[1].Text;
			TextBox1.Text = ListView1.SelectedItems[0].SubItems[2].Text;
			Editid = ListView1.SelectedItems[0].SubItems[2].Text;
		}
		else
		{
			MessageBox.Show("ไม\u0e48ม\u0e35รายการท\u0e35\u0e48เล\u0e37อก", "แก\u0e49ไขรายการ", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			return;
		}
		DataGridView1.Rows.Clear();
		DataSet dataSet = Module1.connect("select * from HT_SET_CusType order by id");
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
				DataGridView1.Rows.Add();
				DataGridView1.Rows[num2].Cells[0].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["name"]);
				DataGridView1.Rows[num2].Cells[1].Value = 0;
				DataGridView1.Rows[num2].Cells[2].Value = 0;
				DataGridView1.Rows[num2].Cells[3].Value = 0;
				num2++;
			}
			dataSet = Module1.connect("select * from HT_Rooms_Price where Room_type='" + ListView1.SelectedItems[0].SubItems[2].Text + "' order by id");
			int num5 = DataGridView1.Rows.Count - 1;
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
					if (Operators.ConditionalCompareObjectEqual(Conversions.ToString(DataGridView1.Rows[num6].Cells[0].Value), dataSet.Tables[0].Rows[num9]["Room_CustType"], TextCompare: false))
					{
						DataGridView1.Rows[num6].Cells[1].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["Room_Price"]);
						DataGridView1.Rows[num6].Cells[2].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["Room_Price_H"]);
						DataGridView1.Rows[num6].Cells[3].Value = RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num9]["Room_Price_M"]);
					}
					num9++;
				}
				num6++;
			}
			Button1.Text = Conversions.ToString(Bupdate);
			TextBox2.Text = ListView1.SelectedItems[0].SubItems[1].Text;
			TextBox1.Text = ListView1.SelectedItems[0].SubItems[2].Text;
			try
			{
				ComboBox1.Text = ListView1.SelectedItems[0].SubItems[3].Text;
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
			Editid = ListView1.SelectedItems[0].SubItems[2].Text;
		}
	}

	private void Button2_Click_1(object sender, EventArgs e)
	{
		load_type();
	}
}
