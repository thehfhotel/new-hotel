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

namespace iHOTEL2025;

[DesignerGenerated]
public class FormSelectRoom : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("Lno")]
	private Label _Lno;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("DateTimePicker1")]
	private DateTimePicker _DateTimePicker1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("Bclean")]
	private ButtonX _Bclean;

	[AccessedThroughProperty("Bcheckin")]
	private ButtonX _Bcheckin;

	[AccessedThroughProperty("Bbook")]
	private ButtonX _Bbook;

	[AccessedThroughProperty("Bbookc")]
	private ButtonX _Bbookc;

	[AccessedThroughProperty("Bcheckout")]
	private ButtonX _Bcheckout;

	[AccessedThroughProperty("BcleanOk")]
	private ButtonX _BcleanOk;

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

	internal virtual Label Lno
	{
		[DebuggerNonUserCode]
		get
		{
			return _Lno;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Lno = value;
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
			_DateTimePicker1 = value;
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

	internal virtual ButtonX Bclean
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bclean;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Bclean_Click;
			if (_Bclean != null)
			{
				_Bclean.Click -= value2;
			}
			_Bclean = value;
			if (_Bclean != null)
			{
				_Bclean.Click += value2;
			}
		}
	}

	internal virtual ButtonX Bcheckin
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bcheckin;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Bcheckin = value;
		}
	}

	internal virtual ButtonX Bbook
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bbook;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Bbook = value;
		}
	}

	internal virtual ButtonX Bbookc
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bbookc;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Bbookc = value;
		}
	}

	internal virtual ButtonX Bcheckout
	{
		[DebuggerNonUserCode]
		get
		{
			return _Bcheckout;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Bcheckout = value;
		}
	}

	internal virtual ButtonX BcleanOk
	{
		[DebuggerNonUserCode]
		get
		{
			return _BcleanOk;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = BcleanOk_Click;
			if (_BcleanOk != null)
			{
				_BcleanOk.Click -= value2;
			}
			_BcleanOk = value;
			if (_BcleanOk != null)
			{
				_BcleanOk.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FormSelectRoom()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormSelectRoom()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormSelectRoom_Load;
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
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.Label1 = new System.Windows.Forms.Label();
		this.Lno = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.DateTimePicker1 = new System.Windows.Forms.DateTimePicker();
		this.Bbook = new DevComponents.DotNetBar.ButtonX();
		this.Bcheckin = new DevComponents.DotNetBar.ButtonX();
		this.Bclean = new DevComponents.DotNetBar.ButtonX();
		this.Bcheckout = new DevComponents.DotNetBar.ButtonX();
		this.Bbookc = new DevComponents.DotNetBar.ButtonX();
		this.BcleanOk = new DevComponents.DotNetBar.ButtonX();
		this.PanelEx1.SuspendLayout();
		this.SuspendLayout();
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.BcleanOk);
		this.PanelEx1.Controls.Add(this.Bbookc);
		this.PanelEx1.Controls.Add(this.Bcheckout);
		this.PanelEx1.Controls.Add(this.Bclean);
		this.PanelEx1.Controls.Add(this.Bcheckin);
		this.PanelEx1.Controls.Add(this.Bbook);
		this.PanelEx1.Controls.Add(this.DateTimePicker1);
		this.PanelEx1.Controls.Add(this.Lno);
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Dock = System.Windows.Forms.DockStyle.Fill;
		this.PanelEx1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		System.Drawing.Size size = new System.Drawing.Size(452, 240);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 0;
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(7, 10);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(99, 19);
		label2.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "หมายเลขห\u0e49อง";
		this.Lno.AutoSize = true;
		this.Lno.ForeColor = System.Drawing.Color.FromArgb(0, 0, 192);
		System.Windows.Forms.Label lno = this.Lno;
		location = new System.Drawing.Point(102, 10);
		lno.Location = location;
		this.Lno.Name = "Lno";
		System.Windows.Forms.Label lno2 = this.Lno;
		size = new System.Drawing.Size(99, 19);
		lno2.Size = size;
		this.Lno.TabIndex = 1;
		this.Lno.Text = "หมายเลขห\u0e49อง";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(225, 10);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(38, 19);
		label4.Size = size;
		this.Label2.TabIndex = 0;
		this.Label2.Text = "ว\u0e31นท\u0e35\u0e48";
		this.DateTimePicker1.Enabled = false;
		System.Windows.Forms.DateTimePicker dateTimePicker = this.DateTimePicker1;
		location = new System.Drawing.Point(266, 7);
		dateTimePicker.Location = location;
		this.DateTimePicker1.Name = "DateTimePicker1";
		System.Windows.Forms.DateTimePicker dateTimePicker2 = this.DateTimePicker1;
		size = new System.Drawing.Size(174, 27);
		dateTimePicker2.Size = size;
		this.DateTimePicker1.TabIndex = 2;
		this.Bbook.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bbook.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bbook.Enabled = false;
		this.Bbook.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bbook = this.Bbook;
		location = new System.Drawing.Point(15, 53);
		bbook.Location = location;
		this.Bbook.Name = "Bbook";
		DevComponents.DotNetBar.ButtonX bbook2 = this.Bbook;
		size = new System.Drawing.Size(203, 48);
		bbook2.Size = size;
		this.Bbook.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.Bbook.TabIndex = 3;
		this.Bbook.Text = "จองห\u0e49องพ\u0e31ก\r\nด\u0e39รายระเอ\u0e35ยดการจอง";
		this.Bcheckin.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bcheckin.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bcheckin.Enabled = false;
		this.Bcheckin.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bcheckin = this.Bcheckin;
		location = new System.Drawing.Point(15, 115);
		bcheckin.Location = location;
		this.Bcheckin.Name = "Bcheckin";
		DevComponents.DotNetBar.ButtonX bcheckin2 = this.Bcheckin;
		size = new System.Drawing.Size(203, 48);
		bcheckin2.Size = size;
		this.Bcheckin.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.Bcheckin.TabIndex = 4;
		this.Bcheckin.Text = "Check in\r\nด\u0e39รายละเอ\u0e35ยดการเข\u0e49าพ\u0e31ก";
		this.Bclean.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bclean.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bclean.Enabled = false;
		this.Bclean.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bclean = this.Bclean;
		location = new System.Drawing.Point(15, 177);
		bclean.Location = location;
		this.Bclean.Name = "Bclean";
		DevComponents.DotNetBar.ButtonX bclean2 = this.Bclean;
		size = new System.Drawing.Size(203, 48);
		bclean2.Size = size;
		this.Bclean.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.Bclean.TabIndex = 5;
		this.Bclean.Text = "รอ ทำความสะอาด";
		this.Bcheckout.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bcheckout.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bcheckout.Enabled = false;
		this.Bcheckout.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bcheckout = this.Bcheckout;
		location = new System.Drawing.Point(235, 115);
		bcheckout.Location = location;
		this.Bcheckout.Name = "Bcheckout";
		DevComponents.DotNetBar.ButtonX bcheckout2 = this.Bcheckout;
		size = new System.Drawing.Size(203, 48);
		bcheckout2.Size = size;
		this.Bcheckout.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.Bcheckout.TabIndex = 6;
		this.Bcheckout.Text = "Check Out";
		this.Bbookc.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.Bbookc.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.Bbookc.Enabled = false;
		this.Bbookc.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bbookc = this.Bbookc;
		location = new System.Drawing.Point(235, 53);
		bbookc.Location = location;
		this.Bbookc.Name = "Bbookc";
		DevComponents.DotNetBar.ButtonX bbookc2 = this.Bbookc;
		size = new System.Drawing.Size(203, 48);
		bbookc2.Size = size;
		this.Bbookc.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.Bbookc.TabIndex = 7;
		this.Bbookc.Text = "ยกเล\u0e34กจองห\u0e49องพ\u0e31ก";
		this.BcleanOk.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.BcleanOk.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.BcleanOk.Enabled = false;
		this.BcleanOk.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX bcleanOk = this.BcleanOk;
		location = new System.Drawing.Point(235, 177);
		bcleanOk.Location = location;
		this.BcleanOk.Name = "BcleanOk";
		DevComponents.DotNetBar.ButtonX bcleanOk2 = this.BcleanOk;
		size = new System.Drawing.Size(203, 48);
		bcleanOk2.Size = size;
		this.BcleanOk.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.BcleanOk.TabIndex = 8;
		this.BcleanOk.Text = "รอ ทำความสะอาด\r\nเร\u0e35ยบร\u0e49อย";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(452, 240);
		this.ClientSize = size;
		this.Controls.Add(this.PanelEx1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FormSelectRoom";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "FormSelectRoom";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx1.PerformLayout();
		this.ResumeLayout(false);
	}

	private void FormSelectRoom_Load(object sender, EventArgs e)
	{
		RefreshStatus();
	}

	public void RefreshStatus()
	{
		Bbook.Enabled = false;
		Bbookc.Enabled = false;
		Bcheckin.Enabled = false;
		Bcheckout.Enabled = false;
		Bclean.Enabled = false;
		BcleanOk.Enabled = false;
		DataSet dataSet = Module1.connect("select * from HT_Room_Status where room_date='" + Conversions.ToString(DateTimePicker1.Value.Date) + "' and room_no='" + Lno.Text + "'");
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			Bbook.Enabled = true;
			Bbookc.Enabled = false;
			Bcheckin.Enabled = true;
			Bcheckout.Enabled = false;
		}
		else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["room_status"], "จอง", TextCompare: false))
		{
			Bbook.Enabled = true;
			Bbookc.Enabled = true;
		}
		else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["room_status"], "เช\u0e48า", TextCompare: false))
		{
			Bcheckin.Enabled = true;
			Bcheckout.Enabled = true;
		}
		dataSet = Module1.connect("select * from HT_Rooms where room_no='" + Lno.Text + "'");
		if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["room_clean"], "no", TextCompare: false))
		{
			Bclean.Enabled = true;
			BcleanOk.Enabled = false;
		}
		else
		{
			Bclean.Enabled = false;
			BcleanOk.Enabled = true;
		}
	}

	private void BcleanOk_Click(object sender, EventArgs e)
	{
		Module1.connect("update HT_Rooms set room_clean='no' where room_no='" + Lno.Text + "'");
		RefreshStatus();
	}

	private void Bclean_Click(object sender, EventArgs e)
	{
		Module1.connect("update HT_Rooms set room_clean='yes' where room_no='" + Lno.Text + "'");
		RefreshStatus();
	}
}
