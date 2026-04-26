using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using CrystalDecisions.Windows.Forms;
using DevComponents.DotNetBar;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FrmPrint : Office2007Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("CrystalReportViewer1")]
	private CrystalReportViewer _CrystalReportViewer1;

	internal virtual CrystalReportViewer CrystalReportViewer1
	{
		[DebuggerNonUserCode]
		get
		{
			return _CrystalReportViewer1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			CrystalDecisions.Windows.Forms.RefreshEventHandler value2 = CrystalReportViewer1_ReportRefresh;
			if (_CrystalReportViewer1 != null)
			{
				_CrystalReportViewer1.ReportRefresh -= value2;
			}
			_CrystalReportViewer1 = value;
			if (_CrystalReportViewer1 != null)
			{
				_CrystalReportViewer1.ReportRefresh += value2;
			}
		}
	}

	[DebuggerNonUserCode]
	static FrmPrint()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	[DebuggerNonUserCode]
	public FrmPrint()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		base.Load += FrmPrint_Load;
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
		this.CrystalReportViewer1 = new CrystalDecisions.Windows.Forms.CrystalReportViewer();
		this.SuspendLayout();
		this.CrystalReportViewer1.ActiveViewIndex = -1;
		this.CrystalReportViewer1.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
		this.CrystalReportViewer1.DisplayGroupTree = false;
		this.CrystalReportViewer1.Dock = System.Windows.Forms.DockStyle.Fill;
		CrystalDecisions.Windows.Forms.CrystalReportViewer crystalReportViewer = this.CrystalReportViewer1;
		System.Drawing.Point location = new System.Drawing.Point(0, 0);
		crystalReportViewer.Location = location;
		this.CrystalReportViewer1.Name = "CrystalReportViewer1";
		this.CrystalReportViewer1.SelectionFormula = "";
		CrystalDecisions.Windows.Forms.CrystalReportViewer crystalReportViewer2 = this.CrystalReportViewer1;
		System.Drawing.Size size = new System.Drawing.Size(881, 664);
		crystalReportViewer2.Size = size;
		this.CrystalReportViewer1.TabIndex = 0;
		this.CrystalReportViewer1.ViewTimeSelectionFormula = "";
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(6f, 13f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(881, 664);
		this.ClientSize = size;
		this.Controls.Add(this.CrystalReportViewer1);
		this.Name = "FrmPrint";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "FrmPrint";
		this.TopMost = true;
		this.ResumeLayout(false);
	}

	private void FrmPrint_Load(object sender, EventArgs e)
	{
		Cursor = Cursors.WaitCursor;
	}

	private void CrystalReportViewer1_ReportRefresh(object sender, ViewerEventArgs e)
	{
		Cursor = Cursors.Default;
	}
}
