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
using iHOTEL2025.My;

namespace iHOTEL2025;

[DesignerGenerated]
public class ReportCustChange : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

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

	internal virtual DateTimePicker DateTimePicker1
	{
		[DebuggerNonUserCode]
		get
		{
			return _DateTimePicker1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = DateTimePicker1_ValueChanged;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged -= value2;
			}
			_DateTimePicker1 = value;
			if (_DateTimePicker1 != null)
			{
				_DateTimePicker1.ValueChanged += value2;
			}
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

	[DebuggerNonUserCode]
	static ReportCustChange()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ReportCustChange()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += ReportDays_Load;
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
		this.Label1 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label2 = new System.Windows.Forms.Label();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(29, 33);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(31, 16);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(63, 29);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(200, 23);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 1;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(279, 29);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(90, 23);
		button2.Size = size;
		this.Button1.TabIndex = 2;
		this.Button1.Text = "ออกรายงาน";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label2.AutoSize = true;
		this.Label2.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(65, 58);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(31, 16);
		label4.Size = size;
		this.Label2.TabIndex = 3;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(402, 111);
		this.ClientSize = size;
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.DateTimePicker1);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "ReportCustChange";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายงานการย\u0e49ายห\u0e49อง";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		Module1.connect("select * from TB_SETTINGS");
		object right = "00:00";
		object right2 = "23:59";
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_changed_room where change_date between '" + Conversions.ToString(DateTimePicker1.Value.Date), " "), right), ":00' and '"), DateTimePicker1.Value.Date), " "), right2), ":59' order by change_date")));
		Module1.localdata.ReportCustIN.Rows.Clear();
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
				Module1.localdata.ReportCustIN.AddReportCustINRow(Strings.Format(DateTimePicker1.Value, "dd/MM/yyyy") + " จากเวลา 00:00 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy") + " 23:59:59", Conversions.ToString(num2 + 1), Conversions.ToString(dataSet.Tables[0].Rows[num2]["cin_no"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Cin_date_in"]), "dd/MM/yyyy HH:mm"), "", "", "", Conversions.ToString(dataSet.Tables[0].Rows[num2]["room_before"]), Conversions.ToString(dataSet.Tables[0].Rows[num2]["room_after"]), Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["change_date"]), "HH:mm"), "", Conversions.ToString(dataSet.Tables[0].Rows[num2]["room_before_price"]), dataSet.Tables[0].Rows[num2]["ToPrice"].ToString(), Conversions.ToString(dataSet.Tables[0].Rows[num2]["cust_name"]), "", "", "", dataSet.Tables[0].Rows[num2]["note"].ToString(), "", "", "", "", "", "", "");
				num2++;
			}
			MyProject.Forms.FrmPrint.Close();
			MyProject.Forms.FrmPrint.Show();
			CrystalReportCustChange crystalReportCustChange = new CrystalReportCustChange();
			crystalReportCustChange.SetDataSource(Module1.localdata);
			MyProject.Forms.FrmPrint.CrystalReportViewer1.ReportSource = crystalReportCustChange;
			MyProject.Forms.FrmPrint.CrystalReportViewer1.RefreshReport();
		}
	}

	private void DateTimePicker1_ValueChanged(object sender, EventArgs e)
	{
		Module1.connect("select * from TB_SETTINGS");
		Label2.Text = "จากเวลา 00:00 ถ\u0e36ง ว\u0e31นท\u0e35\u0e48 " + Strings.Format(DateTimePicker1.Value, "dd/MM/yy") + " 23:59:59";
	}

	private void ReportDays_Load(object sender, EventArgs e)
	{
		DateTimePicker1_ValueChanged(null, null);
	}
}
