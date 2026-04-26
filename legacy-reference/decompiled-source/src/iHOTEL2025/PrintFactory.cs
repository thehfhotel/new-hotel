using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Printing;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Windows.Forms;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

public class PrintFactory
{
	private static List<WeakReference> __ENCList;

	[AccessedThroughProperty("_printDocument")]
	private PrintDocument __printDocument;

	private DataGridView _dgv;

	private StringFormat _stringFormat;

	private StringFormat _stringFormatComboBox;

	private Button _button;

	private CheckBox _checkBox;

	private ComboBox _comboBox;

	private short _totalWidth;

	private short _rowPos;

	private bool _newPage;

	private short _pageNumber;

	private string _userName;

	private string _headerText;

	private bool _blnGotNumPages;

	private string _strNumOfPages;

	private string _orientation;

	private short nnWidth;

	[SpecialName]
	private static StaticLocalInitFlag _0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init;

	[SpecialName]
	private static StaticLocalInitFlag _0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init;

	[SpecialName]
	private static ArrayList _0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts;

	[SpecialName]
	private static ArrayList _0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths;

	[SpecialName]
	private static ArrayList _0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes;

	[SpecialName]
	private static StaticLocalInitFlag _0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init;

	[SpecialName]
	private static short _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight;

	// REFERENCE-CODEBASE PATCH: removed `virtual` (decompiler emitted invalid 'private virtual'; original was likely a VB Property without virtual modifier).
	private PrintDocument _printDocument
	{
		[DebuggerNonUserCode]
		get
		{
			return __printDocument;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			PrintPageEventHandler value2 = _printDocument_PrintPage;
			if (__printDocument != null)
			{
				__printDocument.PrintPage -= value2;
			}
			__printDocument = value;
			if (__printDocument != null)
			{
				__printDocument.PrintPage += value2;
			}
		}
	}

	private DataGridView DGV => _dgv;

	private string HeaderText
	{
		get
		{
			return _headerText;
		}
		set
		{
			_headerText = value;
		}
	}

	private string UserName
	{
		get
		{
			return _userName;
		}
		set
		{
			_userName = value;
		}
	}

	private StringFormat StringFormat
	{
		get
		{
			return _stringFormat;
		}
		set
		{
			_stringFormat = value;
		}
	}

	private StringFormat StringFormatComboBox
	{
		get
		{
			return _stringFormatComboBox;
		}
		set
		{
			_stringFormatComboBox = value;
		}
	}

	private Button Button
	{
		get
		{
			return _button;
		}
		set
		{
			_button = value;
		}
	}

	private CheckBox CheckBox
	{
		get
		{
			return _checkBox;
		}
		set
		{
			_checkBox = value;
		}
	}

	private ComboBox ComboBox
	{
		get
		{
			return _comboBox;
		}
		set
		{
			_comboBox = value;
		}
	}

	private short TotalWidth
	{
		get
		{
			return _totalWidth;
		}
		set
		{
			_totalWidth = value;
		}
	}

	private short RowPos
	{
		get
		{
			return _rowPos;
		}
		set
		{
			_rowPos = value;
		}
	}

	private bool NewPage
	{
		get
		{
			return _newPage;
		}
		set
		{
			_newPage = value;
		}
	}

	private short PageNumber
	{
		get
		{
			return _pageNumber;
		}
		set
		{
			_pageNumber = value;
		}
	}

	private string strNumOfPages
	{
		get
		{
			return _strNumOfPages;
		}
		set
		{
			_strNumOfPages = value;
		}
	}

	private bool blnGotNumPages
	{
		get
		{
			return _blnGotNumPages;
		}
		set
		{
			_blnGotNumPages = value;
		}
	}

	private string orientation
	{
		get
		{
			return _orientation;
		}
		set
		{
			_orientation = value;
		}
	}

