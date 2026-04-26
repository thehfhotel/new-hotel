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
public class FrmShowBookNotify : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

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

	[AccessedThroughProperty("ColumnHeader7")]
	private ColumnHeader _ColumnHeader7;

	[AccessedThroughProperty("Label4")]
	private Label _Label4;

	[AccessedThroughProperty("Label5")]
	private Label _Label5;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Label8")]
	private Label _Label8;

	[AccessedThroughProperty("Label9")]
	private Label _Label9;

	[AccessedThroughProperty("PanelEx1")]
	private PanelEx _PanelEx1;

	[AccessedThroughProperty("PanelEx2")]
	private PanelEx _PanelEx2;

	[AccessedThroughProperty("Label10")]
	private Label _Label10;

	[AccessedThroughProperty("Label11")]
	private Label _Label11;

	[AccessedThroughProperty("Label12")]
	private Label _Label12;

	[AccessedThroughProperty("Label13")]
	private Label _Label13;

	[AccessedThroughProperty("Label14")]
	private Label _Label14;

	public bool ISOK;

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

	internal virtual ButtonX ButtonX4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX4_Click;
			if (_ButtonX4 != null)
			{
				_ButtonX4.Click -= value2;
			}
			_ButtonX4 = value;
			if (_ButtonX4 != null)
			{
				_ButtonX4.Click += value2;
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

	internal virtual ColumnHeader ColumnHeader7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ColumnHeader7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ColumnHeader7 = value;
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

	internal virtual Label Label5
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label5 = value;
		}
	}

	internal virtual Label Label6
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label6 = value;
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

	internal virtual Label Label9
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label9;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label9 = value;
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

	internal virtual Label Label10
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label10 = value;
		}
	}

	internal virtual Label Label11
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label11 = value;
		}
	}

	internal virtual Label Label12
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label12 = value;
		}
	}

	internal virtual Label Label13
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label13 = value;
		}
	}

	internal virtual Label Label14
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label14 = value;
		}
	}

	[DebuggerNonUserCode]
	static FrmShowBookNotify()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FrmShowBookNotify()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmShowBookNotify_Load;
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FrmShowBookNotify));
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.Label3 = new System.Windows.Forms.Label();
		this.ListView1 = new System.Windows.Forms.ListView();
		this.ColumnHeader1 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader2 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader3 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader4 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader6 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader5 = new System.Windows.Forms.ColumnHeader();
		this.ColumnHeader7 = new System.Windows.Forms.ColumnHeader();
		this.Label4 = new System.Windows.Forms.Label();
		this.Label5 = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Label8 = new System.Windows.Forms.Label();
		this.Label9 = new System.Windows.Forms.Label();
		this.PanelEx1 = new DevComponents.DotNetBar.PanelEx();
		this.PanelEx2 = new DevComponents.DotNetBar.PanelEx();
		this.Label10 = new System.Windows.Forms.Label();
		this.Label11 = new System.Windows.Forms.Label();
		this.Label12 = new System.Windows.Forms.Label();
		this.Label13 = new System.Windows.Forms.Label();
		this.Label14 = new System.Windows.Forms.Label();
		this.PanelEx1.SuspendLayout();
		this.PanelEx2.SuspendLayout();
		this.SuspendLayout();
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		System.Drawing.Point location = new System.Drawing.Point(495, 481);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		System.Drawing.Size size = new System.Drawing.Size(95, 43);
		buttonX2.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 3;
		this.ButtonX1.Text = "ป\u0e34ด";
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX4;
		location = new System.Drawing.Point(11, 119);
		buttonX3.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX4;
		size = new System.Drawing.Size(182, 52);
		buttonX4.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 5;
		this.ButtonX4.Text = "แก\u0e49ไข/ชำระเง\u0e34น";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX3;
		location = new System.Drawing.Point(209, 119);
		buttonX5.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX3;
		size = new System.Drawing.Size(182, 52);
		buttonX6.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 4;
		this.ButtonX3.Text = "   ยกเล\u0e34กการจอง\r\n";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.Label1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(4, 7);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(113, 23);
		label2.Size = size;
		this.Label1.TabIndex = 6;
		this.Label1.Text = "การจองเลขท\u0e35\u0e48 :";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label2.BackColor = System.Drawing.Color.Transparent;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.Color.Green;
		System.Windows.Forms.Label label3 = this.Label2;
		location = new System.Drawing.Point(121, 5);
		label3.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label4 = this.Label2;
		size = new System.Drawing.Size(124, 25);
		label4.Size = size;
		this.Label2.TabIndex = 7;
		this.Label2.Text = "R000001";
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX2;
		location = new System.Drawing.Point(408, 119);
		buttonX7.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX2;
		size = new System.Drawing.Size(182, 52);
		buttonX8.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 8;
		this.ButtonX2.Text = "ไม\u0e48แจ\u0e49งเต\u0e37อน\r\nรายการน\u0e35\u0e49อ\u0e35ก";
		this.Label3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label3.AutoSize = true;
		this.Label3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label5 = this.Label3;
		location = new System.Drawing.Point(11, 178);
		label5.Location = location;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label6 = this.Label3;
		size = new System.Drawing.Size(85, 19);
		label6.Size = size;
		this.Label3.TabIndex = 9;
		this.Label3.Text = "รายการห\u0e49อง";
		this.ListView1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ListView1.Columns.AddRange(new System.Windows.Forms.ColumnHeader[7] { this.ColumnHeader1, this.ColumnHeader2, this.ColumnHeader3, this.ColumnHeader4, this.ColumnHeader6, this.ColumnHeader5, this.ColumnHeader7 });
		this.ListView1.Font = new System.Drawing.Font("Tahoma", 9f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.ListView1.FullRowSelect = true;
		System.Windows.Forms.ListView listView = this.ListView1;
		location = new System.Drawing.Point(13, 200);
		listView.Location = location;
		this.ListView1.Name = "ListView1";
		System.Windows.Forms.ListView listView2 = this.ListView1;
		size = new System.Drawing.Size(577, 274);
		listView2.Size = size;
		this.ListView1.TabIndex = 10;
		this.ListView1.UseCompatibleStateImageBehavior = false;
		this.ListView1.View = System.Windows.Forms.View.Details;
		this.ColumnHeader1.Text = "เลขห\u0e49อง";
		this.ColumnHeader1.Width = 100;
		this.ColumnHeader2.Text = "ว\u0e31นท\u0e35\u0e48เข\u0e49า";
		this.ColumnHeader2.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader2.Width = 80;
		this.ColumnHeader3.Text = "ว\u0e31นท\u0e35\u0e48ออก";
		this.ColumnHeader3.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader3.Width = 80;
		this.ColumnHeader4.Text = "จำนวนค\u0e37น";
		this.ColumnHeader4.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader6.Text = "จำนวนห\u0e49อง";
		this.ColumnHeader6.TextAlign = System.Windows.Forms.HorizontalAlignment.Center;
		this.ColumnHeader6.Width = 70;
		this.ColumnHeader5.Text = "Rate";
		this.ColumnHeader5.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader5.Width = 80;
		this.ColumnHeader7.Text = "ราคารวม";
		this.ColumnHeader7.TextAlign = System.Windows.Forms.HorizontalAlignment.Right;
		this.ColumnHeader7.Width = 80;
		this.Label4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label7 = this.Label4;
		location = new System.Drawing.Point(8, 38);
		label7.Location = location;
		this.Label4.Name = "Label4";
		System.Windows.Forms.Label label8 = this.Label4;
		size = new System.Drawing.Size(109, 19);
		label8.Size = size;
		this.Label4.TabIndex = 11;
		this.Label4.Text = "ช\u0e37\u0e48อ :";
		this.Label4.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label9 = this.Label5;
		location = new System.Drawing.Point(8, 71);
		label9.Location = location;
		this.Label5.Name = "Label5";
		System.Windows.Forms.Label label10 = this.Label5;
		size = new System.Drawing.Size(109, 19);
		label10.Size = size;
		this.Label5.TabIndex = 12;
		this.Label5.Text = "เบอร\u0e4cโทร :";
		this.Label5.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
		this.Label6.BackColor = System.Drawing.Color.Transparent;
		this.Label6.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label6.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label11 = this.Label6;
		location = new System.Drawing.Point(121, 38);
		label11.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label12 = this.Label6;
		size = new System.Drawing.Size(269, 23);
		label12.Size = size;
		this.Label6.TabIndex = 13;
		this.Label6.Text = "R000001";
		this.Label7.BackColor = System.Drawing.Color.Transparent;
		this.Label7.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label7.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		System.Windows.Forms.Label label13 = this.Label7;
		location = new System.Drawing.Point(121, 71);
		label13.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label14 = this.Label7;
		size = new System.Drawing.Size(248, 23);
		label14.Size = size;
		this.Label7.TabIndex = 14;
		this.Label7.Text = "R000001";
		this.Label8.AutoSize = true;
		this.Label8.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label15 = this.Label8;
		location = new System.Drawing.Point(115, 476);
		label15.Location = location;
		this.Label8.Name = "Label8";
		System.Windows.Forms.Label label16 = this.Label8;
		size = new System.Drawing.Size(67, 19);
		label16.Size = size;
		this.Label8.TabIndex = 15;
		this.Label8.Text = "ราคารวม";
		this.Label9.BackColor = System.Drawing.Color.Black;
		this.Label9.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label9.ForeColor = System.Drawing.Color.Lime;
		System.Windows.Forms.Label label17 = this.Label9;
		location = new System.Drawing.Point(86, 497);
		label17.Location = location;
		this.Label9.Name = "Label9";
		System.Windows.Forms.Label label18 = this.Label9;
		size = new System.Drawing.Size(135, 30);
		label18.Size = size;
		this.Label9.TabIndex = 16;
		this.Label9.Text = "R000001";
		this.Label9.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.PanelEx1.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx1.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx1.Controls.Add(this.Label14);
		this.PanelEx1.Controls.Add(this.Label1);
		this.PanelEx1.Controls.Add(this.Label2);
		this.PanelEx1.Controls.Add(this.Label4);
		this.PanelEx1.Controls.Add(this.Label7);
		this.PanelEx1.Controls.Add(this.Label5);
		this.PanelEx1.Controls.Add(this.Label6);
		DevComponents.DotNetBar.PanelEx panelEx = this.PanelEx1;
		location = new System.Drawing.Point(11, 8);
		panelEx.Location = location;
		this.PanelEx1.Name = "PanelEx1";
		DevComponents.DotNetBar.PanelEx panelEx2 = this.PanelEx1;
		size = new System.Drawing.Size(397, 101);
		panelEx2.Size = size;
		this.PanelEx1.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx1.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx1.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx1.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx1.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx1.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx1.Style.GradientAngle = 90;
		this.PanelEx1.TabIndex = 17;
		this.PanelEx2.CanvasColor = System.Drawing.SystemColors.Control;
		this.PanelEx2.ColorSchemeStyle = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.PanelEx2.Controls.Add(this.Label10);
		this.PanelEx2.Controls.Add(this.Label11);
		DevComponents.DotNetBar.PanelEx panelEx3 = this.PanelEx2;
		location = new System.Drawing.Point(407, 8);
		panelEx3.Location = location;
		this.PanelEx2.Name = "PanelEx2";
		DevComponents.DotNetBar.PanelEx panelEx4 = this.PanelEx2;
		size = new System.Drawing.Size(183, 101);
		panelEx4.Size = size;
		this.PanelEx2.Style.Alignment = System.Drawing.StringAlignment.Center;
		this.PanelEx2.Style.BackColor1.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground;
		this.PanelEx2.Style.BackColor2.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBackground2;
		this.PanelEx2.Style.Border = DevComponents.DotNetBar.eBorderType.SingleLine;
		this.PanelEx2.Style.BorderColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelBorder;
		this.PanelEx2.Style.ForeColor.ColorSchemePart = DevComponents.DotNetBar.eColorSchemePart.PanelText;
		this.PanelEx2.Style.GradientAngle = 90;
		this.PanelEx2.TabIndex = 18;
		this.Label10.AutoSize = true;
		this.Label10.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label19 = this.Label10;
		location = new System.Drawing.Point(55, 8);
		label19.Location = location;
		this.Label10.Name = "Label10";
		System.Windows.Forms.Label label20 = this.Label10;
		size = new System.Drawing.Size(75, 19);
		label20.Size = size;
		this.Label10.TabIndex = 6;
		this.Label10.Text = "กล\u0e38\u0e48มล\u0e39กค\u0e49า";
		this.Label11.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label11.ForeColor = System.Drawing.Color.FromArgb(192, 0, 192);
		System.Windows.Forms.Label label21 = this.Label11;
		location = new System.Drawing.Point(6, 40);
		label21.Location = location;
		this.Label11.Name = "Label11";
		System.Windows.Forms.Label label22 = this.Label11;
		size = new System.Drawing.Size(169, 49);
		label22.Size = size;
		this.Label11.TabIndex = 7;
		this.Label11.Text = "R000001";
		this.Label11.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label12.BackColor = System.Drawing.Color.Black;
		this.Label12.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label12.ForeColor = System.Drawing.Color.Yellow;
		System.Windows.Forms.Label label23 = this.Label12;
		location = new System.Drawing.Point(19, 497);
		label23.Location = location;
		this.Label12.Name = "Label12";
		System.Windows.Forms.Label label24 = this.Label12;
		size = new System.Drawing.Size(61, 30);
		label24.Size = size;
		this.Label12.TabIndex = 20;
		this.Label12.Text = "30";
		this.Label12.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label13.AutoSize = true;
		this.Label13.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Label label25 = this.Label13;
		location = new System.Drawing.Point(20, 477);
		label25.Location = location;
		this.Label13.Name = "Label13";
		System.Windows.Forms.Label label26 = this.Label13;
		size = new System.Drawing.Size(62, 19);
		label26.Size = size;
		this.Label13.TabIndex = 19;
		this.Label13.Text = "รวมห\u0e49อง";
		this.Label14.BackColor = System.Drawing.Color.Transparent;
		this.Label14.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label14.ForeColor = System.Drawing.Color.Green;
		System.Windows.Forms.Label label27 = this.Label14;
		location = new System.Drawing.Point(227, 5);
		label27.Location = location;
		this.Label14.Name = "Label14";
		System.Windows.Forms.Label label28 = this.Label14;
		size = new System.Drawing.Size(167, 25);
		label28.Size = size;
		this.Label14.TabIndex = 15;
		this.Label14.Text = "R000001";
		this.Label14.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(602, 536);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.Label12);
		this.Controls.Add(this.Label13);
		this.Controls.Add(this.PanelEx2);
		this.Controls.Add(this.PanelEx1);
		this.Controls.Add(this.Label9);
		this.Controls.Add(this.Label8);
		this.Controls.Add(this.ListView1);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX1);
		this.DoubleBuffered = true;
		this.Name = "FrmShowBookNotify";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "แจ\u0e49งเต\u0e37อนเมน\u0e39";
		this.PanelEx1.ResumeLayout(false);
		this.PanelEx2.ResumeLayout(false);
		this.PanelEx2.PerformLayout();
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void FrmShowBookNotify_Load(object sender, EventArgs e)
	{
		ISOK = false;
		loadReeom();
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Close();
	}

	public void loadReeom()
	{
		ListView1.Items.Clear();
		DataSet dataSet = Module1.connect("select * from HT_Book_H where Book_ID='" + Label2.Text + "' ");
		Label6.Text = dataSet.Tables[0].Rows[0]["Book_Cust_Name"].ToString() + " " + dataSet.Tables[0].Rows[0]["Book_Cust_Name2"].ToString();
		Label7.Text = dataSet.Tables[0].Rows[0]["Book_Cust_Tel"].ToString();
		Label9.Text = Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Book_Price_Total"]), "#,##0.00");
		Label11.Text = "-";
		Label12.Text = "0";
		Label14.Text = "(" + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Book_date"]), "dd/MM/yy") + ")";
		dataSet = Module1.connect("select cust_type from View_Customers where cust_no='" + dataSet.Tables[0].Rows[0]["Book_Cust_ID"].ToString() + "'");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			Label11.Text = dataSet.Tables[0].Rows[0]["cust_type"].ToString();
		}
		dataSet = Module1.connect("select * from HT_Book_Ds where book_no='" + Label2.Text + "' order by  Book_Room_Type,Book_Room_Start");
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
					object[] array2 = array;
					DataRow dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow2 = dataRow;
					string columnName = "Book_Room_Type";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					NewLateBinding.LateCall(items, null, "Add", arguments, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					listView.Items[listView.Items.Count - 1].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Book_Room_Start"]), "dd/MM/yy"));
					listView.Items[listView.Items.Count - 1].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Book_Room_end"]), "dd/MM/yy"));
					ListViewItem.ListViewSubItemCollection subItems = listView.Items[listView.Items.Count - 1].SubItems;
					array3 = new object[1];
					object[] array5 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow3 = dataRow;
					columnName = "Book_Room_Night";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					array = array3;
					object[] arguments2 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems, null, "Add", arguments2, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					ListViewItem.ListViewSubItemCollection subItems2 = listView.Items[listView.Items.Count - 1].SubItems;
					array3 = new object[1];
					object[] array6 = array3;
					dataRow = dataSet.Tables[0].Rows[num2];
					DataRow dataRow4 = dataRow;
					columnName = "Book_Room_Num";
					array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
					array = array3;
					object[] arguments3 = array;
					array4 = new bool[1] { true };
					NewLateBinding.LateCall(subItems2, null, "Add", arguments3, null, null, array4, IgnoreReturn: true);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					listView.Items[listView.Items.Count - 1].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Book_Room_Price"]), "#,##0.00"));
					listView.Items[listView.Items.Count - 1].SubItems.Add(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num2]["Book_Room_PriceToTal"]), "#,##0.00"));
					listView = null;
					Label12.Text = Conversions.ToString(Operators.AddObject(Conversions.ToDecimal(Label12.Text), dataSet.Tables[0].Rows[num2]["Book_Room_Num"]));
					num2++;
					continue;
				}
				break;
			}
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select Book_ID,Book_Status,Book_Price_Pay,Book_Cust_ID from HT_Book_H where Book_ID='" + Label2.Text + "'");
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Book_Status"].ToString(), "ยกเล\u0e34ก", TextCompare: false) == 0)
		{
			MessageBox.Show("รายการน\u0e35\u0e49ได\u0e49ยกเล\u0e34กไปแล\u0e49ว");
		}
		else if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Book_Status"].ToString(), "เข\u0e49าพ\u0e31ก", TextCompare: false) == 0)
		{
			MessageBox.Show("รายการน\u0e35\u0e49ได\u0e49เข\u0e49าพ\u0e31กไปแล\u0e49ว");
		}
		else if (MessageBox.Show("ค\u0e38ณต\u0e49องการยกเล\u0e34กใบจองหร\u0e37อไม\u0e48", "ยกเล\u0e34ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			Module1.connect("update HT_Book_H set Book_Status='ยกเล\u0e34ก' where Book_ID='" + Label2.Text + "'");
			Module1.connect("update HT_Book_ds set Book_status=3 where Book_No='" + Label2.Text + "'");
			Module1.connect("delete from  HT_Book_Date where Book_no='" + Label2.Text + "'");
			Module1.SET_STATUS_BOOKING(Label2.Text, bool_4: true);
			MessageBox.Show("ยกเล\u0e34กรายการเสร\u0e47จเร\u0e35ยบร\u0e49อย");
			ISOK = true;
			Close();
		}
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select book_room_type,Book_ID,Book_Status,Book_Price_Pay,Book_Cust_ID from HT_Book_H where Book_ID='" + Label2.Text + "'");
		if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Book_Status"].ToString(), "ยกเล\u0e34ก", TextCompare: false) == 0)
		{
			MessageBox.Show("รายการน\u0e35\u0e49ได\u0e49ยกเล\u0e34กไปแล\u0e49ว");
		}
		else if (Operators.CompareString(dataSet.Tables[0].Rows[0]["Book_Status"].ToString(), "เข\u0e49าพ\u0e31ก", TextCompare: false) == 0)
		{
			MessageBox.Show("รายการน\u0e35\u0e49ได\u0e49เข\u0e49าพ\u0e31กไปแล\u0e49ว");
		}
		else if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["book_room_type"], 1, TextCompare: false))
		{
			FrmAddBook frmAddBook = new FrmAddBook();
			frmAddBook.EDIT_ID = Label2.Text;
			frmAddBook.ShowDialog();
			ISOK = true;
			loadReeom();
		}
		else
		{
			FrmAddBook2 frmAddBook2 = new FrmAddBook2();
			frmAddBook2.EDIT_ID = Label2.Text;
			frmAddBook2.ShowDialog();
			ISOK = true;
			loadReeom();
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		Module1.connect("update HT_Book_H set Book_Notify_Note='ไม\u0e48แจ\u0e49งเต\u0e37อน' where Book_ID='" + Label2.Text + "'");
		ISOK = true;
		Close();
	}
}
