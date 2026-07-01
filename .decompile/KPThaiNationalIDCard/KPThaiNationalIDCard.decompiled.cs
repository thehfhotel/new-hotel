using System;
using System.CodeDom.Compiler;
using System.Collections;
using System.ComponentModel;
using System.Configuration;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.Globalization;
using System.IO;
using System.Reflection;
using System.Resources;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using System.Security.Cryptography;
using System.Text;
using System.Windows.Forms;
using KPNationalIDCard.Example.Properties;
using hU4x3GePSuIEn9q1kR;
using q3uifC6dT931xg0UY6;

[assembly: Guid("14ef7674-685c-4646-9a5b-a830aedb06c7")]
[assembly: AssemblyFileVersion("1.0.0.0")]
[assembly: TargetFramework(".NETFramework,Version=v4.5", FrameworkDisplayName = ".NET Framework 4.5")]
[assembly: ComVisible(false)]
[assembly: AssemblyKeyName("")]
[assembly: AssemblyDelaySign(false)]
[assembly: Debuggable(DebuggableAttribute.DebuggingModes.Default | DebuggableAttribute.DebuggingModes.DisableOptimizations | DebuggableAttribute.DebuggingModes.IgnoreSymbolStoreSequencePoints | DebuggableAttribute.DebuggingModes.EnableEditAndContinue)]
[assembly: SuppressIldasm]
[assembly: AssemblyCompany("บร\u0e34ษ\u0e31ท เคพ\u0e35ซ\u0e34สเต\u0e47มแอนด\u0e4cเน\u0e47ตเว\u0e34ร\u0e4cค จำก\u0e31ด")]
[assembly: RuntimeCompatibility(WrapNonExceptionThrows = true)]
[assembly: AssemblyDescription("http://www.kpsystem.co.th")]
[assembly: AssemblyConfiguration("")]
[assembly: AssemblyTrademark("")]
[assembly: AssemblyProduct("KP ThaiNationalIDCard")]
[assembly: AssemblyCopyright("Copyright ©  2017")]
[assembly: CompilationRelaxations(8)]
[assembly: AssemblyTitle("KP SmartCard Reader")]
[assembly: AssemblyVersion("1.0.0.0")]
internal static class xCtlovwwBPh7AJeZh4
{
}
namespace ThaiNationalIDCard.Example
{
	public class frmMain : Form
	{
		private ThaiIDCard idcard;

		private IContainer components;

		private Label label13;

		private Label lbl_issue;

		private Label label11;

		private Button btnRefreshReaderList;

		private CheckBox chkBoxMonitor;

		private ComboBox cbxReaderList;

		private Button btnReadWithPhoto;

		private ProgressBar PhotoProgressBar1;

		private PictureBox pictureBox1;

		private ImageList imageList1;

		private Label lbl_expire;

		private Label lbl_sex;

		private Label label9;

		private Label lbl_birthday;

		private Label lbl_en_lastname;

		private Label lbl_en_firstname;

		private Label lbl_en_prefix;

		private Label lbl_th_lastname;

		private Label lbl_th_firstname;

		private Label lbl_th_prefix;

		private Label label8;

		private Label label7;

		private Label label6;

		private Label label5;

		private Label label4;

		private Label label3;

		private Label label2;

		private Label lbl_cid;

		private Label label1;

		private Timer timer1;

		private Panel panel1;

		private Label label10;

		private Button button1;

		private Button button2;

		private Label lbl_address;

		private Label label15;

		private Label label_no;

		private Label label14;

		private Label label_moo;

		private Label label17;

		private Label label_soi;

		private Label label19;

		private Label label_road;

		private Label label21;

		private Label label_tanbon;

		private Label label23;

		private Label label_ampore;

		private Label label25;

		private Label label_province;

		private Label label27;

		private Label citizenID;

		private Label label16;

		private Label label12;

		private Label label18;

		private PictureBox pictureBox2;

		private Panel panel2;

		private Label label20;

		private CheckBox checkBox_pic;

		private CheckBox checkBox_read;

		private Label label24;

		private Label label22;

