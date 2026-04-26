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
public class FormShowDEPBack : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("LabelX1")]
	private LabelX _LabelX1;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("DEPPRICE")]
	private LabelX _DEPPRICE;

	[AccessedThroughProperty("LabelX3")]
	private LabelX _LabelX3;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

	[AccessedThroughProperty("ColumnHeader1")]
	private ColumnHeader _ColumnHeader1;

	[AccessedThroughProperty("ColumnHeader2")]
	private ColumnHeader _ColumnHeader2;

	[AccessedThroughProperty("ColumnHeader3")]
	private ColumnHeader _ColumnHeader3;

	[AccessedThroughProperty("CheckBox1")]
	private CheckBox _CheckBox1;

	public bool ISBACK;

	public string cin_id;

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

	internal virtual LabelX LabelX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX1 = value;
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

	internal virtual LabelX DEPPRICE
	{
		[DebuggerNonUserCode]
		get
		{
			return _DEPPRICE;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_DEPPRICE = value;
		}
	}

	internal virtual LabelX LabelX3
	{
		[DebuggerNonUserCode]
		get
		{
			return _LabelX3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_LabelX3 = value;
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
	static FormShowDEPBack()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormShowDEPBack()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormShowDEP_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		ISBACK = false;
		cin_id = "";
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
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.CheckBox1 = new System.Windows.Forms.CheckBox();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.LabelX1 = new DevComponents.DotNetBar.LabelX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.DEPPRICE = new DevComponents.DotNetBar.LabelX();
		this.LabelX3 = new DevComponents.DotNetBar.LabelX();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.CheckBox1);
		this.PanelEx1.Controls.Add(this.ListView1);
		this.PanelEx1.Controls.Add(this.LabelX1);
		this.PanelEx1.Controls.Add(this.ButtonX2);
		this.PanelEx1.Controls.Add(this.ButtonX1);
		this.PanelEx1.Controls.Add(this.DEPPRICE);
		this.PanelEx1.Controls.Add(this.LabelX3);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(576, 491);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.CheckBox1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.CheckBox1.AutoSize = true;
		System.Windows.Forms.CheckBox checkBox = this.CheckBox1;
		location = new System.Drawing.Point(16, 412);
		checkBox.Location = location;
		this.CheckBox1.Name = "CheckBox1";
		System.Windows.Forms.CheckBox checkBox2 = this.CheckBox1;
		size = new System.Drawing.Size(181, 20);
		checkBox2.Size = size;
		this.CheckBox1.TabIndex = 3;
		this.CheckBox1.Text = "เล\u0e37อกท\u0e31\u0e49งหมด/ไม\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
		this.CheckBox1.UseVisualStyleBackColor = true;
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.CheckBoxes = true;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[3] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3 });
		this.ListView1.Font = new System.Drawing.Font("Tahoma", 18f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.ListView1.GridLines = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(12, 12);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(552, 396);
		listView2.Size = size;
		this.ListView1.TabIndex = 2;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "";
		this.ColumnHeader1.Width = 20;
		this.ColumnHeader2.Text = "หมายเลขห\u0e49อง";
		this.ColumnHeader2.Width = 350;
		this.ColumnHeader3.Text = "จำนวนเง\u0e34น";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader3.Width = 150;
		this.LabelX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LabelX1.BackgroundStyle.Class = "";
		this.LabelX1.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX labelX = this.LabelX1;
		location = new System.Drawing.Point(12, 431);
		labelX.Location = location;
		this.LabelX1.Name = "LabelX1";
		DevComponents.DotNetBar.LabelX labelX2 = this.LabelX1;
		size = new System.Drawing.Size(166, 54);
		labelX2.Size = size;
		this.LabelX1.TabIndex = 0;
		this.LabelX1.Text = "ค\u0e37นค\u0e48าม\u0e31ดจำ";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX2;
		location = new System.Drawing.Point(469, 414);
		buttonX.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX2;
		size = new System.Drawing.Size(98, 71);
		buttonX2.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 1;
		this.ButtonX2.Text = "ไม\u0e48ค\u0e37น";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX1;
		location = new System.Drawing.Point(365, 414);
		buttonX3.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX1;
		size = new System.Drawing.Size(98, 71);
		buttonX4.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 0;
		this.ButtonX1.Text = "ค\u0e37น";
		this.DEPPRICE.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.DEPPRICE.BackgroundStyle.Class = "";
		this.DEPPRICE.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		this.DEPPRICE.ForeColor = System.Drawing.Color.Red;
		DevComponents.DotNetBar.LabelX dEPPRICE = this.DEPPRICE;
		location = new System.Drawing.Point(112, 431);
		dEPPRICE.Location = location;
		this.DEPPRICE.Name = "DEPPRICE";
		DevComponents.DotNetBar.LabelX dEPPRICE2 = this.DEPPRICE;
		size = new System.Drawing.Size(182, 54);
		dEPPRICE2.Size = size;
		this.DEPPRICE.TabIndex = 1;
		this.DEPPRICE.Text = "200";
		this.DEPPRICE.TextAlignment = System.Drawing.StringAlignment.Far;
		this.LabelX3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.LabelX3.BackgroundStyle.Class = "";
		this.LabelX3.Font = new System.Drawing.Font("Tahoma", 20.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
		DevComponents.DotNetBar.LabelX labelX3 = this.LabelX3;
		location = new System.Drawing.Point(306, 431);
		labelX3.Location = location;
		this.LabelX3.Name = "LabelX3";
		DevComponents.DotNetBar.LabelX labelX4 = this.LabelX3;
		size = new System.Drawing.Size(118, 54);
		labelX4.Size = size;
		this.LabelX3.TabIndex = 0;
		this.LabelX3.Text = "บาท";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(576, 491);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.PanelEx1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedDialog;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormShowDEPBack";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ค\u0e48าม\u0e31ดจำ";
		this.TopMost = true;
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FormShowDEP_Load(object sender, EventArgs e)
	{
		ISBACK = false;
		CheckBox1.Checked = false;
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect("select * from HT_CheckIn_Ds where CIN_NO='" + cin_id + "' and cin_dep_status='ย\u0e31งไม\u0e48ค\u0e37นค\u0e48าม\u0e31ดจำ' and cin_room_dep<>0 and cin_room_status='Check-Out'");
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
				ListView.ListViewItemCollection items = ListView1.Items;
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
				ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[num2].SubItems;
				array3 = new object[1];
				object[] array5 = array3;
				dataRow = dataSet.Tables[0].Rows[num2];
				DataRow dataRow3 = dataRow;
				columnName = "cin_room_no";
				array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
				array = array3;
				object[] arguments2 = array;
				array4 = new bool[1] { true };
				NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
				if (array4[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
				}
				ListView1.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["cin_room_dep"]), "#,##0.00"));
				num2++;
			}
			sum();
			CheckBox1.Checked = true;
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
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
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Dep_return_date=getdate(),Cin_Dep_Status='ค\u0e37นเง\u0e34นแล\u0e49ว',Cin_Dep_return_by='", Module1.loginName), "' where id="), ListView1.CheckedItems[num2].SubItems[0].Text)));
				num2++;
			}
			ISBACK = true;
			Close();
		}
	}

	private void ListView1_ItemChecked(object sender, ItemCheckedEventArgs e)
	{
		sum();
	}

	public void sum()
	{
		decimal num = default(decimal);
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
				num = decimal.Add(num, Conversions.ToDecimal(ListView1.CheckedItems[num3].SubItems[2].Text));
				num3++;
			}
			DEPPRICE.Text = Strings.Format(num, "#,##0.00");
		}
	}

	private void ListView1_SelectedIndexChanged(object sender, EventArgs e)
	{
	}

	private void CheckBox1_CheckedChanged(object sender, EventArgs e)
	{
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
				ListView1.Items[num2].Checked = CheckBox1.Checked;
				num2++;
			}
			sum();
		}
	}
}
