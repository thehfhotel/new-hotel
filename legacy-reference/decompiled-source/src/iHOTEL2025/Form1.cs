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
public class Form1 : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("CheckBoxX1")]
	private CheckBoxX _CheckBoxX1;

	internal virtual CheckBoxX CheckBoxX_0
	{
		[DebuggerNonUserCode]
		get
		{
			return _CheckBoxX1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = CheckBoxX1_CheckedChanged;
			if (_CheckBoxX1 != null)
			{
				_CheckBoxX1.CheckedChanged -= value2;
			}
			_CheckBoxX1 = value;
			if (_CheckBoxX1 != null)
			{
				_CheckBoxX1.CheckedChanged += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static Form1()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public Form1()
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
		this.CheckBoxX_0 = new DevComponents.DotNetBar.Controls.CheckBoxX();
		this.SuspendLayout();
		this.CheckBoxX_0.BackgroundStyle.Class = "";
		DevComponents.DotNetBar.Controls.CheckBoxX checkBoxX_ = this.CheckBoxX_0;
		System.Drawing.Point location = new System.Drawing.Point(28, 12);
		checkBoxX_.Location = location;
		this.CheckBoxX_0.Name = "CheckBoxX1";
		DevComponents.DotNetBar.Controls.CheckBoxX checkBoxX_2 = this.CheckBoxX_0;
		System.Drawing.Size size = new System.Drawing.Size(107, 23);
		checkBoxX_2.Size = size;
		this.CheckBoxX_0.Style = DevComponents.DotNetBar.eDotNetBarStyle.StyleManagerControlled;
		this.CheckBoxX_0.TabIndex = 0;
		this.CheckBoxX_0.Text = "CheckBoxX1";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(284, 262);
		this.ClientSize = size;
		this.Controls.Add(this.CheckBoxX_0);
		this.Name = "Form1";
		this.Text = "Form1";
		this.ResumeLayout(false);
	}

	private void CheckBoxX1_CheckedChanged(object sender, EventArgs e)
	{
	}
}
