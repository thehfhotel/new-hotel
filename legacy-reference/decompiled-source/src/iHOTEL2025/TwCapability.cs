using System;
using System.Runtime.InteropServices;

namespace iHOTEL2025;

[StructLayout(LayoutKind.Sequential, Pack = 2)]
internal class TwCapability
{
	public short Cap;

	public short ConType;

	public IntPtr Handle;

	// REFERENCE-CODEBASE PATCH: decompiler emitted an instance method with the same name as the enclosing type (invalid C#). The IL was a constructor; renamed to a constructor.
	public TwCapability(TwCap capIn)
	{
		Cap = (short)capIn;
		ConType = -1;
	}

	public TwCapability(TwCap capIn, short sval)
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		Cap = (short)capIn;
		ConType = 5;
		Handle = Twain.GlobalAlloc(66, 6);
		IntPtr ptr = Twain.GlobalLock(Handle);
		Marshal.WriteInt16(ptr, 0, 1);
		Marshal.WriteInt32(ptr, 2, sval);
		Twain.GlobalUnlock(Handle);
	}

	public void Dispose()
	{
		if (!object.Equals(Handle, IntPtr.Zero))
		{
			Twain.GlobalFree(Handle);
		}
	}

	protected virtual void Finalize()
	{
		if (!object.Equals(Handle, IntPtr.Zero))
		{
			Twain.GlobalFree(Handle);
		}
	}
}
