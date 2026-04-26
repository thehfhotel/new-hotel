using System;
using System.IO;
using System.Security.Cryptography;
using Microsoft.VisualBasic;
using Microsoft.VisualBasic.CompilerServices;

namespace iHOTEL2025;

[StandardModule]
internal sealed class Encrypt
{
	public enum CryptoAction
	{
		ActionEncrypt = 1,
		ActionDecrypt
	}

	public static string strFileToEncrypt;

	public static string strFileToDecrypt;

	public static string strOutputEncrypt;

	public static string strOutputDecrypt;

	public static FileStream fsInput;

	public static FileStream fsOutput;

	public static byte[] CreateKey(string strPassword)
	{
		char[] array = strPassword.ToCharArray();
		int upperBound = array.GetUpperBound(0);
		checked
		{
			byte[] array2 = new byte[upperBound + 1];
			int upperBound2 = array.GetUpperBound(0);
			int num = 0;
			while (true)
			{
				int num2 = num;
				int num3 = upperBound2;
				if (num2 > num3)
				{
					break;
				}
				array2[num] = (byte)Strings.Asc(array[num]);
				num++;
			}
			SHA512Managed sHA512Managed = new SHA512Managed();
			byte[] array3 = sHA512Managed.ComputeHash(array2);
			byte[] array4 = new byte[32];
			int num4 = 0;
			int num5;
			do
			{
				array4[num4] = array3[num4];
				num4++;
				num5 = num4;
				int num3 = 31;
			}
			while (num5 <= 31);
			return array4;
		}
	}

	public static byte[] CreateIV(string strPassword)
	{
		char[] array = strPassword.ToCharArray();
		int upperBound = array.GetUpperBound(0);
		checked
		{
			byte[] array2 = new byte[upperBound + 1];
			int upperBound2 = array.GetUpperBound(0);
			int num = 0;
			while (true)
			{
				int num2 = num;
				int num3 = upperBound2;
				if (num2 > num3)
				{
					break;
				}
				array2[num] = (byte)Strings.Asc(array[num]);
				num++;
			}
			SHA512Managed sHA512Managed = new SHA512Managed();
			byte[] array3 = sHA512Managed.ComputeHash(array2);
			byte[] array4 = new byte[16];
			int num4 = 32;
			int num5;
			do
			{
				array4[num4 - 32] = array3[num4];
				num4++;
				num5 = num4;
				int num3 = 47;
			}
			while (num5 <= 47);
			return array4;
		}
	}

	public static void EncryptOrDecryptFile(string strInputFile, string strOutputFile, byte[] bytKey, byte[] bytIV, CryptoAction Direction)
	{
		try
		{
			fsInput = new FileStream(strInputFile, FileMode.Open, FileAccess.Read);
			fsOutput = new FileStream(strOutputFile, FileMode.OpenOrCreate, FileAccess.Write);
			fsOutput.SetLength(0L);
			byte[] array = new byte[4097];
			long num = 0L;
			long length = fsInput.Length;
			RijndaelManaged rijndaelManaged = new RijndaelManaged();
			CryptoStream cryptoStream = default(CryptoStream);
			switch ((int)Direction)
			{
			case 1:
				cryptoStream = new CryptoStream(fsOutput, rijndaelManaged.CreateEncryptor(bytKey, bytIV), CryptoStreamMode.Write);
				break;
			case 2:
				cryptoStream = new CryptoStream(fsOutput, rijndaelManaged.CreateDecryptor(bytKey, bytIV), CryptoStreamMode.Write);
				break;
			}
			int num2;
			for (; num < length; num = checked(num + num2))
			{
				num2 = fsInput.Read(array, 0, 4096);
				cryptoStream.Write(array, 0, num2);
			}
			cryptoStream.Close();
			fsInput.Close();
			fsOutput.Close();
			if (Direction == CryptoAction.ActionEncrypt)
			{
				FileInfo fileInfo = new FileInfo(strFileToEncrypt);
				fileInfo.Delete();
			}
			if (Direction == CryptoAction.ActionDecrypt)
			{
				FileInfo fileInfo2 = new FileInfo(strFileToDecrypt);
				fileInfo2.Delete();
			}
			if (Direction == CryptoAction.ActionEncrypt)
			{
			}
		}
		catch (Exception projectError) when (((Func<bool>)delegate
		{
			// Could not convert BlockContainer to single expression
			ProjectData.SetProjectError(projectError);
			return Information.Err().Number == 53;
		}).Invoke())
		{
			Interaction.MsgBox("Please check to make sure the path and filenameare correct and if the file exists.", MsgBoxStyle.Exclamation, "Invalid Path or Filename");
			ProjectData.ClearProjectError();
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			fsInput.Close();
			fsOutput.Close();
			ProjectData.ClearProjectError();
		}
	}

