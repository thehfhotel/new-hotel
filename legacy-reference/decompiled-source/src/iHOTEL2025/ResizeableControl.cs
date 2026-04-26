using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Windows.Forms;

namespace iHOTEL2025;

public class ResizeableControl
{
	public enum EdgeEnum
	{
		None = 0,
		Right = 1,
		Left = 2,
		Top = 4,
		Bottom = 8,
		TopLeft = 0x10
	}

	public delegate void ResizeOccurredEventHandler(ref Control c);

	private static List<WeakReference> __ENCList;

	[AccessedThroughProperty("mControl")]
	private Control _mControl;

	private bool mMouseDown;

	private EdgeEnum mEdge;

	private int mWidth;

	private bool mOutlineDrawn;

	internal EdgeEnum _AllowEdges;

	private Color _HighlightColor;

	// REFERENCE-CODEBASE PATCH: removed `virtual` (decompiler emitted invalid 'private virtual'; original was likely a VB Property without virtual modifier).
	private Control mControl
	{
		[DebuggerNonUserCode]
		get
		{
			return _mControl;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			MouseEventHandler value2 = mControl_MouseMove;
			MouseEventHandler value3 = mControl_MouseUp;
			MouseEventHandler value4 = mControl_MouseDown;
			EventHandler value5 = mControl_MouseLeave;
			if (_mControl != null)
			{
				_mControl.MouseMove -= value2;
				_mControl.MouseUp -= value3;
				_mControl.MouseDown -= value4;
				_mControl.MouseLeave -= value5;
			}
			_mControl = value;
			if (_mControl != null)
			{
				_mControl.MouseMove += value2;
				_mControl.MouseUp += value3;
				_mControl.MouseDown += value4;
				_mControl.MouseLeave += value5;
			}
		}
	}

	public EdgeEnum AllowEdges
	{
		get
		{
			return _AllowEdges;
		}
		set
		{
			_AllowEdges = value;
		}
	}

	public Color HighlightColor
	{
		get
		{
			return _HighlightColor;
		}
		set
		{
			_HighlightColor = value;
		}
	}

	[method: DebuggerNonUserCode]
	public event ResizeOccurredEventHandler ResizeOccurred;

	[DebuggerNonUserCode]
	static ResizeableControl()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
	}

	public ResizeableControl(Control Control)
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		mMouseDown = false;
		mEdge = EdgeEnum.None;
		mWidth = 4;
		mOutlineDrawn = false;
		_AllowEdges = (EdgeEnum)31;
		_HighlightColor = Color.Fuchsia;
		mControl = Control;
	}

	private void mControl_MouseDown(object sender, MouseEventArgs e)
	{
		if (e.Button == MouseButtons.Left)
		{
			mMouseDown = true;
		}
	}

	private void mControl_MouseUp(object sender, MouseEventArgs e)
	{
		mMouseDown = false;
	}

	private void mControl_MouseMove(object sender, MouseEventArgs e)
	{
		Control c = (Control)sender;
		Graphics graphics = c.CreateGraphics();
		SolidBrush brush = new SolidBrush(_HighlightColor);
		checked
		{
			switch (unchecked((int)mEdge))
			{
			case 0:
				if (mOutlineDrawn)
				{
					c.Refresh();
					mOutlineDrawn = false;
				}
				break;
			case 1:
				graphics.FillRectangle(brush, c.Width - mWidth, 0, c.Width, c.Height);
				mOutlineDrawn = true;
				break;
			case 2:
				graphics.FillRectangle(brush, 0, 0, mWidth, c.Height);
				mOutlineDrawn = true;
				break;
			case 4:
				graphics.FillRectangle(brush, 0, 0, c.Width, mWidth);
				mOutlineDrawn = true;
				break;
			case 8:
				graphics.FillRectangle(brush, 0, c.Height - mWidth, c.Width, mWidth);
				mOutlineDrawn = true;
				break;
			case 16:
				graphics.FillRectangle(brush, 0, 0, mWidth * 4, mWidth * 4);
				mOutlineDrawn = true;
				break;
			}
			if (mMouseDown & (mEdge != EdgeEnum.None))
			{
				c.SuspendLayout();
				switch (unchecked((int)mEdge))
				{
				case 1:
					c.SetBounds(c.Left, c.Top, c.Width - (c.Width - e.X), c.Height);
					ResizeOccurred?.Invoke(ref c);
					break;
				case 2:
					c.SetBounds(c.Left + e.X, c.Top, c.Width - e.X, c.Height);
					ResizeOccurred?.Invoke(ref c);
					break;
				case 4:
					c.SetBounds(c.Left, c.Top + e.Y, c.Width, c.Height - e.Y);
					ResizeOccurred?.Invoke(ref c);
					break;
				case 8:
					c.SetBounds(c.Left, c.Top, c.Width, c.Height - (c.Height - e.Y));
					ResizeOccurred?.Invoke(ref c);
					break;
				case 16:
					c.SetBounds(c.Left + e.X, c.Top + e.Y, c.Width, c.Height);
					ResizeOccurred?.Invoke(ref c);
					break;
				}
				c.ResumeLayout();
			}
			else
			{
				bool flag = true;
				if ((e.X <= mWidth * 4) & (e.Y <= mWidth * 4))
				{
					c.Cursor = Cursors.SizeAll;
					mEdge = EdgeEnum.TopLeft;
				}
				else if (flag == e.X <= mWidth)
				{
					c.Cursor = Cursors.VSplit;
					mEdge = EdgeEnum.Left;
				}
				else if (flag == e.X > c.Width - (mWidth + 1))
				{
					c.Cursor = Cursors.VSplit;
					mEdge = EdgeEnum.Right;
				}
				else if (flag == e.Y <= mWidth)
				{
					c.Cursor = Cursors.HSplit;
					mEdge = EdgeEnum.Top;
				}
				else if (flag == e.Y > c.Height - (mWidth + 1))
				{
					c.Cursor = Cursors.HSplit;
					mEdge = EdgeEnum.Bottom;
				}
				else
				{
					c.Cursor = Cursors.Default;
					mEdge = EdgeEnum.None;
				}
				mEdge &= _AllowEdges;
				if (mEdge == EdgeEnum.None)
				{
					c.Cursor = Cursors.Default;
				}
			}
		}
	}

	private void mControl_MouseLeave(object sender, EventArgs e)
	{
		Control control = (Control)sender;
		mEdge = EdgeEnum.None;
		control.Refresh();
	}
}
