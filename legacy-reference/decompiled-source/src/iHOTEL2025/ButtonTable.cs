using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Windows.Forms;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class ButtonTable : Button
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	private string Gname;

	private string IName;

	public string GroupName
	{
		get
		{
			return Gname;
		}
		set
		{
			Gname = value;
		}
	}

	public string ImageName
	{
		get
		{
			return IName;
		}
		set
		{
			IName = value;
		}
	}

	[DebuggerNonUserCode]
	static ButtonTable()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public ButtonTable()
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
		this.Name = "Bpipe";
		System.Drawing.Size size = new System.Drawing.Size(268, 152);
		this.Size = size;
		this.ResumeLayout(false);
	}
}
