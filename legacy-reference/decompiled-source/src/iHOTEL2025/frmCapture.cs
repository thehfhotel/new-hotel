using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class frmCapture : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("picOutput")]
	private PictureBox _picOutput;

	[AccessedThroughProperty("ButtonX1")]
	private ButtonX _ButtonX1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ComboBox1")]
	private ComboBox _ComboBox1;

	private iCam myCam;

	public static Bitmap myBitmap;

	internal virtual PictureBox picOutput
	{
		[DebuggerNonUserCode]
		get
		{
			return _picOutput;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_picOutput = value;
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

	internal virtual ComboBox ComboBox1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ComboBox1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = ComboBox1_SelectedIndexChanged;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged -= value2;
			}
			_ComboBox1 = value;
			if (_ComboBox1 != null)
			{
				_ComboBox1.SelectedIndexChanged += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static frmCapture()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public frmCapture()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.FormClosing += frmCapture_FormClosing;
		base.Load += frmCapture_Load;
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
		this.picOutput = new System.Windows.Forms.PictureBox();
		this.ButtonX1 = new DevComponents.DotNetBar.ButtonX();
		this.Label1 = new System.Windows.Forms.Label();
		this.ComboBox1 = new System.Windows.Forms.ComboBox();
		((System.ComponentModel.ISupportInitialize)this.picOutput).BeginInit();
		this.SuspendLayout();
		this.picOutput.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.picOutput.BackColor = System.Drawing.Color.DimGray;
		this.picOutput.Enabled = false;
		System.Windows.Forms.PictureBox pictureBox = this.picOutput;
		System.Drawing.Point location = new System.Drawing.Point(3, 33);
		pictureBox.Location = location;
		this.picOutput.Name = "picOutput";
		System.Windows.Forms.PictureBox pictureBox2 = this.picOutput;
		System.Drawing.Size size = new System.Drawing.Size(482, 644);
		pictureBox2.Size = size;
		this.picOutput.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
		this.picOutput.TabIndex = 1;
		this.picOutput.TabStop = false;
		this.ButtonX1.AccessibleRole = System.Windows.Forms.AccessibleRole.PushButton;
		this.ButtonX1.Anchor = System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
		this.ButtonX1.Font = new System.Drawing.Font("Tahoma", 15.75f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
		DevComponents.DotNetBar.ButtonX buttonX = this.ButtonX1;
		location = new System.Drawing.Point(3, 683);
		buttonX.Location = location;
		this.ButtonX1.Name = "ButtonX1";
		DevComponents.DotNetBar.ButtonX buttonX2 = this.ButtonX1;
		size = new System.Drawing.Size(482, 54);
		buttonX2.Size = size;
		this.ButtonX1.TabIndex = 2;
		this.ButtonX1.Text = "ถ\u0e48ายร\u0e39ป";
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(3, 11);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(71, 13);
		label2.Size = size;
		this.Label1.TabIndex = 3;
		this.Label1.Text = "ขนาดจอภาพ :";
		this.ComboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
		this.ComboBox1.FormattingEnabled = true;
		this.ComboBox1.Items.AddRange(new object[6] { "220 x 368", "220 x 406", "287 x 496", "358 x 590", "426 x 683", "495 x 773" });
		System.Windows.Forms.ComboBox comboBox = this.ComboBox1;
		location = new System.Drawing.Point(76, 7);
		comboBox.Location = location;
		this.ComboBox1.Name = "ComboBox1";
		System.Windows.Forms.ComboBox comboBox2 = this.ComboBox1;
		size = new System.Drawing.Size(121, 21);
		comboBox2.Size = size;
		this.ComboBox1.TabIndex = 4;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(489, 741);
		this.ClientSize = size;
		this.Controls.Add(this.ComboBox1);
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.ButtonX1);
		this.Controls.Add(this.picOutput);
		this.DoubleBuffered = true;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "frmCapture";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "นำภาพเข\u0e49าจาก WebCam";
		((System.ComponentModel.ISupportInitialize)this.picOutput).EndInit();
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void frmCapture_FormClosing(object sender, FormClosingEventArgs e)
	{
		try
		{
			myCam.closeCam();
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			ProjectData.ClearProjectError();
		}
	}

	private void frmCapture_Load(object sender, EventArgs e)
	{
		if (Operators.CompareString(Module1.localdata.Config.Rows[0]["store_camera"].ToString(), "", TextCompare: false) == 0)
		{
			Module1.localdata.Config.Rows[0]["store_camera"] = "220 x 368";
			ComboBox1.Text = Conversions.ToString(Module1.localdata.Config.Rows[0]["store_camera"]);
		}
		else
		{
			ComboBox1.Text = Conversions.ToString(Module1.localdata.Config.Rows[0]["store_camera"]);
		}
		SETsize();
		picOutput.SizeMode = PictureBoxSizeMode.StretchImage;
		myCam = new iCam();
		try
		{
			myCam.initCam(picOutput.Handle.ToInt32());
		}
		catch (Exception ex)
		{
			ProjectData.SetProjectError(ex);
			Exception ex2 = ex;
			MessageBox.Show(ex2.Message);
			Close();
			ProjectData.ClearProjectError();
		}
	}

	private void ButtonX1_Click(object sender, EventArgs e)
	{
		if (myCam.iRunning)
		{
			iCam obj = myCam;
			PictureBox src = picOutput;
			RectangleF rect = new RectangleF(0f, 0f, picOutput.Width, picOutput.Height);
			myBitmap = obj.copyFrame(src, rect);
			myBitmap.Save(Module1.PathF + "/capture.bmp");
		}
		else
		{
			MessageBox.Show("กล\u0e49องถ\u0e39กป\u0e34ดอย\u0e39\u0e48!");
		}
		Close();
	}

	public void SETsize()
	{
		if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["store_camera"], "220 x 368", TextCompare: false))
		{
			Size size = new Size(220, 368);
			Size = size;
			size = new Size(220, 368);
			MaximumSize = size;
			size = new Size(220, 368);
			MinimumSize = size;
		}
		else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["store_camera"], "220 x 406", TextCompare: false))
		{
			Size size = new Size(220, 406);
			Size = size;
			size = new Size(220, 406);
			MaximumSize = size;
			size = new Size(220, 406);
			MinimumSize = size;
		}
		else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["store_camera"], "287 x 496", TextCompare: false))
		{
			Size size = new Size(287, 496);
			Size = size;
			size = new Size(287, 496);
			MaximumSize = size;
			size = new Size(287, 496);
			MinimumSize = size;
		}
		else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["store_camera"], "358 x 590", TextCompare: false))
		{
			Size size = new Size(358, 590);
			Size = size;
			size = new Size(358, 590);
			MaximumSize = size;
			size = new Size(358, 590);
			MinimumSize = size;
		}
		else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["store_camera"], "426 x 683", TextCompare: false))
		{
			Size size = new Size(426, 683);
			Size = size;
			size = new Size(426, 683);
			MaximumSize = size;
			size = new Size(426, 683);
			MinimumSize = size;
		}
		else if (Operators.ConditionalCompareObjectEqual(Module1.localdata.Config.Rows[0]["store_camera"], "495 x 773", TextCompare: false))
		{
			Size size = new Size(495, 773);
			Size = size;
			size = new Size(495, 773);
			MaximumSize = size;
			size = new Size(495, 773);
			MinimumSize = size;
		}
	}

	private void ComboBox1_SelectedIndexChanged(object sender, EventArgs e)
	{
		SETsize();
		Module1.localdata.Config.Rows[0]["store_camera"] = ComboBox1.Text;
		Module1.saveConfig();
	}
}