	public static void ReadSerials(string strInputFile, string strOutputFile, byte[] bytKey, byte[] bytIV, CryptoAction Direction)
	{
		try
		{
			fsInput = new FileStream(strInputFile, FileMode.Open, FileAccess.Read);
			fsOutput = new FileStream(strOutputFile, FileMode.OpenOrCreate, FileAccess.Write);
			fsOutput.SetLength(0L);
			byte[] array = new byte[4097];
			long num = 0L;
			long length = fsInput.Length;
			RijndaelManaged rijndaelManaged = new RijndaelManaged();
			CryptoStream cryptoStream = default(CryptoStream);
			switch ((int)Direction)
			{
			case 1:
				cryptoStream = new CryptoStream(fsOutput, rijndaelManaged.CreateEncryptor(bytKey, bytIV), CryptoStreamMode.Write);
				break;
			case 2:
				cryptoStream = new CryptoStream(fsOutput, rijndaelManaged.CreateDecryptor(bytKey, bytIV), CryptoStreamMode.Write);
				break;
			}
			int num2;
			for (; num < length; num = checked(num + num2))
			{
				num2 = fsInput.Read(array, 0, 4096);
				cryptoStream.Write(array, 0, num2);
			}
			cryptoStream.Close();
			fsInput.Close();
			fsOutput.Close();
			if (Direction == CryptoAction.ActionEncrypt)
			{
			}
		}
		catch (Exception projectError) when (((Func<bool>)delegate
		{
			// Could not convert BlockContainer to single expression
			ProjectData.SetProjectError(projectError);
			return Information.Err().Number == 53;
		}).Invoke())
		{
			Interaction.MsgBox("Please check to make sure the path and filenameare correct and if the file exists.", MsgBoxStyle.Exclamation, "Invalid Path or Filename");
			ProjectData.ClearProjectError();
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			fsInput.Close();
			fsOutput.Close();
			if (Direction == CryptoAction.ActionDecrypt)
			{
				Interaction.MsgBox("กร\u0e38ณาเชคว\u0e48าไฟล\u0e4cท\u0e35\u0e48เร\u0e35ยกมาถ\u0e39กต\u0e49องหร\u0e37อไม\u0e48", MsgBoxStyle.Exclamation, "การเร\u0e35ยกค\u0e37นผ\u0e34ดพลาด");
			}
			ProjectData.ClearProjectError();
		}
	}

	public static bool EncryptOrDecryptFile2(string strInputFile, string strOutputFile, byte[] bytKey, byte[] bytIV, CryptoAction Direction)
	{
		bool result;
		try
		{
			fsInput = new FileStream(strInputFile, FileMode.Open, FileAccess.Read);
			fsOutput = new FileStream(strOutputFile, FileMode.OpenOrCreate, FileAccess.Write);
			fsOutput.SetLength(0L);
			byte[] array = new byte[4097];
			long num = 0L;
			long length = fsInput.Length;
			RijndaelManaged rijndaelManaged = new RijndaelManaged();
			CryptoStream cryptoStream = default(CryptoStream);
			switch ((int)Direction)
			{
			case 1:
				cryptoStream = new CryptoStream(fsOutput, rijndaelManaged.CreateEncryptor(bytKey, bytIV), CryptoStreamMode.Write);
				break;
			case 2:
				cryptoStream = new CryptoStream(fsOutput, rijndaelManaged.CreateDecryptor(bytKey, bytIV), CryptoStreamMode.Write);
				break;
			}
			int num2;
			for (; num < length; num = checked(num + num2))
			{
				num2 = fsInput.Read(array, 0, 4096);
				cryptoStream.Write(array, 0, num2);
			}
			cryptoStream.Close();
			fsInput.Close();
			fsOutput.Close();
			if (Direction == CryptoAction.ActionEncrypt)
			{
				FileInfo fileInfo = new FileInfo(strFileToEncrypt);
				fileInfo.Delete();
			}
			if (Direction == CryptoAction.ActionEncrypt)
			{
			}
		}
		catch (Exception projectError) when (((Func<bool>)delegate
		{
			// Could not convert BlockContainer to single expression
			ProjectData.SetProjectError(projectError);
			return Information.Err().Number == 53;
		}).Invoke())
		{
			Interaction.MsgBox("Please check to make sure the path and filenameare correct and if the file exists.", MsgBoxStyle.Exclamation, "Invalid Path or Filename");
			ProjectData.ClearProjectError();
		}
		catch (Exception projectError2)
		{
			ProjectData.SetProjectError(projectError2);
			fsInput.Close();
			fsOutput.Close();
			if (Direction == CryptoAction.ActionDecrypt)
			{
				Interaction.MsgBox("กร\u0e38ณาเชคว\u0e48าไฟล\u0e4cท\u0e35\u0e48เร\u0e35ยกมาถ\u0e39กต\u0e49องหร\u0e37อไม\u0e48", MsgBoxStyle.Exclamation, "การเร\u0e35ยกค\u0e37นผ\u0e34ดพลาด");
			}
			result = false;
			ProjectData.ClearProjectError();
			goto IL_0184;
		}
		result = true;
		goto IL_0184;
		IL_0184:
		return result;
	}
}
