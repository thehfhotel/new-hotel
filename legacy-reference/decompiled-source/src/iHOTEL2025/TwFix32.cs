using System;
using System.Runtime.InteropServices;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal struct TwFix32
{
	public short Whole;

	public short Frac;

	public float ToFloat()
	{
		return (float)Whole + (float)Frac / 65536f;
	}

	public void FromFloat(float f)
	{
		checked
		{
			int num = (int)Math.Round(f * 65536f + 0.5f);
			Whole = (short)Math.Round((double)num / 65536.0);
			Frac = Conversions.ToShort(Conversions.ToString(num) + Conversions.ToString(65535));
		}
	}
}
