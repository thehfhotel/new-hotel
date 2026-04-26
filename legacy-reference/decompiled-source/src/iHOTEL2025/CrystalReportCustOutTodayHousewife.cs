using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using CrystalDecisions.CrystalReports.Engine;

namespace iHOTEL2025;

public class CrystalReportCustOutTodayHousewife : ReportClass
{
	private static List<WeakReference> __ENCList;

	public override string ResourceName
	{
		get
		{
			return "CrystalReportCustOutTodayHousewife.rpt";
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
			return "iHOTEL2025.CrystalReportCustOutTodayHousewife.rpt";
		}
		set
		{
		}
	}

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	public Section Section1 => ReportDefinition.Sections[0];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section Section2 => ReportDefinition.Sections[1];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section Section3 => ReportDefinition.Sections[2];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section Section4 => ReportDefinition.Sections[3];

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public Section Section5 => ReportDefinition.Sections[4];

	[DebuggerNonUserCode]
	static CrystalReportCustOutTodayHousewife()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public CrystalReportCustOutTodayHousewife()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
	}
}
