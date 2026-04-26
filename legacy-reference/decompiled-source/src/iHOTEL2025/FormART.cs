using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using DevComponents.DotNetBar;
using DevComponents.DotNetBar.Controls;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormART : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBarX _ProgressBarX1;

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

	internal virtual ProgressBarX ProgressBarX1
	{
		[DebuggerNonUserCode]
		get
		{
			return _ProgressBarX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_ProgressBarX1 = value;
		}
	}

	[DebuggerNonUserCode]
	static FormART()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FormART()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FormART_Load;
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
		this.ProgressBarX1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(114, 57);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(5, 0, 5, 0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(285, 23);
		label3.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "กำล\u0e31งปร\u0e31บฐานข\u0e49อม\u0e39ลกร\u0e38ณารอส\u0e31กคร\u0e39\u0e48...";
		this.ProgressBarX1.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX = this.ProgressBarX1;
		location = new System.Drawing.Point(22, 23);
		progressBarX.Location = location;
		this.ProgressBarX1.Name = "ProgressBarX1";
		this.ProgressBarX1.ProgressType = DevComponents.DotNetBar.eProgressItemType.Marquee;
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX2 = this.ProgressBarX1;
		size = new System.Drawing.Size(469, 23);
		progressBarX2.Size = size;
		this.ProgressBarX1.TabIndex = 2;
		this.ProgressBarX1.Text = "ProgressBarX1";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(10f, 23f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(517, 93);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.ProgressBarX1);
		this.Controls.Add(this.Label1);
		this.DoubleBuffered = true;
		this.Font = new System.Drawing.Font("Tahoma", 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
		this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
		margin = new System.Windows.Forms.Padding(5);
		this.Margin = margin;
		this.Name = "FormART";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "ปร\u0e31บเปร\u0e35ยนฐานข\u0e49อม\u0e39ล";
		this.TopMost = true;
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	private void FormART_Load(object sender, EventArgs e)
	{
	}
}
