using System;
using System.Collections;
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
public class ClickUSE : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("ButtonX2")]
	private ButtonX _ButtonX2;

	[AccessedThroughProperty("ButtonX3")]
	private ButtonX _ButtonX3;

	[AccessedThroughProperty("ButtonX6")]
	private ButtonX _ButtonX6;

	[AccessedThroughProperty("ButtonX4")]
	private ButtonX _ButtonX4;

	[AccessedThroughProperty("ButtonX5")]
	private ButtonX _ButtonX5;

	[AccessedThroughProperty("ButtonX7")]
	private ButtonX _ButtonX7;

	[AccessedThroughProperty("ButtonX8")]
	private ButtonX _ButtonX8;

	[AccessedThroughProperty("ButtonX9")]
	private ButtonX _ButtonX9;

	[AccessedThroughProperty("ButtonX10")]
	private ButtonX _ButtonX10;

	[AccessedThroughProperty("ButtonX11")]
	private ButtonX _ButtonX11;

	[AccessedThroughProperty("ButtonX12")]
	private ButtonX _ButtonX12;

	[AccessedThroughProperty("ButtonX13")]
	private ButtonX _ButtonX13;

	[AccessedThroughProperty("ButtonX14")]
	private ButtonX _ButtonX14;

	[AccessedThroughProperty("ButtonX15")]
	private ButtonX _ButtonX15;

	[AccessedThroughProperty("ButtonX16")]
	private ButtonX _ButtonX16;

	[AccessedThroughProperty("ButtonX17")]
	private ButtonX _ButtonX17;

	[AccessedThroughProperty("ButtonItem1")]
	private ButtonItem _ButtonItem1;

	[AccessedThroughProperty("ButtonItem2")]
	private ButtonItem _ButtonItem2;

	[AccessedThroughProperty("ButtonItem3")]
	private ButtonItem _ButtonItem3;

	[AccessedThroughProperty("ButtonBackMoney")]
	private ButtonX _ButtonBackMoney;

	public string RoomNo;

	public bool ISOK;

	public ArrayList RoomArr;

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

	internal virtual ButtonX ButtonX6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX6;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX6_Click;
			if (_ButtonX6 != null)
			{
				_ButtonX6.Click -= value2;
			}
			_ButtonX6 = value;
			if (_ButtonX6 != null)
			{
				_ButtonX6.Click += value2;
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

	internal virtual ButtonX ButtonX5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX5;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX5_Click;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click -= value2;
			}
			_ButtonX5 = value;
			if (_ButtonX5 != null)
			{
				_ButtonX5.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX7;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX7_Click;
			if (_ButtonX7 != null)
			{
				_ButtonX7.Click -= value2;
			}
			_ButtonX7 = value;
			if (_ButtonX7 != null)
			{
				_ButtonX7.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX8
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX8;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX8_Click;
			if (_ButtonX8 != null)
			{
				_ButtonX8.Click -= value2;
			}
			_ButtonX8 = value;
			if (_ButtonX8 != null)
			{
				_ButtonX8.Click += value2;
			}
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

	internal virtual ButtonX ButtonX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX10;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX10_Click;
			if (_ButtonX10 != null)
			{
				_ButtonX10.Click -= value2;
			}
			_ButtonX10 = value;
			if (_ButtonX10 != null)
			{
				_ButtonX10.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX11;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX11_Click;
			if (_ButtonX11 != null)
			{
				_ButtonX11.Click -= value2;
			}
			_ButtonX11 = value;
			if (_ButtonX11 != null)
			{
				_ButtonX11.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX12;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX12_Click;
			if (_ButtonX12 != null)
			{
				_ButtonX12.Click -= value2;
			}
			_ButtonX12 = value;
			if (_ButtonX12 != null)
			{
				_ButtonX12.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX13;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX13_Click;
			if (_ButtonX13 != null)
			{
				_ButtonX13.Click -= value2;
			}
			_ButtonX13 = value;
			if (_ButtonX13 != null)
			{
				_ButtonX13.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_4
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX14;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX14_Click;
			if (_ButtonX14 != null)
			{
				_ButtonX14.Click -= value2;
			}
			_ButtonX14 = value;
			if (_ButtonX14 != null)
			{
				_ButtonX14.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_5
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX15;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX15_Click;
			if (_ButtonX15 != null)
			{
				_ButtonX15.Click -= value2;
			}
			_ButtonX15 = value;
			if (_ButtonX15 != null)
			{
				_ButtonX15.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_6
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX16;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX16_Click;
			if (_ButtonX16 != null)
			{
				_ButtonX16.Click -= value2;
			}
			_ButtonX16 = value;
			if (_ButtonX16 != null)
			{
				_ButtonX16.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonX_7
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonX17;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonX17_Click;
			if (_ButtonX17 != null)
			{
				_ButtonX17.Click -= value2;
			}
			_ButtonX17 = value;
			if (_ButtonX17 != null)
			{
				_ButtonX17.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem1_Click;
			if (_ButtonItem1 != null)
			{
				_ButtonItem1.Click -= value2;
			}
			_ButtonItem1 = value;
			if (_ButtonItem1 != null)
			{
				_ButtonItem1.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem2
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem2_Click;
			if (_ButtonItem2 != null)
			{
				_ButtonItem2.Click -= value2;
			}
			_ButtonItem2 = value;
			if (_ButtonItem2 != null)
			{
				_ButtonItem2.Click += value2;
			}
		}
	}

	internal virtual ButtonItem ButtonItem3
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonItem3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonItem3_Click;
			if (_ButtonItem3 != null)
			{
				_ButtonItem3.Click -= value2;
			}
			_ButtonItem3 = value;
			if (_ButtonItem3 != null)
			{
				_ButtonItem3.Click += value2;
			}
		}
	}

	internal virtual ButtonX ButtonBackMoney
	{
		[DebuggerNonUserCode]
		get
		{
			return _ButtonBackMoney;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ButtonBackMoney_Click;
			if (_ButtonBackMoney != null)
			{
				_ButtonBackMoney.Click -= value2;
			}
			_ButtonBackMoney = value;
			if (_ButtonBackMoney != null)
			{
				_ButtonBackMoney.Click += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static ClickUSE()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ClickUSE()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += ClickBook_Load;
		base.FormClosing += ClickBook_FormClosing;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		RoomNo = "";
		ISOK = false;
		RoomArr = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.ClickUSE));
		this.ButtonBackMoney = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonItem1 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonItem2 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonX_6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonItem3 = new DevComponents.DotNetBar.ButtonItem();
		this.ButtonX_5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_1 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX_0 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX9 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX8 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX7 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX5 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX4 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX3 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX2 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX6 = new DevComponents.DotNetBar.ButtonX();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.SuspendLayout();
		this.ButtonBackMoney.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonBackMoney.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonBackMoney.FocusCuesEnabled = false;
		this.ButtonBackMoney.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonBackMoney.ForeColor = System.Drawing.Color.FromArgb(222, 0, 0);
		this.ButtonBackMoney.Image = (System.Drawing.Image)resources.GetObject("ButtonBackMoney.Image");
		DevComponents.DotNetBar.ButtonX buttonBackMoney = this.ButtonBackMoney;
		System.Drawing.Point location = new System.Drawing.Point(180, 192);
		buttonBackMoney.Location = location;
		this.ButtonBackMoney.Name = "ButtonBackMoney";
		DevComponents.DotNetBar.ButtonX buttonBackMoney2 = this.ButtonBackMoney;
		System.Drawing.Size size = new System.Drawing.Size(162, 52);
		buttonBackMoney2.Size = size;
		this.ButtonBackMoney.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonBackMoney.TabIndex = 16;
		this.ButtonBackMoney.Text = "ค\u0e37นเง\u0e34นส\u0e48วนเก\u0e34น";
		this.ButtonX_7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_7.FocusCuesEnabled = false;
		this.ButtonX_7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_7.Image = (System.Drawing.Image)resources.GetObject("ButtonX17.Image");
		DevComponents.DotNetBar.ButtonX buttonX_ = this.ButtonX_7;
		location = new System.Drawing.Point(10, 192);
		buttonX_.Location = location;
		this.ButtonX_7.Name = "ButtonX17";
		DevComponents.DotNetBar.ButtonX buttonX_2 = this.ButtonX_7;
		size = new System.Drawing.Size(162, 52);
		buttonX_2.Size = size;
		this.ButtonX_7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_7.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[2] { this.ButtonItem1, this.ButtonItem2 });
		this.ButtonX_7.TabIndex = 15;
		this.ButtonX_7.Text = "Guest Folio";
		this.ButtonItem1.GlobalItem = false;
		this.ButtonItem1.Name = "ButtonItem1";
		this.ButtonItem1.Text = "แบบปกต\u0e34";
		this.ButtonItem2.GlobalItem = false;
		this.ButtonItem2.Name = "ButtonItem2";
		this.ButtonItem2.Text = "แบบราชการ";
		this.ButtonX_6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_6.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_6.FocusCuesEnabled = false;
		this.ButtonX_6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_6.ForeColor = System.Drawing.SystemColors.ControlText;
		this.ButtonX_6.Image = (System.Drawing.Image)resources.GetObject("ButtonX16.Image");
		DevComponents.DotNetBar.ButtonX buttonX_3 = this.ButtonX_6;
		location = new System.Drawing.Point(180, 378);
		buttonX_3.Location = location;
		this.ButtonX_6.Name = "ButtonX16";
		DevComponents.DotNetBar.ButtonX buttonX_4 = this.ButtonX_6;
		size = new System.Drawing.Size(162, 52);
		buttonX_4.Size = size;
		this.ButtonX_6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_6.SubItems.AddRange(new DevComponents.DotNetBar.BaseItem[1] { this.ButtonItem3 });
		this.ButtonX_6.TabIndex = 14;
		this.ButtonX_6.Text = "ต\u0e48อเวลา";
		this.ButtonX_6.Visible = false;
		this.ButtonItem3.GlobalItem = false;
		this.ButtonItem3.Name = "ButtonItem3";
		this.ButtonItem3.Text = "ต\u0e48อเวลา 30 นาท\u0e35 (ฟร\u0e35)";
		this.ButtonX_5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_5.FocusCuesEnabled = false;
		this.ButtonX_5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_5.Image = (System.Drawing.Image)resources.GetObject("ButtonX15.Image");
		DevComponents.DotNetBar.ButtonX buttonX_5 = this.ButtonX_5;
		location = new System.Drawing.Point(180, 498);
		buttonX_5.Location = location;
		this.ButtonX_5.Name = "ButtonX15";
		DevComponents.DotNetBar.ButtonX buttonX_6 = this.ButtonX_5;
		size = new System.Drawing.Size(162, 52);
		buttonX_6.Size = size;
		this.ButtonX_5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_5.TabIndex = 13;
		this.ButtonX_5.Text = "ป\u0e34ดไฟ";
		this.ButtonX_4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_4.FocusCuesEnabled = false;
		this.ButtonX_4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_4.Image = (System.Drawing.Image)resources.GetObject("ButtonX14.Image");
		DevComponents.DotNetBar.ButtonX buttonX_7 = this.ButtonX_4;
		location = new System.Drawing.Point(10, 498);
		buttonX_7.Location = location;
		this.ButtonX_4.Name = "ButtonX14";
		DevComponents.DotNetBar.ButtonX buttonX_8 = this.ButtonX_4;
		size = new System.Drawing.Size(162, 52);
		buttonX_8.Size = size;
		this.ButtonX_4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_4.TabIndex = 12;
		this.ButtonX_4.Text = "เป\u0e34ดไฟ";
		this.ButtonX_3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_3.FocusCuesEnabled = false;
		this.ButtonX_3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_3.Image = (System.Drawing.Image)resources.GetObject("ButtonX13.Image");
		DevComponents.DotNetBar.ButtonX buttonX_9 = this.ButtonX_3;
		location = new System.Drawing.Point(10, 255);
		buttonX_9.Location = location;
		this.ButtonX_3.Name = "ButtonX13";
		DevComponents.DotNetBar.ButtonX buttonX_10 = this.ButtonX_3;
		size = new System.Drawing.Size(162, 52);
		buttonX_10.Size = size;
		this.ButtonX_3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_3.TabIndex = 11;
		this.ButtonX_3.Text = "ใบแจ\u0e49งหน\u0e35\u0e49";
		this.ButtonX_2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_2.Enabled = false;
		this.ButtonX_2.FocusCuesEnabled = false;
		this.ButtonX_2.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_2.ForeColor = System.Drawing.SystemColors.ControlText;
		this.ButtonX_2.Image = (System.Drawing.Image)resources.GetObject("ButtonX12.Image");
		DevComponents.DotNetBar.ButtonX buttonX_11 = this.ButtonX_2;
		location = new System.Drawing.Point(180, 316);
		buttonX_11.Location = location;
		this.ButtonX_2.Name = "ButtonX12";
		DevComponents.DotNetBar.ButtonX buttonX_12 = this.ButtonX_2;
		size = new System.Drawing.Size(162, 52);
		buttonX_12.Size = size;
		this.ButtonX_2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_2.TabIndex = 10;
		this.ButtonX_2.Text = "พ\u0e34มพ\u0e4cค\u0e39ปองอาหาร";
		this.ButtonX_1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_1.FocusCuesEnabled = false;
		this.ButtonX_1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_1.ForeColor = System.Drawing.SystemColors.ControlText;
		this.ButtonX_1.Image = (System.Drawing.Image)resources.GetObject("ButtonX11.Image");
		DevComponents.DotNetBar.ButtonX buttonX_13 = this.ButtonX_1;
		location = new System.Drawing.Point(180, 12);
		buttonX_13.Location = location;
		this.ButtonX_1.Name = "ButtonX11";
		DevComponents.DotNetBar.ButtonX buttonX_14 = this.ButtonX_1;
		size = new System.Drawing.Size(162, 52);
		buttonX_14.Size = size;
		this.ButtonX_1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_1.TabIndex = 9;
		this.ButtonX_1.Text = "เพ\u0e34\u0e48ม/ลด\r\nว\u0e31นเข\u0e49าพ\u0e31ก";
		this.ButtonX_0.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX_0.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX_0.FocusCuesEnabled = false;
		this.ButtonX_0.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX_0.ForeColor = System.Drawing.SystemColors.ControlText;
		this.ButtonX_0.Image = (System.Drawing.Image)resources.GetObject("ButtonX10.Image");
		DevComponents.DotNetBar.ButtonX buttonX_15 = this.ButtonX_0;
		location = new System.Drawing.Point(180, 72);
		buttonX_15.Location = location;
		this.ButtonX_0.Name = "ButtonX10";
		DevComponents.DotNetBar.ButtonX buttonX_16 = this.ButtonX_0;
		size = new System.Drawing.Size(162, 52);
		buttonX_16.Size = size;
		this.ButtonX_0.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX_0.TabIndex = 8;
		this.ButtonX_0.Text = "เพ\u0e34\u0e48ม/แก\u0e49ไข\r\nห\u0e49องพ\u0e31ก";
		this.ButtonX9.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX9.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX9.FocusCuesEnabled = false;
		this.ButtonX9.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX9.Image = (System.Drawing.Image)resources.GetObject("ButtonX9.Image");
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX9;
		location = new System.Drawing.Point(180, 255);
		buttonX.Location = location;
		this.ButtonX9.Name = "ButtonX9";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX9;
		size = new System.Drawing.Size(162, 52);
		buttonX2.Size = size;
		this.ButtonX9.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX9.TabIndex = 7;
		this.ButtonX9.Text = "ใบกำก\u0e31บภาษ\u0e35";
		this.ButtonX8.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX8.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX8.FocusCuesEnabled = false;
		this.ButtonX8.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX8.ForeColor = System.Drawing.Color.FromArgb(192, 0, 0);
		this.ButtonX8.Image = (System.Drawing.Image)resources.GetObject("ButtonX8.Image");
		DevComponents.DotNetBar.ButtonX buttonX3 = this.ButtonX8;
		location = new System.Drawing.Point(180, 438);
		buttonX3.Location = location;
		this.ButtonX8.Name = "ButtonX8";
		DevComponents.DotNetBar.ButtonX buttonX4 = this.ButtonX8;
		size = new System.Drawing.Size(162, 52);
		buttonX4.Size = size;
		this.ButtonX8.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX8.TabIndex = 6;
		this.ButtonX8.Text = "ยกเล\u0e34กห\u0e49องพ\u0e31ก";
		this.ButtonX7.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX7.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX7.FocusCuesEnabled = false;
		this.ButtonX7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX7.Image = (System.Drawing.Image)resources.GetObject("ButtonX7.Image");
		DevComponents.DotNetBar.ButtonX buttonX5 = this.ButtonX7;
		location = new System.Drawing.Point(312, 456);
		buttonX5.Location = location;
		this.ButtonX7.Name = "ButtonX7";
		DevComponents.DotNetBar.ButtonX buttonX6 = this.ButtonX7;
		size = new System.Drawing.Size(176, 52);
		buttonX6.Size = size;
		this.ButtonX7.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX7.TabIndex = 5;
		this.ButtonX7.Text = "ต\u0e49องการพ\u0e31กต\u0e48อ";
		this.ButtonX7.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX7.Visible = false;
		this.ButtonX5.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX5.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX5.FocusCuesEnabled = false;
		this.ButtonX5.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX5.Image = (System.Drawing.Image)resources.GetObject("ButtonX5.Image");
		DevComponents.DotNetBar.ButtonX buttonX7 = this.ButtonX5;
		location = new System.Drawing.Point(10, 438);
		buttonX7.Location = location;
		this.ButtonX5.Name = "ButtonX5";
		DevComponents.DotNetBar.ButtonX buttonX8 = this.ButtonX5;
		size = new System.Drawing.Size(162, 52);
		buttonX8.Size = size;
		this.ButtonX5.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX5.TabIndex = 4;
		this.ButtonX5.Text = "ย\u0e31งไม\u0e48มาเข\u0e49าพ\u0e31ก";
		this.ButtonX5.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX4.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX4.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX4.FocusCuesEnabled = false;
		this.ButtonX4.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX4.Image = (System.Drawing.Image)resources.GetObject("ButtonX4.Image");
		DevComponents.DotNetBar.ButtonX buttonX9 = this.ButtonX4;
		location = new System.Drawing.Point(10, 316);
		buttonX9.Location = location;
		this.ButtonX4.Name = "ButtonX4";
		DevComponents.DotNetBar.ButtonX buttonX10 = this.ButtonX4;
		size = new System.Drawing.Size(162, 52);
		buttonX10.Size = size;
		this.ButtonX4.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX4.TabIndex = 3;
		this.ButtonX4.Text = "ฝากข\u0e49อความ";
		this.ButtonX3.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX3.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX3.FocusCuesEnabled = false;
		this.ButtonX3.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX3.Image = (System.Drawing.Image)resources.GetObject("ButtonX3.Image");
		DevComponents.DotNetBar.ButtonX buttonX11 = this.ButtonX3;
		location = new System.Drawing.Point(10, 132);
		buttonX11.Location = location;
		this.ButtonX3.Name = "ButtonX3";
		DevComponents.DotNetBar.ButtonX buttonX12 = this.ButtonX3;
		size = new System.Drawing.Size(162, 52);
		buttonX12.Size = size;
		this.ButtonX3.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX3.TabIndex = 2;
		this.ButtonX3.Text = "ค\u0e48าใช\u0e49จ\u0e48ายอ\u0e37\u0e48นๆ";
		this.ButtonX3.TextAlignment = DevComponents.DotNetBar.eButtonTextAlignment.Left;
		this.ButtonX2.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX2.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX2.FocusCuesEnabled = false;
		this.ButtonX2.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX2.Image = (System.Drawing.Image)resources.GetObject("ButtonX2.Image");
		this.ButtonX2.ImagePosition = DevComponents.DotNetBar.eImagePosition.Top;
		DevComponents.DotNetBar.ButtonX buttonX13 = this.ButtonX2;
		location = new System.Drawing.Point(10, 12);
		buttonX13.Location = location;
		this.ButtonX2.Name = "ButtonX2";
		DevComponents.DotNetBar.ButtonX buttonX14 = this.ButtonX2;
		size = new System.Drawing.Size(162, 112);
		buttonX14.Size = size;
		this.ButtonX2.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX2.TabIndex = 2;
		this.ButtonX2.Text = "Check-OUT";
		this.ButtonX6.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX6.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX6.FocusCuesEnabled = false;
		this.ButtonX6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX6.Image = (System.Drawing.Image)resources.GetObject("ButtonX6.Image");
		DevComponents.DotNetBar.ButtonX buttonX15 = this.ButtonX6;
		location = new System.Drawing.Point(180, 134);
		buttonX15.Location = location;
		this.ButtonX6.Name = "ButtonX6";
		DevComponents.DotNetBar.ButtonX buttonX16 = this.ButtonX6;
		size = new System.Drawing.Size(162, 52);
		buttonX16.Size = size;
		this.ButtonX6.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX6.TabIndex = 2;
		this.ButtonX6.Text = "ชำระเง\u0e34น";
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.ColorTable = DevComponents.DotNetBar.eButtonColor.OrangeWithBackground;
		this.ButtonX1.FocusCuesEnabled = false;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.ButtonX1.Image = (System.Drawing.Image)resources.GetObject("ButtonX1.Image");
		DevComponents.DotNetBar.ButtonX buttonX17 = this.ButtonX1;
		location = new System.Drawing.Point(96, 558);
		buttonX17.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX18 = this.ButtonX1;
		size = new System.Drawing.Size(162, 52);
		buttonX18.Size = size;
		this.ButtonX1.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ป\u0e34ด";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(12f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		this.BackColor = System.Drawing.Color.FromArgb(194, 217, 247);
		size = new System.Drawing.Size(354, 624);
		this.ClientSize = size;
		this.Controls.Add(this.ButtonBackMoney);
		this.Controls.Add(this.ButtonX_7);
		this.Controls.Add(this.ButtonX_6);
		this.Controls.Add(this.ButtonX_5);
		this.Controls.Add(this.ButtonX_4);
		this.Controls.Add(this.ButtonX_3);
		this.Controls.Add(this.ButtonX_2);
		this.Controls.Add(this.ButtonX_1);
		this.Controls.Add(this.ButtonX_0);
		this.Controls.Add(this.ButtonX9);
		this.Controls.Add(this.ButtonX8);
		this.Controls.Add(this.ButtonX7);
		this.Controls.Add(this.ButtonX5);
		this.Controls.Add(this.ButtonX4);
		this.Controls.Add(this.ButtonX3);
		this.Controls.Add(this.ButtonX2);
		this.Controls.Add(this.ButtonX6);
		this.Controls.Add(this.ButtonX1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedToolWindow;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 6, 5, 6);
		this.Margin = margin;
		this.Name = "ClickUSE";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ClickBook";
		this.ResumeLayout(false);
	}

	private void ClickBook_FormClosing(object sender, FormClosingEventArgs e)
	{
		MSSQL.CodeErr = false;
		RoomNo = "";
		ButtonX_6.Visible = false;
	}

	public void method_0(string string_0)
	{
		checked
		{
			int num = ButtonX_6.SubItems.Count - 1;
			int num2 = 1;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				ButtonX_6.SubItems.RemoveAt(1);
				num2++;
			}
			string_0 = ((Operators.CompareString(string_0, "0", TextCompare: false) != 0) ? "ช\u0e31\u0e48วคราว" : "รายว\u0e31น");
			DataSet dataSet = Module1.connect("select * from HT_ContinueTime where Con_Type='" + string_0 + "'");
			int num5 = dataSet.Tables[0].Rows.Count - 1;
			int num6 = 0;
			while (true)
			{
				int num7 = num6;
				int num4 = num5;
				if (num7 <= num4)
				{
					ButtonItem buttonItem = new ButtonItem();
					buttonItem.GlobalItem = false;
					buttonItem.Name = string_0;
					buttonItem.Text = Conversions.ToString(dataSet.Tables[0].Rows[num6]["Con_Name"]);
					buttonItem.Click += method_1;
					ButtonX_6.SubItems.Add(buttonItem);
					num6++;
					continue;
				}
				break;
			}
		}
	}

	public void checkMoney(string cin_no)
	{
		ButtonBackMoney.Text = "ค\u0e37นเง\u0e34นส\u0e48วนเก\u0e34น";
		ButtonBackMoney.Enabled = false;
		DataSet dataSet = Module1.connect("select total_price_balance from HT_CheckIn_H where cin_no ='" + cin_no + "'");
		if (Operators.ConditionalCompareObjectLess(dataSet.Tables[0].Rows[0]["total_price_balance"], 0, TextCompare: false))
		{
			ButtonX buttonBackMoney = ButtonBackMoney;
			Type typeFromHandle = typeof(Math);
			object[] array = new object[1];
			DataRow dataRow = dataSet.Tables[0].Rows[0];
			string columnName = "total_price_balance";
			array[0] = RuntimeHelpers.GetObjectValue(dataRow[columnName]);
			object[] array2 = array;
			bool[] array3 = new bool[1] { true };
			object obj = NewLateBinding.LateGet(null, typeFromHandle, "Abs", array2, null, null, array3);
			if (array3[0])
			{
				dataRow[columnName] = RuntimeHelpers.GetObjectValue(array2[0]);
			}
			buttonBackMoney.Text = "ค\u0e37นเง\u0e34นส\u0e48วนเก\u0e34น\r\n" + Strings.Format(RuntimeHelpers.GetObjectValue(obj), "#,##0.00") + " บาท";
			ButtonBackMoney.Enabled = true;
		}
	}

	private void ClickBook_Load(object sender, EventArgs e)
	{
		ButtonX8.Enabled = Module1.bool_0;
		MSSQL.CodeErr = true;
		ISOK = false;
		check_power();
		Text = "รายการห\u0e49อง " + RoomNo;
		checked
		{
			int num = RoomArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (num2 == 0)
				{
					Text = Conversions.ToString(Operators.ConcatenateObject("รายการห\u0e49อง ", RoomArr[num2]));
				}
				else
				{
					Text = Conversions.ToString(Operators.ConcatenateObject(Text, Operators.ConcatenateObject(", ", RoomArr[num2])));
				}
				ButtonBackMoney.Enabled = false;
				num2++;
			}
			if (RoomArr.Count == 0)
			{
				try
				{
					DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
					method_0(Conversions.ToString(dataSet.Tables[0].Rows[0]["cin_type"]));
					checkMoney(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
					dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select cupon_no from HT_Cupon where cupon_cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and cupon_print=0")));
					if (dataSet.Tables[0].Rows.Count == 0)
					{
						ButtonX_2.Enabled = false;
					}
					else
					{
						ButtonX_2.Enabled = true;
					}
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					MessageBox.Show("สถานะห\u0e49อง " + RoomNo + " ค\u0e49างในระบบ ระบบจะทำการเคล\u0e35ยร\u0e4cให\u0e49ใหม\u0e48", "Error", MessageBoxButtons.OK, MessageBoxIcon.Hand);
					Module1.connect("update HT_Rooms set room_use='no' where room_no='" + RoomNo + "'");
					Module1.connect("update HT_Room_Status set room_status='Check-Out' where room_status<>'Check-Out' and  room_no='" + RoomNo + "'");
					ISOK = true;
					Close();
					ProjectData.ClearProjectError();
				}
			}
			if (!Module1.MANUAL_POWER)
			{
				ButtonX_4.Enabled = false;
				ButtonX_5.Enabled = false;
			}
			if (Module1.KichenMode)
			{
			}
		}
	}

	private void method_1(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_ContinueTime where Con_Type='", NewLateBinding.LateGet(sender, null, "name", new object[0], null, null, null)), "' and Con_Name='"), NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null)), "'")));
		if (dataSet.Tables[0].Rows.Count == 0)
		{
			MessageBox.Show("Error!! โปรดลองใหม\u0e48ภายหล\u0e31ง", "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
		}
		else if (MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("ค\u0e38ณต\u0e49องการ ", NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null)), " หร\u0e37อไม\u0e48")), "ต\u0e48อเวลา", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) != DialogResult.No)
		{
			DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Ds where cin_room_no ='" + RoomNo + "' and Cin_Room_Status='เข\u0e49าพ\u0e31ก'");
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				DateTime dateTime = Conversions.ToDate(dataSet2.Tables[0].Rows[0]["Cin_Room_Out"]).AddMinutes(Conversions.ToDouble(dataSet.Tables[0].Rows[0]["Con_Minute"]));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_CheckIn_Ds set  Cin_Room_Out='" + Conversions.ToString(dateTime), "' where id="), dataSet2.Tables[0].Rows[0]["id"])));
				object left = "INSERT INTO [HT_CheckIn_Product]";
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, " [Cin_No]");
				left = Operators.ConcatenateObject(left, ",[Cin_Room_no]");
				left = Operators.ConcatenateObject(left, ",[Cin_Ds_date]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_id]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_name]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_Unit]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_num]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_price]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_priceTotal]");
				left = Operators.ConcatenateObject(left, ",[Cin_Pro_pay],[Cin_Pro_note])");
				left = Operators.ConcatenateObject(left, "VALUES");
				left = Operators.ConcatenateObject(left, "(");
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" '", dataSet2.Tables[0].Rows[0]["Cin_no"]), "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + RoomNo, "'"));
				left = Operators.ConcatenateObject(left, string.Concat(",'" + Conversions.ToString(DateTime.Now), "'"));
				left = Operators.ConcatenateObject(left, ",'-2'");
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null)), "'"));
				left = Operators.ConcatenateObject(left, ",'รายการ'");
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(1));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(",", dataSet.Tables[0].Rows[0]["Con_Price"]));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(",", dataSet.Tables[0].Rows[0]["Con_Price"]));
				left = Operators.ConcatenateObject(left, "," + Conversions.ToString(0));
				left = Operators.ConcatenateObject(left, ",''");
				left = Operators.ConcatenateObject(left, ")");
				Module1.connect(Conversions.ToString(left));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_H set Total_Price_Product=Total_Price_Product+", dataSet.Tables[0].Rows[0]["Con_Price"]), ",Total_Price_Net=Total_Price_Net+"), dataSet.Tables[0].Rows[0]["Con_Price"]), ",Total_Price_Balance=Total_Price_Balance+"), dataSet.Tables[0].Rows[0]["Con_Price"]), " where Cin_no='"), dataSet2.Tables[0].Rows[0]["Cin_no"]), "'")));
				ISOK = true;
				MessageBox.Show(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("ต\u0e48อเวลา ", NewLateBinding.LateGet(sender, null, "text", new object[0], null, null, null)), " เสร\u0e47จเร\u0e35ยบร\u0e49อย"), "\r\n"), "\r\n"), " ค\u0e34ดเง\u0e34นเพ\u0e34\u0e48ม "), dataSet.Tables[0].Rows[0]["Con_Price"]), " บาท")));
				ButtonX14_Click(null, null);
			}
		}
	}

	public void check_power()
	{
		if (!Module1.POWER_USED)
		{
			ButtonX_4.Enabled = false;
			ButtonX_5.Enabled = false;
		}
		else if (RoomArr.Count >= 1)
		{
			ButtonX_4.Text = "เป\u0e34ดไฟ ห\u0e49องท\u0e35\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
			ButtonX_4.Enabled = true;
			ButtonX_5.Text = "ป\u0e34ดไฟ ห\u0e49องท\u0e35\u0e48เล\u0e37อกท\u0e31\u0e49งหมด";
			ButtonX_5.Enabled = true;
		}
		else if (Operators.CompareString(Module1.Power_Status(RoomNo), "OFF", TextCompare: false) == 0)
		{
			ButtonX_4.Text = "เป\u0e34ดไฟ " + RoomNo;
			ButtonX_4.Enabled = true;
			ButtonX_5.Text = "ป\u0e34ดไฟ " + RoomNo;
			ButtonX_5.Enabled = false;
		}
		else
		{
			ButtonX_4.Text = "เป\u0e34ดไฟ " + RoomNo;
			ButtonX_4.Enabled = false;
			ButtonX_5.Text = "ป\u0e34ดไฟ " + RoomNo;
			ButtonX_5.Enabled = true;
		}
	}

	private void ButtonX3_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and cin_room_status='เข\u0e49าพ\u0e31ก'");
		FrmPayAddPro frmPayAddPro = new FrmPayAddPro();
		frmPayAddPro.TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		frmPayAddPro.ShowDialog();
		checkMoney(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		ISOK = true;
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		Close();
	}

	private void ButtonX6_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		FrmPayAdd frmPayAdd = new FrmPayAdd();
		frmPayAdd.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		frmPayAdd.ShowDialog();
		checkMoney(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		ISOK = true;
	}

	private void ButtonX4_Click(object sender, EventArgs e)
	{
		new Room_Note();
		MyProject.Forms.Room_Note.R_NO = RoomNo;
		MyProject.Forms.Room_Note.ShowDialog();
		ISOK = true;
	}

	private void ButtonX5_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Room_Status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก' where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and Cin_room_no='"), RoomNo), "'")));
			}
			else
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_Room_no='", RoomArr[num2]), "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Room_Status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก' where Cin_no='", dataSet2.Tables[0].Rows[0]["Cin_no"]), "' and Cin_room_no='"), RoomArr[num2]), "'")));
					num2++;
				}
			}
			ISOK = true;
			Close();
		}
	}

	private void ButtonX7_Click(object sender, EventArgs e)
	{
		object obj = Interaction.InputBox("กร\u0e38ณาใส\u0e48จำนวนค\u0e37น", "พ\u0e31กต\u0e48อ", "1");
		if (Operators.ConditionalCompareObjectEqual(obj, "", TextCompare: false))
		{
			return;
		}
		if (!Versioned.IsNumeric(RuntimeHelpers.GetObjectValue(obj)))
		{
			MessageBox.Show("กร\u0e38ณากรอกจำนวนค\u0e37นเป\u0e47นต\u0e31วเลข");
			return;
		}
		checked
		{
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
				decimal num = Conversions.ToDecimal(Operators.MultiplyObject(Conversions.ToDecimal(obj), dataSet.Tables[0].Rows[0]["Cin_Room_Price"]));
				object left = "update HT_CheckIn_Ds set ";
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" Cin_Room_Out='", NewLateBinding.LateGet(dataSet.Tables[0].Rows[0]["Cin_Room_Out"], null, "AddDays", new object[1] { Conversions.ToInteger(obj) }, null, null, null)), "'"));
				left = Operators.ConcatenateObject(left, ",Cin_Room_Night=Cin_Room_Night+" + Conversions.ToString(Conversions.ToDecimal(obj)));
				left = Operators.ConcatenateObject(left, ",Cin_Room_PriceToTal=Cin_Room_PriceToTal+" + Conversions.ToString(num));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and Cin_room_no='"), RoomNo), "'"));
				Module1.connect(Conversions.ToString(left));
				object left2 = "update HT_CheckIn_H set ";
				left2 = Operators.ConcatenateObject(left2, " Total_Price_Room=Total_Price_Room+" + Conversions.ToString(num));
				left2 = Operators.ConcatenateObject(left2, ",Total_Price_Net=Total_Price_Net+" + Conversions.ToString(num));
				left2 = Operators.ConcatenateObject(left2, ",Total_Price_Balance=Total_Price_Balance+" + Conversions.ToString(num));
				left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(" where Cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'"));
				Module1.connect(Conversions.ToString(left2));
				int num2 = Conversions.ToInteger(obj) - 1;
				int num3 = 0;
				while (true)
				{
					int num4 = num3;
					int num5 = num2;
					if (num4 <= num5)
					{
						DateTime dateTime = Conversions.ToDate(dataSet.Tables[0].Rows[0]["Cin_Room_Out"]);
						DataSet dataSet2 = Module1.connect("select * from HT_Room_Status where room_date='" + Conversions.ToString(dateTime.AddDays(num3).Date) + "' and room_no='" + RoomNo + "'");
						if (dataSet2.Tables[0].Rows.Count != 0)
						{
							object left3 = "update [HT_Room_Status] SET ";
							left3 = Operators.ConcatenateObject(left3, " [room_status]='เข\u0e49าพ\u0e31ก'");
							left3 = Operators.ConcatenateObject(left3, Operators.ConcatenateObject(Operators.ConcatenateObject(",[room_Details]='", dataSet.Tables[0].Rows[0]["Cin_cust_name"]), "'"));
							left3 = Operators.ConcatenateObject(left3, Operators.ConcatenateObject(Operators.ConcatenateObject(",[room_CheckIn_No]='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'"));
							left3 = Operators.ConcatenateObject(left3, string.Concat(string.Concat(string.Concat(" where room_date='" + Conversions.ToString(dateTime.AddDays(num3).Date), "' and room_no='"), RoomNo), "'"));
							Module1.connect(Conversions.ToString(left3));
						}
						else
						{
							object right = Module1.get_id("HT_Room_Status", "id");
							object left4 = "INSERT INTO [HT_Room_Status]";
							left4 = Operators.ConcatenateObject(left4, "([id]");
							left4 = Operators.ConcatenateObject(left4, ",[room_no]");
							left4 = Operators.ConcatenateObject(left4, ",[room_date]");
							left4 = Operators.ConcatenateObject(left4, ",[room_status]");
							left4 = Operators.ConcatenateObject(left4, ",[room_Details],[room_CheckIn_No],[room_date_oa])");
							left4 = Operators.ConcatenateObject(left4, "VALUES");
							left4 = Operators.ConcatenateObject(left4, "(");
							left4 = Operators.ConcatenateObject(left4, right);
							left4 = Operators.ConcatenateObject(left4, string.Concat(",'" + RoomNo, "'"));
							left4 = Operators.ConcatenateObject(left4, string.Concat(",'" + Conversions.ToString(dateTime.AddDays(num3).Date), "'"));
							left4 = Operators.ConcatenateObject(left4, ",'เข\u0e49าพ\u0e31ก'");
							left4 = Operators.ConcatenateObject(left4, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", dataSet.Tables[0].Rows[0]["Cin_cust_name"]), "'"));
							left4 = Operators.ConcatenateObject(left4, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", dataSet.Tables[0].Rows[0]["Cin_no"]), "'"));
							left4 = Operators.ConcatenateObject(left4, "," + Conversions.ToString(dateTime.AddDays(num3).Date.ToOADate()));
							left4 = Operators.ConcatenateObject(left4, ")");
							Module1.connect(Conversions.ToString(left4));
						}
						num3++;
						continue;
					}
					break;
				}
			}
			else
			{
				int num6 = RoomArr.Count - 1;
				int num7 = 0;
				while (true)
				{
					int num8 = num7;
					int num5 = num6;
					if (num8 > num5)
					{
						break;
					}
					DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_Room_no='", RoomArr[num7]), "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')")));
					decimal num9 = Conversions.ToDecimal(Operators.MultiplyObject(Conversions.ToDecimal(obj), dataSet3.Tables[0].Rows[0]["Cin_Room_Price"]));
					object left5 = "update HT_CheckIn_Ds set ";
					left5 = Operators.ConcatenateObject(left5, Operators.ConcatenateObject(Operators.ConcatenateObject(" Cin_Room_Out='", NewLateBinding.LateGet(dataSet3.Tables[0].Rows[0]["Cin_Room_Out"], null, "AddDays", new object[1] { Conversions.ToInteger(obj) }, null, null, null)), "'"));
					left5 = Operators.ConcatenateObject(left5, ",Cin_Room_Night=Cin_Room_Night+" + Conversions.ToString(Conversions.ToDecimal(obj)));
					left5 = Operators.ConcatenateObject(left5, ",Cin_Room_PriceToTal=Cin_Room_PriceToTal+" + Conversions.ToString(num9));
					left5 = Operators.ConcatenateObject(left5, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(" where Cin_no='", dataSet3.Tables[0].Rows[0]["Cin_no"]), "' and Cin_room_no='"), RoomArr[num7]), "'"));
					Module1.connect(Conversions.ToString(left5));
					object left6 = "update HT_CheckIn_H set ";
					left6 = Operators.ConcatenateObject(left6, " Total_Price_Room=Total_Price_Room+" + Conversions.ToString(num9));
					left6 = Operators.ConcatenateObject(left6, ",Total_Price_Net=Total_Price_Net+" + Conversions.ToString(num9));
					left6 = Operators.ConcatenateObject(left6, ",Total_Price_Balance=Total_Price_Balance+" + Conversions.ToString(num9));
					left6 = Operators.ConcatenateObject(left6, Operators.ConcatenateObject(Operators.ConcatenateObject(" where Cin_no='", dataSet3.Tables[0].Rows[0]["Cin_no"]), "'"));
					Module1.connect(Conversions.ToString(left6));
					int num10 = Conversions.ToInteger(obj) - 1;
					int num11 = 0;
					while (true)
					{
						int num12 = num11;
						num5 = num10;
						if (num12 > num5)
						{
							break;
						}
						DateTime dateTime2 = Conversions.ToDate(dataSet3.Tables[0].Rows[0]["Cin_Room_Out"]);
						DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from HT_Room_Status where room_date='" + Conversions.ToString(dateTime2.AddDays(num11).Date), "' and room_no='"), RoomArr[num7]), "'")));
						if (dataSet4.Tables[0].Rows.Count != 0)
						{
							object left7 = "update [HT_Room_Status] SET ";
							left7 = Operators.ConcatenateObject(left7, " [room_status]='เข\u0e49าพ\u0e31ก'");
							left7 = Operators.ConcatenateObject(left7, Operators.ConcatenateObject(Operators.ConcatenateObject(",[room_Details]='", dataSet3.Tables[0].Rows[0]["Cin_cust_name"]), "'"));
							left7 = Operators.ConcatenateObject(left7, Operators.ConcatenateObject(Operators.ConcatenateObject(",[room_CheckIn_No]='", dataSet3.Tables[0].Rows[0]["Cin_no"]), "'"));
							left7 = Operators.ConcatenateObject(left7, Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(" where room_date='" + Conversions.ToString(dateTime2.AddDays(num11).Date), "' and room_no='"), RoomArr[num7]), "'"));
							Module1.connect(Conversions.ToString(left7));
						}
						else
						{
							object right2 = Module1.get_id("HT_Room_Status", "id");
							object left8 = "INSERT INTO [HT_Room_Status]";
							left8 = Operators.ConcatenateObject(left8, "([id]");
							left8 = Operators.ConcatenateObject(left8, ",[room_no]");
							left8 = Operators.ConcatenateObject(left8, ",[room_date]");
							left8 = Operators.ConcatenateObject(left8, ",[room_status]");
							left8 = Operators.ConcatenateObject(left8, ",[room_Details],[room_CheckIn_No],[room_date_oa])");
							left8 = Operators.ConcatenateObject(left8, "VALUES");
							left8 = Operators.ConcatenateObject(left8, "(");
							left8 = Operators.ConcatenateObject(left8, right2);
							left8 = Operators.ConcatenateObject(left8, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", RoomArr[num7]), "'"));
							left8 = Operators.ConcatenateObject(left8, string.Concat(",'" + Conversions.ToString(dateTime2.AddDays(num11).Date), "'"));
							left8 = Operators.ConcatenateObject(left8, ",'เข\u0e49าพ\u0e31ก'");
							left8 = Operators.ConcatenateObject(left8, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", dataSet3.Tables[0].Rows[0]["Cin_cust_name"]), "'"));
							left8 = Operators.ConcatenateObject(left8, Operators.ConcatenateObject(Operators.ConcatenateObject(",'", dataSet3.Tables[0].Rows[0]["Cin_no"]), "'"));
							left8 = Operators.ConcatenateObject(left8, "," + Conversions.ToString(dateTime2.AddDays(num11).Date.ToOADate()));
							left8 = Operators.ConcatenateObject(left8, ")");
							Module1.connect(Conversions.ToString(left8));
						}
						num11++;
					}
					num7++;
				}
			}
			ISOK = true;
			Close();
		}
	}

	private void ButtonX2_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		FrmCheckOut frmCheckOut = new FrmCheckOut();
		frmCheckOut.TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		frmCheckOut.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		checked
		{
			if (RoomArr.Count == 0)
			{
				frmCheckOut.R_NO.Add(RoomNo);
			}
			else
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 > num4)
					{
						break;
					}
					frmCheckOut.R_NO.Add(RuntimeHelpers.GetObjectValue(RoomArr[num2]));
					num2++;
				}
			}
			frmCheckOut.Autoload = true;
			frmCheckOut.WindowState = FormWindowState.Maximized;
			frmCheckOut.ShowDialog();
			Close();
		}
	}

	private void ButtonX8_Click(object sender, EventArgs e)
	{
		string text = RoomNo;
		checked
		{
			int num = RoomArr.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				text = ((num2 != 0) ? Conversions.ToString(Operators.ConcatenateObject(text, Operators.ConcatenateObject(", ", RoomArr[num2]))) : Conversions.ToString(Operators.ConcatenateObject("รายการห\u0e49อง ", RoomArr[num2])));
				num2++;
			}
			if (MessageBox.Show("ค\u0e38ณต\u0e49องการยกเล\u0e34กห\u0e49องพ\u0e31ก " + text + " หร\u0e37อไม\u0e48", "ยกเล\u0e34กห\u0e49องพ\u0e31ก", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk, MessageBoxDefaultButton.Button2) != DialogResult.Yes)
			{
				return;
			}
			object obj = Interaction.InputBox("กร\u0e38ณากรอกหมายเหต\u0e38", "หมายเหต\u0e38");
			string sIR_PAY = Module1.GetSIR_PAY();
			if (RoomArr.Count == 0)
			{
				DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
				DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("select * from View_CheckIn_Ds where Cin_Room_No='" + RoomNo, "' and Cin_No='"), dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
				if (Operators.ConditionalCompareObjectGreater(dataSet2.Tables[0].Rows[0]["Cin_Room_Pay_Total"], 0, TextCompare: false))
				{
					MyProject.Forms.FormConfirmPay.PTOTAl = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Cin_Room_Pay_Total"]);
					MyProject.Forms.FormConfirmPay.ShowDialog();
					if (!MyProject.Forms.FormConfirmPay.ISOK)
					{
						return;
					}
				}
				decimal num5 = default(decimal);
				decimal num6 = default(decimal);
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("delete from HT_Room_Status where room_no='" + RoomNo, "' and room_CheckIn_No='"), dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat("delete from HT_CheckIn_Ds where Cin_Room_No='" + RoomNo, "' and Cin_No='"), dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
				Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='yes',Room_Use='no' where room_no='", dataSet.Tables[0].Rows[0]["cin_room_no"]), "'")));
				Module1.SaveCancel(RoomNo, Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), Conversions.ToString(obj));
				object left = "UPDATE [HT_CheckIn_H] SET ";
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(" [Total_Price_Room]=Total_Price_Room-", dataSet2.Tables[0].Rows[0]["Cin_Room_PriceToTal"]));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(",[Total_Price_Net]=[Total_Price_Net]-", dataSet2.Tables[0].Rows[0]["Cin_Room_PriceToTal"]));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(",[Total_Price_Pay]=[Total_Price_Pay]-", dataSet2.Tables[0].Rows[0]["Cin_Room_Pay_Total"]));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(",[Total_Price_Balance]=([Total_Price_Balance]-", dataSet2.Tables[0].Rows[0]["Cin_Room_PriceToTal"]), ")+"), dataSet2.Tables[0].Rows[0]["Cin_Room_Pay_Total"]));
				left = Operators.ConcatenateObject(left, Operators.ConcatenateObject(Operators.ConcatenateObject(" where [Cin_no]='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'"));
				Module1.connect(Conversions.ToString(left));
				num5 = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Cin_Room_Pay_Total"]);
				num6 = Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Cin_Room_dep"]);
				if (decimal.Compare(num5, 0m) > 0)
				{
					Module1.Insert_Pay(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), RoomNo, DateTime.Now, Conversions.ToDecimal("-" + Conversions.ToString(MyProject.Forms.FormConfirmPay.PCASH)), Conversions.ToDecimal("-" + Conversions.ToString(MyProject.Forms.FormConfirmPay.PCREDIT)), "ยกเล\u0e34กห\u0e49อง", Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Cin_Room_PriceToTal"]), "รายการ", sIR_PAY, Conversions.ToString(dataSet2.Tables[0].Rows[0]["Cin_cust_no"]), "P001", Conversions.ToDecimal("1"), Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Cin_Room_PriceToTal"]), Conversions.ToDecimal(dataSet2.Tables[0].Rows[0]["Cin_Room_Price"]), "", Conversions.ToDecimal("-" + Conversions.ToString(MyProject.Forms.FormConfirmPay.PFREE)), Conversions.ToDecimal("-" + Conversions.ToString(MyProject.Forms.FormConfirmPay.TRANN)), Conversions.ToDecimal("-" + Conversions.ToString(MyProject.Forms.FormConfirmPay.WEB)));
				}
				DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_Ds where  Cin_No='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
				if (dataSet3.Tables[0].Rows.Count == 0)
				{
					DataSet dataSet4 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_Product where  Cin_No='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
					if (dataSet4.Tables[0].Rows.Count == 0)
					{
						Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_H set cin_status='ยกเล\u0e34ก' where cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
					}
				}
				if (decimal.Compare(num5, 0m) > 0)
				{
					MessageBox.Show("ค\u0e37นค\u0e48าห\u0e49องจำนวน " + Strings.Format(num5, "#,##0.00") + " บาท\r\nค\u0e37นค\u0e48าม\u0e31ดจำจำนวน " + Conversions.ToString(num6) + " บาท\r\nรวม " + Conversions.ToString(decimal.Add(num5, num6)) + " บาท", "ค\u0e37นเง\u0e34น", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
					if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
					{
						Print_Report.Print_Sale(sIR_PAY, preview: false);
					}
				}
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟ เน\u0e37\u0e48องจากยกเล\u0e34กห\u0e49องพ\u0e31ก");
			}
			else
			{
				decimal num7 = default(decimal);
				decimal num8 = default(decimal);
				string no = "";
				string text2 = "";
				string cid = "";
				int num9 = RoomArr.Count - 1;
				int num10 = 0;
				while (true)
				{
					int num11 = num10;
					int num4 = num9;
					if (num11 > num4)
					{
						break;
					}
					DataSet dataSet5 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_Room_no='", RoomArr[num10]), "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')")));
					DataSet dataSet6 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from View_CheckIn_Ds where Cin_Room_No='", RoomArr[num10]), "' and Cin_No='"), dataSet5.Tables[0].Rows[0]["Cin_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_Room_Status where room_no='", RoomArr[num10]), "' and room_CheckIn_No='"), dataSet5.Tables[0].Rows[0]["Cin_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("delete from HT_CheckIn_Ds where Cin_Room_No='", RoomArr[num10]), "' and Cin_No='"), dataSet5.Tables[0].Rows[0]["Cin_no"]), "'")));
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_Rooms set Room_Clean='yes',Room_Use='no' where room_no='", dataSet5.Tables[0].Rows[0]["cin_room_no"]), "'")));
					Module1.SaveCancel(Conversions.ToString(RoomArr[num10]), Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cin_no"]), Conversions.ToString(obj));
					object left2 = "UPDATE [HT_CheckIn_H] SET ";
					left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(" [Total_Price_Room]=Total_Price_Room-", dataSet6.Tables[0].Rows[0]["Cin_Room_PriceToTal"]));
					left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(",[Total_Price_Net]=[Total_Price_Net]-", dataSet6.Tables[0].Rows[0]["Cin_Room_PriceToTal"]));
					left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(",[Total_Price_Pay]=[Total_Price_Pay]-", dataSet6.Tables[0].Rows[0]["Cin_Room_Pay_Total"]));
					left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(",[Total_Price_Balance]=([Total_Price_Balance]-", dataSet6.Tables[0].Rows[0]["Cin_Room_PriceToTal"]), ")+"), dataSet6.Tables[0].Rows[0]["Cin_Room_Pay_Total"]));
					left2 = Operators.ConcatenateObject(left2, Operators.ConcatenateObject(Operators.ConcatenateObject(" where [Cin_no]='", dataSet5.Tables[0].Rows[0]["Cin_no"]), "'"));
					Module1.connect(Conversions.ToString(left2));
					num7 = Conversions.ToDecimal(Operators.AddObject(num7, dataSet6.Tables[0].Rows[0]["Cin_Room_Pay_Total"]));
					num8 = Conversions.ToDecimal(Operators.AddObject(num8, dataSet6.Tables[0].Rows[0]["Cin_Room_dep"]));
					no = Conversions.ToString(dataSet5.Tables[0].Rows[0]["Cin_no"]);
					text2 = Conversions.ToString(Operators.ConcatenateObject(text2, Operators.ConcatenateObject(RoomArr[num10], " ")));
					cid = Conversions.ToString(dataSet6.Tables[0].Rows[0]["Cin_cust_no"]);
					DataSet dataSet7 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_Ds where  Cin_No='", dataSet5.Tables[0].Rows[0]["Cin_no"]), "'")));
					if (dataSet7.Tables[0].Rows.Count == 0)
					{
						DataSet dataSet8 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_Product where  Cin_No='", dataSet5.Tables[0].Rows[0]["Cin_no"]), "'")));
						if (dataSet8.Tables[0].Rows.Count == 0)
						{
							Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_H set cin_status='ยกเล\u0e34ก' where cin_no='", dataSet5.Tables[0].Rows[0]["Cin_no"]), "'")));
						}
					}
					Module1.Power_set(Conversions.ToString(RoomArr[num10]), "OFF", "", "ป\u0e34ดไฟ เน\u0e37\u0e48องจากยกเล\u0e34กห\u0e49องพ\u0e31ก");
					num10++;
				}
				if (decimal.Compare(num7, 0m) > 0)
				{
					Module1.Insert_Pay(no, text2, DateTime.Now, decimal.Negate(num7), 0m, "ยกเล\u0e34กห\u0e49อง (" + text2 + ")", num7, "รายการ", sIR_PAY, cid, "P001", num7, num7, num7, "", 0m, 0m, 0m);
					MessageBox.Show("ค\u0e37นค\u0e48าห\u0e49องจำนวน " + Strings.Format(num7, "#,##0.00") + " บาท\r\nค\u0e37นค\u0e48าม\u0e31ดจำจำนวน " + Conversions.ToString(num8) + " บาท\r\nรวม " + Conversions.ToString(decimal.Add(num7, num8)) + " บาท", "ค\u0e37นเง\u0e34น", MessageBoxButtons.OK, MessageBoxIcon.Asterisk);
					if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
					{
						Print_Report.Print_Sale(sIR_PAY, preview: false);
					}
				}
			}
			ISOK = true;
			Close();
		}
	}

	private void ButtonX9_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก') order BY id DESC");
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_Receipt_H where status_name<>'ยกเล\u0e34ก' and  Receipt_Ref='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' order by id desc")));
		if (dataSet2.Tables[0].Rows.Count == 0)
		{
			MyProject.Forms.FrmAddSale.IEdit = (string)(object)0;
			MyProject.Forms.FrmAddSale.clear();
			MyProject.Forms.FrmAddSale.Tref.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
			MyProject.Forms.FrmAddSale.B2_Click(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
			if (Operators.ConditionalCompareObjectEqual(dataSet.Tables[0].Rows[0]["Cin_type"], 2, TextCompare: false))
			{
				MyProject.Forms.FrmAddSale.Tnote.Text = "เข\u0e49าพ\u0e31ก ว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yy") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), Conversions.ToString(DateTime.DaysInMonth(Conversions.ToInteger(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), "yyyy")), Conversions.ToInteger(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), "MM")))) + "/MM/yy");
			}
			else
			{
				MyProject.Forms.FrmAddSale.Tnote.Text = "เข\u0e49าพ\u0e31ก ว\u0e31นท\u0e35\u0e48 " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_in"]), "dd/MM/yy") + " ถ\u0e36ง " + Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[0]["Cin_room_out"]), "dd/MM/yy");
			}
			MyProject.Forms.FrmAddSale.ShowDialog();
		}
		else
		{
			FormShowVAT formShowVAT = new FormShowVAT();
			formShowVAT.Label_NO.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
			formShowVAT.ShowDialog();
		}
		ISOK = true;
	}

	private void ButtonX10_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		MyProject.Forms.frmMain1.ButtonItem10_Click(null, null, "", Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		checkMoney(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		ISOK = true;
	}

	private void ButtonX11_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		MyProject.Forms.FrmEditDate.open1 = "";
		MyProject.Forms.FrmEditDate.Show();
		MyProject.Forms.FrmEditDate.TdocNum.Text = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		MyProject.Forms.FrmEditDate.EDIT_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		MyProject.Forms.FrmEditDate.R_NO = RoomNo;
		MyProject.Forms.FrmEditDate.LoadBill();
		MyProject.Forms.FrmEditDate.Hide();
		MyProject.Forms.FrmEditDate.open1 = "ไม\u0e48เคล\u0e35ยร\u0e4c";
		MyProject.Forms.FrmEditDate.ShowDialog();
		ISOK = true;
		Close();
	}

	private void ButtonX12_Click(object sender, EventArgs e)
	{
		if (MessageBox.Show("ค\u0e39ปองท\u0e35\u0e48พ\u0e34มพ\u0e4cไปแล\u0e49วจะไม\u0e48สามารถพ\u0e34มพ\u0e4cได\u0e49อ\u0e35ก ค\u0e38ณต\u0e49องการพ\u0e34มพ\u0e4cหร\u0e37อไม\u0e48", "พ\u0e34มพ\u0e4c", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.Yes)
		{
			DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and cin_room_status='เข\u0e49าพ\u0e31ก'");
			dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select cupon_no from HT_Cupon where cupon_cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and cupon_cin_room in () and cupon_print=0")));
			ButtonX_2.Enabled = false;
		}
	}

	private void ButtonX13_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก') order by cin_room_night");
		MyProject.Forms.INV_Note.R_NO = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
		MyProject.Forms.INV_Note.ShowDialog();
		if (MyProject.Forms.INV_Note.isok)
		{
			Print_Report.Print_Reg3(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), preview: true);
		}
	}

	private void ButtonX14_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count != 0)
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องเข\u0e49าพ\u0e31ก");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "ON", "", "เป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องเข\u0e49าพ\u0e31ก");
			}
			check_power();
			ISOK = true;
		}
	}

	private void ButtonX15_Click(object sender, EventArgs e)
	{
		checked
		{
			if (RoomArr.Count != 0)
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						Module1.Power_set(Conversions.ToString(RoomArr[num2]), "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องเข\u0e49าพ\u0e31ก");
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				Module1.Power_set(RoomNo, "OFF", "", "ป\u0e34ดไฟโดยพน\u0e31กงาน จากหน\u0e49าห\u0e49องเข\u0e49าพ\u0e31ก");
			}
			check_power();
			ISOK = true;
		}
	}

	private void ButtonX16_Click(object sender, EventArgs e)
	{
		ButtonX_6.Expanded = true;
	}

	private void ButtonX17_Click(object sender, EventArgs e)
	{
		ButtonX_7.Expanded = true;
	}

	private void ButtonItem1_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			Print_Report.PrintFolio1(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
		}
		else
		{
			MessageBox.Show("ไม\u0e48พบเลขห\u0e49องในรายการเช\u0e47คอ\u0e34น");
		}
	}

	private void ButtonItem2_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		if (dataSet.Tables[0].Rows.Count != 0)
		{
			FormFolio formFolio = new FormFolio();
			formFolio.CIN_ID = Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]);
			formFolio.ShowDialog();
		}
		else
		{
			MessageBox.Show("ไม\u0e48พบเลขห\u0e49องในรายการเช\u0e47คอ\u0e34น");
		}
	}

	private void ButtonItem3_Click(object sender, EventArgs e)
	{
		if (MessageBox.Show("ค\u0e38ณต\u0e49องการเพ\u0e34\u0e48มเวลาเป\u0e34ดไฟหร\u0e37อไม\u0e48", "เพ\u0e34\u0e48มเวลาเป\u0e34ดไฟ", MessageBoxButtons.YesNo, MessageBoxIcon.Asterisk) == DialogResult.No)
		{
			return;
		}
		checked
		{
			if (RoomArr.Count != 0)
			{
				int num = RoomArr.Count - 1;
				int num2 = 0;
				while (true)
				{
					int num3 = num2;
					int num4 = num;
					if (num3 <= num4)
					{
						DataSet dataSet = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select * from HT_CheckIn_Ds where cin_room_no ='", RoomArr[num2]), "' and Cin_Room_Status='เข\u0e49าพ\u0e31ก'")));
						if (dataSet.Tables[0].Rows.Count != 0)
						{
							DateTime dateTime = Conversions.ToDate(dataSet.Tables[0].Rows[0]["Cin_Room_Out"]).AddMinutes(30.0);
							Module1.connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_CheckIn_Ds set  Cin_Room_Out='" + Conversions.ToString(dateTime), "' where id="), dataSet.Tables[0].Rows[0]["id"])));
						}
						num2++;
						continue;
					}
					break;
				}
			}
			else
			{
				DataSet dataSet2 = Module1.connect("select * from HT_CheckIn_Ds where cin_room_no ='" + RoomNo + "' and Cin_Room_Status='เข\u0e49าพ\u0e31ก'");
				if (dataSet2.Tables[0].Rows.Count != 0)
				{
					DateTime dateTime2 = Conversions.ToDate(dataSet2.Tables[0].Rows[0]["Cin_Room_Out"]).AddMinutes(30.0);
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(string.Concat("update HT_CheckIn_Ds set  Cin_Room_Out='" + Conversions.ToString(dateTime2), "' where id="), dataSet2.Tables[0].Rows[0]["id"])));
				}
			}
			ButtonX14_Click(null, null);
			MessageBox.Show("เพ\u0e34\u0e48มเวลาเร\u0e35ยบร\u0e49อย");
			ISOK = true;
		}
	}

	private void ButtonBackMoney_Click(object sender, EventArgs e)
	{
		DataSet dataSet = Module1.connect("select * from View_CheckIn_Ds where Cin_Room_no='" + RoomNo + "' and (cin_room_status='เข\u0e49าพ\u0e31ก' or cin_room_status='ย\u0e31งไม\u0e48เข\u0e49าพ\u0e31ก')");
		decimal num = default(decimal);
		DataSet dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select (Cin_Room_PriceToTal-Cin_Room_Pay_Total) as debt from HT_CheckIn_Ds where cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and (Cin_Room_PriceToTal-Cin_Room_Pay_Total)<0")));
		checked
		{
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				int num2 = dataSet2.Tables[0].Rows.Count - 1;
				int num3 = 0;
				while (true)
				{
					int num4 = num3;
					int num5 = num2;
					if (num4 > num5)
					{
						break;
					}
					object left = num;
					Type typeFromHandle = typeof(Math);
					object[] array = new object[1];
					object[] array2 = array;
					DataRow dataRow = dataSet2.Tables[0].Rows[num3];
					DataRow dataRow2 = dataRow;
					string columnName = "debt";
					array2[0] = RuntimeHelpers.GetObjectValue(dataRow2[columnName]);
					object[] array3 = array;
					object[] arguments = array3;
					bool[] array4 = new bool[1] { true };
					object right = NewLateBinding.LateGet(null, typeFromHandle, "Abs", arguments, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array3[0]);
					}
					num = Conversions.ToDecimal(Operators.AddObject(left, right));
					num3++;
				}
			}
			dataSet2 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select (Cin_Pro_priceTotal-Cin_Pro_pay) as debt from HT_CheckIn_Product where cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and (Cin_Pro_priceTotal-Cin_Pro_pay)<0")));
			if (dataSet2.Tables[0].Rows.Count != 0)
			{
				int num6 = dataSet2.Tables[0].Rows.Count - 1;
				int num7 = 0;
				while (true)
				{
					int num8 = num7;
					int num5 = num6;
					if (num8 > num5)
					{
						break;
					}
					object left2 = num;
					Type typeFromHandle2 = typeof(Math);
					object[] array3 = new object[1];
					object[] array5 = array3;
					DataRow dataRow = dataSet2.Tables[0].Rows[num7];
					DataRow dataRow3 = dataRow;
					string columnName = "debt";
					array5[0] = RuntimeHelpers.GetObjectValue(dataRow3[columnName]);
					object[] array = array3;
					object[] arguments2 = array;
					bool[] array4 = new bool[1] { true };
					object right2 = NewLateBinding.LateGet(null, typeFromHandle2, "Abs", arguments2, null, null, array4);
					if (array4[0])
					{
						dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
					}
					num = Conversions.ToDecimal(Operators.AddObject(left2, right2));
					num7++;
				}
			}
			if (decimal.Compare(num, 0m) == 0)
			{
				MessageBox.Show("ไม\u0e48พบรายการเง\u0e34นเก\u0e34น");
				checkMoney(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
				return;
			}
			if (decimal.Compare(num, 0m) > 0)
			{
				MyProject.Forms.FormConfirmPay.PTOTAl = decimal.Multiply(num, -1m);
				MyProject.Forms.FormConfirmPay.ShowDialog();
				if (MyProject.Forms.FormConfirmPay.ISOK)
				{
					object sIR_PAY = Module1.GetSIR_PAY();
					Module1.Insert_Pay(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]), "ค\u0e37นเง\u0e34นส\u0e48วนเก\u0e34น", DateTime.Now, MyProject.Forms.FormConfirmPay.PCASH, MyProject.Forms.FormConfirmPay.PCREDIT, "ค\u0e37นเง\u0e34นส\u0e48วนเก\u0e34น", decimal.Multiply(num, -1m), "รายการ", Conversions.ToString(sIR_PAY), Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_cust_no"]), "P001", 1m, decimal.Multiply(num, -1m), decimal.Multiply(num, -1m), "", MyProject.Forms.FormConfirmPay.PFREE, MyProject.Forms.FormConfirmPay.TRANN, MyProject.Forms.FormConfirmPay.WEB);
					decimal num9 = default(decimal);
					DataSet dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select id,(Cin_Room_PriceToTal-Cin_Room_Pay_Total) as debt from HT_CheckIn_Ds where cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and (Cin_Room_PriceToTal-Cin_Room_Pay_Total)<0")));
					if (dataSet3.Tables[0].Rows.Count != 0)
					{
						int num10 = dataSet3.Tables[0].Rows.Count - 1;
						int num11 = 0;
						while (true)
						{
							int num12 = num11;
							int num5 = num10;
							if (num12 > num5)
							{
								break;
							}
							Type typeFromHandle3 = typeof(Math);
							object[] array3 = new object[1];
							object[] array6 = array3;
							DataRow dataRow = dataSet3.Tables[0].Rows[num11];
							DataRow dataRow4 = dataRow;
							string columnName = "debt";
							array6[0] = RuntimeHelpers.GetObjectValue(dataRow4[columnName]);
							object[] array = array3;
							object[] arguments3 = array;
							bool[] array4 = new bool[1] { true };
							object right3 = NewLateBinding.LateGet(null, typeFromHandle3, "Abs", arguments3, null, null, array4);
							if (array4[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
							}
							Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Ds set Cin_Room_Pay_Total=Cin_Room_Pay_Total-", right3), "where id="), dataSet3.Tables[0].Rows[num11]["id"])));
							object left3 = num9;
							Type typeFromHandle4 = typeof(Math);
							array3 = new object[1];
							object[] array7 = array3;
							dataRow = dataSet3.Tables[0].Rows[num11];
							DataRow dataRow5 = dataRow;
							columnName = "debt";
							array7[0] = RuntimeHelpers.GetObjectValue(dataRow5[columnName]);
							array = array3;
							object[] arguments4 = array;
							array4 = new bool[1] { true };
							object right4 = NewLateBinding.LateGet(null, typeFromHandle4, "Abs", arguments4, null, null, array4);
							if (array4[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
							}
							num9 = Conversions.ToDecimal(Operators.AddObject(left3, right4));
							num11++;
						}
					}
					dataSet3 = Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject("select id, (Cin_Pro_priceTotal-Cin_Pro_pay) as debt from HT_CheckIn_Product where cin_no='", dataSet.Tables[0].Rows[0]["Cin_no"]), "' and (Cin_Pro_priceTotal-Cin_Pro_pay)<0")));
					if (dataSet3.Tables[0].Rows.Count != 0)
					{
						int num13 = dataSet3.Tables[0].Rows.Count - 1;
						int num14 = 0;
						while (true)
						{
							int num15 = num14;
							int num5 = num13;
							if (num15 > num5)
							{
								break;
							}
							Type typeFromHandle5 = typeof(Math);
							object[] array3 = new object[1];
							object[] array8 = array3;
							DataRow dataRow = dataSet3.Tables[0].Rows[num14];
							DataRow dataRow6 = dataRow;
							string columnName = "debt";
							array8[0] = RuntimeHelpers.GetObjectValue(dataRow6[columnName]);
							object[] array = array3;
							object[] arguments5 = array;
							bool[] array4 = new bool[1] { true };
							object right5 = NewLateBinding.LateGet(null, typeFromHandle5, "Abs", arguments5, null, null, array4);
							if (array4[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
							}
							Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject("update HT_CheckIn_Product set Cin_Pro_pay=Cin_Pro_pay-", right5), "where id="), dataSet3.Tables[0].Rows[num14]["id"])));
							object left4 = num9;
							Type typeFromHandle6 = typeof(Math);
							array3 = new object[1];
							object[] array9 = array3;
							dataRow = dataSet3.Tables[0].Rows[num14];
							DataRow dataRow7 = dataRow;
							columnName = "debt";
							array9[0] = RuntimeHelpers.GetObjectValue(dataRow7[columnName]);
							array = array3;
							object[] arguments6 = array;
							array4 = new bool[1] { true };
							object right6 = NewLateBinding.LateGet(null, typeFromHandle6, "Abs", arguments6, null, null, array4);
							if (array4[0])
							{
								dataRow[columnName] = RuntimeHelpers.GetObjectValue(array[0]);
							}
							num9 = Conversions.ToDecimal(Operators.AddObject(left4, right6));
							num14++;
						}
					}
					Module1.connect(Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(string.Concat(string.Concat(string.Concat("update HT_CheckIn_H set Total_Price_Pay=Total_Price_Pay-" + Conversions.ToString(num9), ",Total_Price_Balance=Total_Price_Balance+"), Conversions.ToString(num9)), " where Cin_no='"), dataSet.Tables[0].Rows[0]["Cin_no"]), "'")));
					MessageBox.Show("ค\u0e37นเง\u0e34นเสร\u0e47จเร\u0e35ยบร\u0e49อย");
					checkMoney(Conversions.ToString(dataSet.Tables[0].Rows[0]["Cin_no"]));
					if (Operators.CompareString(Module1.Receipt_print, "เป\u0e34ด", TextCompare: false) == 0)
					{
						Print_Report.Print_Sale(Conversions.ToString(sIR_PAY), preview: false);
					}
				}
			}
			ISOK = true;
		}
	}
}