		[MethodImpl(MethodImplOptions.NoInlining)]
		public frmMain()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			components = null;
			base..ctor();
			InitializeComponent();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void btnRead_Click(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				lbl_cid.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(0);
				Refresh();
				Personal personal = idcard.readAll();
				if (personal != null)
				{
					lbl_cid.Text = personal.Citizenid;
					lbl_birthday.Text = personal.Birthday.ToString(KYJngMCOCJNSf8T7gH.Ushn1vyok(24));
					lbl_sex.Text = personal.Sex;
					lbl_th_prefix.Text = personal.Th_Prefix;
					lbl_th_firstname.Text = personal.Th_Firstname;
					lbl_th_lastname.Text = personal.Th_Lastname;
					lbl_en_prefix.Text = personal.En_Prefix;
					lbl_en_firstname.Text = personal.En_Firstname;
					lbl_en_lastname.Text = personal.En_Lastname;
					lbl_issue.Text = personal.Issue.ToString(KYJngMCOCJNSf8T7gH.Ushn1vyok(24));
					lbl_expire.Text = personal.Expire.ToString(KYJngMCOCJNSf8T7gH.Ushn1vyok(24));
					panel1.Visible = false;
				}
				else if (idcard.ErrorCode() > 0)
				{
					MessageBox.Show(KYJngMCOCJNSf8T7gH.Ushn1vyok(48));
					panel1.Visible = false;
				}
			}
			catch (Exception)
			{
				MessageBox.Show(KYJngMCOCJNSf8T7gH.Ushn1vyok(48));
				panel1.Visible = false;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void photoProgress(int value, int maximum)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (PhotoProgressBar1.Maximum != maximum)
			{
				PhotoProgressBar1.Maximum = maximum;
			}
			if (PhotoProgressBar1.Maximum > value)
			{
				PhotoProgressBar1.Value = value + 1;
			}
			PhotoProgressBar1.Value = value;
			Refresh();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public void CardInserted(Personal personal)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			btnReadWithPhoto_Click_1(null, null);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void btnReadWithPhoto_Click_1(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			idcard = new ThaiIDCard();
			pictureBox1.Image = null;
			label10.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(142);
			PhotoProgressBar1.Visible = true;
			btnReadWithPhoto.Visible = false;
			PhotoProgressBar1.Value = 0;
			panel1.Visible = true;
			lbl_cid.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(0);
			Refresh();
			idcard.eventPhotoProgress += photoProgress;
			Personal personal = idcard.readAllPhoto();
			if (personal != null)
			{
				lbl_cid.Text = personal.Citizenid;
				lbl_birthday.Text = personal.Birthday.ToString(KYJngMCOCJNSf8T7gH.Ushn1vyok(24));
				lbl_sex.Text = personal.Sex;
				lbl_th_prefix.Text = personal.Th_Prefix;
				lbl_th_firstname.Text = personal.Th_Firstname.Trim();
				lbl_th_lastname.Text = personal.Th_Lastname.Trim();
				lbl_en_prefix.Text = personal.En_Prefix.Trim();
				lbl_en_firstname.Text = personal.En_Firstname.Trim();
				lbl_en_lastname.Text = personal.En_Lastname.Trim();
				lbl_issue.Text = personal.Issue.ToString(KYJngMCOCJNSf8T7gH.Ushn1vyok(24));
				lbl_expire.Text = personal.Expire.ToString(KYJngMCOCJNSf8T7gH.Ushn1vyok(24));
				if (lbl_sex.Text == KYJngMCOCJNSf8T7gH.Ushn1vyok(188))
				{
					lbl_sex.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(194);
				}
				else
				{
					lbl_sex.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(204);
				}
				lbl_address.Text = personal.Address;
				label_no.Text = personal.addrHouseNo;
				label_moo.Text = personal.addrVillageNo.Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(216), "").Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(228), "").Trim();
				label_soi.Text = personal.addrLane.Trim();
				label_road.Text = personal.addrRoad.Trim();
				label_tanbon.Text = personal.addrTambol.Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(238), "").Trim();
				label_ampore.Text = personal.addrAmphur.Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(250), "").Trim();
				label_province.Text = personal.addrProvince.Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(264), "").Trim();
				pictureBox2.Image = personal.PhotoBitmap;
				if (checkBox_pic.Checked)
				{
				}
				pictureBox1.Image = Resources.Untitled_1__Copy_;
				PhotoProgressBar1.Visible = false;
				btnReadWithPhoto.Visible = true;
				label10.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(282);
				button1_Click(null, null);
			}
			else if (idcard.ErrorCode() > 0)
			{
				MessageBox.Show(KYJngMCOCJNSf8T7gH.Ushn1vyok(318) + idcard.Error(), KYJngMCOCJNSf8T7gH.Ushn1vyok(468), MessageBoxButtons.OK, MessageBoxIcon.Hand);
				try
				{
					idcard.Close();
				}
				catch (Exception)
				{
				}
				PhotoProgressBar1.Visible = false;
				btnReadWithPhoto.Visible = true;
				label10.Text = KYJngMCOCJNSf8T7gH.Ushn1vyok(482);
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void btnRefreshReaderList_Click_1(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			cbxReaderList.Items.Clear();
			cbxReaderList.SelectedIndex = -1;
			cbxReaderList.SelectedText = string.Empty;
			cbxReaderList.Text = string.Empty;
			cbxReaderList.Refresh();
			try
			{
				ThaiIDCard thaiIDCard = new ThaiIDCard();
				string[] readers = thaiIDCard.GetReaders();
				if (readers != null)
				{
					string[] array = readers;
					foreach (string item in array)
					{
						cbxReaderList.Items.Add(item);
					}
					cbxReaderList.DroppedDown = true;
				}
			}
			catch (Exception ex)
			{
				MessageBox.Show(ex.ToString());
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void chkBoxMonitor_CheckedChanged_1(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (chkBoxMonitor.Checked)
			{
				if (cbxReaderList.SelectedItem == null)
				{
					MessageBox.Show(KYJngMCOCJNSf8T7gH.Ushn1vyok(664));
					chkBoxMonitor.Checked = false;
				}
				else
				{
					idcard.MonitorStart(cbxReaderList.SelectedItem.ToString());
					idcard.eventCardInsertedWithPhoto += CardInserted;
					idcard.eventPhotoProgress += photoProgress;
				}
			}
			else if (cbxReaderList.SelectedItem != null)
			{
				idcard.MonitorStop(cbxReaderList.SelectedItem.ToString());
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void frmMain_Load(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			LoadSettings();
			try
			{
				File.Delete(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(730));
			}
			catch (Exception)
			{
			}
			try
			{
				File.Delete(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(756));
			}
			catch (Exception)
			{
			}
			try
			{
				if (!File.Exists(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(782)))
				{
					MessageBox.Show(KYJngMCOCJNSf8T7gH.Ushn1vyok(818), KYJngMCOCJNSf8T7gH.Ushn1vyok(468), MessageBoxButtons.OK, MessageBoxIcon.Hand, MessageBoxDefaultButton.Button1);
					Application.Exit();
					return;
				}
			}
			catch (Exception)
			{
			}
			try
			{
				File.Delete(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(782));
			}
			catch (Exception)
			{
			}
			try
			{
				ThaiIDCard thaiIDCard = new ThaiIDCard();
				string[] readers = thaiIDCard.GetReaders();
				if (readers == null)
				{
					return;
				}
				string[] array = readers;
				foreach (string text in array)
				{
					label20.Text = text;
				}
			}
			catch (Exception)
			{
			}
			if (label20.Text == KYJngMCOCJNSf8T7gH.Ushn1vyok(852))
			{
				MessageBox.Show(KYJngMCOCJNSf8T7gH.Ushn1vyok(858), KYJngMCOCJNSf8T7gH.Ushn1vyok(914), MessageBoxButtons.OK, MessageBoxIcon.Question, MessageBoxDefaultButton.Button1);
				Close();
			}
			if (checkBox_read.Checked)
			{
				timer1.Enabled = true;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void timer1_Tick(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			timer1.Enabled = false;
			btnReadWithPhoto_Click_1(null, null);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void button1_Click(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			using (StreamWriter streamWriter = new StreamWriter(File.Open(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(730), FileMode.Create), Encoding.UTF8))
			{
				streamWriter.WriteLine(lbl_cid.Text);
				streamWriter.WriteLine(lbl_th_prefix.Text);
				streamWriter.WriteLine(lbl_th_firstname.Text);
				streamWriter.WriteLine(lbl_th_lastname.Text);
				streamWriter.WriteLine(lbl_sex.Text);
				streamWriter.WriteLine(lbl_birthday.Text);
				streamWriter.WriteLine(lbl_issue.Text);
				streamWriter.WriteLine(lbl_expire.Text);
				streamWriter.WriteLine(lbl_address.Text);
				streamWriter.WriteLine(label_no.Text);
				streamWriter.WriteLine(label_moo.Text);
				streamWriter.WriteLine(label_soi.Text);
				streamWriter.WriteLine(label_road.Text);
				streamWriter.WriteLine(label_tanbon.Text);
				streamWriter.WriteLine(label_ampore.Text);
				streamWriter.WriteLine(label_province.Text);
				streamWriter.WriteLine(citizenID.Text);
				streamWriter.Close();
			}
			if (checkBox_pic.Checked)
			{
				int num = panel2.Size.Width;
				int num2 = panel2.Size.Height;
				Bitmap bitmap = new Bitmap(num, num2);
				panel2.DrawToBitmap(bitmap, new Rectangle(0, 0, num, num2));
				bitmap.Save(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(756), ImageFormat.Png);
			}
			Application.Exit();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void button2_Click(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (File.Exists(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(730)))
			{
				try
				{
					File.Delete(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(730));
				}
				catch (Exception)
				{
				}
			}
			if (File.Exists(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(756)))
			{
				try
				{
					File.Delete(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(756));
				}
				catch (Exception)
				{
				}
			}
			Application.Exit();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void pictureBox1_Click(object sender, EventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void uuu(object sender, PaintEventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			using Font font2 = new Font(KYJngMCOCJNSf8T7gH.Ushn1vyok(936), 22f, FontStyle.Bold);
			using Font font = new Font(KYJngMCOCJNSf8T7gH.Ushn1vyok(936), 22f, FontStyle.Bold);
			using Font font3 = new Font(KYJngMCOCJNSf8T7gH.Ushn1vyok(936), 18f, FontStyle.Bold);
			using Font font4 = new Font(KYJngMCOCJNSf8T7gH.Ushn1vyok(936), 14f, FontStyle.Bold);
			using SolidBrush brush = new SolidBrush(Color.FromArgb(255, 0, 0, 128));
			string text = "";
			string text2 = "";
			string text3 = "";
			int num = 0;
			try
			{
				e.Graphics.DrawString(lbl_cid.Text.Substring(0, 1) + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_cid.Text.Substring(1, 4) + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_cid.Text.Substring(5, 5) + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_cid.Text.Substring(10, 2) + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_cid.Text.Substring(12, 1), font, Brushes.Black, new Point(200, 25));
			}
			catch (Exception)
			{
			}
			try
			{
				label16.Text = lbl_address.Text.Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(238), KYJngMCOCJNSf8T7gH.Ushn1vyok(968)).Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(250), KYJngMCOCJNSf8T7gH.Ushn1vyok(976)).Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(264), KYJngMCOCJNSf8T7gH.Ushn1vyok(986))
					.Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(994), KYJngMCOCJNSf8T7gH.Ushn1vyok(1006));
			}
			catch (Exception)
			{
			}
			try
			{
				label18.Text = label16.Text.Substring(label16.Text.LastIndexOf(KYJngMCOCJNSf8T7gH.Ushn1vyok(1022)) + 1);
			}
			catch (Exception)
			{
			}
			try
			{
				label16.Text = label16.Text.Substring(0, label16.Text.LastIndexOf(KYJngMCOCJNSf8T7gH.Ushn1vyok(1022))).Replace(KYJngMCOCJNSf8T7gH.Ushn1vyok(1022), "");
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(lbl_th_prefix.Text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_th_firstname.Text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_th_lastname.Text, font2, Brushes.Black, new Point(110, 52));
			e.Graphics.DrawString(lbl_en_prefix.Text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + lbl_en_firstname.Text, font3, brush, new Point(180, 78));
			e.Graphics.DrawString(lbl_en_lastname.Text, font3, brush, new Point(205, 96));
			try
			{
				text = lbl_birthday.Text.Substring(0, 2);
				text2 = lbl_birthday.Text.Substring(3, 2);
				text3 = lbl_birthday.Text.Substring(6, 4);
				if (int.Parse(text2) == 1)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1028);
				}
				else if (int.Parse(text2) == 2)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1040);
				}
				else if (int.Parse(text2) == 3)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1052);
				}
				else if (int.Parse(text2) == 4)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1066);
				}
				else if (int.Parse(text2) == 5)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1080);
				}
				else if (int.Parse(text2) == 6)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1092);
				}
				else if (int.Parse(text2) == 7)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1106);
				}
				else if (int.Parse(text2) == 8)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1118);
				}
				else if (int.Parse(text2) == 9)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1130);
				}
				else if (int.Parse(text2) == 10)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1142);
				}
				else if (int.Parse(text2) == 11)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1154);
				}
				else if (int.Parse(text2) == 12)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1166);
				}
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text2 + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text3, font3, Brushes.Black, new Point(190, 115));
			try
			{
				text = lbl_birthday.Text.Substring(0, 2);
				text2 = lbl_birthday.Text.Substring(3, 2);
				text3 = lbl_birthday.Text.Substring(6, 4);
				if (int.Parse(text2) == 1)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1178);
				}
				else if (int.Parse(text2) == 2)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1190);
				}
				else if (int.Parse(text2) == 3)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1202);
				}
				else if (int.Parse(text2) == 4)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1214);
				}
				else if (int.Parse(text2) == 5)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1226);
				}
				else if (int.Parse(text2) == 6)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1238);
				}
				else if (int.Parse(text2) == 7)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1250);
				}
				else if (int.Parse(text2) == 8)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1262);
				}
				else if (int.Parse(text2) == 9)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1274);
				}
				else if (int.Parse(text2) == 10)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1286);
				}
				else if (int.Parse(text2) == 11)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1298);
				}
				else if (int.Parse(text2) == 12)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1310);
				}
			}
			catch (Exception)
			{
			}
			try
			{
				num = int.Parse(text3);
				num -= 543;
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text2 + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + num, font3, brush, new Point(220, 135));
			e.Graphics.DrawString(label16.Text, font3, Brushes.Black, new Point(71, 175));
			e.Graphics.DrawString(label18.Text, font3, Brushes.Black, new Point(44, 195));
			try
			{
				text = lbl_issue.Text.Substring(0, 2);
				text2 = lbl_issue.Text.Substring(3, 2);
				text3 = lbl_issue.Text.Substring(6, 4);
				if (int.Parse(text2) == 1)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1028);
				}
				else if (int.Parse(text2) == 2)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1040);
				}
				else if (int.Parse(text2) == 3)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1052);
				}
				else if (int.Parse(text2) == 4)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1066);
				}
				else if (int.Parse(text2) == 5)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1080);
				}
				else if (int.Parse(text2) == 6)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1092);
				}
				else if (int.Parse(text2) == 7)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1106);
				}
				else if (int.Parse(text2) == 8)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1118);
				}
				else if (int.Parse(text2) == 9)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1130);
				}
				else if (int.Parse(text2) == 10)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1142);
				}
				else if (int.Parse(text2) == 11)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1154);
				}
				else if (int.Parse(text2) == 12)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1166);
				}
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text2 + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text3, font4, Brushes.Black, new Point(44, 215));
			try
			{
				text = lbl_issue.Text.Substring(0, 2);
				text2 = lbl_issue.Text.Substring(3, 2);
				text3 = lbl_issue.Text.Substring(6, 4);
				if (int.Parse(text2) == 1)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1178);
				}
				else if (int.Parse(text2) == 2)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1190);
				}
				else if (int.Parse(text2) == 3)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1202);
				}
				else if (int.Parse(text2) == 4)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1214);
				}
				else if (int.Parse(text2) == 5)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1226);
				}
				else if (int.Parse(text2) == 6)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1238);
				}
				else if (int.Parse(text2) == 7)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1250);
				}
				else if (int.Parse(text2) == 8)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1262);
				}
				else if (int.Parse(text2) == 9)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1274);
				}
				else if (int.Parse(text2) == 10)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1286);
				}
				else if (int.Parse(text2) == 11)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1298);
				}
				else if (int.Parse(text2) == 12)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1310);
				}
			}
			catch (Exception)
			{
			}
			try
			{
				num = int.Parse(text3);
				num -= 543;
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text2 + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + num, font4, brush, new Point(44, 237));
			try
			{
				text = lbl_expire.Text.Substring(0, 2);
				text2 = lbl_expire.Text.Substring(3, 2);
				text3 = lbl_expire.Text.Substring(6, 4);
				if (int.Parse(text2) == 1)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1028);
				}
				else if (int.Parse(text2) == 2)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1040);
				}
				else if (int.Parse(text2) == 3)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1052);
				}
				else if (int.Parse(text2) == 4)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1066);
				}
				else if (int.Parse(text2) == 5)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1080);
				}
				else if (int.Parse(text2) == 6)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1092);
				}
				else if (int.Parse(text2) == 7)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1106);
				}
				else if (int.Parse(text2) == 8)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1118);
				}
				else if (int.Parse(text2) == 9)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1130);
				}
				else if (int.Parse(text2) == 10)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1142);
				}
				else if (int.Parse(text2) == 11)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1154);
				}
				else if (int.Parse(text2) == 12)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1166);
				}
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text2 + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text3, font4, Brushes.Black, new Point(244, 215));
			try
			{
				text = lbl_expire.Text.Substring(0, 2);
				text2 = lbl_expire.Text.Substring(3, 2);
				text3 = lbl_expire.Text.Substring(6, 4);
				if (int.Parse(text2) == 1)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1178);
				}
				else if (int.Parse(text2) == 2)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1190);
				}
				else if (int.Parse(text2) == 3)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1202);
				}
				else if (int.Parse(text2) == 4)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1214);
				}
				else if (int.Parse(text2) == 5)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1226);
				}
				else if (int.Parse(text2) == 6)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1238);
				}
				else if (int.Parse(text2) == 7)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1250);
				}
				else if (int.Parse(text2) == 8)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1262);
				}
				else if (int.Parse(text2) == 9)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1274);
				}
				else if (int.Parse(text2) == 10)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1286);
				}
				else if (int.Parse(text2) == 11)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1298);
				}
				else if (int.Parse(text2) == 12)
				{
					text2 = KYJngMCOCJNSf8T7gH.Ushn1vyok(1310);
				}
			}
			catch (Exception)
			{
			}
			try
			{
				num = int.Parse(text3);
				num -= 543;
			}
			catch (Exception)
			{
			}
			e.Graphics.DrawString(text + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + text2 + KYJngMCOCJNSf8T7gH.Ushn1vyok(962) + num, font4, brush, new Point(244, 237));
			try
			{
				e.Graphics.DrawImage(pictureBox2.Image, 332, 129, 103, 119);
			}
			catch (Exception)
			{
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public void SaveSettings()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			using StreamWriter streamWriter = new StreamWriter(File.Open(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(1322), FileMode.Create), Encoding.UTF8);
			streamWriter.WriteLine(checkBox_read.Checked);
			streamWriter.WriteLine(checkBox_pic.Checked);
			streamWriter.Close();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public void LoadSettings()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (!File.Exists(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(1322)))
			{
				SaveSettings();
			}
			int num = 0;
			StreamReader streamReader = new StreamReader(Directory.GetCurrentDirectory() + KYJngMCOCJNSf8T7gH.Ushn1vyok(1322));
			string value;
			while ((value = streamReader.ReadLine()) != null)
			{
				num++;
				switch (num)
				{
				case 1:
					try
					{
						checkBox_read.Checked = Convert.ToBoolean(value);
					}
					catch (Exception)
					{
					}
					break;
				case 2:
					try
					{
						checkBox_pic.Checked = Convert.ToBoolean(value);
					}
					catch (Exception)
					{
					}
					break;
				}
			}
			streamReader.Close();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void read_click(object sender, MouseEventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			SaveSettings();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void pic_click(object sender, MouseEventArgs e)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			SaveSettings();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		protected override void Dispose(bool disposing)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (disposing && components != null)
			{
				components.Dispose();
			}
			base.Dispose(disposing);
		}

		[System.Runtime.CompilerServices.MethodImpl(System.Runtime.CompilerServices.MethodImplOptions.NoInlining)]
		private void InitializeComponent()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			this.components = new System.ComponentModel.Container();
			System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(ThaiNationalIDCard.Example.frmMain));
			this.label13 = new System.Windows.Forms.Label();
			this.lbl_issue = new System.Windows.Forms.Label();
			this.label11 = new System.Windows.Forms.Label();
			this.btnRefreshReaderList = new System.Windows.Forms.Button();
			this.chkBoxMonitor = new System.Windows.Forms.CheckBox();
			this.cbxReaderList = new System.Windows.Forms.ComboBox();
			this.btnReadWithPhoto = new System.Windows.Forms.Button();
			this.PhotoProgressBar1 = new System.Windows.Forms.ProgressBar();
			this.pictureBox1 = new System.Windows.Forms.PictureBox();
			this.imageList1 = new System.Windows.Forms.ImageList(this.components);
			this.lbl_expire = new System.Windows.Forms.Label();
			this.lbl_sex = new System.Windows.Forms.Label();
			this.label9 = new System.Windows.Forms.Label();
			this.lbl_birthday = new System.Windows.Forms.Label();
			this.lbl_en_lastname = new System.Windows.Forms.Label();
			this.lbl_en_firstname = new System.Windows.Forms.Label();
			this.lbl_en_prefix = new System.Windows.Forms.Label();
			this.lbl_th_lastname = new System.Windows.Forms.Label();
			this.lbl_th_firstname = new System.Windows.Forms.Label();
			this.lbl_th_prefix = new System.Windows.Forms.Label();
			this.label8 = new System.Windows.Forms.Label();
			this.label7 = new System.Windows.Forms.Label();
			this.label6 = new System.Windows.Forms.Label();
			this.label5 = new System.Windows.Forms.Label();
			this.label4 = new System.Windows.Forms.Label();
			this.label3 = new System.Windows.Forms.Label();
			this.label2 = new System.Windows.Forms.Label();
			this.lbl_cid = new System.Windows.Forms.Label();
			this.label1 = new System.Windows.Forms.Label();
			this.timer1 = new System.Windows.Forms.Timer(this.components);
			this.panel1 = new System.Windows.Forms.Panel();
			this.label20 = new System.Windows.Forms.Label();
			this.checkBox_pic = new System.Windows.Forms.CheckBox();
			this.checkBox_read = new System.Windows.Forms.CheckBox();
			this.label24 = new System.Windows.Forms.Label();
			this.label22 = new System.Windows.Forms.Label();
			this.label10 = new System.Windows.Forms.Label();
			this.button1 = new System.Windows.Forms.Button();
			this.button2 = new System.Windows.Forms.Button();
			this.lbl_address = new System.Windows.Forms.Label();
			this.label15 = new System.Windows.Forms.Label();
			this.label_no = new System.Windows.Forms.Label();
			this.label14 = new System.Windows.Forms.Label();
			this.label_moo = new System.Windows.Forms.Label();
			this.label17 = new System.Windows.Forms.Label();
			this.label_soi = new System.Windows.Forms.Label();
			this.label19 = new System.Windows.Forms.Label();
			this.label_road = new System.Windows.Forms.Label();
			this.label21 = new System.Windows.Forms.Label();
			this.label_tanbon = new System.Windows.Forms.Label();
			this.label23 = new System.Windows.Forms.Label();
			this.label_ampore = new System.Windows.Forms.Label();
			this.label25 = new System.Windows.Forms.Label();
			this.label_province = new System.Windows.Forms.Label();
			this.label27 = new System.Windows.Forms.Label();
			this.citizenID = new System.Windows.Forms.Label();
			this.label16 = new System.Windows.Forms.Label();
			this.label12 = new System.Windows.Forms.Label();
			this.label18 = new System.Windows.Forms.Label();
			this.pictureBox2 = new System.Windows.Forms.PictureBox();
			this.panel2 = new System.Windows.Forms.Panel();
			((System.ComponentModel.ISupportInitialize)this.pictureBox1).BeginInit();
			this.panel1.SuspendLayout();
			((System.ComponentModel.ISupportInitialize)this.pictureBox2).BeginInit();
			this.panel2.SuspendLayout();
			base.SuspendLayout();
			this.label13.Location = new System.Drawing.Point(12, 253);
			this.label13.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1364);
			this.label13.Size = new System.Drawing.Size(132, 16);
			this.label13.TabIndex = 59;
			this.label13.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1382);
			this.lbl_issue.AutoSize = true;
			this.lbl_issue.Location = new System.Drawing.Point(152, 237);
			this.lbl_issue.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1406);
			this.lbl_issue.Size = new System.Drawing.Size(12, 16);
			this.lbl_issue.TabIndex = 58;
			this.lbl_issue.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label11.Location = new System.Drawing.Point(12, 237);
			this.label11.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1428);
			this.label11.Size = new System.Drawing.Size(132, 16);
			this.label11.TabIndex = 57;
			this.label11.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1446);
			this.btnRefreshReaderList.Location = new System.Drawing.Point(817, 11);
			this.btnRefreshReaderList.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.btnRefreshReaderList.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1470);
			this.btnRefreshReaderList.Size = new System.Drawing.Size(145, 28);
			this.btnRefreshReaderList.TabIndex = 56;
			this.btnRefreshReaderList.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1514);
			this.btnRefreshReaderList.UseVisualStyleBackColor = true;
			this.btnRefreshReaderList.Click += new System.EventHandler(btnRefreshReaderList_Click_1);
			this.chkBoxMonitor.AutoSize = true;
			this.chkBoxMonitor.Checked = true;
			this.chkBoxMonitor.CheckState = System.Windows.Forms.CheckState.Checked;
			this.chkBoxMonitor.Location = new System.Drawing.Point(658, 16);
			this.chkBoxMonitor.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.chkBoxMonitor.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1556);
			this.chkBoxMonitor.Size = new System.Drawing.Size(101, 20);
			this.chkBoxMonitor.TabIndex = 55;
			this.chkBoxMonitor.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1586);
			this.chkBoxMonitor.UseVisualStyleBackColor = true;
			this.chkBoxMonitor.CheckedChanged += new System.EventHandler(chkBoxMonitor_CheckedChanged_1);
			this.cbxReaderList.FormattingEnabled = true;
			this.cbxReaderList.Location = new System.Drawing.Point(658, 50);
			this.cbxReaderList.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.cbxReaderList.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1614);
			this.cbxReaderList.Size = new System.Drawing.Size(346, 24);
			this.cbxReaderList.TabIndex = 54;
			this.btnReadWithPhoto.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1644), 11.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 222);
			this.btnReadWithPhoto.Location = new System.Drawing.Point(203, 87);
			this.btnReadWithPhoto.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.btnReadWithPhoto.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1660);
			this.btnReadWithPhoto.Size = new System.Drawing.Size(111, 29);
			this.btnReadWithPhoto.TabIndex = 53;
			this.btnReadWithPhoto.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1696);
			this.btnReadWithPhoto.UseVisualStyleBackColor = true;
			this.btnReadWithPhoto.Click += new System.EventHandler(btnReadWithPhoto_Click_1);
			this.PhotoProgressBar1.Location = new System.Drawing.Point(75, 60);
			this.PhotoProgressBar1.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.PhotoProgressBar1.MarqueeAnimationSpeed = 0;
			this.PhotoProgressBar1.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1716);
			this.PhotoProgressBar1.Size = new System.Drawing.Size(366, 20);
			this.PhotoProgressBar1.TabIndex = 52;
			this.PhotoProgressBar1.Visible = false;
			this.pictureBox1.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch;
			this.pictureBox1.Image = KPNationalIDCard.Example.Properties.Resources.Untitled_1__Copy_;
			this.pictureBox1.Location = new System.Drawing.Point(121, 91);
			this.pictureBox1.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.pictureBox1.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1754);
			this.pictureBox1.Size = new System.Drawing.Size(446, 273);
			this.pictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
			this.pictureBox1.TabIndex = 51;
			this.pictureBox1.TabStop = false;
			this.pictureBox1.Click += new System.EventHandler(pictureBox1_Click);
			this.pictureBox1.Paint += new System.Windows.Forms.PaintEventHandler(uuu);
			this.imageList1.ColorDepth = System.Windows.Forms.ColorDepth.Depth8Bit;
			this.imageList1.ImageSize = new System.Drawing.Size(16, 16);
			this.imageList1.TransparentColor = System.Drawing.Color.Transparent;
			this.lbl_expire.AutoSize = true;
			this.lbl_expire.Location = new System.Drawing.Point(152, 253);
			this.lbl_expire.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1780);
			this.lbl_expire.Size = new System.Drawing.Size(12, 16);
			this.lbl_expire.TabIndex = 60;
			this.lbl_expire.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_sex.AutoSize = true;
			this.lbl_sex.Location = new System.Drawing.Point(152, 221);
			this.lbl_sex.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1804);
			this.lbl_sex.Size = new System.Drawing.Size(12, 16);
			this.lbl_sex.TabIndex = 50;
			this.lbl_sex.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label9.Location = new System.Drawing.Point(12, 221);
			this.label9.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1822);
			this.label9.Size = new System.Drawing.Size(132, 16);
			this.label9.TabIndex = 49;
			this.label9.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1838);
			this.lbl_birthday.AutoSize = true;
			this.lbl_birthday.Location = new System.Drawing.Point(152, 205);
			this.lbl_birthday.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1848);
			this.lbl_birthday.Size = new System.Drawing.Size(12, 16);
			this.lbl_birthday.TabIndex = 48;
			this.lbl_birthday.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_en_lastname.AutoSize = true;
			this.lbl_en_lastname.Location = new System.Drawing.Point(152, 177);
			this.lbl_en_lastname.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1876);
			this.lbl_en_lastname.Size = new System.Drawing.Size(12, 16);
			this.lbl_en_lastname.TabIndex = 47;
			this.lbl_en_lastname.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_en_firstname.AutoSize = true;
			this.lbl_en_firstname.Location = new System.Drawing.Point(152, 161);
			this.lbl_en_firstname.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1910);
			this.lbl_en_firstname.Size = new System.Drawing.Size(12, 16);
			this.lbl_en_firstname.TabIndex = 46;
			this.lbl_en_firstname.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_en_prefix.AutoSize = true;
			this.lbl_en_prefix.Location = new System.Drawing.Point(152, 145);
			this.lbl_en_prefix.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1946);
			this.lbl_en_prefix.Size = new System.Drawing.Size(12, 16);
			this.lbl_en_prefix.TabIndex = 45;
			this.lbl_en_prefix.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_th_lastname.AutoSize = true;
			this.lbl_th_lastname.Location = new System.Drawing.Point(152, 117);
			this.lbl_th_lastname.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1976);
			this.lbl_th_lastname.Size = new System.Drawing.Size(12, 16);
			this.lbl_th_lastname.TabIndex = 44;
			this.lbl_th_lastname.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_th_firstname.AutoSize = true;
			this.lbl_th_firstname.Location = new System.Drawing.Point(152, 101);
			this.lbl_th_firstname.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2010);
			this.lbl_th_firstname.Size = new System.Drawing.Size(12, 16);
			this.lbl_th_firstname.TabIndex = 43;
			this.lbl_th_firstname.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.lbl_th_prefix.AutoSize = true;
			this.lbl_th_prefix.Location = new System.Drawing.Point(152, 85);
			this.lbl_th_prefix.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2046);
			this.lbl_th_prefix.Size = new System.Drawing.Size(12, 16);
			this.lbl_th_prefix.TabIndex = 42;
			this.lbl_th_prefix.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label8.Location = new System.Drawing.Point(11, 205);
			this.label8.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2076);
			this.label8.Size = new System.Drawing.Size(132, 16);
			this.label8.TabIndex = 41;
			this.label8.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2092);
			this.label7.Location = new System.Drawing.Point(12, 184);
			this.label7.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2110);
			this.label7.Size = new System.Drawing.Size(132, 16);
			this.label7.TabIndex = 40;
			this.label7.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2126);
			this.label6.Location = new System.Drawing.Point(12, 154);
			this.label6.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2146);
			this.label6.Size = new System.Drawing.Size(132, 16);
			this.label6.TabIndex = 39;
			this.label6.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2162);
			this.label5.Location = new System.Drawing.Point(12, 145);
			this.label5.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2184);
			this.label5.Size = new System.Drawing.Size(132, 16);
			this.label5.TabIndex = 38;
			this.label5.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2200);
			this.label4.Location = new System.Drawing.Point(12, 117);
			this.label4.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2216);
			this.label4.Size = new System.Drawing.Size(132, 16);
			this.label4.TabIndex = 37;
			this.label4.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2232);
			this.label3.Location = new System.Drawing.Point(12, 101);
			this.label3.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2244);
			this.label3.Size = new System.Drawing.Size(132, 16);
			this.label3.TabIndex = 36;
			this.label3.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2260);
			this.label2.Location = new System.Drawing.Point(12, 85);
			this.label2.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2272);
			this.label2.Size = new System.Drawing.Size(132, 16);
			this.label2.TabIndex = 35;
			this.label2.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2288);
			this.lbl_cid.AutoSize = true;
			this.lbl_cid.Location = new System.Drawing.Point(152, 69);
			this.lbl_cid.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2300);
			this.lbl_cid.Size = new System.Drawing.Size(12, 16);
			this.lbl_cid.TabIndex = 34;
			this.lbl_cid.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label1.Location = new System.Drawing.Point(12, 69);
			this.label1.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2318);
			this.label1.Size = new System.Drawing.Size(132, 16);
			this.label1.TabIndex = 33;
			this.label1.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2334);
			this.timer1.Tick += new System.EventHandler(timer1_Tick);
			this.panel1.Anchor = System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right;
			this.panel1.BackColor = System.Drawing.Color.FromArgb(255, 255, 192);
			this.panel1.Controls.Add(this.btnReadWithPhoto);
			this.panel1.Controls.Add(this.label20);
			this.panel1.Controls.Add(this.checkBox_pic);
			this.panel1.Controls.Add(this.checkBox_read);
			this.panel1.Controls.Add(this.label24);
			this.panel1.Controls.Add(this.label22);
			this.panel1.Controls.Add(this.label10);
			this.panel1.Controls.Add(this.PhotoProgressBar1);
			this.panel1.Location = new System.Drawing.Point(10, 11);
			this.panel1.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.panel1.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2376);
			this.panel1.Size = new System.Drawing.Size(524, 162);
			this.panel1.TabIndex = 62;
			this.label20.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1644), 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
			this.label20.ForeColor = System.Drawing.Color.Red;
			this.label20.Location = new System.Drawing.Point(78, 118);
			this.label20.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2392);
			this.label20.Size = new System.Drawing.Size(430, 16);
			this.label20.TabIndex = 54;
			this.label20.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(852);
			this.label20.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
			this.checkBox_pic.AutoSize = true;
			this.checkBox_pic.Checked = true;
			this.checkBox_pic.CheckState = System.Windows.Forms.CheckState.Checked;
			this.checkBox_pic.Location = new System.Drawing.Point(240, 137);
			this.checkBox_pic.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2410);
			this.checkBox_pic.Size = new System.Drawing.Size(176, 20);
			this.checkBox_pic.TabIndex = 56;
			this.checkBox_pic.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2438);
			this.checkBox_pic.UseVisualStyleBackColor = true;
			this.checkBox_pic.MouseClick += new System.Windows.Forms.MouseEventHandler(pic_click);
			this.checkBox_read.AutoSize = true;
			this.checkBox_read.Location = new System.Drawing.Point(77, 137);
			this.checkBox_read.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2496);
			this.checkBox_read.Size = new System.Drawing.Size(161, 20);
			this.checkBox_read.TabIndex = 55;
			this.checkBox_read.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2526);
			this.checkBox_read.UseVisualStyleBackColor = true;
			this.checkBox_read.MouseClick += new System.Windows.Forms.MouseEventHandler(read_click);
			this.label24.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1644), 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
			this.label24.ForeColor = System.Drawing.Color.MediumBlue;
			this.label24.Location = new System.Drawing.Point(3, 138);
			this.label24.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2578);
			this.label24.Size = new System.Drawing.Size(71, 16);
			this.label24.TabIndex = 54;
			this.label24.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2596);
			this.label24.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
			this.label22.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1644), 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
			this.label22.ForeColor = System.Drawing.Color.MediumBlue;
			this.label22.Location = new System.Drawing.Point(3, 118);
			this.label22.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2618);
			this.label22.Size = new System.Drawing.Size(71, 16);
			this.label22.TabIndex = 54;
			this.label22.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2636);
			this.label22.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
			this.label10.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1644), 14.25f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
			this.label10.ForeColor = System.Drawing.Color.MediumBlue;
			this.label10.Location = new System.Drawing.Point(3, 2);
			this.label10.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2666);
			this.label10.Size = new System.Drawing.Size(518, 52);
			this.label10.TabIndex = 0;
			this.label10.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2684);
			this.label10.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
			this.button1.Location = new System.Drawing.Point(10, 422);
			this.button1.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.button1.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2752);
			this.button1.Size = new System.Drawing.Size(114, 34);
			this.button1.TabIndex = 31;
			this.button1.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2770);
			this.button1.UseVisualStyleBackColor = true;
			this.button1.Click += new System.EventHandler(button1_Click);
			this.button2.Location = new System.Drawing.Point(10, 464);
			this.button2.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.button2.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2782);
			this.button2.Size = new System.Drawing.Size(114, 34);
			this.button2.TabIndex = 31;
			this.button2.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2800);
			this.button2.UseVisualStyleBackColor = true;
			this.button2.Click += new System.EventHandler(button2_Click);
			this.lbl_address.AutoSize = true;
			this.lbl_address.Location = new System.Drawing.Point(152, 279);
			this.lbl_address.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2816);
			this.lbl_address.Size = new System.Drawing.Size(12, 16);
			this.lbl_address.TabIndex = 60;
			this.lbl_address.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label15.Location = new System.Drawing.Point(12, 279);
			this.label15.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2842);
			this.label15.Size = new System.Drawing.Size(132, 16);
			this.label15.TabIndex = 59;
			this.label15.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2860);
			this.label_no.AutoSize = true;
			this.label_no.Location = new System.Drawing.Point(152, 310);
			this.label_no.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2888);
			this.label_no.Size = new System.Drawing.Size(12, 16);
			this.label_no.TabIndex = 60;
			this.label_no.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label14.Location = new System.Drawing.Point(12, 310);
			this.label14.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2908);
			this.label14.Size = new System.Drawing.Size(132, 16);
			this.label14.TabIndex = 59;
			this.label14.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2926);
			this.label_moo.AutoSize = true;
			this.label_moo.Location = new System.Drawing.Point(152, 326);
			this.label_moo.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2958);
			this.label_moo.Size = new System.Drawing.Size(12, 16);
			this.label_moo.TabIndex = 60;
			this.label_moo.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label17.Location = new System.Drawing.Point(12, 326);
			this.label17.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2980);
			this.label17.Size = new System.Drawing.Size(132, 16);
			this.label17.TabIndex = 59;
			this.label17.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2998);
			this.label_soi.AutoSize = true;
			this.label_soi.Location = new System.Drawing.Point(152, 342);
			this.label_soi.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3026);
			this.label_soi.Size = new System.Drawing.Size(12, 16);
			this.label_soi.TabIndex = 60;
			this.label_soi.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label19.Location = new System.Drawing.Point(12, 342);
			this.label19.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3048);
			this.label19.Size = new System.Drawing.Size(132, 16);
			this.label19.TabIndex = 59;
			this.label19.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3066);
			this.label_road.AutoSize = true;
			this.label_road.Location = new System.Drawing.Point(152, 358);
			this.label_road.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3092);
			this.label_road.Size = new System.Drawing.Size(12, 16);
			this.label_road.TabIndex = 60;
			this.label_road.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label21.Location = new System.Drawing.Point(12, 358);
			this.label21.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3116);
			this.label21.Size = new System.Drawing.Size(132, 16);
			this.label21.TabIndex = 59;
			this.label21.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3134);
			this.label_tanbon.AutoSize = true;
			this.label_tanbon.Location = new System.Drawing.Point(152, 374);
			this.label_tanbon.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3160);
			this.label_tanbon.Size = new System.Drawing.Size(12, 16);
			this.label_tanbon.TabIndex = 60;
			this.label_tanbon.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label23.Location = new System.Drawing.Point(12, 374);
			this.label23.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3188);
			this.label23.Size = new System.Drawing.Size(132, 16);
			this.label23.TabIndex = 59;
			this.label23.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3206);
			this.label_ampore.AutoSize = true;
			this.label_ampore.Location = new System.Drawing.Point(152, 390);
			this.label_ampore.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3234);
			this.label_ampore.Size = new System.Drawing.Size(12, 16);
			this.label_ampore.TabIndex = 60;
			this.label_ampore.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label25.Location = new System.Drawing.Point(12, 390);
			this.label25.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3262);
			this.label25.Size = new System.Drawing.Size(132, 16);
			this.label25.TabIndex = 59;
			this.label25.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3280);
			this.label_province.AutoSize = true;
			this.label_province.Location = new System.Drawing.Point(152, 406);
			this.label_province.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3310);
			this.label_province.Size = new System.Drawing.Size(12, 16);
			this.label_province.TabIndex = 60;
			this.label_province.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label27.Location = new System.Drawing.Point(12, 406);
			this.label27.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3342);
			this.label27.Size = new System.Drawing.Size(132, 16);
			this.label27.TabIndex = 59;
			this.label27.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3360);
			this.citizenID.AutoSize = true;
			this.citizenID.Location = new System.Drawing.Point(152, 431);
			this.citizenID.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3394);
			this.citizenID.Size = new System.Drawing.Size(12, 16);
			this.citizenID.TabIndex = 60;
			this.citizenID.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(962);
			this.label16.Location = new System.Drawing.Point(12, 431);
			this.label16.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3416);
			this.label16.Size = new System.Drawing.Size(132, 16);
			this.label16.TabIndex = 59;
			this.label16.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3394);
			this.label12.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(936), 14.25f, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, 0);
			this.label12.Location = new System.Drawing.Point(80, 440);
			this.label12.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3434);
			this.label12.Size = new System.Drawing.Size(132, 30);
			this.label12.TabIndex = 63;
			this.label12.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(2200);
			this.label18.Location = new System.Drawing.Point(12, 447);
			this.label18.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3452);
			this.label18.Size = new System.Drawing.Size(132, 16);
			this.label18.TabIndex = 59;
			this.label18.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3394);
			this.pictureBox2.BackgroundImageLayout = System.Windows.Forms.ImageLayout.Stretch;
			this.pictureBox2.Location = new System.Drawing.Point(733, 177);
			this.pictureBox2.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.pictureBox2.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3470);
			this.pictureBox2.Size = new System.Drawing.Size(103, 119);
			this.pictureBox2.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
			this.pictureBox2.TabIndex = 51;
			this.pictureBox2.TabStop = false;
			this.pictureBox2.Click += new System.EventHandler(pictureBox1_Click);
			this.pictureBox2.Paint += new System.Windows.Forms.PaintEventHandler(uuu);
			this.panel2.BackColor = System.Drawing.Color.White;
			this.panel2.Controls.Add(this.pictureBox1);
			this.panel2.Location = new System.Drawing.Point(178, 187);
			this.panel2.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3496);
			this.panel2.Size = new System.Drawing.Size(703, 996);
			this.panel2.TabIndex = 64;
			base.AutoScaleDimensions = new System.Drawing.SizeF(7f, 16f);
			base.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
			base.ClientSize = new System.Drawing.Size(544, 182);
			base.Controls.Add(this.button2);
			base.Controls.Add(this.button1);
			base.Controls.Add(this.panel2);
			base.Controls.Add(this.label12);
			base.Controls.Add(this.pictureBox2);
			base.Controls.Add(this.panel1);
			base.Controls.Add(this.label18);
			base.Controls.Add(this.label16);
			base.Controls.Add(this.label27);
			base.Controls.Add(this.label25);
			base.Controls.Add(this.label23);
			base.Controls.Add(this.label21);
			base.Controls.Add(this.label19);
			base.Controls.Add(this.label17);
			base.Controls.Add(this.label14);
			base.Controls.Add(this.label15);
			base.Controls.Add(this.label13);
			base.Controls.Add(this.lbl_issue);
			base.Controls.Add(this.label11);
			base.Controls.Add(this.btnRefreshReaderList);
			base.Controls.Add(this.chkBoxMonitor);
			base.Controls.Add(this.cbxReaderList);
			base.Controls.Add(this.citizenID);
			base.Controls.Add(this.label_province);
			base.Controls.Add(this.label_ampore);
			base.Controls.Add(this.label_tanbon);
			base.Controls.Add(this.label_road);
			base.Controls.Add(this.label_soi);
			base.Controls.Add(this.label_moo);
			base.Controls.Add(this.label_no);
			base.Controls.Add(this.lbl_address);
			base.Controls.Add(this.lbl_expire);
			base.Controls.Add(this.lbl_sex);
			base.Controls.Add(this.label9);
			base.Controls.Add(this.lbl_birthday);
			base.Controls.Add(this.lbl_en_lastname);
			base.Controls.Add(this.lbl_en_firstname);
			base.Controls.Add(this.lbl_en_prefix);
			base.Controls.Add(this.lbl_th_lastname);
			base.Controls.Add(this.lbl_th_firstname);
			base.Controls.Add(this.lbl_th_prefix);
			base.Controls.Add(this.label8);
			base.Controls.Add(this.label7);
			base.Controls.Add(this.label6);
			base.Controls.Add(this.label5);
			base.Controls.Add(this.label4);
			base.Controls.Add(this.label3);
			base.Controls.Add(this.label2);
			base.Controls.Add(this.lbl_cid);
			base.Controls.Add(this.label1);
			this.Font = new System.Drawing.Font(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(1644), 9.75f, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, 222);
			base.FormBorderStyle = System.Windows.Forms.FormBorderStyle.SizableToolWindow;
			base.Icon = (System.Drawing.Icon)resources.GetObject(q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3512));
			base.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
			this.MinimumSize = new System.Drawing.Size(560, 183);
			base.Name = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3536);
			base.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
			this.Text = q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH.Ushn1vyok(3554);
			base.TopMost = true;
			base.Load += new System.EventHandler(frmMain_Load);
			((System.ComponentModel.ISupportInitialize)this.pictureBox1).EndInit();
			this.panel1.ResumeLayout(false);
			this.panel1.PerformLayout();
			((System.ComponentModel.ISupportInitialize)this.pictureBox2).EndInit();
			this.panel2.ResumeLayout(false);
			base.ResumeLayout(false);
			base.PerformLayout();
		}
	}
	internal static class Program
	{
		[MethodImpl(MethodImplOptions.NoInlining)]
		[STAThread]
		private static void Main()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Application.EnableVisualStyles();
			Application.SetCompatibleTextRenderingDefault(defaultValue: false);
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			Application.Run(new frmMain());
		}
	}
}
namespace KPNationalIDCard.Example.Properties
{
	[CompilerGenerated]
	[GeneratedCode("System.Resources.Tools.StronglyTypedResourceBuilder", "4.0.0.0")]
	[DebuggerNonUserCode]
	internal class Resources
	{
		private static ResourceManager resourceMan;

