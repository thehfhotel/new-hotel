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
public class frmWanting : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("ProgressBarX1")]
	private ProgressBarX _ProgressBarX1;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

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

	[DebuggerNonUserCode]
	static frmWanting()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public frmWanting()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
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
		this.ProgressBarX1 = new DevComponents.DotNetBar.Controls.ProgressBarX();
		this.Label1 = new System.Windows.Forms.Label();
		this.SuspendLayout();
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX = this.ProgressBarX1;
		System.Drawing.Point location = new System.Drawing.Point(11, 8);
		progressBarX.Location = location;
		this.ProgressBarX1.Name = "ProgressBarX1";
		this.ProgressBarX1.ProgressType = DevComponents.DotNetBar.eProgressItemType.Marquee;
		DevComponents.DotNetBar.Controls.ProgressBarX progressBarX2 = this.ProgressBarX1;
		System.Drawing.Size size = new System.Drawing.Size(242, 16);
		progressBarX2.Size = size;
		this.ProgressBarX1.TabIndex = 0;
		this.ProgressBarX1.Text = "ProgressBarX1";
		System.Windows.Forms.Label label = this.Label1;
		location = new System.Drawing.Point(11, 25);
		label.Location = location;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label2 = this.Label1;
		size = new System.Drawing.Size(242, 30);
		label2.Size = size;
		this.Label1.TabIndex = 1;
		this.Label1.Text = "กร\u0e38ณารอส\u0e31กคร\u0e39\u0e48";
		this.Label1.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(269, 53);
		this.ClientSize = size;
		this.ControlBox = false;
		this.Controls.Add(this.Label1);
		this.Controls.Add(this.ProgressBarX1);
		this.Name = "frmWanting";
		this.Opacity = 0.9;
		this.ShowIcon = false;
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "กร\u0e38ณารอ..";
		this.TopMost = true;
		this.ResumeLayout(false);
	}
}
