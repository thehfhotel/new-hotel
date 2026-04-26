using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Runtime.CompilerServices;
using System.Security.Cryptography;
using System.Text;
using System.Windows.Forms;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[DesignerGenerated]
public class FormEN_DE : Form
{
	private static List<WeakReference> __ENCList;

	private IContainer components;

	[AccessedThroughProperty("Label1")]
	private Label _Label1;

	[AccessedThroughProperty("Label2")]
	private Label _Label2;

	[AccessedThroughProperty("ำยระกผห")]
	private TextBox textBox_0;

	[AccessedThroughProperty("eoidfjkl")]
	private TextBox _eoidfjkl;

	[AccessedThroughProperty("Button1")]
	private Button _Button1;

	[AccessedThroughProperty("Label3")]
	private Label _Label3;

	[AccessedThroughProperty("ยรำดdggfoาดนเเห")]
	private TextBox textBox_1;

	private static TripleDESCryptoServiceProvider DES;

	private static MD5CryptoServiceProvider MD5;

	internal virtual Label Label1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label1 = value;
		}
	}

	internal virtual Label Label2
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label2;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label2 = value;
		}
	}

	internal virtual TextBox TextBox_0
	{
		[DebuggerNonUserCode]
		get
		{
			return textBox_0;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBox_0 = value;
		}
	}

	internal virtual TextBox eoidfjkl
	{
		[DebuggerNonUserCode]
		get
		{
			return _eoidfjkl;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_eoidfjkl = value;
		}
	}

	internal virtual Button Button1
	{
		[DebuggerNonUserCode]
		get
		{
			return _Button1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			EventHandler value2 = Button1_Click;
			if (_Button1 != null)
			{
				_Button1.Click -= value2;
			}
			_Button1 = value;
			if (_Button1 != null)
			{
				_Button1.Click += value2;
			}
		}
	}

	internal virtual Label Label3
	{
		[DebuggerNonUserCode]
		get
		{
			return _Label3;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			_Label3 = value;
		}
	}

	internal virtual TextBox TextBox_1
	{
		[DebuggerNonUserCode]
		get
		{
			return textBox_1;
		}
		[MethodImpl(MethodImplOptions.Synchronized)]
		[DebuggerNonUserCode]
		set
		{
			textBox_1 = value;
		}
	}

	[DebuggerNonUserCode]
	public FormEN_DE()
	{
		Class2.LH6iGfYz9j3MJ();
		// base._002Ector();  // REF: stripped — see _ReferenceStubs.cs
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		InitializeComponent();
	}

	[DebuggerNonUserCode]
	protected override void Dispose(bool disposing)
	{
		try
		{
			if (disposing && components != null)
			{
				components.Dispose();
			}
		}
		finally
		{
			base.Dispose(disposing);
		}
	}

	[System.Diagnostics.DebuggerStepThrough]
	private void InitializeComponent()
	{
		this.Label1 = new System.Windows.Forms.Label();
		this.Label2 = new System.Windows.Forms.Label();
		this.TextBox_0 = new System.Windows.Forms.TextBox();
		this.eoidfjkl = new System.Windows.Forms.TextBox();
		this.Button1 = new System.Windows.Forms.Button();
		this.Label3 = new System.Windows.Forms.Label();
		this.TextBox_1 = new System.Windows.Forms.TextBox();
		this.SuspendLayout();
		this.Label1.AutoSize = true;
		System.Windows.Forms.Label label = this.Label1;
		System.Drawing.Point location = new System.Drawing.Point(111, 16);
		label.Location = location;
		System.Windows.Forms.Label label2 = this.Label1;
		System.Windows.Forms.Padding margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label2.Margin = margin;
		this.Label1.Name = "Label1";
		System.Windows.Forms.Label label3 = this.Label1;
		System.Drawing.Size size = new System.Drawing.Size(107, 19);
		label3.Size = size;
		this.Label1.TabIndex = 0;
		this.Label1.Text = "ท\u0e35\u0e48อย\u0e39\u0e48เซ\u0e34ฟเวอร\u0e4c :";
		this.Label2.AutoSize = true;
		System.Windows.Forms.Label label4 = this.Label2;
		location = new System.Drawing.Point(86, 52);
		label4.Location = location;
		System.Windows.Forms.Label label5 = this.Label2;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label5.Margin = margin;
		this.Label2.Name = "Label2";
		System.Windows.Forms.Label label6 = this.Label2;
		size = new System.Drawing.Size(132, 19);
		label6.Size = size;
		this.Label2.TabIndex = 1;
		this.Label2.Text = "รห\u0e31สผ\u0e48านเซ\u0e34ฟเวอร\u0e4c :";
		System.Windows.Forms.TextBox textBox = this.TextBox_0;
		location = new System.Drawing.Point(220, 12);
		textBox.Location = location;
		System.Windows.Forms.TextBox textBox2 = this.TextBox_0;
		margin = new System.Windows.Forms.Padding(4);
		textBox2.Margin = margin;
		this.TextBox_0.Name = "ำยระกผห";
		System.Windows.Forms.TextBox textBox3 = this.TextBox_0;
		size = new System.Drawing.Size(314, 27);
		textBox3.Size = size;
		this.TextBox_0.TabIndex = 2;
		System.Windows.Forms.TextBox textBox4 = this.eoidfjkl;
		location = new System.Drawing.Point(220, 48);
		textBox4.Location = location;
		System.Windows.Forms.TextBox textBox5 = this.eoidfjkl;
		margin = new System.Windows.Forms.Padding(4);
		textBox5.Margin = margin;
		this.eoidfjkl.Name = "eoidfjkl";
		System.Windows.Forms.TextBox textBox6 = this.eoidfjkl;
		size = new System.Drawing.Size(314, 27);
		textBox6.Size = size;
		this.eoidfjkl.TabIndex = 3;
		System.Windows.Forms.Button button = this.Button1;
		location = new System.Drawing.Point(541, 12);
		button.Location = location;
		this.Button1.Name = "Button1";
		System.Windows.Forms.Button button2 = this.Button1;
		size = new System.Drawing.Size(106, 63);
		button2.Size = size;
		this.Button1.TabIndex = 4;
		this.Button1.Text = "สร\u0e49างรห\u0e31ส";
		this.Button1.UseVisualStyleBackColor = true;
		this.Label3.AutoSize = true;
		System.Windows.Forms.Label label7 = this.Label3;
		location = new System.Drawing.Point(14, 89);
		label7.Location = location;
		System.Windows.Forms.Label label8 = this.Label3;
		margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
		label8.Margin = margin;
		this.Label3.Name = "Label3";
		System.Windows.Forms.Label label9 = this.Label3;
		size = new System.Drawing.Size(204, 19);
		label9.Size = size;
		this.Label3.TabIndex = 5;
		this.Label3.Text = "รห\u0e31สสำหร\u0e31บนำไปใส\u0e48เคร\u0e37\u0e48องล\u0e39ก :";
		this.TextBox_1.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
		this.TextBox_1.Font = new System.Drawing.Font("Consolas", 15.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 0);
		this.TextBox_1.ForeColor = System.Drawing.Color.Black;
		System.Windows.Forms.TextBox textBox7 = this.TextBox_1;
		location = new System.Drawing.Point(220, 83);
		textBox7.Location = location;
		System.Windows.Forms.TextBox textBox8 = this.TextBox_1;
		margin = new System.Windows.Forms.Padding(4);
		textBox8.Margin = margin;
		this.TextBox_1.Multiline = true;
		this.TextBox_1.Name = "ยรำดdggfoาดนเเห";
		this.TextBox_1.ReadOnly = true;
		System.Windows.Forms.TextBox textBox9 = this.TextBox_1;
		size = new System.Drawing.Size(427, 177);
		textBox9.Size = size;
		this.TextBox_1.TabIndex = 6;
		System.Drawing.SizeF sizeF = new System.Drawing.SizeF(9f, 19f);
		this.AutoScaleDimensions = sizeF;
		this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
		size = new System.Drawing.Size(672, 273);
		this.ClientSize = size;
		this.Controls.Add(this.TextBox_1);
		this.Controls.Add(this.Label3);
		this.Controls.Add(this.Button1);
		this.Controls.Add(this.eoidfjkl);
		this.Controls.Add(this.TextBox_0);
		this.Controls.Add(this.Label2);
		this.Controls.Add(this.Label1);
		this.Font = new System.Drawing.Font("Tahoma", 12f);
		margin = new System.Windows.Forms.Padding(4, 5, 4, 5);
		this.Margin = margin;
		this.MaximizeBox = false;
		this.MinimizeBox = false;
		this.Name = "FormEN_DE";
		this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
		this.Text = "สร\u0e49างรห\u0e31สสำหร\u0e31บฐานข\u0e49อม\u0e39ล";
		this.ResumeLayout(false);
		this.PerformLayout();
	}

	static FormEN_DE()
	{
		Class2.LH6iGfYz9j3MJ();
		__ENCList = new List<WeakReference>();
		DES = new TripleDESCryptoServiceProvider();
		MD5 = new MD5CryptoServiceProvider();
	}

	public static byte[] MD5Hash(string value)
	{
		return MD5.ComputeHash(Encoding.ASCII.GetBytes(value));
	}

	public static string Encrypt1(string stringToEncrypt, string key)
	{
		DES.Key = MD5Hash(key);
		DES.Mode = CipherMode.ECB;
		byte[] bytes = Encoding.ASCII.GetBytes(stringToEncrypt);
		return Convert.ToBase64String(DES.CreateEncryptor().TransformFinalBlock(bytes, 0, bytes.Length));
	}

	public static string Decrypt1(string encryptedString, string key)
	{
		string result;
		try
		{
			DES.Key = MD5Hash(key);
			DES.Mode = CipherMode.ECB;
			byte[] array = Convert.FromBase64String(encryptedString);
			result = Encoding.ASCII.GetString(DES.CreateDecryptor().TransformFinalBlock(array, 0, array.Length));
		}
		catch (Exception projectError)
		{
			ProjectData.SetProjectError(projectError);
			result = "ERRORR";
			ProjectData.ClearProjectError();
		}
		return result;
	}

	public static string Encrypt(string plainText, string passPhrase, string saltValue, string hashAlgorithm, int passwordIterations, string initVector, int keySize)
	{
		byte[] bytes = Encoding.ASCII.GetBytes(initVector);
		byte[] bytes2 = Encoding.ASCII.GetBytes(saltValue);
		byte[] bytes3 = Encoding.UTF8.GetBytes(plainText);
		PasswordDeriveBytes passwordDeriveBytes = new PasswordDeriveBytes(passPhrase, bytes2, hashAlgorithm, passwordIterations);
		byte[] bytes4 = passwordDeriveBytes.GetBytes(checked((int)Math.Round((double)keySize / 8.0)));
		RijndaelManaged rijndaelManaged = new RijndaelManaged();
		rijndaelManaged.Mode = CipherMode.CBC;
		ICryptoTransform transform = rijndaelManaged.CreateEncryptor(bytes4, bytes);
		MemoryStream memoryStream = new MemoryStream();
		CryptoStream cryptoStream = new CryptoStream(memoryStream, transform, CryptoStreamMode.Write);
		cryptoStream.Write(bytes3, 0, bytes3.Length);
		cryptoStream.FlushFinalBlock();
		byte[] inArray = memoryStream.ToArray();
		memoryStream.Close();
		cryptoStream.Close();
		return Convert.ToBase64String(inArray);
	}

	public static string Decrypt(string cipherText, string passPhrase, string saltValue, string hashAlgorithm, int passwordIterations, string initVector, int keySize)
	{
		byte[] bytes = Encoding.ASCII.GetBytes(initVector);
		byte[] bytes2 = Encoding.ASCII.GetBytes(saltValue);
		byte[] array = Convert.FromBase64String(cipherText);
		PasswordDeriveBytes passwordDeriveBytes = new PasswordDeriveBytes(passPhrase, bytes2, hashAlgorithm, passwordIterations);
		checked
		{
			byte[] bytes3 = passwordDeriveBytes.GetBytes((int)Math.Round((double)keySize / 8.0));
			RijndaelManaged rijndaelManaged = new RijndaelManaged();
			rijndaelManaged.Mode = CipherMode.CBC;
			ICryptoTransform transform = rijndaelManaged.CreateDecryptor(bytes3, bytes);
			MemoryStream memoryStream = new MemoryStream(array);
			CryptoStream cryptoStream = new CryptoStream(memoryStream, transform, CryptoStreamMode.Read);
			byte[] array2 = new byte[array.Length + 1];
			int count = cryptoStream.Read(array2, 0, array2.Length);
			memoryStream.Close();
			cryptoStream.Close();
			return Encoding.UTF8.GetString(array2, 0, count);
		}
	}

	public string method_0(string string_0, int int_0)
	{
		int num = Strings.Len(string_0);
		int num2 = 1;
		string text = default(string);
		while (true)
		{
			int num3 = num2;
			int num4 = num;
			if (num3 > num4)
			{
				break;
			}
			text += Conversions.ToString(Strings.Chr(Strings.Asc(Strings.Mid(string_0, num2, 1)) ^ int_0));
			num2 = checked(num2 + 1);
		}
		return text;
	}

	public string method_1(string string_0, int int_0)
	{
		int num = Strings.Len(string_0);
		int num2 = 1;
		string text = default(string);
		while (true)
		{
			int num3 = num2;
			int num4 = num;
			if (num3 > num4)
			{
				break;
			}
			text += Conversions.ToString(Strings.Chr(Strings.Asc(Strings.Mid(string_0, num2, 1)) ^ int_0));
			num2 = checked(num2 + 1);
		}
		return text;
	}

	private void Button1_Click(object sender, EventArgs e)
	{
		TextBox_1.Text = Encrypt1(TextBox_0.Text + "|" + eoidfjkl.Text, "LOTTO");
		_ = TextBox_0.Text + "|" + eoidfjkl.Text;
		int num = 0;
		string text = "";
		checked
		{
			int num2 = TextBox_1.Text.Length - 1;
			int num3 = 0;
			while (true)
			{
				int num4 = num3;
				int num5 = num2;
				if (num4 > num5)
				{
					break;
				}
				num++;
				text += Conversions.ToString(TextBox_1.Text[num3]);
				if (num == 5)
				{
					num = 0;
					text += "-";
				}
				num3++;
			}
			TextBox_1.Text = text;
		}
	}

	private void Button2_Click(object sender, EventArgs e)
	{
		MessageBox.Show(Decrypt1(TextBox_1.Text.Replace(" ", "").Replace("-", ""), "LOTTO"));
	}
}
