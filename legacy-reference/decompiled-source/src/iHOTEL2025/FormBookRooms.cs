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
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;
using iHOTEL2025.My.Resources;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormBookRooms : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("FlowLayoutPanel1")]
	private FlowLayoutPanel _FlowLayoutPanel1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Button2")]
	private Button _Button2;

	[AccessedThroughProperty("Timer1")]
	private Timer _Timer1;

	[AccessedThroughProperty("Button3")]
	private Button _Button3;

	[AccessedThroughProperty("Lnum")]
	private Label _Lnum;

	[AccessedThroughProperty("Label6")]
	private Label _Label6;

	[AccessedThroughProperty("Label7")]
	private Label _Label7;

	[AccessedThroughProperty("Button4")]
	private Button _Button4;

	[AccessedThroughProperty("SuperTooltip1")]
	private SuperTooltip _SuperTooltip1;

	[AccessedThroughProperty("ProgressBar1")]
	private ProgressBarX _ProgressBar1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	public DateTime start_date;

	public ArrayList Room_Arr;

	public ArrayList Room_Old;

	internal virtual FlowLayoutPanel FlowLayoutPanel1
	{
		[DebuggerNonUserCode]
		get
		{
			return _FlowLayoutPanel1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			PaintEventHandler value2 = FlowLayoutPanel1_Paint;
			if (_FlowLayoutPanel1 != null)
			{
				_FlowLayoutPanel1.Paint -= value2;
			}
			_FlowLayoutPanel1 = value;
			if (_FlowLayoutPanel1 != null)
			{
				_FlowLayoutPanel1.Paint += value2;
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
			_Button1 = value;
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
			_Button2 = value;
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

	internal virtual Button Button3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button3_Click;
			if (_Button3 != null)
			{
				_Button3.Click -= value2;
			}
			_Button3 = value;
			if (_Button3 != null)
			{
				_Button3.Click += value2;
			}
		}
	}

	internal virtual Label Lnum
	{
		[DebuggerNonUserCode]
		get
		{
			return _Lnum;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Lnum_Click;
			if (_Lnum != null)
			{
				_Lnum.Click -= value2;
			}
			_Lnum = value;
			if (_Lnum != null)
			{
				_Lnum.Click += value2;
			}
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
			EventHandler value2 = Label6_Click;
			if (_Label6 != null)
			{
				_Label6.Click -= value2;
			}
			_Label6 = value;
			if (_Label6 != null)
			{
				_Label6.Click += value2;
			}
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
			EventHandler value2 = Label7_Click;
			if (_Label7 != null)
			{
				_Label7.Click -= value2;
			}
			_Label7 = value;
			if (_Label7 != null)
			{
				_Label7.Click += value2;
			}
		}
	}

	internal virtual Button Button4
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button4;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button4_Click;
			if (_Button4 != null)
			{
				_Button4.Click -= value2;
			}
			_Button4 = value;
			if (_Button4 != null)
			{
				_Button4.Click += value2;
			}
		}
	}

	internal virtual SuperTooltip SuperTooltip1
	{
		[DebuggerNonUserCode]
		get
		{
			return _SuperTooltip1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_SuperTooltip1 = value;
		}
	}

	internal virtual ProgressBarX ProgressBar1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ProgressBar1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ProgressBar1 = value;
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
	static FormBookRooms()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public FormBookRooms()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormBookRooms_Load;
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		Room_Arr = new ArrayList();
		Room_Old = new ArrayList();
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
		System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(iHOTEL2025.FormBookRooms));
		this.FlowLayoutPanel1 = new System.Windows.Forms.FlowLayoutPanel();
		this.Label1 = new System.Windows.Forms.Label();
		this.Button1 = new System.Windows.Forms.Button();
		this.Button2 = new System.Windows.Forms.Button();
		this.Timer1 = new System.Windows.Forms.Timer(this.components);
		this.Button3 = new System.Windows.Forms.Button();
		this.Lnum = new System.Windows.Forms.Label();
		this.Label6 = new System.Windows.Forms.Label();
		this.Label7 = new System.Windows.Forms.Label();
		this.Button4 = new System.Windows.Forms.Button();
		this.SuperTooltip1 = new DevComponents.DotNetBar.SuperTooltip();
		this.ProgressBar1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.Label2 = new System.Windows.Forms.Label();
		this.FlowLayoutPanel1.SuspendLayout();
		this.SuspendLayout();
		this.FlowLayoutPanel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.FlowLayoutPanel1.AutoScroll = true;
		this.FlowLayoutPanel1.Controls.Add(this.Label1);
		this.FlowLayoutPanel1.Controls.Add(this.Button1);
		this.FlowLayoutPanel1.Controls.Add(this.Button2);
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel = this.FlowLayoutPanel1;
		System.Drawing.Point location = new System.Drawing.Point(12, 15);
		flowLayoutPanel.Location = location;
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel2 = this.FlowLayoutPanel1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		flowLayoutPanel2.Margin = margin;
		this.FlowLayoutPanel1.Name = "FlowLayoutPanel1";
		System.Windows.Forms.FlowLayoutPanel flowLayoutPanel3 = this.FlowLayoutPanel1;
		System.Drawing.Size size = new System.Drawing.Size(1132, 596);
		flowLayoutPanel3.Size = size;
		this.FlowLayoutPanel1.TabIndex = 0;
		this.Label1.BackColor = System.Drawing.Color.PaleTurquoise;
		this.Label1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(0, 0);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		margin = new System.Windows.Forms.Padding(0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		size = new System.Drawing.Size(84, 26);
		label3.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "หมายเลขห\u0e49อง";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Button1.BackColor = System.Drawing.Color.FromArgb(192, 255, 192);
		this.Button1.Cursor = System.Windows.Forms.Cursors.Hand;
		this.Button1.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
		this.Button1.Font = new System.Drawing.Font("Tahoma", 8.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(84, 0);
		button.Location = location;
		System.Windows.Forms.Button button2 = this.Button1;
		margin = new System.Windows.Forms.Padding(0);
		button2.Margin = margin;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button3 = this.Button1;
		size = new System.Drawing.Size(45, 26);
		button3.Size = size;
		this.Button1.TabIndex = 1;
		this.Button1.Text = "13/10";
		this.Button1.UseVisualStyleBackColor = false;
		this.Button2.BackColor = System.Drawing.Color.Tomato;
		this.Button2.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
		System.Windows.Forms.Button button4 = this.Button2;
		location = new System.Drawing.Point(129, 0);
		button4.Location = location;
		System.Windows.Forms.Button button5 = this.Button2;
		margin = new System.Windows.Forms.Padding(0);
		button5.Margin = margin;
		this.Button2.Name = "Button2";
		System.Windows.Forms.Button button6 = this.Button2;
		size = new System.Drawing.Size(54, 26);
		button6.Size = size;
		this.Button2.TabIndex = 2;
		this.Button2.Text = "13/10";
		this.Button2.UseVisualStyleBackColor = false;
		this.Button3.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button3.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Button3.Image = (System.Drawing.Image)resources.GetObject("Button3.Image");
		this.Button3.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button7 = this.Button3;
		location = new System.Drawing.Point(914, 616);
		button7.Location = location;
		this.Button3.Name = "Button3";
		System.Windows.Forms.Button button8 = this.Button3;
		size = new System.Drawing.Size(114, 40);
		button8.Size = size;
		this.Button3.TabIndex = 1;
		this.Button3.Text = "   ตกลง";
		this.Button3.UseVisualStyleBackColor = true;
		this.Lnum.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Lnum.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Lnum.ForeColor = System.Drawing.Color.Blue;
		System.Windows.Forms.Label lnum = this.Lnum;
		location = new System.Drawing.Point(149, 616);
		lnum.Location = location;
		this.Lnum.Name = "Lnum";
		System.Windows.Forms.Label lnum2 = this.Lnum;
		size = new System.Drawing.Size(62, 35);
		lnum2.Size = size;
		this.Lnum.TabIndex = 13;
		this.Lnum.Text = "0";
		this.Lnum.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		this.Label6.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label6.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label6.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label4 = this.Label6;
		location = new System.Drawing.Point(16, 619);
		label4.Location = location;
		this.Label6.Name = "Label6";
		System.Windows.Forms.Label label5 = this.Label6;
		size = new System.Drawing.Size(136, 35);
		label5.Size = size;
		this.Label6.TabIndex = 12;
		this.Label6.Text = "จำนวนค\u0e37นท\u0e35\u0e48เล\u0e37อก";
		this.Label6.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Label7.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label7.Font = new System.Drawing.Font("Tahoma", 12f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		this.Label7.ForeColor = System.Drawing.Color.Red;
		System.Windows.Forms.Label label6 = this.Label7;
		location = new System.Drawing.Point(212, 618);
		label6.Location = location;
		this.Label7.Name = "Label7";
		System.Windows.Forms.Label label7 = this.Label7;
		size = new System.Drawing.Size(47, 35);
		label7.Size = size;
		this.Label7.TabIndex = 11;
		this.Label7.Text = "ค\u0e37น";
		this.Label7.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		this.Button4.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right;
		this.Button4.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Button4.Image = (System.Drawing.Image)resources.GetObject("Button4.Image");
		this.Button4.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Windows.Forms.Button button9 = this.Button4;
		location = new System.Drawing.Point(1034, 616);
		button9.Location = location;
		this.Button4.Name = "Button4";
		System.Windows.Forms.Button button10 = this.Button4;
		size = new System.Drawing.Size(106, 40);
		button10.Size = size;
		this.Button4.TabIndex = 14;
		this.Button4.Text = "   ป\u0e34ด";
		this.Button4.UseVisualStyleBackColor = true;
		this.SuperTooltip1.AntiAlias = false;
		this.SuperTooltip1.DefaultFont = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.SuperTooltip superTooltip = this.SuperTooltip1;
		size = new System.Drawing.Size(200, 24);
		superTooltip.MinimumTooltipSize = size;
		this.SuperTooltip1.ShowTooltipImmediately = true;
		this.SuperTooltip1.TooltipDuration = 60;
		this.ProgressBar1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ProgressBar1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBar = this.ProgressBar1;
		location = new System.Drawing.Point(374, 624);
		progressBar.Location = location;
		this.ProgressBar1.Name = "ProgressBar1";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBar2 = this.ProgressBar1;
		size = new System.Drawing.Size(516, 23);
		progressBar2.Size = size;
		this.ProgressBar1.TabIndex = 15;
		this.Label2.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left;
		this.Label2.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.Label2.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.Label label8 = this.Label2;
		location = new System.Drawing.Point(254, 620);
		label8.Location = location;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label9 = this.Label2;
		size = new System.Drawing.Size(131, 31);
		label9.Size = size;
		this.Label2.TabIndex = 16;
		this.Label2.Text = "สถานะการโหลดห\u0e49อง";
		this.Label2.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(7f, 16f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(1147, 661);
		this.ClientSize = size;
		this.Controls.Add(this.ProgressBar1);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Button4);
		this.Controls.Add(this.Lnum);
		this.Controls.Add(this.Label6);
		this.Controls.Add(this.Label7);
		this.Controls.Add(this.Button3);
		this.Controls.Add(this.FlowLayoutPanel1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
		this.Margin = margin;
		this.Name = "FormBookRooms";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "รายการห\u0e49องว\u0e48าง";
		this.FlowLayoutPanel1.ResumeLayout(false);
		this.ResumeLayout(false);
	}

	private void FormBookRooms_Load(object sender, EventArgs e)
	{
		ProgressBar1.Value = 0;
		checked
		{
			if (Module1.Booking_Room_amount > 20)
			{
				FlowLayoutPanel flowLayoutPanel = FlowLayoutPanel1;
				Size size = new Size(1132 + (Module1.Booking_Room_amount - 20) * 45, 596);
				flowLayoutPanel.Size = size;
				size = new Size(1163 + (Module1.Booking_Room_amount - 20) * 45, 700);
				Size = size;
			}
			else
			{
				FlowLayoutPanel flowLayoutPanel2 = FlowLayoutPanel1;
				Size size = new Size(1132 - (20 - Module1.Booking_Room_amount) * 45, 596);
				flowLayoutPanel2.Size = size;
				size = new Size(1163 - (20 - Module1.Booking_Room_amount) * 45, 700);
				Size = size;
			}
			Room_Arr.Clear();
			Lnum.Text = Conversions.ToString(0);
			if ((decimal.Compare(Conversions.ToDecimal(Strings.Format(start_date, "HHmm")), 0m) >= 0) & (decimal.Compare(Conversions.ToDecimal(Strings.Format(start_date, "HHmm")), new decimal(Module1.CHK_IN_Before)) <= 0))
			{
				start_date = start_date.AddDays(-1.0);
			}
			Timer1.Enabled = true;
		}
	}

	public void LoadBook()
	{
		Cursor = Cursors.WaitCursor;
		int num = 28;
		int num2 = 45;
		int booking_Room_amount = Module1.Booking_Room_amount;
		string text = "";
		FlowLayoutPanel1.Controls.Clear();
		Application.DoEvents();
		FlowLayoutPanel1.SuspendLayout();
		DataSet dataSet = Module1.connect("select * from HT_Room_Status where (room_status='จอง' or room_status='เข\u0e49าพ\u0e31ก') and (Room_date between '" + Conversions.ToString(start_date.AddDays(-2.0)) + "' and '" + Conversions.ToString(start_date.AddDays(15.0)) + "')");
		DataSet dataSet2 = Module1.connect("select * from View_Book_Date where (book_date_ds between '" + Conversions.ToString(start_date.AddDays(-2.0)) + "' and '" + Conversions.ToString(start_date.AddDays(15.0)) + "') and book_status='จอง'");
		DataSet dataSet3 = Module1.connect("select Room_no,Room_Type,Room_details,room_book,Room_Book_Name,Room_Manternace from HT_Rooms order by room_no");
		DataSet dataSet4 = Module1.connect("select cin_room_no,cin_cust_name,cin_no from View_CheckIn_Ds where cin_room_status='เข\u0e49าพ\u0e31ก' order by id");
		ProgressBar1.Value = 0;
		ProgressBar1.Maximum = dataSet3.Tables[0].Rows.Count;
		checked
		{
			int num3 = dataSet3.Tables[0].Rows.Count - 1;
			int num4 = 0;
			while (true)
			{
				int num5 = num4;
				int num6 = num3;
				if (num5 > num6)
				{
					break;
				}
				try
				{
					ProgressBar1.Value = num4 + 1;
				}
				catch (Exception projectError)
				{
					ProjectData.SetProjectError(projectError);
					ProjectData.ClearProjectError();
				}
				bool flag = false;
				if (unchecked(num4 % 21) == 0)
				{
					FlowLayoutPanel1.ResumeLayout();
					FlowLayoutPanel1.SuspendLayout();
				}
				Application.DoEvents();
				if (Operators.CompareString(dataSet3.Tables[0].Rows[num4]["Room_Manternace"].ToString().ToLower(), "yes", TextCompare: false) == 0)
				{
					flag = true;
				}
				if (!Module1.booking_use_Manternace)
				{
					flag = false;
				}
				Button button = new Button();
				button.BackColor = Color.PaleTurquoise;
				System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(0);
				button.Margin = margin;
				button.Name = Conversions.ToString(dataSet3.Tables[0].Rows[num4]["Room_no"]);
				Size size = new Size(200, num);
				button.Size = size;
				button.Anchor = AnchorStyles.Left;
				button.Anchor = AnchorStyles.Right;
				button.FlatStyle = FlatStyle.Flat;
				button.Text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet3.Tables[0].Rows[num4]["Room_no"], "     "), dataSet3.Tables[0].Rows[num4]["Room_Type"]), "  "), dataSet3.Tables[0].Rows[num4]["Room_details"].ToString()));
				button.TextAlign = ContentAlignment.TopLeft;
				FlowLayoutPanel1.Controls.Add(button);
				int num7 = booking_Room_amount - 1;
				int num8 = 0;
				while (true)
				{
					int num9 = num8;
					num6 = num7;
					if (num9 > num6)
					{
						break;
					}
					bool flag2 = false;
					bool flag3 = false;
					string text2 = "";
					int num10 = Room_Old.Count - 1;
					int num11 = 0;
					while (true)
					{
						int num12 = num11;
						num6 = num10;
						if (num12 > num6)
						{
							break;
						}
						if (Operators.ConditionalCompareObjectEqual(NewLateBinding.LateIndexGet(Room_Old[num11], new object[1] { 0 }, null), dataSet3.Tables[0].Rows[num4]["Room_no"], TextCompare: false))
						{
							int num13 = Conversions.ToInteger(NewLateBinding.LateIndexGet(Room_Old[num11], new object[1] { 2 }, null)) - 1;
							int num14 = 0;
							while (true)
							{
								int num15 = num14;
								num6 = num13;
								if (num15 > num6)
								{
									break;
								}
								DateTime dateTime = Conversions.ToDate(NewLateBinding.LateIndexGet(Room_Old[num11], new object[1] { 1 }, null));
								if (Operators.CompareString(Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy"), Strings.Format(dateTime.AddDays(num14), "MM/dd/yyyy"), TextCompare: false) == 0)
								{
									flag2 = true;
								}
								num14++;
							}
						}
						num11++;
					}
					if (!flag2)
					{
						int num16 = dataSet.Tables[0].Rows.Count - 1;
						int num17 = 0;
						while (true)
						{
							int num18 = num17;
							num6 = num16;
							if (num18 > num6)
							{
								break;
							}
							if (!Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(dataSet.Tables[0].Rows[num17]["room_no"], dataSet3.Tables[0].Rows[num4]["Room_no"], TextCompare: false), Operators.CompareString(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet.Tables[0].Rows[num17]["Room_date"]), "MM/dd/yyyy"), Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy"), TextCompare: false) == 0)))
							{
								num17++;
								continue;
							}
							text2 = "ห\u0e49องน\u0e35\u0e49ม\u0e35ผ\u0e39\u0e49เข\u0e49าพ\u0e31ก ";
							int num19 = dataSet4.Tables[0].Rows.Count - 1;
							int num20 = 0;
							while (true)
							{
								int num21 = num20;
								num6 = num19;
								if (num21 > num6)
								{
									break;
								}
								if (Operators.ConditionalCompareObjectEqual(dataSet4.Tables[0].Rows[num20]["cin_room_no"], dataSet.Tables[0].Rows[num17]["room_no"], TextCompare: false))
								{
									text = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet4.Tables[0].Rows[num20]["cin_no"], "\r\n"), dataSet4.Tables[0].Rows[num20]["cin_cust_name"]));
								}
								num20++;
							}
							flag3 = true;
							break;
						}
					}
					if (!flag2)
					{
						int num22 = dataSet2.Tables[0].Rows.Count - 1;
						int num23 = 0;
						while (true)
						{
							int num24 = num23;
							num6 = num22;
							if (num24 <= num6)
							{
								if (!Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(dataSet2.Tables[0].Rows[num23]["book_type"], dataSet3.Tables[0].Rows[num4]["Room_no"], TextCompare: false), Operators.CompareString(Strings.Format(RuntimeHelpers.GetObjectValue(dataSet2.Tables[0].Rows[num23]["book_date_ds"]), "MM/dd/yyyy"), Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy"), TextCompare: false) == 0)))
								{
									num23++;
									continue;
								}
								text2 = Conversions.ToString(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(Operators.ConcatenateObject(dataSet2.Tables[0].Rows[num23]["book_no"], "\r\n"), "ช\u0e37\u0e48อผ\u0e39\u0e49จอง "), dataSet2.Tables[0].Rows[num23]["book_cust_name"]), " "), dataSet2.Tables[0].Rows[num23]["book_cust_name2"]));
								flag3 = true;
								break;
							}
							break;
						}
					}
					if (Conversions.ToBoolean(Operators.AndObject(unchecked(flag && !flag3 && !flag2), Operators.CompareObjectEqual(dataSet3.Tables[0].Rows[num4]["Room_Manternace"], "yes", TextCompare: false))))
					{
						Button button2 = new Button();
						button2.BackColor = Color.LightGray;
						button2.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button2.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button2.Margin = margin;
						button2.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button2.Size = size;
						button2.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button2.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						FlowLayoutPanel1.Controls.Add(button2);
						SuperTooltip1.SetSuperTooltip(button2, new SuperTooltipInfo("จองห\u0e49องพ\u0e31ก", "iHOTEL", "ห\u0e49องน\u0e35\u0e49ซ\u0e48อมย\u0e31งไม\u0e48เสร\u0e47จ ไม\u0e48สามารถจองได\u0e49", Resources.boy_emoticon_009, null, eTooltipColor.Gray));
					}
					else if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareString(Strings.Format(start_date.AddDays(num8), "dd/MM"), Strings.Format(DateTime.Now, "dd/MM"), TextCompare: false) == 0, Operators.CompareObjectEqual(dataSet3.Tables[0].Rows[num4]["Room_Manternace"], "yes", TextCompare: false))))
					{
						Button button3 = new Button();
						button3.BackColor = Color.LightGray;
						button3.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button3.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button3.Margin = margin;
						button3.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button3.Size = size;
						button3.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button3.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						FlowLayoutPanel1.Controls.Add(button3);
						SuperTooltip1.SetSuperTooltip(button3, new SuperTooltipInfo("จองห\u0e49องพ\u0e31ก", "iHOTEL", "ห\u0e49องน\u0e35\u0e49ซ\u0e48อมย\u0e31งไม\u0e48เสร\u0e47จ ไม\u0e48สามารถจองได\u0e49", Resources.boy_emoticon_009, null, eTooltipColor.Gray));
					}
					else if ((Operators.CompareString(Strings.Format(start_date.AddDays(num8), "dd/MM"), Strings.Format(DateTime.Now, "dd/MM"), TextCompare: false) == 0) & (Operators.CompareString(dataSet3.Tables[0].Rows[num4]["room_book"].ToString(), "", TextCompare: false) != 0))
					{
						Button button4 = new Button();
						button4.BackColor = Color.LightPink;
						button4.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button4.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button4.Margin = margin;
						button4.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button4.Size = size;
						button4.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button4.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						FlowLayoutPanel1.Controls.Add(button4);
						SuperTooltip1.SetSuperTooltip(button4, new SuperTooltipInfo("จองห\u0e49องพ\u0e31ก", "iHOTEL", "ห\u0e49องน\u0e35\u0e49ม\u0e35การจองห\u0e49องแต\u0e48ย\u0e31งไม\u0e48ได\u0e49เข\u0e49าพ\u0e31ก ช\u0e37\u0e48อผ\u0e39\u0e49จองค\u0e37อ " + dataSet3.Tables[0].Rows[num4]["Room_Book_Name"].ToString(), Resources.boy_emoticon_009, null, eTooltipColor.Yellow));
					}
					else if (flag2)
					{
						Button button5 = new Button();
						button5.BackColor = Color.Yellow;
						button5.Cursor = Cursors.Hand;
						button5.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button5.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button5.Margin = margin;
						button5.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button5.Size = size;
						button5.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button5.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						button5.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
						{
							method_0(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
						};
						FlowLayoutPanel1.Controls.Add(button5);
						Lnum.Text = Conversions.ToString(decimal.Add(Conversions.ToDecimal(Lnum.Text), 1m));
					}
					else if (flag3 & (Operators.CompareString(text2, "ห\u0e49องน\u0e35\u0e49ม\u0e35ผ\u0e39\u0e49เข\u0e49าพ\u0e31ก ", TextCompare: false) == 0))
					{
						Button button6 = new Button();
						button6.BackColor = Color.Violet;
						button6.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button6.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button6.Margin = margin;
						button6.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button6.Size = size;
						button6.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button6.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						FlowLayoutPanel1.Controls.Add(button6);
						SuperTooltip1.SetSuperTooltip(button6, new SuperTooltipInfo("จองห\u0e49องพ\u0e31ก", "iHOTEL", text2 + " " + text, Resources.boy_emoticon_009, null, eTooltipColor.Orange));
					}
					else if (DateTime.Compare(DateTime.Now.AddDays(-1.0), start_date.AddDays(num8)) > 0)
					{
						Button button7 = new Button();
						button7.BackColor = Color.LightGray;
						button7.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button7.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button7.Margin = margin;
						button7.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button7.Size = size;
						button7.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button7.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						FlowLayoutPanel1.Controls.Add(button7);
					}
					else if (!flag3)
					{
						Button button8 = new Button();
						button8.BackColor = Color.LightGreen;
						button8.Cursor = Cursors.Hand;
						button8.FlatStyle = FlatStyle.Flat;
						Point location = new Point(0, 0);
						button8.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button8.Margin = margin;
						button8.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button8.Size = size;
						button8.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button8.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						button8.Click += [SpecialName] [DebuggerStepThrough] (object sender, EventArgs e) =>
						{
							method_0(RuntimeHelpers.GetObjectValue(sender), (MouseEventArgs)e);
						};
						FlowLayoutPanel1.Controls.Add(button8);
					}
					else
					{
						Button button9 = new Button();
						button9.BackColor = Color.Tomato;
						button9.FlatStyle = FlatStyle.Flat;
						Point location = new Point(1, 0);
						button9.Location = location;
						margin = new System.Windows.Forms.Padding(0);
						button9.Margin = margin;
						button9.Name = Strings.Format(start_date.AddDays(num8), "MM/dd/yyyy");
						size = new Size(num2, num);
						button9.Size = size;
						button9.Text = Strings.Format(start_date.AddDays(num8), "dd/MM");
						button9.Font = new Font("Tahoma", 8.25f, FontStyle.Regular, GraphicsUnit.Point, 222);
						FlowLayoutPanel1.Controls.Add(button9);
						SuperTooltip1.SetSuperTooltip(button9, new SuperTooltipInfo("จองห\u0e49องพ\u0e31ก", "iHOTEL", text2, Resources.boy_emoticon_009, null, eTooltipColor.Orange));
					}
					num8++;
				}
				num4++;
			}
			FlowLayoutPanel1.ResumeLayout();
			Cursor = Cursors.Default;
		}
	}

	public void method_0(object sender, MouseEventArgs e)
	{
		object obj = NewLateBinding.LateGet(sender, null, "BackColor", new object[0], null, null, null);
		checked
		{
			Color color = default(Color);
			if (((obj != null) ? ((Color)obj) : color) == Color.LightGreen)
			{
				NewLateBinding.LateSet(sender, null, "BackColor", new object[1] { Color.Yellow }, null, null);
				Lnum.Text = Conversions.ToString(Conversions.ToInteger(Lnum.Text) + 1);
			}
			else
			{
				NewLateBinding.LateSet(sender, null, "BackColor", new object[1] { Color.LightGreen }, null, null);
				Lnum.Text = Conversions.ToString(Conversions.ToInteger(Lnum.Text) - 1);
			}
		}
	}

	private void Timer1_Tick(object sender, EventArgs e)
	{
		Timer1.Enabled = false;
		LoadBook();
	}

	private void Button3_Click(object sender, EventArgs e)
	{
		if (ProgressBar1.Value != ProgressBar1.Maximum)
		{
			MessageBox.Show("กร\u0e38ณารอให\u0e49โหลดห\u0e49องเสร\u0e47จก\u0e48อน..", "ERROR", MessageBoxButtons.OK, MessageBoxIcon.Hand);
			return;
		}
		Room_Arr.Clear();
		string text = "";
		string text2 = "";
		string text3 = "";
		checked
		{
			int num = FlowLayoutPanel1.Controls.Count - 1;
			int num2 = 0;
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 > num4)
				{
					break;
				}
				if (FlowLayoutPanel1.Controls[num2].BackColor == Color.PaleTurquoise)
				{
					if ((Operators.CompareString(text2, "", TextCompare: false) != 0) & (Operators.CompareString(text3, "", TextCompare: false) == 0))
					{
						text3 = Conversions.ToString(Conversions.ToDate(FlowLayoutPanel1.Controls[num2 - 1].Name).AddDays(1.0));
					}
					if ((Operators.CompareString(text2, "", TextCompare: false) != 0) & (Operators.CompareString(text3, "", TextCompare: false) != 0))
					{
						string[] value = new string[3] { text, text2, text3 };
						Room_Arr.Add(value);
						text2 = "";
						text3 = "";
					}
					text = FlowLayoutPanel1.Controls[num2].Name;
				}
				else if (FlowLayoutPanel1.Controls[num2].BackColor == Color.Yellow)
				{
					if (Operators.CompareString(text2, "", TextCompare: false) == 0)
					{
						text2 = FlowLayoutPanel1.Controls[num2].Name;
						text3 = "";
					}
					else
					{
						text3 = Conversions.ToString(Conversions.ToDate(FlowLayoutPanel1.Controls[num2].Name).AddDays(1.0));
					}
				}
				else
				{
					if ((Operators.CompareString(text2, "", TextCompare: false) != 0) & (Operators.CompareString(text3, "", TextCompare: false) == 0))
					{
						text3 = FlowLayoutPanel1.Controls[num2].Name;
					}
					if ((Operators.CompareString(text2, "", TextCompare: false) != 0) & (Operators.CompareString(text3, "", TextCompare: false) != 0))
					{
						string[] value2 = new string[3] { text, text2, text3 };
						Room_Arr.Add(value2);
						text2 = "";
						text3 = "";
					}
				}
				if (num2 == FlowLayoutPanel1.Controls.Count - 1)
				{
					if ((Operators.CompareString(text2, "", TextCompare: false) != 0) & (Operators.CompareString(text3, "", TextCompare: false) == 0))
					{
						text3 = Conversions.ToString(Conversions.ToDate(FlowLayoutPanel1.Controls[num2].Name).AddDays(1.0));
					}
					if ((Operators.CompareString(text2, "", TextCompare: false) != 0) & (Operators.CompareString(text3, "", TextCompare: false) != 0))
					{
						string[] value3 = new string[3] { text, text2, text3 };
						Room_Arr.Add(value3);
						text2 = "";
						text3 = "";
					}
				}
				num2++;
			}
			Close();
		}
	}

	private void FlowLayoutPanel1_Paint(object sender, PaintEventArgs e)
	{
	}

	private void Label7_Click(object sender, EventArgs e)
	{
	}

	private void Label6_Click(object sender, EventArgs e)
	{
	}

	private void Lnum_Click(object sender, EventArgs e)
	{
	}

	private void Button4_Click(object sender, EventArgs e)
	{
		Close();
	}
}
