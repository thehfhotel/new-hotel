using System.Diagnostics;
using System.Runtime.InteropServices;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[StandardModule]
internal sealed class ModuleClick
{
	public struct POINTAPI
	{
		public long x;

		public long y;
	}

	[DebuggerNonUserCode]
	static ModuleClick()
	{
		Class2.LH6iGfYz9j3MJ();
	}

	[DllImport("user32", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	public static extern void mouse_event(long dwFlags, long dx, long dy, long cButtons, long dwExtraInfo);

	[DllImport("user32", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	public static extern long SetCursorPos(long x, long y);

	[DllImport("user32", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
	public static extern long GetCursorPos(POINTAPI lpPoint);

	public static long GetX()
	{
		POINTAPI lpPoint = default(POINTAPI);
		GetCursorPos(lpPoint);
		return lpPoint.x;
	}

	public static long GetY()
	{
		POINTAPI lpPoint = default(POINTAPI);
		GetCursorPos(lpPoint);
		return lpPoint.y;
	}

	public static void LeftClick()
	{
		LeftDown();
		LeftUp();
	}

	public static void LeftDown()
	{
		mouse_event(2L, 0L, 0L, 0L, 0L);
	}

	public static void LeftUp()
	{
		mouse_event(4L, 0L, 0L, 0L, 0L);
	}

	public static void MiddleClick()
	{
		MiddleDown();
		MiddleUp();
	}

	public static void MiddleDown()
	{
		mouse_event(32L, 0L, 0L, 0L, 0L);
	}

	public static void MiddleUp()
	{
		mouse_event(64L, 0L, 0L, 0L, 0L);
	}

	public static void RightClick()
	{
		RightDown();
		RightUp();
	}

	public static void RightDown()
	{
		mouse_event(8L, 0L, 0L, 0L, 0L);
	}

	public static void RightUp()
	{
		mouse_event(16L, 0L, 0L, 0L, 0L);
	}

	public static void MoveMouse(long xMove, long yMove)
	{
		mouse_event(1L, xMove, yMove, 0L, 0L);
	}

	public static void SetMousePos(long xPos, long yPos)
	{
		SetCursorPos(xPos, yPos);
	}
}