		private static CultureInfo resourceCulture;

		[EditorBrowsable(EditorBrowsableState.Advanced)]
		internal static ResourceManager ResourceManager
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				if (object.ReferenceEquals(resourceMan, null))
				{
					ResourceManager resourceManager = new ResourceManager("KPNationalIDCard.Example.Properties.Resources", typeof(Resources).Assembly);
					resourceMan = resourceManager;
				}
				return resourceMan;
			}
		}

		[EditorBrowsable(EditorBrowsableState.Advanced)]
		internal static CultureInfo Culture
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return resourceCulture;
			}
			[MethodImpl(MethodImplOptions.NoInlining)]
			set
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				resourceCulture = value;
			}
		}

		internal static Bitmap Untitled_1__Copy_
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				object @object = ResourceManager.GetObject(KYJngMCOCJNSf8T7gH.Ushn1vyok(3660), resourceCulture);
				return (Bitmap)@object;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal Resources()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			base..ctor();
		}
	}
	[CompilerGenerated]
	[GeneratedCode("Microsoft.VisualStudio.Editors.SettingsDesigner.SettingsSingleFileGenerator", "12.0.0.0")]
	internal sealed class Settings : ApplicationSettingsBase
	{
		private static Settings defaultInstance;

		public static Settings Default
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			get
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				return defaultInstance;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Settings()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			base..ctor();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		static Settings()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			defaultInstance = (Settings)SettingsBase.Synchronized(new Settings());
		}
	}
}
internal class <Module>{2BB13052-D3A3-4A86-A9F0-89A7E04C4951}
{
}
namespace JnQCRJrn8pxIJKp8wv
{
	internal class AAnZsP0BS70R6LKReR
	{
		internal delegate void SFU4mbT3GMret7THonf(object o);

