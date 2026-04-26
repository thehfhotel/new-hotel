using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using CrystalDecisions.CrystalReports.Engine;
using CrystalDecisions.ReportSource;
using CrystalDecisions.Shared;

namespace iHOTEL2025;

[ToolboxBitmap(typeof(ExportOptions), "report.bmp")]
public class CachedCrystalReportCustIn : Component, ICachedReport
{
	private static List<WeakReference> __ENCList;

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public virtual bool IsCacheable
	{
		get
		{
			return true;
		}
		set
		{
		}
	}

	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	[Browsable(false)]
	public virtual bool ShareDBLogonInfo
	{
		get
		{
			return false;
		}
		set
		{
		}
	}

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
	public virtual TimeSpan CacheTimeOut
	{
		get
		{
			return CachedReportConstants.DEFAULT_TIMEOUT;
		}
		set
		{
		}
	}

	[DebuggerNonUserCode]
	static CachedCrystalReportCustIn()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public CachedCrystalReportCustIn()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
	}

	public virtual ReportDocument CreateReport()
	{
		CrystalReportCustIn crystalReportCustIn = new CrystalReportCustIn();
		crystalReportCustIn.Site = Site;
		return crystalReportCustIn;
	}

	public virtual string GetCustomizedCacheKey(RequestContext request)
	{
		return null;
	}
}
