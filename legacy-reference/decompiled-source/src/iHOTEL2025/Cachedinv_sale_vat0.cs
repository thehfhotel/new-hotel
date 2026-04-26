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
public class Cachedinv_sale_vat0 : Component, ICachedReport
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

	[Browsable(false)]
	[DesignerSerializationVisibility(DesignerSerializationVisibility.Hidden)]
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
	static Cachedinv_sale_vat0()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public Cachedinv_sale_vat0()
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
		inv_sale_vat0 inv_sale_vat2 = new inv_sale_vat0();
		inv_sale_vat2.Site = Site;
		return inv_sale_vat2;
	}

	public virtual string GetCustomizedCacheKey(RequestContext request)
	{
		return null;
	}
}
