using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using GlacialComponents.Controls;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmSearchStock : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Tsearch")]
	private TextBoxX _Tsearch;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Tsearch2")]
	private TextBoxX _Tsearch2;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Listview1")]
	private GlacialList _Listview1;

	public string Search_id;

	internal virtual TextBoxX Tsearch
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tsearch;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			KeyEventHandler value2 = Tsearch_KeyUp;
			EventHandler value3 = Tsearch_TextChanged;
			if (_Tsearch != null)
			{
				_Tsearch.KeyUp -= value2;
				_Tsearch.TextChanged -= value3;
			}
			_Tsearch = value;
			if (_Tsearch != null)
			{
				_Tsearch.KeyUp += value2;
				_Tsearch.TextChanged += value3;
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

	internal virtual TextBoxX Tsearch2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Tsearch2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Tsearch_TextChanged;
			if (_Tsearch2 != null)
			{
				_Tsearch2.TextChanged -= value2;
			}
			_Tsearch2 = value;
			if (_Tsearch2 != null)
			{
				_Tsearch2.TextChanged += value2;
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

	internal virtual GlacialList Listview1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Listview1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Listview1_Click_1;
			KeyEventHandler value3 = Listview1_KeyUp;
			EventHandler value4 = Listview1_Click;
			if (_Listview1 != null)
			{
				_Listview1.Click -= value2;
				_Listview1.KeyUp -= value3;
				_Listview1.DoubleClick -= value4;
			}
			_Listview1 = value;
			if (_Listview1 != null)
			{
				_Listview1.Click += value2;
				_Listview1.KeyUp += value3;
				_Listview1.DoubleClick += value4;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmSearchStock()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmSearchStock()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += FrmSearchStock_FormClosing;
		base.Load += SearchBook_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		Search_id = "0";
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
		GlacialComponents.Controls.GLColumn gLColumn = new GlacialComponents.Controls.GLColumn();
		GlacialComponents.Controls.GLColumn gLColumn2 = new GlacialComponents.Controls.GLColumn();
		GlacialComponents.Controls.GLColumn gLColumn3 = new GlacialComponents.Controls.GLColumn();
		GlacialComponents.Controls.GLColumn gLColumn4 = new GlacialComponents.Controls.GLColumn();
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmSearchStock));
		this.Tsearch = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Tsearch2 = new DevComponents.DotNetBar.Controls.TextBoxX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label1 = new System.Windows.Forms.Label();
		this.Listview1 = new GlacialComponents.Controls.GlacialList();
		this.SuspendLayout();
		this.Tsearch.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tsearch = this.Tsearch;
		System.Drawing.Point location = new System.Drawing.Point(64, 8);
		tsearch.Location = location;
		this.Tsearch.Name = "Tsearch";
		DevComponents.DotNetBar.Controls.TextBoxX tsearch2 = this.Tsearch;
		System.Drawing.Size size = new System.Drawing.Size(171, 22);
		tsearch2.Size = size;
		this.Tsearch.TabIndex = 0;
		this.Label8.AutoSize = true;
		System.Windows.Forms.Label label = this.Label8;
		location = new System.Drawing.Point(245, 12);
		label.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label2 = this.Label8;
		size = new System.Drawing.Size(49, 14);
		label2.Size = size;
		this.Label8.TabIndex = 23;
		this.Label8.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		this.Label7.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label7;
		location = new System.Drawing.Point(7, 12);
		label3.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label4 = this.Label7;
		size = new System.Drawing.Size(54, 14);
		label4.Size = size;
		this.Label7.TabIndex = 24;
		this.Label7.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		this.Timer1.Interval = 10;
		this.Tsearch2.Border.Class = "TextBoxBorder";
		DevComponents.DotNetBar.Controls.TextBoxX tsearch3 = this.Tsearch2;
		location = new System.Drawing.Point(298, 8);
		tsearch3.Location = location;
		this.Tsearch2.Name = "Tsearch2";
		DevComponents.DotNetBar.Controls.TextBoxX tsearch4 = this.Tsearch2;
		size = new System.Drawing.Size(171, 22);
		tsearch4.Size = size;
		this.Tsearch2.TabIndex = 1;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(400, 376);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		size = new System.Drawing.Size(87, 25);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 30;
		this.ButtonX1.Text = "ป\u0e34ด";
		this.Label1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label5 = this.Label1;
		location = new System.Drawing.Point(9, 378);
		label5.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label6 = this.Label1;
		size = new System.Drawing.Size(110, 14);
		label6.Size = size;
		this.Label1.TabIndex = 31;
		this.Label1.Text = "พบท\u0e31\u0e49งหมด 0 รายการ";
		this.Listview1.AllowColumnResize = true;
		this.Listview1.AllowMultiselect = false;
		this.Listview1.AlternateBackground = System.Drawing.Color.LavenderBlush;
		this.Listview1.AlternatingColors = true;
		this.Listview1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.Listview1.AutoHeight = false;
		this.Listview1.BackColor = System.Drawing.Color.White;
		this.Listview1.BackgroundStretchToFit = true;
		gLColumn.ActivatedEmbeddedType = GlacialComponents.Controls.GLActivatedEmbeddedTypes.None;
		gLColumn.CheckBoxes = false;
		gLColumn.ImageIndex = -1;
		gLColumn.Name = "Column1";
		gLColumn.NumericSort = false;
		gLColumn.Text = "id";
		gLColumn.TextAlignment = System.Drawing.ContentAlignment.MiddleLeft;
		gLColumn.Width = 0;
		gLColumn2.ActivatedEmbeddedType = GlacialComponents.Controls.GLActivatedEmbeddedTypes.None;
		gLColumn2.CheckBoxes = false;
		gLColumn2.ImageIndex = -1;
		gLColumn2.Name = "Column2";
		gLColumn2.NumericSort = false;
		gLColumn2.Text = "รห\u0e31สส\u0e34นค\u0e49า";
		gLColumn2.TextAlignment = System.Drawing.ContentAlignment.MiddleLeft;
		gLColumn2.Width = 100;
		gLColumn3.ActivatedEmbeddedType = GlacialComponents.Controls.GLActivatedEmbeddedTypes.None;
		gLColumn3.CheckBoxes = false;
		gLColumn3.ImageIndex = -1;
		gLColumn3.Name = "Column3";
		gLColumn3.NumericSort = false;
		gLColumn3.Text = "ช\u0e37\u0e48อส\u0e34นค\u0e49า";
		gLColumn3.TextAlignment = System.Drawing.ContentAlignment.MiddleLeft;
		gLColumn3.Width = 220;
		gLColumn4.ActivatedEmbeddedType = GlacialComponents.Controls.GLActivatedEmbeddedTypes.None;
		gLColumn4.CheckBoxes = false;
		gLColumn4.ImageIndex = -1;
		gLColumn4.Name = "Column4";
		gLColumn4.NumericSort = false;
		gLColumn4.Text = "ชน\u0e34ดส\u0e34นค\u0e49า";
		gLColumn4.TextAlignment = System.Drawing.ContentAlignment.MiddleLeft;
		gLColumn4.Width = 110;
		this.Listview1.Columns.AddRange(new GlacialComponents.Controls.GLColumn[4] { gLColumn, gLColumn2, gLColumn3, gLColumn4 });
		this.Listview1.ControlStyle = GlacialComponents.Controls.GLControlStyles.XP;
		this.Listview1.FullRowSelect = true;
		this.Listview1.GridColor = System.Drawing.Color.LightGray;
		this.Listview1.GridLines = GlacialComponents.Controls.GLGridLines.gridBoth;
		this.Listview1.GridLineStyle = GlacialComponents.Controls.GLGridLineStyles.gridDashed;
		this.Listview1.GridTypes = GlacialComponents.Controls.GLGridTypes.gridNormal;
		this.Listview1.HeaderHeight = 22;
		this.Listview1.HeaderVisible = true;
		this.Listview1.HeaderWordWrap = true;
		this.Listview1.HotColumnTracking = true;
		this.Listview1.HotItemTracking = true;
		this.Listview1.HotTrackingColor = System.Drawing.Color.FromArgb(128, 255, 255);
		this.Listview1.HoverEvents = true;
		this.Listview1.HoverTime = 1;
		this.Listview1.ImageList = null;
		this.Listview1.ItemHeight = 19;
		this.Listview1.ItemWordWrap = false;
		GlacialComponents.Controls.GlacialList listview = this.Listview1;
		location = new System.Drawing.Point(7, 39);
		listview.Location = location;
		GlacialComponents.Controls.GlacialList listview2 = this.Listview1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 5, 3, 5);
		listview2.Margin = margin;
		this.Listview1.Name = "Listview1";
		this.Listview1.Selectable = true;
		this.Listview1.SelectedTextColor = System.Drawing.Color.White;
		this.Listview1.SelectionColor = System.Drawing.Color.DarkBlue;
		this.Listview1.ShowBorder = true;
		this.Listview1.ShowFocusRect = true;
		GlacialComponents.Controls.GlacialList listview3 = this.Listview1;
		size = new System.Drawing.Size(477, 329);
		listview3.Size = size;
		this.Listview1.SortType = GlacialComponents.Controls.SortTypes.InsertionSort;
		this.Listview1.SuperFlatHeaderColor = System.Drawing.Color.White;
		this.Listview1.TabIndex = 34;
		this.Listview1.Text = "GlacialList1";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 14f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(492, 404);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.Listview1);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.Tsearch2);
		this.Controls.Add(this.Tsearch);
		this.Controls.Add(this.Label8);
		this.Controls.Add(this.Label7);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Icon = (System.Drawing.Icon)resources.GetObject("$this.Icon");
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FrmSearchStock";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterParent;
		this.Text = "ค\u0e49นหาส\u0e34นค\u0e49า";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	public void search()
	{
		string text = "Select top 100 * from HT_Products where Pro_no<>''";
		if (Operators.CompareString(Tsearch.Text, "", TextCompare: false) != 0)
		{
			text = text + " and Pro_no like '%" + Tsearch.Text.Replace("*", "%") + "%'";
		}
		if (Operators.CompareString(Tsearch2.Text, "", TextCompare: false) != 0)
		{
			text = text + " and Pro_Name like '%" + Tsearch2.Text.Replace("*", "%") + "%'";
		}
		DataSet dataSet = Module1.connect(text);
		Listview1.Items.Clear();
		Listview1.Refresh();
		Listview1.BeginUpdate();
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
				GlacialList listview = Listview1;
				int count = listview.Items.Count;
				GLItemCollection items = listview.Items;
				object[] array = new object[1];
				DataRow dataRow = dataSet.Tables[0].Rows[num2];
				string columnName = "id";
				array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
				object[] array2 = array;
				bool[] array3 = new bool[1] { true };
				NewLateBinding.LateCall(items, null, "Add", array2, null, null, array3, IgnoreReturn: true);
				if (array3[0])
				{
					dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
				}
				listview.Items[count].SubItems[1].Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["Pro_no"]);
				listview.Items[count].SubItems[2].Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["Pro_Name"]);
				listview.Items[count].SubItems[3].Text = Conversions.ToString(dataSet.Tables[0].Rows[num2]["Pro_Type"]);
				listview = null;
				num2++;
			}
			Listview1.EndUpdate();
			Label1.Text = "พบท\u0e31\u0e49งหมด " + Strings.Format(dataSet.Tables[0].Rows.Count, "#,##0") + " รายการ";
		}
	}

	private void FrmSearchStock_FormClosing(object sender, FormClosingEventArgs e)
	{
		Tsearch.Text = "";
		Tsearch2.Text = "";
	}

	private void SearchBook_Load(object sender, EventArgs e)
	{
		Search_id = "0";
		Timer1.Enabled = true;
		Text = "ค\u0e49นหา ";
	}

	protected override bool ProcessCmdKey(ref Message msg, Keys keyData)
	{
		if (keyData == Keys.Escape)
		{
			Search_id = "0";
			Close();
		}
		bool result = default(bool);
		return result;
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		search();
		Tsearch.Focus();
	}

	private void Tsearch_KeyUp(object sender, KeyEventArgs e)
	{
		if ((e.KeyData == Keys.Return) & (Tsearch.Focused | Tsearch2.Focused))
		{
			try
			{
				Listview1.FocusedItem = Listview1.Items[0];
				Listview1.Items[0].Selected = true;
				Listview1.Focus();
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				ProjectData.ClearProjectError();
			}
		}
	}

	private void Tsearch_TextChanged(object sender, EventArgs e)
	{
		search();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Search_id = "0";
		Close();
	}

	private void Listview1_Click(object sender, EventArgs e)
	{
		if (Listview1.SelectedItems.Count != 0)
		{
			try
			{
				Search_id = Conversions.ToString(NewLateBinding.LateGet(NewLateBinding.LateGet(Listview1.SelectedItems[0], null, "SubItems", new object[1] { 0 }, null, null, null), null, "text", new object[0], null, null, null));
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				Search_id = "0";
				ProjectData.ClearProjectError();
			}
			Close();
		}
	}

	private void Listview1_KeyUp(object sender, KeyEventArgs e)
	{
		if (Conversions.ToBoolean(Operators.AndObject(e.KeyData == Keys.Return, Operators.CompareObjectEqual(NewLateBinding.LateGet(sender, null, "Focused", new object[0], null, null, null), true, TextCompare: false))) && Listview1.SelectedItems.Count != 0)
		{
			try
			{
				Search_id = Conversions.ToString(NewLateBinding.LateGet(NewLateBinding.LateGet(Listview1.SelectedItems[0], null, "SubItems", new object[1] { 0 }, null, null, null), null, "text", new object[0], null, null, null));
			}
			catch (Exception projectError)
			{
				ProjectData.SetProjectError(projectError);
				Search_id = "0";
				ProjectData.ClearProjectError();
			}
			Close();
		}
	}

	private void Listview1_Click_1(object sender, EventArgs e)
	{
	}
}
