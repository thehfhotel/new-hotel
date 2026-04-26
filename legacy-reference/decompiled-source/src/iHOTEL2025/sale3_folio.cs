using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using CrystalDecisions.CrystalReports.Engine;

namespace iHOTEL2025;

public class sale3_folio : ReportClass
{
	private static List<WeakReference> __ENCList;

	public override string ResourceName
	{
		get
		{
			return "sale3_folio.rpt";
		}
		set
		{
		}
	}

	public override bool NewGenerator
	{
		get
		{
			return true;
		}
		set
		{
		}
	}

	public override string FullResourceName
	{
		get
		{
			return "iHOTEL2025.sale3_folio.rpt";
		}
		set
		{
		}
	}

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	public Section Section1 => ReportDefinition.Sections[0];

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	public Section Section2 => ReportDefinition.Sections[1];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section GroupHeaderSection1 => ReportDefinition.Sections[2];

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	public Section Section3 => ReportDefinition.Sections[3];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section GroupFooterSection1 => ReportDefinition.Sections[4];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section Section4 => ReportDefinition.Sections[5];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section Section5 => ReportDefinition.Sections[6];

	[DebuggerNonUserCode]
	static sale3_folio()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public sale3_folio()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
	}
}