	[DebuggerNonUserCode]
	static PrintFactory()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
		_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init = new StaticLocalInitFlag();
		_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init = new StaticLocalInitFlag();
		_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init = new StaticLocalInitFlag();
	}

	private PrintFactory(string userName, string headerText)
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		_printDocument = new PrintDocument();
		_userName = userName;
		_headerText = headerText;
	}

	private void _printDocument_PrintPage(object sender, PrintPageEventArgs e)
	{
		EndPrint(this, e);
	}

	public static bool Print(DataGridView dgv, string userName, string headerText, string Orientation)
	{
		Validate(dgv, userName, headerText);
		PrintFactory printFactory = BeginPrint(dgv, userName, headerText);
		if (Operators.CompareString(Orientation, "L", TextCompare: false) == 0)
		{
			printFactory._printDocument.DefaultPageSettings.Landscape = true;
		}
		printFactory._printDocument.Print();
		bool result = default(bool);
		return result;
	}

	public static bool Preview(DataGridView dgv, string userName, string headerText, string Orientation)
	{
		Validate(dgv, userName, headerText);
		PrintFactory printFactory = BeginPrint(dgv, userName, headerText);
		if (Operators.CompareString(Orientation, "L", TextCompare: false) == 0)
		{
			printFactory._printDocument.DefaultPageSettings.Landscape = true;
		}
		PrintPreviewDialog printPreviewDialog = new PrintPreviewDialog();
		printPreviewDialog.Document = printFactory._printDocument;
		printPreviewDialog.ShowDialog();
		bool result = default(bool);
		return result;
	}

	private static PrintFactory BeginPrint(DataGridView dgv, string userName, string headerText)
	{
		PrintFactory printFactory = new PrintFactory(userName, headerText);
		printFactory._dgv = dgv;
		printFactory.StringFormat = new StringFormat();
		printFactory.StringFormat.Alignment = StringAlignment.Near;
		printFactory.StringFormat.LineAlignment = StringAlignment.Center;
		printFactory.StringFormat.Trimming = StringTrimming.EllipsisCharacter;
		printFactory.StringFormatComboBox = new StringFormat();
		printFactory.StringFormatComboBox.LineAlignment = StringAlignment.Center;
		printFactory.StringFormatComboBox.FormatFlags = StringFormatFlags.NoWrap;
		printFactory.StringFormatComboBox.Trimming = StringTrimming.EllipsisCharacter;
		printFactory.Button = new Button();
		printFactory.CheckBox = new CheckBox();
		printFactory.ComboBox = new ComboBox();
		printFactory.TotalWidth = 0;
		foreach (DataGridViewColumn column in dgv.Columns)
		{
			PrintFactory printFactory2 = printFactory;
			printFactory2.TotalWidth = checked((short)(printFactory2.TotalWidth + column.Width));
		}
		printFactory.PageNumber = 1;
		printFactory.NewPage = true;
		printFactory.RowPos = 0;
		return printFactory;
	}

	private static bool EndPrint(PrintFactory printEntity, PrintPageEventArgs e)
	{
		Monitor.Enter(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init);
		try
		{
			if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init.State == 0)
			{
				_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init.State = 2;
				_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts = new ArrayList();
			}
			else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init.State == 2)
			{
				throw new IncompleteInitialization();
			}
		}
		finally
		{
			_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init.State = 1;
			Monitor.Exit(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts_0024Init);
		}
		Monitor.Enter(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init);
		try
		{
			if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init.State == 0)
			{
				_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init.State = 2;
				_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths = new ArrayList();
			}
			else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init.State == 2)
			{
				throw new IncompleteInitialization();
			}
		}
		finally
		{
			_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init.State = 1;
			Monitor.Exit(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths_0024Init);
		}
		Monitor.Enter(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init);
		try
		{
			if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init.State == 0)
			{
				_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init.State = 2;
				_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes = new ArrayList();
			}
			else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init.State == 2)
			{
				throw new IncompleteInitialization();
			}
		}
		finally
		{
			_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init.State = 1;
			Monitor.Exit(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes_0024Init);
		}
		checked
		{
			short num = (short)e.MarginBounds.Top;
			short num2 = (short)(e.MarginBounds.Left - 60);
			short num3 = (short)(e.MarginBounds.Width + 90);
			if (printEntity.PageNumber == 1)
			{
				foreach (DataGridViewColumn column in printEntity.DGV.Columns)
				{
					if (column.Visible)
					{
						short num4 = (short)Math.Round(Math.Floor((double)column.Width / (double)printEntity.TotalWidth * (double)printEntity.TotalWidth * ((double)num3 / (double)printEntity.TotalWidth)));
						_0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight = (short)Math.Round(e.Graphics.MeasureString(column.HeaderText, column.InheritedStyle.Font, num4).Height + 11f);
						_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts.Add(num2);
						_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths.Add(num4);
						_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes.Add(column.GetType());
						num2 = (short)unchecked(num2 + num4);
					}
				}
			}
			short num6 = default(short);
			bool flag = default(bool);
			string text = default(string);
			while (true)
			{
				if (printEntity.RowPos < printEntity.DGV.Rows.Count)
				{
					DataGridViewRow dataGridViewRow = printEntity.DGV.Rows[printEntity.RowPos];
					if ((short)unchecked(num + _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight) < e.MarginBounds.Height + e.MarginBounds.Top)
					{
						short num5;
						if (printEntity.NewPage)
						{
							e.Graphics.DrawString(printEntity.HeaderText, new Font(printEntity.DGV.Font, FontStyle.Bold), Brushes.Black, e.MarginBounds.Left, (float)e.MarginBounds.Top - e.Graphics.MeasureString(printEntity.HeaderText, new Font(printEntity.DGV.Font, FontStyle.Bold), num3).Height - 13f);
							num = (short)e.MarginBounds.Top;
							num5 = 0;
							foreach (DataGridViewColumn column2 in printEntity.DGV.Columns)
							{
								if (column2.Visible)
								{
									Graphics graphics = e.Graphics;
									SolidBrush brush = new SolidBrush(Color.LightGray);
									Rectangle rect = new Rectangle(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num, Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
									graphics.FillRectangle(brush, rect);
									Graphics graphics2 = e.Graphics;
									Pen black = Pens.Black;
									rect = new Rectangle(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num, Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
									graphics2.DrawRectangle(black, rect);
									Graphics graphics3 = e.Graphics;
									string headerText = column2.HeaderText;
									Font font = column2.InheritedStyle.Font;
									SolidBrush brush2 = new SolidBrush(column2.InheritedStyle.ForeColor);
									RectangleF layoutRectangle = new RectangleF(Conversions.ToSingle(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num, Conversions.ToSingle(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
									graphics3.DrawString(headerText, font, brush2, layoutRectangle, printEntity.StringFormat);
									num5++;
								}
							}
							printEntity.NewPage = false;
						}
						num = (short)unchecked(num + _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
						num5 = 0;
						foreach (DataGridViewCell cell in dataGridViewRow.Cells)
						{
							if (!cell.Visible)
							{
								continue;
							}
							Rectangle rect;
							if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes[num5] == typeof(DataGridViewTextBoxColumn) || ((_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes[num5] == typeof(DataGridViewLinkColumn)) ? true : false))
							{
								Graphics graphics4 = e.Graphics;
								string s = cell.Value.ToString();
								Font font2 = cell.InheritedStyle.Font;
								SolidBrush brush3 = new SolidBrush(cell.InheritedStyle.ForeColor);
								RectangleF layoutRectangle = new RectangleF(Conversions.ToSingle(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num, Conversions.ToSingle(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
								graphics4.DrawString(s, font2, brush3, layoutRectangle, printEntity.StringFormat);
							}
							else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes[num5] == typeof(DataGridViewButtonColumn))
							{
								printEntity.Button.Text = cell.Value.ToString();
								Button button = printEntity.Button;
								Size size = new Size(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
								button.Size = size;
								Bitmap bitmap = new Bitmap(printEntity.Button.Width, printEntity.Button.Height);
								Button button2 = printEntity.Button;
								rect = new Rectangle(0, 0, bitmap.Width, bitmap.Height);
								button2.DrawToBitmap(bitmap, rect);
								Graphics graphics5 = e.Graphics;
								Point point = new Point(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num);
								graphics5.DrawImage(bitmap, point);
							}
							else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes[num5] == typeof(DataGridViewCheckBoxColumn))
							{
								CheckBox checkBox = printEntity.CheckBox;
								Size size = new Size(14, 14);
								checkBox.Size = size;
								printEntity.CheckBox.Checked = Conversions.ToBoolean(cell.Value);
								Bitmap bitmap2 = new Bitmap(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
								Graphics graphics6 = Graphics.FromImage(bitmap2);
								Brush white = Brushes.White;
								rect = new Rectangle(0, 0, bitmap2.Width, bitmap2.Height);
								graphics6.FillRectangle(white, rect);
								CheckBox checkBox2 = printEntity.CheckBox;
								rect = new Rectangle((int)Math.Round((double)(bitmap2.Width - printEntity.CheckBox.Width) / 2.0), (int)Math.Round((double)(bitmap2.Height - printEntity.CheckBox.Height) / 2.0), printEntity.CheckBox.Width, printEntity.CheckBox.Height);
								checkBox2.DrawToBitmap(bitmap2, rect);
								Graphics graphics7 = e.Graphics;
								Point point = new Point(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num);
								graphics7.DrawImage(bitmap2, point);
							}
							else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes[num5] == typeof(DataGridViewComboBoxColumn))
							{
								ComboBox comboBox = printEntity.ComboBox;
								Size size = new Size(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
								comboBox.Size = size;
								Bitmap bitmap3 = new Bitmap(printEntity.ComboBox.Width, printEntity.ComboBox.Height);
								ComboBox comboBox2 = printEntity.ComboBox;
								rect = new Rectangle(0, 0, bitmap3.Width, bitmap3.Height);
								comboBox2.DrawToBitmap(bitmap3, rect);
								Graphics graphics8 = e.Graphics;
								Point point = new Point(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num);
								graphics8.DrawImage(bitmap3, point);
								Graphics graphics9 = e.Graphics;
								string s2 = cell.Value.ToString();
								Font font3 = cell.InheritedStyle.Font;
								SolidBrush brush4 = new SolidBrush(cell.InheritedStyle.ForeColor);
								RectangleF layoutRectangle = new RectangleF(Conversions.ToSingle(Operators.AddObject(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5], 1)), num, Conversions.ToSingle(Operators.SubtractObject(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5], 16)), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
								graphics9.DrawString(s2, font3, brush4, layoutRectangle, printEntity.StringFormatComboBox);
							}
							else if (_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnTypes[num5] == typeof(DataGridViewImageColumn))
							{
								Rectangle rectangle = new Rectangle(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num, Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
								Size size2 = ((Image)cell.Value).Size;
								Graphics graphics10 = e.Graphics;
								object[] array = new object[2];
								DataGridViewCell dataGridViewCell2 = cell;
								array[0] = RuntimeHelpers.GetObjectValue(dataGridViewCell2.Value);
								rect = new Rectangle(Conversions.ToInteger(Operators.AddObject(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5], (int)Math.Round((double)(rectangle.Width - size2.Width) / 2.0))), num + (int)Math.Round((double)(rectangle.Height - size2.Height) / 2.0), ((Image)cell.Value).Width, ((Image)cell.Value).Height);
								array[1] = rect;
								object[] array2 = array;
								bool[] array3 = new bool[2] { true, false };
								NewLateBinding.LateCall(graphics10, null, "DrawImage", array2, null, null, array3, IgnoreReturn: true);
								if (array3[0])
								{
									dataGridViewCell2.Value = RuntimeHelpers.GetObjectValue(array2[0]);
								}
							}
							Graphics graphics11 = e.Graphics;
							Pen black2 = Pens.Black;
							rect = new Rectangle(Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnLefts[num5]), num, Conversions.ToInteger(_0024STATIC_0024EndPrint_00240221282C0128249_0024oColumnWidths[num5]), _0024STATIC_0024EndPrint_00240221282C0128249_0024nHeight);
							graphics11.DrawRectangle(black2, rect);
							num5++;
						}
						printEntity.RowPos++;
						num6++;
						continue;
					}
					DrawFooter(flag, text, printEntity.DGV, printEntity, num6, e);
					printEntity.NewPage = true;
					printEntity.PageNumber++;
					e.HasMorePages = true;
					break;
				}
				DrawFooter(flag, text, printEntity.DGV, printEntity, num6, e);
				e.HasMorePages = false;
				break;
			}
			bool result = default(bool);
			return result;
		}
	}

	private static bool DrawFooter(bool blnGotNumPages, string strNumOfPages, DataGridView dgv, PrintFactory printEntity, int RowsPerPage, PrintPageEventArgs e)
	{
		string text;
		if (!printEntity.blnGotNumPages)
		{
			printEntity.blnGotNumPages = true;
			printEntity.strNumOfPages = Math.Ceiling((double)dgv.Rows.Count / (double)RowsPerPage).ToString();
			text = printEntity.PageNumber + " of " + Math.Ceiling((double)dgv.Rows.Count / (double)RowsPerPage);
		}
		else
		{
			text = printEntity.PageNumber + " of " + printEntity.strNumOfPages;
		}
		checked
		{
			e.Graphics.DrawString(printEntity.UserName, dgv.Font, Brushes.Black, (float)e.MarginBounds.Left + ((float)e.MarginBounds.Width - e.Graphics.MeasureString(printEntity.UserName, dgv.Font, e.MarginBounds.Width).Width), e.MarginBounds.Top + e.MarginBounds.Height + 31);
			e.Graphics.DrawString(DateAndTime.Now.ToLongDateString() + " " + DateAndTime.Now.ToShortTimeString(), dgv.Font, Brushes.Black, e.MarginBounds.Left, e.MarginBounds.Top + e.MarginBounds.Height + 31);
			e.Graphics.DrawString(text, dgv.Font, Brushes.Black, (float)e.MarginBounds.Left + ((float)e.MarginBounds.Width - e.Graphics.MeasureString(text, dgv.Font, e.MarginBounds.Width).Width) / 2f, e.MarginBounds.Top + e.MarginBounds.Height + 31);
			bool result = default(bool);
			return result;
		}
	}

	private static void Validate(DataGridView dgv, string userName, string headerText)
	{
		if (dgv == null)
		{
			throw new ArgumentNullException("dgv");
		}
		if (string.IsNullOrEmpty(userName))
		{
			throw new ArgumentNullException("userName");
		}
		if (string.IsNullOrEmpty(headerText))
		{
			throw new ArgumentNullException("headerText");
		}
	}
}
