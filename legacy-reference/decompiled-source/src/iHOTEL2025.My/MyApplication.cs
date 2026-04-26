using System;
using System.CodeDom.Compiler;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Windows.Forms;
using Microsoft.VisualBasic.ApplicationServices;

namespace iHOTEL2025.My;

[EditorBrowsable(EditorBrowsableState.Never)]
[GeneratedCode("MyTemplate", "8.0.0.0")]
internal class MyApplication : WindowsFormsApplicationBase
{
	private static List<WeakReference> __ENCList = new List<WeakReference>();

	[EditorBrowsable(EditorBrowsableState.Advanced)]
	[STAThread]
	[DebuggerHidden]
	internal static void Main(string[] args)
	{
		try
		{
			Application.SetCompatibleTextRenderingDefault(WindowsFormsApplicationBase.UseCompatibleTextRendering);
			Class2.LH6iGfYz9j3MJ();
		}
		finally
		{
		}
		MyProject.Application.Run(args);
	}

	[DebuggerStepThrough]
	public MyApplication()
		: base(AuthenticationMode.Windows)
	{
		lock (__ENCList)
		{
			__ENCList.Add(new WeakReference(this));
		}
		IsSingleInstance = true;
		EnableVisualStyles = true;
		SaveMySettingsOnExit = true;
		ShutdownStyle = ShutdownMode.AfterMainFormCloses;
	}

	[DebuggerStepThrough]
	protected override void OnCreateMainForm()
	{
		MainForm = MyProject.Forms.frmMain1;
	}
}