		internal static Module aX1OwGmdd;

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void qhBY03hhuocS6(int typemdt)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Type type = aX1OwGmdd.ResolveType(33554432 + typemdt);
			FieldInfo[] fields = type.GetFields();
			foreach (FieldInfo fieldInfo in fields)
			{
				MethodInfo method = (MethodInfo)aX1OwGmdd.ResolveMethod(fieldInfo.MetadataToken + 100663296);
				fieldInfo.SetValue(null, (MulticastDelegate)Delegate.CreateDelegate(type, method));
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public AAnZsP0BS70R6LKReR()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			base..ctor();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		static AAnZsP0BS70R6LKReR()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
			aX1OwGmdd = typeof(AAnZsP0BS70R6LKReR).Assembly.ManifestModule;
		}
	}
}
namespace q3uifC6dT931xg0UY6
{
	internal class KYJngMCOCJNSf8T7gH
	{
		internal class KjHRR08XlRopQfagms : Attribute
		{
			internal class ghsislHGr7j3sh1vyo<T>
			{
				[MethodImpl(MethodImplOptions.NoInlining)]
				public ghsislHGr7j3sh1vyo()
				{
					while (false)
					{
						_ = ((object[])null)[0];
					}
					Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
					base..ctor();
				}
			}

			[MethodImpl(MethodImplOptions.NoInlining)]
			[KjHRR08XlRopQfagms(typeof(ghsislHGr7j3sh1vyo<object>[]))]
			public KjHRR08XlRopQfagms(object P_0)
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
				base..ctor();
			}
		}

		internal class OfWtmNPRNb6I2GO75O
		{
			[MethodImpl(MethodImplOptions.NoInlining)]
			[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
			internal static void ce4DmfsmSrOT856tDgfrkMb()
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				if (sHEOcC9b3g(Convert.ToBase64String(aVhy5TW9O.GetName().GetPublicKeyToken()), KYJngMCOCJNSf8T7gH.Ushn1vyok(962)) != KYJngMCOCJNSf8T7gH.Ushn1vyok(3698))
				{
					while (true)
					{
						ce4DmfsmSrOT856tDgfrkMb();
					}
				}
			}

			[MethodImpl(MethodImplOptions.NoInlining)]
			[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
			internal static string sHEOcC9b3g(string P_0, string P_1)
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				byte[] bytes = Encoding.Unicode.GetBytes(P_0);
				byte[] array = bytes;
				byte[] key = new byte[32]
				{
					82, 102, 104, 110, 32, 77, 24, 34, 118, 181,
					51, 17, 18, 51, 12, 109, 10, 32, 77, 24,
					34, 158, 161, 41, 97, 28, 118, 181, 5, 25,
					1, 88
				};
				byte[] iV = F0UsY6AjH(Encoding.Unicode.GetBytes(P_1));
				MemoryStream memoryStream = new MemoryStream();
				SymmetricAlgorithm symmetricAlgorithm = z7g9Hl3ui();
				symmetricAlgorithm.Key = key;
				symmetricAlgorithm.IV = iV;
				CryptoStream cryptoStream = new CryptoStream(memoryStream, symmetricAlgorithm.CreateEncryptor(), CryptoStreamMode.Write);
				cryptoStream.Write(array, 0, array.Length);
				cryptoStream.Close();
				return Convert.ToBase64String(memoryStream.ToArray());
			}

			[MethodImpl(MethodImplOptions.NoInlining)]
			public OfWtmNPRNb6I2GO75O()
			{
				while (false)
				{
					_ = ((object[])null)[0];
				}
				Rul53eN1pQAkHhMgbS.yKfY03hzDUAti();
				base..ctor();
			}
		}

		[UnmanagedFunctionPointer(CallingConvention.StdCall)]
		internal delegate uint Wuqowb9lIc6tX3qWLt(IntPtr classthis, IntPtr comp, IntPtr info, [MarshalAs(UnmanagedType.U4)] uint flags, IntPtr nativeEntry, ref uint nativeSizeOfCode);

		[UnmanagedFunctionPointer(CallingConvention.StdCall)]
		private delegate IntPtr Ati7N1c9sYeYbQmapS();

		internal struct HaxHIysb88myuIQ4yc
		{
			internal bool BhGOsYMDdW;

			internal byte[] znTOhPijeX;
		}

		[Flags]
		private enum D4iFUWh0NXSehWdnLN
		{

		}

		private static uint[] rq9RYIraG;

		private static byte[] aMhMAigfb;

		private static byte[] CkNDKJKMc;

		private static object aKcjgVSlH;

		private static int KgsvT9pRp;

		private static bool kKnZgCw2Z;

		private static int vAuzLKIv1;

		internal static Wuqowb9lIc6tX3qWLt WNmOOvXKn5;

		private static bool tykO8MrcfH;

		private static IntPtr AeXOPMTQBl;

		internal static Hashtable khdO9H49Dh;

