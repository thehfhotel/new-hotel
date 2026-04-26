using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSelectDB_old : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	public bool ISOK;

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
			KeyEventHandler value2 = ListView1_KeyDown;
			EventHandler value3 = ListView1_DoubleClick;
			EventHandler value4 = ListView1_SelectedIndexChanged;
			if (_ListView1 != null)
			{
				_ListView1.KeyDown -= value2;
				_ListView1.DoubleClick -= value3;
				_ListView1.SelectedIndexChanged -= value4;
			}
			_ListView1 = value;
			if (_ListView1 != null)
			{
				_ListView1.KeyDown += value2;
				_ListView1.DoubleClick += value3;
				_ListView1.SelectedIndexChanged += value4;
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

	internal virtual ButtonX ButtonX2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX2_Click;
			if (_ButtonX2 != null)
			{
				_ButtonX2.Click -= value2;
			}
			_ButtonX2 = value;
			if (_ButtonX2 != null)
			{
				_ButtonX2.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX1_Click;
			if (_ButtonX1 != null)
			{
				_ButtonX1.Click -= value2;
			}
			_ButtonX1 = value;
			if (_ButtonX1 != null)
			{
				_ButtonX1.Click += value2;
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

	internal virtual Timer Timer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Timer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Timer1_Tick;
			if (_Timer1 != null)
			{
				_Timer1.Tick -= value2;
			}
			_Timer1 = value;
			if (_Timer1 != null)
			{
				_Timer1.Tick += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX3_Click;
			if (_ButtonX3 != null)
			{
				_ButtonX3.Click -= value2;
			}
			_ButtonX3 = value;
			if (_ButtonX3 != null)
			{
				_ButtonX3.Click += value2;
			}
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
	static FormSelectDB_old()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormSelectDB_old()
	{
		Class2.LH6iGfYz9j3MJ();
		base._002Ector();
		base.Load += FormSelectDB_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISOK = false;
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
		this.Label1 = new System.Windows.Forms.Label();
		this.Button1 = new System.Windows.Forms.Button();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Button2 = new System.Windows.Forms.Button();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(11, 19);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(96, 17);
		label2.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "เล\u0e37อกฐานข\u0e49อม\u0e39ล";
		this.Button1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(495, 262);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(75, 30);
		button2.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.Text = "ตกลง";
		this.Button1.UseVisualStyleBackColor = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[3] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3 });
		this.ListView1.FullRowSelect = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(14, 39);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(556, 217);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "Name";
		this.ColumnHeader1.Width = 250;
		this.ColumnHeader2.Text = "Database";
		this.ColumnHeader2.Width = 120;
		this.ColumnHeader3.Text = "Server IP";
		this.ColumnHeader3.Width = 150;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.Orange;
		this.ButtonX2.Cursor = System.Windows.Forms.Cursors.Hand;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(14, 262);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(86, 30);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 4;
		this.ButtonX2.Text = "ต\u0e31\u0e49งค\u0e48าฐานข\u0e49อม\u0e39ล";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.Orange;
		this.ButtonX1.Cursor = System.Windows.Forms.Cursors.Hand;
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		location = new System.Drawing.Point(121, 262);
		buttonX3.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		size = new System.Drawing.Size(86, 30);
		buttonX4.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 3;
		this.ButtonX1.Text = "ต\u0e31\u0e49งค\u0e48า Server";
		this.Button2.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right;
		System.Windows.Forms.Button button3 = this.Button2;
		location = new System.Drawing.Point(531, 6);
		button3.Location = location;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button4 = this.Button2;
		size = new System.Drawing.Size(39, 29);
		button4.Size = size;
		this.Button2.TabIndex = 5;
		this.Button2.Text = "ป\u0e34ด";
		this.Button2.UseVisualStyleBackColor = true;
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.Orange;
		this.ButtonX3.Cursor = System.Windows.Forms.Cursors.Hand;
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(213, 262);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(117, 30);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 6;
		this.ButtonX3.Text = "อ\u0e31บเดทโปรแกรม";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(583, 300);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.Button2);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 10f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormSelectDB";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "เล\u0e37อกฐานข\u0e49อม\u0e39ล";
		this.TopMost = true;
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	protected override bool ProcessCmdKey(ref Message msg, Keys keyData)
	{
		if (keyData == (Keys.F9 | Keys.Alt))
		{
			TopMost = false;
			MyProject.Forms.GENDB.ShowDialog();
			TopMost = true;
		}
		bool result = default(bool);
		return result;
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		if (ListView1.SelectedItems.Count == 0)
		{
			MessageBox.Show("กร\u0e38ณาเล\u0e37อกฐานข\u0e49อม\u0e39ล");
			return;
		}
		ISOK = true;
		MSSQL.MysqlDatabase = ListView1.SelectedItems[0].SubItems[1].Text;
		MSSQL.MysqlServer = ListView1.SelectedItems[0].SubItems[2].Text;
		MyProject.Forms.frmMain1.LabelStatus.Text = "ฐานข\u0e49อม\u0e39ลท\u0e35\u0e48ใช\u0e49: " + ListView1.SelectedItems[0].SubItems[0].Text;
		Close();
	}

	private void FormSelectDB_Load(object sender, EventArgs e)
	{
		ISOK = false;
		Timer1.Enabled = true;
	}

	private void ListView1_DoubleClick(object sender, EventArgs e)
	{
		MSSQL.MysqlDatabase = ListView1.SelectedItems[0].SubItems[1].Text;
		MSSQL.MysqlServer = ListView1.SelectedItems[0].SubItems[2].Text;
		ISOK = true;
		Close();
	}

	private void ListView1_KeyDown(object sender, KeyEventArgs e)
	{
		if (e.KeyData == Keys.Return && ListView1.SelectedItems.Count != 0)
		{
			MSSQL.MysqlDatabase = ListView1.SelectedItems[0].SubItems[1].Text;
			MSSQL.MysqlServer = ListView1.SelectedItems[0].SubItems[2].Text;
			ISOK = true;
			Close();
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Process process = new Process();
		process.StartInfo.WorkingDirectory = Module1.Path_Program;
		process.StartInfo.FileName = "Config.ini";
		process.Start();
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Process process = new Process();
		process.StartInfo.WorkingDirectory = Module1.Path_Program;
		process.StartInfo.FileName = "db.txt";
		process.Start();
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		if (ListView1.Items.Count != 0)
		{
			ListView1.Items[0].Selected = true;
			ListView1.Focus();
		}
		save_db();
	}

	public void save_db()
	{
		if (File.Exists(Module1.Path_Program + "server.txt"))
		{
			File.Delete(Module1.Path_Program + "server.txt");
		}
		StreamWriter streamWriter = new StreamWriter(Module1.Path_Program + "server.txt", append: true, Encoding.UTF8);
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
				streamWriter.WriteLine(ListView1.Items[num2].SubItems[2].Text + "|" + MSSQL.MysqlUsername + "|" + MSSQL.MysqlPassword + "|" + ListView1.Items[num2].SubItems[1].Text + "|MSSQL|" + ListView1.Items[num2].SubItems[0].Text);
				num2++;
			}
			streamWriter.Close();
			MessageBox.Show("ม\u0e35การอ\u0e31บเดทข\u0e49อม\u0e39ลกร\u0e38ณาเข\u0e49าโปรแกรมใหม\u0e48อ\u0e35กคร\u0e31\u0e49ง", "แจ\u0e49งเต\u0e37อน", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
			Close();
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		MyProject.Forms.FrmUpdate.ShowDialog();
	}
}
