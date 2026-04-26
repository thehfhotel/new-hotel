using System.Text;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[StandardModule]
internal sealed class DecimalToText_TH
{
	private static string[] suffix;

	private static string[] numSpeak;

	static DecimalToText_TH()
	{
		Class2.LH6iGfYz9j3MJ();
		suffix = new string[8] { "", "", "ส\u0e34บ", "ร\u0e49อย", "พ\u0e31น", "หม\u0e37\u0e48น", "แสน", "ล\u0e49าน" };
		numSpeak = new string[10] { "", "หน\u0e36\u0e48ง", "สอง", "สาม", "ส\u0e35\u0e48", "ห\u0e49า", "หก", "เจ\u0e47ด", "แปด", "เก\u0e49า" };
	}

	public static string ThaiBahtText(double m)
	{
		string s = "";
		string s2 = "";
		string s3 = "";
		StringBuilder stringBuilder = new StringBuilder();
		if (m == 0.0)
		{
			return "ศ\u0e39นย\u0e4cบาท";
		}
		splitCurr(m, ref s, ref s2, ref s3);
		if (s.Length > 0)
		{
			stringBuilder.Append(Speak(s) + "ล\u0e49าน");
		}
		if (s2.Length > 0)
		{
			stringBuilder.Append(Speak(s2) + "บาท");
		}
		if (s3.Length > 0)
		{
			stringBuilder.Append(speakStang(s3) + "สตางค\u0e4c");
		}
		else
		{
			stringBuilder.Append("ถ\u0e49วน");
		}
		return stringBuilder.ToString();
	}

	private static string Speak(string s)
	{
		StringBuilder stringBuilder = new StringBuilder();
		if (s.Length == 0)
		{
			return "";
		}
		int length = s.Length;
		int num = length;
		int num2 = 1;
		checked
		{
			while (true)
			{
				int num3 = num2;
				int num4 = num;
				if (num3 <= num4)
				{
					if (Operators.CompareString(Conversions.ToString(s[num2 - 1]), "-", TextCompare: false) == 0)
					{
						stringBuilder.Append("ต\u0e34ดลบ");
					}
					else
					{
						int num5 = Conversion.Val(s[num2 - 1]);
						unchecked
						{
							if (num2 == length && num5 == 1)
							{
								if (length == 1)
								{
									break;
								}
								if ((length > 1) & (Operators.CompareString(Conversions.ToString(s[checked(length - 2)]), "0", TextCompare: false) == 0))
								{
									stringBuilder.Append("หน\u0e36\u0e48ง");
								}
								else
								{
									stringBuilder.Append("เอ\u0e47ด");
								}
							}
							else if (num2 == checked(length - 1) && num5 == 2)
							{
								stringBuilder.Append("ย\u0e35\u0e48ส\u0e34บ");
							}
							else if (num2 == checked(length - 1) && num5 == 1)
							{
								stringBuilder.Append("ส\u0e34บ");
							}
							else if (num5 != 0)
							{
								stringBuilder.Append(numSpeak[num5] + suffix[checked(length - num2 + 1)]);
							}
						}
					}
					num2++;
					continue;
				}
				return stringBuilder.ToString();
			}
			return "หน\u0e36\u0e48ง";
		}
	}

	private static string speakStang(string s)
	{
		StringBuilder stringBuilder = new StringBuilder();
		int num = s.Length;
		switch (num)
		{
		case 0:
			return "";
		case 1:
			s += "0";
			num = 2;
			break;
		}
		if (num > 2)
		{
			s = s.Substring(0, 2);
			num = 2;
		}
		int num2 = 1;
		do
		{
			int num3 = Conversion.Val(s[checked(num2 - 1)]);
			if (!(num2 == num && num3 == 1))
			{
				if (num2 == checked(num - 1) && num3 == 2)
				{
					stringBuilder.Append("ย\u0e35\u0e48ส\u0e34บ");
				}
				else if (num2 == checked(num - 1) && num3 == 1)
				{
					stringBuilder.Append("ส\u0e34บ");
				}
				else if (num3 != 0)
				{
					stringBuilder.Append(numSpeak[num3] + suffix[checked(2 - num2 + 1)]);
				}
			}
			else if (Conversions.ToInteger(Strings.Mid(s, 1, 1)) == 0)
			{
				stringBuilder.Append("หน\u0e36\u0e48ง");
			}
			else
			{
				stringBuilder.Append("เอ\u0e47ด");
			}
			num2 = checked(num2 + 1);
		}
		while (num2 <= 2);
		return stringBuilder.ToString();
	}

	private static void splitCurr(double m, ref string s1, ref string s2, ref string s3)
	{
		string text = Conversions.ToString(m);
		checked
		{
			int num = text.IndexOf(".") + 1;
			if (num != 0)
			{
				s1 = text.Substring(0, num - 1);
				s3 = text.Substring(num);
				if (Operators.CompareString(s3, "00", TextCompare: false) == 0)
				{
					s3 = "";
				}
			}
			else
			{
				s1 = text;
				s3 = "";
			}
			int length = s1.Length;
			if (length > 6)
			{
				s2 = s1.Substring(length - 5 - 1);
				s1 = s1.Substring(0, length - 6);
			}
			else
			{
				s2 = s1;
				s1 = "";
			}
			if (!Versioned.IsNumeric(s1))
			{
				s1 = "";
			}
			if (!Versioned.IsNumeric(s2))
			{
				s2 = "";
			}
			if (Conversion.Val(s1) == 0.0)
			{
				s1 = "";
			}
			if (Conversion.Val(s2) == 0.0)
			{
				s2 = "";
			}
		}
	}
}
