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
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmPermission : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Bdel")]
	private ButtonX _Bdel;

	[AccessedThroughProperty("Bsave")]
	private ButtonX _Bsave;

	[AccessedThroughProperty("Bcan")]
	private ButtonX _Bcan;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("box")]
	private CheckedListBox _box;

	[AccessedThroughProperty("ButtonX9")]
	private ButtonX _ButtonX9;

	[AccessedThroughProperty("TextBox1")]
	private TextBox _TextBox1;

	[AccessedThroughProperty("ListBox1")]
	private ListBox _ListBox1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	private int i;

	internal virtual ButtonX Bdel
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bdel;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bdel_Click;
			if (_Bdel != null)
			{
				_Bdel.Click -= value2;
			}
			_Bdel = value;
			if (_Bdel != null)
			{
				_Bdel.Click += value2;
			}
		}
	}

	internal virtual ButtonX Bsave
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bsave;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bsave_Click;
			if (_Bsave != null)
			{
				_Bsave.Click -= value2;
			}
			_Bsave = value;
			if (_Bsave != null)
			{
				_Bsave.Click += value2;
			}
		}
	}

	internal virtual ButtonX Bcan
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bcan;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bcan_Click;
			if (_Bcan != null)
			{
				_Bcan.Click -= value2;
			}
			_Bcan = value;
			if (_Bcan != null)
			{
				_Bcan.Click += value2;
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

	internal virtual CheckedListBox box
	{
		[DebuggerNonUserCode]
		get
		{
			return _box;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_box = value;
		}
	}

	internal virtual ButtonX ButtonX9
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX9_Click;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click -= value2;
			}
			_ButtonX9 = value;
			if (_ButtonX9 != null)
			{
				_ButtonX9.Click += value2;
			}
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

	internal virtual ListBox ListBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ListBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ListBox1_SelectedIndexChanged;
			if (_ListBox1 != null)
			{
				_ListBox1.SelectedIndexChanged -= value2;
			}
			_ListBox1 = value;
			if (_ListBox1 != null)
			{
				_ListBox1.SelectedIndexChanged += value2;
			}
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

	[DebuggerNonUserCode]
	static FrmPermission()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmPermission()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += Selectlevel_FormClosing;
		base.Load += Selectlevel_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
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
		this.Bdel = new DevComponents.DotNetBar.ButtonX();
		this.Bsave = new DevComponents.DotNetBar.ButtonX();
		this.Bcan = new DevComponents.DotNetBar.ButtonX();
		this.Label1 = new System.Windows.Forms.Label();
		this.box = new System.Windows.Forms.CheckedListBox();
		this.ButtonX9 = new DevComponents.DotNetBar.ButtonX();
		this.TextBox1 = new System.Windows.Forms.TextBox();
		this.ListBox1 = new System.Windows.Forms.ListBox();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.SuspendLayout();
		this.Bdel.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bdel.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Bdel.Enabled = false;
		DevComponents.DotNetBar.ButtonX bdel = this.Bdel;
		System.Drawing.Point location = new System.Drawing.Point(422, 380);
		bdel.Location = location;
		DevComponents.DotNetBar.ButtonX bdel2 = this.Bdel;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bdel2.Margin = margin;
		this.Bdel.Name = "Bdel";
		DevComponents.DotNetBar.ButtonX bdel3 = this.Bdel;
		System.Drawing.Size size = new System.Drawing.Size(87, 28);
		bdel3.Size = size;
		this.Bdel.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2003;
		this.Bdel.TabIndex = 31;
		this.Bdel.Text = "ลบ";
		this.Bsave.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bsave.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Bsave.Enabled = false;
		DevComponents.DotNetBar.ButtonX bsave = this.Bsave;
		location = new System.Drawing.Point(331, 380);
		bsave.Location = location;
		DevComponents.DotNetBar.ButtonX bsave2 = this.Bsave;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bsave2.Margin = margin;
		this.Bsave.Name = "Bsave";
		DevComponents.DotNetBar.ButtonX bsave3 = this.Bsave;
		size = new System.Drawing.Size(87, 28);
		bsave3.Size = size;
		this.Bsave.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2003;
		this.Bsave.TabIndex = 32;
		this.Bsave.Text = "บ\u0e31นท\u0e36ก";
		this.Bcan.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bcan.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		DevComponents.DotNetBar.ButtonX bcan = this.Bcan;
		location = new System.Drawing.Point(513, 380);
		bcan.Location = location;
		DevComponents.DotNetBar.ButtonX bcan2 = this.Bcan;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		bcan2.Margin = margin;
		this.Bcan.Name = "Bcan";
		DevComponents.DotNetBar.ButtonX bcan3 = this.Bcan;
		size = new System.Drawing.Size(87, 28);
		bcan3.Size = size;
		this.Bcan.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2003;
		this.Bcan.TabIndex = 30;
		this.Bcan.Text = "ป\u0e34ด";
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(5, 388);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(66, 16);
		label2.Size = size;
		this.Label1.TabIndex = 29;
		this.Label1.Text = "ช\u0e37\u0e48อ Level :";
		this.box.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.box.CheckOnClick = true;
		this.box.ColumnWidth = 180;
		this.box.Enabled = false;
		this.box.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.box.FormattingEnabled = true;
		System.Windows.Forms.CheckedListBox checkedListBox = this.box;
		location = new System.Drawing.Point(170, 47);
		checkedListBox.Location = location;
		System.Windows.Forms.CheckedListBox checkedListBox2 = this.box;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		checkedListBox2.Margin = margin;
		this.box.MultiColumn = true;
		this.box.Name = "box";
		System.Windows.Forms.CheckedListBox checkedListBox3 = this.box;
		size = new System.Drawing.Size(629, 328);
		checkedListBox3.Size = size;
		this.box.TabIndex = 25;
		this.box.ThreeDCheckBoxes = true;
		this.ButtonX9.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX9.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX9;
		location = new System.Drawing.Point(253, 381);
		buttonX.Location = location;
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX9;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		buttonX2.Margin = margin;
		this.ButtonX9.Name = "ButtonX9";
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX9;
		size = new System.Drawing.Size(61, 26);
		buttonX3.Size = size;
		this.ButtonX9.Style = DevComponents.DotNetBar.eDotNetBarStyle.Office2003;
		this.ButtonX9.TabIndex = 28;
		this.ButtonX9.Text = "เพ\u0e34\u0e48ม";
		this.TextBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.TextBox1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		System.Windows.Forms.TextBox textBox = this.TextBox1;
		location = new System.Drawing.Point(72, 383);
		textBox.Location = location;
		System.Windows.Forms.TextBox textBox2 = this.TextBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		textBox2.Margin = margin;
		this.TextBox1.Name = "TextBox1";
		System.Windows.Forms.TextBox textBox3 = this.TextBox1;
		size = new System.Drawing.Size(179, 23);
		textBox3.Size = size;
		this.TextBox1.TabIndex = 27;
		this.ListBox1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ListBox1.FormattingEnabled = true;
		this.ListBox1.HorizontalScrollbar = true;
		this.ListBox1.ItemHeight = 16;
		System.Windows.Forms.ListBox listBox = this.ListBox1;
		location = new System.Drawing.Point(6, 48);
		listBox.Location = location;
		System.Windows.Forms.ListBox listBox2 = this.ListBox1;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		listBox2.Margin = margin;
		this.ListBox1.Name = "ListBox1";
		System.Windows.Forms.ListBox listBox3 = this.ListBox1;
		size = new System.Drawing.Size(158, 324);
		listBox3.Size = size;
		this.ListBox1.TabIndex = 26;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.Dock = System.Windows.Forms.DockStyle.Top;
		this.PanelEx2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx2;
		location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx2;
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		panelEx2.Margin = margin;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		size = new System.Drawing.Size(806, 39);
		panelEx3.Size = size;
		this.PanelEx2.Style.BackColor1.Color = System.Drawing.Color.FromArgb(95, 136, 215);
		this.PanelEx2.Style.BackColor2.Color = System.Drawing.Color.FromArgb(67, 108, 191);
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.Style.MarginLeft = 8;
		this.PanelEx2.TabIndex = 32;
		this.PanelEx2.Text = "จ\u0e31ดการการใช\u0e49งาน";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(806, 414);
		this.ClientSize = size;
		this.Controls.Add(this.Bdel);
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.Bsave);
		this.Controls.Add(this.Bcan);
		this.Controls.Add(this.ListBox1);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.TextBox1);
		this.Controls.Add(this.box);
		this.Controls.Add(this.ButtonX9);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FrmPermission";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "จ\u0e31ดการการใช\u0e49งาน";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		checked
		{
			int num = ListBox1.Items.Count - 1;
			i = 0;
			while (true)
			{
				int num2 = i;
				int num3 = num;
				if (num2 <= num3)
				{
					if (Operators.CompareString(ListBox1.Items[i].ToString().ToLower(), TextBox1.Text.ToLower(), TextCompare: false) == 0)
					{
						break;
					}
					i++;
					continue;
				}
				if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0)
				{
					ListBox1.Items.Add(TextBox1.Text);
					ListBox1.SelectedItem = TextBox1.Text;
					TextBox1.Text = "";
					TextBox1.Enabled = false;
					ListBox1.Enabled = false;
					Bsave.Enabled = true;
					Bdel.Enabled = true;
					int num4 = box.Items.Count - 1;
					i = 0;
					while (true)
					{
						int num5 = i;
						num3 = num4;
						if (num5 > num3)
						{
							break;
						}
						box.SetItemChecked(i, value: false);
						i++;
					}
					box.Enabled = true;
					Bsave.Enabled = true;
				}
				else
				{
					MessageBox.Show("กร\u0e38ณากรอกช\u0e37\u0e48อ");
				}
				return;
			}
			MessageBox.Show("ม\u0e35ช\u0e37\u0e48อ " + TextBox1.Text + " อย\u0e39\u0e48แล\u0e49ว");
			TextBox1.Focus();
		}
	}

	private void Bsave_Click(object sender, EventArgs e)
	{
		checked
		{
			if (!ListBox1.Enabled)
			{
				int num = box.CheckedItems.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					object left = "";
					left = Operators.ConcatenateObject(left, "INSERT INTO [TB_MRP_PERMISSION]");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, "[level_name]");
					left = Operators.ConcatenateObject(left, ",[level_command]");
					left = Operators.ConcatenateObject(left, ")");
					left = Operators.ConcatenateObject(left, "VALUES");
					left = Operators.ConcatenateObject(left, "(");
					left = Operators.ConcatenateObject(left, string.Concat("'" + ListBox1.Text, "'"));
					left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", box.CheckedItems[num2]), "'"));
					left = Operators.ConcatenateObject(left, ")");
					Module1.connect(Conversions.ToString(left));
					num2++;
				}
				Bsave.Enabled = false;
				TextBox1.Enabled = true;
				ListBox1.Enabled = true;
				MessageBox.Show("เพ\u0e34\u0e48มข\u0e49อม\u0e39ลเสร\u0e47จเร\u0e35ยบร\u0e49อย!!");
				return;
			}
			Module1.connect("delete from TB_MRP_PERMISSION where level_name='" + ListBox1.Text + "'");
			int num5 = box.CheckedItems.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 > num4)
				{
					break;
				}
				string text = "";
				text += "INSERT INTO [TB_MRP_PERMISSION]";
				text += "(";
				text += "[level_name]";
				text += ",[level_command]";
				text += ")";
				text += "VALUES";
				text += "(";
				text = text + "'" + ListBox1.Text + "'";
				text = Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", box.CheckedItems[num6]), "'")));
				text += ")";
				Module1.connect(text);
				num6++;
			}
			MessageBox.Show("อ\u0e31บเดทรายการเสร\u0e47จเร\u0e35ยบร\u0e49อย!");
		}
	}

	private void Bdel_Click(object sender, EventArgs e)
	{
		try
		{
			if (Operators.ConditionalCompareObjectEqual(ListBox1.SelectedItem, "Admin", TextCompare: false))
			{
				MessageBox.Show("ไม\u0e48สามารถลบ Level Admin ได\u0e49");
			}
			else if (Operators.CompareString(TextBox1.Text, "", TextCompare: false) != 0)
			{
				ListBox1.Enabled = true;
				TextBox1.Enabled = true;
				Bsave.Enabled = true;
				Bdel.Enabled = true;
				box.Enabled = false;
				listLevel();
			}
			else if (ListBox1.Enabled)
			{
				DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT * From View_MRP_EMPLOYEE where emp_level='", ListBox1.SelectedItem), "'")));
				if (dataSet.Tables[0].Rows.Count != 0)
				{
					MessageBox.Show("ม\u0e35 User อ\u0e37\u0e48นใช\u0e49งานการเข\u0e49าถ\u0e36งระด\u0e31บน\u0e35\u0e49อย\u0e39\u0e48 ", "ไม\u0e48สามารถลบได\u0e49", MessageBoxButtons.OK, MessageBoxIcon.Hand);
				}
				else if (MessageBox.Show("ค\u0e38ณต\u0e49องการลบหร\u0e37อไม\u0e48", "ลบ", MessageBoxButtons.YesNo, MessageBoxIcon.Question) == DialogResult.Yes)
				{
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("DELETE From TB_MRP_PERMISSION where level_name='", ListBox1.SelectedItem), "'")));
					MessageBox.Show("ลบเสร\u0e47จเร\u0e35ยบร\u0e49อย");
					listLevel();
				}
			}
			else
			{
				ListBox1.Enabled = true;
				TextBox1.Enabled = true;
				Bsave.Enabled = true;
				Bdel.Enabled = true;
				box.Enabled = false;
				listLevel();
			}
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void Bcan_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void Selectlevel_FormClosing(object sender, FormClosingEventArgs e)
	{
	}

	public void listLevel()
	{
		checked
		{
			try
			{
				ListBox1.Items.Clear();
				DataSet dataSet = Module1.connect("SELECT Level_name From TB_MRP_Permission Group by Level_name");
				int num = dataSet.Tables[0].Rows.Count - 1;
				i = 0;
				while (true)
				{
					int num2 = i;
					int num3 = num;
					if (num2 <= num3)
					{
						ListBox1.Items.Add(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[i]["Level_name"]));
						i++;
						continue;
					}
					break;
				}
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void Selectlevel_Load(object sender, EventArgs e)
	{
		MyProject.Application.ChangeCulture("en-US");
		listLevel();
		ListPermission();
	}

	public void ListPermission()
	{
		box.Items.Clear();
		box.Items.Add("สถานะห\u0e49องพ\u0e31ก");
		box.Items.Add("Check-In");
		box.Items.Add("Check-Out");
		box.Items.Add("ยกเล\u0e34กห\u0e49องพ\u0e31ก");
		box.Items.Add("รายการจอง");
		box.Items.Add("ขายส\u0e34นค\u0e49า");
		box.Items.Add("รายการ Check-In/Check-Out");
		box.Items.Add("ใบลงทะเบ\u0e35ยนผ\u0e39\u0e49เข\u0e49าพ\u0e31ก");
		box.Items.Add("ใบม\u0e31ดจำ");
		box.Items.Add("ใบเสร\u0e47จร\u0e31บเง\u0e34น");
		box.Items.Add("ใบกำก\u0e31บภาษ\u0e35");
		box.Items.Add("ชำระเง\u0e34น/ล\u0e39กหน\u0e35\u0e49");
		box.Items.Add("จ\u0e31ดการรอบบ\u0e34ล");
		box.Items.Add("ค\u0e39ปอง");
		box.Items.Add("จ\u0e31ดการผ\u0e39\u0e49ใช\u0e49งาน");
		box.Items.Add("จ\u0e31ดการประเภทห\u0e49องพ\u0e31ก");
		box.Items.Add("จ\u0e31ดการห\u0e49องพ\u0e31ก");
		box.Items.Add("จ\u0e31ดการล\u0e39กค\u0e49า");
		box.Items.Add("จ\u0e31ดการประเภทล\u0e39กค\u0e49า");
		box.Items.Add("ต\u0e31\u0e49งค\u0e48ากล\u0e38\u0e48มราคา");
		box.Items.Add("ต\u0e31\u0e49งค\u0e48าปร\u0e31บราคาลง");
		box.Items.Add("ต\u0e31\u0e49งค\u0e48าปร\u0e31บราคาข\u0e36\u0e49น");
		box.Items.Add("จ\u0e31ดการประเภทส\u0e34นค\u0e49า");
		box.Items.Add("จ\u0e31ดการส\u0e34นค\u0e49า");
		box.Items.Add("ต\u0e31\u0e49งค\u0e48าโปรแกรม");
		box.Items.Add("อ\u0e31บเดทโปรแกรม");
		box.Items.Add("รายงานสร\u0e38ปประจำว\u0e31น");
		box.Items.Add("รายงานแขกเข\u0e49าพ\u0e31ก");
		box.Items.Add("รายงานแขกออก");
		box.Items.Add("รายงานแขกท\u0e35\u0e48อย\u0e39\u0e48ในโรงแรม");
		box.Items.Add("รายงานการย\u0e49ายห\u0e49อง");
		box.Items.Add("รายงานภาษ\u0e35ขาย");
		box.Items.Add("รายงานล\u0e39กหน\u0e35\u0e49");
		box.Items.Add("รายงานส\u0e34นค\u0e49า");
		box.Items.Add("รายงานการขายส\u0e34นค\u0e49า");
		box.Items.Add("รายงานการทำความสะอาด");
		box.Items.Add("รายงานการจองห\u0e49องพ\u0e31ก");
		box.Items.Add("รายงานการยกเล\u0e34กห\u0e49องพ\u0e31ก");
		box.Items.Add("รายงานค\u0e39ปอง");
		box.Items.Add("รายงานเง\u0e34นม\u0e31ดจำ");
		box.Items.Add("รายงานสร\u0e38ปภาพรวม");
		box.Items.Add("รายงานซ\u0e48อม");
		box.Items.Add("รายงานระหว\u0e48างว\u0e31นท\u0e35\u0e48");
		box.Items.Add("รายงานตามรอบบ\u0e34ล");
		box.Items.Add("รายงานการขายห\u0e49อง");
		box.Items.Add("รายงานเง\u0e34นสดคงเหล\u0e37อ");
	}

	private void ListBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		box.Enabled = true;
		Bsave.Enabled = true;
		Bdel.Enabled = true;
		checked
		{
			int num = box.Items.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				box.SetItemChecked(num2, value: false);
				num2++;
			}
			DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("SELECT * From TB_MRP_Permission where Level_Name ='", ListBox1.SelectedItem), "'")));
			if (dataSet.Tables[0].Rows.Count != 0)
			{
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
					int num8 = box.Items.Count - 1;
					int num9 = 0;
					while (true)
					{
						int num10 = num9;
						num4 = num8;
						if (num10 <= num4)
						{
							if (!Operators.ConditionalCompareObjectEqual(box.Items[num9], dataSet.Tables[0].Rows[num6]["Level_Command"], TextCompare: false))
							{
								num9++;
								continue;
							}
							box.SetItemChecked(num9, value: true);
							break;
						}
						break;
					}
					num6++;
				}
			}
			if (Operators.ConditionalCompareObjectEqual(ListBox1.SelectedItem, "Admin", TextCompare: false))
			{
				int num11 = box.Items.Count - 1;
				int num12 = 0;
				while (true)
				{
					int num13 = num12;
					int num4 = num11;
					if (num13 > num4)
					{
						break;
					}
					box.SetItemChecked(num12, value: true);
					num12++;
				}
			}
			if (Operators.ConditionalCompareObjectEqual(ListBox1.SelectedItem, "Admin", TextCompare: false))
			{
				box.Enabled = false;
				Bdel.Enabled = false;
				Bsave.Enabled = false;
			}
		}
	}
}