		[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
		private static bool firstrundone;

		private static long rftOIVErF6;

		private static Assembly aVhy5TW9O;

		private static IntPtr Wpj5CF6TF;

		private static IntPtr Tug3FwLJ0;

		private static SortedList WZpawDXbD;

		private static int WqaOCGO3VR;

		private static byte[] SGlGF9S02;

		private static bool TrcoeJf3R;

		private static int Q75OHXj0Nh;

		private static bool O0JO6ZQEWR;

		internal static Wuqowb9lIc6tX3qWLt VaoO0qMJC7;

		private static int[] yZUxQGV3H;

		private static bool RdMdN7Uv2;

		private static long sCkOrMFhcp;

		private static bool u2CkaNWEg;

		private static byte[] QdBLb2qYc;

		[MethodImpl(MethodImplOptions.NoInlining)]
		static KYJngMCOCJNSf8T7gH()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			TrcoeJf3R = false;
			aVhy5TW9O = typeof(KYJngMCOCJNSf8T7gH).Assembly;
			rq9RYIraG = new uint[64]
			{
				3614090360u, 3905402710u, 606105819u, 3250441966u, 4118548399u, 1200080426u, 2821735955u, 4249261313u, 1770035416u, 2336552879u,
				4294925233u, 2304563134u, 1804603682u, 4254626195u, 2792965006u, 1236535329u, 4129170786u, 3225465664u, 643717713u, 3921069994u,
				3593408605u, 38016083u, 3634488961u, 3889429448u, 568446438u, 3275163606u, 4107603335u, 1163531501u, 2850285829u, 4243563512u,
				1735328473u, 2368359562u, 4294588738u, 2272392833u, 1839030562u, 4259657740u, 2763975236u, 1272893353u, 4139469664u, 3200236656u,
				681279174u, 3936430074u, 3572445317u, 76029189u, 3654602809u, 3873151461u, 530742520u, 3299628645u, 4096336452u, 1126891415u,
				2878612391u, 4237533241u, 1700485571u, 2399980690u, 4293915773u, 2240044497u, 1873313359u, 4264355552u, 2734768916u, 1309151649u,
				4149444226u, 3174756917u, 718787259u, 3951481745u
			};
			u2CkaNWEg = false;
			RdMdN7Uv2 = false;
			SGlGF9S02 = new byte[0];
			aMhMAigfb = new byte[0];
			QdBLb2qYc = new byte[0];
			CkNDKJKMc = new byte[0];
			Wpj5CF6TF = IntPtr.Zero;
			Tug3FwLJ0 = IntPtr.Zero;
			aKcjgVSlH = new string[0];
			yZUxQGV3H = new int[0];
			KgsvT9pRp = 1;
			kKnZgCw2Z = false;
			WZpawDXbD = new SortedList();
			vAuzLKIv1 = 0;
			rftOIVErF6 = 0L;
			WNmOOvXKn5 = null;
			VaoO0qMJC7 = null;
			sCkOrMFhcp = 0L;
			WqaOCGO3VR = 0;
			O0JO6ZQEWR = false;
			tykO8MrcfH = false;
			Q75OHXj0Nh = 0;
			AeXOPMTQBl = IntPtr.Zero;
			firstrundone = false;
			khdO9H49Dh = new Hashtable();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private void QJVY03hCm5RUc()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static byte[] yMM0iKRQG(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			uint[] array = new uint[16];
			int num = 448 - P_0.Length * 8 % 512;
			uint num2 = (uint)((num + 512) % 512);
			if (num2 == 0)
			{
				num2 = 512u;
			}
			uint num3 = (uint)(P_0.Length + num2 / 8 + 8);
			ulong num4 = (ulong)P_0.Length * 8uL;
			byte[] array2 = new byte[num3];
			for (int i = 0; i < P_0.Length; i++)
			{
				array2[i] = P_0[i];
			}
			array2[P_0.Length] |= 128;
			for (int num5 = 8; num5 > 0; num5--)
			{
				array2[num3 - num5] = (byte)((num4 >> (8 - num5) * 8) & 0xFF);
			}
			uint num6 = (uint)(array2.Length * 8) / 32u;
			uint num7 = 1732584193u;
			uint num8 = 4023233417u;
			uint num9 = 2562383102u;
			uint num10 = 271733878u;
			for (uint num11 = 0u; num11 < num6 / 16; num11++)
			{
				uint num12 = num11 << 6;
				for (uint num13 = 0u; num13 < 61; num13 += 4)
				{
					array[num13 >> 2] = (uint)((array2[num12 + (num13 + 3)] << 24) | (array2[num12 + (num13 + 2)] << 16) | (array2[num12 + (num13 + 1)] << 8) | array2[num12 + num13]);
				}
				uint num14 = num7;
				uint num15 = num8;
				uint num16 = num9;
				uint num17 = num10;
				jSArnZsPB(ref num7, num8, num9, num10, 0u, 7, 1u, array);
				jSArnZsPB(ref num10, num7, num8, num9, 1u, 12, 2u, array);
				jSArnZsPB(ref num9, num10, num7, num8, 2u, 17, 3u, array);
				jSArnZsPB(ref num8, num9, num10, num7, 3u, 22, 4u, array);
				jSArnZsPB(ref num7, num8, num9, num10, 4u, 7, 5u, array);
				jSArnZsPB(ref num10, num7, num8, num9, 5u, 12, 6u, array);
				jSArnZsPB(ref num9, num10, num7, num8, 6u, 17, 7u, array);
				jSArnZsPB(ref num8, num9, num10, num7, 7u, 22, 8u, array);
				jSArnZsPB(ref num7, num8, num9, num10, 8u, 7, 9u, array);
				jSArnZsPB(ref num10, num7, num8, num9, 9u, 12, 10u, array);
				jSArnZsPB(ref num9, num10, num7, num8, 10u, 17, 11u, array);
				jSArnZsPB(ref num8, num9, num10, num7, 11u, 22, 12u, array);
				jSArnZsPB(ref num7, num8, num9, num10, 12u, 7, 13u, array);
				jSArnZsPB(ref num10, num7, num8, num9, 13u, 12, 14u, array);
				jSArnZsPB(ref num9, num10, num7, num8, 14u, 17, 15u, array);
				jSArnZsPB(ref num8, num9, num10, num7, 15u, 22, 16u, array);
				A70CR6LKR(ref num7, num8, num9, num10, 1u, 5, 17u, array);
				A70CR6LKR(ref num10, num7, num8, num9, 6u, 9, 18u, array);
				A70CR6LKR(ref num9, num10, num7, num8, 11u, 14, 19u, array);
				A70CR6LKR(ref num8, num9, num10, num7, 0u, 20, 20u, array);
				A70CR6LKR(ref num7, num8, num9, num10, 5u, 5, 21u, array);
				A70CR6LKR(ref num10, num7, num8, num9, 10u, 9, 22u, array);
				A70CR6LKR(ref num9, num10, num7, num8, 15u, 14, 23u, array);
				A70CR6LKR(ref num8, num9, num10, num7, 4u, 20, 24u, array);
				A70CR6LKR(ref num7, num8, num9, num10, 9u, 5, 25u, array);
				A70CR6LKR(ref num10, num7, num8, num9, 14u, 9, 26u, array);
				A70CR6LKR(ref num9, num10, num7, num8, 3u, 14, 27u, array);
				A70CR6LKR(ref num8, num9, num10, num7, 8u, 20, 28u, array);
				A70CR6LKR(ref num7, num8, num9, num10, 13u, 5, 29u, array);
				A70CR6LKR(ref num10, num7, num8, num9, 2u, 9, 30u, array);
				A70CR6LKR(ref num9, num10, num7, num8, 7u, 14, 31u, array);
				A70CR6LKR(ref num8, num9, num10, num7, 12u, 20, 32u, array);
				mRq6nQCRJ(ref num7, num8, num9, num10, 5u, 4, 33u, array);
				mRq6nQCRJ(ref num10, num7, num8, num9, 8u, 11, 34u, array);
				mRq6nQCRJ(ref num9, num10, num7, num8, 11u, 16, 35u, array);
				mRq6nQCRJ(ref num8, num9, num10, num7, 14u, 23, 36u, array);
				mRq6nQCRJ(ref num7, num8, num9, num10, 1u, 4, 37u, array);
				mRq6nQCRJ(ref num10, num7, num8, num9, 4u, 11, 38u, array);
				mRq6nQCRJ(ref num9, num10, num7, num8, 7u, 16, 39u, array);
				mRq6nQCRJ(ref num8, num9, num10, num7, 10u, 23, 40u, array);
				mRq6nQCRJ(ref num7, num8, num9, num10, 13u, 4, 41u, array);
				mRq6nQCRJ(ref num10, num7, num8, num9, 0u, 11, 42u, array);
				mRq6nQCRJ(ref num9, num10, num7, num8, 3u, 16, 43u, array);
				mRq6nQCRJ(ref num8, num9, num10, num7, 6u, 23, 44u, array);
				mRq6nQCRJ(ref num7, num8, num9, num10, 9u, 4, 45u, array);
				mRq6nQCRJ(ref num10, num7, num8, num9, 12u, 11, 46u, array);
				mRq6nQCRJ(ref num9, num10, num7, num8, 15u, 16, 47u, array);
				mRq6nQCRJ(ref num8, num9, num10, num7, 2u, 23, 48u, array);
				n8p8xIJKp(ref num7, num8, num9, num10, 0u, 6, 49u, array);
				n8p8xIJKp(ref num10, num7, num8, num9, 7u, 10, 50u, array);
				n8p8xIJKp(ref num9, num10, num7, num8, 14u, 15, 51u, array);
				n8p8xIJKp(ref num8, num9, num10, num7, 5u, 21, 52u, array);
				n8p8xIJKp(ref num7, num8, num9, num10, 12u, 6, 53u, array);
				n8p8xIJKp(ref num10, num7, num8, num9, 3u, 10, 54u, array);
				n8p8xIJKp(ref num9, num10, num7, num8, 10u, 15, 55u, array);
				n8p8xIJKp(ref num8, num9, num10, num7, 1u, 21, 56u, array);
				n8p8xIJKp(ref num7, num8, num9, num10, 8u, 6, 57u, array);
				n8p8xIJKp(ref num10, num7, num8, num9, 15u, 10, 58u, array);
				n8p8xIJKp(ref num9, num10, num7, num8, 6u, 15, 59u, array);
				n8p8xIJKp(ref num8, num9, num10, num7, 13u, 21, 60u, array);
				n8p8xIJKp(ref num7, num8, num9, num10, 4u, 6, 61u, array);
				n8p8xIJKp(ref num10, num7, num8, num9, 11u, 10, 62u, array);
				n8p8xIJKp(ref num9, num10, num7, num8, 2u, 15, 63u, array);
				n8p8xIJKp(ref num8, num9, num10, num7, 9u, 21, 64u, array);
				num7 += num14;
				num8 += num15;
				num9 += num16;
				num10 += num17;
			}
			byte[] array3 = new byte[16];
			Array.Copy(BitConverter.GetBytes(num7), 0, array3, 0, 4);
			Array.Copy(BitConverter.GetBytes(num8), 0, array3, 4, 4);
			Array.Copy(BitConverter.GetBytes(num9), 0, array3, 8, 4);
			Array.Copy(BitConverter.GetBytes(num10), 0, array3, 12, 4);
			return array3;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void jSArnZsPB(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + fwvHAYJng(P_0 + ((P_1 & P_2) | (~P_1 & P_3)) + P_7[P_4] + rq9RYIraG[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void A70CR6LKR(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + fwvHAYJng(P_0 + ((P_1 & P_3) | (P_2 & ~P_3)) + P_7[P_4] + rq9RYIraG[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void mRq6nQCRJ(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + fwvHAYJng(P_0 + (P_1 ^ P_2 ^ P_3) + P_7[P_4] + rq9RYIraG[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void n8p8xIJKp(ref uint P_0, uint P_1, uint P_2, uint P_3, uint P_4, ushort P_5, uint P_6, uint[] P_7)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			P_0 = P_1 + fwvHAYJng(P_0 + (P_2 ^ (P_1 | ~P_3)) + P_7[P_4] + rq9RYIraG[P_6 - 1], P_5);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static uint fwvHAYJng(uint P_0, ushort P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return (P_0 >> 32 - P_1) | (P_0 << (int)P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static bool QOCPJNSf8()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (!u2CkaNWEg)
			{
				tCdcT931x();
				u2CkaNWEg = true;
			}
			return RdMdN7Uv2;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static SymmetricAlgorithm z7g9Hl3ui()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			SymmetricAlgorithm symmetricAlgorithm = null;
			if (QOCPJNSf8())
			{
				return new AesCryptoServiceProvider();
			}
			try
			{
				return new RijndaelManaged();
			}
			catch
			{
				return (SymmetricAlgorithm)Activator.CreateInstance("System.Core, Version=3.5.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089", "System.Security.Cryptography.AesCryptoServiceProvider").Unwrap();
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void tCdcT931x()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				RdMdN7Uv2 = CryptoConfig.AllowOnlyFipsAlgorithms;
			}
			catch
			{
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static byte[] F0UsY6AjH(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			if (!QOCPJNSf8())
			{
				return new MD5CryptoServiceProvider().ComputeHash(P_0);
			}
			return yMM0iKRQG(P_0);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static uint jiselGr7j(uint P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return (uint)"{11111-22222-10009-11112}".Length;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
		static string Ushn1vyok(int P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			int num = 38;
			byte[] array2 = default(byte[]);
			byte[] array = default(byte[]);
			int num7 = default(int);
			int num6 = default(int);
			uint num18 = default(uint);
			byte[] array7 = default(byte[]);
			uint num10 = default(uint);
			BinaryReader binaryReader = default(BinaryReader);
			int num9 = default(int);
			int num3 = default(int);
			MemoryStream memoryStream = default(MemoryStream);
			int num17 = default(int);
			int num20 = default(int);
			int num12 = default(int);
			byte[] array4 = default(byte[]);
			byte[] array5 = default(byte[]);
			byte[] array3 = default(byte[]);
			int num2 = default(int);
			int num24 = default(int);
			CryptoStream cryptoStream = default(CryptoStream);
			uint num22 = default(uint);
			uint num11 = default(uint);
			ICryptoTransform transform = default(ICryptoTransform);
			SymmetricAlgorithm symmetricAlgorithm = default(SymmetricAlgorithm);
			byte[] array6 = default(byte[]);
			int num13 = default(int);
			uint num23 = default(uint);
			uint num16 = default(uint);
			int num15 = default(int);
			int num21 = default(int);
			int num5 = default(int);
			uint num14 = default(uint);
			int num19 = default(int);
			int num25 = default(int);
			while (true)
			{
				int num4;
				int num8;
				switch (num)
				{
				case 192:
					array2[2] = 133;
					num4 = 177;
					goto IL_0176;
				case 389:
					array[1] = 151;
					num8 = 271;
					goto IL_0172;
				case 55:
					array[27] = (byte)num7;
					num4 = 255;
					if (false)
					{
						goto case 125;
					}
					goto IL_0176;
				case 125:
					array2[10] = (byte)num6;
					num8 = 401;
					goto IL_0172;
				case 80:
					num18 = (uint)((array7[num10 + 3] << 24) | (array7[num10 + 2] << 16) | (array7[num10 + 1] << 8) | array7[num10]);
					num8 = 288;
					goto IL_0172;
				case 18:
					G5AN3eK6lnHWagAG0N(binaryReader);
					num8 = 403;
					goto IL_0172;
				case 142:
					num9 = 6 + 62;
					num4 = 51;
					if (1 == 0)
					{
						goto case 279;
					}
					goto IL_0176;
				case 279:
					num7 = 51 + 30;
					num = 157;
					continue;
				case 410:
					array2[11] = (byte)num9;
					num4 = 378;
					if (false)
					{
						goto case 335;
					}
					goto IL_0176;
				case 335:
					num7 = 209 - 69;
					num = 374;
					continue;
				case 64:
					array[0] = (byte)num3;
					num8 = 101;
					goto IL_0172;
				case 308:
					array[28] = (byte)num7;
					goto case 234;
				default:
					num = 234;
					continue;
				case 360:
					array[29] = 113;
					num4 = 348;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 367;
				case 187:
					num6 = 132 - 44;
					num8 = 193;
					goto IL_0172;
				case 97:
					num7 = 103 + 32;
					num8 = 6;
					goto IL_0172;
				case 244:
					num18 = 0u;
					num = 135;
					continue;
				case 381:
					array2[2] = 134;
					num8 = 192;
					goto IL_0172;
				case 214:
					array[26] = 132;
					num4 = 171;
					goto IL_0176;
				case 131:
					num9 = 167 + 34;
					num = 189;
					continue;
				case 200:
					array2[3] = 193;
					num4 = 148;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 391;
				case 305:
					array2[8] = 90;
					num = 398;
					continue;
				case 322:
					CKU7bxUcwRJMDZJF1y(memoryStream);
					num4 = 281;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 204;
				case 204:
					array2 = new byte[16];
					num4 = 130;
					if (true)
					{
						goto IL_0176;
					}
					goto case 427;
				case 427:
					num17 = array7.Length / 4;
					num8 = 250;
					goto IL_0172;
				case 72:
					array[12] = 89;
					num4 = 220;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 417;
				case 132:
					num3 = 119 + 55;
					num = 226;
					continue;
				case 123:
				case 150:
					if (num20 >= num12)
					{
						num4 = 424;
						if (xhiIslqyx1D5WUBVSt())
						{
							goto IL_0176;
						}
						goto case 262;
					}
					if (num20 > 0)
					{
						num8 = 19;
						goto IL_0172;
					}
					goto case 168;
				case 395:
					array[21] = (byte)num7;
					num4 = 426;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 403;
				case 127:
					array2[6] = 123;
					num4 = 396;
					goto IL_0176;
				case 225:
					num3 = 45 + 81;
					num8 = 208;
					goto IL_0172;
				case 82:
					array[15] = 88;
					num4 = 74;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 112;
				case 112:
					num6 = 120 + 97;
					num4 = 283;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 174;
				case 174:
					num7 = 105 + 58;
					num4 = 337;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 176;
				case 176:
					num6 = 225 - 75;
					num8 = 125;
					goto IL_0172;
				case 110:
					array4[5] = array5[2];
					num4 = 66;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 88;
				case 88:
					array3 = array;
					num = 204;
					continue;
				case 318:
					array[20] = (byte)num3;
					num8 = 425;
					goto IL_0172;
				case 266:
					num3 = 102 + 47;
					num4 = 387;
					if (true)
					{
						goto IL_0176;
					}
					goto case 340;
				case 340:
					num18 = 0u;
					num4 = 77;
					if (true)
					{
						goto IL_0176;
					}
					goto case 99;
				case 99:
					num9 = 165 - 55;
					num8 = 120;
					goto IL_0172;
				case 312:
					array[3] = 91;
					num = 320;
					continue;
				case 56:
					if (array5.Length > 0)
					{
						num = 5;
						continue;
					}
					goto case 218;
				case 111:
					num7 = 47 + 85;
					num = 104;
					continue;
				case 65:
					array[20] = (byte)num3;
					num8 = 319;
					goto IL_0172;
				case 33:
					array[30] = 113;
					num4 = 31;
					goto IL_0176;
				case 228:
					array[23] = (byte)num3;
					num = 103;
					continue;
				case 222:
					array[6] = (byte)num7;
					num4 = 170;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 189;
				case 255:
					array[27] = 164;
					num4 = 323;
					goto IL_0176;
				case 162:
					array2[15] = 160;
					num4 = 30;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 331;
				case 331:
					array2[12] = 151;
					num8 = 145;
					goto IL_0172;
				case 260:
					array[22] = 181;
					num = 57;
					continue;
				case 188:
					array[16] = 143;
					num8 = 350;
					goto IL_0172;
				case 186:
					array[29] = 82;
					num = 116;
					continue;
				case 359:
					array[12] = (byte)num3;
					num4 = 366;
					goto IL_0176;
				case 32:
					array[1] = (byte)num3;
					num8 = 358;
					goto IL_0172;
				case 202:
					array2[10] = (byte)num6;
					num = 191;
					continue;
				case 121:
					array[25] = (byte)num3;
					num8 = 129;
					goto IL_0172;
				case 306:
					array[27] = (byte)num3;
					num4 = 172;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 119;
				case 119:
					array2[0] = 109;
					num4 = 361;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 289;
				case 289:
					num2 = QDTqS570cuCgZu2ShH(QdBLb2qYc, P_0);
					num8 = 434;
					goto IL_0172;
				case 316:
					num3 = 211 - 70;
					num = 197;
					continue;
				case 314:
					array[8] = 67;
					num4 = 174;
					goto IL_0176;
				case 428:
					array2[15] = (byte)num9;
					num = 112;
					continue;
				case 385:
					num7 = 148 - 49;
					num4 = 237;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 261;
				case 261:
					array[15] = (byte)num3;
					num = 106;
					continue;
				case 137:
				case 245:
					if (num24 >= num12)
					{
						num4 = 383;
						goto IL_0176;
					}
					if (num24 > 0)
					{
						num8 = 311;
						goto IL_0172;
					}
					goto case 310;
				case 40:
					num3 = 73 + 2;
					num = 306;
					continue;
				case 201:
					array2[13] = 115;
					num4 = 291;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 184;
				case 184:
					if (P_0 == -1)
					{
						num4 = 363;
						if (xhiIslqyx1D5WUBVSt())
						{
							goto IL_0176;
						}
						goto case 203;
					}
					goto case 431;
				case 281:
					CKU7bxUcwRJMDZJF1y(cryptoStream);
					num4 = 327;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 289;
				case 284:
					num6 = 154 - 51;
					num8 = 247;
					goto IL_0172;
				case 42:
					array[10] = 133;
					num = 147;
					continue;
				case 346:
					array[22] = 192;
					num = 225;
					continue;
				case 288:
					num22 += num11;
					num4 = 169;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 233;
				case 424:
					num22 += num11;
					num4 = 325;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 194;
				case 128:
					array2[7] = 108;
					num = 305;
					continue;
				case 95:
					array2[1] = (byte)num6;
					num4 = 239;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 370;
				case 96:
					array2[8] = 47;
					num4 = 269;
					goto IL_0176;
				case 224:
					array[18] = 176;
					num8 = 85;
					goto IL_0172;
				case 67:
					num7 = 58 + 49;
					num = 253;
					continue;
				case 133:
					array[10] = 121;
					num4 = 292;
					if (true)
					{
						goto IL_0176;
					}
					goto case 166;
				case 166:
					num6 = 208 - 69;
					num4 = 388;
					goto IL_0176;
				case 15:
					num3 = 73 + 2;
					num4 = 303;
					if (true)
					{
						goto IL_0176;
					}
					goto case 30;
				case 30:
					array2[15] = 87;
					num8 = 375;
					goto IL_0172;
				case 182:
					num7 = 58 + 68;
					num4 = 102;
					goto IL_0176;
				case 87:
					num24++;
					num = 137;
					continue;
				case 373:
					array[13] = 182;
					num4 = 297;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 21;
				case 21:
					num7 = 128 - 42;
					num = 294;
					continue;
				case 118:
					KSAvrHdmiBtXgP30M7(true);
					num4 = 364;
					if (true)
					{
						goto IL_0176;
					}
					goto case 276;
				case 276:
					array[7] = (byte)num3;
					num = 405;
					continue;
				case 23:
					array[0] = 18;
					num = 299;
					continue;
				case 101:
					num7 = 205 - 68;
					num4 = 282;
					if (true)
					{
						goto IL_0176;
					}
					goto case 75;
				case 75:
					num6 = 43 + 80;
					num4 = 95;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 418;
				case 418:
					num6 = 49 + 40;
					num8 = 257;
					goto IL_0172;
				case 44:
					array2[13] = (byte)num9;
					num8 = 22;
					goto IL_0172;
				case 57:
					array[22] = 110;
					num8 = 159;
					goto IL_0172;
				case 59:
					array[12] = 2;
					num = 15;
					continue;
				case 356:
					array[18] = (byte)num3;
					num = 224;
					continue;
				case 3:
					transform = (ICryptoTransform)n1q6NkD5E3kG69nYNh(symmetricAlgorithm, array3, array4);
					num4 = 256;
					goto IL_0176;
				case 390:
					num3 = 78 + 71;
					num = 414;
					continue;
				case 105:
					array2[9] = 133;
					num8 = 309;
					goto IL_0172;
				case 25:
					array4[13] = array5[6];
					num = 48;
					continue;
				case 168:
					num18 |= array7[array7.Length - (1 + num20)];
					num4 = 181;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 195;
				case 299:
					num3 = 216 - 72;
					num8 = 32;
					goto IL_0172;
				case 16:
					array[5] = 41;
					num = 384;
					continue;
				case 326:
					array[24] = (byte)num3;
					num4 = 372;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 190;
				case 190:
					array2[4] = 77;
					num4 = 9;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 43;
				case 43:
					array[3] = 41;
					num4 = 385;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 85;
				case 160:
					num22 = 0u;
					num = 404;
					continue;
				case 109:
					array[14] = 52;
					num8 = 280;
					goto IL_0172;
				case 0:
					array[18] = (byte)num7;
					num4 = 380;
					if (true)
					{
						goto IL_0176;
					}
					goto case 70;
				case 70:
					array[11] = 133;
					num = 252;
					continue;
				case 280:
					array[14] = 140;
					num = 17;
					continue;
				case 351:
					num7 = 141 - 47;
					num4 = 0;
					if (true)
					{
						goto IL_0176;
					}
					goto case 313;
				case 313:
					num7 = 85 + 2;
					num4 = 230;
					goto IL_0176;
				case 290:
					array2[10] = 173;
					num8 = 63;
					goto IL_0172;
				case 227:
					array[21] = (byte)num7;
					num = 81;
					continue;
				case 78:
					array2[5] = (byte)num6;
					num8 = 127;
					goto IL_0172;
				case 285:
					QdBLb2qYc = array6;
					num8 = 289;
					goto IL_0172;
				case 79:
					array2[5] = 146;
					num8 = 343;
					goto IL_0172;
				case 317:
					num3 = 137 - 45;
					num4 = 407;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 49;
				case 49:
					array[16] = 152;
					num8 = 421;
					goto IL_0172;
				case 9:
					array2[4] = 137;
					num8 = 115;
					goto IL_0172;
				case 232:
					cryptoStream = new CryptoStream(memoryStream, transform, CryptoStreamMode.Write);
					num8 = 203;
					goto IL_0172;
				case 259:
					array[13] = (byte)num3;
					num = 124;
					continue;
				case 380:
					num3 = 217 - 72;
					num4 = 356;
					if (true)
					{
						goto IL_0176;
					}
					goto case 27;
				case 27:
					array2[2] = 207;
					num4 = 284;
					if (true)
					{
						goto IL_0176;
					}
					goto case 257;
				case 257:
					array2[12] = (byte)num6;
					num8 = 331;
					goto IL_0172;
				case 223:
					array[1] = (byte)num3;
					num = 2;
					continue;
				case 115:
					num9 = 40 + 98;
					num4 = 332;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 234;
				case 234:
					array[28] = 92;
					num4 = 360;
					if (true)
					{
						goto IL_0176;
					}
					goto case 14;
				case 14:
					num3 = 134 - 44;
					num4 = 259;
					goto IL_0176;
				case 246:
					num9 = 204 - 68;
					num = 167;
					continue;
				case 46:
					binaryReader = new BinaryReader((Stream)mJyO00M3UjGXaqPZkZ(aVhy5TW9O, "Ecg9CU6tPYIEAx3i5O.UeYbNxpbrHEVs3g44B"));
					num8 = 76;
					goto IL_0172;
				case 170:
					array[6] = 86;
					num8 = 328;
					goto IL_0172;
				case 361:
					array2[0] = 130;
					num4 = 272;
					goto IL_0176;
				case 378:
					array2[11] = 122;
					num8 = 315;
					goto IL_0172;
				case 129:
					array[25] = 139;
					num4 = 317;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 207;
				case 207:
					array[3] = (byte)num7;
					num4 = 241;
					if (true)
					{
						goto IL_0176;
					}
					goto case 412;
				case 412:
					if (num12 > 0)
					{
						num4 = 353;
						if (true)
						{
							goto IL_0176;
						}
						goto case 208;
					}
					goto IL_2111;
				case 208:
					array[23] = (byte)num3;
					num = 316;
					continue;
				case 148:
					num9 = 88 + 120;
					num4 = 339;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 277;
				case 98:
					num9 = 223 - 74;
					num8 = 410;
					goto IL_0172;
				case 343:
					num6 = 215 + 30;
					num4 = 78;
					if (true)
					{
						goto IL_0176;
					}
					goto case 12;
				case 12:
					array[5] = 196;
					num = 16;
					continue;
				case 28:
					iGCSRRaVF18o3clKoJ(cryptoStream);
					num = 258;
					continue;
				case 178:
					num10 = 0u;
					num = 216;
					continue;
				case 243:
					array2[0] = (byte)num9;
					num = 119;
					continue;
				case 242:
					num3 = 172 - 57;
					num = 379;
					continue;
				case 341:
					array[14] = 46;
					num8 = 423;
					goto IL_0172;
				case 303:
					array[13] = (byte)num3;
					num4 = 14;
					if (true)
					{
						goto IL_0176;
					}
					goto case 13;
				case 13:
					array[19] = 134;
					num4 = 20;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 26;
				case 26:
					array2[5] = (byte)num6;
					num4 = 393;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 102;
				case 102:
					array[7] = (byte)num7;
					num4 = 314;
					goto IL_0176;
				case 264:
					array2[14] = 124;
					num4 = 24;
					goto IL_0176;
				case 196:
					array[10] = 120;
					num8 = 42;
					goto IL_0172;
				case 253:
					array[5] = (byte)num7;
					num4 = 355;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 286;
				case 211:
					num3 = 85 + 120;
					num8 = 140;
					goto IL_0172;
				case 415:
					array[17] = 164;
					num4 = 62;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 203;
				case 203:
					oRfwbQuDKvToZ2xxIJ(cryptoStream, array7, 0, array7.Length);
					num = 28;
					continue;
				case 77:
					if (num12 <= 0)
					{
						goto case 178;
					}
					num4 = 152;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 205;
				case 405:
					array[7] = 95;
					num4 = 182;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 268;
				case 416:
					array2[11] = 110;
					num = 166;
					continue;
				case 47:
					num24 = 0;
					num8 = 245;
					goto IL_0172;
				case 153:
					array2[0] = (byte)num9;
					num4 = 187;
					goto IL_0176;
				case 8:
					num3 = 43 + 40;
					num = 261;
					continue;
				case 158:
					array4[11] = array5[5];
					num4 = 25;
					goto IL_0176;
				case 209:
					array2[4] = (byte)num6;
					num4 = 117;
					goto IL_0176;
				case 426:
					array[22] = 56;
					num8 = 260;
					goto IL_0172;
				case 310:
					array6[num13 + num24] = (byte)((num23 & num16) >> num15);
					num4 = 87;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 298;
				case 298:
					array[21] = 99;
					_ = 0;
					if (xhiIslqyx1D5WUBVSt())
					{
						num4 = 141;
						goto IL_0176;
					}
					num = 161;
					continue;
				case 134:
					if (array5 != null)
					{
						num = 56;
						continue;
					}
					goto case 218;
				case 287:
					num7 = 90 + 36;
					num = 238;
					continue;
				case 297:
					array[14] = 37;
					num4 = 109;
					goto IL_0176;
				case 106:
					array[15] = 146;
					num8 = 49;
					goto IL_0172;
				case 179:
					array2[4] = 132;
					num4 = 190;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 401;
				case 401:
					num6 = 61 + 108;
					num = 202;
					continue;
				case 181:
					num20++;
					num4 = 123;
					goto IL_0176;
				case 366:
					array[12] = 181;
					num4 = 59;
					if (true)
					{
						goto IL_0176;
					}
					goto case 327;
				case 327:
					array7 = QdBLb2qYc;
					num4 = 431;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 219;
				case 432:
					num21++;
					num4 = 36;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 5;
				case 5:
					array4[1] = array5[0];
					num8 = 365;
					goto IL_0172;
				case 136:
					array2[14] = (byte)num6;
					num4 = 275;
					if (true)
					{
						goto IL_0176;
					}
					goto case 157;
				case 157:
					array[19] = (byte)num7;
					num4 = 251;
					if (true)
					{
						goto IL_0176;
					}
					goto case 52;
				case 52:
					array[20] = 107;
					num8 = 73;
					goto IL_0172;
				case 302:
					array[31] = (byte)num3;
					num = 60;
					continue;
				case 328:
					array[6] = 216;
					num4 = 304;
					if (true)
					{
						goto IL_0176;
					}
					goto case 307;
				case 307:
					num3 = 19 + 86;
					num4 = 121;
					goto IL_0176;
				case 294:
					array[30] = (byte)num7;
					num4 = 376;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 107;
				case 107:
					IfBCnwi0sGlQQWMCAp(array4);
					num8 = 386;
					goto IL_0172;
				case 35:
					array[26] = (byte)num3;
					num4 = 91;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 2;
				case 2:
					num7 = 100 + 101;
					num4 = 71;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 194;
				case 194:
					array2[5] = 101;
					num4 = 79;
					if (true)
					{
						goto IL_0176;
					}
					goto case 159;
				case 159:
					array[22] = 177;
					num4 = 287;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 90;
				case 90:
					num7 = 55 - 54;
					num4 = 422;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 4;
				case 4:
					array2[9] = 136;
					num8 = 105;
					goto IL_0172;
				case 91:
					array[26] = 134;
					num = 214;
					continue;
				case 334:
					array[2] = 136;
					num = 211;
					continue;
				case 238:
					array[22] = (byte)num7;
					num = 346;
					continue;
				case 324:
					array[25] = (byte)num3;
					num = 307;
					continue;
				case 392:
					array2[9] = 156;
					num = 210;
					continue;
				case 396:
					array2[6] = 141;
					num4 = 99;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 54;
				case 54:
					array[5] = (byte)num3;
					num8 = 300;
					goto IL_0172;
				case 406:
					num3 = 20 + 29;
					num = 64;
					continue;
				case 342:
					num9 = 157 - 52;
					num4 = 428;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 275;
				case 352:
					array2[9] = (byte)num6;
					num4 = 92;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 364;
				case 364:
					array7 = (byte[])TNt27iem23xwSLUKQp(binaryReader, (int)G06sxiw0LXP5YHrU1R(wbuLYvIupA1qwEr2D6(binaryReader)));
					num8 = 18;
					goto IL_0172;
				case 163:
					array2[7] = 110;
					num4 = 128;
					goto IL_0176;
				case 321:
					array[10] = (byte)num3;
					num4 = 132;
					if (true)
					{
						goto IL_0176;
					}
					goto case 433;
				case 433:
					array[26] = (byte)num7;
					num4 = 40;
					goto IL_0176;
				case 38:
					if (QdBLb2qYc.Length != 0)
					{
						goto case 289;
					}
					num4 = 46;
					if (true)
					{
						goto IL_0176;
					}
					goto case 353;
				case 353:
					num23 = num22 ^ num18;
					num4 = 47;
					if (true)
					{
						goto IL_0176;
					}
					goto case 144;
				case 144:
					array2[14] = (byte)num6;
					num = 342;
					continue;
				case 152:
					num17++;
					num8 = 178;
					goto IL_0172;
				case 53:
					num7 = 53 + 107;
					num4 = 278;
					goto IL_0176;
				case 270:
					array2[14] = (byte)num6;
					num4 = 183;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 277;
				case 277:
					num3 = 184 - 61;
					num4 = 10;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 258;
				case 258:
					QdBLb2qYc = (byte[])N05IrnniHNOagAdNrP(memoryStream);
					num4 = 322;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 11;
				case 11:
					array[3] = (byte)num3;
					num = 43;
					continue;
				case 367:
					num7 = 234 - 78;
					num = 126;
					continue;
				case 45:
					fRgS8FjvgMW4Fgec9h(symmetricAlgorithm, CipherMode.CBC);
					num4 = 3;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 236;
				case 236:
					array[29] = (byte)num3;
					num8 = 338;
					goto IL_0172;
				case 217:
					num7 = 80 - 13;
					num4 = 395;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 108;
				case 108:
					num3 = 71 - 12;
					num = 377;
					continue;
				case 81:
					array[21] = 140;
					num4 = 298;
					if (true)
					{
						goto IL_0176;
					}
					goto case 382;
				case 382:
					array[24] = (byte)num7;
					num4 = 233;
					if (true)
					{
						goto IL_0176;
					}
					goto case 89;
				case 36:
				case 100:
					if (num21 < array4.Length)
					{
						array3[num21] ^= array4[num21];
						num8 = 432;
					}
					else
					{
						num8 = 184;
					}
					goto IL_0172;
				case 84:
					if (num5 == num17 - 1)
					{
						num4 = 39;
						if (xhiIslqyx1D5WUBVSt())
						{
							goto IL_0176;
						}
						goto case 194;
					}
					goto IL_371f;
				case 252:
					array[11] = 99;
					num4 = 53;
					goto IL_0176;
				case 141:
				case 156:
					num7 = 15 + 35;
					num = 41;
					continue;
				case 210:
					num6 = 155 - 76;
					num8 = 352;
					goto IL_0172;
				case 403:
					array = new byte[32];
					num4 = 368;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 199;
				case 199:
					array[5] = (byte)num3;
					num4 = 12;
					if (true)
					{
						goto IL_0176;
					}
					goto case 63;
				case 63:
					array2[10] = 141;
					num8 = 176;
					goto IL_0172;
				case 292:
					array[11] = 95;
					num = 70;
					continue;
				case 407:
					array[25] = (byte)num3;
					num4 = 111;
					if (true)
					{
						goto IL_0176;
					}
					goto case 295;
				case 295:
					num9 = 116 - 49;
					num = 153;
					continue;
				case 251:
					num3 = 146 + 9;
					num8 = 29;
					goto IL_0172;
				case 275:
					num6 = 123 + 41;
					num = 270;
					continue;
				case 337:
					array[8] = (byte)num7;
					num8 = 390;
					goto IL_0172;
				case 116:
					array[30] = 56;
					num4 = 313;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 6;
				case 37:
					num3 = 122 + 36;
					num4 = 113;
					if (true)
					{
						goto IL_0176;
					}
					goto case 135;
				case 135:
					num20 = 0;
					num4 = 150;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 184;
				case 68:
					array[7] = (byte)num7;
					num4 = 7;
					if (true)
					{
						goto IL_0176;
					}
					goto case 282;
				case 282:
					array[0] = (byte)num7;
					num4 = 23;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 137;
				case 221:
					num3 = 87 + 97;
					num = 236;
					continue;
				case 329:
					array[25] = (byte)num7;
					num = 242;
					continue;
				case 172:
					array[27] = 134;
					num8 = 417;
					goto IL_0172;
				case 304:
					num7 = 148 - 49;
					num8 = 68;
					goto IL_0172;
				case 347:
					array6[num13] = (byte)(num14 & 0xFFu);
					num4 = 413;
					goto IL_0176;
				case 17:
					array[14] = 91;
					num = 341;
					continue;
				case 197:
					array[23] = (byte)num3;
					num4 = 335;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 319;
				case 319:
					array[20] = 138;
					num = 52;
					continue;
				case 354:
					num3 = 209 - 69;
					num8 = 228;
					goto IL_0172;
				case 388:
					array2[11] = (byte)num6;
					num = 229;
					continue;
				case 291:
					num6 = 118 + 107;
					num4 = 219;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 315;
				case 315:
					num6 = 154 - 51;
					num4 = 83;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 397;
				case 397:
					array[4] = 124;
					num4 = 344;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 358;
				case 358:
					num3 = 164 - 54;
					num4 = 429;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 233;
				case 233:
					num3 = 113 + 23;
					num8 = 326;
					goto IL_0172;
				case 248:
					num16 = 255u;
					num4 = 402;
					if (true)
					{
						goto IL_0176;
					}
					goto case 29;
				case 29:
					array[19] = (byte)num3;
					num8 = 173;
					goto IL_0172;
				case 265:
					num7 = 151 + 69;
					num = 329;
					continue;
				case 357:
					num3 = 102 + 94;
					num8 = 302;
					goto IL_0172;
				case 430:
					num9 = 30 + 80;
					num8 = 44;
					goto IL_0172;
				case 394:
					num3 = 119 + 0;
					num4 = 199;
					if (true)
					{
						goto IL_0176;
					}
					goto case 355;
				case 355:
					num3 = 105 + 32;
					num8 = 54;
					goto IL_0172;
				case 198:
					array[16] = (byte)num3;
					num = 34;
					continue;
				case 431:
					num12 = array7.Length % 4;
					num = 427;
					continue;
				case 273:
					num19 = array3.Length / 4;
					num8 = 160;
					goto IL_0172;
				case 147:
					num3 = 142 - 47;
					num8 = 321;
					goto IL_0172;
				case 278:
					array[11] = (byte)num7;
					num4 = 212;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 391;
				case 391:
					array[15] = (byte)num3;
					num4 = 82;
					goto IL_0176;
				case 420:
					array[11] = (byte)num7;
					num8 = 108;
					goto IL_0172;
				case 51:
					array2[3] = (byte)num9;
					num8 = 200;
					goto IL_0172;
				case 336:
					array[14] = (byte)num3;
					num = 293;
					continue;
				case 344:
					num7 = 120 + 94;
					num4 = 262;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 365;
				case 365:
					array4[3] = array5[1];
					num = 110;
					continue;
				case 379:
					array[26] = (byte)num3;
					num8 = 369;
					goto IL_0172;
				case 189:
					array2[12] = (byte)num9;
					num = 201;
					continue;
				case 274:
					array[10] = (byte)num3;
					num4 = 133;
					goto IL_0176;
				case 300:
					num7 = 151 - 50;
					num8 = 61;
					goto IL_0172;
				case 409:
					array2[3] = 165;
					num8 = 142;
					goto IL_0172;
				case 124:
					array[13] = 88;
					num8 = 373;
					goto IL_0172;
				case 283:
					array2[15] = (byte)num6;
					num8 = 162;
					goto IL_0172;
				case 155:
					num7 = 62 + 25;
					num = 382;
					continue;
				case 7:
					num3 = 23 + 101;
					num4 = 276;
					if (!xhiIslqyx1D5WUBVSt())
					{
						goto case 89;
					}
					goto IL_0176;
				case 19:
					num18 <<= 8;
					num4 = 168;
					goto IL_0176;
				case 151:
					array6[num13 + 2] = (byte)((num14 & 0xFF0000) >> 16);
					num8 = 231;
					goto IL_0172;
				case 173:
					num3 = 1 + 26;
					num4 = 65;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 338;
				case 348:
					array[29] = 109;
					num8 = 221;
					goto IL_0172;
				case 175:
					num9 = 235 - 78;
					num4 = 240;
					goto IL_0176;
				case 206:
					array2[0] = (byte)num9;
					num4 = 295;
					goto IL_0176;
				case 41:
					array[21] = (byte)num7;
					num = 217;
					continue;
				case 177:
					num6 = 228 - 76;
					num = 114;
					continue;
				case 138:
					if (num5 == num17 - 1)
					{
						num4 = 412;
						goto IL_0176;
					}
					goto IL_2111;
				case 85:
					array[18] = 112;
					num4 = 296;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 323;
				case 323:
					array[27] = 165;
					num4 = 367;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 169;
				case 169:
				case 325:
				{
					uint num26 = num22;
					uint num27 = num22;
					uint num28 = 608433572u;
					uint num29 = 376803549u;
					uint num30 = 1120927345u;
					uint num31 = 1198486640u;
					uint num32 = 1413770303u;
					uint num33 = num31 & 0xFF00FFu;
					uint num34 = num31 & 0xFF00FF00u;
					num33 = ((num33 >> 8) | (num34 << 8)) + num28;
					num31 = (num31 << 5) | (num31 >> 27);
					if ((double)num30 == 0.0)
					{
						num30--;
					}
					uint num35 = (uint)(1753167274.0 / (double)num30 + (double)num30);
					num30 = (uint)((double)(282190 * num35) + 1842983490.0);
					ulong num36 = num31 * num31;
					if (num36 == 0)
					{
						num36--;
					}
					num28 = (uint)(num28 * num28 % num36);
					num29 -= num31;
					num32 += num31;
					num27 ^= num27 >> 5;
					num27 += num28;
					num27 ^= num27 >> 28;
					num27 += num29;
					num27 ^= num27 << 27;
					num27 += num32;
					num27 = (((num31 << 11) + num29) ^ num29) + num27;
					num22 = num26 + (uint)(double)num27;
					num8 = 138;
					goto IL_0172;
				}
				case 231:
					array6[num13 + 3] = (byte)((num14 & 0xFF000000u) >> 24);
					num4 = 89;
					goto IL_0176;
				case 269:
					num6 = 32 + 56;
					num8 = 254;
					goto IL_0172;
				case 301:
					array2[8] = 4;
					num8 = 4;
					goto IL_0172;
				case 193:
					array2[1] = (byte)num6;
					num8 = 175;
					goto IL_0172;
				case 375:
					array4 = array2;
					num4 = 107;
					if (true)
					{
						goto IL_0176;
					}
					goto case 387;
				case 387:
					array[28] = (byte)num3;
					num8 = 94;
					goto IL_0172;
				case 73:
					num3 = 96 + 64;
					num8 = 318;
					goto IL_0172;
				case 165:
					num7 = 123 + 5;
					num4 = 268;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 160;
				case 48:
					array4[15] = array5[7];
					num = 218;
					continue;
				case 371:
					array2[9] = (byte)num9;
					num4 = 392;
					goto IL_0176;
				case 417:
					num7 = 185 - 61;
					num4 = 55;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 393;
				case 393:
					array2[5] = 59;
					num = 194;
					continue;
				case 402:
					num15 = 0;
					num4 = 84;
					if (true)
					{
						goto IL_0176;
					}
					goto case 161;
				case 161:
				case 267:
					array2[3] = (byte)num9;
					num4 = 409;
					goto IL_0176;
				case 384:
					array[6] = 169;
					num4 = 349;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 311;
				case 338:
					array[29] = 160;
					num4 = 186;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 86;
				case 86:
					num7 = 232 - 77;
					num4 = 195;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 372;
				case 372:
					num3 = 216 - 72;
					num4 = 324;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 143;
				case 143:
					num6 = 109 + 62;
					num4 = 26;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 305;
				case 58:
					array2[6] = 196;
					num8 = 246;
					goto IL_0172;
				case 240:
					array2[1] = (byte)num9;
					num8 = 75;
					goto IL_0172;
				case 10:
					array[24] = (byte)num3;
					num8 = 155;
					goto IL_0172;
				case 180:
					array2[1] = 135;
					num8 = 381;
					goto IL_0172;
				case 69:
					array[9] = 132;
					num8 = 165;
					goto IL_0172;
				case 404:
					num11 = 0u;
					num = 340;
					continue;
				case 130:
					array2[0] = 100;
					num4 = 205;
					if (true)
					{
						goto IL_0176;
					}
					goto case 213;
				case 213:
					array[28] = (byte)num7;
					num8 = 266;
					goto IL_0172;
				case 286:
					num10 = (uint)(num25 * 4);
					num = 333;
					continue;
				case 191:
					array2[10] = 235;
					num8 = 98;
					goto IL_0172;
				case 332:
					array2[4] = (byte)num9;
					num4 = 139;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 117;
				case 117:
					array2[5] = 122;
					num4 = 143;
					goto IL_0176;
				case 414:
					array[8] = (byte)num3;
					num4 = 249;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 93;
				case 93:
					array[17] = 88;
					num4 = 185;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 211;
				case 330:
					array[31] = (byte)num7;
					num = 90;
					continue;
				case 272:
					num9 = 125 - 41;
					num4 = 206;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 208;
				case 220:
					num3 = 179 - 59;
					num = 359;
					continue;
				case 145:
					array2[12] = 168;
					num4 = 131;
					if (sv9ZVufpqUtVdKBfBn())
					{
						goto case 363;
					}
					goto IL_0176;
				case 185:
					array[17] = 101;
					num8 = 351;
					goto IL_0172;
				case 34:
					array[16] = 156;
					num = 97;
					continue;
				case 122:
					array4[9] = array5[4];
					num4 = 158;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 215;
				case 215:
					num7 = 204 - 68;
					num8 = 362;
					goto IL_0172;
				case 61:
					array[5] = (byte)num7;
					num4 = 394;
					goto IL_0176;
				case 103:
					array[23] = 17;
					num4 = 277;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 71;
				case 71:
					array[1] = (byte)num7;
					num = 1;
					continue;
				case 239:
					array2[1] = 148;
					num8 = 399;
					goto IL_0172;
				case 320:
					num3 = 80 + 50;
					num4 = 11;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 423;
				case 423:
					num3 = 134 - 91;
					num = 336;
					continue;
				case 250:
					array6 = new byte[array7.Length];
					num4 = 273;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 380;
				case 268:
					array[9] = (byte)num7;
					num = 196;
					continue;
				case 183:
					num6 = 142 + 55;
					num4 = 144;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 104;
				case 104:
					array[25] = (byte)num7;
					num = 265;
					continue;
				case 376:
					array[30] = 142;
					num = 33;
					continue;
				case 146:
					num15 += 8;
					num4 = 310;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 126;
				case 126:
					array[28] = (byte)num7;
					num4 = 149;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 237;
				case 237:
					array[4] = (byte)num7;
					num8 = 397;
					goto IL_0172;
				case 254:
					array2[8] = (byte)num6;
					num8 = 301;
					goto IL_0172;
				case 386:
					array5 = (byte[])vmJP1cFZrHVEWWMLYX(c9ZTiOBR1gjHeMUCBh(aVhy5TW9O));
					num8 = 134;
					goto IL_0172;
				case 226:
					array[10] = (byte)num3;
					num4 = 419;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 3;
				case 94:
					num7 = 92 + 116;
					num4 = 308;
					if (true)
					{
						goto IL_0176;
					}
					goto case 271;
				case 271:
					num3 = 31 + 21;
					num8 = 223;
					goto IL_0172;
				case 249:
					array[8] = 138;
					num = 215;
					continue;
				case 20:
					array[19] = 172;
					num4 = 37;
					if (true)
					{
						goto IL_0176;
					}
					goto case 195;
				case 195:
					array[1] = (byte)num7;
					num8 = 389;
					goto IL_0172;
				case 120:
					array2[6] = (byte)num9;
					num8 = 58;
					goto IL_0172;
				case 241:
					array[3] = 177;
					num8 = 312;
					goto IL_0172;
				case 374:
					array[23] = (byte)num7;
					num = 235;
					continue;
				case 309:
					num9 = 44 + 68;
					num4 = 371;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 140;
				case 140:
					array[2] = (byte)num3;
					num4 = 400;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 413;
				case 413:
					array6[num13 + 1] = (byte)((num14 & 0xFF00) >> 8);
					num8 = 151;
					goto IL_0172;
				case 89:
				case 383:
					num5++;
					num = 411;
					continue;
				case 421:
					array[16] = 163;
					num4 = 188;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 262;
				case 262:
					array[4] = (byte)num7;
					num = 67;
					continue;
				case 149:
					num7 = 123 + 81;
					num8 = 213;
					goto IL_0172;
				case 425:
					array[20] = 101;
					num = 263;
					continue;
				case 212:
					num7 = 222 - 74;
					num4 = 420;
					goto IL_0176;
				case 205:
					num9 = 36 + 76;
					num8 = 243;
					goto IL_0172;
				case 1:
					array[2] = 113;
					num = 334;
					continue;
				case 154:
					num7 = 28 + 123;
					num4 = 330;
					goto IL_0176;
				case 400:
					num7 = 205 - 68;
					num8 = 207;
					goto IL_0172;
				case 369:
					num3 = 174 - 58;
					num4 = 35;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 62;
				case 62:
					array[17] = 90;
					num = 93;
					continue;
				case 114:
					array2[2] = (byte)num6;
					num4 = 27;
					goto IL_0176;
				case 167:
					array2[7] = (byte)num9;
					num = 163;
					continue;
				case 50:
					num13 = num5 * 4;
					num4 = 286;
					if (true)
					{
						goto IL_0176;
					}
					goto case 408;
				case 408:
					array[9] = 99;
					num = 69;
					continue;
				case 76:
					k9tr6aVQNT0yHArtX9(wbuLYvIupA1qwEr2D6(binaryReader), 0L);
					num4 = 118;
					goto IL_0176;
				case 399:
					array2[1] = 136;
					num4 = 180;
					goto IL_0176;
				case 139:
					num6 = 174 - 116;
					num4 = 209;
					goto IL_0176;
				case 362:
					array[9] = (byte)num7;
					num = 408;
					continue;
				case 235:
					array[23] = 100;
					num4 = 354;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 66;
				case 66:
					array4[7] = array5[3];
					num4 = 122;
					if (true)
					{
						goto IL_0176;
					}
					goto case 22;
				case 22:
					array2[13] = 140;
					num4 = 264;
					goto IL_0176;
				case 164:
					num7 = 100 + 67;
					num8 = 227;
					goto IL_0172;
				case 263:
					array[21] = 190;
					num4 = 164;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 149;
				case 339:
					array2[3] = (byte)num9;
					num = 179;
					continue;
				case 229:
					array2[11] = 208;
					num4 = 418;
					if (false)
					{
						break;
					}
					goto IL_0176;
				case 92:
					array2[10] = 85;
					num8 = 290;
					goto IL_0172;
				case 230:
					array[30] = (byte)num7;
					num4 = 21;
					if (true)
					{
						goto IL_0176;
					}
					goto case 419;
				case 419:
					num3 = 231 - 77;
					num4 = 274;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 83;
				case 83:
					array2[11] = (byte)num6;
					num8 = 416;
					goto IL_0172;
				case 6:
					array[16] = (byte)num7;
					num = 415;
					continue;
				case 39:
					if (num12 > 0)
					{
						num4 = 244;
						if (0 == 0)
						{
							goto IL_0176;
						}
						goto case 60;
					}
					goto IL_371f;
				case 60:
					array[31] = 112;
					num4 = 154;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 429;
				case 429:
					array[1] = (byte)num3;
					num4 = 86;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 349;
				case 349:
					num7 = 157 - 52;
					num4 = 222;
					goto IL_0176;
				case 333:
					num11 = (uint)((array3[num10 + 3] << 24) | (array3[num10 + 2] << 16) | (array3[num10 + 1] << 8) | array3[num10]);
					num4 = 248;
					goto IL_0176;
				case 31:
					array[31] = 52;
					num8 = 357;
					goto IL_0172;
				case 377:
					array[11] = (byte)num3;
					num = 72;
					continue;
				case 311:
					num16 <<= 8;
					num4 = 146;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 386;
				case 74:
					array[15] = 94;
					num4 = 8;
					goto IL_0176;
				case 293:
					num3 = 203 - 67;
					num8 = 391;
					goto IL_0172;
				case 350:
					num3 = 201 - 67;
					num4 = 198;
					if (0 == 0)
					{
						goto IL_0176;
					}
					goto case 24;
				case 24:
					num6 = 186 - 62;
					num8 = 136;
					goto IL_0172;
				case 171:
					num7 = 58 + 99;
					num4 = 433;
					goto IL_0176;
				case 345:
					num9 = 56 + 32;
					goto case 161;
				case 370:
				case 411:
					if (num5 < num17)
					{
						num25 = num5 % num19;
						num4 = 50;
						goto IL_0176;
					}
					num = 285;
					continue;
				case 368:
					array[0] = 155;
					num4 = 406;
					if (!sv9ZVufpqUtVdKBfBn())
					{
						goto IL_0176;
					}
					goto case 175;
				case 219:
					array2[13] = (byte)num6;
					num8 = 430;
					goto IL_0172;
				case 296:
					array[19] = 186;
					num = 13;
					continue;
				case 218:
					num21 = 0;
					num8 = 100;
					goto IL_0172;
				case 113:
					array[19] = (byte)num3;
					num8 = 279;
					goto IL_0172;
				case 422:
					array[31] = (byte)num7;
					num = 88;
					continue;
				case 247:
					array2[3] = (byte)num6;
					num4 = 345;
					if (xhiIslqyx1D5WUBVSt())
					{
						goto IL_0176;
					}
					goto case 183;
				case 398:
					array2[8] = 75;
					num = 96;
					continue;
				case 216:
					num5 = 0;
					num = 370;
					continue;
				case 434:
					try
					{
						return (string)AVHlq1RdqCVT8PXkVn(wxKd6LhpIKaNJeRJeH(), QdBLb2qYc, P_0 + 4, num2);
					}
					catch
					{
					}
					return "";
				case 363:
					symmetricAlgorithm = (SymmetricAlgorithm)WvNXiO9h9d281mlDCn();
					num8 = 45;
					goto IL_0172;
				case 256:
					break;
					IL_371f:
					num10 = (uint)num13;
					num4 = 80;
					goto IL_0176;
					IL_2111:
					num14 = num22 ^ num18;
					num4 = 347;
					goto IL_0176;
					IL_0172:
					num4 = num8;
					goto IL_0176;
					IL_0176:
					num = num4;
					continue;
				}
				memoryStream = new MemoryStream();
				num = 232;
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
		internal static string uWtBmNRNb(string P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			"{11111-22222-50001-00000}".Trim();
			byte[] array = Convert.FromBase64String(P_0);
			return Encoding.Unicode.GetString(array, 0, array.Length);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static int jYebYbQma()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return 5;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static void wSuUaxHIy()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				RSACryptoServiceProvider.UseMachineKeyStore = true;
			}
			catch
			{
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private static Delegate s88fmyuIQ(IntPtr P_0, Type P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return (Delegate)typeof(Marshal).GetMethod("GetDelegateForFunctionPointer", new Type[2]
			{
				typeof(IntPtr),
				typeof(Type)
			}).Invoke(null, new object[2] { P_0, P_1 });
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object B0NEXSehW(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			try
			{
				if (File.Exists(((Assembly)P_0).Location))
				{
					return ((Assembly)P_0).Location;
				}
			}
			catch
			{
			}
			try
			{
				if (File.Exists(((Assembly)P_0).GetName().CodeBase.ToString().Replace("file:///", "")))
				{
					return ((Assembly)P_0).GetName().CodeBase.ToString().Replace("file:///", "");
				}
			}
			catch
			{
			}
			try
			{
				if (File.Exists(P_0.GetType().GetProperty("Location").GetValue(P_0, new object[0])
					.ToString()))
				{
					return P_0.GetType().GetProperty("Location").GetValue(P_0, new object[0])
						.ToString();
				}
			}
			catch
			{
			}
			return "";
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
		private static byte[] xQwQQOpQK(string P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			using FileStream fileStream = new FileStream(P_0, FileMode.Open, FileAccess.Read, FileShare.Read);
			int num = 0;
			long length = fileStream.Length;
			int num2 = (int)length;
			byte[] array = new byte[num2];
			while (num2 > 0)
			{
				int num3 = fileStream.Read(array, num, num2);
				num += num3;
				num2 -= num3;
			}
			return array;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		[KjHRR08XlRopQfagms(typeof(KjHRR08XlRopQfagms.ghsislHGr7j3sh1vyo<object>[]))]
		private static byte[] Vv642YWdN(byte[] P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			MemoryStream memoryStream = new MemoryStream();
			SymmetricAlgorithm symmetricAlgorithm = z7g9Hl3ui();
			symmetricAlgorithm.Key = new byte[32]
			{
				97, 206, 145, 165, 225, 182, 95, 57, 148, 61,
				39, 156, 64, 111, 129, 238, 214, 231, 61, 195,
				178, 197, 15, 97, 110, 17, 0, 50, 184, 169,
				162, 199
			};
			symmetricAlgorithm.IV = new byte[16]
			{
				164, 184, 234, 97, 138, 2, 5, 161, 120, 203,
				16, 93, 97, 63, 33, 117
			};
			CryptoStream cryptoStream = new CryptoStream(memoryStream, symmetricAlgorithm.CreateDecryptor(), CryptoStreamMode.Write);
			cryptoStream.Write(P_0, 0, P_0.Length);
			cryptoStream.Close();
			return memoryStream.ToArray();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] vTtmY9OBq()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return null;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] GZv2byBqH()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return null;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] je7gypw7m()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-20001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] O50iGmFDJ()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-20001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] mJtXJWe3Z()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-30001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		private byte[] wV8u78AMI()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-30001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] SnCwjE43O()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-40001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] fSqqnJihy()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-40001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] P9tAhpLsJ()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-50001-00001}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal byte[] etQKR5t68()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			string text = "{11111-22222-50001-00002}";
			if (text.Length > 0)
			{
				return new byte[2] { 1, 2 };
			}
			return new byte[2] { 1, 2 };
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public KYJngMCOCJNSf8T7gH()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			base..ctor();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object mJyO00M3UjGXaqPZkZ(object P_0, object P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Assembly)P_0).GetManifestResourceStream((string)P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object wbuLYvIupA1qwEr2D6(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((BinaryReader)P_0).BaseStream;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void k9tr6aVQNT0yHArtX9(object P_0, long P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((Stream)P_0).Position = P_1;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void KSAvrHdmiBtXgP30M7(bool P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			RSACryptoServiceProvider.UseMachineKeyStore = P_0;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static long G06sxiw0LXP5YHrU1R(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Stream)P_0).Length;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object TNt27iem23xwSLUKQp(object P_0, int P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((BinaryReader)P_0).ReadBytes(P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void G5AN3eK6lnHWagAG0N(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((BinaryReader)P_0).Close();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void IfBCnwi0sGlQQWMCAp(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			Array.Reverse((Array)P_0);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object c9ZTiOBR1gjHeMUCBh(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Assembly)P_0).GetName();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object vmJP1cFZrHVEWWMLYX(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((AssemblyName)P_0).GetPublicKeyToken();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object WvNXiO9h9d281mlDCn()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return z7g9Hl3ui();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void fRgS8FjvgMW4Fgec9h(object P_0, CipherMode P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((SymmetricAlgorithm)P_0).Mode = P_1;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object n1q6NkD5E3kG69nYNh(object P_0, object P_1, object P_2)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((SymmetricAlgorithm)P_0).CreateDecryptor((byte[])P_1, (byte[])P_2);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void oRfwbQuDKvToZ2xxIJ(object P_0, object P_1, int P_2, int P_3)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((Stream)P_0).Write((byte[])P_1, P_2, P_3);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void iGCSRRaVF18o3clKoJ(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((CryptoStream)P_0).FlushFinalBlock();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object N05IrnniHNOagAdNrP(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((MemoryStream)P_0).ToArray();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void CKU7bxUcwRJMDZJF1y(object P_0)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			((Stream)P_0).Close();
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static int QDTqS570cuCgZu2ShH(object P_0, int P_1)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return BitConverter.ToInt32((byte[])P_0, P_1);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object wxKd6LhpIKaNJeRJeH()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return Encoding.Unicode;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static object AVHlq1RdqCVT8PXkVn(object P_0, object P_1, int P_2, int P_3)
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return ((Encoding)P_0).GetString((byte[])P_1, P_2, P_3);
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static bool xhiIslqyx1D5WUBVSt()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return true;
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static bool sv9ZVufpqUtVdKBfBn()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			return false;
		}
	}
}
namespace hU4x3GePSuIEn9q1kR
{
	internal class Rul53eN1pQAkHhMgbS
	{
		private static bool qexONkNhx8;

		[MethodImpl(MethodImplOptions.NoInlining)]
		internal static void yKfY03hzDUAti()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
		}

		[MethodImpl(MethodImplOptions.NoInlining)]
		public Rul53eN1pQAkHhMgbS()
		{
			while (false)
			{
				_ = ((object[])null)[0];
			}
			base..ctor();
		}
	}
}
