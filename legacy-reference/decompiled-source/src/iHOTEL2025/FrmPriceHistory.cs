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
public class FrmPriceHistory : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ListView1")]
	private ListView _ListView1;

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

	public string CUST_NO;

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

	[DebuggerNonUserCode]
	static FrmPriceHistory()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmPriceHistory()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmPriceHistory_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		CUST_NO = "";
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
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.SuspendLayout();
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[6] { this.ColumnHeader1, this.ColumnHeader6, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader5 });
		this.ListView1.Dock = System.Windows.Forms.DockStyle.Fill;
		System.Windows.Forms.ListView listView = this.ListView1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		System.Drawing.Size size = new System.Drawing.Size(785, 411);
		listView2.Size = size;
		this.ListView1.TabIndex = 0;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "ท\u0e35\u0e48";
		this.ColumnHeader1.Width = 40;
		this.ColumnHeader2.Text = "รายการ";
		this.ColumnHeader2.Width = 250;
		this.ColumnHeader3.Text = "ยอดเง\u0e34น";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader3.Width = 100;
		this.ColumnHeader4.Text = "จำนวนเง\u0e34นก\u0e48อนบวกยอด";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader4.Width = 140;
		this.ColumnHeader5.Text = "จำนวนเง\u0e34นหล\u0e31งบวกยอด";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 140;
		this.ColumnHeader6.Text = "ว\u0e31นท\u0e35\u0e48";
		this.ColumnHeader6.Width = 80;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(785, 411);
		this.ClientSize = size;
		this.Controls.Add(this.ListView1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FrmPriceHistory";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ด\u0e39ประว\u0e31ต\u0e34ยอดเง\u0e34น";
		this.ResumeLayout(false);
	}

	private void FrmPriceHistory_Load(object sender, EventArgs e)
	{
		LoadHistory();
	}

	public void LoadHistory()
	{
		DataSet dataSet = Module1.connect("select * from HT_Log_Debt where log_cus='" + CUST_NO + "' order by id desc");
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
					ListView1.Items.Add(Conversions.ToString(dataSet.Tables[0].Rows.Count - num2));
					ListView1.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["log_date"]), "dd-MM-yy"));
					ListViewItem.ListViewSubItemCollection subItems = ListView1.Items[num2].SubItems;
					object[] array = new object[1];
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					string columnName = "log_ds";
					array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
					object[] array2 = array;
					bool[] array3 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", array2, null, null, array3, IgnoreReturn: true);
					if (array3[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
					}
					ListView1.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["log_price"]), "#,##0.00"));
					ListView1.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["log_price_From"]), "#,##0.00"));
					ListView1.Items[num2].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["log_price_To"]), "#,##0.00"));
					num2++;
					continue;
				}
				break;
			}
		}
	}
}
